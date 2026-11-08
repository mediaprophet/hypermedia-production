use super::*;
use crate::*;
use qualia_core_db::container_10d::integrity::verify_whole_file_crc32c;
use qualia_core_db::render::botanical_anatomy::{
    build_scale1_whole_organism_10d, build_scale2_stem_bark_wedge_10d,
    build_scale3_floral_and_gumnut_dissection_10d, build_scale4_leaf_venation_and_histology_10d,
    DevelopmentalStage,
};
use qualia_core_db::render::vertebrate_anatomy::{
    build_scale5_vertebrate_anatomy_10d, evaluate_anatomical_layer_peel_zero_heap,
    ZoologicalAnatomicalLayer,
};
use std::collections::BTreeMap;
use vibe::Value;

impl AssetHost<'_> {
    /// Compiles a 4-scale botanical `.10d` organ container (`Scale 1` whole organism, `Scale 2`
    /// stem/bark/cambium/xylem wedge, `Scale 3` floral/operculum/gumnut dissection, or `Scale 4`
    /// leaf venation & stomatal histology cross-section) and verifies all `.10d` integrity invariants.
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
            1 => build_scale1_whole_organism_10d(&species_id, stage)?,
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

        let mut out = BTreeMap::new();
        out.insert("scale".into(), Value::I64(scale as i64));
        out.insert("species_id".into(), Value::String(species_id));
        out.insert(
            "vertex_count".into(),
            Value::I64(organ.vertex_count() as i64),
        );
        out.insert(
            "triangle_count".into(),
            Value::I64(organ.triangle_count() as i64),
        );
        out.insert(
            "tissue_slice_count".into(),
            Value::I64(organ.tissue_slices.len() as i64),
        );
        out.insert(
            "container_byte_len".into(),
            Value::I64(organ.container_bytes.len() as i64),
        );
        out.insert("verified".into(), Value::Bool(true));

        Ok(Value::Record(out))
    }

    /// Compiles a Scale-5 comparative vertebrate 3-layer `.10d` anatomy container (`Osteology`,
    /// `Myology`, `Integument`, `17` submeshes & `17` materials) and evaluates the zero-heap
    /// anatomical layer peel operator.
    pub(super) fn compile_vertebrate_anatomy_10d(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "archetype",
                "osteology_vis_01",
                "myology_vis_01",
                "integument_vis_01",
                "explode_01",
            ],
        )?;

        let archetype_str = opt_string(a, "archetype", "macropod_saltatorial")?;
        let archetype = parse_fauna_archetype(&archetype_str)?;

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

        let mut compiled = build_scale5_vertebrate_anatomy_10d(
            archetype,
            osteology_vis_01 as f32,
            myology_vis_01 as f32,
            integument_vis_01 as f32,
            explode_01 as f32,
        );
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

        let mut out = BTreeMap::new();
        out.insert("archetype".into(), Value::String(archetype_str));
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
            Value::I64(compiled.container_bytes.len() as i64),
        );
        out.insert(
            "peeled_vertex_count".into(),
            Value::I64(peeled_vertex_count as i64),
        );

        Ok(Value::Record(out))
    }
}
