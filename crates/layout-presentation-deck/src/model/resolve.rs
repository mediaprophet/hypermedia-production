//! Placeholder inheritance resolver.
//! Resolves effective transforms, fills, lines, and text properties of a shape
//! following Slide -> Layout -> Master -> Theme.

use super::master::{Layout, Master, Presentation, Slide};
use super::shape::{Placeholder, Shape, ShapeContent, TextBody};
use super::types::{ColorScheme, Fill, Line, PhType, Rgba, Xfrm};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Owner {
    Slide,
    Layout,
    Master,
}

pub struct Ctx<'a> {
    pub pres: &'a Presentation,
    pub master: &'a Master,
    pub layout: Option<&'a Layout>,
    pub owner: Owner,
    pub scheme: ColorScheme,
}

impl<'a> Ctx<'a> {
    pub fn for_slide(pres: &'a Presentation, slide: &Slide) -> Option<Ctx<'a>> {
        let (master, layout) = pres.master_for_slide(slide)?;
        Some(Ctx {
            pres,
            master,
            layout: Some(layout),
            owner: Owner::Slide,
            scheme: master.theme.colors.clone(),
        })
    }

    pub fn for_layout(pres: &'a Presentation, master: &'a Master, layout: &'a Layout) -> Ctx<'a> {
        Ctx {
            pres,
            master,
            layout: Some(layout),
            owner: Owner::Layout,
            scheme: master.theme.colors.clone(),
        }
    }

    pub fn for_master(pres: &'a Presentation, master: &'a Master) -> Ctx<'a> {
        Ctx {
            pres,
            master,
            layout: None,
            owner: Owner::Master,
            scheme: master.theme.colors.clone(),
        }
    }
}

pub fn compatible(slide: PhType, other: PhType) -> bool {
    match slide {
        PhType::Title | PhType::CtrTitle => matches!(other, PhType::Title | PhType::CtrTitle),
        PhType::SubTitle | PhType::Body | PhType::Obj | PhType::Chart | PhType::Table | PhType::Media | PhType::Picture => {
            !matches!(other, PhType::Title | PhType::CtrTitle | PhType::Date | PhType::Footer | PhType::SlideNum | PhType::Header)
        }
        k => k == other,
    }
}

pub fn find_ph<'s>(shapes: &'s [Shape], shape: &Shape, by_idx: bool) -> Option<&'s Shape> {
    let ph = shape.ph.as_ref()?;
    if by_idx && ph.idx != 0 {
        if let Some(s) = shapes.iter().find(|s| {
            s.ph.as_ref().is_some_and(|p| p.idx == ph.idx && compatible(ph.kind, p.kind))
        }) {
            return Some(s);
        }
    }
    // Same type first, then compatible
    shapes
        .iter()
        .find(|s| s.ph.as_ref().is_some_and(|p| p.kind == ph.kind && (!by_idx || ph.idx == 0 || p.idx == ph.idx || ph.kind.is_title())))
        .or_else(|| shapes.iter().find(|s| s.ph.as_ref().is_some_and(|p| p.kind == ph.kind)))
        .or_else(|| shapes.iter().find(|s| s.ph.as_ref().is_some_and(|p| compatible(ph.kind, p.kind))))
}

pub fn master_ph(master: &Master, kind: PhType) -> Option<&Shape> {
    let want = match kind {
        PhType::Title | PhType::CtrTitle => PhType::Title,
        PhType::Date | PhType::Footer | PhType::SlideNum | PhType::Header => kind,
        _ => PhType::Body,
    };
    master.shapes.iter().find(|s| s.ph_type() == Some(want))
}

/// Layout and master placeholders a shape inherits from.
pub fn parents<'a>(ctx: &Ctx<'a>, shape: &Shape) -> (Option<&'a Shape>, Option<&'a Shape>) {
    let Some(ph) = shape.ph.as_ref() else { return (None, None) };
    match ctx.owner {
        Owner::Slide => {
            let lp = ctx.layout.and_then(|l| find_ph(&l.shapes, shape, true));
            let kind = lp.and_then(Shape::ph_type).unwrap_or(ph.kind);
            (lp, master_ph(ctx.master, kind))
        }
        Owner::Layout => (None, master_ph(ctx.master, ph.kind)),
        Owner::Master => (None, None),
    }
}

/// Effective bounding transform of a shape after placeholder resolution.
pub fn resolve_xfrm(ctx: &Ctx<'_>, shape: &Shape) -> Xfrm {
    if shape.xfrm.w > 0.0 && shape.xfrm.h > 0.0 {
        return shape.xfrm;
    }
    let (lp, mp) = parents(ctx, shape);
    if let Some(l) = lp {
        if l.xfrm.w > 0.0 && l.xfrm.h > 0.0 {
            return l.xfrm;
        }
    }
    if let Some(m) = mp {
        if m.xfrm.w > 0.0 && m.xfrm.h > 0.0 {
            return m.xfrm;
        }
    }
    shape.xfrm
}

/// Effective fill of a shape after placeholder resolution.
pub fn resolve_fill(ctx: &Ctx<'_>, shape: &Shape) -> Option<Fill> {
    if let Some(f) = &shape.fill {
        return Some(f.clone());
    }
    let (lp, mp) = parents(ctx, shape);
    if let Some(l) = lp {
        if let Some(f) = &l.fill {
            return Some(f.clone());
        }
    }
    if let Some(m) = mp {
        if let Some(f) = &m.fill {
            return Some(f.clone());
        }
    }
    None
}

/// Effective line/border of a shape after placeholder resolution.
pub fn resolve_line(ctx: &Ctx<'_>, shape: &Shape) -> Option<Line> {
    if let Some(l) = &shape.line {
        return Some(l.clone());
    }
    let (lp, mp) = parents(ctx, shape);
    if let Some(l) = lp {
        if let Some(line) = &l.line {
            return Some(line.clone());
        }
    }
    if let Some(m) = mp {
        if let Some(line) = &m.line {
            return Some(line.clone());
        }
    }
    None
}
