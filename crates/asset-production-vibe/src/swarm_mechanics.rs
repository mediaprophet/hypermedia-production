//! Bounded procedural grooming, surface bindings, plant geometry, and authored rigs.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use qualia_core_db::{
    container_10d::skeletal_section::KinematicSolverRegime,
    etalon_render::{
        skeletal::{
            Key, MAX_KEYS, Motor, Track,
            exoskeletal_cuticle::{BicondylarHinge, CuticleSclerite, MAX_EXO_SCLERITES},
            valid_motor,
        },
        vegetation::{
            LeafEconomicsProfile, LeafVascularSkeleton, MAX_METAMERS, Metamer, MetamerOrgan,
            MetamerPlantGraph, NO_PARENT_METAMER, ThermalPhenology,
        },
    },
    render::assets::{ImportedMesh, Mesh},
    specialized_libs::computational_geometry::{Point3, isosurface::marching_cubes},
};
use sha2::{Digest, Sha256};

use crate::{
    Args, AssetHost, TypedArtifact, Value,
    artifact::{
        AnimClip, AttributedMesh, BranchGraph10d, CurveGroomBundle, CurveStrand, FieldLattice,
    },
    count, fields, integer, list, num, number, record, required, text, vector,
};

type Vec3 = [f64; 3];

struct Tree {
    children: Vec<Vec<usize>>,
    parents: Vec<Option<usize>>,
    ordered: Vec<usize>,
}

impl Tree {
    fn read(graph: &BranchGraph10d) -> Result<Self, String> {
        let len = graph.nodes.len();
        if !(2..=8192).contains(&len) || graph.edges.is_empty() || graph.edges.len() >= len {
            return Err("Expected a bounded branching forest".into());
        }
        for &point in &graph.nodes {
            pack_position(point)?;
        }
        let mut children = vec![Vec::new(); len];
        let mut parents = vec![None; len];
        for edge in &graph.edges {
            let [a, b] = edge.nodes.map(|index| index as usize);
            if a >= len
                || b >= len
                || parents[b].is_some()
                || norm(sub(graph.nodes[a], graph.nodes[b])) <= 1e-8
            {
                return Err("Branch edge has invalid endpoints or a duplicate parent".into());
            }
            parents[b] = Some(a);
            children[a].push(b);
        }
        let mut ordered = Vec::with_capacity(len);
        let mut queue = parents
            .iter()
            .enumerate()
            .filter_map(|(i, p)| p.is_none().then_some(i))
            .collect::<Vec<_>>();
        if queue.iter().any(|&root| children[root].is_empty()) {
            return Err("Branch forest contains an isolated node".into());
        }
        while let Some(node) = queue.pop() {
            ordered.push(node);
            queue.extend(children[node].iter().rev().copied());
        }
        if ordered.len() != len {
            return Err("Branch graph contains a cycle".into());
        }
        Ok(Self {
            parents,
            children,
            ordered,
        })
    }
}

#[derive(Clone, Copy)]
struct SurfacePatch {
    weights: Vec3,
    uv: [f64; 2],
    origin: Vec3,
    frame: [Vec3; 3],
}

impl AssetHost<'_> {
    /// Shell prototypes place their origin at the hinge midpoint, with local X along
    /// the hinge. Landmark coordinates are relative to the parent's rest frame.
    pub(crate) fn synthesize_invertebrate_and_ergonomic_10d(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "assembly_frame",
                "rig_frame",
                "archetypes",
                "sclerites",
                "root_origin_m",
            ],
        )?;
        let frame = required(args, "assembly_frame")?;
        let resource = crate::qualia_containers_governance_poet_ops::encoded_resource(self, frame)?;
        if resource.kind != "10d" || resource.bytes.len() > self.limits.resource_bytes {
            return Err("Cuticle assembly requires a bounded .10d frame".into());
        }
        let capacity = Self::assembly_frame_leaf_capacity(&resource.bytes)?.min(MAX_EXO_SCLERITES);
        let rig_frame = required(args, "rig_frame")?;
        let rig_bytes = self.workspace.encoded(rig_frame)?;
        if rig_bytes.kind != "10d" || rig_bytes.bytes.len() > self.limits.resource_bytes {
            return Err("Cuticle companion rig requires an admitted .10d rig frame".into());
        }
        let archetypes = required(args, "archetypes")?;
        let sclerites = list(required(args, "sclerites")?)?;
        if !(2..=capacity).contains(&sclerites.len()) {
            return Err(format!(
                "Cuticle assembly requires 2–{capacity} parent-ordered sclerites"
            ));
        }
        let origin = args
            .get("root_origin_m")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0; 3]);
        if norm(origin) > 100.0 {
            return Err("Cuticle root origin must be within 100 m".into());
        }
        self.charge(sclerites.len() * 2048, sclerites.len() * 4096)?;
        let mut instances = Vec::with_capacity(sclerites.len());
        let mut joints = Vec::with_capacity(sclerites.len());
        let mut world_motors = Vec::<Motor>::with_capacity(sclerites.len());
        let mut report_rows = Vec::with_capacity(sclerites.len());
        let mut ids = BTreeSet::new();
        for (index, item) in sclerites.iter().enumerate() {
            let row = record(item)?;
            fields(
                row,
                &[
                    "iri",
                    "leaf_key",
                    "parent",
                    "hinge",
                    "thickness_mm",
                    "sclerotization_ratio",
                    "caco3_calcification_ratio",
                    "shell_mass_kg",
                    "shell_length_m",
                    "effective_inertia_kg_m2",
                ],
            )?;
            let iri = text(required(row, "iri")?)?;
            let hash = qualia_core_db::q_hash(iri);
            if !absolute_iri(iri) || hash == 0 || !ids.insert(hash) {
                return Err(
                    "Sclerites must have distinct absolute, nonzero semantic identities".into(),
                );
            }
            let parent = row
                .get("parent")
                .map(|parent| integer(parent, "sclerite parent"))
                .transpose()?;
            let (pose, local_hinge, limits) = match (index, parent, row.get("hinge")) {
                (0, None, None) => (
                    Motor::from_translation(origin[0], origin[1], origin[2]),
                    None,
                    [0.0; 2],
                ),
                (0, _, _) => return Err("Root sclerite may not have a parent or hinge".into()),
                (_, Some(parent), Some(spec)) if parent < index => {
                    let spec = record(spec)?;
                    fields(
                        spec,
                        &[
                            "anterior_m",
                            "posterior_m",
                            "rest_rad",
                            "min_rad",
                            "max_rad",
                        ],
                    )?;
                    let anterior = vector::<3>(required(spec, "anterior_m")?)?;
                    let posterior = vector::<3>(required(spec, "posterior_m")?)?;
                    if norm(anterior) > 10.0 || norm(posterior) > 10.0 {
                        return Err(
                            "Condyle landmarks must lie within 10 m of the parent origin".into(),
                        );
                    }
                    let pi = std::f64::consts::PI;
                    let minimum = bounded(spec, "min_rad", -pi / 2.0, -pi, pi)?;
                    let maximum = bounded(spec, "max_rad", pi / 2.0, -pi, pi)?;
                    let rest = bounded(spec, "rest_rad", 0.0, -pi, pi)?;
                    if minimum >= maximum || !(minimum..=maximum).contains(&rest) {
                        return Err(
                            "Hinge requires ordered limits containing its rest twist".into()
                        );
                    }
                    let hinge = BicondylarHinge {
                        condyle_anterior_m: anterior,
                        condyle_posterior_m: posterior,
                        theta_rad: rest,
                        rest_angle_rad: rest,
                        min_angle_rad: minimum,
                        max_angle_rad: maximum,
                        ..BicondylarHinge::default()
                    };
                    let (axis, pivot, span) = hinge
                        .axis_and_pivot()
                        .map_err(|error| format!("Bicondylar landmarks: {error:?}"))?;
                    let (sin, cos) = (rest / 2.0).sin_cos();
                    let local = Motor::from_rotation_translation(rotate_x_to(axis), pivot).mul(
                        &Motor::from_rotation_translation([cos, sin, 0.0, 0.0], [0.0; 3]),
                    );
                    let placement = world_motors[parent].mul(&local);
                    let hinge_data = serde_json::json!({
                        "axis_parent":axis, "pivot_parent_m":pivot, "span_m":span,
                        "anterior_parent_m":anterior, "posterior_parent_m":posterior,
                        "rest_rad":rest, "limits_rad":[minimum,maximum],
                    });
                    (
                        placement,
                        Some(hinge_data),
                        [minimum - rest, maximum - rest],
                    )
                }
                _ => {
                    return Err(
                        "Each child sclerite needs a preceding parent and bicondylar hinge".into(),
                    );
                }
            };
            if !valid_motor(&pose) {
                return Err("Composed sclerite pose is not a rigid PGA motor".into());
            }
            if limits
                .iter()
                .any(|limit| limit.abs() > std::f64::consts::PI)
            {
                return Err("Hinge limits relative to rest must each lie within ±π".into());
            }
            world_motors.push(pose);
            let shell = CuticleSclerite {
                parent: parent.map(|index| index as u16),
                thickness_mm: bounded(row, "thickness_mm", 0.4, 0.04, 4.0)?,
                sclerotization_ratio: bounded(row, "sclerotization_ratio", 0.9, 0.05, 1.0)?,
                caco3_calcification_ratio: bounded(
                    row,
                    "caco3_calcification_ratio",
                    0.1,
                    0.0,
                    1.0,
                )?,
                shell_mass_kg: bounded(row, "shell_mass_kg", 0.002, 1e-9, 100.0)?,
                effective_inertia_kg_m2: bounded(
                    row,
                    "effective_inertia_kg_m2",
                    1.5e-5,
                    1e-12,
                    100.0,
                )?,
                pose_world: pose,
            };
            let length = bounded(row, "shell_length_m", 0.04, 1e-6, 10.0)?;
            let mut descriptor = Args::from([
                ("iri".into(), Value::String(iri.into())),
                (
                    "swing_twist_rad".into(),
                    Value::List([0.0, 0.0, limits[0], limits[1]].map(Value::F64).into()),
                ),
                (
                    "solver_regime".into(),
                    Value::U64(
                        KinematicSolverRegime::ExoskeletalCuticleShell
                            .as_u8()
                            .into(),
                    ),
                ),
                ("bone_length_m".into(), Value::F64(length)),
                ("segment_mass_kg".into(), Value::F64(shell.shell_mass_kg)),
            ]);
            if let Some(parent) = parent {
                descriptor.insert("parent".into(), Value::U64(parent as u64));
            }
            joints.push(Value::Record(descriptor));
            let pga = [
                pose.r_w, pose.r_x, pose.r_y, pose.r_z, -pose.d_x, -pose.d_y, -pose.d_z, pose.d_w,
            ];
            instances.push(Value::Record(Args::from([
                ("leaf_key".into(), required(row, "leaf_key")?.clone()),
                ("entity_iri".into(), Value::String(iri.into())),
                (
                    "local_pga_motor".into(),
                    Value::List(pga.map(Value::F64).into()),
                ),
            ])));
            report_rows.push(serde_json::json!({
                "iri":iri, "parent":parent, "hinge":local_hinge, "shell_length_m":length,
                "thickness_mm":shell.thickness_mm, "sclerotization_ratio":shell.sclerotization_ratio,
                "caco3_calcification_ratio":shell.caco3_calcification_ratio,
                "shell_mass_kg":shell.shell_mass_kg, "effective_inertia_kg_m2":shell.effective_inertia_kg_m2,
                "shell_rigidity_factor":shell.shell_rigidity_factor(), "assembly_motor":pga,
            }));
        }
        let assembly = self.compose_assembly_10d(&Args::from([
            ("assembly_frame".into(), frame.clone()),
            ("archetypes".into(), archetypes.clone()),
            ("instances".into(), Value::List(instances)),
        ]))?;
        let rig = self.compile_skl2_rig(&Args::from([
            ("source".into(), rig_frame.clone()),
            ("joints".into(), Value::List(joints)),
        ]))?;
        let report = self.encode(
            "json",
            serde_json::to_vec(&serde_json::json!({
                "schema":"qualia-authored-bicondylar-cuticle/1", "sclerites":report_rows,
            }))
            .map_err(|error| error.to_string())?,
        )?;
        let mut output = record(&assembly)?.clone();
        output.extend(record(&rig)?.clone());
        output.insert("report".into(), report);
        Ok(Value::Record(output))
    }

    pub(crate) fn synthesize_sigml_sign_language_avatar(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "rig",
                "signs",
                "score",
                "sign_rate_hz",
                "coarticulation_blend_s",
            ],
        )?;
        let TypedArtifact::SkeletalRig(rig) = self.workspace.typed(required(args, "rig")?)? else {
            return Err("Sign score requires an admitted typed skeletal rig".into());
        };
        let rig = Arc::clone(rig);
        rig.validate()
            .map_err(|e| format!("Sign score rig: {e:?}"))?;
        let signs = list(required(args, "signs")?)?;
        let score = list(required(args, "score")?)?;
        if !(1..=64).contains(&signs.len()) || !(1..=256).contains(&score.len()) {
            return Err("Sign score requires 1–64 authored poses and 1–256 sign events".into());
        }
        let rate = bounded(args, "sign_rate_hz", 2.0, 0.5, 5.0)?;
        let blend = bounded(args, "coarticulation_blend_s", 0.10, 0.02, 0.4)?;
        let period = 1.0 / rate;
        if blend >= period {
            return Err("Coarticulation time must be shorter than the sign interval".into());
        }
        self.charge(
            signs.len() * 54 * 512 + score.len() * 54 * 128,
            2 * 1024 * 1024,
        )?;
        let mut digest = Sha256::new();
        digest.update(b"qualia-authored-sign-score/1");
        digest.update(rate.to_le_bytes());
        digest.update(blend.to_le_bytes());
        digest.update((rig.joints.len() as u64).to_le_bytes());
        let mut joints = BTreeMap::new();
        for (index, joint) in rig.joints.iter().enumerate() {
            if joint.joint_id == 0 || joints.insert(joint.joint_id, index as u16).is_some() {
                return Err("Sign score rig needs unique nonzero joint identifiers".into());
            }
            digest.update(joint.joint_id.to_le_bytes());
            digest.update(joint.parent_index.to_le_bytes());
        }
        let mut vocabulary = BTreeMap::<&str, BTreeMap<u16, Motor>>::new();
        let mut baseline = None::<Vec<u16>>;
        let mut joint_iris = BTreeMap::new();
        digest.update((signs.len() as u64).to_le_bytes());
        for row in signs {
            let row = record(row)?;
            fields(row, &["sign_iri", "joints"])?;
            let sign = text(required(row, "sign_iri")?)?;
            let channels = list(required(row, "joints")?)?;
            if !absolute_iri(sign) || !(20..=54).contains(&channels.len()) {
                return Err(
                    "A sign pose needs an absolute IRI and 20–54 authored joint channels".into(),
                );
            }
            let mut motors = BTreeMap::new();
            digest.update((sign.len() as u64).to_le_bytes());
            digest.update(sign.as_bytes());
            digest.update((channels.len() as u64).to_le_bytes());
            for item in channels {
                let item = record(item)?;
                fields(item, &["joint_iri", "rotation_wxyz", "local_translation_m"])?;
                let iri = text(required(item, "joint_iri")?)?;
                if !absolute_iri(iri) {
                    return Err("Invalid sign joint IRI".into());
                }
                let joint = qualia_core_db::q_hash(iri);
                if joint_iris
                    .insert(joint, iri)
                    .is_some_and(|prior| prior != iri)
                {
                    return Err("Different sign joint IRIs share one q_hash".into());
                }
                let slot = *joints
                    .get(&joint)
                    .ok_or("Sign pose joint is absent from the typed rig")?;
                let rotation = vector::<4>(required(item, "rotation_wxyz")?)?;
                let translation = vector::<3>(required(item, "local_translation_m")?)?;
                let q2: f64 = rotation.into_iter().map(|x| x * x).sum();
                if (q2 - 1.0).abs() > 1e-5 || norm(translation) > 5.0 {
                    return Err(
                        "Sign score requires unit rotation and local translation within 5 m".into(),
                    );
                }
                let motor = Motor::from_rotation_translation(rotation, translation);
                if !valid_motor(&motor) || motors.insert(slot, motor).is_some() {
                    return Err("Sign pose contains invalid or duplicate motor data".into());
                }
                digest.update(joint.to_le_bytes());
                digest.update((iri.len() as u64).to_le_bytes());
                digest.update(iri.as_bytes());
                for value in rotation.into_iter().chain(translation) {
                    digest.update(value.to_le_bytes());
                }
            }
            let current = motors.keys().copied().collect::<Vec<_>>();
            if baseline.as_ref().is_some_and(|prior| prior != &current) {
                return Err("All signs must author the same local joint channels".into());
            }
            baseline = Some(current);
            if vocabulary.insert(sign, motors).is_some() {
                return Err("Duplicate sign IRI".into());
            }
        }
        let slots = baseline.ok_or("Sign vocabulary is empty")?;
        let score = score.iter().map(text).collect::<Result<Vec<_>, _>>()?;
        digest.update((score.len() as u64).to_le_bytes());
        let timeline = score
            .iter()
            .map(|iri| {
                digest.update((iri.len() as u64).to_le_bytes());
                digest.update(iri.as_bytes());
                vocabulary
                    .get(iri)
                    .ok_or("Sign score refers to an unauthored pose")
            })
            .collect::<Result<Vec<_>, _>>()?;
        let frame_count = 2 * score.len();
        if frame_count * slots.len() > MAX_KEYS {
            return Err(format!(
                "Sign score exceeds the native {MAX_KEYS}-key clip budget"
            ));
        }
        let duration_seconds = score.len() as f64 * period;
        let mut tracks = Vec::with_capacity(slots.len());
        let mut motor_keys = Vec::with_capacity(slots.len() * frame_count);
        for slot in slots {
            tracks.push(Track {
                joint: slot,
                first_key: motor_keys.len() as u32,
                key_count: frame_count as u16,
            });
            for (index, pose) in timeline.iter().enumerate() {
                let motor = pose[&slot];
                motor_keys.push(Key {
                    time: index as f64 * period,
                    local: motor,
                });
                let end = (index + 1) as f64 * period;
                let hold_until = if index + 1 == score.len() {
                    end
                } else {
                    end - blend
                };
                motor_keys.push(Key {
                    time: hold_until,
                    local: motor,
                });
            }
        }
        let context = format!("urn:qualia:authored-sign-score:{:x}", digest.finalize());
        let mut graph = ReceiptGraph::new(&context);
        for (index, sign) in score.iter().enumerate() {
            let tick = format!("{context}:event:{index}");
            graph.fact(&context, "urn:qualia:accessibility:has-sign-event", &tick)?;
            graph.fact(&tick, "urn:qualia:accessibility:authored-sign", sign)?;
            if index > 0 {
                graph.fact(
                    &tick,
                    "urn:qualia:accessibility:follows-sign-event",
                    &format!("{context}:event:{}", index - 1),
                )?;
            }
        }
        let clip = self.admit_artifact(TypedArtifact::AnimationClip(Arc::new(AnimClip {
            duration_seconds,
            looping: false,
            tracks,
            motor_keys,
            morph_keys: Vec::new(),
        })))?;
        let receipt =
            self.admit_encoded("q42", graph.seal(self.limits.resource_bytes.min(2 << 20))?)?;
        Ok(Value::Record(Args::from([
            ("clip".into(), clip),
            ("receipt".into(), receipt),
            ("duration_seconds".into(), Value::F64(duration_seconds)),
        ])))
    }

    pub(crate) fn validate_topological_body(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["samples"])?;
        let list = list(required(args, "samples")?)?;
        if !(1..=8192).contains(&list.len()) {
            return Err("Require 1–8192 authored radial-layer measurements".into());
        }
        self.charge(list.len() * 128, list.len() * 512)?;
        let mut seen = BTreeSet::new();
        let mut violations = Vec::new();
        let mut cavity_checks = 0;
        for row in list {
            let row = record(row)?;
            fields(
                row,
                &[
                    "entity_iri",
                    "layer_kind",
                    "depth_range",
                    "parent_cavity_iri",
                    "suspension_iri",
                    "max_cavity_sdf_phi_m",
                    "cavity_clearance_eps_m",
                    "max_enthesis_uv_drift_m",
                    "volume_det_f",
                    "enthesis_bindings",
                    "crosses_polyaxial_joint",
                    "retinaculum_iri",
                ],
            )?;
            let id = text(required(row, "entity_iri")?)?;
            let kind = text(required(row, "layer_kind")?)?;
            let [lo, hi] = vector::<2>(required(row, "depth_range")?)?;
            if !absolute_iri(id) || !seen.insert(id) || kind.len() > 64 {
                return Err("Layer measurements require distinct absolute entity IRIs and bounded layer names".into());
            }
            let band = match kind {
                "periosteum_capsule" => Some([0.00, 0.08]),
                "ligament_deep_tendon" => Some([0.05, 0.20]),
                "musculature" => Some([0.15, 0.75]),
                "subcutis_fascia" => Some([0.75, 0.92]),
                "dermis_epidermis" => Some([0.92, 1.00]),
                "internal_organ" => None,
                _ => return Err(format!("Unsupported layer kind {kind}")),
            };
            if lo < 0.0 || hi > 1.0 || lo > hi {
                violations.push(format!(
                    "{id}: normalized depth [{lo}, {hi}] violates [0, 1] or is inverted"
                ));
            }
            if let Some([min, max]) = band {
                if lo < min || hi > max {
                    violations.push(format!(
                        "{id}: depth [{lo}, {hi}] escapes {kind} envelope [{min}, {max}]"
                    ));
                }
            }
            if kind == "internal_organ" {
                for key in ["parent_cavity_iri", "suspension_iri"] {
                    if !row
                        .get(key)
                        .map(text)
                        .transpose()?
                        .is_some_and(absolute_iri)
                    {
                        violations.push(format!("{id}: missing absolute {key}"));
                    }
                }
                for key in ["max_cavity_sdf_phi_m", "cavity_clearance_eps_m"] {
                    if !row.contains_key(key) {
                        violations.push(format!("{id}: absent cavity measurement {key}"));
                    }
                }
            }
            let phi = row.get("max_cavity_sdf_phi_m").map(num).transpose()?;
            let epsilon = row.get("cavity_clearance_eps_m").map(num).transpose()?;
            if let Some(eps) = epsilon {
                if !(0.0 < eps && eps <= 1.0) {
                    return Err("cavity_clearance_eps_m must be in (0, 1] metres".into());
                }
            }
            if let (Some(phi), Some(eps)) = (phi, epsilon) {
                cavity_checks += 1;
                if phi > -eps {
                    violations.push(format!(
                        "{id}: cavity phi {phi:.9} m exceeds -epsilon {:.9} m",
                        -eps
                    ));
                }
            }
            if let Some(drift) = row.get("max_enthesis_uv_drift_m").map(num).transpose()? {
                if !(0.0..=1e-4).contains(&drift) {
                    violations.push(format!(
                        "{id}: enthesis drift {drift:.9} m violates [0, 0.0001] m"
                    ));
                }
            } else if matches!(kind, "musculature" | "ligament_deep_tendon") {
                violations.push(format!(
                    "{id}: absent maximum enthesis UV drift measurement"
                ));
            }
            if let Some(det) = row.get("volume_det_f").map(num).transpose()? {
                if !(0.85..=1.15).contains(&det) {
                    violations.push(format!("{id}: det(F) {det:.9} violates [0.85, 1.15]"));
                }
            } else if kind == "musculature" {
                violations.push(format!("{id}: absent muscular det(F) measurement"));
            }
            if matches!(kind, "musculature" | "ligament_deep_tendon") {
                let bindings = row
                    .get("enthesis_bindings")
                    .map(crate::list)
                    .transpose()?
                    .unwrap_or(&[]);
                if bindings.len() > 64 {
                    return Err("Too many distinct enthesis bindings".into());
                }
                let mut unique = BTreeSet::new();
                for value in bindings {
                    let iri = text(value)?;
                    if !absolute_iri(iri) {
                        return Err("Invalid enthesis reference IRI".into());
                    }
                    unique.insert(iri);
                }
                if unique.len() < 2 {
                    violations.push(format!(
                        "{id}: expected at least 2 distinct enthesis bindings, received {}",
                        unique.len()
                    ));
                }
            }
            let polyaxial = match row.get("crosses_polyaxial_joint") {
                Some(Value::Bool(x)) => *x,
                None => false,
                _ => return Err("crosses_polyaxial_joint must be boolean".into()),
            };
            if polyaxial
                && !row
                    .get("retinaculum_iri")
                    .map(text)
                    .transpose()?
                    .is_some_and(absolute_iri)
            {
                violations.push(format!(
                    "{id}: polyaxial crossing lacks retinaculum reference"
                ));
            }
        }
        let valid = violations.is_empty();
        let report = self.encode(
            "json",
            serde_json::to_vec(&serde_json::json!({
                "schema":"qualia-declared-radial-layer-checks/1", "passed":valid,
                "radial_checks":list.len(), "cavity_checks":cavity_checks, "violations":violations,
            }))
            .map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record(Args::from([
            ("passed".into(), Value::Bool(valid)),
            ("report".into(), report),
        ])))
    }

    pub(crate) fn step_fspm_vascular_and_leaf(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "state",
                "profile",
                "temperature_min_c",
                "temperature_max_c",
                "days",
                "light",
                "moisture",
                "wind_m_s",
            ],
        )?;
        let mut plant = match args.get("state") {
            Some(value) => read_plant(record(value)?)?,
            None => MetamerPlantGraph::new(ThermalPhenology::default()),
        };
        let profile = match args
            .get("profile")
            .map(text)
            .transpose()?
            .unwrap_or("broadleaf")
        {
            "broadleaf" => LeafEconomicsProfile::herbaceous_broadleaf(),
            "grass" => LeafEconomicsProfile::monocot_grass(),
            "sclerophyll" => LeafEconomicsProfile::eucalyptus_sclerophyll(),
            _ => return Err("Unknown supported leaf economics profile".into()),
        };
        let t_min = bounded(args, "temperature_min_c", 12.0, -25.0, 55.0)? as f32;
        let t_max = bounded(args, "temperature_max_c", 22.0, -25.0, 55.0)? as f32;
        if t_min > t_max {
            return Err("Minimum temperature exceeds maximum temperature".into());
        }
        let days = count(args, "days", 1, 30)?;
        if days == 0 {
            return Err("FSPM requires positive integral days".into());
        }
        let light = bounded(args, "light", 1.0, 0.0, 1.5)? as f32;
        let moisture = bounded(args, "moisture", 0.8, 0.0, 1.0)? as f32;
        let wind = bounded(args, "wind_m_s", 0.0, 0.0, 30.0)? as f32;
        self.charge(
            days * MAX_METAMERS * 256 + MAX_METAMERS * 65536,
            MAX_METAMERS * 32768,
        )?;
        for _ in 0..days {
            plant.update_thermal_time(t_max, t_min, 1.0);
            plant.solve_growth_cycle(light, moisture);
        }
        let state = plant_state(&plant);
        read_plant(record(&state)?).map_err(|error| format!("Next FSPM state: {error}"))?;
        let mut veins = LeafVascularSkeleton::new(profile, moisture * 1.5);
        veins.generate_venation();
        veins.propagate_murray_law();
        veins.evaluate_hydraulics(0.001);
        let droop = f64::from(veins.evaluate_droop_angle(wind));
        if !droop.is_finite() {
            return Err("Native leaf deformation is not finite".into());
        }
        let mut positions = Vec::new();
        let mut triangles = Vec::new();
        let mut tips = vec![[0.0; 3]; plant.count];
        let mut foliage = 0usize;
        for (i, node) in plant.metamers[..plant.count].iter().enumerate() {
            let root = node.parent_idx == NO_PARENT_METAMER;
            let start = if root {
                [0.0; 3]
            } else {
                tips[node.parent_idx as usize]
            };
            let (s, c) = f64::from(node.pitch_rad).sin_cos();
            let (sy, cy) = f64::from(node.yaw_rad).sin_cos();
            let axis = [s * cy, s * sy, c];
            let end = add(start, scale(axis, f64::from(node.length_m)));
            tips[i] = end;
            let radius = f64::from(plant.radius_m(i));
            append_tube(
                &mut positions,
                &mut triangles,
                start,
                end,
                if root { radius * 1.65 } else { radius },
                radius,
                8,
            )?;
            if node.organ != MetamerOrgan::Leaf {
                continue;
            }
            foliage += 1;
            let long = [cy * droop.cos(), sy * droop.cos(), -droop.sin()];
            let across = [-sy, cy, 0.0];
            let normal = cross(long, across);
            let length = f64::from(profile.leaf_length_m);
            let width = f64::from(profile.leaf_width_m);
            let blade = |u: f64, t: f64| {
                add(
                    end,
                    add(
                        scale(long, length * t),
                        add(scale(across, u), scale(normal, -0.08 * length * t * t)),
                    ),
                )
            };
            let blade_base = positions.len() as u32;
            positions.push(pack_position(blade(0.0, 0.0))?);
            for station in 1..8 {
                let t = f64::from(station) / 8.0;
                let half_width = width * 0.5 * (std::f64::consts::PI * t).sin();
                positions.push(pack_position(blade(-half_width, t))?);
                positions.push(pack_position(blade(0.0, t))?);
                positions.push(pack_position(blade(half_width, t))?);
            }
            let tip = positions.len() as u32;
            positions.push(pack_position(blade(0.0, 1.0))?);
            triangles.extend([
                [blade_base, blade_base + 1, blade_base + 2],
                [blade_base, blade_base + 2, blade_base + 3],
            ]);
            for row in 0..6 {
                let a = blade_base + 1 + 3 * row;
                for j in 0..2 {
                    let index = a + j;
                    triangles.extend([
                        [index, index + 3, index + 1],
                        [index + 1, index + 3, index + 4],
                    ]);
                }
            }
            triangles.extend([[tip - 3, tip, tip - 2], [tip - 2, tip, tip - 1]]);
            for edge in &veins.edges[..veins.edge_count] {
                let endpoint = |pos: [f32; 2]| {
                    let t = f64::from(pos[1]).clamp(0.0, 1.0);
                    let max_width = width * 0.48 * (std::f64::consts::PI * t).sin();
                    let u = (f64::from(pos[0]) * length).clamp(-max_width, max_width);
                    add(blade(u, t), scale(normal, width * 0.01))
                };
                let (a, b) = (endpoint(edge.start), endpoint(edge.end));
                let r = (f64::from(edge.radius_mm) * 0.001)
                    .min(width * 0.03)
                    .max(1e-5);
                if norm(sub(b, a)) > 1e-7 {
                    append_tube(&mut positions, &mut triangles, a, b, r, r * 0.8, 6)?;
                }
            }
        }
        let mesh = self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(make_mesh(
            positions, triangles,
        )?)))?;
        let report_bytes = serde_json::to_vec(&serde_json::json!({
            "schema":"qualia-fspm-vascular-leaf/1", "modeled_days":days, "leaf_count":foliage,
            "venation_edges_per_leaf":veins.edge_count, "leaf_droop_rad":droop,
            "leaf_elastic_modulus_mpa":veins.effective_elastic_modulus_mpa(),
            "carbon_pool_kg":plant.carbon_pool_q_kg, "accumulated_gdd":plant.phenology.accumulated_gdd,
        })).map_err(|e| e.to_string())?;
        let mut digest = Sha256::new();
        digest.update(&report_bytes);
        for node in &plant.metamers[..plant.count] {
            digest.update(node.parent_idx.to_le_bytes());
            digest.update([node.organ as u8]);
            for value in [
                node.length_m,
                node.organ_mass_kg,
                node.pitch_rad,
                node.yaw_rad,
                node.age_gdd,
            ] {
                digest.update(value.to_le_bytes());
            }
            digest.update(node.active_pipes.to_le_bytes());
            digest.update(node.retired_pipes.to_le_bytes());
        }
        let context = format!("urn:qualia:fspm:{:x}", digest.finalize());
        let mut graph = ReceiptGraph::new(&context);
        let root = format!("{context}:plant");
        for (index, node) in plant.metamers[..plant.count].iter().enumerate() {
            let id = format!("{context}:metamer:{index}");
            graph.fact(&root, "urn:qualia:botany:has-metamer", &id)?;
            let kind = format!("urn:qualia:botany:organ:{}", node.organ as u8);
            graph.fact(&id, "urn:qualia:botany:organ-kind", &kind)?;
            if node.parent_idx != NO_PARENT_METAMER {
                let parent = format!("{context}:metamer:{}", node.parent_idx);
                graph.fact(&id, "urn:qualia:botany:parent-metamer", &parent)?;
            }
        }
        let evidence =
            self.admit_encoded("q42", graph.seal(self.limits.resource_bytes.min(2 << 20))?)?;
        let report = self.encode("json", report_bytes)?;
        Ok(Value::Record(Args::from([
            ("mesh".into(), mesh),
            ("state".into(), state),
            ("report".into(), report),
            ("evidence".into(), evidence),
        ])))
    }

    pub(crate) fn amplify_terminal_phyllotaxis(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "graph",
                "assembly_frame",
                "prototype",
                "pattern",
                "per_tip",
                "max_instances",
                "angle_deg",
                "radius_m",
                "internode_m",
                "acrotony",
                "uniform_scale",
            ],
        )?;
        let graph = branches(self, required(args, "graph")?)?;
        let tree = Tree::read(&graph)?;
        let per_tip = count(args, "per_tip", 8, 64)?;
        let frame = required(args, "assembly_frame")?;
        let (frame_id, capacity) = {
            let r = crate::qualia_containers_governance_poet_ops::encoded_resource(self, frame)?;
            if r.kind != "10d" || r.bytes.len() > self.limits.resource_bytes {
                return Err("Phyllotaxis requires a bounded sealed .10d assembly frame".into());
            }
            (
                format!("{:x}", Sha256::digest(&r.bytes)),
                Self::assembly_frame_leaf_capacity(&r.bytes)?,
            )
        };
        let budget = count(args, "max_instances", capacity, capacity)?;
        let pattern = args
            .get("pattern")
            .map(text)
            .transpose()?
            .unwrap_or("shoot");
        if !matches!(pattern, "shoot" | "capitulum") {
            return Err("Phyllotaxis pattern must be shoot or capitulum".into());
        }
        let angle = bounded(args, "angle_deg", 137.507764, -360.0, 360.0)?.to_radians();
        let radius = bounded(args, "radius_m", 0.035, 1e-5, 10.0)?;
        let internode = bounded(args, "internode_m", 0.02, 1e-5, 10.0)?;
        let acrotony = bounded(args, "acrotony", 0.0, 0.0, 1.0)?;
        let uniform_scale = bounded(args, "uniform_scale", 1.0, 0.01, 100.0)?;
        let prototype = required(args, "prototype")?;
        let prototype_row = record(prototype)?;
        fields(
            prototype_row,
            &["leaf_key", "resource", "taxon_or_class_iri"],
        )?;
        let key = text(required(prototype_row, "leaf_key")?)?;
        let tips = tree
            .children
            .iter()
            .enumerate()
            .filter_map(|(i, children)| children.is_empty().then_some(i))
            .collect::<Vec<_>>();
        let requested = per_tip * tips.len();
        if per_tip == 0 || requested > budget {
            return Err(format!(
                "Phyllotaxis requires {requested} placements within capacity {budget}; partition tips or reduce per_tip"
            ));
        }
        self.charge(
            requested * 512 + graph.nodes.len() * 64,
            requested * 768 + graph.nodes.len() * 48,
        )?;
        let mut instances = Vec::with_capacity(requested);
        for tip in tips {
            let parent = tree.parents[tip].ok_or("Terminal branch has no parent")?;
            let direction = unit(sub(graph.nodes[tip], graph.nodes[parent]))
                .ok_or("Degenerate terminal branch")?;
            let tilt = if direction[2] < -1.0 + 1e-12 {
                [0.0, 1.0, 0.0, 0.0]
            } else {
                let w = (0.5 * (1.0 + direction[2])).sqrt();
                [w, -direction[1] / (2.0 * w), direction[0] / (2.0 * w), 0.0]
            };
            let rotation = Motor::from_rotation_translation(tilt, [0.0; 3]);
            let u = rotation.transform_point([1.0, 0.0, 0.0]);
            let v = rotation.transform_point([0.0, 1.0, 0.0]);
            let mut axial = 0.0;
            for k in 0..per_tip {
                let t = (k as f64 + 0.5) / per_tip as f64;
                let theta = k as f64 * angle;
                let (sin, cos) = theta.sin_cos();
                let spread = if pattern == "capitulum" {
                    radius * t.sqrt()
                } else {
                    radius
                };
                if pattern == "shoot" {
                    axial += internode * (1.0 - acrotony * t);
                }
                let position = add(
                    graph.nodes[tip],
                    add(
                        scale(direction, axial),
                        scale(add(scale(u, cos), scale(v, sin)), spread),
                    ),
                );
                let _ = pack_position(position)?;
                let (h_sin, h_cos) = (0.5 * theta).sin_cos();
                let twist = Motor::from_rotation_translation([h_cos, 0.0, 0.0, h_sin], [0.0; 3]);
                let motor = Motor::from_translation(position[0], position[1], position[2])
                    .mul(&rotation)
                    .mul(&twist);
                let pga = [
                    motor.r_w, motor.r_x, motor.r_y, motor.r_z, -motor.d_x, -motor.d_y, -motor.d_z,
                    motor.d_w,
                ];
                let mut identity = Sha256::new();
                identity.update((key.len() as u64).to_le_bytes());
                identity.update(key.as_bytes());
                for value in pga.into_iter().chain([uniform_scale]) {
                    identity.update(value.to_le_bytes());
                }
                let fingerprint = identity.finalize();
                instances.push(Value::Record(Args::from([
                    ("leaf_key".into(), Value::String(key.into())),
                    (
                        "entity_iri".into(),
                        Value::String(format!(
                            "urn:qualia:phyllotaxis:{frame_id}:{tip}:{k}:{fingerprint:x}"
                        )),
                    ),
                    (
                        "local_pga_motor".into(),
                        Value::List(pga.map(Value::F64).into()),
                    ),
                    ("uniform_scale".into(), Value::F64(uniform_scale)),
                ])));
            }
        }
        let output = self.compose_assembly_10d(&Args::from([
            ("assembly_frame".into(), frame.clone()),
            ("archetypes".into(), Value::List(vec![prototype.clone()])),
            ("instances".into(), Value::List(instances)),
        ]))?;
        let Value::Record(mut output) = output else {
            return Err("Expected typed scene assembly".into());
        };
        output.insert("instance_count".into(), Value::U64(requested as u64));
        Ok(Value::Record(output))
    }

    pub(crate) fn populate_allometric_vascular_c1(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "graph",
                "tip_radius_m",
                "alpha",
                "blend_radius_m",
                "resolution",
            ],
        )?;
        let graph = branches(self, required(args, "graph")?)?;
        let tree = Tree::read(&graph)?;
        let tip = bounded(args, "tip_radius_m", 0.03, 1e-5, 2.0)?;
        let alpha = bounded(args, "alpha", 3.0, 2.0, 3.0)?;
        let blend = bounded(args, "blend_radius_m", tip * 0.5, 1e-6, 1.0)?;
        let resolution = count(args, "resolution", 32, 40)?;
        if resolution < 16 || graph.edges.len() > 384 {
            return Err(
                "Allometric extraction requires 16–40 grid samples per axis and at most 384 edges"
                    .into(),
            );
        }
        let mut radii = vec![0.0f64; graph.nodes.len()];
        for &node in tree.ordered.iter().rev() {
            radii[node] = if tree.children[node].is_empty() {
                tip
            } else {
                tree.children[node]
                    .iter()
                    .map(|&child| radii[child].powf(alpha))
                    .sum::<f64>()
                    .powf(1.0 / alpha)
            };
        }
        let mut segments = Vec::with_capacity(graph.edges.len());
        for edge in &graph.edges {
            let [a, b] = edge.nodes.map(|v| v as usize);
            let start = graph.nodes[a];
            let end = graph.nodes[b];
            let axis = sub(end, start);
            let length = norm(axis);
            if length <= (radii[a] - radii[b]).abs() {
                return Err("Branch segment is too short for its allometric taper".into());
            }
            segments.push((
                start,
                end,
                scale(axis, 1.0 / length),
                length,
                radii[a],
                radii[b],
            ));
        }
        let (mut lower, mut upper) = point_bounds(&graph.nodes)?;
        let padding = 2.0 * radii.iter().copied().fold(0.0, f64::max) + 2.0 * blend;
        for axis in 0..3 {
            lower[axis] -= padding;
            upper[axis] += padding;
        }
        let spacing = scale(sub(upper, lower), 1.0 / (resolution - 1) as f64);
        let samples = resolution.pow(3);
        let cells = (resolution - 1).pow(3);
        self.charge(
            samples * segments.len() * 30 + cells * 48,
            cells * 920 + samples * 16,
        )?;
        let mut grid = Vec::with_capacity(samples);
        let mut field = Vec::with_capacity(samples);
        for z in 0..resolution {
            for y in 0..resolution {
                for x in 0..resolution {
                    let p = add(
                        lower,
                        [
                            spacing[0] * x as f64,
                            spacing[1] * y as f64,
                            spacing[2] * z as f64,
                        ],
                    );
                    let d = segments.iter().fold(
                        f64::INFINITY,
                        |current, &(a, b, axis, len, r_a, r_b)| {
                            let dist = tapered_capsule(p, a, b, axis, len, r_a, r_b);
                            if !current.is_finite() {
                                return dist;
                            }
                            let excess = (blend - (dist - current).abs()).max(0.0);
                            current.min(dist) - excess * excess / (4.0 * blend)
                        },
                    );
                    let d = f64::from(d as f32);
                    if !d.is_finite()
                        || (d <= 0.0
                            && [x, y, z].into_iter().any(|n| n == 0 || n + 1 == resolution))
                    {
                        return Err(
                            "Allometric field is non-finite or reaches its padded boundary".into(),
                        );
                    }
                    field.push(d as f32);
                    grid.push(d);
                }
            }
        }
        let surface = extract_closed(&grid, resolution, lower, spacing)?;
        let mesh = self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(surface)))?;
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![resolution as u32; 3],
                samples: field,
            },
        )))?;
        let report = self.encode(
            "json",
            serde_json::to_vec(&serde_json::json!({
                "schema":"qualia-allometric-branch-field/1", "origin_m":lower,"spacing_m":spacing,
                "axis_order":"zyx", "node_radius_m":radii, "alpha":alpha,"blend_radius_m":blend,
            }))
            .map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record(Args::from([
            ("mesh".into(), mesh),
            ("field".into(), field),
            ("report".into(), report),
        ])))
    }

    pub(crate) fn groom_hair_and_pelage(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "skin",
                "guides",
                "roots",
                "samples",
                "clump",
                "curl_radius_m",
                "curl_turns",
                "material",
            ],
        )?;
        let skin = mesh(self, required(args, "skin")?)?;
        let guides = list(required(args, "guides")?)?;
        let roots = list(required(args, "roots")?)?;
        let samples = count(args, "samples", 24, 128)?;
        if !(1..=256).contains(&guides.len()) || !(1..=4096).contains(&roots.len()) || samples < 2 {
            return Err("Groom requires 1–256 guides, 1–4096 roots and 2–128 samples".into());
        }
        let clump = bounded(args, "clump", 0.0, 0.0, 1.0)?;
        let curl_radius = bounded(args, "curl_radius_m", 0.0, 0.0, 0.1)?;
        let curl_turns = bounded(args, "curl_turns", 0.0, -12.0, 12.0)?;
        self.charge(
            guides.len() * 1024 + roots.len() * (guides.len() * 24 + samples * 100),
            (guides.len() * 256 + roots.len() * samples) * 32,
        )?;
        let mut guides_in_chart = Vec::with_capacity(guides.len());
        for item in guides {
            let row = record(item)?;
            fields(row, &["face", "uv", "points_m"])?;
            let face = integer(required(row, "face")?, "guide face")?;
            let patch = surface_patch(&skin, face, vector(required(row, "uv")?)?)?;
            let input = list(required(row, "points_m")?)?;
            if !(2..=256).contains(&input.len()) {
                return Err("Each guide requires 2–256 authored world-space points".into());
            }
            let points = input
                .iter()
                .map(vector::<3>)
                .collect::<Result<Vec<_>, _>>()?;
            if norm(sub(points[0], patch.origin)) > 1e-4 {
                return Err("Guide root is more than 0.1 mm from its bound UV point".into());
            }
            let local = points
                .into_iter()
                .map(|p| {
                    let v = sub(p, patch.origin);
                    patch.frame.map(|axis| dot(v, axis))
                })
                .collect::<Vec<_>>();
            let mut local = resample_by_length(&local, samples)?;
            local[0] = [0.0; 3];
            guides_in_chart.push((face, patch, local));
        }
        let mut strands = Vec::with_capacity(roots.len());
        for item in roots {
            let row = record(item)?;
            fields(row, &["face", "uv"])?;
            let face = integer(required(row, "face")?, "follicle face")?;
            let root = surface_patch(&skin, face, vector(required(row, "uv")?)?)?;
            // A face is the interpolation chart. Seams and overlapping UV islands
            // are never allowed to pull strands through an unrelated surface.
            let mut nearest = guides_in_chart
                .iter()
                .filter(|(f, _, _)| *f == face)
                .map(|(_, p, curve)| (norm(sub(root.origin, p.origin)), p, curve))
                .collect::<Vec<_>>();
            nearest.sort_by(|a, b| a.0.total_cmp(&b.0));
            nearest.truncate(4);
            if nearest.is_empty() {
                return Err(format!("Follicle face {face} has no authored guide"));
            }
            if nearest[0].0 <= 1e-9 {
                nearest.truncate(1);
            }
            let total = nearest
                .iter()
                .map(|(d, _, _)| 1.0 / d.max(1e-9))
                .sum::<f64>();
            let attract = sub(nearest[0].1.origin, root.origin);
            let mut positions = Vec::with_capacity(samples);
            for i in 0..samples {
                let t = i as f64 / (samples - 1) as f64;
                let dim = nearest.iter().fold([0.0; 3], |s, (d, _, local)| {
                    add(s, scale(local[i], 1.0 / (d.max(1e-9) * total)))
                });
                let angle = std::f64::consts::TAU * curl_turns * t;
                let (sin, cos) = angle.sin_cos();
                let curl = curl_radius * t;
                let p = add(
                    root.origin,
                    add(
                        add(
                            scale(root.frame[0], dim[0] + curl * (cos - 1.0)),
                            scale(root.frame[1], dim[1] + curl * sin),
                        ),
                        add(scale(root.frame[2], dim[2]), scale(attract, clump * t * t)),
                    ),
                );
                positions.push(pack_position(p)?);
            }
            strands.push(CurveStrand {
                root_uv: Some(root.uv.map(|x| x as f32)),
                positions,
            });
        }
        let mut material = match args.get("material") {
            Some(value) => record(value)?.clone(),
            None => Args::new(),
        };
        fields(
            &material,
            &["base_color", "roughness", "ior", "preview_size"],
        )?;
        material.insert("closure".into(), Value::String("chiang_hair".into()));
        let material = self.invoke(
            "hypermedia.surface.compile_bsdf_graph",
            &Value::Record(material),
        )?;
        let groom = self.admit_artifact(TypedArtifact::CurveGroomBundle(Arc::new(
            CurveGroomBundle { strands },
        )))?;
        Ok(Value::Record(Args::from([
            ("groom".into(), groom),
            ("material".into(), material),
        ])))
    }

    pub(crate) fn bind_enthesis_and_retinaculum(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "target", "origin", "insertion", "pulley"])?;
        let source = mesh(self, required(args, "source")?)?;
        let target = mesh(self, required(args, "target")?)?;
        let mut anchors = Vec::with_capacity(2);
        let mut maximum = 0.0f64;
        let mut endpoints = [(0, [0.0; 3]); 2];
        for (slot, name) in ["origin", "insertion"].into_iter().enumerate() {
            let row = record(required(args, name)?)?;
            fields(row, &["face", "uv", "source_vertex"])?;
            let face = integer(required(row, "face")?, "anchor face")?;
            let index = integer(required(row, "source_vertex")?, "source vertex")?;
            let position = source
                .surface
                .mesh
                .positions
                .get(index)
                .copied()
                .ok_or("Unknown source attachment vertex")?
                .map(f64::from);
            let patch = surface_patch(&target, face, vector(required(row, "uv")?)?)?;
            let drift = norm(sub(position, patch.origin));
            if !drift.is_finite() || drift > 1e-4 {
                return Err(format!(
                    "{name} enthesis drift {drift:.9} m exceeds 0.000100000 m"
                ));
            }
            maximum = maximum.max(drift);
            endpoints[slot] = (index, patch.origin);
            anchors.push(serde_json::json!({
                "role": name, "face": face, "source_vertex": index, "uv": patch.uv,
                "barycentric": patch.weights, "position_m": patch.origin, "drift_m": drift,
            }));
        }
        if endpoints[0].0 == endpoints[1].0 || norm(sub(endpoints[0].1, endpoints[1].1)) <= 1e-6 {
            return Err("Origin and insertion must bind distinct attachment points".into());
        }
        let pulley = args
            .get("pulley")
            .map(|value| {
                let row = record(value)?;
                fields(
                    row,
                    &[
                        "center_m",
                        "axis",
                        "major_radius_m",
                        "tube_radius_m",
                        "guide_points_m",
                    ],
                )?;
                let center = vector::<3>(required(row, "center_m")?)?;
                let axis = unit(vector::<3>(required(row, "axis")?)?)
                    .ok_or("Pulley axis has zero length")?;
                let major = num(required(row, "major_radius_m")?)?;
                let tube = num(required(row, "tube_radius_m")?)?;
                let points = list(required(row, "guide_points_m")?)?;
                if !(0.000001..=1.0).contains(&tube)
                    || major <= tube
                    || major > 10.0
                    || !(2..=4096).contains(&points.len())
                {
                    return Err(
                        "Pulley requires a valid ring torus and 2–4096 contact samples".into(),
                    );
                }
                self.charge(points.len() * 64, 0)?;
                let mut worst = f64::NEG_INFINITY;
                for point in points {
                    let d = sub(vector::<3>(point)?, center);
                    let h = dot(d, axis);
                    let radius = norm(sub(d, scale(axis, h)));
                    let distance = (radius - major).hypot(h) - tube;
                    if !distance.is_finite() {
                        return Err("Pulley contact distance exceeds numeric range".into());
                    }
                    worst = worst.max(distance);
                }
                if !worst.is_finite() || worst > 1e-4 {
                    return Err(format!("Pulley guide exceeds the channel by {worst:.9} m"));
                }
                Ok(
                    serde_json::json!({"center_m":center,"axis":axis,"major_radius_m":major,
                "tube_radius_m":tube,"samples":points.len(),"max_signed_distance_m":worst}),
                )
            })
            .transpose()?;
        self.charge(512, 2048)?;
        let report = self.encode(
            "json",
            serde_json::to_vec(&serde_json::json!({
                "schema":"qualia-surface-enthesis-binding/1", "anchors":anchors, "pulley":pulley,
                "max_uv_drift_m":maximum,
            }))
            .map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record(Args::from([
            ("binding".into(), report),
            ("max_uv_drift_m".into(), Value::F64(maximum)),
        ])))
    }
}

fn absolute_iri(value: &str) -> bool {
    value.len() <= 512
        && value.split_once(':').is_some_and(|(scheme, rest)| {
            !scheme.is_empty()
                && !rest.is_empty()
                && scheme.starts_with(|ch: char| ch.is_ascii_alphabetic())
                && scheme
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"+-.".contains(&b))
                && !rest.chars().any(|ch| ch.is_whitespace() || ch.is_control())
        })
}

fn read_plant(args: &Args) -> Result<MetamerPlantGraph, String> {
    fields(
        args,
        &[
            "metamers",
            "accumulated_gdd",
            "carbon_pool_kg",
            "chilling_hours",
        ],
    )?;
    let rows = list(required(args, "metamers")?)?;
    if !(1..=MAX_METAMERS).contains(&rows.len()) {
        return Err("Invalid metamer count".into());
    }
    let mut plant = MetamerPlantGraph::new(ThermalPhenology {
        accumulated_gdd: bounded(args, "accumulated_gdd", 0.0, 0.0, 1e7)? as f32,
        chilling_hours: bounded(args, "chilling_hours", 600.0, 0.0, 1e7)? as f32,
        ..ThermalPhenology::default()
    });
    plant.count = rows.len();
    plant.carbon_pool_q_kg = bounded(args, "carbon_pool_kg", 0.05, 0.0, 1e5)? as f32;
    for (i, row) in rows.iter().enumerate() {
        let row = record(row)?;
        fields(
            row,
            &[
                "parent",
                "organ",
                "length_m",
                "mass_kg",
                "pitch_rad",
                "yaw_rad",
                "age_gdd",
                "retired_pipes",
            ],
        )?;
        let parent_idx = match row.get("parent") {
            Some(v) => {
                let parent = integer(v, "metamer parent")?;
                if parent >= i {
                    return Err("Metamer parent must precede its child".into());
                }
                parent as u16
            }
            None if i == 0 => NO_PARENT_METAMER,
            None => return Err("Only the first metamer may be the root".into()),
        };
        let organ = match text(required(row, "organ")?)? {
            "internode" => MetamerOrgan::InternodeOnly,
            "bud" => MetamerOrgan::VegetativeBud,
            "leaf" => MetamerOrgan::Leaf,
            "flower" => MetamerOrgan::Flower,
            "fruit" => MetamerOrgan::Fruit,
            "root_tip" => MetamerOrgan::RootTip,
            _ => return Err("Unknown metamer organ".into()),
        };
        plant.metamers[i] = Metamer {
            parent_idx,
            organ,
            length_m: bounded(row, "length_m", 0.1, 0.001, 20.0)? as f32,
            organ_mass_kg: bounded(row, "mass_kg", 0.01, 0.0, 1e6)? as f32,
            pitch_rad: bounded(row, "pitch_rad", 0.0, -1e3, 1e3)? as f32,
            yaw_rad: bounded(row, "yaw_rad", 0.0, -1e3, 1e3)? as f32,
            age_gdd: bounded(row, "age_gdd", 0.0, 0.0, 1e7)? as f32,
            retired_pipes: count(row, "retired_pipes", 0, 1_000_000)? as u32,
            ..Metamer::default()
        };
    }
    Ok(plant)
}

fn plant_state(plant: &MetamerPlantGraph) -> Value {
    let metamers = plant.metamers[..plant.count]
        .iter()
        .map(|m| {
            let organ = match m.organ {
                MetamerOrgan::InternodeOnly => "internode",
                MetamerOrgan::VegetativeBud => "bud",
                MetamerOrgan::Leaf => "leaf",
                MetamerOrgan::Flower => "flower",
                MetamerOrgan::Fruit => "fruit",
                MetamerOrgan::RootTip => "root_tip",
            };
            let mut row = Args::from([
                ("organ".into(), Value::String(organ.into())),
                ("length_m".into(), Value::F64(m.length_m.into())),
                ("mass_kg".into(), Value::F64(m.organ_mass_kg.into())),
                ("pitch_rad".into(), Value::F64(m.pitch_rad.into())),
                ("yaw_rad".into(), Value::F64(m.yaw_rad.into())),
                ("age_gdd".into(), Value::F64(m.age_gdd.into())),
                ("retired_pipes".into(), Value::U64(m.retired_pipes.into())),
            ]);
            if m.parent_idx != NO_PARENT_METAMER {
                row.insert("parent".into(), Value::U64(m.parent_idx.into()));
            }
            Value::Record(row)
        })
        .collect();
    Value::Record(Args::from([
        ("metamers".into(), Value::List(metamers)),
        (
            "accumulated_gdd".into(),
            Value::F64(plant.phenology.accumulated_gdd.into()),
        ),
        (
            "chilling_hours".into(),
            Value::F64(plant.phenology.chilling_hours.into()),
        ),
        (
            "carbon_pool_kg".into(),
            Value::F64(plant.carbon_pool_q_kg.into()),
        ),
    ]))
}

fn append_tube(
    positions: &mut Vec<[f32; 3]>,
    triangles: &mut Vec<[u32; 3]>,
    start: Vec3,
    end: Vec3,
    start_radius: f64,
    end_radius: f64,
    sides: u32,
) -> Result<(), String> {
    let axis = unit(sub(end, start)).ok_or("Degenerate procedural tube")?;
    let (u, v) = polar_frame(axis)?;
    let base = positions.len() as u32;
    for (p, radius) in [(start, start_radius), (end, end_radius)] {
        for index in 0..sides {
            let (s, c) = (std::f64::consts::TAU * f64::from(index) / f64::from(sides)).sin_cos();
            positions.push(pack_position(add(
                p,
                add(scale(u, c * radius), scale(v, s * radius)),
            ))?);
        }
    }
    let a_cap = positions.len() as u32;
    positions.push(pack_position(start)?);
    positions.push(pack_position(end)?);
    for index in 0..sides {
        let a = base + index;
        let b = base + (index + 1) % sides;
        triangles.extend([
            [a, b, a + sides],
            [b, b + sides, a + sides],
            [a_cap, b, a],
            [a_cap + 1, a + sides, b + sides],
        ]);
    }
    Ok(())
}

fn branches(host: &AssetHost<'_>, value: &Value) -> Result<Arc<BranchGraph10d>, String> {
    match host.workspace.typed(value)? {
        TypedArtifact::BranchGraph10d(graph) => Ok(Arc::clone(graph)),
        _ => Err("Expected a branch_graph_10d handle".into()),
    }
}

fn polar_frame(axis: Vec3) -> Result<(Vec3, Vec3), String> {
    let helper = if axis[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u = unit(cross(helper, axis)).ok_or("Cannot construct branch frame")?;
    Ok((u, cross(axis, u)))
}

fn rotate_x_to([x, y, z]: Vec3) -> [f64; 4] {
    if x >= 0.0 {
        let cos = (0.5 * (1.0 + x)).sqrt();
        return [cos, 0.0, -z / (2.0 * cos), y / (2.0 * cos)];
    }
    let perpendicular = y.hypot(z);
    if perpendicular == 0.0 {
        return [0.0, 0.0, 1.0, 0.0];
    }
    let sin = (0.5 * (1.0 - x)).sqrt();
    [
        perpendicular / (2.0 * sin),
        0.0,
        -sin * z / perpendicular,
        sin * y / perpendicular,
    ]
}

fn tapered_capsule(p: Vec3, a: Vec3, b: Vec3, axis: Vec3, len: f64, r_a: f64, r_b: f64) -> f64 {
    let delta = sub(p, a);
    let axial = dot(delta, axis);
    let radial = norm(sub(delta, scale(axis, axial)));
    let slope = (r_a - r_b) / len;
    let lateral = (1.0 - slope * slope).sqrt();
    let where_on_hull = -slope * radial + lateral * axial;
    if where_on_hull < 0.0 {
        norm(delta) - r_a
    } else if where_on_hull > lateral * len {
        norm(sub(p, b)) - r_b
    } else {
        lateral * radial + slope * axial - r_a
    }
}

fn point_bounds(points: &[Vec3]) -> Result<(Vec3, Vec3), String> {
    let first = *points.first().ok_or("Cannot bound an empty point set")?;
    let (mut lo, mut hi) = (first, first);
    for p in points {
        for i in 0..3 {
            lo[i] = lo[i].min(p[i]);
            hi[i] = hi[i].max(p[i]);
        }
    }
    Ok((lo, hi))
}

fn extract_closed(
    grid: &[f64],
    resolution: usize,
    min: Vec3,
    step: Vec3,
) -> Result<AttributedMesh, String> {
    let cells = (resolution - 1).pow(3);
    let mut vertices = vec![Point3::new(0.0, 0.0, 0.0); cells * 30];
    let mut triangles = vec![[0; 3]; cells * 10];
    let (v, f) = marching_cubes(
        grid,
        resolution,
        resolution,
        resolution,
        step[0],
        step[1],
        step[2],
        min[0],
        min[1],
        min[2],
        0.0,
        &mut vertices,
        &mut triangles,
    )
    .map_err(|e| format!("Allometric isosurface: {e:?}"))?;
    if v == 0 || v > 200_000 || f == 0 || f > 400_000 {
        return Err("Allometric field did not yield a bounded nonempty mesh".into());
    }
    let mut remap = Vec::with_capacity(v);
    let mut positions = Vec::new();
    let mut keys = BTreeMap::new();
    for point in &vertices[..v] {
        // Native marching cubes emits a vertex for each triangle corner. Use
        // the lattice edge as its identity so neighbouring cells share vertices
        // even when reverse interpolation gives slightly different floats.
        let coordinate = sub([point.x, point.y, point.z], min);
        let mut edge_axis = None;
        let mut anchor = [0; 3];
        for i in 0..3 {
            let value = coordinate[i] / step[i];
            let nearest = value.round();
            let integral = (value - nearest).abs() <= 1e-7;
            if !integral && edge_axis.replace(i).is_some() {
                return Err("Marching-cubes vertex is not on a lattice edge".into());
            }
            let anchor_i = if integral { nearest } else { value.floor() };
            if !(0.0..resolution as f64).contains(&anchor_i)
                || (!integral && anchor_i + 1.0 >= resolution as f64)
            {
                return Err("Marching-cubes vertex is outside its lattice".into());
            }
            anchor[i] = anchor_i as usize;
        }
        let key = (anchor, edge_axis);
        let index = match keys.entry(key) {
            std::collections::btree_map::Entry::Occupied(entry) => *entry.get(),
            std::collections::btree_map::Entry::Vacant(entry) => {
                let mut xyz = add(min, std::array::from_fn(|i| anchor[i] as f64 * step[i]));
                if let Some(axis) = edge_axis {
                    let base = anchor[0] + anchor[1] * resolution + anchor[2] * resolution.pow(2);
                    let d0 = grid[base];
                    let d1 = grid[base + resolution.pow(axis as u32)];
                    let t = if (d1 - d0).abs() >= 1e-20 {
                        -d0 / (d1 - d0)
                    } else {
                        0.5
                    };
                    xyz[axis] += t.clamp(0.0, 1.0) * step[axis];
                }
                positions.push(pack_position(xyz)?);
                *entry.insert((positions.len() - 1) as u32)
            }
        };
        remap.push(index);
    }
    drop(vertices);
    drop(keys);
    triangles.truncate(f);
    let mut edges = BTreeMap::<[u32; 2], (usize, i32)>::new();
    for triangle in &mut triangles {
        *triangle = triangle.map(|i| remap[i as usize]);
        // The native lookup winds toward negative samples. These meshes use
        // counterclockwise faces when viewed from outside the filled field.
        triangle.swap(1, 2);
        let [a, b, c] = *triangle;
        let p = positions[a as usize].map(f64::from);
        let q = positions[b as usize].map(f64::from);
        let r = positions[c as usize].map(f64::from);
        if norm(cross(sub(q, p), sub(r, p))) == 0.0 {
            return Err("Isosurface contains a collapsed triangle".into());
        }
        for [i, j] in [[a, b], [b, c], [c, a]] {
            let entry = edges.entry([i.min(j), i.max(j)]).or_default();
            entry.0 += 1;
            entry.1 += if i < j { 1 } else { -1 };
        }
    }
    if edges
        .values()
        .any(|&(count, direction)| count != 2 || direction != 0)
    {
        return Err("Sampled allometric surface failed the oriented closed-edge check".into());
    }
    make_mesh(positions, triangles)
}

fn make_mesh(positions: Vec<[f32; 3]>, triangles: Vec<[u32; 3]>) -> Result<AttributedMesh, String> {
    let mut min = *positions.first().ok_or("No positions in procedural mesh")?;
    let mut max = min;
    for p in &positions {
        for i in 0..3 {
            min[i] = min[i].min(p[i]);
            max[i] = max[i].max(p[i]);
        }
    }
    Ok(AttributedMesh {
        surface: ImportedMesh {
            mesh: Mesh {
                positions,
                triangles,
                min,
                max,
            },
            normals: None,
            tangents: None,
            texture_coordinates_0: None,
            materials: vec![],
            texture_dependencies: vec![],
            submeshes: vec![],
        },
        uv1: vec![],
        vertex_colors: vec![],
        skin_weights: vec![],
        morph_targets: Arc::from([]),
        protected_faces: vec![],
    })
}

fn mesh(host: &AssetHost<'_>, value: &Value) -> Result<Arc<AttributedMesh>, String> {
    match host.workspace.typed(value)? {
        TypedArtifact::AttributedMesh(mesh) => Ok(Arc::clone(mesh)),
        _ => Err("Expected an attributed mesh handle".into()),
    }
}

fn surface_patch(mesh: &AttributedMesh, face: usize, uv: [f64; 2]) -> Result<SurfacePatch, String> {
    let surface = &mesh.surface;
    let triangle = *surface.mesh.triangles.get(face).ok_or("Unknown UV face")?;
    let chart = surface
        .texture_coordinates_0
        .as_ref()
        .ok_or("Mesh has no UV chart")?;
    let mut coords = [[0.0; 2]; 3];
    let mut positions = [[0.0; 3]; 3];
    for (slot, vertex) in triangle.into_iter().enumerate() {
        coords[slot] = chart
            .get(vertex as usize)
            .ok_or("UV chart does not cover face vertex")?
            .map(f64::from);
        positions[slot] = surface
            .mesh
            .positions
            .get(vertex as usize)
            .ok_or("Face vertex out of bounds")?
            .map(f64::from);
    }
    let [a, b, c] = coords;
    let d = [b[0] - a[0], b[1] - a[1]];
    let e = [c[0] - a[0], c[1] - a[1]];
    let determinant = d[0] * e[1] - d[1] * e[0];
    if !determinant.is_finite() || determinant.abs() <= 1e-14 {
        return Err("UV chart triangle is degenerate".into());
    }
    let x = [uv[0] - a[0], uv[1] - a[1]];
    let b_weight = (x[0] * e[1] - x[1] * e[0]) / determinant;
    let c_weight = (d[0] * x[1] - d[1] * x[0]) / determinant;
    let w = [1.0 - b_weight - c_weight, b_weight, c_weight];
    if w.iter().any(|x| !(-1e-6..=1.0 + 1e-6).contains(x)) {
        return Err("Bound UV point lies outside its authored triangle".into());
    }
    let w = w.map(|x| x.max(0.0));
    let weights = scale(w, 1.0 / w.into_iter().sum::<f64>());
    let side = sub(positions[1], positions[0]);
    let other = sub(positions[2], positions[0]);
    let normal = unit(cross(side, other)).ok_or("Degenerate spatial face")?;
    let tangent = unit(scale(
        sub(scale(side, e[1]), scale(other, d[1])),
        1.0 / determinant,
    ))
    .ok_or("Degenerate UV tangent")?;
    let origin = positions
        .into_iter()
        .zip(weights)
        .fold([0.0; 3], |sum, (p, w)| add(sum, scale(p, w)));
    let uv = std::array::from_fn(|axis| {
        coords
            .into_iter()
            .zip(weights)
            .map(|(p, w)| p[axis] * w)
            .sum()
    });
    Ok(SurfacePatch {
        weights,
        uv,
        origin,
        frame: [tangent, cross(normal, tangent), normal],
    })
}

fn resample_by_length(points: &[Vec3], samples: usize) -> Result<Vec<Vec3>, String> {
    let mut lengths = vec![0.0f64];
    for pair in points.windows(2) {
        let length = norm(sub(pair[1], pair[0]));
        if !(1e-9..=10.0).contains(&length) {
            return Err("Guide segment length outside admitted range".into());
        }
        lengths.push(lengths[lengths.len() - 1] + length);
    }
    let total = lengths[lengths.len() - 1];
    let mut output = Vec::with_capacity(samples);
    let mut edge = 0;
    for i in 0..samples {
        let distance = total * i as f64 / (samples - 1) as f64;
        while edge + 2 < lengths.len() && lengths[edge + 1] < distance {
            edge += 1;
        }
        let t = ((distance - lengths[edge]) / (lengths[edge + 1] - lengths[edge])).clamp(0.0, 1.0);
        output.push(add(
            points[edge],
            scale(sub(points[edge + 1], points[edge]), t),
        ));
    }
    Ok(output)
}

fn bounded(args: &Args, key: &str, default: f64, lo: f64, hi: f64) -> Result<f64, String> {
    let x = number(args, key, default)?;
    if (lo..=hi).contains(&x) {
        Ok(x)
    } else {
        Err(format!("{key} outside [{lo}, {hi}]"))
    }
}

fn pack_position(p: Vec3) -> Result<[f32; 3], String> {
    if p.iter().all(|x| x.is_finite() && x.abs() <= 1e6) {
        Ok(p.map(|x| x as f32))
    } else {
        Err("Procedural position exceeds the coordinate envelope".into())
    }
}

fn add(a: Vec3, b: Vec3) -> Vec3 {
    std::array::from_fn(|i| a[i] + b[i])
}
fn sub(a: Vec3, b: Vec3) -> Vec3 {
    std::array::from_fn(|i| a[i] - b[i])
}
fn scale(a: Vec3, k: f64) -> Vec3 {
    a.map(|x| x * k)
}
fn dot(a: Vec3, b: Vec3) -> f64 {
    a.into_iter().zip(b).map(|(x, y)| x * y).sum()
}
fn norm(a: Vec3) -> f64 {
    dot(a, a).sqrt()
}
fn unit(a: Vec3) -> Option<Vec3> {
    let len = norm(a);
    (len.is_finite() && len > 1e-12).then(|| scale(a, 1.0 / len))
}
fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
