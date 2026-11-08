//! # Vector Locus Pathops Engine
//!
//! A pure Rust, UI-agnostic categorical locus domain crate providing cubic Bézier
//! boolean operations (Pathfinder), Shape Builder / Live Paint planar graph face
//! extraction, offset/outline strokes, least-squares curve fitting, non-destructive
//! vector distortion effects, 15 warp envelopes, and 3D surface of revolution.
//!
//! Grounded in international human rights guarantees of artistic freedom, authoring
//! integrity, and scientific collaboration (UDHR Articles 19 & 27; ICESCR Article 15)
//! and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod effects;
pub mod pathops;
pub mod three_d;

pub use effects::{
    catmull_rom, free_distort, map_nonlinear, noise, pucker_bloat, roughen, scribble, tweak, twist, warp, warp_point,
    zig_zag, WarpStyle,
};
pub use pathops::{
    add_anchor_points, area, average, boolean, cut_out, fit_cubics, fit_single, interior_point, join, normalize,
    offset_path, outline_stroke, pathfinder, regions, remove_redundant_points, shape_builder, simplify,
    simplify_with, smooth, split_into_grid, stroke_region, unite_all, AverageAxis, BoolOp, Cap, Join, PathfinderOp,
    Region, Shape, SimplifyOptions, DEFAULT_PRECISION,
};
pub use three_d::{
    extrude, revolve, visible_faces, BevelStyle, Edge, Extrude, Face, Revolve, VisibleFace, V3,
};

#[cfg(test)]
mod tests {
    use super::*;
    use kurbo::Rect;
    use vector_locus_geometry::shapes;

    #[test]
    fn full_pathops_pipeline_smoke() {
        let r1 = shapes::rectangle(Rect::new(0.0, 0.0, 60.0, 60.0));
        let r2 = shapes::rectangle(Rect::new(30.0, 0.0, 90.0, 60.0));

        // 1. Boolean operation
        let union_path = boolean(&r1, &r2, BoolOp::Union);
        assert!(!union_path.is_empty());

        // 2. Warp envelope
        let b = union_path.bounds().unwrap();
        let warped = warp(&union_path, b, WarpStyle::Arc, &serde_json::json!({"bend": 30.0}));
        assert!(!warped.is_empty());

        // 3. 3D extrusion
        let faces = extrude(&warped, Extrude::default()).unwrap();
        assert!(!faces.is_empty());
    }
}
