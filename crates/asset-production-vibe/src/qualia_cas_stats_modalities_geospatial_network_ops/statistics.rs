//! Statistical and econometric tasks over finite records and workbook ranges.

use std::{collections::BTreeMap, sync::Arc};

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::{
    bundle::semantic_model::resource_identity_meta,
    q42_volume::{Q42RangeQueryCursor, Q42RangeQueryPattern, Q42RangeQueryPlan, WasmSliceQ42Reader,
        MAX_COMPRESSED_SUPERBLOCK_SIZE, SUPERBLOCK_SIZE},
    q_hash, query::resolver::{classify_inline_literal, InlineLiteral},
    vibe_host::invoke::production_category_a::evaluate_statistics, NQuin,
};

use super::{external_resource, ReceiptGraph, SYMBOLIC_LIMIT};
use crate::{boolean, fields, integer, record, required, text, Args, AssetHost, TypedArtifact, Value};

const GRAPH: &str = "urn:qualia:statistics:receipt:v1";
const MAX_VALUES: usize = 4_096;
const MAX_DEPTH: usize = 8;

impl AssetHost<'_> {
    pub(crate) fn evaluate_inference_and_econometrics(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["task", "parameters", "table", "ranges", "model", "context_iri", "columns"])?;
        let task = text(required(args, "task")?)?;
        let mut parameters = match args.get("parameters") {
            Some(value) => {
                let mut count = 0;
                visit(value, 0, &mut count, &mut |_, _| Ok(()), "parameters")?;
                record(value)?.clone()
            }
            None => Args::new(),
        };
        if let Some(table) = args.get("table") {
            if args.contains_key("model") { return Err("Supply either a statistics table or model".into()); }
            let TypedArtifact::TabularCube(table) = self.workspace.typed(table)? else {
                return Err("Statistics table requires a TabularCube handle".into());
            };
            let ranges = record(required(args, "ranges")?)?;
            if ranges.is_empty() || ranges.len() > 32 { return Err("Provide 1–32 statistics range bindings".into()); }
            let mut cells = 0;
            for (name, descriptor) in ranges {
                if parameters.contains_key(name) { return Err(format!("Duplicate task field {name}")); }
                parameters.insert(name.clone(), project(table, record(descriptor)?, &mut cells)?);
            }
        } else if args.contains_key("ranges") {
            return Err("Statistics ranges require a table".into());
        }
        if let Some(model) = args.get("model") {
            let columns = record(required(args, "columns")?)?;
            if columns.is_empty() || columns.len() > 32 { return Err("Provide 1–32 statistics predicate bindings".into()); }
            let context = text(required(args, "context_iri")?)?;
            let bytes = external_resource(self, model)?.bytes.len();
            let scratch = 1024 * 1024 + MAX_COMPRESSED_SUPERBLOCK_SIZE + SUPERBLOCK_SIZE
                + (MAX_VALUES + 1) * 128;
            let limit = SYMBOLIC_LIMIT.saturating_sub(scratch);
            if bytes > limit || external_resource(self, model)?.kind != "q42" {
                return Err("Statistics source must be a Q42 volume within the symbolic budget".into());
            }
            self.charge(bytes.saturating_mul(columns.len()), scratch)?;
            let input = external_resource(self, model)?;
            for (name, column) in project_model(&input.bytes, context, columns)? {
                if parameters.insert(name.clone(), column).is_some() { return Err(format!("Duplicate task field {name}")); }
            }
        } else if args.contains_key("columns") || args.contains_key("context_iri") {
            return Err("Statistics columns and context require a model".into());
        }
        validate_parameters(task, &parameters)?;
        let arguments = Value::Record(parameters);
        let mut values = 0;
        visit(&arguments, 0, &mut values, &mut |_, _| Ok(()), "parameters")?;
        let work = values.saturating_mul(values).saturating_mul(4).max(64);
        self.charge(work, MAX_VALUES * 256)?;
        let result = evaluate_statistics(task, &arguments).map_err(|error| error.message)?;

        let args_json = crate::living_world_anatomy::species_spec::vibe_value_to_json(&arguments);
        let canonical = serde_json::to_vec(&(task, args_json)).map_err(|error| error.to_string())?;
        let digest = blake3::hash(&canonical).to_hex();
        let operation = format!("urn:qualia:statistics:run:{digest}");
        let mut graph = ReceiptGraph::new(GRAPH);
        graph.fact(&operation, "urn:qualia:statistics:task", &format!("urn:qualia:statistics:task:{task}"))?;
        let mut rows = 0usize;
        let mut workbook = Workbook::new();
        let sheet = workbook.sheet_mut(0).ok_or("Statistics result workbook has no sheet")?;
        sheet.name = "Results".into();
        sheet.set_value(CellRef::new(0, 0), CellValue::Text("Field".into()));
        sheet.set_value(CellRef::new(0, 1), CellValue::Text("Value".into()));
        let mut output_values = 0;
        visit(&result, 0, &mut output_values, &mut |path, value| {
            rows += 1;
            let subject = format!("{operation}:result:{rows}");
            let name = format!("urn:qualia:statistics:field:{}", escape(path));
            let (cell, iri) = scalar(value)?;
            graph.fact(&operation, "urn:qualia:statistics:has-result", &subject)?;
            graph.fact(&subject, "urn:qualia:statistics:field", &name)?;
            graph.fact(&subject, "urn:qualia:statistics:value", &iri)?;
            sheet.set_value(CellRef::new(rows as u32, 0), CellValue::Text(path.into()));
            sheet.set_value(CellRef::new(rows as u32, 1), cell);
            Ok(())
        }, "result")?;
        if rows == 0 { return Err("Statistics operation produced no values".into()); }
        let receipt = graph.seal(self.limits.resource_bytes.min(SYMBOLIC_LIMIT))?;
        let receipt = self.encode("q42", receipt)?;
        let table = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(workbook)))?;
        Ok(Value::Record(BTreeMap::from([
            ("result".into(), result), ("table".into(), table), ("receipt".into(), receipt),
        ])))
    }
}

fn project_model(bytes: &[u8], context: &str, columns: &Args) -> Result<Args, String> {
    const MAX_TERM: usize = 4096;
    let reader = WasmSliceQ42Reader::open(bytes).map_err(|error| error.to_string())?;
    if !reader.is_unified_volume() { return Err("Statistics requires a unified Q42 volume".into()); }
    let mut lexical = [0; MAX_TERM];
    let mut lex_page = vec![0; 1024 * 1024];
    let mut compressed = vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE];
    let mut decoded = vec![0; SUPERBLOCK_SIZE];
    let mut facts = vec![NQuin::default(); MAX_VALUES + 1];
    let mut total = 0;
    let mut previous = None;
    let mut result = Args::new();
    for (name, column) in columns {
        let predicate = text(column)?;
        for iri in [context, predicate] {
            if iri.len() > MAX_TERM { return Err("Statistics graph IRI exceeds the lexical limit".into()); }
            resource_identity_meta(iri).map_err(|error| error.to_string())?;
            let len = reader.lookup_lexicon_hash_into(q_hash(iri), &mut lex_page, &mut lexical)
                .map_err(|error| error.to_string())?;
            if len.and_then(|len| lexical.get(..len)) != Some(iri.as_bytes()) {
                return Err("Statistics graph predicate or context lacks its canonical lexical binding".into());
            }
        }
        let plan = Q42RangeQueryPlan::for_pattern(Q42RangeQueryPattern {
            predicate: Some(q_hash(predicate)), context: Some(q_hash(context)), ..Default::default()
        });
        let page = reader.execute_query_page_into(plan, Q42RangeQueryCursor::default(),
            &mut compressed, &mut decoded, &mut facts[..(MAX_VALUES - total + 1)])
            .map_err(|error| error.to_string())?;
        total += page.returned;
        if page.returned == 0 || total > MAX_VALUES || page.next_cursor.is_some() {
            return Err("Statistics graph column is empty or exceeds the row limit".into());
        }
        let rows = &mut facts[..page.returned];
        rows.sort_unstable_by_key(|row| row.subject);
        if rows.windows(2).any(|w| w[0].subject == w[1].subject) {
            return Err("Statistics columns require one numeric value per subject".into());
        }
        let subjects: Vec<_> = rows.iter().map(|row| row.subject).collect();
        if previous.as_ref().is_some_and(|last| last != &subjects) {
            return Err("Statistics column subjects are misaligned".into());
        }
        previous = Some(subjects);
        let mut values = Vec::with_capacity(rows.len());
        for row in rows {
            if row.subject == 0 || !row.verify_ecc_parity() { return Err("Invalid statistics graph fact".into()); }
            let spelling = reader.lookup_lexicon_hash_into(row.object, &mut lex_page, &mut lexical)
                .map_err(|error| error.to_string())?;
            let value = if let Some(len) = spelling {
                let term = std::str::from_utf8(lexical.get(..len).ok_or("Oversized statistics graph value")?)
                    .map_err(|error| error.to_string())?;
                let (literal, datatype) = term.strip_prefix('"').and_then(|s| s.split_once("\"^^<"))
                    .ok_or("Statistics graph column requires numeric RDF values")?;
                if !matches!(datatype, "http://www.w3.org/2001/XMLSchema#double>" | "http://www.w3.org/2001/XMLSchema#float>"
                    | "http://www.w3.org/2001/XMLSchema#decimal>" | "http://www.w3.org/2001/XMLSchema#integer>")
                {
                    return Err("Unsupported statistics graph datatype".into());
                }
                literal.parse::<f64>().map_err(|_| "Malformed statistics graph value")?
            } else {
                match classify_inline_literal(row.object) {
                    Some(InlineLiteral::Integer(n)) if n.unsigned_abs() <= (1 << 53) => n as f64,
                    Some(InlineLiteral::Decimal(n)) => n as f64 / 1_000_000.0,
                    Some(InlineLiteral::Float(n)) => f64::from(n),
                    _ => return Err("Statistics graph column requires numeric RDF values".into()),
                }
            };
            if !value.is_finite() { return Err("Non-finite graph column value".into()); }
            values.push(Value::F64(value));
        }
        result.insert(name.clone(), Value::List(values));
    }
    Ok(result)
}

fn validate_parameters(task: &str, args: &Args) -> Result<(), String> {
    let allowed: &[&str] = match task {
        "one_sample_t" => &["values", "mu"],
        "two_sample_t" => &["a", "b", "equal_var"],
        "paired_t" => &["a", "b"],
        "mann_whitney_u" => &["x", "y"],
        "chi_square_gof" => &["observed", "expected"],
        "chi_square_independence" => &["table"],
        "friedman" | "one_way_anova" => &["groups"],
        "fisher_distance" | "kl_bregman_form" => &["p", "q"],
        "ols" => &["x", "y", "n_obs", "n_reg"],
        "wls" => &["x", "y", "weights", "n_obs", "n_reg"],
        "iv_2sls" => &["x_endogenous", "z_instruments", "y", "n_obs", "n_reg", "n_instr"],
        "leontief_inverse" => &["matrix", "n", "max_rounds", "tolerance"],
        "output_multipliers" => &["leontief_inverse", "n"],
        "gini" => &["incomes"],
        "atkinson" => &["incomes", "epsilon"],
        "rawlsian_welfare" => &["utilities"],
        "vcg_payment" => &["valuations"],
        "mixed_nash_2x2" => &["payoff_matrix_a", "payoff_matrix_b"],
        "bellman_update" => &["rewards", "transitions", "discount", "values", "n_states", "n_actions", "state"],
        "social_cost_of_carbon" => &["emissions", "damage_per_ton"],
        "optimal_abatement" => &["baseline_emissions", "abatement_coeff", "damage_coeff"],
        "abatement_net_benefit" => &["baseline_emissions", "actual_emissions", "abatement_coeff", "damage_coeff"],
        _ => return Err("Unknown production statistics task".into()),
    };
    fields(args, allowed)?;
    for (name, limit) in [("n", 64), ("n_obs", 2048), ("n_reg", 32), ("n_instr", 32),
        ("n_states", 64), ("n_actions", 64), ("state", 63), ("max_rounds", 512)] {
        if let Some(value) = args.get(name) {
            if integer(value, name)? > limit { return Err(format!("Statistics bound exceeded for {name}")); }
        }
    }
    Ok(())
}

fn project(table: &Workbook, spec: &Args, total: &mut usize) -> Result<Value, String> {
    fields(spec, &["sheet", "row", "col", "rows", "cols", "flatten"])?;
    let get = |key, default| spec.get(key).map(|v| integer(v, key)).transpose().map(|v| v.unwrap_or(default));
    let sheet = table.sheet(get("sheet", 0)?).ok_or("Unknown statistics input sheet")?;
    let row = get("row", 0)?;
    let col = get("col", 0)?;
    let rows = get("rows", 0)?;
    let cols = get("cols", 1)?;
    let flatten = boolean(spec, "flatten", cols == 1)?;
    let cell_count = rows.checked_mul(cols).filter(|&n| n > 0)
        .ok_or("Statistics input range dimensions must be positive")?;
    *total = total.checked_add(cell_count).filter(|&n| n <= MAX_VALUES)
        .ok_or("Statistics input ranges exceed the cell limit")?;
    let row_end = row.checked_add(rows).filter(|&n| u32::try_from(n).is_ok()).ok_or("Row bounds overflow")?;
    let col_end = col.checked_add(cols).filter(|&n| u32::try_from(n).is_ok()).ok_or("Column bounds overflow")?;
    let mut matrix = Vec::with_capacity(if flatten { cell_count } else { rows });
    for row in row..row_end {
        let mut values = Vec::with_capacity(cols);
        for col in col..col_end {
            let value = sheet.value(CellRef::new(row as u32, col as u32));
            let CellValue::Number(value) = value else { return Err("Statistics ranges require numeric cells".into()); };
            if !value.is_finite() { return Err("Statistics ranges require finite cells".into()); }
            values.push(Value::F64(value));
        }
        if flatten { matrix.extend(values); } else { matrix.push(Value::List(values)); }
    }
    Ok(Value::List(matrix))
}

fn visit(value: &Value, depth: usize, count: &mut usize,
    leaves: &mut impl FnMut(&str, &Value) -> Result<(), String>, path: &str) -> Result<(), String> {
    *count += 1;
    if depth > MAX_DEPTH || *count > MAX_VALUES { return Err("Statistics record exceeds bounds".into()); }
    match value {
        Value::List(items) => {
            for (index, item) in items.iter().enumerate() {
                visit(item, depth + 1, count, leaves, &format!("{path}/{index}"))?;
            }
        }
        Value::Record(record) => {
            for (field, item) in record {
                if field.len() > 128 { return Err("Statistics field is too long".into()); }
                let field = field.replace('~', "~0").replace('/', "~1");
                visit(item, depth + 1, count, leaves, &format!("{path}/{field}"))?;
            }
        }
        Value::String(s) if s.len() > 256 => return Err("Statistics text is too long".into()),
        Value::F64(n) if !n.is_finite() => return Err("Statistics data contains a non-finite value".into()),
        _ => leaves(path, value)?,
    }
    Ok(())
}

fn scalar(value: &Value) -> Result<(CellValue, String), String> {
    let (cell, lexical) = match value {
        Value::F64(n) => (CellValue::Number(*n), format!("double:{:016x}", n.to_bits())),
        Value::U64(n) => (integer_cell(*n as u128, false), format!("unsigned:{n}")),
        Value::I64(n) => (integer_cell(n.unsigned_abs() as u128, *n < 0), format!("integer:{n}")),
        Value::Bool(b) => (CellValue::Bool(*b), format!("boolean:{b}")),
        Value::String(s) => (CellValue::Text(s.clone()), format!("string:{}", escape(s))),
        _ => return Err("Unsupported statistics result type".into()),
    };
    Ok((cell, format!("urn:qualia:statistics:{lexical}")))
}

fn integer_cell(n: u128, negative: bool) -> CellValue {
    if n <= (1 << 53) {
        CellValue::Number(if negative { -(n as f64) } else { n as f64 })
    } else {
        CellValue::Text(format!("{}{n}", if negative { "-" } else { "" }))
    }
}

fn escape(text: &str) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    for byte in text.bytes() { let _ = write!(&mut out, "%{byte:02X}"); }
    out
}
