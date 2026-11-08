//! Project-graph walk over `.tex` file dependencies.

#![forbid(unsafe_code)]

use std::collections::{HashSet, VecDeque};
use std::path::{Path, PathBuf};

use crate::texparse::lex::{lex, Tok, Token};
use crate::texparse::structure::{group_text, opt_arg, skip_ws, Issue};

const MAX_GRAPH_DEPTH: usize = 8;
const MAX_GRAPH_FILES: usize = 256;
const IMAGE_EXTS: &[&str] = &["png", "jpg", "jpeg", "pdf", "eps"];

/// One file-level dependency extracted from a .tex source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dep {
    pub kind: DepKind,
    pub target: String,
    pub line: usize,
}

/// What kind of command produced a [`Dep`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepKind {
    Input,
    Include,
    IncludeGraphics,
    Bibliography,
    AddBibResource,
    DocumentClass,
    UsePackage,
}

impl DepKind {
    fn default_ext(self) -> &'static str {
        match self {
            DepKind::Input | DepKind::Include => "tex",
            DepKind::Bibliography | DepKind::AddBibResource => "bib",
            DepKind::UsePackage => "sty",
            DepKind::DocumentClass => "cls",
            DepKind::IncludeGraphics => "",
        }
    }
}

/// A resolved project: every reachable source file from a root.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProjectGraph {
    pub roots: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
    pub resources: Vec<PathBuf>,
    pub missing: Vec<(PathBuf, Dep)>,
}

/// Extract every file-level dependency reference from `src`.
pub fn extract_deps(src: &str) -> Vec<Dep> {
    let toks = lex(src);
    let mut deps = Vec::new();
    let mut issues: Vec<Issue> = Vec::new();
    let mut i = 0usize;
    while let Some(tok) = toks.get(i) {
        i += 1;
        let name = match &tok.kind {
            Tok::Cmd(name) => name.as_str(),
            _ => continue,
        };
        let line = tok.pos.line;
        match name {
            "input" | "include" => {
                let kind = if name == "input" { DepKind::Input } else { DepKind::Include };
                if let Some((target, ni)) = file_arg(&toks, i, name, &mut issues) {
                    i = ni;
                    push_dep(&mut deps, kind, &target, line);
                }
            }
            "includegraphics" | "documentclass" => {
                let (_opts, ni) = opt_arg(&toks, i);
                i = ni;
                let kind = if name == "includegraphics" { DepKind::IncludeGraphics } else { DepKind::DocumentClass };
                if let Some((target, ni)) = file_arg(&toks, i, name, &mut issues) {
                    i = ni;
                    push_dep(&mut deps, kind, &target, line);
                }
            }
            "usepackage" | "RequirePackage" => {
                let (_opts, ni) = opt_arg(&toks, i);
                i = ni;
                if let Some((list, ni)) = file_arg(&toks, i, name, &mut issues) {
                    i = ni;
                    for part in list.split(',') {
                        push_dep(&mut deps, DepKind::UsePackage, part, line);
                    }
                }
            }
            "bibliography" => {
                if let Some((list, ni)) = file_arg(&toks, i, name, &mut issues) {
                    i = ni;
                    for part in list.split(',') {
                        push_dep(&mut deps, DepKind::Bibliography, part, line);
                    }
                }
            }
            "addbibresource" => {
                if let Some((target, ni)) = file_arg(&toks, i, name, &mut issues) {
                    i = ni;
                    push_dep(&mut deps, DepKind::AddBibResource, &target, line);
                }
            }
            _ => {}
        }
    }
    deps
}

pub fn resolve(dep: &Dep, dir: &Path) -> PathBuf {
    let base = dir.join(&dep.target);
    let has_ext = Path::new(&dep.target).extension().is_some();
    let mut candidates: Vec<PathBuf> = Vec::new();
    if dep.kind == DepKind::IncludeGraphics {
        candidates.push(base.clone());
        if !has_ext {
            for ext in IMAGE_EXTS {
                candidates.push(base.with_extension(ext));
            }
        }
    } else if has_ext {
        candidates.push(base.clone());
    } else {
        candidates.push(base.with_extension(dep.kind.default_ext()));
        candidates.push(base.clone());
    }
    for c in candidates {
        if c.is_file() {
            return c;
        }
    }
    base
}

pub fn build(entry: &Path) -> ProjectGraph {
    let mut g = ProjectGraph::default();
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let mut queue: VecDeque<(PathBuf, usize, PathBuf, Dep)> = VecDeque::new();

    visited.insert(canon(entry));
    queue.push_back((entry.to_path_buf(), 0, entry.to_path_buf(), Dep { kind: DepKind::Input, target: file_name_lossy(entry), line: 0 }));

    while let Some((path, depth, referrer, via)) = queue.pop_front() {
        let src = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => {
                g.missing.push((referrer, via));
                continue;
            }
        };
        g.files.push(path.clone());
        if declares_document_class(&src) {
            g.roots.push(path.clone());
        }
        let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
        for dep in extract_deps(&src) {
            let resolved = resolve(&dep, &dir);
            if !resolved.is_file() {
                g.missing.push((path.clone(), dep));
                continue;
            }
            match dep.kind {
                DepKind::Input | DepKind::Include => {
                    let key = canon(&resolved);
                    if depth < MAX_GRAPH_DEPTH && visited.len() < MAX_GRAPH_FILES && visited.insert(key) {
                        queue.push_back((resolved, depth + 1, path.clone(), dep));
                    }
                }
                _ => g.resources.push(resolved),
            }
        }
    }
    g
}

pub fn declares_document_class(src: &str) -> bool {
    lex(src).iter().any(|t| matches!(&t.kind, Tok::Cmd(c) if c == "documentclass"))
}

fn file_arg(toks: &[Token], i: usize, cmd: &str, issues: &mut Vec<Issue>) -> Option<(String, usize)> {
    let j = skip_ws(toks, i);
    match toks.get(j).map(|t| &t.kind) {
        Some(Tok::LBrace) => group_text(toks, i, cmd, issues),
        Some(Tok::Text(s)) => {
            let name = s.split_whitespace().next().unwrap_or_default();
            if name.is_empty() {
                None
            } else {
                Some((name.to_string(), j + 1))
            }
        }
        _ => None,
    }
}

fn push_dep(deps: &mut Vec<Dep>, kind: DepKind, target: &str, line: usize) {
    let target = target.trim();
    if !target.is_empty() {
        deps.push(Dep { kind, target: target.to_string(), line });
    }
}

fn canon(p: &Path) -> PathBuf {
    std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf())
}

fn file_name_lossy(p: &Path) -> String {
    p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| p.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dep(kind: DepKind, target: &str, line: usize) -> Dep {
        Dep { kind, target: target.to_string(), line }
    }

    #[test]
    fn extracts_all_dep_kinds() {
        let src = "\\documentclass[12pt]{article}\n\\usepackage{amsmath, graphicx}\n\\input{ch1}\n\\include{ch2.tex}\n\\includegraphics[scale=0.5]{fig}\n\\bibliography{refs}\n\\addbibresource{extra.bib}\n";
        let deps = extract_deps(src);
        assert_eq!(
            deps,
            vec![
                dep(DepKind::DocumentClass, "article", 1),
                dep(DepKind::UsePackage, "amsmath", 2),
                dep(DepKind::UsePackage, "graphicx", 2),
                dep(DepKind::Input, "ch1", 3),
                dep(DepKind::Include, "ch2.tex", 4),
                dep(DepKind::IncludeGraphics, "fig", 5),
                dep(DepKind::Bibliography, "refs", 6),
                dep(DepKind::AddBibResource, "extra.bib", 7),
            ]
        );
    }
}
