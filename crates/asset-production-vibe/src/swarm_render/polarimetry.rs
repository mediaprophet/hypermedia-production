//! Plane-parallel lossless isotropic thin-film polarimetry.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{f64::consts::TAU, sync::Arc};

use qualia_core_db::q42::emf_operators::{
    admit_stokes_vector, mueller_apply, rotate_mueller_matrix_passive,
};

use crate::{
    artifact::FieldLattice, fields, list, num, number, required, vector, Args, AssetHost,
    TypedArtifact, Value,
};

impl AssetHost<'_> {
    pub(crate) fn solve_mueller_polarimetry_and_thin_film(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "wavelengths_nm",
                "film_thickness_nm",
                "film_ior",
                "ambient_ior",
                "substrate_ior",
                "angle_deg",
                "basis_rotation_deg",
                "incident_stokes",
            ],
        )?;
        let wavelengths = list(required(args, "wavelengths_nm")?)?;
        if !(1..=256).contains(&wavelengths.len()) {
            return Err("Thin-film polarimetry requires 1..=256 wavelengths".into());
        }
        let wavelengths = wavelengths.iter().map(num).collect::<Result<Vec<_>, _>>()?;
        if wavelengths
            .iter()
            .any(|&w| !(200.0..=20_000.0).contains(&w))
            || wavelengths.windows(2).any(|pair| pair[1] <= pair[0])
        {
            return Err("Wavelengths must increase strictly within 200..=20000 nm".into());
        }
        let thickness = num(required(args, "film_thickness_nm")?)?;
        let ambient = number(args, "ambient_ior", 1.0)?;
        let film = num(required(args, "film_ior")?)?;
        let substrate = num(required(args, "substrate_ior")?)?;
        let angle = number(args, "angle_deg", 0.0)?;
        let basis = number(args, "basis_rotation_deg", 0.0)?;
        let stokes = args
            .get("incident_stokes")
            .map(vector::<4>)
            .transpose()?
            .unwrap_or([1.0, 0.0, 0.0, 0.0]);
        if !(1.0..=1.0e6).contains(&thickness)
            || [ambient, film, substrate]
                .iter()
                .any(|n| !(1.0..=4.0).contains(n))
            || !(0.0..=89.9).contains(&angle)
            || !(-360.0..=360.0).contains(&basis)
            || stokes[0] > 1.0e20
        {
            return Err("Thin-film thickness, index, angle or incident intensity is outside the supported range".into());
        }
        admit_stokes_vector(stokes).map_err(|e| format!("Invalid incident polarization: {e:?}"))?;
        let sine = ambient * angle.to_radians().sin();
        let indices = [ambient, film, substrate];
        let cos2 = indices.map(|n| 1.0 - (sine / n).powi(2));
        if cos2.iter().any(|&x| x <= 0.0) {
            return Err(
                "This isotropic film branch requires propagating waves in all media".into(),
            );
        }
        let c = cos2.map(f64::sqrt);
        let s = std::array::from_fn(|i| indices[i] * c[i]);
        let p = std::array::from_fn(|i| indices[i] / c[i]);
        self.charge(wavelengths.len() * 160, wavelengths.len() * 128)?;
        let mut matrices = Vec::with_capacity(16 * wavelengths.len());
        let mut reflected = Vec::with_capacity(4 * wavelengths.len());
        for &wave in &wavelengths {
            let phase = 2.0 * TAU * film * c[1] * thickness / wave;
            let rs = thin_film_reflection(s, phase);
            let rp = thin_film_reflection(p, phase);
            let ss = rs.0 * rs.0 + rs.1 * rs.1;
            let pp = rp.0 * rp.0 + rp.1 * rp.1;
            if ss > 1.0 + 1e-12 || pp > 1.0 + 1e-12 {
                return Err("Film reflection violates passive power conservation".into());
            }
            let mean = 0.5 * (ss + pp);
            let del = 0.5 * (ss - pp);
            let re = rs.0 * rp.0 + rs.1 * rp.1;
            let im = rs.1 * rp.0 - rs.0 * rp.1;
            let matrix = rotate_mueller_matrix_passive(
                &[
                    [mean, del, 0.0, 0.0],
                    [del, mean, 0.0, 0.0],
                    [0.0, 0.0, re, -im],
                    [0.0, 0.0, im, re],
                ],
                basis.to_radians(),
            );
            let out = mueller_apply(&matrix, stokes).map_err(|e| {
                format!("Film changed the Stokes state outside the physical cone: {e:?}")
            })?;
            if out[0] > stokes[0] * (1.0 + 1e-12) {
                return Err("Reflected Stokes intensity exceeds the passive input".into());
            }
            matrices.extend(matrix.into_iter().flatten().map(|x| x as f32));
            reflected.extend(out.map(|x| x as f32));
        }
        let mut evidence = matrices.clone();
        evidence.extend_from_slice(&reflected);
        evidence.extend(wavelengths.iter().map(|x| *x as f32));
        let receipt = super::receipt(
            self,
            "isotropic-single-film-mueller-reflectance",
            &evidence,
            &[
                ("film_thickness_nm", thickness),
                ("film_ior", film),
                ("ambient_ior", ambient),
                ("substrate_ior", substrate),
                ("angle_deg", angle),
                ("basis_rotation_deg", basis),
            ],
        )?;
        let matrix = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![wavelengths.len() as u32, 4, 4],
                samples: matrices,
            },
        )))?;
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![wavelengths.len() as u32, 4],
                samples: reflected,
            },
        )))?;
        Ok(Value::Record(Args::from([
            ("mueller".into(), matrix),
            ("reflected_stokes".into(), field),
            (
                "wavelengths_nm".into(),
                Value::List(
                    wavelengths
                        .into_iter()
                        .map(Value::F64)
                        .collect::<Vec<_>>()
                        .into(),
                ),
            ),
            ("receipt".into(), receipt),
        ])))
    }
}

/// Airy sum of a front-interface amplitude and all internally reflected waves.
fn thin_film_reflection([n0, n1, n2]: [f64; 3], phase: f64) -> (f64, f64) {
    let a = (n0 - n1) / (n0 + n1);
    let b = (n1 - n2) / (n1 + n2);
    let (s, c) = phase.sin_cos();
    let (x, y) = (a + b * c, b * s);
    let (u, v) = (1.0 + a * b * c, a * b * s);
    let norm = u * u + v * v;
    ((x * u + y * v) / norm, (y * u - x * v) / norm)
}
