//! OpenXML SpreadsheetML (.xlsx) reader and writer.

use crate::xml::dom::{escape_xml, parse_xml, XEl, XNode};
use crate::zip::minizip::ZipPackage;
use chronicle_tabular_engine::core::addr::{col_to_letters, letters_to_col, parse_a1_prefix, CellRef};

/// Spreadsheet table data consisting of sheet name and 2D grid of string values.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxSheet {
    pub name: String,
    pub rows: Vec<Vec<String>>,
}

/// A parsed XLSX workbook.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxWorkbook {
    pub sheets: Vec<XlsxSheet>,
}

impl XlsxWorkbook {
    pub fn new() -> Self {
        XlsxWorkbook { sheets: Vec::new() }
    }

    pub fn with_single_sheet(name: impl Into<String>, rows: Vec<Vec<String>>) -> Self {
        XlsxWorkbook {
            sheets: vec![XlsxSheet {
                name: name.into(),
                rows,
            }],
        }
    }
}

/// Serialize an `XlsxWorkbook` into an OpenXML `.xlsx` ZIP package.
pub fn to_xlsx_package(wb: &XlsxWorkbook) -> ZipPackage {
    let mut pkg = ZipPackage::new();

    // 1. [Content_Types].xml
    let mut ct = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">\n");
    ct.push_str("  <Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\n");
    ct.push_str("  <Default Extension=\"xml\" ContentType=\"application/xml\"/>\n");
    ct.push_str("  <Override PartName=\"/xl/workbook.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml\"/>\n");
    ct.push_str("  <Override PartName=\"/xl/sharedStrings.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml\"/>\n");
    for i in 1..=wb.sheets.len().max(1) {
        ct.push_str(&format!("  <Override PartName=\"/xl/worksheets/sheet{}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml\"/>\n", i));
    }
    ct.push_str("</Types>");
    pkg.insert("[Content_Types].xml", ct.into_bytes());

    // 2. _rels/.rels
    let root_rels = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n  <Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" Target=\"xl/workbook.xml\"/>\n</Relationships>";
    pkg.insert("_rels/.rels", root_rels.as_bytes().to_vec());

    // 3. xl/_rels/workbook.xml.rels
    let mut wb_rels = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n");
    let sheet_count = wb.sheets.len().max(1);
    for i in 1..=sheet_count {
        wb_rels.push_str(&format!("  <Relationship Id=\"rId{}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet\" Target=\"worksheets/sheet{}.xml\"/>\n", i, i));
    }
    wb_rels.push_str(&format!("  <Relationship Id=\"rId{}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings\" Target=\"sharedStrings.xml\"/>\n", sheet_count + 1));
    wb_rels.push_str("</Relationships>");
    pkg.insert("xl/_rels/workbook.xml.rels", wb_rels.into_bytes());

    // 4. Collect shared strings
    let mut shared_strings: Vec<String> = Vec::new();
    let mut get_string_idx = |s: &str| -> usize {
        if let Some(pos) = shared_strings.iter().position(|x| x == s) {
            pos
        } else {
            shared_strings.push(s.to_string());
            shared_strings.len() - 1
        }
    };

    // 5. xl/worksheets/sheet{i}.xml
    for (s_idx, sheet) in wb.sheets.iter().enumerate() {
        let sheet_num = s_idx + 1;
        let mut ws = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<worksheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">\n  <sheetData>\n");

        for (r_idx, row) in sheet.rows.iter().enumerate() {
            let row_num = r_idx + 1;
            ws.push_str(&format!("    <row r=\"{}\">\n", row_num));
            for (c_idx, val) in row.iter().enumerate() {
                if val.is_empty() {
                    continue;
                }
                let col_str = col_to_letters(c_idx as u32);
                let cell_ref = format!("{}{}", col_str, row_num);

                if let Ok(num) = val.parse::<f64>() {
                    if num.is_finite() && !val.starts_with('0') || val == "0" {
                        ws.push_str(&format!("      <c r=\"{}\"><v>{}</v></c>\n", cell_ref, val));
                        continue;
                    }
                }
                let s_id = get_string_idx(val);
                ws.push_str(&format!("      <c r=\"{}\" t=\"s\"><v>{}</v></c>\n", cell_ref, s_id));
            }
            ws.push_str("    </row>\n");
        }
        ws.push_str("  </sheetData>\n</worksheet>");
        pkg.insert(format!("xl/worksheets/sheet{}.xml", sheet_num), ws.into_bytes());
    }

    // 6. xl/sharedStrings.xml
    let mut sst = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<sst xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" count=\"{}\" uniqueCount=\"{}\">\n", shared_strings.len(), shared_strings.len());
    for s in &shared_strings {
        sst.push_str("  <si><t>");
        escape_xml(s, &mut sst);
        sst.push_str("</t></si>\n");
    }
    sst.push_str("</sst>");
    pkg.insert("xl/sharedStrings.xml", sst.into_bytes());

    // 7. xl/workbook.xml
    let mut wb_xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<workbook xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\">\n  <sheets>\n");
    for (i, sheet) in wb.sheets.iter().enumerate() {
        let name = if sheet.name.is_empty() { format!("Sheet{}", i + 1) } else { sheet.name.clone() };
        wb_xml.push_str(&format!("    <sheet name=\"{}\" sheetId=\"{}\" r:id=\"rId{}\"/>\n", name, i + 1, i + 1));
    }
    if wb.sheets.is_empty() {
        wb_xml.push_str("    <sheet name=\"Sheet1\" sheetId=\"1\" r:id=\"rId1\"/>\n");
    }
    wb_xml.push_str("  </sheets>\n</workbook>");
    pkg.insert("xl/workbook.xml", wb_xml.into_bytes());

    pkg
}

/// Parse an OpenXML `.xlsx` ZIP package into an `XlsxWorkbook`.
pub fn parse_xlsx_package(pkg: &ZipPackage) -> Result<XlsxWorkbook, String> {
    // 1. Parse shared strings if present
    let mut shared_strings = Vec::new();
    if let Some(sst_data) = pkg.get("xl/sharedStrings.xml") {
        if let Ok(sst_str) = String::from_utf8(sst_data.to_vec()) {
            if let Ok(sst_el) = parse_xml(&sst_str) {
                for si in sst_el.find_children("si") {
                    shared_strings.push(si.text_content());
                }
            }
        }
    }

    // 2. Parse sheet1.xml (or more)
    let mut sheets = Vec::new();
    let mut sheet_idx = 1;

    while let Some(ws_data) = pkg.get(&format!("xl/worksheets/sheet{}.xml", sheet_idx)) {
        let ws_str = String::from_utf8_lossy(ws_data);
        if let Ok(ws_el) = parse_xml(&ws_str) {
            let mut grid: Vec<Vec<String>> = Vec::new();

            if let Some(sheet_data) = ws_el.find_child("sheetData") {
                for row_el in sheet_data.find_children("row") {
                    let r_attr = row_el.attr("r").and_then(|v| v.parse::<usize>().ok()).unwrap_or(grid.len() + 1);
                    while grid.len() < r_attr {
                        grid.push(Vec::new());
                    }
                    let row_idx = r_attr - 1;

                    for c_el in row_el.find_children("c") {
                        let cell_ref_str = c_el.attr("r").unwrap_or("");
                        let col_idx = parse_a1_col(cell_ref_str);
                        let is_string = c_el.attr("t") == Some("s");

                        let val_str = if let Some(v_el) = c_el.find_child("v") {
                            let raw_v = v_el.text_content();
                            if is_string {
                                if let Ok(s_idx) = raw_v.parse::<usize>() {
                                    shared_strings.get(s_idx).cloned().unwrap_or(raw_v)
                                } else {
                                    raw_v
                                }
                            } else {
                                raw_v
                            }
                        } else if let Some(is_el) = c_el.find_child("is") {
                            is_el.text_content()
                        } else {
                            String::new()
                        };

                        while grid[row_idx].len() <= col_idx {
                            grid[row_idx].push(String::new());
                        }
                        grid[row_idx][col_idx] = val_str;
                    }
                }
            }

            sheets.push(XlsxSheet {
                name: format!("Sheet{}", sheet_idx),
                rows: grid,
            });
        }
        sheet_idx += 1;
    }

    if sheets.is_empty() {
        return Err("No worksheets found in xlsx package".into());
    }

    Ok(XlsxWorkbook { sheets })
}

fn parse_a1_col(r: &str) -> usize {
    let col_letters: String = r.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
    if col_letters.is_empty() {
        0
    } else {
        letters_to_col(&col_letters).unwrap_or(0) as usize
    }
}
