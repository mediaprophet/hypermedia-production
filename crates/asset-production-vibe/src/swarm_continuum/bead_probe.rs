//! Exact probe-shell area for a non-self-contacting chain of equal inert beads.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{number_arg, output_plane, seal_diagnostics};
use crate::{fields, list, required, text, vector, Args, AssetHost, Value};
use qualia_core_db::solvers::calculus::analysis::Vector;
use std::f64::consts::TAU;

impl AssetHost<'_> {
    pub(crate) fn fold_macromolecule_and_cell_sheet(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["solver", "centers", "bead_radius", "probe_radius"])?;
        if text(required(args, "solver")?)? != "inert_uniform_bead_chain_probe_shell" {
            return Err("Supported solver is inert_uniform_bead_chain_probe_shell".into());
        }
        let radius = number_arg(args, "bead_radius", None, 0.01..=50.0)?;
        let probe = number_arg(args, "probe_radius", Some(radius / 4.0), 0.0..=radius / 2.0)?;
        let centers = list(required(args, "centers")?)?;
        if !(2..=32).contains(&centers.len()) {
            return Err("An inert bead chain must have 2..=32 bead centers".into());
        }
        let centers = centers
            .iter()
            .map(|value| {
                let point = vector::<3>(value)?;
                if point
                    .iter()
                    .any(|coordinate| !(-1_000.0..=1_000.0).contains(coordinate))
                {
                    return Err(
                        "Dimensionless inert bead center coordinates must be in [-1000,1000]"
                            .into(),
                    );
                }
                Ok(Vector::new(point))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let count = centers.len();
        self.charge(32 * count * count, 80 * count)?;

        let shell_radius = radius + probe;
        let full_area = 2.0 * TAU * shell_radius.powi(2);
        let mut buried = vec![0.0; count];
        for (i, &a) in centers.iter().enumerate() {
            for (j, &b) in centers.iter().enumerate().skip(i + 1) {
                let distance = a
                    .distance(b)
                    .map_err(|error| format!("Inert bead distance: {error:?}"))?;
                if j == i + 1 {
                    if distance == 0.0 || distance > 2.0 * radius {
                        return Err(format!("Consecutive inert beads {i} and {j} must have distinct centers and touching or overlapping cores"));
                    }
                    // Equal spheres intersect at the midpoint. The spherical
                    // cap area is 2πR times the cap height R - distance/2.
                    let cap = TAU * shell_radius * (shell_radius - 0.5 * distance);
                    buried[i] += cap;
                    buried[j] += cap;
                } else if distance <= 2.0 * shell_radius {
                    return Err(format!("Nonconsecutive probe shells {i} and {j} touch or overlap; this solver requires a chain without self-contact"));
                }
            }
        }

        // The chain constraint rules out covered-cap overlaps on any bead
        // and enclosed voids, so subtracting caps gives the exposed area.
        let exposed = buried
            .iter()
            .map(|area| full_area - area)
            .collect::<Vec<_>>();
        let fractions = exposed
            .iter()
            .map(|area| area / full_area)
            .collect::<Vec<_>>();
        let total = exposed.iter().sum::<f64>();
        let mut digest =
            blake3::Hasher::new_derive_key("qualia exact inert bead chain probe shell v1");
        digest.update(&(count as u64).to_le_bytes());
        for value in [radius, probe]
            .into_iter()
            .chain(centers.iter().flat_map(|point| point.data))
        {
            digest.update(&value.to_bits().to_le_bytes());
        }
        let receipt = seal_diagnostics(
            self,
            "inert-uniform-bead-chain-probe-shell",
            &digest.finalize().to_hex().to_string(),
            &[
                ("bead_count", count as f64),
                ("bead_radius", radius),
                ("probe_radius", probe),
                ("probe_shell_radius", shell_radius),
                ("total_accessible_area", total),
                ("isolated_bead_area", full_area),
            ],
        )?;
        let bead_accessibility = output_plane(self, vec![count as u32], &[&exposed, &fractions])?;
        Ok(Value::Record(Args::from([
            ("bead_accessibility".into(), bead_accessibility),
            ("total_accessible_area".into(), Value::F64(total)),
            ("probe_shell_radius".into(), Value::F64(shell_radius)),
            ("receipt".into(), receipt),
        ])))
    }
}
