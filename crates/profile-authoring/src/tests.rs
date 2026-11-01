//! Adapted from current Poet spatial-authoring source tests; see provenance/test-adaptations.md.
use super::{AuthoringError, Profile, ProfileLimits};

#[test]
fn malformed_profiles_are_rejected() {
    // Preserved profile-only cases from malformed_profiles_and_primitive_budgets_are_rejected.
    let invalid = Profile::OpenChannel {
        width: 1.0,
        height: 0.2,
        wall: 0.3,
        corner_radius: 0.01,
        segments: 4,
    };
    assert!(invalid.points(ProfileLimits::default()).is_err());
    let p = Profile::Circle { radius: f64::NAN, segments: 16 };
    assert!(p.points(ProfileLimits::default()).is_err());
}

#[test]
fn changed_profile_parameters_change_section_dimensions_and_area() {
    // Same profile inputs as changed_profile_parameters_change_real_geometry;
    // inspect the extracted section, leaving mesh/sweep tests with Core's caller.
    let original = Profile::RoundedRectangle {
        width: 1.0, height: 0.3, radius: 0.05, segments: 8,
    }.points(ProfileLimits::default()).unwrap();
    let edited = Profile::RoundedRectangle {
        width: 1.2, height: 0.3, radius: 0.05, segments: 8,
    }.points(ProfileLimits::default()).unwrap();
    let xmax = |points: &[[f64; 2]]| points.iter().map(|p| p[0]).fold(f64::NEG_INFINITY, f64::max);
    let area = |points: &[[f64; 2]]| {
        let vertices: Vec<_> = points.iter().map(|p| poet_geometry::Vec2::new(p[0], p[1])).collect();
        poet_geometry::shoelace(&vertices)
    };
    assert!(xmax(&edited) > xmax(&original));
    assert!(original.len() > 8);
    assert!(area(&edited) > area(&original));
    assert!((xmax(&original) - 0.5).abs() < 1e-12);
}

#[test]
fn open_channel_section_retains_wall_void_and_refuses_caller_budget() {
    // Section inputs/analytic area from concave_channel_caps_preserve_wall_void_and_volume.
    let profile = Profile::OpenChannel {
        width: 0.4, height: 0.2, wall: 0.02, corner_radius: 0.0, segments: 4,
    };
    let points = profile.points(ProfileLimits::default()).unwrap();
    let vertices: Vec<_> = points.iter().map(|p| poet_geometry::Vec2::new(p[0], p[1])).collect();
    let expected_area = 0.4 * 0.2 - (0.4 - 0.04) * (0.2 - 0.02);
    assert!((poet_geometry::shoelace(&vertices) - expected_area).abs() < 1e-12);
    // Additional extraction-boundary assertion: refuse before tessellation allocation.
    assert!(matches!(profile.points(ProfileLimits { points: 3, scratch_bytes: 64 }), Err(AuthoringError::Budget(_))));
}
