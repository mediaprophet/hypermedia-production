//! 4-point perspective corner-pin tracking and 2D planar homography solvers.
//!
//! Grounded in Direct Linear Transform (DLT) algorithm for projective geometry (Hartley & Zisserman).

/// A 3x3 projective transformation matrix (Homography).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Homography(pub [[f64; 3]; 3]);

impl Default for Homography {
    fn default() -> Self {
        Self([
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ])
    }
}

impl Homography {
    /// Transform a 2D point [x, y] by this homography.
    pub fn transform_point(&self, pt: [f64; 2]) -> [f64; 2] {
        let m = self.0;
        let x = pt[0];
        let y = pt[1];
        let w = m[2][0] * x + m[2][1] * y + m[2][2];
        let denom = if w.abs() > 1e-9 { w } else { 1.0 };
        let px = (m[0][0] * x + m[0][1] * y + m[0][2]) / denom;
        let py = (m[1][0] * x + m[1][1] * y + m[1][2]) / denom;
        [px, py]
    }
}

/// Solve for the 3x3 homography mapping 4 source points to 4 destination points using DLT.
pub fn solve_corner_pin_homography(src: &[[f64; 2]; 4], dst: &[[f64; 2]; 4]) -> Option<Homography> {
    // 8x8 linear system: A * h = b with h_33 = 1
    let mut a = [[0.0f64; 8]; 8];
    let mut b = [0.0f64; 8];

    for i in 0..4 {
        let (x, y) = (src[i][0], src[i][1]);
        let (u, v) = (dst[i][0], dst[i][1]);

        // Row 2*i: x, y, 1, 0, 0, 0, -u*x, -u*y = u
        a[2 * i] = [x, y, 1.0, 0.0, 0.0, 0.0, -u * x, -u * y];
        b[2 * i] = u;

        // Row 2*i + 1: 0, 0, 0, x, y, 1, -v*x, -v*y = v
        a[2 * i + 1] = [0.0, 0.0, 0.0, x, y, 1.0, -v * x, -v * y];
        b[2 * i + 1] = v;
    }

    // Solve 8x8 system via Gaussian elimination with partial pivoting
    let h = solve_8x8(&mut a, &mut b)?;

    Some(Homography([
        [h[0], h[1], h[2]],
        [h[3], h[4], h[5]],
        [h[6], h[7], 1.0],
    ]))
}

fn solve_8x8(a: &mut [[f64; 8]; 8], b: &mut [f64; 8]) -> Option<[f64; 8]> {
    const N: usize = 8;
    for i in 0..N {
        // Partial pivoting
        let mut max_row = i;
        let mut max_val = a[i][i].abs();
        for r in i + 1..N {
            if a[r][i].abs() > max_val {
                max_val = a[r][i].abs();
                max_row = r;
            }
        }
        if max_val < 1e-12 {
            return None; // Singular matrix
        }
        if max_row != i {
            a.swap(i, max_row);
            b.swap(i, max_row);
        }

        let pivot = a[i][i];
        for c in i..N {
            a[i][c] /= pivot;
        }
        b[i] /= pivot;

        for r in 0..N {
            if r != i {
                let factor = a[r][i];
                for c in i..N {
                    a[r][c] -= factor * a[i][c];
                }
                b[r] -= factor * b[i];
            }
        }
    }

    let mut result = [0.0; N];
    result.copy_from_slice(b);
    Some(result)
}
