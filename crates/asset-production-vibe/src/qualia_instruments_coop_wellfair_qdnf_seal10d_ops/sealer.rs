//! Cardiac bundle and anatomy matter sealing over bounded retained resources.

use super::{fields, required, resource, text, unsigned, Args, AssetHost, Value, SYMBOLIC_LIMIT};
use crate::{boolean, num};
use qualia_core_db::{
    bundle::{semantic_model::{pack_semantic_hmc, resource_identity_meta, HmcPackAsset,
        HmcPackLimits, SemanticModelBuilder}, BundleReadLimits},
    q42::m64_matter::{M64LocalMetricHeader, LRM3_BLOCK_BYTES},
    q_hash, render::compile_10d::{upgrade_legacy_10d_to_holographic_v2, SourceLicenseTier},
};
use qualia_seal_10d::{parse_obj, parse_occ1, seal_human_heart_m64, seal_mesh,
    seal_vertebrate_bone_m64, OccupancyGrid, MAT_EPICARDIAL_FAT};
use std::collections::BTreeMap;

mod semantic;
mod work;

const MAX_VERTICES: usize = 200_000;
const MAX_TRIANGLES: usize = 400_000;
const MAX_TEN_D_BYTES: usize = 128 * 1024 * 1024;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "cardiac_seal" => cardiac(host, params),
        "heart_m64" | "bone_m64" => matter(host, op, params),
        "work_graph_present" => work::present(host, params),
        _ => Err(format!("Unsupported anatomy sealer operation: {op}")),
    }
}

fn entity(params: &Args) -> Result<&str, String> {
    let iri = text(required(params, "entity_iri")?)?;
    if iri.len() > 2048 {
        return Err("Anatomical entity IRI exceeds byte budget".into());
    }
    Ok(iri)
}

fn occupancy(host: &AssetHost<'_>, value: &Value) -> Result<(OccupancyGrid, usize), String> {
    let bytes = &resource(host, value, "occ1")?.bytes;
    let grid = parse_occ1(bytes)?;
    if grid.min.into_iter().zip(grid.max).any(|(min, max)| {
        !min.is_finite() || !max.is_finite() || min >= max
            || min as f64 * 10_000.0 < i32::MIN as f64
            || max as f64 * 10_000.0 > i32::MAX as f64
    }) || grid.material.iter().any(|&mat| mat > MAT_EPICARDIAL_FAT) {
        return Err("OCC1 bounds or tissue classifications are invalid".into());
    }
    let side = (grid.resolution as usize).div_ceil(16);
    let bricks = side * side * side;
    let volume_extent = 4096usize * 64 * bricks + 64 * (bricks + 1) + 512;
    if volume_extent > host.limits.resource_bytes {
        return Err("OCC1 matter volume may exceed production resource budget".into());
    }
    Ok((grid, volume_extent))
}

fn anchor_m64(bytes: &mut [u8], iri: &str) -> Result<(), String> {
    let view = bytes.get_mut(64..64 + LRM3_BLOCK_BYTES).ok_or("Matter output has no LRM3 header")?;
    let mut frame: M64LocalMetricHeader = bytemuck::pod_read_unaligned(view);
    frame.validate().map_err(|error| error.to_string())?;
    frame.anchor_did_hash = q_hash(iri);
    if frame.anchor_did_hash == 0 {
        return Err("Matter frame cannot use a zero anchor hash".into());
    }
    frame.update_crc();
    view.copy_from_slice(bytemuck::bytes_of(&frame));
    Ok(())
}

fn cardiac(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["obj", "occupancy", "entity_iri", "domain_id", "source_iri", "spdx_license",
        "attribution", "commercial_allowed", "derivatives_allowed", "share_alike_required",
        "modifications_summary"])?;
    let iri = entity(params)?;
    let domain = unsigned(required(params, "domain_id")?)?;
    let source_iri = text(required(params, "source_iri")?)?;
    let spdx = text(required(params, "spdx_license")?)?;
    let attribution = text(required(params, "attribution")?)?;
    let changes = text(required(params, "modifications_summary")?)?;
    let is_commercial = boolean(params, "commercial_allowed", false)?;
    let derivatives = boolean(params, "derivatives_allowed", false)?;
    let share_alike = boolean(params, "share_alike_required", false)?;
    let tier = SourceLicenseTier::classify_spdx(spdx);
    if !matches!(tier, SourceLicenseTier::UpstreamPermissiveOpen | SourceLicenseTier::UpstreamShareAlikeOrNc)
        || spdx.contains("-ND-") || !derivatives
        || is_commercial == spdx.contains("-NC-") || share_alike != spdx.contains("-SA-")
        || [attribution, changes].iter().any(|value| value.trim().is_empty() || value.len() > 4096)
    {
        return Err("Cardiac source licence or attribution does not authorise this derivative".into());
    }
    let licence_iri = format!("https://spdx.org/licenses/{spdx}.html");
    for iri in [iri, source_iri, &licence_iri] {
        resource_identity_meta(iri).map_err(|error| error.to_string())?;
    }
    let occupancy_input = required(params, "occupancy")?;
    let (grid, volume_budget) = occupancy(host, occupancy_input)?;
    let obj_input = required(params, "obj")?;
    let obj = resource(host, obj_input, "obj")?;
    let input_bytes = obj.bytes.len();
    if input_bytes > qualia_core_db::container_10d::provenance_section::MAX_PROVENANCE_FIELD {
        return Err("Cardiac source OBJ exceeds in-envelope provenance limits".into());
    }
    let source = std::str::from_utf8(&obj.bytes).map_err(|_| "Cardiac OBJ must be UTF-8")?;
    let mesh = parse_obj(source).map_err(|error| error.to_string())?;
    if mesh.positions.len() > MAX_VERTICES || mesh.indices.len() / 3 > MAX_TRIANGLES
        || mesh.positions.iter().flatten().any(|x| !x.is_finite())
        || mesh.indices.chunks_exact(3).any(|tri| tri[0] == tri[1] || tri[0] == tri[2] || tri[1] == tri[2])
        || mesh.indices.iter().any(|&idx| idx as usize >= mesh.positions.len())
    {
        return Err("Cardiac OBJ mesh exceeds bounds or has invalid geometry".into());
    }
    let scratch = volume_budget.saturating_add(mesh.positions.len() * 160)
        .saturating_add(mesh.indices.len() * 8).saturating_add(3 * 1024 * 1024);
    host.charge(input_bytes.saturating_add(grid.material.len() * 64), scratch)?;
    let bundle = seal_mesh(&mesh, Some(&grid)).map_err(|error| error.to_string())?;
    let ten_d = upgrade_legacy_10d_to_holographic_v2(&bundle.ten_d, iri, domain, [0.; 10])
        .map_err(|error| error.to_string())?;
    let provenance = serde_json::json!({
        "source_uri": source_iri, "attribution": attribution, "spdx_license": spdx,
        "origin_format": "obj", "commercial_allowed": is_commercial,
        "derivatives_allowed": derivatives, "share_alike_required": share_alike,
        "modifications_summary": changes,
    });
    let mut metadata = Vec::new();
    ciborium::into_writer(&provenance, &mut metadata).map_err(|error| error.to_string())?;
    let source = &resource(host, obj_input, "obj")?.bytes;
    let provenance = qualia_core_db::container_10d::provenance_section::ProvenanceSidecar::new(
        source.clone(), "model/obj", spdx,
    ).with_metadata(metadata, 0, *blake3::hash(source).as_bytes());
    let geometry = semantic::attach_cardiac_dictionary(&ten_d, iri, &provenance,
        host.limits.resource_bytes.min(MAX_TEN_D_BYTES))?;
    let mut m64 = bundle.m64.ok_or("Native cardiac sealer emitted no matter volume")?;
    anchor_m64(&mut m64, iri)?;
    let emf = bundle.emf.ok_or("Native cardiac sealer emitted no electromagnetic volume")?;
    let model = semantic::aspects(iri, source_iri, &licence_iri, &geometry, &m64, &emf)?;
    let m64_meta = resource_identity_meta(&format!("{iri}#matter")).map_err(|error| error.to_string())?;
    let emf_meta = resource_identity_meta(&format!("{iri}#electromagnetic")).map_err(|error| error.to_string())?;
    let asset = |key, kind, bytes, metadata| HmcPackAsset {
        key, kind, bytes, metadata, salience: None,
    };
    let assets = [
        asset("cardiac.10d", "10d", &geometry, None),
        asset("cardiac.m64", "m64", &m64, Some(&m64_meta)),
        asset("cardiac.emf", "emf", &emf, Some(&emf_meta)),
        asset("cardiac-aspects.q42", "q42", &model, None),
    ];
    let output = pack_semantic_hmc(&assets, None, HmcPackLimits {
        bundle: BundleReadLimits { file_bytes: host.limits.resource_bytes,
            ..BundleReadLimits::default() },
        resource_bytes: host.limits.resource_bytes,
        total_resource_bytes: host.limits.total_resource_bytes,
        semantic_bytes: host.limits.resource_bytes.min(SYMBOLIC_LIMIT),
    }, host.cancel).map_err(|error| error.to_string())?;
    let mut result = BTreeMap::new();
    for (key, kind, bytes) in [
        ("geometry", "10d", geometry), ("m64", "m64", m64), ("emf", "emf", emf),
        ("hmc", "hmc", output.hmc),
    ] {
        result.insert(key.into(), host.encode(kind, bytes)?);
    }
    Ok(Value::Record(result))
}

fn matter(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    let bone = op == "bone_m64";
    if bone {
        fields(params, &["occupancy", "entity_iri", "vbmd_mg_cm3", "bv_tv"])?;
    } else {
        fields(params, &["occupancy", "entity_iri"])?;
    }
    let iri = entity(params)?;
    resource_identity_meta(iri).map_err(|error| error.to_string())?;
    let (grid, extent) = occupancy(host, required(params, "occupancy")?)?;
    host.charge(grid.material.len().saturating_mul(64), extent)?;
    let mut bytes = if bone {
        let density = u16::try_from(unsigned(required(params, "vbmd_mg_cm3")?)?)
            .map_err(|_| "Bone mineral density exceeds native range")?;
        let fraction = num(required(params, "bv_tv")?)?;
        if density == 0 || !(0.0..=1.0).contains(&fraction) {
            return Err("Bone mineral density or volume fraction is outside the input domain".into());
        }
        seal_vertebrate_bone_m64(&grid, density, fraction as f32)
    } else {
        seal_human_heart_m64(&grid)
    }.map_err(|error| error.to_string())?;
    anchor_m64(&mut bytes, iri)?;
    let mut graph = SemanticModelBuilder::new(host.limits.resource_bytes.min(SYMBOLIC_LIMIT));
    graph.bind_iri(iri).map_err(|error| error.to_string())?;
    let graph = graph.merge_m64(&bytes, iri).and_then(SemanticModelBuilder::finish)
        .map_err(|error| error.to_string())?;
    Ok(Value::Record(BTreeMap::from([
        ("m64".into(), host.encode("m64", bytes)?),
        ("q42".into(), host.encode("q42", graph)?),
    ])))
}
