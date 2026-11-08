//! Mathematical equation AST for OMML, LaTeX and UnicodeMath.

use serde::{Deserialize, Serialize};

pub type Arg = Vec<MNode>;

pub const MAX_DEPTH: usize = 64;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Math {
    pub nodes: Arg,
    pub jc: MathJc,
    pub linear: bool,
}

impl Math {
    pub fn new(nodes: Arg) -> Self {
        Math {
            nodes,
            jc: MathJc::Center,
            linear: false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MathJc {
    #[default]
    Center,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MSty {
    #[default]
    Plain,
    Bold,
    Italic,
    BoldItalic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MScr {
    #[default]
    Roman,
    Script,
    Fraktur,
    DoubleStruck,
    SansSerif,
    Monospace,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MRun {
    pub text: String,
    pub sty: Option<MSty>,
    pub scr: MScr,
    pub size: Option<f32>,
}

impl MRun {
    pub fn new(text: impl Into<String>) -> Self {
        MRun {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn italic(text: impl Into<String>) -> Self {
        MRun {
            text: text.into(),
            sty: Some(MSty::Italic),
            ..Default::default()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FracKind {
    #[default]
    Bar,
    Skewed,
    Linear,
    NoBar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScriptKind {
    Sup,
    Sub,
    SubSup,
    Pre,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum LimLoc {
    #[default]
    UndOvr,
    SubSup,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ColJc {
    #[default]
    Center,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t", rename_all = "camelCase")]
pub enum MNode {
    Run(MRun),
    Frac {
        kind: FracKind,
        num: Arg,
        den: Arg,
    },
    Script {
        kind: ScriptKind,
        base: Arg,
        sub: Arg,
        sup: Arg,
    },
    Rad {
        deg: Arg,
        deg_hide: bool,
        e: Arg,
    },
    Nary {
        chr: char,
        lim_loc: LimLoc,
        sub: Arg,
        sup: Arg,
        e: Arg,
    },
    Delim {
        beg: Option<char>,
        end: Option<char>,
        elems: Vec<Arg>,
    },
    Matrix {
        rows: Vec<Vec<Arg>>,
        jc: ColJc,
    },
    Acc {
        chr: char,
        e: Arg,
    },
}
