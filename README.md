# Hypermedia production libraries

Repository: `mediaprophet/hypermedia-production`. Clone into a folder named **hypermedia-libraries** beside QualiaDB, Ecopunk and Synth to preserve existing path dependencies. Original per-crate names and notices are retained. This is source preservation/publication, not complete migration, fresh-clone compilation or cinema-quality acceptance.

# Hypermedia libraries

Shared native Rust libraries named by function, for Synth, Poet and other
QualiaDB applications. Each crate documents its actual source and responsibility.

Hypermedia is the umbrella for linked knowledge, editable sources, behaviours
and media resources. Functional crate names remain independent of this umbrella;
existing QualiaDB graph/identifier/resource owners remain authoritative.

QualiaDB remains the foundation for graph/inference, identifiers, scientific
and numerical owners, simulation, native resource codecs, provenance and
authority. This workspace contains extracted authoring capabilities and existing
compatibility behaviour; it does not replace those foundations.

No application shell, UI loop, standalone transport, generation provider or GPU
backend is required by the initial pure-library selection. Original source
workspaces remain intact. Original licence notices belong to each extraction.

Migration evidence and source-to-functional-name mappings are retained under
`migration/`. Workspace versioning describes the extraction, not an assertion of
complete source-suite parity or production asset acceptance. The current initial
crates do not themselves provide a complete species HMC workflow.

| Crate | Extracted responsibility | Current scope |
|---|---|---|
| `drafting-geometry` | Existing double-precision drafting curves, bulged polylines, intersections and tessellation | Retained compatibility semantics; overlaps with some Core numerical functions are documented, not replaced |
| `profile-authoring` | Bounded rounded-rectangle, circle, channel and polygon profiles | Existing native source-generation behaviour; mesh/simulation owners remain in QualiaDB |
| `raster-blend-operators` | Four source-specific normalized channel blend operators | Complements the existing QualiaDB image owner; no layer/alpha/colour-management engine |
| `terrain-brushes` | Bounded local height-tile strokes, COW deltas, history/replay/cancel | Local authoring preview; QualiaDB geospatial/M64/simulation and native project owners retained |
| `raster-selection-morphology` | Checked Euclidean selection growth, contraction and borders | Caller-owned coverage masks; serial default and optional parallel execution |
| `timeline-editing` | Rational media time and transition-span proposals/validation | Editorial editing; no codec, project store, playback or simulation owner |

`checks/consumer-smoke` verifies the small public library closure, including the
temporary dependency alias used to preserve existing Rust imports. It is not a
native Synth/Poet usability or asset-quality acceptance test.

Checks are coordinated and serial. To avoid a full application build, select the
individual crate or this small workspace. Native Poet rebuilding remains a
separate task after its required coding and consumer migrations are complete.

Future source additions must compare the actual selected operation to the
current libraries and QualiaDB owners before copying or replacing code.

Current migration records include [codec and container boundaries](docs/codec-boundaries.md).
Poet source now forwards its spatial profile/error and local terrain APIs to shared crates.
This is source/dependency adoption; full application compilation, UI/HMC workflow and
production acceptance remain separate qualification. Native Poet rebuilding is deferred.

## Dependency publication status

The required QualiaDB/Ecopunk/Aether engine updates are prepared locally but not pushed. See provenance/engine-dependencies.json. Their publication needs separate authorization; this source push does not establish a fresh-clone build.
