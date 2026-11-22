//! Sliding dovetail partition of a closed axis-aligned rectangular blank.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{
    bounded_number, bounds, check_closed_shell, cross, fields, mesh_handle, mesh_output, norm,
    positive_vector, required, sub, text, vector, Args, AssetHost, Value,
};
use crate::{integer, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph};
use qualia_core_db::{
    render::assets::Mesh,
    specialized_libs::computational_geometry::{triangulate_ear_clipping, Point2},
};
use sha2::{Digest, Sha256};

const CONTEXT: &str = "urn:qualia:contexts:rectangular-sliding-dovetail-v1";
const BOX_FACE_CAP: usize = 1024;
const COORDINATE_EPSILON_MM: f64 = 1e-6;

struct Pair {
    axis: usize,
    slide_axis: usize,
    split_mm: f64,
    male: Mesh,
    female: Mesh,
}

impl AssetHost<'_> {
    pub(crate) fn split_joinery_assembly(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mesh",
                "joinery_kind",
                "build_envelope_mm",
                "clearance_mm",
                "shrinkage_xyz",
                "split_axis",
                "rail_neck_width_mm",
                "rail_depth_mm",
                "rail_side_angle_deg",
                "minimum_stock_mm",
            ],
        )?;
        if text(required(args, "joinery_kind")?)? != "sliding_dovetail" {
            return Err("Rectangular partition supports sliding_dovetail".into());
        }
        let envelope = positive_vector(args, "build_envelope_mm", 0.1, 10_000.0)?;
        let clearance = bounded_number(args, "clearance_mm", None, 0.01, 5.0)?;
        let neck = bounded_number(args, "rail_neck_width_mm", None, 0.1, 1000.0)?;
        let depth = bounded_number(args, "rail_depth_mm", None, 0.1, 1000.0)?;
        let slope = bounded_number(args, "rail_side_angle_deg", None, 5.0, 35.0)?
            .to_radians()
            .tan();
        let stock = bounded_number(args, "minimum_stock_mm", None, 0.1, 100.0)?;
        if depth <= 2.0 * clearance {
            return Err("Dovetail depth requires more than twice the specified clearance".into());
        }
        let shrink = args
            .get("shrinkage_xyz")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0; 3]);
        if shrink.iter().any(|&value| !(0.0..=0.05).contains(&value)) {
            return Err("Per-axis shrinkage must be a fractional loss between 0 and 0.05".into());
        }
        let selected_axis = args
            .get("split_axis")
            .map(|v| integer(v, "split_axis"))
            .transpose()?;
        if selected_axis.is_some_and(|v| v > 2) {
            return Err("split_axis must be 0 for X, 1 for Y, or 2 for Z".into());
        }
        let source = mesh_handle(self, required(args, "mesh")?)?;
        let source_mesh = &source.surface.mesh;
        if source_mesh.triangles.len() > BOX_FACE_CAP
            || source.protected_faces.iter().any(|&mask| mask != 0)
        {
            return Err(
                "Rectangular blank requires at most 1024 faces and no face-transform protection"
                    .into(),
            );
        }
        check_closed_shell(self, source_mesh)?;
        let (min, max) = check_box(source_mesh)?;
        let size = sub(max, min);
        let nominal_capacity =
            std::array::from_fn::<_, 3, _>(|axis| envelope[axis] * (1.0 - shrink[axis]));
        let mut axes = [0, 1, 2];
        axes.sort_by(|&a, &b| {
            (size[b] / nominal_capacity[b]).total_cmp(&(size[a] / nominal_capacity[a]))
        });
        let mut candidates = axes
            .into_iter()
            .filter(|axis| selected_axis.is_none_or(|chosen| chosen == *axis));
        let machined = candidates.find_map(|axis| {
            let b = (axis + 1) % 3;
            let c = (axis + 2) % 3;
            if size[b] > nominal_capacity[b] || size[c] > nominal_capacity[c] {
                return None;
            }
            let radius = 0.5 * neck;
            let pocket_allowance = clearance * (1.0 + slope * slope).sqrt();
            let half_width_at_tip = radius + slope * (depth + clearance) + pocket_allowance;
            if 2.0 * (half_width_at_tip + stock) > size[b] {
                return None;
            }
            let lo = (max[axis] - nominal_capacity[axis] - 0.5 * clearance)
                .max(min[axis] + stock + 0.5 * clearance);
            let hi = (min[axis] + nominal_capacity[axis] - depth + 0.5 * clearance)
                .min(max[axis] - stock - depth - 0.5 * clearance);
            if lo > hi {
                return None;
            }
            Some((axis, b, c, 0.5 * (lo + hi), radius, pocket_allowance))
        });
        let Some((a, b, c, split, neck_half, allowance)) = machined else {
            return Err(format!("No candidate rectangular split satisfies the requested envelope, stock and dovetail bounds; blank_mm={size:?}, envelope_mm={envelope:?}"));
        };
        let male_plane = split - 0.5 * clearance;
        let female_plane = split + 0.5 * clearance;
        let middle = 0.5 * (min[b] + max[b]);
        let tip_a = male_plane + depth;
        let tip_half = neck_half + slope * depth;
        let pocket_tip_a = tip_a + clearance;
        let pocket_entry_half = neck_half + slope * clearance + allowance;
        let pocket_tip_half = neck_half + slope * (depth + clearance) + allowance;
        let male_profile = [
            [min[a], min[b]],
            [male_plane, min[b]],
            [male_plane, middle - neck_half],
            [tip_a, middle - tip_half],
            [tip_a, middle + tip_half],
            [male_plane, middle + neck_half],
            [male_plane, max[b]],
            [min[a], max[b]],
        ];
        let female_profile = [
            [female_plane, min[b]],
            [max[a], min[b]],
            [max[a], max[b]],
            [female_plane, max[b]],
            [female_plane, middle + pocket_entry_half],
            [pocket_tip_a, middle + pocket_tip_half],
            [pocket_tip_a, middle - pocket_tip_half],
            [female_plane, middle - pocket_entry_half],
        ];

        let pair = Pair {
            axis: a,
            slide_axis: c,
            split_mm: split,
            male: extruded_profile(&male_profile, [a, b, c], min, max[c], shrink, clearance)?,
            female: extruded_profile(&female_profile, [a, b, c], min, max[c], shrink, clearance)?,
        };
        for part in [&pair.male, &pair.female] {
            check_closed_shell(self, part)?;
            let extent_mm = std::array::from_fn::<_, 3, _>(|i| {
                (f64::from(part.max[i]) - f64::from(part.min[i])) * 1000.0
            });
            if (0..3).any(|axis| extent_mm[axis] > envelope[axis] + 1e-5) {
                return Err(format!("Stored compensated coordinates exceed build envelope; extents_mm={extent_mm:?}"));
            }
        }

        let root = mesh_digest(source_mesh);
        let male_iri = mesh_digest(&pair.male);
        let female_iri = mesh_digest(&pair.female);
        let mut receipt = ReceiptGraph::new(CONTEXT);
        for (part, role) in [
            (&male_iri, "urn:qualia:print:dovetail-rail"),
            (&female_iri, "urn:qualia:print:dovetail-socket"),
        ] {
            receipt.fact(part, "http://www.w3.org/ns/prov#wasDerivedFrom", &root)?;
            receipt.fact(part, "urn:qualia:print:jointRole", role)?;
        }
        receipt.fact(&male_iri, "urn:qualia:print:dovetailMate", &female_iri)?;
        let q42 = receipt.seal(self.limits.resource_bytes.min(1024 * 1024))?;
        let male = mesh_output(self, pair.male.positions, pair.male.triangles)?;
        let female = mesh_output(self, pair.female.positions, pair.female.triangles)?;
        let receipt = self.admit_encoded("q42", q42)?;
        Ok(Value::Record(
            [
                ("parts".into(), Value::List(vec![male, female])),
                ("receipt".into(), receipt),
                ("split_axis".into(), Value::U64(pair.axis as u64)),
                ("slide_axis".into(), Value::U64(pair.slide_axis as u64)),
                ("split_plane_mm".into(), Value::F64(pair.split_mm)),
                ("nominal_flank_clearance_mm".into(), Value::F64(clearance)),
            ]
            .into(),
        ))
    }
}

fn check_box(mesh: &Mesh) -> Result<([f64; 3], [f64; 3]), String> {
    let (min, max) = bounds(&mesh.positions);
    let min = min.map(|v| f64::from(v) * 1000.0);
    let max = max.map(|v| f64::from(v) * 1000.0);
    let size = sub(max, min);
    if size.iter().any(|&v| v < 0.1) {
        return Err("Rectangular blank edges must be at least 0.1 mm".into());
    }
    let mut areas = [[0.0_f64; 2]; 3];
    for (i, tri) in mesh.triangles.iter().enumerate() {
        let [a, b, c] = tri.map(|id| mesh.positions[id as usize].map(|v| f64::from(v) * 1000.0));
        let normal = cross(sub(b, a), sub(c, a));
        let area = 0.5 * norm(normal);
        if area == 0.0 {
            return Err(format!("Blank has degenerate face {i}"));
        }
        let wall = (0..3)
            .find_map(|axis| {
                [min[axis], max[axis]]
                    .into_iter()
                    .enumerate()
                    .find_map(|(which, plane)| {
                        let outward = if which == 0 { -1.0 } else { 1.0 };
                        ([a, b, c]
                            .iter()
                            .all(|v| (v[axis] - plane).abs() <= COORDINATE_EPSILON_MM)
                            && outward * normal[axis] > 0.0)
                            .then_some((axis, which))
                    })
            })
            .ok_or_else(|| format!("Blank face {i} is not on an outward axis-aligned boundary"))?;
        areas[wall.0][wall.1] += area;
    }
    for axis in 0..3 {
        let expected = size[(axis + 1) % 3] * size[(axis + 2) % 3];
        let error = areas[axis].map(|area| (area - expected).abs());
        if error.into_iter().any(|e| e > expected * 1e-6) {
            return Err(format!(
                "Blank wall area mismatch on axis {axis}; absolute residuals_mm2={error:?}"
            ));
        }
    }
    Ok((min, max))
}

fn extruded_profile(
    profile: &[[f64; 2]],
    axes: [usize; 3],
    datum: [f64; 3],
    upper: f64,
    shrink: [f64; 3],
    clearance: f64,
) -> Result<Mesh, String> {
    let polygon = profile
        .iter()
        .map(|p| Point2::new(p[0], p[1]))
        .collect::<Vec<_>>();
    let cap = triangulate_ear_clipping(&polygon);
    if cap.len() != profile.len() - 2 || cap.iter().any(|t| t.signed_area() <= 0.0) {
        return Err(
            "Dovetail section could not be triangulated as a simple positive-area profile".into(),
        );
    }
    let n = polygon.len() as u32;
    let mut positions = Vec::with_capacity(2 * n as usize);
    for height in [datum[axes[2]], upper] {
        for p in &polygon {
            let mut v = [0.0; 3];
            let mut error = [0.0; 3];
            for (axis, value) in axes.into_iter().zip([p.x, p.y, height]) {
                let compensated_mm = datum[axis] + (value - datum[axis]) / (1.0 - shrink[axis]);
                v[axis] = (compensated_mm / 1000.0) as f32;
                error[axis] = f64::from(v[axis]) * 1000.0 - compensated_mm;
            }
            if norm(error) > clearance * 0.125 {
                return Err(format!("Metre-space vertex storage exceeds the dovetail quantization margin; offset_error_mm={:e}", norm(error)));
            }
            positions.push(v);
        }
    }
    let mut triangles = Vec::new();
    for cap in cap {
        let lookup = |point| {
            polygon
                .iter()
                .position(|&v| v == point)
                .map(|i| i as u32)
                .ok_or("Dovetail cap vertex identity changed")
        };
        let [a, b, c] = [lookup(cap.a)?, lookup(cap.b)?, lookup(cap.c)?];
        triangles.extend([[c, b, a], [a + n, b + n, c + n]]);
    }
    for a in 0..n {
        let b = (a + 1) % n;
        triangles.extend([[a, b, b + n], [a, b + n, a + n]]);
    }
    let (min, max) = bounds(&positions);
    Ok(Mesh {
        positions,
        triangles,
        min,
        max,
    })
}

fn mesh_digest(mesh: &Mesh) -> String {
    let mut hash = Sha256::new();
    hash.update(b"qualia:rectangular-joinery-metre-mesh:v1\0");
    hash.update((mesh.positions.len() as u64).to_le_bytes());
    hash.update((mesh.triangles.len() as u64).to_le_bytes());
    for vertex in mesh.positions.iter().flatten() {
        hash.update(vertex.to_le_bytes());
    }
    for index in mesh.triangles.iter().flatten() {
        hash.update(index.to_le_bytes());
    }
    format!("urn:sha256:{:x}", hash.finalize())
}
