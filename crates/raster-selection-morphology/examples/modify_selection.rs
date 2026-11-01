use raster_selection_morphology::{Limits, try_border, try_contract, try_expand};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = (9, 7);
    let mask: Vec<f32> = (0..width * height)
        .map(|i| if (2..7).contains(&(i % width)) && (1..6).contains(&(i / width)) { 1.0 } else { 0.0 })
        .collect();
    let limits = Limits {
        max_pixels: 1_000,
        max_work_units: 16_000,
        max_edt_scratch_bytes: 10_000,
        max_algorithm_payload_bytes: 100_000,
    };

    // The host validates and retains these values in its native editable image/selection state.
    let grown = try_expand(&mask, width, height, 1.5, limits)?;
    let reduced = try_contract(&mask, width, height, 1.0, limits)?;
    let outline = try_border(&mask, width, height, 2.0, limits)?;
    assert_eq!(grown.len(), mask.len());
    assert_eq!(reduced.len(), mask.len());
    assert_eq!(outline.len(), mask.len());
    Ok(())
}

