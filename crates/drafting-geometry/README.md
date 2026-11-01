# Drafting geometry

Functional rename of the existing `poet-geometry` / `cadcraft-geom` library.
All inherited Rust source and source tests are copied byte-for-byte. Public APIs
retain f64 vectors, affine transforms, bounds, curves, bulged polyline semantics,
NURBS splines, region operations, intersections and chord-tolerance tessellation.
Angles are radians, counterclockwise from +X; drawing lengths are caller units.

This is an existing drafting compatibility closure, not a replacement for
QualiaDB geometry. Core already owns computational geometry, NURBS evaluation,
fitting, offsets, triangulation, mesh and transform operations. A future adapter
must demonstrate preserved CAD behavior before removing this inherited closure.
No renderer, host, document authority, filesystem effect or JavaScript runtime
is added. `serde` and its derive dependency closure are the sole external dependency.

MIT option selected from the origin MIT OR Apache-2.0 grant. See LICENSE-MIT,
provenance/original-NOTICE, original attribution and source-map.json. The origin
notice describes excluded dependencies and assets too; those are not imported.
This staged copy has not been compiled or executed by its extraction worker.
