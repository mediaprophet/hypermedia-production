//! Deterministic Cholesky factorization and linear solver.
//!
//! Grounded in algorithmic reproducibility and numerical determinism (UDHR Art. 19, 27).

/// Dense / profile Cholesky factorization of symmetric positive-definite matrix: A = L * L^T.
#[derive(Clone, Debug)]
pub struct CholeskyFactor {
    pub n: usize,
    pub l_values: Vec<f64>,
}

impl CholeskyFactor {
    /// Factorize symmetric positive-definite n x n matrix in row-major slice.
    pub fn factorize(n: usize, a: &[f64]) -> Result<Self, &'static str> {
        assert_eq!(a.len(), n * n);
        let mut l = vec![0.0; n * n];

        for i in 0..n {
            for j in 0..=i {
                let mut sum = 0.0;
                for k in 0..j {
                    sum += l[i * n + k] * l[j * n + k];
                }

                if i == j {
                    let d = a[i * n + i] - sum;
                    if d <= 0.0 {
                        return Err("Matrix is not positive definite in Cholesky factorization");
                    }
                    l[i * n + j] = d.sqrt();
                } else {
                    let lj = l[j * n + j];
                    if lj.abs() < 1e-15 {
                        return Err("Zero pivot in Cholesky factorization");
                    }
                    l[i * n + j] = (a[i * n + j] - sum) / lj;
                }
            }
        }

        Ok(Self { n, l_values: l })
    }

    /// Solve A * x = b via forward substitution (L * y = b) and backward substitution (L^T * x = y).
    pub fn solve(&self, b: &[f64]) -> Vec<f64> {
        let n = self.n;
        assert_eq!(b.len(), n);

        // 1. Forward substitution: L * y = b
        let mut y = vec![0.0; n];
        for i in 0..n {
            let mut sum = 0.0;
            for k in 0..i {
                sum += self.l_values[i * n + k] * y[k];
            }
            y[i] = (b[i] - sum) / self.l_values[i * n + i];
        }

        // 2. Backward substitution: L^T * x = y
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = 0.0;
            for k in (i + 1)..n {
                sum += self.l_values[k * n + i] * x[k];
            }
            x[i] = (y[i] - sum) / self.l_values[i * n + i];
        }

        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cholesky_factorize_and_solve_2x2() {
        let a = [4.0, 12.0, 12.0, 45.0];
        let chol = CholeskyFactor::factorize(2, &a).unwrap();
        let b = [16.0, 57.0];
        let x = chol.solve(&b);
        assert!((x[0] - 1.0).abs() < 1e-10);
        assert!((x[1] - 1.0).abs() < 1e-10);
    }
}
