# `crates/multimodal-synthesis-schema`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_`multimodal-synthesis-schema`

UI-agnostic multimodal synthesis schemas:
- Comprehensive multimodal model capability catalog (`catalog`)
- Reference media attachments and frame coordination (`references`)
- Generation job feed status and deterministic progress/ETA estimation (`job_feed`)
- 2D/3D composition stages and inpainting mask normalization (`stage`)
- 3D Gaussian Splatting and radiance field cost calculation (`splat_cost`)_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl MediaKind::label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:49` | - |  |
| `impl AudioCaps::has_tuning` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:121` | - |  |
| `impl AudioCaps::sample_rate` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:125` | - |  |
| `impl ModelInfo::batch_options` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:134` | - |  |
| `impl ModelInfo::valid_batch` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:142` | - |  |
| `impl ModelInfo::resolve` | 3 | `crates/multimodal-synthesis-schema/src/catalog.rs:153` | - |  |
| `impl ModelInfo::supports_reference_mode` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:163` | - |  |
| `impl ModelInfo::duration_range` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:167` | - |  |
| `impl ModelInfo::resolve_duration` | 3 | `crates/multimodal-synthesis-schema/src/catalog.rs:183` | - |  |
| `impl Catalog::set_image` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:228` | - |  |
| `impl Catalog::set_video` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:233` | - |  |
| `impl Catalog::set_audio` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:238` | - |  |
| `impl Catalog::find` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:243` | - |  |
| `impl Catalog::display_name` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:251` | - |  |
| `impl Catalog::creator` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:256` | - |  |
| `display_name` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:262` | - |  |
| `default_prompt_max` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:313` | - |  |
| `description` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:337` | - |  |
| `expected_secs` | 2 | `crates/multimodal-synthesis-schema/src/catalog.rs:365` | - |  |
| `creator_for` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:381` | - |  |
| `family` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:408` | - |  |
| `family_rank` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:440` | - |  |
| `grouped` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:472` | - |  |
| `image_page_models` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:490` | - |  |
| `video_page_models` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:501` | - |  |
| `aspect_label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:511` | - |  |
| `aspect_proportion` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:536` | - |  |
| `resolution_label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:555` | - |  |
| `resolution_is_hd` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:570` | - |  |
| `quality_label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:574` | - |  |
| `sample_rate_label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:587` | - |  |
| `bitrate_label` | 1 | `crates/multimodal-synthesis-schema/src/catalog.rs:595` | - |  |
| `batch_and_duration_resolution` | 0 | `crates/multimodal-synthesis-schema/src/catalog.rs:608` | - |  |
| `aspect_proportions_and_labels` | 0 | `crates/multimodal-synthesis-schema/src/catalog.rs:624` | - |  |
| `display_name_and_creator` | 0 | `crates/multimodal-synthesis-schema/src/catalog.rs:632` | - |  |
| `pending_status` | 2 | `crates/multimodal-synthesis-schema/src/job_feed.rs:48` | - | Computes estimated progress percentage (0..=99) and a human-readable remaining time label. |
| `time_ago` | 2 | `crates/multimodal-synthesis-schema/src/job_feed.rs:76` | - | Relative human-readable time formatting: "just now", "5 minutes ago", etc. |
| `progress_and_eta_calculation` | 0 | `crates/multimodal-synthesis-schema/src/job_feed.rs:94` | - |  |
| `relative_time_formatting` | 0 | `crates/multimodal-synthesis-schema/src/job_feed.rs:118` | - |  |
| `impl RefKind::noun` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:19` | - |  |
| `impl RefKind::from_path` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:27` | - |  |
| `impl RefMedia::uploading` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:59` | - |  |
| `impl RefMedia::from_library` | 5 | `crates/multimodal-synthesis-schema/src/references.rs:71` | - |  |
| `impl References::is_empty` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:110` | - |  |
| `impl References::clear` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:118` | - |  |
| `impl References::list` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:122` | - |  |
| `impl References::list_mut` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:130` | - |  |
| `impl References::total_secs` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:138` | - |  |
| `impl References::any_uploading` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:142` | - |  |
| `impl References::find_mut` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:152` | - |  |
| `impl References::remove` | 2 | `crates/multimodal-synthesis-schema/src/references.rs:162` | - |  |
| `impl References::swap_frames` | 1 | `crates/multimodal-synthesis-schema/src/references.rs:174` | - |  |
| `routes_files_by_extension` | 0 | `crates/multimodal-synthesis-schema/src/references.rs:184` | - |  |
| `reference_lifecycle_and_frames` | 0 | `crates/multimodal-synthesis-schema/src/references.rs:192` | - |  |
| `impl RadianceModel::api_name` | 1 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:30` | - |  |
| `calculate_splat_cost` | 2 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:71` | - | Calculate the cost of a 3D Gaussian Splat / radiance field generation. |
| `credits_for` | 2 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:81` | - | Returns the credit cost for a given model and input type. |
| `credits_to_usd_cents` | 1 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:108` | - | Converts credits to US dollar cents. |
| `mini_and_draft_costs` | 0 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:117` | - |  |
| `plus_and_1p1_costs` | 0 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:124` | - |  |
| `dollar_conversions` | 0 | `crates/multimodal-synthesis-schema/src/splat_cost.rs:131` | - |  |
| `impl SphericalAdjustment::new` | 3 | `crates/multimodal-synthesis-schema/src/stage.rs:14` | - |  |
| `impl Default for StageCamera::default` | 0 | `crates/multimodal-synthesis-schema/src/stage.rs:33` | - |  |
| `normalize_mask_bytes` | 4 | `crates/multimodal-synthesis-schema/src/stage.rs:55` | - | Normalizes raw RGBA8 or Grayscale image bytes into a deterministic binary mask (255 = target, 0 = ignore). |
| `normalize_alpha_mask` | 0 | `crates/multimodal-synthesis-schema/src/stage.rs:140` | - |  |
| `camera_spherical_adjustment` | 0 | `crates/multimodal-synthesis-schema/src/stage.rs:160` | 3d |  |
