# `crates/radiance-camera-raw`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_# Radiance Camera RAW & Computational Sensor Optics

Provides pure-Rust, memory-bounded, zero-unsafe algorithms for:
- Camera sensor CFA demosaicing (Bayer RGGB/BGGR/GRBG/GBRG with Bilinear, AHD, and RCD, plus Fujifilm 6x6 X-Trans)
- Sensor black/white level calibration and dual-illuminant color matrix white balance
- Non-destructive parametric RAW development pipeline (EV, WB, tonal recovery, tone curves, 8-band HSL, split toning, detail, vignette, radial distortion)
- Multi-exposure bracket HDR radiance map fusion with motion deghosting
- Multi-image panorama stitching with multi-band Burt-Adelson Laplacian pyramid blending
- Metadata structures (EXIF, XMP, IPTC), GPX tracklog geotag interpolation with meridian wrapping
- Reversible append-only catalog journal with undo/redo and smart album boolean query evaluation

Execution and provenance guarantees are grounded in international human rights instruments
(UDHR Articles 12, 18, 19, 27; ICCPR Articles 17, 19; ICESCR Article 15) and Informatics Fiduciary duties._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl CfaPattern::color_at` | 3 | `crates/radiance-camera-raw/src/demosaic.rs:21` | - | Returns 0 for Red, 1 for Green, 2 for Blue at sensor coordinate (x, y). |
| `normalize_raw` | 3 | `crates/radiance-camera-raw/src/demosaic.rs:80` | 3d, numerics | Normalizes 16-bit raw sensor data to scene-linear f32 in [0.0, 1.0] |
| `interpolate_dual_illuminant_matrix` | 5 | `crates/radiance-camera-raw/src/demosaic.rs:90` | numerics | Dual-illuminant color matrix white balance. |
| `demosaic_bilinear` | 4 | `crates/radiance-camera-raw/src/demosaic.rs:117` | - | Bilinear demosaicing for Bayer sensor data. |
| `demosaic_ahd` | 4 | `crates/radiance-camera-raw/src/demosaic.rs:192` | - | Adaptive Homogeneity-Directed (AHD) demosaicing for Bayer CFA. |
| `demosaic_rcd` | 4 | `crates/radiance-camera-raw/src/demosaic.rs:288` | - | Ratio-based Color Difference (RCD) demosaicing for Bayer CFA. |
| `demosaic_xtrans` | 3 | `crates/radiance-camera-raw/src/demosaic.rs:293` | graph | Edge-weighted color-difference demosaicing for Fujifilm 6×6 X-Trans CFA patterns. |
| `demosaic_raw` | 7 | `crates/radiance-camera-raw/src/demosaic.rs:369` | numerics | Demosaics a raw 16-bit sensor buffer into linear RGB f32. |
| `impl Default for ParametricToneCurve::default` | 0 | `crates/radiance-camera-raw/src/develop.rs:43` | - |  |
| `impl DevelopSettings::cache_digest` | 1 | `crates/radiance-camera-raw/src/develop.rs:115` | - | Computes deterministic 64-bit FNV-1a hash over all parameters. |
| `develop_image` | 4 | `crates/radiance-camera-raw/src/develop.rs:188` | numerics | Applies non-destructive develop pipeline to linear RGB image data. |
| `hat_weight` | 1 | `crates/radiance-camera-raw/src/fusion.rs:25` | - | Hat weighting function: favors midtones, drops to 0 at extremes (over/underexposure). |
| `merge_hdr` | 4 | `crates/radiance-camera-raw/src/fusion.rs:35` | - | Multi-exposure bracket HDR radiance map fusion with motion deghosting. |
| `stitch_panorama` | 5 | `crates/radiance-camera-raw/src/fusion.rs:109` | 3d | Multi-band Burt-Adelson pyramid blending for multi-image panorama stitching. |
| `test_bayer_demosaic_rggb` | 0 | `crates/radiance-camera-raw/src/lib.rs:44` | - |  |
| `test_xtrans_demosaic` | 0 | `crates/radiance-camera-raw/src/lib.rs:74` | - |  |
| `test_develop_pipeline_and_hash_determinism` | 0 | `crates/radiance-camera-raw/src/lib.rs:84` | - |  |
| `test_hdr_fusion_deghosting` | 0 | `crates/radiance-camera-raw/src/lib.rs:109` | - |  |
| `test_gpx_tracklog_interpolation_and_meridian_wrap` | 0 | `crates/radiance-camera-raw/src/lib.rs:141` | - |  |
| `test_catalog_journal_undo_redo` | 0 | `crates/radiance-camera-raw/src/lib.rs:166` | - |  |
| `test_smart_rule_evaluation` | 0 | `crates/radiance-camera-raw/src/lib.rs:196` | - |  |
| `test_ontology_contracts` | 0 | `crates/radiance-camera-raw/src/lib.rs:221` | semantics |  |
| `impl GpxTracklog::new` | 0 | `crates/radiance-camera-raw/src/metadata.rs:57` | - |  |
| `impl GpxTracklog::add_point` | 2 | `crates/radiance-camera-raw/src/metadata.rs:61` | - |  |
| `impl GpxTracklog::interpolate_position` | 3 | `crates/radiance-camera-raw/src/metadata.rs:67` | - | Interpolates GPS location at `time` (seconds since epoch) with a maximum gap limit. |
| `impl JournalAction::inverse` | 1 | `crates/radiance-camera-raw/src/metadata.rs:146` | - |  |
| `impl CatalogJournal::new` | 0 | `crates/radiance-camera-raw/src/metadata.rs:177` | - |  |
| `impl CatalogJournal::record_action` | 2 | `crates/radiance-camera-raw/src/metadata.rs:181` | - |  |
| `impl CatalogJournal::undo` | 1 | `crates/radiance-camera-raw/src/metadata.rs:186` | - |  |
| `impl CatalogJournal::redo` | 1 | `crates/radiance-camera-raw/src/metadata.rs:193` | - |  |
| `impl SmartRule::matches` | 2 | `crates/radiance-camera-raw/src/metadata.rs:219` | - |  |
| `impl NQuin48::new` | 5 | `crates/radiance-camera-raw/src/ontology.rs:15` | - |  |
| `impl NQuin48::precondition` | 3 | `crates/radiance-camera-raw/src/ontology.rs:19` | - |  |
| `impl NQuin48::postcondition` | 3 | `crates/radiance-camera-raw/src/ontology.rs:23` | - |  |
| `impl NQuin48::invariant` | 3 | `crates/radiance-camera-raw/src/ontology.rs:27` | - |  |
