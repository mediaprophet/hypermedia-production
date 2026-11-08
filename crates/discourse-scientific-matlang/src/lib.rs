//! discourse-scientific-matlang: UI-agnostic categorical scientific computing and mathematical discourse.
//!
//! Provides dense column-major matrix algebra with numerical decompositions (LU, QR, Cholesky, SVD, Eig),
//! pure-Rust M-code interpreter (lexer, parser, AST, workspace evaluator), and headless SVG vector plotting.
//!
//! Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
//! and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod interpreter;
pub mod matrix;
pub mod plot;

pub use interpreter::{eval_m_code, LangError, Parser, Value, Workspace};
pub use matrix::{chol, eig_symmetric, lu, qr, svd, MatError, Matrix};
pub use plot::{Colormap, Figure, Series, SeriesKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_decompositions() {
        // 2x2 matrix: [4, 1; 1, 3] (symmetric positive-definite)
        let a = Matrix::from_row_major(2, 2, &[4.0, 1.0, 1.0, 3.0]).unwrap();

        // 1. Determinant: 4*3 - 1*1 = 11
        let d = a.det().unwrap();
        assert!((d - 11.0).abs() < 1e-9);

        // 2. Inverse: A * A_inv = I
        let inv = a.inverse().unwrap();
        let eye = a.matmul(&inv).unwrap();
        assert!((eye.get(0, 0).unwrap() - 1.0).abs() < 1e-9);
        assert!(eye.get(0, 1).unwrap().abs() < 1e-9);
        assert!(eye.get(1, 0).unwrap().abs() < 1e-9);
        assert!((eye.get(1, 1).unwrap() - 1.0).abs() < 1e-9);

        // 3. Cholesky: R' * R = A
        let r = chol(&a).unwrap();
        let a_reconstructed = r.transpose().matmul(&r).unwrap();
        assert!((a_reconstructed.get(0, 0).unwrap() - 4.0).abs() < 1e-9);
        assert!((a_reconstructed.get(0, 1).unwrap() - 1.0).abs() < 1e-9);

        // 4. LU: P * A = L * U
        let (l, u, p) = lu(&a).unwrap();
        let lu_prod = l.matmul(&u).unwrap();
        let pa = p.matmul(&a).unwrap();
        assert!((lu_prod.get(0, 0).unwrap() - pa.get(0, 0).unwrap()).abs() < 1e-9);

        // 5. QR: Q * R = A, Q' * Q = I
        let (q, r_qr) = qr(&a).unwrap();
        let qr_prod = q.matmul(&r_qr).unwrap();
        assert!((qr_prod.get(0, 0).unwrap() - a.get(0, 0).unwrap()).abs() < 1e-9);
        let q_orth = q.transpose().matmul(&q).unwrap();
        assert!((q_orth.get(0, 0).unwrap() - 1.0).abs() < 1e-9);

        // 6. Eigenvalues of symmetric matrix
        let (eigvals, _v) = eig_symmetric(&a).unwrap();
        assert_eq!(eigvals.len(), 2);
        // Sum of eigenvalues = trace = 4 + 3 = 7
        let trace = eigvals[0] + eigvals[1];
        assert!((trace - 7.0).abs() < 1e-4);

        // 7. SVD: A = U * S * V'
        let (u_svd, s_svd, v_svd) = svd(&a).unwrap();
        let usv = u_svd.matmul(&s_svd).unwrap().matmul(&v_svd.transpose()).unwrap();
        assert!((usv.get(0, 0).unwrap() - a.get(0, 0).unwrap()).abs() < 1e-4);
    }

    #[test]
    fn test_matrix_slicing() {
        let m = Matrix::from_row_major(3, 3, &[
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ]).unwrap();

        let sub = m.slice(1, 3, 1, 3).unwrap();
        assert_eq!(sub.shape(), (2, 2));
        assert_eq!(sub.get(0, 0).unwrap(), 5.0);
        assert_eq!(sub.get(0, 1).unwrap(), 6.0);
        assert_eq!(sub.get(1, 0).unwrap(), 8.0);
        assert_eq!(sub.get(1, 1).unwrap(), 9.0);
    }

    #[test]
    fn test_m_code_evaluation() {
        let script = r#"
            x = [1, 2; 3, 4];
            y = x * 2;
            s = 0;
            for i = 1:5
                s = s + i;
            end
        "#;

        let ws = eval_m_code(script).unwrap();

        let s_val = ws.vars.get("s").unwrap();
        // 1 + 2 + 3 + 4 + 5 = 15
        assert_eq!(s_val.as_num().unwrap(), 15.0);

        let y_val = ws.vars.get("y").unwrap().as_matrix();
        assert_eq!(y_val.get(0, 0).unwrap(), 2.0);
        assert_eq!(y_val.get(1, 1).unwrap(), 8.0);
    }

    #[test]
    fn test_svg_figure_generation() {
        let mut fig = Figure::new();
        fig.title = Some("Scientific Test Figure".to_string());
        fig.xlabel = Some("Time (s)".to_string());
        fig.ylabel = Some("Amplitude".to_string());

        let x = vec![0.0, 1.0, 2.0, 3.0];
        let y = vec![0.0, 0.8, 0.3, 1.0];
        fig.add_series(Series::line(x, y, "#0066cc"));

        let z = vec![
            1.0, 2.0,
            3.0, 4.0,
        ];
        fig.add_series(Series::surface(z, 2, 2));

        let svg = fig.to_svg(600.0, 400.0);
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("Scientific Test Figure"));
        assert!(svg.contains("<polygon")); // 3D surface mesh
        assert!(svg.ends_with("</svg>\n"));
    }
}
