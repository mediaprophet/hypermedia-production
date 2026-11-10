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
                        let mut rec = M64HoloRecord::default();
                        rec.local_origin = [
                            f32_to_f16(lx as f32),
                            f32_to_f16(ly as f32),
                            f32_to_f16(lz as f32),
                        ];
                        rec.metric_variety = 0;
                        rec.semantic_node_idx = u8::from(occupied);
                        rec.mass_amplitude = if occupied { 0x3c00 } else { 0 };
                        rec.spectral_spd_base = if occupied {
                            [200, 180, 160, 220, 240, 210]
                        } else {
                            [0; 6]
                        };
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

        for (i, &elev_m) in heights.iter().enumerate() {
            let mut rec = M64StrataColumnRecord::default();
            rec.surface_elevation_mm = (elev_m * 1000.0).round() as i32;
            rec.update_crc();
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
            bao_root_hash_prefix: [0u8; 12],
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
        assert_eq!(
            reader.chunk(0).unwrap().directory_entry().chunk_record_mode,
            M64_MODE_HOLO
        );
        assert_eq!(reader.verification().record_crc_checked, 0);
        assert_eq!(
            reader.verification().records_without_checksum,
            M64_BRICK3D_RECORDS as u64
        );
        assert_eq!(
            reader
                .chunk(0)
                .unwrap()
                .directory_entry()
                .srid_geodetic_code,
            M64_SRID_LOCAL_MICROMETER
        );
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
        assert_eq!(
            reader.chunk(0).unwrap().directory_entry().chunk_record_mode,
            M64_MODE_POINT
        );
        assert_eq!(reader.verification().record_crc_checked, 0);
        assert_eq!(
            reader.verification().records_without_checksum,
            M64_BRICK3D_RECORDS as u64
        );
    }
}
