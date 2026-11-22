//! Offline stock visualization for a centered rectangular pocket.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{bounded_number, positive_vector, AssetHost, Value};
use crate::{artifact::FieldLattice, fields, required, text, vector, Args, TypedArtifact};
use qualia_core_db::specialized_libs::computational_geometry::{sdf_ops::sdf_rounded_box, Point3};
use serde_json::json;
use std::{f64::consts::PI, sync::Arc};

const MAX_PREVIEW_SAMPLES: usize = 384_000;
const MAX_PREVIEW_AXIS: usize = 256;

impl AssetHost<'_> {
    pub(crate) fn plan_cnc_subtractive_multiaxis(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "strategy",
                "stock_dimensions_mm",
                "pocket_dimensions_mm",
                "pocket_depth_mm",
                "corner_radius_mm",
                "sample_spacing_mm",
            ],
        )?;
        if text(required(args, "strategy")?)? != "rectangular_pocket_preview" {
            return Err("Expected strategy rectangular_pocket_preview".into());
        }
        let stock = positive_vector(args, "stock_dimensions_mm", 0.1, 1000.0)?;
        let [width, length] = vector::<2>(required(args, "pocket_dimensions_mm")?)?;
        if !(0.05..stock[0]).contains(&width) || !(0.05..stock[1]).contains(&length) {
            return Err("The centered pocket must fit inside the stock planform".into());
        }
        let depth = bounded_number(args, "pocket_depth_mm", None, 0.01, stock[2])?;
        let radius = bounded_number(args, "corner_radius_mm", None, 0.0, width.min(length) * 0.5)?;
        let side_clearance = [(stock[0] - width) * 0.5, (stock[1] - length) * 0.5];
        let floor = stock[2] - depth;
        let minimum_feature = side_clearance[0]
            .min(side_clearance[1])
            .min(width)
            .min(length)
            .min(floor)
            .min(depth);
        let spacing = bounded_number(
            args,
            "sample_spacing_mm",
            None,
            0.001,
            minimum_feature * 0.5,
        )?;
        let [nx, ny, nz] = stock.map(|extent| (extent / spacing).ceil() as usize + 3);
        if nx.max(ny).max(nz) > MAX_PREVIEW_AXIS {
            return Err("Stock preview exceeds the field lattice axis limit".into());
        }
        let samples = nx * ny * nz;
        if samples > MAX_PREVIEW_SAMPLES {
            return Err("Stock preview exceeds the field lattice budget".into());
        }
        self.charge(samples * 64, samples * std::mem::size_of::<f32>() + 4096)?;

        let origin = -spacing;
        let stock_center = Point3::new(stock[0] * 0.5, stock[1] * 0.5, stock[2] * 0.5);
        let pocket_center = [stock_center.x, stock_center.y];
        let mut values = Vec::with_capacity(samples);
        for z in 0..nz {
            self.charge(0, 0)?;
            for y in 0..ny {
                for x in 0..nx {
                    let p = Point3::new(
                        origin + x as f64 * spacing,
                        origin + y as f64 * spacing,
                        origin + z as f64 * spacing,
                    );
                    let stock_d = sdf_rounded_box(p, stock_center, stock_center, 0.0);
                    let xy = [p.x - pocket_center[0], p.y - pocket_center[1]];
                    let qx = xy[0].abs() - (width * 0.5 - radius);
                    let qy = xy[1].abs() - (length * 0.5 - radius);
                    let lateral_d = qx.max(0.0).hypot(qy.max(0.0)) + qx.max(qy).min(0.0) - radius;
                    let pocket_d = lateral_d.max(floor - p.z);
                    values.push((stock_d.max(-pocket_d) / 1000.0) as f32);
                }
            }
        }
        let removed = (width * length - (4.0 - PI) * radius * radius) * depth;
        let receipt = json!({
            "schema": "qualia.offline-pocket-preview/v1",
            "stock_dimensions_mm": stock,
            "pocket_dimensions_mm": [width, length],
            "pocket_depth_mm": depth,
            "corner_radius_mm": radius,
            "side_clearance_mm": side_clearance,
            "floor_thickness_mm": floor,
            "analytic_removed_volume_mm3": removed,
            "analytic_residual_volume_mm3": stock.into_iter().product::<f64>() - removed,
            "field_origin_mm": [origin, origin, origin],
            "sample_spacing_mm": spacing,
            "axes": [nz, ny, nx],
            "field_sample_unit": "m",
            "distance_is_csg_bound": true,
            "fixture_and_holder_clearance_evaluated": false,
        });
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![nz as u32, ny as u32, nx as u32],
                samples: values,
            },
        )))?;
        let [dx, dy] = side_clearance;
        let [sx, sy, _] = stock;
        let diagram = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="640" height="480" viewBox="0 0 {sx} {sy}"><title>Centered pocket preview</title><desc>Top view. Stock {sx} mm by {sy} mm. Pocket depth {depth} mm.</desc><rect width="{sx}" height="{sy}" fill="#64748b"/><rect x="{dx}" y="{dy}" width="{width}" height="{length}" rx="{radius}" fill="#252f42"/></svg>"##
        );
        let preview = self.admit_encoded("svg", diagram.into_bytes())?;
        let receipt = self.admit_encoded(
            "json",
            serde_json::to_vec(&receipt).map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record(
            [
                ("residual_field".into(), field),
                ("preview".into(), preview),
                ("receipt".into(), receipt),
            ]
            .into(),
        ))
    }
}
