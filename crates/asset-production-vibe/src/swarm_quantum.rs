//! Offline unitary simulation of bounded authored circuits.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_4, PI};
use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::artifact::FieldLattice;
use crate::primal_replay::{COUNTER_ALGORITHM, counter_uniform};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{Args, AssetHost, TypedArtifact, Value, fields, integer, list, num, record, required, text};

const MAX_QUBITS: usize = 12;
const MAX_GATES: usize = 512;
const MAX_SOURCE_BYTES: usize = 24 * 1024;
const MAX_SHOTS: usize = 65_536;

#[derive(Clone, Copy)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    const ZERO: Self = Self { re: 0.0, im: 0.0 };
    const ONE: Self = Self { re: 1.0, im: 0.0 };

    fn real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    fn phase(radians: f64) -> Self {
        let (im, re) = radians.sin_cos();
        Self { re, im }
    }

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    fn add(self, other: Self) -> Self {
        Self { re: self.re + other.re, im: self.im + other.im }
    }

    fn norm_squared(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

enum Gate {
    Matrix {
        target: usize,
        control: Option<usize>,
        coefficients: [[Complex; 2]; 2],
    },
    Swap(usize, usize),
}

struct Circuit {
    qubits: usize,
    gates: Vec<Gate>,
}

impl AssetHost<'_> {
    pub(crate) fn simulate_openqasm_circuit(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "circuit", "shots", "seed"])?;
        let circuit = match (args.get("source"), args.get("circuit")) {
            (Some(source), None) => parse_qasm(text(source)?)?,
            (None, Some(circuit)) => parse_authored_circuit(circuit)?,
            _ => return Err("Specify exactly one unitary OpenQASM source or circuit".into()),
        };
        let shots = args.get("shots").map(|value| integer(value, "shots")).transpose()?.unwrap_or(0);
        if shots > MAX_SHOTS {
            return Err(format!("shots exceeds {MAX_SHOTS}"));
        }
        let seed = match args.get("seed") {
            Some(Value::U64(value)) => *value,
            Some(Value::I64(value)) if *value >= 0 => *value as u64,
            None if shots == 0 => 0,
            _ => return Err("Nonzero shots require an explicit unsigned seed".into()),
        };
        let basis = 1usize << circuit.qubits;
        let work = basis * (circuit.gates.len() * 8 + circuit.qubits + 6) + shots * circuit.qubits;
        self.charge(work, basis * 56)?;
        let mut state = vec![Complex::ZERO; basis];
        state[0] = Complex::ONE;
        for gate in &circuit.gates {
            self.charge(0, 0)?;
            apply_gate(gate, &mut state);
        }
        let norm: f64 = state.iter().map(|value| value.norm_squared()).sum();
        if !norm.is_finite() || (norm - 1.0).abs() > 1.0e-10 {
            return Err("Circuit exceeded the state norm error tolerance".into());
        }
        let mut cumulative = Vec::with_capacity(basis);
        let mut probabilities = Vec::with_capacity(basis);
        let mut expectation_z = vec![0.0_f64; circuit.qubits];
        let mut sum = 0.0_f64;
        for (index, amplitude) in state.iter().enumerate() {
            let p = amplitude.norm_squared() / norm;
            probabilities.push(Value::F64(p));
            sum += p;
            cumulative.push(sum.min(1.0));
            for (qubit, expectation) in expectation_z.iter_mut().enumerate() {
                *expectation += if index & (1 << qubit) == 0 { p } else { -p };
            }
        }
        cumulative[basis - 1] = 1.0;
        let mut counts = vec![0_u64; basis];
        for shot in 0..shots {
            let draw = counter_uniform(seed, 0, shot as u64);
            counts[cumulative.partition_point(|p| *p <= draw)] += 1;
        }

        let mut digest = Sha256::new();
        digest.update((circuit.qubits as u64).to_le_bytes());
        for amplitude in &state {
            digest.update(amplitude.re.to_bits().to_le_bytes());
            digest.update(amplitude.im.to_bits().to_le_bytes());
        }
        let subject = format!("urn:qualia:quantum:state:sha256:{:x}", digest.finalize());
        let mut receipt = ReceiptGraph::new("urn:qualia:quantum:unitary-circuit");
        receipt.fact(&subject, "urn:qualia:quantum:qubits", &format!("urn:qualia:uint:{}", circuit.qubits))?;
        receipt.fact(&subject, "urn:qualia:quantum:basis", "urn:qualia:quantum:computational-lsb0")?;
        receipt.fact(&subject, "urn:qualia:quantum:simulation", "urn:qualia:quantum:offline-unitary-f64")?;
        if shots > 0 {
            let run = format!("{subject}:samples:{seed}:{shots}");
            receipt.fact(&run, "urn:qualia:quantum:prepared-state", &subject)?;
            receipt.fact(&run, "urn:qualia:quantum:sampling", &format!("urn:qualia:rng:{COUNTER_ALGORITHM}"))?;
        }
        let proof = self.encode("q42", receipt.seal(self.limits.resource_bytes)?)?;
        let samples = state.into_iter().flat_map(|value| [value.re as f32, value.im as f32]).collect();
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(FieldLattice::DenseF32 {
            axes: vec![2, basis as u32],
            samples,
        })))?;
        Ok(Value::Record(Args::from([
            ("state".into(), field),
            ("state_iri".into(), Value::String(subject)),
            ("probabilities".into(), Value::List(probabilities)),
            ("expectation_z".into(), Value::List(expectation_z.into_iter().map(Value::F64).collect())),
            ("counts".into(), Value::List(counts.into_iter().map(Value::U64).collect())),
            ("norm_squared".into(), Value::F64(norm)),
            ("receipt".into(), proof),
        ])))
    }
}

fn parse_authored_circuit(value: &Value) -> Result<Circuit, String> {
    let value = record(value)?;
    fields(value, &["qubits", "gates"])?;
    let qubits = qubit_count(integer(required(value, "qubits")?, "qubits")?)?;
    let entries = list(required(value, "gates")?)?;
    if entries.len() > MAX_GATES {
        return Err("Circuit exceeds gate limit".into());
    }
    let gates = entries.iter().map(|gate| {
        let gate = record(gate)?;
        fields(gate, &["name", "qubits", "angle_rad"])?;
        let operands = list(required(gate, "qubits")?)?;
        if !(1..=2).contains(&operands.len()) {
            return Err("Gate requires one or two operands".into());
        }
        let operands = operands.iter().map(|value| integer(value, "qubit")).collect::<Result<Vec<_>, _>>()?;
        let angle = gate.get("angle_rad").map(num).transpose()?;
        make_gate(text(required(gate, "name")?)?, &operands, qubits, angle)
    }).collect::<Result<Vec<_>, _>>()?;
    Ok(Circuit { qubits, gates })
}

fn qubit_count(count: usize) -> Result<usize, String> {
    (1..=MAX_QUBITS).contains(&count).then_some(count)
        .ok_or_else(|| format!("Circuit qubits outside 1..{MAX_QUBITS}"))
}

fn parse_qasm(source: &str) -> Result<Circuit, String> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err("OpenQASM source exceeds byte limit".into());
    }
    let source = source.lines().map(|line| line.split_once("//").map_or(line, |(code, _)| code))
        .collect::<Vec<_>>().join("\n");
    let source = source.trim().strip_suffix(';').ok_or("OpenQASM statements require semicolons")?;
    let mut statements = source.split(';');
    let version = statements.next().ok_or("Missing OpenQASM header")?;
    let version: String = version.split_whitespace().collect::<Vec<_>>().join(" ");
    let v3 = match version.as_str() {
        "OPENQASM 2.0" => false,
        "OPENQASM 3" | "OPENQASM 3.0" => true,
        _ => return Err("Expected an OpenQASM 2.0 or 3.0 header".into()),
    };
    let mut register_name = None;
    let mut qubits = 0;
    let mut gates = Vec::new();
    for statement in statements {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        if statement == if v3 { "include \"stdgates.inc\"" } else { "include \"qelib1.inc\"" } {
            continue;
        }
        if register_name.is_none() {
            let (name, count) = parse_register(statement, v3)?;
            register_name = Some(name);
            qubits = qubit_count(count)?;
            continue;
        }
        if gates.len() == MAX_GATES {
            return Err("OpenQASM circuit exceeds gate limit".into());
        }
        let (name, operands) = statement.split_once(char::is_whitespace).ok_or("Expected OpenQASM gate operands")?;
        let (name, angle) = if let Some((name, value)) = name.split_once('(') {
            (name, Some(qasm_angle(value.strip_suffix(')').ok_or("Invalid OpenQASM rotation")?)?))
        } else {
            (name, None)
        };
        let mut indices = [0; 2];
        let mut count = 0;
        for value in operands.split(',') {
            if count == indices.len() {
                return Err("Unsupported OpenQASM gate arity".into());
            }
            let (register, index) = subscript(value.trim())?;
            if Some(register) != register_name {
                return Err("Operand must refer to the declared qubit register".into());
            }
            indices[count] = index;
            count += 1;
        }
        gates.push(make_gate(name, &indices[..count], qubits, angle)?);
    }
    if register_name.is_none() {
        return Err("OpenQASM source requires one declared quantum register".into());
    }
    Ok(Circuit { qubits, gates })
}

fn parse_register(statement: &str, v3: bool) -> Result<(&str, usize), String> {
    if v3 {
        let declaration = statement.strip_prefix("qubit[").ok_or("Expected qubit[N] register declaration")?;
        let (size, name) = declaration.split_once(']').ok_or("Invalid qubit extent")?;
        let name = name.trim();
        if !identifier(name) {
            return Err("Invalid OpenQASM register name".into());
        }
        Ok((name, size.parse().map_err(|_| "Invalid qubit extent")?))
    } else {
        let declaration = statement.strip_prefix("qreg ").ok_or("Expected qreg name[N] declaration")?;
        subscript(declaration.trim())
    }
}

fn subscript(value: &str) -> Result<(&str, usize), String> {
    let (name, index) = value.split_once('[').ok_or("Expected indexed qubit")?;
    if !identifier(name) {
        return Err("Invalid OpenQASM register name".into());
    }
    let index = index.strip_suffix(']').ok_or("Invalid qubit subscript")?;
    Ok((name, index.parse().map_err(|_| "Invalid qubit subscript")?))
}

fn identifier(value: &str) -> bool {
    let mut bytes = value.bytes();
    bytes.next().is_some_and(|b| b.is_ascii_alphabetic() || b == b'_')
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
}

fn qasm_angle(value: &str) -> Result<f64, String> {
    let scalar = |term: &str| -> Result<f64, String> {
        match term {
            "pi" | "+pi" => Ok(PI),
            "-pi" => Ok(-PI),
            _ => term.parse().map_err(|_| "Expected a numeric OpenQASM angle or signed pi".into()),
        }
    };
    if let Some((numerator, denominator)) = value.split_once('/') {
        Ok(scalar(numerator)? / scalar(denominator)?)
    } else {
        scalar(value)
    }
}

fn make_gate(name: &str, operands: &[usize], qubits: usize, angle: Option<f64>) -> Result<Gate, String> {
    let two = matches!(name, "cx" | "cz" | "swap");
    if operands.len() != if two { 2 } else { 1 }
        || operands.iter().any(|&index| index >= qubits)
        || (two && operands[0] == operands[1])
    {
        return Err(format!("Invalid operands for gate {name}"));
    }
    let rotation = matches!(name, "rx" | "ry" | "rz");
    if angle.is_some() != rotation || angle.is_some_and(|a| !a.is_finite() || a.abs() > 1.0e6) {
        return Err(format!("Invalid rotation angle for gate {name}"));
    }
    if name == "swap" {
        return Ok(Gate::Swap(operands[0], operands[1]));
    }
    let zero = Complex::ZERO;
    let one = Complex::ONE;
    let minus_one = Complex::real(-1.0);
    let s = Complex::real(FRAC_1_SQRT_2);
    let coefficients = match name {
        "id" => [[one, zero], [zero, one]],
        "h" => [[s, s], [s, Complex::real(-FRAC_1_SQRT_2)]],
        "x" | "cx" => [[zero, one], [one, zero]],
        "y" => [[zero, Complex { re: 0.0, im: -1.0 }], [Complex { re: 0.0, im: 1.0 }, zero]],
        "z" | "cz" => [[one, zero], [zero, minus_one]],
        "s" => [[one, zero], [zero, Complex { re: 0.0, im: 1.0 }]],
        "sdg" => [[one, zero], [zero, Complex { re: 0.0, im: -1.0 }]],
        "t" => [[one, zero], [zero, Complex::phase(FRAC_PI_4)]],
        "tdg" => [[one, zero], [zero, Complex::phase(-FRAC_PI_4)]],
        "rx" | "ry" | "rz" => {
            let half = angle.ok_or("Missing rotation angle")? / 2.0;
            let (s, c) = half.sin_cos();
            let diagonal = Complex::real(c);
            match name {
                "rx" => {
                    let off = Complex { re: 0.0, im: -s };
                    [[diagonal, off], [off, diagonal]]
                }
                "ry" => [[diagonal, Complex::real(-s)], [Complex::real(s), diagonal]],
                _ => [[Complex::phase(-half), zero], [zero, Complex::phase(half)]],
            }
        }
        _ => return Err(format!("Unsupported unitary circuit gate {name}")),
    };
    let (target, control) = if two { (operands[1], Some(operands[0])) } else { (operands[0], None) };
    Ok(Gate::Matrix { target, control, coefficients })
}

fn apply_gate(gate: &Gate, state: &mut [Complex]) {
    match gate {
        Gate::Matrix { target, control, coefficients } => {
            let target_mask = 1 << target;
            for index in 0..state.len() {
                if index & target_mask != 0 || control.is_some_and(|bit| index & (1 << bit) == 0) {
                    continue;
                }
                let other = index | target_mask;
                let a = state[index];
                let b = state[other];
                state[index] = coefficients[0][0].mul(a).add(coefficients[0][1].mul(b));
                state[other] = coefficients[1][0].mul(a).add(coefficients[1][1].mul(b));
            }
        }
        Gate::Swap(a, b) => {
            let first = 1 << a;
            let second = 1 << b;
            for index in 0..state.len() {
                if index & first == 0 && index & second != 0 {
                    state.swap(index, index ^ first ^ second);
                }
            }
        }
    }
}
