use qualia_core_db::q42::emf_profiles::{Limits as ProfileLimits, ProfileChunk, ProfileReader};
use qualia_core_db::q42::field_envelope::{Family, Limits as StorageLimits, StructuralView};
use qualia_core_db::q42::field_semantics::{field, NodeState, Scratch};
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
}
pub(crate) struct CurrentDocument<'a> {
    pub view: StructuralView<'a>,
    pub chunks: Vec<ProfileChunk<'a>>,
    pub summary: Summary,
    pub shared_work_used: u64,
    pub payload_work_used: u64,
}
impl<'a> CurrentDocument<'a> {
    pub(crate) fn host_work_used(&self) -> Result<u64, String> {
        let records = self
            .summary
            .records
            .iter()
            .try_fold(0u64, |n, &v| n.checked_add(v))
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
        let mut scratch_metadata = vec![NodeState::default(); blocks];
        let mut scratch_entries = vec![NodeState::default(); entries];
        let mut scratch = Scratch {
            metadata: &mut scratch_metadata,
            entries: &mut scratch_entries,
        };
        let mut chunks = Vec::with_capacity(entries);
        let mut referenced_masks = vec![false; entries];
        let mut summary = Summary {
            max_e_magnitude: Some(0.0),
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
            let entry = view
                .entry(index)
                .map_err(|e| format!("EMF directory: {e:?}"))?;
            if entry.mode() == 0x106 {
                continue;
            }
            let chunk = reader.chunk(index).map_err(|e| {
                format!("Unsupported/invalid current EMF profile entry{index}: {e:?}")
            })?;
            let kind = match chunk.mode() {
                0 => 0,
                1 => 1,
                3 => 2,
                _ => return Err("Unsupported current EMF typed mode".into()),
            };
            summary.records[kind] = summary.records[kind]
                .checked_add(chunk.record_count())
                .ok_or("EMF record count overflow")?;
            for i in 0..chunk.record_count() {
                let valid = match chunk.mode() {
                    0 => {
                        if let Some(record) = chunk
                            .field(i)
                            .map_err(|e| format!("EMF field index: {e:?}"))?
                        {
                            let e = record.electric_v_per_m();
                            for v in e {
                                summary.max_abs_e_component =
                                    summary.max_abs_e_component.max(v.abs());
                            }
                            let magnitude = e[0].hypot(e[1]).hypot(e[2]);
                            if magnitude.is_finite() {
                                if let Some(max) = &mut summary.max_e_magnitude {
                                    *max = max.max(magnitude);
                                }
                            } else {
                                summary.max_e_magnitude = None;
                            }
                            true
                        } else {
                            false
                        }
                    }
                    1 => chunk
                        .ray(i)
                        .map_err(|e| format!("EMF ray index: {e:?}"))?
                        .is_some(),
                    3 => chunk
                        .spectrum(i)
                        .map_err(|e| format!("EMF spectrum index: {e:?}"))?
                        .is_some(),
                    _ => unreachable!(),
                };
                if valid {
                    summary.valid_records[kind] += 1;
                }
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
            if view
                .entry(index)
                .map_err(|e| format!("EMF entry: {e:?}"))?
                .mode()
                == 0x106
            {
                if !referenced_masks[index as usize - 1] {
                    return Err(
                        "Unreferenced validity entry has no admitted physical target".into(),
                    );
                }
                view.verify_chunk(index)
                    .map_err(|e| format!("EMF mask digest: {e:?}"))?;
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
            shared_work_used,
            payload_work_used,
        })
    }
}
