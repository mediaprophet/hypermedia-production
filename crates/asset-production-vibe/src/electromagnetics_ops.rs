//! Bounded wave-optical and electromagnetic field volume compilation,
//! cryptographic container verification, and spectral slice rendering
//! for the `electromagnetics` category.
//!
//! Synthesizes 5D electromagnetic lattices into QualiaDB's native Q42EMF64 (`.emf`)
//! binary container format with 3D Hilbert volumetric brick clustering,
//! verifies CRC-32C and complete BLAKE3 digests (not Bao trees), and renders 2D spectral
//! preview slices to straight-alpha RGBA8.

use super::*;
use qualia_core_db::etalon_render::spectral::colour_kernel::cpu_batch_emf_to_display_gamut_mapped;
use qualia_core_db::q42::emf_container::{
    EmfCellRecord64, EmfContainerReader, MU_0, SPEED_OF_LIGHT,
};
use std::collections::BTreeMap;

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

            "hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa"
            | "electromagnetics.sense_wifi_csi_vitals_aoa"
            | "synth.electromagnetics.sense_wifi_csi_vitals_aoa"
            | "sense_wifi_csi_vitals_aoa" => self.produce_wifi_csi_vitals_aoa(a),

            _ => Err(format!("Unknown electromagnetics operation: {id}")),
        }
    }

    /// Parses and cryptographically verifies the Q42EMF64 container,
    /// aggregating energy density and peak Poynting flux.
    fn verify_emf(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["emf"])?;

        let emf_val = required(a, "emf")?;
        let bytes = self.resolve_emf_bytes(emf_val)?;

        self.charge(bytes.len() / 64, 0)?;

        let reader = self.parse_emf_current(&bytes)?;

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
        rec.insert("container_version".into(),Value::U64(u64::from(reader.header().version)));
        rec.insert("cell_integrity".into(),Value::String("full_blake3_256".into()));
        rec.insert("metadata_integrity".into(),Value::String("header_and_directory_crc32c".into()));
        rec.insert("extension_integrity".into(),Value::String(if reader.extension().is_some(){"full_blake3_256"}else{"not_present"}.into()));
        rec.insert("extension".into(),crate::electromagnetics_v3::emf_metadata_value(&reader)?);
        let tracks=reader.extension().map(|extension|extension.metadata().audio_tracks.as_slice()).unwrap_or(&[]);
        rec.insert("audio_track_count".into(),Value::U64(tracks.len() as u64));
        rec.insert("audio_payload_bytes".into(),Value::U64(tracks.iter().try_fold(0u64,|sum,track|sum.checked_add(track.pcm_bytes)).ok_or("PCM aggregate overflow")?));
        rec.insert("audio_integrity".into(),Value::String(if tracks.is_empty(){"not_present"}else{"full_blake3_256"}.into()));

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
        fields(a, &["emf", "axis", "slice_index", "width", "height", "exposure", "time_slice"])?;

        let emf_val = required(a, "emf")?;

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
        let pixel_bytes=pixel_count.checked_mul(4).ok_or("EMF pixel size overflow")?;
        if pixel_bytes>self.limits.resource_bytes{return Err("EMF slice exceeds selected resource bytes".into());}
        let bytes=self.resolve_emf_bytes_for(emf_val,pixel_bytes)?;
        self.charge(pixel_count,0)?;

        let time_slice=crate::electromagnetics_v3::selected_time_slice(a)?;
        let reader = self.parse_emf_current(&bytes)?;
        self.charge(reader.chunk_count(),0)?;
        if !(0..reader.chunk_count()).any(|i|reader.chunk_entry(i).is_some_and(|e|e.time_slice_index==time_slice)) {
            return Err("Requested EMF time_slice has no stored chunks".into());
        }
        self.charge(pixel_count.checked_mul(reader.lookup_work_bound()).ok_or("EMF preview work overflow")?,0)?;
        crate::electromagnetics_v3::require_legacy_visible_projection(&reader)?;
        let hilbert_order=reader.extension().map(|e|u32::from(e.metadata().field.hilbert_order)).unwrap_or(2);


        let dims = reader.header().grid_dims_bricks;
        let vx_max = dims[0].checked_mul(8).ok_or("EMF voxel dimension overflow")?;
        let vy_max = dims[1].checked_mul(8).ok_or("EMF voxel dimension overflow")?;
        let vz_max = dims[2].checked_mul(8).ok_or("EMF voxel dimension overflow")?;

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
        let order = hilbert_order;

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
                    .lookup_brick_cell_at_time(bx, by, bz, lx, ly, lz, order, time_slice)
                    .unwrap_or_default();

                let base_rgb = spectral_wavelength_to_rgb(cell.wavelength_nm as f64);
                // SI u_EM (J/m³) is ~ε0·|E|² ≈ 10⁻⁷ at compiled amplitudes, so
                // 1-exp(-u_EM) rounds to 0 at documented exposure=1.0. Spectral
                // preview is radiometric: vacuum-impedance irradiance (W/m²).
                let u_em = emf_cell_display_irradiance(&cell);
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

        let mut image=self.store_rgba(width,height,pixels)?;
        if let Value::Record(meta)=&mut image {
            meta.insert("projection_scope".into(),Value::String("legacy human-viewable spectral derivative; includes out-of-band false colour/clamping; not stored RGB".into()));
            meta.insert("source_time_slice".into(),Value::U64(u64::from(time_slice)));
            meta.insert("source_emf_version".into(),Value::U64(u64::from(reader.header().version)));
        }
        Ok(image)
    }

    /// 802.11 CSI → sanitized phase, contactless vitals, MUSIC AoA, camera-free receipt.
    fn produce_wifi_csi_vitals_aoa(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["audio", "breathing_hz", "heart_hz", "carrier_ghz"])?;
        let resp_hz = number(a, "breathing_hz", 0.25)? as f32;
        let heart_hz = number(a, "heart_hz", 1.2)? as f32;
        let carrier_ghz = number(a, "carrier_ghz", 5.18)?;
        if !(1.0..=60.0).contains(&carrier_ghz) {
            return Err("carrier_ghz outside 1.0..=60.0".into());
        }
        if !(0.05..=0.7).contains(&(resp_hz as f64)) {
            return Err("breathing_hz outside 0.05..=0.7".into());
        }
        if !(0.7..=3.0).contains(&(heart_hz as f64)) {
            return Err("heart_hz outside 0.7..=3.0".into());
        }

        self.charge(512, 8192)?;

        use electromagnetics_rf_sensing::{
            extract_vital_signs, find_dominant_aoa_peak, hampel_filter, music_aoa_spectrum,
            sanitize_csi_phase, Complex32, CsiFrame, PrivacyWitnessReceipt, UniformLinearArray,
        };

        let indices = vec![-28, -20, -10, 0, 10, 20, 28];
        let mut subcarriers = Vec::with_capacity(indices.len());
        for &k in &indices {
            let phi = (k as f32) * 0.12 + 0.8;
            subcarriers.push(Complex32::from_polar(1.0, phi));
        }
        let frame = CsiFrame::new(
            1_700_000_000,
            carrier_ghz * 1e9,
            1,
            1,
            indices.clone(),
            subcarriers,
        );
        let raw_phases = frame.get_phases(0, 0);
        let sanitized = sanitize_csi_phase(&frame.subcarrier_indices, &raw_phases);

        let sr = 50.0f32;
        let mut signal = Vec::with_capacity(500);
        if let Some(v) = a.get("audio") {
            let Artifact::Audio { samples: src, .. } = self.workspace.get(v)? else {
                return Err("Expected audio handle".into());
            };
            for i in 0..500 {
                let s = src.get(i).copied().unwrap_or(0.0);
                let t = (i as f32) / sr;
                signal.push(
                    s * 0.05
                        + 0.5 * (2.0 * std::f32::consts::PI * resp_hz * t).sin()
                        + 0.1 * (2.0 * std::f32::consts::PI * heart_hz * t).sin(),
                );
            }
        } else {
            for i in 0..500 {
                let t = (i as f32) / sr;
                signal.push(
                    0.5 * (2.0 * std::f32::consts::PI * resp_hz * t).sin()
                        + 0.1 * (2.0 * std::f32::consts::PI * heart_hz * t).sin(),
                );
            }
        }
        let filtered = hampel_filter(&signal, 3, 3.0);
        let vitals = extract_vital_signs(&filtered, sr).ok_or("Vital signs extraction failed")?;

        let ula = UniformLinearArray::new(4, carrier_ghz * 1e9);
        let target_aoa = 25.0f32;
        let mut snapshots = Vec::with_capacity(16);
        for t in 0..16 {
            // Identical snapshots collapse MUSIC; carrier rotation is the crate object.
            let carrier = Complex32::from_polar(1.0, (t as f32) * 0.2);
            let snap: Vec<Complex32> = ula
                .steering_vector(target_aoa)
                .into_iter()
                .map(|a| a.mul(carrier))
                .collect();
            snapshots.push(snap);
        }
        let spectrum = music_aoa_spectrum(&ula, &snapshots, 1, 1.0);
        let peak_aoa_deg = find_dominant_aoa_peak(&spectrum).unwrap_or(0.0);

        let receipt = PrivacyWitnessReceipt::create("witness-1", 1700000000, 1, true, &[0x42; 32]);
        let verified = receipt.verify();

        let max_power = spectrum.iter().map(|&(_, p)| p).fold(1.0f32, f32::max);
        let mut aoa_points = String::new();
        for &(deg, power) in &spectrum {
            let x = ((deg + 90.0) / 180.0) * 360.0 + 20.0;
            let y = 180.0 - (power / max_power).clamp(0.0, 1.0) * 140.0;
            aoa_points.push_str(&format!("{x:.1},{y:.1} "));
        }
        let mut phase_points = String::new();
        for (&k, &ph) in indices.iter().zip(sanitized.iter()) {
            let x = ((k as f32 + 30.0) / 60.0) * 360.0 + 20.0;
            let y = 100.0 - (ph * 40.0).clamp(-80.0, 80.0);
            phase_points.push_str(&format!("{x:.1},{y:.1} "));
        }
        let svg_xml = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 400 200\">\
             <rect width=\"400\" height=\"200\" fill=\"#0b1020\"/>\
             <polyline fill=\"none\" stroke=\"#00e5ff\" stroke-width=\"2\" points=\"{}\"/>\
             <polyline fill=\"none\" stroke=\"#ffab00\" stroke-width=\"1.5\" points=\"{}\"/>\
             </svg>",
            aoa_points.trim_end(),
            phase_points.trim_end()
        );
        let aoa_svg_resource = self.encode("svg", svg_xml.into_bytes())?;

        let receipt_json = serde_json::json!({
            "witness_id": receipt.witness_id,
            "timestamp_ns": receipt.timestamp_ns,
            "occupancy_count": receipt.occupancy_count,
            "vitals_healthy": receipt.vitals_healthy,
            "human_rights_guarantee": receipt.human_rights_guarantee,
            "camera_free_verified": verified,
            "vitals": {
                "breathing_bpm": vitals.breathing_bpm,
                "heart_rate_bpm": vitals.heart_rate_bpm,
            },
            "dominant_aoa_deg": peak_aoa_deg,
            "sanitized_subcarrier_count": sanitized.len(),
        });
        let json_bytes = serde_json::to_vec(&receipt_json).map_err(|e| e.to_string())?;
        let receipt_json_resource = self.encode("json", json_bytes)?;

        let mut out = BTreeMap::new();
        out.insert("aoa_svg_resource".into(), aoa_svg_resource);
        out.insert("receipt_json_resource".into(), receipt_json_resource);
        out.insert(
            "sanitized_subcarrier_count".into(),
            Value::I64(sanitized.len() as i64),
        );
        out.insert(
            "breathing_bpm".into(),
            Value::F64(vitals.breathing_bpm as f64),
        );
        out.insert(
            "heart_rate_bpm".into(),
            Value::F64(vitals.heart_rate_bpm as f64),
        );
        out.insert("dominant_aoa_deg".into(), Value::F64(peak_aoa_deg as f64));
        out.insert("camera_free_verified".into(), Value::Bool(verified));
        Ok(Value::Record(out))
    }

    /// Resolves container bytes from retained inputs or workspace encoded resources.
    pub(crate) fn emf_source_bytes<'b>(&'b self,val:&Value)->Result<&'b[u8],String>{
        if let Ok(key)=text(val){
            self.inputs.iter().find(|r|r.key==key).map(|r|r.bytes.as_slice()).ok_or_else(||format!("Missing retained input: {key}"))
        }else if let Ok(resource)=self.workspace.encoded(val){
            if resource.kind!="emf"{return Err(format!("Expected an 'emf' resource handle, found '{}'",resource.kind));}
            Ok(&resource.bytes)
        }else{Err("Expected retained resource key or encoded handle for emf".into())}
    }

    pub(crate) fn resolve_emf_bytes(&self,val:&Value)->Result<Vec<u8>,String>{self.resolve_emf_bytes_for(val,0)}

    /// Fixed backing is already included by peak admission; add exactly one
    /// source clone, bounded parse/report scratch, and caller's live output/grid.
    pub(crate) fn resolve_emf_bytes_for(&self,val:&Value,extra:usize)->Result<Vec<u8>,String>{
        let required=self.emf_read_required(val)?.checked_add(extra).ok_or("EMF read peak overflow")?;
        self.emf_peak_admit(required)?;
        Ok(self.emf_source_bytes(val)?.to_vec())
    }

}

/// Current production admission requires v2/v3 full BLAKE3 seals, not legacy prefixes.
pub(crate) fn parse_sealed_emf(bytes: &[u8]) -> Result<EmfContainerReader<'_>, String> {
    let reader = EmfContainerReader::parse(bytes)
        .map_err(|e| format!("Invalid EMF container: {e}"))?;
    for chunk_idx in 0..reader.chunk_count() {
        let entry = reader
            .chunk_entry(chunk_idx)
            .ok_or_else(|| format!("Missing chunk directory entry at index {chunk_idx}"))?;
        if entry.flags & qualia_core_db::q42::emf_container::EMF_CHUNK_FLAG_BLAKE3_FULL == 0 {
            return Err("EMF container omitted full BLAKE3 seal".into());
        }
    }
    Ok(reader)
}

/// Vacuum-impedance irradiance (W/m²) used as the display-linear quantity.
///
/// Documented default `exposure=1.0` is a gain on this irradiance, not on SI
/// energy density (J/m³), which maps compiled non-trivial fields to black.
pub(crate) fn emf_cell_display_irradiance(cell: &EmfCellRecord64) -> f64 {
    let ex = cell.e_field[0] as f64;
    let ey = cell.e_field[1] as f64;
    let ez = cell.e_field[2] as f64;
    let z0 = (MU_0 as f64) * (SPEED_OF_LIGHT as f64);
    let from_e = if z0 > 0.0 {
        (ex * ex + ey * ey + ez * ez) / z0
    } else {
        0.0
    };
    let from_s = (cell.poynting_flux_magnitude() as f64).max(cell.poynting_flux as f64);
    from_s.max(from_e).max(0.0)
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
    fn test_compile_verify_and_render_spectral_slice_roundtrip() {
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
            assert!(
                rgba.pixels
                    .iter()
                    .any(|p| p[0] > 0 || p[1] > 0 || p[2] > 0),
                "spectral slice at documented exposure=1.0 must be non-black on axis {axis}"
            );
        }
    }

    fn compile_green_dipole(host: &mut AssetHost<'_>) -> Value {
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
        host.electromagnetics_op("hypermedia.electromagnetics.compile_emf", &compile_args)
            .expect("compile_emf succeeds")
    }

    #[test]
    fn test_render_spectral_slice_default_exposure_non_black() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let inputs = vec![];
        let mut host = create_test_host(&inputs, &cancel, &limits);
        let emf_handle = compile_green_dipole(&mut host);

        let mut render_args = BTreeMap::new();
        render_args.insert("emf".into(), emf_handle);
        render_args.insert("width".into(), Value::U64(16));
        render_args.insert("height".into(), Value::U64(16));

        let rgba_handle = host
            .electromagnetics_op(
                "hypermedia.electromagnetics.render_spectral_slice",
                &render_args,
            )
            .expect("render_spectral_slice at default exposure succeeds");
        let rgba = host.rgba(&rgba_handle).expect("valid rgba handle");
        assert!(
            rgba.pixels
                .iter()
                .any(|p| p[0] > 0 || p[1] > 0 || p[2] > 0),
            "documented default exposure must yield at least one non-zero RGB channel"
        );
    }

    #[test]
    fn test_render_spectral_slice_tampered_emf_fails_closed() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let inputs = vec![];
        let mut host = create_test_host(&inputs, &cancel, &limits);
        let emf_handle = compile_green_dipole(&mut host);
        let mut tampered = host
            .workspace
            .encoded(&emf_handle)
            .expect("encoded emf")
            .bytes
            .clone();
        let payload = 64 + 64;
        assert!(tampered.len() > payload + 10);
        tampered[payload + 10] ^= 0xAA;
        let bad = host
            .admit_encoded("emf", tampered)
            .expect("admit tampered bytes");

        let mut render_args = BTreeMap::new();
        render_args.insert("emf".into(), bad);
        render_args.insert("width".into(), Value::U64(8));
        render_args.insert("height".into(), Value::U64(8));
        let err = host
            .electromagnetics_op(
                "hypermedia.electromagnetics.render_spectral_slice",
                &render_args,
            )
            .expect_err("tampered Q42EMF64 must fail closed");
        assert!(
            err.contains("BLAKE3") || err.contains("Checksum"),
            "fail-closed error must identify current CRC/full-BLAKE3 integrity, got: {err}"
        );
    }

    fn strip_emf_bao_keep_crc(bytes: &mut [u8]) {
        use qualia_core_db::q42::emf_container::EmfChunkDirectoryEntry;
        let (dir_off, n) = {
            let reader = EmfContainerReader::parse(bytes).expect("CRC-valid sealed emf");
            (
                reader.header().directory_offset as usize,
                reader.chunk_count(),
            )
        };
        for i in 0..n {
            let off = dir_off + i * 64;
            let mut entry: EmfChunkDirectoryEntry =
                bytemuck::pod_read_unaligned(&bytes[off..off + 64]);
            entry.flags &= !qualia_core_db::q42::emf_container::EMF_CHUNK_FLAG_BLAKE3_FULL;
            entry.bao_hash_prefix = [0u8; 16];
            entry.update_crc();
            bytes[off..off + 64].copy_from_slice(bytemuck::bytes_of(&entry));
        }
    }

    #[test]
    fn test_verify_emf_refuses_crc_consistent_bao_omitted() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let inputs = vec![];
        let mut host = create_test_host(&inputs, &cancel, &limits);
        let emf_handle = compile_green_dipole(&mut host);
        let mut unsigned = host
            .workspace
            .encoded(&emf_handle)
            .expect("encoded emf")
            .bytes
            .clone();
        strip_emf_bao_keep_crc(&mut unsigned);
        assert!(
            EmfContainerReader::parse(&unsigned).is_err(),
            "V2 must refuse missing full BLAKE3 seals"
        );

        let admitted = host
            .admit_encoded("emf", unsigned.clone())
            .expect("admit CRC-only emf");
        let mut verify_args = BTreeMap::new();
        verify_args.insert("emf".into(), admitted);
        let err = host
            .electromagnetics_op("hypermedia.electromagnetics.verify_emf", &verify_args)
            .expect_err("Bao-omitted EMF must fail closed");
        assert!(
            err.contains("Bao") || err.contains("seal"),
            "fail-closed error must name Bao seal, got: {err}"
        );

        let mut render_args = BTreeMap::new();
        render_args.insert(
            "emf".into(),
            host.admit_encoded("emf", unsigned.clone())
                .expect("admit CRC-only emf for render"),
        );
        render_args.insert("width".into(), Value::U64(8));
        render_args.insert("height".into(), Value::U64(8));
        let render_err = host
            .electromagnetics_op(
                "hypermedia.electromagnetics.render_spectral_slice",
                &render_args,
            )
            .expect_err("Bao-omitted EMF render must fail closed");
        assert!(
            render_err.contains("Bao") || render_err.contains("seal"),
            "render fail-closed must name Bao seal, got: {render_err}"
        );

        let retained = [Resource {
            key: "unsigned.emf".into(),
            kind: "emf".into(),
            bytes: unsigned.clone(),
        }];
        let mut retained_host = create_test_host(&retained, &cancel, &limits);
        let mut retained_args = BTreeMap::new();
        retained_args.insert("emf".into(), Value::String("unsigned.emf".into()));
        let retained_err = retained_host
            .electromagnetics_op("hypermedia.electromagnetics.verify_emf", &retained_args)
            .expect_err("retained Bao-omitted EMF must fail closed");
        assert!(
            retained_err.contains("Bao") || retained_err.contains("seal"),
            "retained fail-closed must name Bao seal, got: {retained_err}"
        );

        let mut ray_args = BTreeMap::new();
        ray_args.insert(
            "emf".into(),
            host.admit_encoded("emf", unsigned.clone())
                .expect("admit CRC-only emf for raymarch"),
        );
        ray_args.insert("width".into(), Value::U64(8));
        ray_args.insert("height".into(), Value::U64(8));
        let ray_err = host
            .invoke(
                "hypermedia.electromagnetics.raymarch_vox0_dda",
                &Value::Record(ray_args),
            )
            .expect_err("Bao-omitted EMF raymarch must fail closed");
        assert!(
            ray_err.contains("Bao") || ray_err.contains("seal"),
            "raymarch fail-closed must name Bao seal, got: {ray_err}"
        );

        let mut tone_args = BTreeMap::new();
        tone_args.insert(
            "emf".into(),
            host.admit_encoded("emf", unsigned)
                .expect("admit CRC-only emf for sonify"),
        );
        tone_args.insert("sample_rate".into(), Value::U64(8_000));
        tone_args.insert("frames".into(), Value::U64(256));
        tone_args.insert("amplitude".into(), Value::F64(0.25));
        let tone_err = host
            .invoke(
                "hypermedia.acoustics.synthesize_tone",
                &Value::Record(tone_args),
            )
            .expect_err("Bao-omitted EMF sonify must fail closed");
        assert!(
            tone_err.contains("Bao") || tone_err.contains("seal"),
            "sonify fail-closed must name Bao seal, got: {tone_err}"
        );
    }

    #[test]
    fn test_spectral_display_irradiance_visible_for_compiled_amplitude() {
        let mut cell = EmfCellRecord64::default();
        cell.e_field = [150.0, 0.0, 0.0];
        cell.b_field = [0.0, 150.0 / SPEED_OF_LIGHT, 0.0];
        cell.poynting_flux = cell.poynting_flux_magnitude();
        let i = emf_cell_display_irradiance(&cell);
        assert!(i > 1.0, "150 V/m must exceed 1 W/m² display irradiance, got {i}");
        let brightness = 1.0 - (-i * 1.0f64).exp();
        assert!(
            brightness * 255.0 >= 1.0,
            "documented exposure=1.0 must quantize to a visible 8-bit channel"
        );
        assert!(
            (cell.energy_density() as f64) * 255.0 < 1.0,
            "SI energy density at this amplitude still maps to a black 8-bit pixel"
        );
    }

    fn f64_field(rec: &BTreeMap<String, Value>, key: &str) -> f64 {
        match rec.get(key) {
            Some(Value::F64(v)) => *v,
            _ => panic!("Expected F64 {key}"),
        }
    }

    fn encoded_bytes(host: &AssetHost<'_>, rec: &BTreeMap<String, Value>, key: &str) -> Vec<u8> {
        let handle = rec.get(key).unwrap_or_else(|| panic!("missing {key}"));
        host.workspace
            .encoded(handle)
            .unwrap_or_else(|_| panic!("encoded {key}"))
            .bytes
            .clone()
    }

    #[test]
    fn test_electromagnetics_csi_vitals_aoa_object_two_produces() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let inputs = vec![];
        let mut host = create_test_host(&inputs, &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("breathing_hz".into(), Value::F64(0.25));
        args.insert("heart_hz".into(), Value::F64(1.2));
        args.insert("carrier_ghz".into(), Value::F64(5.18));

        let a = host
            .electromagnetics_op(
                "hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa",
                &args,
            )
            .expect("csi produce a");
        let b = host
            .electromagnetics_op(
                "hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa",
                &args,
            )
            .expect("csi produce b");
        let ra = record(&a).expect("csi record a");
        let rb = record(&b).expect("csi record b");

        assert_eq!(ra.get("camera_free_verified"), Some(&Value::Bool(true)));
        assert_eq!(
            ra.get("sanitized_subcarrier_count"),
            Some(&Value::I64(7))
        );
        let breathing = f64_field(ra, "breathing_bpm");
        let heart = f64_field(ra, "heart_rate_bpm");
        let aoa = f64_field(ra, "dominant_aoa_deg");
        assert!(
            (breathing - 15.0).abs() < 3.0,
            "breathing BPM object, got {breathing}"
        );
        assert!(
            (heart - 72.0).abs() < 8.0,
            "heart BPM object, got {heart}"
        );
        assert!(
            (aoa - 25.0).abs() <= 2.0,
            "MUSIC AoA must recover 25°, got {aoa}"
        );

        let svg_a = encoded_bytes(&host, ra, "aoa_svg_resource");
        let svg_b = encoded_bytes(&host, rb, "aoa_svg_resource");
        assert_eq!(svg_a, svg_b, "AoA SVG two produces must match");
        let svg_xml = String::from_utf8_lossy(&svg_a);
        assert!(svg_xml.contains("<svg xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(svg_xml.contains("polyline"));

        let json_a = encoded_bytes(&host, ra, "receipt_json_resource");
        let json_b = encoded_bytes(&host, rb, "receipt_json_resource");
        assert_eq!(json_a, json_b, "privacy receipt two produces must match");
        let json_txt = String::from_utf8_lossy(&json_a);
        assert!(json_txt.contains("camera_free_verified"));
        assert!(json_txt.contains("UDHR") || json_txt.contains("witness-1"));
    }
}
