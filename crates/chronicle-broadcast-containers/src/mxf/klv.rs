//! KLV coding (SMPTE ST 336): 16-byte universal label keys, BER lengths, and the key classes used
//! by ST 377-1.

use std::fmt;

/// A SMPTE universal label (16 bytes).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Ul(pub [u8; 16]);

impl Ul {
    /// Every SMPTE label starts with the object identifier 06 0E 2B 34.
    pub fn is_smpte(&self) -> bool {
        self.0[..4] == [0x06, 0x0E, 0x2B, 0x34]
    }
    /// Compare ignoring byte 7 (the registry version, which varies between writers).
    pub fn matches(&self, other: &[u8; 16]) -> bool {
        self.0.iter().zip(other).enumerate().all(|(i, (a, b))| i == 7 || a == b)
    }
    /// Bytes 8.. start with `prefix` (registry-independent comparison of the item designator).
    pub fn item_starts_with(&self, prefix: &[u8]) -> bool {
        self.0[8..].starts_with(prefix)
    }
}

impl fmt::Debug for Ul {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, b) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(".")?;
            }
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Display for Ul {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// A rational number (edit rates, sample rates, aspect ratios).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rational {
    pub num: i32,
    pub den: i32,
}

impl Rational {
    pub const fn new(num: i32, den: i32) -> Self {
        Rational { num, den }
    }
    pub fn is_valid(&self) -> bool {
        self.num > 0 && self.den > 0
    }
    pub fn as_f64(&self) -> f64 {
        if self.den == 0 {
            0.0
        } else {
            self.num as f64 / self.den as f64
        }
    }
}

/// Partition pack keys: 06 0E 2B 34 02 05 01 01 0D 01 02 01 01 kk ss 00 (kk: 02 header, 03 body,
/// 04 footer; ss: status).
pub const PARTITION_PREFIX: [u8; 13] = [
    0x06, 0x0E, 0x2B, 0x34, 0x02, 0x05, 0x01, 0x01, 0x0D, 0x01, 0x02, 0x01, 0x01,
];

/// The kinds of KLV packets the demuxer distinguishes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyClass {
    Partition { kind: u8, status: u8 },
    Primer,
    LocalSet,
    IndexSegment,
    RandomIndexPack,
    Fill,
    Essence,
    Other,
}

pub fn classify(k: &Ul) -> KeyClass {
    let b = &k.0;
    if !k.is_smpte() {
        return KeyClass::Other;
    }
    if b[..13] == PARTITION_PREFIX && (2..=4).contains(&b[13]) && b[15] == 0 {
        return KeyClass::Partition {
            kind: b[13],
            status: b[14],
        };
    }
    if b[4] == 0x02 && b[5] == 0x05 && b[8..16] == [0x0D, 0x01, 0x02, 0x01, 0x01, 0x05, 0x01, 0x00] {
        return KeyClass::Primer;
    }
    if b[4] == 0x02 && b[5] == 0x05 && b[8..16] == [0x0D, 0x01, 0x02, 0x01, 0x01, 17, 1, 0] {
        return KeyClass::RandomIndexPack;
    }
    if b[4] == 0x02 && b[8..16] == [0x0D, 0x01, 0x02, 0x01, 0x01, 16, 1, 0] {
        return KeyClass::IndexSegment;
    }
    if b[4] == 0x01 && b[5] == 0x01 && b[8..12] == [0x03, 0x01, 0x02, 0x10] {
        return KeyClass::Fill;
    }
    if b[8..12] == [0x0D, 0x01, 0x03, 0x01] {
        return KeyClass::Essence;
    }
    if b[4] == 0x02 && b[5] == 0x53 {
        return KeyClass::LocalSet;
    }
    KeyClass::Other
}

/// Decode a BER length at `b[0..]`: (length, bytes used). `None` if truncated or longer than 8 bytes.
pub fn ber_length(b: &[u8]) -> Option<(u64, usize)> {
    let first = *b.first()?;
    if first < 0x80 {
        return Some((first as u64, 1));
    }
    let n = (first & 0x7F) as usize;
    if n == 0 || n > 8 || b.len() < 1 + n {
        return None;
    }
    Some((
        b[1..=n].iter().fold(0u64, |a, &x| (a << 8) | x as u64),
        1 + n,
    ))
}

/// Write a 4-byte BER length.
pub fn write_ber4(out: &mut Vec<u8>, len: u32) {
    out.push(0x83);
    out.push((len >> 16) as u8);
    out.push((len >> 8) as u8);
    out.push(len as u8);
}

/// Big-endian cursor over a slice.
pub struct Cur<'a> {
    b: &'a [u8],
    pub pos: usize,
}

impl<'a> Cur<'a> {
    pub fn new(b: &'a [u8]) -> Self {
        Cur { b, pos: 0 }
    }
    pub fn remaining(&self) -> usize {
        self.b.len().saturating_sub(self.pos)
    }
    pub fn bytes(&mut self, n: usize) -> Option<&'a [u8]> {
        let s = self.b.get(self.pos..self.pos.checked_add(n)?)?;
        self.pos += n;
        Some(s)
    }
    pub fn u16(&mut self) -> Option<u16> {
        self.bytes(2).map(|b| u16::from_be_bytes([b[0], b[1]]))
    }
    pub fn u32(&mut self) -> Option<u32> {
        self.bytes(4).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
    }
    pub fn u64(&mut self) -> Option<u64> {
        self.bytes(8).map(|b| u64::from_be_bytes(b.try_into().unwrap_or([0; 8])))
    }
    pub fn ul(&mut self) -> Option<Ul> {
        self.bytes(16).map(|b| Ul(b.try_into().unwrap_or([0; 16])))
    }
}

pub fn int_of(v: &[u8], signed: bool) -> Option<i64> {
    if v.is_empty() || v.len() > 8 {
        return None;
    }
    let u = v.iter().fold(0u64, |a, &x| (a << 8) | x as u64);
    if signed && v.len() < 8 && v[0] & 0x80 != 0 {
        let shift = 64 - 8 * v.len();
        return Some(((u << shift) as i64) >> shift);
    }
    Some(u as i64)
}

pub fn rational_of(v: &[u8]) -> Option<Rational> {
    if v.len() < 8 {
        return None;
    }
    Some(Rational::new(
        i32::from_be_bytes([v[0], v[1], v[2], v[3]]),
        i32::from_be_bytes([v[4], v[5], v[6], v[7]]),
    ))
}

pub fn ul_of(v: &[u8]) -> Option<Ul> {
    v.get(..16).map(|b| Ul(b.try_into().unwrap_or([0; 16])))
}

pub fn batch_of(v: &[u8]) -> Vec<&[u8]> {
    let mut c = Cur::new(v);
    let (Some(n), Some(size)) = (c.u32(), c.u32()) else {
        return Vec::new();
    };
    let size = size as usize;
    if size == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    for _ in 0..n {
        match c.bytes(size) {
            Some(b) => out.push(b),
            None => break,
        }
    }
    out
}

pub fn utf16_of(v: &[u8]) -> String {
    let units: Vec<u16> = v
        .chunks_exact(2)
        .map(|c| u16::from_be_bytes([c[0], c[1]]))
        .take_while(|&u| u != 0)
        .collect();
    String::from_utf16_lossy(&units)
}
