//! Typed profiles from retained planar DXF and native CAD constructions.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::*;
use drafting_geometry::{Arc, Circle, DxfEntity, Vec2, parse_dxf, region::boundary_at};

impl AssetHost<'_> {
    pub(crate) fn draft_cad_and_trace_vector(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        let options = record(required(args, "parameters")?)?;
        let mut profiles = Vec::new();
        let mut details = Args::new();
        match text(required(args, "op")?)? {
            "parse_dxf" => {
                fields(options, &["source", "metres_per_unit", "tolerance_m"])?;
                let handle = required(options, "source")?;
                let scale = num(required(options, "metres_per_unit")?)?;
                let tolerance = tolerance(options)?;
                if !(1e-9..=1e6).contains(&scale) {
                    return Err("DXF unit scale must be a positive metres-per-unit value".into());
                }
                let resource = encoded_resource(self, handle)?;
                if resource.kind != "dxf" || resource.bytes.len() > 512 * 1024 {
                    return Err("Expected a bounded retained UTF-8 DXF document".into());
                }
                let bytes = resource.bytes.len();
                self.charge(bytes.saturating_mul(64), bytes.saturating_mul(24))?;
                let resource = encoded_resource(self, handle)?;
                let document = std::str::from_utf8(&resource.bytes).map_err(|_| "DXF requires UTF-8")?;
                let document = document.strip_prefix('\u{feff}').unwrap_or(document);
                let drawing = parse_dxf(document).map_err(|error| error.to_string())?;
                if drawing.entities.len() > 256 {
                    return Err("DXF exceeds the 256 admitted entity budget".into());
                }
                let mut layers = Vec::new();
                let mut skipped = 0u64;
                for entity in drawing.entities {
                    let layer = entity.layer().to_owned();
                    let xy = |p: [f64; 3]| -> Result<Vec2, String> {
                        if p.iter().any(|v| !v.is_finite()) || (p[2] * scale).abs() > tolerance {
                            return Err("Drafting profile must lie in the projected XY plane".into());
                        }
                        let x = p[0] * scale;
                        let y = p[1] * scale;
                        if x.abs() > 1e8 || y.abs() > 1e8 {
                            return Err("DXF positions exceed the local coordinate range".into());
                        }
                        Ok(Vec2::new(x,y))
                    };
                    let mut points = Vec::new();
                    match entity {
                        DxfEntity::Line { start, end, .. } => points.extend([xy(start)?, xy(end)?]),
                        DxfEntity::Polyline { points: source, closed, .. } => {
                            points = source.into_iter().map(xy).collect::<Result<_, _>>()?;
                            if closed && points.first() != points.last() {
                                points.push(points[0]);
                            }
                        }
                        DxfEntity::Circle { center, radius, .. } => {
                            Circle::new(xy(center)?, checked_radius(radius * scale)?).tessellate(tolerance, &mut points);
                        }
                        DxfEntity::Arc { center, radius, start_angle, end_angle, .. } => {
                            let radius = checked_radius(radius * scale)?;
                            if !start_angle.is_finite() || !end_angle.is_finite() {
                                return Err("DXF contains a nonfinite arc angle".into());
                            }
                            Arc::new(xy(center)?, radius, start_angle, end_angle).tessellate(tolerance, &mut points);
                        }
                        DxfEntity::Point { .. } | DxfEntity::Text { .. } => {
                            skipped += 1;
                            continue;
                        }
                    }
                    let artifact = admit_profile(self, points)?;
                    profiles.push(artifact);
                    layers.push(Value::String(layer));
                }
                if profiles.is_empty() {
                    return Err("DXF provides no admitted planar profile entities".into());
                }
                details.insert("profile_layers".into(), Value::List(layers));
                details.insert("point_and_text_entities_skipped".into(), Value::U64(skipped));
            }
            "arc3" => {
                fields(options, &["start", "through", "end", "tolerance_m"])?;
                let start = xy_vector(required(options, "start")?)?;
                let through = xy_vector(required(options, "through")?)?;
                let end = xy_vector(required(options, "end")?)?;
                let arc = Arc::from_3_points(start, through, end).ok_or("Arc points are collinear or coincident")?;
                checked_radius(arc.radius)?;
                let reverse = arc.start_point().dist(start) > arc.end_point().dist(start);
                let mut points = Vec::new();
                arc.tessellate(tolerance(options)?, &mut points);
                if reverse {
                    points.reverse();
                }
                profiles.push(admit_profile(self, points)?);
                details.insert("radius_m".into(), Value::F64(arc.radius));
                details.insert("length_m".into(), Value::F64(arc.len()));
            }
            "common_tangents" => {
                fields(options, &["center_a", "radius_a_m", "center_b", "radius_b_m"])?;
                let a = Circle::new(xy_vector(required(options, "center_a")?)?, checked_radius(num(required(options, "radius_a_m")?)?)?);
                let b = Circle::new(xy_vector(required(options, "center_b")?)?, checked_radius(num(required(options, "radius_b_m")?)?)?);
                if a.center.dist(b.center) <= 1e-10 {
                    return Err("Common tangents between coincident circle centres are indeterminate".into());
                }
                for (a, b) in a.common_tangents(&b) {
                    profiles.push(admit_profile(self, vec![a,b])?);
                }
            }
            "region_at" => {
                fields(options, &["profiles", "probe_m", "tolerance_m"])?;
                let input = list(required(options, "profiles")?)?;
                if !(1..=128).contains(&input.len()) {
                    return Err("Region construction requires 1 to 128 profiles".into());
                }
                let mut segments = 0usize;
                let mut tracks = Vec::with_capacity(input.len());
                for profile in input {
                    let Artifact::Profile(points) = self.workspace.get(profile)? else {
                        return Err("Region construction needs existing profile handles".into());
                    };
                    segments = segments.saturating_add(points.len().saturating_sub(1));
                    if segments > 4096 {
                        return Err("Region arrangement exceeds the admitted segment count".into());
                    }
                    tracks.push(points.iter().map(|p| Vec2::new(p[0],p[1])).collect());
                }
                self.charge(segments * segments * 16, segments * segments * 4)?;
                let boundary = boundary_at(&tracks, xy_vector(required(options, "probe_m")?)?, tolerance(options)?)
                    .ok_or("No bounded region encloses the probe point")?;
                let area = drafting_geometry::shoelace(&boundary.outer).abs();
                let mut outer = boundary.outer;
                if outer.first() != outer.last() { outer.push(outer[0]); }
                profiles.push(admit_profile(self, outer)?);
                let mut sources = Vec::new();
                let mut holes_area = 0.0;
                for island in boundary.islands {
                    let mut outline = island.outline;
                    holes_area += drafting_geometry::shoelace(&outline).abs();
                    if outline.first() != outline.last() { outline.push(outline[0]); }
                    sources.push(Value::List(island.sources.into_iter().map(|n| Value::U64(n as u64)).collect()));
                    profiles.push(admit_profile(self, outline)?);
                }
                details.insert("area_m2".into(), Value::F64((area - holes_area).max(0.0)));
                details.insert("island_source_profile_indices".into(), Value::List(sources));
            }
            op => return Err(format!("Unknown native drafting operation {op}")),
        }
        details.insert("profiles".into(), Value::List(profiles));
        Ok(Value::Record(details))
    }
}

fn checked_radius(radius: f64) -> Result<f64, String> {
    (1e-9..=1e8).contains(&radius).then_some(radius).ok_or_else(|| "Drafting radius is out of range".into())
}

fn tolerance(args: &Args) -> Result<f64, String> {
    let tolerance = number(args, "tolerance_m", 0.0001)?;
    (1e-10..=1.0).contains(&tolerance).then_some(tolerance).ok_or_else(|| "Drafting tolerance is out of range".into())
}

fn xy_vector(input: &Value) -> Result<Vec2, String> {
    let [x, y] = vector(input)?;
    if x.abs() > 1e8 || y.abs() > 1e8 { return Err("Drafting point exceeds the local coordinate range".into()); }
    Ok(Vec2::new(x,y))
}

fn admit_profile(host: &mut AssetHost<'_>, points: Vec<Vec2>) -> Result<Value, String> {
    if points.len() < 2 || points.len() > 16_384 || points.iter().any(|p| !p.is_finite()) {
        return Err("Native drafting profile is empty, nonfinite or over budget".into());
    }
    host.charge(points.len() * 16, points.len() * 32)?;
    host.workspace.insert(Artifact::Profile(points.into_iter().map(|p| [p.x, p.y]).collect()), host.limits)
}
