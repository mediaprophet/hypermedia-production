//! Caller-extracted OpenVDB level-set leaves to v3 SDFB on a retained 10D carrier.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    artifact::FieldLattice, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    qualia_containers_governance_poet_ops::encoded_resource, *,
};
use qualia_core_db::{container_10d::sdf_brick_section as sdf, q_hash};
use sha2::{Digest, Sha256};

impl AssetHost<'_> {
    pub(crate) fn transcode_openusd_openvdb_alembic(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "mode",
                "carrier_10d",
                "origin_m",
                "voxel_spacing_m",
                "quantization_m",
                "wall_iri",
                "lumen_iri",
                "leaves",
            ],
        )?;
        if text(required(a, "mode")?)? != "openvdb_levelset_leaf_arrays" {
            return Err("Expected openvdb_levelset_leaf_arrays with decoded OpenVDB Float32 leaves and raw masks".into());
        }
        let leaves = list(required(a, "leaves")?)?;
        if !(1..=8_192).contains(&leaves.len()) {
            return Err("Supply 1 to 8,192 extracted 8³ level-set leaves".into());
        }
        let carrier_handle = required(a, "carrier_10d")?;
        let carrier = encoded_resource(self, carrier_handle)?;
        if carrier.kind != "10d" || carrier.bytes.len() > 32 * 1024 * 1024 {
            return Err("Expected a retained 10D carrier of at most 32 MiB".into());
        }
        let field_len = sdf::sdfb_openvdb_encoded_len(leaves.len()).map_err(|e| e.to_string())?;
        let base_size = carrier.bytes.len();
        self.charge(
            leaves.len() * 4_096 + base_size,
            leaves.len() * 4_096 + base_size * 3 + field_len * 2,
        )?;
        let carrier = encoded_resource(self, carrier_handle)?;
        let origin = vector::<3>(required(a, "origin_m")?)?;
        let spacing = vector::<3>(required(a, "voxel_spacing_m")?)?;
        let step = num(required(a, "quantization_m")?)? as f32;
        if origin.iter().any(|v| v.abs() > 1e6)
            || spacing.iter().any(|v| !(1e-6..=1e4).contains(v))
            || !(1e-12..=1.0).contains(&step)
        {
            return Err(
                "Invalid level-set grid origin, voxel spacing, or positive I16 quantum".into(),
            );
        }
        let wall_iri = text(required(a, "wall_iri")?)?;
        let lumen_iri = text(required(a, "lumen_iri")?)?;
        let wall = q_hash(wall_iri);
        let lumen = q_hash(lumen_iri);
        if wall == 0 || lumen == 0 || wall == lumen {
            return Err("Wall and lumen require distinct nonzero semantic identities".into());
        }
        let mut records = Vec::with_capacity(leaves.len());
        let mut minimum = [i32::MAX; 3];
        let mut maximum = [i32::MIN; 3];
        let mut max_error = 0.0_f64;
        let mut active = 0u64;
        for item in leaves {
            let leaf = record(item)?;
            fields(leaf, &["index_origin", "distances_m", "active_mask_le_hex"])?;
            let at = grid_coordinate(required(leaf, "index_origin")?)?;
            for axis in 0..3 {
                minimum[axis] = minimum[axis].min(at[axis]);
                maximum[axis] = maximum[axis].max(at[axis]);
            }
            let TypedArtifact::FieldLattice(lattice) =
                self.workspace.typed(required(leaf, "distances_m")?)?
            else {
                return Err("Leaf distances require a retained DenseF32 field lattice".into());
            };
            let FieldLattice::DenseF32 { axes, samples } = lattice.as_ref() else {
                return Err("Leaf distances require a retained DenseF32 field lattice".into());
            };
            if axes.as_slice() != [8, 8, 8] || samples.len() != sdf::SDFB_SAMPLES {
                return Err(
                    "Each leaf requires 512 Z-fast Float32 samples in [8,8,8] shape".into(),
                );
            }
            let mask = decode_mask(text(required(leaf, "active_mask_le_hex")?)?)?;
            active += mask.iter().map(|v| u64::from(v.count_ones())).sum::<u64>();
            let mut record = sdf::SdfBrickRecord {
                key: sdf::SdfBrickKey {
                    refinement_level: 0,
                    coordinate: [0; 3],
                },
                wall_semantic_id: wall,
                lumen_semantic_id: lumen,
                samples: [0; sdf::SDFB_SAMPLES],
                occupancy: [0; sdf::SDFB_OCCUPANCY_BYTES],
            };
            for (j, &v) in samples.iter().enumerate() {
                let bits = (f64::from(v) / f64::from(step)).round();
                if !(-32768.0..=32767.0).contains(&bits) {
                    return Err(
                        "Level-set sample is nonfinite or outside the I16 quantum range".into(),
                    );
                }
                let bits = bits as i16;
                let [x, y, z] = [j / 64, (j / 8) % 8, j % 8];
                let out = x + 8 * y + 64 * z;
                record.samples[out] = bits as u16;
                if v < 0.0 {
                    record.occupancy[out / 8] |= 1 << (out % 8);
                }
                max_error = max_error.max((f64::from(v) - f64::from(f32::from(bits) * step)).abs());
            }
            records.push((at, record, mask));
        }
        records.sort_unstable_by_key(|(at, _, _)| *at);
        if records.windows(2).any(|pair| pair[0].0 == pair[1].0) {
            return Err("Duplicate OpenVDB leaf origin".into());
        }
        let mut bricks = Vec::with_capacity(leaves.len());
        let mut masks = Vec::with_capacity(leaves.len());
        for (at, mut record, mask) in records {
            record.key.coordinate =
                std::array::from_fn(|axis| ((at[axis] - minimum[axis]) / 8) as u32);
            bricks.push(record);
            masks.push(mask);
        }
        let physical_origin =
            std::array::from_fn(|i| (origin[i] + f64::from(minimum[i]) * spacing[i]) as f32);
        if (0..3).any(|i| {
            let span = (f64::from(maximum[i]) + 7.0) * spacing[i];
            let magnitude = f64::from(physical_origin[i])
                .abs()
                .max((origin[i] + span).abs());
            magnitude * f64::from(f32::EPSILON) > spacing[i] / 8.0
        }) {
            return Err(
                "Recenter the level-set origin to preserve voxel spacing in the SDFB grid".into(),
            );
        }
        let grid = sdf::SdfBrickGrid {
            origin_m: physical_origin,
            voxel_spacing_m: spacing.map(|v| v as f32),
            base_bricks: std::array::from_fn(|i| ((maximum[i] - minimum[i]) / 8 + 1) as u32),
            encoding: sdf::SdfSampleEncoding::ScaledI16,
            meters_per_i16: step,
        };
        let mut bytes = vec![0; field_len];
        sdf::encode_sdfb_with_openvdb_active_masks_into(grid, &bricks, &masks, &mut bytes)
            .map_err(|e| e.to_string())?;
        let asset = sdf::seal_10d_with_sdfb(&carrier.bytes, &bytes, wall_iri, lumen_iri)?;
        let digest = format!("{:x}", Sha256::digest(&asset));
        let subject = format!("urn:qualia:sdfb-leaf-import:sha256:{digest}");
        let mut graph = ReceiptGraph::new(&subject);
        graph.fact(
            &subject,
            "urn:qualia:sdfb:layout",
            "urn:qualia:sdfb:v3:openvdb-mask-z-fast",
        )?;
        graph.fact(&subject, "urn:qualia:sdfb:wall", wall_iri)?;
        graph.fact(&subject, "urn:qualia:sdfb:lumen", lumen_iri)?;
        let receipt = graph.seal(self.limits.resource_bytes)?;
        Ok(Value::Record(Args::from([
            ("asset".into(), self.encode("10d", asset)?),
            ("field".into(), self.encode("sdfb", bytes)?),
            ("receipt".into(), self.encode("q42", receipt)?),
            ("leaf_count".into(), Value::U64(leaves.len() as u64)),
            ("active_count".into(), Value::U64(active)),
            ("max_distance_error_m".into(), Value::F64(max_error)),
        ])))
    }
}

fn grid_coordinate(value: &Value) -> Result<[i32; 3], String> {
    let components = list(value)?;
    if components.len() != 3 {
        return Err("Leaf origin requires three integer coordinates".into());
    }
    let mut origin = [0; 3];
    for (axis, component) in components.iter().enumerate() {
        let value = match component {
            Value::I64(value) => *value,
            Value::U64(value) => {
                i64::try_from(*value).map_err(|_| "Leaf coordinate exceeds signed range")?
            }
            _ => return Err("Leaf coordinate requires a signed integer".into()),
        };
        if !(-1_000_000..=1_000_000).contains(&value) || value % 8 != 0 {
            return Err("Leaf origins must be multiples of 8 in the bounded index frame".into());
        }
        origin[axis] = value as i32;
    }
    Ok(origin)
}

fn decode_mask(input: &str) -> Result<sdf::OpenVdbActiveMask, String> {
    let mut mask = [0; sdf::SDFB_OPENVDB_ACTIVE_MASK_BYTES];
    if input.len() != mask.len() * 2 {
        return Err("OpenVDB mask needs 128 hexadecimal digits".into());
    }
    for (byte, digits) in mask.iter_mut().zip(input.as_bytes().chunks_exact(2)) {
        let high = (digits[0] as char)
            .to_digit(16)
            .ok_or("Invalid hexadecimal mask")?;
        let low = (digits[1] as char)
            .to_digit(16)
            .ok_or("Invalid hexadecimal mask")?;
        *byte = (high * 16 + low) as u8;
    }
    Ok(mask)
}
