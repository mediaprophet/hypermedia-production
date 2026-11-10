//! Small-LLM Behavioral Macros, Targeted Sub-Element Effector Hooks & Flora Mycelial Neural Network
//! Model Production Plans (`behavioral_hooks.rs`).
//!
//! Designed so that when Synth defines a model (whether Fauna or Flora/Trees/Shrubs), its
//! **Production Plan** inherently includes:
//! 1. **Whole-Model Behavioral Macros**:
//!    - *Fauna*: `saltatorial_hop`, `grazing`, `chewing`, `grooming`, `alert_scan`, `foraging_sniff`, `idle`
//!    - *Flora & Mycelium Network*: `mycelial_nutrient_pulse`, `herbivory_defense_wave`,
//!      `heliotropic_canopy_track`, `wind_elastic_sway`, `anthesis_nectar_bloom`
//! 2. **Targeted Sub-Element / Organ Effector Hooks**:
//!    - *Fauna*: `head_gaze` (`head_yaw`, `head_pitch`), `jaw_mandible` (`jaw_open_01`, `jaw_chew_hz`),
//!      `ears_pinnae` (`ear_swivel_l`, `ear_swivel_r`), `eyes_eyelids` (`eye_blink_01`),
//!      `forepaws_manus` (`forepaw_reach_01`, `forepaw_grasp_01`), `hindlimbs_pes` (`hindlimb_crouch_01`),
//!      `tail_caudal` (`tail_pitch`, `tail_yaw`), `thorax_respiration` (`breath_hz`).
//!    - *Flora & Mycelium ("Wood Wide Web" Neural Network for Plants/Trees/Shrubs)*:
//!      `mycelial_hyphae_hub` (action-potential spikes & C/N/P nutrient flux),
//!      `root_rhizosphere` (exudate & mycorrhizal synapse exchange),
//!      `vascular_phloem_xylem` (turgor & sap advection),
//!      `canopy_crown_branches` (biomechanical sway & phototropism),
//!      `foliage_stomata_petioles` (stomatal aperture & volatile terpene/jasmonate defense release),
//!      `floral_operculum_buds` (anthesis & nectar phenology).

use crate::graphics::Rgba;
use crate::living_world_anatomy::turntable_render;
use crate::*;
use qualia_core_db::render::skeletal::fauna_rig::{
    FaunaSkeletalRig10d, JOINT_CHEST, JOINT_HEAD, JOINT_JAW, JOINT_LEFT_ANKLE, JOINT_LEFT_ELBOW,
    JOINT_LEFT_HIP, JOINT_LEFT_KNEE, JOINT_LEFT_SHOULDER, JOINT_LEFT_WRIST_WING, JOINT_NECK,
    JOINT_RIGHT_ANKLE, JOINT_RIGHT_HIP, JOINT_RIGHT_KNEE, JOINT_SPINE, JOINT_TAIL_BASE,
    JOINT_TAIL_MID, JOINT_TAIL_TIP,
};
use qualia_core_db::render::skeletal::Motor as PgaMotor;
use std::f32::consts::TAU;

#[inline]
fn motor_axis_angle(axis: [f32; 3], angle_rad: f32) -> PgaMotor {
    let half = 0.5 * (angle_rad as f64);
    let (s, c) = half.sin_cos();
    PgaMotor::from_rotation_translation(
        [
            c,
            (axis[0] as f64) * s,
            (axis[1] as f64) * s,
            (axis[2] as f64) * s,
        ],
        [0.0, 0.0, 0.0],
    )
}

/// High-level whole-model behavioral macro selectable by a small LLM (`0.5B–4B`) or Vibe script.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BehaviorMacro {
    /// Bipedal saltatorial hopping locomotion (`speed_m_s > 0.5`).
    SaltatorialHop,
    /// Browsing/grazing low shrubs & grasses: neck/head bowed down, rhythmic mandibular chewing,
    /// forepaws extended to hold foliage, tail counterbalanced on ground.
    GrazingBrowsing,
    /// Upright mastication / chewing: torso semi-erect, rapid jaw lateral/vertical grinding,
    /// forepaws raised near muzzle, ears relaxed.
    EatingChewing,
    /// Forepaw facial/chest grooming: seated tripod posture, forepaws brushing muzzle/chest,
    /// head tilted down.
    ForepawGrooming,
    /// Alert sentinel ear-scan: tall upright posture, independent left/right ear pinna radar
    /// swivel, head yaw scanning horizon, nostrils flaring.
    AlertEarScan,
    /// Slow quadrupedal/pentapedal foraging sniff along ground litter.
    ForagingSniff,
    /// Calm resting / breathing idle state.
    IdleBreathing,
}

impl BehaviorMacro {
    pub fn from_str_name(s: &str) -> Result<Self, String> {
        match s.trim().to_ascii_lowercase().as_str() {
            "hop" | "saltatorial_hop" | "locomotion" | "bound" => Ok(Self::SaltatorialHop),
            "grazing" | "grazing_browsing" | "browse" | "browsing" => Ok(Self::GrazingBrowsing),
            "eating" | "chewing" | "eating_chewing" | "masticate" | "mastication" => {
                Ok(Self::EatingChewing)
            }
            "grooming" | "forepaw_grooming" | "scratch" | "wash" => Ok(Self::ForepawGrooming),
            "alert" | "alert_scan" | "ear_scan" | "sentinel" | "lookout" => Ok(Self::AlertEarScan),
            "sniff" | "foraging_sniff" | "forage" => Ok(Self::ForagingSniff),
            "idle" | "idle_breathing" | "rest" | "stand" => Ok(Self::IdleBreathing),
            other => Err(format!("Unknown behavioral macro: {other}")),
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SaltatorialHop => "saltatorial_hop",
            Self::GrazingBrowsing => "grazing",
            Self::EatingChewing => "chewing",
            Self::ForepawGrooming => "grooming",
            Self::AlertEarScan => "alert_scan",
            Self::ForagingSniff => "foraging_sniff",
            Self::IdleBreathing => "idle",
        }
    }

    /// Returns the baseline `SubElementHooks` preset for this whole-model macro before any
    /// targeted sub-element overrides from the small LLM are layered on top.
    pub const fn default_hooks(self) -> SubElementHooks {
        match self {
            Self::SaltatorialHop => SubElementHooks {
                head_yaw: 0.0,
                head_pitch: -0.08,
                jaw_open_01: 0.05,
                jaw_chew_hz: 0.0,
                ear_swivel_l: 0.15,
                ear_swivel_r: -0.15,
                eye_blink_01: 0.0,
                forepaw_reach_01: 0.22,
                forepaw_grasp_01: 0.45,
                hindlimb_crouch_01: 0.55,
                tail_pitch: 0.24,
                tail_yaw: 0.0,
                breath_hz: 2.8,
            },
            Self::GrazingBrowsing => SubElementHooks {
                head_yaw: 0.12,
                head_pitch: -0.56, // Bow neck and head down to browse shrubs/sedges
                jaw_open_01: 0.48,
                jaw_chew_hz: 2.6, // Rhythmic molar/incisor chewing
                ear_swivel_l: 0.35,
                ear_swivel_r: -0.25,
                eye_blink_01: 0.15,
                forepaw_reach_01: 0.72, // Forepaws reaching out to pull/hold foliage
                forepaw_grasp_01: 0.68,
                hindlimb_crouch_01: 0.35,
                tail_pitch: 0.14,
                tail_yaw: 0.08,
                breath_hz: 1.4,
            },
            Self::EatingChewing => SubElementHooks {
                head_yaw: 0.05,
                head_pitch: -0.14,
                jaw_open_01: 0.65,
                jaw_chew_hz: 3.2, // Active lateral & vertical mastication
                ear_swivel_l: 0.20,
                ear_swivel_r: 0.20,
                eye_blink_01: 0.20,
                forepaw_reach_01: 0.58, // Forepaws raised to mouth holding gumnut/leaf
                forepaw_grasp_01: 0.80,
                hindlimb_crouch_01: 0.20,
                tail_pitch: 0.06,
                tail_yaw: 0.04,
                breath_hz: 1.5,
            },
            Self::ForepawGrooming => SubElementHooks {
                head_yaw: -0.18,
                head_pitch: -0.34,
                jaw_open_01: 0.12,
                jaw_chew_hz: 1.8,
                ear_swivel_l: -0.10,
                ear_swivel_r: 0.30,
                eye_blink_01: 0.40,
                forepaw_reach_01: 0.88, // Rapid forepaw brushing across muzzle & chest
                forepaw_grasp_01: 0.50,
                hindlimb_crouch_01: 0.42,
                tail_pitch: -0.15,
                tail_yaw: 0.05,
                breath_hz: 1.3,
            },
            Self::AlertEarScan => SubElementHooks {
                head_yaw: 0.48, // Scanning horizon
                head_pitch: 0.18,
                jaw_open_01: 0.05,
                jaw_chew_hz: 0.0,
                ear_swivel_l: 0.85, // Independent left/right ear pinna radar
                ear_swivel_r: -0.75,
                eye_blink_01: 0.02,
                forepaw_reach_01: 0.20,
                forepaw_grasp_01: 0.35,
                hindlimb_crouch_01: 0.05, // Tall upright sentinel posture
                tail_pitch: -0.08,
                tail_yaw: 0.12,
                breath_hz: 1.4,
            },
            Self::ForagingSniff => SubElementHooks {
                head_yaw: 0.28,
                head_pitch: -0.44,
                jaw_open_01: 0.12,
                jaw_chew_hz: 4.2, // Rapid nasal/muzzle twitch
                ear_swivel_l: 0.55,
                ear_swivel_r: 0.45,
                eye_blink_01: 0.08,
                forepaw_reach_01: 0.62,
                forepaw_grasp_01: 0.30,
                hindlimb_crouch_01: 0.40,
                tail_pitch: 0.18,
                tail_yaw: 0.22,
                breath_hz: 2.2,
            },
            Self::IdleBreathing => SubElementHooks {
                head_yaw: 0.0,
                head_pitch: 0.0,
                jaw_open_01: 0.02,
                jaw_chew_hz: 0.0,
                ear_swivel_l: 0.10,
                ear_swivel_r: -0.10,
                eye_blink_01: 0.10,
                forepaw_reach_01: 0.15,
                forepaw_grasp_01: 0.20,
                hindlimb_crouch_01: 0.12,
                tail_pitch: 0.04,
                tail_yaw: 0.02,
                breath_hz: 1.1,
            },
        }
    }
}

/// Targeted sub-element / organ effector hooks that a small LLM (`0.5B–4B`) can address
/// individually or layer additively on top of a whole-model `BehaviorMacro`.
#[derive(Debug, Clone, Copy)]
pub struct SubElementHooks {
    /// Head & cervical gaze yaw in radians (`[-1.2, 1.2]`) -> `JOINT_NECK` + `JOINT_HEAD`.
    pub head_yaw: f32,
    /// Head & cervical pitch in radians (`[-1.0, 0.8]`; negative bows head down to graze).
    pub head_pitch: f32,
    /// Mandibular jaw opening amplitude (`[0.0, 1.0]`) -> `JOINT_JAW`.
    pub jaw_open_01: f32,
    /// Mastication / chewing frequency in Hz (`[0.0, 6.0]`) -> drives vertical + lateral molar grind.
    pub jaw_chew_hz: f32,
    /// Left auricular pinna swivel (`[-1.0, 1.0]`) -> independent left ear radar cone.
    pub ear_swivel_l: f32,
    /// Right auricular pinna swivel (`[-1.0, 1.0]`) -> independent right ear radar cone.
    pub ear_swivel_r: f32,
    /// Eyelid blink / squint closure (`[0.0, 1.0]`) -> orbital / palpebral morph channel.
    pub eye_blink_01: f32,
    /// Forepaw / manus forward reach & elevation (`[0.0, 1.0]`) -> `JOINT_LEFT_SHOULDER` + `JOINT_LEFT_ELBOW`.
    pub forepaw_reach_01: f32,
    /// Forepaw digit grasp / curl (`[0.0, 1.0]`) -> `JOINT_LEFT_WRIST_WING`.
    pub forepaw_grasp_01: f32,
    /// Hindlimb crouch / suspension compression (`[0.0, 1.0]`) -> `JOINT_LEFT/RIGHT_HIP/KNEE/ANKLE`.
    pub hindlimb_crouch_01: f32,
    /// Caudal tail counterbalance pitch (`[-0.8, 0.8]`) -> `JOINT_TAIL_BASE..TIP`.
    pub tail_pitch: f32,
    /// Caudal tail lateral sway (`[-0.8, 0.8]`) -> `JOINT_TAIL_BASE..TIP`.
    pub tail_yaw: f32,
    /// Thoracic respiration frequency in Hz (`[0.2, 5.0]`) -> `JOINT_SPINE` + `JOINT_CHEST`.
    pub breath_hz: f32,
}

impl SubElementHooks {
    /// Parses optional sub-element hook overrides from Vibe `Args` on top of `self`.
    pub fn with_overrides(mut self, a: &Args) -> Result<Self, String> {
        if a.contains_key("head_yaw") {
            self.head_yaw = (number(a, "head_yaw", self.head_yaw as f64)? as f32).clamp(-1.4, 1.4);
        }
        if a.contains_key("head_pitch") {
            self.head_pitch =
                (number(a, "head_pitch", self.head_pitch as f64)? as f32).clamp(-1.2, 1.0);
        }
        if a.contains_key("jaw_open_01") {
            self.jaw_open_01 =
                (number(a, "jaw_open_01", self.jaw_open_01 as f64)? as f32).clamp(0.0, 1.0);
        }
        if a.contains_key("jaw_chew_hz") {
            self.jaw_chew_hz =
                (number(a, "jaw_chew_hz", self.jaw_chew_hz as f64)? as f32).clamp(0.0, 8.0);
        }
        if a.contains_key("ear_swivel_l") {
            self.ear_swivel_l =
                (number(a, "ear_swivel_l", self.ear_swivel_l as f64)? as f32).clamp(-1.2, 1.2);
        }
        if a.contains_key("ear_swivel_r") {
            self.ear_swivel_r =
                (number(a, "ear_swivel_r", self.ear_swivel_r as f64)? as f32).clamp(-1.2, 1.2);
        }
        if a.contains_key("eye_blink_01") {
            self.eye_blink_01 =
                (number(a, "eye_blink_01", self.eye_blink_01 as f64)? as f32).clamp(0.0, 1.0);
        }
        if a.contains_key("forepaw_reach_01") {
            self.forepaw_reach_01 = (number(a, "forepaw_reach_01", self.forepaw_reach_01 as f64)?
                as f32)
                .clamp(0.0, 1.0);
        }
        if a.contains_key("forepaw_grasp_01") {
            self.forepaw_grasp_01 = (number(a, "forepaw_grasp_01", self.forepaw_grasp_01 as f64)?
                as f32)
                .clamp(0.0, 1.0);
        }
        if a.contains_key("hindlimb_crouch_01") {
            self.hindlimb_crouch_01 =
                (number(a, "hindlimb_crouch_01", self.hindlimb_crouch_01 as f64)? as f32)
                    .clamp(0.0, 1.0);
        }
        if a.contains_key("tail_pitch") {
            self.tail_pitch =
                (number(a, "tail_pitch", self.tail_pitch as f64)? as f32).clamp(-1.2, 1.2);
        }
        if a.contains_key("tail_yaw") {
            self.tail_yaw = (number(a, "tail_yaw", self.tail_yaw as f64)? as f32).clamp(-1.2, 1.2);
        }
        if a.contains_key("breath_hz") {
            self.breath_hz =
                (number(a, "breath_hz", self.breath_hz as f64)? as f32).clamp(0.1, 8.0);
        }
        Ok(self)
    }

    /// Applies the macro + sub-element hooks onto an 18-joint `FaunaSkeletalRig10d` PGA motor pose.
    pub fn apply_to_rig_pose(
        &self,
        rig: &mut FaunaSkeletalRig10d,
        pose: &mut [PgaMotor; 18],
        phase_01: f32,
    ) {
        let theta = phase_01 * TAU;
        let chew_osc = (theta * self.jaw_chew_hz.max(1.0).round()).sin() * 0.5 + 0.5;

        // 1. Spine & Thorax respiration + posture pitch
        rig.apply_proc1_respiration_oscillation(pose, phase_01, 0.018 * self.breath_hz.min(3.0));
        pose[JOINT_SPINE] = pose[JOINT_SPINE]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], self.head_pitch * 0.35))
            .normalize();
        pose[JOINT_CHEST] = pose[JOINT_CHEST]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], self.head_pitch * 0.45))
            .normalize();

        // 2. Neck & Head gaze hook (yaw + pitch)
        pose[JOINT_NECK] = pose[JOINT_NECK]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], self.head_pitch * 0.65))
            .mul(&motor_axis_angle([0.0, 1.0, 0.0], self.head_yaw * 0.45))
            .normalize();
        pose[JOINT_HEAD] = pose[JOINT_HEAD]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], self.head_pitch * 0.55))
            .mul(&motor_axis_angle([0.0, 1.0, 0.0], self.head_yaw * 0.65))
            .normalize();

        // 3. Mandibular jaw chew / mastication hook (open + lateral molar grind)
        let jaw_pitch = -0.38 * self.jaw_open_01 * chew_osc;
        let jaw_grind = 0.08 * self.jaw_open_01 * (theta * 2.0).cos();
        pose[JOINT_JAW] = pose[JOINT_JAW]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], jaw_pitch))
            .mul(&motor_axis_angle([0.0, 1.0, 0.0], jaw_grind))
            .normalize();

        // 4. Forepaws / Manus reach & grasp hook
        let shoulder_pitch = -0.75 * self.forepaw_reach_01;
        let elbow_flex =
            0.85 * self.forepaw_reach_01 + 0.15 * (theta * 2.0).sin() * self.forepaw_reach_01;
        let wrist_curl = 0.65 * self.forepaw_grasp_01;
        pose[JOINT_LEFT_SHOULDER] = pose[JOINT_LEFT_SHOULDER]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], shoulder_pitch))
            .normalize();
        pose[JOINT_LEFT_ELBOW] = pose[JOINT_LEFT_ELBOW]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], elbow_flex))
            .normalize();
        pose[JOINT_LEFT_WRIST_WING] = pose[JOINT_LEFT_WRIST_WING]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], wrist_curl))
            .normalize();

        // 5. Hindlimbs crouch / IK hook
        let crouch = self.hindlimb_crouch_01;
        pose[JOINT_LEFT_HIP] = pose[JOINT_LEFT_HIP]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], -0.45 * crouch))
            .normalize();
        pose[JOINT_RIGHT_HIP] = pose[JOINT_RIGHT_HIP]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], -0.45 * crouch))
            .normalize();
        pose[JOINT_LEFT_KNEE] = pose[JOINT_LEFT_KNEE]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], 0.65 * crouch))
            .normalize();
        pose[JOINT_RIGHT_KNEE] = pose[JOINT_RIGHT_KNEE]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], 0.65 * crouch))
            .normalize();
        pose[JOINT_LEFT_ANKLE] = pose[JOINT_LEFT_ANKLE]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], -0.30 * crouch))
            .normalize();
        pose[JOINT_RIGHT_ANKLE] = pose[JOINT_RIGHT_ANKLE]
            .mul(&motor_axis_angle([1.0, 0.0, 0.0], -0.30 * crouch))
            .normalize();

        // 6. Caudal tail balance & whip hook
        let tail_sway = self.tail_yaw * theta.sin();
        for &tj in &[JOINT_TAIL_BASE, JOINT_TAIL_MID, JOINT_TAIL_TIP] {
            pose[tj] = pose[tj]
                .mul(&motor_axis_angle([1.0, 0.0, 0.0], self.tail_pitch * 0.35))
                .mul(&motor_axis_angle([0.0, 1.0, 0.0], tail_sway * 0.40))
                .normalize();
        }
    }

    /// Formats the compact ~22-token JSON command packet that a small (0.5B–4B) LLM emits.
    pub fn to_small_llm_json(&self, macro_mode: BehaviorMacro) -> String {
        format!(
            r#"{{"schema":"qualia.small-llm-behavior/v1","macro":"{}","hooks":{{"head_yaw":{:.2},"head_pitch":{:.2},"jaw_open_01":{:.2},"jaw_chew_hz":{:.1},"ear_swivel":[{:.2},{:.2}],"eye_blink_01":{:.2},"forepaw_reach_01":{:.2},"forepaw_grasp_01":{:.2},"hindlimb_crouch_01":{:.2},"tail_pitch":{:.2},"tail_yaw":{:.2},"breath_hz":{:.1}}}}}"#,
            macro_mode.as_str(),
            self.head_yaw,
            self.head_pitch,
            self.jaw_open_01,
            self.jaw_chew_hz,
            self.ear_swivel_l,
            self.ear_swivel_r,
            self.eye_blink_01,
            self.forepaw_reach_01,
            self.forepaw_grasp_01,
            self.hindlimb_crouch_01,
            self.tail_pitch,
            self.tail_yaw,
            self.breath_hz,
        )
    }
}

/// Builds the complete **Model Production Plan** (`qualia.model-production-plan/v1`) for a Fauna
/// species so that when Synth defines a model, its behavioral macro vocabulary, sub-element organ
/// hooks, and ecological trophic couplings (e.g. browsing/grazing triggering flora mycelial signals)
/// are part of the model's production specification.
pub fn build_fauna_production_plan_json(species_id: &str, title: &str) -> String {
    serde_json::json!({
        "schema": "qualia.model-production-plan/v1",
        "domain": "fauna_neuromotor",
        "species_id": species_id,
        "title": title,
        "small_llm_target": {
            "parameter_budget": "0.5B-4B",
            "control_rate_hz": 4,
            "skeletal_expansion_rate_hz": 60,
            "token_budget_per_intent": 22
        },
        "whole_model_macros": [
            { "id": "saltatorial_hop", "description": "Bipedal saltatorial hopping gait with Achilles tendon elastic recoil and caudal counterbalance" },
            { "id": "grazing", "description": "Browsing/grazing low shrubs & sedges: cervical/head bow (-0.56 rad), 2.6 Hz mandibular mastication, forepaw foliage grasp" },
            { "id": "chewing", "description": "Upright mastication: torso semi-erect, 3.2 Hz lateral/vertical molar grind, forepaws raised to muzzle" },
            { "id": "grooming", "description": "Seated tripod posture, rapid bilateral forepaw facial/chest brushing, head tilted" },
            { "id": "alert_scan", "description": "Tall sentinel posture, independent left/right auricular pinna radar swivel, horizon gaze yaw" },
            { "id": "foraging_sniff", "description": "Pentapedal slow walk, muzzle lowered to ground litter, 4.2 Hz rhinarium sniff oscillation" },
            { "id": "idle", "description": "Calm resting posture with 1.1 Hz thoracic respiration and periodic palpebral blink" }
        ],
        "sub_element_effector_hooks": {
            "head_gaze": { "joints": ["neck", "head"], "parameters": ["head_yaw", "head_pitch"], "range_rad": [-1.2, 1.2] },
            "jaw_mandible": { "joints": ["jaw"], "parameters": ["jaw_open_01", "jaw_chew_hz"], "uberon": "UBERON:0001684" },
            "ear_pinnae": { "joints": ["head"], "parameters": ["ear_swivel_l", "ear_swivel_r"], "uberon": "UBERON:0001757" },
            "eyes_eyelids": { "joints": ["head"], "parameters": ["eye_blink_01"], "uberon": "UBERON:0000970" },
            "forepaws_manus": { "joints": ["left_shoulder", "left_elbow", "left_wrist"], "parameters": ["forepaw_reach_01", "forepaw_grasp_01"], "uberon": "UBERON:0002398" },
            "hindlimbs_pes": { "joints": ["left_hip", "left_knee", "left_ankle", "right_hip", "right_knee", "right_ankle"], "parameters": ["hindlimb_crouch_01"], "uberon": "UBERON:0002387" },
            "tail_caudal": { "joints": ["tail_base", "tail_mid", "tail_tip"], "parameters": ["tail_pitch", "tail_yaw"], "uberon": "UBERON:0002415" },
            "thorax_respiration": { "joints": ["spine", "chest"], "parameters": ["breath_hz"], "uberon": "UBERON:0002228" }
        },
        "safe_rom_and_injury_thresholds": {
            "companion_q42_container": "world_of_god/wallabia_bicolor_behavior_logic.q42",
            "safe_physiological_rom": {
                "achilles_ankle_rad": [-0.85, 0.95],
                "cervical_neck_pitch_rad": [-0.75, 0.55],
                "mandibular_jaw_open_01": [0.0, 0.78],
                "forelimb_shoulder_reach_rad": [-0.90, 1.15],
                "caudal_tail_cantilever_rad": [-0.55, 0.65]
            },
            "injury_thresholds_and_reflex_behaviors": [
                {
                    "organ_or_joint": "left_ankle_achilles",
                    "safe_strain_limit_01": 0.72,
                    "injury_threshold_strain_01": 1.00,
                    "deontic_opcode": "OP_FORBID (0x12) -> OP_OBLIGATE (0x10)",
                    "triggered_behavior_macro": "injured_asymmetric_limp",
                    "sub_element_reflex": "Unloads injured hindlimb (-65% crouch weight), tucks forepaw in protective guarding, lowers stride frequency to 1.1 Hz, pins ipsilateral ear"
                },
                {
                    "organ_or_joint": "forepaws_manus",
                    "safe_strain_limit_01": 0.78,
                    "injury_threshold_strain_01": 1.00,
                    "deontic_opcode": "OP_FORBID (0x12) -> OP_OBLIGATE (0x10)",
                    "triggered_behavior_macro": "forepaw_guarding_lick",
                    "sub_element_reflex": "Retracts injured manus to sternum, bows head_pitch (-0.42 rad) to lick/groom wound"
                }
            ]
        },
        "ecological_mycelium_coupling": {
            "triggers_on_macro": ["grazing", "chewing"],
            "target_network": "flora_mycelial_neural_network",
            "signal_transduction": "herbivory_jasmonate_action_potential_wave"
        }
    })
    .to_string()
}

/// Builds the complete **Flora & Mycelial Neural Network Production Plan**
/// (`qualia.flora-mycelium-production-plan/v1` — the "Wood Wide Web" neural network for plants,
/// trees, shrubs, and rhizosphere fungi).
pub fn build_flora_mycelium_production_plan_json(
    species_id: &str,
    scale: usize,
    peel_01: f32,
    anthesis_01: f32,
) -> String {
    serde_json::json!({
        "schema": "qualia.flora-mycelium-production-plan/v1",
        "domain": "flora_mycelial_neural_network",
        "hub_species_id": species_id,
        "scale": scale,
        "peel_01": peel_01,
        "anthesis_01": anthesis_01,
        "mycelium_network_topology": {
            "architecture": "wood_wide_web_mycorrhizal_synapse_graph",
            "fungal_symbionts": ["Pisolithus_tinctorius_ectomycorrhiza", "Rhizophagus_irregularis_arbuscular"],
            "substrate_container": "world_of_god/biosphere_subsurface_strata.m64",
            "connected_flora_nodes": [
                { "node_id": "mother_tree_hub_01", "species": species_id, "role": "canopy_carbon_source_hub", "hyphae_synapses": 48, "resting_potential_mv": -64.0 },
                { "node_id": "understory_shrub_acacia_02", "species": "acacia-dealbata", "role": "nitrogen_fixer_relay", "hyphae_synapses": 24, "resting_potential_mv": -58.0 },
                { "node_id": "browsed_shrub_bursaria_03", "species": "bursaria-spinosa", "role": "herbivory_sentinel_shrub", "hyphae_synapses": 19, "resting_potential_mv": -22.0, "active_signal": "jasmonate_wound_spike" },
                { "node_id": "shaded_sapling_eucalypt_04", "species": species_id, "role": "shaded_carbon_sink_sapling", "hyphae_synapses": 14, "resting_potential_mv": -61.0 },
                { "node_id": "sedge_tussock_lomandra_05", "species": "lomandra-longifolia", "role": "rhizosphere_moisture_sensor", "hyphae_synapses": 12, "resting_potential_mv": -55.0 }
            ],
            "synaptic_edges": [
                { "from": "mother_tree_hub_01", "to": "shaded_sapling_eucalypt_04", "carbon_sucrose_flux_mg_hr": 18.4, "phosphorus_flux_ug_hr": 4.2, "hyphae_conductance_siemens": 0.86 },
                { "from": "understory_shrub_acacia_02", "to": "mother_tree_hub_01", "nitrate_ammonium_flux_mg_hr": 11.2, "hyphae_conductance_siemens": 0.79 },
                { "from": "browsed_shrub_bursaria_03", "to": "mother_tree_hub_01", "action_potential_spike_hz": 1.8, "jasmonic_acid_wave_umol": 6.4, "trigger": "wallabia_bicolor_grazing_macro" },
                { "from": "mother_tree_hub_01", "to": "sedge_tussock_lomandra_05", "hydraulic_lift_water_ml_hr": 42.0, "tannin_defense_priming_01": 0.88 }
            ]
        },
        "whole_flora_macros": [
            { "id": "mycelial_nutrient_pulse", "description": "Bioluminescent/electrochemical C-N-P resource translocation wave from canopy mother-tree hub through rhizosphere hyphae to shaded saplings and shrubs" },
            { "id": "herbivory_defense_wave", "description": "Triggered when fauna (e.g. Wallabia bicolor) grazes a connected shrub: propagates a 1.8 Hz electrical & methyl-jasmonate warning wave through the mycelium, priming leaf tannins and closing stomata" },
            { "id": "heliotropic_canopy_track", "description": "Auxin-mediated petiole and crown branch phototropic orientation toward solar irradiance vector" },
            { "id": "wind_elastic_sway", "description": "WBE pipe-model modal branch oscillation coupled to Gerstner/turbulent wind field and root-plate anchoring" },
            { "id": "anthesis_nectar_bloom", "description": "Operculum shedding, stamen filament extension, and volatile terpenoid pollinator attractor emission" }
        ],
        "flora_sub_element_hooks": {
            "mycelial_hyphae_hub": { "plant_ontology": "PO:0009005", "parameters": ["hyphae_spike_hz", "nutrient_flux_01"], "description": "Subsurface fungal mycelium action-potential & nutrient transport filaments" },
            "root_rhizosphere": { "plant_ontology": "PO:0025025", "parameters": ["exudate_pulse_01", "hydraulic_lift_01"], "description": "Root-crown & Hartig-net ectomycorrhizal exchange interface" },
            "vascular_phloem_xylem": { "plant_ontology": "PO:0005352", "parameters": ["sap_velocity_01", "turgor_pressure_mpa"], "description": "Cambium, phloem sucrose sink-source transport, and xylem transpiration stream" },
            "canopy_crown_branches": { "plant_ontology": "PO:0009006", "parameters": ["branch_sway_rad", "phototropic_tilt_rad"], "description": "Primary and secondary WBE pipe-model structural branches" },
            "foliage_stomata_petioles": { "plant_ontology": "PO:0002000", "parameters": ["stomatal_aperture_01", "tannin_defense_01"], "description": "Leaf lamina, petiole angle, stomatal guard cells, and volatile defense glands" },
            "floral_operculum_buds": { "plant_ontology": "PO:0009046", "parameters": ["anthesis_open_01", "nectar_secretion_01"], "description": "Eucalypt gumnut hypanthium, operculum cap, and stamen whorls" }
        },
        "safe_deflection_and_injury_thresholds": {
            "companion_q42_container": "world_of_god/eucalyptus_mycelium_logic.q42",
            "safe_elastic_envelope": {
                "stem_bending_deflection_rad": [0.0, 0.38],
                "modulus_of_rupture_mpa": 65.0,
                "safe_foliar_browse_fraction_01": [0.0, 0.22],
                "rhizosphere_water_potential_mpa": [-1.8, -0.1]
            },
            "injury_thresholds_and_mycelial_behaviors": [
                {
                    "organ": "foliage_and_cambium",
                    "injury_trigger": "herbivory_browse_fraction_01 > 0.25 OR cambium_bark_strip_01 > 0.15",
                    "deontic_opcode": "OP_FORBID (0x12) -> OP_OBLIGATE (0x10)",
                    "triggered_flora_macro": "herbivory_defense_wave",
                    "response": "Fires 1.8 Hz electrical action-potential + methyl-jasmonate wave into mycelial_hyphae_hub to warn neighboring trees/shrubs; closes stomata (0.08); exudes kino resin at wound"
                },
                {
                    "organ": "canopy_crown_branches",
                    "injury_trigger": "bending_stress_mpa > 65.0 (deflection_rad > 0.38)",
                    "deontic_opcode": "OP_FORBID (0x12) -> OP_OBLIGATE (0x10)",
                    "triggered_flora_macro": "branch_cleavage_compartmentalization",
                    "response": "CODIT xylem vessel plugging (tyloses) to prevent cavitation/embolism + mycelial carbon dump to neighbor saplings"
                }
            ]
        }
    })
    .to_string()
}

/// Deterministic FNV-1a 60-bit hash (`lexicon.rs` mask `0x0FFF_FFFF_FFFF_FFFF`).
#[inline]
pub(super) fn fnv1a_60(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h & 0x0FFF_FFFF_FFFF_FFFF
}

/// Packs a single 48-byte Super-Quin (`NQuin`: `subject`, `predicate`, `object`, `context`,
/// `metadata`, `parity` = XOR fold) into `out`.
pub(super) fn push_nquin_48(
    out: &mut Vec<u8>,
    subject_did: &str,
    property_iri: &str,
    opcode: u8,
    defeater_msb: bool,
    decimal_val: f64,
    context_did: &str,
    routing_lane: u64,
    lamport_clock: u64,
    weight_u32: u32,
) {
    let subject = (1u64 << 63) | fnv1a_60(subject_did.as_bytes());
    let mut predicate = (fnv1a_60(property_iri.as_bytes()) << 8) | (opcode as u64);
    if defeater_msb {
        predicate |= 1u64 << 63;
    }
    // INLINE_TAG_DECIMAL = 0b010 << 60, value * 10^6 in bits [0..59]
    let scaled = ((decimal_val.abs() * 1_000_000.0).round() as u64) & 0x0FFF_FFFF_FFFF_FFFF;
    let object = (0b010u64 << 60) | scaled;
    let context = fnv1a_60(context_did.as_bytes()) & 0x00FF_FFFF_FFFF_FFFF;
    let metadata = ((routing_lane & 0b11) << 61)
        | ((lamport_clock & 0x1FFF_FFFF) << 32)
        | (weight_u32 as u64);
    let parity = subject ^ predicate ^ object ^ context;

    out.extend_from_slice(&subject.to_le_bytes());
    out.extend_from_slice(&predicate.to_le_bytes());
    out.extend_from_slice(&object.to_le_bytes());
    out.extend_from_slice(&context.to_le_bytes());
    out.extend_from_slice(&metadata.to_le_bytes());
    out.extend_from_slice(&parity.to_le_bytes());
}

/// Compiles a deterministic `.q42` (`Q42LOGIC` 64-byte header + 16 parity-verified 48-byte `NQuin`
/// records = 832 bytes) container holding a model's:
/// - **Safe Physiological Range-of-Motion / Elastic Deflection Envelopes (`OP_PERMIT = 0x11`)**
/// - **Injury / Sprain / Yield / Cleavage Thresholds (`OP_FORBID = 0x12` + `DEFEATER_BIT`)**
/// - **Post-Injury Behavioral Reflexes & Mycelial Network Transduction (`OP_OBLIGATE = 0x10`)**
/// - **60Hz Temporal Safety Invariants (`OP_LTL_GLOBALLY = 0x40`)**
pub fn compile_model_logic_q42_bytes(model_id: &str, is_flora_mycelium: bool) -> Vec<u8> {
    let quin_count: u32 = 16;
    let mut quins = Vec::with_capacity((quin_count as usize) * 48);
    let ctx_did = if is_flora_mycelium {
        "did:q42:world-of-god:flora-mycelium-neural-network"
    } else {
        "did:q42:world-of-god:fauna-neuromotor-envelope"
    };
    let subj_did = format!("did:q42:model:{model_id}");

    // 1..=4: OP_PERMIT (0x11) — Safe Physiological Range-of-Motion / Elastic Deflection Envelopes
    let safe_rules: [(&str, f64); 4] = if is_flora_mycelium {
        [
            ("q42:safeStemElasticDeflectionRad", 0.38),
            ("q42:safeModulusOfRuptureMPa", 65.0),
            ("q42:safeFoliarBrowseFraction01", 0.22),
            ("q42:safeXylemWaterPotentialMPa", 1.80),
        ]
    } else {
        [
            ("q42:safeAnkleAchillesFlexionRad", 0.95),
            ("q42:safeCervicalNeckPitchRad", 0.75),
            ("q42:safeMandibularJawOpen01", 0.78),
            ("q42:safeForelimbReachRad", 1.15),
        ]
    };
    for (idx, (prop, limit)) in safe_rules.iter().enumerate() {
        push_nquin_48(
            &mut quins,
            &subj_did,
            prop,
            0x11, // OP_PERMIT
            false,
            *limit,
            ctx_did,
            0b11, // Spatial routing lane
            (idx as u64) + 1,
            1000,
        );
    }

    // 5..=8: OP_FORBID (0x12) with DEFEATER_BIT — Injury / Yield / Cleavage Thresholds
    let injury_rules: [(&str, f64); 4] = if is_flora_mycelium {
        [
            ("q42:injuryThresholdHerbivoryBrowse01", 0.25),
            ("q42:injuryThresholdCambiumBarkStrip01", 0.15),
            ("q42:injuryThresholdBranchCleavageMPa", 65.0),
            ("q42:injuryThresholdXylemCavitationMPa", 2.40),
        ]
    } else {
        [
            ("q42:injuryThresholdAchillesStrain01", 1.00),
            ("q42:injuryThresholdForepawSprain01", 1.00),
            ("q42:injuryThresholdCervicalHyperext01", 1.00),
            ("q42:injuryThresholdCaudalTorque01", 1.00),
        ]
    };
    for (idx, (prop, thresh)) in injury_rules.iter().enumerate() {
        push_nquin_48(
            &mut quins,
            &subj_did,
            prop,
            0x12, // OP_FORBID
            true, // DEFEATER_BIT overrides normal locomotion/sway when threshold exceeded
            *thresh,
            ctx_did,
            0b11,
            (idx as u64) + 5,
            2000,
        );
    }

    // 9..=12: OP_OBLIGATE (0x10) — Post-Injury Behavioral Transitions & Mycelial Transduction
    let obligate_rules: [(&str, f64); 4] = if is_flora_mycelium {
        [
            ("q42:obligateMycelialJasmonateSpikeHz", 1.80),
            ("q42:obligateStomatalEmergencyClosure01", 0.08),
            ("q42:obligateCambiumKinoResinCallus01", 0.95),
            ("q42:obligateMycorrhizalCarbonDumpMgHr", 18.4),
        ]
    } else {
        [
            ("q42:obligateInjuredAsymmetricLimpGait", 0.35),
            ("q42:obligateForepawProtectiveGuarding", 0.90),
            ("q42:obligateNociceptiveEarPinBack", 0.85),
            ("q42:obligateHerbivoryToMyceliumCoupling", 1.00),
        ]
    };
    for (idx, (prop, val)) in obligate_rules.iter().enumerate() {
        push_nquin_48(
            &mut quins,
            &subj_did,
            prop,
            0x10, // OP_OBLIGATE
            false,
            *val,
            ctx_did,
            0b01, // Commons / Ecological lane
            (idx as u64) + 9,
            1500,
        );
    }

    // 13..=16: OP_LTL_GLOBALLY (0x40) — 60Hz Temporal Trace Safety & Small-LLM Hook Guards
    let ltl_rules: [(&str, f64); 4] = [
        ("q42:ltlGloballyEnforceJointConeLimits", 60.0),
        ("q42:ltlGloballyClampSmallLlmHooksToSafeRom", 4.0),
        ("q42:ltlUntilWoundHealedMaintainGuarding", 1.0),
        ("q42:ltlGloballyVerifyNQuinXorParity", 1.0),
    ];
    for (idx, (prop, val)) in ltl_rules.iter().enumerate() {
        push_nquin_48(
            &mut quins,
            &subj_did,
            prop,
            0x40, // OP_LTL_GLOBALLY
            false,
            *val,
            ctx_did,
            0b11,
            (idx as u64) + 13,
            4096,
        );
    }

    // Pack 64-byte Q42LOGIC Header + 16 * 48B NQuin payload
    let mut out = Vec::with_capacity(64 + quins.len());
    out.extend_from_slice(b"Q42LOGIC"); // [0..8] Magic
    out.extend_from_slice(&1u32.to_le_bytes()); // [8..12] Version = 1
    out.extend_from_slice(&quin_count.to_le_bytes()); // [12..16] Total 48B NQuin records (16)
    let domain_code: u32 = if is_flora_mycelium { 2 } else { 1 };
    out.extend_from_slice(&domain_code.to_le_bytes()); // [16..20] 1 = Fauna Neuromotor, 2 = Flora Mycelium
    out.extend_from_slice(&4u32.to_le_bytes()); // [20..24] Safe RoM Permit Quins (OP_PERMIT=0x11)
    out.extend_from_slice(&4u32.to_le_bytes()); // [24..28] Injury Threshold Quins (OP_FORBID=0x12)
    out.extend_from_slice(&4u32.to_le_bytes()); // [28..32] Post-Injury/Mycelium Obligate Quins (OP_OBLIGATE=0x10)
    out.extend_from_slice(&4u32.to_le_bytes()); // [32..36] LTL Temporal Guard Quins (OP_LTL_GLOBALLY=0x40)
    let model_hash = (1u64 << 63) | fnv1a_60(subj_did.as_bytes());
    out.extend_from_slice(&model_hash.to_le_bytes()); // [36..44] Model topological DID pointer
    // Compute FNV-1a / CRC digest over payload
    let payload_digest = fnv1a_60(&quins);
    out.extend_from_slice(&payload_digest.to_le_bytes()); // [44..52] Payload digest
    out.extend_from_slice(&[0u8; 12]); // [52..64] Reserved 64B header alignment
    out.extend_from_slice(&quins);
    out
}

/// Renders an N-frame animated 3D cinema sequence driven by a `BehaviorMacro` and `SubElementHooks`.
/// Articulates individual anatomical regions of the mesh (head/neck bow, mandibular jaw chewing,
/// bilateral ear pinna radar swivel, forepaw reach/grasp, hindlimb crouch/kick, and caudal tail balance)
/// so that both whole-model macros (`grazing`, `chewing`, `grooming`, `alert_scan`, `saltatorial_hop`)
/// and targeted sub-element hooks produce vivid, verifiable motion in `.mp4` cinema streams.
pub fn render_macro_behavior_cinema_frames(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    colors: &[[f32; 4]],
    indices: &[u32],
    macro_mode: BehaviorMacro,
    hooks: &SubElementHooks,
    frame_count: usize,
) -> Vec<Rgba> {
    let w = 160usize;
    let h = 112usize;
    let count = frame_count.clamp(4, 24);
    let mut frames = Vec::with_capacity(count);

    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut min_z = f32::INFINITY;
    let mut max_z = f32::NEG_INFINITY;
    for p in positions {
        min_y = min_y.min(p[1]);
        max_y = max_y.max(p[1]);
        min_z = min_z.min(p[2]);
        max_z = max_z.max(p[2]);
    }
    let span_y = (max_y - min_y).max(0.05);
    let span_z = (max_z - min_z).max(0.05);

    for f in 0..count {
        let phase_01 = (f as f32) / (count as f32);
        let theta = phase_01 * TAU;
        let yaw = -0.68 + 0.36 * theta.sin();
        let chew_osc = (theta * hooks.jaw_chew_hz.max(2.0).round()).sin();

        let mut anim_pos = Vec::with_capacity(positions.len());
        for &p in positions {
            let mut x = p[0];
            let mut y = p[1];
            let mut z = p[2];

            // Identify anatomical sub-element zones in canonical model space:
            // Head & Muzzle: z > 0.16, y > 0.50
            // Mandible / Jaw: z > 0.19, 0.51 <= y <= 0.562
            // Ear Pinnae: y > 0.595, z > 0.15
            // Forepaws / Manus: z > 0.08, 0.27 <= y <= 0.46
            // Caudal Tail: z < -0.14
            if z > 0.14 && y > 0.46 {
                // Cervical + Cranial bow/nod (`head_pitch`, `head_yaw`)
                let neck_t = ((z - 0.14) / 0.19).clamp(0.0, 1.0);
                let pitch_drop =
                    hooks.head_pitch * (0.85 + 0.15 * theta.sin()) * 0.22 * neck_t;
                y += pitch_drop;
                z += (-pitch_drop) * 0.45;
                x += hooks.head_yaw * theta.cos() * 0.04 * neck_t;

                // Mandibular chewing (`jaw_open_01`, `jaw_chew_hz`)
                if z > 0.19 && y < 0.565 + pitch_drop {
                    let jaw_t = ((z - 0.19) / 0.13).clamp(0.0, 1.0);
                    let open_drop =
                        hooks.jaw_open_01 * (0.5 + 0.5 * chew_osc) * 0.032 * jaw_t;
                    y -= open_drop;
                    x += hooks.jaw_open_01 * (theta * 2.0).cos() * 0.008 * jaw_t;
                }

                // Bilateral Auricular Ear Pinna Swivel (`ear_swivel_l`, `ear_swivel_r`)
                if p[1] > 0.595 {
                    let ear_t = ((p[1] - 0.595) / 0.14).clamp(0.0, 1.0);
                    let swivel = if p[0] >= 0.0 {
                        hooks.ear_swivel_l * theta.sin()
                    } else {
                        hooks.ear_swivel_r * theta.cos()
                    };
                    z += swivel * 0.028 * ear_t;
                    x += swivel * 0.016 * ear_t;
                }
            } else if z > 0.07 && (0.26..=0.46).contains(&p[1]) {
                // Forepaw / Manus Reach & Grooming (`forepaw_reach_01`, `forepaw_grasp_01`)
                let paw_t = ((0.46 - p[1]) / 0.18).clamp(0.0, 1.0);
                let reach_wave = 0.5 + 0.5 * (theta * 2.0).sin();
                z += hooks.forepaw_reach_01 * reach_wave * 0.045 * paw_t;
                y += hooks.forepaw_reach_01 * 0.025 * paw_t;
            } else if z < -0.14 {
                // Caudal Tail Balance & Sway (`tail_pitch`, `tail_yaw`)
                let tail_t = ((-0.14 - z) / 0.58).clamp(0.0, 1.0);
                y += hooks.tail_pitch * 0.06 * tail_t
                    + 0.018 * (theta - tail_t * 2.5).sin() * tail_t;
                x += hooks.tail_yaw * theta.sin() * 0.045 * tail_t;
            }

            if macro_mode == BehaviorMacro::SaltatorialHop {
                y += theta.sin().abs() * span_y * 0.06;
            } else {
                // Thoracic breathing micro-motion
                let _ = span_z;
                y += 0.004 * (theta * hooks.breath_hz.max(1.0).round()).sin();
            }

            anim_pos.push([x, y, z]);
        }

        let buf = turntable_render::render_cinema_rgba_frame_at(
            &anim_pos, normals, colors, indices, w, h, yaw, 0.18, phase_01, false,
        );
        frames.push(buf);
    }
    frames
}
