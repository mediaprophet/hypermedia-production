//! Adobe Color Swatch (`.aco`) v1 and v2 parser and serializer.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaletteError {
    UnexpectedEof,
    InvalidSignature,
    UnsupportedVersion(u16),
    LimitExceeded(&'static str),
    Invalid(String),
}

impl std::fmt::Display for PaletteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaletteError::UnexpectedEof => write!(f, "unexpected end of file"),
            PaletteError::InvalidSignature => write!(f, "invalid file signature"),
            PaletteError::UnsupportedVersion(v) => write!(f, "unsupported version {v}"),
            PaletteError::LimitExceeded(msg) => write!(f, "limit exceeded: {msg}"),
            PaletteError::Invalid(msg) => write!(f, "invalid data: {msg}"),
        }
    }
}

impl std::error::Error for PaletteError {}

pub mod space {
    pub const RGB: u16 = 0;
    pub const HSB: u16 = 1;
    pub const CMYK: u16 = 2;
    pub const LAB: u16 = 7;
    pub const GRAY: u16 = 8;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AcoColor {
    Rgb([u16; 3]),
    Hsb([u16; 3]),
    Cmyk([u16; 4]),
    Lab { l: u16, a: i16, b: i16 },
    Gray(u16),
    Other { space: u16, values: [u16; 4] },
}

impl AcoColor {
    pub fn from_record(space: u16, v: [u16; 4]) -> Self {
        match space {
            space::RGB => AcoColor::Rgb([v[0], v[1], v[2]]),
            space::HSB => AcoColor::Hsb([v[0], v[1], v[2]]),
            space::CMYK => AcoColor::Cmyk(v.map(|x| u16::MAX - x)),
            space::LAB => AcoColor::Lab { l: v[0], a: v[1] as i16, b: v[2] as i16 },
            space::GRAY => AcoColor::Gray(v[0]),
            _ => AcoColor::Other { space, values: v },
        }
    }

    pub fn to_record(self) -> (u16, [u16; 4]) {
        match self {
            AcoColor::Rgb([r, g, b]) => (space::RGB, [r, g, b, 0]),
            AcoColor::Hsb([h, s, b]) => (space::HSB, [h, s, b, 0]),
            AcoColor::Cmyk(c) => (space::CMYK, c.map(|x| u16::MAX - x)),
            AcoColor::Lab { l, a, b } => (space::LAB, [l, a as u16, b as u16, 0]),
            AcoColor::Gray(g) => (space::GRAY, [g, 0, 0, 0]),
            AcoColor::Other { space, values } => (space, values),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcoSwatch {
    pub name: String,
    pub color: AcoColor,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AcoFile {
    pub swatches: Vec<AcoSwatch>,
    pub has_names: bool,
}

pub fn parse_aco(data: &[u8]) -> Result<AcoFile, PaletteError> {
    if data.len() < 4 {
        return Err(PaletteError::UnexpectedEof);
    }
    let mut pos = 0;
    let read_u16 = |p: &mut usize| -> Result<u16, PaletteError> {
        if *p + 2 > data.len() {
            return Err(PaletteError::UnexpectedEof);
        }
        let val = u16::from_be_bytes([data[*p], data[*p + 1]]);
        *p += 2;
        Ok(val)
    };
    let read_u32 = |p: &mut usize| -> Result<u32, PaletteError> {
        if *p + 4 > data.len() {
            return Err(PaletteError::UnexpectedEof);
        }
        let val = u32::from_be_bytes([data[*p], data[*p + 1], data[*p + 2], data[*p + 3]]);
        *p += 4;
        Ok(val)
    };

    let version = read_u16(&mut pos)?;
    let read_swatches = |pos: &mut usize, named: bool| -> Result<Vec<AcoSwatch>, PaletteError> {
        let count = read_u16(pos)? as usize;
        let mut swatches = Vec::with_capacity(count.min(65535));
        for _ in 0..count {
            let space = read_u16(pos)?;
            let v = [read_u16(pos)?, read_u16(pos)?, read_u16(pos)?, read_u16(pos)?];
            let color = AcoColor::from_record(space, v);
            let name = if named {
                let name_len = read_u32(pos)? as usize;
                let mut units = Vec::with_capacity(name_len.min(1024));
                for _ in 0..name_len {
                    units.push(read_u16(pos)?);
                }
                while units.last() == Some(&0) {
                    units.pop();
                }
                String::from_utf16_lossy(&units)
            } else {
                String::new()
            };
            swatches.push(AcoSwatch { name, color });
        }
        Ok(swatches)
    };

    match version {
        1 => {
            let v1 = read_swatches(&mut pos, false)?;
            if pos + 4 <= data.len() && data[pos..pos + 2] == [0, 2] {
                pos += 2; // skip version 2 header
                let v2 = read_swatches(&mut pos, true)?;
                return Ok(AcoFile { swatches: v2, has_names: true });
            }
            Ok(AcoFile { swatches: v1, has_names: false })
        }
        2 => {
            let v2 = read_swatches(&mut pos, true)?;
            Ok(AcoFile { swatches: v2, has_names: true })
        }
        v => Err(PaletteError::UnsupportedVersion(v)),
    }
}

pub fn write_aco(swatches: &[AcoSwatch]) -> Result<Vec<u8>, PaletteError> {
    if swatches.len() > 65535 {
        return Err(PaletteError::LimitExceeded("more than 65535 swatches"));
    }
    let count = swatches.len() as u16;
    let mut out = Vec::new();

    // Section 1 (v1, unnamed)
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&count.to_be_bytes());
    for s in swatches {
        let (space, v) = s.color.to_record();
        out.extend_from_slice(&space.to_be_bytes());
        for x in v {
            out.extend_from_slice(&x.to_be_bytes());
        }
    }

    // Section 2 (v2, named)
    out.extend_from_slice(&2u16.to_be_bytes());
    out.extend_from_slice(&count.to_be_bytes());
    for s in swatches {
        let (space, v) = s.color.to_record();
        out.extend_from_slice(&space.to_be_bytes());
        for x in v {
            out.extend_from_slice(&x.to_be_bytes());
        }
        let mut units: Vec<u16> = s.name.encode_utf16().collect();
        units.push(0); // trailing null
        out.extend_from_slice(&(units.len() as u32).to_be_bytes());
        for u in units {
            out.extend_from_slice(&u.to_be_bytes());
        }
    }

    Ok(out)
}
