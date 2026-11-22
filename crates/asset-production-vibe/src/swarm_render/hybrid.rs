//! Hardware-only native offscreen scene rendering with retained 10D admission.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{Args, AssetHost, Value};

#[cfg(all(feature = "gpu-vbd-contact", not(target_arch = "wasm32")))]
use crate::{
    artifact::{FieldLattice, HmcWorkspaceView, LatticeSamples, RadianceLattice},
    fields, integer, list, number, record, required, text, vector, TypedArtifact,
};

impl AssetHost<'_> {
    pub(crate) fn render_gpu_hybrid(&mut self, args: &Args) -> Result<Value, String> {
        render(self, args)
    }
}

#[cfg(not(all(feature = "gpu-vbd-contact", not(target_arch = "wasm32"))))]
fn render(_host: &mut AssetHost<'_>, _args: &Args) -> Result<Value, String> {
    Err(
        "CapabilityUnavailable: native hardware rendering requires the gpu-vbd-contact feature"
            .into(),
    )
}

#[cfg(all(feature = "gpu-vbd-contact", not(target_arch = "wasm32")))]
fn render(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    use qualia_core_db::bundle::{BundleReadLimits, BundleReader};
    use qualia_core_db::render::{
        gpu::PortalGpu,
        portal_load_10d::{
            assembly_residency_digest, decode_hmc_leaf_assembly, decode_load_10d,
            upload_load_10d_to_gpu, Load10dGpuBuffers, LoadAssemblyGpuBuffers,
        },
        telemetry::SystemTelemetry,
    };
    use std::sync::Arc;

    use crate::qualia_containers_governance_poet_ops::encoded_resource;

    enum Drawing {
        Standalone(Load10dGpuBuffers),
        Assembly(LoadAssemblyGpuBuffers),
    }

    fields(
        args,
        &[
            "source",
            "entry",
            "resolution",
            "quality_profile",
            "camera",
            "clear_linear_rgba",
        ],
    )?;
    let [w, h] = list(required(args, "resolution")?)? else {
        return Err("GPU hybrid resolution must be [width,height]".into());
    };
    let (w, h) = (integer(w, "width")?, integer(h, "height")?);
    let profile = args
        .get("quality_profile")
        .map(text)
        .transpose()?
        .unwrap_or("preview");
    let maximum = match profile {
        "preview" => 1024,
        "production16" => 4096,
        _ => return Err("Quality profile must be preview or production16".into()),
    };
    if !(1..=maximum).contains(&w) || !(1..=maximum).contains(&h) {
        return Err(format!(
            "GPU hybrid resolution must be within 1..={maximum}"
        ));
    }
    let pixel_count = w * h;
    if pixel_count * 4 > host.limits.resource_bytes {
        return Err("GPU hybrid RGBA8 output exceeds the resource budget".into());
    }
    let camera = record(required(args, "camera")?)?;
    fields(camera, &["yaw_rad", "pitch_rad", "zoom", "target"])?;
    let yaw = number(camera, "yaw_rad", 0.0)?;
    let pitch = number(camera, "pitch_rad", 0.25)?;
    let zoom = number(camera, "zoom", 3.5)?;
    let target = camera
        .get("target")
        .map(vector::<3>)
        .transpose()?
        .unwrap_or([0.0; 3]);
    if yaw.abs() > std::f64::consts::TAU
        || !(-1.45..=1.45).contains(&pitch)
        || !(0.05..=5000.0).contains(&zoom)
        || target.iter().any(|p| !(-10_000.0..=10_000.0).contains(p))
    {
        return Err("GPU hybrid camera is outside the admitted native orbit range".into());
    }
    let clear = args
        .get("clear_linear_rgba")
        .map(vector::<4>)
        .transpose()?
        .unwrap_or([0.0, 0.0, 0.0, 1.0]);
    if clear.iter().any(|p| !(0.0..=1.0).contains(p)) {
        return Err("Clear color must be finite linear RGBA in [0,1]".into());
    }
    host.charge(pixel_count * 24, pixel_count * 8)?;
    let source = required(args, "source")?;
    let entry = args.get("entry").map(text).transpose()?;
    let (kind, source_bytes) = match host.workspace.typed(source) {
        Ok(TypedArtifact::SceneGraph10d(_)) => {
            return Err(
                "A typed scene graph requires its sealed HMC to resolve retained leaf dependencies"
                    .into(),
            );
        }
        Ok(TypedArtifact::HmcBundle(bundle)) => match bundle.as_ref() {
            HmcWorkspaceView::Sealed(bytes) => ("hmc", bytes.as_ref()),
            HmcWorkspaceView::Staged(_) => {
                return Err("GPU hybrid rendering requires a sealed HMC".into())
            }
        },
        Ok(TypedArtifact::FieldLattice(field)) => match field.as_ref() {
            FieldLattice::TenD(fields) => ("10d", fields.as_10d_bytes()),
            _ => return Err("GPU hybrid rendering requires a 10D field lattice".into()),
        },
        _ => {
            let resource = encoded_resource(host, source)?;
            (resource.kind.as_str(), resource.bytes.as_slice())
        }
    };
    if source_bytes.len() > host.limits.resource_bytes || source_bytes.len() > 128 * 1024 * 1024 {
        return Err("GPU hybrid source exceeds the admitted cold resource budget".into());
    }
    let (drawing, source_frame) = match (kind, entry) {
        ("10d", None) => {
            require_standalone(source_bytes)?;
            (
                Drawing::Standalone(decode_load_10d(source_bytes)?),
                "portal_orbit",
            )
        }
        ("hmc", Some(key)) => {
            let bundle = BundleReader::parse_with_limits(
                source_bytes,
                BundleReadLimits {
                    file_bytes: host.limits.resource_bytes,
                    entries: host.limits.resources,
                    ..BundleReadLimits::default()
                },
            )
            .map_err(|e| format!("GPU hybrid HMC admission: {e}"))?;
            let metadata = bundle
                .entry(key)
                .filter(|entry| entry.kind == "10d")
                .ok_or("GPU hybrid HMC entry must identify a 10D asset")?;
            if !bundle.verify_entry(key) {
                return Err("GPU hybrid HMC entry digest mismatch".into());
            }
            let data = bundle
                .get(key)
                .ok_or("GPU hybrid HMC entry is missing its bytes")?;
            if assembly_residency_digest(&bundle, data, metadata.sha256)?.is_some() {
                (
                    Drawing::Assembly(decode_hmc_leaf_assembly(&bundle, key)?),
                    "authored_assembly",
                )
            } else {
                (Drawing::Standalone(decode_load_10d(data)?), "portal_orbit")
            }
        }
        _ => return Err(
            "GPU hybrid source must be a standalone retained 10d or an HMC with its 10d entry key"
                .into(),
        ),
    };
    let (materials, triangles) = match &drawing {
        Drawing::Standalone(mesh) => {
            if mesh.governance_refused {
                return Err("GPU hybrid source is governance-refused".into());
            }
            (&mesh.materials, mesh.triangle_count)
        }
        Drawing::Assembly(mesh) => (&mesh.materials, mesh.triangle_count),
    };
    if triangles == 0 {
        return Err("GPU hybrid source has no renderable triangles".into());
    }
    if materials.iter().any(|m| {
        [
            m.base_color_texture,
            m.normal_texture,
            m.metallic_roughness_texture,
            m.occlusion_texture,
            m.emissive_texture,
            m.stylized_ramp_texture,
        ]
        .iter()
        .any(|hash| *hash != [0; 32])
    }) {
        return Err(
            "GPU hybrid branch requires materials without external texture dependencies".into(),
        );
    }
    host.charge(
        triangles as usize * 60,
        triangles as usize * 160 + pixel_count * 8,
    )?;
    let mut renderer = PortalGpu::new_offscreen_shared_hardware(w as u32, h as u32)?;
    let [r, g, b, a] = clear;
    renderer.set_clear_color(r, g, b, a);
    let [x, y, z] = target.map(|v| v as f32);
    renderer.set_camera_target(yaw as f32, pitch as f32, zoom as f32, x, y, z);
    match &drawing {
        Drawing::Standalone(mesh) => {
            if upload_load_10d_to_gpu(&mut renderer, mesh)? == 0 {
                return Err("GPU hybrid renderer did not admit the mesh".into());
            }
        }
        Drawing::Assembly(mesh) => mesh.upload(&mut renderer)?,
    }
    renderer.render(0.0, &SystemTelemetry::default())?;
    let mut pixels = vec![0_u8; pixel_count * 4];
    if renderer.read_rgba8_into(&mut pixels)? != pixels.len() || renderer.is_device_lost() {
        return Err("GPU hybrid device did not provide a complete RGBA8 frame".into());
    }
    let adapter = renderer
        .gpu_timing_adapter_info()
        .ok_or("Missing GPU hybrid adapter receipt")?;
    let adapter_name = adapter.name.clone();
    let backend = format!("{:?}", adapter.backend);
    let image = host.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(RadianceLattice {
        width: w as u32,
        height: h as u32,
        channels: 4,
        primaries_iri: "urn:qualia:color:primaries:rec709".into(),
        transfer_iri: "urn:qualia:color:transfer:linear".into(),
        samples: LatticeSamples::U8(pixels),
    })))?;
    Ok(Value::Record(Args::from([
        ("radiance".into(), image),
        ("adapter".into(), Value::String(adapter_name)),
        ("backend".into(), Value::String(backend)),
        ("source_frame".into(), Value::String(source_frame.into())),
        ("triangle_count".into(), Value::U64(u64::from(triangles))),
    ])))
}

#[cfg(all(feature = "gpu-vbd-contact", not(target_arch = "wasm32")))]
fn require_standalone(bytes: &[u8]) -> Result<(), String> {
    use qualia_core_db::container_10d::{
        TypedFieldRecord, TypedPayloadResolver, FIELD_KIND_ARCHETYPE_REF,
        FIELD_KIND_INSTANCE_BATCH, FIELD_KIND_INSTANCE_OVERRIDES, MANIFOLD_IDENTITY_V2_MAX_FIELDS,
    };

    let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = scratch;
    let view = TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut fields)
        .map_err(|error| error.to_string())?;
    let count = view.payloads_of_kind(FIELD_KIND_ARCHETYPE_REF).count();
    let has = |kind| view.payloads_of_kind(kind).next().is_some();
    if count > 1
        || (count == 1 && has(FIELD_KIND_INSTANCE_BATCH) && has(FIELD_KIND_INSTANCE_OVERRIDES))
    {
        return Err(
            "External leaf assemblies require the sealed HMC containing their dependencies".into(),
        );
    }
    Ok(())
}
