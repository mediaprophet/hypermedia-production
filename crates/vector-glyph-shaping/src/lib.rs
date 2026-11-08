//! # Vector Glyph Shaping & Typography Engine
//!
//! A pure Rust, UI-agnostic categorical typography domain crate providing OpenType
//! feature selection, UAX #9 Bidirectional algorithm run reordering, Knuth-Plass
//! optimal total-fit paragraph composition, syllable hyphenation, area text /
//! threaded frame / text-on-path layout, and TrueType/CFF font subsetting.
//!
//! Grounded in international human rights guarantees of linguistic diversity,
//! artistic expression, and cultural heritage (UDHR Articles 19 & 27; ICESCR Article 15)
//! and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod bidi;
pub mod composer;
pub mod features;
pub mod hyphen;
pub mod layout;
pub mod subset;
pub mod thread;

pub use bidi::{bidi_class, reorder_visual, resolve_bidi_runs, BidiClass, BidiRun, Direction as BidiDirection};
pub use composer::{compose, Breakpoint, Params as ComposerParams, SGlyph};
pub use features::{OtFeatures, LIGATURE_TRACKING_LIMITS};
pub use hyphen::{
    hyphen_points, hyphenate_word, hyphenation_exceptions, set_hyphenation_exceptions, MIN_AFTER, MIN_BEFORE, MIN_WORD,
};
pub use layout::{
    layout_area_text, layout_text_on_path, AreaTextOptions, FirstBaseline, LayoutResult, LineBox, PathAlign,
    TextOnPathOptions,
};
pub use subset::{build_sfnt_font, calc_table_checksum, parse_sfnt_directory, SfntTable};
pub use thread::{distribute, story, Slice as ThreadSlice, TextFrame, TextRun};

#[cfg(test)]
mod tests {
    use super::*;
    use kurbo::Rect;

    #[test]
    fn full_typography_pipeline_smoke() {
        // 1. Hyphenation
        let word = "typography";
        let pts = hyphen_points(word);
        assert_eq!(pts, vec![4, 7]);

        // 2. OpenType features
        let features = OtFeatures::from_tags(["dlig", "smcp"]);
        assert!(features.discretionary_ligatures);
        assert!(features.small_caps);

        // 3. BiDi analysis
        let (dir, runs) = resolve_bidi_runs("Hello world", None);
        assert_eq!(dir, BidiDirection::Ltr);
        assert_eq!(runs.len(), 1);

        // 4. Area text layout
        let res = layout_area_text(
            Rect::new(0.0, 0.0, 200.0, 100.0),
            "Typography is the art and technique of arranging type.",
            &AreaTextOptions::default(),
        );
        assert!(!res.lines.is_empty());

        // 5. Knuth-Plass composition
        let glyphs: Vec<SGlyph> = (0..10).map(|_| SGlyph::char_glyph(10.0)).collect();
        let breaks = compose(&glyphs, &|_| 60.0, &|_| 0.0, &[], &ComposerParams::default());
        assert!(breaks.is_some());
    }
}
