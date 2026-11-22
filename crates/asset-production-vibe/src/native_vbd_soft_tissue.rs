//! Production stepping of an admitted FEMC surface and native VBD cage.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::{Arc, atomic::Ordering};

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::{
    etalon_render::surface_frames::tangent::tangents_from_uvs_into,
    q_hash,
    render::{
        assets::{ImportedMesh, Mesh},
        mesh_surface_frames::vertex_normals_from_indices_into,
        portal_load_10d::Load10dGpuBuffers,
        skeletal::semantic_motion::{
            FemcFrame, MAX_FEMC_STEP_S, MacroChannel, MacroKeyframe, MotionGraph, SemanticAction,
            prepare_femc_asset_motion, prepare_q42_rule_macro, prepare_q42_semantic_macro,
            sample_prepared_macro,
        },
    },
};

use crate::kinetics_marine_vehicle::{bundle_bytes, with_semantic_motion};
use crate::{
    Args, AssetHost, TypedArtifact, Value, artifact::AttributedMesh, count, fields, num,
    required, text,
};

const MAX_SUBSTEPS: usize = 256;
const MAX_MOTION_KEYS: usize = 512;

impl AssetHost<'_> {
    pub(crate) fn step_vbd_soft_tissue(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "bundle",
                "asset_key",
                "material_key",
                "action_iri",
                "context_iri",
                "actor_group_iri",
                "step_s",
                "substeps",
                "activation",
                "rule_source",
            ],
        )?;
        let asset_key = text(required(args, "asset_key")?)?;
        let material_key = text(required(args, "material_key")?)?;
        let group_iri = text(required(args, "actor_group_iri")?)?;
        if group_iri.is_empty() || group_iri.len() > 4096 {
            return Err("FEMC requires a bounded HOLO actor IRI".into());
        }
        let group = q_hash(group_iri);
        let dt = num(required(args, "step_s")?)?;
        if dt <= 0.0 || dt > MAX_FEMC_STEP_S {
            return Err(format!("FEMC step_s must be in (0, {MAX_FEMC_STEP_S}]"));
        }
        required(args, "substeps")?;
        let steps = count(args, "substeps", 1, MAX_SUBSTEPS)?;
        let supplied = supplied_activation(args.get("activation"), steps)?;
        let rule_source = args.get("rule_source").map(text).transpose()?;
        if supplied.is_some() && rule_source.is_some() {
            return Err("FEMC accepts either explicit activation or a sealed rule source".into());
        }
        let bundle_size = bundle_bytes(self, required(args, "bundle")?)?.len();
        self.charge(
            bundle_size.saturating_add(steps * 131_072),
            32 * 1024 * 1024 + steps * 512,
        )?;

        let (controller, surface, activation) = with_semantic_motion(
            self,
            args,
            asset_key,
            |bundle, asset, graph, action, material_iri, matter| {
                let material_entry = bundle
                    .entry(material_key)
                    .ok_or("Missing FEMC M64 resource entry")?;
                let digest: &[u8; 32] = material_entry
                    .sha256
                    .as_slice()
                    .try_into()
                    .map_err(|_| "FEMC M64 resource has an invalid SHA-256 length")?;
                let (controller, surface) = prepare_femc_asset_motion(
                    graph,
                    action,
                    group_iri,
                    material_iri,
                    digest,
                    matter,
                    asset,
                )
                .map_err(|error| format!("FEMC source admission: {error:?}"))?;
                let activation = match supplied {
                    Some(values) => values,
                    None => authored_activation(graph, action, group, rule_source, dt, steps)?,
                };
                Ok((controller, surface, activation))
            },
        )?;
        let mut state = controller.initial_state();
        let n = surface.positions.len();
        let mut projected_rest = vec![[0.0; 3]; n];
        let mut projected = vec![[0.0; 3]; n];
        controller
            .rest_surface(&surface.positions, &mut projected_rest)
            .map_err(|error| format!("FEMC rest alignment: {error:?}"))?;

        let mut reports = Vec::with_capacity(steps);
        for (i, &value) in activation.iter().enumerate() {
            if self.cancel.load(Ordering::Acquire) {
                return Err("Production cancelled".into());
            }
            let frame = controller
                .advance(&mut state, dt, value)
                .map_err(|error| format!("FEMC simulation substep {}: {error:?}", i + 1))?;
            reports.push(frame);
        }
        controller
            .deform(&state, &mut projected)
            .map_err(|error| format!("FEMC posed surface: {error:?}"))?;
        let posed = attributed_surface(surface, &projected_rest, &projected)?;
        let (min, max, drift) = reports.iter().fold(
            (f64::INFINITY, f64::NEG_INFINITY, 0.0_f64),
            |(min, max, drift), f| {
                (
                    min.min(f.determinant_min),
                    max.max(f.determinant_max),
                    drift.max(f.enthesis_drift_m),
                )
            },
        );
        let table = diagnostics(dt, &activation, &reports)?;
        let mesh = self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(posed)))?;
        let table = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(table)))?;
        Ok(Value::Record(Args::from([
            ("mesh".into(), mesh),
            ("volume_det_f".into(), table),
            ("determinant_min".into(), Value::F64(min)),
            ("determinant_max".into(), Value::F64(max)),
            ("max_enthesis_drift_m".into(), Value::F64(drift)),
        ])))
    }
}

fn supplied_activation(input: Option<&Value>, steps: usize) -> Result<Option<Vec<f64>>, String> {
    let Some(input) = input else {
        return Ok(None);
    };
    let values = if let Value::List(rows) = input {
        if rows.len() != steps {
            return Err("FEMC activation series requires one value per substep".into());
        }
        rows.iter().map(num).collect::<Result<Vec<_>, _>>()?
    } else {
        vec![num(input)?; steps]
    };
    if values.iter().any(|value| !(0.0..=1.0).contains(value)) {
        return Err("FEMC activation must be in [0,1]".into());
    }
    Ok(Some(values))
}

fn authored_activation(
    graph: &mut MotionGraph<'_, '_, '_>,
    action: &SemanticAction<'_>,
    group: u64,
    source: Option<&str>,
    dt: f64,
    steps: usize,
) -> Result<Vec<f64>, String> {
    let mut keys = vec![MacroKeyframe::default(); MAX_MOTION_KEYS];
    let count = match source {
        Some(source) => prepare_q42_rule_macro(graph, action, source, &mut keys),
        None => prepare_q42_semantic_macro(graph, action, &mut keys),
    }
    .map_err(|error| format!("FEMC recruitment admission: {error:?}"))?;
    let mut staged = [None];
    let mut out = [None];
    (1..=steps)
        .map(|i| {
            let written =
                sample_prepared_macro(&keys[..count], i as f64 * dt, &mut staged, &mut out)
                    .map_err(|error| format!("FEMC recruitment sample {i}: {error:?}"))?;
            let value = out[0].ok_or("FEMC recruitment produced no sample")?;
            if written != 1
                || value.semantic_id != group
                || value.channel != MacroChannel::MuscleRecruitment
                || !(0.0..=1.0).contains(&value.value)
            {
                return Err(
                    "FEMC requires one MuscleRecruitment channel targeting the authored HOLO body"
                        .into(),
                );
            }
            Ok(value.value)
        })
        .collect()
}

fn attributed_surface(
    surface: Load10dGpuBuffers,
    projected_rest: &[[f32; 3]],
    projected: &[[f32; 3]],
) -> Result<AttributedMesh, String> {
    let Load10dGpuBuffers {
        mut positions,
        indices,
        uv0,
        colors,
        materials,
        ranges,
        ..
    } = surface;
    if positions.len() != projected.len()
        || positions.len() != projected_rest.len()
        || indices.len() % 3 != 0
    {
        return Err("FEMC output topology disagrees with its admitted source".into());
    }
    for ((target, deformed), rest) in positions.iter_mut().zip(projected).zip(projected_rest) {
        for axis in 0..3 {
            target[axis] += deformed[axis] - rest[axis];
        }
    }
    if positions.iter().flatten().any(|v| !v.is_finite()) {
        return Err("FEMC output positions are nonfinite".into());
    }
    let triangles = indices
        .chunks_exact(3)
        .map(|tri| [tri[0], tri[1], tri[2]])
        .collect();
    let mut min = positions[0];
    let mut max = positions[0];
    for position in &positions {
        for axis in 0..3 {
            min[axis] = min[axis].min(position[axis]);
            max[axis] = max[axis].max(position[axis]);
        }
    }
    let mut normals = vec![[0.0; 3]; positions.len()];
    vertex_normals_from_indices_into(&positions, &indices, &mut normals)
        .map_err(|error| format!("FEMC output normals: {error:?}"))?;
    let tangents = if let Some(uv) = &uv0 {
        let mut tangents = vec![[0.0; 4]; positions.len()];
        let mut scratch = vec![[0.0; 6]; positions.len()];
        tangents_from_uvs_into(
            &positions,
            uv,
            &normals,
            &indices,
            &mut tangents,
            &mut scratch,
        )
        .map_err(|error| format!("FEMC output tangents: {error:?}"))?;
        Some(tangents)
    } else {
        None
    };
    Ok(AttributedMesh {
        surface: ImportedMesh {
            mesh: Mesh {
                positions,
                triangles,
                min,
                max,
            },
            normals: Some(normals),
            tangents,
            texture_coordinates_0: uv0,
            materials,
            texture_dependencies: Vec::new(),
            submeshes: ranges,
        },
        uv1: Vec::new(),
        vertex_colors: colors,
        skin_weights: Vec::new(),
        morph_targets: Arc::from([]),
        protected_faces: Vec::new(),
    })
}

fn diagnostics(dt: f64, activation: &[f64], reports: &[FemcFrame]) -> Result<Workbook, String> {
    let mut workbook = Workbook::new();
    let sheet_id = workbook.add_sheet("FEMC volume and entheses");
    let sheet = workbook
        .sheet_mut(sheet_id)
        .ok_or("Could not create the FEMC diagnostic table")?;
    for (column, header) in [
        "time_s",
        "activation",
        "det_f_min",
        "det_f_max",
        "enthesis_drift_m",
    ]
    .into_iter()
    .enumerate()
    {
        sheet.set_value(
            CellRef::new(0, column as u32),
            CellValue::Text(header.into()),
        );
    }
    for (i, (frame, &value)) in reports.iter().zip(activation).enumerate() {
        let row = i + 1;
        let values = [
            row as f64 * dt,
            value,
            frame.determinant_min,
            frame.determinant_max,
            frame.enthesis_drift_m,
        ];
        for (column, value) in values.into_iter().enumerate() {
            sheet.set_value(
                CellRef::new(row as u32, column as u32),
                CellValue::Number(value),
            );
        }
    }
    Ok(workbook)
}
