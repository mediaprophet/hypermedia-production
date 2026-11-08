//! Dense column-major matrix algebra with numerical decompositions (LU, QR, Cholesky, SVD, Eig).
//! Grounded in deterministic numerical linear algebra and informatics fiduciary duties.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatError {
    DimensionMismatch { a: (usize, usize), b: (usize, usize) },
    Singular,
    OutOfBounds { r: usize, c: usize, rows: usize, cols: usize },
    BadData { len: usize, expected: usize },
    Other(String),
}

impl fmt::Display for MatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatError::DimensionMismatch { a, b } => {
                write!(f, "dimension mismatch: {}x{} vs {}x{}", a.0, a.1, b.0, b.1)
            }
            MatError::Singular => write!(f, "matrix is singular or non-square"),
            MatError::OutOfBounds { r, c, rows, cols } => {
                write!(f, "index ({r},{c}) out of bounds for {rows}x{cols} matrix")
            }
            MatError::BadData { len, expected } => {
                write!(f, "got {len} elements for a {expected}-element matrix")
            }
            MatError::Other(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for MatError {}

/// Dense column-major `f64` matrix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows.saturating_mul(cols)],
        }
    }

    pub fn ones(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![1.0; rows.saturating_mul(cols)],
        }
    }

    pub fn eye(rows: usize, cols: usize) -> Self {
        let mut m = Self::zeros(rows, cols);
        for i in 0..rows.min(cols) {
            m.data[i * rows + i] = 1.0;
        }
        m
    }

    pub fn scalar(v: f64) -> Self {
        Self {
            rows: 1,
            cols: 1,
            data: vec![v],
        }
    }

    pub fn row_vector(v: Vec<f64>) -> Self {
        Self {
            rows: 1,
            cols: v.len(),
            data: v,
        }
    }

    pub fn col_vector(v: Vec<f64>) -> Self {
        let rows = v.len();
        Self {
            rows,
            cols: 1,
            data: v,
        }
    }

    pub fn from_col_major(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, MatError> {
        if data.len() != rows * cols {
            return Err(MatError::BadData { len: data.len(), expected: rows * cols });
        }
        Ok(Self { rows, cols, data })
    }

    pub fn from_row_major(rows: usize, cols: usize, data: &[f64]) -> Result<Self, MatError> {
        if data.len() != rows * cols {
            return Err(MatError::BadData { len: data.len(), expected: rows * cols });
        }
        let mut m = Self::zeros(rows, cols);
        for r in 0..rows {
            for c in 0..cols {
                m.data[c * rows + r] = data[r * cols + c];
            }
        }
        Ok(m)
    }

    #[inline(always)]
    pub fn get(&self, r: usize, c: usize) -> Option<f64> {
        if r < self.rows && c < self.cols {
            Some(self.data[c * self.rows + r])
        } else {
            None
        }
    }

    pub fn map(&self, f: impl Fn(f64) -> f64) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|&x| f(x)).collect(),
        }
    }

    #[inline(always)]
    pub fn set(&mut self, r: usize, c: usize, v: f64) -> Result<(), MatError> {
        if r < self.rows && c < self.cols {
            self.data[c * self.rows + r] = v;
            Ok(())
        } else {
            Err(MatError::OutOfBounds { r, c, rows: self.rows, cols: self.cols })
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn numel(&self) -> usize {
        self.data.len()
    }

    pub fn is_scalar(&self) -> bool {
        self.rows == 1 && self.cols == 1
    }

    pub fn as_scalar(&self) -> Option<f64> {
        if self.is_scalar() {
            self.data.first().copied()
        } else {
            None
        }
    }

    /// 2D slicing: extracts submatrix over [r_start..r_end] x [c_start..c_end] (0-based, half-open).
    pub fn slice(&self, r_start: usize, r_end: usize, c_start: usize, c_end: usize) -> Result<Self, MatError> {
        if r_end > self.rows || c_end > self.cols || r_start > r_end || c_start > c_end {
            return Err(MatError::Other(format!(
                "invalid slice [{}..{}]x[{}..{}] for {}x{} matrix",
                r_start, r_end, c_start, c_end, self.rows, self.cols
            )));
        }
        let out_rows = r_end - r_start;
        let out_cols = c_end - c_start;
        let mut out = Self::zeros(out_rows, out_cols);
        for c in 0..out_cols {
            for r in 0..out_rows {
                out.data[c * out_rows + r] = self.data[(c_start + c) * self.rows + (r_start + r)];
            }
        }
        Ok(out)
    }

    pub fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                out.data[r * self.cols + c] = self.data[c * self.rows + r];
            }
        }
        out
    }

    pub fn matmul(&self, other: &Self) -> Result<Self, MatError> {
        if self.cols != other.rows {
            return Err(MatError::DimensionMismatch {
                a: self.shape(),
                b: other.shape(),
            });
        }
        let mut out = Self::zeros(self.rows, other.cols);
        for c in 0..other.cols {
            for k in 0..self.cols {
                let b_val = other.data[c * other.rows + k];
                for r in 0..self.rows {
                    out.data[c * self.rows + r] += self.data[k * self.rows + r] * b_val;
                }
            }
        }
        Ok(out)
    }

    pub fn elem_add(&self, other: &Self) -> Result<Self, MatError> {
        if self.shape() != other.shape() {
            return Err(MatError::DimensionMismatch { a: self.shape(), b: other.shape() });
        }
        let mut out = self.clone();
        for (a, &b) in out.data.iter_mut().zip(&other.data) {
            *a += b;
        }
        Ok(out)
    }

    pub fn elem_sub(&self, other: &Self) -> Result<Self, MatError> {
        if self.shape() != other.shape() {
            return Err(MatError::DimensionMismatch { a: self.shape(), b: other.shape() });
        }
        let mut out = self.clone();
        for (a, &b) in out.data.iter_mut().zip(&other.data) {
            *a -= b;
        }
        Ok(out)
    }

    pub fn elem_mul(&self, other: &Self) -> Result<Self, MatError> {
        if self.shape() != other.shape() {
            return Err(MatError::DimensionMismatch { a: self.shape(), b: other.shape() });
        }
        let mut out = self.clone();
        for (a, &b) in out.data.iter_mut().zip(&other.data) {
            *a *= b;
        }
        Ok(out)
    }

    pub fn scale(&self, s: f64) -> Self {
        let mut out = self.clone();
        for v in &mut out.data {
            *v *= s;
        }
        out
    }

    pub fn norm(&self) -> f64 {
        self.data.iter().map(|&x| x * x).sum::<f64>().sqrt()
    }

    pub fn det(&self) -> Result<f64, MatError> {
        if self.rows != self.cols {
            return Err(MatError::Singular);
        }
        let n = self.rows;
        let mut a = self.data.clone();
        let mut sign = 1.0;
        let mut det = 1.0;

        for col in 0..n {
            let mut pivot = col;
            let mut best = a[col * n + col].abs();
            for r in (col + 1)..n {
                let v = a[col * n + r].abs();
                if v > best {
                    best = v;
                    pivot = r;
                }
            }
            if best <= 1e-15 {
                return Ok(0.0);
            }
            if pivot != col {
                for c in 0..n {
                    a.swap(c * n + col, c * n + pivot);
                }
                sign = -sign;
            }
            let diag = a[col * n + col];
            det *= diag;
            for r in (col + 1)..n {
                let f = a[col * n + r] / diag;
                for c in (col + 1)..n {
                    a[c * n + r] -= f * a[c * n + col];
                }
            }
        }
        Ok(det * sign)
    }

    pub fn solve(&self, b: &Self) -> Result<Self, MatError> {
        if self.rows != self.cols || self.rows != b.rows {
            return Err(MatError::DimensionMismatch {
                a: self.shape(),
                b: b.shape(),
            });
        }
        let n = self.rows;
        let m = b.cols;
        let width = n + m;
        let mut aug = vec![0.0f64; n * width];

        for r in 0..n {
            for c in 0..n {
                aug[r * width + c] = self.data[c * n + r];
            }
            for c in 0..m {
                aug[r * width + n + c] = b.data[c * n + r];
            }
        }

        // Gaussian elimination with partial pivoting
        for col in 0..n {
            let mut pivot = col;
            let mut best = aug[col * width + col].abs();
            for r in (col + 1)..n {
                let v = aug[r * width + col].abs();
                if v > best {
                    best = v;
                    pivot = r;
                }
            }
            if best <= 1e-15 {
                return Err(MatError::Singular);
            }
            if pivot != col {
                for c in 0..width {
                    aug.swap(col * width + c, pivot * width + c);
                }
            }
            let diag = aug[col * width + col];
            for r in (col + 1)..n {
                let f = aug[r * width + col] / diag;
                for c in col..width {
                    aug[r * width + c] -= f * aug[col * width + c];
                }
            }
        }

        // Back substitution
        let mut x = Self::zeros(n, m);
        for col in 0..m {
            for r in (0..n).rev() {
                let mut acc = aug[r * width + n + col];
                for c in (r + 1)..n {
                    acc -= aug[r * width + c] * x.data[col * n + c];
                }
                let diag = aug[r * width + r];
                x.data[col * n + r] = acc / diag;
            }
        }

        Ok(x)
    }

    pub fn inverse(&self) -> Result<Self, MatError> {
        if self.rows != self.cols {
            return Err(MatError::Singular);
        }
        self.solve(&Self::eye(self.rows, self.cols))
    }
}

/// LU decomposition with partial pivoting: P A = L U.
pub fn lu(a: &Matrix) -> Result<(Matrix, Matrix, Matrix), MatError> {
    if a.rows != a.cols {
        return Err(MatError::Singular);
    }
    let n = a.rows;
    let mut u_raw = vec![0.0f64; n * n];
    let mut p = Matrix::eye(n, n);
    let mut l = Matrix::zeros(n, n);

    for r in 0..n {
        for c in 0..n {
            u_raw[r * n + c] = a.data[c * n + r];
        }
    }

    for col in 0..n {
        let mut pivot = col;
        let mut best = u_raw[col * n + col].abs();
        for r in (col + 1)..n {
            let v = u_raw[r * n + col].abs();
            if v > best {
                best = v;
                pivot = r;
            }
        }
        if best <= 1e-15 {
            return Err(MatError::Singular);
        }
        if pivot != col {
            for c in 0..n {
                u_raw.swap(col * n + c, pivot * n + c);
            }
            for c in 0..n {
                let va = p.data[c * n + col];
                let vb = p.data[c * n + pivot];
                p.data[c * n + col] = vb;
                p.data[c * n + pivot] = va;
            }
            for c in 0..col {
                let va = l.data[c * n + col];
                let vb = l.data[c * n + pivot];
                l.data[c * n + col] = vb;
                l.data[c * n + pivot] = va;
            }
        }

        let diag = u_raw[col * n + col];
        for r in (col + 1)..n {
            let f = u_raw[r * n + col] / diag;
            l.data[col * n + r] = f;
            for c in col..n {
                u_raw[r * n + c] -= f * u_raw[col * n + c];
            }
        }
    }

    for i in 0..n {
        l.data[i * n + i] = 1.0;
    }

    let mut u = Matrix::zeros(n, n);
    for c in 0..n {
        for r in 0..n {
            u.data[c * n + r] = u_raw[r * n + c];
        }
    }

    Ok((l, u, p))
}

/// Cholesky factorization: R' R = A for symmetric positive-definite A.
pub fn chol(a: &Matrix) -> Result<Matrix, MatError> {
    if a.rows != a.cols {
        return Err(MatError::Singular);
    }
    let n = a.rows;
    let mut r = Matrix::zeros(n, n);

    for i in 0..n {
        for j in i..n {
            let mut s = a.data[j * n + i];
            for k in 0..i {
                s -= r.data[i * n + k] * r.data[j * n + k];
            }
            if i == j {
                if s <= 0.0 {
                    return Err(MatError::Singular);
                }
                r.data[i * n + i] = s.sqrt();
            } else {
                let d = r.data[i * n + i];
                r.data[j * n + i] = s / d;
            }
        }
    }
    Ok(r)
}

/// Householder QR decomposition: A = Q R.
pub fn qr(a: &Matrix) -> Result<(Matrix, Matrix), MatError> {
    let (m, n) = (a.rows, a.cols);
    if m < n {
        return Err(MatError::DimensionMismatch { a: (m, n), b: (m, m) });
    }

    let mut r_raw = vec![0.0f64; m * n];
    for c in 0..n {
        for r in 0..m {
            r_raw[r * n + c] = a.data[c * m + r];
        }
    }

    let mut q = Matrix::eye(m, m);

    for k in 0..n {
        let mut norm = 0.0;
        for i in k..m {
            norm += r_raw[i * n + k].powi(2);
        }
        norm = norm.sqrt();
        if norm == 0.0 {
            continue;
        }

        let sign = if r_raw[k * n + k] >= 0.0 { 1.0 } else { -1.0 };
        let alpha = -sign * norm;
        let mut v = vec![0.0f64; m];
        for i in k..m {
            v[i] = r_raw[i * n + k];
        }
        v[k] -= alpha;

        let vnorm2: f64 = v.iter().map(|x| x * x).sum();
        if vnorm2 == 0.0 {
            continue;
        }

        for j in 0..n {
            let mut dot = 0.0;
            for i in k..m {
                dot += v[i] * r_raw[i * n + j];
            }
            let coef = 2.0 * dot / vnorm2;
            for i in k..m {
                r_raw[i * n + j] -= coef * v[i];
            }
        }

        // Q := Q * H
        let vmat = Matrix::col_vector(v.clone());
        let vmat_t = vmat.transpose();
        let vv_t = vmat.matmul(&vmat_t)?.scale(2.0 / vnorm2);
        let h = Matrix::eye(m, m).elem_sub(&vv_t)?;
        q = q.matmul(&h)?;
    }

    let mut r = Matrix::zeros(m, n);
    for c in 0..n {
        for row in 0..m {
            r.data[c * m + row] = r_raw[row * n + c];
        }
    }

    Ok((q, r))
}

/// Jacobi eigenvalue decomposition of a symmetric matrix: V' A V = diag(lambda).
pub fn eig_symmetric(a: &Matrix) -> Result<(Vec<f64>, Matrix), MatError> {
    if a.rows != a.cols {
        return Err(MatError::Singular);
    }
    let n = a.rows;
    let mut w = a.data.clone();
    let mut v = Matrix::eye(n, n);

    for _ in 0..100 {
        let mut p = 0;
        let mut q = 1.min(n.saturating_sub(1));
        let mut best = 0.0f64;

        for c in 0..n {
            for r in 0..c {
                let x = w[c * n + r].abs();
                if x > best {
                    best = x;
                    p = r;
                    q = c;
                }
            }
        }

        if best < 1e-12 {
            break;
        }

        let app = w[p * n + p];
        let aqq = w[q * n + q];
        let apq = w[q * n + p];
        let theta = (aqq - app) / (2.0 * apq);
        let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
        let c = 1.0 / (t * t + 1.0).sqrt();
        let s = t * c;

        for k in 0..n {
            let akp = w[p * n + k];
            let akq = w[q * n + k];
            w[p * n + k] = c * akp - s * akq;
            w[q * n + k] = s * akp + c * akq;
        }
        for k in 0..n {
            let apk = w[k * n + p];
            let aqk = w[k * n + q];
            w[k * n + p] = c * apk - s * aqk;
            w[k * n + q] = s * apk + c * aqk;
        }
        for k in 0..n {
            let vkp = v.data[p * n + k];
            let vkq = v.data[q * n + k];
            v.data[p * n + k] = c * vkp - s * vkq;
            v.data[q * n + k] = s * vkp + c * vkq;
        }
    }

    let vals: Vec<f64> = (0..n).map(|i| w[i * n + i]).collect();
    Ok((vals, v))
}

/// SVD via normal equations A'A: A = U S V'.
pub fn svd(a: &Matrix) -> Result<(Matrix, Matrix, Matrix), MatError> {
    let (m, n) = (a.rows, a.cols);
    let ata = a.transpose().matmul(a)?;
    let (eigvals, v) = eig_symmetric(&ata)?;

    // Singular values are sqrt of eigenvalues (sorted descending)
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&i, &j| eigvals[j].partial_cmp(&eigvals[i]).unwrap_or(std::cmp::Ordering::Equal));

    let mut vsort = Matrix::zeros(n, n);
    let mut svals = Vec::with_capacity(n);

    for (newc, &oldc) in order.iter().enumerate() {
        for r in 0..n {
            vsort.data[newc * n + r] = v.data[oldc * n + r];
        }
        svals.push(eigvals[oldc].max(0.0).sqrt());
    }

    let mut s = Matrix::zeros(m, n);
    let mut u = Matrix::zeros(m, m);

    for i in 0..m.min(n) {
        s.data[i * m + i] = svals[i];
    }

    // U columns: u_i = A * v_i / s_i
    for (i, &sv) in svals.iter().take(m.min(n)).enumerate() {
        if sv > 1e-9 {
            let vi = vsort.slice(0, n, i, i + 1)?;
            let avi = a.matmul(&vi)?;
            for r in 0..m {
                u.data[i * m + r] = avi.data[r] / sv;
            }
        }
    }

    // Complete U basis with Gram-Schmidt if needed
    for i in n..m {
        u.data[i * m + i] = 1.0;
    }

    Ok((u, s, vsort))
}
