use super::{render_slice, CurrentDocument, GridAtTime, ProjectionLimits, ReadLimits};
use crate::*;
use qualia_core_db::q42::field_envelope::{Family, Limits as StorageLimits, StructuralView};
use qualia_core_db::q42::field_semantics::field;
pub(crate) struct ReadPlan {
    pub memory: usize,
    pub cache: usize,
    pub shared_work: u64,
}
pub(crate) fn time_slice(args: &Args) -> Result<u32, String> {
    u32::try_from(
        args.get("time_slice")
            .map(|v| integer(v, "time_slice"))
            .transpose()?
            .unwrap_or(0),
    )
    .map_err(|_| "time_slice exceeds u32".into())
}
impl AssetHost<'_> {
    pub(crate) fn current_read_plan(&self, value: &Value) -> Result<ReadPlan, String> {
        self.emf_peak_admit(0)?;
        let bytes = self.emf_source_bytes(value)?;
        let mut limits = StorageLimits::default();
        limits.file_bytes = self.limits.resource_bytes as u64;
        limits.aggregate_decoded_bytes = self.limits.resource_bytes as u64;
        let view = StructuralView::open(bytes, Family::EmfV4Candidate, limits)
            .map_err(|e| format!("Current EMF envelope rejected: {e:?}"))?;
        let entries = view.entry_count() as usize;
        let metadata = view.metadata_bytes().len();
        let blocks = view.blocks().count();
        let cache = super::read_memory_bound(0, metadata, entries)?;
        let memory = cache
            .checked_add(bytes.len())
            .ok_or("EMF memory bound overflow")?;
        let shared_work = super::read_work_bound(metadata, blocks, entries)?;
        let host_work = super::document::host_work_bound(shared_work, bytes.len())?;
        if host_work > self.limits.asset_work_units.saturating_sub(self.work) {
            return Err("Current EMF admission exceeds remaining work budget".into());
        }
        Ok(ReadPlan {
            memory,
            cache,
            shared_work,
        })
    }
    pub(crate) fn emf_read_required(&self, value: &Value) -> Result<usize, String> {
        Ok(self.current_read_plan(value)?.memory)
    }
    pub(crate) fn charge_current_read(
        &mut self,
        document: &CurrentDocument<'_>,
    ) -> Result<(), String> {
        let work = document.host_work_used()?;
        self.charge(
            usize::try_from(work).map_err(|_| "EMF read work exceeds usize")?,
            0,
        )
    }
    pub(crate) fn verify_emf_current(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["emf"])?;
        let value = required(args, "emf")?;
        let plan = self.current_read_plan(value)?;
        self.emf_peak_admit(plan.memory)?;
        let bytes = self.emf_source_bytes(value)?.to_vec();
        let document = CurrentDocument::open(
            &bytes,
            ReadLimits {
                file_bytes: self.limits.resource_bytes,
                cold_work: plan.shared_work,
                cache_bytes: plan.cache,
            },
        )?;
        self.charge_current_read(&document)?;
        let mut provenance = Vec::new();
        for block in document
            .view
            .blocks()
            .filter(|b| b.schema_type() == 4 && b.revision() == 1)
        {
            let mut item = BTreeMap::new();
            item.insert("metadata_id".into(), Value::U64(u64::from(block.id())));
            if let Some(role) = field(block, 1) {
                item.insert(
                    "role".into(),
                    Value::U64(u64::from(u32::from_le_bytes(
                        role.value().try_into().unwrap(),
                    ))),
                );
            }
            for (key, name) in [(3, "algorithm"), (4, "algorithm_version"), (7, "lineage")] {
                if let Some(value) = field(block, key) {
                    item.insert(
                        name.into(),
                        Value::String(
                            std::str::from_utf8(value.value())
                                .map_err(|_| "Admitted lineage UTF8 failed")?
                                .to_owned(),
                        ),
                    );
                }
            }
            provenance.push(Value::Record(item));
        }
        let clocks = document
            .view
            .blocks()
            .filter(|b| b.schema_type() == 1 && b.revision() == 1)
            .map(clock_value)
            .collect::<Result<Vec<_>, _>>()?;
        let frames = document
            .view
            .blocks()
            .filter(|b| b.schema_type() == 2 && b.revision() == 1)
            .map(frame_value)
            .collect::<Result<Vec<_>, _>>()?;
        let mut result = BTreeMap::new();
        result.insert("valid".into(), Value::Bool(true));
        result.insert("clocks".into(), Value::List(clocks));
        result.insert("frames".into(), Value::List(frames));
        result.insert(
            "index_digest_blake3".into(),
            Value::String(index_digest(document.view.index_digest())),
        );
        result.insert("container_version".into(), Value::U64(4));
        result.insert(
            "chunk_count".into(),
            Value::U64(u64::from(document.view.entry_count())),
        );
        result.insert(
            "physical_chunk_count".into(),
            Value::U64(document.chunks.len() as u64),
        );
        result.insert(
            "payload_integrity".into(),
            Value::String("whole_chunk_blake3_256".into()),
        );
        result.insert(
            "metadata_integrity".into(),
            Value::String("header_directory_crc32c_and_full_index_blake3".into()),
        );
        result.insert(
            "field_records".into(),
            Value::U64(document.summary.records[0]),
        );
        result.insert(
            "ray_records".into(),
            Value::U64(document.summary.records[1]),
        );
        result.insert(
            "spectrum_records".into(),
            Value::U64(document.summary.records[2]),
        );
        result.insert(
            "valid_records".into(),
            Value::List(
                document
                    .summary
                    .valid_records
                    .iter()
                    .map(|&n| Value::U64(n))
                    .collect(),
            ),
        );
        result.insert(
            "max_abs_e_component_v_per_m".into(),
            if document.summary.valid_records[0] > 0 {
                Value::F64(document.summary.max_abs_e_component)
            } else {
                Value::Null
            },
        );
        result.insert(
            "max_e_magnitude_v_per_m".into(),
            if document.summary.valid_records[0] > 0 {
                document
                    .summary
                    .max_e_magnitude
                    .map(Value::F64)
                    .unwrap_or(Value::Null)
            } else {
                Value::Null
            },
        );
        result.insert("energy_density_j_per_m3".into(), Value::Null);
        result.insert(
            "energy_scope".into(),
            Value::String(
                "not inferred: no constitutive model admitted by this profile subset".into(),
            ),
        );
        result.insert("provenance".into(), Value::List(provenance));
        Ok(Value::Record(result))
    }
    pub(crate) fn render_current_emf_slice(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "emf",
                "axis",
                "slice_index",
                "width",
                "height",
                "exposure",
                "time_slice",
            ],
        )?;
        let value = required(args, "emf")?;
        let axis = match args.get("axis").map(text).transpose()?.unwrap_or("z") {
            "x" => 0,
            "y" => 1,
            "z" => 2,
            _ => return Err("axis must be x,y,z".into()),
        };
        let slice = u32::try_from(
            args.get("slice_index")
                .map(|v| integer(v, "slice_index"))
                .transpose()?
                .unwrap_or(0),
        )
        .map_err(|_| "slice_index exceeds u32")?;
        let selected = time_slice(args)?;
        let width = count(args, "width", 128, 1024)?;
        let height = count(args, "height", 128, 1024)?;
        let exposure = number(args, "exposure", 1.0)?;
        let pixels = width.checked_mul(height).ok_or("Preview pixel overflow")?;
        let pixel_bytes = pixels.checked_mul(4).ok_or("Preview byte overflow")?;
        if pixel_bytes > self.limits.resource_bytes {
            return Err("Preview exceeds resource byte budget".into());
        }
        let plan = self.current_read_plan(value)?;
        self.emf_peak_admit(
            plan.memory
                .checked_add(pixel_bytes)
                .ok_or("Preview peak overflow")?,
        )?;
        let bytes = self.emf_source_bytes(value)?.to_vec();
        let document = CurrentDocument::open(
            &bytes,
            ReadLimits {
                file_bytes: self.limits.resource_bytes,
                cold_work: plan.shared_work,
                cache_bytes: plan.cache,
            },
        )?;
        self.charge_current_read(&document)?;
        let grid = GridAtTime::new(&document, selected)?;
        let work = (pixels as u64)
            .checked_mul(grid.lookup_work())
            .ok_or("Preview work overflow")?;
        self.charge(
            usize::try_from(work).map_err(|_| "Preview work overflow")?,
            0,
        )?;
        let rendered = render_slice(
            &grid,
            axis,
            slice,
            width,
            height,
            exposure,
            ProjectionLimits { pixels, work },
        )?;
        let source_clock = document
            .view
            .blocks()
            .find(|b| b.id() == grid.clock_id)
            .map(clock_value)
            .transpose()?
            .ok_or("Admitted source clock missing")?;
        let source_frame = document
            .view
            .blocks()
            .find(|b| b.id() == grid.frame_id)
            .map(frame_value)
            .transpose()?
            .ok_or("Admitted source frame missing")?;
        let digest = index_digest(document.view.index_digest());
        let frame = grid.frame_id;
        let clock = grid.clock_id;
        let tick = grid.selected_tick;
        drop(grid);
        drop(document);
        let mut image = self.store_rgba(width as u32, height as u32, rendered)?;
        if let Value::Record(meta) = &mut image {
            meta.insert("source_clock".into(), source_clock);
            meta.insert("source_frame".into(), source_frame);
            meta.insert("source_index_digest_blake3".into(), Value::String(digest));
            projection_metadata(
                meta,
                "electric_magnitude_exponential_grayscale_v1",
                frame,
                clock,
                tick,
                selected,
                exposure,
            );
        }
        Ok(image)
    }
}
pub(crate) fn projection_metadata(
    meta: &mut BTreeMap<String, Value>,
    profile: &str,
    frame: u32,
    clock: u32,
    tick: Option<i64>,
    slice: u32,
    exposure: f64,
) {
    for (key, value) in [
        ("projection_profile", profile),
        ("source_quantity", "electric_field_vector_magnitude"),
        ("source_units", "V/m"),
        ("exposure_units", "m/V"),
        (
            "output_quantity",
            "dimensionless display grayscale derivative",
        ),
        (
            "missing_samples",
            "transparent; absent/invalid observations are not default zero",
        ),
    ] {
        meta.insert(key.into(), Value::String(value.into()));
    }
    meta.insert("stored_rgb".into(), Value::Bool(false));
    meta.insert("source_emf_version".into(), Value::U64(4));
    meta.insert("source_frame_id".into(), Value::U64(u64::from(frame)));
    meta.insert("source_clock_id".into(), Value::U64(u64::from(clock)));
    meta.insert(
        "source_tick".into(),
        tick.map(Value::I64).unwrap_or(Value::Null),
    );
    meta.insert("source_time_slice".into(), Value::U64(u64::from(slice)));
    meta.insert("exposure".into(), Value::F64(exposure));
}

pub(crate) fn index_digest(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(64);
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    out
}
pub(crate) fn clock_value(
    block: qualia_core_db::q42::field_envelope::Block<'_>,
) -> Result<Value, String> {
    let mut out = BTreeMap::new();
    out.insert("metadata_id".into(), Value::U64(u64::from(block.id())));
    for (key, name) in [(1, "scale"), (3, "epoch_nanoseconds")] {
        let f = field(block, key).ok_or("Admitted clock field missing")?;
        out.insert(
            name.into(),
            Value::U64(u64::from(u32::from_le_bytes(
                f.value().try_into().map_err(|_| "Clock scalar mismatch")?,
            ))),
        );
    }
    for (key, name) in [(4, "ticks_per_second_num"), (5, "ticks_per_second_den")] {
        let f = field(block, key).ok_or("Admitted clock rate missing")?;
        out.insert(
            name.into(),
            Value::U64(u64::from_le_bytes(
                f.value().try_into().map_err(|_| "Clock rate mismatch")?,
            )),
        );
    }
    let seconds = field(block, 2).ok_or("Admitted clock epoch missing")?;
    out.insert(
        "epoch_seconds".into(),
        Value::I64(i64::from_le_bytes(
            seconds
                .value()
                .try_into()
                .map_err(|_| "Clock epoch mismatch")?,
        )),
    );
    Ok(Value::Record(out))
}
pub(crate) fn frame_value(
    block: qualia_core_db::q42::field_envelope::Block<'_>,
) -> Result<Value, String> {
    let mut out = BTreeMap::new();
    out.insert("metadata_id".into(), Value::U64(u64::from(block.id())));
    for (key, name) in [(3, "origin_m"), (4, "rotation_xyzw"), (5, "grid_spacing_m")] {
        if let Some(f) = field(block, key) {
            out.insert(
                name.into(),
                Value::List(
                    f.value()
                        .chunks_exact(8)
                        .map(|b| Value::F64(f64::from_le_bytes(b.try_into().unwrap())))
                        .collect(),
                ),
            );
        }
    }
    if let Some(f) = field(block, 6) {
        out.insert(
            "grid_dimensions".into(),
            Value::List(
                f.value()
                    .chunks_exact(4)
                    .map(|b| Value::U64(u64::from(u32::from_le_bytes(b.try_into().unwrap()))))
                    .collect(),
            ),
        );
    }
    if let Some(f) = field(block, 7) {
        out.insert(
            "common_index_order".into(),
            Value::U64(u64::from(u32::from_le_bytes(
                f.value().try_into().map_err(|_| "Frame order mismatch")?,
            ))),
        );
    } else {
        out.insert("common_index_order".into(), Value::Null);
    }
    out.insert(
        "coordinate_scope".into(),
        Value::String(
            "local metres; source origin/quaternion retained, not silently transformed".into(),
        ),
    );
    Ok(Value::Record(out))
}
