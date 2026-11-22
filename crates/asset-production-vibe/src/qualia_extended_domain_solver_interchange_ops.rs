//! WP-30 native domain, solver, and interchange production adapters.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

mod contact;
mod cooperative;
mod fem;
mod hydrology;
mod image_masks;
mod office;
mod psd;
mod rf;
mod timeline;
mod vector;

use crate::{fields, integer, list, record, required, text, Args, AssetHost, TypedArtifact, Value};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use std::sync::Arc;

impl AssetHost<'_> {
    pub(crate) fn filter_gpu_vbd_contact_and_seam_links(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        contact::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn roundtrip_psd_smart_filters_and_multi_app_brushes(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        psd::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn segment_slic_grabcut_sky_and_roto_matte(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        image_masks::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn ingest_office_smartart_xlsb_ods_and_formcalc(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        let op = text(required(args, "op")?)?;
        let result = office::evaluate(self, op, record(required(args, "parameters")?)?)?;
        if !matches!(op, "import_ods_sheet" | "read_xlsb_worksheet") {
            return Ok(result);
        }
        let Value::Record(mut output) = result else {
            return Err("Office workbook import requires a sparse cell record".into());
        };
        let rows = list(required(&output, "cells")?)?;
        self.charge(rows.len(), rows.len() * 128)?;
        let mut workbook = Workbook::new();
        let sheet = workbook.sheet_mut(0).ok_or("Workbook has no initial sheet")?;
        if let Some(name) = output.get("sheet") {
            sheet.name = text(name)?.into();
        }
        for cell in rows {
            let cell = record(cell)?;
            let row = u32::try_from(integer(required(cell, "row")?, "row")?)
                .map_err(|_| "Worksheet row exceeds capacity")?;
            let col = u32::try_from(integer(required(cell, "column")?, "column")?)
                .map_err(|_| "Worksheet column exceeds capacity")?;
            let value = match required(cell, "value")? {
                Value::Null => continue,
                Value::Bool(value) => CellValue::Bool(*value),
                Value::F64(value) if value.is_finite() => CellValue::Number(*value),
                Value::U64(value) if *value <= 1u64 << 53 => CellValue::Number(*value as f64),
                Value::I64(value) if value.unsigned_abs() <= 1u64 << 53 => CellValue::Number(*value as f64),
                Value::I64(value) => CellValue::text(value.to_string()),
                Value::U64(value) => CellValue::text(value.to_string()),
                Value::String(value) => CellValue::text(value.as_str()),
                _ => return Err("Unrepresentable value in admitted worksheet".into()),
            };
            sheet.set_value(CellRef::new(row, col), value);
        }
        let cube = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(workbook)))?;
        output.insert("cube".into(), cube);
        Ok(Value::Record(output))
    }

    pub(crate) fn solve_explicit_dynamic_fem_and_buckling(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        let op = text(required(args, "op")?)?;
        let input = required(args, "parameters")?;
        let Value::Record(mut result) = fem::evaluate(self, op, record(input)?)? else {
            return Err("Structural evaluation requires a report record".into());
        };
        let input = crate::living_world_anatomy::species_spec::vibe_value_to_json(input);
        let encoded = serde_json::to_vec(&(op, input)).map_err(|error| error.to_string())?;
        let subject = format!("urn:qualia:fem:blake3:{}", blake3::hash(&encoded).to_hex());
        let mut graph = ReceiptGraph::new("urn:qualia:fem:verification:v1");
        graph.fact(&subject, "urn:qualia:fem:solver", &format!("urn:qualia:fem:{op}"))?;
        for (key, value) in &result {
            let value = match value {
                Value::F64(value) if value.is_finite() => format!("urn:qualia:fem:double:{:016x}", value.to_bits()),
                Value::I64(value) => format!("urn:qualia:fem:integer:{value}"),
                Value::U64(value) => format!("urn:qualia:fem:integer:{value}"),
                _ => continue,
            };
            graph.fact(&subject, &format!("urn:qualia:fem:{key}"), &value)?;
        }
        let bytes = graph.seal(self.limits.resource_bytes.min(1024 * 1024))?;
        result.insert("receipt".into(), self.encode("q42", bytes)?);
        Ok(Value::Record(result))
    }

    pub(crate) fn compute_rolling_ball_concave_hull_and_offset(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        vector::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn erode_multires_rivers_and_ce_droplets(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        hydrology::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn reconcile_landed_cost_retainage_and_web_gis(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        cooperative::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn optimize_csi_privshield_and_bvp(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        rf::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn interchange_aaf_omf_mxf_and_yin_midi(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        timeline::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }
}
