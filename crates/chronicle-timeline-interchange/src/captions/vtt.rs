//! WebVTT (`.vtt`) closed caption and subtitle parser and writer.
//!
//! Grounded in W3C WebVTT: The Web Video Text Tracks Format (W3C Recommendation).

use std::fmt::Write as _;

use crate::captions::{CaptionCue, CaptionDocument};
use crate::time::{Tick, format_clock, parse_clock};

pub fn parse_vtt(text: &str) -> Result<CaptionDocument, String> {
    let mut doc = CaptionDocument::default();
    let lines: Vec<&str> = text.lines().collect();

    let mut i = 0;
    // Skip WEBVTT header
    while i < lines.len() {
        let l = lines[i].trim();
        i += 1;
        if l.starts_with("WEBVTT") {
            break;
        }
    }

    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.starts_with("NOTE") || line.starts_with("STYLE") || line.starts_with("REGION") {
            // Skip block until empty line
            while i < lines.len() && !lines[i].trim().is_empty() {
                i += 1;
            }
            continue;
        }

        let mut id = String::new();
        let (start, end, settings) = if line.contains("-->") {
            parse_vtt_timing_line(line)?
        } else if i + 1 < lines.len() && lines[i + 1].contains("-->") {
            id = line.to_string();
            i += 1;
            parse_vtt_timing_line(lines[i].trim())?
        } else {
            i += 1;
            continue;
        };

        i += 1;
        let mut text_lines = Vec::new();
        while i < lines.len() {
            let l = lines[i];
            if l.trim().is_empty() || l.contains("-->") {
                break;
            }
            text_lines.push(l.trim());
            i += 1;
        }

        let body = text_lines.join("\n");
        if !body.is_empty() {
            doc.cues.push(CaptionCue {
                id: if id.is_empty() {
                    format!("{}", doc.cues.len() + 1)
                } else {
                    id
                },
                start,
                end,
                text: body,
                settings,
            });
        }
    }

    Ok(doc)
}

fn parse_vtt_timing_line(line: &str) -> Result<(Tick, Tick, String), String> {
    let (a, rest) = line
        .split_once("-->")
        .ok_or_else(|| format!("Invalid VTT timing line: {line}"))?;
    let start = parse_clock(a.trim()).ok_or_else(|| format!("Invalid start timecode: {a}"))?;
    let rest = rest.trim_start();
    let end_len = rest.find(char::is_whitespace).unwrap_or(rest.len());
    let end = parse_clock(&rest[..end_len]).ok_or_else(|| format!("Invalid end timecode: {}", &rest[..end_len]))?;
    let settings = rest[end_len..].trim().to_string();
    Ok((start, end, settings))
}

pub fn write_vtt(doc: &CaptionDocument) -> String {
    let mut out = String::from("WEBVTT\n\n");
    for cue in &doc.cues {
        if !cue.id.is_empty() {
            let _ = writeln!(out, "{}", cue.id);
        }
        let start_s = format_clock(cue.start, false);
        let end_s = format_clock(cue.end, false);
        if !cue.settings.is_empty() {
            let _ = writeln!(out, "{start_s} --> {end_s} {}", cue.settings);
        } else {
            let _ = writeln!(out, "{start_s} --> {end_s}");
        }
        let _ = writeln!(out, "{}", cue.text);
        let _ = writeln!(out);
    }
    out
}
