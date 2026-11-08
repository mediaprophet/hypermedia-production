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
                fields(a, &["frequency", "sample_rate", "frames", "amplitude"])?;
                let rate = count(a, "sample_rate", 48000, 192000)? as u32;
                if rate < 8000 {
                    return Err("Sample rate must be 8000..192000".into());
                }
                let frames = count(a, "frames", 48000, 1_000_000)?;
                let freq = number(a, "frequency", 440.)?;
                let amp = number(a, "amplitude", 0.25)?;
                if freq <= 0.
                    || freq >= rate as f64 / 2.
                    || freq as f32 <= 0.
                    || freq as f32 >= rate as f32 / 2.
                    || !(0. ..=1.).contains(&amp)
                {
                    return Err("Tone needs 0 < frequency < Nyquist and amplitude 0..1".into());
                }
                self.charge(frames * 8, frames * 4)?;
                let samples = qualia_audio::media_store::synth_tone_f32(
                    freq as f32,
                    rate,
                    frames,
                    amp as f32,
                );
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
