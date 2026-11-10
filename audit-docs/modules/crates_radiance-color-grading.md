# `crates/radiance-color-grading`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_`radiance-color-grading` — Categorical film color science, color space transforms, and real-time scopes.

Provides:
- Standard color space transforms: Rec.709, Rec.2020, ACEScc, ACEScct, ACEScg, P3-D65
- ASC CDL v1.2 (Slope / Offset / Power / Saturation) forward, inverse, and XML parsing
- 3-way color wheels (Lift / Gamma / Gain / Offset / Balance)
- `.cube` and `.3dl` 3D LUT parser, generator, and tetrahedral/trilinear interpolation
- Camera log curves: Sony S-Log3, Panasonic V-Log, Canon Log 2/3, ARRI LogC3/4, Apple Log, DJI D-Log
- Tone mapping operators: Reinhard, ACES fitted, Filmic, ITU-R BT.2446a
- Real-time video scopes: Waveform monitor, Vectorscope (with skin-tone line and 75%/100% targets), RGB Parade, Histograms

Grounded in international human rights instruments protecting creative expression, moral rights,
cultural heritage, and informatics fiduciary integrity (UDHR Articles 12, 19, 27; ICCPR Articles 17, 19;
ICESCR Article 15)._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl GradeSpace::new` | 2 | `crates/radiance-color-grading/src/grade.rs:28` | - |  |
| `impl GradeSpace::is_hdr` | 1 | `crates/radiance-color-grading/src/grade.rs:43` | - |  |
| `impl GradeSpace::white_nits` | 1 | `crates/radiance-color-grading/src/grade.rs:47` | - |  |
| `impl GradeSpace::encode` | 2 | `crates/radiance-color-grading/src/grade.rs:56` | numerics | Working linear -> grading signal. |
| `impl GradeSpace::decode` | 2 | `crates/radiance-color-grading/src/grade.rs:78` | numerics | Grading signal -> working linear. |
| `impl GradeSpace::signal_of_nits` | 2 | `crates/radiance-color-grading/src/grade.rs:97` | - |  |
| `impl GradeSpace::nits_of_signal` | 2 | `crates/radiance-color-grading/src/grade.rs:102` | - |  |
| `impl Default for AscCdl::default` | 0 | `crates/radiance-color-grading/src/grade.rs:123` | - |  |
| `impl AscCdl::new` | 4 | `crates/radiance-color-grading/src/grade.rs:134` | - |  |
| `impl AscCdl::apply` | 3 | `crates/radiance-color-grading/src/grade.rs:144` | - | Forward ASC CDL: `out = clamp((in * slope + offset)^power)`. |
| `impl AscCdl::invert` | 3 | `crates/radiance-color-grading/src/grade.rs:166` | - | Exact inverse of ASC CDL (within clamped range). |
| `impl AscCdl::parse_xml` | 1 | `crates/radiance-color-grading/src/grade.rs:195` | - | Parse ASC CDL XML string (`.cc`, `.ccc`, `.cdl`). |
| `tag` | 2 | `crates/radiance-color-grading/src/grade.rs:239` | - |  |
| `nums3` | 1 | `crates/radiance-color-grading/src/grade.rs:249` | - |  |
| `impl Default for ColorWheels::default` | 0 | `crates/radiance-color-grading/src/grade.rs:278` | - |  |
| `impl ColorWheels::apply` | 2 | `crates/radiance-color-grading/src/grade.rs:296` | - | Apply Lift/Gamma/Gain + Offset grading to an RGB triplet. |
| `test_ycbcr_roundtrip` | 0 | `crates/radiance-color-grading/src/lib.rs:45` | - |  |
| `test_transfer_roundtrips` | 0 | `crates/radiance-color-grading/src/lib.rs:57` | - |  |
| `test_asc_cdl_apply_and_invert` | 0 | `crates/radiance-color-grading/src/lib.rs:68` | - |  |
| `test_color_wheels_apply` | 0 | `crates/radiance-color-grading/src/lib.rs:84` | - |  |
| `test_lut_cube_tetrahedral` | 0 | `crates/radiance-color-grading/src/lib.rs:96` | - |  |
| `test_log_curves_encode_decode` | 0 | `crates/radiance-color-grading/src/lib.rs:107` | - |  |
| `test_spaces_gamut_matrix` | 0 | `crates/radiance-color-grading/src/lib.rs:117` | numerics |  |
| `impl LogCurve::domain` | 1 | `crates/radiance-color-grading/src/log.rs:48` | - |  |
| `impl LogCurve::label` | 1 | `crates/radiance-color-grading/src/log.rs:55` | - |  |
| `impl LogCurve::encode` | 2 | `crates/radiance-color-grading/src/log.rs:71` | numerics | Linear scene reflectance -> log signal. |
| `impl LogCurve::decode` | 2 | `crates/radiance-color-grading/src/log.rs:87` | numerics | Log signal -> linear scene reflectance. |
| `impl LogCurve::signal_from_normalized` | 3 | `crates/radiance-color-grading/src/log.rs:102` | - |  |
| `impl LogCurve::normalized_from_signal` | 3 | `crates/radiance-color-grading/src/log.rs:111` | - |  |
| `slog3_encode` | 1 | `crates/radiance-color-grading/src/log.rs:122` | - |  |
| `slog3_decode` | 1 | `crates/radiance-color-grading/src/log.rs:130` | - |  |
| `vlog_encode` | 1 | `crates/radiance-color-grading/src/log.rs:145` | - |  |
| `vlog_decode` | 1 | `crates/radiance-color-grading/src/log.rs:153` | - |  |
| `clog2_encode` | 1 | `crates/radiance-color-grading/src/log.rs:162` | - |  |
| `clog2_decode` | 1 | `crates/radiance-color-grading/src/log.rs:172` | - |  |
| `clog3_encode` | 1 | `crates/radiance-color-grading/src/log.rs:181` | - |  |
| `clog3_decode` | 1 | `crates/radiance-color-grading/src/log.rs:191` | - |  |
| `logc3_encode` | 1 | `crates/radiance-color-grading/src/log.rs:208` | - |  |
| `logc3_decode` | 1 | `crates/radiance-color-grading/src/log.rs:216` | - |  |
| `logc4_encode` | 1 | `crates/radiance-color-grading/src/log.rs:231` | - |  |
| `logc4_decode` | 1 | `crates/radiance-color-grading/src/log.rs:239` | - |  |
| `apple_log_encode` | 1 | `crates/radiance-color-grading/src/log.rs:256` | - |  |
| `apple_log_decode` | 1 | `crates/radiance-color-grading/src/log.rs:266` | - |  |
| `dlog_encode` | 1 | `crates/radiance-color-grading/src/log.rs:278` | - |  |
| `dlog_decode` | 1 | `crates/radiance-color-grading/src/log.rs:286` | - |  |
| `aces_cc_encode` | 1 | `crates/radiance-color-grading/src/log.rs:298` | - |  |
| `aces_cc_decode` | 1 | `crates/radiance-color-grading/src/log.rs:308` | - |  |
| `aces_cct_encode` | 1 | `crates/radiance-color-grading/src/log.rs:319` | - |  |
| `aces_cct_decode` | 1 | `crates/radiance-color-grading/src/log.rs:327` | - |  |
| `impl Lut1d::identity` | 1 | `crates/radiance-color-grading/src/lut.rs:17` | - |  |
| `impl Lut1d::size` | 1 | `crates/radiance-color-grading/src/lut.rs:26` | - |  |
| `impl Lut1d::apply` | 2 | `crates/radiance-color-grading/src/lut.rs:31` | - |  |
| `impl Lut3d::identity` | 1 | `crates/radiance-color-grading/src/lut.rs:63` | - |  |
| `impl Lut3d::from_fn` | 2 | `crates/radiance-color-grading/src/lut.rs:67` | - |  |
| `impl Lut3d::at` | 4 | `crates/radiance-color-grading/src/lut.rs:87` | - |  |
| `impl Lut3d::coord` | 3 | `crates/radiance-color-grading/src/lut.rs:92` | - |  |
| `impl Lut3d::apply` | 2 | `crates/radiance-color-grading/src/lut.rs:102` | - | Tetrahedral 3D interpolation. |
| `impl Lut3d::apply_trilinear` | 2 | `crates/radiance-color-grading/src/lut.rs:146` | - | Trilinear 3D interpolation. |
| `impl Lut3d::parse_cube` | 1 | `crates/radiance-color-grading/src/lut.rs:177` | - | Parse a `.cube` file string. |
| `impl Lut3d::to_cube` | 1 | `crates/radiance-color-grading/src/lut.rs:187` | - | Serialize this 3D LUT to `.cube` format string. |
| `impl Lut::from_cube` | 1 | `crates/radiance-color-grading/src/lut.rs:212` | - |  |
| `impl Lut::apply` | 2 | `crates/radiance-color-grading/src/lut.rs:220` | - |  |
| `impl Lut::parse_cube` | 1 | `crates/radiance-color-grading/src/lut.rs:232` | - | Parse a `.cube` text representation. |
| `impl Lut::to_cube` | 1 | `crates/radiance-color-grading/src/lut.rs:326` | - | Serialize to `.cube` format string. |
| `parse_nums3` | 1 | `crates/radiance-color-grading/src/lut.rs:361` | - |  |
| `impl Matrix::kr_kb` | 1 | `crates/radiance-color-grading/src/matrix.rs:24` | - | (Kr, Kb) luma coefficients. |
| `impl Matrix::from_code` | 1 | `crates/radiance-color-grading/src/matrix.rs:33` | numerics | From an ISO/IEC 23091-2 `matrix_coefficients` code. |
| `impl Transfer::from_code` | 1 | `crates/radiance-color-grading/src/matrix.rs:58` | - |  |
| `impl HdrMetadata::peak_nits` | 1 | `crates/radiance-color-grading/src/matrix.rs:127` | - | The content peak for tone mapping. |
| `ycbcr_to_rgb` | 4 | `crates/radiance-color-grading/src/matrix.rs:140` | numerics | Convert normalized Y'CbCr (Y in 0..1, Cb/Cr in -0.5..0.5) to non-linear R'G'B'. |
| `rgb_to_ycbcr` | 4 | `crates/radiance-color-grading/src/matrix.rs:151` | numerics | Convert non-linear R'G'B' (nominal 0..1) to normalized Y'CbCr. |
| `normalize_y` | 3 | `crates/radiance-color-grading/src/matrix.rs:162` | - | Quantized integer sample -> normalized Y float (0..1 nominal). |
| `normalize_c` | 3 | `crates/radiance-color-grading/src/matrix.rs:177` | - | Quantized integer sample -> normalized Cb/Cr float (-0.5..0.5 nominal). |
| `denormalize_y` | 3 | `crates/radiance-color-grading/src/matrix.rs:192` | - | Normalized Y float (0..1 nominal) -> integer sample clamped to range. |
| `denormalize_c` | 3 | `crates/radiance-color-grading/src/matrix.rs:208` | - | Normalized Cb/Cr float (-0.5..0.5 nominal) -> integer sample clamped to range. |
| `to_linear` | 2 | `crates/radiance-color-grading/src/matrix.rs:224` | numerics | Convert encoded display value to linear light. |
| `from_linear` | 2 | `crates/radiance-color-grading/src/matrix.rs:242` | numerics | Convert linear light to encoded display value. |
| `srgb_to_linear` | 1 | `crates/radiance-color-grading/src/matrix.rs:259` | numerics |  |
| `linear_to_srgb` | 1 | `crates/radiance-color-grading/src/matrix.rs:270` | numerics |  |
| `pq_eotf` | 1 | `crates/radiance-color-grading/src/matrix.rs:280` | - |  |
| `pq_inverse_eotf` | 1 | `crates/radiance-color-grading/src/matrix.rs:297` | - |  |
| `hlg_oetf` | 1 | `crates/radiance-color-grading/src/matrix.rs:309` | - |  |
| `hlg_inverse_oetf` | 1 | `crates/radiance-color-grading/src/matrix.rs:318` | - |  |
| `srgb_u8_to_linear_table` | 0 | `crates/radiance-color-grading/src/matrix.rs:328` | numerics | 256-entry table: sRGB-encoded u8 -> linear f32. |
| `linear_to_srgb_u8_table` | 0 | `crates/radiance-color-grading/src/matrix.rs:334` | numerics | 4096-entry table: linear (0..1 quantised to 12 bits) -> sRGB u8. |
| `linear_to_srgb_u8` | 1 | `crates/radiance-color-grading/src/matrix.rs:344` | numerics |  |
| `luma709` | 3 | `crates/radiance-color-grading/src/matrix.rs:351` | semantics, numerics | Rec.709 luma of a linear or encoded RGB triple. |
| `rgb_to_hsl` | 3 | `crates/radiance-color-grading/src/matrix.rs:356` | - | RGB (0..1) -> HSL (h in 0..1). |
| `hsl_to_rgb` | 3 | `crates/radiance-color-grading/src/matrix.rs:379` | - |  |
| `parse_hex` | 1 | `crates/radiance-color-grading/src/matrix.rs:405` | - | Parse `#rrggbb` / `#rrggbbaa` into 0..1 floats. |
| `to_hex` | 1 | `crates/radiance-color-grading/src/matrix.rs:415` | - |  |
| `impl ScopeKind::name` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:40` | - |  |
| `impl ScopeKind::label` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:50` | - |  |
| `impl WaveformType::label` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:79` | - |  |
| `impl ParadeType::label` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:101` | - |  |
| `impl Targets::amplitude` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:120` | - |  |
| `impl Targets::label` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:127` | - |  |
| `impl Default for Params::default` | 0 | `crates/radiance-color-grading/src/scopes/mod.rs:144` | - |  |
| `impl Params::range` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:155` | - |  |
| `steps` | 4 | `crates/radiance-color-grading/src/scopes/mod.rs:171` | - |  |
| `impl Signal::from_rgba8` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:176` | - |  |
| `impl Signal::from_rgba8_max` | 5 | `crates/radiance-color-grading/src/scopes/mod.rs:180` | - |  |
| `impl Signal::from_fn` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:198` | - |  |
| `impl Signal::len` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:208` | - |  |
| `impl Signal::is_empty` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:212` | - |  |
| `impl Grid::new` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:226` | - |  |
| `impl Grid::at` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:235` | - |  |
| `impl Grid::total` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:239` | - |  |
| `impl Grid::max` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:243` | - |  |
| `impl Grid::rows_in` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:247` | - |  |
| `impl Waveform::trace` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:261` | - |  |
| `impl Waveform::level` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:265` | - |  |
| `row_of` | 4 | `crates/radiance-color-grading/src/scopes/mod.rs:271` | - |  |
| `build` | 4 | `crates/radiance-color-grading/src/scopes/mod.rs:281` | - |  |
| `waveform` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:309` | - |  |
| `parade` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:327` | - |  |
| `bin_of` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:361` | - |  |
| `histogram` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:369` | - |  |
| `cell_of` | 4 | `crates/radiance-color-grading/src/scopes/mod.rs:418` | - |  |
| `impl Vectorscope::new` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:429` | - |  |
| `impl Vectorscope::at` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:439` | - |  |
| `impl Vectorscope::cell` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:443` | - |  |
| `impl Vectorscope::point` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:447` | - |  |
| `impl Vectorscope::plot` | 4 | `crates/radiance-color-grading/src/scopes/mod.rs:455` | - |  |
| `impl Vectorscope::finish` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:464` | - |  |
| `vectorscope_yuv` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:475` | - |  |
| `angle_deg` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:490` | - |  |
| `red_angle` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:494` | numerics |  |
| `vectorscope_hls` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:499` | - |  |
| `targets` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:525` | numerics |  |
| `intensity` | 3 | `crates/radiance-color-grading/src/scopes/paint.rs:7` | - |  |
| `grids` | 3 | `crates/radiance-color-grading/src/scopes/paint.rs:15` | - |  |
| `waveform_reference` | 1 | `crates/radiance-color-grading/src/scopes/paint.rs:54` | - |  |
| `vectorscope` | 4 | `crates/radiance-color-grading/src/scopes/paint.rs:58` | - |  |
| `impl Acc::add` | 3 | `crates/radiance-color-grading/src/scopes/summary.rs:24` | - |  |
| `impl Acc::level` | 1 | `crates/radiance-color-grading/src/scopes/summary.rs:36` | - |  |
| `round2` | 1 | `crates/radiance-color-grading/src/scopes/summary.rs:45` | - |  |
| `channel_stats` | 2 | `crates/radiance-color-grading/src/scopes/summary.rs:61` | numerics |  |
| `peaks` | 3 | `crates/radiance-color-grading/src/scopes/summary.rs:88` | - |  |
| `rgba8` | 3 | `crates/radiance-color-grading/src/scopes/tests.rs:6` | - |  |
| `solid` | 3 | `crates/radiance-color-grading/src/scopes/tests.rs:17` | - |  |
| `bars` | 0 | `crates/radiance-color-grading/src/scopes/tests.rs:33` | - |  |
| `code` | 1 | `crates/radiance-color-grading/src/scopes/tests.rs:37` | - |  |
| `single_colour_lands_in_exact_histogram_bins` | 0 | `crates/radiance-color-grading/src/scopes/tests.rs:42` | - |  |
| `ramp_fills_every_bin_and_waveform_row_equals_code_value` | 0 | `crates/radiance-color-grading/src/scopes/tests.rs:55` | - |  |
| `colour_bars_parade_levels_and_vectorscope_targets` | 0 | `crates/radiance-color-grading/src/scopes/tests.rs:78` | - |  |
| `impl Gamut::primaries` | 1 | `crates/radiance-color-grading/src/spaces.rs:35` | - | (x, y) coordinates of R, G, B primaries. |
| `impl Gamut::white_point` | 1 | `crates/radiance-color-grading/src/spaces.rs:52` | - |  |
| `impl Gamut::from_primaries` | 1 | `crates/radiance-color-grading/src/spaces.rs:59` | - |  |
| `impl Gamut::to_xyz` | 1 | `crates/radiance-color-grading/src/spaces.rs:68` | - | RGB -> CIE XYZ (normalised so white Y = 1). |
| `impl Gamut::luma` | 1 | `crates/radiance-color-grading/src/spaces.rs:73` | - | Luminance weights (Y row of `to_xyz`). |
| `npm` | 2 | `crates/radiance-color-grading/src/spaces.rs:79` | numerics | Normalised primary matrix (SMPTE RP 177). |
| `mul` | 2 | `crates/radiance-color-grading/src/spaces.rs:91` | - |  |
| `mul_vec` | 2 | `crates/radiance-color-grading/src/spaces.rs:101` | - |  |
| `inverse` | 1 | `crates/radiance-color-grading/src/spaces.rs:109` | - |  |
| `gamut_matrix` | 2 | `crates/radiance-color-grading/src/spaces.rs:123` | numerics | Linear RGB in `src` -> linear RGB in `dst`. |
| `bradford` | 2 | `crates/radiance-color-grading/src/spaces.rs:138` | numerics | Bradford chromatic adaptation transform matrix between two white points. |
| `to_f32` | 1 | `crates/radiance-color-grading/src/spaces.rs:172` | - |  |
| `apply3` | 2 | `crates/radiance-color-grading/src/spaces.rs:177` | - |  |
| `impl ColorSpace::id` | 1 | `crates/radiance-color-grading/src/spaces.rs:247` | - |  |
| `impl ColorSpace::label` | 1 | `crates/radiance-color-grading/src/spaces.rs:272` | - |  |
| `impl ColorSpace::gamut` | 1 | `crates/radiance-color-grading/src/spaces.rs:297` | - |  |
| `impl ColorSpace::curve` | 1 | `crates/radiance-color-grading/src/spaces.rs:314` | - |  |
| `impl ColorSpace::is_hdr` | 1 | `crates/radiance-color-grading/src/spaces.rs:335` | - |  |
| `impl ColorSpace::is_log` | 1 | `crates/radiance-color-grading/src/spaces.rs:339` | - |  |
| `impl WorkingSpace::gamut` | 1 | `crates/radiance-color-grading/src/spaces.rs:354` | - |  |
| `impl WorkingSpace::is_hdr` | 1 | `crates/radiance-color-grading/src/spaces.rs:361` | - |  |
| `impl ToneMap::map_luma` | 3 | `crates/radiance-color-grading/src/transform.rs:29` | numerics | Apply tone mapping to a linear luminance value `y` with target peak `peak_nits`. |
| `impl ToneMap::apply_rgb` | 3 | `crates/radiance-color-grading/src/transform.rs:63` | numerics | Apply tone mapping to an RGB triplet in linear working units. |
| `impl InputTransform::new` | 2 | `crates/radiance-color-grading/src/transform.rs:86` | - |  |
| `impl InputTransform::apply` | 2 | `crates/radiance-color-grading/src/transform.rs:97` | numerics | Transform an encoded source color triplet to sequence working linear light. |
| `impl OutputTransform::new` | 4 | `crates/radiance-color-grading/src/transform.rs:122` | - |  |
| `impl OutputTransform::apply` | 2 | `crates/radiance-color-grading/src/transform.rs:135` | numerics | Transform sequence working linear color to target encoded display/export color. |
| `impl DecodeTable::from_fn` | 2 | `crates/radiance-color-grading/src/transform.rs:156` | - |  |
| `impl DecodeTable::lookup` | 2 | `crates/radiance-color-grading/src/transform.rs:163` | - |  |
