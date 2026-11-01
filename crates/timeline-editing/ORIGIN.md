# Origin and adaptation record

Adapted from the MIT OR Apache-2.0 FilmCraft workspace at
`C:/github/poet-filmcraft`:

- `crates/time/src/lib.rs`: integer `Tick`, exact rational `FrameRate`, frame
  boundary conversion, and tick/unit conversion. The staged API is a bounded
  adaptation; it omits serde and FilmCraft project-specific helpers.
- `crates/edit/src/transitions.rs`: clip-relative transition bounds, alignment
  inference, checked span validation, start placement, and edge/body drag
  preview. The staged API returns validated values and leaves mutation/history
  to its host.
- `crates/engine/src/commands.rs`, `sequence.setTransition`: operation caller
  used to trace the source command/session boundary.
- `crates/ui-egui/src/panels/transition_controls.rs`: timeline control caller
  used to trace the user-facing editing path.

The FilmCraft source files and manifests were read-only during this adaptation.
FilmCraft's `LICENSE-MIT`, `LICENSE-APACHE`, and `NOTICE` are copied beside the
crate. The stage has no external dependencies. It does not copy FilmCraft's
project/session/engine/UI, renderer, codecs, media IO, serialization, GPU, or
transport layers.

The crate does not prescribe or create a persisted format. Container timebase
interpretation, source timecode identity, import/export policy, and serializer
compatibility require a separate review of the existing codec/format owners.
