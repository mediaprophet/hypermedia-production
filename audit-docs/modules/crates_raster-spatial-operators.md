# `crates/raster-spatial-operators`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Pure Rust raster spatial operators.

Provides five high-performance, UI-agnostic spatial image processing engines:
1. `seam_carve`: Avidan-Shamir dynamic-programming energy-map seam carving.
2. `distort`: Affine, 8-DOF Perspective homography, Arc, Polar/DePolar, and Barrel lens distortion.
3. `convolve`: Separable 2D Gaussian blur, unsharp mask, Sobel/Laplacian edge detection, and general 2D kernel convolution.
4. `resample`: Filter-bank image resampling (`MitchellNetravali`, `CatmullRom`, `Lanczos3`, `Kaiser`, `Blackman`, `Bessel`).
5. `octree`: Octree palette reduction and color quantization with Floyd-Steinberg error diffusion._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Kernel2D::new` | 3 | `crates/raster-spatial-operators/src/convolve.rs:17` | - |  |
| `impl Kernel2D::with_origin` | 5 | `crates/raster-spatial-operators/src/convolve.rs:28` | - |  |
| `impl Kernel2D::normalized` | 1 | `crates/raster-spatial-operators/src/convolve.rs:46` | - | Normalize kernel weights so their sum is 1.0 (if sum is non-zero). |
| `gaussian_kernel_1d` | 2 | `crates/raster-spatial-operators/src/convolve.rs:64` | - | Create a 1D Gaussian kernel with radius and standard deviation sigma. |
| `convolve_rgba8` | 4 | `crates/raster-spatial-operators/src/convolve.rs:87` | - | General 2D convolution for RGBA8 image buffer. |
| `convolve_rgba32f` | 4 | `crates/raster-spatial-operators/src/convolve.rs:132` | - | General 2D convolution for RGBA32F image buffer. |
| `gaussian_blur_rgba8` | 5 | `crates/raster-spatial-operators/src/convolve.rs:178` | 2d | Separable 2D Gaussian blur for RGBA8 buffer. |
| `gaussian_blur_rgba32f` | 5 | `crates/raster-spatial-operators/src/convolve.rs:239` | - | Separable 2D Gaussian blur for RGBA32F buffer. |
| `unsharp_mask_rgba8` | 7 | `crates/raster-spatial-operators/src/convolve.rs:300` | - | Unsharp mask sharpening for RGBA8 buffer. |
| `sobel_edge_rgba8` | 3 | `crates/raster-spatial-operators/src/convolve.rs:330` | graph | Sobel gradient magnitude edge detection for RGBA8 buffer. |
| `laplacian_edge_rgba8` | 3 | `crates/raster-spatial-operators/src/convolve.rs:378` | graph | Laplacian 2D second-derivative edge detector for RGBA8 buffer. |
| `test_identity_convolution` | 0 | `crates/raster-spatial-operators/src/convolve.rs:418` | - |  |
| `test_gaussian_blur_conservation` | 0 | `crates/raster-spatial-operators/src/convolve.rs:428` | - |  |
| `test_sobel_edge_detection` | 0 | `crates/raster-spatial-operators/src/convolve.rs:438` | graph |  |
| `impl DistortTransform::map_inverse` | 7 | `crates/raster-spatial-operators/src/distort.rs:72` | - | Maps destination coordinate `(dst_x, dst_y)` back to source coordinate `Option<(src_x, src_y)>`. |
| `distort_rgba8` | 8 | `crates/raster-spatial-operators/src/distort.rs:169` | 2d | Distort an RGBA8 buffer. |
| `distort_rgba32f` | 8 | `crates/raster-spatial-operators/src/distort.rs:207` | 2d | Distort an RGBA32F buffer. |
| `sample_virtual_rgba8` | 6 | `crates/raster-spatial-operators/src/distort.rs:244` | - |  |
| `sample_virtual_rgba32f` | 6 | `crates/raster-spatial-operators/src/distort.rs:289` | - |  |
| `sample_rgba8` | 7 | `crates/raster-spatial-operators/src/distort.rs:334` | - |  |
| `sample_rgba32f` | 7 | `crates/raster-spatial-operators/src/distort.rs:396` | - |  |
| `test_affine_identity` | 0 | `crates/raster-spatial-operators/src/distort.rs:461` | - |  |
| `test_perspective_homography_identity` | 0 | `crates/raster-spatial-operators/src/distort.rs:484` | - |  |
| `test_polar_depolar_roundtrip` | 0 | `crates/raster-spatial-operators/src/distort.rs:507` | - |  |
| `test_barrel_distortion` | 0 | `crates/raster-spatial-operators/src/distort.rs:530` | - |  |
| `test_all_operators_pipeline_smoke` | 0 | `crates/raster-spatial-operators/src/lib.rs:34` | - |  |
| `impl OctreeNode::new` | 0 | `crates/raster-spatial-operators/src/octree.rs:22` | - |  |
| `impl OctreeNode::is_leaf` | 1 | `crates/raster-spatial-operators/src/octree.rs:34` | - |  |
| `impl OctreeNode::insert` | 6 | `crates/raster-spatial-operators/src/octree.rs:38` | - |  |
| `impl OctreeQuantizer::new` | 0 | `crates/raster-spatial-operators/src/octree.rs:72` | - |  |
| `impl OctreeQuantizer::insert` | 5 | `crates/raster-spatial-operators/src/octree.rs:79` | - |  |
| `impl OctreeQuantizer::build_palette` | 2 | `crates/raster-spatial-operators/src/octree.rs:84` | - | Reduce octree leaves to `max_colors`. |
| `count_leaves` | 2 | `crates/raster-spatial-operators/src/octree.rs:111` | graph |  |
| `find_min_reducible` | 2 | `crates/raster-spatial-operators/src/octree.rs:121` | graph |  |
| `reduce_node` | 2 | `crates/raster-spatial-operators/src/octree.rs:141` | graph |  |
| `collect_palette` | 2 | `crates/raster-spatial-operators/src/octree.rs:172` | graph |  |
| `nearest_palette_color` | 2 | `crates/raster-spatial-operators/src/octree.rs:190` | - |  |
| `octree_quantize_rgba8` | 5 | `crates/raster-spatial-operators/src/octree.rs:211` | - | Quantize RGBA8 image to at most `max_colors` using Octree reduction with optional Floyd-Steinberg dithering. |
| `octree_quantize_rgba32f` | 5 | `crates/raster-spatial-operators/src/octree.rs:292` | - | Quantize RGBA32F image to at most `max_colors` using Octree reduction. |
| `test_octree_quantize_palette_budget` | 0 | `crates/raster-spatial-operators/src/octree.rs:334` | - |  |
| `test_octree_quantize_with_dither` | 0 | `crates/raster-spatial-operators/src/octree.rs:353` | - |  |
| `impl ResampleFilter::support` | 1 | `crates/raster-spatial-operators/src/resample.rs:25` | - | Spatial support radius in pixels. |
| `impl ResampleFilter::weight` | 2 | `crates/raster-spatial-operators/src/resample.rs:36` | - | Continuous weighting function at distance `x`. |
| `sinc` | 1 | `crates/raster-spatial-operators/src/resample.rs:73` | - |  |
| `cubic_bc` | 3 | `crates/raster-spatial-operators/src/resample.rs:82` | - |  |
| `bessel_i0` | 1 | `crates/raster-spatial-operators/src/resample.rs:102` | - | Zeroth-order modified Bessel function of the first kind I_0(x). |
| `bessel_j1` | 1 | `crates/raster-spatial-operators/src/resample.rs:117` | - | First-order Bessel function of the first kind J_1(x). |
| `contributions` | 3 | `crates/raster-spatial-operators/src/resample.rs:138` | - |  |
| `resample_rgba8` | 6 | `crates/raster-spatial-operators/src/resample.rs:176` | - | Resample an RGBA8 buffer to new target dimensions. |
| `resample_rgba32f` | 6 | `crates/raster-spatial-operators/src/resample.rs:252` | - | Resample an RGBA32F buffer to new target dimensions. |
| `fit_size` | 4 | `crates/raster-spatial-operators/src/resample.rs:328` | computational geometry | Compute target dimensions to fit aspect ratio inside bounding box. |
| `test_resample_uniform_conservation` | 0 | `crates/raster-spatial-operators/src/resample.rs:345` | - |  |
| `test_fit_size` | 0 | `crates/raster-spatial-operators/src/resample.rs:371` | - |  |
| `seam_carve_rgba8` | 5 | `crates/raster-spatial-operators/src/seam_carve.rs:13` | - | Carve seams from an RGBA8 buffer to achieve target dimensions. |
| `seam_carve_rgba32f` | 5 | `crates/raster-spatial-operators/src/seam_carve.rs:50` | 2d | Carve seams from an RGBA32F buffer (`[f32]`, 4 values per pixel). |
| `carve_one_vertical_seam_rgba8` | 3 | `crates/raster-spatial-operators/src/seam_carve.rs:84` | - |  |
| `carve_one_horizontal_seam_rgba8` | 3 | `crates/raster-spatial-operators/src/seam_carve.rs:176` | - |  |
| `carve_one_vertical_seam_rgba32f` | 3 | `crates/raster-spatial-operators/src/seam_carve.rs:203` | - |  |
| `carve_one_horizontal_seam_rgba32f` | 3 | `crates/raster-spatial-operators/src/seam_carve.rs:287` | - |  |
| `test_seam_carve_flat_region_dropped_first` | 0 | `crates/raster-spatial-operators/src/seam_carve.rs:318` | - |  |
| `test_seam_carve_identity` | 0 | `crates/raster-spatial-operators/src/seam_carve.rs:350` | - |  |
| `test_seam_carve_both_axes` | 0 | `crates/raster-spatial-operators/src/seam_carve.rs:361` | - |  |
