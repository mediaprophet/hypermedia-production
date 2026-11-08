//! SubRip (`.srt`) closed caption parser and serializer.
//!
//! Grounded in Matroska SubRip conventions and W3C accessibility guidelines.

use std::fmt::Write as _;

use crate::captions::{CaptionCue, CaptionDocument};
use crate::time::{Tick, format_clock, parse_clock};

pub fn parse_srt(text: &str) -> Result<CaptionDocument, String> {
    let mut doc = CaptionDocument::default();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }

        // Check if this line is an index number, followed by a timing line
        let (start, end) = if line.contains("-->") {
            parse_timing_line(line)?
        } else if i + 1 < lines.len() && lines[i + 1].contains("-->") {
            i += 1;
            parse_timing_line(lines[i].trim())?
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
                id: format!("{}", doc.cues.len() + 1),
                start,
                end,
                text: body,
                settings: String::new(),
            });
        }
    }

    Ok(doc)
}

fn parse_timing_line(line: &str) -> Result<(Tick, Tick), String> {
    let (a, b) = line
        .split_once("-->")
        .ok_or_else(|| format!("Invalid timing line: {line}"))?;
    let start = parse_clock(a.trim()).ok_or_else(|| format!("Invalid start timecode: {a}"))?;
    let b_end = b
        .split_whitespace()
        .next()
        .ok_or_else(|| format!("Missing end timecode: {b}"))?;
    let end = parse_clock(b_end).ok_or_else(|| format!("Invalid end timecode: {b_end}"))?;
    Ok((start, end))
}

pub fn write_srt(doc: &CaptionDocument) -> String {
    let mut out = String::new();
    for (idx, cue) in doc.cues.iter().enumerate() {
        let _ = writeln!(out, "{}", idx + 1);
        let start_s = format_clock(cue.start, true);
        let end_s = format_clock(cue.end, true);
        let _ = writeln!(out, "{start_s} --> {end_s}");
        let _ = writeln!(out, "{}", cue.text);
        let _ = writeln!(out);
    }
    out
}
