//! An explicitly chosen AUDIO carrier modulated by a normalized |E| envelope.
//! This is a derivative visualization in sound, not acoustic pressure or an EM
//! frequency/centroid/sigma conversion. Missing source states remain explicit.
use super::CurrentDocument;
enum Envelope {
    Missing,
    Zero,
    LogMagnitude(f64),
}
pub(crate) struct Sonic {
    pub samples: Vec<f32>,
    pub source_samples: usize,
    pub missing_samples: usize,
}
pub(crate) fn sonify(
    document: &CurrentDocument<'_>,
    rate: u32,
    frames: usize,
    frequency: f64,
    amplitude: f64,
    byte_budget: usize,
) -> Result<Sonic, String> {
    if !(8000..=192000).contains(&rate)
        || frames == 0
        || !frequency.is_finite()
        || frequency <= 0.0
        || frequency >= f64::from(rate) * 0.5
        || !amplitude.is_finite()
        || !(0.0..=1.0).contains(&amplitude)
    {
        return Err(
            "Sonification needs explicit finite acoustic frequency below Nyquist and amplitude0..1"
                .into(),
        );
    }
    let bytes = frames
        .checked_mul(4)
        .and_then(|n| n.checked_add(8192 * std::mem::size_of::<Envelope>()))
        .ok_or("Sonification size overflow")?;
    if bytes > byte_budget {
        return Err("Sonification exceeds selected byte budget".into());
    }
    let mut envelope = Vec::with_capacity(8192);
    let mut max_log = None::<f64>;
    let mut missing = 0;
    'chunks: for chunk in document.chunks.iter().filter(|c| c.mode() == 0) {
        for index in 0..chunk.record_count() {
            let sample = match chunk
                .field(index)
                .map_err(|e| format!("Sonification field index: {e:?}"))?
            {
                None => {
                    missing += 1;
                    Envelope::Missing
                }
                Some(record) => {
                    let e = record.electric_v_per_m();
                    let scale = e.iter().map(|x| x.abs()).fold(0.0, f64::max);
                    if scale == 0.0 {
                        Envelope::Zero
                    } else {
                        let e = e.map(|x| x / scale);
                        let log = scale.ln() + e[0].hypot(e[1]).hypot(e[2]).ln();
                        if !log.is_finite() {
                            return Err("Envelope log magnitude is not representable".into());
                        }
                        max_log = Some(max_log.map_or(log, |max| max.max(log)));
                        Envelope::LogMagnitude(log)
                    }
                }
            };
            envelope.push(sample);
            if envelope.len() == 8192 {
                break 'chunks;
            }
        }
    }
    if envelope.is_empty() || missing == envelope.len() {
        return Err("No available field observations for this sonification derivative".into());
    }
    let mut samples = Vec::with_capacity(frames);
    for index in 0..frames {
        let normalized = match envelope[index % envelope.len()] {
            Envelope::Missing | Envelope::Zero => 0.0,
            Envelope::LogMagnitude(log) => (log - max_log.ok_or("Missing envelope maximum")?).exp(),
        };
        let phase = std::f64::consts::TAU * frequency * (index as f64 / f64::from(rate));
        let value = (normalized * amplitude * phase.sin()) as f32;
        if !value.is_finite() {
            return Err("Audio derivative is not representable".into());
        }
        samples.push(value);
    }
    Ok(Sonic {
        samples,
        source_samples: envelope.len(),
        missing_samples: missing,
    })
}
