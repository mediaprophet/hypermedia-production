# `crates/pixel-window-render-adapter`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_An AnyRender WindowRenderer for rendering pixel buffers using the pixels crate_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Default for PixelsRendererOptions::default` | 0 | `crates/pixel-window-render-adapter/src/lib.rs:36` | - |  |
| `impl PixelsRendererOptions::new` | 0 | `crates/pixel-window-render-adapter/src/lib.rs:42` | - |  |
| `impl PixelsRendererOptions::base_color` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:49` | - |  |
| `impl PixelsRendererOptions::composite_alpha_mode` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:53` | - |  |
| `impl From<anyrender::RendererConfig> for PixelsRendererOptions::from` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:65` | - |  |
| `impl PixelsWindowRenderer<Renderer>::new` | 0 | `crates/pixel-window-render-adapter/src/lib.rs:91` | - |  |
| `impl PixelsWindowRenderer<Renderer>::with_renderer` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:95` | - |  |
| `impl PixelsWindowRenderer<Renderer>::with_options` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:104` | - |  |
| `impl PixelsWindowRenderer<Renderer>::with_options_and_renderer` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:113` | - |  |
| `impl RenderContext for PixelsWindowRenderer<Renderer>::try_register_custom_resource` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:129` | - |  |
| `impl RenderContext for PixelsWindowRenderer<Renderer>::unregister_resource` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:136` | - |  |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::is_active` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:146` | - |  |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::resume` | 5 | `crates/pixel-window-render-adapter/src/lib.rs:150` | - |  |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::complete_resume` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:187` | - |  |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::suspend` | 1 | `crates/pixel-window-render-adapter/src/lib.rs:191` | - |  |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::set_size` | 3 | `crates/pixel-window-render-adapter/src/lib.rs:195` | - |  |
| `impl WindowRenderer for PixelsWindowRenderer<Renderer>::render` | 2 | `crates/pixel-window-render-adapter/src/lib.rs:209` | - |  |
