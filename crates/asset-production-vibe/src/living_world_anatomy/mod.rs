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

mod botanical_vertebrate_10d;
mod rig_ontology_continuum_frustum;

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
        for wi in 2..=5 {
            let _soil = couple_rigid_body_with_soil(
                &mut bodies[wi],
                4_500.0,
                0.04,
                0.45,
                &soil_profile,
                dt,
            );
        }
        let _hydro = couple_rigid_body_with_hydro(
            &mut bodies[1],
            &gerstner_spectrum,
            (step_idx as f32) * dt,
            dt,
        );

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

            _ => Err(format!("Unknown living-world anatomy operation: {id}")),
        }
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
        assert_eq!(rec.get("joint_count"), Some(&Value::I64(18)));
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
}
