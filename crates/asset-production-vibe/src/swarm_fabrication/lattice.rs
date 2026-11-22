//! TPMS sheet geometry clipped by an analytic box or sampled shell field.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{
    bounded_number, fields, mesh_output, positive_vector, required, text, vector, Args, AssetHost,
    TypedArtifact, Value,
};
use crate::artifact::FieldLattice;
use qualia_core_db::specialized_libs::computational_geometry::{
    indexed_tetra_surface::{indexed_tetra_surface_capped_into, TetraEdgeSlot},
    Point3,
};
use serde_json::json;
use std::{f64::consts::TAU, mem::size_of, sync::Arc};

const MAX_VOXELS: usize = 384_000;
const MAX_AXIS: usize = 128;
const MAX_VERTICES: usize = 100_000;
const MAX_FACES: usize = 200_000;
const EDGE_SLOTS: usize = 1 << 18;
const TETRA_WORK_LIMIT: usize = 12_000_000;

#[derive(Clone, Copy)]
enum Tpms {
    Gyroid,
    SchwarzP,
    Diamond,
}

impl AssetHost<'_> {
    pub(crate) fn synth_tpms_lattice(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "lattice_kind",
                "unit_cell_mm",
                "wall_thickness_mm",
                "skin_thickness_mm",
                "shell_sdf",
                "box_dimensions_mm",
                "origin_mm",
                "sample_spacing_mm",
            ],
        )?;
        let name = text(required(args, "lattice_kind")?)?;
        let kind = match name {
            "gyroid" => Tpms::Gyroid,
            "schwarz_p" => Tpms::SchwarzP,
            "diamond" => Tpms::Diamond,
            _ => return Err("TPMS lattice_kind must be gyroid, schwarz_p, or diamond".into()),
        };
        let cell = bounded_number(args, "unit_cell_mm", None, 0.2, 1000.0)?;
        let wall = bounded_number(args, "wall_thickness_mm", None, 0.01, cell / 3.0)?;
        let skin = bounded_number(args, "skin_thickness_mm", Some(0.0), 0.0, cell)?;
        let spacing = bounded_number(
            args,
            "sample_spacing_mm",
            None,
            0.005,
            (wall * 0.5).min(cell / 12.0),
        )?;
        if skin > 0.0 && skin < 2.0 * spacing {
            return Err(
                "Sample spacing must resolve the optional skin with at least two intervals".into(),
            );
        }
        let (dimensions, origin, box_size, input) = match (
            args.get("shell_sdf"),
            args.get("box_dimensions_mm"),
        ) {
            (Some(handle), None) => {
                let origin = vector::<3>(required(args, "origin_mm")?)?;
                if origin.iter().any(|v| !(-10_000.0..=10_000.0).contains(v)) {
                    return Err(
                        "TPMS origin_mm is outside the bounded fabrication coordinate range".into(),
                    );
                }
                let TypedArtifact::FieldLattice(field) = self.workspace.typed(handle)? else {
                    return Err("TPMS shell_sdf requires a dense field lattice".into());
                };
                let FieldLattice::DenseF32 { axes, samples } = &**field else {
                    return Err("TPMS shell_sdf requires a dense F32 field".into());
                };
                let [z, y, x] = axes[..] else {
                    return Err("TPMS shell_sdf axes must be [z,y,x]".into());
                };
                let dimensions = [x as usize, y as usize, z as usize];
                if samples.len() != grid_size(dimensions)? || samples.iter().any(|x| !x.is_finite())
                {
                    return Err(
                        "TPMS shell_sdf needs finite metre samples aligned with its axes".into(),
                    );
                }
                (dimensions, origin, None, Some(Arc::clone(field)))
            }
            (None, Some(_)) => {
                if args.contains_key("origin_mm") {
                    return Err("TPMS box mode anchors its envelope at zero; origin_mm applies to shell_sdf".into());
                }
                let size = positive_vector(args, "box_dimensions_mm", spacing * 3.0, 10_000.0)?;
                (
                    size.map(|axis| (axis / spacing).ceil() as usize + 3),
                    [-0.5 * spacing; 3],
                    Some(size),
                    None,
                )
            }
            _ => return Err("Specify either shell_sdf or box_dimensions_mm".into()),
        };
        let count = grid_size(dimensions)?;
        if (0..3)
            .any(|axis| (origin[axis] + spacing * (dimensions[axis] - 1) as f64).abs() > 20_000.0)
        {
            return Err(
                "TPMS sample extent exceeds the bounded fabrication coordinate range".into(),
            );
        }
        self.charge(
            count * 100 + TETRA_WORK_LIMIT,
            count * 12
                + MAX_VERTICES * (size_of::<Point3>() + size_of::<[usize; 2]>() + 12)
                + MAX_FACES * 12
                + EDGE_SLOTS * size_of::<TetraEdgeSlot>(),
        )?;
        let [nx, ny, nz] = dimensions;
        let angular_frequency = TAU / cell;
        let mut combined = Vec::with_capacity(count);
        let mut boundary_inside = 0;
        let mut inside = 0;
        for z in 0..nz {
            if self.cancel.load(std::sync::atomic::Ordering::Relaxed) {
                return Err("Production cancelled".into());
            }
            for y in 0..ny {
                for x in 0..nx {
                    let xyz = [x, y, z].map(|i| i as f64 * spacing);
                    let p = super::add(origin, xyz);
                    let d = if let Some(size) = box_size {
                        box_distance(p, size)
                    } else if let Some(FieldLattice::DenseF32 { samples, .. }) = input.as_deref() {
                        f64::from(samples[combined.len()]) * 1000.0
                    } else {
                        return Err("TPMS source field is unavailable".into());
                    };
                    let (value, gradient) = kind.value_gradient(p.map(|v| v * angular_frequency));
                    let normalizer = angular_frequency * super::norm(gradient).max(0.25);
                    let sheet = value.abs() / normalizer - 0.5 * wall;
                    let filled = if skin > 0.0 {
                        sheet.min(-d - skin)
                    } else {
                        sheet
                    };
                    let distance = d.max(filled);
                    if distance <= 0.0 {
                        inside += 1;
                        if x == 0 || y == 0 || z == 0 || x == nx - 1 || y == ny - 1 || z == nz - 1 {
                            boundary_inside += 1;
                        }
                    }
                    // Keep exact interior zero samples on the exterior side, so the native
                    // extractor has a single deterministic rule for grid-plane coincidence.
                    combined.push(if distance == 0.0 {
                        spacing * 1e-9
                    } else {
                        distance / 1000.0
                    });
                }
            }
        }
        if boundary_inside > 0 || inside == 0 {
            return Err(format!("TPMS closed extraction requires positive grid boundary and nonempty interior; boundary_nonpositive={boundary_inside}, interior_nonpositive={inside}"));
        }
        let mut vertices = vec![Point3::default(); MAX_VERTICES];
        let mut triangles = vec![[0; 3]; MAX_FACES];
        let mut edges = vec![[0; 2]; MAX_VERTICES];
        let mut slots = vec![TetraEdgeSlot::default(); EDGE_SLOTS];
        let [ox, oy, oz] = origin.map(|v| v / 1000.0);
        let stride_m = spacing / 1000.0;
        let (vertex_count, face_count) = indexed_tetra_surface_capped_into(
            &combined,
            dimensions,
            Point3::new(ox, oy, oz),
            Point3::new(stride_m, stride_m, stride_m),
            0.0,
            MAX_VOXELS,
            TETRA_WORK_LIMIT,
            0.0,
            &mut vertices,
            &mut triangles,
            &mut edges,
            &mut slots,
        )
        .map_err(|error| format!("TPMS indexed extraction: {error:?}"))?;
        if face_count == 0 {
            return Err("TPMS sampling produced no surface".into());
        }
        vertices.truncate(vertex_count);
        triangles.truncate(face_count);
        let model = mesh_output(
            self,
            vertices
                .into_iter()
                .map(|p| [p.x as f32, p.y as f32, p.z as f32])
                .collect(),
            triangles,
        )?;
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![nz as u32, ny as u32, nx as u32],
                samples: combined.into_iter().map(|v| v as f32).collect(),
            },
        )))?;
        let metadata = json!({
            "schema": "qualia.tpms-sampled-sheet/v1", "lattice_kind": name,
            "unit_cell_mm": cell, "nominal_wall_thickness_mm": wall, "skin_thickness_mm": skin,
            "thickness_method": "first_order_gradient_normalized_sheet",
            "source": if box_size.is_some() { "analytic_box" } else { "authored_signed_shell_samples" },
            "triangle_count": face_count, "vertex_count": vertex_count,
            "field_origin_mm": origin, "sample_spacing_mm": spacing, "axes": [nz, ny, nx],
        });
        let receipt = self.admit_encoded(
            "json",
            serde_json::to_vec(&metadata).map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record(
            [
                ("mesh".into(), model),
                ("field".into(), field),
                ("receipt".into(), receipt),
            ]
            .into(),
        ))
    }
}

fn grid_size(dimensions: [usize; 3]) -> Result<usize, String> {
    if dimensions.iter().any(|&v| !(3..=MAX_AXIS).contains(&v)) {
        return Err(format!("TPMS axes must have 3 through {MAX_AXIS} samples"));
    }
    let count = dimensions.into_iter().product();
    if count > MAX_VOXELS {
        return Err(format!("TPMS grid exceeds {MAX_VOXELS} samples"));
    }
    Ok(count)
}

impl Tpms {
    fn value_gradient(self, [x, y, z]: [f64; 3]) -> (f64, [f64; 3]) {
        let (sx, cx) = x.sin_cos();
        let (sy, cy) = y.sin_cos();
        let (sz, cz) = z.sin_cos();
        match self {
            Self::Gyroid => (
                sx * cy + sy * cz + sz * cx,
                [cx * cy - sz * sx, cy * cz - sx * sy, cz * cx - sy * sz],
            ),
            Self::SchwarzP => (cx + cy + cz, [-sx, -sy, -sz]),
            Self::Diamond => {
                let value = sx * sy * sz + sx * cy * cz + cx * sy * cz + cx * cy * sz;
                let gradient = [
                    cx * sy * sz + cx * cy * cz - sx * sy * cz - sx * cy * sz,
                    sx * cy * sz - sx * sy * cz + cx * cy * cz - cx * sy * sz,
                    sx * sy * cz - sx * cy * sz - cx * sy * sz + cx * cy * cz,
                ];
                (value, gradient)
            }
        }
    }
}

fn box_distance(p: [f64; 3], size: [f64; 3]) -> f64 {
    let q = std::array::from_fn::<_, 3, _>(|i| (p[i] - size[i] * 0.5).abs() - size[i] * 0.5);
    super::norm(q.map(|v| v.max(0.0))) + q.into_iter().fold(f64::NEG_INFINITY, f64::max).min(0.0)
}
