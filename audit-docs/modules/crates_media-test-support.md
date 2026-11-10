# `crates/media-test-support`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Test-only helpers shared by every FilmCraft crate (used as a dev-dependency only).

- [`oracle`]: find `ffmpeg` / `ffprobe` (env vars, `PATH`, well-known dirs) and skip — or, with
`FILMCRAFT_REQUIRE_ORACLES=1`, fail — when they are missing ([`require_ffmpeg!`] & co.).
- [`fixtures`]: the shared fixture directory (`<workspace>/target/fixtures/<crate>`) and
collision-free temporary names for concurrent generators.
- [`golden`]: RGBA8 images, PNG I/O, image-difference metrics and golden-file checks
(`FILMCRAFT_BLESS=1` regenerates references).

ffmpeg/ffprobe are external test oracles and fixture generators only; they are never linked
or shipped (AGENTS.md §2)._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `workspace_root` | 0 | `crates/media-test-support/src/fixtures.rs:11` | - | The workspace root (the directory holding the top-level `Cargo.toml`). |
| `fixtures_root` | 0 | `crates/media-test-support/src/fixtures.rs:18` | - | Root of all generated fixtures: `$FILMCRAFT_FIXTURES_DIR` or `<workspace>/target/fixtures`. |
| `fixtures_dir` | 1 | `crates/media-test-support/src/fixtures.rs:26` | - | `<fixtures root>/<sub>` (created). `sub` may contain `/` (e.g. `"opus/oracle"`). |
| `temp_path` | 1 | `crates/media-test-support/src/fixtures.rs:37` | - | A temporary sibling of `out` that no other thread or process will pick: the stem gets |
| `generate` | 2 | `crates/media-test-support/src/fixtures.rs:52` | - | Return `out` if it already exists (non-empty); otherwise run `make(tmp)` on a [`temp_path`] |
| `report` | 2 | `crates/media-test-support/src/fixtures.rs:77` | - | Print a `FIXTURE <made\|cached\|skipped> <name>` line (parsed by `cargo xtask fixtures`). |
| `generate_and_report` | 3 | `crates/media-test-support/src/fixtures.rs:88` | - | Run a fixture generator and [`report`] it: `cached` when every path in `outputs` existed |
| `root_is_the_workspace` | 0 | `crates/media-test-support/src/fixtures.rs:107` | - |  |
| `temp_paths_are_unique_and_keep_the_extension` | 0 | `crates/media-test-support/src/fixtures.rs:113` | - |  |
| `generate_is_cached_and_atomic` | 0 | `crates/media-test-support/src/fixtures.rs:124` | - |  |
| `impl Rgba8::new` | 3 | `crates/media-test-support/src/golden.rs:26` | - |  |
| `impl Rgba8::get` | 3 | `crates/media-test-support/src/golden.rs:30` | - |  |
| `impl Rgba8::opaque` | 1 | `crates/media-test-support/src/golden.rs:34` | - |  |
| `encode_png` | 1 | `crates/media-test-support/src/golden.rs:40` | - | Encode as PNG (RGB when fully opaque, else RGBA), maximum compression. |
| `decode_png` | 1 | `crates/media-test-support/src/golden.rs:61` | - | Decode a PNG (8-bit gray/RGB/RGBA, palette expanded) to RGBA8. |
| `read_png` | 1 | `crates/media-test-support/src/golden.rs:79` | - |  |
| `write_png` | 2 | `crates/media-test-support/src/golden.rs:84` | - |  |
| `impl std::fmt::Display for Diff::fmt` | 2 | `crates/media-test-support/src/golden.rs:109` | - |  |
| `diff` | 2 | `crates/media-test-support/src/golden.rs:115` | - | Compare two images of equal size. |
| `impl Tolerance::accepts` | 2 | `crates/media-test-support/src/golden.rs:165` | - |  |
| `blessing` | 0 | `crates/media-test-support/src/golden.rs:171` | - | Whether `FILMCRAFT_BLESS` asks to regenerate references. |
| `failures_dir` | 0 | `crates/media-test-support/src/golden.rs:176` | - | Where failing comparisons write `<name>.actual.png` / `<name>.diff.png`. |
| `diff_image` | 2 | `crates/media-test-support/src/golden.rs:181` | - | Difference image: \|a − b\| per channel ×8, opaque. |
| `today` | 0 | `crates/media-test-support/src/golden.rs:197` | - | Today's date (UTC) as YYYY-MM-DD, for attribution sidecars. |
| `sidecar_text` | 3 | `crates/media-test-support/src/golden.rs:213` | - | The attribution sidecar text for a generated golden image. |
| `attribution_row` | 2 | `crates/media-test-support/src/golden.rs:228` | - | The `ATTRIBUTION.md` table row for a golden image at workspace-relative `rel`. |
| `check_golden` | 5 | `crates/media-test-support/src/golden.rs:234` | - | Compare `actual` with the golden PNG at `golden` (or bless it). `title` and `generated_by` |
| `assert_golden` | 5 | `crates/media-test-support/src/golden.rs:275` | - | [`check_golden`] that panics on failure. |
| `ramp` | 3 | `crates/media-test-support/src/golden.rs:286` | - |  |
| `png_round_trip_rgb_and_rgba` | 0 | `crates/media-test-support/src/golden.rs:292` | - |  |
| `diff_metrics` | 0 | `crates/media-test-support/src/golden.rs:301` | - |  |
| `check_golden_accepts_within_tolerance_and_reports_failures` | 0 | `crates/media-test-support/src/golden.rs:317` | - |  |
| `date_and_sidecar_fields` | 0 | `crates/media-test-support/src/golden.rs:337` | - |  |
| `exe_names` | 1 | `crates/media-test-support/src/oracle.rs:30` | - |  |
| `is_file` | 1 | `crates/media-test-support/src/oracle.rs:34` | - |  |
| `find_tool_in` | 4 | `crates/media-test-support/src/oracle.rs:40` | - | Find `name` given an explicit override, a `PATH`-style list and extra directories. |
| `env_var` | 1 | `crates/media-test-support/src/oracle.rs:67` | - |  |
| `find_tool` | 1 | `crates/media-test-support/src/oracle.rs:72` | - | Find a tool using the process environment (uncached). |
| `ffmpeg` | 0 | `crates/media-test-support/src/oracle.rs:82` | - | Path of ffmpeg, if available (cached for the process). |
| `ffprobe` | 0 | `crates/media-test-support/src/oracle.rs:88` | - | Path of ffprobe, if available (cached for the process). |
| `oracles_required` | 0 | `crates/media-test-support/src/oracle.rs:94` | - | Whether `FILMCRAFT_REQUIRE_ORACLES` is set to a true value. |
| `skip` | 2 | `crates/media-test-support/src/oracle.rs:100` | - | Report a skipped test: prints `SKIPPED (<context>): <reason>`. With |
| `tool_or_skip` | 3 | `crates/media-test-support/src/oracle.rs:107` | - |  |
| `ffmpeg_or_skip` | 1 | `crates/media-test-support/src/oracle.rs:115` | - | ffmpeg, or `None` after reporting a skip (see [`skip`]). |
| `ffprobe_or_skip` | 1 | `crates/media-test-support/src/oracle.rs:120` | - | ffprobe, or `None` after reporting a skip (see [`skip`]). |
| `tmpdir` | 1 | `crates/media-test-support/src/oracle.rs:129` | - |  |
| `explicit_path_wins_and_bad_path_errors` | 0 | `crates/media-test-support/src/oracle.rs:136` | - |  |
| `path_search_finds_plain_and_exe_names` | 0 | `crates/media-test-support/src/oracle.rs:149` | - |  |
| `known_dirs_are_the_fallback` | 0 | `crates/media-test-support/src/oracle.rs:162` | - |  |
| `encode_f32` | 2 | `crates/media-test-support/src/wav.rs:6` | - | Encode planar `f32` channels (equal lengths) as a 32-bit float WAV file. |
| `write_f32` | 3 | `crates/media-test-support/src/wav.rs:33` | - | Write planar `f32` channels to `path` as a float WAV. |
| `header_sizes` | 0 | `crates/media-test-support/src/wav.rs:40` | - |  |
