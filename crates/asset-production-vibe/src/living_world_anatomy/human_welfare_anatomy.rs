//! Human Welfare & Maternal-Fetal Multi-System `.10d` + `.q42` Anatomy Compiler (`human_welfare_anatomy.rs`).
//!
//! Synthesizes complete, self-contained 17-system human anatomical digital twins (`CompiledVertebrateAnatomy10d`),
//! clinical/anthropometric welfare measurement manifests, and `.q42` (`Q42LOGIC` 48-byte `NQuin`)
//! maternal-fetal & biomechanical welfare rule containers across 5 canonical biological profiles:
//! 1. `male` — Adult Male (XY) Reference (`178.0 cm`, `76.0 kg`)
//! 2. `female` — Adult Female (XX) Non-Pregnant Baseline (`165.0 cm`, `62.0 kg`)
//! 3. `female_trimester_1` — Pregnancy Trimester 1 (`Weeks 1–12`, canonical `Week 10`)
//! 4. `female_trimester_2` — Pregnancy Trimester 2 (`Weeks 13–26`, canonical `Week 22`)
//! 5. `female_trimester_3` — Pregnancy Trimester 3 (`Weeks 27–40`, canonical `Week 38` Cephalic Vertex LOA)
//!
//! Designed for the QualiaDB Welfare & Clinical Care Application and Synth / Hypermedia-Production Studio.

use crate::living_world_anatomy::turntable_render;
use crate::*;

const OP_OBLIGATE: u8 = 0x10;
const OP_PERMIT: u8 = 0x11;
const OP_FORBID: u8 = 0x12;
const OP_LTL_GLOBALLY: u8 = 0x40;
const DEFEATER_BIT: u64 = 1u64 << 63;
const INLINE_TAG_INTEGER: u64 = 0b001u64 << 60;
const ROUTING_BILATERAL: u64 = 0b10u64 << 61;
const SENSITIVITY_PUBLIC: u8 = 0;
use qualia_core_db::container_10d::integrity::verify_whole_file_crc32c;
use qualia_core_db::container_10d::SubmeshRange;
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::skeletal::fauna_rig::*;
use qualia_core_db::render::vertebrate_anatomy::{
    finalize_and_seal_vertebrate_anatomy_10d, make_pbr_material, AnatomicalLoftRing,
    AnatomyMeshAccumulator, CompiledVertebrateAnatomy10d, ZoologicalAnatomicalLayer,
};
use qualia_core_db::{q_hash, NQuin};
use std::collections::BTreeMap;
use std::f32::consts::PI;
use vibe::Value;

/// Canonical human biological & gestational profile for the Welfare App.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanAnatomyProfile {
    Male,
    FemaleNonPregnant,
    FemaleTrimester1,
    FemaleTrimester2,
    FemaleTrimester3,
}

impl HumanAnatomyProfile {
    pub fn from_str_and_week(profile: &str, week_opt: Option<u32>) -> Result<(Self, u32), String> {
        let norm = profile.trim().to_ascii_lowercase().replace('-', "_");
        match norm.as_str() {
            "male" | "vhm" | "homo_sapiens_male" | "xy" => Ok((Self::Male, 0)),
            "female" | "vhf" | "homo_sapiens_female" | "xx" | "non_pregnant" => {
                if let Some(w) = week_opt {
                    if w >= 27 {
                        return Ok((Self::FemaleTrimester3, w.min(40)));
                    } else if w >= 13 {
                        return Ok((Self::FemaleTrimester2, w));
                    } else if w >= 1 {
                        return Ok((Self::FemaleTrimester1, w));
                    }
                }
                Ok((Self::FemaleNonPregnant, 0))
            }
            "female_trimester_1" | "trimester_1" | "t1" | "pregnant_t1" | "pregnancy_t1" => {
                Ok((Self::FemaleTrimester1, week_opt.unwrap_or(10).clamp(1, 12)))
            }
            "female_trimester_2" | "trimester_2" | "t2" | "pregnant_t2" | "pregnancy_t2" => {
                Ok((Self::FemaleTrimester2, week_opt.unwrap_or(22).clamp(13, 26)))
            }
            "female_trimester_3" | "trimester_3" | "t3" | "pregnant_t3" | "pregnancy_t3" | "full_term" => {
                Ok((Self::FemaleTrimester3, week_opt.unwrap_or(38).clamp(27, 40)))
            }
            other => Err(format!(
                "Unknown human anatomy profile '{other}' (expected male, female, female_trimester_1, female_trimester_2, female_trimester_3)"
            )),
        }
    }

    pub const fn slug(self) -> &'static str {
        match self {
            Self::Male => "male",
            Self::FemaleNonPregnant => "female",
            Self::FemaleTrimester1 => "female_trimester_1",
            Self::FemaleTrimester2 => "female_trimester_2",
            Self::FemaleTrimester3 => "female_trimester_3",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Male => "Adult Male (XY · Reference)",
            Self::FemaleNonPregnant => "Adult Female (XX · Non-Pregnant Baseline)",
            Self::FemaleTrimester1 => "Female · Pregnancy Trimester 1 (Wks 1–12 · Early Gestation)",
            Self::FemaleTrimester2 => "Female · Pregnancy Trimester 2 (Wks 13–26 · Mid-Gestation)",
            Self::FemaleTrimester3 => "Female · Pregnancy Trimester 3 (Wks 27–40 · Full-Term Cephalic)",
        }
    }
}

/// Anthropometric, physiological, obstetric, and welfare measurements for a human profile.
#[derive(Debug, Clone)]
pub struct HumanWelfareMeasurements {
    pub profile_slug: &'static str,
    pub profile_label: &'static str,
    pub gestational_week: u32,
    pub stature_cm: f32,
    pub body_mass_kg: f32,
    pub bmi_kg_m2: f32,
    pub biacromial_shoulder_cm: f32,
    pub biiliac_pelvic_cm: f32,
    pub pelvic_inlet_transverse_mm: f32,
    pub subpubic_angle_deg: f32,
    pub chest_circumference_cm: f32,
    pub waist_abdominal_circumference_cm: f32,
    pub hip_circumference_cm: f32,
    pub lumbar_lordosis_deg: f32,
    pub com_anterior_shift_mm: f32,
    pub resting_heart_rate_bpm: f32,
    pub cardiac_output_l_min: f32,
    pub blood_volume_expansion_pct: f32,
    pub uterine_blood_flow_ml_min: f32,
    pub diaphragmatic_elevation_cm: f32,
    pub symphysis_fundal_height_cm: f32,
    pub estimated_fetal_weight_g: f32,
    pub crown_rump_length_mm: f32,
    pub biparietal_diameter_mm: f32,
    pub femur_length_mm: f32,
    pub amniotic_fluid_index_cm: f32,
    pub placental_mass_g: f32,
    pub fetal_heart_rate_bpm: f32,
    pub max_recommended_lift_kg: f32,
    pub max_lumbar_flexion_safe_deg: f32,
}

impl HumanWelfareMeasurements {
    pub fn compute(profile: HumanAnatomyProfile, gestational_week: u32) -> Self {
        match profile {
            HumanAnatomyProfile::Male => Self {
                profile_slug: profile.slug(),
                profile_label: profile.label(),
                gestational_week: 0,
                stature_cm: 178.0,
                body_mass_kg: 76.0,
                bmi_kg_m2: 24.0,
                biacromial_shoulder_cm: 41.5,
                biiliac_pelvic_cm: 28.2,
                pelvic_inlet_transverse_mm: 118.0,
                subpubic_angle_deg: 64.0,
                chest_circumference_cm: 99.0,
                waist_abdominal_circumference_cm: 83.0,
                hip_circumference_cm: 96.5,
                lumbar_lordosis_deg: 41.0,
                com_anterior_shift_mm: 0.0,
                resting_heart_rate_bpm: 68.0,
                cardiac_output_l_min: 5.2,
                blood_volume_expansion_pct: 0.0,
                uterine_blood_flow_ml_min: 0.0,
                diaphragmatic_elevation_cm: 0.0,
                symphysis_fundal_height_cm: 0.0,
                estimated_fetal_weight_g: 0.0,
                crown_rump_length_mm: 0.0,
                biparietal_diameter_mm: 0.0,
                femur_length_mm: 0.0,
                amniotic_fluid_index_cm: 0.0,
                placental_mass_g: 0.0,
                fetal_heart_rate_bpm: 0.0,
                max_recommended_lift_kg: 23.0,
                max_lumbar_flexion_safe_deg: 55.0,
            },
            _ => {
                let w = gestational_week.clamp(0, 40) as f32;
                let g_frac = (w / 40.0).clamp(0.0, 1.0);
                let mass_gain = 12.5 * g_frac.powf(1.45);
                let body_mass_kg = 62.0 + mass_gain;
                let stature_cm = 165.0;
                let bmi = body_mass_kg / ((stature_cm / 100.0) * (stature_cm / 100.0));
                let fundal_cm = if w < 4.0 {
                    7.5
                } else if w <= 12.0 {
                    7.5 + (w - 4.0) * 0.55
                } else {
                    w * 0.96
                };
                let efw_g = if w < 6.0 {
                    0.0
                } else {
                    (3400.0 * ((w - 5.0) / 35.0).clamp(0.0, 1.0).powf(3.05)).round()
                };
                let crl_mm = if w < 4.0 {
                    0.0
                } else {
                    (360.0 * ((w - 3.0) / 37.0).clamp(0.0, 1.0).powf(1.15)).round()
                };
                let bpd_mm = if w < 8.0 {
                    0.0
                } else {
                    (95.0 * ((w - 7.0) / 33.0).clamp(0.0, 1.0).powf(0.88)).round()
                };
                let fl_mm = if w < 10.0 {
                    0.0
                } else {
                    (76.0 * ((w - 9.0) / 31.0).clamp(0.0, 1.0).powf(0.94)).round()
                };
                let afi_cm = if w < 6.0 {
                    0.0
                } else {
                    14.5 - 2.5 * ((w - 28.0).max(0.0) / 12.0)
                };
                let placental_g = if w < 6.0 {
                    0.0
                } else {
                    (640.0 * g_frac.powf(1.25)).round()
                };
                let fhr_bpm = if w < 6.0 {
                    0.0
                } else {
                    162.0 - 20.0 * g_frac
                };
                let lift_kg = (18.0 * (1.0 - 0.62 * g_frac)).max(6.0);
                let flex_deg = 52.0 - 22.0 * g_frac;

                Self {
                    profile_slug: profile.slug(),
                    profile_label: profile.label(),
                    gestational_week,
                    stature_cm,
                    body_mass_kg: (body_mass_kg * 10.0).round() / 10.0,
                    bmi_kg_m2: (bmi * 10.0).round() / 10.0,
                    biacromial_shoulder_cm: 36.8,
                    biiliac_pelvic_cm: 29.4 + 0.6 * g_frac,
                    pelvic_inlet_transverse_mm: 132.0 + 4.5 * g_frac,
                    subpubic_angle_deg: 98.0 + 4.0 * g_frac,
                    chest_circumference_cm: 88.5 + 5.5 * g_frac,
                    waist_abdominal_circumference_cm: 71.0 + 34.0 * g_frac.powf(1.3),
                    hip_circumference_cm: 97.5 + 4.5 * g_frac,
                    lumbar_lordosis_deg: 43.0 + 14.0 * g_frac,
                    com_anterior_shift_mm: (32.0 * g_frac.powf(1.35)).round(),
                    resting_heart_rate_bpm: (72.0 + 15.0 * g_frac).round(),
                    cardiac_output_l_min: ((4.8 + 2.3 * g_frac.sqrt()) * 10.0).round() / 10.0,
                    blood_volume_expansion_pct: (48.0 * g_frac.powf(0.85)).round(),
                    uterine_blood_flow_ml_min: (50.0 + 650.0 * g_frac.powf(1.3)).round(),
                    diaphragmatic_elevation_cm: ((4.0 * g_frac.powf(1.4)) * 10.0).round() / 10.0,
                    symphysis_fundal_height_cm: (fundal_cm * 10.0).round() / 10.0,
                    estimated_fetal_weight_g: efw_g,
                    crown_rump_length_mm: crl_mm,
                    biparietal_diameter_mm: bpd_mm,
                    femur_length_mm: fl_mm,
                    amniotic_fluid_index_cm: (afi_cm * 10.0).round() / 10.0,
                    placental_mass_g: placental_g,
                    fetal_heart_rate_bpm: fhr_bpm.round(),
                    max_recommended_lift_kg: (lift_kg * 10.0).round() / 10.0,
                    max_lumbar_flexion_safe_deg: flex_deg.round(),
                }
            }
        }
    }

    pub fn to_json_string(&self) -> String {
        format!(
            concat!(
                "{{\n",
                "  \"schema\": \"qualia.welfare.human_anatomy_measurements.v1\",\n",
                "  \"profile\": \"{}\",\n",
                "  \"label\": \"{}\",\n",
                "  \"gestational_week\": {},\n",
                "  \"anthropometrics\": {{\n",
                "    \"stature_cm\": {:.1},\n",
                "    \"body_mass_kg\": {:.1},\n",
                "    \"bmi_kg_m2\": {:.1},\n",
                "    \"biacromial_shoulder_cm\": {:.1},\n",
                "    \"biiliac_pelvic_cm\": {:.1},\n",
                "    \"pelvic_inlet_transverse_mm\": {:.1},\n",
                "    \"subpubic_angle_deg\": {:.1},\n",
                "    \"chest_circumference_cm\": {:.1},\n",
                "    \"waist_abdominal_circumference_cm\": {:.1},\n",
                "    \"hip_circumference_cm\": {:.1},\n",
                "    \"lumbar_lordosis_deg\": {:.1},\n",
                "    \"com_anterior_shift_mm\": {:.1}\n",
                "  }},\n",
                "  \"maternal_cardiorespiratory\": {{\n",
                "    \"resting_heart_rate_bpm\": {:.0},\n",
                "    \"cardiac_output_l_min\": {:.1},\n",
                "    \"blood_volume_expansion_pct\": {:.0},\n",
                "    \"uterine_blood_flow_ml_min\": {:.0},\n",
                "    \"diaphragmatic_elevation_cm\": {:.1}\n",
                "  }},\n",
                "  \"obstetric_fetal_biometrics\": {{\n",
                "    \"symphysis_fundal_height_cm\": {:.1},\n",
                "    \"estimated_fetal_weight_g\": {:.0},\n",
                "    \"crown_rump_length_mm\": {:.0},\n",
                "    \"biparietal_diameter_mm\": {:.0},\n",
                "    \"femur_length_mm\": {:.0},\n",
                "    \"amniotic_fluid_index_cm\": {:.1},\n",
                "    \"placental_mass_g\": {:.0},\n",
                "    \"fetal_heart_rate_bpm\": {:.0}\n",
                "  }},\n",
                "  \"welfare_thresholds_q42\": {{\n",
                "    \"max_recommended_lift_kg\": {:.1},\n",
                "    \"max_lumbar_flexion_safe_deg\": {:.0},\n",
                "    \"systems_covered\": 17,\n",
                "    \"small_llm_hooks\": [\"respiration_cycle\", \"maternal_fetal_cardiac\", \"postural_lordosis_balance\", \"fetal_movement_kick\", \"pelvic_rom_guard\"]\n",
                "  }}\n",
                "}}"
            ),
            self.profile_slug,
            self.profile_label,
            self.gestational_week,
            self.stature_cm,
            self.body_mass_kg,
            self.bmi_kg_m2,
            self.biacromial_shoulder_cm,
            self.biiliac_pelvic_cm,
            self.pelvic_inlet_transverse_mm,
            self.subpubic_angle_deg,
            self.chest_circumference_cm,
            self.waist_abdominal_circumference_cm,
            self.hip_circumference_cm,
            self.lumbar_lordosis_deg,
            self.com_anterior_shift_mm,
            self.resting_heart_rate_bpm,
            self.cardiac_output_l_min,
            self.blood_volume_expansion_pct,
            self.uterine_blood_flow_ml_min,
            self.diaphragmatic_elevation_cm,
            self.symphysis_fundal_height_cm,
            self.estimated_fetal_weight_g,
            self.crown_rump_length_mm,
            self.biparietal_diameter_mm,
            self.femur_length_mm,
            self.amniotic_fluid_index_cm,
            self.placental_mass_g,
            self.fetal_heart_rate_bpm,
            self.max_recommended_lift_kg,
            self.max_lumbar_flexion_safe_deg,
        )
    }
}

/// Builds a complete 3-layer / 17-system Human Welfare & Maternal-Fetal `.10d` container
/// (`Osteology` = Skeletal & Fetal Skeleton, `MyologyAndTendons` = Brain/Nervous, Heart/Vessels,
/// Lungs/Diaphragm, Viscera, Uteroplacental-Fetal Complex & Striated Musculature,
/// `ExternalIntegument` = Dimorphic & Gestational Skin Envelope).
pub fn build_human_welfare_anatomy_10d(
    profile: HumanAnatomyProfile,
    gestational_week: u32,
    osteology_vis_01: f32,
    myology_vis_01: f32,
    integument_vis_01: f32,
    explode_layer_01: f32,
) -> CompiledVertebrateAnatomy10d {
    let is_male = profile == HumanAnatomyProfile::Male;
    let w = if is_male {
        0.0
    } else {
        (gestational_week.clamp(0, 40) as f32) / 40.0
    };

    // Anthropometric scaling factors (Y in [-0.88, +0.88] m centered around hip/pelvis y = 0.0)
    let h_scale = if is_male { 1.04_f32 } else { 0.97_f32 };
    let shoulder_x = if is_male { 0.215_f32 } else { 0.184_f32 };
    let pelvis_x = if is_male {
        0.138_f32
    } else {
        0.156_f32 + 0.008 * w
    };
    let waist_x = if is_male {
        0.152_f32
    } else {
        0.132_f32 + 0.075 * w.powf(1.25)
    };
    // Anterior gravid abdominal protrusion (+Z is anterior)
    let belly_protrusion_z = if is_male {
        0.0_f32
    } else {
        0.185_f32 * w.powf(1.35)
    };
    let lordosis_z = 0.022_f32 + 0.024_f32 * w;

    let mut acc = AnatomyMeshAccumulator::new();
    let mut materials = Vec::new();
    let mut submesh_ranges = Vec::new();

    let bone_col = [0.93, 0.90, 0.83, 1.0];
    let cart_col = [0.80, 0.85, 0.88, 1.0];
    let muscle_col = [0.72, 0.22, 0.21, 1.0];
    let tendon_col = [0.89, 0.86, 0.80, 1.0];
    let heart_col = [0.78, 0.14, 0.18, 1.0];
    let artery_col = [0.86, 0.16, 0.20, 1.0];
    let vein_col = [0.22, 0.38, 0.76, 1.0];
    let lung_col = [0.86, 0.54, 0.58, 1.0];
    let liver_col = [0.52, 0.20, 0.18, 1.0];
    let gut_col = [0.82, 0.56, 0.48, 1.0];
    let brain_col = [0.90, 0.72, 0.74, 1.0];
    let nerve_col = [0.95, 0.84, 0.32, 1.0];
    let uterus_col = [0.82, 0.34, 0.48, 1.0];
    let placenta_col = [0.64, 0.16, 0.26, 1.0];
    let fetus_col = [0.94, 0.76, 0.68, 1.0];
    let skin_col = if is_male {
        [0.82, 0.65, 0.54, 1.0]
    } else {
        [0.86, 0.69, 0.58, 1.0]
    };

    // Helper to record a named semantic submesh
    let mut push_submesh = |acc: &AnatomyMeshAccumulator,
                            start_idx: u32,
                            id: &str,
                            iri: &str,
                            col: [f32; 4],
                            rough: f32| {
        let count = (acc.indices.len() as u32).saturating_sub(start_idx);
        if count > 0 {
            let mat = make_pbr_material(id, col, rough, 0.0);
            materials.push(mat);
            submesh_ranges.push(SubmeshRange {
                first_index: start_idx,
                index_count: count,
                material_id: mat.id,
                semantic_id: q_hash(iri),
            });
        }
    };

    // ========================================================================
    // LAYER 0: OSTEOLOGY (Cranium, Spine, Ribcage, Dimorphic Pelvis, Limbs & Fetal Skeleton)
    // ========================================================================
    let ost_start_v = acc.positions.len();
    let layer_ost = ZoologicalAnatomicalLayer::Osteology;

    // 1. Cranium & Mandible (`UBERON:0003129`)
    let s0 = acc.indices.len() as u32;
    let head_y = 0.68 * h_scale;
    acc.push_ellipsoid(
        [0.0, head_y + 0.04, -0.01],
        [0.078, 0.098, 0.094],
        14,
        12,
        bone_col,
        layer_ost,
        JOINT_HEAD,
        JOINT_NECK,
        0.95,
    );
    // Facial maxilla/zygomatic + articulated mandible
    acc.push_ellipsoid(
        [0.0, head_y - 0.025, 0.038],
        [0.056, 0.048, 0.052],
        10,
        8,
        bone_col,
        layer_ost,
        JOINT_JAW,
        JOINT_HEAD,
        0.95,
    );
    push_submesh(
        &acc,
        s0,
        "ost_cranium_mandible",
        "http://purl.obolibrary.org/obo/UBERON_0003129",
        bone_col,
        0.42,
    );

    // 2. Vertebral Column (C1-C7, T1-T12, L1-L5 with gestational lordosis, Sacrum) & Ribcage
    let s1 = acc.indices.len() as u32;
    for v_idx in 0..24 {
        let frac = v_idx as f32 / 23.0;
        let vy = (0.04 + frac * 0.56) * h_scale;
        // S-curve: lumbar lordosis (frac < 0.35) + thoracic kyphosis (frac 0.35..0.78) + cervical lordosis
        let vz = if frac < 0.35 {
            -0.045 + lordosis_z * ((frac / 0.35) * PI).sin()
        } else if frac < 0.78 {
            -0.045 - 0.024 * (((frac - 0.35) / 0.43) * PI).sin()
        } else {
            -0.035 + 0.012 * (((frac - 0.78) / 0.22) * PI).sin()
        };
        let vr = 0.024 * (1.0 - 0.38 * frac);
        acc.push_ellipsoid(
            [0.0, vy, vz],
            [vr * 1.15, 0.012, vr],
            8,
            6,
            bone_col,
            layer_ost,
            JOINT_SPINE,
            JOINT_CHEST,
            0.9,
        );
    }
    // Sternum
    acc.push_cylinder(
        [0.0, 0.48 * h_scale, 0.088],
        [0.0, 0.29 * h_scale, 0.096 + 0.012 * w],
        0.022,
        0.016,
        8,
        4,
        bone_col,
        layer_ost,
        JOINT_CHEST,
        JOINT_SPINE,
        0.9,
    );
    // 10 Paired Costal Rib Arches (flaring in late gestation)
    let rib_flare = 1.0 + 0.08 * w;
    for r_idx in 0..10 {
        let rf = r_idx as f32 / 9.0;
        let ry_back = (0.50 - rf * 0.24) * h_scale;
        let ry_front = (0.47 - rf * 0.23) * h_scale;
        let span = (0.085 + 0.058 * (rf * PI).sin()) * (if is_male { 1.08 } else { 0.98 }) * rib_flare;
        let z_front = 0.082 + 0.024 * (rf * PI).sin() + 0.014 * w * rf;
        for side in [-1.0_f32, 1.0_f32] {
            acc.push_curved_rib_arch(
                [side * 0.018, ry_back, -0.055],
                [side * 0.022, ry_front, z_front],
                side * span,
                0.0065,
                bone_col,
                cart_col,
                layer_ost,
                JOINT_CHEST,
                JOINT_SPINE,
            );
        }
    }
    push_submesh(
        &acc,
        s1,
        "ost_spine_ribcage",
        "http://purl.obolibrary.org/obo/UBERON_0001130",
        bone_col,
        0.42,
    );

    // 3. Dimorphic Pelvic Girdle (Iliac Crests, Sacrum, Ischiopubic Rami & Relaxin Symphysis)
    let s2 = acc.indices.len() as u32;
    // Sacrum wedge
    acc.push_cylinder(
        [0.0, 0.05 * h_scale, -0.052],
        [0.0, -0.04 * h_scale, -0.068],
        0.044,
        0.018,
        8,
        3,
        bone_col,
        layer_ost,
        JOINT_PELVIS_ROOT,
        JOINT_SPINE,
        0.95,
    );
    let symphysis_gap = if is_male { 0.004 } else { 0.006 + 0.006 * w };
    for side in [-1.0_f32, 1.0_f32] {
        // Iliac wing arch
        acc.push_curved_rib_arch(
            [side * 0.038, 0.035 * h_scale, -0.052],
            [side * symphysis_gap, -0.045 * h_scale, 0.068],
            side * pelvis_x,
            0.016,
            bone_col,
            cart_col,
            layer_ost,
            JOINT_PELVIS_ROOT,
            JOINT_SPINE,
        );
    }
    push_submesh(
        &acc,
        s2,
        "ost_pelvic_girdle",
        "http://purl.obolibrary.org/obo/UBERON_0001270",
        bone_col,
        0.42,
    );

    // 4. Appendicular Skeleton (Clavicles, Humeri, Radii/Ulnae, Femora, Tibiae/Fibulae)
    let s3 = acc.indices.len() as u32;
    for side in [-1.0_f32, 1.0_f32] {
        // Clavicle
        acc.push_long_bone_with_epiphyses(
            [side * 0.024, 0.50 * h_scale, 0.072],
            [side * shoulder_x, 0.51 * h_scale, 0.015],
            0.008,
            0.013,
            [0.0, 0.005, 0.012],
            bone_col,
            layer_ost,
            JOINT_CHEST,
            JOINT_LEFT_SHOULDER,
        );
        // Humerus
        acc.push_long_bone_with_epiphyses(
            [side * shoulder_x, 0.50 * h_scale, 0.010],
            [side * (shoulder_x + 0.035), 0.22 * h_scale, 0.020],
            0.014,
            0.022,
            [side * 0.006, 0.0, -0.005],
            bone_col,
            layer_ost,
            JOINT_LEFT_SHOULDER,
            JOINT_LEFT_ELBOW,
        );
        // Radius & Ulna + Hand
        acc.push_long_bone_with_epiphyses(
            [side * (shoulder_x + 0.035), 0.22 * h_scale, 0.020],
            [side * (shoulder_x + 0.055), -0.02 * h_scale, 0.045],
            0.011,
            0.017,
            [side * 0.004, 0.0, 0.008],
            bone_col,
            layer_ost,
            JOINT_LEFT_ELBOW,
            JOINT_LEFT_WRIST_WING,
        );
        // Femur (with female Q-angle valgus slant)
        let hip_j = if side < 0.0 { JOINT_LEFT_HIP } else { JOINT_RIGHT_HIP };
        let knee_j = if side < 0.0 { JOINT_LEFT_KNEE } else { JOINT_RIGHT_KNEE };
        let ankle_j = if side < 0.0 { JOINT_LEFT_ANKLE } else { JOINT_RIGHT_ANKLE };
        let hip_x = side * (pelvis_x * 0.74);
        let knee_x = side * 0.088;
        acc.push_long_bone_with_epiphyses(
            [hip_x, -0.04 * h_scale, 0.0],
            [knee_x, -0.44 * h_scale, 0.012],
            0.019,
            0.030,
            [side * 0.008, 0.0, 0.014],
            bone_col,
            layer_ost,
            hip_j,
            knee_j,
        );
        // Tibia & Fibula + Foot
        acc.push_long_bone_with_epiphyses(
            [knee_x, -0.44 * h_scale, 0.012],
            [knee_x, -0.82 * h_scale, 0.0],
            0.015,
            0.024,
            [0.0, 0.0, -0.006],
            bone_col,
            layer_ost,
            knee_j,
            ankle_j,
        );
        acc.push_cylinder(
            [knee_x, -0.83 * h_scale, -0.025],
            [knee_x, -0.86 * h_scale, 0.115],
            0.024,
            0.028,
            8,
            3,
            bone_col,
            layer_ost,
            ankle_j,
            ankle_j,
            0.95,
        );
    }
    push_submesh(
        &acc,
        s3,
        "ost_appendicular_limbs",
        "http://purl.obolibrary.org/obo/UBERON_0002091",
        bone_col,
        0.42,
    );

    // 5. Fetal Skeleton (Trimesters 2 & 3 — Cephalic Presentation inside Gravid Uterus)
    if w >= 0.30 {
        let sf = acc.indices.len() as u32;
        let f_scale = ((w - 0.15) / 0.85).clamp(0.25, 1.0);
        let u_center_y = (0.04 + 0.14 * w) * h_scale;
        let u_center_z = 0.045 + belly_protrusion_z * 0.58;
        // Cephalic vertex head positioned down near pelvic inlet in T3, higher in T2
        let f_head_y = u_center_y - 0.075 * f_scale;
        let f_head_z = u_center_z + 0.015 * f_scale;
        acc.push_ellipsoid(
            [0.0, f_head_y, f_head_z],
            [0.036 * f_scale, 0.042 * f_scale, 0.040 * f_scale],
            10,
            8,
            [0.96, 0.92, 0.86, 1.0],
            layer_ost,
            JOINT_SPINE,
            JOINT_PELVIS_ROOT,
            0.9,
        );
        // Flexed C-shaped fetal vertebral column & ribcage
        for fv in 0..8 {
            let ft = fv as f32 / 7.0;
            let fy = f_head_y + (0.035 + ft * 0.135) * f_scale;
            let fz = f_head_z + (0.038 * (ft * PI).sin()) * f_scale;
            acc.push_ellipsoid(
                [-0.012 * f_scale, fy, fz],
                [0.014 * f_scale, 0.010 * f_scale, 0.014 * f_scale],
                6,
                5,
                [0.96, 0.92, 0.86, 1.0],
                layer_ost,
                JOINT_SPINE,
                JOINT_PELVIS_ROOT,
                0.9,
            );
        }
        push_submesh(
            &acc,
            sf,
            "ost_fetal_skeleton",
            "http://purl.obolibrary.org/obo/UBERON_0001443",
            [0.96, 0.92, 0.86, 1.0],
            0.40,
        );
    }

    let ost_end_v = acc.positions.len();

    // ========================================================================
    // LAYER 1: MYOLOGY, VISCERA, CARDIORESPIRATORY, NERVOUS & UTEROPLACENTAL-FETAL
    // ========================================================================
    let myo_start_v = acc.positions.len();
    let layer_myo = ZoologicalAnatomicalLayer::MyologyAndTendons;

    // 1. Brain (Cerebrum, Cerebellum) & Spinal Cord (`UBERON:0000955`)
    let m0 = acc.indices.len() as u32;
    acc.push_ellipsoid(
        [-0.028, head_y + 0.048, -0.008],
        [0.042, 0.068, 0.076],
        10,
        8,
        brain_col,
        layer_myo,
        JOINT_HEAD,
        JOINT_NECK,
        0.95,
    );
    acc.push_ellipsoid(
        [0.028, head_y + 0.048, -0.008],
        [0.042, 0.068, 0.076],
        10,
        8,
        brain_col,
        layer_myo,
        JOINT_HEAD,
        JOINT_NECK,
        0.95,
    );
    acc.push_cylinder(
        [0.0, head_y - 0.02, -0.032],
        [0.0, 0.05 * h_scale, -0.048],
        0.009,
        0.006,
        8,
        8,
        nerve_col,
        layer_myo,
        JOINT_SPINE,
        JOINT_HEAD,
        0.9,
    );
    push_submesh(
        &acc,
        m0,
        "neuro_brain_spinal_cord",
        "http://purl.obolibrary.org/obo/UBERON_0000955",
        brain_col,
        0.48,
    );

    // 2. Cardiovascular System: Heart Myocardium, Aorta & Vena Cava (`UBERON:0000948`)
    let m1 = acc.indices.len() as u32;
    let diaph_shift = 0.032 * w;
    acc.push_ellipsoid(
        [0.024, (0.37 + diaph_shift * 0.5) * h_scale, 0.038],
        [0.044, 0.054, 0.042],
        12,
        10,
        heart_col,
        layer_myo,
        JOINT_CHEST,
        JOINT_SPINE,
        0.95,
    );
    // Descending Aorta & Inferior Vena Cava
    acc.push_cylinder(
        [0.008, 0.43 * h_scale, 0.012],
        [0.006, 0.04 * h_scale, -0.018],
        0.013,
        0.010,
        8,
        6,
        artery_col,
        layer_myo,
        JOINT_CHEST,
        JOINT_SPINE,
        0.9,
    );
    acc.push_cylinder(
        [-0.016, 0.42 * h_scale, 0.015],
        [-0.014, 0.04 * h_scale, -0.015],
        0.014,
        0.012,
        8,
        6,
        vein_col,
        layer_myo,
        JOINT_CHEST,
        JOINT_SPINE,
        0.9,
    );
    push_submesh(
        &acc,
        m1,
        "cardio_heart_great_vessels",
        "http://purl.obolibrary.org/obo/UBERON_0000948",
        heart_col,
        0.38,
    );

    // 3. Respiratory System: Left & Right Lungs + Thoracic Diaphragm (`UBERON:0002048`)
    let m2 = acc.indices.len() as u32;
    for side in [-1.0_f32, 1.0_f32] {
        acc.push_ellipsoid(
            [side * 0.068, (0.39 + diaph_shift * 0.6) * h_scale, 0.014],
            [0.054, 0.096 - 0.012 * w, 0.064],
            12,
            10,
            lung_col,
            layer_myo,
            JOINT_CHEST,
            JOINT_SPINE,
            0.9,
        );
    }
    push_submesh(
        &acc,
        m2,
        "resp_lungs_bronchi",
        "http://purl.obolibrary.org/obo/UBERON_0002048",
        lung_col,
        0.52,
    );

    // 4. Hepatobiliary, Gastrointestinal & Renal Systems (displaced superiorly in Pregnancy T2/T3)
    let m3 = acc.indices.len() as u32;
    let viscera_lift = 0.068 * w.powf(1.2);
    // Liver (right upper quadrant)
    acc.push_ellipsoid(
        [-0.052, (0.25 + viscera_lift * 0.55) * h_scale, 0.028],
        [0.072, 0.052, 0.058],
        10,
        8,
        liver_col,
        layer_myo,
        JOINT_SPINE,
        JOINT_CHEST,
        0.9,
    );
    // Stomach (left upper quadrant)
    acc.push_ellipsoid(
        [0.048, (0.24 + viscera_lift * 0.65) * h_scale, 0.032],
        [0.048, 0.042, 0.044],
        10,
        8,
        gut_col,
        layer_myo,
        JOINT_SPINE,
        JOINT_CHEST,
        0.9,
    );
    // Small & Large Intestine coil (pushed upward & laterally around gravid uterus)
    acc.push_ellipsoid(
        [0.0, (0.13 + viscera_lift) * h_scale, 0.035 - 0.015 * w],
        [0.088 + 0.018 * w, 0.068, 0.052],
        12,
        10,
        gut_col,
        layer_myo,
        JOINT_SPINE,
        JOINT_PELVIS_ROOT,
        0.9,
    );
    // Left & Right Kidneys (retroperitoneal)
    for side in [-1.0_f32, 1.0_f32] {
        acc.push_ellipsoid(
            [side * 0.048, 0.18 * h_scale, -0.024],
            [0.024, 0.042, 0.022],
            8,
            8,
            [0.62, 0.24, 0.26, 1.0],
            layer_myo,
            JOINT_SPINE,
            JOINT_PELVIS_ROOT,
            0.9,
        );
    }
    push_submesh(
        &acc,
        m3,
        "viscera_digestive_renal",
        "http://purl.obolibrary.org/obo/UBERON_0001007",
        gut_col,
        0.46,
    );

    // 5. Reproductive & Uteroplacental-Fetal System (`UBERON:0000995` / `UBERON:0001987`)
    let m4 = acc.indices.len() as u32;
    if is_male {
        acc.push_ellipsoid(
            [0.0, -0.045 * h_scale, 0.032],
            [0.022, 0.020, 0.022],
            8,
            8,
            uterus_col,
            layer_myo,
            JOINT_PELVIS_ROOT,
            JOINT_PELVIS_ROOT,
            0.95,
        );
    } else {
        // Gravid or non-gravid Uterus + Ovaries + Fallopian Tubes
        let ut_rx = 0.028 + 0.096 * w.powf(1.15);
        let ut_ry = 0.038 + 0.138 * w.powf(1.15);
        let ut_rz = 0.022 + 0.105 * w.powf(1.20);
        let ut_y = (-0.015 + 0.135 * w) * h_scale;
        let ut_z = 0.032 + belly_protrusion_z * 0.52;

        // Uterine Myometrium Envelope
        acc.push_ellipsoid(
            [0.0, ut_y, ut_z],
            [ut_rx, ut_ry, ut_rz],
            14,
            12,
            uterus_col,
            layer_myo,
            JOINT_SPINE,
            JOINT_PELVIS_ROOT,
            0.92,
        );
        // Paired Ovaries
        for side in [-1.0_f32, 1.0_f32] {
            acc.push_ellipsoid(
                [side * (ut_rx + 0.022), ut_y - ut_ry * 0.25, ut_z - 0.012],
                [0.016, 0.012, 0.012],
                8,
                6,
                [0.92, 0.76, 0.58, 1.0],
                layer_myo,
                JOINT_PELVIS_ROOT,
                JOINT_SPINE,
                0.9,
            );
        }

        // If pregnant (Trimesters 1, 2, 3): Discoid Placenta, Umbilical Cord & Developing Fetus
        if w > 0.05 {
            // Discoid Placenta on upper-posterior uterine fundus
            acc.push_ellipsoid(
                [0.015, ut_y + ut_ry * 0.55, ut_z - ut_rz * 0.28],
                [ut_rx * 0.62, ut_ry * 0.26, ut_rz * 0.42],
                10,
                8,
                placenta_col,
                layer_myo,
                JOINT_SPINE,
                JOINT_PELVIS_ROOT,
                0.9,
            );
            // Fetal Body & Umbilical Cord
            let f_rx = ut_rx * 0.58;
            let f_ry = ut_ry * 0.64;
            let f_rz = ut_rz * 0.58;
            acc.push_ellipsoid(
                [0.0, ut_y - ut_ry * 0.08, ut_z + 0.008],
                [f_rx, f_ry, f_rz],
                12,
                10,
                fetus_col,
                layer_myo,
                JOINT_SPINE,
                JOINT_PELVIS_ROOT,
                0.9,
            );
            // Umbilical cord connecting placenta to fetal abdomen
            acc.push_cylinder(
                [0.015, ut_y + ut_ry * 0.45, ut_z - ut_rz * 0.15],
                [0.0, ut_y, ut_z + 0.012],
                0.009 * (0.4 + 0.6 * w),
                0.008 * (0.4 + 0.6 * w),
                8,
                4,
                vein_col,
                layer_myo,
                JOINT_SPINE,
                JOINT_PELVIS_ROOT,
                0.9,
            );
        }
    }
    push_submesh(
        &acc,
        m4,
        "repro_uteroplacental_fetal",
        "http://purl.obolibrary.org/obo/UBERON_0000995",
        uterus_col,
        0.44,
    );

    // 6. Striated Skeletal Musculature (`UBERON:0001630`)
    let m5 = acc.indices.len() as u32;
    for side in [-1.0_f32, 1.0_f32] {
        // Pectoralis Major
        acc.push_fusiform_muscle(
            [side * 0.018, 0.42 * h_scale, 0.088],
            [side * (shoulder_x - 0.01), 0.46 * h_scale, 0.025],
            0.048,
            0.024,
            [side * 0.015, -0.01, 0.028],
            muscle_col,
            tendon_col,
            layer_myo,
            JOINT_CHEST,
            JOINT_LEFT_SHOULDER,
        );
        // Rectus Abdominis (arching over gravid abdomen in pregnancy)
        acc.push_fusiform_muscle(
            [side * (0.032 + 0.024 * w), 0.28 * h_scale, 0.082],
            [side * (0.024 + 0.016 * w), -0.03 * h_scale, 0.068],
            0.032,
            0.016,
            [side * 0.018 * w, 0.0, belly_protrusion_z * 0.88],
            muscle_col,
            tendon_col,
            layer_myo,
            JOINT_SPINE,
            JOINT_PELVIS_ROOT,
        );
        // Deltoid & Biceps/Triceps
        acc.push_fusiform_muscle(
            [side * shoulder_x, 0.50 * h_scale, 0.012],
            [side * (shoulder_x + 0.035), 0.23 * h_scale, 0.022],
            0.034,
            0.032,
            [side * 0.012, 0.0, 0.008],
            muscle_col,
            tendon_col,
            layer_myo,
            JOINT_LEFT_SHOULDER,
            JOINT_LEFT_ELBOW,
        );
        // Gluteus Maximus & Quadriceps Femoris + Gastrocnemius
        let hip_j = if side < 0.0 { JOINT_LEFT_HIP } else { JOINT_RIGHT_HIP };
        let knee_j = if side < 0.0 { JOINT_LEFT_KNEE } else { JOINT_RIGHT_KNEE };
        let ankle_j = if side < 0.0 { JOINT_LEFT_ANKLE } else { JOINT_RIGHT_ANKLE };
        let hip_x = side * (pelvis_x * 0.74);
        let knee_x = side * 0.088;
        acc.push_fusiform_muscle(
            [hip_x, -0.02 * h_scale, 0.012],
            [knee_x, -0.42 * h_scale, 0.022],
            0.056,
            0.058,
            [side * 0.012, 0.0, 0.024],
            muscle_col,
            tendon_col,
            layer_myo,
            hip_j,
            knee_j,
        );
        acc.push_fusiform_muscle(
            [knee_x, -0.44 * h_scale, -0.008],
            [knee_x, -0.80 * h_scale, -0.012],
            0.038,
            0.042,
            [0.0, 0.03, -0.022],
            muscle_col,
            tendon_col,
            layer_myo,
            knee_j,
            ankle_j,
        );
    }
    push_submesh(
        &acc,
        m5,
        "myo_striated_musculature",
        "http://purl.obolibrary.org/obo/UBERON_0001630",
        muscle_col,
        0.54,
    );

    let myo_end_v = acc.positions.len();

    // ========================================================================
    // LAYER 2: EXTERNAL INTEGUMENT (Dimorphic & Gestational Skin Envelope)
    // ========================================================================
    let int_start_v = acc.positions.len();
    let layer_int = ZoologicalAnatomicalLayer::ExternalIntegument;

    let i0 = acc.indices.len() as u32;
    let torso_stations = [
        // (y_unscaled, z_offset, rx, rz, joint)
        (-0.08, 0.005, pelvis_x * 1.16, 0.118, JOINT_PELVIS_ROOT),
        (
            0.02,
            belly_protrusion_z * 0.45,
            pelvis_x * 1.18 + 0.035 * w,
            0.122 + belly_protrusion_z * 0.48,
            JOINT_PELVIS_ROOT,
        ),
        (
            0.13,
            belly_protrusion_z * 0.62,
            waist_x * 1.08,
            0.112 + belly_protrusion_z * 0.62,
            JOINT_SPINE,
        ),
        (
            0.25,
            belly_protrusion_z * 0.38,
            waist_x * 1.05,
            0.114 + belly_protrusion_z * 0.38,
            JOINT_SPINE,
        ),
        (
            0.39,
            0.016,
            shoulder_x * 0.88,
            if is_male { 0.124 } else { 0.132 + 0.018 * w },
            JOINT_CHEST,
        ),
        (0.51, 0.004, shoulder_x * 1.04, 0.106, JOINT_CHEST),
        (0.57, -0.004, 0.062, 0.060, JOINT_NECK),
        (0.64, 0.002, 0.058, 0.058, JOINT_NECK),
        (0.73, 0.012, 0.084, 0.098, JOINT_HEAD),
        (0.82, 0.000, 0.072, 0.086, JOINT_HEAD),
    ];
    let rings: Vec<AnatomicalLoftRing> = torso_stations
        .iter()
        .map(|&(y, z, rx, rz, j)| AnatomicalLoftRing {
            center: [0.0, y * h_scale, z],
            radii: [rx, rz],
            dorsal_color: skin_col,
            flank_color: skin_col,
            ventral_color: skin_col,
            primary_joint: j,
            secondary_joint: j,
            primary_weight: 0.9,
        })
        .collect();
    acc.push_countershaded_loft(&rings, 18, layer_int, true);

    // Mammary Glands on Female profiles (with gestational hypertrophy in Trimesters 1–3)
    if !is_male {
        let mam_r = 0.046 + 0.014 * w;
        for side in [-1.0_f32, 1.0_f32] {
            acc.push_ellipsoid(
                [side * 0.078, 0.38 * h_scale, 0.108 + 0.014 * w],
                [mam_r, mam_r * 0.92, mam_r * 0.95],
                10,
                8,
                skin_col,
                layer_int,
                JOINT_CHEST,
                JOINT_SPINE,
                0.92,
            );
        }
    }

    // Upper & Lower Limb Integument Envelopes
    for side in [-1.0_f32, 1.0_f32] {
        let arm_rings = [
            AnatomicalLoftRing {
                center: [side * shoulder_x, 0.50 * h_scale, 0.010],
                radii: [0.048, 0.048],
                dorsal_color: skin_col,
                flank_color: skin_col,
                ventral_color: skin_col,
                primary_joint: JOINT_LEFT_SHOULDER,
                secondary_joint: JOINT_CHEST,
                primary_weight: 0.9,
            },
            AnatomicalLoftRing {
                center: [side * (shoulder_x + 0.035), 0.22 * h_scale, 0.020],
                radii: [0.038, 0.038],
                dorsal_color: skin_col,
                flank_color: skin_col,
                ventral_color: skin_col,
                primary_joint: JOINT_LEFT_ELBOW,
                secondary_joint: JOINT_LEFT_SHOULDER,
                primary_weight: 0.9,
            },
            AnatomicalLoftRing {
                center: [side * (shoulder_x + 0.055), -0.02 * h_scale, 0.045],
                radii: [0.028, 0.024],
                dorsal_color: skin_col,
                flank_color: skin_col,
                ventral_color: skin_col,
                primary_joint: JOINT_LEFT_WRIST_WING,
                secondary_joint: JOINT_LEFT_ELBOW,
                primary_weight: 0.9,
            },
        ];
        acc.push_countershaded_loft(&arm_rings, 12, layer_int, true);

        let hip_j = if side < 0.0 { JOINT_LEFT_HIP } else { JOINT_RIGHT_HIP };
        let knee_j = if side < 0.0 { JOINT_LEFT_KNEE } else { JOINT_RIGHT_KNEE };
        let ankle_j = if side < 0.0 { JOINT_LEFT_ANKLE } else { JOINT_RIGHT_ANKLE };
        let hip_x = side * (pelvis_x * 0.74);
        let knee_x = side * 0.088;
        let leg_rings = [
            AnatomicalLoftRing {
                center: [hip_x, -0.04 * h_scale, 0.0],
                radii: [0.078, 0.082],
                dorsal_color: skin_col,
                flank_color: skin_col,
                ventral_color: skin_col,
                primary_joint: hip_j,
                secondary_joint: JOINT_PELVIS_ROOT,
                primary_weight: 0.9,
            },
            AnatomicalLoftRing {
                center: [knee_x, -0.44 * h_scale, 0.012],
                radii: [0.052, 0.054],
                dorsal_color: skin_col,
                flank_color: skin_col,
                ventral_color: skin_col,
                primary_joint: knee_j,
                secondary_joint: hip_j,
                primary_weight: 0.9,
            },
            AnatomicalLoftRing {
                center: [knee_x, -0.82 * h_scale, 0.0],
                radii: [0.036, 0.038],
                dorsal_color: skin_col,
                flank_color: skin_col,
                ventral_color: skin_col,
                primary_joint: ankle_j,
                secondary_joint: knee_j,
                primary_weight: 0.9,
            },
        ];
        acc.push_countershaded_loft(&leg_rings, 14, layer_int, true);
    }
    push_submesh(
        &acc,
        i0,
        "integument_skin_envelope",
        "http://purl.obolibrary.org/obo/UBERON_0002097",
        skin_col,
        0.68,
    );

    let int_end_v = acc.positions.len();

    finalize_and_seal_vertebrate_anatomy_10d(
        FaunaBiomechanicalArchetype::MacropodSaltatorial,
        acc,
        materials,
        submesh_ranges,
        ost_start_v..ost_end_v,
        myo_start_v..myo_end_v,
        int_start_v..int_end_v,
        osteology_vis_01,
        myology_vis_01,
        integument_vis_01,
        explode_layer_01,
    )
}

/// Compiles a deterministic 832-byte `.q42` (`Q42LOGIC`) welfare & maternal-fetal rule container
/// with 16 parity-verified 48-byte `NQuin` records encoding Safe Postural RoM, Lifting Limits,
/// Cardiovascular/Uteroplacental Thresholds, and Clinical Care Obligations.
pub fn compile_human_welfare_q42_bytes(
    profile: HumanAnatomyProfile,
    gestational_week: u32,
) -> Vec<u8> {
    let m = HumanWelfareMeasurements::compute(profile, gestational_week);
    let subject = q_hash(&format!("did:q42:welfare:human:{}:w{}", m.profile_slug, m.gestational_week));
    let ctx = ((SENSITIVITY_PUBLIC as u64) << 56)
        | (q_hash("urn:qualia:contract:welfare:maternal-clinical:v1") & 0x00FF_FFFF_FFFF_FFFF);

    let make_quin = |pred_iri: &str, opcode: u8, defeater: bool, val_milli: u64, clock: u64| -> NQuin {
        let p_hash = (q_hash(pred_iri) & 0x007F_FFFF_FFFF_FFFF) << 8;
        let predicate = (if defeater { DEFEATER_BIT } else { 0 }) | p_hash | (opcode as u64);
        let object = INLINE_TAG_INTEGER | (val_milli & 0x0FFF_FFFF_FFFF_FFFF);
        let metadata = ROUTING_BILATERAL | ((clock & 0x1FFF_FFFF) << 32) | 990u64;
        let parity = subject ^ predicate ^ object ^ ctx;
        NQuin {
            subject,
            predicate,
            object,
            context: ctx,
            metadata,
            parity,
        }
    };

    let quins: [NQuin; 16] = [
        // 0-3: OP_PERMIT (0x11) Safe Anthropometric & Biomechanical Envelopes
        make_quin("urn:qualia:welfare:rom:safe_lumbar_flexion_mdeg", OP_PERMIT, false, (m.max_lumbar_flexion_safe_deg * 1000.0) as u64, 1),
        make_quin("urn:qualia:welfare:load:max_recommended_lift_g", OP_PERMIT, false, (m.max_recommended_lift_kg * 1000.0) as u64, 2),
        make_quin("urn:qualia:welfare:cardio:cardiac_output_ml_min", OP_PERMIT, false, (m.cardiac_output_l_min * 1000.0) as u64, 3),
        make_quin("urn:qualia:welfare:pelvis:inlet_transverse_um", OP_PERMIT, false, (m.pelvic_inlet_transverse_mm * 1000.0) as u64, 4),
        // 4-7: OP_FORBID (0x12) + DEFEATER_BIT Strain, Compression & Injury Thresholds
        make_quin("urn:qualia:welfare:injury:lumbar_shear_overload_n", OP_FORBID, true, if m.gestational_week >= 20 { 1800 } else { 3200 }, 5),
        make_quin("urn:qualia:welfare:injury:supine_vena_cava_compression_w", OP_FORBID, true, if m.gestational_week >= 20 { 1 } else { 0 }, 6),
        make_quin("urn:qualia:welfare:injury:pelvic_symphysis_diastasis_mm", OP_FORBID, true, 10_000, 7),
        make_quin("urn:qualia:welfare:injury:uteroplacental_hypoperfusion_ml", OP_FORBID, true, 250_000, 8),
        // 8-11: OP_OBLIGATE (0x10) Welfare Accommodation & Obstetric Monitoring Rules
        make_quin("urn:qualia:welfare:care:fundal_height_target_mm", OP_OBLIGATE, false, (m.symphysis_fundal_height_cm * 10.0) as u64, 9),
        make_quin("urn:qualia:welfare:care:fetal_weight_estimate_g", OP_OBLIGATE, false, m.estimated_fetal_weight_g as u64, 10),
        make_quin("urn:qualia:welfare:care:fetal_heart_rate_target_bpm", OP_OBLIGATE, false, m.fetal_heart_rate_bpm as u64, 11),
        make_quin("urn:qualia:welfare:care:left_lateral_tilt_rest_deg", OP_OBLIGATE, false, if m.gestational_week >= 20 { 15_000 } else { 0 }, 12),
        // 12-15: OP_LTL_GLOBALLY (0x40) Continuous 60Hz Physiological & Small-LLM Guard Traces
        make_quin("urn:qualia:ltl:globally:maternal_fetal_oxygenation_ok", OP_LTL_GLOBALLY, false, 1, 13),
        make_quin("urn:qualia:ltl:globally:pelvic_lordosis_balance_ok", OP_LTL_GLOBALLY, false, (m.lumbar_lordosis_deg * 1000.0) as u64, 14),
        make_quin("urn:qualia:ltl:globally:amniotic_fluid_index_mm", OP_LTL_GLOBALLY, false, (m.amniotic_fluid_index_cm * 10.0) as u64, 15),
        make_quin("urn:qualia:ltl:globally:fiduciary_consent_sanctuary_bound", OP_LTL_GLOBALLY, false, 1, 16),
    ];

    let mut out = Vec::with_capacity(64 + quins.len() * 48);
    out.extend_from_slice(b"Q42LOGIC"); // [0..8] Magic
    out.extend_from_slice(&1u32.to_le_bytes()); // [8..12] Version = 1
    out.extend_from_slice(&(quins.len() as u32).to_le_bytes()); // [12..16] Total 48B NQuin records (16)
    out.extend_from_slice(&3u32.to_le_bytes()); // [16..20] 3 = Human Welfare & Maternal-Fetal Clinical
    out.extend_from_slice(&4u32.to_le_bytes()); // [20..24] Safe RoM / Lift Permit Quins (OP_PERMIT=0x11)
    out.extend_from_slice(&4u32.to_le_bytes()); // [24..28] Strain / Vena Cava Forbid Quins (OP_FORBID=0x12)
    out.extend_from_slice(&4u32.to_le_bytes()); // [28..32] Care / Obstetric Obligate Quins (OP_OBLIGATE=0x10)
    out.extend_from_slice(&4u32.to_le_bytes()); // [32..36] LTL Temporal Guard Quins (OP_LTL_GLOBALLY=0x40)
    out.extend_from_slice(&subject.to_le_bytes()); // [36..44] Subject DID hash
    let mut xor_digest = 0u64;
    for q in &quins {
        xor_digest ^= q.parity;
    }
    out.extend_from_slice(&xor_digest.to_le_bytes()); // [44..52] XOR parity digest
    out.extend_from_slice(&[0u8; 12]); // [52..64] Reserved 64B header alignment

    for q in &quins {
        out.extend_from_slice(&q.subject.to_le_bytes());
        out.extend_from_slice(&q.predicate.to_le_bytes());
        out.extend_from_slice(&q.object.to_le_bytes());
        out.extend_from_slice(&q.context.to_le_bytes());
        out.extend_from_slice(&q.metadata.to_le_bytes());
        out.extend_from_slice(&q.parity.to_le_bytes());
    }
    out
}

impl AssetHost<'_> {
    /// Morphism `hypermedia.manifold.compile_human_welfare_anatomy_10d`:
    /// Compiles a 17-system Human Welfare & Maternal-Fetal `.10d` container (`male`, `female`,
    /// `female_trimester_1`, `female_trimester_2`, `female_trimester_3`), verifies `.10d` CRC-32C,
    /// generates anthropometric/obstetric measurements JSON, and emits a companion `.q42` rule container.
    pub(super) fn compile_human_welfare_anatomy_10d(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "profile",
                "gestational_week",
                "osteology_vis_01",
                "myology_vis_01",
                "integument_vis_01",
                "explode_01",
                "title",
            ],
        )?;

        let profile_str = super::opt_string(a, "profile", "female_trimester_3")?;
        let week_opt = if a.contains_key("gestational_week") {
            Some(count(a, "gestational_week", 0, 40)? as u32)
        } else {
            None
        };
        let (profile, week) = HumanAnatomyProfile::from_str_and_week(&profile_str, week_opt)?;
        let title_str = super::opt_string(a, "title", profile.label())?;

        let osteology_vis_01 = number(a, "osteology_vis_01", 1.0)? as f32;
        let myology_vis_01 = number(a, "myology_vis_01", 0.90)? as f32;
        let integument_vis_01 = number(a, "integument_vis_01", 0.45)? as f32;
        let explode_01 = number(a, "explode_01", 0.0)? as f32;

        self.charge(512, 16384)?;

        let mut compiled = build_human_welfare_anatomy_10d(
            profile,
            week,
            osteology_vis_01,
            myology_vis_01,
            integument_vis_01,
            explode_01,
        );
        verify_whole_file_crc32c(&mut compiled.container_bytes)
            .map_err(|e| format!("Human Welfare .10d CRC-32C verification failed: {e:?}"))?;

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

        let measurements = HumanWelfareMeasurements::compute(profile, week);
        let measurements_json = measurements.to_json_string();
        let logic_q42_bytes = compile_human_welfare_q42_bytes(profile, week);

        let v_count = compiled.vertex_count;
        let t_count = compiled.triangle_count;
        let submesh_count = compiled.submesh_ranges.len();
        let byte_len = compiled.container_bytes.len();

        let container_10d = self.encode("10d", compiled.container_bytes)?;
        let turntable_png = self.encode("png", turntable_bytes)?;
        let blueprint_svg = self.encode("svg", blueprint_svg_bytes)?;
        let measurements_res = self.encode("json", measurements_json.clone().into_bytes())?;
        let logic_q42 = self.encode("q42", logic_q42_bytes)?;

        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for p in &compiled.positions {
            for k in 0..3 {
                min[k] = min[k].min(p[k]);
                max[k] = max[k].max(p[k]);
            }
        }
        let triangles: Vec<[u32; 3]> = compiled
            .indices
            .chunks_exact(3)
            .map(|tri| [tri[0], tri[1], tri[2]])
            .collect();
        let mesh_handle = self.workspace.insert(
            Artifact::Mesh(Mesh {
                positions: compiled.positions,
                triangles,
                min,
                max,
            }),
            self.limits,
        )?;

        let mut out = BTreeMap::new();
        out.insert("profile".into(), Value::String(profile.slug().into()));
        out.insert("label".into(), Value::String(profile.label().into()));
        out.insert("gestational_week".into(), Value::I64(week as i64));
        out.insert("vertex_count".into(), Value::I64(v_count as i64));
        out.insert("triangle_count".into(), Value::I64(t_count as i64));
        out.insert("submesh_count".into(), Value::I64(submesh_count as i64));
        out.insert("container_byte_len".into(), Value::I64(byte_len as i64));
        out.insert("container_10d".into(), container_10d);
        out.insert("mesh".into(), mesh_handle);
        out.insert("turntable_png".into(), turntable_png);
        out.insert("blueprint_svg".into(), blueprint_svg);
        out.insert("measurements_json".into(), Value::String(measurements_json));
        out.insert("measurements_resource".into(), measurements_res);
        out.insert("logic_q42".into(), logic_q42);

        Ok(Value::Record(out))
    }
}
