//! Native parallel-beam FBP for authored numerical phantoms.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{f64::consts::PI, sync::Arc};

use qualia_core_db::medical::{
    dicom_frames::ValueUnits,
    reconstruction::fbp::{
        FbpConfig, FbpFilter, ProjectionGeometry, SinogramView, reconstruct_ct_fbp_into,
    },
};
use serde_json::json;

use super::{encode_json, receipt_iris, result};
use crate::{
    Args, AssetHost, TypedArtifact, Value, artifact::FieldLattice, boolean, count, fields, list,
    num, number, required, text, vector,
};

impl AssetHost<'_> {
    pub(crate) fn ingest_and_reconstruct_dicom(&mut self, args: &Args) -> Result<Value, String> {
        if matches!(
            text(required(args, "modality")?)?,
            "synthetic_binary_seg" | "synthetic_fractional_seg"
        ) {
            return self.import_synthetic_dicom_seg(args);
        }
        fields(
            args,
            &[
                "modality",
                "synthetic_phantom",
                "angles_rad",
                "detector_count",
                "detector_spacing_mm",
                "projections",
                "width",
                "height",
                "pixel_spacing_mm",
                "filter",
                "cutoff_ratio",
            ],
        )?;
        if text(required(args, "modality")?)? != "synthetic_parallel_fbp"
            || !boolean(args, "synthetic_phantom", false)?
        {
            return Err(
                "This reconstruction profile requires an authored synthetic phantom sinogram"
                    .into(),
            );
        }
        let angles = list(required(args, "angles_rad")?)?;
        let detector_count = count(args, "detector_count", 0, 1024)?;
        let projections = list(required(args, "projections")?)?;
        if !(8..=720).contains(&angles.len())
            || detector_count < 2
            || projections.len() != angles.len() * detector_count
        {
            return Err("Sinogram dimensions do not match the bounded phantom profile".into());
        }
        let width = count(args, "width", 128, 512)?;
        let height = count(args, "height", 128, 512)?;
        let pixel_spacing_mm = vector(required(args, "pixel_spacing_mm")?)?;
        let detector_spacing_mm = number(args, "detector_spacing_mm", 0.0)?;
        if !pixel_spacing_mm
            .iter()
            .chain([&detector_spacing_mm])
            .all(|n| (0.01..=100.0).contains(n))
        {
            return Err("Phantom grid spacing must be within 0.01 to 100 mm".into());
        }
        let cutoff_ratio = number(args, "cutoff_ratio", 1.0)?;
        let filter = match args
            .get("filter")
            .map(text)
            .transpose()?
            .unwrap_or("ram_lak")
        {
            "ram_lak" => FbpFilter::RamLak,
            "shepp_logan" => FbpFilter::SheppLogan,
            "hann" => FbpFilter::Hann,
            _ => return Err("Unsupported phantom reconstruction filter".into()),
        };
        let output_size = width * height;
        let transform_work = projections.len() * (detector_count.ilog2() as usize + 1) * 16;
        let backprojection_work = angles.len() * output_size * 10;
        let scratch =
            (projections.len() + angles.len() + output_size * 3 + detector_count * 24) * 8;
        self.charge(transform_work + backprojection_work, scratch)?;
        let angles = angles.iter().map(num).collect::<Result<Vec<_>, _>>()?;
        validate_angles(&angles)?;
        let projections = projections.iter().map(num).collect::<Result<Vec<_>, _>>()?;
        if !projections.iter().all(|n| (0.0..=1e6).contains(n)) {
            return Err("Expected finite, nonnegative authored projection integrals".into());
        }
        let mut digest = blake3::Hasher::new();
        digest.update(b"qualia.synthetic.parallel-beam.sinogram.v1");
        digest.update(&(detector_count as u64).to_le_bytes());
        digest.update(&(angles.len() as u64).to_le_bytes());
        digest.update(&detector_spacing_mm.to_le_bytes());
        for value in angles.iter().chain(&projections) {
            digest.update(&value.to_le_bytes());
        }
        let source_hash = digest.finalize();
        let sino = SinogramView {
            source_blake3: *source_hash.as_bytes(),
            n_angles: angles.len(),
            n_detectors: detector_count,
            detector_spacing_mm,
            center_detector_offset: 0.0,
            angles_rad: &angles,
            projections: &projections,
            units: ValueUnits::RescaleType("phantom-mm^-1"),
        };
        let config = FbpConfig {
            image_width: width,
            image_height: height,
            pixel_spacing_mm,
            filter,
            cutoff_ratio,
            geometry: ProjectionGeometry::Parallel,
            apply_parker_short_scan: false,
        };
        let mut slice = vec![0.0; output_size];
        let provenance =
            reconstruct_ct_fbp_into(&sino, &config, &mut slice).map_err(|e| e.to_string())?;
        let samples: Vec<f32> = slice.into_iter().map(|n| n as f32).collect();
        if samples.iter().any(|n| !n.is_finite()) {
            return Err("Phantom reconstruction is not finite in the F32 output range".into());
        }
        let mut output_digest = blake3::Hasher::new();
        output_digest.update(b"qualia.synthetic.parallel-beam.reconstruction.v1");
        output_digest.update(&(width as u64).to_le_bytes());
        output_digest.update(&(height as u64).to_le_bytes());
        for value in pixel_spacing_mm {
            output_digest.update(&value.to_le_bytes());
        }
        for sample in &samples {
            output_digest.update(&sample.to_le_bytes());
        }
        let output_hash = output_digest.finalize();
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![width as u32, height as u32],
                samples,
            },
        )))?;
        let report = encode_json(
            self,
            &json!({
                "schema":"qualia-phantom-fbp/1", "profile":"synthetic_parallel_fbp",
                "width":width, "height":height, "row_column_spacing_mm":pixel_spacing_mm,
                "source_blake3":source_hash.to_string(), "field_blake3":output_hash.to_string(),
                "derivation_blake3":blake3::Hash::from_bytes(provenance.derivation_blake3).to_string(),
                "units":"phantom-mm^-1",
            }),
        )?;
        let receipt = receipt_iris(
            self,
            "urn:qualia:format:synthetic-parallel-fbp",
            &format!("urn:blake3:{source_hash}"),
            &format!("urn:blake3:{output_hash}"),
        )?;
        Ok(result(&[
            ("field", field),
            ("report", report),
            ("receipt", receipt),
        ]))
    }
}

fn validate_angles(angles: &[f64]) -> Result<(), String> {
    let first = angles[0];
    let last = angles[angles.len() - 1];
    let step = (last - first) / (angles.len() - 1) as f64;
    if !(0.0..TAU).contains(&first)
        || !(0.0..=TAU + PI).contains(&last)
        || step <= 0.0
        || (last - first - PI).abs() > step * 1.01
        || angles
            .windows(2)
            .any(|pair| ((pair[1] - pair[0]) - step).abs() > step * 1e-6)
    {
        return Err("Angles must uniformly cover one approximate half-turn".into());
    }
    Ok(())
}

const TAU: f64 = 2.0 * PI;
