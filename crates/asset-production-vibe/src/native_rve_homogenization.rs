//! Scale transfer from labelled representative volume elements.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::artifact::FieldLattice;
use crate::*;
use qualia_core_db::specialized_libs::physics_simulation::{
    homogenization_acoust::biot_acoustic_velocities, homogenization_hydro::kozeny_carman_permeability_m2,
    homogenization_mech::compute_vrh_elasticity,
    homogenization_optic::{kubelka_munk_mixture, kubelka_munk_reflectance},
};

impl AssetHost<'_> {
    pub(crate) fn homogenize_to_continuum(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["microstructure", "regime", "phases", "grain_diameter_m", "void_phase"])?;
        let volume = required(args, "microstructure")?;
        let regime = text(required(args, "regime")?)?;
        let phases = list(required(args, "phases")?)?;
        let property_names: &[&str] = match regime {
            "mechanical" => &["bulk_pa", "shear_pa"],
            "acoustic" => &["bulk_pa", "shear_pa", "density_kg_m3", "relative_permittivity"],
            "optical" => &["absorption_per_m", "scattering_per_m"],
            "hydraulic" => &[],
            _ => return Err("Unknown RVE homogenization regime".into()),
        };
        if !(1..=64).contains(&phases.len()) {
            return Err("RVE requires 1 to 64 phase descriptions".into());
        }
        let mut data = BTreeMap::new();
        for phase in phases {
            let phase = record(phase)?;
            let mut keys = vec!["id"];
            keys.extend_from_slice(property_names);
            fields(phase, &keys)?;
            let id = integer(required(phase, "id")?, "phase id")?;
            if id > 65535 || data.contains_key(&(id as u16)) {
                return Err("RVE phase IDs must be unique u16 labels".into());
            }
            let mut properties = Vec::with_capacity(property_names.len());
            for &property in property_names {
                let value = num(required(phase, property)?)?;
                let minimum = match property {
                    "bulk_pa" | "shear_pa" | "density_kg_m3" => 1e-12,
                    "relative_permittivity" => 1.0,
                    _ => 0.0,
                };
                if !(minimum..=1e16).contains(&value) {
                    return Err(format!("Invalid constituent property {property}"));
                }
                properties.push(value);
            }
            data.insert(id as u16, (properties, 0usize));
        }
        let (axes, samples) = rve(self, volume)?;
        let extent = axes.to_vec();
        let sample_count = samples.len();
        self.charge(sample_count * 16, data.len() * 1024)?;
        let (_, samples) = rve(self, volume)?;
        for &sample in samples {
            if !(0.0..=65535.0).contains(&sample) || sample.fract() != 0.0 {
                return Err("RVE voxels must carry integral phase labels".into());
            }
            data.get_mut(&(sample as u16)).ok_or("RVE references an undeclared phase")?.1 += 1;
        }
        let fractions = data.values().map(|(_, count)| *count as f64 / sample_count as f64).collect::<Vec<_>>();
        let lanes = |lane| data.values().map(|(properties, _)| properties[lane]).collect::<Vec<f64>>();
        let mut result = Args::from([
            ("axes_zyx".into(), Value::List(extent.into_iter().map(|v| Value::U64(v.into())).collect())),
            ("volume_fractions".into(), Value::List(data.keys().zip(&fractions).map(|(&id, &fraction)| {
                Value::Record(Args::from([("phase_id".into(), Value::U64(id.into())),
                    ("fraction".into(), Value::F64(fraction))]))
            }).collect())),
        ]);
        if regime != "hydraulic" && (args.contains_key("void_phase") || args.contains_key("grain_diameter_m")) {
            return Err("Void label and grain diameter apply to hydraulic homogenization".into());
        }
        match regime {
            "mechanical" | "acoustic" => {
                let moduli = compute_vrh_elasticity(&fractions, &lanes(0), &lanes(1))
                    .ok_or("Could not homogenize elastic phase moduli")?;
                let shear = moduli.shear_modulus_g_pa;
                let bulk = moduli.bulk_modulus_k_pa;
                let lambda = bulk - 2.0 * shear / 3.0;
                let stiffness = (0..6).map(|row| Value::List((0..6).map(|col| {
                    let entry = if row < 3 && col < 3 {
                        if row == col { lambda + 2.0 * shear } else { lambda }
                    } else if row == col { shear } else { 0.0 };
                    Value::F64(entry)
                }).collect())).collect();
                result.insert("stiffness_pa".into(), Value::List(stiffness));
                result.insert("bulk_pa".into(), Value::F64(bulk));
                result.insert("shear_pa".into(), Value::F64(shear));
                result.insert("youngs_pa".into(), Value::F64(moduli.youngs_modulus_e_pa));
                result.insert("poisson".into(), Value::F64(moduli.poissons_ratio_nu));
                if regime == "acoustic" {
                    let density = weighted_sum(&fractions, &lanes(2));
                    let epsilon_root = weighted_sum(&fractions, &lanes(3).into_iter().map(f64::sqrt).collect::<Vec<_>>());
                    let (p, s) = biot_acoustic_velocities(bulk, shear, density)
                        .ok_or("Could not derive bulk acoustic velocities")?;
                    result.insert("density_kg_m3".into(), Value::F64(density));
                    result.insert("relative_permittivity".into(), Value::F64(epsilon_root.powi(2)));
                    result.insert("p_wave_m_s".into(), Value::F64(p));
                    result.insert("s_wave_m_s".into(), Value::F64(s));
                }
            }
            "optical" => {
                let (absorption, scattering) = kubelka_munk_mixture(&fractions, &lanes(0), &lanes(1));
                if !(scattering > 0.0) {
                    return Err("Diffuse optical homogenization requires positive aggregate scattering".into());
                }
                result.insert("absorption_per_m".into(), Value::F64(absorption));
                result.insert("scattering_per_m".into(), Value::F64(scattering));
                result.insert("diffuse_reflectance".into(), Value::F64(kubelka_munk_reflectance(absorption, scattering)));
            }
            "hydraulic" => {
                let label = integer(required(args, "void_phase")?, "void_phase")?;
                let label = u16::try_from(label).map_err(|_| "Void phase label must fit u16")?;
                let porosity = data.get(&label).ok_or("Void phase is not declared")?.1 as f64 / sample_count as f64;
                let grain = num(required(args, "grain_diameter_m")?)?;
                if !(1e-9..=10.0).contains(&grain) {
                    return Err("Mean grain diameter exceeds the admitted range".into());
                }
                let permeability = kozeny_carman_permeability_m2(grain, porosity)
                    .ok_or("Kozeny-Carman requires a nonzero and nonsaturated void fraction")?;
                result.insert("porosity".into(), Value::F64(porosity));
                result.insert("isotropic_permeability_m2".into(), Value::F64(permeability));
            }
            _ => unreachable!("homogenization regime checked above"),
        }
        Ok(Value::Record(result))
    }
}

fn weighted_sum(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(&a, &b)| a * b).sum()
}

fn rve<'a>(host: &'a AssetHost<'_>, handle: &Value) -> Result<(&'a [u32], &'a [f32]), String> {
    let TypedArtifact::FieldLattice(field) = host.workspace.typed(handle)? else {
        return Err("RVE requires an admitted FieldLattice".into());
    };
    let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
        return Err("RVE requires a dense phase-label field".into());
    };
    let count = axes.iter().try_fold(1usize, |n, &axis| n.checked_mul(axis as usize));
    if axes.len() != 3 || count != Some(samples.len()) || !(1..=2_000_000).contains(&samples.len()) {
        return Err("RVE shape must describe a bounded nonempty ZYX volume".into());
    }
    Ok((axes, samples))
}
