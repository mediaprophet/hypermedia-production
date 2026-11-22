//! Bounded inverse fitting of scalar observations with the native reverse tape.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use qualia_core_db::solvers::calculus::reverse_tape::{ReverseNode, ReverseTapeError, with_reverse_tape};
use sha2::{Digest, Sha256};

use crate::{
    Args, AssetHost, Value, count, fields, integer, list, num, number,
    qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, record, required, text,
};

const MAX_PARAMETERS: usize = 32;
const MAX_OPERATIONS: usize = 256;
const MAX_OBSERVATIONS: usize = 64;
const MAX_EVALUATIONS: usize = 1024;
const MAX_TAPE: usize = 1 + MAX_PARAMETERS + MAX_OPERATIONS + 4 * MAX_OBSERVATIONS;

#[derive(Clone, Copy)]
enum Expression {
    Constant(f64),
    Add(usize, usize),
    Subtract(usize, usize),
    Multiply(usize, usize),
    Divide(usize, usize),
    Sin(usize),
    Cos(usize),
    Exp(usize),
    Log(usize),
    Sqrt(usize),
}

#[derive(Clone, Copy)]
struct Parameter {
    lower: f64,
    upper: f64,
    initial: f64,
}

#[derive(Clone, Copy)]
struct Observation {
    slot: usize,
    target: f64,
}

struct Evaluation {
    loss: f64,
    gradient: [f64; MAX_PARAMETERS],
    predictions: [f64; MAX_OBSERVATIONS],
}

impl AssetHost<'_> {
    pub(crate) fn solve_adjoint_inverse_morphism(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["parameters", "expression", "observations", "max_evaluations", "gradient_tolerance"])?;
        let parameters = list(required(args, "parameters")?)?;
        let expression = list(required(args, "expression")?)?;
        let observations = list(required(args, "observations")?)?;
        if !(1..=MAX_PARAMETERS).contains(&parameters.len())
            || expression.len() > MAX_OPERATIONS
            || !(1..=MAX_OBSERVATIONS).contains(&observations.len())
        {
            return Err("Inverse expression exceeds bounded parameter, operation, or observation limits".into());
        }
        let parameters = parameters.iter().map(parse_parameter).collect::<Result<Vec<_>, _>>()?;
        let expression = expression.iter().enumerate()
            .map(|(index, node)| parse_expression(node, parameters.len() + index))
            .collect::<Result<Vec<_>, _>>()?;
        let observations = observations.iter()
            .map(|value| parse_observation(value, parameters.len() + expression.len()))
            .collect::<Result<Vec<_>, _>>()?;
        let budget = count(args, "max_evaluations", 128, MAX_EVALUATIONS)?;
        let tolerance = number(args, "gradient_tolerance", 1.0e-8)?;
        if !(1.0e-14..=1.0).contains(&tolerance) {
            return Err("gradient_tolerance outside 1e-14..1".into());
        }
        let mut tape = [ReverseNode::EMPTY; MAX_TAPE];
        self.charge(0, std::mem::size_of_val(&tape))?;
        let mut point = [0.0; MAX_PARAMETERS];
        for (slot, parameter) in point.iter_mut().zip(&parameters) {
            *slot = parameter.initial;
        }
        let mut evaluation = evaluate(&mut tape, &point[..parameters.len()], &expression, &observations)
            .map_err(|error| format!("Invalid initial inverse expression: {error:?}"))?;
        let initial_loss = evaluation.loss;
        let mut evaluations = 1;
        let work = parameters.len() + expression.len() + observations.len() * 4;
        self.charge(work * 2, 0)?;
        let mut step = 1.0_f64;
        let status = loop {
            let residual = projected_gradient(&point, &evaluation, &parameters);
            if residual <= tolerance {
                break "projected_stationary";
            }
            if evaluations >= budget {
                break "evaluation_budget";
            }
            let mut trial = point;
            let mut slope = 0.0;
            for (index, parameter) in parameters.iter().enumerate() {
                trial[index] = (point[index] - step * evaluation.gradient[index]).clamp(parameter.lower, parameter.upper);
                slope += evaluation.gradient[index] * (trial[index] - point[index]);
            }
            if !slope.is_finite() {
                step *= 0.5;
            } else if slope >= 0.0 {
                break "line_search_stalled";
            } else {
                self.charge(work * 2, 0)?;
                evaluations += 1;
                match evaluate(&mut tape, &trial[..parameters.len()], &expression, &observations) {
                    Ok(next) if next.loss < evaluation.loss && next.loss <= evaluation.loss + 1.0e-4 * slope => {
                        point = trial;
                        evaluation = next;
                        step = (step * 2.0).min(16.0);
                        continue;
                    }
                    Ok(_) | Err(ReverseTapeError::InvalidDomain | ReverseTapeError::NonFinite) => step *= 0.5,
                    Err(error) => return Err(format!("Reverse tape failed: {error:?}")),
                }
            }
            if step < 1.0e-14 {
                break "line_search_stalled";
            }
        };
        let projected = projected_gradient(&point, &evaluation, &parameters);
        let mut evidence = ReceiptGraph::new("urn:qualia:fit:scalar-adjoint");
        let mut digest = Sha256::new();
        for value in [initial_loss, evaluation.loss, projected].into_iter()
            .chain(point[..parameters.len()].iter().copied())
            .chain(evaluation.predictions[..observations.len()].iter().copied())
        {
            digest.update(value.to_bits().to_le_bytes());
        }
        digest.update(evaluations.to_le_bytes());
        digest.update(status.as_bytes());
        let fit = format!("urn:qualia:fit:sha256:{:x}", digest.finalize());
        evidence.fact(&fit, "urn:qualia:fit:method", "urn:qualia:fit:projected-reverse-gradient")?;
        evidence.fact(&fit, "urn:qualia:fit:termination", &format!("urn:qualia:fit:{status}"))?;
        for (property, value) in [("initial_loss", initial_loss), ("loss", evaluation.loss), ("projected_gradient", projected)] {
            evidence.fact(&fit, &format!("urn:qualia:fit:{property}"), &format!("urn:qualia:float64:{:016x}", value.to_bits()))?;
        }
        let receipt = self.encode("q42", evidence.seal(self.limits.resource_bytes)?)?;
        Ok(Value::Record(Args::from([
            ("parameters".into(), numbers(&point[..parameters.len()])),
            ("observations".into(), numbers(&evaluation.predictions[..observations.len()])),
            ("initial_loss".into(), Value::F64(initial_loss)),
            ("loss".into(), Value::F64(evaluation.loss)),
            ("projected_gradient".into(), Value::F64(projected)),
            ("evaluations".into(), Value::U64(evaluations as u64)),
            ("termination".into(), Value::String(status.into())),
            ("receipt".into(), receipt),
        ])))
    }
}

fn parse_parameter(value: &Value) -> Result<Parameter, String> {
    let value = record(value)?;
    fields(value, &["lower", "upper", "initial"])?;
    let lower = num(required(value, "lower")?)?;
    let upper = num(required(value, "upper")?)?;
    let initial = num(required(value, "initial")?)?;
    if lower >= upper || initial < lower || initial > upper || lower < -1.0e12 || upper > 1.0e12 {
        return Err("Invalid inverse parameter bounds".into());
    }
    Ok(Parameter { lower, upper, initial })
}

fn parse_expression(value: &Value, slots: usize) -> Result<Expression, String> {
    let value = record(value)?;
    let operand = |name| -> Result<usize, String> {
        let index = integer(required(value, name)?, name)?;
        (index < slots).then_some(index).ok_or_else(|| "Expression operand must refer to an earlier slot".into())
    };
    let operation = text(required(value, "op")?)?;
    Ok(match operation {
        "constant" => {
            fields(value, &["op", "value"])?;
            Expression::Constant(num(required(value, "value")?)?)
        }
        "add" | "subtract" | "multiply" | "divide" => {
            fields(value, &["op", "a", "b"])?;
            let a = operand("a")?;
            let b = operand("b")?;
            match operation {
                "add" => Expression::Add(a, b),
                "subtract" => Expression::Subtract(a, b),
                "multiply" => Expression::Multiply(a, b),
                _ => Expression::Divide(a, b),
            }
        }
        "sin" | "cos" | "exp" | "log" | "sqrt" => {
            fields(value, &["op", "a"])?;
            let a = operand("a")?;
            match operation {
                "sin" => Expression::Sin(a),
                "cos" => Expression::Cos(a),
                "exp" => Expression::Exp(a),
                "log" => Expression::Log(a),
                _ => Expression::Sqrt(a),
            }
        }
        _ => return Err("Unsupported inverse expression operator".into()),
    })
}

fn parse_observation(value: &Value, slots: usize) -> Result<Observation, String> {
    let value = record(value)?;
    fields(value, &["slot", "target"])?;
    let slot = integer(required(value, "slot")?, "observation slot")?;
    if slot >= slots {
        return Err("Observation slot outside the expression".into());
    }
    Ok(Observation { slot, target: num(required(value, "target")?)? })
}

fn evaluate(
    storage: &mut [ReverseNode],
    point: &[f64],
    expression: &[Expression],
    observations: &[Observation],
) -> Result<Evaluation, ReverseTapeError> {
    with_reverse_tape(storage, |tape| {
        let zero = tape.input(0.0)?;
        let mut slots = [zero; MAX_PARAMETERS + MAX_OPERATIONS];
        for (slot, value) in slots.iter_mut().zip(point) {
            *slot = tape.input(*value)?;
        }
        for (index, node) in expression.iter().enumerate() {
            let value = match *node {
                Expression::Constant(value) => tape.input(value),
                Expression::Add(a, b) => tape.add(slots[a], slots[b]),
                Expression::Subtract(a, b) => tape.sub(slots[a], slots[b]),
                Expression::Multiply(a, b) => tape.mul(slots[a], slots[b]),
                Expression::Divide(a, b) => tape.div(slots[a], slots[b]),
                Expression::Sin(a) => tape.sin(slots[a]),
                Expression::Cos(a) => tape.cos(slots[a]),
                Expression::Exp(a) => tape.exp(slots[a]),
                Expression::Log(a) => tape.ln(slots[a]),
                Expression::Sqrt(a) => tape.sqrt(slots[a]),
            }?;
            slots[point.len() + index] = value;
        }
        let mut loss = zero;
        let mut predictions = [0.0; MAX_OBSERVATIONS];
        for (value, observation) in predictions.iter_mut().zip(observations) {
            let prediction = slots[observation.slot];
            *value = tape.value(prediction)?;
            let target = tape.input(observation.target)?;
            let residual = tape.sub(prediction, target)?;
            let squared = tape.mul(residual, residual)?;
            loss = tape.add(loss, squared)?;
        }
        let mut gradient = [0.0; MAX_PARAMETERS];
        tape.gradient_into(loss, &slots[..point.len()], &mut gradient[..point.len()])?;
        Ok(Evaluation { loss: tape.value(loss)?, gradient, predictions })
    })?
}

fn projected_gradient(point: &[f64], evaluation: &Evaluation, parameters: &[Parameter]) -> f64 {
    parameters.iter().enumerate().map(|(index, parameter)| {
        let current = point[index];
        let gradient = evaluation.gradient[index];
        if (current == parameter.lower && gradient >= 0.0)
            || (current == parameter.upper && gradient <= 0.0)
        {
            0.0
        } else {
            gradient.abs()
        }
    }).fold(0.0, f64::max)
}

fn numbers(values: &[f64]) -> Value {
    Value::List(values.iter().copied().map(Value::F64).collect())
}
