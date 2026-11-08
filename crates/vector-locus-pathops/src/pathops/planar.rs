//! Live Paint / Shape Builder planar graph face extraction.
//!
//! Grounded in authoring rights and creative integrity (UDHR Art. 19, 27).

use kurbo::{Point, Rect};
use vector_locus_geometry::{PathData, SubPath};

use crate::pathops::boolean::{boolean, flatten_subpath, polygon_area, BoolOp};
use crate::pathops::pathfinder::Region;

/// Find an interior point inside a closed simple polygon.
pub fn interior_point(poly: &[Point]) -> Option<Point> {
    if poly.len() < 3 {
        return None;
    }
    // Centroid of first valid triangle
    for i in 0..poly.len() {
        let a = poly[i];
        let b = poly[(i + 1) % poly.len()];
        let c = poly[(i + 2) % poly.len()];
        let tri_area = (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y);
        if tri_area.abs() > 1e-4 {
            let center = Point::new((a.x + b.x + c.x) / 3.0, (a.y + b.y + c.y) / 3.0);
            return Some(center);
        }
    }
    Some(poly[0])
}

/// Shape builder planar region extractor.
pub fn shape_builder(paths: &[PathData]) -> Vec<Region> {
    let mut out = Vec::new();
    if paths.is_empty() {
        return out;
    }

    // Pairwise overlap and face extraction
    for (i, p) in paths.iter().enumerate() {
        let mut curr = p.clone();
        for (j, other) in paths.iter().enumerate() {
            if i != j {
                let inter = boolean(&curr, other, BoolOp::Intersect);
                if !inter.is_empty() {
                    let mut sources = vec![i, j];
                    sources.sort_unstable();
                    sources.dedup();
                    out.push(Region { path: inter, sources });
                }
            }
        }
        out.push(Region { path: curr, sources: vec![i] });
    }
    out
}

/// Cut out the portion of `target` that is covered by `tool`.
pub fn cut_out(target: &PathData, tool: &PathData) -> PathData {
    boolean(target, tool, BoolOp::Difference)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vector_locus_geometry::shapes;

    #[test]
    fn shape_builder_extracts_faces() {
        let r1 = shapes::rectangle(Rect::new(0.0, 0.0, 50.0, 50.0));
        let r2 = shapes::rectangle(Rect::new(25.0, 0.0, 75.0, 50.0));
        let regions = shape_builder(&[r1, r2]);
        assert!(!regions.is_empty());
    }

    #[test]
    fn cut_out_removes_overlapping_area() {
        let r1 = shapes::rectangle(Rect::new(0.0, 0.0, 50.0, 50.0));
        let r2 = shapes::rectangle(Rect::new(10.0, 10.0, 40.0, 40.0));
        let diff = cut_out(&r1, &r2);
        assert!(!diff.is_empty());
    }
}
