use super::*;
use std::collections::BTreeMap;
use vibe::Value;

fn opt_str(a: &Args, k: &str, default: &str) -> Result<String, String> {
    match a.get(k) {
        None => Ok(default.to_string()),
        Some(v) => Ok(text(v)?.to_string()),
    }
}

impl AssetHost<'_> {
    /// 126: `hypermedia.surface.bake_pbr_materialx`
    /// (`surface-pbr-studio` + `surface-layered-codecs`)
    pub(crate) fn bake_pbr_materialx(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "resolution", "samples", "material_name", "uvs"])?;
        let resolution = count(a, "resolution", 32, 256)? as u32;
        let samples = count(a, "samples", 4, 64)? as u32;
        let material_name = opt_str(a, "material_name", "Architectural_Bronze")?;

        self.charge((resolution * resolution * samples) as usize, 32768)?;

        use surface_layered_codecs::{PsdColorMode, PsdDocument, PsdVersion};
        use surface_pbr_studio::{
            BakeMapKind, BakeSettings, Bvh, MaterialXOptions, Mesh as PbrMesh, NormalConvention,
            PbrChannel, ShaderModel, Vec2 as PbrVec2, Vec3 as PbrVec3, Vec4 as PbrVec4, bake_map,
            generate_materialx_document,
        };

        let pbr_mesh = if let Some(v) = a.get("mesh") {
            let Artifact::Mesh(src) = self.workspace.get(v)? else {
                return Err("Expected mesh handle".into());
            };
            let span_x = (src.max[0] - src.min[0]).max(1e-4);
            let span_y = (src.max[1] - src.min[1]).max(1e-4);
            let positions: Vec<PbrVec3> = src
                .positions
                .iter()
                .map(|p| PbrVec3::new(p[0], p[1], p[2]))
                .collect();
            // Prefer SLIM UVs from unwrap; planar XY is only the no-unwrap fallback.
            let uvs: Vec<PbrVec2> = if let Some(v) = a.get("uvs") {
                crate::surface_bsdf_uv::parse_vertex_uvs(self, v, src.positions.len())?
                    .into_iter()
                    .map(|uv| PbrVec2::new(uv[0] as f32, uv[1] as f32))
                    .collect()
            } else {
                src.positions
                    .iter()
                    .map(|p| {
                        PbrVec2::new(
                            ((p[0] - src.min[0]) / span_x).clamp(0.0, 1.0),
                            ((p[1] - src.min[1]) / span_y).clamp(0.0, 1.0),
                        )
                    })
                    .collect()
            };
            let indices: Vec<u32> = src.triangles.iter().flatten().copied().collect();
            let mut m = PbrMesh {
                positions,
                normals: Vec::new(),
                tangents: vec![PbrVec4::new(1.0, 0.0, 0.0, 1.0); src.positions.len()],
                uvs,
                indices,
                submeshes: Vec::new(),
            };
            m.compute_normals();
            m
        } else {
            PbrMesh::create_quad(2.0)
        };

        let bvh = Bvh::build(&pbr_mesh);
        let settings = BakeSettings {
            resolution,
            samples,
            max_distance: 0.2,
            ao_distance: 0.25,
            seed: 42,
        };
        let ao_map = bake_map(&pbr_mesh, &bvh, BakeMapKind::AmbientOcclusion, &settings);
        let norm_map = bake_map(&pbr_mesh, &bvh, BakeMapKind::WorldNormal, &settings);

        let to_u8 = |c: f32| (c.clamp(0.0, 1.0) * 255.0).round() as u8;
        let ao_pixels: Vec<[u8; 4]> = ao_map
            .pixels
            .iter()
            .map(|p| [to_u8(p[0]), to_u8(p[1]), to_u8(p[2]), to_u8(p[3].max(1.0))])
            .collect();
        let norm_pixels: Vec<[u8; 4]> = norm_map
            .pixels
            .iter()
            .map(|p| [to_u8(p[0]), to_u8(p[1]), to_u8(p[2]), to_u8(p[3].max(1.0))])
            .collect();
        let ao_image = self.store_rgba(resolution, resolution, ao_pixels)?;
        let normal_image = self.store_rgba(resolution, resolution, norm_pixels)?;

        let mut maps = BTreeMap::new();
        maps.insert(PbrChannel::BaseColor, "bronze_albedo.png".to_string());
        maps.insert(PbrChannel::Normal, "bronze_normal.png".to_string());
        maps.insert(PbrChannel::AO, "bronze_ao.png".to_string());

        let mtlx_opts = MaterialXOptions {
            shader: ShaderModel::OpenPbr,
            normal: NormalConvention::OpenGl,
            tiling: [1.0, 1.0],
            displacement: false,
            displacement_scale: 1.0,
        };
        let mtlx_xml = generate_materialx_document(&material_name, &maps, &mtlx_opts);
        let mtlx_handle = self.encode("mtlx", mtlx_xml.into_bytes())?;

        let psd_doc = PsdDocument::new(resolution, resolution, PsdColorMode::Rgb, false);
        let psd_bytes = psd_doc.encode();
        let psd_decoded = PsdDocument::decode(&psd_bytes)
            .map_err(|e| format!("PSD roundtrip failed: {e}"))?;
        let psd_resource = self.encode("psd", psd_bytes)?;

        let mut out = BTreeMap::new();
        out.insert("materialx_resource".into(), mtlx_handle);
        out.insert("psd_resource".into(), psd_resource);
        out.insert("ao_image".into(), ao_image);
        out.insert("normal_image".into(), normal_image);
        out.insert("bake_resolution".into(), Value::I64(resolution as i64));
        out.insert("ao_pixel_count".into(), Value::I64((ao_map.width * ao_map.height) as i64));
        out.insert("normal_z_center".into(), Value::F64(norm_map.pixel(resolution / 2, resolution / 2)[2] as f64));
        out.insert("psd_roundtrip_valid".into(), Value::Bool(psd_decoded.header.version == PsdVersion::Psd));
        out.insert("used_slim_uvs".into(), Value::Bool(a.get("uvs").is_some()));
        Ok(Value::Record(out))
    }

    /// 127: `hypermedia.surface.expand_procedural_brush`
    /// (`vector-surface-brush` + `raster-layer-mask-adapter`)
    pub(crate) fn expand_procedural_brush(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "stroke_weight", "bristle_count", "wetness"])?;
        let weight = number(a, "stroke_weight", 8.0)?;
        let bristle_count = count(a, "bristle_count", 12, 64)? as f64;
        let wetness = number(a, "wetness", 0.6)? as f32;

        self.charge(256, 8192)?;

        use raster_layer_mask_adapter::{MixerSettings, MixerState, mix_dab};
        use std::fmt::Write as _;
        use vector_locus_geometry::{BezPath, Point, Shape};
        use vector_surface_brush::{BristleBrush, Brush, CalligraphicBrush, expand_brush_stroke};

        let mut path = BezPath::new();
        if let Some(v) = a.get("profile") {
            let pts = self.profile(v)?;
            if let Some((first, rest)) = pts.split_first() {
                path.move_to(Point::new(first[0], first[1]));
                for p in rest {
                    path.line_to(Point::new(p[0], p[1]));
                }
            }
        } else {
            path.move_to(Point::new(0.0, 0.0));
            path.curve_to(
                Point::new(30.0, 20.0),
                Point::new(60.0, -20.0),
                Point::new(100.0, 0.0),
            );
        }

        let calli = Brush::Calligraphic(CalligraphicBrush::default());
        let calli_pieces = expand_brush_stroke(&calli, &path, weight, None);

        let mut bristle_cfg = BristleBrush::default();
        bristle_cfg.density = bristle_count;
        let bristle_pieces = expand_brush_stroke(&Brush::Bristle(bristle_cfg), &path, weight, None);

        let mut mixer = MixerState::new();
        mixer.reservoir = Some([0.9, 0.4, 0.1, 1.0]);
        let settings = MixerSettings {
            wet: wetness.clamp(0.0, 1.0),
            load: 0.8,
            mix: 0.5,
            flow: 1.0,
            sample_all_layers: false,
        };
        let mixed_color = mix_dab([0.2, 0.3, 0.8, 1.0], &settings, &mut mixer);

        // Extract a closed 2D outline polygon from the expanded calligraphic stroke
        let mut outline_pts: Vec<[f64; 2]> = calli_pieces
            .first()
            .and_then(|piece| piece.path.subpaths.first())
            .map(|sp| sp.anchors.iter().map(|a| [a.p.x, a.p.y]).collect())
            .unwrap_or_default();
        let poly_area = {
            let mut s = 0.0;
            for i in 0..outline_pts.len() {
                let p0 = outline_pts[i];
                let p1 = outline_pts[(i + 1) % outline_pts.len()];
                s += p0[0] * p1[1] - p1[0] * p0[1];
            }
            0.5 * s.abs()
        };
        if outline_pts.len() < 3 || poly_area < 1e-6 {
            let hw = weight.abs().max(1.0) * 0.5;
            outline_pts = vec![
                [0.0, -hw],
                [100.0, -hw],
                [100.0, hw],
                [0.0, hw],
            ];
        }
        let profile_handle = self.store_profile(outline_pts)?;

        let r_u8 = (mixed_color[0].clamp(0.0, 1.0) * 255.0).round() as u8;
        let g_u8 = (mixed_color[1].clamp(0.0, 1.0) * 255.0).round() as u8;
        let b_u8 = (mixed_color[2].clamp(0.0, 1.0) * 255.0).round() as u8;

        let bbox = path.bounding_box();
        let pad = weight.abs().max(4.0) * 2.0;
        let vx0 = bbox.x0 - pad;
        let vy0 = bbox.y0 - pad;
        let vw = (bbox.width() + pad * 2.0).max(20.0);
        let vh = (bbox.height() + pad * 2.0).max(20.0);

        let mut svg_xml = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{vx0:.2} {vy0:.2} {vw:.2} {vh:.2}\">\n"
        );
        for piece in &calli_pieces {
            let bp = piece.path.to_bezpath();
            let _ = writeln!(
                svg_xml,
                "  <path d=\"{}\" fill=\"#{r_u8:02x}{g_u8:02x}{b_u8:02x}\" fill-opacity=\"{:.2}\"/>",
                bp.to_svg(),
                piece.opacity.clamp(0.0, 1.0)
            );
        }
        for strand in &bristle_pieces {
            let bp = strand.path.to_bezpath();
            let _ = writeln!(
                svg_xml,
                "  <path d=\"{}\" fill=\"none\" stroke=\"#{r_u8:02x}{g_u8:02x}{b_u8:02x}\" stroke-width=\"0.75\" stroke-opacity=\"{:.2}\"/>",
                bp.to_svg(),
                strand.opacity.clamp(0.0, 1.0)
            );
        }
        svg_xml.push_str("</svg>\n");
        let svg_resource = self.encode("svg", svg_xml.into_bytes())?;

        // Rasterize a 64x32 preview image of the expanded brush stroke
        let pw = 64u32;
        let ph = 32u32;
        let mut preview_pixels = vec![[248u8, 245, 240, 255]; (pw * ph) as usize];
        for piece in calli_pieces.iter().chain(bristle_pieces.iter()) {
            for sp in &piece.path.subpaths {
                for a in &sp.anchors {
                    let nx = ((a.p.x - vx0) / vw).clamp(0.0, 0.999);
                    let ny = ((a.p.y - vy0) / vh).clamp(0.0, 0.999);
                    let px = (nx * pw as f64) as u32;
                    let py = (ny * ph as f64) as u32;
                    let idx = (py * pw + px) as usize;
                    preview_pixels[idx] = [r_u8, g_u8, b_u8, 255];
                }
            }
        }
        let image_handle = self.store_rgba(pw, ph, preview_pixels)?;

        let mut out = BTreeMap::new();
        out.insert("profile".into(), profile_handle);
        out.insert("svg_resource".into(), svg_resource);
        out.insert("image".into(), image_handle);
        out.insert("calligraphic_piece_count".into(), Value::I64(calli_pieces.len() as i64));
        out.insert("bristle_strand_count".into(), Value::I64(bristle_pieces.len() as i64));
        out.insert("mixer_reservoir_r".into(), Value::F64(mixed_color[0] as f64));
        Ok(Value::Record(out))
    }
}
