//! Native SKL1 skin projection and attributed MRP1 surface binding.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::{BTreeMap, BTreeSet}, sync::{atomic::Ordering, Arc}};
use qualia_core_db::{
    bundle::semantic_model::{resource_identity_meta, SemanticModelBuilder},
    container_10d as ten,
    q_hash, NQuin,
    query::resolver::pack_typed_literal,
    render::{
        assets::ImportedMesh,
        portal_load_10d::decode_load_10d,
        skeletal::{valid_motor, Joint, Key, Motor, SkinWeight, Track, MAX_JOINTS},
        skeletal::semantic_motion::{MORPH_TARGET_GROUP_IRI, MORPH_TARGET_SLOT_IRI},
    },
    specialized_libs::computational_geometry::{heat_geodesic_distances, Point3},
};

use crate::{artifact::{AnimClip, AttributedMesh}, count, fields, integer, list, num, record, required, text, vector,
    Args, AssetHost, TypedArtifact, Value};
use crate::qualia_containers_governance_poet_ops::encoded_resource;
use ten::skeletal_section::{encode_skeletal_section_into, ClipRecord, Counts, SkeletalData,
    SkeletalSectionPayload, SKL2_MAGIC, SKL2_NONE_INDEX};

const MAX_MORPH_DELTAS: usize = 65_536;
const MAX_WEIGHT_VERTICES: u32 = 65_536;
const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const JOINT: &str = "https://qualiadb.org/ns/kinematics#Skl1Joint";
const CLIP: &str = "https://qualiadb.org/ns/kinematics#AnimationClip";
const SKIN_CBLD_CONTEXT: &str = "urn:qualia:10d:skin-bind:v1";

#[derive(Clone, Copy)]
enum Projection { Envelope, HeatDiffusion { iterations: u32 } }

struct Bind<'a> {
    iri: &'a str,
    world: Motor,
    position: [f64; 3],
    seed: Option<usize>,
}

impl AssetHost<'_> {
    pub(crate) fn bind_skin_and_morphs(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "rig", "binds", "method", "falloff_m", "heat_iterations", "actor_group_iri",
            "context_iri", "animation_clip", "clip_iri", "morph_targets"])?;
        let projection = match text(required(args, "method")?)? {
            "envelope" if !args.contains_key("heat_iterations") => Projection::Envelope,
            "heat_diffusion" => Projection::HeatDiffusion { iterations: count(args, "heat_iterations", 32, 128)? as u32 },
            _ => return Err("Skin method must be envelope or heat_diffusion".into()),
        };
        let falloff = num(required(args, "falloff_m")?)?;
        if falloff <= 0.0 { return Err("Skin falloff_m must be positive".into()); }
        let group = text(required(args, "actor_group_iri")?)?;
        let scope = text(required(args, "context_iri")?)?;
        let group_hash = iri_hash(group)?;
        iri_hash(scope)?;
        let rig_handle = required(args, "rig")?;
        let TypedArtifact::SkeletalRig(rig) = self.workspace.typed(rig_handle)?
            else { return Err("Skin binding requires the native typed SKL2 rig".into()); };
        let rig = Arc::clone(rig);
        rig.validate().map_err(|error| format!("Skin SKL2 rig: {error:?}"))?;
        let binds = list(required(args, "binds")?)?;
        if binds.len() != rig.joints.len() || !(1..=MAX_JOINTS).contains(&binds.len()) {
            return Err("Skin binding requires one ordered world pose per native SKL2 joint".into());
        }
        let binds = binds.iter().zip(&rig.joints).map(|(value, joint)| read_bind(value, joint.joint_id, projection))
            .collect::<Result<Vec<_>, _>>()?;
        let root = rig.joints.iter().position(|joint| joint.joint_id == group_hash && joint.parent_index == SKL2_NONE_INDEX)
            .ok_or("Skin actor group must be one of the admitted skeleton roots")?;
        if binds[root].iri != group { return Err("Skin actor group IRI collides with the authored root".into()); }

        let source = required(args, "source")?;
        let limit = self.limits.resource_bytes.min(ten::GRAPHICS_ASSET_RING_BYTES);
        let (source_len, vertex_count, triangle_count) = {
            let source = encoded_resource(self, source)?;
            if source.kind != "10d" || source.bytes.len() > limit { return Err("Skin source requires a sealed .10d within the graphics ring".into()); }
            let mut scratch = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
            let mut fields = scratch;
            let view = ten::TypedPayloadResolver::new(&source.bytes, &[], &mut scratch, &mut fields).map_err(|error| error.to_string())?;
            let mesh = view.section_payload(ten::SectionType::QuantizedMesh).ok_or("Skin source requires an authored quantized mesh")?;
            let (header, _) = ten::parse_mesh_header(mesh).map_err(|error| error.to_string())?;
            if header.vertex_count == 0 || header.vertex_count > MAX_WEIGHT_VERTICES || header.triangle_count == 0 {
                return Err("Skin binding exceeds its bounded surface vertex/triangle domain".into());
            }
            (source.bytes.len(), header.vertex_count as usize, header.triangle_count as usize)
        };
        let morph_count = morph_delta_count(args.get("morph_targets"))?;
        let projection_work = match projection {
            Projection::Envelope => vertex_count.saturating_mul(binds.len()).saturating_mul(16),
            Projection::HeatDiffusion { iterations } => vertex_count.saturating_mul(triangle_count)
                .saturating_mul(9).saturating_mul(binds.len()).saturating_mul(iterations as usize + 10),
        };
        self.charge(source_len.saturating_mul(3).saturating_add(projection_work).saturating_add(morph_count * 16),
            source_len.saturating_mul(3).saturating_add(vertex_count * 256)
                .saturating_add(triangle_count.saturating_mul(400)).saturating_add(morph_count * 64))?;

        let (artifact, encoded) = {
            let source = encoded_resource(self, source)?;
            let visual = decode_load_10d(&source.bytes).map_err(|error| format!("Skin source: {error}"))?;
            if visual.field_representation != "QuantizedMesh" || visual.governance_refused {
                return Err("Skin binding requires an admitted explicit quantized surface".into());
            }
            let header = ten::Container10dHeader::parse(&source.bytes).map_err(|error| error.to_string())?;
            if ten::compute_whole_file_crc32c(&source.bytes) != header.header_crc32c { return Err("Skin source checksum mismatch".into()); }
            let mut scratch = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
            let mut fields = scratch;
            let view = ten::TypedPayloadResolver::new(&source.bytes, &[], &mut scratch, &mut fields).map_err(|error| error.to_string())?;
            let payload = view.section_payload(ten::SectionType::QuantizedMesh).ok_or("Missing skin mesh")?;
            let mesh = ten::decode_mesh_section(payload).map_err(|error| format!("Skin coordinate frame: {error:?}"))?;
            let mut skeletal = Vec::with_capacity(binds.len());
            for (slot, bind) in binds.iter().enumerate() {
                let parent = rig.joints[slot].parent_index;
                let parent = (parent != SKL2_NONE_INDEX).then_some(parent);
                let rest = parent.map_or(bind.world, |p| binds[usize::from(p)].world.conjugate().mul(&bind.world));
                skeletal.push(Joint { parent, rest_local: rest.normalize(), inverse_bind: bind.world.conjugate() });
            }
            let weights = project_weights(self, &mesh.positions, &mesh.triangles, &binds, &skeletal, projection, falloff)?;
            let clip = optional_clip(self, args)?;
            let default_iri = format!("urn:qualia:skl1:{group_hash:016x}:bind-pose");
            let clip_iri = args.get("clip_iri").map(text).transpose()?.unwrap_or(&default_iri);
            if clip.is_some() && !args.contains_key("clip_iri") { return Err("An authored animation_clip requires its canonical clip_iri".into()); }
            let joint_ids = rig.joints.iter().map(|joint| joint.joint_id).collect::<Vec<_>>();
            let skl = encode_skin(&skeletal, &joint_ids, &weights, root, iri_hash(clip_iri)?, clip)?;
            let morph = parse_morph_targets(args.get("morph_targets"), vertex_count)?;
            let existing_morph = view.morph_targets().map_err(|error| error.to_string())?;
            if morph.is_some() && existing_morph.is_some() {
                return Err("Skin source already has a native MRP1 field".into());
            }
            let cbld = skin_semantics(&binds, group, scope, clip_iri, morph.as_ref())?;
            let encoded = reseal(&source.bytes, &header, &view, &rig, &skl, &cbld, morph.as_ref(), limit)?;
            let artifact = AttributedMesh {
                surface: ImportedMesh { mesh, normals: visual.normals, tangents: visual.tangents,
                    texture_coordinates_0: visual.uv0, materials: visual.materials,
                    texture_dependencies: Vec::new(), submeshes: visual.ranges },
                uv1: Vec::new(), vertex_colors: visual.colors, skin_weights: weights,
                morph_targets: morph.as_ref().map(|m| Arc::from(m.bytes.as_slice()))
                    .or_else(|| existing_morph.map(|m| Arc::from(m.bytes))).unwrap_or_else(|| Arc::from([])),
                protected_faces: Vec::new(),
            };
            (artifact, encoded)
        };
        let mesh = self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(artifact)))?;
        let container = self.admit_encoded("10d", encoded)?;
        Ok(Value::Record(Args::from([
            ("mesh".into(), mesh), ("container_10d".into(), container), ("rig".into(), rig_handle.clone()),
        ])))
    }
}

fn iri_hash(iri: &str) -> Result<u64, String> {
    if iri.len() > 4096 { return Err("Skin RDF identity exceeds its lexical bound".into()); }
    resource_identity_meta(iri).map_err(|error| error.to_string())?;
    Ok(q_hash(iri))
}

fn read_bind(value: &Value, expected_id: u64, projection: Projection) -> Result<Bind<'_>, String> {
    let row = record(value)?;
    fields(row, &["iri", "world_position_m", "world_rotor_wxyz", "seed_vertex"])?;
    let iri = text(required(row, "iri")?)?;
    if iri_hash(iri)? != expected_id { return Err("Skin bind order or identity disagrees with SKL2".into()); }
    let position: [f64; 3] = vector(required(row, "world_position_m")?)?;
    let rotation: [f64; 4] = vector(required(row, "world_rotor_wxyz")?)?;
    if (rotation.iter().map(|v| v * v).sum::<f64>() - 1.0).abs() > 1e-6 {
        return Err("Skin world bind rotor must have unit length".into());
    }
    let world = Motor::from_rotation_translation(rotation, position);
    if !valid_motor(&world) { return Err("Skin world bind motor is invalid".into()); }
    let seed = row.get("seed_vertex").map(|value| integer(value, "seed_vertex")).transpose()?;
    if matches!(projection, Projection::Envelope) && seed.is_some() {
        return Err("Only heat diffusion takes an authored surface seed".into());
    }
    Ok(Bind { iri, world, position, seed })
}

fn optional_clip<'a>(host: &'a AssetHost<'_>, args: &Args) -> Result<Option<&'a AnimClip>, String> {
    let Some(value) = args.get("animation_clip") else { return Ok(None); };
    let TypedArtifact::AnimationClip(clip) = host.workspace.typed(value)? else {
        return Err("Skin animation_clip requires the typed native animation handle".into());
    };
    if !clip.morph_keys.is_empty() { return Err("SKL1 clips carry motor tracks; corrective weights remain in the Q42 macro".into()); }
    Ok(Some(clip.as_ref()))
}

fn encode_skin(
    joints: &[Joint], ids: &[u64], weights: &[SkinWeight], root: usize, clip_id: u64, animation: Option<&AnimClip>,
) -> Result<Vec<u8>, String> {
    let rest_track = [Track { joint: root as u16, first_key: 0, key_count: 1 }];
    let rest_key = [Key { time: 0.0, local: joints[root].rest_local }];
    let (duration, looping, tracks, keys) = if let Some(animation) = animation {
        (animation.duration_seconds, animation.looping, animation.tracks.as_slice(), animation.motor_keys.as_slice())
    } else { (1.0, false, &rest_track[..], &rest_key[..]) };
    let clips = [ClipRecord { id: clip_id, duration, first_track: 0, track_count: tracks.len() as u32, looping }];
    let size = Counts { joints: joints.len(), vertices: weights.len(), clips: 1, tracks: tracks.len(), keys: keys.len() }
        .encoded_len().ok_or("Skin animation exceeds native SKL1 counts")?;
    let mut bytes = vec![0; size];
    encode_skeletal_section_into(&SkeletalData { joints, joint_ids: ids, skin: weights, clips: &clips, tracks, keys }, &mut bytes)
        .map_err(|error| format!("Native DQS skin encoding: {error:?}"))?;
    Ok(bytes)
}

fn project_weights(
    host: &AssetHost<'_>, positions: &[[f32; 3]], triangles: &[[u32; 3]], binds: &[Bind<'_>], joints: &[Joint],
    projection: Projection, falloff: f64,
) -> Result<Vec<SkinWeight>, String> {
    let mut sorted = vec![[None; 4]; positions.len()];
    let points = positions.iter().map(|p| p.map(f64::from)).collect::<Vec<_>>();
    if points.iter().flatten().any(|v| !v.is_finite()) { return Err("Skin source contains nonfinite coordinates".into()); }
    let components = if matches!(projection, Projection::HeatDiffusion { .. }) { components(positions.len(), triangles)? } else { Vec::new() };
    let heat_points = if components.is_empty() { Vec::new() }
        else { points.iter().map(|p| Point3::new(p[0], p[1], p[2])).collect::<Vec<_>>() };
    let mut distances = vec![0.0; positions.len()];
    for (joint, bind) in binds.iter().enumerate() {
        if host.cancel.load(Ordering::Acquire) { return Err("Production cancelled".into()); }
        match projection {
            Projection::Envelope => {
                let parent = joints[joint].parent.map_or(bind.position, |slot| binds[usize::from(slot)].position);
                for (dst, &point) in distances.iter_mut().zip(&points) {
                    *dst = segment_distance(point, parent, bind.position);
                }
            }
            Projection::HeatDiffusion { iterations } => {
                let seed = match bind.seed {
                    Some(seed) if seed < points.len() => seed,
                    Some(_) => return Err("Authored heat seed vertex is out of bounds".into()),
                    None => points.iter().enumerate().min_by(|(_, a), (_, b)| {
                        distance_squared(**a, bind.position).total_cmp(&distance_squared(**b, bind.position))
                    }).ok_or("Empty heat source")?.0,
                };
                heat_geodesic_distances(&heat_points, triangles, seed, 0.05, iterations, &mut distances)
                    .map_err(|error| format!("Native skin heat geodesic at joint {joint}: {error:?}"))?;
                let offset = distance_squared(points[seed], bind.position).sqrt();
                for (i, dist) in distances.iter_mut().enumerate() {
                    if components[i] != components[seed] { *dist = f64::INFINITY; }
                    else if !dist.is_finite() || *dist < 0.0 { return Err(format!("Nonfinite or negative heat distance at vertex {i}")); }
                    else { *dist += offset; }
                }
            }
        }
        for (entries, d) in sorted.iter_mut().zip(&distances) {
            if d.is_finite() {
                if let Some(slot) = entries.iter().position(|entry: &Option<(f64, u16)>| entry.is_none_or(|row| *d < row.0)) {
                    entries.copy_within(slot..3, slot + 1);
                    entries[slot] = Some((*d, joint as u16));
                }
            }
        }
    }
    sorted.into_iter().enumerate().map(|(vertex, entries)| {
        let (nearest, _) = entries[0].ok_or_else(|| format!("No authored skin joint reaches vertex {vertex}"))?;
        let mut joints = [0; 4];
        let mut weights = [0.0; 4];
        for (slot, (distance, joint)) in entries.into_iter().flatten().enumerate() {
            joints[slot] = joint;
            weights[slot] = (-(distance - nearest) / falloff).exp();
        }
        let total = weights.iter().sum::<f64>();
        Ok(SkinWeight { joints, weights: weights.map(|weight| (weight / total) as f32) })
    }).collect()
}

fn components(count: usize, triangles: &[[u32; 3]]) -> Result<Vec<usize>, String> {
    fn root(parent: &mut [usize], mut i: usize) -> usize {
        while parent[i] != i {
            parent[i] = parent[parent[i]];
            i = parent[i];
        }
        i
    }
    let mut parents = (0..count).collect::<Vec<_>>();
    for &[a, b, c] in triangles {
        if [a, b, c].iter().any(|&v| v as usize >= count) { return Err("Skin graph index is out of bounds".into()); }
        let a = root(&mut parents, a as usize);
        for index in [b, c] {
            let other = root(&mut parents, index as usize);
            parents[other] = a;
        }
    }
    Ok((0..count).map(|i| root(&mut parents, i)).collect())
}

fn distance_squared(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| (a - b).powi(2)).sum()
}

fn segment_distance(point: [f64; 3], a: [f64; 3], b: [f64; 3]) -> f64 {
    let line: [f64; 3] = std::array::from_fn(|i| b[i] - a[i]);
    let length_squared = line.iter().map(|x| x * x).sum::<f64>();
    if length_squared == 0.0 { return distance_squared(point, a).sqrt(); }
    let t = ((0..3).map(|i| (point[i] - a[i]) * line[i]).sum::<f64>() / length_squared).clamp(0.0, 1.0);
    distance_squared(point, std::array::from_fn(|i| a[i] + t * line[i])).sqrt()
}

struct MorphTargets<'a> {
    bytes: Vec<u8>,
    iris: Vec<&'a str>,
}

fn morph_delta_count(input: Option<&Value>) -> Result<usize, String> {
    let Some(input) = input else { return Ok(0); };
    let targets = list(input)?;
    if !(1..=ten::MAX_MORPH_TARGETS).contains(&targets.len()) { return Err("Native MRP1 target count exceeded".into()); }
    targets.iter().try_fold(0usize, |total, target| {
        let rows = list(required(record(target)?, "deltas")?)?;
        if rows.is_empty() { return Err("Each authored morph target requires sparse deltas".into()); }
        total.checked_add(rows.len()).filter(|&n| n <= MAX_MORPH_DELTAS)
            .ok_or_else(|| "Native MRP1 sparse delta budget exceeded".into())
    })
}

fn parse_morph_targets(input: Option<&Value>, vertices: usize) -> Result<Option<MorphTargets<'_>>, String> {
    let Some(input) = input else { return Ok(None); };
    let mut iris = Vec::new();
    let mut identity = BTreeSet::new();
    let mut values = Vec::with_capacity(morph_delta_count(Some(input))?);
    let mut displacement = 0.0_f32;
    let mut flags = ten::FLAG_MORPH_SPARSE;
    for (slot, target) in list(input)?.iter().enumerate() {
        let target = record(target)?;
        fields(target, &["iri", "deltas"])?;
        let iri = text(required(target, "iri")?)?;
        if !identity.insert(iri_hash(iri)?) { return Err("Repeated native MRP1 semantic identity".into()); }
        iris.push(iri);
        let mut seen = BTreeSet::new();
        for raw in list(required(target, "deltas")?)? {
            let raw = record(raw)?;
            fields(raw, &["vertex", "position_m", "normal_delta"])?;
            let vertex = integer(required(raw, "vertex")?, "morph vertex")?;
            if vertex >= vertices || !seen.insert(vertex) { return Err("Invalid or duplicate sparse MRP1 vertex".into()); }
            let position: [f64; 3] = vector(required(raw, "position_m")?)?;
            let position = position.map(|value| value as f32);
            if position.iter().any(|value| !value.is_finite()) { return Err("Nonfinite native MRP1 displacement".into()); }
            let normal = if raw.contains_key("normal_delta") {
                flags |= ten::FLAG_MORPH_HAS_NORMALS;
                let normal: [f64; 3] = vector(required(raw, "normal_delta")?)?;
                if normal.iter().any(|value| !(-2.0..=2.0).contains(value)) { return Err("MRP1 normal delta exceeds native range [-2,2]".into()); }
                normal.map(|value| value as f32)
            } else { [0.0; 3] };
            displacement = position.iter().fold(displacement, |max, value| max.max(value.abs()));
            values.push((slot as u8, vertex as u32, position, normal));
        }
    }
    values.sort_unstable_by_key(|(slot, vertex, ..)| (*slot, *vertex));
    let mut populated = [false; ten::MAX_MORPH_TARGETS];
    let deltas = values.into_iter().filter_map(|(slot, vertex, position, normal)| {
        let delta = ten::quantize_morph_delta(position, normal, [0.0; 3], displacement, slot, vertex);
        if delta.pos_delta == [0; 3] && delta.norm_delta == [0; 3] { None } else {
            populated[usize::from(slot)] = true;
            Some(delta)
        }
    }).collect::<Vec<_>>();
    if populated[..iris.len()].iter().any(|&populated| !populated) {
        return Err("An authored morph target vanishes at native MRP1 quantization".into());
    }
    let header = ten::MorphSectionHeader { magic: ten::MAGIC_MRP1, version: ten::MORPH_SECTION_VERSION,
        target_count: iris.len() as u16, vertex_count: vertices as u32, flags,
        max_displacement: displacement, ontogenetic_bind_mask: 0, reserved: [0; 8] };
    let size = ten::morph_section_encoded_len(deltas.len()).ok_or("MRP1 payload extent is invalid")?;
    let mut bytes = vec![0; size];
    ten::encode_morph_section(&header, &deltas, &mut bytes).map_err(|error| error.to_string())?;
    Ok(Some(MorphTargets { bytes, iris }))
}

fn skin_semantics(
    binds: &[Bind<'_>], group: &str, scope: &str, clip: &str,
    morph: Option<&MorphTargets<'_>>,
) -> Result<Vec<u8>, String> {
    let mut lexicon = BTreeMap::new();
    for iri in [TYPE, JOINT, CLIP, group, scope, clip, MORPH_TARGET_SLOT_IRI, MORPH_TARGET_GROUP_IRI]
        .into_iter().chain(binds.iter().map(|joint| joint.iri)).chain(morph.into_iter().flat_map(|m| m.iris.iter().copied()))
    {
        let term_hash = iri_hash(iri)?;
        if lexicon.insert(term_hash, iri).is_some_and(|previous| previous != iri) {
            return Err("Skin CBLD has conflicting q_hash identities".into());
        }
    }
    let mut payload = ten::SymbolicRdfCborLdPayload {
        context_iri: SKIN_CBLD_CONTEXT.into(),
        lexicon: lexicon.into_iter().map(|(term_hash, iri)| ten::MultilingualLexicalTerm {
            term_hash,
            iri_utf8: iri.into(),
            lang_bcp47: None,
            label_utf8: String::new(),
        }).collect(),
        ..Default::default()
    };
    let context = q_hash(scope);
    let mut add = |s, p, o| payload.nquins.push([s, p, o, context, 0, NQuin::calculate_parity(s, p, o, context, 0)]);
    for bind in binds { add(q_hash(bind.iri), q_hash(TYPE), q_hash(JOINT)); }
    add(q_hash(clip), q_hash(TYPE), q_hash(CLIP));
    if let Some(morph) = morph {
        for (slot, iri) in morph.iris.iter().enumerate() {
            let slot = pack_typed_literal(&slot.to_string(), "xsd:integer").ok_or("MRP1 semantic slot cannot be encoded")?;
            add(q_hash(iri), q_hash(MORPH_TARGET_SLOT_IRI), slot);
            add(q_hash(iri), q_hash(MORPH_TARGET_GROUP_IRI), q_hash(group));
        }
    }
    payload.nquins.sort_unstable();
    payload.nquins.dedup();
    ten::encode_symbolic_rdf_cbor_ld(&payload).map_err(|error| error.to_string())
}

fn reseal(
    original: &[u8], header: &ten::Container10dHeader, view: &ten::TypedPayloadResolver<'_, '_>,
    rig: &SkeletalSectionPayload, skl: &[u8], cbld: &[u8], morph: Option<&MorphTargets<'_>>, limit: usize,
) -> Result<Vec<u8>, String> {
    let identity = view.identity().ok_or("Skin source lacks an admitted MID2 identity")?;
    let mut fields = view.fields().to_vec();
    if fields.iter().any(|field| field.payload_section_id == ten::SectionType::SkeletalAnimation as u32) {
        return Err("Skin source has typed offsets into its skeletal section".into());
    }
    let mut sidecar = view.section_payload(ten::SectionType::FieldSidecar).unwrap_or_default().to_vec();
    // Keep the existing MID2 records and sidecar prefix intact, including HOLO addresses.
    let additions = std::iter::once((ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD, cbld))
        .chain(morph.map(|morph| (ten::FIELD_KIND_MORPH_TARGETS, morph.bytes.as_slice())));
    for (kind, payload) in additions {
        let field_id = fields.last().map_or(0, |field| field.field_id)
            .checked_add(1).ok_or("No remaining MID2 field IDs")?;
        fields.push(ten::TypedFieldRecord {
            field_id,
            kind,
            payload_section_id: ten::SectionType::FieldSidecar as u32,
            payload_offset: sidecar.len() as u64,
            payload_length: payload.len() as u64,
            ..Default::default()
        });
        sidecar.extend_from_slice(payload);
    }
    let size = ten::manifold_identity_v2_encoded_len(fields.len()).map_err(|error| format!("Skin MID2 extent: {error:?}"))?;
    let mut mid2 = vec![0; size];
    ten::encode_manifold_identity_v2(identity, &fields, &mut mid2).map_err(|error| format!("Skin MID2: {error:?}"))?;
    let mut sections = Vec::new();
    let mut hi = ten::AlignmentTier::Page;
    let mut si = ten::AlignmentTier::Page;
    for descriptor in ten::parse_section_table(original, header).map_err(|error| error.to_string())? {
        let section_type = descriptor.typ().ok_or("Unknown skin source section")?;
        let alignment_tier = descriptor.tier().ok_or("Invalid skin source alignment")?;
        let payload = view.section_payload(section_type).ok_or("Missing skin source section")?;
        match section_type {
            ten::SectionType::ManifoldIdentityV2 => { hi = alignment_tier; }
            ten::SectionType::FieldSidecar => { si = alignment_tier; }
            ten::SectionType::SkeletalAnimation => {
                if !payload.starts_with(&SKL2_MAGIC) { return Err("Skin source already contains an authored SKL1 clip".into()); }
                let embedded = SkeletalSectionPayload::from_bytes(payload).map_err(|error| format!("Source SKL2: {error:?}"))?;
                if &embedded != rig { return Err("Embedded SKL2 disagrees with the selected typed rig".into()); }
            }
            _ => sections.push(ten::SectionInput { section_type, alignment_tier, stride: descriptor.stride,
                element_count: descriptor.element_count, payload }),
        }
    }
    sections.extend([
        ten::SectionInput { section_type: ten::SectionType::ManifoldIdentityV2, alignment_tier: hi, stride: 0, element_count: 0, payload: &mid2 },
        ten::SectionInput { section_type: ten::SectionType::FieldSidecar, alignment_tier: si, stride: 0, element_count: 0, payload: &sidecar },
        ten::SectionInput { section_type: ten::SectionType::SkeletalAnimation, alignment_tier: ten::AlignmentTier::Word,
            stride: 0, element_count: 0, payload: skl },
    ]);
    let needed = match ten::encode_container(header, &sections, &mut []) {
        Err(ten::SectionTableError::OutputBufferTooSmall { needed, .. }) if needed <= limit => needed,
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Err("Skin container did not report its output size".into()),
    };
    let mut output = vec![0; needed];
    let actual = ten::encode_container(header, &sections, &mut output).map_err(|error| error.to_string())?;
    output.truncate(actual);
    ten::seal_whole_file_crc32c(&mut output);
    ten::skeletal_container::read_skeletal_section(&output).map_err(|error| format!("Read-back SKL1: {error:?}"))?;
    SemanticModelBuilder::new(42 * 1024 * 1024).merge_10d(&output)
        .and_then(|(model, _)| model.finish())
        .map_err(|error| format!("Skinned 10D semantic admission: {error}"))?;
    Ok(output)
}
