//! Deterministic spinning range simulation in a synthetic calibration room.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    artifact::FieldLattice, primal_replay::counter_uniform,
    qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, *,
};
use sha2::{Digest, Sha256};
use std::{
    f64::consts::{PI, TAU},
    sync::Arc,
};

struct Sphere {
    center: [f64; 3],
    radius: f64,
    reflectance: f64,
}
struct Pose {
    position: [f64; 3],
    yaw: f64,
    time: f64,
}
struct Hit {
    distance: f64,
    cosine: f64,
    reflectance: f64,
}

impl AssetHost<'_> {
    pub(crate) fn simulate_hardware_sensor_twin(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "mode",
                "room_half_extents_m",
                "wall_reflectance",
                "spheres",
                "poses",
                "azimuth_steps",
                "elevations_rad",
                "range_m",
                "sigma_m",
                "dropout_probability",
                "seed",
            ],
        )?;
        if text(required(a, "mode")?)? != "synthetic_calibration_lidar" {
            return Err(
                "Expected synthetic_calibration_lidar with analytic calibration geometry".into(),
            );
        }
        let room = vector::<3>(required(a, "room_half_extents_m")?)?;
        if room.iter().any(|v| !(0.5..=200.0).contains(v)) {
            return Err("Synthetic room extents must be between 0.5 and 200 m".into());
        }
        let wall = range(
            number(a, "wall_reflectance", 0.5)?,
            0.0,
            1.0,
            "wall_reflectance",
        )?;
        let objects = a.get("spheres").map(list).transpose()?.unwrap_or(&[]);
        if objects.len() > 64 {
            return Err("Synthetic calibration room admits at most 64 target spheres".into());
        }
        let spheres = objects
            .iter()
            .map(|v| {
                let row = record(v)?;
                fields(row, &["center_m", "radius_m", "reflectance"])?;
                let center = vector::<3>(required(row, "center_m")?)?;
                let radius = range(num(required(row, "radius_m")?)?, 0.01, 20.0, "radius_m")?;
                let reflectance =
                    range(num(required(row, "reflectance")?)?, 0.0, 1.0, "reflectance")?;
                if (0..3).any(|i| center[i].abs() + radius >= room[i]) {
                    return Err("Calibration sphere intersects a room wall".into());
                }
                Ok(Sphere {
                    center,
                    radius,
                    reflectance,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let pose_values = list(required(a, "poses")?)?;
        let steps = integer(required(a, "azimuth_steps")?, "azimuth_steps")?;
        let elevation_values = list(required(a, "elevations_rad")?)?;
        if !(2..=65).contains(&pose_values.len())
            || !(4..=2048).contains(&steps)
            || !(1..=32).contains(&elevation_values.len())
        {
            return Err("Unsupported trajectory, beam or azimuth count".into());
        }
        let elevation = elevation_values
            .iter()
            .map(|v| num(v).and_then(|n| range(n, -PI / 2.0, PI / 2.0, "beam elevation")))
            .collect::<Result<Vec<_>, _>>()?;
        if elevation.windows(2).any(|w| w[0] >= w[1]) {
            return Err("Beam elevations must increase strictly".into());
        }
        let distances = vector::<2>(required(a, "range_m")?)?;
        if distances[0] < 0.001 || distances[1] <= distances[0] || distances[1] > 500.0 {
            return Err("Invalid instrument minimum/maximum range".into());
        }
        let sigma = range(number(a, "sigma_m", 0.0)?, 0.0, 2.0, "sigma_m")?;
        let dropout = range(
            number(a, "dropout_probability", 0.0)?,
            0.0,
            1.0,
            "dropout_probability",
        )?;
        let seed = match required(a, "seed")? {
            Value::U64(v) => *v,
            Value::I64(v) if *v >= 0 => *v as u64,
            _ => return Err("Synthetic sensor seed requires an exact unsigned integer".into()),
        };
        let poses = pose_values
            .iter()
            .map(|v| {
                let row = record(v)?;
                fields(row, &["position_m", "yaw_rad", "time_s"])?;
                let position = vector::<3>(required(row, "position_m")?)?;
                if (0..3).any(|i| position[i].abs() >= room[i]) {
                    return Err("Instrument must stay inside the synthetic room".into());
                }
                let yaw = range(num(required(row, "yaw_rad")?)?, -PI, PI, "yaw_rad")?;
                let time = range(num(required(row, "time_s")?)?, 0.0, 60.0, "time_s")?;
                Ok(Pose {
                    position,
                    yaw,
                    time,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        for pair in poses.windows(2) {
            let dt = pair[1].time - pair[0].time;
            if !(0.01..=10.0).contains(&dt)
                || spheres.iter().any(|sphere| {
                    segment_distance_sq(sphere.center, pair[0].position, pair[1].position)
                        <= sphere.radius.powi(2)
                })
            {
                return Err(
                    "Trajectory has an invalid dwell or intersects synthetic geometry".into(),
                );
            }
        }
        let scans = poses.len() - 1;
        let rays = scans * steps * elevation.len();
        if rays > 262_144 {
            return Err("Synthetic range simulation exceeds 262,144 rays".into());
        }
        self.charge(rays * (80 + spheres.len() * 20), rays * 48)?;
        let mut samples = Vec::with_capacity(rays * 7);
        let mut points = Vec::with_capacity(rays * 3);
        for (scan, pair) in poses.windows(2).enumerate() {
            for pulse in 0..steps {
                let phase = pulse as f64 / steps as f64;
                let yaw_delta = (pair[1].yaw - pair[0].yaw + PI).rem_euclid(TAU) - PI;
                let yaw = pair[0].yaw + yaw_delta * phase + phase * TAU;
                let position = std::array::from_fn(|axis| {
                    pair[0].position[axis]
                        + (pair[1].position[axis] - pair[0].position[axis]) * phase
                });
                let time = pair[0].time + (pair[1].time - pair[0].time) * phase;
                for (beam, el) in elevation.iter().enumerate() {
                    let (sin_el, cos_el) = el.sin_cos();
                    let (sin_yaw, cos_yaw) = yaw.sin_cos();
                    let direction = [cos_el * cos_yaw, cos_el * sin_yaw, sin_el];
                    let id = ((scan * steps + pulse) * elevation.len() + beam) as u64;
                    let hit = nearest(position, direction, room, wall, &spheres);
                    let random = (-2.0 * counter_uniform(seed, id, 0).ln()).sqrt()
                        * (TAU * counter_uniform(seed, id, 1)).cos();
                    let measured = hit.distance + random * sigma;
                    let detected = (distances[0]..=distances[1]).contains(&hit.distance)
                        && (distances[0]..=distances[1]).contains(&measured)
                        && counter_uniform(seed, id, 2) >= dropout;
                    if detected {
                        let xyz = std::array::from_fn::<_, 3, _>(|axis| {
                            (position[axis] + measured * direction[axis]) as f32
                        });
                        let power = hit.reflectance * hit.cosine / (1.0 + hit.distance.powi(2));
                        samples.extend([
                            time as f32,
                            1.0,
                            measured as f32,
                            power as f32,
                            xyz[0],
                            xyz[1],
                            xyz[2],
                        ]);
                        points.extend(xyz);
                    } else {
                        samples.extend([time as f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
                    }
                }
            }
        }
        let hits = points.len() / 3;
        let stream = digest_stream(
            [scans as u32, steps as u32, elevation.len() as u32, 7],
            &samples,
        );
        let mut graph = ReceiptGraph::new("urn:qualia:sensing:synthetic-room-lidar:v1");
        graph.fact(
            &format!("urn:sha256:{stream}"),
            "urn:qualia:sensing:observationModel",
            "urn:qualia:sensing:synthetic-calibration-room-lidar",
        )?;
        let receipt = graph.seal(self.limits.resource_bytes)?;
        let observations = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![scans as u32, steps as u32, elevation.len() as u32, 7],
                samples,
            },
        )))?;
        let cloud = if hits == 0 {
            Value::Null
        } else {
            self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
                FieldLattice::DenseF32 {
                    axes: vec![hits as u32, 3],
                    samples: points,
                },
            )))?
        };
        Ok(Value::Record(Args::from([
            ("observations".into(), observations),
            ("point_cloud".into(), cloud),
            ("receipt".into(), self.encode("q42", receipt)?),
            ("observations_sha256".into(), Value::String(stream)),
            ("hits".into(), Value::U64(hits as u64)),
            ("rays".into(), Value::U64(rays as u64)),
            (
                "channels".into(),
                Value::List(
                    [
                        "time_s",
                        "detected",
                        "range_m",
                        "relative_power",
                        "x_m",
                        "y_m",
                        "z_m",
                    ]
                    .into_iter()
                    .map(|s| Value::String(s.into()))
                    .collect(),
                ),
            ),
        ])))
    }
}

fn range(value: f64, min: f64, max: f64, label: &str) -> Result<f64, String> {
    if (min..=max).contains(&value) {
        Ok(value)
    } else {
        Err(format!("{label} outside {min}..={max}"))
    }
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(x, y)| x * y).sum()
}

fn segment_distance_sq(point: [f64; 3], start: [f64; 3], end: [f64; 3]) -> f64 {
    let offset = std::array::from_fn(|i| point[i] - start[i]);
    let delta = std::array::from_fn(|i| end[i] - start[i]);
    let length = dot(delta, delta);
    let t = if length > 0.0 {
        (dot(offset, delta) / length).clamp(0.0, 1.0)
    } else {
        0.0
    };
    (0..3).map(|i| (offset[i] - t * delta[i]).powi(2)).sum()
}

fn nearest(
    position: [f64; 3],
    direction: [f64; 3],
    room: [f64; 3],
    wall: f64,
    spheres: &[Sphere],
) -> Hit {
    let mut hit = Hit {
        distance: f64::INFINITY,
        cosine: 0.0,
        reflectance: wall,
    };
    for i in 0..3 {
        let component = direction[i];
        if component != 0.0 {
            let exit = (room[i].copysign(component) - position[i]) / component;
            if exit < hit.distance {
                hit.distance = exit;
                hit.cosine = component.abs();
            }
        }
    }
    for sphere in spheres {
        let relative = std::array::from_fn(|i| position[i] - sphere.center[i]);
        let projection = dot(relative, direction);
        if projection >= 0.0 {
            continue;
        }
        let c = dot(relative, relative) - sphere.radius.powi(2);
        let discriminant = projection * projection - c;
        if discriminant >= 0.0 {
            let t = c / (-projection + discriminant.sqrt());
            if t > 0.0 && t < hit.distance {
                hit = Hit {
                    distance: t,
                    cosine: (discriminant.sqrt() / sphere.radius).clamp(0.0, 1.0),
                    reflectance: sphere.reflectance,
                };
            }
        }
    }
    hit
}

fn digest_stream(shape: [u32; 4], samples: &[f32]) -> String {
    let mut hash = Sha256::new();
    hash.update(b"qualia-synthetic-room-lidar-v1\0");
    for n in shape {
        hash.update(n.to_le_bytes());
    }
    for n in samples {
        hash.update(n.to_le_bytes());
    }
    format!("{:x}", hash.finalize())
}
