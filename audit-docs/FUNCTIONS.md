# Function Index

Every function/method found in the codebase, alphabetical.

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl CellRef::a1` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:25` | - | `A1`-style text, e.g. `B7`. |
| `impl RangeRef::a1` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:225` | - | `A1:B2`, `A1` for single cells, `1:3` for full rows, `A:C` for full columns. |
| `a_pie_is_its_arc_closed_through_the_centre` | 0 | `crates/vector-locus-geometry/src/shapes.rs:243` | - |  |
| `a_star_has_ten_corners_cut_as_circles` | 0 | `crates/vector-locus-geometry/src/corners.rs:201` | - |  |
| `aabb_hit` | 5 | `crates/surface-pbr-studio/src/bvh.rs:133` | computational geometry, 3d |  |
| `impl SampleEntry::aac` | 3 | `crates/media-isobmff/src/codec.rs:1316` | - | AAC entry (`mp4a` + `esds`) from an AudioSpecificConfig. |
| `aac_esds_roundtrip` | 0 | `crates/media-isobmff/src/codec.rs:1636` | - |  |
| `aac_priming_edit` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:210` | - |  |
| `abaqus_deck_format_test` | 0 | `crates/kinetics-continuum-multiphysics/src/interchange/abaqus.rs:56` | - |  |
| `impl ApplicationHandler<()> for State::about_to_wait` | 2 | `crates/gpu-command-profiler/examples/demo.rs:318` | - |  |
| `abs` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:21` | - |  |
| `impl Complex32::abs` | 1 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:32` | - |  |
| `absolute_grids` | 0 | `crates/audio-timebase/src/grid.rs:245` | - |  |
| `absurd_counts_are_bounded` | 0 | `crates/media-isobmff/src/demux.rs:1225` | - |  |
| `impl PortReq::accepts` | 2 | `crates/kinetics-block-simulation/src/signal.rs:78` | - |  |
| `impl Tolerance::accepts` | 2 | `crates/media-test-support/src/golden.rs:165` | - |  |
| `accepts_fragment_shader_without_struct_values` | 0 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:318` | 3d |  |
| `accepts_static_indexing` | 0 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:56` | - |  |
| `impl JournalLedger::account_balances` | 1 | `crates/chronicle-fiduciary-ledger/src/journal.rs:186` | - | Calculates net balances for all accounts: positive = net debit, negative = net credit. |
| `aces_cc_decode` | 1 | `crates/radiance-color-grading/src/log.rs:308` | - |  |
| `aces_cc_encode` | 1 | `crates/radiance-color-grading/src/log.rs:298` | - |  |
| `aces_cct_decode` | 1 | `crates/radiance-color-grading/src/log.rs:327` | - |  |
| `aces_cct_encode` | 1 | `crates/radiance-color-grading/src/log.rs:319` | - |  |
| `acknowledged_reset_releases_but_a_later_play_invalidates_old_ack` | 0 | `crates/pcm-device-output/src/device_slot_tests.rs:28` | - |  |
| `impl AssetHost<'_>::acoustics_discourse_chronicle_ext_op` | 3 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:120` | - | Categorical operations dispatch for acoustics, discourse, and chronicle extensions. |
| `impl AssetHost<'_>::acoustics_ext` | 3 | `crates/asset-production-vibe/src/acoustics_ops.rs:9` | - | Categorical operations dispatch for the `acoustics` category extensions. |
| `impl SurfaceRenderer<'s>::acquire` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:249` | - | Acquire the next surface frame. |
| `impl DeviceSlot<'a>::acquire` | 2 | `crates/pcm-device-output/src/device.rs:28` | - |  |
| `impl SurfaceRenderer<'s>::acquire_reconfiguring_if_stale` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:258` | - | Acquire a usable surface texture. A `Suboptimal` texture is still |
| `impl AxisRom::active_range` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:139` | - | The range a pose may use: the narrowed range when injured, otherwise the healthy range. |
| `impl Scene::active_rect` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:256` | 2d |  |
| `impl CowTerrainTileGrid::active_samples` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:32` | - |  |
| `impl CowTerrainTileGrid::active_samples` | 1 | `crates/terrain-brushes/src/lib.rs:213` | - |  |
| `actual_gaps_are_budgeted_across_wrap_nonreference_and_mmco5` | 0 | `crates/video-h264-preview/src/lib.rs:471` | - |  |
| `actual_idr_samples_decode_to_different_srgb_frames` | 0 | `crates/video-h264-preview/src/lib.rs:728` | - |  |
| `impl Pixels<'win>::adapter` | 1 | `crates/pixel-buffer-output/src/lib.rs:343` | - | Returns a reference of the `wgpu` adapter used by the crate. |
| `impl DeviceHandle::adapter` | 1 | `crates/vector-compute-renderer/src/util.rs:217` | - | Returns the adapter associated with the device. |
| `impl StreamInstant::add` | 2 | `crates/audio-device-cpal/src/lib.rs:435` | - | Returns the instant in time after the given duration has passed. |
| `impl Cpx::add` | 2 | `crates/audio-opus-codec/src/celt/mdct.rs:19` | - |  |
| `impl std::ops::Add for Tick::add` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:53` | - |  |
| `impl Bounds2::add` | 2 | `crates/drafting-geometry/src/bounds.rs:34` | 2d |  |
| `impl Add for Vec2::add` | 2 | `crates/drafting-geometry/src/vec.rs:102` | 2d |  |
| `impl Add for Vec3::add` | 2 | `crates/drafting-geometry/src/vec.rs:181` | 3d |  |
| `impl Complex32::add` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:52` | - |  |
| `impl Add for DVec2::add` | 2 | `crates/kinetics-garment-draping/src/math.rs:59` | - |  |
| `impl Add for DVec3::add` | 2 | `crates/kinetics-garment-draping/src/math.rs:147` | - |  |
| `impl Add for Jet<N>::add` | 2 | `crates/kinetics-garment-draping/src/math.rs:316` | - |  |
| `add` | 2 | `crates/kinetics-garment-draping/src/math.rs:376` | - |  |
| `impl std::ops::Add for Tick::add` | 2 | `crates/kinetics-motion-compositor/src/time.rs:41` | - |  |
| `add` | 2 | `crates/manifold-bim-csg/src/csg.rs:10` | - |  |
| `impl Acc::add` | 3 | `crates/radiance-color-grading/src/scopes/summary.rs:24` | - |  |
| `impl Add for Vec2::add` | 2 | `crates/surface-pbr-studio/src/math.rs:55` | - |  |
| `impl Add for Vec3::add` | 2 | `crates/surface-pbr-studio/src/math.rs:184` | - |  |
| `impl std::ops::Add for Vec2::add` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:19` | - |  |
| `add_2d` | 2 | `crates/manifold-bim-csg/src/parametric.rs:26` | - |  |
| `impl ChartOfAccounts::add_account` | 5 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:50` | - |  |
| `add_anchor_points` | 1 | `crates/vector-locus-pathops/src/pathops/edit.rs:264` | - | Add an anchor point at the midpoint of every segment. |
| `add_anchor_points_doubles_segments` | 0 | `crates/vector-locus-pathops/src/pathops/edit.rs:425` | - |  |
| `impl AddAssign for Vec2::add_assign` | 2 | `crates/drafting-geometry/src/vec.rs:107` | 2d |  |
| `impl AddAssign for DVec3::add_assign` | 2 | `crates/kinetics-garment-draping/src/math.rs:154` | - |  |
| `impl AddAssign for Vec3::add_assign` | 2 | `crates/surface-pbr-studio/src/math.rs:191` | - |  |
| `add_blend_chain` | 2 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:125` | - |  |
| `impl Model::add_block` | 3 | `crates/kinetics-block-simulation/src/model.rs:39` | - |  |
| `add_chain` | 2 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:115` | - |  |
| `impl AutoLayoutFrame::add_child` | 2 | `crates/layout-box-autolayout/src/frame.rs:145` | - |  |
| `impl SliceDecoder<'a>::add_chroma` | 1 | `crates/video-h264-codec/src/slicedec.rs:948` | - |  |
| `impl ChangeDag::add_comment` | 5 | `crates/kinetics-schedule-cpm/src/change_dag.rs:216` | - |  |
| `impl WgpuEngine::add_compute_shader` | 6 | `crates/vector-compute-renderer/src/wgpu_engine.rs:248` | 3d | Add a shader. |
| `impl WhitegumMeshBuilder::add_curved_tube` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:99` | - | Add a continuous, welded multi-station curved tube with parallel-transported ring frames. |
| `impl Project::add_dependency` | 5 | `crates/kinetics-schedule-cpm/src/model.rs:592` | - |  |
| `add_disconnect_listener` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:449` | - | Register the on-disconnect callback. |
| `impl SpatialGraph::add_edge` | 6 | `crates/geodesy-spatial-analysis/src/network_topology.rs:51` | graph |  |
| `impl ZipArchiveWriter::add_file` | 3 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:55` | - |  |
| `impl WhitegumMeshBuilder::add_foliage_lobe` | 8 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:205` | - | Add an ellipsoid or sculpted foliage cluster lobe with organic surface modulation. |
| `impl PbrLayerStack::add_layer` | 2 | `crates/surface-pbr-studio/src/compose.rs:88` | - |  |
| `impl Atlases::add_layer_atlas` | 2 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:88` | - |  |
| `impl SliceDecoder<'a>::add_luma_block4` | 2 | `crates/video-h264-codec/src/slicedec.rs:920` | - |  |
| `impl SliceDecoder<'a>::add_luma_block8` | 2 | `crates/video-h264-codec/src/slicedec.rs:934` | - |  |
| `impl SpatialGraph::add_node` | 2 | `crates/geodesy-spatial-analysis/src/network_topology.rs:44` | graph |  |
| `impl NodeNetwork::add_node` | 3 | `crates/layout-nodegraph-compiler/src/compiler/node.rs:73` | graph |  |
| `impl ProtoNetwork::add_node` | 3 | `crates/layout-nodegraph-compiler/src/compiler/proto.rs:37` | graph |  |
| `impl Doc::add_page` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:315` | - |  |
| `impl MxfWriter<W>::add_picture_track` | 4 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:102` | - |  |
| `impl GeodesicRuler::add_point` | 3 | `crates/drafting-geometry/src/geodesic_ruler.rs:146` | - |  |
| `impl GpxTracklog::add_point` | 2 | `crates/radiance-camera-raw/src/metadata.rs:61` | - |  |
| `impl IntBbox::add_pt` | 2 | `crates/vector-compute-shaders/src/cpu/flatten.rs:540` | 2d |  |
| `impl WhitegumMeshBuilder::add_quad` | 5 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:93` | - |  |
| `impl WgpuEngine::add_render_shader` | 10 | `crates/vector-compute-renderer/src/wgpu_engine.rs:317` | 3d |  |
| `impl Project::add_resource` | 5 | `crates/kinetics-schedule-cpm/src/model.rs:617` | - |  |
| `impl Figure::add_series` | 2 | `crates/discourse-scientific-matlang/src/plot.rs:148` | - |  |
| `impl Workbook::add_sheet` | 2 | `crates/chronicle-tabular-engine/src/calc/model.rs:172` | - |  |
| `impl MxfWriter<W>::add_sound_track` | 4 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:118` | - |  |
| `impl Project::add_sprint` | 6 | `crates/kinetics-schedule-cpm/src/model.rs:660` | - |  |
| `add_synthetic_audio_edit` | 1 | `crates/video-h264-preview/src/lib.rs:630` | - |  |
| `impl Project::add_task` | 4 | `crates/kinetics-schedule-cpm/src/model.rs:551` | - |  |
| `impl Mp4Writer<W>::add_timecode_track` | 4 | `crates/media-isobmff/src/mux.rs:546` | - | Add a QuickTime timecode track referenced by `video_track`. Its single sample (the start |
| `impl Mp4Writer<W>::add_track` | 2 | `crates/media-isobmff/src/mux.rs:535` | - | Add a track; returns its index (track ids are index + 1). |
| `impl FragmentedWriter<W>::add_track` | 2 | `crates/media-isobmff/src/mux.rs:745` | - | Add a track (before the first fragment). PCM is not supported in fragments. |
| `add_tree` | 3 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:140` | - |  |
| `impl WhitegumMeshBuilder::add_triangle` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:87` | - |  |
| `impl WhitegumMeshBuilder::add_vertex` | 5 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:77` | 3d |  |
| `impl Model::add_wire` | 5 | `crates/kinetics-block-simulation/src/model.rs:53` | - |  |
| `adjust` | 4 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:72` | - |  |
| `adjust_ref` | 2 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:92` | - |  |
| `impl GeometryLimits::admit` | 4 | `crates/profile-authoring/src/geometry.rs:31` | - |  |
| `impl CowTerrainTileGrid::admit` | 2 | `crates/terrain-brushes/adaptation/grid-impl.rs:39` | - |  |
| `impl CowTerrainTileGrid::admit` | 2 | `crates/terrain-brushes/src/lib.rs:220` | - |  |
| `impl GapBudget::admit` | 6 | `crates/video-h264-preview/src/lib.rs:26` | - | Mirrors Decoder::start_picture and submit_pending, including a gap on |
| `admit_audio_edit` | 4 | `crates/video-h264-preview/src/lib.rs:174` | - |  |
| `impl AssetHost<'_>::admit_encoded` | 3 | `crates/asset-production-vibe/src/lib.rs:198` | - | Admit bytes the production host already holds. The files producer accepts the handle. |
| `admit_sample_table` | 2 | `crates/video-h264-preview/src/lib.rs:186` | - | Bound every table allocation before passing the input to the MP4 parser. |
| `impl LatticeProfile::admits_16bit` | 1 | `crates/asset-production-vibe/src/graphics.rs:36` | - | Named origin profiles stay 8-bit; default/2k/16-bit admit 16-bit codecs. |
| `admits_complete_single_stream_with_final_trim` | 0 | `crates/media-ogg/src/strict_opus.rs:140` | - |  |
| `admitted_operations_match_original_same_platform_float_behavior` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:290` | - |  |
| `adobe_rgb` | 0 | `crates/color-transforms/src/icc/builtin.rs:91` | - |  |
| `adts_roundtrip` | 0 | `crates/audio-aac-codec/src/config.rs:425` | - |  |
| `impl Ttf::advance` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:149` | - | Advance width of a glyph in font units. |
| `impl Cursor::advance` | 1 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:128` | - | Advance to the next round. |
| `impl Oscillator::advance_sample` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:37` | - |  |
| `impl AssetHost<'_>::advect_m64_grid` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/m64_continuum.rs:71` | 3d | Solves divergence-free pressure projection via Jacobi Poisson iterations, |
| `advect_m64_grid_strata_kriging_erosion_two_produces_match` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/m64_continuum.rs:532` | - |  |
| `affine_and_conjugation` | 0 | `crates/vector-locus-geometry/src/projective.rs:146` | - |  |
| `impl Homography::affine_at` | 2 | `crates/vector-locus-geometry/src/projective.rs:99` | - | The affine map that matches `self` to first order at `p` (its derivative there). |
| `affine_at_is_the_derivative` | 0 | `crates/vector-locus-geometry/src/projective.rs:156` | - |  |
| `impl SchedulePoint::after` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:402` | - | Return the first occurrence of `stage` strictly after this point. |
| `impl SchedulePoint::after_or_at` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:417` | - | Return the first occurrence of `stage` at or after this point. |
| `impl Resources::after_render` | 3 | `crates/vector-hybrid-renderer/src/text.rs:131` | 2d |  |
| `aggregate_limits_include_conversion_and_quantization` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:39` | - |  |
| `align_cues` | 2 | `crates/chronicle-timeline-interchange/src/captions/align.rs:29` | - | Align all cues in a caption document according to the specified options. |
| `impl BitWriter::align_zero` | 1 | `crates/bitstream-codec/src/lib.rs:223` | - | Pad with zero bits to a byte boundary. |
| `impl ChartOfAccounts::all` | 1 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:84` | - |  |
| `impl RangeRef::all` | 0 | `crates/chronicle-tabular-engine/src/core/addr.rs:161` | - |  |
| `impl DebugLayers::all` | 0 | `crates/vector-compute-renderer/src/debug.rs:87` | - | A `DebugLayers` with all layers enabled. |
| `impl AaSupport::all` | 0 | `crates/vector-compute-renderer/src/lib.rs:218` | 3d | Support every anti-aliasing method. |
| `all_15_warp_presets_evaluate_without_nan` | 0 | `crates/vector-locus-pathops/src/effects/warp.rs:146` | - |  |
| `all_empty_samples_roundtrip` | 0 | `crates/media-isobmff/tests/roundtrip.rs:211` | - | Found by `progressive_roundtrip`: only zero-byte samples must not be written as uniform |
| `all_fixture_names` | 0 | `crates/media-isobmff/tests/common/mod.rs:143` | - |  |
| `all_ontology_nquins_verify_five_field_parity_and_spans` | 0 | `crates/asset-production-vibe/src/ontology/tests.rs:5` | semantics |  |
| `impl CompilerOptions::all_optimizations` | 0 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:20` | - |  |
| `impl StripBufferArena::alloc` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:989` | - |  |
| `alloc_cmd` | 1 | `crates/vector-compute-shaders/shader/coarse.wgsl:68` | - |  |
| `impl TileState::alloc_cmd` | 5 | `crates/vector-compute-shaders/src/cpu/coarse.rs:47` | - |  |
| `impl Atlases::allocate_layer` | 2 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:50` | - |  |
| `impl Cursor::allocate_layer` | 2 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:51` | - |  |
| `trait AtlasExt::allocate_region` | 3 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:196` | - |  |
| `impl AtlasExt for Atlas::allocate_region` | 3 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:206` | - |  |
| `impl Cursor::allocate_reusing` | 2 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:94` | - | Try to accommodate the given layer allocation request, possibly advancing the round counter |
| `allocates_short_deterministic_names` | 0 | `crates/vector-hybrid-shaders/src/minify.rs:330` | - |  |
| `impl AllocatedTextureRegion::allocation_region` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:185` | - |  |
| `impl LayerAllocationRequest::allocation_size` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:136` | - |  |
| `impl RegionProps::allocation_size` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:152` | - | Size of the atlas allocation needed to hold the region and its padding. |
| `impl SyncRole::allows_command` | 2 | `crates/kinetics-schedule-cpm/src/change_dag.rs:71` | - |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::alpha_mode` | 2 | `crates/pixel-buffer-output/src/builder.rs:231` | - | Sets the alpha mode. |
| `impl Targets::amplitude` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:120` | - |  |
| `impl Psy::analyse` | 3 | `crates/audio-aac-codec/src/encoder/psy.rs:103` | - | Analyse a spectrum: 1024 lines (long) or 8×128 (short). |
| `impl Encoder::analyse_element` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:457` | - |  |
| `analyze` | 1 | `crates/discourse-academic-tex/src/texparse/stats.rs:41` | - | Compute stats for `.tex` source `src` in single O(n) pass. |
| `impl AssetHost<'_>::analyze_ebu_r128_camelot_daw` | 2 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:135` | - | 139: `hypermedia.acoustics.analyze_ebu_r128_camelot_daw` |
| `anchor_a1` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:48` | - |  |
| `impl PathData::anchor_count` | 1 | `crates/vector-locus-geometry/src/path.rs:363` | - |  |
| `impl TraceResult::anchor_count` | 1 | `crates/vector-radiance-trace/src/lib.rs:316` | - |  |
| `impl PathData::anchor_mut` | 3 | `crates/vector-locus-geometry/src/path.rs:488` | - |  |
| `anchor_survives_prefix_shift` | 0 | `crates/discourse-academic-tex/src/review/suggest.rs:117` | - |  |
| `impl PathData::anchors` | 1 | `crates/vector-locus-geometry/src/path.rs:485` | - | Iterate (subpath index, anchor index, anchor). |
| `anchors_serialize_compactly_and_read_v1_maps` | 0 | `crates/vector-locus-geometry/src/path.rs:519` | - |  |
| `ang_between` | 2 | `crates/locus-parametric-solver/src/solver/auto.rs:97` | 2d |  |
| `impl Vec2::angle` | 1 | `crates/drafting-geometry/src/vec.rs:43` | - |  |
| `angle_deg` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:490` | - |  |
| `impl Corner::angle_deg` | 1 | `crates/vector-locus-geometry/src/corners.rs:61` | - | The angle between the sides, in degrees (90 at a rectangle's corners). |
| `angle_in_sweep` | 3 | `crates/drafting-geometry/src/lib.rs:57` | - | True when angle `a` lies on the CCW sweep from `start` to `end` (inclusive, with tolerance). |
| `angle_normalization` | 0 | `crates/vector-locus-geometry/src/lib.rs:100` | - |  |
| `impl Vec2::angle_to` | 2 | `crates/drafting-geometry/src/vec.rs:46` | 2d |  |
| `angles_normalize` | 0 | `crates/drafting-geometry/src/tests.rs:8` | - |  |
| `impl GeodesicRuler::angular_distance_degrees` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:301` | - |  |
| `ankle_joint` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:304` | - |  |
| `annexb_nals` | 1 | `crates/bitstream-codec/src/lib.rs:282` | - | Split an Annex-B byte stream (start codes `00 00 01` / `00 00 00 01`) into NAL units (escaped payloads). |
| `annexb_split` | 0 | `crates/bitstream-codec/src/lib.rs:389` | - |  |
| `annual_rate_bp` | 3 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:36` | - | Computes the effective annual rate in basis points (bp) from method and life. |
| `anti_collapse` | 12 | `crates/audio-opus-codec/src/celt/bands.rs:825` | - | Anti-collapse noise injection (RFC 6716 §4.3.5). |
| `impl VelloRendererOptions::antialiasing_method` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:116` | - |  |
| `impl References::any_uploading` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:142` | - |  |
| `anyrender_paint_to_vello_hybrid_paint` | 3 | `crates/vector-hybrid-render-adapter/src/scene.rs:19` | - |  |
| `impl RadianceModel::api_name` | 1 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:30` | - |  |
| `impl FiduciaryAuditTrail::append` | 8 | `crates/chronicle-fiduciary-ledger/src/audit.rs:66` | - | Appends a new verified audit event to the tamper-evident chain. |
| `impl DtvccWindow::append` | 2 | `crates/chronicle-timeline-interchange/src/captions/cea708.rs:68` | - |  |
| `impl Scene::append` | 3 | `crates/vector-compute-renderer/src/scene.rs:444` | - | Appends a child scene. |
| `append_breaking_word` | 8 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:12` | - | Append `word` with per-character breaks so wrap width includes pair kerning. |
| `append_minified_code` | 2 | `crates/vector-hybrid-shaders/src/minify.rs:80` | - | Appends minified code without merging tokens across the append boundary. |
| `append_path_el_to_sk_path` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:1357` | - |  |
| `apple_log_decode` | 1 | `crates/color-transforms/src/log.rs:243` | - |  |
| `apple_log_decode` | 1 | `crates/radiance-color-grading/src/log.rs:266` | - |  |
| `apple_log_encode` | 1 | `crates/color-transforms/src/log.rs:233` | - |  |
| `apple_log_encode` | 1 | `crates/radiance-color-grading/src/log.rs:256` | - |  |
| `apply` | 2 | `crates/color-transforms/src/icc/math.rs:28` | - |  |
| `impl Lut1d::apply` | 2 | `crates/color-transforms/src/lut.rs:35` | - |  |
| `impl Lut3d::apply` | 2 | `crates/color-transforms/src/lut.rs:104` | - | Tetrahedral interpolation. |
| `impl Lut::apply` | 2 | `crates/color-transforms/src/lut.rs:438` | - |  |
| `impl ToneMap::apply` | 2 | `crates/color-transforms/src/transform.rs:147` | - |  |
| `impl InputTransform::apply` | 2 | `crates/color-transforms/src/transform.rs:237` | 2d | The per-pixel stage (input: the table's output). |
| `impl OutputTransform::apply` | 2 | `crates/color-transforms/src/transform.rs:316` | numerics | Working linear → target linear (SDR: 1.0 = white; HDR: working units). |
| `impl Xf::apply` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:95` | - |  |
| `impl Mat3::apply` | 2 | `crates/drafting-geometry/src/mat.rs:67` | 2d |  |
| `impl Mat4::apply` | 2 | `crates/drafting-geometry/src/mat.rs:126` | 3d |  |
| `impl AscCdl::apply` | 3 | `crates/radiance-color-grading/src/grade.rs:144` | - | Forward ASC CDL: `out = clamp((in * slope + offset)^power)`. |
| `impl ColorWheels::apply` | 2 | `crates/radiance-color-grading/src/grade.rs:296` | - | Apply Lift/Gamma/Gain + Offset grading to an RGB triplet. |
| `impl Lut1d::apply` | 2 | `crates/radiance-color-grading/src/lut.rs:31` | - |  |
| `impl Lut3d::apply` | 2 | `crates/radiance-color-grading/src/lut.rs:102` | - | Tetrahedral 3D interpolation. |
| `impl Lut::apply` | 2 | `crates/radiance-color-grading/src/lut.rs:220` | - |  |
| `impl InputTransform::apply` | 2 | `crates/radiance-color-grading/src/transform.rs:97` | numerics | Transform an encoded source color triplet to sequence working linear light. |
| `impl OutputTransform::apply` | 2 | `crates/radiance-color-grading/src/transform.rs:135` | numerics | Transform sequence working linear color to target encoded display/export color. |
| `impl Transform::apply` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:144` | 2d |  |
| `impl Homography::apply` | 2 | `crates/vector-locus-geometry/src/projective.rs:39` | - | Map a point; `None` when it lands on or beyond the horizon (w ≤ 0). |
| `impl BoolOp::apply` | 3 | `crates/vector-locus-pathops/src/pathops/boolean.rs:29` | - |  |
| `apply3` | 2 | `crates/color-transforms/src/spaces.rs:126` | - |  |
| `apply3` | 2 | `crates/radiance-color-grading/src/spaces.rs:177` | - |  |
| `apply_age_morphism` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:1353` | - | Years lived and how old the person looks are separate. |
| `apply_antipop_ramp_down` | 1 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:86` | - | Applies a 64-sample anti-pop lookahead cosine ramp down to zero. |
| `apply_constraints` | 5 | `crates/layout-box-autolayout/src/constraints.rs:9` | - | Adjusts children positions and sizes when their parent changes size from `(ow, oh)` to `(nw, nh)`. |
| `apply_crater` | 6 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:66` | - | Apply a single impact crater cavity and raised rim onto a 2D heightfield. |
| `apply_dab` | 6 | `crates/surface-pbr-studio/src/paint.rs:46` | 2d | Applies a single dab centered at `(uv.x, uv.y)` with the current brush. |
| `apply_decoder` | 4 | `crates/audio-aac-codec/src/tns.rs:184` | - | Decoder: apply the all-pole TNS synthesis filters to `spec` (1024 lines; short windows at 128 stride). |
| `apply_dsp_compressor` | 3 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:100` | - | Soft-knee compressor / limiter insert. |
| `apply_ease` | 3 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:160` | - |  |
| `apply_easing` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:444` | - |  |
| `apply_edits` | 2 | `crates/discourse-academic-tex/src/review/ai_repair.rs:164` | - | Apply a sequence of edits bottom-up so line numbers remain stable. |
| `apply_encoder` | 4 | `crates/audio-aac-codec/src/tns.rs:214` | - | Encoder: apply the all-zero TNS analysis filters (exact inverse of [`apply_decoder`]). |
| `apply_eraser_dab` | 5 | `crates/surface-pbr-studio/src/paint.rs:98` | 2d | Erases pixels within a dab area. |
| `impl Dpb<F>::apply_mmcos` | 5 | `crates/video-h264-codec/src/dpb.rs:264` | - | Adaptive memory control (8.2.5.4). Returns true if MMCO 5 was executed. |
| `apply_morphology` | 5 | `crates/raster-selection-morphology/src/grayscale.rs:227` | - | Applies a morphological method to a 2D grayscale raster. |
| `apply_op_to_vertex` | 7 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:325` | 3d |  |
| `apply_op_to_vertex` | 8 | `crates/terrain-brushes/src/lib.rs:491` | 3d |  |
| `impl FilterPassBuilder<'a>::apply_pass_dimensions` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:520` | - | Compute and update source and destination sizes based on the pass kind. |
| `impl SliceDecoder<'a>::apply_qp_delta` | 2 | `crates/video-h264-codec/src/slicedec.rs:1093` | - | Update QP after mb_qp_delta. |
| `impl ToneMap::apply_rgb` | 3 | `crates/radiance-color-grading/src/transform.rs:63` | numerics | Apply tone mapping to an RGB triplet in linear working units. |
| `trait BlendTarget::apply_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:229` | - |  |
| `impl BlendTarget for BlendOperator::apply_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:233` | - |  |
| `impl BlendTarget for BlendMode::apply_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:239` | - |  |
| `apply_semantic_redaction` | 3 | `crates/discourse-pdf-compliance/src/redact/verifier.rs:32` | semantics | Applies irreversible, fail-closed redaction to an in-memory text buffer. |
| `impl CowTerrainTileGrid::apply_stroke_sample` | 2 | `crates/terrain-brushes/adaptation/grid-impl.rs:114` | - | Prepare all touched tile successors, then install atomically for this sample. |
| `impl CowTerrainTileGrid::apply_stroke_sample` | 2 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:190` | - |  |
| `impl CowTerrainTileGrid::apply_stroke_sample` | 2 | `crates/terrain-brushes/src/lib.rs:295` | - | Prepare all touched tile successors, then install atomically for this sample. |
| `apply_suggestion` | 2 | `crates/discourse-academic-tex/src/review/suggest.rs:79` | - | Apply a suggestion to `src`, returning the new source if uniquely anchored. |
| `apply_terraces` | 4 | `crates/terrain-erosion-solvers/src/fractal_terrace.rs:87` | - | Apply stratigraphic terracing to an entire 2D heightfield in-place. |
| `impl SubElementHooks::apply_to_rig_pose` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:308` | - | Applies the macro + sub-element hooks onto an 18-joint `FaunaSkeletalRig10d` PGA motor pose. |
| `impl Lut3d::apply_trilinear` | 2 | `crates/color-transforms/src/lut.rs:136` | - | Trilinear interpolation (kept for comparison; LUTs render tetrahedrally). |
| `impl Lut3d::apply_trilinear` | 2 | `crates/radiance-color-grading/src/lut.rs:146` | - | Trilinear 3D interpolation. |
| `impl Mat3::apply_vec` | 2 | `crates/drafting-geometry/src/mat.rs:70` | 2d |  |
| `approx_parabola_integral` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:47` | - |  |
| `approx_parabola_inv_integral` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:52` | - |  |
| `impl SampleEntry::apv` | 3 | `crates/media-isobmff/src/codec.rs:1303` | - | Advanced Professional Video entry (`apv1` + `apvC`). |
| `arabic_text_is_rtl` | 0 | `crates/vector-glyph-shaping/src/bidi.rs:181` | - |  |
| `impl Model::arc` | 3 | `crates/locus-parametric-solver/src/solver/model.rs:270` | 2d |  |
| `arc` | 7 | `crates/profile-authoring/src/profile.rs:74` | 2d |  |
| `arc` | 4 | `crates/vector-locus-geometry/src/shapes.rs:164` | - | Quarter-ellipse arc from `a` to `b`. |
| `arc_bounds_include_quadrants` | 0 | `crates/drafting-geometry/src/tests.rs:49` | - |  |
| `arc_handle` | 1 | `crates/vector-locus-geometry/src/corners.rs:126` | - | The handle length, per unit of radius, of a cubic drawing a circular arc that spans an angle |
| `arc_open_and_closed` | 0 | `crates/vector-locus-geometry/src/shapes.rs:342` | - |  |
| `arc_segments` | 3 | `crates/drafting-geometry/src/lib.rs:71` | - | Number of segments needed to approximate an arc of `radius` and `sweep` within `tol`. |
| `arc_segments_bounded` | 0 | `crates/drafting-geometry/src/tests.rs:192` | - |  |
| `arc_three_points_direction` | 0 | `crates/drafting-geometry/src/tests.rs:40` | - |  |
| `arc_to_bulge` | 1 | `crates/drafting-geometry/src/curve.rs:455` | - | Bulge for an arc traversed from `a` to `b` with the given sweep (signed: + = CCW). |
| `archetype_default_rules` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:266` | - | The archetype-class default rules, identical to the legacy |
| `impl Polyline::area` | 1 | `crates/drafting-geometry/src/curve.rs:511` | - | Signed area (CCW positive) of a closed polyline including bulge areas. |
| `impl Body::area` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:157` | - |  |
| `area` | 1 | `crates/manifold-bim-csg/src/csg.rs:415` | computational geometry | Surface area of a triangle mesh. |
| `impl SubPath::area` | 1 | `crates/vector-locus-geometry/src/path.rs:340` | - | Signed area via the shoelace formula on the Bézier path (positive = clockwise in y-down). |
| `area` | 1 | `crates/vector-locus-geometry/src/recognize.rs:144` | - |  |
| `area` | 1 | `crates/vector-locus-pathops/src/pathops/boolean.rs:53` | - | Total signed area of a path. |
| `area` | 1 | `crates/vector-locus-pathops/src/three_d/visibility.rs:36` | - |  |
| `impl AaSupport::area_only` | 0 | `crates/vector-compute-renderer/src/lib.rs:229` | - | Support only [`AaConfig::Area`]. |
| `area_text_layout_wraps_lines` | 0 | `crates/vector-glyph-shaping/src/layout.rs:266` | - |  |
| `impl Complex32::arg` | 1 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:37` | - |  |
| `arguments` | 1 | `crates/asset-production-vibe/src/lib.rs:85` | - |  |
| `impl Cur<'a>::array` | 1 | `crates/media-isobmff/src/bytes.rs:89` | - |  |
| `art_brush_warps_along_curve` | 0 | `crates/vector-surface-brush/src/art.rs:192` | - |  |
| `arxiv_body` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:68` | - |  |
| `arxiv_forms` | 0 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:156` | - |  |
| `trait Block::as_any` | 1 | `crates/kinetics-block-simulation/src/block.rs:93` | - |  |
| `impl Block for Integrator::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:66` | - |  |
| `impl Block for Derivative::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:122` | - |  |
| `impl Block for PidController::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:201` | - |  |
| `impl Block for TransferFcn::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:296` | - |  |
| `impl Block for TransportDelay::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:359` | - |  |
| `impl Block for UnitDelay::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:59` | - |  |
| `impl Block for ZeroOrderHold::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:114` | - |  |
| `impl Block for Gain::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:46` | - |  |
| `impl Block for Sum::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:108` | - |  |
| `impl Block for Product::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:170` | - |  |
| `impl Block for Saturation::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:216` | - |  |
| `impl Block for DeadZone::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:268` | - |  |
| `impl Block for Switch::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:318` | - |  |
| `impl Block for Relational::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:372` | - |  |
| `impl Block for LogicalOperator::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:439` | - |  |
| `impl Block for Abs::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:480` | - |  |
| `impl Block for Mux::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:44` | - |  |
| `impl Block for Demux::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:94` | - |  |
| `impl Block for Scope::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:62` | - |  |
| `impl Block for ToWorkspace::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:109` | - |  |
| `impl Block for Constant::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:38` | - |  |
| `impl Block for Step::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:82` | - |  |
| `impl Block for Sine::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:128` | - |  |
| `impl Block for Ramp::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:176` | - |  |
| `impl Block for Clock::as_any` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:211` | - |  |
| `trait Block::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/block.rs:94` | - |  |
| `impl Block for Integrator::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:69` | - |  |
| `impl Block for Derivative::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:125` | - |  |
| `impl Block for PidController::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:204` | - |  |
| `impl Block for TransferFcn::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:299` | - |  |
| `impl Block for TransportDelay::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:362` | - |  |
| `impl Block for UnitDelay::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:62` | - |  |
| `impl Block for ZeroOrderHold::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:117` | - |  |
| `impl Block for Gain::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:49` | - |  |
| `impl Block for Sum::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:111` | - |  |
| `impl Block for Product::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:173` | - |  |
| `impl Block for Saturation::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:219` | - |  |
| `impl Block for DeadZone::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:271` | - |  |
| `impl Block for Switch::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:321` | - |  |
| `impl Block for Relational::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:375` | - |  |
| `impl Block for LogicalOperator::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:442` | - |  |
| `impl Block for Abs::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:483` | - |  |
| `impl Block for Mux::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:47` | - |  |
| `impl Block for Demux::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:97` | - |  |
| `impl Block for Scope::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:65` | - |  |
| `impl Block for ToWorkspace::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:112` | - |  |
| `impl Block for Constant::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:41` | - |  |
| `impl Block for Step::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:85` | - |  |
| `impl Block for Sine::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:131` | - |  |
| `impl Block for Ramp::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:179` | - |  |
| `impl Block for Clock::as_any_mut` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:214` | - |  |
| `as_array` | 1 | `crates/chronicle-tabular-engine/src/functions/util.rs:56` | - |  |
| `impl ExternalTextureBindings::as_array` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:486` | - |  |
| `impl Param::as_bool` | 1 | `crates/kinetics-block-simulation/src/block.rs:45` | - |  |
| `impl ResourceProxy::as_buf` | 1 | `crates/vector-compute-renderer/src/recording.rs:283` | - |  |
| `impl GpuEncodedPaint::as_bytes` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:486` | - | Returns the byte representation of this paint. |
| `impl Delimiter::as_char` | 1 | `crates/chronicle-office-codecs/src/spreadsheet/csv.rs:12` | - |  |
| `impl SampleRate::as_f64` | 1 | `crates/audio-timebase/src/rate.rs:60` | - |  |
| `impl Rational::as_f64` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:57` | - |  |
| `impl Value::as_f64` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:163` | - |  |
| `impl FrameRate::as_f64` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:87` | - |  |
| `impl Value::as_f64` | 1 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:54` | - |  |
| `impl FrameRate::as_f64` | 1 | `crates/kinetics-motion-compositor/src/time.rs:65` | - |  |
| `impl AttributeValue::as_f64` | 1 | `crates/locus-geospatial-codecs/src/telemetry/query.rs:16` | - |  |
| `impl ResourceProxy::as_image` | 1 | `crates/vector-compute-renderer/src/recording.rs:290` | - |  |
| `impl Value::as_matrix` | 1 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:17` | numerics |  |
| `impl Tick::as_millis` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:46` | - |  |
| `impl StreamInstant::as_nanos` | 1 | `crates/audio-device-cpal/src/lib.rs:452` | - |  |
| `impl Value::as_num` | 1 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:26` | - |  |
| `impl Param::as_num` | 1 | `crates/kinetics-block-simulation/src/block.rs:23` | - |  |
| `impl Matrix::as_scalar` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:153` | - |  |
| `impl Tick::as_seconds` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:38` | - |  |
| `impl Tick::as_seconds` | 1 | `crates/kinetics-motion-compositor/src/time.rs:34` | - |  |
| `impl Data::as_slice` | 1 | `crates/audio-device-cpal/src/lib.rs:560` | - | Access the data as a slice of sample type `T`. |
| `impl CpuBinding<'_>::as_slice` | 1 | `crates/vector-compute-shaders/src/cpu.rs:128` | - |  |
| `impl Data::as_slice_mut` | 1 | `crates/audio-device-cpal/src/lib.rs:576` | - | Access the data as a slice of sample type `T`. |
| `impl CpuBinding<'_>::as_slice_mut` | 1 | `crates/vector-compute-shaders/src/cpu.rs:138` | - |  |
| `impl BehaviorMacro::as_str` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:91` | - |  |
| `impl Axis::as_str` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:70` | - |  |
| `impl SpecSource::as_str` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:29` | - |  |
| `impl RulesSource::as_str` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:103` | - |  |
| `impl DepreciationMethod::as_str` | 1 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:23` | - |  |
| `impl CellError::as_str` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:39` | - |  |
| `impl Value::as_str` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:172` | - |  |
| `impl Param::as_str` | 1 | `crates/kinetics-block-simulation/src/block.rs:38` | - |  |
| `impl SyncRole::as_str` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:35` | - |  |
| `impl ItemKind::as_str` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:43` | - |  |
| `impl TaskStatus::as_str` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:88` | - |  |
| `impl Priority::as_str` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:133` | - |  |
| `impl AttributeValue::as_str` | 1 | `crates/locus-geospatial-codecs/src/telemetry/query.rs:24` | - |  |
| `impl CpuBinding<'_>::as_tex` | 1 | `crates/vector-compute-shaders/src/cpu.rs:151` | - |  |
| `impl CpuBinding<'_>::as_typed` | 1 | `crates/vector-compute-shaders/src/cpu.rs:106` | - |  |
| `impl CpuBinding<'_>::as_typed_mut` | 1 | `crates/vector-compute-shaders/src/cpu.rs:116` | - |  |
| `impl FourCc::as_u32` | 1 | `crates/media-isobmff/src/bytes.rs:15` | - |  |
| `impl Param::as_vec` | 1 | `crates/kinetics-block-simulation/src/block.rs:30` | - |  |
| `asbd_from_config` | 2 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:43` | - |  |
| `asc_parse` | 0 | `crates/media-isobmff/src/codec.rs:1699` | - |  |
| `asc_roundtrip` | 0 | `crates/audio-aac-codec/src/config.rs:381` | - |  |
| `asc_with_pce` | 0 | `crates/audio-aac-codec/src/config.rs:392` | - |  |
| `asio_channel_slice` | 3 | `crates/audio-device-cpal/src/host/asio/stream.rs:686` | - | Shorthand for retrieving the asio buffer slice associated with a channel. |
| `asio_channel_slice_mut` | 3 | `crates/audio-device-cpal/src/host/asio/stream.rs:697` | - | Shorthand for retrieving the asio buffer slice associated with a channel. |
| `asio_ns_to_double` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:605` | - |  |
| `aspect_label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:511` | - |  |
| `aspect_proportion` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:536` | - |  |
| `aspect_proportions_and_labels` | 0 | `crates/multimodal-synthesis-schema/src/catalog.rs:624` | - |  |
| `impl AssetHost<'_>::assemble_species_corpus` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:282` | - | `qualia.species.assemble_corpus`: validate a species document and seal it as `.q42`. |
| `assert_bit_equal` | 4 | `crates/raster-blend-operators/src/migration_oracle.rs:121` | - |  |
| `assert_golden` | 5 | `crates/media-test-support/src/golden.rs:275` | - | [`check_golden`] that panics on failure. |
| `assert_pixels` | 2 | `crates/vector-cpu-render-adapter/src/scene.rs:264` | - |  |
| `assert_same_packets` | 2 | `crates/media-isobmff/tests/oracle_mux.rs:93` | - | Compare ffprobe's view of two files (per-stream packets: pts, dts, size, key flag). |
| `assert_same_streams` | 2 | `crates/media-isobmff/tests/oracle_mux.rs:107` | - |  |
| `assign` | 4 | `crates/discourse-academic-tex/src/texparse/magic.rs:98` | - |  |
| `assign_external_texture_slot` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:501` | - |  |
| `impl Project::assign_resource` | 4 | `crates/kinetics-schedule-cpm/src/model.rs:644` | - |  |
| `impl Lut3d::at` | 4 | `crates/color-transforms/src/lut.rs:78` | - |  |
| `impl Line::at` | 2 | `crates/drafting-geometry/src/curve.rs:31` | 2d |  |
| `impl Arc::at` | 2 | `crates/drafting-geometry/src/curve.rs:87` | 2d | Point at parameter `t` in 0..1 along the sweep. |
| `impl Segment::at` | 2 | `crates/drafting-geometry/src/curve.rs:373` | 2d | Point at parameter `t` (0..1) in segment direction. |
| `impl WallAxis::at` | 3 | `crates/manifold-bim-csg/src/parametric.rs:107` | - |  |
| `impl Lut3d::at` | 4 | `crates/radiance-color-grading/src/lut.rs:87` | - |  |
| `impl Grid::at` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:235` | - |  |
| `impl Vectorscope::at` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:439` | - |  |
| `impl FloatGrid::at` | 3 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:49` | - |  |
| `impl ArcPath::at` | 2 | `crates/vector-locus-geometry/src/arc.rs:47` | 2d | Point and unit tangent at distance `s` along the path. |
| `impl Ellipse::at_param` | 2 | `crates/drafting-geometry/src/curve.rs:244` | 2d |  |
| `impl Vec2::atan2` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:119` | - |  |
| `ath_db` | 1 | `crates/audio-aac-codec/src/encoder/psy.rs:28` | graph | Terhardt's threshold in quiet (dB SPL), capped at 40 dB so content near the band edge is not |
| `impl GlyphRunBackend<'a> for GpuGlyphRunBackend<'a>::atlas_cache` | 2 | `crates/vector-hybrid-renderer/src/text.rs:260` | - |  |
| `impl Resources::atlas_config` | 1 | `crates/vector-hybrid-renderer/src/text.rs:67` | - |  |
| `impl Resources::atlas_count` | 1 | `crates/vector-hybrid-renderer/src/text.rs:71` | - |  |
| `atlas_image` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:715` | - |  |
| `impl glifo::GlyphRenderer for Scene::atlas_image_source` | 2 | `crates/vector-hybrid-renderer/src/text.rs:358` | - |  |
| `impl WebGlRenderer::atlas_info` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:818` | - | Returns the current allocation state of the image/glyph atlas texture. |
| `impl glifo::GlyphRenderer for Scene::atlas_paint_transform` | 2 | `crates/vector-hybrid-renderer/src/text.rs:363` | - |  |
| `atlas_skips_opaque` | 0 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:405` | - |  |
| `impl WebGlRenderer::atlas_texture` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:808` | - | Returns an individual image atlas texture. |
| `impl Renderer::atlas_texture` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:617` | - | Returns an individual image atlas texture. |
| `impl Parser<'_>::atom` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:113` | - |  |
| `attach_script` | 3 | `crates/discourse-academic-tex/src/typeset.rs:306` | - |  |
| `attach_skl1_into_10d` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:626` | - |  |
| `impl XEl::attr` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:50` | - |  |
| `impl XmlNode::attr` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:150` | - |  |
| `attribution_row` | 2 | `crates/media-test-support/src/golden.rs:228` | - | The `ATTRIBUTION.md` table row for a golden image at workspace-relative `rel`. |
| `impl SampleEntry::audio` | 5 | `crates/media-isobmff/src/codec.rs:1261` | - | Convenience constructor for an audio entry (ISO v0 layout; the muxer upgrades PCM in MOV to v2). |
| `impl Track::audio` | 1 | `crates/media-isobmff/src/demux.rs:150` | - |  |
| `audio_callback_fn` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:264` | - |  |
| `impl Stream::audio_context` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:400` | - | Return the [`AudioContext`](https://developer.mozilla.org/docs/Web/API/AudioContext) used |
| `audio_devices` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:16` | - |  |
| `audio_peak` | 2 | `crates/asset-production-vibe/src/sound.rs:266` | - |  |
| `impl Encoder::audio_specific_config` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:241` | - | `AudioSpecificConfig` bytes for the MP4 `esds` decoder-specific info. |
| `audio_unit_from_device` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:473` | - |  |
| `audit_bib` | 1 | `crates/discourse-academic-tex/src/bibliography/audit.rs:42` | - | Run audit rules over a [`BibFile`]. |
| `audit_entry_fields` | 2 | `crates/discourse-academic-tex/src/bibliography/audit.rs:114` | - |  |
| `impl AssetHost<'_>::audit_tex_pdf_ua_matlang` | 2 | `crates/asset-production-vibe/src/discourse_chronicle_fiduciary.rs:15` | - | 142: `hypermedia.discourse.audit_tex_pdf_ua_matlang` |
| `audit_tex_pdf_ua_matlang_emits_tagged_pdf_ua_object` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:944` | - |  |
| `audit_trail_integrity_and_verification` | 0 | `crates/chronicle-fiduciary-ledger/src/audit.rs:181` | - |  |
| `authored_normals_use_full_world_inverse_transpose_and_uvs_survive` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:112` | - |  |
| `authored_normals_use_full_world_inverse_transpose_and_uvs_survive` | 0 | `crates/profile-authoring/src/tests.rs:213` | - |  |
| `autofit_widths` | 3 | `crates/layout-paged-typesetter/src/table_layout/autofit.rs:9` | - | Auto-fits column widths to available table width. |
| `impl SampleEntry::av1` | 3 | `crates/media-isobmff/src/codec.rs:1281` | - | AV1 entry (`av01`). |
| `av1_vp9_opus_roundtrip` | 0 | `crates/media-isobmff/src/codec.rs:1616` | - |  |
| `impl Encoder::available` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:305` | - | Absolute sample index one past the buffered input. |
| `impl SampleEntry::avc` | 3 | `crates/media-isobmff/src/codec.rs:1273` | - | H.264 entry (`avc1`). |
| `avc_entry` | 0 | `crates/media-isobmff/src/mux.rs:884` | - |  |
| `avcc_roundtrip` | 0 | `crates/media-isobmff/src/codec.rs:1574` | - |  |
| `average` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:253` | - |  |
| `average` | 3 | `crates/vector-locus-pathops/src/pathops/edit.rs:283` | - | Average the position of selected anchors along `axis`. |
| `averages_each_tile` | 0 | `crates/vector-radiance-trace/src/mosaic.rs:50` | - |  |
| `avg` | 2 | `crates/video-h264-codec/src/inter.rs:71` | - |  |
| `impl PerspectiveGrid::axes` | 2 | `crates/vector-locus-geometry/src/perspective.rs:220` | 3d | A plane's axes: `[u, v, normal]`. |
| `axial` | 3 | `crates/kinetics-garment-draping/src/energy.rs:56` | - | Smooth tension/compression blend. |
| `impl JointRom::axis` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:132` | - |  |
| `axis_citation` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:288` | - |  |
| `impl PerspectiveGrid::axis_col` | 2 | `crates/vector-locus-geometry/src/perspective.rs:208` | - | The image of `axis`'s point at infinity. |
| `impl Body::b` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:141` | - |  |
| `b_frame_follows_composition_order_and_selection` | 0 | `crates/video-h264-preview/src/lib.rs:599` | - |  |
| `b_mb_type` | 1 | `crates/video-h264-codec/src/mbtypes.rs:118` | - | B-slice mb_type 0..=22 (23.. are intra). |
| `b_sub_mb_type` | 1 | `crates/video-h264-codec/src/mbtypes.rs:170` | - |  |
| `b_types` | 0 | `crates/video-h264-codec/src/mbtypes.rs:227` | - |  |
| `backdrop` | 2 | `crates/vector-compute-shaders/src/cpu/backdrop.rs:25` | - |  |
| `backdrop_main` | 4 | `crates/vector-compute-shaders/src/cpu/backdrop.rs:8` | - |  |
| `impl Imdct::backward` | 7 | `crates/audio-opus-codec/src/celt/mdct.rs:106` | - | Inverse transform of `n/2` coefficients read from `input` at `stride`. |
| `bake_map` | 4 | `crates/surface-pbr-studio/src/bake.rs:147` | computational geometry | Bake a mesh map using BVH raycasting. |
| `impl AssetHost<'_>::bake_pbr_materialx` | 2 | `crates/asset-production-vibe/src/surface_pbr_brush.rs:15` | - | 126: `hypermedia.surface.bake_pbr_materialx` |
| `impl Lut::baked_cube` | 2 | `crates/color-transforms/src/lut.rs:429` | - | The LUT as a single cube over 0..1 (resampled when there is a shaper or a non-unit domain). |
| `balanced_posting_and_trial_balance` | 0 | `crates/chronicle-fiduciary-ledger/src/journal.rs:219` | - |  |
| `band_tables_are_monotonic_and_complete` | 0 | `crates/audio-aac-codec/src/tables.rs:124` | - |  |
| `impl Encoder::bandwidth` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:252` | - | Audio bandwidth in Hz. |
| `impl Series::bar` | 3 | `crates/discourse-scientific-matlang/src/plot.rs:95` | - |  |
| `impl TempoMap::bar_beat_at` | 3 | `crates/audio-timebase/src/tempo.rs:347` | - | Musical `BarBeat` at sample index `s`. |
| `impl TempoMap::bar_beat_at_tick` | 2 | `crates/audio-timebase/src/tempo.rs:299` | - | Musical `BarBeat` position at `tick`. |
| `impl TempoMap::bar_start_tick` | 2 | `crates/audio-timebase/src/tempo.rs:276` | - | Starting tick of 1-based `bar`. |
| `bare_tabstops_drop` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:213` | - |  |
| `bark` | 1 | `crates/audio-aac-codec/src/encoder/psy.rs:22` | - |  |
| `bars` | 0 | `crates/radiance-color-grading/src/scopes/tests.rs:33` | - |  |
| `impl PixelsRendererOptions::base_color` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:49` | - |  |
| `impl VelloRendererOptions::base_color` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:112` | - |  |
| `impl VelloHybridRendererOptions::base_color` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:116` | - |  |
| `impl SkiaRendererOptions::base_color` | 2 | `crates/vector-skia-render-adapter/src/window_renderer.rs:51` | - |  |
| `base_variable_load` | 3 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:143` | - | If `pointer` is a direct load of a `GlobalVariable` or `LocalVariable` (with no |
| `baseline_with_crop` | 4 | `crates/video-h264-codec/src/params.rs:726` | - |  |
| `basic_char` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:25` | - | Decode basic character set (0x20..=0x7F). |
| `basic_entry` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:427` | - |  |
| `basis` | 4 | `crates/drafting-geometry/src/spline.rs:249` | - |  |
| `basis_matches_cosines` | 0 | `crates/video-prores-codec/src/tables.rs:129` | - |  |
| `basis_needs_assertion` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:117` | - |  |
| `batch_and_duration_resolution` | 0 | `crates/multimodal-synthesis-schema/src/catalog.rs:608` | - |  |
| `batch_of` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:199` | - |  |
| `impl ModelInfo::batch_options` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:134` | - |  |
| `impl Drawable for RecordedDraw::bbox` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:82` | - |  |
| `bbox` | 1 | `crates/vector-locus-geometry/src/recognize.rs:33` | 2d |  |
| `bbox_clear` | 2 | `crates/vector-compute-shaders/src/cpu/bbox_clear.rs:17` | - |  |
| `bbox_clear_main` | 2 | `crates/vector-compute-shaders/src/cpu/bbox_clear.rs:8` | - |  |
| `bbox_intersect` | 2 | `crates/vector-compute-shaders/shader/shared/bbox.wgsl:21` | - |  |
| `bbox_intersect` | 2 | `crates/vector-compute-shaders/src/cpu/binning.rs:16` | - |  |
| `be16` | 2 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:91` | - |  |
| `be16` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:25` | - |  |
| `be16` | 2 | `crates/video-prores-codec/src/decode.rs:237` | - |  |
| `be16s` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:31` | - |  |
| `be32` | 2 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:95` | - |  |
| `be32` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:28` | - |  |
| `impl DeviceSlot<'a>::before_play` | 1 | `crates/pcm-device-output/src/device.rs:36` | - |  |
| `impl Resources::before_render` | 4 | `crates/vector-hybrid-renderer/src/text.rs:98` | - |  |
| `impl WebGlRenderer::begin` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:261` | 3d, 2d | Begins creating a WebGL2 renderer and its persistent resources. |
| `impl StripBufferArena::begin_frame` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:978` | - |  |
| `impl WhitegumMeshBuilder::begin_layer` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:51` | - |  |
| `impl GeodesicRuler::begin_move` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:177` | - |  |
| `impl GpuProfiler::begin_pass_query` | 3 | `crates/gpu-command-profiler/src/profiler.rs:266` | physics | Starts a new profiler query to be used for render/compute pass timestamp writes. |
| `impl GpuProfiler::begin_query` | 3 | `crates/gpu-command-profiler/src/profiler.rs:232` | - | Starts a new profiler query on the given encoder or rendering/compute pass (if enabled). |
| `impl GpuProfiler::begin_query_internal` | 4 | `crates/gpu-command-profiler/src/profiler.rs:685` | - |  |
| `impl CowTerrainTileGrid::begin_stroke` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:85` | - |  |
| `impl CowTerrainTileGrid::begin_stroke` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:182` | - |  |
| `impl CowTerrainTileGrid::begin_stroke` | 1 | `crates/terrain-brushes/src/lib.rs:266` | - |  |
| `impl WebGlRenderer::begin_with` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:268` | 2d | Begins creating a WebGL2 renderer with specific settings. |
| `impl Rng::below` | 2 | `crates/media-isobmff/tests/fuzz.rs:17` | - |  |
| `bending` | 3 | `crates/kinetics-garment-draping/src/energy.rs:80` | - | Flat-rest hinge bending energy. |
| `bending_element` | 3 | `crates/kinetics-garment-draping/src/energy.rs:129` | - |  |
| `ber_length` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:118` | - | Decode a BER length at `b[0..]`: (length, bytes used). `None` if truncated or longer than 8 bytes. |
| `bern` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:117` | - |  |
| `bern` | 1 | `crates/vector-radiance-trace/src/fit.rs:504` | - |  |
| `bessel_i0` | 1 | `crates/audio-aac-codec/src/mdct.rs:168` | - |  |
| `bessel_i0` | 1 | `crates/raster-spatial-operators/src/resample.rs:102` | - | Zeroth-order modified Bessel function of the first kind I_0(x). |
| `bessel_j1` | 1 | `crates/raster-spatial-operators/src/resample.rs:117` | - | First-order Bessel function of the first kind J_1(x). |
| `impl HarmonicRelation::between` | 2 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:50` | - |  |
| `bez` | 5 | `crates/kinetics-motion-compositor/src/keyframe/bezier.rs:4` | - | 1D cubic Bézier point evaluation: B(u) for u in [0, 1]. |
| `bez_d` | 5 | `crates/kinetics-motion-compositor/src/keyframe/bezier.rs:10` | - | Derivative of 1D cubic Bézier: B'(u). |
| `bezpath_roundtrip_curves` | 0 | `crates/vector-locus-geometry/src/path.rs:551` | - |  |
| `bezpath_roundtrip_polyline` | 0 | `crates/vector-locus-geometry/src/path.rs:540` | - |  |
| `bib_directives` | 0 | `crates/discourse-academic-tex/src/texparse/magic.rs:146` | - |  |
| `impl Parser::bib_entry` | 3 | `crates/discourse-academic-tex/src/texparse/bib.rs:211` | - |  |
| `bibval` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:36` | - | BibTeX-safe value: braces kept balanced. |
| `bic_combine` | 2 | `crates/vector-compute-shaders/shader/shared/clip.wgsl:9` | - |  |
| `bicubic_sample` | 4 | `crates/vector-compute-shaders/shader/fine.wgsl:950` | 2d |  |
| `bidi_class` | 1 | `crates/vector-glyph-shaping/src/bidi.rs:25` | - | Character classification according to Unicode bidirectional rules. |
| `bin_of` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:361` | - |  |
| `impl Rounds::bind_at_or_after` | 3 | `crates/vector-hybrid-renderer/src/schedule/round.rs:91` | - | Bind the required texture pages in the first compatible round at or after `point`. |
| `impl WebGlRendererContext<'_>::bind_external_textures` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2711` | - | Bind the external texture slots. |
| `impl AssetHost<'_>::bind_theme_scope` | 2 | `crates/asset-production-vibe/src/layout_ops/box_theme.rs:564` | - | Binds structured Webizen scoped design tokens and CSS variables. |
| `impl DeviceOutput::binding` | 1 | `crates/pcm-device-output/src/device.rs:209` | - |  |
| `binding` | 0 | `crates/pcm-device-output/src/queue_tests.rs:6` | - |  |
| `impl ViewFramebuffer::binding` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1216` | - |  |
| `binding_case` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:159` | - |  |
| `binding_conflicts_even` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:701` | - |  |
| `binding_conflicts_odd` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:740` | - |  |
| `binding_conflicts_skip_rounds` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:773` | - |  |
| `impl WebGlIntermediateTexture::binding_texture` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1348` | - |  |
| `binning` | 2 | `crates/vector-compute-shaders/src/cpu/binning.rs:110` | - |  |
| `binning_main` | 9 | `crates/vector-compute-shaders/src/cpu/binning.rs:25` | - |  |
| `impl WindowShape::bit` | 1 | `crates/audio-aac-codec/src/mdct.rs:158` | - |  |
| `impl TnsData::bit_count` | 2 | `crates/audio-aac-codec/src/tns.rs:96` | - | Bits `write` would produce. |
| `bit_count_matches_written_bits` | 0 | `crates/audio-aac-codec/src/encoder/quant.rs:369` | - |  |
| `impl BitWriter::bit_len` | 1 | `crates/bitstream-codec/src/lib.rs:216` | - |  |
| `impl RangeDecoder<'a>::bit_logp` | 2 | `crates/audio-opus-codec/src/range.rs:149` | - | Decodes a binary symbol whose probability of being 1 is `1/2^logp`. |
| `impl RangeEncoder::bit_logp` | 3 | `crates/audio-opus-codec/src/range.rs:320` | - |  |
| `bit_max` | 1 | `crates/color-transforms/src/lut.rs:463` | - | Full-scale value of the smallest common integer depth that holds `v`. |
| `impl Cabac<'a>::bit_pos` | 1 | `crates/video-h264-codec/src/cabac.rs:129` | - | Bit position of the next bit not consumed by the arithmetic decoder (for I_PCM alignment |
| `bitexact_cos` | 1 | `crates/audio-opus-codec/src/celt/bands.rs:198` | - |  |
| `bitexact_log2tan` | 2 | `crates/audio-opus-codec/src/celt/bands.rs:205` | - |  |
| `bitexact_trig_endpoints` | 0 | `crates/audio-opus-codec/src/celt/bands.rs:912` | - |  |
| `bitmap_masks` | 1 | `crates/vector-compute-renderer/src/scene.rs:880` | - |  |
| `impl std::ops::BitOr for DebugLayers::bitor` | 2 | `crates/vector-compute-renderer/src/debug.rs:117` | - |  |
| `impl EncoderStats::bitrate` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:94` | - | Average bitrate of the raw access units. |
| `bitrate_label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:595` | - |  |
| `impl RangeDecoder<'a>::bits` | 2 | `crates/audio-opus-codec/src/range.rs:213` | - | Reads `bits` raw bits (0..=25) from the end of the buffer. |
| `impl RangeEncoder::bits` | 3 | `crates/audio-opus-codec/src/range.rs:355` | - |  |
| `impl DebugLayers::bits` | 1 | `crates/vector-compute-renderer/src/debug.rs:77` | - | Get the raw representation of this value. |
| `impl Mode::bits2pulses` | 4 | `crates/audio-opus-codec/src/celt/rate.rs:119` | - |  |
| `impl BitReader<'a>::bits_left` | 1 | `crates/bitstream-codec/src/lib.rs:51` | - |  |
| `impl Reader<'a>::bits_left` | 1 | `crates/video-prores-codec/src/bits.rs:57` | - |  |
| `impl Encoder::bits_per_mb` | 1 | `crates/video-prores-codec/src/encode.rs:68` | - |  |
| `black_disc_is_one_closed_path_with_disc_area` | 0 | `crates/vector-radiance-trace/src/tests.rs:30` | - |  |
| `black_point` | 2 | `crates/color-transforms/src/icc/transform.rs:230` | - |  |
| `blackbody_rgb` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:391` | - |  |
| `impl BlendMode::blend` | 3 | `crates/surface-pbr-studio/src/compose.rs:21` | - |  |
| `impl RoundStage::blend` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:375` | - |  |
| `blend_case` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:90` | - |  |
| `blend_channel` | 3 | `crates/raster-blend-operators/src/lib.rs:20` | - | Evaluates one operator for one backdrop/source channel pair. |
| `blend_color` | 3 | `crates/asset-production-vibe/src/layout_ops/mod.rs:74` | - |  |
| `blend_is_constrained_to_parent_clip_bbox` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:926` | - |  |
| `blend_mix` | 3 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:147` | 3d |  |
| `impl Drawable for RecordedDraw::blend_mode` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:106` | - |  |
| `blend_mode_from` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1210` | - |  |
| `blend_mode_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:175` | - | Evaluates a `BlendMode` on straight (not premultiplied) RGB in `[0, 1]`. |
| `blend_operator_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:68` | - | Evaluates a `BlendOperator` independently for three RGB channels. |
| `impl Backend for WebGlRendererContext<'_>::blend_pass` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3270` | - |  |
| `impl Backend for RendererContext<'_>::blend_pass` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3131` | - |  |
| `trait Backend::blend_pass` | 4 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:51` | - | Execute a blend pass between a parent and child texture. |
| `impl Backend for Recorder::blend_pass` | 4 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:261` | - |  |
| `impl WebGlRendererContext<'_>::blend_pass_inner` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2943` | - |  |
| `impl RendererContext<'_>::blend_pass_inner` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2850` | - |  |
| `blend_release` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:366` | - |  |
| `blend_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:245` | - | Evaluates an operator or blend mode independently across RGB channels. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::blend_state` | 2 | `crates/pixel-buffer-output/src/builder.rs:211` | - | Set the blend state. |
| `blessing` | 0 | `crates/media-test-support/src/golden.rs:171` | - | Whether `FILMCRAFT_BLESS` asks to regenerate references. |
| `impl SurfaceRenderer<'s>::blit_from_intermediate_texture_to_surface` | 3 | `crates/gpu-device-context/src/surface_renderer.rs:324` | - | Blit from the intermediate texture to the surface texture |
| `blit_opaque` | 8 | `crates/asset-production-vibe/src/graphics_pack.rs:16` | - |  |
| `blit_rect_nn` | 14 | `crates/asset-production-vibe/src/graphics_pack.rs:41` | 2d |  |
| `blk4_maps_consistent` | 0 | `crates/video-h264-codec/src/tables.rs:162` | - |  |
| `impl Model::block` | 2 | `crates/kinetics-block-simulation/src/model.rs:94` | - |  |
| `block_motion` | 3 | `crates/video-h264-codec/src/deblock.rs:26` | - |  |
| `impl Model::block_mut` | 2 | `crates/kinetics-block-simulation/src/model.rs:99` | - |  |
| `impl Model::block_names` | 1 | `crates/kinetics-block-simulation/src/model.rs:90` | - |  |
| `block_on_wgpu` | 2 | `crates/gpu-device-context/src/util.rs:8` | - | Block on a future, polling the device as needed. |
| `block_on_wgpu` | 2 | `crates/vector-compute-renderer/src/util.rs:257` | - | Block on a future, polling the device as needed. |
| `book` | 1 | `crates/audio-aac-codec/src/huffman.rs:52` | - | Spectrum codebook 1..=11. |
| `boolean` | 3 | `crates/asset-production-vibe/src/lib.rs:536` | - |  |
| `impl SigMeta::boolean` | 1 | `crates/kinetics-block-simulation/src/signal.rs:27` | - |  |
| `impl SigVal::boolean` | 1 | `crates/kinetics-block-simulation/src/signal.rs:104` | - |  |
| `boolean` | 3 | `crates/vector-locus-pathops/src/pathops/boolean.rs:496` | - | Execute a binary boolean operation on two paths. |
| `boolean_difference_rectangles` | 0 | `crates/vector-locus-pathops/src/pathops/boolean.rs:585` | - |  |
| `boolean_intersect_rectangles` | 0 | `crates/vector-locus-pathops/src/pathops/boolean.rs:575` | - |  |
| `boolean_union_rectangles` | 0 | `crates/vector-locus-pathops/src/pathops/boolean.rs:565` | - |  |
| `boost_current_thread_priority` | 0 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:376` | - |  |
| `border_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:164` | - |  |
| `border_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:149` | - |  |
| `border_impl` | 4 | `crates/raster-selection-morphology/src/lib.rs:234` | - |  |
| `border_matches_measured_oracle` | 0 | `crates/raster-selection-morphology/src/tests.rs:33` | - |  |
| `impl Rect::bottom` | 1 | `crates/layout-box-autolayout/src/types.rs:116` | - |  |
| `impl Rect::bottom` | 1 | `crates/layout-paged-typesetter/src/pagination/types.rs:34` | - |  |
| `boundaries_at_large_world_coordinates_keep_distinct_vertices` | 0 | `crates/drafting-geometry/src/region.rs:390` | - |  |
| `boundary_at` | 3 | `crates/drafting-geometry/src/region.rs:367` | 2d | The closed loop enclosing `p` plus the islands inside it, or `None` when `p` isn't enclosed. |
| `bounded_count` | 2 | `crates/media-isobmff/src/demux.rs:34` | - |  |
| `impl Arc::bounds` | 1 | `crates/drafting-geometry/src/curve.rs:113` | - |  |
| `impl Circle::bounds` | 1 | `crates/drafting-geometry/src/curve.rs:175` | - |  |
| `impl Ellipse::bounds` | 1 | `crates/drafting-geometry/src/curve.rs:258` | - |  |
| `impl Segment::bounds` | 1 | `crates/drafting-geometry/src/curve.rs:350` | - |  |
| `impl Polyline::bounds` | 1 | `crates/drafting-geometry/src/curve.rs:490` | - |  |
| `impl Spline::bounds` | 1 | `crates/drafting-geometry/src/spline.rs:209` | - |  |
| `bounds` | 1 | `crates/manifold-bim-csg/src/csg.rs:323` | - |  |
| `impl Bvh::bounds` | 1 | `crates/surface-pbr-studio/src/bvh.rs:50` | - |  |
| `impl Mesh::bounds` | 1 | `crates/surface-pbr-studio/src/mesh.rs:39` | - |  |
| `impl PathData::bounds` | 1 | `crates/vector-locus-geometry/src/path.rs:435` | 2d | Tight geometric bounds (curve extrema, not control points). |
| `bounds` | 1 | `crates/vector-locus-pathops/src/three_d/visibility.rs:29` | 2d |  |
| `bounds2` | 1 | `crates/asset-production-vibe/src/geometry.rs:381` | - |  |
| `impl RasterImage::box_blur` | 2 | `crates/surface-pbr-studio/src/raster.rs:97` | - | Fast separable box blur of all channels. |
| `box_spec` | 3 | `crates/asset-production-vibe/src/layout_ops/mod.rs:380` | - |  |
| `box_xywh` | 2 | `crates/asset-production-vibe/src/layout_ops/mod.rs:391` | - |  |
| `boxes` | 1 | `crates/media-isobmff/src/bytes.rs:201` | - |  |
| `boxes` | 3 | `crates/video-h264-preview/src/lib.rs:633` | - |  |
| `boxes_non_overlapping` | 1 | `crates/asset-production-vibe/src/layout_ops/box_theme.rs:68` | - |  |
| `impl Parser::braced` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:364` | - |  |
| `braced` | 3 | `crates/discourse-academic-tex/src/typeset.rs:325` | - |  |
| `bradford` | 2 | `crates/color-transforms/src/icc/math.rs:92` | numerics | Bradford chromatic adaptation matrix from white `src` to white `dst` (both XYZ). |
| `bradford` | 2 | `crates/radiance-color-grading/src/spaces.rs:138` | numerics | Bradford chromatic adaptation transform matrix between two white points. |
| `bristle_bundles` | 0 | `crates/vector-surface-brush/src/bristle.rs:147` | - |  |
| `impl DrawGlyphs<'a>::brush` | 2 | `crates/vector-compute-renderer/src/scene.rs:581` | - | Sets the brush. |
| `impl DrawGlyphs<'a>::brush_alpha` | 2 | `crates/vector-compute-renderer/src/scene.rs:590` | - | Sets an additional alpha multiplier for the brush. |
| `impl DrawGlyphs<'a>::brush_transform` | 2 | `crates/vector-compute-renderer/src/scene.rs:530` | - | Sets the transform applied to the brush contents. |
| `brute` | 3 | `crates/raster-selection-morphology/src/distance/tests.rs:3` | - |  |
| `brute_tetra` | 2 | `crates/color-transforms/src/lut.rs:473` | physics | Brute-force tetrahedral reference: find the Kuhn simplex (one of the 6 tetrahedra along the |
| `impl MasteringDisplay::bt2020` | 2 | `crates/media-isobmff/src/codec.rs:62` | - | BT.2020 primaries, D65, with the given peak and black in cd/m². |
| `bt2020_to_709_matches_bt2087` | 0 | `crates/color-transforms/src/spaces.rs:385` | - |  |
| `bt2390_eetf` | 3 | `crates/color-transforms/src/transform.rs:108` | - | ITU-R BT.2390 EETF: maps `nits` from a display with peak `src_peak` to one with peak |
| `bt709_npm_matches_published_matrix` | 0 | `crates/color-transforms/src/spaces.rs:370` | numerics |  |
| `buffer_duration_to_frames` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:1019` | - |  |
| `impl jack::ProcessHandler for LocalProcessHandler::buffer_size` | 3 | `crates/audio-device-cpal/src/host/jack/stream.rs:378` | - |  |
| `impl SupportedStreamConfig::buffer_size` | 1 | `crates/audio-device-cpal/src/lib.rs:399` | - |  |
| `impl SupportedStreamConfigRange::buffer_size` | 1 | `crates/audio-device-cpal/src/lib.rs:624` | - |  |
| `buffer_size_range_for_params` | 4 | `crates/audio-device-cpal/src/host/oboe/mod.rs:86` | - |  |
| `buffer_size_to_duration` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:1012` | - |  |
| `buffer_time_step_secs` | 2 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:512` | - |  |
| `impl PeakFile::build` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:260` | - |  |
| `impl Tree::build` | 1 | `crates/audio-aac-codec/src/huffman.rs:64` | - |  |
| `impl OpusHead::build` | 6 | `crates/audio-opus-codec/src/header.rs:93` | - |  |
| `impl Lut::build` | 2 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:149` | - |  |
| `build` | 1 | `crates/color-transforms/src/icc/builtin.rs:199` | - |  |
| `build` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:141` | - |  |
| `build` | 2 | `crates/drafting-geometry/src/region.rs:55` | 2d, graph |  |
| `impl ArcLengthTable::build` | 5 | `crates/kinetics-motion-compositor/src/keyframe/spatial.rs:32` | - | Build an arc-length table for a 3D spatial Bézier segment. |
| `impl ImagePyramid::build` | 2 | `crates/kinetics-motion-compositor/src/track/klt.rs:24` | - | Construct a Gaussian pyramid with `num_levels` downsampled by 2x each. |
| `impl Model::build` | 1 | `crates/locus-parametric-solver/src/solver/model.rs:146` | - |  |
| `impl Bsp::build` | 3 | `crates/manifold-bim-csg/src/csg.rs:213` | - |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::build` | 1 | `crates/pixel-buffer-output/src/builder.rs:410` | 2d | Create a pixel buffer from the options builder. |
| `build` | 4 | `crates/radiance-color-grading/src/scopes/mod.rs:281` | - |  |
| `impl SurfaceAttributes::build` | 3 | `crates/surface-pbr-studio/src/bake.rs:55` | computational geometry |  |
| `impl Bvh::build` | 1 | `crates/surface-pbr-studio/src/bvh.rs:29` | computational geometry | Build a BVH for a mesh. An empty mesh yields an empty tree. |
| `impl Scheduler<'a, 'p>::build` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:320` | - | Build the complete dependency-ordered schedule. |
| `impl VlcTable::build` | 1 | `crates/video-h264-codec/src/cavlc.rs:15` | - |  |
| `build_anatomy_from_portable_scaffold` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:22` | - | Compiles a 3-layer `.10d` vertebrate anatomical twin from a species spec (VibeScript record |
| `build_and_check` | 2 | `crates/video-h264-codec/src/synth_tests.rs:374` | - |  |
| `build_and_parse_sfnt_roundtrip` | 0 | `crates/vector-glyph-shaping/src/subset.rs:150` | - |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::build_async` | 1 | `crates/pixel-buffer-output/src/builder.rs:436` | 2d | Create a pixel buffer from the options builder without blocking the current thread. |
| `impl Device::build_audioclient` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:363` | - | Returns an uninitialized `IAudioClient`. |
| `build_authored_part` | 9 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:1401` | computational geometry | One Synth-authored part. The mesh is generated here. No external model is fetched. |
| `build_avc_mp4` | 7 | `crates/asset-production-vibe/src/chronicle_ops.rs:617` | - |  |
| `build_bvh_recursive` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:175` | - |  |
| `impl Rounds::build_draw` | 5 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:929` | - |  |
| `build_fauna_production_plan_json` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:416` | - | Builds the complete **Model Production Plan** (`qualia.model-production-plan/v1`) for a Fauna |
| `build_flora_mycelium_production_plan_json` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:487` | - | Builds the complete **Flora & Mycelial Neural Network Production Plan** |
| `build_human_welfare_anatomy_10d` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:344` | - | Builds a complete 3-layer / 17-system Human Welfare & Maternal-Fetal `.10d` container |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::build_impl` | 1 | `crates/pixel-buffer-output/src/builder.rs:273` | 2d | Create a pixel buffer from the options builder. |
| `build_input_stream` | 5 | `crates/audio-device-cpal/src/host/oboe/mod.rs:222` | - |  |
| `trait DeviceTrait::build_input_stream` | 5 | `crates/audio-device-cpal/src/traits.rs:125` | - | Create an input stream. |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/alsa/mod.rs:88` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/asio/mod.rs:85` | - |  |
| `impl Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/asio/stream.rs:36` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:172` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:113` | - |  |
| `impl Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:515` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:167` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/jack/device.rs:175` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/null/mod.rs:72` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/oboe/mod.rs:324` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/wasapi/device.rs:81` | - |  |
| `impl DeviceTrait for Device::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:170` | - |  |
| `trait DeviceTrait::build_input_stream_raw` | 6 | `crates/audio-device-cpal/src/traits.rs:181` | - | Create a dynamically typed input stream. |
| `impl Device::build_input_stream_raw_inner` | 3 | `crates/audio-device-cpal/src/host/wasapi/device.rs:542` | - |  |
| `impl FragmentedWriter<W>::build_moof` | 2 | `crates/media-isobmff/src/mux.rs:797` | - |  |
| `impl Mp4Writer<W>::build_moov` | 3 | `crates/media-isobmff/src/mux.rs:662` | physics |  |
| `build_morph_frame` | 3 | `crates/layout-presentation-deck/src/morph/transition.rs:43` | - | Compute a morph transition frame between slide `a` and slide `b` at progress `t` (0.0 .. 1.0). |
| `build_output_stream` | 5 | `crates/audio-device-cpal/src/host/oboe/mod.rs:246` | - |  |
| `trait DeviceTrait::build_output_stream` | 5 | `crates/audio-device-cpal/src/traits.rs:153` | - | Create an output stream. |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/alsa/mod.rs:111` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/asio/mod.rs:107` | - |  |
| `impl Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/asio/stream.rs:241` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:255` | - | Create an output stream. |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:135` | - |  |
| `impl Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:625` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:182` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/jack/device.rs:214` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/null/mod.rs:88` | - | Create an output stream. |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/oboe/mod.rs:398` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/wasapi/device.rs:101` | - |  |
| `impl DeviceTrait for Device::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:187` | - | Create an output stream. |
| `trait DeviceTrait::build_output_stream_raw` | 6 | `crates/audio-device-cpal/src/traits.rs:194` | - | Create a dynamically typed output stream. |
| `impl Device::build_output_stream_raw_inner` | 3 | `crates/audio-device-cpal/src/host/wasapi/device.rs:665` | - |  |
| `impl OctreeQuantizer::build_palette` | 2 | `crates/raster-spatial-operators/src/octree.rs:84` | - | Reduce octree leaves to `max_colors`. |
| `build_pat` | 3 | `crates/chronicle-broadcast-containers/src/mpegts/psi.rs:131` | - | Build a PAT section including CRC-32. |
| `build_perm_table` | 1 | `crates/terrain-erosion-solvers/src/fractal_terrace.rs:38` | - | PRNG table generator for strata hash lookup. |
| `build_pes_packet` | 4 | `crates/chronicle-broadcast-containers/src/mpegts/pes.rs:113` | - | Create a PES packet containing `data` with optional PTS/DTS. |
| `build_pmt` | 4 | `crates/chronicle-broadcast-containers/src/mpegts/psi.rs:152` | - | Build a PMT section including CRC-32. |
| `build_pps` | 0 | `crates/asset-production-vibe/src/chronicle_ops.rs:513` | - |  |
| `build_range` | 6 | `crates/surface-pbr-studio/src/bvh.rs:206` | computational geometry, graph |  |
| `impl Dpb<F>::build_ref_lists` | 5 | `crates/video-h264-codec/src/dpb.rs:164` | - | Initial reference picture lists (8.2.4.2) + modification (8.2.4.3) for a frame slice, each |
| `build_rest_tris` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:88` | computational geometry |  |
| `build_set` | 3 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:211` | - |  |
| `build_sfnt_font` | 2 | `crates/vector-glyph-shaping/src/subset.rs:91` | - | Assemble a valid standalone SFNT font binary from a set of table records. |
| `impl WgpuEngine::build_shaders_if_needed` | 3 | `crates/vector-compute-renderer/src/wgpu_engine.rs:163` | - | Initialise (in parallel) any shaders which are yet to be created |
| `build_sps` | 3 | `crates/asset-production-vibe/src/chronicle_ops.rs:470` | - |  |
| `build_stream_err` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:707` | - |  |
| `impl Device::build_stream_inner` | 4 | `crates/audio-device-cpal/src/host/alsa/mod.rs:244` | - |  |
| `build_theme_tokens` | 4 | `crates/asset-production-vibe/src/layout_ops/mod.rs:85` | - |  |
| `impl SliceDecoder<'a>::build_weights` | 4 | `crates/video-h264-codec/src/slicedec.rs:331` | - |  |
| `build_wgpu_context` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:238` | - |  |
| `build_wgpu_context` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:187` | - |  |
| `build_whitegum_whole_organism_10d` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:299` | - | Builds a high-craft Scale-1 Whole-Organism Whitegum / Ghost Gum (`Corymbia aparrerinja`) |
| `builtin_format` | 1 | `crates/chronicle-tabular-engine/src/numfmt/builtin.rs:42` | - |  |
| `builtin_id` | 1 | `crates/chronicle-tabular-engine/src/numfmt/builtin.rs:46` | - |  |
| `builtins_are_available` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:227` | - |  |
| `bulge_quarter_arc` | 0 | `crates/drafting-geometry/src/tests.rs:70` | - |  |
| `bulge_semicircle` | 0 | `crates/drafting-geometry/src/tests.rs:57` | - |  |
| `bulge_to_arc` | 3 | `crates/drafting-geometry/src/curve.rs:435` | 2d | Convert a bulge segment to an arc. `None` for a straight (zero-bulge or degenerate) segment. |
| `bulged_points` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:180` | - |  |
| `impl ElasticIsotropic::bulk_modulus` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:37` | - | Bulk modulus K = E / (3 * (1 - 2*nu)). |
| `bump` | 3 | `crates/discourse-academic-tex/src/texparse/lex.rs:50` | - |  |
| `impl Dpb<F>::bump` | 2 | `crates/video-h264-codec/src/dpb.rs:79` | - | Output the picture with the smallest POC (C.4.5.3 "bumping"). Returns false if none waits. |
| `impl Render::bump_buf` | 1 | `crates/vector-compute-renderer/src/render.rs:639` | - |  |
| `impl Scene::bump_estimate` | 2 | `crates/vector-compute-renderer/src/scene.rs:68` | - | Tally up the bump allocator estimate for the current state of the encoding, |
| `burn_and_dodge_treat_near_extremes_as_exact` | 0 | `crates/raster-blend-operators/src/lib.rs:533` | - |  |
| `bw` | 0 | `crates/vector-radiance-trace/src/tests.rs:7` | - |  |
| `bwexpander` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:191` | - | `silk_bwexpander` on Q12 coefficients (16-bit). |
| `bwexpander_32` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:180` | - |  |
| `impl Enc::bypass` | 2 | `crates/video-h264-codec/src/cabac.rs:261` | - |  |
| `byte` | 1 | `crates/vector-compute-renderer/src/scene.rs:884` | - |  |
| `impl BitReader<'a>::byte_align` | 1 | `crates/bitstream-codec/src/lib.rs:57` | - |  |
| `impl CowTerrainTileGrid::byte_budget` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:27` | - |  |
| `impl CowTerrainTileGrid::byte_budget` | 1 | `crates/terrain-brushes/src/lib.rs:207` | - |  |
| `byte_budget_evicts_lru_but_not_current_frame` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:210` | - |  |
| `impl BitReader<'a>::byte_pos` | 1 | `crates/bitstream-codec/src/lib.rs:61` | - | Byte offset of the current position (rounded down). |
| `impl Artifact::bytes` | 1 | `crates/asset-production-vibe/src/lib.rs:115` | - |  |
| `impl Data::bytes` | 1 | `crates/audio-device-cpal/src/lib.rs:540` | - | The raw slice of memory representing the underlying audio data as a slice of bytes. |
| `impl BitWriter::bytes` | 1 | `crates/bitstream-codec/src/lib.rs:246` | - |  |
| `impl Cur<'a>::bytes` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:154` | - |  |
| `impl Cur<'a>::bytes` | 2 | `crates/media-isobmff/src/bytes.rs:80` | - |  |
| `impl BoxBuf::bytes` | 2 | `crates/media-isobmff/src/bytes.rs:304` | - |  |
| `impl Data::bytes_mut` | 1 | `crates/audio-device-cpal/src/lib.rs:550` | - | The raw slice of memory representing the underlying audio data as a slice of bytes. |
| `impl PcmConfig::bytes_per_frame` | 1 | `crates/media-isobmff/src/codec.rs:613` | - | Bytes per interleaved audio frame (all channels). |
| `trait Scalar::c` | 1 | `crates/kinetics-garment-draping/src/math.rs:240` | - |  |
| `impl Scalar for f64::c` | 1 | `crates/kinetics-garment-draping/src/math.rs:248` | - |  |
| `impl Scalar for Jet<N>::c` | 1 | `crates/kinetics-garment-draping/src/math.rs:292` | - |  |
| `impl Mode::cache` | 3 | `crates/audio-opus-codec/src/celt/rate.rs:114` | - |  |
| `impl DevelopSettings::cache_digest` | 1 | `crates/radiance-camera-raw/src/develop.rs:115` | - | Computes deterministic 64-bit FNV-1a hash over all parameters. |
| `cache_hit_reuses_registered_image` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:157` | - |  |
| `calc_table_checksum` | 1 | `crates/vector-glyph-shaping/src/subset.rs:18` | - | Calculate OpenType table checksum (sum of big-endian u32 words). |
| `impl DimensionConstraints::calculate_dimensions` | 3 | `crates/vector-hybrid-renderer/src/util.rs:36` | - | Calculate dimensions while preserving aspect ratio within constraints |
| `calculate_dimensions_in_range` | 0 | `crates/vector-hybrid-renderer/src/util.rs:255` | - |  |
| `calculate_isochrone` | 3 | `crates/geodesy-spatial-analysis/src/network_topology.rs:172` | graph | Computes isochrone reachable nodes from start within `max_cost` budget. |
| `impl Oscillator::calculate_sine_output_from_freq` | 2 | `crates/audio-device-cpal/examples/synth_tones.rs:45` | - |  |
| `calculate_splat_cost` | 2 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:71` | - | Calculate the cost of a 3D Gaussian Splat / radiance field generation. |
| `calendar_skips_weekends_and_holidays` | 0 | `crates/kinetics-schedule-cpm/src/model.rs:874` | - |  |
| `impl VibeNamespaceProvider for HypermediaOntologyProvider::call` | 4 | `crates/asset-production-vibe/src/ontology/vibe_descriptors.rs:267` | - |  |
| `callback_crosses_slots_in_order_and_starvation_cannot_resume_old_audio` | 0 | `crates/pcm-device-output/src/queue_tests.rs:53` | - |  |
| `caller_limits_refuse_input_track_and_table_counts_before_expansion` | 0 | `crates/media-isobmff/src/demux.rs:1243` | - |  |
| `calligraphic_sweep` | 0 | `crates/vector-surface-brush/src/calligraphic.rs:237` | - |  |
| `camera_spherical_adjustment` | 0 | `crates/multimodal-synthesis-schema/src/stage.rs:160` | 3d |  |
| `impl SyncRole::can_comment` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:59` | - |  |
| `impl SyncRole::can_manage_roles` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:67` | - |  |
| `impl SyncRole::can_read` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:55` | - |  |
| `impl GeodesicRuler::can_redo` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:225` | - |  |
| `impl GeodesicRuler::can_undo` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:217` | - |  |
| `impl SyncRole::can_write_tasks` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:63` | - |  |
| `impl DeviceOutput::cancel` | 1 | `crates/pcm-device-output/src/device.rs:212` | - |  |
| `cancel_and_failed_commit_preserve_redo_and_exact_tile_baseline` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:160` | - |  |
| `impl CowTerrainTileGrid::cancel_stroke` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:220` | - | Restore the precise original tile Arcs without committing or changing history. |
| `impl CowTerrainTileGrid::cancel_stroke` | 1 | `crates/terrain-brushes/src/lib.rs:405` | - | Restore the precise original tile Arcs without committing or changing history. |
| `cancellation_silences_pending_samples_and_rejects_new_enqueue` | 0 | `crates/pcm-device-output/src/queue_tests.rs:70` | - |  |
| `candidates` | 1 | `crates/audio-aac-codec/src/encoder/quant.rs:142` | - | Candidate codebooks for a band whose largest magnitude is `m` (0 for all-zero bands). |
| `canon` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:209` | - |  |
| `canonical_joint` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:45` | - |  |
| `canonical_morphism_id` | 1 | `crates/asset-production-vibe/src/ontology/mod.rs:247` | - | Resolves any operation identifier to its canonical categorical ID (`hypermedia.<category>.<morphism>`). |
| `caps_match_reference_shape` | 0 | `crates/audio-opus-codec/src/celt/rate.rs:573` | - |  |
| `impl Project::capture_baseline` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:688` | - |  |
| `impl AssetHost<'_>::capture_deterministic_movie` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:275` | - |  |
| `impl MaterialPreset::car_paint` | 3 | `crates/surface-pbr-studio/src/material.rs:129` | - |  |
| `impl ScreenBuffer::carriage_return` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:166` | - |  |
| `impl RangeEncoder::carry_out` | 2 | `crates/audio-opus-codec/src/range.rs:285` | - |  |
| `carve_channels` | 6 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:252` | - | Carve fluvial river channels into a conditioned heightfield along high-accumulation paths. |
| `carve_one_horizontal_seam_rgba32f` | 3 | `crates/raster-spatial-operators/src/seam_carve.rs:287` | - |  |
| `carve_one_horizontal_seam_rgba8` | 3 | `crates/raster-spatial-operators/src/seam_carve.rs:176` | - |  |
| `carve_one_vertical_seam_rgba32f` | 3 | `crates/raster-spatial-operators/src/seam_carve.rs:203` | - |  |
| `carve_one_vertical_seam_rgba8` | 3 | `crates/raster-spatial-operators/src/seam_carve.rs:84` | - |  |
| `cast_slice_mut` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:670` | - | Cast a byte slice into a mutable slice of desired type. |
| `catalog` | 0 | `crates/discourse-academic-tex/src/catalog.rs:388` | - | The full catalog, sorted by [`CommandSpec::name`]. |
| `catalog_compile_10d_pbr_albedo_samples_unwrap_uv01` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:638` | - |  |
| `catalog_compile_10d_pbr_auto_unwraps_albedo_without_source_10d` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:775` | - |  |
| `catalog_compile_10d_pbr_consumes_unwrap_uv01` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:569` | - |  |
| `catalog_compile_10d_pbr_source_10d_uv_mismatch_fails_closed` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:951` | - |  |
| `catalog_dispatch_csi_hits_csiframe` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:369` | - |  |
| `catalog_dispatch_cubic_loft_not_interpolation_rejected` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:400` | - |  |
| `catalog_dispatch_nurbs_loft_and_revolve_optional` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:462` | - |  |
| `catalog_emf_spectral_png_is_non_black` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:845` | - |  |
| `catalog_is_large_enough` | 0 | `crates/discourse-academic-tex/src/catalog.rs:422` | - |  |
| `catalog_is_sorted_and_unique` | 0 | `crates/discourse-academic-tex/src/catalog.rs:427` | - |  |
| `catalog_pack_nine_slice_insets_sidecar` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1196` | - |  |
| `catalog_pack_pbr_set_encoded_png` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1233` | - |  |
| `catalog_pack_sprite_sheet_frames_no_overlap` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1133` | 2d |  |
| `catalog_pack_sprite_sheet_two_produce_sha256` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1300` | 2d |  |
| `catalog_pack_sprite_sheet_unknown_argument` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1330` | 2d |  |
| `catalog_rows_raise_wave_b_objects` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:344` | - |  |
| `catalog_synthesize_maps_side_1024_envelope` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1265` | - |  |
| `catalog_text` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:335` | - |  |
| `catalogue` | 0 | `crates/asset-production-vibe/src/lib.rs:73` | - |  |
| `categories_json` | 0 | `crates/asset-production-vibe/src/ontology/mod.rs:261` | semantics | Serializes the category-theoretic ontology taxonomy for agentic discovery (`synth.ontology.categories`). |
| `impl Rule::category` | 1 | `crates/discourse-pdf-compliance/src/a11y/rules.rs:115` | - |  |
| `catmull_rom` | 3 | `crates/vector-locus-pathops/src/effects/util.rs:75` | - | Smooth subpath through points using Catmull-Rom tangents. |
| `catmull_rom_axis` | 5 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:605` | - |  |
| `catmull_rom_point3` | 5 | `crates/asset-production-vibe/src/manifold_ops.rs:915` | - |  |
| `catmull_rom_scalar` | 5 | `crates/asset-production-vibe/src/manifold_ops.rs:906` | - |  |
| `cb` | 3 | `crates/video-prores-codec/src/tables.rs:12` | - |  |
| `impl SliceDecoder<'_>::cbf_chroma_ac_cond` | 5 | `crates/video-h264-codec/src/cabac_mb.rs:367` | - |  |
| `impl SliceDecoder<'_>::cbf_dc_cond` | 4 | `crates/video-h264-codec/src/cabac_mb.rs:358` | - |  |
| `impl SliceDecoder<'_>::cbf_luma_cond` | 4 | `crates/video-h264-codec/src/cabac_mb.rs:347` | - | coded_block_flag ctxIdxInc condition for a neighbouring luma 4x4 block (cat 0..2) or chroma. |
| `impl SliceDecoder<'_>::cbp_luma_cond` | 3 | `crates/video-h264-codec/src/cabac_mb.rs:318` | - | condTermFlagN for coded_block_pattern luma bin from a neighbouring MB; `b8n` is the neighbouring |
| `cbp_table_is_permutation` | 0 | `crates/video-h264-codec/src/mbtypes.rs:206` | - |  |
| `impl EncoderConfig::cbr` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:60` | - |  |
| `ccw_sweep` | 2 | `crates/drafting-geometry/src/lib.rs:51` | - | Counter-clockwise sweep from `start` to `end`, in `(0, 2π]`. |
| `ceiling` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:41` | - |  |
| `impl RangeRef::cell` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:147` | - |  |
| `impl Anchor::cell` | 1 | `crates/chronicle-tabular-engine/src/formula/ast.rs:19` | - |  |
| `impl Vectorscope::cell` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:443` | - |  |
| `impl Dggs::cell_area_m2` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:576` | - | Nominal area of a DGGS cell at resolution `res` (0..=30) in square metres. |
| `impl Dggs::cell_boundary` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:586` | - | Generates boundary vertices for an approximate DGGS hexagonal/pentagonal cell given its centre and resolution. |
| `cell_of` | 4 | `crates/radiance-color-grading/src/scopes/mod.rs:418` | - |  |
| `impl CowTerrainTileGrid::cell_size_m` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:26` | - |  |
| `impl CowTerrainTileGrid::cell_size_m` | 1 | `crates/terrain-brushes/src/lib.rs:206` | - |  |
| `cell_text` | 1 | `crates/chronicle-office-codecs/src/document/markdown.rs:387` | - |  |
| `trait Host::cell_value` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:38` | - |  |
| `impl Host for PassHost<'_>::cell_value` | 3 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:255` | - |  |
| `impl Bandwidth::celt_end_band` | 1 | `crates/audio-opus-codec/src/packet.rs:30` | - | Number of CELT bands coded for this bandwidth. |
| `impl Bounds2::center` | 1 | `crates/drafting-geometry/src/bounds.rs:55` | 2d |  |
| `impl Bounds::center` | 1 | `crates/surface-pbr-studio/src/math.rs:331` | 3d |  |
| `center_stays_finite_for_opposite_large_coordinates` | 0 | `crates/drafting-geometry/src/bounds.rs:92` | - |  |
| `centers` | 1 | `crates/vector-radiance-trace/src/quantize.rs:317` | - |  |
| `impl EarthModel::central_angle_degrees` | 3 | `crates/drafting-geometry/src/geodesic_ruler.rs:37` | - |  |
| `impl Grid::centre` | 2 | `crates/vector-radiance-trace/src/centerline.rs:188` | - |  |
| `ch_w` | 4 | `crates/discourse-academic-tex/src/typeset.rs:477` | - |  |
| `impl Jet<N>::chain` | 4 | `crates/kinetics-garment-draping/src/math.rs:280` | - |  |
| `chain` | 2 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:289` | - |  |
| `chain_case` | 1 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:152` | - |  |
| `impl Shaper::change` | 4 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:148` | - |  |
| `change_dag_records_and_enforces_roles` | 0 | `crates/kinetics-schedule-cpm/src/change_dag.rs:334` | - |  |
| `impl GpuProfiler::change_settings` | 2 | `crates/gpu-command-profiler/src/profiler.rs:117` | - | Changes the settings of an existing profiler. |
| `change_settings_while_query_open` | 0 | `crates/gpu-command-profiler/tests/src/errors.rs:85` | - |  |
| `changed_profile_parameters_change_real_geometry` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:65` | - |  |
| `changed_profile_parameters_change_section_dimensions_and_area` | 0 | `crates/profile-authoring/src/tests.rs:60` | - |  |
| `impl LineSeg::channel_bin` | 1 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:80` | - |  |
| `impl PsdColorMode::channel_count` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:81` | - |  |
| `channel_layouts_3_to_8` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:233` | - |  |
| `channel_roundtrips_every_curve` | 0 | `crates/color-transforms/src/transform.rs:370` | - |  |
| `channel_stats` | 2 | `crates/radiance-color-grading/src/scopes/summary.rs:61` | numerics |  |
| `impl ElementType::channels` | 1 | `crates/audio-aac-codec/src/config.rs:21` | - |  |
| `impl AudioSpecificConfig::channels` | 1 | `crates/audio-aac-codec/src/config.rs:272` | - |  |
| `impl Decoder::channels` | 1 | `crates/audio-aac-codec/src/decoder.rs:105` | - |  |
| `impl SupportedStreamConfig::channels` | 1 | `crates/audio-device-cpal/src/lib.rs:391` | - |  |
| `impl SupportedStreamConfigRange::channels` | 1 | `crates/audio-device-cpal/src/lib.rs:612` | - |  |
| `impl StreamDecoder::channels` | 1 | `crates/audio-opus-codec/src/decoder.rs:62` | - |  |
| `impl Decoder::channels` | 1 | `crates/audio-opus-codec/src/multistream.rs:53` | - |  |
| `impl Toc::channels` | 1 | `crates/audio-opus-codec/src/packet.rs:93` | - |  |
| `impl ColorSpace::channels` | 1 | `crates/color-transforms/src/icc/profile.rs:156` | - |  |
| `impl Profile::channels` | 1 | `crates/color-transforms/src/icc/profile.rs:205` | - |  |
| `impl AacConfig::channels` | 1 | `crates/media-isobmff/src/codec.rs:573` | - | Channel count implied by the channel configuration (0 if unspecified). |
| `char_advance` | 1 | `crates/locus-parametric-solver/src/typography/stroke.rs:119` | - |  |
| `char_class` | 1 | `crates/discourse-academic-tex/src/typeset.rs:168` | - | Class of a raw character for spacing purposes. |
| `impl SGlyph::char_glyph` | 1 | `crates/vector-glyph-shaping/src/composer.rs:30` | - |  |
| `char_to_svg_subpath` | 5 | `crates/asset-production-vibe/src/glyph/mod.rs:193` | - | Emits the clean vector strokes for `ch` into `out` as SVG path instructions `M ... L ...`. |
| `char_to_svg_subpath_ext` | 6 | `crates/asset-production-vibe/src/glyph/stroke_table.rs:5` | - |  |
| `impl Ttf::char_width` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:154` | - | Advance width of a character in font units (0 for .notdef). |
| `impl AssetHost<'_>::charge` | 3 | `crates/asset-production-vibe/src/lib.rs:179` | - |  |
| `chart_lookup_and_normal_balance` | 0 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:120` | 3d |  |
| `check` | 2 | `crates/audio-aac-codec/tests/encoder_oracle.rs:124` | - |  |
| `check` | 1 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:10` | - |  |
| `check` | 1 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:27` | - | Detects user-defined struct values reachable from the `@fragment` entry point. |
| `check_accessibility` | 1 | `crates/discourse-pdf-compliance/src/a11y/checker.rs:88` | - |  |
| `check_complete` | 1 | `crates/audio-aac-codec/src/huffman.rs:235` | - |  |
| `check_config` | 4 | `crates/audio-device-cpal/src/host/asio/stream.rs:631` | - | Check whether or not the desired config is supported by the stream. |
| `impl Cur<'a>::check_count` | 3 | `crates/media-isobmff/src/bytes.rs:137` | - |  |
| `check_dangles` | 2 | `crates/geodesy-spatial-analysis/src/network_topology.rs:325` | - | Checks for dangles (endpoints of lines that do not connect to any other line endpoint within tolerance). |
| `check_duplicate_vertices` | 2 | `crates/geodesy-spatial-analysis/src/network_topology.rs:275` | - | Checks for consecutive duplicate vertices within a distance tolerance. |
| `check_fixture` | 1 | `crates/media-isobmff/tests/oracle_demux.rs:174` | - |  |
| `check_function` | 3 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:34` | - |  |
| `check_function` | 4 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:78` | - |  |
| `check_golden` | 5 | `crates/media-test-support/src/golden.rs:234` | - | Compare `actual` with the golden PNG at `golden` (or bless it). `title` and `generated_by` |
| `check_golden_accepts_within_tolerance_and_reports_failures` | 0 | `crates/media-test-support/src/golden.rs:317` | - |  |
| `check_keys` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:173` | - |  |
| `check_keys` | 2 | `crates/kinetics-block-simulation/src/blocks/params.rs:6` | - |  |
| `check_linear_kernel` | 3 | `crates/vector-hybrid-renderer/src/filter.rs:877` | numerics |  |
| `impl JournalLedger::check_lock` | 2 | `crates/chronicle-fiduciary-ledger/src/journal.rs:88` | - |  |
| `check_os_status` | 1 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:32` | - |  |
| `check_packets` | 4 | `crates/media-isobmff/tests/oracle_demux.rs:127` | - | `strict`: ffprobe timestamps must equal ours + edit offset. Otherwise (edit starting mid-frame, |
| `check_redundancy` | 4 | `crates/locus-parametric-solver/src/solver/conflict.rs:75` | - | Check if a candidate constraint is redundant with existing active constraints. |
| `check_round_trip` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:833` | - |  |
| `check_sample` | 3 | `crates/video-h264-preview/src/lib.rs:396` | - | Reject in-band SPS/PPS changes, B/SP/SI slices and mixed IDR/non-IDR |
| `check_self_intersections` | 1 | `crates/geodesy-spatial-analysis/src/network_topology.rs:293` | - | Checks for self-intersections within a polyline. |
| `check_size` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:38` | - |  |
| `check_stream` | 3 | `crates/media-isobmff/tests/oracle_demux.rs:30` | - |  |
| `impl Sps::check_supported` | 1 | `crates/video-h264-codec/src/params.rs:527` | - |  |
| `check_texture_size` | 3 | `crates/pixel-buffer-output/src/builder.rs:447` | - | Compare the given size to the limits defined by `device`. |
| `check_top_level` | 1 | `crates/video-h264-preview/src/lib.rs:202` | - | Admission for the owning demuxer: reject fragmented/event/extended boxes |
| `checked` | 1 | `crates/video-h264-codec/src/slice.rs:503` | - |  |
| `impl Tick::checked_add` | 2 | `crates/timeline-editing/src/timebase.rs:70` | - |  |
| `impl TransitionSpan::checked_end` | 1 | `crates/timeline-editing/src/transition.rs:40` | - |  |
| `checked_instance_count` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2636` | - |  |
| `impl Tick::checked_sub` | 2 | `crates/timeline-editing/src/timebase.rs:74` | - |  |
| `impl XmlNode::child` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:157` | - |  |
| `child_binding` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:1002` | - |  |
| `child_bindings` | 0 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:375` | - |  |
| `impl XmlNode::child_text` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:165` | - |  |
| `children` | 3 | `crates/video-h264-preview/src/lib.rs:106` | - |  |
| `impl XmlNode::children_named` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:161` | - |  |
| `chisq_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:213` | - |  |
| `chol` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:439` | numerics | Cholesky factorization: R' R = A for symmetric positive-definite A. |
| `cholesky_factorize_and_solve_2x2` | 0 | `crates/kinetics-continuum-multiphysics/src/solver/cholesky.rs:78` | numerics |  |
| `choose_quantisers` | 3 | `crates/video-prores-codec/src/encode.rs:333` | - |  |
| `chord_params` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:101` | - |  |
| `chord_params` | 1 | `crates/vector-radiance-trace/src/fit.rs:488` | - |  |
| `chords` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:87` | - | Music-like: a sequence of decaying harmonic chords. |
| `impl Profile::chroma` | 1 | `crates/video-prores-codec/src/types.rs:118` | - |  |
| `impl Sps::chroma_array_type` | 1 | `crates/video-h264-codec/src/params.rs:447` | - | ChromaArrayType. |
| `chroma_dc` | 0 | `crates/video-h264-codec/src/transform.rs:277` | - |  |
| `chroma_dc_dequant_420` | 3 | `crates/video-h264-codec/src/transform.rs:75` | - | 4:2:0 chroma DC: 2x2 transform + scaling (8.5.11). `c` = [c00, c01, c10, c11]. |
| `chroma_dc_rules` | 0 | `crates/video-h264-codec/src/intra.rs:449` | - |  |
| `impl Frame::chroma_height` | 1 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:70` | - |  |
| `impl SliceDecoder<'a>::chroma_qp` | 3 | `crates/video-h264-codec/src/slicedec.rs:413` | - |  |
| `impl Frame::chroma_width` | 1 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:63` | - |  |
| `impl ChromaFormat::chroma_width` | 2 | `crates/video-prores-codec/src/types.rs:31` | - | Width of a chroma plane for a given luma width. |
| `impl Frame::chroma_width` | 1 | `crates/video-prores-codec/src/types.rs:191` | - |  |
| `impl Frame::chroma_window` | 8 | `crates/video-h264-codec/src/picture.rs:244` | - | Copy the w x h window of chroma component `c` at (x0, y0) (clamped) into `out`. |
| `chronicle_locked_dt_movie_otio_ledger_objects` | 0 | `crates/asset-production-vibe/src/chronicle_ops.rs:876` | - |  |
| `chronicle_mux_mp4_origin_still_returns_mp4` | 0 | `crates/asset-production-vibe/src/chronicle_ops.rs:858` | - |  |
| `impl AssetHost<'_>::chronicle_op` | 3 | `crates/asset-production-vibe/src/chronicle_ops.rs:13` | - |  |
| `cid_info` | 1 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:120` | - |  |
| `impl Model::circle` | 3 | `crates/locus-parametric-solver/src/solver/model.rs:262` | 2d |  |
| `circle` | 2 | `crates/profile-authoring/src/lib.rs:48` | - |  |
| `circle` | 4 | `crates/vector-locus-geometry/src/recognize.rs:338` | - |  |
| `circle_circle` | 2 | `crates/drafting-geometry/src/intersect.rs:55` | 2d | Intersections of two full circles. |
| `circle_circle_hits` | 0 | `crates/drafting-geometry/src/tests.rs:106` | - |  |
| `circle_three_points_accepts_small_valid_triangles` | 0 | `crates/drafting-geometry/src/tests.rs:241` | - |  |
| `circle_three_points_preserves_large_world_coordinates` | 0 | `crates/drafting-geometry/src/tests.rs:231` | - |  |
| `circle_through_three_points` | 0 | `crates/drafting-geometry/src/tests.rs:32` | - |  |
| `civil_from_days` | 1 | `crates/chronicle-tabular-engine/src/core/date.rs:41` | - |  |
| `impl AngleRange::clamp` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:95` | - |  |
| `clamp` | 3 | `crates/layout-presentation-deck/src/morph/interpolate.rs:5` | - |  |
| `clamp_i64` | 1 | `crates/timeline-editing/src/timebase.rs:214` | - |  |
| `clamp_index` | 2 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:29` | - |  |
| `impl Pixels<'win>::clamp_pixel_pos` | 2 | `crates/pixel-buffer-output/src/lib.rs:730` | 2d | Clamp a pixel position to the pixel buffer texture size. |
| `clamp_size` | 3 | `crates/layout-box-autolayout/src/types.rs:147` | - |  |
| `clamped_uniform_knots` | 2 | `crates/drafting-geometry/src/spline.rs:214` | - |  |
| `impl AxisRom::classify` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:147` | - |  |
| `classify` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:85` | - |  |
| `classify_bidi` | 1 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:18` | - |  |
| `impl MixerState::clean` | 1 | `crates/raster-layer-mask-adapter/src/mixer.rs:67` | - |  |
| `clean` | 2 | `crates/vector-locus-pathops/src/three_d/visibility.rs:49` | - |  |
| `clean_tab` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:668` | - |  |
| `impl ScreenBuffer::clear` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:153` | - |  |
| `impl DtvccWindow::clear` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea708.rs:64` | - |  |
| `impl GeodesicRuler::clear` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:273` | - |  |
| `impl References::clear` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:118` | - |  |
| `impl ImageCache::clear` | 2 | `crates/vector-cpu-render-adapter/src/image_cache.rs:132` | - | Drop all cached conversions and their registry entries. |
| `impl Clear for Draw::clear` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:57` | - |  |
| `impl Clear for OpaqueDraw::clear` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:112` | - |  |
| `impl DrawBuffers::clear` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:362` | - |  |
| `impl FilterPassPlan::clear` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:471` | - |  |
| `impl FilterContext::clear` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:621` | - |  |
| `impl ScheduleBuffers::clear` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:981` | - |  |
| `impl ScheduleStorage::clear` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1001` | - |  |
| `impl Clear for Ranges::clear` | 1 | `crates/vector-hybrid-renderer/src/util.rs:138` | - |  |
| `impl Recording::clear_all` | 2 | `crates/vector-compute-renderer/src/recording.rs:202` | - | Commands to clear the whole buffer. |
| `impl Scheduler<'a, 'p>::clear_and_release_allocation` | 4 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:763` | - | Schedule an allocation to be cleared and released after the given round. |
| `impl WebGlRenderer::clear_atlas_region` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:828` | - | Clear a specific region of an atlas texture. |
| `clear_atlas_region` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1099` | 2d |  |
| `impl Renderer::clear_atlas_region` | 6 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:626` | - | Clear a specific region of the atlas texture. |
| `clear_atlas_region` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:863` | 2d |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::clear_color` | 2 | `crates/pixel-buffer-output/src/builder.rs:260` | - | Set the clear color. |
| `impl Pixels<'win>::clear_color` | 2 | `crates/pixel-buffer-output/src/lib.rs:308` | - | Change the clear color. |
| `impl ClearSettings<'_>::clear_color` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:45` | - |  |
| `clear_color` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3424` | - |  |
| `clear_destroys_registry_entries` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:227` | - |  |
| `impl RendererContext<'_>::clear_full_target` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2619` | - |  |
| `impl VelloCpuImageRenderer::clear_image_cache` | 1 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:23` | - | Drop all cached image conversions. |
| `impl VelloCpuScenePainter::clear_image_cache` | 1 | `crates/vector-cpu-render-adapter/src/scene.rs:56` | - | Drop all cached image conversions. |
| `impl Programs::clear_layer_bind_group_caches` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1856` | - |  |
| `impl Backend for WebGlRendererContext<'_>::clear_pass` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3287` | - |  |
| `impl Backend for RendererContext<'_>::clear_pass` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3152` | - |  |
| `trait Backend::clear_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:46` | - | Execute a clear pass with the given rectangles against the target. |
| `impl Backend for Recorder::clear_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:251` | - |  |
| `impl WebGlRendererContext<'_>::clear_pass_inner` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3173` | - |  |
| `impl RendererContext<'_>::clear_pass_inner` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3010` | - |  |
| `impl Round::clear_passes` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:216` | - |  |
| `impl TriggerReceiver::clear_pipe` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:148` | - |  |
| `impl AllocatedTextureRegion::clear_region` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:180` | - |  |
| `impl SurfaceRenderer<'s>::clear_surface_texture` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:244` | - |  |
| `clicks` | 2 | `crates/audio-aac-codec/tests/common/mod.rs:109` | - | Clicks: sharp impulses every 250 ms over a quiet low tone (transient test). |
| `impl Bsp::clip` | 3 | `crates/manifold-bim-csg/src/csg.rs:268` | - |  |
| `impl SkiaScenePainter<'_>::clip` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:116` | - |  |
| `clip` | 1 | `crates/video-h264-codec/src/intra.rs:223` | - |  |
| `clip1` | 1 | `crates/video-h264-codec/src/inter.rs:49` | - |  |
| `clip3` | 3 | `crates/video-h264-codec/src/deblock.rs:76` | - |  |
| `clip_color` | 1 | `crates/raster-blend-operators/src/lib.rs:475` | - |  |
| `clip_color` | 1 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:83` | 3d |  |
| `clip_layers_respect_fill_rule` | 0 | `crates/vector-cpu-render-adapter/src/scene.rs:281` | - |  |
| `clip_leaf` | 2 | `crates/vector-compute-shaders/src/cpu/clip_leaf.rs:75` | - |  |
| `clip_leaf_main` | 5 | `crates/vector-compute-shaders/src/cpu/clip_leaf.rs:20` | - |  |
| `clip_polygon` | 3 | `crates/vector-locus-pathops/src/pathops/boolean.rs:122` | computational geometry | Greiner-Hormann polygon clipping between two simple polygons. |
| `impl ScalingRenderer::clip_rect` | 1 | `crates/pixel-buffer-output/src/renderers.rs:291` | 2d | Get the clipping rectangle for the scaling renderer. |
| `impl ScalingMatrix::clip_rect` | 1 | `crates/pixel-buffer-output/src/renderers.rs:382` | 2d |  |
| `clip_reduce` | 2 | `crates/vector-compute-shaders/src/cpu/clip_reduce.rs:50` | - |  |
| `clip_reduce_main` | 5 | `crates/vector-compute-shaders/src/cpu/clip_reduce.rs:10` | - |  |
| `impl Bsp::clip_to` | 3 | `crates/manifold-bim-csg/src/csg.rs:302` | - |  |
| `clip_u8` | 1 | `crates/video-h264-codec/src/transform.rs:87` | - |  |
| `clipped_away_blend` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:308` | - |  |
| `clipped_away_filtered_blend_under_atlas_pressure` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:332` | - |  |
| `clips_extreme_pocs_before_narrowing` | 0 | `crates/video-h264-codec/src/slicedec.rs:274` | - |  |
| `clog2_decode` | 1 | `crates/color-transforms/src/log.rs:164` | - |  |
| `clog2_decode` | 1 | `crates/radiance-color-grading/src/log.rs:172` | - |  |
| `clog2_encode` | 1 | `crates/color-transforms/src/log.rs:160` | - |  |
| `clog2_encode` | 1 | `crates/radiance-color-grading/src/log.rs:162` | - |  |
| `clog3_decode` | 1 | `crates/color-transforms/src/log.rs:183` | - |  |
| `clog3_decode` | 1 | `crates/radiance-color-grading/src/log.rs:191` | - |  |
| `clog3_encode` | 1 | `crates/color-transforms/src/log.rs:173` | - |  |
| `clog3_encode` | 1 | `crates/radiance-color-grading/src/log.rs:181` | - |  |
| `impl XmlWriter::close` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:102` | - |  |
| `close` | 2 | `crates/drafting-geometry/src/tests.rs:3` | - |  |
| `impl OutlinePen for BezPathOutline::close` | 1 | `crates/vector-compute-renderer/src/scene.rs:1099` | - |  |
| `close_all` | 1 | `crates/vector-locus-pathops/src/pathops/offset.rs:14` | - |  |
| `impl SubpathBuilder<'a>::closed` | 2 | `crates/asset-production-vibe/src/glyph/mod.rs:170` | - |  |
| `impl Partition::closed` | 1 | `crates/chronicle-broadcast-containers/src/mxf/partition.rs:62` | - |  |
| `closed` | 1 | `crates/vector-locus-geometry/src/hit.rs:19` | - | `path` with each open subpath closed by a straight line back to its start. |
| `closed_corners` | 2 | `crates/vector-locus-geometry/src/recognize.rs:119` | - |  |
| `closed_polygon` | 1 | `crates/profile-authoring/src/lib.rs:52` | computational geometry |  |
| `closed_spline_is_smooth_at_the_seam` | 0 | `crates/drafting-geometry/src/tests.rs:149` | - |  |
| `impl Line::closest` | 2 | `crates/drafting-geometry/src/curve.rs:35` | computational geometry, 2d | Closest point on the segment. |
| `impl Arc::closest` | 2 | `crates/drafting-geometry/src/curve.rs:103` | 2d |  |
| `impl Circle::closest` | 2 | `crates/drafting-geometry/src/curve.rs:171` | 2d |  |
| `impl Ellipse::closest` | 2 | `crates/drafting-geometry/src/curve.rs:284` | 2d |  |
| `impl Segment::closest` | 2 | `crates/drafting-geometry/src/curve.rs:344` | 2d |  |
| `impl Polyline::closest` | 2 | `crates/drafting-geometry/src/curve.rs:538` | 2d |  |
| `closest` | 3 | `crates/vector-locus-geometry/src/snap.rs:26` | - | Find the closest target within `tol` to `v`. Returns `(target, delta)`. |
| `closest_point_on_triangle` | 4 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:74` | computational geometry | Computes the exact closest point on a 3D triangle to a query point P |
| `closest_target` | 0 | `crates/vector-locus-geometry/src/snap.rs:61` | - |  |
| `cmap12_lookup` | 4 | `crates/discourse-academic-tex/src/texpdf.rs:210` | - |  |
| `cmap4_lookup` | 4 | `crates/discourse-academic-tex/src/texpdf.rs:231` | - |  |
| `cmd` | 4 | `crates/discourse-academic-tex/src/catalog.rs:79` | - |  |
| `impl Ord for DijkstraItem::cmp` | 2 | `crates/asset-production-vibe/src/locus_generative/rmf_geodesic.rs:23` | - |  |
| `impl Ord for State::cmp` | 2 | `crates/geodesy-spatial-analysis/src/network_topology.rs:90` | - |  |
| `impl Ord for FloodItem::cmp` | 2 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:54` | - |  |
| `impl SupportedStreamConfigRange::cmp_default_heuristics` | 2 | `crates/audio-device-cpal/src/lib.rs:696` | - | A comparison function which compares two [`SupportedStreamConfigRange`]s in terms of their priority of |
| `cmp_guid` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:200` | - |  |
| `cmyk_planes` | 1 | `crates/raster-blend-operators/src/lib.rs:268` | - | Splits a CMYK colour into two additive planes for blending. |
| `coarse` | 2 | `crates/vector-compute-shaders/src/cpu/coarse.rs:392` | - |  |
| `coarse_main` | 9 | `crates/vector-compute-shaders/src/cpu/coarse.rs:183` | - |  |
| `coated_cmyk` | 0 | `crates/color-transforms/src/icc/synth.rs:41` | - |  |
| `impl CellError::code` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:55` | - |  |
| `impl EarthModel::code` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:23` | - |  |
| `impl DepKind::code` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:175` | - |  |
| `code` | 1 | `crates/radiance-color-grading/src/scopes/tests.rs:37` | - |  |
| `codebooks_are_complete_prefix_codes` | 0 | `crates/audio-aac-codec/src/huffman.rs:253` | - |  |
| `impl Track::codec` | 1 | `crates/media-isobmff/src/demux.rs:126` | - | The first sample entry's codec configuration. |
| `codeword_roundtrip_all_codebooks` | 0 | `crates/video-prores-codec/src/bits.rs:241` | - |  |
| `impl FrameHeader::coding_unit_size` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:64` | - |  |
| `coef_quant_roundtrip` | 0 | `crates/audio-aac-codec/src/tns.rs:282` | - |  |
| `coeff_token_tables_are_prefix_codes` | 0 | `crates/video-h264-codec/src/cavlc.rs:204` | - |  |
| `col` | 1 | `crates/discourse-academic-tex/src/catalog.rs:72` | - |  |
| `impl TableData::col_count` | 1 | `crates/layout-paged-typesetter/src/table_layout/measure.rs:37` | - |  |
| `trait StripAlphaFillSegmentExt::col_idx` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:566` | - |  |
| `impl StripAlphaFillSegmentExt for StripAlphaFillSegment::col_idx` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:571` | - |  |
| `col_to_letters` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:102` | - | 0 -> `A`, 25 -> `Z`, 26 -> `AA`, 16383 -> `XFD`. |
| `impl Matrix::col_vector` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:85` | - |  |
| `collapse_collinear` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:164` | numerics | Drop interior samples on ruled/linear spans so they stay named two-point profiles. |
| `collatz_iterations` | 1 | `crates/gpu-command-profiler/examples/compute_shader.wgsl:11` | - |  |
| `collect` | 1 | `crates/locus-parametric-solver/src/solver/auto.rs:36` | - |  |
| `collect_called_functions` | 2 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:175` | - |  |
| `collect_palette` | 2 | `crates/raster-spatial-operators/src/octree.rs:172` | graph |  |
| `impl ColorStopsSource for ColorStopsConverter<'_>::collect_stops` | 2 | `crates/vector-compute-renderer/src/scene.rs:1211` | - |  |
| `collect_stream_info_recursive` | 4 | `crates/gpu-command-profiler/src/puffin.rs:19` | - |  |
| `impl XEl::collect_text` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:91` | - |  |
| `impl LbmGrid3D::collide` | 1 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:99` | - | Two-relaxation-time (TRT) collision operator with magic parameter Lambda = 1/4. |
| `impl SliceDecoder<'a>::colocated` | 2 | `crates/video-h264-codec/src/slicedec.rs:553` | - | Co-located (mvCol, refIdxCol, referenced picture id) for 4x4 block `raster` of the current MB. |
| `color` | 1 | `crates/asset-production-vibe/src/geometry.rs:411` | - |  |
| `color4f_from_alpha_color` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1151` | - |  |
| `color4f_from_dynamic_color` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1160` | - |  |
| `impl CfaPattern::color_at` | 3 | `crates/radiance-camera-raw/src/demosaic.rs:21` | - | Returns 0 for Red, 1 for Green, 2 for Blue at sensor coordinate (x, y). |
| `color_burn` | 2 | `crates/raster-blend-operators/src/lib.rs:417` | - |  |
| `color_burn` | 2 | `crates/raster-blend-operators/src/migration_oracle.rs:21` | - |  |
| `color_burn` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:38` | - |  |
| `color_burn_u8` | 2 | `crates/raster-blend-operators/src/lib.rs:36` | - | Byte-quantized Color Burn operator in `0..=255`. |
| `color_dodge` | 2 | `crates/raster-blend-operators/src/lib.rs:427` | - |  |
| `color_dodge` | 2 | `crates/raster-blend-operators/src/migration_oracle.rs:31` | - |  |
| `color_dodge` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:28` | - |  |
| `color_dodge_u8` | 2 | `crates/raster-blend-operators/src/lib.rs:43` | - | Byte-quantized Color Dodge operator in `0..=255`. |
| `color_index` | 2 | `crates/vector-compute-renderer/src/scene.rs:1177` | - |  |
| `color_mode_palette_size_matches_request` | 0 | `crates/vector-radiance-trace/src/tests.rs:167` | - |  |
| `color_palette` | 2 | `crates/vector-radiance-trace/src/quantize.rs:235` | - | Color mode's palette for the histogram bins `used`, as `params.palette` asks. |
| `color_to_value` | 1 | `crates/asset-production-vibe/src/layout_ops/mod.rs:61` | - |  |
| `colorants` | 4 | `crates/color-transforms/src/icc/builtin.rs:111` | - |  |
| `colour_bars_parade_levels_and_vectorscope_targets` | 0 | `crates/radiance-color-grading/src/scopes/tests.rs:78` | - |  |
| `impl RangeRef::cols` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:157` | - | Whole columns `c0..=c1`. |
| `impl SectionProps::column_width` | 1 | `crates/layout-paged-typesetter/src/pagination/types.rs:80` | - |  |
| `com_initialized` | 0 | `crates/audio-device-cpal/src/host/wasapi/com.rs:55` | - | Ensures that COM is initialized in this thread. |
| `comb_filter` | 10 | `crates/audio-opus-codec/src/celt/mod.rs:593` | - | Pitch post-filter (in place on `buf[off .. off + n]`, reading history before `off`). |
| `impl Mode::combine` | 3 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:39` | - |  |
| `impl IntermediateTextureAllocations::combine` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:233` | - | Combine the current set of allocations with another one. |
| `combine_draw_monoid` | 2 | `crates/vector-compute-shaders/shader/shared/drawtag.wgsl:38` | - |  |
| `combine_tag_monoid` | 2 | `crates/vector-compute-shaders/shader/shared/pathtag.wgsl:48` | - |  |
| `combined_audio_edit_preserves_video_timeline` | 0 | `crates/video-h264-preview/src/lib.rs:648` | - |  |
| `commands_and_groups` | 0 | `crates/discourse-academic-tex/src/texparse/lex.rs:153` | - |  |
| `impl Parser::comment_entry` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:163` | - |  |
| `comments_and_mentions` | 0 | `crates/kinetics-schedule-cpm/src/change_dag.rs:344` | - |  |
| `comments_eat_to_eol` | 0 | `crates/discourse-academic-tex/src/texparse/lex.rs:170` | - |  |
| `commit_identifier_exhaustion_does_not_destroy_active_preview` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:196` | - |  |
| `impl CowTerrainTileGrid::commit_stroke` | 2 | `crates/terrain-brushes/adaptation/grid-impl.rs:188` | - | Commit preparation refuses without changing preview, redo, IDs or history. |
| `impl CowTerrainTileGrid::commit_stroke` | 2 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:245` | - |  |
| `impl CowTerrainTileGrid::commit_stroke` | 2 | `crates/terrain-brushes/src/lib.rs:373` | - | Commit preparation refuses without changing preview, redo, IDs or history. |
| `impl Circle::common_tangents` | 2 | `crates/drafting-geometry/src/curve.rs:195` | 2d | Lines tangent to both `self` and `other`, as (point on `self`, point on `other`) pairs: |
| `common_tangents_of_two_circles` | 0 | `crates/drafting-geometry/src/tests.rs:207` | - |  |
| `compact` | 2 | `crates/vector-radiance-trace/src/quantize.rs:110` | - | Drop unused palette entries and merge duplicates. |
| `impl GradientRampCache::compact_luts` | 3 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:188` | - | Remove LUT data for evicted entries with compacting the LUTs vector, and update remaining offsets. |
| `compare` | 1 | `crates/audio-aac-codec/tests/decoder_oracle.rs:28` | - | Returns (max abs error, samples compared). |
| `compare` | 2 | `crates/chronicle-tabular-engine/src/core/value.rs:223` | - |  |
| `compare_numbers` | 2 | `crates/chronicle-tabular-engine/src/core/value.rs:215` | - |  |
| `compare_text` | 2 | `crates/chronicle-tabular-engine/src/core/value.rs:219` | - |  |
| `compare_values` | 3 | `crates/locus-geospatial-codecs/src/telemetry/query.rs:82` | - |  |
| `compatible` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:57` | - |  |
| `impl Model::compile` | 1 | `crates/kinetics-block-simulation/src/model.rs:104` | - |  |
| `impl NodeGraphCompiler::compile` | 2 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:46` | - |  |
| `impl Timeline::compile` | 1 | `crates/layout-presentation-deck/src/timeline/timeline.rs:21` | - |  |
| `compile_assembly` | 2 | `crates/profile-authoring/src/assembly.rs:164` | - |  |
| `impl AssetHost<'_>::compile_bim_ifc4` | 2 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:360` | - | 124: `hypermedia.manifold.compile_bim_ifc4` |
| `impl AssetHost<'_>::compile_botanical_organ_10d` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/botanical_vertebrate_10d.rs:22` | computational geometry | Compiles a 4-scale botanical `.10d` organ container (`Scale 1` whole organism, `Scale 2` |
| `impl AssetHost<'_>::compile_bsdf_graph` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:14` | graph |  |
| `impl AssetHost<'_>::compile_dataflow_nodegraph` | 2 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:353` | - | 134: `hypermedia.layout.compile_dataflow_nodegraph` (`layout-nodegraph-compiler` + `layout-presentation-deck`) |
| `impl AssetHost<'_>::compile_deck_pptx` | 2 | `crates/asset-production-vibe/src/discourse_docs/docx_pptx.rs:159` | - |  |
| `compile_deck_pptx_emits_one_paragraph_per_bullet` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:1069` | - |  |
| `compile_deck_pptx_emits_openxml_object` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:1032` | - |  |
| `impl AssetHost<'_>::compile_emf` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:57` | - | Synthesizes a 3D wave-optical & electromagnetic field volume and seals it |
| `compile_green_dipole` | 1 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:875` | - |  |
| `compile_green_dipole` | 1 | `crates/asset-production-vibe/src/sound.rs:231` | - |  |
| `impl AssetHost<'_>::compile_human_welfare_anatomy_10d` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:1538` | - | Morphism `hypermedia.manifold.compile_human_welfare_anatomy_10d`: |
| `compile_human_welfare_q42_bytes` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:1273` | - | Compiles a deterministic 832-byte `.q42` (`Q42LOGIC`) welfare & maternal-fetal rule container |
| `impl AssetHost<'_>::compile_m64` | 2 | `crates/asset-production-vibe/src/geodesy_ops.rs:439` | - |  |
| `compile_m64_holo_occupancy_omits_record_crc` | 0 | `crates/asset-production-vibe/src/geodesy_ops.rs:792` | - |  |
| `impl AssetHost<'_>::compile_m64_occupancy` | 6 | `crates/asset-production-vibe/src/geodesy_ops.rs:547` | - |  |
| `compile_m64_point_occupancy_omits_record_crc` | 0 | `crates/asset-production-vibe/src/geodesy_ops.rs:1042` | - |  |
| `impl AssetHost<'_>::compile_m64_strata` | 5 | `crates/asset-production-vibe/src/geodesy_ops.rs:486` | - |  |
| `compile_m64_strata_default_and_e64_magic_alias_round_trip` | 0 | `crates/asset-production-vibe/src/geodesy_ops.rs:739` | - |  |
| `compile_model_logic_q42_bytes` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:615` | - | Compiles a deterministic `.q42` (`Q42LOGIC` 64-byte header + 16 parity-verified 48-byte `NQuin` |
| `impl AssetHost<'_>::compile_pdf` | 2 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:294` | - |  |
| `compile_pdf_emits_tagged_pdf_ua_object` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:910` | - |  |
| `impl AssetHost<'_>::compile_prose_docx` | 2 | `crates/asset-production-vibe/src/discourse_docs/docx_pptx.rs:21` | - |  |
| `compile_prose_docx_emits_openxml_object` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:984` | - |  |
| `compile_rule_pack_q42` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:314` | - | Compiles rules into a `Q42LOGIC` container. Lamport clocks follow rule order, starting at 1. |
| `compile_shader` | 2 | `crates/vector-hybrid-shaders/build.rs:98` | 3d |  |
| `compile_stage` | 5 | `crates/vector-hybrid-shaders/src/compile.rs:61` | 3d |  |
| `impl AssetHost<'_>::compile_vertebrate_anatomy_10d` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/botanical_vertebrate_10d.rs:152` | computational geometry | Compiles a Scale-5 comparative vertebrate 3-layer `.10d` anatomy container (`Osteology`, |
| `compile_vertebrate_anatomy_10d_is_stacked_ellipsoids_not_a_clinic_heart` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:170` | computational geometry | `compile_vertebrate_anatomy_10d` is a stacked-ellipsoid / primitive scaffold |
| `compile_wgsl_shader` | 5 | `crates/vector-hybrid-shaders/src/compile.rs:22` | 3d | Compiles the given wgsl source into GLSL using [naga]. |
| `impl Partition::complete` | 1 | `crates/chronicle-broadcast-containers/src/mxf/partition.rs:66` | - |  |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::complete_resume` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:187` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::complete_resume` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:406` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::complete_resume` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:430` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::complete_resume` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:148` | - |  |
| `impl Parser<'_>::complex` | 1 | `crates/manifold-bim-csg/src/step.rs:293` | - |  |
| `component_bits` | 1 | `crates/video-prores-codec/src/encode.rs:271` | - |  |
| `components` | 1 | `crates/drafting-geometry/src/region.rs:246` | 3d, graph | Connected component id of every vertex. |
| `impl AssetHost<'_>::compose` | 2 | `crates/asset-production-vibe/src/graphics.rs:261` | - |  |
| `compose` | 5 | `crates/vector-glyph-shaping/src/composer.rs:96` | - | Knuth-Plass line composition over glyph slice `g`. |
| `compose_empty_paragraph` | 0 | `crates/vector-glyph-shaping/src/composer.rs:239` | - |  |
| `compose_invert_and_round_trip` | 0 | `crates/vector-locus-geometry/src/projective.rs:129` | - |  |
| `compose_knuth_plass_mixed_bidi_is_produce_safe` | 0 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:374` | - |  |
| `compose_knuth_plass_newline_splits_paragraphs` | 0 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:469` | - |  |
| `impl AssetHost<'_>::compose_knuth_plass_paragraph` | 2 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:197` | - | 131: `hypermedia.glyph.compose_knuth_plass_paragraph` |
| `compose_knuth_plass_paragraph_converges_and_two_produces_match` | 0 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:328` | - |  |
| `compose_knuth_plass_rtl_latin_n1_and_arabic_indic_produce` | 0 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:436` | - |  |
| `impl Scheduler<'a, 'p>::compose_layer` | 5 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:594` | - | Compose a rendered child into a parent layer. |
| `impl Scheduler<'a, 'p>::compose_simple_layer` | 5 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:732` | - | Compose a completed child by drawing it directly into its parent using source-over. |
| `compose_single_line_fit` | 0 | `crates/vector-glyph-shaping/src/composer.rs:246` | - |  |
| `composite` | 3 | `crates/raster-blend-operators/src/lib.rs:250` | - | Source-over compositing of `source` onto `backdrop` with `mode` (straight RGBA in `0..=1`). |
| `impl PbrLayerStack::composite` | 3 | `crates/surface-pbr-studio/src/compose.rs:93` | - | Evaluates the layer stack into rendered images for all present channels. |
| `impl PixelsRendererOptions::composite_alpha_mode` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:53` | - |  |
| `impl VelloRendererOptions::composite_alpha_mode` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:123` | - |  |
| `impl VelloHybridRendererOptions::composite_alpha_mode` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:120` | - |  |
| `impl SkiaRendererOptions::composite_alpha_mode` | 2 | `crates/vector-skia-render-adapter/src/window_renderer.rs:55` | - |  |
| `impl GpuFilterData::composite_original` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:335` | - |  |
| `compositing_layers_respect_fill_rule` | 0 | `crates/vector-cpu-render-adapter/src/scene.rs:286` | - |  |
| `impl HumanWelfareMeasurements::compute` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:137` | - |  |
| `impl PocState::compute` | 3 | `crates/video-h264-codec/src/slice.rs:502` | - | Compute TopFieldOrderCnt/BottomFieldOrderCnt of a frame (8.2.1.1-8.2.1.3). |
| `compute_allocation` | 10 | `crates/audio-opus-codec/src/celt/rate.rs:285` | - | `clt_compute_allocation` (decoder side). |
| `compute_aspect` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:117` | - | Computes aspect in degrees clockwise from North (0..360, 0 = North, 90 = East, etc.). Flat terrain is assigned -1.0. |
| `compute_baseline_variance` | 2 | `crates/kinetics-schedule-cpm/src/schedule.rs:287` | - |  |
| `compute_box_colors` | 2 | `crates/asset-production-vibe/src/layout_ops/mod.rs:269` | - |  |
| `compute_box_corner_radius` | 3 | `crates/asset-production-vibe/src/layout_ops/mod.rs:258` | - |  |
| `compute_caps` | 1 | `crates/audio-opus-codec/src/celt/rate.rs:212` | - |  |
| `impl NeoHookean::compute_cauchy_stress` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:127` | - | Compute Cauchy stress sigma from deformation gradient F (3x3). |
| `compute_class_breaks` | 3 | `crates/geodesy-spatial-analysis/src/cartography.rs:83` | - | Computes class break boundaries for graduated symbology. |
| `compute_curvature_at` | 4 | `crates/geodesy-spatial-analysis/src/morphometry.rs:177` | - | Computes profile and plan curvature at (col, row) using Zevenbergen-Thorne polynomial coefficients. |
| `compute_doppler_spectrum` | 4 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:253` | physics, numerics | Computes a short-time Doppler velocity spectrum from a time-series of CSI amplitude changes. |
| `compute_evm` | 2 | `crates/kinetics-schedule-cpm/src/schedule.rs:515` | - |  |
| `impl FiduciaryAuditEntry::compute_hash` | 9 | `crates/chronicle-fiduciary-ledger/src/audit.rs:34` | - |  |
| `compute_hillshade` | 4 | `crates/geodesy-spatial-analysis/src/morphometry.rs:138` | - | Computes analytical hillshade illumination values (0..255). |
| `impl AssetHost<'_>::compute_mikktspace` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/slim_mikktspace.rs:494` | - |  |
| `compute_node_hashes` | 1 | `crates/layout-nodegraph-compiler/src/compiler/cse.rs:52` | graph | Compute deterministic hash trees for all nodes in topological order. |
| `impl Mesh::compute_normals` | 1 | `crates/surface-pbr-studio/src/mesh.rs:48` | 3d | Compute face/area-weighted vertex normals if not provided. |
| `impl GpuProfilerQuery::compute_pass_timestamp_writes` | 1 | `crates/gpu-command-profiler/src/profiler_query.rs:92` | - | Use the reserved query for compute pass timestamp writes if any. |
| `impl SaintVenantKirchhoff::compute_pk2_stress` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:98` | - | Compute Second Piola-Kirchhoff stress tensor S from Green-Lagrange strain tensor E (3x3). |
| `compute_pulse_cache` | 1 | `crates/audio-opus-codec/src/celt/rate.rs:151` | - |  |
| `compute_qn` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:224` | - |  |
| `compute_slope` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:103` | - | Computes slope in degrees (0..90). |
| `compute_smooth_normals` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:62` | computational geometry, 3d | Compute area-weighted smooth unit vertex normals for a triangle mesh. |
| `compute_smooth_vertex_normals` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:54` | computational geometry, 3d | Compute smooth area-weighted per-vertex unit normals for a 3D mesh. |
| `compute_spatial_covariance` | 2 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:41` | numerics | Computes the $M \times M$ complex spatial covariance matrix $R_{xx} = \frac{1}{K} \sum x_k x_k^H$. |
| `compute_tag_monoid` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:687` | - |  |
| `compute_tag_monoid` | 3 | `crates/vector-compute-shaders/src/cpu/flatten.rs:553` | - |  |
| `impl BandCtx<'_, '_, '_>::compute_theta` | 8 | `crates/audio-opus-codec/src/celt/bands.rs:273` | - |  |
| `impl BrushFalloff::compute_weight` | 3 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:59` | - |  |
| `impl BrushFalloff::compute_weight` | 3 | `crates/terrain-brushes/src/lib.rs:65` | - |  |
| `compute_workload` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:343` | - |  |
| `compute_zonal_stats` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:407` | - | Computes zonal statistics for cells matching `target_zone` in a zone integer raster. |
| `concat` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:6` | - |  |
| `impl SkiaScenePainter<'_>::concat_matrix` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:112` | numerics |  |
| `concave_channel_caps_preserve_wall_void_and_volume` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:38` | - |  |
| `concave_channel_caps_preserve_wall_void_and_volume` | 0 | `crates/profile-authoring/src/tests.rs:122` | - |  |
| `impl StreamDecoder::conceal` | 3 | `crates/audio-opus-codec/src/decoder.rs:113` | - | Conceals `n` samples (a multiple of 2.5 ms). |
| `impl Channel::conceal` | 2 | `crates/audio-opus-codec/src/silk/mod.rs:419` | - | Basic concealment: pitch-periodic repetition for voiced history, otherwise LPC-shaped |
| `impl Decoder::config` | 1 | `crates/audio-aac-codec/src/decoder.rs:93` | - |  |
| `impl Encoder::config` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:236` | - |  |
| `impl SupportedStreamConfig::config` | 1 | `crates/audio-device-cpal/src/lib.rs:407` | - |  |
| `impl Encoder::config` | 1 | `crates/video-prores-codec/src/encode.rs:64` | - |  |
| `config_layout` | 1 | `crates/audio-aac-codec/src/config.rs:140` | - | Element layout of `channelConfiguration` 1..=7 (tags as conventionally assigned). |
| `config_to_waveformatextensible` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:950` | - |  |
| `impl SurfaceRenderer<'s>::configure` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:239` | - |  |
| `impl Decoder::configure_avcc` | 2 | `crates/video-h264-codec/src/decoder.rs:190` | - | Parse an `avcC` record: SPS/PPS and the NAL length size used by [`Decoder::decode`]. |
| `configure_for_device` | 3 | `crates/audio-device-cpal/src/host/oboe/mod.rs:205` | - |  |
| `configure_for_recording` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:384` | - |  |
| `impl RenderContext::configure_surface` | 2 | `crates/vector-compute-renderer/src/util.rs:139` | - |  |
| `conflict_summary` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:334` | - |  |
| `impl Complex32::conj` | 1 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:47` | - |  |
| `impl Homography::conjugated` | 2 | `crates/vector-locus-geometry/src/projective.rs:91` | - | `self` seen through `a`: the map `a ∘ self ∘ a⁻¹`. |
| `impl Stream::connect_to_system_inputs` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:184` | - | Connect to the standard system outputs in jack, system:capture_1 and system:capture_2 |
| `impl Stream::connect_to_system_outputs` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:158` | - | Connect to the standard system outputs in jack, system:playback_1 and system:playback_2 |
| `connection` | 4 | `crates/color-transforms/src/icc/transform.rs:197` | - |  |
| `conservative_history_reference_budget_refuses_commit_without_destroying_preview` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:275` | - |  |
| `console_output` | 2 | `crates/gpu-command-profiler/examples/demo.rs:40` | - |  |
| `consolidate_entities` | 3 | `crates/chronicle-fiduciary-ledger/src/consolidation.rs:54` | - | Performs consolidation across entities with translation and elimination. |
| `constrain_45` | 0 | `crates/vector-locus-geometry/src/lib.rs:108` | - |  |
| `constrain_angle` | 2 | `crates/vector-locus-geometry/src/lib.rs:68` | 2d | Snap a vector to the nearest multiple of `step_deg`. |
| `constrain_angle_from` | 3 | `crates/vector-locus-geometry/src/lib.rs:73` | 2d | [`constrain_angle`] in steps counted from `base_deg`. |
| `construct_reflection_map_from_fragment` | 0 | `crates/vector-hybrid-shaders/src/types.rs:262` | - |  |
| `construct_reflection_map_from_vertex` | 0 | `crates/vector-hybrid-shaders/src/types.rs:212` | 3d |  |
| `impl Reader<'a>::consume` | 2 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:47` | - |  |
| `impl Reader<'a>::consume` | 2 | `crates/video-prores-codec/src/bits.rs:46` | - |  |
| `impl AssetHost<'_>::consume_unwrap_uv01` | 3 | `crates/asset-production-vibe/src/lib.rs:434` | - | When unwrap sealed UV01 into `.10d`, public compile_10d_pbr consumes it. |
| `impl Reader<'a>::consumed` | 1 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:53` | - |  |
| `impl Reader<'a>::consumed` | 1 | `crates/video-prores-codec/src/bits.rs:52` | - |  |
| `impl DeviceOutput::consumed_frames` | 1 | `crates/pcm-device-output/src/device.rs:215` | - |  |
| `container_and_cancellation_refuse_safely` | 0 | `crates/video-h264-preview/src/lib.rs:781` | - |  |
| `container_family` | 2 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:176` | - | Produce-safe family for gallery presentation of sealed Q42 containers. |
| `impl AngleRange::contains` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:87` | - |  |
| `impl SampleRange::contains` | 2 | `crates/audio-timebase/src/rate.rs:105` | - |  |
| `impl RangeRef::contains` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:189` | - |  |
| `impl Bounds2::contains` | 2 | `crates/drafting-geometry/src/bounds.rs:66` | 2d |  |
| `impl Polyline::contains` | 2 | `crates/drafting-geometry/src/curve.rs:550` | 2d | True if `p` is inside a closed polyline (even-odd on the tessellation). |
| `impl Extent::contains` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:83` | - |  |
| `impl DebugLayers::contains` | 2 | `crates/vector-compute-renderer/src/debug.rs:103` | - | Determine whether `self` is a superset of `mask`. |
| `impl Lru<K>::contains` | 2 | `crates/video-h264-preview/src/movie.rs:112` | - |  |
| `impl Arc::contains_angle` | 2 | `crates/drafting-geometry/src/curve.rs:83` | - |  |
| `impl Bounds2::contains_box` | 2 | `crates/drafting-geometry/src/bounds.rs:70` | - | True when `o` lies entirely inside. |
| `impl GenerationalCache<K, T>::contains_key` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:53` | - |  |
| `impl RangeRef::contains_range` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:193` | - |  |
| `impl SectionProps::content_height` | 1 | `crates/layout-paged-typesetter/src/pagination/types.rs:76` | - |  |
| `impl SectionProps::content_width` | 1 | `crates/layout-paged-typesetter/src/pagination/types.rs:72` | - |  |
| `impl Pixels<'win>::context` | 1 | `crates/pixel-buffer-output/src/lib.rs:757` | - | Provides access to the internal [`PixelsContext`]. |
| `context_around` | 3 | `crates/discourse-academic-tex/src/review/suggest.rs:70` | - | Capture context around a range in `src` on char boundaries. |
| `context_init_formula` | 0 | `crates/video-h264-codec/src/cabac.rs:293` | - |  |
| `context_ok` | 5 | `crates/discourse-academic-tex/src/review/suggest.rs:49` | - |  |
| `contour_loops_are_consistent` | 0 | `crates/vector-radiance-trace/src/tests.rs:156` | - |  |
| `contract_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:159` | - |  |
| `contract_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:144` | - |  |
| `contract_impl` | 4 | `crates/raster-selection-morphology/src/lib.rs:226` | - |  |
| `contributions` | 3 | `crates/raster-spatial-operators/src/resample.rs:138` | - |  |
| `impl PathData::control_bounds` | 1 | `crates/vector-locus-geometry/src/path.rs:448` | 2d | Bounds including control handles (used for quick culling). |
| `impl OutConv::conv` | 2 | `crates/video-prores-codec/src/decode.rs:55` | - |  |
| `conv_brush` | 3 | `crates/vector-compute-renderer/src/scene.rs:1120` | - |  |
| `conv_extend` | 1 | `crates/vector-compute-renderer/src/scene.rs:1198` | - |  |
| `conv_point` | 1 | `crates/vector-compute-renderer/src/scene.rs:1194` | - |  |
| `conv_skrifa_transform` | 1 | `crates/vector-compute-renderer/src/scene.rs:1113` | - |  |
| `impl InputTransform::convert` | 2 | `crates/color-transforms/src/transform.rs:259` | 2d, semantics, numerics | Signal → working linear for one RGB triple (table + pixel stage). |
| `convert` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:262` | - | Convert source buffer (CSL-JSON or RIS) to BibTeX entries. |
| `convert_data_type` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:208` | - |  |
| `impl DimensionConstraints::convert_dimension` | 1 | `crates/vector-hybrid-renderer/src/util.rs:82` | - |  |
| `convert_edge_mode` | 1 | `crates/vector-cpu-render-adapter/src/filters.rs:56` | graph |  |
| `convert_edge_mode` | 1 | `crates/vector-hybrid-render-adapter/src/filters.rs:53` | graph |  |
| `impl Transform::convert_f32` | 6 | `crates/color-transforms/src/icc/transform.rs:112` | - | Converts interleaved float pixels. |
| `convert_filter` | 1 | `crates/vector-cpu-render-adapter/src/filters.rs:9` | - |  |
| `convert_filter` | 1 | `crates/vector-hybrid-render-adapter/src/filters.rs:6` | - |  |
| `convert_filter` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:603` | - |  |
| `convert_filter_effect` | 1 | `crates/vector-cpu-render-adapter/src/filters.rs:23` | - |  |
| `convert_filter_effect` | 1 | `crates/vector-hybrid-render-adapter/src/filters.rs:20` | - |  |
| `convert_filter_effect` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:614` | - |  |
| `convert_image` | 1 | `crates/vector-cpu-render-adapter/src/image_convert.rs:15` | - | Convert a [`peniko::ImageData`] to a premultiplied RGBA8 [`Pixmap`]. |
| `impl WebGlScenePainter<'_>::convert_image_paint` | 2 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:90` | - |  |
| `convert_mb_to_rgb` | 3 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:544` | - |  |
| `impl VelloCpuScenePainter::convert_paint` | 2 | `crates/vector-cpu-render-adapter/src/scene.rs:32` | - |  |
| `impl VelloHybridScenePainter<'_>::convert_paint` | 2 | `crates/vector-hybrid-render-adapter/src/scene.rs:152` | - |  |
| `impl WebGlScenePainter<'_>::convert_paint` | 2 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:78` | - |  |
| `impl Transform::convert_u8` | 6 | `crates/color-transforms/src/icc/transform.rs:134` | - | Converts interleaved 8-bit pixels. |
| `convex` | 1 | `crates/asset-production-vibe/src/geometry.rs:393` | - |  |
| `convolve_rgba32f` | 4 | `crates/raster-spatial-operators/src/convolve.rs:132` | - | General 2D convolution for RGBA32F image buffer. |
| `convolve_rgba8` | 4 | `crates/raster-spatial-operators/src/convolve.rs:87` | - | General 2D convolution for RGBA8 image buffer. |
| `cool_teal_orange_preset` | 1 | `crates/asset-production-vibe/src/radiance_color.rs:21` | - |  |
| `impl Lut3d::coord` | 3 | `crates/color-transforms/src/lut.rs:99` | - | Normalised grid coordinate (0..size-1) of an input value. |
| `impl Lut3d::coord` | 3 | `crates/radiance-color-grading/src/lut.rs:92` | - |  |
| `impl AlphaConvertBlitter::copy` | 5 | `crates/gpu-device-context/src/alpha_blitter.rs:156` | - | Copy `source` into `target`, applying the alpha conversion. |
| `impl IntermediateBlitter::copy` | 5 | `crates/gpu-device-context/src/surface_renderer.rs:41` | - |  |
| `copy_clamped` | 3 | `crates/video-h264-codec/src/picture.rs:306` | graph | Copy `out.len()` samples of `line` starting at x0, replicating edge samples outside the line. |
| `impl Frame::copy_cropped` | 2 | `crates/video-h264-codec/src/picture.rs:256` | - | Crop and copy the frame into planar buffers (waits for completion). |
| `impl GpuBlendInstance::copy_from_scratch` | 1 | `crates/vector-hybrid-renderer/src/blend.rs:85` | - |  |
| `impl CodedPlanes::copy_into` | 4 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:158` | - |  |
| `copy_line` | 4 | `crates/video-h264-codec/src/picture.rs:292` | - | Copy `w` samples of `line` starting at x0 into `out`; when possible a fixed-size block of `N` |
| `copy_n` | 3 | `crates/video-h264-codec/src/inter.rs:55` | - | Copy `n` bytes with a fixed-size move for the common block widths. |
| `impl FilterPassPlan::copy_pass` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:467` | - |  |
| `copy_quoted` | 3 | `crates/vector-hybrid-shaders/src/minify.rs:264` | - | Copies one quoted value verbatim, including escaped characters. |
| `impl BufferRenderer::copy_texture_to_buffer` | 2 | `crates/gpu-device-context/src/buffer_renderer.rs:127` | - |  |
| `impl BufferRenderer::copy_texture_to_vec` | 2 | `crates/gpu-device-context/src/buffer_renderer.rs:121` | - |  |
| `copy_to_channel` | 3 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:523` | - |  |
| `copy_to_texture` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3466` | - | Copy from a source texture/framebuffer to an atlas texture. |
| `impl Anchor::corner` | 1 | `crates/vector-locus-geometry/src/path.rs:115` | - | A corner anchor without handles. |
| `impl TraceParams::corner_angle` | 1 | `crates/vector-radiance-trace/src/lib.rs:184` | - |  |
| `impl Track::corner_positions` | 1 | `crates/vector-surface-brush/src/track.rs:117` | - | Arc-length positions of the corner vertices. |
| `corner_sources` | 2 | `crates/vector-locus-geometry/src/shapes.rs:238` | 2d |  |
| `impl Bounds2::corners` | 1 | `crates/drafting-geometry/src/bounds.rs:82` | 2d |  |
| `corpus` | 0 | `crates/audio-aac-codec/tests/robustness.rs:9` | - |  |
| `corpus_joint_ranges_cite_assertion_ids_and_unsourced_rules_stay_assumptions` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:473` | - |  |
| `corrupt_pages_are_skipped_and_truncation_is_safe` | 0 | `crates/media-ogg/src/tests.rs:138` | - |  |
| `impl Corner::cos` | 1 | `crates/vector-locus-geometry/src/corners.rs:51` | - | The cosine of the angle between the sides. |
| `count` | 4 | `crates/asset-production-vibe/src/lib.rs:521` | - |  |
| `impl RangeRef::count` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:173` | - |  |
| `count` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:262` | - |  |
| `count` | 3 | `crates/kinetics-block-simulation/src/blocks/params.rs:55` | - |  |
| `impl Cur<'a>::count` | 2 | `crates/media-isobmff/src/bytes.rs:132` | - | Read a count and verify that `count * entry_size` bytes remain (allocation guard). |
| `count_bits` | 2 | `crates/audio-aac-codec/src/huffman.rs:182` | - | Bits needed to code `vals` (a multiple of the codebook dimension) with codebook `cb` (1..=11). |
| `impl Dsu::count_islands` | 1 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:129` | - |  |
| `count_leaves` | 2 | `crates/raster-spatial-operators/src/octree.rs:111` | graph |  |
| `impl AssetHost<'_>::couple_continuum_xpbd` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/m64_continuum.rs:395` | - | Evaluates Bekker / Mohr-Coulomb terramechanics pressure-sinkage, tractive shear stress, |
| `cow_and_numeric_budget_refusals_are_atomic` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:140` | - |  |
| `cow_untouched_tiles_zero_copy` | 0 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:378` | - |  |
| `cow_untouched_tiles_zero_copy` | 0 | `crates/terrain-brushes/src/lib.rs:544` | - |  |
| `cpm_schedules_sample_program` | 0 | `crates/kinetics-schedule-cpm/src/schedule.rs:840` | - |  |
| `crater_profile` | 2 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:48` | - | Profile calculation for relative crater elevation offset at `normalized_dist` (0 = center, 1 = outer rim lip). |
| `crc` | 0 | `crates/bitstream-codec/src/lib.rs:407` | - |  |
| `crc32` | 1 | `crates/bitstream-codec/src/lib.rs:330` | - | CRC-32 (IEEE, reflected) — used by containers (e.g. Matroska CRC-32 elements, PNG). |
| `crc32` | 1 | `crates/chronicle-office-codecs/src/zip/minizip.rs:32` | - | Standard CRC-32 (IEEE 802.3). |
| `crc32` | 1 | `crates/media-ogg/src/lib.rs:96` | - | CRC-32 of RFC 3533 §6: polynomial 0x04C11DB7, initial value 0, no reflection, no final XOR. |
| `crc32_ieee` | 1 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:23` | - | Branchless IEEE 802.3 CRC-32 checksum (polynomial 0xEDB88320). |
| `crc32_mpeg` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/psi.rs:5` | - | CRC-32 MPEG-2 (polynomial 0x04C11DB7, initial 0xFFFFFFFF). |
| `crc_matches_known_value` | 0 | `crates/media-ogg/src/tests.rs:83` | - |  |
| `impl PrivacyWitnessReceipt::create` | 5 | `crates/electromagnetics-rf-sensing/src/densepose.rs:125` | - | Creates and cryptographically signs a privacy witness receipt. |
| `trait GlResource::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:18` | - |  |
| `impl GlResource for WebGlTexture::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:88` | - |  |
| `impl GlResource for WebGlBuffer::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:100` | - |  |
| `impl GlResource for WebGlFramebuffer::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:112` | - |  |
| `impl GlResource for WebGlRenderbuffer::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:125` | - |  |
| `impl GlResource for WebGlSync::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:138` | - |  |
| `impl GlResource for WebGlProgram::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:150` | - |  |
| `impl GlResource for WebGlVertexShader::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:162` | - |  |
| `impl GlResource for WebGlFragmentShader::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:175` | - |  |
| `impl GlResource for WebGlVertexArrayObject::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:188` | - |  |
| `impl Programs::create_alphas_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1939` | - |  |
| `create_arena_buffer` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:960` | - |  |
| `create_atlas_texture` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2552` | - | Create an image atlas texture. |
| `impl Programs::create_atlas_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1956` | - |  |
| `create_audio_unit` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:380` | - |  |
| `create_backing_texture` | 9 | `crates/pixel-buffer-output/src/builder.rs:464` | 3d |  |
| `impl TransientBindMap<'a>::create_bind_group` | 8 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1081` | - |  |
| `impl WgpuEngine::create_bind_group_layout_entries` | 1 | `crates/vector-compute-renderer/src/wgpu_engine.rs:787` | - |  |
| `impl Programs::create_blend_layer_bind_group` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1876` | - |  |
| `impl WGPUContext::create_buffer_renderer` | 2 | `crates/gpu-device-context/src/lib.rs:183` | - | Creates a new `BufferRenderer` for the specified dimensions. |
| `impl WgpuEngine::create_compute_pipeline` | 5 | `crates/vector-compute-renderer/src/wgpu_engine.rs:839` | - |  |
| `impl Programs::create_config_buffer` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1917` | - |  |
| `impl Programs::create_copy_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1902` | - |  |
| `impl TransientBindMap<'a>::create_cpu_resources` | 3 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1213` | - |  |
| `impl Mesh::create_cube` | 1 | `crates/surface-pbr-studio/src/mesh.rs:101` | - | Creates a unit cube centered at origin. |
| `create_data_texture_storage` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2362` | - | Create a zero-initialized data texture backed by immutable storage (`texStorage2D`). |
| `create_default_device_async` | 1 | `crates/gpu-command-profiler/tests/src/mod.rs:10` | - |  |
| `impl Renderer::create_depth_texture_view` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:216` | - | Creates a depth texture view compatible with [`render`](Self::render). |
| `create_device` | 1 | `crates/gpu-command-profiler/tests/src/mod.rs:7` | - |  |
| `impl WGPUContext::create_device` | 2 | `crates/gpu-device-context/src/lib.rs:243` | - | Creates a compatible device handle id. |
| `impl WGPUContext::create_device_handle` | 2 | `crates/gpu-device-context/src/lib.rs:230` | - |  |
| `create_encoded_gradient` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:311` | - |  |
| `impl Programs::create_encoded_paints_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2059` | - |  |
| `impl Programs::create_encoded_paints_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2042` | - |  |
| `impl Programs::create_external_texture_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2026` | - |  |
| `impl Programs::create_filter_base_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1993` | - |  |
| `impl Programs::create_filter_data_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1976` | - |  |
| `create_filter_input_bind_group` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3243` | - |  |
| `create_filter_original_texture_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3265` | - |  |
| `create_framebuffer_for_texture` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2568` | - | Create a framebuffer for a texture. |
| `create_gr_context` | 6 | `crates/vector-skia-render-adapter/src/vulkan.rs:759` | - |  |
| `create_gradient` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:320` | - |  |
| `impl Programs::create_gradient_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2091` | - |  |
| `impl Programs::create_gradient_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2074` | - |  |
| `create_instance` | 2 | `crates/vector-skia-render-adapter/src/vulkan.rs:524` | - |  |
| `create_intermediate_texture` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2532` | - |  |
| `impl Programs::create_intermediate_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1774` | - |  |
| `create_logical_device` | 3 | `crates/vector-skia-render-adapter/src/vulkan.rs:596` | - |  |
| `impl Programs::create_placeholder_external_texture` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2008` | - |  |
| `create_placeholder_rgba32ui_texture` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2379` | - | Create a 1x1 RGBA32UI placeholder texture. |
| `create_placeholder_rgba8_texture` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2391` | - | Create a 1x1 RGBA8 placeholder texture. |
| `impl Mesh::create_quad` | 1 | `crates/surface-pbr-studio/src/mesh.rs:67` | - | Creates a unit quad on the XY plane with UVs in [0, 1]. |
| `impl RenderContext::create_render_surface` | 5 | `crates/vector-compute-renderer/src/util.rs:69` | - | Creates a new render surface for the specified window and dimensions. |
| `impl Programs::create_run_external_texture_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2361` | - |  |
| `impl Programs::create_strip_bind_group` | 5 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2106` | - |  |
| `impl DeviceHandle::create_surface` | 4 | `crates/gpu-device-context/src/lib.rs:79` | - | Creates a new surface for the specified window and dimensions. |
| `impl WGPUContext::create_surface` | 2 | `crates/gpu-device-context/src/lib.rs:150` | - | Creates a new surface for the specified window and dimensions. |
| `impl RenderContext::create_surface` | 5 | `crates/vector-compute-renderer/src/util.rs:52` | - | Creates a new surface for the specified window and dimensions. |
| `impl OpenGLBackend::create_surface` | 6 | `crates/vector-skia-render-adapter/src/opengl.rs:167` | - |  |
| `impl WGPUContext::create_surface_renderer` | 4 | `crates/gpu-device-context/src/lib.rs:158` | - | Creates a new surface for the specified window and dimensions. |
| `create_swapchain` | 9 | `crates/vector-skia-render-adapter/src/vulkan.rs:626` | - |  |
| `create_sync_objects` | 1 | `crates/vector-skia-render-adapter/src/vulkan.rs:803` | - |  |
| `create_targets` | 3 | `crates/vector-compute-renderer/src/util.rs:196` | 3d | Vello uses a compute shader to render to the provided texture, which means that it can't bind the surface |
| `create_test_host` | 4 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:173` | - |  |
| `create_test_host` | 3 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:176` | - |  |
| `create_test_host` | 3 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/vox0_metamer.rs:526` | - |  |
| `create_test_host` | 3 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:741` | - |  |
| `create_test_host` | 4 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:45` | - |  |
| `create_test_host` | 4 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:42` | - |  |
| `create_test_host` | 4 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:148` | - |  |
| `create_test_host` | 3 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:103` | - |  |
| `create_test_host` | 3 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:150` | - |  |
| `create_texture` | 5 | `crates/gpu-device-context/src/util.rs:30` | - | Create a WGPU Texture, returning a default TextureView |
| `create_texture` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2268` | graph | Create a texture with the requested filtering and clamp-to-edge wrapping. |
| `create_texture_inner` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2281` | - |  |
| `create_texture_storage` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2322` | - | Create a texture backed by immutable storage (`texStorage2D`). |
| `create_tracy_gpu_client` | 3 | `crates/gpu-command-profiler/src/tracy.rs:3` | - |  |
| `create_webgl_resources` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2421` | - | Create all WebGL resources needed for rendering. |
| `impl Catalog::creator` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:256` | - |  |
| `creator_for` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:381` | - |  |
| `impl JLine::credit` | 2 | `crates/chronicle-fiduciary-ledger/src/journal.rs:34` | - |  |
| `credits_for` | 2 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:81` | - | Returns the credit cost for a given model and input type. |
| `credits_to_usd_cents` | 1 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:108` | - | Converts credits to US dollar cents. |
| `crop_preserves_texture_mapping` | 0 | `crates/vector-hybrid-renderer/src/target.rs:350` | - |  |
| `impl Sps::crop_rect` | 1 | `crates/video-h264-codec/src/params.rs:473` | 2d | Cropping rectangle in luma samples: (x, y, width, height). |
| `impl LayerTextureRegion::crop_to` | 2 | `crates/vector-hybrid-renderer/src/target.rs:257` | - | Restrict this region to the given scene-space bounds while preserving its texture mapping. |
| `impl Sps::crop_unit` | 1 | `crates/video-h264-codec/src/params.rs:464` | - | (CropUnitX, CropUnitY). |
| `cross` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:364` | - |  |
| `cross` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:223` | - |  |
| `impl Vec2::cross` | 2 | `crates/drafting-geometry/src/vec.rs:31` | 2d | z of the 3D cross product. |
| `impl Vec3::cross` | 2 | `crates/drafting-geometry/src/vec.rs:163` | 3d |  |
| `impl DVec3::cross` | 2 | `crates/kinetics-garment-draping/src/math.rs:110` | - |  |
| `cross` | 2 | `crates/kinetics-garment-draping/src/math.rs:381` | - |  |
| `cross` | 2 | `crates/manifold-bim-csg/src/csg.rs:30` | - |  |
| `impl MorphologyKernel::cross` | 1 | `crates/raster-selection-morphology/src/grayscale.rs:99` | - | Cross structuring element (`dx == 0 \|\| dy == 0`). |
| `impl Vec3::cross` | 2 | `crates/surface-pbr-studio/src/math.rs:110` | - |  |
| `impl Vec2::cross` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:85` | - |  |
| `impl V3::cross` | 2 | `crates/vector-locus-pathops/src/three_d/revolve.rs:18` | - |  |
| `cross_check_ffmpeg` | 3 | `crates/video-h264-codec/src/synth_tests.rs:439` | - | Validate the hand-built stream and its expected output with ffmpeg when available. |
| `crossing_lines_make_faces` | 0 | `crates/drafting-geometry/src/region.rs:407` | - |  |
| `cryptomatte_murmur3` | 1 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:15` | - |  |
| `cse_deduplicate` | 1 | `crates/layout-nodegraph-compiler/src/compiler/cse.rs:107` | graph | Deduplicate identical subexpressions: |
| `csg_subtract_pocket` | 0 | `crates/manifold-bim-csg/src/csg.rs:444` | - |  |
| `csg_union_and_intersect` | 0 | `crates/manifold-bim-csg/src/csg.rs:454` | - |  |
| `csl_kind` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:80` | - |  |
| `csl_names` | 2 | `crates/discourse-academic-tex/src/bibliography/import.rs:100` | - |  |
| `csl_to_bibtex` | 2 | `crates/discourse-academic-tex/src/bibliography/import.rs:125` | - |  |
| `csr_matvec_matches_dense` | 0 | `crates/kinetics-continuum-multiphysics/src/solver/sparse.rs:91` | - |  |
| `csv_escape` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:207` | - |  |
| `ctx` | 0 | `crates/audio-timebase/src/transport.rs:232` | - |  |
| `cube_roundtrip_3d_1d_and_shaper` | 0 | `crates/color-transforms/src/lut.rs:553` | - |  |
| `cube_sizes_up_to_65_and_errors` | 0 | `crates/color-transforms/src/lut.rs:583` | - |  |
| `cubic_bc` | 3 | `crates/raster-spatial-operators/src/resample.rs:82` | - |  |
| `cubic_bezier` | 5 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:272` | - | Evaluates a 4-point cubic Bezier curve at parameter `t` in `[0, 1]`. |
| `cubic_end_normal` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:316` | 3d |  |
| `cubic_end_tangent` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:303` | - |  |
| `cubic_end_tangent` | 4 | `crates/vector-compute-shaders/src/cpu/flatten.rs:63` | 2d |  |
| `cubic_eval` | 5 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:16` | - |  |
| `cubic_from_points_derivs` | 5 | `crates/vector-compute-shaders/shader/flatten.wgsl:94` | - | Compute cubic parameters from endpoints and derivatives. |
| `cubic_start_normal` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:311` | 3d |  |
| `cubic_start_tangent` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:295` | - |  |
| `cubic_start_tangent` | 4 | `crates/vector-compute-shaders/src/cpu/flatten.rs:50` | 2d |  |
| `cubic_weights` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:925` | - |  |
| `cuboid` | 2 | `crates/manifold-bim-csg/src/csg.rs:428` | - |  |
| `cumipmt` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:115` | - |  |
| `cumprinc` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:130` | - |  |
| `impl Parser::cur` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:86` | - |  |
| `impl WebGlRenderer::current_allocations` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:586` | - |  |
| `impl Renderer::current_allocations` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:393` | - |  |
| `impl VelloWindowRenderer::current_device_handle` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:190` | - |  |
| `impl VelloHybridWindowRenderer::current_device_handle` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:179` | - |  |
| `impl glifo::GlyphRenderer for Scene::current_paint` | 1 | `crates/vector-hybrid-renderer/src/text.rs:353` | - |  |
| `impl Cursor::current_round` | 1 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:37` | - |  |
| `current_space` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:160` | - |  |
| `cursor` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:148` | - |  |
| `cursor_bounds` | 0 | `crates/media-isobmff/src/bytes.rs:355` | - |  |
| `impl ColorSpace::curve` | 1 | `crates/color-transforms/src/spaces.rs:257` | - |  |
| `impl ColorSpace::curve` | 1 | `crates/radiance-color-grading/src/spaces.rs:314` | - |  |
| `curve_elem` | 1 | `crates/color-transforms/src/icc/write.rs:33` | - |  |
| `impl OutlinePen for BezPathOutline::curve_to` | 7 | `crates/vector-compute-renderer/src/scene.rs:1091` | - |  |
| `impl Corner::cut` | 3 | `crates/vector-locus-geometry/src/corners.rs:97` | - | The two anchors replacing the corner when cut with `radius` (> 0, fitted): where the cut |
| `cut_antimeridian_line` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:616` | - | Splits a polyline at the antimeridian (where consecutive points jump across > 180 degrees longitude). |
| `impl SubPath::cut_at` | 2 | `crates/vector-locus-geometry/src/path.rs:310` | - | Cut at the anchors `at`: each becomes the end of one open piece and the start of the next, |
| `cut_corners` | 3 | `crates/vector-locus-geometry/src/corners.rs:150` | - | `path` with each of its corners ([`path_corners`]) cut by the radius in `radii` and the kind in |
| `cut_out` | 2 | `crates/vector-locus-pathops/src/pathops/planar.rs:57` | - | Cut out the portion of `target` that is covered by `tool`. |
| `cut_out_removes_overlapping_area` | 0 | `crates/vector-locus-pathops/src/pathops/planar.rs:75` | - |  |
| `cw_len` | 2 | `crates/video-prores-codec/src/bits.rs:131` | - | Number of bits of the codeword for `val` in codebook `cb`. |
| `cyber_neon_preset` | 1 | `crates/asset-production-vibe/src/radiance_color.rs:38` | - |  |
| `cycle_eq_rotated` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:352` | - |  |
| `cycle_eq_rotated` | 2 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:675` | - |  |
| `cycle_interior` | 3 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:43` | - | Vertices strictly between `from` and `to` walking the cycle forward. |
| `cylinder_revolves_successfully` | 0 | `crates/vector-locus-pathops/src/three_d/revolve.rs:274` | - |  |
| `d50_quantized` | 0 | `crates/color-transforms/src/icc/math.rs:74` | - | Exact quantized D50 in ICC header fixed-point representation. |
| `d65` | 0 | `crates/color-transforms/src/icc/builtin.rs:107` | - |  |
| `d8_flow_directions` | 3 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:189` | - | Compute D8 steepest descent flow directions for each cell. |
| `impl BitReader<'a>::data` | 1 | `crates/bitstream-codec/src/lib.rs:44` | - |  |
| `impl Cabac<'a>::data` | 1 | `crates/video-h264-codec/src/cabac.rs:138` | - |  |
| `impl Device::data_flow` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:520` | - |  |
| `impl Endpoint::data_flow` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:831` | - |  |
| `data_flow_from_immendpoint` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:143` | - |  |
| `date_and_sidecar_fields` | 0 | `crates/media-test-support/src/golden.rs:337` | - |  |
| `dated_flows` | 2 | `crates/chronicle-tabular-engine/src/functions/financial.rs:245` | - |  |
| `datetime_from_serial` | 2 | `crates/chronicle-tabular-engine/src/core/date.rs:101` | - |  |
| `days_from_civil` | 3 | `crates/chronicle-tabular-engine/src/core/date.rs:31` | - | Days from 0000-03-01 style civil algorithm (Howard Hinnant), days since 1970-01-01. |
| `days_in_month` | 2 | `crates/chronicle-tabular-engine/src/core/date.rs:61` | - |  |
| `days_in_month_in` | 3 | `crates/chronicle-tabular-engine/src/core/date.rs:71` | - |  |
| `db` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:404` | - |  |
| `dcmd` | 4 | `crates/discourse-academic-tex/src/catalog.rs:85` | - |  |
| `impl Mdct::dct4` | 3 | `crates/audio-aac-codec/src/mdct.rs:87` | - | DCT-IV: `out[k] = Σ u[n] cos(π/M (n+½)(k+½))`. |
| `ddb` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:426` | - |  |
| `dead_blob_is_evicted` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:174` | - |  |
| `impl Atlases::deallocate` | 2 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:76` | - |  |
| `trait AtlasExt::deallocate_region` | 2 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:202` | - |  |
| `impl AtlasExt for Atlas::deallocate_region` | 2 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:230` | - |  |
| `impl JLine::debit` | 2 | `crates/chronicle-fiduciary-ledger/src/journal.rs:24` | - |  |
| `deblock_mb` | 3 | `crates/video-h264-codec/src/deblock.rs:283` | - | Deblock one macroblock (macroblocks must be processed in raster order). |
| `decimal_digits` | 1 | `crates/chronicle-tabular-engine/src/numfmt/decimal.rs:5` | - |  |
| `declared_sample_sizes_cannot_trigger_unbounded_reads` | 0 | `crates/video-h264-preview/src/lib.rs:764` | - |  |
| `declares_document_class` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:182` | - |  |
| `impl Decoder::decode` | 2 | `crates/audio-aac-codec/src/decoder.rs:126` | - | Decode one access unit (one `raw_data_block`) into 1024 samples per channel. |
| `impl Tree::decode` | 2 | `crates/audio-aac-codec/src/huffman.rs:85` | - |  |
| `impl CeltDecoder::decode` | 6 | `crates/audio-opus-codec/src/celt/mod.rs:124` | - | Decodes one CELT frame of `frame_size` samples (at the output rate) into interleaved `pcm` |
| `impl Decoder::decode` | 2 | `crates/audio-opus-codec/src/multistream.rs:88` | - | Decodes one packet (`None` = lost packet, concealed) into planar channels. |
| `impl RangeDecoder<'a>::decode` | 2 | `crates/audio-opus-codec/src/range.rs:111` | - | Returns the cumulative frequency of the next symbol for total `ft` (call [`Self::update`] next). |
| `impl SilkDecoder::decode` | 6 | `crates/audio-opus-codec/src/silk/mod.rs:691` | - | Decodes one SILK frame (10 or 20 ms) into interleaved `out` at the API rate. |
| `impl GradeSpace::decode` | 2 | `crates/color-transforms/src/grade.rs:85` | numerics | Grading signal → working linear. SDR clamps to 0…1 (white); HDR keeps values above HDR |
| `impl LogCurve::decode` | 2 | `crates/color-transforms/src/log.rs:86` | numerics | Signal (curve domain) → scene-linear. |
| `decode` | 1 | `crates/manifold-bim-csg/src/step.rs:333` | - |  |
| `impl GradeSpace::decode` | 2 | `crates/radiance-color-grading/src/grade.rs:78` | numerics | Grading signal -> working linear. |
| `impl LogCurve::decode` | 2 | `crates/radiance-color-grading/src/log.rs:87` | numerics | Log signal -> linear scene reflectance. |
| `decode` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:84` | - | Decodes PackBits compressed data until `expected_len` bytes are produced. |
| `impl PsdDocument::decode` | 1 | `crates/surface-layered-codecs/src/psd/codec.rs:154` | - | Decodes a PSD or PSB document from bytes. |
| `impl PsdHeader::decode` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:129` | - |  |
| `impl VlcTable::decode` | 2 | `crates/video-h264-codec/src/cavlc.rs:46` | - |  |
| `impl Decoder::decode` | 3 | `crates/video-h264-codec/src/decoder.rs:224` | - | Decode one access unit (all NAL units of one picture, Annex-B or length-prefixed per |
| `decode_adsb_packet` | 1 | `crates/locus-geospatial-codecs/src/telemetry/aircraft.rs:28` | - | Decodes 14-byte (112-bit) raw ADS-B Mode S packet. |
| `decode_ais_char` | 1 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:25` | - | Decodes an AIS 6-bit ASCII character to its 6-bit integer value (0..63). |
| `decode_aivdm_sentence` | 1 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:91` | - | Decodes an NMEA 0183 `!AIVDM` sentence into an `AisMessage`. |
| `decode_alpha` | 3 | `crates/video-prores-codec/src/decode.rs:416` | - | Decode a slice's alpha plane (raster order over the slice's 16 lines) into `out` |
| `impl RangeDecoder<'a>::decode_bin` | 2 | `crates/audio-opus-codec/src/range.rs:120` | - | Like [`Self::decode`] with `ft = 1 << bits`. |
| `impl Decoder::decode_block` | 3 | `crates/audio-aac-codec/src/decoder.rs:130` | - |  |
| `decode_block` | 5 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:464` | - |  |
| `impl Cabac<'a>::decode_bypass` | 1 | `crates/video-h264-codec/src/cabac.rs:179` | - |  |
| `impl SliceDecoder<'_>::decode_cabac` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:42` | - | Decode slice_data() with CABAC. `rbsp` is the whole slice RBSP. |
| `impl SliceDecoder<'a>::decode_cavlc` | 2 | `crates/video-h264-codec/src/slicedec.rs:1456` | - | Decode slice_data() with CAVLC. `r` is positioned after the slice header. |
| `decode_cbor_ld` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:37` | - |  |
| `impl SliceDecoder<'_>::decode_cbp` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:284` | - |  |
| `decode_channel` | 3 | `crates/color-transforms/src/transform.rs:41` | numerics | Decode one channel: decoder-normalised signal → working-linear (no OOTF, no matrix). |
| `impl SliceDecoder<'_>::decode_chroma_mode` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:671` | - |  |
| `decode_clean` | 1 | `crates/media-isobmff/tests/oracle_mux.rs:120` | - |  |
| `decode_component` | 5 | `crates/video-prores-codec/src/decode.rs:363` | - | Entropy-decode and dequantise one slice component into `blocks` (raster-order coefficients). |
| `decode_controls` | 1 | `crates/locus-parametric-solver/src/typography/mtext.rs:16` | - | Decode `%%` control codes: |
| `impl Channel::decode_core` | 4 | `crates/audio-opus-codec/src/silk/mod.rs:309` | - |  |
| `impl Decoder::decode_cpe` | 3 | `crates/audio-aac-codec/src/decoder.rs:226` | - |  |
| `decode_cpr_global` | 2 | `crates/locus-geospatial-codecs/src/telemetry/aircraft.rs:118` | - | Decodes geographic position (latitude, longitude) from an even and an odd CPR frame. |
| `impl Cabac<'a>::decode_decision` | 2 | `crates/video-h264-codec/src/cabac.rs:161` | - | 9.3.3.2.1, branch-free: the MPS / LPS outcome of a context-coded bin is close to random |
| `impl StreamDecoder::decode_fec` | 4 | `crates/audio-opus-codec/src/decoder.rs:130` | - | Decodes the in-band FEC (LBRR) of `packet` for the previous (lost) packet of `n` samples. |
| `impl CeltDecoder::decode_frame` | 5 | `crates/audio-opus-codec/src/celt/mod.rs:162` | - |  |
| `impl StreamDecoder::decode_frame` | 5 | `crates/audio-opus-codec/src/decoder.rs:172` | - | `opus_decode_frame`: decodes one frame (or conceals when `data` is `None`/≤1 byte). |
| `impl Channel::decode_frame` | 5 | `crates/audio-opus-codec/src/silk/mod.rs:207` | - |  |
| `decode_frame` | 1 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:87` | - |  |
| `decode_frame` | 1 | `crates/video-prores-codec/src/decode.rs:29` | - | Decode one ProRes frame (the contents of one QuickTime sample) with default options. |
| `decode_frame_into` | 3 | `crates/video-prores-codec/src/decode.rs:109` | - | Decode one ProRes frame into `out`, reusing its plane allocations (the fastest way to decode |
| `decode_frame_with` | 2 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:91` | - |  |
| `decode_frame_with` | 2 | `crates/video-prores-codec/src/decode.rs:101` | - | Decode one ProRes frame. |
| `impl Movie::decode_h264` | 2 | `crates/video-h264-preview/src/movie.rs:488` | - |  |
| `impl Movie::decode_h264_single` | 4 | `crates/video-h264-preview/src/movie.rs:471` | - |  |
| `impl Decoder::decode_ics` | 3 | `crates/audio-aac-codec/src/decoder.rs:286` | - |  |
| `impl Channel::decode_indices` | 5 | `crates/audio-opus-codec/src/silk/mod.rs:142` | - |  |
| `decode_inner` | 6 | `crates/video-h264-preview/src/lib.rs:259` | - |  |
| `impl StreamDecoder::decode_interleaved` | 3 | `crates/audio-opus-codec/src/decoder.rs:102` | - | Decodes a packet into interleaved samples appended to `out`. `None` conceals one lost |
| `impl Movie::decode_intra` | 3 | `crates/video-h264-preview/src/movie.rs:413` | - |  |
| `impl PsdDocument::decode_layer_section` | 3 | `crates/surface-layered-codecs/src/psd/codec.rs:222` | - |  |
| `impl CeltDecoder::decode_lost` | 3 | `crates/audio-opus-codec/src/celt/mod.rs:457` | - | Packet-loss concealment: pitch-periodic extension for the first lost frames, then |
| `impl SliceDecoder<'_>::decode_mb_cabac` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:522` | - | Parse and reconstruct one macroblock_layer() with CABAC. |
| `impl SliceDecoder<'a>::decode_mb_cavlc` | 2 | `crates/video-h264-codec/src/slicedec.rs:1234` | - | Parse and reconstruct one macroblock_layer() with CAVLC. |
| `impl SliceDecoder<'_>::decode_mb_type` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:104` | - |  |
| `impl SliceDecoder<'_>::decode_mb_type_i` | 3 | `crates/video-h264-codec/src/cabac_mb.rs:77` | - | Intra mb_type suffix / I-slice mb_type (Table 9-36). `base` is 3 (I slice) or 17/32 (prefix |
| `decode_mcc_hex` | 1 | `crates/chronicle-timeline-interchange/src/captions/mcc.rs:79` | - |  |
| `decode_mp4` | 4 | `crates/video-h264-preview/src/lib.rs:74` | economics | Decode a single traversal of the selected media range. Caller must supply a |
| `impl AssetHost<'_>::decode_mp4_frame` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:115` | - |  |
| `decode_mp4_stream` | 5 | `crates/video-h264-preview/src/lib.rs:86` | - | Decode to a caller-owned, backpressured sink without retaining the whole |
| `impl SliceDecoder<'_>::decode_mvd` | 6 | `crates/video-h264-codec/src/cabac_mb.rs:216` | - | One mvd component for the (sub-)partition with top-left 4x4 block (x, y) (9.3.3.1.1.7, UEG3). |
| `decode_mvt_geometry` | 2 | `crates/locus-geospatial-codecs/src/tiles/mvt.rs:31` | - | Decodes an MVT command integer sequence into vector Geometry. |
| `decode_packbits` | 2 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:94` | - |  |
| `impl StreamDecoder::decode_packet` | 3 | `crates/audio-opus-codec/src/decoder.rs:152` | - |  |
| `decode_pair` | 2 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:60` | - |  |
| `impl Channel::decode_parameters` | 2 | `crates/audio-opus-codec/src/silk/mod.rs:236` | - |  |
| `decode_pmtiles_directory` | 1 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:183` | - | Decodes an uncompressed PMTiles v3 directory into tile entries. |
| `decode_png` | 1 | `crates/media-test-support/src/golden.rs:61` | - | Decode a PNG (8-bit gray/RGB/RGBA, palette expanded) to RGBA8. |
| `decode_prores_mov` | 1 | `crates/video-prores-codec/src/mov.rs:110` | - | Decode all ProRes frames from a QuickTime `.mov` container. |
| `decode_psd_channel` | 5 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:38` | - |  |
| `decode_pulses` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:17` | - | Decodes a PVQ codeword of `k` pulses in `n` dimensions; returns the sum of squares. |
| `decode_pulses` | 5 | `crates/audio-opus-codec/src/silk/mod.rs:509` | - |  |
| `impl SliceDecoder<'_>::decode_qp_delta` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:330` | - |  |
| `impl SliceDecoder<'_>::decode_ref_idx` | 5 | `crates/video-h264-codec/src/cabac_mb.rs:181` | - | ref_idx_lX for the partition whose top-left 4x4 block is (x, y) (9.3.3.1.1.6). |
| `decode_rle_alpha` | 4 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:569` | - |  |
| `decode_row` | 3 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:368` | - |  |
| `decode_sf` | 1 | `crates/audio-aac-codec/src/huffman.rs:108` | - | Decode one scalefactor difference (`-60..=60`). |
| `impl SliceDecoder<'a>::decode_skip` | 1 | `crates/video-h264-codec/src/slicedec.rs:1212` | - | Decode a P_Skip or B_Skip macroblock at the current address. |
| `impl SliceDecoder<'_>::decode_skip_flag` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:68` | - |  |
| `decode_slice` | 2 | `crates/video-h264-codec/src/decoder.rs:571` | - |  |
| `decode_slice` | 8 | `crates/video-prores-codec/src/decode.rs:251` | - |  |
| `decode_source_10d_vertex_colors` | 1 | `crates/asset-production-vibe/src/manifold_ops.rs:847` | 3d |  |
| `decode_spectral` | 3 | `crates/audio-aac-codec/src/huffman.rs:126` | - | Decode one codeword of codebook `cb` (1..=11) into `out[..dim]`, including sign bits and escapes. |
| `decode_stroke` | 1 | `crates/locus-parametric-solver/src/typography/stroke.rs:127` | 2d |  |
| `impl SliceDecoder<'_>::decode_sub_mb_type` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:148` | - |  |
| `impl SliceDecoder<'_>::decode_t8x8_flag` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:665` | - |  |
| `impl Cabac<'a>::decode_terminate` | 1 | `crates/video-h264-codec/src/cabac.rs:190` | - |  |
| `decode_unit` | 3 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:243` | - |  |
| `decode_wav` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:78` | - | Decodes standard 16-bit PCM or 32-bit float WAV bytes into an AudioBuffer. |
| `decode_wkb` | 1 | `crates/locus-geospatial-codecs/src/wkb.rs:211` | - | Decodes a geometry from binary WKB bytes. |
| `decode_zigzag` | 1 | `crates/locus-geospatial-codecs/src/tiles/mvt.rs:26` | - | Decodes MVT zigzag-encoded integer parameter: `(n >> 1) ^ -(n & 1)`. |
| `decoded_source_import_preserves_heights_and_refuses_history_replacement` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:93` | - |  |
| `decodes_spec_style_example` | 0 | `crates/video-h264-codec/src/cavlc.rs:248` | - |  |
| `dedup` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:47` | - |  |
| `dedup` | 1 | `crates/vector-radiance-trace/src/fit.rs:431` | - |  |
| `impl CeltDecoder::deemphasis` | 4 | `crates/audio-opus-codec/src/celt/mod.rs:426` | - |  |
| `deep_reuse` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:558` | - |  |
| `deeply_nested_blend_layers` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:599` | - |  |
| `deeply_nested_layers` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:584` | - |  |
| `impl Default for BiquadCoeffs::default` | 0 | `crates/acoustics-daw-session/src/dsp/biquad.rs:30` | - |  |
| `impl Default for Oscillator::default` | 0 | `crates/acoustics-daw-session/src/dsp/osc.rs:26` | - |  |
| `impl Default for FontMetrics::default` | 0 | `crates/asset-production-vibe/src/glyph/mod.rs:25` | - |  |
| `impl Default for ToolLimits::default` | 0 | `crates/asset-production-vibe/src/lib.rs:54` | - |  |
| `impl Default for IcsInfo::default` | 0 | `crates/audio-aac-codec/src/ics.rs:42` | - |  |
| `impl Default for Devices::default` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:21` | - |  |
| `impl Default for Devices::default` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:370` | - |  |
| `impl Default for Devices::default` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:460` | - |  |
| `impl Default for NotSendSyncAcrossAllPlatforms::default` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:739` | - |  |
| `impl Default for GridDivision::default` | 0 | `crates/audio-timebase/src/grid.rs:77` | - |  |
| `impl Default for SampleRate::default` | 0 | `crates/audio-timebase/src/rate.rs:76` | - |  |
| `impl Default for TimeSig::default` | 0 | `crates/audio-timebase/src/tempo.rs:38` | - |  |
| `impl Default for TempoMap::default` | 0 | `crates/audio-timebase/src/tempo.rs:110` | - |  |
| `impl Default for MuxOptions::default` | 0 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:76` | - |  |
| `impl Default for CalcSettings::default` | 0 | `crates/chronicle-tabular-engine/src/calc/model.rs:30` | - |  |
| `impl Default for Workbook::default` | 0 | `crates/chronicle-tabular-engine/src/calc/model.rs:148` | - |  |
| `impl Default for Calc::default` | 0 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:72` | - |  |
| `impl Default for NumberFormat::default` | 0 | `crates/chronicle-tabular-engine/src/numfmt/mod.rs:24` | - |  |
| `impl Default for AlignOptions::default` | 0 | `crates/chronicle-timeline-interchange/src/captions/align.rs:18` | - |  |
| `impl Default for ScreenBuffer::default` | 0 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:143` | - |  |
| `impl Default for Segment::default` | 0 | `crates/chronicle-timeline-interchange/src/interchange/aaf.rs:76` | - |  |
| `impl Default for TimelineSequence::default` | 0 | `crates/chronicle-timeline-interchange/src/model.rs:32` | - |  |
| `impl Default for FrameRate::default` | 0 | `crates/chronicle-timeline-interchange/src/time.rs:72` | - |  |
| `impl Default for CmykParams::default` | 0 | `crates/color-transforms/src/icc/synth.rs:27` | - |  |
| `impl Default for TransformOptions::default` | 0 | `crates/color-transforms/src/icc/transform.rs:20` | - |  |
| `impl Default for Doc::default` | 0 | `crates/discourse-academic-tex/src/texpdf.rs:395` | - |  |
| `impl Default for Workspace::default` | 0 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:51` | - |  |
| `impl Default for Bounds2::default` | 0 | `crates/drafting-geometry/src/bounds.rs:13` | - |  |
| `impl Default for Mat3::default` | 0 | `crates/drafting-geometry/src/mat.rs:17` | - |  |
| `impl Default for Mat4::default` | 0 | `crates/drafting-geometry/src/mat.rs:110` | - |  |
| `impl Default for FallDetector::default` | 0 | `crates/electromagnetics-rf-sensing/src/vitals.rs:151` | - |  |
| `impl Default for CartoSymbol::default` | 0 | `crates/geodesy-spatial-analysis/src/cartography.rs:54` | - |  |
| `impl Default for Extent::default` | 0 | `crates/geodesy-spatial-analysis/src/geodesy.rs:50` | - |  |
| `impl Default for GpuProfilerSettings::default` | 0 | `crates/gpu-command-profiler/src/profiler_settings.rs:40` | - |  |
| `impl Default for WGPUContext::default` | 0 | `crates/gpu-device-context/src/lib.rs:110` | - |  |
| `impl Default for Model::default` | 0 | `crates/kinetics-block-simulation/src/model.rs:24` | - |  |
| `impl Default for SimConfig::default` | 0 | `crates/kinetics-block-simulation/src/sim.rs:38` | - |  |
| `impl Default for FabricMaterial::default` | 0 | `crates/kinetics-garment-draping/src/energy.rs:22` | - |  |
| `impl Default for NewtonSettings::default` | 0 | `crates/kinetics-garment-draping/src/newton.rs:20` | - |  |
| `impl Default for VbdSettings::default` | 0 | `crates/kinetics-garment-draping/src/vbd.rs:17` | - |  |
| `impl Default for Ease::default` | 0 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:29` | - |  |
| `impl Default for Homography::default` | 0 | `crates/kinetics-motion-compositor/src/track/homography.rs:10` | - |  |
| `impl Default for CharTransform::default` | 0 | `crates/kinetics-motion-compositor/src/typography/animator.rs:18` | - |  |
| `impl Default for RangeSelector::default` | 0 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:78` | - |  |
| `impl Default for WigglySelector::default` | 0 | `crates/kinetics-motion-compositor/src/typography/wiggly.rs:17` | - |  |
| `impl Default for Calendar::default` | 0 | `crates/kinetics-schedule-cpm/src/model.rs:421` | - |  |
| `impl Default for Project::default` | 0 | `crates/kinetics-schedule-cpm/src/model.rs:509` | - |  |
| `impl Default for AutoLayoutFrame::default` | 0 | `crates/layout-box-autolayout/src/frame.rs:99` | - |  |
| `impl Default for GridTrack::default` | 0 | `crates/layout-box-autolayout/src/types.rs:96` | - |  |
| `impl Default for NodeGraphCompiler::default` | 0 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:34` | - |  |
| `impl Default for ParaDoc::default` | 0 | `crates/layout-paged-typesetter/src/pagination/flow.rs:21` | - |  |
| `impl Default for SectionProps::default` | 0 | `crates/layout-paged-typesetter/src/pagination/types.rs:56` | - |  |
| `impl Default for Theme::default` | 0 | `crates/layout-presentation-deck/src/model/master.rs:17` | - |  |
| `impl Default for Presentation::default` | 0 | `crates/layout-presentation-deck/src/model/master.rs:122` | - |  |
| `impl Default for TableCell::default` | 0 | `crates/layout-presentation-deck/src/model/shape.rs:98` | - |  |
| `impl Default for SlideTransition::default` | 0 | `crates/layout-presentation-deck/src/model/shape.rs:141` | - |  |
| `impl Default for ColorScheme::default` | 0 | `crates/layout-presentation-deck/src/model/types.rs:135` | - |  |
| `impl Default for Line::default` | 0 | `crates/layout-presentation-deck/src/model/types.rs:201` | - |  |
| `impl Default for Animation::default` | 0 | `crates/layout-presentation-deck/src/timeline/anim.rs:41` | - |  |
| `impl Default for InferOptions::default` | 0 | `crates/locus-parametric-solver/src/solver/auto.rs:21` | - |  |
| `impl Default for MTextParams::default` | 0 | `crates/locus-parametric-solver/src/typography/mtext.rs:138` | - |  |
| `impl Default for Meter::default` | 0 | `crates/manifold-bim-csg/src/csg.rs:53` | - |  |
| `impl Default for IfcWriter::default` | 0 | `crates/manifold-bim-csg/src/ifc.rs:48` | - |  |
| `impl Default for DemuxLimits::default` | 0 | `crates/media-isobmff/src/demux.rs:31` | - |  |
| `impl Default for StageCamera::default` | 0 | `crates/multimodal-synthesis-schema/src/stage.rs:33` | - |  |
| `impl Default for PixelsRendererOptions::default` | 0 | `crates/pixel-window-render-adapter/src/lib.rs:36` | - |  |
| `impl Default for Transform::default` | 0 | `crates/profile-authoring/src/assembly.rs:18` | - |  |
| `impl Default for AssemblyLimits::default` | 0 | `crates/profile-authoring/src/assembly.rs:69` | - |  |
| `impl Default for GeometryLimits::default` | 0 | `crates/profile-authoring/src/geometry.rs:20` | - |  |
| `impl Default for ProfileLimits::default` | 0 | `crates/profile-authoring/src/profile.rs:36` | - |  |
| `impl Default for ParametricToneCurve::default` | 0 | `crates/radiance-camera-raw/src/develop.rs:43` | - |  |
| `impl Default for AscCdl::default` | 0 | `crates/radiance-color-grading/src/grade.rs:123` | - |  |
| `impl Default for ColorWheels::default` | 0 | `crates/radiance-color-grading/src/grade.rs:278` | - |  |
| `impl Default for Params::default` | 0 | `crates/radiance-color-grading/src/scopes/mod.rs:144` | - |  |
| `impl Default for StrokePoint::default` | 0 | `crates/raster-layer-mask-adapter/src/brush.rs:26` | - |  |
| `impl Default for BrushSettings::default` | 0 | `crates/raster-layer-mask-adapter/src/brush.rs:141` | - |  |
| `impl Default for BlendRange::default` | 0 | `crates/raster-layer-mask-adapter/src/masks.rs:162` | - |  |
| `impl Default for MixerSettings::default` | 0 | `crates/raster-layer-mask-adapter/src/mixer.rs:28` | - |  |
| `impl Default for BakeSettings::default` | 0 | `crates/surface-pbr-studio/src/bake.rs:33` | - |  |
| `impl Default for MaterialXOptions::default` | 0 | `crates/surface-pbr-studio/src/materialx.rs:31` | - |  |
| `impl Default for BrushSettings::default` | 0 | `crates/surface-pbr-studio/src/paint.rs:27` | - |  |
| `impl Default for BrushDynamics::default` | 0 | `crates/surface-pbr-studio/src/probrush.rs:23` | - |  |
| `impl Default for CraterfieldConfig::default` | 0 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:19` | - |  |
| `impl Default for TerraceConfig::default` | 0 | `crates/terrain-erosion-solvers/src/fractal_terrace.rs:25` | - |  |
| `impl Default for HydraulicConfig::default` | 0 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:68` | - |  |
| `impl Default for LakeConfig::default` | 0 | `crates/terrain-erosion-solvers/src/lacustrine_fill.rs:21` | - |  |
| `impl Default for SnowConfig::default` | 0 | `crates/terrain-erosion-solvers/src/snow_accumulation.rs:32` | - |  |
| `impl Default for ThermalShaperConfig::default` | 0 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:285` | - |  |
| `impl Default for FrameRate::default` | 0 | `crates/timeline-editing/src/timebase.rs:130` | - |  |
| `impl Default for RendererOptions::default` | 0 | `crates/vector-compute-renderer/src/lib.rs:411` | - |  |
| `impl Default for Render::default` | 0 | `crates/vector-compute-renderer/src/render.rs:115` | - |  |
| `impl Default for IntBbox::default` | 0 | `crates/vector-compute-shaders/src/cpu/flatten.rs:529` | - |  |
| `impl Default for ImageCacheConfig::default` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:27` | - |  |
| `impl Default for Params::default` | 0 | `crates/vector-glyph-shaping/src/composer.rs:63` | - |  |
| `impl Default for OtFeatures::default` | 0 | `crates/vector-glyph-shaping/src/features.rs:37` | - |  |
| `impl Default for AreaTextOptions::default` | 0 | `crates/vector-glyph-shaping/src/layout.rs:35` | - |  |
| `impl Default for TextOnPathOptions::default` | 0 | `crates/vector-glyph-shaping/src/layout.rs:194` | - |  |
| `impl Default for VelloRendererOptions::default` | 0 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:80` | - |  |
| `impl Default for VelloHybridRendererOptions::default` | 0 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:77` | - |  |
| `impl Default for ClearSettings<'_>::default` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:56` | - |  |
| `impl ViewFramebuffer::default` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1204` | - |  |
| `impl Default for LayersConfig::default` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:186` | - |  |
| `impl Default for RenderSettings::default` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:196` | - |  |
| `impl Default for DimensionConstraints::default` | 0 | `crates/vector-hybrid-renderer/src/util.rs:91` | - | Creates default constraints with reasonable values |
| `impl Default for SimplifyOptions::default` | 0 | `crates/vector-locus-pathops/src/pathops/edit.rs:23` | - |  |
| `impl Default for Extrude::default` | 0 | `crates/vector-locus-pathops/src/three_d/bevel.rs:34` | - |  |
| `impl Default for Revolve::default` | 0 | `crates/vector-locus-pathops/src/three_d/revolve.rs:65` | - |  |
| `impl Default for SimplifyOptions::default` | 0 | `crates/vector-radiance-trace/src/fit.rs:221` | - |  |
| `impl Default for TraceParams::default` | 0 | `crates/vector-radiance-trace/src/lib.rs:150` | - |  |
| `impl Default for GenerationalCache<K, T>::default` | 0 | `crates/vector-skia-render-adapter/src/cache.rs:23` | - |  |
| `impl Default for SkiaSceneCache::default` | 0 | `crates/vector-skia-render-adapter/src/scene.rs:59` | - |  |
| `impl Default for SkiaRendererOptions::default` | 0 | `crates/vector-skia-render-adapter/src/window_renderer.rs:38` | - |  |
| `impl Default for SkiaWindowRenderer::default` | 0 | `crates/vector-skia-render-adapter/src/window_renderer.rs:86` | - |  |
| `impl Default for ArtScale::default` | 0 | `crates/vector-surface-brush/src/art.rs:32` | - |  |
| `impl Default for ArtBrush::default` | 0 | `crates/vector-surface-brush/src/art.rs:50` | - |  |
| `impl Default for BristleBrush::default` | 0 | `crates/vector-surface-brush/src/bristle.rs:54` | - |  |
| `impl Default for CalligraphicBrush::default` | 0 | `crates/vector-surface-brush/src/calligraphic.rs:40` | - |  |
| `impl Default for BrushInputSample::default` | 0 | `crates/vector-surface-brush/src/lib.rs:35` | - |  |
| `impl Default for PatternBrush::default` | 0 | `crates/vector-surface-brush/src/pattern.rs:39` | - |  |
| `impl Default for ScatterBrush::default` | 0 | `crates/vector-surface-brush/src/scatter.rs:26` | - |  |
| `impl Default for Decoder::default` | 0 | `crates/video-h264-codec/src/decoder.rs:104` | - |  |
| `impl Default for Dpb<F>::default` | 0 | `crates/video-h264-codec/src/dpb.rs:413` | - |  |
| `impl Default for Vui::default` | 0 | `crates/video-h264-codec/src/params.rs:194` | - |  |
| `impl Default for MbState::default` | 0 | `crates/video-h264-codec/src/picture.rs:91` | - |  |
| `impl Default for DecodeOptions::default` | 0 | `crates/video-prores-codec/src/decode.rs:18` | - |  |
| `impl Default for ColorInfo::default` | 0 | `crates/video-prores-codec/src/types.rs:70` | numerics | BT.709 primaries, transfer and matrix. |
| `default_arrow_art` | 0 | `crates/vector-surface-brush/src/art.rs:62` | - |  |
| `default_blend` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:288` | - |  |
| `impl Device::default_config` | 2 | `crates/audio-device-cpal/src/host/alsa/mod.rs:455` | - |  |
| `impl Device::default_config` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:332` | - |  |
| `impl Device::default_config` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:95` | - |  |
| `default_config_err` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:221` | - |  |
| `default_config_error_from_os_status` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:336` | - |  |
| `default_device` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:917` | - |  |
| `default_dot_art` | 0 | `crates/vector-surface-brush/src/scatter.rs:39` | - |  |
| `impl DepKind::default_ext` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:36` | - |  |
| `impl Device::default_format` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:492` | - |  |
| `impl BehaviorMacro::default_hooks` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:105` | - | Returns the baseline `SubElementHooks` preset for this whole-model macro before any |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:596` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:614` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:632` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:650` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:674` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:692` | - | The default host for the current compilation target platform. |
| `default_host` | 0 | `crates/audio-device-cpal/src/platform/mod.rs:721` | - | The default host for the current compilation target platform. |
| `default_input_asbd` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:414` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:80` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:493` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:132` | - | Returns the default input config |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:77` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:124` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:163` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:105` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:415` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:99` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:159` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:164` | - | Returns the default input config |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:63` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:302` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:73` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:525` | - |  |
| `impl Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:117` | - |  |
| `impl DeviceTrait for Device::default_input_config` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:161` | - |  |
| `trait DeviceTrait::default_input_config` | 1 | `crates/audio-device-cpal/src/traits.rs:119` | - | The default input stream format for the device. |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/alsa/enumerate.rs:50` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:50` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:45` | - |  |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:36` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:67` | - |  |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:91` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:75` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:129` | - |  |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:389` | - |  |
| `impl Device::default_input_device` | 3 | `crates/audio-device-cpal/src/host/jack/device.rs:81` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/jack/mod.rs:115` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:116` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:77` | - |  |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/wasapi/device.rs:928` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:43` | - |  |
| `impl HostTrait for Host::default_input_device` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:67` | - |  |
| `default_input_device` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:480` | - |  |
| `trait HostTrait::default_input_device` | 1 | `crates/audio-device-cpal/src/traits.rs:50` | - | The default input audio device on the system. |
| `default_map_is_120_4_4` | 0 | `crates/audio-timebase/src/tempo.rs:372` | - |  |
| `default_neighborhood_is_legacy_for_existing_journal_callers` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:48` | - |  |
| `default_output_asbd` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:406` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:84` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:497` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:153` | - | Returns the default output config |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:81` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:131` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:168` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:109` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:419` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:103` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:163` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:171` | - | Returns the default output config |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:68` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:313` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:77` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:533` | - |  |
| `impl Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:123` | - |  |
| `impl DeviceTrait for Device::default_output_config` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:166` | - |  |
| `trait DeviceTrait::default_output_config` | 1 | `crates/audio-device-cpal/src/traits.rs:122` | - | The default output stream format for the device. |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/alsa/enumerate.rs:58` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:54` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:50` | - |  |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:41` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:71` | - |  |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:121` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:79` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:133` | - |  |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:394` | - |  |
| `impl Device::default_output_device` | 3 | `crates/audio-device-cpal/src/host/jack/device.rs:67` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/jack/mod.rs:124` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:120` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:81` | - |  |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/wasapi/device.rs:932` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:47` | - |  |
| `impl HostTrait for Host::default_output_device` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:71` | - |  |
| `default_output_device` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:486` | - |  |
| `trait HostTrait::default_output_device` | 1 | `crates/audio-device-cpal/src/traits.rs:55` | - | The default output audio device on the system. |
| `default_prompt_max` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:313` | - |  |
| `default_sr` | 0 | `crates/kinetics-motion-compositor/src/lottie/schema.rs:37` | - |  |
| `default_stitch_tile` | 0 | `crates/vector-surface-brush/src/pattern.rs:55` | - |  |
| `default_supported_configs` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:107` | - |  |
| `default_threads` | 0 | `crates/video-h264-codec/src/decoder.rs:111` | - | The worker threads [`Decoder::new`] uses: the available cores (at most 16) with the `threads` |
| `impl PbrChannel::default_value` | 1 | `crates/surface-pbr-studio/src/material.rs:61` | - |  |
| `impl RuleKind::defaults` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:66` | - |  |
| `deferred_reuse` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:178` | - |  |
| `deg` | 1 | `crates/drafting-geometry/src/lib.rs:63` | - |  |
| `impl Grid::degree` | 2 | `crates/vector-radiance-trace/src/centerline.rs:185` | - |  |
| `deinterleave_hadamard` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:151` | - |  |
| `dejag` | 1 | `crates/vector-radiance-trace/src/fit.rs:85` | 2d | Replace pixel-staircase jogs by their midpoints. |
| `trait GlResource::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:22` | - |  |
| `impl GlResource for WebGlTexture::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:92` | - |  |
| `impl GlResource for WebGlBuffer::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:104` | - |  |
| `impl GlResource for WebGlFramebuffer::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:116` | - |  |
| `impl GlResource for WebGlRenderbuffer::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:129` | - |  |
| `impl GlResource for WebGlSync::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:142` | - |  |
| `impl GlResource for WebGlProgram::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:154` | - |  |
| `impl GlResource for WebGlVertexShader::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:167` | - |  |
| `impl GlResource for WebGlFragmentShader::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:180` | - |  |
| `impl GlResource for WebGlVertexArrayObject::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:192` | - |  |
| `impl Project::delete_task` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:576` | - |  |
| `impl GamutCheck::delta_e` | 2 | `crates/color-transforms/src/icc/gamut.rs:57` | - |  |
| `delta_e76` | 2 | `crates/color-transforms/src/icc/math.rs:152` | - | CIE ΔE*ab (1976) Euclidean color difference. |
| `demosaic_ahd` | 4 | `crates/radiance-camera-raw/src/demosaic.rs:192` | - | Adaptive Homogeneity-Directed (AHD) demosaicing for Bayer CFA. |
| `demosaic_bilinear` | 4 | `crates/radiance-camera-raw/src/demosaic.rs:117` | - | Bilinear demosaicing for Bayer sensor data. |
| `demosaic_raw` | 7 | `crates/radiance-camera-raw/src/demosaic.rs:369` | numerics | Demosaics a raw 16-bit sensor buffer into linear RGB f32. |
| `demosaic_rcd` | 4 | `crates/radiance-camera-raw/src/demosaic.rs:288` | - | Ratio-based Color Difference (RCD) demosaicing for Bayer CFA. |
| `demosaic_xtrans` | 3 | `crates/radiance-camera-raw/src/demosaic.rs:293` | graph | Edge-weighted color-difference demosaicing for Fujifilm 6×6 X-Trans CFA patterns. |
| `demux_matroska` | 1 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:35` | - |  |
| `demux_mxf` | 1 | `crates/chronicle-broadcast-containers/src/mxf/demux.rs:22` | - |  |
| `denoise` | 4 | `crates/vector-radiance-trace/src/quantize.rs:430` | - | Merge 4-connected same-label components smaller than `min_area` pixels into their most common |
| `impl MeterPoint::denominator` | 1 | `crates/audio-timebase/src/tempo.rs:75` | - |  |
| `denormalise_bands` | 8 | `crates/audio-opus-codec/src/celt/bands.rs:879` | - | Multiplies the normalised shapes by the band energies (`denormalise_bands`). |
| `denormalize_c` | 3 | `crates/radiance-color-grading/src/matrix.rs:208` | - | Normalized Cb/Cr float (-0.5..0.5 nominal) -> integer sample clamped to range. |
| `denormalize_y` | 3 | `crates/radiance-color-grading/src/matrix.rs:192` | - | Normalized Y float (0..1 nominal) -> integer sample clamped to range. |
| `densify_axis_cubic` | 2 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:619` | - |  |
| `densify_loft_rings_cubic` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:945` | numerics | Cubic Catmull-Rom densify of loft stations. Linear/ruled skinning is applied after. |
| `densify_loft_rings_nurbs` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:979` | - | NURBS densify of loft stations via `parametric_cad::nurbs_eval` (unit weights). |
| `densify_print` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:118` | - |  |
| `denv` | 4 | `crates/discourse-academic-tex/src/catalog.rs:88` | - |  |
| `dep` | 3 | `crates/discourse-academic-tex/src/texparse/graph.rs:221` | - |  |
| `impl Graph::dependents` | 2 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:60` | - |  |
| `depth_progression` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:1012` | - |  |
| `dequant_coef` | 2 | `crates/audio-aac-codec/src/tns.rs:119` | - | Dequantise one reflection coefficient. |
| `impl Deref for TypedBufGuard<'_, T>::deref` | 1 | `crates/vector-compute-shaders/src/cpu.rs:77` | - |  |
| `impl Deref for TypedBufGuardMut<'_, T>::deref` | 1 | `crates/vector-compute-shaders/src/cpu.rs:88` | - |  |
| `impl Deref for Resource<T>::deref` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:58` | - |  |
| `impl Deref for WebGlVertexShader::deref` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:69` | - |  |
| `impl Deref for WebGlFragmentShader::deref` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:80` | - |  |
| `impl DerefMut for TypedBufGuardMut<'_, T>::deref_mut` | 1 | `crates/vector-compute-shaders/src/cpu.rs:97` | - |  |
| `trait Block::derivatives` | 5 | `crates/kinetics-block-simulation/src/block.rs:61` | - |  |
| `impl Block for Integrator::derivatives` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:45` | - |  |
| `impl Block for PidController::derivatives` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:180` | - |  |
| `impl Block for TransferFcn::derivatives` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:262` | - |  |
| `impl SliceDecoder<'a>::derive_direct` | 2 | `crates/video-h264-codec/src/slicedec.rs:597` | - | Derive direct-mode motion for the 8x8 blocks in `b8_mask` of the current MB. |
| `impl SliceDecoder<'a>::derive_inter_motion` | 1 | `crates/video-h264-codec/src/slicedec.rs:664` | - | Derive mvs of the current (non-skip) inter MB from `self.cur` (ref_idx, mvd). |
| `impl SliceDecoder<'a>::derive_intra_modes` | 2 | `crates/video-h264-codec/src/slicedec.rs:883` | - | Resolve Intra4x4PredMode / Intra8x8PredMode for the current MB from `self.cur.rem_mode`. |
| `impl SliceDecoder<'a>::derive_p_skip` | 1 | `crates/video-h264-codec/src/slicedec.rs:652` | - | P_Skip motion (8.4.1.1). |
| `desaturate_into_gamut` | 2 | `crates/color-transforms/src/transform.rs:273` | - | Bring a colour with negative components into gamut by desaturating towards its luminance |
| `impl wasm_bindgen::describe::WasmDescribe for BufferSize::describe` | 0 | `crates/audio-device-cpal/src/lib.rs:241` | - |  |
| `impl Builtin::description` | 1 | `crates/color-transforms/src/icc/builtin.rs:62` | - |  |
| `description` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:337` | - |  |
| `impl VibeNamespaceProvider for HypermediaOntologyProvider::descriptor` | 1 | `crates/asset-production-vibe/src/ontology/vibe_descriptors.rs:259` | - |  |
| `descriptor` | 1 | `crates/media-isobmff/src/codec.rs:1121` | - |  |
| `impl Deserialize<'de> for WirePoint::deserialize` | 1 | `crates/vector-locus-geometry/src/path.rs:80` | - |  |
| `impl BiquadCoeffs::design` | 5 | `crates/acoustics-daw-session/src/dsp/biquad.rs:45` | - | Designs an audio filter from frequency, Q factor, gain (dB), and sample rate. |
| `design_fractional` | 5 | `crates/audio-opus-codec/src/silk/resampler.rs:34` | - | Least-squares fractional-delay interpolator: taps at offsets `-past+1 ..= future` (in input |
| `impl VelloRendererOptions::desired_maximum_frame_latency` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:140` | - |  |
| `impl VelloHybridRendererOptions::desired_maximum_frame_latency` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:130` | - |  |
| `impl WebGlRenderer::destroy_image` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:784` | - | Destroy an image from the cache and clear the allocated slot in the atlas. |
| `impl Renderer::destroy_image` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:594` | - | Destroy an image from the cache and clear the allocated slot in the atlas. |
| `destructive_empty` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:703` | - |  |
| `det` | 1 | `crates/color-transforms/src/icc/math.rs:36` | - |  |
| `impl Matrix::det` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:254` | - |  |
| `impl Mat3::det` | 1 | `crates/drafting-geometry/src/mat.rs:73` | - |  |
| `impl Homography::det` | 1 | `crates/vector-locus-geometry/src/projective.rs:50` | - |  |
| `det3x3` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:233` | - |  |
| `detect_camelot_key` | 1 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:71` | - | Krumhansl-Schmuckler key detection from a 12-element chroma profile. |
| `detect_caption_format` | 1 | `crates/chronicle-timeline-interchange/src/captions/mod.rs:44` | - |  |
| `detect_pitch_yin` | 5 | `crates/acoustics-daw-session/src/dsp/pitch_detect.rs:16` | - | Computes the fundamental frequency $f_0$ of an audio frame using the YIN algorithm with parabolic interpolation. |
| `detect_shi_tomasi_corners` | 4 | `crates/kinetics-motion-compositor/src/track/klt.rs:54` | - | Detect good features to track using the Shi-Tomasi minimum eigenvalue criterion. |
| `detects_tampering` | 0 | `crates/chronicle-fiduciary-ledger/src/audit.rs:213` | - |  |
| `impl DMat2::determinant` | 1 | `crates/kinetics-garment-draping/src/math.rs:214` | - |  |
| `deterministic_replay_stroke_journal_and_byte_budget_fail_closed` | 0 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:435` | - |  |
| `deterministic_replay_stroke_journal_and_byte_budget_fail_closed` | 0 | `crates/terrain-brushes/src/lib.rs:601` | - |  |
| `impl AssetHost<'_>::develop_camera_raw_hdr` | 2 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:8` | 3d | 128: `hypermedia.radiance.develop_camera_raw_hdr` |
| `develop_image` | 4 | `crates/radiance-camera-raw/src/develop.rs:188` | numerics | Applies non-destructive develop pipeline to linear RGB image data. |
| `impl AssetHost<'_>::develop_photo` | 2 | `crates/asset-production-vibe/src/radiance_photo/develop.rs:9` | - |  |
| `impl BufferRenderer::device` | 1 | `crates/gpu-device-context/src/buffer_renderer.rs:63` | - |  |
| `impl SurfaceRenderer<'s>::device` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:208` | - |  |
| `impl Pixels<'win>::device` | 1 | `crates/pixel-buffer-output/src/lib.rs:740` | - | Provides access to the internal [`wgpu::Device`]. |
| `impl RenderContext::device` | 2 | `crates/vector-compute-renderer/src/util.rs:145` | - | Finds or creates a compatible device handle id. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::device_descriptor` | 2 | `crates/pixel-buffer-output/src/builder.rs:82` | - | Add options for requesting a [`wgpu::Device`]. |
| `device_limits` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:227` | - |  |
| `trait ErrDeviceNotAvailable::device_not_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:61` | - |  |
| `impl ErrDeviceNotAvailable for crate::BuildStreamError::device_not_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:65` | - |  |
| `impl ErrDeviceNotAvailable for crate::SupportedStreamConfigsError::device_not_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:71` | - |  |
| `impl ErrDeviceNotAvailable for crate::DefaultStreamConfigError::device_not_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:77` | - |  |
| `impl ErrDeviceNotAvailable for crate::StreamError::device_not_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:83` | - |  |
| `device_supported_configs` | 2 | `crates/audio-device-cpal/src/host/oboe/mod.rs:142` | - |  |
| `impl Profile::device_to_pcs` | 2 | `crates/color-transforms/src/icc/profile.rs:226` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:46` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:41` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:63` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:71` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:125` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/jack/mod.rs:111` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:112` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:64` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:39` | - |  |
| `impl HostTrait for Host::devices` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:63` | - |  |
| `trait HostTrait::devices` | 1 | `crates/audio-device-cpal/src/traits.rs:45` | - | An iterator yielding all [`Device`](DeviceTrait)s currently available to the host on the system. |
| `dfs` | 5 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:124` | graph |  |
| `diag` | 1 | `crates/vector-locus-pathops/src/effects/distort.rs:10` | 2d |  |
| `diagonal_pixels_stay_separate` | 0 | `crates/vector-radiance-trace/src/tests.rs:143` | - |  |
| `impl MorphologyKernel::diamond` | 1 | `crates/raster-selection-morphology/src/grayscale.rs:74` | - | Diamond structuring element with Manhattan radius `\|dx\| + \|dy\| <= radius`. |
| `impl MaterialPreset::dielectric` | 4 | `crates/surface-pbr-studio/src/material.rs:103` | - |  |
| `diff` | 2 | `crates/media-test-support/src/golden.rs:115` | - | Compare two images of equal size. |
| `diff_image` | 2 | `crates/media-test-support/src/golden.rs:181` | - | Difference image: \|a − b\| per channel ×8, opaque. |
| `diff_metrics` | 0 | `crates/media-test-support/src/golden.rs:301` | - |  |
| `difference_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:347` | - | HDRI Difference: `\|backdrop - source\|`. |
| `difference_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:391` | - |  |
| `dijkstra_shortest_path` | 3 | `crates/geodesy-spatial-analysis/src/network_topology.rs:111` | graph | Computes the shortest route between start_node and end_node using Dijkstra's algorithm. |
| `impl ElementType::dimension` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:34` | - |  |
| `dimensions` | 1 | `crates/asset-production-vibe/src/graphics.rs:70` | - |  |
| `dimensions_and_initial_height_refuse_before_allocation_or_mutation` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:80` | - |  |
| `impl PageSize::dimensions_mm` | 1 | `crates/geodesy-spatial-analysis/src/cartography.rs:276` | - |  |
| `impl Line::dir` | 1 | `crates/drafting-geometry/src/curve.rs:19` | 2d |  |
| `dir_n` | 2 | `crates/locus-parametric-solver/src/solver/residual.rs:105` | 2d |  |
| `direct` | 1 | `crates/audio-aac-codec/src/mdct.rs:217` | - |  |
| `directed_edge_slot` | 3 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:37` | graph |  |
| `directive` | 1 | `crates/discourse-academic-tex/src/texparse/magic.rs:80` | - |  |
| `disc` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:535` | - |  |
| `disc` | 4 | `crates/vector-radiance-trace/src/tests.rs:14` | - |  |
| `impl CowTerrainTileGrid::discard_local_history` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:274` | - | Deliberately discard local undo/redo history, retaining current tile data and Arcs. |
| `impl CowTerrainTileGrid::discard_local_history` | 1 | `crates/terrain-brushes/src/lib.rs:459` | - | Deliberately discard local undo/redo history, retaining current tile data and Arcs. |
| `impl AssetHost<'_>::discourse_op` | 3 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:261` | - | Categorical operations dispatch for the `discourse` category morphisms. |
| `impl MorphologyKernel::disk` | 1 | `crates/raster-selection-morphology/src/grayscale.rs:47` | - | Circular disk structuring element of given floating-point radius. |
| `dispatch` | 3 | `crates/chronicle-tabular-engine/src/functions/mod.rs:14` | - |  |
| `impl Recording::dispatch` | 4 | `crates/vector-compute-renderer/src/recording.rs:159` | 3d |  |
| `impl Decoder::dispatch` | 4 | `crates/video-h264-codec/src/decoder.rs:483` | - |  |
| `impl Recording::dispatch_indirect` | 5 | `crates/vector-compute-renderer/src/recording.rs:173` | 3d | Do an indirect dispatch. |
| `impl OutputTransform::display` | 1 | `crates/color-transforms/src/transform.rs:299` | numerics | For the monitors: SDR BT.709 linear (encode with the sRGB curve as usual). |
| `impl MovieInfo::display_aspect` | 1 | `crates/video-h264-preview/src/movie.rs:63` | - |  |
| `impl Catalog::display_name` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:251` | - |  |
| `display_name` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:262` | - |  |
| `display_name_and_creator` | 0 | `crates/multimodal-synthesis-schema/src/catalog.rs:632` | - |  |
| `display_p3` | 0 | `crates/color-transforms/src/icc/builtin.rs:87` | - |  |
| `impl Track::display_rotation` | 1 | `crates/media-isobmff/src/demux.rs:140` | numerics | Clockwise quarter turns (0–3) the `tkhd` matrix applies for display, or None when it is not |
| `impl LineSeg::dist` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:75` | - |  |
| `impl Line::dist` | 2 | `crates/drafting-geometry/src/curve.rs:42` | 2d |  |
| `impl Vec2::dist` | 2 | `crates/drafting-geometry/src/vec.rs:40` | 2d |  |
| `dist2` | 2 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:10` | - |  |
| `dist2` | 2 | `crates/vector-radiance-trace/src/quantize.rs:212` | - |  |
| `dist_2d` | 2 | `crates/manifold-bim-csg/src/parametric.rs:16` | - |  |
| `impl LineSeg::dist_sq` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:57` | - |  |
| `impl Point2D::dist_sq` | 2 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:18` | - |  |
| `dist_to_segment` | 3 | `crates/vector-locus-geometry/src/recognize.rs:41` | - |  |
| `impl DVec2::distance` | 2 | `crates/kinetics-garment-draping/src/math.rs:38` | - |  |
| `impl Vec2::distance` | 2 | `crates/surface-pbr-studio/src/math.rs:39` | - |  |
| `impl Vec3::distance` | 2 | `crates/surface-pbr-studio/src/math.rs:139` | - |  |
| `impl Vec2::distance` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:97` | - |  |
| `distance` | 3 | `crates/vector-radiance-trace/src/centerline.rs:92` | - |  |
| `distance_3d` | 2 | `crates/kinetics-motion-compositor/src/keyframe/spatial.rs:15` | - | Distance between two 3D points. |
| `impl EarthModel::distance_km` | 3 | `crates/drafting-geometry/src/geodesic_ruler.rs:30` | - |  |
| `impl GeodesicRuler::distance_km` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:282` | - |  |
| `distance_to_outline` | 2 | `crates/vector-locus-geometry/src/hit.rs:57` | - | Distance from `p` to the nearest point on the path outline. |
| `distance_transform_and_skeletonization_work` | 0 | `crates/raster-selection-morphology/src/tests.rs:243` | - |  |
| `distance_transform_f32` | 3 | `crates/raster-selection-morphology/src/grayscale.rs:270` | - | Euclidean distance transform on a thresholded raster mask (`>= 0.5` inside). |
| `distance_transform_release_comparison` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:116` | - |  |
| `distant_light_direction` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:918` | - | Compute the direction vector towards a distant light source from its azimuth and elevation. |
| `distort_rgba32f` | 8 | `crates/raster-spatial-operators/src/distort.rs:207` | 2d | Distort an RGBA32F buffer. |
| `distort_rgba8` | 8 | `crates/raster-spatial-operators/src/distort.rs:169` | 2d | Distort an RGBA8 buffer. |
| `distribute` | 3 | `crates/layout-box-autolayout/src/flex.rs:44` | - | Distributes free main-axis space among open fill items like CSS flexbox. |
| `distribute` | 2 | `crates/vector-glyph-shaping/src/thread.rs:49` | - | Distribute a sequence of story runs across linked text frames in thread order. |
| `distribute_and_rejoin_story` | 0 | `crates/vector-glyph-shaping/src/thread.rs:114` | - |  |
| `distribute_flex_sizes` | 4 | `crates/asset-production-vibe/src/layout_ops/box_theme.rs:9` | - | Freeze-and-redistribute free main-axis space so min/max clamps stay exact. |
| `impl Div<T> for SampleRate::div` | 2 | `crates/audio-device-cpal/src/lib.rs:214` | - |  |
| `impl Div<f64> for Vec2::div` | 2 | `crates/drafting-geometry/src/vec.rs:132` | 2d |  |
| `impl Div<f64> for DVec2::div` | 2 | `crates/kinetics-garment-draping/src/math.rs:83` | - |  |
| `impl Div<f64> for DVec3::div` | 2 | `crates/kinetics-garment-draping/src/math.rs:197` | - |  |
| `impl Div for Jet<N>::div` | 2 | `crates/kinetics-garment-draping/src/math.rs:359` | - |  |
| `impl Div<f32> for Vec2::div` | 2 | `crates/surface-pbr-studio/src/math.rs:79` | - |  |
| `impl Div<f32> for Vec3::div` | 2 | `crates/surface-pbr-studio/src/math.rs:233` | - |  |
| `impl std::ops::Div<f32> for Vec2::div` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:52` | - |  |
| `div32_varq` | 3 | `crates/audio-opus-codec/src/silk/lpc.rs:59` | - | `silk_DIV32_varQ`: approximation of `(a32 << q_res) / b32`. |
| `div_round` | 2 | `crates/chronicle-fiduciary-ledger/src/money.rs:12` | - | Commercial rounding: divide rounding half away from zero. |
| `div_round_bankers` | 2 | `crates/chronicle-fiduciary-ledger/src/money.rs:23` | - | Banker's rounding: divide rounding half to even. |
| `divide` | 3 | `crates/manifold-bim-csg/src/csg.rs:131` | - |  |
| `divide_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:353` | - | HDRI Divide: `backdrop / source` (safeguarded against division by zero). |
| `divide_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:395` | - |  |
| `dlog_decode` | 1 | `crates/color-transforms/src/log.rs:261` | - |  |
| `dlog_decode` | 1 | `crates/radiance-color-grading/src/log.rs:286` | - |  |
| `dlog_encode` | 1 | `crates/color-transforms/src/log.rs:258` | - |  |
| `dlog_encode` | 1 | `crates/radiance-color-grading/src/log.rs:278` | - |  |
| `impl SampleEntry::dnx` | 3 | `crates/media-isobmff/src/codec.rs:1295` | - | Avid DNxHD / DNxHR entry (`AVdn`, or `AVdh` for DNxHR). |
| `impl WebGlRendererContext<'_>::do_filter_instance_pass` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3135` | - |  |
| `document_shape_is_valid_pdf` | 0 | `crates/discourse-academic-tex/src/texpdf.rs:482` | - |  |
| `doi_forms` | 0 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:148` | - |  |
| `dollar_conversions` | 0 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:131` | - |  |
| `dollar_one_wins_over_zero` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:206` | - |  |
| `dollarde` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:364` | - |  |
| `dollarfr` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:374` | - |  |
| `impl LogCurve::domain` | 1 | `crates/color-transforms/src/log.rs:51` | - |  |
| `impl Spline::domain` | 1 | `crates/drafting-geometry/src/spline.rs:168` | - |  |
| `impl LogCurve::domain` | 1 | `crates/radiance-color-grading/src/log.rs:48` | - |  |
| `domain_conversion_roundtrip` | 0 | `crates/color-transforms/src/log.rs:323` | - |  |
| `impl AssetHost<'_>::domain_extension_op` | 3 | `crates/asset-production-vibe/src/domain_extensions.rs:35` | - | Dispatches Wave 13 domain extension morphisms (`120..=143`). |
| `domain_value` | 3 | `crates/color-transforms/src/transform.rs:356` | - | Decoder-normalised value of a curve-domain value — re-exported for callers that build their |
| `impl RangeEncoder::done` | 1 | `crates/audio-opus-codec/src/range.rs:371` | - | Finalises and returns a buffer of exactly `size` bytes. |
| `dops` | 0 | `crates/audio-opus-codec/src/header.rs:173` | - |  |
| `dot` | 2 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:69` | - |  |
| `impl Vec2::dot` | 2 | `crates/drafting-geometry/src/vec.rs:27` | 2d |  |
| `impl Vec3::dot` | 2 | `crates/drafting-geometry/src/vec.rs:160` | 3d |  |
| `impl DVec2::dot` | 2 | `crates/kinetics-garment-draping/src/math.rs:23` | - |  |
| `impl DVec3::dot` | 2 | `crates/kinetics-garment-draping/src/math.rs:105` | - |  |
| `dot` | 2 | `crates/kinetics-garment-draping/src/math.rs:366` | - |  |
| `dot` | 2 | `crates/manifold-bim-csg/src/csg.rs:25` | - |  |
| `impl Vec2::dot` | 2 | `crates/surface-pbr-studio/src/math.rs:24` | - |  |
| `impl Vec3::dot` | 2 | `crates/surface-pbr-studio/src/math.rs:105` | - |  |
| `impl Vec2::dot` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:81` | - |  |
| `impl V3::dot` | 2 | `crates/vector-locus-pathops/src/three_d/revolve.rs:25` | - |  |
| `dot_prod` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:372` | - |  |
| `dot_vec` | 2 | `crates/kinetics-garment-draping/src/newton.rs:201` | - |  |
| `double_declining_balance_with_residual` | 0 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:190` | - |  |
| `impl Recording::download` | 2 | `crates/vector-compute-renderer/src/recording.rs:197` | - | Prepare a buffer for downloading. |
| `downsample` | 4 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:54` | - |  |
| `impl Decoder::draft` | 1 | `crates/video-h264-codec/src/decoder.rs:178` | - |  |
| `drag_transition_span` | 5 | `crates/timeline-editing/src/transition.rs:209` | graph | Compute a drag preview without changing a document. The host commits the |
| `drainage_network` | 4 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:279` | - | Execute full drainage analysis pipeline: priority-flood conditioning, D8 routing, accumulation, and channel carving. |
| `impl AssetHost<'_>::drape_orthotropic_garment` | 2 | `crates/asset-production-vibe/src/kinetics_fem_garment_blocks.rs:105` | - | 136: `hypermedia.kinetics.drape_orthotropic_garment` (`kinetics-garment-draping`) |
| `draw` | 4 | `crates/gpu-command-profiler/examples/demo.rs:326` | - |  |
| `impl Recording::draw` | 2 | `crates/vector-compute-renderer/src/recording.rs:189` | - | Issue a draw call |
| `impl DrawGlyphs<'a>::draw` | 3 | `crates/vector-compute-renderer/src/scene.rs:604` | - | Encodes a fill or stroke for the given sequence of glyphs and consumes the builder. |
| `impl RoundStage::draw` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:361` | - |  |
| `impl SceneCase::draw` | 3 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:30` | 2d |  |
| `draw_after_blend` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:469` | - |  |
| `impl SceneCase::draw_at` | 3 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:35` | - |  |
| `impl Scene::draw_blurred_rounded_rect` | 6 | `crates/vector-compute-renderer/src/scene.rs:244` | 2d | Draw a rounded rectangle blurred with a gaussian filter. |
| `impl Scene::draw_blurred_rounded_rect_in` | 7 | `crates/vector-compute-renderer/src/scene.rs:270` | 2d | Draw a rounded rectangle blurred with a gaussian filter in `shape`. |
| `impl PaintScene for VelloCpuScenePainter::draw_box_shadow` | 6 | `crates/vector-cpu-render-adapter/src/scene.rs:205` | 2d |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::draw_box_shadow` | 6 | `crates/vector-gpu-render-adapter/src/scene.rs:175` | 2d |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::draw_box_shadow` | 6 | `crates/vector-hybrid-render-adapter/src/scene.rs:327` | 2d |  |
| `impl PaintScene for WebGlScenePainter<'_>::draw_box_shadow` | 6 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:239` | 2d |  |
| `impl PaintScene for SkiaScenePainter<'_>::draw_box_shadow` | 6 | `crates/vector-skia-render-adapter/src/scene.rs:562` | 2d |  |
| `draw_cap` | 7 | `crates/vector-compute-shaders/shader/flatten.wgsl:523` | - |  |
| `draw_cap` | 10 | `crates/vector-compute-shaders/src/cpu/flatten.rs:386` | 2d |  |
| `draw_clear` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:953` | - |  |
| `draw_clipped` | 4 | `crates/vector-cpu-render-adapter/src/scene.rs:243` | - |  |
| `impl Scene::draw_glyphs` | 2 | `crates/vector-compute-renderer/src/scene.rs:435` | - | Returns a builder for encoding a glyph run. |
| `impl PaintScene for VelloCpuScenePainter::draw_glyphs` | 12 | `crates/vector-cpu-render-adapter/src/scene.rs:152` | 2d |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::draw_glyphs` | 12 | `crates/vector-gpu-render-adapter/src/scene.rs:139` | 2d |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::draw_glyphs` | 12 | `crates/vector-hybrid-render-adapter/src/scene.rs:273` | 2d |  |
| `impl PaintScene for WebGlScenePainter<'_>::draw_glyphs` | 12 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:185` | 2d |  |
| `impl PaintScene for SkiaScenePainter<'_>::draw_glyphs` | 13 | `crates/vector-skia-render-adapter/src/scene.rs:512` | 2d |  |
| `impl Scene::draw_image` | 3 | `crates/vector-compute-renderer/src/scene.rs:423` | - | Draws an image at its natural size with the given transform. |
| `impl WebGlRendererContext<'_>::draw_instanced_quads` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2642` | - | Draw `instance_count` quad instances using the currently bound VAO. |
| `draw_join` | 8 | `crates/vector-compute-shaders/shader/flatten.wgsl:549` | - |  |
| `draw_join` | 11 | `crates/vector-compute-shaders/src/cpu/flatten.rs:427` | 2d |  |
| `draw_leaf` | 2 | `crates/vector-compute-shaders/src/cpu/draw_leaf.rs:213` | - |  |
| `draw_leaf_main` | 8 | `crates/vector-compute-shaders/src/cpu/draw_leaf.rs:14` | - |  |
| `draw_monoid_identity` | 0 | `crates/vector-compute-shaders/shader/shared/drawtag.wgsl:34` | - |  |
| `trait ScheduleTarget::draw_mut` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1059` | - |  |
| `impl ScheduleTarget for RootTarget::draw_mut` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1065` | - |  |
| `impl ScheduleTarget for LayerTextureRegion::draw_mut` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1079` | - |  |
| `draw_order` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:217` | - |  |
| `impl DrawGlyphs<'a>::draw_outline_glyphs` | 3 | `crates/vector-compute-renderer/src/scene.rs:623` | - |  |
| `impl Backend for WebGlRendererContext<'_>::draw_pass` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3255` | - |  |
| `impl Backend for RendererContext<'_>::draw_pass` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3114` | - |  |
| `trait Backend::draw_pass` | 4 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:36` | - | Execute a draw pass against the given target. |
| `impl Backend for Recorder::draw_pass` | 4 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:240` | - |  |
| `draw_reduce` | 2 | `crates/vector-compute-shaders/src/cpu/draw_reduce.rs:27` | - |  |
| `draw_reduce_main` | 4 | `crates/vector-compute-shaders/src/cpu/draw_reduce.rs:10` | - |  |
| `impl SkiaScenePainter<'_>::draw_shape` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:212` | - |  |
| `impl SkiaScenePainter<'_>::draw_shape_with_fill` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:216` | - |  |
| `trait ScheduleTarget::draw_stage` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1060` | - |  |
| `impl ScheduleTarget for RootTarget::draw_stage` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1069` | - |  |
| `impl ScheduleTarget for LayerTextureRegion::draw_stage` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1083` | - |  |
| `impl WebGlRendererContext<'_>::draw_strip_range` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2701` | 3d | Draw `count` strip instances starting at `first_instance`. |
| `impl WebGlRendererContext<'_>::draw_strips` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2659` | - | Draw `count` strip instances starting at `first_instance`, split into one draw per |
| `impl ScheduledCase::draw_view` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:226` | - |  |
| `impl Drop for TriggerSender::drop` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:164` | - |  |
| `impl Drop for TriggerReceiver::drop` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:172` | - |  |
| `impl Drop for Stream::drop` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:979` | - |  |
| `impl Drop for Stream::drop` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:600` | - |  |
| `impl Drop for AudioObjectPropertyListener::drop` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/property_listener.rs:68` | - |  |
| `impl Drop for ComInitialized::drop` | 1 | `crates/audio-device-cpal/src/host/wasapi/com.rs:44` | - |  |
| `impl Drop for WaveFormatExPtr::drop` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:130` | - |  |
| `impl Drop for IdRAII::drop` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:783` | - |  |
| `impl Drop for Stream::drop` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:177` | - |  |
| `impl Drop for StreamInner::drop` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:202` | - |  |
| `impl Drop for Stream::drop` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:454` | - |  |
| `drop` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:238` | - |  |
| `impl Drop for Scope<'_, R>::drop` | 1 | `crates/gpu-command-profiler/src/scope.rs:17` | - |  |
| `impl Drop for OwningScope<'_, R>::drop` | 1 | `crates/gpu-command-profiler/src/scope.rs:35` | - |  |
| `impl Drop for DeviceSlot<'_>::drop` | 1 | `crates/pcm-device-output/src/device.rs:44` | - |  |
| `impl Drop for DeviceOutput::drop` | 1 | `crates/pcm-device-output/src/device.rs:229` | - |  |
| `impl Drop for Render::drop` | 1 | `crates/vector-compute-renderer/src/render.rs:34` | - |  |
| `impl Drop for Resource<T>::drop` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:50` | - |  |
| `impl Drop for VulkanBackend::drop` | 1 | `crates/vector-skia-render-adapter/src/vulkan.rs:383` | - |  |
| `impl TimecodeConfig::drop_frame` | 1 | `crates/media-isobmff/src/codec.rs:705` | - |  |
| `drop_frame_labels` | 0 | `crates/audio-timebase/src/timecode.rs:312` | - |  |
| `drop_shadow_only_uses_two_layer_textures` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:772` | - |  |
| `impl SigVal::dtype` | 1 | `crates/kinetics-block-simulation/src/signal.rs:115` | - |  |
| `duration` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:462` | - |  |
| `duration_change_and_drag_preview_respect_cut_and_clip_bounds` | 0 | `crates/timeline-editing/src/transition.rs:390` | - |  |
| `impl Timeline::duration_of_step` | 2 | `crates/layout-presentation-deck/src/timeline/timeline.rs:90` | - |  |
| `impl ModelInfo::duration_range` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:167` | - |  |
| `impl StreamInstant::duration_since` | 2 | `crates/audio-device-cpal/src/lib.rs:420` | - | The amount of time elapsed from another instant to this one. |
| `dxf_reader_parses_entities_without_error` | 0 | `crates/drafting-geometry/src/tests.rs:344` | - |  |
| `each_corner_takes_its_own_radius_and_kind` | 0 | `crates/vector-locus-geometry/src/shapes.rs:290` | - |  |
| `ease_progress` | 5 | `crates/kinetics-motion-compositor/src/keyframe/bezier.rs:59` | - | Compute temporal progress in [0, 1] for a segment given normalized speeds and influences. |
| `impl Keyframe::eased` | 1 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:124` | - |  |
| `edge_filter_speed` | 0 | `crates/video-h264-codec/src/deblock.rs:483` | graph | Rough timing of the edge filters (`cargo test --release -p video-h264-codec --lib edge_filter_speed -- --ignored --nocapture`). |
| `edge_mode_to_gpu` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:86` | graph |  |
| `edt` | 3 | `crates/raster-selection-morphology/src/distance.rs:15` | 2d | Euclidean distance to the nearest `true` pixel, in row-major order. |
| `edt_scratch_bound` | 3 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:115` | - |  |
| `edt_scratch_bound` | 3 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:100` | - |  |
| `edt_scratch_bound` | 3 | `crates/raster-selection-morphology/src/lib.rs:157` | - |  |
| `eetf_properties` | 0 | `crates/color-transforms/src/transform.rs:405` | - |  |
| `effect` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:346` | - |  |
| `impl NetworkEdge::effective_cost` | 1 | `crates/geodesy-spatial-analysis/src/network_topology.rs:26` | - |  |
| `effective_edits` | 2 | `crates/media-isobmff/src/mux.rs:177` | - |  |
| `impl Scene::effective_paint_transform` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:278` | - |  |
| `impl Scene::effective_path_transform` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:273` | - |  |
| `eig_symmetric` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:535` | numerics | Jacobi eigenvalue decomposition of a symmetric matrix: V' A V = diag(lambda). |
| `eighth_note_meters` | 0 | `crates/audio-timebase/src/tempo.rs:416` | - |  |
| `el` | 3 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:89` | - |  |
| `el_float` | 3 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:100` | - |  |
| `el_str` | 3 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:104` | - |  |
| `el_uint` | 3 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:95` | - |  |
| `impl ElasticIsotropic::elasticity_matrix_3d` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:42` | numerics | 6x6 elasticity matrix (Voigt notation: [eps_xx, eps_yy, eps_zz, gamma_yz, gamma_xz, gamma_xy]). |
| `elasticity_matrix_symmetry` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:244` | numerics |  |
| `impl AssetHost<'_>::electromagnetics_geodesy_ext_op` | 3 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:18` | - | Categorical operations dispatch for electromagnetics and geodesy extensions. |
| `impl AssetHost<'_>::electromagnetics_op` | 3 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:32` | - | Categorical operations dispatch for the `electromagnetics` category. |
| `impl Matrix::elem_add` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:209` | - |  |
| `impl Matrix::elem_mul` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:231` | - |  |
| `impl Matrix::elem_sub` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:220` | - |  |
| `element_key` | 4 | `crates/chronicle-broadcast-containers/src/mxf/labels.rs:61` | - |  |
| `impl Encoder::element_overhead` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:588` | - |  |
| `ellipse` | 1 | `crates/vector-locus-geometry/src/shapes.rs:67` | 2d | Ellipse inscribed in `r` with 4 smooth anchors (left, top, right, bottom). |
| `ellipse_bounds_and_area` | 0 | `crates/vector-locus-geometry/src/shapes.rs:273` | - |  |
| `ellipse_bounds_find_analytic_extrema` | 0 | `crates/drafting-geometry/src/tests.rs:251` | - |  |
| `ellipse_circle_triangle_hexagon_line` | 0 | `crates/vector-locus-geometry/src/recognize.rs:358` | - |  |
| `ellipse_pie` | 3 | `crates/vector-locus-geometry/src/shapes.rs:86` | 2d | The ellipse inscribed in `r` cut as a pie: from `start` to `end` degrees, measured |
| `ellipse_points` | 0 | `crates/drafting-geometry/src/tests.rs:172` | - |  |
| `ellipse_rotation` | 1 | `crates/vector-locus-geometry/src/recognize.rs:194` | - |  |
| `elliptical_arc_bounds_exclude_extrema_outside_the_sweep` | 0 | `crates/drafting-geometry/src/tests.rs:265` | - |  |
| `emergency_stop_is_cascade` | 0 | `crates/discourse-academic-tex/src/review/logrules.rs:211` | - |  |
| `emf_cell_display_irradiance` | 1 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:673` | numerics | Vacuum-impedance irradiance (W/m²) used as the display-linear quantity. |
| `emit` | 3 | `crates/discourse-academic-tex/src/bibliography/import.rs:58` | - | Emit `@kind{key, fields…}` from `(name, value)` pairs. |
| `impl FilterPassBuilder<'a>::emit` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:541` | - | Emit one pass, reading from the current region and writing to the other texture parity. |
| `impl Decoder::emit` | 2 | `crates/video-h264-codec/src/decoder.rs:500` | - |  |
| `impl FilterPassBuilder<'a>::emit_blur_sequence` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:572` | - | Apply the sequences of passes that is needed to create a full Gaussian blur with |
| `impl InnerError::emit_msg` | 3 | `crates/vector-compute-shaders/src/compile/mod.rs:68` | - |  |
| `emit_submesh_primitives` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:158` | - |  |
| `emit_txbody_paragraphs` | 2 | `crates/chronicle-office-codecs/src/presentation/pptx.rs:92` | - |  |
| `empirical_variogram` | 3 | `crates/geodesy-spatial-analysis/src/kriging.rs:91` | - | Computes experimental semivariogram bins from sample points. |
| `impl XmlWriter::empty` | 3 | `crates/chronicle-timeline-interchange/src/xml.rs:108` | - |  |
| `impl RangedSlice<'a, T>::empty` | 0 | `crates/vector-hybrid-renderer/src/util.rs:174` | - |  |
| `empty_and_garbage_never_panic` | 0 | `crates/audio-opus-codec/src/range.rs:487` | - |  |
| `empty_doc_is_an_error` | 0 | `crates/discourse-academic-tex/src/texpdf.rs:497` | - |  |
| `impl ScheduledCase::empty_draw_view` | 0 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:233` | - |  |
| `empty_generated_path_is_not_recorded` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:972` | - |  |
| `empty_layer` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:692` | - |  |
| `empty_passes_are_skipped` | 0 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:413` | - |  |
| `empty_scene` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:192` | - |  |
| `emulation_prevention` | 0 | `crates/bitstream-codec/src/lib.rs:381` | - |  |
| `trait DrawTarget::enable_depth` | 1 | `crates/vector-hybrid-renderer/src/target.rs:282` | - | Whether this target supports the depth-buffer optimization. |
| `impl DrawTarget for RootTarget::enable_depth` | 1 | `crates/vector-hybrid-renderer/src/target.rs:289` | - |  |
| `impl DrawTarget for LayerTextureRegion::enable_depth` | 1 | `crates/vector-hybrid-renderer/src/target.rs:299` | - |  |
| `impl DrawPassTarget::enable_opaque` | 1 | `crates/vector-hybrid-renderer/src/target.rs:27` | - |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::enable_vsync` | 2 | `crates/pixel-buffer-output/src/builder.rs:124` | - | Enable or disable Vsync. |
| `impl Pixels<'win>::enable_vsync` | 2 | `crates/pixel-buffer-output/src/lib.rs:445` | - | Enable or disable Vsync. |
| `impl log::Log for BuildScriptLog::enabled` | 2 | `crates/vector-compute-shaders/build.rs:146` | - |  |
| `impl AngleRange::encloses` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:91` | - |  |
| `enclosing_loop` | 3 | `crates/drafting-geometry/src/region.rs:361` | 2d | The closed loop enclosing `p`, counter-clockwise, or `None`. |
| `impl AssetHost<'_>::encode` | 3 | `crates/asset-production-vibe/src/lib.rs:193` | - |  |
| `impl Encoder::encode` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:262` | - | Feed planar samples (±1.0 full scale); returns any complete access units. Missing channels are |
| `encode` | 2 | `crates/audio-aac-codec/tests/common/mod.rs:131` | - |  |
| `impl RangeEncoder::encode` | 4 | `crates/audio-opus-codec/src/range.rs:310` | - |  |
| `impl GradeSpace::encode` | 2 | `crates/color-transforms/src/grade.rs:64` | numerics | Working linear → grading signal (1 = white; HDR values above HDR White are > 1). |
| `impl LogCurve::encode` | 2 | `crates/color-transforms/src/log.rs:72` | numerics | Scene-linear → signal (curve domain). |
| `impl OutputTransform::encode` | 2 | `crates/color-transforms/src/transform.rs:329` | numerics | Working linear → encoded signal (0..1, full scale) of the target colour space. |
| `impl GradeSpace::encode` | 2 | `crates/radiance-color-grading/src/grade.rs:56` | numerics | Working linear -> grading signal. |
| `impl LogCurve::encode` | 2 | `crates/radiance-color-grading/src/log.rs:71` | numerics | Linear scene reflectance -> log signal. |
| `encode` | 1 | `crates/surface-layered-codecs/src/psd/codec.rs:46` | - | Encodes data using PackBits run-length encoding. |
| `impl PsdDocument::encode` | 1 | `crates/surface-layered-codecs/src/psd/codec.rs:436` | - | Serializes the document into PSD or PSB binary format with PackBits compression. |
| `impl PsdHeader::encode` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:161` | - |  |
| `impl Enc::encode` | 3 | `crates/video-h264-codec/src/cabac.rs:245` | - |  |
| `impl Enc::encode` | 4 | `crates/video-h264-codec/src/synth_tests.rs:248` | - |  |
| `impl Encoder::encode` | 2 | `crates/video-prores-codec/src/encode.rs:80` | - | Encode one frame. Input planes may be at any bit depth 8..=16 (10 is typical); the frame's |
| `impl AssetHost<'_>::encode_aac` | 2 | `crates/asset-production-vibe/src/acoustics_ops.rs:248` | - | AAC-LC CBR audio encoding with ADTS framing. |
| `encode_advected_strata_m64` | 5 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/m64_continuum.rs:11` | - |  |
| `encode_aiff` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:148` | - | Encodes an AudioBuffer as a standard 16-bit AIFF file. |
| `encode_alpha` | 3 | `crates/video-prores-codec/src/encode.rs:432` | - |  |
| `encode_alpha_slice` | 4 | `crates/video-prores-codec/src/encode.rs:411` | - |  |
| `impl WebGlRenderer::encode_blurred_rounded_rect_paint` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1045` | 2d |  |
| `impl Renderer::encode_blurred_rounded_rect_paint` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:844` | 2d |  |
| `encode_channel` | 3 | `crates/color-transforms/src/transform.rs:59` | numerics | Encode one channel: working-linear → decoder-normalised signal (inverse of [`decode_channel`]). |
| `encode_component` | 3 | `crates/video-prores-codec/src/encode.rs:222` | - |  |
| `encode_copy_pass` | 7 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3165` | - |  |
| `impl Scene::encode_current_paint` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:291` | - | Encode the current paint into a `Paint` that can be used for rendering. |
| `impl WebGlRenderer::encode_external_texture_paint` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:949` | - |  |
| `impl Renderer::encode_external_texture_paint` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:747` | - |  |
| `encode_f32` | 2 | `crates/media-test-support/src/wav.rs:6` | - | Encode planar `f32` channels (equal lengths) as a 32-bit float WAV file. |
| `encode_filter_pass` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3201` | - |  |
| `encode_flac` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:208` | - | Encodes an AudioBuffer as a valid lossless FLAC bitstream (subset: RFC 9639 uncompressed / verbatim subframes). |
| `impl Encoder::encode_frame` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:616` | - | `last`: final frame of the stream — a CBR stream spends (or pads away) the whole reservoir so |
| `encode_geom_into` | 2 | `crates/locus-geospatial-codecs/src/wkb.rs:223` | - |  |
| `impl WebGlRenderer::encode_gradient_paint` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:974` | - |  |
| `impl Renderer::encode_gradient_paint` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:773` | - |  |
| `encode_idr_ipcm_frame` | 4 | `crates/asset-production-vibe/src/chronicle_ops.rs:534` | - |  |
| `impl WebGlRenderer::encode_image_paint` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:923` | - |  |
| `impl Renderer::encode_image_paint` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:721` | - |  |
| `encode_mtc_quarter_frame` | 2 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:242` | - | Encodes an MTC quarter-frame MIDI message (0xF1, data_byte). |
| `encode_png` | 1 | `crates/media-test-support/src/golden.rs:40` | - | Encode as PNG (RGB when fully opaque, else RGBA), maximum compression. |
| `encode_prores_mov` | 3 | `crates/video-prores-codec/src/mov.rs:27` | - | Encode a sequence of same-sized frames into a QuickTime `.mov` container at the given frame rate. |
| `impl AssetHost<'_>::encode_psd` | 2 | `crates/asset-production-vibe/src/radiance_photo/psd_codec.rs:8` | - |  |
| `encode_rgba_png` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:480` | - |  |
| `impl AssetHost<'_>::encode_rgba_png_handle` | 3 | `crates/asset-production-vibe/src/graphics_pack.rs:91` | - |  |
| `encode_timestamp` | 2 | `crates/chronicle-broadcast-containers/src/mpegts/pes.rs:24` | - |  |
| `encode_wav` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:34` | - | Encodes an AudioBuffer as a 16-bit PCM standard WAV file. |
| `encode_wkb` | 1 | `crates/locus-geospatial-codecs/src/wkb.rs:217` | - | Encodes a geometry as ISO WKB little-endian bytes. |
| `encoded` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:853` | - |  |
| `impl AssetWorkspace::encoded` | 2 | `crates/asset-production-vibe/src/lib.rs:134` | - |  |
| `encoded_bytes` | 2 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:901` | - |  |
| `encoded_bytes` | 3 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:1124` | - |  |
| `encoded_bytes` | 2 | `crates/asset-production-vibe/src/geodesy_ops.rs:718` | - |  |
| `encoder_decoder_filters_are_inverse` | 0 | `crates/audio-aac-codec/src/tns.rs:293` | - |  |
| `encoder_handles_hostile_input` | 0 | `crates/audio-aac-codec/tests/robustness.rs:96` | - |  |
| `impl Scene::encoding` | 1 | `crates/vector-compute-renderer/src/scene.rs:74` | - | Returns the underlying raw encoding. |
| `impl Scene::encoding_mut` | 1 | `crates/vector-compute-renderer/src/scene.rs:81` | - | Returns a mutable reference to the underlying raw encoding. |
| `impl Segment::end` | 1 | `crates/drafting-geometry/src/curve.rs:325` | 2d |  |
| `impl StairRun::end` | 1 | `crates/manifold-bim-csg/src/parametric.rs:172` | - |  |
| `impl BoxBuf::end` | 2 | `crates/media-isobmff/src/bytes.rs:270` | - |  |
| `impl TimeRange::end` | 1 | `crates/timeline-editing/src/timebase.rs:111` | - |  |
| `impl GpuProfiler::end_frame` | 1 | `crates/gpu-command-profiler/src/profiler.rs:394` | - | Marks the end of a frame. |
| `end_frame_unclosed_query` | 0 | `crates/gpu-command-profiler/tests/src/errors.rs:25` | - |  |
| `end_frame_unresolved_query` | 0 | `crates/gpu-command-profiler/tests/src/errors.rs:55` | - |  |
| `impl WhitegumMeshBuilder::end_layer` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:61` | - |  |
| `end_mismatch_replaces_wrong_env_name` | 0 | `crates/discourse-academic-tex/src/review/ai_repair.rs:271` | - |  |
| `impl GeodesicRuler::end_move` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:201` | - |  |
| `impl Arc::end_point` | 1 | `crates/drafting-geometry/src/curve.rs:74` | 2d |  |
| `end_point` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:410` | - |  |
| `end_pos` | 1 | `crates/discourse-academic-tex/src/texparse/structure.rs:287` | - |  |
| `impl GpuProfiler::end_query` | 3 | `crates/gpu-command-profiler/src/profiler.rs:284` | - | Ends passed query. |
| `impl ManualOwningScope<'_, R>::end_query` | 1 | `crates/gpu-command-profiler/src/scope.rs:58` | - | Ends the scope allowing the extraction of the owned [`ProfilerCommandRecorder`]. |
| `end_tangent` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:29` | 2d | Direction the curve arrives at `p3` (forward travel direction). |
| `end_tangent` | 1 | `crates/vector-radiance-trace/src/fit.rs:414` | 2d |  |
| `ends_curves_smooth_and_straight_anchors_are_no_corners` | 0 | `crates/vector-locus-geometry/src/corners.rs:237` | - |  |
| `enforces_period_lock_date` | 0 | `crates/chronicle-fiduciary-ledger/src/journal.rs:256` | - |  |
| `impl Magic::engine_hint` | 1 | `crates/discourse-academic-tex/src/texparse/magic.rs:32` | - | Normalised build engine hinted by `program`/`ts-program`, falling back to `% !BIB program`. |
| `impl Patterns::english_standard` | 0 | `crates/layout-paged-typesetter/src/hyphenation/liang.rs:56` | - | Standard English patterns compiled for common typography. |
| `impl DeviceOutput::enqueue` | 4 | `crates/pcm-device-output/src/device.rs:171` | - |  |
| `impl Queue::enqueue` | 4 | `crates/pcm-device-output/src/queue.rs:80` | - |  |
| `enrich_diagnostics` | 2 | `crates/discourse-academic-tex/src/review/logrules.rs:152` | - | Enrich diagnostic items in place. |
| `impl SurfaceRenderer<'s>::ensure_current_surface_texture` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:275` | - |  |
| `impl Rounds::ensure_exists` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:84` | - |  |
| `impl Device::ensure_future_audio_client` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:343` | - | Ensures that `future_audio_client` contains a `Some` and returns a locked mutex to it. |
| `impl Resources::ensure_glyph_resources` | 1 | `crates/vector-hybrid-renderer/src/text.rs:55` | - |  |
| `impl Scheduler<'a, 'p>::ensure_layer_target` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:800` | - | Lazily allocate a target for an open layer and return its scheduling state. |
| `impl FilterPassBuilder<'a>::ensure_result_in_original` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:600` | - |  |
| `impl CowTerrainTileGrid::ensure_tile` | 4 | `crates/terrain-brushes/adaptation/grid-impl.rs:49` | - |  |
| `impl CowTerrainTileGrid::ensure_tile` | 4 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:170` | - |  |
| `impl CowTerrainTileGrid::ensure_tile` | 4 | `crates/terrain-brushes/src/lib.rs:230` | - |  |
| `impl Parser<'_>::enter` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:70` | - |  |
| `entry` | 1 | `crates/media-isobmff/tests/roundtrip.rs:23` | - |  |
| `env` | 4 | `crates/discourse-academic-tex/src/catalog.rs:82` | - |  |
| `env_var` | 1 | `crates/media-test-support/src/oracle.rs:67` | - |  |
| `environments` | 0 | `crates/discourse-academic-tex/src/catalog.rs:413` | - | The catalog entries that are environments (`\begin{name}`…`\end{name}`). |
| `eof` | 0 | `crates/media-isobmff/src/source.rs:20` | - |  |
| `eof_anchor` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:227` | - |  |
| `impl PartialEq for Device::eq` | 2 | `crates/audio-device-cpal/src/host/asio/device.rs:39` | - |  |
| `impl PartialEq for Device::eq` | 2 | `crates/audio-device-cpal/src/host/jack/device.rs:256` | - |  |
| `impl PartialEq for Device::eq` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:771` | - |  |
| `impl PartialEq for CreationError::eq` | 2 | `crates/gpu-command-profiler/src/errors.rs:23` | - |  |
| `impl PartialEq<[u8; 4]> for FourCc::eq` | 2 | `crates/media-isobmff/src/bytes.rs:47` | - |  |
| `impl PartialEq<&[u8; 4]> for FourCc::eq` | 2 | `crates/media-isobmff/src/bytes.rs:52` | - |  |
| `impl PartialEq<&&[u8; 4]> for FourCc::eq` | 2 | `crates/media-isobmff/src/bytes.rs:57` | - |  |
| `impl PartialEq for FloodItem::eq` | 2 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:40` | - |  |
| `equilibrium_d3q19` | 2 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:45` | economics | Equilibrium distribution for incompressible Navier-Stokes. |
| `impl Equivalent<NormalizedTypefaceCacheKey> for NormalizedTypefaceCacheKeyBorrowed<'_>::equivalent` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:115` | - |  |
| `impl Equivalent<FontCacheKey> for FontCacheKeyBorrowed<'_>::equivalent` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:164` | - |  |
| `erf` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:17` | - | Error function erf(x) via Chebyshev approximation |
| `erf7` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:715` | - |  |
| `impl AssetHost<'_>::erode_geomorphic_heightfield` | 2 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:476` | - | 125: `hypermedia.manifold.erode_geomorphic_heightfield` |
| `impl Parser::err` | 2 | `crates/chronicle-tabular-engine/src/formula/parser.rs:65` | - |  |
| `err` | 1 | `crates/video-h264-preview/src/synth.rs:16` | - |  |
| `err_fn` | 1 | `crates/audio-device-cpal/examples/feedback.rs:172` | - |  |
| `impl Parser<'_>::error` | 2 | `crates/manifold-bim-csg/src/step.rs:142` | - |  |
| `es_params_eval` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:197` | - |  |
| `es_params_eval_th` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:160` | - |  |
| `es_params_eval_with_offset` | 3 | `crates/vector-compute-shaders/shader/flatten.wgsl:210` | - |  |
| `es_params_from_angles` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:135` | - |  |
| `es_seg_eval_with_offset` | 3 | `crates/vector-compute-shaders/shader/flatten.wgsl:221` | - |  |
| `es_seg_from_params` | 3 | `crates/vector-compute-shaders/shader/flatten.wgsl:216` | - |  |
| `escape` | 1 | `crates/asset-production-vibe/src/preview.rs:4` | - |  |
| `escape_attr` | 1 | `crates/chronicle-office-codecs/src/presentation/pptx.rs:86` | - |  |
| `escape_bits` | 1 | `crates/audio-aac-codec/src/huffman.rs:174` | - |  |
| `escape_latex` | 1 | `crates/chronicle-office-codecs/src/document/latex.rs:156` | - |  |
| `escape_pdf_str` | 1 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:153` | - | Escape ASCII characters in PDF string literals `(...)`. |
| `escape_rbsp` | 1 | `crates/bitstream-codec/src/lib.rs:267` | - | Insert emulation-prevention bytes so no `00 00 0x` (x ≤ 3) sequence appears. |
| `escape_rtf` | 2 | `crates/chronicle-office-codecs/src/document/rtf.rs:110` | - |  |
| `escape_xml` | 1 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:137` | - | Escape XML entities for OpenXML and SVG documents. |
| `escape_xml` | 1 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:13` | - |  |
| `escape_xml` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:134` | - |  |
| `escape_xml` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:8` | - |  |
| `escape_xml` | 1 | `crates/surface-pbr-studio/src/materialx.rs:60` | - |  |
| `esds_payload` | 5 | `crates/media-isobmff/src/codec.rs:1204` | - |  |
| `espc_int_approx` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:250` | - |  |
| `espc_int_approx` | 1 | `crates/vector-compute-shaders/src/cpu/euler.rs:301` | - |  |
| `espc_int_inv_approx` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:264` | - |  |
| `espc_int_inv_approx` | 1 | `crates/vector-compute-shaders/src/cpu/euler.rs:318` | - |  |
| `estimate_frequency_autocorr` | 4 | `crates/electromagnetics-rf-sensing/src/vitals.rs:120` | - |  |
| `estimate_frequency_zero_crossings` | 2 | `crates/electromagnetics-rf-sensing/src/vitals.rs:105` | - |  |
| `impl CowTerrainTileGrid::estimate_memory_bytes` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:285` | - |  |
| `impl CowTerrainTileGrid::estimate_memory_bytes` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:307` | - |  |
| `impl CowTerrainTileGrid::estimate_memory_bytes` | 1 | `crates/terrain-brushes/src/lib.rs:470` | - |  |
| `impl Evaluator<'h>::eval` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:69` | - |  |
| `impl Clut::eval` | 3 | `crates/color-transforms/src/icc/clut.rs:75` | - | Interpolates at `input` into `out`. |
| `impl Stage::eval` | 3 | `crates/color-transforms/src/icc/pipeline.rs:53` | - |  |
| `impl Pipeline::eval` | 3 | `crates/color-transforms/src/icc/pipeline.rs:145` | - |  |
| `impl Transform::eval` | 3 | `crates/color-transforms/src/icc/transform.rs:99` | - |  |
| `impl Spline::eval` | 2 | `crates/drafting-geometry/src/spline.rs:175` | 2d | Evaluate at parameter `t` (de Boor via basis functions). |
| `impl MathContext::eval` | 2 | `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:28` | - |  |
| `impl Env::eval` | 2 | `crates/locus-parametric-solver/src/solver/expr.rs:184` | - |  |
| `impl Built::eval` | 4 | `crates/locus-parametric-solver/src/solver/residual.rs:317` | - |  |
| `impl EulerParams::eval` | 2 | `crates/vector-compute-shaders/src/cpu/euler.rs:197` | 2d | Evaluate the curve at the given parameter. |
| `impl EulerSeg::eval` | 2 | `crates/vector-compute-shaders/src/cpu/euler.rs:223` | 2d |  |
| `impl Clut::eval1` | 3 | `crates/color-transforms/src/icc/clut.rs:106` | - |  |
| `impl Curve::eval32` | 2 | `crates/color-transforms/src/icc/curve.rs:84` | - | Evaluates the curve at `x` (f32). |
| `impl Curve::eval64` | 2 | `crates/color-transforms/src/icc/curve.rs:57` | - | Evaluates the curve at `x` (f64 reference path). |
| `impl Env::eval_ast` | 3 | `crates/locus-parametric-solver/src/solver/expr.rs:189` | - |  |
| `impl Evaluator<'h>::eval_binary` | 4 | `crates/chronicle-tabular-engine/src/calc/eval.rs:223` | - |  |
| `impl Workspace::eval_binop` | 4 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:210` | - |  |
| `impl Evaluator<'h>::eval_call` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:275` | - |  |
| `impl Workspace::eval_call` | 3 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:251` | - |  |
| `eval_cubic_and_deriv` | 5 | `crates/vector-compute-shaders/shader/flatten.wgsl:285` | - |  |
| `eval_cubic_and_deriv` | 5 | `crates/vector-compute-shaders/src/cpu/flatten.rs:40` | 2d | Evaluate both the point and derivative of a cubic bezier. |
| `eval_cubic_bezier` | 2 | `crates/kinetics-garment-draping/src/pattern.rs:8` | - | Cubic Bézier curve evaluator. |
| `impl Sim::eval_derivatives` | 4 | `crates/kinetics-block-simulation/src/sim.rs:118` | - |  |
| `impl Workspace::eval_expr` | 2 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:118` | - |  |
| `impl Transform::eval_fast` | 3 | `crates/color-transforms/src/icc/transform.rs:104` | - |  |
| `impl Evaluator<'h>::eval_inner` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:79` | - |  |
| `impl Curve::eval_inverse64` | 2 | `crates/color-transforms/src/icc/curve.rs:67` | - | Evaluates the inverse curve at `y`. |
| `impl Evaluator<'h>::eval_invoke` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:358` | - |  |
| `impl Evaluator<'h>::eval_lambda` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:338` | - |  |
| `eval_layer_transform` | 2 | `crates/kinetics-motion-compositor/src/lottie/eval.rs:131` | - | Evaluate the transform of a layer at frame `f`. |
| `impl Evaluator<'h>::eval_let` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:313` | - |  |
| `eval_m_code` | 1 | `crates/discourse-scientific-matlang/src/interpreter/mod.rs:12` | - |  |
| `eval_multi_prop` | 2 | `crates/kinetics-motion-compositor/src/lottie/eval.rs:77` | - | Evaluate a 2D multi-dimensional property at frame `f`. |
| `impl Clut::eval_multilinear` | 3 | `crates/color-transforms/src/icc/clut.rs:161` | - | Multilinear interpolation over all 2^n cube corners. |
| `impl MathContext::eval_node` | 2 | `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:33` | graph |  |
| `impl Sim::eval_outputs` | 3 | `crates/kinetics-block-simulation/src/sim.rs:90` | - |  |
| `impl ScatterBrush::eval_params` | 3 | `crates/vector-surface-brush/src/scatter.rs:95` | - |  |
| `impl Evaluator<'h>::eval_ref` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:121` | - |  |
| `eval_scalar_prop` | 2 | `crates/kinetics-motion-compositor/src/lottie/eval.rs:18` | - | Evaluate a scalar property at frame `f`. |
| `eval_spatial_bezier` | 5 | `crates/kinetics-motion-compositor/src/keyframe/spatial.rs:6` | - | Evaluate a 3D cubic Bézier curve at parameter `u` in [0, 1]. |
| `eval_target` | 2 | `crates/locus-parametric-solver/src/solver/conflict.rs:57` | - | Evaluates target value for dimensional constraints from environment. |
| `impl EulerParams::eval_th` | 2 | `crates/vector-compute-shaders/src/cpu/euler.rs:190` | - |  |
| `eval_tri_sd` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:137` | - |  |
| `impl Clut::eval_trilinear` | 3 | `crates/color-transforms/src/icc/clut.rs:155` | - | Trilinear interpolation (3 inputs). |
| `impl Evaluator<'h>::eval_unary` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:176` | - |  |
| `impl EulerParams::eval_with_offset` | 3 | `crates/vector-compute-shaders/src/cpu/euler.rs:210` | 2d |  |
| `impl EulerSeg::eval_with_offset` | 3 | `crates/vector-compute-shaders/src/cpu/euler.rs:234` | 2d |  |
| `impl KeyframeTrack::evaluate` | 2 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:140` | - |  |
| `impl FilterExpr::evaluate` | 2 | `crates/locus-geospatial-codecs/src/telemetry/query.rs:64` | - | Evaluates the expression against an attribute dictionary. |
| `impl Dynamic::evaluate` | 4 | `crates/raster-layer-mask-adapter/src/brush.rs:72` | - |  |
| `impl BlendIf::evaluate` | 5 | `crates/raster-layer-mask-adapter/src/masks.rs:231` | 2d | Evaluates effective Blend If multiplier for a pixel. |
| `impl AutomationCurve::evaluate_at` | 2 | `crates/acoustics-daw-session/src/mix_session/mod.rs:31` | - |  |
| `impl TextAnimator::evaluate_character` | 4 | `crates/kinetics-motion-compositor/src/typography/animator.rs:48` | - |  |
| `evaluate_knockout` | 7 | `crates/raster-layer-mask-adapter/src/masks.rs:106` | 2d, numerics | Evaluates knockout composition for a single straight-alpha RGBA pixel. |
| `impl AssetHost<'_>::evaluate_nla_channelbag` | 2 | `crates/asset-production-vibe/src/kinetics_sim/nav_nla_zones.rs:229` | - | Blender ActionChannelbag NLA Strip Blending + Godot LensTweenTable evaluation. |
| `evaluate_pdfa` | 3 | `crates/discourse-pdf-compliance/src/preflight/evaluator.rs:84` | - |  |
| `evaluate_pdfx` | 3 | `crates/discourse-pdf-compliance/src/preflight/evaluator.rs:170` | - |  |
| `evaluate_preflight` | 2 | `crates/discourse-pdf-compliance/src/preflight/evaluator.rs:64` | - |  |
| `evaluate_rule` | 2 | `crates/discourse-pdf-compliance/src/a11y/checker.rs:99` | - |  |
| `evaluate_scratch_position` | 2 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:210` | - | Evaluates turntable needle position (in beats) and mute gain at beat time $t$. |
| `evaluate_stratum` | 3 | `crates/terrain-erosion-solvers/src/fractal_terrace.rs:51` | - | Compute single-elevation terraced profile and strata color index. |
| `evaluate_text_animators` | 3 | `crates/kinetics-motion-compositor/src/typography/animator.rs:66` | - | Evaluate all characters of a string through a set of kinetic text animators. |
| `evaluate_tree_lighting` | 5 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:226` | 3d, graph |  |
| `evaluate_tween` | 2 | `crates/asset-production-vibe/src/kinetics_sim/nav_nla_zones.rs:402` | - |  |
| `impl RangeSelector::evaluate_unit` | 3 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:96` | - | Evaluate the selector weight in [-1, 1] for unit index `i` out of `total_units`. |
| `impl WigglySelector::evaluate_unit` | 3 | `crates/kinetics-motion-compositor/src/typography/wiggly.rs:32` | - | Evaluate wiggly weight for unit `i` at time `t_sec`. |
| `evaluate_warp_timeline` | 2 | `crates/acoustics-daw-session/src/elastic_warp/mod.rs:45` | numerics | Evaluates source playback time $t_{\text{src}}$ corresponding to destination time $t_{\text{dst}}$ using piecewise linear interpolation. |
| `even_child` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:448` | - |  |
| `every_entry_has_a_doc_line` | 0 | `crates/discourse-academic-tex/src/catalog.rs:467` | - |  |
| `every_env_is_reachable_via_environments` | 0 | `crates/discourse-academic-tex/src/catalog.rs:434` | - |  |
| `every_shipped_shader_passes_the_lint` | 0 | `crates/vector-hybrid-shaders/src/lib.rs:24` | 3d |  |
| `impl GradientRampCache::evict` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:136` | - | Remove multiple LRU entries and compact the LUTs vector. |
| `evm_computation` | 0 | `crates/kinetics-schedule-cpm/src/schedule.rs:848` | - |  |
| `exact` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:88` | - |  |
| `exact_undo_redo_multi_tile_restoration` | 0 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:402` | - |  |
| `exact_undo_redo_multi_tile_restoration` | 0 | `crates/terrain-brushes/src/lib.rs:568` | - |  |
| `exception_for` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:87` | - |  |
| `exclusion_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:341` | - | HDRI Exclusion: `backdrop + source - 2.0 * backdrop * source`. |
| `exclusion_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:387` | - |  |
| `exe_names` | 1 | `crates/media-test-support/src/oracle.rs:30` | - |  |
| `impl Workspace::exec` | 2 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:67` | - |  |
| `execute` | 4 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:68` | - |  |
| `impl Schedule::execute` | 5 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:84` | - |  |
| `impl Rounds::execute` | 6 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:111` | - |  |
| `impl ScheduledCase::execute` | 2 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:142` | - |  |
| `execute_calculation` | 2 | `crates/discourse-pdf-compliance/src/forms/validation.rs:106` | - |  |
| `execution_case` | 2 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:288` | - |  |
| `exercise` | 1 | `crates/media-isobmff/tests/fuzz.rs:53` | - |  |
| `exp_fn` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:86` | - |  |
| `exp_golomb5_equivalence` | 0 | `crates/video-prores-codec/src/bits.rs:266` | - |  |
| `exp_golomb_increment_rejects_overflow_and_preserves_valid_value` | 0 | `crates/video-h264-codec/src/params.rs:748` | - |  |
| `exp_golomb_known` | 0 | `crates/bitstream-codec/src/lib.rs:358` | - |  |
| `exp_rotation` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:85` | - | Inverse spreading rotation (decoder direction). |
| `exp_rotation1` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:63` | - |  |
| `expand` | 1 | `crates/discourse-academic-tex/src/review/snippets.rs:152` | - | Render `body`: `${n:default}` -> `default`, `$n` -> removed; caret lands |
| `impl Bounds2::expand` | 2 | `crates/drafting-geometry/src/bounds.rs:76` | - |  |
| `impl ArtBrush::expand` | 3 | `crates/vector-surface-brush/src/art.rs:79` | - |  |
| `impl BristleBrush::expand` | 4 | `crates/vector-surface-brush/src/bristle.rs:68` | - |  |
| `impl CalligraphicBrush::expand` | 5 | `crates/vector-surface-brush/src/calligraphic.rs:62` | - |  |
| `impl PatternBrush::expand` | 3 | `crates/vector-surface-brush/src/pattern.rs:60` | - |  |
| `impl ScatterBrush::expand` | 5 | `crates/vector-surface-brush/src/scatter.rs:52` | - |  |
| `expand_all_five_brush_types` | 0 | `crates/vector-surface-brush/src/lib.rs:138` | - |  |
| `expand_brush_stroke` | 4 | `crates/vector-surface-brush/src/lib.rs:67` | - | Expands any procedural brush stroke into vector geometric pieces. |
| `expand_contract_border` | 0 | `crates/raster-selection-morphology/src/tests.rs:19` | - |  |
| `expand_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:151` | - |  |
| `expand_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:136` | - |  |
| `expand_impl` | 4 | `crates/raster-selection-morphology/src/lib.rs:217` | - |  |
| `impl AssetHost<'_>::expand_procedural_brush` | 2 | `crates/asset-production-vibe/src/surface_pbr_brush.rs:157` | - | 127: `hypermedia.surface.expand_procedural_brush` |
| `expands_defaults_and_caret` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:199` | - |  |
| `impl Parser<'_>::expect` | 2 | `crates/manifold-bim-csg/src/step.rs:154` | - |  |
| `impl Requires::expect_time_result` | 2 | `crates/gpu-command-profiler/tests/src/mod.rs:39` | - |  |
| `expected_scope` | 3 | `crates/gpu-command-profiler/tests/src/mod.rs:56` | - |  |
| `expected_secs` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:365` | - |  |
| `impl serde::de::Visitor<'de> for V::expecting` | 2 | `crates/vector-locus-geometry/src/path.rs:84` | - |  |
| `explicit_history_discard_drops_redo_without_changing_accepted_heights_or_ids` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:261` | - |  |
| `explicit_path_wins_and_bad_path_errors` | 0 | `crates/media-test-support/src/oracle.rs:136` | - |  |
| `explicit_weighted_prediction_p_and_b_skip` | 0 | `crates/video-h264-codec/src/synth_tests.rs:523` | - |  |
| `export_abaqus_inp` | 5 | `crates/kinetics-continuum-multiphysics/src/interchange/abaqus.rs:8` | - | Export nodes and hexahedral elements to Abaqus `.inp` format. |
| `export_gmsh_22` | 2 | `crates/kinetics-continuum-multiphysics/src/interchange/gmsh.rs:8` | computational geometry | Export mesh to Gmsh MSH 2.2 ASCII format. |
| `impl AssetHost<'_>::export_lottie` | 2 | `crates/asset-production-vibe/src/kinetics_ops/lottie_export.rs:16` | - |  |
| `export_lsdyna_k` | 5 | `crates/kinetics-continuum-multiphysics/src/interchange/lsdyna.rs:8` | - | Export to LS-DYNA `.k` format. |
| `export_minimal_building` | 8 | `crates/manifold-bim-csg/src/ifc.rs:94` | - | Architectural project exporter creating standard hierarchy: Project -> Site -> Building -> Storey -> Wall. |
| `export_nastran_bdf` | 5 | `crates/kinetics-continuum-multiphysics/src/interchange/nastran.rs:8` | - | Export to 8-field small-field Nastran BDF format. |
| `export_vtu_hex` | 3 | `crates/kinetics-continuum-multiphysics/src/interchange/vtk.rs:12` | computational geometry | Export hexahedral mesh to VTK XML UnstructuredGrid (.vtu) format with nodal displacements. |
| `export_xfdf` | 1 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:11` | - | Export AcroForm field values as an XML XFDF document. |
| `impl Formula::expr` | 1 | `crates/chronicle-tabular-engine/src/calc/model.rs:69` | - |  |
| `impl Parser::expr` | 2 | `crates/chronicle-tabular-engine/src/formula/parser.rs:72` | - |  |
| `impl Parser::expression` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:175` | - |  |
| `impl Bounds::extend` | 2 | `crates/surface-pbr-studio/src/math.rs:319` | 3d |  |
| `extend_mode` | 3 | `crates/vector-compute-shaders/shader/fine.wgsl:881` | - |  |
| `extend_mode_normalized` | 2 | `crates/vector-compute-shaders/shader/fine.wgsl:867` | - |  |
| `impl LutFormat::extension` | 1 | `crates/color-transforms/src/lut.rs:180` | - |  |
| `impl Bounds::extent` | 1 | `crates/surface-pbr-studio/src/math.rs:335` | 3d |  |
| `external` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:698` | - |  |
| `external_sources` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:683` | - |  |
| `impl RendererContext<'_>::external_texture_bind_group_for_textures` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2638` | - |  |
| `impl OpaqueDraw::external_texture_runs` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:106` | - |  |
| `external_texture_slots` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:672` | - |  |
| `impl WGPUContext::extra_features` | 1 | `crates/gpu-device-context/src/lib.rs:141` | - |  |
| `impl Model::extract` | 3 | `crates/locus-parametric-solver/src/solver/model.rs:279` | - | Extract updated entity from solved parameters. |
| `extract_affinity_preview` | 2 | `crates/surface-layered-codecs/src/affinity.rs:228` | - | Inspects the embedded preview thumbnail record without full image decode. |
| `extract_attr` | 2 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:84` | - |  |
| `extract_attr` | 2 | `crates/locus-geospatial-codecs/src/formats/landxml.rs:70` | - |  |
| `extract_attribute` | 2 | `crates/surface-layered-codecs/src/xcf_ora.rs:471` | - | Helper to extract XML attribute value. |
| `extract_blocks` | 2 | `crates/kinetics-schedule-cpm/src/format.rs:472` | - |  |
| `extract_collapse_mask` | 3 | `crates/audio-opus-codec/src/celt/bands.rs:112` | - |  |
| `extract_command` | 1 | `crates/discourse-academic-tex/src/review/logrules.rs:179` | - |  |
| `extract_deps` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:57` | - | Extract every file-level dependency reference from `src`. |
| `extract_dominant_eigenvectors` | 3 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:128` | - |  |
| `extract_flow_blocks` | 2 | `crates/chronicle-office-codecs/src/document/html.rs:193` | - |  |
| `extract_inlines_from_html` | 3 | `crates/chronicle-office-codecs/src/document/html.rs:258` | - |  |
| `extract_mentions` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:304` | - |  |
| `extract_multi_val` | 2 | `crates/kinetics-motion-compositor/src/lottie/eval.rs:121` | - |  |
| `extract_point_tag` | 2 | `crates/locus-geospatial-codecs/src/formats/landxml.rs:143` | - |  |
| `extract_quoted` | 1 | `crates/discourse-academic-tex/src/review/logrules.rs:172` | - |  |
| `extract_segments` | 1 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:99` | - |  |
| `extract_segments_at_level` | 2 | `crates/geodesy-spatial-analysis/src/morphometry.rs:277` | - |  |
| `extract_tag` | 2 | `crates/kinetics-schedule-cpm/src/format.rs:464` | - |  |
| `extract_tag_value` | 2 | `crates/locus-geospatial-codecs/src/formats/kml.rs:43` | - |  |
| `extract_val` | 2 | `crates/kinetics-motion-compositor/src/lottie/eval.rs:62` | - |  |
| `extract_vital_signs` | 2 | `crates/electromagnetics-rf-sensing/src/vitals.rs:74` | - | Extracts respiration and cardiac pulse rates from a time-series of CSI amplitude / sanitized phase variations. |
| `extracts_all_dep_kinds` | 0 | `crates/discourse-academic-tex/src/texparse/graph.rs:226` | - |  |
| `extrude` | 2 | `crates/vector-locus-pathops/src/three_d/bevel.rs:60` | - | Generate 3D extruded and beveled faces from a 2D path. |
| `extrude_rectangle_generates_six_faces` | 0 | `crates/vector-locus-pathops/src/three_d/bevel.rs:148` | - |  |
| `impl GroupReader<'r, 'a>::extrusion` | 1 | `crates/drafting-geometry/src/dxf_reader.rs:169` | - |  |
| `impl Matrix::eye` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:61` | - |  |
| `impl GroupReader<'r, 'a>::f` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:160` | - |  |
| `f2dot14_to_f32` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:296` | - |  |
| `f32_to_f16` | 1 | `crates/asset-production-vibe/src/geodesy_ops.rs:279` | - |  |
| `impl Cur<'a>::f64` | 1 | `crates/media-isobmff/src/bytes.rs:120` | - |  |
| `impl BoxBuf::f64` | 2 | `crates/media-isobmff/src/bytes.rs:301` | - |  |
| `f64_field` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/vox0_metamer.rs:540` | - |  |
| `f64_field` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:1117` | - |  |
| `impl MaterialPreset::fabric` | 3 | `crates/surface-pbr-studio/src/material.rs:155` | - |  |
| `face` | 3 | `crates/profile-authoring/src/frames.rs:57` | - |  |
| `impl CholeskyFactor::factorize` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/cholesky.rs:14` | numerics | Factorize symmetric positive-definite n x n matrix in row-major slice. |
| `impl LdltFactor::factorize` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/ldlt.rs:23` | numerics | Factorize symmetric n x n matrix. |
| `impl Queue::fail` | 2 | `crates/pcm-device-output/src/queue.rs:171` | - |  |
| `failed_gap_update_stops_before_frame_synthesis` | 0 | `crates/video-h264-codec/src/dpb.rs:422` | - |  |
| `failed_play_without_reset_ack_poison_blocks_reacquisition` | 0 | `crates/pcm-device-output/src/device_slot_tests.rs:16` | - |  |
| `failures_dir` | 0 | `crates/media-test-support/src/golden.rs:176` | - | Where failing comparisons write `<name>.actual.png` / `<name>.diff.png`. |
| `family` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:408` | - |  |
| `family_rank` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:440` | - |  |
| `fast_luma_matches_reference` | 0 | `crates/video-h264-codec/src/inter.rs:479` | - |  |
| `impl Scene::fast_rect_bounds` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:514` | 2d |  |
| `fdct8x8` | 1 | `crates/chronicle-broadcast-containers/src/dnx/dct.rs:71` | - | In-place forward DCT (samples in, coefficients out). |
| `fdct8x8` | 1 | `crates/video-prores-codec/src/dct.rs:65` | - | In-place forward DCT (samples in, coefficients out; same layout as [`idct8x8`]). |
| `fdct_inverts_idct` | 0 | `crates/video-prores-codec/src/dct.rs:139` | - |  |
| `feather_mask` | 4 | `crates/raster-layer-mask-adapter/src/masks.rs:29` | - | Applies separable 2D Gaussian feathering to an 8-bit mask buffer. |
| `impl VelloRendererOptions::features` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:98` | - |  |
| `impl VelloHybridRendererOptions::features` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:95` | - |  |
| `feet_and_frames` | 0 | `crates/audio-timebase/src/timecode.rs:333` | - |  |
| `feet_frames` | 3 | `crates/audio-timebase/src/timecode.rs:294` | - | Compatibility function returning raw `(feet, frames)` tuple for 35mm film. |
| `fetch_window` | 6 | `crates/video-h264-codec/src/inter.rs:19` | - | Copy the (bw+5)x(bh+5) window whose top-left is (x-2, y-2) into `win` (stride WIN), clamping |
| `ffmpeg` | 0 | `crates/audio-aac-codec/tests/common/mod.rs:9` | - | ffmpeg (see `media_test_support::oracle`); `None` after printing `SKIPPED`. |
| `ffmpeg` | 0 | `crates/media-isobmff/tests/common/mod.rs:19` | - | ffmpeg, or `None` after printing `SKIPPED` (a failure with `FILMCRAFT_REQUIRE_ORACLES=1`). |
| `ffmpeg` | 0 | `crates/media-test-support/src/oracle.rs:82` | - | Path of ffmpeg, if available (cached for the process). |
| `ffmpeg_decode` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:155` | - | Decode with ffmpeg → planar f32 (ffmpeg's own channel order), plus stderr. |
| `ffmpeg_decode_errors` | 1 | `crates/media-isobmff/tests/common/mod.rs:175` | - | Decode a file with ffmpeg to the null muxer; returns stderr (empty on success). |
| `ffmpeg_encode` | 5 | `crates/audio-aac-codec/tests/common/mod.rs:170` | - | Encode with ffmpeg's AAC encoder to ADTS. |
| `ffmpeg_or_skip` | 1 | `crates/media-test-support/src/oracle.rs:115` | - | ffmpeg, or `None` after reporting a skip (see [`skip`]). |
| `ffprobe` | 0 | `crates/media-isobmff/tests/common/mod.rs:24` | - | ffprobe, or `None` after printing `SKIPPED` (a failure with `FILMCRAFT_REQUIRE_ORACLES=1`). |
| `ffprobe` | 0 | `crates/media-test-support/src/oracle.rs:88` | - | Path of ffprobe, if available (cached for the process). |
| `ffprobe_or_skip` | 1 | `crates/media-test-support/src/oracle.rs:120` | - | ffprobe, or `None` after reporting a skip (see [`skip`]). |
| `fft_matches_dft` | 0 | `crates/audio-opus-codec/src/celt/mdct.rs:161` | numerics |  |
| `impl BibEntry::field` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:22` | - | Case-insensitive field lookup. |
| `impl AcroForm::field` | 2 | `crates/discourse-pdf-compliance/src/forms/model.rs:92` | - |  |
| `impl AcroForm::field_mut` | 2 | `crates/discourse-pdf-compliance/src/forms/model.rs:96` | - |  |
| `fields` | 2 | `crates/asset-production-vibe/src/lib.rs:543` | - |  |
| `fifth_external_texture_starts_a_fresh_run` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:771` | - |  |
| `file` | 1 | `crates/discourse-academic-tex/src/catalog.rs:66` | - |  |
| `file_arg` | 4 | `crates/discourse-academic-tex/src/texparse/graph.rs:186` | - |  |
| `file_name_lossy` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:213` | - |  |
| `fill` | 2 | `crates/discourse-academic-tex/src/review/logrules.rs:166` | - |  |
| `impl Scene::fill` | 6 | `crates/vector-compute-renderer/src/scene.rs:300` | - | Fills a shape using the specified style and brush. |
| `impl ColorPainter for DrawColorGlyphs<'_>::fill` | 2 | `crates/vector-compute-renderer/src/scene.rs:1000` | - |  |
| `impl PaintScene for VelloCpuScenePainter::fill` | 6 | `crates/vector-cpu-render-adapter/src/scene.rs:134` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::fill` | 6 | `crates/vector-gpu-render-adapter/src/scene.rs:104` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::fill` | 6 | `crates/vector-hybrid-render-adapter/src/scene.rs:256` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::fill` | 6 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:168` | - |  |
| `impl PaintScene for SkiaScenePainter<'_>::fill` | 6 | `crates/vector-skia-render-adapter/src/scene.rs:496` | - |  |
| `impl Scene::fill_blurred_rounded_rect` | 5 | `crates/vector-hybrid-renderer/src/scene.rs:550` | 2d | Fill a blurred rectangle with the given corner radius and standard deviation. |
| `impl Oscillator::fill_buffer` | 2 | `crates/acoustics-daw-session/src/dsp/osc.rs:75` | - | Fills a buffer with generated oscillator samples. |
| `fill_contains` | 3 | `crates/vector-locus-geometry/src/hit.rs:10` | - | Is `p` inside the filled area of `path` under `rule`? Open subpaths are implicitly closed. |
| `impl Dpb<F>::fill_frame_num_gap` | 7 | `crates/video-h264-codec/src/dpb.rs:131` | - | Insert "non-existing" frames for a frame_num gap (8.2.5.2). `make_frame` gives each one's |
| `impl ColorPainter for DrawColorGlyphs<'_>::fill_glyph` | 4 | `crates/vector-compute-renderer/src/scene.rs:1042` | - |  |
| `impl GlyphRunBackend<'a> for GpuGlyphRunBackend<'a>::fill_glyphs` | 3 | `crates/vector-hybrid-renderer/src/text.rs:265` | - |  |
| `fill_hit` | 0 | `crates/vector-locus-geometry/src/hit.rs:99` | - |  |
| `fill_lakes` | 4 | `crates/terrain-erosion-solvers/src/lacustrine_fill.rs:45` | economics | Fill lakes, depressions, and seas up to hydrostatic equilibrium across a 2D heightfield. |
| `fill_lakes_creates_flat_surface_in_bowl` | 0 | `crates/terrain-erosion-solvers/src/lacustrine_fill.rs:153` | - |  |
| `impl Decoder::fill_noise` | 3 | `crates/audio-aac-codec/src/decoder.rs:451` | - | Perceptual noise substitution. `pair` = (left channel, ms mask) for the right channel of a CPE: |
| `fill_occupancy_brick` | 6 | `crates/asset-production-vibe/src/geodesy_ops.rs:191` | - |  |
| `fill_path` | 3 | `crates/vector-compute-shaders/shader/fine.wgsl:1005` | 2d |  |
| `fill_path` | 5 | `crates/vector-compute-shaders/src/cpu/fine.rs:51` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::fill_path` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:330` | - |  |
| `impl Scene::fill_path` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:321` | - | Fill a path with the current paint and fill rule. |
| `impl DrawSink for Scene::fill_path` | 2 | `crates/vector-hybrid-renderer/src/text.rs:167` | - |  |
| `fill_path_ms` | 3 | `crates/vector-compute-shaders/shader/fine.wgsl:146` | 2d |  |
| `fill_path_ms_evenodd` | 3 | `crates/vector-compute-shaders/shader/fine.wgsl:511` | 2d |  |
| `impl Scene::fill_path_with` | 6 | `crates/vector-hybrid-renderer/src/scene.rs:341` | - | Build strips for a filled path with the given properties and record the draw. |
| `impl vello_common::probe::ProbeRenderer for Scene::fill_rect` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:334` | 2d |  |
| `impl Scene::fill_rect` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:450` | 2d | Fill a rectangle with the current paint and fill rule. |
| `impl DrawSink for Scene::fill_rect` | 2 | `crates/vector-hybrid-renderer/src/text.rs:172` | 2d |  |
| `impl Array::filled` | 3 | `crates/chronicle-tabular-engine/src/core/value.rs:99` | - |  |
| `impl RasterImage::filled` | 3 | `crates/surface-pbr-studio/src/raster.rs:23` | - |  |
| `impl AssetHost<'_>::filter` | 2 | `crates/asset-production-vibe/src/graphics.rs:337` | - |  |
| `filter` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:30` | - |  |
| `impl LayerSamplePlacement::filter` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:891` | - |  |
| `impl RoundStage::filter` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:368` | - |  |
| `filter8` | 6 | `crates/video-h264-codec/src/deblock.rs:83` | graph | Filter one set of eight samples p3 p2 p1 p0 \| q0 q1 q2 q3 across an edge (8.7.2.3 / 8.7.2.4): |
| `filter_binding_conflict` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:541` | - |  |
| `impl AssetHost<'_>::filter_biquad` | 2 | `crates/asset-production-vibe/src/acoustics_ops.rs:31` | - | Audio EQ Cookbook biquad IIR filter (lowpass, highpass, bandpass, notch, peaking). |
| `impl FiduciaryAuditTrail::filter_by_entity` | 3 | `crates/chronicle-fiduciary-ledger/src/audit.rs:159` | - |  |
| `filter_data_height_must_fit_resource_texture_limit` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:730` | - |  |
| `filter_horizontal` | 6 | `crates/video-h264-codec/src/deblock.rs:258` | graph | Filter a horizontal edge whose q0 row is `y`, for `N` columns starting at `x`. |
| `filter_lanes` | 3 | `crates/video-h264-codec/src/deblock.rs:151` | graph | Filter one edge for `N` lines at once (lane `i` = line `i` across the edge). `s[k][i]` holds |
| `filter_layer` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:750` | - |  |
| `filter_op` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:694` | - |  |
| `filter_op` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:569` | - |  |
| `filter_page_size` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:187` | - |  |
| `impl Backend for WebGlRendererContext<'_>::filter_pass` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3279` | - |  |
| `impl Backend for RendererContext<'_>::filter_pass` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3142` | - |  |
| `trait Backend::filter_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:61` | - | Execute a filter pass according to the filter plan between two textures. |
| `impl Backend for Recorder::filter_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:273` | - |  |
| `impl WebGlRendererContext<'_>::filter_pass_inner` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3067` | - |  |
| `impl RendererContext<'_>::filter_pass_inner` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2945` | - |  |
| `filter_round_resolving` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:794` | - |  |
| `impl KWeightingChannel::filter_sample` | 2 | `crates/acoustics-daw-session/src/dsp/meter.rs:38` | - |  |
| `filter_siblings` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:836` | - |  |
| `filter_spaces` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:37` | - |  |
| `impl GpuFilterData::filter_type` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:326` | - |  |
| `filter_vertical` | 6 | `crates/video-h264-codec/src/deblock.rs:230` | graph | Filter a vertical edge whose q0 column is at `x`, for `N` lines starting at row `y`. |
| `impl StreamDecoder::final_range` | 1 | `crates/audio-opus-codec/src/decoder.rs:67` | - | Final range coder state of the last decoded frame (for conformance checks). |
| `impl Decoder::final_range` | 1 | `crates/audio-opus-codec/src/multistream.rs:76` | - | Final range coder state of the first stream (conformance testing). |
| `impl Dsu::find` | 2 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:101` | - |  |
| `impl Body::find` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:128` | - |  |
| `find` | 2 | `crates/manifold-bim-csg/src/step.rs:104` | - |  |
| `find` | 2 | `crates/media-isobmff/src/bytes.rs:244` | - | Find the first child of the given type (malformed trailing children are ignored). |
| `impl Catalog::find` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:243` | - |  |
| `find_capture` | 3 | `crates/media-ogg/src/lib.rs:331` | - |  |
| `impl XEl::find_child` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:67` | - |  |
| `impl XEl::find_children` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:78` | - |  |
| `impl WGPUContext::find_compatible_device_handle` | 2 | `crates/gpu-device-context/src/lib.rs:209` | - | Finds or creates a compatible device handle id. |
| `find_dominant_aoa_peak` | 1 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:110` | - | Finds the dominant peak in a MUSIC spectrum. |
| `find_exact` | 1 | `crates/discourse-academic-tex/src/catalog.rs:400` | - | Exact lookup by `name` (case-sensitive). |
| `find_exact_hits_and_misses` | 0 | `crates/discourse-academic-tex/src/catalog.rs:461` | - |  |
| `impl WGPUContext::find_existing_device_idx` | 2 | `crates/gpu-device-context/src/lib.rs:218` | - | Finds  a compatible device handle id. |
| `find_min_reducible` | 2 | `crates/raster-spatial-operators/src/octree.rs:121` | graph |  |
| `impl References::find_mut` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:152` | - |  |
| `impl WGPUContext::find_or_create_device` | 2 | `crates/gpu-device-context/src/lib.rs:198` | - | Finds or creates a compatible device handle id. |
| `find_ph` | 3 | `crates/layout-presentation-deck/src/model/resolve.rs:67` | - |  |
| `find_pitch` | 1 | `crates/audio-opus-codec/src/celt/mod.rs:540` | - |  |
| `find_prefix` | 1 | `crates/discourse-academic-tex/src/catalog.rs:393` | - | Commands and environments whose `name` starts with `prefix` (case-sensitive). |
| `find_prefix_finds_subsection_family` | 0 | `crates/discourse-academic-tex/src/catalog.rs:441` | - |  |
| `impl Decoder::find_slot` | 4 | `crates/audio-aac-codec/src/decoder.rs:211` | - |  |
| `find_span` | 4 | `crates/drafting-geometry/src/spline.rs:224` | - |  |
| `find_tag_recursive` | 2 | `crates/chronicle-office-codecs/src/document/html.rs:179` | - |  |
| `find_tool` | 1 | `crates/media-test-support/src/oracle.rs:72` | - | Find a tool using the process environment (uncached). |
| `find_tool_in` | 4 | `crates/media-test-support/src/oracle.rs:40` | - | Find `name` given an explicit override, a `PATH`-style list and extra directories. |
| `finds_square_around_point` | 0 | `crates/drafting-geometry/src/region.rs:384` | - |  |
| `fine_main` | 8 | `crates/vector-compute-shaders/src/cpu/fine.rs:112` | - |  |
| `impl ZipArchiveWriter::finish` | 1 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:62` | - |  |
| `impl BitWriter::finish` | 1 | `crates/bitstream-codec/src/lib.rs:242` | - |  |
| `impl MkvWriter<W>::finish` | 1 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:419` | - | Write Cues, back-patch SeekHead / Segment size / Duration, and return writer. |
| `impl MxfWriter<W>::finish` | 1 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:219` | - |  |
| `impl Hasher for FxHasher::finish` | 1 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:16` | - |  |
| `impl XmlWriter::finish` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:123` | - |  |
| `impl IfcWriter::finish` | 2 | `crates/manifold-bim-csg/src/ifc.rs:74` | - | Finishes the document, wrapping it in standard ISO-10303-21 HEADER and DATA sections. |
| `impl Mp4Writer<W>::finish` | 1 | `crates/media-isobmff/src/mux.rs:677` | - | Finish the file with `moov` after `mdat`. Returns the underlying writer. |
| `impl FragmentedWriter<W>::finish` | 1 | `crates/media-isobmff/src/mux.rs:871` | - | Flush remaining samples and return the writer. |
| `impl Vectorscope::finish` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:464` | - |  |
| `impl VelloCpuScenePainter::finish` | 1 | `crates/vector-cpu-render-adapter/src/scene.rs:60` | - |  |
| `impl WebGlRendererInit::finish` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1081` | 3d | Blocks until shader compilation and program linking finish. |
| `impl PendingShaderProgram::finish` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1401` | - |  |
| `impl PendingWebGlPrograms::finish` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1535` | - |  |
| `impl Writer::finish` | 1 | `crates/video-prores-codec/src/bits.rs:180` | - | Pad with zero bits to a byte boundary and return the bytes. |
| `impl Decoder::finish_channel` | 3 | `crates/audio-aac-codec/src/decoder.rs:486` | - | TNS + filterbank for a single channel element; returns output samples. |
| `impl WebGlPendingProbe::finish_failure` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:248` | - |  |
| `impl Mp4Writer<W>::finish_faststart` | 1 | `crates/media-isobmff/src/mux.rs:688` | - | Finish with `moov` in front of `mdat` ("fast start"): media data is shifted forward in place. |
| `impl Track::finish_indexes` | 1 | `crates/media-isobmff/src/demux.rs:180` | - |  |
| `impl Mp4Writer<W>::finish_mdat` | 1 | `crates/media-isobmff/src/mux.rs:620` | - |  |
| `impl SliceDecoder<'a>::finish_pcm` | 1 | `crates/video-h264-codec/src/slicedec.rs:1078` | - | Common state for an I_PCM MB. |
| `impl RendererContext<'_>::finish_root_clear` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2607` | - |  |
| `impl WebGlPendingProbe::finish_success` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:216` | - |  |
| `finite` | 4 | `crates/vector-locus-pathops/src/three_d/revolve.rs:84` | - |  |
| `finite_positive` | 1 | `crates/profile-authoring/provenance/original-lib.rs:28` | - |  |
| `finite_positive` | 1 | `crates/profile-authoring/src/lib.rs:56` | - |  |
| `first_char` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:284` | - | Returns the first character of a known non-empty source section. |
| `first_class_strata_column` | 3 | `crates/asset-production-vibe/src/geodesy_ops.rs:251` | physics | First-class 6-layer STRATA column: sculpted elevation plus slope hydrology |
| `first_string` | 1 | `crates/manifold-bim-csg/src/step.rs:108` | - |  |
| `fisher_jenks_breaks` | 2 | `crates/geodesy-spatial-analysis/src/cartography.rs:127` | - | Fisher-Jenks natural breaks optimization algorithm for 1D sorted data. |
| `impl Frame::fit` | 3 | `crates/video-h264-preview/src/lib.rs:55` | - |  |
| `fit_cubics` | 4 | `crates/vector-locus-pathops/src/pathops/fit.rs:37` | 2d | Fit a chain of cubics through `pts` with maximum parametric error `tol`. |
| `fit_cubics` | 4 | `crates/vector-radiance-trace/src/fit.rs:421` | 2d |  |
| `fit_loop` | 2 | `crates/vector-radiance-trace/src/fit.rs:115` | 2d | Fit one loop into a closed Bézier subpath (pixel coordinates). |
| `fit_polyline` | 3 | `crates/vector-radiance-trace/src/fit.rs:140` | - | Fit a centre line through `pts` into a Bézier subpath. |
| `fit_rec` | 6 | `crates/vector-locus-pathops/src/pathops/fit.rs:57` | 2d |  |
| `fit_rec` | 6 | `crates/vector-radiance-trace/src/fit.rs:441` | 2d |  |
| `fit_single` | 3 | `crates/vector-locus-pathops/src/pathops/fit.rs:76` | 2d | Best single cubic for `pts` with the given end tangents. |
| `fit_single` | 3 | `crates/vector-radiance-trace/src/fit.rs:459` | 2d |  |
| `fit_single_from` | 4 | `crates/vector-locus-pathops/src/pathops/fit.rs:81` | 2d | Fit single starting from given parameter values `u`. |
| `fit_single_from` | 4 | `crates/vector-radiance-trace/src/fit.rs:463` | 2d |  |
| `fit_size` | 4 | `crates/raster-spatial-operators/src/resample.rs:328` | computational geometry | Compute target dimensions to fit aspect ratio inside bounding box. |
| `fit_size` | 4 | `crates/video-h264-preview/src/movie.rs:33` | - | The size that fits `w` x `h` inside `max_w` x `max_h`, keeping the aspect ratio (>= 1 px). |
| `impl TraceParams::fit_tolerance` | 1 | `crates/vector-radiance-trace/src/lib.rs:179` | - |  |
| `fit_variogram` | 2 | `crates/geodesy-spatial-analysis/src/kriging.rs:141` | - | Automatically fits a variogram model to empirical bins using weighted least squares. |
| `fitness` | 2 | `crates/vector-glyph-shaping/src/composer.rs:82` | - |  |
| `fits_a_cubic_exactly` | 0 | `crates/vector-locus-pathops/src/pathops/fit.rs:217` | - |  |
| `fits_in32` | 2 | `crates/audio-opus-codec/src/celt/rate.rs:71` | - |  |
| `impl Corner::fitted` | 2 | `crates/vector-locus-geometry/src/corners.rs:79` | - | The radius the corner is drawn with: `radius`, no larger than [`Corner::max_radius`], 0 when |
| `fitted_corner_radius` | 3 | `crates/vector-locus-geometry/src/shapes.rs:56` | - | The radius a corner of a `w` × `h` rectangle is drawn with. |
| `fixed` | 2 | `crates/chronicle-tabular-engine/src/numfmt/decimal.rs:64` | - |  |
| `fixed` | 3 | `crates/discourse-academic-tex/src/typeset.rs:513` | - |  |
| `impl PortReq::fixed` | 1 | `crates/kinetics-block-simulation/src/signal.rs:71` | - |  |
| `fixed` | 2 | `crates/video-h264-codec/src/inter.rs:56` | - |  |
| `fixed_from_digits` | 3 | `crates/chronicle-tabular-engine/src/numfmt/decimal.rs:18` | - |  |
| `fixture` | 1 | `crates/media-isobmff/tests/common/mod.rs:128` | - | Path to a fixture, generating it with ffmpeg if needed. `None` if ffmpeg is missing or the |
| `fixture_dir` | 0 | `crates/media-isobmff/tests/common/mod.rs:28` | - |  |
| `fixture_specs` | 0 | `crates/media-isobmff/tests/common/mod.rs:42` | - | (fixture name, ffmpeg arguments after the global options, output file name derived from the name). |
| `fixtures` | 0 | `crates/audio-aac-codec/tests/common/mod.rs:13` | - |  |
| `fixtures_dir` | 1 | `crates/media-test-support/src/fixtures.rs:26` | - | `<fixtures root>/<sub>` (created). `sub` may contain `/` (e.g. `"opus/oracle"`). |
| `fixtures_root` | 0 | `crates/media-test-support/src/fixtures.rs:18` | - | Root of all generated fixtures: `$FILMCRAFT_FIXTURES_DIR` or `<workspace>/target/fixtures`. |
| `flag` | 3 | `crates/vector-locus-pathops/src/effects/util.rs:24` | - |  |
| `flanagan_belytschko_hex8` | 5 | `crates/kinetics-continuum-multiphysics/src/explicit/hourglass.rs:14` | - | Compute Flanagan-Belytschko hourglass resisting nodal forces (8 nodes x 3 DOFs). |
| `impl ScalingMatrices::flat` | 0 | `crates/video-h264-codec/src/params.rs:22` | - |  |
| `impl ZzLists::flat` | 0 | `crates/video-h264-codec/src/params.rs:61` | - |  |
| `flat_colors` | 2 | `crates/vector-radiance-trace/src/quantize.rs:258` | - |  |
| `flat_nlsf_gives_small_lpc` | 0 | `crates/audio-opus-codec/src/silk/lpc.rs:343` | - |  |
| `flatten` | 2 | `crates/vector-compute-shaders/src/cpu/flatten.rs:847` | - |  |
| `flatten` | 1 | `crates/vector-locus-pathops/src/effects/scribble.rs:10` | - |  |
| `flatten_arc` | 6 | `crates/vector-compute-shaders/shader/flatten.wgsl:494` | - |  |
| `flatten_arc` | 9 | `crates/vector-compute-shaders/src/cpu/flatten.rs:158` | 2d |  |
| `flatten_cubic_bezier` | 2 | `crates/kinetics-garment-draping/src/pattern.rs:17` | - | Adaptive subdivision of a cubic Bézier curve within given tolerance in meters. |
| `flatten_euler` | 6 | `crates/vector-compute-shaders/shader/flatten.wgsl:328` | physics |  |
| `flatten_euler` | 9 | `crates/vector-compute-shaders/src/cpu/flatten.rs:203` | physics, 2d |  |
| `flatten_form` | 1 | `crates/discourse-pdf-compliance/src/forms/flatten.rs:24` | - | Flattens an AcroForm: renders the appearance of every field into static elements |
| `flatten_main` | 7 | `crates/vector-compute-shaders/src/cpu/flatten.rs:666` | - |  |
| `flatten_network` | 1 | `crates/layout-nodegraph-compiler/src/compiler/flatten.rs:9` | - |  |
| `flatten_numbers` | 1 | `crates/chronicle-tabular-engine/src/functions/util.rs:37` | - |  |
| `flatten_subpath` | 2 | `crates/vector-locus-pathops/src/pathops/boolean.rs:106` | - | Flatten a subpath into an ordered sequence of points. |
| `impl Poly::flip` | 1 | `crates/manifold-bim-csg/src/csg.rs:101` | - |  |
| `float` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:137` | - | IEEE float of 0, 4 or 8 bytes. |
| `floats` | 1 | `crates/color-transforms/src/lut.rs:188` | - |  |
| `floor` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:33` | - |  |
| `floor_boundary` | 2 | `crates/discourse-academic-tex/src/texparse/magic.rs:121` | - |  |
| `flow_accumulation` | 4 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:220` | - | Compute upstream flow accumulation area for every cell from D8 directions. |
| `flow_accumulation_accumulates_to_sink` | 0 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:319` | - |  |
| `impl Encoder::flush` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:283` | - | Encode the remaining input (zero-padded). Afterwards the decoded stream, minus |
| `impl Writer::flush` | 1 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:116` | - |  |
| `flush` | 6 | `crates/discourse-academic-tex/src/bibliography/import.rs:191` | - |  |
| `impl log::Log for BuildScriptLog::flush` | 1 | `crates/vector-compute-shaders/build.rs:171` | - |  |
| `impl SkiaBackend for MetalBackend::flush` | 2 | `crates/vector-skia-render-adapter/src/metal.rs:141` | - |  |
| `impl SkiaBackend for OpenGLBackend::flush` | 2 | `crates/vector-skia-render-adapter/src/opengl.rs:217` | - |  |
| `impl SkiaBackend for VulkanBackend::flush` | 2 | `crates/vector-skia-render-adapter/src/vulkan.rs:447` | - |  |
| `trait SkiaBackend::flush` | 2 | `crates/vector-skia-render-adapter/src/window_renderer.rs:14` | - |  |
| `impl Decoder::flush` | 1 | `crates/video-h264-codec/src/decoder.rs:256` | - | Output all remaining pictures (end of stream). |
| `impl Dpb<F>::flush` | 2 | `crates/video-h264-codec/src/dpb.rs:91` | - | Output everything in POC order and clear non-reference entries. |
| `impl MkvWriter<W>::flush_cluster` | 1 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:405` | - |  |
| `impl FragmentedWriter<W>::flush_fragment` | 1 | `crates/media-isobmff/src/mux.rs:833` | - | Write all buffered samples as one `moof` + `mdat` fragment (no-op if nothing is buffered). |
| `impl MkvWriter<W>::flush_pending` | 1 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:358` | - |  |
| `impl VulkanBackend::flush_post_multiplied` | 2 | `crates/vector-skia-render-adapter/src/vulkan.rs:322` | 3d | Performs the post-processing blit when compositing in POST_MULTIPLIED alpha mode. |
| `impl PicState::flush_rows` | 2 | `crates/video-h264-codec/src/slicedec.rs:121` | - | Deblock and publish rows. With `finished`, everything remaining is processed (also rows with |
| `impl Display for HostUnavailable::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:9` | - |  |
| `impl Display for BackendSpecificError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:34` | - |  |
| `impl Display for DevicesError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:53` | - |  |
| `impl Display for DeviceNameError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:76` | - |  |
| `impl Display for SupportedStreamConfigsError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:104` | - |  |
| `impl Display for DefaultStreamConfigError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:134` | - |  |
| `impl Display for BuildStreamError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:174` | - |  |
| `impl Display for PlayStreamError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:215` | - |  |
| `impl Display for PauseStreamError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:247` | - |  |
| `impl Display for StreamError::fmt` | 2 | `crates/audio-device-cpal/src/error.rs:276` | - |  |
| `impl fmt::Debug for Device::fmt` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:425` | - |  |
| `impl fmt::Debug for Device::fmt` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:813` | - |  |
| `impl Display for SampleFormat::fmt` | 2 | `crates/audio-device-cpal/src/samples_formats.rs:96` | - |  |
| `impl std::fmt::Display for Error::fmt` | 2 | `crates/audio-opus-codec/src/lib.rs:32` | - |  |
| `impl fmt::Display for BitError::fmt` | 2 | `crates/bitstream-codec/src/lib.rs:21` | - |  |
| `impl fmt::Debug for Ul::fmt` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:26` | - |  |
| `impl fmt::Display for Ul::fmt` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:38` | - |  |
| `impl std::fmt::Debug for Umid::fmt` | 2 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:12` | - |  |
| `impl fmt::Display for XmlParseError::fmt` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:190` | - |  |
| `impl fmt::Display for ZipError::fmt` | 2 | `crates/chronicle-office-codecs/src/zip/minizip.rs:18` | - |  |
| `impl fmt::Display for CellRef::fmt` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:62` | - |  |
| `impl fmt::Display for RangeRef::fmt` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:261` | - |  |
| `impl fmt::Display for CellError::fmt` | 2 | `crates/chronicle-tabular-engine/src/core/value.rs:78` | - |  |
| `impl fmt::Display for LexError::fmt` | 2 | `crates/chronicle-tabular-engine/src/formula/lexer.rs:42` | - |  |
| `impl fmt::Display for ParseError::fmt` | 2 | `crates/chronicle-tabular-engine/src/formula/parser.rs:18` | - |  |
| `impl std::fmt::Debug for MobId::fmt` | 2 | `crates/chronicle-timeline-interchange/src/interchange/aaf.rs:15` | - |  |
| `impl std::fmt::Display for CmsError::fmt` | 2 | `crates/color-transforms/src/icc/profile.rs:18` | - |  |
| `impl std::fmt::Display for PaletteError::fmt` | 2 | `crates/color-transforms/src/palette/aco.rs:13` | - |  |
| `impl std::fmt::Display for PdfError::fmt` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:18` | - |  |
| `impl fmt::Display for LangError::fmt` | 2 | `crates/discourse-scientific-matlang/src/interpreter/lexer.rs:12` | - |  |
| `impl fmt::Display for MatError::fmt` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:17` | - |  |
| `impl std::fmt::Display for DxfError::fmt` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:73` | - |  |
| `impl std::fmt::Debug for BufferRenderer::fmt` | 2 | `crates/gpu-device-context/src/buffer_renderer.rs:26` | - |  |
| `impl Display for WgpuContextError::fmt` | 2 | `crates/gpu-device-context/src/error.rs:30` | - |  |
| `impl std::fmt::Debug for SurfaceRenderer<'_>::fmt` | 2 | `crates/gpu-device-context/src/surface_renderer.rs:135` | - |  |
| `impl fmt::Display for SimError::fmt` | 2 | `crates/kinetics-block-simulation/src/lib.rs:42` | - |  |
| `impl fmt::Display for SyncError::fmt` | 2 | `crates/kinetics-schedule-cpm/src/change_dag.rs:106` | - |  |
| `impl fmt::Display for FormatError::fmt` | 2 | `crates/kinetics-schedule-cpm/src/format.rs:28` | - |  |
| `impl fmt::Display for CoreError::fmt` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:19` | - |  |
| `impl fmt::Display for FieldValue::fmt` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:247` | - |  |
| `impl fmt::Display for SchedError::fmt` | 2 | `crates/kinetics-schedule-cpm/src/schedule.rs:33` | - |  |
| `impl fmt::Display for SolveError::fmt` | 2 | `crates/locus-parametric-solver/src/solver/conflict.rs:23` | - |  |
| `impl fmt::Display for ExprError::fmt` | 2 | `crates/locus-parametric-solver/src/solver/expr.rs:24` | - |  |
| `impl fmt::Display for FourCc::fmt` | 2 | `crates/media-isobmff/src/bytes.rs:32` | - |  |
| `impl fmt::Debug for FourCc::fmt` | 2 | `crates/media-isobmff/src/bytes.rs:37` | - |  |
| `impl fmt::Display for Error::fmt` | 2 | `crates/media-ogg/src/lib.rs:71` | - |  |
| `impl std::fmt::Display for Diff::fmt` | 2 | `crates/media-test-support/src/golden.rs:109` | - |  |
| `impl std::fmt::Display for AuthoringError::fmt` | 2 | `crates/profile-authoring/provenance/original-lib.rs:20` | - |  |
| `impl std::fmt::Display for AuthoringError::fmt` | 2 | `crates/profile-authoring/src/lib.rs:24` | - |  |
| `impl std::fmt::Display for PreflightError::fmt` | 2 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:55` | - |  |
| `impl std::fmt::Display for PreflightError::fmt` | 2 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:55` | - |  |
| `impl std::fmt::Display for AdapterError::fmt` | 2 | `crates/raster-layer-mask-adapter/src/lib.rs:55` | - |  |
| `impl std::fmt::Display for PreflightError::fmt` | 2 | `crates/raster-selection-morphology/src/lib.rs:66` | - |  |
| `impl std::fmt::Display for GeoStudioError::fmt` | 2 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:24` | - |  |
| `impl std::fmt::Display for GeoStudioError::fmt` | 2 | `crates/terrain-brushes/src/lib.rs:30` | - |  |
| `impl Debug for DebugLayers::fmt` | 2 | `crates/vector-compute-renderer/src/debug.rs:22` | - |  |
| `impl fmt::Debug for LineEndpoint::fmt` | 2 | `crates/vector-compute-renderer/src/debug/validate.rs:38` | - |  |
| `impl std::fmt::Debug for RenderSurface<'_>::fmt` | 2 | `crates/vector-compute-renderer/src/util.rs:234` | - |  |
| `impl fmt::Display for ErrorVec::fmt` | 2 | `crates/vector-compute-shaders/src/compile/mod.rs:48` | - |  |
| `impl fmt::Debug for BindingIndex::fmt` | 2 | `crates/vector-compute-shaders/src/types.rs:53` | - |  |
| `impl fmt::Display for MessageSuffix<'_>::fmt` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/error.rs:193` | - |  |
| `impl std::fmt::Debug for Movie::fmt` | 2 | `crates/video-h264-preview/src/movie.rs:134` | - |  |
| `fmt6` | 1 | `crates/color-transforms/src/lut.rs:457` | - |  |
| `fmt_fixed` | 2 | `crates/chronicle-fiduciary-ledger/src/money.rs:97` | - |  |
| `fmt_money` | 1 | `crates/chronicle-fiduciary-ledger/src/money.rs:113` | - |  |
| `fmt_money_grouped` | 1 | `crates/chronicle-fiduciary-ledger/src/money.rs:118` | - | Formats money with thousands separators: e.g. `12,345.60` or `-1,234.56`. |
| `fnv1a_60` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:564` | - | Deterministic FNV-1a 60-bit hash (`lexicon.rs` mask `0x0FFF_FFFF_FFFF_FFFF`). |
| `fnv1a_hash_str` | 2 | `crates/layout-nodegraph-compiler/src/compiler/cse.rs:9` | - |  |
| `fnv1a_hash_tagged_value` | 2 | `crates/layout-nodegraph-compiler/src/compiler/cse.rs:17` | - |  |
| `fnv1a_hex` | 1 | `crates/asset-production-vibe/src/chronicle_ops.rs:455` | - |  |
| `fnv1a_hex` | 1 | `crates/chronicle-fiduciary-ledger/src/audit.rs:167` | - |  |
| `fnv1a_hex` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:320` | - |  |
| `impl Doc::font` | 4 | `crates/discourse-academic-tex/src/texpdf.rs:310` | - |  |
| `impl DrawGlyphs<'a>::font_embolden` | 2 | `crates/vector-compute-renderer/src/scene.rs:546` | - | Sets synthetic embolden settings. |
| `impl DrawGlyphs<'a>::font_size` | 2 | `crates/vector-compute-renderer/src/scene.rs:539` | - | Sets the font size in pixels per em units. |
| `for_chunks` | 3 | `crates/raster-selection-morphology/src/distance.rs:75` | - |  |
| `impl Matrix::for_code` | 3 | `crates/video-h264-preview/src/convert.rs:29` | numerics | Choose by ITU-T H.273 matrix code (1 = 709, 5/6 = 601), else by picture height. |
| `impl DecodeTable::for_curve` | 2 | `crates/color-transforms/src/transform.rs:90` | - | The decode table for a colour space's curve. |
| `impl Ctx<'a>::for_layout` | 3 | `crates/layout-presentation-deck/src/model/resolve.rs:36` | - |  |
| `impl Ctx<'a>::for_master` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:46` | - |  |
| `impl Ctx<'a>::for_slide` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:25` | - |  |
| `force_regular` | 1 | `crates/discourse-academic-tex/src/typeset.rs:346` | physics |  |
| `impl PerspectiveGrid::foreshortening` | 1 | `crates/vector-locus-geometry/src/perspective.rs:199` | - | Foreshortening ratios along receding axes: left, right, vertical. |
| `impl Timecode::format` | 2 | `crates/audio-timebase/src/timecode.rs:201` | - | Format as standard `HH:MM:SS:FF` or drop-frame `HH:MM:SS;FF`. |
| `impl FeetFrames::format` | 1 | `crates/audio-timebase/src/timecode.rs:279` | - | Format as standard `FEET+FRAMES` (e.g. `1+08`). |
| `impl TransportPosition::format` | 3 | `crates/audio-timebase/src/transport.rs:87` | - | Format this position according to the specified `TimeFormat`. |
| `impl NumberFormat::format` | 3 | `crates/chronicle-tabular-engine/src/numfmt/mod.rs:42` | - |  |
| `impl Citation::format` | 3 | `crates/layout-paged-typesetter/src/citations/csl.rs:39` | - |  |
| `impl Unit::format` | 2 | `crates/vector-locus-geometry/src/units.rs:104` | - | Format a point value in this unit (e.g. `12.5 pt`). |
| `format_clock` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:200` | - | Format `Tick` as SRT/WebVTT clock (`HH:MM:SS,mmm` or `HH:MM:SS.mmm`). |
| `format_csl_citation_field` | 1 | `crates/layout-paged-typesetter/src/citations/zotero_fields.rs:23` | - |  |
| `format_fcpxml_time` | 2 | `crates/chronicle-timeline-interchange/src/interchange/fcpxml.rs:31` | - | Format a `Tick` as an FCPXML rational time string. |
| `format_field` | 2 | `crates/discourse-pdf-compliance/src/forms/validation.rs:42` | - |  |
| `impl TimecodeConfig::format_frame` | 2 | `crates/media-isobmff/src/codec.rs:709` | - | Format a frame number as `HH:MM:SS:FF` (`;` before the frames when drop-frame). |
| `format_from_waveformatex_ptr` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:196` | - |  |
| `format_full_entry` | 1 | `crates/layout-paged-typesetter/src/citations/csl.rs:73` | - |  |
| `format_general_fit` | 2 | `crates/chronicle-tabular-engine/src/numfmt/decimal.rs:72` | - |  |
| `format_grouped_money` | 0 | `crates/chronicle-fiduciary-ledger/src/money.rs:136` | - |  |
| `format_length` | 6 | `crates/audio-timebase/src/transport.rs:146` | - | Format a duration or length starting at `start`. |
| `format_min_secs` | 0 | `crates/audio-timebase/src/transport.rs:237` | - |  |
| `format_position` | 6 | `crates/audio-timebase/src/transport.rs:120` | - | Format an absolute sample position into a string. |
| `format_timecode` | 3 | `crates/chronicle-timeline-interchange/src/time.rs:106` | - | Convert frame count to SMPTE timecode (HH:MM:SS:FF or HH:MM:SS;FF). |
| `formatting` | 0 | `crates/vector-locus-geometry/src/units.rs:211` | - |  |
| `impl Mdct::forward` | 3 | `crates/audio-aac-codec/src/mdct.rs:109` | - | Forward MDCT of `x` (length 2M, already windowed) into `out` (length M). |
| `impl Aeqd::forward` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:381` | 3d | Forward projection: lon/lat in degrees -> planar metres east and north of centre. |
| `impl Sinusoidal::forward` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:435` | 3d | Forward projection: lon/lat degrees -> projected metres. |
| `forward_matches_definition` | 0 | `crates/audio-aac-codec/src/mdct.rs:223` | - |  |
| `impl Cur<'a>::fourcc` | 1 | `crates/media-isobmff/src/bytes.rs:123` | - |  |
| `impl Profile::fourcc` | 1 | `crates/video-prores-codec/src/types.rs:103` | - | The QuickTime sample-entry four-character code. |
| `frac_mul16` | 2 | `crates/audio-opus-codec/src/celt/bands.rs:194` | - |  |
| `impl ArcPath::fraction_at` | 2 | `crates/vector-locus-geometry/src/arc.rs:72` | - | Distance as a fraction (0..1) of the path's length. |
| `fractional_radii_follow_coverage_rules` | 0 | `crates/raster-selection-morphology/src/tests.rs:60` | - |  |
| `impl JohnsonCookDamage::fracture_strain` | 4 | `crates/kinetics-continuum-multiphysics/src/explicit/damage.rs:23` | - | Fracture strain eps_f = [d1 + d2 * exp(d3 * sigma*)] * [1 + d4 * ln(eps_dot*)] * [1 + d5 * T*] |
| `fragmented_roundtrip` | 0 | `crates/media-isobmff/src/mux.rs:932` | - |  |
| `fragmented_writer_ffmpeg` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:232` | - |  |
| `impl Pixels<'win>::frame` | 1 | `crates/pixel-buffer-output/src/lib.rs:646` | 2d | Get an immutable byte slice for the pixel buffer. |
| `frame` | 1 | `crates/timeline-editing/src/transition.rs:276` | - |  |
| `impl Track::frame` | 2 | `crates/vector-surface-brush/src/track.rs:123` | 2d | Point and unit tangent at arc length `s`. |
| `impl Poc::frame` | 1 | `crates/video-h264-codec/src/slice.rs:495` | - |  |
| `impl Movie::frame` | 2 | `crates/video-h264-preview/src/movie.rs:334` | - |  |
| `impl FrameRate::frame_at` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:91` | - |  |
| `impl FrameRate::frame_at` | 2 | `crates/kinetics-motion-compositor/src/time.rs:69` | - |  |
| `impl FrameRate::frame_at` | 2 | `crates/timeline-editing/src/timebase.rs:174` | - | Index of the frame containing `time`, rounded down. |
| `impl Movie::frame_at` | 2 | `crates/video-h264-preview/src/movie.rs:325` | - |  |
| `frame_at_inverts_tick_of` | 0 | `crates/chronicle-timeline-interchange/src/time.rs:216` | - |  |
| `frame_boundary` | 2 | `crates/timeline-editing/src/timebase.rs:202` | - | Ceiling division for a frame boundary, matching the chronicle media exact rational |
| `frame_codes` | 0 | `crates/audio-opus-codec/src/packet.rs:289` | - |  |
| `frame_color` | 1 | `crates/video-h264-preview/src/synth.rs:113` | - | Limited-range Y'CbCr of frame `i` for synthetic test streams. |
| `impl FrameRate::frame_duration` | 1 | `crates/timeline-editing/src/timebase.rs:165` | - |  |
| `impl Sps::frame_height_in_mbs` | 1 | `crates/video-h264-codec/src/params.rs:450` | - |  |
| `impl PerspectiveGrid::frame_homography` | 4 | `crates/vector-locus-geometry/src/perspective.rs:243` | - | The plane spanned by axes `u` and `v` through the point `at.1` along axis `at.0`. |
| `frame_index` | 2 | `crates/timeline-editing/src/timebase.rs:194` | - | Keep the mathematical frame index wide until the public API needs an i64. |
| `impl Movie::frame_index_at` | 2 | `crates/video-h264-preview/src/movie.rs:313` | - |  |
| `impl Pixels<'win>::frame_mut` | 1 | `crates/pixel-buffer-output/src/lib.rs:638` | 2d | Get a mutable byte slice for the pixel buffer. The buffer is _not_ cleared for you; it will |
| `impl PocState::frame_num_offset` | 4 | `crates/video-h264-codec/src/slice.rs:550` | - |  |
| `impl Dpb<F>::frame_num_wrap` | 3 | `crates/video-h264-codec/src/dpb.rs:65` | - |  |
| `impl Toc::frame_samples` | 2 | `crates/audio-opus-codec/src/packet.rs:74` | - | Samples per frame at `rate`. |
| `impl Toc::frame_samples_48k` | 1 | `crates/audio-opus-codec/src/packet.rs:69` | - | Samples per frame at 48 kHz. |
| `impl Movie::frame_time` | 2 | `crates/video-h264-preview/src/movie.rs:321` | - |  |
| `impl WebGlIntermediateTexture::framebuffer` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1352` | - |  |
| `impl TimecodeRate::frames_at` | 3 | `crates/audio-timebase/src/timecode.rs:101` | - | Total whole frames at a sample position (floored). |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/alsa/mod.rs:905` | - |  |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/asio/stream.rs:621` | - | Convert the given duration in frames at the given sample rate to a `std::time::Duration`. |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:84` | - |  |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:417` | - |  |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/jack/stream.rs:407` | - |  |
| `frames_to_duration` | 2 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:546` | - | Convert the given duration in frames at the given sample rate to a `std::time::Duration`. |
| `impl Recording::free_buffer` | 2 | `crates/vector-compute-renderer/src/recording.rs:207` | - | Commands to free the given buffer. |
| `free_distort` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:31` | computational geometry, 2d | Free Distort: bilinear mapping of bounding box onto four corners. |
| `impl WgpuEngine::free_download` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:783` | - |  |
| `impl Recording::free_image` | 2 | `crates/vector-compute-renderer/src/recording.rs:212` | - | Commands to free the given image. |
| `impl Recording::free_resource` | 2 | `crates/vector-compute-renderer/src/recording.rs:217` | - | Commands to free the given resource. |
| `impl From<bitstream_codec::BitError> for Error::from` | 1 | `crates/audio-aac-codec/src/lib.rs:69` | - |  |
| `impl From<BackendSpecificError> for DevicesError::from` | 1 | `crates/audio-device-cpal/src/error.rs:63` | - |  |
| `impl From<BackendSpecificError> for DeviceNameError::from` | 1 | `crates/audio-device-cpal/src/error.rs:86` | - |  |
| `impl From<BackendSpecificError> for SupportedStreamConfigsError::from` | 1 | `crates/audio-device-cpal/src/error.rs:116` | - |  |
| `impl From<BackendSpecificError> for DefaultStreamConfigError::from` | 1 | `crates/audio-device-cpal/src/error.rs:150` | - |  |
| `impl From<BackendSpecificError> for BuildStreamError::from` | 1 | `crates/audio-device-cpal/src/error.rs:196` | - |  |
| `impl From<BackendSpecificError> for PlayStreamError::from` | 1 | `crates/audio-device-cpal/src/error.rs:228` | - |  |
| `impl From<BackendSpecificError> for PauseStreamError::from` | 1 | `crates/audio-device-cpal/src/error.rs:260` | - |  |
| `impl From<BackendSpecificError> for StreamError::from` | 1 | `crates/audio-device-cpal/src/error.rs:289` | - |  |
| `impl From<alsa::Error> for DevicesError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/enumerate.rs:66` | - |  |
| `impl From<alsa::Error> for BackendSpecificError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1123` | - |  |
| `impl From<alsa::Error> for BuildStreamError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1131` | - |  |
| `impl From<alsa::Error> for SupportedStreamConfigsError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1138` | - |  |
| `impl From<alsa::Error> for PlayStreamError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1145` | - |  |
| `impl From<alsa::Error> for PauseStreamError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1152` | - |  |
| `impl From<alsa::Error> for StreamError::from` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1159` | - |  |
| `impl From<coreaudio::Error> for BuildStreamError::from` | 1 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:93` | - |  |
| `impl From<coreaudio::Error> for SupportedStreamConfigsError::from` | 1 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:106` | - |  |
| `impl From<coreaudio::Error> for DefaultStreamConfigError::from` | 1 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:115` | - |  |
| `impl From<oboe::Error> for StreamError::from` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:30` | - |  |
| `impl From<oboe::Error> for PlayStreamError::from` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:43` | - |  |
| `impl From<oboe::Error> for PauseStreamError::from` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:56` | - |  |
| `impl From<oboe::Error> for BuildStreamError::from` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:69` | - |  |
| `impl From<Audio::IMMDevice> for Endpoint::from` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:822` | - |  |
| `impl From<windows::core::Error> for BackendSpecificError::from` | 1 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:53` | - |  |
| `impl From<SupportedStreamConfig> for StreamConfig::from` | 1 | `crates/audio-device-cpal/src/lib.rs:813` | - |  |
| `impl From<Value> for Ev::from` | 1 | `crates/chronicle-tabular-engine/src/calc/eval.rs:31` | - |  |
| `impl From<wgpu::CreateSurfaceError> for WgpuContextError::from` | 1 | `crates/gpu-device-context/src/error.rs:58` | - |  |
| `impl From<RequestAdapterError> for WgpuContextError::from` | 1 | `crates/gpu-device-context/src/error.rs:64` | - |  |
| `impl From<RequestDeviceError> for WgpuContextError::from` | 1 | `crates/gpu-device-context/src/error.rs:70` | - |  |
| `impl From<PollError> for WgpuContextError::from` | 1 | `crates/gpu-device-context/src/error.rs:76` | - |  |
| `impl From<&[u8; 4]> for FourCc::from` | 1 | `crates/media-isobmff/src/bytes.rs:42` | - |  |
| `impl From<io::Error> for Error::from` | 1 | `crates/media-ogg/src/lib.rs:83` | - |  |
| `impl From<anyrender::RendererConfig> for PixelsRendererOptions::from` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:65` | - |  |
| `impl From<BufferProxy> for ResourceProxy::from` | 1 | `crates/vector-compute-renderer/src/recording.rs:299` | - |  |
| `impl From<ImageProxy> for ResourceProxy::from` | 1 | `crates/vector-compute-renderer/src/recording.rs:305` | - |  |
| `impl From<Encoding> for Scene::from` | 1 | `crates/vector-compute-renderer/src/scene.rs:453` | - |  |
| `impl From<anyrender::RendererConfig> for VelloRendererOptions::from` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:149` | - |  |
| `impl From<anyrender::RendererConfig> for VelloHybridRendererOptions::from` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:139` | - |  |
| `impl From<&Offset> for GpuOffset::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:204` | - |  |
| `impl From<&Flood> for GpuFlood::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:223` | - |  |
| `impl From<&GaussianBlur> for GpuGaussianBlur::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:248` | - |  |
| `impl From<&DropShadow> for GpuDropShadow::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:287` | - |  |
| `impl From<T> for GpuFilterData::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:354` | - |  |
| `impl From<&PreparedFilter> for GpuFilterData::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:360` | - |  |
| `impl From<TextureParity> for u8::from` | 1 | `crates/vector-hybrid-renderer/src/target.rs:68` | - |  |
| `impl From<TextureParity> for u32::from` | 1 | `crates/vector-hybrid-renderer/src/target.rs:74` | - |  |
| `impl From<AnchorRepr> for Anchor::from` | 1 | `crates/vector-locus-geometry/src/path.rs:58` | - |  |
| `impl From<Anchor> for AnchorRepr::from` | 1 | `crates/vector-locus-geometry/src/path.rs:65` | - |  |
| `impl From<anyrender::RendererConfig> for SkiaRendererOptions::from` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:67` | - |  |
| `impl Circle::from_2_points` | 2 | `crates/drafting-geometry/src/curve.rs:168` | 2d |  |
| `impl Arc::from_3_points` | 3 | `crates/drafting-geometry/src/curve.rs:91` | 2d | Arc through three points, or `None` when collinear. |
| `impl Circle::from_3_points` | 3 | `crates/drafting-geometry/src/curve.rs:143` | 2d |  |
| `impl Decoder::from_adts` | 1 | `crates/audio-aac-codec/src/decoder.rs:64` | - | Create a decoder for an ADTS stream from its first header. With channel configuration 0 the |
| `impl Homography::from_affine` | 1 | `crates/vector-locus-geometry/src/projective.rs:18` | - | The affine map `a` as a homography. |
| `impl Vec2::from_angle` | 1 | `crates/drafting-geometry/src/vec.rs:21` | - | Unit vector at `angle` radians. |
| `impl EulerParams::from_angles` | 2 | `crates/vector-compute-shaders/src/cpu/euler.rs:165` | - |  |
| `impl Vec3::from_array` | 1 | `crates/surface-pbr-studio/src/math.rs:176` | - |  |
| `impl Vec4::from_array` | 1 | `crates/surface-pbr-studio/src/math.rs:261` | - |  |
| `impl Vec2::from_array` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:105` | - |  |
| `impl Homography::from_array` | 1 | `crates/vector-locus-geometry/src/projective.rs:24` | - | From nine row-major numbers; `None` unless all are finite and the map is invertible. |
| `impl AacConfig::from_asc` | 1 | `crates/media-isobmff/src/codec.rs:568` | - | Build from an AudioSpecificConfig, decoding its header fields. |
| `impl Decoder::from_avcc` | 1 | `crates/video-h264-codec/src/decoder.rs:183` | - | Configure from an `avcC` (AVCDecoderConfigurationRecord) box payload. |
| `from_be` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:681` | - | Helper function to convert from little endianness. |
| `impl PathData::from_bezpath` | 1 | `crates/vector-locus-geometry/src/path.rs:374` | - | Convert a kurbo path to anchors. Quadratics are elevated to cubics. |
| `impl WindowShape::from_bit` | 1 | `crates/audio-aac-codec/src/mdct.rs:155` | - |  |
| `impl WindowSequence::from_bits` | 1 | `crates/audio-aac-codec/src/ics.rs:20` | - |  |
| `impl DebugLayers::from_bits` | 1 | `crates/vector-compute-renderer/src/debug.rs:72` | - | Construct a `DebugLayers` from the raw bits. |
| `impl BlendRange::from_bytes` | 1 | `crates/raster-layer-mask-adapter/src/masks.rs:177` | - |  |
| `impl Matrix::from_code` | 1 | `crates/color-transforms/src/lib.rs:47` | numerics | From an ISO/IEC 23091-2 `matrix_coefficients` code. |
| `impl Transfer::from_code` | 1 | `crates/color-transforms/src/lib.rs:72` | - |  |
| `impl Matrix::from_code` | 1 | `crates/radiance-color-grading/src/matrix.rs:33` | numerics | From an ISO/IEC 23091-2 `matrix_coefficients` code. |
| `impl Transfer::from_code` | 1 | `crates/radiance-color-grading/src/matrix.rs:58` | - |  |
| `impl BrushFalloff::from_code` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:49` | - |  |
| `impl BrushFalloff::from_code` | 1 | `crates/terrain-brushes/src/lib.rs:55` | - |  |
| `impl Matrix::from_col_major` | 3 | `crates/discourse-scientific-matlang/src/matrix.rs:94` | - |  |
| `impl DMat2::from_cols` | 2 | `crates/kinetics-garment-draping/src/math.rs:210` | - |  |
| `impl Homography::from_cols` | 3 | `crates/vector-locus-geometry/src/projective.rs:13` | - | From the images of (1,0,0), (0,1,0) and (0,0,1). |
| `impl Decoder::from_config` | 1 | `crates/audio-aac-codec/src/decoder.rs:68` | - |  |
| `impl Spline::from_control` | 2 | `crates/drafting-geometry/src/spline.rs:22` | 2d | A clamped uniform spline over control points. |
| `from_csv` | 2 | `crates/kinetics-schedule-cpm/src/format.rs:106` | - |  |
| `impl Lut::from_cube` | 1 | `crates/color-transforms/src/lut.rs:193` | - |  |
| `impl Lut::from_cube` | 1 | `crates/radiance-color-grading/src/lut.rs:212` | - |  |
| `impl ShaderInfo::from_default` | 0 | `crates/vector-compute-shaders/src/compile/mod.rs:182` | 3d | Same as [`ShaderInfo::from_dir`] but uses the default shader directory provided by [`shader_dir`]. |
| `impl ShaderInfo::from_dir` | 1 | `crates/vector-compute-shaders/src/compile/mod.rs:186` | 3d |  |
| `impl OpusHead::from_dops` | 1 | `crates/audio-opus-codec/src/header.rs:62` | - | Parses the payload of an ISO-BMFF `dOps` box (Opus in MP4, big-endian fields). |
| `impl BlendStrip::from_fill_segment` | 2 | `crates/vector-hybrid-renderer/src/blend.rs:108` | 2d |  |
| `impl GpuStrip::from_fill_segment` | 5 | `crates/vector-hybrid-renderer/src/draw.rs:397` | 2d |  |
| `impl Spline::from_fit` | 2 | `crates/drafting-geometry/src/spline.rs:154` | 2d | Interpolation through fit points, open or closed (see `from_fit_points_closed`). |
| `impl Spline::from_fit_points` | 1 | `crates/drafting-geometry/src/spline.rs:31` | 2d, numerics | Cubic interpolation through fit points (chord-length parametrisation, natural-ish ends |
| `impl Spline::from_fit_points_closed` | 1 | `crates/drafting-geometry/src/spline.rs:94` | 2d | Smooth closed (periodic) cubic interpolation through fit points. Chord-length parameters |
| `impl Lut3d::from_fn` | 2 | `crates/color-transforms/src/lut.rs:64` | - | Sample a colour transform on a `size³` grid over 0..1. |
| `impl Lut3d::from_fn` | 2 | `crates/radiance-color-grading/src/lut.rs:67` | - |  |
| `impl Signal::from_fn` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:198` | - |  |
| `impl DecodeTable::from_fn` | 2 | `crates/radiance-color-grading/src/transform.rs:156` | - |  |
| `impl Raster::from_fn` | 3 | `crates/vector-radiance-trace/src/lib.rs:65` | - |  |
| `impl Profile::from_fourcc` | 1 | `crates/video-prores-codec/src/types.rs:114` | - |  |
| `impl Timecode::from_frames` | 2 | `crates/audio-timebase/src/timecode.rs:150` | - | Compute timecode label for an absolute frame count. |
| `impl Decoder::from_head` | 2 | `crates/audio-opus-codec/src/multistream.rs:34` | - | Creates a decoder for a parsed header at `sample_rate` (8/12/16/24/48 kHz). |
| `impl TimeFormat::from_id` | 1 | `crates/audio-timebase/src/transport.rs:49` | - |  |
| `impl WarpStyle::from_id` | 1 | `crates/vector-locus-pathops/src/effects/warp.rs:34` | - |  |
| `impl Device::from_immdevice` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:335` | - |  |
| `impl ColorSpace::from_info` | 1 | `crates/color-transforms/src/spaces.rs:284` | - | The colour space signalled by stream metadata (VUI / `colr` / MKV Colour). |
| `impl FromIterator<AaConfig> for AaSupport::from_iter` | 1 | `crates/vector-compute-renderer/src/lib.rs:239` | - |  |
| `impl Element::from_jet` | 2 | `crates/kinetics-garment-draping/src/energy.rs:103` | - |  |
| `from_json` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:51` | - |  |
| `impl PsdBlendMode::from_key` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:214` | - |  |
| `from_le` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:676` | - | Helper function to convert from little endianness. |
| `impl RefMedia::from_library` | 5 | `crates/multimodal-synthesis-schema/src/references.rs:71` | - |  |
| `impl LightTreeMeasure::from_light` | 1 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:42` | - |  |
| `from_linear` | 2 | `crates/color-transforms/src/lib.rs:204` | numerics |  |
| `from_linear` | 2 | `crates/radiance-color-grading/src/matrix.rs:242` | numerics | Convert linear light to encoded display value. |
| `impl GrayImage::from_luma` | 3 | `crates/kinetics-motion-compositor/src/track/correlation.rs:22` | - |  |
| `impl Mat4::from_mat3` | 1 | `crates/drafting-geometry/src/mat.rs:136` | 3d |  |
| `impl Tick::from_millis` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:42` | - |  |
| `from_mspdi_xml` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:356` | - |  |
| `impl StreamInstant::from_nanos` | 1 | `crates/audio-device-cpal/src/lib.rs:457` | - |  |
| `impl StreamInstant::from_nanos_i128` | 1 | `crates/audio-device-cpal/src/lib.rs:464` | - |  |
| `impl PerspectiveGrid::from_page` | 3 | `crates/vector-locus-geometry/src/perspective.rs:263` | - | Map a page coordinate point to plane coordinates. |
| `impl EulerSeg::from_params` | 3 | `crates/vector-compute-shaders/src/cpu/euler.rs:218` | 2d |  |
| `impl TextureParity::from_parity` | 1 | `crates/vector-hybrid-renderer/src/target.rs:44` | - |  |
| `impl Data::from_parts` | 3 | `crates/audio-device-cpal/src/lib.rs:512` | - |  |
| `impl LutFormat::from_path` | 1 | `crates/color-transforms/src/lut.rs:172` | - |  |
| `impl RefKind::from_path` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:27` | - |  |
| `impl Frame::from_planes` | 3 | `crates/video-h264-codec/src/picture.rs:220` | - | A frame with every row published from `planes` and no motion (intra), e.g. "non-existing" |
| `impl Bounds2::from_points` | 1 | `crates/drafting-geometry/src/bounds.rs:24` | 2d |  |
| `impl Polyline::from_points` | 2 | `crates/drafting-geometry/src/curve.rs:467` | 2d |  |
| `impl Extent::from_points` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:74` | - |  |
| `impl CubicParams::from_points_derivs` | 5 | `crates/vector-compute-shaders/src/cpu/euler.rs:75` | physics, 2d | Compute parameters from endpoints and derivatives. |
| `impl Complex32::from_polar` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:24` | - |  |
| `from_poly2` | 2 | `crates/vector-compute-shaders/src/cpu/draw_leaf.rs:240` | 2d |  |
| `impl Gamut::from_primaries` | 1 | `crates/color-transforms/src/spaces.rs:57` | - |  |
| `impl Gamut::from_primaries` | 1 | `crates/radiance-color-grading/src/spaces.rs:59` | - |  |
| `from_primavera_xer` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:556` | - |  |
| `impl Unit::from_pt` | 2 | `crates/vector-locus-geometry/src/units.rs:95` | - |  |
| `impl Tick::from_rational` | 3 | `crates/chronicle-timeline-interchange/src/time.rs:17` | - |  |
| `impl Tick::from_rational` | 3 | `crates/kinetics-motion-compositor/src/time.rs:13` | - |  |
| `impl Tick::from_rational` | 3 | `crates/timeline-editing/src/timebase.rs:16` | - | Convert a rational timestamp `pts * num / den` seconds. A zero timebase |
| `impl SliceType::from_raw` | 1 | `crates/video-h264-codec/src/slice.rs:51` | - |  |
| `impl AcoColor::from_record` | 2 | `crates/color-transforms/src/palette/aco.rs:45` | - |  |
| `impl ApvConfig::from_record` | 1 | `crates/media-isobmff/src/codec.rs:753` | - | Build from a raw `APVDecoderConfigurationRecord` (starting with `configurationVersion = 1`). |
| `impl GpuStrip::from_rect` | 4 | `crates/vector-hybrid-renderer/src/draw.rs:423` | 2d |  |
| `impl Signal::from_rgba8` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:176` | - |  |
| `impl Signal::from_rgba8_max` | 5 | `crates/radiance-color-grading/src/scopes/mod.rs:180` | - |  |
| `impl Matrix::from_row_major` | 3 | `crates/discourse-scientific-matlang/src/matrix.rs:101` | - |  |
| `impl BrushOperation::from_sample` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:93` | - |  |
| `impl BrushOperation::from_sample` | 1 | `crates/terrain-brushes/src/lib.rs:99` | - |  |
| `impl Timecode::from_samples` | 3 | `crates/audio-timebase/src/timecode.rs:191` | - | Derive timecode label from sample position. |
| `impl FeetFrames::from_samples` | 3 | `crates/audio-timebase/src/timecode.rs:260` | - | Derive FeetFrames from raw audio samples. |
| `impl TransportPosition::from_samples` | 5 | `crates/audio-timebase/src/transport.rs:69` | - | Calculate all multi-timebase coordinates from raw audio samples. |
| `impl Tick::from_seconds` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:34` | - |  |
| `impl Tick::from_seconds` | 1 | `crates/kinetics-motion-compositor/src/time.rs:30` | - |  |
| `impl StreamInstant::from_secs_f64` | 1 | `crates/audio-device-cpal/src/lib.rs:476` | - |  |
| `impl Umid::from_seed` | 1 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:21` | - |  |
| `impl AafDocument::from_sequence` | 1 | `crates/chronicle-timeline-interchange/src/interchange/aaf.rs:121` | - | Convert a TimelineSequence into an AAF Document model. |
| `impl ProfileClass::from_sig` | 1 | `crates/color-transforms/src/icc/profile.rs:71` | - |  |
| `impl ColorSpace::from_sig` | 1 | `crates/color-transforms/src/icc/profile.rs:115` | - |  |
| `impl MorphologyKernel::from_slice` | 3 | `crates/raster-selection-morphology/src/grayscale.rs:137` | - | Custom structuring element from row-major slice. |
| `impl Arc::from_start_center_end` | 3 | `crates/drafting-geometry/src/curve.rs:100` | 2d | Arc from start point, center and end point (CCW). |
| `impl HumanAnatomyProfile::from_str_and_week` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:49` | - |  |
| `impl BehaviorMacro::from_str_name` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:76` | - |  |
| `impl OtFeatures::from_tags` | 1 | `crates/vector-glyph-shaping/src/features.rs:55` | - | Parse a feature list like `["dlig", "smcp", "-liga"]`. |
| `impl CsrMatrix::from_triplets` | 3 | `crates/kinetics-continuum-multiphysics/src/solver/sparse.rs:29` | numerics | Construct CSR matrix from triplets (row, col, value). |
| `impl AseKind::from_u16` | 1 | `crates/color-transforms/src/palette/ase.rs:25` | - |  |
| `impl PsdVersion::from_u16` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:44` | - |  |
| `impl PsdColorMode::from_u16` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:67` | - |  |
| `impl PsdCompression::from_u16` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:104` | - |  |
| `impl Intent::from_u32` | 1 | `crates/color-transforms/src/icc/profile.rs:47` | - |  |
| `impl FileGdbShapeType::from_u32` | 1 | `crates/locus-geospatial-codecs/src/formats/filegdb.rs:27` | - |  |
| `impl XcfColorType::from_u32` | 1 | `crates/surface-layered-codecs/src/xcf_ora.rs:30` | - |  |
| `impl Compression::from_u8` | 1 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:16` | - |  |
| `impl TileType::from_u8` | 1 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:39` | - |  |
| `impl OperationalPattern::from_ul` | 1 | `crates/chronicle-broadcast-containers/src/mxf/partition.rs:80` | - |  |
| `impl Tick::from_units` | 2 | `crates/timeline-editing/src/timebase.rs:47` | - | Convert a count of units at `per_second`, such as audio samples, |
| `impl ImageFormat::from_wgpu` | 1 | `crates/vector-compute-renderer/src/recording.rs:253` | - |  |
| `fs_main` | 1 | `crates/gpu-command-profiler/examples/shader.wgsl:27` | - |  |
| `fs_main` | 1 | `crates/pixel-buffer-output/shaders/scale.wgsl:29` | 2d |  |
| `fs_main` | 1 | `crates/pixel-buffer-output/shaders/scale_fill.wgsl:30` | 2d |  |
| `impl Ellipse::full` | 3 | `crates/drafting-geometry/src/curve.rs:235` | 2d |  |
| `full` | 4 | `crates/media-isobmff/src/demux.rs:1047` | - |  |
| `full_continuum_multiphysics_smoke` | 0 | `crates/kinetics-continuum-multiphysics/src/lib.rs:51` | - |  |
| `full_discourse_pipeline_smoke` | 0 | `crates/discourse-academic-tex/src/lib.rs:32` | - |  |
| `full_document` | 0 | `crates/discourse-academic-tex/src/texparse/structure.rs:408` | - |  |
| `full_geomorphology_pipeline_integration` | 0 | `crates/terrain-erosion-solvers/src/lib.rs:65` | - |  |
| `impl Cur<'a>::full_header` | 1 | `crates/media-isobmff/src/bytes.rs:127` | - | Full-box header: (version, flags). |
| `full_history_accounting_is_bounded_and_explicit_discard_restores_commit_capacity` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:238` | - |  |
| `full_pathops_pipeline_smoke` | 0 | `crates/vector-locus-pathops/src/lib.rs:39` | - |  |
| `full_pipeline_round_trip` | 0 | `crates/audio-timebase/src/lib.rs:54` | - |  |
| `full_sample_copy_and_clamp` | 0 | `crates/video-h264-codec/src/inter.rs:440` | - |  |
| `full_shaders` | 3 | `crates/vector-compute-renderer/src/shaders.rs:48` | - |  |
| `full_typography_pipeline_smoke` | 0 | `crates/vector-glyph-shaping/src/lib.rs:41` | - |  |
| `fv` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:57` | - |  |
| `fv_calc` | 5 | `crates/chronicle-tabular-engine/src/functions/financial.rs:21` | - |  |
| `g` | 4 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:420` | - |  |
| `gain` | 1 | `crates/audio-aac-codec/src/encoder/quant.rs:89` | - |  |
| `gain34` | 1 | `crates/audio-aac-codec/src/encoder/quant.rs:85` | - |  |
| `impl OpusHead::gain_linear` | 1 | `crates/audio-opus-codec/src/header.rs:127` | numerics | Linear output gain factor. |
| `impl AssetHost<'_>::gallery` | 2 | `crates/asset-production-vibe/src/preview.rs:12` | - |  |
| `gamma_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:177` | - |  |
| `gamma_ln` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:156` | - |  |
| `impl ColorSpace::gamut` | 1 | `crates/color-transforms/src/spaces.rs:242` | - |  |
| `impl ColorSpace::gamut` | 1 | `crates/radiance-color-grading/src/spaces.rs:297` | - |  |
| `impl WorkingSpace::gamut` | 1 | `crates/radiance-color-grading/src/spaces.rs:354` | - |  |
| `gamut_compress` | 1 | `crates/color-transforms/src/transform.rs:166` | 3d | Compress out-of-gamut colours towards the achromatic axis (see module docs). Used on input, |
| `gamut_compression` | 0 | `crates/color-transforms/src/transform.rs:435` | - |  |
| `gamut_matrix` | 2 | `crates/color-transforms/src/spaces.rs:114` | numerics | Linear RGB in `src` → linear RGB in `dst`. |
| `gamut_matrix` | 2 | `crates/radiance-color-grading/src/spaces.rs:123` | numerics | Linear RGB in `src` -> linear RGB in `dst`. |
| `gamut_within` | 2 | `crates/color-transforms/src/transform.rs:265` | - | Whether gamut `a` lies inside gamut `b` (so no compression is needed going a → b). |
| `gauss_hex_2x2x2` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:196` | - | 2x2x2 Gauss-Legendre quadrature points for hexahedra in [-1, 1]^3. |
| `gauss_quad_2x2` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:185` | - | 2x2 Gauss-Legendre quadrature points for quadrilaterals in [-1, 1]^2. |
| `gauss_tet_1pt` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:211` | numerics | 1-point quadrature for linear tetrahedron. |
| `gaussian_blur_rgba32f` | 5 | `crates/raster-spatial-operators/src/convolve.rs:239` | - | Separable 2D Gaussian blur for RGBA32F buffer. |
| `gaussian_blur_rgba8` | 5 | `crates/raster-spatial-operators/src/convolve.rs:178` | 2d | Separable 2D Gaussian blur for RGBA8 buffer. |
| `gaussian_kernel_1d` | 2 | `crates/raster-spatial-operators/src/convolve.rs:64` | - | Create a 1D Gaussian kernel with radius and standard deviation sigma. |
| `gcd` | 2 | `crates/audio-opus-codec/src/silk/resampler.rs:27` | - |  |
| `gcd` | 2 | `crates/media-isobmff/tests/oracle_demux.rs:121` | - |  |
| `gcd` | 2 | `crates/timeline-editing/src/timebase.rs:218` | - |  |
| `impl NumberFormat::general` | 0 | `crates/chronicle-tabular-engine/src/numfmt/mod.rs:38` | - |  |
| `impl Bibliography::generate` | 2 | `crates/layout-paged-typesetter/src/citations/csl.rs:106` | - |  |
| `generate` | 2 | `crates/media-test-support/src/fixtures.rs:52` | - | Return `out` if it already exists (non-empty); otherwise run `make(tmp)` on a [`temp_path`] |
| `generate` | 4 | `crates/vector-locus-pathops/src/pathops/fit.rs:122` | 2d |  |
| `generate` | 4 | `crates/vector-radiance-trace/src/fit.rs:509` | 2d |  |
| `generate_and_report` | 3 | `crates/media-test-support/src/fixtures.rs:88` | - | Run a fixture generator and [`report`] it: `cached` when every path in `outputs` existed |
| `generate_box_hex_mesh` | 5 | `crates/kinetics-continuum-multiphysics/src/mesh/hex.rs:22` | computational geometry | Generate regular mapped hexahedral grid of nx x ny x nz cells. |
| `generate_compiled_shaders_module` | 1 | `crates/vector-hybrid-shaders/build.rs:127` | 3d |  |
| `generate_craterfield` | 4 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:112` | - | Generate a multi-crater planetary field using deterministic cell distribution. |
| `generate_dabs` | 4 | `crates/raster-layer-mask-adapter/src/brush.rs:168` | - | Generates a list of dabs from discrete input points using brush spacing and dynamics. |
| `generate_depreciation_schedule` | 7 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:110` | - |  |
| `generate_fixtures` | 0 | `crates/media-isobmff/tests/oracle_demux.rs:278` | - | Pre-generate every fixture (`cargo xtask fixtures`). |
| `generate_gradient_lut_impl` | 3 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:276` | - |  |
| `generate_graticule` | 2 | `crates/geodesy-spatial-analysis/src/cartography.rs:321` | - | Generates geographic graticule lines (meridians and parallels) at specified degree step. |
| `generate_html_report` | 2 | `crates/discourse-pdf-compliance/src/a11y/report.rs:36` | - |  |
| `generate_is_cached_and_atomic` | 0 | `crates/media-test-support/src/fixtures.rs:124` | - |  |
| `impl GamutCheck::generate_mask` | 4 | `crates/color-transforms/src/icc/gamut.rs:86` | - | Evaluates a mask buffer: writes 255 for out-of-gamut pixels, 0 for in-gamut pixels. |
| `generate_materialx_document` | 3 | `crates/surface-pbr-studio/src/materialx.rs:68` | 3d | Generates a complete MaterialX 1.39 XML document connecting maps to a surface shader. |
| `generate_straight_stair` | 7 | `crates/manifold-bim-csg/src/parametric.rs:205` | - | Generates a straight stair flight with uniform risers and treads. |
| `generate_wgsl_shader_module` | 2 | `crates/vector-hybrid-shaders/build.rs:180` | 3d |  |
| `impl Oscillator::generative_waveform` | 3 | `crates/audio-device-cpal/examples/synth_tones.rs:59` | - |  |
| `generic_cmyk` | 0 | `crates/color-transforms/src/icc/builtin.rs:103` | - |  |
| `geodesic_buffer_point` | 3 | `crates/geodesy-spatial-analysis/src/geodesy.rs:603` | computational geometry | Buffers a single point by `distance_m` on the geodetic manifold, returning a polygon ring. |
| `geodesic_ring_area_m2` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:347` | computational geometry | Area of a closed polygon ring on the spherical earth (square metres, always positive). |
| `geodesic_ruler_measures_distances_and_supports_undo_redo` | 0 | `crates/drafting-geometry/src/tests.rs:327` | - |  |
| `impl AssetHost<'_>::geodesy_op` | 3 | `crates/asset-production-vibe/src/geodesy_ops.rs:302` | - |  |
| `geodesy_strata_holo_kriging_e64_objects` | 0 | `crates/asset-production-vibe/src/geodesy_ops.rs:873` | - |  |
| `impl AssetHost<'_>::geometry` | 3 | `crates/asset-production-vibe/src/geometry.rs:7` | - |  |
| `trait DrawTarget::geometry_shift` | 1 | `crates/vector-hybrid-renderer/src/target.rs:285` | - | A positional shift that needs to be applied to all geometry when rendering to this target. |
| `impl DrawTarget for RootTarget::geometry_shift` | 1 | `crates/vector-hybrid-renderer/src/target.rs:293` | - |  |
| `impl DrawTarget for LayerTextureRegion::geometry_shift` | 1 | `crates/vector-hybrid-renderer/src/target.rs:303` | - |  |
| `impl AssetWorkspace::get` | 2 | `crates/asset-production-vibe/src/lib.rs:140` | - |  |
| `impl Set::get` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:81` | - |  |
| `impl Metadata::get` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:121` | - |  |
| `impl ChartOfAccounts::get` | 2 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:72` | - |  |
| `impl ZipPackage::get` | 2 | `crates/chronicle-office-codecs/src/zip/minizip.rs:66` | - |  |
| `impl Array::get` | 3 | `crates/chronicle-tabular-engine/src/core/value.rs:116` | - |  |
| `impl BibFile::get` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:42` | - | Entry lookup by key (exact match). |
| `impl Matrix::get` | 3 | `crates/discourse-scientific-matlang/src/matrix.rs:115` | - |  |
| `impl Body::get` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:133` | - |  |
| `impl DemGrid::get` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:36` | - |  |
| `impl GrayImage::get` | 3 | `crates/kinetics-motion-compositor/src/track/correlation.rs:32` | - |  |
| `impl ColorScheme::get` | 2 | `crates/layout-presentation-deck/src/model/types.rs:154` | - |  |
| `impl Metadata::get` | 2 | `crates/media-isobmff/src/demux.rs:195` | - |  |
| `impl Rgba8::get` | 3 | `crates/media-test-support/src/golden.rs:30` | - |  |
| `impl BlendIf::get` | 2 | `crates/raster-layer-mask-adapter/src/masks.rs:216` | - |  |
| `impl ScratchTexture<T>::get` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:108` | - |  |
| `impl WebGlTextureBindings::get` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:205` | - | Get a texture binding. |
| `impl TextureBindings::get` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:127` | - | Get a texture binding. |
| `impl Lru<K>::get` | 2 | `crates/video-h264-preview/src/movie.rs:98` | - |  |
| `impl CsiFrame::get_amplitudes` | 3 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:101` | - | Extracts amplitudes for a specific Tx-Rx antenna pair across all subcarriers. |
| `get_audio_clock` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:937` | - | Get the audio clock used to produce `StreamInstant`s. |
| `get_audio_record_min_buffer_size` | 3 | `crates/audio-device-cpal/src/host/oboe/android_media.rs:55` | - |  |
| `get_audio_track_min_buffer_size` | 3 | `crates/audio-device-cpal/src/host/oboe/android_media.rs:46` | - |  |
| `get_available_frames` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:308` | - |  |
| `get_blend_uniforms` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2173` | - |  |
| `impl BindMap::get_buf` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:928` | - |  |
| `impl ResourcePool::get_buf` | 5 | `crates/vector-compute-renderer/src/wgpu_engine.rs:976` | - | Get a buffer from the pool or create one. |
| `impl ChartOfAccounts::get_by_code` | 2 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:76` | - |  |
| `impl ChartOfAccounts::get_by_name` | 2 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:80` | - |  |
| `get_client` | 2 | `crates/audio-device-cpal/src/host/jack/mod.rs:143` | - |  |
| `get_client_options` | 1 | `crates/audio-device-cpal/src/host/jack/mod.rs:134` | - |  |
| `impl glifo::GlyphRenderer for Scene::get_context_color` | 1 | `crates/vector-hybrid-renderer/src/text.rs:344` | - |  |
| `get_copy_uniforms` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2184` | - |  |
| `impl BindMap::get_cpu_buf` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:906` | - | Get a CPU buffer. |
| `get_descriptors` | 0 | `crates/surface-layered-codecs/src/ontology.rs:135` | - |  |
| `impl WgpuEngine::get_download` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:779` | - |  |
| `get_enumerator` | 0 | `crates/audio-device-cpal/src/host/wasapi/device.rs:838` | - |  |
| `get_fabric_by_id` | 1 | `crates/kinetics-garment-draping/src/fabrics.rs:110` | - |  |
| `get_filter_pass_uniforms` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2157` | - |  |
| `impl BindMap::get_gpu_buf` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:896` | - | Get a buffer, only if it's on GPU. |
| `get_imports` | 1 | `crates/vector-compute-shaders/src/compile/preprocess.rs:8` | 3d |  |
| `get_io_buffer_frame_size_range` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:928` | - |  |
| `get_max_texture_dimension_2d` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:109` | - | Query the WebGL context for the max texture size. |
| `get_min_buffer_size` | 4 | `crates/audio-device-cpal/src/host/oboe/android_media.rs:23` | - |  |
| `get_morphism_specs` | 0 | `crates/surface-layered-codecs/src/ontology.rs:93` | - |  |
| `impl DeviceHandles::get_mut` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:222` | - | Get a mutable reference to the `alsa::PCM` handle for a specific `stream_type`. |
| `impl Array::get_mut` | 3 | `crates/chronicle-tabular-engine/src/core/value.rs:124` | - |  |
| `impl SkiaScenePainter<'_>::get_or_cache_font` | 5 | `crates/vector-skia-render-adapter/src/scene.rs:248` | - |  |
| `impl SkiaScenePainter<'_>::get_or_cache_normalized_typeface` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:291` | - |  |
| `impl SkiaScenePainter<'_>::get_or_cache_typeface` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:363` | - |  |
| `impl BindMap::get_or_create_image` | 3 | `crates/vector-compute-renderer/src/wgpu_engine.rs:932` | - |  |
| `impl Device::get_or_create_input_stream` | 3 | `crates/audio-device-cpal/src/host/asio/stream.rs:507` | - | Create a new CPAL Input Stream. |
| `impl Device::get_or_create_output_stream` | 3 | `crates/audio-device-cpal/src/host/asio/stream.rs:554` | - | Create a new CPAL Output Stream. |
| `impl GradientRampCache::get_or_create_ramp` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:75` | - |  |
| `impl DemGrid::get_or_edge` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:50` | graph |  |
| `impl ExternalTextureBindings::get_or_insert` | 2 | `crates/vector-hybrid-renderer/src/draw.rs:466` | - | Return the existing slot for `texture_source`, or insert it into the first empty slot. |
| `impl ImageCache::get_or_register` | 3 | `crates/vector-cpu-render-adapter/src/image_cache.rs:66` | - | Look up the converted image for `image`, converting and registering it |
| `impl TextureParity::get_parity` | 1 | `crates/vector-hybrid-renderer/src/target.rs:52` | - |  |
| `impl CsiFrame::get_phases` | 3 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:112` | - | Extracts raw phases (radians) for a specific Tx-Rx antenna pair. |
| `get_preset_by_name` | 1 | `crates/surface-pbr-studio/src/material.rs:190` | - |  |
| `get_pulses` | 1 | `crates/audio-opus-codec/src/celt/rate.rs:15` | - | Number of pulses represented by pseudo-pulse index `i`. |
| `impl ZipPackage::get_str` | 2 | `crates/chronicle-office-codecs/src/zip/minizip.rs:71` | - |  |
| `get_strip_uniforms` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2096` | - | Get the  uniform locations for the `render_strips` program. |
| `trait Block::get_x` | 2 | `crates/kinetics-block-simulation/src/block.rs:85` | - |  |
| `impl Block for Integrator::get_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:55` | - |  |
| `impl Block for PidController::get_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:190` | - |  |
| `impl Block for TransferFcn::get_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:285` | - |  |
| `impl Ttf::gid` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:131` | - | Unicode scalar -> glyph ID (0 = .notdef). |
| `impl WebGlRenderer::gl_context` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:712` | - | Get a reference to the underlying WebGL context. |
| `impl MaterialPreset::glass` | 3 | `crates/surface-pbr-studio/src/material.rs:116` | - |  |
| `global_snippets_path` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:52` | - | Global snippets path: `$ACADEMIC_TEX_HOME/snippets.json`, else `~/.academic-tex/snippets.json`. |
| `glyph` | 1 | `crates/locus-parametric-solver/src/typography/stroke.rs:14` | - |  |
| `glyph_advance` | 1 | `crates/asset-production-vibe/src/glyph/mod.rs:55` | - | Proportional advance width in normalized em units (1.0 = em height). |
| `glyph_atlas_resources_are_lazy` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:1000` | - |  |
| `impl AssetHost<'_>::glyph_layout_diegetic_op` | 3 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:13` | - |  |
| `glyph_measure_bounds_kerning_and_wrap` | 0 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:577` | - |  |
| `impl AssetHost<'_>::glyph_op` | 3 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:213` | semantics | Dispatches glyph operations admitted by the category-theoretic ontology. |
| `glyph_project_svg_two_produces_match` | 0 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:620` | - |  |
| `glyph_rasterize_rgba_non_empty_and_two_produces_match` | 0 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:659` | 2d |  |
| `impl Scene::glyph_run` | 3 | `crates/vector-hybrid-renderer/src/scene.rs:614` | - | Creates a builder for drawing a run of glyphs that have the same attributes. |
| `impl DrawGlyphs<'a>::glyph_transform` | 2 | `crates/vector-compute-renderer/src/scene.rs:517` | physics | Sets the per-glyph transform. This is applied to all glyphs prior to |
| `gmsh_format_headers` | 0 | `crates/kinetics-continuum-multiphysics/src/interchange/gmsh.rs:45` | - |  |
| `gpu_blur` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:713` | - |  |
| `gpu_flood` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:709` | - |  |
| `gpu_offset` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:705` | - |  |
| `gpu_shadow` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:717` | - |  |
| `gpu_strip` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:655` | - |  |
| `impl AssetHost<'_>::grade_cdl_scopes` | 2 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:124` | - | 129: `hypermedia.radiance.grade_cdl_scopes` |
| `gradient_shader_cs_from_cs_tag` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1170` | 3d |  |
| `gradient_shader_hue_method_from_hue_direction` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1190` | 3d |  |
| `impl AssetHost<'_>::graphics` | 3 | `crates/asset-production-vibe/src/graphics.rs:112` | - |  |
| `gray` | 2 | `crates/color-transforms/src/icc/builtin.rs:159` | - |  |
| `impl Planes::gray` | 2 | `crates/video-h264-codec/src/picture.rs:21` | - |  |
| `grayscale_dilate` | 4 | `crates/raster-selection-morphology/src/grayscale.rs:159` | - | Grayscale morphological dilation: `out(x, y) = max (input + kernel)`. |
| `grayscale_erode` | 4 | `crates/raster-selection-morphology/src/grayscale.rs:193` | - | Grayscale morphological erosion: `out(x, y) = min (input - kernel)`. |
| `grayscale_morphology_methods_behave_as_expected` | 0 | `crates/raster-selection-morphology/src/tests.rs:215` | - |  |
| `greek_names_resolve` | 0 | `crates/discourse-academic-tex/src/typeset.rs:534` | - |  |
| `grey` | 1 | `crates/color-transforms/src/grade.rs:125` | - |  |
| `grid` | 1 | `crates/terrain-brushes/src/adaptation_tests.rs:73` | - |  |
| `grid` | 0 | `crates/vector-locus-geometry/src/snap.rs:52` | - |  |
| `grid_place` | 2 | `crates/layout-box-autolayout/src/grid.rs:20` | - |  |
| `grid_snapping_consistency` | 0 | `crates/audio-timebase/src/lib.rs:77` | - |  |
| `grids` | 3 | `crates/radiance-color-grading/src/scopes/paint.rs:15` | - |  |
| `grids` | 0 | `crates/vector-locus-geometry/src/shapes.rs:336` | - |  |
| `impl BlendMode::group` | 1 | `crates/raster-blend-operators/src/lib.rs:144` | - | Index of the separator group (0..=5). |
| `impl IcsInfo::group_start` | 2 | `crates/audio-aac-codec/src/ics.rs:65` | - | First window of group `g`. |
| `group_text` | 4 | `crates/discourse-academic-tex/src/texparse/structure.rs:335` | - |  |
| `grouped` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:472` | - |  |
| `grouping_roundtrip` | 0 | `crates/audio-aac-codec/src/ics.rs:130` | - |  |
| `h` | 0 | `crates/vector-locus-geometry/src/projective.rs:124` | - |  |
| `h264_mp4` | 2 | `crates/video-h264-preview/src/synth.rs:159` | - | 64x48 H.264 MP4 of `n` solid frames at 24 fps, with an IDR picture every `gop` frames. |
| `h264_mp4_with` | 6 | `crates/video-h264-preview/src/synth.rs:122` | - | Build an H.264 MP4 with customized macroblock dimensions, frame count, and color generator. |
| `impl Movie::h264_to_frame` | 3 | `crates/video-h264-preview/src/movie.rs:446` | - |  |
| `haar1` | 3 | `crates/audio-opus-codec/src/celt/bands.rs:134` | - |  |
| `haar_is_involution` | 0 | `crates/audio-opus-codec/src/celt/bands.rs:953` | - |  |
| `half_sample_constant_is_constant` | 0 | `crates/video-h264-codec/src/inter.rs:452` | - |  |
| `half_sample_horizontal_ramp` | 0 | `crates/video-h264-codec/src/inter.rs:467` | - |  |
| `hampel_filter` | 3 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:188` | - | Hampel outlier removal filter: replaces outliers with local window median. |
| `handle_dropped_frames_gracefully` | 0 | `crates/gpu-command-profiler/tests/src/dropped_frame_handling.rs:7` | - |  |
| `impl Decoder::handle_nal` | 3 | `crates/video-h264-codec/src/decoder.rs:285` | - |  |
| `impl Decoder::handle_slice` | 4 | `crates/video-h264-codec/src/decoder.rs:324` | - |  |
| `handler_of` | 1 | `crates/media-isobmff/src/mux.rs:76` | - |  |
| `handmade` | 1 | `crates/media-isobmff/src/demux.rs:1055` | - | Hand-built file: uuid/wide/skip boxes, stz2 sizes, co64, ctts v1 with negative offsets, |
| `handmade_sample_table` | 0 | `crates/media-isobmff/src/demux.rs:1194` | - |  |
| `hard_light` | 2 | `crates/raster-blend-operators/src/lib.rs:459` | - |  |
| `hard_light` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:48` | 3d |  |
| `hard_mix_ps` | 2 | `crates/raster-blend-operators/src/migration_oracle.rs:59` | - |  |
| `hard_mix_u8` | 2 | `crates/raster-blend-operators/src/lib.rs:57` | - | Byte-quantized Hard Mix operator in `0..=255`. |
| `hard_mix_uses_generic_vivid_light_edge_rules` | 0 | `crates/raster-blend-operators/src/lib.rs:525` | graph |  |
| `impl GradientRampCache::has_changed` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:116` | - | Check if the luts data has changed. |
| `trait Block::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/block.rs:77` | - |  |
| `impl Block for Integrator::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:63` | - |  |
| `impl Block for Derivative::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:119` | - |  |
| `impl Block for PidController::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:198` | - |  |
| `impl Block for TransferFcn::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:293` | - |  |
| `impl Block for TransportDelay::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:356` | - |  |
| `impl Block for UnitDelay::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:56` | - |  |
| `impl Block for ZeroOrderHold::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:111` | - |  |
| `impl Block for Scope::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:59` | - |  |
| `impl Block for ToWorkspace::has_feedthrough` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:106` | - |  |
| `impl Anchor::has_in` | 1 | `crates/vector-locus-geometry/src/path.rs:127` | - |  |
| `has_later_end` | 3 | `crates/discourse-academic-tex/src/review/ai_repair.rs:204` | - |  |
| `impl SliceHeader::has_mmco5` | 1 | `crates/video-h264-codec/src/slice.rs:149` | - |  |
| `impl Anchor::has_out` | 1 | `crates/vector-locus-geometry/src/path.rs:130` | - |  |
| `impl InputTransform::has_pixel_stage` | 1 | `crates/color-transforms/src/transform.rs:231` | 2d | Whether anything happens after the per-channel table. |
| `impl AudioCaps::has_tuning` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:121` | - |  |
| `impl Hash for Device::hash` | 2 | `crates/audio-device-cpal/src/host/asio/device.rs:47` | - |  |
| `impl Hash for Device::hash` | 2 | `crates/audio-device-cpal/src/host/jack/device.rs:265` | - |  |
| `impl Hash for NormalizedTypefaceCacheKey::hash` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:89` | - |  |
| `impl Hash for NormalizedTypefaceCacheKeyBorrowed<'a>::hash` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:105` | - |  |
| `impl Hash for FontCacheKeyBorrowed<'a>::hash` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:131` | - |  |
| `impl Hash for FontCacheKey::hash` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:152` | - |  |
| `hash128` | 1 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:32` | - |  |
| `hash2d` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:377` | - |  |
| `hash_coord` | 3 | `crates/kinetics-motion-compositor/src/typography/wiggly.rs:65` | - |  |
| `hat_weight` | 1 | `crates/radiance-camera-raw/src/fusion.rs:25` | - | Hat weighting function: favors midtones, drops to 0 at extremes (over/underexposure). |
| `haversine_km` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:44` | - | Great-circle distance between two (latitude, longitude) pairs in degrees. |
| `haversine_m` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:163` | - | Great-circle Haversine distance in metres between two points in degrees (lon, lat). |
| `hd` | 8 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:57` | - |  |
| `hdr` | 1 | `crates/color-transforms/src/transform.rs:365` | - |  |
| `hdri_operators_produce_expected_values` | 0 | `crates/raster-blend-operators/src/lib.rs:552` | - |  |
| `he_aac_asc_parses_core` | 0 | `crates/audio-aac-codec/src/config.rs:409` | - |  |
| `impl Decoder::head` | 1 | `crates/audio-opus-codec/src/multistream.rs:45` | - |  |
| `impl Codec::header_count` | 1 | `crates/media-ogg/src/lib.rs:140` | - | Header packets before the audio packets. |
| `header_forms` | 0 | `crates/media-isobmff/src/bytes.rs:320` | - |  |
| `impl SliceData::header_size` | 1 | `crates/video-prores-codec/src/encode.rs:283` | - |  |
| `header_size_for` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:206` | - |  |
| `header_sizes` | 0 | `crates/media-test-support/src/wav.rs:40` | - |  |
| `impl RuleKind::header_slot` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:57` | - |  |
| `impl ParaDoc::heading` | 2 | `crates/layout-paged-typesetter/src/pagination/flow.rs:45` | - |  |
| `impl RangeRef::height` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:165` | - |  |
| `impl Bounds2::height` | 1 | `crates/drafting-geometry/src/bounds.rs:52` | - |  |
| `impl Extent::height` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:91` | - |  |
| `impl MathLayout::height` | 1 | `crates/layout-paged-typesetter/src/equation/layout.rs:37` | - |  |
| `impl PsdLayer::height` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:294` | - |  |
| `impl Cell::height` | 1 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:106` | - |  |
| `trait WebGlAtlasWriter::height` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3310` | - | Get the height of the image. |
| `impl WebGlAtlasWriter for Pixmap::height` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3329` | - |  |
| `impl WebGlAtlasWriter for Arc<Pixmap>::height` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3372` | - |  |
| `impl WebGlAtlasWriter for WebGlTexture::height` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3398` | - |  |
| `impl WebGlAtlasWriter for WebGlTextureWithDimensions::height` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3448` | - |  |
| `trait AtlasWriter::height` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3290` | - | Get the height of the image. |
| `impl AtlasWriter for Texture::height` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3311` | - |  |
| `impl AtlasWriter for Pixmap::height` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3357` | - |  |
| `impl AtlasWriter for Arc<Pixmap>::height` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3405` | - |  |
| `impl Scene::height` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:878` | - | Get the height of the render context. |
| `impl DrawSink for Scene::height` | 1 | `crates/vector-hybrid-renderer/src/text.rs:212` | - |  |
| `impl Sps::height` | 1 | `crates/video-h264-codec/src/params.rs:457` | - |  |
| `hemisphere_sample` | 4 | `crates/surface-pbr-studio/src/bake.rs:243` | 3d |  |
| `impl SampleEntry::hevc` | 3 | `crates/media-isobmff/src/codec.rs:1277` | - | HEVC entry (`hvc1`, parameter sets in the config only). |
| `hex` | 0 | `crates/color-transforms/src/lib.rs:487` | - |  |
| `hex_bytes` | 1 | `crates/asset-production-vibe/src/ontology/mod.rs:251` | - |  |
| `hex_encode` | 1 | `crates/discourse-pdf-compliance/src/sign/digest.rs:118` | - |  |
| `hex_mesh_node_and_element_counts` | 0 | `crates/kinetics-continuum-multiphysics/src/mesh/hex.rs:79` | computational geometry, graph |  |
| `hex_scaled_jacobian` | 1 | `crates/kinetics-continuum-multiphysics/src/mesh/quality.rs:8` | - | Scaled Jacobian metric for a hexahedral element: in [-1, 1], where 1 is an ideal cube. |
| `hex_to_tets` | 1 | `crates/kinetics-continuum-multiphysics/src/mesh/tet.rs:41` | computational geometry | Generate 5 or 6 tetrahedra from a hexahedral bounding box. |
| `hilbert_xy2d` | 3 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:230` | - |  |
| `impl DrawGlyphs<'a>::hint` | 2 | `crates/vector-compute-renderer/src/scene.rs:555` | - | Sets whether to enable hinting. |
| `histogram` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:369` | - |  |
| `impl GenerationalCache<K, T>::hit` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:60` | - |  |
| `hlg_inverse_oetf` | 1 | `crates/color-transforms/src/lib.rs:230` | - |  |
| `hlg_inverse_oetf` | 1 | `crates/radiance-color-grading/src/matrix.rs:318` | - |  |
| `hlg_known_nit_values` | 0 | `crates/color-transforms/src/grade.rs:161` | - |  |
| `hlg_oetf` | 1 | `crates/color-transforms/src/lib.rs:226` | - |  |
| `hlg_oetf` | 1 | `crates/radiance-color-grading/src/matrix.rs:309` | - |  |
| `impl Keyframe::hold` | 1 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:118` | - |  |
| `impl PerspectiveGrid::homography` | 2 | `crates/vector-locus-geometry/src/perspective.rs:232` | - | The homography of `plane` at its current offset. |
| `impl PerspectiveGrid::homography_at` | 3 | `crates/vector-locus-geometry/src/perspective.rs:237` | 3d | The homography of the plane parallel to `plane` that lies `depth` points along its normal. |
| `impl Padding::horizontal` | 1 | `crates/layout-box-autolayout/src/types.rs:139` | - |  |
| `host` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:464` | - |  |
| `host` | 2 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:356` | - |  |
| `host` | 3 | `crates/asset-production-vibe/src/chronicle_ops.rs:825` | - |  |
| `host` | 2 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:888` | - |  |
| `host` | 4 | `crates/asset-production-vibe/src/glyph_layout_diegetic/diegetic_input.rs:423` | - |  |
| `host` | 2 | `crates/asset-production-vibe/src/graphics.rs:684` | - |  |
| `host` | 4 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:461` | - |  |
| `host` | 4 | `crates/asset-production-vibe/src/layout_ops/mod.rs:365` | - |  |
| `host` | 2 | `crates/asset-production-vibe/src/operations_catalog.rs:312` | - |  |
| `host` | 2 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:373` | - |  |
| `host` | 2 | `crates/asset-production-vibe/src/sound.rs:221` | - |  |
| `host_device_setup` | 0 | `crates/audio-device-cpal/examples/synth_tones.rs:115` | - |  |
| `host_time_to_stream_instant` | 1 | `crates/audio-device-cpal/src/host/coreaudio/mod.rs:71` | - |  |
| `hostile_font_bytes_error_not_panic` | 0 | `crates/discourse-academic-tex/src/texpdf.rs:502` | - |  |
| `hostile_input_does_not_panic` | 0 | `crates/drafting-geometry/src/region.rs:514` | - |  |
| `hostile_input_never_panics` | 0 | `crates/discourse-academic-tex/src/texparse/lex.rs:189` | - |  |
| `hostile_nested_box_and_sample_size_are_refused_before_decode` | 0 | `crates/video-h264-preview/src/lib.rs:770` | - |  |
| `hostile_never_panics` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:484` | - |  |
| `hostile_never_panics` | 0 | `crates/discourse-academic-tex/src/typeset.rs:546` | - |  |
| `hourglass_zero_for_rigid_translation` | 0 | `crates/kinetics-continuum-multiphysics/src/explicit/hourglass.rs:53` | - |  |
| `hsl_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:358` | - |  |
| `hsl_to_rgb` | 3 | `crates/color-transforms/src/lib.rs:279` | - |  |
| `hsl_to_rgb` | 3 | `crates/radiance-color-grading/src/matrix.rs:379` | - |  |
| `hsv_to_rgb` | 3 | `crates/surface-pbr-studio/src/bake.rs:264` | - |  |
| `htap_row` | 3 | `crates/video-h264-codec/src/inter.rs:77` | - | Horizontal 6-tap (unclipped) at every position of `row` for `n` outputs: out[c] uses row[c..c + 6]. |
| `html_escape` | 1 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:226` | - |  |
| `huge_circle_secants_remain_finite` | 0 | `crates/drafting-geometry/src/tests.rs:125` | - |  |
| `huge_levels_dequantise_without_overflow_panics` | 0 | `crates/video-h264-codec/src/transform.rs:296` | - | Corrupt streams can carry huge coefficient levels: dequantisation wraps (as release |
| `hvcc_roundtrip` | 0 | `crates/media-isobmff/src/codec.rs:1591` | - |  |
| `hydraulic_drop_is_symmetric_within_one_sample` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:54` | - |  |
| `hydraulic_erosion` | 4 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:140` | physics | Simulate hydraulic erosion over a mutable 2D heightfield. |
| `hydraulic_erosion_modifies_slope_deterministically` | 0 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:316` | - |  |
| `hyphen_points` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:107` | - | Allowed hyphenation points in `word` as char indices (hyphen inserted before char). |
| `impl Patterns::hyphenate` | 3 | `crates/layout-paged-typesetter/src/hyphenation/liang.rs:109` | - | Hyphenates a word, inserting hyphen strings. |
| `hyphenate_word` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:142` | - |  |
| `hyphenation_exceptions` | 0 | `crates/vector-glyph-shaping/src/hyphen.rs:54` | - |  |
| `hypot` | 2 | `crates/vector-compute-shaders/shader/fine.wgsl:724` | - |  |
| `impl SampleRate::hz` | 1 | `crates/audio-timebase/src/rate.rs:56` | - |  |
| `hz_to_midi` | 1 | `crates/acoustics-daw-session/src/dsp/pitch_detect.rs:88` | - | Converts a frequency in Hz to closest MIDI note number (A4 = 440 Hz = 69). |
| `impl Set::i` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:89` | - |  |
| `impl GroupReader<'r, 'a>::i` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:163` | - |  |
| `impl Cur<'a>::i16` | 1 | `crates/media-isobmff/src/bytes.rs:101` | - |  |
| `impl BoxBuf::i16` | 2 | `crates/media-isobmff/src/bytes.rs:286` | - |  |
| `i16_plane_flat_is_flat` | 0 | `crates/video-h264-codec/src/intra.rs:441` | - |  |
| `i16_types` | 0 | `crates/video-h264-codec/src/mbtypes.rs:217` | - |  |
| `impl Cur<'a>::i32` | 1 | `crates/media-isobmff/src/bytes.rs:111` | - |  |
| `impl BoxBuf::i32` | 2 | `crates/media-isobmff/src/bytes.rs:292` | - |  |
| `i4_diag_down_left_hand_computed` | 0 | `crates/video-h264-codec/src/intra.rs:410` | - |  |
| `i4_diag_down_right_hand_computed` | 0 | `crates/video-h264-codec/src/intra.rs:420` | - |  |
| `i4_top_right_substitution` | 0 | `crates/video-h264-codec/src/intra.rs:432` | - |  |
| `i4_vertical_horizontal_dc` | 0 | `crates/video-h264-codec/src/intra.rs:397` | - |  |
| `impl Cur<'a>::i64` | 1 | `crates/media-isobmff/src/bytes.rs:117` | - |  |
| `impl BoxBuf::i64` | 2 | `crates/media-isobmff/src/bytes.rs:298` | - |  |
| `impl RangeDecoder<'a>::icdf` | 3 | `crates/audio-opus-codec/src/range.rs:164` | - | Decodes a symbol with an "inverse CDF" table (`icdf[i] = ft - cdf[i+1]`, `ft = 1 << ftb`). |
| `impl RangeEncoder::icdf` | 4 | `crates/audio-opus-codec/src/range.rs:332` | - |  |
| `icdf` | 1 | `crates/audio-opus-codec/src/silk/tables.rs:7` | - | Converts a PDF (summing to 256) into an inverse CDF (`256 - cdf[i+1]`). |
| `ics` | 3 | `crates/audio-aac-codec/tests/decoder_oracle.rs:97` | - |  |
| `impl TimeFormat::id` | 1 | `crates/audio-timebase/src/transport.rs:39` | - |  |
| `impl Builtin::id` | 1 | `crates/color-transforms/src/icc/builtin.rs:47` | - |  |
| `impl ColorSpace::id` | 1 | `crates/color-transforms/src/spaces.rs:193` | - | Stable id used in commands and project files. |
| `impl WorkingSpace::id` | 1 | `crates/color-transforms/src/spaces.rs:310` | - |  |
| `impl ColorSpace::id` | 1 | `crates/radiance-color-grading/src/spaces.rs:247` | - |  |
| `impl PbrChannel::id` | 1 | `crates/surface-pbr-studio/src/material.rs:33` | - |  |
| `impl Plane::id` | 1 | `crates/vector-locus-geometry/src/perspective.rs:36` | - |  |
| `impl RefPic::id` | 1 | `crates/video-h264-codec/src/picture.rs:331` | - |  |
| `idct4_add` | 3 | `crates/video-h264-codec/src/transform.rs:95` | - | Inverse 4x4 transform of raster-order `d` and add to `dst` (8.5.12.2, 8.5.14). |
| `idct4_dc_add` | 3 | `crates/video-h264-codec/src/transform.rs:128` | - | DC-only shortcut: all AC coefficients zero. |
| `idct4_dc_only_matches_shortcut` | 0 | `crates/video-h264-codec/src/transform.rs:233` | - |  |
| `idct4_hand_example` | 0 | `crates/video-h264-codec/src/transform.rs:245` | - |  |
| `idct8_1d` | 1 | `crates/video-h264-codec/src/transform.rs:138` | - |  |
| `idct8_add` | 3 | `crates/video-h264-codec/src/transform.rs:160` | - | Inverse 8x8 transform of raster-order `d` and add to `dst` (8.5.13.2): rows, then columns, |
| `idct8_dc` | 0 | `crates/video-h264-codec/src/transform.rs:259` | - |  |
| `idct8x8` | 1 | `crates/chronicle-broadcast-containers/src/dnx/dct.rs:7` | - | In-place inverse DCT. `block` holds dequantised coefficients in raster order |
| `idct8x8` | 1 | `crates/video-prores-codec/src/dct.rs:8` | - | In-place inverse DCT. `block` holds dequantised coefficients in raster order |
| `idct_matches_double_reference` | 0 | `crates/video-prores-codec/src/dct.rs:116` | - |  |
| `impl Parser::ident` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:133` | - |  |
| `impl Lut1d::identity` | 1 | `crates/color-transforms/src/lut.rs:27` | - |  |
| `impl Lut3d::identity` | 1 | `crates/color-transforms/src/lut.rs:59` | - |  |
| `impl Lut1d::identity` | 1 | `crates/radiance-color-grading/src/lut.rs:17` | - |  |
| `impl Lut3d::identity` | 1 | `crates/radiance-color-grading/src/lut.rs:63` | - |  |
| `impl Transform::identity` | 0 | `crates/vector-compute-shaders/src/cpu/util.rs:140` | - |  |
| `impl Dpb<F>::idr` | 3 | `crates/video-h264-codec/src/dpb.rs:97` | - | IDR: mark all references unused; output (or drop) prior pictures. |
| `idr` | 1 | `crates/video-h264-preview/src/lib.rs:503` | - |  |
| `idr_slice` | 3 | `crates/video-h264-preview/src/lib.rs:525` | - |  |
| `idr_slice_bits` | 4 | `crates/video-h264-preview/src/lib.rs:526` | - |  |
| `ids_parse` | 0 | `crates/color-transforms/src/spaces.rs:408` | - |  |
| `ids_text` | 1 | `crates/locus-parametric-solver/src/solver/conflict.rs:37` | - |  |
| `impl LbmGrid3D::idx` | 4 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:94` | - |  |
| `ifc4_roundtrip_wall` | 1 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:665` | - |  |
| `impl MepCategory::ifc_class_and_type` | 1 | `crates/manifold-bim-csg/src/parametric.rs:264` | - |  |
| `ifc_export_and_guid` | 0 | `crates/manifold-bim-csg/src/ifc.rs:218` | - |  |
| `ifc_guid` | 2 | `crates/manifold-bim-csg/src/ifc.rs:29` | - | Generates a deterministic 22-character IFC GlobalId from a 64-bit integer seed. |
| `ilog` | 1 | `crates/audio-opus-codec/src/range.rs:20` | - | Number of bits needed to represent `x` (0 for 0). |
| `impl AssetHost<'_>::image` | 2 | `crates/asset-production-vibe/src/raster.rs:246` | - |  |
| `image` | 1 | `crates/vector-cpu-render-adapter/src/image_cache.rs:146` | - |  |
| `image` | 1 | `crates/vector-cpu-render-adapter/src/image_convert.rs:61` | - |  |
| `impl AssetHost<'_>::image_limits` | 1 | `crates/asset-production-vibe/src/graphics.rs:105` | - |  |
| `image_page_models` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:490` | - |  |
| `imdct_matches_direct_formula` | 0 | `crates/audio-opus-codec/src/celt/mdct.rs:181` | - | The folded output must equal the direct IMDCT formula folded by TDAC symmetry. |
| `immendpoint_from_immdevice` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:137` | - |  |
| `impl AssetHost<'_>::import_psd` | 2 | `crates/asset-production-vibe/src/radiance_photo/psd_codec.rs:193` | - |  |
| `impl AssetHost<'_>::import_rgba` | 2 | `crates/asset-production-vibe/src/graphics.rs:175` | - |  |
| `import_xfdf` | 2 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:32` | - | Import field values from an XML XFDF document into an AcroForm. |
| `in_bool` | 2 | `crates/kinetics-block-simulation/src/signal.rs:138` | - |  |
| `in_circle` | 4 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:37` | - | In-circle test: positive if p is inside the circumcircle of counter-clockwise (p0, p1, p2). |
| `impl Track::in_out` | 2 | `crates/vector-surface-brush/src/track.rs:97` | 3d, 2d | Incoming and outgoing unit directions at vertex `i`. |
| `in_real` | 2 | `crates/kinetics-block-simulation/src/signal.rs:130` | - |  |
| `incompatible_siblings` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:514` | - |  |
| `inconsistent_and_empty_dimensions_do_not_panic` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:73` | - |  |
| `inconsistent_composition_table_is_refused` | 0 | `crates/video-h264-preview/src/lib.rs:606` | - |  |
| `impl XmlWriter::indent` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:85` | - |  |
| `index_fn` | 1 | `crates/chronicle-tabular-engine/src/functions/lookup.rs:46` | - |  |
| `index_of` | 2 | `crates/audio-aac-codec/src/huffman.rs:164` | - | Codebook index of the (sign-stripped for unsigned books) values. |
| `impl Model::index_of` | 2 | `crates/kinetics-block-simulation/src/model.rs:86` | - |  |
| `indexed` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:643` | - |  |
| `impl LdltFactor::inertia` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/ldlt.rs:63` | - | Compute Sylvester's inertia from the diagonal entries of D. |
| `infer` | 2 | `crates/locus-parametric-solver/src/solver/auto.rs:106` | - |  |
| `impl Movie::info` | 1 | `crates/video-h264-preview/src/movie.rs:305` | - |  |
| `impl FilterPassPlan::init` | 3 | `crates/vector-hybrid-renderer/src/filter.rs:424` | - |  |
| `impl Enc::init` | 1 | `crates/video-h264-codec/src/synth_tests.rs:216` | - |  |
| `impl Mode::init_caps` | 3 | `crates/audio-opus-codec/src/celt/rate.rs:141` | - | Per-band allocation caps for this frame (`init_caps`). |
| `impl Cabac<'a>::init_contexts` | 3 | `crates/video-h264-codec/src/cabac.rs:88` | - |  |
| `impl Cabac<'a>::init_engine` | 1 | `crates/video-h264-codec/src/cabac.rs:100` | - | 9.3.1.2: codIRange = 510, codIOffset = read_bits(9) (from the current byte-aligned position). |
| `impl RendererContext<'_>::init_root_clear` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2593` | - |  |
| `init_tables` | 0 | `crates/video-h264-codec/src/cavlc.rs:88` | physics | Force table construction (so the first slice does not pay for it). |
| `impl Mode::initial` | 1 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:32` | - |  |
| `initialize_blend_vao` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2217` | - |  |
| `initialize_copy_vao` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2237` | - |  |
| `impl Host::initialize_default_devices` | 1 | `crates/audio-device-cpal/src/host/jack/mod.rs:65` | - |  |
| `initialize_filter_vao` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2197` | - |  |
| `initialize_packed_u32_attribs` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2253` | - |  |
| `initialize_strip_vao` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2594` | - | Initialize strip VAO. |
| `inline_math_skipped_but_counted` | 0 | `crates/discourse-academic-tex/src/texparse/stats.rs:158` | - |  |
| `inner` | 4 | `crates/discourse-academic-tex/src/typeset.rs:200` | - |  |
| `innermost_of_nested` | 0 | `crates/drafting-geometry/src/region.rs:435` | - |  |
| `input` | 2 | `crates/audio-aac-codec/tests/decoder_oracle.rs:15` | - |  |
| `impl FilterPassBindings::input` | 2 | `crates/vector-hybrid-renderer/src/target.rs:139` | - |  |
| `impl Host::input_device_with_name` | 2 | `crates/audio-device-cpal/src/host/jack/mod.rs:55` | - |  |
| `trait HostTrait::input_devices` | 1 | `crates/audio-device-cpal/src/traits.rs:61` | - | An iterator yielding all `Device`s currently available to the system that support one or more |
| `input_stream_worker` | 5 | `crates/audio-device-cpal/src/host/alsa/mod.rs:578` | - |  |
| `input_timestamp` | 2 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:577` | - | Produce the input stream timestamp. |
| `impl GamutCheck::inputs` | 1 | `crates/color-transforms/src/icc/gamut.rs:53` | - |  |
| `impl Transform::inputs` | 1 | `crates/color-transforms/src/icc/transform.rs:82` | - |  |
| `trait Block::inputs` | 1 | `crates/kinetics-block-simulation/src/block.rs:56` | - |  |
| `impl Block for Integrator::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:26` | - |  |
| `impl Block for Derivative::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:92` | - |  |
| `impl Block for PidController::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:156` | - |  |
| `impl Block for TransferFcn::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:232` | - |  |
| `impl Block for TransportDelay::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:323` | - |  |
| `impl Block for UnitDelay::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:29` | - |  |
| `impl Block for ZeroOrderHold::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:86` | - |  |
| `impl Block for Gain::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:26` | - |  |
| `impl Block for Sum::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:80` | - |  |
| `impl Block for Product::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:142` | - |  |
| `impl Block for Saturation::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:196` | - |  |
| `impl Block for DeadZone::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:242` | - |  |
| `impl Block for Switch::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:292` | - |  |
| `impl Block for Relational::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:342` | - |  |
| `impl Block for LogicalOperator::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:402` | - |  |
| `impl Block for Abs::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:461` | - |  |
| `impl Block for Mux::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:24` | - |  |
| `impl Block for Demux::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:69` | - |  |
| `impl Block for Scope::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:42` | - |  |
| `impl Block for ToWorkspace::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:89` | - |  |
| `impl Block for Constant::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:23` | - |  |
| `impl Block for Step::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:66` | - |  |
| `impl Block for Sine::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:112` | - |  |
| `impl Block for Ramp::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:156` | - |  |
| `impl Block for Clock::inputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:196` | - |  |
| `impl AssetWorkspace::insert` | 3 | `crates/asset-production-vibe/src/lib.rs:149` | - |  |
| `impl ZipPackage::insert` | 3 | `crates/chronicle-office-codecs/src/zip/minizip.rs:61` | - |  |
| `impl Graph::insert` | 3 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:51` | graph |  |
| `impl Env::insert` | 3 | `crates/locus-parametric-solver/src/solver/expr.rs:181` | - |  |
| `impl OctreeNode::insert` | 6 | `crates/raster-spatial-operators/src/octree.rs:38` | - |  |
| `impl OctreeQuantizer::insert` | 5 | `crates/raster-spatial-operators/src/octree.rs:79` | - |  |
| `impl WebGlTextureBindings::insert` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:199` | numerics | Insert or replace a texture binding. |
| `impl TextureBindings::insert` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:121` | - | Insert or replace a texture binding. |
| `impl Index::insert` | 3 | `crates/vector-locus-pathops/src/three_d/visibility.rs:128` | 2d |  |
| `impl GenerationalCache<K, T>::insert` | 3 | `crates/vector-skia-render-adapter/src/cache.rs:72` | - |  |
| `impl SubPath::insert_anchor` | 3 | `crates/vector-locus-geometry/src/path.rs:252` | - | Split segment `seg` at parameter `t`, inserting a new anchor. Returns the new anchor index. |
| `insert_anchor_on_curve_keeps_shape` | 0 | `crates/vector-locus-geometry/src/path.rs:574` | - |  |
| `insert_anchor_on_line_keeps_shape` | 0 | `crates/vector-locus-geometry/src/path.rs:563` | - |  |
| `insert_before_edit` | 5 | `crates/discourse-academic-tex/src/review/ai_repair.rs:128` | - |  |
| `impl BindMap::insert_buf` | 3 | `crates/vector-compute-renderer/src/wgpu_engine.rs:885` | - |  |
| `insert_entries` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:297` | - |  |
| `insert_entry` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:306` | - |  |
| `impl BindMap::insert_image` | 4 | `crates/vector-compute-renderer/src/wgpu_engine.rs:924` | - |  |
| `insert_tone` | 4 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:474` | - |  |
| `impl AssetHost<'_>::inspect_engine_telemetry` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:98` | - |  |
| `impl ChangeDag::inspect_post_merge_conflicts` | 2 | `crates/kinetics-schedule-cpm/src/change_dag.rs:269` | - |  |
| `int` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:126` | - | Big-endian two's-complement signed integer of 0..=8 bytes. |
| `int` | 1 | `crates/media-isobmff/tests/common/mod.rs:166` | - |  |
| `int_of` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:173` | - |  |
| `integ_euler_10` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:165` | physics |  |
| `integ_euler_10` | 2 | `crates/vector-compute-shaders/src/cpu/euler.rs:258` | physics | Integrate Euler spiral. |
| `integer` | 2 | `crates/asset-production-vibe/src/lib.rs:511` | - |  |
| `impl EbuR128Meter::integrated_lufs` | 1 | `crates/acoustics-daw-session/src/dsp/meter.rs:99` | - | Computes the gated integrated loudness in LUFS (-70 LUFS absolute gate, -10 LU relative gate). |
| `integrated_lufs_bs1770` | 2 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:8` | - | ITU-R BS.1770-4 / EBU R128 gated integrated LUFS. |
| `intensity` | 3 | `crates/radiance-color-grading/src/scopes/paint.rs:7` | - |  |
| `impl Pic::inter` | 1 | `crates/video-h264-codec/src/synth_tests.rs:112` | - |  |
| `impl Grid::interior` | 1 | `crates/vector-radiance-trace/src/centerline.rs:166` | - |  |
| `interior_point` | 1 | `crates/vector-locus-pathops/src/pathops/planar.rs:12` | computational geometry | Find an interior point inside a closed simple polygon. |
| `interleave` | 2 | `crates/media-isobmff/tests/oracle_mux.rs:12` | - | All samples of all tracks in decode order interleaved by decode time: (track, sample index). |
| `interleave_hadamard` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:172` | - |  |
| `interleaved_scopes` | 0 | `crates/gpu-command-profiler/tests/src/interleaved_command_buffer.rs:8` | - |  |
| `impl Schedule::intermediate_texture_requirements` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:208` | - |  |
| `intermediate_texture_requirements_validate_limit` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:38` | - |  |
| `interp` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:820` | - |  |
| `interp_bits2pulses` | 15 | `crates/audio-opus-codec/src/celt/rate.rs:384` | - |  |
| `interpolate_at` | 3 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:411` | - |  |
| `interpolate_dual_illuminant_matrix` | 5 | `crates/radiance-camera-raw/src/demosaic.rs:90` | numerics | Dual-illuminant color matrix white balance. |
| `impl AssetHost<'_>::interpolate_keyframes` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:246` | - |  |
| `impl AssetHost<'_>::interpolate_motor_fti` | 2 | `crates/asset-production-vibe/src/kinetics_sim/motor_xpbd_space.rs:12` | physics, numerics | PGA MotorFTI Screw-Linear Interpolation (ScLERP) between physics ticks and render frames. |
| `impl GpxTracklog::interpolate_position` | 3 | `crates/radiance-camera-raw/src/metadata.rs:67` | - | Interpolates GPS location at `time` (seconds since epoch) with a maximum gap limit. |
| `impl SampleRange::intersect` | 2 | `crates/audio-timebase/src/rate.rs:111` | - |  |
| `intersect` | 3 | `crates/manifold-bim-csg/src/csg.rs:384` | - | Intersects two solids. |
| `impl Bvh::intersect` | 3 | `crates/surface-pbr-studio/src/bvh.rs:63` | computational geometry, 3d | Finds the closest ray-mesh intersection. |
| `intersect_aabb` | 3 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:52` | computational geometry, 3d | Ray-AABB intersection test returning entry and exit parameters `(t_min, t_max)`. |
| `intersect_segments` | 2 | `crates/drafting-geometry/src/intersect.rs:96` | 2d | Intersections between two segments (finite). |
| `intersect_segments_ext` | 3 | `crates/drafting-geometry/src/intersect.rs:102` | 2d, graph | Intersections; with `extend` true, lines are infinite and arcs are full circles |
| `intersect_triangle` | 3 | `crates/surface-pbr-studio/src/bvh.rs:155` | computational geometry, 3d |  |
| `impl RangeRef::intersection` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:201` | computational geometry |  |
| `impl RangeRef::intersects` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:197` | - |  |
| `impl Bounds2::intersects` | 2 | `crates/drafting-geometry/src/bounds.rs:73` | - |  |
| `impl Rect::intersects` | 2 | `crates/layout-paged-typesetter/src/pagination/types.rs:38` | 2d |  |
| `intersects_rect` | 2 | `crates/vector-locus-geometry/src/hit.rs:68` | 2d | Does the path intersect (or lie inside) the rect? |
| `impl wasm_bindgen::convert::IntoWasmAbi for BufferSize::into_abi` | 1 | `crates/audio-device-cpal/src/lib.rs:247` | - |  |
| `impl Recording::into_commands` | 1 | `crates/vector-compute-renderer/src/recording.rs:230` | - | Returns a [`Vec`] containing all the [`Command`]s in order. |
| `impl ViewFramebuffer::into_framebuffer` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1236` | - |  |
| `intra_mb_type` | 1 | `crates/video-h264-codec/src/mbtypes.rs:83` | - | Intra mb_type (I-slice numbering 0..=25). |
| `impl SliceDecoder<'a>::intra_mode_nb` | 4 | `crates/video-h264-codec/src/slicedec.rs:869` | - |  |
| `impl SliceDecoder<'a>::intra_nb` | 2 | `crates/video-h264-codec/src/slicedec.rs:406` | - | Neighbour MB usable for intra prediction (constrained_intra_pred aware). |
| `intrate` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:584` | - |  |
| `inv_mlfn` | 3 | `crates/geodesy-spatial-analysis/src/geodesy.rs:494` | - |  |
| `invalid` | 1 | `crates/media-isobmff/src/error.rs:36` | - |  |
| `invalid` | 1 | `crates/video-h264-codec/src/error.rs:22` | - |  |
| `invalid_and_unsupported_samples_preserve_preview_and_journal` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:110` | - |  |
| `invalid_assemblies_and_expanded_output_budget_fail_without_mutating_inputs` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:132` | - |  |
| `invalid_assemblies_and_expanded_output_budget_fail_without_mutating_inputs` | 0 | `crates/profile-authoring/src/tests.rs:234` | - |  |
| `invalid_pending_frame_count` | 0 | `crates/gpu-command-profiler/tests/src/errors.rs:6` | - |  |
| `invalid_rates_and_zero_denominators_do_not_panic` | 0 | `crates/timeline-editing/src/timebase.rs:254` | - |  |
| `impl NQuin48::invariant` | 3 | `crates/color-transforms/src/ontology.rs:27` | - |  |
| `impl NQuin48::invariant` | 3 | `crates/radiance-camera-raw/src/ontology.rs:27` | - |  |
| `impl NQuin48::invariant` | 3 | `crates/raster-layer-mask-adapter/src/ontology.rs:27` | - |  |
| `impl Mdct::inverse` | 3 | `crates/audio-aac-codec/src/mdct.rs:122` | - | Inverse MDCT (unscaled): `y[n] = Σ X[k] cos(π/M (n + ½ + M/2)(k + ½))`, `y` has length 2M. |
| `inverse` | 1 | `crates/color-transforms/src/spaces.rs:100` | - |  |
| `impl Matrix::inverse` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:359` | - |  |
| `impl Mat3::inverse` | 1 | `crates/drafting-geometry/src/mat.rs:76` | - |  |
| `impl Aeqd::inverse` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:388` | 3d | Inverse projection: planar metres -> lon/lat in degrees. |
| `impl Sinusoidal::inverse` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:455` | 3d | Inverse projection: projected metres -> lon/lat degrees. |
| `impl DMat2::inverse` | 1 | `crates/kinetics-garment-draping/src/math.rs:218` | - |  |
| `impl JournalAction::inverse` | 1 | `crates/radiance-camera-raw/src/metadata.rs:146` | - |  |
| `inverse` | 1 | `crates/radiance-color-grading/src/spaces.rs:109` | - |  |
| `impl Transform::inverse` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:151` | - |  |
| `impl Homography::inverse` | 1 | `crates/vector-locus-geometry/src/projective.rs:57` | - | Inverse. |
| `inverse32_varq` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:38` | - | `silk_INVERSE32_varQ`: approximation of `(1 << q_res) / b32`. |
| `inverse_approximations` | 0 | `crates/audio-opus-codec/src/silk/lpc.rs:365` | - |  |
| `inverse_matches_definition` | 0 | `crates/audio-aac-codec/src/mdct.rs:236` | - |  |
| `invert` | 1 | `crates/color-transforms/src/icc/math.rs:43` | - | Inverse, or `None` when (nearly) singular. |
| `impl Bsp::invert` | 1 | `crates/manifold-bim-csg/src/csg.rs:309` | - |  |
| `impl AscCdl::invert` | 3 | `crates/radiance-color-grading/src/grade.rs:166` | - | Exact inverse of ASC CDL (within clamped range). |
| `invert_table` | 2 | `crates/color-transforms/src/icc/curve.rs:151` | - |  |
| `impl AssetHost<'_>::invoke` | 3 | `crates/asset-production-vibe/src/lib.rs:208` | - |  |
| `impl AssetHost<'_>::invoke_inner` | 3 | `crates/asset-production-vibe/src/lib.rs:217` | - |  |
| `ipmt` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:93` | - |  |
| `ipmt_calc` | 6 | `crates/chronicle-tabular-engine/src/functions/financial.rs:37` | - |  |
| `irr` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:290` | - |  |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::is_active` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:146` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::is_active` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:250` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::is_active` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:255` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::is_active` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:156` | - |  |
| `is_audio_fourcc` | 1 | `crates/media-isobmff/src/codec.rs:868` | - |  |
| `impl MainAlign::is_auto_gap` | 1 | `crates/layout-box-autolayout/src/types.rs:35` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/alsa/mod.rs:41` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/asio/mod.rs:36` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:59` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:66` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:120` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/jack/mod.rs:107` | - | JACK is available if |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/null/mod.rs:108` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/oboe/mod.rs:60` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:34` | - |  |
| `impl HostTrait for Host::is_available` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:58` | - |  |
| `trait HostTrait::is_available` | 0 | `crates/audio-device-cpal/src/traits.rs:40` | - | Whether or not the host is available on the system. |
| `impl SliceType::is_b` | 1 | `crates/video-h264-codec/src/slice.rs:63` | - |  |
| `impl BitReader<'a>::is_byte_aligned` | 1 | `crates/bitstream-codec/src/lib.rs:54` | - |  |
| `impl BitWriter::is_byte_aligned` | 1 | `crates/bitstream-codec/src/lib.rs:219` | - |  |
| `impl Movie::is_cached` | 2 | `crates/video-h264-preview/src/movie.rs:330` | - |  |
| `impl DatePart::is_calendar` | 1 | `crates/chronicle-tabular-engine/src/numfmt/parse.rs:18` | - |  |
| `impl ArcPath::is_closed` | 1 | `crates/vector-locus-geometry/src/arc.rs:42` | - |  |
| `impl PathData::is_closed` | 1 | `crates/vector-locus-geometry/src/path.rs:465` | - |  |
| `is_collinear` | 3 | `crates/vector-locus-geometry/src/path.rs:172` | - |  |
| `impl PendingShaderProgram::is_complete` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1390` | 3d |  |
| `impl PendingWebGlPrograms::is_complete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1521` | - |  |
| `is_corner` | 1 | `crates/vector-locus-geometry/src/path.rs:53` | - |  |
| `impl BlendIf::is_default` | 1 | `crates/raster-layer-mask-adapter/src/masks.rs:212` | - |  |
| `impl ConstraintKind::is_dimensional` | 1 | `crates/locus-parametric-solver/src/solver/residual.rs:39` | - |  |
| `impl TimecodeRate::is_drop` | 1 | `crates/audio-timebase/src/timecode.rs:75` | - | Whether this is a drop-frame SMPTE timecode rate. |
| `impl FrameRate::is_drop_frame_compatible` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:100` | - |  |
| `impl Body::is_earth` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:137` | - |  |
| `is_east_asian` | 1 | `crates/layout-paged-typesetter/src/hyphenation/kinsoku.rs:15` | - | Determines if character `c` is East Asian text (CJK ideographs, kana, bopomofo, CJK punctuation). |
| `is_email_domain_char` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:201` | - |  |
| `is_email_local_char` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:197` | - |  |
| `impl SampleRange::is_empty` | 1 | `crates/audio-timebase/src/rate.rs:102` | - |  |
| `impl Value::is_empty` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:155` | - |  |
| `impl Bounds2::is_empty` | 1 | `crates/drafting-geometry/src/bounds.rs:31` | - |  |
| `impl Math::is_empty` | 1 | `crates/layout-paged-typesetter/src/equation/ast.rs:25` | - |  |
| `trait ByteSource::is_empty` | 1 | `crates/media-isobmff/src/source.rs:15` | - | True if the source holds no bytes. |
| `trait ByteSource::is_empty` | 1 | `crates/media-ogg/src/lib.rs:27` | - |  |
| `impl References::is_empty` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:110` | - |  |
| `impl Signal::is_empty` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:212` | - |  |
| `impl Bvh::is_empty` | 1 | `crates/surface-pbr-studio/src/bvh.rs:58` | - |  |
| `impl Bounds::is_empty` | 1 | `crates/surface-pbr-studio/src/math.rs:343` | - |  |
| `impl RasterImage::is_empty` | 1 | `crates/surface-pbr-studio/src/raster.rs:32` | - |  |
| `impl TimeRange::is_empty` | 1 | `crates/timeline-editing/src/timebase.rs:117` | - |  |
| `impl DebugLayers::is_empty` | 1 | `crates/vector-compute-renderer/src/debug.rs:98` | - | True if this `DebugLayers` has no layers enabled. |
| `impl OpaqueDraw::is_empty` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:98` | - |  |
| `impl FilterContext::is_empty` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:634` | - |  |
| `impl GradientRampCache::is_empty` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:111` | - | Check if the packed luts is empty. |
| `impl ArcPath::is_empty` | 1 | `crates/vector-locus-geometry/src/arc.rs:38` | - | True if no segment has any length. |
| `impl PathData::is_empty` | 1 | `crates/vector-locus-geometry/src/path.rs:360` | - |  |
| `impl Track::is_empty` | 1 | `crates/vector-surface-brush/src/track.rs:88` | - |  |
| `impl Value::is_error` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:159` | - |  |
| `is_file` | 1 | `crates/media-test-support/src/oracle.rs:34` | - |  |
| `impl Vec2::is_finite` | 1 | `crates/drafting-geometry/src/vec.rs:71` | - |  |
| `impl Vec3::is_finite` | 1 | `crates/drafting-geometry/src/vec.rs:174` | - |  |
| `impl Coord::is_finite` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:35` | - |  |
| `impl DVec2::is_finite` | 1 | `crates/kinetics-garment-draping/src/math.rs:51` | - |  |
| `impl DVec3::is_finite` | 1 | `crates/kinetics-garment-draping/src/math.rs:139` | - |  |
| `impl ScalingMatrices::is_flat` | 1 | `crates/video-h264-codec/src/params.rs:25` | - |  |
| `impl SampleFormat::is_float` | 1 | `crates/audio-device-cpal/src/samples_formats.rs:90` | - |  |
| `is_format_supported` | 2 | `crates/audio-device-cpal/src/host/wasapi/device.rs:150` | - |  |
| `impl Ellipse::is_full` | 1 | `crates/drafting-geometry/src/curve.rs:238` | - |  |
| `impl BlendRange::is_full` | 1 | `crates/raster-layer-mask-adapter/src/masks.rs:173` | - |  |
| `impl RangeRef::is_full_cols` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:185` | - |  |
| `impl RangeRef::is_full_rows` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:181` | - |  |
| `impl GradeSpace::is_hdr` | 1 | `crates/color-transforms/src/grade.rs:50` | - |  |
| `impl ColorSpace::is_hdr` | 1 | `crates/color-transforms/src/spaces.rs:276` | - | Carries more than SDR range (needs tone mapping into an SDR working space). |
| `impl WorkingSpace::is_hdr` | 1 | `crates/color-transforms/src/spaces.rs:329` | - |  |
| `impl GradeSpace::is_hdr` | 1 | `crates/radiance-color-grading/src/grade.rs:43` | - |  |
| `impl ColorSpace::is_hdr` | 1 | `crates/radiance-color-grading/src/spaces.rs:335` | - |  |
| `impl WorkingSpace::is_hdr` | 1 | `crates/radiance-color-grading/src/spaces.rs:361` | - |  |
| `impl Curve::is_identity` | 1 | `crates/color-transforms/src/icc/curve.rs:41` | - |  |
| `impl OutputTransform::is_identity` | 1 | `crates/color-transforms/src/transform.rs:311` | - |  |
| `impl Device::is_input` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:131` | - |  |
| `impl SampleFormat::is_int` | 1 | `crates/audio-device-cpal/src/samples_formats.rs:70` | - |  |
| `impl Frame::is_interlaced` | 1 | `crates/video-prores-codec/src/types.rs:195` | - |  |
| `impl MbKind::is_intra` | 1 | `crates/video-h264-codec/src/picture.rs:46` | - |  |
| `impl SliceType::is_intra` | 1 | `crates/video-h264-codec/src/slice.rs:60` | - |  |
| `impl OctreeNode::is_leaf` | 1 | `crates/raster-spatial-operators/src/octree.rs:34` | - |  |
| `is_leap` | 1 | `crates/chronicle-tabular-engine/src/core/date.rs:57` | - |  |
| `is_left` | 3 | `crates/vector-locus-pathops/src/pathops/boolean.rs:83` | - |  |
| `impl ColorSpace::is_log` | 1 | `crates/color-transforms/src/spaces.rs:279` | - |  |
| `impl ColorSpace::is_log` | 1 | `crates/radiance-color-grading/src/spaces.rs:339` | - |  |
| `is_math_env` | 1 | `crates/discourse-academic-tex/src/texparse/stats.rs:139` | - |  |
| `impl Mat3::is_mirroring` | 1 | `crates/drafting-geometry/src/mat.rs:98` | - |  |
| `impl Oscillator::is_multiple_of_freq_above_nyquist` | 2 | `crates/audio-device-cpal/examples/synth_tones.rs:50` | - |  |
| `impl BindType::is_mutable` | 1 | `crates/vector-compute-shaders/src/types.rs:23` | - |  |
| `impl Vec2::is_nan` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:123` | - |  |
| `is_new_picture` | 3 | `crates/video-h264-codec/src/slice.rs:465` | - | Is `sh` the first slice of a new primary coded picture relative to `prev` (7.4.1.2.4)? |
| `impl ViewFramebuffer::is_offscreen` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1232` | - |  |
| `impl Device::is_output` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:135` | - |  |
| `impl SliceType::is_p` | 1 | `crates/video-h264-codec/src/slice.rs:66` | - |  |
| `impl Pic::is_p` | 1 | `crates/video-h264-codec/src/synth_tests.rs:109` | - |  |
| `trait ProfilerCommandRecorder::is_pass` | 1 | `crates/gpu-command-profiler/src/profiler_command_recorder.rs:4` | - | Returns `true` if it's a pass or `false` if it's an encoder |
| `impl WindowRenderer for VelloWindowRenderer::is_pending` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:254` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::is_pending` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:259` | - |  |
| `impl Spline::is_periodic` | 1 | `crates/drafting-geometry/src/spline.rs:159` | - | Closed with an unclamped (periodic) knot vector, as DXF flags it (group 70 bit 2). |
| `impl ColorPipeline::is_plain` | 1 | `crates/color-transforms/src/spaces.rs:360` | numerics | The classic pipeline: linear BT.709, display-referred SDR. |
| `is_polygonal` | 4 | `crates/vector-locus-geometry/src/recognize.rs:307` | - |  |
| `impl PsdVersion::is_psb` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:33` | - |  |
| `impl Frame::is_published` | 2 | `crates/video-h264-codec/src/picture.rs:159` | - |  |
| `is_radian` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:100` | - |  |
| `impl AcroFormField::is_read_only` | 1 | `crates/discourse-pdf-compliance/src/forms/model.rs:75` | - |  |
| `is_rectangle` | 3 | `crates/vector-locus-geometry/src/recognize.rs:149` | - |  |
| `impl AcroFormField::is_required` | 1 | `crates/discourse-pdf-compliance/src/forms/model.rs:79` | - |  |
| `impl BidiRun::is_rtl` | 1 | `crates/vector-glyph-shaping/src/bidi.rs:64` | - |  |
| `impl Matrix::is_scalar` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:149` | - |  |
| `is_section` | 1 | `crates/discourse-academic-tex/src/texparse/stats.rs:135` | - |  |
| `impl BlendMode::is_separable` | 1 | `crates/raster-blend-operators/src/lib.rs:156` | - | Whether the mode works on each colour channel separately (all but Hue, Saturation, Color, Luminosity). |
| `impl Timeline::is_shape_visible` | 4 | `crates/layout-presentation-deck/src/timeline/timeline.rs:95` | - | Check if shape is visible at a given step and progress t. |
| `impl IcsInfo::is_short` | 1 | `crates/audio-aac-codec/src/ics.rs:49` | - |  |
| `impl RangeRef::is_single` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:177` | - |  |
| `impl Ul::is_smpte` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:12` | - | Every SMPTE label starts with the object identifier 06 0E 2B 34. |
| `is_straight` | 2 | `crates/vector-locus-pathops/src/pathops/fit.rs:190` | - | Is the cubic (within `tol`) a straight line from p0 to p3 with handles inside the chord? |
| `is_straight` | 2 | `crates/vector-radiance-trace/src/fit.rs:576` | - |  |
| `impl CowTerrainTileGrid::is_stroke_active` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:28` | - |  |
| `impl CowTerrainTileGrid::is_stroke_active` | 1 | `crates/terrain-brushes/src/lib.rs:209` | - |  |
| `is_struct` | 2 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:200` | - |  |
| `impl PhType::is_title` | 1 | `crates/layout-presentation-deck/src/model/types.rs:41` | - |  |
| `is_top_level` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ids.rs:28` | - |  |
| `impl Value::is_truthy` | 1 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:35` | - |  |
| `impl SampleFormat::is_uint` | 1 | `crates/audio-device-cpal/src/samples_formats.rs:80` | - |  |
| `impl Rational::is_valid` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:54` | - |  |
| `impl NQuin48::is_valid` | 1 | `crates/chronicle-office-codecs/src/ontology.rs:29` | - |  |
| `impl CellRef::is_valid` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:29` | - |  |
| `impl Spline::is_valid` | 1 | `crates/drafting-geometry/src/spline.rs:163` | - |  |
| `impl NQuin48::is_valid` | 1 | `crates/layout-presentation-deck/src/ontology.rs:29` | - |  |
| `impl MepGeom::is_vertical` | 1 | `crates/manifold-bim-csg/src/parametric.rs:317` | - |  |
| `is_video_fourcc` | 1 | `crates/media-isobmff/src/codec.rs:838` | - |  |
| `is_vowel` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:91` | - |  |
| `is_webaudio_available` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:403` | - |  |
| `is_webaudio_available` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:495` | - |  |
| `is_white` | 1 | `crates/vector-radiance-trace/src/lib.rs:321` | - |  |
| `is_word_char` | 1 | `crates/chronicle-tabular-engine/src/formula/lexer.rs:49` | - |  |
| `is_zero` | 1 | `crates/vector-locus-geometry/src/perspective.rs:60` | - |  |
| `island_from_separate_lines` | 0 | `crates/drafting-geometry/src/region.rs:448` | - |  |
| `island_outline_ignores_dangling_ends_and_inner_edges` | 0 | `crates/drafting-geometry/src/region.rs:470` | - |  |
| `islands` | 3 | `crates/drafting-geometry/src/region.rs:291` | 3d, 2d, graph | Outlines of the groups of curves inside the face `outer` (vertex indices into `g`). |
| `isqrt32` | 1 | `crates/audio-opus-codec/src/celt/bands.rs:213` | - |  |
| `isqrt_exact` | 0 | `crates/audio-opus-codec/src/celt/bands.rs:921` | - |  |
| `impl Parser::issue` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:105` | - |  |
| `impl Parser::issue_at` | 3 | `crates/discourse-academic-tex/src/texparse/bib.rs:110` | - |  |
| `impl MRun::italic` | 1 | `crates/layout-paged-typesetter/src/equation/ast.rs:77` | - |  |
| `impl Ul::item_starts_with` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:20` | - | Bytes 8.. start with `prefix` (registry-independent comparison of the item designator). |
| `impl RangeRef::iter` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:219` | - | Row-major iterator over the cells. |
| `impl Rounds::iter` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:48` | - |  |
| `impl RangedSlice<'a, T>::iter` | 1 | `crates/vector-hybrid-renderer/src/util.rs:198` | - |  |
| `iterate_children` | 0 | `crates/media-isobmff/src/bytes.rs:338` | - |  |
| `j2_plasticity_yields_when_exceeding_threshold` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:255` | - |  |
| `jacobian` | 4 | `crates/locus-parametric-solver/src/solver/lm.rs:27` | - | Numerical Jacobian over the free parameters (row-major, rows x free.len()). |
| `johnson_cook_damage_erosion` | 0 | `crates/kinetics-continuum-multiphysics/src/explicit/damage.rs:63` | - |  |
| `join` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:307` | - | Join open subpaths nearest-endpoint-first. |
| `join` | 2 | `crates/vector-radiance-trace/src/centerline.rs:431` | - |  |
| `joint_rom_fails_closed` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:362` | - |  |
| `joint_rom_from_corpus` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:123` | - | Joint ranges taken from active corpus assertions. Each range cites its assertion. |
| `joint_rom_parses_classifies_and_compiles` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:333` | - |  |
| `joint_rom_rules` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:361` | - | Compiles joint ranges into rules. Property IRIs are `q42:rom:<joint>:<axis>:<zone>{Neg,Pos}`, |
| `joints_json` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:328` | - |  |
| `journal_reversal` | 0 | `crates/chronicle-fiduciary-ledger/src/journal.rs:276` | - |  |
| `impl SampleEntry::jpeg` | 2 | `crates/media-isobmff/src/codec.rs:1311` | - | Motion-JPEG entry (`jpeg`). |
| `json_and_csv_round_trip` | 0 | `crates/kinetics-schedule-cpm/src/format.rs:764` | - |  |
| `k_weight_stage1_coeffs` | 1 | `crates/acoustics-daw-session/src/dsp/meter.rs:7` | - | BS.1770 K-weighting pre-filter stage 1 (high shelf: +4 dB high boost). |
| `k_weight_stage2_coeffs` | 1 | `crates/acoustics-daw-session/src/dsp/meter.rs:16` | - | BS.1770 K-weighting pre-filter stage 2 (high pass: 100 Hz cutoff). |
| `kbd_half` | 2 | `crates/audio-aac-codec/src/mdct.rs:182` | - |  |
| `kbd_windows_no_tns_no_ms` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:190` | - |  |
| `keeps_required_token_separators` | 0 | `crates/vector-hybrid-shaders/src/minify.rs:380` | - |  |
| `kernel_sum` | 0 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:15` | - |  |
| `key` | 1 | `crates/discourse-academic-tex/src/catalog.rs:63` | - |  |
| `key` | 3 | `crates/drafting-geometry/src/region.rs:41` | 2d |  |
| `impl Unit::key` | 1 | `crates/vector-locus-geometry/src/units.rs:122` | - | Canonical key name. |
| `key` | 3 | `crates/vector-radiance-trace/src/quantize.rs:159` | - |  |
| `impl Parser<'_>::keyword` | 1 | `crates/manifold-bim-csg/src/step.rs:176` | - |  |
| `impl Artifact::kind` | 1 | `crates/asset-production-vibe/src/lib.rs:105` | - |  |
| `impl Set::kind` | 1 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:75` | - |  |
| `impl MbState::kind_is_skip` | 1 | `crates/video-h264-codec/src/picture.rs:85` | - |  |
| `kinds` | 1 | `crates/discourse-academic-tex/src/texparse/lex.rs:148` | - |  |
| `kinds_share_their_ends` | 0 | `crates/vector-locus-geometry/src/corners.rs:258` | - |  |
| `kinetics_fem_cloth_ticks_two_produces_match` | 0 | `crates/asset-production-vibe/src/kinetics_fem_garment_blocks.rs:403` | - |  |
| `impl AssetHost<'_>::kinetics_op` | 3 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:10` | - |  |
| `impl AssetHost<'_>::kinetics_sim_op` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:12` | - |  |
| `kink` | 2 | `crates/vector-locus-geometry/src/bez.rs:68` | - | Do segments `a` and `b` (`b` following `a`) meet at a corner? |
| `kmeans_1d` | 2 | `crates/vector-radiance-trace/src/quantize.rs:175` | - | 1-D k-means over a 256-bin histogram (Lloyd iterations, evenly spaced start). |
| `kmeans_rgb` | 2 | `crates/vector-radiance-trace/src/quantize.rs:321` | - |  |
| `impl Size::known` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:20` | - |  |
| `known_dirs_are_the_fallback` | 0 | `crates/media-test-support/src/oracle.rs:162` | - |  |
| `impl OtFeatures::known_tag` | 1 | `crates/vector-glyph-shaping/src/features.rs:105` | - | Check if a tag name is recognized. |
| `trait BlockFactory::known_types` | 1 | `crates/kinetics-block-simulation/src/block.rs:99` | - |  |
| `impl BlockFactory for StandardBlockFactory::known_types` | 1 | `crates/kinetics-block-simulation/src/blocks/factory.rs:83` | - |  |
| `impl Matrix::kr_kb` | 1 | `crates/color-transforms/src/lib.rs:39` | - | (Kr, Kb) luma coefficients. |
| `impl Matrix::kr_kb` | 1 | `crates/radiance-color-grading/src/matrix.rs:24` | - | (Kr, Kb) luma coefficients. |
| `kraft` | 1 | `crates/video-h264-codec/src/cavlc.rs:187` | - |  |
| `impl AssetHost<'_>::kriging_variogram_morphometry` | 2 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:237` | - | 141: `hypermedia.geodesy.kriging_variogram_morphometry` |
| `impl Edge::l` | 2 | `crates/video-h264-codec/src/intra.rs:28` | - | p[-1, y] for y >= -1 |
| `lab` | 0 | `crates/color-transforms/src/icc/builtin.rs:176` | - |  |
| `lab_f` | 1 | `crates/color-transforms/src/icc/math.rs:124` | - |  |
| `lab_finv` | 1 | `crates/color-transforms/src/icc/math.rs:129` | - |  |
| `lab_to_xyz` | 2 | `crates/color-transforms/src/icc/math.rs:143` | - | CIE L*a*b* → CIE XYZ under white point `white`. |
| `impl HumanAnatomyProfile::label` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:90` | - |  |
| `impl NoteValue::label` | 1 | `crates/audio-timebase/src/grid.rs:43` | - |  |
| `impl GridDivision::label` | 1 | `crates/audio-timebase/src/grid.rs:87` | - |  |
| `impl TimecodeRate::label` | 1 | `crates/audio-timebase/src/timecode.rs:80` | - | Short human-readable display label. |
| `impl TimeFormat::label` | 1 | `crates/audio-timebase/src/transport.rs:29` | - |  |
| `impl LogCurve::label` | 1 | `crates/color-transforms/src/log.rs:58` | - |  |
| `impl ColorSpace::label` | 1 | `crates/color-transforms/src/spaces.rs:214` | - |  |
| `impl WorkingSpace::label` | 1 | `crates/color-transforms/src/spaces.rs:317` | - |  |
| `impl Category::label` | 1 | `crates/discourse-pdf-compliance/src/a11y/rules.rs:29` | - |  |
| `impl StandardProfile::label` | 1 | `crates/discourse-pdf-compliance/src/preflight/evaluator.rs:18` | - |  |
| `impl HiddenCategory::label` | 1 | `crates/discourse-pdf-compliance/src/redact/sanitizer.rs:33` | - |  |
| `impl TaskStatus::label` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:99` | - |  |
| `impl TaskConstraint::label` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:209` | - |  |
| `impl MediaKind::label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:49` | - |  |
| `impl LogCurve::label` | 1 | `crates/radiance-color-grading/src/log.rs:55` | - |  |
| `impl ScopeKind::label` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:50` | - |  |
| `impl WaveformType::label` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:79` | - |  |
| `impl ParadeType::label` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:101` | - |  |
| `impl Targets::label` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:127` | - |  |
| `impl ColorSpace::label` | 1 | `crates/radiance-color-grading/src/spaces.rs:272` | - |  |
| `impl BlendMode::label` | 1 | `crates/raster-blend-operators/src/lib.rs:122` | - |  |
| `impl PbrChannel::label` | 1 | `crates/surface-pbr-studio/src/material.rs:47` | - |  |
| `impl Unit::label` | 1 | `crates/vector-locus-geometry/src/units.rs:63` | - |  |
| `label` | 3 | `crates/vector-radiance-trace/src/contour.rs:51` | 2d | 4-connected component labels (`u32::MAX` outside) and per-component pixel counts. |
| `lace` | 2 | `crates/media-ogg/src/tests.rs:22` | - |  |
| `impl ElasticIsotropic::lame_lambda` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:25` | - | First Lamé parameter lambda = E * nu / ((1 + nu) * (1 - 2*nu)). |
| `lanczos_eigensolver` | 5 | `crates/kinetics-continuum-multiphysics/src/solver/lanczos.rs:15` | - | Shift-and-invert Lanczos iteration to extract lowest eigenvalues of K * x = lambda * M * x. |
| `lanczos_finds_fundamental_frequency` | 0 | `crates/kinetics-continuum-multiphysics/src/solver/lanczos.rs:82` | - |  |
| `lane_filter_matches_per_line_reference` | 0 | `crates/video-h264-codec/src/deblock.rs:420` | graph | The vectorised edge filter equals the per-line reference [`filter8`] on random edges |
| `language_codes` | 0 | `crates/media-isobmff/src/demux.rs:991` | - |  |
| `language_pack` | 0 | `crates/media-isobmff/src/mux.rs:889` | - |  |
| `laplace_decode` | 3 | `crates/audio-opus-codec/src/celt/energy.rs:17` | - | Decodes a Laplace-distributed integer (`fs` = probability of 0 in Q15, `decay` in Q14). |
| `laplace_encode` | 4 | `crates/audio-opus-codec/src/celt/energy.rs:125` | - |  |
| `laplace_get_freq1` | 2 | `crates/audio-opus-codec/src/celt/energy.rs:11` | - |  |
| `laplace_roundtrip` | 0 | `crates/audio-opus-codec/src/celt/energy.rs:157` | - |  |
| `laplacian_edge_rgba8` | 3 | `crates/raster-spatial-operators/src/convolve.rs:378` | graph | Laplacian 2D second-derivative edge detector for RGBA8 buffer. |
| `laplacian_smooth_step` | 4 | `crates/kinetics-continuum-multiphysics/src/mesh/quality.rs:51` | graph | Laplacian smoothing step for interior node positions given neighbor adjacency. |
| `large_coordinates_keep_nearby_parabolas_distinct` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:47` | - |  |
| `large_finite_vec3_has_finite_length_and_unit_direction` | 0 | `crates/drafting-geometry/src/vec.rs:220` | 3d |  |
| `large_offsets_use_co64_and_largesize_mdat` | 0 | `crates/media-isobmff/tests/roundtrip.rs:167` | - |  |
| `last_char` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:291` | - | Returns the last character of a known non-empty source section. |
| `impl GeodesicRuler::last_leg` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:316` | - |  |
| `impl StreamDecoder::last_packet_duration` | 1 | `crates/audio-opus-codec/src/decoder.rs:72` | - | Duration (samples per channel) of the last decoded packet. |
| `impl DrawColorGlyphs<'_>::last_transform` | 1 | `crates/vector-compute-renderer/src/scene.rs:1105` | - |  |
| `late_second_tile_failure_leaves_all_tiles_journal_and_prior_history_unchanged` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:210` | - |  |
| `lattice_profile` | 1 | `crates/asset-production-vibe/src/graphics.rs:42` | - | Parse an optional named lattice profile. Default admits 2k / 8-bit. |
| `launch_probe` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:269` | - |  |
| `impl DxfEntity::layer` | 1 | `crates/drafting-geometry/src/dxf_reader.rs:47` | - |  |
| `impl GroupReader<'r, 'a>::layer` | 1 | `crates/drafting-geometry/src/dxf_reader.rs:175` | - |  |
| `impl DrawCase<T>::layer` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:629` | - |  |
| `layer` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:732` | - |  |
| `impl SceneCase::layer` | 2 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:39` | - |  |
| `impl Layers<'_>::layer` | 3 | `crates/vector-radiance-trace/src/lib.rs:423` | - |  |
| `layer_clip` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:723` | - |  |
| `impl Round::layer_draw_mut` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:237` | - |  |
| `impl WebGlResources::layer_framebuffer` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1323` | - |  |
| `layer_id` | 1 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:284` | - |  |
| `layer_id` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:551` | - |  |
| `impl BlendPassBindings::layer_id` | 2 | `crates/vector-hybrid-renderer/src/target.rs:179` | - |  |
| `impl RoundBindings::layer_id` | 2 | `crates/vector-hybrid-renderer/src/target.rs:218` | - | Return the bound texture for a parity, if present. |
| `impl Round::layer_passes` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:151` | - |  |
| `layer_region` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:562` | - |  |
| `layer_requests` | 0 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:266` | - |  |
| `impl WebGlResources::layer_texture` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1318` | - |  |
| `impl Scheduler<'a, 'p>::layer_texture_parity` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:565` | - | Select the texture group for a recorded layer depth. |
| `impl GpuResources::layer_view` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1059` | - |  |
| `impl GpuResources::layer_views` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1055` | - |  |
| `impl SceneCase::layer_with` | 5 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:43` | 2d |  |
| `impl ProgramConfig::layout` | 1 | `crates/audio-aac-codec/src/config.rs:125` | - | Elements in output channel order (front, side, back, LFE). |
| `impl AudioSpecificConfig::layout` | 1 | `crates/audio-aac-codec/src/config.rs:265` | - | Elements in output channel order. |
| `impl Decoder::layout` | 1 | `crates/audio-aac-codec/src/decoder.rs:109` | - | Element layout in output channel order (C, L/R, surround pairs, LFE for configurations 1–7). |
| `layout_area_text` | 3 | `crates/vector-glyph-shaping/src/layout.rs:65` | 2d | Lay out simple area text into a bounded rectangle. |
| `layout_atom` | 5 | `crates/discourse-academic-tex/src/typeset.rs:372` | - |  |
| `layout_curved_line_label` | 3 | `crates/geodesy-spatial-analysis/src/cartography.rs:193` | - | Places text characters along a polyline, rotating each character to follow the line's curvature. |
| `layout_equation` | 2 | `crates/layout-paged-typesetter/src/equation/layout.rs:63` | - |  |
| `layout_kinetics_domain_composable_artifacts_and_handles` | 0 | `crates/asset-production-vibe/src/kinetics_fem_garment_blocks.rs:325` | - |  |
| `layout_mtext` | 2 | `crates/locus-parametric-solver/src/typography/mtext.rs:169` | - | Layout an MTEXT string into positioned lines and strokes. |
| `impl AssetHost<'_>::layout_op` | 3 | `crates/asset-production-vibe/src/layout_ops/mod.rs:42` | - |  |
| `layout_stroke_line` | 4 | `crates/locus-parametric-solver/src/typography/stroke.rs:158` | - |  |
| `layout_text_on_path` | 3 | `crates/vector-glyph-shaping/src/layout.rs:205` | - | Lay out characters along a sequence of 2D curve points. |
| `lbl` | 1 | `crates/discourse-academic-tex/src/catalog.rs:60` | - |  |
| `lbm_mass_conservation` | 0 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:192` | physics |  |
| `impl AudioSpecificConfig::lc` | 2 | `crates/audio-aac-codec/src/config.rs:190` | - | A plain AAC-LC configuration. |
| `lcg` | 1 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:43` | numerics | Linear congruential generator step. |
| `lcg` | 1 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:9` | - | LCG step: deterministically advances generator state. |
| `lcg_rand` | 1 | `crates/audio-opus-codec/src/celt/bands.rs:12` | - |  |
| `ldlt_inertia_count` | 0 | `crates/kinetics-continuum-multiphysics/src/solver/ldlt.rs:128` | - |  |
| `impl BoxBuf::leaf` | 3 | `crates/media-isobmff/src/bytes.rs:275` | - | Append a complete box with the given payload. |
| `left` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:40` | - |  |
| `len` | 1 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:77` | - |  |
| `impl Data::len` | 1 | `crates/audio-device-cpal/src/lib.rs:533` | - | The full length of the buffer in samples. |
| `impl SampleRange::len` | 1 | `crates/audio-timebase/src/rate.rs:99` | - |  |
| `len` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:67` | - |  |
| `len` | 1 | `crates/discourse-academic-tex/src/catalog.rs:69` | - |  |
| `impl Line::len` | 1 | `crates/drafting-geometry/src/curve.rs:16` | - |  |
| `impl Arc::len` | 1 | `crates/drafting-geometry/src/curve.rs:80` | - |  |
| `impl Segment::len` | 1 | `crates/drafting-geometry/src/curve.rs:332` | - |  |
| `impl Polyline::len` | 1 | `crates/drafting-geometry/src/curve.rs:487` | - |  |
| `impl Vec2::len` | 1 | `crates/drafting-geometry/src/vec.rs:34` | - |  |
| `impl Vec3::len` | 1 | `crates/drafting-geometry/src/vec.rs:166` | - |  |
| `impl BoxBuf::len` | 1 | `crates/media-isobmff/src/bytes.rs:310` | - |  |
| `trait ByteSource::len` | 1 | `crates/media-isobmff/src/source.rs:11` | - | Total length in bytes. |
| `impl ByteSource for [u8]::len` | 1 | `crates/media-isobmff/src/source.rs:25` | - |  |
| `impl ByteSource for Vec<u8>::len` | 1 | `crates/media-isobmff/src/source.rs:38` | - |  |
| `impl ByteSource for &T::len` | 1 | `crates/media-isobmff/src/source.rs:47` | - |  |
| `impl ByteSource for Arc<T>::len` | 1 | `crates/media-isobmff/src/source.rs:56` | - |  |
| `impl ByteSource for Box<T>::len` | 1 | `crates/media-isobmff/src/source.rs:65` | - |  |
| `impl ByteSource for std::fs::File::len` | 1 | `crates/media-isobmff/src/source.rs:75` | - |  |
| `trait ByteSource::len` | 1 | `crates/media-ogg/src/lib.rs:25` | - |  |
| `impl ByteSource for [u8]::len` | 1 | `crates/media-ogg/src/lib.rs:33` | - |  |
| `impl ByteSource for Vec<u8>::len` | 1 | `crates/media-ogg/src/lib.rs:45` | - |  |
| `impl ByteSource for &T::len` | 1 | `crates/media-ogg/src/lib.rs:54` | - |  |
| `impl Signal::len` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:208` | - |  |
| `impl Ranges::len` | 1 | `crates/vector-hybrid-renderer/src/util.rs:132` | - |  |
| `impl RangedSlice<'a, T>::len` | 1 | `crates/vector-hybrid-renderer/src/util.rs:190` | - |  |
| `impl ArcPath::len` | 1 | `crates/vector-locus-geometry/src/arc.rs:33` | - | The path's length. |
| `impl Track::len` | 1 | `crates/vector-surface-brush/src/track.rs:84` | - |  |
| `impl Vec2::len2` | 1 | `crates/drafting-geometry/src/vec.rs:37` | - |  |
| `len_sq` | 1 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:376` | - |  |
| `len_sq` | 1 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:73` | - |  |
| `impl Body::length` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:153` | - |  |
| `impl DVec2::length` | 1 | `crates/kinetics-garment-draping/src/math.rs:33` | - |  |
| `impl DVec3::length` | 1 | `crates/kinetics-garment-draping/src/math.rs:124` | - |  |
| `impl WallAxis::length` | 1 | `crates/manifold-bim-csg/src/parametric.rs:90` | - |  |
| `impl MepGeom::length` | 1 | `crates/manifold-bim-csg/src/parametric.rs:310` | - |  |
| `impl Vec2::length` | 1 | `crates/surface-pbr-studio/src/math.rs:34` | - |  |
| `impl Vec3::length` | 1 | `crates/surface-pbr-studio/src/math.rs:124` | - |  |
| `impl Vec2::length` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:89` | - |  |
| `impl PathData::length` | 1 | `crates/vector-locus-geometry/src/path.rs:497` | - | Total length of all segments. |
| `impl Run::length` | 1 | `crates/vector-radiance-trace/src/centerline.rs:265` | - |  |
| `length_of_square` | 0 | `crates/vector-locus-geometry/src/path.rs:614` | - |  |
| `length_prefixed` | 1 | `crates/video-h264-preview/src/synth.rs:105` | - |  |
| `length_prefixed_nals` | 2 | `crates/bitstream-codec/src/lib.rs:308` | - | Split a length-prefixed sample (AVCC/HVCC, `length_size` 1/2/4 bytes) into NAL units. |
| `impl DVec2::length_squared` | 1 | `crates/kinetics-garment-draping/src/math.rs:28` | - |  |
| `impl DVec3::length_squared` | 1 | `crates/kinetics-garment-draping/src/math.rs:119` | - |  |
| `impl Vec2::length_squared` | 1 | `crates/surface-pbr-studio/src/math.rs:29` | - |  |
| `impl Vec3::length_squared` | 1 | `crates/surface-pbr-studio/src/math.rs:119` | - |  |
| `impl Vec2::length_squared` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:93` | - |  |
| `impl Vec2::lerp` | 3 | `crates/drafting-geometry/src/vec.rs:65` | 2d |  |
| `impl DVec2::lerp` | 3 | `crates/kinetics-garment-draping/src/math.rs:43` | - |  |
| `impl Value::lerp` | 3 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:63` | - |  |
| `lerp` | 3 | `crates/layout-presentation-deck/src/morph/interpolate.rs:17` | - |  |
| `impl Vec2::lerp` | 3 | `crates/surface-pbr-studio/src/math.rs:44` | - |  |
| `impl Vec3::lerp` | 3 | `crates/surface-pbr-studio/src/math.rs:162` | - |  |
| `lerp_f32` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:599` | - |  |
| `lerp_rgba` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:285` | - |  |
| `lerp_rgba` | 3 | `crates/layout-presentation-deck/src/morph/interpolate.rs:28` | - |  |
| `letters_to_col` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:115` | - | `A` -> 0, `xfd` -> 16383. `None` for empty, non-letters or beyond `XFD`. |
| `impl Waveform::level` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:265` | - |  |
| `impl Acc::level` | 1 | `crates/radiance-color-grading/src/scopes/summary.rs:36` | - |  |
| `level_resources` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:412` | - |  |
| `impl Decoder::level_scale` | 2 | `crates/video-h264-codec/src/decoder.rs:314` | - |  |
| `levinson` | 2 | `crates/audio-aac-codec/src/tns.rs:246` | - | Levinson–Durbin on autocorrelation `r[0..=order]`: returns reflection coefficients and the |
| `lex` | 1 | `crates/discourse-academic-tex/src/texparse/lex.rs:44` | - | Tokenize `src`. Never fails: every byte lands in some token. |
| `lex` | 1 | `crates/discourse-scientific-matlang/src/interpreter/lexer.rs:54` | - |  |
| `library_colors` | 3 | `crates/vector-radiance-trace/src/quantize.rs:288` | - |  |
| `impl Revolve::light_direction` | 1 | `crates/vector-locus-pathops/src/three_d/revolve.rs:119` | - |  |
| `impl SliceData::limit_bits` | 1 | `crates/video-prores-codec/src/encode.rs:286` | - |  |
| `limited_range_black_white_and_grey` | 0 | `crates/video-h264-preview/src/convert.rs:132` | - |  |
| `limited_range_levels` | 0 | `crates/color-transforms/src/lib.rs:337` | - |  |
| `limits` | 0 | `crates/media-ogg/src/strict_opus.rs:139` | - |  |
| `limits` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:5` | - |  |
| `impl VelloRendererOptions::limits` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:105` | - |  |
| `impl VelloHybridRendererOptions::limits` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:102` | - |  |
| `impl Series::line` | 3 | `crates/discourse-scientific-matlang/src/plot.rs:71` | - |  |
| `impl Model::line` | 3 | `crates/locus-parametric-solver/src/solver/model.rs:244` | 2d |  |
| `line` | 2 | `crates/vector-locus-geometry/src/shapes.rs:159` | - | Open straight line. |
| `impl Seg::line` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:44` | - |  |
| `impl Seg::line` | 2 | `crates/vector-radiance-trace/src/fit.rs:237` | - |  |
| `line_amount` | 3 | `crates/chronicle-fiduciary-ledger/src/money.rs:37` | economics | Calculates line net amount (before tax) = qty * unit price * (1 - discount). |
| `line_amount_calculation` | 0 | `crates/chronicle-fiduciary-ledger/src/money.rs:155` | - |  |
| `line_at` | 2 | `crates/discourse-academic-tex/src/review/ai_repair.rs:223` | - |  |
| `impl GridDivision::line_at_or_before` | 5 | `crates/audio-timebase/src/grid.rs:161` | - | The grid line at or immediately before `pos`. |
| `line_bidi_class` | 1 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:29` | - |  |
| `line_break_opportunities` | 2 | `crates/layout-paged-typesetter/src/hyphenation/kinsoku.rs:33` | - | Computes valid line-break opportunities (byte offsets) in `text` respecting Kinsoku rules. |
| `line_circle` | 2 | `crates/drafting-geometry/src/intersect.rs:24` | 2d | Intersections of the infinite line through `l` with a circle. Returns points with line params. |
| `line_circle_hits` | 0 | `crates/drafting-geometry/src/tests.rs:96` | - |  |
| `line_intersection` | 4 | `crates/vector-locus-pathops/src/pathops/boolean.rs:88` | computational geometry | Line segment intersection: returns parameter t in [0, 1] for segment (p0, p1) and u in [0, 1] for (p2, p3). |
| `line_intersections` | 0 | `crates/drafting-geometry/src/tests.rs:87` | - |  |
| `line_line` | 2 | `crates/drafting-geometry/src/intersect.rs:17` | computational geometry, 2d | Intersection of two finite segments. |
| `line_line_infinite` | 4 | `crates/drafting-geometry/src/intersect.rs:4` | computational geometry, 2d | Intersection of two infinite lines through (a1,a2) and (b1,b2), with parameters on each. |
| `impl OutlinePen for BezPathOutline::line_to` | 3 | `crates/vector-compute-renderer/src/scene.rs:1080` | - |  |
| `linear_burn_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:289` | numerics | HDRI Linear Burn: `backdrop + source - 1.0`. |
| `linear_burn_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:367` | numerics |  |
| `linear_dodge_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:283` | numerics | HDRI Linear Dodge (Additive): `backdrop + source`. |
| `linear_dodge_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:363` | numerics |  |
| `linear_kernel_size_1` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:889` | numerics |  |
| `linear_kernel_size_13` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:910` | numerics |  |
| `linear_kernel_size_3` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:896` | numerics |  |
| `linear_kernel_size_7` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:903` | numerics |  |
| `linear_light_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:295` | numerics | HDRI Linear Light: `backdrop + 2.0 * source - 1.0`. |
| `linear_light_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:371` | numerics |  |
| `linear_to_srgb` | 1 | `crates/color-transforms/src/lib.rs:187` | numerics |  |
| `linear_to_srgb` | 1 | `crates/radiance-color-grading/src/matrix.rs:270` | numerics |  |
| `linear_to_srgb_f32` | 1 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:136` | numerics |  |
| `linear_to_srgb_u8` | 1 | `crates/color-transforms/src/lib.rs:248` | numerics |  |
| `linear_to_srgb_u8` | 1 | `crates/radiance-color-grading/src/matrix.rs:344` | numerics |  |
| `linear_to_srgb_u8_table` | 0 | `crates/color-transforms/src/lib.rs:242` | numerics | 4096-entry table: linear (0..1, quantised to 12 bits) â†’ sRGB-encoded u8. |
| `linear_to_srgb_u8_table` | 0 | `crates/radiance-color-grading/src/matrix.rs:334` | numerics | 4096-entry table: linear (0..1 quantised to 12 bits) -> sRGB u8. |
| `impl GridDivision::lines` | 7 | `crates/audio-timebase/src/grid.rs:202` | - | List all grid lines within `[start, end)`, capped at `max` entries. |
| `lines` | 6 | `crates/vector-radiance-trace/src/centerline.rs:25` | - | The lines of colour `label` in `labels` (`w` x `h`, row-major) no wider than `max_width` |
| `lines_are_capped` | 0 | `crates/audio-timebase/src/grid.rs:253` | - |  |
| `lines_from_breaks` | 3 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:171` | - |  |
| `lines_of` | 1 | `crates/drafting-geometry/src/region.rs:443` | 2d |  |
| `link_stages` | 4 | `crates/color-transforms/src/icc/transform.rs:160` | - |  |
| `lint` | 2 | `crates/vector-hybrid-shaders/src/lint/mod.rs:26` | 3d | Runs every WGSL shader lint over `module` and panics with a single aggregated |
| `lint_source` | 1 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:48` | - |  |
| `list` | 1 | `crates/asset-production-vibe/src/lib.rs:529` | - |  |
| `impl Parser<'_>::list` | 2 | `crates/manifold-bim-csg/src/step.rs:306` | - |  |
| `impl References::list` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:122` | - |  |
| `impl References::list_mut` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:130` | - |  |
| `impl Tok::literal_text` | 1 | `crates/chronicle-tabular-engine/src/numfmt/parse.rs:40` | - |  |
| `live` | 1 | `crates/video-h264-preview/src/lib.rs:391` | - |  |
| `impl AssetHost<'_>::living_world_anatomy_op` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:290` | physics, 3d, semantics | Categorical operations dispatch for Wave 12 Living-World anatomy, fauna rigging, |
| `ln` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:70` | - |  |
| `trait Scalar::ln` | 1 | `crates/kinetics-garment-draping/src/math.rs:243` | - |  |
| `impl Scalar for f64::ln` | 1 | `crates/kinetics-garment-draping/src/math.rs:260` | - |  |
| `impl Scalar for Jet<N>::ln` | 1 | `crates/kinetics-garment-draping/src/math.rs:309` | - |  |
| `impl MixerState::load` | 2 | `crates/raster-layer-mask-adapter/src/mixer.rs:62` | - |  |
| `load_block` | 6 | `crates/video-prores-codec/src/encode.rs:191` | - |  |
| `load_clip_path` | 1 | `crates/vector-compute-shaders/shader/clip_leaf.wgsl:70` | - |  |
| `load_corpus_bytes` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:84` | - |  |
| `load_edge` | 6 | `crates/video-h264-codec/src/intra.rs:33` | graph |  |
| `load_shader_infos` | 1 | `crates/vector-hybrid-shaders/build.rs:70` | 3d |  |
| `load_shader_names` | 1 | `crates/vector-hybrid-shaders/build.rs:82` | 3d |  |
| `load_snippets` | 1 | `crates/discourse-academic-tex/src/review/snippets.rs:121` | - | Load and merge all snippets given an optional project directory root. |
| `impl CowTerrainTileGrid::load_tile` | 4 | `crates/terrain-brushes/adaptation/grid-impl.rs:65` | - | Import one already-decoded finite height tile before local history begins. |
| `impl CowTerrainTileGrid::load_tile` | 4 | `crates/terrain-brushes/src/lib.rs:246` | - | Import one already-decoded finite height tile before local history begins. |
| `impl XEl::local` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:46` | - |  |
| `local_name` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:18` | - |  |
| `impl Clut::locate` | 3 | `crates/color-transforms/src/icc/clut.rs:63` | - |  |
| `locked_dt_frame_rate` | 1 | `crates/asset-production-vibe/src/chronicle_ops.rs:414` | - |  |
| `impl AssetHost<'_>::locus_bishop_rmf_cyclic` | 2 | `crates/asset-production-vibe/src/locus_generative/rmf_geodesic.rs:41` | - | Cyclic double-reflection Bishop rotation-minimizing frame (RMF) generator along a 3D path. |
| `impl AssetHost<'_>::locus_boolean` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:138` | - |  |
| `impl AssetHost<'_>::locus_ext` | 3 | `crates/asset-production-vibe/src/locus_ops.rs:7` | - |  |
| `impl AssetHost<'_>::locus_fillet` | 2 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:56` | computational geometry, numerics | Corner filleting on a 2D polyline or closed polygon. |
| `impl AssetHost<'_>::locus_generative_op` | 3 | `crates/asset-production-vibe/src/locus_generative/mod.rs:14` | - | Dispatch morphism operations for generative curve and geodesic path workflows. |
| `locus_manifold_domain_composable_artifacts_and_handles` | 0 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:546` | - |  |
| `impl AssetHost<'_>::locus_offset` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:71` | - |  |
| `impl AssetHost<'_>::locus_resample` | 2 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:247` | - | Resample a 2D locus by uniform arc length count, fixed step length, or curvature weighting. |
| `impl AssetHost<'_>::locus_shortest_edge_path` | 2 | `crates/asset-production-vibe/src/locus_generative/rmf_geodesic.rs:135` | graph | Shortest edge path across a 3D triangle manifold using deterministic Dijkstra search. |
| `locus_sketch_svg_path_roundtrip_bit_stable` | 0 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:674` | - |  |
| `impl AssetHost<'_>::locus_spline` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:19` | - |  |
| `impl AssetHost<'_>::locus_stroke_to_locus` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:207` | - |  |
| `impl AssetHost<'_>::locus_trim` | 2 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:459` | numerics | Extract a sub-curve from normalized arc length [start, end] with exact linear interpolation. |
| `locus_warp_live_corners_90deg_setback_not_l_vs_c` | 0 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:768` | - |  |
| `locus_warp_print_density_svg_path_roundtrip` | 0 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:703` | - |  |
| `loft_points3` | 1 | `crates/asset-production-vibe/src/manifold_ops.rs:889` | - |  |
| `trait Block::log` | 3 | `crates/kinetics-block-simulation/src/block.rs:69` | - |  |
| `impl Block for Scope::log` | 3 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:54` | - |  |
| `impl Block for ToWorkspace::log` | 3 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:101` | - |  |
| `impl log::Log for BuildScriptLog::log` | 2 | `crates/vector-compute-shaders/build.rs:150` | - |  |
| `log10` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:78` | - |  |
| `log2_frac` | 2 | `crates/audio-opus-codec/src/celt/rate.rs:20` | - | Conservative (rounded up) log2 of `val` with `frac` fractional bits. |
| `log2_frac_values` | 0 | `crates/audio-opus-codec/src/celt/rate.rs:552` | - |  |
| `log2lin` | 1 | `crates/audio-opus-codec/src/silk/mod.rs:482` | - |  |
| `impl Sim::log_sinks` | 2 | `crates/kinetics-block-simulation/src/sim.rs:141` | - |  |
| `log_sources_land_sensibly_in_rec709` | 0 | `crates/color-transforms/src/transform.rs:450` | - |  |
| `log_to_rec709` | 3 | `crates/color-transforms/src/transform.rs:348` | 3d | Convenience: a log colour space → display Rec. 709 (tone mapped), used for the built-in |
| `logc3_decode` | 1 | `crates/color-transforms/src/log.rs:203` | - |  |
| `logc3_decode` | 1 | `crates/radiance-color-grading/src/log.rs:216` | - |  |
| `logc3_encode` | 1 | `crates/color-transforms/src/log.rs:199` | - |  |
| `logc3_encode` | 1 | `crates/radiance-color-grading/src/log.rs:208` | - |  |
| `logc4_consts` | 0 | `crates/color-transforms/src/log.rs:211` | - |  |
| `logc4_decode` | 1 | `crates/color-transforms/src/log.rs:223` | - |  |
| `logc4_decode` | 1 | `crates/radiance-color-grading/src/log.rs:239` | - |  |
| `logc4_encode` | 1 | `crates/color-transforms/src/log.rs:219` | - |  |
| `logc4_encode` | 1 | `crates/radiance-color-grading/src/log.rs:231` | - |  |
| `long_secant_keeps_both_intersections` | 0 | `crates/drafting-geometry/src/tests.rs:316` | - |  |
| `looks_like_arxiv` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:54` | - |  |
| `looks_like_doi` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:48` | - |  |
| `impl DecodeTable::lookup` | 2 | `crates/color-transforms/src/transform.rs:97` | - |  |
| `impl DecodeTable::lookup` | 2 | `crates/radiance-color-grading/src/transform.rs:163` | - |  |
| `lower` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:83` | - |  |
| `lowpass` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:252` | - | Windowed-sinc (Blackman, 401 taps) low-pass at `fc` Hz, zero-phase (delay-compensated). |
| `lpc_analysis_filter` | 4 | `crates/audio-opus-codec/src/silk/mod.rs:494` | - |  |
| `lpc_inverse_pred_gain` | 1 | `crates/audio-opus-codec/src/silk/lpc.rs:201` | - |  |
| `lpc_of` | 2 | `crates/audio-aac-codec/src/tns.rs:178` | - |  |
| `lsdyna_deck_structure` | 0 | `crates/kinetics-continuum-multiphysics/src/interchange/lsdyna.rs:55` | - |  |
| `lu` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:368` | numerics | LU decomposition with partial pivoting: P A = L U. |
| `luhn_check` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:25` | - | Luhn algorithm for credit card checksum verification. |
| `lum` | 1 | `crates/raster-blend-operators/src/lib.rs:467` | - |  |
| `lum` | 1 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:73` | 3d |  |
| `impl Gamut::luma` | 1 | `crates/color-transforms/src/spaces.rs:72` | - | Luminance weights (the Y row of [`Gamut::to_xyz`]). |
| `impl Gamut::luma` | 1 | `crates/radiance-color-grading/src/spaces.rs:73` | - | Luminance weights (Y row of `to_xyz`). |
| `luma` | 1 | `crates/vector-radiance-trace/src/quantize.rs:16` | - |  |
| `luma2020` | 0 | `crates/color-transforms/src/grade.rs:116` | - |  |
| `luma709` | 3 | `crates/color-transforms/src/lib.rs:255` | semantics, numerics | Rec.709 luma of a linear or encoded RGB triple. |
| `luma709` | 3 | `crates/radiance-color-grading/src/matrix.rs:351` | semantics, numerics | Rec.709 luma of a linear or encoded RGB triple. |
| `impl SliceDecoder<'a>::luma_avail4` | 3 | `crates/video-h264-codec/src/slicedec.rs:971` | - |  |
| `luma_dc_dequant` | 3 | `crates/video-h264-codec/src/transform.rs:46` | - | Intra16x16 luma DC: inverse Hadamard + scaling (8.5.10). `c` is raster order (row*4+col); |
| `luma_dc_flat` | 0 | `crates/video-h264-codec/src/transform.rs:268` | - |  |
| `impl SliceDecoder<'a>::luma_list4` | 1 | `crates/video-h264-codec/src/slicedec.rs:1111` | - |  |
| `impl Frame::luma_window` | 7 | `crates/video-h264-codec/src/picture.rs:232` | - | Copy the w x h luma window at (x0, y0) (clamped to the picture) into `out` (stride `os`). |
| `impl Profile::lut_for` | 2 | `crates/color-transforms/src/icc/profile.rs:217` | - |  |
| `impl GradientRampCache::luts_size` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:106` | - | Get the size of the packed luts. |
| `m` | 2 | `crates/vector-compute-renderer/src/scene.rs:881` | - |  |
| `m44_from_affine` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1286` | - |  |
| `main` | 0 | `checks/consumer-smoke/src/main.rs:6` | - |  |
| `main` | 0 | `crates/audio-device-cpal/build.rs:5` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/android.rs:13` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/beep.rs:29` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/enumerate.rs:6` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/feedback.rs:43` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/record_wav.rs:34` | - |  |
| `main` | 0 | `crates/audio-device-cpal/examples/synth_tones.rs:15` | - |  |
| `main` | 1 | `crates/gpu-command-profiler/examples/compute_shader.wgsl:34` | 3d |  |
| `main` | 0 | `crates/gpu-command-profiler/examples/demo.rs:422` | - |  |
| `main` | 0 | `crates/raster-layer-mask-adapter/examples/edit_existing_layer_mask.rs:4` | - |  |
| `main` | 0 | `crates/raster-selection-morphology/examples/modify_selection.rs:3` | - |  |
| `main` | 0 | `crates/terrain-brushes/examples/local-stroke.rs:4` | - |  |
| `main` | 0 | `crates/timeline-editing/examples/transition_edit.rs:7` | - |  |
| `main` | 0 | `crates/vector-compute-shaders/build.rs:20` | - |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/backdrop.wgsl:24` | 3d |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/backdrop_dyn.wgsl:28` | 3d |  |
| `main` | 1 | `crates/vector-compute-shaders/shader/bbox_clear.wgsl:13` | 3d |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/binning.wgsl:55` | 3d |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/clip_leaf.wgsl:80` | 3d |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/clip_reduce.wgsl:24` | 3d |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/coarse.wgsl:156` | 3d |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/draw_leaf.wgsl:52` | 3d |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/draw_reduce.wgsl:22` | 3d |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/fine.wgsl:1064` | 3d |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:831` | 3d |  |
| `main` | 1 | `crates/vector-compute-shaders/shader/path_count.wgsl:51` | 3d |  |
| `main` | 0 | `crates/vector-compute-shaders/shader/path_count_setup.wgsl:17` | - |  |
| `main` | 1 | `crates/vector-compute-shaders/shader/path_tiling.wgsl:39` | 3d |  |
| `main` | 0 | `crates/vector-compute-shaders/shader/path_tiling_setup.wgsl:20` | - |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/pathtag_reduce.wgsl:21` | 3d |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/pathtag_reduce2.wgsl:21` | 3d |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/pathtag_scan.wgsl:28` | 3d |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/pathtag_scan1.wgsl:26` | 3d |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/tile_alloc.wgsl:35` | 3d |  |
| `main` | 0 | `crates/vector-hybrid-renderer/examples/render_to_file.rs:22` | - | Main entry point for the headless rendering example. |
| `main` | 0 | `crates/vector-hybrid-shaders/build.rs:56` | - |  |
| `main` | 0 | `crates/vector-hybrid-shaders/src/main.rs:16` | - |  |
| `main` | 0 | `crates/vector-hybrid-shaders/src/main.rs:48` | - |  |
| `main_offsets` | 5 | `crates/layout-box-autolayout/src/flex.rs:23` | - |  |
| `impl ImageCache::maintain` | 2 | `crates/vector-cpu-render-adapter/src/image_cache.rs:94` | - | Advance the frame counter and evict stale entries. |
| `impl VelloCpuScenePainter::maintain` | 1 | `crates/vector-cpu-render-adapter/src/scene.rs:51` | - | Advance the image cache's frame counter and evict stale entries. |
| `impl GradientRampCache::maintain` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:97` | - | Maintain the gradient cache by evicting old entries. |
| `impl GlyphAtlasResources::maintain` | 2 | `crates/vector-hybrid-renderer/src/text.rs:49` | - |  |
| `trait BlockFactory::make` | 4 | `crates/kinetics-block-simulation/src/block.rs:98` | - |  |
| `impl BlockFactory for StandardBlockFactory::make` | 4 | `crates/kinetics-block-simulation/src/blocks/factory.rs:48` | - |  |
| `impl CpalInputCallback<I, C>::make_callback_info` | 2 | `crates/audio-device-cpal/src/host/oboe/input_callback.rs:32` | - |  |
| `impl CpalOutputCallback<I, C>::make_callback_info` | 2 | `crates/audio-device-cpal/src/host/oboe/output_callback.rs:32` | - |  |
| `make_host` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/m64_continuum.rs:521` | - |  |
| `make_host` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:663` | - |  |
| `make_picture` | 1 | `crates/video-h264-codec/src/decoder.rs:597` | - |  |
| `impl Frame::make_row` | 4 | `crates/video-h264-codec/src/picture.rs:184` | - | Build a published row from a contiguous plane set (rows of MB row `r`) and motion data. |
| `make_sample` | 7 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:364` | - |  |
| `make_sample` | 7 | `crates/terrain-brushes/src/lib.rs:530` | - |  |
| `make_stream` | 2 | `crates/audio-device-cpal/examples/synth_tones.rs:130` | - |  |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/geodesy_ops.rs:704` | - |  |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/locus_generative/mod.rs:44` | - |  |
| `make_test_host` | 2 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:701` | - |  |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:361` | - |  |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/manifold_ops.rs:1165` | - |  |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:183` | - |  |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:373` | - |  |
| `malformed_files_are_empty_not_fatal` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:220` | - |  |
| `malformed_final_slice_is_not_presented` | 0 | `crates/video-h264-preview/src/lib.rs:713` | - |  |
| `malformed_nals_refuse_parameter_sets_and_sp_slices` | 0 | `crates/video-h264-preview/src/lib.rs:750` | - |  |
| `malformed_profiles_and_primitive_budgets_are_rejected` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:151` | - |  |
| `malformed_profiles_and_primitive_budgets_are_rejected` | 0 | `crates/profile-authoring/src/tests.rs:254` | - |  |
| `malformed_profiles_are_rejected` | 0 | `crates/profile-authoring/src/tests.rs:43` | - |  |
| `malformed_recovers` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:476` | - |  |
| `manifest_dir` | 0 | `crates/vector-compute-shaders/src/compile/mod.rs:289` | - |  |
| `impl AssetHost<'_>::manifold_bevel` | 2 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:452` | - |  |
| `impl AssetHost<'_>::manifold_bisect_plane` | 2 | `crates/asset-production-vibe/src/manifold_topology/solidify_bisect_subdivide.rs:83` | - |  |
| `impl AssetHost<'_>::manifold_boolean_3d` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:41` | - |  |
| `impl AssetHost<'_>::manifold_compile_10d_pbr` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:546` | - |  |
| `impl AssetHost<'_>::manifold_evaluate_field` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/fields_instances.rs:12` | - |  |
| `impl AssetHost<'_>::manifold_ext` | 3 | `crates/asset-production-vibe/src/manifold_ops.rs:17` | - |  |
| `impl AssetHost<'_>::manifold_fields_sdf_op` | 3 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:166` | - | Dispatch entrypoint for Wave 7 Manifold Lazy Fields, SDF Bricks, Instancing & Botany. |
| `impl AssetHost<'_>::manifold_fspm_vascular_grow` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/sdf_botany.rs:278` | - |  |
| `impl AssetHost<'_>::manifold_import_mesh` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:267` | computational geometry |  |
| `impl AssetHost<'_>::manifold_inset` | 2 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:597` | - |  |
| `impl AssetHost<'_>::manifold_instance_on_points` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/fields_instances.rs:267` | - |  |
| `impl AssetHost<'_>::manifold_loft` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:447` | - |  |
| `impl AssetHost<'_>::manifold_mesh_to_sdf_brick` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:237` | computational geometry |  |
| `impl AssetHost<'_>::manifold_parse_dxf` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:416` | - |  |
| `impl AssetHost<'_>::manifold_project_dxf` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:306` | - |  |
| `impl AssetHost<'_>::manifold_radial_euler` | 2 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:98` | physics |  |
| `impl AssetHost<'_>::manifold_realize_instances` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/fields_instances.rs:398` | - |  |
| `impl AssetHost<'_>::manifold_sdf_brick_to_mesh` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/sdf_botany.rs:16` | computational geometry |  |
| `impl AssetHost<'_>::manifold_simplify` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:107` | - |  |
| `impl AssetHost<'_>::manifold_simplify_anisotropic` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:231` | - |  |
| `impl AssetHost<'_>::manifold_simplify_decimate` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:147` | - |  |
| `impl AssetHost<'_>::manifold_simplify_remesh` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:187` | - |  |
| `impl AssetHost<'_>::manifold_solidify` | 2 | `crates/asset-production-vibe/src/manifold_topology/solidify_bisect_subdivide.rs:13` | - |  |
| `impl AssetHost<'_>::manifold_subdivide_catmull_clark` | 2 | `crates/asset-production-vibe/src/manifold_topology/solidify_bisect_subdivide.rs:296` | - |  |
| `impl AssetHost<'_>::manifold_topology_op` | 3 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:139` | - | Dispatch manifold topology morphisms. |
| `impl GpuProfiler::manual_owning_scope` | 3 | `crates/gpu-command-profiler/src/profiler.rs:204` | - | Starts a new **manually closed** profiler scope that takes ownership of the passed encoder or rendering/compute pass. |
| `impl Expr::map` | 2 | `crates/chronicle-tabular-engine/src/formula/ast.rs:190` | - |  |
| `impl Matrix::map` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:123` | - |  |
| `impl DebugDownloads<'a>::map` | 3 | `crates/vector-compute-renderer/src/lib.rs:794` | - |  |
| `impl Track::map` | 3 | `crates/vector-surface-brush/src/track.rs:148` | 3d | The point at arc length `s`, offset `d` along the left normal. |
| `map_draw_tag` | 1 | `crates/vector-compute-shaders/shader/shared/drawtag.wgsl:47` | - |  |
| `impl DistortTransform::map_inverse` | 7 | `crates/raster-spatial-operators/src/distort.rs:72` | - | Maps destination coordinate `(dst_x, dst_y)` back to source coordinate `Option<(src_x, src_y)>`. |
| `trait WebGlResultExt::map_js_error` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/error.rs:178` | - | Attach the WebGL operation that produced this JavaScript result. |
| `impl WebGlResultExt<T> for Result<T, JsValue>::map_js_error` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/error.rs:182` | - |  |
| `impl ToneMap::map_luma` | 3 | `crates/radiance-color-grading/src/transform.rs:29` | numerics | Apply tone mapping to a linear luminance value `y` with target peak `peak_nits`. |
| `map_nonlinear` | 3 | `crates/vector-locus-pathops/src/effects/util.rs:108` | numerics | Non-linear path mapping subdividing segments into pieces of at most `max_piece` length. |
| `impl Homography::map_rect_bbox` | 2 | `crates/vector-locus-geometry/src/projective.rs:109` | computational geometry, 2d | The bounding box of the image of `r`. |
| `map_transfer_function` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:645` | - |  |
| `marching_squares_contours` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:227` | - | Extracts contour polylines from a DEM grid at a specified interval using the Marching Squares algorithm. |
| `impl Renderer::mark_override_image_dirty` | 2 | `crates/vector-compute-renderer/src/lib.rs:555` | - | Marks `image` as dirty in the atlas cache, so its override texture will be recopied on |
| `impl GradientRampCache::mark_synced` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:121` | - | Mark the luts as synced. |
| `impl Presentation::master_for_slide` | 2 | `crates/layout-presentation-deck/src/model/master.rs:135` | - |  |
| `master_ph` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:84` | - |  |
| `mat_inverse_roundtrip` | 0 | `crates/drafting-geometry/src/tests.rs:16` | - |  |
| `match_fn` | 1 | `crates/chronicle-tabular-engine/src/functions/lookup.rs:6` | - |  |
| `match_sql_like` | 2 | `crates/locus-geospatial-codecs/src/telemetry/query.rs:110` | - | Simple SQL LIKE pattern matcher (% matches any sequence, _ matches single character). |
| `impl Ul::matches` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:16` | - | Compare ignoring byte 7 (the registry version, which varies between writers). |
| `impl SmartRule::matches` | 2 | `crates/radiance-camera-raw/src/metadata.rs:219` | - |  |
| `matches_brute_force_for_every_small_mask` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:20` | physics |  |
| `matches_ffmpeg_decoder` | 0 | `crates/audio-aac-codec/tests/decoder_oracle.rs:52` | - |  |
| `impl BindMap::materialize_cpu_buf` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:913` | - |  |
| `impl TransientBindMap<'a>::materialize_external_image_for_render_pass` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1073` | - |  |
| `impl TransientBindMap<'a>::materialize_gpu_buf_for_indirect` | 6 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1057` | - |  |
| `math` | 3 | `crates/discourse-academic-tex/src/typeset.rs:485` | - | Typeset a math fragment to positioned items. |
| `math_shift_and_brackets` | 0 | `crates/discourse-academic-tex/src/texparse/lex.rs:183` | - |  |
| `impl Matrix::matmul` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:190` | - |  |
| `impl Transform::matrix` | 1 | `crates/profile-authoring/src/assembly.rs:28` | 3d, numerics |  |
| `impl Stage::matrix3` | 1 | `crates/color-transforms/src/icc/pipeline.rs:44` | - |  |
| `matrix_from_affine` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1315` | numerics |  |
| `impl CsrMatrix::matvec` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/sparse.rs:70` | numerics | Matrix-vector multiplication y = A * x. |
| `max` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:267` | - |  |
| `impl Vec2::max` | 2 | `crates/drafting-geometry/src/vec.rs:85` | 2d |  |
| `impl Grid::max` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:243` | - |  |
| `impl Vec3::max` | 2 | `crates/surface-pbr-studio/src/math.rs:153` | - |  |
| `impl Tick::max` | 2 | `crates/timeline-editing/src/timebase.rs:90` | - |  |
| `impl Vec2::max` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:131` | - |  |
| `max_abs` | 1 | `crates/locus-parametric-solver/src/solver/lm.rs:18` | - |  |
| `max_abs_diff` | 2 | `crates/audio-aac-codec/tests/common/mod.rs:247` | - |  |
| `max_corner_radius` | 2 | `crates/vector-locus-geometry/src/shapes.rs:51` | - | The largest corner radius a `w` × `h` rectangle draws: half its shorter side. |
| `impl Sps::max_dpb_frames` | 1 | `crates/video-h264-codec/src/params.rs:484` | - | MaxDpbFrames from Table A-1 (and the VUI max_dec_frame_buffering when present). |
| `max_error` | 3 | `crates/vector-locus-pathops/src/pathops/fit.rs:157` | - |  |
| `max_error` | 3 | `crates/vector-radiance-trace/src/fit.rs:544` | - |  |
| `impl Sps::max_frame_num` | 1 | `crates/video-h264-codec/src/params.rs:460` | - |  |
| `impl MasteringDisplay::max_nits` | 1 | `crates/media-isobmff/src/codec.rs:71` | - |  |
| `impl Sps::max_num_reorder_frames` | 1 | `crates/video-h264-codec/src/params.rs:516` | - | Number of frames that may precede a frame in decoding order and follow it in output order. |
| `impl Corner::max_radius` | 1 | `crates/vector-locus-geometry/src/corners.rs:73` | - | The largest radius the corner draws: its cut then reaches halfway along the shorter side, |
| `impl SupportedStreamConfigRange::max_sample_rate` | 1 | `crates/audio-device-cpal/src/lib.rs:620` | - |  |
| `impl Encoder::max_sfb` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:349` | - |  |
| `impl MepCategory::may_stand` | 1 | `crates/manifold-bim-csg/src/parametric.rs:260` | - |  |
| `impl SurfaceRenderer<'s>::maybe_blit_and_present` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:306` | - | Present the texture to the surface. If there is an intermediate texture, this first blits |
| `impl WebGlPrograms::maybe_create_atlas_textures` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1670` | - | Create any newly allocated atlas textures. |
| `impl Programs::maybe_create_atlas_textures` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2347` | - | Create any newly allocated atlas textures. |
| `maybe_premul_alpha` | 2 | `crates/vector-compute-shaders/shader/fine.wgsl:853` | 2d |  |
| `impl WebGlPrograms::maybe_resize_alphas_tex` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1782` | - | Grow the alpha texture if needed. |
| `impl Programs::maybe_resize_alphas_tex` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2204` | - | Update the alpha texture size if needed. |
| `impl WebGlPrograms::maybe_resize_encoded_paints_tex` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1814` | - | Grow the encoded paints texture if needed. |
| `impl Programs::maybe_resize_encoded_paints_tex` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2239` | - | Update the encoded paints texture size if needed. |
| `impl WebGlPrograms::maybe_resize_filter_data_tex` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1686` | - | Grow the filter data texture if needed. |
| `impl Programs::maybe_resize_filter_tex` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2165` | - |  |
| `impl WebGlPrograms::maybe_resize_gradient_tex` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1848` | - | Grow the gradient texture if needed. |
| `impl Programs::maybe_resize_gradient_tex` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2282` | - | Update the gradient texture size if needed. |
| `impl WebGlPrograms::maybe_update_config_buffer` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1884` | - | Update config buffer if dimensions changed. |
| `impl Programs::maybe_update_config_buffer` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2320` | - | Update config buffer if dimensions changed. |
| `maybe_warn_about_webgl_feature_conflict` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:704` | - |  |
| `maybe_warn_about_webgl_feature_conflict` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:722` | - |  |
| `impl SliceDecoder<'a>::mb` | 1 | `crates/video-h264-codec/src/slicedec.rs:397` | - |  |
| `impl PicState::mb_done` | 2 | `crates/video-h264-codec/src/slicedec.rs:107` | - | Record a decoded macroblock and deblock / publish rows that became final. |
| `impl Frame::mb_h` | 1 | `crates/video-h264-codec/src/picture.rs:150` | - |  |
| `impl SliceDecoder<'a>::mb_mut` | 1 | `crates/video-h264-codec/src/slicedec.rs:401` | - |  |
| `impl FrameHeader::mb_width` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:60` | - |  |
| `mc_chroma` | 9 | `crates/video-h264-codec/src/inter.rs:222` | - | Chroma sample interpolation (8.4.2.2.2) for a bw x bh block at integer position (x, y) with |
| `mc_chroma_win` | 8 | `crates/video-h264-codec/src/inter.rs:245` | - | Chroma interpolation from a prepared (bw + 1) x (bh + 1) window whose first sample is the integer |
| `mc_luma` | 9 | `crates/video-h264-codec/src/inter.rs:97` | - | Luma sample interpolation for a bw x bh block (8.4.2.2.1). (x, y) is the integer sample position |
| `mc_luma_ref` | 9 | `crates/video-h264-codec/src/inter.rs:332` | - | Luma sample interpolation for a bw x bh block. (x, y) is the integer sample position |
| `mc_luma_win` | 8 | `crates/video-h264-codec/src/inter.rs:113` | - | Luma interpolation from a prepared window: `src[0]` is the sample at (x - 2, y - 2) and `ss` the |
| `impl SliceDecoder<'a>::mc_rect` | 5 | `crates/video-h264-codec/src/slicedec.rs:763` | 2d | Motion-compensated prediction of a rectangle (4x4 units) written into the picture. |
| `impl Encoder::mdct` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:356` | - |  |
| `mdct_long` | 0 | `crates/audio-aac-codec/src/mdct.rs:137` | - | Cached transforms for 1024 and 128 lines. |
| `mdct_short` | 0 | `crates/audio-aac-codec/src/mdct.rs:141` | - |  |
| `mduration` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:485` | - |  |
| `impl Bin::mean` | 1 | `crates/vector-radiance-trace/src/quantize.rs:149` | - |  |
| `impl Body::mean_radius_m` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:145` | - |  |
| `mean_size` | 1 | `crates/vector-locus-pathops/src/effects/util.rs:44` | 2d |  |
| `mean_triangle_edge_length` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:923` | graph |  |
| `impl LightTreeNode::measure` | 1 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:156` | - |  |
| `measure_cell_text` | 2 | `crates/layout-paged-typesetter/src/table_layout/measure.rs:112` | - |  |
| `measure_table_columns` | 2 | `crates/layout-paged-typesetter/src/table_layout/measure.rs:54` | - | Measures the (min_width, max_width) for each column in points. |
| `measured_joint_without_an_assertion_id_is_rejected` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:559` | - |  |
| `impl Profile::media_white` | 1 | `crates/color-transforms/src/icc/profile.rs:209` | - |  |
| `median` | 3 | `crates/video-h264-codec/src/slicedec.rs:260` | - |  |
| `median_cut` | 3 | `crates/vector-radiance-trace/src/quantize.rs:325` | - |  |
| `impl SliceDecoder<'a>::median_pred` | 4 | `crates/video-h264-codec/src/slicedec.rs:510` | - |  |
| `impl CeltDecoder::mem_len` | 1 | `crates/audio-opus-codec/src/celt/mod.rs:117` | - |  |
| `membrane` | 3 | `crates/kinetics-garment-draping/src/energy.rs:64` | - | Nonlinear orthotropic membrane energy on a single triangle. |
| `membrane_element` | 3 | `crates/kinetics-garment-draping/src/energy.rs:116` | - |  |
| `mep_run_classification` | 0 | `crates/manifold-bim-csg/src/parametric.rs:359` | - |  |
| `impl LightTreeMeasure::merge` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:53` | - |  |
| `impl RoundBindings::merge` | 2 | `crates/vector-hybrid-renderer/src/target.rs:205` | - | Try to merge this binding with the other one. |
| `merge_hdr` | 4 | `crates/radiance-camera-raw/src/fusion.rs:35` | - | Multi-exposure bracket HDR radiance map fusion with motion deghosting. |
| `merge_pair` | 3 | `crates/vector-locus-pathops/src/pathops/edit.rs:242` | - |  |
| `impl ChangeDag::merge_remote` | 2 | `crates/kinetics-schedule-cpm/src/change_dag.rs:250` | - |  |
| `merge_remote_idempotent` | 0 | `crates/kinetics-schedule-cpm/src/change_dag.rs:353` | - |  |
| `impl AssetHost<'_>::mesh` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:94` | computational geometry | Retrieve a reference to a `Mesh` artifact from a production handle. |
| `mesh` | 2 | `crates/profile-authoring/src/geometry.rs:52` | computational geometry |  |
| `impl AssetHost<'_>::mesh_budget` | 3 | `crates/asset-production-vibe/src/geometry.rs:313` | computational geometry |  |
| `mesh_golden_checksum` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:1454` | computational geometry |  |
| `impl SigVal::meta` | 1 | `crates/kinetics-block-simulation/src/signal.rs:122` | - |  |
| `metadata_names_in_pixel_payload_do_not_reject_valid_video` | 0 | `crates/video-h264-preview/src/lib.rs:509` | 2d |  |
| `metadata_qt_and_ilst` | 0 | `crates/media-isobmff/src/demux.rs:1020` | - |  |
| `impl MaterialPreset::metal` | 3 | `crates/surface-pbr-studio/src/material.rs:90` | - |  |
| `impl TempoMap::meter_at_tick` | 2 | `crates/audio-timebase/src/tempo.rs:213` | - | Active meter at `tick`. |
| `meter_budget_exhaustion` | 0 | `crates/manifold-bim-csg/src/csg.rs:470` | - |  |
| `meter_change_counts_bars` | 0 | `crates/audio-timebase/src/tempo.rs:399` | - |  |
| `impl TempoMap::meters` | 1 | `crates/audio-timebase/src/tempo.rs:138` | - |  |
| `micros_to_stream_instant` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:399` | - |  |
| `mid` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:56` | - |  |
| `impl Line::mid` | 1 | `crates/drafting-geometry/src/curve.rs:22` | 2d |  |
| `impl Segment::mid` | 1 | `crates/drafting-geometry/src/curve.rs:338` | 2d |  |
| `impl Vec2::mid` | 2 | `crates/drafting-geometry/src/vec.rs:68` | 2d |  |
| `impl Arc::mid_point` | 1 | `crates/drafting-geometry/src/curve.rs:77` | 2d |  |
| `min` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:276` | - |  |
| `impl Vec2::min` | 2 | `crates/drafting-geometry/src/vec.rs:82` | 2d |  |
| `impl Vec3::min` | 2 | `crates/surface-pbr-studio/src/math.rs:144` | - |  |
| `impl Tick::min` | 2 | `crates/timeline-editing/src/timebase.rs:94` | - |  |
| `impl Vec2::min` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:127` | - |  |
| `impl SupportedStreamConfigRange::min_sample_rate` | 1 | `crates/audio-device-cpal/src/lib.rs:616` | - |  |
| `min_secs` | 2 | `crates/audio-timebase/src/transport.rs:110` | - |  |
| `mini_and_draft_costs` | 0 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:117` | - |  |
| `minifies_root_and_imported_declarations` | 0 | `crates/vector-hybrid-shaders/src/minify.rs:341` | - |  |
| `minify_code` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:212` | 3d | Removes comments and optional whitespace while preserving shader token boundaries. |
| `minify_wgsl` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:31` | - | Renames every non-entry-point declaration in linked WGSL and removes redundant whitespace. |
| `minify_whitespace` | 2 | `crates/vector-hybrid-shaders/src/minify.rs:49` | 3d | Removes comments and whitespace that are unnecessary between shader tokens. |
| `impl Ellipse::minor` | 1 | `crates/drafting-geometry/src/curve.rs:241` | 2d |  |
| `mirr` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:314` | - |  |
| `mirror` | 2 | `crates/color-transforms/src/icc/curve.rs:90` | - |  |
| `impl Mat3::mirror` | 2 | `crates/drafting-geometry/src/mat.rs:43` | 2d | Reflection across the line through `p1`, `p2`. |
| `impl Vec2::mirror` | 3 | `crates/drafting-geometry/src/vec.rs:89` | 2d | Mirror across the infinite line through `a` and `b`. |
| `mirror_joint` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:464` | - |  |
| `mirror_matrix` | 0 | `crates/drafting-geometry/src/tests.rs:25` | numerics |  |
| `missing_layer_mask_means_full_coverage_and_returns_replacement_bytes` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:20` | - |  |
| `mitc4_shear_strains` | 5 | `crates/kinetics-continuum-multiphysics/src/fem/structural.rs:88` | graph | MITC4 (Mixed Interpolation of Tensorial Components) 4-node shell element. |
| `mitre_corner` | 4 | `crates/manifold-bim-csg/src/parametric.rs:130` | computational geometry | Calculates mitred corner intersection between two straight walls meeting at an angle. |
| `mitre_corner_calculation` | 0 | `crates/manifold-bim-csg/src/parametric.rs:341` | - |  |
| `impl Vec2::mix` | 3 | `crates/vector-compute-shaders/src/cpu/util.rs:109` | - |  |
| `mix_dab` | 3 | `crates/raster-layer-mask-adapter/src/mixer.rs:75` | 2d | Simulates a single dab of the wet mixer brush over a sampled canvas pixel. |
| `impl AssetHost<'_>::mix_tracks` | 2 | `crates/asset-production-vibe/src/acoustics_ops.rs:98` | numerics | Multi-track audio mixer with linear fade envelopes and master gain. |
| `mixed_ltr_rtl_reordering` | 0 | `crates/vector-glyph-shaping/src/bidi.rs:189` | - |  |
| `mlfn` | 4 | `crates/geodesy-spatial-analysis/src/geodesy.rs:488` | - |  |
| `mlfn_coefficients` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:478` | - |  |
| `mluc` | 1 | `crates/color-transforms/src/icc/write.rs:19` | - |  |
| `mod_fn` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:49` | - |  |
| `mode` | 0 | `crates/audio-opus-codec/src/celt/mod.rs:24` | - |  |
| `impl CalligraphicBrush::modes` | 1 | `crates/vector-surface-brush/src/calligraphic.rs:54` | - |  |
| `impl ScatterBrush::modes` | 1 | `crates/vector-surface-brush/src/scatter.rs:46` | - |  |
| `modify_layer_mask` | 6 | `crates/raster-layer-mask-adapter/src/lib.rs:106` | - | Transform the current U8 layer mask and return replacement bytes only after |
| `mono` | 2 | `crates/audio-aac-codec/tests/encoder_oracle.rs:15` | - |  |
| `mono_and_surround` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:168` | - |  |
| `monochrome_contrast_preset` | 1 | `crates/asset-production-vibe/src/radiance_color.rs:31` | - |  |
| `monte_carlo_simulation` | 0 | `crates/kinetics-schedule-cpm/src/schedule.rs:865` | - |  |
| `month_depreciation` | 6 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:56` | - | Calculates depreciation for one calendar month with residual value clamp. |
| `more_rbsp` | 0 | `crates/bitstream-codec/src/lib.rs:398` | - |  |
| `impl BitReader<'a>::more_rbsp_data` | 1 | `crates/bitstream-codec/src/lib.rs:149` | - | H.264/HEVC `more_rbsp_data()`: true if there is data before the rbsp trailing bits. |
| `morph_pairs` | 2 | `crates/layout-presentation-deck/src/morph/match_pairs.rs:16` | - | Pairs of (shape on `a`, shape on `b`) that Morph animates from one to the other: |
| `morph_xfrm` | 3 | `crates/layout-presentation-deck/src/morph/interpolate.rs:42` | - | Interpolate a box from `a` to `b` at `t` (0..1): |
| `morphism_to_json` | 1 | `crates/asset-production-vibe/src/ontology/mod.rs:284` | - | Serializes a single `AgenticMorphismSpec` including its `OntologicalContract` and `NQuin48` parity hashes. |
| `morphisms_json` | 1 | `crates/asset-production-vibe/src/ontology/mod.rs:346` | semantics | Serializes morphisms filtered by optional `category` (`synth.ontology.morphisms`). |
| `mosaic` | 3 | `crates/vector-radiance-trace/src/mosaic.rs:7` | - | Average colour (straight RGBA) of each of `cols` x `rows` tiles covering `img`, row by row. |
| `motion_bs` | 2 | `crates/video-h264-codec/src/deblock.rs:39` | - |  |
| `motor_axis_angle` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:38` | - |  |
| `impl GeodesicRuler::move_point` | 3 | `crates/drafting-geometry/src/geodesic_ruler.rs:182` | - |  |
| `impl OutlinePen for BezPathOutline::move_to` | 3 | `crates/vector-compute-renderer/src/scene.rs:1076` | - |  |
| `movie_duration` | 2 | `crates/media-isobmff/src/mux.rs:173` | - |  |
| `movie_inspection_and_random_access_h264` | 0 | `crates/video-h264-preview/src/lib.rs:786` | - |  |
| `movie_prores_playback` | 0 | `crates/video-h264-preview/src/lib.rs:804` | - |  |
| `mp4_box` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:597` | - |  |
| `mp4_full_box` | 4 | `crates/asset-production-vibe/src/chronicle_ops.rs:606` | - |  |
| `mspdi_and_xer_export` | 0 | `crates/kinetics-schedule-cpm/src/format.rs:777` | - |  |
| `mth` | 1 | `crates/discourse-academic-tex/src/catalog.rs:75` | - |  |
| `impl Mul<T> for SampleRate::mul` | 2 | `crates/audio-device-cpal/src/lib.rs:204` | - |  |
| `impl Cpx::mul` | 2 | `crates/audio-opus-codec/src/celt/mdct.rs:15` | - |  |
| `mul` | 2 | `crates/color-transforms/src/icc/math.rs:18` | - |  |
| `mul` | 2 | `crates/color-transforms/src/spaces.rs:86` | - |  |
| `impl Mat4::mul` | 2 | `crates/drafting-geometry/src/mat.rs:117` | 3d |  |
| `impl Mul<f64> for Vec2::mul` | 2 | `crates/drafting-geometry/src/vec.rs:126` | 2d |  |
| `impl Mul<f64> for Vec3::mul` | 2 | `crates/drafting-geometry/src/vec.rs:193` | 3d |  |
| `impl Complex32::mul` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:57` | - |  |
| `impl Mul<f64> for DVec2::mul` | 2 | `crates/kinetics-garment-draping/src/math.rs:75` | - |  |
| `impl Mul<f64> for DVec3::mul` | 2 | `crates/kinetics-garment-draping/src/math.rs:189` | - |  |
| `impl Mul for Jet<N>::mul` | 2 | `crates/kinetics-garment-draping/src/math.rs:341` | - |  |
| `mul` | 2 | `crates/manifold-bim-csg/src/csg.rs:20` | - |  |
| `mul` | 2 | `crates/radiance-color-grading/src/spaces.rs:91` | - |  |
| `impl Mul<f32> for Vec2::mul` | 2 | `crates/surface-pbr-studio/src/math.rs:71` | - |  |
| `impl Mul<f32> for Vec3::mul` | 2 | `crates/surface-pbr-studio/src/math.rs:217` | - |  |
| `impl Mul<Vec3> for Vec3::mul` | 2 | `crates/surface-pbr-studio/src/math.rs:225` | - |  |
| `impl Mul<f32> for Vec2::mul` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:41` | - |  |
| `impl Mul<Vec2> for f32::mul` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:63` | 2d |  |
| `impl Mul for Transform::mul` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:184` | - |  |
| `mul_2d` | 2 | `crates/manifold-bim-csg/src/parametric.rs:31` | - |  |
| `impl Tick::mul_ratio` | 3 | `crates/timeline-editing/src/timebase.rs:80` | - | Floor-scale by a rational factor. A zero divisor maps to zero, matching |
| `mul_vec` | 2 | `crates/color-transforms/src/spaces.rs:96` | - |  |
| `mul_vec` | 2 | `crates/radiance-color-grading/src/spaces.rs:101` | - |  |
| `multi_entity_elimination_and_consolidation` | 0 | `crates/chronicle-fiduciary-ledger/src/consolidation.rs:136` | - |  |
| `multiple_resolves_per_frame` | 0 | `crates/gpu-command-profiler/tests/src/multiple_resolves_per_frame.rs:5` | - |  |
| `multithreaded_scopes` | 0 | `crates/gpu-command-profiler/tests/src/interleaved_command_buffer.rs:66` | - |  |
| `music_aoa_spectrum` | 4 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:69` | - | Evaluates MUSIC pseudospectrum over angular search grid (-90..+90 degrees). |
| `mutate_all` | 3 | `crates/media-isobmff/tests/fuzz.rs:75` | - |  |
| `mutated_access_units_never_panic` | 0 | `crates/audio-aac-codec/tests/robustness.rs:24` | - |  |
| `mutated_ffmpeg_fixtures_never_panic` | 0 | `crates/media-isobmff/tests/fuzz.rs:142` | - |  |
| `mutated_synthetic_files_never_panic` | 0 | `crates/media-isobmff/tests/fuzz.rs:134` | - |  |
| `impl AssetHost<'_>::mux_mp4` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:29` | - |  |
| `mv_far` | 2 | `crates/video-h264-codec/src/deblock.rs:34` | - |  |
| `impl SliceDecoder<'a>::mvp` | 8 | `crates/video-h264-codec/src/slicedec.rs:490` | - | Luma motion vector prediction (8.4.1.3). `shape`: 1 = 16x8, 2 = 8x16, 0 = other. |
| `impl GpuFilterData::n_decimations` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:331` | - | Returns the number of decimation levels encoded in the header. |
| `trait Block::n_outputs` | 1 | `crates/kinetics-block-simulation/src/block.rs:57` | - |  |
| `impl Block for Integrator::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:29` | - |  |
| `impl Block for Derivative::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:95` | - |  |
| `impl Block for PidController::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:159` | - |  |
| `impl Block for TransferFcn::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:235` | - |  |
| `impl Block for TransportDelay::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:326` | - |  |
| `impl Block for UnitDelay::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:32` | - |  |
| `impl Block for ZeroOrderHold::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:89` | - |  |
| `impl Block for Gain::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:29` | - |  |
| `impl Block for Sum::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:83` | - |  |
| `impl Block for Product::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:145` | - |  |
| `impl Block for Saturation::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:199` | - |  |
| `impl Block for DeadZone::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:245` | - |  |
| `impl Block for Switch::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:295` | - |  |
| `impl Block for Relational::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:345` | - |  |
| `impl Block for LogicalOperator::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:405` | - |  |
| `impl Block for Abs::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:464` | - |  |
| `impl Block for Mux::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:27` | - |  |
| `impl Block for Demux::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:72` | - |  |
| `impl Block for Scope::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:45` | - |  |
| `impl Block for ToWorkspace::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:92` | - |  |
| `impl Block for Constant::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:26` | - |  |
| `impl Block for Step::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:69` | - |  |
| `impl Block for Sine::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:115` | - |  |
| `impl Block for Ramp::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:159` | - |  |
| `impl Block for Clock::n_outputs` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:199` | - |  |
| `trait Block::n_states` | 1 | `crates/kinetics-block-simulation/src/block.rs:81` | - |  |
| `impl Block for Integrator::n_states` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:52` | - |  |
| `impl Block for PidController::n_states` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:187` | - |  |
| `impl Block for TransferFcn::n_states` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:282` | - |  |
| `nal` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:464` | - |  |
| `nal` | 3 | `crates/video-h264-codec/src/synth_tests.rs:15` | - |  |
| `nal` | 2 | `crates/video-h264-preview/src/lib.rs:484` | - |  |
| `nal_body` | 3 | `crates/video-h264-preview/src/synth.rs:20` | - |  |
| `impl Decoder::nal_length_size` | 1 | `crates/video-h264-codec/src/decoder.rs:216` | - | Length of NAL length prefixes (from avcC), or None for Annex-B input. |
| `impl HevcConfig::nalus_of` | 2 | `crates/media-isobmff/src/codec.rs:412` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:64` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:301` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:53` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:61` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:78` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:144` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:89` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:165` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:68` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:143` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:145` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:44` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:275` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:57` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:282` | - |  |
| `impl Device::name` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:84` | - |  |
| `impl DeviceTrait for Device::name` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:142` | - |  |
| `trait DeviceTrait::name` | 1 | `crates/audio-device-cpal/src/traits.rs:102` | - | The human-readable name of the device. |
| `impl Parser<'_>::name` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:142` | - |  |
| `impl ConstraintKind::name` | 1 | `crates/locus-parametric-solver/src/solver/residual.rs:42` | - |  |
| `impl CodecConfig::name` | 1 | `crates/media-isobmff/src/codec.rs:212` | - | A short human-readable codec name. |
| `impl Codec::name` | 1 | `crates/media-ogg/src/lib.rs:148` | - |  |
| `impl ScopeKind::name` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:40` | - |  |
| `name_string` | 4 | `crates/discourse-academic-tex/src/texpdf.rs:169` | - |  |
| `impl Unit::named` | 1 | `crates/vector-locus-geometry/src/units.rs:138` | - | Find a unit by label, suffix, or key. |
| `narrow` | 1 | `crates/video-h264-codec/src/slice.rs:504` | - |  |
| `nastran_deck_contains_bulk_and_grid` | 0 | `crates/kinetics-continuum-multiphysics/src/interchange/nastran.rs:58` | - |  |
| `impl AssetHost<'_>::navigate_funnel_orca` | 2 | `crates/asset-production-vibe/src/kinetics_sim/nav_nla_zones.rs:11` | - | 3D NavMesh Portal Pathfinding + Simple Stupid Funnel + 3D ORCA Avoidance. |
| `impl SliceDecoder<'a>::nb_abc` | 5 | `crates/video-h264-codec/src/slicedec.rs:477` | - | Returns (A, B, C) neighbour motion for a partition at (x, y) of width w (4x4 units), with C |
| `impl SliceDecoder<'a>::nb_motion` | 4 | `crates/video-h264-codec/src/slicedec.rs:445` | - | Motion data of the neighbouring 4x4 block at (x, y) (4x4 units, relative to the current MB). |
| `impl SliceDecoder<'a>::nc_chroma` | 3 | `crates/video-h264-codec/src/slicedec.rs:431` | - |  |
| `impl SliceDecoder<'a>::nc_luma` | 2 | `crates/video-h264-codec/src/slicedec.rs:418` | - | nC for a luma 4x4 block (raster index) - 9.2.1. |
| `impl Vec2::near` | 3 | `crates/drafting-geometry/src/vec.rs:74` | 2d |  |
| `near` | 2 | `crates/vector-locus-geometry/src/corners.rs:196` | - |  |
| `near_tangent_long_line_does_not_invent_a_circle_hit` | 0 | `crates/drafting-geometry/src/tests.rs:304` | - |  |
| `near_uses_finite_nonnegative_tolerance_without_squaring` | 0 | `crates/drafting-geometry/src/vec.rs:209` | - |  |
| `impl ArcPath::nearest` | 2 | `crates/vector-locus-geometry/src/arc.rs:60` | - | The distance along the path of its point nearest `p`. |
| `impl PathData::nearest` | 2 | `crates/vector-locus-geometry/src/path.rs:469` | - | Nearest point on any segment: (subpath, segment, t, point, distance). |
| `nearest_1d` | 2 | `crates/vector-radiance-trace/src/quantize.rs:163` | - |  |
| `nearest_on_edge` | 0 | `crates/vector-locus-geometry/src/path.rs:583` | graph |  |
| `nearest_on_polyline` | 2 | `crates/drafting-geometry/src/curve.rs:292` | 2d | The nearest point to `p` on a polyline (through tessellated points). |
| `nearest_palette_color` | 2 | `crates/raster-spatial-operators/src/octree.rs:190` | - |  |
| `nearest_rgb` | 2 | `crates/vector-radiance-trace/src/quantize.rs:217` | - |  |
| `impl Encoder::needed_for` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:309` | - | Input needed (absolute end) before frame `t` can be encoded (window + transient look-ahead). |
| `impl GpuFilterData::needs_copy_pass` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:339` | - |  |
| `needs_quotes` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:14` | - |  |
| `needs_separator` | 2 | `crates/vector-hybrid-shaders/src/minify.rs:301` | 3d | Reports whether removing whitespace would merge two adjacent shader tokens. |
| `impl Neg for Vec2::neg` | 1 | `crates/drafting-geometry/src/vec.rs:138` | 2d |  |
| `impl Neg for Vec3::neg` | 1 | `crates/drafting-geometry/src/vec.rs:199` | 3d |  |
| `impl Neg for DVec3::neg` | 1 | `crates/kinetics-garment-draping/src/math.rs:181` | - |  |
| `impl Neg for Jet<N>::neg` | 1 | `crates/kinetics-garment-draping/src/math.rs:327` | - |  |
| `impl Neg for Vec3::neg` | 1 | `crates/surface-pbr-studio/src/math.rs:209` | - |  |
| `impl std::ops::Neg for Vec2::neg` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:71` | - |  |
| `impl DrawPassTarget::negate_ndc` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3558` | - |  |
| `negative_positions` | 0 | `crates/audio-timebase/src/tempo.rs:439` | - |  |
| `neighborhood_min_max` | 4 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:151` | - | Determines the local min and max over a 2x2 neighborhood for BPS advection limiting. |
| `impl CowTerrainTileGrid::neighborhood_sampling` | 1 | `crates/terrain-brushes/src/lib.rs:208` | - |  |
| `neighbours` | 3 | `crates/vector-radiance-trace/src/centerline.rs:73` | - |  |
| `impl Grid::neighbours` | 2 | `crates/vector-radiance-trace/src/centerline.rs:170` | - |  |
| `nested_children` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:636` | - |  |
| `nested_children_spilled` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:662` | - |  |
| `nested_islands_and_outside_curves` | 0 | `crates/drafting-geometry/src/region.rs:496` | - |  |
| `nested_parity` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:426` | - |  |
| `nested_scopes` | 2 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:7` | - |  |
| `nested_scopes_no_features` | 0 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:184` | - |  |
| `nested_scopes_timestamp_enabled` | 0 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:175` | - |  |
| `nested_scopes_timestamp_in_encoders_enabled` | 0 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:164` | - |  |
| `nested_scopes_timestamp_in_passes_and_encoder_enabled` | 0 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:140` | - |  |
| `nested_scopes_timestamp_in_passes_enabled` | 0 | `crates/gpu-command-profiler/tests/src/nested_scopes.rs:153` | - |  |
| `net_area` | 1 | `crates/vector-radiance-trace/src/tests.rs:25` | - |  |
| `never_started_preparation_failure_releases_the_actual_guard` | 0 | `crates/pcm-device-output/src/device_slot_tests.rs:5` | - |  |
| `impl AudioBuffer::new` | 3 | `crates/acoustics-daw-session/src/codecs/mod.rs:13` | - |  |
| `impl BiquadFilter::new` | 1 | `crates/acoustics-daw-session/src/dsp/biquad.rs:152` | - |  |
| `impl KWeightingChannel::new` | 1 | `crates/acoustics-daw-session/src/dsp/meter.rs:30` | - |  |
| `impl EbuR128Meter::new` | 2 | `crates/acoustics-daw-session/src/dsp/meter.rs:57` | - |  |
| `impl TruePeakMeter::new` | 0 | `crates/acoustics-daw-session/src/dsp/meter.rs:145` | - |  |
| `impl Oscillator::new` | 3 | `crates/acoustics-daw-session/src/dsp/osc.rs:32` | - |  |
| `impl SpeakerPosition::new` | 4 | `crates/acoustics-daw-session/src/dsp/pan.rs:45` | - |  |
| `impl VbapPanner<'a>::new` | 1 | `crates/acoustics-daw-session/src/dsp/pan.rs:90` | - |  |
| `impl Eq7::new` | 1 | `crates/acoustics-daw-session/src/dsp/plugins.rs:14` | - |  |
| `impl Compressor::new` | 6 | `crates/acoustics-daw-session/src/dsp/plugins.rs:49` | - |  |
| `impl ExpanderGate::new` | 5 | `crates/acoustics-daw-session/src/dsp/plugins.rs:99` | - |  |
| `impl Maximizer::new` | 3 | `crates/acoustics-daw-session/src/dsp/plugins.rs:148` | - |  |
| `impl DeEsser::new` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:197` | - |  |
| `impl ChannelStrip::new` | 1 | `crates/acoustics-daw-session/src/dsp/plugins.rs:225` | - |  |
| `impl FdnReverb::new` | 1 | `crates/acoustics-daw-session/src/dsp/plugins.rs:252` | - |  |
| `impl SubtractiveSynth::new` | 1 | `crates/acoustics-daw-session/src/dsp/plugins.rs:324` | - |  |
| `impl WarpPoint::new` | 2 | `crates/acoustics-daw-session/src/elastic_warp/mod.rs:39` | - |  |
| `impl WsolaTimeStretch::new` | 2 | `crates/acoustics-daw-session/src/elastic_warp/mod.rs:80` | - |  |
| `impl DawTrack::new` | 3 | `crates/acoustics-daw-session/src/mix_session/mod.rs:83` | - |  |
| `impl DawSession::new` | 3 | `crates/acoustics-daw-session/src/mix_session/mod.rs:110` | - |  |
| `impl PdcMixEngine::new` | 0 | `crates/acoustics-daw-session/src/mix_session/mod.rs:127` | - |  |
| `impl CamelotKey::new` | 2 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:14` | - |  |
| `impl ZipArchiveWriter::new` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:49` | - |  |
| `impl SubpathBuilder<'a>::new` | 4 | `crates/asset-production-vibe/src/glyph/mod.rs:148` | - |  |
| `impl Dsu::new` | 1 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:97` | - |  |
| `impl RuleSpec::new` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:91` | - |  |
| `impl WhitegumMeshBuilder::new` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:37` | - |  |
| `impl Decoder::new` | 1 | `crates/audio-aac-codec/src/decoder.rs:58` | - | Create a decoder from an `AudioSpecificConfig`. |
| `impl Encoder::new` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:143` | - |  |
| `impl BandModel::new` | 3 | `crates/audio-aac-codec/src/encoder/psy.rs:34` | - |  |
| `impl Psy::new` | 2 | `crates/audio-aac-codec/src/encoder/psy.rs:98` | - |  |
| `impl ChannelInput::new` | 5 | `crates/audio-aac-codec/src/encoder/quant.rs:47` | - | `spec` is window-major (short windows at stride 128); `thr[g][sfb]` and the grouping come from |
| `impl Fft::new` | 1 | `crates/audio-aac-codec/src/mdct.rs:15` | numerics |  |
| `impl Mdct::new` | 1 | `crates/audio-aac-codec/src/mdct.rs:69` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/alsa/enumerate.rs:12` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/alsa/mod.rs:32` | - |  |
| `impl StreamWorkerContext::new` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:563` | - |  |
| `impl Devices::new` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:174` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/asio/mod.rs:25` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:15` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:50` | - |  |
| `impl Stream::new` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:337` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:63` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:57` | - |  |
| `impl Stream::new` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:890` | - |  |
| `impl AudioObjectPropertyListener::new` | 3 | `crates/audio-device-cpal/src/host/coreaudio/macos/property_listener.rs:24` | - | Attach the provided callback as a audio object property listener. |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:55` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:61` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/jack/mod.rs:33` | - |  |
| `impl LocalProcessHandler::new` | 8 | `crates/audio-device-cpal/src/host/jack/stream.rs:241` | - |  |
| `impl JackNotificationHandler::new` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:422` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/null/mod.rs:27` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/null/mod.rs:33` | - |  |
| `impl CpalInputCallback<I, C>::new` | 2 | `crates/audio-device-cpal/src/host/oboe/input_callback.rs:18` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/oboe/mod.rs:51` | - |  |
| `impl CpalOutputCallback<I, C>::new` | 2 | `crates/audio-device-cpal/src/host/oboe/output_callback.rs:18` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/wasapi/device.rs:872` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:25` | - |  |
| `impl Host::new` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:49` | - |  |
| `impl Devices::new` | 0 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:77` | - |  |
| `impl SupportedStreamConfig::new` | 4 | `crates/audio-device-cpal/src/lib.rs:377` | - |  |
| `impl StreamInstant::new` | 2 | `crates/audio-device-cpal/src/lib.rs:482` | - |  |
| `impl SupportedStreamConfigRange::new` | 5 | `crates/audio-device-cpal/src/lib.rs:596` | - |  |
| `impl Fft::new` | 1 | `crates/audio-opus-codec/src/celt/mdct.rs:32` | numerics |  |
| `impl Imdct::new` | 1 | `crates/audio-opus-codec/src/celt/mdct.rs:96` | - |  |
| `impl CeltDecoder::new` | 2 | `crates/audio-opus-codec/src/celt/mod.rs:64` | - | `sample_rate` is the output rate (48000, 24000, 16000, 12000 or 8000). |
| `impl Mode::new` | 0 | `crates/audio-opus-codec/src/celt/rate.rs:90` | - |  |
| `impl StreamDecoder::new` | 2 | `crates/audio-opus-codec/src/decoder.rs:34` | - | Creates a decoder producing `channels` (1 or 2) at `rate` (8/12/16/24/48 kHz). |
| `impl Decoder::new` | 1 | `crates/audio-opus-codec/src/multistream.rs:24` | - | Creates a 48 kHz decoder from an `OpusHead` packet (Ogg / Matroska `CodecPrivate`). |
| `impl RangeDecoder<'a>::new` | 1 | `crates/audio-opus-codec/src/range.rs:42` | - |  |
| `impl RangeEncoder::new` | 1 | `crates/audio-opus-codec/src/range.rs:282` | - |  |
| `impl Channel::new` | 0 | `crates/audio-opus-codec/src/silk/mod.rs:87` | - |  |
| `impl SilkDecoder::new` | 2 | `crates/audio-opus-codec/src/silk/mod.rs:663` | - |  |
| `impl Resampler::new` | 2 | `crates/audio-opus-codec/src/silk/resampler.rs:73` | - |  |
| `impl SampleRate::new` | 1 | `crates/audio-timebase/src/rate.rs:48` | - |  |
| `impl SampleRange::new` | 2 | `crates/audio-timebase/src/rate.rs:89` | - |  |
| `impl TimeSig::new` | 2 | `crates/audio-timebase/src/tempo.rs:21` | - |  |
| `impl MeterPoint::new` | 3 | `crates/audio-timebase/src/tempo.rs:64` | - |  |
| `impl BarBeat::new` | 3 | `crates/audio-timebase/src/tempo.rs:97` | - |  |
| `impl TempoMap::new` | 3 | `crates/audio-timebase/src/tempo.rs:127` | - |  |
| `impl Timecode::new` | 5 | `crates/audio-timebase/src/timecode.rs:139` | - |  |
| `impl FeetFrames::new` | 2 | `crates/audio-timebase/src/timecode.rs:255` | - |  |
| `impl BitReader<'a>::new` | 1 | `crates/bitstream-codec/src/lib.rs:41` | - |  |
| `impl BitWriter::new` | 0 | `crates/bitstream-codec/src/lib.rs:178` | - |  |
| `impl Reader<'a>::new` | 1 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:15` | - |  |
| `impl Writer::new` | 0 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:97` | - |  |
| `impl Frame::new` | 5 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:29` | - |  |
| `impl TrackSpec::new` | 2 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:47` | - |  |
| `impl MkvWriter<W>::new` | 3 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:147` | - |  |
| `impl Rational::new` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:51` | - |  |
| `impl Cur<'a>::new` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:148` | - |  |
| `impl MxfWriter<W>::new` | 3 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:83` | - |  |
| `impl ChartOfAccounts::new` | 0 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:43` | - |  |
| `impl FiduciaryAuditTrail::new` | 0 | `crates/chronicle-fiduciary-ledger/src/audit.rs:59` | - |  |
| `impl JournalLedger::new` | 1 | `crates/chronicle-fiduciary-ledger/src/journal.rs:74` | - |  |
| `impl RevenueSchedule::new` | 8 | `crates/chronicle-fiduciary-ledger/src/revrec.rs:37` | - | Generates a new multi-period revenue recognition schedule. |
| `impl FlowPara::new` | 1 | `crates/chronicle-office-codecs/src/flow/model.rs:68` | - |  |
| `impl FlowDoc::new` | 0 | `crates/chronicle-office-codecs/src/flow/model.rs:130` | - |  |
| `impl NQuin48::new` | 5 | `crates/chronicle-office-codecs/src/ontology.rs:17` | - |  |
| `impl XlsxWorkbook::new` | 0 | `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:21` | - |  |
| `impl XEl::new` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:23` | - |  |
| `impl ZipPackage::new` | 0 | `crates/chronicle-office-codecs/src/zip/minizip.rs:55` | - |  |
| `impl Evaluator<'h>::new` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:59` | - |  |
| `impl Formula::new` | 1 | `crates/chronicle-tabular-engine/src/calc/model.rs:55` | - |  |
| `impl Sheet::new` | 1 | `crates/chronicle-tabular-engine/src/calc/model.rs:90` | - |  |
| `impl Workbook::new` | 0 | `crates/chronicle-tabular-engine/src/calc/model.rs:159` | - |  |
| `impl Graph::new` | 0 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:47` | - |  |
| `impl Calc::new` | 0 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:82` | - |  |
| `impl CellRef::new` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:20` | - |  |
| `impl RangeRef::new` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:140` | - |  |
| `impl Array::new` | 3 | `crates/chronicle-tabular-engine/src/core/value.rs:92` | - |  |
| `impl OntologicalTriple::new` | 4 | `crates/chronicle-tabular-engine/src/ontology.rs:21` | - |  |
| `impl DtvccWindow::new` | 4 | `crates/chronicle-timeline-interchange/src/captions/cea708.rs:54` | - |  |
| `impl TimelineTrack::new` | 2 | `crates/chronicle-timeline-interchange/src/model.rs:67` | - |  |
| `impl XmlWriter::new` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:74` | - |  |
| `impl XmlNode::new` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:141` | - |  |
| `impl GradeSpace::new` | 2 | `crates/color-transforms/src/grade.rs:41` | - | The grading space of a working space, with HDR White `white_nits` (ignored for SDR). |
| `impl Clut::new` | 3 | `crates/color-transforms/src/icc/clut.rs:23` | graph | Builds a table from node data. |
| `impl GamutCheck::new` | 3 | `crates/color-transforms/src/icc/gamut.rs:19` | - |  |
| `impl Pipeline::new` | 2 | `crates/color-transforms/src/icc/pipeline.rs:137` | - |  |
| `impl Transform::new` | 4 | `crates/color-transforms/src/icc/transform.rs:49` | - |  |
| `impl NQuin48::new` | 5 | `crates/color-transforms/src/ontology.rs:15` | - |  |
| `impl DecodeTable::new` | 4 | `crates/color-transforms/src/transform.rs:85` | - |  |
| `impl ToneMap::new` | 2 | `crates/color-transforms/src/transform.rs:137` | - |  |
| `impl InputTransform::new` | 4 | `crates/color-transforms/src/transform.rs:205` | - | `src_peak_nits` is the mastering/content peak of PQ media when the file signals it. |
| `impl OutputTransform::new` | 2 | `crates/color-transforms/src/transform.rs:303` | - | For export/encode into `target` (Rec709, Rec2100Pq or Rec2100Hlg). |
| `impl TeXDiagnostic::new` | 1 | `crates/discourse-academic-tex/src/review/logrules.rs:25` | - |  |
| `impl Page::new` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:287` | - |  |
| `impl Doc::new` | 0 | `crates/discourse-academic-tex/src/texpdf.rs:306` | - |  |
| `impl ByteRange::new` | 4 | `crates/discourse-pdf-compliance/src/sign/pades.rs:19` | - |  |
| `impl Workspace::new` | 0 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:57` | - |  |
| `impl Parser::new` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:12` | - |  |
| `impl Figure::new` | 0 | `crates/discourse-scientific-matlang/src/plot.rs:140` | - |  |
| `impl Bounds2::new` | 2 | `crates/drafting-geometry/src/bounds.rs:21` | 2d |  |
| `impl Line::new` | 2 | `crates/drafting-geometry/src/curve.rs:13` | 2d |  |
| `impl Arc::new` | 4 | `crates/drafting-geometry/src/curve.rs:65` | 2d |  |
| `impl Circle::new` | 2 | `crates/drafting-geometry/src/curve.rs:140` | 2d |  |
| `impl PolyVertex::new` | 1 | `crates/drafting-geometry/src/curve.rs:426` | 2d |  |
| `impl Vec2::new` | 2 | `crates/drafting-geometry/src/vec.rs:17` | - |  |
| `impl Vec3::new` | 3 | `crates/drafting-geometry/src/vec.rs:154` | - |  |
| `impl UniformLinearArray::new` | 2 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:16` | - |  |
| `impl Complex32::new` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:20` | - |  |
| `impl CsiFrame::new` | 6 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:82` | - |  |
| `impl FresnelZoneGeometry::new` | 3 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:226` | - |  |
| `impl BandpassFilter::new` | 3 | `crates/electromagnetics-rf-sensing/src/vitals.rs:39` | - |  |
| `impl FallDetector::new` | 0 | `crates/electromagnetics-rf-sensing/src/vitals.rs:157` | - |  |
| `impl RgbaColor::new` | 4 | `crates/geodesy-spatial-analysis/src/cartography.rs:32` | - |  |
| `impl Coord::new` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:31` | - |  |
| `impl Extent::new` | 4 | `crates/geodesy-spatial-analysis/src/geodesy.rs:70` | - |  |
| `impl Aeqd::new` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:373` | - |  |
| `impl Sinusoidal::new` | 5 | `crates/geodesy-spatial-analysis/src/geodesy.rs:411` | - |  |
| `impl VariogramParams::new` | 4 | `crates/geodesy-spatial-analysis/src/kriging.rs:30` | - |  |
| `impl SamplePoint::new` | 3 | `crates/geodesy-spatial-analysis/src/kriging.rs:74` | - |  |
| `impl DemGrid::new` | 7 | `crates/geodesy-spatial-analysis/src/morphometry.rs:23` | - |  |
| `impl SpatialGraph::new` | 0 | `crates/geodesy-spatial-analysis/src/network_topology.rs:40` | - |  |
| `impl GfxState::new` | 1 | `crates/gpu-command-profiler/examples/demo.rs:76` | - |  |
| `impl GpuProfiler::new` | 2 | `crates/gpu-command-profiler/src/profiler.rs:58` | - | Creates a new Profiler object. |
| `impl QueryPool::new` | 2 | `crates/gpu-command-profiler/src/profiler.rs:860` | - |  |
| `impl AlphaConvertBlitter::new` | 3 | `crates/gpu-device-context/src/alpha_blitter.rs:90` | - | Create a blitter that applies `conversion` and writes to a target |
| `impl BufferRenderer::new` | 3 | `crates/gpu-device-context/src/buffer_renderer.rs:36` | - | Creates a new render surface for the specified window and dimensions. |
| `impl WGPUContext::new` | 0 | `crates/gpu-device-context/src/lib.rs:116` | - |  |
| `impl SurfaceRenderer<'s>::new` | 4 | `crates/gpu-device-context/src/surface_renderer.rs:145` | - | Creates a new render surface for the specified window and dimensions. |
| `impl Integrator::new` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:15` | - |  |
| `impl Derivative::new` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:81` | - |  |
| `impl PidController::new` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:141` | - |  |
| `impl TransferFcn::new` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:217` | - |  |
| `impl TransportDelay::new` | 1 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:311` | - |  |
| `impl UnitDelay::new` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:16` | - |  |
| `impl ZeroOrderHold::new` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:74` | - |  |
| `impl Gain::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:15` | - |  |
| `impl Sum::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:60` | - |  |
| `impl Product::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:122` | - |  |
| `impl Saturation::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:185` | - |  |
| `impl DeadZone::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:231` | - |  |
| `impl Switch::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:282` | - |  |
| `impl Relational::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:332` | - |  |
| `impl LogicalOperator::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:386` | - |  |
| `impl Abs::new` | 1 | `crates/kinetics-block-simulation/src/blocks/math.rs:452` | - |  |
| `impl Mux::new` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:14` | - |  |
| `impl Demux::new` | 1 | `crates/kinetics-block-simulation/src/blocks/routing.rs:58` | - |  |
| `impl Trace::new` | 0 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:16` | - |  |
| `impl Scope::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:32` | - |  |
| `impl ToWorkspace::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:77` | - |  |
| `impl Constant::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:15` | - |  |
| `impl Step::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:54` | - |  |
| `impl Sine::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:99` | - |  |
| `impl Ramp::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:144` | - |  |
| `impl Clock::new` | 1 | `crates/kinetics-block-simulation/src/blocks/sources.rs:189` | - |  |
| `impl Model::new` | 0 | `crates/kinetics-block-simulation/src/model.rs:30` | - |  |
| `impl Sim::new` | 2 | `crates/kinetics-block-simulation/src/sim.rs:70` | - |  |
| `impl ExplicitIntegrator::new` | 3 | `crates/kinetics-continuum-multiphysics/src/explicit/dynamics.rs:27` | physics |  |
| `impl ElasticIsotropic::new` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:16` | - |  |
| `impl SaintVenantKirchhoff::new` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:93` | - |  |
| `impl NeoHookean::new` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:122` | - |  |
| `impl J2Plasticity::new` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:169` | - |  |
| `impl LbmGrid3D::new` | 4 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:73` | - |  |
| `impl Point2D::new` | 2 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:15` | - |  |
| `impl Point3D::new` | 3 | `crates/kinetics-continuum-multiphysics/src/mesh/tet.rs:16` | - |  |
| `impl CsrMatrix::new` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/sparse.rs:18` | - |  |
| `impl DVec2::new` | 2 | `crates/kinetics-garment-draping/src/math.rs:18` | - |  |
| `impl DVec3::new` | 3 | `crates/kinetics-garment-draping/src/math.rs:100` | - |  |
| `impl ClothMesh::new` | 4 | `crates/kinetics-garment-draping/src/vbd.rs:40` | - |  |
| `impl Keyframe::new` | 2 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:102` | - |  |
| `impl GrayImage::new` | 2 | `crates/kinetics-motion-compositor/src/track/correlation.rs:14` | - |  |
| `impl ChangeEnvelope::new` | 6 | `crates/kinetics-schedule-cpm/src/change_dag.rs:150` | - |  |
| `impl ChangeDag::new` | 0 | `crates/kinetics-schedule-cpm/src/change_dag.rs:188` | - |  |
| `impl Task::new` | 5 | `crates/kinetics-schedule-cpm/src/model.rs:302` | - |  |
| `impl Project::new` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:515` | - |  |
| `impl XorShift64::new` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:632` | - |  |
| `impl LayoutNode::new` | 3 | `crates/layout-box-autolayout/src/frame.rs:39` | - |  |
| `impl AutoLayoutFrame::new` | 1 | `crates/layout-box-autolayout/src/frame.rs:123` | - |  |
| `impl Rect::new` | 4 | `crates/layout-box-autolayout/src/types.rs:110` | - |  |
| `impl Padding::new` | 4 | `crates/layout-box-autolayout/src/types.rs:130` | - |  |
| `impl NodeId::new` | 1 | `crates/layout-nodegraph-compiler/src/compiler/node.rs:12` | - |  |
| `impl NodeNetwork::new` | 0 | `crates/layout-nodegraph-compiler/src/compiler/node.rs:69` | - |  |
| `impl NodeGraphCompiler::new` | 1 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:42` | - |  |
| `impl ProtoNetwork::new` | 1 | `crates/layout-nodegraph-compiler/src/compiler/proto.rs:29` | - |  |
| `impl MathContext::new` | 0 | `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:15` | - |  |
| `impl Parser<'a>::new` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:14` | - |  |
| `impl Math::new` | 1 | `crates/layout-paged-typesetter/src/equation/ast.rs:17` | - |  |
| `impl MRun::new` | 1 | `crates/layout-paged-typesetter/src/equation/ast.rs:70` | - |  |
| `impl Patterns::new` | 0 | `crates/layout-paged-typesetter/src/hyphenation/liang.rs:15` | - |  |
| `impl OntologicalTriple::new` | 4 | `crates/layout-paged-typesetter/src/ontology.rs:20` | - |  |
| `impl ParaDoc::new` | 1 | `crates/layout-paged-typesetter/src/pagination/flow.rs:38` | - |  |
| `impl Typesetter::new` | 1 | `crates/layout-paged-typesetter/src/pagination/flow.rs:71` | - |  |
| `impl Point::new` | 2 | `crates/layout-paged-typesetter/src/pagination/types.rs:12` | - |  |
| `impl Rect::new` | 4 | `crates/layout-paged-typesetter/src/pagination/types.rs:26` | - |  |
| `impl TableCellData::new` | 1 | `crates/layout-paged-typesetter/src/table_layout/measure.rs:15` | - |  |
| `impl Layout::new` | 3 | `crates/layout-presentation-deck/src/model/master.rs:36` | - |  |
| `impl Master::new` | 2 | `crates/layout-presentation-deck/src/model/master.rs:56` | - |  |
| `impl Slide::new` | 2 | `crates/layout-presentation-deck/src/model/master.rs:78` | - |  |
| `impl Run::new` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:30` | - |  |
| `impl Paragraph::new` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:64` | - |  |
| `impl Shape::new` | 3 | `crates/layout-presentation-deck/src/model/shape.rs:171` | - |  |
| `impl Xfrm::new` | 4 | `crates/layout-presentation-deck/src/model/types.rs:58` | - |  |
| `impl Rgba::new` | 4 | `crates/layout-presentation-deck/src/model/types.rs:94` | - |  |
| `impl NQuin48::new` | 5 | `crates/layout-presentation-deck/src/ontology.rs:17` | - |  |
| `impl Point2D::new` | 2 | `crates/locus-geospatial-codecs/src/geometry.rs:13` | - |  |
| `impl Point3D::new` | 3 | `crates/locus-geospatial-codecs/src/geometry.rs:27` | - |  |
| `impl Polygon2D::new` | 2 | `crates/locus-geospatial-codecs/src/geometry.rs:44` | - |  |
| `impl BitReader<'a>::new` | 1 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:42` | - |  |
| `impl WkbReader<'a>::new` | 1 | `crates/locus-geospatial-codecs/src/wkb.rs:15` | - |  |
| `impl Env::new` | 0 | `crates/locus-parametric-solver/src/solver/expr.rs:178` | - |  |
| `impl Handle::new` | 1 | `crates/locus-parametric-solver/src/solver/model.rs:22` | - |  |
| `impl GeomRef::new` | 2 | `crates/locus-parametric-solver/src/solver/model.rs:69` | - |  |
| `impl Built::new` | 4 | `crates/locus-parametric-solver/src/solver/residual.rs:126` | - |  |
| `impl Meter::new` | 2 | `crates/manifold-bim-csg/src/csg.rs:64` | - |  |
| `impl IfcWriter::new` | 0 | `crates/manifold-bim-csg/src/ifc.rs:54` | - |  |
| `impl FourCc::new` | 1 | `crates/media-isobmff/src/bytes.rs:12` | - |  |
| `impl Cur<'a>::new` | 2 | `crates/media-isobmff/src/bytes.rs:71` | - |  |
| `impl BoxBuf::new` | 0 | `crates/media-isobmff/src/bytes.rs:255` | - |  |
| `impl AvcConfig::new` | 3 | `crates/media-isobmff/src/codec.rs:252` | - | Build from parameter sets (NAL units without start codes); profile/level taken from the first SPS. |
| `impl WriterOptions::new` | 1 | `crates/media-isobmff/src/mux.rs:29` | - |  |
| `impl TrackConfig::new` | 2 | `crates/media-isobmff/src/mux.rs:52` | - |  |
| `impl Tables::new` | 0 | `crates/media-isobmff/src/mux.rs:109` | - |  |
| `impl Mp4Writer<W>::new` | 2 | `crates/media-isobmff/src/mux.rs:521` | - | Start a file: writes `ftyp`, a placeholder box and the `mdat` header. |
| `impl FragmentedWriter<W>::new` | 2 | `crates/media-isobmff/src/mux.rs:740` | - |  |
| `impl Rgba8::new` | 3 | `crates/media-test-support/src/golden.rs:26` | - |  |
| `impl SphericalAdjustment::new` | 3 | `crates/multimodal-synthesis-schema/src/stage.rs:14` | - |  |
| `impl Queue::new` | 7 | `crates/pcm-device-output/src/queue.rs:31` | - |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::new` | 3 | `crates/pixel-buffer-output/src/builder.rs:50` | physics, 2d | Create a builder that can be finalized into a [`Pixels`] pixel buffer. |
| `impl SurfaceTexture<W>::new` | 3 | `crates/pixel-buffer-output/src/lib.rs:192` | - | Create a logical texture for a window surface. |
| `impl Pixels<'win>::new` | 3 | `crates/pixel-buffer-output/src/lib.rs:250` | 2d | Create a pixel buffer instance with default options. |
| `impl ScalingRenderer::new` | 8 | `crates/pixel-buffer-output/src/renderers.rs:23` | - |  |
| `impl ScalingMatrix::new` | 3 | `crates/pixel-buffer-output/src/renderers.rs:317` | - |  |
| `impl PixelsRendererOptions::new` | 0 | `crates/pixel-window-render-adapter/src/lib.rs:42` | - |  |
| `impl PixelsWindowRenderer<Renderer>::new` | 0 | `crates/pixel-window-render-adapter/src/lib.rs:91` | - |  |
| `impl GpxTracklog::new` | 0 | `crates/radiance-camera-raw/src/metadata.rs:57` | - |  |
| `impl CatalogJournal::new` | 0 | `crates/radiance-camera-raw/src/metadata.rs:177` | - |  |
| `impl NQuin48::new` | 5 | `crates/radiance-camera-raw/src/ontology.rs:15` | - |  |
| `impl GradeSpace::new` | 2 | `crates/radiance-color-grading/src/grade.rs:28` | - |  |
| `impl AscCdl::new` | 4 | `crates/radiance-color-grading/src/grade.rs:134` | - |  |
| `impl Grid::new` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:226` | - |  |
| `impl Vectorscope::new` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:429` | - |  |
| `impl InputTransform::new` | 2 | `crates/radiance-color-grading/src/transform.rs:86` | - |  |
| `impl OutputTransform::new` | 4 | `crates/radiance-color-grading/src/transform.rs:122` | - |  |
| `impl MixerState::new` | 0 | `crates/raster-layer-mask-adapter/src/mixer.rs:50` | - |  |
| `impl NQuin48::new` | 5 | `crates/raster-layer-mask-adapter/src/ontology.rs:15` | - |  |
| `impl Kernel2D::new` | 3 | `crates/raster-spatial-operators/src/convolve.rs:17` | - |  |
| `impl OctreeNode::new` | 0 | `crates/raster-spatial-operators/src/octree.rs:22` | - |  |
| `impl OctreeQuantizer::new` | 0 | `crates/raster-spatial-operators/src/octree.rs:72` | - |  |
| `impl NQuin48::new` | 5 | `crates/surface-layered-codecs/src/ontology.rs:27` | - |  |
| `impl OntologicalContract::new` | 4 | `crates/surface-layered-codecs/src/ontology.rs:66` | semantics |  |
| `impl PsdDocument::new` | 4 | `crates/surface-layered-codecs/src/psd/codec.rs:130` | - | Creates a blank document with given dimensions and color mode. |
| `impl PbrLayerStack::new` | 0 | `crates/surface-pbr-studio/src/compose.rs:84` | - |  |
| `impl Vec2::new` | 2 | `crates/surface-pbr-studio/src/math.rs:19` | - |  |
| `impl Vec3::new` | 3 | `crates/surface-pbr-studio/src/math.rs:100` | - |  |
| `impl Vec4::new` | 4 | `crates/surface-pbr-studio/src/math.rs:251` | - |  |
| `impl Ray::new` | 2 | `crates/surface-pbr-studio/src/math.rs:275` | 3d |  |
| `impl Bounds::new` | 2 | `crates/surface-pbr-studio/src/math.rs:311` | 3d |  |
| `impl Mesh::new` | 0 | `crates/surface-pbr-studio/src/mesh.rs:27` | - |  |
| `impl RasterImage::new` | 2 | `crates/surface-pbr-studio/src/raster.rs:14` | - |  |
| `impl CowTerrainTileGrid::new` | 3 | `crates/terrain-brushes/adaptation/grid-impl.rs:10` | - |  |
| `impl CowTerrainTileGrid::new` | 3 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:158` | - |  |
| `impl CowTerrainTileGrid::new` | 3 | `crates/terrain-brushes/src/lib.rs:185` | - |  |
| `impl MinHeap::new` | 0 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:69` | - |  |
| `impl FloatGrid::new` | 2 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:41` | - |  |
| `impl TimeRange::new` | 2 | `crates/timeline-editing/src/timebase.rs:107` | - |  |
| `impl FrameRate::new` | 2 | `crates/timeline-editing/src/timebase.rs:148` | - |  |
| `impl ClipSpan::new` | 2 | `crates/timeline-editing/src/transition.rs:12` | - |  |
| `impl DebugRenderer::new` | 3 | `crates/vector-compute-renderer/src/debug/renderer.rs:29` | - |  |
| `impl LinepointsUniforms::new` | 2 | `crates/vector-compute-renderer/src/debug/renderer.rs:355` | - |  |
| `impl LineEndpoint::new` | 2 | `crates/vector-compute-renderer/src/debug/validate.rs:23` | - |  |
| `impl Renderer::new` | 2 | `crates/vector-compute-renderer/src/lib.rs:434` | - | Creates a new renderer for the specified device. |
| `impl BufferProxy::new` | 2 | `crates/vector-compute-renderer/src/recording.rs:236` | - |  |
| `impl ImageProxy::new` | 3 | `crates/vector-compute-renderer/src/recording.rs:263` | - |  |
| `impl Render::new` | 0 | `crates/vector-compute-renderer/src/render.rs:121` | - |  |
| `impl Scene::new` | 0 | `crates/vector-compute-renderer/src/scene.rs:54` | - | Creates a new scene. |
| `impl DrawGlyphs<'a>::new` | 2 | `crates/vector-compute-renderer/src/scene.rs:477` | - | Creates a new builder for encoding a glyph run for the specified |
| `impl RenderContext::new` | 0 | `crates/vector-compute-renderer/src/util.rs:32` | - |  |
| `impl WgpuEngine::new` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:144` | - |  |
| `impl TransientBindMap<'a>::new` | 1 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1041` | - | Create new transient bind map, seeded from external resources |
| `impl CargoWarningAdapter<W>::new` | 1 | `crates/vector-compute-shaders/build.rs:184` | - |  |
| `impl Error::new` | 3 | `crates/vector-compute-shaders/src/compile/mod.rs:57` | - |  |
| `impl ShaderInfo::new` | 3 | `crates/vector-compute-shaders/src/compile/mod.rs:92` | - |  |
| `impl TileState::new` | 1 | `crates/vector-compute-shaders/src/cpu/coarse.rs:38` | - |  |
| `impl Vec2::new` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:77` | - |  |
| `impl ImageCache::new` | 1 | `crates/vector-cpu-render-adapter/src/image_cache.rs:57` | - |  |
| `impl ImageRenderer for VelloCpuImageRenderer::new` | 2 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:32` | - |  |
| `impl VelloCpuScenePainter::new` | 2 | `crates/vector-cpu-render-adapter/src/scene.rs:20` | - |  |
| `impl ImageRenderer for VelloImageRenderer::new` | 2 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:66` | - |  |
| `impl VelloScenePainter<'_, '_>::new` | 1 | `crates/vector-gpu-render-adapter/src/scene.rs:54` | - |  |
| `impl VelloRendererOptions::new` | 0 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:86` | - |  |
| `impl VelloWindowRenderer::new` | 0 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:174` | - |  |
| `impl ImageManager<'a>::new` | 6 | `crates/vector-hybrid-render-adapter/src/scene.rs:75` | - |  |
| `impl VelloHybridScenePainter<'_>::new` | 4 | `crates/vector-hybrid-render-adapter/src/scene.rs:137` | - |  |
| `impl WebGlImageManager<'a>::new` | 3 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:24` | - |  |
| `impl WebGlScenePainter<'s>::new` | 2 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:68` | - |  |
| `impl VelloHybridRendererOptions::new` | 0 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:90` | - |  |
| `impl VelloHybridWindowRenderer::new` | 0 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:161` | - |  |
| `impl GpuBlendInstance::new` | 3 | `crates/vector-hybrid-renderer/src/blend.rs:37` | - |  |
| `impl DrawBuilder<'a, T>::new` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:132` | - |  |
| `impl DrawState<T>::new` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:382` | - |  |
| `impl DrawCase<T>::new` | 2 | `crates/vector-hybrid-renderer/src/draw.rs:606` | - |  |
| `impl LinearKernel::new` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:144` | - |  |
| `impl FilterPassBuilder<'a>::new` | 3 | `crates/vector-hybrid-renderer/src/filter.rs:503` | - |  |
| `impl GradientRampCache::new` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:58` | - | Create a new gradient ramp cache with the specified retained count. |
| `impl PaintResolver<'a>::new` | 2 | `crates/vector-hybrid-renderer/src/paint.rs:81` | - |  |
| `impl ScratchTexture<T>::new` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:104` | - |  |
| `impl WebGlTextureBindings::new` | 0 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:181` | - | Create an empty binding map. |
| `impl WebGlRenderer::new` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:234` | 3d, 2d | Creates a new WebGL2 renderer and its persistent resources. |
| `impl PendingShaderProgram::new` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1365` | 3d |  |
| `impl PendingWebGlPrograms::new` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1471` | - | Starts compiling all programs before performing any blocking status queries. |
| `impl Resource<T>::new` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:36` | - |  |
| `impl TextureBindings::new` | 0 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:96` | - | Create an empty binding map. |
| `impl Renderer::new` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:161` | - | Creates a new renderer and its persistent resources. |
| `impl StripBufferArena::new` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:970` | - |  |
| `impl Programs::new` | 5 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1087` | - |  |
| `impl Resources::new` | 1 | `crates/vector-hybrid-renderer/src/resources.rs:26` | - |  |
| `impl Scene::new` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:232` | - | Create a new render context with the given width and height in pixels. |
| `impl Atlases::new` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:34` | - |  |
| `impl LayerAllocationRequest::new` | 3 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:112` | - |  |
| `impl AllocatedTextureRegion::new` | 3 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:172` | - |  |
| `impl Cursor::new` | 1 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:29` | - |  |
| `impl Scheduler<'a, 'p>::new` | 8 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:296` | - | Create a scheduler. |
| `impl TargetScheduleState<T>::new` | 4 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1017` | - |  |
| `impl FilterTextureRegions::new` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:493` | - |  |
| `impl SceneCase::new` | 2 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:24` | - |  |
| `impl LayerTextureId::new` | 2 | `crates/vector-hybrid-renderer/src/target.rs:89` | - |  |
| `impl DrawPassBindings::new` | 2 | `crates/vector-hybrid-renderer/src/target.rs:107` | - |  |
| `impl FilterPassBindings::new` | 2 | `crates/vector-hybrid-renderer/src/target.rs:122` | - |  |
| `impl BlendPassBindings::new` | 2 | `crates/vector-hybrid-renderer/src/target.rs:166` | - |  |
| `impl RoundBindings::new` | 1 | `crates/vector-hybrid-renderer/src/target.rs:196` | - |  |
| `impl DimensionConstraints::new` | 4 | `crates/vector-hybrid-renderer/src/util.rs:23` | - | Create new constraints with given min/max dimensions |
| `impl RangedSlice<'a, T>::new` | 2 | `crates/vector-hybrid-renderer/src/util.rs:182` | - |  |
| `impl ReflectionMap::new` | 3 | `crates/vector-hybrid-shaders/src/types.rs:27` | - | Create a new `ReflectionMap` given the [`naga`] compile info. |
| `impl ArcPath::new` | 1 | `crates/vector-locus-geometry/src/arc.rs:19` | - |  |
| `impl SubPath::new` | 2 | `crates/vector-locus-geometry/src/path.rs:188` | - |  |
| `impl PathData::new` | 1 | `crates/vector-locus-geometry/src/path.rs:354` | - |  |
| `impl Shape::new` | 3 | `crates/vector-locus-pathops/src/pathops/pathfinder.rs:18` | - |  |
| `impl Raster::new` | 3 | `crates/vector-radiance-trace/src/lib.rs:56` | - |  |
| `impl GenerationalCache<K, T>::new` | 1 | `crates/vector-skia-render-adapter/src/cache.rs:36` | - |  |
| `impl ImageRenderer for SkiaImageRenderer::new` | 2 | `crates/vector-skia-render-adapter/src/image_renderer.rs:20` | - |  |
| `impl MetalBackend::new` | 4 | `crates/vector-skia-render-adapter/src/metal.rs:27` | - |  |
| `impl OpenGLBackend::new` | 4 | `crates/vector-skia-render-adapter/src/opengl.rs:30` | - |  |
| `impl SkiaSceneCache::new` | 0 | `crates/vector-skia-render-adapter/src/scene.rs:45` | - | Create a new `SkiaSceneCache` |
| `impl SkiaScenePainter<'_>::new` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:82` | 2d |  |
| `impl VulkanBackend::new` | 4 | `crates/vector-skia-render-adapter/src/vulkan.rs:79` | - |  |
| `impl SkiaRendererOptions::new` | 0 | `crates/vector-skia-render-adapter/src/window_renderer.rs:44` | - |  |
| `impl SkiaWindowRenderer::new` | 0 | `crates/vector-skia-render-adapter/src/window_renderer.rs:92` | - |  |
| `impl Nib::new` | 3 | `crates/vector-surface-brush/src/calligraphic.rs:149` | - |  |
| `impl Track::new` | 2 | `crates/vector-surface-brush/src/track.rs:34` | - |  |
| `impl Rng::new` | 1 | `crates/vector-surface-brush/src/track.rs:185` | - |  |
| `impl Cabac<'a>::new` | 4 | `crates/video-h264-codec/src/cabac.rs:81` | - | Create an engine over `data` (slice RBSP) starting at bit `pos` (must be byte aligned), |
| `impl Enc::new` | 1 | `crates/video-h264-codec/src/cabac.rs:216` | - |  |
| `impl Decoder::new` | 0 | `crates/video-h264-codec/src/decoder.rs:124` | - | A decoder using all available cores (with the `threads` feature). |
| `impl Dpb<F>::new` | 0 | `crates/video-h264-codec/src/dpb.rs:61` | - |  |
| `impl Planes::new` | 2 | `crates/video-h264-codec/src/picture.rs:17` | - |  |
| `impl Frame::new` | 4 | `crates/video-h264-codec/src/picture.rs:146` | - |  |
| `impl PicState::new` | 1 | `crates/video-h264-codec/src/slicedec.rs:68` | - |  |
| `impl Scratch::new` | 0 | `crates/video-h264-codec/src/slicedec.rs:195` | - |  |
| `impl SliceDecoder<'a>::new` | 6 | `crates/video-h264-codec/src/slicedec.rs:281` | - |  |
| `impl Enc::new` | 2 | `crates/video-h264-codec/src/synth_tests.rs:212` | - |  |
| `impl LevelScale::new` | 1 | `crates/video-h264-codec/src/transform.rs:16` | - |  |
| `impl Matrix::new` | 2 | `crates/video-h264-preview/src/convert.rs:15` | numerics | BT.601 (SD) or BT.709 (HD) coefficients. |
| `impl Frame::new` | 4 | `crates/video-h264-preview/src/lib.rs:52` | - |  |
| `impl Lru<K>::new` | 1 | `crates/video-h264-preview/src/movie.rs:92` | - |  |
| `impl Reader<'a>::new` | 1 | `crates/video-prores-codec/src/bits.rs:18` | - |  |
| `impl Writer::new` | 0 | `crates/video-prores-codec/src/bits.rs:176` | - |  |
| `impl OutConv::new` | 1 | `crates/video-prores-codec/src/decode.rs:43` | - |  |
| `impl Scratch::new` | 0 | `crates/video-prores-codec/src/decode.rs:92` | - |  |
| `impl EncoderConfig::new` | 3 | `crates/video-prores-codec/src/encode.rs:29` | - |  |
| `impl Encoder::new` | 3 | `crates/video-prores-codec/src/encode.rs:56` | - |  |
| `impl Frame::new` | 5 | `crates/video-prores-codec/src/types.rs:171` | - | A black (video-range) frame with the given geometry. |
| `impl WallAxis::new_arc` | 3 | `crates/manifold-bim-csg/src/parametric.rs:70` | - |  |
| `impl Pixels<'win>::new_async` | 3 | `crates/pixel-buffer-output/src/lib.rs:281` | 2d | Asynchronously create a pixel buffer instance with default options. |
| `impl ResourceProxy::new_buf` | 2 | `crates/vector-compute-renderer/src/recording.rs:275` | - |  |
| `impl AcroFormField::new_checkbox` | 4 | `crates/discourse-pdf-compliance/src/forms/model.rs:61` | 2d |  |
| `impl Device::new_device` | 4 | `crates/audio-device-cpal/src/host/jack/device.rs:38` | - |  |
| `impl RenderContext::new_device` | 2 | `crates/vector-compute-renderer/src/util.rs:162` | - | Creates a compatible device handle id. |
| `impl MepGeom::new_duct` | 4 | `crates/manifold-bim-csg/src/parametric.rs:300` | - |  |
| `impl PbrLayer::new_fill` | 4 | `crates/surface-pbr-studio/src/compose.rs:58` | - |  |
| `impl DeviceHandle::new_from_compatible_surface` | 4 | `crates/gpu-device-context/src/lib.rs:33` | - | Creates a `DeviceHandle` with `Device` that's compatible with the specified `Surface` |
| `impl ResourceProxy::new_image` | 3 | `crates/vector-compute-renderer/src/recording.rs:279` | - |  |
| `impl Stream::new_input` | 4 | `crates/audio-device-cpal/src/host/alsa/mod.rs:913` | - |  |
| `impl Stream::new_input` | 4 | `crates/audio-device-cpal/src/host/jack/stream.rs:26` | - |  |
| `impl Stream::new_input` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:99` | - |  |
| `impl TargetScheduleState<LayerTextureRegion>::new_layer` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1053` | - |  |
| `impl Stream::new_output` | 4 | `crates/audio-device-cpal/src/host/alsa/mod.rs:945` | - |  |
| `impl Stream::new_output` | 4 | `crates/audio-device-cpal/src/host/jack/stream.rs:91` | - |  |
| `impl Stream::new_output` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:132` | - |  |
| `impl RecordedDraw::new_path` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:68` | - |  |
| `impl MepGeom::new_pipe` | 3 | `crates/manifold-bim-csg/src/parametric.rs:290` | - |  |
| `impl RecordedDraw::new_rect` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:72` | 2d |  |
| `impl AcroFormField::new_text` | 3 | `crates/discourse-pdf-compliance/src/forms/model.rs:47` | 2d |  |
| `impl WebGlRenderer::new_with` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:243` | 2d | Creates a new WebGL2 renderer and its persistent resources with specific settings. |
| `impl Renderer::new_with` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:166` | - | Creates a new renderer and its persistent resources with specific settings. |
| `impl Scene::new_with` | 3 | `crates/vector-hybrid-renderer/src/scene.rs:241` | - | Create a new render context with a specific SIMD level. |
| `impl CowTerrainTileGrid::new_with_neighborhood_sampling` | 4 | `crates/terrain-brushes/src/lib.rs:190` | - | Explicitly select neighbourhood semantics for new work or known-version replay. |
| `impl GpuProfiler::new_with_tracy_client` | 4 | `crates/gpu-command-profiler/src/profiler.rs:92` | - | Creates a new profiler and connects to a running Tracy client. |
| `newton` | 3 | `crates/vector-locus-pathops/src/pathops/fit.rs:170` | - |  |
| `newton` | 3 | `crates/vector-radiance-trace/src/fit.rs:557` | - |  |
| `newton_raphson_linear_spring_convergence` | 0 | `crates/kinetics-continuum-multiphysics/src/solver/nonlinear.rs:75` | numerics |  |
| `newton_raphson_solve` | 7 | `crates/kinetics-continuum-multiphysics/src/solver/nonlinear.rs:17` | numerics | Standard full Newton-Raphson nonlinear solver for residual R(u, lambda) = lambda * P_ref - F_int(u) = 0. |
| `newton_step` | 2 | `crates/kinetics-garment-draping/src/newton.rs:41` | physics, computational geometry | Executes one implicit Euler step via regularized Newton-PCG. |
| `impl Rng::next` | 1 | `crates/audio-aac-codec/tests/common/mod.rs:19` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/alsa/enumerate.rs:25` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:184` | - | Load drivers and return device |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/enumerate.rs:30` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/enumerate.rs:83` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:378` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:139` | - |  |
| `impl Iterator for SupportedInputConfigs::next` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:148` | - |  |
| `impl Iterator for SupportedOutputConfigs::next` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:157` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:897` | - |  |
| `impl Iterator for Devices::next` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:469` | - |  |
| `impl Lcg::next` | 1 | `crates/audio-opus-codec/src/range.rs:413` | - |  |
| `impl Parser::next` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:55` | - |  |
| `impl Parser::next` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:20` | - |  |
| `impl Parser<'a>::next` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:22` | - |  |
| `impl Parser::next` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:136` | - |  |
| `impl ShowState::next` | 3 | `crates/layout-presentation-deck/src/timeline/show.rs:70` | - |  |
| `impl Iterator for Boxes<'a>::next` | 1 | `crates/media-isobmff/src/bytes.rs:207` | - |  |
| `impl Rng::next` | 1 | `crates/media-isobmff/tests/fuzz.rs:10` | - |  |
| `impl ResourceId::next` | 0 | `crates/vector-compute-renderer/src/recording.rs:16` | - |  |
| `impl BindingIndexIterator::next` | 2 | `crates/vector-compute-shaders/src/compile/msl.rs:64` | - |  |
| `impl DepthCounter::next` | 2 | `crates/vector-hybrid-renderer/src/draw.rs:535` | - |  |
| `impl NameAllocator::next` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:191` | - | Returns the next spreadsheet-style name: `A` through `Z`, then `AA`, `AB`, and so on. |
| `impl CornerKind::next` | 1 | `crates/vector-locus-geometry/src/shapes.rs:41` | - | The kind after this one (cycling through them). |
| `impl TargetScheduleState<T>::next_draw` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1040` | - | Return the earliest point at which another draw can be appended to this target. |
| `impl TargetScheduleState<T>::next_draw_after` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1045` | - | Return the earliest draw point that executes after an external dependency. |
| `impl XorShift64::next_f64` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:645` | - |  |
| `impl GenerationalCache<K, T>::next_gen` | 1 | `crates/vector-skia-render-adapter/src/cache.rs:44` | - |  |
| `impl SkiaSceneCache::next_gen` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:50` | - | Increment the generation on the caches. Should be called once per frame. |
| `impl IfcWriter::next_id` | 1 | `crates/manifold-bim-csg/src/ifc.rs:61` | - |  |
| `impl GridDivision::next_line` | 5 | `crates/audio-timebase/src/grid.rs:185` | - | The first grid line strictly after `line`. |
| `impl Decoder::next_random` | 1 | `crates/audio-aac-codec/src/decoder.rs:444` | - |  |
| `impl Oscillator::next_sample` | 1 | `crates/acoustics-daw-session/src/dsp/osc.rs:48` | - | Generates the next output sample and advances phase. |
| `impl GpuProfiler::next_scope_tree_handle` | 1 | `crates/gpu-command-profiler/src/profiler.rs:558` | - |  |
| `impl XorShift64::next_u64` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:636` | - |  |
| `impl Rng::next_u64` | 1 | `crates/vector-surface-brush/src/track.rs:188` | - |  |
| `impl SttsCursor<'_>::next_value` | 1 | `crates/media-isobmff/src/demux.rs:603` | - |  |
| `nib_support_axes` | 0 | `crates/vector-surface-brush/src/calligraphic.rs:228` | - |  |
| `nice_scale_distance` | 1 | `crates/geodesy-spatial-analysis/src/cartography.rs:310` | - | Computes nice distance for scale bars given a nominal distance. |
| `impl GradeSpace::nits_of_signal` | 2 | `crates/color-transforms/src/grade.rs:111` | - | The cd/m² of a grey of signal `s` (HDR). |
| `impl GradeSpace::nits_of_signal` | 2 | `crates/radiance-color-grading/src/grade.rs:102` | - |  |
| `nl_table` | 1 | `crates/locus-geospatial-codecs/src/telemetry/aircraft.rs:165` | - |  |
| `nlsf2a` | 1 | `crates/audio-opus-codec/src/silk/lpc.rs:256` | - | Converts NLSFs (Q15) to stable Q12 LPC coefficients (`silk_NLSF2A`). |
| `nlsf_decode` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:93` | - | Decodes the stage-1/stage-2 indices into stabilised NLSFs (Q15). |
| `nlsf_stabilize` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:133` | - |  |
| `no_paints` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:651` | - |  |
| `impl SliceDecoder<'a>::no_sub_8x8_lt` | 1 | `crates/video-h264-codec/src/slicedec.rs:1380` | - | Whether transform_size_8x8_flag may be present for an inter MB (NoSubMbPartSizeLessThan8x8 etc.). |
| `no_worse` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:76` | - |  |
| `no_worse` | 2 | `crates/vector-radiance-trace/src/fit.rs:280` | - |  |
| `impl ProtoNetwork::node` | 2 | `crates/layout-nodegraph-compiler/src/compiler/proto.rs:41` | graph |  |
| `impl Bsp::node` | 1 | `crates/manifold-bim-csg/src/csg.rs:203` | graph |  |
| `impl LightTreeNode::node_and_leaf_count` | 1 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:163` | graph |  |
| `impl ElementType::node_count` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:21` | graph |  |
| `impl Bvh::node_count` | 1 | `crates/surface-pbr-studio/src/bvh.rs:54` | graph |  |
| `impl ProtoNetwork::node_mut` | 2 | `crates/layout-nodegraph-compiler/src/compiler/proto.rs:45` | graph |  |
| `impl Clut::nodes` | 1 | `crates/color-transforms/src/icc/clut.rs:37` | - | Number of nodes in the grid. |
| `noise` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:43` | - |  |
| `noise` | 4 | `crates/vector-locus-pathops/src/effects/util.rs:49` | - | Deterministic hash noise in [-1, 1] using splitmix64. |
| `noise_removal_drops_speckles` | 0 | `crates/vector-radiance-trace/src/tests.rs:109` | - |  |
| `noise_removal_fills_pinholes` | 0 | `crates/vector-radiance-trace/src/tests.rs:127` | - |  |
| `impl TimecodeRate::nominal` | 1 | `crates/audio-timebase/src/timecode.rs:69` | - | Nominal frames per second integer base. |
| `nominal` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:355` | - |  |
| `impl Profile::nominal_bits_per_mb` | 1 | `crates/video-prores-codec/src/types.rs:139` | - | Nominal coded bits per 16x16 macroblock (the nominal rate spread over the 8160 |
| `impl Profile::nominal_mbps_1080p30` | 1 | `crates/video-prores-codec/src/types.rs:126` | - | Apple's published nominal data rate at 1920x1080, 29.97 fps, in Mbit/s. |
| `non_default_blend_into_layer` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:299` | - |  |
| `non_drop_round_trip` | 0 | `crates/audio-timebase/src/timecode.rs:304` | - |  |
| `impl DebugLayers::none` | 0 | `crates/vector-compute-renderer/src/debug.rs:82` | - | A `DebugLayers` with no layers enabled. |
| `nonnegative` | 4 | `crates/asset-production-vibe/src/graphics_pack.rs:7` | - |  |
| `impl Matrix::norm` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:250` | - |  |
| `norm2` | 1 | `crates/locus-parametric-solver/src/solver/lm.rs:22` | - |  |
| `norm_adjust4` | 2 | `crates/video-h264-codec/src/tables.rs:73` | - | normAdjust4x4(m, i, j) for raster position `pos` (i = row, j = col). |
| `norm_adjust8` | 2 | `crates/video-h264-codec/src/tables.rs:85` | - | normAdjust8x8(m, i, j) for raster position `pos` (i = row, j = col). |
| `norm_angle` | 1 | `crates/drafting-geometry/src/lib.rs:42` | - | Normalize an angle to `[0, 2π)`. |
| `norm_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:101` | - |  |
| `norm_inv` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:115` | - |  |
| `norm_s_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:91` | - |  |
| `impl Complex32::norm_sq` | 1 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:42` | - |  |
| `norm_title` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:75` | - | Normalized title for duplicate detection. |
| `normal` | 2 | `crates/profile-authoring/src/frames.rs:46` | 3d |  |
| `normal` | 1 | `crates/vector-surface-brush/src/track.rs:29` | 3d, 2d | Left-hand normal in y-down document space (for a path running +x it points +y). |
| `normal_at` | 2 | `crates/vector-locus-pathops/src/effects/distort.rs:14` | 3d, 2d |  |
| `impl AccountClass::normal_balance_is_debit` | 1 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:18` | 3d |  |
| `normalize` | 1 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:380` | - |  |
| `impl RangeDecoder<'a>::normalize` | 1 | `crates/audio-opus-codec/src/range.rs:98` | - |  |
| `impl RangeEncoder::normalize` | 1 | `crates/audio-opus-codec/src/range.rs:303` | - |  |
| `impl DVec3::normalize` | 1 | `crates/kinetics-garment-draping/src/math.rs:129` | - |  |
| `impl Vec3::normalize` | 1 | `crates/surface-pbr-studio/src/math.rs:129` | - |  |
| `impl Vec2::normalize` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:115` | - |  |
| `impl MemorySettings::normalize` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:140` | - |  |
| `normalize` | 2 | `crates/vector-locus-pathops/src/pathops/boolean.rs:549` | - | Normalize path by removing degenerate slivers. |
| `normalize_alpha_mask` | 0 | `crates/multimodal-synthesis-schema/src/stage.rs:140` | - |  |
| `normalize_c` | 3 | `crates/color-transforms/src/lib.rs:173` | - |  |
| `normalize_c` | 3 | `crates/radiance-color-grading/src/matrix.rs:177` | - | Quantized integer sample -> normalized Cb/Cr float (-0.5..0.5 nominal). |
| `normalize_deg` | 1 | `crates/vector-locus-geometry/src/lib.rs:57` | - | Normalize an angle in degrees to (-180, 180]. |
| `normalize_function_name` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:238` | - |  |
| `normalize_mask_bytes` | 4 | `crates/multimodal-synthesis-schema/src/stage.rs:55` | - | Normalizes raw RGBA8 or Grayscale image bytes into a deterministic binary mask (255 = target, 0 = ignore). |
| `normalize_memory_settings_allows_lazy_initial_atlas_allocation` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:260` | - |  |
| `normalize_memory_settings_clamps_atlas_dimensions_to_backend_limits` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:234` | - |  |
| `normalize_memory_settings_clamps_layer_sizes_to_backend_limit` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:302` | - |  |
| `normalize_memory_settings_clamps_minimum_to_maximum` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:279` | - |  |
| `normalize_quat` | 1 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:81` | - |  |
| `normalize_raw` | 3 | `crates/radiance-camera-raw/src/demosaic.rs:80` | 3d, numerics | Normalizes 16-bit raw sensor data to scene-linear f32 in [0.0, 1.0] |
| `normalize_word` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:83` | - |  |
| `normalize_y` | 3 | `crates/color-transforms/src/lib.rs:165` | - | Normalise an integer code value to (Y 0..1, C -0.5..0.5) given bit depth and range. |
| `normalize_y` | 3 | `crates/radiance-color-grading/src/matrix.rs:162` | - | Quantized integer sample -> normalized Y float (0..1 nominal). |
| `impl Vec2::normalized` | 1 | `crates/drafting-geometry/src/vec.rs:50` | 2d | Unit vector, or zero for a zero-length vector. |
| `impl Vec3::normalized` | 1 | `crates/drafting-geometry/src/vec.rs:170` | 3d |  |
| `impl Kernel2D::normalized` | 1 | `crates/raster-spatial-operators/src/convolve.rs:46` | - | Normalize kernel weights so their sum is 1.0 (if sum is non-zero). |
| `impl DrawGlyphs<'a>::normalized_coords` | 2 | `crates/vector-compute-renderer/src/scene.rs:562` | - | Sets the normalized design space coordinates for a variable font instance. |
| `normalized_from_signal` | 3 | `crates/color-transforms/src/log.rs:120` | - | Curve-domain signal → decoder-normalised value (0..1 in the file's range). |
| `impl LogCurve::normalized_from_signal` | 3 | `crates/radiance-color-grading/src/log.rs:111` | - |  |
| `impl SubtractiveSynth::note_off` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:342` | - |  |
| `impl SubtractiveSynth::note_on` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:333` | - |  |
| `impl GridDivision::note_ticks` | 5 | `crates/audio-timebase/src/grid.rs:105` | - |  |
| `trait Host::note_volatile` | 1 | `crates/chronicle-tabular-engine/src/calc/eval.rs:41` | - |  |
| `impl RefKind::noun` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:19` | - |  |
| `trait Host::now_serial` | 1 | `crates/chronicle-tabular-engine/src/calc/eval.rs:39` | - |  |
| `impl Host for PassHost<'_>::now_serial` | 1 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:259` | - |  |
| `nper` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:71` | - |  |
| `npm` | 2 | `crates/color-transforms/src/spaces.rs:78` | numerics | Normalised primary matrix (SMPTE RP 177). |
| `npm` | 2 | `crates/radiance-color-grading/src/spaces.rs:79` | numerics | Normalised primary matrix (SMPTE RP 177). |
| `npv` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:218` | - |  |
| `num` | 1 | `crates/asset-production-vibe/src/lib.rs:498` | - |  |
| `num` | 2 | `crates/chronicle-tabular-engine/src/functions/util.rs:8` | - |  |
| `num` | 3 | `crates/kinetics-block-simulation/src/blocks/params.rs:15` | - |  |
| `num` | 3 | `crates/vector-locus-pathops/src/effects/util.rs:6` | - |  |
| `impl BandModel::num_bands` | 1 | `crates/audio-aac-codec/src/encoder/psy.rs:77` | - |  |
| `impl AudioBuffer::num_channels` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:20` | - |  |
| `impl AudioBuffer::num_frames` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:24` | - |  |
| `impl Part::num_parts` | 1 | `crates/video-h264-codec/src/mbtypes.rs:20` | - |  |
| `impl SubPart::num_parts` | 1 | `crates/video-h264-codec/src/mbtypes.rs:49` | - |  |
| `impl Dpb<F>::num_refs` | 1 | `crates/video-h264-codec/src/dpb.rs:69` | - |  |
| `impl IcsInfo::num_swb` | 2 | `crates/audio-aac-codec/src/ics.rs:61` | - | Number of bands for this window length (`num_swb`). |
| `impl IcsInfo::num_windows` | 1 | `crates/audio-aac-codec/src/ics.rs:53` | - |  |
| `number` | 3 | `crates/asset-production-vibe/src/lib.rs:518` | - |  |
| `number` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:145` | - |  |
| `impl Value::number` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:147` | - |  |
| `impl Parser<'_>::number` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:133` | - |  |
| `impl Parser<'_>::number` | 1 | `crates/manifold-bim-csg/src/step.rs:202` | - |  |
| `number` | 2 | `crates/media-ogg/src/strict_opus.rs:18` | - |  |
| `impl Unit::number` | 2 | `crates/vector-locus-geometry/src/units.rs:109` | - | Format without suffix. |
| `number_to_text` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:197` | - |  |
| `impl Matrix::numel` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:145` | - |  |
| `impl MeterPoint::numerator` | 1 | `crates/audio-timebase/src/tempo.rs:71` | - |  |
| `nums3` | 1 | `crates/radiance-color-grading/src/grade.rs:249` | - |  |
| `occurrences` | 2 | `crates/discourse-academic-tex/src/review/suggest.rs:45` | - |  |
| `impl Xf::ocs` | 1 | `crates/drafting-geometry/src/dxf_reader.rs:104` | - |  |
| `impl Mat4::ocs` | 1 | `crates/drafting-geometry/src/mat.rs:140` | 3d | The "arbitrary axis algorithm" frame for an extrusion direction (DXF OCS → WCS). |
| `ocs_identity_for_z` | 0 | `crates/drafting-geometry/src/tests.rs:199` | - |  |
| `octree_quantize_rgba32f` | 5 | `crates/raster-spatial-operators/src/octree.rs:292` | - | Quantize RGBA32F image to at most `max_colors` using Octree reduction. |
| `octree_quantize_rgba8` | 5 | `crates/raster-spatial-operators/src/octree.rs:211` | - | Quantize RGBA8 image to at most `max_colors` using Octree reduction with optional Floyd-Steinberg dithering. |
| `odd_child` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:458` | - |  |
| `impl Poly::of` | 1 | `crates/manifold-bim-csg/src/csg.rs:88` | - |  |
| `impl Bsp::of` | 2 | `crates/manifold-bim-csg/src/csg.rs:197` | - |  |
| `impl Codec::of` | 1 | `crates/media-ogg/src/lib.rs:124` | - |  |
| `impl OpusTiming::of` | 2 | `crates/media-ogg/src/lib.rs:411` | - | Timing of the Opus stream `s` (`pre_skip` from its `OpusHead`). |
| `impl Corner::of` | 3 | `crates/vector-locus-geometry/src/corners.rs:34` | - | The corner of anchor `i` of `sp`, whose index in the path is `index`; `None` when the |
| `impl Metadata::of_kind` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:125` | - |  |
| `impl ViewFramebuffer::offscreen` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1209` | - | If `use_depth_buffer` is true, the framebuffer must have a valid depth attachment! |
| `impl CellRef::offset` | 3 | `crates/chronicle-tabular-engine/src/core/addr.rs:34` | - | Moved by `(dr, dc)`, `None` when the result leaves the sheet. |
| `impl Line::offset` | 2 | `crates/drafting-geometry/src/curve.rs:49` | - |  |
| `impl Segment::offset` | 2 | `crates/drafting-geometry/src/curve.rs:398` | - | Offset by `d` to the left of travel direction. `None` if an arc collapses. |
| `impl PerspectiveGrid::offset` | 2 | `crates/vector-locus-geometry/src/perspective.rs:146` | 3d | Where `plane` is along its normal (points; 0: default position). |
| `impl CellRef::offset_clamped` | 3 | `crates/chronicle-tabular-engine/src/core/addr.rs:45` | - | Moved by `(dr, dc)` and clamped to the sheet boundaries. |
| `offset_filter` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:183` | - |  |
| `offset_path` | 4 | `crates/vector-locus-pathops/src/pathops/offset.rs:51` | - | Offset path: expands (positive) or contracts (negative) a closed path by `offset`. |
| `offset_path_inflates_rectangle` | 0 | `crates/vector-locus-pathops/src/pathops/offset.rs:93` | - |  |
| `offset_polygon` | 3 | `crates/asset-production-vibe/src/locus_ops.rs:289` | computational geometry, 2d |  |
| `offset_track` | 4 | `crates/vector-surface-brush/src/bristle.rs:113` | - |  |
| `old_dt1` | 4 | `crates/raster-selection-morphology/src/distance/tests.rs:191` | - | 1D squared distance transform (Felzenszwalb & Huttenlocher). |
| `old_edt` | 3 | `crates/raster-selection-morphology/src/distance/tests.rs:229` | 2d | Euclidean distance to the nearest `true` pixel. |
| `old_entries_age_out` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:192` | - |  |
| `on_arc` | 2 | `crates/drafting-geometry/src/intersect.rs:91` | 2d |  |
| `impl oboe::AudioInputCallback for CpalInputCallback<T, C>::on_audio_ready` | 3 | `crates/audio-device-cpal/src/host/oboe/input_callback.rs:67` | - |  |
| `impl oboe::AudioOutputCallback for CpalOutputCallback<T, C>::on_audio_ready` | 3 | `crates/audio-device-cpal/src/host/oboe/output_callback.rs:67` | - |  |
| `impl Corner::on_bisector` | 2 | `crates/vector-locus-geometry/src/corners.rs:85` | - | The point `k` along the bisector, measured in (u + v): the centre of a round cut of radius |
| `impl oboe::AudioInputCallback for CpalInputCallback<T, C>::on_error_after_close` | 3 | `crates/audio-device-cpal/src/host/oboe/input_callback.rs:59` | - |  |
| `impl oboe::AudioOutputCallback for CpalOutputCallback<T, C>::on_error_after_close` | 3 | `crates/audio-device-cpal/src/host/oboe/output_callback.rs:59` | - |  |
| `impl oboe::AudioInputCallback for CpalInputCallback<T, C>::on_error_before_close` | 3 | `crates/audio-device-cpal/src/host/oboe/input_callback.rs:51` | - |  |
| `impl oboe::AudioOutputCallback for CpalOutputCallback<T, C>::on_error_before_close` | 3 | `crates/audio-device-cpal/src/host/oboe/output_callback.rs:51` | - |  |
| `one_child` | 4 | `crates/video-h264-preview/src/lib.rs:114` | - |  |
| `one_sided_fades_keep_their_clip_edge` | 0 | `crates/timeline-editing/src/transition.rs:358` | graph |  |
| `impl Matrix::ones` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:53` | - |  |
| `impl Reader<'a>::only_padding_left` | 1 | `crates/video-prores-codec/src/bits.rs:68` | - | True when every remaining bit is zero (the component's padding), or nothing is left. |
| `onset_pair` | 2 | `crates/vector-glyph-shaping/src/hyphen.rs:95` | - |  |
| `op1a` | 1 | `crates/chronicle-broadcast-containers/src/mxf/labels.rs:30` | - |  |
| `op_precedence` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:131` | - |  |
| `op_rank` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:209` | - |  |
| `impl MorphFrame::opacity_of` | 2 | `crates/layout-presentation-deck/src/morph/transition.rs:37` | - |  |
| `impl Rgba8::opaque` | 1 | `crates/media-test-support/src/golden.rs:34` | - |  |
| `impl Backend for WebGlRendererContext<'_>::opaque_draw_pass` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3241` | - |  |
| `impl Backend for RendererContext<'_>::opaque_draw_pass` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3098` | - |  |
| `trait Backend::opaque_draw_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:27` | - | Execute the opaque pass against the root target with the given strips. If this method is |
| `impl Backend for Recorder::opaque_draw_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:230` | - |  |
| `opaque_reverse_rebases_texture_runs` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:852` | - |  |
| `opaque_reverse_without_texture_runs` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:940` | - |  |
| `opaque_root` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:232` | - |  |
| `opaque_routing` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:979` | - |  |
| `impl ScheduledCase::opaque_x` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:206` | - |  |
| `impl RuleKind::opcode` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:48` | - |  |
| `impl DeviceHandles::open` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:188` | - | Create `DeviceHandles` for `name` and try to open a handle for both |
| `impl XmlWriter::open` | 3 | `crates/chronicle-timeline-interchange/src/xml.rs:91` | - |  |
| `open` | 1 | `crates/media-isobmff/src/demux.rs:256` | - | Parse an MP4/MOV file. Only box headers, `ftyp`, `moov` and `moof` boxes are read. |
| `open` | 1 | `crates/media-ogg/src/lib.rs:206` | - | Read every page and assemble the packets of every logical bitstream. |
| `impl Movie::open` | 1 | `crates/video-h264-preview/src/movie.rs:141` | - | Open a movie held in memory. |
| `open_bounded_opus` | 2 | `crates/media-ogg/src/strict_opus.rs:45` | - | Validate framing, checksums, sequence and timing before invoking the |
| `open_channel_section_retains_wall_void_and_refuses_caller_budget` | 0 | `crates/profile-authoring/src/tests.rs:97` | - |  |
| `open_curves_and_attached_lines_are_not_islands` | 0 | `crates/drafting-geometry/src/region.rs:482` | - |  |
| `impl Parser::open_delim` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:148` | - |  |
| `impl Scheduler<'a, 'p>::open_layer` | 3 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:536` | - | Create an unallocated scheduling view of a recorded layer with the given visible bounds. |
| `impl Scheduler<'a, 'p>::open_root_layer` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:553` | - | Represent the root command stream as an intermediate layer that can be sampled. |
| `open_shape_has_no_boundary` | 0 | `crates/drafting-geometry/src/region.rs:428` | - |  |
| `impl Movie::open_source` | 1 | `crates/video-h264-preview/src/movie.rs:147` | - | Open a movie from any random-access byte source. |
| `open_with_limits` | 2 | `crates/media-isobmff/src/demux.rs:260` | - |  |
| `operation_ranges` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:802` | - |  |
| `impl TextureParity::opposite` | 1 | `crates/vector-hybrid-renderer/src/target.rs:59` | - |  |
| `opt` | 1 | `crates/discourse-academic-tex/src/catalog.rs:57` | - |  |
| `opt_arg` | 2 | `crates/discourse-academic-tex/src/texparse/structure.rs:302` | - |  |
| `opt_num` | 3 | `crates/chronicle-tabular-engine/src/functions/util.rs:13` | - |  |
| `opt_point3` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:52` | - |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/discourse_chronicle_fiduciary.rs:5` | - |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:6` | - |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:6` | - |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:11` | - |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:6` | - |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/surface_pbr_brush.rs:5` | - |  |
| `opt_string` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:45` | - |  |
| `impl AssetHost<'_>::optical_nm_to_audio_hz` | 2 | `crates/asset-production-vibe/src/sound.rs:7` | - | Heterodyne optical wavelength onto A4 (532 nm → 440 Hz) so the field is hearable. |
| `impl Pipeline::optimize` | 1 | `crates/color-transforms/src/icc/pipeline.rs:170` | - |  |
| `optional_point3` | 3 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:357` | - |  |
| `optional_point3` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:48` | - |  |
| `optional_quat4` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:55` | - |  |
| `impl Transform::options` | 1 | `crates/color-transforms/src/icc/transform.rs:90` | - |  |
| `opus_file` | 3 | `crates/media-ogg/src/tests.rs:58` | - | Header pages, then pages of 3 packets each; the last page's granule trims `trim` samples. |
| `opus_head` | 1 | `crates/media-ogg/src/tests.rs:39` | - |  |
| `opus_packet_samples` | 2 | `crates/media-ogg/src/lib.rs:371` | - | Duration of an Opus packet in 48 kHz samples from its TOC (RFC 6716 §3.1); `None` if invalid. |
| `opus_pages_packets_and_timing` | 0 | `crates/media-ogg/src/tests.rs:91` | - |  |
| `opus_toc_durations` | 0 | `crates/media-ogg/src/tests.rs:174` | - |  |
| `oracles_required` | 0 | `crates/media-test-support/src/oracle.rs:94` | - | Whether `FILMCRAFT_REQUIRE_ORACLES` is set to a true value. |
| `ordery` | 1 | `crates/audio-opus-codec/src/celt/bands.rs:147` | - |  |
| `ordinary_kriging_grid` | 5 | `crates/geodesy-spatial-analysis/src/kriging.rs:309` | - | Interpolates a regular raster grid from scattered points using Ordinary Kriging. |
| `orient2d` | 3 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:32` | - | Orientation test: positive if p0 -> p1 -> p2 is counter-clockwise. |
| `impl ArtBrush::orient_art` | 1 | `crates/vector-surface-brush/src/art.rs:134` | 2d |  |
| `oriented_bounds` | 2 | `crates/vector-locus-geometry/src/recognize.rs:208` | 2d |  |
| `our_decode` | 1 | `crates/audio-aac-codec/tests/common/mod.rs:202` | - | Decode an ADTS file with our decoder. |
| `out_as_string` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:400` | - |  |
| `out_dir` | 0 | `crates/media-isobmff/tests/common/mod.rs:32` | - |  |
| `impl Render::out_image` | 1 | `crates/vector-compute-renderer/src/render.rs:635` | - | Get the output image. |
| `impl GamutCheck::out_of_gamut` | 2 | `crates/color-transforms/src/icc/gamut.rs:81` | - |  |
| `outer_face` | 2 | `crates/drafting-geometry/src/region.rs:211` | 3d, 2d, graph | The face around `p`, as vertex indices (counter-clockwise). |
| `impl StairRun::outline` | 2 | `crates/manifold-bim-csg/src/parametric.rs:176` | - |  |
| `outline_stroke` | 5 | `crates/vector-locus-pathops/src/pathops/offset.rs:42` | - | Outline stroke: generate the filled compound path representing the painted stroke of `path`. |
| `outline_stroke_expands_line` | 0 | `crates/vector-locus-pathops/src/pathops/offset.rs:84` | - |  |
| `impl FilterPassBindings::output` | 2 | `crates/vector-hybrid-renderer/src/target.rs:147` | - |  |
| `impl Host::output_device_with_name` | 2 | `crates/audio-device-cpal/src/host/jack/mod.rs:60` | - |  |
| `trait HostTrait::output_devices` | 1 | `crates/audio-device-cpal/src/traits.rs:75` | - | An iterator yielding all `Device`s currently available to the system that support one or more |
| `output_frame_to_puffin` | 2 | `crates/gpu-command-profiler/src/puffin.rs:6` | - | Visualize the query results in a `puffin::GlobalProfiler`. |
| `output_line` | 3 | `crates/vector-compute-shaders/shader/flatten.wgsl:782` | - |  |
| `output_line` | 6 | `crates/vector-compute-shaders/src/cpu/flatten.rs:117` | 2d |  |
| `output_line_with_transform` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:787` | - |  |
| `output_line_with_transform` | 7 | `crates/vector-compute-shaders/src/cpu/flatten.rs:129` | 2d |  |
| `output_meta` | 3 | `crates/video-h264-codec/src/decoder.rs:582` | - |  |
| `impl NodeNetwork::output_node_id` | 1 | `crates/layout-nodegraph-compiler/src/compiler/node.rs:77` | graph |  |
| `impl WorkingSpace::output_space` | 1 | `crates/color-transforms/src/spaces.rs:333` | - | The colour space exports of this sequence are encoded in. |
| `output_stream_worker` | 5 | `crates/audio-device-cpal/src/host/alsa/mod.rs:629` | - |  |
| `output_timestamp` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:599` | - | Produce the output stream timestamp. |
| `output_two_lines_with_transform` | 6 | `crates/vector-compute-shaders/shader/flatten.wgsl:792` | - |  |
| `output_two_lines_with_transform` | 9 | `crates/vector-compute-shaders/src/cpu/flatten.rs:142` | 2d |  |
| `impl Stage::outputs` | 2 | `crates/color-transforms/src/icc/pipeline.rs:36` | - |  |
| `impl Transform::outputs` | 1 | `crates/color-transforms/src/icc/transform.rs:86` | - |  |
| `trait Block::outputs` | 5 | `crates/kinetics-block-simulation/src/block.rs:59` | - |  |
| `impl Block for Integrator::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:39` | - |  |
| `impl Block for Derivative::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:102` | - |  |
| `impl Block for PidController::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:165` | - |  |
| `impl Block for TransferFcn::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:241` | - |  |
| `impl Block for TransportDelay::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:333` | - |  |
| `impl Block for UnitDelay::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:42` | - |  |
| `impl Block for ZeroOrderHold::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:97` | - |  |
| `impl Block for Gain::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:36` | - |  |
| `impl Block for Sum::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:90` | - |  |
| `impl Block for Product::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:152` | - |  |
| `impl Block for Saturation::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:206` | - |  |
| `impl Block for DeadZone::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:252` | - |  |
| `impl Block for Switch::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:302` | - |  |
| `impl Block for Relational::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:352` | - |  |
| `impl Block for LogicalOperator::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:412` | - |  |
| `impl Block for Abs::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/math.rs:471` | - |  |
| `impl Block for Mux::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/routing.rs:34` | - |  |
| `impl Block for Demux::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/routing.rs:85` | - |  |
| `impl Block for Scope::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:51` | - |  |
| `impl Block for ToWorkspace::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:98` | - |  |
| `impl Block for Constant::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sources.rs:32` | - |  |
| `impl Block for Step::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sources.rs:75` | - |  |
| `impl Block for Sine::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sources.rs:121` | - |  |
| `impl Block for Ramp::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sources.rs:165` | - |  |
| `impl Block for Clock::outputs` | 5 | `crates/kinetics-block-simulation/src/blocks/sources.rs:205` | - |  |
| `outside_subset_falls_back` | 0 | `crates/discourse-academic-tex/src/typeset.rs:539` | - |  |
| `overlapping_separate_lines` | 0 | `crates/drafting-geometry/src/region.rs:415` | - |  |
| `impl SampleRange::overlaps` | 2 | `crates/audio-timebase/src/rate.rs:108` | - |  |
| `overlaps` | 2 | `crates/vector-locus-pathops/src/three_d/visibility.rs:63` | 2d |  |
| `impl Renderer::override_image` | 3 | `crates/vector-compute-renderer/src/lib.rs:538` | - | Overwrite `image` with `texture`. |
| `impl WGPUContext::override_limits` | 1 | `crates/gpu-device-context/src/lib.rs:145` | - |  |
| `impl Reader<'a>::overrun` | 1 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:58` | - |  |
| `impl Cabac<'a>::overrun` | 1 | `crates/video-h264-codec/src/cabac.rs:142` | - | True when the reader has run past the end of the data (corrupt stream). |
| `impl Reader<'a>::overrun` | 1 | `crates/video-prores-codec/src/bits.rs:62` | - |  |
| `oversized_layer_reports_texture_dimensions` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:210` | - |  |
| `oversized_regions` | 0 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:381` | - |  |
| `impl GpuProfiler::owning_scope` | 3 | `crates/gpu-command-profiler/src/profiler.rs:172` | - | Starts a new auto-closing profiler scope that takes ownership of the passed encoder or rendering/compute pass. |
| `p2` | 2 | `crates/locus-parametric-solver/src/solver/model.rs:141` | 2d |  |
| `p_add` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:33` | - |  |
| `p_cross` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:48` | - |  |
| `p_dot` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:43` | - |  |
| `p_len_sq` | 1 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:57` | - |  |
| `p_mb_type` | 1 | `crates/video-h264-codec/src/mbtypes.rs:101` | - | P-slice mb_type 0..=4 (5.. are intra, handled by the caller). |
| `p_normalize` | 1 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:62` | - |  |
| `p_scale` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:38` | - |  |
| `p_skip` | 0 | `crates/video-h264-preview/src/lib.rs:559` | - |  |
| `p_slice_ref_count_overflow_is_error_not_panic_or_wrapped_zero` | 0 | `crates/video-h264-codec/src/slice.rs:442` | - |  |
| `p_slice_rejects_oversized_ref_mod_and_mmco_differences` | 0 | `crates/video-h264-codec/src/slice.rs:407` | - |  |
| `p_slice_uses_prior_reference_when_range_starts_after_idr` | 0 | `crates/video-h264-preview/src/lib.rs:733` | - |  |
| `p_sub` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:28` | - |  |
| `p_sub_mb_type` | 1 | `crates/video-h264-codec/src/mbtypes.rs:159` | - |  |
| `impl PaintResolver<'a>::pack` | 2 | `crates/vector-hybrid-renderer/src/paint.rs:96` | - |  |
| `pack4x8unorm` | 1 | `crates/vector-compute-shaders/src/cpu/fine.rs:42` | - |  |
| `pack_blend_config` | 6 | `crates/vector-hybrid-renderer/src/blend.rs:128` | - |  |
| `pack_compose` | 1 | `crates/vector-hybrid-renderer/src/blend.rs:165` | - |  |
| `pack_header` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:95` | - |  |
| `pack_header_with_gaussian_params` | 4 | `crates/vector-hybrid-renderer/src/filter.rs:101` | graph, numerics |  |
| `pack_image_offset` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:669` | - | Pack image offset coordinates `x` and `y` into a single u32. |
| `pack_image_params` | 3 | `crates/vector-hybrid-renderer/src/render/common.rs:678` | - | Pack image `quality` and extend modes into a single u32. |
| `pack_image_size` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:661` | - | Pack image `width` and `height` into a single u32. |
| `pack_language` | 1 | `crates/media-isobmff/src/mux.rs:136` | - |  |
| `pack_mix` | 1 | `crates/vector-hybrid-renderer/src/blend.rs:144` | - |  |
| `impl AssetHost<'_>::pack_msdf_font_atlas` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:335` | - |  |
| `pack_msdf_font_atlas_metrics_map_em_box_and_two_produces_match` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:580` | - |  |
| `impl AssetHost<'_>::pack_nine_slice` | 2 | `crates/asset-production-vibe/src/graphics_pack.rs:260` | - |  |
| `pack_opacity` | 1 | `crates/vector-hybrid-renderer/src/util.rs:107` | - |  |
| `impl AssetHost<'_>::pack_pbr_set` | 2 | `crates/asset-production-vibe/src/graphics_pack.rs:366` | - |  |
| `pack_radial_kind_and_swapped` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:651` | - | Pack radial gradient `kind` and `f_is_swapped` into a single u32. |
| `impl AssetHost<'_>::pack_sprite_sheet` | 2 | `crates/asset-production-vibe/src/graphics_pack.rs:112` | 2d |  |
| `pack_texture_width_and_extend_mode` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:638` | - | Pack `extend_mode` and `texture_width` into a single u32. |
| `pack_tint` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:690` | - | Pack an optional [`Tint`](vello_common::paint::Tint) into a (`tint_color_u32`, `tint_mode_u32`) pair for the GPU. |
| `pack_u16_pair` | 2 | `crates/vector-hybrid-renderer/src/util.rs:99` | - |  |
| `pack_unorm4x8` | 1 | `crates/vector-hybrid-renderer/src/rect.rs:140` | - |  |
| `packages` | 0 | `crates/discourse-academic-tex/src/catalog.rs:405` | - | Distinct package names present in the catalog, sorted. |
| `packages_include_core_packages` | 0 | `crates/discourse-academic-tex/src/catalog.rs:449` | - |  |
| `packet_samples_48k` | 1 | `crates/audio-opus-codec/src/packet.rs:264` | - | Number of samples (at 48 kHz) in a packet, or an error for malformed framing. |
| `packets` | 1 | `crates/media-isobmff/tests/common/mod.rs:158` | - |  |
| `packets_spanning_pages` | 0 | `crates/media-ogg/src/tests.rs:111` | - |  |
| `pad` | 2 | `crates/locus-parametric-solver/src/solver/residual.rs:503` | - |  |
| `padded_reuse` | 0 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:361` | - |  |
| `page` | 5 | `crates/media-ogg/src/strict_opus.rs:124` | - |  |
| `page` | 7 | `crates/media-ogg/src/tests.rs:6` | - | Write one page holding `segments` (lacing already computed by `lace`). |
| `page_capacity` | 0 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:335` | - |  |
| `impl ScheduledCase::page_counts` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:149` | - |  |
| `page_growth` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:191` | - |  |
| `impl RoundBindings::page_indices` | 1 | `crates/vector-hybrid-renderer/src/target.rs:232` | - |  |
| `page_selection` | 0 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:297` | - |  |
| `page_stream` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:459` | - |  |
| `impl Typesetter::paginate` | 3 | `crates/layout-paged-typesetter/src/pagination/flow.rs:76` | 2d | Paginates paragraphs into pages. |
| `paint` | 1 | `crates/asset-production-vibe/src/graphics.rs:492` | - |  |
| `impl Scene::paint` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:788` | - | Get the current paint. |
| `impl Scene::paint_has_area` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:316` | - | Whether drawing with the current paint can produce any pixels. |
| `paint_stroke` | 4 | `crates/surface-pbr-studio/src/paint.rs:142` | - | Paints an interpolated stroke along a path. |
| `impl Scene::paint_transform_has_area` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:311` | 2d | Whether the effective paint transform maps paint space onto a non-zero area. |
| `pair_kerning` | 2 | `crates/asset-production-vibe/src/glyph/mod.rs:118` | - | Pair kerning adjustment in normalized em units. |
| `impl VbapPanner<'a>::pan` | 4 | `crates/acoustics-daw-session/src/dsp/pan.rs:95` | - | Computes speaker gains for a sound source at (azimuth_deg, elevation_deg). |
| `panic_after_play_attempt_poison_is_retained` | 0 | `crates/pcm-device-output/src/device_slot_tests.rs:43` | - |  |
| `par_rows` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:177` | - |  |
| `par_rows` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:162` | - |  |
| `par_rows` | 4 | `crates/raster-selection-morphology/src/lib.rs:251` | - |  |
| `para` | 3 | `crates/color-transforms/src/icc/curve.rs:94` | - |  |
| `para_inverse` | 3 | `crates/color-transforms/src/icc/curve.rs:106` | - |  |
| `parade` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:327` | - |  |
| `paragraph_glyphs` | 2 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:122` | - | Shape `text` into composer glyphs, word/hyphen breakpoints, and hyphen-point count. |
| `parallel_map` | 2 | `crates/vector-radiance-trace/src/lib.rs:501` | - |  |
| `parallel_map` | 2 | `crates/vector-radiance-trace/src/lib.rs:533` | - |  |
| `impl Curve::param_count` | 1 | `crates/color-transforms/src/icc/curve.rs:31` | - |  |
| `impl Line::param_of` | 2 | `crates/drafting-geometry/src/curve.rs:26` | 3d, 2d | Parameter (0..1 on the segment) of the projection of `p`. |
| `impl Ellipse::param_of` | 2 | `crates/drafting-geometry/src/curve.rs:272` | 2d | Parameter of the point on the ellipse nearest the direction of `p` (approximate). |
| `parameter_sets` | 0 | `crates/video-h264-codec/src/slice.rs:388` | - |  |
| `parcor_to_lpc` | 1 | `crates/audio-aac-codec/src/tns.rs:140` | - | Reflection coefficients → direct-form LPC `a[0..=order]` (`a[0] = 1`). |
| `parens_delimiters` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:469` | - |  |
| `parent_relative_parts_retain_semantics_material_ranges_and_flat_frames` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:96` | - |  |
| `parent_relative_parts_retain_semantics_material_ranges_and_flat_frames` | 0 | `crates/profile-authoring/src/tests.rs:196` | - |  |
| `parents` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:94` | - | Layout and master placeholders a shape inherits from. |
| `impl CamelotKey::parse` | 1 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:27` | - |  |
| `impl Axis::parse` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:61` | - |  |
| `impl RuleKind::parse` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:38` | - |  |
| `impl ProgramConfig::parse` | 1 | `crates/audio-aac-codec/src/config.rs:44` | - |  |
| `impl AudioSpecificConfig::parse` | 1 | `crates/audio-aac-codec/src/config.rs:204` | - |  |
| `impl AdtsHeader::parse` | 1 | `crates/audio-aac-codec/src/config.rs:293` | - |  |
| `impl IcsInfo::parse` | 2 | `crates/audio-aac-codec/src/ics.rs:69` | - |  |
| `impl TnsData::parse` | 2 | `crates/audio-aac-codec/src/tns.rs:38` | - |  |
| `impl OpusHead::parse` | 1 | `crates/audio-opus-codec/src/header.rs:45` | - | Parses an `OpusHead` packet (Ogg first packet, Matroska `CodecPrivate`). |
| `impl Toc::parse` | 1 | `crates/audio-opus-codec/src/packet.rs:52` | - |  |
| `impl Packet<'a>::parse` | 1 | `crates/audio-opus-codec/src/packet.rs:117` | - | Parses a packet (RFC 6716 section 3.2), enforcing requirements R1 to R7. |
| `impl Timecode::parse` | 1 | `crates/audio-timebase/src/timecode.rs:211` | - | Parse timecode string (`HH:MM:SS:FF`, with `:` / `;` / `.` separators). |
| `impl FeetFrames::parse` | 1 | `crates/audio-timebase/src/timecode.rs:284` | - | Parse `FEET+FRAMES` format string. |
| `impl Primer::parse` | 1 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:11` | - |  |
| `impl Set::parse` | 3 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:52` | - |  |
| `impl Partition::parse` | 4 | `crates/chronicle-broadcast-containers/src/mxf/partition.rs:33` | - |  |
| `impl ZipPackage::parse` | 1 | `crates/chronicle-office-codecs/src/zip/minizip.rs:140` | - | Parse a ZIP byte slice and extract uncompressed (stored) entries. |
| `impl CellRef::parse` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:52` | - | Parses `A1`, `$A$1`, `xfd1048576` (case-insensitive). `None` if malformed or out of range. |
| `impl RangeRef::parse` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:240` | - | Parses `A1`, `A1:B2`, `$A$1:$B$2`, `A:C`, `3:5` (no sheet prefix). |
| `impl CellError::parse` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:71` | - |  |
| `parse` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:25` | - |  |
| `impl NumberFormat::parse` | 1 | `crates/chronicle-tabular-engine/src/numfmt/mod.rs:30` | - |  |
| `impl XmlNode::parse` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:170` | - | Parse an XML text document into an `XmlNode` tree. |
| `impl Profile::parse` | 1 | `crates/color-transforms/src/icc/profile.rs:294` | - |  |
| `impl Lut::parse` | 2 | `crates/color-transforms/src/lut.rs:198` | - | Parse by format (sniffs `LUT_*_SIZE` for `.cube` when the format is unknown). |
| `impl ColorSpace::parse` | 1 | `crates/color-transforms/src/spaces.rs:236` | - | Parse an id or a label (case-insensitive, punctuation ignored). |
| `impl WorkingSpace::parse` | 1 | `crates/color-transforms/src/spaces.rs:324` | - |  |
| `parse` | 1 | `crates/discourse-academic-tex/src/texparse/magic.rs:56` | - | Read magic-comment header of a `.tex` source. |
| `impl Ttf::parse` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:50` | - | Parse TrueType table directory + consumed tables. |
| `parse` | 1 | `crates/discourse-academic-tex/src/typeset.rs:189` | - |  |
| `impl Parser::parse` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:36` | - |  |
| `impl Document::parse` | 1 | `crates/kinetics-block-simulation/src/document.rs:40` | - |  |
| `impl Solver::parse` | 1 | `crates/kinetics-block-simulation/src/sim.rs:16` | - |  |
| `impl SyncRole::parse` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:45` | - |  |
| `impl ItemKind::parse` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:54` | - |  |
| `impl TaskStatus::parse` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:110` | - |  |
| `impl Priority::parse` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:142` | - |  |
| `impl DepKind::parse` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:184` | - |  |
| `impl TaskConstraint::parse` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:222` | - |  |
| `impl Unit::parse` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/ast.rs:23` | - |  |
| `parse` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:139` | - |  |
| `impl Patterns::parse` | 1 | `crates/layout-paged-typesetter/src/hyphenation/liang.rs:20` | - | Parse TeX-style patterns, whitespace-separated (e.g. `hy3ph he2n hena4`). |
| `impl MotionPath::parse` | 1 | `crates/layout-presentation-deck/src/timeline/path.rs:57` | - |  |
| `parse` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:155` | - |  |
| `impl Sub::parse` | 1 | `crates/locus-parametric-solver/src/solver/model.rs:41` | - |  |
| `parse` | 1 | `crates/manifold-bim-csg/src/step.rs:39` | - |  |
| `impl MasteringDisplay::parse` | 1 | `crates/media-isobmff/src/codec.rs:87` | - |  |
| `impl AvcConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:257` | - |  |
| `impl HevcConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:333` | - |  |
| `impl Av1Config::parse` | 1 | `crates/media-isobmff/src/codec.rs:443` | - |  |
| `impl VpcConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:501` | - |  |
| `impl OpusConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:631` | - |  |
| `impl FlacConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:668` | - |  |
| `impl ApvConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:789` | - | Parse the payload of an `apvC` box (with or without the 4-byte `FullBox` header). |
| `impl VorbisInfo::parse` | 1 | `crates/media-ogg/src/lib.rs:473` | - |  |
| `impl BlendMode::parse` | 1 | `crates/raster-blend-operators/src/lib.rs:161` | - | Parse a label or identifier, case/space-insensitively. |
| `parse` | 1 | `crates/vector-compute-shaders/src/compile/permutations.rs:14` | - |  |
| `parse` | 1 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:313` | - |  |
| `impl Plane::parse` | 1 | `crates/vector-locus-geometry/src/perspective.rs:26` | - |  |
| `impl Unit::parse` | 2 | `crates/vector-locus-geometry/src/units.rs:117` | - | Parse a length string in this unit with optional arithmetic (`+ - * /`). |
| `impl Sps::parse` | 1 | `crates/video-h264-codec/src/params.rs:341` | - |  |
| `impl Pps::parse` | 2 | `crates/video-h264-codec/src/params.rs:572` | - | Parse a PPS. Needs the SPS table because the tail depends on chroma_format_idc. |
| `impl NalHeader::parse` | 1 | `crates/video-h264-codec/src/slice.rs:35` | - |  |
| `impl SliceHeader::parse` | 3 | `crates/video-h264-codec/src/slice.rs:154` | - | Parse the slice header. `pps_lookup` resolves pic_parameter_set_id to (PPS, SPS). |
| `impl FrameHeader::parse` | 1 | `crates/video-prores-codec/src/header.rs:38` | - | Parse the 8-byte frame prefix and frame header. Returns the header, the byte offset of the |
| `impl PictureHeader::parse` | 1 | `crates/video-prores-codec/src/header.rs:187` | - |  |
| `impl Lut::parse_3dl` | 1 | `crates/color-transforms/src/lut.rs:332` | - | Parse an Autodesk `.3dl`. |
| `parse_a1_col` | 1 | `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:207` | - |  |
| `parse_a1_prefix` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:68` | - | Parses an A1 reference at the start of `s`: `(col, row, col_abs, row_abs, rest)`. |
| `parse_abr` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:383` | - | Parses an Adobe Brush (`.abr`) file from bytes. |
| `parse_aco` | 1 | `crates/color-transforms/src/palette/aco.rs:80` | - |  |
| `impl Parser::parse_add_sub` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:235` | - |  |
| `parse_affinity_archive` | 1 | `crates/surface-layered-codecs/src/affinity.rs:118` | - | Parses an Affinity archive, indexing its FAT entries and preview metadata. |
| `parse_alignment_block` | 1 | `crates/locus-geospatial-codecs/src/formats/landxml.rs:77` | - |  |
| `parse_and_roundtrip_tags` | 0 | `crates/vector-glyph-shaping/src/features.rs:127` | - |  |
| `impl Parser::parse_arg` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:152` | - |  |
| `parse_asc` | 1 | `crates/media-isobmff/src/codec.rs:583` | - | Decode (audio object type, sampling frequency, channel configuration) from an AudioSpecificConfig. |
| `parse_ascii_dxf` | 1 | `crates/asset-production-vibe/src/manifold_ops.rs:1023` | - |  |
| `parse_ase` | 1 | `crates/color-transforms/src/palette/ase.rs:60` | - |  |
| `parse_attrs` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:274` | - |  |
| `parse_bib` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:48` | - | Parse a `.bib` source string. Always returns a file object; problems land in `issues`. |
| `parse_box_header` | 1 | `crates/media-isobmff/src/bytes.rs:157` | - | Parse a box header from `buf` (needs up to 32 bytes; fewer if available). |
| `parse_brush_properties` | 2 | `crates/surface-pbr-studio/src/probrush.rs:93` | - | Parses clean-room brush settings from key-value pairs. |
| `parse_brush_strokes` | 1 | `crates/asset-production-vibe/src/geodesy_ops.rs:27` | - |  |
| `parse_btrt` | 1 | `crates/media-isobmff/src/codec.rs:1106` | - |  |
| `parse_cell_reference` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:248` | - |  |
| `parse_claim_basis` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:106` | - |  |
| `parse_clipitem` | 2 | `crates/chronicle-timeline-interchange/src/interchange/fcp7.rs:112` | graph |  |
| `parse_clock` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:167` | - | Parse clock format: `HH:MM:SS.mmm` or `HH:MM:SS,mmm` into `Tick`. |
| `parse_cluster` | 2 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:260` | - |  |
| `parse_col_reference` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:256` | - |  |
| `parse_colr` | 1 | `crates/media-isobmff/src/codec.rs:1111` | - |  |
| `impl Parser::parse_command` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:265` | - |  |
| `impl Parser::parse_comparison` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:199` | - |  |
| `parse_coordinates` | 1 | `crates/locus-geospatial-codecs/src/formats/kml.rs:51` | - |  |
| `parse_csv` | 2 | `crates/chronicle-office-codecs/src/spreadsheet/csv.rs:42` | numerics | Parse CSV/TSV input into a 2D matrix of strings (`Vec<Vec<String>>`). |
| `impl Lut3d::parse_cube` | 1 | `crates/color-transforms/src/lut.rs:84` | - | Parse a `.cube` file that contains a 3D table (a shaper, if present, is rejected; use |
| `impl Lut::parse_cube` | 1 | `crates/color-transforms/src/lut.rs:207` | - |  |
| `impl Lut3d::parse_cube` | 1 | `crates/radiance-color-grading/src/lut.rs:177` | - | Parse a `.cube` file string. |
| `impl Lut::parse_cube` | 1 | `crates/radiance-color-grading/src/lut.rs:232` | - | Parse a `.cube` text representation. |
| `parse_curve` | 1 | `crates/color-transforms/src/icc/profile.rs:461` | - |  |
| `parse_datetime` | 2 | `crates/chronicle-tabular-engine/src/core/parse.rs:146` | - |  |
| `parse_desc` | 1 | `crates/color-transforms/src/icc/profile.rs:421` | - |  |
| `parse_dimensions` | 1 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:20` | - |  |
| `parse_docx_package` | 1 | `crates/chronicle-office-codecs/src/wordprocessing/docx.rs:115` | - | Parse an OpenXML `.docx` ZIP package into a `FlowDoc`. |
| `parse_dxf` | 1 | `crates/drafting-geometry/src/dxf_reader.rs:216` | - | Parses a DXF ASCII string into drafting entities. |
| `parse_ebml_header` | 2 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:68` | - |  |
| `parse_edl` | 2 | `crates/chronicle-timeline-interchange/src/interchange/edl.rs:11` | - | Parse a CMX 3600 EDL document into a TimelineSequence. |
| `parse_element` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:260` | - |  |
| `parse_end_mismatch` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:239` | - |  |
| `parse_esds` | 1 | `crates/media-isobmff/src/codec.rs:1134` | - |  |
| `parse_exception` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:58` | - |  |
| `impl Parser<'a>::parse_expression` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:30` | - |  |
| `parse_family0_and_roundtrip` | 0 | `crates/audio-opus-codec/src/header.rs:137` | - |  |
| `parse_family1_51` | 0 | `crates/audio-opus-codec/src/header.rs:151` | - |  |
| `parse_fauna_archetype` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:331` | - |  |
| `parse_fcp7_xml` | 1 | `crates/chronicle-timeline-interchange/src/interchange/fcp7.rs:10` | - | Parse FCP7 XML text into a TimelineSequence. |
| `parse_fcpxml` | 1 | `crates/chronicle-timeline-interchange/src/interchange/fcpxml.rs:43` | - | Parse an FCPXML document string into a TimelineSequence. |
| `parse_fcpxml_time` | 1 | `crates/chronicle-timeline-interchange/src/interchange/fcpxml.rs:10` | - | Parse an FCPXML time string (`"1001/24000s"`, `"5s"`, `"0s"`, `"3.5s"`) into a `Tick`. |
| `parse_filegdb_geometry` | 1 | `crates/locus-geospatial-codecs/src/formats/filegdb.rs:47` | - | Decodes an ESRI FileGDB shape binary buffer into standard Geometry. |
| `parse_fixed` | 2 | `crates/chronicle-fiduciary-ledger/src/money.rs:60` | - | Parse a decimal string like "-1,234.50" into a fixed-point integer with `dp` decimals. |
| `parse_fixed_values` | 0 | `crates/chronicle-fiduciary-ledger/src/money.rs:163` | - |  |
| `impl Parser::parse_for` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:128` | - |  |
| `parse_fraction` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:121` | - |  |
| `parse_frame_header` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:99` | - |  |
| `parse_gpl` | 1 | `crates/color-transforms/src/palette/gpl.rs:19` | - |  |
| `parse_grd` | 1 | `crates/color-transforms/src/palette/grd.rs:35` | - | Parses a `.grd` binary stream. |
| `impl Parser::parse_group` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:278` | - |  |
| `parse_hex` | 1 | `crates/color-transforms/src/lib.rs:301` | - | Parse `#rrggbb` / `#rrggbbaa` into 0..1 floats. |
| `parse_hex` | 1 | `crates/radiance-color-grading/src/matrix.rs:405` | - | Parse `#rrggbb` / `#rrggbbaa` into 0..1 floats. |
| `parse_hrd` | 1 | `crates/video-h264-codec/src/params.rs:157` | - |  |
| `parse_html` | 1 | `crates/chronicle-office-codecs/src/document/html.rs:161` | - | Parse HTML string into a `FlowDoc`. |
| `parse_id_list` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:225` | - |  |
| `parse_identifier` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:76` | - | Parse any pasted identifier into a normalized [`Ident`]. |
| `impl Parser::parse_if` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:85` | - |  |
| `impl Packet<'a>::parse_impl` | 2 | `crates/audio-opus-codec/src/packet.rs:127` | - |  |
| `parse_info` | 2 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:111` | - |  |
| `parse_inlines` | 1 | `crates/chronicle-office-codecs/src/document/markdown.rs:178` | - | Parse inline formatting: bold, italic, code, link, inline math. |
| `parse_input` | 2 | `crates/chronicle-tabular-engine/src/core/parse.rs:12` | - |  |
| `parse_iso8601_duration_days` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:437` | - |  |
| `parse_iso_date` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:161` | - |  |
| `parse_joint` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:180` | - |  |
| `parse_joint` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:430` | - |  |
| `parse_joint_rom` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:321` | - | Parses the spec's optional `joints` list. Returns an empty list when absent. |
| `parse_kml` | 1 | `crates/locus-geospatial-codecs/src/formats/kml.rs:15` | - | Parses KML XML text into a collection of Placemarks without external parser crates. |
| `parse_kml_geometry` | 1 | `crates/locus-geospatial-codecs/src/formats/kml.rs:62` | - |  |
| `parse_labels_round_trip` | 0 | `crates/raster-blend-operators/src/lib.rs:602` | - |  |
| `parse_landxml` | 1 | `crates/locus-geospatial-codecs/src/formats/landxml.rs:31` | - | Parses LandXML content into alignments and TIN surfaces without external XML crate dependencies. |
| `parse_lane` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:154` | - |  |
| `parse_language` | 1 | `crates/media-isobmff/src/demux.rs:394` | - |  |
| `parse_las_header` | 1 | `crates/locus-geospatial-codecs/src/formats/lidar.rs:46` | - | Parses LAS header block from raw file bytes. |
| `parse_las_points` | 3 | `crates/locus-geospatial-codecs/src/formats/lidar.rs:104` | - | Decodes points from LAS bytes according to the header. |
| `parse_latex` | 1 | `crates/chronicle-office-codecs/src/document/latex.rs:177` | - | Simple parser for LaTeX `.tex` documents into `FlowDoc`. |
| `parse_latex` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:5` | - |  |
| `parse_layer` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:417` | - |  |
| `parse_linear` | 1 | `crates/layout-paged-typesetter/src/equation/linear.rs:5` | numerics |  |
| `parse_list` | 3 | `crates/discourse-academic-tex/src/typeset.rs:198` | - |  |
| `parse_list` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:31` | - |  |
| `parse_locus_points` | 3 | `crates/asset-production-vibe/src/locus_ops.rs:249` | 2d |  |
| `impl Parser::parse_logical_and` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:189` | - |  |
| `impl Parser::parse_logical_or` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:179` | - |  |
| `parse_lottie` | 1 | `crates/kinetics-motion-compositor/src/lottie/mod.rs:12` | - | Parse a Lottie JSON document string. |
| `parse_lut` | 2 | `crates/color-transforms/src/icc/profile.rs:508` | - |  |
| `parse_m64` | 1 | `crates/asset-production-vibe/src/geodesy_ops.rs:728` | - |  |
| `parse_markdown` | 1 | `crates/chronicle-office-codecs/src/document/markdown.rs:10` | - | Parse a Markdown string into a `FlowDoc`. |
| `impl Parser::parse_matrix_literal` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:338` | numerics |  |
| `parse_mcc` | 1 | `crates/chronicle-timeline-interchange/src/captions/mcc.rs:10` | - |  |
| `parse_measure` | 2 | `crates/vector-locus-geometry/src/units.rs:149` | - |  |
| `parse_meta` | 2 | `crates/media-isobmff/src/demux.rs:954` | - |  |
| `parse_mif_mid` | 2 | `crates/locus-geospatial-codecs/src/formats/mapinfo.rs:21` | - | Parses a MapInfo MIF geometry stream and corresponding MID attribute lines. |
| `parse_minimal_baseline_sps` | 0 | `crates/video-h264-codec/src/params.rs:702` | - |  |
| `parse_moof` | 7 | `crates/media-isobmff/src/demux.rs:847` | - |  |
| `impl Parser::parse_mul_div` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:248` | - |  |
| `impl Parser::parse_nary` | 2 | `crates/layout-paged-typesetter/src/equation/latex.rs:322` | - |  |
| `parse_node` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:292` | graph |  |
| `parse_number_text` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:56` | - |  |
| `parse_numeric` | 2 | `crates/chronicle-tabular-engine/src/core/parse.rs:60` | - |  |
| `parse_nums3` | 1 | `crates/radiance-color-grading/src/lut.rs:361` | - |  |
| `parse_ora` | 1 | `crates/surface-layered-codecs/src/xcf_ora.rs:329` | - | Decodes an OpenRaster package (.ora) from raw ZIP archive bytes. |
| `parse_otio` | 1 | `crates/chronicle-timeline-interchange/src/interchange/otio.rs:11` | - | Parse an OpenTimelineIO JSON string into a TimelineSequence. |
| `parse_packet_header` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea708.rs:12` | - |  |
| `parse_pat` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/psi.rs:33` | - |  |
| `parse_pes_header` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/pes.rs:34` | - |  |
| `parse_plain` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:79` | - |  |
| `parse_pmt` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/psi.rs:83` | - |  |
| `parse_pmtiles_header` | 1 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:81` | - | Parses PMTiles v3 header from a 127-byte slice. |
| `parse_position` | 6 | `crates/audio-timebase/src/transport.rs:174` | - | Parse a user-entered position string into samples. |
| `impl Parser::parse_postfix` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:291` | - |  |
| `impl Parser::parse_power` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:263` | - |  |
| `parse_pptx_package` | 1 | `crates/chronicle-office-codecs/src/presentation/pptx.rs:116` | - | Parse an OpenXML `.pptx` ZIP package into a `Presentation`. |
| `impl Parser<'a>::parse_precedence` | 2 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:34` | - |  |
| `parse_pred_token` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:187` | - |  |
| `impl Parser::parse_primary` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:300` | - |  |
| `impl Parser<'a>::parse_primary` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:74` | - |  |
| `parse_profile` | 1 | `crates/color-transforms/src/icc/profile.rs:327` | - |  |
| `parse_range` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:158` | - |  |
| `impl Parser::parse_range` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:218` | - |  |
| `impl AssetHost<'_>::parse_render_inputs` | 3 | `crates/asset-production-vibe/src/layout_ops/sheet_render.rs:410` | - |  |
| `parse_rgba_raw` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:496` | - |  |
| `parse_rom_property` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:105` | semantics |  |
| `parse_rtf` | 1 | `crates/chronicle-office-codecs/src/document/rtf.rs:129` | - | Simple parser for RTF 1.5 files into `FlowDoc`. |
| `parse_rule` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:169` | - |  |
| `parse_rules_value` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:243` | - | Parses a VibeScript list of rule records. |
| `parse_sample_entry` | 4 | `crates/media-isobmff/src/codec.rs:897` | - | Parse one `stsd` entry. `handler` is the track handler type; `is_qt` selects QuickTime |
| `parse_scaling_list` | 1 | `crates/video-h264-codec/src/params.rs:67` | - | Result of parsing one scaling_list(): Some(list) or None meaning "use default". |
| `parse_scaling_lists` | 3 | `crates/video-h264-codec/src/params.rs:88` | - | Parse scaling lists with a fall-back rule. `fallback` gives the lists used for rule B |
| `parse_scratch_dsl` | 1 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:181` | - | Parses a turntable scratch pattern string into beats of actions (e.g. "AEA.AEA."). |
| `parse_script` | 3 | `crates/discourse-academic-tex/src/typeset.rs:288` | - |  |
| `impl Parser::parse_script_target` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:294` | - |  |
| `parse_sealed_emf` | 1 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:655` | - | QualiaDB `parse` admits CRC-only files; untrusted paths require a writer Bao seal. |
| `parse_sections` | 1 | `crates/chronicle-tabular-engine/src/numfmt/parse.rs:89` | - |  |
| `parse_segment` | 2 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:88` | - |  |
| `impl Packet<'a>::parse_self_delimited` | 1 | `crates/audio-opus-codec/src/packet.rs:123` | - | Parses a self-delimited packet (RFC 6716 Appendix B, used for all but the last stream of |
| `parse_sfnt_directory` | 1 | `crates/vector-glyph-shaping/src/subset.rs:34` | - | Parse the SFNT table directory from raw font bytes. |
| `parse_size` | 1 | `crates/audio-opus-codec/src/packet.rs:107` | - |  |
| `parse_snippets` | 2 | `crates/discourse-academic-tex/src/review/snippets.rs:66` | - | Parse a snippet JSON string: accepts `[{trigger,body,...}]` or `{trigger: body}` / `{trigger: {body,...}}`. |
| `parse_srt` | 1 | `crates/chronicle-timeline-interchange/src/captions/srt.rs:10` | - |  |
| `parse_stack_xml` | 1 | `crates/surface-layered-codecs/src/xcf_ora.rs:410` | - | Parses dimensions and layers from `stack.xml`. |
| `parse_stbl` | 5 | `crates/media-isobmff/src/demux.rs:613` | - |  |
| `parse_step_file` | 0 | `crates/manifold-bim-csg/src/step.rs:401` | - |  |
| `parse_stray_end` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:246` | - |  |
| `parse_structure` | 1 | `crates/discourse-academic-tex/src/texparse/structure.rs:73` | - | Lex `src` and extract its structure. Always succeeds — malformed input lands in `issues`. |
| `parse_surface_block` | 1 | `crates/locus-geospatial-codecs/src/formats/landxml.rs:103` | - |  |
| `parse_theme_record` | 1 | `crates/asset-production-vibe/src/layout_ops/mod.rs:187` | - |  |
| `parse_time_only` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:190` | - |  |
| `parse_timecode` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:142` | - | Parse a timecode string (`HH:MM:SS:FF` or `HH:MM:SS;FF`) into frame count. |
| `parse_timestamp` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/pes.rs:16` | - |  |
| `parse_timing_line` | 1 | `crates/chronicle-timeline-interchange/src/captions/srt.rs:59` | - |  |
| `parse_track_entry` | 1 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:155` | - |  |
| `parse_tracks` | 2 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:133` | - |  |
| `parse_trak` | 4 | `crates/media-isobmff/src/demux.rs:411` | - |  |
| `parse_transfer` | 1 | `crates/asset-production-vibe/src/radiance_color.rs:45` | - |  |
| `parse_ts_packet` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/ts.rs:23` | - | Parse a single 188-byte TS packet. |
| `parse_udta` | 2 | `crates/media-isobmff/src/demux.rs:941` | - |  |
| `impl Parser::parse_unary` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:277` | - |  |
| `parse_unclosed_begin` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:235` | - |  |
| `impl Parser::parse_until` | 2 | `crates/layout-paged-typesetter/src/equation/latex.rs:298` | - |  |
| `impl Parser::parse_until_right` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:310` | - |  |
| `parse_us_date` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:174` | - |  |
| `parse_variants` | 0 | `crates/audio-timebase/src/timecode.rs:323` | - |  |
| `parse_various_formats` | 0 | `crates/audio-timebase/src/transport.rs:259` | - |  |
| `parse_vec2` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:476` | 2d |  |
| `parse_vec3` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:484` | 3d |  |
| `parse_vertex_uvs` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:163` | computational geometry, 3d | Vertex-aligned UVs from a list, unwrap record, or named profile (island outline |
| `parse_vtt` | 1 | `crates/chronicle-timeline-interchange/src/captions/vtt.rs:10` | - |  |
| `parse_vtt_timing_line` | 1 | `crates/chronicle-timeline-interchange/src/captions/vtt.rs:81` | - |  |
| `parse_vui` | 1 | `crates/video-h264-codec/src/params.rs:247` | - |  |
| `impl Parser::parse_while` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:155` | - |  |
| `parse_with_units_and_arithmetic` | 0 | `crates/vector-locus-geometry/src/units.rs:201` | - |  |
| `parse_xcf` | 1 | `crates/surface-layered-codecs/src/xcf_ora.rs:67` | - | Decodes an XCF image file from bytes. |
| `parse_xfdf_fields` | 1 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:48` | - |  |
| `parse_xlsx_package` | 1 | `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:132` | - | Parse an OpenXML `.xlsx` ZIP package into an `XlsxWorkbook`. |
| `parse_xml` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:204` | - | Parse XML string into an element tree. |
| `impl AscCdl::parse_xml` | 1 | `crates/radiance-color-grading/src/grade.rs:195` | - | Parse ASC CDL XML string (`.cc`, `.ccc`, `.cdl`). |
| `parse_xyz` | 1 | `crates/color-transforms/src/icc/profile.rs:453` | - |  |
| `part` | 2 | `crates/profile-authoring/provenance/original-spatial-tests.rs:11` | - |  |
| `part` | 2 | `crates/profile-authoring/src/tests.rs:14` | - |  |
| `part` | 5 | `crates/vector-hybrid-renderer/src/rect.rs:155` | - |  |
| `impl PartialOrd for DijkstraItem::partial_cmp` | 2 | `crates/asset-production-vibe/src/locus_generative/rmf_geodesic.rs:34` | - |  |
| `impl PartialOrd for State::partial_cmp` | 2 | `crates/geodesy-spatial-analysis/src/network_topology.rs:96` | - |  |
| `impl PartialOrd for FloodItem::partial_cmp` | 2 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:48` | - |  |
| `partition_key` | 2 | `crates/chronicle-broadcast-containers/src/mxf/labels.rs:10` | - |  |
| `partition_of_unity` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:223` | - |  |
| `pass_batching` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:749` | - |  |
| `pass_iterators_skip_empty_parities` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:632` | - |  |
| `past_release` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:228` | - |  |
| `impl Face::path` | 1 | `crates/vector-locus-pathops/src/three_d/revolve.rs:136` | - |  |
| `path_corners` | 1 | `crates/vector-locus-geometry/src/corners.rs:135` | - | The corners of `path` Live Corners can cut, in anchor order. |
| `path_count` | 2 | `crates/vector-compute-shaders/src/cpu/path_count.rs:156` | - |  |
| `path_count_main` | 5 | `crates/vector-compute-shaders/src/cpu/path_count.rs:13` | - |  |
| `path_count_setup` | 2 | `crates/vector-compute-shaders/src/cpu/path_count_setup.rs:17` | - |  |
| `path_count_setup_main` | 2 | `crates/vector-compute-shaders/src/cpu/path_count_setup.rs:10` | - |  |
| `path_fill_type_from_fill` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1144` | - |  |
| `path_from_shape` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1341` | - |  |
| `path_len` | 1 | `crates/vector-locus-geometry/src/recognize.rs:37` | - |  |
| `path_search_finds_plain_and_exe_names` | 0 | `crates/media-test-support/src/oracle.rs:149` | - |  |
| `path_svg_d` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:69` | - |  |
| `path_tiling` | 2 | `crates/vector-compute-shaders/src/cpu/path_tiling.rs:168` | - |  |
| `path_tiling_main` | 6 | `crates/vector-compute-shaders/src/cpu/path_tiling.rs:15` | - |  |
| `path_tiling_setup` | 2 | `crates/vector-compute-shaders/src/cpu/path_tiling_setup.rs:17` | - |  |
| `path_tiling_setup_main` | 2 | `crates/vector-compute-shaders/src/cpu/path_tiling_setup.rs:10` | - |  |
| `pathfinder` | 2 | `crates/vector-locus-pathops/src/pathops/pathfinder.rs:56` | - | Execute a Pathfinder operation on a stack of shapes. |
| `pathfinder_minus_front` | 0 | `crates/vector-locus-pathops/src/pathops/pathfinder.rs:197` | - |  |
| `pathfinder_unite_two_shapes` | 0 | `crates/vector-locus-pathops/src/pathops/pathfinder.rs:188` | - |  |
| `impl AssetHost<'_>::pathfinder_warp_envelope` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:368` | - | 121: `hypermedia.locus.pathfinder_warp_envelope` (`vector-locus-pathops` + `vector-locus-geometry`) |
| `impl GeodesicRuler::paths` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:161` | - |  |
| `pathtag_reduce` | 2 | `crates/vector-compute-shaders/src/cpu/pathtag_reduce.rs:27` | - |  |
| `pathtag_reduce_main` | 4 | `crates/vector-compute-shaders/src/cpu/pathtag_reduce.rs:10` | - |  |
| `pathtag_scan` | 2 | `crates/vector-compute-shaders/src/cpu/pathtag_scan.rs:31` | - |  |
| `pathtag_scan_main` | 5 | `crates/vector-compute-shaders/src/cpu/pathtag_scan.rs:10` | - |  |
| `pattern_brush_repeats` | 0 | `crates/vector-surface-brush/src/pattern.rs:125` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:990` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:135` | - |  |
| `impl Stream::pause` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:29` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:359` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:912` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:248` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:215` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:130` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:487` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:193` | - |  |
| `impl StreamTrait for Stream::pause` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:441` | - |  |
| `trait StreamTrait::pause` | 1 | `crates/audio-device-cpal/src/traits.rs:220` | - | Some devices support pausing the audio stream. This can be useful for saving energy in |
| `payload` | 3 | `crates/media-isobmff/tests/roundtrip.rs:19` | - |  |
| `impl LayerTextureRegion::payload_at` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:438` | - |  |
| `impl PackedPaint::payload_at` | 3 | `crates/vector-hybrid-renderer/src/paint.rs:52` | - |  |
| `impl CowTerrainTileGrid::payload_memory_bytes` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:283` | - | Conservative tile-reference/sample payload estimate, not distinct allocation or heap bytes. |
| `impl CowTerrainTileGrid::payload_memory_bytes` | 1 | `crates/terrain-brushes/src/lib.rs:468` | - | Conservative tile-reference/sample payload estimate, not distinct allocation or heap bytes. |
| `impl SampleEntry::pcm` | 1 | `crates/media-isobmff/src/codec.rs:1320` | - | PCM entry. The muxer picks the on-disk format (`sowt`/`twos`/`lpcm` for MOV, `ipcm`/`fpcm` for MP4). |
| `pcm_entries` | 0 | `crates/media-isobmff/src/codec.rs:1663` | - |  |
| `pcm_in_mp4_ipcm` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:200` | - |  |
| `pcm_long_term_mmco_gaps_cabac` | 0 | `crates/video-h264-codec/src/synth_tests.rs:468` | - |  |
| `pcm_long_term_mmco_gaps_cavlc` | 0 | `crates/video-h264-codec/src/synth_tests.rs:462` | - |  |
| `pcm_name_matches` | 2 | `crates/media-isobmff/tests/oracle_demux.rs:9` | - |  |
| `pcm_picture` | 5 | `crates/video-h264-preview/src/synth.rs:67` | - |  |
| `impl Profile::pcs_to_device` | 2 | `crates/color-transforms/src/icc/profile.rs:260` | - |  |
| `peak` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:482` | - |  |
| `impl LogCurve::peak_linear` | 1 | `crates/color-transforms/src/log.rs:101` | numerics | Scene-linear value of the maximum signal (code 1023), used as the source peak for tone |
| `impl HdrMetadata::peak_nits` | 1 | `crates/color-transforms/src/lib.rs:127` | 2d | The content's peak for tone mapping: MaxCLL when signalled (the brightest pixel actually |
| `impl HdrMetadata::peak_nits` | 1 | `crates/radiance-color-grading/src/matrix.rs:127` | - | The content peak for tone mapping. |
| `peaks` | 3 | `crates/radiance-color-grading/src/scopes/summary.rs:88` | - |  |
| `impl BitReader<'a>::peek` | 2 | `crates/bitstream-codec/src/lib.rs:70` | - | Peek up to 32 bits without consuming (zero-padded past the end). |
| `impl Parser::peek` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:47` | - |  |
| `impl Parser::peek` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:16` | - |  |
| `impl Parser<'a>::peek` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:18` | - |  |
| `impl Parser::peek` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:132` | - |  |
| `impl Parser<'_>::peek` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:66` | - |  |
| `impl Parser<'_>::peek` | 1 | `crates/manifold-bim-csg/src/step.rs:122` | - |  |
| `impl Parser<'a>::peek_binary_op` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/parser.rs:62` | - |  |
| `pegtop_light_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:310` | - | HDRI Pegtop Soft Light approximation. |
| `pegtop_light_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:379` | - |  |
| `pending_status` | 2 | `crates/multimodal-synthesis-schema/src/job_feed.rs:48` | - | Computes estimated progress percentage (0..=99) and a human-readable remaining time label. |
| `percent_decode` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:28` | - |  |
| `perfect_cube_scaled_jacobian_is_one` | 0 | `crates/kinetics-continuum-multiphysics/src/mesh/quality.rs:93` | - |  |
| `impl Vec2::perp` | 1 | `crates/drafting-geometry/src/vec.rs:55` | 2d | Rotated 90° counter-clockwise. |
| `perp_2d` | 1 | `crates/manifold-bim-csg/src/parametric.rs:46` | - |  |
| `perspective_grid_preset_and_projections` | 0 | `crates/vector-locus-geometry/src/perspective.rs:273` | - |  |
| `impl Task::pert_expected_days` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:339` | - |  |
| `impl Task::pert_std_dev` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:346` | - |  |
| `impl Shape::ph_type` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:216` | - |  |
| `pick_physical_device` | 3 | `crates/vector-skia-render-adapter/src/vulkan.rs:545` | - |  |
| `pin_light_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:301` | - | HDRI Pin Light: conditional min/max against thresholded source. |
| `pin_light_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:375` | - |  |
| `impl Transform::pipeline` | 1 | `crates/color-transforms/src/icc/transform.rs:94` | - |  |
| `impl VelloRendererOptions::pipeline_cache` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:133` | - |  |
| `impl RasterImage::pixel` | 3 | `crates/surface-pbr-studio/src/raster.rs:37` | 2d |  |
| `impl Raster::pixel` | 3 | `crates/vector-radiance-trace/src/lib.rs:75` | 2d |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::pixel_aspect_ratio` | 2 | `crates/pixel-buffer-output/src/builder.rs:110` | physics, 2d | Set the pixel aspect ratio to simulate non-square pixels. |
| `pixel_format` | 2 | `crates/vector-compute-shaders/shader/fine.wgsl:838` | 2d |  |
| `pixels` | 3 | `crates/asset-production-vibe/src/raster.rs:333` | - |  |
| `pkt` | 2 | `crates/media-ogg/src/tests.rs:51` | - | A CELT 20 ms packet (960 samples): TOC config 31, code 0. |
| `plain_english_is_ltr` | 0 | `crates/vector-glyph-shaping/src/bidi.rs:172` | - |  |
| `plain_math_lays_out` | 0 | `crates/discourse-academic-tex/src/typeset.rs:518` | - |  |
| `plain_mtext` | 1 | `crates/locus-parametric-solver/src/typography/mtext.rs:57` | - | Convert MTEXT formatting codes to plain text with `\n` line breaks. |
| `plain_stream` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/pes.rs:12` | - |  |
| `impl FlowInline::plain_text` | 1 | `crates/chronicle-office-codecs/src/flow/model.rs:28` | - |  |
| `impl FlowPara::plain_text` | 1 | `crates/chronicle-office-codecs/src/flow/model.rs:83` | - |  |
| `plain_text_words_chars_lines` | 0 | `crates/discourse-academic-tex/src/texparse/stats.rs:149` | - |  |
| `plan_reinit` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:784` | - |  |
| `planar_to_rgba` | 10 | `crates/video-h264-preview/src/convert.rs:57` | numerics | Planar 8-bit Y'CbCr with chroma subsampled by `cx` horizontally and `cy` vertically |
| `plane` | 3 | `crates/video-h264-codec/src/inter.rs:435` | - |  |
| `plane_allocator` | 0 | `crates/video-h264-codec/src/picture.rs:285` | - |  |
| `plane_offsets` | 0 | `crates/vector-locus-geometry/src/perspective.rs:299` | - |  |
| `impl ElasticIsotropic::plane_strain_matrix` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:74` | numerics | 3x3 plane strain elasticity matrix [eps_xx, eps_yy, gamma_xy]. |
| `impl ElasticIsotropic::plane_stress_matrix` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:62` | numerics | 3x3 plane stress elasticity matrix [eps_xx, eps_yy, gamma_xy]. |
| `planes_cmyk` | 2 | `crates/raster-blend-operators/src/lib.rs:273` | - | Combines blended CMY and K planes back into CMYK. |
| `planned_fraction_at` | 4 | `crates/kinetics-schedule-cpm/src/schedule.rs:584` | - |  |
| `impl Project::planned_task_cost` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:674` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:986` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:131` | - |  |
| `impl Stream::play` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:24` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:345` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:898` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:233` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/jack/stream.rs:210` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:126` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:474` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:188` | - |  |
| `impl StreamTrait for Stream::play` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:406` | - |  |
| `trait StreamTrait::play` | 1 | `crates/audio-device-cpal/src/traits.rs:213` | - | Run the stream. |
| `impl DeviceOutput::play` | 1 | `crates/pcm-device-output/src/device.rs:193` | - |  |
| `impl Vectorscope::plot` | 4 | `crates/radiance-color-grading/src/scopes/mod.rs:455` | - |  |
| `plus_and_1p1_costs` | 0 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:124` | - |  |
| `pmt` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:47` | - |  |
| `pmt_calc` | 5 | `crates/chronicle-tabular-engine/src/functions/financial.rs:13` | - |  |
| `png_bytes` | 3 | `crates/asset-production-vibe/src/operations_catalog.rs:1043` | - |  |
| `impl AssetHost<'_>::png_or_encode` | 3 | `crates/asset-production-vibe/src/graphics_pack.rs:104` | - |  |
| `png_round_trip_rgb_and_rgba` | 0 | `crates/media-test-support/src/golden.rs:292` | - |  |
| `pns_matches_ffmpeg` | 0 | `crates/audio-aac-codec/tests/decoder_oracle.rs:93` | - | PNS output is random by design, and ffmpeg's encoder does not emit it: build noise-substituted |
| `impl RefPic::poc` | 1 | `crates/video-h264-codec/src/picture.rs:328` | - |  |
| `poc_delta` | 2 | `crates/video-h264-codec/src/slicedec.rs:266` | - | H.264 temporal weighting uses clipped POC distances; widen before |
| `poc_overflow_is_rejected_and_normal_values_survive` | 0 | `crates/video-h264-codec/src/slice.rs:422` | 3d |  |
| `poc_type1_reordering` | 0 | `crates/video-h264-codec/src/synth_tests.rs:532` | - | POC type 1: non-reference pictures are displayed before the preceding reference picture. |
| `point` | 1 | `crates/asset-production-vibe/src/graphics.rs:485` | - |  |
| `impl SampleRange::point` | 1 | `crates/audio-timebase/src/rate.rs:96` | - |  |
| `impl GeodesicRuler::point` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:173` | - |  |
| `impl Model::point` | 3 | `crates/locus-parametric-solver/src/solver/model.rs:195` | 2d |  |
| `impl Vectorscope::point` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:447` | - |  |
| `impl Bounds::point` | 1 | `crates/surface-pbr-studio/src/math.rs:315` | 3d |  |
| `point3` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:352` | - |  |
| `point3` | 2 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:43` | - |  |
| `impl MotionPath::point_at` | 2 | `crates/layout-presentation-deck/src/timeline/path.rs:151` | - | Evaluate position (fractional x, y) at progress t (0.0 .. 1.0). |
| `impl Ray::point_at` | 2 | `crates/surface-pbr-studio/src/math.rs:283` | 3d |  |
| `impl GeodesicRuler::point_count` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:169` | - |  |
| `point_in_polygon` | 2 | `crates/drafting-geometry/src/curve.rs:576` | computational geometry, 2d | Even-odd point-in-polygon test. |
| `point_in_polygon` | 2 | `crates/vector-locus-pathops/src/pathops/boolean.rs:67` | computational geometry | Point-in-polygon test (winding number test). |
| `point_on_span` | 3 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:151` | - |  |
| `impl GeodesicRuler::points` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:165` | - |  |
| `points` | 2 | `crates/drafting-geometry/src/region.rs:206` | 2d, graph |  |
| `impl Patterns::points` | 2 | `crates/layout-paged-typesetter/src/hyphenation/liang.rs:70` | - | Break points (character indices where a hyphen may be inserted before that char) |
| `impl Profile::points` | 2 | `crates/profile-authoring/src/profile.rs:94` | - |  |
| `impl Unit::points` | 1 | `crates/vector-locus-geometry/src/units.rs:36` | - | Points per unit. |
| `points2` | 2 | `crates/asset-production-vibe/src/geometry.rs:354` | - |  |
| `points3` | 2 | `crates/asset-production-vibe/src/geometry.rs:365` | - |  |
| `points_and_nearest_distances_along_a_path` | 0 | `crates/vector-locus-geometry/src/arc.rs:84` | - |  |
| `points_conversion` | 0 | `crates/vector-locus-geometry/src/units.rs:193` | - |  |
| `poisson_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:128` | - |  |
| `impl Vec2::polar` | 3 | `crates/drafting-geometry/src/vec.rs:24` | 2d |  |
| `polar_grid` | 3 | `crates/vector-locus-geometry/src/shapes.rs:219` | 2d | Polar grid: concentric ellipses plus radial dividers. |
| `poll_descriptors_and_prepare_buffer` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:693` | - |  |
| `poly_len` | 1 | `crates/vector-locus-pathops/src/effects/util.rs:70` | - |  |
| `polyblep` | 2 | `crates/acoustics-daw-session/src/dsp/osc.rs:84` | - | PolyBLEP residual function for smoothing step discontinuities across [-dt, +dt]. |
| `polyfill` | 3 | `crates/surface-pbr-studio/src/paint.rs:183` | computational geometry, 2d | Fills a connected UV island or polygon bounding region with constant color. |
| `polygon` | 4 | `crates/vector-locus-geometry/src/shapes.rs:126` | computational geometry, 3d | Regular polygon with `sides` (≥ 3), first vertex straight up from the centre. |
| `polygon` | 2 | `crates/vector-radiance-trace/src/fit.rs:53` | computational geometry, 2d | The polygon approximating a closed loop of pixel-corner vertices within `tol`. |
| `polygon_area` | 1 | `crates/vector-locus-pathops/src/pathops/boolean.rs:40` | computational geometry | Signed area of a polyline ring. |
| `polygon_first_vertex_up` | 0 | `crates/vector-locus-geometry/src/shapes.rs:307` | computational geometry, 3d |  |
| `polygon_rotation` | 2 | `crates/vector-locus-geometry/src/recognize.rs:166` | computational geometry |  |
| `impl TraceParams::polygon_tolerance` | 1 | `crates/vector-radiance-trace/src/lib.rs:174` | computational geometry |  |
| `impl Bsp::polygons` | 1 | `crates/manifold-bim-csg/src/csg.rs:318` | - |  |
| `impl SubPath::polyline` | 2 | `crates/vector-locus-geometry/src/path.rs:192` | - | Polyline from points. |
| `polyline_arclength_point` | 2 | `crates/kinetics-garment-draping/src/pattern.rs:50` | - | Evaluates a point along an arc-length polyline at a normalized fraction in [0, 1]. |
| `polyline_area_and_length` | 0 | `crates/drafting-geometry/src/tests.rs:78` | - |  |
| `polyline_area_is_exact_for_bulge_segments` | 0 | `crates/drafting-geometry/src/tests.rs:275` | - |  |
| `polyline_area_is_stable_far_from_the_origin` | 0 | `crates/drafting-geometry/src/tests.rs:285` | - |  |
| `polyline_svg_d` | 1 | `crates/asset-production-vibe/src/locus_generative/mod.rs:324` | - |  |
| `polyline_svg_d` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:33` | - |  |
| `impl MinHeap::pop` | 1 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:87` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::pop_clip` | 1 | `crates/vector-compute-renderer/src/scene.rs:992` | - |  |
| `impl Scene::pop_clip` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:390` | - | Pop the most recently pushed clip path or clip rectangle from the clip stack. |
| `impl DrawSink for Scene::pop_clip_path` | 1 | `crates/vector-hybrid-renderer/src/text.rs:202` | - |  |
| `trait ProfilerCommandRecorder::pop_debug_group` | 1 | `crates/gpu-command-profiler/src/profiler_command_recorder.rs:7` | - |  |
| `impl Scene::pop_layer` | 1 | `crates/vector-compute-renderer/src/scene.rs:239` | - | Pops the current layer. |
| `impl ColorPainter for DrawColorGlyphs<'_>::pop_layer` | 1 | `crates/vector-compute-renderer/src/scene.rs:1038` | - |  |
| `impl PaintScene for VelloCpuScenePainter::pop_layer` | 1 | `crates/vector-cpu-render-adapter/src/scene.rs:112` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::pop_layer` | 1 | `crates/vector-gpu-render-adapter/src/scene.rs:86` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::pop_layer` | 1 | `crates/vector-hybrid-render-adapter/src/scene.rs:227` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::pop_layer` | 1 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:139` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::pop_layer` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:346` | - |  |
| `impl Scene::pop_layer` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:731` | - | Pop the last pushed layer. |
| `impl DrawSink for Scene::pop_layer` | 1 | `crates/vector-hybrid-renderer/src/text.rs:197` | - |  |
| `impl PaintScene for SkiaScenePainter<'_>::pop_layer` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:475` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::pop_transform` | 1 | `crates/vector-compute-renderer/src/scene.rs:955` | - |  |
| `impl Parser::pos` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:51` | - |  |
| `impl Parser::pos` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:90` | - |  |
| `impl BitReader<'a>::position` | 1 | `crates/bitstream-codec/src/lib.rs:48` | - | Bit position from the start. |
| `positions_are_one_based` | 0 | `crates/discourse-academic-tex/src/texparse/lex.rs:176` | - |  |
| `impl RevenueSchedule::post_due_entries` | 3 | `crates/chronicle-fiduciary-ledger/src/revrec.rs:97` | - | Recognises all unposted periods due on or before `as_of_date` by posting to `ledger`. |
| `impl JournalLedger::post_journal` | 6 | `crates/chronicle-fiduciary-ledger/src/journal.rs:100` | - | Post a balanced journal entry. Fails closed on imbalance, lock dates, or missing accounts. |
| `impl NQuin48::postcondition` | 3 | `crates/color-transforms/src/ontology.rs:23` | - |  |
| `impl NQuin48::postcondition` | 3 | `crates/radiance-camera-raw/src/ontology.rs:23` | - |  |
| `impl NQuin48::postcondition` | 3 | `crates/raster-layer-mask-adapter/src/ontology.rs:23` | - |  |
| `postprocess` | 1 | `crates/vector-compute-shaders/src/compile/mod.rs:255` | - |  |
| `pow43` | 1 | `crates/audio-aac-codec/src/tables.rs:103` | - | `x^(4/3)` for the integer quantised magnitudes 0..=8191. |
| `pow_1_5_signed` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:227` | - |  |
| `power` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:65` | - |  |
| `ppmt` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:103` | - |  |
| `impl HevcConfig::pps` | 1 | `crates/media-isobmff/src/codec.rs:421` | - |  |
| `pps` | 2 | `crates/video-h264-codec/src/synth_tests.rs:64` | - |  |
| `pps` | 0 | `crates/video-h264-preview/src/lib.rs:498` | - |  |
| `pps` | 0 | `crates/video-h264-preview/src/synth.rs:46` | - |  |
| `pq_and_hlg_reference_white` | 0 | `crates/color-transforms/src/transform.rs:387` | - |  |
| `pq_eotf` | 1 | `crates/color-transforms/src/lib.rs:214` | - | SMPTE ST 2084 EOTF, output normalised to 10 000 nits = 1.0. |
| `pq_eotf` | 1 | `crates/radiance-color-grading/src/matrix.rs:280` | - |  |
| `pq_inverse_eotf` | 1 | `crates/color-transforms/src/lib.rs:220` | - |  |
| `pq_inverse_eotf` | 1 | `crates/radiance-color-grading/src/matrix.rs:297` | - |  |
| `pq_known_nit_values` | 0 | `crates/color-transforms/src/grade.rs:140` | - |  |
| `impl Decoder::pre_skip` | 1 | `crates/audio-opus-codec/src/multistream.rs:58` | - | Pre-skip in samples at the output rate. |
| `impl Parser::preamble_body` | 3 | `crates/discourse-academic-tex/src/texparse/bib.rs:179` | - |  |
| `impl BinOp::precedence` | 1 | `crates/chronicle-tabular-engine/src/formula/ast.rs:139` | - |  |
| `precedents` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:382` | - |  |
| `impl NQuin48::precondition` | 3 | `crates/color-transforms/src/ontology.rs:19` | - |  |
| `impl NQuin48::precondition` | 3 | `crates/radiance-camera-raw/src/ontology.rs:19` | - |  |
| `impl NQuin48::precondition` | 3 | `crates/raster-layer-mask-adapter/src/ontology.rs:19` | - |  |
| `pred16x16` | 6 | `crates/video-h264-codec/src/intra.rs:228` | - | Intra_16x16 prediction (8.3.3) for the macroblock at luma (x, y). |
| `pred4x4` | 6 | `crates/video-h264-codec/src/intra.rs:188` | - | Intra_4x4 prediction written into `plane` at (x, y). |
| `pred8x8` | 6 | `crates/video-h264-codec/src/intra.rs:194` | - | Intra_8x8 prediction (with reference sample filtering) written into `plane` at (x, y). |
| `pred_chroma` | 6 | `crates/video-h264-codec/src/intra.rs:292` | - | Chroma intra prediction for a 4:2:0 macroblock (8x8 block at chroma (x, y)), 8.3.4. |
| `pred_nxn` | 6 | `crates/video-h264-codec/src/intra.rs:63` | graph | Directional / DC prediction shared by Intra_4x4 and Intra_8x8 (8.3.1.2.x, 8.3.2.2.x). |
| `predict_block` | 9 | `crates/video-h264-codec/src/slicedec.rs:241` | - | Interpolate a bw x bh luma block at (px, py) with motion vector `mv` from reference `f` into `y`, |
| `impl SliceDecoder<'a>::predict_inter` | 1 | `crates/video-h264-codec/src/slicedec.rs:824` | - | Inter prediction for the whole current MB, using the stored motion. |
| `predict_kriging` | 3 | `crates/geodesy-spatial-analysis/src/kriging.rs:186` | numerics | Solves Ordinary Kriging linear system for a target point. |
| `prefix_free` | 1 | `crates/video-h264-codec/src/cavlc.rs:191` | - |  |
| `preflight` | 2 | `crates/profile-authoring/src/assembly.rs:98` | - |  |
| `preflight` | 6 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:106` | 2d | Validates shape, radius, coverage, and caller-selected limits without allocating algorithm |
| `preflight` | 6 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:67` | 2d | Validates shape, radius, coverage, and caller-selected limits without allocating algorithm |
| `preflight` | 6 | `crates/raster-selection-morphology/src/lib.rs:136` | - | Validates shape, radius, coverage, and caller-selected limits without allocating algorithm buffers. |
| `preflight_dimensions` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:69` | 2d | Validates shape, radius, coverage, and caller-selected limits without allocating algorithm |
| `preflight_dimensions` | 5 | `crates/raster-selection-morphology/src/lib.rs:73` | - | Validates shape, radius, and caller-selected limits before any buffer conversion/allocation. |
| `preflight_mask` | 6 | `crates/raster-layer-mask-adapter/src/lib.rs:64` | - | Admits a checked edit without converting or allocating the source mask. |
| `preflight_rejects_shape_nonfinite_coverage_and_limits` | 0 | `crates/raster-selection-morphology/src/tests.rs:81` | - |  |
| `premul_alpha` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:1400` | - |  |
| `premultiply` | 0 | `crates/vector-cpu-render-adapter/src/image_convert.rs:72` | - |  |
| `premultiply_bgra_opaque` | 0 | `crates/vector-cpu-render-adapter/src/image_convert.rs:82` | - |  |
| `impl DeviceOutput::prepare` | 6 | `crates/pcm-device-output/src/device.rs:63` | - | `live` is called on the native device worker, NEVER inside the realtime |
| `impl WebGlPrograms::prepare` | 9 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1575` | - | Prepare resources for rendering. |
| `impl Programs::prepare` | 9 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2137` | - | Prepare GPU buffers for rendering, given alphas. |
| `impl SkiaBackend for MetalBackend::prepare` | 1 | `crates/vector-skia-render-adapter/src/metal.rs:107` | - |  |
| `impl SkiaBackend for OpenGLBackend::prepare` | 1 | `crates/vector-skia-render-adapter/src/opengl.rs:212` | - |  |
| `impl SkiaBackend for VulkanBackend::prepare` | 1 | `crates/vector-skia-render-adapter/src/vulkan.rs:416` | - |  |
| `trait SkiaBackend::prepare` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:12` | - |  |
| `impl WebGlRenderer::prepare_gpu_encoded_paints` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:870` | - |  |
| `impl Renderer::prepare_gpu_encoded_paints` | 5 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:668` | - |  |
| `impl VulkanBackend::prepare_intermediate_surface` | 1 | `crates/vector-skia-render-adapter/src/vulkan.rs:240` | - | Prepares a Skia-allocated intermediate surface. |
| `impl WebGlPrograms::prepare_intermediate_textures` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1607` | - |  |
| `impl Programs::prepare_intermediate_textures` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1797` | - |  |
| `prepare_slice` | 4 | `crates/video-prores-codec/src/encode.rs:291` | - |  |
| `prepare_stream` | 3 | `crates/pcm-device-output/src/device.rs:234` | - |  |
| `impl VulkanBackend::prepare_swapchain_surface` | 2 | `crates/vector-skia-render-adapter/src/vulkan.rs:281` | - | Prepares a Skia surface that wraps the raw Vulkan swapchain image. |
| `impl Out::prepend` | 2 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:17` | - |  |
| `preprocess` | 4 | `crates/vector-compute-shaders/src/compile/preprocess.rs:37` | 3d |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::present_mode` | 2 | `crates/pixel-buffer-output/src/builder.rs:142` | - | Optimistically set the `wgpu` present mode. |
| `impl Pixels<'win>::present_mode` | 1 | `crates/pixel-buffer-output/src/lib.rs:458` | - | Get the `wgpu` present mode. |
| `presentation_duration` | 2 | `crates/media-isobmff/src/mux.rs:190` | - |  |
| `impl Track::presentation_pts` | 2 | `crates/media-isobmff/src/demux.rs:154` | - | Presentation (edit-adjusted) pts of sample `i`. |
| `impl PerspectiveGrid::preset` | 2 | `crates/vector-locus-geometry/src/perspective.rs:115` | computational geometry, 2d | Preset for `kind` (1, 2, or 3) fitted to a bounding box. |
| `preset` | 1 | `crates/vector-radiance-trace/src/lib.rs:241` | - |  |
| `presets` | 0 | `crates/vector-radiance-trace/src/lib.rs:292` | - |  |
| `pressured` | 3 | `crates/vector-surface-brush/src/calligraphic.rs:121` | - |  |
| `impl ShowState::prev` | 2 | `crates/layout-presentation-deck/src/timeline/show.rs:101` | - |  |
| `price_bond` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:496` | economics |  |
| `pricedisc` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:595` | - |  |
| `impl Gamut::primaries` | 1 | `crates/color-transforms/src/spaces.rs:42` | 3d | (x, y) of R, G, B. Sources: ITU-R BT.709-6, BT.2020-2, SMPTE EG 432-1 (P3), and the camera |
| `impl Gamut::primaries` | 1 | `crates/radiance-color-grading/src/spaces.rs:35` | - | (x, y) coordinates of R, G, B primaries. |
| `impl Parser::primary` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:118` | - |  |
| `impl Encoder::priming_samples` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:247` | - | Encoder delay: the first 1024 decoded samples are priming and should be skipped (MP4 edit |
| `print` | 1 | `crates/audio-aac-codec/tests/encoder_oracle.rs:106` | - |  |
| `print` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:8` | - |  |
| `print_arc_segments` | 3 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:45` | - | Print-density sample count for a circular sweep; `min_seg` is the origin floor. |
| `print_cubic_n` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:108` | - |  |
| `priority_flood_fill` | 3 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:119` | - | Condition a heightfield by filling all internal depression sinks to their spillway elevation. |
| `priority_flood_fills_sink_hole` | 0 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:308` | - |  |
| `probe` | 1 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:83` | - |  |
| `impl WebGlRenderer::probe` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:75` | 2d | Conduct a probing operation. |
| `probe` | 1 | `crates/video-prores-codec/src/decode.rs:24` | - | Parse only the frame header (cheap; for probing geometry and format). |
| `impl WebGlRenderer::probe_inner` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:86` | - |  |
| `probe_json` | 2 | `crates/media-isobmff/tests/common/mod.rs:149` | - |  |
| `probe_prores_mov` | 1 | `crates/video-prores-codec/src/mov.rs:64` | - | Probe a QuickTime `.mov` container and return information about its first ProRes video track. |
| `probe_scene_uses_depth_buffer_when_enabled` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:18` | - |  |
| `impl BrushTipStamp::procedural_round` | 2 | `crates/surface-pbr-studio/src/probrush.rs:49` | - | Generates a procedural circular tip mask with smooth falloff. |
| `impl WsolaTimeStretch::process` | 3 | `crates/acoustics-daw-session/src/elastic_warp/mod.rs:88` | - | Stretches input audio samples to output length using WSOLA. |
| `impl jack::ProcessHandler for LocalProcessHandler::process` | 3 | `crates/audio-device-cpal/src/host/jack/stream.rs:282` | - |  |
| `impl Fft::process` | 3 | `crates/audio-opus-codec/src/celt/mdct.rs:51` | - |  |
| `impl Resampler::process` | 3 | `crates/audio-opus-codec/src/silk/resampler.rs:132` | - | Resamples `input` (16-bit scale) and appends the output samples. |
| `impl BandpassFilter::process` | 2 | `crates/electromagnetics-rf-sensing/src/vitals.rs:63` | - |  |
| `impl BiquadFilter::process_buffer` | 2 | `crates/acoustics-daw-session/src/dsp/biquad.rs:180` | - |  |
| `process_commands` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:222` | - |  |
| `process_commands_and_await_signal` | 2 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:392` | - |  |
| `impl GpuProfiler::process_finished_frame` | 2 | `crates/gpu-command-profiler/src/profiler.rs:493` | - | Checks if all timer queries for the oldest pending finished frame are done and returns that snapshot if any. |
| `process_frame` | 3 | `crates/audio-device-cpal/examples/synth_tones.rs:176` | - |  |
| `process_input` | 5 | `crates/audio-device-cpal/src/host/alsa/mod.rs:783` | - |  |
| `process_input` | 4 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:425` | - |  |
| `process_input_callback` | 7 | `crates/audio-device-cpal/src/host/asio/stream.rs:87` | - | 1. Write from the ASIO buffer to the interleaved CPAL buffer. |
| `impl EbuR128Meter::process_interleaved` | 2 | `crates/acoustics-daw-session/src/dsp/meter.rs:75` | - | Feeds interleaved multi-channel audio samples into the loudness meter. |
| `process_output` | 7 | `crates/audio-device-cpal/src/host/alsa/mod.rs:810` | - |  |
| `process_output` | 4 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:494` | - |  |
| `process_output_callback` | 8 | `crates/audio-device-cpal/src/host/asio/stream.rs:304` | - | 1. Render the given callback to the given buffer of interleaved samples. |
| `impl BiquadFilter::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/biquad.rs:166` | - |  |
| `impl Eq7::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:28` | - |  |
| `impl Compressor::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:62` | - |  |
| `impl ExpanderGate::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:111` | - |  |
| `impl Maximizer::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:162` | - |  |
| `impl DeEsser::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:208` | - |  |
| `impl ChannelStrip::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:234` | - |  |
| `impl FdnReverb::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:282` | - |  |
| `impl SubtractiveSynth::process_sample` | 1 | `crates/acoustics-daw-session/src/dsp/plugins.rs:350` | - |  |
| `impl TruePeakMeter::process_samples` | 2 | `crates/acoustics-daw-session/src/dsp/meter.rs:150` | - | Feeds samples and updates the max inter-sample true-peak value. |
| `impl GpuProfiler::process_timings_recursive` | 3 | `crates/gpu-command-profiler/src/profiler.rs:740` | - |  |
| `impl AssetHost<'_>::produce_locked_dt_maps` | 4 | `crates/asset-production-vibe/src/chronicle_ops.rs:185` | - | Locked-dt ProRes + OTIO + Matroska + fiduciary hash-chain maps. |
| `produce_safe_container_svg` | 4 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:191` | - | Display-referred sRGB SVG card. Binary containers are never inlined as executable payloads. |
| `produce_svg` | 4 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:416` | - |  |
| `impl AssetHost<'_>::produce_wifi_csi_vitals_aoa` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:489` | 3d | 802.11 CSI → sanitized phase, contactless vitals, MUSIC AoA, camera-free receipt. |
| `product` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:12` | - |  |
| `impl Parser<'_>::product` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:85` | - |  |
| `impl AssetHost<'_>::profile` | 2 | `crates/asset-production-vibe/src/geometry.rs:288` | - |  |
| `impl Builtin::profile` | 1 | `crates/color-transforms/src/icc/builtin.rs:77` | - |  |
| `profile_from_path` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:195` | - |  |
| `profile_sweep` | 4 | `crates/profile-authoring/src/geometry.rs:82` | computational geometry | A metric profile swept along a finite 3-D axis. Concave end caps reuse the |
| `progress_and_eta_calculation` | 0 | `crates/multimodal-synthesis-schema/src/job_feed.rs:94` | - |  |
| `impl Line::project` | 2 | `crates/drafting-geometry/src/curve.rs:39` | 2d | Foot of the perpendicular on the infinite line. |
| `impl AssetHost<'_>::project_diegetic_surface` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/diegetic_input.rs:8` | - |  |
| `impl AssetHost<'_>::project_metamer_nullspace` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/vox0_metamer.rs:373` | - | Exposes Etalon's 41-band spectral metamer fiber decomposition: |
| `impl AssetHost<'_>::project_ui_svg` | 2 | `crates/asset-production-vibe/src/layout_ops/sheet_render.rs:258` | - | Projects the resolved layout into a standalone SVG vector resource with scoped CSS tokens. |
| `project_video_container` | 1 | `crates/video-h264-preview/src/lib.rs:105` | 3d | Select one video track without changing source bytes or moving media. For a |
| `propagate_clipping_mask_alpha` | 2 | `crates/raster-layer-mask-adapter/src/masks.rs:86` | - | Propagates clipping mask base alpha onto a clipped layer's alpha buffer. |
| `property_listener_handler_shim` | 4 | `crates/audio-device-cpal/src/host/coreaudio/macos/property_listener.rs:76` | - | Callback used to call user-provided closure as a property listener. |
| `prophoto_rgb` | 0 | `crates/color-transforms/src/icc/builtin.rs:95` | - |  |
| `propose_edits` | 2 | `crates/discourse-academic-tex/src/review/ai_repair.rs:45` | - | Propose a set of deterministic line edits to repair document structure. |
| `propose_then_apply` | 0 | `crates/discourse-academic-tex/src/review/suggest.rs:102` | - |  |
| `impl SampleEntry::prores` | 3 | `crates/media-isobmff/src/codec.rs:1285` | - | ProRes entry; `fourcc` one of `apch`/`apcn`/`apcs`/`apco`/`ap4h`/`ap4x`. |
| `prores_mov` | 1 | `crates/video-h264-preview/src/synth.rs:187` | - | QuickTime MOV containing `n` frames of ProRes 422 HQ (64x48) at 25 fps. |
| `prune` | 1 | `crates/drafting-geometry/src/region.rs:269` | graph | The graph without dangling ends: vertices of degree one are removed until none remain, so |
| `prune` | 3 | `crates/vector-radiance-trace/src/centerline.rs:407` | - |  |
| `impl Dpb<F>::prune` | 1 | `crates/video-h264-codec/src/dpb.rs:74` | - | Remove entries not needed for output and not used for reference. |
| `prune_dead_nodes` | 1 | `crates/layout-nodegraph-compiler/src/compiler/cse.rs:153` | - | Dead-code elimination: |
| `pseudo_permute` | 3 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:175` | - |  |
| `pseudo_rand` | 3 | `crates/raster-layer-mask-adapter/src/brush.rs:157` | - |  |
| `impl SubpathBuilder<'a>::pt` | 3 | `crates/asset-production-vibe/src/glyph/mod.rs:153` | - |  |
| `impl GroupReader<'r, 'a>::pt` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:166` | - |  |
| `pt_from` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1337` | - |  |
| `impl Frame::publish` | 3 | `crates/video-h264-codec/src/picture.rs:155` | - | Publish MB row `r` (ignored if already published). |
| `published_reference_values` | 0 | `crates/color-transforms/src/log.rs:291` | - | Reference points from the published specifications (code values are 10-bit). |
| `pubmed_forms` | 0 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:163` | - |  |
| `pucker_bloat` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:63` | 2d | Pucker & Bloat: anchors move toward/away from center, handles move opposite. |
| `pucker_bloat_alters_path` | 0 | `crates/vector-locus-pathops/src/effects/distort.rs:262` | - |  |
| `impl Mode::pulses2bits` | 4 | `crates/audio-opus-codec/src/celt/rate.rs:136` | - |  |
| `impl Metadata::push` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:114` | - |  |
| `impl Out::push` | 2 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:14` | - |  |
| `impl Trace::push` | 3 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:20` | - |  |
| `impl MinHeap::push` | 2 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:73` | - |  |
| `impl Recording::push` | 2 | `crates/vector-compute-renderer/src/recording.rs:118` | - | Appends a [`Command`] to the back of the [`Recording`]. |
| `impl Draw::push` | 4 | `crates/vector-hybrid-renderer/src/draw.rs:38` | - |  |
| `impl OpaqueDraw::push` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:72` | - |  |
| `impl FilterContext::push` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:625` | - |  |
| `impl Ranges::push` | 2 | `crates/vector-hybrid-renderer/src/util.rs:121` | - |  |
| `impl Scene::push_blend_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:707` | - | Push a new blend layer. |
| `impl DrawSink for Scene::push_blend_layer` | 2 | `crates/vector-hybrid-renderer/src/text.rs:192` | - |  |
| `impl Round::push_blend_op` | 4 | `crates/vector-hybrid-renderer/src/schedule/round.rs:241` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::push_clip_box` | 2 | `crates/vector-compute-renderer/src/scene.rs:977` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::push_clip_glyph` | 2 | `crates/vector-compute-renderer/src/scene.rs:959` | - |  |
| `impl Scene::push_clip_layer` | 4 | `crates/vector-compute-renderer/src/scene.rs:180` | - | Pushes a new layer clipped by the specified `clip` shape. |
| `impl PaintScene for VelloCpuScenePainter::push_clip_layer` | 4 | `crates/vector-cpu-render-adapter/src/scene.rs:105` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::push_clip_layer` | 4 | `crates/vector-gpu-render-adapter/src/scene.rs:82` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::push_clip_layer` | 4 | `crates/vector-hybrid-render-adapter/src/scene.rs:219` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::push_clip_layer` | 4 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:131` | - |  |
| `impl Scene::push_clip_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:702` | - | Push a new clip layer. |
| `impl DrawSink for Scene::push_clip_layer` | 2 | `crates/vector-hybrid-renderer/src/text.rs:177` | - |  |
| `impl PaintScene for SkiaScenePainter<'_>::push_clip_layer` | 4 | `crates/vector-skia-render-adapter/src/scene.rs:462` | - |  |
| `impl Scene::push_clip_path` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:365` | - | Push a new clip path to the clip stack. |
| `impl DrawSink for Scene::push_clip_path` | 2 | `crates/vector-hybrid-renderer/src/text.rs:182` | - |  |
| `impl Scene::push_clip_rect` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:376` | 2d | Push a rectangular clip path. |
| `impl DrawSink for Scene::push_clip_rect` | 2 | `crates/vector-hybrid-renderer/src/text.rs:187` | 2d |  |
| `impl Stream::push_command` | 2 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:166` | - |  |
| `impl FilterPassBuilder<'a>::push_copy_to_scratch_pass` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:606` | - |  |
| `push_cubic` | 6 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:26` | - |  |
| `push_curve` | 5 | `crates/vector-locus-geometry/src/path.rs:502` | - |  |
| `trait ProfilerCommandRecorder::push_debug_group` | 2 | `crates/gpu-command-profiler/src/profiler_command_recorder.rs:6` | - |  |
| `push_dep` | 4 | `crates/discourse-academic-tex/src/texparse/graph.rs:202` | - |  |
| `impl DrawBuilder<'a, T>::push_draw` | 4 | `crates/vector-hybrid-renderer/src/draw.rs:145` | - |  |
| `impl Scheduler<'a, 'p>::push_draws` | 4 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:570` | - | Append a recorded draw range to the next compatible draw pass for the target. |
| `impl vello_common::probe::ProbeRenderer for Scene::push_filter_layer` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:342` | - |  |
| `impl Scene::push_filter_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:726` | - | Push a new filter layer. |
| `impl Round::push_filter_op` | 4 | `crates/vector-hybrid-renderer/src/schedule/round.rs:253` | - |  |
| `impl Scene::push_layer` | 6 | `crates/vector-compute-renderer/src/scene.rs:101` | - | Pushes a new layer clipped by the specified shape and composed with |
| `impl ColorPainter for DrawColorGlyphs<'_>::push_layer` | 2 | `crates/vector-compute-renderer/src/scene.rs:1011` | - |  |
| `impl PaintScene for VelloCpuScenePainter::push_layer` | 8 | `crates/vector-cpu-render-adapter/src/scene.rs:73` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::push_layer` | 8 | `crates/vector-gpu-render-adapter/src/scene.rs:69` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::push_layer` | 8 | `crates/vector-hybrid-render-adapter/src/scene.rs:199` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::push_layer` | 8 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:111` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::push_layer` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:338` | - |  |
| `impl Scene::push_layer` | 6 | `crates/vector-hybrid-renderer/src/scene.rs:636` | - | Push a new layer with the given properties. |
| `impl PaintScene for SkiaScenePainter<'_>::push_layer` | 8 | `crates/vector-skia-render-adapter/src/scene.rs:428` | - |  |
| `impl Rounds::push_layer_clear` | 4 | `crates/vector-hybrid-renderer/src/schedule/round.rs:73` | 2d |  |
| `impl DrawBuilder<'a, T>::push_layer_fill` | 5 | `crates/vector-hybrid-renderer/src/draw.rs:270` | - |  |
| `impl DrawBuilder<'a, T>::push_layer_fill_segment` | 6 | `crates/vector-hybrid-renderer/src/draw.rs:334` | - |  |
| `impl Scene::push_layer_inner` | 5 | `crates/vector-compute-renderer/src/scene.rs:190` | - | Helper for logic shared between [`Self::push_layer`] and [`Self::push_luminance_mask_layer`] |
| `impl Scene::push_luminance_mask_layer` | 5 | `crates/vector-compute-renderer/src/scene.rs:146` | - | Pushes a new layer clipped by the specified shape and treated like a luminance |
| `impl Scene::push_mask_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:721` | - | Push a new mask layer. |
| `push_nquin_48` | 10 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:575` | semantics | Packs a single 48-byte Super-Quin (`NQuin`: `subject`, `predicate`, `object`, `context`, |
| `impl Scene::push_opacity_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:712` | - | Push a new opacity layer. |
| `impl DrawBuilder<'a, T>::push_opaque` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:159` | - |  |
| `impl FlowDoc::push_para` | 2 | `crates/chronicle-office-codecs/src/flow/model.rs:134` | - |  |
| `impl DrawBuilder<'a, T>::push_path` | 4 | `crates/vector-hybrid-renderer/src/draw.rs:168` | - |  |
| `push_pt` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:140` | - |  |
| `push_range` | 7 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:342` | - |  |
| `trait VecExt::push_ranged` | 3 | `crates/vector-hybrid-renderer/src/util.rs:145` | - |  |
| `impl VecExt<T> for Vec<T>::push_ranged` | 3 | `crates/vector-hybrid-renderer/src/util.rs:151` | - |  |
| `impl DrawBuilder<'a, T>::push_rect` | 4 | `crates/vector-hybrid-renderer/src/draw.rs:228` | 2d |  |
| `impl Tables::push_run` | 3 | `crates/media-isobmff/src/mux.rs:112` | - |  |
| `push_surface` | 2 | `crates/discourse-academic-tex/src/texparse/structure.rs:379` | - |  |
| `impl FlowDoc::push_table` | 2 | `crates/chronicle-office-codecs/src/flow/model.rs:138` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::push_transform` | 2 | `crates/vector-compute-renderer/src/scene.rs:949` | - |  |
| `impl Enc::put` | 2 | `crates/video-h264-codec/src/cabac.rs:219` | - |  |
| `impl Enc::put` | 3 | `crates/video-h264-codec/src/synth_tests.rs:222` | - |  |
| `impl Lru<K>::put` | 3 | `crates/video-h264-preview/src/movie.rs:105` | - |  |
| `trait Sink::put` | 3 | `crates/video-prores-codec/src/bits.rs:148` | - |  |
| `impl Sink for Counter::put` | 3 | `crates/video-prores-codec/src/bits.rs:158` | - |  |
| `impl Sink for Writer::put` | 3 | `crates/video-prores-codec/src/bits.rs:191` | - |  |
| `put_block` | 9 | `crates/video-prores-codec/src/decode.rs:466` | - |  |
| `impl SliceDecoder<'a>::put_chroma_ac` | 4 | `crates/video-h264-codec/src/slicedec.rs:1189` | - | Chroma AC levels (scan positions 1..16 in levels[1..]) for block `b` of component `c`. |
| `impl SliceDecoder<'a>::put_chroma_dc` | 3 | `crates/video-h264-codec/src/slicedec.rs:1174` | - | Chroma DC levels (4:2:0: 4 values in raster order) for component `c`. |
| `trait Sink::put_cw` | 3 | `crates/video-prores-codec/src/bits.rs:149` | - |  |
| `impl Sink for Counter::put_cw` | 3 | `crates/video-prores-codec/src/bits.rs:162` | - |  |
| `impl Sink for Writer::put_cw` | 3 | `crates/video-prores-codec/src/bits.rs:206` | - |  |
| `put_descriptor` | 3 | `crates/media-isobmff/src/codec.rs:1193` | - |  |
| `impl SliceDecoder<'a>::put_luma4` | 4 | `crates/video-h264-codec/src/slicedec.rs:1116` | - | Store a 4x4 luma block given in scan order (levels[0..16]); `ac_only` skips position 0. |
| `impl SliceDecoder<'a>::put_luma8` | 3 | `crates/video-h264-codec/src/slicedec.rs:1135` | - | Store an 8x8 luma block given in scan order. |
| `impl SliceDecoder<'a>::put_luma_dc` | 2 | `crates/video-h264-codec/src/slicedec.rs:1157` | - | Intra16x16 DC levels in scan order -> scaled DC of each 4x4 block. |
| `pv` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:64` | - |  |
| `pv_calc` | 5 | `crates/chronicle-tabular-engine/src/functions/financial.rs:29` | - |  |
| `pvq_counts` | 0 | `crates/audio-opus-codec/src/celt/rate.rs:562` | - |  |
| `pvq_roundtrip_all_codewords_small` | 0 | `crates/audio-opus-codec/src/celt/bands.rs:929` | - |  |
| `pvq_v` | 2 | `crates/audio-opus-codec/src/celt/rate.rs:47` | - | PVQ codebook size V(N, K) as u128 (saturating far above 2^32). |
| `impl Shaper::pyramid` | 2 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:225` | - |  |
| `q_hash` | 1 | `crates/surface-layered-codecs/src/ontology.rs:44` | - |  |
| `qfactors` | 2 | `crates/video-prores-codec/src/decode.rs:242` | numerics | Dequantisation factors (matrix x scale) for a slice. |
| `impl SliceHeader::qp` | 2 | `crates/video-h264-codec/src/slice.rs:146` | - |  |
| `qpc` | 0 | `crates/video-h264-codec/src/tables.rs:170` | - |  |
| `qr` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:467` | numerics | Householder QR decomposition: A = Q R. |
| `qscale` | 1 | `crates/video-prores-codec/src/tables.rs:96` | - | Quantisation scale for a slice `quantization_index` (1..=224). |
| `qscale_mapping` | 0 | `crates/video-prores-codec/src/tables.rs:140` | - |  |
| `qt_pcm` | 2 | `crates/media-isobmff/src/codec.rs:1068` | - |  |
| `impl OutlinePen for BezPathOutline::quad_to` | 5 | `crates/vector-compute-renderer/src/scene.rs:1084` | - |  |
| `quality_label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:574` | - |  |
| `quant_all_bands` | 19 | `crates/audio-opus-codec/src/celt/bands.rs:664` | - | Decodes all band shapes (`quant_all_bands`, decoder side). |
| `impl BandCtx<'_, '_, '_>::quant_band` | 10 | `crates/audio-opus-codec/src/celt/bands.rs:469` | - |  |
| `impl BandCtx<'_, '_, '_>::quant_band_n1` | 4 | `crates/audio-opus-codec/src/celt/bands.rs:346` | - |  |
| `impl BandCtx<'_, '_, '_>::quant_band_stereo` | 10 | `crates/audio-opus-codec/src/celt/bands.rs:551` | - |  |
| `quant_coef` | 2 | `crates/audio-aac-codec/src/tns.rs:128` | - | Quantise a reflection coefficient (inverse of [`dequant_coef`], nearest index). |
| `impl BandCtx<'_, '_, '_>::quant_partition` | 9 | `crates/audio-opus-codec/src/celt/bands.rs:366` | - |  |
| `quantise` | 5 | `crates/audio-aac-codec/src/encoder/quant.rs:94` | - | Quantise a slice at `sf`, returning the distortion; writes the (signed) values to `out` if given. |
| `quantise` | 3 | `crates/video-prores-codec/src/encode.rs:207` | - |  |
| `impl Encoder::quantise_all` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:602` | - |  |
| `quantise_all` | 3 | `crates/video-prores-codec/src/encode.rs:320` | - |  |
| `quantise_channel` | 2 | `crates/audio-aac-codec/src/encoder/quant.rs:155` | - | Quantise one channel at rate-distortion multiplier `lambda` and select sections. |
| `quantize` | 2 | `crates/vector-radiance-trace/src/quantize.rs:21` | - | Reduce `img` to a palette according to `params.mode`. |
| `quantize_pga_motor` | 4 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:13` | - | Quantizes an 8-float PGA motor state (position [x,y,z], rotation [qx,qy,qz,qw], sequence, entity_id) |
| `quat_from_to` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:390` | - |  |
| `impl AssetHost<'_>::query_anatomical_ontology_pin` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rig_ontology_continuum_frustum.rs:195` | 3d, semantics | Queries the closest canonical anatomical ontology pin (`CANONICAL_ANATOMICAL_PINS`, `32` pins) |
| `impl AssetHost<'_>::query_direct_space` | 2 | `crates/asset-production-vibe/src/kinetics_sim/motor_xpbd_space.rs:277` | - | Synchronous DirectSpaceState spatial queries over split Static/Dynamic BVH. |
| `impl BufferRenderer::queue` | 1 | `crates/gpu-device-context/src/buffer_renderer.rs:67` | - |  |
| `impl SurfaceRenderer<'s>::queue` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:212` | - |  |
| `queue` | 0 | `crates/pcm-device-output/src/queue_tests.rs:40` | - |  |
| `impl Pixels<'win>::queue` | 1 | `crates/pixel-buffer-output/src/lib.rs:745` | - | Provides access to the internal [`wgpu::Queue`]. |
| `quicktime_mpeg_audio_entries` | 0 | `crates/media-isobmff/src/codec.rs:1651` | - | QuickTime's `.mp2` entry (what ffmpeg writes for MP2 in MOV) is MPEG audio, as `.mp3` is, |
| `quote_sheet` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:25` | - |  |
| `impl Parser::quoted` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:334` | - |  |
| `quoted_values_and_comments` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:461` | - |  |
| `r_at` | 2 | `crates/locus-parametric-solver/src/solver/residual.rs:121` | - |  |
| `r_at_self` | 2 | `crates/locus-parametric-solver/src/solver/residual.rs:499` | - |  |
| `rad` | 1 | `crates/drafting-geometry/src/lib.rs:66` | - |  |
| `rad_to_deg` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1006` | - |  |
| `impl J2Plasticity::radial_return` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:178` | - | Classical radial return algorithm: updates stress, backstress, and equivalent plastic strain. |
| `impl AssetHost<'_>::radiance_apply_lut` | 2 | `crates/asset-production-vibe/src/radiance_color.rs:185` | - |  |
| `impl AssetHost<'_>::radiance_build_light_tree` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:492` | 3d | 5. Hierarchical light tree bounding-cone clustering and direct lighting preview |
| `impl AssetHost<'_>::radiance_color_op` | 3 | `crates/asset-production-vibe/src/radiance_color.rs:59` | - |  |
| `impl AssetHost<'_>::radiance_compositor_gi_op` | 3 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:68` | - |  |
| `impl AssetHost<'_>::radiance_convert_transfer` | 2 | `crates/asset-production-vibe/src/radiance_color.rs:385` | - |  |
| `impl AssetHost<'_>::radiance_decode_cryptomatte_q42` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/filters_cryptomatte.rs:234` | - | 2. Cryptomatte Q42 ID coverage matte extraction and isolated compositing |
| `impl AssetHost<'_>::radiance_filter_kuwahara_anisotropic` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/filters_cryptomatte.rs:10` | graph | 1. Anisotropic Kuwahara structure-tensor edge-preserving filter |
| `impl AssetHost<'_>::radiance_grade_color` | 2 | `crates/asset-production-vibe/src/radiance_color.rs:252` | - |  |
| `impl AssetHost<'_>::radiance_modify_mask` | 2 | `crates/asset-production-vibe/src/radiance_color.rs:75` | - |  |
| `impl AssetHost<'_>::radiance_photo_op` | 3 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:10` | - |  |
| `impl AssetHost<'_>::radiance_solve_sdfgi_froxel` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:282` | numerics | 4. Cascaded SDFGI irradiance probes and 3D froxel atmospheric scattering |
| `impl AssetHost<'_>::radiance_vector_motion_blur` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/filters_cryptomatte.rs:311` | - | 3. Vector motion blur line-integral convolution with Hann shutter window |
| `radii_stop_at_half_the_shorter_side_per_corner` | 0 | `crates/vector-locus-geometry/src/corners.rs:222` | - |  |
| `impl Bounds::radius` | 1 | `crates/surface-pbr-studio/src/math.rs:339` | - |  |
| `impl FresnelZoneGeometry::radius_at_distance` | 3 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:242` | - | Calculates the radius $r_n$ of the $n$-th Fresnel zone at distance $d_1$ from Tx along the LOS path. |
| `impl Corner::radius_change` | 2 | `crates/vector-locus-geometry/src/corners.rs:91` | 2d | How much the radius grows when the centre of the round cut moves by `d`: `d`'s part along |
| `ramp` | 3 | `crates/media-test-support/src/golden.rs:286` | - |  |
| `ramp_fills_every_bin_and_waveform_row_equals_code_value` | 0 | `crates/radiance-color-grading/src/scopes/tests.rs:55` | - |  |
| `trait Host::random` | 1 | `crates/chronicle-tabular-engine/src/calc/eval.rs:40` | - |  |
| `impl Host for PassHost<'_>::random` | 1 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:263` | - |  |
| `random_configs_and_adts_never_panic` | 0 | `crates/audio-aac-codec/tests/robustness.rs:68` | - |  |
| `random_mb` | 1 | `crates/video-h264-codec/src/synth_tests.rs:343` | - |  |
| `randomized` | 2 | `crates/vector-surface-brush/src/calligraphic.rs:107` | - |  |
| `range` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:300` | - |  |
| `impl Reference::range` | 1 | `crates/chronicle-tabular-engine/src/formula/ast.rs:50` | - |  |
| `impl Params::range` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:155` | - |  |
| `impl Rng::range` | 3 | `crates/vector-surface-brush/src/track.rs:200` | - | Uniform in [a, b]. |
| `range_assertion` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:443` | - |  |
| `range_ops` | 0 | `crates/audio-timebase/src/rate.rs:150` | - |  |
| `trait Host::range_values` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:45` | - |  |
| `trait VecExt::ranged` | 2 | `crates/vector-hybrid-renderer/src/util.rs:147` | - |  |
| `impl VecExt<T> for Vec<T>::ranged` | 2 | `crates/vector-hybrid-renderer/src/util.rs:157` | - |  |
| `ranged_slices` | 0 | `crates/vector-hybrid-renderer/src/util.rs:211` | - |  |
| `ranges_clear` | 0 | `crates/vector-hybrid-renderer/src/util.rs:236` | - |  |
| `rank` | 3 | `crates/locus-parametric-solver/src/solver/lm.rs:107` | numerics | Numerical rank of a row-major `rows x cols` matrix (rows normalised first). |
| `impl AssetHost<'_>::raster` | 3 | `crates/asset-production-vibe/src/raster.rs:8` | - |  |
| `impl SurfaceAttributes::rasterize_triangle` | 11 | `crates/surface-pbr-studio/src/bake.rs:97` | 3d, 2d |  |
| `rasterize_uv_chart` | 4 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:251` | 2d | Rasterize UV triangles into a still map so a non-trivial island pixel is observable. |
| `rate` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:199` | - |  |
| `impl Resource::rate_on_day` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:389` | - |  |
| `impl Resampler::rates` | 1 | `crates/audio-opus-codec/src/silk/resampler.rs:127` | - |  |
| `impl Body::ratio` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:149` | - |  |
| `impl TimecodeRate::rational` | 1 | `crates/audio-timebase/src/timecode.rs:50` | - | Exact rational frame rate (numerator, denominator). |
| `impl Set::rational` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:93` | - |  |
| `rational_container_time_round_trips_to_its_timebase` | 0 | `crates/timeline-editing/src/timebase.rs:246` | - |  |
| `rational_of` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:185` | - |  |
| `rational_rates_and_audio_units_have_exact_boundaries` | 0 | `crates/timeline-editing/src/timebase.rs:230` | - |  |
| `ray_intersect_aabb` | 3 | `crates/asset-production-vibe/src/kinetics_sim/motor_xpbd_space.rs:456` | computational geometry, 3d |  |
| `ray_intersects_triangle` | 5 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:129` | computational geometry, 3d | Möller-Trumbore ray-triangle intersection test returning true if ray (origin, dir) intersects. |
| `ray_triangle` | 4 | `crates/surface-pbr-studio/src/math.rs:349` | computational geometry, 3d | Möller–Trumbore ray-triangle intersection (hits both front and back faces). |
| `impl AssetHost<'_>::raymarch_vox0_dda` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/vox0_metamer.rs:25` | - | Raymarches a 3D `.10d` `Vox0DagNode` / `.emf` volumetric brick lattice using exact |
| `impl BitWriter::rbsp_trailing` | 1 | `crates/bitstream-codec/src/lib.rs:229` | - | `rbsp_trailing_bits()`: a one bit then zeros to alignment. |
| `rdp` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:144` | - |  |
| `rdp_points` | 2 | `crates/vector-radiance-trace/src/fit.rs:360` | - |  |
| `rdp_slice` | 5 | `crates/vector-radiance-trace/src/fit.rs:30` | - | Douglas-Peucker on `pts[a..=b]`, marking kept indices. |
| `impl Reader<'a>::read` | 2 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:63` | - |  |
| `impl Transform::read` | 3 | `crates/vector-compute-shaders/src/cpu/util.rs:170` | - |  |
| `impl Movie::read` | 2 | `crates/video-h264-preview/src/movie.rs:402` | - |  |
| `impl Reader<'a>::read` | 2 | `crates/video-prores-codec/src/bits.rs:81` | - |  |
| `read_affinity_header` | 1 | `crates/surface-layered-codecs/src/affinity.rs:72` | - | Reads and validates the 72-byte Affinity container header. |
| `read_aot` | 1 | `crates/audio-aac-codec/src/config.rs:173` | - |  |
| `impl BitReader<'a>::read_ascii` | 2 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:75` | - |  |
| `trait ByteSource::read_at` | 3 | `crates/media-isobmff/src/source.rs:13` | - | Fill `buf` entirely with bytes starting at `offset`; fail with `UnexpectedEof` if past the end. |
| `impl ByteSource for [u8]::read_at` | 3 | `crates/media-isobmff/src/source.rs:28` | - |  |
| `impl ByteSource for Vec<u8>::read_at` | 3 | `crates/media-isobmff/src/source.rs:41` | - |  |
| `impl ByteSource for &T::read_at` | 3 | `crates/media-isobmff/src/source.rs:50` | - |  |
| `impl ByteSource for Arc<T>::read_at` | 3 | `crates/media-isobmff/src/source.rs:59` | - |  |
| `impl ByteSource for Box<T>::read_at` | 3 | `crates/media-isobmff/src/source.rs:68` | - |  |
| `impl ByteSource for std::fs::File::read_at` | 3 | `crates/media-isobmff/src/source.rs:79` | - |  |
| `impl ByteSource for std::fs::File::read_at` | 3 | `crates/media-isobmff/src/source.rs:83` | - |  |
| `impl ByteSource for std::fs::File::read_at` | 3 | `crates/media-isobmff/src/source.rs:96` | - |  |
| `trait ByteSource::read_at` | 3 | `crates/media-ogg/src/lib.rs:26` | - |  |
| `impl ByteSource for [u8]::read_at` | 3 | `crates/media-ogg/src/lib.rs:36` | - |  |
| `impl ByteSource for Vec<u8>::read_at` | 3 | `crates/media-ogg/src/lib.rs:48` | - |  |
| `impl ByteSource for &T::read_at` | 3 | `crates/media-ogg/src/lib.rs:57` | - |  |
| `impl BitReader<'a>::read_bit` | 1 | `crates/bitstream-codec/src/lib.rs:105` | - |  |
| `impl Reader<'a>::read_bit` | 1 | `crates/video-prores-codec/src/bits.rs:93` | - |  |
| `impl BitReader<'a>::read_bits` | 2 | `crates/bitstream-codec/src/lib.rs:86` | - |  |
| `impl BitReader<'a>::read_bits_u64` | 2 | `crates/bitstream-codec/src/lib.rs:95` | - |  |
| `read_blur_rect` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:740` | 2d |  |
| `impl RangeDecoder<'a>::read_byte` | 1 | `crates/audio-opus-codec/src/range.rs:77` | - |  |
| `impl RangeDecoder<'a>::read_byte_from_end` | 1 | `crates/audio-opus-codec/src/range.rs:88` | - |  |
| `read_color` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:735` | - |  |
| `read_color` | 2 | `crates/vector-compute-shaders/src/cpu/fine.rs:13` | - |  |
| `impl Reader<'a>::read_cw` | 2 | `crates/video-prores-codec/src/bits.rs:104` | - | Decode one codeword; `None` for codewords longer than 56 bits (never produced by valid streams). |
| `read_draw_tag_from_scene` | 1 | `crates/vector-compute-shaders/shader/shared/util.wgsl:15` | - |  |
| `read_draw_tag_from_scene` | 3 | `crates/vector-compute-shaders/src/cpu/util.rs:206` | - | Read draw tag, guarded by number of draw objects. |
| `read_end_clip` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:829` | - |  |
| `read_f32_point` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:633` | - |  |
| `read_f32_point` | 2 | `crates/vector-compute-shaders/src/cpu/flatten.rs:507` | 2d |  |
| `impl WkbReader<'a>::read_f64` | 2 | `crates/locus-geospatial-codecs/src/wkb.rs:43` | - |  |
| `read_fill` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:728` | - |  |
| `read_fill` | 2 | `crates/vector-compute-shaders/src/cpu/fine.rs:23` | - |  |
| `impl BitReader<'a>::read_flag` | 1 | `crates/bitstream-codec/src/lib.rs:108` | - |  |
| `impl WkbReader<'a>::read_geometry` | 2 | `crates/locus-geospatial-codecs/src/wkb.rs:75` | - |  |
| `read_i16` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:15` | - |  |
| `read_i16_point` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:639` | - |  |
| `read_i16_point` | 2 | `crates/vector-compute-shaders/src/cpu/flatten.rs:513` | 2d |  |
| `impl BitReader<'a>::read_i32` | 2 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:65` | - |  |
| `read_i32` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:25` | - |  |
| `read_id` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:55` | - | Decode an element ID (marker bits retained). |
| `read_image` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:804` | - |  |
| `read_lin_grad` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:758` | - |  |
| `read_neighboring_segment` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:810` | - |  |
| `read_neighboring_segment` | 4 | `crates/vector-compute-shaders/src/cpu/flatten.rs:637` | - |  |
| `impl OggFile::read_packet` | 4 | `crates/media-ogg/src/lib.rs:364` | - | The bytes of packet `i` of stream `s`. |
| `read_parts` | 2 | `crates/media-ogg/src/lib.rs:348` | - |  |
| `read_path_segment` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:710` | - |  |
| `read_path_segment` | 3 | `crates/vector-compute-shaders/src/cpu/flatten.rs:581` | - |  |
| `read_png` | 1 | `crates/media-test-support/src/golden.rs:79` | - |  |
| `impl WkbReader<'a>::read_point2d` | 4 | `crates/locus-geospatial-codecs/src/wkb.rs:53` | - |  |
| `impl WkbReader<'a>::read_point3d` | 3 | `crates/locus-geospatial-codecs/src/wkb.rs:65` | - |  |
| `read_rad_grad` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:769` | - |  |
| `read_range` | 3 | `crates/media-isobmff/src/demux.rs:245` | - |  |
| `read_rate` | 1 | `crates/audio-aac-codec/src/config.rs:178` | - |  |
| `read_rational_time` | 1 | `crates/chronicle-timeline-interchange/src/interchange/otio.rs:117` | - |  |
| `impl Mp4File::read_sample` | 4 | `crates/media-isobmff/src/demux.rs:228` | - | Read the bytes of sample `index` of track `track` (index into [`tracks`](Self::tracks)). |
| `impl BitReader<'a>::read_se` | 1 | `crates/bitstream-codec/src/lib.rs:138` | - | Signed Exp-Golomb `se(v)`. |
| `read_size` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:49` | - | Decode an element size. |
| `read_snippets_file` | 2 | `crates/discourse-academic-tex/src/review/snippets.rs:110` | - | Read a snippet file bounded; missing or invalid returns empty vec. |
| `read_svint` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:69` | - | Decode a signed VINT as used by EBML lacing (value − (2^(7·len−1) − 1)). |
| `read_sweep_grad` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:788` | - |  |
| `impl BitReader<'a>::read_te` | 2 | `crates/bitstream-codec/src/lib.rs:144` | - | Truncated Exp-Golomb `te(v)` with range `max`. |
| `impl SliceDecoder<'a>::read_te` | 2 | `crates/video-h264-codec/src/slicedec.rs:1229` | - |  |
| `read_transform` | 2 | `crates/vector-compute-shaders/shader/draw_leaf.wgsl:37` | - |  |
| `read_transform` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:655` | - |  |
| `read_u16` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:10` | - |  |
| `impl BitReader<'a>::read_u32` | 2 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:46` | - |  |
| `impl WkbReader<'a>::read_u32` | 2 | `crates/locus-geospatial-codecs/src/wkb.rs:33` | - |  |
| `read_u32` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:20` | - |  |
| `read_u64` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:30` | - |  |
| `impl WkbReader<'a>::read_u8` | 1 | `crates/locus-geospatial-codecs/src/wkb.rs:23` | - |  |
| `impl BitReader<'a>::read_ue` | 1 | `crates/bitstream-codec/src/lib.rs:121` | - | Unsigned Exp-Golomb `ue(v)`. |
| `read_ue_plus` | 2 | `crates/video-h264-codec/src/params.rs:7` | - |  |
| `read_uv01_atlas` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:833` | - |  |
| `read_varint` | 2 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:164` | - | Decodes varint from byte stream. |
| `read_vint` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:29` | - | Decode an unsigned VINT. Returns `(value, length, all_ones)`. |
| `read_zip_entries` | 1 | `crates/surface-layered-codecs/src/xcf_ora.rs:361` | - | Minimal pure-Rust ZIP local file record parser for stored (uncompressed) or raw members. |
| `impl Unit::readout` | 2 | `crates/vector-locus-geometry/src/units.rs:144` | - |  |
| `reads_bits` | 0 | `crates/bitstream-codec/src/lib.rs:347` | - |  |
| `impl Queue::ready` | 1 | `crates/pcm-device-output/src/queue.rs:122` | - |  |
| `impl SigMeta::real` | 1 | `crates/kinetics-block-simulation/src/signal.rs:20` | - |  |
| `impl Calc::rebuild` | 2 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:86` | - |  |
| `rec` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:9` | - |  |
| `impl Fft::rec` | 7 | `crates/audio-opus-codec/src/celt/mdct.rs:57` | - |  |
| `rec2020` | 0 | `crates/color-transforms/src/icc/builtin.rs:99` | - |  |
| `impl Calc::recalc_all` | 2 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:100` | - |  |
| `impl AssetHost<'_>::recalculate_tabular_fiduciary_archive` | 2 | `crates/asset-production-vibe/src/discourse_chronicle_fiduciary.rs:178` | - | 143: `hypermedia.chronicle.recalculate_tabular_fiduciary_archive` |
| `receipt` | 1 | `crates/pcm-device-output/src/queue_tests.rs:16` | - |  |
| `received` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:573` | - |  |
| `recognition_posting_lifecycle` | 0 | `crates/chronicle-fiduciary-ledger/src/revrec.rs:166` | - |  |
| `recognize` | 1 | `crates/vector-locus-geometry/src/recognize.rs:231` | - | Recognise a freehand stroke. |
| `recognize_shape` | 1 | `crates/vector-locus-geometry/src/recognize.rs:303` | - | Alias for [`recognize`]. |
| `impl Pixels<'win>::reconfigure_surface` | 1 | `crates/pixel-buffer-output/src/lib.rs:619` | - | Reconfigure the surface. |
| `impl SliceDecoder<'a>::reconstruct_inter_residual` | 1 | `crates/video-h264-codec/src/slicedec.rs:1045` | - | Add the residual of an inter MB (prediction already written). |
| `impl SliceDecoder<'a>::reconstruct_intra` | 1 | `crates/video-h264-codec/src/slicedec.rs:995` | - | Reconstruct an intra MB (prediction + residual) after its syntax has been parsed. |
| `record` | 1 | `crates/asset-production-vibe/src/lib.rs:481` | - |  |
| `impl GeodesicRuler::record` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:209` | - |  |
| `record` | 2 | `crates/profile-authoring/src/frames.rs:25` | 3d |  |
| `impl CatalogJournal::record_action` | 2 | `crates/radiance-camera-raw/src/metadata.rs:181` | - |  |
| `impl ChangeDag::record_command` | 5 | `crates/kinetics-schedule-cpm/src/change_dag.rs:192` | - |  |
| `impl Render::record_fine` | 3 | `crates/vector-compute-renderer/src/render.rs:560` | - | Run fine rasterization assuming the coarse phase succeeded. |
| `impl Scene::record_generated_path` | 3 | `crates/vector-hybrid-renderer/src/scene.rs:491` | - |  |
| `recorded_rect_bbox_rounds_fractional_outward` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:918` | 2d |  |
| `impl VulkanBackend::recreate_swapchain` | 1 | `crates/vector-skia-render-adapter/src/vulkan.rs:195` | - |  |
| `impl DrawCase<T>::rect` | 5 | `crates/vector-hybrid-renderer/src/draw.rs:614` | 2d |  |
| `rect` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:647` | 2d |  |
| `impl Part::rect` | 2 | `crates/video-h264-codec/src/mbtypes.rs:28` | 2d | (x, y, w, h) of partition `i` in 4x4-block units. |
| `impl SubPart::rect` | 2 | `crates/video-h264-codec/src/mbtypes.rs:57` | 2d | (x, y, w, h) of sub-partition `j` relative to the 8x8 block, in 4x4 units. |
| `rect_bbox_is_the_corners_box` | 0 | `crates/vector-locus-geometry/src/projective.rs:168` | 2d |  |
| `rect_from` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1262` | 2d |  |
| `rect_from_points` | 2 | `crates/vector-locus-geometry/src/lib.rs:84` | 2d | Rect from two corner points in any order. |
| `rect_intersection` | 0 | `crates/vector-locus-geometry/src/hit.rs:113` | computational geometry, 2d |  |
| `rect_is_4_corners_clockwise` | 0 | `crates/vector-locus-geometry/src/shapes.rs:259` | 2d |  |
| `rect_normalizes_negative` | 0 | `crates/vector-locus-geometry/src/shapes.rs:267` | 2d |  |
| `impl MorphologyKernel::rectangle` | 2 | `crates/raster-selection-morphology/src/grayscale.rs:124` | - | Flat rectangular structuring element. |
| `rectangle` | 1 | `crates/vector-locus-geometry/src/shapes.rs:14` | 2d | Rectangle with 4 corner anchors, clockwise from the top-left. |
| `rectangle_rotation` | 1 | `crates/vector-locus-geometry/src/recognize.rs:186` | - |  |
| `rectangle_with_corners` | 3 | `crates/vector-locus-geometry/src/shapes.rs:62` | 2d | Rectangle whose corners (top-left, top-right, bottom-right, bottom-left, y down) each have |
| `rectangular_and_partial_bands_match_the_previous_transform` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:30` | - |  |
| `rectangular_clip_keeps_direct_rect_draw_path` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:928` | 2d |  |
| `rectangular_grid` | 4 | `crates/vector-locus-geometry/src/shapes.rs:201` | 2d | Rectangular grid as separate open lines (plus an optional frame rectangle). |
| `rects_overlap` | 8 | `crates/asset-production-vibe/src/graphics_pack.rs:83` | - |  |
| `red_angle` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:494` | numerics |  |
| `impl GeodesicRuler::redo` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:242` | - |  |
| `impl CatalogJournal::redo` | 1 | `crates/radiance-camera-raw/src/metadata.rs:193` | - |  |
| `impl CowTerrainTileGrid::redo` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:239` | - |  |
| `impl CowTerrainTileGrid::redo` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:283` | - |  |
| `impl CowTerrainTileGrid::redo` | 1 | `crates/terrain-brushes/src/lib.rs:424` | - |  |
| `impl CowTerrainTileGrid::redo_history` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:31` | - |  |
| `impl CowTerrainTileGrid::redo_history` | 1 | `crates/terrain-brushes/src/lib.rs:212` | - |  |
| `reduce_node` | 2 | `crates/raster-spatial-operators/src/octree.rs:141` | graph |  |
| `reduce_tag` | 1 | `crates/vector-compute-shaders/shader/shared/pathtag.wgsl:58` | - |  |
| `ref_ids8` | 2 | `crates/video-h264-codec/src/deblock.rs:16` | - | Referenced picture ids per 8x8 block and list (u32::MAX = list unused). |
| `reference_a1` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:58` | - |  |
| `reference_add` | 4 | `crates/video-h264-codec/src/transform.rs:188` | - | The straightforward per-column formulation (8.5.12.2 / 8.5.13.2) the vectorised |
| `reference_idct` | 1 | `crates/video-prores-codec/src/dct.rs:94` | - |  |
| `reference_lifecycle_and_frames` | 0 | `crates/multimodal-synthesis-schema/src/references.rs:192` | - |  |
| `reference_point` | 2 | `crates/vector-locus-geometry/src/lib.rs:89` | 2d | The 9 reference points of a rect (row-major from top-left). |
| `reference_points` | 0 | `crates/vector-locus-geometry/src/lib.rs:116` | - |  |
| `impl Reader<'a>::refill` | 1 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:27` | - |  |
| `impl Cabac<'a>::refill` | 1 | `crates/video-h264-codec/src/cabac.rs:113` | - | Load 32 more bits (zeros past the end of the data). |
| `impl Reader<'a>::refill` | 1 | `crates/video-prores-codec/src/bits.rs:26` | - | Ensure at least 56 valid bits are cached. |
| `refine` | 2 | `crates/vector-radiance-trace/src/quantize.rs:398` | - |  |
| `reframe` | 1 | `crates/audio-opus-codec/src/multistream.rs:154` | - | Rebuilds a regular (code 3 VBR) packet from a parsed self-delimited one. |
| `refused_edits_cover_locks_short_spans_overhangs_cut_and_overflow` | 0 | `crates/timeline-editing/src/transition.rs:321` | - |  |
| `refuses_corruption_sequence_limits_and_trailing_bytes` | 0 | `crates/media-ogg/src/strict_opus.rs:152` | - |  |
| `region` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:687` | - |  |
| `region` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:555` | - |  |
| `regions` | 3 | `crates/audio-aac-codec/src/tns.rs:155` | - | Filter region `[start, end)` of every TNS filter of window `w`, in spectrum lines of that window. |
| `regions` | 1 | `crates/vector-locus-pathops/src/pathops/pathfinder.rs:158` | - | Extract all distinct planar faces / regions covered by the shapes. |
| `impl Renderer::register_texture` | 2 | `crates/vector-compute-renderer/src/lib.rs:574` | economics | Register a [`wgpu::Texture`] with Vello, to allow drawing GPU-resident data. |
| `impl LayerSamplePlacement::regular` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:884` | - |  |
| `rejects_bad_values` | 0 | `crates/audio-timebase/src/tempo.rs:429` | - |  |
| `rejects_dynamic_indexing_in_entry_point` | 0 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:92` | - |  |
| `rejects_dynamic_indexing_in_helper_function` | 0 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:74` | - |  |
| `rejects_garbage` | 0 | `crates/audio-opus-codec/src/header.rs:161` | - |  |
| `rejects_helper_function_returning_struct` | 0 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:330` | - |  |
| `rejects_no_moov_and_cmov` | 0 | `crates/media-isobmff/src/demux.rs:1009` | - |  |
| `rejects_struct_as_fragment_entry_point_input` | 0 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:338` | - |  |
| `rejects_struct_local_variable_in_fragment_shader` | 0 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:324` | 3d |  |
| `rejects_unbalanced_journal` | 0 | `crates/chronicle-fiduciary-ledger/src/journal.rs:244` | - |  |
| `rejects_whole_uniform_struct_load_in_fragment_shader` | 0 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:348` | 3d |  |
| `relative_time_formatting` | 0 | `crates/multimodal-synthesis-schema/src/job_feed.rs:118` | - |  |
| `impl Cursor::release` | 3 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:114` | - | Mark the given allocation as ready to release in the given round. |
| `impl CapturedBuffers::release_buffers` | 2 | `crates/vector-compute-renderer/src/render.rs:77` | - |  |
| `impl Renderer::reload_shaders` | 2 | `crates/vector-compute-renderer/src/lib.rs:610` | - | Reload the shaders. This should only be used during `vello` development |
| `impl Cur<'a>::remaining` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:151` | - |  |
| `impl WkbReader<'a>::remaining` | 1 | `crates/locus-geospatial-codecs/src/wkb.rs:19` | - |  |
| `impl Cur<'a>::remaining` | 1 | `crates/media-isobmff/src/bytes.rs:74` | - |  |
| `impl RevenueSchedule::remaining_unrecognized` | 1 | `crates/chronicle-fiduciary-ledger/src/revrec.rs:137` | - | Returns the remaining unrecognized deferred revenue balance. |
| `impl AudioObjectPropertyListener::remove` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/property_listener.rs:49` | - | Explicitly remove the property listener. |
| `impl References::remove` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:162` | - |  |
| `impl WebGlTextureBindings::remove` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:213` | - | Remove a texture binding. |
| `impl TextureBindings::remove` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:135` | - | Remove a texture binding. |
| `impl Project::remove_dependency` | 3 | `crates/kinetics-schedule-cpm/src/model.rs:611` | - |  |
| `impl AudioObjectPropertyListener::remove_inner` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/property_listener.rs:53` | - |  |
| `impl GradientRampCache::remove_lru_entries` | 3 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:152` | - | Identify and remove the LRU cache entries. |
| `impl TempoMap::remove_meter` | 2 | `crates/audio-timebase/src/tempo.rs:190` | - | Remove a meter event (the initial event at tick 0 is pinned). |
| `impl GeodesicRuler::remove_point` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:252` | - |  |
| `remove_redundant_points` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:204` | - | Remove redundant points along straight edges and collinear joints. |
| `impl TempoMap::remove_tempo` | 2 | `crates/audio-timebase/src/tempo.rs:158` | - | Remove a tempo event (the initial event at tick 0 is pinned). |
| `remux` | 5 | `crates/media-isobmff/tests/oracle_mux.rs:39` | - |  |
| `remux_h264_aac_mp4_variants` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:126` | - |  |
| `remux_other_codecs` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:147` | - |  |
| `remux_prores_pcm24_with_timecode` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:181` | - |  |
| `rename_function` | 2 | `crates/vector-hybrid-shaders/src/minify.rs:149` | - | Renames a function declaration, its arguments, locals, and named expressions. |
| `rename_module` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:97` | - | Assigns short names throughout a module and returns the global-name reverse map. |
| `rename_optional` | 2 | `crates/vector-hybrid-shaders/src/minify.rs:166` | - | Replaces an optional Naga IR declaration name with the next short name, leaving unnamed |
| `impl AssetHost<'_>::render` | 2 | `crates/asset-production-vibe/src/raster.rs:252` | - |  |
| `impl Queue::render` | 2 | `crates/pcm-device-output/src/queue.rs:127` | - | All slot storage is retained: no allocation/deallocation or source lookup |
| `impl Pixels<'win>::render` | 1 | `crates/pixel-buffer-output/src/lib.rs:498` | 2d | Draw this pixel buffer to the configured [`SurfaceTexture`]. |
| `impl ScalingRenderer::render` | 3 | `crates/pixel-buffer-output/src/renderers.rs:251` | 2d | Draw the pixel buffer to the render target. |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::render` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:209` | - |  |
| `impl DebugRenderer::render` | 8 | `crates/vector-compute-renderer/src/debug/renderer.rs:210` | - |  |
| `impl ImageRenderer for VelloCpuImageRenderer::render` | 3 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:46` | - |  |
| `impl ImageRenderer for VelloImageRenderer::render` | 3 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:88` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::render` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:438` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::render` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:470` | - |  |
| `impl WebGlRenderer::render` | 10 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:437` | - | Render `scene` using WebGL2 |
| `impl Renderer::render` | 11 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:253` | - | Render `scene`. |
| `impl ImageRenderer for SkiaImageRenderer::render` | 3 | `crates/vector-skia-render-adapter/src/image_renderer.rs:80` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::render` | 2 | `crates/vector-skia-render-adapter/src/window_renderer.rs:166` | - |  |
| `render` | 2 | `crates/video-h264-codec/src/synth_tests.rs:353` | - | Expected cropped planes of a picture made of PCM blocks (`None` = copy from `reference`). |
| `render_anatomical_blueprint_svg` | 5 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:497` | computational geometry | Renders a vector orthographic & anatomical callout blueprint SVG from a 3D mesh. |
| `render_animated_cinema_frames` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:27` | 3d | Renders an N-frame animated 3D cinema sequence (`160x112` H.264 macroblock-aligned) combining |
| `impl PdcMixEngine::render_block` | 4 | `crates/acoustics-daw-session/src/mix_session/mod.rs:134` | - | Renders a block of audio (left, right) for the session at `start_sample` with exact latency alignment. |
| `impl AssetHost<'_>::render_chart_svg` | 2 | `crates/asset-production-vibe/src/discourse_docs/pdf_chart.rs:27` | - |  |
| `render_cinema_rgba_frame_at` | 10 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:58` | - |  |
| `render_clip` | 3 | `crates/vector-cpu-render-adapter/src/scene.rs:229` | - |  |
| `render_date` | 3 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:46` | - |  |
| `impl GlyphRunBackend<'a> for GpuGlyphRunBackend<'a>::render_decoration` | 8 | `crates/vector-hybrid-renderer/src/text.rs:294` | - |  |
| `impl Render::render_encoding_coarse` | 7 | `crates/vector-compute-renderer/src/render.rs:135` | - | Prepare a recording for the coarse rasterization phase. |
| `render_encoding_full` | 5 | `crates/vector-compute-renderer/src/render.rs:99` | - | Create a single recording with both coarse and fine render stages. |
| `render_full` | 5 | `crates/vector-compute-renderer/src/render.rs:84` | - |  |
| `render_gantt_svg` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:672` | - |  |
| `render_general` | 3 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:34` | - |  |
| `impl GpuGlyphRunBackend<'a>::render_glyphs` | 4 | `crates/vector-hybrid-renderer/src/text.rs:226` | - |  |
| `render_hero_rgba_frame` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:16` | - | Renders a 256x256 RGBA studio hero frame suitable for downstream Radiance / CDL / MaterialX morphisms. |
| `render_macro_behavior_cinema_frames` | 7 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:766` | computational geometry | Renders an N-frame animated 3D cinema sequence driven by a `BehaviorMacro` and `SubElementHooks`. |
| `render_number` | 2 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:109` | - |  |
| `render_panel` | 15 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:286` | - |  |
| `impl GpuProfilerQuery::render_pass_timestamp_writes` | 1 | `crates/gpu-command-profiler/src/profiler_query.rs:74` | - | Use the reserved query for render pass timestamp writes if any. |
| `impl WebGlRenderer::render_scene` | 8 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:601` | - | Shared render pipeline: prepares GPU resources, runs the scheduler, and |
| `impl Renderer::render_scene` | 13 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:407` | - | Shared render pipeline: prepares GPU resources, runs the scheduler against |
| `impl VelloHybridRendererOptions::render_settings` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:109` | - |  |
| `impl AssetHost<'_>::render_spectral_slice` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:328` | - | Renders a 2D spectral preview slice from an EMF-64 volume container to straight-alpha RGBA8. |
| `render_svg` | 3 | `crates/vector-hybrid-renderer/examples/render_to_file.rs:176` | - |  |
| `render_text` | 2 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:22` | - |  |
| `impl ScreenBuffer::render_text` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:175` | - |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::render_texture_format` | 2 | `crates/pixel-buffer-output/src/builder.rs:170` | 3d | Set the render texture format. |
| `impl Pixels<'win>::render_texture_format` | 1 | `crates/pixel-buffer-output/src/lib.rs:773` | - | Get the render texture format. |
| `impl WebGlRenderer::render_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:517` | - | Render a `scene` directly into an atlas layer. |
| `impl Renderer::render_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:340` | - | Render a `scene` directly into an atlas layer. |
| `impl Renderer::render_to_texture` | 6 | `crates/vector-compute-renderer/src/lib.rs:476` | - | Renders a scene to the target texture. |
| `impl Renderer::render_to_texture_async` | 7 | `crates/vector-compute-renderer/src/lib.rs:644` | - |  |
| `impl Renderer::render_to_texture_async_internal` | 6 | `crates/vector-compute-renderer/src/lib.rs:719` | - |  |
| `impl ImageRenderer for VelloCpuImageRenderer::render_to_vec` | 3 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:72` | - |  |
| `impl ImageRenderer for VelloImageRenderer::render_to_vec` | 3 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:78` | - |  |
| `impl ImageRenderer for SkiaImageRenderer::render_to_vec` | 3 | `crates/vector-skia-render-adapter/src/image_renderer.rs:48` | - |  |
| `render_turnaround_sheet_png` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:171` | - | Renders a 3-panel (`900x420`) RGBA8 turnaround sheet PNG (`Side Profile`, `3/4 Hero`, `Front`). |
| `impl AssetHost<'_>::render_ui_sheet` | 2 | `crates/asset-production-vibe/src/layout_ops/sheet_render.rs:13` | - | Renders the resolved layout into a straight-alpha RGBA8 raster artifact. |
| `impl Pixels<'win>::render_with` | 2 | `crates/pixel-buffer-output/src/lib.rs:547` | 2d | Draw this pixel buffer to the configured [`SurfaceTexture`] using a custom user-provided |
| `impl RenderContext for VelloScenePainter<'_, '_>::renderer_specific_context` | 1 | `crates/vector-gpu-render-adapter/src/scene.rs:47` | - |  |
| `impl RenderContext for VelloWindowRenderer::renderer_specific_context` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:227` | - |  |
| `impl RenderContext for VelloHybridScenePainter<'_>::renderer_specific_context` | 1 | `crates/vector-hybrid-render-adapter/src/scene.rs:189` | - |  |
| `impl RenderContext for VelloHybridWindowRenderer::renderer_specific_context` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:200` | - |  |
| `impl Cabac<'a>::renorm` | 1 | `crates/video-h264-codec/src/cabac.rs:147` | - |  |
| `impl Enc::renorm` | 1 | `crates/video-h264-codec/src/cabac.rs:230` | - |  |
| `impl Enc::renorm` | 2 | `crates/video-h264-codec/src/synth_tests.rs:233` | - |  |
| `renormalise_vector` | 2 | `crates/audio-opus-codec/src/celt/bands.rs:126` | - |  |
| `reorder_visual` | 1 | `crates/vector-glyph-shaping/src/bidi.rs:137` | - | Reorder run indices from logical reading order into visual left-to-right display order. |
| `reordered_b_skip` | 0 | `crates/video-h264-preview/src/lib.rs:583` | - |  |
| `reordered_pcm` | 4 | `crates/video-h264-preview/src/lib.rs:576` | - |  |
| `reordered_sps` | 0 | `crates/video-h264-preview/src/lib.rs:565` | - |  |
| `impl Project::repair_counters` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:541` | - |  |
| `impl ProfileDoc::replace` | 4 | `crates/profile-authoring/src/profile.rs:53` | - | Pure successor, never a second document store or undo journal. |
| `replay_failure_cancels_all_prior_samples_and_does_not_advance_id` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:181` | - |  |
| `impl Resources::replay_pending_atlas_commands` | 2 | `crates/vector-hybrid-renderer/src/text.rs:75` | - |  |
| `impl CowTerrainTileGrid::replay_stroke_journal` | 3 | `crates/terrain-brushes/adaptation/grid-impl.rs:250` | - |  |
| `impl CowTerrainTileGrid::replay_stroke_journal` | 3 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:292` | - |  |
| `impl CowTerrainTileGrid::replay_stroke_journal` | 3 | `crates/terrain-brushes/src/lib.rs:435` | - |  |
| `report` | 2 | `crates/media-test-support/src/fixtures.rs:77` | - | Print a `FIXTURE <made\|cached\|skipped> <name>` line (parsed by `cargo xtask fixtures`). |
| `req` | 1 | `crates/discourse-academic-tex/src/catalog.rs:54` | - |  |
| `req_vec` | 2 | `crates/kinetics-block-simulation/src/blocks/params.rs:35` | - |  |
| `impl DeviceOutput::request` | 2 | `crates/pcm-device-output/src/device.rs:177` | - |  |
| `request` | 3 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:254` | - |  |
| `request` | 2 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:152` | - |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::request_adapter_options` | 2 | `crates/pixel-buffer-output/src/builder.rs:73` | - | Add options for requesting a [`wgpu::Adapter`]. |
| `impl Atlases::require_scratch_texture` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:84` | - |  |
| `impl Cursor::require_scratch_texture` | 1 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:47` | - | Indicate that access to the scratch texture is required. |
| `required` | 2 | `crates/asset-production-vibe/src/lib.rs:488` | - |  |
| `required_bits` | 3 | `crates/audio-opus-codec/src/celt/rate.rs:197` | - | Bits (1/2^frac units, rounded up) needed for each K in `0..=maxk` for dimension `n`. |
| `impl FilterContext::required_filter_data_height` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:659` | - | Calculate the required height for the filter data texture. |
| `impl LayersConfig::required_intermediate_texture_size` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:166` | - |  |
| `required_intermediate_texture_size_rejects_filter_padding_overflow` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:344` | - |  |
| `required_intermediate_texture_size_rejects_oversized_layers` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:322` | - |  |
| `impl Rounds::required_layer_pages` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:53` | - | Derive the required page count for the even and odd texture groups. |
| `impl RoundBindings::required_textures` | 1 | `crates/vector-hybrid-renderer/src/target.rs:223` | - | Return the required textures for this binding. |
| `required_uniform_location` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2083` | - |  |
| `resample` | 2 | `crates/vector-locus-geometry/src/recognize.rs:96` | - | Sample by distance. |
| `resample_rgba32f` | 6 | `crates/raster-spatial-operators/src/resample.rs:252` | - | Resample an RGBA32F buffer to new target dimensions. |
| `resample_rgba8` | 6 | `crates/raster-spatial-operators/src/resample.rs:176` | - | Resample an RGBA8 buffer to new target dimensions. |
| `rescale` | 3 | `crates/media-isobmff/src/demux.rs:404` | - |  |
| `rescale` | 3 | `crates/media-isobmff/src/mux.rs:145` | - |  |
| `impl GpuProfiler::reserve_query_pair` | 1 | `crates/gpu-command-profiler/src/profiler.rs:629` | - |  |
| `impl BiquadFilter::reset` | 1 | `crates/acoustics-daw-session/src/dsp/biquad.rs:160` | - |  |
| `impl Decoder::reset` | 1 | `crates/audio-aac-codec/src/decoder.rs:114` | - | Clear the overlap state (e.g. after a seek). |
| `impl CeltDecoder::reset` | 1 | `crates/audio-opus-codec/src/celt/mod.rs:99` | - | `OPUS_RESET_STATE`. |
| `impl StreamDecoder::reset` | 1 | `crates/audio-opus-codec/src/decoder.rs:87` | - | `OPUS_RESET_STATE`. |
| `impl Decoder::reset` | 1 | `crates/audio-opus-codec/src/multistream.rs:80` | - |  |
| `impl SilkDecoder::reset` | 1 | `crates/audio-opus-codec/src/silk/mod.rs:678` | - |  |
| `impl QueryPool::reset` | 1 | `crates/gpu-command-profiler/src/profiler.rs:888` | - |  |
| `impl Scene::reset` | 1 | `crates/vector-compute-renderer/src/scene.rs:59` | - | Removes all content from the scene. |
| `impl ImageRenderer for VelloCpuImageRenderer::reset` | 1 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:42` | - |  |
| `impl PaintScene for VelloCpuScenePainter::reset` | 1 | `crates/vector-cpu-render-adapter/src/scene.rs:69` | - |  |
| `impl ImageRenderer for VelloImageRenderer::reset` | 1 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:74` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::reset` | 1 | `crates/vector-gpu-render-adapter/src/scene.rs:65` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::reset` | 1 | `crates/vector-hybrid-render-adapter/src/scene.rs:195` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::reset` | 1 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:107` | - |  |
| `impl Scene::reset` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:857` | - | Reset scene to default values. |
| `impl ImageRenderer for SkiaImageRenderer::reset` | 1 | `crates/vector-skia-render-adapter/src/image_renderer.rs:46` | - |  |
| `impl PaintScene for SkiaScenePainter<'_>::reset` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:424` | - |  |
| `impl PicState::reset` | 2 | `crates/video-h264-codec/src/slicedec.rs:88` | - | Reuse the buffers of a finished picture of the same size for a new frame. |
| `impl GpuProfiler::reset_and_cache_unused_query_pools` | 2 | `crates/gpu-command-profiler/src/profiler.rs:570` | - |  |
| `impl Scene::reset_and_resize` | 3 | `crates/vector-hybrid-renderer/src/scene.rs:849` | - | Reset the scene and update its size. |
| `reset_and_resize_updates_scene_size` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:953` | - |  |
| `impl Scene::reset_filter_effect` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:826` | - | Reset the current filter effect. |
| `impl SkiaScenePainter<'_>::reset_paint` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:89` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::reset_paint_transform` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:354` | - |  |
| `impl Scene::reset_paint_transform` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:801` | - | Reset the current paint transform. |
| `reset_restores_default_aliasing_threshold` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:943` | - |  |
| `impl Scene::reset_tint` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:783` | - | Clear the tint, so subsequent image paints are drawn without tinting. |
| `impl Scene::reset_transform` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:816` | - | Reset the transform to identity. |
| `residual_block` | 6 | `crates/video-h264-codec/src/cavlc.rs:95` | - | Parse residual_block_cavlc(). Coefficients are written to `coeff[start_idx..=end_idx]` in scan order |
| `impl SliceDecoder<'_>::residual_block_cabac` | 6 | `crates/video-h264-codec/src/cabac_mb.rs:381` | - | residual_block_cabac for ctxBlockCat `cat` (0..=5). Coefficients are written to `coeff` |
| `impl SliceDecoder<'_>::residual_cabac` | 3 | `crates/video-h264-codec/src/cabac_mb.rs:449` | - |  |
| `impl SliceDecoder<'a>::residual_cavlc` | 3 | `crates/video-h264-codec/src/slicedec.rs:1391` | - |  |
| `impl BufferRenderer::resize` | 3 | `crates/gpu-device-context/src/buffer_renderer.rs:79` | - |  |
| `impl SurfaceRenderer<'s>::resize` | 3 | `crates/gpu-device-context/src/surface_renderer.rs:217` | - | Resizes the surface to the new dimensions. |
| `impl ScalingRenderer::resize` | 4 | `crates/pixel-buffer-output/src/renderers.rs:295` | - |  |
| `impl ImageRenderer for VelloCpuImageRenderer::resize` | 3 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:38` | - |  |
| `impl ImageRenderer for VelloImageRenderer::resize` | 3 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:70` | - |  |
| `impl ImageRenderer for SkiaImageRenderer::resize` | 3 | `crates/vector-skia-render-adapter/src/image_renderer.rs:37` | - |  |
| `impl Pixels<'win>::resize_buffer` | 3 | `crates/pixel-buffer-output/src/lib.rs:363` | 2d | Resize the pixel buffer and zero its contents. |
| `impl Pixels<'win>::resize_surface` | 3 | `crates/pixel-buffer-output/src/lib.rs:408` | 2d | Resize the surface upon which the pixel buffer texture is rendered. |
| `impl RenderContext::resize_surface` | 4 | `crates/vector-compute-renderer/src/util.rs:119` | - | Resizes the surface to the new dimensions. |
| `resolution_is_hd` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:570` | - |  |
| `resolution_label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:555` | - |  |
| `resolve` | 2 | `crates/discourse-academic-tex/src/review/suggest.rs:56` | - | Resolve anchor against source text. |
| `resolve` | 2 | `crates/discourse-academic-tex/src/texparse/graph.rs:116` | - |  |
| `impl ColorRef::resolve` | 2 | `crates/layout-presentation-deck/src/model/types.rs:179` | - |  |
| `impl ModelInfo::resolve` | 3 | `crates/multimodal-synthesis-schema/src/catalog.rs:153` | - |  |
| `resolve_bidi_runs` | 2 | `crates/vector-glyph-shaping/src/bidi.rs:70` | - | Resolve paragraph direction and decompose text into directional runs. |
| `resolve_color` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:512` | - |  |
| `resolve_corpus` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:24` | - |  |
| `impl ModelInfo::resolve_duration` | 3 | `crates/multimodal-synthesis-schema/src/catalog.rs:183` | - |  |
| `impl AssetHost<'_>::resolve_emf_bytes` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:636` | - | Resolves container bytes from retained inputs or workspace encoded resources. |
| `resolve_fill` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:127` | - | Effective fill of a shape after placeholder resolution. |
| `resolve_input` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:607` | - |  |
| `resolve_line` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:146` | - | Effective line/border of a shape after placeholder resolution. |
| `resolve_morphism` | 1 | `crates/asset-production-vibe/src/ontology/mod.rs:240` | - | Resolves any canonical categorical ID or historical `hypermedia.*`/`synth.*` alias |
| `trait Block::resolve_out` | 3 | `crates/kinetics-block-simulation/src/block.rs:58` | - |  |
| `impl Block for Integrator::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:32` | - |  |
| `impl Block for Derivative::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:98` | - |  |
| `impl Block for PidController::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:162` | - |  |
| `impl Block for TransferFcn::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:238` | - |  |
| `impl Block for TransportDelay::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:329` | - |  |
| `impl Block for UnitDelay::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:35` | - |  |
| `impl Block for ZeroOrderHold::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:92` | - |  |
| `impl Block for Gain::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:32` | - |  |
| `impl Block for Sum::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:86` | - |  |
| `impl Block for Product::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:148` | - |  |
| `impl Block for Saturation::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:202` | - |  |
| `impl Block for DeadZone::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:248` | - |  |
| `impl Block for Switch::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:298` | - |  |
| `impl Block for Relational::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:348` | - |  |
| `impl Block for LogicalOperator::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:408` | - |  |
| `impl Block for Abs::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/math.rs:467` | - |  |
| `impl Block for Mux::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/routing.rs:30` | - |  |
| `impl Block for Demux::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/routing.rs:75` | - |  |
| `impl Block for Scope::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:48` | - |  |
| `impl Block for ToWorkspace::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sinks.rs:95` | - |  |
| `impl Block for Constant::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sources.rs:29` | - |  |
| `impl Block for Step::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sources.rs:72` | - |  |
| `impl Block for Sine::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sources.rs:118` | - |  |
| `impl Block for Ramp::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sources.rs:162` | - |  |
| `impl Block for Clock::resolve_out` | 3 | `crates/kinetics-block-simulation/src/blocks/sources.rs:202` | - |  |
| `impl GpuProfiler::resolve_queries` | 2 | `crates/gpu-command-profiler/src/profiler.rs:345` | - | Puts query resolve commands in the encoder for all unresolved, pending queries of the active profiler frame. |
| `resolve_rules` | 5 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:120` | - | Resolves the `rules` argument into a validated `Q42LOGIC` container, appending `derived` |
| `impl LayerSamplePlacement::resolve_sample_region` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:898` | - |  |
| `resolve_species_spec` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:50` | - | Returns the species spec from `spec` (or its legacy alias `scaffold`) and where it came from. |
| `resolve_tracks` | 3 | `crates/layout-box-autolayout/src/grid.rs:127` | - | Resolves track sizes for a list of track specs, given content sizes and available space. |
| `resolve_xfrm` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:108` | - | Effective bounding transform of a shape after placeholder resolution. |
| `resource_levelling` | 0 | `crates/kinetics-schedule-cpm/src/schedule.rs:858` | - |  |
| `impl DeviceLimits::resource_texture_dimension_2d` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:71` | - |  |
| `resource_texture_dimension_is_capped_at_4k` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:254` | - |  |
| `resource_texture_size_must_fit_dimension_limit` | 0 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3569` | - |  |
| `impl Parser<'_>::rest` | 1 | `crates/manifold-bim-csg/src/step.rs:126` | - |  |
| `impl Cur<'a>::rest` | 1 | `crates/media-isobmff/src/bytes.rs:77` | - |  |
| `impl GradientRampCache::restore_luts` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:131` | - | Restore the luts. The restored luts should have the same logical content as the original. |
| `impl Scene::restore_state` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:893` | - | Restore rendering state. |
| `impl glifo::GlyphRenderer for Scene::restore_state` | 2 | `crates/vector-hybrid-renderer/src/text.rs:324` | - |  |
| `results_do_not_depend_on_the_worker_count` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:102` | - |  |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::resume` | 5 | `crates/pixel-window-render-adapter/src/lib.rs:150` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::resume` | 5 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:258` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::resume` | 5 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:263` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::resume` | 5 | `crates/vector-skia-render-adapter/src/window_renderer.rs:115` | - |  |
| `impl ApplicationHandler<()> for State::resumed` | 2 | `crates/gpu-command-profiler/examples/demo.rs:181` | - |  |
| `impl Anchor::retract` | 1 | `crates/vector-locus-geometry/src/path.rs:165` | - | Remove both handles (convert to a sharp corner). |
| `reuse_vec` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:246` | - |  |
| `reversals` | 2 | `crates/vector-locus-geometry/src/recognize.rs:220` | - |  |
| `impl OpaqueDraw::reverse` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:85` | - | Reverse opaque strips for front-to-back rendering. |
| `impl SubPath::reverse` | 1 | `crates/vector-locus-geometry/src/path.rs:235` | - |  |
| `impl PathData::reverse` | 1 | `crates/vector-locus-geometry/src/path.rs:491` | - |  |
| `reverse_flips_area_sign` | 0 | `crates/vector-locus-geometry/src/path.rs:592` | - |  |
| `impl JournalLedger::reverse_journal` | 3 | `crates/chronicle-fiduciary-ledger/src/journal.rs:156` | - | Reverses a previously posted journal entry with inverted amounts. |
| `impl Segment::reversed` | 1 | `crates/drafting-geometry/src/curve.rs:380` | - |  |
| `impl Anchor::reversed` | 1 | `crates/vector-locus-geometry/src/path.rs:161` | - | Swap handles (used when reversing path direction). |
| `impl AssetHost<'_>::review_gallery` | 2 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:752` | - | CSP-locked gallery. Origin image/audio path is preserved; 10d/m64/emf/q42 |
| `review_gallery_e64_magic_fallback_presents_m64_without_kind_sibling` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:1193` | - |  |
| `review_gallery_presents_10d_m64_emf_q42_maps` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:1126` | - |  |
| `revolve` | 3 | `crates/vector-locus-pathops/src/three_d/revolve.rs:150` | 2d | Evaluate a profile in document space. All subpaths share one axis from `bounds`. |
| `rgb` | 4 | `crates/color-transforms/src/icc/builtin.rs:141` | - |  |
| `impl Matrix::rgb` | 4 | `crates/video-h264-preview/src/convert.rs:40` | semantics | Convert one 8-bit sample triple. |
| `impl FrameHeader::rgb_planes` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:56` | - |  |
| `rgb_to_hsl` | 3 | `crates/color-transforms/src/lib.rs:260` | - | RGB (0..1) â†’ HSL (h in 0..1). |
| `rgb_to_hsl` | 3 | `crates/radiance-color-grading/src/matrix.rs:356` | - | RGB (0..1) -> HSL (h in 0..1). |
| `rgb_to_xyz_matrix` | 4 | `crates/color-transforms/src/icc/math.rs:105` | numerics | RGB → XYZ matrix (relative to `white`, Y = 1) for primaries given as chromaticities. |
| `rgb_to_ycbcr` | 4 | `crates/color-transforms/src/lib.rs:156` | numerics | Convert non-linear R'G'B' to normalised Y'CbCr. |
| `rgb_to_ycbcr` | 4 | `crates/radiance-color-grading/src/matrix.rs:151` | numerics | Convert non-linear R'G'B' (nominal 0..1) to normalized Y'CbCr. |
| `rgb_view` | 1 | `crates/asset-production-vibe/src/raster.rs:339` | - |  |
| `impl AssetHost<'_>::rgba` | 2 | `crates/asset-production-vibe/src/graphics.rs:84` | - |  |
| `rgba709` | 2 | `crates/video-h264-preview/src/lib.rs:451` | - |  |
| `rgba8` | 3 | `crates/radiance-color-grading/src/scopes/tests.rs:6` | - |  |
| `rgba_css` | 1 | `crates/asset-production-vibe/src/layout_ops/mod.rs:65` | - |  |
| `rgba_to_prores` | 1 | `crates/asset-production-vibe/src/chronicle_ops.rs:429` | - |  |
| `ri` | 4 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:87` | - |  |
| `ri_frame_size` | 4 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:124` | - |  |
| `impl AssetHost<'_>::rig_biomechanical_archetype` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rig_ontology_continuum_frustum.rs:28` | - | Constructs and steps the 18-joint PGA `Motor` fauna skeletal rig (`FaunaSkeletalRig10d`), |
| `right` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:47` | - |  |
| `impl Rect::right` | 1 | `crates/layout-box-autolayout/src/types.rs:113` | - |  |
| `impl Rect::right` | 1 | `crates/layout-paged-typesetter/src/pagination/types.rs:30` | - |  |
| `right_angles` | 2 | `crates/vector-locus-geometry/src/recognize.rs:156` | - |  |
| `right_of` | 1 | `crates/vector-radiance-trace/src/contour.rs:32` | - |  |
| `impl Grid::ring` | 2 | `crates/vector-radiance-trace/src/centerline.rs:152` | - |  |
| `ring_has_outer_and_hole` | 0 | `crates/vector-radiance-trace/src/tests.rs:55` | - |  |
| `rings_to_path` | 2 | `crates/vector-locus-pathops/src/pathops/boolean.rs:423` | computational geometry | Convert clipped polygon rings into smooth cubic Bézier subpaths. |
| `ris_entries` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:184` | - |  |
| `ris_kind` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:171` | - |  |
| `impl RangeDecoder<'a>::rng` | 1 | `crates/audio-opus-codec/src/range.rs:72` | - |  |
| `root_blend_budget` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:407` | - |  |
| `root_blend_case` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:104` | - |  |
| `root_blend_resources` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:382` | - |  |
| `impl Round::root_draw_mut` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:233` | - |  |
| `impl Round::root_draw_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/round.rs:192` | - |  |
| `root_draws` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:201` | - |  |
| `root_is_the_workspace` | 0 | `crates/media-test-support/src/fixtures.rs:107` | - |  |
| `root_release` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:393` | - |  |
| `root_without_depth_preserves_interleaved_painter_order` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:255` | - |  |
| `rot` | 5 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:243` | - |  |
| `rot` | 2 | `crates/vector-surface-brush/src/calligraphic.rs:164` | 2d |  |
| `impl Mat3::rotate` | 1 | `crates/drafting-geometry/src/mat.rs:31` | - |  |
| `impl Vec2::rotate` | 2 | `crates/drafting-geometry/src/vec.rs:58` | 2d |  |
| `impl Extrude::rotate` | 2 | `crates/vector-locus-pathops/src/three_d/bevel.rs:49` | - |  |
| `impl Revolve::rotate` | 2 | `crates/vector-locus-pathops/src/three_d/revolve.rs:110` | - |  |
| `impl Mat3::rotate_about` | 2 | `crates/drafting-geometry/src/mat.rs:39` | 2d |  |
| `impl Vec2::rotate_about` | 3 | `crates/drafting-geometry/src/vec.rs:62` | 2d |  |
| `impl Mat3::rotation` | 1 | `crates/drafting-geometry/src/mat.rs:95` | - | Rotation angle of the transformed X axis. |
| `rotr` | 2 | `crates/asset-production-vibe/src/operations_catalog.rs:1061` | - |  |
| `rough_rectangle_and_square` | 0 | `crates/vector-locus-geometry/src/recognize.rs:348` | - |  |
| `roughen` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:89` | 2d | Roughen: resamples segments and jitters points. |
| `round` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:26` | - |  |
| `round2` | 1 | `crates/radiance-color-grading/src/scopes/summary.rs:45` | - |  |
| `trait ScheduleTarget::round_bindings` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1061` | - |  |
| `impl ScheduleTarget for RootTarget::round_bindings` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1073` | - |  |
| `impl ScheduleTarget for LayerTextureRegion::round_bindings` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1087` | - |  |
| `impl FilterTextureRegions::round_bindings` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:500` | - |  |
| `round_down` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:674` | - |  |
| `impl Rounds::round_mut` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:69` | - |  |
| `round_order` | 0 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:343` | - |  |
| `round_order_case` | 0 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:306` | - |  |
| `round_trip_random_bins` | 0 | `crates/video-h264-codec/src/cabac.rs:304` | - |  |
| `round_up` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:678` | - |  |
| `rounded_profile_edit_changes_real_geometry_and_rejects_stale_revision` | 0 | `crates/profile-authoring/src/tests.rs:150` | - |  |
| `rounded_rect` | 4 | `crates/profile-authoring/src/lib.rs:39` | 2d |  |
| `rounded_rect_clamps` | 0 | `crates/vector-locus-geometry/src/shapes.rs:282` | 2d |  |
| `rounded_rectangle` | 2 | `crates/vector-locus-geometry/src/shapes.rs:20` | 2d | Rounded rectangle (8 anchors). The radius is clamped to half the shorter side. |
| `rounding_logic` | 0 | `crates/chronicle-fiduciary-ledger/src/money.rs:144` | - |  |
| `roundtrip` | 3 | `crates/media-isobmff/src/codec.rs:1566` | - |  |
| `roundtrip_and_delay` | 0 | `crates/audio-aac-codec/tests/robustness.rs:137` | - |  |
| `roundtrip_every_curve` | 0 | `crates/color-transforms/src/log.rs:270` | - |  |
| `roundtrip_mixed_symbols` | 0 | `crates/audio-opus-codec/src/range.rs:420` | - |  |
| `impl AssetHost<'_>::route_bus_dag` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:11` | - |  |
| `impl AssetHost<'_>::route_input_action` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/diegetic_input.rs:217` | - |  |
| `routes_files_by_extension` | 0 | `crates/multimodal-synthesis-schema/src/references.rs:184` | - |  |
| `impl Frame::row` | 2 | `crates/video-h264-codec/src/picture.rs:165` | - | MB row `r`, waiting until it has been published. |
| `row_distance` | 4 | `crates/raster-selection-morphology/src/distance.rs:88` | - | Lower envelope of parabolas with integer vertical distances. Work is O(row length): each |
| `row_of` | 4 | `crates/radiance-color-grading/src/scopes/mod.rs:271` | - |  |
| `row_slice_widths` | 2 | `crates/video-prores-codec/src/header.rs:211` | - | Slice widths (in macroblocks) of one macroblock row: as many `1 << log2` slices as fit, then |
| `impl Matrix::row_vector` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:77` | - |  |
| `impl RangeRef::rows` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:152` | - | Whole rows `r0..=r1`. |
| `impl Grid::rows_in` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:247` | - |  |
| `rrect_from` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1271` | - |  |
| `rshift_round` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:25` | - |  |
| `rshift_round64` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:29` | - |  |
| `rule` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:83` | - |  |
| `impl Page::rule` | 5 | `crates/discourse-academic-tex/src/texpdf.rs:293` | - |  |
| `rule_pack_reproduces_legacy_hardcoded_packs` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:106` | - |  |
| `rule_pack_validation_rejects_corruption` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:124` | - |  |
| `impl Unit::ruler_step` | 2 | `crates/vector-locus-geometry/src/units.rs:79` | - | The rulers' label step at `zoom` (screen pixels per point), in this unit. |
| `rules_value_parsing_fails_closed` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:144` | - |  |
| `run` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:91` | - |  |
| `impl Fft::run` | 3 | `crates/audio-aac-codec/src/mdct.rs:28` | - |  |
| `run` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:48` | - |  |
| `run` | 2 | `crates/audio-device-cpal/examples/android.rs:41` | - |  |
| `run` | 2 | `crates/audio-device-cpal/examples/beep.rs:97` | - |  |
| `impl Parser::run` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:63` | - |  |
| `impl Sim::run` | 1 | `crates/kinetics-block-simulation/src/sim.rs:175` | - |  |
| `run` | 0 | `crates/vector-hybrid-renderer/examples/render_to_file.rs:26` | - |  |
| `run_ffmpeg` | 3 | `crates/media-isobmff/tests/common/mod.rs:112` | - |  |
| `run_input` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:318` | - |  |
| `run_jfke_tet_edge02` | 1 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:365` | - |  |
| `run_job` | 4 | `crates/video-h264-codec/src/decoder.rs:506` | - | Decode all slices of one picture and publish its rows. |
| `run_monte_carlo` | 3 | `crates/kinetics-schedule-cpm/src/schedule.rs:663` | - |  |
| `run_output` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:347` | - |  |
| `impl WgpuEngine::run_recording` | 8 | `crates/vector-compute-renderer/src/wgpu_engine.rs:380` | - |  |
| `run_sector_physics_verification_step` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:72` | physics | Runs a deterministic 3D XPBD + analytical SDF + 2-way continuum bridge + 4-wheel rover suspension |
| `run_states` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:687` | - |  |
| `s` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:18` | - |  |
| `impl GroupReader<'r, 'a>::s` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:157` | - |  |
| `s15` | 1 | `crates/color-transforms/src/icc/write.rs:7` | - |  |
| `impl Reader<'a>::s15f16` | 2 | `crates/color-transforms/src/icc/profile.rs:319` | - |  |
| `s15f16_round` | 1 | `crates/color-transforms/src/icc/math.rs:69` | - | Round to 16-bit fractional fixed point. |
| `safe_identifier` | 1 | `crates/surface-pbr-studio/src/materialx.rs:42` | - |  |
| `safe_inverse` | 1 | `crates/surface-pbr-studio/src/bvh.rs:120` | - |  |
| `safe_url` | 1 | `crates/layout-presentation-deck/src/timeline/show.rs:16` | - |  |
| `impl Encoder::sample` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:313` | - |  |
| `impl Clut::sample` | 3 | `crates/color-transforms/src/icc/clut.rs:42` | graph | Builds a table by sampling `f` at every node (inputs normalised to `[0, 1]`). |
| `impl Colormap::sample` | 2 | `crates/discourse-scientific-matlang/src/plot.rs:26` | - |  |
| `sample` | 0 | `crates/media-ogg/src/strict_opus.rs:133` | - |  |
| `sample` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:3` | - |  |
| `sample` | 4 | `crates/vector-locus-pathops/src/pathops/fit.rs:205` | - | Sample a cubic at `n+1` evenly spaced parameters. |
| `sample` | 4 | `crates/vector-radiance-trace/src/fit.rs:590` | - |  |
| `impl Track::sample_at_presentation_time` | 2 | `crates/media-isobmff/src/demux.rs:167` | - | Like [`sample_at_pts`](Self::sample_at_pts) but for edit-adjusted presentation time. |
| `impl Track::sample_at_pts` | 2 | `crates/media-isobmff/src/demux.rs:159` | - | Index of the sample being presented at raw media time `pts` (the last sample, in |
| `impl Mp4File::sample_at_pts` | 3 | `crates/media-isobmff/src/demux.rs:233` | - |  |
| `sample_bilinear` | 3 | `crates/kinetics-motion-compositor/src/track/klt.rs:254` | - |  |
| `impl RasterImage::sample_bilinear` | 3 | `crates/surface-pbr-studio/src/raster.rs:67` | - |  |
| `sample_bilinear_2d` | 4 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:129` | - | Bilinear interpolation sampler over a 2D periodic field. |
| `sample_cell` | 5 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:112` | - |  |
| `sample_clamped` | 5 | `crates/raster-selection-morphology/src/grayscale.rs:152` | - |  |
| `sample_format` | 1 | `crates/audio-device-cpal/examples/record_wav.rs:145` | - |  |
| `impl SupportedStreamConfig::sample_format` | 1 | `crates/audio-device-cpal/src/lib.rs:403` | - |  |
| `impl Data::sample_format` | 1 | `crates/audio-device-cpal/src/lib.rs:525` | - | The sample format of the internal audio data. |
| `impl SupportedStreamConfigRange::sample_format` | 1 | `crates/audio-device-cpal/src/lib.rs:628` | - |  |
| `impl CowTerrainTileGrid::sample_height` | 5 | `crates/terrain-brushes/adaptation/grid-impl.rs:265` | - |  |
| `impl CowTerrainTileGrid::sample_height` | 5 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:298` | - |  |
| `impl CowTerrainTileGrid::sample_height` | 5 | `crates/terrain-brushes/src/lib.rs:450` | - |  |
| `sample_height` | 3 | `crates/terrain-brushes/src/lib.rs:487` | - |  |
| `impl Project::sample_launch_program` | 0 | `crates/kinetics-schedule-cpm/src/model.rs:706` | - |  |
| `sample_line` | 0 | `crates/vector-locus-pathops/src/three_d/revolve.rs:266` | 2d |  |
| `sample_lookup` | 0 | `crates/media-isobmff/tests/roundtrip.rs:143` | - |  |
| `impl RasterImage::sample_nearest` | 3 | `crates/surface-pbr-studio/src/raster.rs:56` | - |  |
| `sample_pressure` | 2 | `crates/vector-surface-brush/src/calligraphic.rs:87` | - |  |
| `sample_pressure` | 2 | `crates/vector-surface-brush/src/scatter.rs:111` | - |  |
| `impl AssetHost<'_>::sample_procedural_field` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:320` | - |  |
| `sample_program_builds_and_tracks_costs` | 0 | `crates/kinetics-schedule-cpm/src/model.rs:865` | - |  |
| `impl Decoder::sample_rate` | 1 | `crates/audio-aac-codec/src/decoder.rs:96` | - |  |
| `impl jack::NotificationHandler for JackNotificationHandler::sample_rate` | 3 | `crates/audio-device-cpal/src/host/jack/stream.rs:443` | - |  |
| `impl SupportedStreamConfig::sample_rate` | 1 | `crates/audio-device-cpal/src/lib.rs:395` | - |  |
| `impl StreamDecoder::sample_rate` | 1 | `crates/audio-opus-codec/src/decoder.rs:58` | - |  |
| `impl Decoder::sample_rate` | 1 | `crates/audio-opus-codec/src/multistream.rs:49` | - |  |
| `impl AudioCaps::sample_rate` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:125` | - |  |
| `sample_rate_bounds` | 0 | `crates/audio-timebase/src/rate.rs:129` | - |  |
| `sample_rate_index` | 1 | `crates/audio-aac-codec/src/tables.rs:7` | - | Map an arbitrary sample rate to its `samplingFrequencyIndex`, if it is one of the standard rates. |
| `sample_rate_label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:587` | - |  |
| `sample_rates_8k_to_96k` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:206` | - |  |
| `sample_rgba32f` | 7 | `crates/raster-spatial-operators/src/distort.rs:396` | - |  |
| `sample_rgba8` | 7 | `crates/raster-spatial-operators/src/distort.rs:334` | - |  |
| `impl SampleFormat::sample_size` | 1 | `crates/audio-device-cpal/src/samples_formats.rs:55` | - | Returns the size in bytes of a sample of this format. |
| `sample_strategy` | 0 | `crates/media-isobmff/tests/roundtrip.rs:15` | - |  |
| `sample_tables_reject_hostile_counts_before_mp4_parser` | 0 | `crates/video-h264-preview/src/lib.rs:698` | - |  |
| `trait Block::sample_time` | 1 | `crates/kinetics-block-simulation/src/block.rs:73` | - |  |
| `impl Block for UnitDelay::sample_time` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:53` | - |  |
| `impl Block for ZeroOrderHold::sample_time` | 1 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:108` | - |  |
| `impl XorShift64::sample_triangular` | 4 | `crates/kinetics-schedule-cpm/src/schedule.rs:649` | - |  |
| `sample_virtual_rgba32f` | 6 | `crates/raster-spatial-operators/src/distort.rs:289` | - |  |
| `sample_virtual_rgba8` | 6 | `crates/raster-spatial-operators/src/distort.rs:244` | - |  |
| `impl SampleRate::samples` | 2 | `crates/audio-timebase/src/rate.rs:70` | - | Convert real seconds to samples, rounded to nearest and saturated. |
| `samples_along` | 2 | `crates/vector-surface-brush/src/calligraphic.rs:168` | 2d |  |
| `impl TempoMap::samples_at_bar_beat` | 3 | `crates/audio-timebase/src/tempo.rs:352` | - | Sample index at musical `BarBeat`. |
| `impl TimecodeRate::samples_at_frame` | 3 | `crates/audio-timebase/src/timecode.rs:112` | - | Sample position at the beginning of frame `f`. |
| `impl TimecodeRate::samples_per_frame` | 2 | `crates/audio-timebase/src/timecode.rs:122` | - | Samples per frame (fractional). |
| `impl TempoMap::samples_per_tick` | 2 | `crates/audio-timebase/src/tempo.rs:228` | - |  |
| `impl TempoMap::samples_to_ticks` | 3 | `crates/audio-timebase/src/tempo.rs:271` | - | Convert samples to integer ticks. |
| `impl TempoMap::samples_to_ticks_f` | 3 | `crates/audio-timebase/src/tempo.rs:253` | - | Convert samples to fractional ticks. |
| `impl FrameRate::sane` | 1 | `crates/timeline-editing/src/timebase.rs:157` | - | Invalid or non-positive rates use the source's documented default. |
| `sanitize_csi_phase` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:146` | numerics | Removes Carrier Frequency Offset (CFO) and Sampling Time Offset (STO) via linear regression. |
| `sanitize_document` | 2 | `crates/discourse-pdf-compliance/src/redact/sanitizer.rs:74` | - | Sanitizes a document manifest by removing all selected hidden information categories. |
| `impl Revolve::sanitized` | 1 | `crates/vector-locus-pathops/src/three_d/revolve.rs:93` | - |  |
| `sat` | 1 | `crates/raster-blend-operators/src/lib.rs:471` | - |  |
| `sat` | 1 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:69` | 3d |  |
| `sat16` | 1 | `crates/audio-opus-codec/src/silk/lpc.rs:33` | - |  |
| `impl Doc::save` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:320` | - | Serialize document to valid PDF 1.5 format. |
| `impl Scene::save_current_state` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:888` | - | Save a copy of the current rendering state. |
| `impl glifo::GlyphRenderer for Scene::save_state` | 1 | `crates/vector-hybrid-renderer/src/text.rs:319` | - |  |
| `impl Oscillator::saw_wave` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:75` | - |  |
| `impl Decoder::sbr` | 1 | `crates/audio-aac-codec/src/decoder.rs:102` | - | Whether the stream is HE-AAC: SBR signalled in the `AudioSpecificConfig` (explicit) or SBR |
| `sbr_fill_element_marks_implicit_he_aac` | 0 | `crates/audio-aac-codec/src/decoder.rs:568` | - | Implicitly signalled HE-AAC (an AAC-LC config) is recognised from the SBR fill element. |
| `impl Matrix::scalar` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:69` | - |  |
| `impl SigVal::scalar` | 1 | `crates/kinetics-block-simulation/src/signal.rs:100` | - |  |
| `impl Ttf::scale` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:159` | - | Font-units -> pt scale at `size` pt. |
| `impl Matrix::scale` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:242` | - |  |
| `impl Mat3::scale` | 2 | `crates/drafting-geometry/src/mat.rs:28` | - |  |
| `impl Complex32::scale` | 2 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:65` | - |  |
| `scale4` | 3 | `crates/video-h264-codec/src/transform.rs:34` | - | Scale one 4x4 AC coefficient (8.5.12.1, not the DC of Intra16x16 / chroma blocks). |
| `scale8` | 3 | `crates/video-h264-codec/src/transform.rs:40` | - | Scale one 8x8 coefficient (8.5.13.1). |
| `impl Mat3::scale_about` | 2 | `crates/drafting-geometry/src/mat.rs:36` | 2d | Scale about `base`. |
| `impl Mat3::scale_factor` | 1 | `crates/drafting-geometry/src/mat.rs:91` | - | Uniform scale factor (geometric mean of the axis scales). |
| `scale_functions` | 0 | `crates/video-h264-codec/src/transform.rs:285` | - |  |
| `scale_mask_density` | 2 | `crates/raster-layer-mask-adapter/src/masks.rs:12` | - | Scales layer mask density in-place or returns scaled copy. |
| `scale_rgba` | 5 | `crates/video-h264-preview/src/convert.rs:88` | - | Area-average downscale (or nearest upscale) of packed RGBA. |
| `impl TempoMap::scale_tempos` | 2 | `crates/audio-timebase/src/tempo.rs:357` | - | Scale all tempos by a constant ratio. |
| `scaled_weights` | 2 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:452` | - |  |
| `scalefactor_roundtrip` | 0 | `crates/audio-aac-codec/src/huffman.rs:295` | - |  |
| `scaling_averages` | 0 | `crates/video-h264-preview/src/convert.rs:146` | - |  |
| `scaling_list_default_signal` | 0 | `crates/video-h264-codec/src/params.rs:677` | - |  |
| `scaling_list_repeat_last` | 0 | `crates/video-h264-codec/src/params.rs:688` | - |  |
| `scan_all_pii` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:256` | - | Scans for all PII patterns concurrently. |
| `scan_credit_cards` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:46` | - | Scan text for credit card numbers (Visa, MasterCard, Amex, Discover, etc.) with Luhn verification. |
| `scan_emails` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:160` | - | Scan text for email addresses. |
| `scan_phones` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:206` | - | Scan text for phone numbers (E.164, North American NANP, and international trunk). |
| `scan_ssns` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:91` | - | Scan text for US Social Security Numbers (format `AAA-GG-SSSS` or 9 continuous digits with valid area). |
| `scan_tax_ids` | 1 | `crates/discourse-pdf-compliance/src/redact/scanner.rs:133` | - | Scan text for Employer Identification Numbers / Tax IDs (`XX-XXXXXXX`). |
| `scans_are_permutations` | 0 | `crates/video-prores-codec/src/tables.rs:118` | - |  |
| `impl Series::scatter` | 3 | `crates/discourse-scientific-matlang/src/plot.rs:83` | - |  |
| `scatter_stamps` | 0 | `crates/vector-surface-brush/src/scatter.rs:136` | - |  |
| `scene_replay_preserves_clip_rules` | 0 | `crates/vector-cpu-render-adapter/src/scene.rs:291` | - |  |
| `impl SceneCase::schedule` | 5 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:57` | - |  |
| `impl Scheduler<'a, 'p>::schedule_child_layer` | 4 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:485` | graph | Resolve and schedule the optional child layer referenced by a command node. |
| `impl AssetHost<'_>::schedule_cpm_evm_montecarlo` | 2 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:63` | - | 138: `hypermedia.kinetics.schedule_cpm_evm_montecarlo` |
| `schedule_generation_and_rounding` | 0 | `crates/chronicle-fiduciary-ledger/src/revrec.rs:152` | - |  |
| `impl SceneCase::schedule_into` | 6 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:85` | - |  |
| `impl Scheduler<'a, 'p>::schedule_layer` | 3 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:400` | - | Schedule an open layer bottom-up and return its completed, still-live allocation. |
| `schedule_point_stage_occurrences` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:581` | - |  |
| `schedule_project` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:104` | - | Run CPM forward and backward pass on `project`, updating every task's schedule fields in place. |
| `impl Scheduler<'a, 'p>::schedule_root` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:344` | - | Schedule the root command stream into the configured output target. |
| `impl SceneCase::schedule_root` | 2 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:107` | - |  |
| `schroeder_reverb` | 3 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:20` | - | Deterministic 4-comb, 2-allpass Schroeder reverberator tail generator. |
| `impl GpuProfiler::scope` | 3 | `crates/gpu-command-profiler/src/profiler.rs:143` | - | Starts a new auto-closing profiler scope. |
| `scopes_to_console_recursive` | 2 | `crates/gpu-command-profiler/examples/demo.rs:18` | - |  |
| `impl WebGlResources::scratch_binding_texture` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1328` | - |  |
| `impl WebGlResources::scratch_framebuffer` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1336` | - |  |
| `impl Atlases::scratch_texture` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:42` | - |  |
| `impl Cursor::scratch_texture` | 1 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:42` | - | Whether access to the scratch texture has been requested. |
| `impl ScheduledCase::scratch_texture` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:156` | - |  |
| `impl GpuResources::scratch_view` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1065` | - |  |
| `screen` | 2 | `crates/raster-blend-operators/src/lib.rs:455` | - |  |
| `screen` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:24` | 3d |  |
| `scribble` | 3 | `crates/vector-locus-pathops/src/effects/scribble.rs:32` | 2d | Scribble: generates zigzag hatching across a shape. |
| `scribble_and_noise` | 0 | `crates/vector-locus-geometry/src/recognize.rs:367` | - |  |
| `scribble_generates_hatching` | 0 | `crates/vector-locus-pathops/src/effects/scribble.rs:122` | - |  |
| `scripts` | 6 | `crates/discourse-academic-tex/src/typeset.rs:452` | - |  |
| `scripts_and_frac` | 0 | `crates/discourse-academic-tex/src/typeset.rs:525` | - |  |
| `scrub` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:24` | - |  |
| `impl AssetHost<'_>::sculpt_terrain` | 2 | `crates/asset-production-vibe/src/geodesy_ops.rs:325` | - |  |
| `sculpt_tile` | 5 | `crates/asset-production-vibe/src/geodesy_ops.rs:104` | - |  |
| `sdiv` | 2 | `crates/audio-opus-codec/src/celt/bands.rs:243` | - | C-style truncating signed division. |
| `sdr_in_hdr_keeps_values_and_display_tone_maps` | 0 | `crates/color-transforms/src/transform.rs:466` | - |  |
| `sdr_is_the_srgb_signal` | 0 | `crates/color-transforms/src/grade.rs:130` | - |  |
| `seal_ik_pose_skl1_10d` | 4 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:469` | - | Bind-pose triangle + 3-joint SKL1 attached into a sealed `.10d`. |
| `seal_mesh_10d_with_slim_uvs` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:310` | computational geometry | Seal SLIM TEXCOORD_0 into a QualiaDB `.10d` so unwrap feeds `compile_10d_pbr`. |
| `seal_whitegum_organ_10d` | 8 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:882` | - |  |
| `seam_carve_rgba32f` | 5 | `crates/raster-spatial-operators/src/seam_carve.rs:50` | 2d | Carve seams from an RGBA32F buffer (`[f32]`, 4 values per pixel). |
| `seam_carve_rgba8` | 5 | `crates/raster-spatial-operators/src/seam_carve.rs:13` | - | Carve seams from an RGBA8 buffer to achieve target dimensions. |
| `search_link` | 2 | `crates/vector-compute-shaders/shader/clip_leaf.wgsl:37` | - |  |
| `search_sf` | 3 | `crates/audio-aac-codec/src/encoder/quant.rs:119` | - | Largest scalefactor whose distortion stays within `allowed`. |
| `second_unit_offset` | 2 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:134` | - |  |
| `impl SampleRate::seconds` | 2 | `crates/audio-timebase/src/rate.rs:65` | - | Convert samples to real seconds. |
| `seconds_round_trip` | 0 | `crates/audio-timebase/src/rate.rs:136` | - |  |
| `seed` | 1 | `crates/vector-locus-pathops/src/effects/util.rs:36` | - |  |
| `seed_of` | 2 | `crates/vector-surface-brush/src/track.rs:207` | - | A stable seed for a path (FNV-1a over its coordinates' bits, quantised to 1/64 pt). |
| `impl BitReader<'a>::seek_bits` | 2 | `crates/bitstream-codec/src/lib.rs:64` | - |  |
| `seg_cubic` | 2 | `crates/vector-locus-pathops/src/effects/util.rs:61` | - |  |
| `seg_dist` | 3 | `crates/vector-radiance-trace/src/fit.rs:19` | - |  |
| `seg_snr` | 2 | `crates/audio-aac-codec/tests/common/mod.rs:228` | - | Segmental SNR (1024-sample segments above -50 dBFS, each clamped to [-10, 60] dB). |
| `impl SubPath::segment` | 2 | `crates/vector-locus-geometry/src/path.rs:207` | - | Segment `i` as a cubic (lines are cubics with handles on the anchors). |
| `impl SubPath::segment_count` | 1 | `crates/vector-locus-geometry/src/path.rs:196` | - | Number of segments (a closed path has one more, back to the start). |
| `segment_intersection` | 4 | `crates/geodesy-spatial-analysis/src/network_topology.rs:360` | computational geometry | Returns the intersection point of two 2D segments if they intersect strictly. |
| `impl SubPath::segment_is_line` | 2 | `crates/vector-locus-geometry/src/path.rs:214` | - | True if segment `i` is a straight line. |
| `segment_is_ruled` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:52` | numerics | Ruled/linear cubics stay named two-point spans (no print-density tessellation). |
| `segment_musical_phrases` | 1 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:129` | - | Detects 8-bar musical phrases from energy envelopes across bars. |
| `segment_offset` | 0 | `crates/drafting-geometry/src/tests.rs:180` | - |  |
| `impl Polyline::segments` | 1 | `crates/drafting-geometry/src/curve.rs:471` | - | The segments, including the closing segment when closed. |
| `segments` | 1 | `crates/vector-locus-geometry/src/bez.rs:35` | - | The segments of one subpath. |
| `impl Track::segments` | 1 | `crates/vector-surface-brush/src/track.rs:92` | - |  |
| `segs_to_subpath` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:56` | - |  |
| `segs_to_subpath` | 2 | `crates/vector-radiance-trace/src/fit.rs:245` | - |  |
| `impl Shader::select` | 1 | `crates/vector-compute-renderer/src/wgpu_engine.rs:79` | - |  |
| `selected_operators_are_bit_equal_around_edges_midpoint_and_extremes` | 0 | `crates/raster-blend-operators/src/migration_oracle.rs:89` | - |  |
| `selected_operators_are_bit_equal_to_source_on_normalized_grid` | 0 | `crates/raster-blend-operators/src/migration_oracle.rs:69` | - |  |
| `selected_p_frame_without_prior_idr_is_refused` | 0 | `crates/video-h264-preview/src/lib.rs:740` | - |  |
| `selected_pixels_stay_at_zero_past_coordinate_4096` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:66` | - |  |
| `selection_modifiers_keep_fractional_coverage` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:82` | - |  |
| `self_delimited` | 0 | `crates/audio-opus-codec/src/packet.rs:326` | - |  |
| `self_verify_m64` | 3 | `crates/asset-production-vibe/src/geodesy_ops.rs:146` | - |  |
| `impl VariogramParams::semivariance` | 2 | `crates/geodesy-spatial-analysis/src/kriging.rs:40` | - | Evaluates semivariance $\gamma(h)$ for lag distance $h$. |
| `impl JackNotificationHandler::send_error` | 2 | `crates/audio-device-cpal/src/host/jack/stream.rs:429` | - |  |
| `sep` | 3 | `crates/discourse-academic-tex/src/typeset.rs:179` | - | Space (in fractions of `size`) between adjacent atoms. |
| `separable_modes_follow_the_reference_formulas` | 0 | `crates/raster-blend-operators/src/lib.rs:563` | - |  |
| `sequence` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:7` | - |  |
| `serde_roundtrip` | 0 | `crates/vector-locus-geometry/src/path.rs:619` | - |  |
| `serial_from_ymd` | 4 | `crates/chronicle-tabular-engine/src/core/date.rs:79` | - |  |
| `impl Serialize for WirePoint::serialize` | 2 | `crates/vector-locus-geometry/src/path.rs:74` | - |  |
| `serialize_kml` | 2 | `crates/locus-geospatial-codecs/src/formats/kml.rs:103` | - | Serializes placemarks into a valid KML 2.2 XML document string. |
| `impl FilterContext::serialize_to_buffer` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:646` | - |  |
| `impl GpuEncodedPaint::serialize_to_buffer` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:508` | - | Serialize paint enums directly into the provided buffer. Returns the number of bytes written. |
| `impl Matrix::set` | 4 | `crates/discourse-scientific-matlang/src/matrix.rs:132` | - |  |
| `impl BlendIf::set` | 3 | `crates/raster-layer-mask-adapter/src/masks.rs:220` | - |  |
| `impl Scene::set_aliasing_threshold` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:445` | 2d | Set the aliasing threshold. |
| `impl Catalog::set_audio` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:238` | - |  |
| `impl Cabac<'a>::set_bit_pos` | 2 | `crates/video-h264-codec/src/cabac.rs:133` | - | Continue reading at byte-aligned bit position `pos` (call `init_engine` afterwards). |
| `impl Scene::set_blend_mode` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:744` | - | Set the blend mode for subsequent rendering operations. |
| `impl Movie::set_cache_frames` | 2 | `crates/video-h264-preview/src/movie.rs:309` | - |  |
| `impl Host::set_connect_automatically` | 2 | `crates/audio-device-cpal/src/host/jack/mod.rs:46` | - | Set whether the ports should automatically be connected to system |
| `impl SliceDecoder<'_>::set_cur_mvd` | 7 | `crates/video-h264-codec/src/cabac_mb.rs:274` | - |  |
| `impl SliceDecoder<'_>::set_cur_ref` | 7 | `crates/video-h264-codec/src/cabac_mb.rs:265` | - |  |
| `impl Decoder::set_draft` | 2 | `crates/video-h264-codec/src/decoder.rs:174` | - | Draft mode (off by default) for reduced-resolution playback: pictures submitted from now |
| `impl Scene::set_fill_rule` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:806` | - | Set the fill rule for subsequent fill operations. |
| `impl Scene::set_filter_effect` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:821` | - | Apply filter to the current paint (affects next drawn element). |
| `impl Sheet::set_formula` | 3 | `crates/chronicle-tabular-engine/src/calc/model.rs:111` | - |  |
| `impl Oscillator::set_frequency` | 2 | `crates/acoustics-daw-session/src/dsp/osc.rs:42` | - |  |
| `impl Channel::set_fs` | 3 | `crates/audio-opus-codec/src/silk/mod.rs:120` | - |  |
| `impl StreamDecoder::set_gain_q8` | 2 | `crates/audio-opus-codec/src/decoder.rs:77` | - | Sets the output gain in Q8 dB (as in the OpusHead header). |
| `impl Anchor::set_handle` | 4 | `crates/vector-locus-geometry/src/path.rs:143` | - | Place the end of the outgoing (`out`) or incoming handle at `pos`. `independent` makes the |
| `set_hw_params_from_format` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:996` | - |  |
| `set_hyphenation_exceptions` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:50` | - |  |
| `impl Catalog::set_image` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:228` | - |  |
| `set_key` | 1 | `crates/chronicle-broadcast-containers/src/mxf/labels.rs:3` | - |  |
| `impl Decoder::set_layout` | 2 | `crates/audio-aac-codec/src/decoder.rs:81` | - |  |
| `impl JournalLedger::set_lock_date` | 2 | `crates/chronicle-fiduciary-ledger/src/journal.rs:84` | - |  |
| `set_lum` | 2 | `crates/raster-blend-operators/src/lib.rs:496` | - |  |
| `set_lum` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:97` | 3d |  |
| `impl SkiaScenePainter<'_>::set_matrix` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:108` | numerics |  |
| `impl TempoMap::set_meter` | 4 | `crates/audio-timebase/src/tempo.rs:168` | - | Insert or update the meter at `tick` (snapped to the bar line at or before it). |
| `impl SliceDecoder<'a>::set_motion` | 7 | `crates/video-h264-codec/src/slicedec.rs:527` | - | Store motion for a rectangle (4x4 units) of the current MB and mark it decoded. |
| `impl PerspectiveGrid::set_offset` | 3 | `crates/vector-locus-geometry/src/perspective.rs:156` | 3d | Move `plane` to `offset` along its normal. |
| `impl vello_common::probe::ProbeRenderer for Scene::set_paint` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:326` | - |  |
| `impl Scene::set_paint` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:773` | - |  |
| `impl DrawSink for Scene::set_paint` | 2 | `crates/vector-hybrid-renderer/src/text.rs:157` | - |  |
| `impl SkiaScenePainter<'_>::set_paint_alpha` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:94` | - |  |
| `impl SkiaScenePainter<'_>::set_paint_blend_mode` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:102` | - |  |
| `impl SkiaScenePainter<'_>::set_paint_brush` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:129` | - |  |
| `impl SkiaScenePainter<'_>::set_paint_filter` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:98` | - |  |
| `impl glifo::GlyphRenderer for Scene::set_paint_image` | 2 | `crates/vector-hybrid-renderer/src/text.rs:334` | - |  |
| `impl SkiaScenePainter<'_>::set_paint_style` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:176` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::set_paint_transform` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:350` | - |  |
| `impl Scene::set_paint_transform` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:796` | - | Set the current paint transform. |
| `impl DrawSink for Scene::set_paint_transform` | 2 | `crates/vector-hybrid-renderer/src/text.rs:162` | - |  |
| `impl StreamDecoder::set_phase_inversion_disabled` | 2 | `crates/audio-opus-codec/src/decoder.rs:82` | - | Disables the 180-degree intensity-stereo phase inversion (RFC 8251 §10). |
| `impl Decoder::set_phase_inversion_disabled` | 2 | `crates/audio-opus-codec/src/multistream.rs:69` | - | Disables intensity-stereo phase inversion (useful when downmixing; RFC 8251 section 10). |
| `impl RasterImage::set_pixel` | 4 | `crates/surface-pbr-studio/src/raster.rs:46` | 2d |  |
| `set_plane_allocator` | 1 | `crates/video-h264-codec/src/picture.rs:281` | - | Where output planes come from: an empty buffer with room for the given number of samples. |
| `impl SurfaceRenderer<'s>::set_present_mode` | 2 | `crates/gpu-device-context/src/surface_renderer.rs:234` | - |  |
| `impl Pixels<'win>::set_present_mode` | 2 | `crates/pixel-buffer-output/src/lib.rs:466` | - | Set the `wgpu` present mode. |
| `impl RenderContext::set_present_mode` | 3 | `crates/vector-compute-renderer/src/util.rs:130` | - |  |
| `set_sample_rate` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:733` | - | Attempt to set the device sample rate to the provided rate. |
| `set_sat` | 2 | `crates/raster-blend-operators/src/lib.rs:501` | - |  |
| `set_sat` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:117` | 3d |  |
| `set_sat_inner` | 4 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:101` | - |  |
| `impl Pixels<'win>::set_scaling_mode` | 2 | `crates/pixel-buffer-output/src/lib.rs:325` | 2d | Set the scaling mode. |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::set_size` | 3 | `crates/pixel-window-render-adapter/src/lib.rs:195` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::set_size` | 3 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:432` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::set_size` | 3 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:450` | - |  |
| `impl SkiaBackend for MetalBackend::set_size` | 3 | `crates/vector-skia-render-adapter/src/metal.rs:102` | - |  |
| `impl SkiaBackend for OpenGLBackend::set_size` | 3 | `crates/vector-skia-render-adapter/src/opengl.rs:201` | - |  |
| `impl SkiaBackend for VulkanBackend::set_size` | 3 | `crates/vector-skia-render-adapter/src/vulkan.rs:411` | - |  |
| `trait SkiaBackend::set_size` | 3 | `crates/vector-skia-render-adapter/src/window_renderer.rs:10` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::set_size` | 3 | `crates/vector-skia-render-adapter/src/window_renderer.rs:160` | - |  |
| `impl Host::set_start_server_automatically` | 2 | `crates/audio-device-cpal/src/host/jack/mod.rs:51` | - | Set whether a JACK server should be automatically started if it isn't already. |
| `impl SilkDecoder::set_stream` | 3 | `crates/audio-opus-codec/src/silk/mod.rs:685` | - |  |
| `impl WebGlRendererContext<'_>::set_strip_attrib_offset` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2736` | 3d | Point the strip vertex attributes at the instance at `first_instance`. |
| `impl Scene::set_stroke` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:754` | - | Set the stroke settings for subsequent stroke operations. |
| `set_sw_params_from_format` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:1078` | - |  |
| `impl TempoMap::set_tempo` | 3 | `crates/audio-timebase/src/tempo.rs:143` | - | Insert or update the tempo at `tick`. |
| `set_timeout` | 6 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:344` | - |  |
| `impl Scene::set_tint` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:778` | - | Set the tint for subsequent image paint operations. |
| `impl glifo::GlyphRenderer for Scene::set_tint` | 2 | `crates/vector-hybrid-renderer/src/text.rs:339` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::set_transform` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:322` | - |  |
| `impl Scene::set_transform` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:811` | - | Set the transform for subsequent rendering operations. |
| `impl DrawSink for Scene::set_transform` | 2 | `crates/vector-hybrid-renderer/src/text.rs:152` | - |  |
| `set_transition_span` | 4 | `crates/timeline-editing/src/transition.rs:140` | - | Validate an edit before a host mutates its sequence/session. On refusal the |
| `impl Decoder::set_trim_pre_skip` | 2 | `crates/audio-opus-codec/src/multistream.rs:63` | - | When enabled, the first [`Self::pre_skip`] output samples are dropped. |
| `impl Sheet::set_value` | 3 | `crates/chronicle-tabular-engine/src/calc/model.rs:107` | - |  |
| `impl MathContext::set_var` | 3 | `crates/layout-nodegraph-compiler/src/math_parser/eval.rs:24` | - |  |
| `impl Catalog::set_video` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:233` | - |  |
| `impl Oscillator::set_waveform` | 2 | `crates/audio-device-cpal/examples/synth_tones.rs:41` | - |  |
| `trait Block::set_x` | 2 | `crates/kinetics-block-simulation/src/block.rs:89` | - |  |
| `impl Block for Integrator::set_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:59` | - |  |
| `impl Block for PidController::set_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:194` | - |  |
| `impl Block for TransferFcn::set_x` | 2 | `crates/kinetics-block-simulation/src/blocks/continuous.rs:289` | - |  |
| `impl Corner::setback` | 2 | `crates/vector-locus-geometry/src/corners.rs:66` | - | How far from the corner a round cut of radius `r` meets each side: r / tan(angle / 2). |
| `impl GpuProfiler::settings` | 1 | `crates/gpu-command-profiler/src/profiler.rs:106` | - | Returns currently active settings. |
| `setup` | 0 | `crates/video-h264-codec/src/intra.rs:381` | - |  |
| `sf_bits` | 1 | `crates/audio-aac-codec/src/huffman.rs:121` | - | Bits of one scalefactor difference. |
| `sf_gain` | 1 | `crates/audio-aac-codec/src/tables.rs:115` | - | `2^(0.25 * (sf - 100))`, the dequantisation gain of scalefactor `sf`. |
| `impl Decoder::sf_index` | 1 | `crates/audio-aac-codec/src/decoder.rs:121` | - |  |
| `sgn` | 1 | `crates/locus-parametric-solver/src/solver/residual.rs:111` | - |  |
| `sha256_compress` | 2 | `crates/discourse-pdf-compliance/src/sign/digest.rs:65` | - |  |
| `sha256_hex` | 1 | `crates/asset-production-vibe/src/operations_catalog.rs:1060` | - |  |
| `sha256_slices` | 1 | `crates/discourse-pdf-compliance/src/sign/digest.rs:6` | - | Computes SHA-256 hash over an array of byte slices (e.g. non-contiguous ByteRange segments). |
| `shader_dir` | 0 | `crates/vector-compute-shaders/src/compile/mod.rs:272` | 3d |  |
| `shader_from_gradient` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:1002` | 3d |  |
| `shader_from_image_brush` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:952` | 3d |  |
| `impl Matrix::shape` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:141` | - |  |
| `impl Shaper::shape` | 2 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:196` | - |  |
| `shape_builder` | 1 | `crates/vector-locus-pathops/src/pathops/planar.rs:31` | - | Shape builder planar region extractor. |
| `shape_builder_extracts_faces` | 0 | `crates/vector-locus-pathops/src/pathops/planar.rs:67` | - |  |
| `impl Slide::shape_by_id` | 2 | `crates/layout-presentation-deck/src/model/master.rs:89` | - |  |
| `impl Slide::shape_by_id_mut` | 2 | `crates/layout-presentation-deck/src/model/master.rs:93` | - |  |
| `shape_deriv_hex8` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:132` | - | Hex8 derivatives: [[d/dxi, d/deta, d/dzeta]; 8]. |
| `shape_deriv_quad4` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:79` | - | Quad4 derivatives: [[dN_i/dxi, dN_i/deta]; 4]. |
| `shape_deriv_tet4` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:113` | - | Tet4 derivatives: [[d/dxi, d/deta, d/dzeta]; 4]. |
| `shape_deriv_tri3` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:49` | - | Tri3 shape function derivatives: [d/dxi, d/deta]. |
| `shape_glyph_run` | 7 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:137` | - | Shapes, measures, and positions vector glyph runs into SVG path instructions. |
| `impl AssetHost<'_>::shape_harfbuzz_bidi` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:161` | - |  |
| `shape_harfbuzz_bidi_mixed_and_rtl_numbers_l2` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:550` | - |  |
| `shape_hex8` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:118` | graph | Trilinear 8-node hexahedron (Hex8) shape functions in [-1, 1]^3. |
| `shape_quad4` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:69` | graph | Bilinear 4-node quadrilateral (Quad4) shape functions in [-1, 1]^2. |
| `shape_quad8` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:89` | graph | Serendipity 8-node quadrilateral (Quad8) shape functions in [-1, 1]^2. |
| `shape_tet4` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:107` | graph, numerics | Linear 4-node tetrahedron (Tet4) shape functions in volume coordinates (xi, eta, zeta). |
| `shape_tri3` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:43` | graph, numerics | Linear 3-node triangle (Tri3) shape functions in area coordinates (xi, eta). |
| `shape_tri6` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:54` | graph | Quadratic 6-node triangle (Tri6) shape functions. |
| `shape_value` | 2 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:122` | - |  |
| `impl ElasticIsotropic::shear_modulus` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:32` | - | Second Lamé parameter (shear modulus) mu = E / (2 * (1 + nu)). |
| `impl Workbook::sheet` | 2 | `crates/chronicle-tabular-engine/src/calc/model.rs:163` | - |  |
| `impl Workbook::sheet_mut` | 2 | `crates/chronicle-tabular-engine/src/calc/model.rs:167` | - |  |
| `impl Reference::sheet_name` | 1 | `crates/chronicle-tabular-engine/src/formula/ast.rs:59` | - |  |
| `sheet_prefix` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:33` | - |  |
| `shell_decode` | 3 | `crates/audio-opus-codec/src/silk/mod.rs:563` | - |  |
| `impl MathLayout::shift` | 3 | `crates/layout-paged-typesetter/src/equation/layout.rs:41` | - |  |
| `impl RectPart::shift` | 2 | `crates/vector-hybrid-renderer/src/rect.rs:23` | - |  |
| `shift_anchor` | 3 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:17` | - |  |
| `shift_line` | 4 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:30` | - |  |
| `shift_ref` | 3 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:42` | - |  |
| `shift_relative` | 3 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:7` | - |  |
| `impl SampleRange::shifted` | 2 | `crates/audio-timebase/src/rate.rs:116` | - |  |
| `shoelace` | 1 | `crates/drafting-geometry/src/curve.rs:556` | computational geometry, 2d | Signed polygon area (CCW positive). |
| `shoelace_preserves_small_polygon_area_at_large_coordinates` | 0 | `crates/drafting-geometry/src/tests.rs:296` | computational geometry |  |
| `short_words_do_not_hyphenate` | 0 | `crates/vector-glyph-shaping/src/hyphen.rs:166` | - |  |
| `impl OtFeatures::should_keep_ligatures` | 2 | `crates/vector-glyph-shaping/src/features.rs:114` | - | Determine if standard ligatures should be kept given tracking value. |
| `show_order` | 1 | `crates/layout-presentation-deck/src/timeline/show.rs:43` | - | Slide indices visited in order, filtering out hidden slides and applying ranges/custom shows. |
| `impl RangeDecoder<'a>::shrink_storage` | 2 | `crates/audio-opus-codec/src/range.rs:68` | - | Shrinks the buffer seen by the decoder (raw bits are then read from the new end). |
| `impl jack::NotificationHandler for JackNotificationHandler::shutdown` | 3 | `crates/audio-device-cpal/src/host/jack/stream.rs:439` | - |  |
| `sibling_batch` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:494` | - |  |
| `sibling_case` | 1 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:175` | - |  |
| `impl Line::side` | 2 | `crates/drafting-geometry/src/curve.rs:46` | 2d | Signed side: > 0 left of a→b. |
| `sidecar_text` | 3 | `crates/media-test-support/src/golden.rs:213` | - | The attribution sidecar text for a generated golden image. |
| `impl ProfileClass::sig` | 1 | `crates/color-transforms/src/icc/profile.rs:83` | - |  |
| `impl ColorSpace::sig` | 1 | `crates/color-transforms/src/icc/profile.rs:138` | - |  |
| `sig_chords` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:28` | - |  |
| `sig_clicks` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:31` | - |  |
| `sig_noise` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:22` | - |  |
| `sig_speech` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:25` | - |  |
| `sig_sweep` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:19` | - |  |
| `signal_from_normalized` | 3 | `crates/color-transforms/src/log.rs:111` | - | Decoder-normalised value (0..1 in the file's range) → curve-domain signal. |
| `impl LogCurve::signal_from_normalized` | 3 | `crates/radiance-color-grading/src/log.rs:102` | - |  |
| `impl GradeSpace::signal_of_nits` | 2 | `crates/color-transforms/src/grade.rs:105` | - | The signal of a grey of `nits` cd/m² (HDR) — what a nit-scaled slider position means. |
| `impl GradeSpace::signal_of_nits` | 2 | `crates/radiance-color-grading/src/grade.rs:97` | - |  |
| `signed` | 2 | `crates/asset-production-vibe/src/graphics.rs:76` | - |  |
| `impl SubPath::signed_area` | 1 | `crates/vector-locus-geometry/src/path.rs:243` | - | Signed area, closing an open subpath with a line: positive when it runs clockwise on |
| `signed_unit_conversion_keeps_source_truncation_distinct_from_floor` | 0 | `crates/timeline-editing/src/timebase.rs:240` | - |  |
| `impl OpusHead::simple` | 1 | `crates/audio-opus-codec/src/header.rs:28` | - | A family-0 configuration for plain mono/stereo streams (no header present). |
| `impl Decoder::simple` | 2 | `crates/audio-opus-codec/src/multistream.rs:29` | - | Creates a decoder for plain mono/stereo packets without a header. |
| `simple` | 1 | `crates/video-h264-codec/src/cavlc.rs:73` | - |  |
| `simple_fnv1a` | 1 | `crates/electromagnetics-rf-sensing/src/densepose.rs:175` | - |  |
| `simple_layer` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:271` | - |  |
| `simple_roundtrip_and_faststart` | 0 | `crates/media-isobmff/src/mux.rs:896` | - |  |
| `simplify` | 2 | `crates/vector-locus-geometry/src/recognize.rs:52` | - | Douglas–Peucker simplification with bounded stack use. |
| `simplify` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:33` | - | Simplify with default corner threshold (30°). |
| `simplify_freehand` | 2 | `crates/vector-locus-geometry/src/recognize.rs:91` | - | Alias for [`simplify`] for freehand strokes. |
| `simplify_freehand_test` | 0 | `crates/vector-locus-geometry/src/recognize.rs:374` | - |  |
| `impl AssetHost<'_>::simplify_mesh_buffers` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:134` | computational geometry |  |
| `simplify_with` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:85` | - | Least-squares Bézier refit keeping corners sharper than the threshold. |
| `simplify_with` | 2 | `crates/vector-radiance-trace/src/fit.rs:297` | - |  |
| `impl AssetHost<'_>::simulate_causal_block_diagram` | 2 | `crates/asset-production-vibe/src/kinetics_fem_garment_blocks.rs:209` | physics | 137: `hypermedia.kinetics.simulate_causal_block_diagram` (`kinetics-block-simulation` + `kinetics-motion-compositor`) |
| `simulate_snow` | 4 | `crates/terrain-erosion-solvers/src/snow_accumulation.rs:58` | physics | Simulate snow precipitation, elevation-based freezing, melting, and avalanche transport. |
| `impl Corner::sin` | 1 | `crates/vector-locus-geometry/src/corners.rs:56` | - | The sine of the angle between the sides. |
| `sinc` | 1 | `crates/raster-spatial-operators/src/resample.rs:73` | - |  |
| `sine_half` | 1 | `crates/audio-aac-codec/src/mdct.rs:163` | - |  |
| `impl Oscillator::sine_wave` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:54` | - |  |
| `impl Array::single` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:108` | - |  |
| `impl PathData::single` | 1 | `crates/vector-locus-geometry/src/path.rs:357` | - |  |
| `single_colour_lands_in_exact_histogram_bins` | 0 | `crates/radiance-color-grading/src/scopes/tests.rs:42` | - |  |
| `single_crater_depresses_center_and_elevates_rim` | 0 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:160` | - |  |
| `single_dof_harmonic_oscillator` | 0 | `crates/kinetics-continuum-multiphysics/src/explicit/dynamics.rs:73` | - |  |
| `single_pass_filters_finish_in_original` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:806` | - |  |
| `single_weight` | 5 | `crates/vector-compute-shaders/shader/fine.wgsl:936` | - |  |
| `sixteenth_grid_at_120` | 0 | `crates/audio-timebase/src/grid.rs:232` | - |  |
| `impl Lut1d::size` | 1 | `crates/color-transforms/src/lut.rs:31` | - |  |
| `impl BufferRenderer::size` | 1 | `crates/gpu-device-context/src/buffer_renderer.rs:71` | - |  |
| `impl Lut1d::size` | 1 | `crates/radiance-color-grading/src/lut.rs:26` | - |  |
| `impl ResourcePool::size_class` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1003` | - | Quantize a size up to the nearest size class. |
| `impl Iterator for Devices::size_hint` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:910` | - |  |
| `impl GpuEncodedPaint::size_texels` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:497` | - | Number of `RGBA32Uint` texels occupied by this paint. |
| `skeletonize_mask` | 3 | `crates/raster-selection-morphology/src/grayscale.rs:281` | 2d, graph | Topological skeletonization via the Zhang-Suen thinning algorithm. |
| `impl MaterialPreset::skin` | 2 | `crates/surface-pbr-studio/src/material.rs:142` | - |  |
| `impl BitReader<'a>::skip` | 2 | `crates/bitstream-codec/src/lib.rs:112` | - |  |
| `impl Parser<'_>::skip` | 1 | `crates/manifold-bim-csg/src/step.rs:130` | - |  |
| `impl Cur<'a>::skip` | 2 | `crates/media-isobmff/src/bytes.rs:86` | - |  |
| `skip` | 2 | `crates/media-test-support/src/oracle.rs:100` | - | Report a skipped test: prints `SKIPPED (<context>): <reason>`. With |
| `impl Parser::skip_balanced` | 3 | `crates/discourse-academic-tex/src/texparse/bib.rs:401` | - |  |
| `impl Parser::skip_newlines_and_semicolons` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:30` | - |  |
| `impl RangeDecoder<'a>::skip_to_end` | 2 | `crates/audio-opus-codec/src/range.rs:238` | - | Advances the bit counter so that [`Self::tell`] reports `total` (CELT silence frames). |
| `skip_whitespace` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:250` | - |  |
| `impl Parser::skip_whitespace` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:142` | - |  |
| `impl Parser::skip_ws` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:114` | - |  |
| `skip_ws` | 2 | `crates/discourse-academic-tex/src/texparse/structure.rs:291` | - |  |
| `impl Parser::skip_ws_and_comments` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:120` | - |  |
| `impl Reader<'a>::slice` | 3 | `crates/color-transforms/src/icc/profile.rs:305` | - |  |
| `impl Matrix::slice` | 5 | `crates/discourse-scientific-matlang/src/matrix.rs:162` | - | 2D slicing: extracts submatrix over [r_start..r_end] x [c_start..c_end] (0-based, half-open). |
| `slice_bits` | 2 | `crates/video-prores-codec/src/encode.rs:327` | - |  |
| `slice_header` | 3 | `crates/video-h264-codec/src/synth_tests.rs:117` | - |  |
| `slice_nal` | 2 | `crates/video-h264-codec/src/synth_tests.rs:281` | - |  |
| `slice_reads` | 0 | `crates/media-isobmff/src/source.rs:109` | - |  |
| `slice_widths` | 0 | `crates/video-prores-codec/src/header.rs:231` | - |  |
| `impl RangedSlice<'a, T>::slices` | 1 | `crates/vector-hybrid-renderer/src/util.rs:194` | - |  |
| `impl Dpb<F>::sliding_window` | 4 | `crates/video-h264-codec/src/dpb.rs:110` | - | Sliding window marking (8.2.5.3). |
| `sln` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:388` | - |  |
| `slog3_decode` | 1 | `crates/color-transforms/src/log.rs:134` | - |  |
| `slog3_decode` | 1 | `crates/radiance-color-grading/src/log.rs:130` | - |  |
| `slog3_encode` | 1 | `crates/color-transforms/src/log.rs:131` | - |  |
| `slog3_encode` | 1 | `crates/radiance-color-grading/src/log.rs:122` | - |  |
| `impl Model::slot` | 2 | `crates/locus-parametric-solver/src/solver/model.rs:191` | - |  |
| `impl HumanAnatomyProfile::slug` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:80` | - |  |
| `small_disc_area_within_tolerance` | 0 | `crates/vector-radiance-trace/src/tests.rs:45` | - |  |
| `smlawb` | 3 | `crates/audio-opus-codec/src/silk/lpc.rs:13` | - |  |
| `smmul` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:21` | - |  |
| `impl Anchor::smooth` | 2 | `crates/vector-locus-geometry/src/path.rs:119` | - | A smooth anchor with a symmetric out handle `out` (the in handle is mirrored). |
| `smooth` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:173` | - | Smooth handles towards Catmull-Rom tangents. |
| `impl SubPath::smooth_anchor` | 2 | `crates/vector-locus-geometry/src/path.rs:272` | - | Make anchor `i` smooth: handles in line with its neighbours (its one neighbour at an open |
| `smooth_detection` | 0 | `crates/vector-locus-geometry/src/path.rs:606` | - |  |
| `smooth_fade` | 7 | `crates/audio-opus-codec/src/decoder.rs:408` | - |  |
| `smooth_noise_2d` | 3 | `crates/kinetics-motion-compositor/src/typography/wiggly.rs:44` | - | 2D coherent gradient noise. |
| `smooth_points` | 1 | `crates/vector-locus-pathops/src/effects/util.rs:40` | - |  |
| `smoothing_uses_one_input_snapshot_and_preserves_sequential_samples` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:12` | - |  |
| `smoothstep` | 3 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:54` | - |  |
| `smulwb` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:9` | - |  |
| `smulww` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:17` | - |  |
| `impl GridDivision::snap` | 5 | `crates/audio-timebase/src/grid.rs:144` | - | Snap `pos` to the nearest grid line. |
| `impl FrameRate::snap` | 2 | `crates/timeline-editing/src/timebase.rs:186` | - | Snap down to a frame boundary. |
| `snap0` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:18` | - |  |
| `snap_keeps_exotic_frame_indices_wide_until_the_tick_endpoint` | 0 | `crates/timeline-editing/src/timebase.rs:261` | - |  |
| `snap_point` | 3 | `crates/vector-locus-geometry/src/snap.rs:32` | - | Snap a point `p` against a collection of candidate points within tolerance `tol`. |
| `snap_point_candidates` | 0 | `crates/vector-locus-geometry/src/snap.rs:67` | - |  |
| `snap_point_to_grid` | 2 | `crates/vector-locus-geometry/src/snap.rs:14` | - | Snap a 2D point to an orthogonal grid with uniform spacing. |
| `snap_to_grid` | 3 | `crates/vector-locus-geometry/src/snap.rs:6` | - | Snap a value to a grid with the given spacing and origin. |
| `snap_to_lines` | 2 | `crates/vector-radiance-trace/src/fit.rs:188` | - | Retract the handles of curves that stay within `tol` of their chord. |
| `impl SpatialGraph::snap_to_nearest_node` | 3 | `crates/geodesy-spatial-analysis/src/network_topology.rs:66` | graph |  |
| `impl Unit::snap_to_ruler_tick` | 3 | `crates/vector-locus-geometry/src/units.rs:87` | - | Snap `v` (points) to the nearest ruler tick at `zoom`. |
| `sniff` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:19` | - | Sniff source format: JSON openers -> CSL-JSON, `TY  -` tagged lines -> RIS. |
| `sniff` | 1 | `crates/media-ogg/src/lib.rs:91` | - | The capture pattern at the start of a file. |
| `sniff_delimiter` | 1 | `crates/chronicle-office-codecs/src/spreadsheet/csv.rs:23` | - | Sniff the most likely delimiter based on counts in the first line. |
| `snow_accumulates_above_snowline_and_slips` | 0 | `crates/terrain-erosion-solvers/src/snow_accumulation.rs:161` | - |  |
| `snr` | 2 | `crates/audio-aac-codec/tests/common/mod.rs:217` | - | SNR in dB of `dec` against `refr`. |
| `sobel_edge_rgba8` | 3 | `crates/raster-spatial-operators/src/convolve.rs:330` | graph | Sobel gradient magnitude edge detection for RGBA8 buffer. |
| `soft_light` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:56` | 3d |  |
| `soft_light_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:324` | - | W3C/PDF Soft Light channel blend function. |
| `soft_light_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:383` | - |  |
| `solid` | 3 | `crates/radiance-color-grading/src/scopes/tests.rs:17` | - |  |
| `solid` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:639` | - |  |
| `solid_frame` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:839` | - |  |
| `solid_rgba` | 7 | `crates/asset-production-vibe/src/operations_catalog.rs:1018` | - |  |
| `impl Matrix::solve` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:294` | numerics |  |
| `solve` | 2 | `crates/drafting-geometry/src/spline.rs:274` | numerics | Gaussian elimination with partial pivoting. |
| `impl CholeskyFactor::solve` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/cholesky.rs:45` | numerics | Solve A * x = b via forward substitution (L * y = b) and backward substitution (L^T * x = y). |
| `impl LdltFactor::solve` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/ldlt.rs:86` | numerics | Solve A * x = b via L * z = b, D * y = z, L^T * x = y. |
| `solve` | 6 | `crates/locus-parametric-solver/src/solver/lm.rs:153` | numerics | Run the damped Levenberg-Marquardt solver from `x0`. Only `free` parameters change. |
| `solve_8x8` | 2 | `crates/kinetics-motion-compositor/src/track/homography.rs:62` | numerics |  |
| `solve_autolayout` | 3 | `crates/layout-box-autolayout/src/solver.rs:13` | numerics | Solves auto-layout on a frame with the given available dimensions. |
| `impl AssetHost<'_>::solve_box_tree` | 2 | `crates/asset-production-vibe/src/layout_ops/box_theme.rs:87` | numerics | Solves deterministic 2D spatial flex or CSS-grid-like layouts across viewports. |
| `impl AssetHost<'_>::solve_continuum_fem_lbm` | 2 | `crates/asset-production-vibe/src/kinetics_fem_garment_blocks.rs:9` | numerics | 135: `hypermedia.kinetics.solve_continuum_fem_lbm` (`kinetics-continuum-multiphysics`) |
| `solve_corner_pin_homography` | 2 | `crates/kinetics-motion-compositor/src/track/homography.rs:34` | numerics | Solve for the 3x3 homography mapping 4 source points to 4 destination points using DLT. |
| `solve_dense` | 3 | `crates/locus-parametric-solver/src/solver/lm.rs:60` | numerics | Solve the dense system `a y = b` (n x n, row-major) by Gaussian elimination with partial pivoting. |
| `solve_flex` | 3 | `crates/layout-box-autolayout/src/flex.rs:67` | numerics |  |
| `impl AssetHost<'_>::solve_flexbox_grid_constraints` | 2 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:30` | numerics | 132: `hypermedia.layout.solve_flexbox_grid_constraints` (`layout-box-autolayout`) |
| `solve_grid` | 3 | `crates/layout-box-autolayout/src/grid.rs:168` | numerics |  |
| `solve_linear_system` | 3 | `crates/geodesy-spatial-analysis/src/kriging.rs:252` | numerics | In-place linear solver for `Ax = b` using Gauss-Jordan elimination with partial pivoting. |
| `impl AssetHost<'_>::solve_parametric_sketch` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:230` | numerics | 120: `hypermedia.locus.solve_parametric_sketch` (`locus-parametric-solver` + `drafting-geometry`) |
| `solve_rate` | 2 | `crates/chronicle-tabular-engine/src/functions/financial.rs:151` | numerics |  |
| `solve_system` | 4 | `crates/locus-parametric-solver/src/solver/conflict.rs:109` | numerics | Solve the constraint system on the given model entities. |
| `impl AssetHost<'_>::solve_two_bone_ik` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:22` | numerics |  |
| `solve_u` | 5 | `crates/kinetics-motion-compositor/src/keyframe/bezier.rs:16` | numerics | Solve `bez(x0, x1, x2, x3, u) = x` for parameter `u` in [0, 1] using Newton-Raphson with bisection fallback. |
| `impl AssetHost<'_>::sonify_emf_field` | 5 | `crates/asset-production-vibe/src/sound.rs:16` | - | Field-derived mono PCM: irradiance envelope in sealed chunk/cell order (no HashMap). |
| `sort` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:159` | - |  |
| `sort_compare` | 3 | `crates/chronicle-tabular-engine/src/core/value.rs:254` | - |  |
| `impl AssetHost<'_>::sound` | 3 | `crates/asset-production-vibe/src/sound.rs:90` | - |  |
| `source_border_zero_radius_and_mask_length_refusal_are_preserved` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:28` | - |  |
| `source_hash` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:217` | - |  |
| `impl SGlyph::space_glyph` | 1 | `crates/vector-glyph-shaping/src/composer.rs:37` | - |  |
| `span` | 2 | `crates/vector-compute-shaders/shader/fine.wgsl:113` | - |  |
| `span` | 2 | `crates/vector-compute-shaders/shader/path_count.wgsl:36` | - |  |
| `span` | 2 | `crates/vector-compute-shaders/shader/path_tiling.wgsl:29` | - |  |
| `span` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:196` | - |  |
| `span_alignment_preserves_center_start_end_custom_and_odd_frame_rules` | 0 | `crates/timeline-editing/src/transition.rs:289` | - |  |
| `impl SliceDecoder<'a>::spatial_direct_base` | 1 | `crates/video-h264-codec/src/slicedec.rs:575` | - | Spatial direct: MB-level reference indices, predicted mvs and directZeroPrediction (8.4.1.2.2). |
| `impl AssetHost<'_>::spatialize_pga_zone` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:339` | - |  |
| `spawn_init` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:30` | - | Drive the wgpu init future. On wasm32 we spawn it onto the JS microtask |
| `spawn_init` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:35` | - |  |
| `spawn_init` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:28` | - | Drive the wgpu init future. On wasm32 we spawn it onto the JS microtask |
| `spawn_init` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:33` | - |  |
| `species_model_id` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:86` | - | The model id for rule packs: the spec's `species_id`, otherwise `fallback`. |
| `specific_properties` | 0 | `crates/media-isobmff/tests/oracle_demux.rs:247` | - |  |
| `spectral_roundtrip_every_book` | 0 | `crates/audio-aac-codec/src/huffman.rs:263` | - |  |
| `spectral_wavelength_to_rgb` | 1 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:87` | - | CIE 1931 / Bruton piecewise spectral-to-sRGB mapping for wavelength in nanometers. |
| `spectral_wavelength_to_rgb` | 1 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:692` | numerics | CIE 1931 / Bruton piecewise spectral-to-sRGB mapping for wavelength in nanometers. |
| `speech` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:50` | - | Speech-like: glottal pulse train (gliding pitch) through three resonant formant filters with a |
| `sphere` | 4 | `crates/profile-authoring/src/geometry.rs:201` | computational geometry |  |
| `trait Host::spill_range` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:42` | - |  |
| `spiral` | 5 | `crates/vector-locus-geometry/src/shapes.rs:179` | - | Archimedean-style spiral (decay-based, `segments` quarter turns). |
| `spiral_has_segments` | 0 | `crates/vector-locus-geometry/src/shapes.rs:329` | - |  |
| `splice_face_cycles` | 4 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:66` | graph | Join two face cycles at the shared oriented edge (v0, v1). |
| `spline_interpolates_fit_points` | 0 | `crates/drafting-geometry/src/tests.rs:135` | - |  |
| `split` | 4 | `crates/audio-opus-codec/src/silk/mod.rs:564` | - |  |
| `split` | 4 | `crates/kinetics-garment-draping/src/pattern.rs:19` | - |  |
| `impl Meter::split` | 1 | `crates/manifold-bim-csg/src/csg.rs:73` | - |  |
| `split` | 4 | `crates/vector-locus-pathops/src/three_d/visibility.rs:67` | - |  |
| `split_adts` | 1 | `crates/audio-aac-codec/src/config.rs:352` | - | Split an ADTS stream into `(header, raw payload)` frames. Stops at the first malformed or |
| `split_csv_row` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:215` | - |  |
| `split_endpoint` | 1 | `crates/kinetics-block-simulation/src/document.rs:26` | - |  |
| `split_into_grid` | 4 | `crates/vector-locus-pathops/src/pathops/edit.rs:387` | 2d | Split a rectangular boundary into a regular grid of paths. |
| `split_into_grid_generates_cells` | 0 | `crates/vector-locus-pathops/src/pathops/edit.rs:433` | - |  |
| `split_paragraphs` | 1 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:94` | - | `\r\n` is one paragraph break; `\n` and `\r` each start a new paragraph. |
| `split_rect` | 1 | `crates/vector-hybrid-renderer/src/rect.rs:50` | 2d |  |
| `splits_when_needed` | 0 | `crates/vector-locus-pathops/src/pathops/fit.rs:227` | - |  |
| `splitter_handles_large_rect_with_only_vertical_aa` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:299` | 2d |  |
| `splitter_keeps_large_aligned_rect_as_single_main_rect` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:316` | 2d |  |
| `splitter_keeps_multi_pixel_height_rect_within_one_pixel_width` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:248` | 2d |  |
| `splitter_keeps_multi_pixel_width_rect_within_one_pixel_height` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:231` | 2d |  |
| `splitter_keeps_small_rect_whole` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:163` | 2d |  |
| `splitter_keeps_subpixel_rect_inside_one_pixel` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:180` | 2d |  |
| `splitter_keeps_subpixel_rect_spanning_two_pixels_in_height` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:214` | 2d |  |
| `splitter_keeps_subpixel_rect_spanning_two_pixels_in_width` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:197` | 2d |  |
| `splitter_omits_unneeded_edge_parts` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:282` | graph |  |
| `splitter_splits_large_rect_into_five_parts` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:265` | 2d |  |
| `impl HevcConfig::sps` | 1 | `crates/media-isobmff/src/codec.rs:418` | - |  |
| `sps` | 4 | `crates/video-h264-codec/src/synth_tests.rs:30` | - |  |
| `sps` | 0 | `crates/video-h264-preview/src/lib.rs:485` | - |  |
| `sps` | 2 | `crates/video-h264-preview/src/synth.rs:26` | - |  |
| `sps_crop_offsets_refuse_overflow_without_panicking_and_keep_valid_crop` | 0 | `crates/video-h264-codec/src/params.rs:737` | - |  |
| `sps_for_params` | 2 | `crates/video-h264-preview/src/lib.rs:487` | - |  |
| `sps_for_width` | 1 | `crates/video-h264-preview/src/lib.rs:486` | - |  |
| `sqrt` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:57` | - |  |
| `trait Scalar::sqrt` | 1 | `crates/kinetics-garment-draping/src/math.rs:242` | - |  |
| `impl Scalar for f64::sqrt` | 1 | `crates/kinetics-garment-draping/src/math.rs:256` | - |  |
| `impl Scalar for Jet<N>::sqrt` | 1 | `crates/kinetics-garment-draping/src/math.rs:304` | - |  |
| `sqrt_approx` | 1 | `crates/audio-opus-codec/src/silk/lpc.rs:81` | - |  |
| `square` | 3 | `crates/drafting-geometry/src/region.rs:379` | 2d |  |
| `impl MorphologyKernel::square` | 1 | `crates/raster-selection-morphology/src/grayscale.rs:35` | - | Square structuring element of width `2 * radius + 1`. |
| `square` | 2 | `crates/raster-selection-morphology/src/tests.rs:6` | - |  |
| `square` | 0 | `crates/vector-locus-geometry/src/path.rs:514` | - |  |
| `square_bounds_and_area` | 0 | `crates/vector-locus-geometry/src/path.rs:532` | - |  |
| `square_ring` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:1247` | - |  |
| `square_ring` | 2 | `crates/asset-production-vibe/src/operations_catalog.rs:326` | - |  |
| `square_to` | 2 | `crates/vector-locus-geometry/src/corners.rs:118` | 2d | The unit direction square to unit `a`, on `b`'s side. |
| `impl Oscillator::square_wave` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:71` | - |  |
| `srgb` | 0 | `crates/color-transforms/src/icc/builtin.rs:83` | - |  |
| `srgb_curve` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:439` | - |  |
| `srgb_to_linear` | 1 | `crates/color-transforms/src/lib.rs:183` | numerics | sRGB electro-optical transfer (encoded â†’ linear). |
| `srgb_to_linear` | 1 | `crates/radiance-color-grading/src/matrix.rs:259` | numerics |  |
| `srgb_to_linear_f32` | 1 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:126` | numerics |  |
| `srgb_trc` | 0 | `crates/color-transforms/src/icc/curve.rs:23` | - | The sRGB transfer function (IEC 61966-2.1) as ICC parametric type 3. |
| `srgb_u8_to_linear_table` | 0 | `crates/color-transforms/src/lib.rs:236` | numerics | 256-entry table: sRGB-encoded u8 â†’ linear f32. |
| `srgb_u8_to_linear_table` | 0 | `crates/radiance-color-grading/src/matrix.rs:328` | numerics | 256-entry table: sRGB-encoded u8 -> linear f32. |
| `stabilize_enforces_spacing` | 0 | `crates/audio-opus-codec/src/silk/lpc.rs:354` | - |  |
| `impl Call::stage` | 1 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:200` | - |  |
| `stair_flight_geometry` | 0 | `crates/manifold-bim-csg/src/parametric.rs:349` | - |  |
| `stale_lease_and_wrong_cursor_do_not_enter_the_device_queue` | 0 | `crates/pcm-device-output/src/queue_tests.rs:82` | - |  |
| `stale_returns_err` | 0 | `crates/discourse-academic-tex/src/review/suggest.rs:132` | - |  |
| `standard_fabrics` | 0 | `crates/kinetics-garment-draping/src/fabrics.rs:45` | - |  |
| `standard_presets` | 0 | `crates/surface-pbr-studio/src/material.rs:170` | - | Catalog of standard calibrated PBR materials. |
| `impl ChartOfAccounts::standard_template` | 0 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:99` | - | Sets up a standard standard chart of accounts: |
| `star` | 5 | `crates/vector-locus-geometry/src/shapes.rs:139` | - | Star with `points` tips (≥ 2), outer radius `r1`, inner radius `r2`, first tip up. |
| `star_alternates_radii` | 0 | `crates/vector-locus-geometry/src/shapes.rs:315` | - |  |
| `impl Segment::start` | 1 | `crates/drafting-geometry/src/curve.rs:318` | 2d |  |
| `impl BoxBuf::start` | 2 | `crates/media-isobmff/src/bytes.rs:259` | - | Begin a box; returns a marker to pass to [`end`](Self::end). |
| `impl SchedulePoint::start` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:394` | - |  |
| `start_for_alignment` | 6 | `crates/timeline-editing/src/transition.rs:176` | - | Compute a new start when a user changes the typed duration/alignment. The |
| `impl BoxBuf::start_full` | 4 | `crates/media-isobmff/src/bytes.rs:265` | - |  |
| `impl SliceDecoder<'a>::start_mb` | 2 | `crates/video-h264-codec/src/slicedec.rs:377` | - | Set up the current macroblock address and neighbour availability. |
| `impl Decoder::start_picture` | 4 | `crates/video-h264-codec/src/decoder.rs:403` | - |  |
| `impl Arc::start_point` | 1 | `crates/drafting-geometry/src/curve.rs:71` | 2d |  |
| `start_tangent` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:21` | 2d | Direction the curve leaves `p0` (robust to retracted handles). |
| `start_tangent` | 1 | `crates/vector-radiance-trace/src/fit.rs:407` | 2d |  |
| `impl Decoder::stat` | 2 | `crates/video-h264-codec/src/decoder.rs:273` | - |  |
| `impl Parser::statement` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:46` | - |  |
| `impl VibeNamespaceProvider for HypermediaOntologyProvider::static_ontology_nquins` | 1 | `crates/asset-production-vibe/src/ontology/vibe_descriptors.rs:263` | semantics |  |
| `impl Encoder::stats` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:256` | - |  |
| `impl Decoder::stats` | 1 | `crates/video-h264-codec/src/decoder.rs:269` | - | Statistics accumulated so far (macroblock counts include finished pictures only). |
| `impl DeviceOutput::status` | 1 | `crates/pcm-device-output/src/device.rs:218` | - |  |
| `std_norm_cdf` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:33` | 3d | Standard normal CDF: Phi(x) |
| `std_norm_inv` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:38` | 3d | Inverse standard normal CDF (Acklam approximation) |
| `std_norm_pdf` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:12` | 3d | Standard normal PDF: phi(x) |
| `stdev_s` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:295` | - |  |
| `impl UniformLinearArray::steering_vector` | 2 | `crates/electromagnetics-rf-sensing/src/beamforming.rs:26` | - | Computes steering vector $a(\theta)$ for incidence angle $\theta$ (degrees, -90..+90). |
| `impl ExplicitIntegrator::step` | 3 | `crates/kinetics-continuum-multiphysics/src/explicit/dynamics.rs:47` | - | Advance one time step using central differences: |
| `impl LbmGrid3D::step` | 1 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:181` | - | Step simulation forward one time increment. |
| `step` | 1 | `crates/vector-radiance-trace/src/contour.rs:41` | - |  |
| `step_decode_escapes` | 0 | `crates/manifold-bim-csg/src/step.rs:418` | - |  |
| `impl AssetHost<'_>::step_island_xpbd` | 2 | `crates/asset-production-vibe/src/kinetics_sim/motor_xpbd_space.rs:78` | - | ID-sorted deterministic Island-Parallel XPBD rigid-body simulation. |
| `step_layout` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:738` | - |  |
| `impl FilterPassPlan::step_mut` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:476` | - |  |
| `step_real` | 1 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:657` | - |  |
| `impl GridDivision::step_samples` | 5 | `crates/audio-timebase/src/grid.rs:122` | - | Step size in samples at position `at`. |
| `impl AssetHost<'_>::step_sector_continuum_3d` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rig_ontology_continuum_frustum.rs:265` | - | Runs the 3D XPBD + analytical SDF + 2-way continuum bridge + 4-wheel rover suspension |
| `impl AssetHost<'_>::step_simulation_zone` | 2 | `crates/asset-production-vibe/src/kinetics_sim/nav_nla_zones.rs:307` | - | Double-buffered Simulation / Repeat Zone iterative divergence-free curl flow. |
| `step_string` | 1 | `crates/manifold-bim-csg/src/ifc.rs:9` | - | Formats a string as an ISO 10303-21 STEP string literal with Unicode escapes. |
| `steps` | 4 | `crates/radiance-color-grading/src/scopes/mod.rs:171` | - |  |
| `impl FilterPassPlan::steps` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:463` | - |  |
| `stereo_decode_pred` | 1 | `crates/audio-opus-codec/src/silk/mod.rs:595` | - |  |
| `stereo_matrix_44k_48k` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:133` | numerics |  |
| `stereo_merge` | 3 | `crates/audio-opus-codec/src/celt/bands.rs:637` | - |  |
| `stereo_ms_to_lr` | 6 | `crates/audio-opus-codec/src/silk/mod.rs:615` | - |  |
| `stereo_pan_gains` | 2 | `crates/acoustics-daw-session/src/dsp/pan.rs:17` | - | Computes left and right channel gains for a given pan position (-1.0 hard left .. +1.0 hard right). |
| `still_has_nontrivial_pixel` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:680` | 2d |  |
| `stitch_panorama` | 5 | `crates/radiance-camera-raw/src/fusion.rs:109` | 3d | Multi-band Burt-Adelson pyramid blending for multi-image panorama stitching. |
| `stitch_segments_into_lines` | 1 | `crates/geodesy-spatial-analysis/src/morphometry.rs:346` | - |  |
| `impl DeviceOutput::stop` | 1 | `crates/pcm-device-output/src/device.rs:196` | - |  |
| `impl Stream::stop_and_reset` | 2 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:92` | - | Stop the WASAPI client and reset its pending stream buffer on the |
| `stop_and_reset` | 1 | `crates/pcm-device-output/src/device.rs:282` | - |  |
| `stop_and_reset` | 1 | `crates/pcm-device-output/src/device.rs:290` | - |  |
| `stop_failure_never_resets_and_reset_failure_remains_stopped` | 0 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:269` | - |  |
| `stop_then_reset` | 3 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:212` | - |  |
| `impl RangeDecoder<'a>::storage` | 1 | `crates/audio-opus-codec/src/range.rs:63` | - | Size of the buffer in bytes as seen by the decoder. |
| `storage_reuse` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:858` | - |  |
| `store` | 8 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:520` | - |  |
| `store` | 0 | `crates/vector-glyph-shaping/src/hyphen.rs:24` | - |  |
| `impl AssetHost<'_>::store_layout_rgba` | 4 | `crates/asset-production-vibe/src/layout_ops/sheet_render.rs:486` | - |  |
| `impl AssetHost<'_>::store_locus_profile` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:228` | - |  |
| `impl AssetHost<'_>::store_mesh` | 3 | `crates/asset-production-vibe/src/geometry.rs:319` | computational geometry |  |
| `impl AssetHost<'_>::store_mesh_artifact` | 3 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:102` | computational geometry | Store a new `Mesh` artifact after bounding coordinates and computing bounding box. |
| `impl AssetHost<'_>::store_mesh_positions` | 3 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:197` | computational geometry | Stores a 3D Mesh artifact validating budgets and finite bounds. |
| `impl Dpb<F>::store_picture` | 8 | `crates/video-h264-codec/src/dpb.rs:332` | - | Reference marking for the just-decoded picture and insertion into the DPB with output bumping. |
| `impl AssetHost<'_>::store_profile` | 2 | `crates/asset-production-vibe/src/geometry.rs:294` | - |  |
| `impl SliceDecoder<'a>::store_qp` | 1 | `crates/video-h264-codec/src/slicedec.rs:1099` | - |  |
| `store_quad_mesh` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:387` | computational geometry |  |
| `store_quad_sheet` | 1 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:209` | - |  |
| `store_raw_mesh` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/botanical_vertebrate_10d.rs:459` | computational geometry |  |
| `impl AssetHost<'_>::store_rgba` | 4 | `crates/asset-production-vibe/src/graphics.rs:90` | - |  |
| `store_test_cube` | 1 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:375` | - |  |
| `store_test_mesh` | 1 | `crates/asset-production-vibe/src/locus_generative/mod.rs:64` | computational geometry |  |
| `store_test_profile` | 2 | `crates/asset-production-vibe/src/locus_generative/mod.rs:58` | - |  |
| `store_tetrahedron` | 1 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:197` | - |  |
| `store_tetrahedron_mesh` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:410` | computational geometry |  |
| `store_u16be_as_u8` | 2 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:27` | - |  |
| `stored_code` | 1 | `crates/layout-paged-typesetter/src/citations/zotero_fields.rs:19` | - |  |
| `story` | 1 | `crates/vector-glyph-shaping/src/thread.rs:32` | - | Join all frame slices back into the full story text. |
| `impl WallAxis::straight` | 2 | `crates/manifold-bim-csg/src/parametric.rs:62` | - |  |
| `straight_detection` | 0 | `crates/vector-locus-pathops/src/pathops/fit.rs:237` | - |  |
| `straight_line_depreciation` | 0 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:169` | - |  |
| `straight_star_five_points` | 0 | `crates/vector-locus-geometry/src/shapes.rs:323` | - |  |
| `straight_star_inner_radius` | 2 | `crates/vector-locus-geometry/src/shapes.rs:153` | - | Inner radius of a "straight-armed" star (shoulders aligned). |
| `straight_track_frames` | 0 | `crates/vector-surface-brush/src/track.rs:236` | - |  |
| `stray_end_deletes_line` | 0 | `crates/discourse-academic-tex/src/review/ai_repair.rs:284` | - |  |
| `stray_end_edit` | 4 | `crates/discourse-academic-tex/src/review/ai_repair.rs:138` | - |  |
| `stream` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:413` | - |  |
| `impl LbmGrid3D::stream` | 1 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:151` | - | Stream step with half-way bounce-back on solid boundaries. |
| `stream_config_from_asbd` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:426` | - |  |
| `stream_delivers_more_than_32_frames_and_sink_can_stop` | 0 | `crates/video-h264-preview/src/lib.rs:680` | - |  |
| `stream_instant` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:18` | - |  |
| `stream_instant` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:556` | - | Use the stream's `IAudioClock` to produce the current stream instant. |
| `impl OggFile::stream_of` | 2 | `crates/media-ogg/src/lib.rs:360` | - | The first stream of `codec`. |
| `stream_packets` | 1 | `crates/media-isobmff/tests/oracle_mux.rs:86` | - |  |
| `stream_setup_for` | 0 | `crates/audio-device-cpal/examples/synth_tones.rs:93` | - |  |
| `stream_timestamp` | 2 | `crates/audio-device-cpal/src/host/alsa/mod.rs:865` | - |  |
| `streams` | 1 | `crates/media-isobmff/tests/common/mod.rs:162` | - |  |
| `string` | 3 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:29` | - |  |
| `string` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:62` | - |  |
| `string` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:146` | - | String / UTF-8 value: bytes up to the first NUL, lossily decoded. |
| `impl Set::string` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:101` | - |  |
| `string` | 3 | `crates/kinetics-block-simulation/src/blocks/params.rs:45` | - |  |
| `impl Parser<'_>::string` | 1 | `crates/manifold-bim-csg/src/step.rs:223` | - |  |
| `strings_and_concat` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:446` | - |  |
| `strip_emf_bao_keep_crc` | 1 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:971` | - |  |
| `strip_occupancy_bao_keep_dir_crc` | 1 | `crates/asset-production-vibe/src/geodesy_ops.rs:1095` | - |  |
| `strip_parity` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:16` | - | Verify and strip odd parity bit from a byte. |
| `impl WebGlRendererContext<'_>::strip_pass_inner` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2750` | - | Render strips to the specified render target. |
| `impl RendererContext<'_>::strip_pass_inner` | 6 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2668` | - | Render the strips to the specified render target. |
| `strip_xs` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:668` | - |  |
| `stripes_give_one_path_per_stripe` | 0 | `crates/vector-radiance-trace/src/tests.rs:75` | - |  |
| `impl OpaqueDraw::strips` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:102` | - |  |
| `impl SubpathBuilder<'a>::stroke` | 2 | `crates/asset-production-vibe/src/glyph/mod.rs:157` | - |  |
| `impl Scene::stroke` | 6 | `crates/vector-compute-renderer/src/scene.rs:327` | - | Strokes a shape using the specified style and brush. |
| `impl PaintScene for VelloCpuScenePainter::stroke` | 6 | `crates/vector-cpu-render-adapter/src/scene.rs:116` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::stroke` | 6 | `crates/vector-gpu-render-adapter/src/scene.rs:90` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::stroke` | 6 | `crates/vector-hybrid-render-adapter/src/scene.rs:239` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::stroke` | 6 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:151` | - |  |
| `impl Scene::stroke` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:759` | - | Get the current stroke. |
| `impl PaintScene for SkiaScenePainter<'_>::stroke` | 6 | `crates/vector-skia-render-adapter/src/scene.rs:480` | - |  |
| `stroke_bez` | 5 | `crates/vector-locus-pathops/src/pathops/offset.rs:36` | - |  |
| `stroke_closed` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:479` | 2d |  |
| `stroke_contains` | 4 | `crates/vector-locus-geometry/src/hit.rs:62` | - | Is `p` within `tol` of the path's stroke of width `width`? |
| `impl GlyphRunBackend<'a> for GpuGlyphRunBackend<'a>::stroke_glyphs` | 3 | `crates/vector-hybrid-renderer/src/text.rs:276` | - |  |
| `impl Scene::stroke_gpu_inner` | 4 | `crates/vector-compute-renderer/src/scene.rs:382` | - | Encodes the stroke of a shape using the specified style. The stroke style must have |
| `stroke_hit` | 0 | `crates/vector-locus-geometry/src/hit.rs:106` | - |  |
| `impl Scene::stroke_mut` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:765` | - | Get a mutable reference to the current stroke. |
| `stroke_open` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:368` | 2d |  |
| `impl Scene::stroke_path` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:395` | - | Stroke a path with the current paint and stroke settings. |
| `impl glifo::GlyphRenderer for Scene::stroke_path` | 2 | `crates/vector-hybrid-renderer/src/text.rs:329` | - |  |
| `impl Scene::stroke_path_with` | 5 | `crates/vector-hybrid-renderer/src/scene.rs:414` | - | Build strips for a stroked path with the given properties and record the draw. |
| `impl Scene::stroke_rect` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:538` | 2d | Stroke a rectangle with the current paint and stroke settings. |
| `stroke_region` | 1 | `crates/vector-locus-pathops/src/pathops/offset.rs:65` | - | Area covered by multiple stroke outlines as a single clean compound path. |
| `stroke_sample_is_64_bytes` | 0 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:373` | - |  |
| `stroke_sample_is_64_bytes` | 0 | `crates/terrain-brushes/src/lib.rs:539` | - |  |
| `stts_cursor` | 0 | `crates/media-isobmff/src/demux.rs:999` | - |  |
| `impl StreamInstant::sub` | 2 | `crates/audio-device-cpal/src/lib.rs:446` | - | Returns the instant in time one `duration` ago. |
| `impl std::ops::Sub for Tick::sub` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:60` | - |  |
| `impl Sub for Vec2::sub` | 2 | `crates/drafting-geometry/src/vec.rs:114` | 2d |  |
| `impl Sub for Vec3::sub` | 2 | `crates/drafting-geometry/src/vec.rs:187` | 3d |  |
| `impl Sub for DVec2::sub` | 2 | `crates/kinetics-garment-draping/src/math.rs:67` | - |  |
| `impl Sub for DVec3::sub` | 2 | `crates/kinetics-garment-draping/src/math.rs:164` | - |  |
| `impl Sub for Jet<N>::sub` | 2 | `crates/kinetics-garment-draping/src/math.rs:334` | - |  |
| `sub` | 2 | `crates/kinetics-garment-draping/src/math.rs:371` | - |  |
| `impl std::ops::Sub for Tick::sub` | 2 | `crates/kinetics-motion-compositor/src/time.rs:48` | - |  |
| `sub` | 2 | `crates/manifold-bim-csg/src/csg.rs:15` | - |  |
| `impl Sub for Vec2::sub` | 2 | `crates/surface-pbr-studio/src/math.rs:63` | - |  |
| `impl Sub for Vec3::sub` | 2 | `crates/surface-pbr-studio/src/math.rs:201` | - |  |
| `impl std::ops::Sub for Vec2::sub` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:30` | - |  |
| `impl V3::sub` | 2 | `crates/vector-locus-pathops/src/three_d/revolve.rs:15` | - |  |
| `sub_2d` | 2 | `crates/manifold-bim-csg/src/parametric.rs:21` | - |  |
| `impl SubAssign for Vec2::sub_assign` | 2 | `crates/drafting-geometry/src/vec.rs:119` | 2d |  |
| `impl SubAssign for DVec3::sub_assign` | 2 | `crates/kinetics-garment-draping/src/math.rs:171` | - |  |
| `impl Encoder::subblock_energy` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:318` | - |  |
| `impl Decoder::submit_pending` | 1 | `crates/video-h264-codec/src/decoder.rs:467` | - | Start decoding the pending picture and do its reference marking / DPB insertion. |
| `subpath_segs` | 1 | `crates/vector-locus-pathops/src/pathops/edit.rs:52` | - |  |
| `subpath_segs` | 1 | `crates/vector-radiance-trace/src/fit.rs:288` | - |  |
| `subpaths` | 1 | `crates/vector-locus-geometry/src/bez.rs:10` | - | Element ranges of the subpaths of `bp` (each with whether it is closed). |
| `subpaths_and_tangents` | 0 | `crates/vector-locus-geometry/src/bez.rs:77` | - |  |
| `subtract` | 3 | `crates/manifold-bim-csg/src/csg.rs:336` | - | Subtracts `void` solid from `solid`. |
| `subtract` | 4 | `crates/vector-locus-pathops/src/three_d/visibility.rs:89` | computational geometry |  |
| `impl Unit::suffix` | 1 | `crates/vector-locus-geometry/src/units.rs:49` | - |  |
| `suggested_package` | 2 | `crates/discourse-academic-tex/src/review/logrules.rs:188` | - |  |
| `sum` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:6` | - |  |
| `impl Parser<'_>::sum` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:74` | - |  |
| `summarize` | 1 | `crates/discourse-pdf-compliance/src/a11y/report.rs:18` | - |  |
| `impl ResampleFilter::support` | 1 | `crates/raster-spatial-operators/src/resample.rs:25` | - | Spatial support radius in pixels. |
| `impl Nib::support` | 2 | `crates/vector-surface-brush/src/calligraphic.rs:157` | 2d |  |
| `impl Device::supported_configs` | 2 | `crates/audio-device-cpal/src/host/alsa/mod.rs:305` | - |  |
| `impl Device::supported_configs` | 2 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:218` | - |  |
| `impl Device::supported_configs` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:111` | - |  |
| `impl Device::supported_formats` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:379` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:68` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:441` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:60` | - | Gets the supported input configs. |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:65` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:83` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:149` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:93` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:320` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:73` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:147` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:149` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:49` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:282` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:61` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:466` | - |  |
| `impl Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:89` | - |  |
| `impl DeviceTrait for Device::supported_input_configs` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:147` | - |  |
| `trait DeviceTrait::supported_input_configs` | 1 | `crates/audio-device-cpal/src/traits.rs:107` | - | An iterator yielding formats that are supported by the backend. |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:74` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:447` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/asio/device.rs:97` | - | Gets the supported output configs. |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/asio/mod.rs:71` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:102` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/ios/mod.rs:156` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:99` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/coreaudio/macos/mod.rs:326` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:80` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:153` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/jack/device.rs:155` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/null/mod.rs:56` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/oboe/mod.rs:292` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:67` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/wasapi/device.rs:477` | - |  |
| `impl Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:97` | - |  |
| `impl DeviceTrait for Device::supported_output_configs` | 1 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:154` | - |  |
| `trait DeviceTrait::supported_output_configs` | 1 | `crates/audio-device-cpal/src/traits.rs:114` | - | An iterator yielding output stream formats that are supported by the device. |
| `supports_input` | 1 | `crates/audio-device-cpal/src/traits.rs:62` | - |  |
| `supports_output` | 1 | `crates/audio-device-cpal/src/traits.rs:76` | - |  |
| `impl ModelInfo::supports_reference_mode` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:163` | - |  |
| `impl AssetHost<'_>::surface` | 2 | `crates/asset-production-vibe/src/graphics.rs:386` | - |  |
| `impl Series::surface` | 3 | `crates/discourse-scientific-matlang/src/plot.rs:107` | - |  |
| `impl AssetHost<'_>::surface_bsdf_uv_op` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:340` | - | Categorical operations dispatch for surface BSDF, UV, and procedural field synthesis. |
| `surface_gradient` | 5 | `crates/geodesy-spatial-analysis/src/morphometry.rs:67` | - | Computes the surface gradient $(dz/dx, dz/dy)$ at a cell in metres per metre. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::surface_texture_format` | 2 | `crates/pixel-buffer-output/src/builder.rs:187` | 2d | Set the surface texture format. |
| `impl Pixels<'win>::surface_texture_format` | 1 | `crates/pixel-buffer-output/src/lib.rs:765` | - | Get the surface texture format. |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::suspend` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:191` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::suspend` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:422` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::suspend` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:446` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::suspend` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:152` | - |  |
| `svd` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:596` | 3d, numerics | SVD via normal equations A'A: A = U S V'. |
| `svg_bytes` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:648` | - |  |
| `svg_gantt_renders` | 0 | `crates/kinetics-schedule-cpm/src/format.rs:795` | - |  |
| `svg_lum` | 1 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:78` | 3d |  |
| `svg_num` | 1 | `crates/asset-production-vibe/src/locus_generative/mod.rs:326` | - |  |
| `svg_num` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:27` | - | Fixed-point SVG number; kills signed zero so stills are bit-stable. |
| `svg_path_d` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:656` | - |  |
| `svg_text_lines` | 1 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:402` | - |  |
| `impl References::swap_frames` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:174` | - |  |
| `impl IcsInfo::swb_offsets` | 2 | `crates/audio-aac-codec/src/ics.rs:57` | - | Band offsets of one window. |
| `swb_offsets_long` | 1 | `crates/audio-aac-codec/src/tables.rs:66` | - | Scalefactor band offsets (`swb_offset_long_window`, 1024-line frames); `len() == num_swb + 1`. |
| `swb_offsets_short` | 1 | `crates/audio-aac-codec/src/tables.rs:79` | - | Scalefactor band offsets for one short window (128 lines). |
| `sweep` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:30` | - | Logarithmic sine sweep 40 Hz → 16 kHz. |
| `impl Arc::sweep` | 1 | `crates/drafting-geometry/src/curve.rs:68` | - |  |
| `impl Ellipse::sweep` | 1 | `crates/drafting-geometry/src/curve.rs:247` | - |  |
| `sweep` | 3 | `crates/vector-surface-brush/src/calligraphic.rs:199` | - |  |
| `impl AssetHost<'_>::sweep_assembly_profile` | 2 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:16` | - | 123: `hypermedia.manifold.sweep_assembly_profile` |
| `sweep_sphere_aabb` | 4 | `crates/asset-production-vibe/src/kinetics_sim/motor_xpbd_space.rs:503` | computational geometry |  |
| `syd` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:396` | - |  |
| `syllable_splitting_rules` | 0 | `crates/vector-glyph-shaping/src/hyphen.rs:159` | - |  |
| `impl BinOp::symbol` | 1 | `crates/chronicle-tabular-engine/src/formula/ast.rs:119` | - |  |
| `symbol_of` | 1 | `crates/discourse-academic-tex/src/typeset.rs:59` | - | Greek + symbol command names -> their Unicode scalar. |
| `impl Padding::symmetric` | 2 | `crates/layout-box-autolayout/src/types.rs:136` | - |  |
| `impl Track::sync_sample_before` | 2 | `crates/media-isobmff/src/demux.rs:171` | - | The closest sync sample at or before `index` (decode order); 0 if there is none. |
| `impl Mp4File::sync_sample_before` | 3 | `crates/media-isobmff/src/demux.rs:236` | - |  |
| `impl Track::sync_samples` | 1 | `crates/media-isobmff/src/demux.rs:176` | - | Indices of all sync samples. |
| `impl AssetHost<'_>::sync_spatial_motors` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:322` | - |  |
| `impl AssetHost<'_>::synchronize_companion_frustum` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rig_ontology_continuum_frustum.rs:303` | 3d | Generates a multi-projector conical dome frustum cluster (`rows * cols` `ConicalDomeWedge`s) |
| `synth` | 3 | `crates/audio-aac-codec/src/decoder.rs:495` | - | Inverse MDCT, windowing and overlap-add (§4.6.18), output scaled to ±1.0 full scale. |
| `synth_cmyk` | 1 | `crates/color-transforms/src/icc/synth.rs:45` | - |  |
| `impl CeltDecoder::synthesis` | 9 | `crates/audio-opus-codec/src/celt/mod.rs:384` | - | Denormalisation + IMDCT into the decode memory (`celt_synthesis`). |
| `impl RfSkeletalPose::synthesize_standing` | 4 | `crates/electromagnetics-rf-sensing/src/densepose.rs:72` | - | Generates a normalized human skeletal pose centered at (center_x, center_depth, height_z). |
| `synthetic_av_mp4` | 0 | `crates/video-h264-preview/src/lib.rs:620` | - |  |
| `synthetic_files` | 0 | `crates/media-isobmff/tests/fuzz.rs:22` | - |  |
| `synthetic_gop` | 0 | `crates/video-h264-preview/src/lib.rs:690` | - |  |
| `synthetic_mp4` | 0 | `crates/video-h264-preview/src/lib.rs:613` | - |  |
| `synthetic_reordered` | 0 | `crates/video-h264-preview/src/lib.rs:589` | - |  |
| `system_time_to_stream_instant` | 1 | `crates/audio-device-cpal/src/host/asio/stream.rs:611` | - | Asio retrieves system time via `timeGetTime` which returns the time in milliseconds. |
| `impl Edge::t` | 2 | `crates/video-h264-codec/src/intra.rs:23` | - | p[x, -1] for x >= -1 |
| `t_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:222` | - |  |
| `table` | 2 | `crates/color-transforms/src/icc/curve.rs:137` | - |  |
| `table_checksum_calculation` | 0 | `crates/vector-glyph-shaping/src/subset.rs:143` | - |  |
| `table_index_for_rate` | 1 | `crates/audio-aac-codec/src/tables.rs:13` | - | The table index used for scalefactor-band tables when a non-standard rate is signalled explicitly |
| `tables` | 0 | `crates/video-h264-codec/src/cavlc.rs:66` | - |  |
| `tag` | 2 | `crates/radiance-color-grading/src/grade.rs:239` | - |  |
| `tag_monoid_identity` | 0 | `crates/vector-compute-shaders/shader/shared/pathtag.wgsl:44` | - |  |
| `tags_require_bounded_complete_lengths` | 0 | `crates/media-ogg/src/strict_opus.rs:144` | - |  |
| `impl DeviceHandles::take` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:232` | - | Take ownership of the `alsa::PCM` handle for a specific `stream_type`. |
| `take` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:204` | - |  |
| `impl SttsCursor<'_>::take` | 2 | `crates/media-isobmff/src/demux.rs:587` | - | Advance by `n` samples, returning the summed duration. |
| `impl Render::take_captured_buffers` | 1 | `crates/vector-compute-renderer/src/render.rs:650` | - |  |
| `impl Scene::take_current_state` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:883` | - | Take current rendering state and reset the existing state to its default. |
| `impl Decoder::take_error` | 1 | `crates/video-h264-codec/src/decoder.rs:281` | - | First error reported by a decoding job since the last call (also returned by |
| `impl GradientRampCache::take_luts` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:126` | - | Take ownership of the luts, leaving an empty vector in its place. |
| `impl RendererContext<'_>::take_root_load_op` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2615` | - |  |
| `impl Segment::tangent` | 2 | `crates/drafting-geometry/src/curve.rs:387` | 2d | Tangent direction (unit) at parameter t. |
| `impl WallAxis::tangent` | 2 | `crates/manifold-bim-csg/src/parametric.rs:97` | - |  |
| `tangent` | 2 | `crates/vector-locus-geometry/src/bez.rs:40` | 2d | Unit tangent of `s` at `t`, robust at ends where control points coincide with the end point. |
| `impl Circle::tangent_points` | 2 | `crates/drafting-geometry/src/curve.rs:182` | 2d | Tangent points from external point `p`. |
| `tap6` | 6 | `crates/video-h264-codec/src/inter.rs:44` | - |  |
| `impl FilterPassBindings::target` | 1 | `crates/vector-hybrid-renderer/src/target.rs:131` | - |  |
| `impl BlendPassBindings::target` | 1 | `crates/vector-hybrid-renderer/src/target.rs:175` | - |  |
| `impl Encoder::target_frame_bytes` | 1 | `crates/video-prores-codec/src/encode.rs:73` | - | The rate-control target for one frame in bytes (excluding alpha). |
| `impl BufferRenderer::target_texture_view` | 1 | `crates/gpu-device-context/src/buffer_renderer.rs:117` | - |  |
| `impl SurfaceRenderer<'s>::target_texture_view` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:291` | - | Get a target texture view to render to. |
| `targets` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:525` | numerics |  |
| `impl Project::task` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:568` | - |  |
| `impl Project::task_mut` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:572` | - |  |
| `tbilleq` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:546` | - |  |
| `tbillprice` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:555` | - |  |
| `tbillyield` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:564` | - |  |
| `tdac_perfect_reconstruction` | 0 | `crates/audio-aac-codec/src/mdct.rs:248` | - |  |
| `impl RangeDecoder<'a>::tell` | 1 | `crates/audio-opus-codec/src/range.rs:245` | - | Number of bits consumed so far (rounded up). |
| `impl RangeDecoder<'a>::tell_frac` | 1 | `crates/audio-opus-codec/src/range.rs:250` | - | Number of 1/8 bits consumed so far (rounded up). |
| `tell_starts_at_one_bit` | 0 | `crates/audio-opus-codec/src/range.rs:479` | - |  |
| `temp_buffer_to_data` | 2 | `crates/audio-device-cpal/src/host/jack/stream.rs:273` | - |  |
| `temp_path` | 1 | `crates/media-test-support/src/fixtures.rs:37` | - | A temporary sibling of `out` that no other thread or process will pick: the stem gets |
| `temp_paths_are_unique_and_keep_the_extension` | 0 | `crates/media-test-support/src/fixtures.rs:113` | - |  |
| `impl TempoMap::tempo_at_tick` | 2 | `crates/audio-timebase/src/tempo.rs:200` | - | Active tempo at `tick`. |
| `tempo_change_is_integrated` | 0 | `crates/audio-timebase/src/tempo.rs:384` | - |  |
| `temporal_direct_checks_both_vectors` | 0 | `crates/video-h264-codec/src/slicedec.rs:29` | - |  |
| `temporal_direct_component` | 2 | `crates/video-h264-codec/src/slicedec.rs:17` | - |  |
| `impl FilterPassBindings::temporary` | 1 | `crates/vector-hybrid-renderer/src/target.rs:135` | - |  |
| `impl TempoMap::tempos` | 1 | `crates/audio-timebase/src/tempo.rs:134` | - |  |
| `impl Enc::terminate` | 3 | `crates/video-h264-codec/src/synth_tests.rs:261` | - |  |
| `impl Enc::terminate_flush` | 1 | `crates/video-h264-codec/src/cabac.rs:276` | - |  |
| `terraces_quantize_continuous_slope` | 0 | `crates/terrain-erosion-solvers/src/fractal_terrace.rs:107` | - |  |
| `impl Arc::tessellate` | 3 | `crates/drafting-geometry/src/curve.rs:123` | 2d |  |
| `impl Circle::tessellate` | 3 | `crates/drafting-geometry/src/curve.rs:178` | 2d |  |
| `impl Ellipse::tessellate` | 3 | `crates/drafting-geometry/src/curve.rs:250` | 2d |  |
| `impl Segment::tessellate` | 3 | `crates/drafting-geometry/src/curve.rs:357` | 2d | Points along the segment in its direction, including both ends. |
| `impl Polyline::tessellate` | 2 | `crates/drafting-geometry/src/curve.rs:497` | 2d |  |
| `impl Spline::tessellate` | 2 | `crates/drafting-geometry/src/spline.rs:197` | 2d |  |
| `test_a11y_32_rules_and_report` | 0 | `crates/discourse-pdf-compliance/src/tests.rs:26` | - |  |
| `test_abr_computed_and_sampled_parsing` | 0 | `crates/surface-layered-codecs/src/lib.rs:142` | - |  |
| `test_aco_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:418` | - |  |
| `test_acoustics_bus_loudness_emf_sonic_objects` | 0 | `crates/asset-production-vibe/src/sound.rs:276` | - |  |
| `test_advect_m64_grid_divergence_reduction` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:333` | - |  |
| `test_affine_identity` | 0 | `crates/raster-spatial-operators/src/distort.rs:461` | - |  |
| `test_affinity_container_header_inspection` | 0 | `crates/surface-layered-codecs/src/lib.rs:171` | - |  |
| `test_algebraic_loop_detection` | 0 | `crates/kinetics-block-simulation/src/lib.rs:101` | - |  |
| `test_all_operators_pipeline_smoke` | 0 | `crates/raster-spatial-operators/src/lib.rs:34` | - |  |
| `test_asc_cdl_apply_and_invert` | 0 | `crates/radiance-color-grading/src/lib.rs:68` | - |  |
| `test_ase_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:438` | - |  |
| `test_audit_duplicates` | 0 | `crates/discourse-academic-tex/src/bibliography/audit.rs:164` | - |  |
| `test_autoconstrain_inference` | 0 | `crates/locus-parametric-solver/src/tests.rs:211` | - |  |
| `test_bake_position_and_normal` | 0 | `crates/surface-pbr-studio/src/lib.rs:54` | 3d |  |
| `test_barrel_distortion` | 0 | `crates/raster-spatial-operators/src/distort.rs:530` | - |  |
| `test_baseline_alignment` | 0 | `crates/layout-box-autolayout/src/tests.rs:104` | - |  |
| `test_bayer_demosaic_rggb` | 0 | `crates/radiance-camera-raw/src/lib.rs:44` | - |  |
| `test_bezier_curve_and_arclength` | 0 | `crates/kinetics-garment-draping/src/lib.rs:141` | - |  |
| `test_blend_if_ramps` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:75` | - |  |
| `test_bruton_spectral_mapping_visible_and_false_color` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:752` | - |  |
| `test_bsdf_and_brush_nontrivial_preview_pixels` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:779` | - |  |
| `test_build_light_tree` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:294` | - |  |
| `test_bvh_quad_intersection` | 0 | `crates/surface-pbr-studio/src/lib.rs:40` | computational geometry |  |
| `test_cache_empty` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:351` | - |  |
| `test_camelot_harmony_relations` | 0 | `crates/acoustics-daw-session/src/lib.rs:86` | - |  |
| `test_caption_align_and_overlap_resolution` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:201` | - |  |
| `test_capture_deterministic_movie` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:435` | - |  |
| `test_catalog_journal_undo_redo` | 0 | `crates/radiance-camera-raw/src/lib.rs:166` | - |  |
| `test_cea608_parity_and_decode` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:187` | - |  |
| `test_cmp_default_heuristics` | 0 | `crates/audio-device-cpal/src/lib.rs:744` | - |  |
| `test_coincident_and_tangent_solve` | 0 | `crates/locus-parametric-solver/src/tests.rs:98` | numerics |  |
| `test_color_wheels_apply` | 0 | `crates/radiance-color-grading/src/lib.rs:84` | - |  |
| `test_common_subexpression_elimination` | 0 | `crates/layout-nodegraph-compiler/src/tests.rs:134` | - |  |
| `test_compile_bsdf_graph_all_closures` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:600` | graph |  |
| `test_compile_bsdf_graph_ior_affects_preview` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:820` | graph |  |
| `test_compile_verify_and_render_spectral_slice_roundtrip` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:773` | - |  |
| `test_composite_subnetwork_inlining` | 0 | `crates/layout-nodegraph-compiler/src/tests.rs:74` | - |  |
| `test_compute_mikktspace_tetrahedron` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:553` | - |  |
| `test_conflict_detection_fails_closed` | 0 | `crates/locus-parametric-solver/src/tests.rs:151` | - |  |
| `test_constraints_propagation` | 0 | `crates/layout-box-autolayout/src/tests.rs:171` | - |  |
| `test_corner_pin_homography` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:109` | - |  |
| `test_correct_lru_eviction` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:444` | - |  |
| `test_couple_continuum_xpbd_physics` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:367` | physics |  |
| `test_csi_phase_unwrapping_and_sanitization` | 0 | `crates/electromagnetics-rf-sensing/src/lib.rs:32` | - |  |
| `test_css_grid_tracks_and_spans` | 0 | `crates/layout-box-autolayout/src/tests.rs:125` | - |  |
| `test_csv_tsv_codecs` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:113` | - |  |
| `test_cubic_bezier_solve_and_ease_progress` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:45` | numerics |  |
| `test_dag_recalculation_and_cycles` | 0 | `crates/chronicle-tabular-engine/tests/tabular_tests.rs:14` | - |  |
| `test_dead_node_pruning` | 0 | `crates/layout-nodegraph-compiler/src/tests.rs:185` | graph |  |
| `test_decode_cryptomatte_q42` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:164` | - |  |
| `test_dem_slope_and_contours` | 0 | `crates/geodesy-spatial-analysis/src/lib.rs:88` | - |  |
| `test_dense_linear_solver_and_rank` | 0 | `crates/locus-parametric-solver/src/tests.rs:19` | numerics |  |
| `test_develop_photo_adjustments` | 0 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:286` | - |  |
| `test_develop_photo_default_identity` | 0 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:262` | - |  |
| `test_develop_pipeline_and_hash_determinism` | 0 | `crates/radiance-camera-raw/src/lib.rs:84` | - |  |
| `test_dijkstra_and_tsp` | 0 | `crates/geodesy-spatial-analysis/src/lib.rs:116` | graph |  |
| `test_dispatch_aliases_and_errors` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:419` | - |  |
| `test_dnx_header_write_and_parse` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:194` | - |  |
| `test_dnx_idct_fdct_roundtrip` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:172` | - |  |
| `test_doppler_fall_detection` | 0 | `crates/electromagnetics-rf-sensing/src/lib.rs:72` | - |  |
| `test_drop_shadow_round_trip` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:864` | - |  |
| `test_dynamic_arrays_and_spill` | 0 | `crates/chronicle-tabular-engine/tests/tabular_tests.rs:87` | - |  |
| `test_ebml_vint_roundtrip` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:62` | - |  |
| `test_ebu_r128_loudness_object_not_floor` | 0 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:367` | - |  |
| `test_ebu_r128_metering` | 0 | `crates/acoustics-daw-session/src/lib.rs:40` | - |  |
| `test_edl_parsing_and_writing` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:61` | - |  |
| `test_electromagnetics_csi_vitals_aoa_object_two_produces` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:1134` | - |  |
| `test_electromagnetics_metamer_nullspace_produce_safe_map` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/vox0_metamer.rs:548` | - |  |
| `test_encode_decode_roundtrip` | 0 | `crates/video-prores-codec/src/lib.rs:36` | - |  |
| `test_encode_probe_decode_mov_roundtrip` | 0 | `crates/video-prores-codec/src/mov.rs:139` | - |  |
| `test_equation_latex_and_unicode_math` | 0 | `crates/layout-paged-typesetter/tests/typesetter_tests.rs:4` | - |  |
| `test_error_handling` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:360` | - |  |
| `test_evaluate_field_lazy_chunking` | 0 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:405` | - |  |
| `test_evaluate_nla_channelbag` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:388` | - |  |
| `test_evaluate_nla_channelbag_two_produces_match` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:510` | - |  |
| `test_eviction_over_limit` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:391` | - |  |
| `test_export_lottie` | 0 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:194` | - |  |
| `test_expression_evaluation` | 0 | `crates/locus-parametric-solver/src/tests.rs:190` | - |  |
| `test_fcp7_xml_roundtrip` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:107` | - |  |
| `test_fcpxml_roundtrip` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:134` | - |  |
| `test_fill_distribution_with_clamping` | 0 | `crates/layout-box-autolayout/src/tests.rs:60` | - |  |
| `test_filter_kuwahara_anisotropic` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:118` | - |  |
| `test_financial_pmt_npv_xirr` | 0 | `crates/chronicle-tabular-engine/tests/tabular_tests.rs:42` | - |  |
| `test_fit_size` | 0 | `crates/raster-spatial-operators/src/resample.rs:371` | - |  |
| `test_flood_round_trip` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:848` | - |  |
| `test_forms_validation_flattening_and_xfdf` | 0 | `crates/discourse-pdf-compliance/src/tests.rs:157` | - |  |
| `test_formula_parsing` | 0 | `crates/chronicle-tabular-engine/tests/tabular_tests.rs:6` | - |  |
| `test_fspm_vascular_grow_botany` | 0 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:547` | - |  |
| `test_gamut_warning` | 0 | `crates/color-transforms/src/lib.rs:407` | - |  |
| `test_gaussian_blur_conservation` | 0 | `crates/raster-spatial-operators/src/convolve.rs:428` | - |  |
| `test_gaussian_blur_round_trip` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:856` | - |  |
| `test_gpl_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:465` | - |  |
| `test_gpx_tracklog_interpolation_and_meridian_wrap` | 0 | `crates/radiance-camera-raw/src/lib.rs:141` | - |  |
| `test_hdr_fusion_deghosting` | 0 | `crates/radiance-camera-raw/src/lib.rs:109` | - |  |
| `test_horizontal_flex_basic` | 0 | `crates/layout-box-autolayout/src/tests.rs:16` | - |  |
| `test_host` | 2 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:566` | - |  |
| `test_host` | 2 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:317` | - |  |
| `test_host` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:539` | - |  |
| `test_html_codecs` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:63` | - |  |
| `test_hug_container_sizing` | 0 | `crates/layout-box-autolayout/src/tests.rs:40` | - |  |
| `test_icc_transform_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:369` | - |  |
| `test_identity_convolution` | 0 | `crates/raster-spatial-operators/src/convolve.rs:418` | - |  |
| `test_inspect_engine_telemetry` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:366` | - |  |
| `test_instance_on_points_and_realize` | 0 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:506` | - |  |
| `test_integrator_simulation` | 0 | `crates/kinetics-block-simulation/src/lib.rs:66` | physics |  |
| `test_interpolate_keyframes` | 0 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:145` | - |  |
| `test_interpolate_motor_fti_sclerp` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:164` | - |  |
| `test_interpolate_motor_fti_teleport` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:193` | - |  |
| `test_jet_forward_ad` | 0 | `crates/kinetics-garment-draping/src/lib.rs:31` | - |  |
| `test_jfke_fails_closed_when_cycles_share_same_orientation` | 0 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:712` | - |  |
| `test_jfke_fails_closed_when_vertices_not_adjacent` | 0 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:703` | - |  |
| `test_jfke_tetrahedron_splice_is_quad_not_set_union` | 0 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:689` | - |  |
| `test_json_document_parse_and_run` | 0 | `crates/kinetics-block-simulation/src/lib.rs:123` | - |  |
| `test_keyframe_track_evaluation` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:72` | - |  |
| `test_kinetic_text_animators` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:157` | - |  |
| `test_kinsoku_line_breaking` | 0 | `crates/layout-paged-typesetter/tests/typesetter_tests.rs:34` | - |  |
| `test_kml_parsing` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:108` | - |  |
| `test_landxml_parsing` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:69` | - |  |
| `test_latex_codecs` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:94` | - |  |
| `test_layer_rgba8_conversion` | 0 | `crates/surface-layered-codecs/src/psd/tests.rs:36` | - |  |
| `test_layer_stack_composition` | 0 | `crates/surface-pbr-studio/src/lib.rs:100` | - |  |
| `test_layout_box_tree_flex_grid_nonoverlapping` | 0 | `crates/asset-production-vibe/src/layout_ops/mod.rs:407` | - |  |
| `test_layout_diegetic_projection_invertible` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/diegetic_input.rs:439` | 3d |  |
| `test_layout_flex_grid_exact_fill_and_fr` | 0 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:477` | - |  |
| `test_layout_framlock_crc32c` | 0 | `crates/asset-production-vibe/src/layout_ops/mod.rs:505` | - |  |
| `test_layout_paged_foliation_monotonic_pages` | 0 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:506` | - |  |
| `test_liang_hyphenation` | 0 | `crates/layout-paged-typesetter/tests/typesetter_tests.rs:24` | - |  |
| `test_line` | 0 | `crates/vector-surface-brush/src/lib.rs:130` | - |  |
| `test_lm_minimum_norm` | 0 | `crates/locus-parametric-solver/src/tests.rs:35` | - |  |
| `test_locus_bishop_rmf_cyclic` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:246` | - |  |
| `test_locus_fillet_chamfer` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:120` | - |  |
| `test_locus_fillet_g2_print_density_svg_path_roundtrip` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:344` | - |  |
| `test_locus_fillet_square` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:86` | - |  |
| `test_locus_resample_count` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:149` | - |  |
| `test_locus_resample_length_and_curvature` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:177` | - |  |
| `test_locus_shortest_edge_path` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:286` | graph |  |
| `test_locus_trim` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:218` | - |  |
| `test_log_curves_encode_decode` | 0 | `crates/radiance-color-grading/src/lib.rs:107` | - |  |
| `test_lottie_parsing_and_evaluation` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:121` | - |  |
| `test_lut_compaction_and_offset_updates` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:401` | - |  |
| `test_lut_cube_tetrahedral` | 0 | `crates/radiance-color-grading/src/lib.rs:96` | - |  |
| `test_lut_start_invalidation` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:534` | - |  |
| `test_m_code_evaluation` | 0 | `crates/discourse-scientific-matlang/src/lib.rs:89` | - |  |
| `test_manifold_bevel` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:525` | - |  |
| `test_manifold_bisect_plane` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:591` | - |  |
| `test_manifold_boolean_3d_golden_checksum` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1569` | - |  |
| `test_manifold_boolean_3d_production_union_difference` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1332` | - |  |
| `test_manifold_compile_bim_ifc4_roundtrip` | 0 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:819` | - |  |
| `test_manifold_inset` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:547` | - |  |
| `test_manifold_loft_linear_remains_ruled_cubic_bends` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1257` | numerics |  |
| `test_manifold_loft_nurbs_optional_keeps_linear_cubic` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1616` | numerics |  |
| `test_manifold_project_dxf_mesh_roundtrip` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1380` | computational geometry |  |
| `test_manifold_simplify_decimate_default_qem` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1478` | - |  |
| `test_manifold_simplify_remesh_and_anisotropic_small_mesh` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1514` | computational geometry |  |
| `test_manifold_solidify` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:569` | - |  |
| `test_manifold_subdivide_catmull_clark` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:622` | - |  |
| `test_manifold_subdivide_catmull_clark_produce_safe_and_rounds` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:653` | - |  |
| `test_manifold_subdivide_catmull_clark_rejects_oob_triangle` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:716` | - |  |
| `test_manifold_sweep_assembly_linear_and_cubic` | 0 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:719` | numerics |  |
| `test_manifold_sweep_cubic_densify_caps_at_4096` | 0 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:773` | - |  |
| `test_markdown_codecs` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:35` | - |  |
| `test_mask_density_and_feathering` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:51` | - |  |
| `test_materialx_export` | 0 | `crates/surface-pbr-studio/src/lib.rs:123` | - |  |
| `test_math_parser_algebraic_and_units` | 0 | `crates/layout-nodegraph-compiler/src/tests.rs:220` | - |  |
| `test_matrix_decompositions` | 0 | `crates/discourse-scientific-matlang/src/lib.rs:24` | numerics |  |
| `test_matrix_slicing` | 0 | `crates/discourse-scientific-matlang/src/lib.rs:73` | numerics |  |
| `test_matroska_mux_demux_roundtrip` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:73` | - |  |
| `test_membrane_and_bending_energy` | 0 | `crates/kinetics-garment-draping/src/lib.rs:50` | - |  |
| `test_mesh_to_sdf_brick_and_dual_contouring_roundtrip` | 0 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:439` | computational geometry |  |
| `test_morph_matching_and_interpolation` | 0 | `crates/layout-presentation-deck/tests/deck_tests.rs:50` | - |  |
| `test_motion_path_spline` | 0 | `crates/layout-presentation-deck/tests/deck_tests.rs:82` | - |  |
| `test_mpegts_packet_and_psi` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:137` | - |  |
| `test_mtext_parsing_and_layout` | 0 | `crates/locus-parametric-solver/src/tests.rs:253` | - |  |
| `test_multi_line_wrapping` | 0 | `crates/layout-box-autolayout/src/tests.rs:84` | - |  |
| `test_music_aoa_peak_detection` | 0 | `crates/electromagnetics-rf-sensing/src/lib.rs:94` | - |  |
| `test_mvt_geometry_decoding` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:154` | - |  |
| `test_mxf_klv_and_writer_roundtrip` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:113` | - |  |
| `test_navigate_funnel_orca` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:361` | - |  |
| `test_ncc_point_tracker` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:88` | - |  |
| `test_newton_pcg_solver_step` | 0 | `crates/kinetics-garment-draping/src/lib.rs:113` | - |  |
| `test_no_eviction_at_limit` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:382` | - |  |
| `test_no_eviction_under_limit` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:373` | - |  |
| `test_nodegraph_pipeline_basic` | 0 | `crates/layout-nodegraph-compiler/src/tests.rs:17` | - |  |
| `test_octree_quantize_palette_budget` | 0 | `crates/raster-spatial-operators/src/octree.rs:334` | - |  |
| `test_octree_quantize_with_dither` | 0 | `crates/raster-spatial-operators/src/octree.rs:353` | - |  |
| `test_offset_conversion` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:824` | - |  |
| `test_offset_round_trip` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:843` | - |  |
| `test_ontology_contract_parity` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:200` | semantics |  |
| `test_ontology_contract_parity` | 0 | `crates/layout-presentation-deck/tests/deck_tests.rs:144` | semantics |  |
| `test_ontology_contracts` | 0 | `crates/radiance-camera-raw/src/lib.rs:221` | semantics |  |
| `test_ontology_contracts` | 0 | `crates/surface-layered-codecs/src/lib.rs:247` | semantics |  |
| `test_open_raster_parsing_synthetic` | 0 | `crates/surface-layered-codecs/src/lib.rs:209` | - |  |
| `test_openxml_docx_roundtrip` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:152` | - |  |
| `test_openxml_pptx_roundtrip` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:173` | - |  |
| `test_openxml_xlsx_roundtrip` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:129` | - |  |
| `test_ordinary_kriging` | 0 | `crates/geodesy-spatial-analysis/src/lib.rs:71` | - |  |
| `test_otio_json_roundtrip` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:79` | - |  |
| `test_pack_msdf_font_atlas` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:117` | - |  |
| `test_packbits_decode` | 0 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:165` | - |  |
| `test_packbits_rle_round_trip` | 0 | `crates/surface-layered-codecs/src/lib.rs:43` | - |  |
| `test_pades_signatures_and_timestamps` | 0 | `crates/discourse-pdf-compliance/src/tests.rs:132` | - |  |
| `test_pagination_widow_orphan` | 0 | `crates/layout-paged-typesetter/tests/typesetter_tests.rs:60` | - |  |
| `test_paint_and_polyfill` | 0 | `crates/surface-pbr-studio/src/lib.rs:77` | - |  |
| `test_parametric_sketch_horizontal_and_distance` | 0 | `crates/locus-parametric-solver/src/tests.rs:45` | - |  |
| `test_pattern_mp4` | 1 | `crates/video-h264-preview/src/synth.rs:164` | - | Color-bar test movie for demos, tests, and preview verification (320x192 at 24 fps). |
| `test_pdc_mix_engine` | 0 | `crates/acoustics-daw-session/src/lib.rs:116` | - |  |
| `test_perspective_homography_identity` | 0 | `crates/raster-spatial-operators/src/distort.rs:484` | - |  |
| `test_placeholder_inheritance` | 0 | `crates/layout-presentation-deck/tests/deck_tests.rs:4` | - |  |
| `test_pmtiles_header_parsing` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:135` | - |  |
| `test_polar_depolar_roundtrip` | 0 | `crates/raster-spatial-operators/src/distort.rs:507` | - |  |
| `test_preflight_pdfa_and_pdfx` | 0 | `crates/discourse-pdf-compliance/src/tests.rs:204` | - |  |
| `test_preset_conversion` | 0 | `crates/vector-radiance-trace/src/tests.rs:218` | - |  |
| `test_privacy_witness_receipt_verification` | 0 | `crates/electromagnetics-rf-sensing/src/lib.rs:113` | - |  |
| `test_probrush_dynamics_and_stamp` | 0 | `crates/surface-pbr-studio/src/lib.rs:140` | - |  |
| `test_project_diegetic_surface` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:147` | - |  |
| `test_project_metamer_nullspace_invariance` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:297` | - |  |
| `test_prores_4444_with_alpha` | 0 | `crates/video-prores-codec/src/lib.rs:68` | - |  |
| `test_psb_64bit_encode_decode` | 0 | `crates/surface-layered-codecs/src/lib.rs:110` | - |  |
| `test_psd_encode_decode_round_trip` | 0 | `crates/surface-layered-codecs/src/lib.rs:59` | - |  |
| `test_psd_encode_import_roundtrip` | 0 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:185` | - |  |
| `test_psd_header_and_blend_mode_keys` | 0 | `crates/surface-layered-codecs/src/psd/tests.rs:5` | - |  |
| `test_query_direct_space_cast_motion` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:328` | - |  |
| `test_query_direct_space_intersect_ray` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:293` | 3d |  |
| `test_query_filter_evaluator` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:172` | - |  |
| `test_radial_euler_default_refinement_and_m64_serialization` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:222` | physics |  |
| `test_radial_euler_e64_magic_alias_round_trip` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:275` | physics |  |
| `test_radial_euler_explicit_ops` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:322` | physics |  |
| `test_radial_euler_jfke_fails_closed_on_boundary_edge` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:502` | physics, graph |  |
| `test_radial_euler_jfke_fan_diagonals_not_edge_zero` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:445` | physics, graph |  |
| `test_radial_euler_jfke_tetrahedron_quad_cycle_and_m64_shell` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:379` | physics |  |
| `test_radiance_2k_lattice_and_named_1024_profile` | 0 | `crates/asset-production-vibe/src/graphics.rs:784` | - |  |
| `test_radiance_cdl_scopes_nontrivial_pixel` | 0 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:414` | 2d |  |
| `test_radiance_cdl_scopes_profile_admit_and_charge` | 0 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:476` | - |  |
| `test_radiance_png_16bit_be_high_byte` | 0 | `crates/asset-production-vibe/src/graphics.rs:823` | - |  |
| `test_radiance_png_psd_lattice_roundtrip_nontrivial` | 0 | `crates/asset-production-vibe/src/graphics.rs:695` | - |  |
| `test_radiance_raw_hdr_nontrivial_pixel` | 0 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:384` | 2d |  |
| `test_radiance_raw_hdr_profile_refuses_outside_lattice` | 0 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:445` | - |  |
| `test_raymarch_vox0_dda_procedural` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:191` | - |  |
| `test_raymarch_vox0_dda_sealed_emf_non_black` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:233` | - |  |
| `test_redaction_pii_and_zero_residue` | 0 | `crates/discourse-pdf-compliance/src/tests.rs:87` | - |  |
| `test_render_spectral_slice_default_exposure_non_black` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:908` | - |  |
| `test_render_spectral_slice_tampered_emf_fails_closed` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:936` | - |  |
| `test_resample_uniform_conservation` | 0 | `crates/raster-spatial-operators/src/resample.rs:345` | - |  |
| `test_ris_conversion` | 0 | `crates/discourse-academic-tex/src/bibliography/import.rs:296` | - |  |
| `test_route_bus_dag_3_buses` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:189` | - |  |
| `test_route_bus_dag_cycle_detection` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:275` | - |  |
| `test_route_bus_dag_master_is_non_silent_object` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:490` | - |  |
| `test_route_input_action_stages` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:181` | - |  |
| `test_rtf_codecs` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:75` | - |  |
| `test_sample_procedural_field_all_bases` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:655` | - |  |
| `test_seam_carve_both_axes` | 0 | `crates/raster-spatial-operators/src/seam_carve.rs:361` | - |  |
| `test_seam_carve_flat_region_dropped_first` | 0 | `crates/raster-spatial-operators/src/seam_carve.rs:318` | - |  |
| `test_seam_carve_identity` | 0 | `crates/raster-spatial-operators/src/seam_carve.rs:350` | - |  |
| `test_shape_harfbuzz_bidi_ltr` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:61` | - |  |
| `test_shape_harfbuzz_bidi_rtl_hebrew` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:95` | - |  |
| `test_single_stroke_cam_rendering` | 0 | `crates/locus-parametric-solver/src/tests.rs:242` | - |  |
| `test_smart_rule_evaluation` | 0 | `crates/radiance-camera-raw/src/lib.rs:196` | - |  |
| `test_sniff` | 0 | `crates/discourse-academic-tex/src/bibliography/import.rs:290` | - |  |
| `test_sobel_edge_detection` | 0 | `crates/raster-spatial-operators/src/convolve.rs:438` | graph |  |
| `test_solve_sdfgi_froxel` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:248` | numerics |  |
| `test_solve_two_bone_ik_reachable` | 0 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:58` | numerics |  |
| `test_solve_two_bone_ik_skl1_10d_two_produces_match` | 0 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:262` | numerics |  |
| `test_solve_two_bone_ik_unreachable` | 0 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:115` | numerics |  |
| `test_spaces_gamut_matrix` | 0 | `crates/radiance-color-grading/src/lib.rs:117` | numerics |  |
| `test_spatial_arc_length_table` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:56` | - |  |
| `test_spatialize_pga_zone` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:312` | - |  |
| `test_spatialize_pga_zone_inverse_distance_object` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:542` | - |  |
| `test_spec` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:26` | - |  |
| `test_spectral_display_irradiance_visible_for_compiled_amplitude` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:1099` | - |  |
| `test_srgb_roundtrip` | 0 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:174` | - |  |
| `test_srt_roundtrip` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:161` | - |  |
| `test_step_island_xpbd_area_override` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:257` | - |  |
| `test_step_island_xpbd_ground_and_islands` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:216` | - |  |
| `test_step_island_xpbd_two_produces_match_permuted_ids` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:475` | - |  |
| `test_step_simulation_zone` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:431` | - |  |
| `test_step_simulation_zone_two_produces_match` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:569` | - |  |
| `test_stream_instant` | 0 | `crates/audio-device-cpal/src/lib.rs:840` | - |  |
| `test_svg_figure_generation` | 0 | `crates/discourse-scientific-matlang/src/lib.rs:111` | - |  |
| `test_sync_spatial_motors` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:475` | - |  |
| `test_table_autofit` | 0 | `crates/layout-paged-typesetter/tests/typesetter_tests.rs:45` | - |  |
| `test_take_and_restore_luts` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:501` | - |  |
| `test_tetrahedral_clut_interpolation` | 0 | `crates/color-transforms/src/lib.rs:392` | - |  |
| `test_timeline_and_show_progression` | 0 | `crates/layout-presentation-deck/tests/deck_tests.rs:97` | - |  |
| `test_topology_validation` | 0 | `crates/geodesy-spatial-analysis/src/lib.rs:144` | - |  |
| `test_trace_rgba_direct` | 0 | `crates/vector-radiance-trace/src/tests.rs:198` | - |  |
| `test_transfer_roundtrips` | 0 | `crates/radiance-color-grading/src/lib.rs:57` | - |  |
| `test_unique_entry_creation` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:362` | - |  |
| `test_unknown_op` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:316` | - |  |
| `test_unwrap_slim_mode_slim_default_and_atlas` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:482` | - |  |
| `test_unwrap_slim_planar_quad` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:440` | - |  |
| `test_unwrap_slim_uv_island_10d_roundtrip` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:685` | - |  |
| `test_vbap_panning` | 0 | `crates/acoustics-daw-session/src/lib.rs:59` | - |  |
| `test_vbd_step_simulation` | 0 | `crates/kinetics-garment-draping/src/lib.rs:83` | - |  |
| `test_vector_motion_blur` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:214` | - |  |
| `test_verify_emf_refuses_crc_consistent_bao_omitted` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:992` | - |  |
| `test_verify_fiduciary_rid` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:390` | - |  |
| `test_vital_signs_extraction` | 0 | `crates/electromagnetics-rf-sensing/src/lib.rs:52` | - |  |
| `test_vtt_roundtrip` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:174` | - |  |
| `test_wav_codec_roundtrip` | 0 | `crates/acoustics-daw-session/src/lib.rs:99` | - |  |
| `test_wet_mixer_brush_strokes` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:100` | - |  |
| `test_wgs84_distance_and_area` | 0 | `crates/geodesy-spatial-analysis/src/lib.rs:46` | - |  |
| `test_wkb_roundtrip` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:38` | - |  |
| `test_xcf_parsing_synthetic` | 0 | `crates/surface-layered-codecs/src/lib.rs:191` | - |  |
| `test_xml_dom_parser_and_writer` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:18` | - |  |
| `test_xtrans_demosaic` | 0 | `crates/radiance-camera-raw/src/lib.rs:74` | - |  |
| `test_ycbcr_roundtrip` | 0 | `crates/radiance-color-grading/src/lib.rs:45` | - |  |
| `test_yin_pitch_detection` | 0 | `crates/acoustics-daw-session/src/lib.rs:70` | - |  |
| `test_zip_archive_roundtrip` | 0 | `crates/chronicle-office-codecs/tests/codecs_tests.rs:4` | - |  |
| `tet_volume` | 4 | `crates/kinetics-continuum-multiphysics/src/mesh/tet.rs:28` | - | Signed volume of a tetrahedron (positive if nodes satisfy right-hand rule). |
| `tet_volume_unit_cube_subdivision` | 0 | `crates/kinetics-continuum-multiphysics/src/mesh/tet.rs:56` | - |  |
| `impl Clut::tetra` | 5 | `crates/color-transforms/src/icc/clut.rs:117` | - | Tetrahedral interpolation inside one cube of three dimensions. |
| `tetrahedral_exact_on_affine_and_nodes` | 0 | `crates/color-transforms/src/lut.rs:537` | - |  |
| `tetrahedral_matches_brute_force` | 0 | `crates/color-transforms/src/lut.rs:515` | physics |  |
| `tex_directives` | 0 | `crates/discourse-academic-tex/src/texparse/magic.rs:134` | - |  |
| `texels` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:468` | - |  |
| `text` | 1 | `crates/asset-production-vibe/src/lib.rs:491` | - |  |
| `impl Value::text` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:151` | - |  |
| `impl XmlWriter::text` | 3 | `crates/chronicle-timeline-interchange/src/xml.rs:118` | - |  |
| `impl Page::text` | 6 | `crates/discourse-academic-tex/src/texpdf.rs:290` | - |  |
| `impl Lay<'a>::text` | 5 | `crates/discourse-academic-tex/src/typeset.rs:362` | - |  |
| `impl Paragraph::text` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:71` | - |  |
| `impl TextBody::text` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:83` | - |  |
| `impl Shape::text` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:209` | - |  |
| `text` | 3 | `crates/vector-locus-pathops/src/effects/util.rs:32` | - |  |
| `impl FlowInline::text_content` | 1 | `crates/chronicle-office-codecs/src/flow/model.rs:32` | - |  |
| `impl XEl::text_content` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:85` | - |  |
| `text_of` | 1 | `crates/layout-presentation-deck/src/morph/match_pairs.rs:7` | - |  |
| `text_on_path_positions_glyphs` | 0 | `crates/vector-glyph-shaping/src/layout.rs:275` | - |  |
| `textjoin` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:14` | - |  |
| `impl Pixels<'win>::texture` | 1 | `crates/pixel-buffer-output/src/lib.rs:752` | 2d | Provides access to the internal source [`wgpu::Texture`]. |
| `impl IntermediateTextureAllocations::texture_count` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:243` | - | Return the total number of physical textures represented by these allocations. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::texture_format` | 2 | `crates/pixel-buffer-output/src/builder.rs:155` | 2d | Set the texture format. |
| `texture_format_size` | 1 | `crates/pixel-buffer-output/src/builder.rs:537` | - |  |
| `texture_ids` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:679` | - |  |
| `texture_pair_constraints` | 0 | `crates/vector-hybrid-renderer/src/target.rs:319` | - |  |
| `impl LayerTextureRegion::texture_rect` | 2 | `crates/vector-hybrid-renderer/src/target.rs:272` | 2d | Translate a scene-space rectangle within this layer to texture coordinates. |
| `texture_rect_translation` | 0 | `crates/vector-hybrid-renderer/src/target.rs:334` | 2d |  |
| `texture_runs` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:743` | - |  |
| `texture_runs_coalesce_distinct_paints_for_same_texture` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:810` | - |  |
| `texture_runs_include_atlas_images` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:827` | - |  |
| `impl WebGlRendererContext<'_>::texture_size` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2939` | - |  |
| `impl RendererContext<'_>::texture_size` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2846` | - |  |
| `impl Atlases::texture_size` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:46` | - |  |
| `tf_decode` | 6 | `crates/audio-opus-codec/src/celt/mod.rs:564` | - |  |
| `impl Mat3::then` | 2 | `crates/drafting-geometry/src/mat.rs:64` | - | Apply `self` first, then `o`. |
| `impl Homography::then_after` | 2 | `crates/vector-locus-geometry/src/projective.rs:79` | - | `self ∘ other`: `other` first. |
| `impl Mat3::then_before` | 2 | `crates/drafting-geometry/src/mat.rs:53` | - | `self * o`: apply `o` first, then `self`. |
| `thermal_conduction_matrix_symmetry` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/structural.rs:171` | numerics |  |
| `thermal_conduction_quad4` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/structural.rs:106` | graph, numerics | 4-node 2D thermal conduction element conductivity matrix (4x4). |
| `thermal_shape` | 3 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:297` | - | Apply multi-scale thermal shaping to relax steep slopes across a resolution pyramid. |
| `thermal_shape_with` | 3 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:305` | - | Apply multi-scale thermal shaping with custom world scale configuration. |
| `thermal_shaper_preserves_bounds_and_flattens_spikes` | 0 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:363` | - |  |
| `thin` | 1 | `crates/vector-radiance-trace/src/centerline.rs:196` | - |  |
| `impl Decoder::threads` | 1 | `crates/video-h264-codec/src/decoder.rs:164` | - | Worker threads decoding pictures (1: on the calling thread). |
| `threedl_roundtrip_and_orientation` | 0 | `crates/color-transforms/src/lut.rs:601` | - |  |
| `impl Movie::thumbnail_at` | 3 | `crates/video-h264-preview/src/movie.rs:353` | - |  |
| `impl Movie::thumbnails` | 3 | `crates/video-h264-preview/src/movie.rs:391` | - |  |
| `impl Oscillator::tick` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:83` | - |  |
| `impl TempoMap::tick_at_bar_beat` | 2 | `crates/audio-timebase/src/tempo.rs:338` | - | Musical `BarBeat` to absolute `tick`. |
| `impl FrameRate::tick_of` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:96` | - |  |
| `impl FrameRate::tick_of` | 2 | `crates/timeline-editing/src/timebase.rs:180` | - | Start tick for frame `frame`, rounded up so the tick belongs to that frame. |
| `impl FrameRate::tick_of_frame` | 2 | `crates/kinetics-motion-compositor/src/time.rs:73` | - |  |
| `impl TempoMap::tick_to_samples` | 3 | `crates/audio-timebase/src/tempo.rs:233` | - | Convert ticks to samples. |
| `impl TempoMap::tick_to_samples_f` | 3 | `crates/audio-timebase/src/tempo.rs:237` | - |  |
| `impl NoteValue::ticks` | 1 | `crates/audio-timebase/src/grid.rs:31` | - | Ticks in a straight note of this value in 4/4. |
| `impl TimeSig::ticks_per_bar` | 1 | `crates/audio-timebase/src/tempo.rs:32` | - | Number of ticks in a complete bar under this meter. |
| `impl MeterPoint::ticks_per_bar` | 1 | `crates/audio-timebase/src/tempo.rs:83` | - |  |
| `impl TimeSig::ticks_per_beat` | 1 | `crates/audio-timebase/src/tempo.rs:26` | - | Number of ticks in one beat under this meter. |
| `impl MeterPoint::ticks_per_beat` | 1 | `crates/audio-timebase/src/tempo.rs:79` | - |  |
| `tile_alloc` | 2 | `crates/vector-compute-shaders/src/cpu/tile_alloc.rs:57` | - |  |
| `tile_alloc_main` | 6 | `crates/vector-compute-shaders/src/cpu/tile_alloc.rs:13` | - |  |
| `impl CowTerrainTileGrid::tile_arc` | 3 | `crates/terrain-brushes/adaptation/grid-impl.rs:270` | - |  |
| `impl CowTerrainTileGrid::tile_arc` | 3 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:303` | - |  |
| `impl CowTerrainTileGrid::tile_arc` | 3 | `crates/terrain-brushes/src/lib.rs:455` | - |  |
| `impl CowTerrainTileGrid::tile_bytes` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:34` | - |  |
| `impl CowTerrainTileGrid::tile_bytes` | 1 | `crates/terrain-brushes/src/lib.rs:215` | - |  |
| `tile_mode_from_extend` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1202` | - |  |
| `impl CowTerrainTileGrid::tile_size` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:25` | - |  |
| `impl CowTerrainTileGrid::tile_size` | 1 | `crates/terrain-brushes/src/lib.rs:205` | - |  |
| `impl CowTerrainTileGrid::tiles` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:29` | - |  |
| `impl CowTerrainTileGrid::tiles` | 1 | `crates/terrain-brushes/src/lib.rs:210` | - |  |
| `impl Index::tiles` | 2 | `crates/vector-locus-pathops/src/three_d/visibility.rs:119` | 2d |  |
| `time_ago` | 2 | `crates/multimodal-synthesis-schema/src/job_feed.rs:76` | - | Relative human-readable time formatting: "just now", "5 minutes ago", etc. |
| `time_fraction` | 3 | `crates/chronicle-tabular-engine/src/core/date.rs:143` | - |  |
| `timecode_and_prores` | 0 | `crates/media-isobmff/src/codec.rs:1680` | - |  |
| `timespec_diff_nanos` | 2 | `crates/audio-device-cpal/src/host/alsa/mod.rs:900` | - |  |
| `timespec_to_nanos` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:894` | - |  |
| `impl InputCallbackInfo::timestamp` | 1 | `crates/audio-device-cpal/src/lib.rs:489` | - | The timestamp associated with the call to an input stream's data callback. |
| `impl OutputCallbackInfo::timestamp` | 1 | `crates/audio-device-cpal/src/lib.rs:496` | - | The timestamp associated with the call to an output stream's data callback. |
| `timestamp_query_support` | 3 | `crates/gpu-command-profiler/src/profiler.rs:542` | - | Returns true if a timestamp query is supported. |
| `timoshenko_beam_3d` | 6 | `crates/kinetics-continuum-multiphysics/src/fem/structural.rs:9` | graph, numerics | 3D Timoshenko beam element stiffness matrix (12x12 for 2 nodes with 6 DOFs per node). |
| `timoshenko_beam_symmetry` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/structural.rs:160` | - |  |
| `tiny_circle_secants_and_separated_circles` | 0 | `crates/drafting-geometry/src/tests.rs:113` | - |  |
| `tissot_distortion` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:520` | 3d | Computes Tissot distortion of a projection at a given coordinate using Snyder's central differences. |
| `impl Rule::title` | 1 | `crates/discourse-pdf-compliance/src/a11y/rules.rs:156` | - |  |
| `impl Metadata::title` | 1 | `crates/media-isobmff/src/demux.rs:199` | - | Title from `©nam` or the `com.apple.quicktime.title` key. |
| `tmpdir` | 1 | `crates/media-test-support/src/oracle.rs:129` | - |  |
| `impl Encoder::tns_analyse` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:411` | - |  |
| `tns_max_bands` | 2 | `crates/audio-aac-codec/src/tables.rs:90` | - | `TNS_MAX_BANDS` for AAC-LC (long, short). |
| `tns_max_order` | 1 | `crates/audio-aac-codec/src/tables.rs:98` | - | `TNS_MAX_ORDER` for AAC-LC. |
| `impl Vec2::to3` | 2 | `crates/drafting-geometry/src/vec.rs:79` | 3d |  |
| `impl Point3D::to_2d` | 1 | `crates/locus-geospatial-codecs/src/geometry.rs:31` | - |  |
| `impl Lut::to_3dl` | 1 | `crates/color-transforms/src/lut.rs:407` | computational geometry | Write a `.3dl` (10-bit input mesh, 12-bit output). Shapers are baked into the cube. |
| `impl Vec3::to_array` | 1 | `crates/surface-pbr-studio/src/math.rs:171` | - |  |
| `impl Vec4::to_array` | 1 | `crates/surface-pbr-studio/src/math.rs:256` | - |  |
| `impl Vec2::to_array` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:101` | - |  |
| `impl Homography::to_array` | 1 | `crates/vector-locus-geometry/src/projective.rs:33` | - | The nine row-major numbers. |
| `impl AdtsHeader::to_asc` | 1 | `crates/audio-aac-codec/src/config.rs:323` | - | The equivalent `AudioSpecificConfig`. Channel configuration 0 (PCE in-band) is resolved by the |
| `to_base` | 4 | `crates/chronicle-fiduciary-ledger/src/money.rs:44` | - | Converts an amount in foreign currency to base currency (1 base = rate foreign). |
| `impl Unit::to_base_scale` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/ast.rs:44` | - | Converts standard unit to its base canonical dimension: |
| `impl PathData::to_bezpath` | 1 | `crates/vector-locus-geometry/src/path.rs:366` | - |  |
| `impl SubPath::to_bezpath_into` | 2 | `crates/vector-locus-geometry/src/path.rs:220` | - |  |
| `impl AudioSpecificConfig::to_bytes` | 1 | `crates/audio-aac-codec/src/config.rs:244` | - | Serialise (AAC-LC, no SBR signalling). |
| `impl OpusHead::to_bytes` | 1 | `crates/audio-opus-codec/src/header.rs:77` | - | Serialises as an `OpusHead` packet. |
| `impl MasteringDisplay::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:75` | - | The 24-byte payload shared by `mdcv` and the SEI message. |
| `impl AvcConfig::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:279` | - |  |
| `impl HevcConfig::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:382` | - |  |
| `impl Av1Config::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:466` | - |  |
| `impl VpcConfig::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:532` | - |  |
| `impl OpusConfig::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:643` | - |  |
| `impl ApvConfig::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:805` | - | Serialize the full `apvC` box payload (4-byte `FullBox` header + `APVDecoderConfigurationRecord`). |
| `impl BlendRange::to_bytes` | 1 | `crates/raster-layer-mask-adapter/src/masks.rs:184` | - |  |
| `impl SpeakerPosition::to_cartesian` | 1 | `crates/acoustics-daw-session/src/dsp/pan.rs:50` | - | Unit Cartesian direction vector (x: right, y: front, z: up). |
| `to_clean_points` | 1 | `crates/asset-production-vibe/src/locus_ops.rs:265` | - |  |
| `to_csv` | 2 | `crates/chronicle-office-codecs/src/spreadsheet/csv.rs:98` | numerics | Serialize a 2D matrix of string cells into RFC 4180 CSV / TSV format. |
| `to_csv` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:62` | - |  |
| `impl Lut3d::to_cube` | 1 | `crates/color-transforms/src/lut.rs:93` | - |  |
| `impl Lut::to_cube` | 1 | `crates/color-transforms/src/lut.rs:299` | - |  |
| `impl Lut3d::to_cube` | 1 | `crates/radiance-color-grading/src/lut.rs:187` | - | Serialize this 3D LUT to `.cube` format string. |
| `impl Lut::to_cube` | 1 | `crates/radiance-color-grading/src/lut.rs:326` | - | Serialize to `.cube` format string. |
| `impl Value::to_display_string` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:179` | - |  |
| `to_docx_package` | 1 | `crates/chronicle-office-codecs/src/wordprocessing/docx.rs:10` | - | Serialize a `FlowDoc` into an OpenXML `.docx` ZIP package. |
| `to_f32` | 1 | `crates/color-transforms/src/spaces.rs:121` | - |  |
| `to_f32` | 1 | `crates/radiance-color-grading/src/spaces.rs:172` | - |  |
| `to_f64` | 1 | `crates/chronicle-tabular-engine/src/functions/util.rs:20` | - |  |
| `impl Timecode::to_frames` | 2 | `crates/audio-timebase/src/timecode.rs:179` | - | Absolute frame count corresponding to this timecode label. |
| `impl CompiledGlsl::to_generated_code` | 2 | `crates/vector-hybrid-shaders/src/types.rs:95` | 3d | Generate Rust modules that contain vertex and fragment source code. Uniforms and texture |
| `to_hex` | 1 | `crates/color-transforms/src/lib.rs:311` | - |  |
| `to_hex` | 1 | `crates/radiance-color-grading/src/matrix.rs:415` | - |  |
| `to_html` | 1 | `crates/chronicle-office-codecs/src/document/html.rs:10` | semantics | Serialize a `FlowDoc` to clean, semantic HTML5. |
| `to_jira_json` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:242` | - |  |
| `to_json` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:47` | - |  |
| `impl HumanWelfareMeasurements::to_json_string` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:259` | - |  |
| `impl PsdBlendMode::to_key` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:243` | - |  |
| `to_latex` | 1 | `crates/chronicle-office-codecs/src/document/latex.rs:9` | - | Export a `FlowDoc` to standard LaTeX source. |
| `to_latex` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:13` | - |  |
| `to_linear` | 2 | `crates/color-transforms/src/lib.rs:192` | numerics | Encoded â†’ linear for a transfer function (display-referred; PQ normalised to 100 nits = 1.0). |
| `to_linear` | 1 | `crates/layout-paged-typesetter/src/equation/linear.rs:92` | numerics |  |
| `to_linear` | 2 | `crates/radiance-color-grading/src/matrix.rs:224` | numerics | Convert encoded display value to linear light. |
| `to_markdown` | 1 | `crates/chronicle-office-codecs/src/document/markdown.rs:297` | - | Serialize a `FlowDoc` to CommonMark / GFM Markdown. |
| `impl FabricPreset::to_material` | 1 | `crates/kinetics-garment-draping/src/fabrics.rs:22` | - |  |
| `impl ZzLists::to_matrices` | 1 | `crates/video-h264-codec/src/params.rs:53` | - |  |
| `impl Document::to_model` | 2 | `crates/kinetics-block-simulation/src/document.rs:108` | - |  |
| `to_mspdi_xml` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:292` | - |  |
| `impl TracePreset::to_options` | 1 | `crates/vector-radiance-trace/src/lib.rs:208` | - |  |
| `impl PerspectiveGrid::to_page` | 3 | `crates/vector-locus-geometry/src/perspective.rs:258` | - | Project a 2D point on `plane` (in plane coordinates, points) to page coordinates. |
| `impl ByteRange::to_pdf_string` | 1 | `crates/discourse-pdf-compliance/src/sign/pades.rs:50` | - |  |
| `to_pptx_package` | 1 | `crates/chronicle-office-codecs/src/presentation/pptx.rs:10` | - | Serialize a `Presentation` into an OpenXML `.pptx` ZIP package. |
| `to_primavera_xer` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:491` | - |  |
| `impl Unit::to_pt` | 2 | `crates/vector-locus-geometry/src/units.rs:99` | - |  |
| `impl Tick::to_rational` | 3 | `crates/chronicle-timeline-interchange/src/time.rs:25` | - |  |
| `impl Tick::to_rational` | 3 | `crates/kinetics-motion-compositor/src/time.rs:21` | - |  |
| `impl Tick::to_rational_floor` | 3 | `crates/timeline-editing/src/timebase.rs:34` | - | Inverse rational conversion rounded down in the requested timebase. |
| `impl AcoColor::to_record` | 1 | `crates/color-transforms/src/palette/aco.rs:56` | - |  |
| `impl PsdLayer::to_rgba8` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:299` | - | Converts layer channel data to standard 8-bit RGBA pixels. |
| `to_rtf` | 1 | `crates/chronicle-office-codecs/src/document/rtf.rs:8` | - | Serialize a `FlowDoc` to standard RTF 1.5. |
| `to_samples` | 1 | `crates/audio-timebase/src/rate.rs:18` | - | Round a float sample count to `i64`, mapping NaN to 0 and saturating infinities. |
| `impl Timecode::to_samples` | 3 | `crates/audio-timebase/src/timecode.rs:196` | - | Sample index at the start of this timecode. |
| `impl FeetFrames::to_samples` | 3 | `crates/audio-timebase/src/timecode.rs:274` | - | Convert FeetFrames to audio samples. |
| `to_samples_is_total` | 0 | `crates/audio-timebase/src/rate.rs:143` | - |  |
| `impl AafDocument::to_sequence` | 1 | `crates/chronicle-timeline-interchange/src/interchange/aaf.rs:158` | - | Convert this AAF Document model into a TimelineSequence. |
| `to_signed` | 1 | `crates/video-prores-codec/src/decode.rs:357` | - |  |
| `to_sk_color_channel` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:906` | - | Convert an anyrender displacement-map [`ColorChannel`] to the corresponding Skia channel. |
| `impl SubElementHooks::to_small_llm_json` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:391` | - | Formats the compact ~22-token JSON command packet that a small (0.5B–4B) LLM emits. |
| `to_stream_instant` | 1 | `crates/audio-device-cpal/src/host/oboe/convert.rs:11` | - |  |
| `impl CamelotKey::to_string` | 1 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:23` | - |  |
| `impl FourCc::to_string_lossy` | 1 | `crates/media-isobmff/src/bytes.rs:19` | - | Lossy string form; `©` (0xA9) is mapped to U+00A9, other non-printables to `?`. |
| `impl Figure::to_svg` | 3 | `crates/discourse-scientific-matlang/src/plot.rs:153` | - | Renders figure to a standalone SVG XML string. |
| `impl OtFeatures::to_tags` | 1 | `crates/vector-glyph-shaping/src/features.rs:83` | - | List of tag strings that differ from default settings. |
| `trait RectU16Ext::to_tile_bounds` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:543` | - |  |
| `impl RectU16Ext for RectU16::to_tile_bounds` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:547` | - |  |
| `impl FeetFrames::to_total_frames` | 1 | `crates/audio-timebase/src/timecode.rs:269` | - | Total frames represented by this FeetFrames value. |
| `impl AseKind::to_u16` | 1 | `crates/color-transforms/src/palette/ase.rs:32` | - |  |
| `impl PsdVersion::to_u16` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:37` | - |  |
| `impl Tick::to_units_floor` | 2 | `crates/timeline-editing/src/timebase.rs:60` | - | Convert to units at `per_second`, rounded down (unlike `from_units`, |
| `impl Evaluator<'h>::to_value` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:153` | - |  |
| `impl ImageFormat::to_wgpu` | 1 | `crates/vector-compute-renderer/src/recording.rs:245` | - |  |
| `to_xlsx_package` | 1 | `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:36` | - | Serialize an `XlsxWorkbook` into an OpenXML `.xlsx` ZIP package. |
| `impl XEl::to_xml_string` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:127` | - |  |
| `impl Gamut::to_xyz` | 1 | `crates/color-transforms/src/spaces.rs:67` | - | RGB → CIE XYZ (normalised so white Y = 1). |
| `impl Gamut::to_xyz` | 1 | `crates/radiance-color-grading/src/spaces.rs:68` | - | RGB -> CIE XYZ (normalised so white Y = 1). |
| `toc_configs` | 0 | `crates/audio-opus-codec/src/packet.rs:274` | - |  |
| `today` | 0 | `crates/media-test-support/src/golden.rs:197` | - | Today's date (UTC) as YYYY-MM-DD, for attribution sidecars. |
| `impl DebugLayers::toggle` | 2 | `crates/vector-compute-renderer/src/debug.rs:108` | - | Toggle the value of the layers specified in mask. |
| `tokenize` | 1 | `crates/chronicle-tabular-engine/src/formula/lexer.rs:53` | - |  |
| `tokenize` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:203` | - |  |
| `tokenize` | 1 | `crates/layout-nodegraph-compiler/src/math_parser/lexer.rs:20` | - |  |
| `tokenize` | 1 | `crates/layout-presentation-deck/src/timeline/path.rs:17` | - |  |
| `tolerance` | 1 | `crates/vector-surface-brush/src/calligraphic.rs:135` | - |  |
| `tone_is_masked_less_than_noise` | 0 | `crates/audio-aac-codec/src/encoder/psy.rs:157` | - |  |
| `tone_map_preserves_hue_and_maps_peak` | 0 | `crates/color-transforms/src/transform.rs:423` | - |  |
| `tool` | 1 | `crates/media-isobmff/tests/common/mod.rs:10` | - | Find a tool silently (see `media_test_support::oracle`). |
| `tool_or_skip` | 3 | `crates/media-test-support/src/oracle.rs:107` | - |  |
| `topological_sort` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:53` | graph | Topologically sort task indices, returning [`SchedError::CycleDetected`] if any cycle exists. |
| `topological_sort` | 1 | `crates/layout-nodegraph-compiler/src/compiler/pipeline.rs:110` | graph |  |
| `torus` | 5 | `crates/profile-authoring/src/geometry.rs:230` | computational geometry |  |
| `impl Grid::total` | 1 | `crates/radiance-color-grading/src/scopes/mod.rs:239` | - |  |
| `impl ScheduledCase::total_clears` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:217` | - |  |
| `impl GeodesicRuler::total_distance_km` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:297` | - |  |
| `impl MotionPath::total_length` | 1 | `crates/layout-presentation-deck/src/timeline/path.rs:146` | - |  |
| `impl WorkloadReport::total_overallocated_days` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:338` | - |  |
| `impl References::total_secs` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:138` | - |  |
| `impl FilterContext::total_texels` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:642` | - |  |
| `total_zeros_and_run_before_are_complete_prefix_codes` | 0 | `crates/video-h264-codec/src/cavlc.rs:217` | - |  |
| `tounicode_stream` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:437` | - |  |
| `trace` | 4 | `crates/drafting-geometry/src/region.rs:174` | 3d, graph | Trace the face left of half-edge u→v (vertex indices), giving up after `max_steps` edges. |
| `impl Waveform::trace` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:261` | - |  |
| `trace` | 2 | `crates/vector-radiance-trace/src/lib.rs:347` | - | Trace `img` with `params`. |
| `trace_line` | 4 | `crates/vector-radiance-trace/src/centerline.rs:278` | - |  |
| `trace_mask` | 3 | `crates/vector-radiance-trace/src/contour.rs:91` | - | Trace every component of `mask` (row-major, `w x h`). |
| `trace_rgba` | 4 | `crates/vector-radiance-trace/src/lib.rs:326` | - | High-level function tracing raw RGBA8 slice buffer into vector geometry. |
| `impl AssetHost<'_>::trace_vector_contours` | 2 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:264` | - | 130: `hypermedia.radiance.trace_vector_contours` |
| `trace_within` | 3 | `crates/vector-radiance-trace/src/lib.rs:352` | - | Trace `img` with `params`, bounded by `max_anchors`. |
| `track_config` | 3 | `crates/media-isobmff/tests/oracle_mux.rs:24` | - |  |
| `track_klt_point` | 6 | `crates/kinetics-motion-compositor/src/track/klt.rs:148` | - | Track a point from frame I to frame J using pyramidal Lucas-Kanade. |
| `impl Mp4File::track_of_kind` | 2 | `crates/media-isobmff/src/demux.rs:240` | - | First track of the given kind. |
| `track_point_ncc` | 5 | `crates/kinetics-motion-compositor/src/track/correlation.rs:43` | - | Match template inside target image within search window using Normalized Cross-Correlation. |
| `tracking_ligature_limits` | 0 | `crates/vector-glyph-shaping/src/features.rs:140` | - |  |
| `tracks` | 2 | `crates/vector-surface-brush/src/track.rs:155` | - | Flatten `bp` into tracks (one per subpath) with `tol` accuracy. |
| `impl AssetHost<'_>::transcode_geospatial_vector` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:483` | - | 122: `hypermedia.locus.transcode_geospatial_vector` (`locus-geospatial-codecs`) |
| `transfer_nearest_vertex_colors` | 3 | `crates/asset-production-vibe/src/manifold_ops.rs:865` | 3d |  |
| `transfer_roundtrips` | 0 | `crates/color-transforms/src/lib.rs:345` | - |  |
| `impl DrawGlyphs<'a>::transform` | 2 | `crates/vector-compute-renderer/src/scene.rs:506` | numerics | Sets the global transform. This is applied to all glyphs after the offset |
| `impl Anchor::transform` | 2 | `crates/vector-locus-geometry/src/path.rs:133` | - |  |
| `impl PathData::transform` | 2 | `crates/vector-locus-geometry/src/path.rs:453` | - |  |
| `transform_apply` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:668` | - |  |
| `transform_apply` | 2 | `crates/vector-compute-shaders/shader/shared/transform.wgsl:11` | 2d |  |
| `transform_identity` | 0 | `crates/vector-compute-shaders/shader/flatten.wgsl:651` | - |  |
| `transform_inverse` | 1 | `crates/vector-compute-shaders/shader/shared/transform.wgsl:15` | - |  |
| `transform_moves_bounds` | 0 | `crates/vector-locus-geometry/src/path.rs:600` | - |  |
| `transform_mul` | 2 | `crates/vector-compute-shaders/shader/shared/transform.wgsl:22` | - |  |
| `impl Homography::transform_point` | 2 | `crates/kinetics-motion-compositor/src/track/homography.rs:21` | - | Transform a 2D point [x, y] by this homography. |
| `impl PathData::transformed` | 2 | `crates/vector-locus-geometry/src/path.rs:460` | - |  |
| `impl Scene::transforms` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:265` | - |  |
| `impl Scene::transforms_mut` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:269` | - |  |
| `impl Encoder::transient` | 4 | `crates/audio-aac-codec/src/encoder/mod.rs:332` | - | Transient in the region covered by frame `t`'s short windows. |
| `transition_alignment` | 4 | `crates/timeline-editing/src/transition.rs:111` | - | Infer the alignment stored or shown for a span over the supplied cut. |
| `transition_bounds` | 1 | `crates/timeline-editing/src/transition.rs:79` | - | Return the allowed interval and cut for the referenced clips. |
| `impl Mat3::translate` | 1 | `crates/drafting-geometry/src/mat.rs:25` | 2d |  |
| `translate` | 1 | `crates/vector-compute-shaders/src/compile/msl.rs:8` | 3d |  |
| `impl Anchor::translate` | 2 | `crates/vector-locus-geometry/src/path.rs:136` | 2d |  |
| `translated_cube` | 4 | `crates/asset-production-vibe/src/manifold_ops.rs:1215` | - |  |
| `transport_position_snapshot` | 0 | `crates/audio-timebase/src/transport.rs:250` | - |  |
| `transpose` | 1 | `crates/audio-opus-codec/src/celt/tables.rs:41` | - |  |
| `impl Matrix::transpose` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:180` | - |  |
| `trees` | 0 | `crates/audio-aac-codec/src/huffman.rs:101` | - |  |
| `tri_area_2d` | 3 | `crates/asset-production-vibe/src/kinetics_sim/nav_nla_zones.rs:398` | - |  |
| `impl JournalLedger::trial_balance_totals` | 1 | `crates/chronicle-fiduciary-ledger/src/journal.rs:197` | - | Calculates total debits and total credits across all posted entries. |
| `triangle` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:3` | computational geometry |  |
| `triangle` | 0 | `crates/profile-authoring/src/tests.rs:5` | computational geometry |  |
| `triangle_bounds` | 2 | `crates/surface-pbr-studio/src/bvh.rs:195` | computational geometry |  |
| `impl Mesh::triangle_count` | 1 | `crates/surface-pbr-studio/src/mesh.rs:35` | - |  |
| `impl Oscillator::triangle_wave` | 1 | `crates/audio-device-cpal/examples/synth_tones.rs:79` | - |  |
| `impl Poly::triangles` | 1 | `crates/manifold-bim-csg/src/csg.rs:107` | - |  |
| `triangulate_2d` | 1 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:51` | computational geometry | Compute 2D Delaunay triangulation via incremental point insertion. |
| `triangulate_four_points` | 0 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:147` | computational geometry |  |
| `trigger` | 0 | `crates/audio-device-cpal/src/host/alsa/mod.rs:155` | - |  |
| `trim` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:72` | - |  |
| `triplets_and_dots` | 0 | `crates/audio-timebase/src/grid.rs:262` | - |  |
| `impl TruePeakMeter::true_peak_db` | 1 | `crates/acoustics-daw-session/src/dsp/meter.rs:168` | - |  |
| `truncations_never_panic` | 0 | `crates/media-isobmff/tests/fuzz.rs:154` | - |  |
| `try_border` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:146` | graph | Checked soft band on both sides of the mask edge, following PhotoCraft's measured source oracle. |
| `try_border` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:131` | graph | Checked soft band on both sides of the mask edge, following PhotoCraft's measured source oracle. |
| `try_border` | 5 | `crates/raster-selection-morphology/src/lib.rs:206` | graph | Checked soft band on both sides of the mask edge. |
| `try_contract` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:140` | - | Checked Euclidean contraction, preserving PhotoCraft's inversion/expansion/inversion behavior. |
| `try_contract` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:125` | - | Checked Euclidean contraction, preserving PhotoCraft's inversion/expansion/inversion behavior. |
| `try_contract` | 5 | `crates/raster-selection-morphology/src/lib.rs:194` | - | Checked Euclidean contraction. |
| `impl DrawGlyphs<'a>::try_draw_colr` | 3 | `crates/vector-compute-renderer/src/scene.rs:648` | - |  |
| `try_expand` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:134` | 2d | Checked Euclidean expansion. Caller-selected limits are enforced before mask or EDT buffers |
| `try_expand` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:119` | 2d | Checked Euclidean expansion. Caller-selected limits are enforced before mask or EDT buffers |
| `try_expand` | 5 | `crates/raster-selection-morphology/src/lib.rs:182` | - | Checked Euclidean expansion. Caller-selected limits are enforced before mask or EDT buffers |
| `impl WebGlRendererInit::try_finish` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1069` | 3d | Polls shader compilation and program linking without blocking when |
| `impl WebGlPendingProbe::try_finish` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:200` | - | Try to finish the probe. |
| `impl Schedule::try_new` | 8 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:166` | - |  |
| `impl DeviceHandles::try_open` | 3 | `crates/audio-device-cpal/src/host/alsa/mod.rs:203` | - | Get a mutable reference to the `Option` for a specific `stream_type`. |
| `impl RenderContext for PixelsWindowRenderer<Renderer>::try_register_custom_resource` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:129` | - |  |
| `impl RenderContext for VelloScenePainter<'_, '_>::try_register_custom_resource` | 2 | `crates/vector-gpu-render-adapter/src/scene.rs:19` | - |  |
| `impl RenderContext for VelloWindowRenderer::try_register_custom_resource` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:199` | - |  |
| `impl RenderContext for VelloHybridScenePainter<'_>::try_register_custom_resource` | 2 | `crates/vector-hybrid-render-adapter/src/scene.rs:158` | - |  |
| `impl RenderContext for VelloHybridWindowRenderer::try_register_custom_resource` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:210` | - |  |
| `impl GpuProfiler::try_reserve_query_pair` | 1 | `crates/gpu-command-profiler/src/profiler.rs:591` | - |  |
| `impl SupportedStreamConfigRange::try_with_sample_rate` | 2 | `crates/audio-device-cpal/src/lib.rs:648` | - | Retrieve a [`SupportedStreamConfig`] with the given sample rate and buffer size. |
| `tsp_2opt_solver` | 1 | `crates/geodesy-spatial-analysis/src/network_topology.rs:207` | - | Traveling Salesperson Problem (TSP) solver using Nearest-Neighbor tour + 2-Opt heuristic optimization. |
| `turn_deg` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:71` | - |  |
| `turn_deg` | 2 | `crates/vector-radiance-trace/src/fit.rs:275` | - |  |
| `tweak` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:137` | 2d | Tweak: random displacement of anchors and/or control points. |
| `twist` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:173` | 2d | Twist: rotation strongest at center, fading to zero at bounding circle. |
| `twist_alters_coordinates` | 0 | `crates/vector-locus-pathops/src/effects/distort.rs:270` | - |  |
| `two` | 0 | `crates/vector-locus-geometry/src/perspective.rs:54` | - |  |
| `two_byte_sizes` | 0 | `crates/audio-opus-codec/src/packet.rs:316` | - |  |
| `two_clip_context` | 1 | `crates/timeline-editing/src/transition.rs:280` | - |  |
| `two_colour_stripes_without_ignore_white_trace_both_colours` | 0 | `crates/vector-radiance-trace/src/tests.rs:93` | - |  |
| `two_point_to_unit_line` | 2 | `crates/vector-compute-shaders/src/cpu/draw_leaf.rs:233` | 2d |  |
| `two_slice_picture_decodes_and_changed_picture_identity_is_refused` | 0 | `crates/video-h264-preview/src/lib.rs:540` | - |  |
| `type_aliases_work` | 0 | `crates/vector-locus-geometry/src/lib.rs:133` | - |  |
| `type_name` | 2 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:204` | - |  |
| `impl AssetHost<'_>::typeset_paged_foliation` | 2 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:195` | - | 133: `hypermedia.layout.typeset_paged_foliation` (`layout-paged-typesetter`) |
| `impl Set::u` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:85` | - |  |
| `impl Cur<'a>::u16` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:159` | - |  |
| `impl Reader<'a>::u16` | 2 | `crates/color-transforms/src/icc/profile.rs:311` | - |  |
| `impl Cur<'a>::u16` | 1 | `crates/media-isobmff/src/bytes.rs:98` | - |  |
| `impl BoxBuf::u16` | 2 | `crates/media-isobmff/src/bytes.rs:283` | - |  |
| `u16_to_u8_lattice` | 1 | `crates/asset-production-vibe/src/graphics.rs:60` | - |  |
| `impl Cur<'a>::u24` | 1 | `crates/media-isobmff/src/bytes.rs:104` | - |  |
| `impl Cur<'a>::u32` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:162` | - |  |
| `impl Reader<'a>::u32` | 2 | `crates/color-transforms/src/icc/profile.rs:315` | - |  |
| `impl Cur<'a>::u32` | 1 | `crates/media-isobmff/src/bytes.rs:108` | - |  |
| `impl BoxBuf::u32` | 2 | `crates/media-isobmff/src/bytes.rs:289` | - |  |
| `u32_at` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:364` | - |  |
| `u32_to_extended` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:190` | - |  |
| `impl Cur<'a>::u64` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:165` | - |  |
| `impl Cur<'a>::u64` | 1 | `crates/media-isobmff/src/bytes.rs:114` | - |  |
| `impl BoxBuf::u64` | 2 | `crates/media-isobmff/src/bytes.rs:295` | - |  |
| `u64_at` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:368` | - |  |
| `u64_field` | 2 | `crates/asset-production-vibe/src/operations_catalog.rs:1052` | - |  |
| `impl Reader<'a>::u8` | 2 | `crates/color-transforms/src/icc/profile.rs:308` | - |  |
| `impl Cur<'a>::u8` | 1 | `crates/media-isobmff/src/bytes.rs:95` | - |  |
| `impl BoxBuf::u8` | 2 | `crates/media-isobmff/src/bytes.rs:280` | - |  |
| `u8_operators_behave_consistently` | 0 | `crates/raster-blend-operators/src/lib.rs:541` | - |  |
| `u8_to_u16_lattice` | 1 | `crates/asset-production-vibe/src/graphics.rs:55` | - |  |
| `impl ArcLengthTable::u_at_progress` | 2 | `crates/kinetics-motion-compositor/src/keyframe/spatial.rs:57` | - | Map arc-length progress fraction s in [0, 1] to Bézier parameter `u` in [0, 1]. |
| `impl RangeDecoder<'a>::uint` | 2 | `crates/audio-opus-codec/src/range.rs:190` | - | Decodes a uniformly distributed integer in `[0, ft)` (`ft >= 2`). |
| `impl RangeEncoder::uint` | 3 | `crates/audio-opus-codec/src/range.rs:342` | - |  |
| `uint` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:121` | - | Big-endian unsigned integer of 0..=8 bytes. |
| `impl Parser<'_>::uint` | 1 | `crates/manifold-bim-csg/src/step.rs:190` | - |  |
| `impl Cur<'a>::ul` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:168` | - |  |
| `ul` | 1 | `crates/chronicle-broadcast-containers/src/mxf/labels.rs:68` | - |  |
| `impl Set::ul` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:97` | - |  |
| `ul_of` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:195` | - |  |
| `impl Parser<'_>::unary` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:94` | - |  |
| `unclosed_environments_reported` | 0 | `crates/discourse-academic-tex/src/texparse/structure.rs:441` | - |  |
| `unclosed_envs_close_innermost_first` | 0 | `crates/discourse-academic-tex/src/review/ai_repair.rs:258` | - |  |
| `undefined_command_gets_hint` | 0 | `crates/discourse-academic-tex/src/review/logrules.rs:201` | - |  |
| `undirected_edge_slot` | 3 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:28` | graph |  |
| `impl GeodesicRuler::undo` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:233` | - |  |
| `impl CatalogJournal::undo` | 1 | `crates/radiance-camera-raw/src/metadata.rs:186` | - |  |
| `impl CowTerrainTileGrid::undo` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:228` | - |  |
| `impl CowTerrainTileGrid::undo` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:274` | - |  |
| `impl CowTerrainTileGrid::undo` | 1 | `crates/terrain-brushes/src/lib.rs:413` | - |  |
| `impl CowTerrainTileGrid::undo_history` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:30` | - |  |
| `impl CowTerrainTileGrid::undo_history` | 1 | `crates/terrain-brushes/src/lib.rs:211` | - |  |
| `unescape_rbsp` | 1 | `crates/bitstream-codec/src/lib.rs:252` | - | Remove emulation-prevention bytes (`00 00 03` → `00 00`) from a NAL payload. |
| `unescape_xml` | 1 | `crates/chronicle-office-codecs/src/xml/dom.rs:147` | - |  |
| `unescape_xml` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:23` | - |  |
| `impl Padding::uniform` | 1 | `crates/layout-box-autolayout/src/types.rs:133` | - |  |
| `impl SliceDecoder<'a>::uniform_motion` | 1 | `crates/video-h264-codec/src/slicedec.rs:815` | - | All 16 blocks of the current MB share references and motion vectors. |
| `impl Dsu::union` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:114` | - |  |
| `impl RangeRef::union` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:211` | - |  |
| `impl Bounds2::union` | 2 | `crates/drafting-geometry/src/bounds.rs:40` | - |  |
| `union` | 3 | `crates/manifold-bim-csg/src/csg.rs:363` | - | Unions two solids together. |
| `impl Bounds::union` | 2 | `crates/surface-pbr-studio/src/math.rs:324` | - |  |
| `union_opt` | 2 | `crates/vector-locus-geometry/src/lib.rs:48` | 2d | Union of two optional rects. |
| `union_options` | 0 | `crates/vector-locus-geometry/src/lib.rs:124` | - |  |
| `unique` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:94` | - |  |
| `unit` | 1 | `crates/profile-authoring/src/frames.rs:9` | - |  |
| `unit` | 1 | `crates/vector-locus-geometry/src/bez.rs:62` | 2d | `v` normalised, or +x when it has no length. |
| `unit` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:11` | 2d | Unit vector or `None` if nearly zero. |
| `impl V3::unit` | 1 | `crates/vector-locus-pathops/src/three_d/revolve.rs:28` | - |  |
| `unit` | 1 | `crates/vector-radiance-trace/src/fit.rs:398` | 2d |  |
| `unit` | 1 | `crates/vector-surface-brush/src/track.rs:23` | 2d |  |
| `impl Rng::unit` | 1 | `crates/vector-surface-brush/src/track.rs:196` | - | Uniform in [0, 1). |
| `unit_2d` | 1 | `crates/manifold-bim-csg/src/parametric.rs:36` | - |  |
| `unit_cube_mesh` | 1 | `crates/asset-production-vibe/src/manifold_ops.rs:1183` | computational geometry |  |
| `unit_icosphere_mesh` | 1 | `crates/asset-production-vibe/src/manifold_ops.rs:1428` | computational geometry |  |
| `unit_named` | 0 | `crates/vector-locus-geometry/src/units.rs:218` | - |  |
| `unite_all` | 1 | `crates/vector-locus-pathops/src/pathops/boolean.rs:537` | - | Execute N-ary union of all input paths. |
| `unpack4x8unorm` | 1 | `crates/vector-compute-shaders/src/cpu/fine.rs:34` | - |  |
| `unquant_coarse_energy` | 7 | `crates/audio-opus-codec/src/celt/energy.rs:48` | - | `unquant_coarse_energy`. |
| `unquant_energy_finalise` | 8 | `crates/audio-opus-codec/src/celt/energy.rs:92` | - |  |
| `unquant_fine_energy` | 6 | `crates/audio-opus-codec/src/celt/energy.rs:77` | - |  |
| `impl RenderContext for PixelsWindowRenderer<Renderer>::unregister_resource` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:136` | - |  |
| `impl RenderContext for VelloScenePainter<'_, '_>::unregister_resource` | 2 | `crates/vector-gpu-render-adapter/src/scene.rs:38` | - |  |
| `impl RenderContext for VelloWindowRenderer::unregister_resource` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:217` | - |  |
| `impl RenderContext for VelloHybridScenePainter<'_>::unregister_resource` | 2 | `crates/vector-hybrid-render-adapter/src/scene.rs:185` | - |  |
| `impl RenderContext for VelloHybridWindowRenderer::unregister_resource` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:241` | - |  |
| `impl Renderer::unregister_texture` | 2 | `crates/vector-compute-renderer/src/lib.rs:603` | - | Unregister a [`wgpu::Texture`] that was registered with [`register_texture`](Self::register_texture). |
| `unsharp_mask_rgba8` | 7 | `crates/raster-spatial-operators/src/convolve.rs:300` | - | Unsharp mask sharpening for RGBA8 buffer. |
| `unsupported` | 1 | `crates/video-h264-codec/src/error.rs:26` | - |  |
| `impl Parser<'_>::until` | 2 | `crates/manifold-bim-csg/src/step.rs:245` | - |  |
| `unused_bindings_do_not_resolve_passes` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:619` | - |  |
| `impl AssetHost<'_>::unwrap_atlas_parameterize` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/slim_mikktspace.rs:340` | - |  |
| `unwrap_phase` | 1 | `crates/electromagnetics-rf-sensing/src/csi_signal.rs:124` | - | Unwraps phase values along a sequence to remove 2pi phase wrap discontinuities. |
| `impl AssetHost<'_>::unwrap_slim` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/slim_mikktspace.rs:21` | - |  |
| `impl AssetHost<'_>::unwrap_slim_parameterize` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/slim_mikktspace.rs:34` | - |  |
| `impl RangeDecoder<'a>::update` | 4 | `crates/audio-opus-codec/src/range.rs:128` | - | Consumes the symbol with cumulative range `[fl, fh)` of total `ft`. |
| `impl FallDetector::update` | 3 | `crates/electromagnetics-rf-sensing/src/vitals.rs:162` | physics | Feeds max Doppler velocity (m/s) observed in the current frame and updates fall status. |
| `trait Block::update` | 3 | `crates/kinetics-block-simulation/src/block.rs:65` | - |  |
| `impl Block for UnitDelay::update` | 3 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:48` | - |  |
| `impl Block for ZeroOrderHold::update` | 3 | `crates/kinetics-block-simulation/src/blocks/discrete.rs:103` | - |  |
| `impl PocState::update` | 3 | `crates/video-h264-codec/src/slice.rs:557` | - | Update after a picture has been decoded. `poc` is the value returned by [`PocState::compute`] |
| `impl RangeDecoder<'a>::update_bin` | 4 | `crates/audio-opus-codec/src/range.rs:138` | - | Like [`Self::update`] for a symbol decoded with [`Self::decode_bin`]. |
| `impl JohnsonCookDamage::update_damage` | 6 | `crates/kinetics-continuum-multiphysics/src/explicit/damage.rs:43` | - | Update damage index D = sum(delta_eps_p / eps_f). Returns true if element is eroded (D >= 1.0). |
| `impl Sim::update_discrete` | 2 | `crates/kinetics-block-simulation/src/sim.rs:156` | - |  |
| `impl PocState::update_gap_frame` | 3 | `crates/video-h264-codec/src/slice.rs:578` | - | Account for a "non-existing" frame inferred by the frame_num gap process (8.2.5.2). |
| `impl Programs::update_scratch_bind_groups` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1862` | - |  |
| `impl Recording::upload` | 3 | `crates/vector-compute-renderer/src/recording.rs:124` | - | Commands to upload the given data to a new buffer with the given name. |
| `impl WebGlPrograms::upload_alpha_texture` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1922` | - | Upload alpha data to the texture. |
| `impl Programs::upload_alpha_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2374` | - | Upload alpha data to the texture. |
| `impl WebGlPrograms::upload_blend_instances` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1757` | - |  |
| `impl WebGlPrograms::upload_copy_instances` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1769` | - |  |
| `impl WebGlPrograms::upload_encoded_paints_texture` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1956` | - | Upload encoded paints to the texture. |
| `impl Programs::upload_encoded_paints_texture` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2419` | - | Upload encoded paints to the texture. |
| `impl WebGlPrograms::upload_filter_data_texture` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1715` | - | Upload filter data to the texture. |
| `impl WebGlPrograms::upload_filter_instances` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1741` | - |  |
| `impl Programs::upload_filter_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2464` | - |  |
| `impl WebGlPrograms::upload_gradient_texture` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1983` | - | Upload gradient data to the texture. |
| `impl Programs::upload_gradient_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2499` | - | Upload gradient data to the texture. |
| `impl BindMapBuffer::upload_if_needed` | 5 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1020` | - |  |
| `impl Recording::upload_image` | 5 | `crates/vector-compute-renderer/src/recording.rs:142` | - | Commands to upload the given data to a new image with the given dimensions and format. |
| `impl ImageManager<'a>::upload_image` | 2 | `crates/vector-hybrid-render-adapter/src/scene.rs:93` | - |  |
| `impl WebGlImageManager<'a>::upload_image` | 2 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:36` | - |  |
| `impl WebGlRenderer::upload_image` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:720` | - | Upload image to cache and atlas in one step. Returns the `ImageId`. |
| `impl Renderer::upload_image` | 6 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:519` | - | Upload image to cache and atlas in one step. Returns the `ImageId`. |
| `impl WebGlRenderer::upload_image_with` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:728` | - |  |
| `impl Renderer::upload_image_with` | 7 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:537` | - |  |
| `upload_layer_config_buffer` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2395` | - |  |
| `upload_rgba32ui_rows` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3504` | - | Upload full rows of RGBA32UI data into immutable texture storage. |
| `impl WebGlPrograms::upload_strip_pair` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2034` | - | Uploads two strip slices (opaque then alpha) into a single GPU buffer. |
| `impl Programs::upload_strip_pair` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2542` | - | Uploads two strip slices (opaque then alpha) into the frame arena and |
| `impl Recording::upload_uniform` | 3 | `crates/vector-compute-renderer/src/recording.rs:133` | - | Commands to upload the given data to a new buffer as a uniform with the given name. |
| `impl RefMedia::uploading` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:59` | - |  |
| `upper` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:78` | - |  |
| `upsample` | 3 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:89` | - |  |
| `upsample_sine_and_delay` | 0 | `crates/audio-opus-codec/src/silk/resampler.rs:176` | - |  |
| `impl ViewFramebuffer::use_depth_buffer` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1223` | - |  |
| `impl WgpuEngine::use_parallel_initialisation` | 1 | `crates/vector-compute-renderer/src/wgpu_engine.rs:154` | - | Enable creating any remaining shaders in parallel |
| `impl Sheet::used_range` | 1 | `crates/chronicle-tabular-engine/src/calc/model.rs:116` | - |  |
| `impl CalligraphicBrush::uses_pressure` | 1 | `crates/vector-surface-brush/src/calligraphic.rs:58` | - |  |
| `utf16_of` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:218` | - |  |
| `uv_island_hull` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:202` | - | Monotone-chain hull of a SLIM UV island. Sorted, so iteration is produce-safe. |
| `v3` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:22` | - |  |
| `v_add` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:17` | - |  |
| `v_cross` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:37` | - |  |
| `v_cross` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:30` | - |  |
| `v_dot` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:32` | - |  |
| `v_dot` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:25` | - |  |
| `v_len` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:39` | - |  |
| `v_len_sq` | 1 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:46` | - |  |
| `v_norm` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:44` | - |  |
| `v_normalize` | 1 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:51` | - |  |
| `v_scale` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:27` | - |  |
| `v_sub` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:22` | - |  |
| `v_sub` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:20` | - |  |
| `impl ModelInfo::valid_batch` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:142` | - |  |
| `valid_bpm` | 1 | `crates/audio-timebase/src/tempo.rs:118` | - |  |
| `valid_config` | 2 | `crates/audio-device-cpal/src/host/emscripten/mod.rs:408` | - |  |
| `valid_config` | 2 | `crates/audio-device-cpal/src/host/webaudio/mod.rs:504` | - |  |
| `valid_edit_returns_frame_exact_span_and_alignment` | 0 | `crates/timeline-editing/src/transition.rs:308` | - |  |
| `valid_name` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:46` | - |  |
| `impl ByteRange::validate` | 2 | `crates/discourse-pdf-compliance/src/sign/pades.rs:25` | - | Verifies that the ByteRange covers the file cleanly without overlaps, |
| `impl GpuProfilerSettings::validate` | 1 | `crates/gpu-command-profiler/src/profiler_settings.rs:50` | - |  |
| `impl IntermediateTextureRequirements::validate` | 3 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:251` | - | Validate that satisfying these requirements from the backend's existing allocations stays |
| `impl Rounds::validate` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:130` | - | Validate the texture-binding invariants assumed by the render backends. |
| `impl Round::validate` | 3 | `crates/vector-hybrid-renderer/src/schedule/round.rs:269` | - | Validate the bindings required by this round's scheduled work. |
| `validate` | 1 | `crates/vector-hybrid-shaders/src/compile.rs:109` | - |  |
| `validate` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:175` | - | Validates the renamed module and produces the metadata required by Naga's writer. |
| `impl PerspectiveGrid::validate` | 1 | `crates/vector-locus-geometry/src/perspective.rs:165` | - |  |
| `validate_field` | 2 | `crates/discourse-pdf-compliance/src/forms/validation.rs:86` | - |  |
| `validate_line_soup` | 1 | `crates/vector-compute-renderer/src/debug/validate.rs:47` | - |  |
| `validate_offsets` | 3 | `crates/video-h264-preview/src/lib.rs:346` | - |  |
| `validate_picture_headers` | 4 | `crates/video-h264-preview/src/lib.rs:431` | - | Match the codec's own primary-picture boundary test before it can create |
| `validate_q42_logic` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:374` | - | Validates a `Q42LOGIC` container: magic, version, length, per-kind counts, payload digest and |
| `validate_resource_texture_size` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2345` | - | Validate that a resource texture fits within its configured dimension limit. |
| `validate_results` | 3 | `crates/gpu-command-profiler/tests/src/mod.rs:64` | - |  |
| `validate_results_unordered` | 3 | `crates/gpu-command-profiler/tests/src/mod.rs:87` | - |  |
| `validate_rule_citation` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:121` | - |  |
| `impl CowTerrainTileGrid::validate_sample` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:93` | - |  |
| `impl CowTerrainTileGrid::validate_sample` | 1 | `crates/terrain-brushes/src/lib.rs:274` | - |  |
| `validate_self_contained_data` | 1 | `crates/media-isobmff/src/demux.rs:558` | - | Narrow local-file profile; no external URLs, aliases, extra entries or duplicate tables. |
| `validate_sizes` | 5 | `crates/video-h264-preview/src/lib.rs:378` | - |  |
| `validate_tags` | 1 | `crates/media-ogg/src/strict_opus.rs:17` | - |  |
| `validate_track` | 1 | `crates/media-isobmff/src/mux.rs:490` | - |  |
| `validator_accepts_bound_work` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:661` | - |  |
| `validator_rejects_unbound_clear` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:692` | - |  |
| `validator_rejects_unbound_layer_pass` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:679` | - |  |
| `impl Sheet::value` | 2 | `crates/chronicle-tabular-engine/src/calc/model.rs:100` | - |  |
| `impl Parser::value` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:285` | - |  |
| `trait Scalar::value` | 1 | `crates/kinetics-garment-draping/src/math.rs:241` | - |  |
| `impl Scalar for f64::value` | 1 | `crates/kinetics-garment-draping/src/math.rs:252` | - |  |
| `impl Scalar for Jet<N>::value` | 1 | `crates/kinetics-garment-draping/src/math.rs:300` | - |  |
| `impl Parser<'_>::value` | 2 | `crates/manifold-bim-csg/src/step.rs:256` | - |  |
| `var_s` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:285` | - |  |
| `impl Jet<N>::variable` | 2 | `crates/kinetics-garment-draping/src/math.rs:274` | - |  |
| `variance_range` | 3 | `crates/geodesy-spatial-analysis/src/cartography.rs:171` | - |  |
| `vbd_step` | 2 | `crates/kinetics-garment-draping/src/vbd.rs:58` | computational geometry | Executes one VBD simulation step. |
| `impl EncoderConfig::vbr` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:72` | - |  |
| `vbr_quality_scale` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:249` | - |  |
| `vdb` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:442` | - |  |
| `vec` | 3 | `crates/kinetics-block-simulation/src/blocks/params.rs:25` | - |  |
| `vec3` | 3 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:714` | 3d |  |
| `vec3` | 3 | `crates/asset-production-vibe/src/manifold_ops.rs:1179` | 3d |  |
| `vec3` | 3 | `crates/asset-production-vibe/src/operations_catalog.rs:322` | 3d |  |
| `vec_len` | 1 | `crates/kinetics-continuum-multiphysics/src/mesh/quality.rs:84` | - |  |
| `vec_sub` | 2 | `crates/kinetics-continuum-multiphysics/src/mesh/quality.rs:80` | - |  |
| `vector` | 1 | `crates/asset-production-vibe/src/lib.rs:550` | - |  |
| `impl AssetHost<'_>::vector_render` | 2 | `crates/asset-production-vibe/src/graphics.rs:517` | - |  |
| `vectorised_transforms_match_reference` | 0 | `crates/video-h264-codec/src/transform.rs:208` | - |  |
| `vectorscope` | 4 | `crates/radiance-color-grading/src/scopes/paint.rs:58` | - |  |
| `vectorscope_hls` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:499` | - |  |
| `vectorscope_yuv` | 2 | `crates/radiance-color-grading/src/scopes/mod.rs:475` | - |  |
| `impl PrivacyWitnessReceipt::verify` | 1 | `crates/electromagnetics-rf-sensing/src/densepose.rs:158` | - | Verifies the cryptographic witness receipt integrity and human-rights compliance guarantee. |
| `impl FiduciaryAuditTrail::verify_chain` | 1 | `crates/chronicle-fiduciary-ledger/src/audit.rs:122` | - | Validates the complete cryptographic hash chain from genesis to head. |
| `impl AssetHost<'_>::verify_emf` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:277` | - | Parses and cryptographically verifies the Q42EMF64 container, |
| `impl AssetHost<'_>::verify_fiduciary_rid` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:205` | - |  |
| `impl AssetHost<'_>::verify_m64` | 2 | `crates/asset-production-vibe/src/geodesy_ops.rs:637` | - |  |
| `verify_pades_byte_range` | 3 | `crates/discourse-pdf-compliance/src/sign/pades.rs:77` | - | Verifies that a signed document's byte range digest matches the signed content. |
| `impl OntologicalTriple::verify_parity` | 1 | `crates/chronicle-tabular-engine/src/ontology.rs:32` | - |  |
| `impl OntologicalTriple::verify_parity` | 1 | `crates/layout-paged-typesetter/src/ontology.rs:31` | - |  |
| `impl NQuin48::verify_parity` | 1 | `crates/surface-layered-codecs/src/ontology.rs:39` | - |  |
| `verify_rfc3161_timestamp` | 2 | `crates/discourse-pdf-compliance/src/sign/timestamp.rs:25` | - |  |
| `verify_zero_residue` | 2 | `crates/discourse-pdf-compliance/src/redact/verifier.rs:50` | - | Zero-residue verification pass: |
| `vertebrate_op_accepts_cbor_ld_spec_input` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:414` | - |  |
| `vertebrate_op_accepts_validated_q42_input` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:247` | - |  |
| `vertebrate_op_appends_joint_rules` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:394` | - |  |
| `vertebrate_op_compiles_vibescript_spec_with_rules` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:208` | - |  |
| `vertebrate_op_default_rules_use_spec_species_id` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:227` | - |  |
| `vertebrate_op_rejects_non_vibescript_specs` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:273` | - |  |
| `impl GpuStrip::vertex_attributes` | 0 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1074` | 3d | Vertex attributes for the strip |
| `impl Mesh::vertex_count` | 1 | `crates/surface-pbr-studio/src/mesh.rs:31` | 3d |  |
| `impl Padding::vertical` | 1 | `crates/layout-box-autolayout/src/types.rs:142` | - |  |
| `vibe_record_to_json` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:92` | - |  |
| `vibe_value_to_json` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:174` | - |  |
| `impl SampleEntry::video` | 4 | `crates/media-isobmff/src/codec.rs:1228` | - | Convenience constructor for a video entry. |
| `impl Track::video` | 1 | `crates/media-isobmff/src/demux.rs:129` | - |  |
| `video_page_models` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:501` | - |  |
| `video_projects_combined_av_without_changing_source_or_offsets` | 0 | `crates/video-h264-preview/src/lib.rs:662` | - |  |
| `video_requires_local_data_reference_and_entry_binding` | 0 | `crates/video-h264-preview/src/lib.rs:674` | - |  |
| `impl ScheduledCase::views` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:163` | - |  |
| `vincenty_wgs84_km` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:55` | - | Vincenty inverse geodesic calculation on the WGS84 ellipsoid. |
| `vint_len` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:4` | - |  |
| `visible_faces` | 1 | `crates/vector-locus-pathops/src/three_d/visibility.rs:144` | - | Remove covered portions of opaque faces in back-to-front paint order. |
| `impl serde::de::Visitor<'de> for V::visit_map` | 2 | `crates/vector-locus-geometry/src/path.rs:92` | - |  |
| `impl serde::de::Visitor<'de> for V::visit_seq` | 2 | `crates/vector-locus-geometry/src/path.rs:87` | - |  |
| `visual_reorder_line` | 2 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:47` | - | UAX #9 L2 visual order for a laid-out line. |
| `visual_reorder_line_i1_n1_an_matches_bidi_msdf` | 0 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:429` | - |  |
| `vivid_light` | 2 | `crates/raster-blend-operators/src/lib.rs:403` | - |  |
| `vivid_light_generic` | 2 | `crates/raster-blend-operators/src/lib.rs:437` | - |  |
| `vivid_light_generic` | 2 | `crates/raster-blend-operators/src/migration_oracle.rs:41` | - |  |
| `vivid_light_ps` | 2 | `crates/raster-blend-operators/src/migration_oracle.rs:6` | - |  |
| `vivid_light_source_extremes_win` | 0 | `crates/raster-blend-operators/src/lib.rs:517` | - |  |
| `vivid_light_u8` | 2 | `crates/raster-blend-operators/src/lib.rs:50` | - | Byte-quantized Vivid Light operator in `0..=255`. |
| `impl Reader<'a>::vlc` | 2 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:74` | - |  |
| `vlc` | 1 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:260` | - |  |
| `vlog_decode` | 1 | `crates/color-transforms/src/log.rs:152` | - |  |
| `vlog_decode` | 1 | `crates/radiance-color-grading/src/log.rs:153` | - |  |
| `vlog_encode` | 1 | `crates/color-transforms/src/log.rs:149` | - |  |
| `vlog_encode` | 1 | `crates/radiance-color-grading/src/log.rs:145` | - |  |
| `vlookup` | 1 | `crates/chronicle-tabular-engine/src/functions/lookup.rs:139` | - |  |
| `void` | 2 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:108` | - |  |
| `volume` | 1 | `crates/manifold-bim-csg/src/csg.rs:410` | computational geometry | Signed volume of a closed triangle mesh. |
| `volume` | 1 | `crates/profile-authoring/provenance/original-spatial-tests.rs:25` | computational geometry |  |
| `volume` | 1 | `crates/profile-authoring/src/tests.rs:29` | computational geometry |  |
| `impl HevcConfig::vps` | 1 | `crates/media-isobmff/src/codec.rs:415` | - |  |
| `vs_main` | 2 | `crates/gpu-command-profiler/examples/shader.wgsl:7` | 3d |  |
| `vs_main` | 1 | `crates/pixel-buffer-output/shaders/scale.wgsl:14` | 3d, 2d |  |
| `vs_main` | 1 | `crates/pixel-buffer-output/shaders/scale_fill.wgsl:14` | 3d, 2d |  |
| `vtap_row` | 3 | `crates/video-h264-codec/src/inter.rs:86` | - | Vertical 6-tap (unclipped) over six rows at columns 0..n. |
| `vtu_xml_structure_test` | 0 | `crates/kinetics-continuum-multiphysics/src/interchange/vtk.rs:82` | - |  |
| `impl Frame::wait_complete` | 1 | `crates/video-h264-codec/src/picture.rs:177` | - | Wait until the whole frame is available. |
| `wait_for_handle_signal` | 1 | `crates/audio-device-cpal/src/host/wasapi/stream.rs:286` | - |  |
| `impl std::task::Wake for NullWake::wake` | 1 | `crates/vector-compute-renderer/src/util.rs:250` | - |  |
| `impl TriggerSender::wakeup` | 1 | `crates/audio-device-cpal/src/host/alsa/mod.rs:140` | - |  |
| `impl Expr::walk` | 2 | `crates/chronicle-tabular-engine/src/formula/ast.rs:172` | - |  |
| `walk` | 6 | `crates/video-h264-preview/src/lib.rs:204` | - |  |
| `wall_axis_straight_and_arc` | 0 | `crates/manifold-bim-csg/src/parametric.rs:329` | - |  |
| `warm_cinema_preset` | 1 | `crates/asset-production-vibe/src/radiance_color.rs:10` | - |  |
| `warp` | 4 | `crates/vector-locus-pathops/src/effects/warp.rs:116` | computational geometry, 2d | Apply a warp envelope to `path` within bounding box `b`. |
| `warp_path_geometry` | 2 | `crates/vector-surface-brush/src/art.rs:151` | - |  |
| `warp_path_transforms_rectangle` | 0 | `crates/vector-locus-pathops/src/effects/warp.rs:173` | - |  |
| `warp_point` | 6 | `crates/vector-locus-pathops/src/effects/warp.rs:59` | - | The warp map on normalized coordinates (x, y) in [-1, 1]^2: |
| `wav_spec_from_config` | 1 | `crates/audio-device-cpal/examples/record_wav.rs:153` | - |  |
| `wave12_living_world_morphisms_roundtrip` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:366` | - |  |
| `wave13_domain_extension_morphisms_roundtrip` | 0 | `crates/asset-production-vibe/src/domain_extensions.rs:94` | - |  |
| `waveform` | 3 | `crates/radiance-color-grading/src/scopes/mod.rs:309` | - |  |
| `waveform_reference` | 1 | `crates/radiance-color-grading/src/scopes/paint.rs:54` | - |  |
| `wavelength_to_srgb` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:418` | - |  |
| `impl Task::wbs_depth` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:353` | - |  |
| `webgl_error_name` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:253` | - |  |
| `weibull_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:199` | - |  |
| `impl Priority::weight` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:151` | - |  |
| `impl BlendRange::weight` | 2 | `crates/raster-layer-mask-adapter/src/masks.rs:189` | - | Evaluates visibility weight (0.0..=1.0) for a sample `v` in `0.0..=255.0`. |
| `impl ResampleFilter::weight` | 2 | `crates/raster-spatial-operators/src/resample.rs:36` | - | Continuous weighting function at distance `x`. |
| `impl SliceDecoder<'a>::weight_for` | 3 | `crates/video-h264-codec/src/slicedec.rs:730` | - |  |
| `weighted_check` | 2 | `crates/video-h264-codec/src/synth_tests.rs:473` | - |  |
| `weighted_store` | 8 | `crates/video-h264-codec/src/inter.rs:276` | - | Combine prediction(s) into `dst` (stride `ds`). `p0`/`p1` have stride `ps`. |
| `weighting` | 0 | `crates/video-h264-codec/src/inter.rs:504` | - |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::wgpu_backend` | 2 | `crates/pixel-buffer-output/src/builder.rs:90` | - | Set which backends wgpu will attempt to use. |
| `impl Sinusoidal::wgs84` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:423` | - |  |
| `wgs84_direct` | 4 | `crates/geodesy-spatial-analysis/src/geodesy.rs:275` | - | WGS84 geodesic direct problem: computes destination (lat, lon) and arrival azimuth from start, azimuth, and distance. |
| `wgs84_inverse` | 4 | `crates/geodesy-spatial-analysis/src/geodesy.rs:176` | - | WGS84 geodesic inverse problem: computes distance in metres and initial/final azimuths in degrees. |
| `impl GradeSpace::white_nits` | 1 | `crates/color-transforms/src/grade.rs:55` | - | HDR White in cd/m² (None for SDR). |
| `impl GradeSpace::white_nits` | 1 | `crates/radiance-color-grading/src/grade.rs:47` | - |  |
| `impl Gamut::white_point` | 1 | `crates/radiance-color-grading/src/spaces.rs:52` | - |  |
| `impl GeomRef::whole` | 1 | `crates/locus-parametric-solver/src/solver/model.rs:72` | - |  |
| `wide_layers` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:617` | - |  |
| `wider_frame_number_sps_decodes_within_gap_budget` | 0 | `crates/video-h264-preview/src/lib.rs:532` | - |  |
| `impl RangeRef::width` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:169` | - |  |
| `impl Bounds2::width` | 1 | `crates/drafting-geometry/src/bounds.rs:49` | - |  |
| `impl Extent::width` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:87` | - |  |
| `impl SigVal::width` | 1 | `crates/kinetics-block-simulation/src/signal.rs:108` | - |  |
| `impl PsdLayer::width` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:290` | - |  |
| `trait WebGlAtlasWriter::width` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3308` | - | Get the width of the image. |
| `impl WebGlAtlasWriter for Pixmap::width` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3325` | - |  |
| `impl WebGlAtlasWriter for Arc<Pixmap>::width` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3368` | - |  |
| `impl WebGlAtlasWriter for WebGlTexture::width` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3391` | - |  |
| `impl WebGlAtlasWriter for WebGlTextureWithDimensions::width` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3444` | - |  |
| `trait AtlasWriter::width` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3288` | - | Get the width of the image. |
| `impl AtlasWriter for Texture::width` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3307` | - |  |
| `impl AtlasWriter for Pixmap::width` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3353` | - |  |
| `impl AtlasWriter for Arc<Pixmap>::width` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3401` | - |  |
| `impl Scene::width` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:873` | - | Get the width of the render context. |
| `impl DrawSink for Scene::width` | 1 | `crates/vector-hybrid-renderer/src/text.rs:207` | - |  |
| `impl Sps::width` | 1 | `crates/video-h264-codec/src/params.rs:454` | - | Decoded (uncropped) luma width. |
| `width_of` | 3 | `crates/discourse-academic-tex/src/typeset.rs:443` | - |  |
| `impl Ttf::width_pt` | 3 | `crates/discourse-academic-tex/src/texpdf.rs:164` | - | Advance of `c` in pt at `size`. |
| `widths_array` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:422` | - |  |
| `wild` | 1 | `crates/color-transforms/src/lut.rs:509` | - |  |
| `window` | 2 | `crates/audio-aac-codec/src/mdct.rs:201` | - | Rising half (length N/2) of the window of the given shape; `long` selects N = 2048, else N = 256. |
| `impl ApplicationHandler<()> for State::window_event` | 4 | `crates/gpu-command-profiler/examples/demo.rs:200` | - |  |
| `impl FlexWarpMode::window_params` | 1 | `crates/acoustics-daw-session/src/elastic_warp/mod.rs:20` | - | Window size in seconds and maximum search tolerance fraction for WSOLA. |
| `impl Pixels<'win>::window_pos_to_pixel` | 2 | `crates/pixel-buffer-output/src/lib.rs:677` | 2d | Calculate the pixel location from a physical location on the window, |
| `windows_err_to_cpal_err` | 1 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:88` | - |  |
| `windows_err_to_cpal_err_message` | 2 | `crates/audio-device-cpal/src/host/wasapi/mod.rs:92` | - |  |
| `with_attached` | 1 | `crates/audio-device-cpal/src/host/oboe/android_media.rs:14` | - |  |
| `impl XEl::with_attr` | 3 | `crates/chronicle-office-codecs/src/xml/dom.rs:31` | - |  |
| `impl PolyVertex::with_bulge` | 2 | `crates/drafting-geometry/src/curve.rs:429` | 2d |  |
| `impl XEl::with_child` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:36` | - |  |
| `impl GlyphAtlasResources::with_config` | 3 | `crates/vector-hybrid-renderer/src/text.rs:38` | - |  |
| `impl Encoder::with_config` | 1 | `crates/video-prores-codec/src/encode.rs:60` | - |  |
| `impl LayoutNode::with_constraints` | 3 | `crates/layout-box-autolayout/src/frame.rs:69` | - |  |
| `impl JLine::with_desc` | 2 | `crates/chronicle-fiduciary-ledger/src/journal.rs:44` | - |  |
| `impl WGPUContext::with_features_and_limits` | 2 | `crates/gpu-device-context/src/lib.rs:120` | - |  |
| `with_field` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:292` | - |  |
| `impl AutoLayoutFrame::with_gap` | 2 | `crates/layout-box-autolayout/src/frame.rs:135` | - |  |
| `impl Anchor::with_handles` | 3 | `crates/vector-locus-geometry/src/path.rs:122` | - |  |
| `impl PaintResolver<'a>::with_image_cache` | 2 | `crates/vector-hybrid-renderer/src/paint.rs:90` | - | Add the image cache needed to resolve internal image paints. |
| `impl VelloCpuImageRenderer::with_image_cache_config` | 3 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:12` | - | Create a renderer with a custom image cache configuration. |
| `impl VelloCpuScenePainter::with_image_cache_config` | 3 | `crates/vector-cpu-render-adapter/src/scene.rs:24` | - |  |
| `impl SupportedStreamConfigRange::with_max_sample_rate` | 1 | `crates/audio-device-cpal/src/lib.rs:663` | - | Turns this [`SupportedStreamConfigRange`] into a [`SupportedStreamConfig`] corresponding to the maximum samples rate. |
| `impl Scene::with_optional_filter_or_blend_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:830` | - |  |
| `impl Transform::with_options` | 3 | `crates/color-transforms/src/icc/transform.rs:53` | - |  |
| `impl PixelsWindowRenderer<Renderer>::with_options` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:104` | - |  |
| `impl VelloWindowRenderer::with_options` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:178` | - |  |
| `impl VelloHybridWindowRenderer::with_options` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:165` | - |  |
| `impl SkiaWindowRenderer::with_options` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:98` | - |  |
| `impl PixelsWindowRenderer<Renderer>::with_options_and_renderer` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:113` | - |  |
| `impl Kernel2D::with_origin` | 5 | `crates/raster-spatial-operators/src/convolve.rs:28` | - |  |
| `impl SubElementHooks::with_overrides` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:250` | - | Parses optional sub-element hook overrides from Vibe `Args` on top of `self`. |
| `impl AutoLayoutFrame::with_padding` | 2 | `crates/layout-box-autolayout/src/frame.rs:130` | - |  |
| `impl Shape::with_paragraphs` | 2 | `crates/layout-presentation-deck/src/model/shape.rs:193` | - | One DrawingML paragraph per string so U+000A never lives in a single `a:t`. |
| `impl GpuProfilerQuery::with_parent` | 2 | `crates/gpu-command-profiler/src/profiler_query.rs:106` | - | Makes this scope a child of the passed scope. |
| `with_parity` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:6` | - | Add odd parity bit to a 7-bit value. |
| `impl VelloImageRenderer::with_pipeline_cache` | 3 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:21` | - | Like [`ImageRenderer::new`], but reusing compiled pipelines from a cache |
| `impl PixelsWindowRenderer<Renderer>::with_renderer` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:95` | - |  |
| `impl MixerState::with_reservoir` | 1 | `crates/raster-layer-mask-adapter/src/mixer.rs:54` | - |  |
| `impl SupportedStreamConfigRange::with_sample_rate` | 2 | `crates/audio-device-cpal/src/lib.rs:639` | - | Retrieve a [`SupportedStreamConfig`] with the given sample rate and buffer size. |
| `impl XlsxWorkbook::with_single_sheet` | 2 | `crates/chronicle-office-codecs/src/spreadsheet/xlsx.rs:25` | - |  |
| `impl LayoutNode::with_sizing` | 3 | `crates/layout-box-autolayout/src/frame.rs:63` | - |  |
| `impl TableCellData::with_span` | 2 | `crates/layout-paged-typesetter/src/table_layout/measure.rs:25` | - |  |
| `impl FlowPara::with_text` | 2 | `crates/chronicle-office-codecs/src/flow/model.rs:77` | - |  |
| `impl FlowCell::with_text` | 1 | `crates/chronicle-office-codecs/src/flow/model.rs:96` | - |  |
| `impl XEl::with_text` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:41` | - |  |
| `impl Shape::with_text` | 2 | `crates/layout-presentation-deck/src/model/shape.rs:185` | - |  |
| `impl Decoder::with_threads` | 1 | `crates/video-h264-codec/src/decoder.rs:129` | - | A decoder using up to `threads` worker threads (1 = decode on the calling thread). |
| `impl AutoLayoutFrame::with_wrap` | 2 | `crates/layout-box-autolayout/src/frame.rs:140` | - |  |
| `wobbly` | 3 | `crates/vector-locus-geometry/src/recognize.rs:323` | - |  |
| `impl Parser<'_>::word` | 2 | `crates/manifold-bim-csg/src/step.rs:163` | - |  |
| `impl Meter::work` | 2 | `crates/manifold-bim-csg/src/csg.rs:68` | - |  |
| `impl Calendar::work_day_to_calendar_day` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:433` | - |  |
| `trait Host::workbook` | 1 | `crates/chronicle-tabular-engine/src/calc/eval.rs:37` | - |  |
| `impl Host for PassHost<'_>::workbook` | 1 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:251` | - |  |
| `impl ColorPipeline::working_gamut` | 1 | `crates/color-transforms/src/spaces.rs:356` | numerics | Linear primaries of the compositing space. |
| `workspace_root` | 0 | `crates/media-test-support/src/fixtures.rs:11` | - | The workspace root (the directory holding the top-level `Cargo.toml`). |
| `wrap` | 1 | `crates/locus-parametric-solver/src/solver/residual.rs:115` | - |  |
| `wrap_paragraph` | 4 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:38` | - | Greedy word-wrapping supporting manual newlines and bounded per-character breaking. |
| `impl ProgramConfig::write` | 2 | `crates/audio-aac-codec/src/config.rs:90` | - |  |
| `impl AdtsHeader::write` | 5 | `crates/audio-aac-codec/src/config.rs:328` | - | Write a 7-byte header (no CRC) for a frame whose raw payload is `payload_len` bytes. |
| `impl IcsInfo::write` | 2 | `crates/audio-aac-codec/src/ics.rs:101` | - |  |
| `impl TnsData::write` | 3 | `crates/audio-aac-codec/src/tns.rs:70` | - |  |
| `impl Writer::write` | 3 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:102` | - |  |
| `impl Hasher for FxHasher::write` | 2 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:19` | - |  |
| `impl SampleEntry::write` | 3 | `crates/media-isobmff/src/codec.rs:1325` | - | Serialize as a complete sample entry box. For PCM the format is chosen from the brand. |
| `impl TileState::write` | 4 | `crates/vector-compute-shaders/src/cpu/coarse.rs:67` | - |  |
| `impl FrameHeader::write` | 1 | `crates/video-prores-codec/src/header.rs:136` | - | Serialise the frame header (without the 8-byte size/identifier prefix). |
| `impl PictureHeader::write` | 2 | `crates/video-prores-codec/src/header.rs:201` | - |  |
| `write_aco` | 1 | `crates/color-transforms/src/palette/aco.rs:146` | - |  |
| `write_ase` | 1 | `crates/color-transforms/src/palette/ase.rs:209` | - |  |
| `write_begin_clip` | 0 | `crates/vector-compute-shaders/shader/coarse.wgsl:134` | - |  |
| `impl TileState::write_begin_clip` | 4 | `crates/vector-compute-shaders/src/cpu/coarse.rs:156` | - |  |
| `write_ber4` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:134` | - | Write a 4-byte BER length. |
| `impl BitWriter::write_bit` | 2 | `crates/bitstream-codec/src/lib.rs:195` | - |  |
| `impl BitWriter::write_bits` | 3 | `crates/bitstream-codec/src/lib.rs:181` | - |  |
| `impl TileState::write_blur_rect` | 6 | `crates/vector-compute-shaders/src/cpu/coarse.rs:141` | 2d |  |
| `write_blurred_rounded_rect` | 2 | `crates/vector-compute-shaders/shader/coarse.wgsl:148` | 2d |  |
| `write_btrt` | 2 | `crates/media-isobmff/src/codec.rs:1554` | - |  |
| `impl BitWriter::write_bytes` | 2 | `crates/bitstream-codec/src/lib.rs:233` | - |  |
| `impl ScreenBuffer::write_char` | 2 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:159` | - |  |
| `write_chrometrace` | 2 | `crates/gpu-command-profiler/src/chrometrace.rs:6` | graph | Writes a .json trace file that can be viewed as a flame graph in Chrome or Edge via <chrome://tracing> |
| `write_clipitem` | 3 | `crates/chronicle-timeline-interchange/src/interchange/fcp7.rs:200` | - |  |
| `write_code` | 2 | `crates/video-h264-codec/src/cavlc.rs:241` | - |  |
| `write_color` | 1 | `crates/vector-compute-shaders/shader/coarse.wgsl:112` | - |  |
| `impl TileState::write_color` | 5 | `crates/vector-compute-shaders/src/cpu/coarse.rs:99` | - |  |
| `write_data` | 3 | `crates/audio-device-cpal/examples/android.rs:72` | - |  |
| `write_data` | 3 | `crates/audio-device-cpal/examples/beep.rs:128` | - |  |
| `write_domain` | 3 | `crates/color-transforms/src/lut.rs:450` | - |  |
| `write_edl` | 1 | `crates/chronicle-timeline-interchange/src/interchange/edl.rs:142` | - | Serialize a TimelineSequence to CMX 3600 EDL text format. |
| `write_end_clip` | 1 | `crates/vector-compute-shaders/shader/coarse.wgsl:140` | - |  |
| `impl TileState::write_end_clip` | 6 | `crates/vector-compute-shaders/src/cpu/coarse.rs:167` | - |  |
| `impl IfcWriter::write_entity` | 3 | `crates/manifold-bim-csg/src/ifc.rs:67` | - |  |
| `impl MxfWriter<W>::write_essence_packet` | 3 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:206` | - |  |
| `write_expr` | 3 | `crates/chronicle-tabular-engine/src/formula/printer.rs:78` | - |  |
| `write_f32` | 3 | `crates/media-test-support/src/wav.rs:33` | - | Write planar `f32` channels to `path` as a float WAV. |
| `write_fcp7_xml` | 1 | `crates/chronicle-timeline-interchange/src/interchange/fcp7.rs:144` | - | Serialize a TimelineSequence to FCP7 XML (`xmeml`) string. |
| `write_fcpxml` | 1 | `crates/chronicle-timeline-interchange/src/interchange/fcpxml.rs:155` | - | Serialize a TimelineSequence to an FCPXML document string. |
| `write_fill` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:792` | - | Write fill elements totalling at least `bits - 7` bits (at most `bits + 8`); returns bits written. |
| `impl MkvWriter<W>::write_frame` | 6 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:268` | - | Add one frame. |
| `write_frame_header` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:214` | - |  |
| `write_ftyp` | 2 | `crates/media-isobmff/src/mux.rs:154` | - |  |
| `write_gpl` | 1 | `crates/color-transforms/src/palette/gpl.rs:75` | - |  |
| `write_grad` | 3 | `crates/vector-compute-shaders/shader/coarse.wgsl:119` | - |  |
| `impl TileState::write_grad` | 7 | `crates/vector-compute-shaders/src/cpu/coarse.rs:125` | - |  |
| `write_grd` | 1 | `crates/color-transforms/src/palette/grd.rs:123` | - | Serializes gradients into `.grd` format. |
| `impl MxfWriter<W>::write_header` | 1 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:134` | - |  |
| `write_html_inlines` | 2 | `crates/chronicle-office-codecs/src/document/html.rs:89` | - |  |
| `write_ics` | 4 | `crates/audio-aac-codec/src/encoder/quant.rs:318` | - | Write an `individual_channel_stream`; `write_info` is false inside a common-window CPE. |
| `write_id` | 2 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:76` | - | Encode an element ID. |
| `impl Recording::write_image` | 5 | `crates/vector-compute-renderer/src/recording.rs:155` | - |  |
| `write_image` | 1 | `crates/vector-compute-shaders/shader/coarse.wgsl:127` | - |  |
| `impl TileState::write_image` | 5 | `crates/vector-compute-shaders/src/cpu/coarse.rs:112` | - |  |
| `impl FragmentedWriter<W>::write_init` | 1 | `crates/media-isobmff/src/mux.rs:759` | - |  |
| `write_inlines` | 2 | `crates/chronicle-office-codecs/src/document/markdown.rs:400` | - |  |
| `write_input_data` | 2 | `crates/audio-device-cpal/examples/record_wav.rs:164` | - |  |
| `write_latex_inlines` | 2 | `crates/chronicle-office-codecs/src/document/latex.rs:105` | - |  |
| `write_line` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:769` | - |  |
| `write_line` | 6 | `crates/vector-compute-shaders/src/cpu/flatten.rs:76` | 2d |  |
| `write_line_with_transform` | 5 | `crates/vector-compute-shaders/shader/flatten.wgsl:776` | - |  |
| `write_line_with_transform` | 7 | `crates/vector-compute-shaders/src/cpu/flatten.rs:98` | 2d |  |
| `write_lottie` | 1 | `crates/kinetics-motion-compositor/src/lottie/mod.rs:17` | - | Serialize a LottieAnimation structure to JSON string. |
| `write_mcc` | 2 | `crates/chronicle-timeline-interchange/src/captions/mcc.rs:133` | - |  |
| `write_msl` | 2 | `crates/vector-compute-shaders/build.rs:111` | - |  |
| `write_msl` | 2 | `crates/vector-compute-shaders/build.rs:116` | - |  |
| `write_mvhd` | 4 | `crates/media-isobmff/src/mux.rs:195` | - |  |
| `write_otio` | 1 | `crates/chronicle-timeline-interchange/src/interchange/otio.rs:144` | - | Serialize a TimelineSequence to OpenTimelineIO JSON format. |
| `write_path` | 3 | `crates/vector-compute-shaders/shader/coarse.wgsl:88` | - |  |
| `impl TileState::write_path` | 6 | `crates/vector-compute-shaders/src/cpu/coarse.rs:71` | - |  |
| `impl SliceDecoder<'a>::write_pcm` | 2 | `crates/video-h264-codec/src/slicedec.rs:1060` | - | Write I_PCM samples. |
| `write_pcm` | 2 | `crates/video-h264-codec/src/synth_tests.rs:276` | - |  |
| `write_png` | 2 | `crates/media-test-support/src/golden.rs:84` | - |  |
| `write_profile` | 1 | `crates/color-transforms/src/icc/write.rs:68` | - |  |
| `write_rational_time` | 2 | `crates/chronicle-timeline-interchange/src/interchange/otio.rs:126` | - |  |
| `write_results_recursive` | 3 | `crates/gpu-command-profiler/src/chrometrace.rs:28` | - |  |
| `write_rtf_inlines` | 2 | `crates/chronicle-office-codecs/src/document/rtf.rs:59` | - |  |
| `impl Mp4Writer<W>::write_sample` | 3 | `crates/media-isobmff/src/mux.rs:570` | - | Append one sample to track `track` (decode order). For PCM tracks `data` may hold any whole |
| `impl FragmentedWriter<W>::write_sample` | 3 | `crates/media-isobmff/src/mux.rs:788` | - | Buffer a sample for the next fragment. |
| `impl BitWriter::write_se` | 2 | `crates/bitstream-codec/src/lib.rs:212` | - |  |
| `write_sf` | 2 | `crates/audio-aac-codec/src/huffman.rs:114` | - | Write one scalefactor difference (`-60..=60`). |
| `write_shaders` | 2 | `crates/vector-compute-shaders/build.rs:58` | - |  |
| `write_size` | 3 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:102` | - | Encode a data size as a VINT using the shortest length (or `min_len` if larger). |
| `write_slice` | 2 | `crates/video-prores-codec/src/encode.rs:383` | - |  |
| `write_spectral` | 3 | `crates/audio-aac-codec/src/huffman.rs:202` | - | Write `vals` with codebook `cb` (1..=11). |
| `write_srt` | 1 | `crates/chronicle-timeline-interchange/src/captions/srt.rs:72` | - |  |
| `impl std::fmt::Write for OutString<'_>::write_str` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:407` | - |  |
| `impl fmt::Write for CargoWarningAdapter<W>::write_str` | 2 | `crates/vector-compute-shaders/build.rs:193` | - |  |
| `write_time_range` | 3 | `crates/chronicle-timeline-interchange/src/interchange/otio.rs:135` | - |  |
| `trait ProfilerCommandRecorder::write_timestamp` | 3 | `crates/gpu-command-profiler/src/profiler_command_recorder.rs:5` | - |  |
| `impl WebGlRenderer::write_to_atlas` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:758` | 2d | Write pixel data to an existing atlas allocation. |
| `trait WebGlAtlasWriter::write_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3313` | - | Write image data to an atlas texture at the specified offset. |
| `impl WebGlAtlasWriter for Pixmap::write_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3333` | - |  |
| `impl WebGlAtlasWriter for Arc<Pixmap>::write_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3376` | - |  |
| `impl WebGlAtlasWriter for WebGlTexture::write_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3405` | - |  |
| `impl WebGlAtlasWriter for WebGlTextureWithDimensions::write_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3452` | - |  |
| `impl Renderer::write_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:562` | 2d | Write pixel data to an existing atlas allocation. |
| `trait AtlasWriter::write_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3293` | - | Write image data to an atlas texture at the specified offset. |
| `impl AtlasWriter for Texture::write_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3315` | - |  |
| `impl AtlasWriter for Pixmap::write_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3361` | - |  |
| `impl AtlasWriter for Arc<Pixmap>::write_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3409` | - |  |
| `impl ZipPackage::write_to_bytes` | 1 | `crates/chronicle-office-codecs/src/zip/minizip.rs:76` | - | Serialize all contained files into a standard ZIP archive byte vector. |
| `write_trak` | 6 | `crates/media-isobmff/src/mux.rs:220` | physics |  |
| `write_ts_packet` | 6 | `crates/chronicle-broadcast-containers/src/mpegts/ts.rs:85` | - | Write a 188-byte TS packet into `out`. |
| `write_types` | 2 | `crates/vector-compute-shaders/build.rs:49` | - |  |
| `impl Hasher for FxHasher::write_u64` | 2 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:24` | - |  |
| `write_udta` | 2 | `crates/media-isobmff/src/mux.rs:464` | - |  |
| `impl BitWriter::write_ue` | 2 | `crates/bitstream-codec/src/lib.rs:198` | - |  |
| `write_unknown_size` | 2 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:113` | - | Encode the "unknown size" marker of the given length. |
| `write_vint` | 3 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:90` | - | Encode an unsigned integer as a VINT with a specific width (1..=8 bytes). |
| `write_vtt` | 1 | `crates/chronicle-timeline-interchange/src/captions/vtt.rs:93` | - |  |
| `impl XEl::write_xml` | 2 | `crates/chronicle-office-codecs/src/xml/dom.rs:100` | - |  |
| `impl Parser<'_>::ws` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:61` | - |  |
| `xirr` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:280` | - |  |
| `xlookup` | 1 | `crates/chronicle-tabular-engine/src/functions/lookup.rs:65` | - |  |
| `xml_escape` | 1 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:13` | - |  |
| `xml_escape` | 1 | `crates/asset-production-vibe/src/layout_ops/mod.rs:341` | - |  |
| `xml_escape` | 1 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:91` | - |  |
| `xml_escape` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:456` | - |  |
| `xml_unescape` | 1 | `crates/discourse-pdf-compliance/src/forms/xfdf.rs:99` | - |  |
| `xml_unescape` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:460` | - |  |
| `xnpv` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:271` | - |  |
| `xnpv_calc` | 3 | `crates/chronicle-tabular-engine/src/functions/financial.rs:266` | - |  |
| `xpbd_body` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:458` | - |  |
| `impl jack::NotificationHandler for JackNotificationHandler::xrun` | 2 | `crates/audio-device-cpal/src/host/jack/stream.rs:459` | - |  |
| `impl Vec3::xy` | 1 | `crates/drafting-geometry/src/vec.rs:157` | 2d |  |
| `xy_to_xyz` | 1 | `crates/color-transforms/src/icc/math.rs:79` | - | XYZ (Y = 1) of a chromaticity `(x, y)`. |
| `impl Reader<'a>::xyz` | 2 | `crates/color-transforms/src/icc/profile.rs:322` | - |  |
| `xyz_to_lab` | 2 | `crates/color-transforms/src/icc/math.rs:135` | - | CIE XYZ → CIE L*a*b* under white point `white`. |
| `xyz_type` | 1 | `crates/color-transforms/src/icc/write.rs:11` | - |  |
| `ycbcr_roundtrip` | 0 | `crates/color-transforms/src/lib.rs:325` | - |  |
| `ycbcr_to_rgb` | 4 | `crates/color-transforms/src/lib.rs:145` | numerics | Convert normalised Y'CbCr (Y in 0..1, Cb/Cr in -0.5..0.5) to non-linear R'G'B'. |
| `ycbcr_to_rgb` | 4 | `crates/radiance-color-grading/src/matrix.rs:140` | numerics | Convert normalized Y'CbCr (Y in 0..1, Cb/Cr in -0.5..0.5) to non-linear R'G'B'. |
| `year_of` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:70` | - | First 4-digit run. |
| `yes` | 0 | `crates/vector-locus-geometry/src/perspective.rs:57` | - |  |
| `yield_maturity` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:513` | - |  |
| `yielddisc` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:606` | - |  |
| `zero_area_external_texture_rect_is_a_noop` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:983` | 2d |  |
| `zero_craft_names_in_canonical_categories_or_morphism_ids` | 0 | `crates/asset-production-vibe/src/ontology/tests.rs:32` | - |  |
| `zero_radius_keeps_source_operation_behavior` | 0 | `crates/raster-selection-morphology/src/tests.rs:202` | - |  |
| `impl Matrix::zeros` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:45` | - |  |
| `impl BoxBuf::zeros` | 2 | `crates/media-isobmff/src/bytes.rs:307` | - |  |
| `zig_zag` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:192` | 3d, 2d | Zig Zag: ridges extra points per segment, alternately displaced along normal. |
| `zigzag8` | 0 | `crates/video-h264-codec/src/tables.rs:9` | - |  |
| `zigzag8_known_prefix` | 0 | `crates/video-h264-codec/src/tables.rs:151` | - |  |
| `zotero_code` | 1 | `crates/layout-paged-typesetter/src/citations/zotero_fields.rs:13` | - |  |
| `zxy_to_tile_id` | 3 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:219` | - | Converts (zoom, x, y) tile coordinates to PMTiles Hilbert tile ID. |
| `zz4_to_raster` | 1 | `crates/video-h264-codec/src/params.rs:37` | - |  |
| `zz8_to_raster` | 1 | `crates/video-h264-codec/src/params.rs:44` | - |  |
