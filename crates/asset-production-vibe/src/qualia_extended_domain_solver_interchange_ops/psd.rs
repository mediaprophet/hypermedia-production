//! PSD/PSB layer and resource framing, exact byte preservation, and legacy ABR import.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{fields, integer, required, Args, AssetHost, Resource, Value};
use std::collections::BTreeMap;
use surface_layered_codecs::{parse_abr, AbrTip, PsdHeader};

const MAX_BYTES: usize = 64 * 1024 * 1024;
const MAX_LAYERS: usize = 512;
const MAX_BLOCKS: usize = 8192;
const MAX_BRUSHES: usize = 256;
const MAX_TIP_PIXELS: usize = 8 * 1024 * 1024;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "inspect_psd_layer_info" => inspect(host, params, false),
        "preserve_psd_layer_info" => inspect(host, params, true),
        "extract_psd_layer_info_block" => extract(host, params),
        "import_abr_legacy" => import_abr(host, params),
        _ => Err(format!("Unsupported PSD or brush interchange operation: {op}")),
    }
}

fn resource<'a>(host: &'a AssetHost<'_>, input: &Value) -> Result<&'a Resource, String> {
    let value = match input {
        Value::String(key) => host.inputs.iter().find(|row| &row.key == key)
            .ok_or("Unknown retained layered document or brush input")?,
        _ => host.workspace.encoded(input)?,
    };
    if value.bytes.len() > host.limits.resource_bytes.min(MAX_BYTES) {
        return Err("Layered document or brush input exceeds its bounded extent".into());
    }
    Ok(value)
}

#[derive(Clone, Copy)]
struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, count: usize) -> Result<&'a [u8], String> {
        let bytes = self.bytes;
        let (head, tail) = bytes.split_at_checked(count)
            .ok_or("Layered document field exceeds its enclosing extent")?;
        self.offset += count;
        self.bytes = tail;
        Ok(head)
    }

    fn scope(&mut self, count: usize) -> Result<Self, String> {
        let offset = self.offset;
        let bytes = self.take(count)?;
        Ok(Self { bytes, offset })
    }

    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_be_bytes(self.take(2)?.try_into().expect("two-byte value")))
    }

    fn i32(&mut self) -> Result<i32, String> {
        Ok(i32::from_be_bytes(self.take(4)?.try_into().expect("four-byte value")))
    }

    fn len(&mut self, wide: bool) -> Result<usize, String> {
        let value = if wide {
            u64::from_be_bytes(self.take(8)?.try_into().expect("eight-byte length"))
        } else {
            u64::from(u32::from_be_bytes(self.take(4)?.try_into().expect("four-byte length")))
        };
        usize::try_from(value).map_err(|_| "Layered document field exceeds address space".into())
    }

    fn sized(&mut self, wide: bool) -> Result<Self, String> {
        let count = self.len(wide)?;
        self.scope(count)
    }

    fn padding(&mut self, max: usize) -> Result<(), String> {
        if self.bytes.len() > max || self.bytes.iter().any(|&byte| byte != 0) {
            return Err("Unexpected bytes remaining in a layered document block".into());
        }
        self.take(self.bytes.len())?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Block<'a> {
    scope: &'static str,
    layer_set: &'static str,
    layer_index: Option<usize>,
    signature: &'a [u8],
    identifier: &'a [u8],
    resource_id: Option<u16>,
    offset: usize,
    payload_offset: usize,
    payload: &'a [u8],
    size: usize,
}

#[derive(Default)]
struct Inspection<'a> {
    layers: Vec<Value>,
    blocks: Vec<Block<'a>>,
    merged_transparency: bool,
}

impl<'a> Inspection<'a> {
    fn add_block(&mut self, block: Block<'a>) -> Result<(), String> {
        if self.blocks.len() >= MAX_BLOCKS {
            return Err("Layered document exceeds the opaque-block budget".into());
        }
        self.blocks.push(block);
        Ok(())
    }
}

fn resource_blocks<'a>(mut cursor: Cursor<'a>, result: &mut Inspection<'a>) -> Result<(), String> {
    while !cursor.bytes.is_empty() {
        let offset = cursor.offset;
        let signature = cursor.take(4)?;
        if signature != b"8BIM" {
            return Err("Unsupported PSD image-resource signature".into());
        }
        let id = cursor.u16()?;
        let name_length = usize::from(cursor.u8()?);
        cursor.take(name_length)?;
        cursor.take((name_length + 1) % 2)?;
        let length = cursor.len(false)?;
        let payload_offset = cursor.offset;
        let payload = cursor.take(length)?;
        cursor.take(length % 2)?;
        result.add_block(Block {
            scope: "image_resource", layer_set: "", layer_index: None, signature, identifier: b"8BIM",
            resource_id: Some(id), offset, payload_offset, payload, size: cursor.offset - offset,
        })?;
    }
    Ok(())
}

fn tagged_blocks<'a>(
    mut cursor: Cursor<'a>, psb: bool, layer_set: &'static str, layer: Option<usize>,
    result: &mut Inspection<'a>,
) -> Result<(), String> {
    while cursor.bytes.len() >= 4 {
        let offset = cursor.offset;
        let signature = cursor.take(4)?;
        if signature != b"8BIM" && signature != b"8B64" {
            return Err("Unknown signature in PSD additional layer information".into());
        }
        let key = cursor.take(4)?;
        let long_length = signature == b"8B64" || (psb && [
            b"LMsk", b"Lr16", b"Lr32", b"Layr", b"Mt16", b"Mt32", b"Mtrn", b"Alph",
            b"FMsk", b"lnk2", b"FEid", b"FXid", b"PxSD",
        ].iter().any(|&known| known.as_slice() == key));
        let length = cursor.len(long_length)?;
        let payload_offset = cursor.offset;
        let payload = cursor.take(length)?;
        cursor.take(length % 2)?;
        let short_pad = (length + 1) & !1;
        if layer.is_none() && short_pad % 4 == 2 && cursor.bytes.starts_with(&[0, 0])
            && (cursor.bytes.len() == 2 || cursor.bytes[2..].starts_with(b"8BIM") || cursor.bytes[2..].starts_with(b"8B64"))
        {
            cursor.take(2)?;
        }
        result.add_block(Block {
            scope: if layer.is_some() { "layer" } else { "global_layer" }, layer_set,
            layer_index: layer, signature, identifier: key, resource_id: None, offset, payload_offset,
            payload, size: cursor.offset - offset,
        })?;
        if layer.is_none() {
            let alternative = match key {
                b"Layr" => Some("Layr"),
                b"Lr16" => Some("Lr16"),
                b"Lr32" => Some("Lr32"),
                _ => None,
            };
            if let Some(alternative) = alternative {
                parse_layers(Cursor { bytes: payload, offset: payload_offset }, psb, alternative, result)?;
            }
        }
    }
    cursor.padding(3)
}

fn parse_layers<'a>(
    mut cursor: Cursor<'a>, psb: bool, layer_set: &'static str, result: &mut Inspection<'a>,
) -> Result<(), String> {
    if cursor.bytes.is_empty() {
        return Ok(());
    }
    let count = cursor.u16()? as i16;
    let layers = usize::from(count.unsigned_abs());
    if layers > MAX_LAYERS - result.layers.len() {
        return Err("PSD layer count exceeds its bounded inventory".into());
    }
    result.merged_transparency |= count < 0;
    let mut channel_bytes = 0usize;
    for index in 0..layers {
        let top = cursor.i32()?;
        let left = cursor.i32()?;
        let bottom = cursor.i32()?;
        let right = cursor.i32()?;
        let channels = usize::from(cursor.u16()?);
        if top > bottom || left > right || channels > 64 {
            return Err("PSD layer bounds or channel count is invalid".into());
        }
        for _ in 0..channels {
            cursor.u16()?;
            channel_bytes = channel_bytes.checked_add(cursor.len(psb)?)
                .ok_or("PSD layer channel extent overflow")?;
        }
        if cursor.take(4)? != b"8BIM" {
            return Err("Unsupported PSD layer blend-mode signature".into());
        }
        let blend = cursor.take(4)?;
        let opacity = cursor.u8()?;
        let clipping = cursor.u8()?;
        let flags = cursor.u8()?;
        cursor.u8()?;
        let mut extra = cursor.sized(false)?;
        extra.sized(false)?;
        extra.sized(false)?;
        let length = usize::from(extra.u8()?);
        let name = extra.take(length)?;
        extra.take((4 - (length + 1) % 4) % 4)?;
        tagged_blocks(extra, psb, layer_set, Some(index), result)?;
        result.layers.push(Value::Record(BTreeMap::from([
            ("layer_set".into(), Value::String(layer_set.into())),
            ("layer_index".into(), Value::U64(index as u64)),
            ("name_utf8".into(), std::str::from_utf8(name).ok().map(|name| Value::String(name.into())).unwrap_or(Value::Null)),
            ("name_bytes_hex".into(), Value::String(hex(name))),
            ("bounds_top_left_bottom_right".into(), Value::List([top, left, bottom, right].into_iter()
                .map(|value| Value::I64(i64::from(value))).collect())),
            ("channels".into(), Value::U64(channels as u64)),
            ("blend_key".into(), Value::String(fourcc(blend))),
            ("opacity".into(), Value::U64(u64::from(opacity))),
            ("clipping".into(), Value::U64(u64::from(clipping))),
            ("flags".into(), Value::U64(u64::from(flags))),
        ])));
    }
    cursor.take(channel_bytes)?;
    cursor.padding(3)
}

fn parse_document<'a>(source: &'a Resource) -> Result<(PsdHeader, Inspection<'a>, usize), String> {
    let bytes = &source.bytes;
    let (header, offset) = PsdHeader::decode(bytes)?;
    let psb = header.version.is_psb();
    let (expected_kind, max_dimension) = if psb { ("psb", 300_000) } else { ("psd", 30_000) };
    if source.kind != expected_kind
        || bytes[6..12].iter().any(|&byte| byte != 0)
        || !(1..=56).contains(&header.channels)
        || !(1..=max_dimension).contains(&header.width)
        || !(1..=max_dimension).contains(&header.height)
        || !matches!(header.depth, 1 | 8 | 16 | 32)
    {
        return Err("PSD/PSB header disagrees with its resource kind or declared bounds".into());
    }
    let mut cursor = Cursor { bytes: &bytes[offset..], offset };
    cursor.sized(false)?;
    let resources = cursor.sized(false)?;
    let mut layer_and_mask = cursor.sized(psb)?;
    let composite_offset = cursor.offset;
    if cursor.u16()? > 3 || cursor.bytes.is_empty() {
        return Err("PSD/PSB composite compression or extent is invalid".into());
    }
    let mut result = Inspection::default();
    resource_blocks(resources, &mut result)?;
    if !layer_and_mask.bytes.is_empty() {
        let records = layer_and_mask.sized(psb)?;
        parse_layers(records, psb, "primary", &mut result)?;
        if !layer_and_mask.bytes.is_empty() {
            layer_and_mask.sized(false)?;
            tagged_blocks(layer_and_mask, psb, "", None, &mut result)?;
        }
    }
    Ok((header, result, composite_offset))
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn fourcc(bytes: &[u8]) -> String {
    if bytes.iter().all(|byte| (b' '..=b'~').contains(byte)) {
        String::from_utf8(bytes.to_vec()).expect("printable FourCC")
    } else {
        hex(bytes)
    }
}

fn block_metadata(index: usize, block: Block<'_>) -> Args {
    let family = match (block.resource_id, block.identifier) {
        (Some(1065), _) => "layer_comps",
        (Some(_), _) => "opaque",
        (_, b"SoLd" | b"PlLd" | b"plLd" | b"SoLE") => "smart_objects",
        (_, b"FEid" | b"FXid" | b"FELS") => "filter_effects",
        (_, b"vmsk" | b"vsms" | b"vscg" | b"vstk" | b"vogk") => "vector_shapes",
        (_, b"cmls") => "layer_comps",
        (_, b"Layr" | b"Lr16" | b"Lr32") => "alternate_layer_set",
        _ => "opaque",
    };
    let mut metadata = BTreeMap::from([
        ("block_index".into(), Value::U64(index as u64)),
        ("scope".into(), Value::String(block.scope.into())),
        ("signature".into(), Value::String(fourcc(block.signature))),
        ("family".into(), Value::String(family.into())),
        ("offset".into(), Value::U64(block.offset as u64)),
        ("length".into(), Value::U64(block.size as u64)),
        ("payload_offset".into(), Value::U64(block.payload_offset as u64)),
        ("payload_length".into(), Value::U64(block.payload.len() as u64)),
        ("payload_blake3".into(), Value::String(blake3::hash(block.payload).to_hex().to_string())),
    ]);
    if let Some(id) = block.resource_id {
        metadata.insert("resource_id".into(), Value::U64(u64::from(id)));
    } else {
        metadata.insert("key".into(), Value::String(fourcc(block.identifier)));
    }
    if !block.layer_set.is_empty() {
        metadata.insert("layer_set".into(), Value::String(block.layer_set.into()));
    }
    if let Some(layer) = block.layer_index {
        metadata.insert("layer_index".into(), Value::U64(layer as u64));
    }
    metadata
}

fn inspect(host: &mut AssetHost<'_>, params: &Args, preserve: bool) -> Result<Value, String> {
    fields(params, &["source"])?;
    let input = required(params, "source")?;
    let size = resource(host, input)?.bytes.len();
    host.charge(size.saturating_mul(3), size.saturating_add(8 * 1024 * 1024))?;
    let source = resource(host, input)?;
    let (header, inspection, composite_offset) = parse_document(source)?;
    let mut output = BTreeMap::from([
        ("version".into(), Value::U64(u64::from(header.version.to_u16()))),
        ("width".into(), Value::U64(u64::from(header.width))),
        ("height".into(), Value::U64(u64::from(header.height))),
        ("depth".into(), Value::U64(u64::from(header.depth))),
        ("inspection_scope".into(), Value::String("opaque_layer_and_resource_framing".into())),
        ("composite_offset".into(), Value::U64(composite_offset as u64)),
        ("merged_transparency".into(), Value::Bool(inspection.merged_transparency)),
        ("source_blake3".into(), Value::String(blake3::hash(&source.bytes).to_hex().to_string())),
        ("layers".into(), Value::List(inspection.layers)),
        ("opaque_blocks".into(), Value::List(inspection.blocks.into_iter().enumerate().map(|(index, block)|
            Value::Record(block_metadata(index, block))).collect())),
    ]);
    if preserve {
        let kind = source.kind.clone();
        let bytes = source.bytes.clone();
        output.insert("asset".into(), host.encode(&kind, bytes)?);
    }
    Ok(Value::Record(output))
}

fn extract(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["source", "block_index"])?;
    let input = required(params, "source")?;
    let index = integer(required(params, "block_index")?, "block_index")?;
    let size = resource(host, input)?.bytes.len();
    host.charge(size, size.saturating_add(8 * 1024 * 1024))?;
    let source = resource(host, input)?;
    let (_, inspection, _) = parse_document(source)?;
    let block = *inspection.blocks.get(index).ok_or("PSD opaque block index is out of range")?;
    let mut metadata = block_metadata(index, block);
    let bytes = source.bytes[block.offset..block.offset + block.size].to_vec();
    metadata.insert("block".into(), host.encode("bin", bytes)?);
    Ok(Value::Record(metadata))
}

fn admit_brush_file(bytes: &[u8]) -> Result<(u16, usize), String> {
    let mut cursor = Cursor { bytes, offset: 0 };
    let version = cursor.u16()?;
    let count = usize::from(cursor.u16()?);
    if !matches!(version, 1 | 2) || count == 0 || count > MAX_BRUSHES {
        return Err("ABR import requires a bounded legacy version 1 or 2 collection".into());
    }
    let mut total = 0usize;
    for _ in 0..count {
        let kind = cursor.u16()?;
        let mut record = cursor.sized(false)?;
        match kind {
            1 if record.bytes.len() >= 14 => {}
            2 => {
                record.take(6)?;
                if version == 2 {
                    let length = record.len(false)?;
                    if length > 1024 {
                        return Err("ABR sampled brush name exceeds the text bound".into());
                    }
                    let raw = record.take(length * 2)?;
                    let name = raw.strip_suffix(&[0, 0]).unwrap_or(raw);
                    let units = name.chunks_exact(2).map(|lane|
                        u16::from_be_bytes(lane.try_into().expect("two-byte name unit")));
                    if char::decode_utf16(units).any(|character| !character.is_ok_and(|ch| ch != '\0')) {
                        return Err("ABR brush name contains invalid UTF-16".into());
                    }
                }
                record.take(9)?;
                let top = i64::from(record.i32()?);
                let left = i64::from(record.i32()?);
                let bottom = i64::from(record.i32()?);
                let right = i64::from(record.i32()?);
                let width = right - left;
                let height = bottom - top;
                if !(1..=2048).contains(&width) || !(1..=2048).contains(&height)
                    || record.u16()? != 8 || record.u8()? != 0
                    || record.bytes.len() != (width * height) as usize
                {
                    return Err("ABR sampled tip requires bounded raw 8-bit mask data".into());
                }
                total += record.bytes.len();
                if total > MAX_TIP_PIXELS {
                    return Err("ABR sampled mask bytes exceed the aggregate brush budget".into());
                }
            }
            _ => return Err("Unsupported or truncated legacy ABR brush record".into()),
        }
    }
    cursor.padding(0)?;
    Ok((version, count))
}

fn import_abr(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["source"])?;
    let input = required(params, "source")?;
    let size = resource(host, input)?.bytes.len();
    host.charge(size, size.saturating_add(MAX_TIP_PIXELS))?;
    let source = resource(host, input)?;
    if source.kind != "abr" {
        return Err("Legacy brush import expects a retained or encoded abr input".into());
    }
    let (version, count) = admit_brush_file(&source.bytes)?;
    let file = parse_abr(&source.bytes)?;
    if file.version != version || file.brushes.len() != count {
        return Err("Native ABR result disagrees with the admitted brush inventory".into());
    }
    let mut output = Vec::with_capacity(count);
    for brush in file.brushes {
        let mut values = BTreeMap::from([
            ("name".into(), Value::String(brush.name)),
            ("spacing".into(), Value::U64(u64::from(brush.spacing))),
        ]);
        match brush.tip {
            AbrTip::Computed { diameter, hardness, angle, roundness } => {
                values.insert("type".into(), Value::String("computed".into()));
                values.insert("diameter".into(), Value::U64(u64::from(diameter)));
                values.insert("hardness".into(), Value::U64(u64::from(hardness)));
                values.insert("angle".into(), Value::I64(i64::from(angle)));
                values.insert("roundness".into(), Value::U64(u64::from(roundness)));
            }
            AbrTip::Sampled(tip) => {
                if tip.depth != 8 || u64::from(tip.width) * u64::from(tip.height) != tip.data.len() as u64 {
                    return Err("Native ABR mask disagrees with its admitted raw extent".into());
                }
                values.insert("type".into(), Value::String("sampled".into()));
                values.insert("width".into(), Value::U64(u64::from(tip.width)));
                values.insert("height".into(), Value::U64(u64::from(tip.height)));
                values.insert("depth".into(), Value::U64(u64::from(tip.depth)));
                values.insert("mask".into(), host.encode("gray8", tip.data)?);
            }
        }
        output.push(Value::Record(values));
    }
    Ok(Value::Record(BTreeMap::from([
        ("version".into(), Value::U64(u64::from(version))),
        ("brushes".into(), Value::List(output)),
    ])))
}
