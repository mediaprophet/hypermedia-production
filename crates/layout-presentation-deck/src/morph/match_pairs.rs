//! Shape correspondence matching between adjacent slides.

use crate::model::shape::Shape;
use crate::model::master::Slide;
use crate::model::types::ShapeId;

fn text_of(s: &Shape) -> String {
    s.text().unwrap_or_default()
}

/// Pairs of (shape on `a`, shape on `b`) that Morph animates from one to the other:
/// 1. `!!name` (forced match)
/// 2. Exact name match
/// 3. Same kind and identical non-empty text
/// 4. Same kind and empty text
pub fn morph_pairs(a: &Slide, b: &Slide) -> Vec<(ShapeId, ShapeId)> {
    let mut pairs: Vec<(ShapeId, ShapeId)> = Vec::new();
    let mut used_a = vec![false; a.shapes.len()];
    let mut used_b = vec![false; b.shapes.len()];

    let passes: [&dyn Fn(&Shape, &Shape) -> bool; 4] = [
        // 1. Forced match via !! prefix
        &|x, y| x.name.starts_with("!!") && x.name == y.name,
        // 2. Exact name match
        &|x, y| !x.name.is_empty() && x.name == y.name,
        // 3. Same kind and non-empty matching text
        &|x, y| {
            if x.kind != y.kind {
                return false;
            }
            let tx = text_of(x);
            let ty = text_of(y);
            !tx.trim().is_empty() && tx.trim() == ty.trim()
        },
        // 4. Same kind and geometry
        &|x, y| {
            x.kind == y.kind && text_of(x).trim().is_empty() && text_of(y).trim().is_empty()
        },
    ];

    for pass in passes {
        for (i, sa) in a.shapes.iter().enumerate() {
            if used_a[i] {
                continue;
            }
            if let Some((j, _)) = b.shapes.iter().enumerate().find(|(j, sb)| !used_b[*j] && pass(sa, sb)) {
                used_a[i] = true;
                used_b[j] = true;
                pairs.push((sa.id, b.shapes[j].id));
            }
        }
    }

    pairs
}
