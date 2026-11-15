//! Bounded acoustics operations: biquad IIR filtering, multi-track mixing with
//! linear fade envelopes, and AAC-LC CBR audio compression.

use super::*;
use audio_aac_codec::{Encoder, EncoderConfig, SAMPLE_RATES};

impl AssetHost<'_> {
    /// Categorical operations dispatch for the `acoustics` category extensions.
    pub(super) fn acoustics_ext(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.acoustics.filter_biquad"
            | "synth.acoustics.filter_biquad"
            | "acoustics.filter_biquad"
            | "filter_biquad" => self.filter_biquad(a),

            "hypermedia.acoustics.mix_tracks"
            | "synth.acoustics.mix_tracks"
            | "acoustics.mix_tracks"
            | "mix_tracks" => self.mix_tracks(a),

            "hypermedia.acoustics.encode_aac"
            | "synth.acoustics.encode_aac"
            | "acoustics.encode_aac"
            | "encode_aac" => self.encode_aac(a),

            _ => Err(format!("Unknown acoustics operation: {id}")),
        }
    }

    /// Audio EQ Cookbook biquad IIR filter (lowpass, highpass, bandpass, notch, peaking).
    fn filter_biquad(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["audio", "mode", "cutoff_hz", "q", "gain_db"])?;

        let (rate, samples) = match self.workspace.get(required(a, "audio")?)? {
            Artifact::Audio { rate, samples } => (*rate, samples.clone()),
            _ => return Err("Expected audio handle".into()),
        };

        let mode = match a.get("mode") {
            Some(v) => text(v)?,
            None => "lowpass",
        };

        let cutoff_hz = number(a, "cutoff_hz", 1000.0)?;
        let max_cutoff = (rate as f64) * 0.49;
        if cutoff_hz < 10.0 || cutoff_hz >= max_cutoff {
            return Err(format!(
                "cutoff_hz must satisfy 10.0 <= cutoff_hz < {max_cutoff:.1} (Nyquist margin), got {cutoff_hz}"
            ));
        }

        let q = number(a, "q", 0.70710678)?;
        if !(0.05..=24.0).contains(&q) {
            return Err(format!("q outside 0.05..=24.0: {q}"));
        }

        let gain_db = number(a, "gain_db", 0.0)?;
        if !(-36.0..=36.0).contains(&gain_db) {
            return Err(format!("gain_db outside -36.0..=36.0: {gain_db}"));
        }

        self.charge(samples.len() * 8, samples.len() * 4)?;

        use qualia_audio::features::filters::{
            design_bandpass, design_bandreject, design_highpass, design_lowpass, BiquadCoeffs,
            BiquadState,
        };
        let sr = rate as f32;
        let f0 = cutoff_hz as f32;
        let qf = q as f32;
        let coeffs = match mode {
            "lowpass" => design_lowpass(sr, f0, qf),
            "highpass" => design_highpass(sr, f0, qf),
            "bandpass" => design_bandpass(sr, f0, qf),
            "notch" => design_bandreject(sr, f0, qf),
            "peaking" => BiquadCoeffs::design_peaking_eq(sr, f0, qf, gain_db as f32),
            _ => {
                return Err(format!(
                    "Unknown filter mode: {mode} (expected lowpass, highpass, bandpass, notch, or peaking)"
                ))
            }
        };

        let mut state = BiquadState::new();
        let mut out = vec![0.0f32; samples.len()];
        state
            .process(&coeffs, &samples, &mut out)
            .map_err(|e| format!("Biquad filter process error: {e:?}"))?;
        for s in &mut out {
            *s = if s.is_finite() { s.clamp(-1.0, 1.0) } else { 0.0 };
        }

        self.workspace
            .insert(Artifact::Audio { rate, samples: out }, self.limits)
    }

    /// Multi-track audio mixer with linear fade envelopes and master gain.
    fn mix_tracks(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["tracks", "master_gain"])?;

        let master_gain = number(a, "master_gain", 1.0)?;
        if !(0.0..=16.0).contains(&master_gain) {
            return Err(format!("master_gain outside 0.0..=16.0: {master_gain}"));
        }

        let track_items = list(required(a, "tracks")?)?;
        if track_items.is_empty() || track_items.len() > 16 {
            return Err(format!(
                "tracks list must contain 1..=16 records, got {}",
                track_items.len()
            ));
        }

        struct TrackData {
            samples: Vec<f32>,
            gain: f64,
            offset_seconds: f64,
            fade_in_seconds: f64,
            fade_out_seconds: f64,
        }

        let mut tracks = Vec::with_capacity(track_items.len());
        let mut common_rate: Option<u32> = None;
        let mut total_len: usize = 0;

        for (idx, item) in track_items.iter().enumerate() {
            let tr = record(item)?;
            fields(
                tr,
                &[
                    "audio",
                    "gain",
                    "offset_seconds",
                    "fade_in_seconds",
                    "fade_out_seconds",
                ],
            )?;

            let (track_rate, track_samples) = match self.workspace.get(required(tr, "audio")?)? {
                Artifact::Audio { rate, samples } => (*rate, samples.clone()),
                _ => return Err(format!("Track {idx}: expected audio handle")),
            };

            if let Some(cr) = common_rate {
                if track_rate != cr {
                    return Err(format!(
                        "Track {idx} sample rate {track_rate} mismatch with expected {cr}"
                    ));
                }
            } else {
                common_rate = Some(track_rate);
            }

            let gain = number(tr, "gain", 1.0)?;
            if !gain.is_finite() || !(-16.0..=16.0).contains(&gain) {
                return Err(format!("Track {idx} gain outside -16.0..=16.0: {gain}"));
            }

            let offset_seconds = number(tr, "offset_seconds", 0.0)?;
            if offset_seconds < 0.0 {
                return Err(format!(
                    "Track {idx} offset_seconds must be >= 0.0: {offset_seconds}"
                ));
            }

            let fade_in_seconds = number(tr, "fade_in_seconds", 0.0)?;
            if fade_in_seconds < 0.0 {
                return Err(format!(
                    "Track {idx} fade_in_seconds must be >= 0.0: {fade_in_seconds}"
                ));
            }

            let fade_out_seconds = number(tr, "fade_out_seconds", 0.0)?;
            if fade_out_seconds < 0.0 {
                return Err(format!(
                    "Track {idx} fade_out_seconds must be >= 0.0: {fade_out_seconds}"
                ));
            }

            let rate_f64 = track_rate as f64;
            let offset_samples = (offset_seconds * rate_f64).round() as usize;
            let track_end = offset_samples
                .checked_add(track_samples.len())
                .ok_or_else(|| format!("Track {idx} offset and sample count overflow"))?;
            total_len = total_len.max(track_end);

            tracks.push(TrackData {
                samples: track_samples,
                gain,
                offset_seconds,
                fade_in_seconds,
                fade_out_seconds,
            });
        }

        let rate = common_rate.ok_or("No tracks provided")?;

        if total_len > 1_000_000 {
            return Err(format!(
                "Total mixed length ({total_len} samples) exceeds maximum allowed (1,000,000 samples)"
            ));
        }

        self.charge(total_len * tracks.len() * 4, total_len * 4)?;

        let rate_f64 = rate as f64;
        let mut mixed_accum = vec![0.0_f64; total_len];

        for tr in &tracks {
            let offset_samples = (tr.offset_seconds * rate_f64).round() as usize;
            let fade_in_samples = (tr.fade_in_seconds * rate_f64).round() as usize;
            let fade_out_samples = (tr.fade_out_seconds * rate_f64).round() as usize;
            let n = tr.samples.len();

            for (j, &s) in tr.samples.iter().enumerate() {
                let in_env = if fade_in_samples > 0 && j < fade_in_samples {
                    (j as f64 / fade_in_samples as f64).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                let out_env = if fade_out_samples > 0 && (n.saturating_sub(1 + j)) < fade_out_samples {
                    ((n - 1 - j) as f64 / fade_out_samples as f64).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                let env = in_env.min(out_env);
                let sample_val = (s as f64) * tr.gain * env;
                mixed_accum[offset_samples + j] += sample_val;
            }
        }

        let mut mixed = Vec::with_capacity(total_len);
        for acc in mixed_accum {
            let out_val = (acc * master_gain) as f32;
            let clamped = if out_val.is_finite() {
                out_val.clamp(-1.0, 1.0)
            } else {
                0.0
            };
            mixed.push(clamped);
        }

        self.workspace
            .insert(Artifact::Audio { rate, samples: mixed }, self.limits)
    }

    /// AAC-LC CBR audio encoding with ADTS framing, or canonical QAC v1 (`qac\0\0\0\0\0` v1)
    /// container encoding when `format="qac"`.
    fn encode_aac(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["audio", "bitrate", "format", "species_id", "cue_profile"])?;

        let (rate, samples) = match self.workspace.get(required(a, "audio")?)? {
            Artifact::Audio { rate, samples } => (*rate, samples.clone()),
            _ => return Err("Expected audio handle".into()),
        };

        let format = a.get("format").map(text).transpose()?.unwrap_or("aac");
        let species_id = a
            .get("species_id")
            .map(text)
            .transpose()?
            .unwrap_or("hypermedia.acoustics.encoded");
        let cue_profile = a
            .get("cue_profile")
            .map(text)
            .transpose()?
            .unwrap_or("acoustic_pcm_stft_hoa_modal_v1");

        match format {
            "aac" => {
                let bitrate = count(a, "bitrate", 96_000, 320_000)? as u32;
                if bitrate < 16_000 {
                    return Err(format!("bitrate must be >= 16000, got {bitrate}"));
                }

                self.charge(samples.len() * 16, samples.len() * 2 + 4096)?;

                if !SAMPLE_RATES.contains(&rate) {
                    return Err(format!(
                        "Sample rate {rate} Hz is not supported by AAC (supported rates: {SAMPLE_RATES:?})"
                    ));
                }

                let mut cfg = EncoderConfig::cbr(rate, 1, bitrate);
                cfg.adts = true;
                let mut enc = Encoder::new(cfg).map_err(|e| format!("AAC encoder init: {e}"))?;

                let mut aus = enc.encode(&[&samples]);
                aus.extend(enc.flush());
                let aac_bytes = aus.into_iter().flatten().collect::<Vec<u8>>();

                self.encode("aac", aac_bytes)
            }
            "qac" => {
                if a.contains_key("bitrate") {
                    let bitrate = count(a, "bitrate", 96_000, 320_000)? as u32;
                    if bitrate < 16_000 {
                        return Err(format!("bitrate must be >= 16000, got {bitrate}"));
                    }
                }
                let scratch_est = samples
                    .len()
                    .saturating_mul(40)
                    .saturating_add(16_384);
                self.charge(samples.len() * 24 + 1024, scratch_est)?;

                let (qac_bytes, verification) = crate::sound::produce_qac_v1_from_pcm(
                    rate,
                    &samples,
                    species_id,
                    cue_profile,
                    self.limits.resource_bytes,
                )?;
                let mut handle = self.admit_encoded("qac", qac_bytes)?;
                if let Value::Record(ref mut meta) = handle {
                    for (k, v) in verification {
                        if k != "handle" && k != "type" {
                            meta.insert(k, v);
                        }
                    }
                }
                Ok(handle)
            }
            other => Err(format!(
                "Unsupported audio encoding format '{other}' (expected \"aac\" or \"qac\")"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::q42::field_envelope::{Family, Limits as StorageLimits, StructuralView};
    use std::sync::atomic::AtomicBool;

    fn test_host<'a>(cancel: &'a AtomicBool, limits: &'a ToolLimits) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits,
            cancel,
            work: 0,
        }
    }

    fn synth_tone(host: &mut AssetHost<'_>, frames: u64) -> Value {
        let mut args = Args::new();
        args.insert("frequency".into(), Value::F64(440.0));
        args.insert("sample_rate".into(), Value::U64(48_000));
        args.insert("frames".into(), Value::U64(frames));
        args.insert("amplitude".into(), Value::F64(0.3));
        host.invoke("hypermedia.acoustics.synthesize_tone", &Value::Record(args))
            .expect("synthesize_tone")
    }

    #[test]
    fn test_encode_aac_qac_v1_canonical_and_metadata() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = test_host(&cancel, &limits);
        let audio = synth_tone(&mut host, 256);

        let mut args = Args::new();
        args.insert("audio".into(), audio);
        args.insert("format".into(), Value::String("qac".into()));
        args.insert("species_id".into(), Value::String("wallabia-bicolor".into()));
        args.insert("cue_profile".into(), Value::String("vocal_contact".into()));

        let res_a = host
            .acoustics_ext("hypermedia.acoustics.encode_aac", &args)
            .expect("encode_aac format=qac");
        let res_b = host
            .acoustics_ext("hypermedia.acoustics.encode_aac", &args)
            .expect("encode_aac format=qac repeat");

        let rec = record(&res_a).expect("qac handle record");
        assert_eq!(rec.get("type"), Some(&Value::String("qac".into())));
        assert_eq!(rec.get("valid"), Some(&Value::Bool(true)));
        assert_eq!(rec.get("container_version"), Some(&Value::U64(1)));
        assert_eq!(rec.get("entry_count"), Some(&Value::U64(4)));
        assert_eq!(
            rec.get("species_id"),
            Some(&Value::String("wallabia-bicolor".into()))
        );
        assert_eq!(
            rec.get("cue_profile"),
            Some(&Value::String("vocal_contact".into()))
        );

        let res_a_bytes = host.workspace.encoded(&res_a).expect("qac resource a");
        let res_b_bytes = host.workspace.encoded(&res_b).expect("qac resource b");
        assert_eq!(res_a_bytes.kind, "qac");
        assert_eq!(&res_a_bytes.bytes[0..8], b"qac\0\0\0\0\0");
        assert_eq!(res_a_bytes.bytes, res_b_bytes.bytes, "QAC encoding must be deterministic");

        let view = StructuralView::open(
            &res_a_bytes.bytes,
            Family::QacV1Candidate,
            StorageLimits::default(),
        )
        .expect("StructuralView::open");
        assert_eq!(view.entry_count(), 4);
    }

    #[test]
    fn test_encode_aac_preserves_default_aac_and_rejects_unknown_format() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = test_host(&cancel, &limits);
        let audio = synth_tone(&mut host, 2048);

        let mut default_args = Args::new();
        default_args.insert("audio".into(), audio.clone());
        let aac_res = host
            .acoustics_ext("hypermedia.acoustics.encode_aac", &default_args)
            .expect("default encode_aac");
        assert_eq!(host.workspace.encoded(&aac_res).unwrap().kind, "aac");

        let mut explicit_aac = default_args.clone();
        explicit_aac.insert("format".into(), Value::String("aac".into()));
        let explicit_res = host
            .acoustics_ext("hypermedia.acoustics.encode_aac", &explicit_aac)
            .expect("explicit aac");
        assert_eq!(host.workspace.encoded(&explicit_res).unwrap().kind, "aac");

        let mut bad_format = Args::new();
        bad_format.insert("audio".into(), audio.clone());
        bad_format.insert("format".into(), Value::String("opus".into()));
        assert!(host
            .acoustics_ext("hypermedia.acoustics.encode_aac", &bad_format)
            .is_err());

        let mut empty_species = Args::new();
        empty_species.insert("audio".into(), audio);
        empty_species.insert("format".into(), Value::String("qac".into()));
        empty_species.insert("species_id".into(), Value::String("   ".into()));
        assert!(host
            .acoustics_ext("hypermedia.acoustics.encode_aac", &empty_species)
            .is_err());
    }
}
