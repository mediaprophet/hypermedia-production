# Documentation coverage

A `///`/`/**`/docstring attached to a function counts as documented. Test functions, private helpers and WGSL (no doc-comment convention) lower the raw rate — track **public coverage** as the KPI.

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.

| module | functions | documented | coverage | undocumented | undocumented public |
|---|---|---|---|---|---|
| `crates/chronicle-tabular-engine` | 274 | 17 | 6% | 257 | 195 |
| `crates/vector-compute-shaders` | 259 | 7 | 3% | 252 | 172 |
| `crates/asset-production-vibe` | 814 | 172 | 21% | 642 | 153 |
| `crates/color-transforms` | 288 | 91 | 32% | 197 | 123 |
| `crates/drafting-geometry` | 264 | 62 | 23% | 202 | 122 |
| `crates/vector-hybrid-renderer` | 802 | 209 | 26% | 593 | 120 |
| `crates/radiance-color-grading` | 175 | 40 | 23% | 135 | 86 |
| `crates/audio-device-cpal` | 539 | 86 | 16% | 453 | 79 |
| `crates/chronicle-broadcast-containers` | 148 | 27 | 18% | 121 | 79 |
| `crates/media-isobmff` | 255 | 71 | 28% | 184 | 69 |
| `crates/kinetics-block-simulation` | 267 | 0 | 0% | 267 | 65 |
| `crates/kinetics-schedule-cpm` | 99 | 2 | 2% | 97 | 61 |
| `crates/surface-pbr-studio` | 108 | 17 | 16% | 91 | 60 |
| `crates/terrain-brushes` | 104 | 13 | 12% | 91 | 49 |
| `crates/acoustics-daw-session` | 81 | 27 | 33% | 54 | 45 |
| `crates/multimodal-synthesis-schema` | 66 | 6 | 9% | 60 | 45 |
| `crates/video-h264-codec` | 332 | 144 | 43% | 188 | 44 |
| `crates/audio-opus-codec` | 203 | 77 | 38% | 126 | 43 |
| `crates/discourse-scientific-matlang` | 77 | 7 | 9% | 70 | 42 |
| `crates/manifold-bim-csg` | 84 | 11 | 13% | 73 | 41 |
| `crates/chronicle-timeline-interchange` | 90 | 21 | 23% | 69 | 40 |
| `crates/layout-presentation-deck` | 63 | 11 | 17% | 52 | 38 |
| `crates/layout-paged-typesetter` | 61 | 9 | 15% | 52 | 34 |
| `crates/audio-aac-codec` | 190 | 82 | 43% | 108 | 33 |
| `crates/chronicle-office-codecs` | 83 | 22 | 27% | 61 | 32 |
| `crates/vector-compute-renderer` | 161 | 77 | 48% | 84 | 31 |
| `crates/geodesy-spatial-analysis` | 83 | 38 | 46% | 45 | 30 |
| `crates/kinetics-garment-draping` | 76 | 8 | 11% | 68 | 29 |
| `crates/vector-locus-geometry` | 226 | 109 | 48% | 117 | 29 |
| `crates/vector-skia-render-adapter` | 109 | 7 | 6% | 102 | 29 |
| `crates/audio-timebase` | 111 | 50 | 45% | 61 | 28 |
| `crates/locus-parametric-solver` | 70 | 11 | 16% | 59 | 26 |
| `crates/surface-layered-codecs` | 50 | 15 | 30% | 35 | 23 |
| `crates/discourse-pdf-compliance` | 53 | 16 | 30% | 37 | 22 |
| `crates/layout-box-autolayout` | 35 | 4 | 11% | 31 | 22 |
| `crates/bitstream-codec` | 42 | 14 | 33% | 28 | 21 |
| `crates/layout-nodegraph-compiler` | 41 | 4 | 10% | 37 | 21 |
| `crates/chronicle-fiduciary-ledger` | 56 | 19 | 34% | 37 | 20 |
| `crates/kinetics-motion-compositor` | 65 | 23 | 35% | 42 | 20 |
| `crates/vector-locus-pathops` | 125 | 49 | 39% | 76 | 20 |
| `crates/vector-surface-brush` | 58 | 10 | 17% | 48 | 20 |
| `crates/vector-hybrid-render-adapter` | 57 | 1 | 2% | 56 | 18 |
| `crates/vector-radiance-trace` | 99 | 18 | 18% | 81 | 18 |
| `crates/video-prores-codec` | 88 | 29 | 33% | 59 | 18 |
| `crates/discourse-academic-tex` | 208 | 44 | 21% | 164 | 17 |
| `crates/gpu-device-context` | 48 | 21 | 44% | 27 | 17 |
| `crates/raster-layer-mask-adapter` | 60 | 19 | 32% | 41 | 17 |
| `crates/profile-authoring` | 47 | 2 | 4% | 45 | 14 |
| `crates/electromagnetics-rf-sensing` | 41 | 16 | 39% | 25 | 13 |
| `crates/kinetics-continuum-multiphysics` | 95 | 55 | 58% | 40 | 13 |
| `crates/radiance-camera-raw` | 35 | 14 | 40% | 21 | 12 |
| `crates/vector-gpu-render-adapter` | 44 | 2 | 5% | 42 | 12 |
| `crates/video-h264-preview` | 99 | 21 | 21% | 78 | 12 |
| `crates/locus-geospatial-codecs` | 60 | 21 | 35% | 39 | 11 |
| `crates/timeline-editing` | 43 | 16 | 37% | 27 | 11 |
| `crates/pcm-device-output` | 31 | 2 | 6% | 29 | 10 |
| `crates/raster-blend-operators` | 62 | 23 | 37% | 39 | 10 |
| `crates/pixel-buffer-output` | 53 | 43 | 81% | 10 | 9 |
| `crates/gpu-command-profiler` | 72 | 21 | 29% | 51 | 7 |
| `crates/pixel-window-render-adapter` | 17 | 0 | 0% | 17 | 7 |
| `crates/vector-cpu-render-adapter` | 45 | 8 | 18% | 37 | 6 |
| `crates/vector-glyph-shaping` | 49 | 16 | 33% | 33 | 6 |
| `crates/media-test-support` | 52 | 28 | 54% | 24 | 5 |
| `crates/raster-spatial-operators` | 62 | 24 | 39% | 38 | 4 |
| `crates/media-ogg` | 43 | 12 | 28% | 31 | 2 |
| `crates/raster-selection-morphology` | 46 | 20 | 43% | 26 | 1 |
| `crates/vector-hybrid-shaders` | 51 | 19 | 37% | 32 | 1 |
| `checks/consumer-smoke` | 1 | 0 | 0% | 1 | 0 |
| `crates/terrain-erosion-solvers` | 50 | 18 | 36% | 32 | 0 |
| **total** | **8745** | **2198** | **25%** | **6547** | **2532** |

**Public-surface coverage: 1822/4354 (42%)** — the KPI worth tracking.

## Undocumented-public worklist

Top 25 per module (full list in `modules/*.md` — blank `doc` column):

### `crates/chronicle-tabular-engine` (195 undocumented public)

- `impl Evaluator<'h>::new` — `crates/chronicle-tabular-engine/src/calc/eval.rs:59`
- `impl Evaluator<'h>::eval` — `crates/chronicle-tabular-engine/src/calc/eval.rs:69`
- `impl Evaluator<'h>::to_value` — `crates/chronicle-tabular-engine/src/calc/eval.rs:153`
- `precedents` — `crates/chronicle-tabular-engine/src/calc/eval.rs:382`
- `impl Formula::new` — `crates/chronicle-tabular-engine/src/calc/model.rs:55`
- `impl Formula::expr` — `crates/chronicle-tabular-engine/src/calc/model.rs:69`
- `impl Sheet::new` — `crates/chronicle-tabular-engine/src/calc/model.rs:90`
- `impl Sheet::value` — `crates/chronicle-tabular-engine/src/calc/model.rs:100`
- `impl Sheet::set_value` — `crates/chronicle-tabular-engine/src/calc/model.rs:107`
- `impl Sheet::set_formula` — `crates/chronicle-tabular-engine/src/calc/model.rs:111`
- `impl Sheet::used_range` — `crates/chronicle-tabular-engine/src/calc/model.rs:116`
- `impl Workbook::new` — `crates/chronicle-tabular-engine/src/calc/model.rs:159`
- `impl Workbook::sheet` — `crates/chronicle-tabular-engine/src/calc/model.rs:163`
- `impl Workbook::sheet_mut` — `crates/chronicle-tabular-engine/src/calc/model.rs:167`
- `impl Workbook::add_sheet` — `crates/chronicle-tabular-engine/src/calc/model.rs:172`
- `impl Graph::new` — `crates/chronicle-tabular-engine/src/calc/recalc.rs:47`
- `impl Graph::insert` — `crates/chronicle-tabular-engine/src/calc/recalc.rs:51`
- `impl Graph::dependents` — `crates/chronicle-tabular-engine/src/calc/recalc.rs:60`
- `impl Calc::new` — `crates/chronicle-tabular-engine/src/calc/recalc.rs:82`
- `impl Calc::rebuild` — `crates/chronicle-tabular-engine/src/calc/recalc.rs:86`
- `impl Calc::recalc_all` — `crates/chronicle-tabular-engine/src/calc/recalc.rs:100`
- `impl CellRef::new` — `crates/chronicle-tabular-engine/src/core/addr.rs:20`
- `impl CellRef::is_valid` — `crates/chronicle-tabular-engine/src/core/addr.rs:29`
- `impl RangeRef::new` — `crates/chronicle-tabular-engine/src/core/addr.rs:140`
- `impl RangeRef::cell` — `crates/chronicle-tabular-engine/src/core/addr.rs:147`
- _…170 more in `modules/crates_chronicle-tabular-engine.md`_

### `crates/vector-compute-shaders` (172 undocumented public)

- `main` — `crates/vector-compute-shaders/shader/backdrop.wgsl:24`
- `main` — `crates/vector-compute-shaders/shader/backdrop_dyn.wgsl:28`
- `main` — `crates/vector-compute-shaders/shader/bbox_clear.wgsl:13`
- `main` — `crates/vector-compute-shaders/shader/binning.wgsl:55`
- `search_link` — `crates/vector-compute-shaders/shader/clip_leaf.wgsl:37`
- `load_clip_path` — `crates/vector-compute-shaders/shader/clip_leaf.wgsl:70`
- `main` — `crates/vector-compute-shaders/shader/clip_leaf.wgsl:80`
- `main` — `crates/vector-compute-shaders/shader/clip_reduce.wgsl:24`
- `alloc_cmd` — `crates/vector-compute-shaders/shader/coarse.wgsl:68`
- `write_path` — `crates/vector-compute-shaders/shader/coarse.wgsl:88`
- `write_color` — `crates/vector-compute-shaders/shader/coarse.wgsl:112`
- `write_grad` — `crates/vector-compute-shaders/shader/coarse.wgsl:119`
- `write_image` — `crates/vector-compute-shaders/shader/coarse.wgsl:127`
- `write_begin_clip` — `crates/vector-compute-shaders/shader/coarse.wgsl:134`
- `write_end_clip` — `crates/vector-compute-shaders/shader/coarse.wgsl:140`
- `write_blurred_rounded_rect` — `crates/vector-compute-shaders/shader/coarse.wgsl:148`
- `main` — `crates/vector-compute-shaders/shader/coarse.wgsl:156`
- `read_transform` — `crates/vector-compute-shaders/shader/draw_leaf.wgsl:37`
- `main` — `crates/vector-compute-shaders/shader/draw_leaf.wgsl:52`
- `main` — `crates/vector-compute-shaders/shader/draw_reduce.wgsl:22`
- `span` — `crates/vector-compute-shaders/shader/fine.wgsl:113`
- `fill_path_ms` — `crates/vector-compute-shaders/shader/fine.wgsl:146`
- `fill_path_ms_evenodd` — `crates/vector-compute-shaders/shader/fine.wgsl:511`
- `erf7` — `crates/vector-compute-shaders/shader/fine.wgsl:715`
- `hypot` — `crates/vector-compute-shaders/shader/fine.wgsl:724`
- _…147 more in `modules/crates_vector-compute-shaders.md`_

### `crates/asset-production-vibe` (153 undocumented public)

- `impl AssetHost<'_>::route_bus_dag` — `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:11`
- `impl AssetHost<'_>::spatialize_pga_zone` — `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:339`
- `impl AssetHost<'_>::inspect_engine_telemetry` — `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:98`
- `impl AssetHost<'_>::verify_fiduciary_rid` — `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:205`
- `impl AssetHost<'_>::capture_deterministic_movie` — `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:275`
- `impl AssetHost<'_>::sync_spatial_motors` — `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:322`
- `impl AssetHost<'_>::chronicle_op` — `crates/asset-production-vibe/src/chronicle_ops.rs:13`
- `impl AssetHost<'_>::compile_prose_docx` — `crates/asset-production-vibe/src/discourse_docs/docx_pptx.rs:21`
- `impl AssetHost<'_>::compile_deck_pptx` — `crates/asset-production-vibe/src/discourse_docs/docx_pptx.rs:159`
- `impl ZipArchiveWriter::new` — `crates/asset-production-vibe/src/discourse_docs/mod.rs:49`
- `impl ZipArchiveWriter::add_file` — `crates/asset-production-vibe/src/discourse_docs/mod.rs:55`
- `impl ZipArchiveWriter::finish` — `crates/asset-production-vibe/src/discourse_docs/mod.rs:62`
- `impl AssetHost<'_>::compile_pdf` — `crates/asset-production-vibe/src/discourse_docs/mod.rs:294`
- `impl AssetHost<'_>::render_chart_svg` — `crates/asset-production-vibe/src/discourse_docs/pdf_chart.rs:27`
- `impl AssetHost<'_>::geodesy_op` — `crates/asset-production-vibe/src/geodesy_ops.rs:302`
- `impl AssetHost<'_>::geometry` — `crates/asset-production-vibe/src/geometry.rs:7`
- `impl AssetHost<'_>::profile` — `crates/asset-production-vibe/src/geometry.rs:288`
- `impl AssetHost<'_>::store_profile` — `crates/asset-production-vibe/src/geometry.rs:294`
- `impl AssetHost<'_>::mesh_budget` — `crates/asset-production-vibe/src/geometry.rs:313`
- `impl AssetHost<'_>::store_mesh` — `crates/asset-production-vibe/src/geometry.rs:319`
- `color` — `crates/asset-production-vibe/src/geometry.rs:411`
- `impl SubpathBuilder<'a>::new` — `crates/asset-production-vibe/src/glyph/mod.rs:148`
- `impl SubpathBuilder<'a>::stroke` — `crates/asset-production-vibe/src/glyph/mod.rs:157`
- `impl SubpathBuilder<'a>::closed` — `crates/asset-production-vibe/src/glyph/mod.rs:170`
- `char_to_svg_subpath_ext` — `crates/asset-production-vibe/src/glyph/stroke_table.rs:5`
- _…128 more in `modules/crates_asset-production-vibe.md`_

### `crates/color-transforms` (123 undocumented public)

- `impl GradeSpace::is_hdr` — `crates/color-transforms/src/grade.rs:50`
- `impl Builtin::id` — `crates/color-transforms/src/icc/builtin.rs:47`
- `impl Builtin::description` — `crates/color-transforms/src/icc/builtin.rs:62`
- `impl Builtin::profile` — `crates/color-transforms/src/icc/builtin.rs:77`
- `srgb` — `crates/color-transforms/src/icc/builtin.rs:83`
- `display_p3` — `crates/color-transforms/src/icc/builtin.rs:87`
- `adobe_rgb` — `crates/color-transforms/src/icc/builtin.rs:91`
- `prophoto_rgb` — `crates/color-transforms/src/icc/builtin.rs:95`
- `rec2020` — `crates/color-transforms/src/icc/builtin.rs:99`
- `generic_cmyk` — `crates/color-transforms/src/icc/builtin.rs:103`
- `impl Curve::param_count` — `crates/color-transforms/src/icc/curve.rs:31`
- `impl Curve::is_identity` — `crates/color-transforms/src/icc/curve.rs:41`
- `impl GamutCheck::new` — `crates/color-transforms/src/icc/gamut.rs:19`
- `impl GamutCheck::inputs` — `crates/color-transforms/src/icc/gamut.rs:53`
- `impl GamutCheck::delta_e` — `crates/color-transforms/src/icc/gamut.rs:57`
- `impl GamutCheck::out_of_gamut` — `crates/color-transforms/src/icc/gamut.rs:81`
- `mul` — `crates/color-transforms/src/icc/math.rs:18`
- `apply` — `crates/color-transforms/src/icc/math.rs:28`
- `det` — `crates/color-transforms/src/icc/math.rs:36`
- `impl Stage::outputs` — `crates/color-transforms/src/icc/pipeline.rs:36`
- `impl Stage::matrix3` — `crates/color-transforms/src/icc/pipeline.rs:44`
- `impl Stage::eval` — `crates/color-transforms/src/icc/pipeline.rs:53`
- `impl Pipeline::new` — `crates/color-transforms/src/icc/pipeline.rs:137`
- `impl Pipeline::eval` — `crates/color-transforms/src/icc/pipeline.rs:145`
- `impl Pipeline::optimize` — `crates/color-transforms/src/icc/pipeline.rs:170`
- _…98 more in `modules/crates_color-transforms.md`_

### `crates/drafting-geometry` (122 undocumented public)

- `impl Bounds2::new` — `crates/drafting-geometry/src/bounds.rs:21`
- `impl Bounds2::from_points` — `crates/drafting-geometry/src/bounds.rs:24`
- `impl Bounds2::is_empty` — `crates/drafting-geometry/src/bounds.rs:31`
- `impl Bounds2::add` — `crates/drafting-geometry/src/bounds.rs:34`
- `impl Bounds2::union` — `crates/drafting-geometry/src/bounds.rs:40`
- `impl Bounds2::width` — `crates/drafting-geometry/src/bounds.rs:49`
- `impl Bounds2::height` — `crates/drafting-geometry/src/bounds.rs:52`
- `impl Bounds2::center` — `crates/drafting-geometry/src/bounds.rs:55`
- `impl Bounds2::contains` — `crates/drafting-geometry/src/bounds.rs:66`
- `impl Bounds2::intersects` — `crates/drafting-geometry/src/bounds.rs:73`
- `impl Bounds2::expand` — `crates/drafting-geometry/src/bounds.rs:76`
- `impl Bounds2::corners` — `crates/drafting-geometry/src/bounds.rs:82`
- `impl Line::new` — `crates/drafting-geometry/src/curve.rs:13`
- `impl Line::len` — `crates/drafting-geometry/src/curve.rs:16`
- `impl Line::dir` — `crates/drafting-geometry/src/curve.rs:19`
- `impl Line::mid` — `crates/drafting-geometry/src/curve.rs:22`
- `impl Line::at` — `crates/drafting-geometry/src/curve.rs:31`
- `impl Line::dist` — `crates/drafting-geometry/src/curve.rs:42`
- `impl Line::offset` — `crates/drafting-geometry/src/curve.rs:49`
- `impl Arc::new` — `crates/drafting-geometry/src/curve.rs:65`
- `impl Arc::sweep` — `crates/drafting-geometry/src/curve.rs:68`
- `impl Arc::start_point` — `crates/drafting-geometry/src/curve.rs:71`
- `impl Arc::end_point` — `crates/drafting-geometry/src/curve.rs:74`
- `impl Arc::mid_point` — `crates/drafting-geometry/src/curve.rs:77`
- `impl Arc::len` — `crates/drafting-geometry/src/curve.rs:80`
- _…97 more in `modules/crates_drafting-geometry.md`_

### `crates/vector-hybrid-renderer` (120 undocumented public)

- `impl GpuBlendInstance::new` — `crates/vector-hybrid-renderer/src/blend.rs:37`
- `impl GpuBlendInstance::copy_from_scratch` — `crates/vector-hybrid-renderer/src/blend.rs:85`
- `impl BlendStrip::from_fill_segment` — `crates/vector-hybrid-renderer/src/blend.rs:108`
- `impl OpaqueDraw::is_empty` — `crates/vector-hybrid-renderer/src/draw.rs:98`
- `impl OpaqueDraw::strips` — `crates/vector-hybrid-renderer/src/draw.rs:102`
- `impl OpaqueDraw::external_texture_runs` — `crates/vector-hybrid-renderer/src/draw.rs:106`
- `impl DrawBuilder<'a, T>::new` — `crates/vector-hybrid-renderer/src/draw.rs:132`
- `impl DrawBuilder<'a, T>::push_draw` — `crates/vector-hybrid-renderer/src/draw.rs:145`
- `impl DrawBuilder<'a, T>::push_layer_fill` — `crates/vector-hybrid-renderer/src/draw.rs:270`
- `impl DrawBuffers::clear` — `crates/vector-hybrid-renderer/src/draw.rs:362`
- `impl DrawState<T>::new` — `crates/vector-hybrid-renderer/src/draw.rs:382`
- `impl ExternalTextureBindings::as_array` — `crates/vector-hybrid-renderer/src/draw.rs:486`
- `edge_mode_to_gpu` — `crates/vector-hybrid-renderer/src/filter.rs:86`
- `impl GpuFilterData::filter_type` — `crates/vector-hybrid-renderer/src/filter.rs:326`
- `impl GpuFilterData::composite_original` — `crates/vector-hybrid-renderer/src/filter.rs:335`
- `impl GpuFilterData::needs_copy_pass` — `crates/vector-hybrid-renderer/src/filter.rs:339`
- `impl FilterPassPlan::init` — `crates/vector-hybrid-renderer/src/filter.rs:424`
- `impl FilterPassPlan::steps` — `crates/vector-hybrid-renderer/src/filter.rs:463`
- `impl FilterPassPlan::copy_pass` — `crates/vector-hybrid-renderer/src/filter.rs:467`
- `impl FilterContext::clear` — `crates/vector-hybrid-renderer/src/filter.rs:621`
- `impl FilterContext::push` — `crates/vector-hybrid-renderer/src/filter.rs:625`
- `impl FilterContext::is_empty` — `crates/vector-hybrid-renderer/src/filter.rs:634`
- `impl FilterContext::total_texels` — `crates/vector-hybrid-renderer/src/filter.rs:642`
- `impl FilterContext::serialize_to_buffer` — `crates/vector-hybrid-renderer/src/filter.rs:646`
- `impl GradientRampCache::get_or_create_ramp` — `crates/vector-hybrid-renderer/src/gradient_cache.rs:75`
- _…95 more in `modules/crates_vector-hybrid-renderer.md`_

### `crates/radiance-color-grading` (86 undocumented public)

- `impl GradeSpace::new` — `crates/radiance-color-grading/src/grade.rs:28`
- `impl GradeSpace::is_hdr` — `crates/radiance-color-grading/src/grade.rs:43`
- `impl GradeSpace::white_nits` — `crates/radiance-color-grading/src/grade.rs:47`
- `impl GradeSpace::signal_of_nits` — `crates/radiance-color-grading/src/grade.rs:97`
- `impl GradeSpace::nits_of_signal` — `crates/radiance-color-grading/src/grade.rs:102`
- `impl AscCdl::new` — `crates/radiance-color-grading/src/grade.rs:134`
- `impl LogCurve::domain` — `crates/radiance-color-grading/src/log.rs:48`
- `impl LogCurve::label` — `crates/radiance-color-grading/src/log.rs:55`
- `impl LogCurve::signal_from_normalized` — `crates/radiance-color-grading/src/log.rs:102`
- `impl LogCurve::normalized_from_signal` — `crates/radiance-color-grading/src/log.rs:111`
- `impl Lut1d::identity` — `crates/radiance-color-grading/src/lut.rs:17`
- `impl Lut1d::size` — `crates/radiance-color-grading/src/lut.rs:26`
- `impl Lut1d::apply` — `crates/radiance-color-grading/src/lut.rs:31`
- `impl Lut3d::identity` — `crates/radiance-color-grading/src/lut.rs:63`
- `impl Lut3d::from_fn` — `crates/radiance-color-grading/src/lut.rs:67`
- `impl Lut3d::at` — `crates/radiance-color-grading/src/lut.rs:87`
- `impl Lut::from_cube` — `crates/radiance-color-grading/src/lut.rs:212`
- `impl Lut::apply` — `crates/radiance-color-grading/src/lut.rs:220`
- `impl Transfer::from_code` — `crates/radiance-color-grading/src/matrix.rs:58`
- `srgb_to_linear` — `crates/radiance-color-grading/src/matrix.rs:259`
- `linear_to_srgb` — `crates/radiance-color-grading/src/matrix.rs:270`
- `pq_eotf` — `crates/radiance-color-grading/src/matrix.rs:280`
- `pq_inverse_eotf` — `crates/radiance-color-grading/src/matrix.rs:297`
- `hlg_oetf` — `crates/radiance-color-grading/src/matrix.rs:309`
- `hlg_inverse_oetf` — `crates/radiance-color-grading/src/matrix.rs:318`
- _…61 more in `modules/crates_radiance-color-grading.md`_

### `crates/audio-device-cpal` (79 undocumented public)

- `run` — `crates/audio-device-cpal/examples/beep.rs:97`
- `stream_setup_for` — `crates/audio-device-cpal/examples/synth_tones.rs:93`
- `host_device_setup` — `crates/audio-device-cpal/examples/synth_tones.rs:115`
- `make_stream` — `crates/audio-device-cpal/examples/synth_tones.rs:130`
- `impl Devices::new` — `crates/audio-device-cpal/src/host/alsa/enumerate.rs:12`
- `default_input_device` — `crates/audio-device-cpal/src/host/alsa/enumerate.rs:50`
- `default_output_device` — `crates/audio-device-cpal/src/host/alsa/enumerate.rs:58`
- `impl Host::new` — `crates/audio-device-cpal/src/host/alsa/mod.rs:32`
- `impl Device::name` — `crates/audio-device-cpal/src/host/asio/device.rs:53`
- `impl Devices::new` — `crates/audio-device-cpal/src/host/asio/device.rs:174`
- `convert_data_type` — `crates/audio-device-cpal/src/host/asio/device.rs:208`
- `impl Host::new` — `crates/audio-device-cpal/src/host/asio/mod.rs:25`
- `impl Stream::play` — `crates/audio-device-cpal/src/host/asio/stream.rs:24`
- `impl Stream::pause` — `crates/audio-device-cpal/src/host/asio/stream.rs:29`
- `impl Device::build_input_stream_raw` — `crates/audio-device-cpal/src/host/asio/stream.rs:36`
- `impl Device::build_output_stream_raw` — `crates/audio-device-cpal/src/host/asio/stream.rs:241`
- `impl Devices::new` — `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:15`
- `default_input_device` — `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:36`
- `default_output_device` — `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:41`
- `impl Host::new` — `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:50`
- `impl Devices::new` — `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:63`
- `default_input_device` — `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:91`
- `default_output_device` — `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:121`
- `impl Host::new` — `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:57`
- `impl Host::new` — `crates/audio-device-cpal/src/host/emscripten/mod.rs:55`
- _…54 more in `modules/crates_audio-device-cpal.md`_

### `crates/chronicle-broadcast-containers` (79 undocumented public)

- `impl Reader<'a>::new` — `crates/chronicle-broadcast-containers/src/dnx/bits.rs:15`
- `impl Reader<'a>::consumed` — `crates/chronicle-broadcast-containers/src/dnx/bits.rs:53`
- `impl Reader<'a>::overrun` — `crates/chronicle-broadcast-containers/src/dnx/bits.rs:58`
- `impl Reader<'a>::read` — `crates/chronicle-broadcast-containers/src/dnx/bits.rs:63`
- `impl Reader<'a>::vlc` — `crates/chronicle-broadcast-containers/src/dnx/bits.rs:74`
- `impl Writer::new` — `crates/chronicle-broadcast-containers/src/dnx/bits.rs:97`
- `impl Writer::write` — `crates/chronicle-broadcast-containers/src/dnx/bits.rs:102`
- `impl Writer::flush` — `crates/chronicle-broadcast-containers/src/dnx/bits.rs:116`
- `impl Frame::new` — `crates/chronicle-broadcast-containers/src/dnx/decode.rs:29`
- `impl Frame::chroma_width` — `crates/chronicle-broadcast-containers/src/dnx/decode.rs:63`
- `impl Frame::chroma_height` — `crates/chronicle-broadcast-containers/src/dnx/decode.rs:70`
- `probe` — `crates/chronicle-broadcast-containers/src/dnx/decode.rs:83`
- `decode_frame` — `crates/chronicle-broadcast-containers/src/dnx/decode.rs:87`
- `decode_frame_with` — `crates/chronicle-broadcast-containers/src/dnx/decode.rs:91`
- `impl FrameHeader::rgb_planes` — `crates/chronicle-broadcast-containers/src/dnx/header.rs:56`
- `impl FrameHeader::mb_width` — `crates/chronicle-broadcast-containers/src/dnx/header.rs:60`
- `impl FrameHeader::coding_unit_size` — `crates/chronicle-broadcast-containers/src/dnx/header.rs:64`
- `parse_frame_header` — `crates/chronicle-broadcast-containers/src/dnx/header.rs:99`
- `header_size_for` — `crates/chronicle-broadcast-containers/src/dnx/header.rs:206`
- `write_frame_header` — `crates/chronicle-broadcast-containers/src/dnx/header.rs:214`
- `cid_info` — `crates/chronicle-broadcast-containers/src/dnx/tables.rs:120`
- `ri_frame_size` — `crates/chronicle-broadcast-containers/src/dnx/tables.rs:124`
- `vlc` — `crates/chronicle-broadcast-containers/src/dnx/tables.rs:260`
- `demux_matroska` — `crates/chronicle-broadcast-containers/src/matroska/demux.rs:35`
- `vint_len` — `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:4`
- _…54 more in `modules/crates_chronicle-broadcast-containers.md`_

### `crates/media-isobmff` (69 undocumented public)

- `impl FourCc::new` — `crates/media-isobmff/src/bytes.rs:12`
- `impl FourCc::as_u32` — `crates/media-isobmff/src/bytes.rs:15`
- `impl Cur<'a>::new` — `crates/media-isobmff/src/bytes.rs:71`
- `impl Cur<'a>::remaining` — `crates/media-isobmff/src/bytes.rs:74`
- `impl Cur<'a>::rest` — `crates/media-isobmff/src/bytes.rs:77`
- `impl Cur<'a>::bytes` — `crates/media-isobmff/src/bytes.rs:80`
- `impl Cur<'a>::skip` — `crates/media-isobmff/src/bytes.rs:86`
- `impl Cur<'a>::array` — `crates/media-isobmff/src/bytes.rs:89`
- `impl Cur<'a>::u8` — `crates/media-isobmff/src/bytes.rs:95`
- `impl Cur<'a>::u16` — `crates/media-isobmff/src/bytes.rs:98`
- `impl Cur<'a>::i16` — `crates/media-isobmff/src/bytes.rs:101`
- `impl Cur<'a>::u24` — `crates/media-isobmff/src/bytes.rs:104`
- `impl Cur<'a>::u32` — `crates/media-isobmff/src/bytes.rs:108`
- `impl Cur<'a>::i32` — `crates/media-isobmff/src/bytes.rs:111`
- `impl Cur<'a>::u64` — `crates/media-isobmff/src/bytes.rs:114`
- `impl Cur<'a>::i64` — `crates/media-isobmff/src/bytes.rs:117`
- `impl Cur<'a>::f64` — `crates/media-isobmff/src/bytes.rs:120`
- `impl Cur<'a>::fourcc` — `crates/media-isobmff/src/bytes.rs:123`
- `impl Cur<'a>::check_count` — `crates/media-isobmff/src/bytes.rs:137`
- `boxes` — `crates/media-isobmff/src/bytes.rs:201`
- `impl BoxBuf::new` — `crates/media-isobmff/src/bytes.rs:255`
- `impl BoxBuf::start_full` — `crates/media-isobmff/src/bytes.rs:265`
- `impl BoxBuf::end` — `crates/media-isobmff/src/bytes.rs:270`
- `impl BoxBuf::u8` — `crates/media-isobmff/src/bytes.rs:280`
- `impl BoxBuf::u16` — `crates/media-isobmff/src/bytes.rs:283`
- _…44 more in `modules/crates_media-isobmff.md`_

### `crates/kinetics-block-simulation` (65 undocumented public)

- `impl Param::as_num` — `crates/kinetics-block-simulation/src/block.rs:23`
- `impl Param::as_vec` — `crates/kinetics-block-simulation/src/block.rs:30`
- `impl Param::as_str` — `crates/kinetics-block-simulation/src/block.rs:38`
- `impl Param::as_bool` — `crates/kinetics-block-simulation/src/block.rs:45`
- `impl Integrator::new` — `crates/kinetics-block-simulation/src/blocks/continuous.rs:15`
- `impl Derivative::new` — `crates/kinetics-block-simulation/src/blocks/continuous.rs:81`
- `impl PidController::new` — `crates/kinetics-block-simulation/src/blocks/continuous.rs:141`
- `impl TransferFcn::new` — `crates/kinetics-block-simulation/src/blocks/continuous.rs:217`
- `impl TransportDelay::new` — `crates/kinetics-block-simulation/src/blocks/continuous.rs:311`
- `impl UnitDelay::new` — `crates/kinetics-block-simulation/src/blocks/discrete.rs:16`
- `impl ZeroOrderHold::new` — `crates/kinetics-block-simulation/src/blocks/discrete.rs:74`
- `impl Gain::new` — `crates/kinetics-block-simulation/src/blocks/math.rs:15`
- `impl Sum::new` — `crates/kinetics-block-simulation/src/blocks/math.rs:60`
- `impl Product::new` — `crates/kinetics-block-simulation/src/blocks/math.rs:122`
- `impl Saturation::new` — `crates/kinetics-block-simulation/src/blocks/math.rs:185`
- `impl DeadZone::new` — `crates/kinetics-block-simulation/src/blocks/math.rs:231`
- `impl Switch::new` — `crates/kinetics-block-simulation/src/blocks/math.rs:282`
- `impl Relational::new` — `crates/kinetics-block-simulation/src/blocks/math.rs:332`
- `impl LogicalOperator::new` — `crates/kinetics-block-simulation/src/blocks/math.rs:386`
- `impl Abs::new` — `crates/kinetics-block-simulation/src/blocks/math.rs:452`
- `check_keys` — `crates/kinetics-block-simulation/src/blocks/params.rs:6`
- `num` — `crates/kinetics-block-simulation/src/blocks/params.rs:15`
- `vec` — `crates/kinetics-block-simulation/src/blocks/params.rs:25`
- `req_vec` — `crates/kinetics-block-simulation/src/blocks/params.rs:35`
- `string` — `crates/kinetics-block-simulation/src/blocks/params.rs:45`
- _…40 more in `modules/crates_kinetics-block-simulation.md`_

### `crates/kinetics-schedule-cpm` (61 undocumented public)

- `impl SyncRole::as_str` — `crates/kinetics-schedule-cpm/src/change_dag.rs:35`
- `impl SyncRole::parse` — `crates/kinetics-schedule-cpm/src/change_dag.rs:45`
- `impl SyncRole::can_read` — `crates/kinetics-schedule-cpm/src/change_dag.rs:55`
- `impl SyncRole::can_comment` — `crates/kinetics-schedule-cpm/src/change_dag.rs:59`
- `impl SyncRole::can_write_tasks` — `crates/kinetics-schedule-cpm/src/change_dag.rs:63`
- `impl SyncRole::can_manage_roles` — `crates/kinetics-schedule-cpm/src/change_dag.rs:67`
- `impl SyncRole::allows_command` — `crates/kinetics-schedule-cpm/src/change_dag.rs:71`
- `impl ChangeEnvelope::new` — `crates/kinetics-schedule-cpm/src/change_dag.rs:150`
- `impl ChangeDag::new` — `crates/kinetics-schedule-cpm/src/change_dag.rs:188`
- `impl ChangeDag::record_command` — `crates/kinetics-schedule-cpm/src/change_dag.rs:192`
- `impl ChangeDag::add_comment` — `crates/kinetics-schedule-cpm/src/change_dag.rs:216`
- `impl ChangeDag::merge_remote` — `crates/kinetics-schedule-cpm/src/change_dag.rs:250`
- `impl ChangeDag::inspect_post_merge_conflicts` — `crates/kinetics-schedule-cpm/src/change_dag.rs:269`
- `to_json` — `crates/kinetics-schedule-cpm/src/format.rs:47`
- `from_json` — `crates/kinetics-schedule-cpm/src/format.rs:51`
- `to_csv` — `crates/kinetics-schedule-cpm/src/format.rs:62`
- `from_csv` — `crates/kinetics-schedule-cpm/src/format.rs:106`
- `to_jira_json` — `crates/kinetics-schedule-cpm/src/format.rs:242`
- `to_mspdi_xml` — `crates/kinetics-schedule-cpm/src/format.rs:292`
- `from_mspdi_xml` — `crates/kinetics-schedule-cpm/src/format.rs:356`
- `to_primavera_xer` — `crates/kinetics-schedule-cpm/src/format.rs:491`
- `from_primavera_xer` — `crates/kinetics-schedule-cpm/src/format.rs:556`
- `render_gantt_svg` — `crates/kinetics-schedule-cpm/src/format.rs:672`
- `impl ItemKind::as_str` — `crates/kinetics-schedule-cpm/src/model.rs:43`
- `impl ItemKind::parse` — `crates/kinetics-schedule-cpm/src/model.rs:54`
- _…36 more in `modules/crates_kinetics-schedule-cpm.md`_

### `crates/surface-pbr-studio` (60 undocumented public)

- `impl SurfaceAttributes::build` — `crates/surface-pbr-studio/src/bake.rs:55`
- `impl Bvh::bounds` — `crates/surface-pbr-studio/src/bvh.rs:50`
- `impl Bvh::node_count` — `crates/surface-pbr-studio/src/bvh.rs:54`
- `impl Bvh::is_empty` — `crates/surface-pbr-studio/src/bvh.rs:58`
- `impl BlendMode::blend` — `crates/surface-pbr-studio/src/compose.rs:21`
- `impl PbrLayer::new_fill` — `crates/surface-pbr-studio/src/compose.rs:58`
- `impl PbrLayerStack::new` — `crates/surface-pbr-studio/src/compose.rs:84`
- `impl PbrLayerStack::add_layer` — `crates/surface-pbr-studio/src/compose.rs:88`
- `impl PbrChannel::id` — `crates/surface-pbr-studio/src/material.rs:33`
- `impl PbrChannel::label` — `crates/surface-pbr-studio/src/material.rs:47`
- `impl PbrChannel::default_value` — `crates/surface-pbr-studio/src/material.rs:61`
- `impl MaterialPreset::metal` — `crates/surface-pbr-studio/src/material.rs:90`
- `impl MaterialPreset::dielectric` — `crates/surface-pbr-studio/src/material.rs:103`
- `impl MaterialPreset::glass` — `crates/surface-pbr-studio/src/material.rs:116`
- `impl MaterialPreset::car_paint` — `crates/surface-pbr-studio/src/material.rs:129`
- `impl MaterialPreset::skin` — `crates/surface-pbr-studio/src/material.rs:142`
- `impl MaterialPreset::fabric` — `crates/surface-pbr-studio/src/material.rs:155`
- `get_preset_by_name` — `crates/surface-pbr-studio/src/material.rs:190`
- `impl Vec2::new` — `crates/surface-pbr-studio/src/math.rs:19`
- `impl Vec2::dot` — `crates/surface-pbr-studio/src/math.rs:24`
- `impl Vec2::length_squared` — `crates/surface-pbr-studio/src/math.rs:29`
- `impl Vec2::length` — `crates/surface-pbr-studio/src/math.rs:34`
- `impl Vec2::distance` — `crates/surface-pbr-studio/src/math.rs:39`
- `impl Vec2::lerp` — `crates/surface-pbr-studio/src/math.rs:44`
- `impl Vec3::new` — `crates/surface-pbr-studio/src/math.rs:100`
- _…35 more in `modules/crates_surface-pbr-studio.md`_

### `crates/terrain-brushes` (49 undocumented public)

- `impl CowTerrainTileGrid::new` — `crates/terrain-brushes/adaptation/grid-impl.rs:10`
- `impl CowTerrainTileGrid::tile_size` — `crates/terrain-brushes/adaptation/grid-impl.rs:25`
- `impl CowTerrainTileGrid::cell_size_m` — `crates/terrain-brushes/adaptation/grid-impl.rs:26`
- `impl CowTerrainTileGrid::byte_budget` — `crates/terrain-brushes/adaptation/grid-impl.rs:27`
- `impl CowTerrainTileGrid::is_stroke_active` — `crates/terrain-brushes/adaptation/grid-impl.rs:28`
- `impl CowTerrainTileGrid::tiles` — `crates/terrain-brushes/adaptation/grid-impl.rs:29`
- `impl CowTerrainTileGrid::undo_history` — `crates/terrain-brushes/adaptation/grid-impl.rs:30`
- `impl CowTerrainTileGrid::redo_history` — `crates/terrain-brushes/adaptation/grid-impl.rs:31`
- `impl CowTerrainTileGrid::active_samples` — `crates/terrain-brushes/adaptation/grid-impl.rs:32`
- `impl CowTerrainTileGrid::ensure_tile` — `crates/terrain-brushes/adaptation/grid-impl.rs:49`
- `impl CowTerrainTileGrid::begin_stroke` — `crates/terrain-brushes/adaptation/grid-impl.rs:85`
- `impl CowTerrainTileGrid::undo` — `crates/terrain-brushes/adaptation/grid-impl.rs:228`
- `impl CowTerrainTileGrid::redo` — `crates/terrain-brushes/adaptation/grid-impl.rs:239`
- `impl CowTerrainTileGrid::replay_stroke_journal` — `crates/terrain-brushes/adaptation/grid-impl.rs:250`
- `impl CowTerrainTileGrid::sample_height` — `crates/terrain-brushes/adaptation/grid-impl.rs:265`
- `impl CowTerrainTileGrid::tile_arc` — `crates/terrain-brushes/adaptation/grid-impl.rs:270`
- `impl BrushFalloff::from_code` — `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:49`
- `impl BrushFalloff::compute_weight` — `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:59`
- `impl BrushOperation::from_sample` — `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:93`
- `impl CowTerrainTileGrid::new` — `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:158`
- `impl CowTerrainTileGrid::ensure_tile` — `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:170`
- `impl CowTerrainTileGrid::begin_stroke` — `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:182`
- `impl CowTerrainTileGrid::apply_stroke_sample` — `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:190`
- `impl CowTerrainTileGrid::commit_stroke` — `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:245`
- `impl CowTerrainTileGrid::undo` — `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:274`
- _…24 more in `modules/crates_terrain-brushes.md`_

### `crates/acoustics-daw-session` (45 undocumented public)

- `impl AudioBuffer::new` — `crates/acoustics-daw-session/src/codecs/mod.rs:13`
- `impl AudioBuffer::num_channels` — `crates/acoustics-daw-session/src/codecs/mod.rs:20`
- `impl AudioBuffer::num_frames` — `crates/acoustics-daw-session/src/codecs/mod.rs:24`
- `impl PeakFile::build` — `crates/acoustics-daw-session/src/codecs/mod.rs:260`
- `impl BiquadFilter::new` — `crates/acoustics-daw-session/src/dsp/biquad.rs:152`
- `impl BiquadFilter::reset` — `crates/acoustics-daw-session/src/dsp/biquad.rs:160`
- `impl BiquadFilter::process_sample` — `crates/acoustics-daw-session/src/dsp/biquad.rs:166`
- `impl BiquadFilter::process_buffer` — `crates/acoustics-daw-session/src/dsp/biquad.rs:180`
- `impl KWeightingChannel::new` — `crates/acoustics-daw-session/src/dsp/meter.rs:30`
- `impl KWeightingChannel::filter_sample` — `crates/acoustics-daw-session/src/dsp/meter.rs:38`
- `impl EbuR128Meter::new` — `crates/acoustics-daw-session/src/dsp/meter.rs:57`
- `impl TruePeakMeter::new` — `crates/acoustics-daw-session/src/dsp/meter.rs:145`
- `impl TruePeakMeter::true_peak_db` — `crates/acoustics-daw-session/src/dsp/meter.rs:168`
- `impl Oscillator::new` — `crates/acoustics-daw-session/src/dsp/osc.rs:32`
- `impl Oscillator::set_frequency` — `crates/acoustics-daw-session/src/dsp/osc.rs:42`
- `impl SpeakerPosition::new` — `crates/acoustics-daw-session/src/dsp/pan.rs:45`
- `impl VbapPanner<'a>::new` — `crates/acoustics-daw-session/src/dsp/pan.rs:90`
- `impl Eq7::new` — `crates/acoustics-daw-session/src/dsp/plugins.rs:14`
- `impl Eq7::process_sample` — `crates/acoustics-daw-session/src/dsp/plugins.rs:28`
- `impl Compressor::new` — `crates/acoustics-daw-session/src/dsp/plugins.rs:49`
- `impl Compressor::process_sample` — `crates/acoustics-daw-session/src/dsp/plugins.rs:62`
- `impl ExpanderGate::new` — `crates/acoustics-daw-session/src/dsp/plugins.rs:99`
- `impl ExpanderGate::process_sample` — `crates/acoustics-daw-session/src/dsp/plugins.rs:111`
- `impl Maximizer::new` — `crates/acoustics-daw-session/src/dsp/plugins.rs:148`
- `impl Maximizer::process_sample` — `crates/acoustics-daw-session/src/dsp/plugins.rs:162`
- _…20 more in `modules/crates_acoustics-daw-session.md`_

### `crates/multimodal-synthesis-schema` (45 undocumented public)

- `impl MediaKind::label` — `crates/multimodal-synthesis-schema/src/catalog.rs:49`
- `impl AudioCaps::has_tuning` — `crates/multimodal-synthesis-schema/src/catalog.rs:121`
- `impl AudioCaps::sample_rate` — `crates/multimodal-synthesis-schema/src/catalog.rs:125`
- `impl ModelInfo::batch_options` — `crates/multimodal-synthesis-schema/src/catalog.rs:134`
- `impl ModelInfo::valid_batch` — `crates/multimodal-synthesis-schema/src/catalog.rs:142`
- `impl ModelInfo::resolve` — `crates/multimodal-synthesis-schema/src/catalog.rs:153`
- `impl ModelInfo::supports_reference_mode` — `crates/multimodal-synthesis-schema/src/catalog.rs:163`
- `impl ModelInfo::duration_range` — `crates/multimodal-synthesis-schema/src/catalog.rs:167`
- `impl ModelInfo::resolve_duration` — `crates/multimodal-synthesis-schema/src/catalog.rs:183`
- `impl Catalog::set_image` — `crates/multimodal-synthesis-schema/src/catalog.rs:228`
- `impl Catalog::set_video` — `crates/multimodal-synthesis-schema/src/catalog.rs:233`
- `impl Catalog::set_audio` — `crates/multimodal-synthesis-schema/src/catalog.rs:238`
- `impl Catalog::find` — `crates/multimodal-synthesis-schema/src/catalog.rs:243`
- `impl Catalog::display_name` — `crates/multimodal-synthesis-schema/src/catalog.rs:251`
- `impl Catalog::creator` — `crates/multimodal-synthesis-schema/src/catalog.rs:256`
- `display_name` — `crates/multimodal-synthesis-schema/src/catalog.rs:262`
- `default_prompt_max` — `crates/multimodal-synthesis-schema/src/catalog.rs:313`
- `description` — `crates/multimodal-synthesis-schema/src/catalog.rs:337`
- `expected_secs` — `crates/multimodal-synthesis-schema/src/catalog.rs:365`
- `creator_for` — `crates/multimodal-synthesis-schema/src/catalog.rs:381`
- `family` — `crates/multimodal-synthesis-schema/src/catalog.rs:408`
- `family_rank` — `crates/multimodal-synthesis-schema/src/catalog.rs:440`
- `grouped` — `crates/multimodal-synthesis-schema/src/catalog.rs:472`
- `aspect_label` — `crates/multimodal-synthesis-schema/src/catalog.rs:511`
- `aspect_proportion` — `crates/multimodal-synthesis-schema/src/catalog.rs:536`
- _…20 more in `modules/crates_multimodal-synthesis-schema.md`_

### `crates/video-h264-codec` (44 undocumented public)

- `impl Cabac<'a>::init_contexts` — `crates/video-h264-codec/src/cabac.rs:88`
- `impl Cabac<'a>::data` — `crates/video-h264-codec/src/cabac.rs:138`
- `impl Cabac<'a>::decode_bypass` — `crates/video-h264-codec/src/cabac.rs:179`
- `impl Cabac<'a>::decode_terminate` — `crates/video-h264-codec/src/cabac.rs:190`
- `impl VlcTable::build` — `crates/video-h264-codec/src/cavlc.rs:15`
- `impl VlcTable::decode` — `crates/video-h264-codec/src/cavlc.rs:46`
- `impl Decoder::draft` — `crates/video-h264-codec/src/decoder.rs:178`
- `impl Dpb<F>::new` — `crates/video-h264-codec/src/dpb.rs:61`
- `invalid` — `crates/video-h264-codec/src/error.rs:22`
- `unsupported` — `crates/video-h264-codec/src/error.rs:26`
- `impl Part::num_parts` — `crates/video-h264-codec/src/mbtypes.rs:20`
- `impl SubPart::num_parts` — `crates/video-h264-codec/src/mbtypes.rs:49`
- `p_sub_mb_type` — `crates/video-h264-codec/src/mbtypes.rs:159`
- `b_sub_mb_type` — `crates/video-h264-codec/src/mbtypes.rs:170`
- `impl ScalingMatrices::flat` — `crates/video-h264-codec/src/params.rs:22`
- `impl ScalingMatrices::is_flat` — `crates/video-h264-codec/src/params.rs:25`
- `impl Sps::parse` — `crates/video-h264-codec/src/params.rs:341`
- `impl Sps::frame_height_in_mbs` — `crates/video-h264-codec/src/params.rs:450`
- `impl Sps::height` — `crates/video-h264-codec/src/params.rs:457`
- `impl Sps::max_frame_num` — `crates/video-h264-codec/src/params.rs:460`
- `impl Sps::check_supported` — `crates/video-h264-codec/src/params.rs:527`
- `impl Planes::new` — `crates/video-h264-codec/src/picture.rs:17`
- `impl Planes::gray` — `crates/video-h264-codec/src/picture.rs:21`
- `impl MbKind::is_intra` — `crates/video-h264-codec/src/picture.rs:46`
- `impl MbState::kind_is_skip` — `crates/video-h264-codec/src/picture.rs:85`
- _…19 more in `modules/crates_video-h264-codec.md`_

### `crates/audio-opus-codec` (43 undocumented public)

- `lcg_rand` — `crates/audio-opus-codec/src/celt/bands.rs:12`
- `renormalise_vector` — `crates/audio-opus-codec/src/celt/bands.rs:126`
- `impl BandCtx<'_, '_, '_>::quant_band` — `crates/audio-opus-codec/src/celt/bands.rs:469`
- `impl BandCtx<'_, '_, '_>::quant_band_stereo` — `crates/audio-opus-codec/src/celt/bands.rs:551`
- `unquant_fine_energy` — `crates/audio-opus-codec/src/celt/energy.rs:77`
- `unquant_energy_finalise` — `crates/audio-opus-codec/src/celt/energy.rs:92`
- `impl Fft::new` — `crates/audio-opus-codec/src/celt/mdct.rs:32`
- `impl Fft::process` — `crates/audio-opus-codec/src/celt/mdct.rs:51`
- `impl Imdct::new` — `crates/audio-opus-codec/src/celt/mdct.rs:96`
- `mode` — `crates/audio-opus-codec/src/celt/mod.rs:24`
- `impl Mode::new` — `crates/audio-opus-codec/src/celt/rate.rs:90`
- `impl Mode::cache` — `crates/audio-opus-codec/src/celt/rate.rs:114`
- `impl Mode::bits2pulses` — `crates/audio-opus-codec/src/celt/rate.rs:119`
- `impl Mode::pulses2bits` — `crates/audio-opus-codec/src/celt/rate.rs:136`
- `impl StreamDecoder::sample_rate` — `crates/audio-opus-codec/src/decoder.rs:58`
- `impl StreamDecoder::channels` — `crates/audio-opus-codec/src/decoder.rs:62`
- `impl Decoder::head` — `crates/audio-opus-codec/src/multistream.rs:45`
- `impl Decoder::sample_rate` — `crates/audio-opus-codec/src/multistream.rs:49`
- `impl Decoder::channels` — `crates/audio-opus-codec/src/multistream.rs:53`
- `impl Decoder::reset` — `crates/audio-opus-codec/src/multistream.rs:80`
- `impl Toc::parse` — `crates/audio-opus-codec/src/packet.rs:52`
- `impl Toc::channels` — `crates/audio-opus-codec/src/packet.rs:93`
- `impl RangeDecoder<'a>::new` — `crates/audio-opus-codec/src/range.rs:42`
- `impl RangeDecoder<'a>::rng` — `crates/audio-opus-codec/src/range.rs:72`
- `impl RangeEncoder::new` — `crates/audio-opus-codec/src/range.rs:282`
- _…18 more in `modules/crates_audio-opus-codec.md`_

### `crates/discourse-scientific-matlang` (42 undocumented public)

- `impl Value::as_matrix` — `crates/discourse-scientific-matlang/src/interpreter/eval.rs:17`
- `impl Value::as_num` — `crates/discourse-scientific-matlang/src/interpreter/eval.rs:26`
- `impl Value::is_truthy` — `crates/discourse-scientific-matlang/src/interpreter/eval.rs:35`
- `impl Workspace::new` — `crates/discourse-scientific-matlang/src/interpreter/eval.rs:57`
- `impl Workspace::exec` — `crates/discourse-scientific-matlang/src/interpreter/eval.rs:67`
- `impl Workspace::eval_expr` — `crates/discourse-scientific-matlang/src/interpreter/eval.rs:118`
- `lex` — `crates/discourse-scientific-matlang/src/interpreter/lexer.rs:54`
- `eval_m_code` — `crates/discourse-scientific-matlang/src/interpreter/mod.rs:12`
- `impl Parser::new` — `crates/discourse-scientific-matlang/src/interpreter/parser.rs:12`
- `impl Parser::parse` — `crates/discourse-scientific-matlang/src/interpreter/parser.rs:36`
- `impl Matrix::zeros` — `crates/discourse-scientific-matlang/src/matrix.rs:45`
- `impl Matrix::ones` — `crates/discourse-scientific-matlang/src/matrix.rs:53`
- `impl Matrix::eye` — `crates/discourse-scientific-matlang/src/matrix.rs:61`
- `impl Matrix::scalar` — `crates/discourse-scientific-matlang/src/matrix.rs:69`
- `impl Matrix::row_vector` — `crates/discourse-scientific-matlang/src/matrix.rs:77`
- `impl Matrix::col_vector` — `crates/discourse-scientific-matlang/src/matrix.rs:85`
- `impl Matrix::from_col_major` — `crates/discourse-scientific-matlang/src/matrix.rs:94`
- `impl Matrix::from_row_major` — `crates/discourse-scientific-matlang/src/matrix.rs:101`
- `impl Matrix::get` — `crates/discourse-scientific-matlang/src/matrix.rs:115`
- `impl Matrix::map` — `crates/discourse-scientific-matlang/src/matrix.rs:123`
- `impl Matrix::set` — `crates/discourse-scientific-matlang/src/matrix.rs:132`
- `impl Matrix::shape` — `crates/discourse-scientific-matlang/src/matrix.rs:141`
- `impl Matrix::numel` — `crates/discourse-scientific-matlang/src/matrix.rs:145`
- `impl Matrix::is_scalar` — `crates/discourse-scientific-matlang/src/matrix.rs:149`
- `impl Matrix::as_scalar` — `crates/discourse-scientific-matlang/src/matrix.rs:153`
- _…17 more in `modules/crates_discourse-scientific-matlang.md`_

### `crates/manifold-bim-csg` (41 undocumented public)

- `add` — `crates/manifold-bim-csg/src/csg.rs:10`
- `sub` — `crates/manifold-bim-csg/src/csg.rs:15`
- `mul` — `crates/manifold-bim-csg/src/csg.rs:20`
- `dot` — `crates/manifold-bim-csg/src/csg.rs:25`
- `cross` — `crates/manifold-bim-csg/src/csg.rs:30`
- `impl Meter::new` — `crates/manifold-bim-csg/src/csg.rs:64`
- `impl Poly::of` — `crates/manifold-bim-csg/src/csg.rs:88`
- `impl Poly::flip` — `crates/manifold-bim-csg/src/csg.rs:101`
- `impl Poly::triangles` — `crates/manifold-bim-csg/src/csg.rs:107`
- `impl Bsp::of` — `crates/manifold-bim-csg/src/csg.rs:197`
- `impl Bsp::build` — `crates/manifold-bim-csg/src/csg.rs:213`
- `impl Bsp::clip` — `crates/manifold-bim-csg/src/csg.rs:268`
- `impl Bsp::clip_to` — `crates/manifold-bim-csg/src/csg.rs:302`
- `impl Bsp::invert` — `crates/manifold-bim-csg/src/csg.rs:309`
- `impl Bsp::polygons` — `crates/manifold-bim-csg/src/csg.rs:318`
- `bounds` — `crates/manifold-bim-csg/src/csg.rs:323`
- `cuboid` — `crates/manifold-bim-csg/src/csg.rs:428`
- `impl IfcWriter::new` — `crates/manifold-bim-csg/src/ifc.rs:54`
- `impl IfcWriter::next_id` — `crates/manifold-bim-csg/src/ifc.rs:61`
- `impl IfcWriter::write_entity` — `crates/manifold-bim-csg/src/ifc.rs:67`
- `dist_2d` — `crates/manifold-bim-csg/src/parametric.rs:16`
- `sub_2d` — `crates/manifold-bim-csg/src/parametric.rs:21`
- `add_2d` — `crates/manifold-bim-csg/src/parametric.rs:26`
- `mul_2d` — `crates/manifold-bim-csg/src/parametric.rs:31`
- `unit_2d` — `crates/manifold-bim-csg/src/parametric.rs:36`
- _…16 more in `modules/crates_manifold-bim-csg.md`_

### `crates/chronicle-timeline-interchange` (40 undocumented public)

- `decode_pair` — `crates/chronicle-timeline-interchange/src/captions/cea608.rs:60`
- `impl ScreenBuffer::clear` — `crates/chronicle-timeline-interchange/src/captions/cea608.rs:153`
- `impl ScreenBuffer::write_char` — `crates/chronicle-timeline-interchange/src/captions/cea608.rs:159`
- `impl ScreenBuffer::carriage_return` — `crates/chronicle-timeline-interchange/src/captions/cea608.rs:166`
- `impl ScreenBuffer::render_text` — `crates/chronicle-timeline-interchange/src/captions/cea608.rs:175`
- `parse_packet_header` — `crates/chronicle-timeline-interchange/src/captions/cea708.rs:12`
- `impl DtvccWindow::new` — `crates/chronicle-timeline-interchange/src/captions/cea708.rs:54`
- `impl DtvccWindow::clear` — `crates/chronicle-timeline-interchange/src/captions/cea708.rs:64`
- `impl DtvccWindow::append` — `crates/chronicle-timeline-interchange/src/captions/cea708.rs:68`
- `parse_mcc` — `crates/chronicle-timeline-interchange/src/captions/mcc.rs:10`
- `write_mcc` — `crates/chronicle-timeline-interchange/src/captions/mcc.rs:133`
- `detect_caption_format` — `crates/chronicle-timeline-interchange/src/captions/mod.rs:44`
- `parse_srt` — `crates/chronicle-timeline-interchange/src/captions/srt.rs:10`
- `write_srt` — `crates/chronicle-timeline-interchange/src/captions/srt.rs:72`
- `parse_vtt` — `crates/chronicle-timeline-interchange/src/captions/vtt.rs:10`
- `write_vtt` — `crates/chronicle-timeline-interchange/src/captions/vtt.rs:93`
- `impl TimelineTrack::new` — `crates/chronicle-timeline-interchange/src/model.rs:67`
- `impl Tick::from_rational` — `crates/chronicle-timeline-interchange/src/time.rs:17`
- `impl Tick::to_rational` — `crates/chronicle-timeline-interchange/src/time.rs:25`
- `impl Tick::from_seconds` — `crates/chronicle-timeline-interchange/src/time.rs:34`
- `impl Tick::as_seconds` — `crates/chronicle-timeline-interchange/src/time.rs:38`
- `impl Tick::from_millis` — `crates/chronicle-timeline-interchange/src/time.rs:42`
- `impl Tick::as_millis` — `crates/chronicle-timeline-interchange/src/time.rs:46`
- `impl FrameRate::as_f64` — `crates/chronicle-timeline-interchange/src/time.rs:87`
- `impl FrameRate::frame_at` — `crates/chronicle-timeline-interchange/src/time.rs:91`
- _…15 more in `modules/crates_chronicle-timeline-interchange.md`_

### `crates/layout-presentation-deck` (38 undocumented public)

- `impl Layout::new` — `crates/layout-presentation-deck/src/model/master.rs:36`
- `impl Master::new` — `crates/layout-presentation-deck/src/model/master.rs:56`
- `impl Slide::new` — `crates/layout-presentation-deck/src/model/master.rs:78`
- `impl Slide::shape_by_id` — `crates/layout-presentation-deck/src/model/master.rs:89`
- `impl Slide::shape_by_id_mut` — `crates/layout-presentation-deck/src/model/master.rs:93`
- `impl Presentation::master_for_slide` — `crates/layout-presentation-deck/src/model/master.rs:135`
- `impl Ctx<'a>::for_slide` — `crates/layout-presentation-deck/src/model/resolve.rs:25`
- `impl Ctx<'a>::for_layout` — `crates/layout-presentation-deck/src/model/resolve.rs:36`
- `impl Ctx<'a>::for_master` — `crates/layout-presentation-deck/src/model/resolve.rs:46`
- `compatible` — `crates/layout-presentation-deck/src/model/resolve.rs:57`
- `find_ph` — `crates/layout-presentation-deck/src/model/resolve.rs:67`
- `master_ph` — `crates/layout-presentation-deck/src/model/resolve.rs:84`
- `impl Run::new` — `crates/layout-presentation-deck/src/model/shape.rs:30`
- `impl Paragraph::new` — `crates/layout-presentation-deck/src/model/shape.rs:64`
- `impl Paragraph::text` — `crates/layout-presentation-deck/src/model/shape.rs:71`
- `impl TextBody::text` — `crates/layout-presentation-deck/src/model/shape.rs:83`
- `impl Shape::new` — `crates/layout-presentation-deck/src/model/shape.rs:171`
- `impl Shape::with_text` — `crates/layout-presentation-deck/src/model/shape.rs:185`
- `impl Shape::text` — `crates/layout-presentation-deck/src/model/shape.rs:209`
- `impl Shape::ph_type` — `crates/layout-presentation-deck/src/model/shape.rs:216`
- `impl PhType::is_title` — `crates/layout-presentation-deck/src/model/types.rs:41`
- `impl Xfrm::new` — `crates/layout-presentation-deck/src/model/types.rs:58`
- `impl Rgba::new` — `crates/layout-presentation-deck/src/model/types.rs:94`
- `impl ColorScheme::get` — `crates/layout-presentation-deck/src/model/types.rs:154`
- `impl ColorRef::resolve` — `crates/layout-presentation-deck/src/model/types.rs:179`
- _…13 more in `modules/crates_layout-presentation-deck.md`_

### `crates/layout-paged-typesetter` (34 undocumented public)

- `impl Citation::format` — `crates/layout-paged-typesetter/src/citations/csl.rs:39`
- `format_full_entry` — `crates/layout-paged-typesetter/src/citations/csl.rs:73`
- `impl Bibliography::generate` — `crates/layout-paged-typesetter/src/citations/csl.rs:106`
- `zotero_code` — `crates/layout-paged-typesetter/src/citations/zotero_fields.rs:13`
- `stored_code` — `crates/layout-paged-typesetter/src/citations/zotero_fields.rs:19`
- `format_csl_citation_field` — `crates/layout-paged-typesetter/src/citations/zotero_fields.rs:23`
- `impl Math::new` — `crates/layout-paged-typesetter/src/equation/ast.rs:17`
- `impl Math::is_empty` — `crates/layout-paged-typesetter/src/equation/ast.rs:25`
- `impl MRun::new` — `crates/layout-paged-typesetter/src/equation/ast.rs:70`
- `impl MRun::italic` — `crates/layout-paged-typesetter/src/equation/ast.rs:77`
- `parse_latex` — `crates/layout-paged-typesetter/src/equation/latex.rs:5`
- `to_latex` — `crates/layout-paged-typesetter/src/equation/latex.rs:13`
- `impl MathLayout::height` — `crates/layout-paged-typesetter/src/equation/layout.rs:37`
- `impl MathLayout::shift` — `crates/layout-paged-typesetter/src/equation/layout.rs:41`
- `layout_equation` — `crates/layout-paged-typesetter/src/equation/layout.rs:63`
- `parse_linear` — `crates/layout-paged-typesetter/src/equation/linear.rs:5`
- `to_linear` — `crates/layout-paged-typesetter/src/equation/linear.rs:92`
- `impl Patterns::new` — `crates/layout-paged-typesetter/src/hyphenation/liang.rs:15`
- `impl OntologicalTriple::new` — `crates/layout-paged-typesetter/src/ontology.rs:20`
- `impl OntologicalTriple::verify_parity` — `crates/layout-paged-typesetter/src/ontology.rs:31`
- `impl ParaDoc::new` — `crates/layout-paged-typesetter/src/pagination/flow.rs:38`
- `impl ParaDoc::heading` — `crates/layout-paged-typesetter/src/pagination/flow.rs:45`
- `impl Typesetter::new` — `crates/layout-paged-typesetter/src/pagination/flow.rs:71`
- `impl Point::new` — `crates/layout-paged-typesetter/src/pagination/types.rs:12`
- `impl Rect::new` — `crates/layout-paged-typesetter/src/pagination/types.rs:26`
- _…9 more in `modules/crates_layout-paged-typesetter.md`_

### `crates/audio-aac-codec` (33 undocumented public)

- `impl ElementType::channels` — `crates/audio-aac-codec/src/config.rs:21`
- `impl ProgramConfig::parse` — `crates/audio-aac-codec/src/config.rs:44`
- `impl ProgramConfig::write` — `crates/audio-aac-codec/src/config.rs:90`
- `impl AudioSpecificConfig::parse` — `crates/audio-aac-codec/src/config.rs:204`
- `impl AudioSpecificConfig::channels` — `crates/audio-aac-codec/src/config.rs:272`
- `impl AdtsHeader::parse` — `crates/audio-aac-codec/src/config.rs:293`
- `impl Decoder::from_config` — `crates/audio-aac-codec/src/decoder.rs:68`
- `impl Decoder::config` — `crates/audio-aac-codec/src/decoder.rs:93`
- `impl Decoder::sample_rate` — `crates/audio-aac-codec/src/decoder.rs:96`
- `impl Decoder::channels` — `crates/audio-aac-codec/src/decoder.rs:105`
- `impl EncoderConfig::cbr` — `crates/audio-aac-codec/src/encoder/mod.rs:60`
- `impl EncoderConfig::vbr` — `crates/audio-aac-codec/src/encoder/mod.rs:72`
- `impl Encoder::new` — `crates/audio-aac-codec/src/encoder/mod.rs:143`
- `impl Encoder::config` — `crates/audio-aac-codec/src/encoder/mod.rs:236`
- `impl Encoder::stats` — `crates/audio-aac-codec/src/encoder/mod.rs:256`
- `impl BandModel::num_bands` — `crates/audio-aac-codec/src/encoder/psy.rs:77`
- `impl Psy::new` — `crates/audio-aac-codec/src/encoder/psy.rs:98`
- `impl WindowSequence::from_bits` — `crates/audio-aac-codec/src/ics.rs:20`
- `impl IcsInfo::is_short` — `crates/audio-aac-codec/src/ics.rs:49`
- `impl IcsInfo::num_windows` — `crates/audio-aac-codec/src/ics.rs:53`
- `impl IcsInfo::parse` — `crates/audio-aac-codec/src/ics.rs:69`
- `impl IcsInfo::write` — `crates/audio-aac-codec/src/ics.rs:101`
- `impl Mdct::new` — `crates/audio-aac-codec/src/mdct.rs:69`
- `mdct_short` — `crates/audio-aac-codec/src/mdct.rs:141`
- `impl WindowShape::from_bit` — `crates/audio-aac-codec/src/mdct.rs:155`
- _…8 more in `modules/crates_audio-aac-codec.md`_

### `crates/chronicle-office-codecs` (32 undocumented public)

- `escape_latex` — `crates/chronicle-office-codecs/src/document/latex.rs:156`
- `impl FlowInline::plain_text` — `crates/chronicle-office-codecs/src/flow/model.rs:28`
- `impl FlowInline::text_content` — `crates/chronicle-office-codecs/src/flow/model.rs:32`
- `impl FlowPara::new` — `crates/chronicle-office-codecs/src/flow/model.rs:68`
- `impl FlowPara::with_text` — `crates/chronicle-office-codecs/src/flow/model.rs:77`
- `impl FlowPara::plain_text` — `crates/chronicle-office-codecs/src/flow/model.rs:83`
- `impl FlowCell::with_text` — `crates/chronicle-office-codecs/src/flow/model.rs:96`
- `impl FlowDoc::new` — `crates/chronicle-office-codecs/src/flow/model.rs:130`
- `impl FlowDoc::push_para` — `crates/chronicle-office-codecs/src/flow/model.rs:134`
- `impl FlowDoc::push_table` — `crates/chronicle-office-codecs/src/flow/model.rs:138`
- `impl NQuin48::new` — `crates/chronicle-office-codecs/src/ontology.rs:17`
- `impl NQuin48::is_valid` — `crates/chronicle-office-codecs/src/ontology.rs:29`
- `impl Delimiter::as_char` — `crates/chronicle-office-codecs/src/spreadsheet/csv.rs:12`
- `impl XlsxWorkbook::new` — `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:21`
- `impl XlsxWorkbook::with_single_sheet` — `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:25`
- `impl XEl::new` — `crates/chronicle-office-codecs/src/xml/dom.rs:23`
- `impl XEl::with_attr` — `crates/chronicle-office-codecs/src/xml/dom.rs:31`
- `impl XEl::with_child` — `crates/chronicle-office-codecs/src/xml/dom.rs:36`
- `impl XEl::with_text` — `crates/chronicle-office-codecs/src/xml/dom.rs:41`
- `impl XEl::local` — `crates/chronicle-office-codecs/src/xml/dom.rs:46`
- `impl XEl::attr` — `crates/chronicle-office-codecs/src/xml/dom.rs:50`
- `impl XEl::find_child` — `crates/chronicle-office-codecs/src/xml/dom.rs:67`
- `impl XEl::find_children` — `crates/chronicle-office-codecs/src/xml/dom.rs:78`
- `impl XEl::text_content` — `crates/chronicle-office-codecs/src/xml/dom.rs:85`
- `impl XEl::write_xml` — `crates/chronicle-office-codecs/src/xml/dom.rs:100`
- _…7 more in `modules/crates_chronicle-office-codecs.md`_

### `crates/vector-compute-renderer` (31 undocumented public)

- `impl DebugRenderer::new` — `crates/vector-compute-renderer/src/debug/renderer.rs:29`
- `impl DebugRenderer::render` — `crates/vector-compute-renderer/src/debug/renderer.rs:210`
- `impl LineEndpoint::new` — `crates/vector-compute-renderer/src/debug/validate.rs:23`
- `validate_line_soup` — `crates/vector-compute-renderer/src/debug/validate.rs:47`
- `impl Renderer::render_to_texture_async` — `crates/vector-compute-renderer/src/lib.rs:644`
- `impl DebugDownloads<'a>::map` — `crates/vector-compute-renderer/src/lib.rs:794`
- `impl ResourceId::next` — `crates/vector-compute-renderer/src/recording.rs:16`
- `impl Recording::write_image` — `crates/vector-compute-renderer/src/recording.rs:155`
- `impl Recording::dispatch` — `crates/vector-compute-renderer/src/recording.rs:159`
- `impl BufferProxy::new` — `crates/vector-compute-renderer/src/recording.rs:236`
- `impl ImageFormat::to_wgpu` — `crates/vector-compute-renderer/src/recording.rs:245`
- `impl ImageFormat::from_wgpu` — `crates/vector-compute-renderer/src/recording.rs:253`
- `impl ImageProxy::new` — `crates/vector-compute-renderer/src/recording.rs:263`
- `impl ResourceProxy::new_buf` — `crates/vector-compute-renderer/src/recording.rs:275`
- `impl ResourceProxy::new_image` — `crates/vector-compute-renderer/src/recording.rs:279`
- `impl ResourceProxy::as_buf` — `crates/vector-compute-renderer/src/recording.rs:283`
- `impl ResourceProxy::as_image` — `crates/vector-compute-renderer/src/recording.rs:290`
- `impl CapturedBuffers::release_buffers` — `crates/vector-compute-renderer/src/render.rs:77`
- `render_full` — `crates/vector-compute-renderer/src/render.rs:84`
- `impl Render::new` — `crates/vector-compute-renderer/src/render.rs:121`
- `impl Render::bump_buf` — `crates/vector-compute-renderer/src/render.rs:639`
- `impl Render::take_captured_buffers` — `crates/vector-compute-renderer/src/render.rs:650`
- `full_shaders` — `crates/vector-compute-renderer/src/shaders.rs:48`
- `impl RenderContext::new` — `crates/vector-compute-renderer/src/util.rs:32`
- `impl RenderContext::set_present_mode` — `crates/vector-compute-renderer/src/util.rs:130`
- _…6 more in `modules/crates_vector-compute-renderer.md`_

### `crates/geodesy-spatial-analysis` (30 undocumented public)

- `impl RgbaColor::new` — `crates/geodesy-spatial-analysis/src/cartography.rs:32`
- `impl PageSize::dimensions_mm` — `crates/geodesy-spatial-analysis/src/cartography.rs:276`
- `impl Coord::new` — `crates/geodesy-spatial-analysis/src/geodesy.rs:31`
- `impl Coord::is_finite` — `crates/geodesy-spatial-analysis/src/geodesy.rs:35`
- `impl Extent::new` — `crates/geodesy-spatial-analysis/src/geodesy.rs:70`
- `impl Extent::from_points` — `crates/geodesy-spatial-analysis/src/geodesy.rs:74`
- `impl Extent::contains` — `crates/geodesy-spatial-analysis/src/geodesy.rs:83`
- `impl Extent::width` — `crates/geodesy-spatial-analysis/src/geodesy.rs:87`
- `impl Extent::height` — `crates/geodesy-spatial-analysis/src/geodesy.rs:91`
- `impl Body::find` — `crates/geodesy-spatial-analysis/src/geodesy.rs:128`
- `impl Body::get` — `crates/geodesy-spatial-analysis/src/geodesy.rs:133`
- `impl Body::is_earth` — `crates/geodesy-spatial-analysis/src/geodesy.rs:137`
- `impl Body::b` — `crates/geodesy-spatial-analysis/src/geodesy.rs:141`
- `impl Body::mean_radius_m` — `crates/geodesy-spatial-analysis/src/geodesy.rs:145`
- `impl Body::ratio` — `crates/geodesy-spatial-analysis/src/geodesy.rs:149`
- `impl Body::length` — `crates/geodesy-spatial-analysis/src/geodesy.rs:153`
- `impl Body::area` — `crates/geodesy-spatial-analysis/src/geodesy.rs:157`
- `impl Aeqd::new` — `crates/geodesy-spatial-analysis/src/geodesy.rs:373`
- `impl Sinusoidal::new` — `crates/geodesy-spatial-analysis/src/geodesy.rs:411`
- `impl Sinusoidal::wgs84` — `crates/geodesy-spatial-analysis/src/geodesy.rs:423`
- `impl VariogramParams::new` — `crates/geodesy-spatial-analysis/src/kriging.rs:30`
- `impl SamplePoint::new` — `crates/geodesy-spatial-analysis/src/kriging.rs:74`
- `impl DemGrid::new` — `crates/geodesy-spatial-analysis/src/morphometry.rs:23`
- `impl DemGrid::get` — `crates/geodesy-spatial-analysis/src/morphometry.rs:36`
- `impl DemGrid::get_or_edge` — `crates/geodesy-spatial-analysis/src/morphometry.rs:50`
- _…5 more in `modules/crates_geodesy-spatial-analysis.md`_

### `crates/kinetics-garment-draping` (29 undocumented public)

- `impl Element::from_jet` — `crates/kinetics-garment-draping/src/energy.rs:103`
- `membrane_element` — `crates/kinetics-garment-draping/src/energy.rs:116`
- `bending_element` — `crates/kinetics-garment-draping/src/energy.rs:129`
- `impl FabricPreset::to_material` — `crates/kinetics-garment-draping/src/fabrics.rs:22`
- `standard_fabrics` — `crates/kinetics-garment-draping/src/fabrics.rs:45`
- `get_fabric_by_id` — `crates/kinetics-garment-draping/src/fabrics.rs:110`
- `impl DVec2::new` — `crates/kinetics-garment-draping/src/math.rs:18`
- `impl DVec2::dot` — `crates/kinetics-garment-draping/src/math.rs:23`
- `impl DVec2::length_squared` — `crates/kinetics-garment-draping/src/math.rs:28`
- `impl DVec2::length` — `crates/kinetics-garment-draping/src/math.rs:33`
- `impl DVec2::distance` — `crates/kinetics-garment-draping/src/math.rs:38`
- `impl DVec2::lerp` — `crates/kinetics-garment-draping/src/math.rs:43`
- `impl DVec2::is_finite` — `crates/kinetics-garment-draping/src/math.rs:51`
- `impl DVec3::new` — `crates/kinetics-garment-draping/src/math.rs:100`
- `impl DVec3::dot` — `crates/kinetics-garment-draping/src/math.rs:105`
- `impl DVec3::cross` — `crates/kinetics-garment-draping/src/math.rs:110`
- `impl DVec3::length_squared` — `crates/kinetics-garment-draping/src/math.rs:119`
- `impl DVec3::length` — `crates/kinetics-garment-draping/src/math.rs:124`
- `impl DVec3::normalize` — `crates/kinetics-garment-draping/src/math.rs:129`
- `impl DVec3::is_finite` — `crates/kinetics-garment-draping/src/math.rs:139`
- `impl DMat2::from_cols` — `crates/kinetics-garment-draping/src/math.rs:210`
- `impl DMat2::determinant` — `crates/kinetics-garment-draping/src/math.rs:214`
- `impl DMat2::inverse` — `crates/kinetics-garment-draping/src/math.rs:218`
- `impl Jet<N>::variable` — `crates/kinetics-garment-draping/src/math.rs:274`
- `dot` — `crates/kinetics-garment-draping/src/math.rs:366`
- _…4 more in `modules/crates_kinetics-garment-draping.md`_

### `crates/vector-locus-geometry` (29 undocumented public)

- `impl ArcPath::new` — `crates/vector-locus-geometry/src/arc.rs:19`
- `impl ArcPath::is_closed` — `crates/vector-locus-geometry/src/arc.rs:42`
- `impl Anchor::with_handles` — `crates/vector-locus-geometry/src/path.rs:122`
- `impl Anchor::has_in` — `crates/vector-locus-geometry/src/path.rs:127`
- `impl Anchor::has_out` — `crates/vector-locus-geometry/src/path.rs:130`
- `impl Anchor::transform` — `crates/vector-locus-geometry/src/path.rs:133`
- `impl Anchor::translate` — `crates/vector-locus-geometry/src/path.rs:136`
- `impl SubPath::new` — `crates/vector-locus-geometry/src/path.rs:188`
- `impl SubPath::to_bezpath_into` — `crates/vector-locus-geometry/src/path.rs:220`
- `impl SubPath::reverse` — `crates/vector-locus-geometry/src/path.rs:235`
- `impl PathData::new` — `crates/vector-locus-geometry/src/path.rs:354`
- `impl PathData::single` — `crates/vector-locus-geometry/src/path.rs:357`
- `impl PathData::is_empty` — `crates/vector-locus-geometry/src/path.rs:360`
- `impl PathData::anchor_count` — `crates/vector-locus-geometry/src/path.rs:363`
- `impl PathData::to_bezpath` — `crates/vector-locus-geometry/src/path.rs:366`
- `impl PathData::transform` — `crates/vector-locus-geometry/src/path.rs:453`
- `impl PathData::transformed` — `crates/vector-locus-geometry/src/path.rs:460`
- `impl PathData::is_closed` — `crates/vector-locus-geometry/src/path.rs:465`
- `impl PathData::anchor_mut` — `crates/vector-locus-geometry/src/path.rs:488`
- `impl PathData::reverse` — `crates/vector-locus-geometry/src/path.rs:491`
- `impl Plane::parse` — `crates/vector-locus-geometry/src/perspective.rs:26`
- `impl Plane::id` — `crates/vector-locus-geometry/src/perspective.rs:36`
- `impl PerspectiveGrid::validate` — `crates/vector-locus-geometry/src/perspective.rs:165`
- `impl Homography::det` — `crates/vector-locus-geometry/src/projective.rs:50`
- `impl Unit::suffix` — `crates/vector-locus-geometry/src/units.rs:49`
- _…4 more in `modules/crates_vector-locus-geometry.md`_

### `crates/vector-skia-render-adapter` (29 undocumented public)

- `impl GenerationalCache<K, T>::new` — `crates/vector-skia-render-adapter/src/cache.rs:36`
- `impl GenerationalCache<K, T>::next_gen` — `crates/vector-skia-render-adapter/src/cache.rs:44`
- `impl GenerationalCache<K, T>::contains_key` — `crates/vector-skia-render-adapter/src/cache.rs:53`
- `impl GenerationalCache<K, T>::hit` — `crates/vector-skia-render-adapter/src/cache.rs:60`
- `impl GenerationalCache<K, T>::insert` — `crates/vector-skia-render-adapter/src/cache.rs:72`
- `impl MetalBackend::new` — `crates/vector-skia-render-adapter/src/metal.rs:27`
- `impl OpenGLBackend::new` — `crates/vector-skia-render-adapter/src/opengl.rs:30`
- `impl SkiaScenePainter<'_>::new` — `crates/vector-skia-render-adapter/src/scene.rs:82`
- `shader_from_image_brush` — `crates/vector-skia-render-adapter/src/scene.rs:952`
- `shader_from_gradient` — `crates/vector-skia-render-adapter/src/scene.rs:1002`
- `path_fill_type_from_fill` — `crates/vector-skia-render-adapter/src/scene.rs:1144`
- `color4f_from_alpha_color` — `crates/vector-skia-render-adapter/src/scene.rs:1151`
- `color4f_from_dynamic_color` — `crates/vector-skia-render-adapter/src/scene.rs:1160`
- `gradient_shader_cs_from_cs_tag` — `crates/vector-skia-render-adapter/src/scene.rs:1170`
- `gradient_shader_hue_method_from_hue_direction` — `crates/vector-skia-render-adapter/src/scene.rs:1190`
- `tile_mode_from_extend` — `crates/vector-skia-render-adapter/src/scene.rs:1202`
- `blend_mode_from` — `crates/vector-skia-render-adapter/src/scene.rs:1210`
- `rect_from` — `crates/vector-skia-render-adapter/src/scene.rs:1262`
- `rrect_from` — `crates/vector-skia-render-adapter/src/scene.rs:1271`
- `m44_from_affine` — `crates/vector-skia-render-adapter/src/scene.rs:1286`
- `matrix_from_affine` — `crates/vector-skia-render-adapter/src/scene.rs:1315`
- `pt_from` — `crates/vector-skia-render-adapter/src/scene.rs:1337`
- `path_from_shape` — `crates/vector-skia-render-adapter/src/scene.rs:1341`
- `impl VulkanBackend::new` — `crates/vector-skia-render-adapter/src/vulkan.rs:79`
- `impl SkiaRendererOptions::new` — `crates/vector-skia-render-adapter/src/window_renderer.rs:44`
- _…4 more in `modules/crates_vector-skia-render-adapter.md`_

### `crates/audio-timebase` (28 undocumented public)

- `impl NoteValue::label` — `crates/audio-timebase/src/grid.rs:43`
- `impl GridDivision::label` — `crates/audio-timebase/src/grid.rs:87`
- `impl SampleRate::new` — `crates/audio-timebase/src/rate.rs:48`
- `impl SampleRate::hz` — `crates/audio-timebase/src/rate.rs:56`
- `impl SampleRate::as_f64` — `crates/audio-timebase/src/rate.rs:60`
- `impl SampleRange::new` — `crates/audio-timebase/src/rate.rs:89`
- `impl SampleRange::point` — `crates/audio-timebase/src/rate.rs:96`
- `impl SampleRange::len` — `crates/audio-timebase/src/rate.rs:99`
- `impl SampleRange::is_empty` — `crates/audio-timebase/src/rate.rs:102`
- `impl SampleRange::contains` — `crates/audio-timebase/src/rate.rs:105`
- `impl SampleRange::overlaps` — `crates/audio-timebase/src/rate.rs:108`
- `impl SampleRange::intersect` — `crates/audio-timebase/src/rate.rs:111`
- `impl SampleRange::shifted` — `crates/audio-timebase/src/rate.rs:116`
- `impl TimeSig::new` — `crates/audio-timebase/src/tempo.rs:21`
- `impl MeterPoint::new` — `crates/audio-timebase/src/tempo.rs:64`
- `impl MeterPoint::numerator` — `crates/audio-timebase/src/tempo.rs:71`
- `impl MeterPoint::denominator` — `crates/audio-timebase/src/tempo.rs:75`
- `impl MeterPoint::ticks_per_beat` — `crates/audio-timebase/src/tempo.rs:79`
- `impl MeterPoint::ticks_per_bar` — `crates/audio-timebase/src/tempo.rs:83`
- `impl BarBeat::new` — `crates/audio-timebase/src/tempo.rs:97`
- `impl TempoMap::new` — `crates/audio-timebase/src/tempo.rs:127`
- `impl TempoMap::tempos` — `crates/audio-timebase/src/tempo.rs:134`
- `impl TempoMap::meters` — `crates/audio-timebase/src/tempo.rs:138`
- `impl Timecode::new` — `crates/audio-timebase/src/timecode.rs:139`
- `impl FeetFrames::new` — `crates/audio-timebase/src/timecode.rs:255`
- _…3 more in `modules/crates_audio-timebase.md`_

### `crates/locus-parametric-solver` (26 undocumented public)

- `infer` — `crates/locus-parametric-solver/src/solver/auto.rs:106`
- `valid_name` — `crates/locus-parametric-solver/src/solver/expr.rs:46`
- `parse` — `crates/locus-parametric-solver/src/solver/expr.rs:155`
- `impl Env::new` — `crates/locus-parametric-solver/src/solver/expr.rs:178`
- `impl Env::insert` — `crates/locus-parametric-solver/src/solver/expr.rs:181`
- `impl Env::eval` — `crates/locus-parametric-solver/src/solver/expr.rs:184`
- `max_abs` — `crates/locus-parametric-solver/src/solver/lm.rs:18`
- `norm2` — `crates/locus-parametric-solver/src/solver/lm.rs:22`
- `impl Handle::new` — `crates/locus-parametric-solver/src/solver/model.rs:22`
- `impl Sub::parse` — `crates/locus-parametric-solver/src/solver/model.rs:41`
- `impl GeomRef::new` — `crates/locus-parametric-solver/src/solver/model.rs:69`
- `impl GeomRef::whole` — `crates/locus-parametric-solver/src/solver/model.rs:72`
- `impl Model::build` — `crates/locus-parametric-solver/src/solver/model.rs:146`
- `impl Model::slot` — `crates/locus-parametric-solver/src/solver/model.rs:191`
- `impl Model::point` — `crates/locus-parametric-solver/src/solver/model.rs:195`
- `impl Model::line` — `crates/locus-parametric-solver/src/solver/model.rs:244`
- `impl Model::circle` — `crates/locus-parametric-solver/src/solver/model.rs:262`
- `impl Model::arc` — `crates/locus-parametric-solver/src/solver/model.rs:270`
- `impl ConstraintKind::is_dimensional` — `crates/locus-parametric-solver/src/solver/residual.rs:39`
- `impl ConstraintKind::name` — `crates/locus-parametric-solver/src/solver/residual.rs:42`
- `impl Built::new` — `crates/locus-parametric-solver/src/solver/residual.rs:126`
- `impl Built::eval` — `crates/locus-parametric-solver/src/solver/residual.rs:317`
- `glyph` — `crates/locus-parametric-solver/src/typography/stroke.rs:14`
- `char_advance` — `crates/locus-parametric-solver/src/typography/stroke.rs:119`
- `decode_stroke` — `crates/locus-parametric-solver/src/typography/stroke.rs:127`
- _…1 more in `modules/crates_locus-parametric-solver.md`_

### `crates/surface-layered-codecs` (23 undocumented public)

- `impl NQuin48::new` — `crates/surface-layered-codecs/src/ontology.rs:27`
- `impl NQuin48::verify_parity` — `crates/surface-layered-codecs/src/ontology.rs:39`
- `q_hash` — `crates/surface-layered-codecs/src/ontology.rs:44`
- `impl OntologicalContract::new` — `crates/surface-layered-codecs/src/ontology.rs:66`
- `get_morphism_specs` — `crates/surface-layered-codecs/src/ontology.rs:93`
- `get_descriptors` — `crates/surface-layered-codecs/src/ontology.rs:135`
- `read_u16` — `crates/surface-layered-codecs/src/psd/codec.rs:10`
- `read_i16` — `crates/surface-layered-codecs/src/psd/codec.rs:15`
- `read_u32` — `crates/surface-layered-codecs/src/psd/codec.rs:20`
- `read_i32` — `crates/surface-layered-codecs/src/psd/codec.rs:25`
- `impl PsdVersion::is_psb` — `crates/surface-layered-codecs/src/psd/mod.rs:33`
- `impl PsdVersion::to_u16` — `crates/surface-layered-codecs/src/psd/mod.rs:37`
- `impl PsdVersion::from_u16` — `crates/surface-layered-codecs/src/psd/mod.rs:44`
- `impl PsdColorMode::from_u16` — `crates/surface-layered-codecs/src/psd/mod.rs:67`
- `impl PsdColorMode::channel_count` — `crates/surface-layered-codecs/src/psd/mod.rs:81`
- `impl PsdCompression::from_u16` — `crates/surface-layered-codecs/src/psd/mod.rs:104`
- `impl PsdHeader::decode` — `crates/surface-layered-codecs/src/psd/mod.rs:129`
- `impl PsdHeader::encode` — `crates/surface-layered-codecs/src/psd/mod.rs:161`
- `impl PsdBlendMode::from_key` — `crates/surface-layered-codecs/src/psd/mod.rs:214`
- `impl PsdBlendMode::to_key` — `crates/surface-layered-codecs/src/psd/mod.rs:243`
- `impl PsdLayer::width` — `crates/surface-layered-codecs/src/psd/mod.rs:290`
- `impl PsdLayer::height` — `crates/surface-layered-codecs/src/psd/mod.rs:294`
- `impl XcfColorType::from_u32` — `crates/surface-layered-codecs/src/xcf_ora.rs:30`

### `crates/discourse-pdf-compliance` (22 undocumented public)

- `check_accessibility` — `crates/discourse-pdf-compliance/src/a11y/checker.rs:88`
- `summarize` — `crates/discourse-pdf-compliance/src/a11y/report.rs:18`
- `generate_html_report` — `crates/discourse-pdf-compliance/src/a11y/report.rs:36`
- `impl Category::label` — `crates/discourse-pdf-compliance/src/a11y/rules.rs:29`
- `impl Rule::category` — `crates/discourse-pdf-compliance/src/a11y/rules.rs:115`
- `impl Rule::title` — `crates/discourse-pdf-compliance/src/a11y/rules.rs:156`
- `impl AcroFormField::new_text` — `crates/discourse-pdf-compliance/src/forms/model.rs:47`
- `impl AcroFormField::new_checkbox` — `crates/discourse-pdf-compliance/src/forms/model.rs:61`
- `impl AcroFormField::is_read_only` — `crates/discourse-pdf-compliance/src/forms/model.rs:75`
- `impl AcroFormField::is_required` — `crates/discourse-pdf-compliance/src/forms/model.rs:79`
- `impl AcroForm::field` — `crates/discourse-pdf-compliance/src/forms/model.rs:92`
- `impl AcroForm::field_mut` — `crates/discourse-pdf-compliance/src/forms/model.rs:96`
- `format_field` — `crates/discourse-pdf-compliance/src/forms/validation.rs:42`
- `validate_field` — `crates/discourse-pdf-compliance/src/forms/validation.rs:86`
- `execute_calculation` — `crates/discourse-pdf-compliance/src/forms/validation.rs:106`
- `impl StandardProfile::label` — `crates/discourse-pdf-compliance/src/preflight/evaluator.rs:18`
- `evaluate_preflight` — `crates/discourse-pdf-compliance/src/preflight/evaluator.rs:64`
- `impl HiddenCategory::label` — `crates/discourse-pdf-compliance/src/redact/sanitizer.rs:33`
- `hex_encode` — `crates/discourse-pdf-compliance/src/sign/digest.rs:118`
- `impl ByteRange::new` — `crates/discourse-pdf-compliance/src/sign/pades.rs:19`
- `impl ByteRange::to_pdf_string` — `crates/discourse-pdf-compliance/src/sign/pades.rs:50`
- `verify_rfc3161_timestamp` — `crates/discourse-pdf-compliance/src/sign/timestamp.rs:25`

### `crates/layout-box-autolayout` (22 undocumented public)

- `main_offsets` — `crates/layout-box-autolayout/src/flex.rs:23`
- `solve_flex` — `crates/layout-box-autolayout/src/flex.rs:67`
- `impl LayoutNode::new` — `crates/layout-box-autolayout/src/frame.rs:39`
- `impl LayoutNode::with_sizing` — `crates/layout-box-autolayout/src/frame.rs:63`
- `impl LayoutNode::with_constraints` — `crates/layout-box-autolayout/src/frame.rs:69`
- `impl AutoLayoutFrame::new` — `crates/layout-box-autolayout/src/frame.rs:123`
- `impl AutoLayoutFrame::with_padding` — `crates/layout-box-autolayout/src/frame.rs:130`
- `impl AutoLayoutFrame::with_gap` — `crates/layout-box-autolayout/src/frame.rs:135`
- `impl AutoLayoutFrame::with_wrap` — `crates/layout-box-autolayout/src/frame.rs:140`
- `impl AutoLayoutFrame::add_child` — `crates/layout-box-autolayout/src/frame.rs:145`
- `grid_place` — `crates/layout-box-autolayout/src/grid.rs:20`
- `solve_grid` — `crates/layout-box-autolayout/src/grid.rs:168`
- `impl MainAlign::is_auto_gap` — `crates/layout-box-autolayout/src/types.rs:35`
- `impl Rect::new` — `crates/layout-box-autolayout/src/types.rs:110`
- `impl Rect::right` — `crates/layout-box-autolayout/src/types.rs:113`
- `impl Rect::bottom` — `crates/layout-box-autolayout/src/types.rs:116`
- `impl Padding::new` — `crates/layout-box-autolayout/src/types.rs:130`
- `impl Padding::uniform` — `crates/layout-box-autolayout/src/types.rs:133`
- `impl Padding::symmetric` — `crates/layout-box-autolayout/src/types.rs:136`
- `impl Padding::horizontal` — `crates/layout-box-autolayout/src/types.rs:139`
- `impl Padding::vertical` — `crates/layout-box-autolayout/src/types.rs:142`
- `clamp_size` — `crates/layout-box-autolayout/src/types.rs:147`

### `crates/bitstream-codec` (21 undocumented public)

- `impl BitReader<'a>::new` — `crates/bitstream-codec/src/lib.rs:41`
- `impl BitReader<'a>::data` — `crates/bitstream-codec/src/lib.rs:44`
- `impl BitReader<'a>::bits_left` — `crates/bitstream-codec/src/lib.rs:51`
- `impl BitReader<'a>::is_byte_aligned` — `crates/bitstream-codec/src/lib.rs:54`
- `impl BitReader<'a>::byte_align` — `crates/bitstream-codec/src/lib.rs:57`
- `impl BitReader<'a>::seek_bits` — `crates/bitstream-codec/src/lib.rs:64`
- `impl BitReader<'a>::read_bits` — `crates/bitstream-codec/src/lib.rs:86`
- `impl BitReader<'a>::read_bits_u64` — `crates/bitstream-codec/src/lib.rs:95`
- `impl BitReader<'a>::read_bit` — `crates/bitstream-codec/src/lib.rs:105`
- `impl BitReader<'a>::read_flag` — `crates/bitstream-codec/src/lib.rs:108`
- `impl BitReader<'a>::skip` — `crates/bitstream-codec/src/lib.rs:112`
- `impl BitWriter::new` — `crates/bitstream-codec/src/lib.rs:178`
- `impl BitWriter::write_bits` — `crates/bitstream-codec/src/lib.rs:181`
- `impl BitWriter::write_bit` — `crates/bitstream-codec/src/lib.rs:195`
- `impl BitWriter::write_ue` — `crates/bitstream-codec/src/lib.rs:198`
- `impl BitWriter::write_se` — `crates/bitstream-codec/src/lib.rs:212`
- `impl BitWriter::bit_len` — `crates/bitstream-codec/src/lib.rs:216`
- `impl BitWriter::is_byte_aligned` — `crates/bitstream-codec/src/lib.rs:219`
- `impl BitWriter::write_bytes` — `crates/bitstream-codec/src/lib.rs:233`
- `impl BitWriter::finish` — `crates/bitstream-codec/src/lib.rs:242`
- `impl BitWriter::bytes` — `crates/bitstream-codec/src/lib.rs:246`

### `crates/layout-nodegraph-compiler` (21 undocumented public)

- `flatten_network` — `crates/layout-nodegraph-compiler/src/compiler/flatten.rs:9`
- `impl NodeId::new` — `crates/layout-nodegraph-compiler/src/compiler/node.rs:12`
- `impl NodeNetwork::new` — `crates/layout-nodegraph-compiler/src/compiler/node.rs:69`
- `impl NodeNetwork::add_node` — `crates/layout-nodegraph-compiler/src/compiler/node.rs:73`
- `impl NodeNetwork::output_node_id` — `crates/layout-nodegraph-compiler/src/compiler/node.rs:77`
- `impl CompilerOptions::all_optimizations` — `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:20`
- `impl NodeGraphCompiler::new` — `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:42`
- `impl NodeGraphCompiler::compile` — `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:46`
- `impl ProtoNetwork::new` — `crates/layout-nodegraph-compiler/src/compiler/proto.rs:29`
- `impl ProtoNetwork::add_node` — `crates/layout-nodegraph-compiler/src/compiler/proto.rs:37`
- `impl ProtoNetwork::node` — `crates/layout-nodegraph-compiler/src/compiler/proto.rs:41`
- `impl ProtoNetwork::node_mut` — `crates/layout-nodegraph-compiler/src/compiler/proto.rs:45`
- `impl Unit::parse` — `crates/layout-nodegraph-compiler/src/math_parser/ast.rs:23`
- `impl MathContext::new` — `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:15`
- `impl MathContext::set_var` — `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:24`
- `impl MathContext::eval` — `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:28`
- `impl MathContext::eval_node` — `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:33`
- `tokenize` — `crates/layout-nodegraph-compiler/src/math_parser/lexer.rs:20`
- `impl Parser<'a>::new` — `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:14`
- `impl Parser<'a>::parse_expression` — `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:30`
- `parse` — `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:139`

### `crates/chronicle-fiduciary-ledger` (20 undocumented public)

- `impl AccountClass::normal_balance_is_debit` — `crates/chronicle-fiduciary-ledger/src/accounts.rs:18`
- `impl ChartOfAccounts::new` — `crates/chronicle-fiduciary-ledger/src/accounts.rs:43`
- `impl ChartOfAccounts::add_account` — `crates/chronicle-fiduciary-ledger/src/accounts.rs:50`
- `impl ChartOfAccounts::get` — `crates/chronicle-fiduciary-ledger/src/accounts.rs:72`
- `impl ChartOfAccounts::get_by_code` — `crates/chronicle-fiduciary-ledger/src/accounts.rs:76`
- `impl ChartOfAccounts::get_by_name` — `crates/chronicle-fiduciary-ledger/src/accounts.rs:80`
- `impl ChartOfAccounts::all` — `crates/chronicle-fiduciary-ledger/src/accounts.rs:84`
- `impl FiduciaryAuditEntry::compute_hash` — `crates/chronicle-fiduciary-ledger/src/audit.rs:34`
- `impl FiduciaryAuditTrail::new` — `crates/chronicle-fiduciary-ledger/src/audit.rs:59`
- `impl FiduciaryAuditTrail::filter_by_entity` — `crates/chronicle-fiduciary-ledger/src/audit.rs:159`
- `impl DepreciationMethod::as_str` — `crates/chronicle-fiduciary-ledger/src/depreciation.rs:23`
- `generate_depreciation_schedule` — `crates/chronicle-fiduciary-ledger/src/depreciation.rs:110`
- `impl JLine::debit` — `crates/chronicle-fiduciary-ledger/src/journal.rs:24`
- `impl JLine::credit` — `crates/chronicle-fiduciary-ledger/src/journal.rs:34`
- `impl JLine::with_desc` — `crates/chronicle-fiduciary-ledger/src/journal.rs:44`
- `impl JournalLedger::new` — `crates/chronicle-fiduciary-ledger/src/journal.rs:74`
- `impl JournalLedger::set_lock_date` — `crates/chronicle-fiduciary-ledger/src/journal.rs:84`
- `impl JournalLedger::check_lock` — `crates/chronicle-fiduciary-ledger/src/journal.rs:88`
- `fmt_fixed` — `crates/chronicle-fiduciary-ledger/src/money.rs:97`
- `fmt_money` — `crates/chronicle-fiduciary-ledger/src/money.rs:113`

### `crates/kinetics-motion-compositor` (20 undocumented public)

- `impl Value::as_f64` — `crates/kinetics-motion-compositor/src/keyframe/mod.rs:54`
- `impl Value::lerp` — `crates/kinetics-motion-compositor/src/keyframe/mod.rs:63`
- `impl Keyframe::new` — `crates/kinetics-motion-compositor/src/keyframe/mod.rs:102`
- `impl Keyframe::hold` — `crates/kinetics-motion-compositor/src/keyframe/mod.rs:118`
- `impl Keyframe::eased` — `crates/kinetics-motion-compositor/src/keyframe/mod.rs:124`
- `impl KeyframeTrack::evaluate` — `crates/kinetics-motion-compositor/src/keyframe/mod.rs:140`
- `impl Tick::from_rational` — `crates/kinetics-motion-compositor/src/time.rs:13`
- `impl Tick::to_rational` — `crates/kinetics-motion-compositor/src/time.rs:21`
- `impl Tick::from_seconds` — `crates/kinetics-motion-compositor/src/time.rs:30`
- `impl Tick::as_seconds` — `crates/kinetics-motion-compositor/src/time.rs:34`
- `impl FrameRate::as_f64` — `crates/kinetics-motion-compositor/src/time.rs:65`
- `impl FrameRate::frame_at` — `crates/kinetics-motion-compositor/src/time.rs:69`
- `impl FrameRate::tick_of_frame` — `crates/kinetics-motion-compositor/src/time.rs:73`
- `impl GrayImage::new` — `crates/kinetics-motion-compositor/src/track/correlation.rs:14`
- `impl GrayImage::from_luma` — `crates/kinetics-motion-compositor/src/track/correlation.rs:22`
- `impl GrayImage::get` — `crates/kinetics-motion-compositor/src/track/correlation.rs:32`
- `impl TextAnimator::evaluate_character` — `crates/kinetics-motion-compositor/src/typography/animator.rs:48`
- `impl Mode::initial` — `crates/kinetics-motion-compositor/src/typography/selectors.rs:32`
- `impl Mode::combine` — `crates/kinetics-motion-compositor/src/typography/selectors.rs:39`
- `shape_value` — `crates/kinetics-motion-compositor/src/typography/selectors.rs:122`

### `crates/vector-locus-pathops` (20 undocumented public)

- `normal_at` — `crates/vector-locus-pathops/src/effects/distort.rs:14`
- `num` — `crates/vector-locus-pathops/src/effects/util.rs:6`
- `flag` — `crates/vector-locus-pathops/src/effects/util.rs:24`
- `text` — `crates/vector-locus-pathops/src/effects/util.rs:32`
- `seed` — `crates/vector-locus-pathops/src/effects/util.rs:36`
- `smooth_points` — `crates/vector-locus-pathops/src/effects/util.rs:40`
- `mean_size` — `crates/vector-locus-pathops/src/effects/util.rs:44`
- `seg_cubic` — `crates/vector-locus-pathops/src/effects/util.rs:61`
- `poly_len` — `crates/vector-locus-pathops/src/effects/util.rs:70`
- `impl WarpStyle::from_id` — `crates/vector-locus-pathops/src/effects/warp.rs:34`
- `impl BoolOp::apply` — `crates/vector-locus-pathops/src/pathops/boolean.rs:29`
- `impl Shape::new` — `crates/vector-locus-pathops/src/pathops/pathfinder.rs:18`
- `impl V3::sub` — `crates/vector-locus-pathops/src/three_d/revolve.rs:15`
- `impl V3::cross` — `crates/vector-locus-pathops/src/three_d/revolve.rs:18`
- `impl V3::dot` — `crates/vector-locus-pathops/src/three_d/revolve.rs:25`
- `impl V3::unit` — `crates/vector-locus-pathops/src/three_d/revolve.rs:28`
- `impl Revolve::sanitized` — `crates/vector-locus-pathops/src/three_d/revolve.rs:93`
- `impl Revolve::rotate` — `crates/vector-locus-pathops/src/three_d/revolve.rs:110`
- `impl Revolve::light_direction` — `crates/vector-locus-pathops/src/three_d/revolve.rs:119`
- `impl Face::path` — `crates/vector-locus-pathops/src/three_d/revolve.rs:136`

### `crates/vector-surface-brush` (20 undocumented public)

- `default_arrow_art` — `crates/vector-surface-brush/src/art.rs:62`
- `impl ArtBrush::expand` — `crates/vector-surface-brush/src/art.rs:79`
- `impl BristleBrush::expand` — `crates/vector-surface-brush/src/bristle.rs:68`
- `impl CalligraphicBrush::modes` — `crates/vector-surface-brush/src/calligraphic.rs:54`
- `impl CalligraphicBrush::uses_pressure` — `crates/vector-surface-brush/src/calligraphic.rs:58`
- `impl CalligraphicBrush::expand` — `crates/vector-surface-brush/src/calligraphic.rs:62`
- `impl Nib::new` — `crates/vector-surface-brush/src/calligraphic.rs:149`
- `impl Nib::support` — `crates/vector-surface-brush/src/calligraphic.rs:157`
- `sweep` — `crates/vector-surface-brush/src/calligraphic.rs:199`
- `default_stitch_tile` — `crates/vector-surface-brush/src/pattern.rs:55`
- `impl PatternBrush::expand` — `crates/vector-surface-brush/src/pattern.rs:60`
- `default_dot_art` — `crates/vector-surface-brush/src/scatter.rs:39`
- `impl ScatterBrush::modes` — `crates/vector-surface-brush/src/scatter.rs:46`
- `impl ScatterBrush::expand` — `crates/vector-surface-brush/src/scatter.rs:52`
- `impl Track::new` — `crates/vector-surface-brush/src/track.rs:34`
- `impl Track::len` — `crates/vector-surface-brush/src/track.rs:84`
- `impl Track::is_empty` — `crates/vector-surface-brush/src/track.rs:88`
- `impl Track::segments` — `crates/vector-surface-brush/src/track.rs:92`
- `impl Rng::new` — `crates/vector-surface-brush/src/track.rs:185`
- `impl Rng::next_u64` — `crates/vector-surface-brush/src/track.rs:188`

### `crates/vector-hybrid-render-adapter` (18 undocumented public)

- `convert_filter` — `crates/vector-hybrid-render-adapter/src/filters.rs:6`
- `convert_filter_effect` — `crates/vector-hybrid-render-adapter/src/filters.rs:20`
- `impl ImageManager<'a>::new` — `crates/vector-hybrid-render-adapter/src/scene.rs:75`
- `impl ImageManager<'a>::upload_image` — `crates/vector-hybrid-render-adapter/src/scene.rs:93`
- `impl VelloHybridScenePainter<'_>::new` — `crates/vector-hybrid-render-adapter/src/scene.rs:137`
- `impl WebGlImageManager<'a>::new` — `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:24`
- `impl WebGlImageManager<'a>::upload_image` — `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:36`
- `impl WebGlScenePainter<'s>::new` — `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:68`
- `impl VelloHybridRendererOptions::new` — `crates/vector-hybrid-render-adapter/src/window_renderer.rs:90`
- `impl VelloHybridRendererOptions::features` — `crates/vector-hybrid-render-adapter/src/window_renderer.rs:95`
- `impl VelloHybridRendererOptions::limits` — `crates/vector-hybrid-render-adapter/src/window_renderer.rs:102`
- `impl VelloHybridRendererOptions::render_settings` — `crates/vector-hybrid-render-adapter/src/window_renderer.rs:109`
- `impl VelloHybridRendererOptions::base_color` — `crates/vector-hybrid-render-adapter/src/window_renderer.rs:116`
- `impl VelloHybridRendererOptions::composite_alpha_mode` — `crates/vector-hybrid-render-adapter/src/window_renderer.rs:120`
- `impl VelloHybridRendererOptions::desired_maximum_frame_latency` — `crates/vector-hybrid-render-adapter/src/window_renderer.rs:130`
- `impl VelloHybridWindowRenderer::new` — `crates/vector-hybrid-render-adapter/src/window_renderer.rs:161`
- `impl VelloHybridWindowRenderer::with_options` — `crates/vector-hybrid-render-adapter/src/window_renderer.rs:165`
- `impl VelloHybridWindowRenderer::current_device_handle` — `crates/vector-hybrid-render-adapter/src/window_renderer.rs:179`

### `crates/vector-radiance-trace` (18 undocumented public)

- `impl Seg::line` — `crates/vector-radiance-trace/src/fit.rs:237`
- `segs_to_subpath` — `crates/vector-radiance-trace/src/fit.rs:245`
- `simplify_with` — `crates/vector-radiance-trace/src/fit.rs:297`
- `unit` — `crates/vector-radiance-trace/src/fit.rs:398`
- `start_tangent` — `crates/vector-radiance-trace/src/fit.rs:407`
- `end_tangent` — `crates/vector-radiance-trace/src/fit.rs:414`
- `fit_cubics` — `crates/vector-radiance-trace/src/fit.rs:421`
- `fit_single` — `crates/vector-radiance-trace/src/fit.rs:459`
- `fit_single_from` — `crates/vector-radiance-trace/src/fit.rs:463`
- `is_straight` — `crates/vector-radiance-trace/src/fit.rs:576`
- `sample` — `crates/vector-radiance-trace/src/fit.rs:590`
- `impl Raster::new` — `crates/vector-radiance-trace/src/lib.rs:56`
- `impl Raster::from_fn` — `crates/vector-radiance-trace/src/lib.rs:65`
- `impl Raster::pixel` — `crates/vector-radiance-trace/src/lib.rs:75`
- `impl TracePreset::to_options` — `crates/vector-radiance-trace/src/lib.rs:208`
- `preset` — `crates/vector-radiance-trace/src/lib.rs:241`
- `presets` — `crates/vector-radiance-trace/src/lib.rs:292`
- `impl TraceResult::anchor_count` — `crates/vector-radiance-trace/src/lib.rs:316`

### `crates/video-prores-codec` (18 undocumented public)

- `impl Reader<'a>::new` — `crates/video-prores-codec/src/bits.rs:18`
- `impl Reader<'a>::bits_left` — `crates/video-prores-codec/src/bits.rs:57`
- `impl Reader<'a>::overrun` — `crates/video-prores-codec/src/bits.rs:62`
- `impl Reader<'a>::read` — `crates/video-prores-codec/src/bits.rs:81`
- `impl Reader<'a>::read_bit` — `crates/video-prores-codec/src/bits.rs:93`
- `impl Writer::new` — `crates/video-prores-codec/src/bits.rs:176`
- `impl EncoderConfig::new` — `crates/video-prores-codec/src/encode.rs:29`
- `impl Encoder::new` — `crates/video-prores-codec/src/encode.rs:56`
- `impl Encoder::with_config` — `crates/video-prores-codec/src/encode.rs:60`
- `impl Encoder::config` — `crates/video-prores-codec/src/encode.rs:64`
- `impl Encoder::bits_per_mb` — `crates/video-prores-codec/src/encode.rs:68`
- `encode_alpha` — `crates/video-prores-codec/src/encode.rs:432`
- `impl PictureHeader::parse` — `crates/video-prores-codec/src/header.rs:187`
- `impl PictureHeader::write` — `crates/video-prores-codec/src/header.rs:201`
- `impl Profile::from_fourcc` — `crates/video-prores-codec/src/types.rs:114`
- `impl Profile::chroma` — `crates/video-prores-codec/src/types.rs:118`
- `impl Frame::chroma_width` — `crates/video-prores-codec/src/types.rs:191`
- `impl Frame::is_interlaced` — `crates/video-prores-codec/src/types.rs:195`

### `crates/discourse-academic-tex` (17 undocumented public)

- `csl_to_bibtex` — `crates/discourse-academic-tex/src/bibliography/import.rs:125`
- `ris_kind` — `crates/discourse-academic-tex/src/bibliography/import.rs:171`
- `ris_entries` — `crates/discourse-academic-tex/src/bibliography/import.rs:184`
- `source_hash` — `crates/discourse-academic-tex/src/review/ai_repair.rs:217`
- `impl TeXDiagnostic::new` — `crates/discourse-academic-tex/src/review/logrules.rs:25`
- `resolve` — `crates/discourse-academic-tex/src/texparse/graph.rs:116`
- `build` — `crates/discourse-academic-tex/src/texparse/graph.rs:141`
- `declares_document_class` — `crates/discourse-academic-tex/src/texparse/graph.rs:182`
- `skip_ws` — `crates/discourse-academic-tex/src/texparse/structure.rs:291`
- `opt_arg` — `crates/discourse-academic-tex/src/texparse/structure.rs:302`
- `group_text` — `crates/discourse-academic-tex/src/texparse/structure.rs:335`
- `impl Page::new` — `crates/discourse-academic-tex/src/texpdf.rs:287`
- `impl Page::text` — `crates/discourse-academic-tex/src/texpdf.rs:290`
- `impl Page::rule` — `crates/discourse-academic-tex/src/texpdf.rs:293`
- `impl Doc::new` — `crates/discourse-academic-tex/src/texpdf.rs:306`
- `impl Doc::font` — `crates/discourse-academic-tex/src/texpdf.rs:310`
- `impl Doc::add_page` — `crates/discourse-academic-tex/src/texpdf.rs:315`

### `crates/gpu-device-context` (17 undocumented public)

- `impl BufferRenderer::device` — `crates/gpu-device-context/src/buffer_renderer.rs:63`
- `impl BufferRenderer::queue` — `crates/gpu-device-context/src/buffer_renderer.rs:67`
- `impl BufferRenderer::size` — `crates/gpu-device-context/src/buffer_renderer.rs:71`
- `impl BufferRenderer::resize` — `crates/gpu-device-context/src/buffer_renderer.rs:79`
- `impl BufferRenderer::target_texture_view` — `crates/gpu-device-context/src/buffer_renderer.rs:117`
- `impl BufferRenderer::copy_texture_to_vec` — `crates/gpu-device-context/src/buffer_renderer.rs:121`
- `impl BufferRenderer::copy_texture_to_buffer` — `crates/gpu-device-context/src/buffer_renderer.rs:127`
- `impl WGPUContext::new` — `crates/gpu-device-context/src/lib.rs:116`
- `impl WGPUContext::with_features_and_limits` — `crates/gpu-device-context/src/lib.rs:120`
- `impl WGPUContext::extra_features` — `crates/gpu-device-context/src/lib.rs:141`
- `impl WGPUContext::override_limits` — `crates/gpu-device-context/src/lib.rs:145`
- `impl WGPUContext::create_device_handle` — `crates/gpu-device-context/src/lib.rs:230`
- `impl SurfaceRenderer<'s>::device` — `crates/gpu-device-context/src/surface_renderer.rs:208`
- `impl SurfaceRenderer<'s>::queue` — `crates/gpu-device-context/src/surface_renderer.rs:212`
- `impl SurfaceRenderer<'s>::set_present_mode` — `crates/gpu-device-context/src/surface_renderer.rs:234`
- `impl SurfaceRenderer<'s>::clear_surface_texture` — `crates/gpu-device-context/src/surface_renderer.rs:244`
- `impl SurfaceRenderer<'s>::ensure_current_surface_texture` — `crates/gpu-device-context/src/surface_renderer.rs:275`

### `crates/raster-layer-mask-adapter` (17 undocumented public)

- `par_rows` — `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:177`
- `par_rows` — `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:162`
- `impl Dynamic::evaluate` — `crates/raster-layer-mask-adapter/src/brush.rs:72`
- `impl BlendRange::is_full` — `crates/raster-layer-mask-adapter/src/masks.rs:173`
- `impl BlendRange::from_bytes` — `crates/raster-layer-mask-adapter/src/masks.rs:177`
- `impl BlendRange::to_bytes` — `crates/raster-layer-mask-adapter/src/masks.rs:184`
- `impl BlendIf::is_default` — `crates/raster-layer-mask-adapter/src/masks.rs:212`
- `impl BlendIf::get` — `crates/raster-layer-mask-adapter/src/masks.rs:216`
- `impl BlendIf::set` — `crates/raster-layer-mask-adapter/src/masks.rs:220`
- `impl MixerState::new` — `crates/raster-layer-mask-adapter/src/mixer.rs:50`
- `impl MixerState::with_reservoir` — `crates/raster-layer-mask-adapter/src/mixer.rs:54`
- `impl MixerState::load` — `crates/raster-layer-mask-adapter/src/mixer.rs:62`
- `impl MixerState::clean` — `crates/raster-layer-mask-adapter/src/mixer.rs:67`
- `impl NQuin48::new` — `crates/raster-layer-mask-adapter/src/ontology.rs:15`
- `impl NQuin48::precondition` — `crates/raster-layer-mask-adapter/src/ontology.rs:19`
- `impl NQuin48::postcondition` — `crates/raster-layer-mask-adapter/src/ontology.rs:23`
- `impl NQuin48::invariant` — `crates/raster-layer-mask-adapter/src/ontology.rs:27`

### `crates/profile-authoring` (14 undocumented public)

- `finite_positive` — `crates/profile-authoring/provenance/original-lib.rs:28`
- `compile_assembly` — `crates/profile-authoring/src/assembly.rs:164`
- `unit` — `crates/profile-authoring/src/frames.rs:9`
- `record` — `crates/profile-authoring/src/frames.rs:25`
- `normal` — `crates/profile-authoring/src/frames.rs:46`
- `face` — `crates/profile-authoring/src/frames.rs:57`
- `impl GeometryLimits::admit` — `crates/profile-authoring/src/geometry.rs:31`
- `sphere` — `crates/profile-authoring/src/geometry.rs:201`
- `torus` — `crates/profile-authoring/src/geometry.rs:230`
- `rounded_rect` — `crates/profile-authoring/src/lib.rs:39`
- `circle` — `crates/profile-authoring/src/lib.rs:48`
- `closed_polygon` — `crates/profile-authoring/src/lib.rs:52`
- `finite_positive` — `crates/profile-authoring/src/lib.rs:56`
- `impl Profile::points` — `crates/profile-authoring/src/profile.rs:94`

### `crates/electromagnetics-rf-sensing` (13 undocumented public)

- `impl UniformLinearArray::new` — `crates/electromagnetics-rf-sensing/src/beamforming.rs:16`
- `impl Complex32::new` — `crates/electromagnetics-rf-sensing/src/csi_signal.rs:20`
- `impl Complex32::from_polar` — `crates/electromagnetics-rf-sensing/src/csi_signal.rs:24`
- `impl Complex32::abs` — `crates/electromagnetics-rf-sensing/src/csi_signal.rs:32`
- `impl Complex32::arg` — `crates/electromagnetics-rf-sensing/src/csi_signal.rs:37`
- `impl Complex32::norm_sq` — `crates/electromagnetics-rf-sensing/src/csi_signal.rs:42`
- `impl Complex32::conj` — `crates/electromagnetics-rf-sensing/src/csi_signal.rs:47`
- `impl Complex32::add` — `crates/electromagnetics-rf-sensing/src/csi_signal.rs:52`
- `impl Complex32::mul` — `crates/electromagnetics-rf-sensing/src/csi_signal.rs:57`
- `impl Complex32::scale` — `crates/electromagnetics-rf-sensing/src/csi_signal.rs:65`
- `impl CsiFrame::new` — `crates/electromagnetics-rf-sensing/src/csi_signal.rs:82`
- `impl FresnelZoneGeometry::new` — `crates/electromagnetics-rf-sensing/src/csi_signal.rs:226`
- `impl FallDetector::new` — `crates/electromagnetics-rf-sensing/src/vitals.rs:157`

### `crates/kinetics-continuum-multiphysics` (13 undocumented public)

- `impl ExplicitIntegrator::new` — `crates/kinetics-continuum-multiphysics/src/explicit/dynamics.rs:27`
- `impl ElasticIsotropic::new` — `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:16`
- `impl SaintVenantKirchhoff::new` — `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:93`
- `impl NeoHookean::new` — `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:122`
- `impl J2Plasticity::new` — `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:169`
- `impl ElementType::node_count` — `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:21`
- `impl ElementType::dimension` — `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:34`
- `impl LbmGrid3D::new` — `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:73`
- `impl LbmGrid3D::idx` — `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:94`
- `impl Point2D::new` — `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:15`
- `impl Point2D::dist_sq` — `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:18`
- `impl Point3D::new` — `crates/kinetics-continuum-multiphysics/src/mesh/tet.rs:16`
- `impl CsrMatrix::new` — `crates/kinetics-continuum-multiphysics/src/solver/sparse.rs:18`

### `crates/radiance-camera-raw` (12 undocumented public)

- `impl GpxTracklog::new` — `crates/radiance-camera-raw/src/metadata.rs:57`
- `impl GpxTracklog::add_point` — `crates/radiance-camera-raw/src/metadata.rs:61`
- `impl JournalAction::inverse` — `crates/radiance-camera-raw/src/metadata.rs:146`
- `impl CatalogJournal::new` — `crates/radiance-camera-raw/src/metadata.rs:177`
- `impl CatalogJournal::record_action` — `crates/radiance-camera-raw/src/metadata.rs:181`
- `impl CatalogJournal::undo` — `crates/radiance-camera-raw/src/metadata.rs:186`
- `impl CatalogJournal::redo` — `crates/radiance-camera-raw/src/metadata.rs:193`
- `impl SmartRule::matches` — `crates/radiance-camera-raw/src/metadata.rs:219`
- `impl NQuin48::new` — `crates/radiance-camera-raw/src/ontology.rs:15`
- `impl NQuin48::precondition` — `crates/radiance-camera-raw/src/ontology.rs:19`
- `impl NQuin48::postcondition` — `crates/radiance-camera-raw/src/ontology.rs:23`
- `impl NQuin48::invariant` — `crates/radiance-camera-raw/src/ontology.rs:27`

### `crates/vector-gpu-render-adapter` (12 undocumented public)

- `impl VelloScenePainter<'_, '_>::new` — `crates/vector-gpu-render-adapter/src/scene.rs:54`
- `impl VelloRendererOptions::new` — `crates/vector-gpu-render-adapter/src/window_renderer.rs:86`
- `impl VelloRendererOptions::features` — `crates/vector-gpu-render-adapter/src/window_renderer.rs:98`
- `impl VelloRendererOptions::limits` — `crates/vector-gpu-render-adapter/src/window_renderer.rs:105`
- `impl VelloRendererOptions::base_color` — `crates/vector-gpu-render-adapter/src/window_renderer.rs:112`
- `impl VelloRendererOptions::antialiasing_method` — `crates/vector-gpu-render-adapter/src/window_renderer.rs:116`
- `impl VelloRendererOptions::composite_alpha_mode` — `crates/vector-gpu-render-adapter/src/window_renderer.rs:123`
- `impl VelloRendererOptions::pipeline_cache` — `crates/vector-gpu-render-adapter/src/window_renderer.rs:133`
- `impl VelloRendererOptions::desired_maximum_frame_latency` — `crates/vector-gpu-render-adapter/src/window_renderer.rs:140`
- `impl VelloWindowRenderer::new` — `crates/vector-gpu-render-adapter/src/window_renderer.rs:174`
- `impl VelloWindowRenderer::with_options` — `crates/vector-gpu-render-adapter/src/window_renderer.rs:178`
- `impl VelloWindowRenderer::current_device_handle` — `crates/vector-gpu-render-adapter/src/window_renderer.rs:190`

### `crates/video-h264-preview` (12 undocumented public)

- `impl Frame::new` — `crates/video-h264-preview/src/lib.rs:52`
- `impl Frame::fit` — `crates/video-h264-preview/src/lib.rs:55`
- `impl MovieInfo::display_aspect` — `crates/video-h264-preview/src/movie.rs:63`
- `impl Movie::info` — `crates/video-h264-preview/src/movie.rs:305`
- `impl Movie::set_cache_frames` — `crates/video-h264-preview/src/movie.rs:309`
- `impl Movie::frame_index_at` — `crates/video-h264-preview/src/movie.rs:313`
- `impl Movie::frame_time` — `crates/video-h264-preview/src/movie.rs:321`
- `impl Movie::frame_at` — `crates/video-h264-preview/src/movie.rs:325`
- `impl Movie::is_cached` — `crates/video-h264-preview/src/movie.rs:330`
- `impl Movie::frame` — `crates/video-h264-preview/src/movie.rs:334`
- `impl Movie::thumbnail_at` — `crates/video-h264-preview/src/movie.rs:353`
- `impl Movie::thumbnails` — `crates/video-h264-preview/src/movie.rs:391`

### `crates/locus-geospatial-codecs` (11 undocumented public)

- `impl FileGdbShapeType::from_u32` — `crates/locus-geospatial-codecs/src/formats/filegdb.rs:27`
- `impl Point2D::new` — `crates/locus-geospatial-codecs/src/geometry.rs:13`
- `impl Point3D::new` — `crates/locus-geospatial-codecs/src/geometry.rs:27`
- `impl Point3D::to_2d` — `crates/locus-geospatial-codecs/src/geometry.rs:31`
- `impl Polygon2D::new` — `crates/locus-geospatial-codecs/src/geometry.rs:44`
- `impl AttributeValue::as_f64` — `crates/locus-geospatial-codecs/src/telemetry/query.rs:16`
- `impl AttributeValue::as_str` — `crates/locus-geospatial-codecs/src/telemetry/query.rs:24`
- `impl Compression::from_u8` — `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:16`
- `impl TileType::from_u8` — `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:39`
- `impl WkbReader<'a>::new` — `crates/locus-geospatial-codecs/src/wkb.rs:15`
- `impl WkbReader<'a>::read_geometry` — `crates/locus-geospatial-codecs/src/wkb.rs:75`

### `crates/timeline-editing` (11 undocumented public)

- `impl Tick::checked_add` — `crates/timeline-editing/src/timebase.rs:70`
- `impl Tick::checked_sub` — `crates/timeline-editing/src/timebase.rs:74`
- `impl Tick::max` — `crates/timeline-editing/src/timebase.rs:90`
- `impl Tick::min` — `crates/timeline-editing/src/timebase.rs:94`
- `impl TimeRange::new` — `crates/timeline-editing/src/timebase.rs:107`
- `impl TimeRange::end` — `crates/timeline-editing/src/timebase.rs:111`
- `impl TimeRange::is_empty` — `crates/timeline-editing/src/timebase.rs:117`
- `impl FrameRate::new` — `crates/timeline-editing/src/timebase.rs:148`
- `impl FrameRate::frame_duration` — `crates/timeline-editing/src/timebase.rs:165`
- `impl ClipSpan::new` — `crates/timeline-editing/src/transition.rs:12`
- `impl TransitionSpan::checked_end` — `crates/timeline-editing/src/transition.rs:40`

### `crates/pcm-device-output` (10 undocumented public)

- `impl DeviceOutput::enqueue` — `crates/pcm-device-output/src/device.rs:171`
- `impl DeviceOutput::play` — `crates/pcm-device-output/src/device.rs:193`
- `impl DeviceOutput::stop` — `crates/pcm-device-output/src/device.rs:196`
- `impl DeviceOutput::binding` — `crates/pcm-device-output/src/device.rs:209`
- `impl DeviceOutput::cancel` — `crates/pcm-device-output/src/device.rs:212`
- `impl DeviceOutput::consumed_frames` — `crates/pcm-device-output/src/device.rs:215`
- `impl DeviceOutput::status` — `crates/pcm-device-output/src/device.rs:218`
- `impl Queue::new` — `crates/pcm-device-output/src/queue.rs:31`
- `impl Queue::enqueue` — `crates/pcm-device-output/src/queue.rs:80`
- `impl Queue::ready` — `crates/pcm-device-output/src/queue.rs:122`

### `crates/raster-blend-operators` (10 undocumented public)

- `impl BlendMode::label` — `crates/raster-blend-operators/src/lib.rs:122`
- `linear_dodge_rgb` — `crates/raster-blend-operators/src/lib.rs:363`
- `linear_burn_rgb` — `crates/raster-blend-operators/src/lib.rs:367`
- `linear_light_rgb` — `crates/raster-blend-operators/src/lib.rs:371`
- `pin_light_rgb` — `crates/raster-blend-operators/src/lib.rs:375`
- `pegtop_light_rgb` — `crates/raster-blend-operators/src/lib.rs:379`
- `soft_light_rgb` — `crates/raster-blend-operators/src/lib.rs:383`
- `exclusion_rgb` — `crates/raster-blend-operators/src/lib.rs:387`
- `difference_rgb` — `crates/raster-blend-operators/src/lib.rs:391`
- `divide_rgb` — `crates/raster-blend-operators/src/lib.rs:395`

### `crates/pixel-buffer-output` (9 undocumented public)

- `vs_main` — `crates/pixel-buffer-output/shaders/scale.wgsl:14`
- `fs_main` — `crates/pixel-buffer-output/shaders/scale.wgsl:29`
- `vs_main` — `crates/pixel-buffer-output/shaders/scale_fill.wgsl:14`
- `fs_main` — `crates/pixel-buffer-output/shaders/scale_fill.wgsl:30`
- `create_backing_texture` — `crates/pixel-buffer-output/src/builder.rs:464`
- `impl ScalingRenderer::new` — `crates/pixel-buffer-output/src/renderers.rs:23`
- `impl ScalingRenderer::resize` — `crates/pixel-buffer-output/src/renderers.rs:295`
- `impl ScalingMatrix::new` — `crates/pixel-buffer-output/src/renderers.rs:317`
- `impl ScalingMatrix::clip_rect` — `crates/pixel-buffer-output/src/renderers.rs:382`

### `crates/gpu-command-profiler` (7 undocumented public)

- `collatz_iterations` — `crates/gpu-command-profiler/examples/compute_shader.wgsl:11`
- `main` — `crates/gpu-command-profiler/examples/compute_shader.wgsl:34`
- `vs_main` — `crates/gpu-command-profiler/examples/shader.wgsl:7`
- `fs_main` — `crates/gpu-command-profiler/examples/shader.wgsl:27`
- `impl GpuProfilerSettings::validate` — `crates/gpu-command-profiler/src/profiler_settings.rs:50`
- `create_tracy_gpu_client` — `crates/gpu-command-profiler/src/tracy.rs:3`
- `create_device` — `crates/gpu-command-profiler/tests/src/mod.rs:7`

### `crates/pixel-window-render-adapter` (7 undocumented public)

- `impl PixelsRendererOptions::new` — `crates/pixel-window-render-adapter/src/lib.rs:42`
- `impl PixelsRendererOptions::base_color` — `crates/pixel-window-render-adapter/src/lib.rs:49`
- `impl PixelsRendererOptions::composite_alpha_mode` — `crates/pixel-window-render-adapter/src/lib.rs:53`
- `impl PixelsWindowRenderer<Renderer>::new` — `crates/pixel-window-render-adapter/src/lib.rs:91`
- `impl PixelsWindowRenderer<Renderer>::with_renderer` — `crates/pixel-window-render-adapter/src/lib.rs:95`
- `impl PixelsWindowRenderer<Renderer>::with_options` — `crates/pixel-window-render-adapter/src/lib.rs:104`
- `impl PixelsWindowRenderer<Renderer>::with_options_and_renderer` — `crates/pixel-window-render-adapter/src/lib.rs:113`

### `crates/vector-cpu-render-adapter` (6 undocumented public)

- `convert_filter` — `crates/vector-cpu-render-adapter/src/filters.rs:9`
- `convert_filter_effect` — `crates/vector-cpu-render-adapter/src/filters.rs:23`
- `impl ImageCache::new` — `crates/vector-cpu-render-adapter/src/image_cache.rs:57`
- `impl VelloCpuScenePainter::new` — `crates/vector-cpu-render-adapter/src/scene.rs:20`
- `impl VelloCpuScenePainter::with_image_cache_config` — `crates/vector-cpu-render-adapter/src/scene.rs:24`
- `impl VelloCpuScenePainter::finish` — `crates/vector-cpu-render-adapter/src/scene.rs:60`

### `crates/vector-glyph-shaping` (6 undocumented public)

- `impl BidiRun::is_rtl` — `crates/vector-glyph-shaping/src/bidi.rs:64`
- `impl SGlyph::char_glyph` — `crates/vector-glyph-shaping/src/composer.rs:30`
- `impl SGlyph::space_glyph` — `crates/vector-glyph-shaping/src/composer.rs:37`
- `set_hyphenation_exceptions` — `crates/vector-glyph-shaping/src/hyphen.rs:50`
- `hyphenation_exceptions` — `crates/vector-glyph-shaping/src/hyphen.rs:54`
- `hyphenate_word` — `crates/vector-glyph-shaping/src/hyphen.rs:142`

### `crates/media-test-support` (5 undocumented public)

- `impl Rgba8::new` — `crates/media-test-support/src/golden.rs:26`
- `impl Rgba8::get` — `crates/media-test-support/src/golden.rs:30`
- `read_png` — `crates/media-test-support/src/golden.rs:79`
- `write_png` — `crates/media-test-support/src/golden.rs:84`
- `impl Tolerance::accepts` — `crates/media-test-support/src/golden.rs:165`

### `crates/raster-spatial-operators` (4 undocumented public)

- `impl Kernel2D::new` — `crates/raster-spatial-operators/src/convolve.rs:17`
- `impl Kernel2D::with_origin` — `crates/raster-spatial-operators/src/convolve.rs:28`
- `impl OctreeQuantizer::new` — `crates/raster-spatial-operators/src/octree.rs:72`
- `impl OctreeQuantizer::insert` — `crates/raster-spatial-operators/src/octree.rs:79`

### `crates/media-ogg` (2 undocumented public)

- `impl Codec::name` — `crates/media-ogg/src/lib.rs:148`
- `impl VorbisInfo::parse` — `crates/media-ogg/src/lib.rs:473`

### `crates/raster-selection-morphology` (1 undocumented public)

- `par_rows` — `crates/raster-selection-morphology/src/lib.rs:251`

### `crates/vector-hybrid-shaders` (1 undocumented public)

- `check` — `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:10`

