//! `manifold-bim-csg`
//!
//! UI-agnostic parametric BIM and solid geometry engine:
//! - Work-metered, non-recursive 3D BSP CSG booleans (`csg`)
//! - ISO 10303-21 STEP physical file parser (`step`)
//! - IFC4 / ISO 16739 architectural entity exporter (`ifc`)
//! - Parametric multi-layer walls, stairs, and MEP runs (`parametric`)

#![forbid(unsafe_code)]

pub mod csg;
pub mod ifc;
pub mod parametric;
pub mod step;

pub use csg::{
    area, bounds, cross, dot, intersect, subtract, union, volume, Meter, Poly, Bsp, P3,
    MAX_TRIANGLES,
};
pub use ifc::{export_minimal_building, ifc_guid, step_string, IfcWriter};
pub use parametric::{
    dist_2d, generate_straight_stair, mitre_corner, MepCategory, MepGeom, StairLayout, StairPad,
    StairRun, WallAxis, WallLayer, P2,
};
pub use step::{decode as step_decode, parse as parse_step, Entity as StepEntity, File as StepFile, V as StepVal};
