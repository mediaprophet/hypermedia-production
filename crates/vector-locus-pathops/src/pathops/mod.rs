//! Path operations: booleans, pathfinder, shape builder, offset, and editing.

pub mod boolean;
pub mod edit;
pub mod fit;
pub mod offset;
pub mod pathfinder;
pub mod planar;

pub use boolean::{area, boolean, normalize, unite_all, BoolOp, DEFAULT_PRECISION};
pub use edit::{
    add_anchor_points, average, join, remove_redundant_points, simplify, simplify_with, smooth,
    split_into_grid, AverageAxis, SimplifyOptions,
};
pub use fit::{end_tangent, fit_cubics, fit_single, start_tangent};
pub use offset::{offset_path, outline_stroke, stroke_region, Cap, Join};
pub use pathfinder::{pathfinder, regions, PathfinderOp, Region, Shape};
pub use planar::{cut_out, interior_point, shape_builder};
