//! Deterministic offline execution of an authored Boolean scan program.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use chronicle_tabular_engine::Value as CellValue;

use super::{bounded, table_receipt};
use crate::{Args, AssetHost, Value, fields, integer, list, record, required, text};

enum Op {
    Copy(usize),
    Not(usize),
    And(usize, usize),
    Or(usize, usize),
    Xor(usize, usize),
}

impl Op {
    fn eval(&self, state: &[bool]) -> bool {
        match *self {
            Self::Copy(a) => state[a],
            Self::Not(a) => !state[a],
            Self::And(a, b) => state[a] && state[b],
            Self::Or(a, b) => state[a] || state[b],
            Self::Xor(a, b) => state[a] ^ state[b],
        }
    }
}

fn bit(value: &Value) -> Result<bool, String> {
    if let Value::Bool(value) = value {
        Ok(*value)
    } else {
        Err("PLC state requires Boolean signals".into())
    }
}

impl AssetHost<'_> {
    pub(crate) fn bridge_iec61131_plc_and_ros2_hil(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "scan_cycle_s",
                "initial_registers",
                "scan_inputs",
                "program",
            ],
        )?;
        let cycle = bounded(args, "scan_cycle_s", None, 1e-4, 10.0)?;
        let registers = list(required(args, "initial_registers")?)?;
        let inputs = list(required(args, "scan_inputs")?)?;
        let program = list(required(args, "program")?)?;
        if !(1..=128).contains(&registers.len())
            || !(1..=2048).contains(&inputs.len())
            || inputs.len() * registers.len() > 8192
            || !(1..=512).contains(&program.len())
        {
            return Err("Boolean scan exceeds admitted scenario size".into());
        }
        let first = list(&inputs[0])?;
        if first.len() > 128 {
            return Err("Boolean scan supports up to 128 input bits".into());
        }
        let width = first.len();
        self.charge(
            inputs.len() * (program.len() + registers.len() * 16 + width),
            3072 * inputs.len() * registers.len(),
        )?;
        let slots = width + registers.len();
        let mut state = vec![false; width];
        state.extend(registers.iter().map(bit).collect::<Result<Vec<_>, _>>()?);
        let mut compiled = Vec::with_capacity(program.len());
        for value in program {
            let row = record(value)?;
            fields(row, &["op", "dst", "inputs"])?;
            let dst = integer(required(row, "dst")?, "PLC destination")?;
            if !(width..slots).contains(&dst) {
                return Err("PLC destinations must designate writable register slots".into());
            }
            let read = list(required(row, "inputs")?)?
                .iter()
                .map(|value| {
                    let slot = integer(value, "PLC input slot")?;
                    if slot < slots {
                        Ok(slot)
                    } else {
                        Err("PLC read slot is out of range".into())
                    }
                })
                .collect::<Result<Vec<_>, String>>()?;
            let op = match (text(required(row, "op")?)?, read.as_slice()) {
                ("copy", &[a]) => Op::Copy(a),
                ("not", &[a]) => Op::Not(a),
                ("and", &[a, b]) => Op::And(a, b),
                ("or", &[a, b]) => Op::Or(a, b),
                ("xor", &[a, b]) => Op::Xor(a, b),
                _ => return Err("PLC instruction has unsupported operation or arity".into()),
            };
            compiled.push((dst, op));
        }
        let mut rows = Vec::with_capacity(inputs.len() * registers.len());
        for (scan, inputs) in inputs.iter().enumerate() {
            let inputs = list(inputs)?;
            if inputs.len() != width {
                return Err(format!("Scan {scan} lacks its full input snapshot"));
            }
            for (slot, input) in inputs.iter().enumerate() {
                state[slot] = bit(input)?;
            }
            for (dst, instruction) in &compiled {
                let next = instruction.eval(&state);
                state[*dst] = next;
            }
            for (offset, &register) in state[width..].iter().enumerate() {
                rows.push(vec![
                    CellValue::number(scan as f64),
                    CellValue::number((scan + 1) as f64 * cycle),
                    CellValue::number((width + offset) as f64),
                    CellValue::Bool(register),
                ]);
            }
        }
        let mut out = table_receipt(
            self,
            "urn:qualia:civic:authored-offline-boolean-scan",
            &["scan", "completed_time_s", "register_slot", "value"],
            &rows,
        )?;
        let state = state[width..]
            .iter()
            .copied()
            .map(Value::Bool)
            .collect::<Vec<_>>();
        out.insert("final_registers".into(), Value::List(state.into()));
        Ok(Value::Record(out))
    }
}
