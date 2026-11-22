//! Explicit pixel-region redaction and metadata-free raster extraction.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{mem::size_of, sync::Arc};

#[cfg(not(target_arch = "wasm32"))]
use qualia_core_db::medical::{
    dicom_codecs::NativePixelView,
    dicom_dataset::{ParseLimits, Part10Dataset, TransferSyntax, WorkBudget},
};

use crate::artifact::{LatticeSamples, RadianceLattice};
use crate::{fields, integer, list, record, required, text, Args, AssetHost, TypedArtifact, Value};

use super::accessibility::lattice;
#[cfg(not(target_arch = "wasm32"))]
use super::retained;
use super::{review_fact, review_graph, review_number, REVIEW_NS};

const MAX_PIXELS: usize = 8 * 1024 * 1024;
#[cfg(not(target_arch = "wasm32"))]
const MAX_INPUT_BYTES: usize = 16 * 1024 * 1024;
const MAX_TOTAL_PIXELS: usize = 32 * 1024 * 1024;
const MAX_MASK_WRITES: usize = 128 * 1024 * 1024;

#[derive(Clone, Copy)]
struct Region {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl AssetHost<'_> {
    pub(crate) fn redact_clinical_and_biometric_phi(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        match text(required(args, "mode")?)? {
            "authored_raster_regions" => self.redact_authored_frames(args),
            #[cfg(not(target_arch = "wasm32"))]
            "dicom_roi_raster" => self.redact_dicom_raster(args),
            #[cfg(target_arch = "wasm32")]
            "dicom_roi_raster" => {
                Err("CapabilityUnavailable: DICOM raster extraction requires a native host".into())
            }
            _ => Err("Select authored_raster_regions or dicom_roi_raster".into()),
        }
    }

    fn redact_authored_frames(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["mode", "frames", "regions"])?;
        let handles = list(required(args, "frames")?)?;
        if !(1..=256).contains(&handles.len()) {
            return Err("Raster redaction requires 1 to 256 decoded frames".into());
        }
        let mut frames = Vec::with_capacity(handles.len());
        let mut geometries = Vec::with_capacity(handles.len());
        let mut total_pixels = 0;
        let mut max_scratch = 0;
        for handle in handles {
            let source = lattice(self, handle)?;
            let n = image_area(source.width as usize, source.height as usize)?;
            validate_frame_layout(source, n)?;
            geometries.push([source.width as usize, source.height as usize]);
            max_scratch = max_scratch.max(n * (1 + source.channels as usize * size_of::<f32>()));
            total_pixels += n;
            if total_pixels > MAX_TOTAL_PIXELS {
                return Err("Raster redaction exceeds its aggregate pixel budget".into());
            }
        }
        let (regions, mask_writes) = parse_regions(required(args, "regions")?, &geometries)?;
        self.charge(total_pixels * 16 + mask_writes, max_scratch)?;
        let mut masked = 0;
        for (index, handle) in handles.iter().enumerate() {
            self.charge(0, 0)?;
            let source = lattice(self, handle)?;
            let (mut frame, mask) = {
                let [width, height] = geometries[index];
                (source.clone(), build_mask(width, height, &regions[index]))
            };
            let LatticeSamples::F32(samples) = &mut frame.samples else {
                return Err("Raster redaction needs decoded F32 samples".into());
            };
            for (pixel, &remove) in samples.chunks_exact_mut(frame.channels as usize).zip(&mask) {
                if pixel.get(3).is_some_and(|&alpha| alpha != 1.0)
                    || pixel[..3].iter().any(|value| !value.is_finite())
                {
                    return Err(
                        "Raster redaction requires finite samples and fully opaque pixels".into(),
                    );
                }
                if remove != 0 {
                    pixel[..3].fill(0.0);
                    masked += 1;
                }
            }
            frames.push(self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(frame)))?);
        }
        let receipt =
            self.admit_redaction_receipt("AuthoredRasterRegionMask", frames.len(), masked)?;
        Ok(Value::Record(Args::from([
            ("frames".into(), Value::List(frames.into())),
            ("masked_pixels".into(), Value::U64(masked as u64)),
            ("full_deidentification_assessed".into(), Value::Bool(false)),
            ("receipt".into(), receipt),
        ])))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn redact_dicom_raster(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["mode", "dicom", "regions"])?;
        let input = required(args, "dicom")?;
        let size = {
            let source = retained(self, input)?;
            if !matches!(source.kind.as_str(), "dicom" | "dcm")
                || source.bytes.len() > MAX_INPUT_BYTES
            {
                return Err("DICOM raster review expects a bounded Part 10 input".into());
            }
            source.bytes.len()
        };
        self.charge(size * 8 + 200_000, 2 * 1024 * 1024)?;
        let (width, height, offset, n, invert) = {
            let source = retained(self, input)?;
            // Keep source metadata and parser messages out of artifact handles,
            // receipts and errors. The redacted result is a new raster only.
            let dataset = Part10Dataset::parse(
                &source.bytes,
                ParseLimits {
                    max_bytes: MAX_INPUT_BYTES,
                    ..Default::default()
                },
            )
            .map_err(|_| "DICOM Part 10 dataset could not be parsed")?;
            let mut query_work = WorkBudget::new(200_000);
            let pixels = NativePixelView::from_dataset(&dataset, MAX_PIXELS, &mut query_work)
                .map_err(|_| {
                    "DICOM Part 10 does not contain a supported native monochrome plane"
                })?;
            if dataset.transfer_syntax() != TransferSyntax::ExplicitLittleEndian
                || pixels.frames() != 1
                || dataset
                    .unsigned_short(0x0028_0100, &mut query_work)
                    .ok()
                    .flatten()
                    != Some(8)
                || dataset
                    .unsigned_short(0x0028_0101, &mut query_work)
                    .ok()
                    .flatten()
                    != Some(8)
                || dataset
                    .unsigned_short(0x0028_0103, &mut query_work)
                    .ok()
                    .flatten()
                    != Some(0)
            {
                return Err("DICOM raster review supports one native unsigned 8-bit monochrome plane in Explicit VR Little Endian".into());
            }
            let (width, height) = (pixels.columns() as usize, pixels.rows() as usize);
            let n = image_area(width, height)?;
            let offset = pixels
                .stored_sample_bytes()
                .as_ptr()
                .addr()
                .checked_sub(source.bytes.as_ptr().addr())
                .ok_or("DICOM pixel plane is outside the retained source")?;
            (
                width,
                height,
                offset,
                n,
                pixels.photometric() == "MONOCHROME1",
            )
        };
        let (regions, mask_writes) = parse_regions(required(args, "regions")?, &[[width, height]])?;
        self.charge(n * 8 + mask_writes, n * (1 + size_of::<f32>()))?;
        let mask = build_mask(width, height, &regions[0]);
        let masked = mask.iter().map(|&v| usize::from(v)).sum::<usize>();
        let samples = {
            let source = retained(self, input)?;
            let pixels = source
                .bytes
                .get(offset..offset + n)
                .ok_or("DICOM pixel plane exceeds source extent")?;
            pixels
                .iter()
                .zip(&mask)
                .map(|(&pixel, &remove)| {
                    if remove != 0 {
                        0.0
                    } else {
                        f32::from(if invert { 255 - pixel } else { pixel }) / 255.0
                    }
                })
                .collect::<Vec<_>>()
        };
        let image =
            self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(RadianceLattice {
                width: width as u32,
                height: height as u32,
                channels: 1,
                primaries_iri: "urn:qualia:color:primaries:monochrome".into(),
                transfer_iri: "urn:qualia:color:transfer:uncalibrated-display-code".into(),
                samples: LatticeSamples::F32(samples),
            })))?;
        let receipt = self.admit_redaction_receipt("DicomPixelOnlyRegionMask", 1, masked)?;
        Ok(Value::Record(Args::from([
            ("image".into(), image),
            ("masked_pixels".into(), Value::U64(masked as u64)),
            ("full_deidentification_assessed".into(), Value::Bool(false)),
            ("receipt".into(), receipt),
        ])))
    }

    fn admit_redaction_receipt(
        &mut self,
        mode: &str,
        frames: usize,
        masked: usize,
    ) -> Result<Value, String> {
        let mut graph = review_graph(mode)?;
        review_fact(
            &mut graph,
            "classification",
            &format!("{REVIEW_NS}AuthoredRegionMaskOnly"),
        )?;
        review_number(&mut graph, "frames", frames)?;
        review_number(&mut graph, "maskedPixels", masked)?;
        self.admit_review(graph)
    }
}

fn image_area(width: usize, height: usize) -> Result<usize, String> {
    width
        .checked_mul(height)
        .filter(|n| (1..=MAX_PIXELS).contains(n))
        .ok_or_else(|| "Raster geometry exceeds the redaction pixel budget".to_owned())
}

fn validate_frame_layout(image: &RadianceLattice, area: usize) -> Result<(), String> {
    let LatticeSamples::F32(samples) = &image.samples else {
        return Err("Raster redaction needs decoded F32 samples".into());
    };
    let channels = image.channels as usize;
    // Only pass through recognized color metadata. Arbitrary text or custom
    // labeled profiles would reintroduce a channel for patient metadata.
    if !matches!(channels, 3 | 4)
        || samples.len() != area * channels
        || !matches!(
            image.primaries_iri.as_str(),
            "urn:qualia:color:primaries:rec709" | "http://www.w3.org/ns/ttml/parameter/rec709"
        )
        || !matches!(
            image.transfer_iri.as_str(),
            "urn:qualia:color:transfer:linear" | "urn:qualia:color:transfer:srgb"
        )
    {
        return Err(
            "Raster redaction supports F32 linear or sRGB Rec.709 RGB and opaque RGBA frames"
                .into(),
        );
    }
    Ok(())
}

fn parse_regions(
    value: &Value,
    geometries: &[[usize; 2]],
) -> Result<(Vec<Vec<Region>>, usize), String> {
    let items = list(value)?;
    if items.is_empty() || items.len() > 1024 {
        return Err("Raster redaction requires 1 to 1,024 explicit regions".into());
    }
    let mut by_frame = vec![Vec::new(); geometries.len()];
    let mut writes = 0;
    for item in items {
        let item = record(item)?;
        fields(item, &["frame", "x", "y", "width", "height"])?;
        let frame = item
            .get("frame")
            .map(|value| integer(value, "frame"))
            .transpose()?
            .unwrap_or(0);
        let [width, height] = *geometries
            .get(frame)
            .ok_or("Region frame index is out of range")?;
        let x0 = integer(required(item, "x")?, "x")?;
        let y0 = integer(required(item, "y")?, "y")?;
        let dx = integer(required(item, "width")?, "width")?;
        let dy = integer(required(item, "height")?, "height")?;
        let x1 = x0
            .checked_add(dx)
            .filter(|&right| dx != 0 && right <= width)
            .ok_or("Region exceeds raster width")?;
        let y1 = y0
            .checked_add(dy)
            .filter(|&bottom| dy != 0 && bottom <= height)
            .ok_or("Region exceeds raster height")?;
        writes += dx * dy;
        if writes > MAX_MASK_WRITES {
            return Err("Region masks exceed the redaction work budget".into());
        }
        by_frame[frame].push(Region { x0, y0, x1, y1 });
    }
    if by_frame.iter().any(Vec::is_empty) {
        return Err("Every released frame requires an explicitly authored region".into());
    }
    Ok((by_frame, writes))
}

fn build_mask(width: usize, height: usize, regions: &[Region]) -> Vec<u8> {
    let mut mask = vec![0; width * height];
    for region in regions {
        for y in region.y0..region.y1 {
            let start = y * width;
            mask[start + region.x0..start + region.x1].fill(1);
        }
    }
    mask
}
