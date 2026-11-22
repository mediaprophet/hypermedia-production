//! Single-crease origami with a compliant, pressure-driven hinge model.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{int_arg, mesh_artifact, number_arg, seal_diagnostics};
use crate::{Args, AssetHost, Value, fields, integer, list, required, text, vector};
use std::collections::BTreeMap;

const TRAVEL: f64 = std::f64::consts::PI - 0.001;

struct Pattern {
    positions: Vec<[f64; 2]>,
    triangles: Vec<[u32; 3]>,
    pivot: [f64; 2],
    axis: [f64; 2],
    normal_distances: Vec<f64>,
    area_m2: f64,
    area_moment_m4: f64,
}

impl AssetHost<'_> {
    pub(crate) fn solve_compliant_soft_robotics_origami(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "solver",
                "vertices_m",
                "triangles",
                "hinge_edge",
                "areal_density_kg_m2",
                "hinge_stiffness_nm_rad",
                "hinge_damping_nms_rad",
                "crease_angle_rad",
                "rest_angle_rad",
                "volume_gradient_m3_rad",
                "chamber_pressure_pa",
                "dt_s",
                "steps",
            ],
        )?;
        if let Some(solver) = args.get("solver") {
            if text(solver)? != "rigid_single_crease_pressure_hinge" {
                return Err("Supported solver is rigid_single_crease_pressure_hinge".into());
            }
        }
        let pattern = Pattern::read(args)?;
        let density = number_arg(args, "areal_density_kg_m2", None, 0.001..=1e5)?;
        let stiffness = number_arg(args, "hinge_stiffness_nm_rad", None, 1e-12..=1e9)?;
        let damping = number_arg(args, "hinge_damping_nms_rad", Some(0.0), 0.0..=1e9)?;
        let mut angle = number_arg(args, "crease_angle_rad", Some(0.0), -TRAVEL..=TRAVEL)?;
        let rest = number_arg(args, "rest_angle_rad", Some(0.0), -TRAVEL..=TRAVEL)?;
        let pressure_area = number_arg(args, "volume_gradient_m3_rad", None, 0.0..=1.0)?;
        let pressure = number_arg(args, "chamber_pressure_pa", None, -1e5..=2e6)?;
        let torque = pressure * pressure_area;
        let dt = number_arg(args, "dt_s", None, 1e-7..=10.0)?;
        let steps = int_arg(args, "steps", None, 1..=65_536)?;
        self.charge(
            pattern.positions.len() * 48 + pattern.triangles.len() * 250 + steps * 100,
            pattern.positions.len() * 96 + pattern.triangles.len() * 128,
        )?;
        let inertia = density * pattern.area_moment_m4;
        if !inertia.is_finite() || inertia <= 1e-24 {
            return Err("Compliant crease requires nonzero finite hinge inertia".into());
        }
        let frequency_step = dt * (stiffness / inertia).sqrt();
        if frequency_step > 0.5 {
            return Err(format!(
                "Crease step resolves the natural mode poorly: dt*omega={frequency_step:.12e} exceeds 0.5"
            ));
        }
        let initial_energy = 0.5 * stiffness * (angle - rest).powi(2);
        let initial_angle = angle;
        let mut speed = 0.0;
        let mut work = 0.0;
        let mut dissipated = 0.0;
        let correction = 0.5 * dt * damping + 0.25 * dt * dt * stiffness;
        for step in 0..steps {
            let next_speed = ((inertia - correction) * speed
                + dt * (torque - stiffness * (angle - rest)))
                / (inertia + correction);
            let mean_speed = 0.5 * (speed + next_speed);
            let delta = dt * mean_speed;
            angle += delta;
            if !(-TRAVEL..=TRAVEL).contains(&angle) {
                return Err(format!(
                    "Crease step {} exceeds contact-free travel: angle {angle:.12e} rad",
                    step + 1,
                ));
            }
            work += torque * delta;
            dissipated += dt * damping * mean_speed.powi(2);
            speed = next_speed;
        }
        let energy = 0.5 * inertia * speed.powi(2) + 0.5 * stiffness * (angle - rest).powi(2);
        let residual = energy - initial_energy - work + dissipated;
        let energy_scale = initial_energy.abs() + work.abs() + dissipated.abs() + energy.abs();
        let tolerance = 1e-8 * energy_scale.max(1e-10);
        if !residual.is_finite() || residual.abs() > tolerance {
            return Err(format!(
                "Crease energy-balance residual {residual:.12e} J exceeds {tolerance:.12e}"
            ));
        }
        let positions = pattern.posed(angle);
        let mut hasher =
            blake3::Hasher::new_derive_key("qualia continuum single crease pressure hinge v1");
        for count in [pattern.positions.len(), pattern.triangles.len()] {
            hasher.update(&(count as u64).to_le_bytes());
        }
        for coordinate in pattern.positions.iter().flatten() {
            hasher.update(&coordinate.to_bits().to_le_bytes());
        }
        for index in pattern.triangles.iter().flatten() {
            hasher.update(&index.to_le_bytes());
        }
        for value in pattern.pivot.into_iter().chain(pattern.axis).chain([
            initial_angle,
            rest,
            stiffness,
            density,
            damping,
            pressure_area,
            pressure,
            dt,
        ]) {
            hasher.update(&value.to_bits().to_le_bytes());
        }
        hasher.update(&(steps as u64).to_le_bytes());
        let mesh = mesh_artifact(self, positions, pattern.triangles)?;
        let metrics = [
            ("inertia_kg_m2", inertia),
            ("actuated_area_m2", pattern.area_m2),
            ("crease_angle_rad", angle),
            ("hinge_angular_speed_rad_s", speed),
            ("mechanical_energy_j", energy),
            ("pressure_work_j", work),
            ("dissipated_energy_j", dissipated),
            ("energy_balance_residual_j", residual),
        ];
        let receipt = seal_diagnostics(
            self,
            "rigid-single-crease-pressure-hinge",
            &hasher.finalize().to_hex(),
            &metrics,
        )?;
        Ok(Value::Record(Args::from([
            ("mesh".into(), mesh),
            ("receipt".into(), receipt),
            ("crease_angle_rad".into(), Value::F64(angle)),
            ("hinge_angular_speed_rad_s".into(), Value::F64(speed)),
            ("energy_balance_residual_j".into(), Value::F64(residual)),
        ])))
    }
}

impl Pattern {
    fn read(args: &Args) -> Result<Self, String> {
        let input = list(required(args, "vertices_m")?)?;
        if !(4..=4096).contains(&input.len()) {
            return Err("Crease pattern requires 4..=4096 planar vertices".into());
        }
        let positions: Vec<[f64; 2]> = input.iter().map(vector::<2>).collect::<Result<_, _>>()?;
        if positions.iter().flatten().any(|x| x.abs() > 1000.0) {
            return Err("Crease positions exceed the admitted coordinate range".into());
        }
        let hinge = list(required(args, "hinge_edge")?)?;
        let [a, b] = hinge else {
            return Err("hinge_edge requires two vertex indices".into());
        };
        let a = integer(a, "hinge index")?;
        let b = integer(b, "hinge index")?;
        let pivot = *positions.get(a).ok_or("Invalid hinge pivot")?;
        let end = *positions.get(b).ok_or("Invalid hinge endpoint")?;
        let delta = [end[0] - pivot[0], end[1] - pivot[1]];
        let length = delta[0].hypot(delta[1]);
        if length < 1e-6 {
            return Err("Hinge edge must have length of at least 1 micrometre".into());
        }
        let axis = delta.map(|x| x / length);
        let mut distance: Vec<f64> = positions
            .iter()
            .map(|p| axis[0] * (p[1] - pivot[1]) - axis[1] * (p[0] - pivot[0]))
            .collect();
        distance[a] = 0.0;
        distance[b] = 0.0;
        if distance
            .iter()
            .enumerate()
            .any(|(index, d)| index != a && index != b && d.abs() <= 1e-10)
        {
            return Err("Hinge must be a single unsplit crease edge in the planar mesh".into());
        }
        let input = list(required(args, "triangles")?)?;
        if !(2..=8192).contains(&input.len()) {
            return Err("Crease pattern requires 2..=8192 triangles".into());
        }
        let mut triangles = Vec::with_capacity(input.len());
        // Track incident triangles and topological edge orientation.
        let mut edges: BTreeMap<[usize; 2], (usize, bool, bool)> = BTreeMap::new();
        let mut adjacency = vec![Vec::new(); input.len()];
        let mut used = vec![false; positions.len()];
        let mut area_m2 = 0.0;
        let mut area_moment_m4 = 0.0;
        let mut hinge_pair: Option<[usize; 2]> = None;
        let hinge = [a.min(b), a.max(b)];
        for (face, value) in input.iter().enumerate() {
            let row = list(value)?;
            let [a, b, c] = row else {
                return Err("Each crease triangle requires three vertex indices".into());
            };
            let triangle = [
                integer(a, "triangle")?,
                integer(b, "triangle")?,
                integer(c, "triangle")?,
            ];
            if triangle.iter().any(|&v| v >= positions.len()) {
                return Err("Crease triangle has a vertex outside its input pattern".into());
            }
            let [a, b, c] = triangle.map(|v| positions[v]);
            let area = 0.5 * ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]));
            if area <= 1e-18 {
                return Err("Crease triangles must be counterclockwise with positive area".into());
            }
            let distances = triangle.map(|v| distance[v]);
            let flap = distances.iter().any(|&d| d > 0.0);
            if flap && distances.iter().any(|&d| d < 0.0) {
                return Err(
                    "A crease triangle crosses the hinge instead of sharing its edge".into(),
                );
            }
            if flap {
                let sum = distances.iter().sum::<f64>();
                let sum_squared = distances.iter().map(|d| d * d).sum::<f64>();
                area_m2 += area;
                area_moment_m4 += area * (sum.powi(2) + sum_squared) / 12.0;
            }
            let [a, b, c] = triangle;
            for v in triangle {
                used[v] = true;
            }
            for [p, q] in [[a, b], [b, c], [c, a]] {
                let key = [p.min(q), p.max(q)];
                let positive = p < q;
                if let Some((other, other_positive, other_flap)) = edges.get_mut(&key) {
                    let adjacent = *other;
                    if adjacent == usize::MAX || positive == *other_positive {
                        return Err(
                            "Inconsistent orientation or non-manifold edge in crease pattern"
                                .into(),
                        );
                    }
                    if key == hinge {
                        if flap == *other_flap {
                            return Err("Crease needs one panel on each side of its hinge".into());
                        }
                        hinge_pair = Some([face, adjacent]);
                    } else if flap != *other_flap {
                        return Err("Distinct panels must meet only at the authored hinge".into());
                    }
                    *other = usize::MAX;
                    adjacency[face].push(adjacent);
                    adjacency[adjacent].push(face);
                } else {
                    edges.insert(key, (face, positive, flap));
                }
            }
            triangles.push(triangle.map(|k| k as u32));
        }
        if hinge_pair.is_none() || used.contains(&false) {
            return Err("Crease pattern must share the hinge and use all vertices".into());
        }
        let mut seen = vec![false; input.len()];
        let mut stack = vec![0];
        while let Some(face) = stack.pop() {
            if !seen[face] {
                seen[face] = true;
                stack.extend(&adjacency[face]);
            }
        }
        if seen.contains(&false) {
            return Err("Crease triangles must form one edge-connected pattern".into());
        }
        Ok(Self {
            positions,
            triangles,
            pivot,
            axis,
            normal_distances: distance,
            area_m2,
            area_moment_m4,
        })
    }

    fn posed(&self, angle: f64) -> Vec<[f32; 3]> {
        let (sine, cosine) = angle.sin_cos();
        self.positions
            .iter()
            .zip(&self.normal_distances)
            .map(|(p, &offset)| {
                if offset <= 0.0 {
                    return [p[0] as f32, p[1] as f32, 0.0];
                }
                let along =
                    (p[0] - self.pivot[0]) * self.axis[0] + (p[1] - self.pivot[1]) * self.axis[1];
                [
                    (self.pivot[0] + along * self.axis[0] - offset * cosine * self.axis[1]) as f32,
                    (self.pivot[1] + along * self.axis[1] + offset * cosine * self.axis[0]) as f32,
                    (offset * sine) as f32,
                ]
            })
            .collect()
    }
}
