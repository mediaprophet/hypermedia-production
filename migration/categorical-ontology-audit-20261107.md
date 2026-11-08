# Categorical Ontology & Morphism Audit (2026-11-08)

## 1. Architectural Overview

`crates/asset-production-vibe` consolidates general-purpose asset-production capabilities across **Etalon** (`etalon-signal`, `etalon-render`, `etalon-sim`, `etalon-vibe`), **QualiaDB** (`qualia-core-db`, `qualia-audio`, `vibescript-abi`), and **Hypermedia** (49 domain crates across `C:\Projects\hypermedia-production\crates`) into a single category-theoretic ontology consumed by **Synth** (`C:\Projects\synth`) and ready for **Poet** (`C:\Projects\poet`) without invading Poet's application layer.

All legacy `-craft` and product-derivative names (`photocraft`, `vectorcraft`, `filmcraft`, `cadcraft`, `soundcraft`, `effectcraft`, `lightcraft`, `figcraft`, `wordcraft`, `deckcraft`, `gridcraft`, `bimcraft`, `clothcraft`, `fieldcraft`, `simcraft`, `materialcraft`, `substancecraft`, `matcraft`, `warraqcraft`, `subcraft`, `procraft`, `ledgercraft`, `plancraft`, `projectscraft`, `earthcraft`, `nerthus`, `rustgis`) are retired from canonical operation identifiers and replaced by **12 mathematical categories** of **Objects** and **Morphisms** ($f: \text{Domain} \to \text{Codomain}$), incorporating full Blender 4.x/5.x, Godot 4.x, and all 35 audited source repositories across Waves 0–13.

All provenance, cryptographic verification, and document compliance operations are grounded in international human rights instruments (**UDHR Articles 12, 18, 19, 27**; **ICCPR Articles 17, 19**; **ICESCR Article 15**) and **Informatics Fiduciary** duties.

Every morphism carries:
1. **`vibescript_abi::descriptor::OntologicalContract`** with 3 static 5-field XOR parity-verified `NQuin48` triples (`hasPrecondition`, `hasPostcondition`, `preservesInvariant`) — **144 morphisms × 3 = 432 `NQuin48` triples**.
2. **`AgenticMorphismSpec` (`AgenticUtility`)** providing `domain`, `codomain`, `morphism_kind`, `teleology`, `units_and_frames`, `composable_with`, and `human_preview_affordance`.
3. **Backward-Compatible Alias Resolution** via `resolve_morphism` and `canonical_morphism_id`.

---

## 2. The 12 Canonical Categories (144 Morphisms / 432 `NQuin48` Triples)

| Sub-NS ID | Category (`hypermedia.<cat>`) | Mathematical Object Space | Replaces Legacy Terms | Morphisms |
|---:|---|---|---|---:|
| `1` | `locus` | $\mathcal{L}_2$: 1D/2D parametric curves, B-splines, offset contours, fillets, cyclic Bishop RMFs, parametric sketch solvers, warp envelopes, and geospatial vector codecs | `vectorcraft`, `cadcraft-2d`, `profile` | 15 |
| `2` | `manifold` | $\mathcal{M}_3$: 2-manifolds, Radial-Edge B-Reps, SubD, lazy fields, SDF bricks, FSPM botany, vertebrate/fauna anatomy, swept assemblies, BIM IFC4/CSG, geomorphic erosion, and `.10d` spatial/PBR bundles | `cadcraft-3d`, `meshcraft`, `bimcraft` | 27 |
| `3` | `surface` | $\mathcal{S}_{\text{PBR}}$: Tangential material sheaves, SLIM UV charts, MikkTSpace tangents, Cycles BSDF closures, Gabor/Voronoi fields, MaterialX 1.39 baking, and procedural bristle/mixer brushes | `materialcraft`, `substancecraft`, `shader` | 7 |
| `4` | `radiance` | $\mathcal{R}_2$: 2D colorimetric & alpha-composited lattices, masks, 3D LUTs, PSDs, Kuwahara, Cryptomatte, SDFGI/Froxel, Light Trees, frustum visibility, Camera RAW HDR fusion, ASC CDL/scopes, and vector contour tracing | `photocraft`, `lightcraft`, `raster`, `image` | 29 |
| `5` | `glyph` | $\mathcal{G}$: Typographic glyph outlines, UAX #9 BiDi/HarfBuzz shaped text runs, packed MSDF/vector text projections, and Knuth-Plass paragraph composition | `typecraft`, `font` | 7 |
| `6` | `layout` | $\mathcal{B}_2$: Hierarchical spatial box trees, scoped Webizen UI tokens, 3D diegetic viewports, 4-stage fiduciary input routing, Flexbox/CSS Grid solvers, paged foliation/CSL, and Merkle CSE node-graphs | `figcraft`, `uicraft`, `deckcraft` | 9 |
| `7` | `kinetics` | $\mathcal{K}_t$: PGA MotorFTI, two-bone IK, Island XPBD, dual-BVH `DirectSpaceState`, 3D NavMesh Funnel+ORCA, NLA, Lottie, continuum FEM/LBM, orthotropic VBD cloth draping, causal ODE/DAE block diagrams, and CPM/EVM/Monte-Carlo schedules | `effectcraft`, `clothcraft`, `fieldcraft`, `simcraft`, `plancraft` | 15 |
| `8` | `acoustics` | $\mathcal{A}_t$: 1D pressure-wave buffers, RBJ biquad IIR filters, 32-bus `AudioServer` DAGs, PGA spatialization, WAV/AAC bitstreams, EBU R128 loudness, YIN pitch, 5.1/7.1.4 VBAP, Camelot harmony, and SMPTE/musical timebase | `soundcraft`, `mixstudio`, `pro-wave` | 9 |
| `9` | `electromagnetics` | $\mathcal{E}_5$: 5D wave-optical & electromagnetic field volumes (`.emf` / `Q42EMF64`), Amanatides-Woo DDA raymarches, 38D metamer null-spaces, and camera-free Wi-Fi CSI vital-sign/MUSIC AoA sensing | `emcraft`, `rfcraft`, `ruview`, `optics` | 6 |
| `10` | `geodesy` | $\mathcal{T}_{\oplus}$: Planetary heightfields, `.m64` (`m64\0`) subsurface strata, Poisson/MacCormack advection, Bekker/Mohr-Coulomb coupling, and WGS84 Vincenty/Ordinary Kriging/DEM morphometry | `terraincraft`, `earthcraft`, `rustgis` | 6 |
| `11` | `discourse` | $\mathcal{D}$: Foliated documents (`.pdf`), prose (`.docx`), slide decks (`.pptx`), charts (`.svg`), galleries (`.html`), telemetry, fiduciary RIDs, ontology pins, Academic TeX, PDF/UA-1 + WCAG 2.1 AAA + zero-residue PII redaction, and M-language scientific matrices | `wordcraft`, `warraqcraft`, `pdfcraft`, `matcraft` | 9 |
| `12` | `chronicle` | $\mathcal{C}_t$: Multiplexed audiovisual time-streams, deterministic `ManifoldClock` `.mp4` capture, 60Hz spatial motor packets, tabular DAG workbooks, double-entry fiduciary ledgers, OpenXML/OTIO/Matroska archives, and Apple ProRes 422 frames | `filmcraft`, `gridcraft`, `ledgercraft`, `subcraft` | 5 |

---

## 3. Complete Morphism Inventory (144 Admitted Morphisms)

### Category 1: `locus` (15 Morphisms)
- `hypermedia.locus.construct` (`hypermedia/profile-authoring`)
- `hypermedia.locus.transform` (`hypermedia/drafting-geometry`)
- `hypermedia.locus.project_svg` (`hypermedia/drafting-geometry + shared SVG serialization`)
- `hypermedia.locus.spline` (`hypermedia/drafting-geometry`)
- `hypermedia.locus.offset` (`hypermedia/drafting-geometry`)
- `hypermedia.locus.boolean` (`QualiaDB/computational_geometry/dcel_overlay`)
- `hypermedia.locus.stroke_to_locus` (`hypermedia/drafting-geometry`)
- `hypermedia.locus.fillet` (`hypermedia/asset-production-vibe/locus_generative`)
- `hypermedia.locus.resample` (`hypermedia/asset-production-vibe/locus_generative`)
- `hypermedia.locus.trim` (`hypermedia/asset-production-vibe/locus_generative`)
- `hypermedia.locus.bishop_rmf_cyclic` (`QualiaDB/computational_geometry/sweep_extrusion`)
- `hypermedia.locus.shortest_edge_path` (`hypermedia/asset-production-vibe/locus_generative`)
- `hypermedia.locus.solve_parametric_sketch` (`hypermedia/locus-parametric-solver + drafting-geometry`)
- `hypermedia.locus.pathfinder_warp_envelope` (`hypermedia/vector-locus-pathops + vector-locus-geometry`)
- `hypermedia.locus.transcode_geospatial_vector` (`hypermedia/locus-geospatial-codecs`)

### Category 2: `manifold` (27 Morphisms)
- `hypermedia.manifold.extrude` (`QualiaDB/computational_geometry`)
- `hypermedia.manifold.revolve` (`QualiaDB/parametric_cad`)
- `hypermedia.manifold.loft` (`QualiaDB/parametric_cad`)
- `hypermedia.manifold.compile_10d` (`QualiaDB/render/compile_10d`)
- `hypermedia.manifold.boolean_3d` (`QualiaDB/computational_geometry/boolean_3`)
- `hypermedia.manifold.simplify` (`QualiaDB/computational_geometry/decimate_qem`)
- `hypermedia.manifold.import_mesh` (`QualiaDB/render/assets/import_asset_with_normals`)
- `hypermedia.manifold.project_dxf` (`hypermedia/asset-production-vibe/manifold_ops`)
- `hypermedia.manifold.compile_10d_pbr` (`QualiaDB/render/compile_10d`)
- `hypermedia.manifold.radial_euler` (`QualiaDB/q42/m64_matter + manifold_topology`)
- `hypermedia.manifold.bevel` (`hypermedia/asset-production-vibe/manifold_topology`)
- `hypermedia.manifold.inset` (`hypermedia/asset-production-vibe/manifold_topology`)
- `hypermedia.manifold.solidify` (`hypermedia/asset-production-vibe/manifold_topology`)
- `hypermedia.manifold.bisect_plane` (`hypermedia/asset-production-vibe/manifold_topology`)
- `hypermedia.manifold.subdivide_catmull_clark` (`hypermedia/asset-production-vibe/manifold_topology`)
- `hypermedia.manifold.evaluate_field` (`hypermedia/asset-production-vibe/manifold_fields_sdf`)
- `hypermedia.manifold.mesh_to_sdf_brick` (`hypermedia/asset-production-vibe/manifold_fields_sdf`)
- `hypermedia.manifold.sdf_brick_to_mesh` (`hypermedia/asset-production-vibe/manifold_fields_sdf`)
- `hypermedia.manifold.instance_on_points` (`hypermedia/asset-production-vibe/manifold_fields_sdf`)
- `hypermedia.manifold.realize_instances` (`hypermedia/asset-production-vibe/manifold_fields_sdf`)
- `hypermedia.manifold.fspm_vascular_grow` (`QualiaDB/modalities/botany + manifold_fields_sdf`)
- `hypermedia.manifold.synthesize_botanical_organ` (`QualiaDB/modalities/botany + living_world_anatomy`)
- `hypermedia.manifold.assemble_vertebrate_anatomy` (`QualiaDB/domains/anatomy_3d + living_world_anatomy`)
- `hypermedia.manifold.rig_fauna_locomotion` (`QualiaDB/modalities/zoology + living_world_anatomy`)
- `hypermedia.manifold.sweep_assembly_profile` (`hypermedia/profile-authoring`)
- `hypermedia.manifold.compile_bim_ifc4` (`hypermedia/manifold-bim-csg`)
- `hypermedia.manifold.erode_geomorphic_heightfield` (`hypermedia/terrain-erosion-solvers`)

### Category 3: `surface` (7 Morphisms)
- `hypermedia.surface.synthesize_maps` (`Etalon/surface_frames/authoring via QualiaDB`)
- `hypermedia.surface.unwrap_slim` (`hypermedia/asset-production-vibe/surface_bsdf_uv`)
- `hypermedia.surface.compute_mikktspace` (`hypermedia/asset-production-vibe/surface_bsdf_uv`)
- `hypermedia.surface.compile_bsdf_graph` (`Etalon/etalon-render + surface_bsdf_uv`)
- `hypermedia.surface.sample_procedural_field` (`hypermedia/asset-production-vibe/surface_bsdf_uv`)
- `hypermedia.surface.bake_pbr_materialx` (`hypermedia/surface-pbr-studio + surface-layered-codecs`)
- `hypermedia.surface.expand_procedural_brush` (`hypermedia/vector-surface-brush + raster-layer-mask-adapter`)

### Category 4: `radiance` (29 Morphisms)
- `hypermedia.radiance.rasterize_loci` (`hypermedia/vector-cpu-render-adapter`)
- `hypermedia.radiance.convolve_blur` (`QualiaDB/computer_vision`)
- `hypermedia.radiance.blend_rgb` (`hypermedia/raster-blend-operators`)
- `hypermedia.radiance.transform_exposure` (`hypermedia/color-transforms`)
- `hypermedia.radiance.morphology_mask` (`hypermedia/raster-selection-morphology`)
- `hypermedia.radiance.encode_rgb_png` (`QualiaDB/computer_vision/codecs`)
- `hypermedia.radiance.import_rgb_png` (`QualiaDB/computer_vision/codecs`)
- `hypermedia.radiance.render_vector` (`hypermedia/vector-cpu-render-adapter + kurbo`)
- `hypermedia.radiance.create_rgba` (`QualiaDB/hypermedia_authoring/image`)
- `hypermedia.radiance.compose_rgba` (`QualiaDB/hypermedia_authoring/image`)
- `hypermedia.radiance.filter_rgba` (`QualiaDB/hypermedia_authoring/image`)
- `hypermedia.radiance.encode_rgba_png` (`png codec`)
- `hypermedia.radiance.import_rgba_png` (`png codec`)
- `hypermedia.radiance.modify_mask` (`hypermedia/raster-layer-mask-adapter`)
- `hypermedia.radiance.apply_lut` (`hypermedia/color-transforms/lut`)
- `hypermedia.radiance.grade_color` (`hypermedia/color-transforms/grade`)
- `hypermedia.radiance.convert_transfer` (`hypermedia/color-transforms`)
- `hypermedia.radiance.develop_photo` (`hypermedia/asset-production-vibe/radiance_photo`)
- `hypermedia.radiance.encode_psd` (`hypermedia/asset-production-vibe/radiance_photo`)
- `hypermedia.radiance.import_psd` (`hypermedia/asset-production-vibe/radiance_photo`)
- `hypermedia.radiance.filter_kuwahara_anisotropic` (`hypermedia/asset-production-vibe/radiance_compositor_gi`)
- `hypermedia.radiance.decode_cryptomatte_q42` (`hypermedia/asset-production-vibe/radiance_compositor_gi`)
- `hypermedia.radiance.vector_motion_blur` (`hypermedia/asset-production-vibe/radiance_compositor_gi`)
- `hypermedia.radiance.solve_sdfgi_froxel` (`Etalon/etalon-render + radiance_compositor_gi`)
- `hypermedia.radiance.build_light_tree` (`Etalon/etalon-render + radiance_compositor_gi`)
- `hypermedia.radiance.cull_frustum_visibility` (`QualiaDB/render/frustum_lod + living_world_anatomy`)
- `hypermedia.radiance.develop_camera_raw_hdr` (`hypermedia/radiance-camera-raw + color-transforms`)
- `hypermedia.radiance.grade_cdl_scopes` (`hypermedia/radiance-color-grading + raster-blend-operators`)
- `hypermedia.radiance.trace_vector_contours` (`hypermedia/vector-radiance-trace + raster-spatial-operators + raster-selection-morphology`)

### Category 5: `glyph` (7 Morphisms)
- `hypermedia.glyph.measure_bounds` (`hypermedia/asset-production-vibe/glyph`)
- `hypermedia.glyph.shape_to_svg_path` (`hypermedia/asset-production-vibe/glyph`)
- `hypermedia.glyph.project_svg` (`hypermedia/asset-production-vibe/glyph`)
- `hypermedia.glyph.rasterize_rgba` (`hypermedia/vector-cpu-render-adapter + glyph`)
- `hypermedia.glyph.shape_harfbuzz_bidi` (`hypermedia/asset-production-vibe/glyph_layout_diegetic`)
- `hypermedia.glyph.pack_msdf_font_atlas` (`Etalon/ui_canvas + glyph_layout_diegetic`)
- `hypermedia.glyph.compose_knuth_plass_paragraph` (`hypermedia/vector-glyph-shaping`)

### Category 6: `layout` (9 Morphisms)
- `hypermedia.layout.solve_box_tree` (`hypermedia/asset-production-vibe/layout`)
- `hypermedia.layout.bind_theme_scope` (`hypermedia/asset-production-vibe/layout`)
- `hypermedia.layout.render_ui_sheet` (`hypermedia/vector-cpu-render-adapter + layout`)
- `hypermedia.layout.project_ui_svg` (`hypermedia/asset-production-vibe/layout`)
- `hypermedia.layout.project_diegetic_surface` (`hypermedia/asset-production-vibe/glyph_layout_diegetic`)
- `hypermedia.layout.route_input_action` (`hypermedia/asset-production-vibe/glyph_layout_diegetic`)
- `hypermedia.layout.solve_flexbox_grid_constraints` (`hypermedia/layout-box-autolayout`)
- `hypermedia.layout.typeset_paged_foliation` (`hypermedia/layout-paged-typesetter`)
- `hypermedia.layout.compile_dataflow_nodegraph` (`hypermedia/layout-nodegraph-compiler + layout-presentation-deck`)

### Category 7: `kinetics` (15 Morphisms)
- `hypermedia.kinetics.sample_frames` (`hypermedia/timeline-editing`)
- `hypermedia.kinetics.solve_two_bone_ik` (`Etalon/etalon-render/skeletal + QualiaDB/world3d`)
- `hypermedia.kinetics.interpolate_keyframes` (`QualiaDB/hypermedia_authoring/world3d`)
- `hypermedia.kinetics.export_lottie` (`hypermedia/asset-production-vibe/kinetics_ops`)
- `hypermedia.kinetics.interpolate_motor_fti` (`Etalon/etalon-render/skeletal + kinetics_sim`)
- `hypermedia.kinetics.step_island_xpbd` (`hypermedia/asset-production-vibe/kinetics_sim`)
- `hypermedia.kinetics.query_direct_space` (`hypermedia/asset-production-vibe/kinetics_sim`)
- `hypermedia.kinetics.navigate_funnel_orca` (`hypermedia/asset-production-vibe/kinetics_sim`)
- `hypermedia.kinetics.evaluate_nla_channelbag` (`hypermedia/asset-production-vibe/kinetics_sim`)
- `hypermedia.kinetics.step_simulation_zone` (`hypermedia/asset-production-vibe/kinetics_sim`)
- `hypermedia.kinetics.simulate_continuum_field` (`QualiaDB/solvers/continuum + living_world_anatomy`)
- `hypermedia.kinetics.solve_continuum_fem_lbm` (`hypermedia/kinetics-continuum-multiphysics`)
- `hypermedia.kinetics.drape_orthotropic_garment` (`hypermedia/kinetics-garment-draping`)
- `hypermedia.kinetics.simulate_causal_block_diagram` (`hypermedia/kinetics-block-simulation + kinetics-motion-compositor`)
- `hypermedia.kinetics.schedule_cpm_evm_montecarlo` (`hypermedia/kinetics-schedule-cpm`)

### Category 8: `acoustics` (9 Morphisms)
- `hypermedia.acoustics.synthesize_tone` (`QualiaDB/qualia-audio`)
- `hypermedia.acoustics.scale_gain` (`shared audio buffer adapter`)
- `hypermedia.acoustics.encode_wav` (`QualiaDB/qualia-audio + Etalon/signal`)
- `hypermedia.acoustics.filter_biquad` (`Etalon/etalon-signal + QualiaDB/qualia-audio`)
- `hypermedia.acoustics.mix_tracks` (`QualiaDB/qualia-audio + shared mixer`)
- `hypermedia.acoustics.encode_aac` (`hypermedia/audio-aac-codec`)
- `hypermedia.acoustics.route_bus_dag` (`QualiaDB/qualia-audio + Etalon/etalon-signal`)
- `hypermedia.acoustics.spatialize_pga_zone` (`QualiaDB/qualia-audio + acoustics_discourse_chronicle_ext`)
- `hypermedia.acoustics.analyze_ebu_r128_camelot_daw` (`hypermedia/acoustics-daw-session + audio-timebase`)

### Category 9: `electromagnetics` (6 Morphisms)
- `hypermedia.electromagnetics.compile_emf` (`QualiaDB/q42/emf_container`)
- `hypermedia.electromagnetics.verify_emf` (`QualiaDB/q42/emf_container`)
- `hypermedia.electromagnetics.render_spectral_slice` (`Etalon/etalon-render/spectral + QualiaDB/q42/emf_container`)
- `hypermedia.electromagnetics.raymarch_vox0_dda` (`QualiaDB/q42/emf_container + electromagnetics_geodesy_ext`)
- `hypermedia.electromagnetics.project_metamer_nullspace` (`Etalon/etalon-render/spectral/metamer`)
- `hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa` (`hypermedia/electromagnetics-rf-sensing`)

### Category 10: `geodesy` (6 Morphisms)
- `hypermedia.geodesy.sculpt_terrain` (`hypermedia/terrain-brushes`)
- `hypermedia.geodesy.compile_m64` (`QualiaDB/q42/m64_matter + hypermedia/terrain-brushes`)
- `hypermedia.geodesy.verify_m64` (`QualiaDB/q42/m64_matter`)
- `hypermedia.geodesy.advect_m64_grid` (`hypermedia/asset-production-vibe/electromagnetics_geodesy_ext`)
- `hypermedia.geodesy.couple_continuum_xpbd` (`hypermedia/asset-production-vibe/electromagnetics_geodesy_ext`)
- `hypermedia.geodesy.kriging_variogram_morphometry` (`hypermedia/geodesy-spatial-analysis`)

### Category 11: `discourse` (9 Morphisms)
- `hypermedia.discourse.review_gallery` (`hypermedia/asset-production-vibe`)
- `hypermedia.discourse.compile_pdf` (`hypermedia/asset-production-vibe/discourse_docs`)
- `hypermedia.discourse.compile_prose_docx` (`hypermedia/asset-production-vibe/discourse_docs`)
- `hypermedia.discourse.compile_deck_pptx` (`hypermedia/asset-production-vibe/discourse_docs`)
- `hypermedia.discourse.render_chart_svg` (`hypermedia/asset-production-vibe/discourse_docs`)
- `hypermedia.discourse.inspect_engine_telemetry` (`hypermedia/asset-production-vibe/acoustics_discourse_chronicle_ext`)
- `hypermedia.discourse.verify_fiduciary_rid` (`hypermedia/asset-production-vibe/acoustics_discourse_chronicle_ext`)
- `hypermedia.discourse.resolve_ontology_pin` (`QualiaDB/semantic_organisms + living_world_anatomy`)
- `hypermedia.discourse.audit_tex_pdf_ua_matlang` (`hypermedia/discourse-academic-tex + discourse-pdf-compliance + discourse-scientific-matlang + multimodal-synthesis-schema`)

### Category 12: `chronicle` (5 Morphisms)
- `hypermedia.chronicle.mux_mp4` (`hypermedia/media-isobmff + bitstream-codec + video-h264-preview`)
- `hypermedia.chronicle.decode_mp4_frame` (`hypermedia/video-h264-preview + media-isobmff`)
- `hypermedia.chronicle.capture_deterministic_movie` (`hypermedia/asset-production-vibe/acoustics_discourse_chronicle_ext`)
- `hypermedia.chronicle.sync_spatial_motors` (`hypermedia/asset-production-vibe/acoustics_discourse_chronicle_ext`)
- `hypermedia.chronicle.recalculate_tabular_fiduciary_archive` (`hypermedia/chronicle-tabular-engine + chronicle-fiduciary-ledger + chronicle-office-codecs + chronicle-timeline-interchange + chronicle-broadcast-containers + video-prores-codec`)
