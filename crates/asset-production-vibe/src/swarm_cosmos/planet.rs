//! Surface authoring for cubed-sphere planetary bodies and annular rings.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{count, names, numeric_list, scalar, unit};
use crate::{
    artifact::{AttributedMesh, FieldLattice},
    fields, list, num, record, required, text, vector, Args, AssetHost, TypedArtifact, Value,
};
use qualia_core_db::{
    bundle::semantic_model::SemanticModelBuilder,
    container_10d::{MaterialRecord, OpacityMode, SubmeshRange},
    q_hash,
    render::{
        assets::{ImportedMesh, Mesh},
        compile_10d::upgrade_legacy_10d_to_holographic_v2,
        material_compile::compile_mesh_to_10d_with_materials,
    },
};
use std::{collections::BTreeMap, sync::Arc};
use terrain_erosion_solvers::crater_morphology::{apply_crater, Crater};

const MAX_CRATERS: usize = 128;
const MAX_ZONAL_TERMS: usize = 16;
const MAX_DEPTH_SAMPLES: usize = 8192;
const PROFILE_RADIUS_SAMPLES: usize = 64;
const PROFILE_CENTER: usize = 80;
const PROFILE_SIZE: usize = 2 * PROFILE_CENTER + 1;
const PLANETARY_DOMAIN: u64 = q_hash("urn:qualia:domain:planetary-science");

struct PlanetSurface {
    axes_m: [f64; 3],
    zonal_terms_m: Vec<f64>,
    craters: Vec<AngularCrater>,
}

struct AngularCrater {
    direction: [f64; 3],
    radius_rad: f64,
    cutoff: f64,
    depth_m: f64,
}

struct ResonanceGap {
    radius_m: f64,
    sigma_m: f64,
    opacity_reduction: f64,
}

impl AssetHost<'_> {
    pub(crate) fn synth_planetary_body(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "entity_iri",
                "semi_axes_m",
                "face_subdivisions",
                "zonal_height_terms_m",
                "craters",
                "surface_albedo_rgb",
            ],
        )?;
        let entity = entity_iri(args)?;
        let surface = PlanetSurface::from_args(args)?;
        let divisions = count(args, "face_subdivisions", 32, 4, 96)?;
        let vertices = 6 * divisions * divisions + 2;
        let face_stride = (divisions + 1) * (divisions + 1);
        self.charge(
            vertices * (64 + surface.zonal_terms_m.len() * 8 + surface.craters.len() * 56),
            vertices * 704 + face_stride * 24,
        )?;
        let profile = radial_crater_profile();
        let mut positions = Vec::with_capacity(vertices);
        let mut heights = Vec::with_capacity(vertices);
        let mut triangles = Vec::with_capacity(divisions * divisions * 12);
        let mut field = Vec::with_capacity(6 * face_stride);
        let mut lattice = BTreeMap::new();
        let span = divisions as i32;
        for face in 0..6 {
            self.charge(0, 0)?;
            let mut indices = Vec::with_capacity(face_stride);
            for j in 0..=divisions {
                for i in 0..=divisions {
                    let cube = face_coords(face, span, 2 * i as i32 - span, 2 * j as i32 - span);
                    let index = if let Some(&index) = lattice.get(&cube) {
                        index
                    } else {
                        let direction = unit(cube.map(f64::from), "planet sample")?;
                        let altitude = surface.radial_height(direction, &profile)?;
                        let reference_radius = direction
                            .into_iter()
                            .zip(surface.axes_m)
                            .map(|(component, axis)| (component / axis).powi(2))
                            .sum::<f64>()
                            .sqrt()
                            .recip();
                        let position = direction
                            .map(|component| ((reference_radius + altitude) * component) as f32);
                        let index = positions.len() as u32;
                        lattice.insert(cube, index);
                        positions.push(position);
                        heights.push(altitude as f32);
                        index
                    };
                    indices.push(index);
                    field.push(heights[index as usize]);
                }
            }
            for j in 0..divisions {
                for i in 0..divisions {
                    let a = j * (divisions + 1) + i;
                    let next = a + divisions + 1;
                    quad(
                        &mut triangles,
                        [indices[a], indices[a + 1], indices[next + 1], indices[next]],
                    );
                }
            }
        }
        let mesh = make_mesh(positions, triangles);
        let mut material = MaterialRecord::legacy_default();
        let albedo = color(args, "surface_albedo_rgb", [0.37, 0.30, 0.23])?;
        material.base_color = [albedo[0], albedo[1], albedo[2], 1.0];
        let side = divisions as u32 + 1;
        let (mesh, asset) = self.seal_cosmic_surface(mesh, entity, material)?;
        let radial_height = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![6, side, side],
                samples: field,
            },
        )))?;
        Ok(Value::Record(BTreeMap::from([
            ("mesh".into(), mesh),
            ("asset".into(), asset),
            ("radial_height".into(), radial_height),
            (
                "face_order".into(),
                names(["+x", "-x", "+y", "-y", "+z", "-z"]),
            ),
            ("semi_axes_m".into(), numeric_list(surface.axes_m)),
            ("radial_height_units".into(), Value::String("m".into())),
            (
                "crater_count".into(),
                Value::U64(surface.craters.len() as u64),
            ),
        ])))
    }

    pub(crate) fn synth_planetary_ring(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "entity_iri",
                "inner_radius_m",
                "outer_radius_m",
                "sheet_thickness_m",
                "segments",
                "optical_depth_profile",
                "radial_samples",
                "resonance_gaps",
                "albedo_rgb",
                "phase_g",
                "scattering_angle_degrees",
            ],
        )?;
        let entity = entity_iri(args)?;
        let inner = scalar(args, "inner_radius_m", None, 1.0, 9.999e8)?;
        let outer = scalar(args, "outer_radius_m", None, inner, 1e9)?;
        if (outer - inner) / outer <= 1e-6 {
            return Err("The annular width must exceed one millionth of its outer radius".into());
        }
        let thickness = scalar(args, "sheet_thickness_m", None, 1e-4, outer * 0.05)?;
        let segments = count(args, "segments", 256, 12, 4096)?;
        let sample_count = count(args, "radial_samples", 512, 2, MAX_DEPTH_SAMPLES)?;
        let profile = list(required(args, "optical_depth_profile")?)?;
        if !(2..=MAX_DEPTH_SAMPLES).contains(&profile.len()) {
            return Err("Optical-depth profile requires 2 to 8192 equally spaced entries".into());
        }
        let mut depth_knots = Vec::with_capacity(profile.len());
        for value in profile {
            let value = num(value)?;
            if !(0.0..=100.0).contains(&value) {
                return Err("Optical depth must be in [0, 100]".into());
            }
            depth_knots.push(value);
        }
        let g = scalar(args, "phase_g", Some(0.0), -0.98, 0.98)?;
        let scattering_angle =
            scalar(args, "scattering_angle_degrees", Some(90.0), 0.0, 180.0)?.to_radians();
        let phase = (1.0 - g * g)
            / (4.0
                * std::f64::consts::PI
                * (1.0 + g * g - 2.0 * g * scattering_angle.cos()).powf(1.5));
        let albedo = color(args, "albedo_rgb", [0.73, 0.69, 0.62])?;
        let gaps = resonance_gaps(args, inner, outer)?;
        self.charge(
            segments * 2100 + sample_count * (128 + gaps.len() * 64),
            segments * 6800 + sample_count * 48,
        )?;
        let radial_step = (outer - inner) / (sample_count as f64 - 1.0);
        let mut samples = vec![0.0; sample_count * 2];
        let mut opacity_sum = 0.0;
        for i in 0..sample_count {
            let fraction = i as f64 / (sample_count as f64 - 1.0);
            let radius = inner + radial_step * i as f64;
            let position = fraction * (depth_knots.len() - 1) as f64;
            let index = (position.floor() as usize).min(depth_knots.len() - 2);
            let mix = position - index as f64;
            let mut tau = depth_knots[index] * (1.0 - mix) + depth_knots[index + 1] * mix;
            for gap in &gaps {
                let offset = (radius - gap.radius_m) / gap.sigma_m;
                tau *= 1.0 - gap.opacity_reduction * (-0.5 * offset * offset).exp();
            }
            let transmittance = (-tau).exp();
            samples[i] = tau as f32;
            samples[sample_count + i] = transmittance as f32;
            opacity_sum += 1.0 - transmittance;
        }
        let mut material = MaterialRecord::legacy_default();
        material.opacity_mode = OpacityMode::Blend;
        material.base_color = [
            albedo[0],
            albedo[1],
            albedo[2],
            (opacity_sum / sample_count as f64) as f32,
        ];
        let mesh = annulus_mesh(inner, outer, thickness, segments);
        let (mesh, asset) = self.seal_cosmic_surface(mesh, entity, material)?;
        let optics = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![2, sample_count as u32],
                samples,
            },
        )))?;
        Ok(Value::Record(BTreeMap::from([
            ("mesh".into(), mesh),
            ("asset".into(), asset),
            ("radial_optics".into(), optics),
            (
                "optics_channels".into(),
                names(["normal_optical_depth", "normal_transmittance"]),
            ),
            ("radial_origin_m".into(), Value::F64(inner)),
            ("radial_step_m".into(), Value::F64(radial_step)),
            ("hg_phase_per_sr".into(), Value::F64(phase)),
            (
                "resonance_radii_m".into(),
                numeric_list(gaps.iter().map(|g| g.radius_m)),
            ),
        ])))
    }

    fn seal_cosmic_surface(
        &mut self,
        mesh: Mesh,
        entity: &str,
        material: MaterialRecord,
    ) -> Result<(Value, Value), String> {
        let range = SubmeshRange {
            first_index: 0,
            index_count: (mesh.triangles.len() * 3) as u32,
            material_id: material.id,
            semantic_id: q_hash(entity),
        };
        let compiled = compile_mesh_to_10d_with_materials(&mesh, &[material], &[range])
            .map_err(|err| format!("Cosmic surface material sealing: {err:?}"))?;
        let [x, y, z] = mesh.centroid();
        let sealed = upgrade_legacy_10d_to_holographic_v2(
            &compiled,
            entity,
            PLANETARY_DOMAIN,
            [0.0, 3.0, 2.0, x, y, z, 0.0, 1.0, 1.0, 0.5],
        )
        .map_err(|err| format!("Cosmic surface HOLO sealing: {err:?}"))?;
        SemanticModelBuilder::new(4 << 20)
            .merge_10d(&sealed)
            .map_err(|err| err.to_string())?;
        let artifact = AttributedMesh {
            surface: ImportedMesh {
                mesh,
                normals: None,
                tangents: None,
                texture_coordinates_0: None,
                materials: vec![material],
                texture_dependencies: Vec::new(),
                submeshes: vec![range],
            },
            uv1: Vec::new(),
            vertex_colors: Vec::new(),
            skin_weights: Vec::new(),
            morph_targets: Arc::from(Vec::new()),
            protected_faces: Vec::new(),
        };
        let mesh = self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(artifact)))?;
        let asset = self.admit_encoded("10d", sealed)?;
        Ok((mesh, asset))
    }
}

impl PlanetSurface {
    fn from_args(args: &Args) -> Result<Self, String> {
        let axes_m = vector::<3>(required(args, "semi_axes_m")?)?;
        let small_axis = axes_m.into_iter().fold(f64::INFINITY, f64::min);
        let large_axis = axes_m.into_iter().fold(0.0, f64::max);
        if small_axis < 1.0 || large_axis > 1e9 || large_axis / small_axis > 10.0 {
            return Err(
                "Planetary semi-axes require [1, 1e9] metres and aspect ratio at most 10".into(),
            );
        }
        let zonal_terms = args
            .get("zonal_height_terms_m")
            .map(list)
            .transpose()?
            .unwrap_or_default();
        if zonal_terms.len() > MAX_ZONAL_TERMS {
            return Err("At most 16 zonal Legendre height terms may be supplied".into());
        }
        let zonal_terms_m = zonal_terms.iter().map(num).collect::<Result<Vec<_>, _>>()?;
        if zonal_terms_m.iter().map(|v| v.abs()).sum::<f64>() > small_axis * 0.1 {
            return Err(
                "Zonal height amplitude exceeds ten percent of the shortest semi-axis".into(),
            );
        }
        let entries = args
            .get("craters")
            .map(list)
            .transpose()?
            .unwrap_or_default();
        if entries.len() > MAX_CRATERS {
            return Err(format!(
                "At most {MAX_CRATERS} authored craters may be supplied"
            ));
        }
        let mut craters = Vec::with_capacity(entries.len());
        for value in entries {
            let crater = record(value)?;
            fields(crater, &["direction", "angular_radius_degrees", "depth_m"])?;
            let radius_rad =
                scalar(crater, "angular_radius_degrees", None, 0.1, 45.0)?.to_radians();
            craters.push(AngularCrater {
                direction: unit(
                    vector::<3>(required(crater, "direction")?)?,
                    "crater direction",
                )?,
                radius_rad,
                depth_m: scalar(crater, "depth_m", None, 0.0, small_axis * 0.1)?,
                cutoff: (1.25 * radius_rad).cos(),
            });
        }
        Ok(Self {
            axes_m,
            zonal_terms_m,
            craters,
        })
    }

    fn radial_height(
        &self,
        direction: [f64; 3],
        profile: &[f32; PROFILE_SIZE],
    ) -> Result<f64, String> {
        let latitude_sine = direction[1];
        let mut previous = 0.0;
        let mut legendre = 1.0;
        let mut altitude = 0.0;
        for (index, amplitude) in self.zonal_terms_m.iter().enumerate() {
            let n = (index + 1) as f64;
            let next = ((2.0 * n - 1.0) * latitude_sine * legendre - (n - 1.0) * previous) / n;
            altitude += amplitude * next;
            previous = legendre;
            legendre = next;
        }
        for crater in &self.craters {
            let cosine = direction
                .into_iter()
                .zip(crater.direction)
                .map(|(a, b)| a * b)
                .sum::<f64>();
            if cosine > crater.cutoff {
                let distance = cosine.min(1.0).acos();
                let p = ((distance / crater.radius_rad) * PROFILE_RADIUS_SAMPLES as f64)
                    .min(PROFILE_CENTER as f64);
                let i = (p as usize).min(PROFILE_CENTER - 1);
                let f = p - i as f64;
                let normalized = (1.0 - f) * f64::from(profile[PROFILE_CENTER + i])
                    + f * f64::from(profile[PROFILE_CENTER + i + 1]);
                altitude += normalized * crater.depth_m;
            }
        }
        let shortest = self.axes_m.into_iter().fold(f64::INFINITY, f64::min);
        if altitude.abs() > 0.2 * shortest {
            return Err(
                "Combined planetary relief exceeds twenty percent of the shortest semi-axis".into(),
            );
        }
        Ok(altitude)
    }
}

fn radial_crater_profile() -> [f32; PROFILE_SIZE] {
    let mut row = [0.0; PROFILE_SIZE];
    let crater = Crater {
        x: PROFILE_CENTER as f32,
        y: 0.0,
        radius: PROFILE_RADIUS_SAMPLES as f32,
        depth: 1.0,
    };
    apply_crater(&mut row, PROFILE_SIZE, 1, &crater, 0.35, 0.20);
    row
}

fn resonance_gaps(args: &Args, inner: f64, outer: f64) -> Result<Vec<ResonanceGap>, String> {
    let mut gaps = Vec::new();
    let values = args
        .get("resonance_gaps")
        .map(list)
        .transpose()?
        .unwrap_or_default();
    if values.len() > 64 {
        return Err("Planetary rings support at most 64 authored resonance gaps".into());
    }
    for value in values {
        let gap = record(value)?;
        fields(
            gap,
            &[
                "moon_semi_major_axis_m",
                "azimuthal_order",
                "side",
                "sigma_m",
                "opacity_reduction",
            ],
        )?;
        let moon = scalar(gap, "moon_semi_major_axis_m", None, 1.0, 2e9)?;
        let order = count(gap, "azimuthal_order", 2, 2, 1024)? as f64;
        let sign = match text(required(gap, "side")?)? {
            "inner" => -1.0,
            "outer" => 1.0,
            _ => return Err("Resonance side must be inner or outer".into()),
        };
        let radius_m = moon * ((order + sign) / order).powf(2.0 / 3.0);
        if !(inner..=outer).contains(&radius_m) {
            return Err("An authored Keplerian resonance falls outside the annulus".into());
        }
        gaps.push(ResonanceGap {
            radius_m,
            sigma_m: scalar(gap, "sigma_m", None, 1e-6, outer - inner)?,
            opacity_reduction: scalar(gap, "opacity_reduction", Some(1.0), 0.0, 1.0)?,
        });
    }
    Ok(gaps)
}

fn face_coords(face: usize, span: i32, u: i32, v: i32) -> [i32; 3] {
    match face {
        0 => [span, u, v],
        1 => [-span, -u, v],
        2 => [v, span, u],
        3 => [v, -span, -u],
        4 => [u, v, span],
        _ => [-u, v, -span],
    }
}

fn annulus_mesh(inner: f64, outer: f64, thickness: f64, count: usize) -> Mesh {
    let mut positions = Vec::with_capacity(4 * count);
    let mut triangles = Vec::with_capacity(8 * count);
    for i in 0..count {
        let theta = (i as f64 / count as f64) * std::f64::consts::TAU;
        let (sin, cos) = theta.sin_cos();
        for height in [0.5 * thickness, -0.5 * thickness] {
            for radius in [inner, outer] {
                positions.push([(radius * cos) as f32, height as f32, (radius * sin) as f32]);
            }
        }
    }
    for i in 0..count {
        let a = (i * 4) as u32;
        let b = (((i + 1) % count) * 4) as u32;
        quad(&mut triangles, [a, b, b + 1, a + 1]);
        quad(&mut triangles, [a + 2, a + 3, b + 3, b + 2]);
        quad(&mut triangles, [a + 1, b + 1, b + 3, a + 3]);
        quad(&mut triangles, [a, a + 2, b + 2, b]);
    }
    make_mesh(positions, triangles)
}

fn make_mesh(positions: Vec<[f32; 3]>, triangles: Vec<[u32; 3]>) -> Mesh {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for position in &positions {
        for axis in 0..3 {
            min[axis] = min[axis].min(position[axis]);
            max[axis] = max[axis].max(position[axis]);
        }
    }
    Mesh {
        positions,
        triangles,
        min,
        max,
    }
}

fn quad(triangles: &mut Vec<[u32; 3]>, [a, b, c, d]: [u32; 4]) {
    triangles.extend([[a, b, c], [a, c, d]]);
}

fn entity_iri(args: &Args) -> Result<&str, String> {
    let iri = text(required(args, "entity_iri")?)?;
    if iri.len() > 2048
        || iri.chars().any(|ch| ch.is_control() || ch.is_whitespace())
        || !iri.split_once(':').is_some_and(|(scheme, tail)| {
            let mut chars = scheme.chars();
            chars
                .next()
                .is_some_and(|first| first.is_ascii_alphabetic())
                && chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '+' | '-' | '.'))
                && !tail.is_empty()
        })
        || q_hash(iri) == 0
    {
        return Err("entity_iri must be a bounded absolute RDF IRI".into());
    }
    Ok(iri)
}

fn color(args: &Args, key: &str, default: [f32; 3]) -> Result<[f32; 3], String> {
    let Some(value) = args.get(key) else {
        return Ok(default);
    };
    let color = vector::<3>(value)?;
    if color.iter().any(|lane| !(0.0..=1.0).contains(lane)) {
        return Err(format!("{key} channels must be in [0, 1]"));
    }
    Ok(color.map(|lane| lane as f32))
}
