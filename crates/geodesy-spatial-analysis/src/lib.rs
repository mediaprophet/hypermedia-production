//! Categorical geodesy, planetary ellipsoids, Karney WGS84 geodesic solvers, Ordinary Kriging,
//! DEM morphometry, network topology, and cartographic layout.
//!
//! Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
//! and Informatics Fiduciary duties ensuring scientific precision and geographic accuracy.

#![forbid(unsafe_code)]

pub mod cartography;
pub mod geodesy;
pub mod kriging;
pub mod morphometry;
pub mod network_topology;

pub use cartography::{
    CartoSymbol, ClassificationMethod, ClassificationRange, MapFrame, MarkerShape, PageSize,
    PlacedChar, RgbaColor, ScaleBarElement, compute_class_breaks, generate_graticule,
    layout_curved_line_label, nice_scale_distance,
};
pub use geodesy::{
    Aeqd, BODIES, Body, Coord, Dggs, Distortion, EARTH, EARTH_RADIUS_M, Extent, Sinusoidal,
    WGS84_A, WGS84_B, WGS84_E2, WGS84_F, WGS84_INV_F, cut_antimeridian_line,
    geodesic_buffer_point, geodesic_ring_area_m2, haversine_m, tissot_distortion, wgs84_direct,
    wgs84_inverse,
};
pub use kriging::{
    KrigingEstimate, KrigingSurface, SamplePoint, VariogramBin, VariogramModel, VariogramParams,
    empirical_variogram, fit_variogram, ordinary_kriging_grid, predict_kriging,
};
pub use morphometry::{
    ContourLine, CurvatureResult, DemGrid, GradientAlgorithm, ZonalStats, compute_aspect,
    compute_curvature_at, compute_hillshade, compute_slope, compute_zonal_stats,
    marching_squares_contours, surface_gradient,
};
pub use network_topology::{
    NetworkEdge, NetworkNode, RouteResult, SpatialGraph, TopologyViolation, calculate_isochrone,
    check_dangles, check_duplicate_vertices, check_self_intersections, dijkstra_shortest_path,
    tsp_2opt_solver,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wgs84_distance_and_area() {
        // Paris (48.8566 N, 2.3522 E) to London (51.5074 N, -0.1278 E)
        let (dist, az1, _) = wgs84_inverse(48.8566, 2.3522, 51.5074, -0.1278);
        assert!((dist - 343_556.0).abs() < 2_000.0, "WGS84 distance error: {dist}");
        assert!(az1 > 320.0 && az1 < 350.0, "Azimuth error: {az1}");

        // Direct problem round-trip
        let (lat2, lon2, _) = wgs84_direct(48.8566, 2.3522, az1, dist);
        assert!((lat2 - 51.5074).abs() < 1e-4);
        assert!((lon2 - (-0.1278)).abs() < 1e-4);

        // Spherical ring area of 1 deg square at equator
        let ring = vec![
            Coord::new(0.0, 0.0),
            Coord::new(1.0, 0.0),
            Coord::new(1.0, 1.0),
            Coord::new(0.0, 1.0),
            Coord::new(0.0, 0.0),
        ];
        let area = geodesic_ring_area_m2(&ring);
        // Approx 111 km * 111 km = ~1.23e10 m^2
        assert!((area - 1.236e10).abs() < 5e8, "Ring area mismatch: {area}");
    }

    #[test]
    fn test_ordinary_kriging() {
        let points = vec![
            SamplePoint::new(0.0, 0.0, 10.0),
            SamplePoint::new(10.0, 0.0, 20.0),
            SamplePoint::new(0.0, 10.0, 20.0),
            SamplePoint::new(10.0, 10.0, 30.0),
        ];

        let params = VariogramParams::new(VariogramModel::Spherical, 0.0, 100.0, 20.0);

        // Target at center (5.0, 5.0) should predict 20.0 by symmetry
        let est = predict_kriging(&points, Coord::new(5.0, 5.0), &params).expect("kriging failed");
        assert!((est.estimate - 20.0).abs() < 0.5, "Center estimate: {}", est.estimate);
        assert!(est.variance >= 0.0);
    }

    #[test]
    fn test_dem_slope_and_contours() {
        // Simple 4x4 tilted plane (elevations = col * 10)
        let mut elevations = Vec::new();
        for _ in 0..4 {
            for c in 0..4 {
                elevations.push((c as f64) * 10.0);
            }
        }

        let dem = DemGrid::new(
            4,
            4,
            10.0,
            10.0,
            Extent::new(0.0, 0.0, 40.0, 40.0),
            -9999.0,
            elevations,
        );

        let slopes = compute_slope(&dem, GradientAlgorithm::Horn, 1.0);
        // dz/dx = 1.0, dz/dy = 0.0 -> slope = 45 degrees
        assert!((slopes[5] - 45.0).abs() < 1.0, "Slope at cell (1, 1): {}", slopes[5]);

        let contours = marching_squares_contours(&dem, 10.0, 0.0);
        assert!(!contours.is_empty(), "Contour line generation produced no isolines");
    }

    #[test]
    fn test_dijkstra_and_tsp() {
        let mut graph = SpatialGraph::new();
        let n0 = graph.add_node(Coord::new(0.0, 0.0));
        let n1 = graph.add_node(Coord::new(1.0, 0.0));
        let n2 = graph.add_node(Coord::new(2.0, 0.0));
        let n3 = graph.add_node(Coord::new(1.0, 1.0));

        graph.add_edge(n0, n1, 10.0, 1.0, true);
        graph.add_edge(n1, n2, 10.0, 1.0, true);
        graph.add_edge(n0, n3, 25.0, 1.0, true);
        graph.add_edge(n3, n2, 25.0, 1.0, true);

        let route = dijkstra_shortest_path(&graph, n0, n2).expect("Dijkstra failed");
        assert_eq!(route.path_node_ids, vec![n0, n1, n2]);
        assert_eq!(route.total_cost, 20.0);

        // TSP 2-opt
        let pts = vec![
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 1.0),
            Coord::new(1.0, 1.0),
            Coord::new(1.0, 0.0),
        ];
        let tour = tsp_2opt_solver(&pts);
        assert_eq!(tour.len(), 4);
    }

    #[test]
    fn test_topology_validation() {
        // Polyline with self-intersection (figure-8)
        let figure8 = vec![
            Coord::new(0.0, 0.0),
            Coord::new(2.0, 2.0),
            Coord::new(0.0, 2.0),
            Coord::new(2.0, 0.0),
        ];
        let self_inter = check_self_intersections(&figure8);
        assert_eq!(self_inter.len(), 1, "Self intersection should be detected");

        // Duplicate vertices
        let with_dups = vec![
            Coord::new(0.0, 0.0),
            Coord::new(0.0, 0.0),
            Coord::new(1.0, 1.0),
        ];
        let dups = check_duplicate_vertices(&with_dups, 1e-4);
        assert_eq!(dups.len(), 1, "Duplicate vertex should be detected");
    }
}
