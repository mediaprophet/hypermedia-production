use super::*;
use crate::electromagnetics_ops::emf_cell_display_irradiance;
use qualia_core_db::q42::emf_container::{EmfContainerReader, SPEED_OF_LIGHT};

impl AssetHost<'_> {
    /// Resolves sealed `.emf` bytes from a retained key or encoded handle.
    fn resolve_emf_bytes_for_sonic(&self, val: &Value) -> Result<Vec<u8>, String> {
        if let Ok(key) = text(val) {
            self.inputs
                .iter()
                .find(|r| r.key == key)
                .map(|r| r.bytes.clone())
                .ok_or_else(|| format!("Missing retained input: {key}"))
        } else if let Ok(res) = self.workspace.encoded(val) {
            if res.kind != "emf" {
                return Err(format!(
                    "Expected an 'emf' resource handle, found '{}'",
                    res.kind
                ));
            }
            Ok(res.bytes.clone())
        } else {
            Err("Expected retained resource key or encoded handle for emf".into())
        }
    }

    /// Heterodyne optical wavelength onto A4 (532 nm → 440 Hz) so the field is hearable.
    fn optical_nm_to_audio_hz(wavelength_nm: f64, nyquist: f64) -> f64 {
        if !(wavelength_nm > 0.0) {
            return 440.0;
        }
        let f_em = (SPEED_OF_LIGHT as f64) * 1e9 / wavelength_nm;
        (f_em / 1.281_82e12).clamp(20.0, nyquist)
    }

    /// Field-derived mono PCM: irradiance envelope in sealed chunk/cell order (no HashMap).
    fn sonify_emf_field(
        &self,
        emf_val: &Value,
        rate: u32,
        frames: usize,
        amp: f32,
    ) -> Result<Vec<f32>, String> {
        let bytes = self.resolve_emf_bytes_for_sonic(emf_val)?;
        let reader = EmfContainerReader::parse(&bytes)
            .map_err(|e| format!("Invalid EMF container: {e}"))?;

        let mut envelope: Vec<f32> = Vec::new();
        let mut irr_sum = 0.0f64;
        let mut wl_weight = 0.0f64;
        let mut max_i = 0.0f64;

        let chunk_count = reader.chunk_count();
        for chunk_idx in 0..chunk_count {
            let entry = reader
                .chunk_entry(chunk_idx)
                .ok_or_else(|| format!("Missing chunk directory entry at index {chunk_idx}"))?;
            let cell_count = entry.cell_count as usize;
            for cell_idx in 0..cell_count {
                let cell = reader.cell(chunk_idx, cell_idx).ok_or_else(|| {
                    format!("Missing cell {cell_idx} in chunk {chunk_idx}")
                })?;
                let i = emf_cell_display_irradiance(&cell);
                if i > max_i {
                    max_i = i;
                }
                envelope.push(i as f32);
                if i > 0.0 {
                    irr_sum += i;
                    let wl = if cell.wavelength_nm > 0.0 {
                        cell.wavelength_nm as f64
                    } else {
                        532.0
                    };
                    wl_weight += wl * i;
                }
                if envelope.len() >= 8192 {
                    break;
                }
            }
            if envelope.len() >= 8192 {
                break;
            }
        }

        if max_i <= 0.0 || envelope.is_empty() {
            return Err("emf field has no audible energy (black lattice)".into());
        }

        let nyquist = (rate as f64) * 0.49;
        let mean_wl = if irr_sum > 0.0 {
            wl_weight / irr_sum
        } else {
            532.0
        };
        let f0 = Self::optical_nm_to_audio_hz(mean_wl, nyquist) as f32;
        let peak_amp = ((1.0 - (-max_i).exp()) as f32 * amp).clamp(0.0, 1.0);
        let inv_max = (1.0 / max_i) as f32;
        let n_env = envelope.len();
        let sr = rate as f32;

        let mut samples = vec![0.0f32; frames];
        for i in 0..frames {
            let env = envelope[i % n_env] * inv_max;
            let t = i as f32 / sr;
            let s = env * peak_amp * (std::f32::consts::TAU * f0 * t).sin();
            samples[i] = if s.is_finite() { s.clamp(-1.0, 1.0) } else { 0.0 };
        }
        Ok(samples)
    }

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
                fields(a, &["frequency", "sample_rate", "frames", "amplitude", "emf"])?;
                let rate = count(a, "sample_rate", 48000, 192000)? as u32;
                if rate < 8000 {
                    return Err("Sample rate must be 8000..192000".into());
                }
                let frames = count(a, "frames", 48000, 1_000_000)?;
                let freq = number(a, "frequency", 440.)?;
                let amp = number(a, "amplitude", 0.25)?;
                let has_emf = a.get("emf").is_some();
                if !has_emf
                    && (freq <= 0.
                        || freq >= rate as f64 / 2.
                        || freq as f32 <= 0.
                        || freq as f32 >= rate as f32 / 2.
                        || !(0. ..=1.).contains(&amp))
                {
                    return Err("Tone needs 0 < frequency < Nyquist and amplitude 0..1".into());
                }
                if has_emf && !(0. ..=1.).contains(&amp) {
                    return Err("Amplitude must be 0..1".into());
                }
                self.charge(frames * 8, frames * 4)?;
                let samples = if let Some(emf_val) = a.get("emf") {
                    self.sonify_emf_field(emf_val, rate, frames, amp as f32)?
                } else {
                    qualia_audio::media_store::synth_tone_f32(
                        freq as f32,
                        rate,
                        frames,
                        amp as f32,
                    )
                };
                self.workspace
                    .insert(Artifact::Audio { rate, samples }, self.limits)
            }
            "hypermedia.acoustics.scale_gain"
            | "hypermedia.audio.gain"
            | "hypermedia.acoustics.encode_wav"
            | "hypermedia.audio.wav" => {
                let is_gain = id == "hypermedia.acoustics.scale_gain" || id == "hypermedia.audio.gain";
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
            Artifact::Audio { samples, .. } => samples
                .iter()
                .fold(0.0f32, |m, s| m.max(s.abs())),
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
                assert!(samples.iter().all(|s| s.is_finite() && *s >= -1.0 && *s <= 1.0));
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
            .acoustics_discourse_chronicle_ext_op(
                "hypermedia.acoustics.route_bus_dag",
                &bus_args,
            )
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
                assert!(samples.iter().any(|s| s.abs() > 1e-4), "spatial wave non-silent");
                assert!(samples.iter().all(|s| s.is_finite()));
            }
            _ => panic!("expected spatial Audio"),
        }
    }
}
