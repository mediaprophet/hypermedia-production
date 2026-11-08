//! OpenType feature selection and tag resolution per character style.
//!
//! Grounded in typographic heritage and linguistic cultural rights (UDHR Art. 19, 27; ICESCR Art. 15).

use serde::{Deserialize, Serialize};

/// Tracking limits (in 1/1000 em) outside which standard ligatures are dropped automatically.
pub const LIGATURE_TRACKING_LIMITS: (f64, f64) = (-20.0, 55.0);

/// OpenType features applied during glyph shaping.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtFeatures {
    /// Vertical glyph alternates (`vert`).
    pub vertical: bool,
    /// Standard ligatures (`liga`, `clig`).
    pub ligatures: bool,
    /// Contextual alternates (`calt`).
    pub contextual: bool,
    /// Discretionary ligatures (`dlig`).
    pub discretionary_ligatures: bool,
    /// Small capitals (`smcp`).
    pub small_caps: bool,
    /// Diagonal fractions (`frac`).
    pub fractions: bool,
    /// Oldstyle figures (`onum`).
    pub oldstyle_figures: bool,
    /// Tabular figures (`tnum`).
    pub tabular_figures: bool,
    /// Ordinals (`ordn`).
    pub ordinals: bool,
    /// Swashes (`swsh`).
    pub swash: bool,
}

impl Default for OtFeatures {
    fn default() -> Self {
        Self {
            vertical: false,
            ligatures: true,
            contextual: true,
            discretionary_ligatures: false,
            small_caps: false,
            fractions: false,
            oldstyle_figures: false,
            tabular_figures: false,
            ordinals: false,
            swash: false,
        }
    }
}

impl OtFeatures {
    /// Parse a feature list like `["dlig", "smcp", "-liga"]`.
    pub fn from_tags<'a>(tags: impl IntoIterator<Item = &'a str>) -> Self {
        let mut f = Self::default();
        for tag in tags {
            let (on, t) = if let Some(stripped) = tag.strip_prefix('-') {
                (false, stripped)
            } else if let Some(stripped) = tag.strip_prefix('+') {
                (true, stripped)
            } else {
                (true, tag)
            };
            match t {
                "vert" => f.vertical = on,
                "liga" | "clig" => f.ligatures = on,
                "calt" => f.contextual = on,
                "dlig" => f.discretionary_ligatures = on,
                "smcp" => f.small_caps = on,
                "frac" => f.fractions = on,
                "onum" => f.oldstyle_figures = on,
                "tnum" => f.tabular_figures = on,
                "ordn" => f.ordinals = on,
                "swsh" => f.swash = on,
                _ => {}
            }
        }
        f
    }

    /// List of tag strings that differ from default settings.
    pub fn to_tags(&self) -> Vec<String> {
        let d = Self::default();
        let mut v = Vec::new();
        for (on, def, tag) in [
            (self.ligatures, d.ligatures, "liga"),
            (self.contextual, d.contextual, "calt"),
            (self.discretionary_ligatures, d.discretionary_ligatures, "dlig"),
            (self.small_caps, d.small_caps, "smcp"),
            (self.fractions, d.fractions, "frac"),
            (self.oldstyle_figures, d.oldstyle_figures, "onum"),
            (self.tabular_figures, d.tabular_figures, "tnum"),
            (self.ordinals, d.ordinals, "ordn"),
            (self.swash, d.swash, "swsh"),
        ] {
            if on != def {
                v.push(if on { tag.to_string() } else { format!("-{tag}") });
            }
        }
        v
    }

    /// Check if a tag name is recognized.
    pub fn known_tag(tag: &str) -> bool {
        let t = tag.trim_start_matches(['-', '+']);
        matches!(
            t,
            "vert" | "liga" | "clig" | "calt" | "dlig" | "smcp" | "frac" | "onum" | "tnum" | "ordn" | "swsh"
        )
    }

    /// Determine if standard ligatures should be kept given tracking value.
    pub fn should_keep_ligatures(&self, tracking: f64) -> bool {
        if !self.ligatures {
            return false;
        }
        tracking >= LIGATURE_TRACKING_LIMITS.0 && tracking <= LIGATURE_TRACKING_LIMITS.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_roundtrip_tags() {
        let tags = ["dlig", "smcp", "-liga"];
        let f = OtFeatures::from_tags(tags);
        assert!(f.discretionary_ligatures);
        assert!(f.small_caps);
        assert!(!f.ligatures);
        let out = f.to_tags();
        assert!(out.contains(&"dlig".to_string()));
        assert!(out.contains(&"smcp".to_string()));
        assert!(out.contains(&"-liga".to_string()));
    }

    #[test]
    fn tracking_ligature_limits() {
        let f = OtFeatures::default();
        assert!(f.should_keep_ligatures(0.0));
        assert!(f.should_keep_ligatures(50.0));
        assert!(!f.should_keep_ligatures(100.0));
        assert!(!f.should_keep_ligatures(-30.0));
    }
}
