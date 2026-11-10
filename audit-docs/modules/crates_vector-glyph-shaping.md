# `crates/vector-glyph-shaping`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_# Vector Glyph Shaping & Typography Engine

A pure Rust, UI-agnostic categorical typography domain crate providing OpenType
feature selection, UAX #9 Bidirectional algorithm run reordering, Knuth-Plass
optimal total-fit paragraph composition, syllable hyphenation, area text /
threaded frame / text-on-path layout, and TrueType/CFF font subsetting.

Grounded in international human rights guarantees of linguistic diversity,
artistic expression, and cultural heritage (UDHR Articles 19 & 27; ICESCR Article 15)
and Informatics Fiduciary duties._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `bidi_class` | 1 | `crates/vector-glyph-shaping/src/bidi.rs:25` | - | Character classification according to Unicode bidirectional rules. |
| `impl BidiRun::is_rtl` | 1 | `crates/vector-glyph-shaping/src/bidi.rs:64` | - |  |
| `resolve_bidi_runs` | 2 | `crates/vector-glyph-shaping/src/bidi.rs:70` | - | Resolve paragraph direction and decompose text into directional runs. |
| `reorder_visual` | 1 | `crates/vector-glyph-shaping/src/bidi.rs:137` | - | Reorder run indices from logical reading order into visual left-to-right display order. |
| `plain_english_is_ltr` | 0 | `crates/vector-glyph-shaping/src/bidi.rs:172` | - |  |
| `arabic_text_is_rtl` | 0 | `crates/vector-glyph-shaping/src/bidi.rs:181` | - |  |
| `mixed_ltr_rtl_reordering` | 0 | `crates/vector-glyph-shaping/src/bidi.rs:189` | - |  |
| `impl SGlyph::char_glyph` | 1 | `crates/vector-glyph-shaping/src/composer.rs:30` | - |  |
| `impl SGlyph::space_glyph` | 1 | `crates/vector-glyph-shaping/src/composer.rs:37` | - |  |
| `impl Default for Params::default` | 0 | `crates/vector-glyph-shaping/src/composer.rs:63` | - |  |
| `fitness` | 2 | `crates/vector-glyph-shaping/src/composer.rs:82` | - |  |
| `compose` | 5 | `crates/vector-glyph-shaping/src/composer.rs:96` | - | Knuth-Plass line composition over glyph slice `g`. |
| `compose_empty_paragraph` | 0 | `crates/vector-glyph-shaping/src/composer.rs:239` | - |  |
| `compose_single_line_fit` | 0 | `crates/vector-glyph-shaping/src/composer.rs:246` | - |  |
| `impl Default for OtFeatures::default` | 0 | `crates/vector-glyph-shaping/src/features.rs:37` | - |  |
| `impl OtFeatures::from_tags` | 1 | `crates/vector-glyph-shaping/src/features.rs:55` | - | Parse a feature list like `["dlig", "smcp", "-liga"]`. |
| `impl OtFeatures::to_tags` | 1 | `crates/vector-glyph-shaping/src/features.rs:83` | - | List of tag strings that differ from default settings. |
| `impl OtFeatures::known_tag` | 1 | `crates/vector-glyph-shaping/src/features.rs:105` | - | Check if a tag name is recognized. |
| `impl OtFeatures::should_keep_ligatures` | 2 | `crates/vector-glyph-shaping/src/features.rs:114` | - | Determine if standard ligatures should be kept given tracking value. |
| `parse_and_roundtrip_tags` | 0 | `crates/vector-glyph-shaping/src/features.rs:127` | - |  |
| `tracking_ligature_limits` | 0 | `crates/vector-glyph-shaping/src/features.rs:140` | - |  |
| `store` | 0 | `crates/vector-glyph-shaping/src/hyphen.rs:24` | - |  |
| `parse_list` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:31` | - |  |
| `set_hyphenation_exceptions` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:50` | - |  |
| `hyphenation_exceptions` | 0 | `crates/vector-glyph-shaping/src/hyphen.rs:54` | - |  |
| `parse_exception` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:58` | - |  |
| `normalize_word` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:83` | - |  |
| `exception_for` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:87` | - |  |
| `is_vowel` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:91` | - |  |
| `onset_pair` | 2 | `crates/vector-glyph-shaping/src/hyphen.rs:95` | - |  |
| `hyphen_points` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:107` | - | Allowed hyphenation points in `word` as char indices (hyphen inserted before char). |
| `hyphenate_word` | 1 | `crates/vector-glyph-shaping/src/hyphen.rs:142` | - |  |
| `syllable_splitting_rules` | 0 | `crates/vector-glyph-shaping/src/hyphen.rs:159` | - |  |
| `short_words_do_not_hyphenate` | 0 | `crates/vector-glyph-shaping/src/hyphen.rs:166` | - |  |
| `impl Default for AreaTextOptions::default` | 0 | `crates/vector-glyph-shaping/src/layout.rs:35` | - |  |
| `layout_area_text` | 3 | `crates/vector-glyph-shaping/src/layout.rs:65` | 2d | Lay out simple area text into a bounded rectangle. |
| `impl Default for TextOnPathOptions::default` | 0 | `crates/vector-glyph-shaping/src/layout.rs:194` | - |  |
| `layout_text_on_path` | 3 | `crates/vector-glyph-shaping/src/layout.rs:205` | - | Lay out characters along a sequence of 2D curve points. |
| `area_text_layout_wraps_lines` | 0 | `crates/vector-glyph-shaping/src/layout.rs:266` | - |  |
| `text_on_path_positions_glyphs` | 0 | `crates/vector-glyph-shaping/src/layout.rs:275` | - |  |
| `full_typography_pipeline_smoke` | 0 | `crates/vector-glyph-shaping/src/lib.rs:41` | - |  |
| `calc_table_checksum` | 1 | `crates/vector-glyph-shaping/src/subset.rs:18` | - | Calculate OpenType table checksum (sum of big-endian u32 words). |
| `parse_sfnt_directory` | 1 | `crates/vector-glyph-shaping/src/subset.rs:34` | - | Parse the SFNT table directory from raw font bytes. |
| `build_sfnt_font` | 2 | `crates/vector-glyph-shaping/src/subset.rs:91` | - | Assemble a valid standalone SFNT font binary from a set of table records. |
| `table_checksum_calculation` | 0 | `crates/vector-glyph-shaping/src/subset.rs:143` | - |  |
| `build_and_parse_sfnt_roundtrip` | 0 | `crates/vector-glyph-shaping/src/subset.rs:150` | - |  |
| `story` | 1 | `crates/vector-glyph-shaping/src/thread.rs:32` | - | Join all frame slices back into the full story text. |
| `distribute` | 2 | `crates/vector-glyph-shaping/src/thread.rs:49` | - | Distribute a sequence of story runs across linked text frames in thread order. |
| `distribute_and_rejoin_story` | 0 | `crates/vector-glyph-shaping/src/thread.rs:114` | - |  |
