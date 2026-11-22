//! Native classical annealing of explicitly authored QUBO coefficients.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    fields, list, num, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, record,
    required, text, Args, AssetHost, Value,
};
use qualia_core_db::{
    modalities::logic::qubo as constraint_qubo,
    qubo_compiler::{
        calculate_energy, serialize_matrix, solve_classical, QuboMatrix, MAX_QUBO_VARS,
    },
    NQuin,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const MAX_COUPLERS: usize = MAX_QUBO_VARS * 4;
const MAX_WEIGHT: f64 = 1e6;

impl AssetHost<'_> {
    pub(super) fn compile_qubo_constraint_fixture(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["mode", "constraints"])?;
        let values = list(required(args, "constraints")?)?;
        if !(1..=constraint_qubo::MAX_COUPLERS).contains(&values.len()) {
            return Err("The QUBO fixture requires 1 to 512 constraints".into());
        }
        self.charge(values.len() * 128, 128 * 1024)?;
        let mut quins = values
            .iter()
            .map(constraint_quin)
            .collect::<Result<Vec<_>, _>>()?;
        quins.sort_unstable_by_key(|quin| (quin.subject, quin.predicate, quin.object));
        if quins.windows(2).any(|pair| {
            pair[0].subject == pair[1].subject
                && pair[0].predicate == pair[1].predicate
                && pair[0].object == pair[1].object
        }) {
            return Err("Duplicate QUBO fixture constraint".into());
        }
        let mut compiler_output = constraint_qubo::QuboMatrix::default();
        constraint_qubo::compile_quins_to_qubo(&quins, &mut compiler_output)
            .map_err(|err| format!("Native QUBO constraint compilation failed: {err:?}"))?;
        compiler_output.wipe_index_map();

        let variable_count = usize::from(compiler_output.num_vars);
        let mut matrix = QuboMatrix::new(variable_count);
        for (index, &value) in compiler_output.linear[..variable_count].iter().enumerate() {
            matrix.set_linear(index, f64::from(value));
        }
        for coupling in &compiler_output.couplers[..compiler_output.coupler_count] {
            matrix.set_quadratic(
                usize::from(coupling.var_a),
                usize::from(coupling.var_b),
                f64::from(coupling.weight),
            );
        }
        let encoded = serialize_matrix(&matrix);
        let digest = format!("{:x}", Sha256::digest(&encoded));
        let subject = format!("urn:qualia:offline-qubo-matrix:sha256:{digest}");
        let mut graph = ReceiptGraph::new(&subject);
        graph.fact(
            &subject,
            "urn:qualia:storage:resultKind",
            "urn:qualia:qubo:identity-free-native-fixture-matrix",
        )?;
        let receipt = graph.seal(self.limits.resource_bytes)?;
        Ok(Value::Record(Args::from([
            ("matrix".into(), self.encode("bin", encoded)?),
            ("matrix_sha256".into(), Value::String(digest)),
            ("variable_count".into(), Value::U64(variable_count as u64)),
            (
                "coupler_count".into(),
                Value::U64(compiler_output.coupler_count as u64),
            ),
            ("receipt".into(), self.encode("q42", receipt)?),
        ])))
    }

    pub(super) fn offline_qubo_anneal(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["mode", "linear", "quadratic"])?;
        let linear = list(required(args, "linear")?)?;
        let quadratic = list(required(args, "quadratic")?)?;
        if !(1..=MAX_QUBO_VARS).contains(&linear.len()) || quadratic.len() > MAX_COUPLERS {
            return Err("QUBO exceeds the variable or coupler budget".into());
        }
        let mut matrix = QuboMatrix::new(linear.len());
        for (idx, value) in linear.iter().enumerate() {
            matrix.set_linear(idx, weight(value)?);
        }
        let mut couplers = BTreeMap::new();
        for item in quadratic {
            let row = record(item)?;
            fields(row, &["a", "b", "weight"])?;
            let a = variable(row, "a", linear.len())?;
            let b = variable(row, "b", linear.len())?;
            if a == b {
                return Err("Self couplers belong in the QUBO linear coefficients".into());
            }
            if couplers
                .insert((a.min(b), a.max(b)), weight(required(row, "weight")?)?)
                .is_some()
            {
                return Err("Each unordered QUBO coupler must occur once".into());
            }
        }
        for ((a, b), weight) in couplers {
            matrix.set_quadratic(a, b, weight);
        }
        // The native annealer attempts fewer than 10,000 flips and scans the couplers on each.
        self.charge(10_000 * (1 + quadratic.len()), 128 * 1024)?;
        let mut assignment = vec![0; linear.len()];
        solve_classical(&matrix, &mut assignment);
        let energy = calculate_energy(&matrix, &assignment);
        let terms: Vec<_> = matrix
            .couplers
            .iter()
            .map(|term| json!({ "a": term.var_a, "b": term.var_b, "weight": term.weight }))
            .collect();
        let report = serde_json::to_vec(&json!({
            "schema": "qualia.offline-qubo-anneal/v1",
            "solver": "qualia-core-db-seeded-classical-annealer",
            "linear": matrix.linear,
            "quadratic": terms,
            "assignment": assignment,
            "objective": energy,
            "optimality_proven": false,
        }))
        .map_err(|err| err.to_string())?;
        let digest = format!("{:x}", Sha256::digest(&report));
        let subject = format!("urn:qualia:offline-qubo:sha256:{digest}");
        let mut graph = ReceiptGraph::new(&subject);
        graph.fact(
            &subject,
            "urn:qualia:storage:resultKind",
            "urn:qualia:qubo:classical-annealing-candidate",
        )?;
        let receipt = graph.seal(self.limits.resource_bytes)?;
        Ok(Value::Record(Args::from([
            ("report".into(), self.encode("json", report)?),
            ("receipt".into(), self.encode("q42", receipt)?),
            ("objective".into(), Value::F64(energy)),
            ("variable_count".into(), Value::U64(linear.len() as u64)),
        ])))
    }
}

fn constraint_quin(value: &Value) -> Result<NQuin, String> {
    let row = record(value)?;
    fields(row, &["a", "b", "operation"])?;
    let a = variable(row, "a", constraint_qubo::MAX_QUBO_VARS)?;
    let b = variable(row, "b", constraint_qubo::MAX_QUBO_VARS)?;
    // The native compiler uses small predicate opcodes and equality of opaque local IDs.
    let predicate = match text(required(row, "operation")?)? {
        "obligate" => 0x10,
        "permit" => 0x11,
        "forbid" => 0x12,
        _ => return Err("QUBO fixture operation must be obligate, permit or forbid".into()),
    };
    let subject = a.min(b) as u64 + 1;
    let object = a.max(b) as u64 + 1;
    Ok(NQuin {
        subject,
        predicate,
        object,
        context: 0,
        metadata: 0,
        parity: NQuin::calculate_parity(subject, predicate, object, 0, 0),
    })
}

fn weight(value: &Value) -> Result<f64, String> {
    let value = num(value)?;
    if value.abs() > MAX_WEIGHT {
        return Err("QUBO coefficient magnitude exceeds 1,000,000".into());
    }
    Ok(if value == 0.0 { 0.0 } else { value })
}

fn variable(row: &Args, key: &str, count: usize) -> Result<usize, String> {
    let index = match required(row, key)? {
        Value::U64(n) if *n < count as u64 => *n as usize,
        Value::I64(n) if (0..count as i64).contains(n) => *n as usize,
        _ => {
            return Err(format!(
                "{key} requires an exact zero-based QUBO variable index"
            ))
        }
    };
    Ok(index)
}
