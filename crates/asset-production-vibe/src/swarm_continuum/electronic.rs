//! Hermitian fixed-basis force regression and small electronic teaching models.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{int_arg, number_arg, seal_diagnostics};
use crate::{fields, list, num, record, required, text, Args, AssetHost, Value};
use qualia_core_db::solvers::linear_algebra::eigen::symmetric_eigen;

#[cfg(not(target_arch = "wasm32"))]
mod scf;

impl AssetHost<'_> {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn solve_dft_scf(&mut self, _args: &Args) -> Result<Value, String> {
        Err("LDA/VWN SCF requires the native QualiaDB chemistry host".into())
    }

    pub(crate) fn derive_hellmann_feynman_force_field(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "solver",
                "dimension",
                "hamiltonian_ha",
                "derivatives_ha_per_bohr",
                "eigenstate_index",
                "step_bohr",
                "force_tolerance_ha_per_bohr",
            ],
        )?;
        if let Some(solver) = args.get("solver") {
            if text(solver)? != "fixed_basis_hermitian" {
                return Err("Supported solver is fixed_basis_hermitian".into());
            }
        }
        let dimension = int_arg(args, "dimension", None, 2..=12)?;
        let index = int_arg(args, "eigenstate_index", Some(0), 0..=dimension - 1)?;
        let step = number_arg(args, "step_bohr", Some(1e-4), 1e-6..=1e-2)?;
        let tolerance = number_arg(args, "force_tolerance_ha_per_bohr", Some(1e-6), 1e-9..=1e-4)?;
        let derivatives = list(required(args, "derivatives_ha_per_bohr")?)?;
        if !(1..=24).contains(&derivatives.len()) {
            return Err("Provide 1..=24 Hermitian derivative matrices".into());
        }
        let side = 2 * dimension;
        let nsolves = 1 + 4 * derivatives.len();
        self.charge(600 * nsolves * side.pow(3), 56 * side * side)?;
        let mut hamiltonian = hermitian_embedding(
            required(args, "hamiltonian_ha")?,
            dimension,
            "hamiltonian_ha",
        )?;
        let mut digest = blake3::Hasher::new_derive_key("qualia fixed basis Hermitian forces v1");
        for count in [dimension, index, derivatives.len()] {
            digest.update(&(count as u64).to_le_bytes());
        }
        hash_floats(&mut digest, [step, tolerance].into_iter());
        hash_floats(&mut digest, hamiltonian.iter().copied());

        // Removing a constant identity shift reduces cancellation in differences.
        let shift = (0..dimension)
            .map(|i| hamiltonian[i * side + i])
            .sum::<f64>()
            / dimension as f64;
        for i in 0..side {
            hamiltonian[i * side + i] -= shift;
        }
        let base = isolated_hermitian_state(&hamiltonian, dimension, index)?;
        let mut forces = Vec::with_capacity(derivatives.len());
        let mut difference_forces = Vec::with_capacity(derivatives.len());
        let mut max_force_error = 0.0_f64;
        let mut max_step_error = 0.0_f64;
        let mut min_gap = base.gap;
        let mut max_residual = base.residual;
        for (coordinate, derivative) in derivatives.iter().enumerate() {
            let key = format!("derivatives_ha_per_bohr[{coordinate}]");
            let derivative = hermitian_embedding(derivative, dimension, &key)?;
            hash_floats(&mut digest, derivative.iter().copied());
            // The Frobenius norm bounds the eigenvalue displacement along the
            // entire perturbation. Divide out the duplicated real embedding.
            let derivative_norm = (derivative.iter().map(|x| x * x).sum::<f64>() / 2.0).sqrt();
            if step * derivative_norm > base.gap / 8.0 {
                return Err(format!(
                    "{key} requires a smaller step_bohr to isolate the branch"
                ));
            }
            let analytical = -dot_slice(&base.vector, &matvec(&derivative, &base.vector));
            let mut centered_force = |delta: f64| -> Result<f64, String> {
                let evaluate = |offset: f64| -> Result<HermitianState, String> {
                    let moved = hamiltonian
                        .iter()
                        .zip(&derivative)
                        .map(|(value, slope)| value + offset * slope)
                        .collect::<Vec<_>>();
                    isolated_hermitian_state(&moved, dimension, index)
                };
                let before = evaluate(-delta).map_err(|error| format!("{key}: {error}"))?;
                let after = evaluate(delta).map_err(|error| format!("{key}: {error}"))?;
                min_gap = min_gap.min(before.gap).min(after.gap);
                max_residual = max_residual.max(before.residual).max(after.residual);
                Ok((before.energy - after.energy) / (2.0 * delta))
            };
            let coarse = centered_force(step)?;
            let fine = centered_force(step / 2.0)?;
            let extrapolated = (4.0 * fine - coarse) / 3.0;
            let step_error = (fine - coarse).abs() / 3.0;
            let force_error = (analytical - extrapolated).abs();
            if !force_error.is_finite() || force_error > tolerance || step_error > tolerance {
                return Err(format!(
                    "{key} force regression exceeds {tolerance:.3e} Ha/Bohr, analytic discrepancy {force_error:.3e}, step error {step_error:.3e}"
                ));
            }
            max_force_error = max_force_error.max(force_error);
            max_step_error = max_step_error.max(step_error);
            forces.push(Value::F64(analytical));
            difference_forces.push(Value::F64(extrapolated));
        }
        let energy = base.energy + shift;
        let receipt = seal_diagnostics(
            self,
            "fixed-basis-hellmann-feynman",
            &digest.finalize().to_hex().to_string(),
            &[
                ("energy_ha", energy),
                ("minimum_branch_gap_ha", min_gap),
                ("maximum_eigenpair_residual_ha", max_residual),
                (
                    "maximum_force_regression_error_ha_per_bohr",
                    max_force_error,
                ),
                ("maximum_difference_step_error_ha_per_bohr", max_step_error),
            ],
        )?;
        Ok(Value::Record(Args::from([
            ("energy_ha".into(), Value::F64(energy)),
            ("forces_ha_per_bohr".into(), Value::List(forces.into())),
            (
                "finite_difference_forces_ha_per_bohr".into(),
                Value::List(difference_forces.into()),
            ),
            ("minimum_branch_gap_ha".into(), Value::F64(min_gap)),
            (
                "maximum_force_error_ha_per_bohr".into(),
                Value::F64(max_force_error),
            ),
            ("receipt".into(), receipt),
        ])))
    }
}

struct HermitianState {
    energy: f64,
    vector: Vec<f64>,
    gap: f64,
    residual: f64,
}

fn hermitian_embedding(value: &Value, side: usize, key: &str) -> Result<Vec<f64>, String> {
    let source = record(value)?;
    fields(source, &["real", "imaginary"])?;
    let read = |value: &Value| -> Result<Vec<f64>, String> {
        let row_major = list(value)?;
        if row_major.len() != side * side {
            return Err(format!("{key} component requires {} values", side * side));
        }
        row_major
            .iter()
            .map(|value| {
                let value = num(value)?;
                if (-1e4..=1e4).contains(&value) {
                    Ok(value)
                } else {
                    Err(format!("{key} entries must be finite and bounded by 1e4"))
                }
            })
            .collect()
    };
    let real = read(required(source, "real")?)?;
    let imaginary = match source.get("imaginary") {
        Some(entries) => read(entries)?,
        None => vec![0.0; side * side],
    };
    let mut output = vec![0.0; 4 * side * side];
    let n = 2 * side;
    // Split a complex vector into its real then imaginary components.
    for row in 0..side {
        for col in 0..side {
            let k = row * side + col;
            let t = col * side + row;
            if real[k] != real[t] || imaginary[k] != -imaginary[t] {
                return Err(format!("{key} must be exactly Hermitian at [{row}, {col}]"));
            }
            output[row * n + col] = real[k];
            output[row * n + side + col] = -imaginary[k];
            output[(row + side) * n + col] = imaginary[k];
            output[(row + side) * n + side + col] = real[k];
        }
    }
    Ok(output)
}

fn isolated_hermitian_state(
    matrix: &[f64],
    dimension: usize,
    index: usize,
) -> Result<HermitianState, String> {
    let side = 2 * dimension;
    let spectrum = RealEigensystem::solve(matrix, side)?;
    let scale = matrix
        .iter()
        .fold(1.0_f64, |scale, value| scale.max(value.abs()));
    let pair = 2 * index;
    let first = spectrum.energy(pair);
    let second = spectrum.energy(pair + 1);
    if (first - second).abs() > 1e-11 * scale {
        return Err("Complex Hermitian eigenvalue failed the doubled-spectrum check".into());
    }
    let energy = 0.5 * (first + second);
    let mut gap = f64::INFINITY;
    if pair > 0 {
        gap = gap.min(energy - spectrum.energy(pair - 1));
    }
    if pair + 2 < side {
        gap = gap.min(spectrum.energy(pair + 2) - energy);
    }
    if !gap.is_finite() || gap <= 1e-9 * scale {
        return Err("Selected Hermitian eigenstate must be spectrally isolated".into());
    }
    let vector = spectrum.vector(pair);
    let residual = eigen_residual(matrix, &vector, energy);
    if !residual.is_finite() || residual > (1e-11 * scale).min(1e-4 * gap) {
        return Err("Hermitian eigenpair residual exceeds the regression limit".into());
    }
    Ok(HermitianState {
        energy,
        vector,
        gap,
        residual,
    })
}

struct RealEigensystem {
    diagonal: Vec<f64>,
    vectors: Vec<f64>,
    order: Vec<usize>,
}

impl RealEigensystem {
    fn solve(matrix: &[f64], side: usize) -> Result<Self, String> {
        let mut diagonal = matrix.to_vec();
        let mut vectors = vec![0.0; matrix.len()];
        symmetric_eigen(side, &mut diagonal, &mut vectors)
            .map_err(|error| format!("Native symmetric eigensolver: {error:?}"))?;
        let mut order = (0..side).collect::<Vec<_>>();
        order.sort_by(|&a, &b| diagonal[a * side + a].total_cmp(&diagonal[b * side + b]));
        Ok(Self {
            diagonal,
            vectors,
            order,
        })
    }

    fn energy(&self, state: usize) -> f64 {
        let index = self.order[state];
        self.diagonal[index * self.order.len() + index]
    }

    fn vector(&self, state: usize) -> Vec<f64> {
        let index = self.order[state];
        self.vectors
            .chunks_exact(self.order.len())
            .map(|row| row[index])
            .collect()
    }
}

fn matvec(matrix: &[f64], vector: &[f64]) -> Vec<f64> {
    matrix
        .chunks_exact(vector.len())
        .map(|row| dot_slice(row, vector))
        .collect()
}

fn dot_slice(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn eigen_residual(matrix: &[f64], vector: &[f64], energy: f64) -> f64 {
    matvec(matrix, vector)
        .into_iter()
        .zip(vector)
        .map(|(hv, v)| (hv - energy * v).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn hash_floats(digest: &mut blake3::Hasher, values: impl IntoIterator<Item = f64>) {
    for value in values {
        digest.update(&value.to_bits().to_le_bytes());
    }
}
