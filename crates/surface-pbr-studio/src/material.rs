//! PBR material presets, channel semantics, and physical optical definitions.
//! Grounded in energy-conserving microfacet physics.

use serde::{Deserialize, Serialize};

/// Standard PBR texture channels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PbrChannel {
    BaseColor,
    Roughness,
    Metallic,
    Normal,
    Height,
    AO,
    Emissive,
    Transmission,
    Opacity,
}

impl PbrChannel {
    pub const ALL: [PbrChannel; 9] = [
        PbrChannel::BaseColor,
        PbrChannel::Roughness,
        PbrChannel::Metallic,
        PbrChannel::Normal,
        PbrChannel::Height,
        PbrChannel::AO,
        PbrChannel::Emissive,
        PbrChannel::Transmission,
        PbrChannel::Opacity,
    ];

    pub fn id(self) -> &'static str {
        match self {
            PbrChannel::BaseColor => "base_color",
            PbrChannel::Roughness => "roughness",
            PbrChannel::Metallic => "metallic",
            PbrChannel::Normal => "normal",
            PbrChannel::Height => "height",
            PbrChannel::AO => "ao",
            PbrChannel::Emissive => "emissive",
            PbrChannel::Transmission => "transmission",
            PbrChannel::Opacity => "opacity",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            PbrChannel::BaseColor => "Base Color",
            PbrChannel::Roughness => "Roughness",
            PbrChannel::Metallic => "Metallic",
            PbrChannel::Normal => "Normal",
            PbrChannel::Height => "Height",
            PbrChannel::AO => "Ambient Occlusion",
            PbrChannel::Emissive => "Emissive",
            PbrChannel::Transmission => "Transmission",
            PbrChannel::Opacity => "Opacity",
        }
    }

    pub fn default_value(self) -> [f32; 4] {
        match self {
            PbrChannel::BaseColor => [0.8, 0.8, 0.8, 1.0],
            PbrChannel::Roughness => [0.5, 0.5, 0.5, 1.0],
            PbrChannel::Metallic => [0.0, 0.0, 0.0, 1.0],
            PbrChannel::Normal => [0.5, 0.5, 1.0, 1.0],
            PbrChannel::Height => [0.0, 0.0, 0.0, 1.0],
            PbrChannel::AO => [1.0, 1.0, 1.0, 1.0],
            PbrChannel::Emissive => [0.0, 0.0, 0.0, 1.0],
            PbrChannel::Transmission => [0.0, 0.0, 0.0, 1.0],
            PbrChannel::Opacity => [1.0, 1.0, 1.0, 1.0],
        }
    }
}

/// A physical PBR material preset.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaterialPreset {
    pub name: String,
    pub base_color: [f32; 4],
    pub roughness: f32,
    pub metallic: f32,
    pub ior: f32,
    pub transmission: f32,
    pub subsurface: f32,
    pub emissive: [f32; 3],
}

impl MaterialPreset {
    pub fn metal(name: &str, color: [f32; 4], roughness: f32) -> Self {
        Self {
            name: name.to_string(),
            base_color: color,
            roughness,
            metallic: 1.0,
            ior: 1.5,
            transmission: 0.0,
            subsurface: 0.0,
            emissive: [0.0, 0.0, 0.0],
        }
    }

    pub fn dielectric(name: &str, color: [f32; 4], roughness: f32, ior: f32) -> Self {
        Self {
            name: name.to_string(),
            base_color: color,
            roughness,
            metallic: 0.0,
            ior,
            transmission: 0.0,
            subsurface: 0.0,
            emissive: [0.0, 0.0, 0.0],
        }
    }

    pub fn glass(name: &str, roughness: f32, ior: f32) -> Self {
        Self {
            name: name.to_string(),
            base_color: [1.0, 1.0, 1.0, 1.0],
            roughness,
            metallic: 0.0,
            ior,
            transmission: 1.0,
            subsurface: 0.0,
            emissive: [0.0, 0.0, 0.0],
        }
    }

    pub fn car_paint(name: &str, color: [f32; 4], roughness: f32) -> Self {
        Self {
            name: name.to_string(),
            base_color: color,
            roughness,
            metallic: 0.85,
            ior: 1.5,
            transmission: 0.0,
            subsurface: 0.0,
            emissive: [0.0, 0.0, 0.0],
        }
    }

    pub fn skin(name: &str, color: [f32; 4]) -> Self {
        Self {
            name: name.to_string(),
            base_color: color,
            roughness: 0.45,
            metallic: 0.0,
            ior: 1.4,
            transmission: 0.0,
            subsurface: 0.5,
            emissive: [0.0, 0.0, 0.0],
        }
    }

    pub fn fabric(name: &str, color: [f32; 4], roughness: f32) -> Self {
        Self {
            name: name.to_string(),
            base_color: color,
            roughness,
            metallic: 0.0,
            ior: 1.54,
            transmission: 0.0,
            subsurface: 0.15,
            emissive: [0.0, 0.0, 0.0],
        }
    }
}

/// Catalog of standard calibrated PBR materials.
pub fn standard_presets() -> Vec<MaterialPreset> {
    vec![
        MaterialPreset::metal("Gold", [1.0, 0.766, 0.336, 1.0], 0.2),
        MaterialPreset::metal("Copper", [0.955, 0.637, 0.538, 1.0], 0.25),
        MaterialPreset::metal("Aluminum", [0.913, 0.921, 0.925, 1.0], 0.22),
        MaterialPreset::metal("Iron", [0.56, 0.57, 0.58, 1.0], 0.35),
        MaterialPreset::dielectric("Plastic Red", [0.8, 0.1, 0.1, 1.0], 0.3, 1.5),
        MaterialPreset::dielectric("Plastic Blue", [0.1, 0.2, 0.8, 1.0], 0.3, 1.5),
        MaterialPreset::dielectric("Rubber", [0.05, 0.05, 0.05, 1.0], 0.8, 1.52),
        MaterialPreset::dielectric("Ceramic", [0.92, 0.92, 0.9, 1.0], 0.15, 1.5),
        MaterialPreset::dielectric("Wood", [0.45, 0.28, 0.14, 1.0], 0.6, 1.55),
        MaterialPreset::dielectric("Concrete", [0.6, 0.6, 0.58, 1.0], 0.85, 1.5),
        MaterialPreset::glass("Clear Glass", 0.02, 1.52),
        MaterialPreset::glass("Frosted Glass", 0.35, 1.52),
        MaterialPreset::car_paint("Car Paint Ruby", [0.7, 0.05, 0.1, 1.0], 0.1),
        MaterialPreset::skin("Human Skin Light", [0.9, 0.75, 0.68, 1.0]),
        MaterialPreset::fabric("Cotton Denim", [0.15, 0.25, 0.45, 1.0], 0.85),
    ]
}

pub fn get_preset_by_name(name: &str) -> Option<MaterialPreset> {
    standard_presets().into_iter().find(|p| p.name.eq_ignore_ascii_case(name))
}
