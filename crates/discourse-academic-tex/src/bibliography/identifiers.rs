//! Academic identifier parser: DOI, arXiv, PMID, PMCID.
//!
//! Accepts DOIs in any written form (bare, doi: prefix, doi.org URL),
//! arXiv IDs (old and new, with or without version), and PMID/PMCID.
//! Scrubbing removes zero-width characters and non-breaking spaces.

#![forbid(unsafe_code)]

const MAX_ID_LEN: usize = 512;

/// A normalized academic identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ident {
    /// Canonical lowercase DOI, no scheme/prefix.
    Doi(String),
    /// arXiv id without version (`2501.00663` or `hep-th/9901001`).
    Arxiv(String),
    /// Numeric PubMed id.
    Pmid(String),
    /// `PMC…` PubMed Central id.
    Pmcid(String),
}

fn scrub(text: &str) -> String {
    text.chars().filter(|c| !matches!(c, '\u{200b}'..='\u{200f}' | '\u{feff}' | '\u{00a0}' | '\u{2060}')).collect()
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' {
            if let (Some(h), Some(l)) = (b.get(i + 1), b.get(i + 2)) {
                if let Ok(v) = u8::from_str_radix(&format!("{}{}", *h as char, *l as char), 16) {
                    out.push(v);
                    i += 3;
                    continue;
                }
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn looks_like_doi(s: &str) -> bool {
    let Some(rest) = s.strip_prefix("10.") else { return false };
    let Some(slash) = rest.find('/') else { return false };
    slash >= 4 && rest[..slash].chars().all(|c| c.is_ascii_digit()) && slash + 1 < rest.len()
}

fn looks_like_arxiv(s: &str) -> bool {
    let body = arxiv_body(s);
    if let Some((y, rest)) = body.split_once('.') {
        return y.len() == 4 && y.chars().all(|c| c.is_ascii_digit()) && rest.len() >= 4 && rest.chars().all(|c| c.is_ascii_digit());
    }
    if let Some((arch, rest)) = body.split_once('/') {
        return !arch.is_empty()
            && arch.chars().all(|c| c.is_ascii_lowercase() || c == '-')
            && rest.len() >= 6
            && rest.chars().all(|c| c.is_ascii_digit());
    }
    false
}

fn arxiv_body(s: &str) -> String {
    match s.rsplit_once('v') {
        Some((base, v)) if !base.is_empty() && v.chars().all(|c| c.is_ascii_digit()) => base.to_string(),
        _ => s.to_string(),
    }
}

/// Parse any pasted identifier into a normalized [`Ident`].
pub fn parse_identifier(raw: &str) -> Option<Ident> {
    let s = scrub(raw);
    let s = s.trim();
    if s.is_empty() || s.chars().count() > MAX_ID_LEN {
        return None;
    }
    let low = s.to_ascii_lowercase();

    for (host, kind) in [
        ("doi.org/", "doi"),
        ("dx.doi.org/", "doi"),
        ("arxiv.org/abs/", "arxiv"),
        ("arxiv.org/pdf/", "arxiv"),
        ("pubmed.ncbi.nlm.nih.gov/", "pmid"),
        ("ncbi.nlm.nih.gov/pmc/articles/", "pmcid"),
    ] {
        if let Some(pos) = low.find(host) {
            let path = s.get(pos + host.len()..).unwrap_or("");
            let path = path.split(['?', '#', ' ']).next().unwrap_or("").trim_end_matches(['/', '.', ';', ',', ')']);
            let path = path.strip_suffix(".pdf").unwrap_or(path);
            let dec = percent_decode(path);
            return match kind {
                "doi" if looks_like_doi(&dec) => Some(Ident::Doi(dec.to_ascii_lowercase())),
                "arxiv" if looks_like_arxiv(&dec) => Some(Ident::Arxiv(arxiv_body(&dec))),
                "pmid" if path.chars().all(|c| c.is_ascii_digit()) => Some(Ident::Pmid(path.to_string())),
                "pmcid" => {
                    let id = path.trim_start_matches("PMC");
                    if !id.is_empty() && id.chars().all(|c| c.is_ascii_digit()) {
                        Some(Ident::Pmcid(format!("PMC{id}")))
                    } else {
                        None
                    }
                }
                _ => None,
            };
        }
    }

    for (p, kind) in [("doi:", "doi"), ("doi ", "doi"), ("arxiv:", "arxiv"), ("pmid:", "pmid"), ("pmcid:", "pmcid")] {
        if let Some(rest) = low.strip_prefix(p) {
            let rest = s.get(s.len().saturating_sub(rest.len())..).unwrap_or("").trim();
            return match kind {
                "doi" if looks_like_doi(rest) => Some(Ident::Doi(rest.to_ascii_lowercase())),
                "arxiv" if looks_like_arxiv(rest) => Some(Ident::Arxiv(arxiv_body(rest))),
                "pmid" if rest.chars().all(|c| c.is_ascii_digit()) => Some(Ident::Pmid(rest.to_string())),
                "pmcid" => {
                    let id = rest.trim_start_matches("PMC");
                    if !id.is_empty() && id.chars().all(|c| c.is_ascii_digit()) {
                        Some(Ident::Pmcid(format!("PMC{id}")))
                    } else {
                        None
                    }
                }
                _ => None,
            };
        }
    }

    if looks_like_doi(s) {
        return Some(Ident::Doi(s.to_ascii_lowercase()));
    }
    if looks_like_arxiv(s) {
        return Some(Ident::Arxiv(arxiv_body(s)));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doi_forms() {
        assert_eq!(parse_identifier("10.1038/nature12373"), Some(Ident::Doi("10.1038/nature12373".into())));
        assert_eq!(parse_identifier("doi:10.1145/3292500.3330670"), Some(Ident::Doi("10.1145/3292500.3330670".into())));
        assert_eq!(parse_identifier("DOI 10.1000/xyz"), Some(Ident::Doi("10.1000/xyz".into())));
        assert_eq!(parse_identifier("https://doi.org/10.1109/5.771073"), Some(Ident::Doi("10.1109/5.771073".into())));
    }

    #[test]
    fn arxiv_forms() {
        assert_eq!(parse_identifier("2501.00663"), Some(Ident::Arxiv("2501.00663".into())));
        assert_eq!(parse_identifier("arXiv:2501.00663v2"), Some(Ident::Arxiv("2501.00663".into())));
        assert_eq!(parse_identifier("https://arxiv.org/abs/hep-th/9901001"), Some(Ident::Arxiv("hep-th/9901001".into())));
    }

    #[test]
    fn pubmed_forms() {
        assert_eq!(parse_identifier("pmid:12345678"), Some(Ident::Pmid("12345678".into())));
        assert_eq!(parse_identifier("https://pubmed.ncbi.nlm.nih.gov/12345678/"), Some(Ident::Pmid("12345678".into())));
        assert_eq!(parse_identifier("PMC1234567"), Some(Ident::Pmcid("PMC1234567".into())));
    }
}
