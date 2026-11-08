//! `radiance-color-grading` — Categorical film color science, color space transforms, and real-time scopes.
//!
//! Provides:
//! - Standard color space transforms: Rec.709, Rec.2020, ACEScc, ACEScct, ACEScg, P3-D65
//! - ASC CDL v1.2 (Slope / Offset / Power / Saturation) forward, inverse, and XML parsing
//! - 3-way color wheels (Lift / Gamma / Gain / Offset / Balance)
//! - `.cube` and `.3dl` 3D LUT parser, generator, and tetrahedral/trilinear interpolation
//! - Camera log curves: Sony S-Log3, Panasonic V-Log, Canon Log 2/3, ARRI LogC3/4, Apple Log, DJI D-Log
//! - Tone mapping operators: Reinhard, ACES fitted, Filmic, ITU-R BT.2446a
//! - Real-time video scopes: Waveform monitor, Vectorscope (with skin-tone line and 75%/100% targets), RGB Parade, Histograms
//!
//! Grounded in international human rights instruments protecting creative expression, moral rights,
//! cultural heritage, and informatics fiduciary integrity (UDHR Articles 12, 19, 27; ICCPR Articles 17, 19;
//! ICESCR Article 15).

#![forbid(unsafe_code)]

pub mod grade;
pub mod log;
pub mod lut;
pub mod matrix;
pub mod scopes;
pub mod spaces;
pub mod transform;

pub use grade::{AscCdl, ColorWheels, DEFAULT_HDR_WHITE_NITS, GradeSpace};
pub use log::{Domain as LogDomain, LogCurve};
pub use lut::{Lut, Lut1d, Lut3d, LutFormat};
pub use matrix::{
    ColorInfo, HdrMetadata, Matrix, Primaries, Range, Transfer, denormalize_c, denormalize_y,
    from_linear, hlg_inverse_oetf, hlg_oetf, hsl_to_rgb, linear_to_srgb, linear_to_srgb_u8, luma709,
    normalize_c, normalize_y, parse_hex, pq_eotf, pq_inverse_eotf, rgb_to_hsl, rgb_to_ycbcr,
    srgb_to_linear, to_hex, to_linear, ycbcr_to_rgb,
};
pub use spaces::{
    ColorPipeline, ColorSpace, Curve, Gamut, WorkingSpace, apply3, gamut_matrix,
};
pub use transform::{DecodeTable, InputTransform, OutputTransform, ToneMap};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ycbcr_roundtrip() {
        for m in [Matrix::Bt601, Matrix::Bt709, Matrix::Bt2020Ncl] {
            let rgb = [0.8f32, 0.3, 0.1];
            let ycbcr = rgb_to_ycbcr(rgb[0], rgb[1], rgb[2], m);
            let back = ycbcr_to_rgb(ycbcr[0], ycbcr[1], ycbcr[2], m);
            for k in 0..3 {
                assert!((rgb[k] - back[k]).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn test_transfer_roundtrips() {
        for t in [Transfer::Srgb, Transfer::Pq, Transfer::Hlg, Transfer::Linear] {
            for v in [0.0f32, 0.1, 0.5, 0.9] {
                let l = to_linear(v, t);
                let back = from_linear(l, t);
                assert!((back - v).abs() < 1e-3, "{t:?} {v} -> {back}");
            }
        }
    }

    #[test]
    fn test_asc_cdl_apply_and_invert() {
        let cdl = AscCdl {
            slope: [1.1, 1.0, 0.9],
            offset: [0.02, 0.0, -0.01],
            power: [0.95, 1.0, 1.05],
            sat: 1.2,
        };
        let c = [0.4, 0.5, 0.6];
        let graded = cdl.apply(c, true);
        let back = cdl.invert(graded, true);
        for k in 0..3 {
            assert!((c[k] - back[k]).abs() < 1e-4);
        }
    }

    #[test]
    fn test_color_wheels_apply() {
        let mut wheels = ColorWheels::default();
        wheels.lift = [0.05, 0.0, 0.0];
        wheels.gain = [1.2, 1.0, 0.8];
        wheels.contrast = 1.1;
        let c = [0.2f32, 0.5, 0.8];
        let graded = wheels.apply(c);
        assert!(graded[0] > c[0]); // Lift + gain on red
        assert!(graded[2] < c[2]); // Gain on blue decreased
    }

    #[test]
    fn test_lut_cube_tetrahedral() {
        let lut = Lut3d::from_fn(4, |[r, g, b]| [r * 1.1, g * 0.9, b + 0.05]);
        let sample = [0.5f32, 0.5, 0.5];
        let out_tetra = lut.apply(sample);
        let out_tri = lut.apply_trilinear(sample);
        for k in 0..3 {
            assert!((out_tetra[k] - out_tri[k]).abs() < 1e-2);
        }
    }

    #[test]
    fn test_log_curves_encode_decode() {
        for curve in LogCurve::ALL {
            let x = 0.18; // 18% grey
            let encoded = curve.encode(x);
            let decoded = curve.decode(encoded);
            assert!((x - decoded).abs() < 1e-3, "LogCurve {curve:?} roundtrip failed");
        }
    }

    #[test]
    fn test_spaces_gamut_matrix() {
        let m = gamut_matrix(Gamut::Bt709, Gamut::Bt2020);
        let red_709 = [1.0f32, 0.0, 0.0];
        let red_2020 = apply3(&spaces::to_f32(&m), red_709);
        // Red in 709 converted to 2020 should have positive R and slight G/B
        assert!(red_2020[0] > 0.6);
    }
}
