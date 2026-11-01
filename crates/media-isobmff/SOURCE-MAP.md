# Source and adaptation

This crate stages the MIT-licensed FilmCraft ISO-BMFF implementation from
`C:/github/poet-filmcraft/crates/isobmff`; its original license is retained in
`LICENSE-FILMCRAFT-MIT` and the original source files are pinned in the adjacent
`isobmff-source-pins.tsv`. The functional crate name and dependencies target
the existing shared `bitstream-codec` and `media-test-support` crates. Original
tests are retained with the crate import renamed.

The demuxer adds caller-selected input, header, track, fragment and table
limits before corresponding allocation, an optional audio composition-table
refusal, and a check that the declared sample-entry count was actually parsed.
The default entry point keeps the original broad limits, apart from rejecting
the malformed sample-entry count. Nested box traversal is bounded by input size
but has no separate operation budget; heap and execution time are unmeasured.
This is staged source, not an executed or installed qualification.
