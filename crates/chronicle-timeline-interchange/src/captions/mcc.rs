//! MacCaption MCC (`.mcc`) closed caption format parser and exporter.
//!
//! Grounded in SMPTE ST 334-2 (Caption Distribution Packet) and CTA-608 / CTA-708 broadcast delivery.

use std::fmt::Write as _;

use crate::captions::{CaptionCue, CaptionDocument};
use crate::time::{FrameRate, Tick, format_timecode, parse_timecode};

pub fn parse_mcc(text: &str) -> Result<CaptionDocument, String> {
    let mut doc = CaptionDocument::default();
    let rate = FrameRate::FPS_29_97;
    let mut cur_cue_text = String::new();
    let mut cur_start = Tick::ZERO;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("File Format=") || line.starts_with("//") {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 2 {
            continue;
        }

        let tc_str = parts[0];
        let Some((frames, _)) = parse_timecode(tc_str, rate) else {
            continue;
        };
        let t = rate.tick_of(frames);

        // Scan hex pairs for readable characters
        let data_str = parts[1];
        let mut line_text = String::new();
        let bytes = decode_mcc_hex(data_str);

        // Look for printable ASCII pairs in cc_data
        for w in bytes.windows(2) {
            let b1 = w[0] & 0x7f;
            let b2 = w[1] & 0x7f;
            if (0x20..=0x7e).contains(&b1) && (0x20..=0x7e).contains(&b2) {
                line_text.push(b1 as char);
                line_text.push(b2 as char);
            }
        }

        let trimmed = line_text.trim();
        if !trimmed.is_empty() {
            if cur_cue_text.is_empty() {
                cur_start = t;
                cur_cue_text = trimmed.to_string();
            } else if trimmed != cur_cue_text {
                doc.cues.push(CaptionCue {
                    id: format!("{}", doc.cues.len() + 1),
                    start: cur_start,
                    end: t,
                    text: cur_cue_text.clone(),
                    settings: String::new(),
                });
                cur_start = t;
                cur_cue_text = trimmed.to_string();
            }
        } else if !cur_cue_text.is_empty() {
            doc.cues.push(CaptionCue {
                id: format!("{}", doc.cues.len() + 1),
                start: cur_start,
                end: t,
                text: cur_cue_text.clone(),
                settings: String::new(),
            });
            cur_cue_text.clear();
        }
    }

    Ok(doc)
}

fn decode_mcc_hex(s: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            'Z' => {
                bytes.push(0x00);
                i += 1;
            }
            'S' => {
                bytes.extend_from_slice(&[0x96, 0x69]);
                i += 1;
            }
            'T' => {
                bytes.extend_from_slice(&[0x61, 0x01]);
                i += 1;
            }
            'P' => {
                bytes.extend_from_slice(&[0xfb, 0x80, 0x80]);
                i += 1;
            }
            'Q' => {
                bytes.extend_from_slice(&[0xfc, 0x80, 0x80]);
                i += 1;
            }
            'R' => {
                bytes.extend_from_slice(&[0xfd, 0x80, 0x80]);
                i += 1;
            }
            'G'..='O' => {
                let count = (c as u8 - b'G' + 1) as usize;
                for _ in 0..count {
                    bytes.extend_from_slice(&[0xfa, 0x00, 0x00]);
                }
                i += 1;
            }
            _ => {
                if i + 1 < chars.len() {
                    let hex_str: String = chars[i..=i + 1].iter().collect();
                    if let Ok(b) = u8::from_str_radix(&hex_str, 16) {
                        bytes.push(b);
                        i += 2;
                        continue;
                    }
                }
                i += 1;
            }
        }
    }
    bytes
}

pub fn write_mcc(doc: &CaptionDocument, rate: FrameRate) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "File Format=MacCaption_MCC V1.0");
    let _ = writeln!(out, "Time Code Rate=30DF\n");

    for cue in &doc.cues {
        let start_f = rate.frame_at(cue.start);
        let tc = format_timecode(start_f, rate, true);
        let mut hex_data = String::from("T");
        for b in cue.text.bytes() {
            let _ = write!(hex_data, "{:02X}", crate::captions::cea608::with_parity(b));
        }
        let _ = writeln!(out, "{tc}\t{hex_data}");
    }

    out
}
