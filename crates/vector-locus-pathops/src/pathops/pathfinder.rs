//! Pathfinder panel operations over an ordered stack of shapes.
//!
//! Grounded in authoring rights and creative preservation (UDHR Art. 19, 27).

use vector_locus_geometry::{FillRule, PathData};

use crate::pathops::boolean::{boolean, flatten_subpath, BoolOp};

/// A filled shape in a Pathfinder stack.
#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub path: PathData,
    pub rule: FillRule,
    pub key: u64,
}

impl Shape {
    pub fn new(path: PathData, rule: FillRule, key: u64) -> Self {
        Self { path, rule, key }
    }
}

/// The ten standard Pathfinder operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PathfinderOp {
    /// Shape mode: union of all shapes; keeps front-most key.
    Unite,
    /// Shape mode: back-most minus everything in front; keeps back-most key.
    MinusFront,
    /// Shape mode: area covered by all shapes; keeps front-most key.
    Intersect,
    /// Shape mode: area covered by an odd number of shapes; keeps front-most key.
    Exclude,
    /// Split into every non-overlapping face.
    Divide,
    /// Remove hidden parts; one result per visible shape.
    Trim,
    /// Trim, then merge touching/overlapping visible parts with the same key.
    Merge,
    /// Keep visible parts of lower shapes that lie inside the front-most shape.
    Crop,
    /// Split every edge at intersections into open paths.
    Outline,
    /// Front-most minus everything behind; keeps front-most key.
    MinusBack,
}

/// One face of the planar arrangement.
#[derive(Clone, Debug, PartialEq)]
pub struct Region {
    pub path: PathData,
    pub sources: Vec<usize>,
}

/// Execute a Pathfinder operation on a stack of shapes.
pub fn pathfinder(shapes: &[Shape], op: PathfinderOp) -> Vec<Shape> {
    if shapes.is_empty() {
        return Vec::new();
    }
    if shapes.len() == 1 && matches!(op, PathfinderOp::Unite | PathfinderOp::Trim | PathfinderOp::Merge) {
        return shapes.to_vec();
    }

    match op {
        PathfinderOp::Unite => {
            let front_key = shapes.last().map(|s| s.key).unwrap_or(0);
            let mut acc = shapes[0].path.clone();
            for s in &shapes[1..] {
                acc = boolean(&acc, &s.path, BoolOp::Union);
            }
            vec![Shape::new(acc, shapes[0].rule, front_key)]
        }
        PathfinderOp::MinusFront => {
            let back_key = shapes[0].key;
            let mut acc = shapes[0].path.clone();
            for s in &shapes[1..] {
                acc = boolean(&acc, &s.path, BoolOp::Difference);
            }
            vec![Shape::new(acc, shapes[0].rule, back_key)]
        }
        PathfinderOp::MinusBack => {
            let front_key = shapes.last().map(|s| s.key).unwrap_or(0);
            let n = shapes.len();
            let mut acc = shapes[n - 1].path.clone();
            for s in &shapes[..n - 1] {
                acc = boolean(&acc, &s.path, BoolOp::Difference);
            }
            vec![Shape::new(acc, shapes[n - 1].rule, front_key)]
        }
        PathfinderOp::Intersect => {
            let front_key = shapes.last().map(|s| s.key).unwrap_or(0);
            let mut acc = shapes[0].path.clone();
            for s in &shapes[1..] {
                acc = boolean(&acc, &s.path, BoolOp::Intersect);
            }
            vec![Shape::new(acc, shapes[0].rule, front_key)]
        }
        PathfinderOp::Exclude => {
            let front_key = shapes.last().map(|s| s.key).unwrap_or(0);
            let mut acc = shapes[0].path.clone();
            for s in &shapes[1..] {
                acc = boolean(&acc, &s.path, BoolOp::Xor);
            }
            vec![Shape::new(acc, shapes[0].rule, front_key)]
        }
        PathfinderOp::Trim => {
            let mut out = Vec::new();
            for i in 0..shapes.len() {
                let mut p = shapes[i].path.clone();
                for j in (i + 1)..shapes.len() {
                    p = boolean(&p, &shapes[j].path, BoolOp::Difference);
                }
                if !p.is_empty() {
                    out.push(Shape::new(p, shapes[i].rule, shapes[i].key));
                }
            }
            out
        }
        PathfinderOp::Merge => {
            let trimmed = pathfinder(shapes, PathfinderOp::Trim);
            let mut merged: Vec<Shape> = Vec::new();
            for s in trimmed {
                if let Some(existing) = merged.iter_mut().find(|m| m.key == s.key) {
                    existing.path = boolean(&existing.path, &s.path, BoolOp::Union);
                } else {
                    merged.push(s);
                }
            }
            merged
        }
        PathfinderOp::Crop => {
            let n = shapes.len();
            let cutter = &shapes[n - 1].path;
            let mut out = Vec::new();
            for s in &shapes[..n - 1] {
                let cropped = boolean(&s.path, cutter, BoolOp::Intersect);
                if !cropped.is_empty() {
                    out.push(Shape::new(cropped, s.rule, s.key));
                }
            }
            out
        }
        PathfinderOp::Divide | PathfinderOp::Outline => {
            let regs = regions(shapes);
            regs.into_iter()
                .map(|r| {
                    let front_source = r.sources.last().copied().unwrap_or(0);
                    let key = shapes.get(front_source).map(|s| s.key).unwrap_or(0);
                    let rule = shapes.get(front_source).map(|s| s.rule).unwrap_or(FillRule::NonZero);
                    Shape::new(r.path, rule, key)
                })
                .collect()
        }
    }
}

/// Extract all distinct planar faces / regions covered by the shapes.
pub fn regions(shapes: &[Shape]) -> Vec<Region> {
    if shapes.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for (i, s) in shapes.iter().enumerate() {
        let mut curr = s.path.clone();
        for (j, other) in shapes.iter().enumerate() {
            if i != j {
                let inter = boolean(&curr, &other.path, BoolOp::Intersect);
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

#[cfg(test)]
mod tests {
    use super::*;
    use kurbo::Rect;
    use vector_locus_geometry::shapes;

    #[test]
    fn pathfinder_unite_two_shapes() {
        let s1 = Shape::new(shapes::rectangle(Rect::new(0.0, 0.0, 40.0, 40.0)), FillRule::NonZero, 1);
        let s2 = Shape::new(shapes::rectangle(Rect::new(20.0, 0.0, 60.0, 40.0)), FillRule::NonZero, 2);
        let res = pathfinder(&[s1, s2], PathfinderOp::Unite);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].key, 2);
    }

    #[test]
    fn pathfinder_minus_front() {
        let s1 = Shape::new(shapes::rectangle(Rect::new(0.0, 0.0, 40.0, 40.0)), FillRule::NonZero, 1);
        let s2 = Shape::new(shapes::rectangle(Rect::new(20.0, 0.0, 60.0, 40.0)), FillRule::NonZero, 2);
        let res = pathfinder(&[s1, s2], PathfinderOp::MinusFront);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].key, 1);
    }
}
