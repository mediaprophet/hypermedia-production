//! Bounded adapters: rendering, filters, compositing and material generation stay
//! with their library owners. Pixels are straight-alpha RGBA8 between operations.
//! 16-bit / 2k lattices are available; ruled / 8-bit / 1024 remain named profiles.
use super::geometry::color;
use super::*;
use anyrender::{ImageRenderer, PaintScene};
use qualia_core_db::hypermedia_authoring::image as owner;
use std::io::Cursor;

/// Origin morphism cap (named profiles `ruled`, `8-bit`, `1024`).
pub(crate) const LATTICE_ORIGIN_MAX: u32 = 1024;
/// Raised 2k lattice cap (named profile `2k`, and default when unprofiled).
pub(crate) const LATTICE_2K_MAX: u32 = 2048;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LatticeProfile {
    pub max_dim: u32,
    pub bit_depth: u8,
}

impl LatticeProfile {
    pub const ORIGIN: Self = Self {
        max_dim: LATTICE_ORIGIN_MAX,
        bit_depth: 8,
    };
    pub const RAISED: Self = Self {
        max_dim: LATTICE_2K_MAX,
        bit_depth: 8,
    };
    pub const SIXTEEN: Self = Self {
        max_dim: LATTICE_2K_MAX,
        bit_depth: 16,
    };

    /// Named origin profiles stay 8-bit; default/2k/16-bit admit 16-bit codecs.
    pub fn admits_16bit(self) -> bool {
        !matches!(self, Self::ORIGIN)
    }
}

/// Parse an optional named lattice profile. Default admits 2k / 8-bit.
pub(crate) fn lattice_profile(a: &Args) -> Result<LatticeProfile, String> {
    match a.get("profile").map(text).transpose()? {
        None => Ok(LatticeProfile::RAISED),
        Some("ruled") | Some("8-bit") | Some("8bit") | Some("1024") => Ok(LatticeProfile::ORIGIN),
        Some("16-bit") | Some("16bit") | Some("rgba16") => Ok(LatticeProfile::SIXTEEN),
        Some("2k") | Some("2048") => Ok(LatticeProfile::RAISED),
        Some(other) => Err(format!(
            "Unknown lattice profile '{other}'. Expected ruled, 8-bit, 1024, 16-bit, or 2k"
        )),
    }
}

#[inline]
pub(crate) fn u8_to_u16_lattice(v: u8) -> u16 {
    (v as u16) * 257
}

#[inline]
pub(crate) fn u16_to_u8_lattice(v: u16) -> u8 {
    (v >> 8) as u8
}

#[derive(Clone)]
pub struct Rgba {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<[u8; 4]>,
}
fn dimensions(a: &Args) -> Result<(u32, u32, usize), String> {
    let profile = lattice_profile(a)?;
    let w = count(a, "width", 256, profile.max_dim as usize)? as u32;
    let h = count(a, "height", 256, profile.max_dim as usize)? as u32;
    Ok((w, h, w as usize * h as usize))
}
fn signed(a: &Args, key: &str) -> Result<i32, String> {
    let n = number(a, key, 0.)?;
    if n.fract() != 0. || n.abs() > 1_000_000. {
        return Err(format!("Invalid integer {key}"));
    }
    Ok(n as i32)
}
impl AssetHost<'_> {
    pub(crate) fn rgba(&self, value: &Value) -> Result<&Rgba, String> {
        match self.workspace.get(value)? {
            Artifact::Rgba(r) => Ok(r),
            _ => Err("Expected rgba handle".into()),
        }
    }
    pub(crate) fn store_rgba(
        &mut self,
        width: u32,
        height: u32,
        pixels: Vec<[u8; 4]>,
    ) -> Result<Value, String> {
        self.workspace.insert(
            Artifact::Rgba(Rgba {
                width,
                height,
                pixels,
            }),
            self.limits,
        )
    }
    fn image_limits(&self) -> owner::ImageLimits {
        owner::ImageLimits {
            max_pixels: (LATTICE_2K_MAX as usize) * (LATTICE_2K_MAX as usize),
            max_work: self.limits.asset_work_units.saturating_sub(self.work) as usize,
            max_bytes: self.limits.total_resource_bytes,
        }
    }
    pub(crate) fn graphics(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.radiance.render_vector" | "hypermedia.vector.render" => {
                self.vector_render(a)
            }
            "hypermedia.radiance.create_rgba" | "hypermedia.rgba.create" => {
                fields(a, &["width", "height", "color", "profile"])?;
                let (w, h, n) = dimensions(a)?;
                self.charge(n, n * 4)?;
                let c = a.get("color").map(color).transpose()?.unwrap_or([0; 4]);
                let mut layer = owner::ImageLayer::new("", "", w, h);
                layer.fill(c);
                self.store_rgba(w, h, layer.pixels)
            }
            "hypermedia.radiance.compose_rgba" | "hypermedia.rgba.compose" => self.compose(a),
            "hypermedia.radiance.filter_rgba" | "hypermedia.rgba.filter" => self.filter(a),
            "hypermedia.radiance.encode_rgba_png" | "hypermedia.rgba.png" => {
                fields(a, &["image", "profile"])?;
                let r = self.rgba(required(a, "image")?)?;
                let n = r.pixels.len();
                let profile = lattice_profile(a)?;
                if r.width > profile.max_dim || r.height > profile.max_dim {
                    return Err(format!("PNG dimensions outside 1..{}", profile.max_dim));
                }
                self.charge(n * 10, n * 16 + 65536)?;
                let r = self.rgba(required(a, "image")?)?;
                let mut bytes = Vec::new();
                {
                    let mut encoder = png::Encoder::new(&mut bytes, r.width, r.height);
                    encoder.set_color(png::ColorType::Rgba);
                    if profile.bit_depth == 16 {
                        encoder.set_depth(png::BitDepth::Sixteen);
                        let mut packed = Vec::with_capacity(n * 8);
                        for p in &r.pixels {
                            for c in p {
                                // PNG 16-bit samples are BE; native endian swaps on LE hosts.
                                packed.extend_from_slice(&u8_to_u16_lattice(*c).to_be_bytes());
                            }
                        }
                        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
                        writer
                            .write_image_data(&packed)
                            .map_err(|e| e.to_string())?;
                    } else {
                        encoder.set_depth(png::BitDepth::Eight);
                        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
                        writer
                            .write_image_data(r.pixels.as_flattened())
                            .map_err(|e| e.to_string())?;
                    }
                }
                self.encode("png", bytes)
            }
            "hypermedia.radiance.import_rgba_png" | "hypermedia.rgba.import_png" => {
                self.import_rgba(a)
            }
            "hypermedia.radiance.pack_sprite_sheet" => self.pack_sprite_sheet(a),
            "hypermedia.radiance.pack_nine_slice" => self.pack_nine_slice(a),
            "hypermedia.surface.pack_pbr_set" => self.pack_pbr_set(a),
            "hypermedia.surface.synthesize_maps" | "hypermedia.surface.maps" => self.surface(a),
            _ => Err("Unknown graphics operation".into()),
        }
    }
    fn import_rgba(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["input", "profile"])?;
        let input_val = required(a, "input")?;
        let owned: Vec<u8> = if let Ok(key) = text(input_val) {
            self.inputs
                .iter()
                .find(|r| r.key == key)
                .ok_or("Missing retained input")?
                .bytes
                .clone()
        } else if let Ok(res) = self.workspace.encoded(input_val) {
            res.bytes.clone()
        } else {
            return Err("Expected retained resource key or encoded handle for input".into());
        };
        let b = owned.as_slice();
        let profile = lattice_profile(a)?;
        if b.len() < 33 || &b[..8] != b"\x89PNG\r\n\x1a\n" || &b[12..16] != b"IHDR" || b[24] > 16 {
            return Err("Expected at most 16-bit PNG".into());
        }
        if !profile.admits_16bit() && b[24] > 8 {
            return Err("Named 8-bit profile refuses 16-bit PNG".into());
        }
        let w = u32::from_be_bytes(b[16..20].try_into().unwrap());
        let h = u32::from_be_bytes(b[20..24].try_into().unwrap());
        if w == 0 || h == 0 || w > profile.max_dim || h > profile.max_dim {
            return Err(format!("PNG dimensions outside 1..{}", profile.max_dim));
        }
        let n = w as usize * h as usize;
        self.charge(n * 20, n * 16 + 65536)?;
        let mut decoder = png::Decoder::new_with_limits(
            Cursor::new(b),
            png::Limits {
                bytes: self.limits.total_resource_bytes,
            },
        );
        decoder.set_transformations(png::Transformations::EXPAND);
        let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
        if reader.info().animation_control.is_some() {
            return Err("Animated PNG requires a frame sequence importer".into());
        }
        let size = reader.output_buffer_size().ok_or("PNG size overflow")?;
        if size > n * 8 {
            return Err("PNG decoded size exceeds admission".into());
        }
        let mut bytes = vec![0; size];
        let info = reader.next_frame(&mut bytes).map_err(|e| e.to_string())?;
        if info.width != w || info.height != h {
            return Err("PNG format changed after admission".into());
        }
        let channels = info.color_type.samples();
        let pixels = match info.bit_depth {
            png::BitDepth::Sixteen => {
                let stride = channels * 2;
                bytes[..info.buffer_size()]
                    .chunks_exact(stride)
                    .map(|p| {
                        let sample = |i: usize| {
                            // IDENTITY leaves file-order BE bytes; high byte is the 8-bit lattice.
                            u16_to_u8_lattice(u16::from_be_bytes([p[i * 2], p[i * 2 + 1]]))
                        };
                        match channels {
                            1 => [sample(0), sample(0), sample(0), 255],
                            2 => [sample(0), sample(0), sample(0), sample(1)],
                            3 => [sample(0), sample(1), sample(2), 255],
                            _ => [sample(0), sample(1), sample(2), sample(3)],
                        }
                    })
                    .collect::<Vec<_>>()
            }
            png::BitDepth::Eight => bytes[..info.buffer_size()]
                .chunks_exact(channels)
                .map(|p| match channels {
                    1 => [p[0], p[0], p[0], 255],
                    2 => [p[0], p[0], p[0], p[1]],
                    3 => [p[0], p[1], p[2], 255],
                    _ => [p[0], p[1], p[2], p[3]],
                })
                .collect::<Vec<_>>(),
            _ => return Err("PNG format changed after admission".into()),
        };
        if pixels.len() != n {
            return Err("Invalid PNG pixel count".into());
        }
        self.store_rgba(w, h, pixels)
    }
    fn compose(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["width", "height", "layers", "profile"])?;
        let (w, h, n) = dimensions(a)?;
        let layers = list(required(a, "layers")?)?;
        if layers.len() > 64 {
            return Err("At most 64 layers".into());
        }
        // Check all handles and aggregate storage before cloning any pixel data.
        let mut visits = n;
        let mut bytes = n * 4;
        for v in layers {
            let l = record(v)?;
            fields(l, &["image", "opacity", "mode", "x", "y", "mask"])?;
            let r = self.rgba(required(l, "image")?)?;
            visits += r.pixels.len();
            bytes += r.pixels.len() * 4;
            if let Some(mask) = l.get("mask") {
                let m = self.rgba(mask)?;
                if (r.width, r.height) != (m.width, m.height) {
                    return Err("Mask dimensions must match layer".into());
                }
                bytes += m.pixels.len();
                visits += m.pixels.len();
            }
        }
        self.charge(visits * 4, bytes)?;
        let mut doc = owner::ImageDocument::new("", w, h);
        for v in layers {
            let l = record(v)?;
            let r = self.rgba(required(l, "image")?)?;
            let opacity = number(l, "opacity", 1.)?;
            if !(0. ..=1.).contains(&opacity) {
                return Err("Opacity outside 0..1".into());
            }
            use owner::BlendMode as B;
            let mode = match l.get("mode").map(text).transpose()?.unwrap_or("normal") {
                "normal" => B::Normal,
                "multiply" => B::Multiply,
                "screen" => B::Screen,
                "overlay" => B::Overlay,
                "soft-light" => B::SoftLight,
                "hard-light" => B::HardLight,
                "dodge" => B::ColorDodge,
                "burn" => B::ColorBurn,
                "difference" => B::Difference,
                "exclusion" => B::Exclusion,
                _ => return Err("Unknown layer blend mode".into()),
            };
            let mask = l
                .get("mask")
                .map(|m| {
                    self.rgba(m)
                        .map(|r| r.pixels.iter().map(|p| p[3]).collect())
                })
                .transpose()?;
            doc.layers.push(owner::ImageLayer {
                id: String::new(),
                name: String::new(),
                width: r.width,
                height: r.height,
                pixels: r.pixels.clone(),
                opacity: opacity as f32,
                blend_mode: mode,
                visible: true,
                offset_x: signed(l, "x")?,
                offset_y: signed(l, "y")?,
                mask,
            });
        }
        let mut out = vec![[0; 4]; n];
        // Work already charged; owner independently validates storage and parameters.
        let mut limits = self.image_limits();
        limits.max_work = visits;
        owner::composite_into(&doc, &mut out, limits).map_err(|e| e.to_string())?;
        self.store_rgba(w, h, out)
    }
    fn filter(&mut self, a: &Args) -> Result<Value, String> {
        let op = match text(required(a, "operation")?)? {
            "blur" => {
                fields(a, &["image", "operation", "sigma"])?;
                owner::ImageFilterOp::GaussianBlur {
                    sigma: number(a, "sigma", 1.)? as f32,
                }
            }
            "sharpen" => {
                fields(a, &["image", "operation", "sigma", "amount"])?;
                owner::ImageFilterOp::Sharpen {
                    sigma: number(a, "sigma", 1.)? as f32,
                    amount: number(a, "amount", 1.)? as f32,
                }
            }
            "hue" => {
                fields(a, &["image", "operation", "degrees"])?;
                owner::ImageFilterOp::HueRotate {
                    degrees: number(a, "degrees", 0.)? as f32,
                }
            }
            "emboss" => {
                fields(a, &["image", "operation", "azimuth", "strength"])?;
                owner::ImageFilterOp::Emboss {
                    azimuth_degrees: number(a, "azimuth", 45.)? as f32,
                    strength: number(a, "strength", 1.)? as f32,
                }
            }
            _ => return Err("Unknown filter".into()),
        };
        let r = self.rgba(required(a, "image")?)?;
        let (w, h, n) = (r.width, r.height, r.pixels.len());
        let limits = self.image_limits();
        let plan = owner::filter_plan(w, h, op, limits).map_err(|e| e.to_string())?;
        self.charge(plan.work, plan.bytes)?;
        let mut out = vec![[0; 4]; n];
        let mut scratch = vec![[0.; 4]; plan.scratch_pixels];
        owner::filter_rgba8_into(
            &self.rgba(required(a, "image")?)?.pixels,
            w,
            h,
            op,
            limits,
            &mut out,
            &mut scratch,
        )
        .map_err(|e| e.to_string())?;
        self.store_rgba(w, h, out)
    }
    fn surface(&mut self, a: &Args) -> Result<Value, String> {
        use qualia_core_db::render::surface_authoring::{
            generate_surface_maps_into, SurfaceRecipe,
        };
        fields(
            a,
            &[
                "side",
                "seed",
                "tint",
                "contrast",
                "relief",
                "bands",
                "band_strength",
                "roughness",
                "metallic",
            ],
        )?;
        let side = count(a, "side", 128, 1024)? as u32;
        if side < 8 {
            return Err("Surface side must be 8..1024".into());
        }
        let seed = a
            .get("seed")
            .map(|v| integer(v, "seed"))
            .transpose()?
            .unwrap_or(0);
        let bands = a
            .get("bands")
            .map(vector::<2>)
            .transpose()?
            .unwrap_or([12., 1.]);
        if seed > u32::MAX as usize
            || bands
                .iter()
                .any(|v| v.fract() != 0. || !(0. ..=1024.).contains(v))
        {
            return Err("Invalid surface seed or bands".into());
        }
        let r = SurfaceRecipe {
            seed: seed as u32,
            tint: a
                .get("tint")
                .map(vector::<3>)
                .transpose()?
                .unwrap_or([1.; 3])
                .map(|v| v as f32),
            contrast: number(a, "contrast", 0.2)? as f32,
            relief: number(a, "relief", 0.002)? as f32,
            bands: bands.map(|v| v as u32),
            band_strength: number(a, "band_strength", 0.7)? as f32,
            roughness: number(a, "roughness", 0.8)? as f32,
            metallic: number(a, "metallic", 0.)? as f32,
        };
        let n = side as usize * side as usize;
        let dest_pixels = n
            .checked_mul(3)
            .ok_or("synthesize_maps pixel overflow")?;
        let scratch = dest_pixels.saturating_mul(4);
        if self.work.saturating_add(dest_pixels as u64) > self.limits.asset_work_units
            || scratch > self.limits.total_resource_bytes
        {
            return Err(format!(
                "synthesize_maps {side}²×3 maps exceed envelope (work {dest_pixels}, scratch {scratch}; budget work {} scratch {})",
                self.limits.asset_work_units, self.limits.total_resource_bytes
            ));
        }
        self.charge(dest_pixels, scratch)?;
        let mut colour = vec![0; n * 4];
        let mut normal = vec![0; n * 4];
        let mut orm = vec![0; n * 4];
        generate_surface_maps_into(r, side, &mut colour, &mut normal, &mut orm)
            .map_err(|e| format!("Surface: {e:?}"))?;
        let mut result = BTreeMap::new();
        for (name, bytes) in [("colour", colour), ("normal", normal), ("orm", orm)] {
            result.insert(
                name.into(),
                self.store_rgba(
                    side,
                    side,
                    bytes
                        .chunks_exact(4)
                        .map(|p| [p[0], p[1], p[2], p[3]])
                        .collect(),
                )?,
            );
        }
        Ok(Value::Record(result))
    }
}

struct Draw {
    path: kurbo::BezPath,
    transform: kurbo::Affine,
    fill: Option<peniko::Brush>,
    stroke: Option<peniko::Color>,
    width: f64,
    clip: Option<kurbo::Rect>,
}
fn point(v: &Value) -> Result<kurbo::Point, String> {
    let p = vector::<2>(v)?;
    if p.iter().any(|v| v.abs() > 1e6) {
        return Err("Point outside render bounds".into());
    }
    Ok(kurbo::Point::new(p[0], p[1]))
}
fn paint(v: &Value) -> Result<peniko::Brush, String> {
    if let Value::Record(a) = v {
        fields(a, &["start", "end", "colors"])?;
        let start = point(required(a, "start")?)?;
        let end = point(required(a, "end")?)?;
        if start == end {
            return Err("Gradient endpoints must differ".into());
        }
        let stops = list(required(a, "colors")?)?;
        if !(2..=16).contains(&stops.len()) {
            return Err("Gradient needs 2..16 colors".into());
        }
        let colors = stops
            .iter()
            .map(|c| color(c).map(|c| peniko::Color::from_rgba8(c[0], c[1], c[2], c[3])))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(peniko::Gradient::new_linear(start, end)
            .with_stops(colors.as_slice())
            .into())
    } else {
        let c = color(v)?;
        Ok(peniko::Color::from_rgba8(c[0], c[1], c[2], c[3]).into())
    }
}
impl AssetHost<'_> {
    fn vector_render(&mut self, a: &Args) -> Result<Value, String> {
        use kurbo::Shape;
        fields(a, &["width", "height", "layers", "profile"])?;
        let (w, h, n) = dimensions(a)?;
        let layers = list(required(a, "layers")?)?;
        if layers.len() > 64 {
            return Err("At most 64 vector layers".into());
        }
        self.charge(n * (layers.len() + 1) * 4, n * 48)?;
        let mut draws = Vec::new();
        for layer in layers {
            let l = record(layer)?;
            fields(
                l,
                &["path", "fill", "stroke", "stroke_width", "matrix", "clip"],
            )?;
            let data = text(required(l, "path")?)?;
            if data.len() > 65536 {
                return Err("Path bytes exceed 65536".into());
            }
            self.charge(data.len() * 16, data.len() * 64)?;
            let path = kurbo::BezPath::from_svg(data).map_err(|e| format!("SVG path: {e:?}"))?;
            if path.elements().is_empty() || path.elements().len() > 4096 {
                return Err("Path needs 1..4096 elements".into());
            }
            let matrix = l
                .get("matrix")
                .map(vector::<6>)
                .transpose()?
                .unwrap_or([1., 0., 0., 1., 0., 0.]);
            if matrix.iter().any(|v| v.abs() > 1e6) {
                return Err("Transform outside bounds".into());
            }
            let transform = kurbo::Affine::new(matrix);
            // Include controls, not just curve extrema, in numeric admission.
            for el in path.elements() {
                let pts: Vec<kurbo::Point> = match el {
                    kurbo::PathEl::MoveTo(p) | kurbo::PathEl::LineTo(p) => vec![*p],
                    kurbo::PathEl::QuadTo(a, b) => vec![*a, *b],
                    kurbo::PathEl::CurveTo(a, b, c) => vec![*a, *b, *c],
                    kurbo::PathEl::ClosePath => vec![],
                };
                for p in pts {
                    let q = transform * p;
                    if !p.is_finite()
                        || !q.is_finite()
                        || p.x.abs() > 1e6
                        || p.y.abs() > 1e6
                        || q.x.abs() > 1e6
                        || q.y.abs() > 1e6
                    {
                        return Err("Path coordinates outside bounds".into());
                    }
                }
            }
            let width = number(l, "stroke_width", 1.)?;
            if !(0. ..=1024.).contains(&width) {
                return Err("Stroke width outside 0..1024".into());
            }
            let clip = l
                .get("clip")
                .map(vector::<4>)
                .transpose()?
                .map(|v| kurbo::Rect::new(v[0], v[1], v[2], v[3]));
            if clip.is_some_and(|r| {
                r.width() <= 0.
                    || r.height() <= 0.
                    || [r.x0, r.y0, r.x1, r.y1].iter().any(|v| v.abs() > 1e6)
            }) {
                return Err("Invalid clip rectangle".into());
            }
            let fill = l.get("fill").map(paint).transpose()?;
            let stroke = l
                .get("stroke")
                .map(color)
                .transpose()?
                .map(|c| peniko::Color::from_rgba8(c[0], c[1], c[2], c[3]));
            if fill.is_none() && stroke.is_none() {
                return Err("Path requires fill or stroke".into());
            }
            let _ = path.bounding_box();
            draws.push(Draw {
                path,
                transform,
                fill,
                stroke,
                width,
                clip,
            });
        }
        let mut bytes = vec![0; n * 4];
        let mut renderer = vector_render::VelloCpuImageRenderer::new(w, h);
        renderer.render(
            |scene| {
                for d in &draws {
                    if let Some(clip) = d.clip {
                        scene.push_clip_layer(
                            peniko::Fill::NonZero,
                            kurbo::Affine::IDENTITY,
                            &clip,
                        );
                    }
                    if let Some(fill) = &d.fill {
                        match fill {
                            peniko::Brush::Solid(c) => {
                                scene.fill(peniko::Fill::NonZero, d.transform, *c, None, &d.path);
                            }
                            peniko::Brush::Gradient(g) => {
                                scene.fill(peniko::Fill::NonZero, d.transform, g, None, &d.path);
                            }
                            peniko::Brush::Image(_) => {}
                        }
                    }
                    if let Some(stroke) = d.stroke {
                        scene.stroke(
                            &kurbo::Stroke::new(d.width),
                            d.transform,
                            stroke,
                            None,
                            &d.path,
                        );
                    }
                    if d.clip.is_some() {
                        scene.pop_layer();
                    }
                }
            },
            &mut bytes,
        );
        // Vello's render buffer is premultiplied; PNG and the Qualia compositor
        // consume straight alpha. Avoid dark halos in translucent exports.
        let pixels = bytes
            .chunks_exact(4)
            .map(|p| {
                let alpha = p[3] as u32;
                if alpha == 0 {
                    [0; 4]
                } else {
                    [
                        (p[0] as u32 * 255 + alpha / 2)
                            .checked_div(alpha)
                            .unwrap()
                            .min(255) as u8,
                        (p[1] as u32 * 255 + alpha / 2)
                            .checked_div(alpha)
                            .unwrap()
                            .min(255) as u8,
                        (p[2] as u32 * 255 + alpha / 2)
                            .checked_div(alpha)
                            .unwrap()
                            .min(255) as u8,
                        p[3],
                    ]
                }
            })
            .collect();
        self.store_rgba(w, h, pixels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::atomic::AtomicBool;
    use vibe::Value;

    fn host<'a>(limits: &'a ToolLimits, cancel: &'a AtomicBool) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits,
            cancel,
            work: 0,
        }
    }

    #[test]
    fn test_radiance_png_psd_lattice_roundtrip_nontrivial() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut h = host(&limits, &cancel);
        let pixel = [0x12, 0x34, 0x56, 0xab];
        let img = h
            .store_rgba(2, 2, vec![pixel, [0, 0, 0, 255], pixel, [9, 8, 7, 6]])
            .expect("store");

        let png = h
            .graphics(
                "hypermedia.radiance.encode_rgba_png",
                &BTreeMap::from([
                    ("image".into(), img.clone()),
                    ("profile".into(), Value::String("16-bit".into())),
                ]),
            )
            .expect("encode 16-bit png");
        let png_bytes = h.workspace.encoded(&png).expect("png resource").bytes.clone();
        assert_eq!(png_bytes[24], 16, "IHDR bit depth must be 16");
        let refused = h.graphics(
            "hypermedia.radiance.import_rgba_png",
            &BTreeMap::from([
                ("input".into(), png.clone()),
                ("profile".into(), Value::String("8-bit".into())),
            ]),
        );
        assert!(
            refused.is_err(),
            "named 8-bit profile must refuse 16-bit PNG"
        );
        let imported = h
            .graphics(
                "hypermedia.radiance.import_rgba_png",
                &BTreeMap::from([("input".into(), png)]),
            )
            .expect("import 16-bit png");
        let got = h.rgba(&imported).expect("rgba");
        assert_eq!(got.pixels[0], pixel);
        assert_eq!(got.pixels[2], pixel);

        let psd = h
            .radiance_photo_op(
                "hypermedia.radiance.encode_psd",
                &BTreeMap::from([
                    ("width".into(), Value::U64(2)),
                    ("height".into(), Value::U64(2)),
                    ("profile".into(), Value::String("16-bit".into())),
                    (
                        "layers".into(),
                        Value::List(vec![Value::Record(BTreeMap::from([
                            ("name".into(), Value::String("Hero".into())),
                            ("image".into(), img),
                            ("opacity".into(), Value::F64(1.0)),
                        ]))]),
                    ),
                ]),
            )
            .expect("encode 16-bit psd");
        let psd_bytes = h.workspace.encoded(&psd).expect("psd resource").bytes.clone();
        let depth = u16::from_be_bytes(psd_bytes[22..24].try_into().unwrap());
        assert_eq!(depth, 16, "PSD header depth must be 16");
        let refused_psd = h.radiance_photo_op(
            "hypermedia.radiance.import_psd",
            &BTreeMap::from([
                ("input".into(), psd.clone()),
                ("layer_index".into(), Value::U64(0)),
                ("profile".into(), Value::String("ruled".into())),
            ]),
        );
        assert!(
            refused_psd.is_err(),
            "named ruled profile must refuse 16-bit PSD"
        );
        let layer = h
            .radiance_photo_op(
                "hypermedia.radiance.import_psd",
                &BTreeMap::from([
                    ("input".into(), psd),
                    ("layer_index".into(), Value::U64(0)),
                ]),
            )
            .expect("import 16-bit psd layer");
        let layer_rgba = h.rgba(&layer).expect("layer rgba");
        assert_eq!(layer_rgba.pixels[0], pixel);
        assert_eq!(layer_rgba.pixels[2], pixel);
    }

    #[test]
    fn test_radiance_2k_lattice_and_named_1024_profile() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut h = host(&limits, &cancel);
        let two_k = h
            .graphics(
                "hypermedia.radiance.create_rgba",
                &BTreeMap::from([
                    ("width".into(), Value::U64(1280)),
                    ("height".into(), Value::U64(2)),
                    ("color".into(), Value::List(vec![
                        Value::U64(18),
                        Value::U64(52),
                        Value::U64(86),
                        Value::U64(255),
                    ])),
                    ("profile".into(), Value::String("2k".into())),
                ]),
            )
            .expect("2k lattice");
        let img = h.rgba(&two_k).expect("rgba");
        assert_eq!(img.width, 1280);
        assert_eq!(img.height, 2);
        assert_eq!(img.pixels[0], [18, 52, 86, 255]);

        let err = h
            .graphics(
                "hypermedia.radiance.create_rgba",
                &BTreeMap::from([
                    ("width".into(), Value::U64(1280)),
                    ("height".into(), Value::U64(2)),
                    ("profile".into(), Value::String("1024".into())),
                ]),
            )
            .expect_err("named 1024 profile must refuse 2k");
        assert!(err.contains("1280") || err.contains("1024") || err.contains("width"));
    }

    #[test]
    fn test_radiance_png_16bit_be_high_byte() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut h = host(&limits, &cancel);
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut bytes, 1, 1);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Sixteen);
            // 0x12FF is not 0xVVVV; BE high byte must survive import on LE hosts.
            let packed = [0x12u8, 0xFF, 0x34, 0xAA, 0x56, 0xBB, 0xAB, 0xCD];
            let mut writer = encoder.write_header().expect("png header");
            writer.write_image_data(&packed).expect("png data");
        }
        let png = h.admit_encoded("png", bytes).expect("admit be png");
        let imported = h
            .graphics(
                "hypermedia.radiance.import_rgba_png",
                &BTreeMap::from([("input".into(), png)]),
            )
            .expect("import 16-bit be png");
        let got = h.rgba(&imported).expect("rgba");
        assert_eq!(got.pixels[0][0], 0x12, "BE sample 0x12FF must keep high byte 0x12");
        assert_eq!(got.pixels[0], [0x12, 0x34, 0x56, 0xab]);
    }
}
