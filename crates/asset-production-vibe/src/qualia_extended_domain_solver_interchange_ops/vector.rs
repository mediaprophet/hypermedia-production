//! Signed polynomial moments and bounded native profile offsetting.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{fields, list, num, number, required, text, Args, Artifact, AssetHost, Value};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use kurbo::{BezPath, ParamCurveArclen, PathEl, Point};
use std::collections::BTreeMap;
use vector_locus_geometry::PathData;
use vector_locus_pathops::pathops::{offset_path, Join};

const MAX_SEGMENTS: usize = 2048;
const MAX_OFFSET_POINTS: usize = 16_384;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Value, String> {
    match op {
        "signed_bezier_metrics" => signed_metrics(host, args),
        "offset_profiles" => offsets(host, args),
        _ => Err(format!("Unsupported vector operation: {op}")),
    }
}

fn profiles(host: &AssetHost<'_>, input: &Value) -> Result<BezPath, String> {
    let inputs = list(input)?;
    if inputs.is_empty() || inputs.len() > MAX_SEGMENTS / 3 {
        return Err("Expected a bounded list of closed profile handles".into());
    }
    let mut path = BezPath::new();
    let mut count = 0;
    for input in inputs {
        let Artifact::Profile(points) = host.workspace.get(input)? else {
            return Err("Expected a planar profile handle".into());
        };
        count += points.len();
        if points.len() < 3 || count > MAX_SEGMENTS {
            return Err("Profile vertex count exceeds the vector budget".into());
        }
        if points.iter().flatten().any(|coordinate| !coordinate.is_finite() || coordinate.abs() > 1e12) {
            return Err("Profile contains an invalid planar coordinate".into());
        }
        path.move_to(Point::new(points[0][0], points[0][1]));
        for &[x, y] in &points[1..] {
            path.line_to(Point::new(x, y));
        }
        path.close_path();
    }
    Ok(path)
}

fn admitted_path(host: &AssetHost<'_>, args: &Args) -> Result<BezPath, String> {
    match (args.get("profiles"), args.get("path_data")) {
        (Some(input), None) => profiles(host, input),
        (None, Some(input)) => {
            let input = text(input)?;
            if input.len() > 32 * 1024 || input.bytes().any(|byte| matches!(byte, b'a' | b'A')) {
                return Err("Provide a bounded line, quadratic, or cubic SVG path without elliptical arc commands".into());
            }
            BezPath::from_svg(input).map_err(|error| format!("Path data: {error:?}"))
        }
        _ => Err("Provide exactly one of profiles or path_data".into()),
    }
}

fn signed_metrics(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["profiles", "path_data", "subject_iri"])?;
    let path = admitted_path(host, args)?;
    host.charge(path.elements().len().saturating_mul(1024), MAX_SEGMENTS * 64)?;
    let [area, mx, my] = moments(&path)?;
    if area == 0.0 {
        return Err("Signed area cancels to zero, so the winding-area centroid is undefined".into());
    }
    let centroid = [mx / area, my / area];
    let arc_length = path.segments().map(|segment| segment.arclen(1e-7)).sum::<f64>();
    if centroid.iter().chain([&arc_length]).any(|value| !value.is_finite()) {
        return Err("Computed vector metrics exceed finite precision".into());
    }
    let subject = text(required(args, "subject_iri")?)?;
    let mut graph = ReceiptGraph::new("urn:qualia:vector:winding-moments:v1");
    for (predicate, value) in [
        ("signedArea", area), ("centroidX", centroid[0]), ("centroidY", centroid[1]), ("arcLength", arc_length),
    ] {
        graph.fact(subject, &format!("urn:qualia:vector:{predicate}"), &format!("urn:qualia:vector:double:{:016x}", value.to_bits()))?;
    }
    let receipt = graph.seal(host.limits.resource_bytes.min(1024 * 1024))?;
    Ok(Value::Record(BTreeMap::from([
        ("signed_area".into(), Value::F64(area)),
        ("signed_area_centroid".into(), Value::List(centroid.into_iter().map(Value::F64).collect())),
        ("arc_length".into(), Value::F64(arc_length)),
        ("receipt".into(), host.encode("q42", receipt)?),
    ])))
}

fn moments(path: &BezPath) -> Result<[f64; 3], String> {
    if path.elements().is_empty() || path.elements().len() > MAX_SEGMENTS {
        return Err("Path exceeds the analytic moment segment budget".into());
    }
    let origin = match path.elements()[0] {
        PathEl::MoveTo(point) => point,
        _ => return Err("First path element must move to the contour origin".into()),
    };
    let mut first = origin;
    let mut last = origin;
    let mut open = false;
    let mut sums = [0.0; 3];
    let accumulate = |sums: &mut [f64; 3], controls: &[Point]| -> Result<(), String> {
        if controls.iter().any(|p| !p.x.is_finite() || !p.y.is_finite() || p.x.abs() > 1e12 || p.y.abs() > 1e12) {
            return Err("Curve controls exceed the bounded coordinate domain".into());
        }
        let mut xs = [0.0; 4];
        let mut ys = [0.0; 4];
        for (index, point) in controls.iter().enumerate() {
            xs[index] = point.x - origin.x;
            ys[index] = point.y - origin.y;
        }
        let x = coefficients(&xs[..controls.len()]);
        let y = coefficients(&ys[..controls.len()]);
        for i in 0..4 {
            for j in 1..4 {
                sums[0] += (x[i] * y[j] - y[i] * x[j]) * j as f64 / (2 * (i + j)) as f64;
                for k in 0..4 {
                    let factor = j as f64 / (2 * (i + j + k)) as f64;
                    sums[1] += x[i] * x[k] * y[j] * factor;
                    sums[2] -= y[i] * y[k] * x[j] * factor;
                }
            }
        }
        Ok(())
    };
    for element in path.elements() {
        let end = match *element {
            PathEl::MoveTo(point) if !open => {
                first = point;
                open = true;
                point
            }
            PathEl::LineTo(point) if open => {
                accumulate(&mut sums, &[last, point])?;
                point
            }
            PathEl::QuadTo(control, point) if open => {
                accumulate(&mut sums, &[last, control, point])?;
                point
            }
            PathEl::CurveTo(a, b, point) if open => {
                accumulate(&mut sums, &[last, a, b, point])?;
                point
            }
            PathEl::ClosePath if open => {
                accumulate(&mut sums, &[last, first])?;
                open = false;
                first
            }
            _ => return Err("Analytic moments require explicitly closed contours".into()),
        };
        last = end;
    }
    if open {
        return Err("Analytic moments require explicitly closed contours".into());
    }
    sums[1] += sums[0] * origin.x;
    sums[2] += sums[0] * origin.y;
    Ok(sums)
}

fn coefficients(controls: &[f64]) -> [f64; 4] {
    match *controls {
        [a,b] => [a, b-a, 0.0, 0.0],
        [a,b,c] => [a, 2.0*(b-a), a-2.0*b+c, 0.0],
        [a,b,c,d] => [a, 3.0*(b-a), 3.0*(a-2.0*b+c), -a+3.0*b-3.0*c+d],
        _ => unreachable!("Only line, quadratic, and cubic controls are supplied"),
    }
}

fn offsets(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["profiles", "offset", "join", "miter_limit", "flatten_tolerance"])?;
    let path = profiles(host, required(args, "profiles")?)?;
    let distance = num(required(args, "offset")?)?;
    let tolerance = number(args, "flatten_tolerance", 0.02)?;
    let miter = number(args, "miter_limit", 4.0)?;
    let join = match args.get("join").map(text).transpose()?.unwrap_or("round") {
        "miter" => Join::Miter,
        "bevel" => Join::Bevel,
        "round" => Join::Round,
        _ => return Err("Unknown offset join style".into()),
    };
    if !(0.0001..=10.0).contains(&tolerance) || !(1.0..=64.0).contains(&miter) || distance.abs() > 10_000.0 {
        return Err("Offset distance, miter limit or flatten tolerance is outside the admitted bounds".into());
    }
    let segment_count = path.elements().len();
    host.charge(segment_count * segment_count * 4, MAX_OFFSET_POINTS * 128)?;
    if path.segments().any(|segment| segment.arclen(1e-6) > 100_000.0) {
        return Err("Offset contour extent exceeds the planar processing budget".into());
    }
    let offset = offset_path(&PathData::from_bezpath(&path), distance, join, miter).to_bezpath();
    let mut contours = Vec::new();
    let mut contour = Vec::new();
    let mut points = 0;
    kurbo::flatten(offset, tolerance, |element| {
        if points > MAX_OFFSET_POINTS { return; }
        match element {
            PathEl::MoveTo(point) | PathEl::LineTo(point) => {
                contour.push([point.x, point.y]);
                points += 1;
            }
            PathEl::ClosePath => {
                if contour.len() > 1 && contour.first() == contour.last() {
                    contour.pop();
                }
                if contour.len() >= 3 {
                    contours.push(std::mem::take(&mut contour));
                } else {
                    contour.clear();
                }
            }
            _ => {}
        }
    });
    if points > MAX_OFFSET_POINTS || !contour.is_empty() {
        return Err("Offset output exceeds the closed profile budget".into());
    }
    let output = contours.into_iter().map(|contour| {
        host.workspace.insert(Artifact::Profile(contour), host.limits)
    }).collect::<Result<Vec<_>, _>>()?;
    Ok(Value::List(output))
}
