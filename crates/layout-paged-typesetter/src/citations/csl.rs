//! CSL (Citation Style Language) citation and bibliography model.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CitationAuthor {
    pub family: String,
    pub given: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CitationItem {
    pub id: String,
    pub title: String,
    pub authors: Vec<CitationAuthor>,
    pub year: Option<u32>,
    pub locator: Option<String>,
    pub journal: Option<String>,
    pub volume: Option<String>,
    pub pages: Option<String>,
    pub doi: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Citation {
    pub citation_id: String,
    pub items: Vec<CitationItem>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CitationStyle {
    #[default]
    AuthorDate, // APA style: (Smith, 2026, p. 42)
    Numeric,    // IEEE style: [1]
    Note,       // Chicago style: footnote with full reference
}

impl Citation {
    pub fn format(&self, style: CitationStyle, citation_number: usize) -> String {
        match style {
            CitationStyle::Numeric => {
                format!("[{citation_number}]")
            }
            CitationStyle::AuthorDate => {
                let mut parts = Vec::new();
                for item in &self.items {
                    let author_str = match item.authors.as_slice() {
                        [] => "Anon".to_string(),
                        [a] => a.family.clone(),
                        [a, b] => format!("{} & {}", a.family, b.family),
                        [a, ..] => format!("{} et al.", a.family),
                    };
                    let year_str = item.year.map(|y| y.to_string()).unwrap_or_else(|| "n.d.".into());
                    let mut s = format!("{author_str}, {year_str}");
                    if let Some(loc) = &item.locator {
                        s.push_str(&format!(", p. {loc}"));
                    }
                    parts.push(s);
                }
                format!("({})", parts.join("; "))
            }
            CitationStyle::Note => {
                let mut parts = Vec::new();
                for item in &self.items {
                    parts.push(format_full_entry(item));
                }
                parts.join("; ")
            }
        }
    }
}

pub fn format_full_entry(item: &CitationItem) -> String {
    let authors_str = if item.authors.is_empty() {
        "Anonymous".to_string()
    } else {
        item.authors
            .iter()
            .map(|a| format!("{}, {}.", a.family, a.given.chars().next().unwrap_or(' ')))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let year_str = item.year.map(|y| format!("({y})")).unwrap_or_else(|| "(n.d.)".into());
    let mut out = format!("{authors_str} {year_str}. {}.", item.title);
    if let Some(j) = &item.journal {
        out.push_str(&format!(" {j}"));
    }
    if let Some(v) = &item.volume {
        out.push_str(&format!(", {v}"));
    }
    if let Some(p) = &item.pages {
        out.push_str(&format!(", {p}"));
    }
    if let Some(doi) = &item.doi {
        out.push_str(&format!(". https://doi.org/{doi}"));
    }
    out
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Bibliography {
    pub entries: Vec<String>,
}

impl Bibliography {
    pub fn generate(items: &[CitationItem], style: CitationStyle) -> Self {
        let mut sorted = items.to_vec();
        match style {
            CitationStyle::AuthorDate => {
                sorted.sort_by(|a, b| {
                    let fa = a.authors.first().map(|x| x.family.as_str()).unwrap_or("");
                    let fb = b.authors.first().map(|x| x.family.as_str()).unwrap_or("");
                    fa.cmp(fb).then_with(|| a.year.cmp(&b.year))
                });
                let entries = sorted.iter().map(format_full_entry).collect();
                Bibliography { entries }
            }
            CitationStyle::Numeric => {
                let entries = sorted
                    .iter()
                    .enumerate()
                    .map(|(i, item)| format!("[{}] {}", i + 1, format_full_entry(item)))
                    .collect();
                Bibliography { entries }
            }
            CitationStyle::Note => {
                sorted.sort_by(|a, b| {
                    let fa = a.authors.first().map(|x| x.family.as_str()).unwrap_or("");
                    let fb = b.authors.first().map(|x| x.family.as_str()).unwrap_or("");
                    fa.cmp(fb)
                });
                let entries = sorted.iter().map(format_full_entry).collect();
                Bibliography { entries }
            }
        }
    }
}
