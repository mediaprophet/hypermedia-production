//! Authored rod data advanced through native Etalon Cosserat dynamics.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use crate::artifact::{CurveGroomBundle, CurveStrand};
use crate::*;
use qualia_core_db::etalon_render::skeletal::{
    CosseratElement, CosseratEnvironment, CosseratNode, MAX_COSSERAT_NODES, compute_darboux_vector,
    step_cosserat_rod_zero_heap,
};
use serde_json::json;

impl AssetHost<'_> {
    pub(crate) fn solve_cosserat_strand(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["strands", "dt_s", "steps", "environment"])?;
        let strands = list(required(args, "strands")?)?;
        let dt = bounded(args, "dt_s", 0.0, 1e-7, 0.02)?;
        let steps = count(args, "steps", 1, 1024)?;
        if !(1..=32).contains(&strands.len()) {
            return Err("Cosserat evaluation requires 1 to 32 strands".into());
        }
        let env = environment(record(required(args, "environment")?)?)?;
        let mut out = Vec::with_capacity(strands.len());
        let mut result = Vec::with_capacity(strands.len());
        for (slot, value) in strands.iter().enumerate() {
            let row = record(value)?;
            fields(row, &["nodes", "segments"])?;
            let nodes = list(required(row, "nodes")?)?;
            let segments = list(required(row, "segments")?)?;
            if !(2..=MAX_COSSERAT_NODES).contains(&nodes.len()) || segments.len() + 1 != nodes.len()
            {
                return Err(
                    "Cosserat strand requires one segment between each pair of authored nodes"
                        .into(),
                );
            }
            self.charge(nodes.len() * steps * 32 * 380, nodes.len() * 1600)?;
            let mut nodes = nodes.iter().map(node).collect::<Result<Vec<_>, _>>()?;
            let mut segments = segments
                .iter()
                .map(segment)
                .collect::<Result<Vec<_>, _>>()?;
            let min_mass = nodes
                .iter()
                .map(|node| node.mass)
                .fold(f64::INFINITY, f64::min);
            for segment in &segments {
                let shear = segment.k_shear_stretch.into_iter().fold(0.0_f64, f64::max);
                let bend = segment.k_bend_twist.into_iter().fold(0.0_f64, f64::max);
                let linear_frequency = (2.0 * shear / (min_mass * segment.rest_len)).sqrt();
                let angular_frequency = (2.0 * bend / (1e-4 * segment.rest_len)).sqrt();
                if dt / 32.0 * linear_frequency.max(angular_frequency) > 0.5 {
                    return Err(format!(
                        "Cosserat strand {slot} stiffness requires a shorter timestep"
                    ));
                }
            }
            for _ in 0..steps {
                self.charge(0, 0)?;
                step_cosserat_rod_zero_heap(&mut nodes, &mut segments, env, dt)
                    .map_err(|err| format!("Native Cosserat update: {err:?}"))?;
            }
            if nodes
                .iter()
                .flat_map(|node| node.pos.into_iter().chain(node.vel))
                .chain(
                    segments
                        .iter()
                        .flat_map(|segment| segment.q.into_iter().chain(segment.omega)),
                )
                .any(|v| !v.is_finite() || v.abs() > 1e8)
            {
                return Err(
                    "Cosserat integration diverged outside the admitted state range".into(),
                );
            }
            let frames = segments
                .iter()
                .enumerate()
                .map(|(index, element)| {
                    let darboux = segments.get(index + 1).map(|next| {
                        compute_darboux_vector(
                            element.q,
                            next.q,
                            (element.rest_len + next.rest_len) * 0.5,
                        )
                    });
                    json!({
                        "segment": index,
                        "rotor_wxyz": element.q,
                        "angular_velocity_rad_s": element.omega,
                        "darboux_per_m": darboux,
                    })
                })
                .collect::<Vec<_>>();
            let positions = nodes
                .iter()
                .map(|node| node.pos.map(|v| v as f32))
                .collect();
            out.push(CurveStrand {
                root_uv: None,
                positions,
            });
            result.push(json!({ "strand": slot, "frames": frames }));
        }
        let tracks = self.admit_artifact(TypedArtifact::CurveGroomBundle(Arc::new(
            CurveGroomBundle { strands: out },
        )))?;
        let stats = serde_json::to_vec(&json!({
            "schema": "qualia-cosserat-strands/1", "final_time_s": dt * steps as f64, "strands": result
        })).map_err(|error| error.to_string())?;
        let frames = self.encode("json", stats)?;
        Ok(Value::Record(Args::from([
            ("strands".into(), tracks),
            ("frames".into(), frames),
        ])))
    }
}

fn node(value: &Value) -> Result<CosseratNode, String> {
    let row = record(value)?;
    fields(
        row,
        &[
            "position_m",
            "velocity_m_s",
            "mass_kg",
            "radius_m",
            "pinned",
        ],
    )?;
    Ok(CosseratNode {
        pos: array(row, "position_m", None, -1e6, 1e6)?,
        vel: array(row, "velocity_m_s", Some([0.0; 3]), -1000.0, 1000.0)?,
        mass: bounded(row, "mass_kg", 0.0, 1e-7, 1e7)?,
        radius_m: bounded(row, "radius_m", 0.0, 1e-6, 1e3)?,
        pinned: boolean(row, "pinned", false)?,
    })
}

fn segment(value: &Value) -> Result<CosseratElement, String> {
    let row = record(value)?;
    fields(
        row,
        &[
            "rotor_wxyz",
            "rest_length_m",
            "rest_darboux_per_m",
            "bend_twist_nm2",
            "shear_stretch_n",
            "cavity_pressure_pa",
            "drag_normal",
            "drag_tangent",
        ],
    )?;
    let rotor: [f64; 4] = array(row, "rotor_wxyz", None, -1.0, 1.0)?;
    if (rotor.into_iter().map(|v| v * v).sum::<f64>() - 1.0).abs() > 1e-8 {
        return Err("Cosserat segment requires a unit material-frame rotor".into());
    }
    Ok(CosseratElement {
        q: rotor,
        rest_len: bounded(row, "rest_length_m", 0.0, 1e-5, 1e4)?,
        rest_darboux: array(row, "rest_darboux_per_m", Some([0.0; 3]), -1e5, 1e5)?,
        k_bend_twist: array(row, "bend_twist_nm2", None, 0.0, 1e8)?,
        k_shear_stretch: array(row, "shear_stretch_n", None, 0.0, 1e8)?,
        cavity_pressure_pa: bounded(row, "cavity_pressure_pa", 0.0, 0.0, 2e7)?,
        drag_normal: bounded(row, "drag_normal", 1.2, 0.0, 10.0)?,
        drag_tangent: bounded(row, "drag_tangent", 0.15, 0.0, 10.0)?,
        ..Default::default()
    })
}

fn environment(row: &Args) -> Result<CosseratEnvironment, String> {
    fields(
        row,
        &[
            "gravity_m_s2",
            "fluid_velocity_m_s",
            "fluid_density_kg_m3",
            "linear_damping",
            "angular_damping",
        ],
    )?;
    Ok(CosseratEnvironment {
        gravity: array(
            row,
            "gravity_m_s2",
            Some([0.0, -9.80665, 0.0]),
            -1000.0,
            1000.0,
        )?,
        fluid_velocity: array(row, "fluid_velocity_m_s", Some([0.0; 3]), -1000.0, 1000.0)?,
        fluid_density_kg_m3: bounded(row, "fluid_density_kg_m3", 1.225, 0.0, 1e4)?,
        linear_damping: bounded(row, "linear_damping", 0.992, 0.0, 1.0)?,
        angular_damping: bounded(row, "angular_damping", 0.985, 0.0, 1.0)?,
        ..Default::default()
    })
}

fn bounded(args: &Args, name: &str, default: f64, low: f64, high: f64) -> Result<f64, String> {
    let value = number(args, name, default)?;
    (low..=high)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{name} exceeds its admitted range"))
}

fn array<const N: usize>(
    args: &Args,
    name: &str,
    default: Option<[f64; N]>,
    low: f64,
    high: f64,
) -> Result<[f64; N], String> {
    let value = args
        .get(name)
        .map(vector::<N>)
        .transpose()?
        .or(default)
        .ok_or_else(|| format!("Missing {name}"))?;
    value
        .iter()
        .all(|v| (low..=high).contains(v))
        .then_some(value)
        .ok_or_else(|| format!("{name} exceeds its admitted range"))
}
