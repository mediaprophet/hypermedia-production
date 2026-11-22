//! Bounded continuum simulations over admitted, explicitly shaped fields.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    Args, AssetHost, TypedArtifact, Value,
    artifact::{AttributedMesh, FieldLattice},
    integer, list, num,
    qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    required,
};
use qualia_core_db::render::assets::{ImportedMesh, Mesh};
use std::sync::Arc;

mod bead_probe;
mod electronic;
mod fold;
mod lbm;
mod mpm;
mod particles;
mod smoke;
mod sph;
mod synthetic_membrane;
mod tribology;
mod variational;

fn int_arg(
    args: &Args,
    key: &str,
    default: Option<usize>,
    bounds: std::ops::RangeInclusive<usize>,
) -> Result<usize, String> {
    let value = match args.get(key) {
        Some(value) => integer(value, key)?,
        None => default.ok_or_else(|| format!("Missing {key}"))?,
    };
    if !bounds.contains(&value) {
        return Err(format!("{key} is outside {bounds:?}"));
    }
    Ok(value)
}

fn number_arg(
    args: &Args,
    key: &str,
    default: Option<f64>,
    bounds: std::ops::RangeInclusive<f64>,
) -> Result<f64, String> {
    let value = match args.get(key) {
        Some(value) => num(value)?,
        None => default.ok_or_else(|| format!("Missing {key}"))?,
    };
    if !bounds.contains(&value) {
        return Err(format!("{key} is outside {bounds:?}"));
    }
    Ok(value)
}

fn input_plane(
    host: &AssetHost<'_>,
    value: &Value,
    shape: &[u32],
    bounds: std::ops::RangeInclusive<f64>,
    key: &str,
) -> Result<Vec<f64>, String> {
    let len: usize = shape.iter().map(|axis| *axis as usize).product();
    let samples = match host.workspace.typed(value) {
        Ok(TypedArtifact::FieldLattice(field)) => match field.as_ref() {
            FieldLattice::DenseF32 { axes, samples }
                if axes.as_slice() == shape && samples.len() == len =>
            {
                samples.iter().copied().map(f64::from).collect::<Vec<_>>()
            }
            _ => return Err(format!("{key} requires DenseF32 with axes {shape:?}")),
        },
        Ok(_) => return Err(format!("{key} requires a field, scalar, or flattened list")),
        Err(_) => {
            if let Ok(values) = list(value) {
                if values.len() != len {
                    return Err(format!("{key} requires {len} flattened values"));
                }
                values.iter().map(num).collect::<Result<Vec<_>, _>>()?
            } else {
                vec![num(value)?; len]
            }
        }
    };
    if samples.iter().any(|sample| !bounds.contains(sample)) {
        return Err(format!("{key} contains a sample outside {bounds:?}"));
    }
    Ok(samples)
}

fn output_plane(
    host: &mut AssetHost<'_>,
    mut axes: Vec<u32>,
    planes: &[&[f64]],
) -> Result<Value, String> {
    let len: usize = axes.iter().map(|axis| *axis as usize).product();
    let mut samples = Vec::with_capacity(planes.len() * len);
    for plane in planes {
        if plane.len() != len {
            return Err("Continuum output plane dimensions do not agree".into());
        }
        for &sample in *plane {
            let converted = sample as f32;
            if !converted.is_finite() {
                return Err("Continuum sample is outside finite f32 range".into());
            }
            samples.push(converted);
        }
    }
    if planes.len() != 1 {
        axes.insert(0, planes.len() as u32);
    }
    host.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
        FieldLattice::DenseF32 { axes, samples },
    )))
}

fn read_dimensions(args: &Args, max_axis: usize, cell_limit: usize) -> Result<[usize; 3], String> {
    let source = list(required(args, "dimensions")?)?;
    let [x, y, z] = source else {
        return Err("dimensions requires three grid axes in x, y, z order".into());
    };
    let dimensions = [integer(x, "x")?, integer(y, "y")?, integer(z, "z")?];
    if dimensions.iter().any(|axis| !(3..=max_axis).contains(axis))
        || dimensions.iter().product::<usize>() > cell_limit
    {
        return Err(format!(
            "Grid axes must be 3..={max_axis} with at most {cell_limit} cells"
        ));
    }
    Ok(dimensions)
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(x, y)| x * y).sum()
}

fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|k| a[k] + b[k])
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|k| a[k] - b[k])
}

fn mesh_artifact(
    host: &mut AssetHost<'_>,
    positions: Vec<[f32; 3]>,
    triangles: Vec<[u32; 3]>,
) -> Result<Value, String> {
    if positions.is_empty()
        || triangles.is_empty()
        || positions.iter().flatten().any(|p| !p.is_finite())
    {
        return Err("Continuum mesh is empty or its positions are nonfinite".into());
    }
    let mut min = positions[0];
    let mut max = positions[0];
    for p in &positions {
        for k in 0..3 {
            min[k] = min[k].min(p[k]);
            max[k] = max[k].max(p[k]);
        }
    }
    if triangles.iter().any(|triangle| {
        let [a, b, c] = triangle.map(|slot| positions.get(slot as usize));
        match (a, b, c) {
            (Some(a), Some(b), Some(c)) => {
                let u = sub3(b.map(f64::from), a.map(f64::from));
                let v = sub3(c.map(f64::from), a.map(f64::from));
                let cross = [
                    u[1] * v[2] - u[2] * v[1],
                    u[2] * v[0] - u[0] * v[2],
                    u[0] * v[1] - u[1] * v[0],
                ];
                dot(cross, cross) == 0.0
            }
            _ => true,
        }
    }) {
        return Err("Continuum mesh loses triangle area at output precision".into());
    }
    host.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(AttributedMesh {
        surface: ImportedMesh {
            mesh: Mesh {
                positions,
                triangles,
                min,
                max,
            },
            normals: None,
            tangents: None,
            texture_coordinates_0: None,
            materials: Vec::new(),
            texture_dependencies: Vec::new(),
            submeshes: Vec::new(),
        },
        uv1: Vec::new(),
        vertex_colors: Vec::new(),
        skin_weights: Vec::new(),
        morph_targets: Arc::from([]),
        protected_faces: Vec::new(),
    })))
}

fn seal_diagnostics(
    host: &mut AssetHost<'_>,
    kind: &str,
    parameters_hash: &str,
    metrics: &[(&str, f64)],
) -> Result<Value, String> {
    let context = format!("urn:qualia:continuum:{kind}:{parameters_hash}");
    let mut graph = ReceiptGraph::new(&context);
    graph.fact(
        &context,
        "urn:qualia:continuum:diagnostics",
        &format!("urn:qualia:continuum:{kind}"),
    )?;
    for &(name, value) in metrics {
        if !value.is_finite() {
            return Err(format!("Nonfinite diagnostic {name}"));
        }
        graph.fact(
            &context,
            &format!("urn:qualia:continuum:diagnostic:{name}"),
            &format!("urn:qualia:binary64:{:016x}", value.to_bits()),
        )?;
    }
    let bytes = graph.seal(host.limits.resource_bytes.min(1_048_576))?;
    host.admit_encoded("q42", bytes)
}
