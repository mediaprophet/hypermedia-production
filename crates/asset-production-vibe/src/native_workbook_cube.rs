//! Bounded native workbook recalculation and observation projection.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::*;
use chronicle_tabular_engine::{
    BinOp, Calc, CellRef, Expr, Formula, RangeRef, RefKind, SheetSel, UnOp, Value as CellValue,
    Workbook,
};
use sha2::{Digest, Sha256};

const MAX_CELLS: usize = 65_536;
const MAX_FORMULA_BYTES: usize = 512;
const MAX_FIELDS: usize = 256;
const MAX_ROWS: usize = 16_384;

impl AssetHost<'_> {
    pub(crate) fn evaluate_tabular_workbook_cube(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "formulas", "pivot", "emit_observations"])?;
        let source = required(args, "source")?;
        let mut book = if let Ok(TypedArtifact::TabularCube(book)) = self.workspace.typed(source) {
            book.as_ref().clone()
        } else {
            let resource = encoded_resource(self, source)?;
            if resource.kind != "csv" || resource.bytes.len() > 1024 * 1024 {
                return Err("Workbook source must be a typed cube or bounded CSV".into());
            }
            let size = resource.bytes.len();
            self.charge(size.saturating_mul(16), size.saturating_mul(16))?;
            let resource = encoded_resource(self, source)?;
            let csv = std::str::from_utf8(&resource.bytes).map_err(|_| "CSV requires UTF-8")?;
            csv_workbook(csv)?
        };
        let mutations = args.get("formulas").map(list).transpose()?.unwrap_or(&[]);
        let existing_cells = book
            .sheets
            .iter()
            .map(|sheet| sheet.cells.len())
            .sum::<usize>();
        if book.sheets.is_empty()
            || book.sheets.len() > 64
            || existing_cells.saturating_add(mutations.len()) > MAX_CELLS
        {
            return Err("Workbook exceeds its admitted sheet or cell count".into());
        }
        self.charge(
            (existing_cells + mutations.len()) * 128,
            (existing_cells + mutations.len()) * 640,
        )?;
        for value in mutations {
            let row = record(value)?;
            fields(row, &["sheet", "cell", "formula"])?;
            let sheet = sheet_index(&book, text(required(row, "sheet")?)?)?;
            let cell = CellRef::parse(text(required(row, "cell")?)?)
                .ok_or("Invalid formula cell address")?;
            let input = text(required(row, "formula")?)?;
            if !input.starts_with('=') || input.len() > MAX_FORMULA_BYTES {
                return Err("Formula must start with '=' and fit the formula budget".into());
            }
            book.sheet_mut(sheet)
                .ok_or("Missing workbook sheet")?
                .set_formula(cell, Formula::new(input));
        }
        if book.calc_settings.iterative {
            return Err("Workbook adapter requires acyclic or error-reporting calculation".into());
        }
        let mut computation = 0usize;
        for sheet in &book.sheets {
            for cell in sheet.cells.values() {
                if let Some(formula) = &cell.formula {
                    computation = computation.saturating_add(formula_work(formula)?);
                    if computation > 4 * MAX_CELLS {
                        return Err("Workbook exceeds the explicit formula reference budget".into());
                    }
                }
            }
        }
        self.charge(computation * 32, computation * 128)?;
        Calc::new().recalc_all(&mut book);
        if book.sheets.iter().any(|sheet| {
            sheet.cells.values().map(|cell| &cell.value).chain(sheet.spill.values())
                .any(|value| matches!(value, CellValue::Number(v) if !v.is_finite()))
        }) {
            return Err("Workbook recalculation produced a non-finite numeric cell".into());
        }
        if let Some(pivot) = args.get("pivot") {
            add_pivot(&mut book, pivot)?;
        }
        let emit = boolean(args, "emit_observations", false)?;
        let observations = if emit {
            let bytes = workbook_observations(&book, self.limits.resource_bytes)?;
            self.encode("q42", bytes)?
        } else {
            Value::Null
        };
        let cube = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(book)))?;
        Ok(Value::Record(Args::from([
            ("cube".into(), cube),
            ("observations".into(), observations),
        ])))
    }
}

fn csv_workbook(csv: &str) -> Result<Workbook, String> {
    let rows = chronicle_office_codecs::parse_csv(csv, None);
    if rows.len() > MAX_ROWS
        || rows.iter().any(|row| row.len() > MAX_FIELDS)
        || rows.iter().map(Vec::len).sum::<usize>() > MAX_CELLS
    {
        return Err("CSV exceeds its admitted row, column, or cell count".into());
    }
    let mut workbook = Workbook::new();
    let sheet = workbook
        .sheet_mut(0)
        .ok_or("Could not initialize CSV workbook")?;
    for (r, row) in rows.into_iter().enumerate() {
        for (c, value) in row.into_iter().enumerate() {
            if value.is_empty() {
                continue;
            }
            let value = value
                .parse::<f64>()
                .ok()
                .filter(|n| n.is_finite())
                .map_or_else(|| CellValue::Text(value), CellValue::Number);
            sheet.set_value(CellRef::new(r as u32, c as u32), value);
        }
    }
    Ok(workbook)
}

fn sheet_index(book: &Workbook, name: &str) -> Result<usize, String> {
    let mut matches = book
        .sheets
        .iter()
        .enumerate()
        .filter(|(_, sheet)| sheet.name.eq_ignore_ascii_case(name));
    let (index, _) = matches
        .next()
        .ok_or_else(|| format!("Unknown sheet {name}"))?;
    if matches.next().is_some() {
        return Err(format!("Ambiguous sheet {name}"));
    }
    Ok(index)
}

fn range_cells(range: RangeRef) -> usize {
    (range.end.row.abs_diff(range.start.row) as usize + 1)
        .saturating_mul(range.end.col.abs_diff(range.start.col) as usize + 1)
}

fn formula_work(formula: &Formula) -> Result<usize, String> {
    if formula.text.len() > MAX_FORMULA_BYTES {
        return Err("Workbook formula exceeds the syntax budget".into());
    }
    let expr = formula.expr().ok_or("Formula failed native parse")?;
    let mut work = 0usize;
    let mut supported = true;
    expr.walk(&mut |term| {
        work = work.saturating_add(1);
        match term {
            Expr::Number(value) => {
                supported &= value.is_finite();
            }
            Expr::Ref(reference) => {
                supported &= matches!(reference.kind, RefKind::Cell(_) | RefKind::Range(..))
                    && !matches!(reference.sheet, SheetSel::Span(..));
                work = work.saturating_add(range_cells(reference.range()));
            }
            Expr::Call(name, _) => {
                supported &= [
                    "SUM", "AVERAGE", "COUNT", "MIN", "MAX", "ABS", "ROUND", "IF", "AND", "OR",
                ]
                .iter()
                .any(|function| name.eq_ignore_ascii_case(function));
            }
            Expr::Binary(BinOp::Range | BinOp::Union | BinOp::Intersect | BinOp::Concat, ..)
            | Expr::Unary(UnOp::Spill, _)
            | Expr::Invoke(..)
            | Expr::Name(_)
            | Expr::Struct(_)
            | Expr::Array(_) => {
                supported = false;
            }
            _ => {}
        }
    });
    if !supported || work > 4096 {
        return Err(
            "Formula requires a bounded explicit range and a supported deterministic function"
                .into(),
        );
    }
    Ok(work)
}

fn add_pivot(book: &mut Workbook, pivot: &Value) -> Result<(), String> {
    let spec = record(pivot)?;
    fields(
        spec,
        &["sheet", "range", "group_column", "value_column", "name"],
    )?;
    let source = sheet_index(book, text(required(spec, "sheet")?)?)?;
    let range = RangeRef::parse(text(required(spec, "range")?)?).ok_or("Invalid pivot range")?;
    if range_cells(range) > MAX_CELLS {
        return Err("Pivot range exceeds its cell budget".into());
    }
    let group_column = integer(required(spec, "group_column")?, "group_column")?;
    let value_column = integer(required(spec, "value_column")?, "value_column")?;
    let columns = (range.end.col - range.start.col) as usize + 1;
    if group_column >= columns || value_column >= columns {
        return Err("Pivot column index outside the supplied range".into());
    }
    let input = book.sheet(source).ok_or("Missing pivot sheet")?;
    let mut groups = BTreeMap::<String, (f64, u32)>::new();
    for row in range.start.row..=range.end.row {
        let group = input.value(CellRef::new(row, range.start.col + group_column as u32));
        let value = input.value(CellRef::new(row, range.start.col + value_column as u32));
        if group.is_empty() && value.is_empty() {
            continue;
        }
        let CellValue::Number(value) = value else {
            return Err("Pivot requires numeric measure cells".into());
        };
        if group.is_empty() || group.is_error() || matches!(group, CellValue::Array(_)) {
            return Err("Pivot requires a nonempty scalar group cell".into());
        }
        let key = match group {
            CellValue::Number(n) if n.is_finite() => format!("number:{}", n),
            CellValue::Bool(b) => format!("boolean:{b}"),
            CellValue::Text(value) => format!("text:{value}"),
            _ => return Err("Invalid pivot group".into()),
        };
        let entry = groups.entry(key).or_default();
        entry.0 += value;
        entry.1 += 1;
        if !entry.0.is_finite() || groups.len() > MAX_ROWS {
            return Err("Pivot exceeds its output or numeric limit".into());
        }
    }
    let name = text(required(spec, "name")?)?;
    if name.is_empty()
        || name.chars().count() > 31
        || book
            .sheets
            .iter()
            .any(|sheet| sheet.name.eq_ignore_ascii_case(name))
    {
        return Err("Pivot sheet name must be unique and 1 to 31 characters".into());
    }
    let output = book.add_sheet(name);
    let sheet = book
        .sheet_mut(output)
        .ok_or("Could not construct pivot sheet")?;
    for (col, header) in ["group", "sum", "count", "mean"].into_iter().enumerate() {
        sheet.set_value(CellRef::new(0, col as u32), CellValue::text(header));
    }
    for (row, (group, (sum, count))) in groups.into_iter().enumerate() {
        let row = row as u32 + 1;
        for (col, value) in [
            CellValue::Text(group),
            CellValue::Number(sum),
            CellValue::Number(count.into()),
            CellValue::Number(sum / f64::from(count)),
        ]
        .into_iter()
        .enumerate()
        {
            sheet.set_value(CellRef::new(row, col as u32), value);
        }
    }
    Ok(())
}

fn workbook_observations(workbook: &Workbook, budget: usize) -> Result<Vec<u8>, String> {
    const VALUE: &str = "https://qualiadb.org/ontology/tabular#numericValue";
    const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
    const OBSERVATION: &str = "http://purl.org/linked-data/cube#Observation";
    let mut rows = Vec::new();
    let mut digest = Sha256::new();
    for (sheet_index, sheet) in workbook.sheets.iter().enumerate() {
        digest.update((sheet.name.len() as u64).to_le_bytes());
        digest.update(sheet.name.as_bytes());
        for (address, cell) in &sheet.cells {
            if let CellValue::Number(value) = cell.value {
                if !value.is_finite() || rows.len() >= 4096 {
                    return Err(
                        "Q42 observations require finite values and at most 4096 numeric cells"
                            .into(),
                    );
                }
                digest.update((sheet_index as u64).to_le_bytes());
                digest.update(address.row.to_le_bytes());
                digest.update(address.col.to_le_bytes());
                digest.update(value.to_bits().to_le_bytes());
                rows.push((sheet_index, *address, value));
            }
        }
    }
    let context = format!("urn:qualia:workbook:{:x}", digest.finalize());
    let mut graph = ReceiptGraph::new(&context);
    for (sheet, address, value) in rows {
        let cell = format!("{context}:sheet:{sheet}:cell:{}", address.a1());
        let literal = format!("\"{value:e}\"^^<http://www.w3.org/2001/XMLSchema#double>");
        graph.fact(&cell, TYPE, OBSERVATION)?;
        graph.fact(&cell, VALUE, &literal)?;
    }
    graph.seal(budget)
}
