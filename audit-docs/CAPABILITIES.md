# hypermedia-production — capabilities

Functional overview generated from source on 2026-11-10T12:07:10.5599269Z.
903 source files, 8745 functions (2198 documented).

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


## Domain coverage

| domain | functions |
|---|---|
| 2d | 366 |
| 3d | 173 |
| computational geometry | 98 |
| economics | 6 |
| graph | 95 |
| numerics | 171 |
| physics | 37 |
| semantics | 20 |

## Modules

### `checks/consumer-smoke` — 1 function (0 documented)

_Standalone migration closure smoke; coordinator owns execution._

_1 undocumented function — see `modules/checks_consumer-smoke.md`._

### `crates/acoustics-daw-session` — 81 functions (27 documented)

_Native audio codecs: pure-Rust AIFF/AIFF-C, FLAC encoder, RF64/BWF WAV, and PeakFile waveform pyramids._

**general**

- `encode_wav` (function) — Encodes an AudioBuffer as a 16-bit PCM standard WAV file.
- `decode_wav` (function) — Decodes standard 16-bit PCM or 32-bit float WAV bytes into an AudioBuffer.
- `encode_aiff` (function) — Encodes an AudioBuffer as a standard 16-bit AIFF file.
- `encode_flac` (function) — Encodes an AudioBuffer as a valid lossless FLAC bitstream (subset: RFC 9639 uncompressed / verbatim subframes).
- `design` (method) — Designs an audio filter from frequency, Q factor, gain (dB), and sample rate.
- `k_weight_stage1_coeffs` (function) — BS.1770 K-weighting pre-filter stage 1 (high shelf: +4 dB high boost).
- `k_weight_stage2_coeffs` (function) — BS.1770 K-weighting pre-filter stage 2 (high pass: 100 Hz cutoff).
- `process_interleaved` (method) — Feeds interleaved multi-channel audio samples into the loudness meter.
- `integrated_lufs` (method) — Computes the gated integrated loudness in LUFS (-70 LUFS absolute gate, -10 LU relative gate).
- `process_samples` (method) — Feeds samples and updates the max inter-sample true-peak value.
- `next_sample` (method) — Generates the next output sample and advances phase.
- `fill_buffer` (method) — Fills a buffer with generated oscillator samples.
- `polyblep` (function) — PolyBLEP residual function for smoothing step discontinuities across [-dt, +dt].
- `stereo_pan_gains` (function) — Computes left and right channel gains for a given pan position (-1.0 hard left .. +1.0 hard right).
- `to_cartesian` (method) — Unit Cartesian direction vector (x: right, y: front, z: up).
- `pan` (method) — Computes speaker gains for a sound source at (azimuth_deg, elevation_deg).
- `detect_pitch_yin` (function) — Computes the fundamental frequency $f_0$ of an audio frame using the YIN algorithm with parabolic interpolation.
- `hz_to_midi` (function) — Converts a frequency in Hz to closest MIDI note number (A4 = 440 Hz = 69).
- `window_params` (method) — Window size in seconds and maximum search tolerance fraction for WSOLA.
- `process` (method) — Stretches input audio samples to output length using WSOLA.
- `render_block` (method) — Renders a block of audio (left, right) for the session at `start_sample` with exact latency alignment.
- `detect_camelot_key` (function) — Krumhansl-Schmuckler key detection from a 12-element chroma profile.
- `segment_musical_phrases` (function) — Detects 8-bar musical phrases from energy envelopes across bars.
- `parse_scratch_dsl` (function) — Parses a turntable scratch pattern string into beats of actions (e.g. "AEA.AEA.").
- `evaluate_scratch_position` (function) — Evaluates turntable needle position (in beats) and mute gain at beat time $t$.
- `encode_mtc_quarter_frame` (function) — Encodes an MTC quarter-frame MIDI message (0xF1, data_byte).

**numerics**

- `evaluate_warp_timeline` (function) — Evaluates source playback time $t_{\text{src}}$ corresponding to destination time $t_{\text{dst}}$ using piecewise linear interpolation.

_54 undocumented functions — see `modules/crates_acoustics-daw-session.md`._

### `crates/asset-production-vibe` — 814 functions (172 documented)

_Acoustics 32-Bus AudioServer mixing DAG & PGA spatializer,
Discourse telemetry inspection & fiduciary RID verification,
and Chronicle locked-delta-t deterministic movie capture & 60 Hz PGA motor spatial sync.

Grounded in UDHR Art. 12, 18, 19, 27 and ICCPR Art. 17, 19 for informatics fiduciary
provenance, deterministic multimedia synchronization, and spatial audio reproduction._

**2d**

- `rasterize_uv_chart` (function) — Rasterize UV triangles into a still map so a non-trivial island pixel is observable.

**3d**

- `advect_m64_grid` (method) — Solves divergence-free pressure projection via Jacobi Poisson iterations,
- `produce_wifi_csi_vitals_aoa` (method) — 802.11 CSI → sanitized phase, contactless vitals, MUSIC AoA, camera-free receipt.
- `query_anatomical_ontology_pin` (method) — Queries the closest canonical anatomical ontology pin (`CANONICAL_ANATOMICAL_PINS`, `32` pins)
- `synchronize_companion_frustum` (method) — Generates a multi-projector conical dome frustum cluster (`rows * cols` `ConicalDomeWedge`s)
- `render_animated_cinema_frames` (function) — Renders an N-frame animated 3D cinema sequence (`160x112` H.264 macroblock-aligned) combining
- `radiance_build_light_tree` (method) — 5. Hierarchical light tree bounding-cone clustering and direct lighting preview
- `develop_camera_raw_hdr` (method) — 128: `hypermedia.radiance.develop_camera_raw_hdr`

**computational geometry**

- `intersect_aabb` (function) — Ray-AABB intersection test returning entry and exit parameters `(t_min, t_max)`.
- `render_macro_behavior_cinema_frames` (function) — Renders an N-frame animated 3D cinema sequence driven by a `BehaviorMacro` and `SubElementHooks`.
- `compile_botanical_organ_10d` (method) — Compiles a 4-scale botanical `.10d` organ container (`Scale 1` whole organism, `Scale 2`
- `compile_vertebrate_anatomy_10d` (method) — Compiles a Scale-5 comparative vertebrate 3-layer `.10d` anatomy container (`Osteology`,
- `build_authored_part` (function) — One Synth-authored part. The mesh is generated here. No external model is fetched.
- `compile_vertebrate_anatomy_10d_is_stacked_ellipsoids_not_a_clinic_heart` (function) — `compile_vertebrate_anatomy_10d` is a stacked-ellipsoid / primitive scaffold
- `render_anatomical_blueprint_svg` (function) — Renders a vector orthographic & anatomical callout blueprint SVG from a 3D mesh.
- `locus_fillet` (method) — Corner filleting on a 2D polyline or closed polygon.
- `closest_point_on_triangle` (function) — Computes the exact closest point on a 3D triangle to a query point P
- `ray_intersects_triangle` (function) — Möller-Trumbore ray-triangle intersection test returning true if ray (origin, dir) intersects.
- `store_mesh_positions` (method) — Stores a 3D Mesh artifact validating budgets and finite bounds.
- `compute_smooth_normals` (function) — Compute area-weighted smooth unit vertex normals for a triangle mesh.
- `mesh` (method) — Retrieve a reference to a `Mesh` artifact from a production handle.
- `store_mesh_artifact` (method) — Store a new `Mesh` artifact after bounding coordinates and computing bounding box.
- `compute_smooth_vertex_normals` (function) — Compute smooth area-weighted per-vertex unit normals for a 3D mesh.
- `parse_vertex_uvs` (function) — Vertex-aligned UVs from a list, unwrap record, or named profile (island outline
- `seal_mesh_10d_with_slim_uvs` (function) — Seal SLIM TEXCOORD_0 into a QualiaDB `.10d` so unwrap feeds `compile_10d_pbr`.

**general**

- `quantize_pga_motor` (function) — Quantizes an 8-float PGA motor state (position [x,y,z], rotation [qx,qy,qz,qw], sequence, entity_id)
- `schroeder_reverb` (function) — Deterministic 4-comb, 2-allpass Schroeder reverberator tail generator.
- `apply_antipop_ramp_down` (function) — Applies a 64-sample anti-pop lookahead cosine ramp down to zero.
- `apply_dsp_compressor` (function) — Soft-knee compressor / limiter insert.
- `acoustics_discourse_chronicle_ext_op` (method) — Categorical operations dispatch for acoustics, discourse, and chronicle extensions.
- `acoustics_ext` (method) — Categorical operations dispatch for the `acoustics` category extensions.
- `filter_biquad` (method) — Audio EQ Cookbook biquad IIR filter (lowpass, highpass, bandpass, notch, peaking).
- `encode_aac` (method) — AAC-LC CBR audio encoding with ADTS framing.
- `integrated_lufs_bs1770` (function) — ITU-R BS.1770-4 / EBU R128 gated integrated LUFS.
- `schedule_cpm_evm_montecarlo` (method) — 138: `hypermedia.kinetics.schedule_cpm_evm_montecarlo`
- `analyze_ebu_r128_camelot_daw` (method) — 139: `hypermedia.acoustics.analyze_ebu_r128_camelot_daw`
- `kriging_variogram_morphometry` (method) — 141: `hypermedia.geodesy.kriging_variogram_morphometry`
- `produce_locked_dt_maps` (method) — Locked-dt ProRes + OTIO + Matroska + fiduciary hash-chain maps.
- `audit_tex_pdf_ua_matlang` (method) — 142: `hypermedia.discourse.audit_tex_pdf_ua_matlang`
- `recalculate_tabular_fiduciary_archive` (method) — 143: `hypermedia.chronicle.recalculate_tabular_fiduciary_archive`
- `crc32_ieee` (function) — Branchless IEEE 802.3 CRC-32 checksum (polynomial 0xEDB88320).
- `escape_xml` (function) — Escape XML entities for OpenXML and SVG documents.
- `escape_pdf_str` (function) — Escape ASCII characters in PDF string literals `(...)`.
- `container_family` (function) — Produce-safe family for gallery presentation of sealed Q42 containers.
- `produce_safe_container_svg` (function) — Display-referred sRGB SVG card. Binary containers are never inlined as executable payloads.
- `discourse_op` (method) — Categorical operations dispatch for the `discourse` category morphisms.
- `review_gallery` (method) — CSP-locked gallery. Origin image/audio path is preserved; 10d/m64/emf/q42
- `domain_extension_op` (method) — Dispatches Wave 13 domain extension morphisms (`120..=143`).
- `couple_continuum_xpbd` (method) — Evaluates Bekker / Mohr-Coulomb terramechanics pressure-sinkage, tractive shear stress,
- `electromagnetics_geodesy_ext_op` (method) — Categorical operations dispatch for electromagnetics and geodesy extensions.
- `spectral_wavelength_to_rgb` (function) — CIE 1931 / Bruton piecewise spectral-to-sRGB mapping for wavelength in nanometers.
- `sample_bilinear_2d` (function) — Bilinear interpolation sampler over a 2D periodic field.
- `neighborhood_min_max` (function) — Determines the local min and max over a 2x2 neighborhood for BPS advection limiting.
- `raymarch_vox0_dda` (method) — Raymarches a 3D `.10d` `Vox0DagNode` / `.emf` volumetric brick lattice using exact
- `project_metamer_nullspace` (method) — Exposes Etalon's 41-band spectral metamer fiber decomposition:
- `electromagnetics_op` (method) — Categorical operations dispatch for the `electromagnetics` category.
- `compile_emf` (method) — Synthesizes a 3D wave-optical & electromagnetic field volume and seals it
- `verify_emf` (method) — Parses and cryptographically verifies the Q42EMF64 container,
- `render_spectral_slice` (method) — Renders a 2D spectral preview slice from an EMF-64 volume container to straight-alpha RGBA8.
- `resolve_emf_bytes` (method) — Resolves container bytes from retained inputs or workspace encoded resources.
- `parse_sealed_emf` (function) — QualiaDB `parse` admits CRC-only files; untrusted paths require a writer Bao seal.
- `glyph_advance` (function) — Proportional advance width in normalized em units (1.0 = em height).
- `pair_kerning` (function) — Pair kerning adjustment in normalized em units.
- `char_to_svg_subpath` (function) — Emits the clean vector strokes for `ch` into `out` as SVG path instructions `M ... L ...`.
- `append_breaking_word` (function) — Append `word` with per-character breaks so wrap width includes pair kerning.
- `wrap_paragraph` (function) — Greedy word-wrapping supporting manual newlines and bounded per-character breaking.
- `shape_glyph_run` (function) — Shapes, measures, and positions vector glyph runs into SVG path instructions.
- `visual_reorder_line` (function) — UAX #9 L2 visual order for a laid-out line.
- `split_paragraphs` (function) — `\r\n` is one paragraph break; `\n` and `\r` each start a new paragraph.
- `paragraph_glyphs` (function) — Shape `text` into composer glyphs, word/hyphen breakpoints, and hyphen-point count.
- `compose_knuth_plass_paragraph` (method) — 131: `hypermedia.glyph.compose_knuth_plass_paragraph`
- `admits_16bit` (method) — Named origin profiles stay 8-bit; default/2k/16-bit admit 16-bit codecs.
- `lattice_profile` (function) — Parse an optional named lattice profile. Default admits 2k / 8-bit.
- `drape_orthotropic_garment` (method) — 136: `hypermedia.kinetics.drape_orthotropic_garment` (`kinetics-garment-draping`)
- `seal_ik_pose_skl1_10d` (function) — Bind-pose triangle + 3-joint SKL1 attached into a sealed `.10d`.
- `step_island_xpbd` (method) — ID-sorted deterministic Island-Parallel XPBD rigid-body simulation.
- `query_direct_space` (method) — Synchronous DirectSpaceState spatial queries over split Static/Dynamic BVH.
- `navigate_funnel_orca` (method) — 3D NavMesh Portal Pathfinding + Simple Stupid Funnel + 3D ORCA Avoidance.
- `evaluate_nla_channelbag` (method) — Blender ActionChannelbag NLA Strip Blending + Godot LensTweenTable evaluation.
- `step_simulation_zone` (method) — Double-buffered Simulation / Repeat Zone iterative divergence-free curl flow.
- `typeset_paged_foliation` (method) — 133: `hypermedia.layout.typeset_paged_foliation` (`layout-paged-typesetter`)
- `compile_dataflow_nodegraph` (method) — 134: `hypermedia.layout.compile_dataflow_nodegraph` (`layout-nodegraph-compiler` + `layout-presentation-deck`)
- `distribute_flex_sizes` (function) — Freeze-and-redistribute free main-axis space so min/max clamps stay exact.
- `bind_theme_scope` (method) — Binds structured Webizen scoped design tokens and CSS variables.
- `render_ui_sheet` (method) — Renders the resolved layout into a straight-alpha RGBA8 raster artifact.
- `project_ui_svg` (method) — Projects the resolved layout into a standalone SVG vector resource with scoped CSS tokens.
- `admit_encoded` (method) — Admit bytes the production host already holds. The files producer accepts the handle.
- `consume_unwrap_uv01` (method) — When unwrap sealed UV01 into `.10d`, public compile_10d_pbr consumes it.
- `default_hooks` (method) — Returns the baseline `SubElementHooks` preset for this whole-model macro before any
- `with_overrides` (method) — Parses optional sub-element hook overrides from Vibe `Args` on top of `self`.
- `apply_to_rig_pose` (method) — Applies the macro + sub-element hooks onto an 18-joint `FaunaSkeletalRig10d` PGA motor pose.
- `to_small_llm_json` (method) — Formats the compact ~22-token JSON command packet that a small (0.5B–4B) LLM emits.
- `build_fauna_production_plan_json` (function) — Builds the complete **Model Production Plan** (`qualia.model-production-plan/v1`) for a Fauna
- `build_flora_mycelium_production_plan_json` (function) — Builds the complete **Flora & Mycelial Neural Network Production Plan**
- `fnv1a_60` (function) — Deterministic FNV-1a 60-bit hash (`lexicon.rs` mask `0x0FFF_FFFF_FFFF_FFFF`).
- `compile_model_logic_q42_bytes` (function) — Compiles a deterministic `.q42` (`Q42LOGIC` 64-byte header + 16 parity-verified 48-byte `NQuin`
- `joint_rom_from_corpus` (function) — Joint ranges taken from active corpus assertions. Each range cites its assertion.
- `assemble_species_corpus` (method) — `qualia.species.assemble_corpus`: validate a species document and seal it as `.q42`.
- `build_human_welfare_anatomy_10d` (function) — Builds a complete 3-layer / 17-system Human Welfare & Maternal-Fetal `.10d` container
- `compile_human_welfare_q42_bytes` (function) — Compiles a deterministic 832-byte `.q42` (`Q42LOGIC`) welfare & maternal-fetal rule container
- `apply_age_morphism` (function) — Years lived and how old the person looks are separate.
- `compile_human_welfare_anatomy_10d` (method) — Morphism `hypermedia.manifold.compile_human_welfare_anatomy_10d`:
- `active_range` (method) — The range a pose may use: the narrowed range when injured, otherwise the healthy range.
- `parse_joint_rom` (function) — Parses the spec's optional `joints` list. Returns an empty list when absent.
- `joint_rom_rules` (function) — Compiles joint ranges into rules. Property IRIs are `q42:rom:<joint>:<axis>:<zone>{Neg,Pos}`,
- `build_anatomy_from_portable_scaffold` (function) — Compiles a 3-layer `.10d` vertebrate anatomical twin from a species spec (VibeScript record
- `rig_biomechanical_archetype` (method) — Constructs and steps the 18-joint PGA `Motor` fauna skeletal rig (`FaunaSkeletalRig10d`),
- `step_sector_continuum_3d` (method) — Runs the 3D XPBD + analytical SDF + 2-way continuum bridge + 4-wheel rover suspension
- `parse_rules_value` (function) — Parses a VibeScript list of rule records.
- `archetype_default_rules` (function) — The archetype-class default rules, identical to the legacy
- `compile_rule_pack_q42` (function) — Compiles rules into a `Q42LOGIC` container. Lamport clocks follow rule order, starting at 1.
- `validate_q42_logic` (function) — Validates a `Q42LOGIC` container: magic, version, length, per-kind counts, payload digest and
- `resolve_species_spec` (function) — Returns the species spec from `spec` (or its legacy alias `scaffold`) and where it came from.
- `species_model_id` (function) — The model id for rule packs: the spec's `species_id`, otherwise `fallback`.
- `resolve_rules` (function) — Resolves the `rules` argument into a validated `Q42LOGIC` container, appending `derived`
- `render_hero_rgba_frame` (function) — Renders a 256x256 RGBA studio hero frame suitable for downstream Radiance / CDL / MaterialX morphisms.
- `render_turnaround_sheet_png` (function) — Renders a 3-panel (`900x420`) RGBA8 turnaround sheet PNG (`Side Profile`, `3/4 Hero`, `Front`).
- `add_curved_tube` (method) — Add a continuous, welded multi-station curved tube with parallel-transported ring frames.
- `add_foliage_lobe` (method) — Add an ellipsoid or sculpted foliage cluster lobe with organic surface modulation.
- `cubic_bezier` (function) — Evaluates a 4-point cubic Bezier curve at parameter `t` in `[0, 1]`.
- `build_whitegum_whole_organism_10d` (function) — Builds a high-craft Scale-1 Whole-Organism Whitegum / Ghost Gum (`Corymbia aparrerinja`)
- `print_arc_segments` (function) — Print-density sample count for a circular sweep; `min_seg` is the origin floor.
- `locus_resample` (method) — Resample a 2D locus by uniform arc length count, fixed step length, or curvature weighting.
- `locus_generative_op` (method) — Dispatch morphism operations for generative curve and geodesic path workflows.
- `locus_bishop_rmf_cyclic` (method) — Cyclic double-reflection Bishop rotation-minimizing frame (RMF) generator along a 3D path.
- `svg_num` (function) — Fixed-point SVG number; kills signed zero so stills are bit-stable.
- `pathfinder_warp_envelope` (method) — 121: `hypermedia.locus.pathfinder_warp_envelope` (`vector-locus-pathops` + `vector-locus-geometry`)
- `transcode_geospatial_vector` (method) — 122: `hypermedia.locus.transcode_geospatial_vector` (`locus-geospatial-codecs`)
- `sweep_assembly_profile` (method) — 123: `hypermedia.manifold.sweep_assembly_profile`
- `compile_bim_ifc4` (method) — 124: `hypermedia.manifold.compile_bim_ifc4`
- `erode_geomorphic_heightfield` (method) — 125: `hypermedia.manifold.erode_geomorphic_heightfield`
- `manifold_fields_sdf_op` (method) — Dispatch entrypoint for Wave 7 Manifold Lazy Fields, SDF Bricks, Instancing & Botany.
- `densify_loft_rings_nurbs` (function) — NURBS densify of loft stations via `parametric_cad::nurbs_eval` (unit weights).
- `manifold_topology_op` (method) — Dispatch manifold topology morphisms.
- `cycle_interior` (function) — Vertices strictly between `from` and `to` walking the cycle forward.
- `resolve_morphism` (function) — Resolves any canonical categorical ID or historical `hypermedia.*`/`synth.*` alias
- `canonical_morphism_id` (function) — Resolves any operation identifier to its canonical categorical ID (`hypermedia.<category>.<morphism>`).
- `morphism_to_json` (function) — Serializes a single `AgenticMorphismSpec` including its `OntologicalContract` and `NQuin48` parity hashes.
- `radiance_decode_cryptomatte_q42` (method) — 2. Cryptomatte Q42 ID coverage matte extraction and isolated compositing
- `radiance_vector_motion_blur` (method) — 3. Vector motion blur line-integral convolution with Hann shutter window
- `grade_cdl_scopes` (method) — 129: `hypermedia.radiance.grade_cdl_scopes`
- `trace_vector_contours` (method) — 130: `hypermedia.radiance.trace_vector_contours`
- `optical_nm_to_audio_hz` (method) — Heterodyne optical wavelength onto A4 (532 nm → 440 Hz) so the field is hearable.
- `sonify_emf_field` (method) — Field-derived mono PCM: irradiance envelope in sealed chunk/cell order (no HashMap).
- `uv_island_hull` (function) — Monotone-chain hull of a SLIM UV island. Sorted, so iteration is produce-safe.
- `surface_bsdf_uv_op` (method) — Categorical operations dispatch for surface BSDF, UV, and procedural field synthesis.
- `bake_pbr_materialx` (method) — 126: `hypermedia.surface.bake_pbr_materialx`
- `expand_procedural_brush` (method) — 127: `hypermedia.surface.expand_procedural_brush`

**graph**

- `locus_shortest_edge_path` (method) — Shortest edge path across a 3D triangle manifold using deterministic Dijkstra search.
- `splice_face_cycles` (function) — Join two face cycles at the shared oriented edge (v0, v1).
- `radiance_filter_kuwahara_anisotropic` (method) — 1. Anisotropic Kuwahara structure-tensor edge-preserving filter

**numerics**

- `mix_tracks` (method) — Multi-track audio mixer with linear fade envelopes and master gain.
- `emf_cell_display_irradiance` (function) — Vacuum-impedance irradiance (W/m²) used as the display-linear quantity.
- `spectral_wavelength_to_rgb` (function) — CIE 1931 / Bruton piecewise spectral-to-sRGB mapping for wavelength in nanometers.
- `solve_continuum_fem_lbm` (method) — 135: `hypermedia.kinetics.solve_continuum_fem_lbm` (`kinetics-continuum-multiphysics`)
- `solve_flexbox_grid_constraints` (method) — 132: `hypermedia.layout.solve_flexbox_grid_constraints` (`layout-box-autolayout`)
- `solve_box_tree` (method) — Solves deterministic 2D spatial flex or CSS-grid-like layouts across viewports.
- `locus_trim` (method) — Extract a sub-curve from normalized arc length [start, end] with exact linear interpolation.
- `segment_is_ruled` (function) — Ruled/linear cubics stay named two-point spans (no print-density tessellation).
- `collapse_collinear` (function) — Drop interior samples on ruled/linear spans so they stay named two-point profiles.
- `solve_parametric_sketch` (method) — 120: `hypermedia.locus.solve_parametric_sketch` (`locus-parametric-solver` + `drafting-geometry`)
- `densify_loft_rings_cubic` (function) — Cubic Catmull-Rom densify of loft stations. Linear/ruled skinning is applied after.
- `radiance_solve_sdfgi_froxel` (method) — 4. Cascaded SDFGI irradiance probes and 3D froxel atmospheric scattering

**physics**

- `first_class_strata_column` (function) — First-class 6-layer STRATA column: sculpted elevation plus slope hydrology
- `simulate_causal_block_diagram` (method) — 137: `hypermedia.kinetics.simulate_causal_block_diagram` (`kinetics-block-simulation` + `kinetics-motion-compositor`)
- `interpolate_motor_fti` (method) — PGA MotorFTI Screw-Linear Interpolation (ScLERP) between physics ticks and render frames.
- `run_sector_physics_verification_step` (function) — Runs a deterministic 3D XPBD + analytical SDF + 2-way continuum bridge + 4-wheel rover suspension
- `living_world_anatomy_op` (method) — Categorical operations dispatch for Wave 12 Living-World anatomy, fauna rigging,

**semantics**

- `glyph_op` (method) — Dispatches glyph operations admitted by the category-theoretic ontology.
- `push_nquin_48` (function) — Packs a single 48-byte Super-Quin (`NQuin`: `subject`, `predicate`, `object`, `context`,
- `categories_json` (function) — Serializes the category-theoretic ontology taxonomy for agentic discovery (`synth.ontology.categories`).
- `morphisms_json` (function) — Serializes morphisms filtered by optional `category` (`synth.ontology.morphisms`).

_642 undocumented functions — see `modules/crates_asset-production-vibe.md`._

### `crates/audio-aac-codec` — 190 functions (82 documented)

_AAC-LC encoder.

Pipeline per 1024-sample frame and channel element:
1. transient detection on high-passed 128-sample sub-blocks (one frame of look-ahead) →
window sequence (`ONLY_LONG` / `LONG_START` / `EIGHT_SHORT` / `LONG_STOP`) and short-window grouping;
2. windowed MDCT (sine or KBD);
3. psychoacoustic model → per-band masking thresholds and perceptual entropy;
4. TNS analysis (long windows; Levinson–Durbin, applied when the prediction gain is high);
5. per-band M/S decision for channel pairs (common window);
6. rate–distortion loop: a single multiplier λ scales every band's allowed noise; each band gets the
coarsest scalefactor meeting its noise target, sections/codebooks are chosen by dynamic
programming, and λ is bisected until the frame fits its bit budget (CBR) or fixed (VBR);
7. bit reservoir (CBR): budget follows the frame's PE, bounded by the decoder buffer
(6144 bits/channel); overflow is padded with fill elements so the rate stays constant._

**general**

- `layout` (method) — Elements in output channel order (front, side, back, LFE).
- `config_layout` (function) — Element layout of `channelConfiguration` 1..=7 (tags as conventionally assigned).
- `lc` (method) — A plain AAC-LC configuration.
- `to_bytes` (method) — Serialise (AAC-LC, no SBR signalling).
- `layout` (method) — Elements in output channel order.
- `to_asc` (method) — The equivalent `AudioSpecificConfig`. Channel configuration 0 (PCE in-band) is resolved by the
- `write` (method) — Write a 7-byte header (no CRC) for a frame whose raw payload is `payload_len` bytes.
- `split_adts` (function) — Split an ADTS stream into `(header, raw payload)` frames. Stops at the first malformed or
- `new` (method) — Create a decoder from an `AudioSpecificConfig`.
- `from_adts` (method) — Create a decoder for an ADTS stream from its first header. With channel configuration 0 the
- `sbr` (method) — Whether the stream is HE-AAC: SBR signalled in the `AudioSpecificConfig` (explicit) or SBR
- `layout` (method) — Element layout in output channel order (C, L/R, surround pairs, LFE for configurations 1–7).
- `reset` (method) — Clear the overlap state (e.g. after a seek).
- `decode` (method) — Decode one access unit (one `raw_data_block`) into 1024 samples per channel.
- `fill_noise` (method) — Perceptual noise substitution. `pair` = (left channel, ms mask) for the right channel of a CPE:
- `finish_channel` (method) — TNS + filterbank for a single channel element; returns output samples.
- `synth` (function) — Inverse MDCT, windowing and overlap-add (§4.6.18), output scaled to ±1.0 full scale.
- `sbr_fill_element_marks_implicit_he_aac` (function) — Implicitly signalled HE-AAC (an AAC-LC config) is recognised from the SBR fill element.
- `bitrate` (method) — Average bitrate of the raw access units.
- `audio_specific_config` (method) — `AudioSpecificConfig` bytes for the MP4 `esds` decoder-specific info.
- `priming_samples` (method) — Encoder delay: the first 1024 decoded samples are priming and should be skipped (MP4 edit
- `bandwidth` (method) — Audio bandwidth in Hz.
- `encode` (method) — Feed planar samples (±1.0 full scale); returns any complete access units. Missing channels are
- `flush` (method) — Encode the remaining input (zero-padded). Afterwards the decoded stream, minus
- `available` (method) — Absolute sample index one past the buffered input.
- `needed_for` (method) — Input needed (absolute end) before frame `t` can be encoded (window + transient look-ahead).
- `transient` (method) — Transient in the region covered by frame `t`'s short windows.
- `encode_frame` (method) — `last`: final frame of the stream — a CBR stream spends (or pads away) the whole reservoir so
- `write_fill` (function) — Write fill elements totalling at least `bits - 7` bits (at most `bits + 8`); returns bits written.
- `analyse` (method) — Analyse a spectrum: 1024 lines (long) or 8×128 (short).
- `new` (method) — `spec` is window-major (short windows at stride 128); `thr[g][sfb]` and the grouping come from
- `quantise` (function) — Quantise a slice at `sf`, returning the distortion; writes the (signed) values to `out` if given.
- `search_sf` (function) — Largest scalefactor whose distortion stays within `allowed`.
- `candidates` (function) — Candidate codebooks for a band whose largest magnitude is `m` (0 for all-zero bands).
- `quantise_channel` (function) — Quantise one channel at rate-distortion multiplier `lambda` and select sections.
- `write_ics` (function) — Write an `individual_channel_stream`; `write_info` is false inside a common-window CPE.
- `book` (function) — Spectrum codebook 1..=11.
- `decode_sf` (function) — Decode one scalefactor difference (`-60..=60`).
- `write_sf` (function) — Write one scalefactor difference (`-60..=60`).
- `sf_bits` (function) — Bits of one scalefactor difference.
- `decode_spectral` (function) — Decode one codeword of codebook `cb` (1..=11) into `out[..dim]`, including sign bits and escapes.
- `index_of` (function) — Codebook index of the (sign-stripped for unsigned books) values.
- `count_bits` (function) — Bits needed to code `vals` (a multiple of the codebook dimension) with codebook `cb` (1..=11).
- `write_spectral` (function) — Write `vals` with codebook `cb` (1..=11).
- `swb_offsets` (method) — Band offsets of one window.
- `num_swb` (method) — Number of bands for this window length (`num_swb`).
- `group_start` (method) — First window of group `g`.
- `dct4` (method) — DCT-IV: `out[k] = Σ u[n] cos(π/M (n+½)(k+½))`.
- `forward` (method) — Forward MDCT of `x` (length 2M, already windowed) into `out` (length M).
- `inverse` (method) — Inverse MDCT (unscaled): `y[n] = Σ X[k] cos(π/M (n + ½ + M/2)(k + ½))`, `y` has length 2M.
- `mdct_long` (function) — Cached transforms for 1024 and 128 lines.
- `window` (function) — Rising half (length N/2) of the window of the given shape; `long` selects N = 2048, else N = 256.
- `sample_rate_index` (function) — Map an arbitrary sample rate to its `samplingFrequencyIndex`, if it is one of the standard rates.
- `table_index_for_rate` (function) — The table index used for scalefactor-band tables when a non-standard rate is signalled explicitly
- `swb_offsets_long` (function) — Scalefactor band offsets (`swb_offset_long_window`, 1024-line frames); `len() == num_swb + 1`.
- `swb_offsets_short` (function) — Scalefactor band offsets for one short window (128 lines).
- `tns_max_bands` (function) — `TNS_MAX_BANDS` for AAC-LC (long, short).
- `tns_max_order` (function) — `TNS_MAX_ORDER` for AAC-LC.
- `pow43` (function) — `x^(4/3)` for the integer quantised magnitudes 0..=8191.
- `sf_gain` (function) — `2^(0.25 * (sf - 100))`, the dequantisation gain of scalefactor `sf`.
- `bit_count` (method) — Bits `write` would produce.
- `dequant_coef` (function) — Dequantise one reflection coefficient.
- `quant_coef` (function) — Quantise a reflection coefficient (inverse of [`dequant_coef`], nearest index).
- `parcor_to_lpc` (function) — Reflection coefficients → direct-form LPC `a[0..=order]` (`a[0] = 1`).
- `regions` (function) — Filter region `[start, end)` of every TNS filter of window `w`, in spectrum lines of that window.
- `apply_decoder` (function) — Decoder: apply the all-pole TNS synthesis filters to `spec` (1024 lines; short windows at 128 stride).
- `apply_encoder` (function) — Encoder: apply the all-zero TNS analysis filters (exact inverse of [`apply_decoder`]).
- `levinson` (function) — Levinson–Durbin on autocorrelation `r[0..=order]`: returns reflection coefficients and the
- `ffmpeg` (function) — ffmpeg (see `media_test_support::oracle`); `None` after printing `SKIPPED`.
- `sweep` (function) — Logarithmic sine sweep 40 Hz → 16 kHz.
- `speech` (function) — Speech-like: glottal pulse train (gliding pitch) through three resonant formant filters with a
- `chords` (function) — Music-like: a sequence of decaying harmonic chords.
- `clicks` (function) — Clicks: sharp impulses every 250 ms over a quiet low tone (transient test).
- `ffmpeg_decode` (function) — Decode with ffmpeg → planar f32 (ffmpeg's own channel order), plus stderr.
- `ffmpeg_encode` (function) — Encode with ffmpeg's AAC encoder to ADTS.
- `our_decode` (function) — Decode an ADTS file with our decoder.
- `snr` (function) — SNR in dB of `dec` against `refr`.
- `seg_snr` (function) — Segmental SNR (1024-sample segments above -50 dBFS, each clamped to [-10, 60] dB).
- `lowpass` (function) — Windowed-sinc (Blackman, 401 taps) low-pass at `fc` Hz, zero-phase (delay-compensated).
- `compare` (function) — Returns (max abs error, samples compared).
- `pns_matches_ffmpeg` (function) — PNS output is random by design, and ffmpeg's encoder does not emit it: build noise-substituted

**graph**

- `ath_db` (function) — Terhardt's threshold in quiet (dB SPL), capped at 40 dB so content near the band edge is not

_108 undocumented functions — see `modules/crates_audio-aac-codec.md`._

### `crates/audio-device-cpal` — 539 functions (86 documented)

_coreaudio on iOS looks a bit different from macOS. A lot of configuration needs to use
the AVAudioSession objc API which doesn't exist on macOS.

TODO:
- Use AVAudioSession to enumerate buffer size / sample rate / number of channels and set
buffer size._

**general**

- `open` (method) — Create `DeviceHandles` for `name` and try to open a handle for both
- `try_open` (method) — Get a mutable reference to the `Option` for a specific `stream_type`.
- `get_mut` (method) — Get a mutable reference to the `alsa::PCM` handle for a specific `stream_type`.
- `take` (method) — Take ownership of the `alsa::PCM` handle for a specific `stream_type`.
- `supported_input_configs` (method) — Gets the supported input configs.
- `supported_output_configs` (method) — Gets the supported output configs.
- `default_input_config` (method) — Returns the default input config
- `default_output_config` (method) — Returns the default output config
- `next` (method) — Load drivers and return device
- `process_input_callback` (function) — 1. Write from the ASIO buffer to the interleaved CPAL buffer.
- `process_output_callback` (function) — 1. Render the given callback to the given buffer of interleaved samples.
- `get_or_create_input_stream` (method) — Create a new CPAL Input Stream.
- `get_or_create_output_stream` (method) — Create a new CPAL Output Stream.
- `system_time_to_stream_instant` (function) — Asio retrieves system time via `timeGetTime` which returns the time in milliseconds.
- `frames_to_duration` (function) — Convert the given duration in frames at the given sample rate to a `std::time::Duration`.
- `check_config` (function) — Check whether or not the desired config is supported by the stream.
- `cast_slice_mut` (function) — Cast a byte slice into a mutable slice of desired type.
- `from_le` (function) — Helper function to convert from little endianness.
- `from_be` (function) — Helper function to convert from little endianness.
- `asio_channel_slice` (function) — Shorthand for retrieving the asio buffer slice associated with a channel.
- `asio_channel_slice_mut` (function) — Shorthand for retrieving the asio buffer slice associated with a channel.
- `build_output_stream_raw` (method) — Create an output stream.
- `add_disconnect_listener` (function) — Register the on-disconnect callback.
- `set_sample_rate` (function) — Attempt to set the device sample rate to the provided rate.
- `new` (method) — Attach the provided callback as a audio object property listener.
- `remove` (method) — Explicitly remove the property listener.
- `property_listener_handler_shim` (function) — Callback used to call user-provided closure as a property listener.
- `default_input_config` (method) — Returns the default input config
- `default_output_config` (method) — Returns the default output config
- `set_connect_automatically` (method) — Set whether the ports should automatically be connected to system
- `set_start_server_automatically` (method) — Set whether a JACK server should be automatically started if it isn't already.
- `is_available` (method) — JACK is available if
- `connect_to_system_outputs` (method) — Connect to the standard system outputs in jack, system:playback_1 and system:playback_2
- `connect_to_system_inputs` (method) — Connect to the standard system outputs in jack, system:capture_1 and system:capture_2
- `build_output_stream_raw` (method) — Create an output stream.
- `com_initialized` (function) — Ensures that COM is initialized in this thread.
- `ensure_future_audio_client` (method) — Ensures that `future_audio_client` contains a `Some` and returns a locked mutex to it.
- `build_audioclient` (method) — Returns an uninitialized `IAudioClient`.
- `get_audio_clock` (function) — Get the audio clock used to produce `StreamInstant`s.
- `stop_and_reset` (method) — Stop the WASAPI client and reset its pending stream buffer on the
- `frames_to_duration` (function) — Convert the given duration in frames at the given sample rate to a `std::time::Duration`.
- `stream_instant` (function) — Use the stream's `IAudioClock` to produce the current stream instant.
- `input_timestamp` (function) — Produce the input stream timestamp.
- `output_timestamp` (function) — Produce the output stream timestamp.
- `build_output_stream_raw` (method) — Create an output stream.
- `audio_context` (method) — Return the [`AudioContext`](https://developer.mozilla.org/docs/Web/API/AudioContext) used
- `duration_since` (method) — The amount of time elapsed from another instant to this one.
- `add` (method) — Returns the instant in time after the given duration has passed.
- `sub` (method) — Returns the instant in time one `duration` ago.
- `timestamp` (method) — The timestamp associated with the call to an input stream's data callback.
- `timestamp` (method) — The timestamp associated with the call to an output stream's data callback.
- `sample_format` (method) — The sample format of the internal audio data.
- `len` (method) — The full length of the buffer in samples.
- `bytes` (method) — The raw slice of memory representing the underlying audio data as a slice of bytes.
- `bytes_mut` (method) — The raw slice of memory representing the underlying audio data as a slice of bytes.
- `as_slice` (method) — Access the data as a slice of sample type `T`.
- `as_slice_mut` (method) — Access the data as a slice of sample type `T`.
- `with_sample_rate` (method) — Retrieve a [`SupportedStreamConfig`] with the given sample rate and buffer size.
- `try_with_sample_rate` (method) — Retrieve a [`SupportedStreamConfig`] with the given sample rate and buffer size.
- `with_max_sample_rate` (method) — Turns this [`SupportedStreamConfigRange`] into a [`SupportedStreamConfig`] corresponding to the maximum samples rate.
- `cmp_default_heuristics` (method) — A comparison function which compares two [`SupportedStreamConfigRange`]s in terms of their priority of
- `default_host` (function) — The default host for the current compilation target platform.
- `default_host` (function) — The default host for the current compilation target platform.
- `default_host` (function) — The default host for the current compilation target platform.
- `default_host` (function) — The default host for the current compilation target platform.
- `default_host` (function) — The default host for the current compilation target platform.
- `default_host` (function) — The default host for the current compilation target platform.
- `default_host` (function) — The default host for the current compilation target platform.
- `sample_size` (method) — Returns the size in bytes of a sample of this format.
- `is_available` (method) — Whether or not the host is available on the system.
- `devices` (method) — An iterator yielding all [`Device`](DeviceTrait)s currently available to the host on the system.
- `default_input_device` (method) — The default input audio device on the system.
- `default_output_device` (method) — The default output audio device on the system.
- `input_devices` (method) — An iterator yielding all `Device`s currently available to the system that support one or more
- `output_devices` (method) — An iterator yielding all `Device`s currently available to the system that support one or more
- `name` (method) — The human-readable name of the device.
- `supported_input_configs` (method) — An iterator yielding formats that are supported by the backend.
- `supported_output_configs` (method) — An iterator yielding output stream formats that are supported by the device.
- `default_input_config` (method) — The default input stream format for the device.
- `default_output_config` (method) — The default output stream format for the device.
- `build_input_stream` (method) — Create an input stream.
- `build_output_stream` (method) — Create an output stream.
- `build_input_stream_raw` (method) — Create a dynamically typed input stream.
- `build_output_stream_raw` (method) — Create a dynamically typed output stream.
- `play` (method) — Run the stream.
- `pause` (method) — Some devices support pausing the audio stream. This can be useful for saving energy in

_453 undocumented functions — see `modules/crates_audio-device-cpal.md`._

### `crates/audio-opus-codec` — 203 functions (77 documented)

_CELT layer decoder (RFC 6716 §4.3)._

**general**

- `decode_pulses` (function) — Decodes a PVQ codeword of `k` pulses in `n` dimensions; returns the sum of squares.
- `exp_rotation` (function) — Inverse spreading rotation (decoder direction).
- `sdiv` (function) — C-style truncating signed division.
- `quant_all_bands` (function) — Decodes all band shapes (`quant_all_bands`, decoder side).
- `anti_collapse` (function) — Anti-collapse noise injection (RFC 6716 §4.3.5).
- `denormalise_bands` (function) — Multiplies the normalised shapes by the band energies (`denormalise_bands`).
- `laplace_decode` (function) — Decodes a Laplace-distributed integer (`fs` = probability of 0 in Q15, `decay` in Q14).
- `unquant_coarse_energy` (function) — `unquant_coarse_energy`.
- `backward` (method) — Inverse transform of `n/2` coefficients read from `input` at `stride`.
- `imdct_matches_direct_formula` (function) — The folded output must equal the direct IMDCT formula folded by TDAC symmetry.
- `new` (method) — `sample_rate` is the output rate (48000, 24000, 16000, 12000 or 8000).
- `reset` (method) — `OPUS_RESET_STATE`.
- `decode` (method) — Decodes one CELT frame of `frame_size` samples (at the output rate) into interleaved `pcm`
- `synthesis` (method) — Denormalisation + IMDCT into the decode memory (`celt_synthesis`).
- `decode_lost` (method) — Packet-loss concealment: pitch-periodic extension for the first lost frames, then
- `comb_filter` (function) — Pitch post-filter (in place on `buf[off .. off + n]`, reading history before `off`).
- `get_pulses` (function) — Number of pulses represented by pseudo-pulse index `i`.
- `log2_frac` (function) — Conservative (rounded up) log2 of `val` with `frac` fractional bits.
- `pvq_v` (function) — PVQ codebook size V(N, K) as u128 (saturating far above 2^32).
- `init_caps` (method) — Per-band allocation caps for this frame (`init_caps`).
- `required_bits` (function) — Bits (1/2^frac units, rounded up) needed for each K in `0..=maxk` for dimension `n`.
- `compute_allocation` (function) — `clt_compute_allocation` (decoder side).
- `new` (method) — Creates a decoder producing `channels` (1 or 2) at `rate` (8/12/16/24/48 kHz).
- `final_range` (method) — Final range coder state of the last decoded frame (for conformance checks).
- `last_packet_duration` (method) — Duration (samples per channel) of the last decoded packet.
- `set_gain_q8` (method) — Sets the output gain in Q8 dB (as in the OpusHead header).
- `set_phase_inversion_disabled` (method) — Disables the 180-degree intensity-stereo phase inversion (RFC 8251 §10).
- `reset` (method) — `OPUS_RESET_STATE`.
- `decode_interleaved` (method) — Decodes a packet into interleaved samples appended to `out`. `None` conceals one lost
- `conceal` (method) — Conceals `n` samples (a multiple of 2.5 ms).
- `decode_fec` (method) — Decodes the in-band FEC (LBRR) of `packet` for the previous (lost) packet of `n` samples.
- `decode_frame` (method) — `opus_decode_frame`: decodes one frame (or conceals when `data` is `None`/≤1 byte).
- `simple` (method) — A family-0 configuration for plain mono/stereo streams (no header present).
- `parse` (method) — Parses an `OpusHead` packet (Ogg first packet, Matroska `CodecPrivate`).
- `from_dops` (method) — Parses the payload of an ISO-BMFF `dOps` box (Opus in MP4, big-endian fields).
- `to_bytes` (method) — Serialises as an `OpusHead` packet.
- `new` (method) — Creates a 48 kHz decoder from an `OpusHead` packet (Ogg / Matroska `CodecPrivate`).
- `simple` (method) — Creates a decoder for plain mono/stereo packets without a header.
- `from_head` (method) — Creates a decoder for a parsed header at `sample_rate` (8/12/16/24/48 kHz).
- `pre_skip` (method) — Pre-skip in samples at the output rate.
- `set_trim_pre_skip` (method) — When enabled, the first [`Self::pre_skip`] output samples are dropped.
- `set_phase_inversion_disabled` (method) — Disables intensity-stereo phase inversion (useful when downmixing; RFC 8251 section 10).
- `final_range` (method) — Final range coder state of the first stream (conformance testing).
- `decode` (method) — Decodes one packet (`None` = lost packet, concealed) into planar channels.
- `reframe` (function) — Rebuilds a regular (code 3 VBR) packet from a parsed self-delimited one.
- `celt_end_band` (method) — Number of CELT bands coded for this bandwidth.
- `frame_samples_48k` (method) — Samples per frame at 48 kHz.
- `frame_samples` (method) — Samples per frame at `rate`.
- `parse` (method) — Parses a packet (RFC 6716 section 3.2), enforcing requirements R1 to R7.
- `parse_self_delimited` (method) — Parses a self-delimited packet (RFC 6716 Appendix B, used for all but the last stream of
- `packet_samples_48k` (function) — Number of samples (at 48 kHz) in a packet, or an error for malformed framing.
- `ilog` (function) — Number of bits needed to represent `x` (0 for 0).
- `storage` (method) — Size of the buffer in bytes as seen by the decoder.
- `shrink_storage` (method) — Shrinks the buffer seen by the decoder (raw bits are then read from the new end).
- `decode` (method) — Returns the cumulative frequency of the next symbol for total `ft` (call [`Self::update`] next).
- `decode_bin` (method) — Like [`Self::decode`] with `ft = 1 << bits`.
- `update` (method) — Consumes the symbol with cumulative range `[fl, fh)` of total `ft`.
- `update_bin` (method) — Like [`Self::update`] for a symbol decoded with [`Self::decode_bin`].
- `bit_logp` (method) — Decodes a binary symbol whose probability of being 1 is `1/2^logp`.
- `icdf` (method) — Decodes a symbol with an "inverse CDF" table (`icdf[i] = ft - cdf[i+1]`, `ft = 1 << ftb`).
- `uint` (method) — Decodes a uniformly distributed integer in `[0, ft)` (`ft >= 2`).
- `bits` (method) — Reads `bits` raw bits (0..=25) from the end of the buffer.
- `skip_to_end` (method) — Advances the bit counter so that [`Self::tell`] reports `total` (CELT silence frames).
- `tell` (method) — Number of bits consumed so far (rounded up).
- `tell_frac` (method) — Number of 1/8 bits consumed so far (rounded up).
- `done` (method) — Finalises and returns a buffer of exactly `size` bytes.
- `inverse32_varq` (function) — `silk_INVERSE32_varQ`: approximation of `(1 << q_res) / b32`.
- `div32_varq` (function) — `silk_DIV32_varQ`: approximation of `(a32 << q_res) / b32`.
- `nlsf_decode` (function) — Decodes the stage-1/stage-2 indices into stabilised NLSFs (Q15).
- `bwexpander` (function) — `silk_bwexpander` on Q12 coefficients (16-bit).
- `nlsf2a` (function) — Converts NLSFs (Q15) to stable Q12 LPC coefficients (`silk_NLSF2A`).
- `conceal` (method) — Basic concealment: pitch-periodic repetition for voiced history, otherwise LPC-shaped
- `decode` (method) — Decodes one SILK frame (10 or 20 ms) into interleaved `out` at the API rate.
- `design_fractional` (function) — Least-squares fractional-delay interpolator: taps at offsets `-past+1 ..= future` (in input
- `process` (method) — Resamples `input` (16-bit scale) and appends the output samples.
- `icdf` (function) — Converts a PDF (summing to 256) into an inverse CDF (`256 - cdf[i+1]`).

**numerics**

- `gain_linear` (method) — Linear output gain factor.

_126 undocumented functions — see `modules/crates_audio-opus-codec.md`._

### `crates/audio-timebase` — 111 functions (50 documented)

_Audio timebase, synchronization, metric and SMPTE conversions.

Provides deterministic conversions between musical time (bars, beats, ticks at PPQ = 960),
real elapsed seconds, raw sample indices across sample rates (8 kHz to 768 kHz),
drop-frame and non-drop-frame SMPTE timecodes, 35mm film Feet+Frames, and dynamic tempo maps.

Grounded in international human rights instruments (UDHR Articles 19, 27; ICESCR Article 15)
and Informatics Fiduciary duties ensuring lossless temporal synchronization and moral attribution._

**general**

- `ticks` (method) — Ticks in a straight note of this value in 4/4.
- `step_samples` (method) — Step size in samples at position `at`.
- `snap` (method) — Snap `pos` to the nearest grid line.
- `line_at_or_before` (method) — The grid line at or immediately before `pos`.
- `next_line` (method) — The first grid line strictly after `line`.
- `lines` (method) — List all grid lines within `[start, end)`, capped at `max` entries.
- `to_samples` (function) — Round a float sample count to `i64`, mapping NaN to 0 and saturating infinities.
- `seconds` (method) — Convert samples to real seconds.
- `samples` (method) — Convert real seconds to samples, rounded to nearest and saturated.
- `ticks_per_beat` (method) — Number of ticks in one beat under this meter.
- `ticks_per_bar` (method) — Number of ticks in a complete bar under this meter.
- `set_tempo` (method) — Insert or update the tempo at `tick`.
- `remove_tempo` (method) — Remove a tempo event (the initial event at tick 0 is pinned).
- `set_meter` (method) — Insert or update the meter at `tick` (snapped to the bar line at or before it).
- `remove_meter` (method) — Remove a meter event (the initial event at tick 0 is pinned).
- `tempo_at_tick` (method) — Active tempo at `tick`.
- `meter_at_tick` (method) — Active meter at `tick`.
- `tick_to_samples` (method) — Convert ticks to samples.
- `samples_to_ticks_f` (method) — Convert samples to fractional ticks.
- `samples_to_ticks` (method) — Convert samples to integer ticks.
- `bar_start_tick` (method) — Starting tick of 1-based `bar`.
- `bar_beat_at_tick` (method) — Musical `BarBeat` position at `tick`.
- `tick_at_bar_beat` (method) — Musical `BarBeat` to absolute `tick`.
- `bar_beat_at` (method) — Musical `BarBeat` at sample index `s`.
- `samples_at_bar_beat` (method) — Sample index at musical `BarBeat`.
- `scale_tempos` (method) — Scale all tempos by a constant ratio.
- `rational` (method) — Exact rational frame rate (numerator, denominator).
- `nominal` (method) — Nominal frames per second integer base.
- `is_drop` (method) — Whether this is a drop-frame SMPTE timecode rate.
- `label` (method) — Short human-readable display label.
- `frames_at` (method) — Total whole frames at a sample position (floored).
- `samples_at_frame` (method) — Sample position at the beginning of frame `f`.
- `samples_per_frame` (method) — Samples per frame (fractional).
- `from_frames` (method) — Compute timecode label for an absolute frame count.
- `to_frames` (method) — Absolute frame count corresponding to this timecode label.
- `from_samples` (method) — Derive timecode label from sample position.
- `to_samples` (method) — Sample index at the start of this timecode.
- `format` (method) — Format as standard `HH:MM:SS:FF` or drop-frame `HH:MM:SS;FF`.
- `parse` (method) — Parse timecode string (`HH:MM:SS:FF`, with `:` / `;` / `.` separators).
- `from_samples` (method) — Derive FeetFrames from raw audio samples.
- `to_total_frames` (method) — Total frames represented by this FeetFrames value.
- `to_samples` (method) — Convert FeetFrames to audio samples.
- `format` (method) — Format as standard `FEET+FRAMES` (e.g. `1+08`).
- `parse` (method) — Parse `FEET+FRAMES` format string.
- `feet_frames` (function) — Compatibility function returning raw `(feet, frames)` tuple for 35mm film.
- `from_samples` (method) — Calculate all multi-timebase coordinates from raw audio samples.
- `format` (method) — Format this position according to the specified `TimeFormat`.
- `format_position` (function) — Format an absolute sample position into a string.
- `format_length` (function) — Format a duration or length starting at `start`.
- `parse_position` (function) — Parse a user-entered position string into samples.

_61 undocumented functions — see `modules/crates_audio-timebase.md`._

### `crates/bitstream-codec` — 42 functions (14 documented)

_Bit-level reading and writing shared by the chronicle media codec crates.

- [`BitReader`]: MSB-first reader with Exp-Golomb (`ue(v)`, `se(v)`), as used by H.264/HEVC/AV1 headers.
- [`BitWriter`]: the matching writer (for encoders and header rewriting).
- [`unescape_rbsp`] / [`escape_rbsp`]: remove/insert H.264/HEVC emulation-prevention bytes.
- [`annexb_nals`] / [`length_prefixed_nals`]: split Annex-B byte streams and AVCC/HVCC samples into NAL units._

**general**

- `position` (method) — Bit position from the start.
- `byte_pos` (method) — Byte offset of the current position (rounded down).
- `peek` (method) — Peek up to 32 bits without consuming (zero-padded past the end).
- `read_ue` (method) — Unsigned Exp-Golomb `ue(v)`.
- `read_se` (method) — Signed Exp-Golomb `se(v)`.
- `read_te` (method) — Truncated Exp-Golomb `te(v)` with range `max`.
- `more_rbsp_data` (method) — H.264/HEVC `more_rbsp_data()`: true if there is data before the rbsp trailing bits.
- `align_zero` (method) — Pad with zero bits to a byte boundary.
- `rbsp_trailing` (method) — `rbsp_trailing_bits()`: a one bit then zeros to alignment.
- `unescape_rbsp` (function) — Remove emulation-prevention bytes (`00 00 03` → `00 00`) from a NAL payload.
- `escape_rbsp` (function) — Insert emulation-prevention bytes so no `00 00 0x` (x ≤ 3) sequence appears.
- `annexb_nals` (function) — Split an Annex-B byte stream (start codes `00 00 01` / `00 00 00 01`) into NAL units (escaped payloads).
- `length_prefixed_nals` (function) — Split a length-prefixed sample (AVCC/HVCC, `length_size` 1/2/4 bytes) into NAL units.
- `crc32` (function) — CRC-32 (IEEE, reflected) — used by containers (e.g. Matroska CRC-32 elements, PNG).

_28 undocumented functions — see `modules/crates_bitstream-codec.md`._

### `crates/chronicle-broadcast-containers` — 148 functions (27 documented)

_SMPTE VC-3 Avid DNxHD / DNxHR broadcast intermediate frame decoder._

**general**

- `idct8x8` (function) — In-place inverse DCT. `block` holds dequantised coefficients in raster order
- `fdct8x8` (function) — In-place forward DCT (samples in, coefficients out).
- `read_vint` (function) — Decode an unsigned VINT. Returns `(value, length, all_ones)`.
- `read_size` (function) — Decode an element size.
- `read_id` (function) — Decode an element ID (marker bits retained).
- `read_svint` (function) — Decode a signed VINT as used by EBML lacing (value − (2^(7·len−1) − 1)).
- `write_id` (function) — Encode an element ID.
- `write_vint` (function) — Encode an unsigned integer as a VINT with a specific width (1..=8 bytes).
- `write_size` (function) — Encode a data size as a VINT using the shortest length (or `min_len` if larger).
- `write_unknown_size` (function) — Encode the "unknown size" marker of the given length.
- `uint` (function) — Big-endian unsigned integer of 0..=8 bytes.
- `int` (function) — Big-endian two's-complement signed integer of 0..=8 bytes.
- `float` (function) — IEEE float of 0, 4 or 8 bytes.
- `string` (function) — String / UTF-8 value: bytes up to the first NUL, lossily decoded.
- `write_frame` (method) — Add one frame.
- `finish` (method) — Write Cues, back-patch SeekHead / Segment size / Duration, and return writer.
- `build_pes_packet` (function) — Create a PES packet containing `data` with optional PTS/DTS.
- `crc32_mpeg` (function) — CRC-32 MPEG-2 (polynomial 0x04C11DB7, initial 0xFFFFFFFF).
- `build_pat` (function) — Build a PAT section including CRC-32.
- `build_pmt` (function) — Build a PMT section including CRC-32.
- `parse_ts_packet` (function) — Parse a single 188-byte TS packet.
- `write_ts_packet` (function) — Write a 188-byte TS packet into `out`.
- `is_smpte` (method) — Every SMPTE label starts with the object identifier 06 0E 2B 34.
- `matches` (method) — Compare ignoring byte 7 (the registry version, which varies between writers).
- `item_starts_with` (method) — Bytes 8.. start with `prefix` (registry-independent comparison of the item designator).
- `ber_length` (function) — Decode a BER length at `b[0..]`: (length, bytes used). `None` if truncated or longer than 8 bytes.
- `write_ber4` (function) — Write a 4-byte BER length.

_121 undocumented functions — see `modules/crates_chronicle-broadcast-containers.md`._

### `crates/chronicle-fiduciary-ledger` — 56 functions (19 documented)

_`chronicle-fiduciary-ledger`

UI-agnostic categorical fiduciary ledger engine:
- Exact fixed-point multi-currency money arithmetic (`money`)
- Chart of accounts and balance sheet classifications (`accounts`)
- Balanced double-entry journal posting with period lock enforcement (`journal`)
- ASC 606 / IFRS 15 multi-period revenue recognition amortization schedules (`revrec`)
- Fixed-asset depreciation schedules with residual salvage protection (`depreciation`)
- Multi-entity currency translation & intercompany elimination consolidation (`consolidation`)
- Tamper-evident hash-chained fiduciary audit trail grounded in UDHR/ICCPR/ICESCR (`audit`)_

**economics**

- `line_amount` (function) — Calculates line net amount (before tax) = qty * unit price * (1 - discount).

**general**

- `standard_template` (method) — Sets up a standard standard chart of accounts:
- `append` (method) — Appends a new verified audit event to the tamper-evident chain.
- `verify_chain` (method) — Validates the complete cryptographic hash chain from genesis to head.
- `consolidate_entities` (function) — Performs consolidation across entities with translation and elimination.
- `annual_rate_bp` (function) — Computes the effective annual rate in basis points (bp) from method and life.
- `month_depreciation` (function) — Calculates depreciation for one calendar month with residual value clamp.
- `post_journal` (method) — Post a balanced journal entry. Fails closed on imbalance, lock dates, or missing accounts.
- `reverse_journal` (method) — Reverses a previously posted journal entry with inverted amounts.
- `account_balances` (method) — Calculates net balances for all accounts: positive = net debit, negative = net credit.
- `trial_balance_totals` (method) — Calculates total debits and total credits across all posted entries.
- `div_round` (function) — Commercial rounding: divide rounding half away from zero.
- `div_round_bankers` (function) — Banker's rounding: divide rounding half to even.
- `to_base` (function) — Converts an amount in foreign currency to base currency (1 base = rate foreign).
- `parse_fixed` (function) — Parse a decimal string like "-1,234.50" into a fixed-point integer with `dp` decimals.
- `fmt_money_grouped` (function) — Formats money with thousands separators: e.g. `12,345.60` or `-1,234.56`.
- `new` (method) — Generates a new multi-period revenue recognition schedule.
- `post_due_entries` (method) — Recognises all unposted periods due on or before `as_of_date` by posting to `ledger`.
- `remaining_unrecognized` (method) — Returns the remaining unrecognized deferred revenue balance.

_37 undocumented functions — see `modules/crates_chronicle-fiduciary-ledger.md`._

### `crates/chronicle-office-codecs` — 83 functions (22 documented)

_Document interchange codecs (Markdown, HTML, RTF, LaTeX)._

**general**

- `parse_html` (function) — Parse HTML string into a `FlowDoc`.
- `to_latex` (function) — Export a `FlowDoc` to standard LaTeX source.
- `parse_latex` (function) — Simple parser for LaTeX `.tex` documents into `FlowDoc`.
- `parse_markdown` (function) — Parse a Markdown string into a `FlowDoc`.
- `parse_inlines` (function) — Parse inline formatting: bold, italic, code, link, inline math.
- `to_markdown` (function) — Serialize a `FlowDoc` to CommonMark / GFM Markdown.
- `to_rtf` (function) — Serialize a `FlowDoc` to standard RTF 1.5.
- `parse_rtf` (function) — Simple parser for RTF 1.5 files into `FlowDoc`.
- `to_pptx_package` (function) — Serialize a `Presentation` into an OpenXML `.pptx` ZIP package.
- `parse_pptx_package` (function) — Parse an OpenXML `.pptx` ZIP package into a `Presentation`.
- `sniff_delimiter` (function) — Sniff the most likely delimiter based on counts in the first line.
- `to_xlsx_package` (function) — Serialize an `XlsxWorkbook` into an OpenXML `.xlsx` ZIP package.
- `parse_xlsx_package` (function) — Parse an OpenXML `.xlsx` ZIP package into an `XlsxWorkbook`.
- `to_docx_package` (function) — Serialize a `FlowDoc` into an OpenXML `.docx` ZIP package.
- `parse_docx_package` (function) — Parse an OpenXML `.docx` ZIP package into a `FlowDoc`.
- `parse_xml` (function) — Parse XML string into an element tree.
- `crc32` (function) — Standard CRC-32 (IEEE 802.3).
- `write_to_bytes` (method) — Serialize all contained files into a standard ZIP archive byte vector.
- `parse` (method) — Parse a ZIP byte slice and extract uncompressed (stored) entries.

**numerics**

- `parse_csv` (function) — Parse CSV/TSV input into a 2D matrix of strings (`Vec<Vec<String>>`).
- `to_csv` (function) — Serialize a 2D matrix of string cells into RFC 4180 CSV / TSV format.

**semantics**

- `to_html` (function) — Serialize a `FlowDoc` to clean, semantic HTML5.

_61 undocumented functions — see `modules/crates_chronicle-office-codecs.md`._

### `crates/chronicle-tabular-engine` — 274 functions (17 documented)

_Calculation engine: DAG recalculation, evaluator, workbook model._

**3d**

- `std_norm_pdf` (function) — Standard normal PDF: phi(x)
- `std_norm_cdf` (function) — Standard normal CDF: Phi(x)
- `std_norm_inv` (function) — Inverse standard normal CDF (Acklam approximation)

**general**

- `a1` (method) — `A1`-style text, e.g. `B7`.
- `offset` (method) — Moved by `(dr, dc)`, `None` when the result leaves the sheet.
- `offset_clamped` (method) — Moved by `(dr, dc)` and clamped to the sheet boundaries.
- `parse` (method) — Parses `A1`, `$A$1`, `xfd1048576` (case-insensitive). `None` if malformed or out of range.
- `parse_a1_prefix` (function) — Parses an A1 reference at the start of `s`: `(col, row, col_abs, row_abs, rest)`.
- `col_to_letters` (function) — 0 -> `A`, 25 -> `Z`, 26 -> `AA`, 16383 -> `XFD`.
- `letters_to_col` (function) — `A` -> 0, `xfd` -> 16383. `None` for empty, non-letters or beyond `XFD`.
- `rows` (method) — Whole rows `r0..=r1`.
- `cols` (method) — Whole columns `c0..=c1`.
- `iter` (method) — Row-major iterator over the cells.
- `a1` (method) — `A1:B2`, `A1` for single cells, `1:3` for full rows, `A:C` for full columns.
- `parse` (method) — Parses `A1`, `A1:B2`, `$A$1:$B$2`, `A:C`, `3:5` (no sheet prefix).
- `days_from_civil` (function) — Days from 0000-03-01 style civil algorithm (Howard Hinnant), days since 1970-01-01.
- `erf` (function) — Error function erf(x) via Chebyshev approximation

_257 undocumented functions — see `modules/crates_chronicle-tabular-engine.md`._

### `crates/chronicle-timeline-interchange` — 90 functions (21 documented)

_Caption formats (SRT, WebVTT, CEA-608, CEA-708, and MacCaption MCC)._

**general**

- `align_cues` (function) — Align all cues in a caption document according to the specified options.
- `with_parity` (function) — Add odd parity bit to a 7-bit value.
- `strip_parity` (function) — Verify and strip odd parity bit from a byte.
- `basic_char` (function) — Decode basic character set (0x20..=0x7F).
- `from_sequence` (method) — Convert a TimelineSequence into an AAF Document model.
- `to_sequence` (method) — Convert this AAF Document model into a TimelineSequence.
- `parse_edl` (function) — Parse a CMX 3600 EDL document into a TimelineSequence.
- `write_edl` (function) — Serialize a TimelineSequence to CMX 3600 EDL text format.
- `parse_fcp7_xml` (function) — Parse FCP7 XML text into a TimelineSequence.
- `write_fcp7_xml` (function) — Serialize a TimelineSequence to FCP7 XML (`xmeml`) string.
- `parse_fcpxml_time` (function) — Parse an FCPXML time string (`"1001/24000s"`, `"5s"`, `"0s"`, `"3.5s"`) into a `Tick`.
- `format_fcpxml_time` (function) — Format a `Tick` as an FCPXML rational time string.
- `parse_fcpxml` (function) — Parse an FCPXML document string into a TimelineSequence.
- `write_fcpxml` (function) — Serialize a TimelineSequence to an FCPXML document string.
- `parse_otio` (function) — Parse an OpenTimelineIO JSON string into a TimelineSequence.
- `write_otio` (function) — Serialize a TimelineSequence to OpenTimelineIO JSON format.
- `format_timecode` (function) — Convert frame count to SMPTE timecode (HH:MM:SS:FF or HH:MM:SS;FF).
- `parse_timecode` (function) — Parse a timecode string (`HH:MM:SS:FF` or `HH:MM:SS;FF`) into frame count.
- `parse_clock` (function) — Parse clock format: `HH:MM:SS.mmm` or `HH:MM:SS,mmm` into `Tick`.
- `format_clock` (function) — Format `Tick` as SRT/WebVTT clock (`HH:MM:SS,mmm` or `HH:MM:SS.mmm`).
- `parse` (method) — Parse an XML text document into an `XmlNode` tree.

_69 undocumented functions — see `modules/crates_chronicle-timeline-interchange.md`._

### `crates/color-transforms` — 288 functions (91 documented)

_Pure-Rust ICC v2/v4 color management module.

Provides ICC profile parsing & serialization, PCS XYZ/Lab D50 conversion,
4 rendering intents, Black Point Compensation, tetrahedral CLUT interpolation,
built-in standard profiles, and out-of-gamut detection._

**2d**

- `peak_nits` (method) — The content's peak for tone mapping: MaxCLL when signalled (the brightest pixel actually
- `has_pixel_stage` (method) — Whether anything happens after the per-channel table.
- `apply` (method) — The per-pixel stage (input: the table's output).
- `convert` (method) — Signal → working linear for one RGB triple (table + pixel stage).

**3d**

- `primaries` (method) — (x, y) of R, G, B. Sources: ITU-R BT.709-6, BT.2020-2, SMPTE EG 432-1 (P3), and the camera
- `gamut_compress` (function) — Compress out-of-gamut colours towards the achromatic axis (see module docs). Used on input,
- `log_to_rec709` (function) — Convenience: a log colour space → display Rec. 709 (tone mapped), used for the built-in

**computational geometry**

- `to_3dl` (method) — Write a `.3dl` (10-bit input mesh, 12-bit output). Shapers are baked into the cube.

**general**

- `new` (method) — The grading space of a working space, with HDR White `white_nits` (ignored for SDR).
- `white_nits` (method) — HDR White in cd/m² (None for SDR).
- `signal_of_nits` (method) — The signal of a grey of `nits` cd/m² (HDR) — what a nit-scaled slider position means.
- `nits_of_signal` (method) — The cd/m² of a grey of signal `s` (HDR).
- `nodes` (method) — Number of nodes in the grid.
- `eval` (method) — Interpolates at `input` into `out`.
- `tetra` (method) — Tetrahedral interpolation inside one cube of three dimensions.
- `eval_trilinear` (method) — Trilinear interpolation (3 inputs).
- `eval_multilinear` (method) — Multilinear interpolation over all 2^n cube corners.
- `srgb_trc` (function) — The sRGB transfer function (IEC 61966-2.1) as ICC parametric type 3.
- `eval64` (method) — Evaluates the curve at `x` (f64 reference path).
- `eval_inverse64` (method) — Evaluates the inverse curve at `y`.
- `eval32` (method) — Evaluates the curve at `x` (f32).
- `generate_mask` (method) — Evaluates a mask buffer: writes 255 for out-of-gamut pixels, 0 for in-gamut pixels.
- `invert` (function) — Inverse, or `None` when (nearly) singular.
- `s15f16_round` (function) — Round to 16-bit fractional fixed point.
- `d50_quantized` (function) — Exact quantized D50 in ICC header fixed-point representation.
- `xy_to_xyz` (function) — XYZ (Y = 1) of a chromaticity `(x, y)`.
- `xyz_to_lab` (function) — CIE XYZ → CIE L*a*b* under white point `white`.
- `lab_to_xyz` (function) — CIE L*a*b* → CIE XYZ under white point `white`.
- `delta_e76` (function) — CIE ΔE*ab (1976) Euclidean color difference.
- `convert_f32` (method) — Converts interleaved float pixels.
- `convert_u8` (method) — Converts interleaved 8-bit pixels.
- `kr_kb` (method) — (Kr, Kb) luma coefficients.
- `normalize_y` (function) — Normalise an integer code value to (Y 0..1, C -0.5..0.5) given bit depth and range.
- `pq_eotf` (function) — SMPTE ST 2084 EOTF, output normalised to 10 000 nits = 1.0.
- `rgb_to_hsl` (function) — RGB (0..1) â†’ HSL (h in 0..1).
- `parse_hex` (function) — Parse `#rrggbb` / `#rrggbbaa` into 0..1 floats.
- `signal_from_normalized` (function) — Decoder-normalised value (0..1 in the file's range) → curve-domain signal.
- `normalized_from_signal` (function) — Curve-domain signal → decoder-normalised value (0..1 in the file's range).
- `published_reference_values` (function) — Reference points from the published specifications (code values are 10-bit).
- `from_fn` (method) — Sample a colour transform on a `size³` grid over 0..1.
- `parse_cube` (method) — Parse a `.cube` file that contains a 3D table (a shaper, if present, is rejected; use
- `coord` (method) — Normalised grid coordinate (0..size-1) of an input value.
- `apply` (method) — Tetrahedral interpolation.
- `apply_trilinear` (method) — Trilinear interpolation (kept for comparison; LUTs render tetrahedrally).
- `parse` (method) — Parse by format (sniffs `LUT_*_SIZE` for `.cube` when the format is unknown).
- `parse_3dl` (method) — Parse an Autodesk `.3dl`.
- `baked_cube` (method) — The LUT as a single cube over 0..1 (resampled when there is a shaper or a non-unit domain).
- `bit_max` (function) — Full-scale value of the smallest common integer depth that holds `v`.
- `parse_grd` (function) — Parses a `.grd` binary stream.
- `write_grd` (function) — Serializes gradients into `.grd` format.
- `to_xyz` (method) — RGB → CIE XYZ (normalised so white Y = 1).
- `luma` (method) — Luminance weights (the Y row of [`Gamut::to_xyz`]).
- `id` (method) — Stable id used in commands and project files.
- `parse` (method) — Parse an id or a label (case-insensitive, punctuation ignored).
- `is_hdr` (method) — Carries more than SDR range (needs tone mapping into an SDR working space).
- `from_info` (method) — The colour space signalled by stream metadata (VUI / `colr` / MKV Colour).
- `output_space` (method) — The colour space exports of this sequence are encoded in.
- `for_curve` (method) — The decode table for a colour space's curve.
- `bt2390_eetf` (function) — ITU-R BT.2390 EETF: maps `nits` from a display with peak `src_peak` to one with peak
- `new` (method) — `src_peak_nits` is the mastering/content peak of PQ media when the file signals it.
- `gamut_within` (function) — Whether gamut `a` lies inside gamut `b` (so no compression is needed going a → b).
- `desaturate_into_gamut` (function) — Bring a colour with negative components into gamut by desaturating towards its luminance
- `new` (method) — For export/encode into `target` (Rec709, Rec2100Pq or Rec2100Hlg).
- `domain_value` (function) — Decoder-normalised value of a curve-domain value — re-exported for callers that build their

**graph**

- `new` (method) — Builds a table from node data.
- `sample` (method) — Builds a table by sampling `f` at every node (inputs normalised to `[0, 1]`).

**numerics**

- `encode` (method) — Working linear → grading signal (1 = white; HDR values above HDR White are > 1).
- `decode` (method) — Grading signal → working linear. SDR clamps to 0…1 (white); HDR keeps values above HDR
- `bradford` (function) — Bradford chromatic adaptation matrix from white `src` to white `dst` (both XYZ).
- `rgb_to_xyz_matrix` (function) — RGB → XYZ matrix (relative to `white`, Y = 1) for primaries given as chromaticities.
- `from_code` (method) — From an ISO/IEC 23091-2 `matrix_coefficients` code.
- `ycbcr_to_rgb` (function) — Convert normalised Y'CbCr (Y in 0..1, Cb/Cr in -0.5..0.5) to non-linear R'G'B'.
- `rgb_to_ycbcr` (function) — Convert non-linear R'G'B' to normalised Y'CbCr.
- `srgb_to_linear` (function) — sRGB electro-optical transfer (encoded â†’ linear).
- `to_linear` (function) — Encoded â†’ linear for a transfer function (display-referred; PQ normalised to 100 nits = 1.0).
- `srgb_u8_to_linear_table` (function) — 256-entry table: sRGB-encoded u8 â†’ linear f32.
- `linear_to_srgb_u8_table` (function) — 4096-entry table: linear (0..1, quantised to 12 bits) â†’ sRGB-encoded u8.
- `encode` (method) — Scene-linear → signal (curve domain).
- `decode` (method) — Signal (curve domain) → scene-linear.
- `peak_linear` (method) — Scene-linear value of the maximum signal (code 1023), used as the source peak for tone
- `npm` (function) — Normalised primary matrix (SMPTE RP 177).
- `gamut_matrix` (function) — Linear RGB in `src` → linear RGB in `dst`.
- `working_gamut` (method) — Linear primaries of the compositing space.
- `is_plain` (method) — The classic pipeline: linear BT.709, display-referred SDR.
- `decode_channel` (function) — Decode one channel: decoder-normalised signal → working-linear (no OOTF, no matrix).
- `encode_channel` (function) — Encode one channel: working-linear → decoder-normalised signal (inverse of [`decode_channel`]).
- `display` (method) — For the monitors: SDR BT.709 linear (encode with the sRGB curve as usual).
- `apply` (method) — Working linear → target linear (SDR: 1.0 = white; HDR: working units).
- `encode` (method) — Working linear → encoded signal (0..1, full scale) of the target colour space.

**physics**

- `brute_tetra` (function) — Brute-force tetrahedral reference: find the Kuhn simplex (one of the 6 tetrahedra along the

**semantics**

- `luma709` (function) — Rec.709 luma of a linear or encoded RGB triple.

_197 undocumented functions — see `modules/crates_color-transforms.md`._

### `crates/discourse-academic-tex` — 208 functions (44 documented)

_Academic bibliography utilities: identifier parsing, format conversion, and audit._

**general**

- `audit_bib` (function) — Run audit rules over a [`BibFile`].
- `parse_identifier` (function) — Parse any pasted identifier into a normalized [`Ident`].
- `sniff` (function) — Sniff source format: JSON openers -> CSL-JSON, `TY  -` tagged lines -> RIS.
- `bibval` (function) — BibTeX-safe value: braces kept balanced.
- `emit` (function) — Emit `@kind{key, fields…}` from `(name, value)` pairs.
- `year_of` (function) — First 4-digit run.
- `norm_title` (function) — Normalized title for duplicate detection.
- `convert` (function) — Convert source buffer (CSL-JSON or RIS) to BibTeX entries.
- `catalog` (function) — The full catalog, sorted by [`CommandSpec::name`].
- `find_prefix` (function) — Commands and environments whose `name` starts with `prefix` (case-sensitive).
- `find_exact` (function) — Exact lookup by `name` (case-sensitive).
- `packages` (function) — Distinct package names present in the catalog, sorted.
- `environments` (function) — The catalog entries that are environments (`\begin{name}`…`\end{name}`).
- `propose_edits` (function) — Propose a set of deterministic line edits to repair document structure.
- `apply_edits` (function) — Apply a sequence of edits bottom-up so line numbers remain stable.
- `enrich_diagnostics` (function) — Enrich diagnostic items in place.
- `global_snippets_path` (function) — Global snippets path: `$ACADEMIC_TEX_HOME/snippets.json`, else `~/.academic-tex/snippets.json`.
- `parse_snippets` (function) — Parse a snippet JSON string: accepts `[{trigger,body,...}]` or `{trigger: body}` / `{trigger: {body,...}}`.
- `read_snippets_file` (function) — Read a snippet file bounded; missing or invalid returns empty vec.
- `load_snippets` (function) — Load and merge all snippets given an optional project directory root.
- `expand` (function) — Render `body`: `${n:default}` -> `default`, `$n` -> removed; caret lands
- `resolve` (function) — Resolve anchor against source text.
- `context_around` (function) — Capture context around a range in `src` on char boundaries.
- `apply_suggestion` (function) — Apply a suggestion to `src`, returning the new source if uniquely anchored.
- `field` (method) — Case-insensitive field lookup.
- `get` (method) — Entry lookup by key (exact match).
- `parse_bib` (function) — Parse a `.bib` source string. Always returns a file object; problems land in `issues`.
- `extract_deps` (function) — Extract every file-level dependency reference from `src`.
- `lex` (function) — Tokenize `src`. Never fails: every byte lands in some token.
- `engine_hint` (method) — Normalised build engine hinted by `program`/`ts-program`, falling back to `% !BIB program`.
- `parse` (function) — Read magic-comment header of a `.tex` source.
- `analyze` (function) — Compute stats for `.tex` source `src` in single O(n) pass.
- `parse_structure` (function) — Lex `src` and extract its structure. Always succeeds — malformed input lands in `issues`.
- `parse` (method) — Parse TrueType table directory + consumed tables.
- `gid` (method) — Unicode scalar -> glyph ID (0 = .notdef).
- `advance` (method) — Advance width of a glyph in font units.
- `char_width` (method) — Advance width of a character in font units (0 for .notdef).
- `scale` (method) — Font-units -> pt scale at `size` pt.
- `width_pt` (method) — Advance of `c` in pt at `size`.
- `save` (method) — Serialize document to valid PDF 1.5 format.
- `symbol_of` (function) — Greek + symbol command names -> their Unicode scalar.
- `char_class` (function) — Class of a raw character for spacing purposes.
- `sep` (function) — Space (in fractions of `size`) between adjacent atoms.
- `math` (function) — Typeset a math fragment to positioned items.

_164 undocumented functions — see `modules/crates_discourse-academic-tex.md`._

### `crates/discourse-pdf-compliance` — 53 functions (16 documented)

_PDF/UA and WCAG accessibility compliance module._

**general**

- `flatten_form` (function) — Flattens an AcroForm: renders the appearance of every field into static elements
- `export_xfdf` (function) — Export AcroForm field values as an XML XFDF document.
- `import_xfdf` (function) — Import field values from an XML XFDF document into an AcroForm.
- `sanitize_document` (function) — Sanitizes a document manifest by removing all selected hidden information categories.
- `luhn_check` (function) — Luhn algorithm for credit card checksum verification.
- `scan_credit_cards` (function) — Scan text for credit card numbers (Visa, MasterCard, Amex, Discover, etc.) with Luhn verification.
- `scan_ssns` (function) — Scan text for US Social Security Numbers (format `AAA-GG-SSSS` or 9 continuous digits with valid area).
- `scan_tax_ids` (function) — Scan text for Employer Identification Numbers / Tax IDs (`XX-XXXXXXX`).
- `scan_emails` (function) — Scan text for email addresses.
- `scan_phones` (function) — Scan text for phone numbers (E.164, North American NANP, and international trunk).
- `scan_all_pii` (function) — Scans for all PII patterns concurrently.
- `verify_zero_residue` (function) — Zero-residue verification pass:
- `sha256_slices` (function) — Computes SHA-256 hash over an array of byte slices (e.g. non-contiguous ByteRange segments).
- `validate` (method) — Verifies that the ByteRange covers the file cleanly without overlaps,
- `verify_pades_byte_range` (function) — Verifies that a signed document's byte range digest matches the signed content.

**semantics**

- `apply_semantic_redaction` (function) — Applies irreversible, fail-closed redaction to an in-memory text buffer.

_37 undocumented functions — see `modules/crates_discourse-pdf-compliance.md`._

### `crates/discourse-scientific-matlang` — 77 functions (7 documented)

_Pure-Rust M-code interpreter module._

**3d**

- `svd` (function) — SVD via normal equations A'A: A = U S V'.

**general**

- `slice` (method) — 2D slicing: extracts submatrix over [r_start..r_end] x [c_start..c_end] (0-based, half-open).
- `to_svg` (method) — Renders figure to a standalone SVG XML string.

**numerics**

- `lu` (function) — LU decomposition with partial pivoting: P A = L U.
- `chol` (function) — Cholesky factorization: R' R = A for symmetric positive-definite A.
- `qr` (function) — Householder QR decomposition: A = Q R.
- `eig_symmetric` (function) — Jacobi eigenvalue decomposition of a symmetric matrix: V' A V = diag(lambda).

_70 undocumented functions — see `modules/crates_discourse-scientific-matlang.md`._

### `crates/drafting-geometry` — 264 functions (62 documented)

_Double-precision 2D/3D math for drafting.

Everything a drafting kernel needs below the document model: vectors and affine transforms,
bounding boxes, angles, lines, circular arcs (and polyline bulges), ellipses, NURBS splines,
intersections, closest points, offsets and tessellation. All angles are radians,
counter-clockwise from +X, as in DXF._

**2d**

- `project` (method) — Foot of the perpendicular on the infinite line.
- `side` (method) — Signed side: > 0 left of a→b.
- `at` (method) — Point at parameter `t` in 0..1 along the sweep.
- `from_3_points` (method) — Arc through three points, or `None` when collinear.
- `from_start_center_end` (method) — Arc from start point, center and end point (CCW).
- `tangent_points` (method) — Tangent points from external point `p`.
- `common_tangents` (method) — Lines tangent to both `self` and `other`, as (point on `self`, point on `other`) pairs:
- `param_of` (method) — Parameter of the point on the ellipse nearest the direction of `p` (approximate).
- `nearest_on_polyline` (function) — The nearest point to `p` on a polyline (through tessellated points).
- `tessellate` (method) — Points along the segment in its direction, including both ends.
- `at` (method) — Point at parameter `t` (0..1) in segment direction.
- `tangent` (method) — Tangent direction (unit) at parameter t.
- `bulge_to_arc` (function) — Convert a bulge segment to an arc. `None` for a straight (zero-bulge or degenerate) segment.
- `contains` (method) — True if `p` is inside a closed polyline (even-odd on the tessellation).
- `line_circle` (function) — Intersections of the infinite line through `l` with a circle. Returns points with line params.
- `circle_circle` (function) — Intersections of two full circles.
- `intersect_segments` (function) — Intersections between two segments (finite).
- `intersect_segments_ext` (function) — Intersections; with `extend` true, lines are infinite and arcs are full circles
- `scale_about` (method) — Scale about `base`.
- `mirror` (method) — Reflection across the line through `p1`, `p2`.
- `enclosing_loop` (function) — The closed loop enclosing `p`, counter-clockwise, or `None`.
- `boundary_at` (function) — The closed loop enclosing `p` plus the islands inside it, or `None` when `p` isn't enclosed.
- `from_control` (method) — A clamped uniform spline over control points.
- `from_fit_points` (method) — Cubic interpolation through fit points (chord-length parametrisation, natural-ish ends
- `from_fit_points_closed` (method) — Smooth closed (periodic) cubic interpolation through fit points. Chord-length parameters
- `from_fit` (method) — Interpolation through fit points, open or closed (see `from_fit_points_closed`).
- `eval` (method) — Evaluate at parameter `t` (de Boor via basis functions).
- `cross` (method) — z of the 3D cross product.
- `normalized` (method) — Unit vector, or zero for a zero-length vector.
- `perp` (method) — Rotated 90° counter-clockwise.
- `mirror` (method) — Mirror across the infinite line through `a` and `b`.

**3d**

- `param_of` (method) — Parameter (0..1 on the segment) of the projection of `p`.
- `ocs` (method) — The "arbitrary axis algorithm" frame for an extrusion direction (DXF OCS → WCS).
- `trace` (function) — Trace the face left of half-edge u→v (vertex indices), giving up after `max_steps` edges.
- `outer_face` (function) — The face around `p`, as vertex indices (counter-clockwise).
- `components` (function) — Connected component id of every vertex.
- `islands` (function) — Outlines of the groups of curves inside the face `outer` (vertex indices into `g`).

**computational geometry**

- `closest` (method) — Closest point on the segment.
- `shoelace` (function) — Signed polygon area (CCW positive).
- `point_in_polygon` (function) — Even-odd point-in-polygon test.
- `line_line_infinite` (function) — Intersection of two infinite lines through (a1,a2) and (b1,b2), with parameters on each.
- `line_line` (function) — Intersection of two finite segments.

**general**

- `contains_box` (method) — True when `o` lies entirely inside.
- `offset` (method) — Offset by `d` to the left of travel direction. `None` if an arc collapses.
- `arc_to_bulge` (function) — Bulge for an arc traversed from `a` to `b` with the given sweep (signed: + = CCW).
- `segments` (method) — The segments, including the closing segment when closed.
- `area` (method) — Signed area (CCW positive) of a closed polyline including bulge areas.
- `parse_dxf` (function) — Parses a DXF ASCII string into drafting entities.
- `haversine_km` (function) — Great-circle distance between two (latitude, longitude) pairs in degrees.
- `vincenty_wgs84_km` (function) — Vincenty inverse geodesic calculation on the WGS84 ellipsoid.
- `norm_angle` (function) — Normalize an angle to `[0, 2π)`.
- `ccw_sweep` (function) — Counter-clockwise sweep from `start` to `end`, in `(0, 2π]`.
- `angle_in_sweep` (function) — True when angle `a` lies on the CCW sweep from `start` to `end` (inclusive, with tolerance).
- `arc_segments` (function) — Number of segments needed to approximate an arc of `radius` and `sweep` within `tol`.
- `then_before` (method) — `self * o`: apply `o` first, then `self`.
- `then` (method) — Apply `self` first, then `o`.
- `scale_factor` (method) — Uniform scale factor (geometric mean of the axis scales).
- `rotation` (method) — Rotation angle of the transformed X axis.
- `is_periodic` (method) — Closed with an unclamped (periodic) knot vector, as DXF flags it (group 70 bit 2).
- `from_angle` (method) — Unit vector at `angle` radians.

**graph**

- `prune` (function) — The graph without dangling ends: vertices of degree one are removed until none remain, so

**numerics**

- `solve` (function) — Gaussian elimination with partial pivoting.

_202 undocumented functions — see `modules/crates_drafting-geometry.md`._

### `crates/electromagnetics-rf-sensing` — 41 functions (16 documented)

_Pure-Rust Wi-Fi 802.11n/ax/be Channel State Information (CSI) spatial sensing,
contactless vital signs extraction, MUSIC Angle-of-Arrival (AoA) beamforming,
17-keypoint RF skeletal pose, and camera-free privacy witness verification.

Grounded in international human rights instruments (UDHR Articles 12, 18, 19, 27; ICCPR Article 17)
and Informatics Fiduciary duties ensuring camera-free privacy-first physical sensing._

**general**

- `steering_vector` (method) — Computes steering vector $a(\theta)$ for incidence angle $\theta$ (degrees, -90..+90).
- `music_aoa_spectrum` (function) — Evaluates MUSIC pseudospectrum over angular search grid (-90..+90 degrees).
- `find_dominant_aoa_peak` (function) — Finds the dominant peak in a MUSIC spectrum.
- `get_amplitudes` (method) — Extracts amplitudes for a specific Tx-Rx antenna pair across all subcarriers.
- `get_phases` (method) — Extracts raw phases (radians) for a specific Tx-Rx antenna pair.
- `unwrap_phase` (function) — Unwraps phase values along a sequence to remove 2pi phase wrap discontinuities.
- `hampel_filter` (function) — Hampel outlier removal filter: replaces outliers with local window median.
- `radius_at_distance` (method) — Calculates the radius $r_n$ of the $n$-th Fresnel zone at distance $d_1$ from Tx along the LOS path.
- `synthesize_standing` (method) — Generates a normalized human skeletal pose centered at (center_x, center_depth, height_z).
- `create` (method) — Creates and cryptographically signs a privacy witness receipt.
- `verify` (method) — Verifies the cryptographic witness receipt integrity and human-rights compliance guarantee.
- `extract_vital_signs` (function) — Extracts respiration and cardiac pulse rates from a time-series of CSI amplitude / sanitized phase variations.

**numerics**

- `compute_spatial_covariance` (function) — Computes the $M \times M$ complex spatial covariance matrix $R_{xx} = \frac{1}{K} \sum x_k x_k^H$.
- `sanitize_csi_phase` (function) — Removes Carrier Frequency Offset (CFO) and Sampling Time Offset (STO) via linear regression.

**physics**

- `compute_doppler_spectrum` (function) — Computes a short-time Doppler velocity spectrum from a time-series of CSI amplitude changes.
- `update` (method) — Feeds max Doppler velocity (m/s) observed in the current frame and updates fall status.

_25 undocumented functions — see `modules/crates_electromagnetics-rf-sensing.md`._

### `crates/geodesy-spatial-analysis` — 83 functions (38 documented)

_Categorical geodesy, planetary ellipsoids, Karney WGS84 geodesic solvers, Ordinary Kriging,
DEM morphometry, network topology, and cartographic layout.

Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
and Informatics Fiduciary duties ensuring scientific precision and geographic accuracy._

**3d**

- `forward` (method) — Forward projection: lon/lat in degrees -> planar metres east and north of centre.
- `inverse` (method) — Inverse projection: planar metres -> lon/lat in degrees.
- `forward` (method) — Forward projection: lon/lat degrees -> projected metres.
- `inverse` (method) — Inverse projection: projected metres -> lon/lat degrees.
- `tissot_distortion` (function) — Computes Tissot distortion of a projection at a given coordinate using Snyder's central differences.

**computational geometry**

- `geodesic_ring_area_m2` (function) — Area of a closed polygon ring on the spherical earth (square metres, always positive).
- `geodesic_buffer_point` (function) — Buffers a single point by `distance_m` on the geodetic manifold, returning a polygon ring.
- `segment_intersection` (function) — Returns the intersection point of two 2D segments if they intersect strictly.

**general**

- `compute_class_breaks` (function) — Computes class break boundaries for graduated symbology.
- `fisher_jenks_breaks` (function) — Fisher-Jenks natural breaks optimization algorithm for 1D sorted data.
- `layout_curved_line_label` (function) — Places text characters along a polyline, rotating each character to follow the line's curvature.
- `nice_scale_distance` (function) — Computes nice distance for scale bars given a nominal distance.
- `generate_graticule` (function) — Generates geographic graticule lines (meridians and parallels) at specified degree step.
- `haversine_m` (function) — Great-circle Haversine distance in metres between two points in degrees (lon, lat).
- `wgs84_inverse` (function) — WGS84 geodesic inverse problem: computes distance in metres and initial/final azimuths in degrees.
- `wgs84_direct` (function) — WGS84 geodesic direct problem: computes destination (lat, lon) and arrival azimuth from start, azimuth, and distance.
- `cell_area_m2` (method) — Nominal area of a DGGS cell at resolution `res` (0..=30) in square metres.
- `cell_boundary` (method) — Generates boundary vertices for an approximate DGGS hexagonal/pentagonal cell given its centre and resolution.
- `cut_antimeridian_line` (function) — Splits a polyline at the antimeridian (where consecutive points jump across > 180 degrees longitude).
- `semivariance` (method) — Evaluates semivariance $\gamma(h)$ for lag distance $h$.
- `empirical_variogram` (function) — Computes experimental semivariogram bins from sample points.
- `fit_variogram` (function) — Automatically fits a variogram model to empirical bins using weighted least squares.
- `ordinary_kriging_grid` (function) — Interpolates a regular raster grid from scattered points using Ordinary Kriging.
- `surface_gradient` (function) — Computes the surface gradient $(dz/dx, dz/dy)$ at a cell in metres per metre.
- `compute_slope` (function) — Computes slope in degrees (0..90).
- `compute_aspect` (function) — Computes aspect in degrees clockwise from North (0..360, 0 = North, 90 = East, etc.). Flat terrain is assigned -1.0.
- `compute_hillshade` (function) — Computes analytical hillshade illumination values (0..255).
- `compute_curvature_at` (function) — Computes profile and plan curvature at (col, row) using Zevenbergen-Thorne polynomial coefficients.
- `marching_squares_contours` (function) — Extracts contour polylines from a DEM grid at a specified interval using the Marching Squares algorithm.
- `compute_zonal_stats` (function) — Computes zonal statistics for cells matching `target_zone` in a zone integer raster.
- `tsp_2opt_solver` (function) — Traveling Salesperson Problem (TSP) solver using Nearest-Neighbor tour + 2-Opt heuristic optimization.
- `check_duplicate_vertices` (function) — Checks for consecutive duplicate vertices within a distance tolerance.
- `check_self_intersections` (function) — Checks for self-intersections within a polyline.
- `check_dangles` (function) — Checks for dangles (endpoints of lines that do not connect to any other line endpoint within tolerance).

**graph**

- `dijkstra_shortest_path` (function) — Computes the shortest route between start_node and end_node using Dijkstra's algorithm.
- `calculate_isochrone` (function) — Computes isochrone reachable nodes from start within `max_cost` budget.

**numerics**

- `predict_kriging` (function) — Solves Ordinary Kriging linear system for a target point.
- `solve_linear_system` (function) — In-place linear solver for `Ax = b` using Gauss-Jordan elimination with partial pivoting.

_45 undocumented functions — see `modules/crates_geodesy-spatial-analysis.md`._

### `crates/gpu-command-profiler` — 72 functions (21 documented)

_!

Easy to use profiler scopes for [wgpu](https://github.com/gfx-rs/wgpu) using timer queries.

`wgpu_profiler` manages all the necessary [`wgpu::QuerySet`] and [`wgpu::Buffer`] behind the scenes
and allows you to create to create timer scopes with minimal overhead!

# How to use

```
use wgpu_profiler::*;

# async fn wgpu_init() -> (wgpu::Instance, wgpu::Adapter, wgpu::Device, wgpu::Queue) {
    # let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    # let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    # let (device, queue) = adapter
    #     .request_device(&wgpu::DeviceDescriptor {
    #         required_features: wgpu::Features::TIMESTAMP_QUERY,
    #         ..Default::default()
    #     })
    #     .await
    #     .unwrap();
    # (instance, adapter, device, queue)
# }
# let (instance, adapter, device, queue) = futures_lite::future::block_on(wgpu_init());
# let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
#     label: None,
#     source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("../examples/compute_shader.wgsl"))),
# });
# let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
#        label: None,
#        layout: None,
#        module: &cs_module,
#        entry_point: Some("main"),
#        compilation_options: wgpu::PipelineCompilationOptions::default(),
#        cache: None,
#    });
// ...

let mut profiler = GpuProfiler::new(&device, GpuProfilerSettings::default()).unwrap();

// ...

# let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
{
    // You can now open profiling scopes on any encoder or pass:
    let mut scope = profiler.scope("name of your scope", &mut encoder);

    // Scopes can be nested arbitrarily!
    let mut nested_scope = scope.scope("nested!");

    // Scopes on encoders can be used to easily create profiled passes!
    let mut compute_pass = nested_scope.scoped_compute_pass("profiled compute");


    // Scopes expose the underlying encoder or pass they wrap:
    compute_pass.set_pipeline(&pipeline);
    // ...

    // Scopes created this way are automatically closed when dropped.
}

// Wgpu-profiler needs to insert buffer copy commands.
profiler.resolve_queries(&mut encoder);
# drop(encoder);

// ...

// And finally, to end a profiling frame, call `end_frame`.
// This does a few checks and will let you know if something is off!
profiler.end_frame().unwrap();

// Retrieving the oldest available frame and writing it out to a chrome trace file.
if let Some(profiling_data) = profiler.process_finished_frame(queue.get_timestamp_period()) {
    # let button_pressed = false;
    // You usually want to write to disk only under some condition, e.g. press of a key.
    if button_pressed {
        wgpu_profiler::chrometrace::write_chrometrace(
            std::path::Path::new("mytrace.json"), &profiling_data);
    }
}
```
Check also the [Example](https://github.com/Wumpf/wgpu-profiler/blob/main/examples/demo.rs) where everything can be seen in action.

## Tracy integration

If you want to use [tracy](https://github.com/wolfpld/tracy) for profiling, you can enable the `tracy` feature.

This adds `wgpu_profiler::new_with_tracy_client` which will automatically report profiling data to tracy.

For details check the example code.

## Puffin integration

If you want to use [puffin](https://github.com/EmbarkStudios/puffin) for profiling, you can enable the `puffin` feature.

This adds `wgpu_profiler::puffin::output_frame_to_puffin` which makes it easy to report profiling data to a `puffin::GlobalProfiler`.

You can run the demo example with puffin by running `cargo run --example demo --features puffin`.
All CPU profiling goes to port `8585`, all GPU profiling goes to port `8586`. You can open puffin viewers for both.
```sh
puffin_viewer --url 127.0.0.1:8585
puffin_viewer --url 127.0.0.1:8586
```

For details check the example code.

# Internals

For every frame that hasn't completely finished processing yet
(i.e. hasn't returned results via [`GpuProfiler::process_finished_frame`])
we keep a `PendingFrame` around.

Whenever a profiling scope is opened, we allocate two queries.
This is done by either using the most recent `QueryPool` or creating a new one if there's no non-exhausted one ready.
Ideally, we only ever need a single `QueryPool` per frame! In order to converge to this,
we allocate new query pools with the size of all previous query pools in a given frame, effectively doubling the size.
On [`GpuProfiler::end_frame`], we memorize the total size of all `QueryPool`s in the current frame and make this the new minimum pool size.

`QueryPool` from finished frames are re-used, unless they are deemed too small._

**general**

- `new` (method) — Creates a new Profiler object.
- `new_with_tracy_client` (method) — Creates a new profiler and connects to a running Tracy client.
- `settings` (method) — Returns currently active settings.
- `change_settings` (method) — Changes the settings of an existing profiler.
- `scope` (method) — Starts a new auto-closing profiler scope.
- `owning_scope` (method) — Starts a new auto-closing profiler scope that takes ownership of the passed encoder or rendering/compute pass.
- `manual_owning_scope` (method) — Starts a new **manually closed** profiler scope that takes ownership of the passed encoder or rendering/compute pass.
- `begin_query` (method) — Starts a new profiler query on the given encoder or rendering/compute pass (if enabled).
- `end_query` (method) — Ends passed query.
- `resolve_queries` (method) — Puts query resolve commands in the encoder for all unresolved, pending queries of the active profiler frame.
- `end_frame` (method) — Marks the end of a frame.
- `process_finished_frame` (method) — Checks if all timer queries for the oldest pending finished frame are done and returns that snapshot if any.
- `timestamp_query_support` (function) — Returns true if a timestamp query is supported.
- `is_pass` (method) — Returns `true` if it's a pass or `false` if it's an encoder
- `render_pass_timestamp_writes` (method) — Use the reserved query for render pass timestamp writes if any.
- `compute_pass_timestamp_writes` (method) — Use the reserved query for compute pass timestamp writes if any.
- `with_parent` (method) — Makes this scope a child of the passed scope.
- `output_frame_to_puffin` (function) — Visualize the query results in a `puffin::GlobalProfiler`.
- `end_query` (method) — Ends the scope allowing the extraction of the owned [`ProfilerCommandRecorder`].

**graph**

- `write_chrometrace` (function) — Writes a .json trace file that can be viewed as a flame graph in Chrome or Edge via <chrome://tracing>

**physics**

- `begin_pass_query` (method) — Starts a new profiler query to be used for render/compute pass timestamp writes.

_51 undocumented functions — see `modules/crates_gpu-command-profiler.md`._

### `crates/gpu-device-context` — 48 functions (21 documented)

_Simple helpers for managing wgpu state and surfaces._

**general**

- `new` (method) — Create a blitter that applies `conversion` and writes to a target
- `copy` (method) — Copy `source` into `target`, applying the alpha conversion.
- `new` (method) — Creates a new render surface for the specified window and dimensions.
- `new_from_compatible_surface` (method) — Creates a `DeviceHandle` with `Device` that's compatible with the specified `Surface`
- `create_surface` (method) — Creates a new surface for the specified window and dimensions.
- `create_surface` (method) — Creates a new surface for the specified window and dimensions.
- `create_surface_renderer` (method) — Creates a new surface for the specified window and dimensions.
- `create_buffer_renderer` (method) — Creates a new `BufferRenderer` for the specified dimensions.
- `find_or_create_device` (method) — Finds or creates a compatible device handle id.
- `find_compatible_device_handle` (method) — Finds or creates a compatible device handle id.
- `find_existing_device_idx` (method) — Finds  a compatible device handle id.
- `create_device` (method) — Creates a compatible device handle id.
- `new` (method) — Creates a new render surface for the specified window and dimensions.
- `resize` (method) — Resizes the surface to the new dimensions.
- `acquire` (method) — Acquire the next surface frame.
- `acquire_reconfiguring_if_stale` (method) — Acquire a usable surface texture. A `Suboptimal` texture is still
- `target_texture_view` (method) — Get a target texture view to render to.
- `maybe_blit_and_present` (method) — Present the texture to the surface. If there is an intermediate texture, this first blits
- `blit_from_intermediate_texture_to_surface` (method) — Blit from the intermediate texture to the surface texture
- `block_on_wgpu` (function) — Block on a future, polling the device as needed.
- `create_texture` (function) — Create a WGPU Texture, returning a default TextureView

_27 undocumented functions — see `modules/crates_gpu-device-context.md`._

### `crates/kinetics-block-simulation` — 267 functions (0 documented)

_Standard simulation blocks library._

_267 undocumented functions — see `modules/crates_kinetics-block-simulation.md`._

### `crates/kinetics-continuum-multiphysics` — 95 functions (55 documented)

_Explicit dynamics: central-difference time integration, hourglass control, and damage erosion._

**computational geometry**

- `export_gmsh_22` (function) — Export mesh to Gmsh MSH 2.2 ASCII format.
- `export_vtu_hex` (function) — Export hexahedral mesh to VTK XML UnstructuredGrid (.vtu) format with nodal displacements.
- `triangulate_2d` (function) — Compute 2D Delaunay triangulation via incremental point insertion.
- `generate_box_hex_mesh` (function) — Generate regular mapped hexahedral grid of nx x ny x nz cells.
- `hex_to_tets` (function) — Generate 5 or 6 tetrahedra from a hexahedral bounding box.

**economics**

- `equilibrium_d3q19` (function) — Equilibrium distribution for incompressible Navier-Stokes.

**general**

- `fracture_strain` (method) — Fracture strain eps_f = [d1 + d2 * exp(d3 * sigma*)] * [1 + d4 * ln(eps_dot*)] * [1 + d5 * T*]
- `update_damage` (method) — Update damage index D = sum(delta_eps_p / eps_f). Returns true if element is eroded (D >= 1.0).
- `step` (method) — Advance one time step using central differences:
- `flanagan_belytschko_hex8` (function) — Compute Flanagan-Belytschko hourglass resisting nodal forces (8 nodes x 3 DOFs).
- `lame_lambda` (method) — First Lamé parameter lambda = E * nu / ((1 + nu) * (1 - 2*nu)).
- `shear_modulus` (method) — Second Lamé parameter (shear modulus) mu = E / (2 * (1 + nu)).
- `bulk_modulus` (method) — Bulk modulus K = E / (3 * (1 - 2*nu)).
- `compute_pk2_stress` (method) — Compute Second Piola-Kirchhoff stress tensor S from Green-Lagrange strain tensor E (3x3).
- `compute_cauchy_stress` (method) — Compute Cauchy stress sigma from deformation gradient F (3x3).
- `radial_return` (method) — Classical radial return algorithm: updates stress, backstress, and equivalent plastic strain.
- `shape_deriv_tri3` (function) — Tri3 shape function derivatives: [d/dxi, d/deta].
- `shape_deriv_quad4` (function) — Quad4 derivatives: [[dN_i/dxi, dN_i/deta]; 4].
- `shape_deriv_tet4` (function) — Tet4 derivatives: [[d/dxi, d/deta, d/dzeta]; 4].
- `shape_deriv_hex8` (function) — Hex8 derivatives: [[d/dxi, d/deta, d/dzeta]; 8].
- `gauss_quad_2x2` (function) — 2x2 Gauss-Legendre quadrature points for quadrilaterals in [-1, 1]^2.
- `gauss_hex_2x2x2` (function) — 2x2x2 Gauss-Legendre quadrature points for hexahedra in [-1, 1]^3.
- `export_abaqus_inp` (function) — Export nodes and hexahedral elements to Abaqus `.inp` format.
- `export_lsdyna_k` (function) — Export to LS-DYNA `.k` format.
- `export_nastran_bdf` (function) — Export to 8-field small-field Nastran BDF format.
- `collide` (method) — Two-relaxation-time (TRT) collision operator with magic parameter Lambda = 1/4.
- `stream` (method) — Stream step with half-way bounce-back on solid boundaries.
- `step` (method) — Step simulation forward one time increment.
- `orient2d` (function) — Orientation test: positive if p0 -> p1 -> p2 is counter-clockwise.
- `in_circle` (function) — In-circle test: positive if p is inside the circumcircle of counter-clockwise (p0, p1, p2).
- `hex_scaled_jacobian` (function) — Scaled Jacobian metric for a hexahedral element: in [-1, 1], where 1 is an ideal cube.
- `tet_volume` (function) — Signed volume of a tetrahedron (positive if nodes satisfy right-hand rule).
- `lanczos_eigensolver` (function) — Shift-and-invert Lanczos iteration to extract lowest eigenvalues of K * x = lambda * M * x.
- `inertia` (method) — Compute Sylvester's inertia from the diagonal entries of D.

**graph**

- `shape_tri3` (function) — Linear 3-node triangle (Tri3) shape functions in area coordinates (xi, eta).
- `shape_tri6` (function) — Quadratic 6-node triangle (Tri6) shape functions.
- `shape_quad4` (function) — Bilinear 4-node quadrilateral (Quad4) shape functions in [-1, 1]^2.
- `shape_quad8` (function) — Serendipity 8-node quadrilateral (Quad8) shape functions in [-1, 1]^2.
- `shape_tet4` (function) — Linear 4-node tetrahedron (Tet4) shape functions in volume coordinates (xi, eta, zeta).
- `shape_hex8` (function) — Trilinear 8-node hexahedron (Hex8) shape functions in [-1, 1]^3.
- `timoshenko_beam_3d` (function) — 3D Timoshenko beam element stiffness matrix (12x12 for 2 nodes with 6 DOFs per node).
- `mitc4_shear_strains` (function) — MITC4 (Mixed Interpolation of Tensorial Components) 4-node shell element.
- `thermal_conduction_quad4` (function) — 4-node 2D thermal conduction element conductivity matrix (4x4).
- `laplacian_smooth_step` (function) — Laplacian smoothing step for interior node positions given neighbor adjacency.

**numerics**

- `elasticity_matrix_3d` (method) — 6x6 elasticity matrix (Voigt notation: [eps_xx, eps_yy, eps_zz, gamma_yz, gamma_xz, gamma_xy]).
- `plane_stress_matrix` (method) — 3x3 plane stress elasticity matrix [eps_xx, eps_yy, gamma_xy].
- `plane_strain_matrix` (method) — 3x3 plane strain elasticity matrix [eps_xx, eps_yy, gamma_xy].
- `gauss_tet_1pt` (function) — 1-point quadrature for linear tetrahedron.
- `factorize` (method) — Factorize symmetric positive-definite n x n matrix in row-major slice.
- `solve` (method) — Solve A * x = b via forward substitution (L * y = b) and backward substitution (L^T * x = y).
- `factorize` (method) — Factorize symmetric n x n matrix.
- `solve` (method) — Solve A * x = b via L * z = b, D * y = z, L^T * x = y.
- `newton_raphson_solve` (function) — Standard full Newton-Raphson nonlinear solver for residual R(u, lambda) = lambda * P_ref - F_int(u) = 0.
- `from_triplets` (method) — Construct CSR matrix from triplets (row, col, value).
- `matvec` (method) — Matrix-vector multiplication y = A * x.

_40 undocumented functions — see `modules/crates_kinetics-continuum-multiphysics.md`._

### `crates/kinetics-garment-draping` — 76 functions (8 documented)

_kinetics-garment-draping: UI-agnostic categorical garment mechanics and draping.

Provides second-order forward automatic differentiation (Jet<N>), nonlinear
orthotropic membrane & flat-rest hinge bending energies, calibrated textile libraries,
CPU f64 Vertex Block Descent (VBD), and implicit Euler regularized Newton-PCG solver.

Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
and Informatics Fiduciary duties._

**computational geometry**

- `vbd_step` (function) — Executes one VBD simulation step.

**general**

- `axial` (function) — Smooth tension/compression blend.
- `membrane` (function) — Nonlinear orthotropic membrane energy on a single triangle.
- `bending` (function) — Flat-rest hinge bending energy.
- `eval_cubic_bezier` (function) — Cubic Bézier curve evaluator.
- `flatten_cubic_bezier` (function) — Adaptive subdivision of a cubic Bézier curve within given tolerance in meters.
- `polyline_arclength_point` (function) — Evaluates a point along an arc-length polyline at a normalized fraction in [0, 1].

**physics**

- `newton_step` (function) — Executes one implicit Euler step via regularized Newton-PCG.

_68 undocumented functions — see `modules/crates_kinetics-garment-draping.md`._

### `crates/kinetics-motion-compositor` — 65 functions (23 documented)

_Keyframe structures, temporal ease velocity graphs, and spatial paths._

**general**

- `bez` (function) — 1D cubic Bézier point evaluation: B(u) for u in [0, 1].
- `bez_d` (function) — Derivative of 1D cubic Bézier: B'(u).
- `ease_progress` (function) — Compute temporal progress in [0, 1] for a segment given normalized speeds and influences.
- `eval_spatial_bezier` (function) — Evaluate a 3D cubic Bézier curve at parameter `u` in [0, 1].
- `distance_3d` (function) — Distance between two 3D points.
- `build` (method) — Build an arc-length table for a 3D spatial Bézier segment.
- `u_at_progress` (method) — Map arc-length progress fraction s in [0, 1] to Bézier parameter `u` in [0, 1].
- `eval_scalar_prop` (function) — Evaluate a scalar property at frame `f`.
- `eval_multi_prop` (function) — Evaluate a 2D multi-dimensional property at frame `f`.
- `eval_layer_transform` (function) — Evaluate the transform of a layer at frame `f`.
- `parse_lottie` (function) — Parse a Lottie JSON document string.
- `write_lottie` (function) — Serialize a LottieAnimation structure to JSON string.
- `track_point_ncc` (function) — Match template inside target image within search window using Normalized Cross-Correlation.
- `transform_point` (method) — Transform a 2D point [x, y] by this homography.
- `build` (method) — Construct a Gaussian pyramid with `num_levels` downsampled by 2x each.
- `detect_shi_tomasi_corners` (function) — Detect good features to track using the Shi-Tomasi minimum eigenvalue criterion.
- `track_klt_point` (function) — Track a point from frame I to frame J using pyramidal Lucas-Kanade.
- `evaluate_text_animators` (function) — Evaluate all characters of a string through a set of kinetic text animators.
- `evaluate_unit` (method) — Evaluate the selector weight in [-1, 1] for unit index `i` out of `total_units`.
- `evaluate_unit` (method) — Evaluate wiggly weight for unit `i` at time `t_sec`.
- `smooth_noise_2d` (function) — 2D coherent gradient noise.

**numerics**

- `solve_u` (function) — Solve `bez(x0, x1, x2, x3, u) = x` for parameter `u` in [0, 1] using Newton-Raphson with bisection fallback.
- `solve_corner_pin_homography` (function) — Solve for the 3x3 homography mapping 4 source points to 4 destination points using DLT.

_42 undocumented functions — see `modules/crates_kinetics-motion-compositor.md`._

### `crates/kinetics-schedule-cpm` — 99 functions (2 documented)

_`kinetics-schedule-cpm`

UI-agnostic project schedule and CPM computation engine:
- Unified document model (`model`)
- CPM forward/backward passes, EVM, levelling, Monte Carlo (`schedule`)
- Multi-format import/export & SVG Gantt chart renderer (`format`)
- Content-addressed immutable change DAG and role enforcement (`change_dag`)_

**general**

- `schedule_project` (function) — Run CPM forward and backward pass on `project`, updating every task's schedule fields in place.

**graph**

- `topological_sort` (function) — Topologically sort task indices, returning [`SchedError::CycleDetected`] if any cycle exists.

_97 undocumented functions — see `modules/crates_kinetics-schedule-cpm.md`._

### `crates/layout-box-autolayout` — 35 functions (4 documented)

_`layout-box-autolayout` — UI-agnostic categorical Flexbox and CSS Grid auto-layout engine.

Provides:
- Full Flexbox auto-layout (`LayoutDirection::{Horizontal, Vertical}`)
- CSS Grid auto-layout with fractional `fr` tracks (`LayoutDirection::Grid`, `GridTrack`)
- Sizing modes: `Fixed`, `Hug` (fit content), `Fill` (share free space)
- Free space distribution algorithm (`distribute`) respecting min/max bounds
- Multi-line wrapping with content alignment (`ContentAlign`) and cross alignment (`CrossAlign`)
- Text baseline alignment (`CrossAlign::Baseline`)
- Constraint propagation on frame resize (`HConstraint`, `VConstraint`, `apply_constraints`)
- Top-level `solve_autolayout` orchestrator with nested container support_

**general**

- `apply_constraints` (function) — Adjusts children positions and sizes when their parent changes size from `(ow, oh)` to `(nw, nh)`.
- `distribute` (function) — Distributes free main-axis space among open fill items like CSS flexbox.
- `resolve_tracks` (function) — Resolves track sizes for a list of track specs, given content sizes and available space.

**numerics**

- `solve_autolayout` (function) — Solves auto-layout on a frame with the given available dimensions.

_31 undocumented functions — see `modules/crates_layout-box-autolayout.md`._

### `crates/layout-nodegraph-compiler` — 41 functions (4 documented)

_Pure-Rust typed dataflow node graph compiler._

**general**

- `prune_dead_nodes` (function) — Dead-code elimination:
- `to_base_scale` (method) — Converts standard unit to its base canonical dimension:

**graph**

- `compute_node_hashes` (function) — Compute deterministic hash trees for all nodes in topological order.
- `cse_deduplicate` (function) — Deduplicate identical subexpressions:

_37 undocumented functions — see `modules/crates_layout-nodegraph-compiler.md`._

### `crates/layout-paged-typesetter` — 61 functions (9 documented)

_Citation and bibliography engine._

**2d**

- `paginate` (method) — Paginates paragraphs into pages.

**general**

- `is_east_asian` (function) — Determines if character `c` is East Asian text (CJK ideographs, kana, bopomofo, CJK punctuation).
- `line_break_opportunities` (function) — Computes valid line-break opportunities (byte offsets) in `text` respecting Kinsoku rules.
- `parse` (method) — Parse TeX-style patterns, whitespace-separated (e.g. `hy3ph he2n hena4`).
- `english_standard` (method) — Standard English patterns compiled for common typography.
- `points` (method) — Break points (character indices where a hyphen may be inserted before that char)
- `hyphenate` (method) — Hyphenates a word, inserting hyphen strings.
- `autofit_widths` (function) — Auto-fits column widths to available table width.
- `measure_table_columns` (function) — Measures the (min_width, max_width) for each column in points.

_52 undocumented functions — see `modules/crates_layout-paged-typesetter.md`._

### `crates/layout-presentation-deck` — 63 functions (11 documented)

_Presentation deck model, shape morphing, transition interpolation, and animation timeline.
Grounded in UDHR Articles 19 & 27 and Informatics Fiduciary standards._

**general**

- `parents` (function) — Layout and master placeholders a shape inherits from.
- `resolve_xfrm` (function) — Effective bounding transform of a shape after placeholder resolution.
- `resolve_fill` (function) — Effective fill of a shape after placeholder resolution.
- `resolve_line` (function) — Effective line/border of a shape after placeholder resolution.
- `with_paragraphs` (method) — One DrawingML paragraph per string so U+000A never lives in a single `a:t`.
- `morph_xfrm` (function) — Interpolate a box from `a` to `b` at `t` (0..1):
- `morph_pairs` (function) — Pairs of (shape on `a`, shape on `b`) that Morph animates from one to the other:
- `build_morph_frame` (function) — Compute a morph transition frame between slide `a` and slide `b` at progress `t` (0.0 .. 1.0).
- `point_at` (method) — Evaluate position (fractional x, y) at progress t (0.0 .. 1.0).
- `show_order` (function) — Slide indices visited in order, filtering out hidden slides and applying ranges/custom shows.
- `is_shape_visible` (method) — Check if shape is visible at a given step and progress t.

_52 undocumented functions — see `modules/crates_layout-presentation-deck.md`._

### `crates/locus-geospatial-codecs` — 60 functions (21 documented)

_Geospatial container and exchange file format codecs._

**general**

- `parse_filegdb_geometry` (function) — Decodes an ESRI FileGDB shape binary buffer into standard Geometry.
- `parse_kml` (function) — Parses KML XML text into a collection of Placemarks without external parser crates.
- `serialize_kml` (function) — Serializes placemarks into a valid KML 2.2 XML document string.
- `parse_landxml` (function) — Parses LandXML content into alignments and TIN surfaces without external XML crate dependencies.
- `parse_las_header` (function) — Parses LAS header block from raw file bytes.
- `parse_las_points` (function) — Decodes points from LAS bytes according to the header.
- `parse_mif_mid` (function) — Parses a MapInfo MIF geometry stream and corresponding MID attribute lines.
- `decode_adsb_packet` (function) — Decodes 14-byte (112-bit) raw ADS-B Mode S packet.
- `decode_cpr_global` (function) — Decodes geographic position (latitude, longitude) from an even and an odd CPR frame.
- `evaluate` (method) — Evaluates the expression against an attribute dictionary.
- `match_sql_like` (function) — Simple SQL LIKE pattern matcher (% matches any sequence, _ matches single character).
- `decode_ais_char` (function) — Decodes an AIS 6-bit ASCII character to its 6-bit integer value (0..63).
- `decode_aivdm_sentence` (function) — Decodes an NMEA 0183 `!AIVDM` sentence into an `AisMessage`.
- `decode_zigzag` (function) — Decodes MVT zigzag-encoded integer parameter: `(n >> 1) ^ -(n & 1)`.
- `decode_mvt_geometry` (function) — Decodes an MVT command integer sequence into vector Geometry.
- `parse_pmtiles_header` (function) — Parses PMTiles v3 header from a 127-byte slice.
- `read_varint` (function) — Decodes varint from byte stream.
- `decode_pmtiles_directory` (function) — Decodes an uncompressed PMTiles v3 directory into tile entries.
- `zxy_to_tile_id` (function) — Converts (zoom, x, y) tile coordinates to PMTiles Hilbert tile ID.
- `decode_wkb` (function) — Decodes a geometry from binary WKB bytes.
- `encode_wkb` (function) — Encodes a geometry as ISO WKB little-endian bytes.

_39 undocumented functions — see `modules/crates_locus-geospatial-codecs.md`._

### `crates/locus-parametric-solver` — 70 functions (11 documented)

_`locus-parametric-solver` — Categorical 2D sketch constraint solver and CAD typography.

Provides:
- Weighted minimum-norm damped Gauss-Newton (Levenberg-Marquardt) 2D parametric sketch constraint solver (`solver`)
- Conflict detection and minimal conflicting subset reporting
- Over-constraint / redundancy check via Jacobian rank
- Parametric formula expression evaluator with dependency environment and cycle detection
- Heuristic autoconstrain inference across geometric primitives
- Single-stroke CAM/plotter stick font polyline evaluator (`typography::stroke`)
- AutoCAD MTEXT formatting parser, control code decoder, and word wrap layout engine (`typography::mtext`)_

**general**

- `eval_target` (function) — Evaluates target value for dimensional constraints from environment.
- `check_redundancy` (function) — Check if a candidate constraint is redundant with existing active constraints.
- `jacobian` (function) — Numerical Jacobian over the free parameters (row-major, rows x free.len()).
- `extract` (method) — Extract updated entity from solved parameters.
- `decode_controls` (function) — Decode `%%` control codes:
- `plain_mtext` (function) — Convert MTEXT formatting codes to plain text with `\n` line breaks.
- `layout_mtext` (function) — Layout an MTEXT string into positioned lines and strokes.

**numerics**

- `solve_system` (function) — Solve the constraint system on the given model entities.
- `solve_dense` (function) — Solve the dense system `a y = b` (n x n, row-major) by Gaussian elimination with partial pivoting.
- `rank` (function) — Numerical rank of a row-major `rows x cols` matrix (rows normalised first).
- `solve` (function) — Run the damped Levenberg-Marquardt solver from `x0`. Only `free` parameters change.

_59 undocumented functions — see `modules/crates_locus-parametric-solver.md`._

### `crates/manifold-bim-csg` — 84 functions (11 documented)

_`manifold-bim-csg`

UI-agnostic parametric BIM and solid geometry engine:
- Work-metered, non-recursive 3D BSP CSG booleans (`csg`)
- ISO 10303-21 STEP physical file parser (`step`)
- IFC4 / ISO 16739 architectural entity exporter (`ifc`)
- Parametric multi-layer walls, stairs, and MEP runs (`parametric`)_

**computational geometry**

- `volume` (function) — Signed volume of a closed triangle mesh.
- `area` (function) — Surface area of a triangle mesh.
- `mitre_corner` (function) — Calculates mitred corner intersection between two straight walls meeting at an angle.

**general**

- `subtract` (function) — Subtracts `void` solid from `solid`.
- `union` (function) — Unions two solids together.
- `intersect` (function) — Intersects two solids.
- `step_string` (function) — Formats a string as an ISO 10303-21 STEP string literal with Unicode escapes.
- `ifc_guid` (function) — Generates a deterministic 22-character IFC GlobalId from a 64-bit integer seed.
- `finish` (method) — Finishes the document, wrapping it in standard ISO-10303-21 HEADER and DATA sections.
- `export_minimal_building` (function) — Architectural project exporter creating standard hierarchy: Project -> Site -> Building -> Storey -> Wall.
- `generate_straight_stair` (function) — Generates a straight stair flight with uniform risers and treads.

_73 undocumented functions — see `modules/crates_manifold-bim-csg.md`._

### `crates/media-isobmff` — 255 functions (71 documented)

_Clean-room ISO Base Media File Format (MP4, ISO/IEC 14496-12/14/15) and QuickTime (MOV) support.

- **Demux:** [`open`] parses box headers plus `moov` (and `moof` for fragmented files) from a
[`ByteSource`] and builds per-track [`Sample`] tables; media data is read on demand with
[`Mp4File::read_sample`].
- **Mux:** [`Mp4Writer`] writes progressive MP4/MOV (optionally faststart);
[`FragmentedWriter`] writes fragmented MP4.

See the crate README for supported boxes and limitations._

**general**

- `to_string_lossy` (method) — Lossy string form; `©` (0xA9) is mapped to U+00A9, other non-printables to `?`.
- `full_header` (method) — Full-box header: (version, flags).
- `count` (method) — Read a count and verify that `count * entry_size` bytes remain (allocation guard).
- `parse_box_header` (function) — Parse a box header from `buf` (needs up to 32 bytes; fewer if available).
- `find` (function) — Find the first child of the given type (malformed trailing children are ignored).
- `start` (method) — Begin a box; returns a marker to pass to [`end`](Self::end).
- `leaf` (method) — Append a complete box with the given payload.
- `bt2020` (method) — BT.2020 primaries, D65, with the given peak and black in cd/m².
- `to_bytes` (method) — The 24-byte payload shared by `mdcv` and the SEI message.
- `name` (method) — A short human-readable codec name.
- `new` (method) — Build from parameter sets (NAL units without start codes); profile/level taken from the first SPS.
- `from_asc` (method) — Build from an AudioSpecificConfig, decoding its header fields.
- `channels` (method) — Channel count implied by the channel configuration (0 if unspecified).
- `parse_asc` (function) — Decode (audio object type, sampling frequency, channel configuration) from an AudioSpecificConfig.
- `bytes_per_frame` (method) — Bytes per interleaved audio frame (all channels).
- `format_frame` (method) — Format a frame number as `HH:MM:SS:FF` (`;` before the frames when drop-frame).
- `from_record` (method) — Build from a raw `APVDecoderConfigurationRecord` (starting with `configurationVersion = 1`).
- `parse` (method) — Parse the payload of an `apvC` box (with or without the 4-byte `FullBox` header).
- `to_bytes` (method) — Serialize the full `apvC` box payload (4-byte `FullBox` header + `APVDecoderConfigurationRecord`).
- `parse_sample_entry` (function) — Parse one `stsd` entry. `handler` is the track handler type; `is_qt` selects QuickTime
- `video` (method) — Convenience constructor for a video entry.
- `audio` (method) — Convenience constructor for an audio entry (ISO v0 layout; the muxer upgrades PCM in MOV to v2).
- `avc` (method) — H.264 entry (`avc1`).
- `hevc` (method) — HEVC entry (`hvc1`, parameter sets in the config only).
- `av1` (method) — AV1 entry (`av01`).
- `prores` (method) — ProRes entry; `fourcc` one of `apch`/`apcn`/`apcs`/`apco`/`ap4h`/`ap4x`.
- `dnx` (method) — Avid DNxHD / DNxHR entry (`AVdn`, or `AVdh` for DNxHR).
- `apv` (method) — Advanced Professional Video entry (`apv1` + `apvC`).
- `jpeg` (method) — Motion-JPEG entry (`jpeg`).
- `aac` (method) — AAC entry (`mp4a` + `esds`) from an AudioSpecificConfig.
- `pcm` (method) — PCM entry. The muxer picks the on-disk format (`sowt`/`twos`/`lpcm` for MOV, `ipcm`/`fpcm` for MP4).
- `write` (method) — Serialize as a complete sample entry box. For PCM the format is chosen from the brand.
- `quicktime_mpeg_audio_entries` (function) — QuickTime's `.mp2` entry (what ffmpeg writes for MP2 in MOV) is MPEG audio, as `.mp3` is,
- `codec` (method) — The first sample entry's codec configuration.
- `presentation_pts` (method) — Presentation (edit-adjusted) pts of sample `i`.
- `sample_at_pts` (method) — Index of the sample being presented at raw media time `pts` (the last sample, in
- `sample_at_presentation_time` (method) — Like [`sample_at_pts`](Self::sample_at_pts) but for edit-adjusted presentation time.
- `sync_sample_before` (method) — The closest sync sample at or before `index` (decode order); 0 if there is none.
- `sync_samples` (method) — Indices of all sync samples.
- `title` (method) — Title from `©nam` or the `com.apple.quicktime.title` key.
- `read_sample` (method) — Read the bytes of sample `index` of track `track` (index into [`tracks`](Self::tracks)).
- `track_of_kind` (method) — First track of the given kind.
- `open` (function) — Parse an MP4/MOV file. Only box headers, `ftyp`, `moov` and `moof` boxes are read.
- `validate_self_contained_data` (function) — Narrow local-file profile; no external URLs, aliases, extra entries or duplicate tables.
- `take` (method) — Advance by `n` samples, returning the summed duration.
- `handmade` (function) — Hand-built file: uuid/wide/skip boxes, stz2 sizes, co64, ctts v1 with negative offsets,
- `new` (method) — Start a file: writes `ftyp`, a placeholder box and the `mdat` header.
- `add_track` (method) — Add a track; returns its index (track ids are index + 1).
- `add_timecode_track` (method) — Add a QuickTime timecode track referenced by `video_track`. Its single sample (the start
- `write_sample` (method) — Append one sample to track `track` (decode order). For PCM tracks `data` may hold any whole
- `finish` (method) — Finish the file with `moov` after `mdat`. Returns the underlying writer.
- `finish_faststart` (method) — Finish with `moov` in front of `mdat` ("fast start"): media data is shifted forward in place.
- `add_track` (method) — Add a track (before the first fragment). PCM is not supported in fragments.
- `write_sample` (method) — Buffer a sample for the next fragment.
- `flush_fragment` (method) — Write all buffered samples as one `moof` + `mdat` fragment (no-op if nothing is buffered).
- `finish` (method) — Flush remaining samples and return the writer.
- `len` (method) — Total length in bytes.
- `read_at` (method) — Fill `buf` entirely with bytes starting at `offset`; fail with `UnexpectedEof` if past the end.
- `is_empty` (method) — True if the source holds no bytes.
- `tool` (function) — Find a tool silently (see `media_test_support::oracle`).
- `ffmpeg` (function) — ffmpeg, or `None` after printing `SKIPPED` (a failure with `FILMCRAFT_REQUIRE_ORACLES=1`).
- `ffprobe` (function) — ffprobe, or `None` after printing `SKIPPED` (a failure with `FILMCRAFT_REQUIRE_ORACLES=1`).
- `fixture_specs` (function) — (fixture name, ffmpeg arguments after the global options, output file name derived from the name).
- `fixture` (function) — Path to a fixture, generating it with ffmpeg if needed. `None` if ffmpeg is missing or the
- `ffmpeg_decode_errors` (function) — Decode a file with ffmpeg to the null muxer; returns stderr (empty on success).
- `check_packets` (function) — `strict`: ffprobe timestamps must equal ours + edit offset. Otherwise (edit starting mid-frame,
- `generate_fixtures` (function) — Pre-generate every fixture (`cargo xtask fixtures`).
- `interleave` (function) — All samples of all tracks in decode order interleaved by decode time: (track, sample index).
- `assert_same_packets` (function) — Compare ffprobe's view of two files (per-stream packets: pts, dts, size, key flag).
- `all_empty_samples_roundtrip` (function) — Found by `progressive_roundtrip`: only zero-byte samples must not be written as uniform

**numerics**

- `display_rotation` (method) — Clockwise quarter turns (0–3) the `tkhd` matrix applies for display, or None when it is not

_184 undocumented functions — see `modules/crates_media-isobmff.md`._

### `crates/media-ogg` — 43 functions (12 documented)

_Clean-room Ogg demuxer, written from RFC 3533 (the Ogg encapsulation format), RFC 7845 (Ogg
Opus) and the Vorbis I specification's Ogg mapping (section A).

[`open`] reads every page (capture pattern, header, lacing values, CRC-32 checked; damaged
regions are skipped by searching for the next `OggS`), groups them by logical bitstream and
assembles packets — including packets that continue across pages — as lists of byte ranges,
with the granule position of the page each packet completes on. Codec headers (Opus:
`OpusHead` + `OpusTags`; Vorbis: identification, comment, setup) are collected separately from
the audio packets.

[`OpusTiming`] implements RFC 7845 §4: packet start positions from granule positions and
TOC-derived durations, the pre-skip, and end trimming of the final page, so a player can seek
to any 48 kHz sample.

Layer L0: no dependencies beyond `std`, no `unsafe`, builds for `wasm32-unknown-unknown`._

**general**

- `sniff` (function) — The capture pattern at the start of a file.
- `crc32` (function) — CRC-32 of RFC 3533 §6: polynomial 0x04C11DB7, initial value 0, no reflection, no final XOR.
- `header_count` (method) — Header packets before the audio packets.
- `open` (function) — Read every page and assemble the packets of every logical bitstream.
- `stream_of` (method) — The first stream of `codec`.
- `read_packet` (method) — The bytes of packet `i` of stream `s`.
- `opus_packet_samples` (function) — Duration of an Opus packet in 48 kHz samples from its TOC (RFC 6716 §3.1); `None` if invalid.
- `of` (method) — Timing of the Opus stream `s` (`pre_skip` from its `OpusHead`).
- `open_bounded_opus` (function) — Validate framing, checksums, sequence and timing before invoking the
- `page` (function) — Write one page holding `segments` (lacing already computed by `lace`).
- `pkt` (function) — A CELT 20 ms packet (960 samples): TOC config 31, code 0.
- `opus_file` (function) — Header pages, then pages of 3 packets each; the last page's granule trims `trim` samples.

_31 undocumented functions — see `modules/crates_media-ogg.md`._

### `crates/media-test-support` — 52 functions (28 documented)

_Test-only helpers shared by every FilmCraft crate (used as a dev-dependency only).

- [`oracle`]: find `ffmpeg` / `ffprobe` (env vars, `PATH`, well-known dirs) and skip — or, with
`FILMCRAFT_REQUIRE_ORACLES=1`, fail — when they are missing ([`require_ffmpeg!`] & co.).
- [`fixtures`]: the shared fixture directory (`<workspace>/target/fixtures/<crate>`) and
collision-free temporary names for concurrent generators.
- [`golden`]: RGBA8 images, PNG I/O, image-difference metrics and golden-file checks
(`FILMCRAFT_BLESS=1` regenerates references).

ffmpeg/ffprobe are external test oracles and fixture generators only; they are never linked
or shipped (AGENTS.md §2)._

**general**

- `workspace_root` (function) — The workspace root (the directory holding the top-level `Cargo.toml`).
- `fixtures_root` (function) — Root of all generated fixtures: `$FILMCRAFT_FIXTURES_DIR` or `<workspace>/target/fixtures`.
- `fixtures_dir` (function) — `<fixtures root>/<sub>` (created). `sub` may contain `/` (e.g. `"opus/oracle"`).
- `temp_path` (function) — A temporary sibling of `out` that no other thread or process will pick: the stem gets
- `generate` (function) — Return `out` if it already exists (non-empty); otherwise run `make(tmp)` on a [`temp_path`]
- `report` (function) — Print a `FIXTURE <made|cached|skipped> <name>` line (parsed by `cargo xtask fixtures`).
- `generate_and_report` (function) — Run a fixture generator and [`report`] it: `cached` when every path in `outputs` existed
- `encode_png` (function) — Encode as PNG (RGB when fully opaque, else RGBA), maximum compression.
- `decode_png` (function) — Decode a PNG (8-bit gray/RGB/RGBA, palette expanded) to RGBA8.
- `diff` (function) — Compare two images of equal size.
- `blessing` (function) — Whether `FILMCRAFT_BLESS` asks to regenerate references.
- `failures_dir` (function) — Where failing comparisons write `<name>.actual.png` / `<name>.diff.png`.
- `diff_image` (function) — Difference image: |a − b| per channel ×8, opaque.
- `today` (function) — Today's date (UTC) as YYYY-MM-DD, for attribution sidecars.
- `sidecar_text` (function) — The attribution sidecar text for a generated golden image.
- `attribution_row` (function) — The `ATTRIBUTION.md` table row for a golden image at workspace-relative `rel`.
- `check_golden` (function) — Compare `actual` with the golden PNG at `golden` (or bless it). `title` and `generated_by`
- `assert_golden` (function) — [`check_golden`] that panics on failure.
- `find_tool_in` (function) — Find `name` given an explicit override, a `PATH`-style list and extra directories.
- `find_tool` (function) — Find a tool using the process environment (uncached).
- `ffmpeg` (function) — Path of ffmpeg, if available (cached for the process).
- `ffprobe` (function) — Path of ffprobe, if available (cached for the process).
- `oracles_required` (function) — Whether `FILMCRAFT_REQUIRE_ORACLES` is set to a true value.
- `skip` (function) — Report a skipped test: prints `SKIPPED (<context>): <reason>`. With
- `ffmpeg_or_skip` (function) — ffmpeg, or `None` after reporting a skip (see [`skip`]).
- `ffprobe_or_skip` (function) — ffprobe, or `None` after reporting a skip (see [`skip`]).
- `encode_f32` (function) — Encode planar `f32` channels (equal lengths) as a 32-bit float WAV file.
- `write_f32` (function) — Write planar `f32` channels to `path` as a float WAV.

_24 undocumented functions — see `modules/crates_media-test-support.md`._

### `crates/multimodal-synthesis-schema` — 66 functions (6 documented)

_`multimodal-synthesis-schema`

UI-agnostic multimodal synthesis schemas:
- Comprehensive multimodal model capability catalog (`catalog`)
- Reference media attachments and frame coordination (`references`)
- Generation job feed status and deterministic progress/ETA estimation (`job_feed`)
- 2D/3D composition stages and inpainting mask normalization (`stage`)
- 3D Gaussian Splatting and radiance field cost calculation (`splat_cost`)_

**general**

- `pending_status` (function) — Computes estimated progress percentage (0..=99) and a human-readable remaining time label.
- `time_ago` (function) — Relative human-readable time formatting: "just now", "5 minutes ago", etc.
- `calculate_splat_cost` (function) — Calculate the cost of a 3D Gaussian Splat / radiance field generation.
- `credits_for` (function) — Returns the credit cost for a given model and input type.
- `credits_to_usd_cents` (function) — Converts credits to US dollar cents.
- `normalize_mask_bytes` (function) — Normalizes raw RGBA8 or Grayscale image bytes into a deterministic binary mask (255 = target, 0 = ignore).

_60 undocumented functions — see `modules/crates_multimodal-synthesis-schema.md`._

### `crates/pcm-device-output` — 31 functions (2 documented)

_Native PCM device lifecycle; source, decoding, scheduling and authority retain
their existing QualiaDB/host owners. No application-shell dependency._

**general**

- `prepare` (method) — `live` is called on the native device worker, NEVER inside the realtime
- `render` (method) — All slot storage is retained: no allocation/deallocation or source lookup

_29 undocumented functions — see `modules/crates_pcm-device-output.md`._

### `crates/pixel-buffer-output` — 53 functions (43 documented)

_A tiny library providing a GPU-powered pixel buffer.

[`Pixels`] represents a 2D pixel buffer with an explicit image resolution, making it ideal for
prototyping simple pixel-based games, animations, and emulators. The pixel buffer is rendered
entirely on the GPU, allowing developers to easily incorporate special effects with shaders and
a customizable pipeline.

The GPU interface is offered by [`wgpu`](https://crates.io/crates/wgpu), and is re-exported for
your convenience. Use a windowing framework or context manager of your choice;
[`winit`](https://crates.io/crates/winit) is a good place to start. Any windowing framework that
uses [`raw-window-handle`](https://crates.io/crates/raw-window-handle) will work.

# Environment variables

Pixels will default to selecting the most powerful GPU and most modern graphics API available on
the system, and these choices can be overridden with environment variables. These are the same
vars supported by the [`wgpu` examples](https://github.com/gfx-rs/wgpu/tree/v0.10/wgpu#usage).

* `WGPU_BACKEND`: Select the backend (aka graphics API).
* Supported values: `vulkan`, `metal`, `dx11`, `dx12`, `gl`, `webgpu`
* The default depends on capabilities of the host system, with `vulkan` being preferred on
Linux and Windows, and `metal` preferred on macOS.
* `WGPU_ADAPTER_NAME`: Select an adapter (aka GPU) with substring matching.
* E.g. `1080` will match `NVIDIA GeForce 1080ti`
* `WGPU_POWER_PREF`: Select an adapter (aka GPU) that meets the given power profile.
* Supported values: `low`, `high`
* The default is `low`. I.e. an integrated GPU will be preferred over a discrete GPU.

Note that `WGPU_ADAPTER_NAME` and `WGPU_POWER_PREF` are mutually exclusive and that
`WGPU_ADAPTER_NAME` takes precedence._

**2d**

- `texture_format` (method) — Set the texture format.
- `surface_texture_format` (method) — Set the surface texture format.
- `build_impl` (method) — Create a pixel buffer from the options builder.
- `build` (method) — Create a pixel buffer from the options builder.
- `build_async` (method) — Create a pixel buffer from the options builder without blocking the current thread.
- `new` (method) — Create a pixel buffer instance with default options.
- `new_async` (method) — Asynchronously create a pixel buffer instance with default options.
- `set_scaling_mode` (method) — Set the scaling mode.
- `resize_buffer` (method) — Resize the pixel buffer and zero its contents.
- `resize_surface` (method) — Resize the surface upon which the pixel buffer texture is rendered.
- `render` (method) — Draw this pixel buffer to the configured [`SurfaceTexture`].
- `render_with` (method) — Draw this pixel buffer to the configured [`SurfaceTexture`] using a custom user-provided
- `frame_mut` (method) — Get a mutable byte slice for the pixel buffer. The buffer is _not_ cleared for you; it will
- `frame` (method) — Get an immutable byte slice for the pixel buffer.
- `window_pos_to_pixel` (method) — Calculate the pixel location from a physical location on the window,
- `clamp_pixel_pos` (method) — Clamp a pixel position to the pixel buffer texture size.
- `texture` (method) — Provides access to the internal source [`wgpu::Texture`].
- `render` (method) — Draw the pixel buffer to the render target.
- `clip_rect` (method) — Get the clipping rectangle for the scaling renderer.

**3d**

- `render_texture_format` (method) — Set the render texture format.

**general**

- `request_adapter_options` (method) — Add options for requesting a [`wgpu::Adapter`].
- `device_descriptor` (method) — Add options for requesting a [`wgpu::Device`].
- `wgpu_backend` (method) — Set which backends wgpu will attempt to use.
- `enable_vsync` (method) — Enable or disable Vsync.
- `present_mode` (method) — Optimistically set the `wgpu` present mode.
- `blend_state` (method) — Set the blend state.
- `alpha_mode` (method) — Sets the alpha mode.
- `clear_color` (method) — Set the clear color.
- `check_texture_size` (function) — Compare the given size to the limits defined by `device`.
- `new` (method) — Create a logical texture for a window surface.
- `clear_color` (method) — Change the clear color.
- `adapter` (method) — Returns a reference of the `wgpu` adapter used by the crate.
- `enable_vsync` (method) — Enable or disable Vsync.
- `present_mode` (method) — Get the `wgpu` present mode.
- `set_present_mode` (method) — Set the `wgpu` present mode.
- `reconfigure_surface` (method) — Reconfigure the surface.
- `device` (method) — Provides access to the internal [`wgpu::Device`].
- `queue` (method) — Provides access to the internal [`wgpu::Queue`].
- `context` (method) — Provides access to the internal [`PixelsContext`].
- `surface_texture_format` (method) — Get the surface texture format.
- `render_texture_format` (method) — Get the render texture format.

**physics**

- `new` (method) — Create a builder that can be finalized into a [`Pixels`] pixel buffer.
- `pixel_aspect_ratio` (method) — Set the pixel aspect ratio to simulate non-square pixels.

_10 undocumented functions — see `modules/crates_pixel-buffer-output.md`._

### `crates/pixel-window-render-adapter` — 17 functions (0 documented)

_An AnyRender WindowRenderer for rendering pixel buffers using the pixels crate_

_17 undocumented functions — see `modules/crates_pixel-window-render-adapter.md`._

### `crates/profile-authoring` — 47 functions (2 documented)

_Bounded metric profile authoring. Source, units and project history remain host-owned._

**computational geometry**

- `profile_sweep` (function) — A metric profile swept along a finite 3-D axis. Concave end caps reuse the

**general**

- `replace` (method) — Pure successor, never a second document store or undo journal.

_45 undocumented functions — see `modules/crates_profile-authoring.md`._

### `crates/radiance-camera-raw` — 35 functions (14 documented)

_# Radiance Camera RAW & Computational Sensor Optics

Provides pure-Rust, memory-bounded, zero-unsafe algorithms for:
- Camera sensor CFA demosaicing (Bayer RGGB/BGGR/GRBG/GBRG with Bilinear, AHD, and RCD, plus Fujifilm 6x6 X-Trans)
- Sensor black/white level calibration and dual-illuminant color matrix white balance
- Non-destructive parametric RAW development pipeline (EV, WB, tonal recovery, tone curves, 8-band HSL, split toning, detail, vignette, radial distortion)
- Multi-exposure bracket HDR radiance map fusion with motion deghosting
- Multi-image panorama stitching with multi-band Burt-Adelson Laplacian pyramid blending
- Metadata structures (EXIF, XMP, IPTC), GPX tracklog geotag interpolation with meridian wrapping
- Reversible append-only catalog journal with undo/redo and smart album boolean query evaluation

Execution and provenance guarantees are grounded in international human rights instruments
(UDHR Articles 12, 18, 19, 27; ICCPR Articles 17, 19; ICESCR Article 15) and Informatics Fiduciary duties._

**3d**

- `normalize_raw` (function) — Normalizes 16-bit raw sensor data to scene-linear f32 in [0.0, 1.0]
- `stitch_panorama` (function) — Multi-band Burt-Adelson pyramid blending for multi-image panorama stitching.

**general**

- `color_at` (method) — Returns 0 for Red, 1 for Green, 2 for Blue at sensor coordinate (x, y).
- `demosaic_bilinear` (function) — Bilinear demosaicing for Bayer sensor data.
- `demosaic_ahd` (function) — Adaptive Homogeneity-Directed (AHD) demosaicing for Bayer CFA.
- `demosaic_rcd` (function) — Ratio-based Color Difference (RCD) demosaicing for Bayer CFA.
- `cache_digest` (method) — Computes deterministic 64-bit FNV-1a hash over all parameters.
- `hat_weight` (function) — Hat weighting function: favors midtones, drops to 0 at extremes (over/underexposure).
- `merge_hdr` (function) — Multi-exposure bracket HDR radiance map fusion with motion deghosting.
- `interpolate_position` (method) — Interpolates GPS location at `time` (seconds since epoch) with a maximum gap limit.

**graph**

- `demosaic_xtrans` (function) — Edge-weighted color-difference demosaicing for Fujifilm 6×6 X-Trans CFA patterns.

**numerics**

- `interpolate_dual_illuminant_matrix` (function) — Dual-illuminant color matrix white balance.
- `demosaic_raw` (function) — Demosaics a raw 16-bit sensor buffer into linear RGB f32.
- `develop_image` (function) — Applies non-destructive develop pipeline to linear RGB image data.

_21 undocumented functions — see `modules/crates_radiance-camera-raw.md`._

### `crates/radiance-color-grading` — 175 functions (40 documented)

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

**general**

- `apply` (method) — Forward ASC CDL: `out = clamp((in * slope + offset)^power)`.
- `invert` (method) — Exact inverse of ASC CDL (within clamped range).
- `parse_xml` (method) — Parse ASC CDL XML string (`.cc`, `.ccc`, `.cdl`).
- `apply` (method) — Apply Lift/Gamma/Gain + Offset grading to an RGB triplet.
- `apply` (method) — Tetrahedral 3D interpolation.
- `apply_trilinear` (method) — Trilinear 3D interpolation.
- `parse_cube` (method) — Parse a `.cube` file string.
- `to_cube` (method) — Serialize this 3D LUT to `.cube` format string.
- `parse_cube` (method) — Parse a `.cube` text representation.
- `to_cube` (method) — Serialize to `.cube` format string.
- `kr_kb` (method) — (Kr, Kb) luma coefficients.
- `peak_nits` (method) — The content peak for tone mapping.
- `normalize_y` (function) — Quantized integer sample -> normalized Y float (0..1 nominal).
- `normalize_c` (function) — Quantized integer sample -> normalized Cb/Cr float (-0.5..0.5 nominal).
- `denormalize_y` (function) — Normalized Y float (0..1 nominal) -> integer sample clamped to range.
- `denormalize_c` (function) — Normalized Cb/Cr float (-0.5..0.5 nominal) -> integer sample clamped to range.
- `rgb_to_hsl` (function) — RGB (0..1) -> HSL (h in 0..1).
- `parse_hex` (function) — Parse `#rrggbb` / `#rrggbbaa` into 0..1 floats.
- `primaries` (method) — (x, y) coordinates of R, G, B primaries.
- `to_xyz` (method) — RGB -> CIE XYZ (normalised so white Y = 1).
- `luma` (method) — Luminance weights (Y row of `to_xyz`).

**numerics**

- `encode` (method) — Working linear -> grading signal.
- `decode` (method) — Grading signal -> working linear.
- `encode` (method) — Linear scene reflectance -> log signal.
- `decode` (method) — Log signal -> linear scene reflectance.
- `from_code` (method) — From an ISO/IEC 23091-2 `matrix_coefficients` code.
- `ycbcr_to_rgb` (function) — Convert normalized Y'CbCr (Y in 0..1, Cb/Cr in -0.5..0.5) to non-linear R'G'B'.
- `rgb_to_ycbcr` (function) — Convert non-linear R'G'B' (nominal 0..1) to normalized Y'CbCr.
- `to_linear` (function) — Convert encoded display value to linear light.
- `from_linear` (function) — Convert linear light to encoded display value.
- `srgb_u8_to_linear_table` (function) — 256-entry table: sRGB-encoded u8 -> linear f32.
- `linear_to_srgb_u8_table` (function) — 4096-entry table: linear (0..1 quantised to 12 bits) -> sRGB u8.
- `npm` (function) — Normalised primary matrix (SMPTE RP 177).
- `gamut_matrix` (function) — Linear RGB in `src` -> linear RGB in `dst`.
- `bradford` (function) — Bradford chromatic adaptation transform matrix between two white points.
- `map_luma` (method) — Apply tone mapping to a linear luminance value `y` with target peak `peak_nits`.
- `apply_rgb` (method) — Apply tone mapping to an RGB triplet in linear working units.
- `apply` (method) — Transform an encoded source color triplet to sequence working linear light.
- `apply` (method) — Transform sequence working linear color to target encoded display/export color.

**semantics**

- `luma709` (function) — Rec.709 luma of a linear or encoded RGB triple.

_135 undocumented functions — see `modules/crates_radiance-color-grading.md`._

### `crates/raster-blend-operators` — 62 functions (23 documented)

_Pure channel blend operators, HDRI operators, and W3C/PDF blend modes.

Inputs are expected in the normalized channel range `[0, 1]` or HDR floating-point ranges.
This crate does not own persistent image formats, document state, or display pipelines._

**general**

- `blend_channel` (function) — Evaluates one operator for one backdrop/source channel pair.
- `color_burn_u8` (function) — Byte-quantized Color Burn operator in `0..=255`.
- `color_dodge_u8` (function) — Byte-quantized Color Dodge operator in `0..=255`.
- `vivid_light_u8` (function) — Byte-quantized Vivid Light operator in `0..=255`.
- `hard_mix_u8` (function) — Byte-quantized Hard Mix operator in `0..=255`.
- `blend_operator_rgb` (function) — Evaluates a `BlendOperator` independently for three RGB channels.
- `group` (method) — Index of the separator group (0..=5).
- `is_separable` (method) — Whether the mode works on each colour channel separately (all but Hue, Saturation, Color, Luminosity).
- `parse` (method) — Parse a label or identifier, case/space-insensitively.
- `blend_mode_rgb` (function) — Evaluates a `BlendMode` on straight (not premultiplied) RGB in `[0, 1]`.
- `blend_rgb` (function) — Evaluates an operator or blend mode independently across RGB channels.
- `composite` (function) — Source-over compositing of `source` onto `backdrop` with `mode` (straight RGBA in `0..=1`).
- `cmyk_planes` (function) — Splits a CMYK colour into two additive planes for blending.
- `planes_cmyk` (function) — Combines blended CMY and K planes back into CMYK.
- `pin_light_f32` (function) — HDRI Pin Light: conditional min/max against thresholded source.
- `pegtop_light_f32` (function) — HDRI Pegtop Soft Light approximation.
- `soft_light_f32` (function) — W3C/PDF Soft Light channel blend function.
- `exclusion_f32` (function) — HDRI Exclusion: `backdrop + source - 2.0 * backdrop * source`.
- `difference_f32` (function) — HDRI Difference: `|backdrop - source|`.
- `divide_f32` (function) — HDRI Divide: `backdrop / source` (safeguarded against division by zero).

**numerics**

- `linear_dodge_f32` (function) — HDRI Linear Dodge (Additive): `backdrop + source`.
- `linear_burn_f32` (function) — HDRI Linear Burn: `backdrop + source - 1.0`.
- `linear_light_f32` (function) — HDRI Linear Light: `backdrop + 2.0 * source - 1.0`.

_39 undocumented functions — see `modules/crates_raster-blend-operators.md`._

### `crates/raster-layer-mask-adapter` — 60 functions (19 documented)

_Checked conversion adapter between QualiaDB's layer-mask operations,
digital painting brush dynamics, and selection-morphology pipelines.

Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
and Informatics Fiduciary duties._

**2d**

- `preflight_dimensions` (function) — Validates shape, radius, coverage, and caller-selected limits without allocating algorithm
- `preflight` (function) — Validates shape, radius, coverage, and caller-selected limits without allocating algorithm
- `try_expand` (function) — Checked Euclidean expansion. Caller-selected limits are enforced before mask or EDT buffers
- `preflight` (function) — Validates shape, radius, coverage, and caller-selected limits without allocating algorithm
- `try_expand` (function) — Checked Euclidean expansion. Caller-selected limits are enforced before mask or EDT buffers
- `evaluate_knockout` (function) — Evaluates knockout composition for a single straight-alpha RGBA pixel.
- `evaluate` (method) — Evaluates effective Blend If multiplier for a pixel.
- `mix_dab` (function) — Simulates a single dab of the wet mixer brush over a sampled canvas pixel.

**general**

- `try_contract` (function) — Checked Euclidean contraction, preserving PhotoCraft's inversion/expansion/inversion behavior.
- `try_contract` (function) — Checked Euclidean contraction, preserving PhotoCraft's inversion/expansion/inversion behavior.
- `generate_dabs` (function) — Generates a list of dabs from discrete input points using brush spacing and dynamics.
- `preflight_mask` (function) — Admits a checked edit without converting or allocating the source mask.
- `modify_layer_mask` (function) — Transform the current U8 layer mask and return replacement bytes only after
- `scale_mask_density` (function) — Scales layer mask density in-place or returns scaled copy.
- `feather_mask` (function) — Applies separable 2D Gaussian feathering to an 8-bit mask buffer.
- `propagate_clipping_mask_alpha` (function) — Propagates clipping mask base alpha onto a clipped layer's alpha buffer.
- `weight` (method) — Evaluates visibility weight (0.0..=1.0) for a sample `v` in `0.0..=255.0`.

**graph**

- `try_border` (function) — Checked soft band on both sides of the mask edge, following PhotoCraft's measured source oracle.
- `try_border` (function) — Checked soft band on both sides of the mask edge, following PhotoCraft's measured source oracle.

_41 undocumented functions — see `modules/crates_raster-layer-mask-adapter.md`._

### `crates/raster-selection-morphology` — 46 functions (20 documented)

_Euclidean morphology for caller-owned, row-major selection coverage masks.

Use the checked `try_*` operations to validate finite coverage, dimensions, radius,
and caller-selected work and logical-payload budgets before allocating. The returned
values remain the responsibility of the caller's authoritative editable image state.
This crate owns no document, selection identity, history, canvas-boundary policy,
persistent mask format, or compositor._

**2d**

- `edt` (function) — Euclidean distance to the nearest `true` pixel, in row-major order.
- `old_edt` (function) — Euclidean distance to the nearest `true` pixel.
- `skeletonize_mask` (function) — Topological skeletonization via the Zhang-Suen thinning algorithm.

**general**

- `row_distance` (function) — Lower envelope of parabolas with integer vertical distances. Work is O(row length): each
- `old_dt1` (function) — 1D squared distance transform (Felzenszwalb & Huttenlocher).
- `square` (method) — Square structuring element of width `2 * radius + 1`.
- `disk` (method) — Circular disk structuring element of given floating-point radius.
- `diamond` (method) — Diamond structuring element with Manhattan radius `|dx| + |dy| <= radius`.
- `cross` (method) — Cross structuring element (`dx == 0 || dy == 0`).
- `rectangle` (method) — Flat rectangular structuring element.
- `from_slice` (method) — Custom structuring element from row-major slice.
- `grayscale_dilate` (function) — Grayscale morphological dilation: `out(x, y) = max (input + kernel)`.
- `grayscale_erode` (function) — Grayscale morphological erosion: `out(x, y) = min (input - kernel)`.
- `apply_morphology` (function) — Applies a morphological method to a 2D grayscale raster.
- `distance_transform_f32` (function) — Euclidean distance transform on a thresholded raster mask (`>= 0.5` inside).
- `preflight_dimensions` (function) — Validates shape, radius, and caller-selected limits before any buffer conversion/allocation.
- `preflight` (function) — Validates shape, radius, coverage, and caller-selected limits without allocating algorithm buffers.
- `try_expand` (function) — Checked Euclidean expansion. Caller-selected limits are enforced before mask or EDT buffers
- `try_contract` (function) — Checked Euclidean contraction.

**graph**

- `try_border` (function) — Checked soft band on both sides of the mask edge.

_26 undocumented functions — see `modules/crates_raster-selection-morphology.md`._

### `crates/raster-spatial-operators` — 62 functions (24 documented)

_Pure Rust raster spatial operators.

Provides five high-performance, UI-agnostic spatial image processing engines:
1. `seam_carve`: Avidan-Shamir dynamic-programming energy-map seam carving.
2. `distort`: Affine, 8-DOF Perspective homography, Arc, Polar/DePolar, and Barrel lens distortion.
3. `convolve`: Separable 2D Gaussian blur, unsharp mask, Sobel/Laplacian edge detection, and general 2D kernel convolution.
4. `resample`: Filter-bank image resampling (`MitchellNetravali`, `CatmullRom`, `Lanczos3`, `Kaiser`, `Blackman`, `Bessel`).
5. `octree`: Octree palette reduction and color quantization with Floyd-Steinberg error diffusion._

**2d**

- `gaussian_blur_rgba8` (function) — Separable 2D Gaussian blur for RGBA8 buffer.
- `distort_rgba8` (function) — Distort an RGBA8 buffer.
- `distort_rgba32f` (function) — Distort an RGBA32F buffer.
- `seam_carve_rgba32f` (function) — Carve seams from an RGBA32F buffer (`[f32]`, 4 values per pixel).

**computational geometry**

- `fit_size` (function) — Compute target dimensions to fit aspect ratio inside bounding box.

**general**

- `normalized` (method) — Normalize kernel weights so their sum is 1.0 (if sum is non-zero).
- `gaussian_kernel_1d` (function) — Create a 1D Gaussian kernel with radius and standard deviation sigma.
- `convolve_rgba8` (function) — General 2D convolution for RGBA8 image buffer.
- `convolve_rgba32f` (function) — General 2D convolution for RGBA32F image buffer.
- `gaussian_blur_rgba32f` (function) — Separable 2D Gaussian blur for RGBA32F buffer.
- `unsharp_mask_rgba8` (function) — Unsharp mask sharpening for RGBA8 buffer.
- `map_inverse` (method) — Maps destination coordinate `(dst_x, dst_y)` back to source coordinate `Option<(src_x, src_y)>`.
- `build_palette` (method) — Reduce octree leaves to `max_colors`.
- `octree_quantize_rgba8` (function) — Quantize RGBA8 image to at most `max_colors` using Octree reduction with optional Floyd-Steinberg dithering.
- `octree_quantize_rgba32f` (function) — Quantize RGBA32F image to at most `max_colors` using Octree reduction.
- `support` (method) — Spatial support radius in pixels.
- `weight` (method) — Continuous weighting function at distance `x`.
- `bessel_i0` (function) — Zeroth-order modified Bessel function of the first kind I_0(x).
- `bessel_j1` (function) — First-order Bessel function of the first kind J_1(x).
- `resample_rgba8` (function) — Resample an RGBA8 buffer to new target dimensions.
- `resample_rgba32f` (function) — Resample an RGBA32F buffer to new target dimensions.
- `seam_carve_rgba8` (function) — Carve seams from an RGBA8 buffer to achieve target dimensions.

**graph**

- `sobel_edge_rgba8` (function) — Sobel gradient magnitude edge detection for RGBA8 buffer.
- `laplacian_edge_rgba8` (function) — Laplacian 2D second-derivative edge detector for RGBA8 buffer.

_38 undocumented functions — see `modules/crates_raster-spatial-operators.md`._

### `crates/surface-layered-codecs` — 50 functions (15 documented)

_# Surface Layered Codecs & Multi-Layer Document Exchange

Provides pure-Rust, memory-bounded, zero-unsafe codecs for layered raster document formats:
- Adobe Photoshop PSD (v1) and PSB (v2 64-bit Large Document Format) encoder and decoder
- Channel compression: Apple PackBits RLE and Raw codecs
- Layer records: geometry, clipping masks, blend mode tags (`8BIM`), and Pascal layer names
- Adobe Brush Resource (`.abr`) parser for legacy v1/v2 and modern v6+ sampled brush tips
- Serif Affinity suite container format reader (`.afphoto`, `.afdesign`, `.afpub`)
- GIMP XCF (`.xcf`) and OpenRaster (`.ora`) multi-layer image decoders

All execution and provenance guarantees are grounded in international human rights instruments
(UDHR Articles 12, 18, 19, 27; ICCPR Articles 17, 19; ICESCR Article 15) and Informatics Fiduciary duties._

**general**

- `read_affinity_header` (function) — Reads and validates the 72-byte Affinity container header.
- `parse_affinity_archive` (function) — Parses an Affinity archive, indexing its FAT entries and preview metadata.
- `extract_affinity_preview` (function) — Inspects the embedded preview thumbnail record without full image decode.
- `encode` (function) — Encodes data using PackBits run-length encoding.
- `decode` (function) — Decodes PackBits compressed data until `expected_len` bytes are produced.
- `new` (method) — Creates a blank document with given dimensions and color mode.
- `decode` (method) — Decodes a PSD or PSB document from bytes.
- `encode` (method) — Serializes the document into PSD or PSB binary format with PackBits compression.
- `to_rgba8` (method) — Converts layer channel data to standard 8-bit RGBA pixels.
- `parse_abr` (function) — Parses an Adobe Brush (`.abr`) file from bytes.
- `parse_xcf` (function) — Decodes an XCF image file from bytes.
- `parse_ora` (function) — Decodes an OpenRaster package (.ora) from raw ZIP archive bytes.
- `read_zip_entries` (function) — Minimal pure-Rust ZIP local file record parser for stored (uncompressed) or raw members.
- `parse_stack_xml` (function) — Parses dimensions and layers from `stack.xml`.
- `extract_attribute` (function) — Helper to extract XML attribute value.

_35 undocumented functions — see `modules/crates_surface-layered-codecs.md`._

### `crates/surface-pbr-studio` — 108 functions (17 documented)

_surface-pbr-studio: UI-agnostic categorical PBR surface and radiance studio.

Provides zero-allocation BVH ray acceleration, multi-channel PBR layer compositing,
3D UV texture painting, ray-cast mesh map baking, OpenPBR/MaterialX 1.39 export,
and clean-room brush dynamics.

Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
and Informatics Fiduciary duties._

**2d**

- `apply_dab` (function) — Applies a single dab centered at `(uv.x, uv.y)` with the current brush.
- `apply_eraser_dab` (function) — Erases pixels within a dab area.

**3d**

- `generate_materialx_document` (function) — Generates a complete MaterialX 1.39 XML document connecting maps to a surface shader.
- `compute_normals` (method) — Compute face/area-weighted vertex normals if not provided.

**computational geometry**

- `bake_map` (function) — Bake a mesh map using BVH raycasting.
- `build` (method) — Build a BVH for a mesh. An empty mesh yields an empty tree.
- `intersect` (method) — Finds the closest ray-mesh intersection.
- `ray_triangle` (function) — Möller–Trumbore ray-triangle intersection (hits both front and back faces).
- `polyfill` (function) — Fills a connected UV island or polygon bounding region with constant color.

**general**

- `composite` (method) — Evaluates the layer stack into rendered images for all present channels.
- `standard_presets` (function) — Catalog of standard calibrated PBR materials.
- `create_quad` (method) — Creates a unit quad on the XY plane with UVs in [0, 1].
- `create_cube` (method) — Creates a unit cube centered at origin.
- `paint_stroke` (function) — Paints an interpolated stroke along a path.
- `procedural_round` (method) — Generates a procedural circular tip mask with smooth falloff.
- `parse_brush_properties` (function) — Parses clean-room brush settings from key-value pairs.
- `box_blur` (method) — Fast separable box blur of all channels.

_91 undocumented functions — see `modules/crates_surface-pbr-studio.md`._

### `crates/terrain-brushes` — 104 functions (13 documented)

_Copy-on-Write (`Arc<Vec<f32>>`) tile heightfield snapshot & delta undo/redo stack
with a local 64-byte in-memory StrokeSample journal; no wire format is implied.

Adapted from EarthCraft-main v0.2.0 under MIT License.
Copyright (c) EarthCraft contributors.
Licensed under the MIT License._

**general**

- `load_tile` (method) — Import one already-decoded finite height tile before local history begins.
- `apply_stroke_sample` (method) — Prepare all touched tile successors, then install atomically for this sample.
- `commit_stroke` (method) — Commit preparation refuses without changing preview, redo, IDs or history.
- `cancel_stroke` (method) — Restore the precise original tile Arcs without committing or changing history.
- `discard_local_history` (method) — Deliberately discard local undo/redo history, retaining current tile data and Arcs.
- `payload_memory_bytes` (method) — Conservative tile-reference/sample payload estimate, not distinct allocation or heap bytes.
- `new_with_neighborhood_sampling` (method) — Explicitly select neighbourhood semantics for new work or known-version replay.
- `load_tile` (method) — Import one already-decoded finite height tile before local history begins.
- `apply_stroke_sample` (method) — Prepare all touched tile successors, then install atomically for this sample.
- `commit_stroke` (method) — Commit preparation refuses without changing preview, redo, IDs or history.
- `cancel_stroke` (method) — Restore the precise original tile Arcs without committing or changing history.
- `discard_local_history` (method) — Deliberately discard local undo/redo history, retaining current tile data and Arcs.
- `payload_memory_bytes` (method) — Conservative tile-reference/sample payload estimate, not distinct allocation or heap bytes.

_91 undocumented functions — see `modules/crates_terrain-brushes.md`._

### `crates/terrain-erosion-solvers` — 50 functions (18 documented)

_Deterministic geomorphological heightfield erosion and surface modeling solvers.

Provides pure mathematical 2D heightfield solvers for physical terrain processes:
- Multi-scale thermal slope relaxation and scree synthesis (`thermal_shaper`)
- Particle-based hydraulic droplet erosion and sediment transport (`hydraulic_droplet`)
- Hydrological priority-flood depression conditioning and D8 flow routing (`drainage_pyramid`)
- Elevation-dependent snowfall, freeze/thaw, and avalanche mass transport (`snow_accumulation`)
- Hydrostatic water-table lake filling and coastal shoreline smoothing (`lacustrine_fill`)
- Planetary impact crater cavity and ejecta rim lip morphology (`crater_morphology`)
- Stratigraphic stepped terrace rock layer modulation (`fractal_terrace`)

Grounded in international human rights instruments (UDHR Article 27; ICESCR Article 15)
and Informatics Fiduciary duties ensuring scientific rigor, deterministic simulation,
and moral attribution in environmental modeling._

**economics**

- `fill_lakes` (function) — Fill lakes, depressions, and seas up to hydrostatic equilibrium across a 2D heightfield.

**general**

- `crater_profile` (function) — Profile calculation for relative crater elevation offset at `normalized_dist` (0 = center, 1 = outer rim lip).
- `apply_crater` (function) — Apply a single impact crater cavity and raised rim onto a 2D heightfield.
- `generate_craterfield` (function) — Generate a multi-crater planetary field using deterministic cell distribution.
- `priority_flood_fill` (function) — Condition a heightfield by filling all internal depression sinks to their spillway elevation.
- `d8_flow_directions` (function) — Compute D8 steepest descent flow directions for each cell.
- `flow_accumulation` (function) — Compute upstream flow accumulation area for every cell from D8 directions.
- `carve_channels` (function) — Carve fluvial river channels into a conditioned heightfield along high-accumulation paths.
- `drainage_network` (function) — Execute full drainage analysis pipeline: priority-flood conditioning, D8 routing, accumulation, and channel carving.
- `build_perm_table` (function) — PRNG table generator for strata hash lookup.
- `evaluate_stratum` (function) — Compute single-elevation terraced profile and strata color index.
- `apply_terraces` (function) — Apply stratigraphic terracing to an entire 2D heightfield in-place.
- `lcg` (function) — LCG step: deterministically advances generator state.
- `thermal_shape` (function) — Apply multi-scale thermal shaping to relax steep slopes across a resolution pyramid.
- `thermal_shape_with` (function) — Apply multi-scale thermal shaping with custom world scale configuration.

**numerics**

- `lcg` (function) — Linear congruential generator step.

**physics**

- `hydraulic_erosion` (function) — Simulate hydraulic erosion over a mutable 2D heightfield.
- `simulate_snow` (function) — Simulate snow precipitation, elevation-based freezing, melting, and avalanche transport.

_32 undocumented functions — see `modules/crates_terrain-erosion-solvers.md`._

### `crates/timeline-editing` — 43 functions (16 documented)

_Application-neutral editorial time and transition editing.

The selected chronicle media operations are adapted to a small public surface with
no application model, codec, renderer, file, GPU, or runtime dependencies.
`Tick` uses the chronicle media exact media scale. Simulation time remains owned by
simulation systems; this crate describes editorial positions and frame
boundaries._

**general**

- `from_rational` (method) — Convert a rational timestamp `pts * num / den` seconds. A zero timebase
- `to_rational_floor` (method) — Inverse rational conversion rounded down in the requested timebase.
- `from_units` (method) — Convert a count of units at `per_second`, such as audio samples,
- `to_units_floor` (method) — Convert to units at `per_second`, rounded down (unlike `from_units`,
- `mul_ratio` (method) — Floor-scale by a rational factor. A zero divisor maps to zero, matching
- `sane` (method) — Invalid or non-positive rates use the source's documented default.
- `frame_at` (method) — Index of the frame containing `time`, rounded down.
- `tick_of` (method) — Start tick for frame `frame`, rounded up so the tick belongs to that frame.
- `snap` (method) — Snap down to a frame boundary.
- `frame_index` (function) — Keep the mathematical frame index wide until the public API needs an i64.
- `frame_boundary` (function) — Ceiling division for a frame boundary, matching the chronicle media exact rational
- `transition_bounds` (function) — Return the allowed interval and cut for the referenced clips.
- `transition_alignment` (function) — Infer the alignment stored or shown for a span over the supplied cut.
- `set_transition_span` (function) — Validate an edit before a host mutates its sequence/session. On refusal the
- `start_for_alignment` (function) — Compute a new start when a user changes the typed duration/alignment. The

**graph**

- `drag_transition_span` (function) — Compute a drag preview without changing a document. The host commits the

_27 undocumented functions — see `modules/crates_timeline-editing.md`._

### `crates/vector-compute-renderer` — 161 functions (77 documented)

_Vello is a compute-centric 2D graphics rendering engine written in Rust,
using [`wgpu`].
It efficiently draws large 2D scenes with interactive or near-interactive
performance.

![image](https://github.com/linebender/vello/assets/8573618/cc2b742e-2135-4b70-8051-c49aeddb5d19)


## Motivation

Vello is meant to fill the same place in the graphics stack as other vector graphics renderers like [Skia](https://skia.org/), [Cairo](https://www.cairographics.org/), and its predecessor project [Piet](https://www.cairographics.org/).
On a basic level, that means it provides tools to render shapes, images, gradients, texts, etc, using a PostScript-inspired API, the same that powers SVG files and [the browser `<canvas>` element](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D).

Vello's selling point is that it gets better performance than other renderers by better leveraging the GPU.
In traditional PostScript-style renderers, some steps of the render process like sorting and clipping either need to be handled in the CPU or done through the use of intermediary textures.
Vello avoids this by using prefix-scan algorithms to parallelize work that usually needs to happen in sequence, so that work can be offloaded to the GPU with minimal use of temporary buffers.

This means that Vello needs a GPU with support for compute shaders to run.


## Getting started

Vello is meant to be integrated deep in UI render stacks.
While drawing in a Vello [`Scene`] is easy, actually rendering that scene to a surface setting up a wgpu context, which is a non-trivial task.

To use Vello as the renderer for your PDF reader / GUI toolkit / etc, your code will have to look roughly like this:

```ignore
// Initialize wgpu and get handles
let (width, height) = ...;
let device: wgpu::Device = ...;
let queue: wgpu::Queue = ...;
let mut renderer = Renderer::new(
&device,
RendererOptions {
use_cpu: false,
antialiasing_support: vello::AaSupport::all(),
num_init_threads: NonZeroUsize::new(1),
},
).expect("Failed to create renderer");

// Create scene and draw stuff in it
let mut scene = vello::Scene::new();
scene.fill(
vello::peniko::Fill::NonZero,
vello::Affine::IDENTITY,
vello::Color::from_rgb8(242, 140, 168),
None,
&vello::Circle::new((420.0, 200.0), 120.0),
);

// Draw more stuff
scene.push_layer(...);
scene.fill(...);
scene.stroke(...);
scene.pop_layer(...);

let texture = device.create_texture(&...);
// Render to a wgpu Texture
renderer
.render_to_texture(
&device,
&queue,
&scene,
&texture,
&vello::RenderParams {
base_color: palette::css::BLACK, // Background color
width,
height,
antialiasing_method: AaConfig::Msaa16,
},
)
.expect("Failed to render to a texture");
// Do things with surface texture, such as blitting it to the Surface using
// wgpu::util::TextureBlitter.
```

See the [`research/examples/`](https://github.com/linebender/vello/tree/main/research/examples) folder to see how that code integrates with frameworks like winit.
Utilities which can be used to create an alternative Vello renderer to [`Renderer`][crate::Renderer].

These APIs have not been carefully designed, and might not be powerful enough for this use case._

**2d**

- `draw_blurred_rounded_rect` (method) — Draw a rounded rectangle blurred with a gaussian filter.
- `draw_blurred_rounded_rect_in` (method) — Draw a rounded rectangle blurred with a gaussian filter in `shape`.

**3d**

- `all` (method) — Support every anti-aliasing method.
- `dispatch_indirect` (method) — Do an indirect dispatch.
- `create_targets` (function) — Vello uses a compute shader to render to the provided texture, which means that it can't bind the surface
- `add_compute_shader` (method) — Add a shader.

**economics**

- `register_texture` (method) — Register a [`wgpu::Texture`] with Vello, to allow drawing GPU-resident data.

**general**

- `from_bits` (method) — Construct a `DebugLayers` from the raw bits.
- `bits` (method) — Get the raw representation of this value.
- `none` (method) — A `DebugLayers` with no layers enabled.
- `all` (method) — A `DebugLayers` with all layers enabled.
- `is_empty` (method) — True if this `DebugLayers` has no layers enabled.
- `contains` (method) — Determine whether `self` is a superset of `mask`.
- `toggle` (method) — Toggle the value of the layers specified in mask.
- `area_only` (method) — Support only [`AaConfig::Area`].
- `new` (method) — Creates a new renderer for the specified device.
- `render_to_texture` (method) — Renders a scene to the target texture.
- `override_image` (method) — Overwrite `image` with `texture`.
- `mark_override_image_dirty` (method) — Marks `image` as dirty in the atlas cache, so its override texture will be recopied on
- `unregister_texture` (method) — Unregister a [`wgpu::Texture`] that was registered with [`register_texture`](Self::register_texture).
- `reload_shaders` (method) — Reload the shaders. This should only be used during `vello` development
- `push` (method) — Appends a [`Command`] to the back of the [`Recording`].
- `upload` (method) — Commands to upload the given data to a new buffer with the given name.
- `upload_uniform` (method) — Commands to upload the given data to a new buffer as a uniform with the given name.
- `upload_image` (method) — Commands to upload the given data to a new image with the given dimensions and format.
- `draw` (method) — Issue a draw call
- `download` (method) — Prepare a buffer for downloading.
- `clear_all` (method) — Commands to clear the whole buffer.
- `free_buffer` (method) — Commands to free the given buffer.
- `free_image` (method) — Commands to free the given image.
- `free_resource` (method) — Commands to free the given resource.
- `into_commands` (method) — Returns a [`Vec`] containing all the [`Command`]s in order.
- `render_encoding_full` (function) — Create a single recording with both coarse and fine render stages.
- `render_encoding_coarse` (method) — Prepare a recording for the coarse rasterization phase.
- `record_fine` (method) — Run fine rasterization assuming the coarse phase succeeded.
- `out_image` (method) — Get the output image.
- `new` (method) — Creates a new scene.
- `reset` (method) — Removes all content from the scene.
- `bump_estimate` (method) — Tally up the bump allocator estimate for the current state of the encoding,
- `encoding` (method) — Returns the underlying raw encoding.
- `encoding_mut` (method) — Returns a mutable reference to the underlying raw encoding.
- `push_layer` (method) — Pushes a new layer clipped by the specified shape and composed with
- `push_luminance_mask_layer` (method) — Pushes a new layer clipped by the specified shape and treated like a luminance
- `push_clip_layer` (method) — Pushes a new layer clipped by the specified `clip` shape.
- `push_layer_inner` (method) — Helper for logic shared between [`Self::push_layer`] and [`Self::push_luminance_mask_layer`]
- `pop_layer` (method) — Pops the current layer.
- `fill` (method) — Fills a shape using the specified style and brush.
- `stroke` (method) — Strokes a shape using the specified style and brush.
- `stroke_gpu_inner` (method) — Encodes the stroke of a shape using the specified style. The stroke style must have
- `draw_image` (method) — Draws an image at its natural size with the given transform.
- `draw_glyphs` (method) — Returns a builder for encoding a glyph run.
- `append` (method) — Appends a child scene.
- `new` (method) — Creates a new builder for encoding a glyph run for the specified
- `brush_transform` (method) — Sets the transform applied to the brush contents.
- `font_size` (method) — Sets the font size in pixels per em units.
- `font_embolden` (method) — Sets synthetic embolden settings.
- `hint` (method) — Sets whether to enable hinting.
- `normalized_coords` (method) — Sets the normalized design space coordinates for a variable font instance.
- `brush` (method) — Sets the brush.
- `brush_alpha` (method) — Sets an additional alpha multiplier for the brush.
- `draw` (method) — Encodes a fill or stroke for the given sequence of glyphs and consumes the builder.
- `create_surface` (method) — Creates a new surface for the specified window and dimensions.
- `create_render_surface` (method) — Creates a new render surface for the specified window and dimensions.
- `resize_surface` (method) — Resizes the surface to the new dimensions.
- `device` (method) — Finds or creates a compatible device handle id.
- `new_device` (method) — Creates a compatible device handle id.
- `adapter` (method) — Returns the adapter associated with the device.
- `block_on_wgpu` (function) — Block on a future, polling the device as needed.
- `use_parallel_initialisation` (method) — Enable creating any remaining shaders in parallel
- `build_shaders_if_needed` (method) — Initialise (in parallel) any shaders which are yet to be created
- `get_gpu_buf` (method) — Get a buffer, only if it's on GPU.
- `get_cpu_buf` (method) — Get a CPU buffer.
- `get_buf` (method) — Get a buffer from the pool or create one.
- `size_class` (method) — Quantize a size up to the nearest size class.
- `new` (method) — Create new transient bind map, seeded from external resources

**numerics**

- `transform` (method) — Sets the global transform. This is applied to all glyphs after the offset

**physics**

- `glyph_transform` (method) — Sets the per-glyph transform. This is applied to all glyphs prior to

_84 undocumented functions — see `modules/crates_vector-compute-renderer.md`._

### `crates/vector-compute-shaders` — 259 functions (7 documented)

_This is a utility library to help integrate the [Vello] shader modules into any renderer
project. It provides the necessary metadata to construct the individual compute pipelines
on any GPU API while leaving the responsibility of all API interactions
(such as resource management and command encoding) up to the client.

The shaders can be pre-compiled to any target shading language at build time based on
feature flags. Currently only WGSL and Metal Shading Language are supported.

Your first choice should be to use the build time generated [`SHADERS`].

Alternatively you could use runtime compiled shaders via the [`compile`] module
by enabling the `compile` feature. This option is useful for active development
as it enables you to do hot reloading of shaders.

[Vello]: https://github.com/linebender/vello_

**2d**

- `eval` (method) — Evaluate the curve at the given parameter.
- `eval_cubic_and_deriv` (function) — Evaluate both the point and derivative of a cubic bezier.

**3d**

- `from_default` (method) — Same as [`ShaderInfo::from_dir`] but uses the default shader directory provided by [`shader_dir`].

**general**

- `cubic_from_points_derivs` (function) — Compute cubic parameters from endpoints and derivatives.
- `read_draw_tag_from_scene` (function) — Read draw tag, guarded by number of draw objects.

**physics**

- `from_points_derivs` (method) — Compute parameters from endpoints and derivatives.
- `integ_euler_10` (function) — Integrate Euler spiral.

_252 undocumented functions — see `modules/crates_vector-compute-shaders.md`._

### `crates/vector-cpu-render-adapter` — 45 functions (8 documented)

_A [`vello_cpu`] backend for the [`anyrender`] 2D drawing abstraction_

**general**

- `get_or_register` (method) — Look up the converted image for `image`, converting and registering it
- `maintain` (method) — Advance the frame counter and evict stale entries.
- `clear` (method) — Drop all cached conversions and their registry entries.
- `convert_image` (function) — Convert a [`peniko::ImageData`] to a premultiplied RGBA8 [`Pixmap`].
- `with_image_cache_config` (method) — Create a renderer with a custom image cache configuration.
- `clear_image_cache` (method) — Drop all cached image conversions.
- `maintain` (method) — Advance the image cache's frame counter and evict stale entries.
- `clear_image_cache` (method) — Drop all cached image conversions.

_37 undocumented functions — see `modules/crates_vector-cpu-render-adapter.md`._

### `crates/vector-glyph-shaping` — 49 functions (16 documented)

_# Vector Glyph Shaping & Typography Engine

A pure Rust, UI-agnostic categorical typography domain crate providing OpenType
feature selection, UAX #9 Bidirectional algorithm run reordering, Knuth-Plass
optimal total-fit paragraph composition, syllable hyphenation, area text /
threaded frame / text-on-path layout, and TrueType/CFF font subsetting.

Grounded in international human rights guarantees of linguistic diversity,
artistic expression, and cultural heritage (UDHR Articles 19 & 27; ICESCR Article 15)
and Informatics Fiduciary duties._

**2d**

- `layout_area_text` (function) — Lay out simple area text into a bounded rectangle.

**general**

- `bidi_class` (function) — Character classification according to Unicode bidirectional rules.
- `resolve_bidi_runs` (function) — Resolve paragraph direction and decompose text into directional runs.
- `reorder_visual` (function) — Reorder run indices from logical reading order into visual left-to-right display order.
- `compose` (function) — Knuth-Plass line composition over glyph slice `g`.
- `from_tags` (method) — Parse a feature list like `["dlig", "smcp", "-liga"]`.
- `to_tags` (method) — List of tag strings that differ from default settings.
- `known_tag` (method) — Check if a tag name is recognized.
- `should_keep_ligatures` (method) — Determine if standard ligatures should be kept given tracking value.
- `hyphen_points` (function) — Allowed hyphenation points in `word` as char indices (hyphen inserted before char).
- `layout_text_on_path` (function) — Lay out characters along a sequence of 2D curve points.
- `calc_table_checksum` (function) — Calculate OpenType table checksum (sum of big-endian u32 words).
- `parse_sfnt_directory` (function) — Parse the SFNT table directory from raw font bytes.
- `build_sfnt_font` (function) — Assemble a valid standalone SFNT font binary from a set of table records.
- `story` (function) — Join all frame slices back into the full story text.
- `distribute` (function) — Distribute a sequence of story runs across linked text frames in thread order.

_33 undocumented functions — see `modules/crates_vector-glyph-shaping.md`._

### `crates/vector-gpu-render-adapter` — 44 functions (2 documented)

_A [`vello`] backend for the [`anyrender`] 2D drawing abstraction_

**general**

- `with_pipeline_cache` (method) — Like [`ImageRenderer::new`], but reusing compiled pipelines from a cache
- `spawn_init` (function) — Drive the wgpu init future. On wasm32 we spawn it onto the JS microtask

_42 undocumented functions — see `modules/crates_vector-gpu-render-adapter.md`._

### `crates/vector-hybrid-render-adapter` — 57 functions (1 documented)

_A [`vello_gpu`] backend for the [`anyrender`] 2D drawing abstraction_

**general**

- `spawn_init` (function) — Drive the wgpu init future. On wasm32 we spawn it onto the JS microtask

_56 undocumented functions — see `modules/crates_vector-hybrid-render-adapter.md`._

### `crates/vector-hybrid-renderer` — 802 functions (209 documented)

_A GPU renderer for 2D vector graphics with CPU-side preprocessing.

Vello GPU uses the Sparse Strips architecture to divide work between CPU
preprocessing and GPU rendering:

- The CPU processes paths, tiles, and schedules sparse strips.
- The GPU rasterizes those strips and performs compositing.
- The compact strip representation limits data transfer between them.

## Key Features

- Efficient path rendering with CPU-side processing
- GPU-accelerated compositing and blending
- Support for both windowed and headless rendering

## Feature Flags

- `wgpu` (enabled by default): Enables the GPU rendering backend via wgpu and
includes the required Vello GPU shaders.
- `wgpu_default` (enabled by default): Enables wgpu with its default hardware
backends (such as Vulkan, Metal, and DX12).
- `text` (enabled by default): Enables glyph rendering ([`Scene::glyph_run`]).
- `webgl`: Enables the WebGL rendering backend for browser support, using GLSL
shaders for compatibility.

If you need to customize the set of enabled wgpu features, disable this crate's
default features then enable its `wgpu` feature. You can then depend on wgpu
directly, setting the specific features you require. Don't forget to also
disable wgpu's default features.

## Architecture

The renderer is split into several key components:

- `Scene`: Manages the render context and path processing on the CPU
- `Renderer` or `WebGlRenderer`: Handles GPU resource management and executes
draw operations

See the individual module documentation for more details on usage and
implementation.

## Current state

Vello GPU is a solid GPU-accelerated 2D renderer with broad, reliable feature
support. Although it does not match the compute-centric `vello` renderer's raw
performance on dynamic and vector-heavy workloads, it provides excellent
performance on workloads that benefit from GPU acceleration, such as images,
gradients, and filters. Overall, we still consider it to be slightly less mature
than its CPU-only counterpart Vello CPU.

Vello GPU remains under active development. Known limitations include:

- The following features are not yet supported and will panic: Mask layers,
complex filter graphs as well as certain blend modes for non-isolated
blending.
- Parts of the API and its documentation are still suboptimal, for example the
lifecycle and ownership of external resources through [`Resources`][].
- Some exposed features remain experimental and are not recommended for use,
including glyph caching. Experimental APIs are identified in their method
documentation.
- Parts of the rendering pipeline are not yet fully optimized, particularly the
wgpu backend, but also other aspects.
- Some failures panic instead of being reported through a user-facing error.

With that said, we are continuously improving Vello GPU and will address these
and other limitations in future releases.

## Package rename

This package was previously named `vello_hybrid`. New dependencies and Rust
imports should use `vello_gpu`; no compatibility re-export package is provided
in this repository. Existing crates.io releases under `vello_hybrid` remain
available, and publication under the new name may lag behind the repository
rename._

**2d**

- `new_with` (method) — Creates a new WebGL2 renderer and its persistent resources with specific settings.
- `begin_with` (method) — Begins creating a WebGL2 renderer with specific settings.
- `write_to_atlas` (method) — Write pixel data to an existing atlas allocation.
- `probe` (method) — Conduct a probing operation.
- `write_to_atlas` (method) — Write pixel data to an existing atlas allocation.
- `paint_transform_has_area` (method) — Whether the effective paint transform maps paint space onto a non-zero area.
- `push_clip_rect` (method) — Push a rectangular clip path.
- `set_aliasing_threshold` (method) — Set the aliasing threshold.
- `fill_rect` (method) — Fill a rectangle with the current paint and fill rule.
- `stroke_rect` (method) — Stroke a rectangle with the current paint and stroke settings.
- `fill_blurred_rounded_rect` (method) — Fill a blurred rectangle with the given corner radius and standard deviation.
- `texture_rect` (method) — Translate a scene-space rectangle within this layer to texture coordinates.

**3d**

- `new` (method) — Creates a new WebGL2 renderer and its persistent resources.
- `begin` (method) — Begins creating a WebGL2 renderer and its persistent resources.
- `try_finish` (method) — Polls shader compilation and program linking without blocking when
- `finish` (method) — Blocks until shader compilation and program linking finish.
- `draw_strip_range` (method) — Draw `count` strip instances starting at `first_instance`.
- `set_strip_attrib_offset` (method) — Point the strip vertex attributes at the instance at `first_instance`.
- `vertex_attributes` (method) — Vertex attributes for the strip

**general**

- `main` (function) — Main entry point for the headless rendering example.
- `reverse` (method) — Reverse opaque strips for front-to-back rendering.
- `get_or_insert` (method) — Return the existing slot for `texture_source`, or insert it into the first empty slot.
- `n_decimations` (method) — Returns the number of decimation levels encoded in the header.
- `apply_pass_dimensions` (method) — Compute and update source and destination sizes based on the pass kind.
- `emit` (method) — Emit one pass, reading from the current region and writing to the other texture parity.
- `emit_blur_sequence` (method) — Apply the sequences of passes that is needed to create a full Gaussian blur with
- `required_filter_data_height` (method) — Calculate the required height for the filter data texture.
- `new` (method) — Create a new gradient ramp cache with the specified retained count.
- `maintain` (method) — Maintain the gradient cache by evicting old entries.
- `luts_size` (method) — Get the size of the packed luts.
- `is_empty` (method) — Check if the packed luts is empty.
- `has_changed` (method) — Check if the luts data has changed.
- `mark_synced` (method) — Mark the luts as synced.
- `take_luts` (method) — Take ownership of the luts, leaving an empty vector in its place.
- `restore_luts` (method) — Restore the luts. The restored luts should have the same logical content as the original.
- `evict` (method) — Remove multiple LRU entries and compact the LUTs vector.
- `remove_lru_entries` (method) — Identify and remove the LRU cache entries.
- `compact_luts` (method) — Remove LUT data for evicted entries with compacting the LUTs vector, and update remaining offsets.
- `with_image_cache` (method) — Add the image cache needed to resolve internal image paints.
- `as_bytes` (method) — Returns the byte representation of this paint.
- `size_texels` (method) — Number of `RGBA32Uint` texels occupied by this paint.
- `serialize_to_buffer` (method) — Serialize paint enums directly into the provided buffer. Returns the number of bytes written.
- `pack_texture_width_and_extend_mode` (function) — Pack `extend_mode` and `texture_width` into a single u32.
- `pack_radial_kind_and_swapped` (function) — Pack radial gradient `kind` and `f_is_swapped` into a single u32.
- `pack_image_size` (function) — Pack image `width` and `height` into a single u32.
- `pack_image_offset` (function) — Pack image offset coordinates `x` and `y` into a single u32.
- `pack_image_params` (function) — Pack image `quality` and extend modes into a single u32.
- `pack_tint` (function) — Pack an optional [`Tint`](vello_common::paint::Tint) into a (`tint_color_u32`, `tint_mode_u32`) pair for the GPU.
- `map_js_error` (method) — Attach the WebGL operation that produced this JavaScript result.
- `get_max_texture_dimension_2d` (function) — Query the WebGL context for the max texture size.
- `new` (method) — Create an empty binding map.
- `get` (method) — Get a texture binding.
- `remove` (method) — Remove a texture binding.
- `render` (method) — Render `scene` using WebGL2
- `render_to_atlas` (method) — Render a `scene` directly into an atlas layer.
- `render_scene` (method) — Shared render pipeline: prepares GPU resources, runs the scheduler, and
- `gl_context` (method) — Get a reference to the underlying WebGL context.
- `upload_image` (method) — Upload image to cache and atlas in one step. Returns the `ImageId`.
- `destroy_image` (method) — Destroy an image from the cache and clear the allocated slot in the atlas.
- `atlas_texture` (method) — Returns an individual image atlas texture.
- `atlas_info` (method) — Returns the current allocation state of the image/glyph atlas texture.
- `clear_atlas_region` (method) — Clear a specific region of an atlas texture.
- `offscreen` (method) — If `use_depth_buffer` is true, the framebuffer must have a valid depth attachment!
- `new` (method) — Starts compiling all programs before performing any blocking status queries.
- `prepare` (method) — Prepare resources for rendering.
- `maybe_create_atlas_textures` (method) — Create any newly allocated atlas textures.
- `maybe_resize_filter_data_tex` (method) — Grow the filter data texture if needed.
- `upload_filter_data_texture` (method) — Upload filter data to the texture.
- `maybe_resize_alphas_tex` (method) — Grow the alpha texture if needed.
- `maybe_resize_encoded_paints_tex` (method) — Grow the encoded paints texture if needed.
- `maybe_resize_gradient_tex` (method) — Grow the gradient texture if needed.
- `maybe_update_config_buffer` (method) — Update config buffer if dimensions changed.
- `upload_alpha_texture` (method) — Upload alpha data to the texture.
- `upload_encoded_paints_texture` (method) — Upload encoded paints to the texture.
- `upload_gradient_texture` (method) — Upload gradient data to the texture.
- `upload_strip_pair` (method) — Uploads two strip slices (opaque then alpha) into a single GPU buffer.
- `get_strip_uniforms` (function) — Get the  uniform locations for the `render_strips` program.
- `create_texture_storage` (function) — Create a texture backed by immutable storage (`texStorage2D`).
- `validate_resource_texture_size` (function) — Validate that a resource texture fits within its configured dimension limit.
- `create_data_texture_storage` (function) — Create a zero-initialized data texture backed by immutable storage (`texStorage2D`).
- `create_placeholder_rgba32ui_texture` (function) — Create a 1x1 RGBA32UI placeholder texture.
- `create_placeholder_rgba8_texture` (function) — Create a 1x1 RGBA8 placeholder texture.
- `create_webgl_resources` (function) — Create all WebGL resources needed for rendering.
- `create_atlas_texture` (function) — Create an image atlas texture.
- `create_framebuffer_for_texture` (function) — Create a framebuffer for a texture.
- `initialize_strip_vao` (function) — Initialize strip VAO.
- `draw_instanced_quads` (method) — Draw `instance_count` quad instances using the currently bound VAO.
- `draw_strips` (method) — Draw `count` strip instances starting at `first_instance`, split into one draw per
- `bind_external_textures` (method) — Bind the external texture slots.
- `strip_pass_inner` (method) — Render strips to the specified render target.
- `width` (method) — Get the width of the image.
- `height` (method) — Get the height of the image.
- `write_to_atlas` (method) — Write image data to an atlas texture at the specified offset.
- `copy_to_texture` (function) — Copy from a source texture/framebuffer to an atlas texture.
- `upload_rgba32ui_rows` (function) — Upload full rows of RGBA32UI data into immutable texture storage.
- `try_finish` (method) — Try to finish the probe.
- `new` (method) — Create an empty binding map.
- `insert` (method) — Insert or replace a texture binding.
- `get` (method) — Get a texture binding.
- `remove` (method) — Remove a texture binding.
- `new` (method) — Creates a new renderer and its persistent resources.
- `new_with` (method) — Creates a new renderer and its persistent resources with specific settings.
- `create_depth_texture_view` (method) — Creates a depth texture view compatible with [`render`](Self::render).
- `render` (method) — Render `scene`.
- `render_to_atlas` (method) — Render a `scene` directly into an atlas layer.
- `render_scene` (method) — Shared render pipeline: prepares GPU resources, runs the scheduler against
- `upload_image` (method) — Upload image to cache and atlas in one step. Returns the `ImageId`.
- `destroy_image` (method) — Destroy an image from the cache and clear the allocated slot in the atlas.
- `atlas_texture` (method) — Returns an individual image atlas texture.
- `clear_atlas_region` (method) — Clear a specific region of the atlas texture.
- `prepare` (method) — Prepare GPU buffers for rendering, given alphas.
- `maybe_resize_alphas_tex` (method) — Update the alpha texture size if needed.
- `maybe_resize_encoded_paints_tex` (method) — Update the encoded paints texture size if needed.
- `maybe_resize_gradient_tex` (method) — Update the gradient texture size if needed.
- `maybe_update_config_buffer` (method) — Update config buffer if dimensions changed.
- `maybe_create_atlas_textures` (method) — Create any newly allocated atlas textures.
- `upload_alpha_texture` (method) — Upload alpha data to the texture.
- `upload_encoded_paints_texture` (method) — Upload encoded paints to the texture.
- `upload_gradient_texture` (method) — Upload gradient data to the texture.
- `upload_strip_pair` (method) — Uploads two strip slices (opaque then alpha) into the frame arena and
- `strip_pass_inner` (method) — Render the strips to the specified render target.
- `width` (method) — Get the width of the image.
- `height` (method) — Get the height of the image.
- `write_to_atlas` (method) — Write image data to an atlas texture at the specified offset.
- `new` (method) — Create a new render context with the given width and height in pixels.
- `new_with` (method) — Create a new render context with a specific SIMD level.
- `encode_current_paint` (method) — Encode the current paint into a `Paint` that can be used for rendering.
- `paint_has_area` (method) — Whether drawing with the current paint can produce any pixels.
- `fill_path` (method) — Fill a path with the current paint and fill rule.
- `fill_path_with` (method) — Build strips for a filled path with the given properties and record the draw.
- `push_clip_path` (method) — Push a new clip path to the clip stack.
- `pop_clip` (method) — Pop the most recently pushed clip path or clip rectangle from the clip stack.
- `stroke_path` (method) — Stroke a path with the current paint and stroke settings.
- `stroke_path_with` (method) — Build strips for a stroked path with the given properties and record the draw.
- `glyph_run` (method) — Creates a builder for drawing a run of glyphs that have the same attributes.
- `push_layer` (method) — Push a new layer with the given properties.
- `push_clip_layer` (method) — Push a new clip layer.
- `push_blend_layer` (method) — Push a new blend layer.
- `push_opacity_layer` (method) — Push a new opacity layer.
- `push_mask_layer` (method) — Push a new mask layer.
- `push_filter_layer` (method) — Push a new filter layer.
- `pop_layer` (method) — Pop the last pushed layer.
- `set_blend_mode` (method) — Set the blend mode for subsequent rendering operations.
- `set_stroke` (method) — Set the stroke settings for subsequent stroke operations.
- `stroke` (method) — Get the current stroke.
- `stroke_mut` (method) — Get a mutable reference to the current stroke.
- `set_tint` (method) — Set the tint for subsequent image paint operations.
- `reset_tint` (method) — Clear the tint, so subsequent image paints are drawn without tinting.
- `paint` (method) — Get the current paint.
- `set_paint_transform` (method) — Set the current paint transform.
- `reset_paint_transform` (method) — Reset the current paint transform.
- `set_fill_rule` (method) — Set the fill rule for subsequent fill operations.
- `set_transform` (method) — Set the transform for subsequent rendering operations.
- `reset_transform` (method) — Reset the transform to identity.
- `set_filter_effect` (method) — Apply filter to the current paint (affects next drawn element).
- `reset_filter_effect` (method) — Reset the current filter effect.
- `reset_and_resize` (method) — Reset the scene and update its size.
- `reset` (method) — Reset scene to default values.
- `width` (method) — Get the width of the render context.
- `height` (method) — Get the height of the render context.
- `take_current_state` (method) — Take current rendering state and reset the existing state to its default.
- `save_current_state` (method) — Save a copy of the current rendering state.
- `restore_state` (method) — Restore rendering state.
- `allocation_size` (method) — Size of the atlas allocation needed to hold the region and its padding.
- `scratch_texture` (method) — Whether access to the scratch texture has been requested.
- `require_scratch_texture` (method) — Indicate that access to the scratch texture is required.
- `allocate_reusing` (method) — Try to accommodate the given layer allocation request, possibly advancing the round counter
- `release` (method) — Mark the given allocation as ready to release in the given round.
- `advance` (method) — Advance to the next round.
- `opaque_draw_pass` (method) — Execute the opaque pass against the root target with the given strips. If this method is
- `draw_pass` (method) — Execute a draw pass against the given target.
- `clear_pass` (method) — Execute a clear pass with the given rectangles against the target.
- `blend_pass` (method) — Execute a blend pass between a parent and child texture.
- `filter_pass` (method) — Execute a filter pass according to the filter plan between two textures.
- `combine` (method) — Combine the current set of allocations with another one.
- `texture_count` (method) — Return the total number of physical textures represented by these allocations.
- `validate` (method) — Validate that satisfying these requirements from the backend's existing allocations stays
- `new` (method) — Create a scheduler.
- `build` (method) — Build the complete dependency-ordered schedule.
- `schedule_root` (method) — Schedule the root command stream into the configured output target.
- `schedule_layer` (method) — Schedule an open layer bottom-up and return its completed, still-live allocation.
- `open_layer` (method) — Create an unallocated scheduling view of a recorded layer with the given visible bounds.
- `open_root_layer` (method) — Represent the root command stream as an intermediate layer that can be sampled.
- `layer_texture_parity` (method) — Select the texture group for a recorded layer depth.
- `push_draws` (method) — Append a recorded draw range to the next compatible draw pass for the target.
- `compose_layer` (method) — Compose a rendered child into a parent layer.
- `compose_simple_layer` (method) — Compose a completed child by drawing it directly into its parent using source-over.
- `clear_and_release_allocation` (method) — Schedule an allocation to be cleared and released after the given round.
- `ensure_layer_target` (method) — Lazily allocate a target for an open layer and return its scheduling state.
- `next_draw` (method) — Return the earliest point at which another draw can be appended to this target.
- `next_draw_after` (method) — Return the earliest draw point that executes after an external dependency.
- `required_layer_pages` (method) — Derive the required page count for the even and odd texture groups.
- `bind_at_or_after` (method) — Bind the required texture pages in the first compatible round at or after `point`.
- `validate` (method) — Validate the texture-binding invariants assumed by the render backends.
- `validate` (method) — Validate the bindings required by this round's scheduled work.
- `after` (method) — Return the first occurrence of `stage` strictly after this point.
- `after_or_at` (method) — Return the first occurrence of `stage` at or after this point.
- `merge` (method) — Try to merge this binding with the other one.
- `layer_id` (method) — Return the bound texture for a parity, if present.
- `required_textures` (method) — Return the required textures for this binding.
- `crop_to` (method) — Restrict this region to the given scene-space bounds while preserving its texture mapping.
- `enable_depth` (method) — Whether this target supports the depth-buffer optimization.
- `geometry_shift` (method) — A positional shift that needs to be applied to all geometry when rendering to this target.
- `new` (method) — Create new constraints with given min/max dimensions
- `calculate_dimensions` (method) — Calculate dimensions while preserving aspect ratio within constraints
- `default` (method) — Creates default constraints with reasonable values

**graph**

- `create_texture` (function) — Create a texture with the requested filtering and clamp-to-edge wrapping.
- `schedule_child_layer` (method) — Resolve and schedule the optional child layer referenced by a command node.

**numerics**

- `insert` (method) — Insert or replace a texture binding.

_593 undocumented functions — see `modules/crates_vector-hybrid-renderer.md`._

### `crates/vector-hybrid-shaders` — 51 functions (19 documented)

_WESL shader sources linked to WGSL and optionally compiled to GLSL for `vello_gpu`._

**3d**

- `compile_wgsl_shader` (function) — Compiles the given wgsl source into GLSL using [naga].
- `lint` (function) — Runs every WGSL shader lint over `module` and panics with a single aggregated
- `minify_whitespace` (function) — Removes comments and whitespace that are unnecessary between shader tokens.
- `minify_code` (function) — Removes comments and optional whitespace while preserving shader token boundaries.
- `needs_separator` (function) — Reports whether removing whitespace would merge two adjacent shader tokens.
- `to_generated_code` (method) — Generate Rust modules that contain vertex and fragment source code. Uniforms and texture

**general**

- `check` (function) — Detects user-defined struct values reachable from the `@fragment` entry point.
- `base_variable_load` (function) — If `pointer` is a direct load of a `GlobalVariable` or `LocalVariable` (with no
- `minify_wgsl` (function) — Renames every non-entry-point declaration in linked WGSL and removes redundant whitespace.
- `append_minified_code` (function) — Appends minified code without merging tokens across the append boundary.
- `rename_module` (function) — Assigns short names throughout a module and returns the global-name reverse map.
- `rename_function` (function) — Renames a function declaration, its arguments, locals, and named expressions.
- `rename_optional` (function) — Replaces an optional Naga IR declaration name with the next short name, leaving unnamed
- `validate` (function) — Validates the renamed module and produces the metadata required by Naga's writer.
- `next` (method) — Returns the next spreadsheet-style name: `A` through `Z`, then `AA`, `AB`, and so on.
- `copy_quoted` (function) — Copies one quoted value verbatim, including escaped characters.
- `first_char` (function) — Returns the first character of a known non-empty source section.
- `last_char` (function) — Returns the last character of a known non-empty source section.
- `new` (method) — Create a new `ReflectionMap` given the [`naga`] compile info.

_32 undocumented functions — see `modules/crates_vector-hybrid-shaders.md`._

### `crates/vector-locus-geometry` — 226 functions (109 documented)

_Categorical locus vector geometry kernel.

The editable path model is anchor based: a [`PathData`] is a list of [`SubPath`]s,
each a list of [`Anchor`]s with absolute in/out handle positions.

Coordinates are document points (1/72 in), y pointing down._

**2d**

- `at` (method) — Point and unit tangent at distance `s` along the path.
- `tangent` (function) — Unit tangent of `s` at `t`, robust at ends where control points coincide with the end point.
- `unit` (function) — `v` normalised, or +x when it has no length.
- `radius_change` (method) — How much the radius grows when the centre of the round cut moves by `d`: `d`'s part along
- `square_to` (function) — The unit direction square to unit `a`, on `b`'s side.
- `intersects_rect` (function) — Does the path intersect (or lie inside) the rect?
- `union_opt` (function) — Union of two optional rects.
- `constrain_angle` (function) — Snap a vector to the nearest multiple of `step_deg`.
- `constrain_angle_from` (function) — [`constrain_angle`] in steps counted from `base_deg`.
- `rect_from_points` (function) — Rect from two corner points in any order.
- `reference_point` (function) — The 9 reference points of a rect (row-major from top-left).
- `bounds` (method) — Tight geometric bounds (curve extrema, not control points).
- `control_bounds` (method) — Bounds including control handles (used for quick culling).
- `rectangle` (function) — Rectangle with 4 corner anchors, clockwise from the top-left.
- `rounded_rectangle` (function) — Rounded rectangle (8 anchors). The radius is clamped to half the shorter side.
- `rectangle_with_corners` (function) — Rectangle whose corners (top-left, top-right, bottom-right, bottom-left, y down) each have
- `ellipse` (function) — Ellipse inscribed in `r` with 4 smooth anchors (left, top, right, bottom).
- `ellipse_pie` (function) — The ellipse inscribed in `r` cut as a pie: from `start` to `end` degrees, measured
- `rectangular_grid` (function) — Rectangular grid as separate open lines (plus an optional frame rectangle).
- `polar_grid` (function) — Polar grid: concentric ellipses plus radial dividers.

**3d**

- `offset` (method) — Where `plane` is along its normal (points; 0: default position).
- `set_offset` (method) — Move `plane` to `offset` along its normal.
- `axes` (method) — A plane's axes: `[u, v, normal]`.
- `homography_at` (method) — The homography of the plane parallel to `plane` that lies `depth` points along its normal.

**computational geometry**

- `preset` (method) — Preset for `kind` (1, 2, or 3) fitted to a bounding box.
- `map_rect_bbox` (method) — The bounding box of the image of `r`.
- `polygon` (function) — Regular polygon with `sides` (≥ 3), first vertex straight up from the centre.

**general**

- `len` (method) — The path's length.
- `is_empty` (method) — True if no segment has any length.
- `nearest` (method) — The distance along the path of its point nearest `p`.
- `fraction_at` (method) — Distance as a fraction (0..1) of the path's length.
- `subpaths` (function) — Element ranges of the subpaths of `bp` (each with whether it is closed).
- `segments` (function) — The segments of one subpath.
- `kink` (function) — Do segments `a` and `b` (`b` following `a`) meet at a corner?
- `of` (method) — The corner of anchor `i` of `sp`, whose index in the path is `index`; `None` when the
- `cos` (method) — The cosine of the angle between the sides.
- `sin` (method) — The sine of the angle between the sides.
- `angle_deg` (method) — The angle between the sides, in degrees (90 at a rectangle's corners).
- `setback` (method) — How far from the corner a round cut of radius `r` meets each side: r / tan(angle / 2).
- `max_radius` (method) — The largest radius the corner draws: its cut then reaches halfway along the shorter side,
- `fitted` (method) — The radius the corner is drawn with: `radius`, no larger than [`Corner::max_radius`], 0 when
- `on_bisector` (method) — The point `k` along the bisector, measured in (u + v): the centre of a round cut of radius
- `cut` (method) — The two anchors replacing the corner when cut with `radius` (> 0, fitted): where the cut
- `arc_handle` (function) — The handle length, per unit of radius, of a cubic drawing a circular arc that spans an angle
- `path_corners` (function) — The corners of `path` Live Corners can cut, in anchor order.
- `cut_corners` (function) — `path` with each of its corners ([`path_corners`]) cut by the radius in `radii` and the kind in
- `fill_contains` (function) — Is `p` inside the filled area of `path` under `rule`? Open subpaths are implicitly closed.
- `closed` (function) — `path` with each open subpath closed by a straight line back to its start.
- `distance_to_outline` (function) — Distance from `p` to the nearest point on the path outline.
- `stroke_contains` (function) — Is `p` within `tol` of the path's stroke of width `width`?
- `normalize_deg` (function) — Normalize an angle in degrees to (-180, 180].
- `corner` (method) — A corner anchor without handles.
- `smooth` (method) — A smooth anchor with a symmetric out handle `out` (the in handle is mirrored).
- `set_handle` (method) — Place the end of the outgoing (`out`) or incoming handle at `pos`. `independent` makes the
- `reversed` (method) — Swap handles (used when reversing path direction).
- `retract` (method) — Remove both handles (convert to a sharp corner).
- `polyline` (method) — Polyline from points.
- `segment_count` (method) — Number of segments (a closed path has one more, back to the start).
- `segment` (method) — Segment `i` as a cubic (lines are cubics with handles on the anchors).
- `segment_is_line` (method) — True if segment `i` is a straight line.
- `signed_area` (method) — Signed area, closing an open subpath with a line: positive when it runs clockwise on
- `insert_anchor` (method) — Split segment `seg` at parameter `t`, inserting a new anchor. Returns the new anchor index.
- `smooth_anchor` (method) — Make anchor `i` smooth: handles in line with its neighbours (its one neighbour at an open
- `cut_at` (method) — Cut at the anchors `at`: each becomes the end of one open piece and the start of the next,
- `area` (method) — Signed area via the shoelace formula on the Bézier path (positive = clockwise in y-down).
- `from_bezpath` (method) — Convert a kurbo path to anchors. Quadratics are elevated to cubics.
- `nearest` (method) — Nearest point on any segment: (subpath, segment, t, point, distance).
- `anchors` (method) — Iterate (subpath index, anchor index, anchor).
- `length` (method) — Total length of all segments.
- `foreshortening` (method) — Foreshortening ratios along receding axes: left, right, vertical.
- `axis_col` (method) — The image of `axis`'s point at infinity.
- `homography` (method) — The homography of `plane` at its current offset.
- `frame_homography` (method) — The plane spanned by axes `u` and `v` through the point `at.1` along axis `at.0`.
- `to_page` (method) — Project a 2D point on `plane` (in plane coordinates, points) to page coordinates.
- `from_page` (method) — Map a page coordinate point to plane coordinates.
- `from_cols` (method) — From the images of (1,0,0), (0,1,0) and (0,0,1).
- `from_affine` (method) — The affine map `a` as a homography.
- `from_array` (method) — From nine row-major numbers; `None` unless all are finite and the map is invertible.
- `to_array` (method) — The nine row-major numbers.
- `apply` (method) — Map a point; `None` when it lands on or beyond the horizon (w ≤ 0).
- `inverse` (method) — Inverse.
- `then_after` (method) — `self ∘ other`: `other` first.
- `conjugated` (method) — `self` seen through `a`: the map `a ∘ self ∘ a⁻¹`.
- `affine_at` (method) — The affine map that matches `self` to first order at `p` (its derivative there).
- `simplify` (function) — Douglas–Peucker simplification with bounded stack use.
- `simplify_freehand` (function) — Alias for [`simplify`] for freehand strokes.
- `resample` (function) — Sample by distance.
- `recognize` (function) — Recognise a freehand stroke.
- `recognize_shape` (function) — Alias for [`recognize`].
- `next` (method) — The kind after this one (cycling through them).
- `max_corner_radius` (function) — The largest corner radius a `w` × `h` rectangle draws: half its shorter side.
- `fitted_corner_radius` (function) — The radius a corner of a `w` × `h` rectangle is drawn with.
- `star` (function) — Star with `points` tips (≥ 2), outer radius `r1`, inner radius `r2`, first tip up.
- `straight_star_inner_radius` (function) — Inner radius of a "straight-armed" star (shoulders aligned).
- `line` (function) — Open straight line.
- `arc` (function) — Quarter-ellipse arc from `a` to `b`.
- `spiral` (function) — Archimedean-style spiral (decay-based, `segments` quarter turns).
- `snap_to_grid` (function) — Snap a value to a grid with the given spacing and origin.
- `snap_point_to_grid` (function) — Snap a 2D point to an orthogonal grid with uniform spacing.
- `closest` (function) — Find the closest target within `tol` to `v`. Returns `(target, delta)`.
- `snap_point` (function) — Snap a point `p` against a collection of candidate points within tolerance `tol`.
- `points` (method) — Points per unit.
- `ruler_step` (method) — The rulers' label step at `zoom` (screen pixels per point), in this unit.
- `snap_to_ruler_tick` (method) — Snap `v` (points) to the nearest ruler tick at `zoom`.
- `format` (method) — Format a point value in this unit (e.g. `12.5 pt`).
- `number` (method) — Format without suffix.
- `parse` (method) — Parse a length string in this unit with optional arithmetic (`+ - * /`).
- `key` (method) — Canonical key name.
- `named` (method) — Find a unit by label, suffix, or key.

_117 undocumented functions — see `modules/crates_vector-locus-geometry.md`._

### `crates/vector-locus-pathops` — 125 functions (49 documented)

_Vector effects: distortion, 15 warp envelopes, and scribble hatching._

**2d**

- `pucker_bloat` (function) — Pucker & Bloat: anchors move toward/away from center, handles move opposite.
- `roughen` (function) — Roughen: resamples segments and jitters points.
- `tweak` (function) — Tweak: random displacement of anchors and/or control points.
- `twist` (function) — Twist: rotation strongest at center, fading to zero at bounding circle.
- `scribble` (function) — Scribble: generates zigzag hatching across a shape.
- `split_into_grid` (function) — Split a rectangular boundary into a regular grid of paths.
- `unit` (function) — Unit vector or `None` if nearly zero.
- `start_tangent` (function) — Direction the curve leaves `p0` (robust to retracted handles).
- `end_tangent` (function) — Direction the curve arrives at `p3` (forward travel direction).
- `fit_cubics` (function) — Fit a chain of cubics through `pts` with maximum parametric error `tol`.
- `fit_single` (function) — Best single cubic for `pts` with the given end tangents.
- `fit_single_from` (function) — Fit single starting from given parameter values `u`.
- `revolve` (function) — Evaluate a profile in document space. All subpaths share one axis from `bounds`.

**3d**

- `zig_zag` (function) — Zig Zag: ridges extra points per segment, alternately displaced along normal.

**computational geometry**

- `free_distort` (function) — Free Distort: bilinear mapping of bounding box onto four corners.
- `warp` (function) — Apply a warp envelope to `path` within bounding box `b`.
- `polygon_area` (function) — Signed area of a polyline ring.
- `point_in_polygon` (function) — Point-in-polygon test (winding number test).
- `line_intersection` (function) — Line segment intersection: returns parameter t in [0, 1] for segment (p0, p1) and u in [0, 1] for (p2, p3).
- `clip_polygon` (function) — Greiner-Hormann polygon clipping between two simple polygons.
- `rings_to_path` (function) — Convert clipped polygon rings into smooth cubic Bézier subpaths.
- `interior_point` (function) — Find an interior point inside a closed simple polygon.

**general**

- `noise` (function) — Deterministic hash noise in [-1, 1] using splitmix64.
- `catmull_rom` (function) — Smooth subpath through points using Catmull-Rom tangents.
- `warp_point` (function) — The warp map on normalized coordinates (x, y) in [-1, 1]^2:
- `area` (function) — Total signed area of a path.
- `flatten_subpath` (function) — Flatten a subpath into an ordered sequence of points.
- `boolean` (function) — Execute a binary boolean operation on two paths.
- `unite_all` (function) — Execute N-ary union of all input paths.
- `normalize` (function) — Normalize path by removing degenerate slivers.
- `simplify` (function) — Simplify with default corner threshold (30°).
- `simplify_with` (function) — Least-squares Bézier refit keeping corners sharper than the threshold.
- `smooth` (function) — Smooth handles towards Catmull-Rom tangents.
- `remove_redundant_points` (function) — Remove redundant points along straight edges and collinear joints.
- `add_anchor_points` (function) — Add an anchor point at the midpoint of every segment.
- `average` (function) — Average the position of selected anchors along `axis`.
- `join` (function) — Join open subpaths nearest-endpoint-first.
- `is_straight` (function) — Is the cubic (within `tol`) a straight line from p0 to p3 with handles inside the chord?
- `sample` (function) — Sample a cubic at `n+1` evenly spaced parameters.
- `outline_stroke` (function) — Outline stroke: generate the filled compound path representing the painted stroke of `path`.
- `offset_path` (function) — Offset path: expands (positive) or contracts (negative) a closed path by `offset`.
- `stroke_region` (function) — Area covered by multiple stroke outlines as a single clean compound path.
- `pathfinder` (function) — Execute a Pathfinder operation on a stack of shapes.
- `regions` (function) — Extract all distinct planar faces / regions covered by the shapes.
- `shape_builder` (function) — Shape builder planar region extractor.
- `cut_out` (function) — Cut out the portion of `target` that is covered by `tool`.
- `extrude` (function) — Generate 3D extruded and beveled faces from a 2D path.
- `visible_faces` (function) — Remove covered portions of opaque faces in back-to-front paint order.

**numerics**

- `map_nonlinear` (function) — Non-linear path mapping subdividing segments into pieces of at most `max_piece` length.

_76 undocumented functions — see `modules/crates_vector-locus-pathops.md`._

### `crates/vector-radiance-trace` — 99 functions (18 documented)

_Vector Radiance Trace: raster -> vector image tracing.

A clean-room image tracer:
1. **Quantise**: Black and White threshold, Grayscale k-means, or Color median-cut/k-means.
2. **Denoise**: 4-connected component filtering and pinhole closing.
3. **Follow boundaries**: pixel-crack contour walking into closed loops.
4. **Polygon**: Douglas-Peucker reduction removing staircases.
5. **Bézier fit**: Philip J. Schneider least-squares cubic Bézier fitting with corner preservation.
6. **Centerline**: Zhang-Suen morphological thinning and skeleton graph extraction for stroked paths._

**2d**

- `label` (function) — 4-connected component labels (`u32::MAX` outside) and per-component pixel counts.
- `dejag` (function) — Replace pixel-staircase jogs by their midpoints.
- `fit_loop` (function) — Fit one loop into a closed Bézier subpath (pixel coordinates).

**computational geometry**

- `polygon` (function) — The polygon approximating a closed loop of pixel-corner vertices within `tol`.

**general**

- `lines` (function) — The lines of colour `label` in `labels` (`w` x `h`, row-major) no wider than `max_width`
- `trace_mask` (function) — Trace every component of `mask` (row-major, `w x h`).
- `rdp_slice` (function) — Douglas-Peucker on `pts[a..=b]`, marking kept indices.
- `fit_polyline` (function) — Fit a centre line through `pts` into a Bézier subpath.
- `snap_to_lines` (function) — Retract the handles of curves that stay within `tol` of their chord.
- `trace_rgba` (function) — High-level function tracing raw RGBA8 slice buffer into vector geometry.
- `trace` (function) — Trace `img` with `params`.
- `trace_within` (function) — Trace `img` with `params`, bounded by `max_anchors`.
- `mosaic` (function) — Average colour (straight RGBA) of each of `cols` x `rows` tiles covering `img`, row by row.
- `quantize` (function) — Reduce `img` to a palette according to `params.mode`.
- `compact` (function) — Drop unused palette entries and merge duplicates.
- `kmeans_1d` (function) — 1-D k-means over a 256-bin histogram (Lloyd iterations, evenly spaced start).
- `color_palette` (function) — Color mode's palette for the histogram bins `used`, as `params.palette` asks.
- `denoise` (function) — Merge 4-connected same-label components smaller than `min_area` pixels into their most common

_81 undocumented functions — see `modules/crates_vector-radiance-trace.md`._

### `crates/vector-skia-render-adapter` — 109 functions (7 documented)

**3d**

- `flush_post_multiplied` (method) — Performs the post-processing blit when compositing in POST_MULTIPLIED alpha mode.

**general**

- `new` (method) — Create a new `SkiaSceneCache`
- `next_gen` (method) — Increment the generation on the caches. Should be called once per frame.
- `to_sk_color_channel` (function) — Convert an anyrender displacement-map [`ColorChannel`] to the corresponding Skia channel.
- `distant_light_direction` (function) — Compute the direction vector towards a distant light source from its azimuth and elevation.
- `prepare_intermediate_surface` (method) — Prepares a Skia-allocated intermediate surface.
- `prepare_swapchain_surface` (method) — Prepares a Skia surface that wraps the raw Vulkan swapchain image.

_102 undocumented functions — see `modules/crates_vector-skia-render-adapter.md`._

### `crates/vector-surface-brush` — 58 functions (10 documented)

_Categorical surface brush procedural engines: calligraphic, scatter, art spine warp,
pattern, and bristle brushes._

**2d**

- `frame` (method) — Point and unit tangent at arc length `s`.

**3d**

- `normal` (function) — Left-hand normal in y-down document space (for a path running +x it points +y).
- `in_out` (method) — Incoming and outgoing unit directions at vertex `i`.
- `map` (method) — The point at arc length `s`, offset `d` along the left normal.

**general**

- `expand_brush_stroke` (function) — Expands any procedural brush stroke into vector geometric pieces.
- `corner_positions` (method) — Arc-length positions of the corner vertices.
- `tracks` (function) — Flatten `bp` into tracks (one per subpath) with `tol` accuracy.
- `unit` (method) — Uniform in [0, 1).
- `range` (method) — Uniform in [a, b].
- `seed_of` (function) — A stable seed for a path (FNV-1a over its coordinates' bits, quantised to 1/64 pt).

_48 undocumented functions — see `modules/crates_vector-surface-brush.md`._

### `crates/video-h264-codec` — 332 functions (144 documented)

_Clean-room pure-Rust H.264 / AVC decoder, implemented from ITU-T Rec. H.264 (ISO/IEC 14496-10).

Supported: progressive (frame) coding, 8-bit 4:2:0, Baseline / Main / High profiles — CAVLC and CABAC,
I/P/B slices, 8x8 transform, custom scaling matrices, weighted prediction, spatial/temporal direct,
multiple slices, long-term references and all MMCOs, deblocking.

```no_run
let mut dec = video_h264_codec::Decoder::new();
let stream = std::fs::read("video.h264").unwrap();
for pic in dec.decode(&stream, 0).unwrap() {
println!("{}x{} poc {}", pic.width, pic.height, pic.poc);
}
for pic in dec.flush() {
let _ = pic;
}
```_

**2d**

- `rect` (method) — (x, y, w, h) of partition `i` in 4x4-block units.
- `rect` (method) — (x, y, w, h) of sub-partition `j` relative to the 8x8 block, in 4x4 units.
- `crop_rect` (method) — Cropping rectangle in luma samples: (x, y, width, height).
- `mc_rect` (method) — Motion-compensated prediction of a rectangle (4x4 units) written into the picture.

**general**

- `new` (method) — Create an engine over `data` (slice RBSP) starting at bit `pos` (must be byte aligned),
- `init_engine` (method) — 9.3.1.2: codIRange = 510, codIOffset = read_bits(9) (from the current byte-aligned position).
- `refill` (method) — Load 32 more bits (zeros past the end of the data).
- `bit_pos` (method) — Bit position of the next bit not consumed by the arithmetic decoder (for I_PCM alignment
- `set_bit_pos` (method) — Continue reading at byte-aligned bit position `pos` (call `init_engine` afterwards).
- `overrun` (method) — True when the reader has run past the end of the data (corrupt stream).
- `decode_decision` (method) — 9.3.3.2.1, branch-free: the MPS / LPS outcome of a context-coded bin is close to random
- `decode_cabac` (method) — Decode slice_data() with CABAC. `rbsp` is the whole slice RBSP.
- `decode_mb_type_i` (method) — Intra mb_type suffix / I-slice mb_type (Table 9-36). `base` is 3 (I slice) or 17/32 (prefix
- `decode_ref_idx` (method) — ref_idx_lX for the partition whose top-left 4x4 block is (x, y) (9.3.3.1.1.6).
- `decode_mvd` (method) — One mvd component for the (sub-)partition with top-left 4x4 block (x, y) (9.3.3.1.1.7, UEG3).
- `cbp_luma_cond` (method) — condTermFlagN for coded_block_pattern luma bin from a neighbouring MB; `b8n` is the neighbouring
- `cbf_luma_cond` (method) — coded_block_flag ctxIdxInc condition for a neighbouring luma 4x4 block (cat 0..2) or chroma.
- `residual_block_cabac` (method) — residual_block_cabac for ctxBlockCat `cat` (0..=5). Coefficients are written to `coeff`
- `decode_mb_cabac` (method) — Parse and reconstruct one macroblock_layer() with CABAC.
- `residual_block` (function) — Parse residual_block_cavlc(). Coefficients are written to `coeff[start_idx..=end_idx]` in scan order
- `ref_ids8` (function) — Referenced picture ids per 8x8 block and list (u32::MAX = list unused).
- `deblock_mb` (function) — Deblock one macroblock (macroblocks must be processed in raster order).
- `default_threads` (function) — The worker threads [`Decoder::new`] uses: the available cores (at most 16) with the `threads`
- `new` (method) — A decoder using all available cores (with the `threads` feature).
- `with_threads` (method) — A decoder using up to `threads` worker threads (1 = decode on the calling thread).
- `threads` (method) — Worker threads decoding pictures (1: on the calling thread).
- `set_draft` (method) — Draft mode (off by default) for reduced-resolution playback: pictures submitted from now
- `from_avcc` (method) — Configure from an `avcC` (AVCDecoderConfigurationRecord) box payload.
- `configure_avcc` (method) — Parse an `avcC` record: SPS/PPS and the NAL length size used by [`Decoder::decode`].
- `nal_length_size` (method) — Length of NAL length prefixes (from avcC), or None for Annex-B input.
- `decode` (method) — Decode one access unit (all NAL units of one picture, Annex-B or length-prefixed per
- `flush` (method) — Output all remaining pictures (end of stream).
- `stats` (method) — Statistics accumulated so far (macroblock counts include finished pictures only).
- `take_error` (method) — First error reported by a decoding job since the last call (also returned by
- `submit_pending` (method) — Start decoding the pending picture and do its reference marking / DPB insertion.
- `run_job` (function) — Decode all slices of one picture and publish its rows.
- `prune` (method) — Remove entries not needed for output and not used for reference.
- `bump` (method) — Output the picture with the smallest POC (C.4.5.3 "bumping"). Returns false if none waits.
- `flush` (method) — Output everything in POC order and clear non-reference entries.
- `idr` (method) — IDR: mark all references unused; output (or drop) prior pictures.
- `sliding_window` (method) — Sliding window marking (8.2.5.3).
- `fill_frame_num_gap` (method) — Insert "non-existing" frames for a frame_num gap (8.2.5.2). `make_frame` gives each one's
- `build_ref_lists` (method) — Initial reference picture lists (8.2.4.2) + modification (8.2.4.3) for a frame slice, each
- `apply_mmcos` (method) — Adaptive memory control (8.2.5.4). Returns true if MMCO 5 was executed.
- `store_picture` (method) — Reference marking for the just-decoded picture and insertion into the DPB with output bumping.
- `fetch_window` (function) — Copy the (bw+5)x(bh+5) window whose top-left is (x-2, y-2) into `win` (stride WIN), clamping
- `copy_n` (function) — Copy `n` bytes with a fixed-size move for the common block widths.
- `htap_row` (function) — Horizontal 6-tap (unclipped) at every position of `row` for `n` outputs: out[c] uses row[c..c + 6].
- `vtap_row` (function) — Vertical 6-tap (unclipped) over six rows at columns 0..n.
- `mc_luma` (function) — Luma sample interpolation for a bw x bh block (8.4.2.2.1). (x, y) is the integer sample position
- `mc_luma_win` (function) — Luma interpolation from a prepared window: `src[0]` is the sample at (x - 2, y - 2) and `ss` the
- `mc_chroma` (function) — Chroma sample interpolation (8.4.2.2.2) for a bw x bh block at integer position (x, y) with
- `mc_chroma_win` (function) — Chroma interpolation from a prepared (bw + 1) x (bh + 1) window whose first sample is the integer
- `weighted_store` (function) — Combine prediction(s) into `dst` (stride `ds`). `p0`/`p1` have stride `ps`.
- `mc_luma_ref` (function) — Luma sample interpolation for a bw x bh block. (x, y) is the integer sample position
- `t` (method) — p[x, -1] for x >= -1
- `l` (method) — p[-1, y] for y >= -1
- `pred4x4` (function) — Intra_4x4 prediction written into `plane` at (x, y).
- `pred8x8` (function) — Intra_8x8 prediction (with reference sample filtering) written into `plane` at (x, y).
- `pred16x16` (function) — Intra_16x16 prediction (8.3.3) for the macroblock at luma (x, y).
- `pred_chroma` (function) — Chroma intra prediction for a 4:2:0 macroblock (8x8 block at chroma (x, y)), 8.3.4.
- `intra_mb_type` (function) — Intra mb_type (I-slice numbering 0..=25).
- `p_mb_type` (function) — P-slice mb_type 0..=4 (5.. are intra, handled by the caller).
- `b_mb_type` (function) — B-slice mb_type 0..=22 (23.. are intra).
- `parse_scaling_list` (function) — Result of parsing one scaling_list(): Some(list) or None meaning "use default".
- `parse_scaling_lists` (function) — Parse scaling lists with a fall-back rule. `fallback` gives the lists used for rule B
- `chroma_array_type` (method) — ChromaArrayType.
- `width` (method) — Decoded (uncropped) luma width.
- `crop_unit` (method) — (CropUnitX, CropUnitY).
- `max_dpb_frames` (method) — MaxDpbFrames from Table A-1 (and the VUI max_dec_frame_buffering when present).
- `max_num_reorder_frames` (method) — Number of frames that may precede a frame in decoding order and follow it in output order.
- `parse` (method) — Parse a PPS. Needs the SPS table because the tail depends on chroma_format_idc.
- `publish` (method) — Publish MB row `r` (ignored if already published).
- `row` (method) — MB row `r`, waiting until it has been published.
- `wait_complete` (method) — Wait until the whole frame is available.
- `make_row` (method) — Build a published row from a contiguous plane set (rows of MB row `r`) and motion data.
- `from_planes` (method) — A frame with every row published from `planes` and no motion (intra), e.g. "non-existing"
- `luma_window` (method) — Copy the w x h luma window at (x0, y0) (clamped to the picture) into `out` (stride `os`).
- `chroma_window` (method) — Copy the w x h window of chroma component `c` at (x0, y0) (clamped) into `out`.
- `copy_cropped` (method) — Crop and copy the frame into planar buffers (waits for completion).
- `set_plane_allocator` (function) — Where output planes come from: an empty buffer with room for the given number of samples.
- `copy_line` (function) — Copy `w` samples of `line` starting at x0 into `out`; when possible a fixed-size block of `N`
- `parse` (method) — Parse the slice header. `pps_lookup` resolves pic_parameter_set_id to (PPS, SPS).
- `is_new_picture` (function) — Is `sh` the first slice of a new primary coded picture relative to `prev` (7.4.1.2.4)?
- `compute` (method) — Compute TopFieldOrderCnt/BottomFieldOrderCnt of a frame (8.2.1.1-8.2.1.3).
- `update` (method) — Update after a picture has been decoded. `poc` is the value returned by [`PocState::compute`]
- `update_gap_frame` (method) — Account for a "non-existing" frame inferred by the frame_num gap process (8.2.5.2).
- `reset` (method) — Reuse the buffers of a finished picture of the same size for a new frame.
- `mb_done` (method) — Record a decoded macroblock and deblock / publish rows that became final.
- `flush_rows` (method) — Deblock and publish rows. With `finished`, everything remaining is processed (also rows with
- `predict_block` (function) — Interpolate a bw x bh luma block at (px, py) with motion vector `mv` from reference `f` into `y`,
- `poc_delta` (function) — H.264 temporal weighting uses clipped POC distances; widen before
- `start_mb` (method) — Set up the current macroblock address and neighbour availability.
- `intra_nb` (method) — Neighbour MB usable for intra prediction (constrained_intra_pred aware).
- `nc_luma` (method) — nC for a luma 4x4 block (raster index) - 9.2.1.
- `nb_motion` (method) — Motion data of the neighbouring 4x4 block at (x, y) (4x4 units, relative to the current MB).
- `nb_abc` (method) — Returns (A, B, C) neighbour motion for a partition at (x, y) of width w (4x4 units), with C
- `mvp` (method) — Luma motion vector prediction (8.4.1.3). `shape`: 1 = 16x8, 2 = 8x16, 0 = other.
- `set_motion` (method) — Store motion for a rectangle (4x4 units) of the current MB and mark it decoded.
- `colocated` (method) — Co-located (mvCol, refIdxCol, referenced picture id) for 4x4 block `raster` of the current MB.
- `spatial_direct_base` (method) — Spatial direct: MB-level reference indices, predicted mvs and directZeroPrediction (8.4.1.2.2).
- `derive_direct` (method) — Derive direct-mode motion for the 8x8 blocks in `b8_mask` of the current MB.
- `derive_p_skip` (method) — P_Skip motion (8.4.1.1).
- `derive_inter_motion` (method) — Derive mvs of the current (non-skip) inter MB from `self.cur` (ref_idx, mvd).
- `uniform_motion` (method) — All 16 blocks of the current MB share references and motion vectors.
- `predict_inter` (method) — Inter prediction for the whole current MB, using the stored motion.
- `derive_intra_modes` (method) — Resolve Intra4x4PredMode / Intra8x8PredMode for the current MB from `self.cur.rem_mode`.
- `reconstruct_intra` (method) — Reconstruct an intra MB (prediction + residual) after its syntax has been parsed.
- `reconstruct_inter_residual` (method) — Add the residual of an inter MB (prediction already written).
- `write_pcm` (method) — Write I_PCM samples.
- `finish_pcm` (method) — Common state for an I_PCM MB.
- `apply_qp_delta` (method) — Update QP after mb_qp_delta.
- `put_luma4` (method) — Store a 4x4 luma block given in scan order (levels[0..16]); `ac_only` skips position 0.
- `put_luma8` (method) — Store an 8x8 luma block given in scan order.
- `put_luma_dc` (method) — Intra16x16 DC levels in scan order -> scaled DC of each 4x4 block.
- `put_chroma_dc` (method) — Chroma DC levels (4:2:0: 4 values in raster order) for component `c`.
- `put_chroma_ac` (method) — Chroma AC levels (scan positions 1..16 in levels[1..]) for block `b` of component `c`.
- `decode_skip` (method) — Decode a P_Skip or B_Skip macroblock at the current address.
- `decode_mb_cavlc` (method) — Parse and reconstruct one macroblock_layer() with CAVLC.
- `no_sub_8x8_lt` (method) — Whether transform_size_8x8_flag may be present for an inter MB (NoSubMbPartSizeLessThan8x8 etc.).
- `decode_cavlc` (method) — Decode slice_data() with CAVLC. `r` is positioned after the slice header.
- `render` (function) — Expected cropped planes of a picture made of PCM blocks (`None` = copy from `reference`).
- `cross_check_ffmpeg` (function) — Validate the hand-built stream and its expected output with ffmpeg when available.
- `poc_type1_reordering` (function) — POC type 1: non-reference pictures are displayed before the preceding reference picture.
- `norm_adjust4` (function) — normAdjust4x4(m, i, j) for raster position `pos` (i = row, j = col).
- `norm_adjust8` (function) — normAdjust8x8(m, i, j) for raster position `pos` (i = row, j = col).
- `scale4` (function) — Scale one 4x4 AC coefficient (8.5.12.1, not the DC of Intra16x16 / chroma blocks).
- `scale8` (function) — Scale one 8x8 coefficient (8.5.13.1).
- `luma_dc_dequant` (function) — Intra16x16 luma DC: inverse Hadamard + scaling (8.5.10). `c` is raster order (row*4+col);
- `chroma_dc_dequant_420` (function) — 4:2:0 chroma DC: 2x2 transform + scaling (8.5.11). `c` = [c00, c01, c10, c11].
- `idct4_add` (function) — Inverse 4x4 transform of raster-order `d` and add to `dst` (8.5.12.2, 8.5.14).
- `idct4_dc_add` (function) — DC-only shortcut: all AC coefficients zero.
- `idct8_add` (function) — Inverse 8x8 transform of raster-order `d` and add to `dst` (8.5.13.2): rows, then columns,
- `reference_add` (function) — The straightforward per-column formulation (8.5.12.2 / 8.5.13.2) the vectorised
- `huge_levels_dequantise_without_overflow_panics` (function) — Corrupt streams can carry huge coefficient levels: dequantisation wraps (as release

**graph**

- `filter8` (function) — Filter one set of eight samples p3 p2 p1 p0 | q0 q1 q2 q3 across an edge (8.7.2.3 / 8.7.2.4):
- `filter_lanes` (function) — Filter one edge for `N` lines at once (lane `i` = line `i` across the edge). `s[k][i]` holds
- `filter_vertical` (function) — Filter a vertical edge whose q0 column is at `x`, for `N` lines starting at row `y`.
- `filter_horizontal` (function) — Filter a horizontal edge whose q0 row is `y`, for `N` columns starting at `x`.
- `lane_filter_matches_per_line_reference` (function) — The vectorised edge filter equals the per-line reference [`filter8`] on random edges
- `edge_filter_speed` (function) — Rough timing of the edge filters (`cargo test --release -p video-h264-codec --lib edge_filter_speed -- --ignored --nocapture`).
- `pred_nxn` (function) — Directional / DC prediction shared by Intra_4x4 and Intra_8x8 (8.3.1.2.x, 8.3.2.2.x).
- `copy_clamped` (function) — Copy `out.len()` samples of `line` starting at x0, replicating edge samples outside the line.

**physics**

- `init_tables` (function) — Force table construction (so the first slice does not pay for it).

_188 undocumented functions — see `modules/crates_video-h264-codec.md`._

### `crates/video-h264-preview` — 99 functions (21 documented)

_Bounded, silent, P/I/B-frame MP4/H.264 preview using the existing QualiaDB MP4
demux dependency and H.264/color operations. This is an adapter,
not a project/session, HMC format, media authority, or timeline clock.
Informed by open standards and Informatics Fiduciary guarantees under UDHR Articles 19 and 27._

**3d**

- `project_video_container` (function) — Select one video track without changing source bytes or moving media. For a

**economics**

- `decode_mp4` (function) — Decode a single traversal of the selected media range. Caller must supply a

**general**

- `scale_rgba` (function) — Area-average downscale (or nearest upscale) of packed RGBA.
- `admit` (method) — Mirrors Decoder::start_picture and submit_pending, including a gap on
- `decode_mp4_stream` (function) — Decode to a caller-owned, backpressured sink without retaining the whole
- `admit_sample_table` (function) — Bound every table allocation before passing the input to the MP4 parser.
- `check_top_level` (function) — Admission for the owning demuxer: reject fragmented/event/extended boxes
- `check_sample` (function) — Reject in-band SPS/PPS changes, B/SP/SI slices and mixed IDR/non-IDR
- `validate_picture_headers` (function) — Match the codec's own primary-picture boundary test before it can create
- `fit_size` (function) — The size that fits `w` x `h` inside `max_w` x `max_h`, keeping the aspect ratio (>= 1 px).
- `open` (method) — Open a movie held in memory.
- `open_source` (method) — Open a movie from any random-access byte source.
- `frame_color` (function) — Limited-range Y'CbCr of frame `i` for synthetic test streams.
- `h264_mp4_with` (function) — Build an H.264 MP4 with customized macroblock dimensions, frame count, and color generator.
- `h264_mp4` (function) — 64x48 H.264 MP4 of `n` solid frames at 24 fps, with an IDR picture every `gop` frames.
- `test_pattern_mp4` (function) — Color-bar test movie for demos, tests, and preview verification (320x192 at 24 fps).
- `prores_mov` (function) — QuickTime MOV containing `n` frames of ProRes 422 HQ (64x48) at 25 fps.

**numerics**

- `new` (method) — BT.601 (SD) or BT.709 (HD) coefficients.
- `for_code` (method) — Choose by ITU-T H.273 matrix code (1 = 709, 5/6 = 601), else by picture height.
- `planar_to_rgba` (function) — Planar 8-bit Y'CbCr with chroma subsampled by `cx` horizontally and `cy` vertically

**semantics**

- `rgb` (method) — Convert one 8-bit sample triple.

_78 undocumented functions — see `modules/crates_video-h264-preview.md`._

### `crates/video-prores-codec` — 88 functions (29 documented)

_Clean-room Apple ProRes codec (SMPTE RDD 36) and QuickTime MOV container support.

Technical implementation grounded in open standards and Informatics Fiduciary duties
(supporting open cultural participation and creative expression under UDHR Articles 19 and 27;
ICESCR Article 15).

- **Decoder** ([`decode_frame`], [`decode_frame_with`], [`decode_frame_into`]):
ProRes 422 (Proxy, LT, Standard, HQ) and 4444 / 4444 XQ including 16-bit alpha channel.
- **Encoder** ([`Encoder`]):
ProRes 422 and 4444/XQ progressive encoding with per-slice rate control.
- **QuickTime Container** ([`encode_prores_mov`], [`decode_prores_mov`], [`probe_prores_mov`]):
Full `.mov` muxing, demuxing, and metadata extraction._

**general**

- `refill` (method) — Ensure at least 56 valid bits are cached.
- `only_padding_left` (method) — True when every remaining bit is zero (the component's padding), or nothing is left.
- `read_cw` (method) — Decode one codeword; `None` for codewords longer than 56 bits (never produced by valid streams).
- `cw_len` (function) — Number of bits of the codeword for `val` in codebook `cb`.
- `finish` (method) — Pad with zero bits to a byte boundary and return the bytes.
- `idct8x8` (function) — In-place inverse DCT. `block` holds dequantised coefficients in raster order
- `fdct8x8` (function) — In-place forward DCT (samples in, coefficients out; same layout as [`idct8x8`]).
- `probe` (function) — Parse only the frame header (cheap; for probing geometry and format).
- `decode_frame` (function) — Decode one ProRes frame (the contents of one QuickTime sample) with default options.
- `decode_frame_with` (function) — Decode one ProRes frame.
- `decode_frame_into` (function) — Decode one ProRes frame into `out`, reusing its plane allocations (the fastest way to decode
- `decode_component` (function) — Entropy-decode and dequantise one slice component into `blocks` (raster-order coefficients).
- `decode_alpha` (function) — Decode a slice's alpha plane (raster order over the slice's 16 lines) into `out`
- `target_frame_bytes` (method) — The rate-control target for one frame in bytes (excluding alpha).
- `encode` (method) — Encode one frame. Input planes may be at any bit depth 8..=16 (10 is typical); the frame's
- `parse` (method) — Parse the 8-byte frame prefix and frame header. Returns the header, the byte offset of the
- `write` (method) — Serialise the frame header (without the 8-byte size/identifier prefix).
- `row_slice_widths` (function) — Slice widths (in macroblocks) of one macroblock row: as many `1 << log2` slices as fit, then
- `encode_prores_mov` (function) — Encode a sequence of same-sized frames into a QuickTime `.mov` container at the given frame rate.
- `probe_prores_mov` (function) — Probe a QuickTime `.mov` container and return information about its first ProRes video track.
- `decode_prores_mov` (function) — Decode all ProRes frames from a QuickTime `.mov` container.
- `qscale` (function) — Quantisation scale for a slice `quantization_index` (1..=224).
- `chroma_width` (method) — Width of a chroma plane for a given luma width.
- `fourcc` (method) — The QuickTime sample-entry four-character code.
- `nominal_mbps_1080p30` (method) — Apple's published nominal data rate at 1920x1080, 29.97 fps, in Mbit/s.
- `nominal_bits_per_mb` (method) — Nominal coded bits per 16x16 macroblock (the nominal rate spread over the 8160
- `new` (method) — A black (video-range) frame with the given geometry.

**numerics**

- `qfactors` (function) — Dequantisation factors (matrix x scale) for a slice.
- `default` (method) — BT.709 primaries, transfer and matrix.

_59 undocumented functions — see `modules/crates_video-prores-codec.md`._

