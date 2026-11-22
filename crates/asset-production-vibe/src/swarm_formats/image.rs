//! Linear image interchange for bounded EXR, KTX2, and JPEG XL profiles.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, sync::Arc};

#[cfg(not(target_arch = "wasm32"))]
use qualia_core_db::medical::dicom_codecs::jxl::{JxlLimits, decode_jxl_codestream_into};
use qualia_core_db::render::texture_ktx2::{Ktx2ColorSpace, Ktx2Document, Ktx2Supercompression};
use serde_json::json;

use super::{encode_json, receipt, result, sha256};
use crate::{
    Args, AssetHost, TypedArtifact, Value,
    artifact::{LatticeSamples, RadianceLattice},
    fields, integer,
    qualia_containers_governance_poet_ops::encoded_resource,
    required, text,
};

const LINEAR: &str = "urn:qualia:color:transfer:linear";
const REC709: &str = "urn:qualia:color:primaries:rec709";
const UNKNOWN_PRIMARIES: &str = "urn:qualia:color:primaries:unspecified";
const MAGIC: [u8; 4] = [0x76, 0x2f, 0x31, 0x01];
const MAX_AXIS: usize = 16_384;
const MAX_IMAGE_BYTES: usize = 64 * 1024 * 1024;
const MAX_HEADER: usize = 64 * 1024;
const SRGB_XY: [f32; 8] = [0.64, 0.33, 0.30, 0.60, 0.15, 0.06, 0.3127, 0.3290];

impl AssetHost<'_> {
    pub(crate) fn encode_exr_jxl_ktx2(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["image", "format"])?;
        let format = args
            .get("format")
            .map(text)
            .transpose()?
            .unwrap_or("exr_scanline");
        if format != "exr_scanline" {
            return Err(
                "This image encoder supports exr_scanline (uncompressed, flat, single-part)".into(),
            );
        }
        let handle = required(args, "image")?;
        let image = radiance(self, handle)?;
        let channels = usize::from(image.channels);
        if image.transfer_iri != LINEAR
            || !matches!(channels, 3 | 4)
            || !matches!(image.primaries_iri.as_str(), REC709 | UNKNOWN_PRIMARIES)
        {
            return Err("EXR scanline export requires linear RGB(A) radiance with Rec.709 or explicitly unspecified primaries".into());
        }
        let width = image.width as usize;
        let height = image.height as usize;
        let n = extent(width, height, channels, self.limits.resource_bytes)?;
        let estimate = (n * 4)
            .checked_add(16 * height + 1024)
            .ok_or("EXR size overflow")?;
        if estimate > self.limits.resource_bytes {
            return Err("EXR output exceeds the admitted resource limit".into());
        }
        self.charge(n.saturating_mul(28), estimate.saturating_add(n * 4))?;
        let image = radiance(self, handle)?;
        let samples = match &image.samples {
            LatticeSamples::F32(samples) if samples.len() == n => samples.as_slice(),
            _ => {
                return Err(
                    "EXR export requires an exact little-endian FLOAT sample extent".into(),
                );
            }
        };
        if samples.iter().any(|value| !value.is_finite()) {
            return Err("EXR export requires finite samples".into());
        }
        let output = encode_scanlines(
            width,
            height,
            channels,
            samples,
            image.primaries_iri == REC709,
            estimate,
        );
        let header = read_exr_header(&output)?;
        let decoded = read_exr_pixels(&output, &header, n)?;
        let decoded_samples = decoded.data()?;
        if (decoded.width, decoded.height, decoded.channels)
            != (image.width, image.height, image.channels)
            || decoded.primaries_iri != image.primaries_iri
            || decoded.transfer_iri != image.transfer_iri
            || !decoded_samples
                .iter()
                .zip(samples)
                .all(|(&actual, &expected)| actual.to_bits() == expected.to_bits())
        {
            return Err(
                "EXR readback changed the radiance lattice or its admitted metadata".into(),
            );
        }
        let lattice_sha256 = lattice_digest(&decoded, decoded_samples);
        drop(decoded);
        let metadata = json!({
            "schema":"qualia-image-interchange/1", "format":"exr_scanline",
            "width":width, "height":height, "channels":channels, "sample_type":"float32",
            "compression":"none", "sha256":sha256(&output), "lattice_sha256":lattice_sha256
        });
        let image = self.encode("exr", output)?;
        let report = encode_json(self, &metadata)?;
        Ok(result(&[("image", image), ("report", report)]))
    }

    pub(crate) fn import_exr_jxl_ktx2(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "level",
                "width",
                "height",
                "channels",
                "bits",
                "transfer",
                "lossless_only",
            ],
        )?;
        let source = required(args, "source")?;
        let input = encoded_resource(self, source)?;
        let size = input.bytes.len();
        if size < 2 || size > self.limits.resource_bytes.min(MAX_IMAGE_BYTES) {
            return Err("Image container exceeds bounded input size".into());
        }
        self.charge(size.saturating_mul(4), size)?;
        let input = encoded_resource(self, source)?;
        let ceiling = self.limits.resource_bytes;
        let level = args
            .get("level")
            .map(|v| integer(v, "level"))
            .transpose()?
            .unwrap_or(0);
        let (bitmap, format, sha) = match input.kind.as_str() {
            "exr" => {
                fields(args, &["source"])?;
                let header = read_exr_header(&input.bytes)?;
                let n = extent(
                    header.width(),
                    header.height(),
                    header.channels.len(),
                    ceiling,
                )?;
                self.charge(n.saturating_mul(12), n * 4)?;
                let input = encoded_resource(self, source)?;
                (
                    read_exr_pixels(&input.bytes, &header, n)?,
                    "exr_scanline",
                    sha256(&input.bytes),
                )
            }
            "ktx2" => {
                fields(args, &["source", "level"])?;
                let info = inspect_ktx2(&input.bytes, level, ceiling)?;
                self.charge(info.samples.saturating_mul(10), info.samples * 4)?;
                let input = encoded_resource(self, source)?;
                (
                    read_ktx2(&input.bytes, level, info)?,
                    "ktx2_rgba8",
                    sha256(&input.bytes),
                )
            }
            #[cfg(not(target_arch = "wasm32"))]
            "jxl" => {
                fields(
                    args,
                    &[
                        "source",
                        "width",
                        "height",
                        "channels",
                        "bits",
                        "transfer",
                        "lossless_only",
                    ],
                )?;
                let spec = jxl_spec(args, ceiling)?;
                self.charge(
                    spec.samples.saturating_mul(140),
                    spec.samples.saturating_mul(32),
                )?;
                let input = encoded_resource(self, source)?;
                (
                    read_jxl(&input.bytes, spec)?,
                    "jxl_integer",
                    sha256(&input.bytes),
                )
            }
            #[cfg(target_arch = "wasm32")]
            "jxl" => {
                return Err(
                    "CapabilityUnavailable: JPEG XL decoding requires a native host".into(),
                );
            }
            _ => return Err("Expected EXR, JPEG XL, or KTX2 retained bytes".into()),
        };
        let output = lattice_digest(&bitmap, bitmap.data()?);
        let metadata = json!({"schema":"qualia-image-interchange/1","format":format,
            "width":bitmap.width,"height":bitmap.height,"channels":bitmap.channels,
            "primaries_iri":bitmap.primaries_iri,"transfer_iri":LINEAR,"level":level,
            "source_sha256":sha,"lattice_sha256":output});
        let image = self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(bitmap)))?;
        let report = encode_json(self, &metadata)?;
        let proof = receipt(
            self,
            "urn:qualia:interchange:decoded-radiance",
            &sha,
            &output,
        )?;
        Ok(result(&[
            ("image", image),
            ("report", report),
            ("receipt", proof),
        ]))
    }
}

fn lattice_digest(bitmap: &RadianceLattice, samples: &[f32]) -> String {
    use sha2::Digest;
    let mut hash = sha2::Sha256::new();
    hash.update(bitmap.width.to_le_bytes());
    hash.update(bitmap.height.to_le_bytes());
    hash.update([bitmap.channels]);
    hash.update(bitmap.primaries_iri.as_bytes());
    hash.update([0]);
    for value in samples {
        hash.update(value.to_le_bytes());
    }
    format!("{:x}", hash.finalize())
}

trait FloatSamples {
    fn data(&self) -> Result<&[f32], String>;
}

impl FloatSamples for RadianceLattice {
    fn data(&self) -> Result<&[f32], String> {
        match &self.samples {
            LatticeSamples::F32(values) => Ok(values),
            _ => Err("Decoded image should have FLOAT samples".into()),
        }
    }
}

fn radiance<'a>(host: &'a AssetHost<'_>, handle: &Value) -> Result<&'a RadianceLattice, String> {
    match host.workspace.typed(handle)? {
        TypedArtifact::RadianceLattice(image) => Ok(image),
        _ => Err("Expected a RadianceLattice image".into()),
    }
}

fn extent(width: usize, height: usize, channels: usize, limit: usize) -> Result<usize, String> {
    if !(1..=MAX_AXIS).contains(&width)
        || !(1..=MAX_AXIS).contains(&height)
        || !(1..=4).contains(&channels)
    {
        return Err("Image dimensions or channel count exceed the supported range".into());
    }
    width
        .checked_mul(height)
        .and_then(|n| n.checked_mul(channels))
        .filter(|&n| n <= limit.min(MAX_IMAGE_BYTES) / 4)
        .ok_or_else(|| "Decoded image exceeds the admitted byte extent".into())
}

fn image(
    width: usize,
    height: usize,
    channels: usize,
    primaries: &str,
    data: Vec<f32>,
) -> RadianceLattice {
    RadianceLattice {
        width: width as u32,
        height: height as u32,
        channels: channels as u8,
        primaries_iri: primaries.into(),
        transfer_iri: LINEAR.into(),
        samples: LatticeSamples::F32(data),
    }
}

fn attribute(out: &mut Vec<u8>, name: &[u8], kind: &[u8], data: &[u8]) {
    out.extend_from_slice(name);
    out.push(0);
    out.extend_from_slice(kind);
    out.push(0);
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(data);
}

fn encode_scanlines(
    width: usize,
    height: usize,
    channels: usize,
    samples: &[f32],
    srgb: bool,
    bound: usize,
) -> Vec<u8> {
    let mut out = Vec::with_capacity(bound);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&2_u32.to_le_bytes());
    let names: &[(u8, usize)] = if channels == 4 {
        &[(b'A', 3), (b'B', 2), (b'G', 1), (b'R', 0)]
    } else {
        &[(b'B', 2), (b'G', 1), (b'R', 0)]
    };
    let mut chlist = Vec::new();
    for &(name, _) in names {
        chlist.extend_from_slice(&[name, 0]);
        chlist.extend_from_slice(&2_u32.to_le_bytes());
        chlist.extend_from_slice(&[0, 0, 0, 0]);
        chlist.extend_from_slice(&1_u32.to_le_bytes());
        chlist.extend_from_slice(&1_u32.to_le_bytes());
    }
    chlist.push(0);
    attribute(&mut out, b"channels", b"chlist", &chlist);
    attribute(&mut out, b"compression", b"compression", &[0]);
    let window = [0_u32, 0, (width - 1) as u32, (height - 1) as u32].map(u32::to_le_bytes);
    attribute(&mut out, b"dataWindow", b"box2i", window.as_flattened());
    attribute(&mut out, b"displayWindow", b"box2i", window.as_flattened());
    attribute(&mut out, b"lineOrder", b"lineOrder", &[0]);
    attribute(
        &mut out,
        b"pixelAspectRatio",
        b"float",
        &1_f32.to_le_bytes(),
    );
    attribute(&mut out, b"screenWindowCenter", b"v2f", &[0; 8]);
    attribute(
        &mut out,
        b"screenWindowWidth",
        b"float",
        &1_f32.to_le_bytes(),
    );
    if srgb {
        attribute(
            &mut out,
            b"chromaticities",
            b"chromaticities",
            SRGB_XY.map(f32::to_le_bytes).as_flattened(),
        );
    }
    out.push(0);
    let first = out.len() + height * 8;
    let row_bytes = width * channels * 4;
    for y in 0..height {
        out.extend_from_slice(&((first + y * (row_bytes + 8)) as u64).to_le_bytes());
    }
    for (y, row) in samples.chunks_exact(width * channels).enumerate() {
        out.extend_from_slice(&(y as i32).to_le_bytes());
        out.extend_from_slice(&(row_bytes as u32).to_le_bytes());
        for &(_, c) in names {
            for pixel in row.chunks_exact(channels) {
                out.extend_from_slice(&pixel[c].to_le_bytes());
            }
        }
    }
    out
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, count: usize) -> Result<&'a [u8], String> {
        let end = self.pos.checked_add(count).ok_or("EXR extent overflow")?;
        let slice = self.bytes.get(self.pos..end).ok_or("Truncated EXR data")?;
        self.pos = end;
        Ok(slice)
    }

    fn name(&mut self) -> Result<&'a [u8], String> {
        let tail = &self.bytes[self.pos..];
        let length = tail
            .iter()
            .take(32)
            .position(|&c| c == 0)
            .ok_or("EXR name exceeds profile bounds")?;
        let name = self.take(length + 1)?;
        Ok(&name[..length])
    }

    fn word(&mut self) -> Result<[u8; 4], String> {
        Ok(self.take(4)?.try_into().unwrap())
    }
}

struct Channel {
    index: usize,
    width_bytes: usize,
}

struct ExrHeader {
    window: [i32; 4],
    channels: Vec<Channel>,
    table: usize,
    primaries: &'static str,
    descending: bool,
}

impl ExrHeader {
    fn width(&self) -> usize {
        (i64::from(self.window[2]) - i64::from(self.window[0]) + 1) as usize
    }

    fn height(&self) -> usize {
        (i64::from(self.window[3]) - i64::from(self.window[1]) + 1) as usize
    }
}

fn read_exr_header(bytes: &[u8]) -> Result<ExrHeader, String> {
    let mut cursor = Cursor {
        bytes: &bytes[..bytes.len().min(MAX_HEADER)],
        pos: 0,
    };
    if cursor.take(4)? != MAGIC || u32::from_le_bytes(cursor.word()?) != 2 {
        return Err("EXR decoder requires single-part non-deep scanline OpenEXR v2".into());
    }
    let mut attrs: BTreeMap<&[u8], (&[u8], &[u8])> = BTreeMap::new();
    loop {
        let name = cursor.name()?;
        if name.is_empty() {
            break;
        }
        let kind = cursor.name()?;
        let size = u32::from_le_bytes(cursor.word()?) as usize;
        let value = cursor.take(size)?;
        if attrs.len() == 128 || attrs.insert(name, (kind, value)).is_some() {
            return Err("Duplicate or excessive OpenEXR attributes".into());
        }
    }
    let attr = |name: &[u8], kind: &[u8]| -> Result<&[u8], String> {
        match attrs.get(name) {
            Some(&(actual, value)) if actual == kind => Ok(value),
            _ => Err("Missing or mistyped required EXR attribute".into()),
        }
    };
    if attr(b"compression", b"compression")? != [0] {
        return Err("Only uncompressed EXR scanline data are supported".into());
    }
    let descending = match attr(b"lineOrder", b"lineOrder")? {
        [0] => false,
        [1] => true,
        _ => return Err("EXR scanlines require increasing or decreasing Y order".into()),
    };
    let window = attr(b"dataWindow", b"box2i")?;
    let window: [u8; 16] = window
        .try_into()
        .map_err(|_| "Invalid EXR data window length")?;
    let window: [i32; 4] =
        std::array::from_fn(|i| i32::from_le_bytes(window[i * 4..i * 4 + 4].try_into().unwrap()));
    if !(1..=MAX_AXIS as i64).contains(&(i64::from(window[2]) - i64::from(window[0]) + 1))
        || !(1..=MAX_AXIS as i64).contains(&(i64::from(window[3]) - i64::from(window[1]) + 1))
    {
        return Err("EXR data window is reversed or exceeds the admitted image extent".into());
    }
    let mut chlist = Cursor {
        bytes: attr(b"channels", b"chlist")?,
        pos: 0,
    };
    let mut channels = Vec::new();
    let mut previous: Option<&[u8]> = None;
    loop {
        let name = chlist.name()?;
        if name.is_empty() {
            break;
        }
        let index = match name {
            b"R" => 0,
            b"G" => 1,
            b"B" => 2,
            b"A" => 3,
            _ => return Err("EXR interchange accepts a single RGB or RGBA plane".into()),
        };
        if previous.is_some_and(|name_before| name_before >= name) {
            return Err("EXR channels must be strictly sorted and unique".into());
        }
        previous = Some(name);
        let width_bytes = match u32::from_le_bytes(chlist.word()?) {
            1 => 2,
            2 => 4,
            _ => return Err("Only OpenEXR HALF or FLOAT channels are supported".into()),
        };
        let _ = chlist.take(4)?;
        let sx = u32::from_le_bytes(chlist.word()?);
        let sy = u32::from_le_bytes(chlist.word()?);
        if sx != 1 || sy != 1 {
            return Err("Subsampled EXR channels are outside the profile".into());
        }
        channels.push(Channel { index, width_bytes });
    }
    if chlist.pos != chlist.bytes.len()
        || ![0, 1, 2]
            .iter()
            .all(|i| channels.iter().any(|c| c.index == *i))
    {
        return Err("Incomplete RGB EXR channels".into());
    }
    let primaries = match attrs.get(&b"chromaticities"[..]) {
        None => UNKNOWN_PRIMARIES,
        Some(&(kind, payload))
            if kind == b"chromaticities"
                && payload.len() == 32
                && payload.chunks_exact(4).zip(SRGB_XY).all(|(coord, srgb)| {
                    (f32::from_le_bytes(coord.try_into().unwrap()) - srgb).abs() <= 0.0001
                }) =>
        {
            REC709
        }
        _ => return Err("EXR declares unsupported chromaticities".into()),
    };
    Ok(ExrHeader {
        window,
        channels,
        table: cursor.pos,
        primaries,
        descending,
    })
}

fn decode_half(bits: u16) -> f32 {
    let sign = if bits & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exp = (bits >> 10) & 31;
    let mantissa = f32::from(bits & 1023);
    match exp {
        0 => sign * mantissa * (1.0 / 16_777_216.0),
        31 => f32::NAN,
        _ => sign * (1024.0 + mantissa) * f32::from_bits(u32::from(exp + 102) << 23),
    }
}

fn read_exr_pixels(
    bytes: &[u8],
    header: &ExrHeader,
    count: usize,
) -> Result<RadianceLattice, String> {
    let (width, height) = (header.width(), header.height());
    let channel_count = header.channels.len();
    let start = header.table + height * 8;
    let table = bytes
        .get(header.table..start)
        .ok_or("Truncated EXR line offset table")?;
    let row_size = width * header.channels.iter().map(|c| c.width_bytes).sum::<usize>();
    let block_size = row_size + 8;
    let mut out = vec![0f32; count];
    let mut previous: Option<usize> = None;
    for (row, pointer) in table.chunks_exact(8).enumerate() {
        let pos = usize::try_from(u64::from_le_bytes(pointer.try_into().unwrap()))
            .map_err(|_| "EXR scanline offset exceeds address space")?;
        let end = pos
            .checked_add(block_size)
            .filter(|&end| end <= bytes.len())
            .ok_or("EXR scanline exceeds the source extent")?;
        if pos < start
            || previous.is_some_and(|prior| {
                if header.descending {
                    end > prior
                } else {
                    pos < prior + block_size
                }
            })
        {
            return Err(
                "EXR scanline aliases another block or violates the declared line order".into(),
            );
        }
        previous = Some(pos);
        let mut scan = Cursor { bytes, pos };
        let y = i32::from_le_bytes(scan.word()?);
        let length = u32::from_le_bytes(scan.word()?) as usize;
        if i64::from(y) != i64::from(header.window[1]) + row as i64 || length != row_size {
            return Err("EXR scanline address conflicts with the requested row".into());
        }
        let row = &mut out[row * width * channel_count..][..width * channel_count];
        for channel in &header.channels {
            let raw = scan.take(width * channel.width_bytes)?;
            for (pixel, value) in row
                .chunks_exact_mut(channel_count)
                .zip(raw.chunks_exact(channel.width_bytes))
            {
                let value = if channel.width_bytes == 4 {
                    f32::from_le_bytes(value.try_into().unwrap())
                } else {
                    decode_half(u16::from_le_bytes(value.try_into().unwrap()))
                };
                if !value.is_finite() {
                    return Err("Non-finite EXR sample".into());
                }
                pixel[channel.index] = value;
            }
        }
    }
    Ok(image(width, height, channel_count, header.primaries, out))
}

struct KtxMip {
    width: usize,
    height: usize,
    samples: usize,
    srgb: bool,
}

fn inspect_ktx2(bytes: &[u8], level: usize, ceiling: usize) -> Result<KtxMip, String> {
    let doc =
        Ktx2Document::parse(bytes).map_err(|error| format!("Invalid KTX2 image: {error:?}"))?;
    if doc.pixel_width == 0
        || doc.pixel_height == 0
        || doc.pixel_depth != 0
        || doc.face_count != 1
        || doc.layer_count() != 1
        || doc.type_size != 1
        || !matches!(doc.vk_format, 37 | 43)
        || doc.supercompression() != Ktx2Supercompression::None
        || doc.level_count() > 16
        || level >= doc.level_count()
    {
        return Err(
            "KTX2 import requires one RGBA8 2D face without supercompression and up to 16 mips"
                .into(),
        );
    }
    let width = ((doc.pixel_width >> level).max(1)) as usize;
    let height = ((doc.pixel_height >> level).max(1)) as usize;
    let srgb = match doc.color_space() {
        Ktx2ColorSpace::Linear if doc.vk_format == 37 => false,
        Ktx2ColorSpace::Srgb if doc.vk_format == 43 => true,
        _ => return Err("KTX2 has an incompatible or unknown transfer function".into()),
    };
    Ok(KtxMip {
        width,
        height,
        srgb,
        samples: extent(width, height, 4, ceiling)?,
    })
}

fn read_ktx2(bytes: &[u8], level: usize, info: KtxMip) -> Result<RadianceLattice, String> {
    let doc =
        Ktx2Document::parse(bytes).map_err(|error| format!("Invalid KTX2 image: {error:?}"))?;
    let mip = doc.level(level).ok_or("KTX2 level is absent")?;
    if mip.bytes.len() != info.samples {
        return Err("KTX2 mip byte count differs from its RGBA8 geometry".into());
    }
    let samples = mip
        .bytes
        .iter()
        .enumerate()
        .map(|(i, &byte)| {
            let x = f32::from(byte) / 255.0;
            if info.srgb && i % 4 < 3 {
                color_transforms::srgb_to_linear(x)
            } else {
                x
            }
        })
        .collect();
    Ok(image(
        info.width,
        info.height,
        4,
        UNKNOWN_PRIMARIES,
        samples,
    ))
}

#[cfg(not(target_arch = "wasm32"))]
struct JxlSpec {
    width: usize,
    height: usize,
    channels: usize,
    bits: usize,
    samples: usize,
    srgb: bool,
    lossless: bool,
}

#[cfg(not(target_arch = "wasm32"))]
fn jxl_spec(args: &Args, ceiling: usize) -> Result<JxlSpec, String> {
    let width = integer(required(args, "width")?, "width")?;
    let height = integer(required(args, "height")?, "height")?;
    let channels = integer(required(args, "channels")?, "channels")?;
    let bits = integer(required(args, "bits")?, "bits")?;
    if !matches!(channels, 1 | 3) || !matches!(bits, 8 | 16) {
        return Err(
            "JPEG XL import accepts monochrome or RGB integer channels, at 8 or 16 bits".into(),
        );
    }
    let srgb = match text(required(args, "transfer")?)? {
        "linear" => false,
        "srgb" => true,
        _ => return Err("JPEG XL import requires explicit linear or sRGB source transfer".into()),
    };
    let lossless = match args.get("lossless_only") {
        None => true,
        Some(Value::Bool(value)) => *value,
        _ => return Err("lossless_only must be boolean".into()),
    };
    Ok(JxlSpec {
        width,
        height,
        channels,
        bits,
        srgb,
        lossless,
        samples: extent(width, height, channels, ceiling)?,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn read_jxl(bytes: &[u8], spec: JxlSpec) -> Result<RadianceLattice, String> {
    let bytes_per_sample = spec.bits / 8;
    let mut raw = vec![0u8; spec.samples * bytes_per_sample];
    let limits = JxlLimits {
        max_compressed_bytes: MAX_IMAGE_BYTES,
        max_decoded_bytes: raw.len(),
        max_width: spec.width as u16,
        max_height: spec.height as u16,
        max_extra_channels: 0,
        max_tree_nodes: 4096,
        ..Default::default()
    };
    decode_jxl_codestream_into(
        bytes,
        spec.width as u16,
        spec.height as u16,
        spec.channels as u16,
        spec.bits as u16,
        spec.bits as u16,
        spec.lossless,
        limits,
        &mut raw,
    )
    .map_err(|error| format!("JPEG XL profile decode failed: {error:?}"))?;
    let samples = raw
        .chunks_exact(bytes_per_sample)
        .map(|value| {
            let x = if spec.bits == 8 {
                f32::from(value[0]) / 255.0
            } else {
                f32::from(u16::from_le_bytes(value.try_into().unwrap())) / 65_535.0
            };
            if spec.srgb {
                color_transforms::srgb_to_linear(x)
            } else {
                x
            }
        })
        .collect();
    Ok(image(
        spec.width,
        spec.height,
        spec.channels,
        UNKNOWN_PRIMARIES,
        samples,
    ))
}
