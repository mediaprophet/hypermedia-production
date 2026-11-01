# Profile regression adaptation

Origin: `C:/github/poet/crates/spatial-authoring/src/tests.rs`, exact full original
retained as original-spatial-tests.rs with SHA-256 pins. MIT selected.

| Active extracted test | Original test and adaptation |
|---|---|
| malformed_profiles_are_rejected | Copies the OpenChannel invalid wall and Circle NaN cases from malformed_profiles_and_primitive_budgets_are_rejected; sphere/torus remain with the original Core caller. |
| changed_profile_parameters_change_section_dimensions_and_area | Same profiles and width-edit intent as changed_profile_parameters_change_real_geometry, asserts section bounds/area rather than downstream mesh output. Area/dimension assertions added at extracted boundary. |
| open_channel_section_retains_wall_void_and_refuses_caller_budget | Same section/analytic area as concave_channel_caps_preserve_wall_void_and_volume, checks profile area rather than mesh volume/watertightness. Adds explicit caller-budget refusal. |

These are adapted regressions with explicit scope, not evidence mesh generation,
cap topology, frames, materials or native workflow have passed. Worker did not
execute them; coordinator owns serial Cargo checks.
