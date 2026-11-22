//! Native VBD draping of authored garment panels with welded seams.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use kinetics_garment_draping::{
    ClothMesh, DMat2, DVec2, DVec3, Hinge, Triangle, VbdSettings, standard_fabrics, vbd_step,
};
use qualia_core_db::{
    etalon_render::surface_frames::tangent::tangents_from_uvs_into,
    render::mesh_surface_frames::vertex_normals_from_indices_into,
};

use crate::{
    Args, AssetHost, TypedArtifact, Value, artifact::AttributedMesh, count, fields, integer, list,
    num, number, required, text, vector,
};

const MAX_VERTICES: usize = 8192;
const MAX_FACES: usize = 16_384;
const MAX_SEAMS: usize = 4096;

#[derive(Clone, Copy)]
struct EdgeSide {
    from: usize,
    to: usize,
    opposite: usize,
    face: usize,
}

type EdgeIncidence = BTreeMap<[usize; 2], Vec<EdgeSide>>;

impl AssetHost<'_> {
    pub(crate) fn sew_and_drape_garment(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "pattern",
                "initial_drape",
                "seam_pairs",
                "max_initial_gap_m",
                "fabric",
                "pins",
                "step_s",
                "steps",
                "iterations",
                "gravity_m_s2",
                "damping_per_s",
            ],
        )?;
        let pattern = attributed(self, required(args, "pattern")?)?;
        let drape = attributed(self, required(args, "initial_drape")?)?;
        let rest = &pattern.surface.mesh;
        let start = &drape.surface.mesh;
        let vertex_count = rest.positions.len();
        let face_count = rest.triangles.len();
        let seams = list(required(args, "seam_pairs")?)?;
        if !(3..=MAX_VERTICES).contains(&vertex_count)
            || !(1..=MAX_FACES).contains(&face_count)
            || !(1..=MAX_SEAMS).contains(&seams.len())
            || start.positions.len() != vertex_count
            || rest.triangles != start.triangles
        {
            return Err(
                "Garment requires bounded matching indexed pattern and initial drape surfaces"
                    .into(),
            );
        }
        if !drape.morph_targets.is_empty()
            || !drape.skin_weights.is_empty()
            || drape.protected_faces.iter().any(|&mask| mask != 0)
        {
            return Err(
                "Garment drape needs a pre-skin surface with no MRP1 basis or protected faces"
                    .into(),
            );
        }
        if rest.positions.iter().any(|p| p[2] != 0.0)
            || rest
                .positions
                .iter()
                .chain(&start.positions)
                .flatten()
                .any(|coordinate| !coordinate.is_finite())
        {
            return Err("Garment pattern must be planar XY with finite geometry in metres".into());
        }
        let fabric = text(required(args, "fabric")?)?;
        let material = standard_fabrics()
            .into_iter()
            .find(|entry| entry.id == fabric)
            .ok_or("Garment fabric must be a native orthotropic preset ID")?
            .to_material();
        let gap_limit = num(required(args, "max_initial_gap_m")?)?;
        let steps = count(args, "steps", 1, 128)?;
        let iterations = count(args, "iterations", 8, 32)?;
        if steps == 0 || iterations == 0 {
            return Err("Garment integration requires positive step and iteration counts".into());
        }
        let settings = VbdSettings {
            dt: num(required(args, "step_s")?)?,
            iterations,
            gravity: args
                .get("gravity_m_s2")
                .map(vector::<3>)
                .transpose()?
                .map_or(DVec3::new(0.0, -9.81, 0.0), |[x, y, z]| DVec3::new(x, y, z)),
            damping: number(args, "damping_per_s", VbdSettings::default().damping)?,
        };
        if !(1e-4..=1.0 / 30.0).contains(&settings.dt) || settings.damping < 0.0 || gap_limit <= 0.0
        {
            return Err("Garment integration requires step_s in [0.0001, 1/30], nonnegative damping and a positive seam gap limit".into());
        }
        self.charge(
            face_count.saturating_mul(steps * settings.iterations * 256),
            vertex_count * 512 + face_count * 768 + seams.len() * 128,
        )?;
        let original = rest
            .triangles
            .iter()
            .map(|face| face.map(|slot| slot as usize))
            .collect::<Vec<_>>();
        let incidence = edge_incidence(&original, vertex_count)?;
        let positions = start
            .positions
            .iter()
            .map(|p| DVec3::new(p[0].into(), p[1].into(), p[2].into()))
            .collect::<Vec<_>>();
        let (map, groups, max_gap) = sew(&incidence, &positions, seams, gap_limit)?;
        let (mut welded, counts) = map.iter().zip(&positions).fold(
            (vec![DVec3::ZERO; groups], vec![0usize; groups]),
            |(mut sum, mut count), (&slot, &position)| {
                sum[slot] += position;
                count[slot] += 1;
                (sum, count)
            },
        );
        for (position, &count) in welded.iter_mut().zip(&counts) {
            *position = *position / count as f64;
        }
        let welded_faces = original
            .iter()
            .map(|face| face.map(|vertex| map[vertex]))
            .collect::<Vec<_>>();
        edge_incidence(&welded_faces, groups)?;
        let mut unique = BTreeSet::new();
        if welded_faces.iter().any(|&face| {
            let mut sorted = face;
            sorted.sort_unstable();
            !unique.insert(sorted)
        }) {
            return Err("Garment seams identify two faces with the same welded vertices".into());
        }

        let rest_points = rest
            .positions
            .iter()
            .map(|p| DVec2::new(p[0].into(), p[1].into()))
            .collect::<Vec<_>>();
        let triangles = original
            .iter()
            .zip(&welded_faces)
            .map(|(raw, &vertices)| {
                let rest = raw.map(|v| rest_points[v]);
                let dm = DMat2::from_cols(rest[1] - rest[0], rest[2] - rest[0]);
                let area = 0.5 * dm.determinant().abs();
                let inv_dm = dm
                    .inverse()
                    .ok_or("Garment contains a degenerate planar pattern face")?;
                Ok(Triangle {
                    vertices,
                    rest,
                    inv_dm,
                    area,
                    material: 0,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let hinges = incidence
            .values()
            .filter_map(|sides| {
                // Same-panel folds keep native orthotropic bending. Welded seams can rotate.
                let [a, b] = sides.as_slice() else {
                    return None;
                };
                let edge = rest_points[a.to] - rest_points[a.from];
                let length = edge.x.hypot(edge.y);
                let weight = length * length / (triangles[a.face].area + triangles[b.face].area);
                Some(Hinge {
                    vertices: [a.from, a.to, a.opposite, b.opposite].map(|v| map[v]),
                    weight,
                    across: DVec2::new(-edge.y / length, edge.x / length),
                    material: 0,
                })
            })
            .collect();
        let density = material.density;
        let mut cloth = ClothMesh::new(welded, triangles, hinges, material);
        cloth.masses.fill(0.0);
        for face in &cloth.triangles {
            for vertex in face.vertices {
                cloth.masses[vertex] += density * face.area / 3.0;
            }
        }
        if cloth
            .masses
            .iter()
            .any(|mass| !mass.is_finite() || *mass <= 0.0)
        {
            return Err("Garment contains an unreferenced or invalid mass vertex".into());
        }
        if let Some(raw) = args.get("pins") {
            let pins = list(raw)?;
            if pins.len() > vertex_count {
                return Err("Too many garment pins".into());
            }
            for value in pins {
                let vertex = slot(value, vertex_count)?;
                let target = positions[vertex];
                let pin = &mut cloth.pins[map[vertex]];
                if pin.is_some_and(|prior| prior != target) {
                    return Err("Garment weld would join mutually incompatible pins".into());
                }
                *pin = Some(target);
                cloth.positions[map[vertex]] = target;
            }
        }

        validate_deformed(&cloth)?;
        for _ in 0..steps {
            self.charge(0, 0)?;
            vbd_step(&mut cloth, &settings);
            validate_deformed(&cloth)?;
        }
        let strain = strain_table(&cloth)?;
        let posed = render_mesh(&drape, &cloth.positions, &welded_faces, &map)?;
        let mesh = self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(posed)))?;
        let strain = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(strain)))?;
        Ok(Value::Record(Args::from([
            ("mesh".into(), mesh),
            ("strain".into(), strain),
            ("max_initial_gap_m".into(), Value::F64(max_gap)),
            ("welded_vertex_count".into(), Value::U64(groups as u64)),
        ])))
    }
}

fn attributed(host: &AssetHost<'_>, value: &Value) -> Result<Arc<AttributedMesh>, String> {
    let TypedArtifact::AttributedMesh(mesh) = host.workspace.typed(value)? else {
        return Err("Garment requires typed attributed surfaces".into());
    };
    Ok(Arc::clone(mesh))
}

fn edge_key(a: usize, b: usize) -> [usize; 2] {
    [a.min(b), a.max(b)]
}

fn edge_incidence(faces: &[[usize; 3]], count: usize) -> Result<EdgeIncidence, String> {
    let mut edges = EdgeIncidence::new();
    for (face, &[a, b, c]) in faces.iter().enumerate() {
        if a == b || b == c || c == a || [a, b, c].into_iter().any(|v| v >= count) {
            return Err(format!("Invalid garment triangle {face}"));
        }
        for [from, to, opposite] in [[a, b, c], [b, c, a], [c, a, b]] {
            let sides = edges.entry(edge_key(from, to)).or_default();
            if sides.len() == 2 || sides.first().is_some_and(|prior| prior.from == from) {
                return Err(format!(
                    "Nonmanifold or inconsistently wound garment edge {from}–{to}"
                ));
            }
            sides.push(EdgeSide {
                from,
                to,
                opposite,
                face,
            });
        }
    }
    Ok(edges)
}

fn slot(value: &Value, count: usize) -> Result<usize, String> {
    let vertex = integer(value, "garment vertex")?;
    (vertex < count)
        .then_some(vertex)
        .ok_or_else(|| "Garment vertex is out of bounds".into())
}

fn seam_edge(
    value: &Value,
    index: &EdgeIncidence,
    count: usize,
) -> Result<([usize; 2], bool), String> {
    let [a, b] = list(value)? else {
        return Err("Garment seam side requires two vertex indices".into());
    };
    let (a, b) = (slot(a, count)?, slot(b, count)?);
    let sides = index
        .get(&edge_key(a, b))
        .ok_or("Garment seam side must be a boundary edge")?;
    let [edge] = sides.as_slice() else {
        return Err("Garment seam side must be a boundary edge".into());
    };
    Ok(([a, b], edge.from == a))
}

fn root(parent: &mut [usize], mut slot: usize) -> usize {
    while parent[slot] != slot {
        parent[slot] = parent[parent[slot]];
        slot = parent[slot];
    }
    slot
}

fn sew(
    index: &EdgeIncidence,
    points: &[DVec3],
    seams: &[Value],
    limit: f64,
) -> Result<(Vec<usize>, usize, f64), String> {
    let mut parent = (0..points.len()).collect::<Vec<_>>();
    let mut used = BTreeSet::new();
    let mut max_gap: f64 = 0.0;
    for (seam, pair) in seams.iter().enumerate() {
        let [a, b] = list(pair)? else {
            return Err("Garment seam requires two matched boundary edges".into());
        };
        let (a, forward_a) = seam_edge(a, index, points.len())?;
        let (b, forward_b) = seam_edge(b, index, points.len())?;
        if forward_a == forward_b
            || !used.insert(edge_key(a[0], a[1]))
            || !used.insert(edge_key(b[0], b[1]))
        {
            return Err(format!(
                "Garment seam {seam} must join unused, opposing boundaries in stitch correspondence order"
            ));
        }
        for (a, b) in a.into_iter().zip(b) {
            let d = points[a] - points[b];
            let gap = d.x.hypot(d.y).hypot(d.z);
            if gap > limit {
                return Err(format!(
                    "Garment seam {seam} initial residual {gap:e} m exceeds {limit:e} m"
                ));
            }
            max_gap = max_gap.max(gap);
            let (left, right) = (root(&mut parent, a), root(&mut parent, b));
            parent[left.max(right)] = left.min(right);
        }
    }
    let mut groups = BTreeMap::new();
    let map = (0..points.len())
        .map(|slot| {
            let next = groups.len();
            *groups.entry(root(&mut parent, slot)).or_insert(next)
        })
        .collect();
    Ok((map, groups.len(), max_gap))
}

fn validate_deformed(cloth: &ClothMesh) -> Result<(), String> {
    if cloth
        .positions
        .iter()
        .chain(&cloth.velocities)
        .any(|p| !p.x.is_finite() || !p.y.is_finite() || !p.z.is_finite())
    {
        return Err("Native garment solver produced a nonfinite state".into());
    }
    for (index, face) in cloth.triangles.iter().enumerate() {
        let [a, b, c] = face.vertices.map(|vertex| cloth.positions[vertex]);
        let normal = (b - a).cross(c - a);
        let area2 = normal.x.hypot(normal.y).hypot(normal.z);
        if !area2.is_finite() || area2 <= face.area * 1e-8 {
            return Err(format!(
                "Native garment face {index} collapsed to doubled area {area2:e} m²"
            ));
        }
    }
    Ok(())
}

fn strain_table(cloth: &ClothMesh) -> Result<Workbook, String> {
    let mut book = Workbook::new();
    let sheet_id = book.add_sheet("Garment Green-Lagrange strain");
    let sheet = book
        .sheet_mut(sheet_id)
        .ok_or("Could not create garment strain table")?;
    for (col, name) in ["triangle", "strain_xx", "strain_xy", "strain_yy"]
        .into_iter()
        .enumerate()
    {
        sheet.set_value(CellRef::new(0, col as u32), CellValue::Text(name.into()));
    }
    for (index, face) in cloth.triangles.iter().enumerate() {
        let [p0, p1, p2] = face.vertices.map(|vertex| cloth.positions[vertex]);
        let (a, b) = (p1 - p0, p2 - p0);
        let [x, y] = [face.inv_dm.x_axis, face.inv_dm.y_axis].map(|c| a * c.x + b * c.y);
        let values = [
            index as f64,
            0.5 * (x.dot(x) - 1.0),
            0.5 * x.dot(y),
            0.5 * (y.dot(y) - 1.0),
        ];
        if values.iter().any(|x| !x.is_finite()) {
            return Err("Garment strain is not finite".into());
        }
        for (col, value) in values.into_iter().enumerate() {
            sheet.set_value(
                CellRef::new(index as u32 + 1, col as u32),
                CellValue::Number(value),
            );
        }
    }
    Ok(book)
}

fn render_mesh(
    source: &AttributedMesh,
    welded: &[DVec3],
    welded_faces: &[[usize; 3]],
    map: &[usize],
) -> Result<AttributedMesh, String> {
    let welded = welded
        .iter()
        .map(|p| [p.x as f32, p.y as f32, p.z as f32])
        .collect::<Vec<_>>();
    if welded
        .iter()
        .flatten()
        .any(|coordinate| !coordinate.is_finite())
    {
        return Err("Garment surface exceeds its render coordinate range".into());
    }
    let indices_welded = welded_faces
        .iter()
        .flatten()
        .map(|&vertex| vertex as u32)
        .collect::<Vec<_>>();
    let mut normals = vec![[0.0; 3]; welded.len()];
    vertex_normals_from_indices_into(&welded, &indices_welded, &mut normals)
        .map_err(|error| format!("Garment normal reconstruction: {error:?}"))?;
    let mut result = source.clone();
    let surface = &mut result.surface;
    surface.normals = Some(map.iter().map(|&slot| normals[slot]).collect());
    let geometry = &mut surface.mesh;
    geometry.min = [f32::INFINITY; 3];
    geometry.max = [f32::NEG_INFINITY; 3];
    for (point, &slot) in geometry.positions.iter_mut().zip(map) {
        *point = welded[slot];
        for axis in 0..3 {
            geometry.min[axis] = geometry.min[axis].min(point[axis]);
            geometry.max[axis] = geometry.max[axis].max(point[axis]);
        }
    }
    surface.tangents = match (&surface.texture_coordinates_0, &surface.normals) {
        (Some(uv), Some(normals)) => {
            let indices = geometry
                .triangles
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<_>>();
            let mut tangents = vec![[0.0; 4]; map.len()];
            let mut scratch = vec![[0.0; 6]; map.len()];
            tangents_from_uvs_into(
                &geometry.positions,
                uv,
                normals,
                &indices,
                &mut tangents,
                &mut scratch,
            )
            .map_err(|error| format!("Garment tangent reconstruction: {error:?}"))?;
            Some(tangents)
        }
        _ => None,
    };
    Ok(result)
}
