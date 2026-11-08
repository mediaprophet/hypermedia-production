//! Bounded wave-optical and electromagnetic field volume compilation,
//! cryptographic container verification, and spectral slice rendering
//! for the `electromagnetics` category.
//!
//! Synthesizes 5D electromagnetic lattices into QualiaDB's native Q42EMF64 (`.emf`)
//! binary container format with 3D Hilbert volumetric brick clustering,
//! verifies CRC-32C and BLAKE3-Bao chunk digest trees, and renders 2D spectral
//! preview slices to straight-alpha RGBA8.

use super::*;
use qualia_core_db::etalon_render::spectral::colour_kernel::cpu_batch_emf_to_display_gamut_mapped;
use qualia_core_db::q42::emf_container::{
    emf_brick_voxel_linear_index, EmfCellRecord64, EmfContainerReader, EmfContainerWriter,
    EMF_BRICK_CELLS, SPEED_OF_LIGHT,
};
use qualia_core_db::q42::m64_matter::brick3d::encode_hilbert3d_u64;
use std::collections::BTreeMap;
use std::f64::consts::PI;

/// A wave/dipole emitter in 3D space with polarization and coherence parameters.
#[derive(Clone, Debug)]
struct WaveEmitter {
    position: [f64; 3],
    e_amplitude: [f64; 3],
    wavelength_nm: f64,
    phase: f64,
    stokes: [f64; 3],
}

impl AssetHost<'_> {
    /// Categorical operations dispatch for the `electromagnetics` category.
    pub(super) fn electromagnetics_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.electromagnetics.compile_emf"
            | "electromagnetics.compile_emf"
            | "compile_emf" => self.compile_emf(a),

            "hypermedia.electromagnetics.verify_emf"
            | "electromagnetics.verify_emf"
            | "verify_emf" => self.verify_emf(a),

            "hypermedia.electromagnetics.render_spectral_slice"
            | "electromagnetics.render_spectral_slice"
            | "render_spectral_slice" => self.render_spectral_slice(a),

            _ => Err(format!("Unknown electromagnetics operation: {id}")),
        }
    }

    /// Synthesizes a 3D wave-optical & electromagnetic field volume and seals it
    /// into QualiaDB's native `Q42EMF64` (`.emf`) binary container.
    fn compile_emf(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["origin", "voxel_size", "grid_dims", "epoch_s", "sources"])?;

        let origin = match a.get("origin") {
            Some(v) => {
                let o = vector::<3>(v)?;
                [o[0] as f32, o[1] as f32, o[2] as f32]
            }
            None => [0.0f32, 0.0f32, 0.0f32],
        };

        let voxel_size = number(a, "voxel_size", 0.5)? as f32;
        if !(0.001..=1000.0).contains(&voxel_size) {
            return Err("voxel_size outside 0.001..=1000.0".into());
        }

        let grid_dims = match a.get("grid_dims") {
            Some(v) => {
                let l = list(v)?;
                if l.len() != 3 {
                    return Err("grid_dims must be a list of 3 integers".into());
                }
                let bx = integer(&l[0], "grid_dims[0]")?;
                let by = integer(&l[1], "grid_dims[1]")?;
                let bz = integer(&l[2], "grid_dims[2]")?;
                if !(1..=4).contains(&bx) || !(1..=4).contains(&by) || !(1..=4).contains(&bz) {
                    return Err("grid_dims components must be in 1..=4".into());
                }
                [bx as u32, by as u32, bz as u32]
            }
            None => [1u32, 1u32, 1u32],
        };

        let epoch_s = number(a, "epoch_s", 1_700_000_000.0)?;

        let sources_val = required(a, "sources")?;
        let sources_list = list(sources_val)?;
        if sources_list.is_empty() || sources_list.len() > 16 {
            return Err("sources must contain 1..=16 emitters".into());
        }

        let mut emitters = Vec::with_capacity(sources_list.len());
        for s_val in sources_list {
            let s_rec = record(s_val)?;
            fields(
                s_rec,
                &["position", "e_amplitude", "wavelength_nm", "phase", "stokes"],
            )?;
            let pos = vector::<3>(required(s_rec, "position")?)?;
            let e_amp = vector::<3>(required(s_rec, "e_amplitude")?)?;
            let wavelength_nm = number(s_rec, "wavelength_nm", 550.0)?;
            if !(1.0..=1e9).contains(&wavelength_nm) {
                return Err("wavelength_nm outside 1.0..=1e9".into());
            }
            let phase = number(s_rec, "phase", 0.0)?;
            let stokes = match s_rec.get("stokes") {
                Some(v) => vector::<3>(v)?,
                None => [0.0, 0.0, 0.0],
            };
            emitters.push(WaveEmitter {
                position: pos,
                e_amplitude: e_amp,
                wavelength_nm,
                phase,
                stokes,
            });
        }

        let total_voxels =
            (grid_dims[0] as usize) * (grid_dims[1] as usize) * (grid_dims[2] as usize) * EMF_BRICK_CELLS;
        let work = total_voxels * emitters.len();
        let scratch = total_voxels * 64;
        self.charge(work, scratch)?;

        let mut writer = EmfContainerWriter::new(origin, voxel_size, grid_dims, epoch_s);

        let c_light = SPEED_OF_LIGHT as f64;
        let order = 2u32;
        let num_bricks = (grid_dims[0] * grid_dims[1] * grid_dims[2]) as usize;
        let mut bricks = Vec::with_capacity(num_bricks);

        for bz in 0..grid_dims[2] {
            for by in 0..grid_dims[1] {
                for bx in 0..grid_dims[0] {
                    let hilbert_key = encode_hilbert3d_u64(bx, by, bz, order);
                    let mut brick_cells = [EmfCellRecord64::default(); EMF_BRICK_CELLS];

                    for lz in 0..8u8 {
                        for ly in 0..8u8 {
                            for lx in 0..8u8 {
                                let gx = (bx * 8 + lx as u32) as f64;
                                let gy = (by * 8 + ly as u32) as f64;
                                let gz = (bz * 8 + lz as u32) as f64;

                                let px = origin[0] as f64 + gx * (voxel_size as f64);
                                let py = origin[1] as f64 + gy * (voxel_size as f64);
                                let pz = origin[2] as f64 + gz * (voxel_size as f64);

                                let mut total_e = [0.0f64; 3];
                                let mut total_b = [0.0f64; 3];
                                let mut sum_weight = 0.0f64;
                                let mut sum_wavelength = 0.0f64;
                                let mut sum_q = 0.0f64;
                                let mut sum_u = 0.0f64;
                                let mut sum_v = 0.0f64;
                                let mut phasor_x = 0.0f64;
                                let mut phasor_y = 0.0f64;

                                for emitter in &emitters {
                                    let dx = px - emitter.position[0];
                                    let dy = py - emitter.position[1];
                                    let dz = pz - emitter.position[2];
                                    let r = (dx * dx + dy * dy + dz * dz).sqrt();

                                    let atten = 1.0 / (1.0 + r * r);

                                    let lambda_m = emitter.wavelength_nm * 1e-9;
                                    let k = 2.0 * PI / lambda_m;

                                    let wave_phase = k * r + emitter.phase;
                                    let cos_phase = wave_phase.cos();
                                    let sin_phase = wave_phase.sin();

                                    let e_contrib = [
                                        emitter.e_amplitude[0] * atten * cos_phase,
                                        emitter.e_amplitude[1] * atten * cos_phase,
                                        emitter.e_amplitude[2] * atten * cos_phase,
                                    ];

                                    total_e[0] += e_contrib[0];
                                    total_e[1] += e_contrib[1];
                                    total_e[2] += e_contrib[2];

                                    let k_hat = if r > 1e-9 {
                                        [dx / r, dy / r, dz / r]
                                    } else {
                                        [0.0, 0.0, 1.0]
                                    };

                                    let k_cross_e = [
                                        k_hat[1] * e_contrib[2] - k_hat[2] * e_contrib[1],
                                        k_hat[2] * e_contrib[0] - k_hat[0] * e_contrib[2],
                                        k_hat[0] * e_contrib[1] - k_hat[1] * e_contrib[0],
                                    ];

                                    total_b[0] += k_cross_e[0] / c_light;
                                    total_b[1] += k_cross_e[1] / c_light;
                                    total_b[2] += k_cross_e[2] / c_light;

                                    let amp_mag = (emitter.e_amplitude[0] * emitter.e_amplitude[0]
                                        + emitter.e_amplitude[1] * emitter.e_amplitude[1]
                                        + emitter.e_amplitude[2] * emitter.e_amplitude[2])
                                        .sqrt();
                                    let weight = (amp_mag * atten).max(1e-12);

                                    sum_weight += weight;
                                    sum_wavelength += weight * emitter.wavelength_nm;
                                    sum_q += weight * emitter.stokes[0];
                                    sum_u += weight * emitter.stokes[1];
                                    sum_v += weight * emitter.stokes[2];

                                    phasor_x += weight * cos_phase;
                                    phasor_y += weight * sin_phase;
                                }

                                let (eff_wavelength, eff_q, eff_u, eff_v, eff_phase) =
                                    if sum_weight > 0.0 {
                                        (
                                            (sum_wavelength / sum_weight) as f32,
                                            (sum_q / sum_weight) as f32,
                                            (sum_u / sum_weight) as f32,
                                            (sum_v / sum_weight) as f32,
                                            phasor_y.atan2(phasor_x) as f32,
                                        )
                                    } else {
                                        (550.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32)
                                    };

                                let mut cell = EmfCellRecord64 {
                                    e_field: [total_e[0] as f32, total_e[1] as f32, total_e[2] as f32],
                                    b_field: [total_b[0] as f32, total_b[1] as f32, total_b[2] as f32],
                                    poynting_flux: 0.0,
                                    wavelength_nm: eff_wavelength,
                                    refractive_n: 1.0,
                                    extinction_k: 0.0,
                                    coherence_phase: eff_phase,
                                    stokes_q: eff_q,
                                    stokes_u: eff_u,
                                    stokes_v: eff_v,
                                    spectral_sigma: 0.0,
                                    confidence_alpha: 1.0,
                                };
                                cell.poynting_flux = cell.poynting_flux_magnitude();

                                let voxel_idx = emf_brick_voxel_linear_index(lx, ly, lz);
                                brick_cells[voxel_idx] = cell;
                            }
                        }
                    }

                    bricks.push((hilbert_key, brick_cells));
                }
            }
        }

        bricks.sort_by_key(|(h, _)| *h);
        for (h, cells) in bricks {
            writer
                .add_brick_chunk(h, &cells, 0, 1, 0)
                .map_err(|e| format!("Failed to add brick chunk: {e}"))?;
        }

        let bytes = writer.to_bytes();
        self.encode("emf", bytes)
    }

    /// Parses and cryptographically verifies the Q42EMF64 container,
    /// aggregating energy density and peak Poynting flux.
    fn verify_emf(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["emf"])?;

        let emf_val = required(a, "emf")?;
        let bytes = self.resolve_emf_bytes(emf_val)?;

        self.charge(bytes.len() / 64, bytes.len())?;

        let reader = EmfContainerReader::parse(&bytes)
            .map_err(|e| format!("Invalid EMF container: {e}"))?;

        let chunk_count = reader.chunk_count();
        let mut total_energy_density = 0.0f64;
        let mut max_poynting_flux = 0.0f64;

        for chunk_idx in 0..chunk_count {
            let entry = reader
                .chunk_entry(chunk_idx)
                .ok_or_else(|| format!("Missing chunk directory entry at index {chunk_idx}"))?;
            let cell_count = entry.cell_count as usize;
            for cell_idx in 0..cell_count {
                let cell = reader
                    .cell(chunk_idx, cell_idx)
                    .ok_or_else(|| format!("Missing cell {cell_idx} in chunk {chunk_idx}"))?;

                let u = cell.energy_density() as f64;
                total_energy_density += u;

                let p = cell.poynting_flux_magnitude() as f64;
                if p > max_poynting_flux {
                    max_poynting_flux = p;
                }
            }
        }

        let mut rec = BTreeMap::new();
        rec.insert("valid".into(), Value::Bool(true));
        rec.insert("chunk_count".into(), Value::U64(chunk_count as u64));
        rec.insert(
            "voxel_size_meters".into(),
            Value::F64(reader.header().voxel_size_meters as f64),
        );
        rec.insert(
            "total_energy_density".into(),
            Value::F64(total_energy_density),
        );
        rec.insert("max_poynting_flux".into(), Value::F64(max_poynting_flux));

        Ok(Value::Record(rec))
    }

    /// Renders a 2D spectral preview slice from an EMF-64 volume container to straight-alpha RGBA8.
    fn render_spectral_slice(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["emf", "axis", "slice_index", "width", "height", "exposure"])?;

        let emf_val = required(a, "emf")?;
        let bytes = self.resolve_emf_bytes(emf_val)?;

        let axis = match a.get("axis") {
            Some(v) => text(v)?,
            None => "z",
        };
        if axis != "x" && axis != "y" && axis != "z" {
            return Err("axis must be 'x', 'y', or 'z'".into());
        }

        let slice_index = match a.get("slice_index") {
            Some(v) => integer(v, "slice_index")?,
            None => 0,
        };

        let width = match a.get("width") {
            Some(v) => integer(v, "width")? as u32,
            None => 128u32,
        };
        if !(8..=1024).contains(&width) {
            return Err("width outside 8..=1024".into());
        }

        let height = match a.get("height") {
            Some(v) => integer(v, "height")? as u32,
            None => 128u32,
        };
        if !(8..=1024).contains(&height) {
            return Err("height outside 8..=1024".into());
        }

        let exposure = number(a, "exposure", 1.0)?;
        if !(0.001..=1e6).contains(&exposure) {
            return Err("exposure outside 0.001..=1e6".into());
        }

        let pixel_count = (width as usize) * (height as usize);
        self.charge(pixel_count, pixel_count * 4)?;

        let reader = EmfContainerReader::parse(&bytes)
            .map_err(|e| format!("Invalid EMF container: {e}"))?;

        let dims = reader.header().grid_dims_bricks;
        let vx_max = dims[0] * 8;
        let vy_max = dims[1] * 8;
        let vz_max = dims[2] * 8;

        match axis {
            "z" => {
                if slice_index >= vz_max as usize {
                    return Err(format!(
                        "slice_index {slice_index} out of bounds for z-axis (max {})",
                        vz_max.saturating_sub(1)
                    ));
                }
            }
            "y" => {
                if slice_index >= vy_max as usize {
                    return Err(format!(
                        "slice_index {slice_index} out of bounds for y-axis (max {})",
                        vy_max.saturating_sub(1)
                    ));
                }
            }
            "x" => {
                if slice_index >= vx_max as usize {
                    return Err(format!(
                        "slice_index {slice_index} out of bounds for x-axis (max {})",
                        vx_max.saturating_sub(1)
                    ));
                }
            }
            _ => unreachable!(),
        }

        let mut pixels = Vec::with_capacity(pixel_count);
        let order = 2u32;

        for iy in 0..height {
            for ix in 0..width {
                let u = ix as f64 / width as f64;
                let v = iy as f64 / height as f64;

                let (vx, vy, vz) = match axis {
                    "z" => {
                        let sx = ((u * vx_max as f64).floor() as u32).min(vx_max.saturating_sub(1));
                        let sy = ((v * vy_max as f64).floor() as u32).min(vy_max.saturating_sub(1));
                        (sx, sy, slice_index as u32)
                    }
                    "y" => {
                        let sx = ((u * vx_max as f64).floor() as u32).min(vx_max.saturating_sub(1));
                        let sz = ((v * vz_max as f64).floor() as u32).min(vz_max.saturating_sub(1));
                        (sx, slice_index as u32, sz)
                    }
                    "x" => {
                        let sy = ((u * vy_max as f64).floor() as u32).min(vy_max.saturating_sub(1));
                        let sz = ((v * vz_max as f64).floor() as u32).min(vz_max.saturating_sub(1));
                        (slice_index as u32, sy, sz)
                    }
                    _ => unreachable!(),
                };

                let bx = vx / 8;
                let by = vy / 8;
                let bz = vz / 8;
                let lx = (vx % 8) as u8;
                let ly = (vy % 8) as u8;
                let lz = (vz % 8) as u8;

                let cell = reader
                    .lookup_brick_cell(bx, by, bz, lx, ly, lz, order)
                    .unwrap_or_default();

                let base_rgb = spectral_wavelength_to_rgb(cell.wavelength_nm as f64);
                let u_em = (cell.energy_density() as f64).max(0.0);
                let brightness = (1.0 - (-u_em * exposure).exp()).clamp(0.0, 1.0);

                // Also evaluate Etalon's CIE 1931 SPD -> XYZ -> sRGB gamut-mapped CPU oracle
                let sigma = ((cell.wavelength_nm - 400.0) / 300.0).clamp(0.0, 1.0);
                let mu = cell.spectral_sigma.clamp(0.0, 1.0);
                let mut etalon_rgb = [0u8; 3];
                let _ = cpu_batch_emf_to_display_gamut_mapped(
                    &[brightness as f32 * 4.0, mu, sigma],
                    &mut etalon_rgb,
                );

                let pol = (cell.stokes_q as f64)
                    .hypot(cell.stokes_u as f64)
                    .hypot(cell.stokes_v as f64)
                    .min(1.0);
                let phase_cos = (cell.coherence_phase as f64).cos();
                let modulation = (1.0 + 0.10 * (pol - 0.5) + 0.05 * phase_cos).clamp(0.0, 2.0);

                let r_bruton = (base_rgb[0] * brightness * modulation).clamp(0.0, 1.0) * 255.0;
                let g_bruton = (base_rgb[1] * brightness * modulation).clamp(0.0, 1.0) * 255.0;
                let b_bruton = (base_rgb[2] * brightness * modulation).clamp(0.0, 1.0) * 255.0;

                let (r, g, b) = if (380.0..=780.0).contains(&(cell.wavelength_nm as f64)) {
                    (
                        (0.65 * r_bruton + 0.35 * (etalon_rgb[0] as f64)).round().clamp(0.0, 255.0) as u8,
                        (0.65 * g_bruton + 0.35 * (etalon_rgb[1] as f64)).round().clamp(0.0, 255.0) as u8,
                        (0.65 * b_bruton + 0.35 * (etalon_rgb[2] as f64)).round().clamp(0.0, 255.0) as u8,
                    )
                } else {
                    (r_bruton.round() as u8, g_bruton.round() as u8, b_bruton.round() as u8)
                };

                pixels.push([r, g, b, 255]);
            }
        }

        self.store_rgba(width, height, pixels)
    }

    /// Resolves container bytes from retained inputs or workspace encoded resources.
    pub(super) fn resolve_emf_bytes(&self, val: &Value) -> Result<Vec<u8>, String> {
        if let Ok(key) = text(val) {
            self.inputs
                .iter()
                .find(|r| r.key == key)
                .map(|r| r.bytes.clone())
                .ok_or_else(|| format!("Missing retained input: {key}"))
        } else if let Ok(res) = self.workspace.encoded(val) {
            if res.kind != "emf" {
                return Err(format!("Expected an 'emf' resource handle, found '{}'", res.kind));
            }
            Ok(res.bytes.clone())
        } else {
            Err("Expected retained resource key or encoded handle for emf".into())
        }
    }
}

/// CIE 1931 / Bruton piecewise spectral-to-sRGB mapping for wavelength in nanometers.
///
/// Converts visible wavelengths (380..=780 nm) to linear RGB factors, with
/// false-color magenta (< 380 nm, UV) and false-color cyan (> 780 nm, IR/RF)
/// for invisible radiation bands.
fn spectral_wavelength_to_rgb(wavelength_nm: f64) -> [f64; 3] {
    if !wavelength_nm.is_finite() || wavelength_nm <= 0.0 {
        return [0.0, 0.0, 0.0];
    }

    if wavelength_nm < 380.0 {
        // UV / RF below visible: false-color magenta
        return [0.95, 0.05, 0.85];
    }
    if wavelength_nm > 780.0 {
        // IR / RF above visible: false-color cyan
        return [0.05, 0.85, 0.95];
    }

    let (r, g, b) = if wavelength_nm < 440.0 {
        let r = -(wavelength_nm - 440.0) / (440.0 - 380.0);
        (r, 0.0, 1.0)
    } else if wavelength_nm < 490.0 {
        let g = (wavelength_nm - 440.0) / (490.0 - 440.0);
        (0.0, g, 1.0)
    } else if wavelength_nm < 510.0 {
        let b = -(wavelength_nm - 510.0) / (510.0 - 490.0);
        (0.0, 1.0, b)
    } else if wavelength_nm < 580.0 {
        let r = (wavelength_nm - 510.0) / (580.0 - 510.0);
        (r, 1.0, 0.0)
    } else if wavelength_nm < 645.0 {
        let g = -(wavelength_nm - 645.0) / (645.0 - 580.0);
        (1.0, g, 0.0)
    } else {
        (1.0, 0.0, 0.0)
    };

    let factor = if wavelength_nm < 420.0 {
        0.3 + 0.7 * (wavelength_nm - 380.0) / (420.0 - 380.0)
    } else if wavelength_nm > 700.0 {
        0.3 + 0.7 * (780.0 - wavelength_nm) / (780.0 - 700.0)
    } else {
        1.0
    };

    [r * factor, g * factor, b * factor]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    fn create_test_host<'a>(inputs: &'a [Resource], cancel: &'a AtomicBool, limits: &'a ToolLimits) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs,
            limits,
            cancel,
            work: 0,
        }
    }

    #[test]
    fn test_bruton_spectral_mapping_visible_and_false_color() {
        let uv = spectral_wavelength_to_rgb(250.0);
        assert!(uv[0] > 0.8 && uv[2] > 0.8 && uv[1] < 0.2, "UV must be false-color magenta");

        let ir = spectral_wavelength_to_rgb(950.0);
        assert!(ir[1] > 0.8 && ir[2] > 0.8 && ir[0] < 0.2, "IR must be false-color cyan");

        let green = spectral_wavelength_to_rgb(550.0);
        assert!(green[1] > 0.9, "550nm must have high green component");

        let blue = spectral_wavelength_to_rgb(450.0);
        assert!(blue[2] > 0.9, "450nm must have high blue component");

        let red = spectral_wavelength_to_rgb(660.0);
        assert!(red[0] > 0.9, "660nm must have high red component");

        let zero = spectral_wavelength_to_rgb(0.0);
        assert_eq!(zero, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_compile_verify_and_render_roundtrip() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let inputs = vec![];
        let mut host = create_test_host(&inputs, &cancel, &limits);

        let mut compile_args = BTreeMap::new();
        compile_args.insert(
            "origin".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        compile_args.insert("voxel_size".into(), Value::F64(0.25));
        compile_args.insert(
            "grid_dims".into(),
            Value::List(vec![Value::U64(1), Value::U64(1), Value::U64(1)]),
        );
        compile_args.insert("epoch_s".into(), Value::F64(1_700_000_000.0));

        let mut src = BTreeMap::new();
        src.insert(
            "position".into(),
            Value::List(vec![Value::F64(0.5), Value::F64(0.5), Value::F64(0.5)]),
        );
        src.insert(
            "e_amplitude".into(),
            Value::List(vec![Value::F64(150.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        src.insert("wavelength_nm".into(), Value::F64(532.0));
        src.insert("phase".into(), Value::F64(0.0));
        src.insert(
            "stokes".into(),
            Value::List(vec![Value::F64(1.0), Value::F64(0.0), Value::F64(0.0)]),
        );

        compile_args.insert("sources".into(), Value::List(vec![Value::Record(src)]));

        let emf_handle = host
            .electromagnetics_op("hypermedia.electromagnetics.compile_emf", &compile_args)
            .expect("compile_emf succeeds");

        if let Value::Record(ref r) = emf_handle {
            assert_eq!(r.get("type"), Some(&Value::String("emf".into())));
        } else {
            panic!("Expected record handle from compile_emf");
        }

        let mut verify_args = BTreeMap::new();
        verify_args.insert("emf".into(), emf_handle.clone());

        let verify_res = host
            .electromagnetics_op("hypermedia.electromagnetics.verify_emf", &verify_args)
            .expect("verify_emf succeeds");

        if let Value::Record(ref r) = verify_res {
            assert_eq!(r.get("valid"), Some(&Value::Bool(true)));
            assert_eq!(r.get("chunk_count"), Some(&Value::U64(1)));
            assert_eq!(r.get("voxel_size_meters"), Some(&Value::F64(0.25)));

            let total_u = match r.get("total_energy_density") {
                Some(Value::F64(v)) => *v,
                _ => panic!("Expected F64 total_energy_density"),
            };
            assert!(total_u > 0.0, "Total energy density must be positive");

            let max_s = match r.get("max_poynting_flux") {
                Some(Value::F64(v)) => *v,
                _ => panic!("Expected F64 max_poynting_flux"),
            };
            assert!(max_s > 0.0, "Max Poynting flux must be positive");
        } else {
            panic!("Expected record from verify_emf");
        }

        for axis in ["z", "y", "x"] {
            let mut render_args = BTreeMap::new();
            render_args.insert("emf".into(), emf_handle.clone());
            render_args.insert("axis".into(), Value::String(axis.into()));
            render_args.insert("slice_index".into(), Value::U64(0));
            render_args.insert("width".into(), Value::U64(32));
            render_args.insert("height".into(), Value::U64(32));
            render_args.insert("exposure".into(), Value::F64(1.0));

            let rgba_handle = host
                .electromagnetics_op(
                    "hypermedia.electromagnetics.render_spectral_slice",
                    &render_args,
                )
                .expect("render_spectral_slice succeeds");

            let rgba = host.rgba(&rgba_handle).expect("valid rgba handle");
            assert_eq!(rgba.width, 32);
            assert_eq!(rgba.height, 32);
            assert_eq!(rgba.pixels.len(), 1024);
        }
    }
}
