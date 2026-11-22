//! Bounded 2.5D terrain navigation and authored funnel portals.

use crate::{
    artifact::FieldLattice, fields, num, required, vector, Args, AssetHost, TypedArtifact, Value,
};
use qualia_core_db::{
    render::assets::Mesh,
    simulation::navigation::{
        ZeroHeapNavMesh, MAX_NAVMESH_NODES, MAX_PATH_WAYPOINTS, MAX_POLY_VERTICES,
    },
    specialized_libs::computational_geometry::{orient_3d, Point3},
};
use std::{collections::BTreeMap, sync::Arc};

const MAX_AXIS: usize = 129;
const ROW_WIDTH: usize = 1 + 3 + 3 * MAX_POLY_VERTICES + MAX_POLY_VERTICES;
const NO_NEIGHBOR: u8 = 255;

#[derive(Clone, Copy)]
enum Diagonal {
    Descending,
    Ascending,
}

impl Diagonal {
    const fn triangles(self) -> [[usize; 3]; 2] {
        match self {
            Self::Descending => [[0, 1, 2], [0, 2, 3]],
            Self::Ascending => [[0, 1, 3], [1, 2, 3]],
        }
    }
}

struct Terrain {
    x: Vec<f32>,
    z: Vec<f32>,
    heights: Vec<Option<f32>>,
    cells: Vec<Option<Diagonal>>,
}

impl Terrain {
    fn cell_width(&self) -> usize {
        self.x.len() - 1
    }
    fn cell_height(&self) -> usize {
        self.z.len() - 1
    }
    fn cell_index(&self, x: usize, z: usize) -> usize {
        z * self.cell_width() + x
    }
    fn corner(&self, x: usize, z: usize) -> usize {
        z * self.x.len() + x
    }
    fn quad(&self, x0: usize, z0: usize, x1: usize, z1: usize) -> [usize; 4] {
        [
            self.corner(x0, z0),
            self.corner(x1, z0),
            self.corner(x1, z1),
            self.corner(x0, z1),
        ]
    }
    fn point(&self, index: usize) -> Option<[f32; 3]> {
        Some([
            self.x[index % self.x.len()],
            self.heights[index]?,
            self.z[index / self.x.len()],
        ])
    }
    fn bounds(&self, x: usize, z: usize) -> [[f64; 2]; 2] {
        [
            [f64::from(self.x[x]), f64::from(self.z[z])],
            [f64::from(self.x[x + 1]), f64::from(self.z[z + 1])],
        ]
    }
}

struct NavTerrain {
    native: ZeroHeapNavMesh,
    heights: [[f32; MAX_POLY_VERTICES]; MAX_NAVMESH_NODES],
    centre_heights: [f32; MAX_NAVMESH_NODES],
}

impl AssetHost<'_> {
    pub(crate) fn build_navmesh(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "cell_size_m",
                "origin_m",
                "agent_radius_m",
                "max_slope_deg",
                "step_height_m",
                "start",
                "goal",
            ],
        )?;
        let radius = metric(args, "agent_radius_m", 0.0, 10.0)?;
        let slope = metric(args, "max_slope_deg", 0.0, 85.0)?;
        let step = metric(args, "step_height_m", 0.0, 10.0)?;
        let terrain = terrain(self, required(args, "source")?, args)?;
        let cell_count = terrain.cells.len();
        self.charge(cell_count * 72, cell_count * 192)?;
        let walkable = walkable_cells(&terrain, slope.to_radians().tan(), step)?;
        let walkable_count = walkable.iter().filter(|&&value| value).count();
        if radius > 0.0 {
            self.charge(
                walkable_count * (cell_count - walkable_count + 1) * 8,
                cell_count * 40,
            )?;
        }
        let planar = if radius > 0.0 && walkable_count == cell_count {
            inset_planar(&terrain, radius)?
        } else {
            None
        };
        let (tile, retained_count) = if let Some((plane, clipped_cells)) = planar {
            (
                assemble(&plane, &[plane.quad(0, 0, 1, 1).to_vec()])?,
                clipped_cells,
            )
        } else {
            let retained = erode_cells(&terrain, &walkable, radius);
            let retained_count = retained.iter().filter(|&&value| value).count();
            if retained_count == 0 {
                return Err("The source contains no navigable cells at the requested slope, step, and radius".into());
            }
            let polygons = compact(&terrain, &retained)?;
            (assemble(&terrain, &polygons)?, retained_count)
        };
        let route = match (args.get("start"), args.get("goal")) {
            (Some(start), Some(goal)) => Some(route(&tile, vector(start)?, vector(goal)?)?),
            (None, None) => None,
            _ => return Err("Supply both XZ route endpoints or neither".into()),
        };
        let native = &tile.native;
        let mut rows = Vec::with_capacity(native.node_count * ROW_WIDTH);
        for (index, polygon) in native.polygons[..native.node_count].iter().enumerate() {
            rows.extend([
                f32::from(polygon.vertex_count),
                polygon.center[0],
                tile.centre_heights[index],
                polygon.center[1],
            ]);
            for (point, &height) in polygon.vertices.iter().zip(&tile.heights[index]) {
                rows.extend([point[0], height, point[1]]);
            }
            rows.extend(polygon.neighbors.map(f32::from));
        }
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![native.node_count as u32, ROW_WIDTH as u32],
                samples: rows,
            },
        )))?;
        let mut out = BTreeMap::from([
            ("navmesh".into(), field),
            ("layout".into(), Value::String("navmesh25d.xyz-v1".into())),
            ("polygon_count".into(), Value::U64(native.node_count as u64)),
            (
                "retained_cell_count".into(),
                Value::U64(retained_count as u64),
            ),
            (
                "walkable_before_radius".into(),
                Value::U64(walkable_count as u64),
            ),
            ("vertex_slots".into(), Value::U64(MAX_POLY_VERTICES as u64)),
            ("wall_neighbor".into(), Value::U64(u64::from(NO_NEIGHBOR))),
        ]);
        if let Some(route) = route {
            out.insert("route".into(), route);
        }
        Ok(Value::Record(out))
    }
}

fn metric(args: &Args, key: &str, min: f64, max: f64) -> Result<f64, String> {
    let value = num(required(args, key)?)?;
    (min..=max)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} is outside {min}..={max}"))
}

fn terrain(host: &AssetHost<'_>, source: &Value, args: &Args) -> Result<Terrain, String> {
    match host.workspace.typed(source) {
        Ok(TypedArtifact::AttributedMesh(mesh)) => {
            forbid_overrides(args)?;
            mesh_grid(&mesh.surface.mesh)
        }
        Ok(TypedArtifact::FieldLattice(field)) => {
            let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
                return Err("Navmesh FieldLattice source requires DenseF32 height samples".into());
            };
            let [height, width] = axes.as_slice() else {
                return Err("Navmesh FieldLattice axes must be [height, width]".into());
            };
            let origin = args
                .get("origin_m")
                .map(vector::<3>)
                .transpose()?
                .unwrap_or([0.0; 3]);
            let spacing = metric(args, "cell_size_m", 1e-5, 10_000.0)?;
            regular_grid(*width as usize, *height as usize, spacing, origin, samples)
        }
        Ok(_) => Err("Navmesh source must be a heightfield or an AttributedMesh".into()),
        Err(_) => {
            forbid_overrides(args)?;
            let (side, spacing, [east, north, up], height) =
                crate::geodesy_ops::parse_heightfield(source)?;
            regular_grid(side, side, spacing, [east, up, north], &height)
        }
    }
}

fn forbid_overrides(args: &Args) -> Result<(), String> {
    if args.contains_key("cell_size_m") || args.contains_key("origin_m") {
        return Err("cell_size_m and origin_m only configure a DenseF32 heightfield".into());
    }
    Ok(())
}

fn grid_size(width: usize, height: usize) -> Result<(), String> {
    if !(2..=MAX_AXIS).contains(&width) || !(2..=MAX_AXIS).contains(&height) {
        return Err(format!(
            "Navmesh terrain requires axes of 2..={MAX_AXIS} samples"
        ));
    }
    Ok(())
}

fn regular_grid(
    width: usize,
    height: usize,
    spacing: f64,
    origin: [f64; 3],
    samples: &[f32],
) -> Result<Terrain, String> {
    grid_size(width, height)?;
    if samples.len() != width * height {
        return Err("Navmesh height count differs from its raster shape".into());
    }
    let x = (0..width)
        .map(|i| (origin[0] + i as f64 * spacing) as f32)
        .collect::<Vec<_>>();
    let z = (0..height)
        .map(|i| (origin[2] + i as f64 * spacing) as f32)
        .collect::<Vec<_>>();
    let heights = samples
        .iter()
        .map(|&h| Some((origin[1] + f64::from(h)) as f32))
        .collect::<Vec<_>>();
    if !valid_axis(&x) || !valid_axis(&z) || heights.iter().flatten().any(|&h| !valid_coordinate(h))
    {
        return Err("Heightfield calibration loses sample precision or exceeds finite navigation coordinates".into());
    }
    Ok(Terrain {
        x,
        z,
        heights,
        cells: vec![Some(Diagonal::Descending); (width - 1) * (height - 1)],
    })
}

fn valid_coordinate(value: f32) -> bool {
    value.is_finite() && value.abs() <= 1e8
}
fn valid_axis(axis: &[f32]) -> bool {
    axis.iter().all(|&v| valid_coordinate(v)) && axis.windows(2).all(|pair| pair[0] < pair[1])
}

fn canonical_zero(value: f32) -> f32 {
    if value == 0.0 {
        0.0
    } else {
        value
    }
}

fn mesh_grid(mesh: &Mesh) -> Result<Terrain, String> {
    if mesh.positions.len() > MAX_AXIS * MAX_AXIS
        || mesh.triangles.len() > (MAX_AXIS - 1) * (MAX_AXIS - 1) * 2
        || mesh
            .positions
            .iter()
            .flatten()
            .any(|&value| !valid_coordinate(value))
    {
        return Err("AttributedMesh exceeds the bounded single-sheet terrain domain".into());
    }
    let axis = |lane: usize| {
        let mut values: Vec<_> = mesh
            .positions
            .iter()
            .map(|p| canonical_zero(p[lane]))
            .collect();
        values.sort_unstable_by(f32::total_cmp);
        values.dedup();
        values
    };
    let (x, z) = (axis(0), axis(2));
    grid_size(x.len(), z.len())?;
    let mut heights = vec![None; x.len() * z.len()];
    let mut tags = Vec::with_capacity(mesh.positions.len());
    for &[px, py, pz] in &mesh.positions {
        let lookup = |axis: &[f32], value| {
            axis.binary_search_by(|entry| entry.total_cmp(&canonical_zero(value)))
                .map_err(|_| "AttributedMesh coordinate escaped its rectilinear terrain grid")
        };
        let coord = [lookup(&x, px)?, lookup(&z, pz)?];
        let height = &mut heights[coord[1] * x.len() + coord[0]];
        if height.is_some_and(|previous| previous != py) {
            return Err("AttributedMesh must have one elevation per XZ node".into());
        }
        *height = Some(py);
        tags.push(coord);
    }
    let mut triangles = vec![[0u8; 2]; (x.len() - 1) * (z.len() - 1)];
    for &[a, b, c] in &mesh.triangles {
        let tag = |index: u32| {
            tags.get(index as usize)
                .copied()
                .ok_or("AttributedMesh triangle index is out of bounds")
        };
        let coordinates = [tag(a)?, tag(b)?, tag(c)?];
        let [x0, z0] = std::array::from_fn(|axis| {
            coordinates[0][axis]
                .min(coordinates[1][axis])
                .min(coordinates[2][axis])
        });
        let [x1, z1] = std::array::from_fn(|axis| {
            coordinates[0][axis]
                .max(coordinates[1][axis])
                .max(coordinates[2][axis])
        });
        if x1 - x0 != 1 || z1 - z0 != 1 {
            return Err("AttributedMesh triangles must occupy one rectilinear terrain cell".into());
        }
        let corners = [[x0, z0], [x1, z0], [x1, z1], [x0, z1]];
        let mut tag = 0u8;
        for point in coordinates {
            let corner = corners
                .iter()
                .position(|&candidate| candidate == point)
                .ok_or("Invalid terrain grid corner")?;
            tag |= 1 << corner;
        }
        if tag.count_ones() != 3 {
            return Err("AttributedMesh contains a collapsed terrain triangle".into());
        }
        let masks = &mut triangles[z0 * (x.len() - 1) + x0];
        let lane = masks
            .iter_mut()
            .find(|entry| **entry == 0)
            .ok_or("Terrain cell has more than two triangles")?;
        *lane = tag;
    }
    let cells = triangles
        .into_iter()
        .map(|mut masks| {
            masks.sort_unstable();
            match masks {
                [0, _] => Ok(None),
                [0b0111, 0b1101] => Ok(Some(Diagonal::Descending)),
                [0b1011, 0b1110] => Ok(Some(Diagonal::Ascending)),
                _ => Err("AttributedMesh has overlapping triangles or an invalid cell diagonal"),
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Terrain {
        x,
        z,
        heights,
        cells,
    })
}

fn walkable_cells(terrain: &Terrain, tangent: f64, step: f64) -> Result<Vec<bool>, String> {
    let mut accepted = Vec::with_capacity(terrain.cells.len());
    for (index, diagonal) in terrain.cells.iter().enumerate() {
        let Some(diagonal) = diagonal else {
            accepted.push(false);
            continue;
        };
        let x = index % terrain.cell_width();
        let z = index / terrain.cell_width();
        let [a, b, c, d] = terrain.quad(x, z, x + 1, z + 1).map(|corner| {
            terrain
                .point(corner)
                .ok_or("Walkable terrain cell lacks an elevation")
        });
        let points = [a?, b?, c?, d?].map(|p| p.map(f64::from));
        let within_step = (0..4).all(|i| (points[i][1] - points[(i + 1) % 4][1]).abs() <= step);
        accepted.push(
            within_step
                && diagonal.triangles().into_iter().all(|indices| {
                    let [a, b, c] = indices.map(|i| points[i]);
                    let u: [f64; 3] = std::array::from_fn(|i| b[i] - a[i]);
                    let v: [f64; 3] = std::array::from_fn(|i| c[i] - a[i]);
                    let horizontal = (u[1] * v[2] - u[2] * v[1]).hypot(u[0] * v[1] - u[1] * v[0]);
                    let up = (u[2] * v[0] - u[0] * v[2]).abs();
                    horizontal <= up * tangent
                }),
        );
    }
    Ok(accepted)
}

fn erode_cells(terrain: &Terrain, walkable: &[bool], radius: f64) -> Vec<bool> {
    if radius == 0.0 {
        return walkable.to_vec();
    }
    let width = terrain.cell_width();
    let height = terrain.cell_height();
    let blocked: Vec<_> = walkable
        .iter()
        .enumerate()
        .filter_map(|(i, &walk)| (!walk).then(|| terrain.bounds(i % width, i / width)))
        .collect();
    let outer_min = terrain.bounds(0, 0)[0];
    let outer_max = terrain.bounds(width - 1, height - 1)[1];
    walkable
        .iter()
        .enumerate()
        .map(|(i, &walk)| {
            if !walk {
                return false;
            }
            let [lo, hi] = terrain.bounds(i % width, i / width);
            (0..2).all(|axis| {
                lo[axis] - outer_min[axis] >= radius && outer_max[axis] - hi[axis] >= radius
            }) && blocked.iter().all(|&[blocked_lo, blocked_hi]| {
                let distance2: f64 = (0..2)
                    .map(|axis| {
                        (blocked_lo[axis] - hi[axis])
                            .max(lo[axis] - blocked_hi[axis])
                            .max(0.0)
                            .powi(2)
                    })
                    .sum();
                distance2 >= radius * radius
            })
        })
        .collect()
}

fn point3(point: [f32; 3]) -> Point3 {
    Point3::new(
        f64::from(point[0]),
        f64::from(point[1]),
        f64::from(point[2]),
    )
}

fn inset_planar(terrain: &Terrain, radius: f64) -> Result<Option<(Terrain, usize)>, String> {
    let [a, b, c, d] = terrain
        .quad(0, 0, 1, 1)
        .map(|index| terrain.point(index).map(point3));
    let (Some(a), Some(b), Some(d)) = (a, b, d) else {
        return Ok(None);
    };
    if c.is_none()
        || (0..terrain.heights.len()).any(|i| {
            terrain
                .point(i)
                .is_none_or(|p| orient_3d(a, b, d, point3(p)) as i32 != 0)
        })
    {
        return Ok(None);
    }
    let inset_axis = |axis: &[f32]| {
        let min = f64::from(axis[0]) + radius;
        let max = f64::from(axis[axis.len() - 1]) - radius;
        let (mut lo, mut hi) = (min as f32, max as f32);
        if f64::from(lo) < min {
            lo = lo.next_up();
        }
        if f64::from(hi) > max {
            hi = hi.next_down();
        }
        (lo < hi).then_some([lo, hi])
    };
    let (Some(x), Some(z)) = (inset_axis(&terrain.x), inset_axis(&terrain.z)) else {
        return Ok(None);
    };
    let dy_dx = (b.y - a.y) / (b.x - a.x);
    let dy_dz = (d.y - a.y) / (d.z - a.z);
    let elevation = |x: f32, z: f32| {
        Some((a.y + (f64::from(x) - a.x) * dy_dx + (f64::from(z) - a.z) * dy_dz) as f32)
    };
    let heights = vec![
        elevation(x[0], z[0]),
        elevation(x[1], z[0]),
        elevation(x[0], z[1]),
        elevation(x[1], z[1]),
    ];
    let crossed = |axis: &[f32], [low, high]: [f32; 2]| {
        axis.windows(2)
            .filter(|pair| pair[0] < high && pair[1] > low)
            .count()
    };
    let clipped_cells = crossed(&terrain.x, x) * crossed(&terrain.z, z);
    Ok(Some((
        Terrain {
            x: x.to_vec(),
            z: z.to_vec(),
            heights,
            cells: vec![Some(Diagonal::Descending)],
        },
        clipped_cells,
    )))
}

fn compact(terrain: &Terrain, retained: &[bool]) -> Result<Vec<Vec<usize>>, String> {
    let width = terrain.cell_width();
    let height = terrain.cell_height();
    let planar_patch = |x: usize, z: usize| {
        let quad = terrain
            .quad(x, z, x + 1, z + 1)
            .map(|i| terrain.point(i).map(point3));
        match quad {
            [Some(a), Some(b), Some(c), Some(d)] if orient_3d(a, b, c, d) as i32 == 0 => {
                Some([a, b, c, d])
            }
            _ => None,
        }
    };
    let can_join = |(ax, az), (bx, bz)| {
        let a = retained[terrain.cell_index(ax, az)];
        let b = retained[terrain.cell_index(bx, bz)];
        match (a, b) {
            (false, false) => true,
            (true, true) => match (planar_patch(ax, az), planar_patch(bx, bz)) {
                (Some([a, b, c, _]), Some(other)) => {
                    other.into_iter().all(|p| orient_3d(a, b, c, p) as i32 == 0)
                }
                _ => false,
            },
            _ => false,
        }
    };
    let mut x_cuts = vec![0];
    x_cuts.extend((1..width).filter(|&x| (0..height).any(|z| !can_join((x - 1, z), (x, z)))));
    x_cuts.push(width);
    let mut z_cuts = vec![0];
    z_cuts.extend((1..height).filter(|&z| (0..width).any(|x| !can_join((x, z - 1), (x, z)))));
    z_cuts.push(height);
    let mut polygons = Vec::with_capacity(MAX_NAVMESH_NODES);
    for z in z_cuts.windows(2) {
        for x in x_cuts.windows(2) {
            let cell = terrain.cell_index(x[0], z[0]);
            if !retained[cell] {
                continue;
            }
            let corners = terrain.quad(x[0], z[0], x[1], z[1]);
            if planar_patch(x[0], z[0]).is_some() {
                polygons.push(corners.to_vec());
            } else {
                let diagonal = terrain.cells[cell]
                    .ok_or("Retained terrain cell lacks its authored diagonal")?;
                polygons.extend(
                    diagonal
                        .triangles()
                        .map(|face| face.map(|index| corners[index]).to_vec()),
                );
            }
            if polygons.len() > MAX_NAVMESH_NODES {
                return Err(format!(
                    "Convex terrain sector exceeds the native {MAX_NAVMESH_NODES}-polygon capacity"
                ));
            }
        }
    }
    Ok(polygons)
}

fn assemble(terrain: &Terrain, polygons: &[Vec<usize>]) -> Result<NavTerrain, String> {
    let mut native = ZeroHeapNavMesh::default();
    native.node_count = polygons.len();
    let mut heights = [[0.0; MAX_POLY_VERTICES]; MAX_NAVMESH_NODES];
    let mut centre_heights = [0.0; MAX_NAVMESH_NODES];
    let mut half_edges = BTreeMap::new();
    for (index, face) in polygons.iter().enumerate() {
        let polygon = &mut native.polygons[index];
        polygon.vertex_count = face.len() as u8;
        let mut centre = [0.0; 3];
        for (lane, &vertex) in face.iter().enumerate() {
            let p = terrain
                .point(vertex)
                .ok_or("Navmesh polygon refers to an absent terrain vertex")?;
            polygon.vertices[lane] = [p[0], p[2]];
            heights[index][lane] = p[1];
            for axis in 0..3 {
                centre[axis] += f64::from(p[axis]);
            }
        }
        let centre = centre.map(|v| (v / face.len() as f64) as f32);
        polygon.center = [centre[0], centre[2]];
        centre_heights[index] = centre[1];
        for (edge, &a) in face.iter().enumerate() {
            let b = face[(edge + 1) % face.len()];
            if half_edges.insert([a, b], (index, edge)).is_some() {
                return Err("Navmesh has a nonmanifold oriented edge".into());
            }
        }
    }
    for (&[a, b], &(node, edge)) in &half_edges {
        if let Some(&(neighbor, _)) = half_edges.get(&[b, a]) {
            native.polygons[node].neighbors[edge] = neighbor as u8;
        }
    }
    Ok(NavTerrain {
        native,
        heights,
        centre_heights,
    })
}

fn area(a: [f32; 2], b: [f32; 2], p: [f64; 2]) -> f64 {
    let [a, b] = [a, b].map(|point| point.map(f64::from));
    (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])
}

fn locate(nav: &NavTerrain, xz: [f64; 2]) -> Result<(usize, [f64; 3]), String> {
    if xz.iter().any(|v| !v.is_finite() || v.abs() > 1e8) {
        return Err("Route endpoint is outside the finite navigation coordinate domain".into());
    }
    let (node, polygon) = nav.native.polygons[..nav.native.node_count]
        .iter()
        .enumerate()
        .find(|(_, polygon)| {
            let vertices = &polygon.vertices[..usize::from(polygon.vertex_count)];
            (0..vertices.len())
                .all(|i| area(vertices[i], vertices[(i + 1) % vertices.len()], xz) >= 0.0)
        })
        .ok_or("Route endpoint is outside radius-cleared navigation polygons")?;
    let [a, b, c, ..] = polygon.vertices;
    let denom = area(a, b, c.map(f64::from));
    let weights = [
        area(b, c, xz) / denom,
        area(c, a, xz) / denom,
        area(a, b, xz) / denom,
    ];
    let elevation = weights
        .into_iter()
        .zip(nav.heights[node])
        .map(|(w, h)| w * f64::from(h))
        .sum();
    Ok((node, [xz[0], elevation, xz[1]]))
}

fn coordinates(point: [f64; 3]) -> Value {
    Value::List(point.into_iter().map(Value::F64).collect())
}

fn route(nav: &NavTerrain, start: [f64; 2], goal: [f64; 2]) -> Result<Value, String> {
    let (start_node, start) = locate(nav, start)?;
    let (goal_node, goal) = locate(nav, goal)?;
    let native = &nav.native;
    let a = native.polygons[start_node].center;
    let b = native.polygons[goal_node].center;
    if native.find_containing_polygon(a) != Some(start_node)
        || native.find_containing_polygon(b) != Some(goal_node)
    {
        return Err(
            "Route polygon centres are ambiguous at the native navigation precision".into(),
        );
    }
    let mut points = [[0.0; 2]; MAX_PATH_WAYPOINTS];
    let count = native.find_path(a, b, &mut points);
    if count == 0 {
        return Err("No connected navigation corridor between the supplied route endpoints".into());
    }
    let mut nodes = vec![start_node];
    for point in &points[1..count - 1] {
        let node = native.polygons[..native.node_count]
            .iter()
            .position(|polygon| polygon.center == *point)
            .ok_or("Native navigation waypoint cannot be matched to its polygon")?;
        nodes.push(node);
    }
    if goal_node != start_node {
        nodes.push(goal_node);
    }
    let mut portals = Vec::with_capacity(nodes.len().saturating_sub(1));
    for pair in nodes.windows(2) {
        let polygon = &native.polygons[pair[0]];
        let count = usize::from(polygon.vertex_count);
        let edge = polygon.neighbors[..count]
            .iter()
            .position(|&id| id as usize == pair[1])
            .ok_or("Native route exceeded the bounded corridor waypoint count")?;
        let endpoint = |i: usize| {
            coordinates([
                f64::from(polygon.vertices[i][0]),
                f64::from(nav.heights[pair[0]][i]),
                f64::from(polygon.vertices[i][1]),
            ])
        };
        portals.push(Value::Record(BTreeMap::from([
            ("left".into(), endpoint((edge + 1) % count)),
            ("right".into(), endpoint(edge)),
        ])));
    }
    Ok(Value::Record(BTreeMap::from([
        ("start".into(), coordinates(start)),
        ("goal".into(), coordinates(goal)),
        ("portals".into(), Value::List(portals)),
    ])))
}
