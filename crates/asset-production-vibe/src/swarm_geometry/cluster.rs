//! Authored `.10d` CDAG leaf clusters with draw-ready companion streams.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use bytemuck::Zeroable;
use qualia_core_db::bundle::semantic_model::SemanticModelBuilder;
use qualia_core_db::container_10d::cluster_dag_section::{
    cluster_dag_encoded_len, decode_cluster_dag_section, encode_cluster_dag_section,
    ClusterDagMiniHeader, MeshletClusterNode, MAX_MESHLET_TRIANGLES, MAX_MESHLET_VERTICES,
};
use qualia_core_db::container_10d::{self as ten, MaterialRecord, SectionType, SubmeshRange};
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d, upgrade_legacy_10d_to_holographic_v2,
};
use qualia_core_db::specialized_libs::computational_geometry::{
    geometry_workspace::{
        production_budget::{BufferClass, CapacityPlan, ProductionBudget},
        Cancellation,
    },
    production_contracts::ContentId,
    production_runtime::{
        generate_indexed_draws, DrawIndexedCommand, ReplacementLink, ValidatedHierarchy,
    },
};
use sha2::{Digest, Sha256};

use super::*;
use crate::qualia_containers_governance_poet_ops::encoded_resource;

mod impostor;

const MAX_SOURCE_TRIANGLES: usize = 20_000;
const MAX_SOURCE_VERTICES: usize = 60_000;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Binding {
    material_slot: u8,
    semantic_key: u32,
}

struct Streams {
    nodes: Vec<MeshletClusterNode>,
    positions: Vec<u8>,
    triangles: Vec<u8>,
}

impl AssetHost<'_> {
    pub(crate) fn build_cluster_dag_and_impostor(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mesh",
                "source_10d",
                "entity_iri",
                "meshlet_vertices",
                "meshlet_triangles",
                "bake_octahedral_impostor",
                "impostor_grid",
                "impostor_view_resolution",
            ],
        )?;
        let bake = crate::boolean(args, "bake_octahedral_impostor", false)?;
        if !bake
            && (args.contains_key("impostor_grid") || args.contains_key("impostor_view_resolution"))
        {
            return Err("Impostor settings require bake_octahedral_impostor".into());
        }
        let vertex_cap = count(
            args,
            "meshlet_vertices",
            usize::from(MAX_MESHLET_VERTICES),
            usize::from(MAX_MESHLET_VERTICES),
        )?;
        let triangle_cap = count(
            args,
            "meshlet_triangles",
            usize::from(MAX_MESHLET_TRIANGLES),
            usize::from(MAX_MESHLET_TRIANGLES),
        )?;
        if vertex_cap < 3 || triangle_cap == 0 {
            return Err("Meshlet budgets need at least three vertices and one triangle".into());
        }
        let bytes = match (args.get("mesh"), args.get("source_10d")) {
            (Some(handle), None) => mesh_leaf(self, handle, args.get("entity_iri"))?,
            (None, Some(handle)) => {
                if args.contains_key("entity_iri") {
                    return Err("Authored source_10d supplies its own sealed entity IRI".into());
                }
                let source = encoded_resource(self, handle)?;
                if source.kind != "10d"
                    || source.bytes.len()
                        > self
                            .limits
                            .resource_bytes
                            .min(ten::GRAPHICS_ASSET_RING_BYTES)
                {
                    return Err("Cluster authoring requires a bounded authored .10d source".into());
                }
                let size = source.bytes.len();
                self.charge(8 * size, 3 * size)?;
                encoded_resource(self, handle)?.bytes.clone()
            }
            _ => return Err("Supply exactly one of mesh or source_10d".into()),
        };
        let limit = self
            .limits
            .resource_bytes
            .min(ten::GRAPHICS_ASSET_RING_BYTES);
        let header = ten::Container10dHeader::parse(&bytes)
            .map_err(|error| format!("Cluster source header: {error:?}"))?;
        if header.header_crc32c != ten::compute_whole_file_crc32c(&bytes) {
            return Err("Cluster source failed its sealed .10d integrity check".into());
        }
        let table = ten::parse_section_table(&bytes, &header)
            .map_err(|error| format!("Cluster source section table: {error:?}"))?;
        if table.iter().any(|d| {
            matches!(
                d.typ(),
                Some(
                    SectionType::ClusterDag
                        | SectionType::ImpostorAtlas
                        | SectionType::SkeletalAnimation
                )
            )
        }) {
            return Err(
                "Cluster authoring requires a static source with no prior CDAG or IMPO".into(),
            );
        }
        let mut scratch = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
        let mut records = scratch;
        let view = ten::TypedPayloadResolver::new(&bytes, &[], &mut scratch, &mut records)
            .map_err(|error| error.to_string())?;
        if view
            .payloads_of_kind(ten::FIELD_KIND_ARCHETYPE_REF)
            .next()
            .is_some()
            || view
                .payloads_of_kind(ten::FIELD_KIND_MORPH_TARGETS)
                .next()
                .is_some()
        {
            return Err(
                "Cluster authoring accepts a static leaf with no dependent or deforming geometry"
                    .into(),
            );
        }
        let mesh_payload = view
            .section_payload(SectionType::QuantizedMesh)
            .ok_or("Cluster authoring requires authored QuantizedMesh geometry")?;
        let (mesh_header, _) = ten::parse_mesh_header(mesh_payload)
            .map_err(|error| format!("Cluster mesh header: {error:?}"))?;
        let vertex_count = mesh_header.vertex_count as usize;
        let triangle_count = mesh_header.triangle_count as usize;
        if !(3..=MAX_SOURCE_VERTICES).contains(&vertex_count)
            || !(1..=MAX_SOURCE_TRIANGLES).contains(&triangle_count)
        {
            return Err(
                "Cluster geometry exceeds the 60,000 vertex or 20,000 triangle limit".into(),
            );
        }
        self.charge(
            triangle_count * 1600 + vertex_count * 30,
            triangle_count * 360 + vertex_count * 24,
        )?;
        let (_, spatial) = SemanticModelBuilder::new(limit)
            .merge_10d(&bytes)
            .map_err(|error| format!("Cluster source semantic bindings: {error}"))?;
        let spatial = spatial
            .filter(|identity| identity.stratum_mask != 0)
            .ok_or("Cluster authoring requires a locally addressed HOLO/MSTR root")?;
        let mesh = ten::decode_mesh_section(mesh_payload)
            .map_err(|error| format!("Cluster mesh codec: {error:?}"))?;
        let (materials, ranges) = read_materials(&bytes, triangle_count)?;
        let (bindings, semantic_slots) =
            bindings(&materials, &ranges, triangle_count, spatial.root_q_hash)?;
        let streams = meshlets(&mesh, &bindings, vertex_cap, triangle_cap)?;
        let header_cdag = ClusterDagMiniHeader {
            max_lod_depth: 0,
            group_count: 0,
            total_meshlet_vertices: (streams.positions.len() / 12) as u32,
            total_meshlet_triangles: (streams.triangles.len() / 6) as u32,
            root_cluster_offset: 0,
            flags: 0,
            ..ClusterDagMiniHeader::default()
        };
        let mut cdag = vec![
            0;
            cluster_dag_encoded_len(streams.nodes.len())
                .ok_or("Cluster section length is out of range")?
        ];
        encode_cluster_dag_section(&header_cdag, &streams.nodes, &mut cdag)
            .map_err(|error| error.to_string())?;
        let mut reread = vec![MeshletClusterNode::zeroed(); streams.nodes.len()];
        let (admitted, n) =
            decode_cluster_dag_section(&cdag, &mut reread).map_err(|error| error.to_string())?;
        if n != streams.nodes.len()
            || reread != streams.nodes
            || admitted.total_meshlet_triangles != triangle_count as u32
        {
            return Err("Cluster CDAG failed deterministic native readback".into());
        }
        drop(reread);
        validate_draw_streams(&bytes, &mesh, &streams, self.limits.total_resource_bytes)?;
        let impostor = if bake {
            if view.section_payload(SectionType::SurfaceReading).is_some() {
                return Err(
                    "Impostor baking requires constant material albedo throughout each range"
                        .into(),
                );
            }
            let grid = count(args, "impostor_grid", 8, 16)?;
            let resolution = count(args, "impostor_view_resolution", 32, 96)?;
            if !(2..=16).contains(&grid) || !(8..=96).contains(&resolution) {
                return Err(
                    "Impostor grid or tile resolution is outside the admitted range".into(),
                );
            }
            let pixels = grid * grid * resolution * resolution;
            self.charge(
                pixels.saturating_mul(triangle_count.saturating_mul(8)),
                pixels.saturating_mul(12),
            )?;
            Some(impostor::bake(
                self, &mesh, &materials, &bindings, grid, resolution,
            )?)
        } else {
            None
        };
        let encoded = append_sections(&bytes, &header, table, &cdag, impostor.as_deref(), limit)?;
        let fingerprint = format!("{:x}", Sha256::digest(&encoded));
        let positions_digest = format!("{:x}", Sha256::digest(&streams.positions));
        let indices_digest = format!("{:x}", Sha256::digest(&streams.triangles));
        let asset = self.admit_encoded("10d", encoded)?;
        let positions = self.admit_encoded("bin", streams.positions)?;
        let triangles = self.admit_encoded("bin", streams.triangles)?;
        let semantic_slots = semantic_slots
            .into_iter()
            .map(|(id, hash)| {
                Value::Record(Args::from([
                    ("cdag_id".into(), Value::U64(u64::from(id))),
                    ("q_hash".into(), Value::U64(hash)),
                ]))
            })
            .collect();
        Ok(Value::Record(Args::from([
            ("asset".into(), asset),
            ("asset_sha256".into(), Value::String(fingerprint)),
            ("meshlet_positions_f32x3_le".into(), positions),
            (
                "meshlet_positions_sha256".into(),
                Value::String(positions_digest),
            ),
            ("meshlet_triangle_indices_u16_le".into(), triangles),
            (
                "meshlet_triangle_indices_sha256".into(),
                Value::String(indices_digest),
            ),
            ("semantic_slots".into(), Value::List(semantic_slots)),
            (
                "klod_metrics".into(),
                Value::Record(Args::from([
                    ("geometry_lod_depth".into(), Value::U64(0)),
                    ("meshlet_count".into(), Value::U64(n as u64)),
                    (
                        "source_triangle_count".into(),
                        Value::U64(triangle_count as u64),
                    ),
                    (
                        "meshlet_vertex_count".into(),
                        Value::U64(u64::from(admitted.total_meshlet_vertices)),
                    ),
                    ("impostor_baked".into(), Value::Bool(bake)),
                ])),
            ),
        ])))
    }
}

fn mesh_leaf(
    host: &mut AssetHost<'_>,
    handle: &Value,
    iri: Option<&Value>,
) -> Result<Vec<u8>, String> {
    let source = source_mesh(host, handle)?;
    let mesh = &source.surface.mesh;
    if mesh.positions.len() > MAX_SOURCE_VERTICES
        || mesh.triangles.len() > MAX_SOURCE_TRIANGLES
        || source.surface.texture_coordinates_0.is_some()
        || !source.surface.texture_dependencies.is_empty()
        || !source.uv1.is_empty()
        || !source.vertex_colors.is_empty()
        || !source.skin_weights.is_empty()
        || !source.morph_targets.is_empty()
        || source.protected_faces.iter().any(|&mask| mask != 0)
    {
        return Err("Cluster mesh input needs a bounded static untextured leaf".into());
    }
    let work = 700 * mesh.triangles.len() + 30 * mesh.positions.len();
    let scratch = 160 * mesh.triangles.len() + 24 * mesh.positions.len();
    host.charge(work, scratch)?;
    let source = source_mesh(host, handle)?;
    let positions = source
        .surface
        .mesh
        .positions
        .iter()
        .copied()
        .map(point)
        .collect::<Vec<_>>();
    let mesh = super::faceted_step::plain_mesh(&positions, source.surface.mesh.triangles.clone())
        .surface
        .mesh;
    let bytes =
        compile_mesh_to_10d(&mesh).map_err(|error| format!("Cluster mesh encoding: {error:?}"))?;
    let entity = crate::asset_entity_iri(iri, &bytes)?;
    let hash = qualia_core_db::q_hash(&entity);
    let mut materials = source.surface.materials.clone();
    let mut ranges = source.surface.submeshes.clone();
    if ranges
        .iter()
        .any(|range| range.semantic_id != 0 && range.semantic_id != hash)
    {
        return Err(
            "Cluster mesh subparts require a source_10d with authored local CBLD terms".into(),
        );
    }
    for range in &mut ranges {
        range.semantic_id = hash;
    }
    materials.sort_by_key(|m| m.id);
    if materials.is_empty() != ranges.is_empty() || materials.iter().any(impostor::textured) {
        return Err(
            "Cluster mesh material bindings require declared untextured materials and ranges"
                .into(),
        );
    }
    let raw = if materials.is_empty() {
        bytes
    } else {
        ten::attach_material_section(
            &bytes,
            &materials,
            &ranges,
            (3 * mesh.triangles.len()) as u32,
        )
        .map_err(|error| format!("Cluster mesh material bindings: {error:?}"))?
    };
    let [x, y, z] = mesh.centroid();
    upgrade_legacy_10d_to_holographic_v2(
        &raw,
        &entity,
        1,
        [0.0, 3.0, 2.0, x, y, z, 0.0, 1.0, 1.0, 0.5],
    )
    .map_err(|error| format!("Cluster mesh semantic sealing: {error:?}"))
}

fn read_materials(
    bytes: &[u8],
    triangles: usize,
) -> Result<(Vec<MaterialRecord>, Vec<SubmeshRange>), String> {
    match ten::material_container_counts(bytes) {
        Ok((materials, ranges)) if materials <= 256 => {
            let mut materials = vec![MaterialRecord::legacy_default(); materials];
            let mut ranges = vec![
                SubmeshRange {
                    first_index: 0,
                    index_count: 0,
                    material_id: 0,
                    semantic_id: 0
                };
                ranges
            ];
            ten::read_material_section_into(
                bytes,
                (triangles * 3) as u32,
                &mut materials,
                &mut ranges,
            )
            .map_err(|error| format!("Cluster material bindings: {error:?}"))?;
            Ok((materials, ranges))
        }
        Err(ten::MaterialContainerError::Missing) => {
            Ok((vec![MaterialRecord::legacy_default()], Vec::new()))
        }
        Ok(_) => Err("CDAG admits up to 256 material slots".into()),
        Err(error) => Err(format!("Cluster material section: {error:?}")),
    }
}

fn bindings(
    materials: &[MaterialRecord],
    ranges: &[SubmeshRange],
    count: usize,
    root: u64,
) -> Result<(Vec<Binding>, BTreeMap<u32, u64>), String> {
    let mut slots = BTreeMap::new();
    let mut bound = vec![
        Binding {
            material_slot: 0,
            semantic_key: root as u32
        };
        count
    ];
    if ranges.is_empty() {
        let key = semantic_slot(&mut slots, root)?;
        bound.fill(Binding {
            material_slot: 0,
            semantic_key: key,
        });
    } else {
        let mut cursor = 0;
        for range in ranges {
            if range.first_index as usize != cursor * 3 {
                return Err("Cluster material ranges must cover the source mesh".into());
            }
            let end = cursor + (range.index_count / 3) as usize;
            let material_slot = materials
                .binary_search_by_key(&range.material_id, |m| m.id)
                .map_err(|_| "Cluster material range is missing its declared material")?;
            let bind = Binding {
                material_slot: material_slot as u8,
                semantic_key: semantic_slot(&mut slots, range.semantic_id)?,
            };
            bound[cursor..end].fill(bind);
            cursor = end;
        }
        if cursor != count {
            return Err("Cluster material ranges must cover the source mesh".into());
        }
    }
    Ok((bound, slots))
}

fn semantic_slot(slots: &mut BTreeMap<u32, u64>, hash: u64) -> Result<u32, String> {
    let key = hash as u32;
    if key == 0 || slots.insert(key, hash).is_some_and(|prior| prior != hash) {
        return Err(
            "CDAG legacy 32-bit subpart hash cannot uniquely represent its local 64-bit identities"
                .into(),
        );
    }
    Ok(key)
}

fn meshlets(
    mesh: &Mesh,
    slots: &[Binding],
    vertex_cap: usize,
    triangle_cap: usize,
) -> Result<Streams, String> {
    let mut streams = Streams {
        nodes: Vec::new(),
        positions: Vec::new(),
        triangles: Vec::new(),
    };
    let mut local = BTreeMap::new();
    let mut vertices = Vec::with_capacity(vertex_cap);
    let mut indices = Vec::with_capacity(triangle_cap * 3);
    let mut active = slots[0];
    for (&triangle, &slot) in mesh.triangles.iter().zip(slots) {
        let [a, b, c] = triangle.map(|i| mesh.positions.get(i as usize).copied());
        let (Some(a), Some(b), Some(c)) = (a, b, c) else {
            return Err("Cluster triangle is outside the source vertex stream".into());
        };
        face_normal([point(a), point(b), point(c)])?;
        let additional = triangle.iter().filter(|i| !local.contains_key(*i)).count();
        if !indices.is_empty()
            && (slot != active
                || vertices.len() + additional > vertex_cap
                || indices.len() == triangle_cap * 3)
        {
            write_meshlet(&mut streams, &vertices, &indices, active)?;
            vertices.clear();
            indices.clear();
            local.clear();
        }
        active = slot;
        for (index, p) in triangle.into_iter().zip([a, b, c]) {
            let local_id = *local.entry(index).or_insert_with(|| {
                let id = vertices.len() as u16;
                vertices.push(p);
                id
            });
            indices.push(local_id);
        }
    }
    write_meshlet(&mut streams, &vertices, &indices, active)?;
    Ok(streams)
}

fn write_meshlet(
    streams: &mut Streams,
    positions: &[[f32; 3]],
    indices: &[u16],
    bind: Binding,
) -> Result<(), String> {
    let sphere = enclosing_sphere(positions)?;
    streams.nodes.push(MeshletClusterNode {
        bounding_sphere: sphere,
        parent_error_sphere: sphere,
        lod_error_self: 0.0,
        lod_error_parent: f32::INFINITY,
        lod_level: 0,
        vertex_count: positions.len() as u8,
        triangle_count: (indices.len() / 3) as u8,
        material_slot: bind.material_slot,
        child_group_first: u16::MAX,
        vertex_data_offset: (streams.positions.len() / 12) as u32,
        triangle_data_offset: (streams.triangles.len() / 2) as u32,
        semantic_subpart_id: bind.semantic_key,
        ..MeshletClusterNode::zeroed()
    });
    for value in positions.iter().flatten() {
        streams.positions.extend(value.to_le_bytes());
    }
    for index in indices {
        streams.triangles.extend(index.to_le_bytes());
    }
    Ok(())
}

fn enclosing_sphere(positions: &[[f32; 3]]) -> Result<[f32; 4], String> {
    let mut low = [f32::INFINITY; 3];
    let mut high = [f32::NEG_INFINITY; 3];
    for p in positions {
        for i in 0..3 {
            low[i] = low[i].min(p[i]);
            high[i] = high[i].max(p[i]);
        }
    }
    let center = std::array::from_fn(|i| ((f64::from(low[i]) + f64::from(high[i])) * 0.5) as f32);
    let radius = positions
        .iter()
        .map(|&p| length(sub(point(p), point(center))))
        .fold(0.0, f64::max);
    if !radius.is_finite() || radius <= 0.0 || center.iter().any(|x| !x.is_finite()) {
        return Err("Meshlet bounding sphere is degenerate".into());
    }
    Ok([center[0], center[1], center[2], (radius as f32).next_up()])
}

fn validate_draw_streams(
    source: &[u8],
    mesh: &Mesh,
    streams: &Streams,
    limit: usize,
) -> Result<(), String> {
    let count = mesh.triangles.len();
    let n = streams.nodes.len();
    let mut plan = CapacityPlan::default();
    for bytes in [
        source.len(),
        streams.positions.len(),
        streams.triangles.len(),
        n * std::mem::size_of::<MeshletClusterNode>(),
    ] {
        plan.add_bytes(BufferClass::Input, bytes)
            .map_err(|error| error.to_string())?;
    }
    plan.add_slice::<ReplacementLink>(BufferClass::Scratch, n)
        .map_err(|error| error.to_string())?;
    plan.add_slice::<u32>(BufferClass::Scratch, n + count)
        .map_err(|error| error.to_string())?;
    plan.add_slice::<bool>(BufferClass::Scratch, count)
        .map_err(|error| error.to_string())?;
    plan.add_slice::<DrawIndexedCommand>(BufferClass::Output, n)
        .map_err(|error| error.to_string())?;
    let cancel = Cancellation::new();
    let mut budget = ProductionBudget::admit(plan, limit, (8 * (n + count)) as u64, &cancel)
        .map_err(|error| error.to_string())?;
    let roots = (0..n as u32).collect::<Vec<_>>();
    let source_order = (0..count as u32).collect::<Vec<_>>();
    let source_identity = ContentId::sha256(source);
    let no_border = ContentId::sha256(&[]);
    let mut coverage = 0;
    let links = streams
        .nodes
        .iter()
        .map(|node| {
            let coverage_count = u32::from(node.triangle_count);
            let link = ReplacementLink {
                parent: None,
                children_start: 0,
                children_count: 0,
                coverage_start: coverage,
                coverage_count,
                border_identity: no_border,
            };
            coverage += coverage_count;
            link
        })
        .collect::<Vec<_>>();
    let mut seen = vec![false; count];
    let hierarchy = ValidatedHierarchy::admit(
        &streams.nodes,
        &links,
        &[],
        &roots,
        source_identity,
        &source_order,
        &mut seen,
        &mut budget,
    )
    .map_err(|error| format!("Cluster root coverage: {error}"))?;
    hierarchy
        .validate_cut(&roots, &mut budget)
        .map_err(|error| format!("Cluster zero-error cut: {error}"))?;
    let mut commands = vec![DrawIndexedCommand::EMPTY; n];
    let report = generate_indexed_draws(&hierarchy, &roots, &roots, &mut commands, &mut budget)
        .map_err(|error| format!("Cluster native indexed draws: {error}"))?;
    if report.command_count != n {
        return Err("Native CDAG draw command count differs from admitted meshlets".into());
    }
    for (command, link) in commands.into_iter().zip(links) {
        for offset in 0..command.index_count as usize {
            let index = (command.first_index as usize + offset) * 2;
            let Some(&[lo, hi]) = streams.triangles.get(index..index + 2) else {
                return Err("Native CDAG draw index exceeds its companion stream".into());
            };
            let vertex = command.base_vertex as usize + usize::from(u16::from_le_bytes([lo, hi]));
            let face = mesh.triangles[link.coverage_start as usize + offset / 3];
            let expected = mesh.positions[face[offset % 3] as usize];
            let position = streams
                .positions
                .get(vertex * 12..(vertex + 1) * 12)
                .ok_or("Native CDAG draw vertex exceeds its companion stream")?;
            if !expected
                .into_iter()
                .zip(position.chunks_exact(4))
                .all(|(coordinate, bytes)| coordinate.to_le_bytes().as_slice() == bytes)
            {
                return Err("Native CDAG draw changes authored triangle vertex order".into());
            }
        }
    }
    Ok(())
}

fn append_sections(
    source: &[u8],
    header: &ten::Container10dHeader,
    table: &[ten::SectionDescriptor],
    cdag: &[u8],
    imp: Option<&[u8]>,
    limit: usize,
) -> Result<Vec<u8>, String> {
    let mut sections = Vec::with_capacity(table.len() + 2);
    for d in table {
        let start = d.byte_offset as usize;
        let end = start
            .checked_add(d.byte_length as usize)
            .ok_or("Cluster input section extent overflow")?;
        let payload = source
            .get(start..end)
            .ok_or("Cluster input section extent exceeds file")?;
        if ten::crc32c(payload) != d.crc32c {
            return Err("Cluster input section failed CRC-32C".into());
        }
        sections.push(ten::SectionInput {
            section_type: d.typ().ok_or("Cluster input section type is invalid")?,
            alignment_tier: d
                .tier()
                .ok_or("Cluster input section alignment is invalid")?,
            stride: d.stride,
            element_count: d.element_count,
            payload,
        });
    }
    for (kind, payload) in [
        (SectionType::ClusterDag, Some(cdag)),
        (SectionType::ImpostorAtlas, imp),
    ] {
        if let Some(payload) = payload {
            sections.push(ten::SectionInput {
                section_type: kind,
                alignment_tier: ten::AlignmentTier::Page,
                stride: 0,
                element_count: 0,
                payload,
            });
        }
    }
    let capacity = source
        .len()
        .checked_add(cdag.len())
        .and_then(|len| len.checked_add(imp.map_or(0, <[u8]>::len)))
        .and_then(|len| len.checked_add(4096))
        .ok_or("Cluster container length overflow")?
        .min(limit);
    let mut output = vec![0; capacity];
    let len = ten::encode_container(header, &sections, &mut output)
        .map_err(|error| format!("Cluster container encoding: {error:?}"))?;
    output.truncate(len);
    ten::seal_whole_file_crc32c(&mut output);
    let (_, spatial) = SemanticModelBuilder::new(limit)
        .merge_10d(&output)
        .map_err(|error| format!("Cluster output semantic bindings: {error}"))?;
    spatial.ok_or("Cluster output lost its spatial identity")?;
    Ok(output)
}
