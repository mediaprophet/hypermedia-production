# Terrain brushes

Cold terrain authoring extracted from current Poet `geo_studio/cow_delta_undo`.
Provides existing falloff/brush operations, local tiled metre height previews,
copy-on-write tile deltas, commit/undo/redo, journal replay and a new cancel path.
There are no Cargo dependencies, features, build scripts, backend, GPU device,
application shell, network client or persistence service.

Source grid admission and transaction defects are repaired explicitly; original
source, source tests and MIT notice remain in provenance. See adaptation/ for the
replacement grid implementation and provenance/changes.md for the exact semantic
changes. Existing pure vertex formulas are preserved. The full original module
is compiled only as a test oracle. Source tests do not certify the old defects.

Local positions, height, cell size, brush radius and terrace step are metres.
Strength retains source semantics: Raise/Lower use height deltas, while smoothing,
flattening, terracing and HydraulicDrop clamp the effective interpolation weight.
HydraulicDrop is the inherited local averaging brush, not a physical erosion
simulation. Tile-edge smoothing clamps to the current tile and is not a halo-
aware cross-tile simulation. Host-origin/large-coordinate precision needs its
existing geographic adapter and separate qualification.

`byte_budget` admits a conservative tile-reference payload estimate and StrokeSample values plus
conservative prepared tile successors before mutation. It excludes HashMap/Vec/
String capacity, Arc/control blocks, allocator overhead, external retained Arcs
and process memory. Explicit tile, tile-size, journal/history, label and per-
sample vertex-work limits additionally bound logical structures. This is neither
zero-allocation nor exact total-heap accounting. The estimator itself allocates
no scratch: it reads current/baseline map lengths and at most 256 history-entry
patch/sample Vec lengths, without walking any patch/sample/tile payload. Duplicate
Arc references are deliberately charged again; this may refuse work that a
distinct-allocation counter would admit. Commit reserves the added history
reference charge before changing preview/history. Rust Arc/control allocation
still uses the allocator; no allocator-failure proof is claimed.

Invalid/unsupported sample and resource errors leave logical tiles, history and
journal unchanged. A failed commit leaves its active preview available to retry
or cancel; cancel restores original Arcs and leaves history/IDs unchanged. Replay
refusal cancels its own whole stroke. Read-only history/tile APIs replace public
mutable fields. Those encapsulation and stricter admission changes are explicit
compatibility changes. Identifiers in this local journal are not grants or global
referent assertions.
`discard_local_history()` explicitly discards local undo and redo, is forbidden
during an active stroke, and retains current tile Arcs/heights and the monotonic
next commit identifier. It restores commit capacity after the 256-entry cap.
The host decides when discarding local history is appropriate; this is not a
canonical native-project rebase.

StrokeSample retains repr(C) and the inherited 64-byte layout test; it is an
in-memory sample, not a new canonical wire format or serializable project. No
raw byte casting, codec, serializer or resource header is introduced. Host source,
authoritative journal/revision, roles, cancellation/stale job admission and HMC
publication stay with native owners. Current Core terrain DAG, qualia-geospatial
M64 producer/reader, QualiaDB codec registries and qualia-sim remain authoritative.

The caller uses `tiles()`, `tile_arc`, `sample_height`, read-only undo/redo history
and active samples for a bounded proposed edit. `load_tile` imports an exactly
sized finite decoded metre tile before local
history begins; shape/nonfinite errors refuse without changing current source.
No nodata value or decode policy is invented by that API. The native host must
compare its baseline/source/resource revision, convert finite metres to the existing integer-
millimetre/nodata policy where appropriate, and submit through the canonical
resource edit service. This library does not commit a native project.

`examples/local-stroke.rs` is an actual independent operation consumer, not a
claim of native human usability. Source tests, adaptation regressions and source
oracle are prepared for coordinator execution. Extraction worker ran no Cargo.
