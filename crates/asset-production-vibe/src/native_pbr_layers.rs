//! Layered material graph evaluation and linked MaterialX export.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::*;
use color_transforms::{linear_to_srgb, srgb_to_linear};
use sha2::{Digest, Sha256};
use surface_pbr_studio::{
    BlendMode, ChannelSource, MaterialXOptions, NormalConvention, PbrChannel, PbrLayer,
    PbrLayerStack, RasterImage, ShaderModel, generate_materialx_document,
};

impl AssetHost<'_> {
    pub(crate) fn node_graph_eval(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["layers", "resolution", "name", "shader"])?;
        let [width, height] = list(required(args, "resolution")?)? else {
            return Err("PBR resolution must be [width, height]".into());
        };
        let (width, height) = (
            integer(width, "resolution")?,
            integer(height, "resolution")?,
        );
        if !(1..=2048).contains(&width) || !(1..=2048).contains(&height) {
            return Err("PBR resolution axes must each be within 1 to 2048 pixels".into());
        }
        let layers = list(required(args, "layers")?)?;
        if !(1..=32).contains(&layers.len()) {
            return Err("Material graph requires 1 to 32 ordered layers".into());
        }
        let name = text(required(args, "name")?)?;
        if name.is_empty() || name.len() > 64 || name.chars().any(char::is_control) {
            return Err(
                "Material name must contain 1 to 64 UTF-8 bytes without control characters".into(),
            );
        }
        let shader = match args
            .get("shader")
            .map(text)
            .transpose()?
            .unwrap_or("open_pbr")
        {
            "open_pbr" => ShaderModel::OpenPbr,
            "standard_surface" => ShaderModel::StandardSurface,
            _ => return Err("PBR shader must be open_pbr or standard_surface".into()),
        };

        let mut stack = PbrLayerStack::new();
        let mut source_evidence = Vec::with_capacity(layers.len());
        let mut resident_pixels = 0usize;
        let mut active_channels = std::collections::BTreeSet::new();
        let mut layer_channels = 0usize;
        for value in layers {
            let row = record(value)?;
            fields(row, &["blend", "opacity", "channels", "mask"])?;
            let blend = args_text(row, "blend", "normal")?;
            let blend_mode = match blend {
                "normal" => BlendMode::Normal,
                "multiply" => BlendMode::Multiply,
                "screen" => BlendMode::Screen,
                "overlay" => BlendMode::Overlay,
                "add" => BlendMode::Add,
                "subtract" => BlendMode::Subtract,
                _ => return Err(format!("Unknown material layer blend mode {blend}")),
            };
            let opacity = unit_interval(number(row, "opacity", 1.0)?, "opacity")?;
            let channels = record(required(row, "channels")?)?;
            if channels.is_empty() || channels.len() > 4 {
                return Err("Material layer requires 1 to 4 supported channels".into());
            }
            let mut layer = PbrLayer {
                name: format!("layer_{}", stack.layers.len()),
                visible: true,
                opacity,
                blend_mode,
                channels: Default::default(),
                mask: None,
            };
            let mut evidence = serde_json::Map::new();
            for (key, input) in channels {
                let channel = channel(key)?;
                let color = channel == PbrChannel::BaseColor;
                let (source, metadata) = if let Value::Record(options) = input {
                    fields(options, &["image", "transfer"])?;
                    let transfer = args_text(options, "transfer", "linear")?;
                    match transfer {
                        "linear" => {}
                        "srgb" if color => {}
                        _ => return Err("Only base_color accepts sRGB-encoded inputs".into()),
                    }
                    let (image, digest) = self.pbr_image(
                        required(options, "image")?,
                        transfer == "srgb",
                        !color,
                        &mut resident_pixels,
                    )?;
                    (
                        ChannelSource::Image(image),
                        serde_json::json!({"sha256":digest,"transfer":transfer}),
                    )
                } else if color {
                    let [r, g, b] = vector::<3>(input)?;
                    let sample = [
                        unit_interval(r, key)?,
                        unit_interval(g, key)?,
                        unit_interval(b, key)?,
                        1.0,
                    ];
                    (ChannelSource::Uniform(sample), serde_json::json!([r, g, b]))
                } else {
                    let sample = num(input)?;
                    let lane = unit_interval(sample, key)?;
                    (
                        ChannelSource::Uniform([lane, lane, lane, 1.0]),
                        serde_json::json!(sample),
                    )
                };
                active_channels.insert(channel);
                layer.channels.insert(channel, source);
                evidence.insert(key.to_owned(), metadata);
                layer_channels += 1;
            }
            let mask_digest = if let Some(mask) = row.get("mask") {
                let (image, digest) = self.pbr_image(mask, false, true, &mut resident_pixels)?;
                layer.mask = Some(image);
                Some(digest)
            } else {
                None
            };
            source_evidence.push(serde_json::json!({"blend":blend,"opacity":opacity,"channels":evidence,"mask_sha256":mask_digest}));
            stack.add_layer(layer);
        }

        let pixels = width * height;
        self.charge(
            pixels.saturating_mul(layer_channels * 48),
            resident_pixels * 16 + pixels * (active_channels.len() * 24 + 40),
        )?;
        let mut maps = BTreeMap::new();
        let mut dependencies = Vec::new();
        let mut map_resources = BTreeMap::new();
        let mut graph_maps = serde_json::Map::new();
        for (channel, image) in stack.composite(width as u32, height as u32) {
            let srgb = channel == PbrChannel::BaseColor;
            let quantize = |v: f32| {
                let v = if srgb { linear_to_srgb(v) } else { v };
                (255.0 * v.clamp(0.0, 1.0)).round() as u8
            };
            let rgba = self.store_rgba(
                image.width,
                image.height,
                image
                    .pixels
                    .into_iter()
                    .map(|[r, g, b, _]| [quantize(r), quantize(g), quantize(b), 255])
                    .collect(),
            )?;
            let png = self.graphics(
                "hypermedia.radiance.encode_rgba_png",
                &Args::from([("image".into(), rgba)]),
            )?;
            let resource = encoded_resource(self, &png)?;
            let digest = format!("{:x}", Sha256::digest(&resource.bytes));
            let key = format!("pbr/{digest}.png");
            if !maps.values().any(|existing| existing == &key) {
                dependencies.push(Value::Record(Args::from([
                    ("key".into(), Value::String(key.clone())),
                    ("resource".into(), png.clone()),
                ])));
            }
            graph_maps.insert(
                channel.id().into(),
                serde_json::json!({"key":key,"sha256":digest}),
            );
            maps.insert(channel, key);
            map_resources.insert(channel.id().into(), png);
        }

        let mut materialx = self.encode(
            "mtlx",
            generate_materialx_document(
                name,
                &maps,
                &MaterialXOptions {
                    shader,
                    normal: NormalConvention::OpenGl,
                    ..Default::default()
                },
            )
            .into_bytes(),
        )?;
        let Value::Record(handle) = &mut materialx else {
            return Err("MaterialX output must have an encoded resource handle".into());
        };
        handle.insert("dependencies".into(), Value::List(dependencies.into()));
        let graph = serde_json::to_vec(&serde_json::json!({"schema":"qualia-pbr-layer-graph/1", "name":name,
            "resolution":[width,height],"layers":source_evidence,"maps":graph_maps,"working_space":"linear_rec709",
        })).map_err(|err| err.to_string())?;
        let graph = self.encode("json", graph)?;
        Ok(Value::Record(Args::from([
            ("materialx".into(), materialx),
            ("graph".into(), graph),
            ("maps".into(), Value::Record(map_resources)),
        ])))
    }

    fn pbr_image(
        &mut self,
        value: &Value,
        srgb: bool,
        scalar: bool,
        resident: &mut usize,
    ) -> Result<(RasterImage, String), String> {
        let image = self.rgba(value)?;
        let (width, height) = (image.width, image.height);
        let samples = image.pixels.len();
        if width == 0
            || height == 0
            || width > 2048
            || height > 2048
            || samples != width as usize * height as usize
        {
            return Err("Material raster extent must match its retained RGBA8 pixels".into());
        }
        *resident = resident.saturating_add(samples);
        self.charge(samples * 12, *resident * 16)?;
        let image = self.rgba(value)?;
        let mut hasher = Sha256::new();
        hasher.update(b"qualia:rgba8:dimensions-le:v1\0");
        hasher.update(width.to_le_bytes());
        hasher.update(height.to_le_bytes());
        hasher.update(image.pixels.as_flattened());
        let mut pixels = Vec::with_capacity(samples);
        for &[r, g, b, a] in &image.pixels {
            if a != 255 || (scalar && (r != g || g != b)) {
                return Err("Material input images require opaque RGBA8 and scalar inputs require grayscale".into());
            }
            let transfer = |c| {
                let linear = f32::from(c) / 255.0;
                if srgb { srgb_to_linear(linear) } else { linear }
            };
            pixels.push([transfer(r), transfer(g), transfer(b), 1.0]);
        }
        Ok((
            RasterImage {
                width,
                height,
                pixels,
            },
            format!("{:x}", hasher.finalize()),
        ))
    }
}

fn args_text<'a>(args: &'a Args, key: &str, default: &'a str) -> Result<&'a str, String> {
    Ok(args.get(key).map(text).transpose()?.unwrap_or(default))
}

fn unit_interval(value: f64, key: &str) -> Result<f32, String> {
    if !(0.0..=1.0).contains(&value) {
        return Err(format!("Material {key} samples must be within [0, 1]"));
    }
    Ok(value as f32)
}

fn channel(name: &str) -> Result<PbrChannel, String> {
    match name {
        "base_color" => Ok(PbrChannel::BaseColor),
        "roughness" => Ok(PbrChannel::Roughness),
        "metallic" => Ok(PbrChannel::Metallic),
        "ao" => Ok(PbrChannel::AO),
        _ => Err(format!("Unsupported layered PBR channel {name}")),
    }
}
