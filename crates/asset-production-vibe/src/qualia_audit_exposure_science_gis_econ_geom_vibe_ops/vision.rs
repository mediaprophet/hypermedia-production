//! Bounded image and optical observation adapters over retained buffers.

use crate::{
    count, fields, integer, list, num, required, text, vector, Args, Artifact, AssetHost, Raster,
    Value,
};
use qualia_core_db::specialized_libs::computer_vision::{
    bio::{
        medical::{
            isotropic_resample_3d_nn,
            spectral_unmix_nnls::{self, MAX_CHANNELS, MAX_COMPONENTS},
        },
        morphology::positive_od_threshold::{
            positive_od_threshold, OdPositiveIndex, MAX_OD_LABELS,
        },
    },
    cv::buffer::GrayView,
    sr::{super_resolve_tiled, ClassicalKernel, EnhancementMode, SrBackend, SrRequest, TilePolicy},
};
use std::collections::{BTreeMap, BTreeSet};

const MAX_EDGE: usize = 4096;
const MAX_VOXELS: usize = 1_048_576;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Value, String> {
    match op {
        "super_resolve_tiled" => super_resolve(host, args),
        "spectral_unmix" => spectral_unmix(host, args),
        "isotropic_volume_nn" => isotropic_volume(host, args),
        "positive_od_index" => positive_index(host, args),
        _ => Err(format!("Unsupported vision operation: {op}")),
    }
}

fn retained<'a>(
    host: &'a AssetHost<'_>,
    value: &Value,
    kind: &str,
    len: usize,
) -> Result<&'a [u8], String> {
    let resource = crate::qualia_containers_governance_poet_ops::encoded_resource(host, value)?;
    if resource.kind != kind || resource.bytes.len() != len || len > host.limits.resource_bytes {
        return Err(format!(
            "Expected exactly {len} bytes of retained {kind} data"
        ));
    }
    Ok(&resource.bytes)
}

fn dimensions<const N: usize>(value: &Value) -> Result<[usize; N], String> {
    let values =
        <&[Value; N]>::try_from(list(value)?).map_err(|_| format!("Expected {N} dimensions"))?;
    let mut dimensions = [0; N];
    for (out, value) in dimensions.iter_mut().zip(values) {
        let dimension = integer(value, "dimension")?;
        if !(1..=MAX_EDGE).contains(&dimension) {
            return Err(format!("Dimensions must be within 1..={MAX_EDGE}"));
        }
        *out = dimension;
    }
    Ok(dimensions)
}

fn super_resolve(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(
        args,
        &["image", "dimensions", "scale", "kernel", "tile", "overlap"],
    )?;
    let source = required(args, "image")?;
    let raster = match host.workspace.get(source) {
        Ok(Artifact::Image(image)) => Some(image),
        _ => None,
    };
    let [width, height] = match (raster, args.get("dimensions")) {
        (Some(image), None) => [image.width as usize, image.height as usize],
        (None, Some(value)) => dimensions::<2>(value)?,
        _ => return Err("Provide an image handle or retained RGB8 with its dimensions".into()),
    };
    let scale = count(args, "scale", 2, 4)?;
    if scale < 2 || width * scale > MAX_EDGE || height * scale > MAX_EDGE {
        return Err("Super-resolution scale must be 2–4 within the output size limit".into());
    }
    let kernel = match args
        .get("kernel")
        .map(text)
        .transpose()?
        .unwrap_or("bicubic")
    {
        "nearest" => ClassicalKernel::Nearest,
        "bilinear" => ClassicalKernel::Bilinear,
        "bicubic" => ClassicalKernel::Bicubic,
        "lanczos3" => ClassicalKernel::Lanczos3,
        _ => return Err("Unknown classical resampling kernel".into()),
    };
    let tile = count(args, "tile", 256, 1024)?;
    let overlap = args
        .get("overlap")
        .map(|value| integer(value, "overlap"))
        .transpose()?
        .unwrap_or(32);
    if tile < 32 || overlap > tile / 2 {
        return Err(
            "Use tiles of at least 32 pixels and overlap at most half the tile size".into(),
        );
    }
    let output_pixels = width * height * scale * scale;
    let scratch = output_pixels * 19 + (tile * tile * scale * scale * 3);
    host.charge(output_pixels * 128, scratch)?;
    let rgb = match host.workspace.get(source) {
        Ok(Artifact::Image(image)) => image.rgb.as_slice(),
        _ => retained(host, source, "rgb8", width * height * 3)?,
    };
    let mut pixels = vec![0; output_pixels * 3];
    let request = SrRequest {
        rgb,
        width: width as u32,
        height: height as u32,
        scale: scale as u8,
        backend: SrBackend::Classical(kernel),
        mode: EnhancementMode::Sharpen,
    };
    let report = super_resolve_tiled(
        &request,
        TilePolicy {
            tile_w: tile as u32,
            tile_h: tile as u32,
            overlap: overlap as u32,
            max_tiles: 4096,
        },
        &mut pixels,
    )
    .map_err(|error| format!("Tiled resampling: {error:?}"))?;
    let image = host.workspace.insert(
        Artifact::Image(Raster {
            width: report.out_width,
            height: report.out_height,
            rgb: pixels,
        }),
        host.limits,
    )?;
    Ok(Value::Record(BTreeMap::from([
        ("image".into(), image),
        (
            "dimensions".into(),
            sizes([report.out_width as usize, report.out_height as usize]),
        ),
        ("backend".into(), Value::String(report.backend_id.into())),
        ("tiles".into(), Value::U64(u64::from(report.tile_count))),
        ("generative".into(), Value::Bool(report.generative)),
    ])))
}

fn spectrum(value: &Value, max_len: usize) -> Result<Vec<f32>, String> {
    let values = list(value)?;
    if values.is_empty() || values.len() > max_len {
        return Err("Spectrum has an invalid channel count".into());
    }
    values
        .iter()
        .map(|value| {
            let sample = num(value)? as f32;
            if sample.is_finite() && sample >= 0.0 {
                Ok(sample)
            } else {
                Err("Spectral intensities require finite nonnegative values".into())
            }
        })
        .collect()
}

fn spectral_unmix(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["spectrum", "endmembers"])?;
    let observed = spectrum(required(args, "spectrum")?, MAX_CHANNELS)?;
    let endmembers = list(required(args, "endmembers")?)?;
    if !(1..=MAX_COMPONENTS).contains(&endmembers.len()) {
        return Err("Endmembers exceed the native component limit".into());
    }
    let mut matrix = Vec::with_capacity(observed.len() * endmembers.len());
    for row in endmembers {
        let row = spectrum(row, observed.len())?;
        if row.len() != observed.len() || row.iter().all(|sample| *sample == 0.0) {
            return Err(
                "Each endmember requires the same channels as the measured spectrum".into(),
            );
        }
        matrix.extend(row);
    }
    host.charge(matrix.len() * 64 * 3, matrix.len() * 4)?;
    let result = spectral_unmix_nnls::spectral_unmix_nnls(
        &observed,
        &matrix,
        observed.len(),
        endmembers.len(),
    )
    .map_err(|error| format!("Spectral unmix: {error:?}"))?;
    let abundances = &result.abundances[..result.n_components];
    if !result.residual_l2.is_finite() || abundances.iter().any(|value| !value.is_finite()) {
        return Err("Spectral unmix produced non-finite results".into());
    }
    Ok(Value::Record(BTreeMap::from([
        (
            "abundances".into(),
            Value::List(abundances.iter().copied().map(Value::F64).collect()),
        ),
        ("residual_l2".into(), Value::F64(result.residual_l2)),
    ])))
}

fn isotropic_volume(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(
        args,
        &["voxels", "dimensions", "spacing_m", "target_spacing_m"],
    )?;
    let size = dimensions::<3>(required(args, "dimensions")?)?;
    let [w, h, d] = size;
    let spacing = vector::<3>(required(args, "spacing_m")?)?;
    let target = num(required(args, "target_spacing_m")?)?;
    if target <= 0.0 || spacing.iter().any(|component| *component <= 0.0) {
        return Err("Voxel spacing requires positive metre values".into());
    }
    let output_size = std::array::from_fn::<_, 3, _>(|axis| {
        ((size[axis] as f64 * spacing[axis] / target).round() as usize).max(1)
    });
    let voxels = |axes: [usize; 3]| -> Result<usize, String> {
        axes.into_iter()
            .try_fold(1usize, |total, next| total.checked_mul(next))
            .filter(|count| *count <= MAX_VOXELS)
            .ok_or_else(|| "Volume exceeds the voxel limit".into())
    };
    let in_count = voxels(size)?;
    let out_count = voxels(output_size)?;
    host.charge(out_count, (in_count + 2 * out_count) * 4)?;
    let input = retained(host, required(args, "voxels")?, "f32le", in_count * 4)?;
    let input = input
        .chunks_exact(4)
        .map(|word| f32::from_le_bytes([word[0], word[1], word[2], word[3]]))
        .collect::<Vec<_>>();
    if input.iter().any(|value| !value.is_finite()) {
        return Err("Input volume contains a non-finite sample".into());
    }
    let mut out = vec![0.0f32; out_count];
    let [sx, sy, sz] = spacing;
    let (w, h, d) = isotropic_resample_3d_nn(&input, w, h, d, (sx, sy, sz), target, &mut out)
        .map_err(|error| format!("Isotropic resampling: {error:?}"))?;
    let bytes = out.into_iter().flat_map(f32::to_le_bytes).collect();
    Ok(Value::Record(BTreeMap::from([
        ("voxels".into(), host.encode("f32le", bytes)?),
        ("dimensions".into(), sizes([w, h, d])),
        ("spacing_m".into(), Value::F64(target)),
    ])))
}

fn positive_index(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["luminance", "labels", "dimensions", "threshold_u8"])?;
    let [w, h] = dimensions::<2>(required(args, "dimensions")?)?;
    let count = w * h;
    if count > MAX_VOXELS {
        return Err("Optical density image exceeds the pixel limit".into());
    }
    let threshold = u8::try_from(integer(required(args, "threshold_u8")?, "threshold_u8")?)
        .map_err(|_| "Optical density threshold must be 0..=255")?;
    host.charge(count.saturating_mul(MAX_OD_LABELS), count * 2 + 4096)?;
    let gray = retained(host, required(args, "luminance")?, "gray8", count)?;
    let labels = retained(host, required(args, "labels")?, "u16le", count * 2)?;
    let view =
        GrayView::new(w as u32, h as u32, w as u32, gray).ok_or("Invalid luminance frame")?;
    let labels = labels
        .chunks_exact(2)
        .map(|word| u16::from_le_bytes([word[0], word[1]]))
        .collect::<Vec<_>>();
    let mut distinct = BTreeSet::new();
    for &label in &labels {
        if label != 0 {
            distinct.insert(label);
            if distinct.len() > MAX_OD_LABELS {
                return Err("Nucleus count exceeds the native optical density limit".into());
            }
        }
    }
    let mut result = OdPositiveIndex::default();
    positive_od_threshold(view, &labels, threshold, &mut result)
        .map_err(|error| format!("Optical-density analysis: {error:?}"))?;
    Ok(Value::Record(BTreeMap::from([
        (
            "total_nuclei".into(),
            Value::U64(u64::from(result.total_nuclei)),
        ),
        (
            "positive_nuclei".into(),
            Value::U64(u64::from(result.positive_nuclei)),
        ),
        (
            "fraction_q16".into(),
            Value::U64(u64::from(result.fraction_q16)),
        ),
        (
            "mean_od_u8".into(),
            Value::U64(u64::from(result.mean_od_u8)),
        ),
    ])))
}

fn sizes<const N: usize>(dimensions: [usize; N]) -> Value {
    Value::List(
        dimensions
            .into_iter()
            .map(|value| Value::U64(value as u64))
            .collect(),
    )
}
