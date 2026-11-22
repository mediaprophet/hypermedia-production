//! Deep Qualia core production adapters.

use std::collections::BTreeMap;

use qualia_core_db::specialized_libs::physics_simulation::{
    cartoon_deformation::{self as cartoon, CartoonKineticsConfig},
    homogenization_acoust as acoustic, homogenization_hydro as hydro,
    homogenization_mech as mechanical, homogenization_optic as optical,
};

use crate::{fields, list, num, number, record, required, text, vector, Args, AssetHost, Value};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;

const GRAPH: &str = "urn:qualia:physics:homogenization:v1";
const MAX_PHASES: usize = 64;

mod geometry;
mod gtop;
mod rdf_temporal;
mod ternary;
mod twin;

impl AssetHost<'_> {
    pub(crate) fn prove_equivalence_scrub_and_rdf_star(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        rdf_temporal::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn homogenize_multiphase_and_cartoon_kinetics(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["task", "parameters"])?;
        let task = text(required(args, "task")?)?;
        let parameters = record(required(args, "parameters")?)?;
        self.charge(4 * MAX_PHASES, 64 * 1024)?;
        let values = physics(task, parameters)?;
        let args_json = crate::living_world_anatomy::species_spec::vibe_value_to_json(&Value::Record(parameters.clone()));
        let canonical = serde_json::to_vec(&(task, args_json)).map_err(|error| error.to_string())?;
        let subject = format!("urn:qualia:physics:result:{}", blake3::hash(&canonical).to_hex());
        let mut receipt = ReceiptGraph::new(GRAPH);
        receipt.fact(&subject, "urn:qualia:physics:task", &format!("urn:qualia:physics:task:{task}"))?;
        for (field, value) in &values {
            let object = match value {
                Value::F64(value) if value.is_finite() => format!("urn:qualia:physics:double:{:016x}", value.to_bits()),
                Value::Bool(value) => format!("urn:qualia:physics:boolean:{value}"),
                _ => return Err("Physics evaluation produced a non-finite result".into()),
            };
            receipt.fact(&subject, &format!("urn:qualia:physics:{field}"), &object)?;
        }
        let budget = self.limits.resource_bytes.min(4 * 1024 * 1024);
        Ok(Value::Record(BTreeMap::from([
            ("result".into(), Value::Record(values)),
            ("receipt".into(), self.encode("q42", receipt.seal(budget)?)?),
        ])))
    }
}

fn physics(task: &str, args: &Args) -> Result<Args, String> {
    let get = |key| num(required(args, key)?);
    let result = match task {
        "cartoon_wafer_floor" => {
            fields(args, &["compression_lambda", "initial_volume_m3", "config"])?;
            let config = kinetics_config(args)?;
            let compression = nonnegative(get("compression_lambda")?, "compression_lambda")?;
            let volume = nonnegative(get("initial_volume_m3")?, "initial_volume_m3")?;
            let (lambda, determinant) = cartoon::compute_cartoon_jacobian_det(compression, &config);
            numeric([("adjusted_lambda", lambda), ("determinant", determinant),
                ("volume_deficit_m3", cartoon::compute_wafer_volume_deficit_m3(volume, compression, &config))])
        }
        "cartoon_anharmonic_step" => {
            fields(args, &["displacement_m", "velocity_m_s", "mass_kg", "force_n", "dt_s", "config"])?;
            let config = kinetics_config(args)?;
            let dt = positive(get("dt_s")?, "dt_s")?;
            if dt > 0.1 { return Err("Cartoon oscillator step must be at most 0.1 seconds".into()); }
            let (q, v) = cartoon::step_anharmonic_oscillator(get("displacement_m")?, get("velocity_m_s")?,
                positive(get("mass_kg")?, "mass_kg")?, get("force_n")?, dt, &config);
            numeric([("displacement_m", q), ("velocity_m_s", v)])
        }
        "cartoon_apex_aerodynamics" => {
            fields(args, &["velocity_m_s", "density_kg_m3", "drag_coefficient", "area_m2", "config"])?;
            let config = kinetics_config(args)?;
            let [x, y, z] = cartoon::compute_comedic_aerodynamics(vector(required(args, "velocity_m_s")?)?,
                nonnegative(get("density_kg_m3")?, "density_kg_m3")?,
                nonnegative(get("drag_coefficient")?, "drag_coefficient")?,
                nonnegative(get("area_m2")?, "area_m2")?, &config);
            numeric([("force_x_n", x), ("force_y_n", y), ("force_z_n", z)])
        }
        "voigt_reuss_hill" => {
            fields(args, &["fractions", "bulk_moduli_pa", "shear_moduli_pa"])?;
            let fractions = fractions(args)?;
            let bulk = phases(args, "bulk_moduli_pa", fractions.len(), f64::MIN_POSITIVE)?;
            let shear = phases(args, "shear_moduli_pa", fractions.len(), f64::MIN_POSITIVE)?;
            let vrh = mechanical::compute_vrh_elasticity(&fractions, &bulk, &shear).ok_or("Invalid VRH material mixture")?;
            numeric([("bulk_modulus_pa", vrh.bulk_modulus_k_pa), ("shear_modulus_pa", vrh.shear_modulus_g_pa),
                ("youngs_modulus_pa", vrh.youngs_modulus_e_pa), ("poisson_ratio", vrh.poissons_ratio_nu)])
        }
        "drucker_prager" => {
            fields(args, &["cauchy_stress_pa", "cohesion_pa", "friction_angle_rad"])?;
            let cohesion = nonnegative(get("cohesion_pa")?, "cohesion_pa")?;
            let angle = get("friction_angle_rad")?;
            if !(0.0..std::f64::consts::FRAC_PI_2).contains(&angle) { return Err("Friction angle must be in [0, π/2)".into()); }
            let stress = vector::<6>(required(args, "cauchy_stress_pa")?)?;
            if stress.iter().any(|value| value.abs() > 1.0e20) { return Err("Cauchy stress exceeds configured magnitude".into()); }
            let yield_pa = mechanical::drucker_prager_yield_stress(cohesion, angle, stress[..3].iter().sum());
            let mut result = numeric([("yield_stress_pa", yield_pa)]);
            result.insert("failed".into(), Value::Bool(mechanical::check_drucker_prager_failure(stress, cohesion, angle)));
            result
        }
        "bachmann_wetting" => {
            fields(args, &["dry_reflectance", "mineral_index", "water_index", "saturation"])?;
            numeric([("wet_reflectance", optical::bachmann_wetting_reflectance(
                fraction(get("dry_reflectance")?, "dry_reflectance")?,
                positive(get("mineral_index")?, "mineral_index")?, positive(get("water_index")?, "water_index")?,
                fraction(get("saturation")?, "saturation")?))])
        }
        "kubelka_munk_pigment" => {
            fields(args, &["fractions", "absorption", "scattering"])?;
            let fractions = fractions(args)?;
            let absorption = phases(args, "absorption", fractions.len(), 0.0)?;
            let scattering = phases(args, "scattering", fractions.len(), 0.0)?;
            let (k, s) = optical::kubelka_munk_mixture(&fractions, &absorption, &scattering);
            positive(s, "aggregate scattering")?;
            numeric([("absorption", k), ("scattering", s), ("reflectance", optical::kubelka_munk_reflectance(k, s))])
        }
        "crim_permittivity" => {
            fields(args, &["fractions", "mineral_permittivities", "porosity", "saturation", "water_permittivity", "air_permittivity"])?;
            let fractions = fractions(args)?;
            let minerals = phases(args, "mineral_permittivities", fractions.len(), 1.0)?;
            let porosity = get("porosity")?;
            let water = get("water_permittivity")?;
            let air = get("air_permittivity")?;
            if !(0.0..=0.99).contains(&porosity) || water < 1.0 || air < 1.0 {
                return Err("CRIM requires porosity in [0, 0.99] and relative permittivities at least one".into());
            }
            numeric([("relative_permittivity", acoustic::crim_dielectric_permittivity(porosity,
                fraction(get("saturation")?, "saturation")?, &fractions, &minerals, water, air))])
        }
        "biot_velocities" => {
            fields(args, &["bulk_modulus_pa", "shear_modulus_pa", "density_kg_m3"])?;
            let (p, s) = acoustic::biot_acoustic_velocities(get("bulk_modulus_pa")?, get("shear_modulus_pa")?,
                get("density_kg_m3")?).ok_or("Invalid Biot material properties")?;
            numeric([("p_wave_m_s", p), ("s_wave_m_s", s)])
        }
        "kozeny_carman" => {
            fields(args, &["grain_diameter_m", "porosity"])?;
            numeric([("permeability_m2", hydro::kozeny_carman_permeability_m2(get("grain_diameter_m")?, get("porosity")?)
                .ok_or("Invalid grain diameter or porosity")?)])
        }
        "exner_sediment" => {
            fields(args, &["dt_s", "bed_porosity", "flux_divergence_m_s"])?;
            numeric([("bed_delta_m", hydro::exner_elevation_change_m(get("dt_s")?, get("bed_porosity")?,
                get("flux_divergence_m_s")?).ok_or("Invalid Exner sediment parameters")?)])
        }
        "van_genuchten" => {
            fields(args, &["suction_head_m", "alpha_inv_m", "pore_exponent"])?;
            numeric([("effective_saturation", hydro::van_genuchten_effective_saturation(get("suction_head_m")?,
                get("alpha_inv_m")?, get("pore_exponent")?).ok_or("Invalid van Genuchten parameters")?)])
        }
        _ => return Err(format!("Unknown multiphase or cartoon task {task}")),
    };
    Ok(result)
}

fn numeric<const N: usize>(values: [(&str, f64); N]) -> Args {
    values.into_iter().map(|(name, value)| (name.into(), Value::F64(value))).collect()
}

fn positive(value: f64, field: &str) -> Result<f64, String> {
    if value > 0.0 && value.is_finite() { Ok(value) } else { Err(format!("{field} must be positive")) }
}

fn nonnegative(value: f64, field: &str) -> Result<f64, String> {
    if value >= 0.0 && value.is_finite() { Ok(value) } else { Err(format!("{field} must be nonnegative")) }
}

fn fraction(value: f64, field: &str) -> Result<f64, String> {
    if (0.0..=1.0).contains(&value) { Ok(value) } else { Err(format!("{field} must be in [0, 1]")) }
}

fn fractions(args: &Args) -> Result<Vec<f64>, String> {
    let raw = list(required(args, "fractions")?)?;
    if raw.is_empty() || raw.len() > MAX_PHASES { return Err("Provide 1–64 constituent fractions".into()); }
    let values = raw.iter().map(|item| fraction(num(item)?, "fraction")).collect::<Result<Vec<_>, _>>()?;
    if (values.iter().sum::<f64>() - 1.0).abs() > 1e-8 { return Err("Constituent fractions must sum to one".into()); }
    Ok(values)
}

fn phases(args: &Args, field: &str, len: usize, min: f64) -> Result<Vec<f64>, String> {
    let raw = list(required(args, field)?)?;
    if raw.len() != len { return Err(format!("{field} must have one value per constituent")); }
    raw.iter().map(|item| {
        let value = num(item)?;
        if value >= min { Ok(value) } else { Err(format!("{field} values must be at least {min}")) }
    }).collect()
}

fn kinetics_config(args: &Args) -> Result<CartoonKineticsConfig, String> {
    let mut config = CartoonKineticsConfig::default();
    let Some(value) = args.get("config") else { return Ok(config); };
    let overrides = record(value)?;
    fields(overrides, &["wafer_floor", "poisson_exponent", "linear_stiffness", "quadratic_asymmetry",
        "cubic_hardening", "damping", "apex_force_n", "apex_velocity_m_s"])?;
    let get = |key, default| number(overrides, key, default);
    config.wafer_floor_epsilon = positive(get("wafer_floor", config.wafer_floor_epsilon)?, "wafer_floor")?;
    config.cartoon_poisson_nu = get("poisson_exponent", config.cartoon_poisson_nu)?;
    if config.wafer_floor_epsilon > 1.0 || !(0.0..=0.5).contains(&config.cartoon_poisson_nu) {
        return Err("Wafer floor must be at most one and Poisson exponent in [0, 0.5]".into());
    }
    config.linear_stiffness_k = nonnegative(get("linear_stiffness", config.linear_stiffness_k)?, "linear_stiffness")?;
    config.quadratic_asymmetry_a = get("quadratic_asymmetry", config.quadratic_asymmetry_a)?;
    config.cubic_hardening_b = nonnegative(get("cubic_hardening", config.cubic_hardening_b)?, "cubic_hardening")?;
    config.damping_gamma = nonnegative(get("damping", config.damping_gamma)?, "damping")?;
    config.comedic_apex_hang_force_n = nonnegative(get("apex_force_n", config.comedic_apex_hang_force_n)?, "apex_force_n")?;
    config.apex_velocity_threshold_v0 = positive(get("apex_velocity_m_s", config.apex_velocity_threshold_v0)?, "apex_velocity_m_s")?;
    Ok(config)
}
