/// Versioned categorical operation contracts exposed by application Vibe capability hosts.
/// Canonical names follow the category-theoretic `hypermedia.<category>.<morphism>` taxonomy;
/// historical `hypermedia.*` and `synth.*` names resolve losslessly via `ontology::canonical_morphism_id`.
pub const OPERATIONS: &[(&str, &str, &str)] = &[
    ("hypermedia.locus.construct", "hypermedia/profile-authoring",
     "Parametric circle, rounded rectangle, open channel, or polygon -> closed 2D locus"),
    ("hypermedia.locus.transform", "hypermedia/drafting-geometry",
     "2D affine matrix transformation -> new closed 2D locus"),
    ("hypermedia.locus.project_svg", "hypermedia/drafting-geometry + shared SVG serialization",
     "2D locus -> standalone UTF-8 SVG vector resource"),
    ("hypermedia.manifold.extrude", "QualiaDB/computational_geometry",
     "2D locus swept along a 3D transport path -> 3D surface/solid manifold"),
    ("hypermedia.manifold.revolve", "QualiaDB/parametric_cad",
     "Positive radial contour revolved around axis -> 3D surface manifold"),
    ("hypermedia.manifold.loft", "QualiaDB/parametric_cad",
     "Two corresponding 3D boundary curves -> ruled origin loft; interpolation=cubic Catmull-Rom densified manifold"),
    ("hypermedia.manifold.compile_10d", "QualiaDB/render/compile_10d",
     "3D manifold -> sealed QualiaDB native .10d spatial container resource"),
    ("hypermedia.radiance.rasterize_loci", "hypermedia/vector-cpu-render-adapter",
     "Affine-transformed 2D locus layers over opaque background -> sRGB8 image lattice"),
    ("hypermedia.radiance.convolve_blur", "QualiaDB/computer_vision",
     "Opaque sRGB8 lattice -> per-channel 3x3 Gaussian low-pass filtered lattice"),
    ("hypermedia.radiance.blend_rgb", "hypermedia/raster-blend-operators",
     "Two congruent sRGB8 lattices -> burn, dodge, vivid-light, or hard-mix blend"),
    ("hypermedia.radiance.transform_exposure", "hypermedia/color-transforms",
     "sRGB8 -> scene-linear EV exposure scaling -> sRGB8"),
    ("hypermedia.radiance.morphology_mask", "hypermedia/raster-selection-morphology",
     "Luminance threshold -> exact Euclidean expand, contract, or border mask lattice"),
    ("hypermedia.radiance.encode_rgb_png", "QualiaDB/computer_vision/codecs",
     "Opaque sRGB8 lattice -> lossless RGB8 PNG resource"),
    ("hypermedia.radiance.import_rgb_png", "QualiaDB/computer_vision/codecs",
     "Retained opaque RGB8 PNG input -> sRGB8 lattice handle"),
    ("hypermedia.acoustics.synthesize_tone", "QualiaDB/qualia-audio",
     "Harmonic sine oscillator parameters -> mono f32 acoustic wave buffer"),
    ("hypermedia.acoustics.scale_gain", "shared audio buffer adapter",
     "Linear amplitude gain with [-1, 1] hard clipping -> new mono f32 wave buffer"),
    ("hypermedia.acoustics.encode_wav", "QualiaDB/qualia-audio + Etalon/signal",
     "Mono f32 wave buffer -> RIFF WAVE 16-bit PCM little-endian resource"),
    ("hypermedia.kinetics.sample_frames", "hypermedia/timeline-editing",
     "Rational frame rate (numerator/denominator) -> exact flick ticks and seconds"),
    ("hypermedia.radiance.render_vector", "hypermedia/vector-cpu-render-adapter + kurbo",
     "SVG Bézier paths, solid/linear-gradient fills, strokes, and clip rects -> straight-alpha RGBA8 lattice"),
    ("hypermedia.radiance.create_rgba", "QualiaDB/hypermedia_authoring/image",
     "Uniform straight-alpha RGBA8 color fill -> RGBA8 image lattice"),
    ("hypermedia.radiance.compose_rgba", "QualiaDB/hypermedia_authoring/image",
     "Ordered RGBA layers with pixel offsets, opacity, separable blend modes, and alpha masks -> RGBA8 lattice"),
    ("hypermedia.radiance.filter_rgba", "QualiaDB/hypermedia_authoring/image",
     "Bounded Gaussian blur, unsharp mask sharpen, hue rotation, or directional emboss -> RGBA8 lattice"),
    ("hypermedia.radiance.encode_rgba_png", "png codec",
     "Straight-alpha RGBA8 lattice + optional profile=16-bit -> lossless 8-bit or 16-bit transparent PNG resource"),
    ("hypermedia.radiance.import_rgba_png", "png codec",
     "Retained 8-bit or 16-bit PNG (RGBA/RGB/grayscale/indexed) + lattice profile -> straight-alpha RGBA8 lattice"),
    ("hypermedia.surface.synthesize_maps", "Etalon/surface_frames/authoring via QualiaDB",
     "Procedural surface field parameters -> tileable PBR {colour, normal, orm} RGBA8 handles"),
    ("hypermedia.discourse.review_gallery", "hypermedia/asset-production-vibe",
     "Named encoded resources -> CSP-locked, self-contained HTML human & agent review gallery"),
    ("hypermedia.locus.spline", "hypermedia/drafting-geometry",
     "Cubic B-spline interpolation over control/fit points -> closed 2D locus"),
    ("hypermedia.locus.offset", "hypermedia/drafting-geometry",
     "Miter-limited inward or outward parallel contour offset -> closed 2D locus"),
    ("hypermedia.locus.boolean", "QualiaDB/computational_geometry/dcel_overlay",
     "Exact 2D planar union, intersection, difference, or xor -> closed 2D locus"),
    ("hypermedia.locus.stroke_to_locus", "hypermedia/drafting-geometry",
     "Open 2D polyline spine + width + cap/join style -> closed 2D outline locus"),
    ("hypermedia.glyph.measure_bounds", "hypermedia/asset-production-vibe/glyph",
     "Text string + font metrics + wrapping width -> exact bounding box & glyph count record"),
    ("hypermedia.glyph.shape_to_svg_path", "hypermedia/asset-production-vibe/glyph",
     "Text string + font size + kerning/wrapping -> shaped SVG path d string & metrics record"),
    ("hypermedia.glyph.project_svg", "hypermedia/asset-production-vibe/glyph",
     "Shaped typographic text run -> standalone UTF-8 vector SVG resource"),
    ("hypermedia.glyph.rasterize_rgba", "hypermedia/vector-cpu-render-adapter + glyph",
     "Shaped typographic text run -> anti-aliased straight-alpha RGBA8 lattice"),
    ("hypermedia.layout.solve_box_tree", "hypermedia/asset-production-vibe/layout",
     "Viewport dimensions + flex/grid box specifications -> solved 2D spatial box coordinates"),
    ("hypermedia.layout.bind_theme_scope", "hypermedia/asset-production-vibe/layout",
     "Webizen scope (environment|app|page|module) + mode -> resolved RGBA tokens & CSS variables"),
    ("hypermedia.layout.render_ui_sheet", "hypermedia/vector-cpu-render-adapter + layout",
     "Solved box tree + scoped theme tokens -> straight-alpha RGBA8 UI component/page sheet"),
    ("hypermedia.layout.project_ui_svg", "hypermedia/asset-production-vibe/layout",
     "Solved box tree + scoped theme tokens -> standalone UTF-8 SVG UI/website design sheet"),
    ("hypermedia.radiance.modify_mask", "hypermedia/raster-layer-mask-adapter",
     "Straight-alpha or luma channel -> exact Euclidean expand, contract, or border mask RGBA8 lattice"),
    ("hypermedia.radiance.apply_lut", "hypermedia/color-transforms/lut",
     "Straight-alpha RGBA8 lattice + .cube or 3D LUT preset -> tetrahedrally interpolated RGBA8 lattice"),
    ("hypermedia.radiance.grade_color", "hypermedia/color-transforms/grade",
     "Straight-alpha RGBA8 lattice + SDR/PQ/HLG GradeSpace ASC-CDL lift/gamma/gain/contrast/saturation -> RGBA8 lattice"),
    ("hypermedia.radiance.convert_transfer", "hypermedia/color-transforms",
     "Straight-alpha RGBA8 lattice + source/target transfer function (srgb|bt709|linear|pq|hlg) -> RGBA8 lattice"),
    ("hypermedia.radiance.develop_photo", "hypermedia/asset-production-vibe/radiance_photo",
     "Straight-alpha RGBA8 lattice + exposure/WB/highlights/shadows/clarity/vibrance/denoise -> developed RGBA8 lattice"),
    ("hypermedia.radiance.encode_psd", "hypermedia/asset-production-vibe/radiance_photo",
     "Named straight-alpha RGBA8 layer stack -> multi-layer Adobe Photoshop .psd binary resource"),
    ("hypermedia.radiance.import_psd", "hypermedia/asset-production-vibe/radiance_photo",
     "Retained or encoded .psd resource (+ optional layer_index) -> straight-alpha RGBA8 lattice"),
    ("hypermedia.manifold.boolean_3d", "QualiaDB/computational_geometry/boolean_3",
     "Two closed 3D triangle manifolds + union|intersection|difference -> 3D manifold"),
    ("hypermedia.manifold.simplify", "QualiaDB/computational_geometry/decimate_qem",
     "3D triangle manifold + target_triangles + max_error -> simplified QEM 3D manifold"),
    ("hypermedia.manifold.import_mesh", "QualiaDB/render/assets/import_asset_with_normals",
     "Retained OBJ, STL, or GLB binary/ASCII input -> 3D triangle manifold handle"),
    ("hypermedia.manifold.project_dxf", "hypermedia/asset-production-vibe/manifold_ops",
     "2D locus and/or 3D manifold -> standards-compliant AutoCAD ASCII .dxf resource"),
    ("hypermedia.manifold.compile_10d_pbr", "QualiaDB/render/compile_10d",
     "3D manifold + optional PBR albedo RGBA + topology/BVH extras + unwrap UV01 source_10d -> sealed .10d container"),
    ("hypermedia.electromagnetics.compile_emf", "QualiaDB/q42/emf_container",
     "Wave/dipole emitter parameters + 3D Hilbert brick grid -> sealed Q42EMF64 .emf container"),
    ("hypermedia.electromagnetics.verify_emf", "QualiaDB/q42/emf_container",
     "Sealed .emf container -> CRC-32C + BLAKE3-Bao verification, total energy density & peak Poynting flux"),
    ("hypermedia.electromagnetics.render_spectral_slice", "QualiaDB/q42/emf_container + CIE 1931 spectral projection",
     "Sealed .emf container + slice axis/index + exposure -> straight-alpha RGBA8 spectral slice lattice"),
    ("hypermedia.geodesy.sculpt_terrain", "hypermedia/terrain-brushes",
     "CoW heightfield tile + reversible brush strokes -> hypsometric relief RGBA8 handle & elevation stats"),
    ("hypermedia.geodesy.compile_m64", "QualiaDB/q42/m64_matter + hypermedia/terrain-brushes",
     "Geodetic origin + sculpted terrain & subsurface strata columns -> sealed Matter-64 .m64 STRATA/HOLO/POINT container"),
    ("hypermedia.geodesy.verify_m64", "QualiaDB/q42/m64_matter",
     "Sealed .m64 container -> structural & per-record CRC-32C verification summary record"),
    ("hypermedia.kinetics.solve_two_bone_ik", "Etalon/etalon-render/skeletal + QualiaDB/world3d",
     "Root/target/pole 3D coordinates + bone lengths -> analytical PGA Motor two-bone IK solution & 3-bone Rig"),
    ("hypermedia.kinetics.interpolate_keyframes", "QualiaDB/hypermedia_authoring/world3d",
     "3D keyframe track + easing curve + rational frame rate -> sampled AnimationClip trajectory"),
    ("hypermedia.kinetics.export_lottie", "hypermedia/asset-production-vibe/kinetics_ops",
     "Animated vector shape layers & transform tracks -> Bodymovin/Lottie v5.7.4 .json animation resource"),
    ("hypermedia.acoustics.filter_biquad", "Etalon/etalon-signal + QualiaDB/qualia-audio",
     "Mono f32 wave buffer + lowpass|highpass|bandpass|notch|peaking biquad parameters -> filtered wave buffer"),
    ("hypermedia.acoustics.mix_tracks", "QualiaDB/qualia-audio + shared mixer",
     "Multi-track wave buffers with gain, offset, and fade-in/out envelopes -> mixed mono f32 wave buffer"),
    ("hypermedia.acoustics.encode_aac", "hypermedia/audio-aac-codec",
     "Mono f32 wave buffer -> ISO/IEC 14496-3 AAC-LC CBR ADTS .aac bitstream resource"),
    ("hypermedia.chronicle.mux_mp4", "hypermedia/media-isobmff + bitstream-codec + video-h264-preview",
     "Sequence of straight-alpha RGBA8 frames + fps -> self-contained H.264 Baseline I_PCM .mp4 video resource"),
    ("hypermedia.chronicle.decode_mp4_frame", "hypermedia/video-h264-preview + media-isobmff",
     "Encoded or retained .mp4 container + frame_index -> decoded BT.709 straight-alpha RGBA8 lattice handle"),
    ("hypermedia.discourse.compile_pdf", "hypermedia/asset-production-vibe/discourse_docs",
     "Structured pages with headings, paragraphs, and vector shapes -> ISO 32000-1 %PDF-1.7 document resource"),
    ("hypermedia.discourse.compile_prose_docx", "hypermedia/asset-production-vibe/discourse_docs",
     "Structured prose sections, paragraphs, and tabular cells -> ECMA-376 OpenXML .docx document resource"),
    ("hypermedia.discourse.compile_deck_pptx", "hypermedia/asset-production-vibe/discourse_docs",
     "Ordered slide specifications with titles and bullet hierarchies -> ECMA-376 OpenXML .pptx with one DrawingML paragraph per bullet"),
    ("hypermedia.discourse.render_chart_svg", "hypermedia/asset-production-vibe/discourse_docs",
     "Quantitative data series + bar|line|area|scatter chart mode -> standalone UTF-8 data visualization .svg"),
    ("hypermedia.locus.fillet", "hypermedia/asset-production-vibe/locus_generative",
     "2D closed/open locus + corner radius + arc|chamfer segments -> filleted 2D locus"),
    ("hypermedia.locus.resample", "hypermedia/asset-production-vibe/locus_generative",
     "2D locus + count|length|curvature mode -> arc-length or curvature-adaptive resampled 2D locus"),
    ("hypermedia.locus.trim", "hypermedia/asset-production-vibe/locus_generative",
     "2D locus + normalized arc-length [start, end] interval -> trimmed sub-curve locus"),
    ("hypermedia.locus.bishop_rmf_cyclic", "QualiaDB/computational_geometry/sweep_extrusion",
     "3D spine points + closed-loop holonomy flag -> twist-free Double-Reflection Bishop RMF frame sequence"),
    ("hypermedia.locus.shortest_edge_path", "hypermedia/asset-production-vibe/locus_generative",
     "3D manifold + start/end vertices -> Dijkstra geodesic edge path & projected 2D profile"),
    ("hypermedia.manifold.radial_euler", "QualiaDB/q42/m64_matter + manifold_topology",
     "3D manifold + Radial-Edge Euler ops (SEMV, SFME, JEKV, JVKE, JFKE) -> non-manifold B-Rep & Mode 0x04 .m64 shell"),
    ("hypermedia.manifold.bevel", "hypermedia/asset-production-vibe/manifold_topology",
     "3D manifold + bevel width & segments -> edge/corner beveled 3D manifold"),
    ("hypermedia.manifold.inset", "hypermedia/asset-production-vibe/manifold_topology",
     "3D manifold + inset thickness & extrusion depth -> face-inset 3D manifold"),
    ("hypermedia.manifold.solidify", "hypermedia/asset-production-vibe/manifold_topology",
     "Open or closed 3D surface manifold + shell thickness -> closed two-sided watertight 3D manifold"),
    ("hypermedia.manifold.bisect_plane", "hypermedia/asset-production-vibe/manifold_topology",
     "3D manifold + cutting plane origin/normal + cap fill -> plane-bisected 3D manifold"),
    ("hypermedia.manifold.subdivide_catmull_clark", "hypermedia/asset-production-vibe/manifold_topology",
     "3D manifold + subdivision levels + edge crease weight -> Catmull-Clark/Loop smoothed 3D manifold"),
    ("hypermedia.manifold.evaluate_field", "hypermedia/asset-production-vibe/manifold_fields_sdf",
     "3D manifold + domain (point|edge|face|instance) + lazy field expression -> chunk-evaluated field & displaced manifold"),
    ("hypermedia.manifold.mesh_to_sdf_brick", "hypermedia/asset-production-vibe/manifold_fields_sdf",
     "3D manifold + voxel resolution + narrow-band half-width -> sparse Q42SDF16 narrow-band SDF brick resource"),
    ("hypermedia.manifold.sdf_brick_to_mesh", "hypermedia/asset-production-vibe/manifold_fields_sdf",
     "Sparse Q42SDF16 SDF brick + iso_level + adaptivity -> Dual-Contoured / Surface-Nets 3D triangle manifold"),
    ("hypermedia.manifold.instance_on_points", "hypermedia/asset-production-vibe/manifold_fields_sdf",
     "Prototype 3D manifold + target points/rotations/scales -> instanced manifold assembly & preview mesh"),
    ("hypermedia.manifold.realize_instances", "hypermedia/asset-production-vibe/manifold_fields_sdf",
     "Instanced manifold assembly -> welded single-buffer 3D triangle manifold"),
    ("hypermedia.manifold.fspm_vascular_grow", "QualiaDB/modalities/botany + manifold_fields_sdf",
     "Allometric seed + Murray's Law exponent + Hagen-Poiseuille xylem conductance -> 3D botanical organ manifold & NQuin48 state"),
    ("hypermedia.surface.unwrap_slim", "hypermedia/asset-production-vibe/surface_bsdf_uv",
     "3D manifold + SLIM iterations -> bijective flip-free Symmetric Dirichlet UV parameterization, SVG chart & UV01-sealed .10d"),
    ("hypermedia.surface.compute_mikktspace", "hypermedia/asset-production-vibe/surface_bsdf_uv",
     "3D manifold -> orthogonal MikkTSpace [Tx, Ty, Tz, sign] tangent frame attributes & normal-map verification RGBA8"),
    ("hypermedia.surface.compile_bsdf_graph", "Etalon/etalon-render + surface_bsdf_uv",
     "Cycles Principled v2 / Chiang Hair / Fournier-Forand Water / Random-Walk SSS closure -> energy-conserving BSDF JSON & preview RGBA8"),
    ("hypermedia.surface.sample_procedural_field", "hypermedia/asset-production-vibe/surface_bsdf_uv",
     "Gabor / Voronoi / Wave / Blackbody / Wavelength procedural basis -> tileable straight-alpha RGBA8 field"),
    ("hypermedia.radiance.filter_kuwahara_anisotropic", "hypermedia/asset-production-vibe/radiance_compositor_gi",
     "Straight-alpha RGBA8 + structure-tensor eccentricity & sharpness -> edge-preserving Anisotropic Kuwahara RGBA8"),
    ("hypermedia.radiance.decode_cryptomatte_q42", "hypermedia/asset-production-vibe/radiance_compositor_gi",
     "Straight-alpha RGBA8 + did:q42 MurmurHash3 object ID -> anti-aliased Cryptomatte coverage mask & isolated RGBA8"),
    ("hypermedia.radiance.vector_motion_blur", "hypermedia/asset-production-vibe/radiance_compositor_gi",
     "Straight-alpha RGBA8 + 2D velocity vector + shutter integration -> line-integral motion-blurred RGBA8"),
    ("hypermedia.radiance.solve_sdfgi_froxel", "Etalon/etalon-render + radiance_compositor_gi",
     "Base RGBA8 + cascaded SDFGI probes + Henyey-Greenstein volumetric froxel phase -> global-illuminated RGBA8"),
    ("hypermedia.radiance.build_light_tree", "Etalon/etalon-render + radiance_compositor_gi",
     "Emitter descriptors -> Cycles-grade hierarchical bounding-cone Light Tree BVH & importance-sampled preview RGBA8"),
    ("hypermedia.glyph.shape_harfbuzz_bidi", "hypermedia/asset-production-vibe/glyph_layout_diegetic",
     "Multilingual UTF-8 text + UAX #9 BiDi level resolution + OpenType features -> shaped glyph cluster sequence"),
    ("hypermedia.glyph.pack_msdf_font_atlas", "Etalon/ui_canvas + glyph_layout_diegetic",
     "Charset + cell dimension + SDF pixel range -> packed multi-channel MSDF RGBA8 atlas & JSON glyph metric table"),
    ("hypermedia.layout.project_diegetic_surface", "hypermedia/asset-production-vibe/glyph_layout_diegetic",
     "2D UI sheet RGBA8 + 3D quad pose + screen pick ray -> perspective diegetic render & reverse-projected 2D UI UV"),
    ("hypermedia.layout.route_input_action", "hypermedia/asset-production-vibe/glyph_layout_diegetic",
     "Hardware input event + 2D HUD rects + 3D diegetic quad + InputMap -> 4-stage fiduciary routed action record"),
    ("hypermedia.kinetics.interpolate_motor_fti", "Etalon/etalon-render/skeletal + kinetics_sim",
     "Previous/current PGA motor poses + interpolation fraction + teleport flag -> ScLERP interpolated MotorFTI pose"),
    ("hypermedia.kinetics.step_island_xpbd", "hypermedia/asset-production-vibe/kinetics_sim",
     "Rigid bodies + Area3D gravity/damping overrides + substeps -> deterministic ID-sorted XPBD island simulation step"),
    ("hypermedia.kinetics.query_direct_space", "hypermedia/asset-production-vibe/kinetics_sim",
     "Static/dynamic BVH shapes + ray/sweep motion query + collision mask -> DirectSpaceState safe/unsafe hit record"),
    ("hypermedia.kinetics.navigate_funnel_orca", "hypermedia/asset-production-vibe/kinetics_sim",
     "3D NavMesh portals + agent velocity + neighbor obstacles -> Funnel string-pulled path & ORCA collision-free velocity"),
    ("hypermedia.kinetics.evaluate_nla_channelbag", "hypermedia/asset-production-vibe/kinetics_sim",
     "Layered ActionChannelbag strips + 12-mode LensTweenTable easing -> blended NLA animation channel value"),
    ("hypermedia.kinetics.step_simulation_zone", "hypermedia/asset-production-vibe/kinetics_sim",
     "Point cloud + curl-noise/attractor field + ping-pong iterations -> double-buffered Simulation Zone state"),
    ("hypermedia.acoustics.route_bus_dag", "QualiaDB/qualia-audio + Etalon/etalon-signal",
     "Multi-bus audio DAG + 64-sample anti-pop state ramp + biquad/compressor/reverb chain -> mixed master wave buffer"),
    ("hypermedia.acoustics.spatialize_pga_zone", "QualiaDB/qualia-audio + acoustics_discourse_chronicle_ext",
     "Mono wave buffer + source/listener 3D PGA poses & velocities + occlusion -> Doppler/HRTF/Area3D spatialized audio"),
    ("hypermedia.electromagnetics.raymarch_vox0_dda", "QualiaDB/q42/emf_container + electromagnetics_geodesy_ext",
     "Sealed .emf or harmonic Vox0 brick + ADM 3+1 lensing mass + Stokes filter -> Amanatides-Woo DDA raymarched RGBA8"),
    ("hypermedia.electromagnetics.project_metamer_nullspace", "Etalon/etalon-render/spectral/metamer",
     "41-band SPD + null-space scale -> Cohen- Kappauf fundamental + 38D metameric black projection with zero CIE XYZ drift"),
    ("hypermedia.geodesy.advect_m64_grid", "hypermedia/asset-production-vibe/electromagnetics_geodesy_ext",
     "2D/3D fluid grid + Poisson divergence-free projection + MacCormack advection + Eikonal redistancing -> advected field & RGBA8"),
    ("hypermedia.geodesy.couple_continuum_xpbd", "hypermedia/asset-production-vibe/electromagnetics_geodesy_ext",
     "Wheel/hull contact patch + Bekker-Wong/Mohr-Coulomb regolith + Gerstner wave state -> terramechanical sinkage, drawbar pull & buoyancy"),
    ("hypermedia.discourse.inspect_engine_telemetry", "hypermedia/asset-production-vibe/acoustics_discourse_chronicle_ext",
     "256-frame subsystem ring-buffer + QuinRidTable VRAM ledger -> capacity telemetry JSON & P99 frame compliance record"),
    ("hypermedia.discourse.verify_fiduciary_rid", "hypermedia/asset-production-vibe/acoustics_discourse_chronicle_ext",
     "Resource handle + AgreementDID + deontic modality (permit|oblige|forbid) -> human-rights fiduciary compliance verification"),
    ("hypermedia.chronicle.capture_deterministic_movie", "hypermedia/asset-production-vibe/acoustics_discourse_chronicle_ext",
     "RGBA8 frame sequence + wave buffer + ManifoldClock locked-dt -> deterministic offline .mp4 movie & sync report"),
    ("hypermedia.chronicle.sync_spatial_motors", "hypermedia/asset-production-vibe/acoustics_discourse_chronicle_ext",
     "Entity PGA motor states + sequence number -> quantized 60Hz 16-byte spatial replication packet & dequantized error report"),
    ("hypermedia.manifold.compile_botanical_organ_10d", "QualiaDB/render/botanical_anatomy",
     "Botanical scale (1..=4) + species_id + peel/anthesis parameters -> sealed 4-scale botanical .10d organ & histology verification record"),
    ("hypermedia.manifold.compile_vertebrate_anatomy_10d", "QualiaDB/render/vertebrate_anatomy",
     "Fauna biomechanical archetype + 3-layer visibility & explode factors -> Scale-5 comparative vertebrate 17-submesh .10d anatomy & layer peel record"),
    ("hypermedia.kinetics.rig_biomechanical_archetype", "QualiaDB/render/skeletal/fauna_rig",
     "Fauna biomechanical archetype + locomotion speed & turn rate -> stepped 18-joint PGA Motor rig, IK solution & verified .10d SKL1 section record"),
    ("hypermedia.discourse.query_anatomical_ontology_pin", "QualiaDB/modalities/biological_ontology",
     "Biological scale (1..=5) + 3D picking ray -> zero-heap Uberon/PO/FMA/DwC anatomical callout pin & biophysical metadata record"),
    ("hypermedia.geodesy.step_sector_continuum_3d", "Etalon/etalon-sim/physics3d + QualiaDB/simulation",
     "Geodetic sector ID + step count -> 3D XPBD + analytical SDF + 2-way soil/hydro/botanical continuum bridge + 4-wheel rover + stress-cleavage fracture report"),
    ("hypermedia.layout.synchronize_companion_frustum", "Etalon/etalon-render/frame_lock",
     "Dome grid (rows x cols) + FOV degrees + Lamport frame index -> conical frustum wedge cluster & 64-byte CRC-32C verified FrameLockPacket record"),
    ("hypermedia.locus.solve_parametric_sketch", "hypermedia/locus-parametric-solver + drafting-geometry",
     "2D parametric sketch constraints + MTEXT/stick-font layout + WGS84 geodesic ruler -> Levenberg-Marquardt solved sketch & geodesic distance record"),
    ("hypermedia.locus.pathfinder_warp_envelope", "hypermedia/vector-locus-pathops + vector-locus-geometry",
     "Cubic Bezier contours + Pathfinder boolean + Live Corners + 15-style Warp envelope -> warped 2D locus & 3D bevel extrusion record"),
    ("hypermedia.locus.transcode_geospatial_vector", "hypermedia/locus-geospatial-codecs",
     "Geodetic coordinates -> OGC WKB binary resource, KML placemark XML, LandXML alignment/TIN & PMTiles v3 Hilbert tile ID record"),
    ("hypermedia.manifold.sweep_assembly_profile", "hypermedia/profile-authoring",
     "Concave/open-channel 2D profile + 3D spine + sphere/torus primitives -> concave-capped 3D manifold & hierarchical TRS CompiledAssembly"),
    ("hypermedia.manifold.compile_bim_ifc4", "hypermedia/manifold-bim-csg",
     "Parametric wall/window/stair parameters -> work-metered 3D BSP CSG boolean solid & ISO 16739 IFC4 STEP architectural model resource"),
    ("hypermedia.manifold.erode_geomorphic_heightfield", "hypermedia/terrain-erosion-solvers",
     "Heightfield grid -> impact crater + stratigraphic terraces + thermal slope relaxation + D8 drainage conditioning + hydraulic droplet erosion record"),
    ("hypermedia.surface.bake_pbr_materialx", "hypermedia/surface-pbr-studio + surface-layered-codecs",
     "3D surface mesh + PBR preset -> BVH ray-cast AO/normal maps, OpenPBR MaterialX 1.39 XML resource & verified PSD layer roundtrip"),
    ("hypermedia.surface.expand_procedural_brush", "hypermedia/vector-surface-brush + raster-layer-mask-adapter",
     "Cubic Bezier spline + stylus weight & wetness -> Calligraphic/Bristle procedural vector stroke pieces & wet Mixer Brush pigment state"),
    ("hypermedia.radiance.develop_camera_raw_hdr", "hypermedia/radiance-camera-raw + color-transforms",
     "Bayer/X-Trans CFA sensor mosaic + exposure brackets -> RCD demosaiced RAW development, deghosted HDR fusion & ICC v4 Display-P3 PCS transform"),
    ("hypermedia.radiance.grade_cdl_scopes", "hypermedia/radiance-color-grading + raster-blend-operators",
     "ASC CDL v1.2 + 3-way color wheels + ARRI LogC4 + tetrahedral 3D LUT + W3C/CMYK blend -> graded RGB & Waveform/Vectorscope/Parade telemetry"),
    ("hypermedia.radiance.trace_vector_contours", "hypermedia/vector-radiance-trace + raster-spatial-operators + raster-selection-morphology",
     "RGBA8 raster -> Schneider cubic Bezier contour trace, Avidan-Shamir seam carving, Octree quantization & Zhang-Suen morphological skeleton"),
    ("hypermedia.glyph.compose_knuth_plass_paragraph", "hypermedia/vector-glyph-shaping",
     "Multilingual paragraph + frame bounds -> UAX #9 BiDi runs, syllable hyphenation, OpenType features, Knuth-Plass optimal line-breaking & SFNT font subset"),
    ("hypermedia.layout.solve_flexbox_grid_constraints", "hypermedia/layout-box-autolayout",
     "Flexbox & CSS Grid container + Fixed/Hug/Fill children -> solved 2D layout rectangles & fractional `fr` track distribution"),
    ("hypermedia.layout.typeset_paged_foliation", "hypermedia/layout-paged-typesetter",
     "Multi-column section + paragraphs + LaTeX math + CSL bibliography -> widow/orphan-controlled paginated pages, equation metrics & formatted citation"),
    ("hypermedia.layout.compile_dataflow_nodegraph", "hypermedia/layout-nodegraph-compiler + layout-presentation-deck",
     "Typed dataflow NodeNetwork + unit-aware math expression + slide pair -> Merkle CSE deduplicated ProtoNetwork, evaluated scalar & interpolated MorphFrame"),
    ("hypermedia.kinetics.solve_continuum_fem_lbm", "hypermedia/kinetics-continuum-multiphysics",
     "Elastic/hyperelastic constitutive parameters + D3Q19 LBM grid -> Cholesky/FEM structural solution, mass-conserving CFD step & Abaqus `.inp` deck resource"),
    ("hypermedia.kinetics.drape_orthotropic_garment", "hypermedia/kinetics-garment-draping",
     "2nd-order Jet<N> auto-diff orthotropic textile + pinned vertices -> membrane/bending energy & Vertex Block Descent draped cloth state"),
    ("hypermedia.kinetics.simulate_causal_block_diagram", "hypermedia/kinetics-block-simulation + kinetics-motion-compositor",
     "Causal block diagram + RK4 solver + 3D spatial Bezier + template image -> integrated state trajectory, arc-length table & NCC point track"),
    ("hypermedia.kinetics.schedule_cpm_evm_montecarlo", "hypermedia/kinetics-schedule-cpm",
     "Task dependency network + trial count -> CPM forward/backward critical path, ANSI/EIA-748 EVM metrics, Monte Carlo P50/P90 risk & Gantt SVG resource"),
    ("hypermedia.acoustics.analyze_ebu_r128_camelot_daw", "hypermedia/acoustics-daw-session + audio-timebase",
     "Audio signal + tempo map + 5.1 azimuth -> ITU-R BS.1770-4 / EBU R128 gated integrated LUFS, YIN pitch/MIDI, VBAP surround gains, Camelot harmony & SMPTE/35mm timecode"),
    ("hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa", "hypermedia/electromagnetics-rf-sensing",
     "CsiFrame 802.11 CSI subcarrier phases + ULA snapshots -> sanitized CSI phase, contactless respiration/cardiac BPM, MUSIC AoA peak & camera-free privacy witness receipt"),
    ("hypermedia.geodesy.kriging_variogram_morphometry", "hypermedia/geodesy-spatial-analysis",
     "WGS84 control points + variogram model + DEM grid -> Karney geodesic inverse, Ordinary Kriging estimate/variance, Horn slope/contours & Dijkstra shortest route"),
    ("hypermedia.discourse.audit_tex_pdf_ua_matlang", "hypermedia/discourse-academic-tex + discourse-pdf-compliance + discourse-scientific-matlang + multimodal-synthesis-schema",
     "TeX manuscript + PDF/UA snapshot + M-code script -> TeX math metrics, 32-rule accessibility & zero-residue PII redaction, SVD decomposition & 3DGS cost"),
    ("hypermedia.chronicle.recalculate_tabular_fiduciary_archive", "hypermedia/chronicle-tabular-engine + chronicle-fiduciary-ledger + chronicle-office-codecs + chronicle-timeline-interchange + chronicle-broadcast-containers + video-prores-codec",
     "Spreadsheet formulas + double-entry journal + timeline clips -> DAG recalculated cells, hash-chained fiduciary audit, DOCX/OTIO/Matroska & Apple ProRes 422"),
    ("qualia.species.assemble_corpus", "QualiaDB/q42/species_corpus",
     "Sources and evidenced assertions -> species .q42 volume, CBOR-LD projection, digest, conflicts and gaps"),
];

#[cfg(test)]
mod tests {
    use crate::*;
    use qualia_core_db::container_10d::read_texture_coordinate_section_into;
    use qualia_core_db::render::assets::Mesh;
    use std::collections::BTreeMap;
    use std::sync::atomic::AtomicBool;
    use vibe::Value;

    fn host<'a>(cancel: &'a AtomicBool, limits: &'a ToolLimits) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits,
            cancel,
            work: 0,
        }
    }

    fn vec3(x: f64, y: f64, z: f64) -> Value {
        Value::List(vec![Value::F64(x), Value::F64(y), Value::F64(z)])
    }

    fn square_ring(z: f64, s: f64) -> Value {
        Value::List(vec![
            vec3(-s, -s, z),
            vec3(s, -s, z),
            vec3(s, s, z),
            vec3(-s, s, z),
        ])
    }

    fn catalog_text() -> String {
        OPERATIONS
            .iter()
            .flat_map(|(id, owner, summary)| [*id, *owner, *summary])
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn catalog_rows_raise_wave_b_objects() {
        let text = catalog_text();
        for needle in [
            "CsiFrame",
            "interpolation=cubic",
            "UV01",
            "metamer",
            "STRATA/HOLO",
            "SKL1",
            "OTIO",
            "ProRes",
            "Live Corners",
            "16-bit",
            "gated",
            "paragraph per bullet",
        ] {
            assert!(
                text.contains(needle),
                "catalog must raise Wave B object {needle}"
            );
        }
        assert_eq!(OPERATIONS.len(), ontology::MORPHISMS.len());
    }

    #[test]
    fn catalog_dispatch_csi_hits_csiframe() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = host(&cancel, &limits);
        let mut args = BTreeMap::new();
        args.insert("breathing_hz".into(), Value::F64(0.25));
        args.insert("heart_hz".into(), Value::F64(1.2));
        args.insert("carrier_ghz".into(), Value::F64(5.18));
        let res = host
            .invoke(
                "hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa",
                &Value::Record(args),
            )
            .expect("public CSI name must hit raised CsiFrame produce");
        let rec = record(&res).expect("csi record");
        let aoa = match rec.get("dominant_aoa_deg") {
            Some(Value::F64(v)) => *v,
            _ => panic!("dominant_aoa_deg"),
        };
        assert!(
            (aoa - 25.0).abs() <= 2.0,
            "CsiFrame MUSIC AoA must recover 25°, got {aoa}"
        );
        assert_eq!(rec.get("camera_free_verified"), Some(&Value::Bool(true)));
        assert_eq!(
            rec.get("sanitized_subcarrier_count"),
            Some(&Value::I64(7))
        );
    }

    #[test]
    fn catalog_dispatch_cubic_loft_not_interpolation_rejected() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = host(&cancel, &limits);

        let origin = host
            .invoke(
                "hypermedia.manifold.loft",
                &Value::Record({
                    let mut a = BTreeMap::new();
                    a.insert("a".into(), square_ring(0.0, 1.0));
                    a.insert("b".into(), square_ring(1.0, 1.0));
                    a.insert("interpolation".into(), Value::String("linear".into()));
                    a
                }),
            )
            .expect("linear interpolation remains ruled origin");
        let origin_n = match host.workspace.get(&origin).expect("origin mesh") {
            Artifact::Mesh(m) => m.positions.len(),
            _ => panic!("expected mesh"),
        };

        let cubic = host
            .invoke(
                "hypermedia.manifold.loft",
                &Value::Record({
                    let mut a = BTreeMap::new();
                    a.insert(
                        "sections".into(),
                        Value::List(vec![
                            square_ring(0.0, 1.0),
                            square_ring(1.0, 2.0),
                            square_ring(2.0, 1.0),
                        ]),
                    );
                    a.insert("caps".into(), Value::Bool(false));
                    a.insert("interpolation".into(), Value::String("cubic".into()));
                    a.insert("samples".into(), Value::U64(4));
                    a
                }),
            )
            .unwrap_or_else(|e| {
                panic!("cubic loft must not be interpolation-rejected, got {e}")
            });
        let Artifact::Mesh(cubic_mesh) = host.workspace.get(&cubic).expect("cubic mesh") else {
            panic!("expected mesh");
        };
        assert!(
            cubic_mesh.positions.len() > origin_n,
            "raised cubic loft densifies stations"
        );
        let densified = cubic_mesh.positions[4];
        let linear_chord = 1.0 + 0.25 * (2.0 - 1.0);
        assert!(
            (densified[0].abs() - linear_chord as f32).abs() > 1e-4,
            "cubic loft must leave the ruled chord, got {}",
            densified[0]
        );
        assert!(!format!("{densified:?}").contains("interpolation rejected"));
    }

    #[test]
    fn catalog_compile_10d_pbr_consumes_unwrap_uv01() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = host(&cancel, &limits);
        let mesh_h = host
            .workspace
            .insert(
                Artifact::Mesh(Mesh {
                    positions: vec![
                        [0.0, 0.0, 0.0],
                        [1.0, 0.0, 0.0],
                        [1.0, 1.0, 0.0],
                        [0.0, 1.0, 0.0],
                    ],
                    triangles: vec![[0, 1, 2], [0, 2, 3]],
                    min: [0.0, 0.0, 0.0],
                    max: [1.0, 1.0, 0.0],
                }),
                host.limits,
            )
            .expect("quad");

        let unwrap = host
            .invoke(
                "hypermedia.surface.unwrap_slim",
                &Value::Record({
                    let mut a = BTreeMap::new();
                    a.insert("mesh".into(), mesh_h.clone());
                    a.insert("iterations".into(), Value::U64(8));
                    a
                }),
            )
            .expect("unwrap");
        let urec = record(&unwrap).expect("unwrap record");
        let source_10d = required(urec, "container_10d")
            .expect("unwrap sealed UV01")
            .clone();

        let compiled = host
            .invoke(
                "hypermedia.manifold.compile_10d_pbr",
                &Value::Record({
                    let mut a = BTreeMap::new();
                    a.insert("mesh".into(), mesh_h);
                    a.insert("source_10d".into(), source_10d);
                    a.insert("topology".into(), Value::Bool(false));
                    a
                }),
            )
            .expect("compile_10d_pbr consumes unwrap UV01");
        let bytes = host
            .workspace
            .encoded(&compiled)
            .expect("compiled 10d")
            .bytes
            .clone();
        let mut recovered = vec![[0.0f32; 2]; 4];
        let n = read_texture_coordinate_section_into(&bytes, 4, &mut recovered)
            .expect("compiled 10d must carry unwrap UV01");
        assert_eq!(n, 4);
        let span_u = recovered
            .iter()
            .map(|p| p[0])
            .fold(f32::NEG_INFINITY, f32::max)
            - recovered.iter().map(|p| p[0]).fold(f32::INFINITY, f32::min);
        assert!(span_u > 0.2, "consumed UV01 island must be non-degenerate");
    }
}
