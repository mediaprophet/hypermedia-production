//! Deterministic Current-Format EMF v4 (`Q42EMF64` version 4) Producer.
//!
//! Assembles canonical 128-byte header + 128-byte directory + 8-byte aligned TLV metadata +
//! 64-byte aligned payload chunks using `qualia_core_db::q42::field_envelope` and
//! `qualia_core_db::q42::emf_profiles` / `field_spectrum`:
//! - Mode `0x0100` (`instantaneous_E_B_v1`, 128B stride)
//! - Mode `0x0101` (`polarimetric_ray_v1`, 128B stride, Schema 135 `PolarizationBasis`)
//! - Mode `0x0103` (`sampled_spectrum_v1`, Schema 132 continuous spectral bins)
//! - Mode `0x0107` (`bioelectric_dipole_v1`, 128B stride with Schema 137 `BioContext`,
//!   Schema 134 `ModelEvidence`, Schema 107 `Calibration`, Schema 4 `authority_receipt_digest`,
//!   and per-record CRC-32C at `124..128`)

use crate::emf_qac_anatomy_bridge::rig_metamer::{
    planck_spectral_radiance_w_sr_m3, BotanicalMetamerRig, VertebrateAnatomicalRig,
    VACUUM_C_M_PER_S, VACUUM_IMPEDANCE_OHM,
};
use crate::Resource;
use qualia_core_db::container_10d::crc32c::crc32c;
use qualia_core_db::q42::emf_profiles::ProfileRegistry;
use qualia_core_db::q42::field_envelope::{Block, Entry};
use qualia_core_db::q42::field_semantics::{self as semantics, Context, DomainRegistry, FieldRule};
use qualia_core_db::q42::species_corpus::SpeciesCorpus;
use std::collections::BTreeMap;

/// Canonical wire-freeze EMF v4 Mode `0x0100`: `instantaneous_E_B_v1` (128B stride).
pub const MODE_INSTANTANEOUS_E_B_V1: u16 = 0x0100;
/// Canonical wire-freeze EMF v4 Mode `0x0101`: `polarimetric_ray_v1` (128B stride).
pub const MODE_POLARIMETRIC_RAY_V1: u16 = 0x0101;
/// Canonical wire-freeze EMF v4 Mode `0x0103`: `sampled_spectrum_v1` (`64 + ((8*N + 63) & !63)` stride).
pub const MODE_SAMPLED_SPECTRUM_V1: u16 = 0x0103;
/// Canonical wire-freeze EMF v4 Mode `0x0107`: `bioelectric_dipole_v1` (128B stride).
pub const MODE_BIOELECTRIC_DIPOLE_V1: u16 = 0x0107;

/// Candidate point-stream mode IDs used by `qualia_core_db::q42::emf_profiles::ProfileRegistry`.
pub const CANDIDATE_MODE_E_B: u16 = 0;
pub const CANDIDATE_MODE_RAY: u16 = 1;
pub const CANDIDATE_MODE_SPECTRUM: u16 = 3;

pub const BIOELECTRIC_PROFILE: &str = "bioelectric_dipole_v1";
pub const BIOELECTRIC_MODEL_URI: &str = "urn:qualia:emf:bioelectric_dipole_v1";

/// Domain registry admitting wire-freeze EMF v4 metadata schemas (`103, 107, 132, 134, 135, 137`),
/// delegating directly to `qualia_core_db::q42::emf_profiles::ProfileRegistry`.
#[derive(Clone, Copy, Debug, Default)]
pub struct EmfV4AnatomyRegistry;

impl DomainRegistry for EmfV4AnatomyRegistry {
    fn rules(&self, schema: u16, revision: u16) -> Option<&'static [FieldRule]> {
        ProfileRegistry.rules(schema, revision)
    }

    fn descriptor_target_schema(&self, key: u16) -> Option<u16> {
        ProfileRegistry.descriptor_target_schema(key)
    }

    fn validate_block(
        &self,
        block: Block<'_>,
        ctx: &Context<'_, '_>,
    ) -> Result<(), semantics::Error> {
        ProfileRegistry.validate_block(block, ctx)
    }

    fn validate_grid_frame(
        &self,
        frame: Block<'_>,
        ctx: &Context<'_, '_>,
    ) -> Result<(), semantics::Error> {
        ProfileRegistry.validate_grid_frame(frame, ctx)
    }

    fn validate_spatial_entry(
        &self,
        entry: Entry<'_>,
        frame: Block<'_>,
        ctx: &Context<'_, '_>,
    ) -> Result<(), semantics::Error> {
        ProfileRegistry.validate_spatial_entry(entry, frame, ctx)
    }

    fn validate_units(&self, units: &str, ctx: &Context<'_, '_>) -> Result<(), semantics::Error> {
        ProfileRegistry.validate_units(units, ctx)
    }
}

/// Output bundle from deterministic EMF v4 production.
#[derive(Clone, Debug, PartialEq)]
pub struct EmfV4ProducedBundle {
    /// Canonical 4-mode wire-freeze `Q42EMF64` version 4 container (`0x0100, 0x0101, 0x0103, 0x0107`).
    pub canonical_resource: Resource,
    pub canonical_sha256_hex: String,
    pub canonical_blake3_hex: String,
    /// Companion `emf_profiles`-admitted point-stream container (modes `0, 1, 3`).
    pub profile_resource: Resource,
    pub profile_sha256_hex: String,
    pub profile_blake3_hex: String,
    /// Deterministic per-mode record counts in the canonical container.
    pub mode_record_counts: BTreeMap<u16, u64>,
    /// Spectral axis sample wavelengths in metres (Schema 132).
    pub spectral_wavelengths_m: Vec<f64>,
    /// Species identifier from the backing `SpeciesCorpus`.
    pub species_id: String,
    /// Primary anatomical ontology identifier in Schema 137 `BioContext`.
    pub primary_anatomy_iri: String,
}

/// Encode one 128-byte Mode `0x0100` (`instantaneous_E_B_v1`) record.
pub fn encode_instantaneous_e_b_record(
    electric_v_per_m: [f64; 3],
    magnetic_tesla: [f64; 3],
    local_position_m: [f64; 3],
    record_tick: i64,
    carrier_hz: f64,
    stored_poynting_w_per_m2: [f64; 3],
    confidence: f32,
) -> [u8; 128] {
    let mut rec = [0u8; 128];
    for (i, v) in electric_v_per_m.into_iter().enumerate() {
        rec[i * 8..(i + 1) * 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    for (i, v) in magnetic_tesla.into_iter().enumerate() {
        let at = 24 + i * 8;
        rec[at..at + 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    for (i, v) in local_position_m.into_iter().enumerate() {
        let at = 48 + i * 8;
        rec[at..at + 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    rec[72..80].copy_from_slice(&record_tick.to_le_bytes());
    rec[80..88].copy_from_slice(&carrier_hz.to_bits().to_le_bytes());
    // 88..96 coherence_phase_rad = 0.0 (all zero bytes)
    for (i, v) in stored_poynting_w_per_m2.into_iter().enumerate() {
        let at = 96 + i * 8;
        rec[at..at + 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    // flags: bit0 (carrier_hz) | bit2 (stored_poynting) | bit3 (confidence) = 13
    rec[120..124].copy_from_slice(&13u32.to_le_bytes());
    rec[124..128].copy_from_slice(&confidence.to_bits().to_le_bytes());
    rec
}

/// Encode one 128-byte Mode `0x0101` (`polarimetric_ray_v1`) record.
pub fn encode_polarimetric_ray_record(
    position_m: [f64; 3],
    direction: [f64; 3],
    stokes_i_q_u_v: [f64; 4],
    centre_frequency_hz: f64,
    bandwidth_hz: f64,
    geometric_path_m: f64,
    divergence_rad: f64,
) -> [u8; 128] {
    let mut rec = [0u8; 128];
    for (i, v) in position_m.into_iter().enumerate() {
        rec[i * 8..(i + 1) * 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    for (i, v) in direction.into_iter().enumerate() {
        let at = 24 + i * 8;
        rec[at..at + 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    for (i, v) in stokes_i_q_u_v.into_iter().enumerate() {
        let at = 48 + i * 8;
        rec[at..at + 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    rec[80..88].copy_from_slice(&centre_frequency_hz.to_bits().to_le_bytes());
    rec[88..96].copy_from_slice(&bandwidth_hz.to_bits().to_le_bytes());
    rec[96..104].copy_from_slice(&geometric_path_m.to_bits().to_le_bytes());
    // 104..112 phase_rad = 0.0 when flags == 0
    rec[112..120].copy_from_slice(&divergence_rad.to_bits().to_le_bytes());
    // 120..124 material_entry = 0, 124..128 flags = 0
    rec
}

/// Compute the 64-byte aligned stride for Mode `0x0103` (`sampled_spectrum_v1`) with `bin_count` f64 bins.
pub const fn sampled_spectrum_stride(bin_count: usize) -> usize {
    64 + ((bin_count * 8 + 63) & !63)
}

/// Encode one Mode `0x0103` (`sampled_spectrum_v1`) record with explicit trailing zero padding.
pub fn encode_sampled_spectrum_record(
    position_m: [f64; 3],
    record_tick: i64,
    spectral_axis_metadata_id: u32,
    temperature_k: f64,
    spectral_bins: &[f64],
) -> Vec<u8> {
    let stride = sampled_spectrum_stride(spectral_bins.len());
    let mut rec = vec![0u8; stride];
    for (i, v) in position_m.into_iter().enumerate() {
        rec[i * 8..(i + 1) * 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    rec[24..32].copy_from_slice(&record_tick.to_le_bytes());
    // 32..36 material_entry = 0
    rec[36..40].copy_from_slice(&spectral_axis_metadata_id.to_le_bytes());
    rec[40..48].copy_from_slice(&temperature_k.to_bits().to_le_bytes());
    // flags = 1 (temperature_k valid)
    rec[48..52].copy_from_slice(&1u32.to_le_bytes());
    // 52..64 reserved zero
    for (i, &bin) in spectral_bins.iter().enumerate() {
        let at = 64 + i * 8;
        rec[at..at + 8].copy_from_slice(&bin.to_bits().to_le_bytes());
    }
    rec
}

/// Encode one 128-byte Mode `0x0107` (`bioelectric_dipole_v1`) record with sealed CRC-32C at `124..128`.
pub fn encode_bioelectric_dipole_record(
    position_m: [f64; 3],
    current_dipole_moment_a_m: [f64; 3],
    extracellular_potential_v: f64,
    tick: i64,
    carrier_hz: f64,
    uncertainty: f64,
    anatomy_metadata_ref: u32,
    model_ref: u32,
    calibration_ref: u32,
    consent_provenance_ref: u32,
) -> [u8; 128] {
    let mut rec = [0u8; 128];
    for (i, v) in position_m.into_iter().enumerate() {
        rec[i * 8..(i + 1) * 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    for (i, v) in current_dipole_moment_a_m.into_iter().enumerate() {
        let at = 24 + i * 8;
        rec[at..at + 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    rec[48..56].copy_from_slice(&extracellular_potential_v.to_bits().to_le_bytes());
    rec[56..64].copy_from_slice(&tick.to_le_bytes());
    rec[64..72].copy_from_slice(&carrier_hz.to_bits().to_le_bytes());
    rec[72..80].copy_from_slice(&uncertainty.to_bits().to_le_bytes());
    rec[80..84].copy_from_slice(&anatomy_metadata_ref.to_le_bytes());
    rec[84..88].copy_from_slice(&model_ref.to_le_bytes());
    rec[88..92].copy_from_slice(&calibration_ref.to_le_bytes());
    rec[92..96].copy_from_slice(&consent_provenance_ref.to_le_bytes());
    // flags: bit0 (extracellular_potential_v) | bit1 (carrier_hz) | bit2 (uncertainty) = 7
    rec[96..100].copy_from_slice(&7u32.to_le_bytes());
    // 100..124 reserved zero
    let crc = crc32c(&rec[0..124]);
    rec[124..128].copy_from_slice(&crc.to_le_bytes());
    rec
}

/// Produce deterministic canonical EMF v4 (`0x0100, 0x0101, 0x0103, 0x0107`) and companion
/// `emf_profiles` (`0, 1, 3`) containers from an articulated vertebrate anatomical rig.
pub fn produce_vertebrate_emf_v4(
    rig: &VertebrateAnatomicalRig,
    corpus: &SpeciesCorpus,
    key_prefix: &str,
) -> Result<EmfV4ProducedBundle, String> {
    rig.validate(corpus)?;

    let mut eb_payload = Vec::with_capacity(rig.segments.len() * 128);
    let mut ray_payload = Vec::with_capacity(rig.segments.len() * 128);
    let spec_stride = sampled_spectrum_stride(rig.spectral_wavelengths_m.len());
    let mut spec_payload = Vec::with_capacity(rig.segments.len() * spec_stride);

    for (idx, seg) in rig.segments.iter().enumerate() {
        let tick = idx as i64;
        let pos = seg.midpoint_m();
        let axis = seg.unit_axis()?;
        let rad = planck_spectral_radiance_w_sr_m3(10.8e-6, seg.metabolic_temperature_k)?;
        let irradiance = (rad * 1e-6 * 0.92).max(1e-6);
        let e_mag = (irradiance * VACUUM_IMPEDANCE_OHM).sqrt();
        let b_mag = e_mag / VACUUM_C_M_PER_S;
        let electric = [
            axis[0] * e_mag * (0.65 + 0.35 * seg.muscular_activation_01),
            axis[1] * e_mag * 0.25,
            axis[2] * e_mag * (0.75 + 0.25 * seg.muscular_activation_01),
        ];
        let magnetic = [-axis[2] * b_mag, axis[0] * b_mag * 0.5, axis[1] * b_mag];
        let poynting = [
            axis[0] * irradiance,
            axis[1] * irradiance,
            axis[2] * irradiance,
        ];
        let carrier_hz = VACUUM_C_M_PER_S / 10.8e-6;
        eb_payload.extend_from_slice(&encode_instantaneous_e_b_record(
            electric,
            magnetic,
            pos,
            tick,
            carrier_hz,
            poynting,
            0.985,
        ));

        let mean_refl = seg.dermal_reflectance_bins.iter().sum::<f64>()
            / (seg.dermal_reflectance_bins.len() as f64);
        let stokes_i = (irradiance * (0.5 + mean_refl)).max(1e-4);
        let stokes = [stokes_i, stokes_i * 0.18, stokes_i * -0.09, stokes_i * 0.04];
        ray_payload.extend_from_slice(&encode_polarimetric_ray_record(
            pos,
            axis,
            stokes,
            VACUUM_C_M_PER_S / 700.0e-9,
            1.2e14,
            seg.length_m(),
            0.085,
        ));

        spec_payload.extend_from_slice(&encode_sampled_spectrum_record(
            pos,
            tick,
            6,
            seg.metabolic_temperature_k,
            &seg.dermal_reflectance_bins,
        ));
    }

    let mut bio_payload = Vec::with_capacity(rig.dipoles.len() * 128);
    for (idx, dip) in rig.dipoles.iter().enumerate() {
        let seg = &rig.segments[dip.segment_index];
        let mid = seg.midpoint_m();
        let pos = [
            mid[0] + dip.local_offset_m[0],
            mid[1] + dip.local_offset_m[1],
            mid[2] + dip.local_offset_m[2],
        ];
        bio_payload.extend_from_slice(&encode_bioelectric_dipole_record(
            pos,
            dip.current_dipole_moment_a_m,
            dip.extracellular_potential_v,
            idx as i64,
            dip.carrier_hz,
            dip.uncertainty_a_m,
            9, // Schema 137 BioContext block ID
            8, // Schema 134 ModelEvidence block ID
            5, // Schema 107 Calibration block ID
            4, // Schema 4 Consent Provenance block ID
        ));
    }

    let primary_ontology = rig.dipoles[0].anatomy_ontology.as_str();
    let primary_iri = rig.dipoles[0].anatomy_identifier.as_str();
    let obs_type = rig.dipoles[0].observation_type.as_str();
    let authority_digest = *blake3::hash(
        format!(
            "QUALIA-BIO-AUTHORITY-v1:{}:{}:{}",
            corpus.species_id, rig.rig_id, primary_iri
        )
        .as_bytes(),
    )
    .as_bytes();

    let canonical_bytes = assemble_emf_v4_container(
        rig.epoch_seconds,
        &rig.spectral_wavelengths_m,
        &eb_payload,
        rig.segments.len() as u64,
        &ray_payload,
        rig.segments.len() as u64,
        &spec_payload,
        rig.segments.len() as u64,
        Some((&bio_payload, rig.dipoles.len() as u64)),
        &authority_digest,
        primary_ontology,
        primary_iri,
        obs_type,
        &corpus.species_id,
        &rig.rig_id,
        true,
    )?;

    let profile_bytes = assemble_emf_v4_container(
        rig.epoch_seconds,
        &rig.spectral_wavelengths_m,
        &eb_payload,
        rig.segments.len() as u64,
        &ray_payload,
        rig.segments.len() as u64,
        &spec_payload,
        rig.segments.len() as u64,
        None,
        &authority_digest,
        primary_ontology,
        primary_iri,
        obs_type,
        &corpus.species_id,
        &rig.rig_id,
        false,
    )?;

    let mut mode_record_counts = BTreeMap::new();
    mode_record_counts.insert(MODE_INSTANTANEOUS_E_B_V1, rig.segments.len() as u64);
    mode_record_counts.insert(MODE_POLARIMETRIC_RAY_V1, rig.segments.len() as u64);
    mode_record_counts.insert(MODE_SAMPLED_SPECTRUM_V1, rig.segments.len() as u64);
    mode_record_counts.insert(MODE_BIOELECTRIC_DIPOLE_V1, rig.dipoles.len() as u64);

    Ok(EmfV4ProducedBundle {
        canonical_sha256_hex: sha256_hex(&canonical_bytes),
        canonical_blake3_hex: blake3::hash(&canonical_bytes).to_hex().to_string(),
        canonical_resource: Resource {
            key: format!("{key_prefix}_canonical_v4.emf"),
            kind: "emf".to_string(),
            bytes: canonical_bytes,
        },
        profile_sha256_hex: sha256_hex(&profile_bytes),
        profile_blake3_hex: blake3::hash(&profile_bytes).to_hex().to_string(),
        profile_resource: Resource {
            key: format!("{key_prefix}_profiles_v4.emf"),
            kind: "emf".to_string(),
            bytes: profile_bytes,
        },
        mode_record_counts,
        spectral_wavelengths_m: rig.spectral_wavelengths_m.clone(),
        species_id: corpus.species_id.clone(),
        primary_anatomy_iri: primary_iri.to_string(),
    })
}

/// Produce deterministic canonical EMF v4 (`0x0100, 0x0101, 0x0103, 0x0107`) and companion
/// `emf_profiles` (`0, 1, 3`) containers from a botanical FSPM metamer rig.
pub fn produce_botanical_emf_v4(
    rig: &BotanicalMetamerRig,
    corpus: &SpeciesCorpus,
    key_prefix: &str,
) -> Result<EmfV4ProducedBundle, String> {
    rig.validate(corpus)?;

    let count = rig.metamers.len();
    let spec_stride = sampled_spectrum_stride(rig.spectral_wavelengths_m.len());
    let mut eb_payload = Vec::with_capacity(count * 128);
    let mut ray_payload = Vec::with_capacity(count * 128);
    let mut spec_payload = Vec::with_capacity(count * spec_stride);
    let mut bio_payload = Vec::with_capacity(count * 128);

    for (idx, meta) in rig.metamers.iter().enumerate() {
        let tick = idx as i64;
        let pos = meta.midpoint_m();
        let axis = meta.unit_axis()?;
        let rad = planck_spectral_radiance_w_sr_m3(9.8e-6, meta.leaf_temperature_k)?;
        let irradiance = (rad * 1e-6 * 0.96).max(1e-6);
        let e_mag = (irradiance * VACUUM_IMPEDANCE_OHM).sqrt();
        let b_mag = e_mag / VACUUM_C_M_PER_S;
        let electric = [axis[0] * e_mag, axis[1] * e_mag * 0.4, axis[2] * e_mag];
        let magnetic = [-axis[1] * b_mag, axis[0] * b_mag, 0.0];
        let poynting = [
            axis[0] * irradiance,
            axis[1] * irradiance,
            axis[2] * irradiance,
        ];
        eb_payload.extend_from_slice(&encode_instantaneous_e_b_record(
            electric,
            magnetic,
            pos,
            tick,
            VACUUM_C_M_PER_S / 9.8e-6,
            poynting,
            0.99,
        ));

        let nir_refl = meta.red_edge_reflectance_bins.last().copied().unwrap_or(0.5);
        let stokes_i = (irradiance * (0.4 + nir_refl)).max(1e-4);
        let stokes = [stokes_i, stokes_i * 0.14, stokes_i * 0.06, stokes_i * 0.02];
        ray_payload.extend_from_slice(&encode_polarimetric_ray_record(
            pos,
            axis,
            stokes,
            VACUUM_C_M_PER_S / 715.0e-9,
            8.5e13,
            meta.length_m(),
            0.065,
        ));

        spec_payload.extend_from_slice(&encode_sampled_spectrum_record(
            pos,
            tick,
            6,
            meta.leaf_temperature_k,
            &meta.red_edge_reflectance_bins,
        ));

        // Phloem electrotonic variation potential dipole moment proportional to phloem area * length
        let dipole_scale = (meta.phloem_radius_m * meta.phloem_radius_m * meta.length_m()).max(1e-9);
        let phloem_dipole = [
            axis[0] * dipole_scale * 2.5e-2,
            axis[1] * dipole_scale * 2.5e-2,
            axis[2] * dipole_scale * 2.5e-2,
        ];
        let phloem_potential_v = -0.065 + 0.008 * (idx as f64);
        bio_payload.extend_from_slice(&encode_bioelectric_dipole_record(
            pos,
            phloem_dipole,
            phloem_potential_v,
            tick,
            1.25,
            1.5e-11,
            9,
            8,
            5,
            4,
        ));
    }

    let primary_iri = rig.metamers[0].plant_ontology_iri.as_str();
    let authority_digest = *blake3::hash(
        format!(
            "QUALIA-BIO-AUTHORITY-v1:{}:{}:{}",
            corpus.species_id, rig.plant_id, primary_iri
        )
        .as_bytes(),
    )
    .as_bytes();

    let canonical_bytes = assemble_emf_v4_container(
        rig.epoch_seconds,
        &rig.spectral_wavelengths_m,
        &eb_payload,
        count as u64,
        &ray_payload,
        count as u64,
        &spec_payload,
        count as u64,
        Some((&bio_payload, count as u64)),
        &authority_digest,
        "PO",
        primary_iri,
        "current_dipole",
        &corpus.species_id,
        &rig.plant_id,
        true,
    )?;

    let profile_bytes = assemble_emf_v4_container(
        rig.epoch_seconds,
        &rig.spectral_wavelengths_m,
        &eb_payload,
        count as u64,
        &ray_payload,
        count as u64,
        &spec_payload,
        count as u64,
        None,
        &authority_digest,
        "PO",
        primary_iri,
        "current_dipole",
        &corpus.species_id,
        &rig.plant_id,
        false,
    )?;

    let mut mode_record_counts = BTreeMap::new();
    mode_record_counts.insert(MODE_INSTANTANEOUS_E_B_V1, count as u64);
    mode_record_counts.insert(MODE_POLARIMETRIC_RAY_V1, count as u64);
    mode_record_counts.insert(MODE_SAMPLED_SPECTRUM_V1, count as u64);
    mode_record_counts.insert(MODE_BIOELECTRIC_DIPOLE_V1, count as u64);

    Ok(EmfV4ProducedBundle {
        canonical_sha256_hex: sha256_hex(&canonical_bytes),
        canonical_blake3_hex: blake3::hash(&canonical_bytes).to_hex().to_string(),
        canonical_resource: Resource {
            key: format!("{key_prefix}_canonical_v4.emf"),
            kind: "emf".to_string(),
            bytes: canonical_bytes,
        },
        profile_sha256_hex: sha256_hex(&profile_bytes),
        profile_blake3_hex: blake3::hash(&profile_bytes).to_hex().to_string(),
        profile_resource: Resource {
            key: format!("{key_prefix}_profiles_v4.emf"),
            kind: "emf".to_string(),
            bytes: profile_bytes,
        },
        mode_record_counts,
        spectral_wavelengths_m: rig.spectral_wavelengths_m.clone(),
        species_id: corpus.species_id.clone(),
        primary_anatomy_iri: primary_iri.to_string(),
    })
}

pub use crate::emf_qac_anatomy_bridge::emf_v4_envelope::sha256_hex;
use crate::emf_qac_anatomy_bridge::emf_v4_envelope::assemble_emf_v4_container;

