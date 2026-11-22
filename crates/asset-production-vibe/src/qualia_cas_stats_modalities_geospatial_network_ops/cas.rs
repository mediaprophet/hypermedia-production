//! Bounded CAS authoring and graph round-trips for morphism #138.

use super::{exact_u64, ReceiptGraph};
use crate::{count, fields, list, num, number, required, text, Args, Resource, Value};
use qualia_core_db::{
    q42_volume::{Q42RangeQueryCursor, Q42RangeQueryPattern, Q42RangeQueryPlan,
        WasmSliceQ42Reader, MAX_COMPRESSED_SUPERBLOCK_SIZE, SUPERBLOCK_SIZE},
    q_hash,
    solvers::transforms::{laplace_table, ztransform},
    specialized_libs::{constructibility, multivar_calculus, polynomial_algebra::Polynomial,
        symbolic_algebra::{self as cas, Expr}, symbolic_assumptions as assumptions,
        symbolic_integration as integration, symbolic_limits as limits, symbolic_ode as ode,
        symbolic_series as series, symbolic_solve as solve, symbolic_trig as trig},
    NQuin,
};

const CONTEXT: &str = "urn:qualia:cas:ast:v1";
const AST_NODE: &str = "urn:qualia:cas:has-node";
const NODE_PREFIX: &str = "urn:qualia:cas:quin:v1:";
const MAX_NODES: usize = 256;
const LEX_SCRATCH: usize = 65_536;
pub(super) const READ_SCRATCH: usize = MAX_COMPRESSED_SUPERBLOCK_SIZE + SUPERBLOCK_SIZE
    + LEX_SCRATCH + (MAX_NODES + 1) * 48 * 2;

pub(super) enum Solution {
    Expr(Expr),
    Implicit(Expr, Expr),
    Characteristic(Expr),
    Value(Value),
}

pub(super) fn work(op: &str) -> usize {
    match op {
        "taylor" | "roots" | "resultant" => 262_144,
        "integrate_definite" => 32_768,
        _ => 8_192,
    }
}

fn key(hash: u64) -> String {
    format!("urn:qualia:cas:expression:{hash:016x}")
}

fn parse(value: &Value, bound: usize) -> Result<Expr, String> {
    let source = text(value)?;
    if source.len() > bound {
        return Err("CAS expression exceeds the source limit".into());
    }
    cas::parse(source)
}

fn real(value: f64) -> Result<Value, String> {
    if value.is_finite() { Ok(Value::F64(value)) } else { Err("CAS result is non-finite".into()) }
}

fn variable(value: &str) -> Result<&str, String> {
    if matches!(cas::parse(value), Ok(Expr::Var(name)) if name == value) && value.len() <= 8 {
        Ok(value)
    } else {
        Err("CAS variable must be a single identifier encodable in the quin format".into())
    }
}

fn parameter_variable<'a>(args: &'a Args, name: &str, default: &'static str) -> Result<&'a str, String> {
    variable(args.get(name).map(text).transpose()?.unwrap_or(default))
}

fn ode_result(solution: ode::OdeSolution) -> Solution {
    match solution {
        ode::OdeSolution::Explicit(expr) => Solution::Expr(expr),
        ode::OdeSolution::Implicit { f_y, g_x } => Solution::Implicit(f_y, g_x),
    }
}

pub(super) fn evaluate(op: &str, parameters: &Args) -> Result<Solution, String> {
    let expression = || parse(required(parameters, "expr")?, 512);
    let read_number = |name| num(required(parameters, name)?);
    let var = |name, default| parameter_variable(parameters, name, default);
    let coefficients = |name| -> Result<Vec<f64>, String> {
        let values = list(required(parameters, name)?)?;
        if !(1..=64).contains(&values.len()) {
            return Err("CAS coefficient vectors must contain 1–64 numbers".into());
        }
        values.iter().map(num).collect()
    };
    Ok(match op {
        "to_quins" | "simplify_trig" => {
            fields(parameters, &["expr"])?;
            let input = expression()?;
            Solution::Expr(if op == "to_quins" { input } else { trig::simplify_trig(&input) })
        }
        "differentiate" | "integrate" => {
            fields(parameters, &["expr", "variable"])?;
            let input = expression()?;
            let x = var("variable", "x")?;
            if op == "differentiate" {
                Solution::Expr(cas::simplify(&multivar_calculus::partial(&input, x)))
            } else {
                Solution::Expr(integration::integrate(&input, x).map_err(|e| format!("{e:?}"))?)
            }
        }
        "simplify" => {
            fields(parameters, &["expr", "assumptions"])?;
            let input = expression()?;
            let mut spec = assumptions::Assumptions::new();
            if let Some(signs) = parameters.get("assumptions") {
                let signs = crate::record(signs)?;
                if signs.len() > 64 { return Err("Too many CAS assumptions".into()); }
                for (name, assumption) in signs {
                    use assumptions::Sign;
                    let sign = match text(assumption)? {
                        "positive" => Sign::Positive,
                        "nonnegative" => Sign::NonNegative,
                        "negative" => Sign::Negative,
                        "nonpositive" => Sign::NonPositive,
                        "nonzero" => Sign::Nonzero,
                        _ => return Err(format!("Unknown CAS assumption for {name}")),
                    };
                    spec = spec.assume(variable(name)?, sign);
                }
            }
            Solution::Expr(assumptions::simplify_with_assumptions(&input, &spec))
        }
        "integrate_definite" => {
            fields(parameters, &["expr", "variable", "lower", "upper", "steps"])?;
            let result = integration::integrate_definite(&expression()?, var("variable", "x")?,
                read_number("lower")?, read_number("upper")?, count(parameters, "steps", 512, 4096)?)
                .ok_or("Integrand is undefined over the requested interval")?;
            Solution::Value(real(result)?)
        }
        "limit" | "limit_at_infinity" => {
            let input = expression()?;
            let x = var("variable", "x")?;
            let result = if op == "limit" {
                fields(parameters, &["expr", "variable", "point"])?;
                limits::limit(&input, x, read_number("point")?)
            } else {
                fields(parameters, &["expr", "variable"])?;
                limits::limit_at_infinity(&input, x)
            };
            Solution::Value(real(result.ok_or("No finite limit found")?)?)
        }
        "taylor" => {
            fields(parameters, &["expr", "variable", "point", "order"])?;
            let input = parse(required(parameters, "expr")?, 64)?;
            let x = var("variable", "x")?;
            let point = number(parameters, "point", 0.0)?;
            let order = parameters.get("order").map(exact_u64).transpose()?.unwrap_or(3);
            if order > 6 { return Err("CAS Taylor order must be between 0 and 6".into()); }
            let values = series::taylor_coefficients(&input, x, point, order as usize)
                .ok_or("Taylor expansion is undefined at the requested point")?;
            let centered = cas::sub(cas::var(x), cas::c(point));
            let expansion = values.into_iter().rev().try_fold(cas::c(0.0), |sum, term| {
                real(term)?;
                Ok::<_, String>(cas::add(cas::c(term), cas::mul(centered.clone(), sum)))
            })?;
            Solution::Expr(cas::simplify(&expansion))
        }
        "factor_quadratic" => {
            fields(parameters, &["a", "b", "c", "variable"])?;
            Solution::Expr(cas::factor_quadratic(read_number("a")?, read_number("b")?,
                read_number("c")?, var("variable", "x")?).ok_or("No real quadratic factorization")?)
        }
        "roots" => {
            fields(parameters, &["coefficients"])?;
            let roots = solve::roots(&coefficients("coefficients")?).map_err(|e| format!("{e:?}"))?;
            Solution::Value(Value::List(roots.into_iter().map(|root| {
                Ok(Value::List(vec![real(root.re)?, real(root.im)?]))
            }).collect::<Result<_, String>>()?))
        }
        "resultant" => {
            fields(parameters, &["left", "right"])?;
            let left = Polynomial::new(coefficients("left")?);
            let right = Polynomial::new(coefficients("right")?);
            Solution::Value(real(left.resultant(&right))?)
        }
        "ode_linear_first_order" => {
            fields(parameters, &["a", "b", "variable"])?;
            ode_result(ode::solve_linear_first_order(read_number("a")?, read_number("b")?, var("variable", "x")?))
        }
        "ode_linear_second_order" => {
            fields(parameters, &["a", "b", "c", "variable"])?;
            ode_result(ode::solve_linear_second_order(read_number("a")?, read_number("b")?,
                read_number("c")?, var("variable", "x")?).map_err(|e| format!("{e:?}"))?)
        }
        "ode_separable" => {
            fields(parameters, &["g", "h", "x", "y"])?;
            let g = parse(required(parameters, "g")?, 512)?;
            let h = parse(required(parameters, "h")?, 512)?;
            ode_result(ode::solve_separable(&g, &h, var("x", "x")?, var("y", "y")?)
                .map_err(|e| format!("{e:?}"))?)
        }
        "pde_linear_first_order" => {
            fields(parameters, &["a", "b", "x", "y"])?;
            let ode::PdeSolution::GeneralFunctionOf { invariant } = ode::solve_first_order_linear_pde(
                read_number("a")?, read_number("b")?, var("x", "x")?, var("y", "y")?,
            ).map_err(|e| format!("{e:?}"))?;
            Solution::Characteristic(invariant)
        }
        "pde_classify_second_order" => {
            fields(parameters, &["a", "b", "c"])?;
            let class = ode::classify_second_order_pde(read_number("a")?, read_number("b")?, read_number("c")?);
            Solution::Value(Value::String(format!("{class:?}")))
        }
        "constructibility" => {
            fields(parameters, &["expr"])?;
            let verdict = constructibility::is_constructible_number(&expression()?);
            Solution::Value(Value::String(format!("{verdict:?}")))
        }
        "laplace" => {
            fields(parameters, &["expr"])?;
            Solution::Expr(laplace_table(&expression()?).map_err(|e| format!("{e:?}"))?)
        }
        "z_transform" => {
            fields(parameters, &["samples", "re", "im"])?;
            let response = ztransform::z_transform_finite(&coefficients("samples")?,
                (read_number("re")?, number(parameters, "im", 0.0)?)).ok_or("Invalid Z transform point")?;
            Solution::Value(Value::List(vec![real(response.0)?, real(response.1)?]))
        }
        _ => return Err(format!("Unsupported CAS operation: {op}")),
    })
}

fn record_expression(graph: &mut ReceiptGraph, expr: &Expr) -> Result<u64, String> {
    let nodes = cas::to_quins(expr);
    if checked_expression(&nodes)? != *expr {
        return Err("CAS expression cannot be represented losslessly with the current quin codec".into());
    }
    let citation = cas::expr_citation_hash(expr);
    let subject = key(citation);
    for node in nodes {
        let object = format!("{NODE_PREFIX}{:016x}{:016x}{:016x}{:016x}{:016x}{:016x}",
            node.subject, node.predicate, node.object, node.context, node.metadata, node.parity);
        graph.fact(&subject, AST_NODE, &object)?;
    }
    Ok(citation)
}

pub(super) fn output(solution: Solution, budget: usize) -> Result<(Args, Option<Vec<u8>>), String> {
    let mut values = Args::new();
    let mut graph = ReceiptGraph::new(CONTEXT);
    let (role, expr) = match solution {
        Solution::Expr(expr) => ("expression", expr),
        Solution::Characteristic(expr) => ("characteristic", expr),
        Solution::Implicit(left, right) => {
            values.insert("right_citation_hash".into(), Value::U64(record_expression(&mut graph, &right)?));
            values.insert("right_expression".into(), Value::String(right.to_string()));
            ("left_expression", left)
        }
        Solution::Value(value) => {
            values.insert("result".into(), value);
            return Ok((values, None));
        }
    };
    values.insert("citation_hash".into(), Value::U64(record_expression(&mut graph, &expr)?));
    values.insert(role.into(), Value::String(expr.to_string()));
    Ok((values, Some(graph.seal(budget)?)))
}

pub(super) fn load(resource: &Resource, citation: u64) -> Result<Solution, String> {
    if resource.kind != "q42" { return Err("CAS decode requires a Q42 artifact".into()); }
    let reader = WasmSliceQ42Reader::open(&resource.bytes).map_err(|e| e.to_string())?;
    if !reader.is_unified_volume() { return Err("CAS decode requires a Q42 volume".into()); }
    let mut compressed = vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE];
    let mut decoded = vec![0; SUPERBLOCK_SIZE];
    let empty = NQuin { subject: 0, predicate: 0, object: 0, context: 0, metadata: 0, parity: 0 };
    let mut facts = vec![empty; MAX_NODES + 1];
    let plan = Q42RangeQueryPlan::for_pattern(Q42RangeQueryPattern {
        subject: Some(q_hash(&key(citation))), predicate: Some(q_hash(AST_NODE)),
        object: None, context: Some(q_hash(CONTEXT)),
    });
    let page = reader.execute_query_page_into(plan, Q42RangeQueryCursor::default(),
        &mut compressed, &mut decoded, &mut facts).map_err(|e| e.to_string())?;
    if page.next_cursor.is_some() || page.returned == 0 || page.returned > MAX_NODES {
        return Err("CAS AST is missing or exceeds the node limit".into());
    }
    let mut lexicon = vec![0; LEX_SCRATCH];
    let mut spelling = [0; 128];
    let mut nodes = Vec::with_capacity(page.returned);
    for fact in &facts[..page.returned] {
        if !fact.verify_ecc_parity() || fact.metadata != 0 { return Err("Invalid CAS graph fact".into()); }
        let len = reader.lookup_lexicon_hash_into(fact.object, &mut lexicon, &mut spelling)
            .map_err(|e| e.to_string())?.ok_or("CAS AST quin term is missing from Q42LEX")?;
        let term = std::str::from_utf8(spelling.get(..len).ok_or("CAS AST term exceeds lexical limit")?)
            .map_err(|e| e.to_string())?;
        if q_hash(term) != fact.object { return Err("CAS AST quin term hash mismatch".into()); }
        let body = term.strip_prefix(NODE_PREFIX).filter(|s| s.is_ascii() && s.len() == 96)
            .ok_or("Invalid CAS AST quin term")?;
        let mut columns = [0; 6];
        for (index, column) in columns.iter_mut().enumerate() {
            *column = u64::from_str_radix(&body[index * 16..(index + 1) * 16], 16)
                .map_err(|_| "Invalid CAS AST quin word")?;
        }
        let [subject, predicate, object, context, metadata, parity] = columns;
        nodes.push(NQuin { subject, predicate, object, context, metadata, parity });
    }
    nodes.sort_unstable_by_key(|node| node.subject);
    let expression = checked_expression(&nodes)?;
    if cas::to_quins(&expression) != nodes { return Err("Noncanonical CAS AST encoding".into()); }
    if cas::expr_citation_hash(&expression) != citation { return Err("CAS citation hash mismatch".into()); }
    Ok(Solution::Expr(expression))
}

/// Validate an exact post-order tree before entering the recursive core decoder.
fn checked_expression(nodes: &[NQuin]) -> Result<Expr, String> {
    if nodes.is_empty() || nodes.len() > MAX_NODES { return Err("Invalid CAS AST size".into()); }
    let mut parents = [false; MAX_NODES];
    let binary = ["cas:add", "cas:sub", "cas:mul", "cas:div"].map(q_hash);
    let unary = ["cas:pow", "cas:neg", "cas:sqrt", "cas:exp", "cas:ln", "cas:sin", "cas:cos", "cas:tan"].map(q_hash);
    let terminal = ["cas:const", "cas:var", "cas:int_const"].map(q_hash);
    for (index, node) in nodes.iter().enumerate() {
        if node.subject != index as u64 || !node.verify_ecc_parity() {
            return Err("CAS AST is not a parity-checked post-order tree".into());
        }
        let children = [node.object, node.context];
        let count = if binary.contains(&node.predicate) { 2 } else if unary.contains(&node.predicate) { 1 }
            else if terminal.contains(&node.predicate) { 0 } else { return Err("Unknown CAS AST node".into()); };
        for &child in &children[..count] {
            let child = usize::try_from(child).map_err(|_| "Invalid CAS AST child")?;
            if child >= index || parents[child] { return Err("CAS AST has an invalid or shared child".into()); }
            parents[child] = true;
        }
        if node.predicate == terminal[0] && !f64::from_bits(node.object).is_finite() {
            return Err("CAS AST contains a non-finite constant".into());
        }
    }
    if !parents[..nodes.len() - 1].iter().all(|parent| *parent) { return Err("CAS AST contains an orphan node".into()); }
    cas::from_quins(nodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expression_roundtrips_through_q42_and_can_be_cited() {
        let expr = cas::parse("x^3+2*x").unwrap();
        let (values, model) = output(Solution::Expr(expr.clone()), 2 * 1024 * 1024).unwrap();
        let citation = exact_u64(&values["citation_hash"]).unwrap();
        let model = Resource { kind: "q42".into(), key: "cas.q42".into(), bytes: model.unwrap() };
        let Solution::Expr(decoded) = load(&model, citation).unwrap() else { panic!("missing expression") };
        assert_eq!(expr, decoded);
        assert!(load(&model, citation.wrapping_add(1)).is_err());
    }

    #[test]
    fn a_shared_child_cannot_amplify_a_malformed_ast() {
        let mut nodes = cas::to_quins(&cas::parse("x+x").unwrap());
        let node = &mut nodes[2];
        node.context = node.object;
        node.parity = NQuin::calculate_parity(node.subject, node.predicate, node.object, node.context, node.metadata);
        assert!(checked_expression(&nodes).is_err());
    }
}
