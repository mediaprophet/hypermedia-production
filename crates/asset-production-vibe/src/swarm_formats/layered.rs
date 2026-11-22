//! Bounded 8-bit RGB layer interchange for PSD and PSB.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::borrow::Cow;

use serde_json::json;
use surface_layered_codecs::psd::{PsdBlendMode, PsdColorMode, PsdHeader, PsdVersion};

use super::{encode_json, result, sha256};
use crate::{
    Args, AssetHost, Value, boolean, fields, graphics::Rgba, list,
    qualia_containers_governance_poet_ops::encoded_resource, record, required, text,
};

const MAX_LAYERS: usize = 128;
const MAX_BYTES: usize = 32 * 1024 * 1024;
const MAX_COORDINATE: i32 = 300_000;

struct Input<'a>(&'a [u8]);

impl<'a> Input<'a> {
    fn take(&mut self, size: usize) -> Result<&'a [u8], String> {
        let (data, remainder) = self.0.split_at_checked(size).ok_or("Truncated PSD/PSB")?;
        self.0 = remainder;
        Ok(data)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], String> {
        self.take(N)?
            .try_into()
            .map_err(|_| "Truncated PSD/PSB field".into())
    }

    fn sized32(&mut self) -> Result<Input<'a>, String> {
        let size = u32::from_be_bytes(self.array()?) as usize;
        self.take(size).map(Input)
    }

    fn sized(&mut self, version: PsdVersion) -> Result<Input<'a>, String> {
        let size = self.size(version)?;
        self.take(size).map(Input)
    }

    fn size(&mut self, version: PsdVersion) -> Result<usize, String> {
        let length = if version.is_psb() {
            u64::from_be_bytes(self.array()?)
        } else {
            u64::from(u32::from_be_bytes(self.array()?))
        };
        usize::try_from(length).map_err(|_| "PSD/PSB section length does not fit this host".into())
    }

    fn empty(&self, description: &str) -> Result<(), String> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(format!("Unsupported PSD/PSB {description}"))
        }
    }
}

#[derive(Clone, Copy)]
struct Bounds {
    top: i32,
    left: i32,
    bottom: i32,
    right: i32,
}

impl Bounds {
    fn dimensions(self, version: PsdVersion, ceiling: usize) -> Result<(u32, u32, usize), String> {
        if [self.top, self.left, self.bottom, self.right]
            .iter()
            .any(|&coord| !(-MAX_COORDINATE..=MAX_COORDINATE).contains(&coord))
        {
            return Err("PSD/PSB layer bounds exceed the admitted coordinate range".into());
        }
        let width =
            u32::try_from(self.right - self.left).map_err(|_| "Inverted PSD layer bounds")?;
        let height =
            u32::try_from(self.bottom - self.top).map_err(|_| "Inverted PSD layer bounds")?;
        Ok((width, height, pixel_count(width, height, version, ceiling)?))
    }
}

struct LayerRecord<'a> {
    bounds: Bounds,
    name: Cow<'a, str>,
    blend: [u8; 4],
    opacity: u8,
    clipping: bool,
    visible: bool,
    transparency_protected: bool,
}

struct Layer<'a> {
    record: LayerRecord<'a>,
    width: u32,
    height: u32,
    size: usize,
    channels: Vec<(usize, usize)>,
}

struct Stack<'a> {
    header: PsdHeader,
    layers: Vec<(Layer<'a>, Vec<[u8; 4]>)>,
    composite: Vec<[u8; 4]>,
}

impl AssetHost<'_> {
    pub(crate) fn transcode_layered_raster(&mut self, args: &Args) -> Result<Value, String> {
        match text(required(args, "direction")?)? {
            "import_stack" => {
                fields(args, &["direction", "source"])?;
                self.import_psd_stack(required(args, "source")?)
            }
            "export_layered" => {
                fields(args, &["direction", "stack", "format"])?;
                let version = match text(required(args, "format")?)? {
                    "psd" => PsdVersion::Psd,
                    "psb" => PsdVersion::Psb,
                    _ => return Err("PSD and PSB are the supported layered export formats".into()),
                };
                self.export_psd(required(args, "stack")?, version)
            }
            _ => Err("direction must be import_stack or export_layered".into()),
        }
    }

    fn import_psd_stack(&mut self, source: &Value) -> Result<Value, String> {
        let ceiling = self.limits.resource_bytes.min(MAX_BYTES);
        let content = encoded_resource(self, source)?;
        let len = content.bytes.len();
        if !matches!(content.kind.as_str(), "psd" | "psb") || len > ceiling {
            return Err("Expected PSD or PSB retained bytes within the admitted input size".into());
        }
        self.charge(len.saturating_mul(4), len)?;
        let available = self.limits.asset_work_units.saturating_sub(self.work) as usize / 5;
        let content = encoded_resource(self, source)?;
        let kind = content.kind.clone();
        let bytes = content.bytes.clone();
        let (stack, samples) = read_stack(&bytes, ceiling.min(available))?;
        let format = name(stack.header.version);
        if format != kind {
            return Err("PSD/PSB bytes do not match the retained format".into());
        }
        self.charge(samples.saturating_mul(20), samples.saturating_mul(4))?;
        let metadata = json!({
            "schema":"qualia-layered-raster/1", "format":format, "color_mode":"rgb",
            "depth":8, "layers":stack.layers.len(), "sha256":sha256(&bytes)
        });
        let composite =
            self.store_rgba(stack.header.width, stack.header.height, stack.composite)?;
        let mut layers = Vec::with_capacity(stack.layers.len());
        for (layer, pixels) in stack.layers {
            let image = self.store_rgba(layer.width, layer.height, pixels)?;
            let record = layer.record;
            let blend = std::str::from_utf8(&record.blend).map_err(|_| "Invalid PSD blend key")?;
            layers.push(result(&[
                ("name", Value::String(record.name.into_owned())),
                ("left", Value::I64(i64::from(record.bounds.left))),
                ("top", Value::I64(i64::from(record.bounds.top))),
                ("opacity", Value::U64(u64::from(record.opacity))),
                ("blend_key", Value::String(blend.into())),
                ("clipping", Value::Bool(record.clipping)),
                ("visible", Value::Bool(record.visible)),
                (
                    "transparency_protected",
                    Value::Bool(record.transparency_protected),
                ),
                ("image", image),
            ]));
        }
        let stack = result(&[("composite", composite), ("layers", Value::List(layers))]);
        Ok(result(&[
            ("stack", stack),
            ("report", encode_json(self, &metadata)?),
        ]))
    }

    fn export_psd(&mut self, stack: &Value, version: PsdVersion) -> Result<Value, String> {
        let ceiling = self.limits.resource_bytes.min(MAX_BYTES);
        let stack = record(stack)?;
        fields(stack, &["composite", "layers"])?;
        let composite = required(stack, "composite")?;
        let layers = list(required(stack, "layers")?)?;
        if layers.is_empty() || layers.len() > MAX_LAYERS {
            return Err(format!("Expected 1..={MAX_LAYERS} actual raster layers"));
        }
        let raster = self.rgba(composite)?;
        let mut size = check_image(raster, version, ceiling)?.saturating_mul(4) + 64;
        for input in layers {
            let (_, image) = export_layer(self, input, version, ceiling)?;
            size = size
                .checked_add(image.pixels.len() * 4 + 1024)
                .ok_or("PSD/PSB size overflow")?;
        }
        if size > ceiling {
            return Err("PSD/PSB layer export exceeds admitted size".into());
        }
        self.charge(size.saturating_mul(8), size.saturating_mul(3))?;
        let merged = self.rgba(composite)?;
        let mut records = Vec::new();
        let mut channels = Vec::new();
        records.extend_from_slice(&(-(layers.len() as i16)).to_be_bytes());
        for input in layers {
            let (record, image) = export_layer(self, input, version, ceiling)?;
            write_layer(&mut records, &mut channels, record, image, version);
        }
        let pad = (records.len() + channels.len()) % 2;
        let info_size = records.len() + channels.len() + pad;
        let header = PsdHeader {
            version,
            channels: 4,
            width: merged.width,
            height: merged.height,
            depth: 8,
            color_mode: PsdColorMode::Rgb,
        };
        let mut output = Vec::with_capacity(size);
        output.extend_from_slice(&header.encode());
        output.extend_from_slice(&[0; 8]);
        push_size(
            &mut output,
            info_size + if version.is_psb() { 12 } else { 8 },
            version,
        );
        push_size(&mut output, info_size, version);
        output.extend_from_slice(&records);
        output.extend_from_slice(&channels);
        output.extend_from_slice(&[0; 7][..pad + 6]);
        for channel in 0..4 {
            output.extend(merged.pixels.iter().map(|pixel| pixel[channel]));
        }
        let format = name(version);
        let metadata = json!({
            "schema":"qualia-layered-raster/1", "format":format, "color_mode":"rgb",
            "depth":8, "layers":layers.len(), "compression":"raw", "sha256":sha256(&output)
        });
        let image = self.encode(format, output)?;
        Ok(result(&[
            ("image", image),
            ("report", encode_json(self, &metadata)?),
        ]))
    }
}

fn name(version: PsdVersion) -> &'static str {
    if version.is_psb() { "psb" } else { "psd" }
}

fn pixel_count(
    width: u32,
    height: u32,
    version: PsdVersion,
    ceiling: usize,
) -> Result<usize, String> {
    let max_axis = if version.is_psb() { 300_000 } else { 30_000 };
    let size = (width as usize)
        .checked_mul(height as usize)
        .ok_or("PSD/PSB sample count overflow")?;
    if width == 0 || height == 0 || width > max_axis || height > max_axis || size > ceiling / 4 {
        return Err("PSD/PSB raster exceeds admitted dimensions or byte budget".into());
    }
    Ok(size)
}

fn check_image(image: &Rgba, version: PsdVersion, ceiling: usize) -> Result<usize, String> {
    let size = pixel_count(image.width, image.height, version, ceiling)?;
    if size != image.pixels.len() {
        return Err("Layer raster sample count differs from dimensions".into());
    }
    Ok(size)
}

fn supported_blend(blend: [u8; 4]) -> Result<[u8; 4], String> {
    if PsdBlendMode::from_key(&blend).to_key() != blend {
        return Err("Unsupported PSD/PSB blend key".into());
    }
    Ok(blend)
}

fn read_layer<'a>(
    input: &mut Input<'a>,
    version: PsdVersion,
    ceiling: usize,
) -> Result<Layer<'a>, String> {
    let bounds = Bounds {
        top: i32::from_be_bytes(input.array()?),
        left: i32::from_be_bytes(input.array()?),
        bottom: i32::from_be_bytes(input.array()?),
        right: i32::from_be_bytes(input.array()?),
    };
    let (width, height, size) = bounds.dimensions(version, ceiling)?;
    let channel_count = u16::from_be_bytes(input.array()?);
    if !matches!(channel_count, 3 | 4) {
        return Err("PSD/PSB layer requires RGB with optional transparency".into());
    }
    let mut channels = Vec::with_capacity(channel_count as usize);
    for _ in 0..channel_count {
        let id = match i16::from_be_bytes(input.array()?) {
            id @ 0..=2 => id as usize,
            -1 => 3,
            _ => return Err("Unsupported PSD/PSB layer channel".into()),
        };
        if channels.iter().any(|&(previous, _)| previous == id) {
            return Err("Duplicate PSD/PSB layer channel".into());
        }
        channels.push((id, input.size(version)?));
    }
    if (0..3).any(|id| !channels.iter().any(|&(present, _)| present == id)) {
        return Err("PSD/PSB layer is missing an RGB channel".into());
    }
    if input.take(4)? != b"8BIM" {
        return Err("Invalid PSD/PSB blend signature".into());
    }
    let blend = supported_blend(input.array()?)?;
    let [opacity, clipping, flags, reserved] = input.array()?;
    if clipping > 1 || flags & !0x0b != 0 || reserved != 0 {
        return Err("Unsupported PSD/PSB layer flags".into());
    }
    let mut extra = input.sized32()?;
    extra.sized32()?.empty("layer mask")?;
    extra.sized32()?.empty("layer blending ranges")?;
    let [name_length] = extra.array()?;
    let name_length = usize::from(name_length);
    let pascal = extra.take(name_length)?;
    extra.take((4 - (name_length + 1) % 4) % 4)?;
    let name = if extra.0.is_empty() {
        Cow::Borrowed(
            std::str::from_utf8(pascal)
                .map_err(|_| "Layer name requires UTF-8 or a Unicode tag")?,
        )
    } else {
        if extra.take(8)? != b"8BIMluni" {
            return Err("Unsupported PSD/PSB additional layer information".into());
        }
        let mut unicode = extra.sized32()?;
        let count = u32::from_be_bytes(unicode.array()?) as usize;
        if count > 256 {
            return Err("PSD/PSB Unicode layer name exceeds the profile budget".into());
        }
        let mut units = Vec::with_capacity(count);
        for _ in 0..count {
            units.push(u16::from_be_bytes(unicode.array()?));
        }
        if unicode.0 == [0, 0] {
            unicode.take(2)?;
        }
        unicode.empty("additional Unicode layer name bytes")?;
        if units.last() == Some(&0) {
            units.pop();
        }
        Cow::Owned(String::from_utf16(&units).map_err(|_| "Invalid PSD/PSB Unicode layer name")?)
    };
    extra.empty("additional layer information")?;
    let record = LayerRecord {
        bounds,
        name,
        blend,
        opacity,
        clipping: clipping == 1,
        visible: flags & 2 == 0,
        transparency_protected: flags & 1 != 0,
    };
    Ok(Layer {
        record,
        width,
        height,
        size,
        channels,
    })
}

fn read_stack(bytes: &[u8], ceiling: usize) -> Result<(Stack<'_>, usize), String> {
    let (header, start) = PsdHeader::decode(bytes)?;
    if header.depth != 8
        || header.color_mode != PsdColorMode::Rgb
        || !matches!(header.channels, 3 | 4)
        || bytes[6..12] != [0; 6]
    {
        return Err("This PSD/PSB stack profile requires 8-bit RGB(A)".into());
    }
    let mut input = Input(&bytes[start..]);
    let version = header.version;
    let pixels = pixel_count(header.width, header.height, version, ceiling)?;
    input.sized32()?.empty("color-mode data")?;
    input.sized32()?.empty("image resources")?;
    let mut section = input.sized(version)?;
    let mut info = section.sized(version)?;
    if info.0.len() % 2 != 0 {
        return Err("Unpadded PSD/PSB layer information".into());
    }
    section.sized32()?.empty("global layer mask")?;
    section.empty("global additional layer information")?;
    let signed_layers = i16::from_be_bytes(info.array()?);
    let count = usize::from(signed_layers.unsigned_abs());
    if count == 0 || count > MAX_LAYERS || (signed_layers < 0) != (header.channels == 4) {
        return Err(
            "PSD/PSB layered profile requires a valid merged-transparency flag and actual layers"
                .into(),
        );
    }
    let mut samples = pixels;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        let layer = read_layer(&mut info, version, ceiling)?;
        samples += layer.size;
        if samples > ceiling / 4 {
            return Err("PSD/PSB decoded layers exceed admitted size".into());
        }
        records.push(layer);
    }
    let mut layers = Vec::with_capacity(count);
    for layer in records {
        let mut decoded = vec![[0, 0, 0, 255]; layer.size];
        for &(channel, length) in &layer.channels {
            let mut data = Input(info.take(length)?);
            read_channels(
                &mut data,
                layer.width as usize,
                &mut decoded,
                &[channel],
                version,
            )?;
        }
        layers.push((layer, decoded));
    }
    if !matches!(info.0, [] | [0]) {
        return Err("Unexpected PSD/PSB trailing layer information".into());
    }
    let mut composite = vec![[0, 0, 0, 255]; pixels];
    let channels = &([0, 1, 2, 3])[..usize::from(header.channels)];
    read_channels(
        &mut input,
        header.width as usize,
        &mut composite,
        channels,
        version,
    )?;
    Ok((
        Stack {
            header,
            layers,
            composite,
        },
        samples,
    ))
}

fn read_channels(
    data: &mut Input<'_>,
    width: usize,
    pixels: &mut [[u8; 4]],
    channels: &[usize],
    version: PsdVersion,
) -> Result<(), String> {
    let compression = u16::from_be_bytes(data.array()?);
    if compression == 0 {
        for &channel in channels {
            let plane = data.take(pixels.len())?;
            for (pixel, &sample) in pixels.iter_mut().zip(plane) {
                pixel[channel] = sample;
            }
        }
    } else if compression == 1 {
        let length_width = if version.is_psb() { 4 } else { 2 };
        let mut lengths = Input(data.take(pixels.len() / width * length_width * channels.len())?);
        for &channel in channels {
            for row in pixels.chunks_exact_mut(width) {
                let length = if version.is_psb() {
                    u32::from_be_bytes(lengths.array()?) as usize
                } else {
                    usize::from(u16::from_be_bytes(lengths.array()?))
                };
                read_packbits(data.take(length)?, row, channel)?;
            }
        }
    } else {
        return Err("PSD/PSB layer profile supports raw and PackBits samples".into());
    }
    data.empty("trailing raster bytes")
}

fn read_packbits(bytes: &[u8], row: &mut [[u8; 4]], channel: usize) -> Result<(), String> {
    let mut source = Input(bytes);
    let mut destination = row;
    while !source.0.is_empty() {
        let [code] = source.array()?;
        let signed = code as i8;
        if signed == -128 {
            continue;
        }
        let length = if signed >= 0 {
            usize::from(code) + 1
        } else {
            1 + usize::from(signed.unsigned_abs())
        };
        let (span, rest) = destination
            .split_at_mut_checked(length)
            .ok_or("PSD PackBits row overrun")?;
        destination = rest;
        if signed >= 0 {
            for (pixel, &sample) in span.iter_mut().zip(source.take(length)?) {
                pixel[channel] = sample;
            }
        } else {
            let [sample] = source.array()?;
            for pixel in span {
                pixel[channel] = sample;
            }
        }
    }
    if destination.is_empty() {
        Ok(())
    } else {
        Err("PSD PackBits row underrun".into())
    }
}

fn signed(value: &Value) -> Result<i32, String> {
    match value {
        Value::I64(n) => i32::try_from(*n).map_err(|_| "Invalid PSD layer coordinate".into()),
        Value::U64(n) => i32::try_from(*n).map_err(|_| "Invalid PSD layer coordinate".into()),
        _ => Err("Expected an integer PSD layer coordinate".into()),
    }
}

fn export_layer<'a, 'b>(
    host: &'a AssetHost<'_>,
    layer: &'b Value,
    version: PsdVersion,
    ceiling: usize,
) -> Result<(LayerRecord<'b>, &'a Rgba), String> {
    let layer = record(layer)?;
    fields(
        layer,
        &[
            "name",
            "left",
            "top",
            "opacity",
            "blend_key",
            "clipping",
            "visible",
            "transparency_protected",
            "image",
        ],
    )?;
    let image = host.rgba(required(layer, "image")?)?;
    check_image(image, version, ceiling)?;
    let top = signed(required(layer, "top")?)?;
    let left = signed(required(layer, "left")?)?;
    let bottom = top
        .checked_add(image.height as i32)
        .ok_or("PSD layer bounds overflow")?;
    let right = left
        .checked_add(image.width as i32)
        .ok_or("PSD layer bounds overflow")?;
    let bounds = Bounds {
        top,
        left,
        bottom,
        right,
    };
    bounds.dimensions(version, ceiling)?;
    let name = text(required(layer, "name")?)?;
    if name.len() > 255 || name.chars().any(char::is_control) {
        return Err("PSD/PSB layer name requires at most 255 UTF-8 bytes without controls".into());
    }
    let blend = text(required(layer, "blend_key")?)?
        .as_bytes()
        .try_into()
        .map_err(|_| "Expected a four-byte PSD blend key")?;
    let blend = supported_blend(blend)?;
    let opacity = u8::try_from(crate::integer(required(layer, "opacity")?, "opacity")?)
        .map_err(|_| "PSD layer opacity must be in 0..=255")?;
    let record = LayerRecord {
        bounds,
        name: Cow::Borrowed(name),
        blend,
        opacity,
        clipping: boolean(layer, "clipping", false)?,
        visible: boolean(layer, "visible", true)?,
        transparency_protected: boolean(layer, "transparency_protected", false)?,
    };
    Ok((record, image))
}

fn push_size(bytes: &mut Vec<u8>, size: usize, version: PsdVersion) {
    if version.is_psb() {
        bytes.extend_from_slice(&(size as u64).to_be_bytes());
    } else {
        bytes.extend_from_slice(&(size as u32).to_be_bytes());
    }
}

fn write_layer(
    records: &mut Vec<u8>,
    samples: &mut Vec<u8>,
    layer: LayerRecord<'_>,
    image: &Rgba,
    version: PsdVersion,
) {
    for coord in [
        layer.bounds.top,
        layer.bounds.left,
        layer.bounds.bottom,
        layer.bounds.right,
    ] {
        records.extend_from_slice(&coord.to_be_bytes());
    }
    records.extend_from_slice(&4u16.to_be_bytes());
    for (channel, id) in [0i16, 1, 2, -1].into_iter().enumerate() {
        records.extend_from_slice(&id.to_be_bytes());
        push_size(records, image.pixels.len() + 2, version);
        samples.extend_from_slice(&[0, 0]);
        samples.extend(image.pixels.iter().map(|pixel| pixel[channel]));
    }
    records.extend_from_slice(b"8BIM");
    records.extend_from_slice(&layer.blend);
    let flags = 8 | (u8::from(!layer.visible) << 1) | u8::from(layer.transparency_protected);
    records.extend_from_slice(&[layer.opacity, u8::from(layer.clipping), flags, 0]);
    let legacy_name: Vec<u8> = layer
        .name
        .chars()
        .map(|c| if c.is_ascii() { c as u8 } else { b'?' })
        .collect();
    let unicode_name: Vec<u16> = if layer.name.is_ascii() {
        Vec::new()
    } else {
        layer.name.encode_utf16().collect()
    };
    let padded_length = (legacy_name.len() + 1).next_multiple_of(4);
    let unicode_length = if unicode_name.is_empty() {
        0
    } else {
        4 + 2 * (unicode_name.len() + 1)
    };
    let unicode_block_length = if unicode_length == 0 {
        0
    } else {
        12 + unicode_length
    };
    records.extend_from_slice(&((8 + padded_length + unicode_block_length) as u32).to_be_bytes());
    records.extend_from_slice(&[0; 8]);
    records.push(legacy_name.len() as u8);
    records.extend_from_slice(&legacy_name);
    records.extend_from_slice(&[0; 3][..padded_length - legacy_name.len() - 1]);
    if unicode_length != 0 {
        records.extend_from_slice(b"8BIMluni");
        records.extend_from_slice(&(unicode_length as u32).to_be_bytes());
        records.extend_from_slice(&(unicode_name.len() as u32).to_be_bytes());
        for unit in unicode_name {
            records.extend_from_slice(&unit.to_be_bytes());
        }
        records.extend_from_slice(&[0, 0]);
    }
}
