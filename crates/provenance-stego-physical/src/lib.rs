//! Compact, error-corrected creation-tree anchors for manufacture and inspection.

pub const ANCHOR_BYTES: usize = 23;
pub const CODEWORD_BYTES: usize = 64;
const DATA_BYTES: usize = 32;
// A 13-chip Barker sequence plus three framing bits.
const PREAMBLE: [u8; 2] = [0xf9, 0xad];
const PROTOCOL: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreationTreeAnchor {
    pub carrier: u8,
    pub digest_128: [u8; 16],
    pub bom_slot: u16,
    pub governance_flags: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorError {
    InvalidCarrier,
    InvalidPreamble,
    UnsupportedProtocol,
    InvalidChecksum,
    InvalidPadding,
    TooManyErasures,
    Uncorrectable,
}

fn crc8(bytes: &[u8]) -> u8 {
    let mut crc = 0u8;
    for &byte in bytes {
        crc ^= byte;
        for _ in 0..8 {
            crc = (crc << 1) ^ if crc & 0x80 == 0 { 0 } else { 0x07 };
        }
    }
    crc
}

impl CreationTreeAnchor {
    pub fn to_bytes(self) -> Result<[u8; ANCHOR_BYTES], AnchorError> {
        if !(1..=6).contains(&self.carrier) {
            return Err(AnchorError::InvalidCarrier);
        }
        let mut bytes = [0; ANCHOR_BYTES];
        bytes[..2].copy_from_slice(&PREAMBLE);
        bytes[2] = (PROTOCOL << 4) | self.carrier;
        bytes[3..19].copy_from_slice(&self.digest_128);
        bytes[19..21].copy_from_slice(&self.bom_slot.to_be_bytes());
        bytes[21] = self.governance_flags;
        bytes[22] = crc8(&bytes[..22]);
        Ok(bytes)
    }

    pub fn from_bytes(bytes: [u8; ANCHOR_BYTES]) -> Result<Self, AnchorError> {
        if bytes[..2] != PREAMBLE {
            return Err(AnchorError::InvalidPreamble);
        }
        if bytes[2] >> 4 != PROTOCOL {
            return Err(AnchorError::UnsupportedProtocol);
        }
        if crc8(&bytes[..22]) != bytes[22] {
            return Err(AnchorError::InvalidChecksum);
        }
        let carrier = bytes[2] & 0x0f;
        if !(1..=6).contains(&carrier) {
            return Err(AnchorError::InvalidCarrier);
        }
        let mut digest_128 = [0; 16];
        digest_128.copy_from_slice(&bytes[3..19]);
        Ok(Self {
            carrier,
            digest_128,
            bom_slot: u16::from_be_bytes([bytes[19], bytes[20]]),
            governance_flags: bytes[21],
        })
    }
}

// GF(256) with primitive polynomial x^8 + x^4 + x^3 + x^2 + 1.
fn multiply(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0;
    while b != 0 {
        if b & 1 != 0 {
            result ^= a;
        }
        a = (a << 1) ^ if a & 0x80 == 0 { 0 } else { 0x1d };
        b >>= 1;
    }
    result
}

fn inverse(value: u8) -> u8 {
    debug_assert_ne!(value, 0);
    let mut power = value;
    let mut inverse = 1;
    for bit in 0..8 {
        if 254 & (1 << bit) != 0 {
            inverse = multiply(inverse, power);
        }
        power = multiply(power, power);
    }
    inverse
}

fn evaluate(polynomial: &[u8], point: u8) -> u8 {
    polynomial.iter().rev().fold(0, |value, &coefficient| multiply(value, point) ^ coefficient)
}

// Transpose an 8×8 symbol grid before framing to spread contiguous surface loss.
const fn channel_slot(symbol: usize) -> usize {
    (symbol % 8) * 8 + symbol / 8
}

/// Encode the anchor in a 512-bit RS(64,32) block with fixed zero padding.
pub fn encode_anchor(anchor: CreationTreeAnchor) -> Result<[u8; CODEWORD_BYTES], AnchorError> {
    let mut message = [0; DATA_BYTES];
    message[..ANCHOR_BYTES].copy_from_slice(&anchor.to_bytes()?);
    let mut packet = [0; CODEWORD_BYTES];
    for symbol in 0..CODEWORD_BYTES {
        packet[channel_slot(symbol)] = evaluate(&message, symbol as u8 + 1);
    }
    Ok(packet)
}

/// Decode a framed RS block. Missing or low-confidence symbols are erasures.
/// A block is correctable when twice the number of errors plus the number of
/// erasures is at most 32.
pub fn decode_anchor(packet: &[Option<u8>; CODEWORD_BYTES]) -> Result<CreationTreeAnchor, AnchorError> {
    let received = packet.iter().filter(|symbol| symbol.is_some()).count();
    if received < DATA_BYTES {
        return Err(AnchorError::TooManyErasures);
    }
    for errors in 0..=(received - DATA_BYTES) / 2 {
        if let Some(message) = decode_message(packet, errors) {
            if message[ANCHOR_BYTES..].iter().any(|&byte| byte != 0) {
                return Err(AnchorError::InvalidPadding);
            }
            let mut anchor = [0; ANCHOR_BYTES];
            anchor.copy_from_slice(&message[..ANCHOR_BYTES]);
            return CreationTreeAnchor::from_bytes(anchor);
        }
    }
    Err(AnchorError::Uncorrectable)
}

fn decode_message(packet: &[Option<u8>; CODEWORD_BYTES], errors: usize) -> Option<[u8; DATA_BYTES]> {
    // Berlekamp–Welch: Q(x) = y E(x), with E monic of degree 'errors'.
    let q_terms = DATA_BYTES + errors;
    let variables = q_terms + errors;
    let mut matrix = [[0u8; CODEWORD_BYTES + 1]; CODEWORD_BYTES];
    let mut rows = 0;
    for symbol in 0..CODEWORD_BYTES {
        if let Some(value) = packet[channel_slot(symbol)] {
            let point = symbol as u8 + 1;
            let row = &mut matrix[rows];
            let mut power = 1;
            for basis in &mut row[..q_terms] {
                *basis = power;
                power = multiply(power, point);
            }
            power = 1;
            for basis in &mut row[q_terms..variables] {
                *basis = multiply(value, power);
                power = multiply(power, point);
            }
            row[variables] = multiply(value, power);
            rows += 1;
        }
    }
    for column in 0..variables {
        let pivot = (column..rows).find(|&row| matrix[row][column] != 0)?;
        matrix.swap(column, pivot);
        let scale = inverse(matrix[column][column]);
        for value in &mut matrix[column][column..=variables] {
            *value = multiply(*value, scale);
        }
        let pivot_row = matrix[column];
        for (i, row) in matrix[..rows].iter_mut().enumerate() {
            let scale = row[column];
            if i != column && scale != 0 {
                for (value, basis) in row[column..=variables].iter_mut().zip(&pivot_row[column..=variables]) {
                    *value ^= multiply(scale, *basis);
                }
            }
        }
    }
    if matrix[variables..rows].iter().any(|row| row[variables] != 0) {
        return None;
    }
    let mut numerator = [0; CODEWORD_BYTES];
    let mut locator = [0; DATA_BYTES];
    for (coefficient, row) in numerator[..q_terms].iter_mut().zip(&matrix[..q_terms]) {
        *coefficient = row[variables];
    }
    for (coefficient, row) in locator[..errors].iter_mut().zip(&matrix[q_terms..variables]) {
        *coefficient = row[variables];
    }
    locator[errors] = 1;
    let mut message = [0; DATA_BYTES];
    for degree in (errors..q_terms).rev() {
        let factor = numerator[degree];
        let offset = degree - errors;
        message[offset] = factor;
        for (index, &coefficient) in locator[..=errors].iter().enumerate() {
            numerator[offset + index] ^= multiply(factor, coefficient);
        }
    }
    if numerator[..errors].iter().any(|&value| value != 0) {
        return None;
    }
    let disagreements = (0..CODEWORD_BYTES).filter(|&symbol| {
        packet[channel_slot(symbol)].is_some_and(|value| value != evaluate(&message, symbol as u8 + 1))
    }).count();
    (disagreements <= errors).then_some(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: CreationTreeAnchor = CreationTreeAnchor {
        carrier: 5,
        digest_128: [0x29; 16],
        bom_slot: 32768,
        governance_flags: 0x63,
    };

    #[test]
    fn recovers_184_bits_from_19_erasures_and_six_symbol_errors() {
        let mut observed = encode_anchor(SAMPLE).unwrap().map(Some);
        observed[..19].fill(None);
        for symbol in &mut observed[19..25] {
            *symbol = symbol.map(|byte| byte ^ 0xa7);
        }
        assert_eq!(decode_anchor(&observed), Ok(SAMPLE));
    }

    #[test]
    fn rejects_invalid_framing_and_excessive_erasures() {
        let checksum = SAMPLE.to_bytes().unwrap();
        let mut damaged = checksum;
        damaged[5] ^= 1;
        assert_eq!(CreationTreeAnchor::from_bytes(damaged), Err(AnchorError::InvalidChecksum));
        let mut observed = encode_anchor(SAMPLE).unwrap().map(Some);
        observed[..33].fill(None);
        assert_eq!(decode_anchor(&observed), Err(AnchorError::TooManyErasures));
    }
}
