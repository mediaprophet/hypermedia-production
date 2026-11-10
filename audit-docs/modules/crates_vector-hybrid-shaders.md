# `crates/vector-hybrid-shaders`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_WESL shader sources linked to WGSL and optionally compiled to GLSL for `vello_gpu`._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `main` | 0 | `crates/vector-hybrid-shaders/build.rs:56` | - |  |
| `load_shader_infos` | 1 | `crates/vector-hybrid-shaders/build.rs:70` | 3d |  |
| `load_shader_names` | 1 | `crates/vector-hybrid-shaders/build.rs:82` | 3d |  |
| `compile_shader` | 2 | `crates/vector-hybrid-shaders/build.rs:98` | 3d |  |
| `generate_compiled_shaders_module` | 1 | `crates/vector-hybrid-shaders/build.rs:127` | 3d |  |
| `generate_wgsl_shader_module` | 2 | `crates/vector-hybrid-shaders/build.rs:180` | 3d |  |
| `compile_wgsl_shader` | 5 | `crates/vector-hybrid-shaders/src/compile.rs:22` | 3d | Compiles the given wgsl source into GLSL using [naga]. |
| `compile_stage` | 5 | `crates/vector-hybrid-shaders/src/compile.rs:61` | 3d |  |
| `validate` | 1 | `crates/vector-hybrid-shaders/src/compile.rs:109` | - |  |
| `every_shipped_shader_passes_the_lint` | 0 | `crates/vector-hybrid-shaders/src/lib.rs:24` | 3d |  |
| `lint` | 2 | `crates/vector-hybrid-shaders/src/lint/mod.rs:26` | 3d | Runs every WGSL shader lint over `module` and panics with a single aggregated |
| `check` | 1 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:10` | - |  |
| `check_function` | 3 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:34` | - |  |
| `lint_source` | 1 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:48` | - |  |
| `accepts_static_indexing` | 0 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:56` | - |  |
| `rejects_dynamic_indexing_in_helper_function` | 0 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:74` | - |  |
| `rejects_dynamic_indexing_in_entry_point` | 0 | `crates/vector-hybrid-shaders/src/lint/no_dynamic_indexing.rs:92` | - |  |
| `check` | 1 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:27` | - | Detects user-defined struct values reachable from the `@fragment` entry point. |
| `check_function` | 4 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:78` | - |  |
| `base_variable_load` | 3 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:143` | - | If `pointer` is a direct load of a `GlobalVariable` or `LocalVariable` (with no |
| `collect_called_functions` | 2 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:175` | - |  |
| `is_struct` | 2 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:200` | - |  |
| `type_name` | 2 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:204` | - |  |
| `parse` | 1 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:313` | - |  |
| `accepts_fragment_shader_without_struct_values` | 0 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:318` | 3d |  |
| `rejects_struct_local_variable_in_fragment_shader` | 0 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:324` | 3d |  |
| `rejects_helper_function_returning_struct` | 0 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:330` | - |  |
| `rejects_struct_as_fragment_entry_point_input` | 0 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:338` | - |  |
| `rejects_whole_uniform_struct_load_in_fragment_shader` | 0 | `crates/vector-hybrid-shaders/src/lint/no_structs_in_fragment.rs:348` | 3d |  |
| `main` | 0 | `crates/vector-hybrid-shaders/src/main.rs:16` | - |  |
| `main` | 0 | `crates/vector-hybrid-shaders/src/main.rs:48` | - |  |
| `minify_wgsl` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:31` | - | Renames every non-entry-point declaration in linked WGSL and removes redundant whitespace. |
| `minify_whitespace` | 2 | `crates/vector-hybrid-shaders/src/minify.rs:49` | 3d | Removes comments and whitespace that are unnecessary between shader tokens. |
| `append_minified_code` | 2 | `crates/vector-hybrid-shaders/src/minify.rs:80` | - | Appends minified code without merging tokens across the append boundary. |
| `rename_module` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:97` | - | Assigns short names throughout a module and returns the global-name reverse map. |
| `rename_function` | 2 | `crates/vector-hybrid-shaders/src/minify.rs:149` | - | Renames a function declaration, its arguments, locals, and named expressions. |
| `rename_optional` | 2 | `crates/vector-hybrid-shaders/src/minify.rs:166` | - | Replaces an optional Naga IR declaration name with the next short name, leaving unnamed |
| `validate` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:175` | - | Validates the renamed module and produces the metadata required by Naga's writer. |
| `impl NameAllocator::next` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:191` | - | Returns the next spreadsheet-style name: `A` through `Z`, then `AA`, `AB`, and so on. |
| `minify_code` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:212` | 3d | Removes comments and optional whitespace while preserving shader token boundaries. |
| `copy_quoted` | 3 | `crates/vector-hybrid-shaders/src/minify.rs:264` | - | Copies one quoted value verbatim, including escaped characters. |
| `first_char` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:284` | - | Returns the first character of a known non-empty source section. |
| `last_char` | 1 | `crates/vector-hybrid-shaders/src/minify.rs:291` | - | Returns the last character of a known non-empty source section. |
| `needs_separator` | 2 | `crates/vector-hybrid-shaders/src/minify.rs:301` | 3d | Reports whether removing whitespace would merge two adjacent shader tokens. |
| `allocates_short_deterministic_names` | 0 | `crates/vector-hybrid-shaders/src/minify.rs:330` | - |  |
| `minifies_root_and_imported_declarations` | 0 | `crates/vector-hybrid-shaders/src/minify.rs:341` | - |  |
| `keeps_required_token_separators` | 0 | `crates/vector-hybrid-shaders/src/minify.rs:380` | - |  |
| `impl ReflectionMap::new` | 3 | `crates/vector-hybrid-shaders/src/types.rs:27` | - | Create a new `ReflectionMap` given the [`naga`] compile info. |
| `impl CompiledGlsl::to_generated_code` | 2 | `crates/vector-hybrid-shaders/src/types.rs:95` | 3d | Generate Rust modules that contain vertex and fragment source code. Uniforms and texture |
| `construct_reflection_map_from_vertex` | 0 | `crates/vector-hybrid-shaders/src/types.rs:212` | 3d |  |
| `construct_reflection_map_from_fragment` | 0 | `crates/vector-hybrid-shaders/src/types.rs:262` | - |  |
