# timeline-editing

Dependency-free exact media-time and transition-span primitives adapted from
FilmCraft. The crate owns editorial frame/sample time conversion, clip-span
bounds, alignment inference, duration/alignment placement, and drag previews.
The caller retains authority over documents, roles, history, persistence, and
commit/reject behavior.

It does not define a media container format, codec, source-import policy,
serializer, animation graph, project model, renderer, or playback runtime.
Persisted timecode/source-import semantics remain with the existing codec and
format owners until those owners are reviewed. Editorial ticks are separate
from QualiaDB simulation time.

The regression cases are in `src/timebase.rs` and `src/transition.rs` under
`cfg(test)`. They have not been run in this stage. See the migration audit for
source pins, operation comparisons, and the native Poet integration seam.

FilmCraft's MIT and Apache-2.0 license texts and NOTICE are preserved in this
directory. `ORIGIN.md` identifies adapted source files and exclusions.
