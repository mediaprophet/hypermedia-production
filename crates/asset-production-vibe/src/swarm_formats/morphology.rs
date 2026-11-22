//! Native morphology and texture statistics for authored binary patterns.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, sync::Arc};

use qualia_core_db::specialized_libs::computer_vision::{
    bio::{
        morphology::voronoi_otsu_label,
        radiomics::{glcm_features_d1, shape_2d_from_mask},
    },
    cv::GrayView,
};
use serde_json::json;

use super::{encode_json, result};
use crate::{
    Args, AssetHost, TypedArtifact, Value,
    artifact::{FieldLattice, LatticeSamples},
    boolean, fields, integer, required,
};

const MAX_AXIS: u32 = 512;

impl AssetHost<'_> {
    pub(crate) fn analyze_histopathology_and_radiomics(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["mask", "synthetic_phantom", "quantization_levels"])?;
        if !boolean(args, "synthetic_phantom", false)? {
            return Err("Morphology requires an authored synthetic binary pattern".into());
        }
        let source = required(args, "mask")?;
        let mask = BinaryMask::read(self.workspace.typed(source)?)?;
        let size = mask.width as usize * mask.height as usize;
        let levels = args
            .get("quantization_levels")
            .map(|value| integer(value, "quantization_levels"))
            .transpose()?
            .unwrap_or(16);
        if !matches!(levels, 2 | 16 | 32) {
            return Err("Quantization levels must be 2, 16 or 32".into());
        }
        let (width, height) = (mask.width, mask.height);
        self.charge(size.saturating_mul(220), size.saturating_mul(30))?;
        let mask = BinaryMask::read(self.workspace.typed(source)?)?;
        let bytes = mask.to_bytes()?;
        let foreground = bytes.iter().filter(|&&byte| byte == 255).count();
        if foreground >= u16::MAX as usize {
            return Err("Binary mask exceeds the native watershed label capacity".into());
        }
        let view =
            GrayView::new(width, height, width, &bytes).ok_or("Invalid binary mask extent")?;
        let mut labels = vec![0; size];
        voronoi_otsu_label(view, &mut labels)
            .map_err(|error| format!("Native morphology: {error:?}"))?;
        let features = glcm_features_d1(&bytes, width as usize, height as usize, levels)
            .map_err(|error| format!("Native GLCM: {error:?}"))?;
        let shape = if foreground == 0 {
            None
        } else {
            let shape = shape_2d_from_mask(&bytes, width as usize, height as usize)
                .map_err(|error| format!("Native mask shape: {error:?}"))?;
            Some(json!({
                "perimeter_cell_edges":shape.perimeter, "circularity":shape.circularity,
                "diameter_cells":shape.max_diameter,
                "diameter_method":if foreground <= 256 { "point_pairs" } else { "bounds_diagonal" },
            }))
        };
        let mut areas = BTreeMap::new();
        for label in labels.iter().copied().filter(|&id| id != 0) {
            *areas.entry(label).or_insert(0usize) += 1;
        }
        let labeled = areas.values().sum::<usize>();
        let regions: Vec<_> = areas
            .into_iter()
            .map(|(label, area)| json!({ "label":label, "area_cells":area }))
            .collect();
        let report = encode_json(
            self,
            &json!({
                "schema":"qualia-authored-binary-morphology/1",
                "width":width, "height":height, "method":"voronoi_otsu",
                "foreground_cells":foreground, "labeled_cells":labeled, "region_count":regions.len(), "regions":regions,
                "shape":shape,
                "glcm": { "contrast":features.contrast, "homogeneity":features.homogeneity,
                    "energy":features.energy, "correlation":features.correlation, "levels":features.levels,
                    "distance_cells":1, "directions_degrees":[0, 90] },
            }),
        )?;
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![width, height],
                samples: labels.into_iter().map(f32::from).collect(),
            },
        )))?;
        Ok(result(&[("labels", field), ("report", report)]))
    }
}

#[derive(Clone, Copy)]
enum Samples<'a> {
    Float(&'a [f32]),
    Byte(&'a [u8]),
}

struct BinaryMask<'a> {
    width: u32,
    height: u32,
    samples: Samples<'a>,
}

impl<'a> BinaryMask<'a> {
    fn read(artifact: &'a TypedArtifact) -> Result<Self, String> {
        let (width, height, samples, len) = match artifact {
            TypedArtifact::FieldLattice(field) => {
                let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
                    return Err("Binary mask requires a dense F32 lattice".into());
                };
                let [width, height] = axes.as_slice() else {
                    return Err("Binary mask requires two axes".into());
                };
                (*width, *height, Samples::Float(samples), samples.len())
            }
            TypedArtifact::RadianceLattice(image) if image.channels == 1 => {
                let LatticeSamples::U8(samples) = &image.samples else {
                    return Err("Binary mask radiance input requires U8 samples".into());
                };
                (
                    image.width,
                    image.height,
                    Samples::Byte(samples),
                    samples.len(),
                )
            }
            _ => return Err("Expected a binary 2D field or single-channel U8 lattice".into()),
        };
        if !(2..=MAX_AXIS).contains(&width)
            || !(2..=MAX_AXIS).contains(&height)
            || len != width as usize * height as usize
        {
            return Err(
                "Binary mask dimensions or sample extent are outside the admitted range".into(),
            );
        }
        Ok(Self {
            width,
            height,
            samples,
        })
    }

    fn to_bytes(&self) -> Result<Vec<u8>, String> {
        match self.samples {
            Samples::Byte(samples) if samples.iter().all(|&sample| matches!(sample, 0 | 255)) => {
                Ok(samples.to_vec())
            }
            Samples::Float(samples)
                if samples.iter().all(|&sample| sample == 0.0 || sample == 1.0) =>
            {
                Ok(samples
                    .iter()
                    .map(|&sample| 255 * u8::from(sample == 1.0))
                    .collect())
            }
            _ => Err("Synthetic mask samples must be binary (0/1 for F32 or 0/255 for U8)".into()),
        }
    }
}
