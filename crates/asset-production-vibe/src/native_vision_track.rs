//! Deterministic image feature tracks with content-addressed Q42 evidence.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::graphics::Rgba;
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::*;
use kinetics_motion_compositor::{
    GrayImage, ImagePyramid, detect_shi_tomasi_corners, track_klt_point,
};
use sha2::{Digest, Sha256};

const VISION: &str = "urn:qualia:vision:";
const TRACK_WINDOW: usize = 3;

struct FeatureTrack {
    id: usize,
    initial_score: f64,
    samples_px: Vec<Option<[f64; 2]>>,
}

impl AssetHost<'_> {
    pub(crate) fn process_and_track(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &["op", "frames", "max_features", "roundtrip_limit_px"],
        )?;
        if text(required(args, "op")?)? != "optical_flow_klt" {
            return Err("This adapter accepts optical_flow_klt".into());
        }
        let frames = list(required(args, "frames")?)?;
        if !(2..=120).contains(&frames.len()) {
            return Err("KLT requires 2 to 120 retained RGBA8 image handles".into());
        }
        let max_features = count(args, "max_features", 48, 256)?;
        if max_features == 0 {
            return Err("KLT requires at least one feature".into());
        }
        let roundtrip_limit = args
            .get("roundtrip_limit_px")
            .map(num)
            .transpose()?
            .unwrap_or(0.75);
        if !(0.05..=3.0).contains(&roundtrip_limit) {
            return Err("KLT round-trip threshold must be within 0.05 to 3 pixels".into());
        }
        let (width, height, initial, first_hash) = {
            let first = self.rgba(&frames[0])?;
            let (width, height) = (first.width as usize, first.height as usize);
            if !(24..=2048).contains(&width) || !(24..=2048).contains(&height) {
                return Err("KLT frame dimensions must each be within 24 to 2048 pixels".into());
            }
            (width, height, rgba_luma(first)?, frame_digest(first))
        };
        let pixels = width * height;
        let links = frames.len() - 1;
        self.charge(
            pixels
                .saturating_mul(128)
                .saturating_add(links.saturating_mul(pixels * 16 + max_features * 2 * 3 * 49 * 24)),
            pixels
                .saturating_mul(52)
                .saturating_add(frames.len() * max_features * 160),
        )?;

        let within = |[x, y]: [f64; 2]| {
            x >= 5.0 && y >= 5.0 && x < (width - 5) as f64 && y < (height - 5) as f64
        };
        let selected = detect_shi_tomasi_corners(&initial, max_features * 2, 0.02, 5.0);
        let mut tracks = selected
            .into_iter()
            .filter(|feature| feature.score > 1e-6 && within([feature.x, feature.y]))
            .take(max_features)
            .enumerate()
            .map(|(id, feature)| FeatureTrack {
                id,
                initial_score: feature.score,
                samples_px: vec![Some([feature.x, feature.y])],
            })
            .collect::<Vec<_>>();
        if tracks.is_empty() {
            return Err("KLT found no trackable interior corners in the first frame".into());
        }

        let mut source_hashes = vec![first_hash];
        let mut previous = ImagePyramid::build(&initial, 3);
        let mut accepted_links = 0;
        for frame in &frames[1..] {
            let image = self.rgba(frame)?;
            if image.width as usize != width || image.height as usize != height {
                return Err("KLT frames must have matching extents".into());
            }
            let current = ImagePyramid::build(&rgba_luma(image)?, 3);
            source_hashes.push(frame_digest(image));
            for track in &mut tracks {
                let point = track
                    .samples_px
                    .last()
                    .copied()
                    .flatten()
                    .and_then(|start| {
                        let end = track_one(&previous, &current, start).filter(|p| within(*p))?;
                        let back = track_one(&current, &previous, end).filter(|p| within(*p))?;
                        let error = (back[0] - start[0]).hypot(back[1] - start[1]);
                        (error <= roundtrip_limit
                            && patch_matches(&previous.levels[0], &current.levels[0], start, end))
                        .then_some(end)
                    });
                accepted_links += usize::from(point.is_some());
                track.samples_px.push(point);
            }
            previous = current;
        }
        if accepted_links == 0 {
            return Err("KLT found no links within the round-trip threshold".into());
        }

        let tracks_json: Vec<_> = tracks
            .iter()
            .map(|t| {
                serde_json::json!({
                    "id": t.id,
                    "initial_score": t.initial_score,
                    "samples_px": t.samples_px,
                })
            })
            .collect();
        let report = serde_json::to_vec(&serde_json::json!({
            "schema": "qualia-image-feature-track/1", "width": width, "height": height,
            "source_sha256": source_hashes, "tracks": tracks_json, "roundtrip_limit_px": roundtrip_limit,
            "accepted_links": accepted_links,
        }))
        .map_err(|err| err.to_string())?;
        let mut receipt = ReceiptGraph::new("urn:qualia:vision:optical-flow-evidence");
        let result = format!("urn:sha256:{:x}", Sha256::digest(&report));
        receipt.fact(
            &result,
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
            &format!("{VISION}KltTrackSet"),
        )?;
        for (index, digest) in source_hashes.iter().enumerate() {
            let binding = format!("{result}:frame:{index}");
            receipt.fact(&result, &format!("{VISION}sourceFrame"), &binding)?;
            receipt.fact(
                &binding,
                "http://www.w3.org/ns/prov#wasDerivedFrom",
                &format!("urn:sha256:{digest}"),
            )?;
        }
        let evidence = receipt.seal(self.limits.resource_bytes)?;
        let tracks = self.encode("json", report)?;
        let evidence = self.encode("q42", evidence)?;
        Ok(Value::Record(Args::from([
            ("tracks".into(), tracks),
            ("evidence".into(), evidence),
        ])))
    }
}

fn track_one(from: &ImagePyramid, to: &ImagePyramid, point: [f64; 2]) -> Option<[f64; 2]> {
    let (x, y, success) = track_klt_point(from, to, point[0], point[1], TRACK_WINDOW, 24)?;
    (success && x.is_finite() && y.is_finite()).then_some([x, y])
}

fn patch_matches(from: &GrayImage, to: &GrayImage, a: [f64; 2], b: [f64; 2]) -> bool {
    let (mut ab, mut aa, mut bb, mut sum_a, mut sum_b) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for y in -2..=2 {
        for x in -2..=2 {
            let left = sample_luma(from, a[0] + f64::from(x), a[1] + f64::from(y));
            let right = sample_luma(to, b[0] + f64::from(x), b[1] + f64::from(y));
            ab += left * right;
            aa += left * left;
            bb += right * right;
            sum_a += left;
            sum_b += right;
        }
    }
    let variance_a = aa - sum_a * sum_a / 25.0;
    let variance_b = bb - sum_b * sum_b / 25.0;
    let covariance = ab - sum_a * sum_b / 25.0;
    variance_a > 1e-5
        && variance_b > 1e-5
        && covariance >= 0.75 * f64::sqrt(variance_a * variance_b)
}

fn sample_luma(image: &GrayImage, x: f64, y: f64) -> f64 {
    let (ix, iy) = (x.floor() as usize, y.floor() as usize);
    let (u, v) = (x - ix as f64, y - iy as f64);
    let a = f64::from(image.get(ix, iy)) * (1.0 - u) + f64::from(image.get(ix + 1, iy)) * u;
    let b = f64::from(image.get(ix, iy + 1)) * (1.0 - u) + f64::from(image.get(ix + 1, iy + 1)) * u;
    a * (1.0 - v) + b * v
}

fn rgba_luma(image: &Rgba) -> Result<GrayImage, String> {
    if image.pixels.len() != image.width as usize * image.height as usize
        || image.pixels.iter().any(|pixel| pixel[3] != 255)
    {
        return Err("KLT requires a full opaque RGBA8 frame".into());
    }
    Ok(GrayImage::from_luma(
        image.width as usize,
        image.height as usize,
        image.pixels.as_flattened(),
    ))
}

fn frame_digest(image: &Rgba) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"qualia:rgba8:dimensions-le:v1\0");
    hasher.update(image.width.to_le_bytes());
    hasher.update(image.height.to_le_bytes());
    hasher.update(image.pixels.as_flattened());
    format!("{:x}", hasher.finalize())
}
