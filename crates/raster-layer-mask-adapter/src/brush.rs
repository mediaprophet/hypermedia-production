//! Digital painting brush engine: stroke point sampling, dabs, and brush dynamics.
//!
//! Grounded in UDHR Articles 12, 19, 27 and ICESCR Article 15.

use serde::{Deserialize, Serialize};

/// Input sample from mouse, stylus, or touch device.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StrokePoint {
    pub x: f64,
    pub y: f64,
    /// Pressure in `[0.0, 1.0]`. Defaults to 1.0 for mouse.
    pub pressure: f32,
    /// Tilt in degrees -90..=90.
    pub tilt_x: f32,
    pub tilt_y: f32,
    /// Barrel rotation in degrees 0..=360.
    pub rotation: f32,
    /// Airbrush wheel in `[0.0, 1.0]`.
    pub wheel: f32,
    /// Millisecond timestamp.
    pub time: f64,
}

impl Default for StrokePoint {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            pressure: 1.0,
            tilt_x: 0.0,
            tilt_y: 0.0,
            rotation: 0.0,
            wheel: 0.0,
            time: 0.0,
        }
    }
}

/// A rendered stamp footprint (dab) deposited along the stroke path.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dab {
    pub x: f64,
    pub y: f64,
    pub radius: f32,
    pub angle: f32,
    pub opacity: f32,
    pub hardness: f32,
    pub color: [f32; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Control {
    #[default]
    Off,
    Fade,
    PenPressure,
    PenTilt,
    StylusWheel,
    Rotation,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dynamic {
    pub jitter: f32,
    pub control: Control,
    pub fade_steps: u32,
    pub minimum: f32,
}

impl Dynamic {
    pub fn evaluate(&self, point: &StrokePoint, step: usize, rand_val: f32) -> f32 {
        let ctrl_val = match self.control {
            Control::Off => 1.0,
            Control::Fade => {
                if self.fade_steps == 0 {
                    1.0
                } else {
                    (1.0 - (step as f32 / self.fade_steps as f32)).clamp(0.0, 1.0)
                }
            }
            Control::PenPressure => point.pressure,
            Control::PenTilt => (point.tilt_x.abs().max(point.tilt_y.abs()) / 90.0).clamp(0.0, 1.0),
            Control::StylusWheel => point.wheel,
            Control::Rotation => (point.rotation / 360.0).fract(),
        };

        let modulated = self.minimum + (1.0 - self.minimum) * ctrl_val;
        let jittered = modulated * (1.0 - self.jitter * (1.0 - rand_val));
        jittered.clamp(0.0, 1.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ShapeDynamics {
    pub size: Dynamic,
    pub angle: Dynamic,
    pub roundness: Dynamic,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Scattering {
    pub scatter: f32,
    pub both_axes: bool,
    pub count: usize,
    pub count_jitter: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DualBrush {
    pub enabled: bool,
    pub size: f32,
    pub spacing: f32,
    pub scatter: f32,
    pub count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ColorDynamics {
    pub fg_bg_jitter: f32,
    pub hue_jitter: f32,
    pub sat_jitter: f32,
    pub bright_jitter: f32,
    pub purity: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrushSettings {
    pub size: f32,
    pub spacing: f32,
    pub hardness: f32,
    pub opacity: f32,
    pub flow: f32,
    pub shape_dynamics: ShapeDynamics,
    pub scattering: Scattering,
    pub dual_brush: DualBrush,
    pub color_dynamics: ColorDynamics,
}

impl Default for BrushSettings {
    fn default() -> Self {
        Self {
            size: 20.0,
            spacing: 0.25,
            hardness: 0.8,
            opacity: 1.0,
            flow: 1.0,
            shape_dynamics: ShapeDynamics::default(),
            scattering: Scattering::default(),
            dual_brush: DualBrush::default(),
            color_dynamics: ColorDynamics::default(),
        }
    }
}

#[inline]
fn pseudo_rand(seed: u64, index: usize, channel: u64) -> f32 {
    let mut h = seed ^ (index as u64).wrapping_mul(0x517c_c1b7_2722_0a95) ^ channel;
    h ^= h >> 16;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2_ae35);
    h ^= h >> 16;
    ((h & 0xFFFF_FFFF) as f32) / (0xFFFF_FFFFu32 as f32)
}

/// Generates a list of dabs from discrete input points using brush spacing and dynamics.
pub fn generate_dabs(
    points: &[StrokePoint],
    settings: &BrushSettings,
    base_color: [f32; 4],
    seed: u64,
) -> Vec<Dab> {
    if points.is_empty() {
        return Vec::new();
    }
    if points.len() == 1 {
        let p = &points[0];
        let rand_val = pseudo_rand(seed, 0, 1);
        let scale = settings.shape_dynamics.size.evaluate(p, 0, rand_val);
        return vec![Dab {
            x: p.x,
            y: p.y,
            radius: (settings.size * 0.5 * scale).max(0.5),
            angle: p.rotation,
            opacity: settings.opacity * settings.flow * p.pressure,
            hardness: settings.hardness,
            color: base_color,
        }];
    }

    let mut dabs = Vec::new();
    let step_dist = (settings.size * settings.spacing).max(1.0) as f64;
    let mut distance_travelled = 0.0;
    let mut dab_index = 0;

    for i in 0..points.len() - 1 {
        let p0 = &points[i];
        let p1 = &points[i + 1];
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let seg_len = (dx * dx + dy * dy).sqrt();
        if seg_len <= 1e-6 {
            continue;
        }

        let mut t = 0.0;
        while t < seg_len {
            let frac = t / seg_len;
            let current_pt = StrokePoint {
                x: p0.x + dx * frac,
                y: p0.y + dy * frac,
                pressure: p0.pressure + (p1.pressure - p0.pressure) * (frac as f32),
                tilt_x: p0.tilt_x + (p1.tilt_x - p0.tilt_x) * (frac as f32),
                tilt_y: p0.tilt_y + (p1.tilt_y - p0.tilt_y) * (frac as f32),
                rotation: p0.rotation + (p1.rotation - p0.rotation) * (frac as f32),
                wheel: p0.wheel + (p1.wheel - p0.wheel) * (frac as f32),
                time: p0.time + (p1.time - p0.time) * frac,
            };

            let r_size = pseudo_rand(seed, dab_index, 1);
            let r_angle = pseudo_rand(seed, dab_index, 2);
            let r_scatter = pseudo_rand(seed, dab_index, 3);

            let scale = settings.shape_dynamics.size.evaluate(&current_pt, dab_index, r_size);
            let angle_jitter = (r_angle - 0.5) * settings.shape_dynamics.angle.jitter * 360.0;
            let scatter_offset = (r_scatter - 0.5) * settings.scattering.scatter * settings.size;

            let nx = -dy / seg_len;
            let ny = dx / seg_len;

            dabs.push(Dab {
                x: current_pt.x + nx * (scatter_offset as f64),
                y: current_pt.y + ny * (scatter_offset as f64),
                radius: (settings.size * 0.5 * scale).max(0.5),
                angle: current_pt.rotation + angle_jitter,
                opacity: (settings.opacity * settings.flow * current_pt.pressure).clamp(0.0, 1.0),
                hardness: settings.hardness,
                color: base_color,
            });

            dab_index += 1;
            t += step_dist;
        }
        distance_travelled += seg_len;
    }

    dabs
}
