//! Vector Base Amplitude Panning (VBAP) and stereo pan laws for surround and 3D immersive audio.

use serde::{Deserialize, Serialize};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

/// Attenuation curve of a centered stereo signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum PanLaw {
    #[default]
    Minus3,   // -3 dB constant power
    Minus4_5, // -4.5 dB compromise
    Minus6,   // -6 dB constant voltage
    Zero,     // Linear balance
}

/// Computes left and right channel gains for a given pan position (-1.0 hard left .. +1.0 hard right).
pub fn stereo_pan_gains(pan: f32, law: PanLaw) -> (f32, f32) {
    let p = pan.clamp(-1.0, 1.0);
    if law == PanLaw::Zero {
        return ((1.0 - p).min(1.0), (1.0 + p).min(1.0));
    }

    let theta = (p + 1.0) * FRAC_PI_4;
    let (l, r) = (theta.cos().max(0.0), theta.sin().max(0.0));
    let center_db = match law {
        PanLaw::Minus3 => -3.0103,
        PanLaw::Minus4_5 => -4.5,
        PanLaw::Minus6 => -6.0206,
        PanLaw::Zero => 0.0,
    };
    let exp = center_db / -3.0103;
    (l.powf(exp), r.powf(exp))
}

/// A loudspeaker position in spherical coordinates (azimuth, elevation).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpeakerPosition {
    pub name: &'static str,
    pub azimuth_deg: f32,   // 0 = front, +deg = right/east, -deg = left/west
    pub elevation_deg: f32, // 0 = ear level, +deg = height
    pub is_lfe: bool,
}

impl SpeakerPosition {
    pub const fn new(name: &'static str, az: f32, el: f32, is_lfe: bool) -> Self {
        Self { name, azimuth_deg: az, elevation_deg: el, is_lfe }
    }

    /// Unit Cartesian direction vector (x: right, y: front, z: up).
    pub fn to_cartesian(&self) -> (f32, f32, f32) {
        let az_rad = self.azimuth_deg.to_radians();
        let el_rad = self.elevation_deg.to_radians();
        let cos_el = el_rad.cos();
        (az_rad.sin() * cos_el, az_rad.cos() * cos_el, el_rad.sin())
    }
}

/// Standard 5.1 surround speaker setup (ITU-R BS.775).
pub const LAYOUT_5_1: [SpeakerPosition; 6] = [
    SpeakerPosition::new("L", -30.0, 0.0, false),
    SpeakerPosition::new("R", 30.0, 0.0, false),
    SpeakerPosition::new("C", 0.0, 0.0, false),
    SpeakerPosition::new("LFE", 0.0, 0.0, true),
    SpeakerPosition::new("Ls", -110.0, 0.0, false),
    SpeakerPosition::new("Rs", 110.0, 0.0, false),
];

/// Standard 7.1.4 immersive Dolby Atmos / MPEG-H speaker setup.
pub const LAYOUT_7_1_4: [SpeakerPosition; 12] = [
    SpeakerPosition::new("L", -30.0, 0.0, false),
    SpeakerPosition::new("R", 30.0, 0.0, false),
    SpeakerPosition::new("C", 0.0, 0.0, false),
    SpeakerPosition::new("LFE", 0.0, 0.0, true),
    SpeakerPosition::new("Lss", -90.0, 0.0, false),
    SpeakerPosition::new("Rss", 90.0, 0.0, false),
    SpeakerPosition::new("Lsr", -140.0, 0.0, false),
    SpeakerPosition::new("Rsr", 140.0, 0.0, false),
    SpeakerPosition::new("Tfl", -30.0, 35.0, false),
    SpeakerPosition::new("Tfr", 30.0, 35.0, false),
    SpeakerPosition::new("Trl", -135.0, 35.0, false),
    SpeakerPosition::new("Trr", 135.0, 35.0, false),
];

/// Vector Base Amplitude Panning (VBAP) panner for arbitrary speaker layouts.
pub struct VbapPanner<'a> {
    pub speakers: &'a [SpeakerPosition],
}

impl<'a> VbapPanner<'a> {
    pub fn new(speakers: &'a [SpeakerPosition]) -> Self {
        Self { speakers }
    }

    /// Computes speaker gains for a sound source at (azimuth_deg, elevation_deg).
    pub fn pan(&self, azimuth_deg: f32, elevation_deg: f32, out_gains: &mut [f32]) {
        let n = self.speakers.len().min(out_gains.len());
        out_gains[..n].fill(0.0);

        let src_az = azimuth_deg.to_radians();
        let src_el = elevation_deg.to_radians();
        let cos_el = src_el.cos();
        let p = (src_az.sin() * cos_el, src_az.cos() * cos_el, src_el.sin());

        // Find best speaker pair/triplet maximizing dot product
        let mut best_idx1 = 0;
        let mut best_idx2 = 1;
        let mut best_score = -f32::INFINITY;

        for i in 0..n {
            if self.speakers[i].is_lfe {
                continue;
            }
            let v1 = self.speakers[i].to_cartesian();
            for j in (i + 1)..n {
                if self.speakers[j].is_lfe {
                    continue;
                }
                let v2 = self.speakers[j].to_cartesian();

                // 2D projection dot product
                let dot = (v1.0 * p.0 + v1.1 * p.1 + v1.2 * p.2) + (v2.0 * p.0 + v2.1 * p.1 + v2.2 * p.2);
                if dot > best_score {
                    best_score = dot;
                    best_idx1 = i;
                    best_idx2 = j;
                }
            }
        }

        // Two-speaker VBAP solution
        let v1 = self.speakers[best_idx1].to_cartesian();
        let v2 = self.speakers[best_idx2].to_cartesian();

        let dot1 = (v1.0 * p.0 + v1.1 * p.1 + v1.2 * p.2).max(0.0);
        let dot2 = (v2.0 * p.0 + v2.1 * p.1 + v2.2 * p.2).max(0.0);

        let norm = (dot1 * dot1 + dot2 * dot2).sqrt().max(1e-6);
        out_gains[best_idx1] = dot1 / norm;
        out_gains[best_idx2] = dot2 / norm;
    }
}
