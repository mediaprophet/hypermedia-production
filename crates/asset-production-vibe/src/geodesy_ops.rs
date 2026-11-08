//! Bounded geospatial terrain heightfield sculpting, hypsometric relief rendering,
//! and Matter-64 (`.m64`) subsurface strata compilation/verification for the `geodesy` category.

use super::*;
use qualia_core_db::q42::m64_matter::{
    encode_hilbert3d_u64, M64ChunkDirectoryEntry, M64Header, M64PayloadInterval, M64ReadLimits,
    M64Reader, M64StrataColumnRecord, M64_MODE_STRATA,
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
            grid.apply_stroke_sample(sample).map_err(|e| e.to_string())?;
        }
        grid.commit_stroke("sculpt").map_err(|e| e.to_string())?;
    }
    let tile = grid
        .tiles()
        .get(&(0, 0))
        .ok_or_else(|| "Missing terrain tile (0, 0)".to_string())?;
    Ok((**tile).clone())
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
        self.charge(work, tile_res * tile_res * 16 + (w as usize * h as usize * 8))?;

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
                    (
                        48.0 + u * 130.0,
                        110.0 + u * 50.0,
                        56.0 + u * 30.0,
                    )
                } else {
                    let u = (norm_t - 0.5) * 2.0;
                    (
                        178.0 + u * 68.0,
                        160.0 + u * 82.0,
                        86.0 + u * 156.0,
                    )
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

        let record_count = side * side;
        self.charge(
            record_count * (strokes.len() + 8),
            record_count * 128 + 16384,
        )?;

        let heights = sculpt_tile(
            side,
            1.0,
            base_elev,
            &strokes,
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

        // Self-verify structural and per-record CRC-32C integrity before sealing
        let limits = M64ReadLimits {
            max_file_bytes: total_file_size + 1024,
            max_chunks: 4,
            max_records: record_count as u64 + 16,
        };
        let mut scratch = [M64PayloadInterval::default(); 4];
        M64Reader::parse(&buffer, limits, &mut scratch)
            .map_err(|e| format!("M64 self-verification failed: {e}"))?;

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
