//! OpenXML PresentationML (.pptx) presentation reader and writer.

use crate::xml::dom::{escape_xml, parse_xml, XEl, XNode};
use crate::zip::minizip::ZipPackage;
use layout_presentation_deck::model::master::{Presentation, Slide};
use layout_presentation_deck::model::shape::{Paragraph, Shape, ShapeContent, TextBody};
use layout_presentation_deck::model::types::{LayoutId, ShapeId, SlideId, Xfrm};

/// Serialize a `Presentation` into an OpenXML `.pptx` ZIP package.
pub fn to_pptx_package(pres: &Presentation) -> ZipPackage {
    let mut pkg = ZipPackage::new();

    let slide_count = pres.slides.len().max(1);

    // 1. [Content_Types].xml
    let mut ct = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">\n");
    ct.push_str("  <Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\n");
    ct.push_str("  <Default Extension=\"xml\" ContentType=\"application/xml\"/>\n");
    ct.push_str("  <Override PartName=\"/ppt/presentation.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml\"/>\n");
    for i in 1..=slide_count {
        ct.push_str(&format!("  <Override PartName=\"/ppt/slides/slide{}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>\n", i));
    }
    ct.push_str("</Types>");
    pkg.insert("[Content_Types].xml", ct.into_bytes());

    // 2. _rels/.rels
    let root_rels = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n  <Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" Target=\"ppt/presentation.xml\"/>\n</Relationships>";
    pkg.insert("_rels/.rels", root_rels.as_bytes().to_vec());

    // 3. ppt/_rels/presentation.xml.rels
    let mut pres_rels = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n");
    for i in 1..=slide_count {
        pres_rels.push_str(&format!("  <Relationship Id=\"rId{}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide{}.xml\"/>\n", i, i));
    }
    pres_rels.push_str("</Relationships>");
    pkg.insert("ppt/_rels/presentation.xml.rels", pres_rels.into_bytes());

    // 4. ppt/presentation.xml
    let mut pres_xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<p:presentation xmlns:p=\"http://schemas.openxmlformats.org/presentationml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\">\n  <p:sldIdLst>\n");
    for i in 1..=slide_count {
        pres_xml.push_str(&format!("    <p:sldId id=\"{}\" r:id=\"rId{}\"/>\n", 255 + i, i));
    }
    pres_xml.push_str("  </p:sldIdLst>\n</p:presentation>");
    pkg.insert("ppt/presentation.xml", pres_xml.into_bytes());

    // 5. ppt/slides/slide{i}.xml
    let default_slide = Slide::new(SlideId(1), LayoutId(1));
    let slides_slice = if pres.slides.is_empty() {
        std::slice::from_ref(&default_slide)
    } else {
        pres.slides.as_slice()
    };

    for (s_idx, slide) in slides_slice.iter().enumerate() {
        let slide_num = s_idx + 1;
        let mut sld_xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<p:sld xmlns:p=\"http://schemas.openxmlformats.org/presentationml/2006/main\" xmlns:a=\"http://schemas.openxmlformats.org/drawingml/2006/main\">\n  <p:cSld>\n    <p:spTree>\n      <p:nvGrpSpPr><p:cNvPr id=\"1\" name=\"\"/><p:cNvGrpSpPr/><p:grpSpPr/></p:nvGrpSpPr>\n      <p:grpSpPr/>\n");

        for shape in &slide.shapes {
            sld_xml.push_str("      <p:sp>\n");
            sld_xml.push_str(&format!("        <p:nvSpPr><p:cNvPr id=\"{}\" name=\"{}\"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>\n", shape.id.0, escape_attr(&shape.name)));
            sld_xml.push_str("        <p:spPr>\n");
            // Points to EMUs (1 pt = 12700 EMUs)
            let emu_x = (shape.xfrm.x * 12700.0) as i64;
            let emu_y = (shape.xfrm.y * 12700.0) as i64;
            let emu_w = (shape.xfrm.w * 12700.0) as i64;
            let emu_h = (shape.xfrm.h * 12700.0) as i64;
            sld_xml.push_str(&format!("          <a:xfrm><a:off x=\"{}\" y=\"{}\"/><a:ext cx=\"{}\" cy=\"{}\"/></a:xfrm>\n", emu_x, emu_y, emu_w, emu_h));
            sld_xml.push_str("        </p:spPr>\n");

            // ECMA-376 forbids U+000A in a:t; one a:p per paragraph.
            if let ShapeContent::Text(tb) = &shape.content {
                sld_xml.push_str("        <p:txBody><a:bodyPr/><a:lstStyle/>");
                emit_txbody_paragraphs(&tb.paragraphs, &mut sld_xml);
                sld_xml.push_str("</p:txBody>\n");
            }
            sld_xml.push_str("      </p:sp>\n");
        }

        sld_xml.push_str("    </p:spTree>\n  </p:cSld>\n</p:sld>");
        pkg.insert(format!("ppt/slides/slide{}.xml", slide_num), sld_xml.into_bytes());
    }

    pkg
}

fn escape_attr(s: &str) -> String {
    let mut out = String::new();
    escape_xml(s, &mut out);
    out
}

fn emit_txbody_paragraphs(paragraphs: &[Paragraph], out: &mut String) {
    if paragraphs.is_empty() {
        out.push_str("<a:p/>");
        return;
    }
    for para in paragraphs {
        let txt = para.text();
        if txt.is_empty() {
            out.push_str("<a:p/>");
            continue;
        }
        for line in txt.split('\n') {
            out.push_str("<a:p>");
            if !line.is_empty() {
                out.push_str("<a:r><a:t>");
                escape_xml(line, out);
                out.push_str("</a:t></a:r>");
            }
            out.push_str("</a:p>");
        }
    }
}

/// Parse an OpenXML `.pptx` ZIP package into a `Presentation`.
pub fn parse_pptx_package(pkg: &ZipPackage) -> Result<Presentation, String> {
    let mut pres = Presentation::default();
    let mut slide_idx = 1;

    while let Some(sld_bytes) = pkg.get(&format!("ppt/slides/slide{}.xml", slide_idx)) {
        let sld_str = String::from_utf8_lossy(sld_bytes);
        if let Ok(root) = parse_xml(&sld_str) {
            let mut slide = Slide::new(SlideId(slide_idx as u32), LayoutId(1));

            if let Some(csld) = root.find_child("cSld") {
                if let Some(sptree) = csld.find_child("spTree") {
                    for sp_el in sptree.find_children("sp") {
                        let mut shape_id = ShapeId(0);
                        let mut shape_name = String::new();
                        let mut xfrm = Xfrm::default();

                        if let Some(nvsp) = sp_el.find_child("nvSpPr") {
                            if let Some(cnv) = nvsp.find_child("cNvPr") {
                                if let Some(id_str) = cnv.attr("id") {
                                    shape_id = ShapeId(id_str.parse::<u32>().unwrap_or(0));
                                }
                                shape_name = cnv.attr("name").unwrap_or("").to_string();
                            }
                        }

                        if let Some(sppr) = sp_el.find_child("spPr") {
                            if let Some(xfrm_el) = sppr.find_child("xfrm") {
                                if let Some(off) = xfrm_el.find_child("off") {
                                    let x_emu = off.attr("x").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
                                    let y_emu = off.attr("y").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
                                    xfrm.x = x_emu / 12700.0;
                                    xfrm.y = y_emu / 12700.0;
                                }
                                if let Some(ext) = xfrm_el.find_child("ext") {
                                    let cx_emu = ext.attr("cx").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
                                    let cy_emu = ext.attr("cy").and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
                                    xfrm.w = cx_emu / 12700.0;
                                    xfrm.h = cy_emu / 12700.0;
                                }
                            }
                        }

                        let mut s = Shape::new(shape_id, shape_name, xfrm);
                        if let Some(txbody) = sp_el.find_child("txBody") {
                            let mut tb = TextBody::default();
                            for p_el in txbody.find_children("p") {
                                let mut para_text = String::new();
                                for r_el in p_el.find_children("r") {
                                    for t_el in r_el.find_children("t") {
                                        para_text.push_str(&t_el.text_content());
                                    }
                                }
                                tb.paragraphs.push(Paragraph::new(para_text));
                            }
                            if tb.paragraphs.iter().any(|p| !p.text().is_empty()) {
                                s.content = ShapeContent::Text(tb);
                            }
                        }
                        slide.shapes.push(s);
                    }
                }
            }

            pres.slides.push(slide);
        }
        slide_idx += 1;
    }

    if pres.slides.is_empty() {
        return Err("No slides found in pptx package".into());
    }

    Ok(pres)
}
