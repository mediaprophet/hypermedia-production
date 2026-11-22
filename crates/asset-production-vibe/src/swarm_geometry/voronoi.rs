//! Convex authored Voronoi fracture and reference-only scene admission.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use qualia_core_db::container_10d::{
    attach_material_section, decode_mesh_section, parse_section_table, Container10dHeader,
    MaterialRecord, SectionType, SubmeshRange,
};
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d, upgrade_legacy_10d_to_holographic_v2,
};
use qualia_core_db::{bundle::semantic_model::resource_identity_meta, q_hash};
use sha2::{Digest, Sha256};

use super::faceted_step::{plain_mesh, validate_convex_shell};
use super::*;

const MAX_SITES: usize = 12;
const MAX_SOURCE_TRIANGLES: usize = 256;
const MAX_FRAGMENT_TRIANGLES: usize = 2048;
const FRAGMENT_CLASS: &str = "https://qualiadb.org/ontology/hmc#FractureFragment";

#[derive(Clone)]
struct Face {
    points: Vec<Point>,
    exterior_slot: Option<u64>,
    adjacent_site: Option<usize>,
}

struct Fragment {
    points: Vec<Point>,
    triangles: Vec<[u32; 3]>,
    material_ids: Vec<u64>,
    interfaces: BTreeMap<usize, Contact>,
    volume: f64,
}

struct Contact {
    area_m2: f64,
    inner_uv_faces: Vec<Value>,
}

impl AssetHost<'_> {
    pub(crate) fn fracture_voronoi_3d(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mesh",
                "sites_m",
                "cleavage_weights",
                "interior_material_iri",
                "tolerance_m",
                "entity_iri",
            ],
        )?;
        let mesh_handle = required(args, "mesh")?;
        let source = source_mesh(self, mesh_handle)?;
        let surface = &source.surface;
        let source_mesh_ref = &surface.mesh;
        if source_mesh_ref.triangles.len() > MAX_SOURCE_TRIANGLES
            || source_mesh_ref.positions.len() > MAX_SOURCE_TRIANGLES
            || surface.texture_coordinates_0.is_some()
            || !surface.texture_dependencies.is_empty()
            || !source.uv1.is_empty()
            || !source.vertex_colors.is_empty()
            || !source.skin_weights.is_empty()
            || !source.morph_targets.is_empty()
            || source.protected_faces.iter().any(|&mask| mask != 0)
        {
            return Err("Fracture requires a static, untextured, unprotected shell of up to 256 vertices and triangles".into());
        }
        let weights: Point = args
            .get("cleavage_weights")
            .map(vector)
            .transpose()?
            .unwrap_or([1.0; 3]);
        if weights.iter().any(|x| !(0.01..=100.0).contains(x)) {
            return Err("Cleavage weights must each be in [0.01, 100]".into());
        }
        let sites = list(required(args, "sites_m")?)?;
        if !(2..=MAX_SITES).contains(&sites.len()) {
            return Err("Fracture requires between 2 and 12 authored interior sites".into());
        }
        let sites = sites
            .iter()
            .map(vector)
            .collect::<Result<Vec<Point>, _>>()?;
        let entity = text(required(args, "entity_iri")?)?;
        let interior = text(required(args, "interior_material_iri")?)?;
        if entity.len() > 2048
            || interior.len() > 2048
            || resource_identity_meta(entity).is_err()
            || resource_identity_meta(interior).is_err()
        {
            return Err("Fracture requires canonical entity and interior material IRIs".into());
        }
        let interior_id = q_hash(interior);
        let default_material = MaterialRecord::legacy_default();
        let materials = surface.materials.clone();
        if materials.len() > MAX_SOURCE_TRIANGLES
            || materials.iter().any(textured)
            || materials
                .iter()
                .map(|m| m.id)
                .collect::<BTreeSet<_>>()
                .len()
                != materials.len()
        {
            return Err("Fracture admits up to 256 distinct untextured exterior materials".into());
        }
        let authored_interior = materials.iter().any(|material| material.id == interior_id);
        let slots = material_slots(source)?;
        if slots.iter().flatten().any(|&(slot, semantic)| {
            semantic != 0 && semantic != q_hash(entity)
                || !materials.iter().any(|material| material.id == slot)
        }) {
            return Err("Fracture supports whole-shell authored semantics and declared untextured materials".into());
        }
        let vertices = source_mesh_ref
            .positions
            .iter()
            .copied()
            .map(point)
            .collect::<Vec<_>>();
        let source_geometry = plain_mesh(&vertices, source_mesh_ref.triangles.clone())
            .surface
            .mesh;
        let triangles = &source_geometry.triangles;
        let extent = length(sub(point(source_geometry.max), point(source_geometry.min)));
        let tolerance = number(args, "tolerance_m", (extent / 65535.0).max(1e-6))?;
        if !(1e-9..=1e-3).contains(&tolerance) {
            return Err("Fracture tolerance_m must be in [1e-9, 1e-3]".into());
        }
        if !extent.is_finite() || extent < tolerance * 128.0 {
            return Err("Fracture shell is too small for its geometric tolerance".into());
        }
        self.charge(
            sites
                .len()
                .saturating_mul(sites.len())
                .saturating_mul(triangles.len())
                .saturating_mul(240),
            sites
                .len()
                .saturating_mul(MAX_FRAGMENT_TRIANGLES)
                .saturating_mul(700),
        )?;
        let original_volume = validate_convex_shell(&vertices, triangles, tolerance)?;
        let base = triangles
            .iter()
            .zip(slots)
            .map(|(&triangle, slot)| Face {
                points: triangle.iter().map(|&i| vertices[i as usize]).collect(),
                exterior_slot: Some(slot.map_or(default_material.id, |pair| pair.0)),
                adjacent_site: None,
            })
            .collect::<Vec<_>>();
        for (i, &site) in sites.iter().enumerate() {
            if sites[..i]
                .iter()
                .any(|&prior| length(sub(prior, site)) <= tolerance * 32.0)
            {
                return Err("Fracture sites are too close for their geometric tolerance".into());
            }
            for &triangle in triangles {
                let [a, b, c] = triangle.map(|v| vertices[v as usize]);
                if dot(face_normal([a, b, c])?, sub(site, a)) >= -tolerance * 8.0 {
                    return Err("Every fracture site must lie strictly inside the shell".into());
                }
            }
        }
        let mut fragments = Vec::with_capacity(sites.len());
        for (i, &center) in sites.iter().enumerate() {
            let mut shell = base.clone();
            for (j, &other) in sites.iter().enumerate().filter(|&(j, _)| j != i) {
                let normal =
                    normalized(std::array::from_fn(|k| (other[k] - center[k]) * weights[k]))?;
                shell = clip_shell(shell, mul(add(center, other), 0.5), normal, tolerance, j)?;
            }
            let mut fragment =
                triangulate(shell, &sites, i, weights, extent, tolerance, interior_id)?;
            self.charge(
                fragment
                    .points
                    .len()
                    .saturating_mul(fragment.triangles.len())
                    .saturating_mul(12),
                0,
            )?;
            fragment.volume =
                validate_convex_shell(&fragment.points, &fragment.triangles, tolerance)?;
            fragments.push(fragment);
        }
        let tolerance_area = tolerance * extent * 16.0;
        let glue = glue_constraints(&fragments, tolerance_area)?;
        let fractured_volume = fragments
            .iter()
            .map(|fragment| fragment.volume)
            .sum::<f64>();
        let residual = (original_volume - fractured_volume).abs();
        if residual > tolerance_area * extent || residual > original_volume * 1e-3 {
            return Err("Fracture exceeds its volume conservation tolerance".into());
        }

        let mut new_materials = materials
            .into_iter()
            .map(|m| (m.id, m))
            .collect::<BTreeMap<_, _>>();
        new_materials
            .entry(default_material.id)
            .or_insert(default_material);
        new_materials.entry(interior_id).or_insert(MaterialRecord {
            id: interior_id,
            ..default_material
        });
        let new_materials = new_materials.into_values().collect::<Vec<_>>();
        let source_ids = base
            .iter()
            .map(|face| face.exterior_slot.unwrap_or(default_material.id))
            .collect::<Vec<_>>();
        let source_ranges = fragment_ranges(&source_ids, q_hash(entity));
        let source_file = seal_mesh(
            &source_geometry,
            entity,
            &new_materials,
            &source_ranges,
            tolerance,
        )?;
        let family = fracture_identity(&source_file, &sites, weights, tolerance, interior);
        if fragments.len() > Self::assembly_frame_leaf_capacity(&source_file)? {
            return Err("Fracture site count exceeds available assembly reference slots".into());
        }
        let frame = self.admit_encoded("10d", source_file)?;
        let mut definitions = Vec::with_capacity(sites.len());
        let mut placements = Vec::with_capacity(sites.len());
        let mut outputs = Vec::with_capacity(sites.len());
        for (i, fragment) in fragments.into_iter().enumerate() {
            let iri = format!("urn:qualia:voronoi:{family}:fragment:{i}");
            let key = format!("voronoi/{family}/fragment-{i}.10d");
            let mut mesh = plain_mesh(&fragment.points, fragment.triangles);
            let ranges = fragment_ranges(&fragment.material_ids, q_hash(&iri));
            let bytes = seal_mesh(&mesh.surface.mesh, &iri, &new_materials, &ranges, tolerance)?;
            let encoded = self.admit_encoded("10d", bytes)?;
            mesh.surface.materials = new_materials.clone();
            mesh.surface.submeshes = ranges;
            let typed = self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(mesh)))?;
            let uv = fragment
                .interfaces
                .into_values()
                .flat_map(|contact| contact.inner_uv_faces)
                .collect();
            outputs.push(Value::Record(Args::from([
                ("leaf_key".into(), Value::String(key.clone())),
                ("entity_iri".into(), Value::String(iri.clone())),
                ("mesh".into(), typed),
                ("resource".into(), encoded.clone()),
                ("volume_m3".into(), Value::F64(fragment.volume)),
                ("interior_corner_uv_m".into(), Value::List(uv)),
            ])));
            definitions.push(Value::Record(Args::from([
                ("leaf_key".into(), Value::String(key.clone())),
                ("resource".into(), encoded),
                (
                    "taxon_or_class_iri".into(),
                    Value::String(FRAGMENT_CLASS.into()),
                ),
            ])));
            placements.push(Value::Record(Args::from([
                ("leaf_key".into(), Value::String(key)),
                (
                    "entity_iri".into(),
                    Value::String(format!("{iri}:instance")),
                ),
                (
                    "local_pga_motor".into(),
                    coordinates([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
                ),
            ])));
        }
        let Value::Record(mut result) = self.compose_assembly_10d(&Args::from([
            ("assembly_frame".into(), frame),
            ("archetypes".into(), Value::List(definitions)),
            ("instances".into(), Value::List(placements)),
        ]))?
        else {
            return Err("Unexpected scene assembly result".into());
        };
        result.extend([
            ("fragments".into(), Value::List(outputs)),
            ("glue_constraints".into(), Value::List(glue)),
            (
                "damage_report".into(),
                Value::Record(Args::from([
                    ("source_volume_m3".into(), Value::F64(original_volume)),
                    ("fragment_volume_m3".into(), Value::F64(fractured_volume)),
                    ("volume_residual_m3".into(), Value::F64(residual)),
                    ("tolerance_m".into(), Value::F64(tolerance)),
                    ("interior_material_id".into(), Value::U64(interior_id)),
                    (
                        "authored_interior_material".into(),
                        Value::Bool(authored_interior),
                    ),
                ])),
            ),
        ]);
        Ok(Value::Record(result))
    }
}

fn clip_shell(
    shell: Vec<Face>,
    origin: Point,
    normal: Point,
    tolerance: f64,
    neighbor: usize,
) -> Result<Vec<Face>, String> {
    let epsilon = tolerance * 1e-3;
    let classify = |p: Point| {
        let residual = dot(normal, sub(p, origin));
        if residual.abs() <= epsilon {
            0.0
        } else {
            residual
        }
    };
    if shell
        .iter()
        .flat_map(|face| &face.points)
        .all(|&p| classify(p) <= 0.0)
    {
        return Ok(shell);
    }
    let mut output = Vec::with_capacity(shell.len() + 1);
    let mut cap = Vec::new();
    for mut face in shell {
        let mut clipped = Vec::new();
        for (&a, &b) in face.points.iter().zip(face.points.iter().cycle().skip(1)) {
            let da = classify(a);
            let db = classify(b);
            if da <= 0.0 {
                clipped.push(a);
            }
            if da * db < 0.0 {
                let crossing = mul(sub(mul(b, da), mul(a, db)), (da - db).recip());
                clipped.push(crossing);
            }
        }
        if clipped.len() >= 3 {
            for &p in &clipped {
                if classify(p) == 0.0
                    && !cap
                        .iter()
                        .any(|&existing| length(sub(existing, p)) <= epsilon)
                {
                    cap.push(p);
                }
            }
            face.points = clipped;
            output.push(face);
        }
    }
    if cap.len() < 3 {
        return Err("Fracture clipping produced a numerically unresolved interface".into());
    }
    let center = centroid(&cap);
    let (u, v) = planar_axes(normal)?;
    cap.sort_by(|&a, &b| {
        let p = sub(a, center);
        let q = sub(b, center);
        dot(p, v)
            .atan2(dot(p, u))
            .total_cmp(&dot(q, v).atan2(dot(q, u)))
    });
    output.push(Face {
        points: cap,
        exterior_slot: None,
        adjacent_site: Some(neighbor),
    });
    Ok(output)
}

fn triangulate(
    faces: Vec<Face>,
    sites: &[Point],
    index: usize,
    weights: Point,
    extent: f64,
    tolerance: f64,
    interior_id: u64,
) -> Result<Fragment, String> {
    let mut points = Vec::new();
    let mut by_coordinate = BTreeMap::<[i64; 3], Vec<(Point, u32)>>::new();
    let mut triangles = Vec::new();
    let mut material_ids = Vec::new();
    let mut interfaces = BTreeMap::new();
    let mut weld = |raw: Point| -> Result<u32, String> {
        let snapped = raw.map(|value| {
            if value.abs() <= tolerance * 1e-5 {
                0.0
            } else {
                (value as f32) as f64
            }
        });
        if length(sub(raw, snapped)) > tolerance * 0.25 {
            return Err("Fragment position exceeds its f32 rounding allowance".into());
        }
        let width = tolerance * 0.015625;
        let cell = raw.map(|value| (value / width).floor() as i64);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let key = [cell[0] + dx, cell[1] + dy, cell[2] + dz];
                    if let Some((_, i)) = by_coordinate
                        .get(&key)
                        .into_iter()
                        .flatten()
                        .find(|&&(candidate, _)| length(sub(candidate, raw)) <= width)
                    {
                        return Ok(*i);
                    }
                }
            }
        }
        let i = points.len() as u32;
        points.push(snapped);
        by_coordinate.entry(cell).or_default().push((raw, i));
        Ok(i)
    };
    for face in faces {
        let geometry = face.points;
        let ring = geometry
            .iter()
            .copied()
            .map(&mut weld)
            .collect::<Result<Vec<_>, _>>()?;
        let mut area = 0.0;
        let mut uv = Vec::new();
        let chart = face
            .adjacent_site
            .map(|j| {
                let low = index.min(j);
                let high = index.max(j);
                let normal = normalized(std::array::from_fn(|k| {
                    (sites[high][k] - sites[low][k]) * weights[k]
                }))?;
                Ok::<_, String>((mul(add(sites[low], sites[high]), 0.5), planar_axes(normal)?))
            })
            .transpose()?;
        let center = centroid(&geometry);
        let n = ring.len();
        let mut emit = |triangle: [u32; 3], xyz: [Point; 3]| -> Result<(), String> {
            let [a, b, c] = xyz;
            let triangle_area = length(cross(sub(b, a), sub(c, a))) * 0.5;
            if triangle_area <= tolerance * extent * 1e-5
                || triangle[0] == triangle[1]
                || triangle[1] == triangle[2]
                || triangle[2] == triangle[0]
            {
                return Err("Fracture creates a face below its geometric resolution".into());
            }
            if let Some((origin, (u, v))) = chart {
                uv.push(Value::Record(Args::from([
                    ("face_index".into(), Value::U64(triangles.len() as u64)),
                    (
                        "uv_m".into(),
                        Value::List(
                            xyz.map(|p| {
                                let p = sub(p, origin);
                                coordinates([dot(p, u), dot(p, v)])
                            })
                            .into(),
                        ),
                    ),
                ])));
            }
            triangles.push(triangle);
            material_ids.push(face.exterior_slot.unwrap_or(interior_id));
            area += triangle_area;
            Ok(())
        };
        if n == 3 {
            emit(
                [ring[0], ring[1], ring[2]],
                [geometry[0], geometry[1], geometry[2]],
            )?;
        } else {
            let hub = weld(center)?;
            for i in 0..n {
                let next = (i + 1) % n;
                emit(
                    [hub, ring[i], ring[next]],
                    [center, geometry[i], geometry[next]],
                )?;
            }
        }
        if let Some(j) = face.adjacent_site {
            interfaces.insert(
                j,
                Contact {
                    area_m2: area,
                    inner_uv_faces: uv,
                },
            );
        }
    }
    if triangles.len() > MAX_FRAGMENT_TRIANGLES || points.len() > MAX_FRAGMENT_TRIANGLES {
        return Err("Fracture fragment exceeds its admission limit".into());
    }
    Ok(Fragment {
        points,
        triangles,
        material_ids,
        interfaces,
        volume: 0.0,
    })
}

fn glue_constraints(fragments: &[Fragment], tolerance: f64) -> Result<Vec<Value>, String> {
    let mut edges = Vec::new();
    for (i, fragment) in fragments.iter().enumerate() {
        for (&j, contact) in &fragment.interfaces {
            let twin = fragments[j]
                .interfaces
                .get(&i)
                .ok_or("Fragment contact is missing its opposite face")?;
            let area = (contact.area_m2 + twin.area_m2) * 0.5;
            let residual = (contact.area_m2 - twin.area_m2).abs();
            if area <= 0.0 || residual > tolerance || residual > area * 1e-3 {
                return Err("Opposite fragment contact areas disagree".into());
            }
            if i < j {
                edges.push(Value::Record(Args::from([
                    ("fragment_indices".into(), indices([i as u32, j as u32])),
                    ("shared_area_m2".into(), Value::F64(area)),
                    ("area_residual_m2".into(), Value::F64(residual)),
                ])));
            }
        }
    }
    if edges.is_empty() {
        return Err("Fracture produced no glue contact interfaces".into());
    }
    Ok(edges)
}

fn planar_axes(normal: Point) -> Result<(Point, Point), String> {
    let reference = if normal[0].abs() <= 0.55 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let u = normalized(cross(reference, normal))?;
    Ok((u, cross(normal, u)))
}

fn centroid(points: &[Point]) -> Point {
    mul(
        points.iter().copied().fold([0.0; 3], add),
        (points.len() as f64).recip(),
    )
}

fn seal_mesh(
    mesh: &qualia_core_db::render::assets::Mesh,
    iri: &str,
    materials: &[MaterialRecord],
    ranges: &[SubmeshRange],
    tolerance: f64,
) -> Result<Vec<u8>, String> {
    let mut bytes =
        compile_mesh_to_10d(mesh).map_err(|error| format!("Fracture mesh codec: {error:?}"))?;
    let header =
        Container10dHeader::parse(&bytes).map_err(|error| format!("Fracture header: {error:?}"))?;
    let descriptors = parse_section_table(&bytes, &header)
        .map_err(|error| format!("Fracture mesh section: {error:?}"))?;
    let section = descriptors
        .iter()
        .find(|d| d.section_type == SectionType::QuantizedMesh as u8)
        .ok_or("Fracture encoder emitted no mesh section")?;
    let start = section.byte_offset as usize;
    let end = start
        .checked_add(section.byte_length as usize)
        .ok_or("Fracture mesh end overflow")?;
    let decoded = decode_mesh_section(
        bytes
            .get(start..end)
            .ok_or("Truncated fracture mesh section")?,
    )
    .map_err(|error| format!("Fracture mesh readback: {error:?}"))?;
    if decoded.triangles != mesh.triangles
        || decoded.positions.len() != mesh.positions.len()
        || decoded
            .positions
            .iter()
            .zip(&mesh.positions)
            .any(|(&a, &b)| length(sub(point(a), point(b))) > tolerance)
    {
        return Err("Fracture mesh codec exceeded its topology or position tolerance".into());
    }
    for &triangle in &mesh.triangles {
        let expected = face_normal(triangle.map(|i| point(mesh.positions[i as usize])))?;
        let actual = face_normal(triangle.map(|i| point(decoded.positions[i as usize])))?;
        if dot(expected, actual) <= 0.0 {
            return Err("Fracture mesh quantization changes a triangle's orientation".into());
        }
    }
    if !materials.is_empty() {
        bytes =
            attach_material_section(&bytes, materials, ranges, (mesh.triangles.len() * 3) as u32)
                .map_err(|error| format!("Fracture material codec: {error:?}"))?;
    }
    let [x, y, z] = mesh.centroid();
    upgrade_legacy_10d_to_holographic_v2(
        &bytes,
        iri,
        1,
        [0.0, 3.0, 2.0, x, y, z, 0.0, 1.0, 1.0, 0.5],
    )
    .map_err(|error| format!("Fracture semantic sealing: {error:?}"))
}

fn textured(m: &MaterialRecord) -> bool {
    [
        m.base_color_texture,
        m.normal_texture,
        m.metallic_roughness_texture,
        m.occlusion_texture,
        m.emissive_texture,
        m.stylized_ramp_texture,
    ]
    .iter()
    .any(|digest| *digest != [0; 32])
}

fn fracture_identity(
    source: &[u8],
    sites: &[Point],
    weights: Point,
    tolerance: f64,
    interior: &str,
) -> String {
    let mut digest = Sha256::new();
    digest.update(b"qualia:voronoi:v1");
    digest.update(Sha256::digest(source));
    digest.update([sites.len() as u8]);
    for value in sites
        .iter()
        .flatten()
        .copied()
        .chain(weights)
        .chain([tolerance])
    {
        digest.update(value.to_bits().to_le_bytes());
    }
    digest.update(interior.as_bytes());
    format!("{:x}", digest.finalize())
}

fn fragment_ranges(ids: &[u64], semantic_id: u64) -> Vec<SubmeshRange> {
    let mut offset = 0;
    ids.chunk_by(|a, b| a == b)
        .map(|run| {
            let count = (3 * run.len()) as u32;
            let range = SubmeshRange {
                first_index: offset,
                index_count: count,
                material_id: run[0],
                semantic_id,
            };
            offset += count;
            range
        })
        .collect()
}

fn coordinates<const N: usize>(values: [f64; N]) -> Value {
    Value::List(values.into_iter().map(Value::F64).collect())
}
