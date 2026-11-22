//! Isoviscous line-contact Reynolds pressure with elastic compliance and wear.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{input_plane, int_arg, number_arg, output_plane, seal_diagnostics};
use crate::{Args, AssetHost, Value, fields, required, text};

#[derive(Clone, Copy)]
struct Contact {
    spacing: f64,
    viscosity: f64,
    speed: f64,
    compliance: f64,
    relaxation: f64,
}

const MAX_PRESSURE_PA: f64 = 1e10;
const MAX_GAP_M: f64 = 1e-2;

impl AssetHost<'_> {
    pub(crate) fn solve_tribology_ehl_and_wear(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "solver",
                "cell_count",
                "cell_size_m",
                "film_thickness_m",
                "dynamic_viscosity_pa_s",
                "surface_speed_m_s",
                "compliance_m_pa",
                "wear_coefficient",
                "surface_hardness_pa",
                "wear_dt_s",
                "wear_steps",
                "pressure_sweeps",
                "relative_pressure_tolerance",
                "pressure_relaxation",
            ],
        )?;
        if let Some(solver) = args.get("solver") {
            if text(solver)? != "isoviscous_winkler_archard" {
                return Err("Supported solver is isoviscous_winkler_archard".into());
            }
        }
        let cells = int_arg(args, "cell_count", None, 3..=256)?;
        let steps = int_arg(args, "wear_steps", Some(1), 0..=128)?;
        let sweeps = int_arg(args, "pressure_sweeps", Some(20_000), 1..=80_000)?;
        let tolerance = number_arg(
            args,
            "relative_pressure_tolerance",
            Some(1e-6),
            1e-10..=1e-3,
        )?;
        let dt = number_arg(args, "wear_dt_s", Some(0.01), 1e-8..=1e3)?;
        let coefficient = number_arg(args, "wear_coefficient", Some(0.0), 0.0..=1.0)?;
        let hardness = number_arg(args, "surface_hardness_pa", Some(1e9), 1e3..=1e12)?;
        let contact = Contact {
            spacing: number_arg(args, "cell_size_m", None, 1e-7..=10.0)?,
            viscosity: number_arg(args, "dynamic_viscosity_pa_s", None, 1e-5..=1e4)?,
            speed: number_arg(args, "surface_speed_m_s", None, -1e3..=1e3)?,
            compliance: number_arg(args, "compliance_m_pa", Some(0.0), 0.0..=1e-4)?,
            relaxation: number_arg(args, "pressure_relaxation", Some(0.8), 0.05..=1.0)?,
        };
        let axes = [cells as u32];
        let base = input_plane(
            self,
            required(args, "film_thickness_m")?,
            &axes,
            1e-10..=MAX_GAP_M,
            "film_thickness_m",
        )?;
        let mut pressure = vec![0.0; cells];
        let mut wear = vec![0.0; cells];
        let mut gap = vec![0.0; cells];
        let mut max_residual: f64 = 0.0;
        let mut total_sweeps = 0usize;
        for step in 0..=steps {
            let (used, residual) = contact.equilibrate(
                self,
                &base,
                &wear,
                &mut pressure,
                &mut gap,
                sweeps,
                tolerance,
            )?;
            max_residual = max_residual.max(residual);
            total_sweeps += used;
            if step < steps {
                let scale = coefficient * contact.speed.abs() * dt / hardness;
                for (w, p) in wear.iter_mut().zip(&pressure) {
                    *w += scale * p;
                }
            }
        }
        let field = output_plane(self, axes.to_vec(), &[&pressure, &gap, &wear])?;
        let load = pressure.iter().sum::<f64>() * contact.spacing;
        let max_pressure = pressure.iter().copied().fold(0.0, f64::max);
        let max_wear = wear.iter().copied().fold(0.0, f64::max);
        let min_gap = gap.iter().copied().fold(f64::INFINITY, f64::min);
        let mut hasher = blake3::Hasher::new_derive_key("qualia continuum Reynolds Archard v1");
        for value in [
            contact.spacing,
            contact.speed,
            contact.viscosity,
            contact.compliance,
            contact.relaxation,
            tolerance,
            dt,
            coefficient,
            hardness,
        ]
        .into_iter()
        .chain(base)
        {
            hasher.update(&value.to_bits().to_le_bytes());
        }
        for value in [steps, sweeps, cells] {
            hasher.update(&(value as u64).to_le_bytes());
        }
        let metrics = [
            ("load_n_per_m", load),
            ("max_pressure_pa", max_pressure),
            ("maximum_wear_m", max_wear),
            ("minimum_film_m", min_gap),
            ("max_normalized_complementarity_residual", max_residual),
            ("wear_duration_s", dt * steps as f64),
        ];
        let receipt = seal_diagnostics(
            self,
            "isoviscous-winkler-archard",
            &hasher.finalize().to_hex(),
            &metrics,
        )?;
        Ok(Value::Record(Args::from([
            ("field".into(), field),
            (
                "plane_names".into(),
                Value::List(
                    ["gauge_pressure_pa", "film_thickness_m", "wear_m"]
                        .map(|name| Value::String(name.into()))
                        .into(),
                ),
            ),
            ("receipt".into(), receipt),
            ("pressure_sweeps".into(), Value::U64(total_sweeps as u64)),
            ("line_load_n_per_m".into(), Value::F64(load)),
            (
                "max_normalized_complementarity_residual".into(),
                Value::F64(max_residual),
            ),
        ])))
    }
}

impl Contact {
    fn film(
        &self,
        base: &[f64],
        wear: &[f64],
        pressure: &[f64],
        gap: &mut [f64],
    ) -> Result<(), String> {
        for i in 0..gap.len() {
            gap[i] = base[i] + wear[i] + self.compliance * pressure[i];
            if !(1e-10..=MAX_GAP_M).contains(&gap[i]) {
                return Err(format!(
                    "Cell {i} film {:.12e} m is outside admissible range",
                    gap[i]
                ));
            }
        }
        Ok(())
    }

    fn projected_pressure(&self, i: usize, gap: &[f64], pressure: &[f64]) -> f64 {
        let left_gap = 0.5 * (gap[i - 1] + gap[i]);
        let right_gap = 0.5 * (gap[i] + gap[i + 1]);
        let left_conductance = left_gap.powi(3);
        let right_conductance = right_gap.powi(3);
        let source = -6.0 * self.viscosity * self.speed * self.spacing * (right_gap - left_gap);
        let gauge =
            (left_conductance * pressure[i - 1] + right_conductance * pressure[i + 1] + source)
                / (left_conductance + right_conductance);
        gauge.max(0.0)
    }

    fn equilibrate(
        &self,
        host: &mut AssetHost<'_>,
        base: &[f64],
        wear: &[f64],
        pressure: &mut [f64],
        gap: &mut [f64],
        sweeps: usize,
        tolerance: f64,
    ) -> Result<(usize, f64), String> {
        let interior = 1..pressure.len() - 1;
        let mut residual = f64::INFINITY;
        self.film(base, wear, pressure, gap)?;
        for sweep in 0..sweeps {
            host.charge(pressure.len() * 100, pressure.len() * 96)?;
            for i in interior.clone() {
                let projected = self.projected_pressure(i, gap, pressure);
                pressure[i] += self.relaxation * (projected - pressure[i]);
                if !(0.0..=MAX_PRESSURE_PA).contains(&pressure[i]) {
                    return Err(format!(
                        "Projected pressure {:.12e} Pa at cell {i} exceeds admitted range",
                        pressure[i]
                    ));
                }
            }
            self.film(base, wear, pressure, gap)?;
            let pressure_scale = pressure.iter().copied().fold(1.0, f64::max);
            residual = interior
                .clone()
                .map(|i| (pressure[i] - self.projected_pressure(i, gap, pressure)).abs())
                .fold(0.0, f64::max)
                / pressure_scale;
            if residual <= tolerance {
                return Ok((sweep + 1, residual));
            }
        }
        Err(format!(
            "Reynolds complementarity residual {residual:.12e} exceeds tolerance {tolerance:.12e} after {sweeps} sweeps"
        ))
    }
}
