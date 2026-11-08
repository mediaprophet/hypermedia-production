use super::geometry::color;
use super::*;
use anyrender::{ImageRenderer, PaintScene};
use qualia_core_db::specialized_libs::computer_vision::cv;
use raster_selection_morphology as morphology;

impl AssetHost<'_> {
    pub(super) fn raster(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        if id == "hypermedia.radiance.rasterize_loci" || id == "hypermedia.image.render" {
            return self.render(a);
        }
        if id == "hypermedia.radiance.import_rgb_png" || id == "hypermedia.image.import_png" {
            fields(a, &["input"])?;
            let key = text(required(a, "input")?)?;
            let input = self
                .inputs
                .iter()
                .find(|r| r.key == key)
                .ok_or("Missing retained PNG input")?;
            // Preflight IHDR before the owner's decoder allocates its output. This
            // initial contract admits opaque RGB8, with no transparency conversion.
            let b = &input.bytes;
            if b.len() < 33
                || &b[..8] != b"\x89PNG\r\n\x1a\n"
                || b[8..12] != 13u32.to_be_bytes()
                || &b[12..16] != b"IHDR"
                || b[24] != 8
                || b[25] != 2
            {
                return Err("PNG import requires RGB8 PNG".into());
            }
            let width = u32::from_be_bytes(b[16..20].try_into().unwrap());
            let height = u32::from_be_bytes(b[20..24].try_into().unwrap());
            let n = pixels(width, height)?;
            let mut pos = 8usize;
            while pos + 12 <= b.len() {
                let len = u32::from_be_bytes(b[pos..pos + 4].try_into().unwrap()) as usize;
                let end = pos
                    .checked_add(12)
                    .and_then(|v| v.checked_add(len))
                    .ok_or("PNG chunk overflow")?;
                if end > b.len() {
                    return Err("Truncated PNG chunk".into());
                }
                if &b[pos + 4..pos + 8] == b"tRNS" {
                    return Err(
                        "Transparent PNG import is not supported by the RGB contract".into(),
                    );
                }
                pos = end;
            }
            self.charge(n * 10, n * 16)?;
            let (rgb, w, h) =
                cv::codecs::png_decode::decode_png(b).map_err(|e| format!("PNG decode: {e:?}"))?;
            if w != width || h != height || rgb.len() != n * 3 {
                return Err("PNG decoded dimensions mismatch".into());
            }
            return self
                .workspace
                .insert(Artifact::Image(Raster { width, height, rgb }), self.limits);
        }
        let image = self.image(required(a, "image")?)?;
        let (width, height) = (image.width, image.height);
        let n = pixels(width, height)?;
        let (work, scratch) = match id {
            "hypermedia.radiance.convolve_blur" | "hypermedia.image.blur" => (n * 36, n * 8),
            "hypermedia.radiance.morphology_mask" | "hypermedia.image.mask" => (n * 60, n * 40),
            "hypermedia.radiance.encode_rgb_png" | "hypermedia.image.png" => (n * 10, n * 16),
            _ => (n * 12, n * 6),
        };
        self.charge(work, scratch)?;
        let image = self.image(required(a, "image")?)?;
        let rgb = match id {
            "hypermedia.radiance.encode_rgb_png" | "hypermedia.image.png" => {
                fields(a, &["image"])?;
                let bytes = cv::codecs::png_encode::encode_png(rgb_view(image)?)
                    .map_err(|e| format!("PNG encode: {e:?}"))?;
                return self.encode("png", bytes);
            }
            "hypermedia.radiance.convolve_blur" | "hypermedia.image.blur" => {
                fields(a, &["image"])?;
                let mut rgb = vec![0; n * 3];
                let mut channel = vec![0; n];
                let mut blurred = vec![0; n];
                for c in 0..3 {
                    for i in 0..n {
                        channel[i] = image.rgb[i * 3 + c];
                    }
                    let view = cv::GrayView::new(width, height, width, &channel)
                        .ok_or("Invalid grayscale layout")?;
                    cv::gaussian_blur_u8(view, &mut blurred).map_err(|e| format!("Blur: {e:?}"))?;
                    for i in 0..n {
                        rgb[i * 3 + c] = blurred[i];
                    }
                }
                rgb
            }
            "hypermedia.radiance.transform_exposure" | "hypermedia.image.exposure" => {
                fields(a, &["image", "stops"])?;
                let stops = number(a, "stops", 0.)?;
                if !(-16. ..=16.).contains(&stops) {
                    return Err("Exposure stops outside -16..16".into());
                }
                let gain = 2f32.powf(stops as f32);
                let table = color_transforms::srgb_u8_to_linear_table();
                image
                    .rgb
                    .iter()
                    .map(|v| color_transforms::linear_to_srgb_u8(table[*v as usize] * gain))
                    .collect()
            }
            "hypermedia.radiance.blend_rgb" | "hypermedia.image.blend" => {
                fields(a, &["image", "source", "mode"])?;
                let source = self.image(required(a, "source")?)?;
                if source.width != width || source.height != height {
                    return Err("Blend dimensions must match".into());
                }
                use raster_blend_operators::{blend_rgb, BlendOperator as B};
                let op = match text(required(a, "mode")?)? {
                    "burn" => B::ColorBurn,
                    "dodge" => B::ColorDodge,
                    "vivid-light" => B::VividLight,
                    "hard-mix" => B::HardMix,
                    _ => return Err("Unknown blend mode".into()),
                };
                let mut rgb = Vec::with_capacity(n * 3);
                for (b, s) in image.rgb.chunks_exact(3).zip(source.rgb.chunks_exact(3)) {
                    let out = blend_rgb(
                        op,
                        [b[0] as f32 / 255., b[1] as f32 / 255., b[2] as f32 / 255.],
                        [s[0] as f32 / 255., s[1] as f32 / 255., s[2] as f32 / 255.],
                    );
                    rgb.extend(out.map(|v| (v.clamp(0., 1.) * 255.).round() as u8));
                }
                rgb
            }
            "hypermedia.radiance.morphology_mask" | "hypermedia.image.mask" => {
                fields(a, &["image", "mode", "radius", "threshold"])?;
                let threshold = number(a, "threshold", 0.5)?;
                if !(0. ..=1.).contains(&threshold) {
                    return Err("Mask threshold outside 0..1".into());
                }
                let radius = number(a, "radius", 1.)?;
                if !(0. ..=1024.).contains(&radius) {
                    return Err("Mask radius outside 0..1024".into());
                }
                let op = match text(required(a, "mode")?)? {
                    "expand" => morphology::Operation::Expand,
                    "contract" => morphology::Operation::Contract,
                    "border" => morphology::Operation::Border,
                    _ => return Err("Unknown mask mode".into()),
                };
                let limits = morphology::Limits {
                    max_pixels: 1024 * 1024,
                    max_work_units: n * 52,
                    max_edt_scratch_bytes: self.limits.total_resource_bytes,
                    max_algorithm_payload_bytes: self.limits.total_resource_bytes,
                };
                morphology::preflight_dimensions(
                    op,
                    width as usize,
                    height as usize,
                    radius as f32,
                    limits,
                )
                .map_err(|e| e.to_string())?;
                let mut gray = vec![0; n];
                cv::rgb_to_gray_u8(rgb_view(image)?, &mut gray)
                    .map_err(|e| format!("Luminance: {e:?}"))?;
                let mask: Vec<f32> = gray
                    .iter()
                    .map(|v| {
                        if *v as f64 / 255. >= threshold {
                            1.
                        } else {
                            0.
                        }
                    })
                    .collect();
                let output = match op {
                    morphology::Operation::Expand => morphology::try_expand(
                        &mask,
                        width as usize,
                        height as usize,
                        radius as f32,
                        limits,
                    ),
                    morphology::Operation::Contract => morphology::try_contract(
                        &mask,
                        width as usize,
                        height as usize,
                        radius as f32,
                        limits,
                    ),
                    morphology::Operation::Border => morphology::try_border(
                        &mask,
                        width as usize,
                        height as usize,
                        radius as f32,
                        limits,
                    ),
                }
                .map_err(|e| e.to_string())?;
                output
                    .iter()
                    .flat_map(|v| [(v.clamp(0., 1.) * 255.).round() as u8; 3])
                    .collect()
            }
            _ => return Err("Unknown image operation".into()),
        };
        self.workspace
            .insert(Artifact::Image(Raster { width, height, rgb }), self.limits)
    }
    fn image(&self, value: &Value) -> Result<&Raster, String> {
        match self.workspace.get(value)? {
            Artifact::Image(r) => Ok(r),
            _ => Err("Expected image handle".into()),
        }
    }
    fn render(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["width", "height", "background", "layers"])?;
        let width = count(a, "width", 256, 1024)? as u32;
        let height = count(a, "height", 256, 1024)? as u32;
        let n = pixels(width, height)?;
        let background = a
            .get("background")
            .map(color)
            .transpose()?
            .unwrap_or([255; 4]);
        if background[3] != 255 {
            return Err("RGB render requires an opaque background".into());
        }
        let layers = list(required(a, "layers")?)?;
        if layers.len() > 64 {
            return Err("At most 64 render layers".into());
        }
        self.charge(n * (layers.len() + 1) * 4, n * 48)?;
        let mut prepared = Vec::new();
        for layer in layers {
            let layer = record(layer)?;
            fields(layer, &["profile", "fill", "matrix"])?;
            let c = color(required(layer, "fill")?)?;
            let m = layer
                .get("matrix")
                .map(vector::<6>)
                .transpose()?
                .unwrap_or([1., 0., 0., 1., 0., 0.]);
            if m.iter().any(|v| v.abs() > 1e6) {
                return Err("Render transform outside bounds".into());
            }
            let points = self.profile(required(layer, "profile")?)?;
            let count = points.len();
            self.charge(count * 16, count * 64)?;
            let points = self.profile(required(layer, "profile")?)?;
            let mut path = kurbo::BezPath::new();
            path.move_to((points[0][0], points[0][1]));
            for p in &points[1..] {
                path.line_to((p[0], p[1]));
            }
            path.close_path();
            // Bound transformed coordinates too, before handing them to the rasterizer.
            let transform = kurbo::Affine::new(m);
            if points.iter().any(|p| {
                let q = transform * kurbo::Point::new(p[0], p[1]);
                !q.x.is_finite() || !q.y.is_finite() || q.x.abs() > 1e6 || q.y.abs() > 1e6
            }) {
                return Err("Transformed render coordinates outside bounds".into());
            }
            prepared.push((
                path,
                transform,
                peniko::Color::from_rgba8(c[0], c[1], c[2], c[3]),
            ));
        }
        let mut rgba = vec![0; n * 4];
        let mut renderer = vector_render::VelloCpuImageRenderer::new(width, height);
        renderer.render(
            |scene| {
                scene.fill(
                    peniko::Fill::NonZero,
                    kurbo::Affine::IDENTITY,
                    peniko::Color::from_rgba8(background[0], background[1], background[2], 255),
                    None,
                    &kurbo::Rect::new(0., 0., width as f64, height as f64),
                );
                for (path, transform, c) in &prepared {
                    scene.fill(peniko::Fill::NonZero, *transform, *c, None, path);
                }
            },
            &mut rgba,
        );
        let rgb = rgba
            .chunks_exact(4)
            .flat_map(|p| [p[0], p[1], p[2]])
            .collect();
        self.workspace
            .insert(Artifact::Image(Raster { width, height, rgb }), self.limits)
    }
}
fn pixels(width: u32, height: u32) -> Result<usize, String> {
    if width == 0 || height == 0 || width > 1024 || height > 1024 {
        return Err("Image dimensions must be 1..1024".into());
    }
    Ok(width as usize * height as usize)
}
fn rgb_view(r: &Raster) -> Result<cv::RgbView<'_>, String> {
    cv::RgbView::new(r.width, r.height, r.width * 3, &r.rgb)
        .ok_or_else(|| "Invalid RGB layout".into())
}
