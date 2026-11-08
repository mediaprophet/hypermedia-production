//! Lottie vector animation schema definitions (Bodymovin / LottieFiles 1.0).

use serde::{Deserialize, Serialize};

/// Root Lottie animation document.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LottieAnimation {
    pub v: String,
    pub fr: f64,
    pub ip: f64,
    pub op: f64,
    pub w: f64,
    pub h: f64,
    pub nm: Option<String>,
    #[serde(default)]
    pub layers: Vec<LottieLayer>,
}

/// A layer in a Lottie composition.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LottieLayer {
    /// Layer type: 0=Precomp, 1=Solid, 2=Image, 3=Null, 4=Shape, 5=Text.
    pub ty: u32,
    pub nm: String,
    pub ip: f64,
    pub op: f64,
    #[serde(default)]
    pub st: f64,
    #[serde(default = "default_sr")]
    pub sr: f64,
    #[serde(default)]
    pub ks: LottieTransform,
    #[serde(default)]
    pub shapes: Vec<LottieShape>,
}

fn default_sr() -> f64 {
    1.0
}

/// Animated transform properties of a layer.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LottieTransform {
    #[serde(default)]
    pub a: LottiePropMulti,
    #[serde(default)]
    pub p: LottiePropMulti,
    #[serde(default)]
    pub s: LottiePropMulti,
    #[serde(default)]
    pub r: LottiePropScalar,
    #[serde(default)]
    pub o: LottiePropScalar,
}

/// A multi-dimensional animated property.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LottiePropMulti {
    #[serde(default)]
    pub a: u32,
    pub k: serde_json::Value,
}

/// A scalar animated property.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LottiePropScalar {
    #[serde(default)]
    pub a: u32,
    pub k: serde_json::Value,
}

/// A vector shape item inside a shape layer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "ty")]
pub enum LottieShape {
    #[serde(rename = "rc")]
    Rect {
        nm: Option<String>,
        p: LottiePropMulti,
        s: LottiePropMulti,
        r: LottiePropScalar,
    },
    #[serde(rename = "el")]
    Ellipse {
        nm: Option<String>,
        p: LottiePropMulti,
        s: LottiePropMulti,
    },
    #[serde(rename = "fl")]
    Fill {
        nm: Option<String>,
        c: LottiePropMulti,
        o: LottiePropScalar,
    },
    #[serde(rename = "st")]
    Stroke {
        nm: Option<String>,
        c: LottiePropMulti,
        o: LottiePropScalar,
        w: LottiePropScalar,
    },
    #[serde(rename = "gr")]
    Group {
        nm: Option<String>,
        it: Vec<LottieShape>,
    },
    #[serde(rename = "tr")]
    Transform(LottieTransform),
}
