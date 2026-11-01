#![forbid(unsafe_code)]

//! Checked conversion adapter between QualiaDB's current U8 layer-mask field and
//! the source-derived f32 selection-morphology operations. This owns no image
//! document, selection identity, persistence, host authority, or history.

use raster_selection_morphology::{
    Limits as MorphologyLimits, Operation, Plan as MorphologyPlan, PreflightError,
    preflight_dimensions, try_border, try_contract, try_expand,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdapterLimits {
    /// Caps passed through to the morphology owner.
    pub morphology: MorphologyLimits,
    /// Additional adapter-owned payload: normalized f32 input plus U8 output (5 bytes/pixel).
    pub max_adapter_payload_bytes: usize,
    /// Aggregate logical payload bound: morphology plan plus adapter-owned payload.
    pub max_total_payload_bytes: usize,
    /// Aggregate work bound: morphology work plus U8-to-f32 and f32-to-U8 visits.
    pub max_total_work_units: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdapterPlan {
    pub pixels: usize,
    pub morphology: MorphologyPlan,
    pub adapter_payload_bytes_upper_bound: usize,
    pub total_payload_bytes_upper_bound: usize,
    pub total_work_units_upper_bound: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdapterError {
    Morphology(PreflightError),
    MaskDimensions,
    AdapterPayloadBudget,
    TotalPayloadBudget,
    TotalWorkBudget,
    Allocation,
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "layer-mask morphology adapter: {self:?}")
    }
}
impl std::error::Error for AdapterError {}

/// Admits a checked edit without converting or allocating the source mask.
/// `None` means full coverage, matching `ImageLayer.mask == None` in the
/// existing compositor. `Some(mask)` must match `width * height` exactly.
pub fn preflight_mask(
    mask: Option<&[u8]>,
    width: usize,
    height: usize,
    operation: Operation,
    radius: f32,
    limits: AdapterLimits,
) -> Result<AdapterPlan, AdapterError> {
    let morphology = preflight_dimensions(operation, width, height, radius, limits.morphology)
        .map_err(AdapterError::Morphology)?;
    if mask.is_some_and(|values| values.len() != morphology.pixels) {
        return Err(AdapterError::MaskDimensions);
    }
    let adapter_payload_bytes_upper_bound = morphology.pixels.checked_mul(5).ok_or(AdapterError::AdapterPayloadBudget)?;
    if adapter_payload_bytes_upper_bound > limits.max_adapter_payload_bytes {
        return Err(AdapterError::AdapterPayloadBudget);
    }
    let total_payload_bytes_upper_bound = morphology.algorithm_payload_bytes_upper_bound
        .checked_add(adapter_payload_bytes_upper_bound)
        .ok_or(AdapterError::TotalPayloadBudget)?;
    if total_payload_bytes_upper_bound > limits.max_total_payload_bytes {
        return Err(AdapterError::TotalPayloadBudget);
    }
    let conversion_work = morphology.pixels.checked_mul(2).ok_or(AdapterError::TotalWorkBudget)?;
    let total_work_units_upper_bound = morphology.work_units_upper_bound
        .checked_add(conversion_work)
        .ok_or(AdapterError::TotalWorkBudget)?;
    if total_work_units_upper_bound > limits.max_total_work_units {
        return Err(AdapterError::TotalWorkBudget);
    }
    Ok(AdapterPlan {
        pixels: morphology.pixels,
        morphology,
        adapter_payload_bytes_upper_bound,
        total_payload_bytes_upper_bound,
        total_work_units_upper_bound,
    })
}

/// Transform the current U8 layer mask and return replacement bytes only after
/// validation and computation succeed. The input is not mutated. A missing mask
/// is interpreted as full coverage, exactly as it is by QualiaDB composition.
pub fn modify_layer_mask(
    mask: Option<&[u8]>,
    width: usize,
    height: usize,
    operation: Operation,
    radius: f32,
    limits: AdapterLimits,
) -> Result<Vec<u8>, AdapterError> {
    let plan = preflight_mask(mask, width, height, operation, radius, limits)?;
    let mut normalized = Vec::new();
    normalized.try_reserve_exact(plan.pixels).map_err(|_| AdapterError::Allocation)?;
    normalized.extend((0..plan.pixels).map(|i| {
        mask.and_then(|values| values.get(i)).map_or(1.0, |value| f32::from(*value) / 255.0)
    }));

    let coverage = match operation {
        Operation::Expand => try_expand(&normalized, width, height, radius, limits.morphology),
        Operation::Contract => try_contract(&normalized, width, height, radius, limits.morphology),
        Operation::Border => try_border(&normalized, width, height, radius, limits.morphology),
    }.map_err(AdapterError::Morphology)?;

    let mut output = Vec::new();
    output.try_reserve_exact(plan.pixels).map_err(|_| AdapterError::Allocation)?;
    output.extend(coverage.iter().map(|value| (value.clamp(0.0, 1.0) * 255.0).round() as u8));
    Ok(output)
}

#[cfg(test)]
mod tests;
