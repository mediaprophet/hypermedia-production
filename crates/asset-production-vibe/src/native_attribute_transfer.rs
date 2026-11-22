//! Typed surface attribute transfer across an authored triangle correspondence.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use crate::artifact::AttributedMesh;
use crate::*;
use qualia_core_db::container_10d::morph_section::{
    MORPH_SECTION_HEADER_SIZE, QUANTIZED_MORPH_DELTA_SIZE, QuantizedMorphDelta,
    decode_morph_section_into, encode_morph_section, morph_section_encoded_len,
};
use qualia_core_db::render::skeletal::SkinWeight;
use qualia_core_db::specialized_libs::computational_geometry::{
    SparseEntry,
    attribute_transfer::{ProtectedDomain, ScalarRule, SparseTransfer, TransferIdentities},
    geometry_workspace::{
        Cancellation,
        production_budget::{CapacityPlan, ProductionBudget},
    },
    production_contracts::{ContentId, EntityDomain},
};
use sha2::{Digest, Sha256};

impl AssetHost<'_> {
    pub(crate) fn transfer_attributes(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "target", "bindings", "channels"])?;
        let source_key = required(args, "source")?;
        let target_key = required(args, "target")?;
        let bindings = list(required(args, "bindings")?)?;
        let channels = list(required(args, "channels")?)?
            .iter()
            .map(text)
            .collect::<Result<std::collections::BTreeSet<_>, _>>()?;
        if channels.is_empty()
            || !channels
                .iter()
                .all(|name| matches!(*name, "uv" | "normal" | "color" | "skin" | "morph"))
        {
            return Err("Select UV, normal, color, skin, or morph channels".into());
        }
        let source = attributed(self, source_key)?;
        let target = attributed(self, target_key)?;
        let input_count = source.surface.mesh.positions.len();
        let output_count = target.surface.mesh.positions.len();
        if input_count == 0 || output_count == 0 || bindings.len() != output_count {
            return Err(
                "Attribute transfer needs one source-triangle binding per target vertex".into(),
            );
        }
        let source_faces = source.surface.mesh.triangles.len();
        let target_faces = target.surface.mesh.triangles.len();
        let morph_targets = if channels.contains("morph") {
            let header = source
                .morph_targets
                .get(..MORPH_SECTION_HEADER_SIZE)
                .ok_or("Source has no MRP1 morph header")?;
            bytemuck::pod_read_unaligned::<
                qualia_core_db::container_10d::morph_section::MorphSectionHeader,
            >(header)
            .target_count as usize
        } else {
            0
        };
        let lanes = input_count + output_count;
        let morph_lanes = lanes.saturating_mul(morph_targets);
        let work = lanes
            .saturating_mul(256)
            .saturating_add((source_faces + target_faces).saturating_mul(24))
            .saturating_add(morph_lanes.saturating_mul(32));
        let scratch = lanes
            .saturating_mul(560)
            .saturating_add(morph_lanes.saturating_mul(24));
        self.charge(work, scratch)?;
        let source = attributed(self, source_key)?;
        let target = attributed(self, target_key)?;
        let source_domains = domains(source)?;
        let target_domains = domains(target)?;
        let mut entries = Vec::with_capacity(output_count * 3);
        for (index, binding) in bindings.iter().enumerate() {
            let row = record(binding)?;
            fields(row, &["source_triangle", "barycentric"])?;
            let face = integer(required(row, "source_triangle")?, "source_triangle")?;
            let triangle = source
                .surface
                .mesh
                .triangles
                .get(face)
                .ok_or("Source triangle out of range")?;
            let weights: [f64; 3] = vector(required(row, "barycentric")?)?;
            if weights.iter().any(|&weight| !(0.0..=1.0).contains(&weight))
                || (weights.iter().sum::<f64>() - 1.0).abs() > 1e-9
            {
                return Err(format!(
                    "Invalid barycentric partition at target vertex {index}"
                ));
            }
            let mut terms = [
                (triangle[0], weights[0]),
                (triangle[1], weights[1]),
                (triangle[2], weights[2]),
            ];
            terms.sort_unstable_by_key(|&(vertex, _)| vertex);
            for (vertex, weight) in terms {
                if weight > 0.0 {
                    entries.push(SparseEntry {
                        row: index as u32,
                        col: vertex,
                        value: weight,
                    });
                }
            }
        }
        let frame = ContentId::sha256(b"hypermedia:common-transfer-frame");
        let identities = TransferIdentities {
            source: mesh_identity(source),
            target: mesh_identity(target),
            source_frame: frame,
            target_frame: frame,
        };
        let cancellation = Cancellation::new();
        let mut budget =
            ProductionBudget::admit(CapacityPlan::default(), scratch, work as u64, &cancellation)
                .map_err(|err| err.to_string())?;
        let map = SparseTransfer::new(
            identities,
            identities,
            EntityDomain::Vertex,
            ScalarRule::Intensive,
            &entries,
            &source_domains,
            &target_domains,
            &mut vec![0.0; input_count],
            1e-9,
            &mut budget,
        )
        .map_err(|err| format!("Attribute transfer map: {err:?}"))?;

        let mut output = target.clone();
        if channels.contains("uv") {
            let uv0 = source
                .surface
                .texture_coordinates_0
                .as_ref()
                .ok_or("Source has no UV0 stream")?;
            output.surface.texture_coordinates_0 = Some(interpolate(&map, uv0, &mut budget)?);
            if !source.uv1.is_empty() {
                output.uv1 = interpolate(&map, &source.uv1, &mut budget)?;
            }
            output.surface.tangents = None;
        }
        if channels.contains("normal") {
            let normals = source
                .surface
                .normals
                .as_ref()
                .ok_or("Source has no normal stream")?;
            let mut normals = interpolate(&map, normals, &mut budget)?;
            for normal in &mut normals {
                let length = normal[0].hypot(normal[1]).hypot(normal[2]);
                if !length.is_finite() || length <= 1e-12 {
                    return Err("Interpolated normal has no direction".into());
                }
                *normal = normal.map(|v| v / length);
            }
            output.surface.normals = Some(normals);
            output.surface.tangents = None;
        }
        if channels.contains("color") {
            if source
                .vertex_colors
                .iter()
                .flatten()
                .any(|v| !(0.0..=1.0).contains(v))
            {
                return Err("Source color channels must be normalized RGBA".into());
            }
            output.vertex_colors = interpolate(&map, &source.vertex_colors, &mut budget)?;
        }
        if channels.contains("skin") {
            output.skin_weights = transfer_skin(&map, &source.skin_weights)?;
        }
        if channels.contains("morph") {
            output.morph_targets =
                Arc::from(transfer_morphs(&map, &source.morph_targets, &mut budget)?);
        }
        self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(output)))
    }
}

fn attributed<'a>(host: &'a AssetHost<'_>, handle: &Value) -> Result<&'a AttributedMesh, String> {
    match host.workspace.typed(handle)? {
        TypedArtifact::AttributedMesh(mesh) => Ok(mesh),
        _ => Err("Attribute transfer requires typed AttributedMesh handles".into()),
    }
}

fn domains(mesh: &AttributedMesh) -> Result<Vec<ProtectedDomain>, String> {
    let surface = &mesh.surface;
    let faces = &surface.mesh.triangles;
    if !mesh.protected_faces.is_empty() && mesh.protected_faces.len() != faces.len() {
        return Err("Invalid protected-face mask extent".into());
    }
    let mut face_domains = vec![ProtectedDomain::default(); faces.len()];
    let mut end = 0usize;
    for band in &surface.submeshes {
        if band.first_index % 3 != 0 || band.index_count % 3 != 0 {
            return Err("Submesh material band splits a triangle".into());
        }
        let first = band.first_index as usize / 3;
        let next = first + band.index_count as usize / 3;
        if first < end || next > faces.len() {
            return Err("Overlapping or out-of-range submesh material bands".into());
        }
        for item in &mut face_domains[first..next] {
            item.material = band.material_id;
            item.subpart = band.semantic_id;
        }
        end = next;
    }
    let mut domains = vec![None; surface.mesh.positions.len()];
    for (index, face) in faces.iter().enumerate() {
        let mut domain = face_domains[index];
        domain.protected = u64::from(mesh.protected_faces.get(index).copied().unwrap_or(0));
        for &vertex in face {
            let slot = domains
                .get_mut(vertex as usize)
                .ok_or("Triangle vertex out of range")?;
            if slot.is_some_and(|previous| previous != domain) {
                return Err(
                    "Split material, semantic, or protected boundaries before vertex transfer"
                        .into(),
                );
            }
            *slot = Some(domain);
        }
    }
    Ok(domains.into_iter().map(Option::unwrap_or_default).collect())
}

fn mesh_identity(mesh: &AttributedMesh) -> ContentId {
    let geometry = &mesh.surface.mesh;
    let mut digest = Sha256::new();
    digest.update((geometry.positions.len() as u64).to_le_bytes());
    for point in &geometry.positions {
        for value in point {
            digest.update(value.to_le_bytes());
        }
    }
    digest.update((geometry.triangles.len() as u64).to_le_bytes());
    for triangle in &geometry.triangles {
        for vertex in triangle {
            digest.update(vertex.to_le_bytes());
        }
    }
    ContentId::sha256(&digest.finalize())
}

fn interpolate<const N: usize>(
    map: &SparseTransfer<'_>,
    values: &[[f32; N]],
    budget: &mut ProductionBudget<'_>,
) -> Result<Vec<[f32; N]>, String> {
    let source: Vec<_> = values.iter().map(|value| value.map(f64::from)).collect();
    let mut result = vec![[0.0; N]; map.target_count()];
    map.apply(
        &source,
        &mut result,
        &mut vec![[0.0; N]; map.target_count()],
        budget,
    )
    .map_err(|err| format!("Attribute interpolation: {err:?}"))?;
    Ok(result
        .into_iter()
        .map(|value| value.map(|x| x as f32))
        .collect())
}

fn transfer_skin(
    map: &SparseTransfer<'_>,
    source: &[SkinWeight],
) -> Result<Vec<SkinWeight>, String> {
    if source.len() != map.source_count()
        || source.iter().any(|weights| {
            weights.weights.iter().any(|&v| !(0.0..=1.0).contains(&v))
                || (weights.weights.iter().sum::<f32>() - 1.0).abs() > 1e-4
        })
    {
        return Err("Source skin stream needs normalized weights for every vertex".into());
    }
    let mut output = Vec::with_capacity(map.target_count());
    for row in map.entries().chunk_by(|a, b| a.row == b.row) {
        let mut influences = BTreeMap::<u16, f64>::new();
        for binding in row {
            let weights = &source[binding.col as usize];
            for (&joint, &weight) in weights.joints.iter().zip(&weights.weights) {
                *influences.entry(joint).or_default() += binding.value * f64::from(weight);
            }
        }
        let mut ranked: Vec<_> = influences.into_iter().collect();
        ranked.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        ranked.truncate(4);
        let sum: f64 = ranked.iter().map(|entry| entry.1).sum();
        let mut influence = SkinWeight {
            joints: [0; 4],
            weights: [0.0; 4],
        };
        for (lane, (joint, weight)) in ranked.into_iter().enumerate() {
            influence.joints[lane] = joint;
            influence.weights[lane] = (weight / sum) as f32;
        }
        output.push(influence);
    }
    Ok(output)
}

fn transfer_morphs(
    map: &SparseTransfer<'_>,
    bytes: &[u8],
    budget: &mut ProductionBudget<'_>,
) -> Result<Vec<u8>, String> {
    use bytemuck::Zeroable;

    let count = bytes.len().saturating_sub(MORPH_SECTION_HEADER_SIZE) / QUANTIZED_MORPH_DELTA_SIZE;
    let mut deltas = vec![QuantizedMorphDelta::zeroed(); count];
    let (mut header, _) =
        decode_morph_section_into(bytes, &mut deltas).map_err(|err| err.to_string())?;
    if header.vertex_count as usize != map.source_count() {
        return Err("MRP1 vertex count differs from the transfer source".into());
    }
    deltas.sort_unstable_by_key(|delta| (delta.target_index, delta.vertex_index));
    if deltas.windows(2).any(|pair| {
        (pair[0].target_index, pair[0].vertex_index) == (pair[1].target_index, pair[1].vertex_index)
    }) {
        return Err("MRP1 input contains duplicate deltas for a target vertex".into());
    }
    let mut output = Vec::new();
    for group in deltas.chunk_by(|a, b| a.target_index == b.target_index) {
        let target_index = group[0].target_index;
        let mut source = vec![[0.0_f32; 8]; map.source_count()];
        for delta in group {
            let values = &mut source[delta.vertex_index as usize];
            for lane in 0..3 {
                values[lane] = f32::from(delta.pos_delta[lane]);
                values[3 + lane] = f32::from(delta.norm_delta[lane]);
            }
            values[6] = f32::from(delta.tangent_oct_delta[0]);
            values[7] = f32::from(delta.tangent_oct_delta[1]);
        }
        let target = interpolate(map, &source, budget)?;
        for (vertex, values) in target.into_iter().enumerate() {
            let pos_delta = std::array::from_fn(|i| values[i].round() as i16);
            let norm_delta = std::array::from_fn(|i| values[i + 3].round() as i8);
            let tangent_oct_delta = std::array::from_fn(|i| values[i + 6].round() as i8);
            if pos_delta != [0; 3] || norm_delta != [0; 3] || tangent_oct_delta != [0; 2] {
                output.push(QuantizedMorphDelta {
                    pos_delta,
                    norm_delta,
                    target_index,
                    tangent_oct_delta,
                    vertex_index: vertex as u32,
                });
            }
        }
    }
    header.vertex_count = map.target_count() as u32;
    let length =
        morph_section_encoded_len(output.len()).ok_or("Transferred MRP1 exceeds record limit")?;
    let mut bytes = vec![0; length];
    encode_morph_section(&header, &output, &mut bytes).map_err(|err| err.to_string())?;
    Ok(bytes)
}
