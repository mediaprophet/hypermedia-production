//! OpenXML WordprocessingML (.docx) document reader and writer for FlowDoc.

use crate::flow::model::{
    FlowBlock, FlowCell, FlowDoc, FlowFmt, FlowInline, FlowKind, FlowPara, FlowRow, FlowTable,
};
use crate::xml::dom::{escape_xml, parse_xml, XEl, XNode};
use crate::zip::minizip::ZipPackage;

/// Serialize a `FlowDoc` into an OpenXML `.docx` ZIP package.
pub fn to_docx_package(doc: &FlowDoc) -> ZipPackage {
    let mut pkg = ZipPackage::new();

    // 1. [Content_Types].xml
    let ct = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">\n  <Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\n  <Default Extension=\"xml\" ContentType=\"application/xml\"/>\n  <Override PartName=\"/word/document.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml\"/>\n</Types>";
    pkg.insert("[Content_Types].xml", ct.as_bytes().to_vec());

    // 2. _rels/.rels
    let root_rels = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n  <Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" Target=\"word/document.xml\"/>\n</Relationships>";
    pkg.insert("_rels/.rels", root_rels.as_bytes().to_vec());

    // 3. word/document.xml
    let mut doc_xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">\n  <w:body>\n");

    for block in &doc.blocks {
        match block {
            FlowBlock::Para(p) => {
                doc_xml.push_str("    <w:p>\n");
                // Paragraph properties
                match p.kind {
                    FlowKind::Heading(lvl) => {
                        doc_xml.push_str(&format!("      <w:pPr><w:pStyle w:val=\"Heading{}\"/></w:pPr>\n", lvl));
                    }
                    FlowKind::Title => {
                        doc_xml.push_str("      <w:pPr><w:pStyle w:val=\"Title\"/></w:pPr>\n");
                    }
                    _ => {}
                }

                // Inlines as runs
                for inline in &p.inlines {
                    match inline {
                        FlowInline::Text(t, fmt) => {
                            doc_xml.push_str("      <w:r>\n");
                            let has_props = fmt.bold || fmt.italic || fmt.underline || fmt.strike || fmt.code;
                            if has_props {
                                doc_xml.push_str("        <w:rPr>\n");
                                if fmt.bold {
                                    doc_xml.push_str("          <w:b/>\n");
                                }
                                if fmt.italic {
                                    doc_xml.push_str("          <w:i/>\n");
                                }
                                if fmt.underline {
                                    doc_xml.push_str("          <w:u w:val=\"single\"/>\n");
                                }
                                if fmt.strike {
                                    doc_xml.push_str("          <w:strike/>\n");
                                }
                                if fmt.code {
                                    doc_xml.push_str("          <w:rFonts w:ascii=\"Courier New\" w:hAnsi=\"Courier New\"/>\n");
                                }
                                doc_xml.push_str("        </w:rPr>\n");
                            }
                            doc_xml.push_str("        <w:t xml:space=\"preserve\">");
                            escape_xml(t, &mut doc_xml);
                            doc_xml.push_str("</w:t>\n      </w:r>\n");
                        }
                        FlowInline::Math { expr, .. } => {
                            doc_xml.push_str("      <w:r><w:rPr><w:i/></w:rPr><w:t xml:space=\"preserve\">");
                            escape_xml(expr, &mut doc_xml);
                            doc_xml.push_str("</w:t></w:r>\n");
                        }
                        FlowInline::Image { alt, .. } => {
                            doc_xml.push_str("      <w:r><w:t>[Image: ");
                            escape_xml(alt, &mut doc_xml);
                            doc_xml.push_str("]</w:t></w:r>\n");
                        }
                        FlowInline::Anchor(_) => {}
                    }
                }
                doc_xml.push_str("    </w:p>\n");
            }
            FlowBlock::Table(tbl) => {
                doc_xml.push_str("    <w:tbl>\n");
                for row in &tbl.rows {
                    doc_xml.push_str("      <w:tr>\n");
                    for cell in &row.cells {
                        doc_xml.push_str("        <w:tc>\n");
                        for b in &cell.blocks {
                            if let FlowBlock::Para(cp) = b {
                                doc_xml.push_str("          <w:p><w:r><w:t>");
                                escape_xml(&cp.plain_text(), &mut doc_xml);
                                doc_xml.push_str("</w:t></w:r></w:p>\n");
                            }
                        }
                        if cell.blocks.is_empty() {
                            doc_xml.push_str("          <w:p/>\n");
                        }
                        doc_xml.push_str("        </w:tc>\n");
                    }
                    doc_xml.push_str("      </w:tr>\n");
                }
                doc_xml.push_str("    </w:tbl>\n");
            }
        }
    }

    doc_xml.push_str("  </w:body>\n</w:document>");
    pkg.insert("word/document.xml", doc_xml.into_bytes());

    pkg
}

/// Parse an OpenXML `.docx` ZIP package into a `FlowDoc`.
pub fn parse_docx_package(pkg: &ZipPackage) -> Result<FlowDoc, String> {
    let doc_bytes = pkg
        .get("word/document.xml")
        .ok_or_else(|| "Missing word/document.xml in docx package".to_string())?;

    let doc_str = String::from_utf8_lossy(doc_bytes);
    let root = parse_xml(&doc_str).map_err(|e| format!("XML parse error: {}", e))?;

    let body = root
        .find_child("body")
        .ok_or_else(|| "Missing w:body in document.xml".to_string())?;

    let mut doc = FlowDoc::new();

    for node in &body.children {
        if let XNode::Element(el) = node {
            if el.local() == "p" {
                let mut kind = FlowKind::Normal;
                if let Some(ppr) = el.find_child("pPr") {
                    if let Some(style) = ppr.find_child("pStyle") {
                        if let Some(val) = style.attr("val") {
                            if val.starts_with("Heading") {
                                if let Ok(lvl) = val.trim_start_matches("Heading").parse::<u8>() {
                                    kind = FlowKind::Heading(lvl);
                                }
                            } else if val == "Title" {
                                kind = FlowKind::Title;
                            }
                        }
                    }
                }

                let mut p = FlowPara::new(kind);
                for r_el in el.find_children("r") {
                    let mut fmt = FlowFmt::default();
                    if let Some(rpr) = r_el.find_child("rPr") {
                        fmt.bold = rpr.find_child("b").is_some();
                        fmt.italic = rpr.find_child("i").is_some();
                        fmt.underline = rpr.find_child("u").is_some();
                        fmt.strike = rpr.find_child("strike").is_some();
                    }

                    for t_el in r_el.find_children("t") {
                        let text = t_el.text_content();
                        p.inlines.push(FlowInline::Text(text, fmt.clone()));
                    }
                }
                doc.push_para(p);
            } else if el.local() == "tbl" {
                let mut rows = Vec::new();
                for tr_el in el.find_children("tr") {
                    let mut cells = Vec::new();
                    for tc_el in tr_el.find_children("tc") {
                        let mut cell_blocks = Vec::new();
                        for p_el in tc_el.find_children("p") {
                            let mut cp = FlowPara::new(FlowKind::Normal);
                            for r_el in p_el.find_children("r") {
                                for t_el in r_el.find_children("t") {
                                    cp.inlines.push(FlowInline::plain_text(t_el.text_content()));
                                }
                            }
                            cell_blocks.push(FlowBlock::Para(cp));
                        }
                        cells.push(FlowCell {
                            blocks: cell_blocks,
                            col_span: 1,
                            row_span: 1,
                        });
                    }
                    rows.push(FlowRow { cells });
                }
                let cols = rows.first().map(|r| r.cells.len()).unwrap_or(0);
                doc.push_table(FlowTable {
                    col_widths: vec![100.0; cols],
                    rows,
                });
            }
        }
    }

    Ok(doc)
}
