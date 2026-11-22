//! Bounded linear Hex8 elasticity and Quad4 steady conduction.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use crate::artifact::FieldLattice;
use crate::*;
use kinetics_continuum_multiphysics::{
    CholeskyFactor, Point3D, export_vtu_hex, gauss_hex_2x2x2, gauss_quad_2x2, shape_deriv_hex8,
    shape_deriv_quad4, thermal_conduction_quad4,
};

impl AssetHost<'_> {
    pub(crate) fn solve_structural_fea_multiphysics(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "nodes",
                "elements",
                "loads",
                "constraints",
                "material",
            ],
        )?;
        let mode = text(required(args, "mode")?)?;
        let material = record(required(args, "material")?)?;
        let nodes = list(required(args, "nodes")?)?;
        let elements = list(required(args, "elements")?)?;
        if !(2..=256).contains(&nodes.len()) || !(1..=4096).contains(&elements.len()) {
            return Err(
                "Finite element document exceeds the dense solver node or element budget".into(),
            );
        }
        let (width, element_work) = match mode {
            "static_hex8" => (3, 32_000),
            "steady_thermal_quad4" => (1, 800),
            _ => return Err("Unsupported finite element mode".into()),
        };
        let ndof = nodes.len() * width;
        self.charge(
            ndof.pow(3).saturating_add(elements.len() * element_work),
            ndof.pow(2) * 32 + elements.len() * 256,
        )?;
        let constraints = dof_values(required(args, "constraints")?, nodes.len(), width)?;
        if constraints.is_empty() {
            return Err("Finite element solve needs prescribed degrees of freedom".into());
        }
        let loads = dof_values(required(args, "loads")?, nodes.len(), width)?;
        let mut rhs = vec![0.0_f64; ndof];
        for (index, value) in loads {
            rhs[index] = value;
        }
        match mode {
            "static_hex8" => {
                fields(material, &["youngs_pa", "poisson"])?;
                let e = num(required(material, "youngs_pa")?)?;
                let nu = num(required(material, "poisson")?)?;
                if !(1.0..=1e13).contains(&e) || !(-0.95..=0.495).contains(&nu) {
                    return Err(
                        "Elastic Young's modulus or Poisson ratio is outside its admitted range"
                            .into(),
                    );
                }
                let nodes = positions::<3>(nodes)?;
                let hex = connectivity::<8>(elements, nodes.len())?;
                let lambda = e * nu / ((1.0 + nu) * (1.0 - 2.0 * nu));
                let mu = e / (2.0 * (1.0 + nu));
                let mut matrix = vec![0.0; ndof * ndof];
                for row in &hex {
                    let xyz = row.map(|i| nodes[i]);
                    let dofs = std::array::from_fn::<_, 24, _>(|i| row[i / 3] * 3 + i % 3);
                    for point in gauss_hex_2x2x2() {
                        let (basis, det) = hex_basis(&xyz, point.coords)?;
                        let scale = det * point.weight;
                        for i in 0..24 {
                            let stress = isotropic_stress(basis[i], lambda, mu);
                            for j in 0..24 {
                                let entry = stress
                                    .iter()
                                    .zip(basis[j])
                                    .map(|(&a, b)| a * b)
                                    .sum::<f64>();
                                matrix[dofs[i] * ndof + dofs[j]] += scale * entry;
                            }
                        }
                    }
                }
                let (motion, reaction, residual) = solve_constraints(&matrix, &rhs, &constraints)?;
                let displacement: Vec<_> =
                    motion.chunks_exact(3).map(|v| [v[0], v[1], v[2]]).collect();
                let mut diagnostics = Vec::with_capacity(hex.len() * 12);
                for row in &hex {
                    let xyz = row.map(|i| nodes[i]);
                    let (basis, _) = hex_basis(&xyz, [0.0; 3])?;
                    let mut strain = [0.0; 6];
                    for i in 0..24 {
                        let value = motion[row[i / 3] * 3 + i % 3];
                        for (lane, weight) in strain.iter_mut().zip(basis[i]) {
                            *lane += weight * value;
                        }
                    }
                    diagnostics.extend(strain);
                    diagnostics.extend(isotropic_stress(strain, lambda, mu));
                }
                let points: Vec<_> = nodes
                    .iter()
                    .map(|p| Point3D::new(p[0], p[1], p[2]))
                    .collect();
                let file = export_vtu_hex(&points, &hex, Some(&displacement));
                Ok(Value::Record(Args::from([
                    (
                        "displacement_m".into(),
                        field(self, vec![nodes.len() as u32, 3], motion)?,
                    ),
                    (
                        "reaction_n".into(),
                        field(self, vec![nodes.len() as u32, 3], reaction)?,
                    ),
                    (
                        "centre_strain_and_stress_pa".into(),
                        field(self, vec![hex.len() as u32, 12], diagnostics)?,
                    ),
                    ("relative_residual".into(), Value::F64(residual)),
                    ("vtu".into(), self.encode("vtu", file.into_bytes())?),
                ])))
            }
            "steady_thermal_quad4" => {
                fields(material, &["conductivity_w_mk", "thickness_m"])?;
                let conductivity = num(required(material, "conductivity_w_mk")?)?;
                let thickness = num(required(material, "thickness_m")?)?;
                if !(1e-9..=1e8).contains(&conductivity) || !(1e-6..=1e3).contains(&thickness) {
                    return Err("Thermal conductivity or plate thickness is out of range".into());
                }
                let nodes = positions::<2>(nodes)?;
                let quad = connectivity::<4>(elements, nodes.len())?;
                let mut matrix = vec![0.0; ndof * ndof];
                for row in &quad {
                    let xy = row.map(|i| nodes[i]);
                    for coord in gauss_quad_2x2().map(|q| q.coords).into_iter().chain([
                        [-1.0, -1.0],
                        [1.0, -1.0],
                        [1.0, 1.0],
                        [-1.0, 1.0],
                    ]) {
                        let basis = shape_deriv_quad4(coord[0], coord[1]);
                        let j = jacobian(&basis, &xy);
                        let det = j[0][0] * j[1][1] - j[0][1] * j[1][0];
                        if !det.is_finite() || det <= 1e-12 {
                            return Err(
                                "Quad4 contains an inverted or degenerate thermal element".into()
                            );
                        }
                    }
                    let element = thermal_conduction_quad4(xy, conductivity, thickness);
                    for (i, &a) in row.iter().enumerate() {
                        for (j, &b) in row.iter().enumerate() {
                            matrix[a * ndof + b] += element[i][j];
                        }
                    }
                }
                let (temperature, thermal_reaction, residual) =
                    solve_constraints(&matrix, &rhs, &constraints)?;
                if temperature.iter().any(|&v| v < 0.0) {
                    return Err(
                        "Thermal boundary conditions produce a negative absolute temperature"
                            .into(),
                    );
                }
                Ok(Value::Record(Args::from([
                    (
                        "temperature_k".into(),
                        field(self, vec![ndof as u32], temperature)?,
                    ),
                    (
                        "thermal_reaction_w".into(),
                        field(self, vec![ndof as u32], thermal_reaction)?,
                    ),
                    ("relative_residual".into(), Value::F64(residual)),
                ])))
            }
            _ => unreachable!("finite element mode checked before assembly"),
        }
    }
}

fn positions<const D: usize>(values: &[Value]) -> Result<Vec<[f64; D]>, String> {
    let nodes = values
        .iter()
        .map(vector::<D>)
        .collect::<Result<Vec<_>, _>>()?;
    if nodes.iter().flatten().any(|v| v.abs() > 1e6) {
        return Err("Element positions exceed local metre range".into());
    }
    Ok(nodes)
}

fn connectivity<const N: usize>(values: &[Value], nodes: usize) -> Result<Vec<[usize; N]>, String> {
    values
        .iter()
        .map(|value| {
            let row = list(value)?;
            let mut result = [0; N];
            if row.len() != N {
                return Err("Element connectivity has the wrong local node count".into());
            }
            for (i, value) in row.iter().enumerate() {
                let node = integer(value, "element node")?;
                if node >= nodes || result[..i].contains(&node) {
                    return Err("Element references a repeated or absent node".into());
                }
                result[i] = node;
            }
            Ok(result)
        })
        .collect()
}

fn dof_values(input: &Value, nodes: usize, width: usize) -> Result<BTreeMap<usize, f64>, String> {
    let mut entries = BTreeMap::new();
    for value in list(input)? {
        let row = record(value)?;
        fields(row, &["node", "dof", "value"])?;
        let node = integer(required(row, "node")?, "node")?;
        let dof = integer(required(row, "dof")?, "dof")?;
        let scalar = num(required(row, "value")?)?;
        if node >= nodes
            || dof >= width
            || scalar.abs() > 1e16
            || entries.insert(node * width + dof, scalar).is_some()
        {
            return Err("Degree of freedom condition is invalid or repeated".into());
        }
    }
    Ok(entries)
}

fn solve_constraints(
    matrix: &[f64],
    loads: &[f64],
    prescribed: &BTreeMap<usize, f64>,
) -> Result<(Vec<f64>, Vec<f64>, f64), String> {
    let n = loads.len();
    let free: Vec<_> = (0..n)
        .filter(|index| !prescribed.contains_key(index))
        .collect();
    let mut solution = vec![0.0; n];
    for (&index, &value) in prescribed {
        solution[index] = value;
    }
    if !free.is_empty() {
        let mut reduced = Vec::with_capacity(free.len().pow(2));
        let mut reduced_loads = Vec::with_capacity(free.len());
        for &row in &free {
            let applied = prescribed
                .iter()
                .map(|(&col, &v)| matrix[row * n + col] * v)
                .sum::<f64>();
            reduced_loads.push(loads[row] - applied);
            reduced.extend(free.iter().map(|&col| matrix[row * n + col]));
        }
        let diagonal = reduced
            .iter()
            .step_by(free.len() + 1)
            .copied()
            .fold(0.0_f64, f64::max);
        if !diagonal.is_finite() || diagonal <= 0.0 {
            return Err("Finite element system is disconnected or has no stiffness".into());
        }
        let factor = CholeskyFactor::factorize(free.len(), &reduced)?;
        if factor
            .l_values
            .iter()
            .step_by(free.len() + 1)
            .any(|v| !v.is_finite() || v * v < diagonal * 1e-14)
        {
            return Err("Finite element system is singular or ill-conditioned".into());
        }
        for (&index, value) in free.iter().zip(factor.solve(&reduced_loads)) {
            solution[index] = value;
        }
    }
    let mut reactions = vec![0.0; n];
    let mut residual = 0.0_f64;
    for (index, (&load, row)) in loads.iter().zip(matrix.chunks_exact(n)).enumerate() {
        let internal = row.iter().zip(&solution).map(|(&k, &v)| k * v).sum::<f64>();
        let error = internal - load;
        if !error.is_finite() {
            return Err("Finite element balance has a nonfinite residual".into());
        }
        if prescribed.contains_key(&index) {
            reactions[index] = error;
        } else {
            let magnitude = row
                .iter()
                .zip(&solution)
                .map(|(&k, &v)| (k * v).abs())
                .sum::<f64>();
            residual = residual.max(error.abs() / (1.0 + magnitude + load.abs()));
        }
    }
    if residual > 1e-8 {
        return Err("Finite element solve exceeded its force or heat balance error bound".into());
    }
    Ok((solution, reactions, residual))
}

fn jacobian<const N: usize, const D: usize>(
    shape: &[[f64; D]; N],
    points: &[[f64; D]; N],
) -> [[f64; D]; D] {
    std::array::from_fn(|row| {
        std::array::from_fn(|col| {
            shape
                .iter()
                .zip(points)
                .map(|(d, p)| d[row] * (p[col] - points[0][col]))
                .sum()
        })
    })
}

fn hex_basis(xyz: &[[f64; 3]; 8], coordinate: [f64; 3]) -> Result<([[f64; 6]; 24], f64), String> {
    let shape = shape_deriv_hex8(coordinate[0], coordinate[1], coordinate[2]);
    let j = jacobian(&shape, xyz);
    let det = color_transforms::icc::math::det(&j);
    let largest = j.iter().flatten().map(|v| v.abs()).fold(0.0_f64, f64::max);
    if det <= largest.powi(3) * 1e-10 || !det.is_finite() {
        return Err("Hex8 contains an inverted or degenerate integration site".into());
    }
    let inverted =
        color_transforms::icc::math::invert(&j).ok_or("Cannot invert Hex8 element Jacobian")?;
    let mut b = [[0.0; 6]; 24];
    for (i, gradient) in shape.into_iter().enumerate() {
        let [x, y, z] = color_transforms::icc::math::apply(&inverted, gradient);
        b[3 * i] = [x, 0.0, 0.0, y, 0.0, z];
        b[3 * i + 1] = [0.0, y, 0.0, x, z, 0.0];
        b[3 * i + 2] = [0.0, 0.0, z, 0.0, y, x];
    }
    Ok((b, det))
}

fn isotropic_stress(strain: [f64; 6], lambda: f64, mu: f64) -> [f64; 6] {
    let hydrostatic = lambda * (strain[0] + strain[1] + strain[2]);
    std::array::from_fn(|i| {
        if i < 3 {
            2.0 * mu * strain[i] + hydrostatic
        } else {
            mu * strain[i]
        }
    })
}

fn field(host: &mut AssetHost<'_>, axes: Vec<u32>, samples: Vec<f64>) -> Result<Value, String> {
    if samples
        .iter()
        .any(|&v| !v.is_finite() || v.abs() > f64::from(f32::MAX))
    {
        return Err("Finite element field exceeds float32 output range".into());
    }
    host.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
        FieldLattice::DenseF32 {
            axes,
            samples: samples.into_iter().map(|v| v as f32).collect(),
        },
    )))
}
