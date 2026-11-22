//! Civilian water rendering using direct spectral refraction and bathymetry hits.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use qualia_core_db::etalon_render::water_surface::DielectricVolumeIop;
use surface_pbr_studio::{Bvh, Mesh, Ray, Vec3};

use crate::artifact::{FieldLattice, LatticeSamples, RadianceLattice};
use crate::primal_replay::counter_uniform;
use crate::{
    count, fields, number, record, required, vector, Args, AssetHost, TypedArtifact, Value,
};

struct Receiver {
    width: usize,
    height: usize,
    origin: [f32; 2],
    cell: [f32; 2],
    mesh: Mesh,
}

impl AssetHost<'_> {
    pub(crate) fn bake_refractive_caustics(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "water_mesh",
                "bathymetry",
                "bed_origin_xz_m",
                "bed_spacing_xz_m",
                "sun_direction",
                "incident_irradiance_rgb",
                "photon_budget",
                "seed",
                "white_irradiance",
                "iop",
            ],
        )?;
        let TypedArtifact::AttributedMesh(water) =
            self.workspace.typed(required(args, "water_mesh")?)?
        else {
            return Err("Caustics requires a retained AttributedMesh water surface".into());
        };
        let water = Arc::clone(water);
        let TypedArtifact::FieldLattice(bed) =
            self.workspace.typed(required(args, "bathymetry")?)?
        else {
            return Err("Caustics requires a retained DenseF32 bathymetry lattice".into());
        };
        let bed = Arc::clone(bed);
        let FieldLattice::DenseF32 { axes, samples } = bed.as_ref() else {
            return Err("Caustics bathymetry requires DenseF32 elevations".into());
        };
        let [height, width] = axes.as_slice() else {
            return Err("Bathymetry axes must be [height,width] X-fastest".into());
        };
        let (w, h) = (*width as usize, *height as usize);
        if !(2..=256).contains(&w)
            || !(2..=256).contains(&h)
            || samples.len() != w * h
            || samples.iter().any(|v| !(-10_000.0..=10_000.0).contains(v))
        {
            return Err(
                "Bathymetry needs a complete finite 2..=256 elevation grid in metres".into(),
            );
        }
        let origin = vector::<2>(required(args, "bed_origin_xz_m")?)?;
        let spacing = vector::<2>(required(args, "bed_spacing_xz_m")?)?;
        if origin.iter().any(|v| !(-10_000.0..=10_000.0).contains(v))
            || spacing.iter().any(|v| !(0.002..=100.0).contains(v))
        {
            return Err(
                "Caustics bed coordinates or spacing exceed the admitted metric frame".into(),
            );
        }
        let sun = vector::<3>(required(args, "sun_direction")?)?;
        if sun.iter().any(|v| !(-1.0..=1.0).contains(v)) {
            return Err("Incident direction must be finite XYZ within [-1,1]".into());
        }
        let incident = Vec3::from_array(sun.map(|v| v as f32)).normalize();
        if incident.y > -0.02 {
            return Err("Caustics requires downhill incident rays in Y-up coordinates".into());
        }
        let intensity = args
            .get("incident_irradiance_rgb")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([1.0; 3]);
        let white = number(args, "white_irradiance", 1.0)?;
        if intensity.iter().any(|v| !(0.0..=1e6).contains(v)) || !(1e-6..=1e12).contains(&white) {
            return Err(
                "Caustics irradiance requires finite bounded nonnegative values and positive white"
                    .into(),
            );
        }
        let photons = count(args, "photon_budget", 16_384, 1_048_576)?;
        let seed = match args.get("seed") {
            None => 0,
            Some(Value::U64(seed)) => *seed,
            Some(Value::I64(seed)) if *seed >= 0 => *seed as u64,
            _ => return Err("Caustics seed must be unsigned 64-bit".into()),
        };
        let (ior, attenuation) = optical_parameters(args.get("iop"))?;

        let geometry = &water.surface.mesh;
        if !(3..=65_536).contains(&geometry.positions.len())
            || !(1..=65_536).contains(&geometry.triangles.len())
            || geometry
                .positions
                .iter()
                .flatten()
                .any(|v| !(-10_000.0..=10_000.0).contains(v))
        {
            return Err(
                "Caustic water mesh exceeds its admitted topology or coordinate bounds".into(),
            );
        }
        let water_mesh = Mesh {
            positions: geometry
                .positions
                .iter()
                .copied()
                .map(Vec3::from_array)
                .collect(),
            indices: geometry.triangles.iter().copied().flatten().collect(),
            ..Mesh::default()
        };
        for triangle in &geometry.triangles {
            if triangle
                .iter()
                .any(|&i| i as usize >= geometry.positions.len())
            {
                return Err("Caustic water mesh includes an out-of-range triangle".into());
            }
            let [a, b, c] = triangle.map(|i| water_mesh.positions[i as usize]);
            if (b - a).cross(c - a).length_squared() <= 1.0e-16 {
                return Err("Caustic water mesh includes a degenerate triangle".into());
            }
        }
        let bounds = water_mesh.bounds();
        let ray_bias =
            (bounds.max.y.abs().max(bounds.min.y.abs()) * 4.0 * f32::EPSILON).max(0.0001);
        if samples
            .iter()
            .any(|y| *y > bounds.min.y - (2.0 * ray_bias).max(0.002))
        {
            return Err(
                "Caustic bed must lie below the water surface for this submerged branch".into(),
            );
        }
        let launch_y = bounds.max.y + ray_bias;
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_z = f32::NEG_INFINITY;
        for &vertex in &water_mesh.positions {
            let projected = vertex + incident * ((launch_y - vertex.y) / incident.y);
            min_x = min_x.min(projected.x);
            max_x = max_x.max(projected.x);
            min_z = min_z.min(projected.z);
            max_z = max_z.max(projected.z);
        }
        let size_x = max_x - min_x;
        let size_z = max_z - min_z;
        if size_x <= 0.002 || size_z <= 0.002 {
            return Err("Water aperture must have positive XZ area".into());
        }
        let cells = (w - 1) * (h - 1);
        let geometry_size =
            (geometry.triangles.len() + cells * 2) * 160 + (geometry.positions.len() + w * h) * 16;
        self.charge(
            photons.saturating_mul(
                96 + 16 * (cells.ilog2() + geometry.triangles.len().ilog2()) as usize,
            ),
            geometry_size + cells * 64,
        )?;
        let bed = Receiver::new(
            w,
            h,
            origin.map(|v| v as f32),
            spacing.map(|v| v as f32),
            samples,
        );
        let water_tree = Bvh::build(&water_mesh);
        let bed_tree = Bvh::build(&bed.mesh);
        let area = f64::from(size_x) * f64::from(size_z);
        let cell_area = f64::from(bed.cell[0]) * f64::from(bed.cell[1]);
        let scale = area * f64::from(-incident.y) / (photons as f64 * cell_area);
        let mut irradiance = vec![0.0_f64; cells * 3];
        let mut deposited = [0_u64; 3];
        for i in 0..photons {
            if i % 1024 == 0 {
                self.charge(0, 0)?;
            }
            let x = min_x + size_x * counter_uniform(seed, i as u64, 0) as f32;
            let z = min_z + size_z * counter_uniform(seed, i as u64, 1) as f32;
            let ray = Ray::new(Vec3::new(x, launch_y, z), incident);
            let Some(surface) = water_tree.intersect(&water_mesh, &ray) else {
                continue;
            };
            let n = if surface.normal.dot(incident) > 0.0 {
                -surface.normal
            } else {
                surface.normal
            };
            let c = (-n.dot(incident)).clamp(0.0, 1.0);
            for lane in 0..3 {
                let n2 = ior[lane];
                let ratio = n2.recip();
                let c2 = (1.0 - ratio * ratio * (1.0 - c * c)).max(0.0).sqrt();
                let refracted = Ray::new(
                    surface.point - n * ray_bias,
                    incident * ratio + n * (ratio * c - c2),
                );
                if refracted.direction.y >= 0.0 {
                    continue;
                }
                let Some(hit) = bed_tree.intersect(&bed.mesh, &refracted) else {
                    continue;
                };
                if water_tree
                    .intersect(&water_mesh, &refracted)
                    .is_some_and(|s| s.distance < hit.distance)
                {
                    continue;
                }
                let Some(cell) = bed.cell_index(hit.point) else {
                    continue;
                };
                let rs = (c - n2 * c2) / (c + n2 * c2);
                let rp = (n2 * c - c2) / (n2 * c + c2);
                let fresnel = 1.0 - (rs * rs + rp * rp) * 0.5;
                let beam = f64::from(-attenuation[lane] * hit.distance).exp();
                irradiance[cell * 3 + lane] +=
                    intensity[lane] * scale * f64::from(fresnel.clamp(0.0, 1.0)) * beam;
                deposited[lane] += 1;
            }
        }

        let field_samples: Vec<_> = irradiance.iter().map(|x| *x as f32).collect();
        let rgba: Vec<_> = irradiance
            .chunks_exact(3)
            .flat_map(|c| {
                let lane = |i: usize| (65535.0 * (c[i] / white).clamp(0.0, 1.0)).round() as u16;
                [lane(0), lane(1), lane(2), u16::MAX]
            })
            .collect();
        let receipt = super::receipt(
            self,
            "direct-refractive-caustic-bake",
            &field_samples,
            &[
                ("photon_budget", photons as f64),
                ("white_irradiance", white),
                ("bed_cell_area_m2", cell_area),
                ("seed_high", (seed >> 32) as f64),
                ("seed_low", (seed as u32) as f64),
            ],
        )?;
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![(h - 1) as u32, (w - 1) as u32, 3],
                samples: field_samples,
            },
        )))?;
        let image =
            self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(RadianceLattice {
                width: (w - 1) as u32,
                height: (h - 1) as u32,
                channels: 4,
                primaries_iri: "urn:qualia:color:primaries:rec709".into(),
                transfer_iri: "urn:qualia:color:transfer:linear".into(),
                samples: LatticeSamples::U16(rgba),
            })))?;
        Ok(Value::Record(Args::from([
            ("caustic_map".into(), image),
            ("irradiance".into(), field),
            ("receipt".into(), receipt),
            (
                "deposited_photons_rgb".into(),
                Value::List(deposited.map(Value::U64).into()),
            ),
        ])))
    }
}

fn optical_parameters(value: Option<&Value>) -> Result<([f32; 3], [f32; 3]), String> {
    if let Some(value) = value {
        let parameters = record(value)?;
        fields(
            parameters,
            &[
                "ior",
                "attenuation_per_m",
                "wavelength_nm",
                "absorption_per_m",
                "rayleigh_scattering",
                "mie_scattering",
                "mie_g",
            ],
        )?;
        let index = vector::<3>(required(parameters, "ior")?)?;
        let c = vector::<3>(required(parameters, "attenuation_per_m")?)?;
        if index.iter().any(|n| !(1.001..=3.0).contains(n))
            || c.iter().any(|v| !(0.0..=1.0e4).contains(v))
        {
            return Err(
                "Caustic IOP needs bounded refractive index and total beam attenuation".into(),
            );
        }
        if let Some(wavelength) = parameters.get("wavelength_nm") {
            if vector::<3>(wavelength)? != [650.0, 532.0, 450.0] {
                return Err("Caustic IOP requires the 650/532/450 nm optical lanes".into());
            }
        }
        Ok((index.map(|v| v as f32), c.map(|v| v as f32)))
    } else {
        let iop = DielectricVolumeIop::default();
        Ok((iop.ior_spectrum, iop.beam_attenuation()))
    }
}

impl Receiver {
    fn new(width: usize, height: usize, origin: [f32; 2], cell: [f32; 2], samples: &[f32]) -> Self {
        let mut mesh = Mesh::new();
        for (i, &y) in samples.iter().enumerate() {
            mesh.positions.push(Vec3::new(
                origin[0] + (i % width) as f32 * cell[0],
                y,
                origin[1] + (i / width) as f32 * cell[1],
            ));
        }
        for z in 0..height - 1 {
            for x in 0..width - 1 {
                let i = (z * width + x) as u32;
                let stride = width as u32;
                mesh.indices
                    .extend([i, i + stride, i + 1, i + 1, i + stride, i + stride + 1]);
            }
        }
        Self {
            width,
            height,
            origin,
            cell,
            mesh,
        }
    }

    fn cell_index(&self, hit: Vec3) -> Option<usize> {
        let x = (hit.x - self.origin[0]) / self.cell[0];
        let z = (hit.z - self.origin[1]) / self.cell[1];
        if x < -0.001
            || z < -0.001
            || x > (self.width - 1) as f32 + 0.001
            || z > (self.height - 1) as f32 + 0.001
        {
            return None;
        }
        let x = (x.floor() as usize).min(self.width - 2);
        let z = (z.floor() as usize).min(self.height - 2);
        Some(z * (self.width - 1) + x)
    }
}
