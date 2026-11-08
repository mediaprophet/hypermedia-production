//! Cyclic double-reflection Bishop rotation-minimizing frames (RMF)
//! and mesh geodesic shortest edge path search.

use crate::*;
use drafting_geometry::Vec2;
use qualia_core_db::specialized_libs::computational_geometry::{
    math_geometry::AffineFrame3,
    sweep_extrusion::compute_bishop_frames_cyclic,
    Point3,
};
use std::collections::{BTreeMap, BinaryHeap};
use vibe::Value;

#[derive(Copy, Clone, PartialEq)]
struct DijkstraItem {
    cost: f64,
    node: usize,
}

impl Eq for DijkstraItem {}

impl Ord for DijkstraItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Min-heap ordering; tie-break on node index for 100% deterministic search order.
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for DijkstraItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl AssetHost<'_> {
    /// Cyclic double-reflection Bishop rotation-minimizing frame (RMF) generator along a 3D path.
    pub(super) fn locus_bishop_rmf_cyclic(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["path", "closed", "initial_normal"])?;
        let path_raw = list(required(a, "path")?)?;
        if path_raw.len() < 3 || path_raw.len() > 2048 {
            return Err("Path must have between 3 and 2048 points".into());
        }
        let closed = boolean(a, "closed", true)?;
        let initial_normal = if let Some(nv) = a.get("initial_normal") {
            let [nx, ny, nz] = vector::<3>(nv)?;
            if !nx.is_finite() || !ny.is_finite() || !nz.is_finite() {
                return Err("initial_normal components must be finite numbers".into());
            }
            Some(Point3::new(nx, ny, nz))
        } else {
            None
        };

        let mut pts3 = Vec::with_capacity(path_raw.len());
        for v in path_raw {
            let [x, y, z] = vector::<3>(v)?;
            if !x.is_finite() || !y.is_finite() || !z.is_finite() {
                return Err("Path point coordinates must be finite numbers".into());
            }
            pts3.push(Point3::new(x, y, z));
        }

        let n = pts3.len();
        self.charge(n * 32 + 1024, n * 128 + 4096)?;

        let mut frames = vec![
            AffineFrame3 {
                origin: Point3::default(),
                e0: Point3::default(),
                e1: Point3::default(),
                e2: Point3::default(),
            };
            n
        ];

        let (frame_count, defect_rad) =
            compute_bishop_frames_cyclic(&pts3, closed, initial_normal, &mut frames)
                .map_err(|e| format!("Bishop RMF cyclic computation failed: {e:?}"))?;

        frames.truncate(frame_count);

        let frame_records: Vec<Value> = frames
            .into_iter()
            .map(|f| {
                Value::Record(BTreeMap::from([
                    (
                        "origin".into(),
                        Value::List(vec![
                            Value::F64(f.origin.x),
                            Value::F64(f.origin.y),
                            Value::F64(f.origin.z),
                        ]),
                    ),
                    (
                        "normal".into(),
                        Value::List(vec![
                            Value::F64(f.e0.x),
                            Value::F64(f.e0.y),
                            Value::F64(f.e0.z),
                        ]),
                    ),
                    (
                        "binormal".into(),
                        Value::List(vec![
                            Value::F64(f.e1.x),
                            Value::F64(f.e1.y),
                            Value::F64(f.e1.z),
                        ]),
                    ),
                    (
                        "tangent".into(),
                        Value::List(vec![
                            Value::F64(f.e2.x),
                            Value::F64(f.e2.y),
                            Value::F64(f.e2.z),
                        ]),
                    ),
                ]))
            })
            .collect();

        Ok(Value::Record(BTreeMap::from([
            ("frame_count".into(), Value::U64(frame_count as u64)),
            ("closed".into(), Value::Bool(closed)),
            ("holonomy_defect_rad".into(), Value::F64(defect_rad)),
            ("frames".into(), Value::List(frame_records)),
        ])))
    }

    /// Shortest edge path across a 3D triangle manifold using deterministic Dijkstra search.
    pub(super) fn locus_shortest_edge_path(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "start_vertex", "end_vertex"])?;
        let mesh = self.mesh(required(a, "mesh")?)?.clone();
        let nv = mesh.positions.len();
        let nt = mesh.triangles.len();
        if nv == 0 {
            return Err("Mesh has no vertices".into());
        }

        let start_vertex = match a.get("start_vertex") {
            Some(v) => integer(v, "start_vertex")?,
            None => 0,
        };
        let end_vertex = match a.get("end_vertex") {
            Some(v) => integer(v, "end_vertex")?,
            None => 1.min(nv.saturating_sub(1)),
        };

        if start_vertex >= nv {
            return Err(format!(
                "start_vertex {start_vertex} out of bounds (mesh has {nv} vertices)"
            ));
        }
        if end_vertex >= nv {
            return Err(format!(
                "end_vertex {end_vertex} out of bounds (mesh has {nv} vertices)"
            ));
        }

        self.charge((nv + nt) * 16 + 2048, nv * 64 + nt * 32 + 8192)?;

        // Build undirected adjacency list over triangle edges weighted by 3D Euclidean distance
        let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); nv];
        for tri in &mesh.triangles {
            let u0 = tri[0] as usize;
            let u1 = tri[1] as usize;
            let u2 = tri[2] as usize;
            for &(u, v) in &[(u0, u1), (u1, u2), (u2, u0)] {
                if u < nv && v < nv && u != v {
                    let pu = mesh.positions[u];
                    let pv = mesh.positions[v];
                    let dx = (pu[0] - pv[0]) as f64;
                    let dy = (pu[1] - pv[1]) as f64;
                    let dz = (pu[2] - pv[2]) as f64;
                    let d = (dx * dx + dy * dy + dz * dz).sqrt();
                    if !adj[u].iter().any(|&(neighbor, _)| neighbor == v) {
                        adj[u].push((v, d));
                        adj[v].push((u, d));
                    }
                }
            }
        }

        // Deterministic Dijkstra search
        let mut dist = vec![f64::INFINITY; nv];
        let mut prev: Vec<Option<usize>> = vec![None; nv];
        let mut heap = BinaryHeap::new();

        dist[start_vertex] = 0.0;
        heap.push(DijkstraItem {
            cost: 0.0,
            node: start_vertex,
        });

        while let Some(DijkstraItem { cost, node }) = heap.pop() {
            if node == end_vertex {
                break;
            }
            if cost > dist[node] {
                continue;
            }
            for &(next, weight) in &adj[node] {
                let next_cost = cost + weight;
                if next_cost < dist[next] {
                    dist[next] = next_cost;
                    prev[next] = Some(node);
                    heap.push(DijkstraItem {
                        cost: next_cost,
                        node: next,
                    });
                }
            }
        }

        let (path, total_dist) = if start_vertex == end_vertex {
            (vec![start_vertex], 0.0)
        } else {
            if dist[end_vertex].is_infinite() {
                return Err(format!(
                    "No path found between vertex {start_vertex} and {end_vertex} in mesh"
                ));
            }
            let mut p = Vec::new();
            let mut curr = Some(end_vertex);
            while let Some(u) = curr {
                p.push(u);
                if u == start_vertex {
                    break;
                }
                curr = prev[u];
            }
            p.reverse();
            (p, dist[end_vertex])
        };

        let hops = path.len().saturating_sub(1);
        let points_3d: Vec<[f64; 3]> = path
            .iter()
            .map(|&idx| {
                let pt = mesh.positions[idx];
                [pt[0] as f64, pt[1] as f64, pt[2] as f64]
            })
            .collect();

        // Project 3D path (x, y) into a 2D profile handle (minimum 3 vertices)
        let mut pts_2d: Vec<[f64; 2]> = points_3d.iter().map(|p| [p[0], p[1]]).collect();
        if pts_2d.len() < 3 {
            if pts_2d.len() == 1 {
                let p = pts_2d[0];
                pts_2d = vec![p, [p[0] + 0.01, p[1]], [p[0], p[1] + 0.01]];
            } else if pts_2d.len() == 2 {
                let a_pt = pts_2d[0];
                let b_pt = pts_2d[1];
                let dx = b_pt[0] - a_pt[0];
                let dy = b_pt[1] - a_pt[1];
                let len = (dx * dx + dy * dy).sqrt();
                let (nx, ny) = if len > 1e-9 {
                    (-dy / len, dx / len)
                } else {
                    (0.0, 1.0)
                };
                let offset = (len * 0.05).max(0.001);
                let mid = [
                    (a_pt[0] + b_pt[0]) * 0.5 + nx * offset,
                    (a_pt[1] + b_pt[1]) * 0.5 + ny * offset,
                ];
                pts_2d.insert(1, mid);
            }
        }

        if pts_2d.len() >= 3 {
            let pts_vec: Vec<Vec2> = pts_2d.iter().map(|p| Vec2::new(p[0], p[1])).collect();
            let area = drafting_geometry::shoelace(&pts_vec);
            if area.abs() < 1e-12 {
                let mid_idx = pts_2d.len() / 2;
                pts_2d[mid_idx][1] += 0.001;
            } else if area < 0.0 {
                pts_2d.reverse();
            }
        }

        let profile_2d = self.workspace.insert(Artifact::Profile(pts_2d), self.limits)?;

        Ok(Value::Record(BTreeMap::from([
            (
                "vertex_indices".into(),
                Value::List(path.iter().map(|&idx| Value::U64(idx as u64)).collect()),
            ),
            (
                "points_3d".into(),
                Value::List(
                    points_3d
                        .iter()
                        .map(|p| {
                            Value::List(vec![
                                Value::F64(p[0]),
                                Value::F64(p[1]),
                                Value::F64(p[2]),
                            ])
                        })
                        .collect(),
                ),
            ),
            ("total_length".into(), Value::F64(total_dist)),
            ("hop_count".into(), Value::U64(hops as u64)),
            ("profile_2d".into(), profile_2d),
        ])))
    }
}
