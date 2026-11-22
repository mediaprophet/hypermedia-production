//! Local Touchstone S-parameter import and linear DC circuit analysis.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::{BTreeMap, BTreeSet};

use chronicle_tabular_engine::Value as CellValue;

use super::{least_squares, numeric_row, table_receipt};
use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::{Args, AssetHost, Value, fields, required, text};

#[derive(Clone, Copy)]
enum ParameterForm {
    RealImaginary,
    MagnitudeAngle,
    DecibelAngle,
}

impl ParameterForm {
    fn convert(self, x: f64, y: f64) -> Result<[f64; 2], String> {
        let result = match self {
            Self::RealImaginary => [x, y],
            Self::MagnitudeAngle if x < 0.0 => {
                return Err("Touchstone magnitude cannot be negative".into());
            }
            form => {
                let mag = if matches!(form, Self::DecibelAngle) {
                    10.0_f64.powf(x / 20.0)
                } else {
                    x
                };
                let (sine, cosine) = y.rem_euclid(360.0).to_radians().sin_cos();
                [mag * cosine, mag * sine]
            }
        };
        if result[0].hypot(result[1]) > 1e6 || result.iter().any(|x| !x.is_finite()) {
            return Err("Touchstone parameter exceeds admitted amplitude".into());
        }
        Ok(result)
    }
}

fn scalar(value: &str) -> Result<f64, String> {
    let parsed = value
        .parse::<f64>()
        .map_err(|_| "Malformed circuit numerical token")?;
    if parsed.is_finite() {
        Ok(parsed)
    } else {
        Err("Circuit numerical token must be finite".into())
    }
}

fn spectral_norm(s: [[f64; 2]; 4]) -> f64 {
    // File order is S11, S21, S12, S22. Columns therefore occupy adjacent pairs.
    let [[a, b], [c, d], [e, f], [g, h]] = s;
    let x = a * a + b * b + c * c + d * d;
    let y = e * e + f * f + g * g + h * h;
    let real = a * e + b * f + c * g + d * h;
    let imag = a * f - b * e + c * h - d * g;
    // Largest eigenvalue of the Hermitian matrix S* S.
    (0.5 * (x + y + (x - y).hypot(2.0 * real.hypot(imag)))).sqrt()
}

fn touchstone(source: &str) -> Result<(f64, f64, Vec<Vec<CellValue>>), String> {
    // Explicit one-line S2P data sets in the established 1.x order.
    // See https://www.ibis.org/touchstone_ver2.1/touchstone_ver2_1.pdf
    let mut options = None;
    let mut previous = -1.0;
    let mut maximum = 0.0_f64;
    let mut rows = Vec::new();
    for raw in source.lines() {
        let line = raw.split('!').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if let Some(option) = line.strip_prefix('#') {
            if options.is_some() {
                return Err("Duplicate Touchstone option line".into());
            }
            let option = option.to_ascii_uppercase();
            let fields = option.split_whitespace().collect::<Vec<_>>();
            let [units, "S", form, "R", reference] = fields.as_slice() else {
                return Err(
                    "Touchstone requires explicit units, S, RI/MA/DB and R reference".into(),
                );
            };
            let scale = match *units {
                "HZ" => 1.0,
                "KHZ" => 1e3,
                "MHZ" => 1e6,
                "GHZ" => 1e9,
                _ => return Err("Unknown Touchstone frequency unit".into()),
            };
            let form = match *form {
                "RI" => ParameterForm::RealImaginary,
                "MA" => ParameterForm::MagnitudeAngle,
                "DB" => ParameterForm::DecibelAngle,
                _ => return Err("Unknown Touchstone complex parameter form".into()),
            };
            let resistance = scalar(reference)?;
            if !(1e-3..=1e6).contains(&resistance) {
                return Err("Touchstone reference impedance outside admitted [1e-3, 1e6] Ω".into());
            }
            options = Some((scale, form, resistance));
            continue;
        }
        let (scale, form, _) =
            options.ok_or("Expected explicit Touchstone option line before data")?;
        if rows.len() == 8192 {
            return Err("Touchstone exceeds the admitted frequency sample count".into());
        }
        let mut values = [0.0; 9];
        let mut tokens = line.split_whitespace();
        for value in &mut values {
            *value = scalar(
                tokens
                    .next()
                    .ok_or("Touchstone requires a complete two-port row")?,
            )?;
        }
        if tokens.next().is_some() {
            return Err("Touchstone row does not have exactly nine numerical fields".into());
        }
        let freq = values[0] * scale;
        if !(0.0..=1e12).contains(&freq) || freq <= previous {
            return Err("Touchstone frequencies must strictly increase within [0, 1e12] Hz".into());
        }
        previous = freq;
        let [a, b, c, d] = [
            form.convert(values[1], values[2])?,
            form.convert(values[3], values[4])?,
            form.convert(values[5], values[6])?,
            form.convert(values[7], values[8])?,
        ];
        let gain = spectral_norm([a, b, c, d]);
        maximum = maximum.max(gain);
        rows.push(numeric_row([
            freq, a[0], a[1], b[0], b[1], c[0], c[1], d[0], d[1], gain,
        ]));
    }
    let (_, _, resistance) = options.ok_or("Missing Touchstone option line")?;
    if rows.is_empty() {
        return Err("Touchstone has no two-port samples".into());
    }
    Ok((resistance, maximum, rows))
}

enum Element {
    Resistor(f64),
    Current(f64),
    Voltage(f64),
}

fn spice_scalar(value: &str) -> Result<f64, String> {
    if let Ok(value) = scalar(value) {
        return Ok(value);
    }
    let lower = value.to_ascii_lowercase();
    for (suffix, factor) in [
        ("meg", 1e6),
        ("g", 1e9),
        ("k", 1e3),
        ("m", 1e-3),
        ("u", 1e-6),
        ("n", 1e-9),
        ("p", 1e-12),
    ] {
        if let Some(base) = lower.strip_suffix(suffix) {
            let parsed = scalar(base)? * factor;
            if parsed.is_finite() {
                return Ok(parsed);
            }
        }
    }
    Err("Unsupported SPICE numeric value or suffix".into())
}

fn spice_node(ids: &mut BTreeMap<String, usize>, token: &str) -> Result<Option<usize>, String> {
    if token == "0" {
        return Ok(None);
    }
    if token.is_empty()
        || token.len() > 63
        || !token
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'.'))
    {
        return Err("Unsupported SPICE node name".into());
    }
    let slot = ids.len();
    Ok(Some(*ids.entry(token.to_ascii_lowercase()).or_insert(slot)))
}

fn solve_dc(source: &str) -> Result<(Vec<Vec<CellValue>>, f64), String> {
    let mut nodes = BTreeMap::new();
    let mut devices = BTreeSet::new();
    let mut elements = Vec::new();
    let mut grounds = false;
    let mut ended = false;
    let mut voltage_count = 0;
    for raw in source.lines() {
        let line = raw.split('$').next().unwrap_or("").trim();
        if line.is_empty() || line.starts_with('*') {
            continue;
        }
        if ended {
            return Err("Unexpected SPICE statement after .end".into());
        }
        if line.eq_ignore_ascii_case(".end") {
            ended = true;
            continue;
        }
        if line.eq_ignore_ascii_case(".op") {
            continue;
        }
        let row = line.split_whitespace().collect::<Vec<_>>();
        if row
            .first()
            .is_some_and(|name| name.eq_ignore_ascii_case(".title"))
            && elements.is_empty()
        {
            continue;
        }
        let (name, p, n, value) = match row.as_slice() {
            &[name, p, n, value] => (name, p, n, value),
            &[name, p, n, dc, value]
                if !name.as_bytes()[0].eq_ignore_ascii_case(&b'R')
                    && dc.eq_ignore_ascii_case("dc") =>
            {
                (name, p, n, value)
            }
            _ => return Err("SPICE DC subset accepts explicit R, I and V devices".into()),
        };
        if elements.len() >= 512 || name.len() > 64 || !devices.insert(name.to_ascii_lowercase()) {
            return Err("SPICE device names must be unique and fit the 512-device budget".into());
        }
        let p = spice_node(&mut nodes, p)?;
        let n = spice_node(&mut nodes, n)?;
        if p == n || nodes.len() > 128 {
            return Err("SPICE device terminals coincide or exceed the 128-node budget".into());
        }
        grounds |= p.is_none() || n.is_none();
        let value = spice_scalar(value)?;
        let element = match name.as_bytes()[0].to_ascii_uppercase() {
            b'R' if (1e-6..=1e12).contains(&value) => Element::Resistor(1.0 / value),
            b'I' if value.abs() <= 1e9 => Element::Current(value),
            b'V' if value.abs() <= 1e9 => {
                voltage_count += 1;
                Element::Voltage(value)
            }
            _ => return Err("Unsupported SPICE DC element or rating".into()),
        };
        elements.push((name, p, n, element));
    }
    let order = nodes.len() + voltage_count;
    if !grounds || order == 0 || order > 192 {
        return Err("Linear DC netlist needs a ground and at most 192 unknowns".into());
    }
    let mut a = vec![0.0; order * order];
    let mut b = vec![0.0; order];
    let mut voltages = Vec::with_capacity(voltage_count);
    for (name, p, n, element) in elements {
        match element {
            Element::Resistor(g) => {
                for (a_node, b_node) in [(p, n), (n, p)] {
                    if let Some(i) = a_node {
                        a[i * order + i] += g;
                        if let Some(j) = b_node {
                            a[i * order + j] -= g;
                        }
                    }
                }
            }
            Element::Current(current) => {
                if let Some(i) = p {
                    b[i] -= current;
                }
                if let Some(i) = n {
                    b[i] += current;
                }
            }
            Element::Voltage(voltage) => {
                let j = nodes.len() + voltages.len();
                for (node, sign) in [(p, 1.0), (n, -1.0)] {
                    if let Some(i) = node {
                        a[i * order + j] += sign;
                        a[j * order + i] += sign;
                    }
                }
                b[j] = voltage;
                voltages.push((name, j));
            }
        }
    }
    let mut normal = a.clone();
    let mut rhs = b.clone();
    for i in 0..order {
        let row = &mut normal[i * order..(i + 1) * order];
        let scale = row.iter().map(|v| v.abs()).fold(0.0_f64, f64::max);
        if scale == 0.0 {
            return Err("SPICE DC system contains an unconstrained node".into());
        }
        for v in row {
            *v /= scale;
        }
        rhs[i] /= scale;
    }
    let x = least_squares(order, order, normal, rhs)?;
    let residual = a
        .chunks_exact(order)
        .zip(&b)
        .map(|(row, &rhs)| {
            let total: f64 = row.iter().zip(&x).map(|(a, x)| a * x).sum();
            let scale = row.iter().zip(&x).map(|(a, x)| (a * x).abs()).sum::<f64>() + rhs.abs();
            (total - rhs).abs() / scale.max(1.0)
        })
        .fold(0.0_f64, f64::max);
    if !residual.is_finite() || residual > 1e-9 {
        return Err(format!(
            "SPICE DC scaled equation residual {residual:.9e} above 1e-9"
        ));
    }
    let rows = nodes
        .iter()
        .map(|(name, &i)| (name.as_str(), i, "node_voltage_v"))
        .chain(
            voltages
                .into_iter()
                .map(|(name, i)| (name, i, "source_current_a")),
        )
        .map(|(name, i, quantity)| {
            vec![
                CellValue::text(name),
                CellValue::text(quantity),
                CellValue::number(x[i]),
            ]
        })
        .collect();
    Ok((rows, residual))
}

impl AssetHost<'_> {
    pub(crate) fn transcode_eda_spice_touchstone_odb(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["operation", "source"])?;
        let operation = text(required(args, "operation")?)?;
        let source = required(args, "source")?;
        let resource = encoded_resource(self, source)?;
        let bytes = resource.bytes.len();
        if !(1..=1024 * 1024).contains(&bytes) {
            return Err("EDA source exceeds the admitted 1 MiB text budget".into());
        }
        let work = match operation {
            "touchstone_s2p" => bytes * 32,
            "spice_dc" => bytes * 32 + 192_usize.pow(3) * 8,
            _ => return Err("Supported EDA modes are touchstone_s2p and spice_dc".into()),
        };
        self.charge(work, bytes * 8 + 24 * 1024 * 1024)?;
        let resource = encoded_resource(self, source)?;
        let input = std::str::from_utf8(&resource.bytes).map_err(|_| "EDA text must be UTF-8")?;
        let result = if operation == "touchstone_s2p" {
            let (reference, maximum, rows) = touchstone(input)?;
            let mut out = table_receipt(
                self,
                "urn:qualia:civic:touchstone-s2p-sampled",
                &[
                    "frequency_hz",
                    "s11_real",
                    "s11_imag",
                    "s21_real",
                    "s21_imag",
                    "s12_real",
                    "s12_imag",
                    "s22_real",
                    "s22_imag",
                    "max_singular_value",
                ],
                &rows,
            )?;
            out.insert("reference_impedance_ohm".into(), Value::F64(reference));
            out.insert("max_sampled_singular_value".into(), Value::F64(maximum));
            out.insert(
                "passive_at_samples".into(),
                Value::Bool(maximum <= 1.0 + 1e-9),
            );
            out
        } else {
            let (rows, residual) = solve_dc(input)?;
            let mut out = table_receipt(
                self,
                "urn:qualia:civic:linear-spice-dc",
                &["name", "quantity", "value"],
                &rows,
            )?;
            out.insert("max_scaled_equation_residual".into(), Value::F64(residual));
            out
        };
        Ok(Value::Record(result))
    }
}
