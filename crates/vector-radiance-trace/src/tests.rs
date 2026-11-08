use super::*;
use vector_locus_geometry::PathData;

const BLACK: [u8; 4] = [0, 0, 0, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];

fn bw() -> TraceParams {
    TraceParams {
        ignore_white: true,
        ..TraceParams::default()
    }
}

fn disc(size: u32, cx: f64, cy: f64, r: f64) -> Raster {
    Raster::from_fn(size, size, |x, y| {
        let (dx, dy) = (x as f64 + 0.5 - cx, y as f64 + 0.5 - cy);
        if dx * dx + dy * dy <= r * r {
            BLACK
        } else {
            WHITE
        }
    })
}

fn net_area(p: &PathData) -> f64 {
    p.subpaths.iter().map(|s| s.area()).sum()
}

#[test]
fn black_disc_is_one_closed_path_with_disc_area() {
    let r = 80.0;
    let res = trace(&disc(200, 100.0, 100.0, r), &bw());
    assert_eq!(res.paths.len(), 1);
    let p = &res.paths[0];
    assert_eq!(p.path.subpaths.len(), 1);
    assert!(p.path.subpaths[0].closed);
    assert_eq!(p.color, [0, 0, 0]);
    let want = std::f64::consts::PI * r * r;
    let got = net_area(&p.path);
    assert!((got - want).abs() / want < 0.03, "area {got} vs {want}");
    assert!(p.path.anchor_count() < 40, "anchors {}", p.path.anchor_count());
}

#[test]
fn small_disc_area_within_tolerance() {
    let r = 20.0;
    let res = trace(&disc(64, 32.0, 32.0, r), &bw());
    assert_eq!(res.paths.len(), 1);
    let want = std::f64::consts::PI * r * r;
    let got = net_area(&res.paths[0].path);
    assert!((got - want).abs() / want < 0.03, "area {got} vs {want}");
}

#[test]
fn ring_has_outer_and_hole() {
    let img = Raster::from_fn(200, 200, |x, y| {
        let d2 = (x as f64 + 0.5 - 100.0).powi(2) + (y as f64 + 0.5 - 100.0).powi(2);
        if (40.0 * 40.0..=80.0 * 80.0).contains(&d2) {
            BLACK
        } else {
            WHITE
        }
    });
    let res = trace(&img, &bw());
    assert_eq!(res.paths.len(), 1);
    assert_eq!(res.paths[0].path.subpaths.len(), 2);
    let want = std::f64::consts::PI * (80.0f64.powi(2) - 40.0f64.powi(2));
    let got = net_area(&res.paths[0].path);
    assert!((got - want).abs() / want < 0.03, "area {got} vs {want}");
    let a: Vec<f64> = res.paths[0].path.subpaths.iter().map(|s| s.area()).collect();
    assert!(a[0] * a[1] < 0.0);
}

#[test]
fn stripes_give_one_path_per_stripe() {
    let img = Raster::from_fn(100, 60, |x, _| {
        if (x / 10) % 2 == 0 {
            BLACK
        } else {
            WHITE
        }
    });
    let res = trace(&img, &bw());
    assert_eq!(res.paths.len(), 5);
    for p in &res.paths {
        let a = net_area(&p.path);
        assert!((a - 600.0).abs() < 1.0, "stripe area {a}");
        assert_eq!(p.path.anchor_count(), 4, "rectangles keep their 4 corners");
    }
}

#[test]
fn two_colour_stripes_without_ignore_white_trace_both_colours() {
    let img = Raster::from_fn(100, 60, |x, _| {
        if (x / 10) % 2 == 0 {
            BLACK
        } else {
            WHITE
        }
    });
    let res = trace(&img, &TraceParams::default());
    assert_eq!(res.paths.len(), 10);
    assert_eq!(res.palette.len(), 2);
    let total: f64 = res.paths.iter().map(|p| net_area(&p.path)).sum();
    assert!((total - 6000.0).abs() < 2.0);
}

#[test]
fn noise_removal_drops_speckles() {
    let img = Raster::from_fn(100, 100, |x, y| {
        let square = (20..60).contains(&x) && (20..60).contains(&y);
        let speck = (x % 13 == 3 && y % 11 == 5) && !(15..65).contains(&x);
        if square || speck {
            BLACK
        } else {
            WHITE
        }
    });
    let noisy = trace(&img, &TraceParams { noise: 0, ..bw() });
    assert!(noisy.paths.len() > 10, "{} paths", noisy.paths.len());
    let clean = trace(&img, &TraceParams { noise: 10, ..bw() });
    assert_eq!(clean.paths.len(), 1);
    assert!((net_area(&clean.paths[0].path) - 1600.0).abs() < 1.0);
}

#[test]
fn noise_removal_fills_pinholes() {
    let img = Raster::from_fn(60, 60, |x, y| {
        let square = (10..50).contains(&x) && (10..50).contains(&y);
        let hole = x == 30 && y == 30;
        if square && !hole {
            BLACK
        } else {
            WHITE
        }
    });
    let res = trace(&img, &TraceParams { noise: 4, ..bw() });
    assert_eq!(res.paths.len(), 1);
    assert_eq!(res.paths[0].path.subpaths.len(), 1);
}

#[test]
fn diagonal_pixels_stay_separate() {
    let img = Raster::from_fn(4, 4, |x, y| {
        if (x, y) == (1, 1) || (x, y) == (2, 2) {
            BLACK
        } else {
            WHITE
        }
    });
    let res = trace(&img, &TraceParams { noise: 0, ..bw() });
    assert_eq!(res.paths.len(), 2);
}

#[test]
fn contour_loops_are_consistent() {
    let mask = vec![true, true, true, true, false, true, true, true, true];
    let comps = trace_mask(&mask, 3, 3);
    assert_eq!(comps.len(), 1);
    assert_eq!(comps[0].pixels, 8);
    assert_eq!(comps[0].outer.area2, 18);
    assert_eq!(comps[0].holes.len(), 1);
    assert_eq!(comps[0].holes[0].area2, -2);
}

#[test]
fn color_mode_palette_size_matches_request() {
    let quad = |x: u32, y: u32| match (x < 50, y < 50) {
        (true, true) => [220, 30, 30, 255],
        (false, true) => [30, 200, 40, 255],
        (true, false) => [30, 40, 210, 255],
        (false, false) => [240, 220, 20, 255],
    };
    let img = Raster::from_fn(100, 100, quad);
    let p4 = trace(
        &img,
        &TraceParams {
            mode: Mode::Color,
            colors: 4,
            ..TraceParams::default()
        },
    );
    assert_eq!(p4.palette.len(), 4);
    assert_eq!(p4.paths.len(), 4);
    let p2 = trace(
        &img,
        &TraceParams {
            mode: Mode::Color,
            colors: 2,
            ..TraceParams::default()
        },
    );
    assert_eq!(p2.palette.len(), 2);
    assert!(p4.palette.iter().any(|c| c[0] > 200 && c[1] < 60 && c[2] < 60));
}

#[test]
fn test_trace_rgba_direct() {
    let width = 40;
    let height = 40;
    let mut rgba = vec![255u8; (width * height * 4) as usize];
    for y in 10..30 {
        for x in 10..30 {
            let idx = ((y * width + x) * 4) as usize;
            rgba[idx] = 0;
            rgba[idx + 1] = 0;
            rgba[idx + 2] = 0;
            rgba[idx + 3] = 255;
        }
    }
    let opts = bw();
    let res = trace_rgba(width, height, &rgba, &opts).expect("trace_rgba ok");
    assert_eq!(res.paths.len(), 1);
    assert_eq!(res.paths[0].color, [0, 0, 0]);
}

#[test]
fn test_preset_conversion() {
    let p = TracePreset::HighFidelityPhoto.to_options();
    assert_eq!(p.mode, Mode::Color);
    assert_eq!(p.colors, 64);
}
