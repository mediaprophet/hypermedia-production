//! Bounded adapters: rendering, filters, compositing and material generation stay
//! with their library owners. Pixels are straight-alpha RGBA8 between operations.
use super::geometry::color;
use super::*;
use anyrender::{ImageRenderer, PaintScene};
use qualia_core_db::hypermedia_authoring::image as owner;
use std::io::Cursor;

#[derive(Clone)]
pub struct Rgba {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<[u8; 4]>,
}
fn dimensions(a: &Args) -> Result<(u32, u32, usize), String> {
    let w = count(a, "width", 256, 1024)? as u32;
    let h = count(a, "height", 256, 1024)? as u32;
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
            max_pixels: 1024 * 1024,
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
                fields(a, &["width", "height", "color"])?;
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
                fields(a, &["image"])?;
                let r = self.rgba(required(a, "image")?)?;
                let n = r.pixels.len();
                self.charge(n * 10, n * 16 + 65536)?;
                let r = self.rgba(required(a, "image")?)?;
                let mut bytes = Vec::new();
                {
                    let mut encoder = png::Encoder::new(&mut bytes, r.width, r.height);
                    encoder.set_color(png::ColorType::Rgba);
                    encoder.set_depth(png::BitDepth::Eight);
                    let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
                    writer
                        .write_image_data(r.pixels.as_flattened())
                        .map_err(|e| e.to_string())?;
                }
                self.encode("png", bytes)
            }
            "hypermedia.radiance.import_rgba_png" | "hypermedia.rgba.import_png" => {
                self.import_rgba(a)
            }
            "hypermedia.surface.synthesize_maps" | "hypermedia.surface.maps" => self.surface(a),
            _ => Err("Unknown graphics operation".into()),
        }
    }
    fn import_rgba(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["input"])?;
        let key = text(required(a, "input")?)?;
        let b = &self
            .inputs
            .iter()
            .find(|r| r.key == key)
            .ok_or("Missing retained input")?
            .bytes;
        if b.len() < 33 || &b[..8] != b"\x89PNG\r\n\x1a\n" || &b[12..16] != b"IHDR" || b[24] > 8 {
            return Err("Expected at most 8-bit PNG".into());
        }
        let w = u32::from_be_bytes(b[16..20].try_into().unwrap());
        let h = u32::from_be_bytes(b[20..24].try_into().unwrap());
        if w == 0 || h == 0 || w > 1024 || h > 1024 {
            return Err("PNG dimensions outside 1..1024".into());
        }
        let n = w as usize * h as usize;
        self.charge(n * 20, n * 16 + 65536)?;
        let b = &self.inputs.iter().find(|r| r.key == key).unwrap().bytes;
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
        if size > n * 4 {
            return Err("PNG decoded size exceeds admission".into());
        }
        let mut bytes = vec![0; size];
        let info = reader.next_frame(&mut bytes).map_err(|e| e.to_string())?;
        if info.width != w || info.height != h || info.bit_depth != png::BitDepth::Eight {
            return Err("PNG format changed after admission".into());
        }
        let channels = info.color_type.samples();
        let pixels = bytes[..info.buffer_size()]
            .chunks_exact(channels)
            .map(|p| match channels {
                1 => [p[0], p[0], p[0], 255],
                2 => [p[0], p[0], p[0], p[1]],
                3 => [p[0], p[1], p[2], 255],
                _ => [p[0], p[1], p[2], p[3]],
            })
            .collect::<Vec<_>>();
        if pixels.len() != n {
            return Err("Invalid PNG pixel count".into());
        }
        self.store_rgba(w, h, pixels)
    }
    fn compose(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["width", "height", "layers"])?;
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
        let side = count(a, "side", 128, 512)? as u32;
        if side < 8 {
            return Err("Surface side must be 8..512".into());
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
        self.charge(n * 128, n * 24)?;
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
        fields(a, &["width", "height", "layers"])?;
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
