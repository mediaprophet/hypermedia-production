use super::{border_impl as border, contract_impl as contract, expand_impl as expand};

fn square(w: usize, h: usize) -> Vec<f32> {
    (0..w * h).map(|i| if (5..15).contains(&(i % w)) && (5..15).contains(&(i / w)) { 1.0 } else { 0.0 }).collect()
}

// Source-derived PhotoCraft `expand_contract_border` regression, selection.rs:1097-1110.
#[test]
fn expand_contract_border() {
    let m = square(20, 20);
    let e = expand(&m, 20, 20, 2.0);
    assert_eq!(e[10 * 20 + 3], 1.0);
    assert_eq!(e[10 * 20 + 2], 0.0);
    let c = contract(&m, 20, 20, 2.0);
    assert_eq!(c[10 * 20 + 6], 0.0);
    assert_eq!(c[10 * 20 + 7], 1.0);
    let b = border(&m, 20, 20, 4.0);
    assert_eq!(b[10 * 20 + 10], 0.0);
    assert!(b[10 * 20 + 5] > 0.5 && b[10 * 20 + 4] > 0.5);
}

/// PhotoCraft source records this oracle as measured in Photoshop 27.11:
/// a 32×32 rectangle at (16,16) in 64×64, Border 4 px.
#[test]
fn border_matches_photoshop_27_11_source_oracle() {
    let (w, h) = (64, 64);
    let m: Vec<f32> = (0..w * h).map(|i| if (16..48).contains(&(i % w)) && (16..48).contains(&(i / w)) { 1.0 } else { 0.0 }).collect();
    let b = border(&m, w, h, 4.0);
    let v = |x: usize, y: usize| (b[y * w + x] * 255.0).round() as i32;
    let row: Vec<i32> = (8..=24).map(|x| v(x, 32)).collect();
    let diag: Vec<i32> = (16..=24).map(|d| v(d, d)).collect();
    let near = |got: &[i32], want: &[i32]| got.iter().zip(want).all(|(g, w)| (g - w).abs() <= 1);
    let want_row = [0, 0, 0, 0, 63, 127, 191, 255, 255, 191, 127, 63, 0, 0, 0, 0, 0];
    assert!(near(&row, &want_row), "{row:?}");
    assert!(near(&diag, &[255, 191, 127, 63, 0, 0, 0, 0, 0]), "{diag:?}");
}

#[test]
fn fractional_radii_follow_source_coverage_rules() {
    let (w, h) = (13, 11);
    let mask: Vec<f32> = (0..w * h).map(|i| if (3..9).contains(&(i % w)) && (2..8).contains(&(i / w)) { 0.8 } else { 0.2 }).collect();
    let grown = expand(&mask, w, h, 1.25);
    assert!(grown.iter().any(|coverage| *coverage > 0.0 && *coverage < 1.0));
    // Source expansion promotes >=0.5 seed coverage to 1; sufficiently distant
    // sub-threshold coverage remains unchanged. It does not preserve every 0.8.
    assert_eq!(grown[0], mask[0]);
    assert_eq!(grown[2 * w + 3], 1.0);
    assert!((grown[w + 2] - (2.25 - 2.0f32.sqrt())).abs() < 1e-6);
    let shrunk = contract(&mask, w, h, 1.25);
    assert!(shrunk.iter().any(|coverage| *coverage > 0.0 && *coverage < 1.0));
}


#[test]
fn preflight_rejects_shape_nonfinite_coverage_and_limits() {
    use super::{Limits, Operation, PreflightError, preflight};
    let limits = Limits { max_pixels: 16, max_work_units: 256, max_edt_scratch_bytes: 4096, max_algorithm_payload_bytes: 4096 };
    let good = [0.0f32; 4];
    assert!(preflight(Operation::Expand, &good, 2, 2, 1.0, limits).is_ok());
    assert_eq!(preflight(Operation::Expand, &good, 3, 2, 1.0, limits), Err(PreflightError::DimensionMismatch));
    assert_eq!(preflight(Operation::Expand, &good, 2, 2, f32::INFINITY, limits), Err(PreflightError::InvalidRadius));
    assert_eq!(preflight(Operation::Expand, &[0.0, f32::NAN, 0.0, 0.0], 2, 2, 1.0, limits), Err(PreflightError::InvalidCoverage));
    assert_eq!(preflight(Operation::Expand, &[0.0, f32::NAN, 0.0, 0.0], 2, 2, 1.0, Limits { max_work_units: 0, ..limits }), Err(PreflightError::WorkBudget));
    assert_eq!(preflight(Operation::Expand, &good, 2, 2, 1.0, Limits { max_pixels: 3, ..limits }), Err(PreflightError::PixelBudget));
    assert_eq!(preflight(Operation::Border, &good, 2, 2, 1.0, Limits { max_work_units: 67, ..limits }), Err(PreflightError::WorkBudget));
    assert_eq!(preflight(Operation::Expand, &good, 2, 2, 1.0, Limits { max_edt_scratch_bytes: 1, ..limits }), Err(PreflightError::ScratchBudget));
    assert_eq!(preflight(Operation::Expand, &good, 2, 2, 1.0, Limits { max_algorithm_payload_bytes: 1, ..limits }), Err(PreflightError::PayloadBudget));
    assert_eq!(preflight(Operation::Expand, &[], 0, 2, 1.0, limits), Err(PreflightError::EmptyDimensions));
    assert_eq!(preflight(Operation::Expand, &[], usize::MAX, 2, 1.0, limits), Err(PreflightError::DimensionOverflow));
    assert_eq!(preflight(Operation::Expand, &[], usize::MAX / 32 + 1, 1, 1.0, limits), Err(PreflightError::DimensionOverflow));
}

#[test]
fn zero_radius_keeps_source_operation_behavior() {
    use super::{Limits, try_border};
    let limits = Limits { max_pixels: 16, max_work_units: 256, max_edt_scratch_bytes: 4096, max_algorithm_payload_bytes: 4096 };
    let mask = [0.0, 1.0, 1.0, 0.0];
    assert_eq!(try_border(&mask, 2, 2, 0.0, limits).unwrap(), [0.0; 4]);
}

