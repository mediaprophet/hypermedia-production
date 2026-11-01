#![forbid(unsafe_code)]

//! Euclidean morphology for caller-owned, row-major selection coverage masks.
//!
//! Use the checked `try_*` operations to validate finite coverage, dimensions, radius,
//! and caller-selected work and logical-payload budgets before allocating. The returned
//! values remain the responsibility of the caller's authoritative editable image state.
//! This crate owns no document, selection identity, history, canvas-boundary policy,
//! persistent mask format, or compositor.

mod distance;
use distance::edt;
const EDT_BAND_ROWS: usize = 32;

/// Which morphology is admitted by [`preflight`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation { Expand, Contract, Border }

/// Caller-selected caps. Byte and work caps use logical element and loop-step bounds,
/// documented below; allocator metadata, heap fragmentation, and a shared Rayon pool are excluded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_pixels: usize,
    pub max_work_units: usize,
    pub max_edt_scratch_bytes: usize,
    pub max_algorithm_payload_bytes: usize,
}

/// Upper bounds calculated before any algorithm allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Plan {
    pub pixels: usize,
    pub work_units_upper_bound: usize,
    /// Peak simultaneously-live element payload owned by the operation, including returned
    /// output vectors and EDT work arrays, but excluding caller input and allocator overhead.
    pub algorithm_payload_bytes_upper_bound: usize,
    /// Peak EDT band scratch payload across the configured execution mode.
    pub edt_scratch_bytes_upper_bound: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreflightError {
    DimensionOverflow,
    DimensionMismatch,
    EmptyDimensions,
    PixelBudget,
    WorkBudget,
    PayloadBudget,
    ScratchBudget,
    InvalidRadius,
    InvalidCoverage,
}

impl std::fmt::Display for PreflightError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "selection morphology preflight: {self:?}") }
}
impl std::error::Error for PreflightError {}

/// Validates shape, radius, coverage, and caller-selected limits without allocating algorithm
/// buffers. Work units conservatively count coverage validation and source loop iterations:
/// at most 27 per pixel for expand, 29 for contract, and 52 for border. Each EDT is budgeted at
/// 24 visits per pixel to cover its seed `any`/`all` scans, column sweeps, output initialization,
/// band copies, row candidate filtering/insertion and removals, envelope advancement, and output
/// writes, with headroom for loop-condition and index checks. Payload bounds count Vec element storage, including
/// masks, EDT arrays, results, and concurrent EDT band scratch. They do not claim to bound total
/// process heap or allocator/Rayon runtime overhead.
/// Validates shape, radius, and caller-selected limits before any buffer conversion/allocation.
/// This is useful to adapters whose source representation is not the morphology crate's f32 mask.
pub fn preflight_dimensions(operation: Operation, width: usize, height: usize, radius: f32, limits: Limits) -> Result<Plan, PreflightError> {
    let pixels = width.checked_mul(height).ok_or(PreflightError::DimensionOverflow)?;
    if width == 0 || height == 0 { return Err(PreflightError::EmptyDimensions); }
    // The copied EDT allocates a fixed `width * BAND_ROWS` band buffer (BAND_ROWS = 32),
    // even when the image has fewer than 32 rows. Reject the same overflow before it can
    // make the checked wrapper return an empty result as if the operation succeeded.
    if u32::try_from(width).is_err()
        || u32::try_from(height).is_err()
        || pixels > isize::MAX as usize / std::mem::size_of::<u32>()
        || width.checked_mul(EDT_BAND_ROWS).is_none()
    {
        return Err(PreflightError::DimensionOverflow);
    }
    if pixels > limits.max_pixels { return Err(PreflightError::PixelBudget); }
    if !radius.is_finite() || radius < 0.0 { return Err(PreflightError::InvalidRadius); }
    let units_per_pixel = match operation { Operation::Expand => 27usize, Operation::Contract => 29, Operation::Border => 52 };
    let work_units_upper_bound = pixels.checked_mul(units_per_pixel).ok_or(PreflightError::WorkBudget)?;
    if work_units_upper_bound > limits.max_work_units { return Err(PreflightError::WorkBudget); }

    let scratch = edt_scratch_bound(width, height, pixels).ok_or(PreflightError::PayloadBudget)?;
    if scratch > limits.max_edt_scratch_bytes { return Err(PreflightError::ScratchBudget); }
    let payload_per_pixel = match operation { Operation::Expand => 9usize, Operation::Contract => 13, Operation::Border => 14 };
    let algorithm_payload_bytes_upper_bound = pixels.checked_mul(payload_per_pixel)
        .and_then(|base| base.checked_add(scratch))
        .ok_or(PreflightError::PayloadBudget)?;
    if algorithm_payload_bytes_upper_bound > limits.max_algorithm_payload_bytes { return Err(PreflightError::PayloadBudget); }
    Ok(Plan { pixels, work_units_upper_bound, algorithm_payload_bytes_upper_bound, edt_scratch_bytes_upper_bound: scratch })
}

/// Validates shape, radius, coverage, and caller-selected limits without allocating algorithm
/// buffers. Work units conservatively count coverage validation and source loop iterations:
/// at most 27 per pixel for expand, 29 for contract, and 52 for border. Each EDT is budgeted at
/// 24 visits per pixel to cover its seed `any`/`all` scans, column sweeps, output initialization,
/// band copies, row candidate filtering/insertion and removals, envelope advancement, and output
/// writes, with headroom for loop-condition and index checks. Payload bounds count Vec element storage, including
/// masks, EDT arrays, results, and concurrent EDT band scratch. They do not claim to bound total
/// process heap or allocator/Rayon runtime overhead.
pub fn preflight(operation: Operation, mask: &[f32], width: usize, height: usize, radius: f32, limits: Limits) -> Result<Plan, PreflightError> {
    let plan = preflight_dimensions(operation, width, height, radius, limits)?;
    if plan.pixels != mask.len() { return Err(PreflightError::DimensionMismatch); }
    // Full input validation is O(N), so do the scalar work/payload admissions first. Its one
    // visit per pixel is included in each operation's conservative bound above.
    if mask.iter().any(|coverage| !coverage.is_finite() || !(0.0..=1.0).contains(coverage)) { return Err(PreflightError::InvalidCoverage); }
    Ok(plan)
}

fn edt_scratch_bound(width: usize, height: usize, pixels: usize) -> Option<usize> {
    if pixels == 0 || width == 0 || height == 0 { return Some(0); }
    let band_rows = height.min(32);
    let band_pixels = width.checked_mul(band_rows)?;
    let band_bytes = band_pixels.checked_mul(std::mem::size_of::<u32>())?
        .checked_add(width.checked_mul(std::mem::size_of::<usize>() + std::mem::size_of::<f64>())?)?;
    #[cfg(all(feature = "parallel", not(target_arch = "wasm32")))]
    let concurrent_bands = if pixels >= 65_536 {
        let count = height.checked_add(31)?.checked_div(32)?;
        count.min(rayon::current_num_threads()).max(1)
    } else { 1 };
    #[cfg(not(all(feature = "parallel", not(target_arch = "wasm32"))))]
    let concurrent_bands = 1usize;
    band_bytes.checked_mul(concurrent_bands)
}

/// Checked Euclidean expansion. Caller-selected limits are enforced before mask or EDT buffers
/// are allocated. Seed threshold, fractional coverage, and pixel-center radius behavior follow
/// PhotoCraft's source operation.
pub fn try_expand(mask: &[f32], width: usize, height: usize, radius: f32, limits: Limits) -> Result<Vec<f32>, PreflightError> {
    preflight(Operation::Expand, mask, width, height, radius, limits)?;
    Ok(expand_impl(mask, width, height, radius))
}

/// Checked Euclidean contraction, preserving PhotoCraft's inversion/expansion/inversion behavior.
pub fn try_contract(mask: &[f32], width: usize, height: usize, radius: f32, limits: Limits) -> Result<Vec<f32>, PreflightError> {
    preflight(Operation::Contract, mask, width, height, radius, limits)?;
    Ok(contract_impl(mask, width, height, radius))
}

/// Checked soft band on both sides of the mask edge, following PhotoCraft's measured source oracle.
pub fn try_border(mask: &[f32], width: usize, height: usize, radius: f32, limits: Limits) -> Result<Vec<f32>, PreflightError> {
    preflight(Operation::Border, mask, width, height, radius, limits)?;
    Ok(border_impl(mask, width, height, radius))
}

fn expand_impl(mask: &[f32], width: usize, height: usize, radius: f32) -> Vec<f32> {
    let inside: Vec<bool> = mask.iter().map(|v| *v >= 0.5).collect();
    let distances = edt(&inside, width, height);
    // A pixel whose centre is `d` away from the nearest selected centre is
    // covered when d ≤ radius (fractional radii give partial coverage).
    mask.iter().zip(distances).map(|(v, d)| v.max((radius + 1.0 - d).clamp(0.0, 1.0))).collect()
}

fn contract_impl(mask: &[f32], width: usize, height: usize, radius: f32) -> Vec<f32> {
    let inverse: Vec<f32> = mask.iter().map(|v| 1.0 - v).collect();
    expand_impl(&inverse, width, height, radius).into_iter().map(|v| 1.0 - v).collect()
}

fn border_impl(mask: &[f32], width: usize, height: usize, radius: f32) -> Vec<f32> {
    let inside: Vec<bool> = mask.iter().map(|v| *v >= 0.5).collect();
    let outside: Vec<bool> = inside.iter().map(|v| !v).collect();
    let (to_inside, to_outside) = (edt(&inside, width, height), edt(&outside, width, height));
    if radius <= 0.0 || to_inside.len() != mask.len() || to_outside.len() != mask.len() {
        return vec![0.0; mask.len()];
    }
    inside.iter().zip(to_inside.iter().zip(&to_outside)).map(|(&selected, (&di, &dout))| (1.0 - ((if selected { dout } else { di }) - 1.0) / radius).clamp(0.0, 1.0)).collect()
}

// Compatibility path for the byte-preserved EDT source. Default builds are serial; Rayon is
// available only when the consumer explicitly enables the `parallel` feature.
mod photo_util {
    pub(crate) fn par_rows<T: Send + Sync>(buf: &mut [T], w: usize, ch: usize, f: impl Fn(usize, &mut [T]) + Sync + Send) {
        let stride = (w * ch).max(1);
        #[cfg(all(feature = "parallel", not(target_arch = "wasm32")))]
        {
            use rayon::prelude::*;
            buf.par_chunks_mut(stride).enumerate().for_each(|(y, row)| f(y, row));
        }
        #[cfg(not(all(feature = "parallel", not(target_arch = "wasm32"))))]
        buf.chunks_mut(stride).enumerate().for_each(|(y, row)| f(y, row));
    }
}

#[cfg(test)]
mod selection {
    // The copied distance tests use PhotoCraft's historical module path.
    pub(super) use crate::{contract_impl as contract, expand_impl as expand};
}

#[cfg(test)]
mod tests;
