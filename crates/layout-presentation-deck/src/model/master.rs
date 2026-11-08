//! Presentation, slide master, layout, and theme hierarchies.

use serde::{Deserialize, Serialize};
use super::types::{ColorScheme, LayoutId, MasterId, ShapeId, Size, SlideId, Xfrm};
use super::shape::{Shape, SlideTransition};
use crate::timeline::anim::Animation;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub colors: ColorScheme,
    pub font_heading: String,
    pub font_body: String,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            name: "Default".into(),
            colors: ColorScheme::default(),
            font_heading: "Calibri".into(),
            font_body: "Calibri".into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Layout {
    pub id: LayoutId,
    pub name: String,
    pub master_id: MasterId,
    pub shapes: Vec<Shape>,
}

impl Layout {
    pub fn new(id: LayoutId, name: impl Into<String>, master_id: MasterId) -> Self {
        Layout {
            id,
            name: name.into(),
            master_id,
            shapes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Master {
    pub id: MasterId,
    pub name: String,
    pub theme: Theme,
    pub shapes: Vec<Shape>,
    pub layout_ids: Vec<LayoutId>,
}

impl Master {
    pub fn new(id: MasterId, name: impl Into<String>) -> Self {
        Master {
            id,
            name: name.into(),
            theme: Theme::default(),
            shapes: Vec::new(),
            layout_ids: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Slide {
    pub id: SlideId,
    pub layout_id: LayoutId,
    pub shapes: Vec<Shape>,
    pub transition: SlideTransition,
    pub animations: Vec<Animation>,
    pub hidden: bool,
}

impl Slide {
    pub fn new(id: SlideId, layout_id: LayoutId) -> Self {
        Slide {
            id,
            layout_id,
            shapes: Vec::new(),
            transition: SlideTransition::default(),
            animations: Vec::new(),
            hidden: false,
        }
    }

    pub fn shape_by_id(&self, id: ShapeId) -> Option<&Shape> {
        self.shapes.iter().find(|s| s.id == id)
    }

    pub fn shape_by_id_mut(&mut self, id: ShapeId) -> Option<&mut Shape> {
        self.shapes.iter_mut().find(|s| s.id == id)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomShow {
    pub name: String,
    pub slides: Vec<SlideId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShowSettings {
    pub loop_continuously: bool,
    pub custom_show: Option<String>,
    pub range: Option<(u32, u32)>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Presentation {
    pub size: Size,
    pub masters: Vec<Master>,
    pub layouts: Vec<Layout>,
    pub slides: Vec<Slide>,
    pub custom_shows: Vec<CustomShow>,
    pub show: ShowSettings,
}

impl Default for Presentation {
    fn default() -> Self {
        Presentation {
            size: Size { w: 960.0, h: 540.0 }, // 16:9 standard points
            masters: Vec::new(),
            layouts: Vec::new(),
            slides: Vec::new(),
            custom_shows: Vec::new(),
            show: ShowSettings::default(),
        }
    }
}

impl Presentation {
    pub fn master_for_slide(&self, slide: &Slide) -> Option<(&Master, &Layout)> {
        let layout = self.layouts.iter().find(|l| l.id == slide.layout_id)?;
        let master = self.masters.iter().find(|m| m.id == layout.master_id)?;
        Some((master, layout))
    }
}
