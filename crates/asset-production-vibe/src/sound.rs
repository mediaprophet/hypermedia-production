use super::*;

impl AssetHost<'_> {
    pub(super) fn sound(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.kinetics.sample_frames" | "hypermedia.timeline.frames" => {
                fields(a, &["numerator", "denominator", "count"])?;
                let numerator = count(a, "numerator", 24, 1_000_000)? as i64;
                let denominator = count(a, "denominator", 1, 1_000_000)? as i64;
                let frames = count(a, "count", 1, 256)?;
                let last_numerator = (frames - 1) as i128
                    * i128::from(timeline_editing::TICKS_PER_SECOND)
                    * i128::from(denominator);
                if last_numerator > i128::from(i64::MAX) * i128::from(numerator) {
                    return Err("Frame times exceed the signed tick range".into());
                }
                self.charge(frames, frames * 64)?;
                let rate = timeline_editing::FrameRate::new(numerator, denominator);
                Ok(Value::List(
                    (0..frames)
                        .map(|frame| {
                            let tick = rate.tick_of(frame as i64).0;
                            Value::Record(BTreeMap::from([
                                ("frame".into(), Value::I64(frame as i64)),
                                ("tick".into(), Value::I64(tick)),
                                (
                                    "seconds".into(),
                                    Value::F64(
                                        tick as f64 / timeline_editing::TICKS_PER_SECOND as f64,
                                    ),
                                ),
                            ]))
                        })
                        .collect(),
                ))
            }
            "hypermedia.acoustics.synthesize_tone" | "hypermedia.audio.tone" => {
                fields(
                    a,
                    &["frequency", "sample_rate", "frames", "amplitude", "emf"],
                )?;
                let rate = count(a, "sample_rate", 48000, 192000)? as u32;
                if rate < 8000 {
                    return Err("Sample rate must be 8000..192000".into());
                }
                let frames = count(a, "frames", 48000, 1_000_000)?;
                let freq = number(a, "frequency", 440.)?;
                let amp = number(a, "amplitude", 0.25)?;
                let has_emf = a.get("emf").is_some();
                if freq <= 0.
                    || freq >= rate as f64 / 2.
                    || freq as f32 <= 0.
                    || freq as f32 >= rate as f32 / 2.
                    || !(0. ..=1.).contains(&amp)
                {
                    return Err("Tone needs 0 < frequency < Nyquist and amplitude 0..1".into());
                }
                if has_emf && !(0. ..=1.).contains(&amp) {
                    return Err("Amplitude must be 0..1".into());
                }
                self.charge(frames * 8, frames * 4)?;
                let mut source_counts = None;
                let samples = if let Some(emf_val) = a.get("emf") {
                    let (samples, source, missing) =
                        self.sonify_current_emf(emf_val, rate, frames, freq, amp)?;
                    source_counts = Some((source, missing));
                    samples
                } else {
                    qualia_audio::media_store::synth_tone_f32(freq as f32, rate, frames, amp as f32)
                };
                let mut handle = self
                    .workspace
                    .insert(Artifact::Audio { rate, samples }, self.limits)?;
                if let (Value::Record(meta), Some((source, missing))) = (&mut handle, source_counts)
                {
                    meta.insert(
                        "derivative_profile".into(),
                        Value::String(
                            "electric_magnitude_normalized_amplitude_modulation_v1".into(),
                        ),
                    );
                    meta.insert("source_units".into(), Value::String("V/m".into()));
                    meta.insert(
                        "output_units".into(),
                        Value::String("normalized_audio_amplitude".into()),
                    );
                    meta.insert("acoustic_frequency_hz".into(), Value::F64(freq));
                    meta.insert("carrier_source".into(),Value::String("explicit synthesize_tone frequency argument; independent of EM carrier/sigma".into()));
                    meta.insert("source_sample_limit".into(), Value::U64(8192));
                    meta.insert("source_samples".into(), Value::U64(source as u64));
                    meta.insert("missing_source_samples".into(), Value::U64(missing as u64));
                    meta.insert("missing_policy".into(),Value::String("missing source states map to silence only in this derivative; source observations unchanged".into()));
                }
                Ok(handle)
            }
            "hypermedia.acoustics.scale_gain"
            | "hypermedia.audio.gain"
            | "hypermedia.acoustics.encode_wav"
            | "hypermedia.audio.wav" => {
                let is_gain =
                    id == "hypermedia.acoustics.scale_gain" || id == "hypermedia.audio.gain";
                fields(
                    a,
                    if is_gain {
                        &["audio", "gain"]
                    } else {
                        &["audio"]
                    },
                )?;
                let Artifact::Audio { rate, samples } =
                    self.workspace.get(required(a, "audio")?)?
                else {
                    return Err("Expected audio handle".into());
                };
                let rate = *rate;
                let n = samples.len();
                self.charge(n * 4, n * 10 + 44)?;
                let Artifact::Audio { samples, .. } = self.workspace.get(required(a, "audio")?)?
                else {
                    unreachable!()
                };
                if is_gain {
                    let gain = number(a, "gain", 1.)?;
                    if !(-16. ..=16.).contains(&gain) {
                        return Err("Audio gain outside -16..16".into());
                    }
                    let samples = samples
                        .iter()
                        .map(|s| (*s * gain as f32).clamp(-1., 1.))
                        .collect();
                    self.workspace
                        .insert(Artifact::Audio { rate, samples }, self.limits)
                } else {
                    let mut pcm = vec![0; n * 2];
                    qualia_audio::convert::mono_f32_to_i16_le(samples, &mut pcm)
                        .map_err(|e| format!("PCM conversion: {e:?}"))?;
                    let pcm: Vec<i16> = pcm
                        .chunks_exact(2)
                        .map(|b| i16::from_le_bytes([b[0], b[1]]))
                        .collect();
                    let mut bytes = vec![0; 44 + n * 2];
                    let size = qualia_audio::wav::encode_wav_i16_mono(&pcm, rate, &mut bytes)
                        .map_err(|e| format!("WAV encoding: {e:?}"))?;
                    bytes.truncate(size);
                    self.encode("wav", bytes)
                }
            }
            _ => Err("Unknown audio/time operation".into()),
        }
    }

    pub(crate) fn resolve_qac_bytes<'b>(&'b self, value: &Value) -> Result<&'b [u8], String> {
        let resource = if let Value::String(key) = value {
            self.inputs
                .iter()
                .find(|r| r.key == *key)
                .ok_or_else(|| format!("Missing retained input: {key}"))?
        } else {
            self.workspace.encoded(value)?
        };
        if resource.kind != "qac" {
            return Err(format!(
                "QAC inspection requires an encoded 'qac' resource, got '{}'",
                resource.kind
            ));
        }
        Ok(&resource.bytes)
    }
}

fn qac_stft_fft_size(sample_count: usize) -> Result<(u32, u32), String> {
    if sample_count < 2 {
        return Err("QAC v1 encoding requires at least 2 finite audio samples".into());
    }
    let fft_size = if sample_count >= 64 {
        64u32
    } else {
        let pow2 = sample_count.next_power_of_two();
        let chosen = if pow2 == sample_count { pow2 } else { pow2 >> 1 };
        chosen.max(2) as u32
    };
    let hop_size = (fft_size / 2).max(1);
    Ok((fft_size, hop_size))
}

fn deterministic_hoa_angles(species_id: &str, cue_profile: &str) -> (f64, f64) {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in species_id
        .as_bytes()
        .iter()
        .chain(b":")
        .chain(cue_profile.as_bytes())
    {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    let az_unit = ((hash & 0xffff_ffff) as f64) / (u32::MAX as f64);
    let el_unit = (((hash >> 32) & 0xffff_ffff) as f64) / (u32::MAX as f64);
    let az_rad = (az_unit * 2.0 - 1.0) * std::f64::consts::PI;
    let el_rad = (el_unit * 2.0 - 1.0) * (std::f64::consts::FRAC_PI_4);
    (az_rad, el_rad)
}

fn extract_audio_energy_and_modals(
    rate: u32,
    samples: &[f32],
    spectra_le: &[u8],
    fft_size: usize,
    stft_frames: usize,
) -> (f64, f32, Vec<u8>, u64) {
    use crate::emf_qac_anatomy_bridge::encode_modal_resonance_record;

    let mut sum_sq = 0.0f64;
    let mut peak = 0.0f32;
    for &s in samples {
        let abs = s.abs();
        if abs > peak {
            peak = abs;
        }
        let s64 = f64::from(s);
        sum_sq += s64 * s64;
    }
    let rms = (sum_sq / (samples.len() as f64)).sqrt();

    let nyquist_bin = fft_size / 2;
    let half_frames = (stft_frames / 2).max(1);
    let late_frames = stft_frames.saturating_sub(half_frames);
    let mut bin_stats: Vec<(usize, f64, f64, f64)> = Vec::with_capacity(nyquist_bin);

    for bin in 1..=nyquist_bin {
        let mut mag_sq_sum = 0.0f64;
        let mut early_sq = 0.0f64;
        let mut late_sq = 0.0f64;
        for frame in 0..stft_frames {
            let offset = (frame * fft_size + bin) * 8;
            let re = f32::from_le_bytes(spectra_le[offset..offset + 4].try_into().unwrap()) as f64;
            let im =
                f32::from_le_bytes(spectra_le[offset + 4..offset + 8].try_into().unwrap()) as f64;
            let e = re * re + im * im;
            mag_sq_sum += e;
            if frame < half_frames {
                early_sq += e;
            } else {
                late_sq += e;
            }
        }
        bin_stats.push((bin, mag_sq_sum, early_sq, late_sq));
    }

    bin_stats.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let duration_s = ((samples.len() as f64) / f64::from(rate)).max(1e-3);
    let mut modal_payload = Vec::new();
    let mut modal_count = 0u64;

    for (rank, (bin, mag_sq_sum, early_sq, late_sq)) in bin_stats.into_iter().take(4).enumerate() {
        let freq_hz = ((bin as f64) * f64::from(rate) / (fft_size as f64)).max(20.0) as f32;
        let mean_mag = (mag_sq_sum / (stft_frames as f64)).sqrt() / (fft_size as f64).max(1.0);
        let amp_pa = (mean_mag.max(rms * 0.25).max(1e-6)) as f32;
        let decay_s1 = if late_frames > 0 && late_sq > 1e-18 && early_sq > late_sq {
            let ratio = (early_sq / (half_frames as f64)) / (late_sq / (late_frames as f64));
            (ratio.sqrt().ln() / (0.5 * duration_s)).clamp(0.5, 25.0) as f32
        } else {
            2.5f32
        };
        let phase_rad = 0.0f32;
        modal_payload.extend_from_slice(&encode_modal_resonance_record(
            freq_hz,
            amp_pa,
            decay_s1,
            phase_rad,
            rank as u32,
            0,
            1,
        ));
        modal_count += 1;
    }

    if modal_count == 0 {
        modal_payload.extend_from_slice(&encode_modal_resonance_record(
            440.0,
            rms.max(1e-6) as f32,
            2.5,
            0.0,
            0,
            0,
            1,
        ));
        modal_count = 1;
    }

    (rms, peak, modal_payload, modal_count)
}

pub(crate) fn produce_qac_v1_from_pcm(
    rate: u32,
    samples: &[f32],
    species_id: &str,
    cue_profile: &str,
    max_resource_bytes: usize,
) -> Result<(Vec<u8>, BTreeMap<String, Value>), String> {
    use crate::emf_qac_anatomy_bridge::qac_v1_envelope::assemble_canonical_4mode_qac;
    use crate::emf_qac_anatomy_bridge::encode_hoa_order1_sn3d_frame;
    use qualia_core_db::audio::qac::signal::stft::{transform_into, Plan, ProviderLimits};

    if rate == 0 || rate > 384_000 {
        return Err(format!("Sample rate {rate} Hz outside valid QAC range 1..=384000"));
    }
    if samples.len() < 2 {
        return Err("QAC v1 encoding requires at least 2 samples".into());
    }
    if samples.iter().any(|s| !s.is_finite()) {
        return Err("QAC v1 encoding rejected non-finite PCM sample".into());
    }
    if species_id.trim().is_empty() || species_id.len() > 256 {
        return Err("species_id must be 1..=256 non-whitespace bytes".into());
    }
    if cue_profile.trim().is_empty() || cue_profile.len() > 256 {
        return Err("cue_profile must be 1..=256 non-whitespace bytes".into());
    }

    let (fft_size, hop_size) = qac_stft_fft_size(samples.len())?;
    let pcm_byte_len = samples
        .len()
        .checked_mul(4)
        .ok_or("QAC PCM byte size overflow")?;
    let provider_limits = ProviderLimits::default();
    let plan = Plan::new(
        pcm_byte_len,
        1,
        fft_size as usize,
        hop_size as usize,
        provider_limits,
    )
    .map_err(|e| format!("QAC STFT plan rejected: {e:?}"))?;

    let hoa_byte_len = samples
        .len()
        .checked_mul(16)
        .ok_or("QAC HOA byte size overflow")?;
    let estimated_bytes = pcm_byte_len
        .saturating_add(plan.output_bytes())
        .saturating_add(hoa_byte_len)
        .saturating_add(16_384);
    if estimated_bytes > max_resource_bytes {
        return Err(format!(
            "Encoded QAC v1 estimated size ({estimated_bytes} bytes) exceeds resource_bytes budget ({max_resource_bytes})"
        ));
    }

    let mut pcm_bytes = Vec::with_capacity(pcm_byte_len);
    for &sample in samples {
        pcm_bytes.extend_from_slice(&sample.to_bits().to_le_bytes());
    }
    let pcm_digest = *blake3::hash(&pcm_bytes).as_bytes();

    let mut mono_buf = vec![0.0f32; fft_size as usize];
    let mut fft_buf = vec![0.0f32; (fft_size as usize) * 2];
    let mut win_buf = vec![0.0f32; fft_size as usize];
    let mut spectra_buf = vec![0u8; plan.output_bytes()];
    transform_into(
        &pcm_bytes,
        1,
        fft_size as usize,
        hop_size as usize,
        &mut mono_buf,
        &mut fft_buf,
        &mut win_buf,
        &mut spectra_buf,
        provider_limits,
    )
    .map_err(|e| format!("QAC STFT transform failed: {e:?}"))?;

    let (rms_energy, peak_amplitude, modal_payload, modal_count) =
        extract_audio_energy_and_modals(rate, samples, &spectra_buf, fft_size as usize, plan.frames());

    let (az_rad, el_rad) = deterministic_hoa_angles(species_id, cue_profile);
    let mut hoa_payload = Vec::with_capacity(hoa_byte_len);
    for &sample in samples {
        hoa_payload.extend_from_slice(&encode_hoa_order1_sn3d_frame(sample, az_rad, el_rad));
    }

    let lineage_id = format!("hypermedia.acoustics.qac_v1:{species_id}:{cue_profile}");
    let qac_bytes = assemble_canonical_4mode_qac(
        1_700_000_000,
        u64::from(rate),
        samples.len() as u64,
        fft_size,
        hop_size,
        plan.frames() as u64,
        &win_buf,
        &pcm_bytes,
        &pcm_digest,
        &spectra_buf,
        &hoa_payload,
        &modal_payload,
        modal_count,
        cue_profile,
        &lineage_id,
    )?;

    if qac_bytes.len() > max_resource_bytes {
        return Err(format!(
            "Encoded QAC v1 container ({} bytes) exceeds resource_bytes budget ({max_resource_bytes})",
            qac_bytes.len()
        ));
    }

    let mut verification = inspect_and_verify_qac_v1_bytes(&qac_bytes)?;
    verification.insert("species_id".into(), Value::String(species_id.into()));
    verification.insert("cue_profile".into(), Value::String(cue_profile.into()));
    verification.insert("rms_energy".into(), Value::F64(rms_energy));
    verification.insert("peak_amplitude".into(), Value::F64(f64::from(peak_amplitude)));

    Ok((qac_bytes, verification))
}

fn verify_qac_chunk_samples(
    mode: u16,
    stride: u32,
    record_count: u64,
    chunk: &[u8],
    is_cue_container: bool,
) -> Result<Option<(f64, f32)>, String> {
    use crate::emf_qac_anatomy_bridge::{
        QAC_MODE_HOA_SPATIAL_AUDIO, QAC_MODE_MODAL_RESONANCE, QAC_MODE_PCM_PRESERVATION,
        QAC_MODE_STFT_PRESERVATION,
    };

    let expected_len = (record_count as usize)
        .checked_mul(stride as usize)
        .ok_or("QAC chunk size overflow")?;
    if mode != 0 && chunk.len() != expected_len {
        return Err(format!(
            "QAC v1 mode 0x{mode:04x} chunk length {} != record_count * stride ({expected_len})",
            chunk.len()
        ));
    }

    match mode {
        QAC_MODE_PCM_PRESERVATION => {
            if stride == 4 && !chunk.is_empty() {
                let mut sum_sq = 0.0f64;
                let mut peak = 0.0f32;
                let mut n = 0usize;
                for raw in chunk.chunks_exact(4) {
                    let s = f32::from_le_bytes(raw.try_into().unwrap());
                    if !s.is_finite() {
                        return Err("QAC v1 PCM chunk contains non-finite f32 sample".into());
                    }
                    let abs = s.abs();
                    if abs > peak {
                        peak = abs;
                    }
                    let s64 = f64::from(s);
                    sum_sq += s64 * s64;
                    n += 1;
                }
                let rms = (sum_sq / (n as f64)).sqrt();
                return Ok(Some((rms, peak)));
            }
        }
        QAC_MODE_STFT_PRESERVATION => {
            for raw in chunk.chunks_exact(4) {
                let v = f32::from_le_bytes(raw.try_into().unwrap());
                if !v.is_finite() {
                    return Err("QAC v1 STFT chunk contains non-finite float".into());
                }
            }
        }
        QAC_MODE_HOA_SPATIAL_AUDIO => {
            if stride != 16 {
                return Err(format!("QAC v1 HOA mode 0x0102 stride {stride} != 16"));
            }
            for raw in chunk.chunks_exact(4) {
                let v = f32::from_le_bytes(raw.try_into().unwrap());
                if !v.is_finite() {
                    return Err("QAC v1 HOA chunk contains non-finite sample".into());
                }
            }
        }
        QAC_MODE_MODAL_RESONANCE if is_cue_container => {
            if stride != 32 {
                return Err(format!("QAC v1 modal mode 0x0104 stride {stride} != 32"));
            }
            for rec in chunk.chunks_exact(32) {
                let freq = f32::from_le_bytes(rec[0..4].try_into().unwrap());
                let amp = f32::from_le_bytes(rec[4..8].try_into().unwrap());
                let decay = f32::from_le_bytes(rec[8..12].try_into().unwrap());
                let phase = f32::from_le_bytes(rec[12..16].try_into().unwrap());
                let reserved = u32::from_le_bytes(rec[28..32].try_into().unwrap());
                if !freq.is_finite()
                    || freq <= 0.0
                    || !amp.is_finite()
                    || amp <= 0.0
                    || !decay.is_finite()
                    || decay <= 0.0
                    || !phase.is_finite()
                    || reserved != 0
                {
                    return Err("QAC v1 modal_resonance record invalid".into());
                }
            }
        }
        _ => {}
    }
    Ok(None)
}

pub(crate) fn inspect_and_verify_qac_v1_bytes(
    bytes: &[u8],
) -> Result<BTreeMap<String, Value>, String> {
    use crate::emf_qac_anatomy_bridge::{
        sha256_hex, QacV1CueRegistry, QAC_MODE_HOA_SPATIAL_AUDIO, QAC_MODE_PCM_PRESERVATION,
    };
    use qualia_core_db::audio::qac::metadata::Transform;
    use qualia_core_db::audio::qac::signal::{admit as admit_qac_signal, Limits as QacSignalLimits};
    use qualia_core_db::audio::qac::{QacStorageOptions, QacStorageView};
    use qualia_core_db::q42::field_envelope::{Family, Limits as StorageLimits, StructuralView};
    use qualia_core_db::q42::field_semantics::{
        self as semantics, field, Limits as SharedLimits, NodeState, Scratch,
    };

    let storage_limits = StorageLimits::default();
    let raw_view = StructuralView::open(bytes, Family::QacV1Candidate, storage_limits)
        .map_err(|e| format!("QAC v1 structural verification failed: {e:?}"))?;
    if raw_view.entry_count() == 0 {
        return Err("QAC v1 raw structural view contains no entries".into());
    }
    let qac_view = QacStorageView::open(bytes, QacStorageOptions::default())
        .map_err(|e| format!("QAC v1 storage admission failed: {e:?}"))?;
    let view = qac_view.structure();
    let entry_count = view.entry_count();
    if entry_count == 0 {
        return Err("QAC v1 container contains no entries".into());
    }

    let mut has_hoa = false;
    for idx in 1..=entry_count {
        let entry = view.entry(idx).map_err(|e| format!("QAC v1 entry {idx}: {e:?}"))?;
        if entry.mode() == QAC_MODE_HOA_SPATIAL_AUDIO {
            has_hoa = true;
        }
    }
    let is_cue_container = has_hoa
        || view
            .blocks()
            .any(|b| matches!(b.schema_type(), 204 | 206));

    let mut mode_counts: BTreeMap<u16, u64> = BTreeMap::new();
    let mut pcm_sample_count: Option<u64> = None;
    let mut pcm_stats: Option<(f64, f32)> = None;

    for idx in 1..=entry_count {
        let entry = view.entry(idx).map_err(|e| format!("QAC v1 entry {idx}: {e:?}"))?;
        let chunk = view
            .verify_chunk(idx)
            .map_err(|e| format!("QAC v1 chunk {idx} verification failed: {e:?}"))?;
        *mode_counts.entry(entry.mode()).or_insert(0) += entry.record_count();
        if entry.mode() == QAC_MODE_PCM_PRESERVATION && pcm_sample_count.is_none() {
            pcm_sample_count = Some(entry.record_count());
        }
        if let Some(stats) = verify_qac_chunk_samples(
            entry.mode(),
            entry.stride(),
            entry.record_count(),
            chunk,
            is_cue_container,
        )? {
            if pcm_stats.is_none() {
                pcm_stats = Some(stats);
            }
        }
    }

    if is_cue_container {
        let mut states = [NodeState::default(); 64];
        let mut visited = [NodeState::default(); 64];
        let mut scratch = Scratch {
            metadata: &mut states,
            entries: &mut visited,
        };
        semantics::admit_shared(view, &QacV1CueRegistry, &mut scratch, SharedLimits::default())
            .map_err(|e| format!("QAC v1 shared semantic admission failed: {e:?}"))?;
    } else {
        for idx in 1..=entry_count {
            let mut states = [NodeState::default(); 64];
            let mut visited = [NodeState::default(); 64];
            let mut scratch = Scratch {
                metadata: &mut states,
                entries: &mut visited,
            };
            admit_qac_signal(
                view,
                idx,
                &mut scratch,
                SharedLimits::default(),
                QacSignalLimits::default(),
            )
            .map_err(|e| format!("QAC v1 signal::admit entry {idx} failed: {e:?}"))?;
        }
    }

    let mut sample_rate_hz: Option<u64> = None;
    let mut stft_fft_size: Option<u32> = None;
    let mut stft_hop_size: Option<u32> = None;
    let mut lineage: Option<String> = None;
    let mut excitation_label: Option<String> = None;

    for block in view.blocks() {
        match (block.schema_type(), block.revision()) {
            (1, 1) if sample_rate_hz.is_none() => {
                if let (Some(n_f), Some(d_f)) = (field(block, 4), field(block, 5)) {
                    if n_f.value().len() == 8 && d_f.value().len() == 8 {
                        let num = u64::from_le_bytes(n_f.value().try_into().unwrap());
                        let den = u64::from_le_bytes(d_f.value().try_into().unwrap());
                        if den > 0 {
                            sample_rate_hz = Some(num / den);
                        }
                    }
                }
            }
            (4, 1) if lineage.is_none() => {
                if let Some(f) = field(block, 7) {
                    if let Ok(s) = std::str::from_utf8(f.value()) {
                        if !s.is_empty() {
                            lineage = Some(s.to_owned());
                        }
                    }
                }
            }
            (201, 1) if stft_fft_size.is_none() => {
                if let Ok(tr) = Transform::from_block(block) {
                    stft_fft_size = Some(tr.fft_size);
                    stft_hop_size = Some(tr.hop);
                }
            }
            (210, 1) if excitation_label.is_none() => {
                if let Some(f) = field(block, 4) {
                    if let Ok(s) = std::str::from_utf8(f.value()) {
                        if !s.is_empty()
                            && s != "hann_stft_complex_preservation"
                            && s != "hoa_order1_sn3d_spatial_field"
                        {
                            excitation_label = Some(s.to_owned());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let index_digest_blake3: String = view
        .index_digest()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let modes_map: BTreeMap<String, Value> = mode_counts
        .into_iter()
        .map(|(mode, count)| (format!("0x{mode:04x}"), Value::U64(count)))
        .collect();

    let mut out = BTreeMap::new();
    out.insert("valid".into(), Value::Bool(true));
    out.insert("format".into(), Value::String("qac_v1".into()));
    out.insert("container_version".into(), Value::U64(1));
    out.insert("entry_count".into(), Value::U64(u64::from(entry_count)));
    out.insert(
        "chunk_digests_verified".into(),
        Value::U64(u64::from(entry_count)),
    );
    out.insert("byte_len".into(), Value::U64(bytes.len() as u64));
    out.insert("sha256_hex".into(), Value::String(sha256_hex(bytes)));
    out.insert(
        "blake3_hex".into(),
        Value::String(blake3::hash(bytes).to_hex().to_string()),
    );
    out.insert(
        "index_digest_blake3".into(),
        Value::String(index_digest_blake3),
    );
    out.insert("mode_record_counts".into(), Value::Record(modes_map));
    if let Some(sr) = sample_rate_hz {
        out.insert("sample_rate_hz".into(), Value::U64(sr));
    }
    if let Some(n) = pcm_sample_count {
        out.insert("pcm_sample_count".into(), Value::U64(n));
    }
    if let Some(fft) = stft_fft_size {
        out.insert("stft_fft_size".into(), Value::U64(u64::from(fft)));
    }
    if let Some(hop) = stft_hop_size {
        out.insert("stft_hop_size".into(), Value::U64(u64::from(hop)));
    }
    if let Some(lin) = lineage {
        out.insert("lineage".into(), Value::String(lin));
    }
    if let Some(exc) = excitation_label {
        out.insert("excitation_label".into(), Value::String(exc));
    }
    if let Some((rms, peak)) = pcm_stats {
        out.insert("rms_energy".into(), Value::F64(rms));
        out.insert("peak_amplitude".into(), Value::F64(f64::from(peak)));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::atomic::AtomicBool;

    fn host<'a>(cancel: &'a AtomicBool, limits: &'a ToolLimits) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits,
            cancel,
            work: 0,
        }
    }

    fn compile_green_dipole(host: &mut AssetHost<'_>) -> Value {
        let mut compile_args = BTreeMap::new();
        compile_args.insert(
            "origin".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        compile_args.insert("voxel_size".into(), Value::F64(0.25));
        compile_args.insert(
            "grid_dims".into(),
            Value::List(vec![Value::U64(1), Value::U64(1), Value::U64(1)]),
        );
        compile_args.insert("epoch_s".into(), Value::F64(1_700_000_000.0));
        let mut src = BTreeMap::new();
        src.insert(
            "position".into(),
            Value::List(vec![Value::F64(0.5), Value::F64(0.5), Value::F64(0.5)]),
        );
        src.insert(
            "e_amplitude".into(),
            Value::List(vec![Value::F64(150.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        src.insert("wavelength_nm".into(), Value::F64(532.0));
        src.insert("phase".into(), Value::F64(0.0));
        src.insert(
            "stokes".into(),
            Value::List(vec![Value::F64(1.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        compile_args.insert("sources".into(), Value::List(vec![Value::Record(src)]));
        host.invoke(
            "hypermedia.electromagnetics.compile_emf",
            &Value::Record(compile_args),
        )
        .expect("compile_emf succeeds")
    }

    fn audio_peak(host: &AssetHost<'_>, handle: &Value) -> f32 {
        match host.workspace.get(handle).expect("audio handle") {
            Artifact::Audio { samples, .. } => samples.iter().fold(0.0f32, |m, s| m.max(s.abs())),
            _ => panic!("expected Audio artifact"),
        }
    }

    #[test]
    fn test_acoustics_bus_loudness_emf_sonic_objects() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = host(&cancel, &limits);

        let emf = compile_green_dipole(&mut host);

        let mut tone_args = BTreeMap::new();
        tone_args.insert("emf".into(), emf);
        tone_args.insert("sample_rate".into(), Value::U64(48_000));
        tone_args.insert("frames".into(), Value::U64(24_000));
        tone_args.insert("amplitude".into(), Value::F64(0.25));
        let sonic = host
            .sound("hypermedia.acoustics.synthesize_tone", &tone_args)
            .expect("emf-sonic synthesize_tone");
        match host.workspace.get(&sonic).expect("emf-sonic handle") {
            Artifact::Audio { rate, samples } => {
                assert_eq!(*rate, 48_000);
                assert_eq!(samples.len(), 24_000);
                assert!(
                    samples.iter().any(|s| s.abs() > 0.01),
                    "field-derived sonic from sealed .emf must be non-silent"
                );
                assert!(samples
                    .iter()
                    .all(|s| s.is_finite() && *s >= -1.0 && *s <= 1.0));
            }
            _ => panic!("expected Audio from emf-sonic"),
        }

        let mut lufs_args = BTreeMap::new();
        lufs_args.insert("audio".into(), sonic.clone());
        lufs_args.insert("freq_hz".into(), Value::F64(440.0));
        lufs_args.insert("bpm".into(), Value::F64(120.0));
        lufs_args.insert("azimuth_deg".into(), Value::F64(30.0));
        let loudness = host
            .analyze_ebu_r128_camelot_daw(&lufs_args)
            .expect("loudness");
        let rec = record(&loudness).expect("loudness record");
        let lufs = match rec.get("integrated_lufs") {
            Some(Value::F64(v)) => *v,
            _ => panic!("integrated_lufs"),
        };
        assert!(
            lufs.is_finite() && lufs > -70.0 && lufs < 0.0,
            "gated EBU R128 LUFS must be a real loudness, got {lufs}"
        );
        let vbap = match rec.get("vbap_5_1_power") {
            Some(Value::F64(v)) => *v,
            _ => panic!("vbap_5_1_power"),
        };
        assert!(
            (vbap - 1.0).abs() < 0.15,
            "VBAP 5.1 power must be near unity, got {vbap}"
        );
        let measured = rec.get("audio").expect("loudness audio").clone();
        assert!(audio_peak(&host, &measured) > 0.01);

        let mut bus = BTreeMap::new();
        bus.insert("name".into(), Value::String("Field".into()));
        bus.insert("audio".into(), sonic.clone());
        bus.insert("send_to".into(), Value::String("Master".into()));
        bus.insert("gain_db".into(), Value::F64(0.0));
        bus.insert("state".into(), Value::String("playing".into()));
        let mut master = BTreeMap::new();
        master.insert("name".into(), Value::String("Master".into()));
        master.insert("send_to".into(), Value::String("".into()));
        let mut bus_args = BTreeMap::new();
        bus_args.insert(
            "buses".into(),
            Value::List(vec![Value::Record(bus), Value::Record(master)]),
        );
        let mixed = host
            .acoustics_discourse_chronicle_ext_op("hypermedia.acoustics.route_bus_dag", &bus_args)
            .expect("route_bus_dag");
        assert!(
            audio_peak(&host, &mixed) > 0.01,
            "bus DAG master must be a non-silent Audio object"
        );

        let mut spat_args = BTreeMap::new();
        spat_args.insert("audio".into(), mixed);
        spat_args.insert(
            "source_pos".into(),
            Value::List(vec![Value::F64(5.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        spat_args.insert(
            "listener_pos".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        spat_args.insert("occlusion".into(), Value::F64(0.15));
        spat_args.insert("reverb_room_size".into(), Value::F64(0.2));
        let spatial = host
            .acoustics_discourse_chronicle_ext_op(
                "hypermedia.acoustics.spatialize_pga_zone",
                &spat_args,
            )
            .expect("spatialize_pga_zone");
        match host.workspace.get(&spatial).expect("spatial handle") {
            Artifact::Audio { samples, .. } => {
                assert!(
                    samples.iter().any(|s| s.abs() > 1e-4),
                    "spatial wave non-silent"
                );
                assert!(samples.iter().all(|s| s.is_finite()));
            }
            _ => panic!("expected spatial Audio"),
        }
    }
}
