use raster_layer_mask_adapter::{AdapterLimits, modify_layer_mask};
use raster_selection_morphology::{Limits, Operation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let limits = AdapterLimits {
        morphology: Limits {
            max_pixels: 4096,
            max_work_units: 52 * 4096,
            max_edt_scratch_bytes: 1_000_000,
            max_algorithm_payload_bytes: 1_000_000,
        },
        max_adapter_payload_bytes: 4096 * 5,
        max_total_payload_bytes: 2_000_000,
        max_total_work_units: 60 * 4096,
    };
    // This value belongs to the existing ImageLayer.mask field; None means full coverage.
    let current_mask: Option<&[u8]> = None;
    let replacement = modify_layer_mask(current_mask, 32, 32, Operation::Border, 2.0, limits)?;
    assert_eq!(replacement.len(), 32 * 32);
    // The existing image owner replaces its mask only after this returns successfully.
    Ok(())
}
