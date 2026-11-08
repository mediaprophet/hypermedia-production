//! Multi-channel non-destructive PBR layer stack compositor.
//! Evaluates layers, blend modes, masks, and opacity into final channel maps.

use crate::material::PbrChannel;
use crate::raster::RasterImage;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlendMode {
    #[default]
    Normal,
    Multiply,
    Screen,
    Overlay,
    Add,
    Subtract,
}

impl BlendMode {
    pub fn blend(self, base: f32, top: f32) -> f32 {
        match self {
            BlendMode::Normal => top,
            BlendMode::Multiply => base * top,
            BlendMode::Screen => 1.0 - (1.0 - base) * (1.0 - top),
            BlendMode::Overlay => {
                if base < 0.5 {
                    2.0 * base * top
                } else {
                    1.0 - 2.0 * (1.0 - base) * (1.0 - top)
                }
            }
            BlendMode::Add => (base + top).clamp(0.0, 1.0),
            BlendMode::Subtract => (base - top).clamp(0.0, 1.0),
        }
    }
}

/// A channel payload in a layer: either a constant value or a raster image.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ChannelSource {
    Uniform([f32; 4]),
    Image(RasterImage),
}

/// A single non-destructive PBR layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PbrLayer {
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub channels: BTreeMap<PbrChannel, ChannelSource>,
    pub mask: Option<RasterImage>,
}

impl PbrLayer {
    pub fn new_fill(name: &str, color: [f32; 4], roughness: f32, metallic: f32) -> Self {
        let mut channels = BTreeMap::new();
        channels.insert(PbrChannel::BaseColor, ChannelSource::Uniform(color));
        channels.insert(PbrChannel::Roughness, ChannelSource::Uniform([roughness; 4]));
        channels.insert(PbrChannel::Metallic, ChannelSource::Uniform([metallic; 4]));
        channels.insert(PbrChannel::Normal, ChannelSource::Uniform([0.5, 0.5, 1.0, 1.0]));
        channels.insert(PbrChannel::AO, ChannelSource::Uniform([1.0; 4]));

        Self {
            name: name.to_string(),
            visible: true,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            channels,
            mask: None,
        }
    }
}

/// A stack of PBR layers evaluated from bottom to top.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PbrLayerStack {
    pub layers: Vec<PbrLayer>,
}

impl PbrLayerStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_layer(&mut self, layer: PbrLayer) {
        self.layers.push(layer);
    }

    /// Evaluates the layer stack into rendered images for all present channels.
    pub fn composite(&self, width: u32, height: u32) -> BTreeMap<PbrChannel, RasterImage> {
        let mut results = BTreeMap::new();

        for &channel in &PbrChannel::ALL {
            let mut active = false;
            for l in &self.layers {
                if l.visible && l.channels.contains_key(&channel) {
                    active = true;
                    break;
                }
            }
            if !active {
                continue;
            }

            let mut comp = RasterImage::filled(width, height, channel.default_value());

            for layer in &self.layers {
                if !layer.visible || layer.opacity <= 0.0 {
                    continue;
                }
                let Some(source) = layer.channels.get(&channel) else {
                    continue;
                };

                for y in 0..height {
                    for x in 0..width {
                        let top_pixel = match source {
                            ChannelSource::Uniform(val) => *val,
                            ChannelSource::Image(img) => {
                                let u = (x as f32 + 0.5) / width as f32;
                                let v = (y as f32 + 0.5) / height as f32;
                                img.sample_bilinear(u, v)
                            }
                        };

                        let mask_val = if let Some(ref mask) = layer.mask {
                            let u = (x as f32 + 0.5) / width as f32;
                            let v = (y as f32 + 0.5) / height as f32;
                            mask.sample_bilinear(u, v)[0]
                        } else {
                            1.0
                        };

                        let effective_alpha = layer.opacity * mask_val;
                        if effective_alpha <= 0.0 {
                            continue;
                        }

                        let base_pixel = comp.pixel(x, y);
                        let mut blended = [0.0f32; 4];

                        for c in 0..3 {
                            let b = layer.blend_mode.blend(base_pixel[c], top_pixel[c]);
                            blended[c] = base_pixel[c] * (1.0 - effective_alpha) + b * effective_alpha;
                        }
                        blended[3] = 1.0;

                        comp.set_pixel(x, y, blended);
                    }
                }
            }

            results.insert(channel, comp);
        }

        results
    }
}
