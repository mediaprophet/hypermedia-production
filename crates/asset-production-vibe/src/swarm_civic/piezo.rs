//! Homogeneous small-strain piezoelectric constitutive evaluation.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use qualia_core_db::q42::material_coupling::{PiezoelectricConstitutiveModel, TensorStrain6};

use super::{least_squares, numeric_row, table_receipt};
use crate::{Args, AssetHost, Value, fields, list, record, required, vector};

fn matrix<const R: usize, const C: usize>(
    value: &Value,
    maximum_abs: f64,
) -> Result<[[f64; C]; R], String> {
    let rows = list(value)?;
    if rows.len() != R {
        return Err(format!("Constitutive matrix needs {R} rows"));
    }
    let mut out = [[0.0; C]; R];
    for (dest, source) in out.iter_mut().zip(rows) {
        *dest = vector::<C>(source)?;
        if dest.iter().any(|x| x.abs() > maximum_abs) {
            return Err("Constitutive matrix exceeds its admitted magnitude".into());
        }
    }
    Ok(out)
}

fn check_positive_definite<const N: usize>(
    a: &[[f64; N]; N],
    lower_diagonal: f64,
    name: &str,
) -> Result<(), String> {
    if (0..N).any(|i| a[i][i] < lower_diagonal) {
        return Err(format!("{name} has a nonpositive or ill-scaled diagonal"));
    }
    // Cholesky on the normalized symmetric form avoids dimensionful thresholds.
    let diagonal: [f64; N] = std::array::from_fn(|i| a[i][i].sqrt());
    let mut l = [[0.0; N]; N];
    for i in 0..N {
        for j in 0..=i {
            let scale = diagonal[i] * diagonal[j];
            if (a[i][j] - a[j][i]).abs() > 1e-12 * scale {
                return Err(format!("{name} is not symmetric at ({i}, {j})"));
            }
            let mut value = (a[i][j] + a[j][i]) * 0.5 / scale;
            for k in 0..j {
                value -= l[i][k] * l[j][k];
            }
            l[i][j] = if i == j {
                if value < 1e-12 {
                    return Err(format!(
                        "{name} fails the positive-definite gate at pivot {i}: {value:.9e}"
                    ));
                }
                value.sqrt()
            } else {
                value / l[j][j]
            };
        }
    }
    Ok(())
}

fn stress_driven_strain(
    model: &PiezoelectricConstitutiveModel,
    electric: [f64; 3],
    stress: [f64; 6],
) -> Result<[f64; 6], String> {
    // C S = T + eᵀE. C acts on Engineering Voigt shear components.
    let mut matrix = Vec::with_capacity(36);
    let mut rhs = Vec::with_capacity(6);
    for (i, (&stress, row)) in stress.iter().zip(&model.stiffness_c_e_pa).enumerate() {
        let scale = row[i];
        matrix.extend(row.iter().map(|coefficient| coefficient / scale));
        let electrode_term = model
            .piezo_e_c_per_m2
            .iter()
            .zip(electric)
            .map(|(coupling, field)| coupling[i] * field)
            .sum::<f64>();
        rhs.push((stress + electrode_term) / scale);
    }
    let result = least_squares(6, 6, matrix, rhs)?;
    let [xx, yy, zz, xy, yz, xz] = result[..] else {
        return Err("Converse piezoelectric solve requires six strain components".into());
    };
    Ok([xx, yy, zz, 0.5 * xy, 0.5 * yz, 0.5 * xz])
}

fn dot<const N: usize>(left: [f64; N], right: [f64; N]) -> f64 {
    left.into_iter().zip(right).map(|(a, b)| a * b).sum()
}

impl AssetHost<'_> {
    pub(crate) fn solve_piezoelectric_multiferroic_coupling(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "stiffness_c_e_pa",
                "permittivity_eps_s_f_per_m",
                "piezo_e_c_per_m2",
                "cases",
            ],
        )?;
        let input = list(required(args, "cases")?)?;
        if !(1..=1024).contains(&input.len()) {
            return Err("Piezoelectric evaluation requires 1–1024 cases".into());
        }
        self.charge(2048 * input.len(), 6144 * input.len())?;
        let model = PiezoelectricConstitutiveModel {
            stiffness_c_e_pa: matrix(required(args, "stiffness_c_e_pa")?, 1e13)?,
            permittivity_eps_s_f_per_m: matrix(
                required(args, "permittivity_eps_s_f_per_m")?,
                1e-3,
            )?,
            piezo_e_c_per_m2: matrix(required(args, "piezo_e_c_per_m2")?, 1e4)?,
        };
        let axial_k_squared = model
            .validate()
            .map_err(|error| format!("Native piezoelectric material validation: {error:?}"))?;
        check_positive_definite(&model.stiffness_c_e_pa, 1e3, "Elastic stiffness")?;
        check_positive_definite(
            &model.permittivity_eps_s_f_per_m,
            1e-15,
            "Clamped permittivity",
        )?;
        let zero_strain = TensorStrain6 {
            xx_yy_zz_xy_yz_xz: [0.0; 6],
        };
        let mut max_reciprocal_error = 0.0_f64;
        let mut max_energy_error = 0.0_f64;
        let mut max_stress_error = 0.0_f64;
        let mut rows = Vec::with_capacity(input.len());
        for (i, value) in input.iter().enumerate() {
            let row = record(value)?;
            fields(row, &["electric_field_v_m", "strain_tensor", "stress_pa"])?;
            let electric = vector::<3>(required(row, "electric_field_v_m")?)?;
            if electric.iter().any(|v| v.abs() > 1e9) {
                return Err(format!("Case {i}: electric field exceeds ±1e9 V/m"));
            }
            let (strain_components, target) = match (row.get("strain_tensor"), row.get("stress_pa"))
            {
                (Some(strain), None) => (vector::<6>(strain)?, None),
                (None, Some(boundary)) => {
                    let stress = vector::<6>(boundary)?;
                    if stress.iter().any(|v| v.abs() > 1e11) {
                        return Err(format!("Case {i}: prescribed stress exceeds ±1e11 Pa"));
                    }
                    (
                        stress_driven_strain(&model, electric, stress)?,
                        Some(stress),
                    )
                }
                _ => {
                    return Err(
                        "Specify exactly one strain_tensor or stress_pa boundary per case".into(),
                    );
                }
            };
            if strain_components
                .iter()
                .any(|v| !v.is_finite() || v.abs() > 0.1)
            {
                return Err(format!(
                    "Case {i}: strain exceeds the admitted constitutive range"
                ));
            }
            let strain = TensorStrain6 {
                xx_yy_zz_xy_yz_xz: strain_components,
            };
            let evaluate = |strain, electric| {
                model
                    .evaluate_coupled_state(strain, electric)
                    .map_err(|error| {
                        format!("Case {i}: native piezoelectric evaluation: {error:?}")
                    })
            };
            let state = evaluate(&strain, electric)?;
            let stress = state.stress_pa.xx_yy_zz_xy_yz_xz_pa;
            if stress.iter().any(|v| !v.is_finite() || v.abs() > 1e11) {
                return Err(format!("Case {i}: evaluated stress exceeds ±1e11 Pa"));
            }
            let mut stress_residual = 0.0_f64;
            if let Some(target) = target {
                let scale = target.iter().map(|v| v.abs()).fold(1.0_f64, f64::max);
                stress_residual = stress
                    .into_iter()
                    .zip(target)
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0_f64, f64::max);
                if stress_residual > scale * 1e-9 {
                    return Err(format!(
                        "Case {i}: prescribed stress residual {stress_residual:.9e} Pa"
                    ));
                }
            }
            let direct = evaluate(&strain, [0.0; 3])?;
            let converse = evaluate(&zero_strain, electric)?;
            let [xx, yy, zz, xy, yz, xz] = strain_components;
            let work_direct = dot(direct.electric_displacement_c_per_m2, electric);
            let work_converse = dot(
                converse.stress_pa.xx_yy_zz_xy_yz_xz_pa,
                [xx, yy, zz, 2.0 * xy, 2.0 * yz, 2.0 * xz],
            );
            let reciprocal_error = (work_direct + work_converse).abs();
            let stored = state.mechanical_energy_j_per_m3 + state.dielectric_energy_j_per_m3;
            let energy_error = (state.total_internal_energy_j_per_m3 - stored).abs();
            if !reciprocal_error.is_finite()
                || reciprocal_error
                    > 1e-10
                        * stored
                            .abs()
                            .max(work_direct.abs())
                            .max(work_converse.abs())
                            .max(1e-12)
                || !energy_error.is_finite()
                || energy_error > 1e-10 * stored.abs().max(1e-12)
            {
                return Err(format!(
                    "Case {i}: reciprocity residual {reciprocal_error:.9e}, energy residual {energy_error:.9e} J/m³"
                ));
            }
            max_reciprocal_error = max_reciprocal_error.max(reciprocal_error);
            max_energy_error = max_energy_error.max(energy_error);
            max_stress_error = max_stress_error.max(stress_residual);
            let [dx, dy, dz] = state.electric_displacement_c_per_m2;
            let [txx, tyy, tzz, txy, tyz, txz] = stress;
            rows.push(numeric_row([
                i as f64,
                electric[0],
                electric[1],
                electric[2],
                xx,
                yy,
                zz,
                xy,
                yz,
                xz,
                txx,
                tyy,
                tzz,
                txy,
                tyz,
                txz,
                dx,
                dy,
                dz,
                state.mechanical_energy_j_per_m3,
                state.dielectric_energy_j_per_m3,
                state.total_internal_energy_j_per_m3,
                axial_k_squared,
                reciprocal_error,
                energy_error,
                stress_residual,
            ]));
        }
        let mut out = table_receipt(
            self,
            "urn:qualia:civic:homogeneous-linear-piezoelectric-stress-charge",
            &[
                "case",
                "field_x_v_m",
                "field_y_v_m",
                "field_z_v_m",
                "strain_xx",
                "strain_yy",
                "strain_zz",
                "strain_xy",
                "strain_yz",
                "strain_xz",
                "stress_xx_pa",
                "stress_yy_pa",
                "stress_zz_pa",
                "stress_xy_pa",
                "stress_yz_pa",
                "stress_xz_pa",
                "displacement_x_c_m2",
                "displacement_y_c_m2",
                "displacement_z_c_m2",
                "elastic_j_m3",
                "dielectric_j_m3",
                "combined_j_m3",
                "max_axial_k_squared",
                "reciprocity_residual_j_m3",
                "internal_energy_residual_j_m3",
                "prescribed_stress_residual_pa",
            ],
            &rows,
        )?;
        out.insert("max_axial_k_squared".into(), Value::F64(axial_k_squared));
        out.insert(
            "max_reciprocity_residual_j_m3".into(),
            Value::F64(max_reciprocal_error),
        );
        out.insert(
            "max_internal_energy_residual_j_m3".into(),
            Value::F64(max_energy_error),
        );
        out.insert(
            "max_prescribed_stress_residual_pa".into(),
            Value::F64(max_stress_error),
        );
        Ok(Value::Record(out))
    }
}
