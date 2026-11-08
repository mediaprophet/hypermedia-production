//! Streaming ADS-B Mode S / 1090 MHz aircraft position, velocity, and callsign decoder.

use serde::{Deserialize, Serialize};

/// Decoded ADS-B aircraft telemetry message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AircraftMessage {
    pub icao: u32,
    pub callsign: Option<String>,
    pub altitude_ft: Option<i32>,
    pub cpr_lat_raw: Option<u32>,
    pub cpr_lon_raw: Option<u32>,
    pub cpr_odd: Option<bool>,
    pub heading_deg: Option<f64>,
    pub speed_knots: Option<f64>,
}

/// A stored raw CPR frame for global dual-frame position reconstruction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CprFrame {
    pub cpr_lat: u32,
    pub cpr_lon: u32,
    pub is_odd: bool,
    pub timestamp_ms: u64,
}

/// Decodes 14-byte (112-bit) raw ADS-B Mode S packet.
pub fn decode_adsb_packet(bytes: &[u8]) -> Option<AircraftMessage> {
    if bytes.len() < 14 {
        return None;
    }

    let df = bytes[0] >> 3;
    if df != 17 && df != 18 {
        // Downlink Format 17/18 is Extended Squitter
        return None;
    }

    let icao = ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | (bytes[3] as u32);
    let type_code = bytes[4] >> 3;

    let mut msg = AircraftMessage {
        icao,
        callsign: None,
        altitude_ft: None,
        cpr_lat_raw: None,
        cpr_lon_raw: None,
        cpr_odd: None,
        heading_deg: None,
        speed_knots: None,
    };

    match type_code {
        1..=4 => {
            // Aircraft Identification and Category (Callsign)
            let mut chars = String::with_capacity(8);
            let b0 = bytes[5] as u64;
            let b1 = bytes[6] as u64;
            let b2 = bytes[7] as u64;
            let b3 = bytes[8] as u64;
            let b4 = bytes[9] as u64;
            let b5 = bytes[10] as u64;
            let payload = (b0 << 40) | (b1 << 32) | (b2 << 24) | (b3 << 16) | (b4 << 8) | b5;

            let char_table = b"#ABCDEFGHIJKLMNOPQRSTUVWXYZ##### ###############0123456789######";
            for i in (0..8).rev() {
                let code = ((payload >> (i * 6)) & 0x3f) as usize;
                let ch = char_table[code] as char;
                if ch != ' ' && ch != '#' {
                    chars.push(ch);
                }
            }
            msg.callsign = Some(chars.trim().to_string());
        }
        9..=18 => {
            // Airborne Position
            let raw_alt = (((bytes[5] as u32) & 0xff) << 4) | ((bytes[6] as u32) >> 4);
            // Q bit is bit 4 of raw_alt (bit 47 of packet)
            let q_bit = (raw_alt & 0x10) != 0;
            if q_bit {
                let n = ((raw_alt & 0xfe0) >> 1) | (raw_alt & 0x0f);
                msg.altitude_ft = Some((n as i32) * 25 - 1000);
            }

            let cpr_odd = (bytes[6] & 0x04) != 0;
            let cpr_lat = (((bytes[6] as u32) & 0x03) << 15) | ((bytes[7] as u32) << 7) | ((bytes[8] as u32) >> 1);
            let cpr_lon = (((bytes[8] as u32) & 0x01) << 16) | ((bytes[9] as u32) << 8) | (bytes[10] as u32);

            msg.cpr_odd = Some(cpr_odd);
            msg.cpr_lat_raw = Some(cpr_lat);
            msg.cpr_lon_raw = Some(cpr_lon);
        }
        19 => {
            // Airborne Velocity
            let sub_type = bytes[4] & 0x07;
            if sub_type == 1 || sub_type == 2 {
                let v_ew_sign = (bytes[5] & 0x04) != 0;
                let v_ew = (((bytes[5] as i32) & 0x03) << 8) | (bytes[6] as i32);
                let v_ns_sign = (bytes[7] & 0x80) != 0;
                let v_ns = (((bytes[7] as i32) & 0x7f) << 3) | ((bytes[8] as i32) >> 5);

                let vx = if v_ew_sign { -(v_ew - 1) } else { v_ew - 1 };
                let vy = if v_ns_sign { -(v_ns - 1) } else { v_ns - 1 };

                let speed = ((vx * vx + vy * vy) as f64).sqrt();
                let heading = (vx as f64).atan2(vy as f64).to_degrees().rem_euclid(360.0);
                msg.speed_knots = Some(speed);
                msg.heading_deg = Some(heading);
            }
        }
        _ => {}
    }

    Some(msg)
}

/// Decodes geographic position (latitude, longitude) from an even and an odd CPR frame.
pub fn decode_cpr_global(even: &CprFrame, odd: &CprFrame) -> Option<(f64, f64)> {
    if even.is_odd || !odd.is_odd {
        return None;
    }

    let y0 = (even.cpr_lat as f64) / 131_072.0;
    let y1 = (odd.cpr_lat as f64) / 131_072.0;

    let d_lat0 = 360.0 / 60.0;
    let d_lat1 = 360.0 / 59.0;

    let j = (59.0 * y0 - 60.0 * y1 + 0.5).floor();
    let mut lat0 = d_lat0 * (j.rem_euclid(60.0) + y0);
    let mut lat1 = d_lat1 * (j.rem_euclid(59.0) + y1);

    if lat0 >= 270.0 { lat0 -= 360.0; }
    if lat1 >= 270.0 { lat1 -= 360.0; }

    // Use most recent frame for final coordinates
    let (chosen_lat, chosen_frame) = if even.timestamp_ms >= odd.timestamp_ms {
        (lat0, even)
    } else {
        (lat1, odd)
    };

    let nl = nl_table(chosen_lat);
    let ni = if chosen_frame.is_odd { nl.saturating_sub(1).max(1) } else { nl.max(1) };
    let d_lon = 360.0 / (ni as f64);

    let x0 = (even.cpr_lon as f64) / 131_072.0;
    let x1 = (odd.cpr_lon as f64) / 131_072.0;

    let m = if chosen_frame.is_odd {
        ((x0 * ((nl - 1) as f64) - x1 * (nl as f64)) + 0.5).floor()
    } else {
        ((x0 * ((nl - 1) as f64) - x1 * (nl as f64)) + 0.5).floor()
    };

    let chosen_x = if chosen_frame.is_odd { x1 } else { x0 };
    let mut lon = d_lon * (m.rem_euclid(ni as f64) + chosen_x);
    if lon >= 180.0 {
        lon -= 360.0;
    }

    Some((chosen_lat, lon))
}

fn nl_table(lat: f64) -> usize {
    let lat_abs = lat.abs();
    if lat_abs >= 87.0 {
        1
    } else if lat_abs >= 80.0 {
        2
    } else if lat_abs >= 70.0 {
        4
    } else if lat_abs >= 60.0 {
        8
    } else if lat_abs >= 45.0 {
        15
    } else if lat_abs >= 30.0 {
        25
    } else {
        59
    }
}
