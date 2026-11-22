//! SDR visual accessibility review on admitted decoded radiance lattices.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::VecDeque;
use std::mem::size_of;
use std::sync::Arc;

use crate::artifact::{LatticeSamples, RadianceLattice};
use crate::{fields, integer, list, num, required, text, Args, AssetHost, TypedArtifact, Value};

use super::{review_fact, review_graph, review_number, REVIEW_NS};

const LINEAR: &str = "urn:qualia:color:transfer:linear";
const MAX_PIXELS: usize = 8 * 1024 * 1024;
const MAX_FRAME_PIXELS: usize = 96 * 1024 * 1024;
type Matrix = [[f64; 3]; 3];

const IDENTITY: Matrix = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];

impl AssetHost<'_> {
    pub(crate) fn certify_pse_cvd_and_plain_language(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        match text(required(args, "mode")?)? {
            "simulate_cvd" => self.review_cvd(args),
            "screen_sdr_flashes" => self.review_flashes(args),
            "plain_language_metrics" => self.review_plain_language(args),
            _ => Err("Select simulate_cvd, screen_sdr_flashes, or plain_language_metrics".into()),
        }
    }

    fn review_cvd(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["mode", "image", "deficiency", "severity"])?;
        let deficiency = text(required(args, "deficiency")?)?;
        let severity = num(required(args, "severity")?)?;
        if !(0.0..=1.0).contains(&severity) {
            return Err("CVD severity must be in [0, 1]".into());
        }
        let matrix = simulation_matrix(deficiency, severity)?;
        let input = required(args, "image")?;
        let (n, channels) = {
            let image = lattice(self, input)?;
            let pixels = LinearPixels::new(image)?;
            (pixels.len(), image.channels as usize)
        };
        self.charge(n * 32, n * channels * size_of::<f32>())?;
        let (result, clipped) = {
            let image = lattice(self, input)?;
            let pixels = LinearPixels::new(image)?;
            let mut samples = Vec::with_capacity(n * channels);
            let mut clipped = 0;
            for pixel in pixels.iter() {
                let pixel = pixel?;
                let before_clip = matrix.map(|row| dot(row, pixel));
                clipped += usize::from(before_clip.into_iter().any(|v| !(0.0..=1.0).contains(&v)));
                samples.extend(before_clip.map(|v| v.clamp(0.0, 1.0) as f32));
                if channels == 4 {
                    samples.push(1.0);
                }
            }
            (
                RadianceLattice {
                    width: image.width,
                    height: image.height,
                    channels: image.channels,
                    primaries_iri: image.primaries_iri.clone(),
                    transfer_iri: image.transfer_iri.clone(),
                    samples: LatticeSamples::F32(samples),
                },
                clipped,
            )
        };
        let image = self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(result)))?;
        let mut graph = review_graph("CvdSimulation")?;
        review_fact(&mut graph, "model", "https://doi.org/10.1109/TVCG.2009.113")?;
        review_fact(
            &mut graph,
            "deficiency",
            &format!("{REVIEW_NS}{deficiency}"),
        )?;
        review_fact(
            &mut graph,
            "severityBinary64",
            &format!("urn:ieee754:binary64:{:016x}", severity.to_bits()),
        )?;
        review_number(&mut graph, "clippedPixels", clipped)?;
        Ok(Value::Record(Args::from([
            ("image".into(), image),
            ("clipped_pixels".into(), Value::U64(clipped as u64)),
            ("receipt".into(), self.admit_review(graph)?),
        ])))
    }

    fn review_flashes(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &["mode", "frames", "fps_numerator", "fps_denominator"],
        )?;
        let handles = list(required(args, "frames")?)?;
        if !(2..=2048).contains(&handles.len()) {
            return Err("SDR flash review requires 2 to 2,048 timestamp-ordered frames".into());
        }
        let fps_num = integer(required(args, "fps_numerator")?, "fps_numerator")?;
        let fps_den = args
            .get("fps_denominator")
            .map(|v| integer(v, "fps_denominator"))
            .transpose()?
            .unwrap_or(1);
        if !(1..=100_000).contains(&fps_den) || !(fps_den..=fps_den * 120).contains(&fps_num) {
            return Err(
                "SDR flash review requires a constant frame rate of 1 to 120 frames per second"
                    .into(),
            );
        }
        let first = lattice(self, &handles[0])?;
        let (width, height) = (first.width, first.height);
        let total_pixels = LinearPixels::new(first)?
            .len()
            .checked_mul(handles.len())
            .filter(|n| *n <= MAX_FRAME_PIXELS)
            .ok_or("SDR flash review exceeds its pixel budget")?;
        self.charge(total_pixels.saturating_mul(100), handles.len() * 32)?;

        let mut luminance = RollingTransitions::default();
        let mut red = RollingTransitions::default();
        let mut any = RollingTransitions::default();
        for (index, pair) in handles.windows(2).enumerate() {
            if index % 32 == 0 {
                self.charge(0, 0)?;
            }
            let before = lattice(self, &pair[0])?;
            let after = lattice(self, &pair[1])?;
            if (after.width, after.height) != (width, height)
                || (before.width, before.height) != (width, height)
            {
                return Err("All SDR frames must have the same raster geometry".into());
            }
            let mut general_change = false;
            let mut red_change = false;
            for (a, b) in LinearPixels::new(before)?
                .iter()
                .zip(LinearPixels::new(after)?.iter())
            {
                let (a, b) = (a?, b?);
                let (luma_a, luma_b) = (
                    dot([0.2126, 0.7152, 0.0722], a),
                    dot([0.2126, 0.7152, 0.0722], b),
                );
                general_change |= (luma_a - luma_b).abs() >= 0.1 && luma_a.min(luma_b) < 0.8;
                red_change |= saturated_red_change(a, b);
            }
            luminance.observe(index, general_change, fps_num, fps_den);
            red.observe(index, red_change, fps_num, fps_den);
            any.observe(index, general_change || red_change, fps_num, fps_den);
        }
        // This is an intentionally conservative transition screen. It counts a
        // transition at any viewing area and does not classify opposing pairs.
        let status = if any.peak > 6 {
            "review_required"
        } else {
            "no_trigger_in_provided_frames"
        };
        let mut graph = review_graph("SdrFlashTransitionScreen")?;
        review_fact(
            &mut graph,
            "sourceCriteria",
            "https://www.w3.org/WAI/WCAG22/Understanding/three-flashes-or-below-threshold",
        )?;
        review_fact(
            &mut graph,
            "classification",
            &format!("{REVIEW_NS}{status}"),
        )?;
        review_fact(
            &mut graph,
            "comparisonScope",
            &format!("{REVIEW_NS}AdjacentSdrFrames"),
        )?;
        review_number(&mut graph, "frames", handles.len())?;
        review_number(
            &mut graph,
            "peakLuminanceTransitionsPerSecond",
            luminance.peak,
        )?;
        review_number(&mut graph, "peakRedTransitionsPerSecond", red.peak)?;
        review_number(&mut graph, "peakTotalTransitionsPerSecond", any.peak)?;
        Ok(Value::Record(Args::from([
            ("status".into(), Value::String(status.into())),
            (
                "comparison_scope".into(),
                Value::String("adjacent_sdr_frames".into()),
            ),
            ("conformance_assessed".into(), Value::Bool(false)),
            (
                "peak_luminance_transitions_per_second".into(),
                Value::U64(luminance.peak as u64),
            ),
            (
                "peak_red_transitions_per_second".into(),
                Value::U64(red.peak as u64),
            ),
            (
                "peak_total_transitions_per_second".into(),
                Value::U64(any.peak as u64),
            ),
            ("receipt".into(), self.admit_review(graph)?),
        ])))
    }

    fn review_plain_language(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["mode", "text", "long_sentence_words"])?;
        let text = text(required(args, "text")?)?;
        let limit = args
            .get("long_sentence_words")
            .map(|value| integer(value, "long_sentence_words"))
            .transpose()?
            .unwrap_or(25);
        if !(1..=100).contains(&limit) || text.len() > 256 * 1024 || text.trim().is_empty() {
            return Err("Plain language metrics require nonempty bounded text and a word threshold from 1 to 100".into());
        }
        self.charge(text.len() * 4, 0)?;
        let mut sentence_count = 0;
        let mut word_count = 0;
        let mut over_threshold = 0;
        // Punctuation and word counts are editorial indicators rather than a
        // language-specific cognitive or readability score.
        for sentence in text.split(['.', '?', '!', '\u{3002}', '\u{ff01}', '\u{ff1f}']) {
            let count = sentence
                .split_whitespace()
                .filter(|word| word.chars().any(char::is_alphanumeric))
                .count();
            if count == 0 {
                continue;
            }
            word_count += count;
            sentence_count += 1;
            over_threshold += usize::from(count > limit);
        }
        if sentence_count == 0 {
            return Err("No words found by the whitespace-delimited text metric".into());
        }
        let mut graph = review_graph("PlainLanguageTextMetrics")?;
        review_number(&mut graph, "sentences", sentence_count)?;
        review_number(&mut graph, "words", word_count)?;
        review_number(&mut graph, "longSentenceWordThreshold", limit)?;
        review_number(&mut graph, "sentencesOverThreshold", over_threshold)?;
        Ok(Value::Record(Args::from([
            (
                "mean_words_per_sentence".into(),
                Value::F64(word_count as f64 / sentence_count as f64),
            ),
            (
                "sentences_over_threshold".into(),
                Value::U64(over_threshold as u64),
            ),
            ("conformance_assessed".into(), Value::Bool(false)),
            ("receipt".into(), self.admit_review(graph)?),
        ])))
    }
}

pub(super) fn lattice<'host>(
    host: &'host AssetHost<'_>,
    value: &Value,
) -> Result<&'host RadianceLattice, String> {
    match host.workspace.typed(value)? {
        TypedArtifact::RadianceLattice(image) => Ok(image.as_ref()),
        _ => Err("Expected a decoded RadianceLattice handle".into()),
    }
}

pub(super) struct LinearPixels<'a> {
    channels: usize,
    samples: &'a [f32],
}

impl<'a> LinearPixels<'a> {
    pub(super) fn new(image: &'a RadianceLattice) -> Result<Self, String> {
        let channels = image.channels as usize;
        if !matches!(channels, 3 | 4)
            || image.transfer_iri != LINEAR
            || !matches!(
                image.primaries_iri.as_str(),
                "urn:qualia:color:primaries:rec709" | "http://www.w3.org/ns/ttml/parameter/rec709"
            )
        {
            return Err(
                "Accessibility review requires normalized linear Rec.709 RGB or opaque RGBA".into(),
            );
        }
        let LatticeSamples::F32(samples) = &image.samples else {
            return Err("Accessibility review requires decoded F32 samples".into());
        };
        let count = (image.width as usize)
            .checked_mul(image.height as usize)
            .filter(|count| (1..=MAX_PIXELS).contains(count))
            .ok_or("Raster exceeds the review pixel budget")?;
        if samples.len() != count * channels {
            return Err("Raster samples do not match the declared geometry".into());
        }
        Ok(Self { channels, samples })
    }

    pub(super) fn len(&self) -> usize {
        self.samples.len() / self.channels
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = Result<[f64; 3], String>> + '_ + use<'_, 'a> {
        self.samples
            .chunks_exact(self.channels)
            .map(Self::normalized_rgb)
    }

    pub(super) fn pixel(&self, index: usize) -> Result<[f64; 3], String> {
        if index >= self.len() {
            return Err("Pixel index is out of range".into());
        }
        let offset = index * self.channels;
        Self::normalized_rgb(&self.samples[offset..offset + self.channels])
    }

    fn normalized_rgb(pixel: &[f32]) -> Result<[f64; 3], String> {
        if pixel.get(3).is_some_and(|&alpha| alpha != 1.0)
            || pixel[..3].iter().any(|v| !(0.0..=1.0).contains(v))
        {
            return Err(
                "Every SDR sample must be finite and normalized, and every pixel opaque".into(),
            );
        }
        Ok([
            f64::from(pixel[0]),
            f64::from(pixel[1]),
            f64::from(pixel[2]),
        ])
    }
}

// Rounded anchors from Machado (2010), Appendix A Table A.1, based on
// Machado, Oliveira and Fernandes (2009). Interpolate between 0, 0.5, and 1.
fn simulation_matrix(deficiency: &str, severity: f64) -> Result<Matrix, String> {
    let (middle, full) = match deficiency {
        "protan" => (
            [
                [0.458, 0.680, -0.138],
                [0.093, 0.846, 0.061],
                [-0.007, -0.017, 1.024],
            ],
            [
                [0.152, 1.053, -0.205],
                [0.115, 0.786, 0.099],
                [-0.004, -0.048, 1.052],
            ],
        ),
        "deutan" => (
            [
                [0.547, 0.608, -0.155],
                [0.182, 0.782, 0.037],
                [-0.010, 0.027, 0.983],
            ],
            [
                [0.367, 0.861, -0.228],
                [0.280, 0.673, 0.047],
                [-0.012, 0.043, 0.969],
            ],
        ),
        "tritan" => (
            [
                [1.017, 0.027, -0.044],
                [-0.006, 0.958, 0.048],
                [0.006, 0.249, 0.745],
            ],
            [
                [1.256, -0.077, -0.179],
                [-0.078, 0.931, 0.148],
                [0.005, 0.691, 0.304],
            ],
        ),
        _ => return Err("CVD deficiency must be protan, deutan, or tritan".into()),
    };
    let (a, b, weight) = if severity <= 0.5 {
        (IDENTITY, middle, severity * 2.)
    } else {
        (middle, full, (severity - 0.5) * 2.)
    };
    // The tabulated coefficients have three decimals. Normalize rounded row sums
    // so neutral gray remains neutral at every interpolated severity.
    Ok(std::array::from_fn(|i| {
        let row: [f64; 3] = std::array::from_fn(|j| a[i][j] * (1. - weight) + b[i][j] * weight);
        let sum = row.iter().sum::<f64>();
        row.map(|x| x / sum)
    }))
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn saturated_red_change(a: [f64; 3], b: [f64; 3]) -> bool {
    let is_saturated_red = |sample: [f64; 3]| {
        let total = sample.iter().sum::<f64>();
        total > 0.0 && sample[0] >= 0.8 * total
    };
    if !is_saturated_red(a) && !is_saturated_red(b) {
        return false;
    }
    match (cie_uv(a), cie_uv(b)) {
        (Some([u0, v0]), Some([u1, v1])) => (u0 - u1).powi(2) + (v0 - v1).powi(2) > 0.04,
        // Chromaticity is undefined at black. Escalate a transition involving
        // saturated red instead of assuming it falls below threshold.
        _ => true,
    }
}

fn cie_uv(rgb: [f64; 3]) -> Option<[f64; 2]> {
    let x = dot([0.4124564, 0.3575761, 0.1804375], rgb);
    let y = dot([0.2126729, 0.7151522, 0.0721750], rgb);
    let z = dot([0.0193339, 0.1191920, 0.9503041], rgb);
    let denominator = x + 15.0 * y + 3.0 * z;
    (denominator > 0.0).then(|| [4.0 * x / denominator, 9.0 * y / denominator])
}

#[derive(Default)]
struct RollingTransitions {
    window: VecDeque<usize>,
    peak: usize,
}

impl RollingTransitions {
    fn observe(&mut self, index: usize, change: bool, numerator: usize, denominator: usize) {
        while self
            .window
            .front()
            .is_some_and(|first| (index - first) * denominator > numerator)
        {
            self.window.pop_front();
        }
        if change {
            self.window.push_back(index);
        }
        self.peak = self.peak.max(self.window.len());
    }
}
