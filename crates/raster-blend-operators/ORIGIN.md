# Origin and migration scope

This staged crate is a bounded source extraction from PhotoCraft's public
compositing implementation. It retains the four channel formulas and the
edge-case regression intent listed below. The source workspace remains intact.

- Origin: `C:/github/poet-photocraft`
- Source implementation: `crates/compose/src/psblend.rs`, functions
  `vivid_light_ps`, `color_burn`, `color_dodge`, `vivid_light_generic`, and
  `hard_mix_ps` (source file SHA-256 recorded in the parent audit).
- Source callers: `crates/compose/src/lib.rs` routes document pixel blending
  through `psblend::composite`; `crates/compose/src/effects.rs` calls
  `psblend::composite` for effects and gamma-aware text overlays. The selected
  channel helpers feed public `psblend::blend_rgb`; `blend_rgb_fill` and
  `composite_fill` are outside this first narrow extraction.
- API adaptation: the application `BlendMode` and `photocraft-color` dependency
  are replaced with a four-variant `BlendOperator` for this staged channel-only
  seam. Formula branches and selected edge cases follow the source. This is not
  a byte-for-byte file copy and is not yet qualified as behavior-preserving.
- Test oracle: `src/migration_oracle.rs` preserves the five selected source
  helper bodies plus `EDGE` exactly from `psblend.rs:17-72` at the pinned hash.
  Test-only grid and boundary comparisons assert bit equality with those
  helpers. They were not run; Fill/Opacity/alpha/gamma/HDR/Lab and full document
  compositor behavior remain outside this stage.
- Source terms: dual licensed MIT OR Apache-2.0. The corresponding root license
  texts and complete root NOTICE were copied here. Review third-party notices
  before widening the copied source scope.
- QualiaDB comparison: `crates/qualia-core-db/src/hypermedia_authoring/image.rs`
  and `image_composite.rs` already own editable RGBA8 layer composition and
  ten generic modes. Their composition runs in encoded-sample RGB8. This staged
  crate owns none of those concerns and supplies only the four source-specific
  normalized channel operators absent from that mode enum.

No shared repository manifest, source workspace, or consumer was changed. No
Cargo command, test, build, or runtime check was run. The included tests are
preserved source-derived regression cases and migration comparisons, not
executed results.
