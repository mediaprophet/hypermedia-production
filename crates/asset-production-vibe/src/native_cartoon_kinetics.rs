//! Authored cartoon mechanics on native rigid animation and attributed surfaces.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::{Arc, atomic::Ordering};

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::{
    etalon_render::skeletal::{Key, MAX_JOINTS, MAX_KEYS, Motor, Track, valid_motor},
    specialized_libs::physics_simulation::cartoon_deformation::{
        CartoonKineticsConfig, compute_cartoon_jacobian_det, compute_comedic_aerodynamics,
        step_anharmonic_oscillator,
    },
};

use crate::{
    Args, AssetHost, TypedArtifact, Value,
    artifact::{AnimClip, AttributedMesh},
    count, fields, integer, list, num, number, record, required, vector,
};

const MAX_STEPS: usize = 256;

impl AssetHost<'_> {
    pub(crate) fn step_cartoon_kinetics(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "mesh",
                "motion_joint",
                "step_s",
                "steps",
                "mass_kg",
                "rest_height_m",
                "pivot_m",
                "config",
                "spring_forces_n",
                "initial_spring_displacement_m",
                "initial_spring_velocity_m_s",
                "root_velocity_m_s",
                "gravity_m_s2",
            ],
        )?;
        let steps = count(args, "steps", 1, MAX_STEPS)?;
        let step_s = num(required(args, "step_s")?)?;
        let mass = num(required(args, "mass_kg")?)?;
        let height = num(required(args, "rest_height_m")?)?;
        let pivot: [f64; 3] = vector(required(args, "pivot_m")?)?;
        if !(1e-4..=0.1).contains(&step_s) || mass <= 0.0 || height <= 0.0 {
            return Err("Cartoon kinetics requires step_s in [0.0001, 0.1] and positive mass_kg and rest_height_m".into());
        }
        let joint = args
            .get("motion_joint")
            .map(|value| integer(value, "motion_joint"))
            .transpose()?
            .unwrap_or(0);
        if joint >= MAX_JOINTS {
            return Err("Cartoon motion_joint exceeds the native skeletal bound".into());
        }
        let joint = joint as u16;
        let (parameters, aero) = config(args.get("config"))?;
        let forces = args.get("spring_forces_n").map(list).transpose()?;
        if forces.is_some_and(|series| series.len() != steps) {
            return Err("Cartoon spring_forces_n requires one sample per step".into());
        }
        let forces = match forces {
            Some(series) => series.iter().map(num).collect::<Result<Vec<_>, _>>()?,
            None => vec![0.0; steps],
        };
        let mut q = number(args, "initial_spring_displacement_m", 0.0)?;
        let mut v = number(args, "initial_spring_velocity_m_s", 0.0)?;
        let source_value = required(args, "source")?;
        let (mesh_value, clip) = match self.workspace.typed(source_value)? {
            TypedArtifact::AttributedMesh(_) if !args.contains_key("mesh") => (source_value, None),
            TypedArtifact::AnimationClip(clip)
                if !args.contains_key("root_velocity_m_s") && !args.contains_key("gravity_m_s2") =>
            {
                (required(args, "mesh")?, Some(Arc::clone(clip)))
            }
            _ => return Err("Cartoon kinetics requires an attributed mesh or an animation clip with its companion mesh".into()),
        };
        let mesh_artifact = self.workspace.typed(mesh_value)?;
        let TypedArtifact::AttributedMesh(mesh) = mesh_artifact else {
            return Err("Cartoon companion mesh must be a typed attributed mesh".into());
        };
        if clip
            .as_ref()
            .is_some_and(|clip| !clip.morph_keys.is_empty())
            && mesh.morph_targets.is_empty()
        {
            return Err("Cartoon source morph keys require their companion MRP1 basis".into());
        }
        let mesh = Arc::clone(mesh);
        let mesh_bytes = mesh_artifact.bytes(self.limits.resource_bytes)?;
        let output_duration = steps as f64 * step_s;
        let input_root = clip
            .as_deref()
            .map(|clip| root_track(clip, joint, output_duration, steps))
            .transpose()?;
        let duration = clip
            .as_ref()
            .map_or(output_duration, |clip| clip.duration_seconds);
        let dt = duration / steps as f64;
        let mut velocity = args
            .get("root_velocity_m_s")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0; 3]);
        let gravity = args
            .get("gravity_m_s2")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or(if clip.is_some() {
                [0.0; 3]
            } else {
                [0.0, 0.0, -9.81]
            });
        self.charge(
            mesh_bytes
                .saturating_add(mesh.surface.mesh.positions.len().saturating_mul(40))
                .saturating_add(steps * 4096),
            mesh_bytes.saturating_add((steps + 1) * 4096),
        )?;

        let mut root = Vec::with_capacity(steps + 1);
        let mut offset = [0.0; 3];
        let mut previous_position = [0.0; 3];
        let mut table = Workbook::new();
        let sheet_id = table.add_sheet("Cartoon kinetics");
        let sheet = table
            .sheet_mut(sheet_id)
            .ok_or("Could not create cartoon kinetics table")?;
        let columns = [
            "time_s",
            "transverse_scale",
            "axial_scale",
            "det_f",
            "spring_displacement_m",
            "spring_velocity_m_s",
            "aerodynamic_force_z_n",
        ];
        for (column, name) in columns.into_iter().enumerate() {
            sheet.set_value(CellRef::new(0, column as u32), CellValue::Text(name.into()));
        }
        let mut scale = [1.0; 3];
        for index in 0..=steps {
            if self.cancel.load(Ordering::Acquire) {
                return Err("Production cancelled".into());
            }
            let time = if index == steps {
                duration
            } else {
                index as f64 * dt
            };
            let (rotation, position) = input_root
                .map(|keys| sample_root(keys, time).to_rotation_translation())
                .unwrap_or(([1.0, 0.0, 0.0, 0.0], [0.0; 3]));
            let mut aerodynamic_z = 0.0;
            if index > 0 {
                let total_velocity = std::array::from_fn(|axis| {
                    velocity[axis] + (position[axis] - previous_position[axis]) / dt
                });
                let force = compute_comedic_aerodynamics(
                    total_velocity,
                    aero[0],
                    aero[1],
                    aero[2],
                    &parameters,
                );
                aerodynamic_z = force[2];
                for axis in 0..3 {
                    velocity[axis] += (gravity[axis] + force[axis] / mass) * dt;
                    offset[axis] += velocity[axis] * dt;
                }
                (q, v) = step_anharmonic_oscillator(q, v, mass, forces[index - 1], dt, &parameters);
            }
            previous_position = position;
            let (axial, determinant) = compute_cartoon_jacobian_det(1.0 + q / height, &parameters);
            let transverse = (determinant / axial).sqrt();
            scale = [transverse, transverse, axial];
            let local = Motor::from_rotation_translation(
                rotation,
                std::array::from_fn(|axis| position[axis] + offset[axis]),
            );
            let facts = [time, transverse, axial, determinant, q, v, aerodynamic_z];
            if !valid_motor(&local)
                || facts.iter().any(|value| !value.is_finite())
                || scale.iter().any(|value| *value <= 0.0)
            {
                return Err(format!("Cartoon kinetics diverged at sample {index}"));
            }
            root.push(Key { time, local });
            for (column, value) in facts.into_iter().enumerate() {
                sheet.set_value(
                    CellRef::new(index as u32 + 1, column as u32),
                    CellValue::Number(value),
                );
            }
        }
        let posed = deform_surface(&mesh, pivot, scale)?;
        let clip = styled_clip(clip.as_deref(), joint, root, duration);
        let mesh = self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(posed)))?;
        let clip = self.admit_artifact(TypedArtifact::AnimationClip(Arc::new(clip)))?;
        let samples = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(table)))?;
        Ok(Value::Record(Args::from([
            ("clip".into(), clip),
            ("mesh".into(), mesh),
            ("deformation_samples".into(), samples),
        ])))
    }
}

fn config(value: Option<&Value>) -> Result<(CartoonKineticsConfig, [f64; 3]), String> {
    let empty = Args::new();
    let fields_value = value.map(record).transpose()?.unwrap_or(&empty);
    fields(
        fields_value,
        &[
            "wafer_floor_fraction",
            "poisson_exponent",
            "linear_stiffness_n_m",
            "quadratic_stiffness_n_m2",
            "cubic_stiffness_n_m3",
            "damping_per_s",
            "apex_hang_force_n",
            "apex_velocity_m_s",
            "fluid_density_kg_m3",
            "drag_coefficient",
            "frontal_area_m2",
        ],
    )?;
    let defaults = CartoonKineticsConfig::default();
    let parameter = |key, default| number(fields_value, key, default);
    let result = CartoonKineticsConfig {
        wafer_floor_epsilon: parameter("wafer_floor_fraction", defaults.wafer_floor_epsilon)?,
        cartoon_poisson_nu: parameter("poisson_exponent", 0.5)?,
        linear_stiffness_k: parameter("linear_stiffness_n_m", defaults.linear_stiffness_k)?,
        quadratic_asymmetry_a: parameter(
            "quadratic_stiffness_n_m2",
            defaults.quadratic_asymmetry_a,
        )?,
        cubic_hardening_b: parameter("cubic_stiffness_n_m3", defaults.cubic_hardening_b)?,
        damping_gamma: parameter("damping_per_s", defaults.damping_gamma)?,
        comedic_apex_hang_force_n: parameter(
            "apex_hang_force_n",
            defaults.comedic_apex_hang_force_n,
        )?,
        apex_velocity_threshold_v0: parameter(
            "apex_velocity_m_s",
            defaults.apex_velocity_threshold_v0,
        )?,
    };
    let aero = [
        parameter("fluid_density_kg_m3", 0.0)?,
        parameter("drag_coefficient", 0.0)?,
        parameter("frontal_area_m2", 0.0)?,
    ];
    if result.wafer_floor_epsilon <= 0.0
        || result.wafer_floor_epsilon > 1.0
        || !(0.0..=0.5).contains(&result.cartoon_poisson_nu)
        || [
            result.linear_stiffness_k,
            result.quadratic_asymmetry_a,
            result.cubic_hardening_b,
            result.damping_gamma,
            result.comedic_apex_hang_force_n,
            result.apex_velocity_threshold_v0,
        ]
        .into_iter()
        .chain(aero)
        .any(|coefficient| coefficient < 0.0)
    {
        return Err(
            "Invalid cartoon wafer floor, Poisson exponent, or kinetic coefficients".into(),
        );
    }
    Ok((result, aero))
}

fn root_track(clip: &AnimClip, joint: u16, duration: f64, steps: usize) -> Result<&[Key], String> {
    if clip.looping
        || !clip.duration_seconds.is_finite()
        || (clip.duration_seconds - duration).abs() > 1e-9 * duration
        || clip.tracks.len() > MAX_JOINTS
        || clip.motor_keys.len() > MAX_KEYS
        || clip.morph_keys.len() > MAX_KEYS
    {
        return Err(
            "Cartoon animation must be bounded, nonlooping and cover exactly steps × step_s".into(),
        );
    }
    if clip.morph_keys.iter().any(|key| {
        !key.time_seconds.is_finite()
            || !(0.0..=clip.duration_seconds).contains(&key.time_seconds)
            || key.target_id == 0
            || !(0.0..=1.0).contains(&key.weight)
    }) {
        return Err("Invalid cartoon source morph key".into());
    }
    let mut seen = [false; MAX_JOINTS];
    let mut source_root = None;
    let mut keys_needed = steps + 1;
    for track in &clip.tracks {
        let slot = usize::from(track.joint);
        let first = track.first_key as usize;
        let end = first
            .checked_add(usize::from(track.key_count))
            .ok_or("Invalid cartoon source animation track span")?;
        let keys = clip
            .motor_keys
            .get(first..end)
            .ok_or("Invalid cartoon source animation track span")?;
        if slot >= MAX_JOINTS
            || seen[slot]
            || keys.is_empty()
            || keys.windows(2).any(|pair| pair[0].time >= pair[1].time)
            || keys.iter().any(|key| {
                !key.time.is_finite()
                    || !(0.0..=clip.duration_seconds).contains(&key.time)
                    || !valid_motor(&key.local)
            })
        {
            return Err("Invalid cartoon source animation track".into());
        }
        seen[slot] = true;
        if track.joint == joint {
            source_root = Some(keys);
        } else {
            keys_needed += keys.len();
        }
    }
    if keys_needed > MAX_KEYS {
        return Err("Cartoon animation exceeds its native key budget".into());
    }
    source_root.ok_or_else(|| "Cartoon motion_joint is absent from the authored clip".into())
}

fn sample_root(keys: &[Key], time: f64) -> Motor {
    let upper = keys.partition_point(|key| key.time <= time);
    if upper == 0 {
        return keys[0].local;
    }
    if upper == keys.len() {
        return keys[upper - 1].local;
    }
    let (a, b) = (keys[upper - 1], keys[upper]);
    Motor::sclerp(&a.local, &b.local, (time - a.time) / (b.time - a.time))
}

fn styled_clip(source: Option<&AnimClip>, joint: u16, root: Vec<Key>, duration: f64) -> AnimClip {
    let mut tracks = vec![Track {
        joint,
        first_key: 0,
        key_count: root.len() as u16,
    }];
    let mut motor_keys = root;
    let mut morph_keys = Vec::new();
    if let Some(source) = source {
        for track in source.tracks.iter().filter(|track| track.joint != joint) {
            let first = track.first_key as usize;
            tracks.push(Track {
                first_key: motor_keys.len() as u32,
                ..*track
            });
            motor_keys
                .extend_from_slice(&source.motor_keys[first..first + usize::from(track.key_count)]);
        }
        morph_keys.clone_from(&source.morph_keys);
    }
    AnimClip {
        duration_seconds: duration,
        looping: false,
        tracks,
        motor_keys,
        morph_keys,
    }
}

fn deform_surface(
    mesh: &AttributedMesh,
    pivot: [f64; 3],
    scale: [f64; 3],
) -> Result<AttributedMesh, String> {
    let geometry = &mesh.surface.mesh;
    let count = geometry.positions.len();
    if count == 0
        || geometry.triangles.is_empty()
        || geometry
            .positions
            .iter()
            .flatten()
            .any(|position| !position.is_finite())
        || geometry
            .triangles
            .iter()
            .flatten()
            .any(|&vertex| vertex as usize >= count)
        || mesh
            .surface
            .normals
            .as_ref()
            .is_some_and(|rows| rows.len() != count)
        || mesh
            .surface
            .tangents
            .as_ref()
            .is_some_and(|rows| rows.len() != count)
        || !mesh.protected_faces.is_empty()
            && mesh.protected_faces.len() != geometry.triangles.len()
    {
        return Err(
            "Cartoon deformation needs an attributed surface with valid vertex and face streams"
                .into(),
        );
    }
    if scale == [1.0; 3] {
        return Ok(mesh.clone());
    }
    if mesh.protected_faces.iter().any(|&flag| flag != 0) || !mesh.morph_targets.is_empty() {
        return Err("Cartoon deformation requires a surface without protected faces or an existing MRP1 basis".into());
    }
    let mut result = mesh.clone();
    let geometry = &mut result.surface.mesh;
    geometry.min = [f32::INFINITY; 3];
    geometry.max = [f32::NEG_INFINITY; 3];
    for (position, source) in geometry
        .positions
        .iter_mut()
        .zip(&mesh.surface.mesh.positions)
    {
        for axis in 0..3 {
            let base = f64::from(source[axis]);
            let x = (base + (base - pivot[axis]) * (scale[axis] - 1.0)) as f32;
            if !x.is_finite() {
                return Err(
                    "Cartoon deformation exceeded the native surface coordinate range".into(),
                );
            }
            position[axis] = x;
            geometry.min[axis] = geometry.min[axis].min(x);
            geometry.max[axis] = geometry.max[axis].max(x);
        }
    }
    if let Some(normals) = &mut result.surface.normals {
        for normal in normals {
            *normal = frame_direction(*normal, scale.map(f64::recip))?;
        }
    }
    if let Some(tangents) = &mut result.surface.tangents {
        for tangent in tangents {
            let [x, y, z] = frame_direction([tangent[0], tangent[1], tangent[2]], scale)?;
            if !tangent[3].is_finite() {
                return Err("Invalid cartoon tangent handedness".into());
            }
            *tangent = [x, y, z, tangent[3]];
        }
    }
    Ok(result)
}

fn frame_direction(direction: [f32; 3], scale: [f64; 3]) -> Result<[f32; 3], String> {
    let [x, y, z] = std::array::from_fn(|axis| f64::from(direction[axis]) * scale[axis]);
    let norm = x.hypot(y).hypot(z);
    if !norm.is_finite() || norm == 0.0 {
        return Err("Invalid cartoon surface frame".into());
    }
    Ok([(x / norm) as f32, (y / norm) as f32, (z / norm) as f32])
}
