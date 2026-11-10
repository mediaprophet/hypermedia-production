# `crates/geodesy-spatial-analysis`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Categorical geodesy, planetary ellipsoids, Karney WGS84 geodesic solvers, Ordinary Kriging,
DEM morphometry, network topology, and cartographic layout.

Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
and Informatics Fiduciary duties ensuring scientific precision and geographic accuracy._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl RgbaColor::new` | 4 | `crates/geodesy-spatial-analysis/src/cartography.rs:32` | - |  |
| `impl Default for CartoSymbol::default` | 0 | `crates/geodesy-spatial-analysis/src/cartography.rs:54` | - |  |
| `compute_class_breaks` | 3 | `crates/geodesy-spatial-analysis/src/cartography.rs:83` | - | Computes class break boundaries for graduated symbology. |
| `fisher_jenks_breaks` | 2 | `crates/geodesy-spatial-analysis/src/cartography.rs:127` | - | Fisher-Jenks natural breaks optimization algorithm for 1D sorted data. |
| `variance_range` | 3 | `crates/geodesy-spatial-analysis/src/cartography.rs:171` | - |  |
| `layout_curved_line_label` | 3 | `crates/geodesy-spatial-analysis/src/cartography.rs:193` | - | Places text characters along a polyline, rotating each character to follow the line's curvature. |
| `impl PageSize::dimensions_mm` | 1 | `crates/geodesy-spatial-analysis/src/cartography.rs:276` | - |  |
| `nice_scale_distance` | 1 | `crates/geodesy-spatial-analysis/src/cartography.rs:310` | - | Computes nice distance for scale bars given a nominal distance. |
| `generate_graticule` | 2 | `crates/geodesy-spatial-analysis/src/cartography.rs:321` | - | Generates geographic graticule lines (meridians and parallels) at specified degree step. |
| `impl Coord::new` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:31` | - |  |
| `impl Coord::is_finite` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:35` | - |  |
| `impl Default for Extent::default` | 0 | `crates/geodesy-spatial-analysis/src/geodesy.rs:50` | - |  |
| `impl Extent::new` | 4 | `crates/geodesy-spatial-analysis/src/geodesy.rs:70` | - |  |
| `impl Extent::from_points` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:74` | - |  |
| `impl Extent::contains` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:83` | - |  |
| `impl Extent::width` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:87` | - |  |
| `impl Extent::height` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:91` | - |  |
| `impl Body::find` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:128` | - |  |
| `impl Body::get` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:133` | - |  |
| `impl Body::is_earth` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:137` | - |  |
| `impl Body::b` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:141` | - |  |
| `impl Body::mean_radius_m` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:145` | - |  |
| `impl Body::ratio` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:149` | - |  |
| `impl Body::length` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:153` | - |  |
| `impl Body::area` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:157` | - |  |
| `haversine_m` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:163` | - | Great-circle Haversine distance in metres between two points in degrees (lon, lat). |
| `wgs84_inverse` | 4 | `crates/geodesy-spatial-analysis/src/geodesy.rs:176` | - | WGS84 geodesic inverse problem: computes distance in metres and initial/final azimuths in degrees. |
| `wgs84_direct` | 4 | `crates/geodesy-spatial-analysis/src/geodesy.rs:275` | - | WGS84 geodesic direct problem: computes destination (lat, lon) and arrival azimuth from start, azimuth, and distance. |
| `geodesic_ring_area_m2` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:347` | computational geometry | Area of a closed polygon ring on the spherical earth (square metres, always positive). |
| `impl Aeqd::new` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:373` | - |  |
| `impl Aeqd::forward` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:381` | 3d | Forward projection: lon/lat in degrees -> planar metres east and north of centre. |
| `impl Aeqd::inverse` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:388` | 3d | Inverse projection: planar metres -> lon/lat in degrees. |
| `impl Sinusoidal::new` | 5 | `crates/geodesy-spatial-analysis/src/geodesy.rs:411` | - |  |
| `impl Sinusoidal::wgs84` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:423` | - |  |
| `impl Sinusoidal::forward` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:435` | 3d | Forward projection: lon/lat degrees -> projected metres. |
| `impl Sinusoidal::inverse` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:455` | 3d | Inverse projection: projected metres -> lon/lat degrees. |
| `mlfn_coefficients` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:478` | - |  |
| `mlfn` | 4 | `crates/geodesy-spatial-analysis/src/geodesy.rs:488` | - |  |
| `inv_mlfn` | 3 | `crates/geodesy-spatial-analysis/src/geodesy.rs:494` | - |  |
| `tissot_distortion` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:520` | 3d | Computes Tissot distortion of a projection at a given coordinate using Snyder's central differences. |
| `impl Dggs::cell_area_m2` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:576` | - | Nominal area of a DGGS cell at resolution `res` (0..=30) in square metres. |
| `impl Dggs::cell_boundary` | 2 | `crates/geodesy-spatial-analysis/src/geodesy.rs:586` | - | Generates boundary vertices for an approximate DGGS hexagonal/pentagonal cell given its centre and resolution. |
| `geodesic_buffer_point` | 3 | `crates/geodesy-spatial-analysis/src/geodesy.rs:603` | computational geometry | Buffers a single point by `distance_m` on the geodetic manifold, returning a polygon ring. |
| `cut_antimeridian_line` | 1 | `crates/geodesy-spatial-analysis/src/geodesy.rs:616` | - | Splits a polyline at the antimeridian (where consecutive points jump across > 180 degrees longitude). |
| `impl VariogramParams::new` | 4 | `crates/geodesy-spatial-analysis/src/kriging.rs:30` | - |  |
| `impl VariogramParams::semivariance` | 2 | `crates/geodesy-spatial-analysis/src/kriging.rs:40` | - | Evaluates semivariance $\gamma(h)$ for lag distance $h$. |
| `impl SamplePoint::new` | 3 | `crates/geodesy-spatial-analysis/src/kriging.rs:74` | - |  |
| `empirical_variogram` | 3 | `crates/geodesy-spatial-analysis/src/kriging.rs:91` | - | Computes experimental semivariogram bins from sample points. |
| `fit_variogram` | 2 | `crates/geodesy-spatial-analysis/src/kriging.rs:141` | - | Automatically fits a variogram model to empirical bins using weighted least squares. |
| `predict_kriging` | 3 | `crates/geodesy-spatial-analysis/src/kriging.rs:186` | numerics | Solves Ordinary Kriging linear system for a target point. |
| `solve_linear_system` | 3 | `crates/geodesy-spatial-analysis/src/kriging.rs:252` | numerics | In-place linear solver for `Ax = b` using Gauss-Jordan elimination with partial pivoting. |
| `ordinary_kriging_grid` | 5 | `crates/geodesy-spatial-analysis/src/kriging.rs:309` | - | Interpolates a regular raster grid from scattered points using Ordinary Kriging. |
| `test_wgs84_distance_and_area` | 0 | `crates/geodesy-spatial-analysis/src/lib.rs:46` | - |  |
| `test_ordinary_kriging` | 0 | `crates/geodesy-spatial-analysis/src/lib.rs:71` | - |  |
| `test_dem_slope_and_contours` | 0 | `crates/geodesy-spatial-analysis/src/lib.rs:88` | - |  |
| `test_dijkstra_and_tsp` | 0 | `crates/geodesy-spatial-analysis/src/lib.rs:116` | graph |  |
| `test_topology_validation` | 0 | `crates/geodesy-spatial-analysis/src/lib.rs:144` | - |  |
| `impl DemGrid::new` | 7 | `crates/geodesy-spatial-analysis/src/morphometry.rs:23` | - |  |
| `impl DemGrid::get` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:36` | - |  |
| `impl DemGrid::get_or_edge` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:50` | graph |  |
| `surface_gradient` | 5 | `crates/geodesy-spatial-analysis/src/morphometry.rs:67` | - | Computes the surface gradient $(dz/dx, dz/dy)$ at a cell in metres per metre. |
| `compute_slope` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:103` | - | Computes slope in degrees (0..90). |
| `compute_aspect` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:117` | - | Computes aspect in degrees clockwise from North (0..360, 0 = North, 90 = East, etc.). Flat terrain is assigned -1.0. |
| `compute_hillshade` | 4 | `crates/geodesy-spatial-analysis/src/morphometry.rs:138` | - | Computes analytical hillshade illumination values (0..255). |
| `compute_curvature_at` | 4 | `crates/geodesy-spatial-analysis/src/morphometry.rs:177` | - | Computes profile and plan curvature at (col, row) using Zevenbergen-Thorne polynomial coefficients. |
| `marching_squares_contours` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:227` | - | Extracts contour polylines from a DEM grid at a specified interval using the Marching Squares algorithm. |
| `extract_segments_at_level` | 2 | `crates/geodesy-spatial-analysis/src/morphometry.rs:277` | - |  |
| `stitch_segments_into_lines` | 1 | `crates/geodesy-spatial-analysis/src/morphometry.rs:346` | - |  |
| `compute_zonal_stats` | 3 | `crates/geodesy-spatial-analysis/src/morphometry.rs:407` | - | Computes zonal statistics for cells matching `target_zone` in a zone integer raster. |
| `impl NetworkEdge::effective_cost` | 1 | `crates/geodesy-spatial-analysis/src/network_topology.rs:26` | - |  |
| `impl SpatialGraph::new` | 0 | `crates/geodesy-spatial-analysis/src/network_topology.rs:40` | - |  |
| `impl SpatialGraph::add_node` | 2 | `crates/geodesy-spatial-analysis/src/network_topology.rs:44` | graph |  |
| `impl SpatialGraph::add_edge` | 6 | `crates/geodesy-spatial-analysis/src/network_topology.rs:51` | graph |  |
| `impl SpatialGraph::snap_to_nearest_node` | 3 | `crates/geodesy-spatial-analysis/src/network_topology.rs:66` | graph |  |
| `impl Ord for State::cmp` | 2 | `crates/geodesy-spatial-analysis/src/network_topology.rs:90` | - |  |
| `impl PartialOrd for State::partial_cmp` | 2 | `crates/geodesy-spatial-analysis/src/network_topology.rs:96` | - |  |
| `dijkstra_shortest_path` | 3 | `crates/geodesy-spatial-analysis/src/network_topology.rs:111` | graph | Computes the shortest route between start_node and end_node using Dijkstra's algorithm. |
| `calculate_isochrone` | 3 | `crates/geodesy-spatial-analysis/src/network_topology.rs:172` | graph | Computes isochrone reachable nodes from start within `max_cost` budget. |
| `tsp_2opt_solver` | 1 | `crates/geodesy-spatial-analysis/src/network_topology.rs:207` | - | Traveling Salesperson Problem (TSP) solver using Nearest-Neighbor tour + 2-Opt heuristic optimization. |
| `check_duplicate_vertices` | 2 | `crates/geodesy-spatial-analysis/src/network_topology.rs:275` | - | Checks for consecutive duplicate vertices within a distance tolerance. |
| `check_self_intersections` | 1 | `crates/geodesy-spatial-analysis/src/network_topology.rs:293` | - | Checks for self-intersections within a polyline. |
| `check_dangles` | 2 | `crates/geodesy-spatial-analysis/src/network_topology.rs:325` | - | Checks for dangles (endpoints of lines that do not connect to any other line endpoint within tolerance). |
| `segment_intersection` | 4 | `crates/geodesy-spatial-analysis/src/network_topology.rs:360` | computational geometry | Returns the intersection point of two 2D segments if they intersect strictly. |
