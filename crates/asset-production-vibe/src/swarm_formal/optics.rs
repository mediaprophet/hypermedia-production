//! Reverse-mode composition for finite Euclidean parametric optics.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{
    collections::BTreeMap,
    mem::{size_of, size_of_val},
};

use sha2::{Digest, Sha256};

use crate::{
    fields, integer, list, num, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    record, required, text, Args, AssetHost, Value,
};

use super::budget;

const MAX_STAGES: usize = 256;
const MAX_WIDTH: usize = 128;
const MAX_PARAMETERS: usize = 262_144;
const CONTEXT: &str = "urn:qualia:category:finite-affine-activation-optics-v1";

#[derive(Clone, Copy)]
enum Activation {
    Identity,
    Tanh,
    Sigmoid,
}

impl Activation {
    fn name(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::Tanh => "tanh",
            Self::Sigmoid => "sigmoid",
        }
    }

    fn apply(self, value: f64) -> f64 {
        match self {
            Self::Identity => value,
            Self::Tanh => value.tanh(),
            Self::Sigmoid if value < 0.0 => {
                let exponential = value.exp();
                exponential / (1.0 + exponential)
            }
            Self::Sigmoid => 1.0 / (1.0 + (-value).exp()),
        }
    }

    fn derivative(self, output: f64) -> f64 {
        match self {
            Self::Identity => 1.0,
            Self::Tanh => 1.0 - output * output,
            Self::Sigmoid => output * (1.0 - output),
        }
    }
}

struct Stage {
    in_width: usize,
    out_width: usize,
    activation: Activation,
    parameters: Vec<f64>,
    activations: Vec<f64>,
    gradients: Vec<f64>,
}

impl AssetHost<'_> {
    pub(crate) fn compose_parametric_optics_crdc(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "input",
                "stages",
                "target",
                "output_cotangent",
                "learning_rate",
                "arena_budget_bytes",
            ],
        )?;
        let stage_rows = list(required(args, "stages")?)?;
        let input = list(required(args, "input")?)?;
        if !(1..=MAX_STAGES).contains(&stage_rows.len()) || !(1..=MAX_WIDTH).contains(&input.len())
        {
            return Err(
                "This finite optics branch requires 1..=256 stages and width 1..=128".into(),
            );
        }
        let target = args.get("target").map(list).transpose()?;
        let cotangent = args.get("output_cotangent").map(list).transpose()?;
        if target.is_some() == cotangent.is_some() {
            return Err("Provide exactly one target or output_cotangent".into());
        }
        let rate = args.get("learning_rate").map(num).transpose()?;
        if rate.is_some_and(|rate| !(1e-12..=1e2).contains(&rate)) {
            return Err("learning_rate must be in 1e-12..=1e2".into());
        }
        let mut shape = Vec::with_capacity(stage_rows.len());
        let mut parameter_count = 0;
        let mut activation_count = 0;
        let mut in_width = input.len();
        for value in stage_rows {
            let row = record(value)?;
            fields(row, &["output_width", "activation", "weights", "bias"])?;
            let out_width = integer(required(row, "output_width")?, "output_width")?;
            let activation = match text(required(row, "activation")?)? {
                "identity" => Activation::Identity,
                "tanh" => Activation::Tanh,
                "sigmoid" => Activation::Sigmoid,
                _ => return Err("Unsupported smooth activation in finite optics chain".into()),
            };
            if !(1..=MAX_WIDTH).contains(&out_width) {
                return Err("A finite optics stage requires output_width in 1..=128".into());
            }
            let weights = list(required(row, "weights")?)?;
            let bias = list(required(row, "bias")?)?;
            if weights.len() != in_width * out_width || bias.len() != out_width {
                return Err(
                    "Stage coefficients must match the row-major weight and bias shapes".into(),
                );
            }
            shape.push((in_width, out_width, activation, weights, bias));
            parameter_count += weights.len() + bias.len();
            activation_count += out_width;
            in_width = out_width;
        }
        if parameter_count > MAX_PARAMETERS {
            return Err("Finite optics parameter count exceeds 262144".into());
        }
        let output_vector = target.or(cotangent).ok_or("Missing objective")?;
        if output_vector.len() != in_width {
            return Err("The objective vector must match the final output width".into());
        }
        let scratch = (parameter_count * 2 + activation_count + 3 * MAX_WIDTH) * size_of::<f64>()
            + stage_rows.len() * (size_of::<Stage>() + size_of_val(&shape[0]));
        budget(args, scratch)?;
        self.charge(parameter_count.saturating_mul(8), scratch)?;
        let input = numbers(input)?;
        let objective_vector = numbers(output_vector)?;
        let mut stages = shape
            .into_iter()
            .map(|(in_width, out_width, activation, weights, bias)| {
                let mut parameters = Vec::with_capacity(weights.len() + bias.len());
                for value in weights.iter().chain(bias) {
                    parameters.push(coefficient(value)?);
                }
                Ok(Stage {
                    in_width,
                    out_width,
                    activation,
                    parameters,
                    activations: vec![0.0; out_width],
                    gradients: vec![0.0; weights.len() + bias.len()],
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let problem = problem_identity(&input, &stages, &objective_vector, target.is_some(), rate);
        let mut propagated = [0.0; MAX_WIDTH];
        let (loss, output_width) = differentiate(
            &input,
            &objective_vector,
            target.is_some(),
            &mut stages,
            &mut propagated,
        )?;
        let value = stages.last().ok_or("Missing optics output stage")?;
        let mut receipt = ReceiptGraph::new(CONTEXT);
        receipt.fact(
            &problem,
            "urn:qualia:category:verified-finite-reverse-composition",
            CONTEXT,
        )?;
        receipt.fact(
            &problem,
            "urn:qualia:category:output-width",
            &format!("urn:qualia:integer:{output_width}"),
        )?;
        if let Some(loss) = loss {
            receipt.fact(
                &problem,
                "urn:qualia:category:objective-bits",
                &format!("urn:qualia:binary64:{:016x}", loss.to_bits()),
            )?;
        }
        let sealed = receipt.seal(self.limits.resource_bytes)?;
        let output = floats(&value.activations);
        let mut gradients = Vec::with_capacity(stages.len());
        let mut parameters = Vec::with_capacity(stages.len());
        for stage in stages {
            let weight_count = stage.in_width * stage.out_width;
            gradients.push(Value::Record(BTreeMap::from([
                ("weights".into(), floats(&stage.gradients[..weight_count])),
                ("bias".into(), floats(&stage.gradients[weight_count..])),
            ])));
            if let Some(rate) = rate {
                let updated = stage
                    .parameters
                    .iter()
                    .zip(&stage.gradients)
                    .map(|(parameter, gradient)| finite(parameter - rate * gradient))
                    .collect::<Result<Vec<_>, String>>()?;
                parameters.push(Value::Record(BTreeMap::from([
                    ("output_width".into(), Value::U64(stage.out_width as u64)),
                    (
                        "activation".into(),
                        Value::String(stage.activation.name().into()),
                    ),
                    ("weights".into(), floats(&updated[..weight_count])),
                    ("bias".into(), floats(&updated[weight_count..])),
                ])));
            }
        }
        let receipt = self.encode("q42", sealed)?;
        Ok(Value::Record(BTreeMap::from([
            ("output".into(), output),
            ("loss".into(), loss.map_or(Value::Null, Value::F64)),
            ("input_cotangent".into(), floats(&propagated[..input.len()])),
            ("gradients".into(), Value::List(gradients)),
            (
                "updated_stages".into(),
                rate.map_or(Value::Null, |_| Value::List(parameters)),
            ),
            ("receipt".into(), receipt),
        ])))
    }
}

fn finite(value: f64) -> Result<f64, String> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err("Nonfinite arithmetic in finite optics".into())
    }
}

fn coefficient(value: &Value) -> Result<f64, String> {
    let coefficient = num(value)?;
    if coefficient.abs() <= 1e12 {
        Ok(coefficient)
    } else {
        Err("Optics coefficient exceeds the 1e12 magnitude cap".into())
    }
}

fn numbers(values: &[Value]) -> Result<Vec<f64>, String> {
    values.iter().map(coefficient).collect()
}

fn floats(values: &[f64]) -> Value {
    Value::List(values.iter().copied().map(Value::F64).collect())
}

fn differentiate(
    input: &[f64],
    target: &[f64],
    squared_error: bool,
    stages: &mut [Stage],
    seed: &mut [f64; MAX_WIDTH],
) -> Result<(Option<f64>, usize), String> {
    for index in 0..stages.len() {
        let (previous, stages) = stages.split_at_mut(index);
        let stage = &mut stages[0];
        let source = previous
            .last()
            .map_or(input, |stage| stage.activations.as_slice());
        let count = stage.in_width * stage.out_width;
        for (row, value) in stage.activations.iter_mut().enumerate() {
            let weights = &stage.parameters[row * stage.in_width..(row + 1) * stage.in_width];
            let affine = weights
                .iter()
                .zip(source)
                .map(|(weight, input)| weight * input)
                .sum::<f64>()
                + stage.parameters[count + row];
            *value = finite(stage.activation.apply(finite(affine)?))?;
        }
    }
    let output = &stages.last().ok_or("Missing optics stages")?.activations;
    let output_width = output.len();
    let loss = if squared_error {
        let mut loss = 0.0;
        for (index, (&actual, &desired)) in output.iter().zip(target).enumerate() {
            seed[index] = finite(actual - desired)?;
            loss += 0.5 * seed[index] * seed[index];
        }
        Some(finite(loss)?)
    } else {
        seed[..output_width].copy_from_slice(target);
        None
    };
    let mut input_cotangent = [0.0; MAX_WIDTH];
    for index in (0..stages.len()).rev() {
        let (previous, stages) = stages.split_at_mut(index);
        let stage = &mut stages[0];
        let source = previous
            .last()
            .map_or(input, |stage| stage.activations.as_slice());
        input_cotangent[..stage.in_width].fill(0.0);
        let count = stage.in_width * stage.out_width;
        for (row, (&seed, &output)) in seed.iter().zip(&stage.activations).enumerate() {
            let delta = finite(seed * stage.activation.derivative(output))?;
            stage.gradients[count + row] = delta;
            for column in 0..stage.in_width {
                let weight = row * stage.in_width + column;
                stage.gradients[weight] = finite(delta * source[column])?;
                input_cotangent[column] =
                    finite(input_cotangent[column] + delta * stage.parameters[weight])?;
            }
        }
        seed[..stage.in_width].copy_from_slice(&input_cotangent[..stage.in_width]);
    }
    Ok((loss, output_width))
}

fn problem_identity(
    input: &[f64],
    stages: &[Stage],
    target: &[f64],
    squared_error: bool,
    rate: Option<f64>,
) -> String {
    let mut sha = Sha256::new();
    sha.update(b"Qualia finite parameter optic v1\0");
    sha.update([u8::from(squared_error)]);
    sha.update(rate.unwrap_or(0.0).to_le_bytes());
    sha.update((input.len() as u64).to_le_bytes());
    sha.update((stages.len() as u64).to_le_bytes());
    for value in input.iter().chain(target) {
        sha.update(value.to_le_bytes());
    }
    for stage in stages {
        sha.update((stage.in_width as u64).to_le_bytes());
        sha.update((stage.out_width as u64).to_le_bytes());
        sha.update(stage.activation.name().as_bytes());
        sha.update([0]);
        for value in &stage.parameters {
            sha.update(value.to_le_bytes());
        }
    }
    format!("urn:sha256:{:x}", sha.finalize())
}
