//! Symmetric indefinite LDL^T factorization and inertia count.
//!
//! Grounded in structural stability verification and constraint determinism (UDHR Art. 19, 27).

/// Inertia of a symmetric matrix: count of positive, negative, and zero eigenvalues.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inertia {
    pub positive: usize,
    pub negative: usize,
    pub zero: usize,
}

/// LDL^T factorization: A = L * D * L^T where L is unit lower triangular and D is diagonal.
#[derive(Clone, Debug)]
pub struct LdltFactor {
    pub n: usize,
    pub l_values: Vec<f64>,
    pub d_values: Vec<f64>,
}

impl LdltFactor {
    /// Factorize symmetric n x n matrix.
    pub fn factorize(n: usize, a: &[f64]) -> Result<Self, &'static str> {
        assert_eq!(a.len(), n * n);
        let mut l = vec![0.0; n * n];
        let mut d = vec![0.0; n];

        // Diagonal of L is 1
        for i in 0..n {
            l[i * n + i] = 1.0;
        }

        for j in 0..n {
            let mut sum_d = 0.0;
            for k in 0..j {
                sum_d += l[j * n + k] * l[j * n + k] * d[k];
            }
            d[j] = a[j * n + j] - sum_d;

            if d[j].abs() < 1e-15 {
                d[j] = 0.0;
                continue;
            }

            let inv_dj = 1.0 / d[j];
            for i in (j + 1)..n {
                let mut sum_l = 0.0;
                for k in 0..j {
                    sum_l += l[i * n + k] * l[j * n + k] * d[k];
                }
                l[i * n + j] = (a[i * n + j] - sum_l) * inv_dj;
            }
        }

        Ok(Self {
            n,
            l_values: l,
            d_values: d,
        })
    }

    /// Compute Sylvester's inertia from the diagonal entries of D.
    pub fn inertia(&self, tol: f64) -> Inertia {
        let mut pos = 0;
        let mut neg = 0;
        let mut zero = 0;

        for &val in &self.d_values {
            if val > tol {
                pos += 1;
            } else if val < -tol {
                neg += 1;
            } else {
                zero += 1;
            }
        }

        Inertia {
            positive: pos,
            negative: neg,
            zero,
        }
    }

    /// Solve A * x = b via L * z = b, D * y = z, L^T * x = y.
    pub fn solve(&self, b: &[f64]) -> Result<Vec<f64>, &'static str> {
        let n = self.n;
        assert_eq!(b.len(), n);

        // 1. Forward substitution: L * z = b
        let mut z = vec![0.0; n];
        for i in 0..n {
            let mut sum = 0.0;
            for k in 0..i {
                sum += self.l_values[i * n + k] * z[k];
            }
            z[i] = b[i] - sum;
        }

        // 2. Diagonal solve: D * y = z
        let mut y = vec![0.0; n];
        for i in 0..n {
            if self.d_values[i].abs() < 1e-15 {
                return Err("Singular matrix in LDL^T diagonal solve");
            }
            y[i] = z[i] / self.d_values[i];
        }

        // 3. Backward substitution: L^T * x = y
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = 0.0;
            for k in (i + 1)..n {
                sum += self.l_values[k * n + i] * x[k];
            }
            x[i] = y[i] - sum;
        }

        Ok(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ldlt_inertia_count() {
        // Indefinite matrix [1, 2; 2, 1] with eigenvalues 3 and -1
        let a = [1.0, 2.0, 2.0, 1.0];
        let ldlt = LdltFactor::factorize(2, &a).unwrap();
        let inertia = ldlt.inertia(1e-10);
        assert_eq!(inertia.positive, 1);
        assert_eq!(inertia.negative, 1);
        assert_eq!(inertia.zero, 0);

        let b = [5.0, 4.0];
        let x = ldlt.solve(&b).unwrap();
        // A * x = [1, 2; 2, 1] * [-1, 3] = [5, 4]
        assert!((x[0] - (-1.0)).abs() < 1e-10);
        assert!((x[1] - 3.0).abs() < 1e-10);
    }
}
