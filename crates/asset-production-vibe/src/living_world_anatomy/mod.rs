//! Wave 12 Living-World Categorical Morphisms:
//! - `hypermedia.manifold.compile_botanical_organ_10d`: 4-scale botanical `.10d` anatomy
//!   (`WholeOrganism`, `StemBarkWedge`, `FloralAndGumnutDissection`, `LeafVenationAndHistology`).
//! - `hypermedia.manifold.compile_vertebrate_anatomy_10d`: Scale-5 comparative vertebrate 3-layer
//!   `.10d` anatomy (`Osteology`, `Myology`, `Integument`) with zero-heap anatomical layer peel.
//! - `hypermedia.kinetics.rig_biomechanical_archetype`: 18-joint PGA `Motor` fauna skeletal rig
//!   with ABSM locomotion, `PROC1` respiration/tail undulation, 2-bone & FABRIK IK, and `.10d` `SKL1` encoding.
//! - `hypermedia.discourse.query_anatomical_ontology_pin`: Zero-heap ray-sphere anatomical pin query
//!   grounded in Uberon, Plant Ontology (PO), Foundational Model of Anatomy (FMA), and TDWG Darwin Core,
//!   aligned with international human rights instruments (UDHR Art. 19, 27; ICESCR Art. 15) and
//!   Informatics Fiduciary protections.
//! - `hypermedia.geodesy.step_sector_continuum_3d`: 3D XPBD + analytical SDF + 2-way continuum
//!   soil/hydro/botanical bridge + 4-wheel rover suspension + tensile stress-cleavage dynamic fracture.
//! - `hypermedia.layout.synchronize_companion_frustum`: Multi-projector conical dome frustum cluster
//!   slicing and deterministic 64-byte UDP multicast `FrameLockPacket` CRC-32C verification.

use crate::*;
use qualia_core_db::etalon_render::spatial_math::pga::{motor_translate, Motor as PgaMotor32};
use qualia_core_db::etalon_render::vegetation::{MetamerPlantGraph, ThermalPhenology};
use qualia_core_db::etalon_render::water_surface::SimdGerstnerSpectrum;
use qualia_core_db::render::skeletal::fauna_rig::FaunaBiomechanicalArchetype;
use qualia_core_db::simulation::environment::SoilMechanicsProfile;
use qualia_core_db::simulation::physics3d::{
    couple_rigid_body_with_botanical_stem, couple_rigid_body_with_hydro,
    couple_rigid_body_with_soil, evaluate_and_fracture_body, ray_sdf_local, solve_motor_joint_xpbd,
    step_xpbd_world, BallMotorJoint, BroadphaseBvh, ColliderShape3D, ContactManifold3D,
    MotorJoint3D, RigidBody3D, SdfPrimitive, SpringSuspensionJoint,
};
use vibe::Value;

pub mod behavioral_hooks;
pub mod human_welfare_anatomy;
pub mod whitegum_canopy;
mod botanical_vertebrate_10d;
pub mod portable_scaffold;
mod rig_ontology_continuum_frustum;
pub mod corpus_values;
pub mod joint_rom;
mod morphogenesis_and_wave09;
pub mod rule_pack;
pub mod species_spec;
#[cfg(test)]
mod species_spec_tests;
#[cfg(test)]
mod botanical_visual_tests;
pub mod turntable_render;

pub(super) fn opt_string(a: &Args, k: &str, default: &str) -> Result<String, String> {
    match a.get(k) {
        None => Ok(default.to_string()),
        Some(v) => Ok(text(v)?.to_string()),
    }
}

pub(super) fn opt_point3(a: &Args, k: &str, default: [f64; 3]) -> Result<[f64; 3], String> {
    match a.get(k) {
        None => Ok(default),
        Some(v) => vector::<3>(v),
    }
}

/// Deterministic 3D sector physics verification step result.
#[derive(Debug, Clone)]
pub struct SectorContinuumStepReport {
    pub sector_id: String,
    pub sdf_ray_hit: bool,
    pub xpbd_joint_error_m: f64,
    pub rover_grounded_wheels: usize,
    pub continuum_tree_sway_rad: f64,
    pub fractured_fragments: usize,
}

/// Runs a deterministic 3D XPBD + analytical SDF + 2-way continuum bridge + 4-wheel rover suspension
/// + stress-cleavage dynamic fracture verification step for `sector_id`.
pub fn run_sector_physics_verification_step(
    sector_id: &str,
    steps: usize,
) -> SectorContinuumStepReport {
    // 1. Analytical SDF karst formation raycast (O(1) local evaluation + sphere tracing).
    let karst_sdf = SdfPrimitive::SmoothUnionPair {
        center_a: [-0.35, 0.0, 0.0],
        radius_a: 0.75,
        center_b: [0.35, 0.0, 0.0],
        radius_b: 0.75,
        blend_radius: 0.35,
    };
    let sdf_hit = ray_sdf_local([0.0, 0.0, -3.0], [0.0, 0.0, 1.0], &karst_sdf, 10.0);
    let sdf_ray_hit = sdf_hit.is_some();

    // 2. 3D XPBD world with static ground, chassis body, 4 rover wheel bodies, and a jointed pendulum link.
    let ground = RigidBody3D::new_static(
        motor_translate([0.0, -0.5, 0.0]),
        ColliderShape3D::Cuboid {
            half_extents: [50.0, 0.5, 50.0],
        },
    );
    let chassis = RigidBody3D::new_dynamic(
        motor_translate([0.0, 0.85, 0.0]),
        120.0,
        ColliderShape3D::Cuboid {
            half_extents: [1.1, 0.35, 1.8],
        },
    );
    let wheel_offsets: [[f32; 3]; 4] = [
        [-0.95, 0.32, 1.35],
        [0.95, 0.32, 1.35],
        [-0.95, 0.32, -1.35],
        [0.95, 0.32, -1.35],
    ];
    let mut bodies = [
        ground,
        chassis,
        RigidBody3D::new_dynamic(
            motor_translate(wheel_offsets[0]),
            18.0,
            ColliderShape3D::Sphere { radius: 0.35 },
        ),
        RigidBody3D::new_dynamic(
            motor_translate(wheel_offsets[1]),
            18.0,
            ColliderShape3D::Sphere { radius: 0.35 },
        ),
        RigidBody3D::new_dynamic(
            motor_translate(wheel_offsets[2]),
            18.0,
            ColliderShape3D::Sphere { radius: 0.35 },
        ),
        RigidBody3D::new_dynamic(
            motor_translate(wheel_offsets[3]),
            18.0,
            ColliderShape3D::Sphere { radius: 0.35 },
        ),
        // Articulated sensor boom attached to chassis via BallMotorJoint
        RigidBody3D::new_dynamic(
            motor_translate([0.0, 1.35, 0.0]),
            8.0,
            ColliderShape3D::Sphere { radius: 0.2 },
        ),
    ];

    let mut joints = [
        MotorJoint3D::SpringSuspension(SpringSuspensionJoint {
            body_a: 1,
            body_b: 2,
            local_anchor_a: [-0.95, -0.20, 1.35],
            local_anchor_b: [0.0, 0.0, 0.0],
            rest_length: 0.35,
            stiffness: 18_000.0,
            damping: 1_800.0,
        }),
        MotorJoint3D::SpringSuspension(SpringSuspensionJoint {
            body_a: 1,
            body_b: 3,
            local_anchor_a: [0.95, -0.20, 1.35],
            local_anchor_b: [0.0, 0.0, 0.0],
            rest_length: 0.35,
            stiffness: 18_000.0,
            damping: 1_800.0,
        }),
        MotorJoint3D::SpringSuspension(SpringSuspensionJoint {
            body_a: 1,
            body_b: 4,
            local_anchor_a: [-0.95, -0.20, -1.35],
            local_anchor_b: [0.0, 0.0, 0.0],
            rest_length: 0.35,
            stiffness: 18_000.0,
            damping: 1_800.0,
        }),
        MotorJoint3D::SpringSuspension(SpringSuspensionJoint {
            body_a: 1,
            body_b: 5,
            local_anchor_a: [0.95, -0.20, -1.35],
            local_anchor_b: [0.0, 0.0, 0.0],
            rest_length: 0.35,
            stiffness: 18_000.0,
            damping: 1_800.0,
        }),
        MotorJoint3D::Ball(BallMotorJoint {
            body_a: 1,
            body_b: 6,
            local_anchor_a: [0.0, 0.50, 0.0],
            local_anchor_b: [0.0, 0.0, 0.0],
            compliance: 1e-6,
        }),
    ];

    let mut manifolds = [ContactManifold3D::default(); 32];
    let mut manifold_count = 0usize;
    let mut bvh: BroadphaseBvh<16> = BroadphaseBvh::new(0.10);
    let dt = 1.0 / 64.0f32;

    let soil_profile = SoilMechanicsProfile::loam();
    let gerstner_spectrum = SimdGerstnerSpectrum::default();
    let mut plant_graph = MetamerPlantGraph::new(ThermalPhenology::default());
    plant_graph.solve_growth_cycle(1.0, 0.8);

    let mut tree_sway_rad = 0.0f32;

    for step_idx in 0..steps.max(1) {
        step_xpbd_world(
            &mut bodies,
            &mut joints,
            &mut manifolds,
            &mut manifold_count,
            &mut bvh,
            [0.0, -9.81, 0.0],
            dt,
            4,
        );
        for j in &joints {
            solve_motor_joint_xpbd(j, &mut bodies, dt);
        }

        // 3. Two-way continuum coupling (Bekker-Wong soil + Archimedes/Gerstner hydro + FSPM botanical sway).
        let mut soil_sinkage_max = 0.0f32;
        for wi in 2..=5 {
            let soil = couple_rigid_body_with_soil(
                &mut bodies[wi],
                4_500.0,
                0.04,
                0.45,
                &soil_profile,
                dt,
            );
            soil_sinkage_max = soil_sinkage_max.max(soil.sinkage_depth_m);
        }
        let hydro = couple_rigid_body_with_hydro(
            &mut bodies[1],
            &gerstner_spectrum,
            (step_idx as f32) * dt,
            dt,
        );
        debug_assert!(soil_sinkage_max.is_finite() && hydro.buoyant_force_n.is_finite());

        // Give a probe body a brush impulse against a roadside Corymbia sapling on step 0
        if step_idx == 0 {
            let mut probe = RigidBody3D::new_dynamic(
                motor_translate([0.0, 0.2, 0.0]),
                25.0,
                ColliderShape3D::Sphere { radius: 0.3 },
            );
            probe.linear_velocity = [3.0, 0.0, 0.0];
            let bot = couple_rigid_body_with_botanical_stem(
                &mut probe,
                &mut plant_graph,
                1,
                0.02,
                0.05,
                65.0,
            );
            tree_sway_rad = bot.elastic_deflection_rad;
        }
    }

    // Measure BallMotorJoint anchor error between chassis (body 1) and boom (body 6).
    let anchor_a = bodies[1].pose.transform_point([0.0, 0.50, 0.0]);
    let anchor_b = bodies[6].pose.transform_point([0.0, 0.0, 0.0]);
    let dx = (anchor_a[0] - anchor_b[0]) as f64;
    let dy = (anchor_a[1] - anchor_b[1]) as f64;
    let dz = (anchor_a[2] - anchor_b[2]) as f64;
    let xpbd_joint_error_m = (dx * dx + dy * dy + dz * dz).sqrt();

    let mut rover_grounded_wheels = 0usize;
    for wi in 2..=5 {
        let wy = bodies[wi].pose.translation()[1];
        if wy <= 0.45 {
            rover_grounded_wheels += 1;
        }
    }

    // 4. Principled tensile stress-cleavage dynamic fracture on a weathered limestone block.
    let karst_block = RigidBody3D::new_dynamic(
        PgaMotor32::identity(),
        16.0,
        ColliderShape3D::Cuboid {
            half_extents: [0.6, 0.4, 0.5],
        },
    );
    let fracture_outcome =
        evaluate_and_fracture_body(&karst_block, 65.0, 0.08, [0.0, 1.0, 0.0], 120.0);
    let fractured_fragments = if fracture_outcome.is_some() { 2 } else { 1 };

    SectorContinuumStepReport {
        sector_id: sector_id.to_string(),
        sdf_ray_hit,
        xpbd_joint_error_m,
        rover_grounded_wheels,
        continuum_tree_sway_rad: (tree_sway_rad.abs() as f64).max(1e-4),
        fractured_fragments,
    }
}

impl AssetHost<'_> {
    /// Categorical operations dispatch for Wave 12 Living-World anatomy, fauna rigging,
    /// biological ontology pins, 3D continuum physics, and dome frustum frame-lock.
    pub(crate) fn living_world_anatomy_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.manifold.compile_botanical_organ_10d"
            | "manifold.compile_botanical_organ_10d"
            | "synth.manifold.compile_botanical_organ_10d"
            | "compile_botanical_organ_10d" => self.compile_botanical_organ_10d(a),

            "hypermedia.manifold.compile_vertebrate_anatomy_10d"
            | "manifold.compile_vertebrate_anatomy_10d"
            | "synth.manifold.compile_vertebrate_anatomy_10d"
            | "compile_vertebrate_anatomy_10d" => self.compile_vertebrate_anatomy_10d(a),

            "hypermedia.kinetics.rig_biomechanical_archetype"
            | "kinetics.rig_biomechanical_archetype"
            | "synth.kinetics.rig_biomechanical_archetype"
            | "rig_biomechanical_archetype" => self.rig_biomechanical_archetype(a),

            "hypermedia.discourse.query_anatomical_ontology_pin"
            | "discourse.query_anatomical_ontology_pin"
            | "synth.discourse.query_anatomical_ontology_pin"
            | "query_anatomical_ontology_pin" => self.query_anatomical_ontology_pin(a),

            "hypermedia.geodesy.step_sector_continuum_3d"
            | "geodesy.step_sector_continuum_3d"
            | "synth.geodesy.step_sector_continuum_3d"
            | "step_sector_continuum_3d" => self.step_sector_continuum_3d(a),

            "hypermedia.layout.synchronize_companion_frustum"
            | "layout.synchronize_companion_frustum"
            | "synth.layout.synchronize_companion_frustum"
            | "synchronize_companion_frustum" => self.synchronize_companion_frustum(a),

            "qualia.species.assemble_corpus"
            | "species.assemble_corpus"
            | "assemble_species_corpus" => self.assemble_species_corpus(a),

            "hypermedia.manifold.calibrate_biological_subject"
            | "manifold.calibrate_biological_subject"
            | "synth.manifold.calibrate_biological_subject"
            | "calibrate_biological_subject" => self.calibrate_biological_subject(a),

            "hypermedia.kinetics.evaluate_mobility_fracture_risk"
            | "kinetics.evaluate_mobility_fracture_risk"
            | "synth.kinetics.evaluate_mobility_fracture_risk"
            | "evaluate_mobility_fracture_risk" => self.evaluate_mobility_fracture_risk(a),

            _ => Err(format!("Unknown living-world anatomy operation: {id}")),
        }
    }

    fn calibrate_biological_subject(&self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "stature_m",
                "mass_kg",
                "age_years",
                "biological_sex",
                "reproductive_phase",
                "cortical_vbmd_mg_cm3",
                "trabecular_bv_tv",
                "epistemic_tier",
                "observation_method",
                "confidence",
            ],
        )?;
        let stature_m = number(a, "stature_m", 1.72)?;
        let mass_kg = number(a, "mass_kg", 70.0)?;
        let age_years = number(a, "age_years", 30.0)?;
        let biological_sex = opt_string(a, "biological_sex", "female")?;
        let reproductive_phase = opt_string(a, "reproductive_phase", "baseline")?;
        let cortical_vbmd_mg_cm3 = number(a, "cortical_vbmd_mg_cm3", 1150.0)?;
        let trabecular_bv_tv = number(a, "trabecular_bv_tv", 0.22)?;
        let epistemic_tier = opt_string(a, "epistemic_tier", "InferredFromObservation")?;
        let observation_method = opt_string(a, "observation_method", "q42m:DEXA")?;
        let confidence = number(a, "confidence", 0.90)?;

        if !stature_m.is_finite() || stature_m <= 0.0 {
            return Err("calibrate_biological_subject: stature_m must be finite and > 0".into());
        }
        if !mass_kg.is_finite() || mass_kg <= 0.0 {
            return Err("calibrate_biological_subject: mass_kg must be finite and > 0".into());
        }
        if !age_years.is_finite() || !(0.0..=130.0).contains(&age_years) {
            return Err(
                "calibrate_biological_subject: age_years must be finite and in [0, 130]".into(),
            );
        }
        if !cortical_vbmd_mg_cm3.is_finite() || cortical_vbmd_mg_cm3 <= 0.0 {
            return Err(
                "calibrate_biological_subject: cortical_vbmd_mg_cm3 must be finite and > 0".into(),
            );
        }
        if !trabecular_bv_tv.is_finite() || trabecular_bv_tv <= 0.0 || trabecular_bv_tv > 1.0 {
            return Err(
                "calibrate_biological_subject: trabecular_bv_tv must be finite and in (0, 1]"
                    .into(),
            );
        }
        if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
            return Err(
                "calibrate_biological_subject: confidence must be finite and in [0, 1]".into(),
            );
        }

        let sex_norm = biological_sex.trim().to_ascii_lowercase();
        let is_female = match sex_norm.as_str() {
            "female" | "xx" | "f" => true,
            "male" | "xy" | "m" => false,
            "other" => false,
            _ => {
                return Err(format!(
                    "calibrate_biological_subject: unrecognized biological_sex '{biological_sex}'"
                ));
            }
        };

        let phase_norm = reproductive_phase.trim().to_ascii_lowercase();
        let (relaxin_ng_ml, phase_bmd_factor, is_postmenopausal) = match phase_norm.as_str() {
            "baseline" | "none" => (0.0, 1.0, false),
            "ovulatory" => (0.25, 1.0, false),
            "trimester1" | "first_trimester" => (1.0, 1.0, false),
            "trimester2" | "second_trimester" => (0.85, 0.98, false),
            "trimester3" | "third_trimester" => (0.95, 0.95, false),
            "lactation" => (0.05, 0.94, false),
            "perimenopause" => (0.0, 0.95, false),
            "menopause" | "postmenopausal" => (0.0, 1.0, true),
            _ => {
                return Err(format!(
                    "calibrate_biological_subject: unrecognized reproductive_phase '{reproductive_phase}'"
                ));
            }
        };

        let ligament_stiffness_modifier = 1.0 - 0.35 * (relaxin_ng_ml / (relaxin_ng_ml + 0.5));
        let age_bmd_mult = if age_years < 25.0 {
            0.45 + 0.55 * (age_years / 25.0)
        } else if age_years <= 35.0 {
            1.0
        } else {
            let years_past_peak = age_years - 35.0;
            let base_decline = 0.006 * years_past_peak;
            if is_female && is_postmenopausal {
                let estrogen_withdrawal = 0.18 * (1.0 - (-0.10 * years_past_peak).exp());
                (1.0 - base_decline - estrogen_withdrawal).max(0.35)
            } else {
                (1.0 - base_decline).max(0.40)
            }
        };

        let bone_scale_y = stature_m / 1.70;
        let allometric_radial = (mass_kg / 68.0).powf(0.375) * (1.70 / stature_m).powf(0.125);
        let cortical_apparent_density_g_cm3 = 0.70 + 0.001 * cortical_vbmd_mg_cm3;
        let trabecular_apparent_density_g_cm3 =
            trabecular_bv_tv * 1.85 * age_bmd_mult * phase_bmd_factor;
        let strain_factor = 0.01_f64.powf(0.06);
        let effective_cortical_modulus_gpa =
            3.790 * cortical_apparent_density_g_cm3.powi(3) * strain_factor;
        let effective_trabecular_modulus_gpa =
            3.790 * trabecular_apparent_density_g_cm3.powi(3) * strain_factor;

        let mut out = std::collections::BTreeMap::new();
        out.insert("bone_scale_x".into(), Value::F64(allometric_radial));
        out.insert("bone_scale_y".into(), Value::F64(bone_scale_y));
        out.insert("bone_scale_z".into(), Value::F64(allometric_radial));
        out.insert(
            "cortical_apparent_density_g_cm3".into(),
            Value::F64(cortical_apparent_density_g_cm3),
        );
        out.insert(
            "trabecular_apparent_density_g_cm3".into(),
            Value::F64(trabecular_apparent_density_g_cm3),
        );
        out.insert(
            "effective_cortical_modulus_gpa".into(),
            Value::F64(effective_cortical_modulus_gpa),
        );
        out.insert(
            "effective_trabecular_modulus_gpa".into(),
            Value::F64(effective_trabecular_modulus_gpa),
        );
        out.insert(
            "ligament_stiffness_modifier".into(),
            Value::F64(ligament_stiffness_modifier),
        );
        out.insert(
            "container_10d_profile".into(),
            Value::String("SKL2+UV01".into()),
        );
        out.insert("m64_mode".into(), Value::String("0x05:BioTissue".into()));
        out.insert("epistemic_tier".into(), Value::String(epistemic_tier));
        out.insert(
            "observation_method".into(),
            Value::String(observation_method),
        );
        out.insert("confidence".into(), Value::F64(confidence));
        Ok(Value::Record(out))
    }

    fn evaluate_mobility_fracture_risk(&self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "required_torque_nm",
                "muscle_pcsa_cm2",
                "specific_tension_n_cm2",
                "pennation_angle_rad",
                "moment_arm_m",
                "hu_muscle",
                "imat_fraction",
                "neural_activation_factor",
                "com_pos_m",
                "com_vel_m_s",
                "bos_boundary_m",
                "leg_length_m",
                "applied_bone_stress_mpa",
                "trabecular_bv_tv",
                "cortical_vbmd_mg_cm3",
                "callus_stage",
            ],
        )?;
        let required_torque_nm = number(a, "required_torque_nm", 85.0)?;
        let muscle_pcsa_cm2 = number(a, "muscle_pcsa_cm2", 45.0)?;
        let specific_tension_n_cm2 = number(a, "specific_tension_n_cm2", 40.0)?;
        let pennation_angle_rad = number(a, "pennation_angle_rad", 0.20)?;
        let moment_arm_m = number(a, "moment_arm_m", 0.055)?;
        let hu_muscle = number(a, "hu_muscle", 48.0)?;
        let imat_fraction = number(a, "imat_fraction", 0.05)?;
        let neural_activation_factor = number(a, "neural_activation_factor", 1.0)?;
        let com_pos_m = number(a, "com_pos_m", 0.02)?;
        let com_vel_m_s = number(a, "com_vel_m_s", 0.15)?;
        let bos_boundary_m = number(a, "bos_boundary_m", 0.12)?;
        let leg_length_m = number(a, "leg_length_m", 0.95)?;
        let applied_bone_stress_mpa = number(a, "applied_bone_stress_mpa", 1.8)?;
        let trabecular_bv_tv = number(a, "trabecular_bv_tv", 0.22)?;
        let cortical_vbmd_mg_cm3 = number(a, "cortical_vbmd_mg_cm3", 1150.0)?;
        let callus_stage = number(a, "callus_stage", 4.0)?.round() as i64;

        if !required_torque_nm.is_finite()
            || !muscle_pcsa_cm2.is_finite()
            || !specific_tension_n_cm2.is_finite()
            || !pennation_angle_rad.is_finite()
            || !moment_arm_m.is_finite()
            || !hu_muscle.is_finite()
            || !imat_fraction.is_finite()
            || !neural_activation_factor.is_finite()
            || !com_pos_m.is_finite()
            || !com_vel_m_s.is_finite()
            || !bos_boundary_m.is_finite()
            || !leg_length_m.is_finite()
            || !applied_bone_stress_mpa.is_finite()
            || !trabecular_bv_tv.is_finite()
            || !cortical_vbmd_mg_cm3.is_finite()
        {
            return Err("evaluate_mobility_fracture_risk: all numeric inputs must be finite".into());
        }
        if muscle_pcsa_cm2 <= 0.0
            || specific_tension_n_cm2 <= 0.0
            || moment_arm_m <= 0.0
            || leg_length_m <= 0.0
            || cortical_vbmd_mg_cm3 <= 0.0
        {
            return Err(
                "evaluate_mobility_fracture_risk: pcsa, specific_tension, moment_arm, leg_length, and cortical_vbmd must be > 0"
                    .into(),
            );
        }
        if !(0.0..std::f64::consts::FRAC_PI_2).contains(&pennation_angle_rad) {
            return Err(
                "evaluate_mobility_fracture_risk: pennation_angle_rad must be in [0, pi/2)".into(),
            );
        }
        if !(-1000.0..=3000.0).contains(&hu_muscle)
            || !(0.0..=1.0).contains(&imat_fraction)
            || !(0.0..=1.0).contains(&neural_activation_factor)
            || trabecular_bv_tv <= 0.0
            || trabecular_bv_tv > 1.0
        {
            return Err(
                "evaluate_mobility_fracture_risk: hu_muscle, imat_fraction, neural_activation_factor, or trabecular_bv_tv out of bounds"
                    .into(),
            );
        }
        let callus_factor = match callus_stage {
            0 => 0.05,
            1 => 0.20,
            2 => 0.55,
            3 => 0.85,
            4 => 1.0,
            _ => {
                return Err(
                    "evaluate_mobility_fracture_risk: callus_stage must be in 0..=4".into(),
                );
            }
        };

        let hu_quality = ((hu_muscle + 20.0) / 60.0).clamp(0.0, 1.0);
        let muscle_max_force_n = muscle_pcsa_cm2
            * specific_tension_n_cm2
            * pennation_angle_rad.cos()
            * (1.0 - imat_fraction)
            * hu_quality
            * neural_activation_factor;
        let muscle_max_torque_nm = muscle_max_force_n * moment_arm_m;
        if muscle_max_torque_nm <= 0.0 {
            return Err(
                "evaluate_mobility_fracture_risk: effective muscle max torque must be > 0".into(),
            );
        }
        let rer = required_torque_nm.abs() / muscle_max_torque_nm;

        let omega0 = (9.80665 / leg_length_m).sqrt();
        let xcom = com_pos_m + com_vel_m_s / omega0;
        let mos_m = bos_boundary_m - xcom;

        let mu_min = (cortical_vbmd_mg_cm3 / 1150.0).clamp(0.3, 1.2);
        let bone_yield_strength_mpa =
            0.3 * 140.0 * mu_min * trabecular_bv_tv.powf(1.5) * callus_factor;
        let fracture_factor_of_risk = applied_bone_stress_mpa.abs() / bone_yield_strength_mpa;

        let fall_risk_class = if mos_m >= 0.04 && rer <= 0.65 {
            "low"
        } else if mos_m >= 0.0 && rer <= 0.90 {
            "moderate"
        } else {
            "high"
        };
        let fracture_risk_class = if fracture_factor_of_risk < 0.60 {
            "low"
        } else if fracture_factor_of_risk < 1.0 {
            "moderate"
        } else {
            "high"
        };

        let mut out = std::collections::BTreeMap::new();
        out.insert("rer".into(), Value::F64(rer));
        out.insert("mos_m".into(), Value::F64(mos_m));
        out.insert(
            "fracture_factor_of_risk".into(),
            Value::F64(fracture_factor_of_risk),
        );
        out.insert("fall_risk_class".into(), Value::String(fall_risk_class.into()));
        out.insert(
            "fracture_risk_class".into(),
            Value::String(fracture_risk_class.into()),
        );
        out.insert(
            "muscle_max_force_n".into(),
            Value::F64(muscle_max_force_n),
        );
        out.insert(
            "muscle_max_torque_nm".into(),
            Value::F64(muscle_max_torque_nm),
        );
        out.insert(
            "bone_yield_strength_mpa".into(),
            Value::F64(bone_yield_strength_mpa),
        );
        Ok(Value::Record(out))
    }
}

pub(super) fn parse_fauna_archetype(s: &str) -> Result<FaunaBiomechanicalArchetype, String> {
    if let Some(arch) = FaunaBiomechanicalArchetype::from_archetype_str(s) {
        return Ok(arch);
    }
    match s.trim().to_ascii_lowercase().as_str() {
        "macropod" | "macropod_saltatorial" | "kangaroo" => {
            Ok(FaunaBiomechanicalArchetype::MacropodSaltatorial)
        }
        "quadruped" | "quadruped_fossorial" | "vombatid_fossorial" | "wombat" => {
            Ok(FaunaBiomechanicalArchetype::VombatidFossorial)
        }
        "arboreal" | "phascolarctid_arboreal" | "koala" => {
            Ok(FaunaBiomechanicalArchetype::PhascolarctidArboreal)
        }
        "avian" | "avian_raptor" | "raptor_soaring" | "eagle" => {
            Ok(FaunaBiomechanicalArchetype::RaptorSoaring)
        }
        "reptile" | "reptile_sprawling" | "squamate_sprawling" | "goanna" | "skink" => {
            Ok(FaunaBiomechanicalArchetype::SquamateSprawling)
        }
        "monotreme" | "monotreme_semi_aquatic" | "monotreme_teleost_amphibian" | "platypus"
        | "echidna" => Ok(FaunaBiomechanicalArchetype::MonotremeTeleostAmphibian),
        "marine" | "marine_pelagic" | "marine_pinniped_cetacean" | "seal" | "dolphin" => {
            Ok(FaunaBiomechanicalArchetype::MarinePinnipedCetacean)
        }
        other => Err(format!("Unknown fauna biomechanical archetype: {other}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn authored_mesh_skin_is_sealed_and_checked() {
        use qualia_core_db::render::assets::Mesh;
        use qualia_core_db::render::compile_10d::compile_mesh_to_10d;
        use qualia_core_db::container_10d::skeletal_container::read_skeletal_section;
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = AssetHost { workspace: AssetWorkspace::default(), inputs: &[], limits: &limits, cancel: &cancel, work: 0 };
        let mesh = Mesh { positions: vec![[0.,0.,0.], [1.,0.,0.], [0.,1.,0.]], triangles: vec![[0,1,2]], min: [0.,0.,0.], max: [1.,1.,0.] };
        let source = host.encode("10d", compile_mesh_to_10d(&mesh).unwrap()).unwrap();
        let influence = Value::Record(BTreeMap::from([
            ("joints".into(), Value::List(vec![Value::U64(0), Value::U64(1), Value::U64(2), Value::U64(3)])),
            ("weights".into(), Value::List(vec![Value::F64(0.25), Value::F64(0.25), Value::F64(0.25), Value::F64(0.25)])),
        ]));
        let mut args = Args::new();
        args.insert("source_10d".into(), source.clone());
        args.insert("skin".into(), Value::List(vec![influence.clone(); 3]));
        let result = host.rig_biomechanical_archetype(&args).unwrap();
        let bytes = &host.workspace.encoded(record(&result).unwrap().get("container_10d").unwrap()).unwrap().bytes;
        let section = read_skeletal_section(bytes).unwrap();
        assert_eq!(section.counts().vertices, 3);
        assert_eq!(section.skin(2).unwrap().weights, [0.25; 4]);
        use qualia_core_db::render::skeletal::{Clip, Motor, sample_pose_into, compose_palette_into, skin_vertices_into};
        let c = section.counts();
        let joints: Vec<_> = (0..c.joints).map(|i| section.joint(i).unwrap()).collect();
        let tracks: Vec<_> = (0..c.tracks).map(|i| section.track(i).unwrap()).collect();
        let keys: Vec<_> = (0..c.keys).map(|i| section.key(i).unwrap()).collect();
        let skin: Vec<_> = (0..c.vertices).map(|i| section.skin(i).unwrap()).collect();
        let clip = Clip { duration: 1.0, looping: true, tracks: &tracks, keys: &keys };
        let mut local = vec![Motor::identity(); c.joints];
        let mut global = local.clone();
        let mut palette = local.clone();
        let mut frames = Vec::new();
        for time in [0.0, 0.25, 0.5] {
            sample_pose_into(&joints, clip, time, None, 1.0, &mut local).unwrap();
            compose_palette_into(&joints, &local, Motor::identity(), &mut global, &mut palette).unwrap();
            let mut positions = vec![[0.0; 3]; 3];
            let mut normals = positions.clone();
            skin_vertices_into(&mesh.positions, &[[0.0, 0.0, 1.0]; 3], &skin, &palette, &mut positions, &mut normals).unwrap();
            frames.push(positions);
        }
        for (actual, expected) in frames[0].iter().zip(&mesh.positions) {
            for (a, b) in actual.iter().zip(expected) { assert!((a-b).abs() < 1e-4); }
        }
        assert_ne!(frames[0], frames[1]);
        assert_ne!(frames[1], frames[2]);
        args.insert("skin".into(), Value::List(vec![influence]));
        assert!(host.rig_biomechanical_archetype(&args).is_err());

        // Transfer a distinct two-joint authored rig, inverse binds and clip.
        use qualia_core_db::container_10d::skeletal_section::{Counts, SkeletalData, ClipRecord, encode_skeletal_section_into};
        use qualia_core_db::render::skeletal::{Joint, Key, Track, SkinWeight};
        let id = Motor::identity();
        let up = Motor::from_translation(0.0, 1.0, 0.0);
        let joints = [Joint { parent: None, rest_local: id, inverse_bind: id }, Joint { parent: Some(0), rest_local: up, inverse_bind: Motor::from_translation(0.0, -1.0, 0.0) }];
        let ids = [17, 18];
        let skin = [SkinWeight { joints: [0,1,0,0], weights: [0.5,0.5,0.0,0.0] }; 3];
        let clips = [ClipRecord { id: 22, duration: 1.0, first_track: 0, track_count: 2, looping: false }];
        let tracks = [Track { joint: 0, first_key: 0, key_count: 1 }, Track { joint: 1, first_key: 1, key_count: 3 }];
        let keys = [Key { time: 0.0, local: id }, Key { time: 0.0, local: up }, Key { time: 0.5, local: Motor::from_rotation_translation([0.9238795325,0.0,0.0,0.3826834324],[0.0,1.0,0.0]) }, Key { time: 1.0, local: Motor::from_rotation_translation([0.7071067812,0.0,0.0,0.7071067812],[0.0,1.0,0.0]) }];
        let counts = Counts { joints: 2, vertices: 3, clips: 1, tracks: 2, keys: 4 };
        let mut payload = vec![0; counts.encoded_len().unwrap()];
        encode_skeletal_section_into(&SkeletalData { joints: &joints, joint_ids: &ids, skin: &skin, clips: &clips, tracks: &tracks, keys: &keys }, &mut payload).unwrap();
        let base = host.workspace.encoded(&source).unwrap().bytes.clone();
        let mut carrier = vec![0; base.len()+payload.len()+4096];
        let len = qualia_core_db::container_10d::skeletal_container::attach_skeletal_section_into(&base, &payload, &mut carrier).unwrap();
        carrier.truncate(len);
        let carrier = host.encode("10d", carrier).unwrap();
        let retained = [Resource { key:"mesh.10d".into(), kind:"file".into(), bytes: base.clone() }, Resource { key:"skeleton.10d".into(), kind:"file".into(), bytes: host.workspace.encoded(&carrier).unwrap().bytes.clone() }];
        let mut retained_host = AssetHost { workspace: AssetWorkspace::default(), inputs: &retained, limits: &limits, cancel: &cancel, work: 0 };
        let retained_result = retained_host.rig_biomechanical_archetype(&BTreeMap::from([("source_10d".into(), Value::String("mesh.10d".into())), ("skeleton_10d".into(), Value::String("skeleton.10d".into()))])).unwrap();
        assert!(retained_host.workspace.encoded(record(&retained_result).unwrap().get("container_10d").unwrap()).is_ok());
        let transferred = host.rig_biomechanical_archetype(&BTreeMap::from([("source_10d".into(), source), ("skeleton_10d".into(), carrier)])).unwrap();
        let transferred = host.workspace.encoded(record(&transferred).unwrap().get("container_10d").unwrap()).unwrap();
        let section = read_skeletal_section(&transferred.bytes).unwrap();
        assert_eq!(section.counts().joints, 2);
        assert_eq!(section.joint_id(1), Some(18));
        assert_eq!(section.skin(0).unwrap().weights, [0.5,0.5,0.0,0.0]);
        assert_eq!(section.clip(0).unwrap().id, 22);
        let joints: Vec<_> = (0..2).map(|i| section.joint(i).unwrap()).collect();
        let tracks: Vec<_> = (0..2).map(|i| section.track(i).unwrap()).collect();
        let keys: Vec<_> = (0..4).map(|i| section.key(i).unwrap()).collect();
        let weights: Vec<_> = (0..3).map(|i| section.skin(i).unwrap()).collect();
        let clip = Clip { duration: 1.0, looping: false, tracks: &tracks, keys: &keys };
        let mut local = vec![id; 2]; let mut global = local.clone(); let mut palette = local.clone();
        let mut samples = Vec::new();
        for time in [0.0, 0.5, 1.0] {
            sample_pose_into(&joints, clip, time, None, 1.0, &mut local).unwrap();
            compose_palette_into(&joints, &local, id, &mut global, &mut palette).unwrap();
            let mut p = vec![[0.0;3];3]; let mut n = p.clone();
            skin_vertices_into(&mesh.positions, &[[0.0,0.0,1.0];3], &weights, &palette, &mut p, &mut n).unwrap();
            samples.push(p);
        }
        assert_ne!(samples[0], samples[1]); assert_ne!(samples[1], samples[2]);
    }

    #[test]
    fn wave12_living_world_morphisms_roundtrip() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits: &limits,
            cancel: &cancel,
            work: 0,
        };

        // 1. Botanical organ .10d (Scales 1..=4)
        for scale in 1..=4 {
            let mut a = Args::new();
            a.insert("scale".into(), Value::I64(scale));
            let res = host
                .living_world_anatomy_op("hypermedia.manifold.compile_botanical_organ_10d", &a)
                .unwrap();
            let rec = record(&res).unwrap();
            assert_eq!(rec.get("scale"), Some(&Value::I64(scale)));
            assert_eq!(rec.get("verified"), Some(&Value::Bool(true)));
        }

        // 2. Vertebrate anatomy .10d (Scale 5)
        let res = host
            .living_world_anatomy_op(
                "hypermedia.manifold.compile_vertebrate_anatomy_10d",
                &Args::new(),
            )
            .unwrap();
        let rec = record(&res).unwrap();
        assert_eq!(rec.get("submesh_count"), Some(&Value::I64(17)));
        assert_eq!(rec.get("material_count"), Some(&Value::I64(17)));

        // 3. Fauna 18-joint PGA rig + SKL1 encoding
        let res = host
            .living_world_anatomy_op(
                "hypermedia.kinetics.rig_biomechanical_archetype",
                &Args::new(),
            )
            .unwrap();
        let rec = record(&res).unwrap();
        assert_eq!(rec.get("joint_count"), Some(&Value::I64(24)));
        assert_eq!(rec.get("valid_motors"), Some(&Value::Bool(true)));

        // 4. Biological ontology pin picking
        let res = host
            .living_world_anatomy_op(
                "hypermedia.discourse.query_anatomical_ontology_pin",
                &Args::new(),
            )
            .unwrap();
        let rec = record(&res).unwrap();
        assert_eq!(rec.get("total_canonical_pins"), Some(&Value::I64(32)));

        // 5. Sector 3D continuum physics verification
        let res = host
            .living_world_anatomy_op("hypermedia.geodesy.step_sector_continuum_3d", &Args::new())
            .unwrap();
        let rec = record(&res).unwrap();
        assert_eq!(rec.get("sdf_ray_hit"), Some(&Value::Bool(true)));
        assert_eq!(rec.get("fractured_fragments"), Some(&Value::I64(2)));

        // 6. Multi-projector dome frustum + FrameLockPacket CRC-32C
        let res = host
            .living_world_anatomy_op(
                "hypermedia.layout.synchronize_companion_frustum",
                &Args::new(),
            )
            .unwrap();
        let rec = record(&res).unwrap();
        assert_eq!(rec.get("wedge_count"), Some(&Value::I64(60)));
        assert_eq!(rec.get("packet_valid_crc32c"), Some(&Value::Bool(true)));
    }

    #[test]
    fn test_calibrate_biological_subject_and_mobility_fracture_risk_dispatch() {
        assert_eq!(OPERATIONS.len(), ontology::MORPHISMS.len());

        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits: &limits,
            cancel: &cancel,
            work: 0,
        };

        // 1. Calibrate biological subject (pregnant first-trimester female)
        let mut cal_args = Args::new();
        cal_args.insert("stature_m".into(), Value::F64(1.68));
        cal_args.insert("mass_kg".into(), Value::F64(66.0));
        cal_args.insert("age_years".into(), Value::F64(31.0));
        cal_args.insert("biological_sex".into(), Value::String("female".into()));
        cal_args.insert("reproductive_phase".into(), Value::String("trimester1".into()));
        cal_args.insert("cortical_vbmd_mg_cm3".into(), Value::F64(1150.0));
        cal_args.insert("trabecular_bv_tv".into(), Value::F64(0.24));
        let cal_res = host
            .invoke(
                "hypermedia.manifold.calibrate_biological_subject",
                &Value::Record(cal_args),
            )
            .expect("calibrate_biological_subject");
        let cal_rec = record(&cal_res).unwrap();
        assert_eq!(
            cal_rec.get("container_10d_profile"),
            Some(&Value::String("SKL2+UV01".into()))
        );
        assert_eq!(
            cal_rec.get("m64_mode"),
            Some(&Value::String("0x05:BioTissue".into()))
        );
        let lig_mod = match cal_rec.get("ligament_stiffness_modifier") {
            Some(Value::F64(v)) => *v,
            other => panic!("expected f64 ligament_stiffness_modifier, got {other:?}"),
        };
        assert!(lig_mod < 0.80 && lig_mod > 0.70);

        // 2. Evaluate mobility & fracture risk (osteoporotic + myosteatotic high-risk case)
        let mut risk_args = Args::new();
        risk_args.insert("required_torque_nm".into(), Value::F64(90.0));
        risk_args.insert("muscle_pcsa_cm2".into(), Value::F64(32.0));
        risk_args.insert("hu_muscle".into(), Value::F64(15.0));
        risk_args.insert("imat_fraction".into(), Value::F64(0.25));
        risk_args.insert("com_pos_m".into(), Value::F64(0.06));
        risk_args.insert("com_vel_m_s".into(), Value::F64(0.35));
        risk_args.insert("bos_boundary_m".into(), Value::F64(0.12));
        risk_args.insert("applied_bone_stress_mpa".into(), Value::F64(3.0));
        risk_args.insert("trabecular_bv_tv".into(), Value::F64(0.12));
        risk_args.insert("callus_stage".into(), Value::I64(4));
        let risk_res = host
            .invoke(
                "hypermedia.kinetics.evaluate_mobility_fracture_risk",
                &Value::Record(risk_args),
            )
            .expect("evaluate_mobility_fracture_risk");
        let risk_rec = record(&risk_res).unwrap();
        assert_eq!(
            risk_rec.get("fall_risk_class"),
            Some(&Value::String("high".into()))
        );
        assert_eq!(
            risk_rec.get("fracture_risk_class"),
            Some(&Value::String("high".into()))
        );

        // 3. Fail-closed validation on invalid inputs
        let mut bad_args = Args::new();
        bad_args.insert("stature_m".into(), Value::F64(-1.0));
        assert!(host
            .invoke(
                "hypermedia.manifold.calibrate_biological_subject",
                &Value::Record(bad_args),
            )
            .is_err());
    }
}
