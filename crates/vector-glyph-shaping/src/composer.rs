//! Knuth-Plass optimal line-breaking total-fit paragraph composer.
//!
//! Grounded in typographic fidelity and authoring rights (UDHR Art. 19, 27).

use serde::{Deserialize, Serialize};

pub const STRETCH: f64 = 0.5;
pub const SHRINK: f64 = 0.2;
pub const HYPHEN_PENALTY: f64 = 50.0;
pub const DOUBLE_HYPHEN_DEMERITS: f64 = 3000.0;
pub const LINE_PENALTY: f64 = 10.0;
pub const ADJ_DEMERITS_JUSTIFIED: f64 = 10_000.0;
pub const ADJ_DEMERITS_RAGGED: f64 = 1_000.0;
pub const HYPHEN_PENALTY_RAGGED: f64 = 25.0;
pub const RAG_ZONE: f64 = 1.0 / 6.0;
pub const SHORT_LAST: f64 = 1.0 / 3.0;
pub const SHORT_LAST_ZONE: f64 = 1.0 / 3.0;
pub const MAX_ACTIVE: usize = 4096;
pub const EPS: f64 = 1e-6;

/// Shaped glyph summary for composition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SGlyph {
    pub x_advance: f64,
    pub is_space: bool,
    pub is_break: bool,
}

impl SGlyph {
    pub fn char_glyph(width: f64) -> Self {
        Self {
            x_advance: width,
            is_space: false,
            is_break: false,
        }
    }
    pub fn space_glyph(width: f64) -> Self {
        Self {
            x_advance: width,
            is_space: true,
            is_break: false,
        }
    }
}

/// A candidate breakpoint: glyph index `end` (exclusive) and optional hyphen width.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Breakpoint {
    pub end: usize,
    pub hyphen: f64,
}

/// Configuration parameters for paragraph line composition.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Params {
    pub justify_last: bool,
    pub ragged: bool,
    pub tolerance: f64,
    pub uniform_from: usize,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            justify_last: false,
            ragged: true,
            tolerance: 2.0,
            uniform_from: 0,
        }
    }
}

struct Node {
    pos: usize,
    line: usize,
    fitness: u8,
    demerits: f64,
    hyphenated: bool,
    prev: Option<usize>,
}

fn fitness(r: f64, ragged: bool) -> u8 {
    let bounds = if ragged { [0.15, 0.4, 0.75] } else { [-0.5, 0.5, 1.0] };
    if r < bounds[0] {
        0
    } else if r <= bounds[1] {
        1
    } else if r <= bounds[2] {
        2
    } else {
        3
    }
}

/// Knuth-Plass line composition over glyph slice `g`.
pub fn compose(
    g: &[SGlyph],
    width_fn: &dyn Fn(usize) -> f64,
    start_credit_fn: &dyn Fn(usize) -> f64,
    cands: &[Breakpoint],
    p: &Params,
) -> Option<Vec<(usize, bool)>> {
    let n = g.len();
    if n == 0 {
        return Some(vec![]);
    }

    let mut breaks = cands.to_vec();
    if breaks.last().map_or(true, |b| b.end < n) {
        breaks.push(Breakpoint { end: n, hyphen: 0.0 });
    }

    let mut nodes = vec![Node {
        pos: 0,
        line: 0,
        fitness: 1,
        demerits: 0.0,
        hyphenated: false,
        prev: None,
    }];
    let mut active = vec![0];

    for (cand_idx, brk) in breaks.iter().enumerate() {
        let is_last = brk.end == n;
        let mut new_active = Vec::new();

        for &act_idx in &active {
            let act = &nodes[act_idx];
            let start = act.pos;
            if start >= brk.end {
                continue;
            }

            let mut natural_w = 0.0;
            let mut stretch_w = 0.0;
            let mut shrink_w = 0.0;

            for glyph in &g[start..brk.end] {
                natural_w += glyph.x_advance;
                if glyph.is_space {
                    stretch_w += glyph.x_advance * STRETCH;
                    shrink_w += glyph.x_advance * SHRINK;
                }
            }

            natural_w += brk.hyphen - start_credit_fn(start);
            let target_w = width_fn(act.line);
            let diff = target_w - natural_w;

            let (r, demerits) = if p.ragged {
                let r = diff / (target_w * RAG_ZONE).max(1.0);
                let badness = r.abs() * 10.0;
                let penalty = if brk.hyphen > 0.0 { HYPHEN_PENALTY_RAGGED } else { 0.0 };
                let d = (LINE_PENALTY + badness + penalty).powi(2);
                (r, d)
            } else {
                let r = if diff >= 0.0 {
                    if stretch_w > EPS { diff / stretch_w } else { 0.0 }
                } else if shrink_w > EPS {
                    diff / shrink_w
                } else {
                    -10.0
                };
                if r < -1.0 || (r > p.tolerance && !is_last) {
                    continue;
                }
                let badness = 100.0 * (r.abs()).powi(3);
                let penalty = if brk.hyphen > 0.0 { HYPHEN_PENALTY } else { 0.0 };
                let d = (LINE_PENALTY + badness + penalty).powi(2);
                (r, d)
            };

            let fit_class = fitness(r, p.ragged);
            let adj_demerits = if (fit_class as i32 - act.fitness as i32).abs() > 1 {
                if p.ragged { ADJ_DEMERITS_RAGGED } else { ADJ_DEMERITS_JUSTIFIED }
            } else {
                0.0
            };
            let double_hyphen = if act.hyphenated && brk.hyphen > 0.0 {
                DOUBLE_HYPHEN_DEMERITS
            } else {
                0.0
            };

            let total_demerits = act.demerits + demerits + adj_demerits + double_hyphen;

            let next_line = if act.line >= p.uniform_from {
                act.line
            } else {
                act.line + 1
            };

            let new_node_idx = nodes.len();
            nodes.push(Node {
                pos: brk.end,
                line: next_line,
                fitness: fit_class,
                demerits: total_demerits,
                hyphenated: brk.hyphen > 0.0,
                prev: Some(act_idx),
            });
            new_active.push(new_node_idx);
        }

        if !new_active.is_empty() {
            active.extend(new_active);
            if active.len() > MAX_ACTIVE {
                active.sort_by(|&a, &b| nodes[a].demerits.total_cmp(&nodes[b].demerits));
                active.truncate(MAX_ACTIVE / 2);
            }
        }
    }

    // Find best terminal node ending at n
    let best_terminal = active
        .iter()
        .filter(|&&idx| nodes[idx].pos == n)
        .min_by(|&&a, &&b| nodes[a].demerits.total_cmp(&nodes[b].demerits))
        .copied();

    let Some(mut curr) = best_terminal else {
        return None;
    };

    let mut result = Vec::new();
    while let Some(prev) = nodes[curr].prev {
        result.push((nodes[curr].pos, nodes[curr].hyphenated));
        curr = prev;
    }
    result.reverse();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_empty_paragraph() {
        let p = Params::default();
        let res = compose(&[], &|_| 100.0, &|_| 0.0, &[], &p);
        assert_eq!(res, Some(vec![]));
    }

    #[test]
    fn compose_single_line_fit() {
        let glyphs = [
            SGlyph::char_glyph(10.0),
            SGlyph::char_glyph(10.0),
            SGlyph::space_glyph(10.0),
            SGlyph::char_glyph(10.0),
        ];
        let p = Params::default();
        let res = compose(&glyphs, &|_| 100.0, &|_| 0.0, &[], &p).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0, 4);
    }
}
