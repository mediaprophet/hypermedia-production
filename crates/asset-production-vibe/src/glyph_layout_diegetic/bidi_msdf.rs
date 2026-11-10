//! Unicode BiDi (UAX #9) complex script shaping and MSDF font atlas packing.

use crate::*;
use kurbo::{BezPath, PathEl};
use std::collections::BTreeMap;
use vibe::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BidiClass {
    L,
    R,
    AL,
    EN,
    AN,
    N,
}

fn classify_bidi(ch: char) -> BidiClass {
    let cp = ch as u32;
    match cp {
        0x0590..=0x05FF | 0xFB1D..=0xFB4F => BidiClass::R,
        0x0600..=0x065F
        | 0x066A..=0x06EF
        | 0x06FA..=0x074F
        | 0x0750..=0x077F
        | 0x08A0..=0x08FF
        | 0xFB50..=0xFDFF
        | 0xFE70..=0xFEFF => BidiClass::AL,
        0x0660..=0x0669 | 0x06F0..=0x06F9 => BidiClass::AN,
        0x0030..=0x0039 => BidiClass::EN,
        0x0009..=0x000D | 0x0020 | 0x00A0 => BidiClass::N,
        0x0021..=0x002F
        | 0x003A..=0x0040
        | 0x005B..=0x0060
        | 0x007B..=0x007E
        | 0x00A1..=0x00BF
        | 0x2000..=0x206F => BidiClass::N,
        _ => BidiClass::L,
    }
}

#[derive(Clone, Debug)]
struct BidiChar {
    orig_idx: usize,
    ch: char,
    level: u8,
}

#[derive(Clone, Copy, Debug)]
struct LineSeg {
    p0: (f64, f64),
    p1: (f64, f64),
}

impl LineSeg {
    #[inline]
    fn dist_sq(&self, p: (f64, f64)) -> f64 {
        let vx = self.p1.0 - self.p0.0;
        let vy = self.p1.1 - self.p0.1;
        let wx = p.0 - self.p0.0;
        let wy = p.1 - self.p0.1;
        let l2 = vx * vx + vy * vy;
        if l2 < 1e-10 {
            return wx * wx + wy * wy;
        }
        let t = ((wx * vx + wy * vy) / l2).clamp(0.0, 1.0);
        let qx = self.p0.0 + t * vx;
        let qy = self.p0.1 + t * vy;
        let dx = p.0 - qx;
        let dy = p.1 - qy;
        dx * dx + dy * dy
    }

    #[inline]
    fn dist(&self, p: (f64, f64)) -> f64 {
        self.dist_sq(p).sqrt()
    }

    #[inline]
    fn channel_bin(&self) -> usize {
        let dx = self.p1.0 - self.p0.0;
        let dy = self.p1.1 - self.p0.1;
        let angle = dy.atan2(dx);
        let pi = std::f64::consts::PI;
        let mut phi = angle % pi;
        if phi < 0.0 {
            phi += pi;
        }
        if phi < pi / 6.0 || phi >= 5.0 * pi / 6.0 {
            0
        } else if phi >= pi / 3.0 && phi < 2.0 * pi / 3.0 {
            1
        } else {
            2
        }
    }
}

fn extract_segments(path: &BezPath) -> Vec<LineSeg> {
    let mut segs = Vec::new();
    let mut cur = (0.0, 0.0);
    let mut start = (0.0, 0.0);

    for el in path.elements() {
        match el {
            PathEl::MoveTo(p) => {
                cur = (p.x, p.y);
                start = cur;
            }
            PathEl::LineTo(p) => {
                let next = (p.x, p.y);
                segs.push(LineSeg { p0: cur, p1: next });
                cur = next;
            }
            PathEl::QuadTo(p1, p2) => {
                const STEPS: usize = 4;
                for i in 1..=STEPS {
                    let t = i as f64 / STEPS as f64;
                    let omt = 1.0 - t;
                    let qx = omt * omt * cur.0 + 2.0 * omt * t * p1.x + t * t * p2.x;
                    let qy = omt * omt * cur.1 + 2.0 * omt * t * p1.y + t * t * p2.y;
                    segs.push(LineSeg {
                        p0: cur,
                        p1: (qx, qy),
                    });
                    cur = (qx, qy);
                }
            }
            PathEl::CurveTo(p1, p2, p3) => {
                const STEPS: usize = 6;
                for i in 1..=STEPS {
                    let t = i as f64 / STEPS as f64;
                    let omt = 1.0 - t;
                    let cx = omt * omt * omt * cur.0
                        + 3.0 * omt * omt * t * p1.x
                        + 3.0 * omt * t * t * p2.x
                        + t * t * t * p3.x;
                    let cy = omt * omt * omt * cur.1
                        + 3.0 * omt * omt * t * p1.y
                        + 3.0 * omt * t * t * p2.y
                        + t * t * t * p3.y;
                    segs.push(LineSeg {
                        p0: cur,
                        p1: (cx, cy),
                    });
                    cur = (cx, cy);
                }
            }
            PathEl::ClosePath => {
                if (cur.0 - start.0).hypot(cur.1 - start.1) > 1e-6 {
                    segs.push(LineSeg { p0: cur, p1: start });
                }
                cur = start;
            }
        }
    }
    segs
}

impl AssetHost<'_> {
    pub(super) fn shape_harfbuzz_bidi(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["text", "base_direction", "font_size", "script"])?;

        let text_val = text(required(a, "text")?)?;
        if text_val.len() > 65536 {
            return Err("text exceeds 65536 bytes".into());
        }

        let base_dir_str = match a.get("base_direction") {
            Some(v) => text(v)?,
            None => "auto",
        };
        if base_dir_str != "auto" && base_dir_str != "ltr" && base_dir_str != "rtl" {
            return Err("base_direction must be 'auto', 'ltr', or 'rtl'".into());
        }

        let font_size = number(a, "font_size", 20.0)?;
        if !(4.0..=256.0).contains(&font_size) {
            return Err("font_size outside 4.0..=256.0".into());
        }

        let script_str = match a.get("script") {
            Some(v) => text(v)?,
            None => "auto",
        };
        if script_str != "auto"
            && script_str != "latin"
            && script_str != "arabic"
            && script_str != "hebrew"
            && script_str != "cjk"
            && script_str != "devanagari"
        {
            return Err(
                "script must be 'auto', 'latin', 'arabic', 'hebrew', 'cjk', or 'devanagari'".into(),
            );
        }

        self.charge(text_val.len().max(1) * 32, text_val.len() * 64 + 1024)?;

        let resolved_dir = match base_dir_str {
            "ltr" => "ltr",
            "rtl" => "rtl",
            _ => {
                let mut d = "ltr";
                for ch in text_val.chars() {
                    match classify_bidi(ch) {
                        BidiClass::R | BidiClass::AL => {
                            d = "rtl";
                            break;
                        }
                        BidiClass::L => {
                            d = "ltr";
                            break;
                        }
                        _ => {}
                    }
                }
                d
            }
        };

        let base_level: u8 = if resolved_dir == "rtl" { 1 } else { 0 };

        let chars: Vec<char> = text_val.chars().collect();
        let n = chars.len();
        let classes: Vec<BidiClass> = chars.iter().copied().map(classify_bidi).collect();
        let mut levels = vec![base_level; n];

        for i in 0..n {
            match classes[i] {
                BidiClass::R | BidiClass::AL => {
                    levels[i] = 1;
                }
                BidiClass::L | BidiClass::EN | BidiClass::AN => {
                    levels[i] = if base_level == 1 { 2 } else { 0 };
                }
                BidiClass::N => {}
            }
        }

        for i in 0..n {
            if classes[i] == BidiClass::N {
                let mut prev_level = base_level;
                for j in (0..i).rev() {
                    if classes[j] != BidiClass::N {
                        prev_level = levels[j];
                        break;
                    }
                }
                let mut next_level = base_level;
                for j in (i + 1)..n {
                    if classes[j] != BidiClass::N {
                        next_level = levels[j];
                        break;
                    }
                }
                if (prev_level % 2) == (next_level % 2) {
                    levels[i] = prev_level;
                } else {
                    levels[i] = base_level;
                }
            }
        }

        let mut rtl_run_count = 0u64;
        let mut idx = 0;
        while idx < levels.len() {
            if levels[idx] % 2 == 1 {
                rtl_run_count += 1;
                while idx < levels.len() && levels[idx] % 2 == 1 {
                    idx += 1;
                }
            } else {
                idx += 1;
            }
        }

        // UAX #9 L2: reverse contiguous sequences from the highest level down.
        let visual_order = vector_glyph_shaping::reorder_visual(&levels);
        let bidi_chars: Vec<BidiChar> = visual_order
            .into_iter()
            .map(|i| BidiChar {
                orig_idx: i,
                ch: chars[i],
                level: levels[i],
            })
            .collect();

        let visual_text: String = bidi_chars.iter().map(|bc| bc.ch).collect();

        let mut glyph_clusters = Vec::with_capacity(bidi_chars.len());
        let mut total_advance_px = 0.0;
        let mut prev_char: Option<char> = None;

        for bc in &bidi_chars {
            let mut adv = crate::glyph::glyph_advance(bc.ch);
            let cp = bc.ch as u32;
            if (0x4E00..=0x9FFF).contains(&cp) || (0x3000..=0x30FF).contains(&cp) {
                adv = 1.0;
            }
            let kern = if let Some(pc) = prev_char {
                crate::glyph::pair_kerning(pc, bc.ch)
            } else {
                0.0
            };
            let adv_px = (adv + kern).max(0.0) * font_size;
            total_advance_px += adv_px;
            prev_char = Some(bc.ch);

            let mut cluster = BTreeMap::new();
            cluster.insert("cluster_index".into(), Value::U64(bc.orig_idx as u64));
            cluster.insert("codepoint".into(), Value::U64(bc.ch as u32 as u64));
            cluster.insert("glyph".into(), Value::String(bc.ch.to_string()));
            cluster.insert("advance_x".into(), Value::F64(adv_px));
            cluster.insert("offset_x".into(), Value::F64(0.0));
            cluster.insert("offset_y".into(), Value::F64(0.0));
            cluster.insert("bidi_level".into(), Value::U64(bc.level as u64));
            glyph_clusters.push(Value::Record(cluster));
        }

        let mut record = BTreeMap::new();
        record.insert("logical_text".into(), Value::String(text_val.to_string()));
        record.insert("visual_text".into(), Value::String(visual_text));
        record.insert(
            "resolved_direction".into(),
            Value::String(resolved_dir.to_string()),
        );
        record.insert("rtl_run_count".into(), Value::U64(rtl_run_count));
        record.insert("total_advance_px".into(), Value::F64(total_advance_px));
        record.insert("glyph_clusters".into(), Value::List(glyph_clusters));

        Ok(Value::Record(record))
    }

    pub(super) fn pack_msdf_font_atlas(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["charset", "cell_size", "px_range"])?;

        let default_charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+-*/=<>()";
        let charset_str = match a.get("charset") {
            Some(v) => text(v)?,
            None => default_charset,
        };

        let cell_size = count(a, "cell_size", 24, 64)?;
        if cell_size < 12 {
            return Err("cell_size must be at least 12".into());
        }

        let px_range = number(a, "px_range", 4.0)?;
        if !(1.0..=16.0).contains(&px_range) {
            return Err("px_range outside 1.0..=16.0".into());
        }

        let mut unique = BTreeMap::new();
        for ch in charset_str.chars() {
            unique.entry(ch as u32).or_insert(ch);
        }
        if unique.is_empty() {
            unique.insert('?' as u32, '?');
        }
        let unique_chars: Vec<char> = unique.into_values().collect();
        let glyph_count = unique_chars.len();

        let cols = ((glyph_count as f64).sqrt().ceil() as usize).max(1);
        let rows = (glyph_count + cols - 1) / cols;
        let atlas_w = (cols * cell_size) as u32;
        let atlas_h = (rows * cell_size) as u32;

        let num_pixels = (atlas_w * atlas_h) as usize;
        self.charge(
            num_pixels * 4 + glyph_count * 128,
            num_pixels * 16 + 8192,
        )?;

        let mut pixels = vec![[0u8, 0, 0, 0]; num_pixels];
        let mut glyph_metrics = Vec::with_capacity(glyph_count);

        let map_sd = |sd: f64| -> u8 {
            let val = 128.0 + (sd / px_range) * 127.0;
            val.clamp(0.0, 255.0).round() as u8
        };

        for (g_idx, &ch) in unique_chars.iter().enumerate() {
            let col = g_idx % cols;
            let row = g_idx / cols;
            let cell_x = col * cell_size;
            let cell_y = row * cell_size;

            let scale = cell_size as f64 * 0.82;
            let origin_x = cell_x as f64 + cell_size as f64 * 0.10;
            let origin_y = cell_y as f64 + cell_size as f64 * 0.08;

            let mut svg_d = String::new();
            crate::glyph::char_to_svg_subpath(ch, origin_x, origin_y, scale, &mut svg_d);

            let bez_path = if svg_d.trim().is_empty() {
                BezPath::new()
            } else {
                BezPath::from_svg(&svg_d).unwrap_or_else(|_| BezPath::new())
            };

            let segs = extract_segments(&bez_path);
            let stroke_radius = cell_size as f64 * 0.07;

            for cy in 0..cell_size {
                let py = cell_y + cy;
                for cx in 0..cell_size {
                    let px = cell_x + cx;
                    let pt = (px as f64 + 0.5, py as f64 + 0.5);

                    let mut min_d = [f64::MAX; 3];
                    let mut global_min_d = f64::MAX;

                    for seg in &segs {
                        let d = seg.dist(pt);
                        if d < global_min_d {
                            global_min_d = d;
                        }
                        let bin = seg.channel_bin();
                        if d < min_d[bin] {
                            min_d[bin] = d;
                        }
                    }

                    for b in 0..3 {
                        if min_d[b] == f64::MAX {
                            min_d[b] = global_min_d;
                        }
                    }
                    if global_min_d == f64::MAX {
                        global_min_d = 9999.0;
                        min_d = [9999.0; 3];
                    }

                    let sd_r = stroke_radius - min_d[0];
                    let sd_g = stroke_radius - min_d[1];
                    let sd_b = stroke_radius - min_d[2];
                    let sd_a = stroke_radius - global_min_d;

                    let pixel_idx = (py as usize) * (atlas_w as usize) + (px as usize);
                    pixels[pixel_idx] = [
                        map_sd(sd_r),
                        map_sd(sd_g),
                        map_sd(sd_b),
                        map_sd(sd_a),
                    ];
                }
            }

            let advance = crate::glyph::glyph_advance(ch);
            // em (0,0) maps to (origin_x, origin_y); em (advance, 1) maps to that plus scale.
            let atlas_left = origin_x;
            let atlas_top = origin_y;
            let atlas_right = origin_x + advance * scale;
            let atlas_bottom = origin_y + scale;
            let u0 = atlas_left / atlas_w as f64;
            let v0 = atlas_top / atlas_h as f64;
            let u1 = atlas_right / atlas_w as f64;
            let v1 = atlas_bottom / atlas_h as f64;

            let gm = serde_json::json!({
                "char": ch.to_string(),
                "codepoint": ch as u32,
                "advance": advance,
                "cell": {
                    "x": cell_x,
                    "y": cell_y,
                    "width": cell_size,
                    "height": cell_size
                },
                "uv": {
                    "u0": u0,
                    "v0": v0,
                    "u1": u1,
                    "v1": v1
                },
                "plane_bounds": {
                    "left": 0.0,
                    "top": 0.0,
                    "right": advance,
                    "bottom": 1.0
                },
                "atlas_bounds": {
                    "left": atlas_left,
                    "top": atlas_top,
                    "right": atlas_right,
                    "bottom": atlas_bottom
                }
            });
            glyph_metrics.push(gm);
        }

        let fm = crate::glyph::FontMetrics::default();
        let atlas_meta = serde_json::json!({
            "atlas": {
                "type": "msdf",
                "distance_range": px_range,
                "cell_size": cell_size,
                "width": atlas_w,
                "height": atlas_h,
                "glyph_count": glyph_count,
                "y_origin": "top",
                "em_size": fm.em_size
            },
            "metrics": {
                "em_size": fm.em_size,
                "ascender": fm.ascender,
                "cap_height": fm.cap_height,
                "x_height": fm.x_height,
                "baseline": fm.baseline,
                "descender": fm.descender,
                "line_height": fm.line_height
            },
            "glyphs": glyph_metrics
        });

        let json_bytes = serde_json::to_vec_pretty(&atlas_meta)
            .map_err(|e| format!("Failed to serialize metrics json: {e}"))?;

        let metrics_json = self.encode("json", json_bytes)?;
        let atlas_rgba = self.store_rgba(atlas_w, atlas_h, pixels)?;

        let mut record = BTreeMap::new();
        record.insert("atlas_rgba".into(), atlas_rgba);
        record.insert("metrics_json".into(), metrics_json);
        record.insert("glyph_count".into(), Value::U64(glyph_count as u64));
        record.insert("atlas_width".into(), Value::U64(atlas_w as u64));
        record.insert("atlas_height".into(), Value::U64(atlas_h as u64));

        Ok(Value::Record(record))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    fn test_host<'a>(cancel: &'a AtomicBool, limits: &'a ToolLimits) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits,
            cancel,
            work: 0,
        }
    }

    #[test]
    fn shape_harfbuzz_bidi_mixed_and_rtl_numbers_l2() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = test_host(&cancel, &limits);

        let mut mixed = BTreeMap::new();
        mixed.insert("text".into(), Value::String("Hello שלום".into()));
        mixed.insert("font_size".into(), Value::F64(16.0));
        let res = host.shape_harfbuzz_bidi(&mixed).expect("mixed bidi");
        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("resolved_direction"), Some(&Value::String("ltr".into())));
        assert_eq!(r.get("visual_text"), Some(&Value::String("Hello םולש".into())));
        assert_eq!(r.get("rtl_run_count"), Some(&Value::U64(1)));

        let mut rtl_num = BTreeMap::new();
        rtl_num.insert("text".into(), Value::String("שלום 123".into()));
        let res = host.shape_harfbuzz_bidi(&rtl_num).expect("rtl numbers");
        let Value::Record(r) = &res else { panic!("expected record") };
        assert_eq!(r.get("resolved_direction"), Some(&Value::String("rtl".into())));
        assert_eq!(
            r.get("visual_text"),
            Some(&Value::String("123 םולש".into())),
            "UAX #9 L2 keeps European numbers LTR on the visual left of RTL"
        );

        let again = host.shape_harfbuzz_bidi(&rtl_num).expect("rtl numbers b");
        assert_eq!(res, again, "two BiDi produces must match");
    }

    #[test]
    fn pack_msdf_font_atlas_metrics_map_em_box_and_two_produces_match() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = test_host(&cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("charset".into(), Value::String("CAB012".into()));
        args.insert("cell_size".into(), Value::U64(24));
        args.insert("px_range".into(), Value::F64(4.0));

        let a = host.pack_msdf_font_atlas(&args).expect("atlas a");
        let b = host.pack_msdf_font_atlas(&args).expect("atlas b");
        let Value::Record(ra) = &a else { panic!("expected record") };
        let Value::Record(rb) = &b else { panic!("expected record") };
        assert_eq!(ra.get("glyph_count"), Some(&Value::U64(6)));
        assert_eq!(ra.get("atlas_width"), Some(&Value::U64(72)));
        assert_eq!(ra.get("atlas_height"), Some(&Value::U64(48)));

        let json_a = host.workspace.encoded(ra.get("metrics_json").unwrap()).expect("json a");
        let json_b = host.workspace.encoded(rb.get("metrics_json").unwrap()).expect("json b");
        assert_eq!(json_a.bytes, json_b.bytes, "two metrics JSON produces must match");

        let parsed: serde_json::Value = serde_json::from_slice(&json_a.bytes).expect("valid json");
        assert_eq!(parsed["atlas"]["type"], "msdf");
        assert_eq!(parsed["atlas"]["y_origin"], "top");
        assert_eq!(parsed["metrics"]["em_size"], 1.0);
        assert_eq!(parsed["metrics"]["baseline"], 0.78);
        assert_eq!(parsed["metrics"]["line_height"], 1.25);
        assert_eq!(parsed["metrics"]["ascender"], 0.12);
        assert_eq!(parsed["metrics"]["descender"], 0.96);

        let glyphs = parsed["glyphs"].as_array().expect("glyphs array");
        assert_eq!(glyphs.len(), 6);
        let cps: Vec<u32> = glyphs
            .iter()
            .map(|g| g["codepoint"].as_u64().unwrap() as u32)
            .collect();
        let mut sorted = cps.clone();
        sorted.sort_unstable();
        assert_eq!(cps, sorted, "atlas glyphs packed in codepoint order");

        let cell_size = 24.0f64;
        let scale = cell_size * 0.82;
        let pad_x = cell_size * 0.10;
        let pad_y = cell_size * 0.08;
        let atlas_w = 72.0f64;
        let atlas_h = 48.0f64;
        for g in glyphs {
            let adv = g["advance"].as_f64().unwrap();
            let expected_adv = crate::glyph::glyph_advance(
                char::from_u32(g["codepoint"].as_u64().unwrap() as u32).unwrap(),
            );
            assert!((adv - expected_adv).abs() < 1e-12);
            assert_eq!(g["plane_bounds"]["left"], 0.0);
            assert_eq!(g["plane_bounds"]["top"], 0.0);
            assert_eq!(g["plane_bounds"]["bottom"], 1.0);
            assert!((g["plane_bounds"]["right"].as_f64().unwrap() - adv).abs() < 1e-12);

            let cell_x = g["cell"]["x"].as_f64().unwrap();
            let cell_y = g["cell"]["y"].as_f64().unwrap();
            let ab_left = g["atlas_bounds"]["left"].as_f64().unwrap();
            let ab_top = g["atlas_bounds"]["top"].as_f64().unwrap();
            let ab_right = g["atlas_bounds"]["right"].as_f64().unwrap();
            let ab_bottom = g["atlas_bounds"]["bottom"].as_f64().unwrap();
            assert!((ab_left - (cell_x + pad_x)).abs() < 1e-9);
            assert!((ab_top - (cell_y + pad_y)).abs() < 1e-9);
            assert!((ab_right - (cell_x + pad_x + adv * scale)).abs() < 1e-9);
            assert!((ab_bottom - (cell_y + pad_y + scale)).abs() < 1e-9);
            assert!((g["uv"]["u0"].as_f64().unwrap() - ab_left / atlas_w).abs() < 1e-12);
            assert!((g["uv"]["v0"].as_f64().unwrap() - ab_top / atlas_h).abs() < 1e-12);
            assert!((g["uv"]["u1"].as_f64().unwrap() - ab_right / atlas_w).abs() < 1e-12);
            assert!((g["uv"]["v1"].as_f64().unwrap() - ab_bottom / atlas_h).abs() < 1e-12);
        }

        let atlas_a = host.rgba(ra.get("atlas_rgba").unwrap()).expect("rgba a");
        let atlas_b = host.rgba(rb.get("atlas_rgba").unwrap()).expect("rgba b");
        assert_eq!(atlas_a.pixels, atlas_b.pixels, "two atlas rasters must match");
        let max_ch = atlas_a
            .pixels
            .iter()
            .flat_map(|p| p.iter().copied())
            .max()
            .unwrap_or(0);
        let min_ch = atlas_a
            .pixels
            .iter()
            .flat_map(|p| p.iter().copied())
            .min()
            .unwrap_or(255);
        assert!(max_ch > 128, "MSDF must have near/inside-stroke samples, max={max_ch}");
        assert!(min_ch < 128, "MSDF must have outside-stroke samples, min={min_ch}");
    }
}
