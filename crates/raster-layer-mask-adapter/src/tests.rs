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
}
