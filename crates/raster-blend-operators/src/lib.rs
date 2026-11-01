//! Pure channel blend operators with the edge behavior used by PhotoCraft.
//!
//! Inputs are expected in the normalized channel range `[0, 1]`. This crate
//! does not own pixel buffers, alpha composition, color management, document
//! state, or application effects.

/// The four operators with source-specific edge behavior in PhotoCraft.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlendOperator {
    ColorBurn,
    ColorDodge,
    VividLight,
    HardMix,
}

/// Near-zero/near-one tolerance used by burn, dodge, and generic vivid-light
/// edge handling.
pub const EDGE: f32 = 1e-4;

/// Evaluates one operator for one backdrop/source channel pair.
pub fn blend_channel(operator: BlendOperator, backdrop: f32, source: f32) -> f32 {
    match operator {
        BlendOperator::ColorBurn => color_burn(backdrop, source),
        BlendOperator::ColorDodge => color_dodge(backdrop, source),
        BlendOperator::VividLight => vivid_light(backdrop, source),
        BlendOperator::HardMix => {
            if vivid_light_generic(backdrop, source) >= 0.5 - 1e-6 { 1.0 } else { 0.0 }
        }
    }
}

/// Evaluates an operator independently for the three RGB channels.
pub fn blend_rgb(operator: BlendOperator, backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|index| blend_channel(operator, backdrop[index], source[index]))
}

#[cfg(test)]
mod migration_oracle;

fn vivid_light(backdrop: f32, source: f32) -> f32 {
    if source <= 0.5 {
        if source <= 0.0 { 0.0 } else { 1.0 - ((1.0 - backdrop) / (2.0 * source)).min(1.0) }
    } else if source >= 1.0 {
        1.0
    } else {
        (backdrop / (2.0 * (1.0 - source))).min(1.0)
    }
}

fn color_burn(backdrop: f32, source: f32) -> f32 {
    if backdrop >= 1.0 - EDGE {
        1.0
    } else if source <= 0.0 {
        0.0
    } else {
        1.0 - ((1.0 - backdrop) / source).min(1.0)
    }
}

fn color_dodge(backdrop: f32, source: f32) -> f32 {
    if backdrop <= EDGE {
        0.0
    } else if source >= 1.0 {
        1.0
    } else {
        (backdrop / (1.0 - source)).min(1.0)
    }
}

fn vivid_light_generic(backdrop: f32, source: f32) -> f32 {
    if source <= 0.5 {
        if backdrop >= 1.0 - EDGE {
            1.0
        } else if source <= 0.0 {
            0.0
        } else {
            1.0 - ((1.0 - backdrop) / (2.0 * source)).min(1.0)
        }
    } else if backdrop <= EDGE {
        0.0
    } else if source >= 1.0 {
        1.0
    } else {
        (backdrop / (2.0 * (1.0 - source))).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{blend_channel, BlendOperator};

    #[test]
    fn vivid_light_source_extremes_win() {
        assert_eq!(blend_channel(BlendOperator::VividLight, 0.0, 1.0), 1.0);
        assert_eq!(blend_channel(BlendOperator::VividLight, 1.0, 0.0), 0.0);
        assert!((blend_channel(BlendOperator::VividLight, 0.5, 0.75) - 1.0).abs() < 1e-6);
        assert!((blend_channel(BlendOperator::VividLight, 0.5, 0.25) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn hard_mix_uses_generic_vivid_light_edge_rules() {
        assert_eq!(blend_channel(BlendOperator::HardMix, 1.0, 0.0), 1.0);
        assert_eq!(blend_channel(BlendOperator::HardMix, 0.0, 1.0), 0.0);
        assert_eq!(blend_channel(BlendOperator::HardMix, 0.6, 0.6), 1.0);
        assert_eq!(blend_channel(BlendOperator::HardMix, 0.3, 0.3), 0.0);
    }

    #[test]
    fn burn_and_dodge_treat_near_extremes_as_exact() {
        assert_eq!(blend_channel(BlendOperator::ColorBurn, 1.0 - 1e-6, 0.0), 1.0);
        assert_eq!(blend_channel(BlendOperator::ColorDodge, 1e-6, 1.0), 0.0);
        assert!((blend_channel(BlendOperator::ColorBurn, 0.5, 0.5)).abs() < 1e-6);
        assert!((blend_channel(BlendOperator::ColorDodge, 0.25, 0.5) - 0.5).abs() < 1e-6);
    }
}
