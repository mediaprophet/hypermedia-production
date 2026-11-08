//! Preflight compliance evaluator for PDF/A and PDF/X standards.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StandardProfile {
    PdfA1b,
    PdfA2b,
    PdfA3b,
    PdfX1a,
    PdfX3,
    PdfX4,
}

impl StandardProfile {
    pub fn label(self) -> &'static str {
        match self {
            StandardProfile::PdfA1b => "PDF/A-1b (ISO 19005-1)",
            StandardProfile::PdfA2b => "PDF/A-2b (ISO 19005-2)",
            StandardProfile::PdfA3b => "PDF/A-3b (ISO 19005-3)",
            StandardProfile::PdfX1a => "PDF/X-1a (ISO 15930-1)",
            StandardProfile::PdfX3 => "PDF/X-3 (ISO 15930-3)",
            StandardProfile::PdfX4 => "PDF/X-4 (ISO 15930-7)",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PreflightIssue {
    pub standard: StandardProfile,
    pub clause: &'static str,
    pub message: String,
    pub page: Option<usize>,
    pub fixable: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PreflightSnapshot {
    pub pdfa_part: Option<u8>,
    pub pdfa_conformance: Option<String>,
    pub pdfx_version: Option<String>,
    pub is_encrypted: bool,
    pub uses_lzw: bool,
    pub unembedded_font_count: usize,
    pub output_intent_identifier: Option<String>,
    pub forbidden_actions_found: Vec<String>,
    pub forbidden_annotations_found: Vec<String>,
    pub has_rgb_content: bool,
    pub has_trim_box: bool,
    pub has_bleed_box: bool,
    pub trapped_key_present: bool,
    pub has_unattached_embedded_files: bool, // PDF/A-3 requires AFRelationship
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PreflightReport {
    pub profile: StandardProfile,
    pub compliant: bool,
    pub issues: Vec<PreflightIssue>,
}

pub fn evaluate_preflight(snapshot: &PreflightSnapshot, profile: StandardProfile) -> PreflightReport {
    let mut issues = Vec::new();

    match profile {
        StandardProfile::PdfA1b | StandardProfile::PdfA2b | StandardProfile::PdfA3b => {
            evaluate_pdfa(snapshot, profile, &mut issues);
        }
        StandardProfile::PdfX1a | StandardProfile::PdfX3 | StandardProfile::PdfX4 => {
            evaluate_pdfx(snapshot, profile, &mut issues);
        }
    }

    let compliant = issues.is_empty();
    PreflightReport {
        profile,
        compliant,
        issues,
    }
}

fn evaluate_pdfa(snapshot: &PreflightSnapshot, profile: StandardProfile, issues: &mut Vec<PreflightIssue>) {
    let target_part = match profile {
        StandardProfile::PdfA1b => 1,
        StandardProfile::PdfA2b => 2,
        StandardProfile::PdfA3b => 3,
        _ => 2,
    };

    // 1. Identification
    if snapshot.pdfa_part != Some(target_part) {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "6.7.2",
            message: format!("XMP metadata lacks required pdfaid:part={target_part}"),
            page: None,
            fixable: true,
        });
    }

    // 2. Encryption
    if snapshot.is_encrypted {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "6.1.4",
            message: "Document must not be encrypted or password-protected".into(),
            page: None,
            fixable: true,
        });
    }

    // 3. Compression
    if snapshot.uses_lzw {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "6.1.5",
            message: "LZW compression is forbidden in PDF/A".into(),
            page: None,
            fixable: true,
        });
    }

    // 4. Fonts
    if snapshot.unembedded_font_count > 0 {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "6.2.11",
            message: format!("{} fonts are not embedded in the PDF document", snapshot.unembedded_font_count),
            page: None,
            fixable: false,
        });
    }

    // 5. Output Intent
    if snapshot.output_intent_identifier.is_none() {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "6.2.2",
            message: "GTS_PDFA1 Output Intent with embedded ICC profile is required".into(),
            page: None,
            fixable: true,
        });
    }

    // 6. Forbidden actions
    for action in &snapshot.forbidden_actions_found {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "6.6.1",
            message: format!("Forbidden interactive action `{action}` present in document"),
            page: None,
            fixable: true,
        });
    }

    // 7. Embedded files
    if profile == StandardProfile::PdfA2b && snapshot.has_unattached_embedded_files {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "6.8.1",
            message: "Embedded files in PDF/A-2 must themselves conform to PDF/A-1 or PDF/A-2".into(),
            page: None,
            fixable: false,
        });
    }
}

fn evaluate_pdfx(snapshot: &PreflightSnapshot, profile: StandardProfile, issues: &mut Vec<PreflightIssue>) {
    // 1. Encryption forbidden
    if snapshot.is_encrypted {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "ISO 15930 §4.1",
            message: "Encryption is forbidden in PDF/X graphic arts exchange".into(),
            page: None,
            fixable: true,
        });
    }

    // 2. Unembedded fonts forbidden
    if snapshot.unembedded_font_count > 0 {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "ISO 15930 §4.3",
            message: format!("{} fonts are not fully embedded in PDF/X", snapshot.unembedded_font_count),
            page: None,
            fixable: false,
        });
    }

    // 3. TrimBox or ArtBox required
    if !snapshot.has_trim_box {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "ISO 15930 §5.2",
            message: "Pages must specify TrimBox or ArtBox for prepress trimming".into(),
            page: None,
            fixable: true,
        });
    }

    // 4. PDF/X-1a specific: No RGB
    if profile == StandardProfile::PdfX1a && snapshot.has_rgb_content {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "ISO 15930-1 §4.2",
            message: "PDF/X-1a restricts color spaces to DeviceCMYK, DeviceGray, and spot separations (no RGB)".into(),
            page: None,
            fixable: true,
        });
    }

    // 5. Output Intent required
    if snapshot.output_intent_identifier.is_none() {
        issues.push(PreflightIssue {
            standard: profile,
            clause: "ISO 15930 §6.1",
            message: "PDF/X requires a valid prepress Output Intent (e.g. FOGRA39, CGATS SWOP)".into(),
            page: None,
            fixable: true,
        });
    }
}
