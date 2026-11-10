# `crates/terrain-erosion-solvers`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Deterministic geomorphological heightfield erosion and surface modeling solvers.

Provides pure mathematical 2D heightfield solvers for physical terrain processes:
- Multi-scale thermal slope relaxation and scree synthesis (`thermal_shaper`)
- Particle-based hydraulic droplet erosion and sediment transport (`hydraulic_droplet`)
- Hydrological priority-flood depression conditioning and D8 flow routing (`drainage_pyramid`)
- Elevation-dependent snowfall, freeze/thaw, and avalanche mass transport (`snow_accumulation`)
- Hydrostatic water-table lake filling and coastal shoreline smoothing (`lacustrine_fill`)
- Planetary impact crater cavity and ejecta rim lip morphology (`crater_morphology`)
- Stratigraphic stepped terrace rock layer modulation (`fractal_terrace`)

Grounded in international human rights instruments (UDHR Article 27; ICESCR Article 15)
and Informatics Fiduciary duties ensuring scientific rigor, deterministic simulation,
and moral attribution in environmental modeling._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Default for CraterfieldConfig::default` | 0 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:19` | - |  |
| `lcg` | 1 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:43` | numerics | Linear congruential generator step. |
| `crater_profile` | 2 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:48` | - | Profile calculation for relative crater elevation offset at `normalized_dist` (0 = center, 1 = outer rim lip). |
| `apply_crater` | 6 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:66` | - | Apply a single impact crater cavity and raised rim onto a 2D heightfield. |
| `generate_craterfield` | 4 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:112` | - | Generate a multi-crater planetary field using deterministic cell distribution. |
| `single_crater_depresses_center_and_elevates_rim` | 0 | `crates/terrain-erosion-solvers/src/crater_morphology.rs:160` | - |  |
| `impl PartialEq for FloodItem::eq` | 2 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:40` | - |  |
| `impl PartialOrd for FloodItem::partial_cmp` | 2 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:48` | - |  |
| `impl Ord for FloodItem::cmp` | 2 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:54` | - |  |
| `impl MinHeap::new` | 0 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:69` | - |  |
| `impl MinHeap::push` | 2 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:73` | - |  |
| `impl MinHeap::pop` | 1 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:87` | - |  |
| `priority_flood_fill` | 3 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:119` | - | Condition a heightfield by filling all internal depression sinks to their spillway elevation. |
| `d8_flow_directions` | 3 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:189` | - | Compute D8 steepest descent flow directions for each cell. |
| `flow_accumulation` | 4 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:220` | - | Compute upstream flow accumulation area for every cell from D8 directions. |
| `carve_channels` | 6 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:252` | - | Carve fluvial river channels into a conditioned heightfield along high-accumulation paths. |
| `drainage_network` | 4 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:279` | - | Execute full drainage analysis pipeline: priority-flood conditioning, D8 routing, accumulation, and channel carving. |
| `priority_flood_fills_sink_hole` | 0 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:308` | - |  |
| `flow_accumulation_accumulates_to_sink` | 0 | `crates/terrain-erosion-solvers/src/drainage_pyramid.rs:319` | - |  |
| `impl Default for TerraceConfig::default` | 0 | `crates/terrain-erosion-solvers/src/fractal_terrace.rs:25` | - |  |
| `build_perm_table` | 1 | `crates/terrain-erosion-solvers/src/fractal_terrace.rs:38` | - | PRNG table generator for strata hash lookup. |
| `evaluate_stratum` | 3 | `crates/terrain-erosion-solvers/src/fractal_terrace.rs:51` | - | Compute single-elevation terraced profile and strata color index. |
| `apply_terraces` | 4 | `crates/terrain-erosion-solvers/src/fractal_terrace.rs:87` | - | Apply stratigraphic terracing to an entire 2D heightfield in-place. |
| `terraces_quantize_continuous_slope` | 0 | `crates/terrain-erosion-solvers/src/fractal_terrace.rs:107` | - |  |
| `lcg` | 1 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:9` | - | LCG step: deterministically advances generator state. |
| `impl Default for HydraulicConfig::default` | 0 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:68` | - |  |
| `impl Cell::height` | 1 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:106` | - |  |
| `sample_cell` | 5 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:112` | - |  |
| `hydraulic_erosion` | 4 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:140` | physics | Simulate hydraulic erosion over a mutable 2D heightfield. |
| `hydraulic_erosion_modifies_slope_deterministically` | 0 | `crates/terrain-erosion-solvers/src/hydraulic_droplet.rs:316` | - |  |
| `impl Default for LakeConfig::default` | 0 | `crates/terrain-erosion-solvers/src/lacustrine_fill.rs:21` | - |  |
| `fill_lakes` | 4 | `crates/terrain-erosion-solvers/src/lacustrine_fill.rs:45` | economics | Fill lakes, depressions, and seas up to hydrostatic equilibrium across a 2D heightfield. |
| `fill_lakes_creates_flat_surface_in_bowl` | 0 | `crates/terrain-erosion-solvers/src/lacustrine_fill.rs:153` | - |  |
| `full_geomorphology_pipeline_integration` | 0 | `crates/terrain-erosion-solvers/src/lib.rs:65` | - |  |
| `impl Default for SnowConfig::default` | 0 | `crates/terrain-erosion-solvers/src/snow_accumulation.rs:32` | - |  |
| `simulate_snow` | 4 | `crates/terrain-erosion-solvers/src/snow_accumulation.rs:58` | physics | Simulate snow precipitation, elevation-based freezing, melting, and avalanche transport. |
| `snow_accumulates_above_snowline_and_slips` | 0 | `crates/terrain-erosion-solvers/src/snow_accumulation.rs:161` | - |  |
| `kernel_sum` | 0 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:15` | - |  |
| `clamp_index` | 2 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:29` | - |  |
| `impl FloatGrid::new` | 2 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:41` | - |  |
| `impl FloatGrid::at` | 3 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:49` | - |  |
| `downsample` | 4 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:54` | - |  |
| `upsample` | 3 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:89` | - |  |
| `impl Shaper::change` | 4 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:148` | - |  |
| `impl Shaper::shape` | 2 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:196` | - |  |
| `impl Shaper::pyramid` | 2 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:225` | - |  |
| `impl Default for ThermalShaperConfig::default` | 0 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:285` | - |  |
| `thermal_shape` | 3 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:297` | - | Apply multi-scale thermal shaping to relax steep slopes across a resolution pyramid. |
| `thermal_shape_with` | 3 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:305` | - | Apply multi-scale thermal shaping with custom world scale configuration. |
| `thermal_shaper_preserves_bounds_and_flattens_spikes` | 0 | `crates/terrain-erosion-solvers/src/thermal_shaper.rs:363` | - |  |
