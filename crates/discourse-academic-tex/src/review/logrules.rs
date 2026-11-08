//! TeX log diagnostic enricher.
//!
//! Rule-based data-driven engine mapping TeX log signatures to:
//! - Root cause rule IDs
//! - Plain-language explanations
//! - Suggested missing CTAN/LaTeX packages
//! - Cascade noise suppression.

#![forbid(unsafe_code)]

/// A diagnostic entry from TeX log parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeXDiagnostic {
    pub file: Option<String>,
    pub line: Option<usize>,
    pub message: String,
    pub rule: Option<&'static str>,
    pub hint: Option<String>,
    pub package: Option<String>,
    pub command: Option<String>,
    pub cascade: bool,
}

impl TeXDiagnostic {
    pub fn new(message: &str) -> Self {
        Self {
            file: None,
            line: None,
            message: message.to_string(),
            rule: None,
            hint: None,
            package: None,
            command: None,
            cascade: false,
        }
    }
}

struct Rule {
    id: &'static str,
    has: &'static [&'static str],
    hint: &'static str,
    cascade_on_prior: bool,
}

const RULES: &[Rule] = &[
    Rule {
        id: "missing-file",
        has: &["File `", "' not found"],
        hint: "`{file}` isn't on the TeX search path — a missing .sty/.cls usually means a package to install or load; a missing .tex/image means the path is wrong",
        cascade_on_prior: false,
    },
    Rule {
        id: "undefined-environment",
        has: &["Environment", "undefined"],
        hint: "the environment isn't defined — check the name or load the package that provides it",
        cascade_on_prior: false,
    },
    Rule {
        id: "undefined-command",
        has: &["Undefined control sequence"],
        hint: "`{cmd}` isn't defined — check the spelling, or load the package that provides it",
        cascade_on_prior: false,
    },
    Rule {
        id: "math-outside-math",
        has: &["Missing $ inserted"],
        hint: "math content outside math mode — wrap it in \\(…\\), $…$ or a math environment",
        cascade_on_prior: false,
    },
    Rule {
        id: "missing-begin-document",
        has: &["Missing \\begin{document}"],
        hint: "typeset material appeared before \\begin{document} — or the document body is missing entirely",
        cascade_on_prior: false,
    },
    Rule {
        id: "stray-newline",
        has: &["There's no line here to end"],
        hint: "a `\\\\` line break sits where no line is open — remove it or leave a blank line instead",
        cascade_on_prior: false,
    },
    Rule {
        id: "incomplete-arg",
        has: &["Paragraph ended before"],
        hint: "a command argument ran past a paragraph break — usually an unclosed brace `{`",
        cascade_on_prior: false,
    },
    Rule {
        id: "misplaced-align-tab",
        has: &["Misplaced alignment tab"],
        hint: "`&` is only legal inside tabular/align environments — escape it as `\\&` in text",
        cascade_on_prior: false,
    },
    Rule {
        id: "brace-mismatch",
        has: &["Extra }, or forgotten"],
        hint: "a `}` has no matching `{` — check the braces around this line",
        cascade_on_prior: false,
    },
    Rule {
        id: "option-clash",
        has: &["Option clash for package"],
        hint: "the package was loaded twice with conflicting options — drop the earlier \\usepackage or move its options there",
        cascade_on_prior: false,
    },
    Rule {
        id: "font-not-found",
        has: &["Font", "not found"],
        hint: "the font isn't installed — install it system-wide or pick another with fontspec",
        cascade_on_prior: false,
    },
    Rule {
        id: "missing-glyph",
        has: &["Missing character:"],
        hint: "the current font has no glyph for this character — pick a covering font or a Unicode-aware engine",
        cascade_on_prior: false,
    },
    Rule {
        id: "undefined-reference",
        has: &["Reference `", "undefined"],
        hint: "no \\label defines `{x}` — check the key or compile once more",
        cascade_on_prior: false,
    },
    Rule {
        id: "undefined-citation",
        has: &["Citation `", "undefined"],
        hint: "key `{x}` isn't in any project .bib — check the key or add the entry",
        cascade_on_prior: false,
    },
    Rule {
        id: "overfull-hbox",
        has: &["Overfull \\hbox"],
        hint: "a line ran past the margin — rephrase, allow a break, or widen the measure",
        cascade_on_prior: false,
    },
    Rule {
        id: "emergency-stop",
        has: &["Emergency stop"],
        hint: "compilation aborted — consequence of the error above",
        cascade_on_prior: true,
    },
    Rule {
        id: "fatal-error",
        has: &["Fatal error"],
        hint: "fatal engine error — consequence of the error above",
        cascade_on_prior: true,
    },
];

/// Enrich diagnostic items in place.
pub fn enrich_diagnostics(errors: &mut [TeXDiagnostic], warnings: &mut [TeXDiagnostic]) {
    let mut any_prior = false;
    for diag in errors.iter_mut().chain(warnings.iter_mut()) {
        if let Some(rule) = RULES.iter().find(|r| r.has.iter().all(|s| diag.message.contains(s))) {
            diag.rule = Some(rule.id);
            diag.hint = Some(fill(rule.hint, &diag.message));
            diag.package = suggested_package(rule.id, &diag.message);
            diag.command = extract_command(&diag.message);
            diag.cascade = rule.cascade_on_prior && any_prior;
        }
        any_prior = true;
    }
}

fn fill(hint: &str, msg: &str) -> String {
    hint.replace("{x}", &extract_quoted(msg).unwrap_or_default())
        .replace("{file}", &extract_quoted(msg).unwrap_or_default())
        .replace("{cmd}", &extract_command(msg).unwrap_or_default())
}

fn extract_quoted(msg: &str) -> Option<String> {
    let start = msg.find('`')? + 1;
    let rest = msg.get(start..)?;
    let end = rest.find('\'')?;
    rest.get(..end).map(str::to_string).filter(|s| !s.is_empty())
}

fn extract_command(msg: &str) -> Option<String> {
    msg.rsplit('\\')
        .next()
        .map(|t| t.chars().take_while(|c| c.is_ascii_alphabetic()).collect::<String>())
        .filter(|s| !s.is_empty())
        .map(|s| format!("\\{s}"))
        .filter(|s| msg.contains(s.as_str()))
}

fn suggested_package(rule: &str, msg: &str) -> Option<String> {
    if rule != "missing-file" {
        return None;
    }
    let file = extract_quoted(msg)?;
    [".sty", ".cls"].iter().find_map(|ext| file.strip_suffix(ext).map(str::to_string))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undefined_command_gets_hint() {
        let mut errs = vec![TeXDiagnostic::new("! Undefined control sequence.\nl.7 \\badcmd")];
        let mut warns = Vec::new();
        enrich_diagnostics(&mut errs, &mut warns);
        assert_eq!(errs[0].rule, Some("undefined-command"));
        assert!(errs[0].hint.is_some());
        assert!(!errs[0].cascade);
    }

    #[test]
    fn emergency_stop_is_cascade() {
        let mut errs = vec![TeXDiagnostic::new("! Undefined control sequence.\nl.7 \\badcmd"), TeXDiagnostic::new("! Emergency stop.")];
        let mut warns = Vec::new();
        enrich_diagnostics(&mut errs, &mut warns);
        assert!(errs[1].cascade);
        assert_eq!(errs[1].rule, Some("emergency-stop"));
    }
}
