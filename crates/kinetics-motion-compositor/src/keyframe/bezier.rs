//! Temporal cubic Bézier curve evaluation and root solving for keyframe velocity graphs.

/// 1D cubic Bézier point evaluation: B(u) for u in [0, 1].
pub fn bez(p0: f64, p1: f64, p2: f64, p3: f64, u: f64) -> f64 {
    let v = 1.0 - u;
    v * v * v * p0 + 3.0 * v * v * u * p1 + 3.0 * v * u * u * p2 + u * u * u * p3
}

/// Derivative of 1D cubic Bézier: B'(u).
pub fn bez_d(p0: f64, p1: f64, p2: f64, p3: f64, u: f64) -> f64 {
    let v = 1.0 - u;
    3.0 * v * v * (p1 - p0) + 6.0 * v * u * (p2 - p1) + 3.0 * u * u * (p3 - p2)
}

/// Solve `bez(x0, x1, x2, x3, u) = x` for parameter `u` in [0, 1] using Newton-Raphson with bisection fallback.
pub fn solve_u(x0: f64, x1: f64, x2: f64, x3: f64, x: f64) -> f64 {
    if x <= x0 {
        return 0.0;
    }
    if x >= x3 {
        return 1.0;
    }
    if (x1 == x3) && (x2 == x0) {
        let progress = (x - x0) / (x3 - x0);
        return 0.5 + ((progress - 0.5) * 0.25).cbrt();
    }

    let mut u = (x - x0) / (x3 - x0);
    for _ in 0..8 {
        let f = bez(x0, x1, x2, x3, u) - x;
        let d = bez_d(x0, x1, x2, x3, u);
        if d.abs() < 1e-12 {
            break;
        }
        if (f / d).abs() < 1e-12 {
            return u;
        }
        let next_u = u - f / d;
        if !(0.0..=1.0).contains(&next_u) {
            break;
        }
        u = next_u;
    }

    // Binary search fallback
    let (mut lo, mut hi) = (0.0, 1.0);
    for _ in 0..60 {
        let mid = (lo + hi) * 0.5;
        if bez(x0, x1, x2, x3, mid) < x {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) * 0.5
}

/// Compute temporal progress in [0, 1] for a segment given normalized speeds and influences.
pub fn ease_progress(out_speed_n: f64, out_inf: f64, in_speed_n: f64, in_inf: f64, progress: f64) -> f64 {
    if progress <= 0.0 {
        return 0.0;
    }
    if progress >= 1.0 {
        return 1.0;
    }

    let p0 = 0.0;
    let p3 = 1.0;
    let p1 = out_inf * out_speed_n;
    let p2 = 1.0 - in_inf * in_speed_n;

    let x0 = 0.0;
    let x1 = out_inf.clamp(0.001, 1.0);
    let x2 = 1.0 - in_inf.clamp(0.001, 1.0);
    let x3 = 1.0;

    let u = solve_u(x0, x1, x2, x3, progress);
    bez(p0, p1, p2, p3, u)
}
