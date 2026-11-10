//! Etalon/wgpu GPU GI beyond the CPU SDFGI/froxel preview.
//!
//! Cascaded irradiance probes + GGX fragment shading + Henyey-Greenstein
//! froxel fog run as compute passes. Deterministic: no time-based noise.
//! Same inputs → same readback on the same device. CI without a GPU adapter
//! is expected to take the fail-closed `"gpu backend requires"` error path.

use bytemuck::{Pod, Zeroable};

/// Uniforms for [`SDFGI_GPU_WGSL`] (64 bytes, 16-byte aligned).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GiUniforms {
    pub width: u32,
    pub height: u32,
    pub cascades: u32,
    pub _pad0: u32,
    /// xyz = normalized sun direction, w = bounce intensity.
    pub sun_dir_bounce: [f32; 4],
    /// xyz = sky radiance, w = fog density.
    pub sky_fog: [f32; 4],
    /// x = HG anisotropy g, y = roughness, z = sun illuminance, w unused.
    pub g_rough: [f32; 4],
}

const _: [(); 64] = [(); std::mem::size_of::<GiUniforms>()];

/// Inputs for the GPU SDFGI/froxel solve.
#[derive(Clone, Copy, Debug)]
pub struct GpuGiParams {
    pub width: u32,
    pub height: u32,
    pub cascades: u32,
    pub fog_density: f32,
    pub anisotropy_g: f32,
    pub sun_dir: [f32; 3],
    pub sky_radiance: [f32; 3],
    pub bounce_intensity: f32,
}

/// GPU readback: shaded RGBA8 plus receipt stats.
#[derive(Clone, Debug)]
pub struct GpuGiResult {
    pub pixels: Vec<[u8; 4]>,
    pub adapter_name: String,
    pub mean_irradiance: f64,
    pub mean_transmittance: f64,
}

/// WGSL: cascade irradiance convolution (`cascade_probes`) then GGX + HG
/// froxel shade (`shade_froxel`). Semantics follow Etalon
/// `evaluate_fragment_radiance` (GGX D/V/F + probe ambient) and IBL
/// cosine-weighted sky irradiance, with Beer-Lambert + HG inscatter.
pub const SDFGI_GPU_WGSL: &str = r#"
struct GiUniforms {
    width: u32,
    height: u32,
    cascades: u32,
    _pad0: u32,
    sun_dir_bounce: vec4<f32>,
    sky_fog: vec4<f32>,
    g_rough: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: GiUniforms;
@group(0) @binding(1) var<storage, read> input_rgba: array<u32>;
@group(0) @binding(2) var<storage, read_write> probes: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read_write> output_rgba: array<u32>;
@group(0) @binding(4) var<storage, read_write> stats: array<vec2<f32>>;

const PI: f32 = 3.141592653589793;
const SAMPLE_COUNT: u32 = 32u;

fn unpack_rgba(p: u32) -> vec4<f32> {
    return vec4<f32>(
        f32(p & 255u),
        f32((p >> 8u) & 255u),
        f32((p >> 16u) & 255u),
        f32((p >> 24u) & 255u)
    ) / 255.0;
}

fn pack_rgba(c: vec4<f32>) -> u32 {
    let r = u32(clamp(c.r, 0.0, 1.0) * 255.0 + 0.5);
    let g = u32(clamp(c.g, 0.0, 1.0) * 255.0 + 0.5);
    let b = u32(clamp(c.b, 0.0, 1.0) * 255.0 + 0.5);
    let a = u32(clamp(c.a, 0.0, 1.0) * 255.0 + 0.5);
    return r | (g << 8u) | (b << 16u) | (a << 24u);
}

fn radical_inverse_vdc(bits_in: u32) -> f32 {
    var bits = bits_in;
    bits = (bits << 16u) | (bits >> 16u);
    bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);
    bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);
    bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);
    bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);
    return f32(bits) * 2.3283064365386963e-10;
}

fn hammersley(i: u32, n: u32) -> vec2<f32> {
    return vec2<f32>(f32(i) / f32(max(n, 1u)), radical_inverse_vdc(i));
}

fn eval_procedural_sky(dir_in: vec3<f32>) -> vec3<f32> {
    let sun_dir = normalize(u.sun_dir_bounce.xyz);
    let bounce = max(u.sun_dir_bounce.w, 0.05);
    let sky = max(u.sky_fog.xyz, vec3<f32>(0.0));
    let g = clamp(u.g_rough.x, 0.0, 0.96);
    let dir = normalize(dir_in);
    let cos_theta = clamp(dir.y, -1.0, 1.0);
    let cos_gamma = clamp(dot(dir, sun_dir), -1.0, 1.0);
    if (cos_theta < 0.0) {
        let sun_elev = max(sun_dir.y, 0.05);
        return sky * sun_elev * (0.25 + 0.75 * (1.0 + cos_theta)) * bounce;
    }
    let zenith_deg = acos(cos_theta) * 180.0 / PI;
    let optical_mass = min(1.0 / (cos_theta + 0.15 * pow(max(93.885 - zenith_deg, 1.0), -1.253)), 16.0);
    let rayleigh_phase = 0.75 * (1.0 + cos_gamma * cos_gamma);
    let g2 = g * g;
    let mie_phase = (1.0 - g2) / (4.0 * PI * pow(max(1.0 + g2 - 2.0 * g * cos_gamma, 1e-4), 1.5));
    let beta_r = vec3<f32>(5.8e-3, 13.5e-3, 33.1e-3);
    let beta_m = 4.0e-3 * 2.2;
    let ext = exp(-(beta_r + vec3<f32>(beta_m)) * optical_mass);
    let zenith = vec3<f32>(0.22, 0.48, 0.92);
    let inscatter = (zenith * rayleigh_phase + vec3<f32>(1.0, 0.92, 0.80) * mie_phase * 0.12 * 2.2)
        * (1.0 - ext);
    let t = clamp((cos_gamma - 0.9992) / (0.99995 - 0.9992), 0.0, 1.0);
    let solar = t * t * (3.0 - 2.0 * t) * vec3<f32>(18.0, 16.5, 14.0) * ext;
    return (inscatter + solar) * bounce + sky * 0.15;
}

fn distribution_ggx(n_dot_h: f32, roughness: f32) -> f32 {
    let alpha = roughness * roughness;
    let alpha_sq = alpha * alpha;
    let n_dot_h_sq = n_dot_h * n_dot_h;
    let denom = n_dot_h_sq * (alpha_sq - 1.0) + 1.0;
    return alpha_sq / max(PI * denom * denom, 1e-7);
}

fn visibility_smith_ggx(n_dot_v: f32, n_dot_l: f32, roughness: f32) -> f32 {
    let alpha = roughness * roughness;
    let ggx_v = n_dot_l * (n_dot_v * (1.0 - alpha) + alpha);
    let ggx_l = n_dot_v * (n_dot_l * (1.0 - alpha) + alpha);
    return 0.5 / max(ggx_v + ggx_l, 1e-7);
}

fn fresnel_schlick(v_dot_h: f32, f0: vec3<f32>) -> vec3<f32> {
    let f = pow(1.0 - clamp(v_dot_h, 0.0, 1.0), 5.0);
    return f0 + (vec3<f32>(1.0) - f0) * f;
}

fn hg_phase(cos_theta: f32, g: f32) -> f32 {
    let g2 = g * g;
    return (1.0 - g2) / (4.0 * PI * pow(max(1.0 + g2 - 2.0 * g * cos_theta, 1e-4), 1.5));
}

fn evaluate_fragment_radiance(
    base_color: vec3<f32>,
    normal: vec3<f32>,
    view_dir: vec3<f32>,
    sun_dir: vec3<f32>,
    ambient: vec3<f32>,
) -> vec3<f32> {
    let n = normalize(normal);
    let v = normalize(view_dir);
    let n_dot_v = clamp(dot(n, v), 1e-4, 1.0);
    let metallic = 0.0;
    let roughness = max(u.g_rough.y, 0.04);
    let f0 = vec3<f32>(0.04);
    let l = normalize(-sun_dir);
    let h = normalize(v + l);
    let n_dot_l = dot(n, l);
    var direct = vec3<f32>(0.0);
    if (n_dot_l > 0.0) {
        let n_dot_h = clamp(dot(n, h), 0.0, 1.0);
        let v_dot_h = clamp(dot(v, h), 0.0, 1.0);
        let d = n_dot_l / PI;
        let d_term = distribution_ggx(n_dot_h, roughness);
        let v_term = visibility_smith_ggx(n_dot_v, n_dot_l, roughness);
        let f_term = fresnel_schlick(v_dot_h, f0);
        let spec = f_term * (d_term * v_term);
        let illuminance = u.g_rough.z;
        let sun_color = vec3<f32>(1.2, 1.1, 0.9);
        let diff = (1.0 - metallic) * base_color * d;
        direct = (diff + spec) * sun_color * illuminance;
    }
    return direct + base_color * ambient + vec3<f32>(0.0);
}

@compute @workgroup_size(1)
fn cascade_probes(@builtin(global_invocation_id) gid: vec3<u32>) {
    let c = gid.x;
    if (c >= u.cascades || c >= 6u) {
        return;
    }
    let scale = 1.0 / f32(c + 1u);
    let up = vec3<f32>(0.0, 1.0, 0.0);
    let tangent = vec3<f32>(1.0, 0.0, 0.0);
    let bitangent = cross(up, tangent);
    var irradiance = vec3<f32>(0.0);
    for (var i: u32 = 0u; i < SAMPLE_COUNT; i = i + 1u) {
        let xi = hammersley(i, SAMPLE_COUNT);
        let phi = 2.0 * PI * xi.x;
        let cos_theta = sqrt(1.0 - xi.y);
        let sin_theta = sqrt(xi.y);
        let dir = normalize(
            tangent * (cos(phi) * sin_theta) +
            bitangent * (sin(phi) * sin_theta) +
            up * cos_theta
        );
        irradiance += eval_procedural_sky(dir);
    }
    irradiance = irradiance / f32(SAMPLE_COUNT);
    let sky = u.sky_fog.xyz;
    let bounce = u.sun_dir_bounce.w;
    let mixed = (irradiance + sky * scale) * bounce;
    probes[c] = vec4<f32>(mixed, 1.0);
}

@compute @workgroup_size(8, 8, 1)
fn shade_froxel(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = gid.x;
    let y = gid.y;
    if (x >= u.width || y >= u.height) {
        return;
    }
    let idx = y * u.width + x;
    let pix = unpack_rgba(input_rgba[idx]);
    let u_n = (f32(x) + 0.5) / f32(u.width);
    let v_n = (f32(y) + 0.5) / f32(u.height);
    let nx = (u_n - 0.5) * 2.0;
    let ny = (0.5 - v_n) * 2.0;
    let ray_len = sqrt(nx * nx + ny * ny + 1.0);
    let ray = vec3<f32>(nx / ray_len, ny / ray_len, 1.0 / ray_len);

    let cascade_count = max(u.cascades, 1u);
    var ambient = vec3<f32>(0.0);
    var wsum = 0.0;
    for (var c: u32 = 0u; c < cascade_count; c = c + 1u) {
        let w = 1.0 / f32(c + 1u);
        ambient += probes[c].xyz * w;
        wsum += w;
    }
    ambient = ambient / max(wsum, 1e-4);

    let fog = u.sky_fog.w;
    let g = u.g_rough.x;
    let fog_t = exp(-fog * (0.35 + 0.45 * (1.0 - v_n)));
    let dist = 400.0 + fog * 1200.0 * (1.0 - v_n);
    let trans_mean = exp(-fog * dist * 0.0008);
    let transmittance = clamp(0.55 * trans_mean + 0.45 * fog_t, 1e-4, 1.0);
    let cos_sun = clamp(dot(ray, normalize(u.sun_dir_bounce.xyz)), -1.0, 1.0);
    let inscatter = u.sky_fog.xyz * (1.0 - transmittance) * hg_phase(cos_sun, g) * 4.0;
    let sky = eval_procedural_sky(ray);

    let n = vec3<f32>(0.0, 1.0, 0.0);
    let view = vec3<f32>(0.0, 0.2, 1.0);
    let frag = evaluate_fragment_radiance(pix.xyz, n, view, u.sun_dir_bounce.xyz, ambient);
    let mean_irr = max((ambient.x + ambient.y + ambient.z) / 3.0, 0.0)
        + u.sun_dir_bounce.w * 0.05;
    stats[idx] = vec2<f32>(mean_irr, transmittance);

    let lit = frag * transmittance + inscatter + sky * (1.0 - transmittance) * 0.25;
    let mapped = clamp(lit / (1.0 + lit) * 1.4, vec3<f32>(0.0), vec3<f32>(1.0));
    output_rgba[idx] = pack_rgba(vec4<f32>(mapped, pix.a));
}
"#;

/// Dispatch cascaded irradiance probes and GGX/HG froxel shading on GPU.
/// Fail-closed: never returns Ok after silently running CPU.
pub fn dispatch_gpu_sdfgi(
    pixels: &[[u8; 4]],
    params: GpuGiParams,
) -> Result<GpuGiResult, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (pixels, params);
        Err(
            "gpu backend requires a native wgpu/etalon adapter (QualiaDB PortalGpu is WASM; produce path is native)"
                .into(),
        )
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        dispatch_gpu_sdfgi_native(pixels, params)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn dispatch_gpu_sdfgi_native(
    pixels: &[[u8; 4]],
    params: GpuGiParams,
) -> Result<GpuGiResult, String> {
    use std::future::Future;
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};
    use std::time::Duration;
    use wgpu::util::DeviceExt;

    let n = (params.width as usize)
        .checked_mul(params.height as usize)
        .ok_or_else(|| "gpu backend requires finite lattice dimensions".to_string())?;
    if pixels.len() != n || n == 0 {
        return Err("gpu backend requires width*height RGBA lattice matching the input".into());
    }
    if !(1..=6).contains(&params.cascades) {
        return Err("gpu backend requires cascades in 1..=6".into());
    }

    fn block_on<F: Future>(fut: F) -> F::Output {
        struct ThreadWake(std::thread::Thread);
        impl Wake for ThreadWake {
            fn wake(self: Arc<Self>) {
                self.0.unpark();
            }
        }
        let waker = Waker::from(Arc::new(ThreadWake(std::thread::current())));
        let mut cx = Context::from_waker(&waker);
        let mut fut = std::pin::pin!(fut);
        loop {
            match fut.as_mut().poll(&mut cx) {
                Poll::Ready(v) => return v,
                Poll::Pending => std::thread::park_timeout(Duration::from_millis(5)),
            }
        }
    }

    let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
    if cfg!(target_os = "windows") {
        desc.backends = wgpu::Backends::DX12 | wgpu::Backends::VULKAN | wgpu::Backends::GL;
    }
    let instance = wgpu::Instance::new(desc);

    let adapter = match block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: None,
        ..Default::default()
    })) {
        Ok(adapter) => adapter,
        Err(_) => match block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: None,
            ..Default::default()
        })) {
            Ok(adapter) => adapter,
            Err(_) => block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                force_fallback_adapter: true,
                compatible_surface: None,
                ..Default::default()
            }))
            .map_err(|e| {
                format!("gpu backend requires wgpu/etalon adapter (none available): {e}")
            })?,
        },
    };

    let info = adapter.get_info();
    let adapter_name = if info.name.trim().is_empty() {
        format!("{:?}", info.backend)
    } else {
        info.name.clone()
    };

    let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("hypermedia.radiance.solve_sdfgi_froxel gpu"),
        required_features: wgpu::Features::empty(),
        required_limits: adapter.limits(),
        ..Default::default()
    }))
    .map_err(|e| format!("gpu backend requires wgpu/etalon device: {e}"))?;

    let uniforms = GiUniforms {
        width: params.width,
        height: params.height,
        cascades: params.cascades,
        _pad0: 0,
        sun_dir_bounce: [
            params.sun_dir[0],
            params.sun_dir[1],
            params.sun_dir[2],
            params.bounce_intensity,
        ],
        sky_fog: [
            params.sky_radiance[0],
            params.sky_radiance[1],
            params.sky_radiance[2],
            params.fog_density,
        ],
        g_rough: [params.anisotropy_g, 0.55, 1.8, 0.0],
    };

    let mut packed = vec![0u32; n];
    for (dst, px) in packed.iter_mut().zip(pixels.iter()) {
        *dst = u32::from_le_bytes(*px);
    }

    let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("sdfgi_gi_uniforms"),
        contents: bytemuck::bytes_of(&uniforms),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let input_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("sdfgi_input_rgba"),
        contents: bytemuck::cast_slice(&packed),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });
    let probe_bytes = (params.cascades as usize).max(1) * std::mem::size_of::<[f32; 4]>();
    let probe_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sdfgi_cascade_probes"),
        size: probe_bytes as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let output_bytes = (n * 4) as u64;
    let output_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sdfgi_output_rgba"),
        size: output_bytes,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let stats_bytes = (n * 8) as u64;
    let stats_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sdfgi_stats"),
        size: stats_bytes,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let staging_out = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sdfgi_staging_rgba"),
        size: output_bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let staging_stats = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("sdfgi_staging_stats"),
        size: stats_bytes,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("sdfgi_gpu_gi_wgsl"),
        source: wgpu::ShaderSource::Wgsl(SDFGI_GPU_WGSL.into()),
    });

    let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("sdfgi_gpu_bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("sdfgi_gpu_pl"),
        bind_group_layouts: &[Some(&bgl)],
        immediate_size: 0,
    });

    let probe_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("sdfgi_cascade_probes"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("cascade_probes"),
        compilation_options: Default::default(),
        cache: None,
    });
    let shade_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("sdfgi_shade_froxel"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("shade_froxel"),
        compilation_options: Default::default(),
        cache: None,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("sdfgi_gpu_bg"),
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: input_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: probe_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: output_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: stats_buf.as_entire_binding(),
            },
        ],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("sdfgi_gpu_encoder"),
    });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("sdfgi_cascade_probes_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&probe_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(params.cascades, 1, 1);
    }
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("sdfgi_shade_froxel_pass"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&shade_pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(params.width.div_ceil(8), params.height.div_ceil(8), 1);
    }
    encoder.copy_buffer_to_buffer(&output_buf, 0, &staging_out, 0, output_bytes);
    encoder.copy_buffer_to_buffer(&stats_buf, 0, &staging_stats, 0, stats_bytes);
    queue.submit(Some(encoder.finish()));

    let map_buf = |device: &wgpu::Device, buf: &wgpu::Buffer| -> Result<Vec<u8>, String> {
        let slice = buf.slice(..);
        let (send, recv) = std::sync::mpsc::sync_channel(1);
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = send.send(result);
        });
        device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| format!("gpu backend requires device poll: {e}"))?;
        recv.recv_timeout(Duration::from_secs(30))
            .map_err(|_| "gpu backend requires mapped readback".to_string())?
            .map_err(|e| format!("gpu backend requires mapped buffer: {e}"))?;
        let view = slice
            .get_mapped_range()
            .map_err(|e| format!("gpu backend requires mapped range: {e}"))?;
        let data = view.to_vec();
        drop(view);
        buf.unmap();
        Ok(data)
    };

    let out_bytes = map_buf(&device, &staging_out)?;
    let stats_bytes_v = map_buf(&device, &staging_stats)?;
    if out_bytes.len() < n * 4 {
        return Err("gpu backend requires complete RGBA8 readback".into());
    }
    if stats_bytes_v.len() < n * 8 {
        return Err("gpu backend requires complete stats readback".into());
    }

    let mut out_pixels = Vec::with_capacity(n);
    for i in 0..n {
        let o = i * 4;
        out_pixels.push([
            out_bytes[o],
            out_bytes[o + 1],
            out_bytes[o + 2],
            out_bytes[o + 3],
        ]);
    }
    let mut total_irr = 0.0f64;
    let mut total_trans = 0.0f64;
    for i in 0..n {
        let o = i * 8;
        let irr = f32::from_le_bytes([
            stats_bytes_v[o],
            stats_bytes_v[o + 1],
            stats_bytes_v[o + 2],
            stats_bytes_v[o + 3],
        ]) as f64;
        let trans = f32::from_le_bytes([
            stats_bytes_v[o + 4],
            stats_bytes_v[o + 5],
            stats_bytes_v[o + 6],
            stats_bytes_v[o + 7],
        ]) as f64;
        total_irr += irr;
        total_trans += trans;
    }
    let nf = n as f64;

    Ok(GpuGiResult {
        pixels: out_pixels,
        adapter_name,
        mean_irradiance: total_irr / nf,
        mean_transmittance: total_trans / nf,
    })
}
