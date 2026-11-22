//! Bounded periodic Maxwell evolution on a staggered Cartesian Yee lattice.

use crate::{
    Args, AssetHost, TypedArtifact, Value, artifact::FieldLattice, fields, integer, list, num,
    required, vector,
};
use qualia_core_db::q42::{
    emf_operators::{
        BoundaryCondition, Grid3dSpec, YeeMaterialParams, YeeSolverProfile, admit_yee_timestep,
        yee_div_b_2nd, yee_div_d_2nd, yee_step_leapfrog_in_place,
    },
    emf_profiles::{
        ConstitutiveSpec, ExtendedModeSpec, FieldClock, FieldFrame, FieldProvenance,
        FieldWriteLimits, GridIndexing, GridSpec, YeeRecord, write_extended_envelope,
    },
    field_semantics::{NodeState, Scratch},
};
use std::{collections::BTreeMap, sync::Arc};

const MAX_CELLS: usize = 65_536;
const MAX_AXIS: usize = 128;
const MAX_STEPS: usize = 4_096;
const FEMTOSECONDS_PER_SECOND: u64 = 1_000_000_000_000_000;
const PERIODIC: BoundaryCondition = BoundaryCondition::Periodic;
const STAGGER_NUM: [i64; 6] = [0, 0, 0, 1, 1, 1];
const STAGGER_DEN: [u64; 6] = [1, 1, 1, 2, 2, 2];
const COMPONENTS: [&str; 6] = ["Ex_V_m", "Ey_V_m", "Ez_V_m", "Hx_A_m", "Hy_A_m", "Hz_A_m"];

impl AssetHost<'_> {
    pub(crate) fn solve_maxwell_yee_3d(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "grid_dims",
                "spacing_m",
                "origin_m",
                "initial_e_v_per_m",
                "initial_h_a_per_m",
                "impressed_current_a_per_m2",
                "epsilon_relative",
                "mu_relative",
                "conductivity_s_per_m",
                "dt_s",
                "steps",
            ],
        )?;
        let dims = list(required(args, "grid_dims")?)?;
        let [nx, ny, nz] = dims else {
            return Err("grid_dims must contain [nx, ny, nz]".into());
        };
        let [nx, ny, nz] = [
            bounded_integer(nx, 2, MAX_AXIS, "grid_dims")?,
            bounded_integer(ny, 2, MAX_AXIS, "grid_dims")?,
            bounded_integer(nz, 2, MAX_AXIS, "grid_dims")?,
        ];
        let spacing = vector::<3>(required(args, "spacing_m")?)?;
        if spacing.iter().any(|x| !(1e-9..=1e8).contains(x)) {
            return Err("spacing_m must contain finite spacings from 1e-9 to 1e8 m".into());
        }
        let origin_m = args
            .get("origin_m")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0; 3]);
        if !origin_m.iter().all(|x| x.is_finite()) {
            return Err("origin_m components must be finite".into());
        }
        let [dx, dy, dz] = spacing;
        let grid = Grid3dSpec {
            nx,
            ny,
            nz,
            dx,
            dy,
            dz,
        };
        let n = grid
            .validate()
            .map_err(|err| format!("Yee grid: {err:?}"))?;
        if n > MAX_CELLS {
            return Err(format!("Yee grid exceeds {MAX_CELLS} cells"));
        }
        let axes = [nz as u32, ny as u32, nx as u32];
        let steps = bounded_integer(required(args, "steps")?, 1, MAX_STEPS, "steps")?;
        let material = YeeMaterialParams {
            epsilon_r: bounded_number(args, "epsilon_relative", 1.0, 1e-6, 1e6)?,
            mu_r: bounded_number(args, "mu_relative", 1.0, 1e-6, 1e6)?,
            sigma_s_per_m: bounded_number(args, "conductivity_s_per_m", 0.0, 0.0, 1e8)?,
        };
        let profile = if material.sigma_s_per_m == 0.0 {
            YeeSolverProfile::LosslessPeriodicNoPhiV1
        } else {
            YeeSolverProfile::ConductivePeriodicNoPhiV1
        };
        let requested_dt = num(required(args, "dt_s")?)?;
        let cfl_limit_s = admit_yee_timestep(&grid, &material, requested_dt)
            .map_err(|err| format!("Yee timestep: {err:?}"))?;
        let (clock, dt, dt_fs) = solver_clock(requested_dt)?;
        admit_yee_timestep(&grid, &material, dt)
            .map_err(|err| format!("Yee EMF clock timestep: {err:?}"))?;

        let scratch_bytes = n * 512 + (64 << 10);
        self.charge(n * 32, scratch_bytes)?;
        let mut e = read_vector(self, required(args, "initial_e_v_per_m")?, axes, n)?;
        let mut h = read_vector(self, required(args, "initial_h_a_per_m")?, axes, n)?;
        let j = match args.get("impressed_current_a_per_m2") {
            Some(value) => read_vector(self, value, axes, n)?,
            None => zero_vector(n),
        };
        let mut curl = zero_vector(n);

        let mu = material.mu_h_per_m();
        let initial_div_b = magnetic_divergence(&grid, &h, mu, &mut curl[0])?;
        let max_h = h
            .iter()
            .flatten()
            .fold(0.0_f64, |peak, x| peak.max(x.abs()));
        let divergence_scale = mu * max_h * (dx.recip() + dy.recip() + dz.recip());
        let divergence_tolerance = 256.0 * f64::EPSILON * divergence_scale;
        if initial_div_b > divergence_tolerance {
            return Err(format!(
                "Initial Yee H violates discrete magnetic Gauss law ({initial_div_b:e} T/m; tolerated {divergence_tolerance:e} T/m)"
            ));
        }

        let [ex, ey, ez] = &mut e;
        let [hx, hy, hz] = &mut h;
        let [jx, jy, jz] = &j;
        let [curl_x, curl_y, curl_z] = &mut curl;
        for _ in 0..steps {
            self.charge(n * 96, scratch_bytes)?;
            yee_step_leapfrog_in_place(
                &grid, profile, PERIODIC, &material, dt, ex, ey, ez, hx, hy, hz, jx, jy, jz,
                curl_x, curl_y, curl_z,
            )
            .map_err(|err| format!("Yee step: {err:?}"))?;
        }

        self.charge(n * 40, scratch_bytes)?;
        let final_div_b = magnetic_divergence(&grid, &h, mu, &mut curl[0])?;
        let rho = &mut curl[0];
        yee_div_d_2nd(&grid, PERIODIC, &e[0], &e[1], &e[2], rho)
            .map_err(|err| format!("Yee electric divergence: {err:?}"))?;
        let epsilon = material.epsilon_f_per_m();
        rho.iter_mut().for_each(|sample| *sample *= epsilon);

        let samples = e
            .iter()
            .chain(h.iter())
            .flatten()
            .map(|&x| x as f32)
            .collect::<Vec<_>>();
        if !samples.iter().all(|x| x.is_finite()) {
            return Err("Final Yee fields exceed the finite DenseF32 projection".into());
        }
        let mut payload = vec![0; n * 128];
        for (i, cell) in payload.chunks_exact_mut(128).enumerate() {
            let record = YeeRecord {
                electric_field_v_per_m: [e[0][i], e[1][i], e[2][i]],
                magnetic_field_h_a_per_m: [h[0][i], h[1][i], h[2][i]],
                impressed_current_j_a_per_m2: [j[0][i], j[1][i], j[2][i]],
                scalar_potential_v: 0.0,
                charge_density_c_per_m3: rho[i],
                epsilon_relative: material.epsilon_r,
                mu_relative: material.mu_r,
                conductivity_s_per_m: material.sigma_s_per_m,
                material_entry: 0,
                constitutive_metadata_id: 6,
                state_record_index: u64::MAX,
            };
            let cell: &mut [u8; 128] = cell.try_into().map_err(|_| "Invalid Yee cell stride")?;
            record
                .encode(true, cell)
                .map_err(|err| format!("Yee EMF cell {i}: {err:?}"))?;
        }
        let frame = FieldFrame {
            origin_m,
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            grid: Some(GridSpec {
                dimensions: [nx as u32, ny as u32, nz as u32],
                brick_dimensions: [nx as u32, ny as u32, nz as u32],
                spacing_m: spacing,
                indexing: GridIndexing::Linear,
            }),
        };
        let emf = encode_emf(clock, frame, material, steps, &payload)?;
        let fields = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![6, axes[0], axes[1], axes[2]],
                samples,
            },
        )))?;
        let emf = self.admit_encoded("emf", emf)?;
        Ok(Value::Record(BTreeMap::from([
            ("fields".into(), fields),
            ("emf".into(), emf),
            (
                "component_names".into(),
                Value::List(COMPONENTS.map(|name| Value::String(name.into())).into()),
            ),
            ("boundary".into(), Value::String("periodic".into())),
            ("requested_dt_s".into(), Value::F64(requested_dt)),
            ("dt_s".into(), Value::F64(dt)),
            ("dt_femtoseconds".into(), Value::U64(dt_fs)),
            ("cfl_limit_s".into(), Value::F64(cfl_limit_s)),
            ("e_time_s".into(), Value::F64(steps as f64 * dt)),
            ("h_time_s".into(), Value::F64((steps as f64 + 0.5) * dt)),
            ("phi_available".into(), Value::Bool(false)),
            ("initial_div_b_t_per_m".into(), Value::F64(initial_div_b)),
            ("final_div_b_t_per_m".into(), Value::F64(final_div_b)),
        ])))
    }
}

fn encode_emf(
    clock: FieldClock,
    frame: FieldFrame,
    material: YeeMaterialParams,
    steps: usize,
    payload: &[u8],
) -> Result<Vec<u8>, String> {
    let model = if material == YeeMaterialParams::vacuum() {
        "vacuum"
    } else {
        "isotropic_nondispersive"
    };
    let spec = ExtendedModeSpec::Yee {
        constitutive: ConstitutiveSpec {
            model,
            epsilon_infinity_relative: material.epsilon_r,
            mu_relative: material.mu_r,
            conductivity_s_per_m: material.sigma_s_per_m,
            delta_epsilon: &[],
            relaxation_time_s: &[],
            resonance_rad_s: &[],
            damping_s1: &[],
        },
        time_offset_num: &STAGGER_NUM,
        time_offset_den: &STAGGER_DEN,
        material: None,
    };
    let provenance = FieldProvenance {
        role: 2,
        algorithm: Some("fdtd-yee-periodic"),
        algorithm_version: Some("1.0"),
        lineage: Some("authored-staggered-fields"),
    };
    let mut metadata = [0; 4_096];
    let mut staging = vec![0; payload.len() + 16_384];
    let mut output = vec![0; staging.len()];
    let mut scratch = Scratch {
        metadata: &mut [NodeState::default(); 32],
        entries: &mut [NodeState::default(); 8],
    };
    let used = write_extended_envelope(
        clock,
        frame,
        provenance,
        0x0102,
        Some(1),
        steps as i64,
        1,
        Some(0),
        (payload.len() / 128) as u64,
        payload,
        &spec,
        &mut metadata,
        &mut staging,
        &mut output,
        &mut scratch,
        FieldWriteLimits::default(),
    )
    .map_err(|err| format!("Yee EMF envelope: {err:?}"))?
    .len();
    output.truncate(used);
    Ok(output)
}

fn solver_clock(requested: f64) -> Result<(FieldClock, f64, u64), String> {
    // Keep the actual step at or below the authored step. Coarser units allow
    // long steps to fit the native envelope's reduced denominator bound.
    let scale = FEMTOSECONDS_PER_SECOND as f64;
    if !(scale.recip()..=1.0).contains(&requested) {
        return Err("dt_s must be between one femtosecond and one second for the EMF clock".into());
    }
    for unit_fs in [1, 10, 100, 1_000] {
        let mut duration = (requested * scale / unit_fs as f64).round() as u64 * unit_fs;
        if duration == 0 {
            continue;
        }
        let (mut num, mut den) = clock_rate(duration);
        let mut dt = den as f64 / num as f64;
        if dt > requested {
            duration -= unit_fs;
            if duration == 0 {
                continue;
            }
            (num, den) = clock_rate(duration);
            dt = den as f64 / num as f64;
        }
        if dt <= requested && den <= 1_000_000_000_000 {
            return Ok((
                FieldClock {
                    scale: 0,
                    epoch_seconds: 0,
                    epoch_nanoseconds: 0,
                    ticks_per_second_num: num,
                    ticks_per_second_den: den,
                },
                dt,
                duration,
            ));
        }
    }
    Err("dt_s is outside the native reduced-rational EMF clock bounds".into())
}

fn clock_rate(duration_fs: u64) -> (u64, u64) {
    let (mut a, mut b) = (FEMTOSECONDS_PER_SECOND, duration_fs);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    (FEMTOSECONDS_PER_SECOND / a, duration_fs / a)
}

fn magnetic_divergence(
    grid: &Grid3dSpec,
    h: &[Vec<f64>; 3],
    mu: f64,
    scratch: &mut [f64],
) -> Result<f64, String> {
    yee_div_b_2nd(grid, PERIODIC, &h[0], &h[1], &h[2], scratch)
        .map_err(|err| format!("Yee magnetic divergence: {err:?}"))?;
    let mut peak = 0.0_f64;
    for &sample in scratch.iter() {
        if !sample.is_finite() {
            return Err("Yee magnetic divergence is non-finite".into());
        }
        peak = peak.max(sample.abs());
    }
    Ok(mu * peak)
}

fn zero_vector(n: usize) -> [Vec<f64>; 3] {
    std::array::from_fn(|_| vec![0.0; n])
}

fn read_vector(
    host: &AssetHost<'_>,
    value: &Value,
    axes: [u32; 3],
    n: usize,
) -> Result<[Vec<f64>; 3], String> {
    let components = list(value)?;
    let [x, y, z] = components else {
        return Err("Yee field vector requires exactly three component inputs".into());
    };
    Ok([
        read_component(host, x, axes, n)?,
        read_component(host, y, axes, n)?,
        read_component(host, z, axes, n)?,
    ])
}

fn read_component(
    host: &AssetHost<'_>,
    value: &Value,
    axes: [u32; 3],
    n: usize,
) -> Result<Vec<f64>, String> {
    let result = match host.workspace.typed(value) {
        Ok(TypedArtifact::FieldLattice(field)) => match field.as_ref() {
            FieldLattice::DenseF32 {
                axes: actual,
                samples,
            } if actual.as_slice() == axes && samples.len() == n => {
                samples.iter().copied().map(f64::from).collect::<Vec<_>>()
            }
            _ => return Err("Yee field artifact requires DenseF32 shape [nz, ny, nx]".into()),
        },
        Ok(_) => return Err("Yee field component must be numeric or a DenseF32 artifact".into()),
        Err(_) => match value {
            Value::List(values) if values.len() == n => {
                values.iter().map(num).collect::<Result<_, _>>()?
            }
            Value::List(_) => return Err("Yee field component count differs from nx×ny×nz".into()),
            uniform => vec![num(uniform)?; n],
        },
    };
    if result.iter().any(|x| !x.is_finite() || x.abs() > 1e30) {
        return Err("Yee field component must be finite with magnitude at most 1e30".into());
    }
    Ok(result)
}

fn bounded_number(args: &Args, key: &str, default: f64, min: f64, max: f64) -> Result<f64, String> {
    let value = args.get(key).map(num).transpose()?.unwrap_or(default);
    (min..=max)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} must be in {min}..={max}"))
}

fn bounded_integer(value: &Value, min: usize, max: usize, name: &str) -> Result<usize, String> {
    let value = integer(value, name)?;
    (min..=max)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{name} must be in {min}..={max}"))
}
