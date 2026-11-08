//! Pure Rust raster spatial operators.
//!
//! Provides five high-performance, UI-agnostic spatial image processing engines:
//! 1. `seam_carve`: Avidan-Shamir dynamic-programming energy-map seam carving.
//! 2. `distort`: Affine, 8-DOF Perspective homography, Arc, Polar/DePolar, and Barrel lens distortion.
//! 3. `convolve`: Separable 2D Gaussian blur, unsharp mask, Sobel/Laplacian edge detection, and general 2D kernel convolution.
//! 4. `resample`: Filter-bank image resampling (`MitchellNetravali`, `CatmullRom`, `Lanczos3`, `Kaiser`, `Blackman`, `Bessel`).
//! 5. `octree`: Octree palette reduction and color quantization with Floyd-Steinberg error diffusion.

#![forbid(unsafe_code)]

pub mod convolve;
pub mod distort;
pub mod octree;
pub mod resample;
pub mod seam_carve;

pub use convolve::{
    Kernel2D, convolve_rgba8, convolve_rgba32f, gaussian_blur_rgba8, gaussian_blur_rgba32f,
    gaussian_kernel_1d, laplacian_edge_rgba8, sobel_edge_rgba8, unsharp_mask_rgba8,
};
pub use distort::{
    DistortTransform, Interpolation, VirtualPixelMode, distort_rgba8, distort_rgba32f,
};
pub use octree::{OctreeQuantizer, octree_quantize_rgba8, octree_quantize_rgba32f};
pub use resample::{ResampleFilter, fit_size, resample_rgba8, resample_rgba32f};
pub use seam_carve::{seam_carve_rgba8, seam_carve_rgba32f};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_operators_pipeline_smoke() {
        let w = 16;
        let h = 16;
        let mut img = vec![128u8; w * h * 4];
        // Draw crosshair pattern
        for i in 0..16 {
            let h_idx = (8 * w + i) * 4;
            let v_idx = (i * w + 8) * 4;
            img[h_idx] = 255;
            img[h_idx + 1] = 0;
            img[h_idx + 2] = 0;
            img[h_idx + 3] = 255;

            img[v_idx] = 0;
            img[v_idx + 1] = 255;
            img[v_idx + 2] = 0;
            img[v_idx + 3] = 255;
        }

        // 1. Resample down to 8x8 with Mitchell-Netravali
        let resized = resample_rgba8(&img, w, h, 8, 8, ResampleFilter::MitchellNetravali);
        assert_eq!(resized.len(), 8 * 8 * 4);

        // 2. Convolve with separable Gaussian blur
        let blurred = gaussian_blur_rgba8(&resized, 8, 8, 1.0, 0.8);
        assert_eq!(blurred.len(), 8 * 8 * 4);

        // 3. Distort with affine transformation
        let affine = DistortTransform::Affine([1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        let distorted = distort_rgba8(
            &blurred,
            8,
            8,
            &affine,
            8,
            8,
            VirtualPixelMode::Edge,
            Interpolation::Bilinear,
        );
        assert_eq!(distorted.len(), 8 * 8 * 4);

        // 4. Seam carve 8x8 down to 6x6
        let (carved, cw, ch) = seam_carve_rgba8(&distorted, 8, 8, 6, 6);
        assert_eq!(cw, 6);
        assert_eq!(ch, 6);
        assert_eq!(carved.len(), 6 * 6 * 4);

        // 5. Octree quantize to 4 colors with Floyd-Steinberg dithering
        let (quantized, palette) = octree_quantize_rgba8(&carved, cw, ch, 4, true);
        assert!(!palette.is_empty());
        assert!(palette.len() <= 4);
        assert_eq!(quantized.len(), 6 * 6 * 4);
    }
}
