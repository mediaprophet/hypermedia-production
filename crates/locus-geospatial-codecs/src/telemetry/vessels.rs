//! Marine Automatic Identification System (AIS) NMEA 0183 !AIVDM position and static data decoder.

use serde::{Deserialize, Serialize};

/// Decoded AIS vessel telemetry message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AisMessage {
    PositionReport {
        mmsi: u32,
        nav_status: u8,
        sog_knots: f64,
        cog_deg: f64,
        heading_deg: Option<u16>,
        longitude: f64,
        latitude: f64,
    },
    StaticData {
        mmsi: u32,
        vessel_name: String,
        callsign: String,
    },
}

/// Decodes an AIS 6-bit ASCII character to its 6-bit integer value (0..63).
fn decode_ais_char(c: u8) -> Option<u8> {
    if (48..=87).contains(&c) {
        Some(c - 48)
    } else if (96..=119).contains(&c) {
        Some(c - 56)
    } else {
        None
    }
}

/// Bit stream extractor for 6-bit unpacked AIS payload.
struct BitReader<'a> {
    payload: &'a [u8],
    bit_pos: usize,
}

impl<'a> BitReader<'a> {
    fn new(payload: &'a [u8]) -> Self {
        Self { payload, bit_pos: 0 }
    }

    fn read_u32(&mut self, num_bits: usize) -> Option<u32> {
        if num_bits > 32 {
            return None;
        }
        let mut val = 0u32;
        for _ in 0..num_bits {
            let char_idx = self.bit_pos / 6;
            let bit_in_char = 5 - (self.bit_pos % 6);
            if char_idx >= self.payload.len() {
                return None;
            }
            let char_val = decode_ais_char(self.payload[char_idx])?;
            let bit = (char_val >> bit_in_char) & 1;
            val = (val << 1) | (bit as u32);
            self.bit_pos += 1;
        }
        Some(val)
    }

    fn read_i32(&mut self, num_bits: usize) -> Option<i32> {
        let u = self.read_u32(num_bits)?;
        let sign_bit = 1 << (num_bits - 1);
        if (u & sign_bit) != 0 {
            Some((u as i32) - ((1 << num_bits) as i32))
        } else {
            Some(u as i32)
        }
    }

    fn read_ascii(&mut self, num_chars: usize) -> String {
        let mut out = String::with_capacity(num_chars);
        let char_table = b"@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_ !\"#$%&'()*+,-./0123456789:;<=>?";
        for _ in 0..num_chars {
            if let Some(code) = self.read_u32(6) {
                let ch = char_table[code as usize & 0x3f] as char;
                if ch != '@' {
                    out.push(ch);
                }
            }
        }
        out.trim().to_string()
    }
}

/// Decodes an NMEA 0183 `!AIVDM` sentence into an `AisMessage`.
pub fn decode_aivdm_sentence(sentence: &str) -> Option<AisMessage> {
    let trimmed = sentence.trim();
    if !trimmed.starts_with("!AIVDM") && !trimmed.starts_with("!AIVDO") {
        return None;
    }

    let body = trimmed.split('*').next()?;
    let fields: Vec<&str> = body.split(',').collect();
    if fields.len() < 6 {
        return None;
    }

    let payload = fields[5].as_bytes();
    let mut reader = BitReader::new(payload);

    let msg_type = reader.read_u32(6)? as u8;
    let _repeat = reader.read_u32(2)?;
    let mmsi = reader.read_u32(30)?;

    match msg_type {
        1 | 2 | 3 => {
            // Position Report Class A
            let nav_status = reader.read_u32(4)? as u8;
            let _rot = reader.read_i32(8)?;
            let raw_sog = reader.read_u32(10)?;
            let sog_knots = (raw_sog as f64) / 10.0;
            let _pos_acc = reader.read_u32(1)?;

            let raw_lon = reader.read_i32(28)?;
            let raw_lat = reader.read_i32(27)?;
            let longitude = (raw_lon as f64) / 600_000.0;
            let latitude = (raw_lat as f64) / 600_000.0;

            let raw_cog = reader.read_u32(12)?;
            let cog_deg = (raw_cog as f64) / 10.0;

            let raw_heading = reader.read_u32(9)?;
            let heading_deg = if raw_heading < 360 { Some(raw_heading as u16) } else { None };

            Some(AisMessage::PositionReport {
                mmsi,
                nav_status,
                sog_knots,
                cog_deg,
                heading_deg,
                longitude,
                latitude,
            })
        }
        5 => {
            // Static and Voyage Data
            let _ais_version = reader.read_u32(2)?;
            let _imo = reader.read_u32(30)?;
            let callsign = reader.read_ascii(7);
            let vessel_name = reader.read_ascii(20);

            Some(AisMessage::StaticData {
                mmsi,
                vessel_name,
                callsign,
            })
        }
        _ => None,
    }
}
