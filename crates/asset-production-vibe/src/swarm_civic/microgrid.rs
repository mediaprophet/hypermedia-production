//! Independent-panel MPPT and a dispatch and embodied-carbon ledger.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use qualia_core_db::domains::physical::thermodynamics::SolarPanel;

use super::{bounded, numeric_row, table_receipt};
use crate::{Args, AssetHost, Value, fields, list, record, required};

struct Storage {
    capacity_wh: f64,
    state_wh: f64,
    rate_w: f64,
    eta_charge: f64,
    eta_discharge: f64,
}

impl Storage {
    fn read(args: &Args) -> Result<Self, String> {
        fields(
            args,
            &[
                "capacity_wh",
                "initial_soc",
                "max_power_w",
                "charge_efficiency",
                "discharge_efficiency",
            ],
        )?;
        let capacity_wh = bounded(args, "capacity_wh", None, 1.0, 1e12)?;
        Ok(Self {
            capacity_wh,
            state_wh: capacity_wh * bounded(args, "initial_soc", Some(0.5), 0.0, 1.0)?,
            rate_w: bounded(args, "max_power_w", None, 0.0, 1e10)?,
            eta_charge: bounded(args, "charge_efficiency", None, 0.001, 1.0)?,
            eta_discharge: bounded(args, "discharge_efficiency", None, 0.001, 1.0)?,
        })
    }

    fn dispatch(&mut self, surplus_wh: f64, hours: f64) -> (f64, f64) {
        let rate_wh = self.rate_w * hours;
        // Positive signed flow enters storage at the bus; negative flow leaves it.
        let before = self.state_wh;
        let flow = if surplus_wh >= 0.0 {
            surplus_wh
                .min(rate_wh)
                .min((self.capacity_wh - before) / self.eta_charge)
        } else {
            surplus_wh.max(-rate_wh).max(-before * self.eta_discharge)
        };
        let increment = if flow >= 0.0 {
            flow * self.eta_charge
        } else {
            flow / self.eta_discharge
        };
        self.state_wh = (before + increment).clamp(0.0, self.capacity_wh);
        (flow, (flow - increment).max(0.0))
    }
}

impl AssetHost<'_> {
    pub(crate) fn solve_microgrid_and_lca_commons(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "panels",
                "battery",
                "profile",
                "grid_import_limit_w",
                "grid_kg_co2e_per_kwh",
                "materials",
            ],
        )?;
        let panel_spec = list(required(args, "panels")?)?;
        let profile = list(required(args, "profile")?)?;
        let materials = list(required(args, "materials")?)?;
        if !(1..=128).contains(&panel_spec.len())
            || !(1..=2048).contains(&profile.len())
            || materials.len() > 2048
        {
            return Err(
                "Microgrid scenario exceeds the panel, profile, or inventory budget".into(),
            );
        }
        self.charge(
            profile.len() * (128 * panel_spec.len() + 32) + 16 * materials.len(),
            profile.len() * 4096 + materials.len() * 128,
        )?;
        let panels = panel_spec
            .iter()
            .map(|value| {
                let row = record(value)?;
                fields(
                    row,
                    &[
                        "short_circuit_current_a",
                        "open_circuit_voltage_v",
                        "fill_factor",
                    ],
                )?;
                Ok(SolarPanel {
                    short_circuit_current_a: bounded(
                        row,
                        "short_circuit_current_a",
                        None,
                        0.001,
                        1e5,
                    )?,
                    open_circuit_voltage_v: bounded(
                        row,
                        "open_circuit_voltage_v",
                        None,
                        0.001,
                        1e5,
                    )?,
                    fill_factor: bounded(row, "fill_factor", None, 0.05, 0.95)?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut storage = Storage::read(record(required(args, "battery")?)?)?;
        let limit = bounded(args, "grid_import_limit_w", Some(0.0), 0.0, 1e10)?;
        let grid_factor = bounded(args, "grid_kg_co2e_per_kwh", None, 0.0, 100.0)?;
        let mut embodied = 0.0;
        for material in materials {
            let row = record(material)?;
            fields(row, &["mass_kg", "kg_co2e_per_kg"])?;
            embodied += bounded(row, "mass_kg", None, 0.0, 1e10)?
                * bounded(row, "kg_co2e_per_kg", None, 0.0, 1e6)?;
        }
        let mut operating = 0.0;
        let mut max_error = 0.0_f64;
        let mut time = 0.0;
        let mut rows = Vec::with_capacity(profile.len());
        for value in profile {
            let row = record(value)?;
            fields(row, &["duration_s", "irradiance_w_m2", "load_w"])?;
            let duration = bounded(row, "duration_s", None, 1.0, 86_400.0)?;
            let irradiance = bounded(row, "irradiance_w_m2", None, 0.0, 1500.0)?;
            let hours = duration / 3600.0;
            let load = bounded(row, "load_w", None, 0.0, 1e10)? * hours;
            let solar = hours
                * panels
                    .iter()
                    .map(|panel| {
                        SolarPanel {
                            short_circuit_current_a: panel.short_circuit_current_a * irradiance
                                / 1000.0,
                            ..*panel
                        }
                        .max_power_point(128)
                        .2
                    })
                    .sum::<f64>();
            let before = storage.state_wh;
            let (stored_at_bus, losses) = storage.dispatch(solar - load, hours);
            let surplus = solar - load - stored_at_bus;
            let curtailed = surplus.max(0.0);
            let imported = (-surplus).clamp(0.0, limit * hours);
            let unserved = (-surplus - imported).max(0.0);
            // Source energy equals served and curtailed energy, storage change, and loss.
            let error = (solar + imported + unserved
                - load
                - curtailed
                - losses
                - (storage.state_wh - before))
                .abs();
            if error > 1e-6 {
                return Err(format!(
                    "Microgrid interval energy-balance residual {error:.9e} Wh exceeds 1e-6 Wh"
                ));
            }
            max_error = max_error.max(error);
            operating += imported * grid_factor / 1000.0;
            time += duration;
            rows.push(numeric_row([
                time,
                solar,
                load,
                imported,
                curtailed,
                unserved,
                storage.state_wh,
                losses,
                error,
            ]));
        }
        let mut out = table_receipt(
            self,
            "urn:qualia:civic:single-bus-solar-battery",
            &[
                "time_s",
                "solar_wh",
                "load_wh",
                "import_wh",
                "curtailed_wh",
                "unserved_wh",
                "battery_wh",
                "storage_loss_wh",
                "residual_wh",
            ],
            &rows,
        )?;
        out.insert("embodied_kg_co2e".into(), Value::F64(embodied));
        out.insert("operating_kg_co2e".into(), Value::F64(operating));
        out.insert("max_balance_residual_wh".into(), Value::F64(max_error));
        Ok(Value::Record(out))
    }
}
