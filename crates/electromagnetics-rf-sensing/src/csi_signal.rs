//! Wi-Fi 802.11n/ax/be Channel State Information (CSI) matrix processing, phase unwrapping,
//! linear phase sanitization, Hampel outlier filtering, Fresnel zone geometry, and Doppler spectrum estimation.

use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

/// Speed of light in vacuum (m/s).
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// Complex 32-bit floating point number for I/Q subcarrier representation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Complex32 {
    pub re: f32,
    pub im: f32,
}

impl Complex32 {
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };

    pub const fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    pub fn from_polar(r: f32, theta: f32) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    #[inline]
    pub fn abs(self) -> f32 {
        self.re.hypot(self.im)
    }

    #[inline]
    pub fn arg(self) -> f32 {
        self.im.atan2(self.re)
    }

    #[inline]
    pub fn norm_sq(self) -> f32 {
        self.re * self.re + self.im * self.im
    }

    #[inline]
    pub fn conj(self) -> Self {
        Self { re: self.re, im: -self.im }
    }

    #[inline]
    pub fn add(self, other: Self) -> Self {
        Self { re: self.re + other.re, im: self.im + other.im }
    }

    #[inline]
    pub fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    #[inline]
    pub fn scale(self, factor: f32) -> Self {
        Self { re: self.re * factor, im: self.im * factor }
    }
}

/// A captured raw CSI packet frame.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsiFrame {
    pub timestamp_ns: u64,
    pub carrier_freq_hz: f64,
    pub num_tx: usize,
    pub num_rx: usize,
    pub subcarrier_indices: Vec<i32>,
    pub subcarriers: Vec<Complex32>, // Flattened: tx * rx * num_subcarriers
}

impl CsiFrame {
    pub fn new(
        timestamp_ns: u64,
        carrier_freq_hz: f64,
        num_tx: usize,
        num_rx: usize,
        subcarrier_indices: Vec<i32>,
        subcarriers: Vec<Complex32>,
    ) -> Self {
        Self {
            timestamp_ns,
            carrier_freq_hz,
            num_tx,
            num_rx,
            subcarrier_indices,
            subcarriers,
        }
    }

    /// Extracts amplitudes for a specific Tx-Rx antenna pair across all subcarriers.
    pub fn get_amplitudes(&self, tx: usize, rx: usize) -> Vec<f32> {
        let n_sub = self.subcarrier_indices.len();
        let offset = (tx * self.num_rx + rx) * n_sub;
        if offset + n_sub <= self.subcarriers.len() {
            self.subcarriers[offset..offset + n_sub].iter().map(|c| c.abs()).collect()
        } else {
            Vec::new()
        }
    }

    /// Extracts raw phases (radians) for a specific Tx-Rx antenna pair.
    pub fn get_phases(&self, tx: usize, rx: usize) -> Vec<f32> {
        let n_sub = self.subcarrier_indices.len();
        let offset = (tx * self.num_rx + rx) * n_sub;
        if offset + n_sub <= self.subcarriers.len() {
            self.subcarriers[offset..offset + n_sub].iter().map(|c| c.arg()).collect()
        } else {
            Vec::new()
        }
    }
}

/// Unwraps phase values along a sequence to remove 2pi phase wrap discontinuities.
pub fn unwrap_phase(phases: &[f32]) -> Vec<f32> {
    if phases.is_empty() {
        return Vec::new();
    }
    let mut unwrapped = Vec::with_capacity(phases.len());
    unwrapped.push(phases[0]);
    let mut cumulative_adjust = 0.0f32;

    for i in 1..phases.len() {
        let diff = phases[i] - phases[i - 1];
        if diff > PI {
            cumulative_adjust -= 2.0 * PI;
        } else if diff < -PI {
            cumulative_adjust += 2.0 * PI;
        }
        unwrapped.push(phases[i] + cumulative_adjust);
    }
    unwrapped
}

/// Removes Carrier Frequency Offset (CFO) and Sampling Time Offset (STO) via linear regression.
/// Sanitized phase: $\tilde{\phi}_i = \phi_i - a \cdot k_i - b$
pub fn sanitize_csi_phase(subcarrier_indices: &[i32], raw_phases: &[f32]) -> Vec<f32> {
    let n = subcarrier_indices.len().min(raw_phases.len());
    if n < 2 {
        return raw_phases.to_vec();
    }

    let unwrapped = unwrap_phase(&raw_phases[..n]);

    // Fit linear line: phi = a * k + b
    let mut sum_k = 0.0f64;
    let mut sum_k2 = 0.0f64;
    let mut sum_phi = 0.0f64;
    let mut sum_k_phi = 0.0f64;

    for i in 0..n {
        let k = subcarrier_indices[i] as f64;
        let phi = unwrapped[i] as f64;
        sum_k += k;
        sum_k2 += k * k;
        sum_phi += phi;
        sum_k_phi += k * phi;
    }

    let n_f = n as f64;
    let denom = n_f * sum_k2 - sum_k * sum_k;
    let (a, b) = if denom.abs() > 1e-12 {
        let slope = (n_f * sum_k_phi - sum_k * sum_phi) / denom;
        let intercept = (sum_phi - slope * sum_k) / n_f;
        (slope as f32, intercept as f32)
    } else {
        (0.0, (sum_phi / n_f) as f32)
    };

    let mut sanitized = Vec::with_capacity(n);
    for i in 0..n {
        let k = subcarrier_indices[i] as f32;
        sanitized.push(unwrapped[i] - a * k - b);
    }
    sanitized
}

/// Hampel outlier removal filter: replaces outliers with local window median.
pub fn hampel_filter(signal: &[f32], half_window: usize, n_sigma: f32) -> Vec<f32> {
    let n = signal.len();
    if n == 0 {
        return Vec::new();
    }
    let mut out = signal.to_vec();

    for i in 0..n {
        let start = i.saturating_sub(half_window);
        let end = (i + half_window + 1).min(n);
        let mut window: Vec<f32> = signal[start..end].to_vec();
        window.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let median = window[window.len() / 2];

        // Median Absolute Deviation (MAD)
        let mut abs_dev: Vec<f32> = window.iter().map(|&x| (x - median).abs()).collect();
        abs_dev.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mad = abs_dev[abs_dev.len() / 2];

        let threshold = n_sigma * 1.4826 * mad;
        if (signal[i] - median).abs() > threshold && threshold > 1e-6 {
            out[i] = median;
        }
    }
    out
}

/// Fresnel zone spatial geometry between transmitter (Tx) and receiver (Rx).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FresnelZoneGeometry {
    pub tx_pos: (f64, f64, f64),
    pub rx_pos: (f64, f64, f64),
    pub wavelength_m: f64,
    pub direct_distance_m: f64,
}

impl FresnelZoneGeometry {
    pub fn new(tx_pos: (f64, f64, f64), rx_pos: (f64, f64, f64), carrier_freq_hz: f64) -> Self {
        let dx = rx_pos.0 - tx_pos.0;
        let dy = rx_pos.1 - tx_pos.1;
        let dz = rx_pos.2 - tx_pos.2;
        let direct_dist = (dx * dx + dy * dy + dz * dz).sqrt();
        let wavelength = SPEED_OF_LIGHT / carrier_freq_hz.max(1e6);

        Self {
            tx_pos,
            rx_pos,
            wavelength_m: wavelength,
            direct_distance_m: direct_dist,
        }
    }

    /// Calculates the radius $r_n$ of the $n$-th Fresnel zone at distance $d_1$ from Tx along the LOS path.
    pub fn radius_at_distance(&self, zone_number: usize, d1_m: f64) -> f64 {
        let d = self.direct_distance_m;
        let d1 = d1_m.clamp(0.01, d - 0.01);
        let d2 = d - d1;
        let n = zone_number.max(1) as f64;

        ((n * self.wavelength_m * d1 * d2) / d).sqrt()
    }
}

/// Computes a short-time Doppler velocity spectrum from a time-series of CSI amplitude changes.
pub fn compute_doppler_spectrum(
    amplitudes: &[f32],
    sampling_rate_hz: f32,
    carrier_freq_hz: f64,
    fft_size: usize,
) -> Vec<(f32, f32)> { // (velocity_m_s, power)
    let n = fft_size.min(amplitudes.len());
    if n < 8 {
        return Vec::new();
    }

    let wavelength = (SPEED_OF_LIGHT / carrier_freq_hz) as f32;

    // Apply Hann window and compute DFT
    let mut spectrum = Vec::with_capacity(n / 2);
    for k in 0..(n / 2) {
        let mut re = 0.0f32;
        let mut im = 0.0f32;
        for t in 0..n {
            let w = 0.5 * (1.0 - (2.0 * PI * (t as f32) / (n as f32)).cos());
            let angle = -2.0 * PI * (k as f32) * (t as f32) / (n as f32);
            let s = amplitudes[t] * w;
            re += s * angle.cos();
            im += s * angle.sin();
        }
        let power = re * re + im * im;
        let freq_hz = (k as f32) * sampling_rate_hz / (n as f32);
        let velocity_m_s = freq_hz * wavelength / 2.0;
        spectrum.push((velocity_m_s, power));
    }

    spectrum
}
