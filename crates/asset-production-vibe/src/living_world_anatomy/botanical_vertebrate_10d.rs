use super::*;
use crate::*;
use qualia_core_db::container_10d::integrity::verify_whole_file_crc32c;
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::botanical_anatomy::{
    build_scale1_whole_organism_10d, build_scale2_stem_bark_wedge_10d,
    build_scale3_floral_and_gumnut_dissection_10d, build_scale4_leaf_venation_and_histology_10d,
    DevelopmentalStage,
};
use qualia_core_db::render::vertebrate_anatomy::{
    build_scale5_vertebrate_anatomy_10d, evaluate_anatomical_layer_peel_zero_heap,
    ZoologicalAnatomicalLayer,
};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;
use vibe::Value;

impl AssetHost<'_> {
    /// Compiles a 4-scale botanical `.10d` organ container (`Scale 1` whole organism, `Scale 2`
    /// stem/bark/cambium/xylem wedge, `Scale 3` floral/operculum/gumnut dissection, or `Scale 4`
    /// leaf venation & stomatal histology cross-section), verifies all `.10d` integrity invariants,
    /// and returns `container_10d`, `mesh`, `turntable_png`, and `blueprint_svg` handles.
    pub(super) fn compile_botanical_organ_10d(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &["scale", "species_id", "peel_01", "anthesis_01", "stage"],
        )?;

        let scale = count(a, "scale", 4, 4)?;
        if !(1..=4).contains(&scale) {
            return Err("scale outside 1..=4".into());
        }
        let species_id = opt_string(a, "species_id", "corymbia-aparrerinja")?;
        if species_id.trim().is_empty() {
            return Err("species_id cannot be empty".into());
        }
        let peel_01 = number(a, "peel_01", 0.5)?;
        if !(0.0..=1.0).contains(&peel_01) {
            return Err("peel_01 outside 0.0..=1.0".into());
        }
        let anthesis_01 = number(a, "anthesis_01", 0.75)?;
        if !(0.0..=1.0).contains(&anthesis_01) {
            return Err("anthesis_01 outside 0.0..=1.0".into());
        }

        let stage = if let Some(stage_val) = a.get("stage") {
            match text(stage_val)? {
                "seedling" | "juvenile" => DevelopmentalStage::Seedling,
                "sapling" => DevelopmentalStage::Sapling,
                "mature" | "anthesis" => DevelopmentalStage::Mature,
                "ancient" | "ancient_hollow_bearing" | "senescent_gumnut" | "gumnut" => {
                    DevelopmentalStage::AncientHollowBearing
                }
                other => return Err(format!("Unknown developmental stage: {other}")),
            }
        } else if anthesis_01 < 0.25 {
            DevelopmentalStage::Seedling
        } else if anthesis_01 < 0.5 {
            DevelopmentalStage::Sapling
        } else if anthesis_01 <= 0.85 {
            DevelopmentalStage::Mature
        } else {
            DevelopmentalStage::AncientHollowBearing
        };

        self.charge(256, 8192)?;

        let organ = match scale {
            1 => whitegum_canopy::build_whitegum_whole_organism_10d(&species_id, stage)?,
            2 => build_scale2_stem_bark_wedge_10d(&species_id, peel_01 as f32)?,
            3 => build_scale3_floral_and_gumnut_dissection_10d(
                &species_id,
                anthesis_01 as f32,
                peel_01 as f32,
            )?,
            4 => build_scale4_leaf_venation_and_histology_10d(&species_id, peel_01 as f32)?,
            _ => unreachable!(),
        };

        organ.verify_all()?;

        let v_count = organ.vertex_count();
        let t_count = organ.triangle_count();
        let slice_count = organ.tissue_slices.len();
        let byte_len = organ.container_bytes.len();

        let rgba_f32: Vec<[f32; 4]> = organ
            .colors
            .iter()
            .map(|c| {
                [
                    c[0] as f32 / 255.0,
                    c[1] as f32 / 255.0,
                    c[2] as f32 / 255.0,
                    c[3] as f32 / 255.0,
                ]
            })
            .collect();
        let turntable_bytes = turntable_render::render_turnaround_sheet_png(
            &organ.vertices,
            &organ.normals,
            &rgba_f32,
            &organ.indices,
        )?;
        let blueprint_svg_bytes = turntable_render::render_anatomical_blueprint_svg(
            &format!("{species_id} (Scale {scale})"),
            &organ.vertices,
            &rgba_f32,
            &organ.indices,
            slice_count,
        );

        let container_10d = self.encode("10d", organ.container_bytes)?;
        let turntable_png = self.encode("png", turntable_bytes)?;
        let blueprint_svg = self.encode("svg", blueprint_svg_bytes)?;
        let mesh_handle = store_raw_mesh(self, organ.vertices, &organ.indices)?;
        let flora_plan_json = behavioral_hooks::build_flora_mycelium_production_plan_json(
            &species_id,
            scale,
            peel_01 as f32,
            anthesis_01 as f32,
        );
        let flora_plan_res = self.encode("json", flora_plan_json.clone().into_bytes())?;
        let flora_logic_q42 = self.encode(
            "q42",
            behavioral_hooks::compile_model_logic_q42_bytes(&species_id, true),
        )?;

        let mut out = BTreeMap::new();
        out.insert("scale".into(), Value::I64(scale as i64));
        out.insert("species_id".into(), Value::String(species_id));
        out.insert("vertex_count".into(), Value::I64(v_count as i64));
        out.insert("triangle_count".into(), Value::I64(t_count as i64));
        out.insert("tissue_slice_count".into(), Value::I64(slice_count as i64));
        out.insert("container_byte_len".into(), Value::I64(byte_len as i64));
        out.insert("verified".into(), Value::Bool(true));
        out.insert("container_10d".into(), container_10d);
        out.insert("mesh".into(), mesh_handle);
        out.insert("turntable_png".into(), turntable_png);
        out.insert("blueprint_svg".into(), blueprint_svg);
        out.insert("production_plan_json".into(), Value::String(flora_plan_json));
        out.insert("production_plan_resource".into(), flora_plan_res.clone());
        out.insert("mycelium_network_resource".into(), flora_plan_res);
        out.insert("logic_q42".into(), flora_logic_q42);

        Ok(Value::Record(out))
    }

    /// Compiles a Scale-5 comparative vertebrate 3-layer `.10d` anatomy container (`Osteology`,
    /// `Myology`, `Integument`) from either a portable `.json` / `.vibe` scaffold (`scaffold` or
    /// `scaffold_input`) or a baseline biomechanical archetype, evaluates the zero-heap anatomical
    /// layer peel operator, and returns `container_10d`, `mesh`, `turntable_png`, and `blueprint_svg`.
    pub(super) fn compile_vertebrate_anatomy_10d(&mut self, a: &Args) -> Result<Value, String> {
        if a.contains_key("profile") {
            return self.compile_human_welfare_anatomy_10d(a);
        }
        if let Some(arch_val) = a.get("archetype") {
            if let Ok(arch_s) = text(arch_val) {
                let norm = arch_s.trim().to_ascii_lowercase().replace('-', "_");
                if matches!(
                    norm.as_str(),
                    "male"
                        | "female"
                        | "female_trimester_1"
                        | "female_trimester_2"
                        | "female_trimester_3"
                        | "human_male"
                        | "human_female"
                        | "human_trimester_1"
                        | "human_trimester_2"
                        | "human_trimester_3"
                        | "homo_sapiens_male"
                        | "homo_sapiens_female"
                        | "homo_sapiens_pregnancy_t1"
                        | "homo_sapiens_pregnancy_t2"
                        | "homo_sapiens_pregnancy_t3"
                ) {
                    let mut human_args = a.clone();
                    let prof = norm
                        .trim_start_matches("human_")
                        .trim_start_matches("homo_sapiens_")
                        .replace("pregnancy_t", "female_trimester_");
                    human_args.remove("archetype");
                    human_args.insert("profile".into(), Value::String(prof));
                    return self.compile_human_welfare_anatomy_10d(&human_args);
                }
            }
        }

        fields(
            a,
            &[
                "archetype",
                "osteology_vis_01",
                "myology_vis_01",
                "integument_vis_01",
                "explode_01",
                "scaffold",
                "scaffold_input",
                "title",
            ],
        )?;

        let archetype_str = opt_string(a, "archetype", "macropod_saltatorial")?;
        let archetype = parse_fauna_archetype(&archetype_str)?;
        let title_str = opt_string(a, "title", &archetype_str)?;

        let osteology_vis_01 = number(a, "osteology_vis_01", 1.0)?;
        if !(0.0..=1.0).contains(&osteology_vis_01) {
            return Err("osteology_vis_01 outside 0.0..=1.0".into());
        }
        let myology_vis_01 = number(a, "myology_vis_01", 0.75)?;
        if !(0.0..=1.0).contains(&myology_vis_01) {
            return Err("myology_vis_01 outside 0.0..=1.0".into());
        }
        let integument_vis_01 = number(a, "integument_vis_01", 0.25)?;
        if !(0.0..=1.0).contains(&integument_vis_01) {
            return Err("integument_vis_01 outside 0.0..=1.0".into());
        }
        let explode_01 = number(a, "explode_01", 0.35)?;
        if !(0.0..=1.0).contains(&explode_01) {
            return Err("explode_01 outside 0.0..=1.0".into());
        }

        self.charge(512, 16384)?;

        let scaffold_json_opt = resolve_portable_scaffold_arg(self.inputs, a)?;
        let mut compiled = if let Some(scaffold_val) = scaffold_json_opt {
            portable_scaffold::build_anatomy_from_portable_scaffold(
                &scaffold_val,
                archetype,
                osteology_vis_01 as f32,
                myology_vis_01 as f32,
                integument_vis_01 as f32,
                explode_01 as f32,
            )?
        } else {
            build_scale5_vertebrate_anatomy_10d(
                archetype,
                osteology_vis_01 as f32,
                myology_vis_01 as f32,
                integument_vis_01 as f32,
                explode_01 as f32,
            )
        };
        verify_whole_file_crc32c(&mut compiled.container_bytes)
            .map_err(|e| format!("Vertebrate .10d CRC-32C verification failed: {e:?}"))?;

        let v_count = compiled.vertex_count;
        let in_positions = vec![[0.0f32, 0.5, 0.0]; v_count];
        let in_normals = vec![[0.0f32, 1.0, 0.0]; v_count];
        let mut in_layer_tags = vec![ZoologicalAnatomicalLayer::Osteology; v_count];
        for idx in compiled.myology_vertex_range.clone() {
            if idx < v_count {
                in_layer_tags[idx] = ZoologicalAnatomicalLayer::MyologyAndTendons;
            }
        }
        for idx in compiled.integument_vertex_range.clone() {
            if idx < v_count {
                in_layer_tags[idx] = ZoologicalAnatomicalLayer::ExternalIntegument;
            }
        }
        let mut out_positions = vec![[0.0f32; 3]; v_count];
        let mut out_alphas = vec![0.0f32; v_count];

        let peeled_vertex_count = evaluate_anatomical_layer_peel_zero_heap(
            &in_positions,
            &in_normals,
            &in_layer_tags,
            osteology_vis_01 as f32,
            myology_vis_01 as f32,
            integument_vis_01 as f32,
            explode_01 as f32,
            &mut out_positions,
            &mut out_alphas,
        );

        let turntable_bytes = turntable_render::render_turnaround_sheet_png(
            &compiled.positions,
            &compiled.normals,
            &compiled.colors,
            &compiled.indices,
        )?;
        let blueprint_svg_bytes = turntable_render::render_anatomical_blueprint_svg(
            &title_str,
            &compiled.positions,
            &compiled.colors,
            &compiled.indices,
            compiled.submesh_ranges.len(),
        );
        let hero_rgba_img = turntable_render::render_hero_rgba_frame(
            &compiled.positions,
            &compiled.normals,
            &compiled.colors,
            &compiled.indices,
        );
        let (cinema_rgba_imgs, grazing_rgba_imgs) = if integument_vis_01 >= 0.8 {
            let hop_frames = turntable_render::render_animated_cinema_frames(
                &compiled.positions,
                &compiled.normals,
                &compiled.colors,
                &compiled.indices,
                8,
                true,
            );
            let graze_hooks = behavioral_hooks::BehaviorMacro::GrazingBrowsing.default_hooks();
            let graze_frames = behavioral_hooks::render_macro_behavior_cinema_frames(
                &compiled.positions,
                &compiled.normals,
                &compiled.colors,
                &compiled.indices,
                behavioral_hooks::BehaviorMacro::GrazingBrowsing,
                &graze_hooks,
                8,
            );
            (hop_frames, graze_frames)
        } else {
            (Vec::new(), Vec::new())
        };

        let container_byte_len = compiled.container_bytes.len();
        let container_10d = self.encode("10d", compiled.container_bytes)?;
        let turntable_png = self.encode("png", turntable_bytes)?;
        let blueprint_svg = self.encode("svg", blueprint_svg_bytes)?;
        let hero_rgba = self
            .workspace
            .insert(Artifact::Rgba(hero_rgba_img), self.limits)?;
        let mut cinema_frames = Vec::with_capacity(cinema_rgba_imgs.len());
        for frame_img in cinema_rgba_imgs {
            cinema_frames.push(
                self.workspace
                    .insert(Artifact::Rgba(frame_img), self.limits)?,
            );
        }
        let mut grazing_frames = Vec::with_capacity(grazing_rgba_imgs.len());
        for frame_img in grazing_rgba_imgs {
            grazing_frames.push(
                self.workspace
                    .insert(Artifact::Rgba(frame_img), self.limits)?,
            );
        }
        let mesh_handle = store_raw_mesh(self, compiled.positions, &compiled.indices)?;

        let mut out = BTreeMap::new();
        out.insert("archetype".into(), Value::String(archetype_str.clone()));
        out.insert("vertex_count".into(), Value::I64(v_count as i64));
        out.insert(
            "triangle_count".into(),
            Value::I64(compiled.triangle_count as i64),
        );
        out.insert(
            "submesh_count".into(),
            Value::I64(compiled.submesh_ranges.len() as i64),
        );
        out.insert(
            "material_count".into(),
            Value::I64(compiled.materials.len() as i64),
        );
        out.insert(
            "osteology_vertex_count".into(),
            Value::I64(compiled.osteology_vertex_range.len() as i64),
        );
        out.insert(
            "myology_vertex_count".into(),
            Value::I64(compiled.myology_vertex_range.len() as i64),
        );
        out.insert(
            "integument_vertex_count".into(),
            Value::I64(compiled.integument_vertex_range.len() as i64),
        );
        out.insert(
            "container_byte_len".into(),
            Value::I64(container_byte_len as i64),
        );
        out.insert(
            "peeled_vertex_count".into(),
            Value::I64(peeled_vertex_count as i64),
        );
        out.insert("container_10d".into(), container_10d);
        out.insert("mesh".into(), mesh_handle);
        out.insert("turntable_png".into(), turntable_png);
        out.insert("blueprint_svg".into(), blueprint_svg);
        out.insert("hero_rgba".into(), hero_rgba);
        out.insert("cinema_frames".into(), Value::List(cinema_frames));
        out.insert("grazing_frames".into(), Value::List(grazing_frames));
        let fauna_plan_json =
            behavioral_hooks::build_fauna_production_plan_json(&archetype_str, &title_str);
        let fauna_plan_res = self.encode("json", fauna_plan_json.clone().into_bytes())?;
        let fauna_logic_q42 = self.encode(
            "q42",
            behavioral_hooks::compile_model_logic_q42_bytes("wallabia-bicolor", false),
        )?;
        out.insert("production_plan_json".into(), Value::String(fauna_plan_json));
        out.insert("production_plan_resource".into(), fauna_plan_res);
        out.insert("logic_q42".into(), fauna_logic_q42);

        Ok(Value::Record(out))
    }
}

fn store_raw_mesh(
    host: &mut AssetHost<'_>,
    positions: Vec<[f32; 3]>,
    indices: &[u32],
) -> Result<Value, String> {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in &positions {
        for i in 0..3 {
            min[i] = min[i].min(p[i]);
            max[i] = max[i].max(p[i]);
        }
    }
    if !min[0].is_finite() {
        min = [0.0; 3];
        max = [0.0; 3];
    }
    let triangles: Vec<[u32; 3]> = indices
        .chunks_exact(3)
        .map(|tri| [tri[0], tri[1], tri[2]])
        .collect();
    host.workspace.insert(
        Artifact::Mesh(Mesh {
            positions,
            triangles,
            min,
            max,
        }),
        host.limits,
    )
}

fn resolve_portable_scaffold_arg(
    inputs: &[Resource],
    a: &Args,
) -> Result<Option<JsonValue>, String> {
    if let Some(inp_val) = a.get("scaffold_input") {
        let key = text(inp_val)?;
        let res = inputs
            .iter()
            .find(|r| r.key == key)
            .ok_or_else(|| format!("Portable scaffold input '{key}' not found in workspace inputs"))?;
        let s = std::str::from_utf8(&res.bytes)
            .map_err(|e| format!("Portable scaffold input '{key}' is not UTF-8: {e}"))?;
        let parsed: JsonValue = serde_json::from_str(s)
            .map_err(|e| format!("Portable scaffold input '{key}' JSON error: {e}"))?;
        return Ok(Some(parsed));
    }
    if let Some(scaf_val) = a.get("scaffold") {
        match scaf_val {
            Value::String(s) => {
                if let Some(res) = inputs.iter().find(|r| r.key == *s) {
                    let utf8 = std::str::from_utf8(&res.bytes)
                        .map_err(|e| format!("Portable scaffold '{s}' is not UTF-8: {e}"))?;
                    let parsed: JsonValue = serde_json::from_str(utf8)
                        .map_err(|e| format!("Portable scaffold '{s}' JSON error: {e}"))?;
                    return Ok(Some(parsed));
                }
                let parsed: JsonValue = serde_json::from_str(s)
                    .map_err(|e| format!("Inline portable scaffold JSON error: {e}"))?;
                return Ok(Some(parsed));
            }
            other => {
                let parsed = vibe_value_to_json(other);
                return Ok(Some(parsed));
            }
        }
    }
    Ok(None)
}

fn vibe_value_to_json(v: &Value) -> JsonValue {
    match v {
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::I64(n) => JsonValue::Number((*n).into()),
        Value::U64(n) => JsonValue::Number((*n).into()),
        Value::F64(f) => serde_json::Number::from_f64(*f)
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null),
        Value::String(s) => JsonValue::String(s.clone()),
        Value::List(items) => JsonValue::Array(items.iter().map(vibe_value_to_json).collect()),
        Value::Record(map) => {
            let mut obj = serde_json::Map::new();
            for (k, val) in map {
                obj.insert(k.clone(), vibe_value_to_json(val));
            }
            JsonValue::Object(obj)
        }
        _ => JsonValue::Null,
    }
}
