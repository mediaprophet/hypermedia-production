//! Standalone migration closure smoke; coordinator owns execution.
use poet_geometry::{Polyline, Vec2};
use profile_authoring::{AuthoringError, Profile, ProfileLimits};
use raster_blend_operators::{BlendOperator, blend_rgb};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let profile = Profile::RoundedRectangle {
        width: 1.0,
        height: 0.3,
        radius: 0.05,
        segments: 8,
    };
    let points = profile.points(ProfileLimits::default())?;
    let vertices: Vec<_> = points.iter().map(|p| Vec2::new(p[0], p[1])).collect();
    let polyline = Polyline::from_points(&vertices, true);
    assert!(polyline.area() > 0.0);
    assert!(points.len() > 4);
    let invalid = Profile::Circle { radius: f64::NAN, segments: 16 };
    assert!(matches!(invalid.points(ProfileLimits::default()), Err(AuthoringError::Invalid(_))));
    assert!(matches!(profile.points(ProfileLimits { points: 3, scratch_bytes: 64 }), Err(AuthoringError::Budget(_))));
    assert_eq!(blend_rgb(BlendOperator::VividLight, [0.5; 3], [0.75; 3]), [1.0; 3]);
    println!("migrated profile and aliased drafting geometry consumer passed ({} points)", points.len());
    Ok(())
}
