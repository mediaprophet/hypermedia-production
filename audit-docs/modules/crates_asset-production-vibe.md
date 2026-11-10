# `crates/asset-production-vibe`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Acoustics 32-Bus AudioServer mixing DAG & PGA spatializer,
Discourse telemetry inspection & fiduciary RID verification,
and Chronicle locked-delta-t deterministic movie capture & 60 Hz PGA motor spatial sync.

Grounded in UDHR Art. 12, 18, 19, 27 and ICCPR Art. 17, 19 for informatics fiduciary
provenance, deterministic multimedia synchronization, and spatial audio reproduction._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl AssetHost<'_>::route_bus_dag` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:11` | - |  |
| `impl AssetHost<'_>::spatialize_pga_zone` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:339` | - |  |
| `host` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:464` | - |  |
| `insert_tone` | 4 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:474` | - |  |
| `peak` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:482` | - |  |
| `test_route_bus_dag_master_is_non_silent_object` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:490` | - |  |
| `test_spatialize_pga_zone_inverse_distance_object` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/acoustics_bus_spatial.rs:542` | - |  |
| `quantize_pga_motor` | 4 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:13` | - | Quantizes an 8-float PGA motor state (position [x,y,z], rotation [qx,qy,qz,qw], sequence, entity_id) |
| `impl AssetHost<'_>::inspect_engine_telemetry` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:98` | - |  |
| `impl AssetHost<'_>::verify_fiduciary_rid` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:205` | - |  |
| `impl AssetHost<'_>::capture_deterministic_movie` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:275` | - |  |
| `impl AssetHost<'_>::sync_spatial_motors` | 2 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/discourse_chronicle.rs:322` | - |  |
| `schroeder_reverb` | 3 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:20` | - | Deterministic 4-comb, 2-allpass Schroeder reverberator tail generator. |
| `apply_antipop_ramp_down` | 1 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:86` | - | Applies a 64-sample anti-pop lookahead cosine ramp down to zero. |
| `apply_dsp_compressor` | 3 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:100` | - | Soft-knee compressor / limiter insert. |
| `impl AssetHost<'_>::acoustics_discourse_chronicle_ext_op` | 3 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:120` | - | Categorical operations dispatch for acoustics, discourse, and chronicle extensions. |
| `create_test_host` | 4 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:173` | - |  |
| `test_route_bus_dag_3_buses` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:189` | - |  |
| `test_route_bus_dag_cycle_detection` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:275` | - |  |
| `test_spatialize_pga_zone` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:312` | - |  |
| `test_inspect_engine_telemetry` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:366` | - |  |
| `test_verify_fiduciary_rid` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:390` | - |  |
| `test_capture_deterministic_movie` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:435` | - |  |
| `test_sync_spatial_motors` | 0 | `crates/asset-production-vibe/src/acoustics_discourse_chronicle_ext/mod.rs:475` | - |  |
| `impl AssetHost<'_>::acoustics_ext` | 3 | `crates/asset-production-vibe/src/acoustics_ops.rs:9` | - | Categorical operations dispatch for the `acoustics` category extensions. |
| `impl AssetHost<'_>::filter_biquad` | 2 | `crates/asset-production-vibe/src/acoustics_ops.rs:31` | - | Audio EQ Cookbook biquad IIR filter (lowpass, highpass, bandpass, notch, peaking). |
| `impl AssetHost<'_>::mix_tracks` | 2 | `crates/asset-production-vibe/src/acoustics_ops.rs:98` | numerics | Multi-track audio mixer with linear fade envelopes and master gain. |
| `impl AssetHost<'_>::encode_aac` | 2 | `crates/asset-production-vibe/src/acoustics_ops.rs:248` | - | AAC-LC CBR audio encoding with ADTS framing. |
| `integrated_lufs_bs1770` | 2 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:8` | - | ITU-R BS.1770-4 / EBU R128 gated integrated LUFS. |
| `impl AssetHost<'_>::schedule_cpm_evm_montecarlo` | 2 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:63` | - | 138: `hypermedia.kinetics.schedule_cpm_evm_montecarlo` |
| `impl AssetHost<'_>::analyze_ebu_r128_camelot_daw` | 2 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:135` | - | 139: `hypermedia.acoustics.analyze_ebu_r128_camelot_daw` |
| `impl AssetHost<'_>::kriging_variogram_morphometry` | 2 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:237` | - | 141: `hypermedia.geodesy.kriging_variogram_morphometry` |
| `host` | 2 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:356` | - |  |
| `test_ebu_r128_loudness_object_not_floor` | 0 | `crates/asset-production-vibe/src/acoustics_rf_kriging_cpm.rs:367` | - |  |
| `impl AssetHost<'_>::chronicle_op` | 3 | `crates/asset-production-vibe/src/chronicle_ops.rs:13` | - |  |
| `impl AssetHost<'_>::mux_mp4` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:29` | - |  |
| `impl AssetHost<'_>::decode_mp4_frame` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:115` | - |  |
| `impl AssetHost<'_>::produce_locked_dt_maps` | 4 | `crates/asset-production-vibe/src/chronicle_ops.rs:185` | - | Locked-dt ProRes + OTIO + Matroska + fiduciary hash-chain maps. |
| `locked_dt_frame_rate` | 1 | `crates/asset-production-vibe/src/chronicle_ops.rs:414` | - |  |
| `rgba_to_prores` | 1 | `crates/asset-production-vibe/src/chronicle_ops.rs:429` | - |  |
| `fnv1a_hex` | 1 | `crates/asset-production-vibe/src/chronicle_ops.rs:455` | - |  |
| `nal` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:464` | - |  |
| `build_sps` | 3 | `crates/asset-production-vibe/src/chronicle_ops.rs:470` | - |  |
| `build_pps` | 0 | `crates/asset-production-vibe/src/chronicle_ops.rs:513` | - |  |
| `encode_idr_ipcm_frame` | 4 | `crates/asset-production-vibe/src/chronicle_ops.rs:534` | - |  |
| `mp4_box` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:597` | - |  |
| `mp4_full_box` | 4 | `crates/asset-production-vibe/src/chronicle_ops.rs:606` | - |  |
| `build_avc_mp4` | 7 | `crates/asset-production-vibe/src/chronicle_ops.rs:617` | - |  |
| `host` | 3 | `crates/asset-production-vibe/src/chronicle_ops.rs:825` | - |  |
| `solid_frame` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:839` | - |  |
| `encoded` | 2 | `crates/asset-production-vibe/src/chronicle_ops.rs:853` | - |  |
| `chronicle_mux_mp4_origin_still_returns_mp4` | 0 | `crates/asset-production-vibe/src/chronicle_ops.rs:858` | - |  |
| `chronicle_locked_dt_movie_otio_ledger_objects` | 0 | `crates/asset-production-vibe/src/chronicle_ops.rs:876` | - |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/discourse_chronicle_fiduciary.rs:5` | - |  |
| `impl AssetHost<'_>::audit_tex_pdf_ua_matlang` | 2 | `crates/asset-production-vibe/src/discourse_chronicle_fiduciary.rs:15` | - | 142: `hypermedia.discourse.audit_tex_pdf_ua_matlang` |
| `impl AssetHost<'_>::recalculate_tabular_fiduciary_archive` | 2 | `crates/asset-production-vibe/src/discourse_chronicle_fiduciary.rs:178` | - | 143: `hypermedia.chronicle.recalculate_tabular_fiduciary_archive` |
| `impl AssetHost<'_>::compile_prose_docx` | 2 | `crates/asset-production-vibe/src/discourse_docs/docx_pptx.rs:21` | - |  |
| `impl AssetHost<'_>::compile_deck_pptx` | 2 | `crates/asset-production-vibe/src/discourse_docs/docx_pptx.rs:159` | - |  |
| `crc32_ieee` | 1 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:23` | - | Branchless IEEE 802.3 CRC-32 checksum (polynomial 0xEDB88320). |
| `impl ZipArchiveWriter::new` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:49` | - |  |
| `impl ZipArchiveWriter::add_file` | 3 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:55` | - |  |
| `impl ZipArchiveWriter::finish` | 1 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:62` | - |  |
| `escape_xml` | 1 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:137` | - | Escape XML entities for OpenXML and SVG documents. |
| `escape_pdf_str` | 1 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:153` | - | Escape ASCII characters in PDF string literals `(...)`. |
| `container_family` | 2 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:176` | - | Produce-safe family for gallery presentation of sealed Q42 containers. |
| `produce_safe_container_svg` | 4 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:191` | - | Display-referred sRGB SVG card. Binary containers are never inlined as executable payloads. |
| `html_escape` | 1 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:226` | - |  |
| `impl AssetHost<'_>::discourse_op` | 3 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:261` | - | Categorical operations dispatch for the `discourse` category morphisms. |
| `impl AssetHost<'_>::compile_pdf` | 2 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:294` | - |  |
| `impl AssetHost<'_>::review_gallery` | 2 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:752` | - | CSP-locked gallery. Origin image/audio path is preserved; 10d/m64/emf/q42 |
| `host` | 2 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:888` | - |  |
| `encoded_bytes` | 2 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:901` | - |  |
| `compile_pdf_emits_tagged_pdf_ua_object` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:910` | - |  |
| `audit_tex_pdf_ua_matlang_emits_tagged_pdf_ua_object` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:944` | - |  |
| `compile_prose_docx_emits_openxml_object` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:984` | - |  |
| `compile_deck_pptx_emits_openxml_object` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:1032` | - |  |
| `compile_deck_pptx_emits_one_paragraph_per_bullet` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:1069` | - |  |
| `review_gallery_presents_10d_m64_emf_q42_maps` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:1126` | - |  |
| `review_gallery_e64_magic_fallback_presents_m64_without_kind_sibling` | 0 | `crates/asset-production-vibe/src/discourse_docs/mod.rs:1193` | - |  |
| `impl AssetHost<'_>::render_chart_svg` | 2 | `crates/asset-production-vibe/src/discourse_docs/pdf_chart.rs:27` | - |  |
| `impl AssetHost<'_>::domain_extension_op` | 3 | `crates/asset-production-vibe/src/domain_extensions.rs:35` | - | Dispatches Wave 13 domain extension morphisms (`120..=143`). |
| `wave13_domain_extension_morphisms_roundtrip` | 0 | `crates/asset-production-vibe/src/domain_extensions.rs:94` | - |  |
| `encode_advected_strata_m64` | 5 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/m64_continuum.rs:11` | - |  |
| `impl AssetHost<'_>::advect_m64_grid` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/m64_continuum.rs:71` | 3d | Solves divergence-free pressure projection via Jacobi Poisson iterations, |
| `impl AssetHost<'_>::couple_continuum_xpbd` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/m64_continuum.rs:395` | - | Evaluates Bekker / Mohr-Coulomb terramechanics pressure-sinkage, tractive shear stress, |
| `make_host` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/m64_continuum.rs:521` | - |  |
| `advect_m64_grid_strata_kriging_erosion_two_produces_match` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/m64_continuum.rs:532` | - |  |
| `impl AssetHost<'_>::electromagnetics_geodesy_ext_op` | 3 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:18` | - | Categorical operations dispatch for electromagnetics and geodesy extensions. |
| `intersect_aabb` | 3 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:52` | computational geometry, 3d | Ray-AABB intersection test returning entry and exit parameters `(t_min, t_max)`. |
| `spectral_wavelength_to_rgb` | 1 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:87` | - | CIE 1931 / Bruton piecewise spectral-to-sRGB mapping for wavelength in nanometers. |
| `sample_bilinear_2d` | 4 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:129` | - | Bilinear interpolation sampler over a 2D periodic field. |
| `neighborhood_min_max` | 4 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:151` | - | Determines the local min and max over a 2x2 neighborhood for BPS advection limiting. |
| `create_test_host` | 3 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:176` | - |  |
| `test_raymarch_vox0_dda_procedural` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:191` | - |  |
| `test_raymarch_vox0_dda_sealed_emf_non_black` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:233` | - |  |
| `test_project_metamer_nullspace_invariance` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:297` | - |  |
| `test_advect_m64_grid_divergence_reduction` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:333` | - |  |
| `test_couple_continuum_xpbd_physics` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:367` | physics |  |
| `test_dispatch_aliases_and_errors` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/mod.rs:419` | - |  |
| `impl AssetHost<'_>::raymarch_vox0_dda` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/vox0_metamer.rs:25` | - | Raymarches a 3D `.10d` `Vox0DagNode` / `.emf` volumetric brick lattice using exact |
| `impl AssetHost<'_>::project_metamer_nullspace` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/vox0_metamer.rs:373` | - | Exposes Etalon's 41-band spectral metamer fiber decomposition: |
| `create_test_host` | 3 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/vox0_metamer.rs:526` | - |  |
| `f64_field` | 2 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/vox0_metamer.rs:540` | - |  |
| `test_electromagnetics_metamer_nullspace_produce_safe_map` | 0 | `crates/asset-production-vibe/src/electromagnetics_geodesy_ext/vox0_metamer.rs:548` | - |  |
| `impl AssetHost<'_>::electromagnetics_op` | 3 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:32` | - | Categorical operations dispatch for the `electromagnetics` category. |
| `impl AssetHost<'_>::compile_emf` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:57` | - | Synthesizes a 3D wave-optical & electromagnetic field volume and seals it |
| `impl AssetHost<'_>::verify_emf` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:277` | - | Parses and cryptographically verifies the Q42EMF64 container, |
| `impl AssetHost<'_>::render_spectral_slice` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:328` | - | Renders a 2D spectral preview slice from an EMF-64 volume container to straight-alpha RGBA8. |
| `impl AssetHost<'_>::produce_wifi_csi_vitals_aoa` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:489` | 3d | 802.11 CSI → sanitized phase, contactless vitals, MUSIC AoA, camera-free receipt. |
| `impl AssetHost<'_>::resolve_emf_bytes` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:636` | - | Resolves container bytes from retained inputs or workspace encoded resources. |
| `parse_sealed_emf` | 1 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:655` | - | QualiaDB `parse` admits CRC-only files; untrusted paths require a writer Bao seal. |
| `emf_cell_display_irradiance` | 1 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:673` | numerics | Vacuum-impedance irradiance (W/m²) used as the display-linear quantity. |
| `spectral_wavelength_to_rgb` | 1 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:692` | numerics | CIE 1931 / Bruton piecewise spectral-to-sRGB mapping for wavelength in nanometers. |
| `create_test_host` | 3 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:741` | - |  |
| `test_bruton_spectral_mapping_visible_and_false_color` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:752` | - |  |
| `test_compile_verify_and_render_spectral_slice_roundtrip` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:773` | - |  |
| `compile_green_dipole` | 1 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:875` | - |  |
| `test_render_spectral_slice_default_exposure_non_black` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:908` | - |  |
| `test_render_spectral_slice_tampered_emf_fails_closed` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:936` | - |  |
| `strip_emf_bao_keep_crc` | 1 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:971` | - |  |
| `test_verify_emf_refuses_crc_consistent_bao_omitted` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:992` | - |  |
| `test_spectral_display_irradiance_visible_for_compiled_amplitude` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:1099` | - |  |
| `f64_field` | 2 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:1117` | - |  |
| `encoded_bytes` | 3 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:1124` | - |  |
| `test_electromagnetics_csi_vitals_aoa_object_two_produces` | 0 | `crates/asset-production-vibe/src/electromagnetics_ops.rs:1134` | - |  |
| `parse_brush_strokes` | 1 | `crates/asset-production-vibe/src/geodesy_ops.rs:27` | - |  |
| `sculpt_tile` | 5 | `crates/asset-production-vibe/src/geodesy_ops.rs:104` | - |  |
| `self_verify_m64` | 3 | `crates/asset-production-vibe/src/geodesy_ops.rs:146` | - |  |
| `fill_occupancy_brick` | 6 | `crates/asset-production-vibe/src/geodesy_ops.rs:191` | - |  |
| `first_class_strata_column` | 3 | `crates/asset-production-vibe/src/geodesy_ops.rs:251` | physics | First-class 6-layer STRATA column: sculpted elevation plus slope hydrology |
| `f32_to_f16` | 1 | `crates/asset-production-vibe/src/geodesy_ops.rs:279` | - |  |
| `impl AssetHost<'_>::geodesy_op` | 3 | `crates/asset-production-vibe/src/geodesy_ops.rs:302` | - |  |
| `impl AssetHost<'_>::sculpt_terrain` | 2 | `crates/asset-production-vibe/src/geodesy_ops.rs:325` | - |  |
| `impl AssetHost<'_>::compile_m64` | 2 | `crates/asset-production-vibe/src/geodesy_ops.rs:439` | - |  |
| `impl AssetHost<'_>::compile_m64_strata` | 5 | `crates/asset-production-vibe/src/geodesy_ops.rs:486` | - |  |
| `impl AssetHost<'_>::compile_m64_occupancy` | 6 | `crates/asset-production-vibe/src/geodesy_ops.rs:547` | - |  |
| `impl AssetHost<'_>::verify_m64` | 2 | `crates/asset-production-vibe/src/geodesy_ops.rs:637` | - |  |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/geodesy_ops.rs:704` | - |  |
| `encoded_bytes` | 2 | `crates/asset-production-vibe/src/geodesy_ops.rs:718` | - |  |
| `parse_m64` | 1 | `crates/asset-production-vibe/src/geodesy_ops.rs:728` | - |  |
| `compile_m64_strata_default_and_e64_magic_alias_round_trip` | 0 | `crates/asset-production-vibe/src/geodesy_ops.rs:739` | - |  |
| `compile_m64_holo_occupancy_omits_record_crc` | 0 | `crates/asset-production-vibe/src/geodesy_ops.rs:792` | - |  |
| `geodesy_strata_holo_kriging_e64_objects` | 0 | `crates/asset-production-vibe/src/geodesy_ops.rs:873` | - |  |
| `compile_m64_point_occupancy_omits_record_crc` | 0 | `crates/asset-production-vibe/src/geodesy_ops.rs:1042` | - |  |
| `strip_occupancy_bao_keep_dir_crc` | 1 | `crates/asset-production-vibe/src/geodesy_ops.rs:1095` | - |  |
| `impl AssetHost<'_>::geometry` | 3 | `crates/asset-production-vibe/src/geometry.rs:7` | - |  |
| `impl AssetHost<'_>::profile` | 2 | `crates/asset-production-vibe/src/geometry.rs:288` | - |  |
| `impl AssetHost<'_>::store_profile` | 2 | `crates/asset-production-vibe/src/geometry.rs:294` | - |  |
| `impl AssetHost<'_>::mesh_budget` | 3 | `crates/asset-production-vibe/src/geometry.rs:313` | computational geometry |  |
| `impl AssetHost<'_>::store_mesh` | 3 | `crates/asset-production-vibe/src/geometry.rs:319` | computational geometry |  |
| `points2` | 2 | `crates/asset-production-vibe/src/geometry.rs:354` | - |  |
| `points3` | 2 | `crates/asset-production-vibe/src/geometry.rs:365` | - |  |
| `bounds2` | 1 | `crates/asset-production-vibe/src/geometry.rs:381` | - |  |
| `convex` | 1 | `crates/asset-production-vibe/src/geometry.rs:393` | - |  |
| `color` | 1 | `crates/asset-production-vibe/src/geometry.rs:411` | - |  |
| `impl Default for FontMetrics::default` | 0 | `crates/asset-production-vibe/src/glyph/mod.rs:25` | - |  |
| `glyph_advance` | 1 | `crates/asset-production-vibe/src/glyph/mod.rs:55` | - | Proportional advance width in normalized em units (1.0 = em height). |
| `pair_kerning` | 2 | `crates/asset-production-vibe/src/glyph/mod.rs:118` | - | Pair kerning adjustment in normalized em units. |
| `impl SubpathBuilder<'a>::new` | 4 | `crates/asset-production-vibe/src/glyph/mod.rs:148` | - |  |
| `impl SubpathBuilder<'a>::pt` | 3 | `crates/asset-production-vibe/src/glyph/mod.rs:153` | - |  |
| `impl SubpathBuilder<'a>::stroke` | 2 | `crates/asset-production-vibe/src/glyph/mod.rs:157` | - |  |
| `impl SubpathBuilder<'a>::closed` | 2 | `crates/asset-production-vibe/src/glyph/mod.rs:170` | - |  |
| `char_to_svg_subpath` | 5 | `crates/asset-production-vibe/src/glyph/mod.rs:193` | - | Emits the clean vector strokes for `ch` into `out` as SVG path instructions `M ... L ...`. |
| `append_breaking_word` | 8 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:12` | - | Append `word` with per-character breaks so wrap width includes pair kerning. |
| `wrap_paragraph` | 4 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:38` | - | Greedy word-wrapping supporting manual newlines and bounded per-character breaking. |
| `shape_glyph_run` | 7 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:137` | - | Shapes, measures, and positions vector glyph runs into SVG path instructions. |
| `impl AssetHost<'_>::glyph_op` | 3 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:213` | semantics | Dispatches glyph operations admitted by the category-theoretic ontology. |
| `test_host` | 2 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:566` | - |  |
| `glyph_measure_bounds_kerning_and_wrap` | 0 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:577` | - |  |
| `glyph_project_svg_two_produces_match` | 0 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:620` | - |  |
| `glyph_rasterize_rgba_non_empty_and_two_produces_match` | 0 | `crates/asset-production-vibe/src/glyph/shaping_svg.rs:659` | 2d |  |
| `char_to_svg_subpath_ext` | 6 | `crates/asset-production-vibe/src/glyph/stroke_table.rs:5` | - |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:6` | - |  |
| `escape_xml` | 1 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:13` | - |  |
| `line_bidi_class` | 1 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:29` | - |  |
| `visual_reorder_line` | 2 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:47` | - | UAX #9 L2 visual order for a laid-out line. |
| `split_paragraphs` | 1 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:94` | - | `\r\n` is one paragraph break; `\n` and `\r` each start a new paragraph. |
| `paragraph_glyphs` | 2 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:122` | - | Shape `text` into composer glyphs, word/hyphen breakpoints, and hyphen-point count. |
| `lines_from_breaks` | 3 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:171` | - |  |
| `impl AssetHost<'_>::compose_knuth_plass_paragraph` | 2 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:197` | - | 131: `hypermedia.glyph.compose_knuth_plass_paragraph` |
| `test_host` | 2 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:317` | - |  |
| `compose_knuth_plass_paragraph_converges_and_two_produces_match` | 0 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:328` | - |  |
| `compose_knuth_plass_mixed_bidi_is_produce_safe` | 0 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:374` | - |  |
| `svg_text_lines` | 1 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:402` | - |  |
| `produce_svg` | 4 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:416` | - |  |
| `visual_reorder_line_i1_n1_an_matches_bidi_msdf` | 0 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:429` | - |  |
| `compose_knuth_plass_rtl_latin_n1_and_arabic_indic_produce` | 0 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:436` | - |  |
| `compose_knuth_plass_newline_splits_paragraphs` | 0 | `crates/asset-production-vibe/src/glyph_knuth_plass.rs:469` | - |  |
| `classify_bidi` | 1 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:18` | - |  |
| `impl LineSeg::dist_sq` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:57` | - |  |
| `impl LineSeg::dist` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:75` | - |  |
| `impl LineSeg::channel_bin` | 1 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:80` | - |  |
| `extract_segments` | 1 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:99` | - |  |
| `impl AssetHost<'_>::shape_harfbuzz_bidi` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:161` | - |  |
| `impl AssetHost<'_>::pack_msdf_font_atlas` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:335` | - |  |
| `test_host` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:539` | - |  |
| `shape_harfbuzz_bidi_mixed_and_rtl_numbers_l2` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:550` | - |  |
| `pack_msdf_font_atlas_metrics_map_em_box_and_two_produces_match` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/bidi_msdf.rs:580` | - |  |
| `impl AssetHost<'_>::project_diegetic_surface` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/diegetic_input.rs:8` | - |  |
| `impl AssetHost<'_>::route_input_action` | 2 | `crates/asset-production-vibe/src/glyph_layout_diegetic/diegetic_input.rs:217` | - |  |
| `host` | 4 | `crates/asset-production-vibe/src/glyph_layout_diegetic/diegetic_input.rs:423` | - |  |
| `test_layout_diegetic_projection_invertible` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/diegetic_input.rs:439` | 3d |  |
| `impl AssetHost<'_>::glyph_layout_diegetic_op` | 3 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:13` | - |  |
| `create_test_host` | 4 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:45` | - |  |
| `test_shape_harfbuzz_bidi_ltr` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:61` | - |  |
| `test_shape_harfbuzz_bidi_rtl_hebrew` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:95` | - |  |
| `test_pack_msdf_font_atlas` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:117` | - |  |
| `test_project_diegetic_surface` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:147` | - |  |
| `test_route_input_action_stages` | 0 | `crates/asset-production-vibe/src/glyph_layout_diegetic/mod.rs:181` | - |  |
| `impl LatticeProfile::admits_16bit` | 1 | `crates/asset-production-vibe/src/graphics.rs:36` | - | Named origin profiles stay 8-bit; default/2k/16-bit admit 16-bit codecs. |
| `lattice_profile` | 1 | `crates/asset-production-vibe/src/graphics.rs:42` | - | Parse an optional named lattice profile. Default admits 2k / 8-bit. |
| `u8_to_u16_lattice` | 1 | `crates/asset-production-vibe/src/graphics.rs:55` | - |  |
| `u16_to_u8_lattice` | 1 | `crates/asset-production-vibe/src/graphics.rs:60` | - |  |
| `dimensions` | 1 | `crates/asset-production-vibe/src/graphics.rs:70` | - |  |
| `signed` | 2 | `crates/asset-production-vibe/src/graphics.rs:76` | - |  |
| `impl AssetHost<'_>::rgba` | 2 | `crates/asset-production-vibe/src/graphics.rs:84` | - |  |
| `impl AssetHost<'_>::store_rgba` | 4 | `crates/asset-production-vibe/src/graphics.rs:90` | - |  |
| `impl AssetHost<'_>::image_limits` | 1 | `crates/asset-production-vibe/src/graphics.rs:105` | - |  |
| `impl AssetHost<'_>::graphics` | 3 | `crates/asset-production-vibe/src/graphics.rs:112` | - |  |
| `impl AssetHost<'_>::import_rgba` | 2 | `crates/asset-production-vibe/src/graphics.rs:175` | - |  |
| `impl AssetHost<'_>::compose` | 2 | `crates/asset-production-vibe/src/graphics.rs:261` | - |  |
| `impl AssetHost<'_>::filter` | 2 | `crates/asset-production-vibe/src/graphics.rs:337` | - |  |
| `impl AssetHost<'_>::surface` | 2 | `crates/asset-production-vibe/src/graphics.rs:386` | - |  |
| `point` | 1 | `crates/asset-production-vibe/src/graphics.rs:485` | - |  |
| `paint` | 1 | `crates/asset-production-vibe/src/graphics.rs:492` | - |  |
| `impl AssetHost<'_>::vector_render` | 2 | `crates/asset-production-vibe/src/graphics.rs:517` | - |  |
| `host` | 2 | `crates/asset-production-vibe/src/graphics.rs:684` | - |  |
| `test_radiance_png_psd_lattice_roundtrip_nontrivial` | 0 | `crates/asset-production-vibe/src/graphics.rs:695` | - |  |
| `test_radiance_2k_lattice_and_named_1024_profile` | 0 | `crates/asset-production-vibe/src/graphics.rs:784` | - |  |
| `test_radiance_png_16bit_be_high_byte` | 0 | `crates/asset-production-vibe/src/graphics.rs:823` | - |  |
| `nonnegative` | 4 | `crates/asset-production-vibe/src/graphics_pack.rs:7` | - |  |
| `blit_opaque` | 8 | `crates/asset-production-vibe/src/graphics_pack.rs:16` | - |  |
| `blit_rect_nn` | 14 | `crates/asset-production-vibe/src/graphics_pack.rs:41` | 2d |  |
| `rects_overlap` | 8 | `crates/asset-production-vibe/src/graphics_pack.rs:83` | - |  |
| `impl AssetHost<'_>::encode_rgba_png_handle` | 3 | `crates/asset-production-vibe/src/graphics_pack.rs:91` | - |  |
| `impl AssetHost<'_>::png_or_encode` | 3 | `crates/asset-production-vibe/src/graphics_pack.rs:104` | - |  |
| `impl AssetHost<'_>::pack_sprite_sheet` | 2 | `crates/asset-production-vibe/src/graphics_pack.rs:112` | 2d |  |
| `impl AssetHost<'_>::pack_nine_slice` | 2 | `crates/asset-production-vibe/src/graphics_pack.rs:260` | - |  |
| `impl AssetHost<'_>::pack_pbr_set` | 2 | `crates/asset-production-vibe/src/graphics_pack.rs:366` | - |  |
| `impl AssetHost<'_>::solve_continuum_fem_lbm` | 2 | `crates/asset-production-vibe/src/kinetics_fem_garment_blocks.rs:9` | numerics | 135: `hypermedia.kinetics.solve_continuum_fem_lbm` (`kinetics-continuum-multiphysics`) |
| `impl AssetHost<'_>::drape_orthotropic_garment` | 2 | `crates/asset-production-vibe/src/kinetics_fem_garment_blocks.rs:105` | - | 136: `hypermedia.kinetics.drape_orthotropic_garment` (`kinetics-garment-draping`) |
| `impl AssetHost<'_>::simulate_causal_block_diagram` | 2 | `crates/asset-production-vibe/src/kinetics_fem_garment_blocks.rs:209` | physics | 137: `hypermedia.kinetics.simulate_causal_block_diagram` (`kinetics-block-simulation` + `kinetics-motion-compositor`) |
| `layout_kinetics_domain_composable_artifacts_and_handles` | 0 | `crates/asset-production-vibe/src/kinetics_fem_garment_blocks.rs:325` | - |  |
| `kinetics_fem_cloth_ticks_two_produces_match` | 0 | `crates/asset-production-vibe/src/kinetics_fem_garment_blocks.rs:403` | - |  |
| `impl AssetHost<'_>::solve_two_bone_ik` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:22` | numerics |  |
| `impl AssetHost<'_>::interpolate_keyframes` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:246` | - |  |
| `point3` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:352` | - |  |
| `optional_point3` | 3 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:357` | - |  |
| `cross` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:364` | - |  |
| `dot_prod` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:372` | - |  |
| `len_sq` | 1 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:376` | - |  |
| `normalize` | 1 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:380` | - |  |
| `quat_from_to` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:390` | - |  |
| `interpolate_at` | 3 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:411` | - |  |
| `apply_easing` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:444` | - |  |
| `seal_ik_pose_skl1_10d` | 4 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:469` | - | Bind-pose triangle + 3-joint SKL1 attached into a sealed `.10d`. |
| `attach_skl1_into_10d` | 2 | `crates/asset-production-vibe/src/kinetics_ops/ik_keyframes.rs:626` | - |  |
| `impl AssetHost<'_>::export_lottie` | 2 | `crates/asset-production-vibe/src/kinetics_ops/lottie_export.rs:16` | - |  |
| `impl AssetHost<'_>::kinetics_op` | 3 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:10` | - |  |
| `string` | 3 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:29` | - |  |
| `create_test_host` | 4 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:42` | - |  |
| `test_solve_two_bone_ik_reachable` | 0 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:58` | numerics |  |
| `test_solve_two_bone_ik_unreachable` | 0 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:115` | numerics |  |
| `test_interpolate_keyframes` | 0 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:145` | - |  |
| `test_export_lottie` | 0 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:194` | - |  |
| `test_solve_two_bone_ik_skl1_10d_two_produces_match` | 0 | `crates/asset-production-vibe/src/kinetics_ops/mod.rs:262` | numerics |  |
| `impl AssetHost<'_>::kinetics_sim_op` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:12` | - |  |
| `point3` | 2 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:43` | - |  |
| `optional_point3` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:48` | - |  |
| `optional_quat4` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:55` | - |  |
| `string` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:62` | - |  |
| `dot` | 2 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:69` | - |  |
| `len_sq` | 1 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:73` | - |  |
| `len` | 1 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:77` | - |  |
| `normalize_quat` | 1 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:81` | - |  |
| `impl Dsu::new` | 1 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:97` | - |  |
| `impl Dsu::find` | 2 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:101` | - |  |
| `impl Dsu::union` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:114` | - |  |
| `impl Dsu::count_islands` | 1 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:129` | - |  |
| `create_test_host` | 4 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:148` | - |  |
| `test_interpolate_motor_fti_sclerp` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:164` | - |  |
| `test_interpolate_motor_fti_teleport` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:193` | - |  |
| `test_step_island_xpbd_ground_and_islands` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:216` | - |  |
| `test_step_island_xpbd_area_override` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:257` | - |  |
| `test_query_direct_space_intersect_ray` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:293` | 3d |  |
| `test_query_direct_space_cast_motion` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:328` | - |  |
| `test_navigate_funnel_orca` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:361` | - |  |
| `test_evaluate_nla_channelbag` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:388` | - |  |
| `test_step_simulation_zone` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:431` | - |  |
| `xpbd_body` | 3 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:458` | - |  |
| `test_step_island_xpbd_two_produces_match_permuted_ids` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:475` | - |  |
| `test_evaluate_nla_channelbag_two_produces_match` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:510` | - |  |
| `test_step_simulation_zone_two_produces_match` | 0 | `crates/asset-production-vibe/src/kinetics_sim/mod.rs:569` | - |  |
| `impl AssetHost<'_>::interpolate_motor_fti` | 2 | `crates/asset-production-vibe/src/kinetics_sim/motor_xpbd_space.rs:12` | physics, numerics | PGA MotorFTI Screw-Linear Interpolation (ScLERP) between physics ticks and render frames. |
| `impl AssetHost<'_>::step_island_xpbd` | 2 | `crates/asset-production-vibe/src/kinetics_sim/motor_xpbd_space.rs:78` | - | ID-sorted deterministic Island-Parallel XPBD rigid-body simulation. |
| `impl AssetHost<'_>::query_direct_space` | 2 | `crates/asset-production-vibe/src/kinetics_sim/motor_xpbd_space.rs:277` | - | Synchronous DirectSpaceState spatial queries over split Static/Dynamic BVH. |
| `ray_intersect_aabb` | 3 | `crates/asset-production-vibe/src/kinetics_sim/motor_xpbd_space.rs:456` | computational geometry, 3d |  |
| `sweep_sphere_aabb` | 4 | `crates/asset-production-vibe/src/kinetics_sim/motor_xpbd_space.rs:503` | computational geometry |  |
| `impl AssetHost<'_>::navigate_funnel_orca` | 2 | `crates/asset-production-vibe/src/kinetics_sim/nav_nla_zones.rs:11` | - | 3D NavMesh Portal Pathfinding + Simple Stupid Funnel + 3D ORCA Avoidance. |
| `impl AssetHost<'_>::evaluate_nla_channelbag` | 2 | `crates/asset-production-vibe/src/kinetics_sim/nav_nla_zones.rs:229` | - | Blender ActionChannelbag NLA Strip Blending + Godot LensTweenTable evaluation. |
| `impl AssetHost<'_>::step_simulation_zone` | 2 | `crates/asset-production-vibe/src/kinetics_sim/nav_nla_zones.rs:307` | - | Double-buffered Simulation / Repeat Zone iterative divergence-free curl flow. |
| `tri_area_2d` | 3 | `crates/asset-production-vibe/src/kinetics_sim/nav_nla_zones.rs:398` | - |  |
| `evaluate_tween` | 2 | `crates/asset-production-vibe/src/kinetics_sim/nav_nla_zones.rs:402` | - |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:6` | - |  |
| `xml_escape` | 1 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:13` | - |  |
| `impl AssetHost<'_>::solve_flexbox_grid_constraints` | 2 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:30` | numerics | 132: `hypermedia.layout.solve_flexbox_grid_constraints` (`layout-box-autolayout`) |
| `impl AssetHost<'_>::typeset_paged_foliation` | 2 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:195` | - | 133: `hypermedia.layout.typeset_paged_foliation` (`layout-paged-typesetter`) |
| `impl AssetHost<'_>::compile_dataflow_nodegraph` | 2 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:353` | - | 134: `hypermedia.layout.compile_dataflow_nodegraph` (`layout-nodegraph-compiler` + `layout-presentation-deck`) |
| `host` | 4 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:461` | - |  |
| `test_layout_flex_grid_exact_fill_and_fr` | 0 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:477` | - |  |
| `test_layout_paged_foliation_monotonic_pages` | 0 | `crates/asset-production-vibe/src/layout_autolayout_foliation_nodegraph.rs:506` | - |  |
| `distribute_flex_sizes` | 4 | `crates/asset-production-vibe/src/layout_ops/box_theme.rs:9` | - | Freeze-and-redistribute free main-axis space so min/max clamps stay exact. |
| `boxes_non_overlapping` | 1 | `crates/asset-production-vibe/src/layout_ops/box_theme.rs:68` | - |  |
| `impl AssetHost<'_>::solve_box_tree` | 2 | `crates/asset-production-vibe/src/layout_ops/box_theme.rs:87` | numerics | Solves deterministic 2D spatial flex or CSS-grid-like layouts across viewports. |
| `impl AssetHost<'_>::bind_theme_scope` | 2 | `crates/asset-production-vibe/src/layout_ops/box_theme.rs:564` | - | Binds structured Webizen scoped design tokens and CSS variables. |
| `impl AssetHost<'_>::layout_op` | 3 | `crates/asset-production-vibe/src/layout_ops/mod.rs:42` | - |  |
| `color_to_value` | 1 | `crates/asset-production-vibe/src/layout_ops/mod.rs:61` | - |  |
| `rgba_css` | 1 | `crates/asset-production-vibe/src/layout_ops/mod.rs:65` | - |  |
| `blend_color` | 3 | `crates/asset-production-vibe/src/layout_ops/mod.rs:74` | - |  |
| `build_theme_tokens` | 4 | `crates/asset-production-vibe/src/layout_ops/mod.rs:85` | - |  |
| `parse_theme_record` | 1 | `crates/asset-production-vibe/src/layout_ops/mod.rs:187` | - |  |
| `compute_box_corner_radius` | 3 | `crates/asset-production-vibe/src/layout_ops/mod.rs:258` | - |  |
| `compute_box_colors` | 2 | `crates/asset-production-vibe/src/layout_ops/mod.rs:269` | - |  |
| `xml_escape` | 1 | `crates/asset-production-vibe/src/layout_ops/mod.rs:341` | - |  |
| `host` | 4 | `crates/asset-production-vibe/src/layout_ops/mod.rs:365` | - |  |
| `box_spec` | 3 | `crates/asset-production-vibe/src/layout_ops/mod.rs:380` | - |  |
| `box_xywh` | 2 | `crates/asset-production-vibe/src/layout_ops/mod.rs:391` | - |  |
| `test_layout_box_tree_flex_grid_nonoverlapping` | 0 | `crates/asset-production-vibe/src/layout_ops/mod.rs:407` | - |  |
| `test_layout_framlock_crc32c` | 0 | `crates/asset-production-vibe/src/layout_ops/mod.rs:505` | - |  |
| `impl AssetHost<'_>::render_ui_sheet` | 2 | `crates/asset-production-vibe/src/layout_ops/sheet_render.rs:13` | - | Renders the resolved layout into a straight-alpha RGBA8 raster artifact. |
| `impl AssetHost<'_>::project_ui_svg` | 2 | `crates/asset-production-vibe/src/layout_ops/sheet_render.rs:258` | - | Projects the resolved layout into a standalone SVG vector resource with scoped CSS tokens. |
| `impl AssetHost<'_>::parse_render_inputs` | 3 | `crates/asset-production-vibe/src/layout_ops/sheet_render.rs:410` | - |  |
| `impl AssetHost<'_>::store_layout_rgba` | 4 | `crates/asset-production-vibe/src/layout_ops/sheet_render.rs:486` | - |  |
| `impl Default for ToolLimits::default` | 0 | `crates/asset-production-vibe/src/lib.rs:54` | - |  |
| `catalogue` | 0 | `crates/asset-production-vibe/src/lib.rs:73` | - |  |
| `arguments` | 1 | `crates/asset-production-vibe/src/lib.rs:85` | - |  |
| `impl Artifact::kind` | 1 | `crates/asset-production-vibe/src/lib.rs:105` | - |  |
| `impl Artifact::bytes` | 1 | `crates/asset-production-vibe/src/lib.rs:115` | - |  |
| `impl AssetWorkspace::encoded` | 2 | `crates/asset-production-vibe/src/lib.rs:134` | - |  |
| `impl AssetWorkspace::get` | 2 | `crates/asset-production-vibe/src/lib.rs:140` | - |  |
| `impl AssetWorkspace::insert` | 3 | `crates/asset-production-vibe/src/lib.rs:149` | - |  |
| `impl AssetHost<'_>::charge` | 3 | `crates/asset-production-vibe/src/lib.rs:179` | - |  |
| `impl AssetHost<'_>::encode` | 3 | `crates/asset-production-vibe/src/lib.rs:193` | - |  |
| `impl AssetHost<'_>::admit_encoded` | 3 | `crates/asset-production-vibe/src/lib.rs:198` | - | Admit bytes the production host already holds. The files producer accepts the handle. |
| `impl AssetHost<'_>::invoke` | 3 | `crates/asset-production-vibe/src/lib.rs:208` | - |  |
| `impl AssetHost<'_>::invoke_inner` | 3 | `crates/asset-production-vibe/src/lib.rs:217` | - |  |
| `impl AssetHost<'_>::consume_unwrap_uv01` | 3 | `crates/asset-production-vibe/src/lib.rs:434` | - | When unwrap sealed UV01 into `.10d`, public compile_10d_pbr consumes it. |
| `record` | 1 | `crates/asset-production-vibe/src/lib.rs:481` | - |  |
| `required` | 2 | `crates/asset-production-vibe/src/lib.rs:488` | - |  |
| `text` | 1 | `crates/asset-production-vibe/src/lib.rs:491` | - |  |
| `num` | 1 | `crates/asset-production-vibe/src/lib.rs:498` | - |  |
| `integer` | 2 | `crates/asset-production-vibe/src/lib.rs:511` | - |  |
| `number` | 3 | `crates/asset-production-vibe/src/lib.rs:518` | - |  |
| `count` | 4 | `crates/asset-production-vibe/src/lib.rs:521` | - |  |
| `list` | 1 | `crates/asset-production-vibe/src/lib.rs:529` | - |  |
| `boolean` | 3 | `crates/asset-production-vibe/src/lib.rs:536` | - |  |
| `fields` | 2 | `crates/asset-production-vibe/src/lib.rs:543` | - |  |
| `vector` | 1 | `crates/asset-production-vibe/src/lib.rs:550` | - |  |
| `motor_axis_angle` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:38` | - |  |
| `impl BehaviorMacro::from_str_name` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:76` | - |  |
| `impl BehaviorMacro::as_str` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:91` | - |  |
| `impl BehaviorMacro::default_hooks` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:105` | - | Returns the baseline `SubElementHooks` preset for this whole-model macro before any |
| `impl SubElementHooks::with_overrides` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:250` | - | Parses optional sub-element hook overrides from Vibe `Args` on top of `self`. |
| `impl SubElementHooks::apply_to_rig_pose` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:308` | - | Applies the macro + sub-element hooks onto an 18-joint `FaunaSkeletalRig10d` PGA motor pose. |
| `impl SubElementHooks::to_small_llm_json` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:391` | - | Formats the compact ~22-token JSON command packet that a small (0.5B–4B) LLM emits. |
| `build_fauna_production_plan_json` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:416` | - | Builds the complete **Model Production Plan** (`qualia.model-production-plan/v1`) for a Fauna |
| `build_flora_mycelium_production_plan_json` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:487` | - | Builds the complete **Flora & Mycelial Neural Network Production Plan** |
| `fnv1a_60` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:564` | - | Deterministic FNV-1a 60-bit hash (`lexicon.rs` mask `0x0FFF_FFFF_FFFF_FFFF`). |
| `push_nquin_48` | 10 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:575` | semantics | Packs a single 48-byte Super-Quin (`NQuin`: `subject`, `predicate`, `object`, `context`, |
| `compile_model_logic_q42_bytes` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:615` | - | Compiles a deterministic `.q42` (`Q42LOGIC` 64-byte header + 16 parity-verified 48-byte `NQuin` |
| `render_macro_behavior_cinema_frames` | 7 | `crates/asset-production-vibe/src/living_world_anatomy/behavioral_hooks.rs:766` | computational geometry | Renders an N-frame animated 3D cinema sequence driven by a `BehaviorMacro` and `SubElementHooks`. |
| `impl AssetHost<'_>::compile_botanical_organ_10d` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/botanical_vertebrate_10d.rs:22` | computational geometry | Compiles a 4-scale botanical `.10d` organ container (`Scale 1` whole organism, `Scale 2` |
| `impl AssetHost<'_>::compile_vertebrate_anatomy_10d` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/botanical_vertebrate_10d.rs:152` | computational geometry | Compiles a Scale-5 comparative vertebrate 3-layer `.10d` anatomy container (`Osteology`, |
| `store_raw_mesh` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/botanical_vertebrate_10d.rs:459` | computational geometry |  |
| `resolve_corpus` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:24` | - |  |
| `load_corpus_bytes` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:84` | - |  |
| `vibe_record_to_json` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:92` | - |  |
| `is_radian` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:100` | - |  |
| `parse_rom_property` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:105` | semantics |  |
| `joint_rom_from_corpus` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:123` | - | Joint ranges taken from active corpus assertions. Each range cites its assertion. |
| `impl AssetHost<'_>::assemble_species_corpus` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:282` | - | `qualia.species.assemble_corpus`: validate a species document and seal it as `.q42`. |
| `conflict_summary` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/corpus_values.rs:334` | - |  |
| `impl HumanAnatomyProfile::from_str_and_week` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:49` | - |  |
| `impl HumanAnatomyProfile::slug` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:80` | - |  |
| `impl HumanAnatomyProfile::label` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:90` | - |  |
| `impl HumanWelfareMeasurements::compute` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:137` | - |  |
| `impl HumanWelfareMeasurements::to_json_string` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:259` | - |  |
| `build_human_welfare_anatomy_10d` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:344` | - | Builds a complete 3-layer / 17-system Human Welfare & Maternal-Fetal `.10d` container |
| `compile_human_welfare_q42_bytes` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:1273` | - | Compiles a deterministic 832-byte `.q42` (`Q42LOGIC`) welfare & maternal-fetal rule container |
| `apply_age_morphism` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:1353` | - | Years lived and how old the person looks are separate. |
| `build_authored_part` | 9 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:1401` | computational geometry | One Synth-authored part. The mesh is generated here. No external model is fetched. |
| `impl AssetHost<'_>::compile_human_welfare_anatomy_10d` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/human_welfare_anatomy.rs:1538` | - | Morphism `hypermedia.manifold.compile_human_welfare_anatomy_10d`: |
| `canonical_joint` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:45` | - |  |
| `impl Axis::parse` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:61` | - |  |
| `impl Axis::as_str` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:70` | - |  |
| `impl AngleRange::contains` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:87` | - |  |
| `impl AngleRange::encloses` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:91` | - |  |
| `impl AngleRange::clamp` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:95` | - |  |
| `impl JointRom::axis` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:132` | - |  |
| `impl AxisRom::active_range` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:139` | - | The range a pose may use: the narrowed range when injured, otherwise the healthy range. |
| `impl AxisRom::classify` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:147` | - |  |
| `parse_range` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:158` | - |  |
| `check_keys` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:173` | - |  |
| `parse_joint` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:180` | - |  |
| `axis_citation` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:288` | - |  |
| `parse_joint_rom` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:321` | - | Parses the spec's optional `joints` list. Returns an empty list when absent. |
| `push_range` | 7 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:342` | - |  |
| `joint_rom_rules` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/joint_rom.rs:361` | - | Compiles joint ranges into rules. Property IRIs are `q42:rom:<joint>:<axis>:<zone>{Neg,Pos}`, |
| `opt_string` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:45` | - |  |
| `opt_point3` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:52` | - |  |
| `run_sector_physics_verification_step` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:72` | physics | Runs a deterministic 3D XPBD + analytical SDF + 2-way continuum bridge + 4-wheel rover suspension |
| `impl AssetHost<'_>::living_world_anatomy_op` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:290` | physics, 3d, semantics | Categorical operations dispatch for Wave 12 Living-World anatomy, fauna rigging, |
| `parse_fauna_archetype` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:331` | - |  |
| `wave12_living_world_morphisms_roundtrip` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/mod.rs:366` | - |  |
| `build_anatomy_from_portable_scaffold` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:22` | - | Compiles a 3-layer `.10d` vertebrate anatomical twin from a species spec (VibeScript record |
| `emit_submesh_primitives` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:158` | - |  |
| `parse_layer` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:417` | - |  |
| `parse_joint` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:430` | - |  |
| `mirror_joint` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:464` | - |  |
| `parse_vec2` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:476` | 2d |  |
| `parse_vec3` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:484` | 3d |  |
| `parse_rgba_raw` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:496` | - |  |
| `resolve_color` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/portable_scaffold.rs:512` | - |  |
| `impl AssetHost<'_>::rig_biomechanical_archetype` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rig_ontology_continuum_frustum.rs:28` | - | Constructs and steps the 18-joint PGA `Motor` fauna skeletal rig (`FaunaSkeletalRig10d`), |
| `impl AssetHost<'_>::query_anatomical_ontology_pin` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rig_ontology_continuum_frustum.rs:195` | 3d, semantics | Queries the closest canonical anatomical ontology pin (`CANONICAL_ANATOMICAL_PINS`, `32` pins) |
| `impl AssetHost<'_>::step_sector_continuum_3d` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rig_ontology_continuum_frustum.rs:265` | - | Runs the 3D XPBD + analytical SDF + 2-way continuum bridge + 4-wheel rover suspension |
| `impl AssetHost<'_>::synchronize_companion_frustum` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rig_ontology_continuum_frustum.rs:303` | 3d | Generates a multi-projector conical dome frustum cluster (`rows * cols` `ConicalDomeWedge`s) |
| `impl RuleKind::parse` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:38` | - |  |
| `impl RuleKind::opcode` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:48` | - |  |
| `impl RuleKind::header_slot` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:57` | - |  |
| `impl RuleKind::defaults` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:66` | - |  |
| `impl RuleSpec::new` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:91` | - |  |
| `parse_claim_basis` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:106` | - |  |
| `basis_needs_assertion` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:117` | - |  |
| `validate_rule_citation` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:121` | - |  |
| `number` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:145` | - |  |
| `parse_lane` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:154` | - |  |
| `parse_rule` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:169` | - |  |
| `parse_id_list` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:225` | - |  |
| `parse_rules_value` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:243` | - | Parses a VibeScript list of rule records. |
| `archetype_default_rules` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:266` | - | The archetype-class default rules, identical to the legacy |
| `compile_rule_pack_q42` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:314` | - | Compiles rules into a `Q42LOGIC` container. Lamport clocks follow rule order, starting at 1. |
| `u32_at` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:364` | - |  |
| `u64_at` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:368` | - |  |
| `validate_q42_logic` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/rule_pack.rs:374` | - | Validates a `Q42LOGIC` container: magic, version, length, per-kind counts, payload digest and |
| `impl SpecSource::as_str` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:29` | - |  |
| `decode_cbor_ld` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:37` | - |  |
| `resolve_species_spec` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:50` | - | Returns the species spec from `spec` (or its legacy alias `scaffold`) and where it came from. |
| `species_model_id` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:86` | - | The model id for rule packs: the spec's `species_id`, otherwise `fallback`. |
| `impl RulesSource::as_str` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:103` | - |  |
| `resolve_rules` | 5 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:120` | - | Resolves the `rules` argument into a validated `Q42LOGIC` container, appending `derived` |
| `vibe_value_to_json` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec.rs:174` | - |  |
| `rec` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:9` | - |  |
| `s` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:18` | - |  |
| `v3` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:22` | - |  |
| `test_spec` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:26` | - |  |
| `rule` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:83` | - |  |
| `run` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:91` | - |  |
| `rule_pack_reproduces_legacy_hardcoded_packs` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:106` | - |  |
| `rule_pack_validation_rejects_corruption` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:124` | - |  |
| `rules_value_parsing_fails_closed` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:144` | - |  |
| `compile_vertebrate_anatomy_10d_is_stacked_ellipsoids_not_a_clinic_heart` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:170` | computational geometry | `compile_vertebrate_anatomy_10d` is a stacked-ellipsoid / primitive scaffold |
| `vertebrate_op_compiles_vibescript_spec_with_rules` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:208` | - |  |
| `vertebrate_op_default_rules_use_spec_species_id` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:227` | - |  |
| `vertebrate_op_accepts_validated_q42_input` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:247` | - |  |
| `vertebrate_op_rejects_non_vibescript_specs` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:273` | - |  |
| `with_field` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:292` | - |  |
| `range` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:300` | - |  |
| `ankle_joint` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:304` | - |  |
| `joints_json` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:328` | - |  |
| `joint_rom_parses_classifies_and_compiles` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:333` | - |  |
| `joint_rom_fails_closed` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:362` | - |  |
| `vertebrate_op_appends_joint_rules` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:394` | - |  |
| `vertebrate_op_accepts_cbor_ld_spec_input` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:414` | - |  |
| `range_assertion` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:443` | - |  |
| `corpus_joint_ranges_cite_assertion_ids_and_unsourced_rules_stay_assumptions` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:473` | - |  |
| `measured_joint_without_an_assertion_id_is_rejected` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/species_spec_tests.rs:559` | - |  |
| `render_hero_rgba_frame` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:16` | - | Renders a 256x256 RGBA studio hero frame suitable for downstream Radiance / CDL / MaterialX morphisms. |
| `render_animated_cinema_frames` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:27` | 3d | Renders an N-frame animated 3D cinema sequence (`160x112` H.264 macroblock-aligned) combining |
| `render_cinema_rgba_frame_at` | 10 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:58` | - |  |
| `render_turnaround_sheet_png` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:171` | - | Renders a 3-panel (`900x420`) RGBA8 turnaround sheet PNG (`Side Profile`, `3/4 Hero`, `Front`). |
| `render_panel` | 15 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:286` | - |  |
| `encode_rgba_png` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:480` | - |  |
| `render_anatomical_blueprint_svg` | 5 | `crates/asset-production-vibe/src/living_world_anatomy/turntable_render.rs:497` | computational geometry | Renders a vector orthographic & anatomical callout blueprint SVG from a 3D mesh. |
| `impl WhitegumMeshBuilder::new` | 0 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:37` | - |  |
| `impl WhitegumMeshBuilder::begin_layer` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:51` | - |  |
| `impl WhitegumMeshBuilder::end_layer` | 1 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:61` | - |  |
| `impl WhitegumMeshBuilder::add_vertex` | 5 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:77` | 3d |  |
| `impl WhitegumMeshBuilder::add_triangle` | 4 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:87` | - |  |
| `impl WhitegumMeshBuilder::add_quad` | 5 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:93` | - |  |
| `impl WhitegumMeshBuilder::add_curved_tube` | 6 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:99` | - | Add a continuous, welded multi-station curved tube with parallel-transported ring frames. |
| `impl WhitegumMeshBuilder::add_foliage_lobe` | 8 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:205` | - | Add an ellipsoid or sculpted foliage cluster lobe with organic surface modulation. |
| `cubic_bezier` | 5 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:272` | - | Evaluates a 4-point cubic Bezier curve at parameter `t` in `[0, 1]`. |
| `lerp_rgba` | 3 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:285` | - |  |
| `build_whitegum_whole_organism_10d` | 2 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:299` | - | Builds a high-craft Scale-1 Whole-Organism Whitegum / Ghost Gum (`Corymbia aparrerinja`) |
| `seal_whitegum_organ_10d` | 8 | `crates/asset-production-vibe/src/living_world_anatomy/whitegum_canopy.rs:882` | - |  |
| `dist2` | 2 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:10` | - |  |
| `cubic_eval` | 5 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:16` | - |  |
| `push_cubic` | 6 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:26` | - |  |
| `print_arc_segments` | 3 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:45` | - | Print-density sample count for a circular sweep; `min_seg` is the origin floor. |
| `impl AssetHost<'_>::locus_fillet` | 2 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:56` | computational geometry, numerics | Corner filleting on a 2D polyline or closed polygon. |
| `impl AssetHost<'_>::locus_resample` | 2 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:247` | - | Resample a 2D locus by uniform arc length count, fixed step length, or curvature weighting. |
| `impl AssetHost<'_>::locus_trim` | 2 | `crates/asset-production-vibe/src/locus_generative/fillet_resample_trim.rs:459` | numerics | Extract a sub-curve from normalized arc length [start, end] with exact linear interpolation. |
| `impl AssetHost<'_>::locus_generative_op` | 3 | `crates/asset-production-vibe/src/locus_generative/mod.rs:14` | - | Dispatch morphism operations for generative curve and geodesic path workflows. |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/locus_generative/mod.rs:44` | - |  |
| `store_test_profile` | 2 | `crates/asset-production-vibe/src/locus_generative/mod.rs:58` | - |  |
| `store_test_mesh` | 1 | `crates/asset-production-vibe/src/locus_generative/mod.rs:64` | computational geometry |  |
| `test_locus_fillet_square` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:86` | - |  |
| `test_locus_fillet_chamfer` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:120` | - |  |
| `test_locus_resample_count` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:149` | - |  |
| `test_locus_resample_length_and_curvature` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:177` | - |  |
| `test_locus_trim` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:218` | - |  |
| `test_locus_bishop_rmf_cyclic` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:246` | - |  |
| `test_locus_shortest_edge_path` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:286` | graph |  |
| `test_unknown_op` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:316` | - |  |
| `polyline_svg_d` | 1 | `crates/asset-production-vibe/src/locus_generative/mod.rs:324` | - |  |
| `svg_num` | 1 | `crates/asset-production-vibe/src/locus_generative/mod.rs:326` | - |  |
| `test_locus_fillet_g2_print_density_svg_path_roundtrip` | 0 | `crates/asset-production-vibe/src/locus_generative/mod.rs:344` | - |  |
| `impl Ord for DijkstraItem::cmp` | 2 | `crates/asset-production-vibe/src/locus_generative/rmf_geodesic.rs:23` | - |  |
| `impl PartialOrd for DijkstraItem::partial_cmp` | 2 | `crates/asset-production-vibe/src/locus_generative/rmf_geodesic.rs:34` | - |  |
| `impl AssetHost<'_>::locus_bishop_rmf_cyclic` | 2 | `crates/asset-production-vibe/src/locus_generative/rmf_geodesic.rs:41` | - | Cyclic double-reflection Bishop rotation-minimizing frame (RMF) generator along a 3D path. |
| `impl AssetHost<'_>::locus_shortest_edge_path` | 2 | `crates/asset-production-vibe/src/locus_generative/rmf_geodesic.rs:135` | graph | Shortest edge path across a 3D triangle manifold using deterministic Dijkstra search. |
| `impl AssetHost<'_>::locus_ext` | 3 | `crates/asset-production-vibe/src/locus_ops.rs:7` | - |  |
| `impl AssetHost<'_>::locus_spline` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:19` | - |  |
| `impl AssetHost<'_>::locus_offset` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:71` | - |  |
| `impl AssetHost<'_>::locus_boolean` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:138` | - |  |
| `impl AssetHost<'_>::locus_stroke_to_locus` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:207` | - |  |
| `impl AssetHost<'_>::store_locus_profile` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:228` | - |  |
| `parse_locus_points` | 3 | `crates/asset-production-vibe/src/locus_ops.rs:249` | 2d |  |
| `to_clean_points` | 1 | `crates/asset-production-vibe/src/locus_ops.rs:265` | - |  |
| `offset_polygon` | 3 | `crates/asset-production-vibe/src/locus_ops.rs:289` | computational geometry, 2d |  |
| `stroke_open` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:368` | 2d |  |
| `stroke_closed` | 2 | `crates/asset-production-vibe/src/locus_ops.rs:479` | 2d |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:11` | - |  |
| `snap0` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:18` | - |  |
| `svg_num` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:27` | - | Fixed-point SVG number; kills signed zero so stills are bit-stable. |
| `polyline_svg_d` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:33` | - |  |
| `segment_is_ruled` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:52` | numerics | Ruled/linear cubics stay named two-point spans (no print-density tessellation). |
| `path_svg_d` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:69` | - |  |
| `print_cubic_n` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:108` | - |  |
| `densify_print` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:118` | - |  |
| `push_pt` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:140` | - |  |
| `point_on_span` | 3 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:151` | - |  |
| `collapse_collinear` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:164` | numerics | Drop interior samples on ruled/linear spans so they stay named two-point profiles. |
| `profile_from_path` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:195` | - |  |
| `impl AssetHost<'_>::solve_parametric_sketch` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:230` | numerics | 120: `hypermedia.locus.solve_parametric_sketch` (`locus-parametric-solver` + `drafting-geometry`) |
| `impl AssetHost<'_>::pathfinder_warp_envelope` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:368` | - | 121: `hypermedia.locus.pathfinder_warp_envelope` (`vector-locus-pathops` + `vector-locus-geometry`) |
| `impl AssetHost<'_>::transcode_geospatial_vector` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:483` | - | 122: `hypermedia.locus.transcode_geospatial_vector` (`locus-geospatial-codecs`) |
| `locus_manifold_domain_composable_artifacts_and_handles` | 0 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:546` | - |  |
| `svg_bytes` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:648` | - |  |
| `svg_path_d` | 1 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:656` | - |  |
| `make_host` | 2 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:663` | - |  |
| `locus_sketch_svg_path_roundtrip_bit_stable` | 0 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:674` | - |  |
| `locus_warp_print_density_svg_path_roundtrip` | 0 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:703` | - |  |
| `locus_warp_live_corners_90deg_setback_not_l_vs_c` | 0 | `crates/asset-production-vibe/src/locus_sketch_warp_geospatial.rs:768` | - |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:6` | - |  |
| `impl AssetHost<'_>::sweep_assembly_profile` | 2 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:16` | - | 123: `hypermedia.manifold.sweep_assembly_profile` |
| `impl AssetHost<'_>::compile_bim_ifc4` | 2 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:360` | - | 124: `hypermedia.manifold.compile_bim_ifc4` |
| `impl AssetHost<'_>::erode_geomorphic_heightfield` | 2 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:476` | - | 125: `hypermedia.manifold.erode_geomorphic_heightfield` |
| `catmull_rom_axis` | 5 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:605` | - |  |
| `densify_axis_cubic` | 2 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:619` | - |  |
| `step_real` | 1 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:657` | - |  |
| `ifc4_roundtrip_wall` | 1 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:665` | - |  |
| `make_test_host` | 2 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:701` | - |  |
| `vec3` | 3 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:714` | 3d |  |
| `test_manifold_sweep_assembly_linear_and_cubic` | 0 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:719` | numerics |  |
| `test_manifold_sweep_cubic_densify_caps_at_4096` | 0 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:773` | - |  |
| `test_manifold_compile_bim_ifc4_roundtrip` | 0 | `crates/asset-production-vibe/src/manifold_assembly_bim_erosion.rs:819` | - |  |
| `impl AssetHost<'_>::manifold_evaluate_field` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/fields_instances.rs:12` | - |  |
| `impl AssetHost<'_>::manifold_instance_on_points` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/fields_instances.rs:267` | - |  |
| `impl AssetHost<'_>::manifold_realize_instances` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/fields_instances.rs:398` | - |  |
| `p_sub` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:28` | - |  |
| `p_add` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:33` | - |  |
| `p_scale` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:38` | - |  |
| `p_dot` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:43` | - |  |
| `p_cross` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:48` | - |  |
| `p_len_sq` | 1 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:57` | - |  |
| `p_normalize` | 1 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:62` | - |  |
| `closest_point_on_triangle` | 4 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:74` | computational geometry | Computes the exact closest point on a 3D triangle to a query point P |
| `ray_intersects_triangle` | 5 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:129` | computational geometry, 3d | Möller-Trumbore ray-triangle intersection test returning true if ray (origin, dir) intersects. |
| `impl AssetHost<'_>::manifold_fields_sdf_op` | 3 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:166` | - | Dispatch entrypoint for Wave 7 Manifold Lazy Fields, SDF Bricks, Instancing & Botany. |
| `impl AssetHost<'_>::store_mesh_positions` | 3 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:197` | computational geometry | Stores a 3D Mesh artifact validating budgets and finite bounds. |
| `impl AssetHost<'_>::manifold_mesh_to_sdf_brick` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:237` | computational geometry |  |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:361` | - |  |
| `store_test_cube` | 1 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:375` | - |  |
| `test_evaluate_field_lazy_chunking` | 0 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:405` | - |  |
| `test_mesh_to_sdf_brick_and_dual_contouring_roundtrip` | 0 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:439` | computational geometry |  |
| `test_instance_on_points_and_realize` | 0 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:506` | - |  |
| `test_fspm_vascular_grow_botany` | 0 | `crates/asset-production-vibe/src/manifold_fields_sdf/mod.rs:547` | - |  |
| `impl AssetHost<'_>::manifold_sdf_brick_to_mesh` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/sdf_botany.rs:16` | computational geometry |  |
| `impl AssetHost<'_>::manifold_fspm_vascular_grow` | 2 | `crates/asset-production-vibe/src/manifold_fields_sdf/sdf_botany.rs:278` | - |  |
| `impl AssetHost<'_>::manifold_ext` | 3 | `crates/asset-production-vibe/src/manifold_ops.rs:17` | - |  |
| `impl AssetHost<'_>::manifold_boolean_3d` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:41` | - |  |
| `impl AssetHost<'_>::manifold_simplify` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:107` | - |  |
| `impl AssetHost<'_>::simplify_mesh_buffers` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:134` | computational geometry |  |
| `impl AssetHost<'_>::manifold_simplify_decimate` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:147` | - |  |
| `impl AssetHost<'_>::manifold_simplify_remesh` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:187` | - |  |
| `impl AssetHost<'_>::manifold_simplify_anisotropic` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:231` | - |  |
| `impl AssetHost<'_>::manifold_import_mesh` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:267` | computational geometry |  |
| `impl AssetHost<'_>::manifold_project_dxf` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:306` | - |  |
| `impl AssetHost<'_>::manifold_parse_dxf` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:416` | - |  |
| `impl AssetHost<'_>::manifold_loft` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:447` | - |  |
| `impl AssetHost<'_>::manifold_compile_10d_pbr` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:546` | - |  |
| `read_uv01_atlas` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:833` | - |  |
| `decode_source_10d_vertex_colors` | 1 | `crates/asset-production-vibe/src/manifold_ops.rs:847` | 3d |  |
| `transfer_nearest_vertex_colors` | 3 | `crates/asset-production-vibe/src/manifold_ops.rs:865` | 3d |  |
| `loft_points3` | 1 | `crates/asset-production-vibe/src/manifold_ops.rs:889` | - |  |
| `catmull_rom_scalar` | 5 | `crates/asset-production-vibe/src/manifold_ops.rs:906` | - |  |
| `catmull_rom_point3` | 5 | `crates/asset-production-vibe/src/manifold_ops.rs:915` | - |  |
| `mean_triangle_edge_length` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:923` | graph |  |
| `densify_loft_rings_cubic` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:945` | numerics | Cubic Catmull-Rom densify of loft stations. Linear/ruled skinning is applied after. |
| `densify_loft_rings_nurbs` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:979` | - | NURBS densify of loft stations via `parametric_cad::nurbs_eval` (unit weights). |
| `parse_ascii_dxf` | 1 | `crates/asset-production-vibe/src/manifold_ops.rs:1023` | - |  |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/manifold_ops.rs:1165` | - |  |
| `vec3` | 3 | `crates/asset-production-vibe/src/manifold_ops.rs:1179` | 3d |  |
| `unit_cube_mesh` | 1 | `crates/asset-production-vibe/src/manifold_ops.rs:1183` | computational geometry |  |
| `translated_cube` | 4 | `crates/asset-production-vibe/src/manifold_ops.rs:1215` | - |  |
| `square_ring` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:1247` | - |  |
| `test_manifold_loft_linear_remains_ruled_cubic_bends` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1257` | numerics |  |
| `test_manifold_boolean_3d_production_union_difference` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1332` | - |  |
| `test_manifold_project_dxf_mesh_roundtrip` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1380` | computational geometry |  |
| `unit_icosphere_mesh` | 1 | `crates/asset-production-vibe/src/manifold_ops.rs:1428` | computational geometry |  |
| `mesh_golden_checksum` | 2 | `crates/asset-production-vibe/src/manifold_ops.rs:1454` | computational geometry |  |
| `test_manifold_simplify_decimate_default_qem` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1478` | - |  |
| `test_manifold_simplify_remesh_and_anisotropic_small_mesh` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1514` | computational geometry |  |
| `test_manifold_boolean_3d_golden_checksum` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1569` | - |  |
| `test_manifold_loft_nurbs_optional_keeps_linear_cubic` | 0 | `crates/asset-production-vibe/src/manifold_ops.rs:1616` | numerics |  |
| `v_add` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:17` | - |  |
| `v_sub` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:22` | - |  |
| `v_scale` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:27` | - |  |
| `v_dot` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:32` | - |  |
| `v_cross` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:37` | - |  |
| `v_len_sq` | 1 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:46` | - |  |
| `v_normalize` | 1 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:51` | - |  |
| `compute_smooth_normals` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:62` | computational geometry, 3d | Compute area-weighted smooth unit vertex normals for a triangle mesh. |
| `impl AssetHost<'_>::mesh` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:94` | computational geometry | Retrieve a reference to a `Mesh` artifact from a production handle. |
| `impl AssetHost<'_>::store_mesh_artifact` | 3 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:102` | computational geometry | Store a new `Mesh` artifact after bounding coordinates and computing bounding box. |
| `impl AssetHost<'_>::manifold_topology_op` | 3 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:139` | - | Dispatch manifold topology morphisms. |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:183` | - |  |
| `store_tetrahedron` | 1 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:197` | - |  |
| `store_quad_sheet` | 1 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:209` | - |  |
| `test_radial_euler_default_refinement_and_m64_serialization` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:222` | physics |  |
| `test_radial_euler_e64_magic_alias_round_trip` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:275` | physics |  |
| `test_radial_euler_explicit_ops` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:322` | physics |  |
| `cycle_eq_rotated` | 2 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:352` | - |  |
| `run_jfke_tet_edge02` | 1 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:365` | - |  |
| `test_radial_euler_jfke_tetrahedron_quad_cycle_and_m64_shell` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:379` | physics |  |
| `test_radial_euler_jfke_fan_diagonals_not_edge_zero` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:445` | physics, graph |  |
| `test_radial_euler_jfke_fails_closed_on_boundary_edge` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:502` | physics, graph |  |
| `test_manifold_bevel` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:525` | - |  |
| `test_manifold_inset` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:547` | - |  |
| `test_manifold_solidify` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:569` | - |  |
| `test_manifold_bisect_plane` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:591` | - |  |
| `test_manifold_subdivide_catmull_clark` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:622` | - |  |
| `test_manifold_subdivide_catmull_clark_produce_safe_and_rounds` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:653` | - |  |
| `test_manifold_subdivide_catmull_clark_rejects_oob_triangle` | 0 | `crates/asset-production-vibe/src/manifold_topology/mod.rs:716` | - |  |
| `undirected_edge_slot` | 3 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:28` | graph |  |
| `directed_edge_slot` | 3 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:37` | graph |  |
| `cycle_interior` | 3 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:43` | - | Vertices strictly between `from` and `to` walking the cycle forward. |
| `splice_face_cycles` | 4 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:66` | graph | Join two face cycles at the shared oriented edge (v0, v1). |
| `impl AssetHost<'_>::manifold_radial_euler` | 2 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:98` | physics |  |
| `impl AssetHost<'_>::manifold_bevel` | 2 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:452` | - |  |
| `impl AssetHost<'_>::manifold_inset` | 2 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:597` | - |  |
| `cycle_eq_rotated` | 2 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:675` | - |  |
| `test_jfke_tetrahedron_splice_is_quad_not_set_union` | 0 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:689` | - |  |
| `test_jfke_fails_closed_when_vertices_not_adjacent` | 0 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:703` | - |  |
| `test_jfke_fails_closed_when_cycles_share_same_orientation` | 0 | `crates/asset-production-vibe/src/manifold_topology/radial_bevel_inset.rs:712` | - |  |
| `impl AssetHost<'_>::manifold_solidify` | 2 | `crates/asset-production-vibe/src/manifold_topology/solidify_bisect_subdivide.rs:13` | - |  |
| `impl AssetHost<'_>::manifold_bisect_plane` | 2 | `crates/asset-production-vibe/src/manifold_topology/solidify_bisect_subdivide.rs:83` | - |  |
| `impl AssetHost<'_>::manifold_subdivide_catmull_clark` | 2 | `crates/asset-production-vibe/src/manifold_topology/solidify_bisect_subdivide.rs:296` | - |  |
| `resolve_morphism` | 1 | `crates/asset-production-vibe/src/ontology/mod.rs:240` | - | Resolves any canonical categorical ID or historical `hypermedia.*`/`synth.*` alias |
| `canonical_morphism_id` | 1 | `crates/asset-production-vibe/src/ontology/mod.rs:247` | - | Resolves any operation identifier to its canonical categorical ID (`hypermedia.<category>.<morphism>`). |
| `hex_bytes` | 1 | `crates/asset-production-vibe/src/ontology/mod.rs:251` | - |  |
| `categories_json` | 0 | `crates/asset-production-vibe/src/ontology/mod.rs:261` | semantics | Serializes the category-theoretic ontology taxonomy for agentic discovery (`synth.ontology.categories`). |
| `morphism_to_json` | 1 | `crates/asset-production-vibe/src/ontology/mod.rs:284` | - | Serializes a single `AgenticMorphismSpec` including its `OntologicalContract` and `NQuin48` parity hashes. |
| `morphisms_json` | 1 | `crates/asset-production-vibe/src/ontology/mod.rs:346` | semantics | Serializes morphisms filtered by optional `category` (`synth.ontology.morphisms`). |
| `all_ontology_nquins_verify_five_field_parity_and_spans` | 0 | `crates/asset-production-vibe/src/ontology/tests.rs:5` | semantics |  |
| `zero_craft_names_in_canonical_categories_or_morphism_ids` | 0 | `crates/asset-production-vibe/src/ontology/tests.rs:32` | - |  |
| `impl VibeNamespaceProvider for HypermediaOntologyProvider::descriptor` | 1 | `crates/asset-production-vibe/src/ontology/vibe_descriptors.rs:259` | - |  |
| `impl VibeNamespaceProvider for HypermediaOntologyProvider::static_ontology_nquins` | 1 | `crates/asset-production-vibe/src/ontology/vibe_descriptors.rs:263` | semantics |  |
| `impl VibeNamespaceProvider for HypermediaOntologyProvider::call` | 4 | `crates/asset-production-vibe/src/ontology/vibe_descriptors.rs:267` | - |  |
| `host` | 2 | `crates/asset-production-vibe/src/operations_catalog.rs:312` | - |  |
| `vec3` | 3 | `crates/asset-production-vibe/src/operations_catalog.rs:322` | 3d |  |
| `square_ring` | 2 | `crates/asset-production-vibe/src/operations_catalog.rs:326` | - |  |
| `catalog_text` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:335` | - |  |
| `catalog_rows_raise_wave_b_objects` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:344` | - |  |
| `catalog_dispatch_csi_hits_csiframe` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:369` | - |  |
| `catalog_dispatch_cubic_loft_not_interpolation_rejected` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:400` | - |  |
| `catalog_dispatch_nurbs_loft_and_revolve_optional` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:462` | - |  |
| `catalog_compile_10d_pbr_consumes_unwrap_uv01` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:569` | - |  |
| `catalog_compile_10d_pbr_albedo_samples_unwrap_uv01` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:638` | - |  |
| `catalog_compile_10d_pbr_auto_unwraps_albedo_without_source_10d` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:775` | - |  |
| `catalog_emf_spectral_png_is_non_black` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:845` | - |  |
| `catalog_compile_10d_pbr_source_10d_uv_mismatch_fails_closed` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:951` | - |  |
| `solid_rgba` | 7 | `crates/asset-production-vibe/src/operations_catalog.rs:1018` | - |  |
| `png_bytes` | 3 | `crates/asset-production-vibe/src/operations_catalog.rs:1043` | - |  |
| `u64_field` | 2 | `crates/asset-production-vibe/src/operations_catalog.rs:1052` | - |  |
| `sha256_hex` | 1 | `crates/asset-production-vibe/src/operations_catalog.rs:1060` | - |  |
| `rotr` | 2 | `crates/asset-production-vibe/src/operations_catalog.rs:1061` | - |  |
| `catalog_pack_sprite_sheet_frames_no_overlap` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1133` | 2d |  |
| `catalog_pack_nine_slice_insets_sidecar` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1196` | - |  |
| `catalog_pack_pbr_set_encoded_png` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1233` | - |  |
| `catalog_synthesize_maps_side_1024_envelope` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1265` | - |  |
| `catalog_pack_sprite_sheet_two_produce_sha256` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1300` | 2d |  |
| `catalog_pack_sprite_sheet_unknown_argument` | 0 | `crates/asset-production-vibe/src/operations_catalog.rs:1330` | 2d |  |
| `escape` | 1 | `crates/asset-production-vibe/src/preview.rs:4` | - |  |
| `impl AssetHost<'_>::gallery` | 2 | `crates/asset-production-vibe/src/preview.rs:12` | - |  |
| `warm_cinema_preset` | 1 | `crates/asset-production-vibe/src/radiance_color.rs:10` | - |  |
| `cool_teal_orange_preset` | 1 | `crates/asset-production-vibe/src/radiance_color.rs:21` | - |  |
| `monochrome_contrast_preset` | 1 | `crates/asset-production-vibe/src/radiance_color.rs:31` | - |  |
| `cyber_neon_preset` | 1 | `crates/asset-production-vibe/src/radiance_color.rs:38` | - |  |
| `parse_transfer` | 1 | `crates/asset-production-vibe/src/radiance_color.rs:45` | - |  |
| `impl AssetHost<'_>::radiance_color_op` | 3 | `crates/asset-production-vibe/src/radiance_color.rs:59` | - |  |
| `impl AssetHost<'_>::radiance_modify_mask` | 2 | `crates/asset-production-vibe/src/radiance_color.rs:75` | - |  |
| `impl AssetHost<'_>::radiance_apply_lut` | 2 | `crates/asset-production-vibe/src/radiance_color.rs:185` | - |  |
| `impl AssetHost<'_>::radiance_grade_color` | 2 | `crates/asset-production-vibe/src/radiance_color.rs:252` | - |  |
| `impl AssetHost<'_>::radiance_convert_transfer` | 2 | `crates/asset-production-vibe/src/radiance_color.rs:385` | - |  |
| `impl AssetHost<'_>::radiance_filter_kuwahara_anisotropic` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/filters_cryptomatte.rs:10` | graph | 1. Anisotropic Kuwahara structure-tensor edge-preserving filter |
| `impl AssetHost<'_>::radiance_decode_cryptomatte_q42` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/filters_cryptomatte.rs:234` | - | 2. Cryptomatte Q42 ID coverage matte extraction and isolated compositing |
| `impl AssetHost<'_>::radiance_vector_motion_blur` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/filters_cryptomatte.rs:311` | - | 3. Vector motion blur line-integral convolution with Hann shutter window |
| `cryptomatte_murmur3` | 1 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:15` | - |  |
| `smoothstep` | 3 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:54` | - |  |
| `impl AssetHost<'_>::radiance_compositor_gi_op` | 3 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:68` | - |  |
| `create_test_host` | 3 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:103` | - |  |
| `test_filter_kuwahara_anisotropic` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:118` | - |  |
| `test_decode_cryptomatte_q42` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:164` | - |  |
| `test_vector_motion_blur` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:214` | - |  |
| `test_solve_sdfgi_froxel` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:248` | numerics |  |
| `test_build_light_tree` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:294` | - |  |
| `test_error_handling` | 0 | `crates/asset-production-vibe/src/radiance_compositor_gi/mod.rs:360` | - |  |
| `impl LightTreeMeasure::from_light` | 1 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:42` | - |  |
| `impl LightTreeMeasure::merge` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:53` | - |  |
| `impl LightTreeNode::measure` | 1 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:156` | - |  |
| `impl LightTreeNode::node_and_leaf_count` | 1 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:163` | graph |  |
| `build_bvh_recursive` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:175` | - |  |
| `evaluate_tree_lighting` | 5 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:226` | 3d, graph |  |
| `impl AssetHost<'_>::radiance_solve_sdfgi_froxel` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:282` | numerics | 4. Cascaded SDFGI irradiance probes and 3D froxel atmospheric scattering |
| `impl AssetHost<'_>::radiance_build_light_tree` | 2 | `crates/asset-production-vibe/src/radiance_compositor_gi/sdfgi_light_tree.rs:492` | 3d | 5. Hierarchical light tree bounding-cone clustering and direct lighting preview |
| `impl AssetHost<'_>::develop_photo` | 2 | `crates/asset-production-vibe/src/radiance_photo/develop.rs:9` | - |  |
| `impl AssetHost<'_>::radiance_photo_op` | 3 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:10` | - |  |
| `parse_dimensions` | 1 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:20` | - |  |
| `store_u16be_as_u8` | 2 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:27` | - |  |
| `decode_psd_channel` | 5 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:38` | - |  |
| `decode_packbits` | 2 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:94` | - |  |
| `srgb_to_linear_f32` | 1 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:126` | numerics |  |
| `linear_to_srgb_f32` | 1 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:136` | numerics |  |
| `create_test_host` | 3 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:150` | - |  |
| `test_packbits_decode` | 0 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:165` | - |  |
| `test_srgb_roundtrip` | 0 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:174` | - |  |
| `test_psd_encode_import_roundtrip` | 0 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:185` | - |  |
| `test_develop_photo_default_identity` | 0 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:262` | - |  |
| `test_develop_photo_adjustments` | 0 | `crates/asset-production-vibe/src/radiance_photo/mod.rs:286` | - |  |
| `impl AssetHost<'_>::encode_psd` | 2 | `crates/asset-production-vibe/src/radiance_photo/psd_codec.rs:8` | - |  |
| `impl AssetHost<'_>::import_psd` | 2 | `crates/asset-production-vibe/src/radiance_photo/psd_codec.rs:193` | - |  |
| `impl AssetHost<'_>::develop_camera_raw_hdr` | 2 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:8` | 3d | 128: `hypermedia.radiance.develop_camera_raw_hdr` |
| `impl AssetHost<'_>::grade_cdl_scopes` | 2 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:124` | - | 129: `hypermedia.radiance.grade_cdl_scopes` |
| `impl AssetHost<'_>::trace_vector_contours` | 2 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:264` | - | 130: `hypermedia.radiance.trace_vector_contours` |
| `host` | 2 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:373` | - |  |
| `test_radiance_raw_hdr_nontrivial_pixel` | 0 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:384` | 2d |  |
| `test_radiance_cdl_scopes_nontrivial_pixel` | 0 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:414` | 2d |  |
| `test_radiance_raw_hdr_profile_refuses_outside_lattice` | 0 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:445` | - |  |
| `test_radiance_cdl_scopes_profile_admit_and_charge` | 0 | `crates/asset-production-vibe/src/radiance_raw_cdl_trace.rs:476` | - |  |
| `impl AssetHost<'_>::raster` | 3 | `crates/asset-production-vibe/src/raster.rs:8` | - |  |
| `impl AssetHost<'_>::image` | 2 | `crates/asset-production-vibe/src/raster.rs:246` | - |  |
| `impl AssetHost<'_>::render` | 2 | `crates/asset-production-vibe/src/raster.rs:252` | - |  |
| `pixels` | 3 | `crates/asset-production-vibe/src/raster.rs:333` | - |  |
| `rgb_view` | 1 | `crates/asset-production-vibe/src/raster.rs:339` | - |  |
| `impl AssetHost<'_>::optical_nm_to_audio_hz` | 2 | `crates/asset-production-vibe/src/sound.rs:7` | - | Heterodyne optical wavelength onto A4 (532 nm → 440 Hz) so the field is hearable. |
| `impl AssetHost<'_>::sonify_emf_field` | 5 | `crates/asset-production-vibe/src/sound.rs:16` | - | Field-derived mono PCM: irradiance envelope in sealed chunk/cell order (no HashMap). |
| `impl AssetHost<'_>::sound` | 3 | `crates/asset-production-vibe/src/sound.rs:90` | - |  |
| `host` | 2 | `crates/asset-production-vibe/src/sound.rs:221` | - |  |
| `compile_green_dipole` | 1 | `crates/asset-production-vibe/src/sound.rs:231` | - |  |
| `audio_peak` | 2 | `crates/asset-production-vibe/src/sound.rs:266` | - |  |
| `test_acoustics_bus_loudness_emf_sonic_objects` | 0 | `crates/asset-production-vibe/src/sound.rs:276` | - |  |
| `impl AssetHost<'_>::compile_bsdf_graph` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:14` | graph |  |
| `impl AssetHost<'_>::sample_procedural_field` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:320` | - |  |
| `hash2d` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:377` | - |  |
| `blackbody_rgb` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:391` | - |  |
| `wavelength_to_srgb` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:418` | - |  |
| `g` | 4 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:420` | - |  |
| `srgb_curve` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/bsdf_procedural.rs:439` | - |  |
| `v_sub` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:20` | - |  |
| `v_dot` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:25` | - |  |
| `v_cross` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:30` | - |  |
| `v_len` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:39` | - |  |
| `v_norm` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:44` | - |  |
| `compute_smooth_vertex_normals` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:54` | computational geometry, 3d | Compute smooth area-weighted per-vertex unit normals for a 3D mesh. |
| `build_rest_tris` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:88` | computational geometry |  |
| `eval_tri_sd` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:137` | - |  |
| `parse_vertex_uvs` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:163` | computational geometry, 3d | Vertex-aligned UVs from a list, unwrap record, or named profile (island outline |
| `uv_island_hull` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:202` | - | Monotone-chain hull of a SLIM UV island. Sorted, so iteration is produce-safe. |
| `cross` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:223` | - |  |
| `rasterize_uv_chart` | 4 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:251` | 2d | Rasterize UV triangles into a still map so a non-trivial island pixel is observable. |
| `seal_mesh_10d_with_slim_uvs` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:310` | computational geometry | Seal SLIM TEXCOORD_0 into a QualiaDB `.10d` so unwrap feeds `compile_10d_pbr`. |
| `impl AssetHost<'_>::surface_bsdf_uv_op` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:340` | - | Categorical operations dispatch for surface BSDF, UV, and procedural field synthesis. |
| `make_test_host` | 3 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:373` | - |  |
| `store_quad_mesh` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:387` | computational geometry |  |
| `store_tetrahedron_mesh` | 1 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:410` | computational geometry |  |
| `test_unwrap_slim_planar_quad` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:440` | - |  |
| `test_unwrap_slim_mode_slim_default_and_atlas` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:482` | - |  |
| `test_compute_mikktspace_tetrahedron` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:553` | - |  |
| `test_compile_bsdf_graph_all_closures` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:600` | graph |  |
| `test_sample_procedural_field_all_bases` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:655` | - |  |
| `still_has_nontrivial_pixel` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:680` | 2d |  |
| `test_unwrap_slim_uv_island_10d_roundtrip` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:685` | - |  |
| `test_bsdf_and_brush_nontrivial_preview_pixels` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:779` | - |  |
| `test_compile_bsdf_graph_ior_affects_preview` | 0 | `crates/asset-production-vibe/src/surface_bsdf_uv/mod.rs:820` | graph |  |
| `impl AssetHost<'_>::unwrap_slim` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/slim_mikktspace.rs:21` | - |  |
| `impl AssetHost<'_>::unwrap_slim_parameterize` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/slim_mikktspace.rs:34` | - |  |
| `impl AssetHost<'_>::unwrap_atlas_parameterize` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/slim_mikktspace.rs:340` | - |  |
| `impl AssetHost<'_>::compute_mikktspace` | 2 | `crates/asset-production-vibe/src/surface_bsdf_uv/slim_mikktspace.rs:494` | - |  |
| `opt_str` | 3 | `crates/asset-production-vibe/src/surface_pbr_brush.rs:5` | - |  |
| `impl AssetHost<'_>::bake_pbr_materialx` | 2 | `crates/asset-production-vibe/src/surface_pbr_brush.rs:15` | - | 126: `hypermedia.surface.bake_pbr_materialx` |
| `impl AssetHost<'_>::expand_procedural_brush` | 2 | `crates/asset-production-vibe/src/surface_pbr_brush.rs:157` | - | 127: `hypermedia.surface.expand_procedural_brush` |
