//! Bounded foliation and discourse document morphisms: PDF compilation,
//! OpenXML DOCX prose compilation, OpenXML PPTX slide deck compilation,
//! and standalone SVG data visualization chart rendering for the `discourse` category.

use crate::*;
use std::collections::BTreeMap;
use std::fmt::Write;

mod docx_pptx;
mod pdf_chart;

// ============================================================================
// Deterministic IEEE 802.3 CRC-32 & Pure-Rust STORE ZIP Archive Writer
// ============================================================================

/// Branchless IEEE 802.3 CRC-32 checksum (polynomial 0xEDB88320).
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

pub(super) struct ZipEntry {
    pub(super) name: String,
    pub(super) data: Vec<u8>,
}

/// Pure-Rust uncompressed (STORE method 0) ZIP writer with zero external dependencies.
pub(super) struct ZipArchiveWriter {
    pub(super) entries: Vec<ZipEntry>,
}

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
            | "gallery" => self.gallery(a),

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
        let total_objs = 4 + 2 * page_count;

        // Render page content streams
        let margin = 54.0;
        let mut page_streams = Vec::with_capacity(page_count);

        for page in &pages {
            let mut stream = String::new();

            // Vector rectangles
            for rect in &page.rects {
                let _ = write!(
                    stream,
                    "q\n{:.4} {:.4} {:.4} rg\n{:.2} {:.2} {:.2} {:.2} re f\nQ\n",
                    rect.fill_rgb[0],
                    rect.fill_rgb[1],
                    rect.fill_rgb[2],
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h
                );
            }

            let mut cur_y = height_pt - margin;

            // Heading with Helvetica-Bold 18pt
            if let Some(ref heading) = page.heading {
                cur_y -= 20.0;
                let escaped = escape_pdf_str(heading);
                let _ = write!(
                    stream,
                    "BT\n/F2 18 Tf\n1 0 0 1 {:.2} {:.2} Tm\n({}) Tj\nET\n",
                    margin, cur_y, escaped
                );
                cur_y -= 14.0;
            }

            // Body lines with Helvetica 11pt
            for line in &page.lines {
                cur_y -= 15.0;
                if cur_y < margin {
                    break;
                }
                let escaped = escape_pdf_str(line);
                let _ = write!(
                    stream,
                    "BT\n/F1 11 Tf\n1 0 0 1 {:.2} {:.2} Tm\n({}) Tj\nET\n",
                    margin, cur_y, escaped
                );
            }

            page_streams.push(stream);
        }

        // Build deterministic PDF-1.7 byte stream
        let mut pdf = Vec::new();
        pdf.extend_from_slice(b"%PDF-1.7\n%\xE2\xE3\xCF\xD3\n");
        let mut offsets = Vec::with_capacity(total_objs);

        // Object 1: Catalog
        offsets.push(pdf.len());
        pdf.extend_from_slice(b"1 0 obj\n<<\n  /Type /Catalog\n  /Pages 2 0 R\n>>\nendobj\n");

        // Object 2: Pages Root
        offsets.push(pdf.len());
        let mut kids = String::new();
        for i in 0..page_count {
            let _ = write!(kids, "{} 0 R ", 5 + 2 * i);
        }
        let pages_obj = format!(
            "2 0 obj\n<<\n  /Type /Pages\n  /Kids [ {}]\n  /Count {}\n>>\nendobj\n",
            kids, page_count
        );
        pdf.extend_from_slice(pages_obj.as_bytes());

        // Object 3: Font Helvetica (/F1)
        offsets.push(pdf.len());
        pdf.extend_from_slice(
            b"3 0 obj\n<<\n  /Type /Font\n  /Subtype /Type1\n  /BaseFont /Helvetica\n>>\nendobj\n",
        );

        // Object 4: Font Helvetica-Bold (/F2)
        offsets.push(pdf.len());
        pdf.extend_from_slice(
            b"4 0 obj\n<<\n  /Type /Font\n  /Subtype /Type1\n  /BaseFont /Helvetica-Bold\n>>\nendobj\n",
        );

        // Page and Contents objects
        for (i, stream_content) in page_streams.iter().enumerate() {
            let page_id = 5 + 2 * i;
            let contents_id = 6 + 2 * i;

            // Page object
            offsets.push(pdf.len());
            let page_obj = format!(
                "{page_id} 0 obj\n<<\n  /Type /Page\n  /Parent 2 0 R\n  /MediaBox [0 0 {width_pt:.2} {height_pt:.2}]\n  /Resources <<\n    /Font <<\n      /F1 3 0 R\n      /F2 4 0 R\n    >>\n  >>\n  /Contents {contents_id} 0 R\n>>\nendobj\n"
            );
            pdf.extend_from_slice(page_obj.as_bytes());

            // Contents stream object
            offsets.push(pdf.len());
            let stream_hdr = format!("{contents_id} 0 obj\n<< /Length {} >>\nstream\n", stream_content.len());
            pdf.extend_from_slice(stream_hdr.as_bytes());
            pdf.extend_from_slice(stream_content.as_bytes());
            pdf.extend_from_slice(b"\nendstream\nendobj\n");
        }

        // Xref table
        let start_xref = pdf.len();
        pdf.extend_from_slice(b"xref\n");
        let xref_header = format!("0 {}\n", total_objs + 1);
        pdf.extend_from_slice(xref_header.as_bytes());
        pdf.extend_from_slice(b"0000000000 65535 f \r\n");
        for off in &offsets {
            let entry = format!("{:010} 00000 n \r\n", off);
            pdf.extend_from_slice(entry.as_bytes());
        }

        // Trailer
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

        Ok(Value::Record(rec))
    }
}
