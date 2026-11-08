//! Presentation deck model, shape morphing, transition interpolation, and animation timeline.
//! Grounded in UDHR Articles 19 & 27 and Informatics Fiduciary standards.

#![forbid(unsafe_code)]

pub mod model;
pub mod morph;
pub mod ontology;
pub mod timeline;

pub use model::{
    resolve_fill, resolve_line, resolve_xfrm, ColorRef, ColorScheme, Ctx, CustomShow, Fill,
    Layout, LayoutId, Line, Master, MasterId, Owner, ParaProps, Paragraph, PhType, Placeholder,
    Presentation, Rect, Rgba, Run, RunProps, SchemeSlot, Shape, ShapeContent, ShapeId, ShapeKind,
    ShowSettings, Size, Slide, SlideTransition, Table, TableCell, TableRow, TextAlign, TextBody,
    Theme, TransitionKind, Xfrm,
};
pub use morph::{
    build_morph_frame, clamp, lerp, lerp_rgba, morph_pairs, morph_xfrm, MorphFrame, MorphPath,
    MorphText,
};
pub use timeline::{
    safe_url, show_order, AnimClass, AnimStart, Animation, EffectTiming, LinkJump, MotionPath,
    Sequence, ShowAction, ShowState, StepInfo, Timeline,
};
