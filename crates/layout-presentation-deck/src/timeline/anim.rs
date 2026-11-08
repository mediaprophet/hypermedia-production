//! Animation definition and timing structures.

use serde::{Deserialize, Serialize};
use crate::model::types::ShapeId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimClass {
    Entrance,
    Exit,
    Emphasis,
    Motion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum AnimStart {
    #[default]
    OnClick,
    WithPrev,
    AfterPrev,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sequence {
    Main,
    Trigger(ShapeId),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    pub shape: ShapeId,
    pub class: AnimClass,
    pub start: AnimStart,
    pub duration: f64,
    pub delay: f64,
    pub repeat_count: f64,
    pub motion_path: Option<String>,
    pub by_paragraph: bool,
}

impl Default for Animation {
    fn default() -> Self {
        Animation {
            shape: ShapeId(0),
            class: AnimClass::Entrance,
            start: AnimStart::OnClick,
            duration: 0.5,
            delay: 0.0,
            repeat_count: 1.0,
            motion_path: None,
            by_paragraph: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EffectTiming {
    pub anim_index: usize,
    pub shape: ShapeId,
    pub paragraph: Option<usize>,
    pub sequence: Sequence,
    pub step: usize,
    pub start: f64,
    pub duration: f64,
    pub span: f64,
    pub class: AnimClass,
}
