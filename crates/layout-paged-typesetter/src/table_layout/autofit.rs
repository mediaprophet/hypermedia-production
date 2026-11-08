//! Table auto-fitting algorithm.

use crate::table_layout::measure::{TableData, measure_table_columns};

/// Auto-fits column widths to available table width.
///
/// If available_width is None, allocates max content widths (AutoFit to Content).
/// If available_width is Some(target), balances between min and max widths to fit the target width.
pub fn autofit_widths(table: &TableData, available_width: Option<f32>, font_size: f32) -> Vec<f32> {
    let bounds = measure_table_columns(table, font_size);
    if bounds.is_empty() {
        return Vec::new();
    }

    let Some(target) = available_width else {
        return bounds.into_iter().map(|(_, max)| max).collect();
    };

    let total_min: f32 = bounds.iter().map(|(min, _)| *min).sum();
    let total_max: f32 = bounds.iter().map(|(_, max)| *max).sum();

    if target >= total_max {
        // Expand proportionally beyond max
        let excess = target - total_max;
        let n = bounds.len() as f32;
        bounds.into_iter().map(|(_, max)| max + excess / n).collect()
    } else if target <= total_min {
        // Squeeze down to min widths proportionally
        let ratio = if total_min > 0.0 { target / total_min } else { 1.0 };
        bounds.into_iter().map(|(min, _)| min * ratio).collect()
    } else {
        // Interpolate between min and max
        let fraction = (target - total_min) / (total_max - total_min);
        bounds.into_iter().map(|(min, max)| min + (max - min) * fraction).collect()
    }
}
