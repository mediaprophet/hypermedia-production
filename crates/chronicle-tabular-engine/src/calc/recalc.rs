//! Dependency-driven DAG topological recalculation engine with spill propagation and cycle detection.

use std::collections::VecDeque;
use std::hash::{BuildHasherDefault, Hasher};
use std::sync::Arc;

use crate::core::addr::{CellRef, RangeRef};
use crate::core::value::{Array, CellError, Value};
use crate::calc::eval::{Area, Evaluator, Host, precedents};
use crate::calc::model::{Sheet, Workbook};

#[derive(Default, Clone, Copy)]
pub struct FxHasher(u64);

impl Hasher for FxHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.write_u64(*b as u64);
        }
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
}

pub type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasherDefault<FxHasher>>;
pub type HashSet<K> = std::collections::HashSet<K, BuildHasherDefault<FxHasher>>;

pub type Key = (usize, CellRef);

#[derive(Clone, Debug, Default)]
pub struct Node {
    pub areas: Vec<Area>,
    pub dynamic: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Graph {
    pub nodes: HashMap<Key, Node>,
    pub cell_deps: HashMap<Key, HashSet<Key>>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, k: Key, node: Node) {
        for a in &node.areas {
            for c in a.range.iter() {
                self.cell_deps.entry((a.sheet, c)).or_default().insert(k);
            }
        }
        self.nodes.insert(k, node);
    }

    pub fn dependents(&self, k: Key) -> Option<&HashSet<Key>> {
        self.cell_deps.get(&k)
    }
}

pub struct Calc {
    pub graph: Graph,
    pub now: f64,
    pub rng_state: u64,
}

impl Default for Calc {
    fn default() -> Self {
        Calc {
            graph: Graph::new(),
            now: 46302.0, // Fixed default serial date
            rng_state: 0x853c49e6748fea9b,
        }
    }
}

impl Calc {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn rebuild(&mut self, wb: &Workbook) {
        self.graph = Graph::new();
        for (si, sheet) in wb.sheets.iter().enumerate() {
            for (c, cell) in &sheet.cells {
                if let Some(f) = &cell.formula {
                    if let Some(e) = f.expr() {
                        let (areas, dynamic) = precedents(wb, si, &e);
                        self.graph.insert((si, *c), Node { areas, dynamic });
                    }
                }
            }
        }
    }

    pub fn recalc_all(&mut self, wb: &mut Workbook) {
        self.rebuild(wb);

        // Find topological order or detect cycles
        let mut in_degrees: HashMap<Key, usize> = HashMap::default();
        let mut adj: HashMap<Key, Vec<Key>> = HashMap::default();

        for (&dependent_k, node) in &self.graph.nodes {
            in_degrees.entry(dependent_k).or_insert(0);
            for a in &node.areas {
                for c in a.range.iter() {
                    let prec_k = (a.sheet, c);
                    if self.graph.nodes.contains_key(&prec_k) {
                        adj.entry(prec_k).or_default().push(dependent_k);
                        *in_degrees.entry(dependent_k).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut queue: VecDeque<Key> = in_degrees
            .iter()
            .filter(|(_, deg)| **deg == 0)
            .map(|(k, _)| *k)
            .collect();

        let mut order: Vec<Key> = Vec::with_capacity(self.graph.nodes.len());

        while let Some(k) = queue.pop_front() {
            order.push(k);
            if let Some(children) = adj.get(&k) {
                for &child in children {
                    if let Some(deg) = in_degrees.get_mut(&child) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(child);
                        }
                    }
                }
            }
        }

        // Any nodes with in_degree > 0 are in a circular dependency
        let mut cycles = HashSet::default();
        for (&k, &deg) in &in_degrees {
            if deg > 0 {
                cycles.insert(k);
            }
        }

        // Mark cycle cells with #CIRC!
        for &(si, c) in &cycles {
            if let Some(sh) = wb.sheet_mut(si) {
                sh.set_value(c, Value::Error(CellError::Circ));
            }
        }

        // Clear existing spills
        for sh in &mut wb.sheets {
            let sh_mut = Arc::make_mut(sh);
            sh_mut.spill.clear();
            sh_mut.spill_ranges.clear();
        }

        // Recalculate in topological order
        for (si, c) in order {
            if cycles.contains(&(si, c)) {
                continue;
            }
            let formula = {
                let Some(sh) = wb.sheet(si) else { continue };
                let Some(cell) = sh.cells.get(&c) else { continue };
                cell.formula.clone()
            };
            let Some(f) = formula else { continue };
            let Some(expr) = f.expr() else { continue };

            let mut host = PassHost {
                wb,
                now: self.now,
                rng: &mut self.rng_state,
            };

            let mut ev = Evaluator::new(&mut host, si, c);
            let result_ev = ev.eval(&expr);
            let val = ev.to_value(result_ev);

            // Handle dynamic array spills
            if let Value::Array(ref arr) = val {
                if arr.rows > 1 || arr.cols > 1 {
                    let end_r = c.row.saturating_add(arr.rows as u32 - 1);
                    let end_c = c.col.saturating_add(arr.cols as u32 - 1);
                    let spill_range = RangeRef::new(c, CellRef::new(end_r, end_c));

                    // Check for obstruction
                    let mut blocked = false;
                    if let Some(sh) = wb.sheet(si) {
                        for r in c.row..=end_r {
                            for col in c.col..=end_c {
                                let check_cell = CellRef::new(r, col);
                                if check_cell != c {
                                    if let Some(existing) = sh.cells.get(&check_cell) {
                                        if existing.formula.is_some() || !existing.value.is_empty() {
                                            blocked = true;
                                            break;
                                        }
                                    }
                                }
                            }
                            if blocked { break; }
                        }
                    }

                    if blocked {
                        if let Some(sh) = wb.sheet_mut(si) {
                            sh.set_value(c, Value::Error(CellError::Spill));
                        }
                    } else {
                        if let Some(sh) = wb.sheet_mut(si) {
                            sh.set_value(c, val.clone());
                            sh.spill_ranges.insert(c, spill_range);
                            for r in 0..arr.rows {
                                for col in 0..arr.cols {
                                    let cell_pos = CellRef::new(c.row + r as u32, c.col + col as u32);
                                    if cell_pos != c {
                                        if let Some(elem) = arr.get(r, col) {
                                            sh.spill.insert(cell_pos, elem.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    continue;
                }
            }

            if let Some(sh) = wb.sheet_mut(si) {
                sh.set_value(c, val);
            }
        }
    }
}

struct PassHost<'a> {
    wb: &'a mut Workbook,
    now: f64,
    rng: &'a mut u64,
}

impl Host for PassHost<'_> {
    fn workbook(&self) -> &Workbook {
        self.wb
    }

    fn cell_value(&mut self, sheet: usize, c: CellRef) -> Value {
        self.wb.sheet(sheet).map(|s| s.value(c)).unwrap_or(Value::Empty)
    }

    fn now_serial(&self) -> f64 {
        self.now
    }

    fn random(&mut self) -> f64 {
        let mut x = *self.rng;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        *self.rng = x;
        (x.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 11) as f64 / (1u64 << 53) as f64
    }
}
