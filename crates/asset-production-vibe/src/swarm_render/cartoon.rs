//! CPU-authored cel, rim, contour and optional UV halftone vertex colors.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use layout_nodegraph_compiler::{ConstructionArgs, NodeId, ProtoNetwork, ProtoNode, TaggedValue};
use surface_pbr_studio::Vec3;

use crate::{count, fields, number, required, vector, Args, AssetHost, TypedArtifact, Value};

const MATERIAL_X: &str = r#"<?xml version="1.0"?>
<materialx version="1.39" colorspace="lin_rec709">
  <geompropvalue name="cartoon_color" type="color3">
    <input name="geomprop" type="string" value="COLOR_0" />
  </geompropvalue>
  <surface_unlit name="cartoon_surface" type="surfaceshader">
    <input name="emission" type="float" value="1.0" />
    <input name="emission_color" type="color3" nodename="cartoon_color" />
  </surface_unlit>
  <surfacematerial name="cartoon_material" type="material">
    <input name="surfaceshader" type="surfaceshader" nodename="cartoon_surface" />
  </surfacematerial>
</materialx>
"#;

impl AssetHost<'_> {
    pub(crate) fn stylize_npr_cartoon(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mesh",
                "light_direction",
                "view_direction",
                "cel_bands",
                "ambient",
                "rim_width",
                "rim_color_linear",
                "contour_angle_deg",
                "contour_color_linear",
                "halftone_luma",
                "halftone_cycles",
            ],
        )?;
        let TypedArtifact::AttributedMesh(source) =
            self.workspace.typed(required(args, "mesh")?)?
        else {
            return Err("Cartoon authoring requires a retained AttributedMesh".into());
        };
        let source = Arc::clone(source);
        let geometry = &source.surface.mesh;
        let vertices = geometry.positions.len();
        let triangles = geometry.triangles.len();
        if !(3..=262_144).contains(&vertices) || !(1..=262_144).contains(&triangles) {
            return Err("Cartoon mesh exceeds the 262,144 vertex or triangle budget".into());
        }
        if geometry.positions.iter().flatten().any(|x| !x.is_finite()) {
            return Err("Cartoon mesh contains a non-finite vertex".into());
        }
        if !source.vertex_colors.is_empty() && source.vertex_colors.len() != vertices {
            return Err("Cartoon colors must match the mesh vertices".into());
        }
        let light = direction(args, "light_direction", [0.4, 0.7, 0.6])?;
        let view = direction(args, "view_direction", [0.0, 0.0, 1.0])?;
        let bands = count(args, "cel_bands", 4, 32)?;
        if bands < 2 {
            return Err("cel_bands must be in 2..=32".into());
        }
        let ambient = bounded(args, "ambient", 0.16, 0.0, 1.0)?;
        let rim_width = bounded(args, "rim_width", 0.15, 0.0, 1.0)?;
        let rim = color(args, "rim_color_linear", [0.85, 0.89, 1.0])?;
        let contour_angle = bounded(args, "contour_angle_deg", 0.0, 0.0, 60.0)?;
        let contour = color(args, "contour_color_linear", [0.02, 0.02, 0.025])?;
        let halftone_luma = bounded(args, "halftone_luma", 0.5, 0.0, 1.0)?;
        let halftone_cycles = bounded(args, "halftone_cycles", 0.0, 0.0, 128.0)?;
        let uv = source.surface.texture_coordinates_0.as_deref();
        if halftone_cycles > 0.0
            && !uv.is_some_and(|uv| {
                uv.len() == vertices
                    && uv
                        .iter()
                        .flatten()
                        .all(|x| x.is_finite() && x.abs() <= 10_000.0)
            })
        {
            return Err(
                "UV halftone requires one finite, bounded UV0 coordinate per vertex".into(),
            );
        }
        let copy_bytes = source
            .surface
            .texture_dependencies
            .iter()
            .try_fold(0_usize, |n, t| n.checked_add(t.bytes.len()))
            .ok_or("Cartoon texture dependency bytes overflow")?;
        let bytes = vertices * 128 + triangles * 24 + source.morph_targets.len() + copy_bytes;
        if bytes > self.limits.total_resource_bytes {
            return Err("Cartoon mesh copy exceeds the resource budget".into());
        }
        self.charge(vertices * 50 + triangles * 20, bytes)?;

        let mut weighted = vec![Vec3::ZERO; vertices];
        for triangle in &geometry.triangles {
            let slots = triangle.map(|i| i as usize);
            if slots.into_iter().any(|i| i >= vertices) {
                return Err("Cartoon mesh contains an invalid triangle index".into());
            }
            let [a, b, c] = slots.map(|i| Vec3::from_array(geometry.positions[i]));
            let n = (b - a).cross(c - a);
            if !n.to_array().into_iter().all(f32::is_finite) || n.length_squared() == 0.0 {
                return Err("Cartoon mesh contains a degenerate face".into());
            }
            for i in slots {
                weighted[i] += n;
            }
        }
        if source
            .surface
            .normals
            .as_ref()
            .is_some_and(|v| v.len() != vertices)
        {
            return Err("Authored normals do not match the cartoon mesh vertices".into());
        }
        let contour_threshold = contour_angle.to_radians().sin();
        let mut colors = Vec::with_capacity(vertices);
        for i in 0..vertices {
            let n = source
                .surface
                .normals
                .as_ref()
                .map_or(weighted[i], |v| Vec3::from_array(v[i]));
            let n = n.normalize();
            if n == Vec3::ZERO || !n.to_array().into_iter().all(f32::is_finite) {
                return Err("Cartoon mesh contains a vertex without a valid normal".into());
            }
            let input = source.vertex_colors.get(i).copied().unwrap_or([1.0; 4]);
            if input.iter().any(|x| !(0.0..=1.0).contains(x)) {
                return Err("Cartoon input colors must be finite linear RGBA in [0,1]".into());
            }
            let diffuse = n.dot(light).clamp(0.0, 1.0);
            let band =
                (diffuse * bands as f32).floor().min((bands - 1) as f32) / (bands - 1) as f32;
            let mut tone = ambient + (1.0 - ambient) * band;
            if halftone_cycles > 0.0 && tone < halftone_luma {
                let [x, y] = uv.ok_or("Missing validated UV0")?[i]
                    .map(|p| (p * halftone_cycles).rem_euclid(1.0) - 0.5);
                let radius = 0.45 * ((halftone_luma - tone) / halftone_luma).sqrt();
                if x * x + y * y < radius * radius {
                    tone *= 0.5;
                }
            }
            let facing = n.dot(view).abs().clamp(0.0, 1.0);
            let rim_mix = if rim_width > 0.0 {
                let t = (1.0 - facing / rim_width).clamp(0.0, 1.0);
                t * t * (3.0 - 2.0 * t)
            } else {
                0.0
            };
            let rgb = if facing < contour_threshold {
                contour
            } else {
                std::array::from_fn(|lane| {
                    input[lane] * tone * (1.0 - rim_mix) + rim[lane] * rim_mix
                })
            };
            colors.push([rgb[0], rgb[1], rgb[2], input[3]]);
        }

        let mut baked = (*source).clone();
        baked.vertex_colors = colors;
        let mut graph = ProtoNetwork::new(NodeId(2));
        graph.add_node(
            NodeId(1),
            ProtoNode {
                identifier: "ND_geompropvalue_color3".into(),
                construction_args: ConstructionArgs::Value(TaggedValue::String("COLOR_0".into())),
                input_args: vec![],
            },
        );
        graph.add_node(
            NodeId(2),
            ProtoNode {
                identifier: "ND_surface_unlit".into(),
                construction_args: ConstructionArgs::Nodes(vec![NodeId(1)]),
                input_args: vec![NodeId(1)],
            },
        );
        Ok(Value::Record(Args::from([
            (
                "mesh".into(),
                self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(baked)))?,
            ),
            (
                "material".into(),
                self.admit_artifact(TypedArtifact::MaterialGraph(Arc::new(graph)))?,
            ),
            (
                "materialx".into(),
                self.admit_encoded("mtlx", MATERIAL_X.as_bytes().to_vec())?,
            ),
        ])))
    }
}

fn bounded(args: &Args, name: &str, fallback: f64, min: f64, max: f64) -> Result<f32, String> {
    let value = number(args, name, fallback)?;
    if !(min..=max).contains(&value) {
        return Err(format!("{name} must be within {min}..={max}"));
    }
    Ok(value as f32)
}

fn color(args: &Args, name: &str, fallback: [f64; 3]) -> Result<[f32; 3], String> {
    let value = args
        .get(name)
        .map(vector::<3>)
        .transpose()?
        .unwrap_or(fallback);
    if value.iter().any(|x| !(0.0..=1.0).contains(x)) {
        return Err(format!("{name} must be finite linear RGB in [0,1]"));
    }
    Ok(value.map(|x| x as f32))
}

fn direction(args: &Args, name: &str, fallback: [f64; 3]) -> Result<Vec3, String> {
    let value = args
        .get(name)
        .map(vector::<3>)
        .transpose()?
        .unwrap_or(fallback);
    if value.iter().any(|x| !(-1.0..=1.0).contains(x)) {
        return Err(format!("{name} must be finite XYZ in [-1,1]"));
    }
    let v = Vec3::from_array(value.map(|x| x as f32)).normalize();
    if v == Vec3::ZERO {
        return Err(format!("{name} requires a nonzero vector"));
    }
    Ok(v)
}
