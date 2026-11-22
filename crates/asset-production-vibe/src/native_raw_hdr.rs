//! Native sensor demosaic, exposure fusion, and registered strip assembly.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use crate::artifact::{LatticeSamples, RadianceLattice};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::*;
use color_transforms::{Transfer, to_linear};
use radiance_camera_raw::{
    CfaPattern, DemosaicAlgo, ExposureBracket, GpxTracklog, HdrOptions, TrackPoint, demosaic_raw,
    merge_hdr, stitch_panorama,
};
use radiance_color_grading::spaces::{Gamut, Mat3, gamut_matrix, mul_vec};
use sha2::{Digest, Sha256};

impl AssetHost<'_> {
    pub(crate) fn develop_raw_hdr_panorama(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "frames",
                "mode",
                "width",
                "height",
                "camera_to_ap1",
                "overlap_px",
                "deghost_threshold",
                "track",
                "max_track_gap_s",
            ],
        )?;
        let frames = list(required(args, "frames")?)?;
        let mode = text(required(args, "mode")?)?;
        let width = count(args, "width", 0, 4096)?;
        let height = count(args, "height", 0, 4096)?;
        let overlap = if mode == "registered_strip" {
            count(args, "overlap_px", 0, width / 2)?
        } else {
            if args.contains_key("overlap_px") {
                return Err("Overlap is only used for registered strips".into());
            }
            0
        };
        if frames.len() > 16
            || match mode {
                "single" => frames.len() != 1,
                "hdr" | "registered_strip" => frames.len() < 2,
                _ => true,
            }
        {
            return Err(
                "Use single, hdr, or registered_strip with the required bounded frame count".into(),
            );
        }
        if mode != "hdr" && args.contains_key("deghost_threshold") {
            return Err("Deghosting is only used for exposure fusion".into());
        }
        let deghost = number(args, "deghost_threshold", 0.0)?;
        if !(0.0..=1.0).contains(&deghost) {
            return Err("HDR deghosting threshold must be in [0,1]".into());
        }
        let output_width = width * frames.len() - overlap * (frames.len() - 1);
        let output_width = if mode == "registered_strip" {
            output_width
        } else {
            width
        };
        if output_width > 16384 {
            return Err("Registered strip exceeds its output width budget".into());
        }
        let samples = width * height * frames.len();
        self.charge(samples.saturating_mul(220), samples.saturating_mul(40))?;
        let camera_matrix = args.get("camera_to_ap1").map(camera_matrix).transpose()?;
        let color_matrix =
            camera_matrix.unwrap_or_else(|| gamut_matrix(Gamut::Bt709, Gamut::AcesAp1));
        let track = args.get("track").map(gps_track).transpose()?;
        let max_gap = number(args, "max_track_gap_s", 30.0)?;
        if !(0.0..=3600.0).contains(&max_gap) {
            return Err("Track interpolation gap exceeds its admitted time interval".into());
        }
        let mut raw_family = None;
        let mut brackets = Vec::with_capacity(frames.len());
        let mut provenance = Vec::with_capacity(frames.len());
        let mut semantic = ReceiptGraph::new("urn:qualia:camera:source");
        for (ordinal, frame) in frames.iter().enumerate() {
            let row = record(frame)?;
            fields(
                row,
                &[
                    "source",
                    "cfa",
                    "demosaic",
                    "black",
                    "white",
                    "exposure_s",
                    "utc_s",
                ],
            )?;
            let handle = required(row, "source")?;
            let exposure = num(required(row, "exposure_s")?)?;
            if !(1e-7..=3600.0).contains(&exposure) {
                return Err("Camera exposure duration exceeds the admitted range".into());
            }
            let (rgb, digest, raw) = if let Ok(Artifact::Rgba16(image)) = self.workspace.get(handle)
            {
                if image.width as usize != width
                    || image.height as usize != height
                    || ["cfa", "demosaic", "black", "white"]
                        .iter()
                        .any(|name| row.contains_key(*name))
                    || image.pixels.iter().any(|p| p[3] != u16::MAX)
                {
                    return Err(
                        "RGBA16 camera input requires matched dimensions and opaque samples".into(),
                    );
                }
                let transfer = image
                    .transfer
                    .filter(|tf| matches!(tf, Transfer::Srgb | Transfer::Linear))
                    .ok_or("RGBA16 must declare linear or sRGB transfer")?;
                let mut digest = Sha256::new();
                digest.update(image.width.to_le_bytes());
                digest.update(image.height.to_le_bytes());
                digest.update([u8::from(transfer == Transfer::Srgb)]);
                let mut rgb = Vec::with_capacity(width * height * 3);
                for pixel in &image.pixels {
                    for value in pixel {
                        digest.update(value.to_le_bytes());
                    }
                    rgb.extend(
                        pixel[..3]
                            .iter()
                            .map(|&v| to_linear(f32::from(v) / 65535.0, transfer)),
                    );
                }
                (rgb, format!("{:x}", digest.finalize()), false)
            } else {
                let resource = encoded_resource(self, handle)?;
                if resource.kind != "raw16le" || resource.bytes.len() != width * height * 2 {
                    return Err(
                        "Sensor input requires exact raw16le mosaic bytes for the declared extent"
                            .into(),
                    );
                }
                if width < 8 || height < 8 {
                    return Err("Sensor demosaicing requires at least 8 by 8 samples".into());
                }
                if camera_matrix.is_none() {
                    return Err(
                        "Sensor mosaic input requires its calibrated camera-to-AP1 matrix".into(),
                    );
                }
                let (pattern, algo) = sensor_modes(row)?;
                let black = integer(required(row, "black")?, "black")?;
                let white = integer(required(row, "white")?, "white")?;
                if black >= white || white > 65535 {
                    return Err(
                        "Sensor black and white levels must be increasing u16 counts".into(),
                    );
                }
                let raw: Vec<_> = resource
                    .bytes
                    .chunks_exact(2)
                    .map(|v| u16::from_le_bytes([v[0], v[1]]))
                    .collect();
                let rgb = demosaic_raw(
                    &raw,
                    width,
                    height,
                    pattern,
                    black as u16,
                    white as u16,
                    algo,
                );
                (rgb, format!("{:x}", Sha256::digest(&resource.bytes)), true)
            };
            if raw_family.is_some_and(|other| raw != other) || (!raw && camera_matrix.is_some()) {
                return Err("Combine frames with the same native color input family".into());
            }
            raw_family = Some(raw);
            let time = row.get("utc_s").map(num).transpose()?;
            let location =
                time.and_then(|time| track.as_ref()?.interpolate_position(time, max_gap));
            let asset = format!("urn:sha256:{digest}");
            let capture = format!("{asset}:capture:{ordinal}");
            semantic.fact(&capture, "http://www.w3.org/ns/prov#wasDerivedFrom", &asset)?;
            camera_number(
                &mut semantic,
                &capture,
                "urn:qualia:camera:exposureSeconds",
                exposure,
            )?;
            if let Some(time) = time {
                camera_number(
                    &mut semantic,
                    &capture,
                    "urn:qualia:camera:utcSeconds",
                    time,
                )?;
            }
            if let Some(location) = location {
                camera_number(
                    &mut semantic,
                    &capture,
                    "http://www.w3.org/2003/01/geo/wgs84_pos#lat",
                    location.latitude,
                )?;
                camera_number(
                    &mut semantic,
                    &capture,
                    "http://www.w3.org/2003/01/geo/wgs84_pos#long",
                    location.longitude,
                )?;
                if let Some(altitude) = location.altitude {
                    camera_number(
                        &mut semantic,
                        &capture,
                        "http://www.w3.org/2003/01/geo/wgs84_pos#alt",
                        altitude,
                    )?;
                }
            }
            provenance.push(Value::Record(Args::from([
                ("source_sha256".into(), Value::String(digest)),
                ("exposure_s".into(), Value::F64(exposure)),
                ("utc_s".into(), time.map_or(Value::Null, Value::F64)),
                (
                    "location".into(),
                    location.map_or(Value::Null, |location| {
                        Value::Record(Args::from([
                            ("latitude_deg".into(), Value::F64(location.latitude)),
                            ("longitude_deg".into(), Value::F64(location.longitude)),
                            (
                                "altitude_m".into(),
                                location.altitude.map_or(Value::Null, Value::F64),
                            ),
                        ]))
                    }),
                ),
            ])));
            brackets.push(ExposureBracket {
                rgb,
                exposure_time: exposure as f32,
            });
        }
        let mut rgb = match mode {
            "single" => brackets.pop().ok_or("No camera frame")?.rgb,
            "hdr" => merge_hdr(
                &brackets,
                width,
                height,
                HdrOptions {
                    deghost_threshold: deghost as f32,
                    reference_index: brackets.len() / 2,
                },
            )?,
            "registered_strip" => {
                let frames = brackets
                    .into_iter()
                    .map(|bracket| bracket.rgb)
                    .collect::<Vec<_>>();
                stitch_panorama(
                    &frames,
                    width,
                    height,
                    overlap,
                    radiance_camera_raw::fusion::PanoramaProjection::Cylindrical,
                )?
            }
            _ => unreachable!("camera mode admitted above"),
        };
        let mut rgba = Vec::with_capacity(output_width * height * 4);
        for pixel in rgb.chunks_exact_mut(3) {
            let mapped = mul_vec(
                &color_matrix,
                [
                    f64::from(pixel[0]),
                    f64::from(pixel[1]),
                    f64::from(pixel[2]),
                ],
            );
            if mapped
                .iter()
                .any(|v| !v.is_finite() || v.abs() > f64::from(f32::MAX))
            {
                return Err("Camera development produced a non-finite AP1 sample".into());
            }
            rgba.extend([mapped[0] as f32, mapped[1] as f32, mapped[2] as f32, 1.0]);
        }
        let image =
            self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(RadianceLattice {
                width: output_width as u32,
                height: height as u32,
                channels: 4,
                primaries_iri: "urn:qualia:color:primaries:aces-ap1".into(),
                transfer_iri: "urn:qualia:color:transfer:linear".into(),
                samples: LatticeSamples::F32(rgba),
            })))?;
        let metadata = self.encode("q42", semantic.seal(self.limits.resource_bytes)?)?;
        Ok(Value::Record(Args::from([
            ("image".into(), image),
            ("metadata".into(), metadata),
            ("sources".into(), Value::List(provenance)),
        ])))
    }
}

fn camera_number(
    graph: &mut ReceiptGraph,
    subject: &str,
    predicate: &str,
    value: f64,
) -> Result<(), String> {
    let term = format!("\"{value:e}\"^^<http://www.w3.org/2001/XMLSchema#double>");
    graph.fact(subject, predicate, &term)
}

fn camera_matrix(value: &Value) -> Result<Mat3, String> {
    let rows = list(value)?
        .iter()
        .map(vector::<3>)
        .collect::<Result<Vec<_>, _>>()?;
    let matrix: Mat3 = rows
        .try_into()
        .map_err(|_| "Camera matrix requires three rows")?;
    if matrix.iter().flatten().any(|v| v.abs() > 100.0)
        || color_transforms::icc::math::det(&matrix).abs() < 1e-8
    {
        return Err("Camera matrix must be finite, bounded and nonsingular".into());
    }
    Ok(matrix)
}

fn sensor_modes(options: &Args) -> Result<(CfaPattern, DemosaicAlgo), String> {
    let pattern = match text(required(options, "cfa")?)? {
        "rggb" => CfaPattern::Rggb,
        "bggr" => CfaPattern::Bggr,
        "grbg" => CfaPattern::Grbg,
        "gbrg" => CfaPattern::Gbrg,
        "xtrans" => CfaPattern::XTrans,
        _ => return Err("Unknown camera CFA pattern".into()),
    };
    let algo = match options.get("demosaic").map(text).transpose()? {
        None if pattern == CfaPattern::XTrans => DemosaicAlgo::XTrans,
        None | Some("rcd") => DemosaicAlgo::Rcd,
        Some("ahd") => DemosaicAlgo::Ahd,
        Some("xtrans") => DemosaicAlgo::XTrans,
        _ => return Err("Unknown demosaicing method".into()),
    };
    if (pattern == CfaPattern::XTrans) != (algo == DemosaicAlgo::XTrans) {
        return Err("X-Trans demosaicing and CFA pattern must agree".into());
    }
    Ok((pattern, algo))
}

fn gps_track(value: &Value) -> Result<GpxTracklog, String> {
    let points = list(value)?;
    if !(1..=4096).contains(&points.len()) {
        return Err("Geotagging requires 1 to 4096 timestamped track points".into());
    }
    let mut track = GpxTracklog::new();
    for point in points {
        let row = record(point)?;
        fields(
            row,
            &["utc_s", "latitude_deg", "longitude_deg", "altitude_m"],
        )?;
        let time = num(required(row, "utc_s")?)?;
        let latitude = num(required(row, "latitude_deg")?)?;
        let longitude = num(required(row, "longitude_deg")?)?;
        let altitude = row.get("altitude_m").map(num).transpose()?;
        if track.points.last().is_some_and(|p| p.time >= time)
            || !(-90.0..=90.0).contains(&latitude)
            || !(-180.0..=180.0).contains(&longitude)
            || track
                .points
                .last()
                .is_some_and(|p| (p.longitude - longitude).abs() > 180.0)
        {
            return Err("Track points must have valid coordinates, increasing times and continuous longitude".into());
        }
        track.points.push(TrackPoint {
            time,
            latitude,
            longitude,
            altitude,
        });
    }
    Ok(track)
}
