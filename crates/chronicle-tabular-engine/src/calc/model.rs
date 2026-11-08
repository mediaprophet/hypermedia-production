//! Workbook, sheet and cell definitions for calculation.

use std::collections::BTreeMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::core::addr::{CellRef, RangeRef};
use crate::core::date::DateSystem;
use crate::core::value::Value;
use crate::formula::ast::Expr;
use crate::formula::parser::parse;
use crate::formula::printer::print;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalcMode {
    #[default]
    Automatic,
    Manual,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalcSettings {
    pub mode: CalcMode,
    pub iterative: bool,
    pub max_iterations: u32,
    pub max_change: f64,
}

impl Default for CalcSettings {
    fn default() -> Self {
        CalcSettings {
            mode: CalcMode::Automatic,
            iterative: false,
            max_iterations: 100,
            max_change: 0.001,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefinedName {
    pub name: String,
    pub scope: Option<usize>,
    pub formula: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Formula {
    pub text: String,
    #[serde(skip)]
    pub expr: Option<Arc<Expr>>,
}

impl Formula {
    pub fn new(text: &str) -> Formula {
        let body = text.strip_prefix('=').unwrap_or(text);
        match parse(body) {
            Ok(e) => Formula {
                text: print(&e),
                expr: Some(Arc::new(e)),
            },
            Err(_) => Formula {
                text: body.to_string(),
                expr: None,
            },
        }
    }

    pub fn expr(&self) -> Option<Expr> {
        self.expr.as_deref().cloned().or_else(|| parse(&self.text).ok())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    pub value: Value,
    pub formula: Option<Arc<Formula>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Sheet {
    pub name: String,
    pub cells: BTreeMap<CellRef, Cell>,
    pub spill_ranges: BTreeMap<CellRef, RangeRef>,
    pub spill: BTreeMap<CellRef, Value>,
    pub merges: Vec<RangeRef>,
}

impl Sheet {
    pub fn new(name: impl Into<String>) -> Self {
        Sheet {
            name: name.into(),
            cells: BTreeMap::new(),
            spill_ranges: BTreeMap::new(),
            spill: BTreeMap::new(),
            merges: Vec::new(),
        }
    }

    pub fn value(&self, c: CellRef) -> Value {
        if let Some(v) = self.spill.get(&c) {
            return v.clone();
        }
        self.cells.get(&c).map(|cell| cell.value.clone()).unwrap_or(Value::Empty)
    }

    pub fn set_value(&mut self, c: CellRef, v: Value) {
        self.cells.entry(c).or_default().value = v;
    }

    pub fn set_formula(&mut self, c: CellRef, f: Formula) {
        let cell = self.cells.entry(c).or_default();
        cell.formula = Some(Arc::new(f));
    }

    pub fn used_range(&self) -> Option<RangeRef> {
        let mut min_r = u32::MAX;
        let mut max_r = 0u32;
        let mut min_c = u32::MAX;
        let mut max_c = 0u32;
        let mut found = false;

        for c in self.cells.keys().chain(self.spill.keys()) {
            found = true;
            min_r = min_r.min(c.row);
            max_r = max_r.max(c.row);
            min_c = min_c.min(c.col);
            max_c = max_c.max(c.col);
        }

        if found {
            Some(RangeRef::new(CellRef::new(min_r, min_c), CellRef::new(max_r, max_c)))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workbook {
    pub sheets: Vec<Arc<Sheet>>,
    pub date_system: DateSystem,
    pub calc_settings: CalcSettings,
    pub names: Vec<DefinedName>,
}

impl Default for Workbook {
    fn default() -> Self {
        Workbook {
            sheets: vec![Arc::new(Sheet::new("Sheet1"))],
            date_system: DateSystem::D1900,
            calc_settings: CalcSettings::default(),
            names: Vec::new(),
        }
    }
}

impl Workbook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sheet(&self, idx: usize) -> Option<&Sheet> {
        self.sheets.get(idx).map(|s| s.as_ref())
    }

    pub fn sheet_mut(&mut self, idx: usize) -> Option<&mut Sheet> {
        let arc = self.sheets.get_mut(idx)?;
        Some(Arc::make_mut(arc))
    }

    pub fn add_sheet(&mut self, name: impl Into<String>) -> usize {
        let idx = self.sheets.len();
        self.sheets.push(Arc::new(Sheet::new(name)));
        idx
    }
}
