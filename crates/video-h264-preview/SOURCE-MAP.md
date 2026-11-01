# Source and license

This adapter uses the existing QualiaDB `mp4` demux dependency and the
functionally named FilmCraft-derived `video-h264-codec`, `bitstream-codec` and
`color-transforms` crates. FilmCraft is MIT-licensed; the full upstream notice
is in `LICENSE-FILMCRAFT-MIT`. The synthetic I_PCM fixture and color adaptation
are based on its `crates/h264/src/synth_tests.rs` and `crates/codecs/src/video.rs`.
The migration package retains original and staged source SHA-256 pins in
`codec-source-pins.tsv`; these pins must be preserved with the shared crates.
