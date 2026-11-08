//! Presentation model module.

pub mod master;
pub mod resolve;
pub mod shape;
pub mod types;

pub use master::{CustomShow, Layout, Master, Presentation, ShowSettings, Slide, Theme};
pub use resolve::{resolve_fill, resolve_line, resolve_xfrm, Ctx, Owner};
pub use shape::{
    ParaProps, Paragraph, Placeholder, Run, RunProps, Shape, ShapeContent, SlideTransition, Table,
    TableCell, TableRow, TextAlign, TextBody, TransitionKind,
};
pub use types::{
    ColorRef, ColorScheme, Fill, LayoutId, Line, MasterId, PhType, Rect, Rgba, SchemeSlot,
    ShapeId, ShapeKind, Size, Xfrm,
};
