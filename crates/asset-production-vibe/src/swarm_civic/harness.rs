//! Tangent circular fillets and cut lengths for authored wire centerlines.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{bounded, numeric_row, table_receipt};
use crate::{Args, AssetHost, Value, count, fields, list, required, vector};

type Point = [f64; 3];

fn add(a: Point, b: Point) -> Point {
    std::array::from_fn(|i| a[i] + b[i])
}

fn sub(a: Point, b: Point) -> Point {
    std::array::from_fn(|i| a[i] - b[i])
}

fn scale(v: Point, s: f64) -> Point {
    v.map(|x| s * x)
}

fn norm(v: Point) -> f64 {
    v[0].hypot(v[1]).hypot(v[2])
}

fn dot(a: Point, b: Point) -> f64 {
    (0..3).map(|i| a[i] * b[i]).sum()
}

fn cross(a: Point, b: Point) -> Point {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[derive(Clone, Copy)]
struct Fillet {
    trim_m: f64,
    angle: f64,
    normal: Point,
}

impl AssetHost<'_> {
    pub(crate) fn route_pcb_and_wire_harness(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "anchor_points_m",
                "minimum_bend_radius_m",
                "bend_radius_m",
                "samples_per_bend",
                "termination_allowance_m",
            ],
        )?;
        let input = list(required(args, "anchor_points_m")?)?;
        if !(2..=128).contains(&input.len()) {
            return Err("Harness routing requires 2 to 128 ordered design anchors".into());
        }
        let subdivisions = count(args, "samples_per_bend", 12, 64)?;
        if subdivisions < 2 {
            return Err("Each sampled bend requires at least two subdivisions".into());
        }
        let point_budget = 2 + (input.len() - 2) * (subdivisions + 1);
        self.charge(point_budget * 64, point_budget * 4096)?;
        let minimum_radius = bounded(args, "minimum_bend_radius_m", None, 1e-5, 100.0)?;
        let radius = bounded(
            args,
            "bend_radius_m",
            Some(minimum_radius),
            minimum_radius,
            100.0,
        )?;
        let termination = bounded(args, "termination_allowance_m", Some(0.0), 0.0, 100.0)?;
        let points = input
            .iter()
            .map(|value| {
                let point = vector::<3>(value)?;
                if point.iter().any(|x| x.abs() > 1e4) {
                    return Err(
                        "Harness design anchor is beyond the admitted coordinate range".into(),
                    );
                }
                Ok(point)
            })
            .collect::<Result<Vec<_>, String>>()?;
        let strips = points
            .windows(2)
            .map(|pair| {
                let vector = sub(pair[1], pair[0]);
                let length = norm(vector);
                if length < 1e-7 {
                    return Err("Harness anchors must be distinct".into());
                }
                Ok((scale(vector, length.recip()), length))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut bends = vec![None; points.len()];
        for (i, segments) in strips.windows(2).enumerate() {
            let [a, b] = [segments[0].0, segments[1].0];
            let axis = cross(a, b);
            let sine = norm(axis);
            let cosine = dot(a, b);
            if sine <= 1e-12 && cosine > 0.0 {
                continue;
            }
            if sine <= 1e-12 {
                return Err(format!(
                    "Harness reverses at anchor {} without an admissible bend plane",
                    i + 1
                ));
            }
            let angle = sine.atan2(cosine);
            bends[i + 1] = Some(Fillet {
                trim_m: radius * (0.5 * angle).tan(),
                angle,
                normal: scale(axis, sine.recip()),
            });
        }
        let mut length = 0.0;
        for (i, &(_, whole)) in strips.iter().enumerate() {
            let trim = bends[i].map_or(0.0, |bend| bend.trim_m)
                + bends[i + 1].map_or(0.0, |bend| bend.trim_m);
            if trim >= whole {
                return Err(format!(
                    "Harness segment {i}: {:.9e} m of straight stock cannot fit {:.9e} m of minimum-radius tangencies",
                    whole, trim
                ));
            }
            length += whole - trim;
        }
        let mut rows = Vec::with_capacity(point_budget);
        let mut push = |[x, y, z]: [f64; 3], segment: usize, recorded_radius: f64| {
            rows.push(numeric_row([
                rows.len() as f64,
                segment as f64,
                x,
                y,
                z,
                recorded_radius,
            ]));
        };
        push(points[0], 0, 0.0);
        let mut bend_count = 0;
        for i in 1..points.len() - 1 {
            let Some(bend) = bends[i] else {
                push(points[i], i, 0.0);
                continue;
            };
            let incoming = strips[i - 1].0;
            let tangent = sub(points[i], scale(incoming, bend.trim_m));
            let radial = scale(cross(bend.normal, incoming), -radius);
            let center = sub(tangent, radial);
            let quadrature = cross(bend.normal, radial);
            length += radius * bend.angle;
            bend_count += 1;
            for sample in 0..=subdivisions {
                let theta = bend.angle * sample as f64 / subdivisions as f64;
                let (sine, cosine) = theta.sin_cos();
                push(
                    add(center, add(scale(radial, cosine), scale(quadrature, sine))),
                    i,
                    radius,
                );
            }
        }
        push(points[points.len() - 1], points.len() - 1, 0.0);
        // The last schedule row reports analytic length and allowance, avoiding
        // underestimation from sampled chord lengths.
        rows.push(numeric_row([
            -1.0,
            bend_count as f64,
            length,
            termination,
            length + 2.0 * termination,
            radius,
        ]));
        let mut out = table_receipt(
            self,
            "urn:qualia:civic:analytic-harness-fillets",
            &[
                "point_index_or_schedule_marker",
                "anchor_index_or_bend_count",
                "x_or_route_length_m",
                "y_or_termination_allowance_m",
                "z_or_cut_length_m",
                "bend_radius_m",
            ],
            &rows,
        )?;
        out.insert("route_length_m".into(), Value::F64(length));
        out.insert(
            "cut_length_m".into(),
            Value::F64(length + 2.0 * termination),
        );
        out.insert("bend_count".into(), Value::U64(bend_count as u64));
        Ok(Value::Record(out))
    }
}
