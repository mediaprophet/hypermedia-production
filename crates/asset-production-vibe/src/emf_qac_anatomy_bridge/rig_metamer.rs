//! Articulated vertebrate anatomical rig and botanical FSPM metamer field parameterization.
//!
//! Backed by `qualia_core_db::q42::species_corpus::SpeciesCorpus` evidenced assertions.
//! Zero comparative ellipsoid placeholders: every vertebrate segment is an articulated
//! Bishop-frame anatomical span with explicit proximal/distal joints and mediolateral/dorsoventral
//! cross-section radii, and every botanical metamer enforces Murray's law ($r_p^3 = \sum r_c^3$)
//! across xylem/phloem vascular bundles and chlorophyll red-edge (680–730 nm) reflectance.

use qualia_core_db::q42::species_corpus::SpeciesCorpus;
use std::collections::{BTreeMap, BTreeSet};

/// Planck constant $h$ in $\text{J}\cdot\text{s}$.
pub const PLANCK_H_J_S: f64 = 6.626_070_15e-34;
/// Boltzmann constant $k_B$ in $\text{J}\cdot\text{K}^{-1}$.
pub const BOLTZMANN_K_J_PER_K: f64 = 1.380_649e-23;
/// Exact speed of light in vacuum $c$ in $\text{m}\cdot\text{s}^{-1}$.
pub const VACUUM_C_M_PER_S: f64 = 299_792_458.0;
/// Free-space impedance $Z_0 = \mu_0 c$ in $\Omega$.
pub const VACUUM_IMPEDANCE_OHM: f64 = 376.730_313_668;

/// Functional anatomical category of an articulated vertebrate segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum VertebrateSegmentKind {
    AxialSpine,
    Thorax,
    Cranium,
    Forelimb,
    Hindlimb,
    Tail,
}

impl VertebrateSegmentKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AxialSpine => "axial_spine",
            Self::Thorax => "thorax",
            Self::Cranium => "cranium",
            Self::Forelimb => "forelimb",
            Self::Hindlimb => "hindlimb",
            Self::Tail => "tail",
        }
    }
}

/// One articulated vertebrate anatomical segment backed by a `SpeciesCorpus` assertion.
#[derive(Clone, Debug, PartialEq)]
pub struct ArticulatedVertebrateSegment {
    pub segment_id: String,
    pub kind: VertebrateSegmentKind,
    pub uberon_iri: String,
    pub parent_index: Option<usize>,
    pub proximal_joint_m: [f64; 3],
    pub distal_joint_m: [f64; 3],
    /// Mediolateral and dorsoventral cross-section radii in metres along the articulated span.
    pub cross_section_radii_m: [f64; 2],
    /// Normalized neuromuscular activation in `[0.0, 1.0]`.
    pub muscular_activation_01: f64,
    /// Segment metabolic tissue temperature in Kelvin.
    pub metabolic_temperature_k: f64,
    /// Dermal spectral reflectance values in `[0.0, 1.0]` matching the rig's spectral axis.
    pub dermal_reflectance_bins: Vec<f64>,
    /// Structural damped acoustic modal resonance frequency in Hz.
    pub structural_modal_freq_hz: f64,
    /// Structural modal exponential decay rate in $\text{s}^{-1}$.
    pub structural_damping_s1: f64,
    /// Cited `EvidencedAssertion` identifier in the backing `SpeciesCorpus`.
    pub assertion_id: String,
}

impl ArticulatedVertebrateSegment {
    /// Segment midpoint in local frame metres.
    pub fn midpoint_m(&self) -> [f64; 3] {
        [
            0.5 * (self.proximal_joint_m[0] + self.distal_joint_m[0]),
            0.5 * (self.proximal_joint_m[1] + self.distal_joint_m[1]),
            0.5 * (self.proximal_joint_m[2] + self.distal_joint_m[2]),
        ]
    }

    /// Segment length in metres.
    pub fn length_m(&self) -> f64 {
        let dx = self.distal_joint_m[0] - self.proximal_joint_m[0];
        let dy = self.distal_joint_m[1] - self.proximal_joint_m[1];
        let dz = self.distal_joint_m[2] - self.proximal_joint_m[2];
        dx.hypot(dy).hypot(dz)
    }

    /// Unit axis from proximal to distal joint.
    pub fn unit_axis(&self) -> Result<[f64; 3], String> {
        let len = self.length_m();
        if !len.is_finite() || len <= 1e-6 {
            return Err(format!(
                "Segment '{}' has degenerate span length {len}",
                self.segment_id
            ));
        }
        Ok([
            (self.distal_joint_m[0] - self.proximal_joint_m[0]) / len,
            (self.distal_joint_m[1] - self.proximal_joint_m[1]) / len,
            (self.distal_joint_m[2] - self.proximal_joint_m[2]) / len,
        ])
    }
}

/// Physiological origin of a bioelectric dipole source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BioelectricSourceKind {
    CardiacVentricularSyncytium,
    NeuralCorticalColumn,
    MyoelectricMotorUnit,
}

impl BioelectricSourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CardiacVentricularSyncytium => "cardiac_ventricular_syncytium",
            Self::NeuralCorticalColumn => "neural_cortical_column",
            Self::MyoelectricMotorUnit => "myoelectric_motor_unit",
        }
    }
}

/// Bioelectric current dipole source bound to an articulated vertebrate segment (Mode `0x0107`).
#[derive(Clone, Debug, PartialEq)]
pub struct BioelectricDipoleSource {
    pub source_id: String,
    pub kind: BioelectricSourceKind,
    pub anatomy_ontology: String,
    pub anatomy_identifier: String,
    pub observation_type: String,
    pub segment_index: usize,
    pub local_offset_m: [f64; 3],
    /// Current dipole moment $\mathbf{p}$ in $\text{A}\cdot\text{m}$.
    pub current_dipole_moment_a_m: [f64; 3],
    /// Extracellular potential $\Phi$ in Volts at the segment boundary.
    pub extracellular_potential_v: f64,
    /// Characteristic physiological carrier frequency in Hz.
    pub carrier_hz: f64,
    /// Standard measurement/model uncertainty in $\text{A}\cdot\text{m}$.
    pub uncertainty_a_m: f64,
    /// Cited `EvidencedAssertion` identifier in the backing `SpeciesCorpus`.
    pub assertion_id: String,
}

/// Vocalization and footfall acoustic cue parameters for a vertebrate rig.
#[derive(Clone, Debug, PartialEq)]
pub struct VertebrateAcousticCueSpec {
    pub vocalization_fundamental_hz: f64,
    pub vocalization_formants_hz: [f64; 3],
    pub footfall_impulse_pa: f64,
    pub hoa_azimuth_rad: f64,
    pub hoa_elevation_rad: f64,
    pub sample_rate_hz: u32,
    pub frame_count: usize,
}

/// Complete articulated vertebrate rig parameter set backed by a `SpeciesCorpus`.
#[derive(Clone, Debug, PartialEq)]
pub struct VertebrateAnatomicalRig {
    pub rig_id: String,
    pub species_id: String,
    pub taxon_concept_iri: String,
    pub corpus_iri: String,
    pub epoch_seconds: i64,
    pub spectral_wavelengths_m: Vec<f64>,
    pub segments: Vec<ArticulatedVertebrateSegment>,
    pub dipoles: Vec<BioelectricDipoleSource>,
    pub acoustic_cue: VertebrateAcousticCueSpec,
}

/// Verified summary of an articulated vertebrate rig after corpus admission.
#[derive(Clone, Debug, PartialEq)]
pub struct VertebrateRigValidationSummary {
    pub rig_id: String,
    pub species_id: String,
    pub segment_count: usize,
    pub dipole_count: usize,
    pub spectral_bin_count: usize,
    pub cited_assertion_ids: Vec<String>,
    pub total_axial_length_m: f64,
    pub mean_metabolic_temperature_k: f64,
}

/// Functional organ kind of a botanical FSPM metamer.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BotanicalMetamerKind {
    Internode,
    Petiole,
    Lamina,
}

impl BotanicalMetamerKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Internode => "internode",
            Self::Petiole => "petiole",
            Self::Lamina => "lamina",
        }
    }
}

/// One botanical FSPM metamer segment with vascular radii and foliar red-edge spectrum.
#[derive(Clone, Debug, PartialEq)]
pub struct BotanicalMetamerSegment {
    pub metamer_id: String,
    pub kind: BotanicalMetamerKind,
    pub plant_ontology_iri: String,
    pub parent_index: Option<usize>,
    pub proximal_node_m: [f64; 3],
    pub distal_node_m: [f64; 3],
    /// Murray-law xylem conducting radius in metres ($r_{\text{parent}}^3 = \sum r_{\text{child}}^3$).
    pub xylem_radius_m: f64,
    /// Murray-law phloem conducting radius in metres ($r_{\text{parent}}^3 = \sum r_{\text{child}}^3$).
    pub phloem_radius_m: f64,
    /// One-sided projected foliar lamina area in $\text{m}^2$ (0.0 for leafless woody internodes).
    pub lamina_area_m2: f64,
    /// Leaf/stem surface temperature in Kelvin.
    pub leaf_temperature_k: f64,
    /// Hemispherical spectral reflectance bins in `[0.0, 1.0]` across `spectral_wavelengths_m`.
    pub red_edge_reflectance_bins: Vec<f64>,
    /// Aeroacoustic leaf/branch modal frequency in Hz.
    pub rustle_modal_freq_hz: f64,
    /// Aeroacoustic modal decay rate in $\text{s}^{-1}$.
    pub rustle_damping_s1: f64,
    /// Aeroacoustic modal excitation pressure amplitude in Pascals.
    pub rustle_amplitude_pa: f64,
    /// Cited `EvidencedAssertion` identifier in the backing `SpeciesCorpus`.
    pub assertion_id: String,
}

impl BotanicalMetamerSegment {
    pub fn midpoint_m(&self) -> [f64; 3] {
        [
            0.5 * (self.proximal_node_m[0] + self.distal_node_m[0]),
            0.5 * (self.proximal_node_m[1] + self.distal_node_m[1]),
            0.5 * (self.proximal_node_m[2] + self.distal_node_m[2]),
        ]
    }

    pub fn length_m(&self) -> f64 {
        let dx = self.distal_node_m[0] - self.proximal_node_m[0];
        let dy = self.distal_node_m[1] - self.proximal_node_m[1];
        let dz = self.distal_node_m[2] - self.proximal_node_m[2];
        dx.hypot(dy).hypot(dz)
    }

    pub fn unit_axis(&self) -> Result<[f64; 3], String> {
        let len = self.length_m();
        if !len.is_finite() || len <= 1e-6 {
            return Err(format!(
                "Botanical metamer '{}' has degenerate length {len}",
                self.metamer_id
            ));
        }
        Ok([
            (self.distal_node_m[0] - self.proximal_node_m[0]) / len,
            (self.distal_node_m[1] - self.proximal_node_m[1]) / len,
            (self.distal_node_m[2] - self.proximal_node_m[2]) / len,
        ])
    }
}

/// Complete botanical FSPM metamer rig parameter set backed by a `SpeciesCorpus`.
#[derive(Clone, Debug, PartialEq)]
pub struct BotanicalMetamerRig {
    pub plant_id: String,
    pub species_id: String,
    pub taxon_concept_iri: String,
    pub corpus_iri: String,
    pub epoch_seconds: i64,
    pub spectral_wavelengths_m: Vec<f64>,
    pub metamers: Vec<BotanicalMetamerSegment>,
    pub wind_velocity_m_s: [f64; 3],
    pub sample_rate_hz: u32,
    pub frame_count: usize,
}

/// Verified summary of a botanical FSPM metamer rig after Murray-law and red-edge admission.
#[derive(Clone, Debug, PartialEq)]
pub struct BotanicalRigValidationSummary {
    pub plant_id: String,
    pub species_id: String,
    pub metamer_count: usize,
    pub branching_junction_count: usize,
    pub max_murray_xylem_relative_error: f64,
    pub max_murray_phloem_relative_error: f64,
    pub red_edge_slope_per_nm: f64,
    pub cited_assertion_ids: Vec<String>,
}

fn validate_spectral_wavelengths(wavelengths_m: &[f64]) -> Result<(), String> {
    if wavelengths_m.len() < 4 || wavelengths_m.len() > 4096 {
        return Err(format!(
            "spectral_wavelengths_m count {} outside 4..=4096",
            wavelengths_m.len()
        ));
    }
    let mut prev = 0.0;
    for &w in wavelengths_m {
        if !w.is_finite() || w <= 0.0 || w <= prev {
            return Err(
                "spectral_wavelengths_m must be finite, positive, and strictly ascending".into(),
            );
        }
        prev = w;
    }
    Ok(())
}

/// Evaluates Planck spectral radiance $B_\lambda(\lambda, T)$ in $\text{W}\cdot\text{sr}^{-1}\cdot\text{m}^{-2}\cdot\text{m}^{-1}$.
pub fn planck_spectral_radiance_w_sr_m3(wavelength_m: f64, temperature_k: f64) -> Result<f64, String> {
    if !wavelength_m.is_finite()
        || wavelength_m <= 0.0
        || !temperature_k.is_finite()
        || temperature_k <= 0.0
    {
        return Err("Planck radiance requires positive finite wavelength and temperature".into());
    }
    let c1 = 2.0 * PLANCK_H_J_S * VACUUM_C_M_PER_S * VACUUM_C_M_PER_S;
    let exponent = (PLANCK_H_J_S * VACUUM_C_M_PER_S) / (wavelength_m * BOLTZMANN_K_J_PER_K * temperature_k);
    if !exponent.is_finite() || exponent <= 0.0 {
        return Err("Non-representable Planck exponent".into());
    }
    let denom = wavelength_m.powi(5) * exponent.min(700.0).exp_m1();
    if !denom.is_finite() || denom <= 0.0 {
        return Ok(0.0);
    }
    let radiance = c1 / denom;
    if !radiance.is_finite() || radiance < 0.0 {
        return Err("Planck radiance overflow".into());
    }
    Ok(radiance)
}

impl VertebrateAnatomicalRig {
    /// Validates the articulated vertebrate rig against its backing `SpeciesCorpus`.
    pub fn validate(&self, corpus: &SpeciesCorpus) -> Result<VertebrateRigValidationSummary, String> {
        corpus.validate()?;
        if self.rig_id.trim().is_empty() {
            return Err("Vertebrate rig_id is empty".into());
        }
        if self.species_id != corpus.species_id {
            return Err(format!(
                "Rig species_id '{}' does not match corpus species_id '{}'",
                self.species_id, corpus.species_id
            ));
        }
        if self.taxon_concept_iri != corpus.taxon_concept_iri {
            return Err(format!(
                "Rig taxon_concept_iri '{}' does not match corpus '{}'",
                self.taxon_concept_iri, corpus.taxon_concept_iri
            ));
        }
        if self.corpus_iri != corpus.corpus_iri {
            return Err(format!(
                "Rig corpus_iri '{}' does not match corpus '{}'",
                self.corpus_iri, corpus.corpus_iri
            ));
        }
        validate_spectral_wavelengths(&self.spectral_wavelengths_m)?;
        if self.segments.len() < 4 || self.segments.len() > 256 {
            return Err(format!(
                "Articulated vertebrate rig requires 4..=256 segments (rejects single-ellipsoid placeholders), found {}",
                self.segments.len()
            ));
        }
        if self.dipoles.is_empty() || self.dipoles.len() > 128 {
            return Err("Articulated vertebrate rig requires 1..=128 bioelectric dipoles".into());
        }
        if !(8000..=192_000).contains(&self.acoustic_cue.sample_rate_hz)
            || !(64..=65_536).contains(&self.acoustic_cue.frame_count)
            || !self.acoustic_cue.vocalization_fundamental_hz.is_finite()
            || self.acoustic_cue.vocalization_fundamental_hz <= 20.0
            || !self.acoustic_cue.footfall_impulse_pa.is_finite()
            || self.acoustic_cue.footfall_impulse_pa <= 0.0
        {
            return Err("Invalid vertebrate acoustic cue parameters".into());
        }

        let active_assertions: BTreeMap<&str, _> = corpus
            .facts()
            .into_iter()
            .filter(|f| f.active)
            .map(|f| (f.assertion_id, f))
            .collect();

        let mut seen_segments = BTreeSet::new();
        let mut kinds_present = BTreeSet::new();
        let mut cited = BTreeSet::new();
        let mut total_axial_length_m = 0.0;
        let mut temp_sum_k = 0.0;

        for (idx, seg) in self.segments.iter().enumerate() {
            if seg.segment_id.trim().is_empty() || !seen_segments.insert(seg.segment_id.as_str()) {
                return Err(format!("Duplicate or empty segment_id '{}'", seg.segment_id));
            }
            if seg.segment_id.contains("ellipsoid") || seg.uberon_iri.trim().is_empty() {
                return Err(format!(
                    "Segment '{}' uses comparative ellipsoid placeholder or empty UBERON IRI",
                    seg.segment_id
                ));
            }
            if idx == 0 {
                if seg.parent_index.is_some() {
                    return Err("Root segment 0 must have parent_index == None".into());
                }
            } else {
                let Some(p) = seg.parent_index else {
                    return Err(format!("Non-root segment '{}' missing parent_index", seg.segment_id));
                };
                if p >= idx {
                    return Err(format!(
                        "Segment '{}' parent_index {p} must precede {idx}",
                        seg.segment_id
                    ));
                }
            }
            if seg
                .proximal_joint_m
                .iter()
                .chain(&seg.distal_joint_m)
                .any(|v| !v.is_finite() || v.abs() > 10.0)
            {
                return Err(format!("Segment '{}' joint coordinates out of bounds", seg.segment_id));
            }
            let len = seg.length_m();
            let _ = seg.unit_axis()?;
            if seg
                .cross_section_radii_m
                .iter()
                .any(|&r| !r.is_finite() || r <= 1e-4 || r > 2.0)
            {
                return Err(format!("Segment '{}' has invalid cross_section_radii_m", seg.segment_id));
            }
            if !seg.muscular_activation_01.is_finite()
                || !(0.0..=1.0).contains(&seg.muscular_activation_01)
            {
                return Err(format!(
                    "Segment '{}' muscular_activation_01 outside 0..=1",
                    seg.segment_id
                ));
            }
            if !seg.metabolic_temperature_k.is_finite()
                || !(270.0..=325.0).contains(&seg.metabolic_temperature_k)
            {
                return Err(format!(
                    "Segment '{}' metabolic_temperature_k outside 270..=325 K",
                    seg.segment_id
                ));
            }
            if seg.dermal_reflectance_bins.len() != self.spectral_wavelengths_m.len()
                || seg
                    .dermal_reflectance_bins
                    .iter()
                    .any(|&v| !v.is_finite() || !(0.0..=1.0).contains(&v))
            {
                return Err(format!(
                    "Segment '{}' dermal_reflectance_bins must match spectral axis and lie in 0..=1",
                    seg.segment_id
                ));
            }
            if !seg.structural_modal_freq_hz.is_finite()
                || seg.structural_modal_freq_hz <= 0.0
                || !seg.structural_damping_s1.is_finite()
                || seg.structural_damping_s1 <= 0.0
            {
                return Err(format!(
                    "Segment '{}' has invalid modal frequency or damping",
                    seg.segment_id
                ));
            }
            if !active_assertions.contains_key(seg.assertion_id.as_str()) {
                return Err(format!(
                    "Segment '{}' cites unknown or inactive corpus assertion '{}'",
                    seg.segment_id, seg.assertion_id
                ));
            }
            cited.insert(seg.assertion_id.clone());
            kinds_present.insert(seg.kind);
            if matches!(
                seg.kind,
                VertebrateSegmentKind::AxialSpine
                    | VertebrateSegmentKind::Thorax
                    | VertebrateSegmentKind::Cranium
                    | VertebrateSegmentKind::Tail
            ) {
                total_axial_length_m += len;
            }
            temp_sum_k += seg.metabolic_temperature_k;
        }

        for required_kind in [
            VertebrateSegmentKind::AxialSpine,
            VertebrateSegmentKind::Thorax,
            VertebrateSegmentKind::Cranium,
            VertebrateSegmentKind::Hindlimb,
        ] {
            if !kinds_present.contains(&required_kind) {
                return Err(format!(
                    "Articulated vertebrate rig missing required anatomical segment kind '{}'",
                    required_kind.as_str()
                ));
            }
        }

        let mut seen_dipoles = BTreeSet::new();
        for dip in &self.dipoles {
            if dip.source_id.trim().is_empty() || !seen_dipoles.insert(dip.source_id.as_str()) {
                return Err(format!("Duplicate or empty dipole source_id '{}'", dip.source_id));
            }
            if dip.segment_index >= self.segments.len() {
                return Err(format!(
                    "Dipole '{}' references out-of-range segment_index {}",
                    dip.source_id, dip.segment_index
                ));
            }
            if dip.anatomy_ontology.trim().is_empty()
                || dip.anatomy_identifier.trim().is_empty()
                || dip.observation_type.trim().is_empty()
            {
                return Err(format!(
                    "Dipole '{}' requires non-empty Schema 137 BioContext strings",
                    dip.source_id
                ));
            }
            if dip
                .local_offset_m
                .iter()
                .chain(&dip.current_dipole_moment_a_m)
                .any(|v| !v.is_finite())
                || !dip.extracellular_potential_v.is_finite()
                || !dip.carrier_hz.is_finite()
                || dip.carrier_hz <= 0.0
                || !dip.uncertainty_a_m.is_finite()
                || dip.uncertainty_a_m <= 0.0
            {
                return Err(format!(
                    "Dipole '{}' has non-finite or non-positive physical parameters",
                    dip.source_id
                ));
            }
            let moment_mag = dip.current_dipole_moment_a_m[0]
                .hypot(dip.current_dipole_moment_a_m[1])
                .hypot(dip.current_dipole_moment_a_m[2]);
            if moment_mag <= 0.0 {
                return Err(format!(
                    "Dipole '{}' must have non-zero dipole moment magnitude",
                    dip.source_id
                ));
            }
            if !active_assertions.contains_key(dip.assertion_id.as_str()) {
                return Err(format!(
                    "Dipole '{}' cites unknown or inactive corpus assertion '{}'",
                    dip.source_id, dip.assertion_id
                ));
            }
            cited.insert(dip.assertion_id.clone());
        }

        Ok(VertebrateRigValidationSummary {
            rig_id: self.rig_id.clone(),
            species_id: self.species_id.clone(),
            segment_count: self.segments.len(),
            dipole_count: self.dipoles.len(),
            spectral_bin_count: self.spectral_wavelengths_m.len(),
            cited_assertion_ids: cited.into_iter().collect(),
            total_axial_length_m,
            mean_metabolic_temperature_k: temp_sum_k / (self.segments.len() as f64),
        })
    }
}

impl BotanicalMetamerRig {
    /// Validates the botanical FSPM metamer rig against its `SpeciesCorpus`,
    /// enforcing Murray's Law ($r_p^3 = \sum r_c^3$) and the 680–730 nm chlorophyll red-edge.
    pub fn validate(&self, corpus: &SpeciesCorpus) -> Result<BotanicalRigValidationSummary, String> {
        corpus.validate()?;
        if self.plant_id.trim().is_empty() {
            return Err("Botanical plant_id is empty".into());
        }
        if self.species_id != corpus.species_id
            || self.taxon_concept_iri != corpus.taxon_concept_iri
            || self.corpus_iri != corpus.corpus_iri
        {
            return Err("Botanical rig identity does not match SpeciesCorpus".into());
        }
        validate_spectral_wavelengths(&self.spectral_wavelengths_m)?;
        if self.metamers.len() < 3 || self.metamers.len() > 512 {
            return Err(format!(
                "Botanical metamer rig requires 3..=512 metamers, found {}",
                self.metamers.len()
            ));
        }
        if self.wind_velocity_m_s.iter().any(|v| !v.is_finite())
            || !(8000..=192_000).contains(&self.sample_rate_hz)
            || !(64..=65_536).contains(&self.frame_count)
        {
            return Err("Invalid botanical aeroacoustic parameters".into());
        }

        // Locate 680 nm and 730 nm red-edge indices in the spectral axis.
        let idx_680 = self
            .spectral_wavelengths_m
            .iter()
            .position(|&w| (w - 680e-9).abs() <= 15e-9)
            .ok_or("Botanical spectral axis must include a ~680 nm chlorophyll absorption bin")?;
        let idx_730 = self
            .spectral_wavelengths_m
            .iter()
            .position(|&w| (w - 730e-9).abs() <= 20e-9 && w > self.spectral_wavelengths_m[idx_680])
            .ok_or("Botanical spectral axis must include a ~730 nm red-edge shoulder bin")?;

        let active_assertions: BTreeMap<&str, _> = corpus
            .facts()
            .into_iter()
            .filter(|f| f.active)
            .map(|f| (f.assertion_id, f))
            .collect();

        let mut seen_ids = BTreeSet::new();
        let mut children_by_parent: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        let mut cited = BTreeSet::new();
        let mut max_red_edge_slope = 0.0f64;

        for (idx, m) in self.metamers.iter().enumerate() {
            if m.metamer_id.trim().is_empty() || !seen_ids.insert(m.metamer_id.as_str()) {
                return Err(format!("Duplicate or empty metamer_id '{}'", m.metamer_id));
            }
            if m.plant_ontology_iri.trim().is_empty() {
                return Err(format!("Metamer '{}' has empty plant_ontology_iri", m.metamer_id));
            }
            if idx == 0 {
                if m.parent_index.is_some() {
                    return Err("Root metamer 0 must have parent_index == None".into());
                }
            } else {
                let Some(p) = m.parent_index else {
                    return Err(format!("Non-root metamer '{}' missing parent_index", m.metamer_id));
                };
                if p >= idx {
                    return Err(format!(
                        "Metamer '{}' parent_index {p} must precede {idx}",
                        m.metamer_id
                    ));
                }
                children_by_parent.entry(p).or_default().push(idx);
            }
            let _ = m.unit_axis()?;
            if !m.xylem_radius_m.is_finite()
                || m.xylem_radius_m <= 1e-5
                || !m.phloem_radius_m.is_finite()
                || m.phloem_radius_m <= 1e-5
                || !m.lamina_area_m2.is_finite()
                || m.lamina_area_m2 < 0.0
                || !m.leaf_temperature_k.is_finite()
                || !(260.0..=330.0).contains(&m.leaf_temperature_k)
            {
                return Err(format!(
                    "Metamer '{}' has invalid vascular radii, area, or temperature",
                    m.metamer_id
                ));
            }
            if m.red_edge_reflectance_bins.len() != self.spectral_wavelengths_m.len()
                || m.red_edge_reflectance_bins
                    .iter()
                    .any(|&r| !r.is_finite() || !(0.0..=1.0).contains(&r))
            {
                return Err(format!(
                    "Metamer '{}' red_edge_reflectance_bins invalid",
                    m.metamer_id
                ));
            }
            let r680 = m.red_edge_reflectance_bins[idx_680];
            let r730 = m.red_edge_reflectance_bins[idx_730];
            let dw_nm = (self.spectral_wavelengths_m[idx_730] - self.spectral_wavelengths_m[idx_680]) * 1e9;
            let slope = (r730 - r680) / dw_nm;
            if m.kind == BotanicalMetamerKind::Lamina && (slope < 0.005 || r730 < 3.5 * r680) {
                return Err(format!(
                    "Foliar lamina metamer '{}' fails chlorophyll red-edge transition (r680={r680:.4}, r730={r730:.4}, slope={slope:.5}/nm)",
                    m.metamer_id
                ));
            }
            max_red_edge_slope = max_red_edge_slope.max(slope);

            if !m.rustle_modal_freq_hz.is_finite()
                || m.rustle_modal_freq_hz <= 0.0
                || !m.rustle_damping_s1.is_finite()
                || m.rustle_damping_s1 <= 0.0
                || !m.rustle_amplitude_pa.is_finite()
                || m.rustle_amplitude_pa <= 0.0
            {
                return Err(format!(
                    "Metamer '{}' has invalid aeroacoustic modal parameters",
                    m.metamer_id
                ));
            }
            if !active_assertions.contains_key(m.assertion_id.as_str()) {
                return Err(format!(
                    "Metamer '{}' cites unknown or inactive corpus assertion '{}'",
                    m.metamer_id, m.assertion_id
                ));
            }
            cited.insert(m.assertion_id.clone());
        }

        if children_by_parent.is_empty() {
            return Err("Botanical metamer rig requires at least one branching junction".into());
        }

        let mut max_xylem_err = 0.0f64;
        let mut max_phloem_err = 0.0f64;
        for (&parent_idx, child_indices) in &children_by_parent {
            let parent = &self.metamers[parent_idx];
            let parent_x3 = parent.xylem_radius_m.powi(3);
            let child_x3: f64 = child_indices
                .iter()
                .map(|&ci| self.metamers[ci].xylem_radius_m.powi(3))
                .sum();
            let x_err = (parent_x3 - child_x3).abs() / parent_x3;
            if x_err > 1e-5 {
                return Err(format!(
                    "Murray's law xylem violation at parent '{}': r_p^3={parent_x3:.6e} vs sum(r_c^3)={child_x3:.6e} (rel_err={x_err:.3e})",
                    parent.metamer_id
                ));
            }
            max_xylem_err = max_xylem_err.max(x_err);

            let parent_p3 = parent.phloem_radius_m.powi(3);
            let child_p3: f64 = child_indices
                .iter()
                .map(|&ci| self.metamers[ci].phloem_radius_m.powi(3))
                .sum();
            let p_err = (parent_p3 - child_p3).abs() / parent_p3;
            if p_err > 1e-5 {
                return Err(format!(
                    "Murray's law phloem violation at parent '{}': r_p^3={parent_p3:.6e} vs sum(r_c^3)={child_p3:.6e} (rel_err={p_err:.3e})",
                    parent.metamer_id
                ));
            }
            max_phloem_err = max_phloem_err.max(p_err);
        }

        Ok(BotanicalRigValidationSummary {
            plant_id: self.plant_id.clone(),
            species_id: self.species_id.clone(),
            metamer_count: self.metamers.len(),
            branching_junction_count: children_by_parent.len(),
            max_murray_xylem_relative_error: max_xylem_err,
            max_murray_phloem_relative_error: max_phloem_err,
            red_edge_slope_per_nm: max_red_edge_slope,
            cited_assertion_ids: cited.into_iter().collect(),
        })
    }
}

pub use crate::emf_qac_anatomy_bridge::rig_fixtures::{
    canonical_ghost_gum_metamer_rig_and_corpus, canonical_wallaby_rig_and_corpus,
};

