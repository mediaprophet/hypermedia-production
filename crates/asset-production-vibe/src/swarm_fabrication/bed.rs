//! Deterministic orthogonal part orientations and clearance-aware bed packing.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{
    bounded_number, cross, fields, mesh_handle, mul, norm, positive_vector, required, sub, Args,
    AssetHost, Value,
};
use crate::list;
use serde_json::json;
use std::cmp::Ordering;

const MAX_PARTS: usize = 24;

#[derive(Clone, Copy)]
struct Pose {
    axes: [usize; 3],
    signs: [f64; 3],
    min: [f64; 3],
    size: [f64; 3],
    area: f64,
    protected_area: f64,
}

impl Pose {
    fn rotation(self) -> [[i32; 3]; 3] {
        let mut rotation = [[0; 3]; 3];
        for i in 0..3 {
            rotation[i][self.axes[i]] = self.signs[i] as i32;
        }
        rotation
    }
}

struct Placement {
    original_index: usize,
    pose: Pose,
    xy: [f64; 2],
}

impl AssetHost<'_> {
    pub(crate) fn orient_nest_tree_support(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "parts",
                "build_plate_mm",
                "max_overhang_deg",
                "clearance_mm",
            ],
        )?;
        let bed = positive_vector(args, "build_plate_mm", 0.1, 10_000.0)?;
        let clearance = bounded_number(args, "clearance_mm", Some(2.0), 0.0, 1000.0)?;
        let threshold = bounded_number(args, "max_overhang_deg", None, 0.0, 90.0)?
            .to_radians()
            .sin();
        if clearance * 2.0 >= bed[0].min(bed[1]) {
            return Err("Clearance leaves no usable build-plate rectangle".into());
        }
        let parts = list(required(args, "parts")?)?;
        if !(1..=MAX_PARTS).contains(&parts.len()) {
            return Err(format!(
                "Orthogonal AABB packing requires 1 through {MAX_PARTS} parts"
            ));
        }
        let meshes = parts
            .iter()
            .map(|v| mesh_handle(self, v))
            .collect::<Result<Vec<_>, _>>()?;
        let faces = meshes
            .iter()
            .map(|m| m.surface.mesh.triangles.len())
            .sum::<usize>();
        self.charge(faces * 144 + 48 * parts.len().pow(4), faces * 64)?;

        let mut candidates = Vec::new();
        for (i, mesh) in meshes.iter().enumerate() {
            let positions = &mesh.surface.mesh.positions;
            let v: Vec<_> = positions
                .iter()
                .map(|v| v.map(|p| f64::from(p) * 1000.0))
                .collect();
            let (min, max) = v.iter().fold(
                ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]),
                |(low, high), v| {
                    (
                        std::array::from_fn(|i| low[i].min(v[i])),
                        std::array::from_fn(|i| high[i].max(v[i])),
                    )
                },
            );
            let size = sub(max, min);
            if size.into_iter().any(|value| value <= 0.0) {
                return Err(format!(
                    "Part {i} does not have a nonzero three-dimensional extent"
                ));
            }
            // Signed up directions only. In-plane rotation changes packing, while overhang
            // and protected-face scoring depend solely on the third row of the pose.
            let mut areas = [[(0.0, 0.0); 2]; 3];
            for (face, triangle) in mesh.surface.mesh.triangles.iter().enumerate() {
                let [a, b, c] = triangle.map(|id| v[id as usize]);
                let cross = cross(sub(b, a), sub(c, a));
                let twice_area = norm(cross);
                if twice_area <= 0.0 {
                    return Err(format!("Part {i} has a degenerate face at {face}"));
                }
                for axis in 0..3 {
                    for (sign_index, sign) in [-1.0, 1.0].into_iter().enumerate() {
                        let bed_plane = if sign < 0.0 { max[axis] } else { min[axis] };
                        let on_bed = [a, b, c]
                            .iter()
                            .all(|vertex| (vertex[axis] - bed_plane).abs() <= 1e-5);
                        if !on_bed && cross[axis] * sign / twice_area < -threshold {
                            let area = 0.5 * twice_area;
                            areas[axis][sign_index].0 += area;
                            if mesh.protected_faces.get(face).is_some_and(|v| *v != 0) {
                                areas[axis][sign_index].1 += area;
                            }
                        }
                    }
                }
            }
            let options = orientations(min, max, areas)
                .into_iter()
                .filter(|pose| {
                    pose.size[0] + 2.0 * clearance <= bed[0]
                        && pose.size[1] + 2.0 * clearance <= bed[1]
                        && pose.size[2] <= bed[2]
                })
                .collect::<Vec<_>>();
            if options.is_empty() {
                return Err(format!("Part {i} has no orthogonal orientation inside the build envelope; extents_mm={size:?}"));
            }
            let minimum_footprint = options
                .iter()
                .map(|p| p.size[0] * p.size[1])
                .fold(f64::INFINITY, f64::min);
            candidates.push((i, minimum_footprint, options));
        }
        candidates.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));

        let mut placed: Vec<Placement> = Vec::new();
        for (index, _, options) in candidates {
            let mut xs = vec![clearance];
            let mut ys = vec![clearance];
            let mut used = [0.0_f64; 2];
            for item in &placed {
                let end = end_xy(item.xy, item.pose.size);
                xs.push(end[0] + clearance);
                ys.push(end[1] + clearance);
                used = [used[0].max(end[0]), used[1].max(end[1])];
            }
            xs.sort_by(f64::total_cmp);
            ys.sort_by(f64::total_cmp);
            xs.dedup();
            ys.dedup();
            let mut best: Option<([f64; 7], Pose, [f64; 2])> = None;
            for pose in options {
                for &y in &ys {
                    for &x in &xs {
                        let xy = [x, y];
                        let end = end_xy(xy, pose.size);
                        if end[0] + clearance > bed[0] || end[1] + clearance > bed[1] {
                            continue;
                        }
                        if placed.iter().any(|other| {
                            let other_end = end_xy(other.xy, other.pose.size);
                            (0..2).all(|a| {
                                end[a] + clearance > other.xy[a] && other_end[a] + clearance > xy[a]
                            })
                        }) {
                            continue;
                        }
                        let occupied_area = end[0].max(used[0]) * end[1].max(used[1]);
                        let key = [
                            pose.protected_area,
                            pose.area,
                            occupied_area,
                            pose.size[2],
                            y,
                            x,
                            pose.size[0],
                        ];
                        if best.as_ref().is_none_or(|(old, _, _)| better(&key, old)) {
                            best = Some((key, pose, xy));
                        }
                    }
                }
            }
            let Some((_, pose, xy)) = best else {
                return Err(format!("Part {index} has no feasible AABB placement at clearance {clearance} mm after {} placements", placed.len()));
            };
            placed.push(Placement {
                original_index: index,
                pose,
                xy,
            });
        }
        placed.sort_by_key(|entry| entry.original_index);
        let placements = placed
            .iter()
            .map(|entry| {
                let pose = entry.pose;
                let translation = sub([entry.xy[0], entry.xy[1], 0.0], pose.min);
                Value::Record(
                    [
                        ("part_index".into(), Value::U64(entry.original_index as u64)),
                        (
                            "rotation_row_major".into(),
                            Value::List(
                                pose.rotation()
                                    .into_iter()
                                    .flatten()
                                    .map(|v| Value::I64(i64::from(v)))
                                    .collect(),
                            ),
                        ),
                        (
                            "translation_m".into(),
                            Value::List(
                                mul(translation, 0.001)
                                    .into_iter()
                                    .map(Value::F64)
                                    .collect(),
                            ),
                        ),
                        (
                            "bounds_mm".into(),
                            Value::List(pose.size.into_iter().map(Value::F64).collect()),
                        ),
                        ("downward_face_area_mm2".into(), Value::F64(pose.area)),
                        (
                            "protected_downward_face_area_mm2".into(),
                            Value::F64(pose.protected_area),
                        ),
                    ]
                    .into(),
                )
            })
            .collect();
        let metadata = json!({
            "schema": "qualia.orthogonal-build-plate-packing/v1", "build_plate_mm": bed,
            "clearance_mm": clearance, "part_count": placed.len(),
            "part_order_policy": "descending_minimum_footprint",
            "collision_model": "axis_aligned_clearance_boxes", "orientation_candidates": 24,
            "total_downward_face_area_mm2": placed.iter().map(|v| v.pose.area).sum::<f64>(),
            "support_geometry": null, "peel_force_n": null,
        });
        let receipt = self.admit_encoded(
            "json",
            serde_json::to_vec(&metadata).map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record(
            [
                ("placements".into(), Value::List(placements)),
                ("receipt".into(), receipt),
            ]
            .into(),
        ))
    }
}

fn end_xy(xy: [f64; 2], size: [f64; 3]) -> [f64; 2] {
    [xy[0] + size[0], xy[1] + size[1]]
}

fn better(a: &[f64; 7], b: &[f64; 7]) -> bool {
    a.iter()
        .zip(b)
        .map(|(a, b)| a.total_cmp(b))
        .find(|v| *v != Ordering::Equal)
        == Some(Ordering::Less)
}

fn orientations(min: [f64; 3], max: [f64; 3], areas: [[(f64, f64); 2]; 3]) -> Vec<Pose> {
    let original_size = sub(max, min);
    let mut poses = Vec::with_capacity(24);
    for axes in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let inversions = usize::from(axes[0] > axes[1])
            + usize::from(axes[0] > axes[2])
            + usize::from(axes[1] > axes[2]);
        let parity = if inversions % 2 == 0 { 1.0 } else { -1.0 };
        for x in [-1.0, 1.0] {
            for y in [-1.0, 1.0] {
                let signs = [x, y, parity * x * y];
                let (area, protected_area) = areas[axes[2]][usize::from(signs[2] > 0.0)];
                poses.push(Pose {
                    axes,
                    signs,
                    min: std::array::from_fn(|axis| {
                        if signs[axis] > 0.0 {
                            min[axes[axis]]
                        } else {
                            -max[axes[axis]]
                        }
                    }),
                    size: axes.map(|axis| original_size[axis]),
                    area,
                    protected_area,
                });
            }
        }
    }
    poses
}
