//! Transition-span editing adapted from the chronicle media `filmcraft-edit` operation.

use crate::Tick;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClipSpan {
    pub start: Tick,
    pub end: Tick,
}

impl ClipSpan {
    pub const fn new(start: Tick, end: Tick) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransitionContext {
    /// The outgoing clip. `None` describes a fade-in from empty space.
    pub from: Option<ClipSpan>,
    /// The incoming clip. `None` describes a fade-out to empty space.
    pub to: Option<ClipSpan>,
    pub track_locked: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimelineBounds {
    pub earliest_start: Tick,
    pub cut: Tick,
    pub latest_end: Tick,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransitionSpan {
    pub start: Tick,
    pub duration: Tick,
}

impl TransitionSpan {
    pub fn checked_end(self) -> Option<Tick> {
        self.start.checked_add(self.duration)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidatedTransitionSpan {
    pub span: TransitionSpan,
    pub alignment: TransitionAlignment,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionAlignment {
    Center,
    Start,
    End,
    /// A custom span that still covers the cut but is not centered on it.
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionEdge {
    In,
    Out,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransitionEditError {
    Locked,
    MissingClip,
    InvalidClipSpan,
    InvalidBounds,
    TooShort,
    OutsideClips,
    DoesNotCoverCut,
    OutOfRange,
}

/// Return the allowed interval and cut for the referenced clips.
pub fn transition_bounds(
    context: TransitionContext,
) -> Result<TimelineBounds, TransitionEditError> {
    let bounds = match (context.from, context.to) {
        (Some(from), Some(to)) => TimelineBounds {
            earliest_start: from.start,
            cut: to.start,
            latest_end: to.end,
        },
        (None, Some(to)) => TimelineBounds {
            earliest_start: to.start,
            cut: to.start,
            latest_end: to.end,
        },
        (Some(from), None) => TimelineBounds {
            earliest_start: from.start,
            cut: from.end,
            latest_end: from.end,
        },
        (None, None) => return Err(TransitionEditError::MissingClip),
    };
    let spans = context.from.into_iter().chain(context.to);
    if spans.into_iter().any(|span| span.end < span.start) {
        return Err(TransitionEditError::InvalidClipSpan);
    }
    if bounds.earliest_start > bounds.cut || bounds.cut > bounds.latest_end {
        return Err(TransitionEditError::InvalidBounds);
    }
    Ok(bounds)
}

/// Infer the alignment stored or shown for a span over the supplied cut.
pub fn transition_alignment(
    context: TransitionContext,
    span: TransitionSpan,
    cut: Tick,
    frame: Tick,
) -> TransitionAlignment {
    if context.from.is_none() {
        return TransitionAlignment::Start;
    }
    if context.to.is_none() {
        return TransitionAlignment::End;
    }
    if span.start == cut {
        return TransitionAlignment::Start;
    }
    if span.checked_end() == Some(cut) {
        return TransitionAlignment::End;
    }
    let centre_delta = i128::from(span.start.0) * 2 + i128::from(span.duration.0)
        - i128::from(cut.0) * 2;
    if centre_delta.abs() <= i128::from(frame.0.max(0)) {
        TransitionAlignment::Center
    } else {
        TransitionAlignment::Custom
    }
}

/// Validate an edit before a host mutates its sequence/session. On refusal the
/// caller still owns its unchanged source document and history.
pub fn set_transition_span(
    context: TransitionContext,
    span: TransitionSpan,
    minimum: Tick,
    frame: Tick,
) -> Result<ValidatedTransitionSpan, TransitionEditError> {
    if context.track_locked {
        return Err(TransitionEditError::Locked);
    }
    let bounds = transition_bounds(context)?;
    if span.duration < minimum.max(Tick(1)) {
        return Err(TransitionEditError::TooShort);
    }
    let end = span
        .checked_end()
        .ok_or(TransitionEditError::OutOfRange)?;
    if span.start < bounds.earliest_start || end > bounds.latest_end {
        return Err(TransitionEditError::OutsideClips);
    }
    let covers_cut = match (context.from, context.to) {
        (None, Some(_)) => span.start == bounds.cut,
        (Some(_), None) => end == bounds.cut,
        (Some(_), Some(_)) => span.start <= bounds.cut && bounds.cut <= end,
        (None, None) => false,
    };
    if !covers_cut {
        return Err(TransitionEditError::DoesNotCoverCut);
    }
    Ok(ValidatedTransitionSpan {
        span,
        alignment: transition_alignment(context, span, bounds.cut, frame),
    })
}

/// Compute a new start when a user changes the typed duration/alignment. The
/// caller supplies the frame snapping policy used by its current sequence.
pub fn start_for_alignment(
    context: TransitionContext,
    current: TransitionSpan,
    duration: Tick,
    alignment: TransitionAlignment,
    frame: Tick,
    snap: impl Fn(Tick) -> Tick,
) -> Option<Tick> {
    let bounds = transition_bounds(context).ok()?;
    let one_sided = transition_alignment(context, current, bounds.cut, frame);
    let alignment = match one_sided {
        TransitionAlignment::Start if context.from.is_none() => TransitionAlignment::Start,
        TransitionAlignment::End if context.to.is_none() => TransitionAlignment::End,
        _ => alignment,
    };
    let half = duration.mul_ratio(1, 2);
    match alignment {
        TransitionAlignment::Start => Some(bounds.cut),
        TransitionAlignment::End => bounds.cut.checked_sub(duration),
        TransitionAlignment::Center => bounds.cut.checked_sub(half).map(snap),
        TransitionAlignment::Custom => current
            .start
            .checked_add(current.duration.mul_ratio(1, 2))
            .and_then(|centre| centre.checked_sub(half))
            .map(snap),
    }
}

/// Compute a drag preview without changing a document. The host commits the
/// returned span through its own authorized command/session/history path.
/// Like the chronicle media low-level preview helper, this does not reject a locked
/// track; the authoritative commit must call [`set_transition_span`] with the
/// current context, which refuses locked tracks.
pub fn drag_transition_span(
    context: TransitionContext,
    current: TransitionSpan,
    edge: Option<TransitionEdge>,
    delta: Tick,
    frame: Tick,
) -> Option<TransitionSpan> {
    let bounds = transition_bounds(context).ok()?;
    let frame = frame.max(Tick(1));
    let start = current.start;
    let one_sided = context.from.is_none() || context.to.is_none();
    let unchanged = Some(current);
    let Some(end_tick) = current.start.checked_add(current.duration) else {
        return unchanged;
    };
    let end = end_tick.0;
    match edge {
        None if one_sided => unchanged,
        None => {
            let minimum_start = bounds.earliest_start.max(Tick(
                bounds.cut.0.saturating_sub(current.duration.0),
            ));
            let maximum_start = bounds.latest_end.checked_sub(current.duration)?.min(bounds.cut);
            if minimum_start > maximum_start {
                unchanged
            } else {
                let moved = start.0.saturating_add(delta.0);
                Some(TransitionSpan {
                    start: Tick(moved.max(minimum_start.0).min(maximum_start.0)),
                    duration: current.duration,
                })
            }
        }
        Some(TransitionEdge::In) if context.from.is_none() => unchanged,
        Some(TransitionEdge::In) => {
            let maximum_start = bounds.cut.min(Tick(end.saturating_sub(frame.0)));
            if maximum_start < bounds.earliest_start {
                return unchanged;
            }
            let moved = start.0.saturating_add(delta.0);
            let next_start = Tick(moved.max(bounds.earliest_start.0).min(maximum_start.0));
            Some(TransitionSpan {
                start: next_start,
                duration: Tick(end.saturating_sub(next_start.0)),
            })
        }
        Some(TransitionEdge::Out) if context.to.is_none() => unchanged,
        Some(TransitionEdge::Out) => {
            let minimum_end = bounds.cut.max(Tick(start.0.saturating_add(frame.0)));
            if minimum_end > bounds.latest_end {
                return unchanged;
            }
            let moved = end.saturating_add(delta.0);
            let next_end = Tick(moved.max(minimum_end.0).min(bounds.latest_end.0));
            Some(TransitionSpan {
                start,
                duration: Tick(next_end.0.saturating_sub(start.0)),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FrameRate;

    fn frame(n: i64) -> Tick {
        FrameRate::FPS_24.tick_of(n)
    }

    fn two_clip_context(locked: bool) -> TransitionContext {
        TransitionContext {
            from: Some(ClipSpan::new(frame(0), frame(48))),
            to: Some(ClipSpan::new(frame(48), frame(96))),
            track_locked: locked,
        }
    }

    #[test]
    fn span_alignment_preserves_center_start_end_custom_and_odd_frame_rules() {
        let context = two_clip_context(false);
        let cut = frame(48);
        let classify = |start, duration| {
            transition_alignment(
                context,
                TransitionSpan { start: frame(start), duration: frame(duration) },
                cut,
                frame(1),
            )
        };
        assert_eq!(classify(36, 24), TransitionAlignment::Center);
        assert_eq!(classify(48, 24), TransitionAlignment::Start);
        assert_eq!(classify(24, 24), TransitionAlignment::End);
        assert_eq!(classify(40, 20), TransitionAlignment::Custom);
        assert_eq!(classify(42, 13), TransitionAlignment::Center);
    }

    #[test]
    fn valid_edit_returns_frame_exact_span_and_alignment() {
        let updated = set_transition_span(
            two_clip_context(false),
            TransitionSpan { start: frame(40), duration: frame(20) },
            frame(1),
            frame(1),
        ).expect("in-bounds transition spanning the cut");
        assert_eq!(updated.span.start, frame(40));
        assert_eq!(updated.span.duration, frame(20));
        assert_eq!(updated.alignment, TransitionAlignment::Custom);
    }

    #[test]
    fn refused_edits_cover_locks_short_spans_overhangs_cut_and_overflow() {
        let context = two_clip_context(false);
        for (start, duration) in [
            (-1, 24),
            (40, 60),
            (10, 20),
            (50, 10),
            (40, 0),
        ] {
            assert!(set_transition_span(
                context,
                TransitionSpan { start: frame(start), duration: frame(duration) },
                frame(1),
                frame(1),
            ).is_err());
        }
        assert_eq!(
            set_transition_span(
                two_clip_context(true),
                TransitionSpan { start: frame(36), duration: frame(24) },
                frame(1),
                frame(1),
            ),
            Err(TransitionEditError::Locked),
        );
        assert_eq!(
            set_transition_span(
                context,
                TransitionSpan { start: Tick(i64::MAX - 1), duration: Tick(i64::MAX) },
                frame(1),
                frame(1),
            ),
            Err(TransitionEditError::OutOfRange),
        );
    }

    #[test]
    fn one_sided_fades_keep_their_clip_edge() {
        let fade_in = TransitionContext {
            from: None,
            to: Some(ClipSpan::new(frame(0), frame(48))),
            track_locked: false,
        };
        let fade_out = TransitionContext {
            from: Some(ClipSpan::new(frame(96), frame(144))),
            to: None,
            track_locked: false,
        };
        assert_eq!(
            set_transition_span(fade_in, TransitionSpan { start: frame(0), duration: frame(12) }, frame(1), frame(1))
                .map(|v| v.alignment),
            Ok(TransitionAlignment::Start),
        );
        assert_eq!(
            set_transition_span(fade_in, TransitionSpan { start: frame(1), duration: frame(12) }, frame(1), frame(1)),
            Err(TransitionEditError::DoesNotCoverCut),
        );
        assert_eq!(
            set_transition_span(fade_out, TransitionSpan { start: frame(132), duration: frame(12) }, frame(1), frame(1))
                .map(|v| v.alignment),
            Ok(TransitionAlignment::End),
        );
        assert_eq!(
            set_transition_span(fade_out, TransitionSpan { start: frame(131), duration: frame(12) }, frame(1), frame(1)),
            Err(TransitionEditError::DoesNotCoverCut),
        );
    }

    #[test]
    fn duration_change_and_drag_preview_respect_cut_and_clip_bounds() {
        let context = two_clip_context(false);
        let current = TransitionSpan { start: frame(36), duration: frame(24) };
        let snap = |time| FrameRate::FPS_24.snap(time);
        assert_eq!(
            start_for_alignment(context, current, frame(36), TransitionAlignment::Center, frame(1), snap),
            Some(frame(30)),
        );
        assert_eq!(
            start_for_alignment(context, current, frame(36), TransitionAlignment::Start, frame(1), snap),
            Some(frame(48)),
        );
        assert_eq!(
            start_for_alignment(context, current, frame(36), TransitionAlignment::End, frame(1), snap),
            Some(frame(12)),
        );
        assert_eq!(
            drag_transition_span(context, current, Some(TransitionEdge::Out), frame(-100), frame(1)),
            Some(TransitionSpan { start: frame(36), duration: frame(12) }),
        );
        assert_eq!(
            drag_transition_span(context, current, None, frame(100), frame(1)),
            Some(TransitionSpan { start: frame(48), duration: frame(24) }),
        );
        let locked = TransitionContext { track_locked: true, ..context };
        assert_eq!(
            drag_transition_span(locked, current, None, frame(1), frame(1)),
            Some(TransitionSpan { start: frame(37), duration: frame(24) }),
        );
        assert_eq!(
            set_transition_span(locked, current, frame(1), frame(1)),
            Err(TransitionEditError::Locked),
        );
    }
}
