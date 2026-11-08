//! EBML primitives (RFC 8794): variable-length integers (VINT) and element headers.

#[inline]
pub fn vint_len(b: u8) -> Option<usize> {
    if b == 0 {
        None
    } else {
        Some(b.leading_zeros() as usize + 1)
    }
}

/// Element data size: known, or unknown (all value bits set).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Known(u64),
    Unknown,
}

impl Size {
    pub fn known(self) -> Option<u64> {
        match self {
            Size::Known(n) => Some(n),
            Size::Unknown => None,
        }
    }
}

/// Decode an unsigned VINT. Returns `(value, length, all_ones)`.
pub fn read_vint(data: &[u8]) -> Option<(u64, usize, bool)> {
    let first = *data.first()?;
    let len = vint_len(first)?;
    if data.len() < len {
        return None;
    }
    let mask = if len == 8 { 0 } else { 0xFFu8 >> len };
    let mut v = (first & mask) as u64;
    for &b in &data[1..len] {
        v = (v << 8) | b as u64;
    }
    let all_ones = if len == 8 {
        v == 0x00FF_FFFF_FFFF_FFFF
    } else {
        v == (1u64 << (7 * len)) - 1
    };
    Some((v, len, all_ones))
}

/// Decode an element size.
pub fn read_size(data: &[u8]) -> Option<(Size, usize)> {
    let (v, len, ones) = read_vint(data)?;
    Some((if ones { Size::Unknown } else { Size::Known(v) }, len))
}

/// Decode an element ID (marker bits retained).
pub fn read_id(data: &[u8]) -> Option<(u32, usize)> {
    let first = *data.first()?;
    let len = vint_len(first)?;
    if len > 4 || data.len() < len {
        return None;
    }
    let mut id = 0u32;
    for &b in &data[..len] {
        id = (id << 8) | b as u32;
    }
    Some((id, len))
}

/// Decode a signed VINT as used by EBML lacing (value − (2^(7·len−1) − 1)).
pub fn read_svint(data: &[u8]) -> Option<(i64, usize)> {
    let (v, len, _) = read_vint(data)?;
    let bias = (1i64 << (7 * len - 1)) - 1;
    Some((v as i64 - bias, len))
}

/// Encode an element ID.
pub fn write_id(out: &mut Vec<u8>, id: u32) {
    let n = if id >= 0x100_0000 {
        4
    } else if id >= 0x1_0000 {
        3
    } else if id >= 0x100 {
        2
    } else {
        1
    };
    out.extend_from_slice(&id.to_be_bytes()[4 - n..]);
}

/// Encode an unsigned integer as a VINT with a specific width (1..=8 bytes).
pub fn write_vint(val: u64, width: usize, out: &mut Vec<u8>) {
    assert!(width >= 1 && width <= 8);
    let marker = 1u8 << (8 - width);
    let shift = (width - 1) * 8;
    let first = marker | ((val >> shift) as u8);
    out.push(first);
    for i in (0..width - 1).rev() {
        out.push((val >> (i * 8)) as u8);
    }
}

/// Encode a data size as a VINT using the shortest length (or `min_len` if larger).
pub fn write_size(out: &mut Vec<u8>, size: u64, min_len: usize) {
    let mut len = 1;
    while len < 8 && size >= (1u64 << (7 * len)) - 1 {
        len += 1;
    }
    let len = len.max(min_len).min(8);
    let v = size | (1u64 << (7 * len));
    out.extend_from_slice(&v.to_be_bytes()[8 - len..]);
}

/// Encode the "unknown size" marker of the given length.
pub fn write_unknown_size(out: &mut Vec<u8>, len: usize) {
    out.push(0xFFu8 >> (len - 1));
    for _ in 1..len {
        out.push(0xFF);
    }
}

/// Big-endian unsigned integer of 0..=8 bytes.
pub fn uint(data: &[u8]) -> u64 {
    data.iter().take(8).fold(0u64, |a, &b| (a << 8) | b as u64)
}

/// Big-endian two's-complement signed integer of 0..=8 bytes.
pub fn int(data: &[u8]) -> i64 {
    let n = data.len().min(8);
    if n == 0 {
        return 0;
    }
    let v = uint(&data[..n]);
    let shift = 64 - 8 * n as u32;
    ((v << shift) as i64) >> shift
}

/// IEEE float of 0, 4 or 8 bytes.
pub fn float(data: &[u8]) -> f64 {
    match data.len() {
        4 => f32::from_be_bytes(data.try_into().unwrap_or([0; 4])) as f64,
        8 => f64::from_be_bytes(data.try_into().unwrap_or([0; 8])),
        _ => 0.0,
    }
}

/// String / UTF-8 value: bytes up to the first NUL, lossily decoded.
pub fn string(data: &[u8]) -> String {
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
    String::from_utf8_lossy(&data[..end]).into_owned()
}
