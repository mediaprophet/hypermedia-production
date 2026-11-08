//! Antenna array spatial covariance, beamforming, and MUSIC Angle-of-Arrival (AoA) estimation.

use crate::csi_signal::Complex32;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

/// Uniform Linear Array (ULA) configuration.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct UniformLinearArray {
    pub num_elements: usize,
    pub element_spacing_m: f32, // typically lambda / 2
    pub carrier_freq_hz: f64,
}

impl UniformLinearArray {
    pub fn new(num_elements: usize, carrier_freq_hz: f64) -> Self {
        let wavelength = (crate::csi_signal::SPEED_OF_LIGHT / carrier_freq_hz) as f32;
        Self {
            num_elements: num_elements.max(2),
            element_spacing_m: wavelength / 2.0,
            carrier_freq_hz,
        }
    }

    /// Computes steering vector $a(\theta)$ for incidence angle $\theta$ (degrees, -90..+90).
    pub fn steering_vector(&self, angle_deg: f32) -> Vec<Complex32> {
        let wavelength = (crate::csi_signal::SPEED_OF_LIGHT / self.carrier_freq_hz) as f32;
        let theta_rad = angle_deg.to_radians();
        let sin_theta = theta_rad.sin();
        let mut a = Vec::with_capacity(self.num_elements);

        for m in 0..self.num_elements {
            let phase = -2.0 * PI * (m as f32) * self.element_spacing_m * sin_theta / wavelength;
            a.push(Complex32::from_polar(1.0, phase));
        }
        a
    }
}

/// Computes the $M \times M$ complex spatial covariance matrix $R_{xx} = \frac{1}{K} \sum x_k x_k^H$.
pub fn compute_spatial_covariance(snapshots: &[Vec<Complex32>], num_antennas: usize) -> Vec<Complex32> {
    let mut r = vec![Complex32::ZERO; num_antennas * num_antennas];
    if snapshots.is_empty() {
        return r;
    }

    for snap in snapshots {
        if snap.len() < num_antennas {
            continue;
        }
        for i in 0..num_antennas {
            for j in 0..num_antennas {
                // snap[i] * conj(snap[j])
                let prod = snap[i].mul(snap[j].conj());
                r[i * num_antennas + j] = r[i * num_antennas + j].add(prod);
            }
        }
    }

    let inv_k = 1.0 / (snapshots.len() as f32);
    for val in &mut r {
        *val = val.scale(inv_k);
    }

    r
}

/// Evaluates MUSIC pseudospectrum over angular search grid (-90..+90 degrees).
pub fn music_aoa_spectrum(
    array: &UniformLinearArray,
    snapshots: &[Vec<Complex32>],
    num_signals: usize,
    angular_step_deg: f32,
) -> Vec<(f32, f32)> { // (angle_deg, pseudospectrum_power)
    let m = array.num_elements;
    let r_xx = compute_spatial_covariance(snapshots, m);

    // Power iteration / deflation to approximate signal eigenvectors and construct noise projector
    // For small M (3 or 4 antennas typical in Wi-Fi), project out the dominant signal eigenvectors
    let signal_eigenvectors = extract_dominant_eigenvectors(&r_xx, m, num_signals);

    let mut spectrum = Vec::new();
    let mut angle = -90.0f32;
    while angle <= 90.0 {
        let a = array.steering_vector(angle);

        // Project steering vector onto noise subspace: ||E_n^H a||^2 = ||a||^2 - \sum |s_k^H a|^2
        let a_norm_sq = (m as f32); // Each element has magnitude 1
        let mut proj_signal_sq = 0.0f32;

        for s_vec in &signal_eigenvectors {
            let mut dot = Complex32::ZERO;
            for i in 0..m {
                dot = dot.add(s_vec[i].conj().mul(a[i]));
            }
            proj_signal_sq += dot.norm_sq();
        }

        let noise_denom = (a_norm_sq - proj_signal_sq).max(1e-5);
        let pseudospectrum = 1.0 / noise_denom;
        spectrum.push((angle, pseudospectrum));

        angle += angular_step_deg.max(0.5);
    }

    spectrum
}

/// Finds the dominant peak in a MUSIC spectrum.
pub fn find_dominant_aoa_peak(spectrum: &[(f32, f32)]) -> Option<f32> {
    if spectrum.len() < 3 {
        return None;
    }
    let mut best_angle = 0.0f32;
    let mut max_power = -f32::INFINITY;

    for i in 1..(spectrum.len() - 1) {
        let (ang, pow) = spectrum[i];
        if pow > spectrum[i - 1].1 && pow > spectrum[i + 1].1 && pow > max_power {
            max_power = pow;
            best_angle = ang;
        }
    }

    Some(best_angle)
}

fn extract_dominant_eigenvectors(r: &[Complex32], m: usize, num_signals: usize) -> Vec<Vec<Complex32>> {
    let mut e_sigs = Vec::new();
    let mut current_r = r.to_vec();

    for _ in 0..num_signals.min(m.saturating_sub(1)) {
        // Power iteration to find dominant eigenvector
        let mut v = vec![Complex32::new(1.0, 0.0); m];
        for _ in 0..20 {
            let mut next_v = vec![Complex32::ZERO; m];
            for i in 0..m {
                for j in 0..m {
                    next_v[i] = next_v[i].add(current_r[i * m + j].mul(v[j]));
                }
            }
            let norm = next_v.iter().map(|c| c.norm_sq()).sum::<f32>().sqrt().max(1e-6);
            for x in &mut next_v {
                *x = x.scale(1.0 / norm);
            }
            v = next_v;
        }

        // Deflate matrix: R = R - lambda * v * v^H
        let mut r_v = vec![Complex32::ZERO; m];
        for i in 0..m {
            for j in 0..m {
                r_v[i] = r_v[i].add(current_r[i * m + j].mul(v[j]));
            }
        }
        let mut lambda = 0.0f32;
        for i in 0..m {
            lambda += (v[i].conj().mul(r_v[i])).re;
        }

        for i in 0..m {
            for j in 0..m {
                let outer = v[i].mul(v[j].conj()).scale(lambda);
                current_r[i * m + j] = Complex32::new(
                    current_r[i * m + j].re - outer.re,
                    current_r[i * m + j].im - outer.im,
                );
            }
        }

        e_sigs.push(v);
    }

    e_sigs
}
