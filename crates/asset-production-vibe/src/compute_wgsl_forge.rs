//! Closed, bounded WGSL Forge authoring for the canonical affine-f32 kernel.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    artifact::FieldLattice, boolean, fields, list, num, required, text, Args, AssetHost,
    TypedArtifact, Value,
};
use serde_json::{json, Value as Receipt};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, sync::Arc};

const MAX_ELEMENTS: usize = 65_536;
const MAX_RANK: usize = 8;
const MAX_MAGNITUDE: f32 = 1e12;

struct Tensor {
    axes: Vec<u32>,
    samples: Vec<f32>,
}

impl AssetHost<'_> {
    pub(crate) fn dispatch_wgsl_forge(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "kernel",
                "input",
                "scale",
                "bias",
                "target_backend",
                "verify_cpu_oracle",
            ],
        )?;
        if text(required(args, "kernel")?)? != "affine-f32" {
            return Err("WGSL Forge authoring admits the canonical affine-f32 built-in".into());
        }
        let backend = args
            .get("target_backend")
            .map(text)
            .transpose()?
            .unwrap_or("cpu_reference");
        let scale = bounded_scalar(num(required(args, "scale")?)? as f32)?;
        let bias = bounded_scalar(num(required(args, "bias")?)? as f32)?;
        let verify = boolean(args, "verify_cpu_oracle", true)?;
        let input = read_tensor(self, required(args, "input")?)?;
        self.charge(
            input.samples.len() * 16,
            input.samples.len() * 48 + (64 << 10),
        )?;
        let count = input.samples.len();
        let input_digest = tensor_digest(&input.axes, &input.samples);
        let result = match backend {
            "cpu_reference" => (
                input
                    .samples
                    .iter()
                    .map(|value| value.mul_add(scale, bias))
                    .collect(),
                json!({"status": "cpu_reference", "gpu_executed": false}),
            ),
            "wgsl_gpu" => dispatch_gpu(&input.samples, scale, bias, verify)?,
            _ => return Err("target_backend must be cpu_reference or wgsl_gpu".into()),
        };
        let (samples, execution) = result;
        if samples.len() != count || samples.iter().any(|value| !value.is_finite()) {
            return Err("WGSL Forge affine executor returned an invalid output tensor".into());
        }
        let receipt = json!({
            "schema": "qualia.wgsl-forge-execution/v1",
            "kernel": "affine-f32",
            "target_backend": backend,
            "axes": &input.axes,
            "element_count": count,
            "input_sha256": input_digest,
            "output_sha256": tensor_digest(&input.axes, &samples),
            "scale_f32_bits": scale.to_bits(),
            "bias_f32_bits": bias.to_bits(),
            "execution": execution,
        });
        let receipt = serde_json::to_vec(&receipt).map_err(|error| error.to_string())?;
        let output = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: input.axes,
                samples,
            },
        )))?;
        let receipt = self.admit_encoded("json", receipt)?;
        Ok(Value::Record(BTreeMap::from([
            ("output".into(), output),
            ("receipt".into(), receipt),
            ("target_backend".into(), Value::String(backend.into())),
            ("element_count".into(), Value::U64(count as u64)),
        ])))
    }
}

fn read_tensor(host: &AssetHost<'_>, value: &Value) -> Result<Tensor, String> {
    let (axes, samples) = match host.workspace.typed(value) {
        Ok(TypedArtifact::FieldLattice(field)) => {
            let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
                return Err("WGSL Forge input artifact must be DenseF32".into());
            };
            validate_shape(axes, samples.len())?;
            (axes.clone(), samples.clone())
        }
        Ok(_) => {
            return Err("WGSL Forge input must be a DenseF32 lattice or flat numeric list".into())
        }
        Err(_) => {
            let values = list(value)?;
            if !(1..=MAX_ELEMENTS).contains(&values.len()) {
                return Err(format!("WGSL Forge accepts 1..={MAX_ELEMENTS} elements"));
            }
            let samples = values
                .iter()
                .map(|v| num(v).map(|value| value as f32))
                .collect::<Result<Vec<_>, _>>()?;
            (vec![values.len() as u32], samples)
        }
    };
    for &value in &samples {
        bounded_scalar(value)?;
    }
    Ok(Tensor { axes, samples })
}

fn validate_shape(axes: &[u32], count: usize) -> Result<(), String> {
    if !(1..=MAX_RANK).contains(&axes.len()) || !(1..=MAX_ELEMENTS).contains(&count) {
        return Err("WGSL Forge input exceeds its tensor rank or element limits".into());
    }
    let extent = axes
        .iter()
        .try_fold(1usize, |product, &axis| product.checked_mul(axis as usize));
    if extent != Some(count) {
        return Err("WGSL Forge tensor shape differs from its sample count".into());
    }
    Ok(())
}

fn bounded_scalar(value: f32) -> Result<f32, String> {
    if value.is_finite() && value.abs() <= MAX_MAGNITUDE && (value == 0.0 || value.is_normal()) {
        Ok(value)
    } else {
        Err("WGSL Forge input values must be zero or normal finite f32 within ±1e12".into())
    }
}

fn tensor_digest(axes: &[u32], samples: &[f32]) -> String {
    let mut hash = Sha256::new();
    hash.update(b"QUALIA-FORGE-DENSE-F32-v1\0");
    hash.update([axes.len() as u8]);
    for axis in axes {
        hash.update(axis.to_le_bytes());
    }
    for sample in samples {
        hash.update(sample.to_le_bytes());
    }
    format!("{:x}", hash.finalize())
}

#[cfg(not(all(feature = "gpu-vbd-contact", not(target_arch = "wasm32"))))]
fn dispatch_gpu(
    _input: &[f32],
    _scale: f32,
    _bias: f32,
    _verify: bool,
) -> Result<(Vec<f32>, Receipt), String> {
    Err("CapabilityUnavailable: native GPU runtime feature is disabled for WGSL Forge".into())
}

#[cfg(all(feature = "gpu-vbd-contact", not(target_arch = "wasm32")))]
fn dispatch_gpu(
    input: &[f32],
    scale: f32,
    bias: f32,
    verify: bool,
) -> Result<(Vec<f32>, Receipt), String> {
    use qualia_core_db::{
        etalon_wgsl_forge::numerical::{
            emit_shader,
            execute::{BindingUsage, QualiaCompute, WgpuComputeContext, WgpuPipeline},
            oracle::{affine_cpu, compare_f32, AffineParams, OracleTolerance},
            validate_wgsl, BuiltinKernel, ForgeError, Schedule, TargetBackend,
        },
        gpu_context::{global_vram_ledger, try_shared_gpu, VramResourceClass},
    };
    use std::mem::size_of_val;
    let owner = std::thread::scope(|scope| scope.spawn(try_shared_gpu).join())
        .map_err(|_| "CapabilityUnavailable: process Etalon device initialization failed")?
        .ok_or("CapabilityUnavailable: process Etalon device is unavailable")?;
    if !matches!(
        owner.adapter_caps.device_type,
        etalon::DeviceType::DiscreteGpu | etalon::DeviceType::IntegratedGpu
    ) {
        return Err("CapabilityUnavailable: WGSL Forge requires an admitted hardware GPU".into());
    }
    let bytes = size_of_val(input);
    let slab_capacity = (bytes * 2 + 512).next_multiple_of(256).max(4_096);
    let limits = owner.device.limits();
    if slab_capacity as u64 > limits.max_buffer_size
        || bytes as u64 > u64::from(limits.max_storage_buffer_binding_size)
    {
        return Err("CapabilityUnavailable: affine buffers exceed this GPU device's bounds".into());
    }
    let reservation = (3 * slab_capacity + 3 * bytes + (64 << 10)) as u64;
    let _reservation = global_vram_ledger()
        .try_reserve_graphics(VramResourceClass::UploadStaging, reservation)
        .map_err(|error| format!("WGSL Forge staging reservation: {error}"))?;
    let error = |error| match error {
        ForgeError::GpuUnavailable(reason) => format!("CapabilityUnavailable: {reason}"),
        error => format!("WGSL Forge affine dispatch: {error:?}"),
    };
    let mut context = WgpuComputeContext::from_device(
        owner.device.clone(),
        owner.queue.clone(),
        &owner.adapter_caps,
        slab_capacity,
    )
    .map_err(error)?;
    let kernel = BuiltinKernel::AffineF32.spec();
    let schedule = Schedule::default();
    schedule
        .validate(&kernel, &context.constraints)
        .map_err(error)?;
    if schedule.dispatch_workgroups(input.len()) > limits.max_compute_workgroups_per_dimension {
        return Err(
            "CapabilityUnavailable: affine workgroups exceed this GPU device's bounds".into(),
        );
    }
    let generated = emit_shader(&kernel, schedule, TargetBackend::Wgsl).map_err(error)?;
    let validation = validate_wgsl(&generated.source).map_err(error)?;
    if !validation.naga_validated {
        return Err("WGSL Forge affine shader did not pass native validation".into());
    }
    let params = AffineParams {
        length: input.len() as u32,
        scale,
        bias,
        _pad: 0,
    };
    let execution = (|| {
        let view_in = context
            .allocate_and_write(bytemuck::cast_slice(input), 0, 0, BindingUsage::StorageRead)
            .map_err(error)?;
        let view_out = context
            .allocate_transient(bytes, 1, 0, BindingUsage::StorageReadWrite)
            .map_err(error)?;
        let view_params = context
            .allocate_and_write(bytemuck::bytes_of(&params), 2, 0, BindingUsage::Uniform)
            .map_err(error)?;
        let pipeline = WgpuPipeline::compile(&context, &generated.source, &kernel.entry_point)
            .map_err(error)?;
        let elapsed_ns = pipeline
            .dispatch(&[view_in, view_out, view_params], &schedule, input.len())
            .map_err(error)?;
        let mut output = vec![0.0; input.len()];
        let count = context
            .read_buffer_f32_into(&view_out, &mut output)
            .map_err(error)?;
        if count != output.len() {
            return Err("WGSL Forge returned an incomplete affine readback".into());
        }
        Ok::<_, String>((output, elapsed_ns))
    })();
    context.clear_transient_allocations();
    let (output, elapsed_ns) = execution?;
    let comparison = if verify {
        let reference = affine_cpu(input, params);
        let tolerance = OracleTolerance::default();
        let report = compare_f32(&reference, &output, tolerance);
        if !report.passed() {
            return Err(format!(
                "Affine GPU oracle mismatch: {} elements, first {:?}, max absolute error {}, max relative error {}",
                report.mismatch_count, report.first_mismatch, report.max_absolute_error, report.max_relative_error,
            ));
        }
        json!({
            "status": "passed", "compared": report.compared,
            "max_absolute_error": report.max_absolute_error, "max_relative_error": report.max_relative_error,
            "absolute_tolerance": tolerance.absolute, "relative_tolerance": tolerance.relative,
        })
    } else {
        json!({"status": "disabled"})
    };
    Ok((
        output,
        json!({
            "status": "dispatched", "gpu_executed": true,
            "adapter": context.adapter.name, "adapter_backend": context.adapter.backend,
            "naga_validated": validation.naga_validated,
            "source_hash": generated.source_hash, "semantic_hash": generated.semantic_hash,
            "workgroup_size": schedule.workgroup_size,
            "comparison": comparison,
            "elapsed_ns": elapsed_ns,
            "clock": if owner.timestamps_supported { "gpu_timestamp" } else { "completion_clock" },
        }),
    ))
}
