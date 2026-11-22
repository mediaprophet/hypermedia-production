//! Two-phase visualization on an authored synthetic membrane.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{input_plane, int_arg, number_arg, output_plane, seal_diagnostics};
use crate::{fields, integer, list, required, text, vector, Args, AssetHost, Value};
use qualia_core_db::specialized_libs::computational_geometry::{
    ddg_operators::{cotangent_laplacian, heat_step, SparseEntry},
    Point3,
};
use std::collections::{BTreeMap, BTreeSet};

struct Membrane {
    vertices: Vec<Point3>,
    triangles: Vec<[u32; 3]>,
}

impl AssetHost<'_> {
    pub(crate) fn simulate_pathology_regime(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "solver",
                "vertices",
                "triangles",
                "initial_phase",
                "steps",
                "step_size",
                "diffusivity",
            ],
        )?;
        if let Some(solver) = args.get("solver") {
            if text(solver)? != "synthetic_membrane_bistable" {
                return Err("Supported solver is synthetic_membrane_bistable".into());
            }
        }
        let membrane = Membrane::read(args)?;
        let n = membrane.vertices.len();
        let steps = int_arg(args, "steps", None, 1..=512)?;
        let step = number_arg(args, "step_size", Some(0.025), 1e-5..=0.25)?;
        let diffusivity = number_arg(args, "diffusivity", Some(0.02), 1e-6..=1.0)?;
        let shape = [n as u32];
        let mut phase = input_plane(
            self,
            required(args, "initial_phase")?,
            &shape,
            0.0..=1.0,
            "initial_phase",
        )?;
        let digest = membrane.digest(&phase, steps, step, diffusivity);

        let capacity = 9 * membrane.triangles.len();
        let scratch = 8 * n * n + 64 * capacity + 64 * n;
        self.charge(10 * n * n + 150 * capacity, scratch)?;
        let mut stiffness = vec![
            SparseEntry {
                row: 0,
                col: 0,
                value: 0.0
            };
            capacity
        ];
        let mut mass = vec![0.0; n];
        let entries = cotangent_laplacian(
            &membrane.vertices,
            &membrane.triangles,
            &mut stiffness,
            &mut mass,
        )
        .map_err(|error| format!("Synthetic membrane Laplacian: {error:?}"))?;
        stiffness.truncate(entries);
        let diffusion_step = diffusivity * step;
        if mass.iter().any(|area| !area.is_finite() || *area <= 0.0)
            || stiffness.iter().any(|entry| !entry.value.is_finite())
        {
            return Err("Synthetic membrane must have finite positive mass at every vertex".into());
        }
        let mut max_diffusion_fraction = 0.0_f64;
        for entry in &stiffness {
            if entry.row == entry.col {
                max_diffusion_fraction = max_diffusion_fraction
                    .max(diffusion_step * entry.value / mass[entry.row as usize]);
            } else if entry.value > 0.0 {
                return Err(
                    "Synthetic membrane diffusion requires nonnegative cotangent edge weights"
                        .into(),
                );
            }
        }
        if max_diffusion_fraction > 1.0 {
            return Err(format!("Synthetic membrane step violates the diffusion maximum principle: {max_diffusion_fraction:.3e} > 1"));
        }
        let area: f64 = mass.iter().sum();
        let initial = weighted_phase(&phase, &mass) / area;
        let reaction_half_step = (-0.5 * step).exp();
        let mut max_diffusion_drift = 0.0_f64;
        for index in 0..steps {
            const STEPS_PER_CHARGE: usize = 16;
            if index % STEPS_PER_CHARGE == 0 {
                self.charge(
                    (4 * n * entries + 120 * n) * (steps - index).min(STEPS_PER_CHARGE),
                    scratch,
                )?;
            }
            advance_phase(&mut phase, reaction_half_step);
            let before_heat = weighted_phase(&phase, &mass);
            heat_step(n, &stiffness, &mass, diffusion_step, &mut phase)
                .map_err(|error| format!("Synthetic membrane heat step: {error:?}"))?;
            let drift = (weighted_phase(&phase, &mass) - before_heat).abs() / area;
            if !drift.is_finite() || drift > 2e-11 {
                return Err(format!(
                    "Synthetic membrane diffusion lost phase-weight conservation at step {}",
                    index + 1
                ));
            }
            max_diffusion_drift = max_diffusion_drift.max(drift);
            for value in &mut phase {
                if !(-1e-12..=1.0 + 1e-12).contains(value) {
                    return Err(format!(
                        "Synthetic membrane phase left [0,1] at step {}",
                        index + 1
                    ));
                }
                *value = value.clamp(0.0, 1.0);
            }
            advance_phase(&mut phase, reaction_half_step);
        }
        let mean = weighted_phase(&phase, &mass) / area;
        let high_area = mass
            .iter()
            .zip(&phase)
            .filter(|(_, value)| **value > 0.5)
            .map(|(area, _)| area)
            .sum::<f64>()
            / area;
        let receipt = seal_diagnostics(
            self,
            "synthetic-membrane-bistable",
            &digest,
            &[
                ("initial_mean_phase", initial),
                ("mean_phase", mean),
                ("high_phase_area_fraction", high_area),
                ("max_diffusion_fraction", max_diffusion_fraction),
                ("max_diffusion_only_mean_drift", max_diffusion_drift),
                ("generated_mesh_count", 0.0),
            ],
        )?;
        let field = output_plane(self, shape.to_vec(), &[&phase])?;
        Ok(Value::Record(Args::from([
            ("field".into(), field),
            ("mean_phase".into(), Value::F64(mean)),
            ("high_phase_area_fraction".into(), Value::F64(high_area)),
            ("receipt".into(), receipt),
        ])))
    }
}

impl Membrane {
    fn read(args: &Args) -> Result<Self, String> {
        let authored = list(required(args, "vertices")?)?;
        if !(3..=256).contains(&authored.len()) {
            return Err("Provide 3..=256 authored membrane vertices".into());
        }
        let vertices = authored
            .iter()
            .map(|value| {
                let [x, y, z] = vector::<3>(value)?;
                if [x, y, z]
                    .iter()
                    .any(|value| !(-1_000.0..=1_000.0).contains(value))
                {
                    return Err(
                        "Synthetic membrane coordinates must be dimensionless and in [-1000,1000]"
                            .into(),
                    );
                }
                Ok(Point3::new(x, y, z))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let authored = list(required(args, "triangles")?)?;
        if !(1..=512).contains(&authored.len()) {
            return Err("Provide 1..=512 authored membrane triangles".into());
        }
        let mut triangles = Vec::with_capacity(authored.len());
        let mut unique_triangles = BTreeSet::new();
        let mut links = vec![BTreeMap::<u32, Vec<u32>>::new(); vertices.len()];
        for face in authored {
            let [a, b, c] = list(face)? else {
                return Err("Each membrane triangle requires three indices".into());
            };
            let triangle = [
                integer(a, "triangle index")?,
                integer(b, "triangle index")?,
                integer(c, "triangle index")?,
            ];
            let mut sorted = triangle;
            sorted.sort_unstable();
            if sorted[0] == sorted[1] || sorted[1] == sorted[2] || sorted[2] >= vertices.len() {
                return Err("Membrane triangles require distinct existing vertex indices".into());
            }
            if !unique_triangles.insert(sorted) {
                return Err("Synthetic membrane contains a duplicate triangle".into());
            }
            let [a, b, c] = triangle.map(|i| i as u32);
            for (center, left, right) in [(a, b, c), (b, c, a), (c, a, b)] {
                let link = &mut links[center as usize];
                link.entry(left).or_default().push(right);
                link.entry(right).or_default().push(left);
            }
            triangles.push([a, b, c]);
        }
        for (i, link) in links.iter().enumerate() {
            if !is_single_fan(link) {
                return Err(format!(
                    "Synthetic membrane vertex {i} lacks a single manifold fan"
                ));
            }
        }
        Ok(Self {
            vertices,
            triangles,
        })
    }

    fn digest(&self, initial: &[f64], steps: usize, dt: f64, diffusivity: f64) -> String {
        let mut digest = blake3::Hasher::new_derive_key("qualia synthetic membrane bistable v1");
        for count in [self.vertices.len(), self.triangles.len(), steps] {
            digest.update(&(count as u64).to_le_bytes());
        }
        for value in [dt, diffusivity]
            .into_iter()
            .chain(self.vertices.iter().flat_map(|p| [p.x, p.y, p.z]))
            .chain(initial.iter().copied())
        {
            digest.update(&value.to_bits().to_le_bytes());
        }
        for index in self.triangles.iter().flatten() {
            digest.update(&index.to_le_bytes());
        }
        digest.finalize().to_hex().to_string()
    }
}

fn is_single_fan(link: &BTreeMap<u32, Vec<u32>>) -> bool {
    let Some((&first, _)) = link.first_key_value() else {
        return false;
    };
    let ends = link.values().filter(|row| row.len() == 1).count();
    if (ends != 0 && ends != 2) || link.values().any(|row| !(1..=2).contains(&row.len())) {
        return false;
    }
    let mut visited = BTreeSet::new();
    let mut remaining = vec![first];
    while let Some(vertex) = remaining.pop() {
        if visited.insert(vertex) {
            remaining.extend(&link[&vertex]);
        }
    }
    visited.len() == link.len()
}

fn weighted_phase(phase: &[f64], mass: &[f64]) -> f64 {
    phase
        .iter()
        .zip(mass)
        .map(|(value, weight)| value * weight)
        .sum()
}

fn advance_phase(phase: &mut [f64], decay: f64) {
    // Exact flow of u' = u(1-u)(2u-1), written for the signed value 2u-1.
    for value in phase {
        let signed = 2.0 * *value - 1.0;
        let square = signed * signed;
        *value = 0.5 * (1.0 + signed / (square + (1.0 - square) * decay).sqrt());
    }
}
