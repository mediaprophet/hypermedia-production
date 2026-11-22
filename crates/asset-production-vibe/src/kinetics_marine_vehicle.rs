//! Production sampling of a Q42-authorized six-DOF marine vehicle controller.

use crate::{
    artifact::{AnimClip, HmcWorkspaceView},
    fields, list, num, required, text, vector, Args, AssetHost, TypedArtifact, Value,
};
use bytemuck::Zeroable;
use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::{
    bundle::{
        semantic_model::{hmc_entry_key_iri, open_semantic_model},
        BundleReadLimits, BundleReader,
    },
    container_10d::{
        decode_entity_ontology_section_into, AffordanceBinaryRecord, SectionType, TypedFieldRecord,
        TypedPayloadResolver, MANIFOLD_IDENTITY_V2_MAX_FIELDS, MAX_AFFORDANCES_PER_SECTION,
    },
    q42::m64_matter::{M64PayloadInterval, M64ReadLimits, M64Reader},
    q42_volume::{MAX_COMPRESSED_SUPERBLOCK_SIZE, SUPERBLOCK_SIZE},
    q_hash,
    render::{
        portal_load_10d::admit_leaf_assembly,
        skeletal::{
            semantic_motion::{
                prepare_marine_vehicle, MarineVehicleControl, MarineVehicleStep, MotionGraph,
                SemanticAction,
            },
            Key, Motor, Track,
        },
    },
};
use std::{collections::BTreeMap, sync::Arc};

const MAX_STEPS: usize = 4096;

impl AssetHost<'_> {
    pub(crate) fn simulate_vehicle_aero_hydro_6dof(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "bundle",
                "assembly_key",
                "material_key",
                "action_iri",
                "context_iri",
                "step_s",
                "controls",
            ],
        )?;
        let bundle_arg = required(args, "bundle")?;
        let assembly_key = text(required(args, "assembly_key")?)?;
        let dt = num(required(args, "step_s")?)?;
        if dt <= 0.0 || dt > 0.25 {
            return Err("Marine vehicle step must be in (0, 0.25] seconds".into());
        }
        let commands = list(required(args, "controls")?)?;
        if !(1..=MAX_STEPS).contains(&commands.len()) {
            return Err(format!(
                "Marine vehicle requires 1–{MAX_STEPS} control samples"
            ));
        }
        let bytes_len = bundle_bytes(self, bundle_arg)?.len();
        self.charge(
            bytes_len.saturating_add(commands.len() * 4096),
            commands.len() * 2048 + 2 * 1024 * 1024,
        )?;
        let (controller, joint) = with_semantic_motion(
            self,
            args,
            assembly_key,
            |bundle, _, graph, action, iri, matter| {
                let leaves = admit_leaf_assembly(bundle, assembly_key)?;
                let [leaf] = leaves.as_slice() else {
                    return Err("Marine motion requires one admitted rigid hull leaf".into());
                };
                let controller = prepare_marine_vehicle(graph, action, iri, matter)
                    .map_err(|error| format!("Marine vehicle controller: {error:?}"))?;
                if leaf.semantic_id != controller.semantic_id() {
                    return Err(
                        "Marine controller identity differs from the admitted hull leaf".into(),
                    );
                }
                controller
                    .validate_instance_root(&leaf.overrides)
                    .map_err(|error| format!("Marine vehicle authored rest frame: {error:?}"))?;
                let joint = u16::try_from(leaf.slot)
                    .map_err(|_| "Assembly slot exceeds animation track capacity")?;
                Ok((controller, joint))
            },
        )?;
        let identity = controller.semantic_id();
        let mut state = controller.initial_state();
        let mut keys = Vec::with_capacity(commands.len() + 1);
        keys.push(Key {
            time: 0.0,
            local: Motor::identity(),
        });
        let mut workbook = Workbook::new();
        let sheet = workbook
            .sheet_mut(0)
            .ok_or("Telemetry workbook has no initial sheet")?;
        sheet.name = "Marine telemetry".into();
        for (column, label) in [
            "time_s",
            "velocity_x_m_s",
            "velocity_y_m_s",
            "velocity_z_m_s",
            "angular_x_rad_s",
            "angular_y_rad_s",
            "angular_z_rad_s",
            "submerged_m3",
            "buoyancy_n",
            "water_drag_x_n",
            "water_drag_y_n",
            "water_drag_z_n",
            "air_drag_x_n",
            "air_drag_y_n",
            "air_drag_z_n",
            "propulsive_power_w",
        ]
        .into_iter()
        .enumerate()
        {
            sheet.set_value(CellRef::new(0, column as u32), CellValue::text(label));
        }
        for (index, command) in commands.iter().enumerate() {
            let [throttle, rudder] = vector::<2>(command)?;
            if !(-1.0..=1.0).contains(&throttle) || !(-1.0..=1.0).contains(&rudder) {
                return Err(
                    "Marine control samples are [throttle, rudder] with values in [-1, 1]".into(),
                );
            }
            let pose = controller
                .advance(
                    &mut state,
                    MarineVehicleControl {
                        throttle: throttle as f32,
                        rudder: rudder as f32,
                    },
                    dt,
                )
                .map_err(|error| format!("Marine vehicle sample {index}: {error:?}"))?;
            keys.push(Key {
                time: pose.time_s,
                local: pose.local_delta,
            });
            for (column, value) in telemetry(&pose).into_iter().enumerate() {
                sheet.set_value(
                    CellRef::new(index as u32 + 1, column as u32),
                    CellValue::Number(value),
                );
            }
        }
        let duration = keys.last().ok_or("Marine motion has no keys")?.time;
        let clip = AnimClip {
            duration_seconds: duration,
            looping: false,
            tracks: vec![Track {
                joint,
                first_key: 0,
                key_count: keys.len() as u16,
            }],
            motor_keys: keys,
            morph_keys: Vec::new(),
        };
        self.charge(0, 0)?;
        let clip = self.admit_artifact(TypedArtifact::AnimationClip(Arc::new(clip)))?;
        let telemetry = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(workbook)))?;
        Ok(Value::Record(BTreeMap::from([
            ("clip".into(), clip),
            ("telemetry".into(), telemetry),
            ("instance_slot".into(), Value::U64(u64::from(joint))),
            ("semantic_id".into(), Value::U64(identity)),
        ])))
    }
}

pub(super) fn with_semantic_motion<T>(
    host: &AssetHost<'_>,
    args: &Args,
    asset_key: &str,
    prepare: impl FnOnce(
        &BundleReader<'_>,
        &[u8],
        &mut MotionGraph<'_, '_, '_>,
        &SemanticAction<'_>,
        &str,
        &M64Reader<'_>,
    ) -> Result<T, String>,
) -> Result<T, String> {
    let material_key = text(required(args, "material_key")?)?;
    let action_iri = text(required(args, "action_iri")?)?;
    let context_iri = text(required(args, "context_iri")?)?;
    if [asset_key, material_key, action_iri, context_iri]
        .iter()
        .any(|value| value.is_empty() || value.len() > 4096)
    {
        return Err("Motion requires bounded asset, material, and semantic selectors".into());
    }
    let bytes = bundle_bytes(host, required(args, "bundle")?)?;
    let bundle = BundleReader::parse_with_limits(
        bytes,
        BundleReadLimits {
            file_bytes: bytes.len(),
            entries: host.limits.resources,
            ..BundleReadLimits::default()
        },
    )
    .map_err(|error| format!("Semantic motion HMC: {error}"))?;
    let asset = bundle
        .entry(asset_key)
        .filter(|entry| entry.kind == "10d")
        .and_then(|entry| bundle.get(&entry.key))
        .ok_or("Semantic motion requires an admitted 10D asset")?;
    let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = scratch;
    let view = TypedPayloadResolver::new(asset, &[], &mut scratch, &mut fields)
        .map_err(|error| error.to_string())?;
    let eont = view
        .section_payload(SectionType::EntityOntology)
        .ok_or("Semantic motion requires an EONT action on the asset")?;
    let mut affordances = [AffordanceBinaryRecord::zeroed(); MAX_AFFORDANCES_PER_SECTION];
    let (physiology, count) = decode_entity_ontology_section_into(eont, &mut affordances)
        .map_err(|error| format!("Motion affordance admission: {error:?}"))?;
    let action = SemanticAction {
        iri: action_iri,
        affordances: &affordances[..count],
        capability_ids: &[],
        observer_status: 0,
        energy_permill: physiology.energy_permill,
        damage_permill: physiology.damage_permill,
        hydration_permill: physiology.hydration_permill,
        wear_permill: physiology.wear_permill,
    };
    let model = open_semantic_model(&bundle).map_err(|error| error.to_string())?;
    let mut compressed = vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE];
    let mut decoded = vec![0; SUPERBLOCK_SIZE];
    let mut lex_page = vec![0; 1024 * 1024];
    let mut lexical = [0; 4096];
    let mut graph = MotionGraph {
        model: &model,
        context: q_hash(context_iri),
        max_model_blocks: 256,
        queries_left: 4096,
        compressed: &mut compressed,
        decoded: &mut decoded,
        lex_page: &mut lex_page,
        lex_value: &mut lexical,
    };
    graph
        .verify_context_iri(context_iri)
        .map_err(|error| format!("Motion context: {error:?}"))?;
    let hull = bundle
        .entry(material_key)
        .filter(|entry| entry.kind == "m64")
        .and_then(|entry| bundle.get(&entry.key))
        .ok_or("Semantic motion requires an admitted M64 resource")?;
    let mut intervals = vec![M64PayloadInterval::default(); 4096];
    let materials = M64Reader::parse(
        hull,
        M64ReadLimits {
            max_file_bytes: host.limits.resource_bytes,
            max_chunks: 4096,
            max_records: 1_048_576,
        },
        &mut intervals,
    )
    .map_err(|error| format!("Motion M64 admission: {error:?}"))?;
    prepare(
        &bundle,
        asset,
        &mut graph,
        &action,
        &hmc_entry_key_iri(material_key),
        &materials,
    )
}

pub(super) fn bundle_bytes<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a [u8], String> {
    if let Ok(TypedArtifact::HmcBundle(view)) = host.workspace.typed(value) {
        return match view.as_ref() {
            HmcWorkspaceView::Sealed(bytes) => Ok(bytes),
            HmcWorkspaceView::Staged(_) => Err("Marine simulation requires a sealed HMC".into()),
        };
    }
    let resource = crate::qualia_containers_governance_poet_ops::encoded_resource(host, value)?;
    if resource.kind != "hmc" {
        return Err("Marine simulation requires an HMC resource".into());
    }
    Ok(&resource.bytes)
}

fn telemetry(sample: &MarineVehicleStep) -> [f64; 16] {
    let [vx, vy, vz] = sample.linear_velocity_m_s.map(f64::from);
    let [wx, wy, wz] = sample.angular_velocity_rad_s.map(f64::from);
    let [dx, dy, dz] = sample.hydrodynamic_drag_n.map(f64::from);
    let [ax, ay, az] = sample.aerodynamic_drag_n.map(f64::from);
    [
        sample.time_s,
        vx,
        vy,
        vz,
        wx,
        wy,
        wz,
        f64::from(sample.submerged_volume_m3),
        f64::from(sample.buoyancy_n),
        dx,
        dy,
        dz,
        ax,
        ay,
        az,
        f64::from(sample.propulsive_power_w),
    ]
}
