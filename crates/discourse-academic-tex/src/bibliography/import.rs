//! Bibliography import converter: CSL-JSON and RIS to sanitized BibTeX.

#![forbid(unsafe_code)]

use serde_json::Value;

const MAX_ENTRIES: usize = 2048;

/// Which serialization the source uses, sniffed from content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// JSON array/object of CSL items.
    CslJson,
    /// RIS tagged records (`TY  -` … `ER  -`).
    Ris,
}

/// Sniff source format: JSON openers -> CSL-JSON, `TY  -` tagged lines -> RIS.
pub fn sniff(data: &str) -> Option<Format> {
    let t = data.trim_start();
    if t.starts_with('[') || t.starts_with('{') {
        return Some(Format::CslJson);
    }
    for line in t.lines().take(8) {
        if line.starts_with("TY  -") {
            return Some(Format::Ris);
        }
        if !line.trim().is_empty() && !line.contains("  -") {
            break;
        }
    }
    None
}

/// BibTeX-safe value: braces kept balanced.
pub fn bibval(v: &str) -> String {
    let mut depth = 0i32;
    let mut out = String::with_capacity(v.len());
    for c in v.chars() {
        match c {
            '{' => {
                depth += 1;
                out.push(c);
            }
            '}' => {
                if depth > 0 {
                    depth -= 1;
                    out.push(c);
                }
            }
            _ => out.push(c),
        }
    }
    out
}

/// Emit `@kind{key, fields…}` from `(name, value)` pairs.
pub fn emit(kind: &str, key: &str, fields: &[(&str, String)]) -> String {
    let mut e = format!("@{kind}{{{key},\n");
    for (name, value) in fields {
        if !value.trim().is_empty() {
            e.push_str(&format!("  {name} = {{{}}},\n", bibval(value)));
        }
    }
    e.push_str("}\n");
    e
}

/// First 4-digit run.
pub fn year_of(raw: &str) -> String {
    raw.split(|c: char| !c.is_ascii_digit()).find(|s| s.len() == 4).unwrap_or("").to_string()
}

/// Normalized title for duplicate detection.
pub fn norm_title(t: &str) -> String {
    let n: String = t.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect::<String>().to_lowercase();
    n.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn csl_kind(v: &Value) -> &'static str {
    let t = v.get("type").and_then(Value::as_str).unwrap_or("");
    match t {
        "article-journal" | "article-magazine" | "article-newspaper" => "article",
        "book" => "book",
        "chapter" => "inbook",
        "paper-conference" => "inproceedings",
        "thesis" => {
            if v.get("genre").and_then(Value::as_str).is_some_and(|g| g.to_ascii_lowercase().contains("master")) {
                "mastersthesis"
            } else {
                "phdthesis"
            }
        }
        "report" => "techreport",
        "manuscript" => "unpublished",
        _ => "misc",
    }
}

fn csl_names(v: &Value, field: &str) -> String {
    v.get(field)
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|n| {
                    if let Some(lit) = n.get("literal").and_then(Value::as_str) {
                        return Some(lit.to_string());
                    }
                    let fam = n.get("family").and_then(Value::as_str).unwrap_or("");
                    let giv = n.get("given").and_then(Value::as_str).unwrap_or("");
                    if fam.is_empty() && giv.is_empty() {
                        None
                    } else if fam.is_empty() {
                        Some(giv.to_string())
                    } else {
                        Some(format!("{fam}, {giv}").trim_end_matches(", ").to_string())
                    }
                })
                .collect::<Vec<_>>()
                .join(" and ")
        })
        .unwrap_or_default()
}

pub fn csl_to_bibtex(v: &Value, key: &str) -> Option<String> {
    if !v.is_object() {
        return None;
    }
    let kind = csl_kind(v);
    let title = v.get("title").and_then(Value::as_str).unwrap_or("");
    let container = v.get("container-title").and_then(Value::as_str).unwrap_or("");
    if title.is_empty() && container.is_empty() {
        return None;
    }
    let venue_field = if matches!(kind, "article") { "journal" } else { "booktitle" };
    let year = v
        .get("issued")
        .and_then(|i| {
            i.get("date-parts")
                .and_then(Value::as_array)
                .and_then(|dp| dp.first())
                .and_then(Value::as_array)
                .and_then(|d| d.first())
                .and_then(Value::as_i64)
        })
        .map(|y| y.to_string())
        .or_else(|| v.get("issued").and_then(|i| i.get("raw")).and_then(Value::as_str).map(year_of))
        .unwrap_or_default();
    let fields: Vec<(&str, String)> = vec![
        ("title", title.to_string()),
        ("author", csl_names(v, "author")),
        ("editor", csl_names(v, "editor")),
        ("year", year),
        (venue_field, container.to_string()),
        ("series", v.get("collection-title").and_then(Value::as_str).unwrap_or("").to_string()),
        ("volume", v.get("volume").and_then(Value::as_str).unwrap_or("").to_string()),
        ("number", v.get("issue").and_then(Value::as_str).unwrap_or("").to_string()),
        ("pages", v.get("page").and_then(Value::as_str).unwrap_or("").replace('-', "--")),
        ("publisher", v.get("publisher").and_then(Value::as_str).unwrap_or("").to_string()),
        ("address", v.get("publisher-place").and_then(Value::as_str).unwrap_or("").to_string()),
        ("edition", v.get("edition").and_then(Value::as_str).unwrap_or("").to_string()),
        ("doi", v.get("DOI").and_then(Value::as_str).unwrap_or("").to_string()),
        ("isbn", v.get("ISBN").and_then(Value::as_str).unwrap_or("").to_string()),
        ("issn", v.get("ISSN").and_then(Value::as_str).unwrap_or("").to_string()),
        ("url", v.get("URL").and_then(Value::as_str).unwrap_or("").to_string()),
        ("abstract", v.get("abstract").and_then(Value::as_str).unwrap_or("").to_string()),
    ];
    Some(emit(kind, key, &fields))
}

pub fn ris_kind(t: &str) -> &'static str {
    match t {
        "JOUR" | "JFULL" | "MGZN" | "NEWS" => "article",
        "BOOK" => "book",
        "CHAP" | "SECT" => "inbook",
        "CONF" | "CPAPER" => "inproceedings",
        "THES" => "phdthesis",
        "RPRT" => "techreport",
        "UNPB" => "unpublished",
        _ => "misc",
    }
}

pub fn ris_entries(data: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut kind = "";
    let mut authors: Vec<String> = Vec::new();
    let mut fields: Vec<(&'static str, String)> = Vec::new();
    let (mut sp, mut ep) = (String::new(), String::new());

    fn flush(kind: &str, authors: &[String], fields: &[(&'static str, String)], sp: &str, ep: &str, out: &mut Vec<String>) {
        if kind.is_empty() {
            return;
        }
        let mut f: Vec<(&'static str, String)> = fields.to_vec();
        f.insert(0, ("author", authors.join(" and ")));
        if !sp.is_empty() {
            f.push(("pages", if ep.is_empty() { sp.to_string() } else { format!("{sp}--{ep}") }));
        }
        if f.iter().any(|(n, v)| n != &"author" && !v.is_empty()) {
            let key = format!("ref{}", out.len() + 1);
            out.push(emit(ris_kind(kind), &key, &f));
        }
    }

    for line in data.lines().take(MAX_ENTRIES * 64) {
        let Some(tag) = line.get(..2) else { continue };
        match tag {
            "TY" => {
                flush(kind, &authors, &fields, &sp, &ep, &mut out);
                kind = line.get(6..).unwrap_or("").trim();
                authors.clear();
                fields.clear();
                sp.clear();
                ep.clear();
                continue;
            }
            "ER" => {
                flush(kind, &authors, &fields, &sp, &ep, &mut out);
                kind = "";
                authors.clear();
                fields.clear();
                sp.clear();
                ep.clear();
                if out.len() >= MAX_ENTRIES {
                    break;
                }
                continue;
            }
            _ => {}
        }
        let Some(sep) = line.get(2..6) else { continue };
        if sep != "  - " && sep != "  -" {
            continue;
        }
        let val = line.get(6..).unwrap_or("").trim().to_string();
        match tag {
            "AU" | "A1" => authors.push(val),
            "ED" | "A2" => fields.push(("editor", val)),
            "TI" | "T1" | "CT" => fields.push(("title", val)),
            "T2" | "JO" | "JF" | "JA" | "BT" => fields.push((if ris_kind(kind) == "article" { "journal" } else { "booktitle" }, val)),
            "PY" | "Y1" | "DA" => fields.push(("year", year_of(&val))),
            "VL" => fields.push(("volume", val)),
            "IS" => fields.push(("number", val)),
            "SP" => sp = val,
            "EP" => ep = val,
            "DO" => fields.push(("doi", val)),
            "UR" | "L1" | "L2" => fields.push(("url", val)),
            "SN" => fields.push((if ris_kind(kind) == "book" || ris_kind(kind) == "inbook" { "isbn" } else { "issn" }, val)),
            "PB" => fields.push(("publisher", val)),
            "CY" | "PP" => fields.push(("address", val)),
            "ET" => fields.push(("edition", val)),
            "AB" | "N2" => fields.push(("abstract", val)),
            _ => {}
        }
    }
    flush(kind, &authors, &fields, &sp, &ep, &mut out);
    out
}

/// Convert source buffer (CSL-JSON or RIS) to BibTeX entries.
pub fn convert(data: &str) -> Result<(Format, Vec<String>), String> {
    let fmt = sniff(data).ok_or_else(|| "unrecognized bibliography format".to_string())?;
    let entries = match fmt {
        Format::CslJson => {
            let v: Value = serde_json::from_str(data).map_err(|e| format!("invalid JSON: {e}"))?;
            let items: Vec<Value> = match v {
                Value::Array(a) => a,
                o => vec![o],
            };
            let mut out = Vec::new();
            for (idx, item) in items.iter().take(MAX_ENTRIES).enumerate() {
                let key = format!("csl{}", idx + 1);
                if let Some(e) = csl_to_bibtex(item, &key) {
                    out.push(e);
                }
            }
            out
        }
        Format::Ris => ris_entries(data),
    };
    Ok((fmt, entries))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sniff() {
        assert_eq!(sniff("[{\"type\": \"article\"}]"), Some(Format::CslJson));
        assert_eq!(sniff("TY  - JOUR\nTI  - Test\nER  -"), Some(Format::Ris));
    }

    #[test]
    fn test_ris_conversion() {
        let ris = "TY  - JOUR\nTI  - Quantum Computing\nAU  - Turing, A.\nPY  - 1950\nDO  - 10.1000/182\nER  -";
        let (fmt, entries) = convert(ris).unwrap();
        assert_eq!(fmt, Format::Ris);
        assert_eq!(entries.len(), 1);
        assert!(entries[0].contains("title = {Quantum Computing}"));
        assert!(entries[0].contains("author = {Turing, A.}"));
        assert!(entries[0].contains("year = {1950}"));
    }
}
