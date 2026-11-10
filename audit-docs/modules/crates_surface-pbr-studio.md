# `crates/surface-pbr-studio`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_surface-pbr-studio: UI-agnostic categorical PBR surface and radiance studio.

Provides zero-allocation BVH ray acceleration, multi-channel PBR layer compositing,
3D UV texture painting, ray-cast mesh map baking, OpenPBR/MaterialX 1.39 export,
and clean-room brush dynamics.

Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
and Informatics Fiduciary duties._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Default for BakeSettings::default` | 0 | `crates/surface-pbr-studio/src/bake.rs:33` | - |  |
| `impl SurfaceAttributes::build` | 3 | `crates/surface-pbr-studio/src/bake.rs:55` | computational geometry |  |
| `impl SurfaceAttributes::rasterize_triangle` | 11 | `crates/surface-pbr-studio/src/bake.rs:97` | 3d, 2d |  |
| `bake_map` | 4 | `crates/surface-pbr-studio/src/bake.rs:147` | computational geometry | Bake a mesh map using BVH raycasting. |
| `hemisphere_sample` | 4 | `crates/surface-pbr-studio/src/bake.rs:243` | 3d |  |
| `hsv_to_rgb` | 3 | `crates/surface-pbr-studio/src/bake.rs:264` | - |  |
| `impl Bvh::build` | 1 | `crates/surface-pbr-studio/src/bvh.rs:29` | computational geometry | Build a BVH for a mesh. An empty mesh yields an empty tree. |
| `impl Bvh::bounds` | 1 | `crates/surface-pbr-studio/src/bvh.rs:50` | - |  |
| `impl Bvh::node_count` | 1 | `crates/surface-pbr-studio/src/bvh.rs:54` | graph |  |
| `impl Bvh::is_empty` | 1 | `crates/surface-pbr-studio/src/bvh.rs:58` | - |  |
| `impl Bvh::intersect` | 3 | `crates/surface-pbr-studio/src/bvh.rs:63` | computational geometry, 3d | Finds the closest ray-mesh intersection. |
| `safe_inverse` | 1 | `crates/surface-pbr-studio/src/bvh.rs:120` | - |  |
| `aabb_hit` | 5 | `crates/surface-pbr-studio/src/bvh.rs:133` | computational geometry, 3d |  |
| `intersect_triangle` | 3 | `crates/surface-pbr-studio/src/bvh.rs:155` | computational geometry, 3d |  |
| `triangle_bounds` | 2 | `crates/surface-pbr-studio/src/bvh.rs:195` | computational geometry |  |
| `build_range` | 6 | `crates/surface-pbr-studio/src/bvh.rs:206` | computational geometry, graph |  |
| `impl BlendMode::blend` | 3 | `crates/surface-pbr-studio/src/compose.rs:21` | - |  |
| `impl PbrLayer::new_fill` | 4 | `crates/surface-pbr-studio/src/compose.rs:58` | - |  |
| `impl PbrLayerStack::new` | 0 | `crates/surface-pbr-studio/src/compose.rs:84` | - |  |
| `impl PbrLayerStack::add_layer` | 2 | `crates/surface-pbr-studio/src/compose.rs:88` | - |  |
| `impl PbrLayerStack::composite` | 3 | `crates/surface-pbr-studio/src/compose.rs:93` | - | Evaluates the layer stack into rendered images for all present channels. |
| `test_bvh_quad_intersection` | 0 | `crates/surface-pbr-studio/src/lib.rs:40` | computational geometry |  |
| `test_bake_position_and_normal` | 0 | `crates/surface-pbr-studio/src/lib.rs:54` | 3d |  |
| `test_paint_and_polyfill` | 0 | `crates/surface-pbr-studio/src/lib.rs:77` | - |  |
| `test_layer_stack_composition` | 0 | `crates/surface-pbr-studio/src/lib.rs:100` | - |  |
| `test_materialx_export` | 0 | `crates/surface-pbr-studio/src/lib.rs:123` | - |  |
| `test_probrush_dynamics_and_stamp` | 0 | `crates/surface-pbr-studio/src/lib.rs:140` | - |  |
| `impl PbrChannel::id` | 1 | `crates/surface-pbr-studio/src/material.rs:33` | - |  |
| `impl PbrChannel::label` | 1 | `crates/surface-pbr-studio/src/material.rs:47` | - |  |
| `impl PbrChannel::default_value` | 1 | `crates/surface-pbr-studio/src/material.rs:61` | - |  |
| `impl MaterialPreset::metal` | 3 | `crates/surface-pbr-studio/src/material.rs:90` | - |  |
| `impl MaterialPreset::dielectric` | 4 | `crates/surface-pbr-studio/src/material.rs:103` | - |  |
| `impl MaterialPreset::glass` | 3 | `crates/surface-pbr-studio/src/material.rs:116` | - |  |
| `impl MaterialPreset::car_paint` | 3 | `crates/surface-pbr-studio/src/material.rs:129` | - |  |
| `impl MaterialPreset::skin` | 2 | `crates/surface-pbr-studio/src/material.rs:142` | - |  |
| `impl MaterialPreset::fabric` | 3 | `crates/surface-pbr-studio/src/material.rs:155` | - |  |
| `standard_presets` | 0 | `crates/surface-pbr-studio/src/material.rs:170` | - | Catalog of standard calibrated PBR materials. |
| `get_preset_by_name` | 1 | `crates/surface-pbr-studio/src/material.rs:190` | - |  |
| `impl Default for MaterialXOptions::default` | 0 | `crates/surface-pbr-studio/src/materialx.rs:31` | - |  |
| `safe_identifier` | 1 | `crates/surface-pbr-studio/src/materialx.rs:42` | - |  |
| `escape_xml` | 1 | `crates/surface-pbr-studio/src/materialx.rs:60` | - |  |
| `generate_materialx_document` | 3 | `crates/surface-pbr-studio/src/materialx.rs:68` | 3d | Generates a complete MaterialX 1.39 XML document connecting maps to a surface shader. |
| `impl Vec2::new` | 2 | `crates/surface-pbr-studio/src/math.rs:19` | - |  |
| `impl Vec2::dot` | 2 | `crates/surface-pbr-studio/src/math.rs:24` | - |  |
| `impl Vec2::length_squared` | 1 | `crates/surface-pbr-studio/src/math.rs:29` | - |  |
| `impl Vec2::length` | 1 | `crates/surface-pbr-studio/src/math.rs:34` | - |  |
| `impl Vec2::distance` | 2 | `crates/surface-pbr-studio/src/math.rs:39` | - |  |
| `impl Vec2::lerp` | 3 | `crates/surface-pbr-studio/src/math.rs:44` | - |  |
| `impl Add for Vec2::add` | 2 | `crates/surface-pbr-studio/src/math.rs:55` | - |  |
| `impl Sub for Vec2::sub` | 2 | `crates/surface-pbr-studio/src/math.rs:63` | - |  |
| `impl Mul<f32> for Vec2::mul` | 2 | `crates/surface-pbr-studio/src/math.rs:71` | - |  |
| `impl Div<f32> for Vec2::div` | 2 | `crates/surface-pbr-studio/src/math.rs:79` | - |  |
| `impl Vec3::new` | 3 | `crates/surface-pbr-studio/src/math.rs:100` | - |  |
| `impl Vec3::dot` | 2 | `crates/surface-pbr-studio/src/math.rs:105` | - |  |
| `impl Vec3::cross` | 2 | `crates/surface-pbr-studio/src/math.rs:110` | - |  |
| `impl Vec3::length_squared` | 1 | `crates/surface-pbr-studio/src/math.rs:119` | - |  |
| `impl Vec3::length` | 1 | `crates/surface-pbr-studio/src/math.rs:124` | - |  |
| `impl Vec3::normalize` | 1 | `crates/surface-pbr-studio/src/math.rs:129` | - |  |
| `impl Vec3::distance` | 2 | `crates/surface-pbr-studio/src/math.rs:139` | - |  |
| `impl Vec3::min` | 2 | `crates/surface-pbr-studio/src/math.rs:144` | - |  |
| `impl Vec3::max` | 2 | `crates/surface-pbr-studio/src/math.rs:153` | - |  |
| `impl Vec3::lerp` | 3 | `crates/surface-pbr-studio/src/math.rs:162` | - |  |
| `impl Vec3::to_array` | 1 | `crates/surface-pbr-studio/src/math.rs:171` | - |  |
| `impl Vec3::from_array` | 1 | `crates/surface-pbr-studio/src/math.rs:176` | - |  |
| `impl Add for Vec3::add` | 2 | `crates/surface-pbr-studio/src/math.rs:184` | - |  |
| `impl AddAssign for Vec3::add_assign` | 2 | `crates/surface-pbr-studio/src/math.rs:191` | - |  |
| `impl Sub for Vec3::sub` | 2 | `crates/surface-pbr-studio/src/math.rs:201` | - |  |
| `impl Neg for Vec3::neg` | 1 | `crates/surface-pbr-studio/src/math.rs:209` | - |  |
| `impl Mul<f32> for Vec3::mul` | 2 | `crates/surface-pbr-studio/src/math.rs:217` | - |  |
| `impl Mul<Vec3> for Vec3::mul` | 2 | `crates/surface-pbr-studio/src/math.rs:225` | - |  |
| `impl Div<f32> for Vec3::div` | 2 | `crates/surface-pbr-studio/src/math.rs:233` | - |  |
| `impl Vec4::new` | 4 | `crates/surface-pbr-studio/src/math.rs:251` | - |  |
| `impl Vec4::to_array` | 1 | `crates/surface-pbr-studio/src/math.rs:256` | - |  |
| `impl Vec4::from_array` | 1 | `crates/surface-pbr-studio/src/math.rs:261` | - |  |
| `impl Ray::new` | 2 | `crates/surface-pbr-studio/src/math.rs:275` | 3d |  |
| `impl Ray::point_at` | 2 | `crates/surface-pbr-studio/src/math.rs:283` | 3d |  |
| `impl Bounds::new` | 2 | `crates/surface-pbr-studio/src/math.rs:311` | 3d |  |
| `impl Bounds::point` | 1 | `crates/surface-pbr-studio/src/math.rs:315` | 3d |  |
| `impl Bounds::extend` | 2 | `crates/surface-pbr-studio/src/math.rs:319` | 3d |  |
| `impl Bounds::union` | 2 | `crates/surface-pbr-studio/src/math.rs:324` | - |  |
| `impl Bounds::center` | 1 | `crates/surface-pbr-studio/src/math.rs:331` | 3d |  |
| `impl Bounds::extent` | 1 | `crates/surface-pbr-studio/src/math.rs:335` | 3d |  |
| `impl Bounds::radius` | 1 | `crates/surface-pbr-studio/src/math.rs:339` | - |  |
| `impl Bounds::is_empty` | 1 | `crates/surface-pbr-studio/src/math.rs:343` | - |  |
| `ray_triangle` | 4 | `crates/surface-pbr-studio/src/math.rs:349` | computational geometry, 3d | Möller–Trumbore ray-triangle intersection (hits both front and back faces). |
| `impl Mesh::new` | 0 | `crates/surface-pbr-studio/src/mesh.rs:27` | - |  |
| `impl Mesh::vertex_count` | 1 | `crates/surface-pbr-studio/src/mesh.rs:31` | 3d |  |
| `impl Mesh::triangle_count` | 1 | `crates/surface-pbr-studio/src/mesh.rs:35` | - |  |
| `impl Mesh::bounds` | 1 | `crates/surface-pbr-studio/src/mesh.rs:39` | - |  |
| `impl Mesh::compute_normals` | 1 | `crates/surface-pbr-studio/src/mesh.rs:48` | 3d | Compute face/area-weighted vertex normals if not provided. |
| `impl Mesh::create_quad` | 1 | `crates/surface-pbr-studio/src/mesh.rs:67` | - | Creates a unit quad on the XY plane with UVs in [0, 1]. |
| `impl Mesh::create_cube` | 1 | `crates/surface-pbr-studio/src/mesh.rs:101` | - | Creates a unit cube centered at origin. |
| `impl Default for BrushSettings::default` | 0 | `crates/surface-pbr-studio/src/paint.rs:27` | - |  |
| `apply_dab` | 6 | `crates/surface-pbr-studio/src/paint.rs:46` | 2d | Applies a single dab centered at `(uv.x, uv.y)` with the current brush. |
| `apply_eraser_dab` | 5 | `crates/surface-pbr-studio/src/paint.rs:98` | 2d | Erases pixels within a dab area. |
| `paint_stroke` | 4 | `crates/surface-pbr-studio/src/paint.rs:142` | - | Paints an interpolated stroke along a path. |
| `polyfill` | 3 | `crates/surface-pbr-studio/src/paint.rs:183` | computational geometry, 2d | Fills a connected UV island or polygon bounding region with constant color. |
| `impl Default for BrushDynamics::default` | 0 | `crates/surface-pbr-studio/src/probrush.rs:23` | - |  |
| `impl BrushTipStamp::procedural_round` | 2 | `crates/surface-pbr-studio/src/probrush.rs:49` | - | Generates a procedural circular tip mask with smooth falloff. |
| `parse_brush_properties` | 2 | `crates/surface-pbr-studio/src/probrush.rs:93` | - | Parses clean-room brush settings from key-value pairs. |
| `impl RasterImage::new` | 2 | `crates/surface-pbr-studio/src/raster.rs:14` | - |  |
| `impl RasterImage::filled` | 3 | `crates/surface-pbr-studio/src/raster.rs:23` | - |  |
| `impl RasterImage::is_empty` | 1 | `crates/surface-pbr-studio/src/raster.rs:32` | - |  |
| `impl RasterImage::pixel` | 3 | `crates/surface-pbr-studio/src/raster.rs:37` | 2d |  |
| `impl RasterImage::set_pixel` | 4 | `crates/surface-pbr-studio/src/raster.rs:46` | 2d |  |
| `impl RasterImage::sample_nearest` | 3 | `crates/surface-pbr-studio/src/raster.rs:56` | - |  |
| `impl RasterImage::sample_bilinear` | 3 | `crates/surface-pbr-studio/src/raster.rs:67` | - |  |
| `impl RasterImage::box_blur` | 2 | `crates/surface-pbr-studio/src/raster.rs:97` | - | Fast separable box blur of all channels. |
