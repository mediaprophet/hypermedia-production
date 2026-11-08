//! Kinetic text animator and per-character 3D transformation evaluation.

use serde::{Deserialize, Serialize};

use crate::typography::selectors::{Mode, RangeSelector};
use crate::typography::wiggly::WigglySelector;

/// 3D transformation and opacity applied to a single character.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharTransform {
    pub position: [f64; 3],
    pub rotation: [f64; 3], // Degrees around X, Y, Z
    pub scale: [f64; 3],    // Scale factors (1.0 = 100%)
    pub opacity: f64,       // 0.0 to 1.0
}

impl Default for CharTransform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            opacity: 1.0,
        }
    }
}

/// Animated text property targets.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TextProperties {
    pub position: [f64; 3],
    pub rotation: [f64; 3],
    pub scale: [f64; 3],    // Target scale deviation from 1.0
    pub opacity_delta: f64, // Target opacity delta
}

/// A kinetic text animator combining selectors and transformation properties.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TextAnimator {
    pub name: String,
    pub range_selector: Option<RangeSelector>,
    pub wiggly_selector: Option<WigglySelector>,
    pub mode: Mode,
    pub properties: TextProperties,
}

impl TextAnimator {
    pub fn evaluate_character(&self, idx: usize, total: usize, time_sec: f64) -> f64 {
        let mut weight = self.mode.initial();

        if let Some(r) = &self.range_selector {
            let w = r.evaluate_unit(idx, total);
            weight = self.mode.combine(weight, w);
        }

        if let Some(wig) = &self.wiggly_selector {
            let w = wig.evaluate_unit(idx, time_sec);
            weight = self.mode.combine(weight, w);
        }

        weight
    }
}

/// Evaluate all characters of a string through a set of kinetic text animators.
pub fn evaluate_text_animators(
    text: &str,
    animators: &[TextAnimator],
    time_sec: f64,
) -> Vec<CharTransform> {
    let chars: Vec<char> = text.chars().collect();
    let total = chars.len();
    let mut results = vec![CharTransform::default(); total];

    for anim in animators {
        for (i, xform) in results.iter_mut().enumerate() {
            let weight = anim.evaluate_character(i, total, time_sec);
            if weight.abs() > 1e-6 {
                // Apply position offset
                xform.position[0] += anim.properties.position[0] * weight;
                xform.position[1] += anim.properties.position[1] * weight;
                xform.position[2] += anim.properties.position[2] * weight;

                // Apply rotation offset
                xform.rotation[0] += anim.properties.rotation[0] * weight;
                xform.rotation[1] += anim.properties.rotation[1] * weight;
                xform.rotation[2] += anim.properties.rotation[2] * weight;

                // Apply scale offset
                xform.scale[0] *= 1.0 + anim.properties.scale[0] * weight;
                xform.scale[1] *= 1.0 + anim.properties.scale[1] * weight;
                xform.scale[2] *= 1.0 + anim.properties.scale[2] * weight;

                // Apply opacity delta
                xform.opacity = (xform.opacity + anim.properties.opacity_delta * weight).clamp(0.0, 1.0);
            }
        }
    }

    results
}
