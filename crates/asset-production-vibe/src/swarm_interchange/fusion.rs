//! Priority-window fusion of authored samples from a static thermal fixture.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, *};
use qualia_core_db::{
    container_10d::{
        field_section::{
            FieldDescriptor, FieldRepresentationKind, FieldSupportKind, FieldTypeKind,
        },
        manifold_identity_v2::{FIELD_KIND_SCALAR, TypedFieldRecord},
    },
    q_hash,
    sensing::{
        MeasurementModality, PhysicalFieldSemantics, SemanticId,
        fusion::{
            CoordinateMapping, FusionClockStamp, FusionPolicy, FusionSample, FusionTarget,
            MAX_FUSION_CANDIDATES, SampleQuality, plan_fusion,
        },
    },
    specialized_libs::computational_geometry::authoring::fnv1a_hash,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

const FRAME_IRI: &str = "urn:qualia:sensing:fixture:static-thermal-point";
const CLOCK_IRI: &str = "urn:qualia:sensing:fixture:monotonic-nanoseconds";
const MAP_IRI: &str = "urn:qualia:sensing:fixture:identity-spatial-mapping";
const FIELD_IRI: &str = "urn:qualia:sensing:fixture:thermometer-field";
const TEMPERATURE_IRI: &str = "http://qudt.org/vocab/quantitykind/ThermodynamicTemperature";
const KELVIN_IRI: &str = "http://qudt.org/vocab/unit/K";
const METRE_IRI: &str = "http://qudt.org/vocab/unit/M";
const MODE_IRI: &str = "urn:qualia:sensing:fixture:synthetic-thermal-fusion";
const FIXTURE_DURATION_NS: u64 = 86_400_000_000_000;

struct Reading {
    sample_id: u64,
    source_id: u64,
    ticks_ns: u64,
    uncertainty_ns: u64,
    priority: i32,
    value_kelvin: f64,
    variance_kelvin_squared: f64,
}

impl AssetHost<'_> {
    pub(crate) fn bridge_etalon_nfc_radio_and_fusion(&mut self, a: &Args) -> Result<Value, String> {
        match text(required(a, "mode")?)? {
            "synthetic_thermal_fusion" => self.synthetic_thermal_fusion(a),
            "encode_nfc1_fixture" => self.encode_nfc1_fixture(a),
            "verify_pinned_nfc1_fixture_receipt" => self.verify_pinned_nfc1_fixture_receipt(a),
            _ => Err("Select synthetic_thermal_fusion or a detached NFC1 fixture mode".into()),
        }
    }

    fn synthetic_thermal_fusion(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mode", "samples", "max_samples", "max_time_span_ns"])?;
        let inputs = list(required(a, "samples")?)?;
        let maximum = exact_unsigned(a, "max_samples")?;
        let span = exact_unsigned(a, "max_time_span_ns")?;
        if !(2..=MAX_FUSION_CANDIDATES).contains(&inputs.len())
            || !(2..=inputs.len() as u64).contains(&maximum)
            || span > FIXTURE_DURATION_NS
        {
            return Err("Invalid sample count or temporal window for the thermal fixture".into());
        }
        self.charge(inputs.len().pow(3), 0)?;
        let maximum = maximum as usize;
        let mut readings = inputs
            .iter()
            .map(Reading::parse)
            .collect::<Result<Vec<_>, _>>()?;
        let mut sources = BTreeSet::new();
        if readings
            .iter()
            .any(|reading| !sources.insert(reading.source_id))
        {
            return Err("Each thermal reading must come from a distinct synthetic source".into());
        }
        readings.sort_unstable_by_key(|reading| reading.sample_id);

        let clock_id = q_hash(CLOCK_IRI);
        let frame_id = q_hash(FRAME_IRI);
        let frame = semantic_id(FRAME_IRI)?;
        let physical = thermal_semantics()?;
        let samples = readings
            .iter()
            .map(|reading| {
                let stamp = checked(FusionClockStamp::new(
                    clock_id,
                    0,
                    reading.ticks_ns,
                    reading.uncertainty_ns,
                ))?;
                checked(FusionSample::new(
                    reading.sample_id,
                    physical,
                    Some(stamp),
                    frame,
                    None,
                    SampleQuality::Unknown,
                    reading.priority,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mapping = checked(CoordinateMapping::new(
            q_hash(MAP_IRI),
            frame_id,
            frame_id,
            0.0,
            semantic_id(METRE_IRI)?,
        ))?;
        let target = checked(FusionTarget::new(clock_id, 0, frame_id))?;
        let policy = checked(FusionPolicy::new(maximum, span))?;
        let mut plan = [None; MAX_FUSION_CANDIDATES];
        let count = checked(plan_fusion(
            &samples,
            &[],
            &[mapping],
            target,
            policy,
            &mut plan,
        ))?;

        let mut precision = 0.0;
        let mut weighted_kelvin = 0.0;
        let mut lower_ns = u64::MAX;
        let mut upper_ns = 0;
        let mut selected = Vec::with_capacity(count);
        for entry in &plan[..count] {
            let entry = entry.ok_or("Thermal fusion produced an incomplete selection")?;
            let reading = &readings[entry.input_index()];
            let weight = reading.variance_kelvin_squared.recip();
            let window = entry.aligned_window();
            precision += weight;
            weighted_kelvin += reading.value_kelvin * weight;
            lower_ns = lower_ns.min(window.lower_ns());
            upper_ns = upper_ns.max(window.upper_ns());
            selected.push(json!({
                "sample_id": entry.sample_id(),
                "lower_ns": window.lower_ns(),
                "upper_ns": window.upper_ns(),
            }));
        }
        let value = weighted_kelvin / precision;
        let variance = precision.recip();
        let candidates: Vec<_> = readings.iter().map(Reading::json).collect();
        let report = serde_json::to_vec(&json!({
            "schema": "qualia.synthetic-thermal-fusion/v1",
            "clock_iri": CLOCK_IRI,
            "frame_iri": FRAME_IRI,
            "field_iri": FIELD_IRI,
            "mapping_iri": MAP_IRI,
            "quantity_iri": TEMPERATURE_IRI,
            "unit_iri": KELVIN_IRI,
            "variance_model": "independent_fixture_errors_at_one_constant_temperature",
            "max_samples": maximum,
            "max_time_span_ns": span,
            "lower_ns": lower_ns,
            "upper_ns": upper_ns,
            "candidates": candidates,
            "selected": selected,
            "value_kelvin": value,
            "variance_kelvin_squared": variance,
        }))
        .map_err(|err| err.to_string())?;
        let digest = format!("{:x}", Sha256::digest(&report));
        let subject = format!("urn:qualia:synthetic-thermal-fusion:sha256:{digest}");
        let mut graph = ReceiptGraph::new(&subject);
        graph.fact(&subject, "urn:qualia:sensing:fixtureKind", MODE_IRI)?;
        graph.fact(&subject, "urn:qualia:sensing:fixtureFrame", FRAME_IRI)?;
        graph.fact(&subject, "urn:qualia:sensing:fixtureClock", CLOCK_IRI)?;
        graph.fact(&subject, "urn:qualia:sensing:fixtureField", FIELD_IRI)?;
        graph.fact(&subject, "urn:qualia:sensing:fixtureMapping", MAP_IRI)?;
        graph.fact(
            &subject,
            "urn:qualia:sensing:fixtureQuantity",
            TEMPERATURE_IRI,
        )?;
        graph.fact(&subject, "urn:qualia:sensing:fixtureUnit", KELVIN_IRI)?;
        graph.fact(MAP_IRI, "urn:qualia:sensing:coordinateErrorUnit", METRE_IRI)?;
        let receipt = graph.seal(self.limits.resource_bytes)?;
        Ok(Value::Record(Args::from([
            ("report".into(), self.encode("json", report)?),
            ("receipt".into(), self.encode("q42", receipt)?),
            ("value_kelvin".into(), Value::F64(value)),
            ("variance_kelvin_squared".into(), Value::F64(variance)),
            ("selected_count".into(), Value::U64(count as u64)),
        ])))
    }
}

impl Reading {
    fn parse(input: &Value) -> Result<Self, String> {
        let row = record(input)?;
        fields(
            row,
            &[
                "sample_id",
                "source_id",
                "ticks_ns",
                "uncertainty_ns",
                "priority",
                "value_kelvin",
                "variance_kelvin_squared",
            ],
        )?;
        let sample_id = exact_unsigned(row, "sample_id")?;
        let source_id = exact_unsigned(row, "source_id")?;
        let ticks_ns = exact_unsigned(row, "ticks_ns")?;
        let uncertainty_ns = exact_unsigned(row, "uncertainty_ns")?;
        let priority = match required(row, "priority")? {
            Value::I64(n) => i32::try_from(*n).ok(),
            Value::U64(n) => i32::try_from(*n).ok(),
            _ => None,
        }
        .ok_or("Thermal sample priority requires an exact i32")?;
        let value_kelvin = num(required(row, "value_kelvin")?)?;
        let variance_kelvin_squared = num(required(row, "variance_kelvin_squared")?)?;
        if sample_id == 0
            || source_id == 0
            || ticks_ns < uncertainty_ns
            || ticks_ns
                .checked_add(uncertainty_ns)
                .is_none_or(|end| end > FIXTURE_DURATION_NS)
            || !(0.0..=1000.0).contains(&value_kelvin)
            || !(1e-12..=1e4).contains(&variance_kelvin_squared)
        {
            return Err(
                "Thermal sample exceeds its fixture identity, time, temperature or variance bounds"
                    .into(),
            );
        }
        Ok(Self {
            sample_id,
            source_id,
            ticks_ns,
            uncertainty_ns,
            priority,
            value_kelvin,
            variance_kelvin_squared,
        })
    }

    fn json(&self) -> serde_json::Value {
        json!({
            "sample_id": self.sample_id, "source_id": self.source_id, "ticks_ns": self.ticks_ns,
            "uncertainty_ns": self.uncertainty_ns, "priority": self.priority,
            "value_kelvin": self.value_kelvin, "variance_kelvin_squared": self.variance_kelvin_squared,
        })
    }
}

fn exact_unsigned(a: &Args, key: &str) -> Result<u64, String> {
    match required(a, key)? {
        Value::U64(n) => Ok(*n),
        Value::I64(n) => u64::try_from(*n).map_err(|_| format!("{key} must be nonnegative")),
        _ => Err(format!("{key} requires an exact integer")),
    }
}

fn semantic_id(iri: &str) -> Result<SemanticId, String> {
    SemanticId::new(q_hash(iri)).map_err(|err| format!("Invalid fixture IRI identifier: {err:?}"))
}

fn thermal_semantics() -> Result<PhysicalFieldSemantics, String> {
    let field = FieldDescriptor {
        name_hash: fnv1a_hash(FIELD_IRI.as_bytes()),
        unit_hash: fnv1a_hash(KELVIN_IRI.as_bytes()),
        type_kind: FieldTypeKind::Scalar as u8,
        support: FieldSupportKind::Point as u8,
        representation: FieldRepresentationKind::Sampled as u8,
        ..Default::default()
    };
    let typed = TypedFieldRecord {
        field_id: q_hash(FIELD_IRI),
        kind: FIELD_KIND_SCALAR,
        unit_id: q_hash(KELVIN_IRI),
        ..Default::default()
    };
    PhysicalFieldSemantics::from_existing(
        &field,
        &typed,
        MeasurementModality::Thermal,
        semantic_id(TEMPERATURE_IRI)?,
    )
    .map_err(|err| format!("Invalid thermal fixture semantics: {err:?}"))
}

fn checked<T>(
    result: Result<T, qualia_core_db::sensing::fusion::FusionError>,
) -> Result<T, String> {
    result.map_err(|err| format!("Invalid thermal fusion plan: {err:?}"))
}
