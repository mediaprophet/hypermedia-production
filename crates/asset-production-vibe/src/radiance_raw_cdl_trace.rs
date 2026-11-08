use super::*;
use std::collections::BTreeMap;
use vibe::Value;

impl AssetHost<'_> {
    /// 128: `hypermedia.radiance.develop_camera_raw_hdr`
    /// (`radiance-camera-raw` + `color-transforms`)
    pub(crate) fn develop_camera_raw_hdr(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["image", "width", "height", "exposure_ev", "kelvin"])?;
        let exposure_ev = number(a, "exposure_ev", 0.5)?;
        let kelvin = number(a, "kelvin", 5500.0)?;

        let (width, height, raw) = if let Some(v) = a.get("image") {
            let img = self.rgba(v)?;
            let w = (img.width as usize).clamp(4, 256);
            let h = (img.height as usize).clamp(4, 256);
            let mut bayer = vec![1024u16; w * h];
            for y in 0..h {
                for x in 0..w {
                    let px = img
                        .pixels
                        .get(y * (img.width as usize) + x)
                        .copied()
                        .unwrap_or([128, 128, 128, 255]);
                    let ch = match (y & 1, x & 1) {
                        (0, 0) => px[0],
                        (1, 1) => px[2],
                        _ => px[1],
                    };
                    bayer[y * w + x] = 512 + (ch as u16) * 60;
                }
            }
            (w, h, bayer)
        } else {
            let w = count(a, "width", 8, 128)?;
            let h = count(a, "height", 8, 128)?;
            let mut bayer = vec![1024u16; w * h];
            for y in 0..h {
                for x in 0..w {
                    bayer[y * w + x] = (1024 + ((x + y) * 512) % 8192) as u16;
                }
            }
            (w, h, bayer)
        };

        self.charge(width * height * 8, width * height * 32)?;

        use color_transforms::icc::{Intent, Transform, display_p3, srgb, write::write_profile};
        use radiance_camera_raw::{
            CfaPattern, DemosaicAlgo, DevelopSettings, ExposureBracket, HdrOptions, demosaic_raw,
            develop_image, merge_hdr,
        };

        let demosaiced = demosaic_raw(
            &raw,
            width,
            height,
            CfaPattern::Rggb,
            16384,
            512,
            DemosaicAlgo::Rcd,
        );

        let mut dev_settings = DevelopSettings::default();
        dev_settings.wb.temp = kelvin;
        dev_settings.tone.exposure = exposure_ev;
        let developed = develop_image(&demosaiced, width, height, &dev_settings);
        let settings_hash = dev_settings.cache_digest();

        let brackets = vec![
            ExposureBracket {
                rgb: developed.clone(),
                exposure_time: 0.01,
            },
            ExposureBracket {
                rgb: developed,
                exposure_time: 0.04,
            },
        ];
        let hdr = merge_hdr(&brackets, width, height, HdrOptions::default())
            .map_err(|e| format!("HDR merge failed: {e}"))?;

        let src_prof = srgb();
        let dst_prof = display_p3();
        let icc_xform = Transform::new(&src_prof, &dst_prof, Intent::RelativeColorimetric, true)
            .map_err(|e| format!("ICC transform init failed: {e}"))?;
        let mut p3_sample = [0.0f32; 3];
        icc_xform.eval(&hdr[0..3], &mut p3_sample);

        let mut p3_pixels = Vec::with_capacity(width * height);
        for rgb in hdr.chunks_exact(3) {
            let mut p3 = [0.0f32; 3];
            icc_xform.eval(rgb, &mut p3);
            p3_pixels.push([
                (p3[0].clamp(0.0, 1.0) * 255.0).round() as u8,
                (p3[1].clamp(0.0, 1.0) * 255.0).round() as u8,
                (p3[2].clamp(0.0, 1.0) * 255.0).round() as u8,
                255,
            ]);
        }
        let image_handle = self.store_rgba(width as u32, height as u32, p3_pixels)?;
        let icc_bytes = write_profile(&dst_prof);
        let icc_resource = self.encode("icc", icc_bytes)?;

        let mut out = BTreeMap::new();
        out.insert("image".into(), image_handle);
        out.insert("icc_resource".into(), icc_resource);
        out.insert("pixel_count".into(), Value::I64((hdr.len() / 3) as i64));
        out.insert("develop_settings_hash".into(), Value::U64(settings_hash));
        out.insert("icc_p3_r".into(), Value::F64(p3_sample[0] as f64));
        out.insert("icc_p3_g".into(), Value::F64(p3_sample[1] as f64));
        out.insert("icc_p3_b".into(), Value::F64(p3_sample[2] as f64));
        Ok(Value::Record(out))
    }

    /// 129: `hypermedia.radiance.grade_cdl_scopes`
    /// (`radiance-color-grading` + `raster-blend-operators`)
    pub(crate) fn grade_cdl_scopes(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["image", "slope_r", "slope_g", "slope_b", "saturation"])?;
        let slope_r = number(a, "slope_r", 1.1)?;
        let slope_g = number(a, "slope_g", 1.0)?;
        let slope_b = number(a, "slope_b", 0.9)?;
        let sat = number(a, "saturation", 1.15)?;

        self.charge(256, 4096)?;

        use radiance_color_grading::{
            AscCdl, ColorWheels, LogCurve, Lut3d,
            scopes::{
                ParadeType, Params as ScopeParams, Signal as ScopeSignal, WaveformType,
                paint::vectorscope as paint_vectorscope, parade, vectorscope_yuv, waveform,
            },
        };
        use raster_blend_operators::{BlendMode, blend_mode_rgb, cmyk_planes, planes_cmyk};

        let cdl = AscCdl {
            slope: [slope_r, slope_g, slope_b],
            offset: [0.01, 0.0, -0.01],
            power: [1.0, 1.0, 1.0],
            sat,
        };
        let wheels = ColorWheels::default();
        let lut = Lut3d::from_fn(4, |rgb| {
            let w = wheels.apply(rgb);
            let g = cdl.apply([w[0] as f64, w[1] as f64, w[2] as f64], true);
            [g[0] as f32, g[1] as f32, g[2] as f32]
        });

        let sample_log = LogCurve::LogC4.encode(0.18);
        let sample_lin = LogCurve::LogC4.decode(sample_log) as f32;
        let graded_rgb = lut.apply([sample_lin, sample_lin, sample_lin]);
        let blended = blend_mode_rgb(BlendMode::SoftLight, [0.5, 0.5, 0.5], graded_rgb);

        let (w, h, src_pixels) = if let Some(v) = a.get("image") {
            let img = self.rgba(v)?;
            (img.width, img.height, img.pixels.clone())
        } else {
            let mut bars = Vec::with_capacity(16 * 16);
            let palette: [[u8; 4]; 8] = [
                [235, 235, 235, 255],
                [235, 235, 16, 255],
                [16, 235, 235, 255],
                [16, 235, 16, 255],
                [235, 16, 235, 255],
                [235, 16, 16, 255],
                [16, 16, 235, 255],
                [46, 46, 46, 255],
            ];
            for y in 0..16u32 {
                let luma_scale = 0.5 + 0.5 * (y as f32 / 15.0);
                for x in 0..16u32 {
                    let c = palette[(x as usize / 2) % 8];
                    bars.push([
                        ((c[0] as f32) * luma_scale).round() as u8,
                        ((c[1] as f32) * luma_scale).round() as u8,
                        ((c[2] as f32) * luma_scale).round() as u8,
                        255,
                    ]);
                }
            }
            (16, 16, bars)
        };

        let mut graded_pixels = Vec::with_capacity(src_pixels.len());
        let mut rgba_frame = Vec::with_capacity(src_pixels.len() * 4);
        for px in &src_pixels {
            let r_lin = LogCurve::LogC4.decode(LogCurve::LogC4.encode(px[0] as f64 / 255.0)) as f32;
            let g_lin = LogCurve::LogC4.decode(LogCurve::LogC4.encode(px[1] as f64 / 255.0)) as f32;
            let b_lin = LogCurve::LogC4.decode(LogCurve::LogC4.encode(px[2] as f64 / 255.0)) as f32;
            let g_rgb = lut.apply([r_lin, g_lin, b_lin]);
            let b_rgb = blend_mode_rgb(BlendMode::SoftLight, [r_lin, g_lin, b_lin], g_rgb);
            let out_px = [
                (b_rgb[0].clamp(0.0, 1.0) * 255.0).round() as u8,
                (b_rgb[1].clamp(0.0, 1.0) * 255.0).round() as u8,
                (b_rgb[2].clamp(0.0, 1.0) * 255.0).round() as u8,
                px[3],
            ];
            rgba_frame.extend_from_slice(&out_px);
            graded_pixels.push(out_px);
        }
        let image_handle = self.store_rgba(w, h, graded_pixels)?;

        let sig = ScopeSignal::from_rgba8(w as usize, h as usize, &rgba_frame);
        let sp = ScopeParams {
            rows: 16,
            vector_size: 16,
            ..Default::default()
        };
        let wf = waveform(&sig, WaveformType::Rgb, &sp);
        let vs = vectorscope_yuv(&sig, &sp);
        let pr = parade(&sig, ParadeType::Rgb, &sp);

        let (sw, sh, vs_rgba) = paint_vectorscope(&vs, [0.2, 0.9, 0.4], 1.5, true);
        let scope_pixels: Vec<[u8; 4]> = vs_rgba
            .chunks_exact(4)
            .map(|c| {
                if c[3] == 0 {
                    [16, 20, 24, 255]
                } else {
                    [c[0], c[1], c[2], 255]
                }
            })
            .collect();
        let scope_image = self.store_rgba(sw as u32, sh as u32, scope_pixels)?;

        let cube_str = lut.to_cube();
        let cube_lut_resource = self.encode("cube", cube_str.into_bytes())?;

        let [cmy, k_arr] = cmyk_planes([0.2, 0.4, 0.6, 0.1]);
        let cmyk_back = planes_cmyk(cmy, k_arr[0]);

        let mut out = BTreeMap::new();
        out.insert("image".into(), image_handle);
        out.insert("scope_image".into(), scope_image);
        out.insert("cube_lut_resource".into(), cube_lut_resource);
        out.insert("graded_r".into(), Value::F64(blended[0] as f64));
        out.insert("graded_g".into(), Value::F64(blended[1] as f64));
        out.insert("graded_b".into(), Value::F64(blended[2] as f64));
        out.insert("waveform_bins".into(), Value::I64(wf.traces.len() as i64));
        out.insert("vectorscope_bins".into(), Value::I64(vs.data.len() as i64));
        out.insert("parade_bins".into(), Value::I64(pr.traces.len() as i64));
        out.insert("cmyk_roundtrip_len".into(), Value::I64(cmyk_back.len() as i64));
        Ok(Value::Record(out))
    }

    /// 130: `hypermedia.radiance.trace_vector_contours`
    /// (`vector-radiance-trace` + `raster-spatial-operators` + `raster-selection-morphology`)
    pub(crate) fn trace_vector_contours(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["image", "width", "height", "target_width"])?;

        let (width, height, rgba, mask_f32) = if let Some(v) = a.get("image") {
            let img = self.rgba(v)?;
            let w = img.width as usize;
            let h = img.height as usize;
            let mut raw_rgba = Vec::with_capacity(w * h * 4);
            let mut mask = Vec::with_capacity(w * h);
            for px in &img.pixels {
                raw_rgba.extend_from_slice(px);
                let luma = 0.2126 * (px[0] as f32 / 255.0)
                    + 0.7152 * (px[1] as f32 / 255.0)
                    + 0.0722 * (px[2] as f32 / 255.0);
                let active = if px[3] > 127 && luma < 0.85 { 1.0 } else { 0.0 };
                mask.push(active);
            }
            if mask.iter().all(|&m| m == 0.0) && !mask.is_empty() {
                mask[(h / 2) * w + (w / 2)] = 1.0;
            }
            (w, h, raw_rgba, mask)
        } else {
            let w = count(a, "width", 16, 128)?;
            let h = count(a, "height", 16, 128)?;
            let mut raw_rgba = vec![255u8; w * h * 4];
            let mut mask = vec![0.0f32; w * h];
            for y in (h / 4)..(h * 3 / 4) {
                for x in (w / 4)..(w * 3 / 4) {
                    let idx = (y * w + x) * 4;
                    raw_rgba[idx] = 20;
                    raw_rgba[idx + 1] = 40;
                    raw_rgba[idx + 2] = 180;
                    raw_rgba[idx + 3] = 255;
                    mask[y * w + x] = 1.0;
                }
            }
            (w, h, raw_rgba, mask)
        };
        let target_width = count(a, "target_width", 12.min(width), width)?;

        self.charge(width * height * 8, width * height * 16)?;

        use raster_selection_morphology::{distance_transform_f32, skeletonize_mask};
        use raster_spatial_operators::{octree_quantize_rgba8, seam_carve_rgba8};
        use std::fmt::Write as _;
        use vector_radiance_trace::{TraceParams, trace_rgba};

        let traced = trace_rgba(width as u32, height as u32, &rgba, &TraceParams::default())
            .map_err(|e| format!("Image trace failed: {e}"))?;
        let (carved, cw, ch) = seam_carve_rgba8(&rgba, width, height, target_width, height);
        let (quantized_flat, palette) = octree_quantize_rgba8(&carved, cw, ch, 4, true);
        let skeleton = skeletonize_mask(&mask_f32, width, height);
        let edt = distance_transform_f32(&mask_f32, width, height);
        let max_edt = edt.iter().copied().fold(0.0f32, f32::max);

        let quantized_pixels: Vec<[u8; 4]> = quantized_flat
            .chunks_exact(4)
            .map(|c| [c[0], c[1], c[2], c[3]])
            .collect();
        let carved_image = self.store_rgba(cw as u32, ch as u32, quantized_pixels)?;

        let norm_edt = max_edt.max(1.0);
        let skel_pixels: Vec<[u8; 4]> = (0..(width * height))
            .map(|i| {
                let s = skeleton[i].clamp(0.0, 1.0);
                let d = (edt[i] / norm_edt).clamp(0.0, 1.0);
                let r = (s * 255.0).round() as u8;
                let g = ((s * 0.8 + d * 0.5).clamp(0.0, 1.0) * 255.0).round() as u8;
                let b = (d * 220.0).round() as u8;
                [r, g, b, 255]
            })
            .collect();
        let skeleton_image = self.store_rgba(width as u32, height as u32, skel_pixels)?;

        let mut traced_svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\">\n"
        );
        for tp in &traced.paths {
            let bp = tp.path.to_bezpath();
            let [r, g, b] = tp.color;
            let _ = writeln!(
                traced_svg,
                "  <path d=\"{}\" fill=\"#{r:02x}{g:02x}{b:02x}\"/>",
                bp.to_svg()
            );
        }
        traced_svg.push_str("</svg>\n");
        let svg_resource = self.encode("svg", traced_svg.into_bytes())?;

        let mut out = BTreeMap::new();
        out.insert("carved_image".into(), carved_image);
        out.insert("skeleton_image".into(), skeleton_image);
        out.insert("svg_resource".into(), svg_resource);
        out.insert("traced_path_count".into(), Value::I64(traced.paths.len() as i64));
        out.insert("traced_anchor_count".into(), Value::I64(traced.anchor_count() as i64));
        out.insert("carved_width".into(), Value::I64(cw as i64));
        out.insert("carved_height".into(), Value::I64(ch as i64));
        out.insert("quantized_palette_size".into(), Value::I64(palette.len() as i64));
        out.insert("skeleton_active_pixels".into(), Value::I64(skeleton.iter().filter(|&&v| v > 0.0).count() as i64));
        out.insert("max_euclidean_distance".into(), Value::F64(max_edt as f64));
        Ok(Value::Record(out))
    }
}
