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
    AgenticMorphismSpec, FuncDescriptor, GPX_GEOTAG_MORPHISM, HDR_FUSION_MORPHISM,
    RAW_DEMOSAIC_MORPHISM, RAW_DEVELOP_MORPHISM,
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
            CfaPattern::Rggb,
            black,
            white,
            DemosaicAlgo::Bilinear,
        );

        assert_eq!(result.len(), width * height * 3);
        assert!(result[0] > 0.0, "Red channel must be reconstructed");
    }

    #[test]
    fn test_xtrans_demosaic() {
        let width = 6;
        let height = 6;
        let normalized = demosaic::normalize_raw(&vec![1000u16; width * height], 0, 4000);
        let rgb = demosaic_xtrans(&normalized, width, height);
        assert_eq!(rgb.len(), width * height * 3);
        assert!(rgb[0] >= 0.0 && rgb[0] <= 1.0);
    }

    #[test]
    fn test_develop_pipeline_and_hash_determinism() {
        let width = 4;
        let height = 4;
        let mut input = Vec::with_capacity(width * height * 3);
        for _ in 0..(width * height) {
            input.extend_from_slice(&[0.2f32, 0.3, 0.4]);
        }

        let mut settings = DevelopSettings::default();
        settings.wb.temp = 6500.0;
        settings.tone.exposure = 0.5;
        settings.tone.highlights = -20.0;
        settings.tone.shadows = 15.0;
        settings.detail.clarity = 10.0;
        settings.lens.vignette_amount = -30.0;

        let hash1 = settings.cache_digest();
        let hash2 = settings.cache_digest();
        assert_eq!(hash1, hash2, "DevelopSettings hash must be deterministic");

        let developed = develop_image(&input, width, height, &settings);
        assert_eq!(developed.len(), input.len());
        assert_ne!(
            &developed[0..3],
            &input[0..3],
            "Developed pixels must reflect parameter adjustments"
        );
    }

    #[test]
    fn test_hdr_fusion_deghosting() {
        let width = 4;
        let height = 4;
        let bracket1 = ExposureBracket {
            rgb: vec![0.2f32; width * height * 3],
            exposure_time: 0.01,
        };
        let bracket2 = ExposureBracket {
            rgb: vec![0.4f32; width * height * 3],
            exposure_time: 0.04,
        };

        let options = HdrOptions {
            deghost_threshold: 0.5,
            reference_index: 0,
        };

        let hdr = merge_hdr(&[bracket1, bracket2], width, height, options)
            .expect("HDR merge should succeed");
        assert_eq!(hdr.len(), width * height * 3);
        for &ch in &hdr {
            assert!(ch >= 0.0 && ch.is_finite());
        }
    }

    #[test]
    fn test_gpx_tracklog_interpolation_and_meridian_wrap() {
        let mut tracklog = GpxTracklog::new();
        tracklog.add_point(TrackPoint {
            time: 1000.0,
            latitude: -33.8688,
            longitude: 179.9,
            altitude: Some(10.0),
        });
        tracklog.add_point(TrackPoint {
            time: 1020.0,
            latitude: -33.8688,
            longitude: -179.9,
            altitude: Some(20.0),
        });

        let interp = tracklog
            .interpolate_position(1010.0, 60.0)
            .expect("Interpolation midpoint should exist");
        assert_eq!(
            interp.altitude,
            Some(15.0),
            "Elevation must be linearly interpolated"
        );
        // Crosses 180 meridian: 179.9 to -179.9 (+180.1) has diff +0.2 deg.
        // Midpoint should wrap correctly around 180 / -180.
        assert!(
            interp.longitude.abs() >= 179.0,
            "Longitude should handle meridian wrap"
        );
    }

    #[test]
    fn test_catalog_journal_undo_redo() {
        let mut journal = CatalogJournal::new();
        journal.record_action(JournalAction::SetRating {
            photo_id: 1,
            old_rating: 0,
            new_rating: 5,
        });
        journal.record_action(JournalAction::AddTag {
            photo_id: 1,
            tag: "landscape".to_string(),
        });

        assert_eq!(journal.history.len(), 2);
        let undone = journal.undo().expect("Undo should succeed");
        match undone {
            JournalAction::RemoveTag { photo_id, ref tag } => {
                assert_eq!(photo_id, 1);
                assert_eq!(tag, "landscape");
            }
            _ => panic!("Expected inverse RemoveTag action on undo"),
        }

        assert_eq!(journal.redo_stack.len(), 1);
        let redone = journal.redo().expect("Redo should succeed");
        assert_eq!(
            redone,
            JournalAction::AddTag {
                photo_id: 1,
                tag: "landscape".to_string(),
            }
        );
    }

    #[test]
    fn test_smart_rule_evaluation() {
        let mut record = PhotoRecord {
            id: 1,
            rating: 4,
            tags: vec!["golden_hour".to_string(), "coast".to_string()],
            album: Some("sunset".to_string()),
        };

        let rule_rating = SmartRule::RatingGte(3);
        let rule_tag = SmartRule::HasTag("coast".to_string());
        let rule_album = SmartRule::InAlbum("featured".to_string());

        assert!(rule_rating.matches(&record));
        assert!(rule_tag.matches(&record));
        assert!(!rule_album.matches(&record));

        record.album = Some("featured".to_string());
        assert!(rule_album.matches(&record));
    }

    #[test]
    fn test_ontology_contracts() {
        let descriptors = [
            &RAW_DEMOSAIC_MORPHISM,
            &RAW_DEVELOP_MORPHISM,
            &HDR_FUSION_MORPHISM,
            &GPX_GEOTAG_MORPHISM,
        ];
        assert_eq!(descriptors.len(), 4);
        for d in descriptors {
            assert!(d.id.starts_with("hypermedia."));
            assert!(d.descriptor.deterministic);
            assert!(d.descriptor.pure_compute);
            assert_eq!(d.contract.precondition.subject, u64::from(d.descriptor.stable_id));
            assert_eq!(d.contract.postcondition.subject, u64::from(d.descriptor.stable_id));
            assert_eq!(d.contract.invariant.subject, u64::from(d.descriptor.stable_id));
        }
    }
}
