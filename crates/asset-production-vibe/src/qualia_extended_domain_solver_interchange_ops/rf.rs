//! Authored radio-link geometry and semantic clearance observations.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    fields, list, num, number, record, required, text, vector, Args, AssetHost, Value,
};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use std::collections::{BTreeMap, BTreeSet};

const C_M_S: f64 = 299_792_458.0;
const GRAPH: &str = "urn:qualia:rf:clearance-observation:v1";

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Value, String> {
    if op != "fresnel_clearance" {
        return Err(format!("Unsupported RF geometry operation: {op}"));
    }
    fields(args, &["link_iri", "transmitter_m", "receiver_m", "frequency_hz", "clearance_fraction", "samples"])?;
    let link = text(required(args, "link_iri")?)?;
    let transmitter = point(required(args, "transmitter_m")?)?;
    let receiver = point(required(args, "receiver_m")?)?;
    let frequency = num(required(args, "frequency_hz")?)?;
    let fraction = number(args, "clearance_fraction", 0.6)?;
    if !(1.0..=1e15).contains(&frequency) || !(0.0..=1.0).contains(&fraction) {
        return Err("RF frequency or desired first-zone clearance fraction is outside the supported range".into());
    }
    let displacement = difference(receiver, transmitter);
    let distance = length(displacement);
    if distance < 1e-6 {
        return Err("Fresnel clearance requires distinct link endpoints".into());
    }
    let samples = list(required(args, "samples")?)?;
    if samples.is_empty() || samples.len() > 8192 {
        return Err("Fresnel clearance requires 1–8192 authored obstruction samples".into());
    }
    host.charge(samples.len() * 32, samples.len() * 256)?;
    let wavelength = C_M_S / frequency;
    let unit = displacement.map(|value| value / distance);
    let mut seen = BTreeSet::new();
    let mut closest = None::<f64>;
    let mut tested = 0u64;
    let mut observations = Vec::with_capacity(samples.len());
    let mut graph = ReceiptGraph::new(GRAPH);
    graph.fact(link, "urn:qualia:rf:frequencyHz", &word(frequency))?;
    graph.fact(link, "urn:qualia:rf:clearanceFraction", &word(fraction))?;
    for value in samples {
        let sample = record(value)?;
        fields(sample, &["iri", "position_m"])?;
        let iri = text(required(sample, "iri")?)?;
        if iri.len() > 4096 || !seen.insert(iri) {
            return Err("Obstruction sample IRIs must be distinct and bounded".into());
        }
        let relative = difference(point(required(sample, "position_m")?)?, transmitter);
        let along = dot(relative, unit);
        let in_span = along > 0.0 && along < distance;
        let mut observation = BTreeMap::from([
            ("iri".into(), Value::String(iri.into())),
            ("along_m".into(), Value::F64(along)),
            ("in_span".into(), Value::Bool(in_span)),
        ]);
        if in_span {
            let perpendicular = length(difference(relative, unit.map(|component| component * along)));
            let radius = (wavelength * along * ((distance - along) / distance)).sqrt();
            let margin = perpendicular - fraction * radius;
            closest = Some(closest.map_or(margin, |previous| previous.min(margin)));
            tested += 1;
            graph.fact(link, "urn:qualia:rf:clearanceSample", iri)?;
            graph.fact(iri, "urn:qualia:rf:firstZoneRadiusMetres", &word(radius))?;
            graph.fact(iri, "urn:qualia:rf:clearanceMarginMetres", &word(margin))?;
            observation.insert("radius_m".into(), Value::F64(radius));
            observation.insert("radial_distance_m".into(), Value::F64(perpendicular));
            observation.insert("margin_m".into(), Value::F64(margin));
        }
        observations.push(Value::Record(observation));
    }
    let receipt = graph.seal(host.limits.resource_bytes.min(4 * 1024 * 1024))?;
    Ok(Value::Record(BTreeMap::from([
        ("observations".into(), Value::List(observations)),
        ("in_span_count".into(), Value::U64(tested)),
        ("minimum_sample_margin_m".into(), closest.map(Value::F64).unwrap_or(Value::Null)),
        ("link_distance_m".into(), Value::F64(distance)),
        ("receipt".into(), host.encode("q42", receipt)?),
    ])))
}

fn point(value: &Value) -> Result<[f64; 3], String> {
    let coordinates = vector::<3>(value)?;
    if coordinates.iter().any(|value| value.abs() > 1e12) {
        return Err("Radio link coordinates exceed the supported metric frame".into());
    }
    Ok(coordinates)
}

fn difference(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|axis| a[axis] - b[axis])
}

fn length(value: [f64; 3]) -> f64 {
    dot(value, value).sqrt()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}

fn word(value: f64) -> String {
    format!("urn:qualia:rf:double:{:016x}", value.to_bits())
}
