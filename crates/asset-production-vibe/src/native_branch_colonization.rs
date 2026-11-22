//! Native space colonization into an attributed branching scaffold.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use qualia_core_db::specialized_libs::computational_geometry::{
    space_colonization_3d, Point3, SpaceColonizationConfig, ROOT_PARENT,
};

use crate::{
    artifact::{BranchEdge, BranchGraph10d},
    count, fields, integer, list, num, number, record, required, text, vector, Args, AssetHost,
    TypedArtifact, Value,
};

const MAX_ATTRACTORS: usize = 8192;
const MAX_NODES: usize = 4096;
const MAX_SEEDS: usize = 1024;
const MAX_ITERATIONS: usize = 128;
// The native provider filters children within squared distance 1e-10 m² of any node.
const MIN_STEP_M: f64 = 1e-5;

impl AssetHost<'_> {
    pub(crate) fn colonize_branching_graph(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "algorithm",
                "attractors_m",
                "seed_nodes",
                "root_tropism",
                "step_m",
                "kill_radius_m",
                "perception_radius_m",
                "cone_angle_rad",
                "min_attractors_per_bud",
                "max_iterations",
                "max_nodes",
                "radius_m",
                "allometric_exponent",
                "initial_state",
            ],
        )?;
        if args
            .get("algorithm")
            .map(text)
            .transpose()?
            .unwrap_or("space_colonization")
            != "space_colonization"
        {
            return Err("This branch provider accepts algorithm space_colonization".into());
        }
        let attractors = list(required(args, "attractors_m")?)?;
        let seeds = list(required(args, "seed_nodes")?)?;
        if !(1..=MAX_ATTRACTORS).contains(&attractors.len())
            || !(1..=MAX_SEEDS).contains(&seeds.len())
        {
            return Err(
                "Branch colonization requires 1–8192 attractors and 1–1024 seed nodes".into(),
            );
        }
        let cfg = SpaceColonizationConfig {
            node_length: num(required(args, "step_m")?)?,
            kill_radius: num(required(args, "kill_radius_m")?)?,
            perception_radius: num(required(args, "perception_radius_m")?)?,
            cone_angle_rad: number(args, "cone_angle_rad", std::f64::consts::TAU)?,
            min_attractors_per_bud: count(args, "min_attractors_per_bud", 1, attractors.len())?,
            max_iterations: count(args, "max_iterations", 64, MAX_ITERATIONS)?,
        };
        let native_extent_m = f64::MAX.sqrt() / 16.0;
        if cfg.node_length <= MIN_STEP_M
            || cfg.node_length * cfg.max_iterations as f64 > native_extent_m
            || cfg.kill_radius <= 0.0
            || cfg.perception_radius < cfg.kill_radius
            || cfg.perception_radius > native_extent_m
            || cfg.cone_angle_rad <= 0.0
            || cfg.cone_angle_rad > std::f64::consts::TAU
        {
            return Err("Invalid native branch step, perception/kill radii or cone angle".into());
        }
        let edge = edge_state(args)?;
        let capacity = count(args, "max_nodes", MAX_NODES, MAX_NODES)?;
        if capacity < seeds.len() {
            return Err("Branch max_nodes is smaller than the seed topology".into());
        }
        let max_spawns_per_iteration = attractors.len() / cfg.min_attractors_per_bud;
        let capacity = capacity.min(seeds.len() + max_spawns_per_iteration * cfg.max_iterations);
        let work = capacity
            .saturating_mul(cfg.max_iterations)
            .saturating_mul(attractors.len() * 2 + max_spawns_per_iteration.min(capacity))
            .saturating_add(attractors.len().saturating_mul(seeds.len()));
        self.charge(
            work,
            capacity
                .saturating_mul(256)
                .saturating_add(attractors.len() * 96),
        )?;

        let mut seed_nodes = Vec::with_capacity(seeds.len());
        let mut seed_parents = Vec::with_capacity(seeds.len());
        for (index, seed) in seeds.iter().enumerate() {
            let seed = record(seed)?;
            fields(seed, &["position_m", "parent"])?;
            let position = vector::<3>(required(seed, "position_m")?)?;
            let parent = match seed.get("parent") {
                Some(value) => {
                    let parent = integer(value, "seed parent")?;
                    if parent >= index || seed_nodes[parent] == position {
                        return Err(format!(
                            "Branch seed {index} requires an earlier distinct parent node"
                        ));
                    }
                    parent as u32
                }
                None => ROOT_PARENT,
            };
            seed_nodes.push(position);
            seed_parents.push(parent);
        }
        let tropism = args
            .get("root_tropism")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0, 0.0, 1.0]);
        let frame = GrowthFrame::new(seed_nodes[0], tropism)?;
        let seed_local = seed_nodes
            .iter()
            .map(|&point| frame.local(point, native_extent_m))
            .collect::<Result<Vec<_>, _>>()?;
        let attractors = attractors
            .iter()
            .map(|value| vector::<3>(value).and_then(|p| frame.local(p, native_extent_m)))
            .collect::<Result<Vec<_>, _>>()?;
        let mut grown = vec![Point3::default(); capacity];
        let mut parents = vec![ROOT_PARENT; capacity];
        let receipt = space_colonization_3d(
            &attractors,
            &seed_local,
            &seed_parents,
            &cfg,
            &mut grown,
            &mut parents,
        )
        .map_err(|error| format!("Native branch colonization: {error:?}"))?;
        self.charge(0, 0)?;

        let seed_count = seed_nodes.len();
        let mut nodes = seed_nodes;
        nodes.extend(
            grown[seed_count..receipt.node_count]
                .iter()
                .map(|&point| frame.world(point)),
        );
        let mut has_children = vec![false; nodes.len()];
        let mut roots = Vec::new();
        let mut edges = Vec::with_capacity(nodes.len().saturating_sub(1));
        for (child, &parent) in parents[..nodes.len()].iter().enumerate() {
            if parent == ROOT_PARENT {
                roots.push(Value::U64(child as u64));
                continue;
            }
            let source = parent as usize;
            if source >= child
                || nodes[source] == nodes[child]
                || nodes[child]
                    .iter()
                    .any(|coordinate| !coordinate.is_finite())
            {
                return Err(format!(
                    "Branch edge {parent}–{child} does not resolve to a finite, nonzero segment"
                ));
            }
            edges.push(BranchEdge {
                nodes: [parent, child as u32],
                ..edge.clone()
            });
            has_children[source] = true;
        }
        let terminal_buds = has_children
            .into_iter()
            .enumerate()
            .filter_map(|(index, has_children)| (!has_children).then_some(Value::U64(index as u64)))
            .collect();
        let graph =
            self.admit_artifact(TypedArtifact::BranchGraph10d(Arc::new(BranchGraph10d {
                nodes,
                edges,
            })))?;
        Ok(Value::Record(Args::from([
            ("graph".into(), graph),
            ("roots".into(), Value::List(roots)),
            ("terminal_buds".into(), Value::List(terminal_buds)),
            (
                "iterations_run".into(),
                Value::U64(receipt.iterations_run as u64),
            ),
            (
                "remaining_attractors".into(),
                Value::U64(receipt.remaining_attractors as u64),
            ),
        ])))
    }
}

fn edge_state(args: &Args) -> Result<BranchEdge, String> {
    let empty = Args::new();
    let state = args
        .get("initial_state")
        .map(record)
        .transpose()?
        .unwrap_or(&empty);
    fields(
        state,
        &[
            "water_potential_pa",
            "hydraulic_conductance",
            "metabolite_concentration",
            "defense_signal",
        ],
    )?;
    let edge = BranchEdge {
        nodes: [0; 2],
        radius_m: num(required(args, "radius_m")?)?,
        allometric_exponent: number(args, "allometric_exponent", 3.0)?,
        water_potential_pa: number(state, "water_potential_pa", 0.0)?,
        hydraulic_conductance: number(state, "hydraulic_conductance", 0.0)?,
        metabolite_concentration: number(state, "metabolite_concentration", 0.0)?,
        defense_signal: number(state, "defense_signal", 0.0)?,
    };
    if edge.radius_m <= 0.0
        || edge.allometric_exponent <= 0.0
        || edge.hydraulic_conductance < 0.0
        || edge.metabolite_concentration < 0.0
        || edge.defense_signal < 0.0
    {
        return Err("Branch edge radius and allometric exponent must be positive and initial transport state nonnegative".into());
    }
    Ok(edge)
}

struct GrowthFrame {
    origin: [f64; 3],
    axes: [[f64; 3]; 3],
}

impl GrowthFrame {
    fn new(origin: [f64; 3], direction: [f64; 3]) -> Result<Self, String> {
        // The provider uses local +Z only for root headings.
        let z = unit(direction).ok_or("Branch root_tropism must be a nonzero direction")?;
        let (axis, reference) = if z[0].abs() > 0.8 {
            (1, [0.0, 1.0, 0.0])
        } else {
            (0, [1.0, 0.0, 0.0])
        };
        let x = unit(std::array::from_fn(|i| reference[i] - z[i] * z[axis]))
            .ok_or("Branch root_tropism did not yield a stable reference frame")?;
        let y = [
            z[1] * x[2] - z[2] * x[1],
            z[2] * x[0] - z[0] * x[2],
            z[0] * x[1] - z[1] * x[0],
        ];
        Ok(Self {
            origin,
            axes: [x, y, z],
        })
    }

    fn local(&self, point: [f64; 3], limit: f64) -> Result<Point3, String> {
        let relative: [f64; 3] = std::array::from_fn(|i| point[i] - self.origin[i]);
        let projected = self.axes.map(|axis| {
            relative
                .into_iter()
                .zip(axis)
                .map(|(p, a)| p * a)
                .sum::<f64>()
        });
        if projected
            .iter()
            .any(|coordinate| !coordinate.is_finite() || coordinate.abs() > limit)
        {
            return Err("Branch coordinates exceed the native squared-distance range".into());
        }
        Ok(Point3::new(projected[0], projected[1], projected[2]))
    }

    fn world(&self, point: Point3) -> [f64; 3] {
        let [x, y, z] = self.axes;
        std::array::from_fn(|i| self.origin[i] + (x[i] * point.x + y[i] * point.y + z[i] * point.z))
    }
}

fn unit(vector: [f64; 3]) -> Option<[f64; 3]> {
    let max = vector.into_iter().map(f64::abs).fold(0.0, f64::max);
    if max == 0.0 {
        return None;
    }
    let scaled = vector.map(|axis| axis / max);
    let norm = scaled[0].hypot(scaled[1]).hypot(scaled[2]);
    Some(scaled.map(|axis| axis / norm))
}
