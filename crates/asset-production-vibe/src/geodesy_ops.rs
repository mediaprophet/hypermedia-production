//! Bounded geospatial terrain heightfield sculpting, hypsometric relief rendering,
//! and Matter-64 (`.m64`) subsurface strata compilation/verification for the `geodesy` category.

use super::*;
use qualia_core_db::q42::m64_matter::{
    brick_voxel_linear_index, encode_hilbert3d_u64, M64ChunkDirectoryEntry, M64Header,
    M64HoloRecord, M64LocalMetricHeader, M64PayloadInterval, M64PointRecord, M64ReadLimits,
    M64Reader, M64StrataColumnRecord, COORD_FRAME_CARTESIAN_MICROMETER, LRM3_BLOCK_BYTES,
    M64_BRICK3D_BYTES, M64_BRICK3D_RECORDS, M64_BRICK3D_SIDE, M64_FLAG_3D_BRICK_16X16X16,
    M64_FLAG_LOCAL_METRIC_LRM3, M64_MODE_HOLO, M64_MODE_POINT, M64_MODE_STRATA,
    M64_SRID_LOCAL_MICROMETER,
};
use std::collections::BTreeMap;
use terrain_brushes::{CowTerrainTileGrid, StrokeSample};

struct ParsedBrushStroke {
    op_code: u8,
    falloff_code: u8,
    x: f32,
    y: f32,
    radius: f32,
    strength: f32,
    target_height: f32,
    step_m: f32,
}

fn parse_brush_strokes(v: &Value) -> Result<Vec<ParsedBrushStroke>, String> {
    let items = list(v)?;
    if items.len() > 256 {
        return Err("strokes count exceeds 256".into());
    }
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        let r = record(item)?;
        fields(
            r,
            &[
                "op",
                "x",
                "y",
                "radius",
                "strength",
                "falloff",
                "target_height",
                "step_m",
            ],
        )?;
        let op_str = r.get("op").map(text).transpose()?.unwrap_or("raise");
        let op_code = match op_str {
            "raise" => 0,
            "lower" => 1,
            "flatten" => 2,
            "smooth" => 3,
            "terrace" => 4,
            "hydraulic" => 5,
            _ => {
                return Err(format!(
                    "Unknown terrain brush op '{op_str}'; expected raise|lower|flatten|smooth|terrace|hydraulic"
                ))
            }
        };
        let falloff_str = r
            .get("falloff")
            .map(text)
            .transpose()?
            .unwrap_or("smoothstep");
        let falloff_code = match falloff_str {
            "smoothstep" => 0,
            "gaussian" => 1,
            "linear" => 2,
            "plateau" => 3,
            _ => {
                return Err(format!(
                    "Unknown falloff '{falloff_str}'; expected smoothstep|gaussian|linear|plateau"
                ))
            }
        };
        let x = number(r, "x", 0.0)? as f32;
        let y = number(r, "y", 0.0)? as f32;
        let radius = number(r, "radius", 8.0)? as f32;
        if !(0.1..=10_000.0).contains(&radius) {
            return Err("stroke radius outside 0.1..=10000.0".into());
        }
        let strength = number(r, "strength", 5.0)? as f32;
        if !(0.0..=10_000.0).contains(&strength) {
            return Err("stroke strength outside 0.0..=10000.0".into());
        }
        let target_height = number(r, "target_height", 100.0)? as f32;
        let step_m = number(r, "step_m", 5.0)? as f32;
        out.push(ParsedBrushStroke {
            op_code,
            falloff_code,
            x,
            y,
            radius,
            strength,
            target_height,
            step_m,
        });
    }
    Ok(out)
}

fn sculpt_tile(
    tile_res: usize,
    cell_spacing_m: f32,
    base_height_m: f32,
    strokes: &[ParsedBrushStroke],
    byte_budget: usize,
) -> Result<Vec<f32>, String> {
    let mut grid = CowTerrainTileGrid::new(tile_res, cell_spacing_m, byte_budget.max(65536))
        .map_err(|e| e.to_string())?;
    grid.ensure_tile(0, 0, base_height_m)
        .map_err(|e| e.to_string())?;
    if !strokes.is_empty() {
        grid.begin_stroke().map_err(|e| e.to_string())?;
        for (idx, s) in strokes.iter().enumerate() {
            let sample = StrokeSample {
                stroke_id: (idx + 1) as u64,
                timestamp_ms: (idx as u64 + 1) * 16,
                center_x_m: s.x,
                center_y_m: s.y,
                radius_m: s.radius,
                strength: s.strength,
                target_height_m: s.target_height,
                step_m: s.step_m,
                op_code: s.op_code,
                falloff_code: s.falloff_code,
                layer_index: 0,
                flags: 0,
                seed: 42,
                pad: [0; 2],
            };
            grid.apply_stroke_sample(sample)
                .map_err(|e| e.to_string())?;
        }
        grid.commit_stroke("sculpt").map_err(|e| e.to_string())?;
    }
    let tile = grid
        .tiles()
        .get(&(0, 0))
        .ok_or_else(|| "Missing terrain tile (0, 0)".to_string())?;
    Ok((**tile).clone())
}

fn self_verify_m64(buffer: &[u8], record_count: u64, expected_mode: u8) -> Result<(), String> {
    let limits = M64ReadLimits {
        max_file_bytes: buffer.len() + 1024,
        max_chunks: 4,
        max_records: record_count + 16,
    };
    let mut scratch = [M64PayloadInterval::default(); 4];
    let reader = M64Reader::parse(buffer, limits, &mut scratch)
        .map_err(|e| format!("M64 self-verification failed: {e}"))?;
    let mode = reader
        .chunk(0)
        .map(|c| c.directory_entry().chunk_record_mode);
    if mode != Some(expected_mode) {
        return Err(format!(
            "M64 self-verification failed: expected mode {expected_mode:#04x}, got {mode:?}"
        ));
    }
    let v = reader.verification();
    match expected_mode {
        M64_MODE_STRATA => {
            if v.record_crc_checked != record_count {
                return Err("M64 self-verification failed: strata CRC count mismatch".into());
            }
        }
        M64_MODE_HOLO | M64_MODE_POINT => {
            if v.record_crc_checked != 0 || v.records_without_checksum != record_count {
                return Err(
                    "M64 self-verification failed: occupancy records must omit per-record CRC"
                        .into(),
                );
            }
            let prefix = reader
                .chunk(0)
                .map(|c| c.directory_entry().bao_root_hash_prefix);
            if prefix == Some([0u8; 12]) {
                return Err(
                    "M64 self-verification failed: occupancy Bao prefix is unsigned".into(),
                );
            }
        }
        _ => {}
    }
    Ok(())
}

fn fill_occupancy_brick(
    mode: u8,
    heights: &[f32],
    side: usize,
    min_h: f32,
    span: f32,
    out: &mut [u8],
) {
    let denom = (M64_BRICK3D_SIDE - 1).max(1);
    for lz in 0..M64_BRICK3D_SIDE as u8 {
        for ly in 0..M64_BRICK3D_SIDE as u8 {
            for lx in 0..M64_BRICK3D_SIDE as u8 {
                let gx = (lx as usize * (side - 1)) / denom;
                let gy = (ly as usize * (side - 1)) / denom;
                let elev = heights[gy * side + gx];
                let t = ((elev - min_h) / span).clamp(0.0, 1.0);
                let z_top = (t * denom as f32).round() as u8;
                let occupied = lz <= z_top;
                let idx = brick_voxel_linear_index(lx, ly, lz);
                let off = idx * 64;
                match mode {
                    M64_MODE_HOLO => {
                        // Occupied voxels keep Default holographic fields (tropism, scale, SPD).
                        let mut rec = M64HoloRecord::default();
                        rec.local_origin = [
                            f32_to_f16(lx as f32),
                            f32_to_f16(ly as f32),
                            f32_to_f16(lz as f32),
                        ];
                        rec.metric_variety = 0;
                        rec.semantic_node_idx = u8::from(occupied);
                        if occupied {
                            rec.mass_amplitude = 0x3c00;
                            rec.sdf_harmonics[0] = (t * 40.0).round().clamp(-127.0, 127.0) as i8;
                        } else {
                            rec.mass_amplitude = 0;
                            rec.spectral_spd_base = [0; 6];
                            rec.tropism_sensitivity = [0; 4];
                            rec.sdf_harmonics = [0; 12];
                        }
                        out[off..off + 64].copy_from_slice(bytemuck::bytes_of(&rec));
                    }
                    _ => {
                        let mut rec = M64PointRecord::default();
                        rec.x = lx as f32 * 1e-3;
                        rec.y = ly as f32 * 1e-3;
                        rec.z = lz as f32 * 1e-3;
                        rec.intensity = if occupied { 1.0 } else { 0.0 };
                        rec.classification = if occupied { 2 } else { 0 };
                        rec.pack_normal([0.0, 0.0, 1.0]);
                        out[off..off + 64].copy_from_slice(bytemuck::bytes_of(&rec));
                    }
                }
            }
        }
    }
}

/// First-class 6-layer STRATA column: sculpted elevation plus slope hydrology
/// (overland-flow velocity, sediment load) without replacing Default LUT/compaction.
fn first_class_strata_column(heights: &[f32], side: usize, i: usize) -> M64StrataColumnRecord {
    let gx = i % side;
    let gy = i / side;
    let elev = heights[i];
    let xl = heights[gy * side + gx.saturating_sub(1)];
    let xr = heights[gy * side + (gx + 1).min(side - 1)];
    let yd = heights[gy.saturating_sub(1) * side + gx];
    let yu = heights[(gy + 1).min(side - 1) * side + gx];
    let dzdx = xr - xl;
    let dzdy = yu - yd;
    let slope = (dzdx * dzdx + dzdy * dzdy).sqrt();
    let vx = (-dzdx * 50.0)
        .round()
        .clamp(i16::MIN as f32, i16::MAX as f32) as i16;
    let vy = (-dzdy * 50.0)
        .round()
        .clamp(i16::MIN as f32, i16::MAX as f32) as i16;
    let mut rec = M64StrataColumnRecord::default();
    rec.surface_elevation_mm = (elev * 1000.0).round() as i32;
    rec.surface_fluid_depth_mm = (slope * 20.0).round().clamp(0.0, 65535.0) as u16;
    rec.fluid_velocity_xy = [vx, vy];
    rec.fluid_sediment_load_ppm = (slope * 80.0).round().clamp(0.0, 65535.0) as u16;
    rec.layer_thickness_mm[0] = rec.layer_thickness_mm[0]
        .saturating_add(((elev.abs() * 0.1) as u16).min(200));
    rec.update_crc();
    rec
}

fn f32_to_f16(f: f32) -> u16 {
    let f = if f.is_finite() { f } else { 0.0 };
    let bits = f.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exp = ((bits >> 23) & 0xff) as i32;
    let mant = bits & 0x7fffff;
    if exp == 0 {
        return sign;
    }
    if exp == 255 {
        return sign | 0x7c00;
    }
    let new_exp = exp - 127 + 15;
    if new_exp <= 0 {
        return sign;
    }
    if new_exp >= 31 {
        return sign | 0x7c00;
    }
    sign | ((new_exp as u16) << 10) | ((mant >> 13) as u16)
}

impl AssetHost<'_> {
    pub(super) fn geodesy_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.geodesy.sculpt_terrain" | "geodesy.sculpt_terrain" => {
                self.sculpt_terrain(a)
            }
            "hypermedia.geodesy.compile_m64" | "geodesy.compile_m64" => self.compile_m64(a),
            "hypermedia.geodesy.verify_m64" | "geodesy.verify_m64" => self.verify_m64(a),
            "hypermedia.geodesy.kriging_variogram_morphometry"
            | "geodesy.kriging_variogram_morphometry"
            | "synth.geodesy.kriging_variogram_morphometry"
            | "kriging_variogram_morphometry" => self.kriging_variogram_morphometry(a),
            "hypermedia.geodesy.advect_m64_grid"
            | "geodesy.advect_m64_grid"
            | "synth.geodesy.advect_m64_grid"
            | "advect_m64_grid" => self.advect_m64_grid(a),
            "hypermedia.geodesy.couple_continuum_xpbd"
            | "geodesy.couple_continuum_xpbd"
            | "synth.geodesy.couple_continuum_xpbd"
            | "couple_continuum_xpbd" => self.couple_continuum_xpbd(a),
            _ => Err(format!("Unknown geodesy operation: {id}")),
        }
    }

    fn sculpt_terrain(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "tile_resolution",
                "cell_spacing_m",
                "base_height_m",
                "strokes",
                "width",
                "height",
            ],
        )?;
        let tile_res = count(a, "tile_resolution", 64, 256)?;
        if tile_res < 8 {
            return Err("tile_resolution must be in 8..=256".into());
        }
        let cell_spacing_m = number(a, "cell_spacing_m", 1.0)? as f32;
        if !(0.01..=1000.0).contains(&cell_spacing_m) {
            return Err("cell_spacing_m outside 0.01..=1000.0".into());
        }
        let base_height_m = number(a, "base_height_m", 100.0)? as f32;
        if !(-10_000.0..=10_000.0).contains(&base_height_m) {
            return Err("base_height_m outside -10000..=10000".into());
        }
        let strokes = match a.get("strokes") {
            Some(v) => parse_brush_strokes(v)?,
            None => Vec::new(),
        };
        let w = count(a, "width", 128, 1024)? as u32;
        let h = count(a, "height", 128, 1024)? as u32;

        let work = tile_res * tile_res * (strokes.len() + 2) + (w as usize * h as usize * 4);
        self.charge(
            work,
            tile_res * tile_res * 16 + (w as usize * h as usize * 8),
        )?;

        let heights = sculpt_tile(
            tile_res,
            cell_spacing_m,
            base_height_m,
            &strokes,
            self.limits.total_resource_bytes,
        )?;

        let mut min_h = f32::INFINITY;
        let mut max_h = f32::NEG_INFINITY;
        let mut sum_h = 0.0f64;
        for &v in &heights {
            min_h = min_h.min(v);
            max_h = max_h.max(v);
            sum_h += v as f64;
        }
        let mean_h = sum_h / (heights.len().max(1) as f64);
        let span = (max_h - min_h).max(1e-3);

        // Render hypsometric shaded-relief + contour-line straight-alpha RGBA8 lattice
        let mut pixels = Vec::with_capacity(w as usize * h as usize);
        let light_dir = [-0.577_350_26f32, -0.577_350_26, 0.577_350_26];
        for py in 0..h {
            let gy = ((py as usize * (tile_res - 1)) / (h.max(2) as usize - 1)).min(tile_res - 1);
            for px in 0..w {
                let gx =
                    ((px as usize * (tile_res - 1)) / (w.max(2) as usize - 1)).min(tile_res - 1);
                let elev = heights[gy * tile_res + gx];
                let norm_t = ((elev - min_h) / span).clamp(0.0, 1.0);

                // Central differences for surface normal
                let xl = heights[gy * tile_res + gx.saturating_sub(1)];
                let xr = heights[gy * tile_res + (gx + 1).min(tile_res - 1)];
                let yd = heights[gy.saturating_sub(1) * tile_res + gx];
                let yu = heights[(gy + 1).min(tile_res - 1) * tile_res + gx];
                let dzdx = (xr - xl) / (2.0 * cell_spacing_m);
                let dzdy = (yu - yd) / (2.0 * cell_spacing_m);
                let mut nx = -dzdx;
                let mut ny = -dzdy;
                let mut nz = 1.0f32;
                let nlen = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-6);
                nx /= nlen;
                ny /= nlen;
                nz /= nlen;

                let shade =
                    (nx * light_dir[0] + ny * light_dir[1] + nz * light_dir[2]).clamp(0.2, 1.0);

                // Hypsometric ramp (valley green -> ochre slope -> alpine snow)
                let (base_r, base_g, base_b) = if norm_t < 0.5 {
                    let u = norm_t * 2.0;
                    (48.0 + u * 130.0, 110.0 + u * 50.0, 56.0 + u * 30.0)
                } else {
                    let u = (norm_t - 0.5) * 2.0;
                    (178.0 + u * 68.0, 160.0 + u * 82.0, 86.0 + u * 156.0)
                };

                // Contour lines every 5 meters
                let contour = ((elev / 5.0).fract() - 0.5).abs() < 0.04;
                let contour_mul = if contour { 0.72 } else { 1.0 };

                let r = (base_r * shade * contour_mul).round().clamp(0.0, 255.0) as u8;
                let g = (base_g * shade * contour_mul).round().clamp(0.0, 255.0) as u8;
                let b = (base_b * shade * contour_mul).round().clamp(0.0, 255.0) as u8;
                pixels.push([r, g, b, 255]);
            }
        }

        let rgba_handle = self.store_rgba(w, h, pixels)?;
        Ok(Value::Record(BTreeMap::from([
            ("relief_rgba".into(), rgba_handle),
            ("min_elevation_m".into(), Value::F64(min_h as f64)),
            ("max_elevation_m".into(), Value::F64(max_h as f64)),
            ("mean_elevation_m".into(), Value::F64(mean_h)),
        ])))
    }

    fn compile_m64(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "origin_lat_lon",
                "base_elevation_m",
                "grid_side",
                "strokes",
                "record_mode",
            ],
        )?;
        let origin = match a.get("origin_lat_lon") {
            Some(v) => vector::<2>(v)?,
            None => [-35.78, 148.29],
        };
        if !(-90.0..=90.0).contains(&origin[0]) || !(-180.0..=180.0).contains(&origin[1]) {
            return Err("origin_lat_lon outside valid geodetic range".into());
        }
        let base_elev = number(a, "base_elevation_m", 350.0)? as f32;
        if !(-10_000.0..=10_000.0).contains(&base_elev) {
            return Err("base_elevation_m outside -10000..=10000".into());
        }
        let side = count(a, "grid_side", 8, 32)?;
        if side < 2 {
            return Err("grid_side must be in 2..=32".into());
        }
        let strokes = match a.get("strokes") {
            Some(v) => parse_brush_strokes(v)?,
            None => Vec::new(),
        };
        let record_mode = a
            .get("record_mode")
            .map(text)
            .transpose()?
            .unwrap_or("strata");
        match record_mode {
            "strata" => self.compile_m64_strata(origin, base_elev, side, &strokes),
            "holo" => self.compile_m64_occupancy(origin, base_elev, side, &strokes, M64_MODE_HOLO),
            "point" => {
                self.compile_m64_occupancy(origin, base_elev, side, &strokes, M64_MODE_POINT)
            }
            other => Err(format!(
                "Unknown record_mode '{other}'; expected strata|holo|point"
            )),
        }
    }

    fn compile_m64_strata(
        &mut self,
        origin: [f64; 2],
        base_elev: f32,
        side: usize,
        strokes: &[ParsedBrushStroke],
    ) -> Result<Value, String> {
        let record_count = side * side;
        self.charge(
            record_count * (strokes.len() + 8),
            record_count * 128 + 16384,
        )?;

        let heights = sculpt_tile(
            side,
            1.0,
            base_elev,
            strokes,
            self.limits.total_resource_bytes,
        )?;

        let payload_bytes_len = record_count * 64;
        let total_file_size = 128 + payload_bytes_len;
        let mut buffer = vec![0u8; total_file_size];

        for (i, _) in heights.iter().enumerate() {
            let rec = first_class_strata_column(&heights, side, i);
            let offset = 128 + i * 64;
            buffer[offset..offset + 64].copy_from_slice(bytemuck::bytes_of(&rec));
        }

        let mut header = M64Header::default();
        header.total_file_size = total_file_size as u64;
        header.chunk_count = 1;
        header.chunk_table_offset = 64;
        header.spatial_origin = [origin[0] as f32, origin[1] as f32];
        header.elevation_datum = [base_elev, 1.0];
        header.update_crc();

        let mut entry = M64ChunkDirectoryEntry {
            chunk_uuid: [0x64; 16],
            spatial_hilbert_index: encode_hilbert3d_u64(0, 0, 0, 2),
            byte_offset: 128,
            byte_length: payload_bytes_len as u32,
            record_count: record_count as u32,
            srid_geodetic_code: 4326,
            lut_bank_epoch_id: 1,
            chunk_record_mode: M64_MODE_STRATA,
            compression_flags: 0,
            bao_root_hash_prefix: [0u8; 12],
            entry_crc32c: 0,
        };
        entry.update_crc();

        buffer[..64].copy_from_slice(bytemuck::bytes_of(&header));
        buffer[64..128].copy_from_slice(bytemuck::bytes_of(&entry));

        self_verify_m64(&buffer, record_count as u64, M64_MODE_STRATA)?;
        self.encode("m64", buffer)
    }

    fn compile_m64_occupancy(
        &mut self,
        origin: [f64; 2],
        base_elev: f32,
        side: usize,
        strokes: &[ParsedBrushStroke],
        mode: u8,
    ) -> Result<Value, String> {
        let record_count = M64_BRICK3D_RECORDS;
        self.charge(
            record_count * (strokes.len() + 8),
            record_count * 128 + LRM3_BLOCK_BYTES + 16384,
        )?;

        let heights = sculpt_tile(
            side,
            1.0,
            base_elev,
            strokes,
            self.limits.total_resource_bytes,
        )?;

        let mut min_h = f32::INFINITY;
        let mut max_h = f32::NEG_INFINITY;
        for &v in &heights {
            min_h = min_h.min(v);
            max_h = max_h.max(v);
        }
        let span = (max_h - min_h).max(1e-3);

        // header (64) + LRM3 (128) + directory (64) + one 16³ brick
        let dir_off = 64 + LRM3_BLOCK_BYTES;
        let payload_off = dir_off + 64;
        let total_file_size = payload_off + M64_BRICK3D_BYTES;
        let mut buffer = vec![0u8; total_file_size];

        let mut lrm3 = M64LocalMetricHeader::default();
        lrm3.coord_frame_kind = COORD_FRAME_CARTESIAN_MICROMETER;
        lrm3.anisotropic_scale_um = [1000.0, 1000.0, 1000.0];
        let bbox = (M64_BRICK3D_SIDE as i32) * 1000;
        lrm3.bbox_max_um = [bbox, bbox, bbox];
        lrm3.update_crc();
        buffer[64..dir_off].copy_from_slice(bytemuck::bytes_of(&lrm3));

        fill_occupancy_brick(
            mode,
            &heights,
            side,
            min_h,
            span,
            &mut buffer[payload_off..],
        );

        // Occupancy omits per-record CRC; a non-zero prefix makes M64Reader hash the brick.
        let digest = blake3::hash(&buffer[payload_off..]);
        let mut bao_root_hash_prefix = [0u8; 12];
        bao_root_hash_prefix.copy_from_slice(&digest.as_bytes()[..12]);

        let mut header = M64Header::default();
        header.total_file_size = total_file_size as u64;
        header.chunk_count = 1;
        header.chunk_table_offset = dir_off as u64;
        header.spatial_origin = [origin[0] as f32, origin[1] as f32];
        header.elevation_datum = [base_elev, 1.0];
        header.container_flags =
            (M64_FLAG_3D_BRICK_16X16X16 as u32) | (M64_FLAG_LOCAL_METRIC_LRM3 as u32);
        header.update_crc();

        let mut entry = M64ChunkDirectoryEntry {
            chunk_uuid: [0x64; 16],
            spatial_hilbert_index: encode_hilbert3d_u64(0, 0, 0, 4),
            byte_offset: payload_off as u64,
            byte_length: M64_BRICK3D_BYTES as u32,
            record_count: record_count as u32,
            srid_geodetic_code: M64_SRID_LOCAL_MICROMETER,
            lut_bank_epoch_id: 1,
            chunk_record_mode: mode,
            compression_flags: 0,
            bao_root_hash_prefix,
            entry_crc32c: 0,
        };
        entry.update_crc();

        buffer[..64].copy_from_slice(bytemuck::bytes_of(&header));
        buffer[dir_off..payload_off].copy_from_slice(bytemuck::bytes_of(&entry));

        self_verify_m64(&buffer, record_count as u64, mode)?;
        self.encode("m64", buffer)
    }

    fn verify_m64(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["m64"])?;
        let m64_val = required(a, "m64")?;
        let bytes: Vec<u8> = if let Ok(key) = text(m64_val) {
            self.inputs
                .iter()
                .find(|r| r.key == key)
                .ok_or("Missing retained M64 input")?
                .bytes
                .clone()
        } else if let Ok(res) = self.workspace.encoded(m64_val) {
            if res.kind != "m64" {
                return Err(format!("Expected 'm64' resource, got '{}'", res.kind));
            }
            res.bytes.clone()
        } else {
            return Err("Expected retained key or encoded 'm64' handle".into());
        };

        self.charge(bytes.len() * 4 + 256, bytes.len() + 4096)?;

        let limits = M64ReadLimits {
            max_file_bytes: self.limits.resource_bytes,
            max_chunks: 1024,
            max_records: 262_144,
        };
        let mut scratch = vec![M64PayloadInterval::default(); 1024];
        let reader = M64Reader::parse(&bytes, limits, &mut scratch)
            .map_err(|e| format!("M64 verification failed: {e}"))?;
        // QualiaDB parse admits HOLO/POINT with a zero prefix; untrusted occupancy needs a Bao seal.
        for i in 0..reader.verification().chunks {
            let chunk = reader
                .chunk(i)
                .ok_or_else(|| format!("M64 verification failed: missing chunk {i}"))?;
            let entry = chunk.directory_entry();
            if matches!(
                entry.chunk_record_mode,
                M64_MODE_HOLO | M64_MODE_POINT
            ) && entry.bao_root_hash_prefix == [0u8; 12]
                && entry.compression_flags != 2
            {
                return Err("M64 occupancy omitted BLAKE3-Bao seal".into());
            }
        }
        let v = reader.verification();
        let h = reader.header();

        Ok(Value::Record(BTreeMap::from([
            ("valid".into(), Value::Bool(true)),
            ("chunks".into(), Value::U64(v.chunks as u64)),
            ("total_records".into(), Value::U64(v.total_records)),
            ("crc_checked".into(), Value::U64(v.record_crc_checked)),
            ("origin_lat".into(), Value::F64(h.spatial_origin[0] as f64)),
            ("origin_lon".into(), Value::F64(h.spatial_origin[1] as f64)),
        ])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::q42::m64_matter::{
        has_m64_magic, M64Header, M64PayloadInterval, M64ReadLimits, M64Reader, E64_MAGIC,
        M64_MAGIC, M64_MODE_HOLO, M64_MODE_POINT, M64_MODE_STRATA,
    };
    use std::sync::atomic::AtomicBool;

    fn make_test_host<'a>(
        limits: &'a ToolLimits,
        cancel: &'a AtomicBool,
        inputs: &'a [Resource],
    ) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs,
            limits,
            cancel,
            work: 0,
        }
    }

    fn encoded_bytes(host: &AssetHost<'_>, handle: &Value) -> Vec<u8> {
        match host.workspace.get(handle).expect("encoded m64") {
            Artifact::Encoded(resource) => {
                assert_eq!(resource.kind, "m64");
                resource.bytes.clone()
            }
            _ => panic!("Expected encoded m64 resource"),
        }
    }

    fn parse_m64(bytes: &[u8]) -> M64Reader<'_> {
        let limits = M64ReadLimits {
            max_file_bytes: bytes.len() + 1024,
            max_chunks: 4,
            max_records: 8192,
        };
        let mut scratch = [M64PayloadInterval::default(); 4];
        M64Reader::parse(bytes, limits, &mut scratch).expect("M64Reader::parse")
    }

    #[test]
    fn compile_m64_strata_default_and_e64_magic_alias_round_trip() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mut args = BTreeMap::new();
        args.insert("grid_side".into(), Value::U64(4));
        args.insert("base_elevation_m".into(), Value::F64(350.0));

        let handle = host
            .geodesy_op("hypermedia.geodesy.compile_m64", &args)
            .expect("compile_m64 strata");
        let bytes = encoded_bytes(&host, &handle);
        assert_eq!(&bytes[..4], &M64_MAGIC);
        assert!(has_m64_magic(&bytes));

        let reader = parse_m64(&bytes);
        assert_eq!(
            reader.chunk(0).unwrap().directory_entry().chunk_record_mode,
            M64_MODE_STRATA
        );
        assert_eq!(reader.verification().record_crc_checked, 16);
        assert_eq!(reader.verification().records_without_checksum, 0);

        let mut verify_args = BTreeMap::new();
        verify_args.insert("m64".into(), handle);
        let summary = host
            .geodesy_op("hypermedia.geodesy.verify_m64", &verify_args)
            .expect("verify_m64");
        let rec = record(&summary).expect("record");
        assert_eq!(required(rec, "valid").ok(), Some(&Value::Bool(true)));

        let mut aliased = bytes.to_vec();
        let mut header: M64Header = bytemuck::pod_read_unaligned(&aliased[..64]);
        header.magic = E64_MAGIC;
        header.update_crc();
        aliased[..64].copy_from_slice(bytemuck::bytes_of(&header));
        assert!(has_m64_magic(&aliased));
        assert_eq!(&aliased[..4], &E64_MAGIC);
        let aliased_reader = parse_m64(&aliased);
        assert_eq!(aliased_reader.header().magic, E64_MAGIC);
        assert!(has_m64_magic(&aliased_reader.header().magic));
        assert_eq!(
            aliased_reader
                .chunk(0)
                .unwrap()
                .directory_entry()
                .chunk_record_mode,
            M64_MODE_STRATA
        );
    }

    #[test]
    fn compile_m64_holo_occupancy_omits_record_crc() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mut args = BTreeMap::new();
        args.insert("grid_side".into(), Value::U64(4));
        args.insert("record_mode".into(), Value::String("holo".into()));

        let handle = host
            .geodesy_op("hypermedia.geodesy.compile_m64", &args)
            .expect("compile_m64 holo");
        let bytes = encoded_bytes(&host, &handle);
        assert!(has_m64_magic(&bytes));
        assert_eq!(&bytes[..4], &M64_MAGIC);

        let reader = parse_m64(&bytes);
        let chunk = reader.chunk(0).unwrap();
        let entry = chunk.directory_entry();
        assert_eq!(entry.chunk_record_mode, M64_MODE_HOLO);
        assert_eq!(reader.verification().record_crc_checked, 0);
        assert_eq!(
            reader.verification().records_without_checksum,
            M64_BRICK3D_RECORDS as u64
        );
        assert_ne!(
            entry.bao_root_hash_prefix,
            [0u8; 12],
            "occupancy Bao prefix must be non-zero"
        );
        assert_eq!(entry.srid_geodetic_code, M64_SRID_LOCAL_MICROMETER);

        let mut verify_args = BTreeMap::new();
        verify_args.insert("m64".into(), handle);
        let summary = host
            .geodesy_op("hypermedia.geodesy.verify_m64", &verify_args)
            .expect("verify_m64 holo");
        let rec = record(&summary).expect("record");
        assert_eq!(required(rec, "valid").ok(), Some(&Value::Bool(true)));

        let mut tampered = bytes.clone();
        let payload_off = 64 + LRM3_BLOCK_BYTES + 64;
        tampered[payload_off] ^= 0x01;
        let limits = M64ReadLimits {
            max_file_bytes: tampered.len() + 1024,
            max_chunks: 4,
            max_records: 8192,
        };
        let mut scratch = [M64PayloadInterval::default(); 4];
        assert!(
            M64Reader::parse(&tampered, limits, &mut scratch).is_err(),
            "occupancy payload tamper must fail Bao"
        );

        let mut unsigned = bytes.clone();
        strip_occupancy_bao_keep_dir_crc(&mut unsigned);
        let unsigned_limits = M64ReadLimits {
            max_file_bytes: unsigned.len() + 1024,
            max_chunks: 4,
            max_records: 8192,
        };
        let mut unsigned_scratch = [M64PayloadInterval::default(); 4];
        assert!(
            M64Reader::parse(&unsigned, unsigned_limits, &mut unsigned_scratch).is_ok(),
            "QualiaDB parse still admits CRC-only occupancy"
        );
        let admitted = host
            .admit_encoded("m64", unsigned)
            .expect("admit CRC-only occupancy");
        let mut unsigned_args = BTreeMap::new();
        unsigned_args.insert("m64".into(), admitted);
        let err = host
            .geodesy_op("hypermedia.geodesy.verify_m64", &unsigned_args)
            .expect_err("Bao-omitted occupancy must fail closed");
        assert!(
            err.contains("Bao") || err.contains("seal"),
            "fail-closed error must name Bao seal, got: {err}"
        );
    }

    #[test]
    fn geodesy_strata_holo_kriging_e64_objects() {
        use geodesy_spatial_analysis::{
            Coord, SamplePoint, VariogramModel, VariogramParams, predict_kriging,
        };
        use qualia_core_db::q42::m64_matter::{M64HoloRecord, M64StrataColumnRecord};

        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mut stroke = BTreeMap::new();
        stroke.insert("op".into(), Value::String("raise".into()));
        stroke.insert("x".into(), Value::F64(0.0));
        stroke.insert("y".into(), Value::F64(0.0));
        stroke.insert("radius".into(), Value::F64(8.0));
        stroke.insert("strength".into(), Value::F64(20.0));
        stroke.insert("falloff".into(), Value::String("smoothstep".into()));
        stroke.insert("target_height".into(), Value::F64(400.0));
        stroke.insert("step_m".into(), Value::F64(5.0));

        let mut strata_args = BTreeMap::new();
        strata_args.insert("grid_side".into(), Value::U64(4));
        strata_args.insert("base_elevation_m".into(), Value::F64(350.0));
        strata_args.insert("record_mode".into(), Value::String("strata".into()));
        strata_args.insert("strokes".into(), Value::List(vec![Value::Record(stroke)]));

        let strata_handle = host
            .geodesy_op("hypermedia.geodesy.compile_m64", &strata_args)
            .expect("compile_m64 strata object");
        let strata_bytes = encoded_bytes(&host, &strata_handle);
        assert_eq!(&strata_bytes[..4], &M64_MAGIC);

        let strata_reader = parse_m64(&strata_bytes);
        assert_eq!(
            strata_reader
                .chunk(0)
                .unwrap()
                .directory_entry()
                .chunk_record_mode,
            M64_MODE_STRATA
        );
        assert_eq!(strata_reader.verification().record_crc_checked, 16);

        let mut elev_mm = Vec::with_capacity(16);
        let mut layer0 = 0u16;
        for i in 0..16 {
            let rec: M64StrataColumnRecord =
                bytemuck::pod_read_unaligned(&strata_bytes[128 + i * 64..128 + (i + 1) * 64]);
            rec.validate().expect("STRATA CRC");
            assert!(
                rec.layer_thickness_mm.iter().all(|&t| t > 0),
                "6-layer STRATA stack must be first-class"
            );
            layer0 = rec.layer_thickness_mm[0];
            elev_mm.push(rec.surface_elevation_mm);
        }
        assert!(layer0 >= 400, "topsoil thickness stays first-class");
        assert!(
            elev_mm.iter().any(|&z| z != elev_mm[0]),
            "raise stroke must vary STRATA elevations"
        );

        let samples = vec![
            SamplePoint::new(0.0, 0.0, elev_mm[0] as f64),
            SamplePoint::new(3.0, 0.0, elev_mm[3] as f64),
            SamplePoint::new(0.0, 3.0, elev_mm[12] as f64),
            SamplePoint::new(3.0, 3.0, elev_mm[15] as f64),
        ];
        let vparams = VariogramParams::new(VariogramModel::Spherical, 0.0, 100.0, 4.0);
        let kriged = predict_kriging(&samples, Coord::new(1.5, 1.5), &vparams)
            .expect("ordinary kriging of STRATA corners");
        assert!(kriged.estimate.is_finite());
        assert!(kriged.variance >= 0.0);

        let mut aliased = strata_bytes.clone();
        let mut header: M64Header = bytemuck::pod_read_unaligned(&aliased[..64]);
        header.magic = E64_MAGIC;
        header.update_crc();
        aliased[..64].copy_from_slice(bytemuck::bytes_of(&header));
        assert_eq!(&aliased[..4], &E64_MAGIC);
        assert!(has_m64_magic(&aliased));
        let e64_reader = parse_m64(&aliased);
        assert_eq!(e64_reader.header().magic, E64_MAGIC);
        assert_eq!(
            e64_reader
                .chunk(0)
                .unwrap()
                .directory_entry()
                .chunk_record_mode,
            M64_MODE_STRATA
        );

        let mut holo_args = BTreeMap::new();
        holo_args.insert("grid_side".into(), Value::U64(4));
        holo_args.insert("record_mode".into(), Value::String("holo".into()));
        let holo_handle = host
            .geodesy_op("hypermedia.geodesy.compile_m64", &holo_args)
            .expect("compile_m64 holo object");
        let holo_bytes = encoded_bytes(&host, &holo_handle);
        let holo_reader = parse_m64(&holo_bytes);
        assert_eq!(
            holo_reader
                .chunk(0)
                .unwrap()
                .directory_entry()
                .chunk_record_mode,
            M64_MODE_HOLO
        );
        assert_eq!(holo_reader.verification().record_crc_checked, 0);
        let payload_off = 64 + LRM3_BLOCK_BYTES + 64;
        let holo_rec: M64HoloRecord = bytemuck::pod_read_unaligned(
            &holo_bytes[payload_off..payload_off + 64],
        );
        assert_eq!(holo_rec.metric_variety, 0);
        assert_eq!(holo_rec.semantic_node_idx, 1);
        assert_eq!(holo_rec.mass_amplitude, 0x3c00);

        let kriging = host
            .geodesy_op(
                "hypermedia.geodesy.kriging_variogram_morphometry",
                &BTreeMap::new(),
            )
            .expect("kriging object");
        let krec = record(&kriging).expect("kriging record");
        let estimate = match krec.get("kriging_estimate") {
            Some(Value::F64(v)) => *v,
            _ => panic!("kriging_estimate object"),
        };
        let variance = match krec.get("kriging_variance") {
            Some(Value::F64(v)) => *v,
            _ => panic!("kriging_variance object"),
        };
        assert!(
            (estimate - 120.0).abs() < 1.0,
            "ordinary kriging center object: {estimate}"
        );
        assert!(variance >= 0.0);
        assert!(krec.contains_key("kriging_image"));
        assert!(krec.contains_key("contour_svg_resource"));

        let mut advect_args = BTreeMap::new();
        advect_args.insert("grid_side".into(), Value::U64(8));
        advect_args.insert("steps".into(), Value::U64(1));
        advect_args.insert("poisson_iterations".into(), Value::U64(4));
        let advected = host
            .geodesy_op("hypermedia.geodesy.advect_m64_grid", &advect_args)
            .expect("advect object");
        let arec = record(&advected).expect("advect record");
        assert_eq!(arec.get("record_mode"), Some(&Value::String("strata".into())));
        let adv_m64 = host
            .workspace
            .encoded(arec.get("m64").expect("advected m64"))
            .expect("encoded advected m64");
        assert_eq!(adv_m64.kind, "m64");
        assert!(has_m64_magic(&adv_m64.bytes));

        let eroded = host
            .erode_geomorphic_heightfield(&BTreeMap::new())
            .expect("erosion object");
        let erec = record(&eroded).expect("erosion record");
        assert!(erec.contains_key("heightfield_image"));
        assert!(erec.contains_key("mesh"));
        match erec.get("cell_count") {
            Some(Value::I64(n)) => assert!(*n > 0, "eroded cell_count"),
            _ => panic!("erosion cell_count object"),
        }
    }

    #[test]
    fn compile_m64_point_occupancy_omits_record_crc() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mut args = BTreeMap::new();
        args.insert("grid_side".into(), Value::U64(4));
        args.insert("record_mode".into(), Value::String("point".into()));

        let handle = host
            .geodesy_op("hypermedia.geodesy.compile_m64", &args)
            .expect("compile_m64 point");
        let bytes = encoded_bytes(&host, &handle);
        assert!(has_m64_magic(&bytes));

        let reader = parse_m64(&bytes);
        let chunk = reader.chunk(0).unwrap();
        let entry = chunk.directory_entry();
        assert_eq!(entry.chunk_record_mode, M64_MODE_POINT);
        assert_eq!(reader.verification().record_crc_checked, 0);
        assert_eq!(
            reader.verification().records_without_checksum,
            M64_BRICK3D_RECORDS as u64
        );
        assert_ne!(
            entry.bao_root_hash_prefix,
            [0u8; 12],
            "occupancy Bao prefix must be non-zero"
        );

        let mut verify_args = BTreeMap::new();
        verify_args.insert("m64".into(), handle);
        let summary = host
            .geodesy_op("hypermedia.geodesy.verify_m64", &verify_args)
            .expect("verify_m64 point");
        let rec = record(&summary).expect("record");
        assert_eq!(required(rec, "valid").ok(), Some(&Value::Bool(true)));

        let mut tampered = bytes.clone();
        let payload_off = 64 + LRM3_BLOCK_BYTES + 64;
        tampered[payload_off] ^= 0x01;
        let limits = M64ReadLimits {
            max_file_bytes: tampered.len() + 1024,
            max_chunks: 4,
            max_records: 8192,
        };
        let mut scratch = [M64PayloadInterval::default(); 4];
        assert!(
            M64Reader::parse(&tampered, limits, &mut scratch).is_err(),
            "occupancy payload tamper must fail Bao"
        );
    }

    fn strip_occupancy_bao_keep_dir_crc(bytes: &mut [u8]) {
        let dir_off = 64 + LRM3_BLOCK_BYTES;
        let mut entry: M64ChunkDirectoryEntry =
            bytemuck::pod_read_unaligned(&bytes[dir_off..dir_off + 64]);
        entry.bao_root_hash_prefix = [0u8; 12];
        entry.compression_flags = 0;
        entry.update_crc();
        bytes[dir_off..dir_off + 64].copy_from_slice(bytemuck::bytes_of(&entry));
    }
}
