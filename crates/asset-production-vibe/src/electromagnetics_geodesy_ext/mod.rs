//! Wave 9 extensions in electromagnetics and geodesy:
//! - `hypermedia.electromagnetics.raymarch_vox0_dda`: 3D Amanatides-Woo DDA voxel traversal
//!   over Vox0 / .emf lattices with ADM 3+1 relativistic gravitational lensing deflection.
//! - `hypermedia.electromagnetics.project_metamer_nullspace`: 41-band spectral metamer fiber
//!   projection, decomposing into fundamental tristimulus component and 38D metameric black null-space.
//! - `hypermedia.geodesy.advect_m64_grid`: Divergence-free Jacobi Poisson pressure projection,
//!   second-order MacCormack advection, and Eikonal fast-sweeping redistancing over .m64 geodetic grids.
//! - `hypermedia.geodesy.couple_continuum_xpbd`: Terramechanics coupling (Bekker pressure-sinkage,
//!   Mohr-Coulomb shear stress) and Archimedes + Gerstner hydrodynamic wave buoyancy.

use crate::*;

mod m64_continuum;
mod vox0_metamer;

impl AssetHost<'_> {
    /// Categorical operations dispatch for electromagnetics and geodesy extensions.
    pub(crate) fn electromagnetics_geodesy_ext_op(
        &mut self,
        id: &str,
        a: &Args,
    ) -> Result<Value, String> {
        match id {
            "hypermedia.electromagnetics.raymarch_vox0_dda"
            | "electromagnetics.raymarch_vox0_dda"
            | "synth.electromagnetics.raymarch_vox0_dda"
            | "raymarch_vox0_dda" => self.raymarch_vox0_dda(a),

            "hypermedia.electromagnetics.project_metamer_nullspace"
            | "electromagnetics.project_metamer_nullspace"
            | "synth.electromagnetics.project_metamer_nullspace"
            | "project_metamer_nullspace" => self.project_metamer_nullspace(a),

            "hypermedia.geodesy.advect_m64_grid"
            | "geodesy.advect_m64_grid"
            | "synth.geodesy.advect_m64_grid"
            | "advect_m64_grid" => self.advect_m64_grid(a),

            "hypermedia.geodesy.couple_continuum_xpbd"
            | "geodesy.couple_continuum_xpbd"
            | "synth.geodesy.couple_continuum_xpbd"
            | "couple_continuum_xpbd" => self.couple_continuum_xpbd(a),

            _ => Err(format!(
                "Unknown electromagnetics/geodesy extension operation: {id}"
            )),
        }
    }
}

/// Ray-AABB intersection test returning entry and exit parameters `(t_min, t_max)`.
pub(super) fn intersect_aabb(
    origin: [f64; 3],
    dir: [f64; 3],
    box_max: [f64; 3],
) -> Option<(f64, f64)> {
    let mut t_min = f64::NEG_INFINITY;
    let mut t_max = f64::INFINITY;

    for i in 0..3 {
        if dir[i].abs() < 1e-9 {
            if origin[i] < 0.0 || origin[i] > box_max[i] {
                return None;
            }
        } else {
            let inv_d = 1.0 / dir[i];
            let mut t1 = -origin[i] * inv_d;
            let mut t2 = (box_max[i] - origin[i]) * inv_d;
            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            t_min = t_min.max(t1);
            t_max = t_max.min(t2);
            if t_min > t_max {
                return None;
            }
        }
    }

    if t_max < 0.0 {
        return None;
    }
    Some((t_min.max(0.0), t_max))
}

/// CIE 1931 / Bruton piecewise spectral-to-sRGB mapping for wavelength in nanometers.
pub(super) fn spectral_wavelength_to_rgb(wavelength_nm: f64) -> [f64; 3] {
    if !wavelength_nm.is_finite() || wavelength_nm <= 0.0 {
        return [0.0, 0.0, 0.0];
    }
    if wavelength_nm < 380.0 {
        return [0.95, 0.05, 0.85]; // UV false-color magenta
    }
    if wavelength_nm > 780.0 {
        return [0.05, 0.85, 0.95]; // IR false-color cyan
    }

    let (r, g, b) = if wavelength_nm < 440.0 {
        let r = -(wavelength_nm - 440.0) / (440.0 - 380.0);
        (r, 0.0, 1.0)
    } else if wavelength_nm < 490.0 {
        let g = (wavelength_nm - 440.0) / (490.0 - 440.0);
        (0.0, g, 1.0)
    } else if wavelength_nm < 510.0 {
        let b = -(wavelength_nm - 510.0) / (510.0 - 490.0);
        (0.0, 1.0, b)
    } else if wavelength_nm < 580.0 {
        let r = (wavelength_nm - 510.0) / (580.0 - 510.0);
        (r, 1.0, 0.0)
    } else if wavelength_nm < 645.0 {
        let g = -(wavelength_nm - 645.0) / (645.0 - 580.0);
        (1.0, g, 0.0)
    } else {
        (1.0, 0.0, 0.0)
    };

    let factor = if wavelength_nm < 420.0 {
        0.3 + 0.7 * (wavelength_nm - 380.0) / (420.0 - 380.0)
    } else if wavelength_nm > 700.0 {
        0.3 + 0.7 * (780.0 - wavelength_nm) / (780.0 - 700.0)
    } else {
        1.0
    };

    [r * factor, g * factor, b * factor]
}

/// Bilinear interpolation sampler over a 2D periodic field.
pub(super) fn sample_bilinear_2d(field: &[f64], n: usize, px: f64, py: f64) -> f64 {
    let fn_dim = n as f64;
    let x_wrapped = ((px % fn_dim) + fn_dim) % fn_dim;
    let y_wrapped = ((py % fn_dim) + fn_dim) % fn_dim;
    let x0 = x_wrapped.floor() as usize;
    let y0 = y_wrapped.floor() as usize;
    let x1 = (x0 + 1) % n;
    let y1 = (y0 + 1) % n;
    let fx = x_wrapped - x_wrapped.floor();
    let fy = y_wrapped - y_wrapped.floor();

    let c00 = field[y0 * n + x0];
    let c10 = field[y0 * n + x1];
    let c01 = field[y1 * n + x0];
    let c11 = field[y1 * n + x1];

    let c0 = c00 * (1.0 - fx) + c10 * fx;
    let c1 = c01 * (1.0 - fx) + c11 * fx;
    c0 * (1.0 - fy) + c1 * fy
}

/// Determines the local min and max over a 2x2 neighborhood for BPS advection limiting.
pub(super) fn neighborhood_min_max(field: &[f64], n: usize, px: f64, py: f64) -> (f64, f64) {
    let fn_dim = n as f64;
    let x_wrapped = ((px % fn_dim) + fn_dim) % fn_dim;
    let y_wrapped = ((py % fn_dim) + fn_dim) % fn_dim;
    let x0 = x_wrapped.floor() as usize;
    let y0 = y_wrapped.floor() as usize;
    let x1 = (x0 + 1) % n;
    let y1 = (y0 + 1) % n;

    let c00 = field[y0 * n + x0];
    let c10 = field[y0 * n + x1];
    let c01 = field[y1 * n + x0];
    let c11 = field[y1 * n + x1];

    let min_val = c00.min(c10).min(c01).min(c11);
    let max_val = c00.max(c10).max(c01).max(c11);
    (min_val, max_val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::atomic::AtomicBool;

    fn create_test_host<'a>(
        inputs: &'a [Resource],
        cancel: &'a AtomicBool,
        limits: &'a ToolLimits,
    ) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs,
            limits,
            cancel,
            work: 0,
        }
    }

    #[test]
    fn test_raymarch_vox0_dda_procedural() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(&[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("width".into(), Value::U64(16));
        args.insert("height".into(), Value::U64(16));
        args.insert("deflection_mass".into(), Value::F64(0.06));
        args.insert("step_limit".into(), Value::U64(32));

        let val = host
            .electromagnetics_geodesy_ext_op("hypermedia.electromagnetics.raymarch_vox0_dda", &args)
            .expect("raymarch_vox0_dda op must succeed");

        let rec = record(&val).expect("result must be record");
        assert!(rec.contains_key("image"));
        assert!(rec.contains_key("dda_steps_total"));
        assert!(rec.contains_key("brick_hits"));
        assert_eq!(rec.get("width"), Some(&Value::U64(16)));
        assert_eq!(rec.get("height"), Some(&Value::U64(16)));

        match rec.get("dda_steps_total") {
            Some(Value::U64(steps)) => assert!(*steps > 0, "DDA steps must be positive"),
            _ => panic!("Expected dda_steps_total as U64"),
        }
        match rec.get("brick_hits") {
            Some(Value::U64(hits)) => assert!(*hits > 0, "Brick hits must be positive"),
            _ => panic!("Expected brick_hits as U64"),
        }
    }

    #[test]
    fn test_project_metamer_nullspace_invariance() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(&[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("nullspace_scale".into(), Value::F64(0.75));

        let val = host
            .electromagnetics_geodesy_ext_op(
                "hypermedia.electromagnetics.project_metamer_nullspace",
                &args,
            )
            .expect("project_metamer_nullspace op must succeed");

        let rec = record(&val).expect("result must be record");
        let drift = match rec.get("tristimulus_drift_l2") {
            Some(Value::F64(d)) => *d,
            _ => panic!("Expected tristimulus_drift_l2 as F64"),
        };
        assert!(
            drift < 1e-4,
            "Tristimulus drift under nullspace scale must be negligible: {drift}"
        );

        match rec.get("fundamental_energy") {
            Some(Value::F64(e)) => assert!(*e > 0.0, "Fundamental energy must be positive"),
            _ => panic!("Expected fundamental_energy as F64"),
        }
        match rec.get("spd_samples_41") {
            Some(Value::List(l)) => assert_eq!(l.len(), 41, "Must contain 41 spectral samples"),
            _ => panic!("Expected spd_samples_41 as List"),
        }
    }

    #[test]
    fn test_advect_m64_grid_divergence_reduction() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(&[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("grid_side".into(), Value::U64(16));
        args.insert("steps".into(), Value::U64(4));
        args.insert("dt".into(), Value::F64(0.08));
        args.insert("poisson_iterations".into(), Value::U64(24));

        let val = host
            .electromagnetics_geodesy_ext_op("hypermedia.geodesy.advect_m64_grid", &args)
            .expect("advect_m64_grid op must succeed");

        let rec = record(&val).expect("result must be record");
        let div_before = match rec.get("max_divergence_before") {
            Some(Value::F64(d)) => *d,
            _ => panic!("Expected max_divergence_before as F64"),
        };
        let div_after = match rec.get("max_divergence_after") {
            Some(Value::F64(d)) => *d,
            _ => panic!("Expected max_divergence_after as F64"),
        };

        assert!(
            div_after < div_before,
            "Poisson projection must reduce divergence: before {div_before}, after {div_after}"
        );
        assert!(rec.contains_key("preview_rgba"));
        assert_eq!(rec.get("grid_side"), Some(&Value::U64(16)));
    }

    #[test]
    fn test_couple_continuum_xpbd_physics() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(&[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("wheel_load_n".into(), Value::F64(4500.0));
        args.insert("contact_width_m".into(), Value::F64(0.35));
        args.insert("contact_length_m".into(), Value::F64(0.45));
        args.insert("soil_kc".into(), Value::F64(15000.0));
        args.insert("soil_kphi".into(), Value::F64(850000.0));
        args.insert("soil_n".into(), Value::F64(0.8));
        args.insert("soil_cohesion_pa".into(), Value::F64(12000.0));
        args.insert("soil_friction_deg".into(), Value::F64(31.0));
        args.insert("submerged_volume_m3".into(), Value::F64(0.65));
        args.insert("wave_amplitude_m".into(), Value::F64(0.4));

        let val = host
            .electromagnetics_geodesy_ext_op("hypermedia.geodesy.couple_continuum_xpbd", &args)
            .expect("couple_continuum_xpbd op must succeed");

        let rec = record(&val).expect("result must be record");
        let sinkage = match rec.get("sinkage_m") {
            Some(Value::F64(z)) => *z,
            _ => panic!("Expected sinkage_m as F64"),
        };
        let shear = match rec.get("max_shear_pa") {
            Some(Value::F64(s)) => *s,
            _ => panic!("Expected max_shear_pa as F64"),
        };
        let dp = match rec.get("drawbar_pull_n") {
            Some(Value::F64(p)) => *p,
            _ => panic!("Expected drawbar_pull_n as F64"),
        };
        let buoyancy = match rec.get("buoyancy_force_n") {
            Some(Value::F64(b)) => *b,
            _ => panic!("Expected buoyancy_force_n as F64"),
        };

        assert!(
            sinkage > 0.005 && sinkage < 0.15,
            "Sinkage must be realistic: {sinkage} m"
        );
        assert!(shear > 10000.0, "Shear stress must be positive: {shear} Pa");
        assert!(dp > 1000.0, "Drawbar pull must be positive: {dp} N");
        assert!(
            buoyancy > 5000.0,
            "Buoyancy force must be positive: {buoyancy} N"
        );
    }

    #[test]
    fn test_dispatch_aliases_and_errors() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(&[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("grid_side".into(), Value::U64(8));
        args.insert("steps".into(), Value::U64(1));

        assert!(host
            .electromagnetics_geodesy_ext_op("synth.geodesy.advect_m64_grid", &args)
            .is_ok());
        assert!(host
            .electromagnetics_geodesy_ext_op("geodesy.advect_m64_grid", &args)
            .is_ok());
        assert!(host
            .electromagnetics_geodesy_ext_op("unknown_operation", &args)
            .is_err());
    }
}
