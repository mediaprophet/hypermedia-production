//! Network analysis, Dijkstra shortest route, isochrones, Traveling Salesperson (TSP) heuristic,
//! and vector topology validation rules.

use crate::geodesy::Coord;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// A node in a spatial network.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkNode {
    pub id: usize,
    pub coord: Coord,
}

/// A weighted edge in a spatial network.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkEdge {
    pub from: usize,
    pub to: usize,
    pub length_m: f64,
    pub cost_weight: f64,
}

impl NetworkEdge {
    pub fn effective_cost(&self) -> f64 {
        self.length_m * self.cost_weight.max(1e-6)
    }
}

/// A spatial routing graph.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SpatialGraph {
    pub nodes: Vec<NetworkNode>,
    pub edges: Vec<NetworkEdge>,
    adj: Vec<Vec<(usize, usize)>>, // node_idx -> list of (neighbor_node_idx, edge_idx)
}

impl SpatialGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, coord: Coord) -> usize {
        let id = self.nodes.len();
        self.nodes.push(NetworkNode { id, coord });
        self.adj.push(Vec::new());
        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize, length_m: f64, cost_weight: f64, bidirectional: bool) {
        let edge_idx = self.edges.len();
        self.edges.push(NetworkEdge { from, to, length_m, cost_weight });
        if from < self.adj.len() {
            self.adj[from].push((to, edge_idx));
        }
        if bidirectional {
            let back_idx = self.edges.len();
            self.edges.push(NetworkEdge { from: to, to: from, length_m, cost_weight });
            if to < self.adj.len() {
                self.adj[to].push((from, back_idx));
            }
        }
    }

    pub fn snap_to_nearest_node(&self, target: Coord, max_snap_m: f64) -> Option<usize> {
        let mut best_node = None;
        let mut min_dist = max_snap_m;

        for (idx, node) in self.nodes.iter().enumerate() {
            let d = (node.coord.x - target.x).hypot(node.coord.y - target.y);
            if d < min_dist {
                min_dist = d;
                best_node = Some(idx);
            }
        }
        best_node
    }
}

#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: f64,
    position: usize,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Result of Dijkstra shortest route search.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RouteResult {
    pub path_node_ids: Vec<usize>,
    pub path_coords: Vec<Coord>,
    pub total_length_m: f64,
    pub total_cost: f64,
}

/// Computes the shortest route between start_node and end_node using Dijkstra's algorithm.
pub fn dijkstra_shortest_path(graph: &SpatialGraph, start: usize, end: usize) -> Option<RouteResult> {
    if start >= graph.nodes.len() || end >= graph.nodes.len() {
        return None;
    }

    let mut dist = vec![f64::INFINITY; graph.nodes.len()];
    let mut prev = vec![None; graph.nodes.len()];
    let mut prev_edge = vec![None; graph.nodes.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0.0;
    heap.push(State { cost: 0.0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            break;
        }
        if cost > dist[position] {
            continue;
        }

        for &(next, edge_idx) in &graph.adj[position] {
            let next_cost = cost + graph.edges[edge_idx].effective_cost();
            if next_cost < dist[next] {
                dist[next] = next_cost;
                prev[next] = Some(position);
                prev_edge[next] = Some(edge_idx);
                heap.push(State { cost: next_cost, position: next });
            }
        }
    }

    if dist[end].is_infinite() {
        return None;
    }

    // Reconstruct path
    let mut curr = end;
    let mut path = vec![curr];
    let mut total_length = 0.0;

    while let Some(p) = prev[curr] {
        if let Some(e) = prev_edge[curr] {
            total_length += graph.edges[e].length_m;
        }
        curr = p;
        path.push(curr);
    }
    path.reverse();

    let path_coords = path.iter().map(|&idx| graph.nodes[idx].coord).collect();

    Some(RouteResult {
        path_node_ids: path,
        path_coords,
        total_length_m: total_length,
        total_cost: dist[end],
    })
}

/// Computes isochrone reachable nodes from start within `max_cost` budget.
pub fn calculate_isochrone(graph: &SpatialGraph, start: usize, max_cost: f64) -> Vec<(usize, f64)> {
    if start >= graph.nodes.len() {
        return Vec::new();
    }

    let mut dist = vec![f64::INFINITY; graph.nodes.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0.0;
    heap.push(State { cost: 0.0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > max_cost || cost > dist[position] {
            continue;
        }

        for &(next, edge_idx) in &graph.adj[position] {
            let next_cost = cost + graph.edges[edge_idx].effective_cost();
            if next_cost <= max_cost && next_cost < dist[next] {
                dist[next] = next_cost;
                heap.push(State { cost: next_cost, position: next });
            }
        }
    }

    let mut reachable = Vec::new();
    for (idx, &d) in dist.iter().enumerate() {
        if d <= max_cost {
            reachable.push((idx, d));
        }
    }
    reachable
}

/// Traveling Salesperson Problem (TSP) solver using Nearest-Neighbor tour + 2-Opt heuristic optimization.
pub fn tsp_2opt_solver(points: &[Coord]) -> Vec<usize> {
    let n = points.len();
    if n <= 1 {
        return (0..n).collect();
    }
    if n == 2 {
        return vec![0, 1];
    }

    // Step 1: Nearest-Neighbor greedy initial tour
    let mut visited = vec![false; n];
    let mut tour = Vec::with_capacity(n);
    let mut current = 0;
    tour.push(current);
    visited[current] = true;

    for _ in 1..n {
        let mut best_dist = f64::INFINITY;
        let mut best_next = current;
        for candidate in 0..n {
            if !visited[candidate] {
                let d = (points[current].x - points[candidate].x).hypot(points[current].y - points[candidate].y);
                if d < best_dist {
                    best_dist = d;
                    best_next = candidate;
                }
            }
        }
        visited[best_next] = true;
        tour.push(best_next);
        current = best_next;
    }

    // Step 2: 2-Opt edge swap improvement
    let dist = |t: &[usize], i: usize, j: usize| -> f64 {
        (points[t[i]].x - points[t[j]].x).hypot(points[t[i]].y - points[t[j]].y)
    };

    let mut improved = true;
    let mut iterations = 0;
    while improved && iterations < 50 {
        improved = false;
        iterations += 1;
        for i in 0..(n - 1) {
            for k in (i + 1)..n {
                let d_curr = dist(&tour, i, (i + 1) % n) + dist(&tour, k, (k + 1) % n);
                let d_swap = dist(&tour, i, k) + dist(&tour, (i + 1) % n, (k + 1) % n);
                if d_swap < d_curr - 1e-6 {
                    // Reverse subsegment tour[i+1..=k]
                    tour[(i + 1)..=k].reverse();
                    improved = true;
                }
            }
        }
    }

    tour
}

/// A vector topology rule violation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TopologyViolation {
    pub rule: &'static str,
    pub location: Coord,
    pub description: String,
}

/// Checks for consecutive duplicate vertices within a distance tolerance.
pub fn check_duplicate_vertices(polyline: &[Coord], tolerance_m: f64) -> Vec<TopologyViolation> {
    let mut violations = Vec::new();
    for i in 0..polyline.len().saturating_sub(1) {
        let p1 = polyline[i];
        let p2 = polyline[i + 1];
        let d = (p1.x - p2.x).hypot(p1.y - p2.y);
        if d <= tolerance_m {
            violations.push(TopologyViolation {
                rule: "NoDuplicateVertices",
                location: p1,
                description: format!("Duplicate vertex at index {i} and {} (distance {d:.4} <= {tolerance_m})", i + 1),
            });
        }
    }
    violations
}

/// Checks for self-intersections within a polyline.
pub fn check_self_intersections(polyline: &[Coord]) -> Vec<TopologyViolation> {
    let mut violations = Vec::new();
    let n = polyline.len();
    if n < 4 {
        return violations;
    }

    for i in 0..(n - 1) {
        let a1 = polyline[i];
        let a2 = polyline[i + 1];

        for j in (i + 2)..(n - 1) {
            // Skip adjacent segments
            if i == 0 && j == n - 2 && (polyline[0].x - polyline[n - 1].x).hypot(polyline[0].y - polyline[n - 1].y) < 1e-9 {
                continue;
            }
            let b1 = polyline[j];
            let b2 = polyline[j + 1];

            if let Some(pt) = segment_intersection(a1, a2, b1, b2) {
                violations.push(TopologyViolation {
                    rule: "NoSelfIntersection",
                    location: pt,
                    description: format!("Self-intersection between segment {i}-{} and {j}-{}", i + 1, j + 1),
                });
            }
        }
    }
    violations
}

/// Checks for dangles (endpoints of lines that do not connect to any other line endpoint within tolerance).
pub fn check_dangles(lines: &[Vec<Coord>], tolerance_m: f64) -> Vec<TopologyViolation> {
    let mut violations = Vec::new();
    let mut endpoints = Vec::new();

    for (line_idx, line) in lines.iter().enumerate() {
        if let (Some(&start), Some(&end)) = (line.first(), line.last()) {
            endpoints.push((start, line_idx));
            endpoints.push((end, line_idx));
        }
    }

    for (idx, &(pt, line_idx)) in endpoints.iter().enumerate() {
        let mut connected = false;
        for (other_idx, &(other_pt, other_line_idx)) in endpoints.iter().enumerate() {
            if idx != other_idx && line_idx != other_line_idx {
                let d = (pt.x - other_pt.x).hypot(pt.y - other_pt.y);
                if d <= tolerance_m {
                    connected = true;
                    break;
                }
            }
        }
        if !connected {
            violations.push(TopologyViolation {
                rule: "NoDangles",
                location: pt,
                description: format!("Dangling endpoint on line {line_idx}"),
            });
        }
    }

    violations
}

/// Returns the intersection point of two 2D segments if they intersect strictly.
fn segment_intersection(p1: Coord, p2: Coord, p3: Coord, p4: Coord) -> Option<Coord> {
    let d = (p2.x - p1.x) * (p4.y - p3.y) - (p2.y - p1.y) * (p4.x - p3.x);
    if d.abs() < 1e-12 {
        return None; // Parallel or collinear
    }

    let t = ((p3.x - p1.x) * (p4.y - p3.y) - (p3.y - p1.y) * (p4.x - p3.x)) / d;
    let u = ((p3.x - p1.x) * (p2.y - p1.y) - (p3.y - p1.y) * (p2.x - p1.x)) / d;

    if (1e-6..=(1.0 - 1e-6)).contains(&t) && (1e-6..=(1.0 - 1e-6)).contains(&u) {
        Some(Coord::new(
            p1.x + t * (p2.x - p1.x),
            p1.y + t * (p2.y - p1.y),
        ))
    } else {
        None
    }
}
