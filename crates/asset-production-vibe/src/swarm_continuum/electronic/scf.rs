//! Restricted LDA/VWN SCF for a two-electron soft-Coulomb teaching trap.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{dot_slice, eigen_residual, hash_floats, matvec, RealEigensystem};
use crate::swarm_continuum::{int_arg, number_arg, output_plane, seal_diagnostics};
use crate::{fields, text, Args, AssetHost, Value};
use qualia_core_db::specialized_libs::chemistry_modeling::dft::{
    lda_correlation_vwn, lda_exchange,
};

impl AssetHost<'_> {
    pub(crate) fn solve_dft_scf(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "solver",
                "grid_sites",
                "half_width_bohr",
                "trap_frequency_ha",
                "softening_bohr",
                "transverse_area_bohr2",
                "external_field_ha_per_bohr",
                "density_mixing",
                "max_iterations",
                "energy_tolerance_ha",
                "density_tolerance_e",
            ],
        )?;
        if let Some(solver) = args.get("solver") {
            if text(solver)? != "restricted_lda_vwn_soft_coulomb_trap" {
                return Err("Supported solver is restricted_lda_vwn_soft_coulomb_trap".into());
            }
        }
        let sites = int_arg(args, "grid_sites", Some(19), 9..=33)?;
        let max_iterations = int_arg(args, "max_iterations", Some(80), 2..=160)?;
        let width = number_arg(args, "half_width_bohr", Some(8.0), 2.0..=20.0)?;
        let frequency = number_arg(args, "trap_frequency_ha", Some(0.55), 0.10..=4.0)?;
        let softening = number_arg(args, "softening_bohr", Some(1.0), 0.25..=4.0)?;
        let area = number_arg(args, "transverse_area_bohr2", Some(4.0), 1.0..=100.0)?;
        let field = number_arg(args, "external_field_ha_per_bohr", Some(0.0), -0.25..=0.25)?;
        let mixing = number_arg(args, "density_mixing", Some(0.3), 0.05..=0.8)?;
        let energy_tol = number_arg(args, "energy_tolerance_ha", Some(1e-8), 1e-10..=1e-5)?;
        let density_tol = number_arg(args, "density_tolerance_e", Some(1e-7), 1e-9..=1e-4)?;

        let trap = Trap::new(sites, width, frequency, softening, area, field);
        let scratch = 64 * sites * sites;
        let eigensolve_work = 600 * sites.pow(3);
        self.charge(eigensolve_work, scratch)?;
        let mut density = charge_density(&ground_orbital(&trap.one_body, sites)?);
        let mut previous_energy = f64::INFINITY;
        for iteration in 1..=max_iterations {
            self.charge(eigensolve_work, scratch)?;
            let trial = trap.functional(&density);
            let orbital = ground_orbital(&trial.hamiltonian, sites)?;
            let next_density = charge_density(&orbital);
            let density_change: f64 = density
                .iter()
                .zip(&next_density)
                .map(|(before, after)| (after - before).abs())
                .sum();
            let next = trap.functional(&next_density);
            let energy = 2.0 * dot_slice(&orbital, &matvec(&trap.one_body, &orbital))
                + next.hartree_energy
                + next.xc_energy;
            let orbital_energy = dot_slice(&orbital, &matvec(&next.hamiltonian, &orbital));
            let residual = eigen_residual(&next.hamiltonian, &orbital, orbital_energy);
            let electron_count: f64 = next_density.iter().sum();
            let energy_change = (energy - previous_energy).abs();
            if !energy.is_finite()
                || !orbital_energy.is_finite()
                || !residual.is_finite()
                || (electron_count - 2.0).abs() > 1e-10
            {
                return Err("LDA/VWN trap returned a nonfinite or unnormalized state".into());
            }
            if energy_change <= energy_tol && density_change <= density_tol && residual <= 1e-7 {
                let dipole = -dot_slice(&trap.positions, &next_density);
                let rho = next_density
                    .iter()
                    .map(|q| q / trap.volume)
                    .collect::<Vec<_>>();
                let psi = orbital
                    .iter()
                    .map(|c| c / trap.spacing.sqrt())
                    .collect::<Vec<_>>();
                let edge_count = next_density[0] + next_density[sites - 1];
                let mut digest =
                    blake3::Hasher::new_derive_key("qualia restricted LDA VWN trap v1");
                for count in [sites, max_iterations] {
                    digest.update(&(count as u64).to_le_bytes());
                }
                hash_floats(
                    &mut digest,
                    [
                        width,
                        frequency,
                        softening,
                        area,
                        field,
                        mixing,
                        energy_tol,
                        density_tol,
                    ],
                );
                let receipt = seal_diagnostics(
                    self,
                    "restricted-lda-vwn-soft-coulomb-trap",
                    &digest.finalize().to_hex().to_string(),
                    &[
                        ("iterations", iteration as f64),
                        ("electronic_energy_ha", energy),
                        ("orbital_energy_ha", orbital_energy),
                        ("hartree_energy_ha", next.hartree_energy),
                        ("exchange_correlation_energy_ha", next.xc_energy),
                        ("energy_change_ha", energy_change),
                        ("density_fixed_point_l1_e", density_change),
                        ("orbital_residual_ha", residual),
                        ("electron_count", electron_count),
                        ("edge_electron_count", edge_count),
                        ("electronic_dipole_e_bohr", dipole),
                    ],
                )?;
                let density = output_plane(self, vec![sites as u32], &[&rho])?;
                let orbital = output_plane(self, vec![sites as u32], &[&psi])?;
                return Ok(Value::Record(Args::from([
                    ("density_e_per_bohr3".into(), density),
                    ("orbital_per_sqrt_bohr".into(), orbital),
                    ("grid_origin_bohr".into(), Value::F64(trap.positions[0])),
                    ("grid_spacing_bohr".into(), Value::F64(trap.spacing)),
                    ("electronic_energy_ha".into(), Value::F64(energy)),
                    ("orbital_energy_ha".into(), Value::F64(orbital_energy)),
                    ("electronic_dipole_e_bohr".into(), Value::F64(dipole)),
                    ("receipt".into(), receipt),
                ])));
            }
            if iteration == max_iterations {
                return Err(format!(
                    "LDA/VWN trap did not converge in {iteration} sweeps, energy change {energy_change:.3e} Ha, density change {density_change:.3e} e, orbital residual {residual:.3e} Ha"
                ));
            }
            previous_energy = energy;
            for (value, target) in density.iter_mut().zip(next_density) {
                *value = (1.0 - mixing) * *value + mixing * target;
            }
        }
        Err("LDA/VWN SCF iteration budget exhausted".into())
    }
}

struct Trap {
    positions: Vec<f64>,
    spacing: f64,
    volume: f64,
    one_body: Vec<f64>,
    coulomb: Vec<f64>,
}

struct FunctionalEvaluation {
    hamiltonian: Vec<f64>,
    hartree_energy: f64,
    xc_energy: f64,
}

impl Trap {
    fn new(
        sites: usize,
        width: f64,
        frequency: f64,
        softening: f64,
        area: f64,
        field: f64,
    ) -> Self {
        // Place the unknowns at interior sites, with zero wavefunction at ±width.
        let spacing = 2.0 * width / (sites + 1) as f64;
        let positions = (1..=sites)
            .map(|i| -width + i as f64 * spacing)
            .collect::<Vec<_>>();
        let mut one_body = vec![0.0; sites * sites];
        let mut coulomb = vec![0.0; sites * sites];
        let laplacian_scale = 0.5 / spacing.powi(2);
        for (i, &x) in positions.iter().enumerate() {
            one_body[i * sites + i] =
                2.0 * laplacian_scale + 0.5 * (frequency * x).powi(2) + field * x;
            if i + 1 < sites {
                one_body[i * sites + i + 1] = -laplacian_scale;
                one_body[(i + 1) * sites + i] = -laplacian_scale;
            }
            for (j, &y) in positions.iter().enumerate() {
                coulomb[i * sites + j] = 1.0 / (softening.powi(2) + (x - y).powi(2)).sqrt();
            }
        }
        Self {
            positions,
            spacing,
            volume: spacing * area,
            one_body,
            coulomb,
        }
    }

    fn functional(&self, density: &[f64]) -> FunctionalEvaluation {
        let mut hamiltonian = self.one_body.clone();
        let hartree_potential = matvec(&self.coulomb, density);
        let hartree_energy = 0.5 * dot_slice(density, &hartree_potential);
        let mut xc_energy = 0.0;
        let sites = self.positions.len();
        for (i, (&charge, &vh)) in density.iter().zip(&hartree_potential).enumerate() {
            let rho = charge / self.volume;
            let (ex, vx) = lda_exchange(rho);
            let (ec, vc) = lda_correlation_vwn(rho);
            xc_energy += charge * (ex + ec);
            hamiltonian[i * sites + i] += vh + vx + vc;
        }
        FunctionalEvaluation {
            hamiltonian,
            hartree_energy,
            xc_energy,
        }
    }
}

fn ground_orbital(hamiltonian: &[f64], sites: usize) -> Result<Vec<f64>, String> {
    let spectrum = RealEigensystem::solve(hamiltonian, sites)?;
    let energy = spectrum.energy(0);
    let gap = spectrum.energy(1) - energy;
    let mut orbital = spectrum.vector(0);
    let residual = eigen_residual(hamiltonian, &orbital, energy);
    if !gap.is_finite() || gap <= 1e-9 || !residual.is_finite() || residual > 1e-10 {
        return Err("LDA/VWN trap ground orbital is unresolved".into());
    }
    // Fix the arbitrary phase for a reproducible retained orbital.
    let reference = orbital
        .iter()
        .copied()
        .max_by(|a, b| a.abs().total_cmp(&b.abs()));
    if reference.is_some_and(|value| value < 0.0) {
        for value in &mut orbital {
            *value = -*value;
        }
    }
    Ok(orbital)
}

fn charge_density(orbital: &[f64]) -> Vec<f64> {
    orbital.iter().map(|c| 2.0 * c * c).collect()
}
