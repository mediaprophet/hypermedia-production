//! Seeded segmentation of synthetic 3D scalar lattices with semantic SDFB output.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeMap;

use qualia_core_db::specialized_libs::computational_geometry::Point3;
use serde_json::json;

use super::{encode_json, result};
use crate::{
    Args, AssetHost, TypedArtifact, Value, artifact::FieldLattice, boolean, count, fields, integer,
    list, num, record, required, text, vector,
};

const MAX_AXIS: u32 = 64;
const MAX_SURFACE_FACES: usize = 16_384;

impl AssetHost<'_> {
    pub(crate) fn segment_volume_to_10d(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "volume",
                "synthetic_phantom",
                "method",
                "seed",
                "threshold",
                "spacing_m",
                "origin_m",
                "resolution",
                "entity_iri",
                "wall_iri",
                "lumen_iri",
            ],
        )?;
        if !boolean(args, "synthetic_phantom", false)?
            || text(required(args, "method")?)? != "seeded_threshold_voxels"
        {
            return Err("Volume segmentation requires an authored synthetic scalar lattice and seeded_threshold_voxels".into());
        }
        let input = required(args, "volume")?;
        let (dimensions, values) = lattice(self, input)?;
        let voxel_count = values.len();
        let threshold = num(required(args, "threshold")?)?;
        let spacing: [f64; 3] = vector(required(args, "spacing_m")?)?;
        let origin: [f64; 3] = args
            .get("origin_m")
            .map(vector)
            .transpose()?
            .unwrap_or([0.0; 3]);
        if spacing.iter().any(|value| !(0.0001..=1.0).contains(value))
            || origin.iter().any(|value| value.abs() > 100.0)
        {
            return Err(
                "Synthetic grid spacing or origin exceeds the admitted metric range".into(),
            );
        }
        let seed = seed(required(args, "seed")?, dimensions)?;
        let [nx, ny, nz] = dimensions;
        let resolution = count(args, "resolution", nx.max(ny).max(nz).max(16), 128)?;
        let pad = spacing.iter().copied().fold(f64::INFINITY, f64::min);
        let entity_iri = required(args, "entity_iri")?;
        let wall_iri = required(args, "wall_iri")?;
        let lumen_iri = required(args, "lumen_iri")?;
        let keys = [entity_iri, wall_iri, lumen_iri].map(semantic_key);
        let [entity, wall, lumen] = keys;
        let [entity, wall, lumen] = [entity?, wall?, lumen?];
        if [entity, wall, lumen].contains(&0) || entity == wall || entity == lumen || wall == lumen
        {
            return Err(
                "Expected three distinct canonical IRI identifiers for the synthetic asset".into(),
            );
        }
        let max_faces = MAX_SURFACE_FACES.min(voxel_count * 6);
        self.charge(voxel_count * 32, voxel_count * 15 + max_faces * 336)?;
        let (_, values) = lattice(self, input)?;
        let (filled, selected) = select(values, dimensions, seed, threshold)?;
        let surface = Surface::from_voxels(dimensions, &filled, &selected, origin, spacing)?;
        let triangles = surface.triangles.len();
        let vertices = surface.vertices.len();
        let edges = surface.closed_edges()?;
        self.mesh_budget(vertices, triangles)?;
        let mesh = self.store_mesh(surface.vertices, surface.triangles)?;
        let outputs = self.manifold_fields_sdf_op(
            "hypermedia.manifold.seal_sdf_brick_10d",
            &Args::from([
                ("mesh".into(), mesh),
                ("entity_iri".into(), entity_iri.clone()),
                ("wall_iri".into(), wall_iri.clone()),
                ("lumen_iri".into(), lumen_iri.clone()),
                ("resolution".into(), Value::U64(resolution as u64)),
                ("padding".into(), Value::F64(pad)),
            ]),
        )?;
        let outputs = record(&outputs)?;
        let report = encode_json(
            self,
            &json!({
                "schema":"qualia-seeded-synthetic-volume/1", "segmentation":"seeded_threshold_voxels",
                "dimensions_xyz":dimensions, "spacing_m":spacing, "origin_corner_m":origin,
                "seed_xyz":seed, "threshold":threshold, "connectivity":6,
                "selected_cells":selected.len(), "selected_volume_m3":selected.len() as f64 * spacing.iter().product::<f64>(),
                "surface_vertices":vertices, "surface_edges":edges, "surface_triangles":triangles,
                "surface_euler_characteristic":vertices as i64 - edges as i64 + triangles as i64,
            }),
        )?;
        Ok(result(&[
            ("asset", required(outputs, "asset")?.clone()),
            ("field", required(outputs, "field")?.clone()),
            ("report", report),
        ]))
    }
}

fn semantic_key(value: &Value) -> Result<u64, String> {
    let iri = text(value)?;
    let (scheme, tail) = iri.split_once(':').ok_or("Expected an absolute IRI")?;
    let mut scheme = scheme.bytes();
    if iri.len() > 1024
        || !scheme.next().is_some_and(|b| b.is_ascii_alphabetic())
        || !scheme.all(|b| b.is_ascii_alphanumeric() || b"+-.".contains(&b))
        || tail.is_empty()
        || tail
            .chars()
            .any(|ch| ch.is_whitespace() || ch.is_control() || "<>\"{}|\\^`".contains(ch))
    {
        return Err("Invalid synthetic segmentation IRI".into());
    }
    Ok(qualia_core_db::q_hash(iri))
}

fn lattice<'a>(host: &'a AssetHost<'_>, input: &Value) -> Result<([usize; 3], &'a [f32]), String> {
    let TypedArtifact::FieldLattice(lattice) = host.workspace.typed(input)? else {
        return Err("Expected a typed synthetic FieldLattice".into());
    };
    let FieldLattice::DenseF32 { axes, samples } = lattice.as_ref() else {
        return Err("Synthetic volume requires a dense F32 lattice".into());
    };
    let &[nx, ny, nz] = axes.as_slice() else {
        return Err("Synthetic volume requires three axes".into());
    };
    let axes = [nx, ny, nz];
    if axes.iter().any(|n| !(3..=MAX_AXIS).contains(n)) {
        return Err("Synthetic volume dimensions exceed the admitted range".into());
    }
    let dimensions = axes.map(|axis| axis as usize);
    if samples.len() != dimensions.iter().product::<usize>() {
        return Err("Synthetic scalar sample count differs from the lattice extent".into());
    }
    Ok((dimensions, samples))
}

fn seed(input: &Value, dimensions: [usize; 3]) -> Result<[usize; 3], String> {
    let [x, y, z] = list(input)? else {
        return Err("Seed requires three integer voxel coordinates".into());
    };
    let result = [
        integer(x, "seed x")?,
        integer(y, "seed y")?,
        integer(z, "seed z")?,
    ];
    if (0..3).any(|axis| result[axis] >= dimensions[axis]) {
        return Err("Seed lies outside the scalar lattice".into());
    }
    Ok(result)
}

fn index([x, y, z]: [usize; 3], [nx, ny, _]: [usize; 3]) -> usize {
    x + y * nx + z * nx * ny
}

fn point(index: usize, [nx, ny, _]: [usize; 3]) -> [usize; 3] {
    [index % nx, index / nx % ny, index / (nx * ny)]
}

fn adjacent(
    mut point: [usize; 3],
    axis: usize,
    positive: bool,
    dimensions: [usize; 3],
) -> Option<[usize; 3]> {
    point[axis] = point[axis].checked_add_signed(if positive { 1 } else { -1 })?;
    (point[axis] < dimensions[axis]).then_some(point)
}

fn select(
    values: &[f32],
    dimensions: [usize; 3],
    seed: [usize; 3],
    threshold: f64,
) -> Result<(Vec<bool>, Vec<usize>), String> {
    let first = index(seed, dimensions);
    if values.iter().any(|sample| !sample.is_finite()) || f64::from(values[first]) < threshold {
        return Err("Threshold seed requires finite values and must be in the foreground".into());
    }
    let mut filled = vec![false; values.len()];
    let mut selected = vec![first];
    filled[first] = true;
    let mut cursor = 0;
    while let Some(&cell) = selected.get(cursor) {
        cursor += 1;
        let point = point(cell, dimensions);
        if (0..3).any(|axis| point[axis] == 0 || point[axis] + 1 == dimensions[axis]) {
            return Err("Seeded component touches the synthetic lattice boundary".into());
        }
        for axis in 0..3 {
            for positive in [false, true] {
                if let Some(next) = adjacent(point, axis, positive, dimensions) {
                    let next = index(next, dimensions);
                    if !filled[next] && f64::from(values[next]) >= threshold {
                        filled[next] = true;
                        selected.push(next);
                    }
                }
            }
        }
    }
    Ok((filled, selected))
}

struct Surface {
    vertices: Vec<Point3>,
    triangles: Vec<[u32; 3]>,
}

impl Surface {
    fn from_voxels(
        dimensions: [usize; 3],
        filled: &[bool],
        selected: &[usize],
        origin: [f64; 3],
        spacing: [f64; 3],
    ) -> Result<Self, String> {
        let corner_dimensions = dimensions.map(|dim| dim + 1);
        let mut corner_ids = vec![u32::MAX; corner_dimensions.iter().product()];
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        for &cell in selected {
            let point = point(cell, dimensions);
            for axis in 0..3 {
                let u = (axis + 1) % 3;
                let v = (axis + 2) % 3;
                for positive in [false, true] {
                    if adjacent(point, axis, positive, dimensions)
                        .is_some_and(|next| filled[index(next, dimensions)])
                    {
                        continue;
                    }
                    if triangles.len() == MAX_SURFACE_FACES * 2 {
                        return Err("Synthetic voxel surface exceeds its face budget".into());
                    }
                    let mut corner = point;
                    corner[axis] += positive as usize;
                    let ids = [(0, 0), (1, 0), (1, 1), (0, 1)].map(|(du, dv)| {
                        let mut point = corner;
                        point[u] += du;
                        point[v] += dv;
                        let id = &mut corner_ids[index(point, corner_dimensions)];
                        if *id == u32::MAX {
                            *id = vertices.len() as u32;
                            let [x, y, z] = std::array::from_fn(|axis| {
                                origin[axis] + spacing[axis] * point[axis] as f64
                            });
                            vertices.push(Point3::new(x, y, z));
                        }
                        *id
                    });
                    let [a, b, c, d] = ids;
                    if positive {
                        triangles.extend_from_slice(&[[a, b, c], [a, c, d]]);
                    } else {
                        triangles.extend_from_slice(&[[a, c, b], [a, d, c]]);
                    }
                }
            }
        }
        Ok(Self {
            vertices,
            triangles,
        })
    }

    fn closed_edges(&self) -> Result<usize, String> {
        let mut edges = BTreeMap::new();
        let mut links = vec![Vec::new(); self.vertices.len()];
        for &[a, b, c] in &self.triangles {
            for (center, from, to) in [(a, b, c), (b, c, a), (c, a, b)] {
                links[center as usize].push((from, to));
                let (n, orientation) = edges
                    .entry((from.min(to), from.max(to)))
                    .or_insert((0usize, 0i32));
                *n += 1;
                *orientation += if from < to { 1 } else { -1 };
            }
        }
        if edges.values().any(|&incidence| incidence != (2, 0))
            || links.iter().any(|link| !one_ring(link))
        {
            return Err("Threshold result has a nonmanifold voxel edge or corner".into());
        }
        Ok(edges.len())
    }
}

fn one_ring(edges: &[(u32, u32)]) -> bool {
    let Some(&(start, _)) = edges.first() else {
        return false;
    };
    let mut vertex = start;
    for step in 0..edges.len() {
        let Some(&(_, next)) = edges.iter().find(|&&(from, _)| from == vertex) else {
            return false;
        };
        if next == start {
            return step + 1 == edges.len();
        }
        vertex = next;
    }
    false
}
