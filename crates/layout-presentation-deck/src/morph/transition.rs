//! Slide transition blending and morph frame generation.

use crate::model::master::Slide;
use crate::model::shape::{Shape, TransitionKind};
use crate::model::types::{ShapeId, Xfrm};
use super::interpolate::{clamp, morph_xfrm};
use super::match_pairs::morph_pairs;

#[derive(Clone, Debug, PartialEq)]
pub struct MorphPath {
    pub id: ShapeId,
    pub from_path: Option<String>,
    pub to_path: Option<String>,
    pub from_box: Xfrm,
    pub to_box: Xfrm,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MorphText {
    pub old_id: ShapeId,
    pub new_id: ShapeId,
    pub from_box: Xfrm,
    pub to_box: Xfrm,
    pub by_chars: bool,
}

#[derive(Clone, Debug, Default)]
pub struct MorphFrame {
    pub shapes: Vec<(Shape, bool)>, // (shape, is_old_slide)
    pub opacity: Vec<(ShapeId, f64)>,
    pub mix: f64,
    pub text: Vec<MorphText>,
    pub paths: Vec<MorphPath>,
}

impl MorphFrame {
    pub fn opacity_of(&self, id: ShapeId) -> f64 {
        self.opacity.iter().find(|(s, _)| *s == id).map(|(_, o)| *o).unwrap_or(1.0)
    }
}

/// Compute a morph transition frame between slide `a` and slide `b` at progress `t` (0.0 .. 1.0).
pub fn build_morph_frame(a: &Slide, b: &Slide, t: f64) -> MorphFrame {
    let t = clamp(t, 0.0, 1.0);
    let pairs = morph_pairs(a, b);

    let mut frame = MorphFrame {
        mix: t,
        ..Default::default()
    };

    // 1. Matched pairs move and interpolate
    for &(id_a, id_b) in &pairs {
        if let (Some(sa), Some(sb)) = (a.shape_by_id(id_a), b.shape_by_id(id_b)) {
            let mut interpolated = sb.clone();
            interpolated.xfrm = morph_xfrm(sa.xfrm, sb.xfrm, t);

            frame.shapes.push((interpolated, false));
            frame.opacity.push((sb.id, 1.0));

            if sa.geom_path.is_some() || sb.geom_path.is_some() {
                frame.paths.push(MorphPath {
                    id: sb.id,
                    from_path: sa.geom_path.clone(),
                    to_path: sb.geom_path.clone(),
                    from_box: sa.xfrm,
                    to_box: sb.xfrm,
                });
            }
        }
    }

    // 2. Unmatched shapes on slide A fade out
    for sa in &a.shapes {
        if !pairs.iter().any(|(id_a, _)| *id_a == sa.id) {
            frame.shapes.push((sa.clone(), true));
            frame.opacity.push((sa.id, 1.0 - t));
        }
    }

    // 3. Unmatched shapes on slide B fade in
    for sb in &b.shapes {
        if !pairs.iter().any(|(_, id_b)| *id_b == sb.id) {
            frame.shapes.push((sb.clone(), false));
            frame.opacity.push((sb.id, t));
        }
    }

    frame
}
