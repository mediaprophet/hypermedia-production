#[cfg(test)]
mod tests {
    use crate::*;

    fn limits() -> AdapterLimits {
        AdapterLimits {
            morphology: MorphologyLimits {
                max_pixels: 16,
                max_work_units: 1_000,
                max_edt_scratch_bytes: 4096,
                max_algorithm_payload_bytes: 4096,
            },
            max_adapter_payload_bytes: 80,
            max_total_payload_bytes: 8192,
            max_total_work_units: 1200,
        }
    }

    #[test]
    fn missing_layer_mask_means_full_coverage_and_returns_replacement_bytes() {
        let full = modify_layer_mask(None, 2, 2, Operation::Expand, 1.0, limits()).unwrap();
        assert_eq!(full, [255; 4]);
        let outline = modify_layer_mask(None, 2, 2, Operation::Border, 1.0, limits()).unwrap();
        assert_eq!(outline, [0; 4]);
    }

    #[test]
    fn source_border_zero_radius_and_mask_length_refusal_are_preserved() {
        let current = [0, 255, 255, 0];
        let outline = modify_layer_mask(Some(&current), 2, 2, Operation::Border, 0.0, limits()).unwrap();
        assert_eq!(outline, [0; 4]);
        assert_eq!(
            preflight_mask(Some(&current[..3]), 2, 2, Operation::Expand, 1.0, limits()),
            Err(AdapterError::MaskDimensions),
        );
    }

    #[test]
    fn aggregate_limits_include_conversion_and_quantization() {
        let current = [0, 255, 255, 0];
        let plan = preflight_mask(Some(&current), 2, 2, Operation::Expand, 1.0, limits()).unwrap();
        assert_eq!(plan.adapter_payload_bytes_upper_bound, 20);
        assert_eq!(plan.total_work_units_upper_bound, plan.morphology.work_units_upper_bound + 8);
        assert!(matches!(
            preflight_mask(Some(&current), 2, 2, Operation::Expand, 1.0, AdapterLimits { max_adapter_payload_bytes: 19, ..limits() }),
            Err(AdapterError::AdapterPayloadBudget),
        ));
    }

    #[test]
    fn test_mask_density_and_feathering() {
        let mask = [0u8, 128, 255, 0];
        // At 100% density, mask is unmodified
        let d100 = scale_mask_density(&mask, 1.0);
        assert_eq!(d100, mask);

        // At 0% density, mask is completely white (255)
        let d0 = scale_mask_density(&mask, 0.0);
        assert_eq!(d0, [255; 4]);

        // At 50% density, black (0) becomes 128
        let d50 = scale_mask_density(&mask, 0.5);
        assert!((d50[0] as i32 - 128).abs() <= 1);
        assert_eq!(d50[2], 255);

        // Gaussian feathering test on a 4x4 image with central dot
        let mut img = [0u8; 16];
        img[5] = 255;
        let feathered = feather_mask(&img, 4, 4, 1.0);
        assert!(feathered[5] > 0);
        assert!(feathered[0] < feathered[5]); // central peak spreads to borders
    }

    #[test]
    fn test_blend_if_ramps() {
        let mut blend = BlendIf::default();
        assert!(blend.is_default());
        assert_eq!(blend.evaluate(0.5, 0.5, &[], &[]), 1.0);

        // This Layer: black range [50, 100], white range [150, 200]
        let range = BlendRange {
            black: [50, 100],
            white: [150, 200],
        };
        blend.set(0, [range, BlendRange::FULL]);
        assert!(!blend.is_default());

        // Value < 50 is hidden (0.0)
        assert_eq!(blend.evaluate(40.0 / 255.0, 0.5, &[], &[]), 0.0);
        // Value between 50 and 100 ramps up
        let mid = blend.evaluate(75.0 / 255.0, 0.5, &[], &[]);
        assert!((mid - 0.5).abs() < 0.05);
        // Value between 100 and 150 is fully visible (1.0)
        assert_eq!(blend.evaluate(125.0 / 255.0, 0.5, &[], &[]), 1.0);
        // Value > 200 is hidden (0.0)
        assert_eq!(blend.evaluate(210.0 / 255.0, 0.5, &[], &[]), 0.0);
    }

    #[test]
    fn test_wet_mixer_brush_strokes() {
        let settings = MixerSettings {
            wet: 0.8,
            load: 0.2, // low load => dries out
            mix: 0.5,
            flow: 1.0,
            sample_all_layers: false,
        };
        let mut state = MixerState::with_reservoir([1.0, 0.0, 0.0, 1.0]); // Pure red
        let canvas = [0.0, 0.0, 1.0, 1.0]; // Pure blue

        // First dab picks up canvas blue
        let col1 = mix_dab(canvas, &settings, &mut state);
        assert!(col1[0] > 0.0, "Contains reservoir red");
        assert!(col1[2] > 0.0, "Contains picked up blue");
        assert!(state.amount < 1.0, "Reservoir drained due to load < 1.0");

        // Several strokes dry out the brush reservoir
        for _ in 0..50 {
            mix_dab(canvas, &settings, &mut state);
        }
        assert_eq!(state.amount, 0.0, "Brush dried out completely");
    }
}
