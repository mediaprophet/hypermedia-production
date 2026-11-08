//! Compressed sparse matrix representation and operations.
//!
//! Grounded in algorithmic determinism and verifiable scientific computation (UDHR Art. 19, 27).

use serde::{Deserialize, Serialize};

/// Compressed Sparse Row (CSR) matrix.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CsrMatrix {
    pub nrows: usize,
    pub ncols: usize,
    pub row_ptrs: Vec<usize>,
    pub col_indices: Vec<usize>,
    pub values: Vec<f64>,
}

impl CsrMatrix {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        Self {
            nrows,
            ncols,
            row_ptrs: vec![0; nrows + 1],
            col_indices: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Construct CSR matrix from triplets (row, col, value).
    pub fn from_triplets(nrows: usize, ncols: usize, mut triplets: Vec<(usize, usize, f64)>) -> Self {
        triplets.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        let mut row_ptrs = vec![0; nrows + 1];
        let mut col_indices = Vec::new();
        let mut values = Vec::new();

        for &(r, _, _) in &triplets {
            if r < nrows {
                row_ptrs[r + 1] += 1;
            }
        }

        // Prefix sum
        for i in 0..nrows {
            row_ptrs[i + 1] += row_ptrs[i];
        }

        let mut curr_row_ptrs = row_ptrs.clone();
        col_indices.resize(triplets.len(), 0);
        values.resize(triplets.len(), 0.0);

        for (r, c, v) in triplets {
            if r < nrows && c < ncols {
                let dest = curr_row_ptrs[r];
                col_indices[dest] = c;
                values[dest] = v;
                curr_row_ptrs[r] += 1;
            }
        }

        Self {
            nrows,
            ncols,
            row_ptrs,
            col_indices,
            values,
        }
    }

    /// Matrix-vector multiplication y = A * x.
    pub fn matvec(&self, x: &[f64]) -> Vec<f64> {
        assert_eq!(x.len(), self.ncols);
        let mut y = vec![0.0; self.nrows];
        for i in 0..self.nrows {
            let start = self.row_ptrs[i];
            let end = self.row_ptrs[i + 1];
            let mut sum = 0.0;
            for idx in start..end {
                sum += self.values[idx] * x[self.col_indices[idx]];
            }
            y[i] = sum;
        }
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csr_matvec_matches_dense() {
        let triplets = vec![(0, 0, 2.0), (0, 1, -1.0), (1, 0, -1.0), (1, 1, 2.0)];
        let csr = CsrMatrix::from_triplets(2, 2, triplets);
        let x = [1.0, 2.0];
        let y = csr.matvec(&x);
        assert_eq!(y, vec![0.0, 3.0]);
    }
}
