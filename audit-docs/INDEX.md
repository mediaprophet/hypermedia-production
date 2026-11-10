# hypermedia-production — audit docs

Generated 2026-11-10T12:07:10.5599269Z · 903 files · 8745 functions · 69 modules

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


## Documents

- [`FUNCTIONS.md`](FUNCTIONS.md) — complete alphabetical index (grep this first)
- [`CAPABILITIES.md`](CAPABILITIES.md) — functional-purpose overview per module/domain
- [`DOCS-COVERAGE.md`](DOCS-COVERAGE.md) — doc coverage + undocumented-public worklist
- [`docrefs.ttl`](docrefs.ttl) — RDF overlay: every function → its module doc (external-doc refs)
- [`modules/`](modules/) — per-module registries:
  - [`checks/consumer-smoke`](modules/checks_consumer-smoke.md)
  - [`crates/acoustics-daw-session`](modules/crates_acoustics-daw-session.md)
  - [`crates/asset-production-vibe`](modules/crates_asset-production-vibe.md)
  - [`crates/audio-aac-codec`](modules/crates_audio-aac-codec.md)
  - [`crates/audio-device-cpal`](modules/crates_audio-device-cpal.md)
  - [`crates/audio-opus-codec`](modules/crates_audio-opus-codec.md)
  - [`crates/audio-timebase`](modules/crates_audio-timebase.md)
  - [`crates/bitstream-codec`](modules/crates_bitstream-codec.md)
  - [`crates/chronicle-broadcast-containers`](modules/crates_chronicle-broadcast-containers.md)
  - [`crates/chronicle-fiduciary-ledger`](modules/crates_chronicle-fiduciary-ledger.md)
  - [`crates/chronicle-office-codecs`](modules/crates_chronicle-office-codecs.md)
  - [`crates/chronicle-tabular-engine`](modules/crates_chronicle-tabular-engine.md)
  - [`crates/chronicle-timeline-interchange`](modules/crates_chronicle-timeline-interchange.md)
  - [`crates/color-transforms`](modules/crates_color-transforms.md)
  - [`crates/discourse-academic-tex`](modules/crates_discourse-academic-tex.md)
  - [`crates/discourse-pdf-compliance`](modules/crates_discourse-pdf-compliance.md)
  - [`crates/discourse-scientific-matlang`](modules/crates_discourse-scientific-matlang.md)
  - [`crates/drafting-geometry`](modules/crates_drafting-geometry.md)
  - [`crates/electromagnetics-rf-sensing`](modules/crates_electromagnetics-rf-sensing.md)
  - [`crates/geodesy-spatial-analysis`](modules/crates_geodesy-spatial-analysis.md)
  - [`crates/gpu-command-profiler`](modules/crates_gpu-command-profiler.md)
  - [`crates/gpu-device-context`](modules/crates_gpu-device-context.md)
  - [`crates/kinetics-block-simulation`](modules/crates_kinetics-block-simulation.md)
  - [`crates/kinetics-continuum-multiphysics`](modules/crates_kinetics-continuum-multiphysics.md)
  - [`crates/kinetics-garment-draping`](modules/crates_kinetics-garment-draping.md)
  - [`crates/kinetics-motion-compositor`](modules/crates_kinetics-motion-compositor.md)
  - [`crates/kinetics-schedule-cpm`](modules/crates_kinetics-schedule-cpm.md)
  - [`crates/layout-box-autolayout`](modules/crates_layout-box-autolayout.md)
  - [`crates/layout-nodegraph-compiler`](modules/crates_layout-nodegraph-compiler.md)
  - [`crates/layout-paged-typesetter`](modules/crates_layout-paged-typesetter.md)
  - [`crates/layout-presentation-deck`](modules/crates_layout-presentation-deck.md)
  - [`crates/locus-geospatial-codecs`](modules/crates_locus-geospatial-codecs.md)
  - [`crates/locus-parametric-solver`](modules/crates_locus-parametric-solver.md)
  - [`crates/manifold-bim-csg`](modules/crates_manifold-bim-csg.md)
  - [`crates/media-isobmff`](modules/crates_media-isobmff.md)
  - [`crates/media-ogg`](modules/crates_media-ogg.md)
  - [`crates/media-test-support`](modules/crates_media-test-support.md)
  - [`crates/multimodal-synthesis-schema`](modules/crates_multimodal-synthesis-schema.md)
  - [`crates/pcm-device-output`](modules/crates_pcm-device-output.md)
  - [`crates/pixel-buffer-output`](modules/crates_pixel-buffer-output.md)
  - [`crates/pixel-window-render-adapter`](modules/crates_pixel-window-render-adapter.md)
  - [`crates/profile-authoring`](modules/crates_profile-authoring.md)
  - [`crates/radiance-camera-raw`](modules/crates_radiance-camera-raw.md)
  - [`crates/radiance-color-grading`](modules/crates_radiance-color-grading.md)
  - [`crates/raster-blend-operators`](modules/crates_raster-blend-operators.md)
  - [`crates/raster-layer-mask-adapter`](modules/crates_raster-layer-mask-adapter.md)
  - [`crates/raster-selection-morphology`](modules/crates_raster-selection-morphology.md)
  - [`crates/raster-spatial-operators`](modules/crates_raster-spatial-operators.md)
  - [`crates/surface-layered-codecs`](modules/crates_surface-layered-codecs.md)
  - [`crates/surface-pbr-studio`](modules/crates_surface-pbr-studio.md)
  - [`crates/terrain-brushes`](modules/crates_terrain-brushes.md)
  - [`crates/terrain-erosion-solvers`](modules/crates_terrain-erosion-solvers.md)
  - [`crates/timeline-editing`](modules/crates_timeline-editing.md)
  - [`crates/vector-compute-renderer`](modules/crates_vector-compute-renderer.md)
  - [`crates/vector-compute-shaders`](modules/crates_vector-compute-shaders.md)
  - [`crates/vector-cpu-render-adapter`](modules/crates_vector-cpu-render-adapter.md)
  - [`crates/vector-glyph-shaping`](modules/crates_vector-glyph-shaping.md)
  - [`crates/vector-gpu-render-adapter`](modules/crates_vector-gpu-render-adapter.md)
  - [`crates/vector-hybrid-render-adapter`](modules/crates_vector-hybrid-render-adapter.md)
  - [`crates/vector-hybrid-renderer`](modules/crates_vector-hybrid-renderer.md)
  - [`crates/vector-hybrid-shaders`](modules/crates_vector-hybrid-shaders.md)
  - [`crates/vector-locus-geometry`](modules/crates_vector-locus-geometry.md)
  - [`crates/vector-locus-pathops`](modules/crates_vector-locus-pathops.md)
  - [`crates/vector-radiance-trace`](modules/crates_vector-radiance-trace.md)
  - [`crates/vector-skia-render-adapter`](modules/crates_vector-skia-render-adapter.md)
  - [`crates/vector-surface-brush`](modules/crates_vector-surface-brush.md)
  - [`crates/video-h264-codec`](modules/crates_video-h264-codec.md)
  - [`crates/video-h264-preview`](modules/crates_video-h264-preview.md)
  - [`crates/video-prores-codec`](modules/crates_video-prores-codec.md)
