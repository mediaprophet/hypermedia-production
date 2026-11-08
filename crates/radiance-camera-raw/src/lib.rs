//! # Radiance Camera RAW & Computational Sensor Optics
//!
//! Provides pure-Rust, memory-bounded, zero-unsafe algorithms for:
//! - Camera sensor CFA demosaicing (Bayer RGGB/BGGR/GRBG/GBRG with Bilinear, AHD, and RCD, plus Fujifilm 6x6 X-Trans)
//! - Sensor black/white level calibration and dual-illuminant color matrix white balance
//! - Non-destructive parametric RAW development pipeline (EV, WB, tonal recovery, tone curves, 8-band HSL, split toning, detail, vignette, radial distortion)
//! - Multi-exposure bracket HDR radiance map fusion with motion deghosting
//! - Multi-image panorama stitching with multi-band Burt-Adelson Laplacian pyramid blending
//! - Metadata structures (EXIF, XMP, IPTC), GPX tracklog geotag interpolation with meridian wrapping
//! - Reversible append-only catalog journal with undo/redo and smart album boolean query evaluation
//!
//! Execution and provenance guarantees are grounded in international human rights instruments
//! (UDHR Articles 12, 18, 19, 27; ICCPR Articles 17, 19; ICESCR Article 15) and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod demosaic;
pub mod develop;
pub mod fusion;
pub mod metadata;
pub mod ontology;

pub use demosaic::{demosaic_raw, demosaic_xtrans, CfaPattern, DemosaicAlgo};
pub use develop::{
    develop_image, DetailSettings, DevelopSettings, HslBand, HslMixer, HslMixer as HslAdjustment,
    LensCorrections, ParametricToneCurve, ParametricToneCurve as ToneCurve, SplitToning,
    ToneSettings, WhiteBalance,
};
pub use fusion::{merge_hdr, stitch_panorama, ExposureBracket, HdrOptions};
pub use metadata::{
    CatalogJournal, ExifMetadata, GpsCoord, GpxTracklog, IptcMetadata, JournalAction, PhotoRecord,
    SmartRule, TrackPoint, XmpMetadata,
};
pub use ontology::{
    AgenticMorphismSpec, FuncDescriptor, HDR_FUSION_MORPHISM, RAW_DEMOSAIC_MORPHISM,
    RAW_DEVELOP_MORPHISM,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bayer_demosaic_rggb() {
        let width = 6;
        let height = 6;
        let black = 512;
        let white = 16384;
        let mut raw = vec![black; width * height];

        // Put red at (0,0), green at (0,1), green at (1,0), blue at (1,1)
        raw[0] = 8192; // R
        raw[1] = 4096; // G
        raw[width] = 4096; // G
        raw[width + 1] = 2048; // B

        let result = demosaic_raw(
            &raw,
            width,
            height,
            black,
            white,
            CfaPattern::Rggb,
            DemosaicAlgo::Bilinear,
            5000.0,
        )
        .expect("demosaic should succeed");

        assert_eq!(result.len(), width * height);
        assert!(result[0][0] > 0.0, "Red channel must be reconstructed");
    }

    #[test]
    fn test_xtrans_demosaic() {
        let width = 6;
        let height = 6;
        let raw = vec![1000u16; width * height];
        let rgb = demosaic_xtrans(&raw, width, height, 0, 4000);
        assert_eq!(rgb.len(), width * height);
        assert!(rgb[0][0] >= 0.0 && rgb[0][0] <= 1.0);
    }

    #[test]
    fn test_develop_pipeline_and_hash_determinism() {
        let width = 4;
        let height = 4;
        let input = vec![[0.2f32, 0.3, 0.4]; width * height];

        let mut settings = DevelopSettings::default();
        settings.exposure = 0.5;
        settings.highlights = -20.0;
        settings.shadows = 15.0;
        settings.clarity = 10.0;
        settings.lens.vignette_amount = -30.0;

        let hash1 = settings.compute_hash();
        let hash2 = settings.compute_hash();
        assert_eq!(hash1, hash2, "DevelopSettings hash must be deterministic");

        let developed = develop_image(&input, width, height, &settings);
        assert_eq!(developed.len(), input.len());
        assert_ne!(
            developed[0], input[0],
            "Developed pixels must reflect parameter adjustments"
        );
    }

    #[test]
    fn test_hdr_fusion_deghosting() {
        let width = 4;
        let height = 4;
        let bracket1 = ExposureBracket {
            image: vec![[0.1f32, 0.1, 0.1]; width * height],
            width,
            height,
            exposure_time: 0.01,
            iso: 100.0,
        };
        let bracket2 = ExposureBracket {
            image: vec![[0.4f32, 0.4, 0.4]; width * height],
            width,
            height,
            exposure_time: 0.04,
            iso: 100.0,
        };

        let options = HdrOptions {
            deghosting: true,
            reference_bracket: 0,
            tonemap: true,
        };

        let hdr = merge_hdr(&[bracket1, bracket2], &options).expect("HDR merge should succeed");
        assert_eq!(hdr.len(), width * height);
        for px in &hdr {
            assert!(px[0] >= 0.0 && px[0] <= 1.0);
        }
    }

    #[test]
    fn test_gpx_tracklog_interpolation_and_meridian_wrap() {
        let mut tracklog = GpxTracklog::new();
        tracklog.add_point(TrackPoint {
            latitude: -33.8688,
            longitude: 179.9,
            elevation: 10.0,
            timestamp_secs: 1000.0,
        });
        tracklog.add_point(TrackPoint {
            latitude: -33.8688,
            longitude: -179.9,
            elevation: 20.0,
            timestamp_secs: 1020.0,
        });

        let interp = tracklog
            .interpolate(1010.0)
            .expect("Interpolation midpoint should exist");
        assert_eq!(interp.2, 15.0, "Elevation must be linearly interpolated");
        // Crosses 180 meridian: 179.9 to -179.9 (+180.1) has diff +0.2 deg.
        // Midpoint should wrap correctly around 180 / -180.
        assert!(interp.1.abs() >= 179.0, "Longitude should handle meridian wrap");
    }

    #[test]
    fn test_catalog_journal_undo_redo() {
        let mut journal = CatalogJournal::new();
        journal.record_action(
            "img_001",
            JournalAction::SetRating {
                old_val: 0,
                new_val: 5,
            },
        );
        journal.record_action(
            "img_001",
            JournalAction::AddTag {
                tag: "landscape".to_string(),
            },
        );

        assert!(journal.can_undo());
        let undone = journal.undo().expect("Undo should succeed");
        assert_eq!(undone.image_id, "img_001");
        match undone.action {
            JournalAction::AddTag { ref tag } => assert_eq!(tag, "landscape"),
            _ => panic!("Expected AddTag action undone"),
        }

        assert!(journal.can_redo());
        let redone = journal.redo().expect("Redo should succeed");
        assert_eq!(redone.image_id, "img_001");
    }

    #[test]
    fn test_smart_rule_evaluation() {
        let mut record = PhotoRecord {
            id: "photo_1".to_string(),
            filename: "sunset.raw".to_string(),
            rating: 4,
            flagged: false,
            color_label: "Yellow".to_string(),
            tags: vec!["golden_hour".to_string(), "coast".to_string()],
            iso: 200.0,
            capture_timestamp: 1600000000.0,
        };

        let rule_rating = SmartRule::RatingGte(3);
        let rule_tag = SmartRule::HasTag("coast".to_string());
        let rule_flag = SmartRule::IsFlagged;

        assert!(rule_rating.matches(&record));
        assert!(rule_tag.matches(&record));
        assert!(!rule_flag.matches(&record));

        record.flagged = true;
        assert!(rule_flag.matches(&record));
    }

    #[test]
    fn test_ontology_contracts() {
        let descriptors = get_descriptors();
        assert_eq!(descriptors.len(), 4);
        for d in descriptors {
            assert_eq!(d.contract.triples.len(), 3);
            for quin in &d.contract.triples {
                assert!(quin.verify_parity());
            }
        }
    }
}
