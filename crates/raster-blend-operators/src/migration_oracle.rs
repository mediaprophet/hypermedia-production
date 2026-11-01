//! Test-only migration oracle copied from PhotoCraft's selected helper bodies.
//!
//! Origin: `C:/github/poet-photocraft/crates/compose/src/psblend.rs`, SHA-256
//! `683B2EAD37899A7BE7EB385492F7C45B4534C9B6FC3BBDBD449D0FDE8AC36976`,
//! lines 17-72 (inclusive). The five helper bodies and `EDGE` below are
//! preserved as source text (line endings normalized); this module is test-only and has no
//! runtime dependency on PhotoCraft. Copyright and license terms are in the
//! crate's copied `NOTICE`, `LICENSE-MIT`, and `LICENSE-APACHE` files.

fn vivid_light_ps(cb: f32, cs: f32) -> f32 {
    if cs <= 0.5 {
        if cs <= 0.0 { 0.0 } else { 1.0 - ((1.0 - cb) / (2.0 * cs)).min(1.0) }
    } else if cs >= 1.0 {
        1.0
    } else {
        (cb / (2.0 * (1.0 - cs))).min(1.0)
    }
}

/// Backdrop values within `EDGE` of 0 / 1 count as exact in the modes whose result jumps there
/// (Color Burn / Dodge, Hard Mix): a composited value that should be 1 can come out a rounding
/// step below it (and differently on the GPU), flipping the corner case by up to 255 levels.
pub const EDGE: f32 = 1e-4;

fn color_burn(cb: f32, cs: f32) -> f32 {
    if cb >= 1.0 - EDGE {
        1.0
    } else if cs <= 0.0 {
        0.0
    } else {
        1.0 - ((1.0 - cb) / cs).min(1.0)
    }
}

fn color_dodge(cb: f32, cs: f32) -> f32 {
    if cb <= EDGE {
        0.0
    } else if cs >= 1.0 {
        1.0
    } else {
        (cb / (1.0 - cs)).min(1.0)
    }
}

fn vivid_light_generic(cb: f32, cs: f32) -> f32 {
    if cs <= 0.5 {
        if cb >= 1.0 - EDGE {
            1.0
        } else if cs <= 0.0 {
            0.0
        } else {
            1.0 - ((1.0 - cb) / (2.0 * cs)).min(1.0)
        }
    } else if cb <= EDGE {
        0.0
    } else if cs >= 1.0 {
        1.0
    } else {
        (cb / (2.0 * (1.0 - cs))).min(1.0)
    }
}

fn hard_mix_ps(cb: f32, cs: f32) -> f32 {
    if vivid_light_generic(cb, cs) >= 0.5 - 1e-6 { 1.0 } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::{color_burn, color_dodge, hard_mix_ps, vivid_light_ps, EDGE};
    use crate::{blend_channel, BlendOperator};

    #[test]
    fn selected_operators_are_bit_equal_to_source_on_normalized_grid() {
        let operators = [
            (BlendOperator::ColorBurn, color_burn as fn(f32, f32) -> f32),
            (BlendOperator::ColorDodge, color_dodge as fn(f32, f32) -> f32),
            (BlendOperator::VividLight, vivid_light_ps as fn(f32, f32) -> f32),
            (BlendOperator::HardMix, hard_mix_ps as fn(f32, f32) -> f32),
        ];

        for backdrop_index in 0..=256 {
            let backdrop = backdrop_index as f32 / 256.0;
            for source_index in 0..=256 {
                let source = source_index as f32 / 256.0;
                for (operator, reference) in operators {
                    assert_bit_equal(operator, backdrop, source, reference(backdrop, source));
                }
            }
        }
    }

    #[test]
    fn selected_operators_are_bit_equal_around_edges_midpoint_and_extremes() {
        let values = [
            0.0,
            EDGE / 2.0,
            EDGE,
            EDGE + f32::EPSILON,
            2.0 * EDGE,
            0.5 - f32::EPSILON,
            0.5,
            0.5 + f32::EPSILON,
            1.0 - 2.0 * EDGE,
            1.0 - EDGE,
            1.0 - EDGE / 2.0,
            1.0,
        ];
        let operators = [
            (BlendOperator::ColorBurn, color_burn as fn(f32, f32) -> f32),
            (BlendOperator::ColorDodge, color_dodge as fn(f32, f32) -> f32),
            (BlendOperator::VividLight, vivid_light_ps as fn(f32, f32) -> f32),
            (BlendOperator::HardMix, hard_mix_ps as fn(f32, f32) -> f32),
        ];

        assert_eq!(crate::EDGE.to_bits(), EDGE.to_bits());
        for backdrop in values {
            for source in values {
                for (operator, reference) in operators {
                    assert_bit_equal(operator, backdrop, source, reference(backdrop, source));
                }
            }
        }
    }

    fn assert_bit_equal(operator: BlendOperator, backdrop: f32, source: f32, expected: f32) {
        let actual = blend_channel(operator, backdrop, source);
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "{operator:?}: backdrop={backdrop:?}, source={source:?}, actual={actual:?}, expected={expected:?}"
        );
    }
}
