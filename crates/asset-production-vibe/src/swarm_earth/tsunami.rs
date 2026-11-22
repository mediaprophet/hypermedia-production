//! Hydrostatic transfer of authored seafloor displacements into the native SWE solver.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use crate::artifact::FieldLattice;
use crate::{
    Args, AssetHost, TypedArtifact, Value, fields, integer, list, num, record, required, text,
};

use super::{bounded, receipt};

impl AssetHost<'_> {
    pub(crate) fn simulate_seismology_and_tsunami_coupling(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "width",
                "height",
                "cell_size_m",
                "bathymetry_m",
                "seafloor_displacement_m",
                "rest_surface_m",
                "manning_n",
                "dt_s",
                "steps",
                "max_substeps_per_step",
            ],
        )?;
        if text(required(args, "mode")?)? != "authored_seafloor_displacement" {
            return Err("Expected authored_seafloor_displacement mode".into());
        }
        let width = integer(required(args, "width")?, "width")?;
        let height = integer(required(args, "height")?, "height")?;
        if !(2..=256).contains(&width) || !(2..=256).contains(&height) {
            return Err("Tsunami raster axes must be within 2..=256".into());
        }
        let cells = width * height;
        let spacing = bounded(args, "cell_size_m", 1.0, 100_000.0)?;
        let surface = bounded(args, "rest_surface_m", -1e4, 1e4)?;
        self.charge(cells * 42, cells * 96)?;
        let before = raster(
            self,
            required(args, "bathymetry_m")?,
            width,
            height,
            [-20_000.0, 10_000.0],
        )?;
        let displacement = raster(
            self,
            required(args, "seafloor_displacement_m")?,
            width,
            height,
            [-100.0, 100.0],
        )?;
        let mut hasher = Sha256::new();
        hasher.update((width as u32).to_le_bytes());
        hasher.update((height as u32).to_le_bytes());
        hasher.update(spacing.to_le_bytes());
        hasher.update(surface.to_le_bytes());
        let mut post_event_bed = Vec::with_capacity(cells);
        let mut initial_depth = Vec::with_capacity(cells);
        let mut wet_count = 0;
        let mut displaced_wet_volume_m3: f64 = 0.0;
        for (bed, lift) in before.into_iter().zip(displacement) {
            hasher.update(bed.to_le_bytes());
            hasher.update(lift.to_le_bytes());
            let depth = (surface - bed).max(0.0);
            // An instantaneous vertical motion lifts the wet water column without
            // changing its depth. The already dry bed receives no surrogate water.
            if depth > 0.0 {
                wet_count += 1;
                displaced_wet_volume_m3 += lift * spacing * spacing;
            }
            post_event_bed.push(Value::F64(bed + lift));
            initial_depth.push(Value::F64(depth));
        }
        if wet_count == 0 {
            return Err("Tsunami grid has no initially submerged columns".into());
        }
        let mut swe = BTreeMap::from([
            ("width".into(), Value::U64(width as u64)),
            ("height".into(), Value::U64(height as u64)),
            ("cell_size_m".into(), Value::F64(spacing)),
            ("bathymetry_m".into(), Value::List(post_event_bed.into())),
            ("initial_depth_m".into(), Value::List(initial_depth.into())),
            ("rain_rate_m_s".into(), Value::F64(0.0)),
            ("manning_n".into(), required(args, "manning_n")?.clone()),
            ("dt_s".into(), required(args, "dt_s")?.clone()),
            ("steps".into(), required(args, "steps")?.clone()),
        ]);
        if let Some(limit) = args.get("max_substeps_per_step") {
            swe.insert("max_substeps_per_step".into(), limit.clone());
        }
        let mut output = record(&self.solve_shallow_water_25d(&swe)?)?.clone();
        let source_id = format!("urn:sha256:{:x}", hasher.finalize());
        let water = required(&output, "water")?;
        let water_id = match self.workspace.typed(water)? {
            TypedArtifact::FieldLattice(field) => match field.as_ref() {
                FieldLattice::DenseF32 { axes, samples } => {
                    let mut digest = Sha256::new();
                    for axis in axes {
                        digest.update(axis.to_le_bytes());
                    }
                    for value in samples {
                        digest.update(value.to_le_bytes());
                    }
                    format!("urn:sha256:{:x}", digest.finalize())
                }
                _ => {
                    return Err("Shallow-water solver returned an unexpected lattice layout".into());
                }
            },
            _ => return Err("Shallow-water solver returned an unexpected artifact".into()),
        };
        output.insert(
            "displaced_wet_volume_m3".into(),
            Value::F64(displaced_wet_volume_m3),
        );
        output.insert(
            "source_model".into(),
            Value::String("instantaneous_vertical_seafloor_shift".into()),
        );
        output.insert(
            "receipt".into(),
            receipt(
                self,
                "authored-seafloor-displacement",
                &[
                    (
                        &water_id,
                        "http://www.w3.org/ns/prov#wasDerivedFrom",
                        &source_id,
                    ),
                    (
                        &water_id,
                        "http://www.w3.org/ns/prov#wasGeneratedBy",
                        "urn:qualia:earth:shallow-water-displacement",
                    ),
                ],
            )?,
        );
        Ok(Value::Record(output))
    }
}

fn raster(
    host: &AssetHost<'_>,
    value: &Value,
    width: usize,
    height: usize,
    bounds: [f64; 2],
) -> Result<Vec<f64>, String> {
    let shape = [height as u32, width as u32];
    let cells = width * height;
    let values: Vec<f64> = if let Ok(TypedArtifact::FieldLattice(field)) =
        host.workspace.typed(value)
    {
        match field.as_ref() {
            FieldLattice::DenseF32 { axes, samples }
                if axes.as_slice() == shape && samples.len() == cells =>
            {
                samples.iter().copied().map(f64::from).collect()
            }
            _ => return Err("Expected retained DenseF32 raster of shape [height, width]".into()),
        }
    } else if matches!(value, Value::List(_)) {
        let values = list(value)?;
        if values.len() != cells {
            return Err("Raster list must match width × height".into());
        }
        values.iter().map(num).collect::<Result<_, _>>()?
    } else {
        vec![num(value)?; cells]
    };
    if values.iter().any(|v| !(bounds[0]..=bounds[1]).contains(v)) {
        return Err("Raster contains nonfinite or out-of-range elevations".into());
    }
    Ok(values)
}
