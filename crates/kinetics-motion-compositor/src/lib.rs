//! `kinetics-motion-compositor` — Categorical motion graphics, keyframe velocity graphs, tracking, and typography.
//!
//! Provides:
//! - Keyframe velocity graphs, speed/influence temporal cubic Béziers, hold/linear/roving interpolation, and 3D arc-length spatial paths (`keyframe`)
//! - 2D normalized cross-correlation luminance point tracking, 4-point perspective corner-pin homography, and pyramidal Lucas-Kanade optical flow (`track`)
//! - Lottie JSON vector animation schema, importer, exporter, and frame evaluator (`lottie`)
//! - Kinetic text animators with range selectors, wiggly selectors, and per-character 3D position/rotation/scale/opacity offsets (`typography`)
//!
//! Grounded in international human rights instruments protecting creative freedom, intellectual property,
//! and moral rights of authors (UDHR Articles 12, 19, 27; ICCPR Articles 17, 19; ICESCR Article 15)
//! and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod keyframe;
pub mod lottie;
pub mod time;
pub mod track;
pub mod typography;

pub use keyframe::{
    Ease, Interp, Keyframe, KeyframeTrack, Value,
    bezier::{bez, ease_progress, solve_u},
    spatial::{ArcLengthTable, eval_spatial_bezier},
};
pub use lottie::{
    EvaluatedTransform, LottieAnimation, LottieLayer, LottiePropMulti, LottiePropScalar,
    LottieShape, LottieTransform, eval_layer_transform, parse_lottie, write_lottie,
};
pub use time::{FrameRate, TICKS_PER_SECOND, Tick};
pub use track::{
    FeaturePoint, GrayImage, Homography, ImagePyramid, detect_shi_tomasi_corners,
    solve_corner_pin_homography, track_klt_point, track_point_ncc,
};
pub use typography::{
    BasedOn, CharTransform, Mode, RangeSelector, Shape, TextAnimator, TextProperties,
    WigglySelector, evaluate_text_animators,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_bezier_solve_and_ease_progress() {
        // At t = 0.0, progress is 0.0
        assert!((ease_progress(0.0, 0.33, 0.0, 0.33, 0.0) - 0.0).abs() < 1e-6);
        // At t = 1.0, progress is 1.0
        assert!((ease_progress(0.0, 0.33, 0.0, 0.33, 1.0) - 1.0).abs() < 1e-6);
        // Symmetric ease at midpoint should be 0.5
        let mid = ease_progress(0.0, 0.5, 0.0, 0.5, 0.5);
        assert!((mid - 0.5).abs() < 1e-4);
    }

    #[test]
    fn test_spatial_arc_length_table() {
        let p0 = [0.0, 0.0, 0.0];
        let p1 = [0.0, 50.0, 0.0];
        let p2 = [100.0, 50.0, 0.0];
        let p3 = [100.0, 0.0, 0.0];

        let table = ArcLengthTable::build(p0, p1, p2, p3, 64);
        assert!(table.total_length > 100.0);

        let u_mid = table.u_at_progress(0.5);
        assert!((u_mid - 0.5).abs() < 0.05);
        let pt_mid = eval_spatial_bezier(p0, p1, p2, p3, u_mid);
        assert!((pt_mid[0] - 50.0).abs() < 1.0); // X should be centered
    }

    #[test]
    fn test_keyframe_track_evaluation() {
        let mut track = KeyframeTrack::default();
        let k0 = Keyframe::new(Tick::ZERO, Value::Float(10.0));
        let k1 = Keyframe::new(Tick::from_seconds(2.0), Value::Float(30.0)).eased();
        track.keyframes.push(k0);
        track.keyframes.push(k1);

        assert_eq!(track.evaluate(Tick::ZERO).unwrap().as_f64(), 10.0);
        assert_eq!(track.evaluate(Tick::from_seconds(2.0)).unwrap().as_f64(), 30.0);

        // At midpoint 1.0s, value should be 20.0
        let mid_val = track.evaluate(Tick::from_seconds(1.0)).unwrap().as_f64();
        assert!((mid_val - 20.0).abs() < 0.5);
    }

    #[test]
    fn test_ncc_point_tracker() {
        // Create 32x32 target image with a bright spot at (15, 15)
        let mut target = GrayImage::new(32, 32);
        for y in 14..=16 {
            for x in 14..=16 {
                target.data[y * 32 + x] = 1.0;
            }
        }

        // 3x3 template with bright spot
        let mut template = GrayImage::new(3, 3);
        template.data.fill(1.0);

        let match_res = track_point_ncc(&target, &template, 14, 14, 5).unwrap();
        let (bx, by, conf) = match_res;
        assert_eq!(bx.round() as usize, 14);
        assert_eq!(by.round() as usize, 14);
        assert!(conf > 0.9);
    }

    #[test]
    fn test_corner_pin_homography() {
        let src = [[0.0, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]];
        // Translate +20 X, +10 Y
        let dst = [[20.0, 10.0], [120.0, 10.0], [120.0, 110.0], [20.0, 110.0]];

        let h = solve_corner_pin_homography(&src, &dst).unwrap();
        let mapped = h.transform_point([50.0, 50.0]);
        assert!((mapped[0] - 70.0).abs() < 1e-3);
        assert!((mapped[1] - 60.0).abs() < 1e-3);
    }

    #[test]
    fn test_lottie_parsing_and_evaluation() {
        let lottie_json = r#"{
            "v": "5.5.7",
            "fr": 30,
            "ip": 0,
            "op": 60,
            "w": 500,
            "h": 500,
            "layers": [
                {
                    "ty": 4,
                    "nm": "Shape Layer",
                    "ip": 0,
                    "op": 60,
                    "st": 0,
                    "ks": {
                        "p": { "a": 0, "k": [250, 250] },
                        "s": { "a": 0, "k": [100, 100] },
                        "r": { "a": 0, "k": 45 },
                        "o": { "a": 0, "k": 100 }
                    }
                }
            ]
        }"#;

        let anim = parse_lottie(lottie_json).unwrap();
        assert_eq!(anim.layers.len(), 1);
        let layer = &anim.layers[0];
        let xform = eval_layer_transform(layer, 15.0);
        assert_eq!(xform.position, [250.0, 250.0]);
        assert_eq!(xform.scale, [100.0, 100.0]);
        assert_eq!(xform.rotation_deg, 45.0);
        assert_eq!(xform.opacity, 100.0);
    }

    #[test]
    fn test_kinetic_text_animators() {
        let text = "HELLO";
        let mut anim = TextAnimator::default();
        anim.name = "Fly In".into();
        anim.range_selector = Some(RangeSelector {
            start: 0.0,
            end: 1.0,
            offset: 0.0,
            amount: 1.0,
            shape: Shape::RampUp,
            ..Default::default()
        });
        anim.properties = TextProperties {
            position: [0.0, 50.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [0.0, 0.0, 0.0],
            opacity_delta: -1.0,
        };

        let results = evaluate_text_animators(text, &[anim], 0.0);
        assert_eq!(results.len(), 5);
        // RampUp: index 0 gets weight 0, index 4 gets weight ~1.0
        assert_eq!(results[0].position[1], 0.0);
        assert!(results[4].position[1] > 30.0);
    }
}
