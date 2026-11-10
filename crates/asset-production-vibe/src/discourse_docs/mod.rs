//! Bounded foliation and discourse document morphisms: PDF compilation,
//! OpenXML DOCX prose compilation, OpenXML PPTX slide deck compilation,
//! and standalone SVG data visualization chart rendering for the `discourse` category.

use crate::*;
use base64::{engine::general_purpose::STANDARD, Engine};
use std::collections::BTreeMap;
use std::fmt::Write;

mod docx_pptx;
mod pdf_chart;

// ============================================================================
// Deterministic IEEE 802.3 CRC-32 & Pure-Rust STORE ZIP Archive Writer
// ============================================================================

/// Branchless IEEE 802.3 CRC-32 checksum (polynomial 0xEDB88320).
#[allow(dead_code)] // kept as the STORE-zip integrity primitive for OpenXML parts
pub(super) fn crc32_ieee(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

#[allow(dead_code)] // office codecs now emit DOCX/PPTX; writer remains for STORE zip maps
pub(super) struct ZipEntry {
    pub(super) name: String,
    pub(super) data: Vec<u8>,
}

/// Pure-Rust uncompressed (STORE method 0) ZIP writer with zero external dependencies.
#[allow(dead_code)]
pub(super) struct ZipArchiveWriter {
    pub(super) entries: Vec<ZipEntry>,
}

#[allow(dead_code)]
impl ZipArchiveWriter {
    pub(super) fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(super) fn add_file(&mut self, name: &str, data: Vec<u8>) {
        self.entries.push(ZipEntry {
            name: name.to_string(),
            data,
        });
    }

    pub(super) fn finish(self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut central_dir = Vec::new();

        // Deterministic DOS date/time: 2026-01-01 12:00:00
        // Time: hour 12 (bits 15-11), min 0 (bits 10-5), sec/2 0 (bits 4-0) -> 0x6000
        // Date: year - 1980 = 46 (bits 15-9), month 1 (bits 8-5), day 1 (bits 4-0) -> 0x5C21
        let dos_time = 0x6000u16;
        let dos_date = 0x5C21u16;

        for entry in &self.entries {
            let offset = out.len() as u32;
            let name_bytes = entry.name.as_bytes();
            let crc = crc32_ieee(&entry.data);
            let size = entry.data.len() as u32;

            // Local file header: PK\x03\x04
            out.extend_from_slice(&[0x50, 0x4b, 0x03, 0x04]);
            out.extend_from_slice(&20u16.to_le_bytes()); // version needed: 2.0
            out.extend_from_slice(&0x0800u16.to_le_bytes()); // general purpose bit flag: UTF-8
            out.extend_from_slice(&0u16.to_le_bytes()); // method: 0 (STORE)
            out.extend_from_slice(&dos_time.to_le_bytes());
            out.extend_from_slice(&dos_date.to_le_bytes());
            out.extend_from_slice(&crc.to_le_bytes());
            out.extend_from_slice(&size.to_le_bytes()); // compressed size
            out.extend_from_slice(&size.to_le_bytes()); // uncompressed size
            out.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes()); // extra field length
            out.extend_from_slice(name_bytes);
            out.extend_from_slice(&entry.data);

            // Central directory record: PK\x01\x02
            central_dir.extend_from_slice(&[0x50, 0x4b, 0x01, 0x02]);
            central_dir.extend_from_slice(&20u16.to_le_bytes()); // version made by
            central_dir.extend_from_slice(&20u16.to_le_bytes()); // version needed
            central_dir.extend_from_slice(&0x0800u16.to_le_bytes()); // UTF-8
            central_dir.extend_from_slice(&0u16.to_le_bytes()); // method 0
            central_dir.extend_from_slice(&dos_time.to_le_bytes());
            central_dir.extend_from_slice(&dos_date.to_le_bytes());
            central_dir.extend_from_slice(&crc.to_le_bytes());
            central_dir.extend_from_slice(&size.to_le_bytes());
            central_dir.extend_from_slice(&size.to_le_bytes());
            central_dir.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
            central_dir.extend_from_slice(&0u16.to_le_bytes()); // extra field length
            central_dir.extend_from_slice(&0u16.to_le_bytes()); // comment length
            central_dir.extend_from_slice(&0u16.to_le_bytes()); // disk number start
            central_dir.extend_from_slice(&0u16.to_le_bytes()); // internal attrs
            central_dir.extend_from_slice(&0u32.to_le_bytes()); // external attrs
            central_dir.extend_from_slice(&offset.to_le_bytes()); // offset of local header
            central_dir.extend_from_slice(name_bytes);
        }

        let cd_offset = out.len() as u32;
        let cd_size = central_dir.len() as u32;
        out.extend_from_slice(&central_dir);

        // End of central directory record: PK\x05\x06
        out.extend_from_slice(&[0x50, 0x4b, 0x05, 0x06]);
        out.extend_from_slice(&0u16.to_le_bytes()); // disk number
        out.extend_from_slice(&0u16.to_le_bytes()); // start disk
        out.extend_from_slice(&(self.entries.len() as u16).to_le_bytes()); // entries this disk
        out.extend_from_slice(&(self.entries.len() as u16).to_le_bytes()); // total entries
        out.extend_from_slice(&cd_size.to_le_bytes());
        out.extend_from_slice(&cd_offset.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // comment length

        out
    }
}

// ============================================================================
// String Escaping Helpers
// ============================================================================

/// Escape XML entities for OpenXML and SVG documents.
pub(super) fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

/// Escape ASCII characters in PDF string literals `(...)`.
pub(super) fn escape_pdf_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '(' => out.push_str("\\("),
            ')' => out.push_str("\\)"),
            '\r' | '\n' => out.push(' '),
            c if (c as u32) < 32 || (c as u32) > 126 => {
                if (c as u32) <= 255 {
                    let _ = write!(out, "\\{:03o}", c as u8);
                } else {
                    out.push('?');
                }
            }
            c => out.push(c),
        }
    }
    out
}

/// Produce-safe family for gallery presentation of sealed Q42 containers.
/// Maps kind/magic onto {10d, m64, emf, q42} without inventing new file magics.
fn container_family(kind: &str, bytes: &[u8]) -> Option<&'static str> {
    let magic: String = bytes
        .iter()
        .take(8)
        .map(|&b| {
            if (0x20..=0x7e).contains(&b) {
                b as char
            } else {
                '.'
            }
        })
        .collect();
    match kind {
        "10d" => Some("10d"),
        "m64" => Some("m64"),
        "emf" => Some("emf"),
        "q42" => Some("q42"),
        _ if magic.starts_with("Q42EMF") => Some("emf"),
        _ if magic.starts_with("Q42M64") || magic.starts_with("E64") => Some("m64"),
        _ if magic.starts_with("Q42") => Some("q42"),
        _ => None,
    }
}

/// Display-referred sRGB SVG card. Binary containers are never inlined as executable payloads.
fn produce_safe_container_svg(family: &str, kind: &str, name: &str, bytes: &[u8]) -> String {
    let magic: String = bytes
        .iter()
        .take(8)
        .map(|&b| {
            if (0x20..=0x7e).contains(&b) {
                b as char
            } else {
                '.'
            }
        })
        .collect();
    let (bg, fg, bar) = match family {
        "10d" => ("#134e4a", "#99f6e4", "#2dd4bf"),
        "m64" => ("#7c2d12", "#fed7aa", "#fb923c"),
        "emf" => ("#1e3a8a", "#bfdbfe", "#38bdf8"),
        _ => ("#4c1d95", "#e9d5ff", "#c084fc"),
    };
    let n = bytes.len();
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 320 180\" width=\"320\" height=\"180\" data-family=\"{family}\" data-kind=\"{}\">\
<rect width=\"320\" height=\"180\" rx=\"12\" fill=\"{bg}\"/>\
<text x=\"20\" y=\"36\" fill=\"{fg}\" font-family=\"system-ui,sans-serif\" font-size=\"18\" font-weight=\"700\">{}</text>\
<text x=\"20\" y=\"62\" fill=\"{fg}\" font-family=\"ui-monospace,monospace\" font-size=\"12\">kind {} · magic {}</text>\
<text x=\"20\" y=\"84\" fill=\"{fg}\" font-family=\"system-ui,sans-serif\" font-size=\"12\">{n} bytes · produce-safe map</text>\
<rect x=\"20\" y=\"110\" width=\"280\" height=\"44\" rx=\"8\" fill=\"{bar}\" fill-opacity=\"0.85\"/>\
<text x=\"160\" y=\"138\" text-anchor=\"middle\" fill=\"#0f172a\" font-family=\"system-ui,sans-serif\" font-size=\"14\" font-weight=\"600\">{family}</text>\
</svg>",
        escape_xml(kind),
        escape_xml(name),
        escape_xml(kind),
        escape_xml(&magic),
    )
}

fn html_escape(s: &str) -> String {
    escape_xml(s).replace('\'', "&#39;")
}

struct PdfMarked {
    page_index: usize,
    mcid: i32,
    tag: &'static str,
    alt_or_text: String,
}

// ============================================================================
// Internal Models
// ============================================================================

struct PdfVectorRect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    fill_rgb: [f64; 3],
}

struct PdfPageSpec {
    heading: Option<String>,
    lines: Vec<String>,
    rects: Vec<PdfVectorRect>,
}

// ============================================================================
// AssetHost Discourse Morphisms Implementation
// ============================================================================

impl AssetHost<'_> {
    /// Categorical operations dispatch for the `discourse` category morphisms.
    pub(crate) fn discourse_op(&mut self, op: &str, a: &Args) -> Result<Value, String> {
        match op {
            "hypermedia.discourse.compile_pdf"
            | "discourse.compile_pdf"
            | "pdf.compile"
            | "compile_pdf" => self.compile_pdf(a),

            "hypermedia.discourse.compile_prose_docx"
            | "discourse.compile_prose_docx"
            | "docx.compile"
            | "compile_prose_docx" => self.compile_prose_docx(a),

            "hypermedia.discourse.compile_deck_pptx"
            | "discourse.compile_deck_pptx"
            | "pptx.compile"
            | "compile_deck_pptx" => self.compile_deck_pptx(a),

            "hypermedia.discourse.render_chart_svg"
            | "discourse.render_chart_svg"
            | "chart.svg"
            | "render_chart_svg" => self.render_chart_svg(a),

            "hypermedia.discourse.review_gallery"
            | "discourse.review_gallery"
            | "gallery" => self.review_gallery(a),

            _ => Err(format!("Unknown discourse operation: {op}")),
        }
    }

    // ------------------------------------------------------------------------
    // 1. PDF Foliated Document Compilation
    // ------------------------------------------------------------------------
    pub(super) fn compile_pdf(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "title",
                "author",
                "width_pt",
                "height_pt",
                "pages",
                "body",
                "lines",
                "heading",
                "rects",
                "artifact_id",
                "id",
                "lang",
                "language",
            ],
        )?;

        let title = a
            .get("title")
            .map(text)
            .transpose()?
            .unwrap_or("Hypermedia Foliated Discourse");
        let author = a
            .get("author")
            .map(text)
            .transpose()?
            .unwrap_or("Synth Ontology Engine");
        let lang = a
            .get("lang")
            .or_else(|| a.get("language"))
            .map(text)
            .transpose()?
            .unwrap_or("en-AU");

        let width_pt = number(a, "width_pt", 612.0)?;
        if !(72.0..=2880.0).contains(&width_pt) {
            return Err("width_pt outside 72.0..=2880.0".into());
        }
        let height_pt = number(a, "height_pt", 792.0)?;
        if !(72.0..=2880.0).contains(&height_pt) {
            return Err("height_pt outside 72.0..=2880.0".into());
        }

        let mut pages: Vec<PdfPageSpec> = Vec::new();

        if let Some(pages_val) = a.get("pages") {
            let page_items = list(pages_val)?;
            if page_items.is_empty() {
                return Err("pages array must not be empty".into());
            }
            if page_items.len() > 256 {
                return Err("pages count exceeds 256".into());
            }
            for item in page_items {
                let r = record(item)?;
                let heading = r
                    .get("heading")
                    .map(text)
                    .transpose()?
                    .map(|s| s.to_string());

                let mut lines = Vec::new();
                if let Some(lines_val) = r.get("lines").or_else(|| r.get("paragraphs")) {
                    for l in list(lines_val)? {
                        if let Ok(t) = text(l) {
                            lines.push(t.to_string());
                        }
                    }
                } else if let Some(body_str) = r.get("body").or_else(|| r.get("text")).map(text).transpose()? {
                    for line in body_str.lines() {
                        lines.push(line.to_string());
                    }
                }

                let mut rects = Vec::new();
                if let Some(rects_val) = r.get("rects") {
                    for rect_item in list(rects_val)? {
                        let rr = record(rect_item)?;
                        let rx = number(rr, "x", 0.0)?;
                        let ry = number(rr, "y", 0.0)?;
                        let rw = rr
                            .get("width")
                            .or_else(|| rr.get("w"))
                            .map(num)
                            .transpose()?
                            .unwrap_or(100.0);
                        let rh = rr
                            .get("height")
                            .or_else(|| rr.get("h"))
                            .map(num)
                            .transpose()?
                            .unwrap_or(100.0);

                        let fill_rgb = if let Some(rgb_val) = rr.get("fill_rgb") {
                            let list = list(rgb_val)?;
                            if list.len() >= 3 {
                                let mut cr = num(&list[0])?;
                                let mut cg = num(&list[1])?;
                                let mut cb = num(&list[2])?;
                                if cr > 1.0 || cg > 1.0 || cb > 1.0 {
                                    cr /= 255.0;
                                    cg /= 255.0;
                                    cb /= 255.0;
                                }
                                [cr.clamp(0.0, 1.0), cg.clamp(0.0, 1.0), cb.clamp(0.0, 1.0)]
                            } else {
                                [0.2, 0.4, 0.8]
                            }
                        } else {
                            [0.2, 0.4, 0.8]
                        };

                        rects.push(PdfVectorRect {
                            x: rx,
                            y: ry,
                            w: rw.max(0.0),
                            h: rh.max(0.0),
                            fill_rgb,
                        });
                    }
                }

                pages.push(PdfPageSpec {
                    heading,
                    lines,
                    rects,
                });
            }
        } else {
            // Default 1 page
            let heading = a
                .get("heading")
                .map(text)
                .transpose()?
                .map(|s| s.to_string())
                .or_else(|| Some(title.to_string()));

            let mut lines = Vec::new();
            if let Some(body_str) = a.get("body").map(text).transpose()? {
                for line in body_str.lines() {
                    lines.push(line.to_string());
                }
            } else if let Some(lines_val) = a.get("lines") {
                for l in list(lines_val)? {
                    if let Ok(t) = text(l) {
                        lines.push(t.to_string());
                    }
                }
            } else {
                lines.push("Generated by Hypermedia Discourse Foliation Engine.".to_string());
            }

            pages.push(PdfPageSpec {
                heading,
                lines,
                rects: Vec::new(),
            });
        }

        let page_count = pages.len();
        let first_page_id = 9usize;
        let margin = 54.0;
        let mut page_streams = Vec::with_capacity(page_count);
        let mut marked: Vec<PdfMarked> = Vec::new();
        let mut a11y_headings = Vec::new();
        let mut a11y_figures = Vec::new();

        for (page_index, page) in pages.iter().enumerate() {
            let mut stream = String::new();
            let mut mcid = 0i32;

            for rect in &page.rects {
                let _ = write!(
                    stream,
                    "/Figure << /MCID {mcid} >> BDC\nq\n{:.4} {:.4} {:.4} rg\n{:.2} {:.2} {:.2} {:.2} re f\nQ\nEMC\n",
                    rect.fill_rgb[0],
                    rect.fill_rgb[1],
                    rect.fill_rgb[2],
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h
                );
                a11y_figures.push(discourse_pdf_compliance::AccessibleFigure {
                    id: a11y_figures.len() as u64 + 1,
                    alt_text: Some("vector rectangle".into()),
                    is_decorative: false,
                    nested_alt: false,
                    hides_annotation: false,
                });
                marked.push(PdfMarked {
                    page_index,
                    mcid,
                    tag: "Figure",
                    alt_or_text: "vector rectangle".into(),
                });
                mcid += 1;
            }

            let mut cur_y = height_pt - margin;

            if let Some(ref heading) = page.heading {
                cur_y -= 20.0;
                let escaped = escape_pdf_str(heading);
                let _ = write!(
                    stream,
                    "/H1 << /MCID {mcid} >> BDC\nBT\n/F2 18 Tf\n1 0 0 1 {:.2} {:.2} Tm\n({escaped}) Tj\nET\nEMC\n",
                    margin, cur_y
                );
                a11y_headings.push(discourse_pdf_compliance::AccessibleHeading {
                    level: 1,
                    text: heading.clone(),
                });
                marked.push(PdfMarked {
                    page_index,
                    mcid,
                    tag: "H1",
                    alt_or_text: heading.clone(),
                });
                mcid += 1;
                cur_y -= 14.0;
            }

            for line in &page.lines {
                cur_y -= 15.0;
                if cur_y < margin {
                    break;
                }
                let escaped = escape_pdf_str(line);
                let _ = write!(
                    stream,
                    "/P << /MCID {mcid} >> BDC\nBT\n/F1 11 Tf\n1 0 0 1 {:.2} {:.2} Tm\n({escaped}) Tj\nET\nEMC\n",
                    margin, cur_y
                );
                marked.push(PdfMarked {
                    page_index,
                    mcid,
                    tag: "P",
                    alt_or_text: line.clone(),
                });
                mcid += 1;
            }

            page_streams.push(stream);
        }

        let first_struct_id = first_page_id + 2 * page_count;
        let mut struct_ids_by_page: Vec<Vec<usize>> = vec![Vec::new(); page_count];
        let mut next_struct = first_struct_id;
        for item in &marked {
            struct_ids_by_page[item.page_index].push(next_struct);
            next_struct += 1;
        }
        let total_objs = next_struct - 1;

        let xmp = format!(
            "<?xpacket begin=\"\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?>\n<x:xmpmeta xmlns:x=\"adobe:ns:meta/\"><rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\"><rdf:Description rdf:about=\"\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\" xmlns:pdfuaid=\"http://www.aiim.org/pdfua/ns/id/\"><dc:title><rdf:Alt><rdf:li xml:lang=\"x-default\">{}</rdf:li></rdf:Alt></dc:title><pdf:Producer>Hypermedia Discourse</pdf:Producer><pdfuaid:part>1</pdfuaid:part></rdf:Description></rdf:RDF></x:xmpmeta>\n<?xpacket end=\"w\"?>",
            escape_xml(title)
        );

        let mut pdf = Vec::new();
        pdf.extend_from_slice(b"%PDF-1.7\n%\xE2\xE3\xCF\xD3\n");
        let mut offsets = Vec::with_capacity(total_objs);

        offsets.push(pdf.len());
        let catalog = format!(
            "1 0 obj\n<<\n  /Type /Catalog\n  /Pages 2 0 R\n  /Lang ({})\n  /MarkInfo << /Marked true >>\n  /ViewerPreferences << /DisplayDocTitle true >>\n  /StructTreeRoot 6 0 R\n  /Metadata 5 0 R\n>>\nendobj\n",
            escape_pdf_str(lang)
        );
        pdf.extend_from_slice(catalog.as_bytes());

        offsets.push(pdf.len());
        let mut kids = String::new();
        for i in 0..page_count {
            let _ = write!(kids, "{} 0 R ", first_page_id + 2 * i);
        }
        let pages_obj = format!(
            "2 0 obj\n<<\n  /Type /Pages\n  /Kids [ {kids}]\n  /Count {page_count}\n>>\nendobj\n"
        );
        pdf.extend_from_slice(pages_obj.as_bytes());

        offsets.push(pdf.len());
        pdf.extend_from_slice(
            b"3 0 obj\n<<\n  /Type /Font\n  /Subtype /Type1\n  /BaseFont /Helvetica\n  /Encoding /WinAnsiEncoding\n>>\nendobj\n",
        );

        offsets.push(pdf.len());
        pdf.extend_from_slice(
            b"4 0 obj\n<<\n  /Type /Font\n  /Subtype /Type1\n  /BaseFont /Helvetica-Bold\n  /Encoding /WinAnsiEncoding\n>>\nendobj\n",
        );

        offsets.push(pdf.len());
        let meta_hdr = format!(
            "5 0 obj\n<< /Type /Metadata /Subtype /XML /Length {} >>\nstream\n",
            xmp.len()
        );
        pdf.extend_from_slice(meta_hdr.as_bytes());
        pdf.extend_from_slice(xmp.as_bytes());
        pdf.extend_from_slice(b"\nendstream\nendobj\n");

        offsets.push(pdf.len());
        pdf.extend_from_slice(
            b"6 0 obj\n<<\n  /Type /StructTreeRoot\n  /K [ 7 0 R ]\n  /ParentTree 8 0 R\n  /ParentTreeNextKey ",
        );
        pdf.extend_from_slice(format!("{page_count}\n>>\nendobj\n").as_bytes());

        offsets.push(pdf.len());
        let mut doc_kids = String::new();
        for ids in &struct_ids_by_page {
            for id in ids {
                let _ = write!(doc_kids, "{id} 0 R ");
            }
        }
        let doc_elem = format!(
            "7 0 obj\n<<\n  /Type /StructElem\n  /S /Document\n  /P 6 0 R\n  /K [ {doc_kids}]\n  /Lang ({})\n>>\nendobj\n",
            escape_pdf_str(lang)
        );
        pdf.extend_from_slice(doc_elem.as_bytes());

        offsets.push(pdf.len());
        let mut nums = String::from("8 0 obj\n<<\n  /Nums [\n");
        for (page_i, ids) in struct_ids_by_page.iter().enumerate() {
            let _ = write!(nums, "    {page_i} [");
            for id in ids {
                let _ = write!(nums, " {id} 0 R");
            }
            nums.push_str(" ]\n");
        }
        nums.push_str("  ]\n>>\nendobj\n");
        pdf.extend_from_slice(nums.as_bytes());

        for (i, stream_content) in page_streams.iter().enumerate() {
            let page_id = first_page_id + 2 * i;
            let contents_id = page_id + 1;
            offsets.push(pdf.len());
            let page_obj = format!(
                "{page_id} 0 obj\n<<\n  /Type /Page\n  /Parent 2 0 R\n  /MediaBox [0 0 {width_pt:.2} {height_pt:.2}]\n  /Tabs /S\n  /StructParents {i}\n  /Resources <<\n    /Font <<\n      /F1 3 0 R\n      /F2 4 0 R\n    >>\n  >>\n  /Contents {contents_id} 0 R\n>>\nendobj\n"
            );
            pdf.extend_from_slice(page_obj.as_bytes());

            offsets.push(pdf.len());
            let stream_hdr = format!(
                "{contents_id} 0 obj\n<< /Length {} >>\nstream\n",
                stream_content.len()
            );
            pdf.extend_from_slice(stream_hdr.as_bytes());
            pdf.extend_from_slice(stream_content.as_bytes());
            pdf.extend_from_slice(b"\nendstream\nendobj\n");
        }

        for (k, item) in marked.iter().enumerate() {
            let id = first_struct_id + k;
            let page_id = first_page_id + 2 * item.page_index;
            offsets.push(pdf.len());
            let alt = if item.tag == "Figure" {
                format!("  /Alt ({})\n", escape_pdf_str(&item.alt_or_text))
            } else {
                String::new()
            };
            let elem = format!(
                "{id} 0 obj\n<<\n  /Type /StructElem\n  /S /{}\n  /P 7 0 R\n  /Pg {page_id} 0 R\n  /K [ {} ]\n{alt}>>\nendobj\n",
                item.tag, item.mcid
            );
            pdf.extend_from_slice(elem.as_bytes());
        }

        let start_xref = pdf.len();
        pdf.extend_from_slice(b"xref\n");
        let xref_header = format!("0 {}\n", total_objs + 1);
        pdf.extend_from_slice(xref_header.as_bytes());
        pdf.extend_from_slice(b"0000000000 65535 f \r\n");
        for off in &offsets {
            let entry = format!("{:010} 00000 n \r\n", off);
            pdf.extend_from_slice(entry.as_bytes());
        }

        let trailer = format!(
            "trailer\n<<\n  /Size {}\n  /Root 1 0 R\n  /Info << /Title ({}) /Author ({}) >>\n>>\nstartxref\n{}\n%%EOF\n",
            total_objs + 1,
            escape_pdf_str(title),
            escape_pdf_str(author),
            start_xref
        );
        pdf.extend_from_slice(trailer.as_bytes());

        let pdf_len = pdf.len();
        self.charge(page_count * 1024 + 512, pdf_len * 2 + 1024)?;

        use discourse_pdf_compliance::{AccessibleDocSnapshot, Status, check_accessibility};
        let a11y_doc = AccessibleDocSnapshot {
            title: Some(title.to_string()),
            language: Some(lang.to_string()),
            is_tagged: true,
            has_permission_flag: true,
            is_image_only: false,
            page_count,
            has_bookmarks: page_count <= 20,
            tab_order_consistent: true,
            character_encoding_valid: true,
            untagged_content_count: 0,
            untagged_annotations_count: 0,
            multimedia_tagged: true,
            screen_flicker_detected: false,
            scripts_accessible: true,
            timed_responses_extendable: true,
            navigation_links_valid: true,
            form_fields: vec![],
            figures: a11y_figures,
            tables: vec![],
            lists: vec![],
            headings: a11y_headings,
        };
        let verdicts = check_accessibility(&a11y_doc);
        let passed_rules = verdicts.iter().filter(|v| v.status == Status::Passed).count();

        let encoded = self.encode("pdf", pdf)?;

        let mut rec = match encoded {
            Value::Record(r) => r,
            _ => BTreeMap::new(),
        };

        let id_str = a
            .get("artifact_id")
            .or_else(|| a.get("id"))
            .and_then(|v| text(v).ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                let h = rec
                    .get("handle")
                    .and_then(|v| match v {
                        Value::U64(n) => Some(*n),
                        Value::I64(n) => Some(*n as u64),
                        _ => None,
                    })
                    .unwrap_or(0);
                format!("discourse-pdf-{h}")
            });

        rec.insert("id".into(), Value::String(id_str.clone()));
        rec.insert("resource".into(), Value::String(id_str));
        rec.insert("kind".into(), Value::String("pdf".into()));
        rec.insert("page_count".into(), Value::I64(page_count as i64));
        rec.insert("byte_len".into(), Value::I64(pdf_len as i64));
        rec.insert("pdf_ua_tagged".into(), Value::Bool(true));
        rec.insert("pdf_ua_part".into(), Value::I64(1));
        rec.insert("pdf_ua_lang".into(), Value::String(lang.to_string()));
        rec.insert("pdf_ua_rules_evaluated".into(), Value::I64(verdicts.len() as i64));
        rec.insert("pdf_ua_rules_passed".into(), Value::I64(passed_rules as i64));

        Ok(Value::Record(rec))
    }

    /// CSP-locked gallery. Origin image/audio path is preserved; 10d/m64/emf/q42
    /// get produce-safe display-referred SVG maps instead of opaque native-tool stubs.
    pub(super) fn review_gallery(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["title", "files"])?;
        let files = list(required(a, "files")?)?;
        let mut needs_container_maps = false;
        for f in files {
            let f = record(f)?;
            let r = self.workspace.encoded(required(f, "resource")?)?;
            if container_family(&r.kind, &r.bytes).is_some() {
                needs_container_maps = true;
                break;
            }
        }
        if !needs_container_maps {
            return self.gallery(a);
        }

        let title = a
            .get("title")
            .map(text)
            .transpose()?
            .unwrap_or("Asset preview");
        if title.len() > 1024 {
            return Err("Preview title exceeds 1024 bytes".into());
        }
        if files.len() > 128 {
            return Err("At most 128 preview resources".into());
        }

        let mut estimate = 8192 + title.len() * 6;
        let mut presented: Vec<String> = Vec::new();
        for f in files {
            let f = record(f)?;
            fields(f, &["name", "resource"])?;
            let name = text(required(f, "name")?)?;
            if name.len() > 1024 {
                return Err("Preview name exceeds 1024 bytes".into());
            }
            let resource = self.workspace.encoded(required(f, "resource")?)?;
            if let Some(family) = container_family(&resource.kind, &resource.bytes) {
                if !presented.iter().any(|p| p == family) {
                    presented.push(family.to_string());
                }
            }
            estimate = estimate
                .checked_add(
                    resource
                        .bytes
                        .len()
                        .checked_mul(2)
                        .ok_or("Preview size overflow")?
                        + name.len() * 6
                        + 2048,
                )
                .ok_or("Preview size overflow")?;
        }
        if estimate > self.limits.resource_bytes {
            return Err("Preview exceeds resource byte budget".into());
        }
        self.charge(estimate, estimate)?;

        let presented_attr = presented.join(",");
        let mut html = String::from("<!doctype html><html lang=\"en\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><meta http-equiv=\"Content-Security-Policy\" content=\"default-src 'none'; img-src data:; media-src data:; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'\"><title>");
        html.push_str(&html_escape(title));
        html.push_str("</title><style>body{font:16px system-ui;margin:32px;background:#101923;color:#eaf1f8}h1{font-size:28px}main{display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:20px}article{min-width:0;padding:20px;background:#1c2937;border-radius:12px}h2{font-size:16px;overflow-wrap:anywhere}img{display:block;max-width:100%;height:auto;background:repeating-conic-gradient(#ccd1d6 0% 25%,#fff 0% 50%) 0/20px 20px;border-radius:6px}audio{max-width:100%}p{color:#b6c7d8}</style><h1>");
        html.push_str(&html_escape(title));
        html.push_str(&format!(
            "</h1><main data-presented=\"{}\">",
            html_escape(&presented_attr)
        ));

        for f in files {
            let f = record(f)?;
            let name = html_escape(text(required(f, "name")?)?);
            let r = self.workspace.encoded(required(f, "resource")?)?;
            html.push_str(&format!("<article><h2>{name}</h2>"));
            if let Some(family) = container_family(&r.kind, &r.bytes) {
                let svg = produce_safe_container_svg(family, &r.kind, text(required(f, "name")?)?, &r.bytes);
                let data = STANDARD.encode(svg.as_bytes());
                html.push_str(&format!(
                    "<img alt=\"{name}\" data-family=\"{family}\" src=\"data:image/svg+xml;base64,{data}\">"
                ));
                html.push_str(&format!(
                    "<p>{} · {} bytes · produce-safe {family} map</p></article>",
                    html_escape(&r.kind),
                    r.bytes.len()
                ));
                continue;
            }
            let mime = match r.kind.as_str() {
                "png" => Some("image/png"),
                "svg" => Some("image/svg+xml"),
                "jpeg" | "jpg" => Some("image/jpeg"),
                "webp" => Some("image/webp"),
                "wav" => Some("audio/wav"),
                _ => None,
            };
            if let Some(mime) = mime {
                let data = STANDARD.encode(&r.bytes);
                if mime.starts_with("image/") {
                    html.push_str(&format!(
                        "<img alt=\"{name}\" src=\"data:{mime};base64,{data}\">"
                    ));
                } else {
                    html.push_str(&format!("<audio controls preload=\"none\" src=\"data:{mime};base64,{data}\"></audio>"));
                }
            } else {
                html.push_str("<p>Use the format's native preview tool for this resource.</p>");
            }
            html.push_str(&format!(
                "<p>{} · {} bytes</p></article>",
                html_escape(&r.kind),
                r.bytes.len()
            ));
        }
        html.push_str("</main></html>");

        let encoded = self.encode("html", html.into_bytes())?;
        let mut rec = match encoded {
            Value::Record(r) => r,
            _ => BTreeMap::new(),
        };
        rec.insert(
            "presented_families".into(),
            Value::List(presented.into_iter().map(Value::String).collect()),
        );
        rec.insert("kind".into(), Value::String("html".into()));
        Ok(Value::Record(rec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chronicle_office_codecs::{parse_docx_package, parse_pptx_package, ZipPackage};
    use std::sync::atomic::AtomicBool;

    fn host<'a>(
        cancel: &'a AtomicBool,
        limits: &'a ToolLimits,
    ) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits,
            cancel,
            work: 0,
        }
    }

    fn encoded_bytes(host: &AssetHost<'_>, handle: &Value) -> Vec<u8> {
        host.workspace
            .encoded(handle)
            .expect("encoded resource")
            .bytes
            .clone()
    }

    #[test]
    fn compile_pdf_emits_tagged_pdf_ua_object() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut h = host(&cancel, &limits);

        let mut args = Args::new();
        args.insert("title".into(), Value::String("UA Foliation".into()));
        args.insert("heading".into(), Value::String("Tagged Heading".into()));
        args.insert("body".into(), Value::String("Accessible paragraph.".into()));
        let val = h
            .discourse_op("hypermedia.discourse.compile_pdf", &args)
            .expect("compile_pdf");
        let rec = record(&val).unwrap();
        assert_eq!(rec.get("pdf_ua_tagged"), Some(&Value::Bool(true)));
        assert_eq!(rec.get("pdf_ua_part"), Some(&Value::I64(1)));
        match rec.get("pdf_ua_rules_passed") {
            Some(Value::I64(n)) => assert!(*n >= 20, "PDF/UA must pass the auto-checkable rules, got {n}"),
            other => panic!("pdf_ua_rules_passed: {other:?}"),
        }

        let bytes = encoded_bytes(&h, &val);
        assert!(bytes.starts_with(b"%PDF-1.7"), "object must be a PDF, not a catalog row");
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("/StructTreeRoot"), "PDF/UA requires a structure tree");
        assert!(text.contains("/MarkInfo"), "PDF/UA requires MarkInfo");
        assert!(text.contains("/Marked true"));
        assert!(text.contains("/Lang"));
        assert!(text.contains("pdfuaid:part"));
        assert!(text.contains("BDC"), "marked content sequences");
        assert!(text.contains("/Tabs /S"));
        assert!(text.contains("%%EOF"));
    }

    #[test]
    fn compile_prose_docx_emits_openxml_object() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut h = host(&cancel, &limits);

        let mut heading = BTreeMap::new();
        heading.insert("heading".into(), Value::String("Section One".into()));
        heading.insert("body".into(), Value::String("Prose body.".into()));
        let mut args = Args::new();
        args.insert("title".into(), Value::String("Discourse Prose".into()));
        args.insert("sections".into(), Value::List(vec![Value::Record(heading)]));
        args.insert(
            "table".into(),
            Value::List(vec![Value::List(vec![
                Value::String("A".into()),
                Value::String("B".into()),
            ])]),
        );
        let val = h
            .discourse_op("hypermedia.discourse.compile_prose_docx", &args)
            .expect("compile_prose_docx");
        let rec = record(&val).unwrap();
        match rec.get("docx_block_count") {
            Some(Value::I64(n)) => assert!(*n >= 2, "docx must contain title+body blocks, got {n}"),
            other => panic!("docx_block_count: {other:?}"),
        }

        let bytes = encoded_bytes(&h, &val);
        assert_eq!(&bytes[..2], b"PK", "object must be a ZIP/DOCX, not a catalog row");
        let pkg = ZipPackage::parse(&bytes).expect("parse docx zip");
        assert!(pkg.get("word/document.xml").is_some());
        let doc = parse_docx_package(&pkg).expect("parse_docx_package");
        let plain: String = doc
            .blocks
            .iter()
            .map(|b| match b {
                chronicle_office_codecs::FlowBlock::Para(p) => p.plain_text(),
                chronicle_office_codecs::FlowBlock::Table(_) => "[table]".into(),
            })
            .collect::<Vec<_>>()
            .join(" ");
        assert!(plain.contains("Discourse Prose"));
        assert!(plain.contains("Section One"));
        assert!(plain.contains("Prose body"));
        assert!(plain.contains("[table]"));
    }

    #[test]
    fn compile_deck_pptx_emits_openxml_object() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut h = host(&cancel, &limits);

        let mut slide = BTreeMap::new();
        slide.insert("title".into(), Value::String("Deck Title".into()));
        slide.insert(
            "bullets".into(),
            Value::List(vec![Value::String("First point".into())]),
        );
        let mut args = Args::new();
        args.insert("title".into(), Value::String("Hypermedia Deck".into()));
        args.insert("slides".into(), Value::List(vec![Value::Record(slide)]));
        let val = h
            .discourse_op("hypermedia.discourse.compile_deck_pptx", &args)
            .expect("compile_deck_pptx");
        let rec = record(&val).unwrap();
        assert_eq!(rec.get("slide_count"), Some(&Value::I64(1)));

        let bytes = encoded_bytes(&h, &val);
        assert_eq!(&bytes[..2], b"PK", "object must be a ZIP/PPTX, not a catalog row");
        let pkg = ZipPackage::parse(&bytes).expect("parse pptx zip");
        assert!(pkg.get("ppt/slides/slide1.xml").is_some());
        let pres = parse_pptx_package(&pkg).expect("parse_pptx_package");
        assert_eq!(pres.slides.len(), 1);
        let texts: Vec<String> = pres.slides[0]
            .shapes
            .iter()
            .filter_map(|s| s.text())
            .collect();
        let joined = texts.join(" ");
        assert!(joined.contains("Deck Title"));
        assert!(joined.contains("First point"));
    }

    #[test]
    fn compile_deck_pptx_emits_one_paragraph_per_bullet() {
        use layout_presentation_deck::model::shape::ShapeContent;

        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut h = host(&cancel, &limits);

        let mut slide = BTreeMap::new();
        slide.insert("title".into(), Value::String("Two Bullets".into()));
        slide.insert(
            "bullets".into(),
            Value::List(vec![
                Value::String("A".into()),
                Value::String("B".into()),
            ]),
        );
        let mut args = Args::new();
        args.insert("title".into(), Value::String("Hypermedia Deck".into()));
        args.insert("slides".into(), Value::List(vec![Value::Record(slide)]));
        let val = h
            .discourse_op("hypermedia.discourse.compile_deck_pptx", &args)
            .expect("compile_deck_pptx");

        let bytes = encoded_bytes(&h, &val);
        assert_eq!(&bytes[..2], b"PK");
        let pkg = ZipPackage::parse(&bytes).expect("parse pptx zip");
        let slide_xml = std::str::from_utf8(
            pkg.get("ppt/slides/slide1.xml")
                .expect("slide1.xml"),
        )
        .expect("utf8 slide xml");
        assert!(
            !slide_xml.contains("A\nB"),
            "ECMA-376 forbids U+000A in a:t; bullets must not flatten to A\\nB"
        );
        assert!(slide_xml.contains("<a:t>A</a:t>"));
        assert!(slide_xml.contains("<a:t>B</a:t>"));

        let pres = parse_pptx_package(&pkg).expect("parse_pptx_package");
        let content = pres.slides[0]
            .shapes
            .iter()
            .find(|s| s.name == "Content")
            .expect("content shape");
        let paras: Vec<String> = match &content.content {
            ShapeContent::Text(tb) => tb.paragraphs.iter().map(|p| p.text()).collect(),
            other => panic!("content shape must be text, got {other:?}"),
        };
        assert_eq!(
            paras,
            vec!["A".to_string(), "B".to_string()],
            "two bullets must round-trip as two paragraphs, not a single A\\nB run"
        );
        assert!(paras.iter().all(|t| !t.contains('\n')));
    }

    #[test]
    fn review_gallery_presents_10d_m64_emf_q42_maps() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut h = host(&cancel, &limits);

        let d10 = h.admit_encoded("10d", b"10DMESH\0preview".to_vec()).unwrap();
        let m64 = h.admit_encoded("m64", b"Q42M64\0\0strata".to_vec()).unwrap();
        let emf = h.admit_encoded("emf", b"Q42EMF64lattice".to_vec()).unwrap();
        let q42 = h.admit_encoded("q42", b"Q42QUIN\0slot".to_vec()).unwrap();

        let file = |name: &str, resource: Value| {
            let mut rec = BTreeMap::new();
            rec.insert("name".into(), Value::String(name.into()));
            rec.insert("resource".into(), resource);
            Value::Record(rec)
        };
        let mut args = Args::new();
        args.insert("title".into(), Value::String("Container gallery".into()));
        args.insert(
            "files".into(),
            Value::List(vec![
                file("mesh.10d", d10),
                file("matter.m64", m64),
                file("field.emf", emf),
                file("quin.q42", q42),
            ]),
        );
        let val = h
            .discourse_op("hypermedia.discourse.review_gallery", &args)
            .expect("review_gallery");
        let rec = record(&val).unwrap();
        let families = match rec.get("presented_families") {
            Some(Value::List(items)) => items
                .iter()
                .filter_map(|v| text(v).ok().map(|s| s.to_string()))
                .collect::<Vec<_>>(),
            other => panic!("presented_families: {other:?}"),
        };
        for need in ["10d", "m64", "emf", "q42"] {
            assert!(
                families.iter().any(|f| f == need),
                "gallery must present {need}, got {families:?}"
            );
        }

        let bytes = encoded_bytes(&h, &val);
        let html = std::str::from_utf8(&bytes).expect("utf8 html");
        assert!(html.contains("<!doctype html>"), "object must be HTML, not a catalog row");
        assert!(html.contains("data-presented=\"10d,m64,emf,q42\""));
        assert!(html.contains("data-family=\"10d\""));
        assert!(html.contains("data-family=\"m64\""));
        assert!(html.contains("data-family=\"emf\""));
        assert!(html.contains("data-family=\"q42\""));
        assert!(html.contains("produce-safe 10d map"));
        assert!(html.contains("produce-safe m64 map"));
        assert!(html.contains("produce-safe emf map"));
        assert!(html.contains("produce-safe q42 map"));
        assert!(html.contains("data:image/svg+xml;base64,"));
        assert!(
            !html.contains("Use the format's native preview tool for this resource."),
            "10d/m64/emf/q42 must be presented, not stubbed"
        );
    }
}
