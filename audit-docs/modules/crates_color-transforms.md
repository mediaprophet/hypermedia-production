# `crates/color-transforms`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Pure-Rust ICC v2/v4 color management module.

Provides ICC profile parsing & serialization, PCS XYZ/Lab D50 conversion,
4 rendering intents, Black Point Compensation, tetrahedral CLUT interpolation,
built-in standard profiles, and out-of-gamut detection._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl GradeSpace::new` | 2 | `crates/color-transforms/src/grade.rs:41` | - | The grading space of a working space, with HDR White `white_nits` (ignored for SDR). |
| `impl GradeSpace::is_hdr` | 1 | `crates/color-transforms/src/grade.rs:50` | - |  |
| `impl GradeSpace::white_nits` | 1 | `crates/color-transforms/src/grade.rs:55` | - | HDR White in cd/m² (None for SDR). |
| `impl GradeSpace::encode` | 2 | `crates/color-transforms/src/grade.rs:64` | numerics | Working linear → grading signal (1 = white; HDR values above HDR White are > 1). |
| `impl GradeSpace::decode` | 2 | `crates/color-transforms/src/grade.rs:85` | numerics | Grading signal → working linear. SDR clamps to 0…1 (white); HDR keeps values above HDR |
| `impl GradeSpace::signal_of_nits` | 2 | `crates/color-transforms/src/grade.rs:105` | - | The signal of a grey of `nits` cd/m² (HDR) — what a nit-scaled slider position means. |
| `impl GradeSpace::nits_of_signal` | 2 | `crates/color-transforms/src/grade.rs:111` | - | The cd/m² of a grey of signal `s` (HDR). |
| `luma2020` | 0 | `crates/color-transforms/src/grade.rs:116` | - |  |
| `grey` | 1 | `crates/color-transforms/src/grade.rs:125` | - |  |
| `sdr_is_the_srgb_signal` | 0 | `crates/color-transforms/src/grade.rs:130` | - |  |
| `pq_known_nit_values` | 0 | `crates/color-transforms/src/grade.rs:140` | - |  |
| `hlg_known_nit_values` | 0 | `crates/color-transforms/src/grade.rs:161` | - |  |
| `impl Builtin::id` | 1 | `crates/color-transforms/src/icc/builtin.rs:47` | - |  |
| `impl Builtin::description` | 1 | `crates/color-transforms/src/icc/builtin.rs:62` | - |  |
| `impl Builtin::profile` | 1 | `crates/color-transforms/src/icc/builtin.rs:77` | - |  |
| `srgb` | 0 | `crates/color-transforms/src/icc/builtin.rs:83` | - |  |
| `display_p3` | 0 | `crates/color-transforms/src/icc/builtin.rs:87` | - |  |
| `adobe_rgb` | 0 | `crates/color-transforms/src/icc/builtin.rs:91` | - |  |
| `prophoto_rgb` | 0 | `crates/color-transforms/src/icc/builtin.rs:95` | - |  |
| `rec2020` | 0 | `crates/color-transforms/src/icc/builtin.rs:99` | - |  |
| `generic_cmyk` | 0 | `crates/color-transforms/src/icc/builtin.rs:103` | - |  |
| `d65` | 0 | `crates/color-transforms/src/icc/builtin.rs:107` | - |  |
| `colorants` | 4 | `crates/color-transforms/src/icc/builtin.rs:111` | - |  |
| `rgb` | 4 | `crates/color-transforms/src/icc/builtin.rs:141` | - |  |
| `gray` | 2 | `crates/color-transforms/src/icc/builtin.rs:159` | - |  |
| `lab` | 0 | `crates/color-transforms/src/icc/builtin.rs:176` | - |  |
| `build` | 1 | `crates/color-transforms/src/icc/builtin.rs:199` | - |  |
| `impl Clut::new` | 3 | `crates/color-transforms/src/icc/clut.rs:23` | graph | Builds a table from node data. |
| `impl Clut::nodes` | 1 | `crates/color-transforms/src/icc/clut.rs:37` | - | Number of nodes in the grid. |
| `impl Clut::sample` | 3 | `crates/color-transforms/src/icc/clut.rs:42` | graph | Builds a table by sampling `f` at every node (inputs normalised to `[0, 1]`). |
| `impl Clut::locate` | 3 | `crates/color-transforms/src/icc/clut.rs:63` | - |  |
| `impl Clut::eval` | 3 | `crates/color-transforms/src/icc/clut.rs:75` | - | Interpolates at `input` into `out`. |
| `impl Clut::eval1` | 3 | `crates/color-transforms/src/icc/clut.rs:106` | - |  |
| `impl Clut::tetra` | 5 | `crates/color-transforms/src/icc/clut.rs:117` | - | Tetrahedral interpolation inside one cube of three dimensions. |
| `impl Clut::eval_trilinear` | 3 | `crates/color-transforms/src/icc/clut.rs:155` | - | Trilinear interpolation (3 inputs). |
| `impl Clut::eval_multilinear` | 3 | `crates/color-transforms/src/icc/clut.rs:161` | - | Multilinear interpolation over all 2^n cube corners. |
| `srgb_trc` | 0 | `crates/color-transforms/src/icc/curve.rs:23` | - | The sRGB transfer function (IEC 61966-2.1) as ICC parametric type 3. |
| `impl Curve::param_count` | 1 | `crates/color-transforms/src/icc/curve.rs:31` | - |  |
| `impl Curve::is_identity` | 1 | `crates/color-transforms/src/icc/curve.rs:41` | - |  |
| `impl Curve::eval64` | 2 | `crates/color-transforms/src/icc/curve.rs:57` | - | Evaluates the curve at `x` (f64 reference path). |
| `impl Curve::eval_inverse64` | 2 | `crates/color-transforms/src/icc/curve.rs:67` | - | Evaluates the inverse curve at `y`. |
| `impl Curve::eval32` | 2 | `crates/color-transforms/src/icc/curve.rs:84` | - | Evaluates the curve at `x` (f32). |
| `mirror` | 2 | `crates/color-transforms/src/icc/curve.rs:90` | - |  |
| `para` | 3 | `crates/color-transforms/src/icc/curve.rs:94` | - |  |
| `para_inverse` | 3 | `crates/color-transforms/src/icc/curve.rs:106` | - |  |
| `table` | 2 | `crates/color-transforms/src/icc/curve.rs:137` | - |  |
| `invert_table` | 2 | `crates/color-transforms/src/icc/curve.rs:151` | - |  |
| `impl GamutCheck::new` | 3 | `crates/color-transforms/src/icc/gamut.rs:19` | - |  |
| `impl GamutCheck::inputs` | 1 | `crates/color-transforms/src/icc/gamut.rs:53` | - |  |
| `impl GamutCheck::delta_e` | 2 | `crates/color-transforms/src/icc/gamut.rs:57` | - |  |
| `impl GamutCheck::out_of_gamut` | 2 | `crates/color-transforms/src/icc/gamut.rs:81` | - |  |
| `impl GamutCheck::generate_mask` | 4 | `crates/color-transforms/src/icc/gamut.rs:86` | - | Evaluates a mask buffer: writes 255 for out-of-gamut pixels, 0 for in-gamut pixels. |
| `mul` | 2 | `crates/color-transforms/src/icc/math.rs:18` | - |  |
| `apply` | 2 | `crates/color-transforms/src/icc/math.rs:28` | - |  |
| `det` | 1 | `crates/color-transforms/src/icc/math.rs:36` | - |  |
| `invert` | 1 | `crates/color-transforms/src/icc/math.rs:43` | - | Inverse, or `None` when (nearly) singular. |
| `s15f16_round` | 1 | `crates/color-transforms/src/icc/math.rs:69` | - | Round to 16-bit fractional fixed point. |
| `d50_quantized` | 0 | `crates/color-transforms/src/icc/math.rs:74` | - | Exact quantized D50 in ICC header fixed-point representation. |
| `xy_to_xyz` | 1 | `crates/color-transforms/src/icc/math.rs:79` | - | XYZ (Y = 1) of a chromaticity `(x, y)`. |
| `bradford` | 2 | `crates/color-transforms/src/icc/math.rs:92` | numerics | Bradford chromatic adaptation matrix from white `src` to white `dst` (both XYZ). |
| `rgb_to_xyz_matrix` | 4 | `crates/color-transforms/src/icc/math.rs:105` | numerics | RGB → XYZ matrix (relative to `white`, Y = 1) for primaries given as chromaticities. |
| `lab_f` | 1 | `crates/color-transforms/src/icc/math.rs:124` | - |  |
| `lab_finv` | 1 | `crates/color-transforms/src/icc/math.rs:129` | - |  |
| `xyz_to_lab` | 2 | `crates/color-transforms/src/icc/math.rs:135` | - | CIE XYZ → CIE L*a*b* under white point `white`. |
| `lab_to_xyz` | 2 | `crates/color-transforms/src/icc/math.rs:143` | - | CIE L*a*b* → CIE XYZ under white point `white`. |
| `delta_e76` | 2 | `crates/color-transforms/src/icc/math.rs:152` | - | CIE ΔE*ab (1976) Euclidean color difference. |
| `impl Stage::outputs` | 2 | `crates/color-transforms/src/icc/pipeline.rs:36` | - |  |
| `impl Stage::matrix3` | 1 | `crates/color-transforms/src/icc/pipeline.rs:44` | - |  |
| `impl Stage::eval` | 3 | `crates/color-transforms/src/icc/pipeline.rs:53` | - |  |
| `impl Pipeline::new` | 2 | `crates/color-transforms/src/icc/pipeline.rs:137` | - |  |
| `impl Pipeline::eval` | 3 | `crates/color-transforms/src/icc/pipeline.rs:145` | - |  |
| `impl Pipeline::optimize` | 1 | `crates/color-transforms/src/icc/pipeline.rs:170` | - |  |
| `impl std::fmt::Display for CmsError::fmt` | 2 | `crates/color-transforms/src/icc/profile.rs:18` | - |  |
| `impl Intent::from_u32` | 1 | `crates/color-transforms/src/icc/profile.rs:47` | - |  |
| `impl ProfileClass::from_sig` | 1 | `crates/color-transforms/src/icc/profile.rs:71` | - |  |
| `impl ProfileClass::sig` | 1 | `crates/color-transforms/src/icc/profile.rs:83` | - |  |
| `impl ColorSpace::from_sig` | 1 | `crates/color-transforms/src/icc/profile.rs:115` | - |  |
| `impl ColorSpace::sig` | 1 | `crates/color-transforms/src/icc/profile.rs:138` | - |  |
| `impl ColorSpace::channels` | 1 | `crates/color-transforms/src/icc/profile.rs:156` | - |  |
| `impl Profile::channels` | 1 | `crates/color-transforms/src/icc/profile.rs:205` | - |  |
| `impl Profile::media_white` | 1 | `crates/color-transforms/src/icc/profile.rs:209` | - |  |
| `impl Profile::lut_for` | 2 | `crates/color-transforms/src/icc/profile.rs:217` | - |  |
| `impl Profile::device_to_pcs` | 2 | `crates/color-transforms/src/icc/profile.rs:226` | - |  |
| `impl Profile::pcs_to_device` | 2 | `crates/color-transforms/src/icc/profile.rs:260` | - |  |
| `impl Profile::parse` | 1 | `crates/color-transforms/src/icc/profile.rs:294` | - |  |
| `impl Reader<'a>::slice` | 3 | `crates/color-transforms/src/icc/profile.rs:305` | - |  |
| `impl Reader<'a>::u8` | 2 | `crates/color-transforms/src/icc/profile.rs:308` | - |  |
| `impl Reader<'a>::u16` | 2 | `crates/color-transforms/src/icc/profile.rs:311` | - |  |
| `impl Reader<'a>::u32` | 2 | `crates/color-transforms/src/icc/profile.rs:315` | - |  |
| `impl Reader<'a>::s15f16` | 2 | `crates/color-transforms/src/icc/profile.rs:319` | - |  |
| `impl Reader<'a>::xyz` | 2 | `crates/color-transforms/src/icc/profile.rs:322` | - |  |
| `parse_profile` | 1 | `crates/color-transforms/src/icc/profile.rs:327` | - |  |
| `parse_desc` | 1 | `crates/color-transforms/src/icc/profile.rs:421` | - |  |
| `parse_xyz` | 1 | `crates/color-transforms/src/icc/profile.rs:453` | - |  |
| `parse_curve` | 1 | `crates/color-transforms/src/icc/profile.rs:461` | - |  |
| `parse_lut` | 2 | `crates/color-transforms/src/icc/profile.rs:508` | - |  |
| `impl Default for CmykParams::default` | 0 | `crates/color-transforms/src/icc/synth.rs:27` | - |  |
| `coated_cmyk` | 0 | `crates/color-transforms/src/icc/synth.rs:41` | - |  |
| `synth_cmyk` | 1 | `crates/color-transforms/src/icc/synth.rs:45` | - |  |
| `impl Default for TransformOptions::default` | 0 | `crates/color-transforms/src/icc/transform.rs:20` | - |  |
| `impl Transform::new` | 4 | `crates/color-transforms/src/icc/transform.rs:49` | - |  |
| `impl Transform::with_options` | 3 | `crates/color-transforms/src/icc/transform.rs:53` | - |  |
| `impl Transform::inputs` | 1 | `crates/color-transforms/src/icc/transform.rs:82` | - |  |
| `impl Transform::outputs` | 1 | `crates/color-transforms/src/icc/transform.rs:86` | - |  |
| `impl Transform::options` | 1 | `crates/color-transforms/src/icc/transform.rs:90` | - |  |
| `impl Transform::pipeline` | 1 | `crates/color-transforms/src/icc/transform.rs:94` | - |  |
| `impl Transform::eval` | 3 | `crates/color-transforms/src/icc/transform.rs:99` | - |  |
| `impl Transform::eval_fast` | 3 | `crates/color-transforms/src/icc/transform.rs:104` | - |  |
| `impl Transform::convert_f32` | 6 | `crates/color-transforms/src/icc/transform.rs:112` | - | Converts interleaved float pixels. |
| `impl Transform::convert_u8` | 6 | `crates/color-transforms/src/icc/transform.rs:134` | - | Converts interleaved 8-bit pixels. |
| `link_stages` | 4 | `crates/color-transforms/src/icc/transform.rs:160` | - |  |
| `connection` | 4 | `crates/color-transforms/src/icc/transform.rs:197` | - |  |
| `black_point` | 2 | `crates/color-transforms/src/icc/transform.rs:230` | - |  |
| `s15` | 1 | `crates/color-transforms/src/icc/write.rs:7` | - |  |
| `xyz_type` | 1 | `crates/color-transforms/src/icc/write.rs:11` | - |  |
| `mluc` | 1 | `crates/color-transforms/src/icc/write.rs:19` | - |  |
| `curve_elem` | 1 | `crates/color-transforms/src/icc/write.rs:33` | - |  |
| `write_profile` | 1 | `crates/color-transforms/src/icc/write.rs:68` | - |  |
| `impl Matrix::kr_kb` | 1 | `crates/color-transforms/src/lib.rs:39` | - | (Kr, Kb) luma coefficients. |
| `impl Matrix::from_code` | 1 | `crates/color-transforms/src/lib.rs:47` | numerics | From an ISO/IEC 23091-2 `matrix_coefficients` code. |
| `impl Transfer::from_code` | 1 | `crates/color-transforms/src/lib.rs:72` | - |  |
| `impl HdrMetadata::peak_nits` | 1 | `crates/color-transforms/src/lib.rs:127` | 2d | The content's peak for tone mapping: MaxCLL when signalled (the brightest pixel actually |
| `ycbcr_to_rgb` | 4 | `crates/color-transforms/src/lib.rs:145` | numerics | Convert normalised Y'CbCr (Y in 0..1, Cb/Cr in -0.5..0.5) to non-linear R'G'B'. |
| `rgb_to_ycbcr` | 4 | `crates/color-transforms/src/lib.rs:156` | numerics | Convert non-linear R'G'B' to normalised Y'CbCr. |
| `normalize_y` | 3 | `crates/color-transforms/src/lib.rs:165` | - | Normalise an integer code value to (Y 0..1, C -0.5..0.5) given bit depth and range. |
| `normalize_c` | 3 | `crates/color-transforms/src/lib.rs:173` | - |  |
| `srgb_to_linear` | 1 | `crates/color-transforms/src/lib.rs:183` | numerics | sRGB electro-optical transfer (encoded â†’ linear). |
| `linear_to_srgb` | 1 | `crates/color-transforms/src/lib.rs:187` | numerics |  |
| `to_linear` | 2 | `crates/color-transforms/src/lib.rs:192` | numerics | Encoded â†’ linear for a transfer function (display-referred; PQ normalised to 100 nits = 1.0). |
| `from_linear` | 2 | `crates/color-transforms/src/lib.rs:204` | numerics |  |
| `pq_eotf` | 1 | `crates/color-transforms/src/lib.rs:214` | - | SMPTE ST 2084 EOTF, output normalised to 10 000 nits = 1.0. |
| `pq_inverse_eotf` | 1 | `crates/color-transforms/src/lib.rs:220` | - |  |
| `hlg_oetf` | 1 | `crates/color-transforms/src/lib.rs:226` | - |  |
| `hlg_inverse_oetf` | 1 | `crates/color-transforms/src/lib.rs:230` | - |  |
| `srgb_u8_to_linear_table` | 0 | `crates/color-transforms/src/lib.rs:236` | numerics | 256-entry table: sRGB-encoded u8 â†’ linear f32. |
| `linear_to_srgb_u8_table` | 0 | `crates/color-transforms/src/lib.rs:242` | numerics | 4096-entry table: linear (0..1, quantised to 12 bits) â†’ sRGB-encoded u8. |
| `linear_to_srgb_u8` | 1 | `crates/color-transforms/src/lib.rs:248` | numerics |  |
| `luma709` | 3 | `crates/color-transforms/src/lib.rs:255` | semantics, numerics | Rec.709 luma of a linear or encoded RGB triple. |
| `rgb_to_hsl` | 3 | `crates/color-transforms/src/lib.rs:260` | - | RGB (0..1) â†’ HSL (h in 0..1). |
| `hsl_to_rgb` | 3 | `crates/color-transforms/src/lib.rs:279` | - |  |
| `parse_hex` | 1 | `crates/color-transforms/src/lib.rs:301` | - | Parse `#rrggbb` / `#rrggbbaa` into 0..1 floats. |
| `to_hex` | 1 | `crates/color-transforms/src/lib.rs:311` | - |  |
| `ycbcr_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:325` | - |  |
| `limited_range_levels` | 0 | `crates/color-transforms/src/lib.rs:337` | - |  |
| `transfer_roundtrips` | 0 | `crates/color-transforms/src/lib.rs:345` | - |  |
| `hsl_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:358` | - |  |
| `test_icc_transform_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:369` | - |  |
| `test_tetrahedral_clut_interpolation` | 0 | `crates/color-transforms/src/lib.rs:392` | - |  |
| `test_gamut_warning` | 0 | `crates/color-transforms/src/lib.rs:407` | - |  |
| `test_aco_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:418` | - |  |
| `test_ase_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:438` | - |  |
| `test_gpl_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:465` | - |  |
| `hex` | 0 | `crates/color-transforms/src/lib.rs:487` | - |  |
| `impl LogCurve::domain` | 1 | `crates/color-transforms/src/log.rs:51` | - |  |
| `impl LogCurve::label` | 1 | `crates/color-transforms/src/log.rs:58` | - |  |
| `impl LogCurve::encode` | 2 | `crates/color-transforms/src/log.rs:72` | numerics | Scene-linear → signal (curve domain). |
| `impl LogCurve::decode` | 2 | `crates/color-transforms/src/log.rs:86` | numerics | Signal (curve domain) → scene-linear. |
| `impl LogCurve::peak_linear` | 1 | `crates/color-transforms/src/log.rs:101` | numerics | Scene-linear value of the maximum signal (code 1023), used as the source peak for tone |
| `signal_from_normalized` | 3 | `crates/color-transforms/src/log.rs:111` | - | Decoder-normalised value (0..1 in the file's range) → curve-domain signal. |
| `normalized_from_signal` | 3 | `crates/color-transforms/src/log.rs:120` | - | Curve-domain signal → decoder-normalised value (0..1 in the file's range). |
| `slog3_encode` | 1 | `crates/color-transforms/src/log.rs:131` | - |  |
| `slog3_decode` | 1 | `crates/color-transforms/src/log.rs:134` | - |  |
| `vlog_encode` | 1 | `crates/color-transforms/src/log.rs:149` | - |  |
| `vlog_decode` | 1 | `crates/color-transforms/src/log.rs:152` | - |  |
| `clog2_encode` | 1 | `crates/color-transforms/src/log.rs:160` | - |  |
| `clog2_decode` | 1 | `crates/color-transforms/src/log.rs:164` | - |  |
| `clog3_encode` | 1 | `crates/color-transforms/src/log.rs:173` | - |  |
| `clog3_decode` | 1 | `crates/color-transforms/src/log.rs:183` | - |  |
| `logc3_encode` | 1 | `crates/color-transforms/src/log.rs:199` | - |  |
| `logc3_decode` | 1 | `crates/color-transforms/src/log.rs:203` | - |  |
| `logc4_consts` | 0 | `crates/color-transforms/src/log.rs:211` | - |  |
| `logc4_encode` | 1 | `crates/color-transforms/src/log.rs:219` | - |  |
| `logc4_decode` | 1 | `crates/color-transforms/src/log.rs:223` | - |  |
| `apple_log_encode` | 1 | `crates/color-transforms/src/log.rs:233` | - |  |
| `apple_log_decode` | 1 | `crates/color-transforms/src/log.rs:243` | - |  |
| `dlog_encode` | 1 | `crates/color-transforms/src/log.rs:258` | - |  |
| `dlog_decode` | 1 | `crates/color-transforms/src/log.rs:261` | - |  |
| `roundtrip_every_curve` | 0 | `crates/color-transforms/src/log.rs:270` | - |  |
| `published_reference_values` | 0 | `crates/color-transforms/src/log.rs:291` | - | Reference points from the published specifications (code values are 10-bit). |
| `domain_conversion_roundtrip` | 0 | `crates/color-transforms/src/log.rs:323` | - |  |
| `impl Lut1d::identity` | 1 | `crates/color-transforms/src/lut.rs:27` | - |  |
| `impl Lut1d::size` | 1 | `crates/color-transforms/src/lut.rs:31` | - |  |
| `impl Lut1d::apply` | 2 | `crates/color-transforms/src/lut.rs:35` | - |  |
| `impl Lut3d::identity` | 1 | `crates/color-transforms/src/lut.rs:59` | - |  |
| `impl Lut3d::from_fn` | 2 | `crates/color-transforms/src/lut.rs:64` | - | Sample a colour transform on a `size³` grid over 0..1. |
| `impl Lut3d::at` | 4 | `crates/color-transforms/src/lut.rs:78` | - |  |
| `impl Lut3d::parse_cube` | 1 | `crates/color-transforms/src/lut.rs:84` | - | Parse a `.cube` file that contains a 3D table (a shaper, if present, is rejected; use |
| `impl Lut3d::to_cube` | 1 | `crates/color-transforms/src/lut.rs:93` | - |  |
| `impl Lut3d::coord` | 3 | `crates/color-transforms/src/lut.rs:99` | - | Normalised grid coordinate (0..size-1) of an input value. |
| `impl Lut3d::apply` | 2 | `crates/color-transforms/src/lut.rs:104` | - | Tetrahedral interpolation. |
| `impl Lut3d::apply_trilinear` | 2 | `crates/color-transforms/src/lut.rs:136` | - | Trilinear interpolation (kept for comparison; LUTs render tetrahedrally). |
| `impl LutFormat::from_path` | 1 | `crates/color-transforms/src/lut.rs:172` | - |  |
| `impl LutFormat::extension` | 1 | `crates/color-transforms/src/lut.rs:180` | - |  |
| `floats` | 1 | `crates/color-transforms/src/lut.rs:188` | - |  |
| `impl Lut::from_cube` | 1 | `crates/color-transforms/src/lut.rs:193` | - |  |
| `impl Lut::parse` | 2 | `crates/color-transforms/src/lut.rs:198` | - | Parse by format (sniffs `LUT_*_SIZE` for `.cube` when the format is unknown). |
| `impl Lut::parse_cube` | 1 | `crates/color-transforms/src/lut.rs:207` | - |  |
| `impl Lut::to_cube` | 1 | `crates/color-transforms/src/lut.rs:299` | - |  |
| `impl Lut::parse_3dl` | 1 | `crates/color-transforms/src/lut.rs:332` | - | Parse an Autodesk `.3dl`. |
| `impl Lut::to_3dl` | 1 | `crates/color-transforms/src/lut.rs:407` | computational geometry | Write a `.3dl` (10-bit input mesh, 12-bit output). Shapers are baked into the cube. |
| `impl Lut::baked_cube` | 2 | `crates/color-transforms/src/lut.rs:429` | - | The LUT as a single cube over 0..1 (resampled when there is a shaper or a non-unit domain). |
| `impl Lut::apply` | 2 | `crates/color-transforms/src/lut.rs:438` | - |  |
| `write_domain` | 3 | `crates/color-transforms/src/lut.rs:450` | - |  |
| `fmt6` | 1 | `crates/color-transforms/src/lut.rs:457` | - |  |
| `bit_max` | 1 | `crates/color-transforms/src/lut.rs:463` | - | Full-scale value of the smallest common integer depth that holds `v`. |
| `brute_tetra` | 2 | `crates/color-transforms/src/lut.rs:473` | physics | Brute-force tetrahedral reference: find the Kuhn simplex (one of the 6 tetrahedra along the |
| `wild` | 1 | `crates/color-transforms/src/lut.rs:509` | - |  |
| `tetrahedral_matches_brute_force` | 0 | `crates/color-transforms/src/lut.rs:515` | physics |  |
| `tetrahedral_exact_on_affine_and_nodes` | 0 | `crates/color-transforms/src/lut.rs:537` | - |  |
| `cube_roundtrip_3d_1d_and_shaper` | 0 | `crates/color-transforms/src/lut.rs:553` | - |  |
| `cube_sizes_up_to_65_and_errors` | 0 | `crates/color-transforms/src/lut.rs:583` | - |  |
| `threedl_roundtrip_and_orientation` | 0 | `crates/color-transforms/src/lut.rs:601` | - |  |
| `impl NQuin48::new` | 5 | `crates/color-transforms/src/ontology.rs:15` | - |  |
| `impl NQuin48::precondition` | 3 | `crates/color-transforms/src/ontology.rs:19` | - |  |
| `impl NQuin48::postcondition` | 3 | `crates/color-transforms/src/ontology.rs:23` | - |  |
| `impl NQuin48::invariant` | 3 | `crates/color-transforms/src/ontology.rs:27` | - |  |
| `impl std::fmt::Display for PaletteError::fmt` | 2 | `crates/color-transforms/src/palette/aco.rs:13` | - |  |
| `impl AcoColor::from_record` | 2 | `crates/color-transforms/src/palette/aco.rs:45` | - |  |
| `impl AcoColor::to_record` | 1 | `crates/color-transforms/src/palette/aco.rs:56` | - |  |
| `parse_aco` | 1 | `crates/color-transforms/src/palette/aco.rs:80` | - |  |
| `write_aco` | 1 | `crates/color-transforms/src/palette/aco.rs:146` | - |  |
| `impl AseKind::from_u16` | 1 | `crates/color-transforms/src/palette/ase.rs:25` | - |  |
| `impl AseKind::to_u16` | 1 | `crates/color-transforms/src/palette/ase.rs:32` | - |  |
| `parse_ase` | 1 | `crates/color-transforms/src/palette/ase.rs:60` | - |  |
| `write_ase` | 1 | `crates/color-transforms/src/palette/ase.rs:209` | - |  |
| `parse_gpl` | 1 | `crates/color-transforms/src/palette/gpl.rs:19` | - |  |
| `write_gpl` | 1 | `crates/color-transforms/src/palette/gpl.rs:75` | - |  |
| `parse_grd` | 1 | `crates/color-transforms/src/palette/grd.rs:35` | - | Parses a `.grd` binary stream. |
| `write_grd` | 1 | `crates/color-transforms/src/palette/grd.rs:123` | - | Serializes gradients into `.grd` format. |
| `impl Gamut::primaries` | 1 | `crates/color-transforms/src/spaces.rs:42` | 3d | (x, y) of R, G, B. Sources: ITU-R BT.709-6, BT.2020-2, SMPTE EG 432-1 (P3), and the camera |
| `impl Gamut::from_primaries` | 1 | `crates/color-transforms/src/spaces.rs:57` | - |  |
| `impl Gamut::to_xyz` | 1 | `crates/color-transforms/src/spaces.rs:67` | - | RGB → CIE XYZ (normalised so white Y = 1). |
| `impl Gamut::luma` | 1 | `crates/color-transforms/src/spaces.rs:72` | - | Luminance weights (the Y row of [`Gamut::to_xyz`]). |
| `npm` | 2 | `crates/color-transforms/src/spaces.rs:78` | numerics | Normalised primary matrix (SMPTE RP 177). |
| `mul` | 2 | `crates/color-transforms/src/spaces.rs:86` | - |  |
| `mul_vec` | 2 | `crates/color-transforms/src/spaces.rs:96` | - |  |
| `inverse` | 1 | `crates/color-transforms/src/spaces.rs:100` | - |  |
| `gamut_matrix` | 2 | `crates/color-transforms/src/spaces.rs:114` | numerics | Linear RGB in `src` → linear RGB in `dst`. |
| `to_f32` | 1 | `crates/color-transforms/src/spaces.rs:121` | - |  |
| `apply3` | 2 | `crates/color-transforms/src/spaces.rs:126` | - |  |
| `impl ColorSpace::id` | 1 | `crates/color-transforms/src/spaces.rs:193` | - | Stable id used in commands and project files. |
| `impl ColorSpace::label` | 1 | `crates/color-transforms/src/spaces.rs:214` | - |  |
| `impl ColorSpace::parse` | 1 | `crates/color-transforms/src/spaces.rs:236` | - | Parse an id or a label (case-insensitive, punctuation ignored). |
| `impl ColorSpace::gamut` | 1 | `crates/color-transforms/src/spaces.rs:242` | - |  |
| `impl ColorSpace::curve` | 1 | `crates/color-transforms/src/spaces.rs:257` | - |  |
| `impl ColorSpace::is_hdr` | 1 | `crates/color-transforms/src/spaces.rs:276` | - | Carries more than SDR range (needs tone mapping into an SDR working space). |
| `impl ColorSpace::is_log` | 1 | `crates/color-transforms/src/spaces.rs:279` | - |  |
| `impl ColorSpace::from_info` | 1 | `crates/color-transforms/src/spaces.rs:284` | - | The colour space signalled by stream metadata (VUI / `colr` / MKV Colour). |
| `impl WorkingSpace::id` | 1 | `crates/color-transforms/src/spaces.rs:310` | - |  |
| `impl WorkingSpace::label` | 1 | `crates/color-transforms/src/spaces.rs:317` | - |  |
| `impl WorkingSpace::parse` | 1 | `crates/color-transforms/src/spaces.rs:324` | - |  |
| `impl WorkingSpace::is_hdr` | 1 | `crates/color-transforms/src/spaces.rs:329` | - |  |
| `impl WorkingSpace::output_space` | 1 | `crates/color-transforms/src/spaces.rs:333` | - | The colour space exports of this sequence are encoded in. |
| `impl ColorPipeline::working_gamut` | 1 | `crates/color-transforms/src/spaces.rs:356` | numerics | Linear primaries of the compositing space. |
| `impl ColorPipeline::is_plain` | 1 | `crates/color-transforms/src/spaces.rs:360` | numerics | The classic pipeline: linear BT.709, display-referred SDR. |
| `bt709_npm_matches_published_matrix` | 0 | `crates/color-transforms/src/spaces.rs:370` | numerics |  |
| `bt2020_to_709_matches_bt2087` | 0 | `crates/color-transforms/src/spaces.rs:385` | - |  |
| `ids_parse` | 0 | `crates/color-transforms/src/spaces.rs:408` | - |  |
| `decode_channel` | 3 | `crates/color-transforms/src/transform.rs:41` | numerics | Decode one channel: decoder-normalised signal → working-linear (no OOTF, no matrix). |
| `encode_channel` | 3 | `crates/color-transforms/src/transform.rs:59` | numerics | Encode one channel: working-linear → decoder-normalised signal (inverse of [`decode_channel`]). |
| `impl DecodeTable::new` | 4 | `crates/color-transforms/src/transform.rs:85` | - |  |
| `impl DecodeTable::for_curve` | 2 | `crates/color-transforms/src/transform.rs:90` | - | The decode table for a colour space's curve. |
| `impl DecodeTable::lookup` | 2 | `crates/color-transforms/src/transform.rs:97` | - |  |
| `bt2390_eetf` | 3 | `crates/color-transforms/src/transform.rs:108` | - | ITU-R BT.2390 EETF: maps `nits` from a display with peak `src_peak` to one with peak |
| `impl ToneMap::new` | 2 | `crates/color-transforms/src/transform.rs:137` | - |  |
| `impl ToneMap::apply` | 2 | `crates/color-transforms/src/transform.rs:147` | - |  |
| `gamut_compress` | 1 | `crates/color-transforms/src/transform.rs:166` | 3d | Compress out-of-gamut colours towards the achromatic axis (see module docs). Used on input, |
| `impl InputTransform::new` | 4 | `crates/color-transforms/src/transform.rs:205` | - | `src_peak_nits` is the mastering/content peak of PQ media when the file signals it. |
| `impl InputTransform::has_pixel_stage` | 1 | `crates/color-transforms/src/transform.rs:231` | 2d | Whether anything happens after the per-channel table. |
| `impl InputTransform::apply` | 2 | `crates/color-transforms/src/transform.rs:237` | 2d | The per-pixel stage (input: the table's output). |
| `impl InputTransform::convert` | 2 | `crates/color-transforms/src/transform.rs:259` | 2d, semantics, numerics | Signal → working linear for one RGB triple (table + pixel stage). |
| `gamut_within` | 2 | `crates/color-transforms/src/transform.rs:265` | - | Whether gamut `a` lies inside gamut `b` (so no compression is needed going a → b). |
| `desaturate_into_gamut` | 2 | `crates/color-transforms/src/transform.rs:273` | - | Bring a colour with negative components into gamut by desaturating towards its luminance |
| `impl OutputTransform::display` | 1 | `crates/color-transforms/src/transform.rs:299` | numerics | For the monitors: SDR BT.709 linear (encode with the sRGB curve as usual). |
| `impl OutputTransform::new` | 2 | `crates/color-transforms/src/transform.rs:303` | - | For export/encode into `target` (Rec709, Rec2100Pq or Rec2100Hlg). |
| `impl OutputTransform::is_identity` | 1 | `crates/color-transforms/src/transform.rs:311` | - |  |
| `impl OutputTransform::apply` | 2 | `crates/color-transforms/src/transform.rs:316` | numerics | Working linear → target linear (SDR: 1.0 = white; HDR: working units). |
| `impl OutputTransform::encode` | 2 | `crates/color-transforms/src/transform.rs:329` | numerics | Working linear → encoded signal (0..1, full scale) of the target colour space. |
| `log_to_rec709` | 3 | `crates/color-transforms/src/transform.rs:348` | 3d | Convenience: a log colour space → display Rec. 709 (tone mapped), used for the built-in |
| `domain_value` | 3 | `crates/color-transforms/src/transform.rs:356` | - | Decoder-normalised value of a curve-domain value — re-exported for callers that build their |
| `hdr` | 1 | `crates/color-transforms/src/transform.rs:365` | - |  |
| `channel_roundtrips_every_curve` | 0 | `crates/color-transforms/src/transform.rs:370` | - |  |
| `pq_and_hlg_reference_white` | 0 | `crates/color-transforms/src/transform.rs:387` | - |  |
| `eetf_properties` | 0 | `crates/color-transforms/src/transform.rs:405` | - |  |
| `tone_map_preserves_hue_and_maps_peak` | 0 | `crates/color-transforms/src/transform.rs:423` | - |  |
| `gamut_compression` | 0 | `crates/color-transforms/src/transform.rs:435` | - |  |
| `log_sources_land_sensibly_in_rec709` | 0 | `crates/color-transforms/src/transform.rs:450` | - |  |
| `sdr_in_hdr_keeps_values_and_display_tone_maps` | 0 | `crates/color-transforms/src/transform.rs:466` | - |  |
