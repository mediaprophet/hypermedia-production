# AAC codec source and boundary

`src/` and `tests/` derive from `C:/github/poet-filmcraft/crates/aac`,
FilmCraft 0.5.0. `LICENSE-FILMCRAFT-MIT` preserves its MIT notice. The
functional package name is `audio-aac-codec`; source imports use the existing
`bitstream-codec` through the original dependency alias, and testkit imports
use the existing `media-test-support` package through an alias. Test imports
and the doctest were mechanically changed from `filmcraft_aac` to the new
library identifier. Runtime codec mathematics and data structures have not
been rewritten. The upstream test suite is preserved in `tests/`; its external
ffmpeg oracle cases remain optional and have not been executed.

This library does not own HMC, timeline authority, device output, MP4 demux,
source state or user interface. Its AAC-LC decoder can return only the core of
HE-AAC; callers needing faithful playback must reject SBR/PS rather than claim
that output is complete. The bounded native ADTS profile in this stage explicitly
rejects SBR and is not an MP4 audio-track integration.
