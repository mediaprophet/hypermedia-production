//! Finite real cellular sheaf cohomology from admitted GTOP coboundaries.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, mem::size_of, sync::Arc};

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::{
    bundle::semantic_model::resource_identity_meta,
    container_10d::{
        compute_whole_file_crc32c, Container10dHeader, TypedFieldRecord, TypedPayloadResolver,
        FIELD_KIND_GEOMETRY_TOPOLOGY_COMPLEX, MANIFOLD_IDENTITY_V2_MAX_FIELDS,
    },
    q42_volume::{payload_catalogue::decode_gtop_section, WasmSliceQ42Reader},
    q_hash,
    topology::{admit_gtop_segment_sheaf, GtopRestrictionCsrView},
};
use sha2::{Digest, Sha256};

use crate::{
    fields, integer, list, num, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    qualia_containers_governance_poet_ops::encoded_resource, required, text, Args, AssetHost,
    Resource, TypedArtifact, Value,
};

use super::{budget, count_value};

const MAX_AXIS: usize = 128;
const LEX_PAGE: usize = 1024 * 1024;
const LEX_TERM: usize = 4096;
const TABLE_SCRATCH: usize = MAX_AXIS * MAX_AXIS * size_of::<f64>();
const FRAME_SCRATCH: usize = 2 * MANIFOLD_IDENTITY_V2_MAX_FIELDS * size_of::<TypedFieldRecord>();
const SCRATCH: usize =
    LEX_PAGE + LEX_TERM + TABLE_SCRATCH + FRAME_SCRATCH + 8 * MAX_AXIS * size_of::<f64>();
const Q42_CONTEXT: &str = "urn:qualia:topology:cellular-sheaf-cohomology-v1";

macro_rules! topology {
    ($name:literal) => {
        concat!("https://qualiadb.org/ontology/cellular-sheaf#", $name)
    };
}

impl AssetHost<'_> {
    pub(crate) fn solve_cellular_sheaf_cohomology(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "incoming",
                "semantic_model",
                "geometry_iri",
                "stalk_schema_iri",
                "cohomology_degree",
                "rank_tolerance",
                "cochain",
                "arena_budget_bytes",
            ],
        )?;
        let outgoing_handle = required(args, "source")?;
        let model_handle = required(args, "semantic_model")?;
        let incoming_handle = args.get("incoming");
        let degree = integer(required(args, "cohomology_degree")?, "cohomology_degree")?;
        if degree > 3 || incoming_handle.is_some() != (degree != 0) {
            return Err(
                "Degrees 1 through 3 require an incoming coboundary and degree 0 omits it".into(),
            );
        }
        let tolerance = args
            .get("rank_tolerance")
            .map(num)
            .transpose()?
            .unwrap_or(1e-10);
        if !(1e-14..=1e-6).contains(&tolerance) {
            return Err("rank_tolerance must be in 1e-14..=1e-6".into());
        }
        let geometry = text(required(args, "geometry_iri")?)?;
        let schema = text(required(args, "stalk_schema_iri")?)?;
        let arena = budget(args, SCRATCH)?;
        let byte_work = encoded_resource(self, outgoing_handle)?
            .bytes
            .len()
            .saturating_add(encoded_resource(self, model_handle)?.bytes.len())
            .saturating_add(
                incoming_handle
                    .map(|value| encoded_resource(self, value))
                    .transpose()?
                    .map_or(0, |resource| resource.bytes.len()),
            );
        self.charge(byte_work.saturating_mul(3) + 4 * MAX_AXIS.pow(3), SCRATCH)?;

        let model = encoded_resource(self, model_handle)?;
        check_model(model, geometry, schema, arena.saturating_sub(SCRATCH))?;
        let outgoing_source = encoded_resource(self, outgoing_handle)?;
        let incoming_source = incoming_handle
            .map(|value| encoded_resource(self, value))
            .transpose()?;
        let outgoing = matrix(outgoing_source, geometry, schema)?;
        let incoming = incoming_source
            .map(|resource| matrix(resource, geometry, schema))
            .transpose()?;
        let dimension = outgoing.columns as usize;
        if incoming.is_some_and(|previous| previous.rows != outgoing.columns) {
            return Err(
                "The adjacent coboundaries disagree on their shared cochain dimension".into(),
            );
        }
        let cochain = args.get("cochain").map(list).transpose()?;
        if cochain.is_some_and(|values| values.len() != dimension) {
            return Err("Cochain length must match the selected degree".into());
        }
        let mut input = [0.0; MAX_AXIS];
        if let Some(values) = cochain {
            for (coordinate, value) in input.iter_mut().zip(values) {
                *coordinate = num(value)?;
            }
        }
        let mut scratch = vec![0.0; MAX_AXIS * MAX_AXIS];
        let outgoing_scale = scale(outgoing);
        let incoming_scale = incoming.map_or(1.0, scale);
        let composition = match incoming {
            Some(previous) => chain_residual(outgoing, previous, outgoing_scale, incoming_scale)?,
            None => 0.0,
        };
        if composition > tolerance {
            return Err(format!(
                "GTOP coboundaries fail the normalized chain condition, residual {composition}"
            ));
        }
        let outgoing_rank = rank(outgoing, outgoing_scale, tolerance, &mut scratch)?;
        let incoming_rank = incoming
            .map(|matrix| rank(matrix, incoming_scale, tolerance, &mut scratch))
            .transpose()?
            .unwrap_or(0);
        let betti = dimension
            .checked_sub(outgoing_rank + incoming_rank)
            .ok_or("Numerical ranks disagree with the cochain dimension")?;
        let mut laplacian = [0.0; MAX_AXIS];
        let mut outgoing_energy = 0.0;
        let mut incoming_energy = 0.0;
        if cochain.is_some() {
            let mut temporary = [0.0; MAX_AXIS];
            apply(outgoing, &input, &mut temporary, false)?;
            outgoing_energy = square_sum(&temporary[..outgoing.rows as usize])?;
            apply(outgoing, &temporary, &mut laplacian, true)?;
            if let Some(previous) = incoming {
                temporary.fill(0.0);
                apply(previous, &input, &mut temporary, true)?;
                incoming_energy = square_sum(&temporary[..previous.columns as usize])?;
                apply(previous, &temporary, &mut laplacian, false)?;
            }
        }
        let energy = outgoing_energy + incoming_energy;
        if !energy.is_finite() || energy > 1e12 {
            return Err("Inconsistency energy exceeds the admitted scope".into());
        }
        let model_iri = digest_iri(&model.bytes);
        let outgoing_iri = digest_iri(&outgoing_source.bytes);
        let incoming_iri = incoming_source.map(|resource| digest_iri(&resource.bytes));
        let mut digest = Sha256::new();
        digest.update(Q42_CONTEXT.as_bytes());
        digest.update((degree as u64).to_le_bytes());
        digest.update(tolerance.to_le_bytes());
        digest.update(model_iri.as_bytes());
        digest.update(outgoing_iri.as_bytes());
        digest.update(incoming_iri.as_deref().unwrap_or("").as_bytes());
        digest.update([u8::from(cochain.is_some())]);
        if cochain.is_some() {
            for value in &input[..dimension] {
                digest.update(value.to_le_bytes());
            }
        }
        let operation = format!("urn:sha256:{:x}", digest.finalize());
        let mut receipt = ReceiptGraph::new(Q42_CONTEXT);
        for (predicate, object) in [
            (topology!("geometry"), geometry),
            (topology!("stalkSchema"), schema),
            (topology!("semanticModel"), model_iri.as_str()),
            (topology!("outgoingCoboundary"), outgoing_iri.as_str()),
        ] {
            receipt.fact(&operation, predicate, object)?;
        }
        if let Some(incoming_iri) = incoming_iri.as_deref() {
            receipt.fact(&operation, topology!("incomingCoboundary"), incoming_iri)?;
        }
        for (predicate, result) in [
            (topology!("degree"), degree),
            (topology!("cochainDimension"), dimension),
            (topology!("numericalBetti"), betti),
            (topology!("outgoingRank"), outgoing_rank),
            (topology!("incomingRank"), incoming_rank),
        ] {
            receipt.fact(
                &operation,
                predicate,
                &format!("urn:qualia:integer:{result}"),
            )?;
        }
        for (predicate, result) in [
            (topology!("rankToleranceBits"), tolerance),
            (topology!("chainResidualBits"), composition),
        ] {
            receipt.fact(
                &operation,
                predicate,
                &format!("urn:qualia:binary64:{:016x}", result.to_bits()),
            )?;
        }
        if cochain.is_some() {
            receipt.fact(
                &operation,
                topology!("inconsistencyEnergyBits"),
                &format!("urn:qualia:binary64:{:016x}", energy.to_bits()),
            )?;
        }
        let graph = receipt.seal(self.limits.resource_bytes)?;

        let mut book = Workbook::new();
        let id = book.add_sheet("Cellular sheaf");
        let sheet = book
            .sheet_mut(id)
            .ok_or("Cannot create sheaf result sheet")?;
        for (row, (label, value)) in [
            ("degree", degree as f64),
            ("cochain_dimension", dimension as f64),
            ("numerical_betti", betti as f64),
            ("outgoing_rank", outgoing_rank as f64),
            ("incoming_rank", incoming_rank as f64),
            ("normalized_chain_residual", composition),
            ("rank_tolerance", tolerance),
        ]
        .into_iter()
        .enumerate()
        {
            sheet.set_value(CellRef::new(row as u32, 0), CellValue::Text(label.into()));
            sheet.set_value(CellRef::new(row as u32, 1), CellValue::Number(value));
        }
        if cochain.is_some() {
            let id = book.add_sheet("Hodge response");
            let sheet = book
                .sheet_mut(id)
                .ok_or("Cannot create Hodge result sheet")?;
            for (row, (&input, &response)) in
                input.iter().zip(&laplacian).take(dimension).enumerate()
            {
                sheet.set_value(CellRef::new(row as u32, 0), CellValue::Number(input));
                sheet.set_value(CellRef::new(row as u32, 1), CellValue::Number(response));
            }
        }
        let table = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(book)))?;
        let receipt = self.encode("q42", graph)?;
        let mut result = BTreeMap::from([
            ("table".into(), table),
            ("receipt".into(), receipt),
            ("cohomology_degree".into(), count_value(degree)),
            ("cochain_dimension".into(), count_value(dimension)),
            ("betti".into(), count_value(betti)),
            ("outgoing_rank".into(), count_value(outgoing_rank)),
            ("incoming_rank".into(), count_value(incoming_rank)),
            ("normalized_chain_residual".into(), Value::F64(composition)),
        ]);
        if cochain.is_some() {
            result.insert("inconsistency_energy".into(), Value::F64(energy));
        }
        Ok(Value::Record(result))
    }
}

fn check_model(
    resource: &Resource,
    geometry: &str,
    schema: &str,
    byte_budget: usize,
) -> Result<(), String> {
    if resource.kind != "q42" || resource.bytes.len() > byte_budget {
        return Err("The semantic model must be a bounded Q42LEX volume".into());
    }
    let reader = WasmSliceQ42Reader::open(&resource.bytes).map_err(|error| error.to_string())?;
    if !reader.is_unified_volume() {
        return Err("The semantic model requires a Q42LEX volume".into());
    }
    let mut page = vec![0; LEX_PAGE];
    let mut term = [0; LEX_TERM];
    for iri in [geometry, schema] {
        if iri.len() > LEX_TERM {
            return Err("Sheaf identity exceeds the lexical budget".into());
        }
        resource_identity_meta(iri).map_err(|error| error.to_string())?;
        let length = reader
            .lookup_lexicon_hash_into(q_hash(iri), &mut page, &mut term)
            .map_err(|error| error.to_string())?;
        if length.and_then(|length| term.get(..length)) != Some(iri.as_bytes()) {
            return Err("Sheaf identity lacks its exact Q42LEX binding".into());
        }
    }
    Ok(())
}

fn matrix<'a>(
    resource: &'a Resource,
    geometry: &str,
    schema: &str,
) -> Result<GtopRestrictionCsrView<'a>, String> {
    let gtop = match resource.kind.as_str() {
        "gtop" => decode_gtop_section(&resource.bytes)?,
        "10d" => {
            let header =
                Container10dHeader::parse(&resource.bytes).map_err(|error| format!("{error:?}"))?;
            if compute_whole_file_crc32c(&resource.bytes) != header.header_crc32c {
                return Err(
                    "The cellular sheaf 10D input has an invalid whole-file checksum".into(),
                );
            }
            let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
            let mut fields = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
            let view = TypedPayloadResolver::new(&resource.bytes, &[], &mut scratch, &mut fields)
                .map_err(|error| error.to_string())?;
            let mut gtop = view.payloads_of_kind(FIELD_KIND_GEOMETRY_TOPOLOGY_COMPLEX);
            let first = gtop
                .next()
                .ok_or("The 10D input lacks a GTOP field")?
                .map_err(|error| error.to_string())?;
            if gtop.next().is_some() {
                return Err("The selected 10D input requires exactly one GTOP field".into());
            }
            first.bytes
        }
        _ => return Err("A coboundary must be a GTOP companion or a 10D asset".into()),
    };
    let metadata = admit_gtop_segment_sheaf(gtop)
        .map_err(|error| error.to_string())?
        .ok_or("GTOP lacks a cellular sheaf footer")?;
    if metadata.geometry_id != q_hash(geometry) || metadata.stalk_schema_q_hash != q_hash(schema) {
        return Err(
            "GTOP geometry or stalk schema disagrees with the admitted semantic model".into(),
        );
    }
    let matrix = metadata
        .restrictions
        .ok_or("GTOP lacks a cellular sheaf restriction matrix")?;
    if matrix.rows as usize > MAX_AXIS || matrix.columns as usize > MAX_AXIS {
        return Err(
            "The bounded cellular sheaf solver admits at most 128 coordinates per axis".into(),
        );
    }
    Ok(matrix)
}

fn scale(matrix: GtopRestrictionCsrView<'_>) -> f64 {
    (0..matrix.rows)
        .flat_map(|row| matrix.row(row).into_iter().flatten())
        .fold(0.0_f64, |maximum, (_, value)| maximum.max(value.abs()))
        .max(f64::MIN_POSITIVE)
}

fn chain_residual(
    out: GtopRestrictionCsrView<'_>,
    inc: GtopRestrictionCsrView<'_>,
    a: f64,
    b: f64,
) -> Result<f64, String> {
    let mut maximum = 0.0_f64;
    for row in 0..out.rows {
        let mut result = [0.0_f64; MAX_AXIS];
        for (middle, left) in out.row(row).ok_or("Missing outgoing row")? {
            for (col, right) in inc.row(middle).ok_or("Missing incoming row")? {
                result[col as usize] += (left / a) * (right / b);
            }
        }
        maximum = result[..inc.columns as usize]
            .iter()
            .fold(maximum, |maximum, value| maximum.max(value.abs()));
    }
    Ok(maximum)
}

fn rank(
    matrix: GtopRestrictionCsrView<'_>,
    scale: f64,
    tolerance: f64,
    scratch: &mut [f64],
) -> Result<usize, String> {
    let (rows, cols) = (matrix.rows as usize, matrix.columns as usize);
    let a = scratch
        .get_mut(..rows * cols)
        .ok_or("Insufficient numerical rank scratch")?;
    a.fill(0.0);
    for row in 0..rows {
        for (column, coefficient) in matrix.row(row as u32).ok_or("Missing coboundary row")? {
            a[row * cols + column as usize] = coefficient / scale;
        }
    }
    let mut next = 0;
    for column in 0..cols {
        if next == rows {
            break;
        }
        let Some(pivot) = (next..rows).max_by(|&x, &y| {
            a[x * cols + column]
                .abs()
                .total_cmp(&a[y * cols + column].abs())
        }) else {
            break;
        };
        if a[pivot * cols + column].abs() <= tolerance {
            continue;
        }
        for c in column..cols {
            a.swap(next * cols + c, pivot * cols + c);
        }
        let divisor = a[next * cols + column];
        for row in next + 1..rows {
            let factor = a[row * cols + column] / divisor;
            a[row * cols + column] = 0.0;
            for c in column + 1..cols {
                let term = a[row * cols + c] - factor * a[next * cols + c];
                if !term.is_finite() {
                    return Err("Nonfinite rank arithmetic".into());
                }
                a[row * cols + c] = term;
            }
        }
        next += 1;
    }
    Ok(next)
}

fn apply(
    matrix: GtopRestrictionCsrView<'_>,
    input: &[f64],
    output: &mut [f64],
    transpose: bool,
) -> Result<(), String> {
    for row in 0..matrix.rows {
        for (col, coefficient) in matrix.row(row).ok_or("Missing Hodge row")? {
            let (out, inc) = if transpose { (col, row) } else { (row, col) };
            output[out as usize] += coefficient * input[inc as usize];
            if !output[out as usize].is_finite() {
                return Err("Nonfinite Hodge response".into());
            }
        }
    }
    Ok(())
}

fn square_sum(coordinates: &[f64]) -> Result<f64, String> {
    let energy = coordinates.iter().map(|v| v * v).sum::<f64>();
    if energy.is_finite() {
        Ok(energy)
    } else {
        Err("Nonfinite sheaf inconsistency energy".into())
    }
}

fn digest_iri(bytes: &[u8]) -> String {
    format!("urn:sha256:{:x}", Sha256::digest(bytes))
}
