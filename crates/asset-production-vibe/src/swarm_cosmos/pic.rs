//! Periodic one-coordinate electromagnetic PIC with three velocity components.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{count, cross, names, numeric_list, scalar};
use crate::{artifact::FieldLattice, fields, vector, Args, AssetHost, TypedArtifact, Value};
use qualia_core_db::q42::emf_operators::{
    admit_yee_timestep, yee_curl_e_2nd, yee_step_leapfrog_in_place, BoundaryCondition, Grid3dSpec,
    YeeMaterialParams, YeeSolverProfile, C0_M_PER_S, ELEMENTARY_CHARGE_C, EPS0_REF_F_PER_M,
    MU0_REF_H_PER_M,
};
use qualia_core_db::q42::emf_profiles::{
    write_extended_envelope, ConstitutiveSpec, ExtendedModeSpec, FieldClock, FieldFrame,
    FieldProvenance, FieldWriteLimits, GridIndexing, GridSpec, YeeRecord,
};
use qualia_core_db::q42::field_semantics::{NodeState, Scratch};
use std::{collections::BTreeMap, f64::consts::TAU, sync::Arc};

const ELECTRON_MASS_KG: f64 = 9.109_383_713_9e-31;
const FS_PER_SECOND: u64 = 1_000_000_000_000_000;
const PERIODIC: BoundaryCondition = BoundaryCondition::Periodic;

struct Particle {
    cell_position: f64,
    proper_velocity_over_c: [f64; 3],
}

struct Plasma {
    grid: Grid3dSpec,
    dt: f64,
    density: f64,
    sheet_charge: f64,
    particles: Vec<Particle>,
    electric: [Vec<f64>; 3],
    magnetic_h: [Vec<f64>; 3],
    current: [Vec<f64>; 3],
    charge: Vec<f64>,
    scratch_charge: Vec<f64>,
    curl: [Vec<f64>; 3],
}

impl AssetHost<'_> {
    pub(crate) fn solve_vlasov_maxwell_pic_plasma(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "cells",
                "cell_size_m",
                "electron_density_m3",
                "particles_per_cell",
                "steps",
                "dt_femtoseconds",
                "density_perturbation",
                "counterstream_speed_m_s",
                "transverse_seed_speed_m_s",
                "background_b_t",
            ],
        )?;
        let n = count(args, "cells", 64, 8, 512)?;
        let per_cell = count(args, "particles_per_cell", 8, 2, 32)?;
        if per_cell % 2 != 0 {
            return Err("particles_per_cell must be even for a balanced counterstream".into());
        }
        let steps = count(args, "steps", 128, 1, 4_096)?;
        let dx = scalar(args, "cell_size_m", Some(0.01), 1e-5, 100.0)?;
        let density = scalar(args, "electron_density_m3", Some(1e11), 1e4, 1e15)?;
        let perturbation = scalar(args, "density_perturbation", Some(0.01), -0.25, 0.25)?;
        let counterstream = scalar(args, "counterstream_speed_m_s", Some(0.0), 0.0, 1e7)?;
        let transverse_seed = scalar(args, "transverse_seed_speed_m_s", Some(0.0), 0.0, 1e7)?;
        let background_b = args
            .get("background_b_t")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0; 3]);
        if norm_squared(background_b) > 0.05_f64.powi(2) {
            return Err("background_b_t magnitude must be at most 0.05 T".into());
        }
        let plasma_omega =
            (density * ELEMENTARY_CHARGE_C.powi(2) / (EPS0_REF_F_PER_M * ELECTRON_MASS_KG)).sqrt();
        let cyclotron_omega =
            ELEMENTARY_CHARGE_C * norm_squared(background_b).sqrt() / ELECTRON_MASS_KG;
        let cfl = dx / (3.0_f64.sqrt() * C0_M_PER_S);
        let resolution_limit = (0.85 * cfl).min(0.2 / plasma_omega.max(cyclotron_omega));
        let max_dt_fs = (resolution_limit * FS_PER_SECOND as f64).floor() as usize;
        let dt_fs = count(
            args,
            "dt_femtoseconds",
            (max_dt_fs / 2).max(1),
            1,
            max_dt_fs,
        )?;
        let dt = dt_fs as f64 / FS_PER_SECOND as f64;
        let grid = Grid3dSpec {
            nx: n,
            ny: 1,
            nz: 1,
            dx,
            dy: dx,
            dz: dx,
        };
        let vacuum = YeeMaterialParams::vacuum();
        admit_yee_timestep(&grid, &vacuum, dt).map_err(|err| format!("PIC timestep: {err:?}"))?;
        let total = n * per_cell;
        self.charge(total * 128 + n * 96, total * 72 + n * 1000 + 65_536)?;
        let particles = (0..total)
            .map(|i| {
                let fraction = (i as f64 + 0.5) / total as f64;
                let (sin, cos) = (TAU * fraction).sin_cos();
                let ux = if i % 2 == 0 {
                    counterstream
                } else {
                    -counterstream
                };
                let velocity_over_c =
                    [ux, transverse_seed * sin, transverse_seed * cos].map(|v| v / C0_M_PER_S);
                let gamma = (1.0 - norm_squared(velocity_over_c)).sqrt().recip();
                Particle {
                    cell_position: n as f64 * (fraction - perturbation * sin / TAU),
                    proper_velocity_over_c: velocity_over_c.map(|v| v * gamma),
                }
            })
            .collect();
        let zero_fields = || std::array::from_fn(|_| vec![0.0; n]);
        let mut plasma = Plasma {
            grid,
            dt,
            density,
            sheet_charge: -ELEMENTARY_CHARGE_C * density * dx / per_cell as f64,
            particles,
            electric: zero_fields(),
            magnetic_h: background_b.map(|field| vec![field / MU0_REF_H_PER_M; n]),
            current: zero_fields(),
            charge: vec![0.0; n],
            scratch_charge: vec![0.0; n],
            curl: zero_fields(),
        };
        plasma.initialize_electric_charge();
        let initial_energy = plasma.energy_per_area();
        let mut peak_gauss = plasma.gauss_defect();
        let mut peak_continuity: f64 = 0.0;
        let tolerance =
            64.0 * f64::EPSILON * (steps + n + per_cell) as f64 * ELEMENTARY_CHARGE_C * density;
        for _ in 0..steps {
            self.charge(total * 192 + n * 256, 0)?;
            peak_continuity = peak_continuity.max(plasma.step(&vacuum)?);
            peak_gauss = peak_gauss.max(plasma.gauss_defect());
            if peak_gauss > tolerance || peak_continuity > tolerance {
                return Err(
                    "Continuity or Gauss-law defect exceeded the roundoff allowance".into(),
                );
            }
        }
        let final_energy = plasma.energy_per_area();
        if !initial_energy.is_finite() || !final_energy.is_finite() {
            return Err("PIC solver produced nonfinite staggered energy".into());
        }
        let samples = plasma
            .electric
            .iter()
            .chain(&plasma.magnetic_h)
            .chain(&plasma.current)
            .flatten()
            .chain(&plasma.charge)
            .map(|x| *x as f32)
            .collect::<Vec<_>>();
        if samples.iter().any(|x| !x.is_finite()) {
            return Err("PIC fields exceed finite DenseF32 output".into());
        }
        let particle_samples = (0..4)
            .flat_map(|column| {
                plasma.particles.iter().map(move |p| {
                    if column == 0 {
                        (p.cell_position * dx) as f32
                    } else {
                        let u = p.proper_velocity_over_c;
                        (C0_M_PER_S * u[column - 1] / (1.0 + norm_squared(u)).sqrt()) as f32
                    }
                })
            })
            .collect();
        let emf = plasma.encode_emf(dt_fs as u64, steps)?;
        let grid_fields = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![10, n as u32],
                samples,
            },
        )))?;
        let particles = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![4, total as u32],
                samples: particle_samples,
            },
        )))?;
        let emf = self.admit_encoded("emf", emf)?;
        Ok(Value::Record(BTreeMap::from([
            ("fields".into(), grid_fields),
            ("particles".into(), particles),
            ("emf".into(), emf),
            (
                "field_channels".into(),
                names([
                    "ex_v_m",
                    "ey_v_m",
                    "ez_v_m",
                    "hx_a_m",
                    "hy_a_m",
                    "hz_a_m",
                    "jx_a_m2",
                    "jy_a_m2",
                    "jz_a_m2",
                    "charge_c_m3",
                ]),
            ),
            (
                "particle_channels".into(),
                names(["x_m", "vx_m_s", "vy_m_s", "vz_m_s"]),
            ),
            (
                "field_x_offsets_cells".into(),
                numeric_list([0.5, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0]),
            ),
            ("x_step_m".into(), Value::F64(dx)),
            ("dt_s".into(), Value::F64(dt)),
            ("dt_femtoseconds".into(), Value::U64(dt_fs as u64)),
            (
                "e_and_particle_position_time_s".into(),
                Value::F64(steps as f64 * dt),
            ),
            ("h_time_s".into(), Value::F64((steps as f64 + 0.5) * dt)),
            (
                "j_and_particle_velocity_time_s".into(),
                Value::F64((steps as f64 - 0.5) * dt),
            ),
            ("peak_gauss_defect_c_m3".into(), Value::F64(peak_gauss)),
            (
                "peak_step_continuity_defect_c_m3".into(),
                Value::F64(peak_continuity),
            ),
            (
                "initial_staggered_energy_j_m2".into(),
                Value::F64(initial_energy),
            ),
            (
                "final_staggered_energy_j_m2".into(),
                Value::F64(final_energy),
            ),
            (
                "model".into(),
                Value::String("periodic_1d3v_electron_pic_immobile_ions".into()),
            ),
        ])))
    }
}

impl Plasma {
    fn initialize_electric_charge(&mut self) {
        let volume_charge = self.sheet_charge / self.grid.dx;
        self.charge.fill(ELEMENTARY_CHARGE_C * self.density);
        for particle in &self.particles {
            deposit_nodes(&mut self.charge, particle.cell_position, volume_charge);
        }
        let mut field = 0.0;
        for (face, charge) in self.electric[0].iter_mut().zip(&self.charge) {
            field += charge * self.grid.dx / EPS0_REF_F_PER_M;
            *face = field;
        }
        let mean = self.electric[0].iter().sum::<f64>() / self.grid.nx as f64;
        self.electric[0].iter_mut().for_each(|v| *v -= mean);
    }

    fn step(&mut self, vacuum: &YeeMaterialParams) -> Result<f64, String> {
        let [cx, cy, cz] = &mut self.curl;
        let [ex, ey, ez] = &mut self.electric;
        let [hx, hy, hz] = &mut self.magnetic_h;
        yee_curl_e_2nd(&self.grid, PERIODIC, ex, ey, ez, cx, cy, cz)
            .map_err(|err| format!("PIC magnetic centering: {err:?}"))?;
        self.current.iter_mut().for_each(|lane| lane.fill(0.0));
        self.scratch_charge.fill(ELEMENTARY_CHARGE_C * self.density);
        let n = self.grid.nx;
        let dx = self.grid.dx;
        let charge_per_volume = self.sheet_charge / dx;
        let charge_per_time = self.sheet_charge / self.dt;
        for particle in &mut self.particles {
            let x = particle.cell_position;
            let edge = x - 0.5;
            let e = [
                interpolate(ex, edge),
                interpolate(ey, x),
                interpolate(ez, x),
            ];
            let h = [
                interpolate(hx, x),
                interpolate(hy, edge),
                interpolate(hz, edge),
            ];
            let curl = [
                interpolate(cx, x),
                interpolate(cy, edge),
                interpolate(cz, edge),
            ];
            // The native H field is advanced by half a step relative to E at loop entry.
            let b = std::array::from_fn(|i| MU0_REF_H_PER_M * h[i] + 0.5 * self.dt * curl[i]);
            let u = boris(particle.proper_velocity_over_c, e, b, self.dt);
            let u_squared = norm_squared(u);
            if !u_squared.is_finite() {
                return Err("Relativistic PIC particle left the finite state domain".into());
            }
            let gamma = (1.0 + u_squared).sqrt();
            let v = u.map(|p| C0_M_PER_S * (p / gamma));
            let delta = v[0] * self.dt / dx;
            let end = x + delta;
            deposit_current(
                &mut self.current,
                x,
                end,
                charge_per_time,
                [v[1], v[2]].map(|v| charge_per_volume * v),
            );
            particle.cell_position = end.rem_euclid(n as f64);
            particle.proper_velocity_over_c = u;
            deposit_nodes(
                &mut self.scratch_charge,
                particle.cell_position,
                charge_per_volume,
            );
        }
        let [jx, jy, jz] = &self.current;
        let continuity = self
            .scratch_charge
            .iter()
            .zip(&self.charge)
            .enumerate()
            .fold(0.0_f64, |peak, (i, (next, previous))| {
                let left = (i + n - 1) % n;
                peak.max((next - previous + (self.dt / dx) * (jx[i] - jx[left])).abs())
            });
        yee_step_leapfrog_in_place(
            &self.grid,
            YeeSolverProfile::LosslessPeriodicNoPhiV1,
            PERIODIC,
            vacuum,
            self.dt,
            ex,
            ey,
            ez,
            hx,
            hy,
            hz,
            jx,
            jy,
            jz,
            cx,
            cy,
            cz,
        )
        .map_err(|err| format!("PIC Yee field step: {err:?}"))?;
        std::mem::swap(&mut self.charge, &mut self.scratch_charge);
        Ok(continuity)
    }

    fn gauss_defect(&self) -> f64 {
        let ex = &self.electric[0];
        let n = ex.len();
        ex.iter()
            .zip(&self.charge)
            .enumerate()
            .fold(0.0_f64, |peak, (i, (field, charge))| {
                let div_d = EPS0_REF_F_PER_M * (field - ex[(i + n - 1) % n]) / self.grid.dx;
                peak.max((div_d - charge).abs())
            })
    }

    fn energy_per_area(&self) -> f64 {
        let square_sum = |field: &[Vec<f64>; 3]| field.iter().flatten().map(|v| v * v).sum::<f64>();
        let field_energy = 0.5
            * self.grid.dx
            * (EPS0_REF_F_PER_M * square_sum(&self.electric)
                + MU0_REF_H_PER_M * square_sum(&self.magnetic_h));
        let particle_energy = self
            .particles
            .iter()
            .map(|p| {
                let u_squared = norm_squared(p.proper_velocity_over_c);
                u_squared / ((1.0 + u_squared).sqrt() + 1.0)
            })
            .sum::<f64>()
            * (-self.sheet_charge / ELEMENTARY_CHARGE_C)
            * ELECTRON_MASS_KG
            * C0_M_PER_S.powi(2);
        field_energy + particle_energy
    }

    fn encode_emf(&self, dt_fs: u64, steps: usize) -> Result<Vec<u8>, String> {
        let n = self.grid.nx;
        let mut payload = vec![0; n * 128];
        for (i, cell) in payload.chunks_exact_mut(128).enumerate() {
            let record = YeeRecord {
                electric_field_v_per_m: std::array::from_fn(|d| self.electric[d][i]),
                magnetic_field_h_a_per_m: std::array::from_fn(|d| self.magnetic_h[d][i]),
                impressed_current_j_a_per_m2: std::array::from_fn(|d| self.current[d][i]),
                scalar_potential_v: 0.0,
                charge_density_c_per_m3: self.charge[i],
                epsilon_relative: 1.0,
                mu_relative: 1.0,
                conductivity_s_per_m: 0.0,
                material_entry: 0,
                constitutive_metadata_id: 6,
                state_record_index: u64::MAX,
            };
            let cell: &mut [u8; 128] =
                cell.try_into().map_err(|_| "Invalid PIC EMF cell stride")?;
            record
                .encode(true, cell)
                .map_err(|err| format!("PIC EMF cell {i}: {err:?}"))?;
        }
        let (mut a, mut b) = (FS_PER_SECOND, dt_fs);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        let clock = FieldClock {
            scale: 0,
            epoch_seconds: 0,
            epoch_nanoseconds: 0,
            ticks_per_second_num: FS_PER_SECOND / a,
            ticks_per_second_den: dt_fs / a,
        };
        let frame = FieldFrame {
            origin_m: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            grid: Some(GridSpec {
                dimensions: [n as u32, 1, 1],
                brick_dimensions: [n as u32, 1, 1],
                spacing_m: [self.grid.dx, self.grid.dy, self.grid.dz],
                indexing: GridIndexing::Linear,
            }),
        };
        let spec = ExtendedModeSpec::Yee {
            constitutive: ConstitutiveSpec {
                model: "vacuum",
                epsilon_infinity_relative: 1.0,
                mu_relative: 1.0,
                conductivity_s_per_m: 0.0,
                delta_epsilon: &[],
                relaxation_time_s: &[],
                resonance_rad_s: &[],
                damping_s1: &[],
            },
            time_offset_num: &[0, 0, 0, 1, 1, 1],
            time_offset_den: &[1, 1, 1, 2, 2, 2],
            material: None,
        };
        let provenance = FieldProvenance {
            role: 2,
            algorithm: Some("periodic-1d3v-boris-yee-pic"),
            algorithm_version: Some("1.0"),
            lineage: Some("uniform-ion-background-current-at-previous-half-step"),
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
            n as u64,
            &payload,
            &spec,
            &mut metadata,
            &mut staging,
            &mut output,
            &mut scratch,
            FieldWriteLimits::default(),
        )
        .map_err(|err| format!("PIC EMF envelope: {err:?}"))?
        .len();
        output.truncate(used);
        Ok(output)
    }
}

fn norm_squared(v: [f64; 3]) -> f64 {
    v.into_iter().map(|p| p * p).sum()
}

fn boris(u: [f64; 3], e: [f64; 3], b: [f64; 3], dt: f64) -> [f64; 3] {
    let half = -0.5 * ELEMENTARY_CHARGE_C * dt / ELECTRON_MASS_KG;
    let kick = e.map(|v| v * half / C0_M_PER_S);
    let minus = std::array::from_fn(|i| u[i] + kick[i]);
    let gamma = (1.0 + norm_squared(minus)).sqrt();
    let t = b.map(|v| v * half / gamma);
    let t_squared = norm_squared(t);
    let s = t.map(|v| 2.0 * v / (1.0 + t_squared));
    let dv = cross(minus, t);
    let turned = cross(std::array::from_fn(|i| minus[i] + dv[i]), s);
    std::array::from_fn(|i| minus[i] + turned[i] + kick[i])
}

fn linear_shape(position: f64, n: usize) -> (usize, usize, f64) {
    let cell = position.floor();
    let lower = (cell as isize).rem_euclid(n as isize) as usize;
    (lower, (lower + 1) % n, position - cell)
}

fn interpolate(field: &[f64], position: f64) -> f64 {
    let (i, next, fraction) = linear_shape(position, field.len());
    field[i] * (1.0 - fraction) + field[next] * fraction
}

fn deposit_nodes(field: &mut [f64], position: f64, value: f64) {
    let (i, next, fraction) = linear_shape(position, field.len());
    field[i] += value * (1.0 - fraction);
    field[next] += value * fraction;
}

fn deposit_current(j: &mut [Vec<f64>; 3], from: f64, to: f64, rate: f64, transverse: [f64; 2]) {
    // CFL bounds a path to less than one cell width. Split it at any node
    // crossing so the longitudinal face flux exactly matches the CIC charge change.
    let displacement = to - from;
    let (low, high) = (from.min(to), from.max(to));
    let boundary = (low.floor() + 1.0).min(high);
    let [jx, jy, jz] = j;
    for (start, end) in [(low, boundary), (boundary, high)] {
        let length = end - start;
        if length <= 0.0 {
            continue;
        }
        let midpoint = 0.5 * (start + end);
        let (cell, _, _) = linear_shape(midpoint, jx.len());
        jx[cell] += rate * length.copysign(displacement);
        let fraction = length / displacement.abs();
        deposit_nodes(jy, midpoint, transverse[0] * fraction);
        deposit_nodes(jz, midpoint, transverse[1] * fraction);
    }
    if displacement == 0.0 {
        deposit_nodes(jy, from, transverse[0]);
        deposit_nodes(jz, from, transverse[1]);
    }
}
