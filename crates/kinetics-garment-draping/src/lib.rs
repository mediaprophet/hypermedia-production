//! kinetics-garment-draping: UI-agnostic categorical garment mechanics and draping.
//!
//! Provides second-order forward automatic differentiation (Jet<N>), nonlinear
//! orthotropic membrane & flat-rest hinge bending energies, calibrated textile libraries,
//! CPU f64 Vertex Block Descent (VBD), and implicit Euler regularized Newton-PCG solver.
//!
//! Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
//! and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod energy;
pub mod fabrics;
pub mod math;
pub mod newton;
pub mod pattern;
pub mod vbd;

pub use energy::{Element, FabricMaterial, Hinge, Triangle, bending_element, membrane_element};
pub use fabrics::{FabricPreset, standard_fabrics};
pub use math::{DMat2, DVec2, DVec3, Jet, Scalar};
pub use newton::{NewtonMetrics, NewtonSettings, newton_step};
pub use pattern::{AvatarLandmark, BodyMeasurements, SeamCoupling, eval_cubic_bezier, flatten_cubic_bezier, polyline_arclength_point};
pub use vbd::{ClothMesh, VbdSettings, vbd_step};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jet_forward_ad() {
        // f(x, y) = x^2 * y
        // df/dx = 2xy, df/dy = x^2
        // d2f/dx2 = 2y, d2f/dy2 = 0, d2f/dxdy = 2x
        let x0 = 3.0;
        let y0 = 4.0;
        let x = Jet::<2>::variable(x0, 0);
        let y = Jet::<2>::variable(y0, 1);

        let f = x * x * y;
        assert_eq!(f.v, 36.0); // 3^2 * 4 = 36
        assert_eq!(f.g[0], 24.0); // 2 * 3 * 4 = 24
        assert_eq!(f.g[1], 9.0); // 3^2 = 9
        assert_eq!(f.h[0][0], 8.0); // 2 * 4 = 8
        assert_eq!(f.h[1][1], 0.0);
        assert_eq!(f.h[0][1], 6.0); // 2 * 3 = 6
    }

    #[test]
    fn test_membrane_and_bending_energy() {
        let mat = standard_fabrics()[0].to_material();
        let tri = Triangle {
            vertices: [0, 1, 2],
            rest: [DVec2::new(0.0, 0.0), DVec2::new(1.0, 0.0), DVec2::new(0.0, 1.0)],
            inv_dm: DMat2::from_cols(DVec2::new(1.0, 0.0), DVec2::new(0.0, 1.0)),
            area: 0.5,
            material: 0,
        };

        // At rest position
        let x_rest = vec![
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
        ];

        let elem = membrane_element(&x_rest, &tri, &mat);
        assert!(elem.energy.abs() < 1e-6);

        // Stretched position
        let x_stretched = vec![
            DVec3::new(0.0, 0.0, 0.0),
            DVec3::new(1.1, 0.0, 0.0),
            DVec3::new(0.0, 1.0, 0.0),
        ];
        let elem_str = membrane_element(&x_stretched, &tri, &mat);
        assert!(elem_str.energy > 0.0);
        assert_eq!(elem_str.gradient.len(), 9);
        assert_eq!(elem_str.hessian.len(), 81);
    }

    #[test]
    fn test_vbd_step_simulation() {
        let mat = standard_fabrics()[0].to_material();
        let tri1 = Triangle {
            vertices: [0, 1, 2],
            rest: [DVec2::new(0.0, 0.0), DVec2::new(1.0, 0.0), DVec2::new(0.0, 1.0)],
            inv_dm: DMat2::from_cols(DVec2::new(1.0, 0.0), DVec2::new(0.0, 1.0)),
            area: 0.5,
            material: 0,
        };

        let positions = vec![
            DVec3::new(0.0, 1.0, 0.0),
            DVec3::new(1.0, 1.0, 0.0),
            DVec3::new(0.0, 1.0, 1.0),
        ];

        let mut cloth = ClothMesh::new(positions, vec![tri1], vec![], mat);
        // Pin vertex 0 and 1
        cloth.pins[0] = Some(DVec3::new(0.0, 1.0, 0.0));
        cloth.pins[1] = Some(DVec3::new(1.0, 1.0, 0.0));

        let settings = VbdSettings::default();
        vbd_step(&mut cloth, &settings);

        // Vertex 2 (unpinned) should drop due to gravity
        assert!(cloth.positions[2].y < 1.0);
        assert_eq!(cloth.positions[0].y, 1.0);
    }

    #[test]
    fn test_newton_pcg_solver_step() {
        let mat = standard_fabrics()[0].to_material();
        let tri1 = Triangle {
            vertices: [0, 1, 2],
            rest: [DVec2::new(0.0, 0.0), DVec2::new(1.0, 0.0), DVec2::new(0.0, 1.0)],
            inv_dm: DMat2::from_cols(DVec2::new(1.0, 0.0), DVec2::new(0.0, 1.0)),
            area: 0.5,
            material: 0,
        };

        let positions = vec![
            DVec3::new(0.0, 1.0, 0.0),
            DVec3::new(1.0, 1.0, 0.0),
            DVec3::new(0.0, 1.0, 1.0),
        ];

        let mut cloth = ClothMesh::new(positions, vec![tri1], vec![], mat);
        cloth.pins[0] = Some(DVec3::new(0.0, 1.0, 0.0));
        cloth.pins[1] = Some(DVec3::new(1.0, 1.0, 0.0));

        let settings = NewtonSettings::default();
        let metrics = newton_step(&mut cloth, &settings);

        assert!(metrics.newton_iters > 0);
        assert!(cloth.positions[2].y < 1.0);
    }

    #[test]
    fn test_bezier_curve_and_arclength() {
        let p = [
            DVec2::new(0.0, 0.0),
            DVec2::new(0.0, 1.0),
            DVec2::new(1.0, 1.0),
            DVec2::new(1.0, 0.0),
        ];

        let mid = eval_cubic_bezier(p, 0.5);
        assert!((mid.y - 0.75).abs() < 1e-4);

        let flattened = flatten_cubic_bezier(p, 0.01);
        assert!(flattened.len() >= 4);

        let pt_at_quarter = polyline_arclength_point(&flattened, 0.5);
        assert!(pt_at_quarter.y > 0.5);
    }
}
