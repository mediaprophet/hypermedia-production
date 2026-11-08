use layout_presentation_deck::*;

#[test]
fn test_placeholder_inheritance() {
    let mut pres = Presentation::default();

    let mut master = Master::new(MasterId(1), "Main Master");
    let mut master_title = Shape::new(ShapeId(101), "Title", Xfrm::new(50.0, 40.0, 800.0, 100.0));
    master_title.ph = Some(Placeholder {
        kind: PhType::Title,
        idx: 0,
    });
    master_title.line = Some(Line {
        fill: Fill::Solid(ColorRef::Srgb(Rgba::BLACK)),
        width: 2.0,
    });
    master.shapes.push(master_title);

    let mut layout = Layout::new(LayoutId(1), "Title Layout", MasterId(1));
    let mut layout_title = Shape::new(ShapeId(201), "Title", Xfrm::new(60.0, 50.0, 840.0, 90.0));
    layout_title.ph = Some(Placeholder {
        kind: PhType::Title,
        idx: 0,
    });
    layout.shapes.push(layout_title);

    let mut slide = Slide::new(SlideId(1), LayoutId(1));
    // Shape on slide has empty dimensions, should inherit from layout
    let mut slide_title = Shape::new(ShapeId(301), "Title", Xfrm::default());
    slide_title.ph = Some(Placeholder {
        kind: PhType::Title,
        idx: 0,
    });
    slide.shapes.push(slide_title.clone());

    pres.masters.push(master);
    pres.layouts.push(layout);
    pres.slides.push(slide.clone());

    let ctx = Ctx::for_slide(&pres, &slide).expect("ctx should resolve");
    let eff_xfrm = resolve_xfrm(&ctx, &slide_title);
    assert_eq!(eff_xfrm.x, 60.0);
    assert_eq!(eff_xfrm.w, 840.0);

    let eff_line = resolve_line(&ctx, &slide_title).expect("should inherit line from master");
    assert_eq!(eff_line.width, 2.0);
}

#[test]
fn test_morph_matching_and_interpolation() {
    let mut slide_a = Slide::new(SlideId(1), LayoutId(1));
    let mut s1 = Shape::new(ShapeId(1), "!!card", Xfrm::new(10.0, 10.0, 100.0, 100.0));
    s1.xfrm.rot = 10.0;
    let s2 = Shape::new(ShapeId(2), "HeroText", Xfrm::new(200.0, 50.0, 300.0, 40.0)).with_text("Hello World");
    slide_a.shapes.push(s1);
    slide_a.shapes.push(s2);

    let mut slide_b = Slide::new(SlideId(2), LayoutId(1));
    let mut s3 = Shape::new(ShapeId(3), "!!card", Xfrm::new(50.0, 100.0, 200.0, 150.0));
    s3.xfrm.rot = 350.0; // Shortest distance is -20 degrees, not +340
    let s4 = Shape::new(ShapeId(4), "DifferentName", Xfrm::new(200.0, 200.0, 300.0, 40.0)).with_text("Hello World");
    slide_b.shapes.push(s3);
    slide_b.shapes.push(s4);

    let pairs = morph_pairs(&slide_a, &slide_b);
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0], (ShapeId(1), ShapeId(3))); // Forced !! match
    assert_eq!(pairs[1], (ShapeId(2), ShapeId(4))); // Content text match

    // Test shortest-path rotation interpolation: 10 deg -> 350 deg (-10 deg)
    let mid_xfrm = morph_xfrm(slide_a.shapes[0].xfrm, slide_b.shapes[0].xfrm, 0.5);
    assert_eq!(mid_xfrm.x, 30.0); // (10 + 50) / 2
    assert_eq!(mid_xfrm.w, 150.0); // (100 + 200) / 2
    // Shortest path: 10 -> -10 (which is 350). Midpoint is 0.0 degrees!
    assert!((mid_xfrm.rot - 0.0).abs() < 1e-4);

    let frame = build_morph_frame(&slide_a, &slide_b, 0.5);
    assert_eq!(frame.shapes.len(), 2);
}

#[test]
fn test_motion_path_spline() {
    let path_str = "M 0 0 L 100 0 C 100 50 150 100 200 100";
    let mp = MotionPath::parse(path_str);
    assert!(mp.points.len() > 2);
    assert!(mp.total_length() > 200.0);

    let start = mp.point_at(0.0);
    assert_eq!(start, (0.0, 0.0));

    let end = mp.point_at(1.0);
    assert!((end.0 - 200.0).abs() < 1e-4);
    assert!((end.1 - 100.0).abs() < 1e-4);
}

#[test]
fn test_timeline_and_show_progression() {
    let mut slide = Slide::new(SlideId(1), LayoutId(1));
    slide.animations.push(Animation {
        shape: ShapeId(1),
        class: AnimClass::Entrance,
        start: AnimStart::OnClick,
        duration: 1.0,
        ..Default::default()
    });
    slide.animations.push(Animation {
        shape: ShapeId(2),
        class: AnimClass::Entrance,
        start: AnimStart::WithPrev,
        duration: 0.8,
        ..Default::default()
    });
    slide.animations.push(Animation {
        shape: ShapeId(3),
        class: AnimClass::Emphasis,
        start: AnimStart::OnClick,
        duration: 0.5,
        ..Default::default()
    });

    let timeline = Timeline::compile(&slide);
    assert_eq!(timeline.total_steps, 2);
    assert_eq!(timeline.steps[0].effects.len(), 2); // Click step 0 has anim 1 & 2
    assert_eq!(timeline.steps[1].effects.len(), 1); // Click step 1 has anim 3

    // Shape 3 is not visible during step 0 if it had an entrance, but here shape 1 and 2 enter at step 0
    assert!(timeline.is_shape_visible(ShapeId(1), 0, 0.5));
    assert!(!timeline.is_shape_visible(ShapeId(1), 0, -0.1));

    let mut pres = Presentation::default();
    pres.slides.push(slide);
    pres.slides.push(Slide::new(SlideId(2), LayoutId(1)));

    let mut state = ShowState::default();
    let act1 = state.next(&pres, 2);
    assert_eq!(act1, ShowAction::PlayStep(0));
    let act2 = state.next(&pres, 2);
    assert_eq!(act2, ShowAction::PlayStep(1));
    let act3 = state.next(&pres, 2);
    assert_eq!(act3, ShowAction::Slide(1)); // Advances to slide index 1
}

#[test]
fn test_ontology_contract_parity() {
    use layout_presentation_deck::ontology::{PRESENTATION_MORPH_SPEC, PRESENTATION_TIMELINE_SPEC};
    assert!(PRESENTATION_MORPH_SPEC.contract.precondition.is_valid());
    assert!(PRESENTATION_MORPH_SPEC.contract.postcondition.is_valid());
    assert!(PRESENTATION_MORPH_SPEC.contract.invariant.is_valid());

    assert!(PRESENTATION_TIMELINE_SPEC.contract.precondition.is_valid());
    assert!(PRESENTATION_TIMELINE_SPEC.contract.postcondition.is_valid());
    assert!(PRESENTATION_TIMELINE_SPEC.contract.invariant.is_valid());
}
