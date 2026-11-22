//! Bounded ProRes master encoding and range decoding.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::io::Cursor;

use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::*;
use color_transforms::{Transfer, linear_to_srgb, to_linear};
use media_isobmff::{
    Brand, CodecConfig, FourCc, Mp4Writer, SampleEntry, TrackConfig, TrackKind, WriteSample,
    WriterOptions,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use video_prores_codec::{
    ChromaFormat, ColorInfo, DecodeOptions, Encoder, EncoderConfig, Frame, FrameHeader, Profile,
    decode_frame_with, probe,
};

impl AssetHost<'_> {
    pub(crate) fn encode_video_master(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["frames", "codec", "fps"])?;
        let frames = list(required(args, "frames")?)?;
        let profile = match text(required(args, "codec")?)? {
            "prores422" => Profile::Hq,
            "prores4444" => Profile::P4444,
            _ => return Err("Video master supports native ProRes 422 HQ and ProRes 4444".into()),
        };
        let rate = crate::chronicle_sequence::frame_rate(args, 24, 240)?;
        let first = frames.first().ok_or("Missing video frames")?;
        let (width, height, transfer) = input_format(self.workspace.get(first)?)?;
        if frames.len() > 120 {
            return Err("ProRes master requires at most 120 frames".into());
        }
        let pixels = frame_pixels(width, height)?;
        let total = frames.len().saturating_mul(pixels);
        self.charge(total.saturating_mul(256), total.saturating_mul(20))?;
        let fourcc = profile.fourcc();
        let mut movie = Mp4Writer::new(Cursor::new(Vec::new()), WriterOptions::new(Brand::Mov))
            .map_err(|err| err.to_string())?;
        let track = movie
            .add_track(TrackConfig::new(
                SampleEntry::prores(FourCc(fourcc), width as u16, height as u16),
                rate.num as u32,
            ))
            .map_err(|err| err.to_string())?;
        let alpha = profile == Profile::P4444;
        let chroma = if alpha {
            ChromaFormat::Yuv444
        } else {
            ChromaFormat::Yuv422
        };
        let color = ColorInfo {
            primaries: 1,
            transfer: if transfer == Transfer::Linear { 8 } else { 13 },
            matrix: 1,
        };
        let mut config = EncoderConfig::new(profile, width, height);
        config.color = color;
        config.encode_alpha = alpha;
        let mut encoder = Encoder::with_config(config);
        let mut frame = Frame::new(width, height, chroma, 16, alpha);
        frame.color = color;
        let limit = self.limits.resource_bytes;
        let mut samples_bytes = 0usize;
        for value in frames {
            let image = self.workspace.get(value)?;
            if input_format(image)? != (width, height, transfer) {
                return Err(
                    "ProRes input frames must have consistent dimensions and color encoding".into(),
                );
            }
            fill_frame(&mut frame, image)?;
            let bytes = encoder.encode(&frame).map_err(|err| err.to_string())?;
            samples_bytes = samples_bytes.saturating_add(bytes.len());
            if samples_bytes > limit {
                return Err("ProRes samples exceed the encoded resource budget".into());
            }
            movie
                .write_sample(
                    track,
                    WriteSample {
                        data: &bytes,
                        duration: rate.den as u32,
                        composition_offset: 0,
                        is_sync: true,
                    },
                )
                .map_err(|err| err.to_string())?;
        }
        let bytes = movie.finish().map_err(|err| err.to_string())?.into_inner();
        let receipt = json!({
            "schema": "qualia-video-master-encode/1", "backend": "native_software",
            "codec_fourcc": std::str::from_utf8(&fourcc).map_err(|err| err.to_string())?,
            "width": width, "height": height, "frame_count": frames.len(), "alpha": alpha,
            "fps": { "numerator": rate.num, "denominator": rate.den },
            "transfer_code": color.transfer, "sha256": format!("{:x}", Sha256::digest(&bytes))
        });
        let movie = self.encode("mov", bytes)?;
        let receipt = self.encode(
            "json",
            serde_json::to_vec(&receipt).map_err(|err| err.to_string())?,
        )?;
        Ok(Value::Record(Args::from([
            ("movie".into(), movie),
            ("receipt".into(), receipt),
        ])))
    }

    pub(crate) fn decode_video_master(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "start_frame", "frame_count", "surface"])?;
        let source = required(args, "source")?;
        let first = args
            .get("start_frame")
            .map(|v| integer(v, "start_frame"))
            .transpose()?
            .unwrap_or(0);
        let count = super::count(args, "frame_count", 1, 120)?;
        let linear = match args
            .get("surface")
            .map(text)
            .transpose()?
            .unwrap_or("rgba16_linear")
        {
            "rgba16_linear" => true,
            "rgba8" => false,
            _ => return Err("Video master decoder supports rgba16_linear or rgba8".into()),
        };
        let resource = encoded_resource(self, source)?;
        if !matches!(resource.kind.as_str(), "mov" | "mp4")
            || resource.bytes.len() > self.limits.resource_bytes
        {
            return Err("Expected bounded MOV or MP4 containing ProRes video".into());
        }
        let input_size = resource.bytes.len();
        self.charge(input_size.saturating_mul(8), input_size)?;
        let resource = encoded_resource(self, source)?;
        let bytes = resource.bytes.as_slice();
        let file = media_isobmff::open(bytes).map_err(|err| err.to_string())?;
        let (track_index, track) = file
            .tracks
            .iter()
            .enumerate()
            .find(|(_, track)| {
                track.kind == TrackKind::Video
                    && matches!(track.codec(), Some(CodecConfig::ProRes { .. }))
            })
            .ok_or("No native ProRes track in the selected container")?;
        let Some(CodecConfig::ProRes { fourcc }) = track.codec() else {
            unreachable!()
        };
        let codec = std::str::from_utf8(&fourcc.0)
            .map_err(|err| err.to_string())?
            .to_string();
        let end = first.checked_add(count).ok_or("Frame range overflow")?;
        let samples = track
            .samples
            .get(first..end)
            .ok_or("Frame range exceeds ProRes track")?;
        if track.timescale == 0
            || track.entries.len() != 1
            || samples.iter().any(|s| !s.is_sync || s.duration == 0)
        {
            return Err(
                "ProRes range has an invalid timescale, sample description, or sync flag".into(),
            );
        }
        let description = track.video().ok_or("Missing ProRes sample geometry")?;
        let (width, height) = (u32::from(description.width), u32::from(description.height));
        let pixels = frame_pixels(width, height)?;
        let timescale = track.timescale;
        let timing: Vec<_> = samples
            .iter()
            .map(|sample| {
                json!({
                    "dts": sample.dts, "pts": sample.pts, "duration": sample.duration
                })
            })
            .collect();
        let total = count.saturating_mul(pixels);
        self.charge(total.saturating_mul(144), total.saturating_mul(20))?;
        let bytes = encoded_resource(self, source)?.bytes.as_slice();

        let mut pixels = Vec::with_capacity(count);
        let mut frame_format = None::<FrameHeader>;
        for index in first..end {
            let sample = file
                .read_sample(bytes, track_index, index)
                .map_err(|err| err.to_string())?;
            let header = probe(&sample).map_err(|err| err.to_string())?;
            if (u32::from(header.width), u32::from(header.height)) != (width, height)
                || header.color.matrix != 1
                || header.color.primaries != 1
                || header.interlace != video_prores_codec::Interlace::Progressive
            {
                return Err("ProRes RGB output requires consistent progressive BT.709 geometry and primaries".into());
            }
            if frame_format.as_ref().is_some_and(|mark| {
                mark.chroma != header.chroma
                    || mark.color != header.color
                    || mark.alpha != header.alpha
            }) {
                return Err(
                    "ProRes sample color or alpha format changes within the selected range".into(),
                );
            }
            let transfer = Transfer::from_code(header.color.transfer)
                .filter(|tf| matches!(tf, Transfer::Bt709 | Transfer::Srgb | Transfer::Linear))
                .ok_or("ProRes RGB output requires a supported SDR transfer")?;
            let frame = decode_frame_with(
                &sample,
                &DecodeOptions {
                    bit_depth: Some(16),
                },
            )
            .map_err(|err| err.to_string())?;
            pixels.push(decode_pixels(&frame, transfer, linear));
            frame_format = Some(header);
        }
        let header = frame_format.ok_or("Missing decoded ProRes frames")?;
        let telemetry = json!({
            "schema": "qualia-video-master-decode/1", "backend": "native_software",
            "codec_fourcc": codec, "width": width, "height": height,
            "coded_bit_depth": if header.chroma == ChromaFormat::Yuv422 { 10 } else { 12 },
            "source_transfer_code": header.color.transfer, "track_timescale": timescale,
            "first_frame": first, "raw_media_timing": timing
        });
        let handles = pixels
            .into_iter()
            .map(|pixels| {
                if linear {
                    self.store_rgba16(width, height, pixels, Some(Transfer::Linear))
                } else {
                    self.store_rgba(
                        width,
                        height,
                        pixels
                            .into_iter()
                            .map(|px| px.map(|v| ((u32::from(v) + 128) / 257) as u8))
                            .collect(),
                    )
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        let telemetry = self.encode(
            "json",
            serde_json::to_vec(&telemetry).map_err(|err| err.to_string())?,
        )?;
        Ok(Value::Record(Args::from([
            ("frames".into(), Value::List(handles)),
            ("telemetry".into(), telemetry),
        ])))
    }
}

fn frame_pixels(width: u32, height: u32) -> Result<usize, String> {
    if !(1..=3840).contains(&width) || !(1..=2160).contains(&height) {
        return Err("ProRes frames require dimensions within 3840 by 2160".into());
    }
    Ok(width as usize * height as usize)
}

fn input_format(artifact: &Artifact) -> Result<(u32, u32, Transfer), String> {
    match artifact {
        Artifact::Rgba(image) => Ok((image.width, image.height, Transfer::Srgb)),
        Artifact::Rgba16(image) => {
            let transfer = image
                .transfer
                .filter(|tf| matches!(tf, Transfer::Linear | Transfer::Srgb))
                .ok_or("ProRes requires a declared linear or sRGB transfer on RGBA16")?;
            Ok((image.width, image.height, transfer))
        }
        _ => Err("ProRes frames require RGBA8 or RGBA16 image handles".into()),
    }
}

fn fill_frame(frame: &mut Frame, image: &Artifact) -> Result<(), String> {
    let width = frame.width as usize;
    let stride = frame.chroma_width() as usize;
    let subsample = frame.chroma == ChromaFormat::Yuv422;
    let get = |index: usize| match image {
        Artifact::Rgba(img) => img.pixels[index].map(|sample| u16::from(sample) * 257),
        Artifact::Rgba16(img) => img.pixels[index],
        _ => unreachable!("frame source admitted by input_format"),
    };
    let ycbcr = |px: [u16; 4]| {
        let [r, g, b, _] = px.map(|v| f64::from(v) / 65535.0);
        let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        [y, (b - y) / 1.8556, (r - y) / 1.5748]
    };
    for y in 0..frame.height as usize {
        for x in 0..width {
            let index = y * width + x;
            let pixel = get(index);
            let [luma, mut cb, mut cr] = ycbcr(pixel);
            frame.y[index] = ((16.0 + 219.0 * luma) * 256.0).round() as u16;
            if let Some(alpha) = &mut frame.alpha {
                alpha[index] = pixel[3];
            } else if pixel[3] != u16::MAX {
                return Err("Use ProRes 4444 for images with a nonopaque alpha channel".into());
            }
            if subsample && x % 2 != 0 {
                continue;
            }
            if subsample && x + 1 < width {
                let [_, next_cb, next_cr] = ycbcr(get(index + 1));
                cb = (cb + next_cb) * 0.5;
                cr = (cr + next_cr) * 0.5;
            }
            let offset = y * stride + if subsample { x / 2 } else { x };
            frame.cb[offset] = ((128.0 + 224.0 * cb) * 256.0).round() as u16;
            frame.cr[offset] = ((128.0 + 224.0 * cr) * 256.0).round() as u16;
        }
    }
    Ok(())
}

fn decode_pixels(frame: &Frame, transfer: Transfer, linear: bool) -> Vec<[u16; 4]> {
    let width = frame.width as usize;
    let chroma_stride = frame.chroma_width() as usize;
    frame
        .y
        .iter()
        .enumerate()
        .map(|(index, &sample)| {
            let x = index % width;
            let x = if frame.chroma == ChromaFormat::Yuv422 {
                x / 2
            } else {
                x
            };
            let c = index / width * chroma_stride + x;
            let y = (f32::from(sample) / 256.0 - 16.0) / 219.0;
            let cb = (f32::from(frame.cb[c]) / 256.0 - 128.0) / 224.0;
            let cr = (f32::from(frame.cr[c]) / 256.0 - 128.0) / 224.0;
            let red = y + 1.5748 * cr;
            let blue = y + 1.8556 * cb;
            let green = (y - 0.2126 * red - 0.0722 * blue) / 0.7152;
            let [r, g, b] = [red, green, blue].map(|v| {
                let decoded = to_linear(v.clamp(0.0, 1.0), transfer);
                let value = if linear {
                    decoded
                } else {
                    linear_to_srgb(decoded)
                };
                (value.clamp(0.0, 1.0) * 65535.0).round() as u16
            });
            [
                r,
                g,
                b,
                frame.alpha.as_ref().map_or(u16::MAX, |alpha| alpha[index]),
            ]
        })
        .collect()
}
