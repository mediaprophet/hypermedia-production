//! Double-precision vector primitives and second-order forward automatic differentiation (AD).
//! Grounded in deterministic physical mechanics and informatics fiduciary duties.

use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

/// Double-precision 2D vector.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DVec2 {
    pub x: f64,
    pub y: f64,
}

impl DVec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    #[inline(always)]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    #[inline(always)]
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn distance(self, other: Self) -> f64 {
        (self - other).length()
    }

    #[inline(always)]
    pub fn lerp(self, other: Self, t: f64) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    #[inline(always)]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl Add for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f64> for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for DVec2 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

/// Double-precision 3D vector.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl DVec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    #[inline(always)]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline(always)]
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline(always)]
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline(always)]
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    #[inline(always)]
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline(always)]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len > 1e-15 {
            self / len
        } else {
            Self::ZERO
        }
    }

    #[inline(always)]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl Add for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for DVec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for DVec3 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for DVec3 {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

/// 2x2 double precision matrix (column-major).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DMat2 {
    pub x_axis: DVec2,
    pub y_axis: DVec2,
}

impl DMat2 {
    pub fn from_cols(x_axis: DVec2, y_axis: DVec2) -> Self {
        Self { x_axis, y_axis }
    }

    pub fn determinant(self) -> f64 {
        self.x_axis.x * self.y_axis.y - self.x_axis.y * self.y_axis.x
    }

    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < 1e-15 {
            return None;
        }
        let inv_det = 1.0 / det;
        Some(Self {
            x_axis: DVec2::new(self.y_axis.y * inv_det, -self.x_axis.y * inv_det),
            y_axis: DVec2::new(-self.y_axis.x * inv_det, self.x_axis.x * inv_det),
        })
    }
}

/// Scalar trait for automatic differentiation numbers and regular floats.
pub trait Scalar:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
    fn c(v: f64) -> Self;
    fn value(self) -> f64;
    fn sqrt(self) -> Self;
    fn ln(self) -> Self;
}

impl Scalar for f64 {
    #[inline(always)]
    fn c(v: f64) -> Self {
        v
    }
    #[inline(always)]
    fn value(self) -> f64 {
        self
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    #[inline(always)]
    fn ln(self) -> Self {
        self.ln()
    }
}

/// Second-order forward automatic differentiation jet with N independent variables.
#[derive(Clone, Copy, Debug)]
pub struct Jet<const N: usize> {
    pub v: f64,
    pub g: [f64; N],
    pub h: [[f64; N]; N],
}

impl<const N: usize> Jet<N> {
    pub fn variable(v: f64, i: usize) -> Self {
        let mut x = Self::c(v);
        x.g[i] = 1.0;
        x
    }

    fn chain(self, value: f64, first: f64, second: f64) -> Self {
        Self {
            v: value,
            g: std::array::from_fn(|i| first * self.g[i]),
            h: std::array::from_fn(|i| {
                std::array::from_fn(|j| first * self.h[i][j] + second * self.g[i] * self.g[j])
            }),
        }
    }
}

impl<const N: usize> Scalar for Jet<N> {
    fn c(v: f64) -> Self {
        Self {
            v,
            g: [0.0; N],
            h: [[0.0; N]; N],
        }
    }

    fn value(self) -> f64 {
        self.v
    }

    fn sqrt(self) -> Self {
        let r = self.v.sqrt();
        self.chain(r, 0.5 / r, -0.25 / (r * r * r))
    }

    fn ln(self) -> Self {
        self.chain(self.v.ln(), 1.0 / self.v, -1.0 / (self.v * self.v))
    }
}

impl<const N: usize> Add for Jet<N> {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self {
            v: self.v + b.v,
            g: std::array::from_fn(|i| self.g[i] + b.g[i]),
            h: std::array::from_fn(|i| std::array::from_fn(|j| self.h[i][j] + b.h[i][j])),
        }
    }
}

impl<const N: usize> Neg for Jet<N> {
    type Output = Self;
    fn neg(self) -> Self {
        self * Self::c(-1.0)
    }
}

impl<const N: usize> Sub for Jet<N> {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        self + -b
    }
}

impl<const N: usize> Mul for Jet<N> {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        Self {
            v: self.v * b.v,
            g: std::array::from_fn(|i| self.v * b.g[i] + self.g[i] * b.v),
            h: std::array::from_fn(|i| {
                std::array::from_fn(|j| {
                    self.v * b.h[i][j]
                        + self.h[i][j] * b.v
                        + self.g[i] * b.g[j]
                        + self.g[j] * b.g[i]
                })
            }),
        }
    }
}

impl<const N: usize> Div for Jet<N> {
    type Output = Self;
    fn div(self, b: Self) -> Self {
        let inv_b = Self::c(1.0).chain(1.0 / b.v, -1.0 / (b.v * b.v), 2.0 / (b.v * b.v * b.v));
        self * inv_b
    }
}

#[inline(always)]
pub fn dot<S: Scalar>(a: [S; 3], b: [S; 3]) -> S {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline(always)]
pub fn sub<S: Scalar>(a: [S; 3], b: [S; 3]) -> [S; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline(always)]
pub fn add<S: Scalar>(a: [S; 3], b: [S; 3]) -> [S; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline(always)]
pub fn cross<S: Scalar>(a: [S; 3], b: [S; 3]) -> [S; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
