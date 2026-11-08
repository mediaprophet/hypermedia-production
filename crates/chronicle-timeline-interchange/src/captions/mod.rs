//! Caption formats (SRT, WebVTT, CEA-608, CEA-708, and MacCaption MCC).

pub mod align;
pub mod cea608;
pub mod cea708;
pub mod mcc;
pub mod srt;
pub mod vtt;

use serde::{Deserialize, Serialize};

use crate::time::Tick;

pub use align::{AlignOptions, align_cues};
pub use cea608::{Cea608Command, ScreenBuffer, decode_pair, strip_parity, with_parity};
pub use cea708::{DtvccCommand, DtvccWindow};
pub use mcc::{parse_mcc, write_mcc};
pub use srt::{parse_srt, write_srt};
pub use vtt::{parse_vtt, write_vtt};

/// A single timed closed caption or subtitle cue.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptionCue {
    pub id: String,
    pub start: Tick,
    pub end: Tick,
    pub text: String,
    pub settings: String,
}

/// A collection of timed caption cues.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptionDocument {
    pub cues: Vec<CaptionCue>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CaptionFormat {
    Srt,
    WebVtt,
    Mcc,
}

pub fn detect_caption_format(text: &str) -> Option<CaptionFormat> {
    let t = text.trim();
    if t.starts_with("WEBVTT") {
        Some(CaptionFormat::WebVtt)
    } else if t.starts_with("File Format=MacCaption") {
        Some(CaptionFormat::Mcc)
    } else if t.contains("-->") {
        Some(CaptionFormat::Srt)
    } else {
        None
    }
}
