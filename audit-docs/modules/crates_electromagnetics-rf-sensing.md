# `crates/electromagnetics-rf-sensing`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Pure-Rust Wi-Fi 802.11n/ax/be Channel State Information (CSI) spatial sensing,
contactless vital signs extraction, MUSIC Angle-of-Arrival (AoA) beamforming,
17-keypoint RF skeletal pose, and camera-free privacy witness verification.

Grounded in international human rights instruments (UDHR Articles 12, 18, 19, 27; ICCPR Article 17)
and Informatics Fiduciary duties ensuring camera-free privacy-first physical sensing._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl UniformLinearArray::new` | 2 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:16` | - |  |
| `impl UniformLinearArray::steering_vector` | 2 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:26` | - | Computes steering vector $a(\theta)$ for incidence angle $\theta$ (degrees, -90..+90). |
| `compute_spatial_covariance` | 2 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:41` | numerics | Computes the $M \times M$ complex spatial covariance matrix $R_{xx} = \frac{1}{K} \sum x_k x_k^H$. |
| `music_aoa_spectrum` | 4 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:69` | - | Evaluates MUSIC pseudospectrum over angular search grid (-90..+90 degrees). |
| `find_dominant_aoa_peak` | 1 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:110` | - | Finds the dominant peak in a MUSIC spectrum. |
| `extract_dominant_eigenvectors` | 3 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:128` | - |  |
| `impl Complex32::new` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:20` | - |  |
| `impl Complex32::from_polar` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:24` | - |  |
| `impl Complex32::abs` | 1 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:32` | - |  |
| `impl Complex32::arg` | 1 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:37` | - |  |
| `impl Complex32::norm_sq` | 1 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:42` | - |  |
| `impl Complex32::conj` | 1 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:47` | - |  |
| `impl Complex32::add` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:52` | - |  |
| `impl Complex32::mul` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:57` | - |  |
| `impl Complex32::scale` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:65` | - |  |
| `impl CsiFrame::new` | 6 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:82` | - |  |
| `impl CsiFrame::get_amplitudes` | 3 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:101` | - | Extracts amplitudes for a specific Tx-Rx antenna pair across all subcarriers. |
| `impl CsiFrame::get_phases` | 3 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:112` | - | Extracts raw phases (radians) for a specific Tx-Rx antenna pair. |
| `unwrap_phase` | 1 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:124` | - | Unwraps phase values along a sequence to remove 2pi phase wrap discontinuities. |
| `sanitize_csi_phase` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:146` | numerics | Removes Carrier Frequency Offset (CFO) and Sampling Time Offset (STO) via linear regression. |
| `hampel_filter` | 3 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:188` | - | Hampel outlier removal filter: replaces outliers with local window median. |
| `impl FresnelZoneGeometry::new` | 3 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:226` | - |  |
| `impl FresnelZoneGeometry::radius_at_distance` | 3 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:242` | - | Calculates the radius $r_n$ of the $n$-th Fresnel zone at distance $d_1$ from Tx along the LOS path. |
| `compute_doppler_spectrum` | 4 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:253` | physics, numerics | Computes a short-time Doppler velocity spectrum from a time-series of CSI amplitude changes. |
| `impl RfSkeletalPose::synthesize_standing` | 4 | `crates/electromagnetics-rf-sensing/src/densepose.rs:72` | - | Generates a normalized human skeletal pose centered at (center_x, center_depth, height_z). |
| `impl PrivacyWitnessReceipt::create` | 5 | `crates/electromagnetics-rf-sensing/src/densepose.rs:125` | - | Creates and cryptographically signs a privacy witness receipt. |
| `impl PrivacyWitnessReceipt::verify` | 1 | `crates/electromagnetics-rf-sensing/src/densepose.rs:158` | - | Verifies the cryptographic witness receipt integrity and human-rights compliance guarantee. |
| `simple_fnv1a` | 1 | `crates/electromagnetics-rf-sensing/src/densepose.rs:175` | - |  |
| `test_csi_phase_unwrapping_and_sanitization` | 0 | `crates/electromagnetics-rf-sensing/src/lib.rs:32` | - |  |
| `test_vital_signs_extraction` | 0 | `crates/electromagnetics-rf-sensing/src/lib.rs:52` | - |  |
| `test_doppler_fall_detection` | 0 | `crates/electromagnetics-rf-sensing/src/lib.rs:72` | - |  |
| `test_music_aoa_peak_detection` | 0 | `crates/electromagnetics-rf-sensing/src/lib.rs:94` | - |  |
| `test_privacy_witness_receipt_verification` | 0 | `crates/electromagnetics-rf-sensing/src/lib.rs:113` | - |  |
| `impl BandpassFilter::new` | 3 | `crates/electromagnetics-rf-sensing/src/vitals.rs:39` | - |  |
| `impl BandpassFilter::process` | 2 | `crates/electromagnetics-rf-sensing/src/vitals.rs:63` | - |  |
| `extract_vital_signs` | 2 | `crates/electromagnetics-rf-sensing/src/vitals.rs:74` | - | Extracts respiration and cardiac pulse rates from a time-series of CSI amplitude / sanitized phase variations. |
| `estimate_frequency_zero_crossings` | 2 | `crates/electromagnetics-rf-sensing/src/vitals.rs:105` | - |  |
| `estimate_frequency_autocorr` | 4 | `crates/electromagnetics-rf-sensing/src/vitals.rs:120` | - |  |
| `impl Default for FallDetector::default` | 0 | `crates/electromagnetics-rf-sensing/src/vitals.rs:151` | - |  |
| `impl FallDetector::new` | 0 | `crates/electromagnetics-rf-sensing/src/vitals.rs:157` | - |  |
| `impl FallDetector::update` | 3 | `crates/electromagnetics-rf-sensing/src/vitals.rs:162` | physics | Feeds max Doppler velocity (m/s) observed in the current frame and updates fall status. |
