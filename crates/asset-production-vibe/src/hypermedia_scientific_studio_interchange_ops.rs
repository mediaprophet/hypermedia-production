//! Bounded scientific interchange branches that preserve canonical units.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{fields, list, num, record, required, text, Args, AssetHost, Value};
use sha2::{Digest, Sha256};
use vibescript_abi::qudt_units::validate_qudt_unit;

impl AssetHost<'_> {
    pub(crate) fn transcode_scientific_cae_bio_and_qudt(
        &mut self, args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["mode", "units"])?;
        if args.get("mode").map(text).transpose()?.unwrap_or("normalize_qudt_unit")
            != "normalize_qudt_unit"
        {
            return Err("Scientific interchange accepts normalize_qudt_unit".into());
        }
        let units = list(required(args, "units")?)?;
        if !(1..=256).contains(&units.len()) {
            return Err("QUDT normalization requires 1 to 256 unit declarations".into());
        }
        self.charge(units.len() * 1024, 1024 * 1024)?;
        let mut results = Vec::with_capacity(units.len());
        let mut graph = ReceiptGraph::new("urn:qualia:interchange:qudt:3.5.3");
        for (index, row) in units.iter().enumerate() {
            let row = record(row)?;
            fields(row, &["unit", "dimension", "conversion_multiplier"])?;
            let spelling = text(required(row, "unit")?)?;
            let dimension = text(required(row, "dimension")?)?;
            let multiplier = num(required(row, "conversion_multiplier")?)?;
            let unit = validate_qudt_unit(spelling, dimension, multiplier)
                .map_err(|error| format!("QUDT declaration at slot {index}: {error:?}"))?;
            let digest = Sha256::digest(spelling.as_bytes());
            let subject = format!("urn:qualia:interchange:qudt:declaration:{digest:x}");
            graph.fact(&subject, "http://qudt.org/schema/qudt/unit", unit.iri)?;
            graph.fact(unit.iri, "http://qudt.org/schema/qudt/hasDimensionVector", unit.dimension_iri)?;
            graph.fact(unit.iri, "urn:qualia:interchange:qudt:conversionMultiplierBits",
                &format!("urn:ieee754:binary64:{:016x}", unit.conversion_multiplier_bits))?;
            results.push(Value::Record(Args::from([
                ("unit_iri".into(), Value::String(unit.iri.into())),
                ("dimension_iri".into(), Value::String(unit.dimension_iri.into())),
                ("conversion_multiplier".into(), Value::F64(unit.conversion_multiplier())),
                ("declaration_iri".into(), Value::String(subject)),
            ])));
        }
        let receipt = self.encode("q42", graph.seal(self.limits.resource_bytes.min(1024 * 1024))?)?;
        Ok(Value::Record(Args::from([
            ("units".into(), Value::List(results.into())),
            ("receipt".into(), receipt),
        ])))
    }
}
