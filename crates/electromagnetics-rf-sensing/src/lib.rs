//! Pure-Rust Wi-Fi 802.11n/ax/be Channel State Information (CSI) spatial sensing,
//! contactless vital signs extraction, MUSIC Angle-of-Arrival (AoA) beamforming,
//! 17-keypoint RF skeletal pose, and camera-free privacy witness verification.
//!
//! Grounded in international human rights instruments (UDHR Articles 12, 18, 19, 27; ICCPR Article 17)
//! and Informatics Fiduciary duties ensuring camera-free privacy-first physical sensing.

#![forbid(unsafe_code)]

pub mod beamforming;
pub mod csi_signal;
pub mod densepose;
pub mod vitals;

pub use beamforming::{
    UniformLinearArray, compute_spatial_covariance, find_dominant_aoa_peak, music_aoa_spectrum,
};
pub use csi_signal::{
    Complex32, CsiFrame, FresnelZoneGeometry, SPEED_OF_LIGHT, compute_doppler_spectrum,
    hampel_filter, sanitize_csi_phase, unwrap_phase,
};
pub use densepose::{
    KeypointType, PrivacyWitnessReceipt, RfKeypoint, RfSkeletalPose,
};
pub use vitals::{FallDetector, FallState, VitalSigns, extract_vital_signs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csi_phase_unwrapping_and_sanitization() {
        let indices = vec![-28, -20, -10, 0, 10, 20, 28];
        // Create wrapped phases with a linear slope plus a 2pi wrap
        let mut raw_phases = Vec::new();
        for &k in &indices {
            let phi = (k as f32) * 0.15 + 1.2;
            raw_phases.push((phi + std::f32::consts::PI).rem_euclid(2.0 * std::f32::consts::PI) - std::f32::consts::PI);
        }

        let unwrapped = unwrap_phase(&raw_phases);
        assert_eq!(unwrapped.len(), raw_phases.len());

        let sanitized = sanitize_csi_phase(&indices, &raw_phases);
        assert_eq!(sanitized.len(), raw_phases.len());
        // After linear regression removal, sanitized residual should have near-zero mean
        let mean: f32 = sanitized.iter().sum::<f32>() / (sanitized.len() as f32);
        assert!(mean.abs() < 1e-3, "Sanitized phase mean: {mean}");
    }

    #[test]
    fn test_vital_signs_extraction() {
        let sr = 50.0; // 50 Hz CSI packet rate
        let duration_sec = 10.0;
        let num_samples = (sr * duration_sec) as usize;

        // Synthetic breathing (0.25 Hz = 15 BPM) and cardiac pulse (1.2 Hz = 72 BPM)
        let mut signal = Vec::with_capacity(num_samples);
        for i in 0..num_samples {
            let t = (i as f32) / sr;
            let resp = 0.5 * (2.0 * std::f32::consts::PI * 0.25 * t).sin();
            let heart = 0.1 * (2.0 * std::f32::consts::PI * 1.2 * t).sin();
            signal.push(resp + heart);
        }

        let vitals = extract_vital_signs(&signal, sr).expect("Vital sign extraction failed");
        assert!((vitals.breathing_bpm - 15.0).abs() < 3.0, "Estimated breathing: {}", vitals.breathing_bpm);
        assert!((vitals.heart_rate_bpm - 72.0).abs() < 5.0, "Estimated heart rate: {}", vitals.heart_rate_bpm);
    }

    #[test]
    fn test_doppler_fall_detection() {
        let mut detector = FallDetector::new();
        assert_eq!(detector.state, FallState::Normal);

        // Frame 1: Sudden high velocity impact spike
        let s1 = detector.update(2.2, 12.0);
        assert_eq!(s1, FallState::PreImpactSpike);

        // Frame 2: Quiescent post-impact
        let s2 = detector.update(0.1, 0.2);
        assert_eq!(s2, FallState::PostImpactQuiescence(1));

        // Frame 3: Continued quiescence
        let s3 = detector.update(0.1, 0.2);
        assert_eq!(s3, FallState::PostImpactQuiescence(2));

        // Frame 4: 3-frame debounce complete -> Fall confirmed
        let s4 = detector.update(0.1, 0.2);
        assert_eq!(s4, FallState::FallConfirmed);
    }

    #[test]
    fn test_music_aoa_peak_detection() {
        let ula = UniformLinearArray::new(4, 5.18e9); // 4 antennas, 5.18 GHz Wi-Fi
        let target_angle = 30.0f32; // Incoming signal from +30 degrees

        // Generate synthetic snapshots with incident wavefront at +30 degrees
        let mut snapshots = Vec::new();
        for t in 0..64 {
            let carrier = Complex32::from_polar(1.0, (t as f32) * 0.2);
            let s_vec = ula.steering_vector(target_angle);
            let snap: Vec<Complex32> = s_vec.into_iter().map(|a| a.mul(carrier)).collect();
            snapshots.push(snap);
        }

        let spectrum = music_aoa_spectrum(&ula, &snapshots, 1, 1.0);
        let peak_angle = find_dominant_aoa_peak(&spectrum).expect("No MUSIC peak found");
        assert!((peak_angle - target_angle).abs() <= 2.0, "Estimated AoA: {peak_angle}, expected: {target_angle}");
    }

    #[test]
    fn test_privacy_witness_receipt_verification() {
        let secret_seed = [42u8; 32];
        let receipt = PrivacyWitnessReceipt::create(
            "did:qualia:rf-sensor-unit-01",
            1731000000_000_000_000,
            1,
            true,
            &secret_seed,
        );

        assert!(receipt.verify(), "Valid privacy witness receipt should verify");

        // Tamper with occupancy count
        let mut tampered = receipt.clone();
        tampered.occupancy_count = 5;
        assert!(!tampered.verify(), "Tampered receipt must fail verification");
    }
}
