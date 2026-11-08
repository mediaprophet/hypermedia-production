//! Clean-room brush format and dynamics parser for tablet painting and 3D surface texturing.
//! Parses brush parameters and generates parametric or stamp-based tip masks.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Dynamics and stamp settings for a digital brush.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrushDynamics {
    pub name: String,
    pub size: f32,
    pub min_size: f32,
    pub opacity: f32,
    pub spacing: f32,
    pub hardness: f32,
    pub jitter: f32,
    pub scatter: f32,
    pub pressure_size: f32,
    pub pressure_opacity: f32,
}

impl Default for BrushDynamics {
    fn default() -> Self {
        Self {
            name: "Round Brush".to_string(),
            size: 32.0,
            min_size: 2.0,
            opacity: 1.0,
            spacing: 0.1,
            hardness: 0.8,
            jitter: 0.0,
            scatter: 0.0,
            pressure_size: 0.8,
            pressure_opacity: 0.5,
        }
    }
}

/// A grayscale brush tip stamp.
#[derive(Clone, Debug, PartialEq)]
pub struct BrushTipStamp {
    pub width: u32,
    pub height: u32,
    pub mask: Vec<u8>,
}

impl BrushTipStamp {
    /// Generates a procedural circular tip mask with smooth falloff.
    pub fn procedural_round(size: u32, hardness: f32) -> Self {
        let size = size.max(2);
        let radius = size as f32 * 0.5;
        let inner_radius = radius * hardness.clamp(0.0, 1.0);
        let falloff = (radius - inner_radius).max(1e-4);

        let mut mask = vec![0u8; (size * size) as usize];
        let center = radius;

        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 + 0.5 - center;
                let dy = y as f32 + 0.5 - center;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= radius {
                    let factor = if dist <= inner_radius {
                        1.0
                    } else {
                        1.0 - (dist - inner_radius) / falloff
                    };
                    mask[(y * size + x) as usize] = (factor * 255.0).clamp(0.0, 255.0) as u8;
                }
            }
        }

        Self {
            width: size,
            height: size,
            mask,
        }
    }
}

/// Simple clean-room dictionary property for brush archives.
#[derive(Clone, Debug, PartialEq)]
pub enum BrushProp {
    Bool(bool),
    Number(f64),
    Text(String),
    Dict(BTreeMap<String, BrushProp>),
}

/// Parses clean-room brush settings from key-value pairs.
pub fn parse_brush_properties(dict: &BTreeMap<String, BrushProp>, fallback_name: &str) -> BrushDynamics {
    let mut brush = BrushDynamics::default();

    if let Some(BrushProp::Text(name)) = dict.get("name") {
        brush.name = name.clone();
    } else {
        brush.name = fallback_name.to_string();
    }

    if let Some(BrushProp::Number(s)) = dict.get("size").or_else(|| dict.get("paintSize")) {
        brush.size = (*s as f32).clamp(1.0, 1024.0);
    }

    if let Some(BrushProp::Number(sp)) = dict.get("spacing") {
        brush.spacing = (*sp as f32).clamp(0.01, 2.0);
    }

    if let Some(BrushProp::Number(h)) = dict.get("hardness") {
        brush.hardness = (*h as f32).clamp(0.0, 1.0);
    }

    if let Some(BrushProp::Number(op)) = dict.get("opacity") {
        brush.opacity = (*op as f32).clamp(0.0, 1.0);
    }

    if let Some(BrushProp::Number(ps)) = dict.get("pressureSize") {
        brush.pressure_size = (*ps as f32).clamp(0.0, 1.0);
    }

    brush
}
