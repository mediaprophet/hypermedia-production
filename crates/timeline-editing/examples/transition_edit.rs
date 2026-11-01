//! Independent operation consumer; document authority and persistence remain host-owned.
use timeline_editing::{
    ClipSpan, FrameRate, Tick, TransitionContext, TransitionEdge, TransitionEditError,
    TransitionSpan, drag_transition_span, set_transition_span,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seconds = |value| Tick::from_units(value, 1);
    let context = TransitionContext {
        from: Some(ClipSpan::new(seconds(0), seconds(10))),
        to: Some(ClipSpan::new(seconds(10), seconds(20))),
        track_locked: false,
    };
    let frame = FrameRate::FPS_24.frame_duration();
    let initial = TransitionSpan { start: seconds(9), duration: seconds(2) };
    let accepted = set_transition_span(context, initial, frame, frame)
        .map_err(|error| format!("initial transition refused: {error:?}"))?;
    let proposed = drag_transition_span(context, accepted.span, Some(TransitionEdge::Out), seconds(1), frame)
        .ok_or("transition proposal unavailable")?;
    let edited = set_transition_span(context, proposed, frame, frame)
        .map_err(|error| format!("edited transition refused: {error:?}"))?;
    assert_eq!(edited.span.duration, seconds(3));
    let locked = TransitionContext { track_locked: true, ..context };
    assert_eq!(set_transition_span(locked, edited.span, frame, frame), Err(TransitionEditError::Locked));
    assert_eq!(FrameRate::FPS_23_976.frame_at(FrameRate::FPS_23_976.tick_of(1001)), 1001);
    println!("timeline transition proposal, validation, lock refusal and rational frame check passed");
    Ok(())
}
