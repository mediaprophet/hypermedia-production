//! Adobe Swatch Exchange (`.ase`) parser and serializer.

use crate::palette::aco::PaletteError;

pub const SIGNATURE: &[u8; 4] = b"ASEF";

#[derive(Debug, Clone, PartialEq)]
pub enum AseColor {
    Rgb([f32; 3]),
    Cmyk([f32; 4]),
    Lab([f32; 3]),
    Gray(f32),
    Other { model: [u8; 4], data: Vec<u8> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AseKind {
    Global,
    Spot,
    #[default]
    Process,
}

impl AseKind {
    pub fn from_u16(v: u16) -> Self {
        match v {
            0 => AseKind::Global,
            1 => AseKind::Spot,
            _ => AseKind::Process,
        }
    }
    pub fn to_u16(self) -> u16 {
        match self {
            AseKind::Global => 0,
            AseKind::Spot => 1,
            AseKind::Process => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AseEntry {
    pub name: String,
    pub color: AseColor,
    pub kind: AseKind,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AseGroup {
    pub name: String,
    pub entries: Vec<AseEntry>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AseFile {
    pub groups: Vec<AseGroup>,
    pub loose: Vec<AseEntry>,
}

pub fn parse_ase(data: &[u8]) -> Result<AseFile, PaletteError> {
    if data.len() < 12 {
        return Err(PaletteError::UnexpectedEof);
    }
    if &data[0..4] != SIGNATURE {
        return Err(PaletteError::InvalidSignature);
    }
    let major = u16::from_be_bytes([data[4], data[5]]);
    if major != 1 {
        return Err(PaletteError::UnsupportedVersion(major));
    }
    let block_count = u32::from_be_bytes([data[8], data[9], data[10], data[11]]) as usize;
    let mut pos = 12;

    let mut out = AseFile::default();
    let mut current_group: Option<AseGroup> = None;

    for _ in 0..block_count {
        if pos + 6 > data.len() {
            break;
        }
        let block_type = u16::from_be_bytes([data[pos], data[pos + 1]]);
        let block_len = u32::from_be_bytes([data[pos + 2], data[pos + 3], data[pos + 4], data[pos + 5]]) as usize;
        pos += 6;
        if pos + block_len > data.len() {
            return Err(PaletteError::UnexpectedEof);
        }
        let block_data = &data[pos..pos + block_len];
        pos += block_len;

        match block_type {
            0xC001 => {
                // Group start
                if block_data.len() < 2 {
                    continue;
                }
                let name_len = u16::from_be_bytes([block_data[0], block_data[1]]) as usize;
                let mut units = Vec::with_capacity(name_len);
                for i in 0..name_len {
                    let off = 2 + i * 2;
                    if off + 2 <= block_data.len() {
                        units.push(u16::from_be_bytes([block_data[off], block_data[off + 1]]));
                    }
                }
                while units.last() == Some(&0) {
                    units.pop();
                }
                let name = String::from_utf16_lossy(&units);
                if let Some(prev) = current_group.take() {
                    out.groups.push(prev);
                }
                current_group = Some(AseGroup { name, entries: Vec::new() });
            }
            0xC002 => {
                // Group end
                if let Some(g) = current_group.take() {
                    out.groups.push(g);
                }
            }
            0x0001 => {
                // Color entry
                if block_data.len() < 2 {
                    continue;
                }
                let name_len = u16::from_be_bytes([block_data[0], block_data[1]]) as usize;
                let mut units = Vec::with_capacity(name_len);
                for i in 0..name_len {
                    let off = 2 + i * 2;
                    if off + 2 <= block_data.len() {
                        units.push(u16::from_be_bytes([block_data[off], block_data[off + 1]]));
                    }
                }
                while units.last() == Some(&0) {
                    units.pop();
                }
                let name = String::from_utf16_lossy(&units);
                let color_off = 2 + name_len * 2;
                if color_off + 4 > block_data.len() {
                    continue;
                }
                let model = [
                    block_data[color_off],
                    block_data[color_off + 1],
                    block_data[color_off + 2],
                    block_data[color_off + 3],
                ];
                let comp_off = color_off + 4;

                let read_f32 = |offset: usize| -> f32 {
                    if offset + 4 <= block_data.len() {
                        f32::from_be_bytes([
                            block_data[offset],
                            block_data[offset + 1],
                            block_data[offset + 2],
                            block_data[offset + 3],
                        ])
                    } else {
                        0.0
                    }
                };

                let (color, kind_off) = match &model {
                    b"RGB " => (AseColor::Rgb([read_f32(comp_off), read_f32(comp_off + 4), read_f32(comp_off + 8)]), comp_off + 12),
                    b"CMYK" => (
                        AseColor::Cmyk([read_f32(comp_off), read_f32(comp_off + 4), read_f32(comp_off + 8), read_f32(comp_off + 12)]),
                        comp_off + 16,
                    ),
                    b"LAB " => (AseColor::Lab([read_f32(comp_off), read_f32(comp_off + 4), read_f32(comp_off + 8)]), comp_off + 12),
                    b"Gray" => (AseColor::Gray(read_f32(comp_off)), comp_off + 4),
                    _ => {
                        let remaining = if block_data.len() >= comp_off + 2 {
                            block_data.len() - comp_off - 2
                        } else {
                            0
                        };
                        (
                            AseColor::Other {
                                model,
                                data: block_data[comp_off..comp_off + remaining].to_vec(),
                            },
                            comp_off + remaining,
                        )
                    }
                };

                let kind = if kind_off + 2 <= block_data.len() {
                    AseKind::from_u16(u16::from_be_bytes([block_data[kind_off], block_data[kind_off + 1]]))
                } else {
                    AseKind::Process
                };

                let entry = AseEntry { name, color, kind };
                if let Some(g) = &mut current_group {
                    g.entries.push(entry);
                } else {
                    out.loose.push(entry);
                }
            }
            _ => {}
        }
    }

    if let Some(g) = current_group {
        out.groups.push(g);
    }

    Ok(out)
}

pub fn write_ase(file: &AseFile) -> Result<Vec<u8>, PaletteError> {
    let mut out = Vec::new();
    out.extend_from_slice(SIGNATURE);
    out.extend_from_slice(&1u16.to_be_bytes()); // major 1
    out.extend_from_slice(&0u16.to_be_bytes()); // minor 0

    // Count blocks: 2 per group (start/end) + entries in groups + loose entries
    let mut block_count = file.loose.len();
    for g in &file.groups {
        block_count += 2 + g.entries.len();
    }
    out.extend_from_slice(&(block_count as u32).to_be_bytes());

    let write_entry = |out: &mut Vec<u8>, entry: &AseEntry| {
        out.extend_from_slice(&0x0001u16.to_be_bytes());
        let mut body = Vec::new();

        let mut units: Vec<u16> = entry.name.encode_utf16().collect();
        units.push(0);
        body.extend_from_slice(&(units.len() as u16).to_be_bytes());
        for u in units {
            body.extend_from_slice(&u.to_be_bytes());
        }

        match &entry.color {
            AseColor::Rgb([r, g, b]) => {
                body.extend_from_slice(b"RGB ");
                body.extend_from_slice(&r.to_be_bytes());
                body.extend_from_slice(&g.to_be_bytes());
                body.extend_from_slice(&b.to_be_bytes());
            }
            AseColor::Cmyk([c, m, y, k]) => {
                body.extend_from_slice(b"CMYK");
                body.extend_from_slice(&c.to_be_bytes());
                body.extend_from_slice(&m.to_be_bytes());
                body.extend_from_slice(&y.to_be_bytes());
                body.extend_from_slice(&k.to_be_bytes());
            }
            AseColor::Lab([l, a, b]) => {
                body.extend_from_slice(b"LAB ");
                body.extend_from_slice(&l.to_be_bytes());
                body.extend_from_slice(&a.to_be_bytes());
                body.extend_from_slice(&b.to_be_bytes());
            }
            AseColor::Gray(v) => {
                body.extend_from_slice(b"Gray");
                body.extend_from_slice(&v.to_be_bytes());
            }
            AseColor::Other { model, data } => {
                body.extend_from_slice(model);
                body.extend_from_slice(data);
            }
        }
        body.extend_from_slice(&entry.kind.to_u16().to_be_bytes());

        out.extend_from_slice(&(body.len() as u32).to_be_bytes());
        out.extend_from_slice(&body);
    };

    // Write groups
    for g in &file.groups {
        // Group start
        out.extend_from_slice(&0xC001u16.to_be_bytes());
        let mut body = Vec::new();
        let mut units: Vec<u16> = g.name.encode_utf16().collect();
        units.push(0);
        body.extend_from_slice(&(units.len() as u16).to_be_bytes());
        for u in units {
            body.extend_from_slice(&u.to_be_bytes());
        }
        out.extend_from_slice(&(body.len() as u32).to_be_bytes());
        out.extend_from_slice(&body);

        for e in &g.entries {
            write_entry(&mut out, e);
        }

        // Group end
        out.extend_from_slice(&0xC002u16.to_be_bytes());
        out.extend_from_slice(&0u32.to_be_bytes()); // length 0
    }

    // Write loose entries
    for e in &file.loose {
        write_entry(&mut out, e);
    }

    Ok(out)
}
