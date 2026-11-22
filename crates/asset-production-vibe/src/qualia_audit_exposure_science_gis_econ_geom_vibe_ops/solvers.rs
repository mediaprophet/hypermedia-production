//! Bounded native numerical and graph solver adapters.

use crate::{
    count, fields, integer, list, num, required, text, Args, AssetHost, TypedArtifact, Value,
};
use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::solvers::{
    calculus::{
        tensor_integrity::{commit_state, integrity_root},
        tensor_provenance::{TensorProvenance, TensorState},
    },
    graph_opt::spreading_activation::{spreading_activation, top_k, Edge},
    learning::{
        active::committee::{average_kl_disagreement, consensus},
        dimensionality::som::Som,
    },
    ontology_align::{
        alignment_guard::{
            check_contest_budget, detect_poisoned_alignment_flood, AlignmentSecurityError,
            ContestRecord,
        },
        correspondence::{Alignment, Correspondence},
    },
    special_functions::{
        airy::{airy_ai, airy_bi},
        bessel::{bessel_i, bessel_j, bessel_k, bessel_y},
        orthogonal::{chebyshev_t, chebyshev_u, hermite, laguerre, legendre},
        zeta::zeta,
    },
    statistics::{
        anomaly::grubbs_test,
        regression::{arch_lm, chow_test, durbin_watson, jarque_bera, ramsey_reset, vif_columns},
    },
};
use qualia_core_db::{bundle::semantic_model::SemanticModelBuilder, NQuin};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, sync::Arc};

const MAX_NUMBERS: usize = 65_536;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    let result = match op {
        "spreading_activation" => activation(host, params),
        "committee_disagreement" => committee(host, params),
        "kohonen_som" => som(host, params),
        "regression_diagnostic" => regression(host, params),
        "special_function" => special(host, params),
        "tensor_commitment" => commitment(host, params),
        "alignment_guard" => alignment_guard(host, params),
        _ => Err(format!("Unsupported numerical solver operation: {op}")),
    }?;
    emit_result(host, op, result)
}

fn emit_result(host: &mut AssetHost<'_>, op: &str, result: Value) -> Result<Value, String> {
    let mut book = Workbook::new();
    let sheet = book.sheet_mut(0).expect("new workbook has one sheet");
    sheet.name = "Solver Result".into();
    sheet.set_value(CellRef::new(0, 0), CellValue::text("Metric"));
    sheet.set_value(CellRef::new(0, 1), CellValue::text("Value"));
    let mut row = 1;
    flatten(&result, "result", sheet, &mut row)?;
    let mut canon = Vec::new();
    ciborium::into_writer(&book, &mut canon).map_err(|error| error.to_string())?;
    host.charge(row as usize, canon.len())?;
    let payload_iri = format!("urn:sha256:{:x}", Sha256::digest(&canon));
    let operation_iri = format!("urn:qualia:solver:{op}");
    let mut graph = SemanticModelBuilder::new(host.limits.resource_bytes.min(42 * 1024 * 1024));
    let subject = graph
        .bind_iri(&payload_iri)
        .map_err(|error| error.to_string())?;
    let predicate = graph
        .bind_iri("https://qualiadb.org/ontology/q42#solverOperation")
        .map_err(|error| error.to_string())?;
    let object = graph
        .bind_iri(&operation_iri)
        .map_err(|error| error.to_string())?;
    let context = graph
        .bind_iri("https://qualiadb.org/ontology/q42#SolverReceipt")
        .map_err(|error| error.to_string())?;
    let quin = NQuin {
        subject,
        predicate,
        object,
        context,
        metadata: 0,
        parity: NQuin::calculate_parity(subject, predicate, object, context, 0),
    };
    let receipt = graph
        .seal_projection(std::iter::once(quin))
        .map_err(|error| error.to_string())?;
    Ok(Value::Record(BTreeMap::from([
        ("result".into(), result),
        (
            "cube".into(),
            host.admit_artifact(TypedArtifact::TabularCube(Arc::new(book)))?,
        ),
        ("receipt".into(), host.encode("q42", receipt)?),
    ])))
}

fn flatten(
    value: &Value,
    path: &str,
    sheet: &mut chronicle_tabular_engine::Sheet,
    row: &mut u32,
) -> Result<(), String> {
    let cell = match value {
        Value::Record(fields) => {
            for (key, value) in fields {
                flatten(value, &format!("{path}.{key}"), sheet, row)?;
            }
            return Ok(());
        }
        Value::List(items) => {
            for (index, value) in items.iter().enumerate() {
                flatten(value, &format!("{path}[{index}]"), sheet, row)?;
            }
            return Ok(());
        }
        Value::F64(value) => CellValue::Number(*value),
        Value::U64(value) if *value <= (1u64 << 53) => CellValue::Number(*value as f64),
        Value::U64(value) => CellValue::text(value.to_string()),
        Value::String(value) => CellValue::text(value),
        Value::Bool(value) => CellValue::Bool(*value),
        Value::Null => CellValue::Empty,
        _ => return Err("Solver result cannot be represented in a tabular cube".into()),
    };
    if *row > MAX_NUMBERS as u32 * 2 {
        return Err("Solver result exceeds tabular cube row budget".into());
    }
    sheet.set_value(CellRef::new(*row, 0), CellValue::text(path));
    sheet.set_value(CellRef::new(*row, 1), cell);
    *row += 1;
    Ok(())
}

fn numbers(value: &Value) -> Result<Vec<f64>, String> {
    let values = list(value)?;
    if values.is_empty() || values.len() > MAX_NUMBERS {
        return Err("Numerical input must have a bounded positive length".into());
    }
    values.iter().map(num).collect()
}

fn numeric_input(host: &AssetHost<'_>, value: &Value) -> Result<Vec<f64>, String> {
    let Ok(TypedArtifact::TabularCube(book)) = host.workspace.typed(value) else {
        return numbers(value);
    };
    let sheet = book.sheet(0).ok_or("Tabular input has no first sheet")?;
    let range = sheet
        .used_range()
        .ok_or("Tabular numeric input has no cells")?;
    let area = (u64::from(range.end.row) - u64::from(range.start.row) + 1)
        .saturating_mul(u64::from(range.end.col) - u64::from(range.start.col) + 1);
    if area > MAX_NUMBERS as u64 {
        return Err("Tabular numeric input exceeds cell budget".into());
    }
    let mut out = Vec::with_capacity(area as usize);
    for row in range.start.row..=range.end.row {
        for col in range.start.col..=range.end.col {
            match sheet.value(CellRef::new(row, col)) {
                CellValue::Number(value) if value.is_finite() => out.push(value),
                _ => {
                    return Err(
                        "Tabular numeric input requires finite values in every used cell".into(),
                    )
                }
            }
        }
    }
    Ok(out)
}

fn finite(value: f64) -> Result<Value, String> {
    if value.is_finite() {
        Ok(Value::F64(value))
    } else {
        Err("Numerical solver produced a non-finite result".into())
    }
}

fn numeric_list(values: Vec<f64>) -> Result<Value, String> {
    values
        .into_iter()
        .map(finite)
        .collect::<Result<Vec<_>, _>>()
        .map(Value::List)
}

fn activation(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "nodes",
            "edges",
            "seeds",
            "decay",
            "threshold",
            "hops",
            "top",
        ],
    )?;
    let nodes = count(params, "nodes", 1, 8192)?;
    let hops = count(params, "hops", 8, 128)?;
    let top = count(params, "top", nodes, nodes)?;
    let decay = num(required(params, "decay")?)?;
    let threshold = num(required(params, "threshold")?)?;
    if !(0.0 < decay && decay <= 1.0) || threshold < 0. {
        return Err("Activation requires a decay in (0,1] and a nonnegative threshold".into());
    }
    let edge_rows = list(required(params, "edges")?)?;
    let seed_rows = list(required(params, "seeds")?)?;
    if edge_rows.len() > MAX_NUMBERS || seed_rows.is_empty() || seed_rows.len() > nodes {
        return Err("Activation edge or seed count exceeds budget".into());
    }
    let vertex = |value: &Value| -> Result<usize, String> {
        let index = integer(value, "vertex")?;
        if index >= nodes {
            Err("Activation edge or seed refers to an unknown vertex".into())
        } else {
            Ok(index)
        }
    };
    let edges = edge_rows
        .iter()
        .map(|row| {
            let [from, to, weight] = list(row)? else {
                return Err("Activation edge requires [from, to, weight]".into());
            };
            let weight = num(weight)?;
            if weight < 0. {
                return Err("Activation weight cannot be negative".into());
            }
            Ok(Edge {
                from: vertex(from)?,
                to: vertex(to)?,
                weight,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let seeds = seed_rows
        .iter()
        .map(|row| {
            let [index, mass] = list(row)? else {
                return Err("Activation seed requires [vertex, mass]".into());
            };
            let mass = num(mass)?;
            if mass < 0. {
                return Err("Activation mass cannot be negative".into());
            }
            Ok((vertex(index)?, mass))
        })
        .collect::<Result<Vec<_>, String>>()?;
    host.charge(
        hops.saturating_mul(nodes + edges.len()),
        nodes * 128 + edges.len() * 48,
    )?;
    let scores = spreading_activation(nodes, &edges, &seeds, decay, threshold, hops);
    let top = top_k(&scores, top);
    Ok(Value::Record(BTreeMap::from([
        ("scores".into(), numeric_list(scores)?),
        (
            "top".into(),
            Value::List(
                top.into_iter()
                    .map(|index| Value::U64(index as u64))
                    .collect(),
            ),
        ),
    ])))
}

fn committee(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["members"])?;
    let members = list(required(params, "members")?)?;
    if !(2..=128).contains(&members.len()) {
        return Err("Committee needs two to 128 prediction distributions".into());
    }
    let mut cells = 0;
    let distributions = members
        .iter()
        .map(|row| {
            let values = numbers(row)?;
            cells += values.len();
            if values.len() > 1024
                || cells > MAX_NUMBERS
                || values.iter().any(|p| !(0.0..=1.0).contains(p))
                || (values.iter().sum::<f64>() - 1.).abs() > 1.0e-9
            {
                return Err("Committee member probabilities must be bounded and sum to one".into());
            }
            Ok(values)
        })
        .collect::<Result<Vec<_>, String>>()?;
    host.charge(cells, cells * 8)?;
    let disagreement = average_kl_disagreement(&distributions)
        .map_err(|error| format!("Committee KL calculation: {error:?}"))?;
    let average =
        consensus(&distributions).map_err(|error| format!("Committee mean: {error:?}"))?;
    Ok(Value::Record(BTreeMap::from([
        ("average_kl".into(), finite(disagreement)?),
        ("consensus".into(), numeric_list(average)?),
    ])))
}

fn som(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "data",
            "dimensions",
            "width",
            "height",
            "epochs",
            "learning_rate",
            "radius",
            "seed",
        ],
    )?;
    let data = numeric_input(host, required(params, "data")?)?;
    let dim = count(params, "dimensions", 2, 256)?;
    let w = count(params, "width", 4, 64)?;
    let h = count(params, "height", 4, 64)?;
    let epochs = count(params, "epochs", 1, 1024)?;
    let rate = num(required(params, "learning_rate")?)?;
    let radius = num(required(params, "radius")?)?;
    let seed = integer(required(params, "seed")?, "seed")? as u64;
    if data.len() % dim != 0
        || !(0. < rate && rate <= 1.)
        || radius <= 0.
        || data.iter().any(|value| value.abs() > 1.0e100)
    {
        return Err("SOM dimensions, numeric scale or learning parameters are invalid".into());
    }
    let cells = w * h;
    let work = data
        .len()
        .saturating_mul(cells)
        .saturating_mul(epochs + 1)
        .saturating_mul(2);
    host.charge(work, (data.len() + cells * dim) * 8)?;
    let model = Som::train(
        &data,
        data.len() / dim,
        dim,
        w,
        h,
        epochs,
        rate,
        radius,
        seed,
    )
    .map_err(|error| format!("SOM training: {error:?}"))?;
    let assignments = data
        .chunks_exact(dim)
        .map(|sample| {
            let (x, y) = model.bmu(sample);
            Value::List(vec![Value::U64(x as u64), Value::U64(y as u64)])
        })
        .collect();
    Ok(Value::Record(BTreeMap::from([(
        "assignments".into(),
        Value::List(assignments),
    )])))
}

fn report(value: serde_json::Value) -> Result<Value, String> {
    use serde_json::Value as Json;
    match value {
        Json::Object(values) => values
            .into_iter()
            .map(|(key, value)| Ok((key, report(value)?)))
            .collect::<Result<BTreeMap<_, _>, String>>()
            .map(Value::Record),
        Json::Array(values) => values
            .into_iter()
            .map(report)
            .collect::<Result<Vec<_>, _>>()
            .map(Value::List),
        Json::Number(value) => {
            if let Some(index) = value.as_u64() {
                Ok(Value::U64(index))
            } else {
                finite(
                    value
                        .as_f64()
                        .ok_or("Regression number is outside f64 range")?,
                )
            }
        }
        Json::Bool(value) => Ok(Value::Bool(value)),
        _ => Err("Regression diagnostic is undefined for the supplied data".into()),
    }
}

fn regression(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    let name = text(required(params, "test")?)?;
    let values = numeric_input(host, required(params, "values")?)?;
    let json = match name {
        "durbin_watson" | "jarque_bera" | "arch_lm" | "grubbs" => {
            fields(params, &["test", "values", "lag", "alpha"])?;
            let lag = count(params, "lag", 1, 64)?;
            host.charge(
                values.len().saturating_mul(lag * lag),
                values.len() * lag * 16,
            )?;
            match name {
                "durbin_watson" => serde_json::to_value(durbin_watson(&values)),
                "jarque_bera" => serde_json::to_value(jarque_bera(&values)),
                "arch_lm" => serde_json::to_value(arch_lm(&values, lag)),
                _ => {
                    let alpha = num(required(params, "alpha")?)?;
                    if !(0. < alpha && alpha < 1.) {
                        return Err("Grubbs significance must be in (0,1)".into());
                    }
                    let out =
                        grubbs_test(&values, alpha).ok_or("Grubbs diagnostic is undefined")?;
                    Ok(serde_json::json!({
                        "index": out.index, "statistic": out.statistic,
                        "critical": out.critical, "is_outlier": out.is_outlier,
                    }))
                }
            }
        }
        "vif" | "ramsey_reset" | "chow" => {
            fields(
                params,
                &[
                    "test",
                    "values",
                    "predictors",
                    "dimensions",
                    "power",
                    "break_index",
                ],
            )?;
            let dim = count(params, "dimensions", 1, 64)?;
            let predictor = if name == "vif" {
                values.clone()
            } else {
                numeric_input(host, required(params, "predictors")?)?
            };
            if predictor.len() % dim != 0
                || (name != "vif" && predictor.len() / dim != values.len())
            {
                return Err(
                    "Regression predictor dimensions differ from the observation count".into(),
                );
            }
            let n = predictor.len() / dim;
            host.charge(n.saturating_mul(dim * dim * dim), predictor.len() * 24)?;
            match name {
                "vif" => serde_json::to_value(vif_columns(&predictor, n, dim)),
                "ramsey_reset" => {
                    let power = count(params, "power", 3, 8)?;
                    serde_json::to_value(ramsey_reset(&predictor, &values, n, dim, power))
                }
                _ => {
                    let at = integer(required(params, "break_index")?, "break_index")?;
                    if at >= n {
                        return Err("Chow break index lies outside the observations".into());
                    }
                    serde_json::to_value(chow_test(&predictor, &values, n, dim, at))
                }
            }
        }
        _ => return Err("Unknown regression diagnostic".into()),
    }
    .map_err(|error| error.to_string())?;
    report(json)
}

fn special(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["function", "x", "order"])?;
    let x = num(required(params, "x")?)?;
    let n = u32::try_from(
        params
            .get("order")
            .map(|value| integer(value, "order"))
            .transpose()?
            .unwrap_or(0),
    )
    .ok()
    .filter(|&order| order <= 4096)
    .ok_or("Special-function order exceeds production bounds")?;
    let name = text(required(params, "function")?)?;
    host.charge(16_384 + n as usize, 0)?;
    let result = match name {
        "bessel_j" => Some(bessel_j(n as i32, x)),
        "bessel_i" => Some(bessel_i(n as i32, x)),
        "bessel_y" => bessel_y(n, x),
        "bessel_k" => bessel_k(n, x),
        "airy_ai" => Some(airy_ai(x)),
        "airy_bi" => Some(airy_bi(x)),
        "legendre" => Some(legendre(n, x)),
        "laguerre" => Some(laguerre(n, x)),
        "chebyshev_t" => Some(chebyshev_t(n, x)),
        "chebyshev_u" => Some(chebyshev_u(n, x)),
        "hermite" => Some(hermite(n, x)),
        "zeta" => zeta(x),
        _ => return Err("Unknown special function".into()),
    };
    finite(result.ok_or("Special function is outside its supported domain")?)
}

fn commitment(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["source", "shape", "data"])?;
    let data = numeric_input(host, required(params, "data")?)?;
    let shape = list(required(params, "shape")?)?;
    if shape.is_empty() || shape.len() > 16 {
        return Err("Tensor shape rank is outside its bounded domain".into());
    }
    let shape = shape
        .iter()
        .map(|size| integer(size, "axis"))
        .collect::<Result<Vec<_>, _>>()?;
    let size = shape
        .iter()
        .try_fold(1usize, |total, &axis| total.checked_mul(axis))
        .ok_or("Tensor shape overflows")?;
    if size != data.len() {
        return Err("Tensor data length differs from the declared shape".into());
    }
    let source = text(required(params, "source")?)?;
    if source.is_empty() || source.len() > 2048 {
        return Err("Tensor commitment requires a bounded source identity".into());
    }
    host.charge(data.len(), data.len() * 8)?;
    let state = TensorState {
        state_id: qualia_core_db::q_hash(source),
        provenance: TensorProvenance::Root {
            source: format!("{}:{source}:shape={shape:?}", source.len()),
            timestamp: 0,
        },
        shape,
        data,
    };
    let commit = commit_state(&state, None);
    let root = integrity_root(&[commit]);
    let format = |digest: [u8; 32]| Value::String(blake3::Hash::from(digest).to_hex().to_string());
    Ok(Value::Record(BTreeMap::from([
        ("commitment".into(), format(commit.0)),
        ("root".into(), format(root.0)),
    ])))
}

fn alignment_guard(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "matches",
            "recent_proposals",
            "proposal_limit",
            "min_mean_quality",
            "actor",
            "recent_contests",
            "now_unix",
            "window_seconds",
            "contest_limit",
        ],
    )?;
    let matches = list(required(params, "matches")?)?;
    let contests = list(required(params, "recent_contests")?)?;
    if matches.len() > 4096 || contests.len() > 4096 {
        return Err("Alignment guard inputs exceed history and proposal budgets".into());
    }
    let u64_value = |value: &Value| -> Result<u64, String> {
        match value {
            Value::U64(value) => Ok(*value),
            Value::I64(value) => u64::try_from(*value).map_err(|_| "Expected unsigned ID".into()),
            _ => Err("Expected unsigned ID".into()),
        }
    };
    let now = u32::try_from(u64_value(required(params, "now_unix")?)?)
        .map_err(|_| "Alignment guard Unix timestamp exceeds u32")?;
    let mut unique = std::collections::BTreeSet::new();
    let mut quality = 0.;
    let correspondences = matches
        .iter()
        .map(|row| {
            let [from, to, degree] = list(row)? else {
                return Err("Alignment match requires [from, to, degree]".into());
            };
            let from = integer(from, "source")?;
            let to = integer(to, "target")?;
            let degree = num(degree)?;
            if from == 0 || to == 0 || !(0.0..=1.0).contains(&degree) || !unique.insert((from, to))
            {
                return Err(
                    "Alignment matches must be unique, nonzero, and quality-bounded".into(),
                );
            }
            quality += degree;
            Ok(Correspondence::propose(from, to, degree))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let recent = integer(required(params, "recent_proposals")?, "recent_proposals")?;
    let proposal_limit = count(params, "proposal_limit", 1024, 1_000_000)?;
    if recent.checked_add(matches.len()).is_none() {
        return Err("Alignment history count overflows".into());
    }
    let min = num(required(params, "min_mean_quality")?)?;
    if !(0.0..=1.0).contains(&min) {
        return Err("Alignment minimum mean quality must be in [0,1]".into());
    }
    let actor = u64_value(required(params, "actor")?)?;
    if actor == 0 {
        return Err("Contest actor must have a nonzero identifier".into());
    }
    let window = count(params, "window_seconds", 3600, 31_536_000)? as u32;
    let limit = count(params, "contest_limit", 4, 4096)?;
    let contests = contests
        .iter()
        .map(|row| {
            let [who, target, timestamp, grounds] = list(row)? else {
                return Err("Contest row requires [actor, assertion, timestamp, grounds]".into());
            };
            let time = u32::try_from(u64_value(timestamp)?)
                .map_err(|_| "Contest timestamp exceeds u32")?;
            let who = u64_value(who)?;
            let target = u64_value(target)?;
            let grounds = u64_value(grounds)?;
            if time > now || [who, target, grounds].contains(&0) {
                return Err("Contest contains future or unbound evidence".into());
            }
            Ok(ContestRecord {
                contestant_did: who,
                target_assertion: target,
                timestamp_epoch: time,
                grounds_hash: grounds,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    host.charge(
        matches.len().saturating_add(contests.len()),
        matches.len() * 128 + contests.len() * 32,
    )?;
    let alignment = Alignment {
        correspondences,
        quality,
    };
    let flood = detect_poisoned_alignment_flood(&alignment, recent, proposal_limit, min);
    let contest = check_contest_budget(actor, &contests, now, window, limit);
    let blocked = |result: Result<(), AlignmentSecurityError>| {
        result
            .err()
            .map_or(Value::Null, |error| Value::String(format!("{error:?}")))
    };
    Ok(Value::Record(BTreeMap::from([
        (
            "allowed".into(),
            Value::Bool(flood.is_ok() && contest.is_ok()),
        ),
        ("g30".into(), blocked(flood)),
        ("g31".into(), blocked(contest)),
    ])))
}
