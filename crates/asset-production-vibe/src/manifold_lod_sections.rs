//! Authored mesh cluster to native SURF, with a bounded diagnostic turntable.

use crate::{
    artifact::{LatticeSamples, RadianceLattice}, fields, integer, list, num, record, required,
    vector, Args, AssetHost, Resource, TypedArtifact, Value,
};
use qualia_core_db::{
    bundle::entity_verifier::verify_entity_10d,
    container_10d::{
        self as ten,
        header::HEADER_BYTE_SIZE,
        holographic_phase_section::{
            decode_holographic_phase_section_into, EulerianWavefieldRecord, HolographicPhaseHeader,
            HOLOGRAPHIC_PHASE_HEADER_SIZE, MAX_EULERIAN_WAVEFIELDS, MAX_HOLO_SEGMENTS,
        },
        impostor_section::ImpostorAtlasMiniHeader,
        integrity::check_whole_file_crc32c,
        section::SECTION_DESCRIPTOR_SIZE,
        surfel_section::{
            decode_octahedral_normal, decode_surfel_section, encode_surfel_section,
            generate_surfel_from_cluster, SurfelBinaryRecord, SurfelSectionMiniHeader,
            FLAG_SURFEL_HIERARCHICAL, MAGIC_SURF,
            SURFEL_MINI_HEADER_SIZE, SURFEL_RECORD_SIZE, SURFEL_SECTION_VERSION,
        },
        AlignmentTier, Container10dHeader, SectionDescriptor, SectionType, TypedFieldRecord,
        TypedPayloadResolver, FIELD_KIND_ARCHETYPE_REF, FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE,
        MANIFOLD_IDENTITY_V2_MAX_FIELDS,
    },
    modalities::evidential_etau::f16_to_f32,
    q42::m64_matter::M64HoloRecord,
    render::{assets::Mesh, compile_10d::decode_10d_mesh},
};
use std::{collections::BTreeMap, sync::Arc};

const MAX_CLUSTERS: usize = 4096;
const MAX_CLUSTER_TRIANGLES: usize = 64;
const RING_BYTES: usize = 128 * 1024 * 1024;
const TILE: usize = 96;
const TURN_FRAMES: usize = 8;

impl AssetHost<'_> {
    pub(crate) fn bake_gaussian_surfels_10d(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source_10d", "clusters", "target_surfel_count"])?;
        let source = required(args, "source_10d")?;
        let clusters = list(required(args, "clusters")?)?;
        let count = integer(required(args, "target_surfel_count")?, "target_surfel_count")?;
        if !(1..=MAX_CLUSTERS).contains(&count) || count != clusters.len() {
            return Err("Target surfel count must equal the number of bounded authored clusters".into());
        }
        let size = source_10d(self, source)?.bytes.len();
        let surfel_bytes = SURFEL_MINI_HEADER_SIZE + count * SURFEL_RECORD_SIZE;
        let output_budget = self.limits.resource_bytes.min(RING_BYTES);
        self.charge(
            size.saturating_mul(4).saturating_add(count * TURN_FRAMES * TILE * TILE),
            size.saturating_add(count * MAX_CLUSTER_TRIANGLES * 16 + 16 * 1024 * 1024),
        )?;
        let bytes = &source_10d(self, source)?.bytes;
        let source_digest = blake3::hash(bytes).to_hex().to_string();
        check_whole_file_crc32c(bytes).map_err(|error| error.to_string())?;
        let header = Container10dHeader::parse(bytes).map_err(|error| error.to_string())?;
        let table = ten::parse_section_table(bytes, &header).map_err(|error| error.to_string())?;
        if table.iter().any(|row| row.typ() == Some(SectionType::HierarchicalSurfels)) {
            return Err("Source .10d already contains a SURF section".into());
        }
        require_holographic_leaf(bytes)?;
        let mesh_section = table.iter().find(|row| row.typ() == Some(SectionType::QuantizedMesh))
            .ok_or("Surfel baking requires authored, quantized triangle geometry")?;
        let (mesh_header, _) = ten::parse_mesh_header(section(bytes, mesh_section)?)
            .map_err(|error| error.to_string())?;
        if mesh_header.triangle_count == 0
            || mesh_header.triangle_count as usize > count * MAX_CLUSTER_TRIANGLES
        {
            return Err("Source triangle count cannot be covered by the requested SURF clusters".into());
        }
        let mesh = decode_10d_mesh(bytes).map_err(|error| format!("Surfel source mesh: {error:?}"))?;
        if mesh.positions.iter().flatten().any(|coordinate| !coordinate.is_finite() || coordinate.abs() > 1e6) {
            return Err("Surfel source vertices exceed finite local-metre precision".into());
        }
        if mesh.triangles.iter().flatten().any(|&index| index as usize >= mesh.positions.len()) {
            return Err("Surfel source contains an out-of-range triangle vertex".into());
        }
        check_impostor_alignment(bytes, table)?;
        let verification = verify_entity_10d(bytes);
        if !verification.valid || !verification.crc32c_valid || verification.triangles != mesh.triangles.len() {
            return Err(format!("Source .10d failed entity verification: {:?}", verification.violations));
        }
        let surfels = bake_clusters(&mesh, clusters)?;
        let mini = surfel_header(&surfels)?;
        let mut surf_payload = vec![0; surfel_bytes];
        let written = encode_surfel_section(&mini, &surfels, &mut surf_payload)
            .map_err(|error| error.to_string())?;
        if written != surfel_bytes {
            return Err("Native SURF codec returned an inconsistent section length".into());
        }
        let sealed = attach_surf(bytes, header, table, &surf_payload, output_budget)?;
        let verified = verify_entity_10d(&sealed);
        if !verified.valid || !verified.crc32c_valid || !verified.reverse_lod_valid
            || verified.surfels.as_ref().map(|summary| summary.surfel_count as usize) != Some(count)
            || verified.vertices != mesh.positions.len() || verified.triangles != mesh.triangles.len()
        {
            return Err(format!("Sealed SURF .10d failed entity verification: {:?}", verified.violations));
        }
        require_holographic_leaf(&sealed)?;
        let preview = turntable(&surfels, &mini);
        let asset = self.encode("10d", sealed)?;
        let preview = self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(preview)))?;
        Ok(Value::Record(BTreeMap::from([
            ("asset".into(), asset),
            ("turntable_preview".into(), preview),
            ("surfel_count".into(), Value::U64(count as u64)),
            ("preview_view_count".into(), Value::U64(TURN_FRAMES as u64)),
            ("preview_mode".into(), Value::String("orthographic_geometry_diagnostic".into())),
            ("source_blake3".into(), Value::String(source_digest)),
        ])))
    }
}

fn source_10d<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
    let item = match value {
        Value::String(key) => host.inputs.iter().find(|row| &row.key == key)
            .ok_or("Unknown retained .10d source")?,
        _ => host.workspace.encoded(value)?,
    };
    if item.kind != "10d" || item.bytes.len() > host.limits.resource_bytes.min(RING_BYTES) {
        return Err("Expected a bounded retained or encoded .10d source".into());
    }
    Ok(item)
}

fn require_holographic_leaf(bytes: &[u8]) -> Result<(), String> {
    let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = scratch;
    let resolver = TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut fields)
        .map_err(|error| error.to_string())?;
    if resolver.payloads_of_kind(FIELD_KIND_ARCHETYPE_REF).next().is_some() {
        return Err("Surfel clustering requires an authored leaf rather than assembly proxy geometry".into());
    }
    let mut holo = resolver.payloads_of_kind(FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE);
    let payload = holo.next().ok_or("Surfel source requires a HOLO section")?
        .map_err(|error| error.to_string())?.bytes;
    if holo.next().is_some() || payload.len() < HOLOGRAPHIC_PHASE_HEADER_SIZE {
        return Err("Surfel source requires exactly one complete HOLO payload".into());
    }
    let header: HolographicPhaseHeader = bytemuck::pod_read_unaligned(&payload[..HOLOGRAPHIC_PHASE_HEADER_SIZE]);
    let segment_count = usize::from(header.holo_segment_count);
    let wave_count = usize::from(header.wavefield_count);
    if segment_count == 0 || segment_count > MAX_HOLO_SEGMENTS || wave_count > MAX_EULERIAN_WAVEFIELDS {
        return Err("Surfel source HOLO segment counts exceed the admitted leaf profile".into());
    }
    let mut segments = vec![M64HoloRecord::default(); segment_count];
    let mut wavefields = vec![<EulerianWavefieldRecord as bytemuck::Zeroable>::zeroed(); wave_count];
    decode_holographic_phase_section_into(payload, &mut segments, &mut wavefields)
        .map_err(|error| format!("Surfel source HOLO decode: {error:?}"))?;
    Ok(())
}

fn bake_clusters(mesh: &Mesh, clusters: &[Value]) -> Result<Vec<SurfelBinaryRecord>, String> {
    let mut covered = vec![false; mesh.triangles.len()];
    let mut output = Vec::with_capacity(clusters.len());
    let mut indices = Vec::with_capacity(3 * MAX_CLUSTER_TRIANGLES);
    for (cluster_index, cluster) in clusters.iter().enumerate() {
        let cluster = record(cluster)?;
        fields(cluster, &["triangle_indices", "spectral_coeffs", "roughness", "flags"])?;
        let members = list(required(cluster, "triangle_indices")?)?;
        if !(1..=MAX_CLUSTER_TRIANGLES).contains(&members.len()) {
            return Err(format!("Surfel cluster {cluster_index} requires 1–64 source triangles"));
        }
        let spectral = vector::<3>(required(cluster, "spectral_coeffs")?)?;
        let roughness = num(required(cluster, "roughness")?)?;
        if spectral.iter().any(|coeff| coeff.abs() > 65_504.0) || !(0.0..=1.0).contains(&roughness) {
            return Err(format!("Surfel cluster {cluster_index} has out-of-range material parameters"));
        }
        let flags = cluster.get("flags").map(|value| integer(value, "flags")).transpose()?.unwrap_or(0);
        let flags = u32::try_from(flags).map_err(|_| "Surfel flags exceed u32")?;
        indices.clear();
        let mut normal_sum = [0.0; 3];
        let mut double_area = 0.0;
        for member in members {
            let triangle_index = integer(member, "triangle index")?;
            let used = covered.get_mut(triangle_index).ok_or("Surfel triangle index is out of range")?;
            if std::mem::replace(used, true) {
                return Err("Source triangle belongs to more than one surfel cluster".into());
            }
            let vertices = mesh.triangles[triangle_index];
            let [a, b, c] = vertices.map(|index| mesh.positions[index as usize]);
            let normal = cross(subtract(b, a), subtract(c, a));
            let area = dot(normal, normal).sqrt();
            if !area.is_finite() || area <= 2e-8 {
                return Err(format!("Surfel source triangle {triangle_index} is degenerate at stored precision"));
            }
            double_area += area;
            for axis in 0..3 {
                normal_sum[axis] += normal[axis];
            }
            indices.extend(vertices);
        }
        let normal_length = dot(normal_sum, normal_sum).sqrt();
        if normal_length <= 1e-6 || normal_length < double_area * 0.05 {
            return Err(format!("Surfel cluster {cluster_index} has no stable aggregate orientation"));
        }
        let surfel = generate_surfel_from_cluster(&mesh.positions, &indices, spectral.map(|v| v as f32),
            roughness as f32, flags).map_err(|error| format!("Surfel cluster {cluster_index}: {error}"))?;
        let [major, minor] = surfel.radii.map(f16_to_f32);
        if !major.is_finite() || !minor.is_finite() || minor <= 0.0 || major < minor
            || surfel.position.iter().any(|value| !value.is_finite())
        {
            return Err(format!("Surfel cluster {cluster_index} exceeds its finite encoded position or radius"));
        }
        output.push(surfel);
    }
    if covered.iter().any(|used| !used) {
        return Err("Authored surfel clusters must cover every source triangle exactly once".into());
    }
    Ok(output)
}

fn surfel_header(surfels: &[SurfelBinaryRecord]) -> Result<SurfelSectionMiniHeader, String> {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for surfel in surfels {
        let major = f16_to_f32(surfel.radii[0]);
        for axis in 0..3 {
            min[axis] = min[axis].min(surfel.position[axis] - major);
            max[axis] = max[axis].max(surfel.position[axis] + major);
        }
    }
    let center = std::array::from_fn(|axis| (min[axis] + max[axis]) * 0.5);
    let extent = subtract(max, center);
    let radius = dot(extent, extent).sqrt();
    if !radius.is_finite() || radius <= 0.0 {
        return Err("SURF bounds violate the finite nonempty section invariant".into());
    }
    Ok(SurfelSectionMiniHeader {
        magic: MAGIC_SURF,
        version: SURFEL_SECTION_VERSION,
        flags: FLAG_SURFEL_HIERARCHICAL,
        surfel_count: surfels.len() as u32,
        stride: SURFEL_RECORD_SIZE as u32,
        sphere_center: center,
        sphere_radius: radius,
        aabb_min: min,
        aabb_max: max,
        reserved: [0; 8],
    })
}

fn section<'a>(bytes: &'a [u8], descriptor: &SectionDescriptor) -> Result<&'a [u8], String> {
    let start = descriptor.byte_offset as usize;
    let end = start.checked_add(descriptor.byte_length as usize).ok_or("10D section extent overflow")?;
    bytes.get(start..end).ok_or_else(|| "10D section exceeds file extent".into())
}

fn check_impostor_alignment(bytes: &[u8], table: &[SectionDescriptor]) -> Result<(), String> {
    if let Some(descriptor) = table.iter().find(|row| row.typ() == Some(SectionType::ImpostorAtlas)) {
        let payload = section(bytes, descriptor)?;
        if (payload.as_ptr() as usize) % std::mem::align_of::<ImpostorAtlasMiniHeader>() != 0 {
            return Err("Source IMPO payload is not aligned for native verification".into());
        }
    }
    Ok(())
}

fn attach_surf(source: &[u8], mut header: Container10dHeader, table: &[SectionDescriptor],
    surf: &[u8], budget: usize) -> Result<Vec<u8>, String> {
    // Section discovery keeps the usual type-sorted descriptor table. Payloads
    // start with SURF, then keep the physical order of all untouched sections.
    let mut entries = Vec::with_capacity(table.len() + 1);
    entries.push((SectionDescriptor {
        section_type: SectionType::HierarchicalSurfels as u8,
        alignment_tier: AlignmentTier::CacheLine as u8,
        reserved16: 0,
        byte_offset: 0,
        byte_length: u32::try_from(surf.len()).map_err(|_| "SURF section byte length overflow")?,
        stride: 0,
        element_count: 0,
        crc32c: ten::crc32c(surf),
    }, surf));
    let mut physical = table.to_vec();
    physical.sort_by_key(|descriptor| (descriptor.byte_offset, descriptor.byte_length));
    for descriptor in physical {
        entries.push((descriptor, section(source, &descriptor)?));
    }
    let table_end = HEADER_BYTE_SIZE + entries.len() * SECTION_DESCRIPTOR_SIZE;
    let mut cursor = table_end;
    for (descriptor, payload) in &mut entries {
        let align = AlignmentTier::from_u8(descriptor.alignment_tier).ok_or("Invalid 10D payload alignment")?.to_bytes();
        cursor = (cursor + align - 1) & !(align - 1);
        descriptor.byte_offset = u32::try_from(cursor).map_err(|_| "SURF section byte offset overflow")?;
        cursor = cursor.checked_add(payload.len()).ok_or("SURF aggregate container length overflow")?;
        if cursor > budget {
            return Err("SURF output exceeds the production .10d ring budget".into());
        }
    }
    let mut output = vec![0; cursor];
    header.section_table_offset = HEADER_BYTE_SIZE as u32;
    header.section_count = entries.len() as u32;
    header.header_crc32c = 0;
    let mut header_bytes = [0; HEADER_BYTE_SIZE];
    header.encode(&mut header_bytes);
    output[..HEADER_BYTE_SIZE].copy_from_slice(&header_bytes);
    entries.sort_by_key(|(descriptor, _)| descriptor.section_type);
    for (index, (descriptor, payload)) in entries.iter().enumerate() {
        let start = HEADER_BYTE_SIZE + index * SECTION_DESCRIPTOR_SIZE;
        output[start..start + SECTION_DESCRIPTOR_SIZE].copy_from_slice(bytemuck::bytes_of(descriptor));
        let offset = descriptor.byte_offset as usize;
        output[offset..offset + payload.len()].copy_from_slice(payload);
    }
    ten::seal_whole_file_crc32c(&mut output);
    check_whole_file_crc32c(&output).map_err(|error| error.to_string())?;
    let result_header = Container10dHeader::parse(&output).map_err(|error| error.to_string())?;
    let result_table = ten::parse_section_table(&output, &result_header).map_err(|error| error.to_string())?;
    check_impostor_alignment(&output, result_table)?;
    for (expected, payload) in entries {
        let actual = result_table.iter().find(|row| row.section_type == expected.section_type)
            .ok_or("Repacked 10D lost a source section")?;
        if actual != &expected || section(&output, actual)? != payload {
            return Err("Repacked 10D changed an admitted section or descriptor".into());
        }
    }
    let surf_descriptor = result_table.iter().find(|row| row.typ() == Some(SectionType::HierarchicalSurfels))
        .ok_or("Repacked 10D has no SURF section")?;
    let payload = section(&output, surf_descriptor)?;
    if (payload.as_ptr() as usize) % std::mem::align_of::<SurfelSectionMiniHeader>() != 0 {
        return Err("Repacked SURF payload is not aligned for native verification".into());
    }
    decode_surfel_section(payload).map_err(|error| error.to_string())?;
    Ok(output)
}

fn subtract(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|axis| a[axis] - b[axis])
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn turntable(surfels: &[SurfelBinaryRecord], header: &SurfelSectionMiniHeader) -> RadianceLattice {
    let (width, height) = (TILE * 4, TILE * 2);
    let mut pixels = vec![0; width * height * 4];
    for pixel in pixels.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[19, 25, 34, 255]);
    }
    let diagonal = std::f32::consts::FRAC_1_SQRT_2;
    let directions = [[0.0, 1.0], [diagonal, diagonal], [1.0, 0.0], [diagonal, -diagonal],
        [0.0, -1.0], [-diagonal, -diagonal], [-1.0, 0.0], [-diagonal, diagonal]];
    let scale = (TILE as f32 * 0.45) / header.sphere_radius;
    let mut order = (0..surfels.len()).collect::<Vec<_>>();
    for (view, [sin, cos]) in directions.into_iter().enumerate() {
        let eye = [sin * 0.970_142_5, 0.242_535_64, cos * 0.970_142_5];
        let right = [cos, 0.0, -sin];
        let up = cross(eye, right);
        order.sort_unstable_by(|&a, &b| {
            dot(subtract(surfels[a].position, header.sphere_center), eye)
                .total_cmp(&dot(subtract(surfels[b].position, header.sphere_center), eye))
                .then(a.cmp(&b))
        });
        let offset = [(view % 4) * TILE, (view / 4) * TILE];
        for &index in &order {
            let surfel = surfels[index];
            let normal = decode_octahedral_normal(surfel.normal_oct32);
            let [x, y, z] = normal;
            let sign = if z >= 0.0 { 1.0 } else { -1.0 };
            let a = -1.0 / (sign + z);
            let b = x * y * a;
            let tangent = [1.0 + sign * x * x * a, sign * b, -sign * x];
            let bitangent = [b, sign + y * y * a, -y];
            let center = subtract(surfel.position, header.sphere_center);
            let [major, minor] = surfel.radii.map(f16_to_f32);
            let pivot = [TILE as f32 * 0.5 + dot(center, right) * scale,
                TILE as f32 * 0.5 - dot(center, up) * scale];
            let t = [dot(tangent, right) * major * scale, -dot(tangent, up) * major * scale];
            let b = [dot(bitangent, right) * minor * scale, -dot(bitangent, up) * minor * scale];
            let shade = 0.45 + 0.55 * dot(normal, eye).abs();
            paint_splat(&mut pixels, width, offset, pivot, t, b, shade);
        }
    }
    RadianceLattice {
        width: width as u32,
        height: height as u32,
        channels: 4,
        primaries_iri: "http://www.w3.org/ns/ttml/parameter/rec709".into(),
        transfer_iri: "http://www.w3.org/ns/ttml/parameter/sRGB".into(),
        samples: LatticeSamples::U8(pixels),
    }
}

fn paint_splat(pixels: &mut [u8], width: usize, offset: [usize; 2], pivot: [f32; 2],
    tangent: [f32; 2], bitangent: [f32; 2], shade: f32) {
    let [t0, t1] = tangent;
    let [b0, b1] = bitangent;
    let determinant = t0 * b1 - t1 * b0;
    if determinant.abs() < 1e-5 {
        return;
    }
    let span = [(t0 * t0 + b0 * b0).sqrt(), (t1 * t1 + b1 * b1).sqrt()];
    let min: [usize; 2] = std::array::from_fn(|axis|
        (pivot[axis] - span[axis]).floor().clamp(0.0, TILE as f32) as usize);
    let max: [usize; 2] = std::array::from_fn(|axis|
        (pivot[axis] + span[axis]).ceil().clamp(0.0, TILE as f32) as usize);
    let color = [174.0, 198.0, 228.0].map(|channel| channel * shade);
    for y in min[1]..max[1] {
        for x in min[0]..max[0] {
            let dx = (x as f32 + 0.5) - pivot[0];
            let dy = (y as f32 + 0.5) - pivot[1];
            let u = (b1 * dx - b0 * dy) / determinant;
            let v = (t0 * dy - t1 * dx) / determinant;
            let d2 = u * u + v * v;
            if d2 <= 1.0 {
                let alpha = (-2.0 * d2).exp();
                let index = ((y + offset[1]) * width + x + offset[0]) * 4;
                for (channel, target) in pixels[index..index + 3].iter_mut().zip(color) {
                    *channel = (f32::from(*channel) * (1.0 - alpha) + target * alpha).round() as u8;
                }
            }
        }
    }
}
