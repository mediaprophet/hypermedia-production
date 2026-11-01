# Dependency closure

Default runtime dependency closure is empty and serial. The optional `parallel` feature adds direct `rayon = "1"` on non-WASM targets and enables the source's parallel row traversal. On `wasm32`, operations remain serial. This extraction does not depend on `photocraft-geom`, `photocraft-color`, `photocraft-raster`, serde, FFT, UI, document, image-store, or compositor crates.

No Cargo metadata or build was run for this staged crate. Root coordinator owns manifest integration and serial dependency/check policy.
