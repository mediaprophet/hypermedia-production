//! Shape recognition for gesture/freehand input: a rough freehand stroke becomes a clean shape.

use kurbo::{Affine, Point, Rect, Vec2};

/// What was recognized from the freehand gesture.
#[derive(Clone, Debug, PartialEq)]
pub enum Recognized {
    Line {
        a: Point,
        b: Point,
    },
    /// Rectangle at a 45° step (squares snap when the sides are within 10%).
    Rectangle {
        rect: Rect,
        rotation: f64,
    },
    /// Ellipse at a 45° step (circles snap when the axes are within 10%).
    Ellipse {
        rect: Rect,
        rotation: f64,
    },
    /// Regular polygon: centre, radius, side count and rotation (degrees).
    Polygon {
        center: Point,
        radius: f64,
        sides: u32,
        rotation: f64,
    },
    /// A zig-zag over existing art: scribble area.
    Scribble(Rect),
}

fn bbox(pts: &[Point]) -> Rect {
    pts.iter().fold(Rect::new(f64::MAX, f64::MAX, f64::MIN, f64::MIN), |r, p| r.union_pt(*p))
}

fn path_len(pts: &[Point]) -> f64 {
    pts.windows(2).map(|w| w[0].distance(w[1])).sum()
}

fn dist_to_segment(p: Point, a: Point, b: Point) -> f64 {
    let ab = b - a;
    let l2 = ab.hypot2();
    if l2 < 1e-12 {
        return p.distance(a);
    }
    let t = ((p - a).dot(ab) / l2).clamp(0.0, 1.0);
    p.distance(a + ab * t)
}

/// Douglas–Peucker simplification with bounded stack use.
pub fn simplify(pts: &[Point], tol: f64) -> Vec<Point> {
    if pts.len() < 3 {
        return pts.to_vec();
    }
    let mut keep = vec![false; pts.len()];
    if let Some(first) = keep.first_mut() {
        *first = true;
    }
    if let Some(last) = keep.last_mut() {
        *last = true;
    }
    let mut pending = vec![(0, pts.len() - 1)];
    while let Some((start, end)) = pending.pop() {
        let Some(window) = pts.get(start..=end) else { continue };
        let (Some(&a), Some(&b)) = (window.first(), window.last()) else { continue };
        let (offset, deviation) = window
            .iter()
            .enumerate()
            .skip(1)
            .take(window.len().saturating_sub(2))
            .map(|(i, p)| (i, dist_to_segment(*p, a, b)))
            .fold((0, 0.0), |best, candidate| if candidate.1 > best.1 { candidate } else { best });
        if deviation > tol {
            let split = start + offset;
            if let Some(k) = keep.get_mut(split) {
                *k = true;
            }
            if split > start + 1 {
                pending.push((start, split));
            }
            if end > split + 1 {
                pending.push((split, end));
            }
        }
    }
    pts.iter().zip(keep).filter_map(|(p, keep)| keep.then_some(*p)).collect()
}

/// Alias for [`simplify`] for freehand strokes.
pub fn simplify_freehand(pts: &[Point], tol: f64) -> Vec<Point> {
    simplify(pts, tol)
}

/// Sample by distance.
fn resample(pts: &[Point], length: f64) -> Vec<Point> {
    const SAMPLES: usize = 256;
    let (Some(&first), Some(&last)) = (pts.first(), pts.last()) else { return Vec::new() };
    let mut out = vec![first];
    let mut walked = 0.0;
    for (&a, &b) in pts.iter().zip(pts.iter().skip(1)) {
        let segment = a.distance(b);
        if segment <= 0.0 {
            continue;
        }
        while out.len() < SAMPLES - 1 {
            let target = length * (out.len() as f64 / (SAMPLES - 1) as f64);
            if target > walked + segment {
                break;
            }
            out.push(a + (b - a) * ((target - walked) / segment).clamp(0.0, 1.0));
        }
        walked += segment;
    }
    out.push(last);
    out
}

fn closed_corners(pts: &[Point], tol: f64) -> Vec<Point> {
    let Some(&first) = pts.first() else { return Vec::new() };
    let start = pts.iter().enumerate().max_by(|(_, a), (_, b)| a.distance(first).total_cmp(&b.distance(first))).map_or(0, |(i, _)| i);
    let ring: Vec<Point> = pts.iter().skip(start).chain(pts.iter().take(start)).copied().collect();
    let Some(&first) = ring.first() else { return Vec::new() };
    let split = ring.iter().enumerate().max_by(|(_, a), (_, b)| a.distance(first).total_cmp(&b.distance(first))).map_or(0, |(i, _)| i);
    let (Some(left), Some(right)) = (ring.get(..=split), ring.get(split..)) else { return Vec::new() };
    let mut corners = simplify(left, tol);
    corners.pop();
    let other: Vec<Point> = right.iter().copied().chain(std::iter::once(first)).collect();
    let mut other = simplify(&other, tol);
    other.pop();
    corners.extend(other);
    while corners.len() > 3 {
        let n = corners.len();
        let redundant = corners.iter().enumerate().find_map(|(i, p)| {
            let (&a, &b) = (corners.get((i + n - 1) % n)?, corners.get((i + 1) % n)?);
            (dist_to_segment(*p, a, b) <= tol).then_some(i)
        });
        let Some(i) = redundant else { break };
        corners.remove(i);
    }
    corners
}

fn area(poly: &[Point]) -> f64 {
    let Some(&origin) = poly.first() else { return 0.0 };
    poly.iter().zip(poly.iter().skip(1)).map(|(&a, &b)| (a - origin).cross(b - origin)).sum::<f64>().abs() / 2.0
}

fn is_rectangle(pts: &[Point], corners: &[Point], size: f64) -> bool {
    if corners.len() != 4 || !is_polygonal(pts, corners, size, 0.055) {
        return false;
    }
    right_angles(corners, size)
}

fn right_angles(corners: &[Point], size: f64) -> bool {
    let around: Vec<Point> = corners.iter().cycle().take(corners.len() + 2).copied().collect();
    around.windows(3).all(|w| {
        let [a, b, c] = w else { return false };
        let (u, v) = (*b - *a, *c - *b);
        let (lu, lv) = (u.hypot(), v.hypot());
        lu > 0.03 * size && lv > 0.03 * size && (u.dot(v) / (lu * lv)).abs() < 0.45
    })
}

fn polygon_rotation(corners: &[Point], step: f64) -> f64 {
    let Some(&origin) = corners.first() else { return 0.0 };
    let n = corners.len() as f64;
    let center = origin + corners.iter().fold(Vec2::ZERO, |sum, p| sum + (*p - origin) / n);
    let period = 360.0 / n;
    let error = |rotation: f64| {
        corners
            .iter()
            .map(|p| {
                let angle = (*p - center).atan2().to_degrees() + 90.0;
                let delta = (angle - rotation).rem_euclid(period);
                delta.min(period - delta).powi(2)
            })
            .sum::<f64>()
    };
    let candidates = [0.0, 90.0, 180.0, 270.0].into_iter().filter(|r| r % step == 0.0);
    let distinct = candidates.clone().enumerate().filter(|&(i, r)| !candidates.clone().take(i).any(|q| (r - q) % period == 0.0));
    distinct.map(|(_, r)| r).min_by(|a, b| error(*a).total_cmp(&error(*b))).unwrap_or(0.0)
}

fn rectangle_rotation(corners: &[Point]) -> f64 {
    let (sin, cos) = corners.iter().zip(corners.iter().cycle().skip(1)).fold((0.0, 0.0), |(sin, cos), (&a, &b)| {
        let angle = (b - a).atan2() * 4.0;
        (sin + angle.sin(), cos + angle.cos())
    });
    (sin.atan2(cos).to_degrees() / 4.0 / 45.0).round().mul_add(45.0, 0.0).rem_euclid(180.0)
}

fn ellipse_rotation(pts: &[Point]) -> f64 {
    let Some(&origin) = pts.first() else { return 0.0 };
    let n = pts.len() as f64;
    let center = origin + pts.iter().fold(Vec2::ZERO, |sum, p| sum + (*p - origin) / n);
    let (xx, yy, xy) = pts.iter().fold((0.0, 0.0, 0.0), |(xx, yy, xy), p| {
        let v = *p - center;
        (xx + v.x * v.x, yy + v.y * v.y, xy + v.x * v.y)
    });
    if (xx - yy).hypot(2.0 * xy) < 0.1 * (xx + yy) {
        return 0.0;
    }
    ((2.0 * xy).atan2(xx - yy).to_degrees() / 2.0 / 45.0).round().mul_add(45.0, 0.0).rem_euclid(180.0)
}

fn oriented_bounds(pts: &[Point], rotation: f64) -> Rect {
    if rotation == 0.0 {
        return bbox(pts);
    }
    let origin = pts.first().copied().unwrap_or(Point::ZERO);
    let rotate = Affine::rotate(rotation.to_radians());
    let inverse = Affine::rotate(-rotation.to_radians());
    let local: Vec<Point> = pts.iter().map(|p| inverse * (*p - origin).to_point()).collect();
    let b = bbox(&local);
    Rect::from_center_size(origin + (rotate * b.center()).to_vec2(), (b.width(), b.height()))
}

fn reversals(pts: &[Point], min_seg: f64) -> usize {
    let s = simplify(pts, min_seg);
    s.windows(3)
        .filter(|w| {
            let (u, v): (Vec2, Vec2) = (w[1] - w[0], w[2] - w[1]);
            u.hypot() > min_seg && v.hypot() > min_seg && u.dot(v) / (u.hypot() * v.hypot()) < -0.5
        })
        .count()
}

/// Recognise a freehand stroke.
pub fn recognize(pts: &[Point]) -> Option<Recognized> {
    if pts.len() < 2 || pts.iter().any(|p| !p.x.is_finite() || !p.y.is_finite()) {
        return None;
    }
    let b = bbox(pts);
    let size = b.width().max(b.height());
    if size < 2.0 || !size.is_finite() {
        return None;
    }
    let len = path_len(pts);
    if !len.is_finite() || len <= 0.0 {
        return None;
    }
    let original = pts;
    let samples = resample(pts, len);
    let pts = samples.as_slice();
    let (&first, &last) = (pts.first()?, pts.last()?);
    let chord = first.distance(last);

    if reversals(pts, size * 0.08) >= 2 && len > 1.5 * size {
        let corners = closed_corners(pts, size * 0.12);
        let triangle = chord <= 0.25 * size && corners.len() == 3 && area(pts) > 0.2 * size * size && is_polygonal(pts, &corners, size, 0.055);
        if !triangle {
            return Some(Recognized::Scribble(b));
        }
    }

    let max_dev = pts.iter().map(|p| dist_to_segment(*p, first, last)).fold(0.0, f64::max);
    if chord > 0.5 * size && max_dev < 0.08 * chord.max(1.0) && len < 1.3 * chord {
        return Some(Recognized::Line { a: first, b: last });
    }

    if chord > 0.25 * size {
        return None;
    }
    let corners = closed_corners(pts, size * 0.12);
    let fill = area(pts) / (b.width() * b.height()).max(1e-9);
    if !fill.is_finite() {
        return None;
    }
    let center = b.center();
    let square = |r: Rect| {
        let (w, h) = (r.width(), r.height());
        if (w - h).abs() < 0.1 * w.max(h) {
            let s = (w + h) / 2.0;
            Rect::from_center_size(r.center(), (s, s))
        } else {
            r
        }
    };
    match corners.len() {
        3 => {
            let radius = corners.iter().map(|p| p.distance(center)).sum::<f64>() / 3.0;
            Some(Recognized::Polygon { center, radius, sides: 3, rotation: polygon_rotation(&corners, 180.0) })
        }
        4 if is_rectangle(pts, &corners, size) => {
            let rotation = rectangle_rotation(&corners);
            Some(Recognized::Rectangle { rect: square(oriented_bounds(original, rotation)), rotation })
        }
        n @ 5..=8 if fill > 0.6 && fill < 0.85 && is_polygonal(pts, &corners, size, 0.02) => {
            let radius = corners.iter().map(|p| p.distance(center)).sum::<f64>() / n as f64;
            Some(Recognized::Polygon { center, radius, sides: n as u32, rotation: if n == 6 { polygon_rotation(&corners, 90.0) } else { 0.0 } })
        }
        _ => {
            let rotation = ellipse_rotation(pts);
            let rect = oriented_bounds(original, rotation);
            (area(pts) / (rect.width() * rect.height()).max(1e-9) > 0.6).then(|| Recognized::Ellipse { rect: square(rect), rotation })
        }
    }
}

/// Alias for [`recognize`].
pub fn recognize_shape(pts: &[Point]) -> Option<Recognized> {
    recognize(pts)
}

fn is_polygonal(pts: &[Point], corners: &[Point], size: f64, tolerance: f64) -> bool {
    if pts.is_empty() || corners.len() < 3 {
        return false;
    }
    let dev: f64 = pts
        .iter()
        .map(|p| corners.iter().zip(corners.iter().cycle().skip(1)).map(|(&a, &b)| dist_to_segment(*p, a, b)).fold(f64::MAX, f64::min))
        .sum::<f64>()
        / pts.len() as f64;
    dev < tolerance * size
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wobbly(corners: &[Point], k: usize, wobble: f64) -> Vec<Point> {
        let mut out = vec![];
        let n = corners.len();
        for i in 0..n {
            let (a, b) = (corners[i], corners[(i + 1) % n]);
            for j in 0..k {
                let t = j as f64 / k as f64;
                let w = ((i * k + j) as f64 * 1.7).sin() * wobble;
                out.push(a + (b - a) * t + Vec2::new(w, -w));
            }
        }
        out.push(corners[0] + Vec2::new(3.0, 2.0));
        out
    }

    fn circle(c: Point, rx: f64, ry: f64, k: usize) -> Vec<Point> {
        (0..=k)
            .map(|i| {
                let a = std::f64::consts::TAU * i as f64 / k as f64;
                c + Vec2::new(rx * a.cos() + (a * 5.0).sin(), ry * a.sin())
            })
            .collect()
    }

    #[test]
    fn rough_rectangle_and_square() {
        let r = wobbly(&[Point::new(0.0, 0.0), Point::new(200.0, 0.0), Point::new(200.0, 100.0), Point::new(0.0, 100.0)], 20, 2.0);
        let Some(Recognized::Rectangle { rect: b, .. }) = recognize_shape(&r) else { panic!("{:?}", recognize(&r)) };
        assert!((b.width() - 200.0).abs() < 8.0 && (b.height() - 100.0).abs() < 8.0);
        let s = wobbly(&[Point::new(0.0, 0.0), Point::new(100.0, 0.0), Point::new(100.0, 95.0), Point::new(0.0, 95.0)], 20, 1.0);
        let Some(Recognized::Rectangle { rect: b, .. }) = recognize_shape(&s) else { panic!() };
        assert!((b.width() - b.height()).abs() < 1e-9);
    }

    #[test]
    fn ellipse_circle_triangle_hexagon_line() {
        assert!(matches!(recognize_shape(&circle(Point::new(100.0, 100.0), 80.0, 40.0, 72)), Some(Recognized::Ellipse { rect: r, .. }) if r.width() > 150.0 && r.height() < 100.0));
        let tri = wobbly(&[Point::new(100.0, 0.0), Point::new(200.0, 170.0), Point::new(0.0, 170.0)], 25, 1.5);
        assert!(matches!(recognize_shape(&tri), Some(Recognized::Polygon { sides: 3, rotation, .. }) if rotation.abs() < 5.0));
        let line: Vec<Point> = (0..30).map(|i| Point::new(i as f64 * 10.0, i as f64 * 3.0 + (i as f64).sin())).collect();
        assert!(matches!(recognize_shape(&line), Some(Recognized::Line { .. })));
    }

    #[test]
    fn scribble_and_noise() {
        let z: Vec<Point> = (0..40).map(|i| Point::new(100.0 + (i % 2) as f64 * 60.0, 100.0 + i as f64 * 2.0)).collect();
        assert!(matches!(recognize_shape(&z), Some(Recognized::Scribble(_))));
        assert_eq!(recognize_shape(&[Point::new(0.0, 0.0)]), None);
    }

    #[test]
    fn simplify_freehand_test() {
        let pts = vec![Point::new(0.0, 0.0), Point::new(5.0, 0.1), Point::new(10.0, 0.0)];
        let simp = simplify_freehand(&pts, 0.5);
        assert_eq!(simp.len(), 2);
    }
}
