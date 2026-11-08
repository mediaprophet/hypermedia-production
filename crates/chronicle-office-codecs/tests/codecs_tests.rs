use chronicle_office_codecs::*;

#[test]
fn test_zip_archive_roundtrip() {
    let mut pkg = ZipPackage::new();
    pkg.insert("hello.txt", b"Hello, World!".to_vec());
    pkg.insert("nested/doc.xml", b"<xml><val>42</val></xml>".to_vec());

    let bytes = pkg.write_to_bytes();
    assert!(bytes.len() > 100);

    let parsed = ZipPackage::parse(&bytes).expect("zip should parse cleanly");
    assert_eq!(parsed.get("hello.txt"), Some(&b"Hello, World!"[..]));
    assert_eq!(parsed.get_str("nested/doc.xml"), Some("<xml><val>42</val></xml>".into()));
}

#[test]
fn test_xml_dom_parser_and_writer() {
    let xml_str = r#"<w:document xmlns:w="http://schemas.org/word"><w:body><w:p><w:r><w:t>Hello &amp; welcome</w:t></w:r></w:p></w:body></w:document>"#;
    let root = parse_xml(xml_str).expect("xml should parse");
    assert_eq!(root.local(), "document");
    assert_eq!(root.attr("xmlns:w"), Some("http://schemas.org/word"));

    let body = root.find_child("body").expect("should find body");
    let p = body.find_child("p").expect("should find p");
    let r = p.find_child("r").expect("should find r");
    let t = r.find_child("t").expect("should find t");
    assert_eq!(t.text_content(), "Hello & welcome");

    let written = root.to_xml_string();
    assert!(written.contains("<w:t>Hello &amp; welcome</w:t>"));
}

#[test]
fn test_markdown_codecs() {
    let md = r#"# Document Title

This is a **bold** and *italic* paragraph with `code` and inline math $E = mc^2$.

$$
\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
$$

| Header A | Header B |
| --- | --- |
| Cell 1 | Cell 2 |

- List item 1
- List item 2
"#;

    let doc = parse_markdown(md);
    assert_eq!(doc.blocks.len(), 5);

    let serialized = to_markdown(&doc);
    assert!(serialized.contains("# Document Title"));
    assert!(serialized.contains("**bold**"));
    assert!(serialized.contains("$E = mc^2$"));
    assert!(serialized.contains("| Header A | Header B |"));
}

#[test]
fn test_html_codecs() {
    let html = r#"<html><body><h1>Main Heading</h1><p>A paragraph with <strong>bold</strong> text.</p><table><tr><td>Row 1</td></tr></table></body></html>"#;
    let doc = parse_html(html).expect("html should parse");
    assert_eq!(doc.blocks.len(), 3);

    let html_out = to_html(&doc);
    assert!(html_out.contains("<h1>Main Heading</h1>"));
    assert!(html_out.contains("<strong>bold</strong>"));
    assert!(html_out.contains("<table>"));
}

#[test]
fn test_rtf_codecs() {
    let mut doc = FlowDoc::new();
    let mut p = FlowPara::new(FlowKind::Heading(1));
    p.inlines.push(FlowInline::plain_text("RTF Heading"));
    doc.push_para(p);

    let mut p2 = FlowPara::new(FlowKind::Normal);
    p2.inlines.push(FlowInline::Text("Bold text".into(), FlowFmt { bold: true, ..Default::default() }));
    doc.push_para(p2);

    let rtf_str = to_rtf(&doc);
    assert!(rtf_str.starts_with("{\\rtf1"));
    assert!(rtf_str.contains("\\b Bold text"));

    let parsed_doc = parse_rtf(&rtf_str).expect("rtf should parse");
    assert!(!parsed_doc.blocks.is_empty());
}

#[test]
fn test_latex_codecs() {
    let mut doc = FlowDoc::new();
    doc.title = Some("LaTeX Report".into());
    doc.push_para(FlowPara::with_text(FlowKind::Heading(1), "Introduction"));
    let mut p = FlowPara::new(FlowKind::Normal);
    p.inlines.push(FlowInline::Math { expr: "x^2 + y^2 = z^2".into(), display: true });
    doc.push_para(p);

    let tex = to_latex(&doc);
    assert!(tex.contains("\\documentclass{article}"));
    assert!(tex.contains("\\section{Introduction}"));
    assert!(tex.contains("\\begin{equation}"));
    assert!(tex.contains("x^2 + y^2 = z^2"));

    let parsed = parse_latex(&tex);
    assert!(!parsed.blocks.is_empty());
}

#[test]
fn test_csv_tsv_codecs() {
    let csv_data = "Name,Age,Notes\r\nAlice,30,\"Enjoys, hiking and music\"\r\nBob,25,\"Line 1\nLine 2\"\r\n";
    assert_eq!(sniff_delimiter(csv_data), Delimiter::Comma);

    let parsed = parse_csv(csv_data, None);
    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[1][0], "Alice");
    assert_eq!(parsed[1][2], "Enjoys, hiking and music");
    assert_eq!(parsed[2][1], "25");
    assert_eq!(parsed[2][2], "Line 1\nLine 2");

    let out_csv = to_csv(&parsed, Delimiter::Comma);
    assert!(out_csv.contains("\"Enjoys, hiking and music\""));
}

#[test]
fn test_openxml_xlsx_roundtrip() {
    let rows = vec![
        vec!["Item".to_string(), "Quantity".to_string(), "Price".to_string()],
        vec!["Widget".to_string(), "10".to_string(), "4.99".to_string()],
        vec!["Gadget".to_string(), "25".to_string(), "12.50".to_string()],
    ];
    let wb = XlsxWorkbook::with_single_sheet("Inventory", rows.clone());

    let pkg = to_xlsx_package(&wb);
    let bytes = pkg.write_to_bytes();
    assert!(bytes.len() > 200);

    let unzipped = ZipPackage::parse(&bytes).expect("xlsx zip should parse");
    let read_wb = parse_xlsx_package(&unzipped).expect("xlsx package should parse");
    assert_eq!(read_wb.sheets.len(), 1);
    let sheet = &read_wb.sheets[0];
    assert_eq!(sheet.rows[0][0], "Item");
    assert_eq!(sheet.rows[0][1], "Quantity");
    assert_eq!(sheet.rows[1][0], "Widget");
    assert_eq!(sheet.rows[1][1], "10");
}

#[test]
fn test_openxml_docx_roundtrip() {
    let mut doc = FlowDoc::new();
    doc.push_para(FlowPara::with_text(FlowKind::Heading(1), "Report Chapter"));
    let mut p = FlowPara::new(FlowKind::Normal);
    p.inlines.push(FlowInline::Text("WordprocessingML test run".into(), FlowFmt { bold: true, ..Default::default() }));
    doc.push_para(p);

    let pkg = to_docx_package(&doc);
    let bytes = pkg.write_to_bytes();
    assert!(bytes.len() > 200);

    let unzipped = ZipPackage::parse(&bytes).expect("docx zip should parse");
    let read_doc = parse_docx_package(&unzipped).expect("docx package should parse");
    assert_eq!(read_doc.blocks.len(), 2);
    if let FlowBlock::Para(p) = &read_doc.blocks[0] {
        assert_eq!(p.kind, FlowKind::Heading(1));
        assert_eq!(p.plain_text(), "Report Chapter");
    }
}

#[test]
fn test_openxml_pptx_roundtrip() {
    use layout_presentation_deck::model::master::{LayoutId, Presentation, Slide};
    use layout_presentation_deck::model::shape::Shape;
    use layout_presentation_deck::model::types::{ShapeId, SlideId, Xfrm};

    let mut pres = Presentation::default();
    let mut slide = Slide::new(SlideId(1), LayoutId(1));
    let shape = Shape::new(ShapeId(101), "SlideTitle", Xfrm::new(50.0, 50.0, 400.0, 80.0))
        .with_text("PresentationML Heading");
    slide.shapes.push(shape);
    pres.slides.push(slide);

    let pkg = to_pptx_package(&pres);
    let bytes = pkg.write_to_bytes();
    assert!(bytes.len() > 200);

    let unzipped = ZipPackage::parse(&bytes).expect("pptx zip should parse");
    let read_pres = parse_pptx_package(&unzipped).expect("pptx package should parse");
    assert_eq!(read_pres.slides.len(), 1);
    let read_shape = &read_pres.slides[0].shapes[0];
    assert_eq!(read_shape.id, ShapeId(101));
    assert_eq!(read_shape.text(), Some("PresentationML Heading".into()));
    assert_eq!(read_shape.xfrm.x, 50.0);
    assert_eq!(read_shape.xfrm.w, 400.0);
}

#[test]
fn test_ontology_contract_parity() {
    use chronicle_office_codecs::ontology::{OFFICE_CODEC_EXPORT_SPEC, OFFICE_CODEC_IMPORT_SPEC};
    assert!(OFFICE_CODEC_IMPORT_SPEC.contract.precondition.is_valid());
    assert!(OFFICE_CODEC_IMPORT_SPEC.contract.postcondition.is_valid());
    assert!(OFFICE_CODEC_IMPORT_SPEC.contract.invariant.is_valid());

    assert!(OFFICE_CODEC_EXPORT_SPEC.contract.precondition.is_valid());
    assert!(OFFICE_CODEC_EXPORT_SPEC.contract.postcondition.is_valid());
    assert!(OFFICE_CODEC_EXPORT_SPEC.contract.invariant.is_valid());
}
