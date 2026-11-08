//! XML XFDF packet import and export (ISO 19444-1).

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt::Write as _;

use super::model::AcroForm;

/// Export AcroForm field values as an XML XFDF document.
pub fn export_xfdf(form: &AcroForm) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    let _ = writeln!(out, "<xfdf xmlns=\"http://ns.adobe.com/xfdf/\" xml:space=\"preserve\">");
    let _ = writeln!(out, "  <fields>");

    for f in &form.fields {
        let escaped_name = xml_escape(&f.name);
        let escaped_val = xml_escape(&f.value);
        let _ = writeln!(out, "    <field name=\"{}\">", escaped_name);
        let _ = writeln!(out, "      <value>{}</value>", escaped_val);
        let _ = writeln!(out, "    </field>");
    }

    let _ = writeln!(out, "  </fields>");
    let _ = writeln!(out, "</xfdf>");
    out
}

/// Import field values from an XML XFDF document into an AcroForm.
/// Returns the number of fields updated.
pub fn import_xfdf(form: &mut AcroForm, xfdf_xml: &str) -> Result<usize, String> {
    let map = parse_xfdf_fields(xfdf_xml)?;
    let mut updated = 0;

    for (name, val) in map {
        if let Some(f) = form.field_mut(&name) {
            if !f.is_read_only() {
                f.value = val;
                updated += 1;
            }
        }
    }

    Ok(updated)
}

fn parse_xfdf_fields(xml: &str) -> Result<BTreeMap<String, String>, String> {
    let mut map = BTreeMap::new();
    let mut pos = 0;

    while let Some(field_start) = xml[pos..].find("<field ") {
        let abs_field_start = pos + field_start;
        let field_header_end = xml[abs_field_start..].find('>').ok_or("Unclosed field tag")?;
        let header = &xml[abs_field_start..abs_field_start + field_header_end];

        let name = extract_attr(header, "name").ok_or("Missing name attribute in XFDF field")?;

        let val_start_tag = "<value>";
        let val_end_tag = "</value>";

        let field_end_tag = "</field>";
        let abs_field_end = xml[abs_field_start..].find(field_end_tag).ok_or("Unclosed field block")? + abs_field_start;

        let block = &xml[abs_field_start..abs_field_end];
        let val = if let Some(vs) = block.find(val_start_tag) {
            let val_content_start = vs + val_start_tag.len();
            if let Some(ve) = block[val_content_start..].find(val_end_tag) {
                xml_unescape(&block[val_content_start..val_content_start + ve])
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        map.insert(name, val);
        pos = abs_field_end + field_end_tag.len();
    }

    Ok(map)
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let pat = format!("{attr}=\"");
    let start = tag.find(&pat)? + pat.len();
    let end = tag[start..].find('"')? + start;
    Some(xml_unescape(&tag[start..end]))
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn xml_unescape(s: &str) -> String {
    s.replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}
