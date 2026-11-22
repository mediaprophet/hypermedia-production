//! Steady laminar pipe flow from prescribed total heads and node withdrawals.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use chronicle_tabular_engine::Value as CellValue;

use super::{bounded, least_squares, table_receipt};
use crate::{Args, AssetHost, Value, fields, integer, list, record, required};

struct Node {
    elevation_m: f64,
    head_m: Option<f64>,
    demand: f64,
}

struct Pipe {
    from: usize,
    to: usize,
    diameter_m: f64,
    conductance: f64,
}

impl AssetHost<'_> {
    pub(crate) fn solve_hydraulic_pipe_and_stormwater(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["nodes", "pipes", "density_kg_m3", "viscosity_pa_s"])?;
        let node_input = list(required(args, "nodes")?)?;
        let pipe_input = list(required(args, "pipes")?)?;
        if !(2..=128).contains(&node_input.len()) || !(1..=512).contains(&pipe_input.len()) {
            return Err("Steady hydraulics requires 2–128 nodes and 1–512 pipes".into());
        }
        let n = node_input.len();
        self.charge(
            n * n * n * 8 + pipe_input.len() * 128,
            n * n * 64 + pipe_input.len() * 4096,
        )?;
        let density = bounded(args, "density_kg_m3", Some(998.2), 100.0, 3000.0)?;
        let viscosity = bounded(args, "viscosity_pa_s", Some(0.001002), 1e-6, 100.0)?;
        let nodes = node_input
            .iter()
            .map(|value| {
                let row = record(value)?;
                fields(
                    row,
                    &["elevation_m", "fixed_total_head_m", "withdrawal_m3_s"],
                )?;
                let elevation_m = bounded(row, "elevation_m", Some(0.0), -1000.0, 10000.0)?;
                let head_m = row
                    .contains_key("fixed_total_head_m")
                    .then(|| bounded(row, "fixed_total_head_m", None, -1010.0, 12000.0))
                    .transpose()?;
                let demand = bounded(row, "withdrawal_m3_s", Some(0.0), 0.0, 1000.0)?;
                Ok(Node {
                    elevation_m,
                    head_m,
                    demand,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut unknowns = Vec::new();
        let index = nodes
            .iter()
            .enumerate()
            .map(|(i, node)| {
                if node.head_m.is_some() {
                    None
                } else {
                    let next = unknowns.len();
                    unknowns.push(i);
                    Some(next)
                }
            })
            .collect::<Vec<_>>();
        let m = unknowns.len();
        if m == n {
            return Err("At least one fixed total head is required".into());
        }
        let mut degree = vec![0usize; n];
        let mut matrix = vec![0.0; m * m];
        let mut rhs = unknowns
            .iter()
            .map(|&i| -nodes[i].demand)
            .collect::<Vec<_>>();
        let mut pipes = Vec::with_capacity(pipe_input.len());
        for value in pipe_input {
            let row = record(value)?;
            fields(row, &["from", "to", "length_m", "diameter_m"])?;
            let from = integer(required(row, "from")?, "pipe source node")?;
            let to = integer(required(row, "to")?, "pipe destination node")?;
            if from >= n || to >= n || from == to {
                return Err("Pipe endpoints must be distinct existing nodes".into());
            }
            let length_m = bounded(row, "length_m", None, 0.001, 100_000.0)?;
            let diameter_m = bounded(row, "diameter_m", None, 1e-5, 10.0)?;
            let conductance = std::f64::consts::PI * density * 9.80665 * diameter_m.powi(4)
                / (128.0 * viscosity * length_m);
            for (a, b) in [(from, to), (to, from)] {
                degree[a] += 1;
                if let Some(i) = index[a] {
                    matrix[i * m + i] += conductance;
                    match index[b] {
                        Some(j) => matrix[i * m + j] -= conductance,
                        None => {
                            rhs[i] += conductance * nodes[b].head_m.ok_or("Missing fixed head")?
                        }
                    }
                }
            }
            pipes.push(Pipe {
                from,
                to,
                diameter_m,
                conductance,
            });
        }
        if degree.contains(&0) {
            return Err("Hydraulic network has an isolated node".into());
        }
        // Equilibrate rows before QR: large conductance differences should not
        // impose a common dimensionful numerical pivot threshold.
        for i in 0..m {
            let norm = matrix[i * m + i];
            for value in &mut matrix[i * m..(i + 1) * m] {
                *value /= norm;
            }
            rhs[i] /= norm;
        }
        let unknown_heads = if m > 0 {
            least_squares(m, m, matrix, rhs)?
        } else {
            Vec::new()
        };
        let heads = nodes
            .iter()
            .zip(index)
            .map(|(node, index)| match index {
                Some(i) => unknown_heads[i],
                None => node.head_m.unwrap_or(0.0),
            })
            .collect::<Vec<_>>();
        let mut net_out = vec![0.0; n];
        let mut rows = Vec::with_capacity(n + pipes.len());
        for (i, pipe) in pipes.iter().enumerate() {
            let delta = heads[pipe.from] - heads[pipe.to];
            let flow = pipe.conductance * delta;
            let reynolds =
                4.0 * density * flow.abs() / (std::f64::consts::PI * viscosity * pipe.diameter_m);
            if flow.abs() > 1000.0 || reynolds >= 2300.0 {
                return Err(format!(
                    "Pipe {i} outside laminar model: Re={reynolds:.9e}, Q={flow:.9e} m³/s"
                ));
            }
            net_out[pipe.from] += flow;
            net_out[pipe.to] -= flow;
            rows.push(vec![
                CellValue::text("pipe"),
                CellValue::number(i as f64),
                CellValue::number(delta),
                CellValue::number(flow),
                CellValue::number(reynolds),
            ]);
        }
        let mut worst_residual = 0.0_f64;
        for (i, node) in nodes.iter().enumerate() {
            let pressure_head = heads[i] - node.elevation_m;
            if !(-10.0..=2000.0).contains(&pressure_head) {
                return Err(format!(
                    "Node {i} pressure head {pressure_head:.9e} m outside admitted range"
                ));
            }
            let residual_or_supply = net_out[i] + node.demand;
            if node.head_m.is_none() {
                worst_residual = worst_residual.max(residual_or_supply.abs());
                if residual_or_supply.abs() > 1e-9 {
                    return Err(format!(
                        "Node {i} flow residual {residual_or_supply:.9e} m³/s exceeds 1e-9 m³/s"
                    ));
                }
            }
            rows.push(vec![
                CellValue::text(if node.head_m.is_some() {
                    "reservoir"
                } else {
                    "junction"
                }),
                CellValue::number(i as f64),
                CellValue::number(pressure_head),
                CellValue::number(residual_or_supply),
                CellValue::number(0.0),
            ]);
        }
        let mut out = table_receipt(
            self,
            "urn:qualia:civic:steady-hagen-poiseuille-network",
            &[
                "component",
                "index",
                "pipe_head_drop_or_node_pressure_head_m",
                "pipe_flow_or_node_supply_residual_m3_s",
                "pipe_reynolds",
            ],
            &rows,
        )?;
        out.insert(
            "max_junction_residual_m3_s".into(),
            Value::F64(worst_residual),
        );
        Ok(Value::Record(out))
    }
}
