//! Color-managed image ingest and scopes.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::*;
use color_transforms::Transfer;
use color_transforms::icc::{Builtin, ColorSpace, Curve, Intent, Profile, Transform, math, srgb};
use radiance_color_grading::LogCurve;
use radiance_color_grading::scopes::{
    ParadeType, Params, Signal, WaveformType, parade, vectorscope_yuv, waveform,
};
use radiance_color_grading::spaces::Gamut;

impl AssetHost<'_> {
    pub(crate) fn compute_scopes_and_icc(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "image",
                "icc",
                "camera_log",
                "black_point_compensation",
                "bins",
            ],
        )?;
        let image = required(args, "image")?;
        let (width, height, count, transfer) = match self.workspace.get(image)? {
            Artifact::Rgba(img) => (img.width, img.height, img.pixels.len(), None),
            Artifact::Rgba16(img) => (img.width, img.height, img.pixels.len(), img.transfer),
            _ => return Err("Scopes require an RGBA8 or RGBA16 image".into()),
        };
        let bins = super::count(args, "bins", 64, 256)?;
        if count == 0 || bins < 2 {
            return Err("Scopes require a nonempty image and at least two bins".into());
        }
        let camera = args
            .get("camera_log")
            .map(text)
            .transpose()?
            .map(camera_log)
            .transpose()?;
        let bpc = boolean(args, "black_point_compensation", true)?;
        if (camera.is_some() || args.contains_key("icc")) && transfer.is_some() {
            return Err(
                "ICC or camera-log ingest requires an image with unspecified transfer metadata"
                    .into(),
            );
        }
        if camera.is_some() && args.contains_key("icc") {
            return Err("Select a camera-log source or an ICC source profile".into());
        }
        let scope_width = (width as usize).min(512);
        let scope_height = (height as usize).min(512);
        let grid_cells = scope_width * bins * 6 + bins * bins;
        self.charge(
            count.saturating_mul(128).saturating_add(grid_cells),
            count
                .saturating_mul(8)
                .saturating_add(grid_cells * 32 + scope_width * scope_height * 12),
        )?;

        let source =
            match (camera, args.get("icc"), transfer) {
                (Some((_, gamut)), _, _) => linear_camera_profile(gamut),
                (_, Some(key), _) => {
                    let resource = encoded_resource(self, key)?;
                    if resource.kind != "icc" || resource.bytes.len() > 4 * 1024 * 1024 {
                        return Err("Expected a retained ICC profile of at most 4 MiB".into());
                    }
                    self.charge(
                        resource.bytes.len().saturating_mul(4),
                        resource.bytes.len().saturating_mul(4),
                    )?;
                    Profile::parse(&encoded_resource(self, key)?.bytes)
                        .map_err(|err| format!("ICC profile: {err}"))?
                }
                (_, _, Some(Transfer::Linear)) => Builtin::LinearSrgb.profile().clone(),
                (_, _, None | Some(Transfer::Srgb)) => srgb().clone(),
                _ => return Err(
                    "Convert the tagged image to sRGB or linear sRGB before plotting RGB scopes"
                        .into(),
                ),
            };
        if source.color_space != ColorSpace::Rgb {
            return Err("An RGBA source requires an RGB ICC profile".into());
        }
        let lab = Builtin::LabD50.profile();
        let convert = Transform::new(&source, srgb(), Intent::RelativeColorimetric, bpc)
            .map_err(|err| format!("ICC conversion: {err}"))?;
        let as_lab = |profile: &Profile| {
            Transform::new(profile, lab, Intent::RelativeColorimetric, false)
                .map_err(|err| format!("ICC Lab transform: {err}"))
        };
        let source_lab = as_lab(&source)?;
        let target_lab = as_lab(srgb())?;
        let mut pixels = Vec::with_capacity(count);
        let mut mean_delta = 0.0;
        let mut max_delta = 0.0_f64;
        let image = self.workspace.get(image)?;
        for index in 0..count {
            let pixel = match image {
                Artifact::Rgba(img) => img.pixels[index].map(|v| u16::from(v) * 257),
                Artifact::Rgba16(img) => img.pixels[index],
                _ => unreachable!("image kind checked before conversion"),
            };
            let linear_or_encoded = [pixel[0], pixel[1], pixel[2]].map(|v| {
                let signal = f64::from(v) / 65535.0;
                camera.map_or(signal, |(curve, _)| curve.decode(signal)) as f32
            });
            let mut rgb = [0.0; 3];
            convert.eval(&linear_or_encoded, &mut rgb);
            if rgb.iter().any(|v| !v.is_finite()) {
                return Err("ICC conversion produced a non-finite RGB component".into());
            }
            let quantized = rgb.map(|v| (v.clamp(0.0, 1.0) * 65535.0).round() as u16);
            let mut before = [0.0; 3];
            let mut after = [0.0; 3];
            source_lab.eval(&linear_or_encoded, &mut before);
            target_lab.eval(&quantized.map(|v| f32::from(v) / 65535.0), &mut after);
            let cie_lab = |v: [f32; 3]| {
                [
                    f64::from(v[0]) * 100.0,
                    f64::from(v[1]) * 255.0 - 128.0,
                    f64::from(v[2]) * 255.0 - 128.0,
                ]
            };
            let delta = math::delta_e76(cie_lab(before), cie_lab(after));
            if !delta.is_finite() {
                return Err("ICC conversion produced a non-finite Delta E76".into());
            }
            mean_delta += delta;
            max_delta = max_delta.max(delta);
            pixels.push([quantized[0], quantized[1], quantized[2], pixel[3]]);
        }

        let signal = Signal::from_fn(scope_width, scope_height, |x, y| {
            let index = y * height as usize / scope_height * width as usize
                + x * width as usize / scope_width;
            let px = pixels[index];
            [px[0], px[1], px[2]].map(|v| f32::from(v) / 65535.0)
        });
        let params = Params {
            rows: bins,
            vector_size: bins,
            ..Default::default()
        };
        let vector = vectorscope_yuv(&signal, &params);
        let report = serde_json::json!({
            "schema": "qualia-rgb-scopes/1",
            "output_profile": "srgb",
            "black_point_compensation": bpc,
            "sampled_dimensions": [signal.w, signal.h],
            "waveform": waveform(&signal, WaveformType::Rgb, &params).traces,
            "parade": parade(&signal, ParadeType::Rgb, &params).traces,
            "vectorscope": { "size": vector.size, "extent": vector.extent, "counts": vector.data },
            "delta_e76": { "count": count, "mean": mean_delta / count as f64, "max": max_delta },
        });
        let report = self.encode(
            "json",
            serde_json::to_vec(&report).map_err(|err| err.to_string())?,
        )?;
        let image = self.store_rgba16(width, height, pixels, Some(Transfer::Srgb))?;
        Ok(Value::Record(BTreeMap::from([
            ("image".into(), image),
            ("scopes".into(), report),
        ])))
    }
}

fn camera_log(name: &str) -> Result<(LogCurve, Gamut), String> {
    match name {
        "logc4" => Ok((LogCurve::LogC4, Gamut::Awg4)),
        "slog3-sgamut3" => Ok((LogCurve::SLog3, Gamut::SGamut3)),
        "slog3-sgamut3cine" => Ok((LogCurve::SLog3, Gamut::SGamut3Cine)),
        "acescc" => Ok((LogCurve::AcesCc, Gamut::AcesAp1)),
        "acescct" => Ok((LogCurve::AcesCct, Gamut::AcesAp1)),
        _ => Err("Unknown camera log and gamut pairing".into()),
    }
}

fn linear_camera_profile(gamut: Gamut) -> Profile {
    let mut profile = Builtin::LinearSrgb.profile().clone();
    let white = math::xy_to_xyz(gamut.white_point());
    profile.matrix = Some(math::mul(
        &math::bradford(white, math::D50),
        &gamut.to_xyz(),
    ));
    profile.trc = Some([Curve::Identity, Curve::Identity, Curve::Identity]);
    profile
}
