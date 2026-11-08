//! Model identifier types and geometric primitives.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SlideId(pub u32);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ShapeId(pub u32);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LayoutId(pub u32);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MasterId(pub u32);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PhType {
    Title,
    CtrTitle,
    SubTitle,
    #[default]
    Body,
    Obj,
    Chart,
    Table,
    Media,
    Picture,
    Date,
    Footer,
    SlideNum,
    Header,
}

impl PhType {
    pub fn is_title(&self) -> bool {
        matches!(self, PhType::Title | PhType::CtrTitle)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Xfrm {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub rot: f64,
    pub flip_h: bool,
    pub flip_v: bool,
}

impl Xfrm {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Xfrm {
            x,
            y,
            w,
            h,
            rot: 0.0,
            flip_h: false,
            flip_v: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Rgba { r, g, b, a }
    }

    pub const BLACK: Rgba = Rgba::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Rgba = Rgba::new(1.0, 1.0, 1.0, 1.0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SchemeSlot {
    Dk1,
    Lt1,
    Dk2,
    Lt2,
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    Accent5,
    Accent6,
    Hlink,
    FolHlink,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorScheme {
    pub dk1: Rgba,
    pub lt1: Rgba,
    pub dk2: Rgba,
    pub lt2: Rgba,
    pub accent1: Rgba,
    pub accent2: Rgba,
    pub accent3: Rgba,
    pub accent4: Rgba,
    pub accent5: Rgba,
    pub accent6: Rgba,
    pub hlink: Rgba,
    pub fol_hlink: Rgba,
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme {
            dk1: Rgba::BLACK,
            lt1: Rgba::WHITE,
            dk2: Rgba::new(0.2, 0.2, 0.2, 1.0),
            lt2: Rgba::new(0.9, 0.9, 0.9, 1.0),
            accent1: Rgba::new(0.26, 0.45, 0.77, 1.0),
            accent2: Rgba::new(0.93, 0.49, 0.19, 1.0),
            accent3: Rgba::new(0.65, 0.65, 0.65, 1.0),
            accent4: Rgba::new(1.0, 0.75, 0.0, 1.0),
            accent5: Rgba::new(0.36, 0.61, 0.84, 1.0),
            accent6: Rgba::new(0.44, 0.68, 0.28, 1.0),
            hlink: Rgba::new(0.0, 0.4, 0.8, 1.0),
            fol_hlink: Rgba::new(0.5, 0.2, 0.6, 1.0),
        }
    }
}

impl ColorScheme {
    pub fn get(&self, slot: SchemeSlot) -> Rgba {
        match slot {
            SchemeSlot::Dk1 => self.dk1,
            SchemeSlot::Lt1 => self.lt1,
            SchemeSlot::Dk2 => self.dk2,
            SchemeSlot::Lt2 => self.lt2,
            SchemeSlot::Accent1 => self.accent1,
            SchemeSlot::Accent2 => self.accent2,
            SchemeSlot::Accent3 => self.accent3,
            SchemeSlot::Accent4 => self.accent4,
            SchemeSlot::Accent5 => self.accent5,
            SchemeSlot::Accent6 => self.accent6,
            SchemeSlot::Hlink => self.hlink,
            SchemeSlot::FolHlink => self.fol_hlink,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ColorRef {
    Srgb(Rgba),
    Scheme(SchemeSlot),
}

impl ColorRef {
    pub fn resolve(&self, scheme: &ColorScheme) -> Rgba {
        match self {
            ColorRef::Srgb(rgba) => *rgba,
            ColorRef::Scheme(slot) => scheme.get(*slot),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Fill {
    None,
    Solid(ColorRef),
    Gradient { stops: Vec<(f32, ColorRef)>, angle: f32 },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Line {
    pub fill: Fill,
    pub width: f64,
}

impl Default for Line {
    fn default() -> Self {
        Line {
            fill: Fill::None,
            width: 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ShapeKind {
    #[default]
    Rect,
    RoundRect,
    Ellipse,
    Triangle,
    Line,
    Custom,
}
