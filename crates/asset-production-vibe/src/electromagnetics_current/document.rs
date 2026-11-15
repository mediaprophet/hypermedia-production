use crate::emf_qac_anatomy_bridge::EmfV4AnatomyRegistry;
use qualia_core_db::container_10d::crc32c::crc32c;
use qualia_core_db::q42::emf_profiles::{
    DOSE_PROFILE, Limits as ProfileLimits, ProfileChunk, ProfileReader,
};
use qualia_core_db::q42::field_envelope::{Family, Limits as StorageLimits, StructuralView};
use qualia_core_db::q42::field_semantics::{
    admit_shared, field, Limits as SemanticLimits, NodeState, Scratch,
};
use qualia_core_db::q42::field_spectrum::{AxisKind, AxisView, SigmaInterpretation};
use std::mem::size_of;
#[derive(Clone, Copy)]
pub(crate) struct ReadLimits {
    pub file_bytes: usize,
    pub cold_work: u64,
    pub cache_bytes: usize,
}
pub(crate) fn read_memory_bound(
    file_bytes: usize,
    metadata_bytes: usize,
    entries: usize,
) -> Result<usize, String> {
    entries
        .checked_mul(size_of::<ProfileChunk<'_>>() * 3 + size_of::<NodeState>())
        .and_then(|n| n.checked_add(4096 * size_of::<NodeState>()))
        .and_then(|n| n.checked_add(metadata_bytes.checked_mul(12)?))
        .and_then(|n| n.checked_add(file_bytes))
        .ok_or_else(|| "EMF read memory overflow".into())
}
/// Conservative logical cold budget, not a measured cycle or monetary estimate.
/// Shared graph/schema admission runs once per document; the nonforgeable core
/// reader charges selected binding/payload work and chunks are cached for hot use.
pub(crate) fn read_work_bound(
    metadata_bytes: usize,
    blocks: usize,
    entries: usize,
) -> Result<u64, String> {
    let per = (metadata_bytes as u64)
        .checked_mul(64)
        .and_then(|n| n.checked_add((blocks as u64).checked_mul(blocks as u64)?.checked_mul(8)?))
        .and_then(|n| {
            n.checked_add(
                (entries as u64)
                    .checked_mul(entries as u64 + blocks as u64)?
                    .checked_mul(8)?,
            )
        })
        .and_then(|n| n.checked_add(4096))
        .ok_or("EMF cold work overflow")?;
    per.checked_add(
        (entries as u64)
            .checked_mul(65536)
            .ok_or("EMF cold work overflow")?,
    )
    .ok_or_else(|| "EMF cold work overflow".into())
}
/// Upper bound for the host charge before admission: selected payload work is
/// at most 3 * file bytes, and every physical record occupies at least one byte.
/// Therefore payload_work/16 + 1 + records <= 2 * file bytes + 1024.
pub(crate) fn host_work_bound(shared_work: u64, file_bytes: usize) -> Result<u64, String> {
    shared_work
        .checked_add(
            (file_bytes as u64)
                .checked_mul(2)
                .ok_or("EMF host work overflow")?,
        )
        .and_then(|n| n.checked_add(1024))
        .ok_or_else(|| "EMF host work overflow".into())
}
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Summary {
    pub records: [u64; 3],
    pub valid_records: [u64; 3],
    pub max_abs_e_component: f64,
    pub max_e_magnitude: Option<f64>,
    pub dipole_records: usize,
    pub radiative_transfer_records: usize,
    pub sh_coeff_records: usize,
    pub constitutive_records: usize,
    pub validity_mask_records: usize,
    pub format_profile: &'static str,
}
pub(crate) struct CurrentDocument<'a> {
    pub view: StructuralView<'a>,
    pub chunks: Vec<ProfileChunk<'a>>,
    pub summary: Summary,
    pub wire_freeze_e_samples: Vec<[f64; 3]>,
    pub representative_wavelength_m: Option<f64>,
    pub representative_sigma: Option<f64>,
    pub shared_work_used: u64,
    pub payload_work_used: u64,
}
fn is_validity_mask_entry(view: &StructuralView<'_>, entry_index: u32) -> bool {
    let Ok(entry) = view.entry(entry_index) else {
        return false;
    };
    if entry.mode() != 0x0106 {
        return false;
    }
    let Some(desc) = view
        .blocks()
        .find(|b| b.id() == entry.descriptor_id() && b.schema_type() == 3)
    else {
        return true;
    };
    let Some(ext_ref) = field(desc, 18)
        .and_then(|f| f.value().try_into().ok())
        .map(u32::from_le_bytes)
    else {
        return true;
    };
    let Some(ext) = view
        .blocks()
        .find(|b| b.id() == ext_ref && b.schema_type() == 103)
    else {
        return true;
    };
    field(ext, 6).is_none_or(|f| f.value() != DOSE_PROFILE.as_bytes())
}
fn update_electric_summary(summary: &mut Summary, e: [f64; 3]) {
    for v in e {
        summary.max_abs_e_component = summary.max_abs_e_component.max(v.abs());
    }
    let magnitude = e[0].hypot(e[1]).hypot(e[2]);
    if magnitude.is_finite() {
        if let Some(max) = &mut summary.max_e_magnitude {
            *max = max.max(magnitude);
        }
    } else {
        summary.max_e_magnitude = None;
    }
}
fn extract_spectral_representative(axis: AxisView<'_>) -> (Option<f64>, Option<f64>) {
    let count = axis.coordinate_count();
    if count == 0 {
        return (None, None);
    }
    let mid = count / 2;
    let Ok(lambda_m) = axis.ascending_derived_coordinate(AxisKind::VacuumWavelengthMetres, mid)
    else {
        return (None, None);
    };
    if !lambda_m.is_finite() || lambda_m <= 0.0 {
        return (None, None);
    }
    let sigma = match axis.sigma() {
        SigmaInterpretation::LegacyDisplay400700nm => {
            Some(((lambda_m * 1e9 - 400.0) / 300.0).clamp(0.0, 1.0))
        }
        SigmaInterpretation::LegacyEtalonDisplay380750nm => {
            Some(((lambda_m * 1e9 - 380.0) / 370.0).clamp(0.0, 1.0))
        }
        SigmaInterpretation::LogFrequency { .. } => axis
            .ascending_derived_coordinate(AxisKind::FrequencyHz, mid)
            .ok()
            .and_then(|freq_hz| axis.log_frequency_sigma(freq_hz).ok()),
        SigmaInterpretation::None => None,
    };
    (Some(lambda_m), sigma)
}
impl<'a> CurrentDocument<'a> {
    pub(crate) fn host_work_used(&self) -> Result<u64, String> {
        let extra = self
            .summary
            .dipole_records
            .checked_add(self.summary.radiative_transfer_records)
            .and_then(|n| n.checked_add(self.summary.sh_coeff_records))
            .and_then(|n| n.checked_add(self.summary.constitutive_records))
            .and_then(|n| n.checked_add(self.summary.validity_mask_records))
            .and_then(|n| u64::try_from(n).ok())
            .ok_or("EMF record work overflow")?;
        let records = self
            .summary
            .records
            .iter()
            .try_fold(0u64, |n, &v| n.checked_add(v))
            .and_then(|n| n.checked_add(extra))
            .ok_or("EMF record work overflow")?;
        self.shared_work_used
            .checked_add(self.payload_work_used / 16 + 1)
            .and_then(|n| n.checked_add(records))
            .ok_or_else(|| "EMF read charge overflow".into())
    }
    pub fn open(bytes: &'a [u8], limits: ReadLimits) -> Result<Self, String> {
        let mut storage = StorageLimits::default();
        storage.file_bytes = limits.file_bytes as u64;
        storage.aggregate_decoded_bytes = limits.file_bytes as u64;
        let view = StructuralView::open(bytes, Family::EmfV4Candidate, storage)
            .map_err(|e| format!("Invalid current EMF envelope: {e:?}"))?;
        let blocks = view.blocks().count();
        let entries = view.entry_count() as usize;
        let metadata = view.metadata_bytes().len();
        let memory = read_memory_bound(0, metadata, entries)?;
        if memory > limits.cache_bytes {
            return Err("EMF admission cache/report exceeds selected memory budget".into());
        }
        let work = read_work_bound(metadata, blocks, entries)?;
        if work > limits.cold_work {
            return Err("EMF cold typed admission exceeds selected work budget".into());
        }
        let mut has_wire_freeze = false;
        for index in 1..=view.entry_count() {
            let entry = view
                .entry(index)
                .map_err(|e| format!("EMF directory: {e:?}"))?;
            if (0x0100..=0x0107).contains(&entry.mode()) && !is_validity_mask_entry(&view, index) {
                has_wire_freeze = true;
                break;
            }
        }
        if has_wire_freeze {
            let mut anatomy_meta = vec![NodeState::default(); blocks];
            let mut anatomy_entries = vec![NodeState::default(); entries];
            let mut anatomy_scratch = Scratch {
                metadata: &mut anatomy_meta,
                entries: &mut anatomy_entries,
            };
            let mut semantic_limits = SemanticLimits::default();
            semantic_limits.work = work;
            admit_shared(
                &view,
                &EmfV4AnatomyRegistry,
                &mut anatomy_scratch,
                semantic_limits,
            )
            .map_err(|e| format!("Invalid current EMF wire-freeze semantics: {e:?}"))?;
        }
        let mut scratch_metadata = vec![NodeState::default(); blocks];
        let mut scratch_entries = vec![NodeState::default(); entries];
        let mut scratch = Scratch {
            metadata: &mut scratch_metadata,
            entries: &mut scratch_entries,
        };
        let mut chunks = Vec::with_capacity(entries);
        let mut referenced_masks = vec![false; entries];
        let mut wire_freeze_e_samples = Vec::new();
        let mut representative_wavelength_m = None;
        let mut representative_sigma = None;
        let mut summary = Summary {
            max_e_magnitude: Some(0.0),
            format_profile: if has_wire_freeze {
                "wire_freeze_v4"
            } else {
                "profile_v4"
            },
            ..Summary::default()
        };
        let mut profile = ProfileLimits::default();
        profile.shared.work = work;
        profile.payload_work = (limits.file_bytes as u64)
            .checked_mul(2)
            .ok_or("EMF payload work overflow")?;
        let payload_budget = (limits.file_bytes as u64)
            .checked_mul(3)
            .ok_or("EMF payload work overflow")?;
        let mut reader = ProfileReader::open(&view, &mut scratch, profile, payload_budget)
            .map_err(|e| format!("Invalid current EMF shared metadata: {e:?}"))?;
        for index in 1..=view.entry_count() {
            if is_validity_mask_entry(&view, index) {
                continue;
            }
            let entry = view
                .entry(index)
                .map_err(|e| format!("EMF directory: {e:?}"))?;
            // Verify 0x0107 whole-chunk BLAKE3 + per-record CRC-32C before ProfileReader
            // so trailer/payload corruption yields an explicit CRC-32C diagnostic.
            if entry.mode() == 0x0107 {
                let raw = view
                    .verify_chunk(index)
                    .map_err(|e| format!("EMF dipole digest: {e:?}"))?;
                let expected_bytes = usize::try_from(entry.record_count())
                    .ok()
                    .and_then(|c| c.checked_mul(128))
                    .ok_or("EMF dipole payload size overflow")?;
                if raw.len() != expected_bytes {
                    return Err("EMF bioelectric_dipole_v1 byte length mismatch".into());
                }
                for (rec_idx, rec) in raw.chunks_exact(128).enumerate() {
                    let expected_crc = crc32c(&rec[..124]);
                    let stored_crc = u32::from_le_bytes(rec[124..128].try_into().unwrap());
                    if expected_crc != stored_crc {
                        return Err(format!(
                            "EMF bioelectric_dipole_v1 record {rec_idx} CRC-32C mismatch"
                        ));
                    }
                }
            }
            let chunk = reader.chunk(index).map_err(|e| {
                format!("Unsupported/invalid current EMF profile entry{index}: {e:?}")
            })?;
            match chunk.canonical_mode() {
                0x0100 => {
                    summary.records[0] = summary.records[0]
                        .checked_add(chunk.record_count())
                        .ok_or("EMF record count overflow")?;
                    let is_point_stream = chunk.brick_layout().is_none();
                    for i in 0..chunk.record_count() {
                        if let Some(record) = chunk
                            .field(i)
                            .map_err(|e| format!("EMF field index: {e:?}"))?
                        {
                            let e = record.electric_v_per_m();
                            update_electric_summary(&mut summary, e);
                            if is_point_stream {
                                if e.iter().any(|v| !v.is_finite()) {
                                    return Err("EMF point-stream E field is not finite".into());
                                }
                                wire_freeze_e_samples.push(e);
                            }
                            summary.valid_records[0] += 1;
                        }
                    }
                }
                0x0101 => {
                    summary.records[1] = summary.records[1]
                        .checked_add(chunk.record_count())
                        .ok_or("EMF record count overflow")?;
                    for i in 0..chunk.record_count() {
                        if chunk
                            .ray(i)
                            .map_err(|e| format!("EMF ray index: {e:?}"))?
                            .is_some()
                        {
                            summary.valid_records[1] += 1;
                        }
                    }
                }
                0x0102 => {
                    let count = usize::try_from(chunk.record_count())
                        .map_err(|_| "EMF radiative transfer record count overflow")?;
                    let is_point_stream = chunk.brick_layout().is_none();
                    for i in 0..chunk.record_count() {
                        if let Some(yee) = chunk
                            .yee(i)
                            .map_err(|e| format!("EMF radiative transfer index: {e:?}"))?
                        {
                            if is_point_stream
                                && yee.electric_field_v_per_m.iter().all(|v| v.is_finite())
                            {
                                wire_freeze_e_samples.push(yee.electric_field_v_per_m);
                            }
                        }
                    }
                    summary.radiative_transfer_records = summary
                        .radiative_transfer_records
                        .checked_add(count)
                        .ok_or("EMF radiative transfer record count overflow")?;
                }
                0x0103 => {
                    summary.records[2] = summary.records[2]
                        .checked_add(chunk.record_count())
                        .ok_or("EMF record count overflow")?;
                    if representative_wavelength_m.is_none() {
                        if let Some(axis) = chunk.spectral_axis() {
                            let (lambda_m, sigma) = extract_spectral_representative(axis);
                            representative_wavelength_m = lambda_m;
                            if representative_sigma.is_none() {
                                representative_sigma = sigma;
                            }
                        }
                    }
                    for i in 0..chunk.record_count() {
                        if chunk
                            .spectrum(i)
                            .map_err(|e| format!("EMF spectrum index: {e:?}"))?
                            .is_some()
                        {
                            summary.valid_records[2] += 1;
                        }
                    }
                }
                0x0104 => {
                    let count = usize::try_from(chunk.record_count())
                        .map_err(|_| "EMF SH coeff record count overflow")?;
                    for i in 0..chunk.record_count() {
                        chunk
                            .mueller(i)
                            .map_err(|e| format!("EMF SH coeff index: {e:?}"))?;
                    }
                    summary.sh_coeff_records = summary
                        .sh_coeff_records
                        .checked_add(count)
                        .ok_or("EMF SH coeff record count overflow")?;
                }
                0x0105 => {
                    let count = usize::try_from(chunk.record_count())
                        .map_err(|_| "EMF constitutive record count overflow")?;
                    for i in 0..chunk.record_count() {
                        chunk
                            .iq(i)
                            .map_err(|e| format!("EMF constitutive index: {e:?}"))?;
                    }
                    summary.constitutive_records = summary
                        .constitutive_records
                        .checked_add(count)
                        .ok_or("EMF constitutive record count overflow")?;
                }
                0x0106 => {
                    let count = usize::try_from(chunk.record_count())
                        .map_err(|_| "EMF validity/dose record count overflow")?;
                    for i in 0..chunk.record_count() {
                        chunk
                            .dose(i)
                            .map_err(|e| format!("EMF dose/validity index: {e:?}"))?;
                    }
                    summary.validity_mask_records = summary
                        .validity_mask_records
                        .checked_add(count)
                        .ok_or("EMF validity mask record count overflow")?;
                }
                0x0107 => {
                    let count = usize::try_from(chunk.record_count())
                        .map_err(|_| "EMF dipole record count overflow")?;
                    for i in 0..chunk.record_count() {
                        if let Some(bio) = chunk
                            .biofield(i)
                            .map_err(|e| format!("EMF biofield index: {e:?}"))?
                        {
                            if bio.current_dipole_moment_a_m.iter().any(|v| !v.is_finite()) {
                                return Err("EMF bioelectric dipole moment is not finite".into());
                            }
                            wire_freeze_e_samples.push(bio.current_dipole_moment_a_m);
                        }
                    }
                    summary.dipole_records = summary
                        .dipole_records
                        .checked_add(count)
                        .ok_or("EMF dipole record count overflow")?;
                }
                _ => return Err("Unsupported current EMF typed mode".into()),
            }
            let descriptor = view
                .blocks()
                .find(|b| b.id() == chunk.descriptor_id())
                .ok_or("EMF descriptor missing after admission")?;
            if let Some(mask) = field(descriptor, 16) {
                let index = u32::from_le_bytes(mask.value().try_into().unwrap());
                referenced_masks[index as usize - 1] = true;
            }
            chunks.push(chunk);
        }
        for index in 1..=view.entry_count() {
            if is_validity_mask_entry(&view, index) {
                if !referenced_masks[index as usize - 1] {
                    return Err(
                        "Unreferenced validity entry has no admitted physical target".into(),
                    );
                }
                let mask = view
                    .verify_chunk(index)
                    .map_err(|e| format!("EMF mask digest: {e:?}"))?;
                summary.validity_mask_records = summary
                    .validity_mask_records
                    .checked_add(mask.len())
                    .ok_or("EMF validity mask record count overflow")?;
            }
        }
        if summary.valid_records[0] == 0 {
            summary.max_e_magnitude = None;
        }
        let shared_work_used = work - reader.remaining_shared_work();
        let payload_work_used = payload_budget - reader.remaining_payload_work();
        drop(reader);
        Ok(Self {
            view,
            chunks,
            summary,
            wire_freeze_e_samples,
            representative_wavelength_m,
            representative_sigma,
            shared_work_used,
            payload_work_used,
        })
    }
}
