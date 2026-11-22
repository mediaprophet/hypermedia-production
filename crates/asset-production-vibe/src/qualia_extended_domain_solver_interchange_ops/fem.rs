//! Explicit planar FEM and shifted structural stability diagnostics.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, sync::Arc};

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use kinetics_continuum_multiphysics::{CholeskyFactor, ExplicitIntegrator, LdltFactor};
use qualia_core_db::{
    solvers::linear_algebra::eigen::symmetric_eigen,
    specialized_libs::engineering_analysis::fem::{assemble_mass, assemble_stiffness, FeElement, FeModel, FeNode},
};

use crate::{count, fields, integer, list, num, required, Args, AssetHost, TypedArtifact, Value};

const MAX_NODES: usize = 96;
const MAX_ELEMENTS: usize = 512;
const MAX_STEPS: usize = 4096;
const MAX_BUCKLING_DOFS: usize = 64;
const MAX_RELATIVE_ENERGY_RESIDUAL: f64 = 0.01;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Value, String> {
    match op {
        "explicit_truss" => explicit_truss(host, args),
        "buckling_inertia" => buckling_inertia(host, args),
        _ => Err(format!("Unsupported structural dynamics operation: {op}")),
    }
}

fn explicit_truss(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["nodes_m", "elements", "fixed_dofs", "loads_n", "initial_displacements_m",
        "initial_velocities_m_s", "dt_s", "steps"])?;
    let nodes_input = list(required(args, "nodes_m")?)?;
    let elements_input = list(required(args, "elements")?)?;
    if !(2..=MAX_NODES).contains(&nodes_input.len()) || !(1..=MAX_ELEMENTS).contains(&elements_input.len()) {
        return Err("Truss nodes or bars exceed the supported bounds".into());
    }
    let dt = num(required(args, "dt_s")?)?;
    required(args, "steps")?;
    let steps = count(args, "steps", 1, MAX_STEPS)?;
    if dt <= 0.0 || !((steps as f64) * dt).is_finite() { return Err("Invalid explicit step duration".into()); }
    let physical_dofs = 2 * nodes_input.len();
    let native_dofs = 3 * nodes_input.len();
    host.charge(
        (2 * steps + 4) * physical_dofs * physical_dofs + elements_input.len() * 32,
        16 * native_dofs * native_dofs + 8 * physical_dofs * physical_dofs + 1024 * (steps + 1),
    )?;

    let nodes = nodes_input.iter().map(|node| {
        let [x, y] = list(node)? else { return Err("Each node requires [x_m, y_m]".into()); };
        Ok(FeNode { x: num(x)?, y: num(y)? })
    }).collect::<Result<Vec<_>, String>>()?;
    let elements = elements_input.iter().map(|bar| {
        let [left, right, modulus, area, density] = list(bar)?
        else { return Err("Each element requires [node_i, node_j, E_pa, area_m2, density_kg_m3]".into()); };
        let ni = integer(left, "node_i")?;
        let nj = integer(right, "node_j")?;
        let [e, area, rho] = [num(modulus)?, num(area)?, num(density)?];
        if ni == nj || ni >= nodes.len() || nj >= nodes.len() || e <= 0.0 || area <= 0.0 || rho <= 0.0 {
            return Err("Invalid truss bar topology or material".into());
        }
        Ok(FeElement::Truss { ni, nj, e, area, rho })
    }).collect::<Result<Vec<_>, String>>()?;
    let mut fixed = vec![false; physical_dofs];
    let constraints = list(required(args, "fixed_dofs")?)?;
    if constraints.len() >= physical_dofs { return Err("Truss requires at least one free translation DOF".into()); }
    for value in constraints {
        let slot = integer(value, "fixed_dofs entry")?;
        let lane = fixed.get_mut(slot).ok_or("Fixed truss DOF is out of range")?;
        if std::mem::replace(lane, true) { return Err("Fixed truss DOF is repeated".into()); }
    }
    let model = FeModel { nodes, elements, constraints: Vec::new(), loads: Vec::new() };
    let stiffness = assemble_stiffness(&model).map_err(|error| format!("Native truss stiffness failed: {error:?}"))?;
    let mass = assemble_mass(&model).map_err(|error| format!("Native truss mass failed: {error:?}"))?;
    let native_slot = |slot: usize| 3 * (slot / 2) + slot % 2;
    let physical_mass = (0..physical_dofs).map(|i| {
        let j = native_slot(i);
        mass[j * native_dofs + j]
    }).collect::<Vec<_>>();
    if physical_mass.iter().any(|m| !m.is_finite() || *m < 1e-12) {
        return Err("Truss contains an unsupported unloaded node or a mass below the native integrator limit".into());
    }
    let free = (0..physical_dofs).filter(|i| !fixed[*i]).collect::<Vec<_>>();
    let reduced_mass = free.iter().map(|&i| physical_mass[i]).collect::<Vec<_>>();
    let n = free.len();
    let mut k = Vec::with_capacity(n * n);
    for &i in &free {
        k.extend(free.iter().map(|&j| stiffness[native_slot(i) * native_dofs + native_slot(j)]));
    }
    let omega_squared_bound = k.chunks_exact(n).zip(&reduced_mass).map(|(row, &mi)| {
        row.iter().zip(&reduced_mass).map(|(ki, &mj)| ki.abs() / (mi.sqrt() * mj.sqrt())).sum::<f64>()
    }).collect::<Vec<_>>();
    if omega_squared_bound.iter().any(|row| !row.is_finite()) {
        return Err("Truss stiffness cannot produce a finite CFL bound".into());
    }
    let largest_bound = omega_squared_bound.into_iter().fold(0.0, f64::max);
    if largest_bound <= 0.0 { return Err("Truss has no elastic stiffness on its free translations".into()); }
    let safe_dt = 1.8 / largest_bound.sqrt();
    if dt > safe_dt { return Err(format!("Explicit step {dt:e} s exceeds the truss CFL limit {safe_dt:e} s")); }

    let load = lanes(args, "loads_n", physical_dofs)?;
    let displacement = lanes(args, "initial_displacements_m", physical_dofs)?;
    let velocity = lanes(args, "initial_velocities_m_s", physical_dofs)?;
    if fixed.iter().enumerate().any(|(i, &f)| f && (displacement[i] != 0.0 || velocity[i] != 0.0)) {
        return Err("Fixed truss DOFs require zero initial motion".into());
    }
    let force = free.iter().map(|&i| load[i]).collect::<Vec<_>>();
    let origin = free.iter().map(|&i| displacement[i]).collect::<Vec<_>>();
    let v0 = free.iter().map(|&i| velocity[i]).collect::<Vec<_>>();
    let initial_ku = matvec(&k, &origin);
    let mut solver = ExplicitIntegrator::new(n, dt, reduced_mass);
    solver.state.displacements.copy_from_slice(&origin);
    for (i, v) in solver.state.velocities.iter_mut().enumerate() {
        let acceleration = (force[i] - initial_ku[i]) / solver.lumped_mass[i];
        *v = v0[i] - 0.5 * dt * acceleration;
    }
    let (initial_kinetic, initial_strain) = energy(&solver.lumped_mass, &origin, &v0, &initial_ku);
    let initial_energy = initial_kinetic + initial_strain;
    if !initial_energy.is_finite() { return Err("Initial truss energy is nonfinite".into()); }

    let mut workbook = Workbook::new();
    let sheet_index = workbook.add_sheet("Explicit truss energy");
    let sheet = workbook.sheet_mut(sheet_index).ok_or("Could not create the explicit energy table")?;
    for (column, header) in ["time_s", "kinetic_j", "strain_j", "input_work_j", "relative_balance_residual"].iter().enumerate() {
        sheet.set_value(CellRef::new(0, column as u32), CellValue::Text((*header).into()));
    }
    let mut write_row = |row: usize, values: [f64; 5]| {
        for (column, value) in values.into_iter().enumerate() {
            sheet.set_value(CellRef::new(row as u32, column as u32), CellValue::Number(value));
        }
    };
    write_row(1, [0.0, initial_kinetic, initial_strain, 0.0, 0.0]);
    let mut full_velocity = vec![0.0; n];
    let mut max_relative = 0.0_f64;
    let mut internal = initial_ku;
    for step in 1..=steps {
        solver.step(&force, |_| std::mem::take(&mut internal));
        let u = &solver.state.displacements;
        internal = matvec(&k, u);
        for (i, v) in full_velocity.iter_mut().enumerate() {
            let acceleration = (force[i] - internal[i]) / solver.lumped_mass[i];
            *v = solver.state.velocities[i] + 0.5 * dt * acceleration;
        }
        let (kinetic, strain) = energy(&solver.lumped_mass, u, &full_velocity, &internal);
        let work = force.iter().zip(u.iter().zip(&origin)).map(|(f, (u, u0))| f * (u - u0)).sum::<f64>();
        let current = kinetic + strain;
        let scale = initial_energy.abs().max(current.abs()).max(work.abs()).max(1e-15);
        let relative = (current - initial_energy - work).abs() / scale;
        if !relative.is_finite() || !solver.state.time.is_finite() {
            return Err(format!("Nonfinite truss trajectory at step {step}"));
        }
        if relative >= MAX_RELATIVE_ENERGY_RESIDUAL {
            return Err(format!("Truss energy/work balance residual {relative:e} at step {step} exceeds the one-percent gate"));
        }
        max_relative = max_relative.max(relative);
        write_row(step + 1, [solver.state.time, kinetic, strain, work, relative]);
    }
    let result = |v: &[f64]| {
        let mut lanes = vec![Value::F64(0.0); physical_dofs];
        for (&i, &x) in free.iter().zip(v) { lanes[i] = Value::F64(x); }
        Value::List(lanes)
    };
    let energy_table = host.admit_artifact(TypedArtifact::TabularCube(Arc::new(workbook)))?;
    Ok(Value::Record(BTreeMap::from([
        ("displacements_m".into(), result(&solver.state.displacements)),
        ("velocities_m_s".into(), result(&full_velocity)),
        ("energy".into(), energy_table),
        ("max_relative_energy_residual".into(), Value::F64(max_relative)),
        ("safe_dt_s".into(), Value::F64(safe_dt)),
    ])))
}

fn lanes(args: &Args, key: &str, n: usize) -> Result<Vec<f64>, String> {
    let Some(value) = args.get(key) else { return Ok(vec![0.0; n]); };
    let values = list(value)?;
    if values.len() != n { return Err(format!("{key} requires {n} translational DOF values")); }
    values.iter().map(num).collect()
}

fn matvec(matrix: &[f64], vector: &[f64]) -> Vec<f64> {
    matrix.chunks_exact(vector.len()).map(|row| row.iter().zip(vector).map(|(a, x)| a * x).sum()).collect()
}

fn energy(mass: &[f64], u: &[f64], v: &[f64], ku: &[f64]) -> (f64, f64) {
    let kinetic = mass.iter().zip(v).map(|(m, v)| m * v * v).sum::<f64>() * 0.5;
    let strain = u.iter().zip(ku).map(|(u, f)| u * f).sum::<f64>() * 0.5;
    (kinetic, strain)
}

fn buckling_inertia(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["elastic_stiffness", "geometric_stiffness", "shift", "modes"])?;
    let elastic = list(required(args, "elastic_stiffness")?)?;
    let geometric = list(required(args, "geometric_stiffness")?)?;
    let n = elastic.len();
    if !(1..=MAX_BUCKLING_DOFS).contains(&n) || geometric.len() != n {
        return Err("Buckling matrices require equal supported dimensions".into());
    }
    let shift = num(required(args, "shift")?)?;
    if shift < 0.0 { return Err("Buckling shift must be nonnegative".into()); }
    let modes = args.get("modes").map(|_| count(args, "modes", 1, n)).transpose()?;
    host.charge(if modes.is_some() { 400 * n * n * n } else { 5 * n * n * n }, 112 * n * n)?;
    let (elastic, _, elastic_scale) = symmetric_positive(elastic, n, "elastic_stiffness")?;
    let (geometric, geometric_factor, geometric_scale) = symmetric_positive(geometric, n, "geometric_stiffness")?;
    let mut pencil = elastic.iter().zip(&geometric).map(|(k, g)| k - shift * g).collect::<Vec<_>>();
    let scale = pencil.iter().map(|v| v.abs()).fold(0.0, f64::max);
    if scale == 0.0 || pencil.iter().any(|v| !v.is_finite()) {
        return Err("Shifted buckling pencil is singular or nonfinite".into());
    }
    pencil.iter_mut().for_each(|v| *v /= scale);
    let factor = LdltFactor::factorize(n, &pencil).map_err(|error| error.to_owned())?;
    let pivot_tolerance = 1e-12;
    if factor.d_values.iter().any(|p| !p.is_finite() || p.abs() <= pivot_tolerance) {
        return Err("Shifted buckling pencil has an unsafe LDLᵀ pivot. The native factorization requires a separated shift and safe unpivoted pivots".into());
    }
    let mut residual = 0.0_f64;
    for i in 0..n {
        for j in 0..=i {
            let reconstructed = (0..=j).map(|q| {
                factor.l_values[i * n + q] * factor.d_values[q] * factor.l_values[j * n + q]
            }).sum::<f64>();
            let error = (reconstructed - pencil[i * n + j]).abs();
            if !error.is_finite() { return Err("Shifted buckling factorization is nonfinite".into()); }
            residual = residual.max(error);
        }
    }
    if residual > 1e-9 { return Err(format!("Shifted LDLᵀ normalized reconstruction residual {residual:e} exceeds tolerance")); }
    let inertia = factor.inertia(pivot_tolerance);
    let mut result = BTreeMap::from([
        ("shift".into(), Value::F64(shift)),
        ("modes_below_shift".into(), Value::U64(inertia.negative as u64)),
        ("modes_above_shift".into(), Value::U64(inertia.positive as u64)),
        ("normalized_factor_residual".into(), Value::F64(residual)),
    ]);
    if let Some(count) = modes {
        let pairs = generalized_modes(
            &elastic, &geometric, elastic_scale, geometric_scale, &geometric_factor, count,
        )?;
        result.insert("modes".into(), Value::List(pairs));
    }
    Ok(Value::Record(result))
}

fn symmetric_positive(rows: &[Value], n: usize, label: &str) -> Result<(Vec<f64>, CholeskyFactor, f64), String> {
    let mut matrix = Vec::with_capacity(n * n);
    for row in rows {
        let row = list(row)?;
        if row.len() != n { return Err(format!("{label} is not square")); }
        for value in row { matrix.push(num(value)?); }
    }
    let scale = matrix.iter().map(|v| v.abs()).fold(0.0, f64::max);
    if scale == 0.0 { return Err(format!("{label} has zero stiffness")); }
    for i in 0..n {
        for j in 0..i {
            let a = matrix[i * n + j];
            let b = matrix[j * n + i];
            if (a / scale - b / scale).abs() > 1e-12 { return Err(format!("{label} is not symmetric")); }
            let value = 0.5 * a + 0.5 * b;
            matrix[i * n + j] = value;
            matrix[j * n + i] = value;
        }
    }
    let normalized = matrix.iter().map(|v| v / scale).collect::<Vec<_>>();
    let factor = CholeskyFactor::factorize(n, &normalized)
        .map_err(|error| format!("{label} must be positive definite: {error}"))?;
    if (0..n).any(|i| !factor.l_values[i * n + i].is_finite() || factor.l_values[i * n + i] <= 1e-6) {
        return Err(format!("{label} is too ill-conditioned for buckling inertia"));
    }
    Ok((matrix, factor, scale))
}

/// Dense generalized modes for SPD pencils by Cholesky congruence and native
/// Jacobi eigendecomposition. The shifted inertia count is independent.
fn generalized_modes(
    elastic: &[f64], geometric: &[f64], elastic_scale: f64, geometric_scale: f64,
    factor: &CholeskyFactor, count: usize,
) -> Result<Vec<Value>, String> {
    let n = factor.n;
    let l = &factor.l_values;
    let mut left = elastic.iter().map(|v| v / elastic_scale).collect::<Vec<_>>();
    for row in 0..n {
        for col in 0..n {
            let contribution = (0..row).map(|k| l[row * n + k] * left[k * n + col]).sum::<f64>();
            left[row * n + col] = (left[row * n + col] - contribution) / l[row * n + row];
        }
    }
    let mut transformed = left;
    for col in 0..n {
        for row in 0..n {
            let contribution = (0..col).map(|k| l[col * n + k] * transformed[row * n + k]).sum::<f64>();
            transformed[row * n + col] = (transformed[row * n + col] - contribution) / l[col * n + col];
        }
    }
    if transformed.iter().any(|v| !v.is_finite()) { return Err("Nonfinite generalized buckling operator".into()); }
    for i in 0..n {
        for j in 0..i {
            let value = 0.5 * transformed[i * n + j] + 0.5 * transformed[j * n + i];
            transformed[i * n + j] = value;
            transformed[j * n + i] = value;
        }
    }
    let mut eigvectors = vec![0.0; n * n];
    symmetric_eigen(n, &mut transformed, &mut eigvectors).map_err(|error| format!("Native modal decomposition failed: {error:?}"))?;
    let mut order = (0..n).collect::<Vec<_>>();
    order.sort_by(|&i, &j| transformed[i * n + i].total_cmp(&transformed[j * n + j]));
    let mut pairs = Vec::with_capacity(count);
    for &col in &order[..count] {
        let theta = transformed[col * n + col];
        let load = theta * (elastic_scale / geometric_scale);
        let mut shape = vec![0.0; n];
        for i in (0..n).rev() {
            let suffix = ((i + 1)..n).map(|k| l[k * n + i] * shape[k]).sum::<f64>();
            shape[i] = (eigvectors[i * n + col] - suffix) / l[i * n + i];
        }
        let norm = shape.iter().map(|q| q * q).sum::<f64>().sqrt();
        if !load.is_finite() || load <= 0.0 || !norm.is_finite() || norm <= 0.0 {
            return Err("Generalized buckling mode is nonfinite or nonpositive".into());
        }
        let sign = shape.iter().max_by(|a, b| a.abs().total_cmp(&b.abs())).ok_or("Missing mode shape")?.signum();
        shape.iter_mut().for_each(|q| *q *= sign / norm);
        let mut magnitude = 0.0_f64;
        let mut residual = 0.0_f64;
        for (k, g) in elastic.chunks_exact(n).zip(geometric.chunks_exact(n)) {
            let a = k.iter().zip(&shape).map(|(v, x)| (v / elastic_scale) * x).sum::<f64>();
            let b = g.iter().zip(&shape).map(|(v, x)| (v / geometric_scale) * x).sum::<f64>() * theta;
            magnitude = magnitude.max(a.abs()).max(b.abs());
            residual = residual.max((a - b).abs());
        }
        let relative = residual / magnitude;
        if !relative.is_finite() || relative > 1e-8 {
            return Err(format!("Native buckling mode {col} residual {relative:e} exceeds tolerance"));
        }
        pairs.push(Value::Record(BTreeMap::from([
            ("load_factor".into(), Value::F64(load)),
            ("mode_shape".into(), Value::List(shape.into_iter().map(Value::F64).collect())),
            ("relative_residual".into(), Value::F64(relative)),
        ])));
    }
    Ok(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{record, AssetWorkspace, ToolLimits};
    use std::sync::atomic::AtomicBool;

    fn host<'a>(limits: &'a ToolLimits, cancel: &'a AtomicBool) -> AssetHost<'a> {
        AssetHost { workspace: AssetWorkspace::default(), inputs: &[], limits, cancel, work: 0 }
    }

    fn row(values: &[f64]) -> Value {
        Value::List(values.iter().copied().map(Value::F64).collect())
    }

    #[test]
    fn planar_axial_mode_preserves_energy_and_static_supports() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = host(&limits, &cancel);
        let parameters = BTreeMap::from([
            ("nodes_m".into(), Value::List(vec![row(&[0.0, 0.0]), row(&[1.0, 0.0])])),
            ("elements".into(), Value::List(vec![Value::List(vec![
                Value::U64(0), Value::U64(1), Value::F64(100.0), Value::F64(1.0), Value::F64(2.0),
            ])])),
            ("fixed_dofs".into(), Value::List(vec![Value::U64(0), Value::U64(1), Value::U64(3)])),
            ("initial_displacements_m".into(), row(&[0.0, 0.0, 0.001, 0.0])),
            ("dt_s".into(), Value::F64(0.001)),
            ("steps".into(), Value::U64(100)),
        ]);
        let result = explicit_truss(&mut host, &parameters).unwrap();
        let fields = record(&result).unwrap();
        let displacement = list(&fields["displacements_m"]).unwrap();
        assert!((num(&displacement[2]).unwrap() - 0.001 * 1.0_f64.cos()).abs() < 1e-8);
        assert_eq!(num(&displacement[0]).unwrap(), 0.0);
        assert!(num(&fields["max_relative_energy_residual"]).unwrap() < 1e-4);
    }

    #[test]
    fn shifted_native_inertia_counts_and_validates_generalized_modes() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = host(&limits, &cancel);
        let mut parameters = BTreeMap::from([
            ("elastic_stiffness".into(), Value::List(vec![row(&[3.0, 1.0]), row(&[1.0, 3.0])])),
            ("geometric_stiffness".into(), Value::List(vec![row(&[2.0, 0.0]), row(&[0.0, 2.0])])),
            ("shift".into(), Value::F64(1.25)),
            ("modes".into(), Value::U64(2)),
        ]);
        let result = buckling_inertia(&mut host, &parameters).unwrap();
        let fields = record(&result).unwrap();
        assert_eq!(num(&fields["modes_below_shift"]).unwrap(), 1.0);
        let modes = list(&fields["modes"]).unwrap();
        for (mode, expected) in modes.iter().zip([1.0, 2.0]) {
            assert!((num(&record(mode).unwrap()["load_factor"]).unwrap() - expected).abs() < 1e-12);
        }
        parameters.insert("shift".into(), Value::F64(1.0));
        assert!(buckling_inertia(&mut host, &parameters).unwrap_err().contains("LDLᵀ pivot"));
    }
}
