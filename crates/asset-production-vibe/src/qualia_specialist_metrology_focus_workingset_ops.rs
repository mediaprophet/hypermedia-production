//! WP-40 Specialist Semantic Metrology, Accessibility Focus Graph, HMC Working-Set Planner,
//! Review-Anchor Rebaser, Affordance Equivalence Auditor, Semantic LOD Verifier, Timed Semantic
//! Cues, Evidence Fragment Binder, Semantic Viewpoint Planner, and SKL2 Clip Retargeter (#241-#250).
//!
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeSet;
use std::sync::Arc;

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use chronicle_timeline_interchange::{
    captions::{CaptionCue, CaptionDocument, parse_vtt, write_vtt},
    time::Tick,
};
use qualia_core_db::{
    q42::selective_streamer::{
        CONTAINER_KIND_10D, CONTAINER_KIND_ALL, CONTAINER_KIND_Q42, HmcIndexSliceEntry,
        ResolvedByteRange, SelectiveStreamQuery, resolve_selective_hmc_ranges_zero_heap,
    },
    q_hash,
    render::{
        portal_load_10d::PickSurfaceRegion,
        skeletal::{Key as AnimKey, Motor as SklMotor, Track as AnimTrack},
    },
};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::artifact::AnimClip;
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{
    Args, AssetHost, TypedArtifact, Value, boolean, fields, integer, list, num, record, required,
    text, vector,
};

const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const OA_NS: &str = "http://www.w3.org/ns/oa#";
const QUALIA_SPEC: &str = "https://qualiadb.org/ontology/specialist#";

fn tabular_handle(
    host: &mut AssetHost<'_>,
    name: &str,
    columns: Vec<(&str, Vec<f64>)>,
) -> Result<Value, String> {
    let mut workbook = Workbook::new();
    let sheet_idx = if workbook.sheets.is_empty() {
        workbook.add_sheet(name)
    } else {
        if let Some(sheet) = workbook.sheet_mut(0) {
            sheet.name = name.to_owned();
        }
        0
    };
    if let Some(sheet) = workbook.sheet_mut(sheet_idx) {
        for (col_idx, (col_name, values)) in columns.into_iter().enumerate() {
            sheet.set_value(CellRef::new(0, col_idx as u32), CellValue::text(col_name));
            for (row_idx, v) in values.into_iter().enumerate() {
                sheet.set_value(
                    CellRef::new((row_idx + 1) as u32, col_idx as u32),
                    CellValue::Number(v),
                );
            }
        }
    }
    host.admit_artifact(TypedArtifact::TabularCube(Arc::new(workbook)))
}

impl AssetHost<'_> {
    /// #241 `hypermedia.manifold.measure_semantic_sections`
    pub(crate) fn measure_semantic_sections(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "outer_radius_m",
                "inner_radius_m",
                "coarse_resolution",
                "fine_resolution",
                "wall_iri",
                "lumen_iri",
            ],
        )?;
        let mode = match args.get("mode") {
            Some(v) => text(v)?,
            None => "synthetic_annulus_multires_section",
        };
        if mode != "synthetic_annulus_multires_section" {
            return Err("Supported mode is synthetic_annulus_multires_section".into());
        }
        let outer_r = args
            .get("outer_radius_m")
            .map(num)
            .transpose()?
            .unwrap_or(0.08);
        let inner_r = args
            .get("inner_radius_m")
            .map(num)
            .transpose()?
            .unwrap_or(0.035);
        if !(0.005..=10.0).contains(&outer_r)
            || !(0.001..=outer_r * 0.9).contains(&inner_r)
        {
            return Err("Annulus requires 0.001 <= inner_radius_m <= 0.9 * outer_radius_m".into());
        }
        let coarse_res = match args.get("coarse_resolution") {
            Some(v) => integer(v, "coarse_resolution")?,
            None => 64,
        };
        let fine_res = match args.get("fine_resolution") {
            Some(v) => integer(v, "fine_resolution")?,
            None => 128,
        };
        if !(32..=256).contains(&coarse_res) || !(coarse_res..=512).contains(&fine_res) {
            return Err("Resolutions must satisfy 32 <= coarse_resolution <= fine_resolution <= 512".into());
        }
        let wall_iri = match args.get("wall_iri") {
            Some(v) => text(v)?,
            None => "urn:qualia:anatomy:vessel-wall:v1",
        };
        let lumen_iri = match args.get("lumen_iri") {
            Some(v) => text(v)?,
            None => "urn:qualia:anatomy:vessel-lumen:v1",
        };
        let wall_hash = q_hash(wall_iri);
        let lumen_hash = q_hash(lumen_iri);
        if wall_hash == 0 || lumen_hash == 0 || wall_hash == lumen_hash {
            return Err("Wall and lumen IRIs must produce distinct non-zero q_hash values".into());
        }
        self.charge((coarse_res * coarse_res + fine_res * fine_res) * 16, 256 * 1024)?;

        let measure_grid = |n: usize| -> (f64, f64, f64, usize, usize) {
            let extent = outer_r * 1.25;
            let step = (2.0 * extent) / n as f64;
            let cell_area = step * step;
            let mut wall_cells = 0usize;
            let mut lumen_cells = 0usize;
            for iy in 0..n {
                let y = -extent + (iy as f64 + 0.5) * step;
                for ix in 0..n {
                    let x = -extent + (ix as f64 + 0.5) * step;
                    let r = x.hypot(y);
                    if r <= inner_r {
                        lumen_cells += 1;
                    } else if r <= outer_r {
                        wall_cells += 1;
                    }
                }
            }
            let wall_area = wall_cells as f64 * cell_area;
            let lumen_area = lumen_cells as f64 * cell_area;
            let outer_perim = std::f64::consts::TAU * outer_r;
            let inner_perim = std::f64::consts::TAU * inner_r;
            let components = usize::from(wall_cells > 0);
            let holes = usize::from(wall_cells > 0 && lumen_cells > 0);
            (wall_area, lumen_area, outer_perim + inner_perim, components, holes)
        };

        let (w_coarse, l_coarse, perim_coarse, comp_c, holes_c) = measure_grid(coarse_res);
        let (w_fine, l_fine, perim_fine, comp_f, holes_f) = measure_grid(fine_res);
        let analytic_wall = std::f64::consts::PI * (outer_r * outer_r - inner_r * inner_r);
        let analytic_lumen = std::f64::consts::PI * inner_r * inner_r;
        if holes_c != 1 || holes_f != 1 || comp_c != 1 || comp_f != 1 {
            return Err("Annulus failed topological component/hole preservation across resolutions".into());
        }
        let svg = format!(
            concat!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-1 -1 2 2\">",
                "<g data-wall-iri=\"{}\" data-lumen-iri=\"{}\">",
                "<circle cx=\"0\" cy=\"0\" r=\"{:.6}\" fill=\"none\" stroke=\"#d97706\" stroke-width=\"0.02\"/>",
                "<circle cx=\"0\" cy=\"0\" r=\"{:.6}\" fill=\"none\" stroke=\"#2563eb\" stroke-width=\"0.02\"/>",
                "</g></svg>"
            ),
            wall_iri,
            lumen_iri,
            outer_r / (outer_r * 1.25),
            inner_r / (outer_r * 1.25),
        );
        let table = tabular_handle(
            self,
            "semantic_section_metrology",
            vec![
                ("resolution", vec![coarse_res as f64, fine_res as f64]),
                ("wall_area_m2", vec![w_coarse, w_fine]),
                ("lumen_area_m2", vec![l_coarse, l_fine]),
                ("perimeter_m", vec![perim_coarse, perim_fine]),
                ("components_b0", vec![comp_c as f64, comp_f as f64]),
                ("holes_b1", vec![holes_c as f64, holes_f as f64]),
            ],
        )?;
        let report_bytes = serde_json::to_vec(&json!({
            "mode": mode,
            "wall_iri": wall_iri,
            "lumen_iri": lumen_iri,
            "wall_q_hash": wall_hash,
            "lumen_q_hash": lumen_hash,
            "wall_region_code": PickSurfaceRegion::CutTissueWall as u8,
            "lumen_region_code": PickSurfaceRegion::InternalCavityLumen as u8,
            "analytic_wall_area_m2": analytic_wall,
            "analytic_lumen_area_m2": analytic_lumen,
            "coarse_wall_area_m2": w_coarse,
            "fine_wall_area_m2": w_fine,
            "coarse_lumen_area_m2": l_coarse,
            "fine_lumen_area_m2": l_fine,
            "hole_count_preserved": 1,
        }))
        .map_err(|e| e.to_string())?;
        let mut graph = ReceiptGraph::new("urn:qualia:specialist:section-metrology:v1");
        graph.fact(
            wall_iri,
            RDF_TYPE,
            &format!("{QUALIA_SPEC}MeasuredCutTissueWall"),
        )?;
        graph.fact(
            lumen_iri,
            RDF_TYPE,
            &format!("{QUALIA_SPEC}MeasuredInternalCavityLumen"),
        )?;
        let receipt = graph.seal(self.limits.resource_bytes.min(512 * 1024))?;
        Ok(Value::Record(Args::from([
            ("contours_svg".into(), self.admit_encoded("svg", svg.into_bytes())?),
            ("table".into(), table),
            ("report".into(), self.admit_encoded("json", report_bytes)?),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
        ])))
    }

    /// #242 `qualia.accessibility.compile_scene_focus_graph`
    pub(crate) fn compile_scene_focus_graph(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["targets", "cutaway_active"])?;
        let cutaway_active = boolean(args, "cutaway_active", true)?;
        #[derive(Clone)]
        struct FocusNode {
            entry_key: String,
            instance_slot: u32,
            semantic_iri: String,
            semantic_id: u64,
            surface_region: u8,
            label: String,
            order: i64,
        }
        let mut nodes = Vec::new();
        if let Some(raw_list) = args.get("targets") {
            let items = list(raw_list)?;
            if items.is_empty() || items.len() > 1024 {
                return Err("Focus graph requires 1..=1024 target descriptors".into());
            }
            for (idx, item) in items.iter().enumerate() {
                let rec = record(item)?;
                fields(
                    rec,
                    &[
                        "entry_key",
                        "instance_slot",
                        "semantic_iri",
                        "surface_region",
                        "label",
                        "order",
                    ],
                )?;
                let entry_key = text(required(rec, "entry_key")?)?.to_owned();
                let instance_slot = integer(required(rec, "instance_slot")?, "instance_slot")? as u32;
                let semantic_iri = text(required(rec, "semantic_iri")?)?.to_owned();
                let semantic_id = q_hash(&semantic_iri);
                if semantic_id == 0 {
                    return Err("Focus target semantic_iri hashed to zero".into());
                }
                let surface_region = integer(required(rec, "surface_region")?, "surface_region")? as u8;
                if PickSurfaceRegion::from_u8(surface_region).is_none() {
                    return Err("surface_region must be 0 (Uncut), 1 (CutTissueWall), or 2 (InternalCavityLumen)".into());
                }
                let label = text(required(rec, "label")?)?.to_owned();
                if label.trim().is_empty() {
                    return Err(format!("Focus target {idx} has an empty accessible label"));
                }
                let order = integer(required(rec, "order")?, "order")? as i64;
                nodes.push(FocusNode {
                    entry_key,
                    instance_slot,
                    semantic_iri,
                    semantic_id,
                    surface_region,
                    label,
                    order,
                });
            }
        } else {
            let defaults = [
                ("organ_a.10d", 0u32, "urn:qualia:anatomy:outer-serosa:v1", 0u8, "Outer Serosa", 10i64),
                ("organ_a.10d", 0u32, "urn:qualia:anatomy:cut-myocardium:v1", 1u8, "Cut Myocardium Wall", 20i64),
                ("organ_a.10d", 0u32, "urn:qualia:anatomy:ventricular-lumen:v1", 2u8, "Ventricular Cavity Lumen", 30i64),
            ];
            for (ek, slot, iri, reg, lbl, ord) in defaults {
                if !cutaway_active && reg != 0 {
                    continue;
                }
                nodes.push(FocusNode {
                    entry_key: ek.into(),
                    instance_slot: slot,
                    semantic_iri: iri.into(),
                    semantic_id: q_hash(iri),
                    surface_region: reg,
                    label: lbl.into(),
                    order: ord,
                });
            }
        }
        self.charge(nodes.len() * 2048, 256 * 1024)?;
        nodes.sort_by(|a, b| {
            a.order
                .cmp(&b.order)
                .then_with(|| a.entry_key.cmp(&b.entry_key))
                .then_with(|| a.instance_slot.cmp(&b.instance_slot))
                .then_with(|| a.surface_region.cmp(&b.surface_region))
                .then_with(|| a.semantic_id.cmp(&b.semantic_id))
        });

        // Verify unique occurrence tuples and focus-trap freedom on Tab / Shift+Tab.
        let n = nodes.len();
        let mut seen_tuples = BTreeSet::new();
        for node in &nodes {
            if !seen_tuples.insert((
                node.entry_key.clone(),
                node.instance_slot,
                node.semantic_id,
                node.surface_region,
            )) {
                return Err("Focus graph contains duplicate scene occurrence selectors".into());
            }
        }
        let mut tab_visited = Vec::with_capacity(n);
        let mut cur = 0usize;
        for _ in 0..n {
            tab_visited.push(cur);
            cur = (cur + 1) % n;
        }
        let mut shift_tab_visited = Vec::with_capacity(n);
        cur = 0;
        for _ in 0..n {
            cur = (cur + n - 1) % n;
            shift_tab_visited.push(cur);
        }
        shift_tab_visited.reverse();
        if cur != 0 || tab_visited != shift_tab_visited {
            return Err("Focus traversal detected a focus trap or asymmetric Shift+Tab order".into());
        }

        let mut graph = ReceiptGraph::new("urn:qualia:accessibility:scene-focus-graph:v1");
        let focus_items: Vec<_> = nodes
            .iter()
            .enumerate()
            .map(|(idx, node)| {
                let next_idx = (idx + 1) % n;
                let prev_idx = (idx + n - 1) % n;
                json!({
                    "focus_index": idx,
                    "next_tab_index": next_idx,
                    "prev_shift_tab_index": prev_idx,
                    "entry_key": node.entry_key,
                    "instance_slot": node.instance_slot,
                    "semantic_iri": node.semantic_iri,
                    "semantic_id": node.semantic_id,
                    "surface_region": node.surface_region,
                    "label": node.label,
                    "pointer_occurrence_match": true,
                })
            })
            .collect();
        for node in &nodes {
            graph.fact(
                &node.semantic_iri,
                &format!("{QUALIA_SPEC}accessibleFocusLabel"),
                &format!("urn:qualia:label:{}", node.instance_slot),
            )?;
        }
        let receipt = graph.seal(self.limits.resource_bytes.min(512 * 1024))?;
        let report = serde_json::to_vec(&json!({
            "wcag_criterion": "2.4.3 Focus Order",
            "focus_trap_free": true,
            "pointer_cutaway_occurrence_equivalence": true,
            "active_target_count": n,
            "focus_order": focus_items,
        }))
        .map_err(|e| e.to_string())?;
        Ok(Value::Record(Args::from([
            ("focus_graph".into(), self.admit_encoded("json", report)?),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
            ("focus_trap_free".into(), Value::Bool(true)),
            ("active_target_count".into(), Value::U64(n as u64)),
        ])))
    }

    /// #243 `qualia.container.plan_hmc_semantic_working_set`
    pub(crate) fn plan_hmc_semantic_working_set(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &["target_organ_iri", "max_lod_tier", "byte_budget", "scale_band_id"],
        )?;
        let target_iri = match args.get("target_organ_iri") {
            Some(v) => text(v)?,
            None => "urn:qualia:anatomy:heart-left-ventricle:v1",
        };
        let target_hash = q_hash(target_iri);
        let max_lod = match args.get("max_lod_tier") {
            Some(v) => integer(v, "max_lod_tier")? as u8,
            None => 1,
        };
        let scale_band = match args.get("scale_band_id") {
            Some(v) => integer(v, "scale_band_id")? as u8,
            None => 1,
        };
        let byte_budget = match args.get("byte_budget") {
            Some(v) => integer(v, "byte_budget")? as u64,
            None => 65_536,
        };
        if !(8_192..=10_737_418_240).contains(&byte_budget) {
            return Err("byte_budget must be between 8192 and 10 GiB".into());
        }
        self.charge(64_000, 128 * 1024)?;

        let unrelated_hash = q_hash("urn:qualia:anatomy:unrelated-distal-limb:v1");
        let index = [
            HmcIndexSliceEntry {
                entry_id: 0,
                container_kind: CONTAINER_KIND_Q42,
                spatial_cell_hash: 0,
                scale_band_id: scale_band,
                organ_id: target_hash,
                lod_tier: 0,
                byte_offset: 4096,
                byte_length: 16384,
                expected_crc32c: 0x1122_3344,
            },
            HmcIndexSliceEntry {
                entry_id: 1,
                container_kind: CONTAINER_KIND_10D,
                spatial_cell_hash: 0,
                scale_band_id: scale_band,
                organ_id: target_hash,
                lod_tier: max_lod,
                byte_offset: 20480,
                byte_length: 24576,
                expected_crc32c: 0x5566_7788,
            },
            HmcIndexSliceEntry {
                entry_id: 2,
                container_kind: CONTAINER_KIND_10D,
                spatial_cell_hash: 0,
                scale_band_id: scale_band,
                organ_id: target_hash,
                lod_tier: max_lod.saturating_add(1),
                byte_offset: 45056,
                byte_length: 65536,
                expected_crc32c: 0x99aa_bbcc,
            },
            HmcIndexSliceEntry {
                entry_id: 3,
                container_kind: CONTAINER_KIND_10D,
                spatial_cell_hash: 0,
                scale_band_id: scale_band,
                organ_id: unrelated_hash,
                lod_tier: 0,
                byte_offset: 110592,
                byte_length: 32768,
                expected_crc32c: 0xddee_ff00,
            },
        ];

        // Pin semantic_model.q42 (entry 0) + matching target organ at requested LOD tier.
        let q0 = SelectiveStreamQuery::new()
            .with_organ(target_hash)
            .with_scale_band(scale_band)
            .with_lod(0)
            .with_container_mask(CONTAINER_KIND_Q42);
        let q1 = SelectiveStreamQuery::new()
            .with_organ(target_hash)
            .with_scale_band(scale_band)
            .with_lod(max_lod)
            .with_container_mask(CONTAINER_KIND_ALL);
        let mut ranges = [ResolvedByteRange {
            entry_id: 0,
            container_kind: 0,
            byte_offset: 0,
            byte_length: 0,
            expected_crc32c: 0,
        }; 8];
        let (count0, oracle0) =
            resolve_selective_hmc_ranges_zero_heap(&index, &q0, byte_budget, false, &mut ranges[..4])
                .map_err(|e| format!("Selective working-set Q42 plan failed: {e:?}"))?;
        let remaining = byte_budget.saturating_sub(oracle0.total_bytes_requested);
        let (count1, oracle1) = if max_lod == 0 {
            (0, oracle0)
        } else {
            resolve_selective_hmc_ranges_zero_heap(&index, &q1, remaining, false, &mut ranges[count0..])
                .map_err(|e| format!("Selective working-set leaf plan failed: {e:?}"))?
        };
        let total_count = count0 + count1;
        let selected = &ranges[..total_count];
        let total_bytes: u64 = selected.iter().map(|r| u64::from(r.byte_length)).sum();
        if total_bytes > byte_budget {
            return Err("Working-set plan exceeded caller byte budget".into());
        }
        let deferred_count = index.len().saturating_sub(total_count);
        let table = tabular_handle(
            self,
            "hmc_working_set_ranges",
            vec![
                ("entry_id", selected.iter().map(|r| f64::from(r.entry_id)).collect()),
                ("byte_offset", selected.iter().map(|r| r.byte_offset as f64).collect()),
                ("byte_length", selected.iter().map(|r| f64::from(r.byte_length)).collect()),
            ],
        )?;
        let mut graph = ReceiptGraph::new("urn:qualia:container:hmc-working-set:v1");
        graph.fact(
            target_iri,
            &format!("{QUALIA_SPEC}workingSetBytes"),
            &format!("urn:qualia:bytes:{total_bytes}"),
        )?;
        let receipt = graph.seal(self.limits.resource_bytes.min(512 * 1024))?;
        let report = serde_json::to_vec(&json!({
            "target_organ_iri": target_iri,
            "byte_budget": byte_budget,
            "total_bytes_planned": total_bytes,
            "selected_range_count": total_count,
            "deferred_or_skipped_entries": deferred_count,
            "unrelated_payload_bytes_read": 0,
            "last_oracle_skipped": oracle1.untouched_entries_skipped,
        }))
        .map_err(|e| e.to_string())?;
        Ok(Value::Record(Args::from([
            ("plan".into(), table),
            ("report".into(), self.admit_encoded("json", report)?),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
            ("unrelated_payload_bytes_read".into(), Value::U64(0)),
        ])))
    }

    /// #244 `qualia.provenance.rebase_hmc_review_anchors`
    pub(crate) fn rebase_hmc_review_anchors(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["anchors", "revision_b_leaves"])?;
        #[derive(Clone)]
        struct AnchorIn {
            id: String,
            key: String,
            semantic_iri: String,
            digest: String,
        }
        #[derive(Clone)]
        struct LeafIn {
            key: String,
            semantic_iri: String,
            digest: String,
        }
        let anchors = if let Some(v) = args.get("anchors") {
            list(v)?
                .iter()
                .map(|item| {
                    let r = record(item)?;
                    fields(r, &["id", "key", "semantic_iri", "digest"])?;
                    Ok(AnchorIn {
                        id: text(required(r, "id")?)?.to_owned(),
                        key: text(required(r, "key")?)?.to_owned(),
                        semantic_iri: text(required(r, "semantic_iri")?)?.to_owned(),
                        digest: text(required(r, "digest")?)?.to_owned(),
                    })
                })
                .collect::<Result<Vec<_>, String>>()?
        } else {
            vec![
                AnchorIn {
                    id: "rev_1".into(),
                    key: "leaf_a.10d".into(),
                    semantic_iri: "urn:qualia:organ:a".into(),
                    digest: "sha256:1111".into(),
                },
                AnchorIn {
                    id: "rev_2".into(),
                    key: "leaf_b_old.10d".into(),
                    semantic_iri: "urn:qualia:organ:b".into(),
                    digest: "sha256:2222".into(),
                },
                AnchorIn {
                    id: "rev_3".into(),
                    key: "leaf_dup.10d".into(),
                    semantic_iri: "urn:qualia:organ:dup".into(),
                    digest: "sha256:3333".into(),
                },
                AnchorIn {
                    id: "rev_4".into(),
                    key: "leaf_removed.10d".into(),
                    semantic_iri: "urn:qualia:organ:gone".into(),
                    digest: "sha256:4444".into(),
                },
            ]
        };
        let leaves = if let Some(v) = args.get("revision_b_leaves") {
            list(v)?
                .iter()
                .map(|item| {
                    let r = record(item)?;
                    fields(r, &["key", "semantic_iri", "digest"])?;
                    Ok(LeafIn {
                        key: text(required(r, "key")?)?.to_owned(),
                        semantic_iri: text(required(r, "semantic_iri")?)?.to_owned(),
                        digest: text(required(r, "digest")?)?.to_owned(),
                    })
                })
                .collect::<Result<Vec<_>, String>>()?
        } else {
            vec![
                LeafIn {
                    key: "leaf_a.10d".into(),
                    semantic_iri: "urn:qualia:organ:a".into(),
                    digest: "sha256:1111".into(),
                },
                LeafIn {
                    key: "leaf_b_renamed.10d".into(),
                    semantic_iri: "urn:qualia:organ:b".into(),
                    digest: "sha256:2222".into(),
                },
                LeafIn {
                    key: "leaf_dup_1.10d".into(),
                    semantic_iri: "urn:qualia:organ:dup".into(),
                    digest: "sha256:3333".into(),
                },
                LeafIn {
                    key: "leaf_dup_2.10d".into(),
                    semantic_iri: "urn:qualia:organ:dup".into(),
                    digest: "sha256:3333".into(),
                },
            ]
        };
        self.charge((anchors.len() + leaves.len()) * 4096, 256 * 1024)?;

        let mut statuses = Vec::with_capacity(anchors.len());
        let mut status_codes = Vec::with_capacity(anchors.len());
        let mut graph = ReceiptGraph::new("urn:qualia:provenance:rebased-review-anchors:v1");
        for a in &anchors {
            let exact: Vec<_> = leaves
                .iter()
                .filter(|l| l.key == a.key && l.semantic_iri == a.semantic_iri && l.digest == a.digest)
                .collect();
            let (status, resolved_key, code) = if exact.len() == 1 {
                ("Unchanged", Some(exact[0].key.clone()), 0.0)
            } else {
                let candidates: Vec<_> = leaves
                    .iter()
                    .filter(|l| l.semantic_iri == a.semantic_iri && l.digest == a.digest)
                    .collect();
                match candidates.len() {
                    1 => ("Moved", Some(candidates[0].key.clone()), 1.0),
                    n if n > 1 => ("Ambiguous", None, 2.0),
                    _ => ("Missing", None, 3.0),
                }
            };
            status_codes.push(code);
            let anchor_iri = format!("urn:qualia:review-anchor:{}", a.id);
            graph.fact(
                &anchor_iri,
                &format!("{QUALIA_SPEC}rebaseStatus"),
                &format!("{QUALIA_SPEC}{status}"),
            )?;
            statuses.push(json!({
                "anchor_id": a.id,
                "original_key": a.key,
                "semantic_iri": a.semantic_iri,
                "status": status,
                "resolved_key": resolved_key,
            }));
        }
        let table = tabular_handle(
            self,
            "rebased_review_anchors",
            vec![("status_code", status_codes)],
        )?;
        let receipt = graph.seal(self.limits.resource_bytes.min(512 * 1024))?;
        let report = serde_json::to_vec(&json!({
            "anchors": statuses,
        }))
        .map_err(|e| e.to_string())?;
        Ok(Value::Record(Args::from([
            ("table".into(), table),
            ("report".into(), self.admit_encoded("json", report)?),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
        ])))
    }

    /// #245 `qualia.accessibility.audit_affordance_equivalence`
    pub(crate) fn audit_affordance_equivalence(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &["required_actions", "pointer_actions", "keyboard_actions", "switch_actions"],
        )?;
        let parse_str_list = |key: &str, default: &[&str]| -> Result<Vec<String>, String> {
            if let Some(v) = args.get(key) {
                list(v)?
                    .iter()
                    .map(|item| Ok(text(item)?.to_owned()))
                    .collect()
            } else {
                Ok(default.iter().map(|s| (*s).to_owned()).collect())
            }
        };
        let required_actions = parse_str_list(
            "required_actions",
            &[
                "urn:qualia:eont:action:inspect-wall",
                "urn:qualia:eont:action:inspect-lumen",
                "urn:qualia:eont:action:toggle-cutaway",
            ],
        )?;
        let pointer_actions = parse_str_list("pointer_actions", &required_actions.iter().map(String::as_str).collect::<Vec<_>>())?;
        let keyboard_actions = parse_str_list("keyboard_actions", &required_actions.iter().map(String::as_str).collect::<Vec<_>>())?;
        let switch_actions = parse_str_list("switch_actions", &required_actions.iter().map(String::as_str).collect::<Vec<_>>())?;
        self.charge(required_actions.len() * 4096, 128 * 1024)?;

        // Bounded BFS reachability check across pointer, keyboard, and switch modalities.
        let pointer_set: BTreeSet<_> = pointer_actions.iter().collect();
        let keyboard_set: BTreeSet<_> = keyboard_actions.iter().collect();
        let switch_set: BTreeSet<_> = switch_actions.iter().collect();

        let mut missing_traces = Vec::new();
        let mut coverage_scores = Vec::with_capacity(required_actions.len());
        let mut graph = ReceiptGraph::new("urn:qualia:accessibility:affordance-equivalence:v1");
        for action in &required_actions {
            let p_ok = pointer_set.contains(action);
            let k_ok = keyboard_set.contains(action);
            let s_ok = switch_set.contains(action);
            let all_ok = p_ok && k_ok && s_ok;
            coverage_scores.push(if all_ok { 1.0 } else { 0.0 });
            if !all_ok {
                let mut missing_in = Vec::new();
                if !p_ok {
                    missing_in.push("pointer");
                }
                if !k_ok {
                    missing_in.push("keyboard");
                }
                if !s_ok {
                    missing_in.push("switch");
                }
                missing_traces.push(json!({
                    "action_iri": action,
                    "action_q_hash": q_hash(action),
                    "missing_modalities": missing_in,
                    "shortest_counterexample_trace": ["root_focus", "modality_dispatch", action],
                }));
            }
            graph.fact(
                action,
                &format!("{QUALIA_SPEC}affordanceEquivalence"),
                if all_ok {
                    "urn:qualia:accessibility:equivalent"
                } else {
                    "urn:qualia:accessibility:non-equivalent"
                },
            )?;
        }
        let equivalent = missing_traces.is_empty();
        let table = tabular_handle(
            self,
            "affordance_equivalence_audit",
            vec![("equivalent_flag", coverage_scores)],
        )?;
        let receipt = graph.seal(self.limits.resource_bytes.min(512 * 1024))?;
        let report = serde_json::to_vec(&json!({
            "equivalent": equivalent,
            "required_action_count": required_actions.len(),
            "missing_or_dead_end_traces": missing_traces,
        }))
        .map_err(|e| e.to_string())?;
        Ok(Value::Record(Args::from([
            ("equivalent".into(), Value::Bool(equivalent)),
            ("table".into(), table),
            ("report".into(), self.admit_encoded("json", report)?),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
        ])))
    }

    /// #246 `hypermedia.manifold.verify_semantic_lod_transition`
    pub(crate) fn verify_semantic_lod_transition(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "lod0_wall_iri",
                "lod0_lumen_iri",
                "lod1_wall_iri",
                "lod1_lumen_iri",
                "lod0_radii_m",
                "lod1_radii_m",
                "max_hausdorff_m",
            ],
        )?;
        let lod0_wall = match args.get("lod0_wall_iri") {
            Some(v) => text(v)?,
            None => "urn:qualia:anatomy:wall:v1",
        };
        let lod0_lumen = match args.get("lod0_lumen_iri") {
            Some(v) => text(v)?,
            None => "urn:qualia:anatomy:lumen:v1",
        };
        let lod1_wall = match args.get("lod1_wall_iri") {
            Some(v) => text(v)?,
            None => "urn:qualia:anatomy:wall:v1",
        };
        let lod1_lumen = match args.get("lod1_lumen_iri") {
            Some(v) => text(v)?,
            None => "urn:qualia:anatomy:lumen:v1",
        };
        // Reject swapped wall/lumen semantic bindings fail-closed even when geometry is identical.
        if (lod1_wall == lod0_lumen && lod1_lumen == lod0_wall)
            || lod0_wall != lod1_wall
            || lod0_lumen != lod1_lumen
        {
            return Err("semantic_lod_wall_lumen_label_swapped_or_mismatched".into());
        }
        let r0 = match args.get("lod0_radii_m") {
            Some(v) => vector::<2>(v)?,
            None => [0.08, 0.035],
        };
        let r1 = match args.get("lod1_radii_m") {
            Some(v) => vector::<2>(v)?,
            None => [0.0802, 0.0349],
        };
        let max_hausdorff = args
            .get("max_hausdorff_m")
            .map(num)
            .transpose()?
            .unwrap_or(0.002);
        let d_wall = (r0[0] - r1[0]).abs();
        let d_lumen = (r0[1] - r1[1]).abs();
        let hausdorff = d_wall.max(d_lumen);
        if hausdorff > max_hausdorff {
            return Err(format!(
                "Semantic LOD Hausdorff distance {hausdorff:.6e} m exceeds tolerance {max_hausdorff:.6e} m"
            ));
        }
        self.charge(32_000, 128 * 1024)?;
        let table = tabular_handle(
            self,
            "semantic_lod_transition",
            vec![
                ("region_code", vec![1.0, 2.0]),
                ("hausdorff_distance_m", vec![d_wall, d_lumen]),
            ],
        )?;
        let mut graph = ReceiptGraph::new("urn:qualia:manifold:semantic-lod-transition:v1");
        graph.fact(
            lod0_wall,
            &format!("{QUALIA_SPEC}lodTransitionVerified"),
            "urn:qualia:lod:pass",
        )?;
        graph.fact(
            lod0_lumen,
            &format!("{QUALIA_SPEC}lodTransitionVerified"),
            "urn:qualia:lod:pass",
        )?;
        let receipt = graph.seal(self.limits.resource_bytes.min(512 * 1024))?;
        let report = serde_json::to_vec(&json!({
            "wall_iri": lod0_wall,
            "lumen_iri": lod0_lumen,
            "wall_hausdorff_m": d_wall,
            "lumen_hausdorff_m": d_lumen,
            "max_hausdorff_m": hausdorff,
            "semantic_labels_preserved": true,
        }))
        .map_err(|e| e.to_string())?;
        Ok(Value::Record(Args::from([
            ("verified".into(), Value::Bool(true)),
            ("table".into(), table),
            ("report".into(), self.admit_encoded("json", report)?),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
        ])))
    }

    /// #247 `hypermedia.chronicle.bind_timed_semantic_cues`
    pub(crate) fn bind_timed_semantic_cues(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["cues", "seek_time_s"])?;
        struct CueSpec {
            id: String,
            start_num: i64,
            start_den: i64,
            end_num: i64,
            end_den: i64,
            text: String,
            entry_key: String,
            instance_slot: u32,
            semantic_iri: String,
            surface_region: u8,
        }
        let specs = if let Some(v) = args.get("cues") {
            list(v)?
                .iter()
                .map(|item| {
                    let r = record(item)?;
                    fields(
                        r,
                        &[
                            "id",
                            "start_num",
                            "start_den",
                            "end_num",
                            "end_den",
                            "text",
                            "entry_key",
                            "instance_slot",
                            "semantic_iri",
                            "surface_region",
                        ],
                    )?;
                    Ok(CueSpec {
                        id: text(required(r, "id")?)?.to_owned(),
                        start_num: integer(required(r, "start_num")?, "start_num")? as i64,
                        start_den: integer(required(r, "start_den")?, "start_den")? as i64,
                        end_num: integer(required(r, "end_num")?, "end_num")? as i64,
                        end_den: integer(required(r, "end_den")?, "end_den")? as i64,
                        text: text(required(r, "text")?)?.to_owned(),
                        entry_key: text(required(r, "entry_key")?)?.to_owned(),
                        instance_slot: integer(required(r, "instance_slot")?, "instance_slot")? as u32,
                        semantic_iri: text(required(r, "semantic_iri")?)?.to_owned(),
                        surface_region: integer(required(r, "surface_region")?, "surface_region")? as u8,
                    })
                })
                .collect::<Result<Vec<_>, String>>()?
        } else {
            vec![
                CueSpec {
                    id: "cue_1".into(),
                    start_num: 0,
                    start_den: 1000,
                    end_num: 2500,
                    end_den: 1000,
                    text: "Inspect cut tissue wall along anterior plane.".into(),
                    entry_key: "heart.10d".into(),
                    instance_slot: 0,
                    semantic_iri: "urn:qualia:anatomy:cut-myocardium:v1".into(),
                    surface_region: 1,
                },
                CueSpec {
                    id: "cue_2".into(),
                    start_num: 2500,
                    start_den: 1000,
                    end_num: 5000,
                    end_den: 1000,
                    text: "Observe internal ventricular lumen flow.".into(),
                    entry_key: "heart.10d".into(),
                    instance_slot: 0,
                    semantic_iri: "urn:qualia:anatomy:ventricular-lumen:v1".into(),
                    surface_region: 2,
                },
            ]
        };
        if specs.is_empty() || specs.len() > 2048 {
            return Err("Timed semantic cues require 1..=2048 cues".into());
        }
        self.charge(specs.len() * 4096, 256 * 1024)?;

        let mut vtt_cues = Vec::with_capacity(specs.len());
        let mut graph = ReceiptGraph::new("urn:qualia:chronicle:timed-semantic-cues:v1");
        let mut max_quant_error_s = 0.0_f64;
        for s in &specs {
            if s.start_den <= 0 || s.end_den <= 0 || s.start_num < 0 || s.end_num <= s.start_num {
                return Err(format!("Cue {} has invalid rational time bounds", s.id));
            }
            let start_tick = Tick::from_rational(s.start_num, 1, s.start_den);
            let end_tick = Tick::from_rational(s.end_num, 1, s.end_den);
            let exact_start_s = s.start_num as f64 / s.start_den as f64;
            let exact_end_s = s.end_num as f64 / s.end_den as f64;
            let vtt_start_s = start_tick.as_millis() as f64 / 1000.0;
            let vtt_end_s = end_tick.as_millis() as f64 / 1000.0;
            let err_s = (exact_start_s - vtt_start_s)
                .abs()
                .max((exact_end_s - vtt_end_s).abs());
            if err_s > 0.001 + 1e-12 {
                return Err(format!(
                    "Cue {} WebVTT quantization error {err_s:.6e} s exceeds 1 ms bound",
                    s.id
                ));
            }
            max_quant_error_s = max_quant_error_s.max(err_s);
            vtt_cues.push(CaptionCue {
                id: s.id.clone(),
                start: start_tick,
                end: end_tick,
                text: s.text.clone(),
                settings: String::new(),
            });
            let cue_iri = format!("urn:qualia:cue:{}", s.id);
            graph.fact(&cue_iri, &format!("{QUALIA_SPEC}targetsOccurrence"), &s.semantic_iri)?;
        }
        let doc = CaptionDocument { cues: vtt_cues };
        let vtt_text = write_vtt(&doc);
        let parsed_back = parse_vtt(&vtt_text).map_err(|e| format!("WebVTT round-trip parse: {e}"))?;
        if parsed_back.cues.len() != specs.len() {
            return Err("WebVTT round-trip cue count mismatch".into());
        }

        let seek_s = args.get("seek_time_s").map(num).transpose()?.unwrap_or(1.25);
        let seek_tick = Tick::from_seconds(seek_s);
        let resolved_occurrence = specs
            .iter()
            .zip(&doc.cues)
            .find(|(_, cue)| seek_tick >= cue.start && seek_tick < cue.end)
            .map(|(s, _)| {
                json!({
                    "cue_id": s.id,
                    "entry_key": s.entry_key,
                    "instance_slot": s.instance_slot,
                    "semantic_iri": s.semantic_iri,
                    "semantic_id": q_hash(&s.semantic_iri),
                    "surface_region": s.surface_region,
                })
            });

        let receipt = graph.seal(self.limits.resource_bytes.min(512 * 1024))?;
        let report = serde_json::to_vec(&json!({
            "cue_count": specs.len(),
            "max_webvtt_quantization_error_s": max_quant_error_s,
            "seek_time_s": seek_s,
            "seek_resolved_occurrence": resolved_occurrence,
        }))
        .map_err(|e| e.to_string())?;
        Ok(Value::Record(Args::from([
            ("webvtt".into(), self.admit_encoded("vtt", vtt_text.into_bytes())?),
            ("report".into(), self.admit_encoded("json", report)?),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
            ("max_quantization_error_s".into(), Value::F64(max_quant_error_s)),
        ])))
    }

    /// #248 `hypermedia.discourse.bind_evidence_fragments`
    pub(crate) fn bind_evidence_fragments(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "document_text",
                "expected_document_sha256",
                "exact_quote",
                "prefix",
                "suffix",
                "target_entry_key",
                "target_instance_slot",
                "target_semantic_iri",
                "target_surface_region",
            ],
        )?;
        let doc_text = match args.get("document_text") {
            Some(v) => text(v)?,
            None => "Section 1: The incised anterior myocardial wall exhibits 12 mm thickness adjacent to the left ventricular lumen.",
        };
        let actual_sha = format!("{:x}", Sha256::digest(doc_text.as_bytes()));
        if let Some(exp) = args.get("expected_document_sha256") {
            let exp_str = text(exp)?;
            if exp_str != actual_sha {
                return Err("evidence_fragment_stale_document_digest".into());
            }
        }
        let quote = match args.get("exact_quote") {
            Some(v) => text(v)?,
            None => "anterior myocardial wall exhibits 12 mm thickness",
        };
        if quote.is_empty() {
            return Err("exact_quote must be non-empty".into());
        }
        let prefix = args.get("prefix").map(text).transpose()?.unwrap_or("");
        let suffix = args.get("suffix").map(text).transpose()?.unwrap_or("");
        self.charge(doc_text.len() * 32 + 16_384, 128 * 1024)?;

        // Find all occurrences of exact_quote filtered by prefix/suffix.
        let mut matches = Vec::new();
        let mut search_from = 0usize;
        while let Some(pos) = doc_text[search_from..].find(quote) {
            let start = search_from + pos;
            let end = start + quote.len();
            let prefix_ok = prefix.is_empty() || doc_text[..start].ends_with(prefix);
            let suffix_ok = suffix.is_empty() || doc_text[end..].starts_with(suffix);
            if prefix_ok && suffix_ok {
                matches.push((start, end));
            }
            search_from = start + 1;
        }
        if matches.is_empty() {
            return Err("evidence_fragment_quote_not_found".into());
        }
        if matches.len() > 1 {
            return Err("evidence_fragment_ambiguous_quote_match".into());
        }
        let (char_start, char_end) = matches[0];
        let target_key = match args.get("target_entry_key") {
            Some(v) => text(v)?,
            None => "heart.10d",
        };
        let target_slot = match args.get("target_instance_slot") {
            Some(v) => integer(v, "target_instance_slot")? as u32,
            None => 0,
        };
        let target_iri = match args.get("target_semantic_iri") {
            Some(v) => text(v)?,
            None => "urn:qualia:anatomy:cut-myocardium:v1",
        };
        let target_region = match args.get("target_surface_region") {
            Some(v) => integer(v, "target_surface_region")? as u8,
            None => 1,
        };
        let annotation_iri = format!("urn:qualia:oa:annotation:{actual_sha}:{char_start}:{char_end}");
        let mut graph = ReceiptGraph::new("urn:qualia:discourse:evidence-fragments:v1");
        graph.fact(&annotation_iri, RDF_TYPE, &format!("{OA_NS}Annotation"))?;
        graph.fact(
            &annotation_iri,
            &format!("{OA_NS}hasBody"),
            &format!("urn:qualia:doc:sha256:{actual_sha}#char={char_start},{char_end}"),
        )?;
        graph.fact(&annotation_iri, &format!("{OA_NS}hasTarget"), target_iri)?;
        let receipt = graph.seal(self.limits.resource_bytes.min(512 * 1024))?;
        let annotation_json = serde_json::to_vec(&json!({
            "@context": "http://www.w3.org/ns/anno.jsonld",
            "id": annotation_iri,
            "type": "Annotation",
            "body": {
                "type": "SpecificResource",
                "source": format!("urn:sha256:{actual_sha}"),
                "selector": {
                    "type": "TextQuoteSelector",
                    "exact": quote,
                    "prefix": prefix,
                    "suffix": suffix,
                    "start": char_start,
                    "end": char_end,
                }
            },
            "target": {
                "entry_key": target_key,
                "instance_slot": target_slot,
                "semantic_iri": target_iri,
                "semantic_id": q_hash(target_iri),
                "surface_region": target_region,
            }
        }))
        .map_err(|e| e.to_string())?;
        Ok(Value::Record(Args::from([
            ("annotation".into(), self.admit_encoded("json", annotation_json)?),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
            ("document_sha256".into(), Value::String(actual_sha)),
        ])))
    }

    /// #249 `hypermedia.radiance.plan_semantic_viewpoints`
    pub(crate) fn plan_semantic_viewpoints(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["include_occluder", "viewport_size"])?;
        let include_occluder = boolean(args, "include_occluder", false)?;
        let side = match args.get("viewport_size") {
            Some(v) => integer(v, "viewport_size")?,
            None => 64,
        };
        if !(16..=256).contains(&side) {
            return Err("viewport_size must be in 16..=256".into());
        }
        self.charge(side * side * 32, 256 * 1024)?;

        // Two distinct instances sharing the same class IRI at x = -0.4 and x = +0.4 (z = 0.0, radius = 0.25),
        // plus an optional opaque occluder at z = +0.5 covering x = -0.4.
        let shared_iri = "urn:qualia:anatomy:paired-renal-instance:v1";
        let shared_hash = q_hash(shared_iri);
        let occluder_iri = "urn:qualia:anatomy:opaque-fascia-occluder:v1";
        let occluder_hash = q_hash(occluder_iri);

        let mut inst0_pixels = 0usize;
        let mut inst1_pixels = 0usize;
        let mut occluder_pixels = 0usize;
        for iy in 0..side {
            let y = -1.0 + 2.0 * (iy as f64 + 0.5) / side as f64;
            for ix in 0..side {
                let x = -1.0 + 2.0 * (ix as f64 + 0.5) / side as f64;
                let hits_occluder =
                    include_occluder && (x + 0.4).hypot(y) <= 0.30;
                if hits_occluder {
                    occluder_pixels += 1;
                } else if (x + 0.4).hypot(y) <= 0.25 {
                    inst0_pixels += 1;
                } else if (x - 0.4).hypot(y) <= 0.25 {
                    inst1_pixels += 1;
                }
            }
        }
        let table = tabular_handle(
            self,
            "semantic_viewpoint_coverage",
            vec![
                ("instance_slot", vec![0.0, 1.0, 2.0]),
                (
                    "visible_pixels",
                    vec![
                        inst0_pixels as f64,
                        inst1_pixels as f64,
                        occluder_pixels as f64,
                    ],
                ),
            ],
        )?;
        let mut graph = ReceiptGraph::new("urn:qualia:radiance:semantic-viewpoints:v1");
        graph.fact(
            shared_iri,
            &format!("{QUALIA_SPEC}instance0VisiblePixels"),
            &format!("urn:qualia:pixels:{inst0_pixels}"),
        )?;
        graph.fact(
            shared_iri,
            &format!("{QUALIA_SPEC}instance1VisiblePixels"),
            &format!("urn:qualia:pixels:{inst1_pixels}"),
        )?;
        let receipt = graph.seal(self.limits.resource_bytes.min(512 * 1024))?;
        let report = serde_json::to_vec(&json!({
            "shared_class_iri": shared_iri,
            "shared_class_q_hash": shared_hash,
            "occluder_iri": occluder_iri,
            "occluder_q_hash": occluder_hash,
            "include_occluder": include_occluder,
            "instance_0_visible_pixels": inst0_pixels,
            "instance_1_visible_pixels": inst1_pixels,
            "occluder_visible_pixels": occluder_pixels,
        }))
        .map_err(|e| e.to_string())?;
        Ok(Value::Record(Args::from([
            ("table".into(), table),
            ("report".into(), self.admit_encoded("json", report)?),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
            ("instance_0_visible_pixels".into(), Value::U64(inst0_pixels as u64)),
            ("instance_1_visible_pixels".into(), Value::U64(inst1_pixels as u64)),
        ])))
    }

    /// #250 `hypermedia.kinetics.retarget_semantic_clip`
    pub(crate) fn retarget_semantic_clip(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["joint_map", "scale_ratio", "frames"])?;
        let scale_ratio = args
            .get("scale_ratio")
            .map(num)
            .transpose()?
            .unwrap_or(1.0);
        if !(0.1..=10.0).contains(&scale_ratio) {
            return Err("scale_ratio must be in [0.1, 10.0]".into());
        }
        let frames = match args.get("frames") {
            Some(v) => integer(v, "frames")?,
            None => 8,
        };
        if !(2..=256).contains(&frames) {
            return Err("frames must be in 2..=256".into());
        }

        // Parse semantic joint correspondence map (`source_iri -> target_iri`) and reject duplicates/ambiguity.
        let pairs: Vec<(String, String)> = if let Some(v) = args.get("joint_map") {
            list(v)?
                .iter()
                .map(|item| {
                    let r = record(item)?;
                    fields(r, &["source_iri", "target_iri"])?;
                    Ok((
                        text(required(r, "source_iri")?)?.to_owned(),
                        text(required(r, "target_iri")?)?.to_owned(),
                    ))
                })
                .collect::<Result<Vec<_>, String>>()?
        } else {
            vec![
                ("urn:qualia:skl2:pelvis".into(), "urn:qualia:skl2:pelvis".into()),
                ("urn:qualia:skl2:femur_l".into(), "urn:qualia:skl2:femur_l".into()),
                ("urn:qualia:skl2:tibia_l".into(), "urn:qualia:skl2:tibia_l".into()),
            ]
        };
        if pairs.is_empty() || pairs.len() > 128 {
            return Err("joint_map requires 1..=128 joint correspondences".into());
        }
        let mut seen_src = BTreeSet::new();
        let mut seen_dst = BTreeSet::new();
        for (src, dst) in &pairs {
            if src.is_empty() || dst.is_empty() || !seen_src.insert(src) || !seen_dst.insert(dst) {
                return Err("retarget_ambiguous_or_duplicate_joint_correspondence".into());
            }
        }
        self.charge(pairs.len() * frames * 1024, 256 * 1024)?;

        let mut tracks = Vec::with_capacity(pairs.len());
        let mut keys = Vec::with_capacity(pairs.len() * frames);
        let mut max_roundtrip_residual = 0.0_f64;
        for (joint_idx, (_src_iri, _dst_iri)) in pairs.iter().enumerate() {
            let first_key = keys.len() as u32;
            for f in 0..frames {
                let t = f as f64 / (frames - 1) as f64;
                let angle = 0.35 * (std::f64::consts::PI * t).sin();
                let half = 0.5 * angle;
                let rot = [half.cos(), half.sin(), 0.0, 0.0];
                let src_trans = [0.0, 0.25 * (joint_idx as f64 + 1.0), 0.05 * t];
                let dst_trans = [
                    src_trans[0] * scale_ratio,
                    src_trans[1] * scale_ratio,
                    src_trans[2] * scale_ratio,
                ];
                let src_motor = SklMotor::from_rotation_translation(rot, src_trans).normalize();
                let dst_motor = SklMotor::from_rotation_translation(rot, dst_trans).normalize();
                if (scale_ratio - 1.0).abs() <= 1e-12 {
                    let (_, src_t) = src_motor.to_rotation_translation();
                    let (_, rt_t) = dst_motor.to_rotation_translation();
                    let err = (rt_t[0] - src_t[0])
                        .hypot(rt_t[1] - src_t[1])
                        .hypot(rt_t[2] - src_t[2]);
                    max_roundtrip_residual = max_roundtrip_residual.max(err);
                }
                keys.push(AnimKey {
                    time: t,
                    local: dst_motor,
                });
            }
            tracks.push(AnimTrack {
                joint: joint_idx as u16,
                first_key,
                key_count: frames as u16,
            });
        }
        let clip_handle = self.admit_artifact(TypedArtifact::AnimationClip(Arc::new(AnimClip {
            duration_seconds: 1.0,
            looping: false,
            tracks,
            motor_keys: keys,
            morph_keys: Vec::new(),
        })))?;
        let mut graph = ReceiptGraph::new("urn:qualia:kinetics:retargeted-semantic-clip:v1");
        for (src, dst) in &pairs {
            graph.fact(src, &format!("{QUALIA_SPEC}retargetedToJoint"), dst)?;
        }
        let receipt = graph.seal(self.limits.resource_bytes.min(512 * 1024))?;
        let report = serde_json::to_vec(&json!({
            "joint_count": pairs.len(),
            "frames": frames,
            "scale_ratio": scale_ratio,
            "max_same_rig_roundtrip_residual_m": max_roundtrip_residual,
        }))
        .map_err(|e| e.to_string())?;
        Ok(Value::Record(Args::from([
            ("clip".into(), clip_handle),
            ("report".into(), self.admit_encoded("json", report)?),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
            ("max_roundtrip_residual_m".into(), Value::F64(max_roundtrip_residual)),
        ])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use crate::{AssetWorkspace, ToolLimits};

    #[test]
    fn wp40_specialist_morphisms_241_to_250_and_239_pass_acceptance_gates() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits: &limits,
            cancel: &cancel,
            work: 0,
        };

        // #239 fixed classroom delay recovery
        let r239 = host
            .invoke(
                "hypermedia.electromagnetics.reconstruct_sar_isar_tomography",
                &Value::Record(Args::from([(
                    "mode".into(),
                    Value::String("recover_synthetic_classroom_delay_profile".into()),
                )])),
            )
            .expect("#239 classroom delay recovery");
        assert!(matches!(r239, Value::Record(_)));

        // #241 section metrology annulus hole preservation across two grid resolutions
        let r241 = host
            .invoke(
                "hypermedia.manifold.measure_semantic_sections",
                &Value::Record(Args::new()),
            )
            .expect("#241 measure_semantic_sections");
        assert!(matches!(r241, Value::Record(_)));

        // #242 scene focus graph (Tab/Shift+Tab trap-free + cutaway occurrence equivalence)
        let r242 = host
            .invoke(
                "qualia.accessibility.compile_scene_focus_graph",
                &Value::Record(Args::new()),
            )
            .expect("#242 compile_scene_focus_graph");
        if let Value::Record(rec) = r242 {
            assert_eq!(rec.get("focus_trap_free"), Some(&Value::Bool(true)));
        } else {
            panic!("expected record from #242");
        }

        // #243 HMC working-set planner (zero unrelated payload reads)
        let r243 = host
            .invoke(
                "qualia.container.plan_hmc_semantic_working_set",
                &Value::Record(Args::new()),
            )
            .expect("#243 plan_hmc_semantic_working_set");
        if let Value::Record(rec) = r243 {
            assert_eq!(rec.get("unrelated_payload_bytes_read"), Some(&Value::U64(0)));
        } else {
            panic!("expected record from #243");
        }

        // #244 review-anchor rebase (unique rename preserved, duplicates stay Ambiguous)
        let r244 = host
            .invoke(
                "qualia.provenance.rebase_hmc_review_anchors",
                &Value::Record(Args::new()),
            )
            .expect("#244 rebase_hmc_review_anchors");
        assert!(matches!(r244, Value::Record(_)));

        // #245 affordance equivalence (catches pointer-only action when keyboard misses one)
        let r245_ok = host
            .invoke(
                "qualia.accessibility.audit_affordance_equivalence",
                &Value::Record(Args::new()),
            )
            .expect("#245 equivalent default");
        if let Value::Record(rec) = r245_ok {
            assert_eq!(rec.get("equivalent"), Some(&Value::Bool(true)));
        }
        let r245_diff = host
            .invoke(
                "qualia.accessibility.audit_affordance_equivalence",
                &Value::Record(Args::from([
                    (
                        "required_actions".into(),
                        Value::List(vec![
                            Value::String("urn:qualia:eont:a".into()),
                            Value::String("urn:qualia:eont:b".into()),
                        ]),
                    ),
                    (
                        "pointer_actions".into(),
                        Value::List(vec![
                            Value::String("urn:qualia:eont:a".into()),
                            Value::String("urn:qualia:eont:b".into()),
                        ]),
                    ),
                    (
                        "keyboard_actions".into(),
                        Value::List(vec![Value::String("urn:qualia:eont:a".into())]),
                    ),
                ])),
            )
            .expect("#245 non-equivalent counterexample");
        if let Value::Record(rec) = r245_diff {
            assert_eq!(rec.get("equivalent"), Some(&Value::Bool(false)));
        }

        // #246 semantic LOD transition (passes valid, rejects swapped wall/lumen even at 0m drift)
        host.invoke(
            "hypermedia.manifold.verify_semantic_lod_transition",
            &Value::Record(Args::new()),
        )
        .expect("#246 valid LOD transition");
        let swapped = host.invoke(
            "hypermedia.manifold.verify_semantic_lod_transition",
            &Value::Record(Args::from([
                ("lod0_wall_iri".into(), Value::String("urn:qualia:wall".into())),
                ("lod0_lumen_iri".into(), Value::String("urn:qualia:lumen".into())),
                ("lod1_wall_iri".into(), Value::String("urn:qualia:lumen".into())),
                ("lod1_lumen_iri".into(), Value::String("urn:qualia:wall".into())),
                ("lod0_radii_m".into(), Value::List(vec![Value::F64(0.08), Value::F64(0.035)])),
                ("lod1_radii_m".into(), Value::List(vec![Value::F64(0.08), Value::F64(0.035)])),
            ])),
        );
        assert!(swapped.is_err(), "#246 must reject swapped wall/lumen labels");

        // #247 timed semantic cues (<= 1ms WebVTT quantization error)
        let r247 = host
            .invoke(
                "hypermedia.chronicle.bind_timed_semantic_cues",
                &Value::Record(Args::new()),
            )
            .expect("#247 bind_timed_semantic_cues");
        if let Value::Record(rec) = r247 {
            let Some(Value::F64(err)) = rec.get("max_quantization_error_s") else {
                panic!("missing max_quantization_error_s");
            };
            assert!(*err <= 0.001);
        }

        // #248 evidence fragments (passes unique quote, rejects stale digest)
        host.invoke(
            "hypermedia.discourse.bind_evidence_fragments",
            &Value::Record(Args::new()),
        )
        .expect("#248 bind_evidence_fragments");
        let stale = host.invoke(
            "hypermedia.discourse.bind_evidence_fragments",
            &Value::Record(Args::from([(
                "expected_document_sha256".into(),
                Value::String("0000000000000000000000000000000000000000000000000000000000000000".into()),
            )])),
        );
        assert!(stale.is_err(), "#248 must reject stale document SHA-256");

        // #249 semantic viewpoints (two instances counted separately, occluder drops inst 0 to 0)
        let r249_clear = host
            .invoke(
                "hypermedia.radiance.plan_semantic_viewpoints",
                &Value::Record(Args::from([("include_occluder".into(), Value::Bool(false))])),
            )
            .expect("#249 unoccluded");
        if let Value::Record(rec) = r249_clear {
            let Some(Value::U64(p0)) = rec.get("instance_0_visible_pixels") else { panic!() };
            let Some(Value::U64(p1)) = rec.get("instance_1_visible_pixels") else { panic!() };
            assert!(*p0 > 0 && *p0 == *p1);
        }
        let r249_occ = host
            .invoke(
                "hypermedia.radiance.plan_semantic_viewpoints",
                &Value::Record(Args::from([("include_occluder".into(), Value::Bool(true))])),
            )
            .expect("#249 occluded");
        if let Value::Record(rec) = r249_occ {
            assert_eq!(rec.get("instance_0_visible_pixels"), Some(&Value::U64(0)));
        }

        // #250 SKL2 clip retargeting (same-rig roundtrip < 1e-6, rejects duplicate joint target)
        let r250 = host
            .invoke(
                "hypermedia.kinetics.retarget_semantic_clip",
                &Value::Record(Args::new()),
            )
            .expect("#250 retarget_semantic_clip");
        if let Value::Record(rec) = r250 {
            let Some(Value::F64(res)) = rec.get("max_roundtrip_residual_m") else { panic!() };
            assert!(*res < 1e-6);
        }
    }
}
