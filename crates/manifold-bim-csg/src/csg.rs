//! Work-metered, non-recursive 3D BSP solid constructive solid geometry (CSG).
//!
//! Provides boolean operations (`subtract`, `union`, `intersect`) on closed surfaces of triangles.
//! Trees are built and traversed iteratively with worklists to run safely within bounded call stacks,
//! and operations are counted against a [`Meter`] budget to guarantee termination.

pub type P3 = [f64; 3];

#[inline]
pub fn add(a: P3, b: P3) -> P3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
pub fn sub(a: P3, b: P3) -> P3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
pub fn mul(a: P3, s: f64) -> P3 {
    [a[0] * s, a[1] * s, a[2] * s]
}

#[inline]
pub fn dot(a: P3, b: P3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
pub fn cross(a: P3, b: P3) -> P3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Triangles either solid may contain.
pub const MAX_TRIANGLES: usize = 20_000;
/// Plane distance tolerance in millimeters.
pub const EPS: f64 = 1e-5;
/// Triangle sliver area threshold in square millimeters.
pub const SLIVER: f64 = 1e-6;

/// Execution budget: corners evaluated against cutting planes and polygon splits.
#[derive(Clone, Copy, Debug)]
pub struct Meter {
    pub work: u64,
    pub splits: u64,
}

impl Default for Meter {
    fn default() -> Meter {
        Meter {
            work: 40_000_000,
            splits: 400_000,
        }
    }
}

const TOO_MUCH: &str = "solid operation exceeded budget: simplify geometry or reduce triangle count";

impl Meter {
    pub fn new(work: u64, splits: u64) -> Self {
        Self { work, splits }
    }

    fn work(&mut self, corners: usize) -> Result<(), String> {
        self.work = self.work.checked_sub(corners as u64 + 1).ok_or(TOO_MUCH)?;
        Ok(())
    }

    fn split(&mut self) -> Result<(), String> {
        self.splits = self.splits.checked_sub(1).ok_or(TOO_MUCH)?;
        Ok(())
    }
}

/// Convex polygon with counter-clockwise vertices seen from its normal `n`.
#[derive(Clone, Debug)]
pub struct Poly {
    pub v: Vec<P3>,
    pub n: P3,
    pub w: f64,
}

impl Poly {
    pub fn of(t: &[P3; 3]) -> Option<Poly> {
        let n = cross(sub(t[1], t[0]), sub(t[2], t[0]));
        let len = dot(n, n).sqrt();
        (len > 2.0 * SLIVER).then(|| {
            let n = mul(n, 1.0 / len);
            Poly {
                v: t.to_vec(),
                n,
                w: dot(n, t[0]),
            }
        })
    }

    pub fn flip(&mut self) {
        self.v.reverse();
        self.n = mul(self.n, -1.0);
        self.w = -self.w;
    }

    pub fn triangles(&self) -> Vec<[P3; 3]> {
        let first = match self.v.first().copied() {
            Some(f) => f,
            None => return Vec::new(),
        };
        self.v
            .windows(2)
            .skip(1)
            .map(|w| [first, w[0], w[1]])
            .filter(|[a, b, c]| {
                let n = cross(sub(*b, *a), sub(*c, *a));
                dot(n, n) > (2.0 * SLIVER).powi(2)
            })
            .collect()
    }
}

enum Side {
    On(Poly, bool),
    Front(Poly),
    Back(Poly),
    Across(Option<Poly>, Option<Poly>),
}

fn divide(n: P3, w: f64, p: Poly) -> Side {
    let kinds: Vec<i8> = p
        .v
        .iter()
        .map(|c| {
            let d = dot(n, *c) - w;
            if d > EPS {
                1
            } else if d < -EPS {
                -1
            } else {
                0
            }
        })
        .collect();

    match (kinds.contains(&1), kinds.contains(&-1)) {
        (false, false) => {
            let same = dot(n, p.n) > 0.0;
            Side::On(p, same)
        }
        (true, false) => Side::Front(p),
        (false, true) => Side::Back(p),
        (true, true) => {
            let (mut front, mut back) = (Vec::new(), Vec::new());
            let m = p.v.len();
            for i in 0..m {
                let (Some(&a), Some(&b), Some(&ka), Some(&kb)) = (
                    p.v.get(i),
                    p.v.get((i + 1) % m),
                    kinds.get(i),
                    kinds.get((i + 1) % m),
                ) else {
                    continue;
                };
                if ka != -1 {
                    front.push(a);
                }
                if ka != 1 {
                    back.push(a);
                }
                if ka * kb == -1 {
                    let c = add(a, mul(sub(b, a), (w - dot(n, a)) / dot(n, sub(b, a))));
                    front.push(c);
                    back.push(c);
                }
            }
            let part = |v: Vec<P3>| (v.len() >= 3).then_some(Poly { v, n: p.n, w: p.w });
            Side::Across(part(front), part(back))
        }
    }
}

struct Node {
    n: P3,
    w: f64,
    polys: Vec<Poly>,
    front: Option<usize>,
    back: Option<usize>,
}

pub struct Bsp {
    nodes: Vec<Node>,
}

impl Bsp {
    pub fn of(tris: &[[P3; 3]], meter: &mut Meter) -> Result<Bsp, String> {
        let mut bsp = Bsp { nodes: Vec::new() };
        bsp.build(tris.iter().filter_map(Poly::of).collect(), meter)?;
        Ok(bsp)
    }

    fn node(first: &Poly) -> Node {
        Node {
            n: first.n,
            w: first.w,
            polys: Vec::new(),
            front: None,
            back: None,
        }
    }

    pub fn build(&mut self, polys: Vec<Poly>, meter: &mut Meter) -> Result<(), String> {
        let Some(first) = polys.first() else {
            return Ok(());
        };
        if self.nodes.is_empty() {
            self.nodes.push(Bsp::node(first));
        }
        let mut stack = vec![(0, polys)];
        while let Some((i, polys)) = stack.pop() {
            let Some((n, w)) = self.nodes.get(i).map(|x| (x.n, x.w)) else {
                continue;
            };
            let (mut here, mut front, mut back) = (Vec::new(), Vec::new(), Vec::new());
            for p in polys {
                meter.work(p.v.len())?;
                match divide(n, w, p) {
                    Side::On(p, _) => here.push(p),
                    Side::Front(p) => front.push(p),
                    Side::Back(p) => back.push(p),
                    Side::Across(f, b) => {
                        meter.split()?;
                        front.extend(f);
                        back.extend(b);
                    }
                }
            }
            if let Some(node) = self.nodes.get_mut(i) {
                node.polys.extend(here);
            }
            for (list, ahead) in [(front, true), (back, false)] {
                let Some(first) = list.first() else {
                    continue;
                };
                let known = self
                    .nodes
                    .get(i)
                    .and_then(|x| if ahead { x.front } else { x.back });
                let child = known.unwrap_or_else(|| {
                    let c = self.nodes.len();
                    self.nodes.push(Bsp::node(first));
                    c
                });
                if let Some(node) = self.nodes.get_mut(i) {
                    *(if ahead {
                        &mut node.front
                    } else {
                        &mut node.back
                    }) = Some(child);
                }
                stack.push((child, list));
            }
        }
        Ok(())
    }

    pub fn clip(&self, polys: Vec<Poly>, meter: &mut Meter) -> Result<Vec<Poly>, String> {
        if self.nodes.is_empty() {
            return Ok(polys);
        }
        let mut kept = Vec::new();
        let mut stack = vec![(0, polys)];
        while let Some((i, polys)) = stack.pop() {
            let Some(node) = self.nodes.get(i) else {
                continue;
            };
            let (mut front, mut back) = (Vec::new(), Vec::new());
            for p in polys {
                meter.work(p.v.len())?;
                match divide(node.n, node.w, p) {
                    Side::On(p, true) | Side::Front(p) => front.push(p),
                    Side::On(p, false) | Side::Back(p) => back.push(p),
                    Side::Across(f, b) => {
                        meter.split()?;
                        front.extend(f);
                        back.extend(b);
                    }
                }
            }
            match node.front {
                Some(c) => stack.push((c, front)),
                None => kept.extend(front),
            }
            if let Some(c) = node.back {
                stack.push((c, back));
            }
        }
        Ok(kept)
    }

    pub fn clip_to(&mut self, other: &Bsp, meter: &mut Meter) -> Result<(), String> {
        for node in &mut self.nodes {
            node.polys = other.clip(std::mem::take(&mut node.polys), meter)?;
        }
        Ok(())
    }

    pub fn invert(&mut self) {
        for node in &mut self.nodes {
            node.polys.iter_mut().for_each(Poly::flip);
            node.n = mul(node.n, -1.0);
            node.w = -node.w;
            std::mem::swap(&mut node.front, &mut node.back);
        }
    }

    pub fn polygons(&self) -> Vec<Poly> {
        self.nodes.iter().flat_map(|n| n.polys.iter().cloned()).collect()
    }
}

pub fn bounds(tris: &[[P3; 3]]) -> (P3, P3) {
    tris.iter().flatten().fold(
        ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]),
        |(lo, hi), p| {
            (
                std::array::from_fn(|k| lo[k].min(p[k])),
                std::array::from_fn(|k| hi[k].max(p[k])),
            )
        },
    )
}

/// Subtracts `void` solid from `solid`.
pub fn subtract(
    solid: &[[P3; 3]],
    void: &[[P3; 3]],
    meter: &mut Meter,
) -> Result<Vec<[P3; 3]>, String> {
    for (what, n) in [("the form cut", solid.len()), ("the void", void.len())] {
        if n > MAX_TRIANGLES {
            return Err(format!("{what} has {n} triangles; cannot exceed {MAX_TRIANGLES}"));
        }
    }
    let ((lo_s, hi_s), (lo_v, hi_v)) = (bounds(solid), bounds(void));
    if (0..3).any(|k| lo_v[k] > hi_s[k] + EPS || hi_v[k] < lo_s[k] - EPS) {
        return Ok(solid.to_vec());
    }
    let (mut a, mut b) = (Bsp::of(solid, meter)?, Bsp::of(void, meter)?);
    a.invert();
    a.clip_to(&b, meter)?;
    b.clip_to(&a, meter)?;
    b.invert();
    b.clip_to(&a, meter)?;
    b.invert();
    a.build(b.polygons(), meter)?;
    a.invert();
    Ok(a.polygons().iter().flat_map(Poly::triangles).collect())
}

/// Unions two solids together.
pub fn union(
    solid_a: &[[P3; 3]],
    solid_b: &[[P3; 3]],
    meter: &mut Meter,
) -> Result<Vec<[P3; 3]>, String> {
    for (what, n) in [("solid A", solid_a.len()), ("solid B", solid_b.len())] {
        if n > MAX_TRIANGLES {
            return Err(format!("{what} has {n} triangles; cannot exceed {MAX_TRIANGLES}"));
        }
    }
    let (mut a, mut b) = (Bsp::of(solid_a, meter)?, Bsp::of(solid_b, meter)?);
    a.clip_to(&b, meter)?;
    b.clip_to(&a, meter)?;
    b.invert();
    b.clip_to(&a, meter)?;
    b.invert();
    a.build(b.polygons(), meter)?;
    Ok(a.polygons().iter().flat_map(Poly::triangles).collect())
}

/// Intersects two solids.
pub fn intersect(
    solid_a: &[[P3; 3]],
    solid_b: &[[P3; 3]],
    meter: &mut Meter,
) -> Result<Vec<[P3; 3]>, String> {
    for (what, n) in [("solid A", solid_a.len()), ("solid B", solid_b.len())] {
        if n > MAX_TRIANGLES {
            return Err(format!("{what} has {n} triangles; cannot exceed {MAX_TRIANGLES}"));
        }
    }
    let ((lo_a, hi_a), (lo_b, hi_b)) = (bounds(solid_a), bounds(solid_b));
    if (0..3).any(|k| lo_b[k] > hi_a[k] + EPS || hi_b[k] < lo_a[k] - EPS) {
        return Ok(Vec::new());
    }
    let (mut a, mut b) = (Bsp::of(solid_a, meter)?, Bsp::of(solid_b, meter)?);
    a.invert();
    b.clip_to(&a, meter)?;
    b.invert();
    a.clip_to(&b, meter)?;
    b.clip_to(&a, meter)?;
    a.build(b.polygons(), meter)?;
    a.invert();
    Ok(a.polygons().iter().flat_map(Poly::triangles).collect())
}

/// Signed volume of a closed triangle mesh.
pub fn volume(tris: &[[P3; 3]]) -> f64 {
    tris.iter().map(|[a, b, c]| dot(*a, cross(*b, *c)) / 6.0).sum()
}

/// Surface area of a triangle mesh.
pub fn area(tris: &[[P3; 3]]) -> f64 {
    tris.iter()
        .map(|[a, b, c]| {
            let n = cross(sub(*b, *a), sub(*c, *a));
            dot(n, n).sqrt() / 2.0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn cuboid(lo: P3, hi: P3) -> Vec<[P3; 3]> {
        let v = |x: usize, y: usize, z: usize| [[lo[0], hi[0]][x], [lo[1], hi[1]][y], [lo[2], hi[2]][z]];
        [
            [v(0, 0, 1), v(1, 0, 1), v(1, 1, 1), v(0, 1, 1)],
            [v(0, 0, 0), v(0, 1, 0), v(1, 1, 0), v(1, 0, 0)],
            [v(0, 0, 0), v(1, 0, 0), v(1, 0, 1), v(0, 0, 1)],
            [v(1, 1, 0), v(0, 1, 0), v(0, 1, 1), v(1, 1, 1)],
            [v(0, 1, 0), v(0, 0, 0), v(0, 0, 1), v(0, 1, 1)],
            [v(1, 0, 0), v(1, 1, 0), v(1, 1, 1), v(1, 0, 1)],
        ]
        .iter()
        .flat_map(|[a, b, c, d]| [[*a, *b, *c], [*a, *c, *d]])
        .collect()
    }

    #[test]
    fn csg_subtract_pocket() {
        let slab = cuboid([0.0, 0.0, 0.0], [1000.0, 1000.0, 100.0]);
        let pocket = cuboid([400.0, 400.0, 50.0], [600.0, 600.0, 100.0]);
        let mut meter = Meter::default();
        let cut = subtract(&slab, &pocket, &mut meter).unwrap();
        let v = volume(&cut);
        assert!((v - (1e8 - 2e6)).abs() < 1e-3, "volume was {v}");
    }

    #[test]
    fn csg_union_and_intersect() {
        let box_a = cuboid([0.0, 0.0, 0.0], [100.0, 100.0, 100.0]);
        let box_b = cuboid([50.0, 0.0, 0.0], [150.0, 100.0, 100.0]);
        let mut meter = Meter::default();

        let inter = intersect(&box_a, &box_b, &mut meter).unwrap();
        let v_inter = volume(&inter);
        assert!((v_inter - 500_000.0).abs() < 1e-3, "intersect volume was {v_inter}");

        let mut meter2 = Meter::default();
        let u = union(&box_a, &box_b, &mut meter2).unwrap();
        let v_u = volume(&u);
        assert!((v_u - 1_500_000.0).abs() < 1e-3, "union volume was {v_u}");
    }

    #[test]
    fn meter_budget_exhaustion() {
        let tube = cuboid([0.0, 0.0, 0.0], [100.0, 100.0, 100.0]);
        let half = cuboid([-10.0, -10.0, -10.0], [50.0, 110.0, 110.0]);
        let mut poor = Meter::new(10, 10);
        let res = subtract(&tube, &half, &mut poor);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("budget"));
    }
}
