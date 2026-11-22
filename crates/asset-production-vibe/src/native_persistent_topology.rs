//! Vietoris–Rips persistence over F₂ and cotangent Laplace–Beltrami modes.

use std::{collections::BTreeMap, sync::atomic::{AtomicBool, Ordering}};

use qualia_core_db::{
    solvers::linear_algebra::eigen::symmetric_eigen,
    specialized_libs::computational_geometry::{
        ddg_operators::{boundary_of_simplex, cotangent_laplacian, ChainTerm, SparseEntry},
        primitives::Point3,
        vr_filtration::{vr_filtration, VrSimplex},
    },
    tensor::Tensor10D,
};
use sha2::{Digest, Sha256};

use crate::{
    artifact::FieldLattice, fields, integer, list, num, required, text, Args, AssetHost,
    TypedArtifact, Value, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
};

const MAX_POINTS: usize = 16;
const MAX_FIELD_SAMPLES: usize = 65_536;
const MAX_SPECTRAL_VERTICES: usize = 96;
const MAX_SPECTRAL_TRIANGLES: usize = 512;
const MAX_OUTPUT_MODES: usize = 32;
const BARCODE_CONTEXT: &str = "urn:qualia:topology:vietoris-rips-f2-v1";

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Simplex {
    birth_bits: u64,
    dim: u8,
    vertices: [u32; 4],
}

impl AssetHost<'_> {
    pub(crate) fn compute_persistent_homology_ddg(&mut self, args: &Args) -> Result<Value, String> {
        match text(required(args, "task")?)? {
            "VietorisRipsPersistence_H0_H2" => self.vr_barcode(args),
            "LaplaceBeltramiEigenmodes" => self.laplace_beltrami_modes(args),
            "Hodge1FormDecomp" | "VectorHeatTransport" => {
                Err("The selected topology task has no production adapter".into())
            }
            _ => Err("Unknown topology task".into()),
        }
    }

    fn vr_barcode(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "task", "max_dimension", "max_radius", "lattice_sampling"])?;
        let dimension = args.get("max_dimension").map(|v| integer(v, "max_dimension"))
            .transpose()?.unwrap_or(2);
        if dimension > 2 {
            return Err("Vietoris–Rips max_dimension must be in 0..=2".into());
        }
        let cutoff = args.get("max_radius").map(num).transpose()?;
        if cutoff.is_some_and(|v| v < 0.0) {
            return Err("Vietoris–Rips max_radius must be nonnegative".into());
        }
        let cutoff = cutoff.map(|v| v + 0.0);
        let source = required(args, "source")?;
        let points = match self.workspace.typed(source)? {
            TypedArtifact::FieldLattice(lattice) => {
                let FieldLattice::DenseF32 { samples, .. } = lattice.as_ref() else {
                    return Err("Vietoris–Rips requires a scalar DenseF32 lattice with explicit spatial sampling".into());
                };
                let samples_len = samples.len();
                if samples_len > MAX_FIELD_SAMPLES {
                    return Err("Vietoris–Rips lattice exceeds the 65,536-sample limit".into());
                }
                let lattice = lattice.clone();
                self.charge(samples_len.saturating_mul(3), 4096)?;
                let FieldLattice::DenseF32 { axes, samples } = lattice.as_ref() else {
                    return Err("Expected a scalar DenseF32 lattice".into());
                };
                sample_lattice(axes, samples, required(args, "lattice_sampling")?, self.cancel)?
            }
            TypedArtifact::AttributedMesh(mesh) => {
                if args.contains_key("lattice_sampling") {
                    return Err("lattice_sampling requires a DenseF32 lattice".into());
                }
                bounded_points(&mesh.surface.mesh.positions)?
            }
            TypedArtifact::ParticleSet(particles) => {
                if args.contains_key("lattice_sampling") {
                    return Err("lattice_sampling requires a DenseF32 lattice".into());
                }
                bounded_points(&particles.positions)?
            }
            _ => return Err("Vietoris–Rips requires an AttributedMesh, ParticleSet, or DenseF32 lattice".into()),
        };
        if points.is_empty() {
            return Err("Vietoris–Rips requires at least one point".into());
        }
        let n = points.len();
        let edges = n * n.saturating_sub(1) / 2;
        let triangles = n * n.saturating_sub(1) * n.saturating_sub(2) / 6;
        let tetrahedra = triangles * n.saturating_sub(3) / 4;
        let native_count = n + edges + if dimension >= 1 { triangles } else { 0 };
        let max_columns = native_count + if dimension == 2 { tetrahedra } else { 0 };
        let words = max_columns.div_ceil(64);
        let reduction_work = max_columns * max_columns * words;
        let scratch = max_columns * words * 8 + max_columns * 256 + 1024 * 1024;
        self.charge(reduction_work, scratch)?;

        let simplices = filtration(&points, dimension, cutoff, native_count)?;
        let pairs = reduce_boundaries(&simplices, self.cancel)?;
        let mut identity = Sha256::new();
        identity.update(BARCODE_CONTEXT.as_bytes());
        identity.update([dimension as u8, u8::from(cutoff.is_some()), n as u8]);
        identity.update(cutoff.unwrap_or(0.0).to_bits().to_le_bytes());
        for point in &points {
            for value in point {
                identity.update(value.to_bits().to_le_bytes());
            }
        }
        let iri = format!("urn:qualia:topology:barcode:{:x}", identity.finalize());
        let mut graph = ReceiptGraph::new(BARCODE_CONTEXT);
        graph.fact(&iri, "urn:qualia:topology:coefficientField", "urn:qualia:field:F2")?;
        graph.fact(&iri, "urn:qualia:topology:pointCount", &format!("urn:qualia:integer:{n}"))?;
        graph.fact(&iri, "urn:qualia:topology:maxDimension", &format!("urn:qualia:integer:{dimension}"))?;
        graph.fact(&iri, "urn:qualia:topology:maxRadius", &cutoff.map_or_else(
            || "urn:qualia:topology:unbounded".into(), |r| radius_iri(r.to_bits())))?;

        let mut intervals = Vec::new();
        for (i, death) in pairs.into_iter().enumerate() {
            let Some(death) = death else { continue };
            let birth = simplices[i];
            if birth.dim as usize > dimension {
                continue;
            }
            let death = death.map(|j| simplices[j]);
            if death.is_some_and(|end| end.birth_bits == birth.birth_bits) {
                continue;
            }
            let end_status = match (death, cutoff) {
                (Some(_), _) => "finite",
                (None, Some(_)) => "right-censored",
                (None, None) => "essential",
            };
            let interval_iri = format!("{iri}:interval:{i}");
            graph.fact(&iri, "urn:qualia:topology:interval", &interval_iri)?;
            graph.fact(&interval_iri, "urn:qualia:topology:dimension", &format!("urn:qualia:integer:{}", birth.dim))?;
            graph.fact(&interval_iri, "urn:qualia:topology:birthRadius", &radius_iri(birth.birth_bits))?;
            graph.fact(&interval_iri, "urn:qualia:topology:endStatus", &format!("urn:qualia:topology:{end_status}"))?;
            if let Some(end) = death {
                graph.fact(&interval_iri, "urn:qualia:topology:deathRadius", &radius_iri(end.birth_bits))?;
            }
            intervals.push(Value::Record(Args::from([
                ("iri".into(), Value::String(interval_iri)),
                ("dimension".into(), Value::U64(birth.dim.into())),
                ("birth_radius".into(), Value::F64(f64::from_bits(birth.birth_bits))),
                ("death_radius".into(), death.map_or(Value::Null, |s| Value::F64(f64::from_bits(s.birth_bits)))),
                ("end_status".into(), Value::String(end_status.into())),
                ("birth_sample_indices".into(), simplex_vertices(birth)),
                ("death_sample_indices".into(), death.map_or(Value::Null, simplex_vertices)),
            ])));
        }
        let barcode = graph.seal(self.limits.resource_bytes.min(2 * 1024 * 1024))?;
        self.charge(0, 0)?;
        let barcode = self.encode("q42", barcode)?;
        Ok(Value::Record(Args::from([
            ("barcode".into(), barcode), ("barcode_iri".into(), Value::String(iri)),
            ("intervals".into(), Value::List(intervals)),
            ("point_count".into(), Value::U64(n as u64)),
            ("simplex_count".into(), Value::U64(simplices.len() as u64)),
            ("max_radius".into(), cutoff.map_or(Value::Null, Value::F64)),
        ])))
    }

    fn laplace_beltrami_modes(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "task", "mode_count"])?;
        let source = required(args, "source")?;
        let TypedArtifact::AttributedMesh(mesh) = self.workspace.typed(source)? else {
            return Err("Laplace–Beltrami eigenmodes require an AttributedMesh".into());
        };
        let n = mesh.surface.mesh.positions.len();
        let face_count = mesh.surface.mesh.triangles.len();
        if !(3..=MAX_SPECTRAL_VERTICES).contains(&n)
            || !(1..=MAX_SPECTRAL_TRIANGLES).contains(&face_count)
        {
            return Err("Laplace–Beltrami requires 3–96 vertices and 1–512 triangles".into());
        }
        let modes = args.get("mode_count").map(|v| integer(v, "mode_count"))
            .transpose()?.unwrap_or(8.min(n));
        if !(1..=MAX_OUTPUT_MODES.min(n)).contains(&modes) {
            return Err("mode_count exceeds the 32-mode or mesh vertex limit".into());
        }
        let mesh = mesh.clone();
        self.charge(600 * n * n * n, 72 * n * n + 160 * face_count)?;
        let modes = spectral_modes(&mesh.surface.mesh.positions, &mesh.surface.mesh.triangles, modes)?;
        self.charge(0, 0)?;
        Ok(Value::Record(Args::from([
            ("mesh".into(), source.clone()),
            ("mode_basis".into(), Value::String("cotangent-lumped-mass-neumann".into())),
            ("vertex_modes".into(), Value::List(modes)),
        ])))
    }
}

fn radius_iri(bits: u64) -> String {
    format!("urn:qualia:topology:f64-bits:{bits:016x}")
}

fn simplex_vertices(simplex: Simplex) -> Value {
    Value::List(simplex.vertices[..=simplex.dim as usize].iter()
        .map(|&v| Value::U64(v.into())).collect())
}

fn bounded_points(points: &[[f32; 3]]) -> Result<Vec<[f32; 3]>, String> {
    if points.len() > MAX_POINTS {
        return Err("Vietoris–Rips admits at most 16 points".into());
    }
    Ok(points.to_vec())
}

fn triple(value: &Value) -> Result<[f64; 3], String> {
    let [x, y, z] = list(value)? else {
        return Err("Lattice spatial vectors must contain three coordinates".into());
    };
    Ok([num(x)?, num(y)?, num(z)?])
}

fn sample_lattice(axes: &[u32], samples: &[f32], sampling: &Value, cancel: &AtomicBool)
    -> Result<Vec<[f32; 3]>, String>
{
    let &[x, y, z] = axes else { return Err("Expected a three-dimensional scalar lattice".into()) };
    let nx = x as usize;
    let ny = y as usize;
    let nz = z as usize;
    if nx == 0 || ny == 0 || nz == 0 || nx.checked_mul(ny).and_then(|n| n.checked_mul(nz)) != Some(samples.len()) {
        return Err("DenseF32 lattice dimensions disagree with sample storage".into());
    }
    let sampling = crate::record(sampling)?;
    fields(sampling, &["origin", "spacing", "minimum_value"])?;
    let origin = triple(required(sampling, "origin")?)?;
    let spacing = triple(required(sampling, "spacing")?)?;
    if spacing.iter().any(|&v| v <= 0.0) {
        return Err("Lattice spacing must be positive on all three axes".into());
    }
    let threshold = num(required(sampling, "minimum_value")?)?;
    let mut points = Vec::with_capacity(MAX_POINTS);
    for (i, value) in samples.iter().enumerate() {
        if i % 4096 == 0 && cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        if !value.is_finite() {
            return Err("DenseF32 lattice contains a nonfinite sample".into());
        }
        if f64::from(*value) >= threshold {
            if points.len() == MAX_POINTS {
                return Err("Vietoris–Rips lattice selection exceeds 16 points".into());
            }
            let index = [i % nx, i / nx % ny, i / (nx * ny)];
            points.push(std::array::from_fn(|axis| (origin[axis] + spacing[axis] * index[axis] as f64) as f32));
        }
    }
    Ok(points)
}

fn filtration(points: &[[f32; 3]], dimension: usize, cutoff: Option<f64>, native_count: usize)
    -> Result<Vec<Simplex>, String>
{
    let mut tensors = Vec::with_capacity(points.len());
    for (i, &[x, y, z]) in points.iter().enumerate() {
        if [x, y, z].iter().any(|v| !v.is_finite() || v.abs() > f32::MAX / 2.0) {
            return Err(format!("Vietoris–Rips point {i} exceeds the finite f32-delta coordinate domain"));
        }
        tensors.push(Tensor10D { x, y, z, ..Default::default() });
    }
    let n = tensors.len();
    let mut native = vec![VrSimplex::default(); native_count];
    let count = vr_filtration(&tensors, (dimension + 1).min(2) as u8, cutoff.unwrap_or(0.0), &mut native)
        .map_err(|e| e.to_string())?;
    let mut edge = [[None; MAX_POINTS]; MAX_POINTS];
    let mut simplices = Vec::with_capacity(count);
    for s in native[..count].iter().filter(|s| cutoff.is_none_or(|r| s.birth_f64() <= r)) {
        if s.dim == 1 {
            edge[s.v0 as usize][s.v1 as usize] = Some(s.birth);
        }
        simplices.push(Simplex { birth_bits: s.birth, dim: s.dim, vertices: [s.v0, s.v1, s.v2, 0] });
    }
    if dimension == 2 {
        // A tetrahedron enters at the largest birth of its six admitted edges.
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    for m in k + 1..n {
                        let [Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)] = [
                            edge[i][j], edge[i][k], edge[i][m], edge[j][k], edge[j][m], edge[k][m],
                        ] else { continue };
                        simplices.push(Simplex {
                            birth_bits: a.max(b).max(c).max(d).max(e).max(f),
                            dim: 3, vertices: [i as u32, j as u32, k as u32, m as u32],
                        });
                    }
                }
            }
        }
    }
    simplices.sort_unstable();
    Ok(simplices)
}

// Each positive column has an optional death index. Negative columns have no birth.
fn reduce_boundaries(simplices: &[Simplex], cancel: &AtomicBool) -> Result<Vec<Option<Option<usize>>>, String> {
    let words = simplices.len().div_ceil(64);
    let mut columns = vec![0u64; simplices.len() * words];
    let mut active = vec![0u64; words];
    let mut births = vec![None; simplices.len()];
    let indices: BTreeMap<_, _> = simplices.iter().enumerate()
        .map(|(i, s)| ((s.dim, s.vertices), i)).collect();
    let empty = ChainTerm { dim: 0, vertex_count: 0, sign: 0, vertices: [0; 4] };
    let mut boundary = [empty; 4];
    for (i, s) in simplices.iter().enumerate() {
        if cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        if s.dim == 0 {
            births[i] = Some(None);
            continue;
        }
        active.fill(0);
        let simplex = ChainTerm { dim: s.dim, vertex_count: s.dim + 1, sign: 1, vertices: s.vertices };
        let count = boundary_of_simplex(simplex, &mut boundary).map_err(|e| format!("DDG boundary: {e:?}"))?;
        for face in &boundary[..count] {
            let row = *indices.get(&(face.dim, face.vertices)).ok_or("Vietoris–Rips boundary face is absent")?;
            if row >= i {
                return Err("Vietoris–Rips boundary face occurs after its coface".into());
            }
            active[row / 64] ^= 1 << (row % 64);
        }
        while let Some(word) = active.iter().rposition(|&w| w != 0) {
            let pivot = 64 * word + 63 - active[word].leading_zeros() as usize;
            let death = births[pivot].as_mut().ok_or("Vietoris–Rips boundary pivot did not create a class")?;
            let stored = &mut columns[pivot * words..(pivot + 1) * words];
            if death.is_none() {
                *death = Some(i);
                stored[..=word].copy_from_slice(&active[..=word]);
                break;
            }
            for (value, &previous) in active[..=word].iter_mut().zip(&stored[..=word]) {
                *value ^= previous;
            }
        }
        if active.iter().all(|&w| w == 0) {
            births[i] = Some(None);
        }
    }
    Ok(births)
}

fn spectral_modes(vertices: &[[f32; 3]], triangles: &[[u32; 3]], count: usize) -> Result<Vec<Value>, String> {
    let n = vertices.len();
    if vertices.iter().flatten().any(|v| !v.is_finite()) {
        return Err("Laplace–Beltrami mesh positions must be finite".into());
    }
    let vertices: Vec<Point3> = vertices.iter()
        .map(|v| Point3::new(f64::from(v[0]), f64::from(v[1]), f64::from(v[2]))).collect();
    let zero = SparseEntry { row: 0, col: 0, value: 0.0 };
    let mut stiffness = vec![zero; triangles.len() * 9];
    let mut mass = vec![0.0; n];
    let nnz = cotangent_laplacian(&vertices, triangles, &mut stiffness, &mut mass)
        .map_err(|e| format!("Laplace–Beltrami assembly: {e:?}"))?;
    if mass.iter().any(|m| !m.is_finite() || *m <= 0.0) {
        return Err("Laplace–Beltrami requires positive, finite incident area at every vertex".into());
    }
    let inv_sqrt_mass: Vec<f64> = mass.iter().map(|m| m.sqrt().recip()).collect();
    let mut normalized = vec![0.0; n * n];
    for entry in &stiffness[..nnz] {
        let (r, c) = (entry.row as usize, entry.col as usize);
        normalized[r * n + c] = entry.value * (inv_sqrt_mass[r] * inv_sqrt_mass[c]);
    }
    if normalized.iter().any(|v| !v.is_finite()) {
        return Err("Laplace–Beltrami normalization returned a nonfinite coefficient".into());
    }
    let scale = normalized.iter().map(|v| v.abs()).fold(0.0, f64::max);
    if scale == 0.0 {
        return Err("Laplace–Beltrami stiffness matrix is zero".into());
    }
    for v in &mut normalized {
        *v /= scale;
    }
    let mut rotated = normalized.clone();
    let mut eigenvectors = vec![0.0; n * n];
    symmetric_eigen(n, &mut rotated, &mut eigenvectors).map_err(|e| format!("Laplace–Beltrami eigensolver: {e:?}"))?;
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| rotated[a * n + a].total_cmp(&rotated[b * n + b]).then(a.cmp(&b)));
    let mut modes = Vec::with_capacity(count);
    for &column in &order[..count] {
        let scaled_eigenvalue = rotated[column * n + column];
        if !scaled_eigenvalue.is_finite() || scaled_eigenvalue < -1e-10 {
            return Err("Laplace–Beltrami eigensolver returned an invalid eigenvalue".into());
        }
        let scaled_eigenvalue = scaled_eigenvalue.max(0.0);
        let mut vector: Vec<f64> = (0..n).map(|row| eigenvectors[row * n + column]).collect();
        let sign_anchor = vector.iter().copied().fold(0.0_f64, |anchor, value|
            if value.abs() > anchor.abs() { value } else { anchor });
        if sign_anchor < 0.0 {
            vector.iter_mut().for_each(|v| *v = -*v);
        }
        let residual_sq: f64 = normalized.chunks_exact(n).zip(&vector).map(|(row, &component)| {
            let applied: f64 = row.iter().zip(&vector).map(|(&coefficient, &value)| coefficient * value).sum();
            (applied - scaled_eigenvalue * component).powi(2)
        }).sum();
        let residual = residual_sq.sqrt();
        let norm: f64 = vector.iter().map(|v| v * v).sum();
        if !residual.is_finite() || residual > 1e-8 || !norm.is_finite() || (norm - 1.0).abs() > 1e-8 {
            return Err(format!("Laplace–Beltrami eigenpair failed residual/normalization validation: {residual}, {norm}"));
        }
        let eigenvalue = scaled_eigenvalue * scale;
        if !eigenvalue.is_finite() {
            return Err("Laplace–Beltrami eigenvalue overflowed world scale".into());
        }
        let vertex_values = vector.into_iter().zip(&inv_sqrt_mass)
            .map(|(v, &inv_mass)| Value::F64(v * inv_mass)).collect();
        modes.push(Value::Record(Args::from([
            ("eigenvalue".into(), Value::F64(eigenvalue)),
            ("scaled_operator_residual".into(), Value::F64(residual)),
            ("vertex_values".into(), Value::List(vertex_values)),
        ])));
    }
    Ok(modes)
}
