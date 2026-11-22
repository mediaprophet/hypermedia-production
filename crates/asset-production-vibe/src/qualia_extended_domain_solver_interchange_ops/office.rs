//! Bounded OpenDocument, BIFF12 worksheet, and DrawingML geometry projections.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{fields, list, num, required, text, Args, AssetHost, Resource, Value};
use chronicle_office_codecs::{xml::dom::{parse_xml, XEl, XNode}, zip::minizip::crc32};
use std::collections::BTreeMap;

const MAX_SOURCE: usize = 8 * 1024 * 1024;
const MAX_XML: usize = 2 * 1024 * 1024;
const MAX_RECORDS: usize = 32_768;
const MAX_CELLS: usize = 8192;
const MAX_ROWS: u32 = 1_048_576;
const MAX_COLUMNS: u32 = 16_384;
const MAX_TEXT: usize = 8192;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "import_ods_sheet" => ods(host, params),
        "read_xlsb_worksheet" => xlsb(host, params),
        "read_drawingml_geometry" => drawingml(host, params),
        "render_sparkline" => sparkline(host, params),
        _ => Err(format!("Unsupported office interchange operation: {op}")),
    }
}

fn resource<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
    let source = match value {
        Value::String(key) => host.inputs.iter().find(|source| &source.key == key)
            .ok_or("Unknown retained office input")?,
        _ => host.workspace.encoded(value)?,
    };
    if source.bytes.len() > host.limits.resource_bytes.min(MAX_SOURCE) {
        return Err("Office input exceeds the bounded source extent".into());
    }
    Ok(source)
}

fn u16_at(bytes: &[u8], index: usize) -> Result<u16, String> {
    let lane = bytes.get(index..index.checked_add(2).ok_or("Office offset overflow")?)
        .ok_or("Truncated office u16 field")?;
    Ok(u16::from_le_bytes(lane.try_into().expect("two-byte field")))
}

fn u32_at(bytes: &[u8], index: usize) -> Result<u32, String> {
    let lane = bytes.get(index..index.checked_add(4).ok_or("Office offset overflow")?)
        .ok_or("Truncated office u32 field")?;
    Ok(u32::from_le_bytes(lane.try_into().expect("four-byte field")))
}

fn stored_member<'a>(bytes: &'a [u8], wanted: &str) -> Result<Option<&'a [u8]>, String> {
    let eocd = (bytes.len().saturating_sub(65_557)..bytes.len().saturating_sub(21)).rev().find(|&offset| {
        let tail = &bytes[offset..];
        tail.starts_with(b"PK\x05\x06") && u16_at(tail, 20).is_ok_and(|n| tail.len() == usize::from(n) + 22)
    }).ok_or("Invalid office ZIP directory trailer")?;
    let tail = &bytes[eocd..];
    let count = usize::from(u16_at(tail, 10)?);
    let size = u32_at(tail, 12)? as usize;
    let offset = u32_at(tail, 16)? as usize;
    if u32_at(tail, 4)? != 0 || u16_at(tail, 8)? as usize != count
        || count > 1024 || offset.checked_add(size) != Some(eocd)
    {
        return Err("Office ZIP requires a bounded single-disk directory".into());
    }
    let mut directory = &bytes[offset..eocd];
    let mut selected = None;
    for _ in 0..count {
        if !directory.starts_with(b"PK\x01\x02") || directory.len() < 46 {
            return Err("Malformed office ZIP central entry".into());
        }
        let n = usize::from(u16_at(directory, 28)?);
        let extra = usize::from(u16_at(directory, 30)?);
        let comment = usize::from(u16_at(directory, 32)?);
        let extent = 46 + n + extra + comment;
        let entry = directory.get(..extent).ok_or("Truncated office ZIP central entry")?;
        if &entry[46..46 + n] == wanted.as_bytes() {
            if selected.is_some() || u16_at(entry, 34)? != 0 {
                return Err("Ambiguous office ZIP member".into());
            }
            let flags = u16_at(entry, 8)?;
            let method = u16_at(entry, 10)?;
            if flags & !0x0800 != 0 || method != 0 || u32_at(entry, 20)? != u32_at(entry, 24)? {
                return Err("Office ZIP extraction requires a stored member or a retained unpacked part".into());
            }
            let local = u32_at(entry, 42)? as usize;
            let local = bytes.get(local..offset).ok_or("Office ZIP local header outside data region")?;
            if !local.starts_with(b"PK\x03\x04") || u16_at(local, 6)? != flags
                || u16_at(local, 8)? != method || u16_at(local, 26)? as usize != n
                || local.get(30..30 + n) != Some(wanted.as_bytes())
                || u32_at(local, 14)? != u32_at(entry, 16)?
                || u32_at(local, 18)? != u32_at(entry, 20)?
                || u32_at(local, 22)? != u32_at(entry, 24)?
            {
                return Err("Office ZIP local entry disagrees with its directory".into());
            }
            let start = 30 + n + usize::from(u16_at(local, 28)?);
            let end = start.checked_add(u32_at(entry, 24)? as usize).ok_or("Office ZIP member extent overflow")?;
            let data = local.get(start..end).ok_or("Office ZIP member is truncated")?;
            if crc32(data) != u32_at(entry, 16)? {
                return Err("Office ZIP member failed CRC-32 verification".into());
            }
            selected = Some(data);
        }
        directory = &directory[extent..];
    }
    if !directory.is_empty() {
        return Err("Office ZIP directory count disagrees with its extent".into());
    }
    Ok(selected)
}

fn xml(bytes: &[u8]) -> Result<XEl, String> {
    let input = std::str::from_utf8(bytes).map_err(|_| "Office XML must be UTF-8")?;
    let input = input.strip_prefix('\u{feff}').unwrap_or(input);
    if input.len() > MAX_XML || input.contains("<!") || input.contains("&#")
        || input.bytes().filter(|&byte| byte == b'<').take(MAX_RECORDS + 1).count() > MAX_RECORDS
    {
        return Err("Office XML exceeds the supported lexical profile".into());
    }
    let mut depth = 0usize;
    let mut roots = 0;
    let mut remaining = input.trim();
    while let Some(start) = remaining.find('<') {
        let prefix = &remaining[..start];
        if depth == 0 && !prefix.trim().is_empty() {
            return Err("Office XML has text outside its root element".into());
        }
        remaining = &remaining[start + 1..];
        let Some(end) = remaining.find('>') else {
            return Err("Office XML tag is unterminated".into());
        };
        let token = &remaining[..end];
        remaining = &remaining[end + 1..];
        if token.contains('<') || token.is_empty() {
            return Err("Malformed office XML tag".into());
        }
        if token.starts_with('?') {
            if depth != 0 || roots != 0 || !token.starts_with("?xml ") || !token.ends_with('?') {
                return Err("Unsupported office XML processing instruction".into());
            }
        } else if token.starts_with('/') {
            depth = depth.checked_sub(1).ok_or("Unmatched office XML closing element")?;
        } else {
            roots += usize::from(depth == 0);
            if !token.ends_with('/') {
                depth += 1;
            }
            if depth > 40 || roots > 1 {
                return Err("Office XML exceeds its bounded element structure".into());
            }
        }
    }
    if depth != 0 || roots != 1 || !remaining.trim().is_empty() {
        return Err("Incomplete office XML document".into());
    }
    parse_xml(input).map_err(|error| error.to_string())
}

fn attr<'a>(node: &'a XEl, name: &str) -> Option<&'a str> {
    node.attrs.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
}

fn children<'a>(node: &'a XEl, name: &'a str) -> impl Iterator<Item = &'a XEl> {
    node.children.iter().filter_map(move |node| match node {
        XNode::Element(el) if el.name == name => Some(el),
        _ => None,
    })
}

fn repeat(node: &XEl, key: &str) -> Result<u32, String> {
    match attr(node, key) {
        None => Ok(1),
        Some(value) => value.parse::<u32>().ok().filter(|&value| value > 0)
            .ok_or_else(|| "Invalid OpenDocument repetition count".into()),
    }
}

fn finite(text: &str) -> Result<f64, String> {
    text.parse::<f64>().ok().filter(|value| value.is_finite())
        .ok_or_else(|| "Office formula cache or numeric cell is not finite".into())
}

fn bounded_text(text: &str) -> Result<String, String> {
    if text.len() > MAX_TEXT {
        return Err("Office cell text exceeds the bounded text extent".into());
    }
    Ok(text.to_owned())
}

fn odf_value(cell: &XEl) -> Result<Option<Args>, String> {
    let kind = attr(cell, "office:value-type").unwrap_or_default();
    let value = |key| attr(cell, key).ok_or("OpenDocument typed cell is missing its value");
    let formula = attr(cell, "table:formula");
    let data = match kind {
        "float" | "currency" | "percentage" => Value::F64(finite(value("office:value")?)?),
        "boolean" => match value("office:boolean-value")? {
            "true" | "1" => Value::Bool(true),
            "false" | "0" => Value::Bool(false),
            _ => return Err("Invalid OpenDocument boolean cell".into()),
        },
        "date" => Value::String(bounded_text(value("office:date-value")?)?),
        "time" => Value::String(bounded_text(value("office:time-value")?)?),
        "string" => {
            let content = attr(cell, "office:string-value").map(str::to_owned).unwrap_or_else(||
                children(cell, "text:p").map(XEl::text_content).collect::<Vec<_>>().join("\n"));
            Value::String(bounded_text(&content)?)
        }
        "" if formula.is_some() => Value::Null,
        "" if cell.children.iter().any(|child| matches!(child, XNode::Element(el) if el.name == "text:p")) =>
            return Err("OpenDocument textual data lacks a value type".into()),
        "" | "void" => return Ok(None),
        _ => return Err("OpenDocument cell value type is unsupported".into()),
    };
    let mut row = BTreeMap::from([("value".into(), data), ("value_type".into(), Value::String(kind.into()))]);
    if let Some(formula) = formula {
        row.insert("formula".into(), Value::String(bounded_text(formula)?));
    }
    if kind == "currency" {
        row.insert("currency".into(), Value::String(bounded_text(value("office:currency")?)?));
    }
    Ok(Some(row))
}

fn ods(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["source", "sheet"])?;
    let input = required(params, "source")?;
    let name = text(required(params, "sheet")?)?;
    let size = resource(host, input)?.bytes.len();
    host.charge(size, size.min(MAX_XML).saturating_mul(16))?;
    let input = resource(host, input)?;
    let bytes = match input.kind.as_str() {
        "ods" => {
            if stored_member(&input.bytes, "mimetype")? != Some(b"application/vnd.oasis.opendocument.spreadsheet".as_slice()) {
                return Err("ODS package mimetype is missing or incorrect".into());
            }
            stored_member(&input.bytes, "content.xml")?.ok_or("ODS package lacks content.xml")?
        }
        "ods-xml" => input.bytes.as_slice(),
        _ => return Err("ODS import expects retained ods or unpacked ods-xml data".into()),
    };
    let root = xml(bytes)?;
    if !matches!(root.name.as_str(), "office:document" | "office:document-content")
        || attr(&root, "xmlns:office") != Some("urn:oasis:names:tc:opendocument:xmlns:office:1.0")
        || attr(&root, "xmlns:table") != Some("urn:oasis:names:tc:opendocument:xmlns:table:1.0")
        || attr(&root, "xmlns:text") != Some("urn:oasis:names:tc:opendocument:xmlns:text:1.0")
    {
        return Err("Unsupported OpenDocument spreadsheet root or namespace bindings".into());
    }
    let body = children(&root, "office:body").next().ok_or("ODS document lacks office:body")?;
    let document = children(body, "office:spreadsheet").next().ok_or("ODS document lacks office:spreadsheet")?;
    let mut matching = children(document, "table:table").filter(|table| attr(table, "table:name") == Some(name));
    let table = matching.next().ok_or("Named ODS sheet was not found")?;
    if matching.next().is_some() {
        return Err("Named ODS sheet is ambiguous".into());
    }
    let mut rows = Vec::new();
    collect_odf_rows(table, &mut rows)?;
    let mut output = Vec::new();
    let (mut height, mut width) = (0u32, 0u32);
    let mut text_bytes = 0usize;
    for row in rows {
        let row_repeat = repeat(row, "table:number-rows-repeated")?;
        let next_height = height.checked_add(row_repeat).filter(|&value| value <= MAX_ROWS)
            .ok_or("ODS sheet exceeds its row-extent budget")?;
        let mut column = 0u32;
        for child in &row.children {
            let XNode::Element(cell) = child else { continue; };
            if !matches!(cell.name.as_str(), "table:table-cell" | "table:covered-table-cell") {
                return Err("Unsupported element in an ODS spreadsheet row".into());
            }
            let repeated = repeat(cell, "table:number-columns-repeated")?;
            let next_column = column.checked_add(repeated).filter(|&value| value <= MAX_COLUMNS)
                .ok_or("ODS sheet exceeds its column-extent budget")?;
            if let Some(data) = odf_value(cell)? {
                let expanded = u64::from(row_repeat) * u64::from(repeated);
                if expanded > (MAX_CELLS - output.len()) as u64 {
                    return Err("OpenDocument repeated cell expansion exceeds the cell budget".into());
                }
                let added_text = data.values().filter_map(|value| match value {
                    Value::String(string) => Some(string.len()),
                    _ => None,
                }).sum::<usize>().saturating_mul(expanded as usize);
                text_bytes = text_bytes.saturating_add(added_text);
                if text_bytes > MAX_XML {
                    return Err("OpenDocument repeated cell expansion exceeds the text budget".into());
                }
                for r in height..next_height {
                    for c in column..next_column {
                        let mut item = data.clone();
                        item.insert("row".into(), Value::U64(u64::from(r)));
                        item.insert("column".into(), Value::U64(u64::from(c)));
                        if cell.name == "table:covered-table-cell" {
                            item.insert("covered".into(), Value::Bool(true));
                        }
                        output.push(Value::Record(item));
                    }
                }
            }
            column = next_column;
        }
        width = width.max(column);
        height = next_height;
    }
    output.sort_by_key(|cell| {
        if let Value::Record(fields) = cell {
            if let (Some(Value::U64(row)), Some(Value::U64(column))) = (fields.get("row"), fields.get("column")) {
                return (*row, *column);
            }
        }
        unreachable!("assembled ODS cell coordinates")
    });
    Ok(Value::Record(BTreeMap::from([
        ("sheet".into(), Value::String(name.into())),
        ("row_count".into(), Value::U64(u64::from(height))),
        ("column_count".into(), Value::U64(u64::from(width))),
        ("cells".into(), Value::List(output)),
    ])))
}

fn collect_odf_rows<'a>(node: &'a XEl, out: &mut Vec<&'a XEl>) -> Result<(), String> {
    for child in &node.children {
        if let XNode::Element(child) = child {
            match child.name.as_str() {
                "table:table-row" => out.push(child),
                "table:table-row-group" | "table:table-header-rows" | "table:table-rows" =>
                    collect_odf_rows(child, out)?,
                _ => {}
            }
        }
    }
    Ok(())
}

fn next_biff<'a>(remaining: &mut &'a [u8]) -> Result<(u16, &'a [u8]), String> {
    let data = *remaining;
    let mut bytes = data.iter().copied();
    let first = bytes.next().ok_or("BIFF12 record is missing its type")?;
    let mut kind = u16::from(first & 127);
    let mut header = 1;
    if first & 128 != 0 {
        let high = bytes.next().ok_or("Truncated BIFF12 record type")?;
        if high & 127 == 0 {
            return Err("BIFF12 record type encoding is nonminimal".into());
        }
        kind |= u16::from(high & 127) << 7;
        header += 1;
    }
    let mut size = 0usize;
    for i in 0..4 {
        let lane = bytes.next().ok_or("Truncated BIFF12 record size")?;
        header += 1;
        size |= usize::from(lane & 127) << (7 * i);
        if lane & 128 == 0 || i == 3 {
            break;
        }
    }
    let end = header + size;
    let body = data.get(header..end).ok_or("Truncated BIFF12 record data")?;
    *remaining = &data[end..];
    Ok((kind, body))
}

fn biff_string(body: &[u8]) -> Result<String, String> {
    let len = u32_at(body, 0)? as usize;
    if len > MAX_TEXT / 2 || body.len() != 4 + len * 2 {
        return Err("BIFF12 string extent is outside the supported profile".into());
    }
    let utf16 = body[4..].chunks_exact(2).map(|unit|
        u16::from_le_bytes(unit.try_into().expect("two-byte UTF-16 unit"))).collect::<Vec<_>>();
    String::from_utf16(&utf16).map_err(|_| "Invalid UTF-16 in BIFF12 string".into())
}

fn biff_sst(mut bytes: &[u8]) -> Result<Vec<String>, String> {
    let (start, header) = next_biff(&mut bytes)?;
    if start != 159 || header.len() != 8 {
        return Err("BIFF12 shared strings require a BrtBeginSst header".into());
    }
    let count = u32_at(header, 4)? as usize;
    if count > MAX_CELLS || u32_at(header, 0)? < count as u32 {
        return Err("BIFF12 shared-string count exceeds the supported extent".into());
    }
    let mut output = Vec::with_capacity(count);
    for _ in 0..count {
        let (kind, body) = next_biff(&mut bytes)?;
        if kind != 19 || body.first() != Some(&0) {
            return Err("BIFF12 shared-string import requires plain BrtSSTItem entries".into());
        }
        output.push(biff_string(&body[1..])?);
    }
    let (end, record) = next_biff(&mut bytes)?;
    if end != 160 || !record.is_empty() || !bytes.is_empty() {
        return Err("BIFF12 shared-string table has an invalid trailer".into());
    }
    Ok(output)
}

fn biff_cell(kind: u16, body: &[u8], strings: &[String]) -> Result<(Value, &'static str), String> {
    let value = body.get(8..).ok_or("Truncated BIFF12 cell header")?;
    match (kind, value.len()) {
        (1, 0) => Ok((Value::Null, "blank")),
        (2, 4) => {
            let rk = u32_at(value, 0)?;
            let number = if rk & 2 != 0 {
                f64::from((rk as i32) >> 2)
            } else {
                f64::from_bits(u64::from(rk & !3) << 32)
            };
            let number = if rk & 1 != 0 { number / 100.0 } else { number };
            if !number.is_finite() {
                return Err("BIFF12 RK cell is non-finite".into());
            }
            Ok((Value::F64(number), "number"))
        }
        (3, 1) => Ok((Value::U64(u64::from(value[0])), "error")),
        (4, 1) if value[0] <= 1 => Ok((Value::Bool(value[0] != 0), "boolean")),
        (5, 8) => {
            let number = f64::from_le_bytes(value.try_into().expect("eight-byte BIFF12 real"));
            if !number.is_finite() {
                return Err("BIFF12 real cell is non-finite".into());
            }
            Ok((Value::F64(number), "number"))
        }
        (6, _) => Ok((Value::String(biff_string(value)?), "string")),
        (7, 4) => Ok((Value::String(strings.get(u32_at(value, 0)? as usize)
            .ok_or("BIFF12 shared-string reference is unresolved")?.clone()), "string")),
        _ => Err("Unsupported BIFF12 cell representation or length".into()),
    }
}

fn biff_row_spans(body: &[u8], spans: &mut Vec<(u32, u32)>) -> Result<(), String> {
    let count = u32_at(body, 13)? as usize;
    if count > 16 || body.len() != 17 + count * 8 {
        return Err("BIFF12 row header has an invalid column-span extent".into());
    }
    spans.clear();
    for bytes in body[17..].chunks_exact(8) {
        let first = u32_at(bytes, 0)?;
        let last = u32_at(bytes, 4)?;
        if first > last || last >= MAX_COLUMNS || first / 1024 != last / 1024
            || spans.last().is_some_and(|&(_, prior)| prior >= first)
        {
            return Err("BIFF12 row header contains invalid column spans".into());
        }
        spans.push((first, last));
    }
    Ok(())
}

fn xlsb(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["source", "part", "shared_strings"])?;
    let input = required(params, "source")?;
    let shared = params.get("shared_strings");
    let size = resource(host, input)?.bytes.len();
    let shared_size = shared.map(|value| resource(host, value).map(|source| source.bytes.len()))
        .transpose()?.unwrap_or_default();
    host.charge((size + shared_size).saturating_mul(3), size + shared_size + 4 * MAX_XML)?;
    let input = resource(host, input)?;
    let (mut bytes, mut inline_sst) = match input.kind.as_str() {
        "xlsb-sheet" if params.get("part").is_none() => (input.bytes.as_slice(), None),
        "xlsb" => {
            let part = text(required(params, "part")?)?;
            let number = part.strip_prefix("xl/worksheets/sheet").and_then(|name| name.strip_suffix(".bin"))
                .and_then(|number| number.parse::<u32>().ok()).filter(|&number| number > 0)
                .ok_or("XLSB part must identify an exact xl/worksheets/sheetN.bin member")?;
            if part != format!("xl/worksheets/sheet{number}.bin") {
                return Err("XLSB worksheet member path is noncanonical".into());
            }
            (stored_member(&input.bytes, part)?.ok_or("XLSB package lacks the selected worksheet member")?,
                if shared.is_none() { stored_member(&input.bytes, "xl/sharedStrings.bin")? } else { None })
        }
        _ => return Err("XLSB import expects retained xlsb or unpacked xlsb-sheet data".into()),
    };
    if let Some(shared) = shared {
        let source = resource(host, shared)?;
        if source.kind != "xlsb-sst" {
            return Err("Shared strings require retained xlsb-sst data".into());
        }
        inline_sst = Some(&source.bytes);
    }
    let strings = inline_sst.map(biff_sst).transpose()?.unwrap_or_default();
    let (begin, body) = next_biff(&mut bytes)?;
    if begin != 129 || !body.is_empty() {
        return Err("XLSB worksheet is missing its BrtBeginSheet record".into());
    }
    let mut rows = Vec::new();
    let (mut active, mut closed, mut end_sheet) = (false, false, false);
    let (mut row, mut column) = (None, None);
    let mut spans = Vec::with_capacity(16);
    let mut text_bytes = 0usize;
    let mut scanned = 0usize;
    while !bytes.is_empty() {
        scanned += 1;
        if scanned > MAX_RECORDS || end_sheet {
            return Err("XLSB worksheet exceeds its record budget or has a trailing record".into());
        }
        let (kind, body) = next_biff(&mut bytes)?;
        match kind {
            129 => return Err("Repeated BrtBeginSheet record".into()),
            130 if !active && closed && body.is_empty() => end_sheet = true,
            145 if !active && !closed && body.is_empty() => active = true,
            146 if active && body.is_empty() => { active = false; closed = true; }
            130 | 145 | 146 => return Err("BIFF12 worksheet framing is invalid".into()),
            0 if active => {
                let index = u32_at(body, 0)?;
                if index >= MAX_ROWS || row.is_some_and(|old| index <= old) {
                    return Err("BIFF12 row coordinate is duplicated, unordered or outside the sheet".into());
                }
                biff_row_spans(body, &mut spans)?;
                row = Some(index);
                column = None;
            }
            1..=7 if active => {
                let row = row.ok_or("BIFF12 cell appears before its row header")?;
                let index = u32_at(body, 0)?;
                if index >= MAX_COLUMNS || column.is_some_and(|old| index <= old) || rows.len() >= MAX_CELLS
                    || !spans.iter().any(|&(first, last)| (first..=last).contains(&index))
                {
                    return Err("BIFF12 cell coordinate or count exceeds the supported extent".into());
                }
                column = Some(index);
                let (value, kind) = biff_cell(kind, body, &strings)?;
                if let Value::String(value) = &value {
                    text_bytes = text_bytes.saturating_add(value.len());
                    if text_bytes > MAX_XML {
                        return Err("BIFF12 cell output exceeds the text budget".into());
                    }
                }
                rows.push(Value::Record(BTreeMap::from([
                    ("row".into(), Value::U64(u64::from(row))),
                    ("column".into(), Value::U64(u64::from(index))),
                    ("style_index".into(), Value::U64(u64::from(u32_at(body, 4)? & 0xff_ffff))),
                    ("value_type".into(), Value::String(kind.into())),
                    ("value".into(), value),
                ])));
            }
            _ if active => return Err(format!("Unsupported BIFF12 record {kind} in sheet data")),
            0..=11 | 62 => return Err("BIFF12 row or cell record appears outside sheet data".into()),
            _ => {}
        }
    }
    if !end_sheet {
        return Err("XLSB worksheet is missing its BrtEndSheet record".into());
    }
    Ok(Value::Record(BTreeMap::from([
        ("record_count".into(), Value::U64((scanned + 1) as u64)),
        ("cells".into(), Value::List(rows)),
    ])))
}

fn drawingml(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["source"])?;
    let input = required(params, "source")?;
    let size = resource(host, input)?.bytes.len();
    host.charge(size, size.min(MAX_XML).saturating_mul(16))?;
    let input = resource(host, input)?;
    if input.kind != "drawingml-xml" {
        return Err("Expected retained DrawingML or cached SmartArt drawing XML".into());
    }
    let root = xml(&input.bytes)?;
    if attr(&root, "xmlns:a") != Some("http://schemas.openxmlformats.org/drawingml/2006/main")
        || !matches!(root.name.as_str(), "a:custGeom" | "dsp:drawing")
    {
        return Err("Expected an a:custGeom or dsp:drawing root with DrawingML namespace binding".into());
    }
    if root.name == "dsp:drawing"
        && attr(&root, "xmlns:dsp") != Some("http://schemas.microsoft.com/office/drawing/2008/diagram")
    {
        return Err("Cached SmartArt drawing namespace binding is invalid".into());
    }
    let mut stack = vec![&root];
    let mut paths = Vec::new();
    let mut commands = 0;
    while let Some(node) = stack.pop() {
        if node.name != "a:custGeom" {
            stack.extend(node.children.iter().rev().filter_map(|child| match child {
                XNode::Element(child) => Some(child),
                _ => None,
            }));
            continue;
        }
        let list = children(node, "a:pathLst").next().ok_or("DrawingML custGeom lacks a:pathLst")?;
        for path in children(list, "a:path") {
            let mut steps = Vec::new();
            let mut started = false;
            for command in &path.children {
                let XNode::Element(command) = command else { continue; };
                commands += 1;
                if commands > MAX_CELLS {
                    return Err("DrawingML geometry exceeds the path-command budget".into());
                }
                let (op, count) = match command.name.as_str() {
                    "a:moveTo" => { started = true; ("move", 1) }
                    "a:lnTo" if started => ("line", 1),
                    "a:quadBezTo" if started => ("quadratic", 2),
                    "a:cubicBezTo" if started => ("cubic", 3),
                    "a:close" if started => { started = false; ("close", 0) }
                    _ => return Err("Unsupported or misplaced DrawingML path command".into()),
                };
                let points = children(command, "a:pt").map(|point| {
                    let coordinate = |axis| attr(point, axis).and_then(|value| value.parse::<i64>().ok())
                        .filter(|value| value.unsigned_abs() <= 1_000_000_000)
                        .ok_or("DrawingML path requires bounded literal coordinates");
                    Ok(Value::List(vec![Value::I64(coordinate("x")?), Value::I64(coordinate("y")?)]))
                }).collect::<Result<Vec<_>, String>>()?;
                if points.len() != count {
                    return Err("DrawingML path command has an invalid point count".into());
                }
                steps.push(Value::Record(BTreeMap::from([
                    ("op".into(), Value::String(op.into())),
                    ("points".into(), Value::List(points)),
                ])));
            }
            if steps.is_empty() || paths.len() >= MAX_CELLS {
                return Err("DrawingML path count or size exceeds the supported extent".into());
            }
            let dim = |key| attr(path, key).map(|value| value.parse::<u64>().ok()
                .filter(|&n| n <= 1_000_000_000).map(Value::U64).ok_or("Invalid DrawingML path extent"))
                .transpose().map(|value| value.unwrap_or(Value::Null));
            paths.push(Value::Record(BTreeMap::from([
                ("width".into(), dim("w")?),
                ("height".into(), dim("h")?),
                ("commands".into(), Value::List(steps)),
            ])));
        }
    }
    if paths.is_empty() {
        return Err("DrawingML source has no supported literal custom-geometry paths".into());
    }
    Ok(Value::Record(BTreeMap::from([
        ("source_root".into(), Value::String(root.name)),
        ("coordinate_space".into(), Value::String("path_local".into())),
        ("paths".into(), Value::List(paths)),
    ])))
}

fn sparkline(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    use std::fmt::Write;
    fields(params, &["values"])?;
    let values = list(required(params, "values")?)?;
    if !(2..=1024).contains(&values.len()) {
        return Err("Sparkline requires 2–1024 finite points".into());
    }
    host.charge(values.len(), values.len() * 32)?;
    let values = values.iter().map(num).collect::<Result<Vec<_>, _>>()?;
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = max - min;
    if !span.is_finite() {
        return Err("Sparkline value span exceeds finite range".into());
    }
    let mut svg = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"160\" height=\"42\" viewBox=\"0 0 160 42\"><polyline fill=\"none\" stroke=\"#285f8f\" stroke-width=\"1.5\" points=\"");
    for (index, value) in values.iter().enumerate() {
        let x = 2.0 + 156.0 * index as f64 / (values.len() - 1) as f64;
        let y = if span == 0.0 { 21.0 } else { 40.0 - 38.0 * ((value - min) / span) };
        let _ = write!(svg, "{x:.3},{y:.3} ");
    }
    svg.push_str("\"/></svg>");
    host.encode("svg", svg.into_bytes())
}
