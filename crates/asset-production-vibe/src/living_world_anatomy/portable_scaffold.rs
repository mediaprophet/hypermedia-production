//! Portable Model-Family Anatomical Scaffold Compiler (`portable_scaffold.rs`).
//!
//! Compiles VibeScript species-spec records (`qualia.portable-anatomy-scaffold/v1`, resolved by
//! [`super::species_spec`]) into a complete 3-layer
//! [`CompiledVertebrateAnatomy10d`] living container (`Osteology`, `MyologyAndTendons`,
//! `ExternalIntegument`) with `mirror_x` bilateral symmetry, named palette lookup,
//! named skeletal joint aliases, multi-station countershaded lofts, bicondylar long bones,
//! costal rib arches, fusiform muscle bellies, cylinders, and ellipsoids.

use qualia_core_db::container_10d::SubmeshRange;
use qualia_core_db::q_hash;
use qualia_core_db::render::skeletal::fauna_rig::*;
use qualia_core_db::render::vertebrate_anatomy::{
    finalize_and_seal_vertebrate_anatomy_10d, make_pbr_material, AnatomicalLoftRing,
    AnatomyMeshAccumulator, CompiledVertebrateAnatomy10d, ZoologicalAnatomicalLayer,
};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
#[path = "portable_bind.rs"]
mod portable_bind;
use qualia_core_db::render::vertebrate_anatomy::authored_skin::{RIGHT_SHOULDER, RIGHT_ELBOW, RIGHT_WRIST};

/// Compiles a 3-layer `.10d` vertebrate anatomical twin from a species spec (VibeScript record
/// converted to its JSON data model).
pub fn build_anatomy_from_portable_scaffold(
    scaffold: &JsonValue,
    default_archetype: FaunaBiomechanicalArchetype,
    osteology_vis_01: f32,
    myology_vis_01: f32,
    integument_vis_01: f32,
    explode_layer_01: f32,
) -> Result<CompiledVertebrateAnatomy10d, String> {
    let obj = scaffold
        .as_object()
        .ok_or_else(|| "Portable anatomy scaffold must be a JSON object".to_string())?;

    let archetype = if let Some(arch_str) = obj.get("archetype").and_then(|v| v.as_str()) {
        super::parse_fauna_archetype(arch_str).unwrap_or(default_archetype)
    } else {
        default_archetype
    };

    // Parse optional named color palette: { "slate_dorsal": [0.16, 0.14, 0.13, 1.0], ... }
    let mut palette: BTreeMap<String, [f32; 4]> = BTreeMap::new();
    if let Some(pal_obj) = obj.get("palette").and_then(|v| v.as_object()) {
        for (k, v) in pal_obj {
            if let Some(rgba) = parse_rgba_raw(v) {
                palette.insert(k.clone(), rgba);
            }
        }
    }

    let submeshes_arr = obj
        .get("submeshes")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Portable scaffold requires a 'submeshes' array".to_string())?;
    if submeshes_arr.is_empty() {
        return Err("Portable scaffold 'submeshes' array cannot be empty".into());
    }

    // Group submeshes in strict layer order: Osteology -> Myology -> Integument
    // so the contiguous vertex ranges match `.10d` layer expectations.
    let mut grouped_submeshes: [Vec<&JsonValue>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    for sm in submeshes_arr {
        let layer_str = sm
            .get("layer")
            .and_then(|v| v.as_str())
            .unwrap_or("integument");
        match parse_layer(layer_str)? {
            ZoologicalAnatomicalLayer::Osteology => grouped_submeshes[0].push(sm),
            ZoologicalAnatomicalLayer::MyologyAndTendons => grouped_submeshes[1].push(sm),
            ZoologicalAnatomicalLayer::ExternalIntegument => grouped_submeshes[2].push(sm),
        }
    }

    let mut acc = AnatomyMeshAccumulator::new();
    acc.authored_skin = portable_bind::parse(obj.get("bind_landmarks"), archetype)?;
    let mut materials = Vec::with_capacity(submeshes_arr.len().max(3));
    let mut submesh_ranges = Vec::with_capacity(submeshes_arr.len().max(3));

    let mut layer_v_ranges = [0..0, 0..0, 0..0];
    let layers = [
        ZoologicalAnatomicalLayer::Osteology,
        ZoologicalAnatomicalLayer::MyologyAndTendons,
        ZoologicalAnatomicalLayer::ExternalIntegument,
    ];

    for (li, layer) in layers.iter().copied().enumerate() {
        let layer_start_v = acc.positions.len();
        for (si, sm) in grouped_submeshes[li].iter().enumerate() {
            let sm_start_idx = acc.indices.len() as u32;
            let sm_id = sm
                .get("id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("submesh_{li}_{si}"));
            let semantic_iri = sm
                .get("semantic_iri")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("urn:qualia:bio:submesh:{sm_id}"));
            let default_col = match layer {
                ZoologicalAnatomicalLayer::Osteology => [0.91, 0.88, 0.81, 1.0],
                ZoologicalAnatomicalLayer::MyologyAndTendons => [0.68, 0.20, 0.19, 1.0],
                ZoologicalAnatomicalLayer::ExternalIntegument => [0.24, 0.20, 0.17, 1.0],
            };
            let mat_color = resolve_color(sm.get("color"), &palette, default_col);
            let roughness = sm
                .get("roughness")
                .and_then(|v| v.as_f64())
                .unwrap_or(match layer {
                    ZoologicalAnatomicalLayer::Osteology => 0.42,
                    ZoologicalAnatomicalLayer::MyologyAndTendons => 0.55,
                    ZoologicalAnatomicalLayer::ExternalIntegument => 0.82,
                }) as f32;
            let metallic = sm
                .get("metallic")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0) as f32;
            let sm_mirror = sm
                .get("mirror_x")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            emit_submesh_primitives(&mut acc, sm, layer, mat_color, sm_mirror, &palette)?;

            let index_count = (acc.indices.len() as u32).saturating_sub(sm_start_idx);
            if index_count > 0 {
                let mat = make_pbr_material(&sm_id, mat_color, roughness, metallic);
                materials.push(mat);
                submesh_ranges.push(SubmeshRange {
                    first_index: sm_start_idx,
                    index_count,
                    material_id: mat.id,
                    semantic_id: q_hash(&semantic_iri),
                });
            }
        }
        let layer_end_v = acc.positions.len();
        layer_v_ranges[li] = layer_start_v..layer_end_v;
    }

    if acc.positions.is_empty() || acc.indices.is_empty() {
        return Err("Portable scaffold produced zero geometry".into());
    }

    Ok(finalize_and_seal_vertebrate_anatomy_10d(
        archetype,
        acc,
        materials,
        submesh_ranges,
        layer_v_ranges[0].clone(),
        layer_v_ranges[1].clone(),
        layer_v_ranges[2].clone(),
        osteology_vis_01,
        myology_vis_01,
        integument_vis_01,
        explode_layer_01,
    ))
}

fn emit_submesh_primitives(
    acc: &mut AnatomyMeshAccumulator,
    sm: &JsonValue,
    layer: ZoologicalAnatomicalLayer,
    default_col: [f32; 4],
    sm_mirror: bool,
    palette: &BTreeMap<String, [f32; 4]>,
) -> Result<(), String> {
    let bilateral_forelimbs = acc.authored_skin.is_some();
    let mirror_joint = |joint| mirror_joint_for_binding(joint, bilateral_forelimbs);
    // 1. Multi-station countershaded lofts
    if let Some(lofts) = sm.get("lofts").and_then(|v| v.as_array()) {
        for loft in lofts {
            let radial_segs = loft
                .get("radial_segs")
                .and_then(|v| v.as_u64())
                .unwrap_or(14) as usize;
            let cap_ends = loft
                .get("cap_ends")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            let mirror = loft
                .get("mirror_x")
                .and_then(|v| v.as_bool())
                .unwrap_or(sm_mirror);
            let loft_dorsal = resolve_color(loft.get("dorsal"), palette, default_col);
            let loft_flank = resolve_color(loft.get("flank"), palette, loft_dorsal);
            let loft_ventral = resolve_color(loft.get("ventral"), palette, loft_flank);
            let def_j1 = parse_joint(loft.get("joint1"), JOINT_PELVIS_ROOT);
            let def_j2 = parse_joint(loft.get("joint2"), def_j1);
            let def_w = loft.get("weight").and_then(|v| v.as_f64()).unwrap_or(0.85) as f32;

            let st_arr = loft
                .get("stations")
                .and_then(|v| v.as_array())
                .ok_or_else(|| "Loft primitive requires 'stations' array".to_string())?;
            let mut rings = Vec::with_capacity(st_arr.len());
            for st in st_arr {
                let center = parse_vec3(st.get("center")).unwrap_or([0.0, 0.0, 0.0]);
                let radii = parse_vec2(st.get("radii")).unwrap_or([0.03, 0.03]);
                let dorsal = resolve_color(st.get("dorsal"), palette, loft_dorsal);
                let flank = resolve_color(st.get("flank"), palette, loft_flank);
                let ventral = resolve_color(st.get("ventral"), palette, loft_ventral);
                let primary_joint = parse_joint(st.get("joint1"), def_j1);
                let secondary_joint = parse_joint(st.get("joint2"), def_j2);
                let primary_weight = st
                    .get("weight")
                    .and_then(|v| v.as_f64())
                    .map(|w| w as f32)
                    .unwrap_or(def_w);
                rings.push(AnatomicalLoftRing {
                    center,
                    radii,
                    dorsal_color: dorsal,
                    flank_color: flank,
                    ventral_color: ventral,
                    primary_joint,
                    secondary_joint,
                    primary_weight,
                });
            }
            acc.push_countershaded_loft(&rings, radial_segs, layer, cap_ends);
            if mirror {
                let mirrored: Vec<AnatomicalLoftRing> = rings
                    .iter()
                    .map(|r| AnatomicalLoftRing {
                        center: [-r.center[0], r.center[1], r.center[2]],
                        radii: r.radii,
                        dorsal_color: r.dorsal_color,
                        flank_color: r.flank_color,
                        ventral_color: r.ventral_color,
                        primary_joint: mirror_joint(r.primary_joint),
                        secondary_joint: mirror_joint(r.secondary_joint),
                        primary_weight: r.primary_weight,
                    })
                    .collect();
                acc.push_countershaded_loft(&mirrored, radial_segs, layer, cap_ends);
            }
        }
    }

    // 2. Bicondylar long bones
    if let Some(bones) = sm.get("long_bones").and_then(|v| v.as_array()) {
        for b in bones {
            let prox = parse_vec3(b.get("prox")).unwrap_or([0.0, 0.0, 0.0]);
            let dist = parse_vec3(b.get("dist")).unwrap_or([0.0, -0.1, 0.0]);
            let shaft_r = b.get("shaft_r").and_then(|v| v.as_f64()).unwrap_or(0.012) as f32;
            let condyle_r =
                b.get("condyle_r").and_then(|v| v.as_f64()).unwrap_or((shaft_r * 1.45) as f64) as f32;
            let bow = parse_vec3(b.get("bow")).unwrap_or([0.0, 0.0, 0.0]);
            let col = resolve_color(b.get("color"), palette, default_col);
            let j1 = parse_joint(b.get("joint1"), JOINT_PELVIS_ROOT);
            let j2 = parse_joint(b.get("joint2"), j1);
            let mirror = b
                .get("mirror_x")
                .and_then(|v| v.as_bool())
                .unwrap_or(sm_mirror);
            acc.push_long_bone_with_epiphyses(prox, dist, shaft_r, condyle_r, bow, col, layer, j1, j2);
            if mirror {
                acc.push_long_bone_with_epiphyses(
                    [-prox[0], prox[1], prox[2]],
                    [-dist[0], dist[1], dist[2]],
                    shaft_r,
                    condyle_r,
                    [-bow[0], bow[1], bow[2]],
                    col,
                    layer,
                    mirror_joint(j1),
                    mirror_joint(j2),
                );
            }
        }
    }

    // 3. Costal rib / girdle arches
    if let Some(arches) = sm.get("arches").and_then(|v| v.as_array()) {
        for a in arches {
            let spine_p = parse_vec3(a.get("spine_p")).unwrap_or([0.0, 0.3, 0.0]);
            let sternum_p = parse_vec3(a.get("sternum_p")).unwrap_or([0.0, 0.2, 0.0]);
            let span = a.get("span").and_then(|v| v.as_f64()).unwrap_or(0.06) as f32;
            let radius = a.get("radius").and_then(|v| v.as_f64()).unwrap_or(0.005) as f32;
            let bone_col = resolve_color(a.get("bone_color"), palette, default_col);
            let cart_col = resolve_color(
                a.get("cartilage_color"),
                palette,
                [0.82, 0.84, 0.85, 1.0],
            );
            let j1 = parse_joint(a.get("joint1"), JOINT_CHEST);
            let j2 = parse_joint(a.get("joint2"), JOINT_SPINE);
            let mirror = a
                .get("mirror_x")
                .and_then(|v| v.as_bool())
                .unwrap_or(sm_mirror);
            acc.push_curved_rib_arch(
                spine_p, sternum_p, span, radius, bone_col, cart_col, layer, j1, j2,
            );
            if mirror {
                acc.push_curved_rib_arch(
                    [-spine_p[0], spine_p[1], spine_p[2]],
                    [-sternum_p[0], sternum_p[1], sternum_p[2]],
                    -span,
                    radius,
                    bone_col,
                    cart_col,
                    layer,
                    mirror_joint(j1),
                    mirror_joint(j2),
                );
            }
        }
    }

    // 4. Fusiform muscle bellies with tendons
    if let Some(muscles) = sm.get("muscles").and_then(|v| v.as_array()) {
        for m in muscles {
            let origin = parse_vec3(m.get("origin")).unwrap_or([0.0, 0.0, 0.0]);
            let insertion = parse_vec3(m.get("insertion")).unwrap_or([0.0, -0.1, 0.0]);
            let rx = m.get("rx").and_then(|v| v.as_f64()).unwrap_or(0.025) as f32;
            let ry = m.get("ry").and_then(|v| v.as_f64()).unwrap_or(rx as f64) as f32;
            let bow = parse_vec3(m.get("bow")).unwrap_or([0.0, 0.0, 0.0]);
            let m_col = resolve_color(m.get("muscle_color"), palette, default_col);
            let t_col = resolve_color(
                m.get("tendon_color"),
                palette,
                [0.88, 0.85, 0.79, 1.0],
            );
            let j1 = parse_joint(m.get("joint1"), JOINT_PELVIS_ROOT);
            let j2 = parse_joint(m.get("joint2"), j1);
            let mirror = m
                .get("mirror_x")
                .and_then(|v| v.as_bool())
                .unwrap_or(sm_mirror);
            acc.push_fusiform_muscle(origin, insertion, rx, ry, bow, m_col, t_col, layer, j1, j2);
            if mirror {
                acc.push_fusiform_muscle(
                    [-origin[0], origin[1], origin[2]],
                    [-insertion[0], insertion[1], insertion[2]],
                    rx,
                    ry,
                    [-bow[0], bow[1], bow[2]],
                    m_col,
                    t_col,
                    layer,
                    mirror_joint(j1),
                    mirror_joint(j2),
                );
            }
        }
    }

    // 5. Ellipsoids (eyes, condyles, cranium bulbs, viscera, bursae)
    if let Some(ellipsoids) = sm.get("ellipsoids").and_then(|v| v.as_array()) {
        for el in ellipsoids {
            let center = parse_vec3(el.get("center")).unwrap_or([0.0, 0.0, 0.0]);
            let radii = parse_vec3(el.get("radii")).unwrap_or([0.02, 0.02, 0.02]);
            let u_segs = el.get("u_segs").and_then(|v| v.as_u64()).unwrap_or(12) as usize;
            let v_segs = el.get("v_segs").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
            let col = resolve_color(el.get("color"), palette, default_col);
            let j1 = parse_joint(el.get("joint1"), JOINT_HEAD);
            let j2 = parse_joint(el.get("joint2"), j1);
            let w = el.get("weight").and_then(|v| v.as_f64()).unwrap_or(0.95) as f32;
            let mirror = el
                .get("mirror_x")
                .and_then(|v| v.as_bool())
                .unwrap_or(sm_mirror);
            acc.push_ellipsoid(center, radii, u_segs, v_segs, col, layer, j1, j2, w);
            if mirror {
                acc.push_ellipsoid(
                    [-center[0], center[1], center[2]],
                    radii,
                    u_segs,
                    v_segs,
                    col,
                    layer,
                    mirror_joint(j1),
                    mirror_joint(j2),
                    w,
                );
            }
        }
    }

    // 6. Tapered cylinders
    if let Some(cyls) = sm.get("cylinders").and_then(|v| v.as_array()) {
        for c in cyls {
            let p1 = parse_vec3(c.get("p1")).unwrap_or([0.0, 0.0, 0.0]);
            let p2 = parse_vec3(c.get("p2")).unwrap_or([0.0, -0.05, 0.0]);
            let r1 = c.get("r1").and_then(|v| v.as_f64()).unwrap_or(0.01) as f32;
            let r2 = c.get("r2").and_then(|v| v.as_f64()).unwrap_or(r1 as f64) as f32;
            let r_segs = c.get("radial_segs").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
            let h_segs = c.get("height_segs").and_then(|v| v.as_u64()).unwrap_or(3) as usize;
            let col = resolve_color(c.get("color"), palette, default_col);
            let j1 = parse_joint(c.get("joint1"), JOINT_PELVIS_ROOT);
            let j2 = parse_joint(c.get("joint2"), j1);
            let w = c.get("weight").and_then(|v| v.as_f64()).unwrap_or(0.85) as f32;
            let mirror = c
                .get("mirror_x")
                .and_then(|v| v.as_bool())
                .unwrap_or(sm_mirror);
            acc.push_cylinder(p1, p2, r1, r2, r_segs, h_segs, col, layer, j1, j2, w);
            if mirror {
                acc.push_cylinder(
                    [-p1[0], p1[1], p1[2]],
                    [-p2[0], p2[1], p2[2]],
                    r1,
                    r2,
                    r_segs,
                    h_segs,
                    col,
                    layer,
                    mirror_joint(j1),
                    mirror_joint(j2),
                    w,
                );
            }
        }
    }

    Ok(())
}

fn parse_layer(s: &str) -> Result<ZoologicalAnatomicalLayer, String> {
    match s.trim().to_ascii_lowercase().as_str() {
        "osteology" | "bone" | "skeleton" | "skeletal" => Ok(ZoologicalAnatomicalLayer::Osteology),
        "myology" | "muscle" | "muscles" | "myology_and_tendons" => {
            Ok(ZoologicalAnatomicalLayer::MyologyAndTendons)
        }
        "integument" | "external_integument" | "skin" | "pelage" | "fur" => {
            Ok(ZoologicalAnatomicalLayer::ExternalIntegument)
        }
        other => Err(format!("Unknown anatomical layer '{other}'")),
    }
}

fn parse_joint(val: Option<&JsonValue>, default_joint: usize) -> usize {
    let Some(v) = val else {
        return default_joint;
    };
    if let Some(idx) = v.as_u64() {
        return (idx as usize).min(RIGHT_WRIST);
    }
    if let Some(s) = v.as_str() {
        match s.trim().to_ascii_lowercase().as_str() {
            "pelvis" | "pelvis_root" | "root" => JOINT_PELVIS_ROOT,
            "spine" | "lumbar" => JOINT_SPINE,
            "chest" | "thorax" => JOINT_CHEST,
            "neck" | "cervical" => JOINT_NECK,
            "head" | "cranium" | "skull" => JOINT_HEAD,
            "jaw" | "mandible" => JOINT_JAW,
            "left_hip" | "hip_l" | "hip" => JOINT_LEFT_HIP,
            "left_knee" | "knee_l" | "knee" => JOINT_LEFT_KNEE,
            "left_ankle" | "ankle_l" | "ankle" | "foot" => JOINT_LEFT_ANKLE,
            "right_hip" | "hip_r" => JOINT_RIGHT_HIP,
            "right_knee" | "knee_r" => JOINT_RIGHT_KNEE,
            "right_ankle" | "ankle_r" => JOINT_RIGHT_ANKLE,
            "left_shoulder" | "shoulder_l" | "shoulder" => JOINT_LEFT_SHOULDER,
            "left_elbow" | "elbow_l" | "elbow" => JOINT_LEFT_ELBOW,
            "left_wrist" | "left_wrist_wing" | "wrist" | "forepaw" => JOINT_LEFT_WRIST_WING,
            "right_shoulder" | "shoulder_r" => RIGHT_SHOULDER,
            "right_elbow" | "elbow_r" => RIGHT_ELBOW,
            "right_wrist" | "wrist_r" => RIGHT_WRIST,
            "tail_base" => JOINT_TAIL_BASE,
            "tail_mid" => JOINT_TAIL_MID,
            "tail_tip" => JOINT_TAIL_TIP,
            _ => default_joint,
        }
    } else {
        default_joint
    }
}

fn mirror_joint_for_binding(joint: usize, bilateral_forelimbs: bool) -> usize {
    match joint {
        JOINT_LEFT_SHOULDER if bilateral_forelimbs => RIGHT_SHOULDER,
        JOINT_LEFT_ELBOW if bilateral_forelimbs => RIGHT_ELBOW,
        JOINT_LEFT_WRIST_WING if bilateral_forelimbs => RIGHT_WRIST,
        RIGHT_SHOULDER if bilateral_forelimbs => JOINT_LEFT_SHOULDER,
        RIGHT_ELBOW if bilateral_forelimbs => JOINT_LEFT_ELBOW,
        RIGHT_WRIST if bilateral_forelimbs => JOINT_LEFT_WRIST_WING,
        JOINT_LEFT_HIP => JOINT_RIGHT_HIP,
        JOINT_LEFT_KNEE => JOINT_RIGHT_KNEE,
        JOINT_LEFT_ANKLE => JOINT_RIGHT_ANKLE,
        JOINT_RIGHT_HIP => JOINT_LEFT_HIP,
        JOINT_RIGHT_KNEE => JOINT_LEFT_KNEE,
        JOINT_RIGHT_ANKLE => JOINT_LEFT_ANKLE,
        other => other,
    }
}

fn parse_vec2(val: Option<&JsonValue>) -> Option<[f32; 2]> {
    let arr = val?.as_array()?;
    if arr.len() < 2 {
        return None;
    }
    Some([arr[0].as_f64()? as f32, arr[1].as_f64()? as f32])
}

fn parse_vec3(val: Option<&JsonValue>) -> Option<[f32; 3]> {
    let arr = val?.as_array()?;
    if arr.len() < 3 {
        return None;
    }
    Some([
        arr[0].as_f64()? as f32,
        arr[1].as_f64()? as f32,
        arr[2].as_f64()? as f32,
    ])
}

fn parse_rgba_raw(val: &JsonValue) -> Option<[f32; 4]> {
    let arr = val.as_array()?;
    if arr.len() < 3 {
        return None;
    }
    let r = arr[0].as_f64()? as f32;
    let g = arr[1].as_f64()? as f32;
    let b = arr[2].as_f64()? as f32;
    let a = if arr.len() >= 4 {
        arr[3].as_f64()? as f32
    } else {
        1.0
    };
    Some([r, g, b, a])
}

fn resolve_color(
    val: Option<&JsonValue>,
    palette: &BTreeMap<String, [f32; 4]>,
    fallback: [f32; 4],
) -> [f32; 4] {
    let Some(v) = val else {
        return fallback;
    };
    if let Some(name) = v.as_str() {
        if let Some(col) = palette.get(name) {
            return *col;
        }
    }
    parse_rgba_raw(v).unwrap_or(fallback)
}
