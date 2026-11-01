# CPAL source and Ecopunk modification

This is the locally cached CPAL **0.15.3** crate, from the published RustAudio
CPAL source, retained under its existing Apache-2.0 `LICENSE`. Ecopunk adds one
WASAPI `StopAndReset` command in `src/host/wasapi/stream.rs`, and exposes an
acknowledged `stop_and_reset` method on the concrete WASAPI stream. Other CPAL
backends and codecs are unchanged. The method acknowledges the actual client
stop/reset; it does not prove removal of samples already rendered downstream.

Poet's proposed workspace `[patch.crates-io]` applies this same CPAL package to
all compatible CPAL consumers resolved in the Poet workspace, including any
transitive QualiaDB consumers. QualiaDB's standalone workspace is not changed by
that patch. Its effect on each consumer, all other backends, dependency locking,
and real devices remains unverified until the deferred serial checks.
