//! Scribble hatching lines generator across vector shapes.
//!
//! Grounded in authoring rights and artistic expression (UDHR Art. 19, 27).

use serde_json::Value;
use vector_locus_geometry::{Affine, PathData, Point, Rect, SubPath};

use crate::effects::util::{catmull_rom, noise, num, seed};

fn flatten(path: &PathData) -> Vec<Vec<Point>> {
    let mut polys: Vec<Vec<Point>> = Vec::new();
    kurbo::flatten(path.to_bezpath(), 0.25, |el| match el {
        kurbo::PathEl::MoveTo(p) => polys.push(vec![p]),
        kurbo::PathEl::LineTo(p) => {
            if let Some(l) = polys.last_mut() {
                l.push(p);
            }
        }
        kurbo::PathEl::ClosePath => {
            if let Some(l) = polys.last_mut() {
                if let Some(&f) = l.first() {
                    l.push(f);
                }
            }
        }
        _ => {}
    });
    polys
}

/// Scribble: generates zigzag hatching across a shape.
pub fn scribble(path: &PathData, b: Rect, p: &Value) -> PathData {
    let angle = num(p, "angle", 30.0).to_radians();
    let overlap = num(p, "overlap", 0.0);
    let width = num(p, "strokeWidth", 3.0).clamp(0.01, 1000.0);
    let curvy = num(p, "curviness", 5.0).clamp(0.0, 100.0) / 100.0;
    let spacing = num(p, "spacing", 5.0).max(0.1);
    let variation = num(p, "variation", 0.5).clamp(0.0, 100.0);
    let seed_val = seed(p);
    let c = b.center();
    let to_local = Affine::translate(c.to_vec2()) * Affine::rotate(angle) * Affine::translate(-c.to_vec2());
    let from_local = to_local.inverse();

    let polys: Vec<Vec<Point>> = flatten(path)
        .into_iter()
        .map(|pl| pl.into_iter().map(|q| from_local * q).collect())
        .collect();

    let (mut y0, mut y1) = (f64::INFINITY, f64::NEG_INFINITY);
    for q in polys.iter().flatten() {
        y0 = y0.min(q.y);
        y1 = y1.max(q.y);
    }
    if !y0.is_finite() || y1 - y0 < 1e-9 {
        return PathData::default();
    }

    let step = spacing.max((y1 - y0) / 2000.0);
    let mut pts: Vec<Point> = Vec::new();
    let mut y = y0 + step / 2.0;
    let mut row = 0u64;

    while y < y1 {
        let mut xs: Vec<f64> = Vec::new();
        for pl in &polys {
            for w in pl.windows(2) {
                let (a, bq) = (w[0], w[1]);
                if (a.y <= y && bq.y > y) || (bq.y <= y && a.y > y) {
                    xs.push(a.x + (y - a.y) / (bq.y - a.y) * (bq.x - a.x));
                }
            }
        }
        xs.sort_by(f64::total_cmp);
        let mut spans: Vec<(f64, f64)> = xs
            .chunks_exact(2)
            .map(|s| (s[0] - overlap, s[1] + overlap))
            .collect();
        if row % 2 == 1 {
            spans.reverse();
        }
        for (a, bx) in spans {
            let jitter = |k: u64| variation * noise(seed_val, row, k, 3);
            let (l, r) = (
                Point::new(a + jitter(0), y + jitter(1)),
                Point::new(bx + jitter(2), y + jitter(3)),
            );
            if row % 2 == 0 {
                pts.push(l);
                pts.push(r);
            } else {
                pts.push(r);
                pts.push(l);
            }
        }
        row += 1;
        y += step;
    }

    if pts.len() < 2 {
        return PathData::default();
    }

    let line = if curvy > 0.0 {
        catmull_rom(&pts, false, curvy * 2.0)
    } else {
        SubPath::polyline(&pts, false)
    };
    let mut center = PathData::single(line);
    center.transform(to_local);

    let stroke = kurbo::Stroke::new(width).with_join(kurbo::Join::Round).with_caps(kurbo::Cap::Round);
    let outline = kurbo::stroke(center.to_bezpath(), &stroke, &kurbo::StrokeOpts::default(), 0.05);
    PathData::from_bezpath(&outline)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vector_locus_geometry::shapes;

    #[test]
    fn scribble_generates_hatching() {
        let r = shapes::rectangle(Rect::new(0.0, 0.0, 100.0, 100.0));
        let b = r.bounds().unwrap();
        let hatched = scribble(&r, b, &serde_json::json!({"spacing": 10.0, "strokeWidth": 2.0}));
        assert!(!hatched.is_empty());
    }
}
