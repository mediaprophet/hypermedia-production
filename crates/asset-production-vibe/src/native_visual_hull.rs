//! Calibrated silhouette intersection and native isosurface extraction.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use crate::artifact::{AttributedMesh, FieldLattice};
use crate::*;
use qualia_core_db::{
    render::assets::{ImportedMesh, Mesh},
    specialized_libs::computational_geometry::{Point3, isosurface::marching_cubes},
};

struct Silhouette<'a> {
    raster: &'a crate::graphics::Rgba,
    /// Row-major clip-from-local transform. Vertical NDC is positive upward.
    clip: [[f64; 4]; 4],
}

impl Silhouette<'_> {
    fn contains(&self, position: [f64; 3], threshold: u8) -> bool {
        let clip = self
            .clip
            .map(|row| row[0] * position[0] + row[1] * position[1] + row[2] * position[2] + row[3]);
        let [x, y, z, w] = clip;
        if w <= 0.0 || x.abs() > w || y.abs() > w || z.abs() > w {
            return false;
        }
        let width = self.raster.width as usize;
        let height = self.raster.height as usize;
        let x = (((x / w + 1.0) * 0.5 * width as f64).floor() as usize).min(width - 1);
        let y = (((1.0 - y / w) * 0.5 * height as f64).floor() as usize).min(height - 1);
        self.raster.pixels[y * width + x][3] >= threshold
    }
}

impl AssetHost<'_> {
    pub(crate) fn reconstruct_multiview_3d(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "views",
                "bounds_min_m",
                "bounds_max_m",
                "resolution",
                "alpha_threshold",
            ],
        )?;
        let source = list(required(args, "views")?)?;
        if !(2..=16).contains(&source.len()) {
            return Err("Visual hull requires 2 to 16 calibrated silhouette views".into());
        }
        let resolution = count(args, "resolution", 32, 56)?;
        if resolution < 8 {
            return Err("Visual hull resolution must be at least 8".into());
        }
        let threshold = count(args, "alpha_threshold", 128, 255)? as u8;
        let min: [f64; 3] = vector(required(args, "bounds_min_m")?)?;
        let max: [f64; 3] = vector(required(args, "bounds_max_m")?)?;
        if min.iter().chain(&max).any(|v| v.abs() > 1e6)
            || min.iter().zip(max).any(|(&a, b)| b - a < 1e-5)
        {
            return Err("Visual hull needs increasing local metre bounds".into());
        }
        let samples = resolution.pow(3);
        let cells = (resolution - 1).pow(3);
        self.charge(
            samples.saturating_mul(source.len() * 64 + 128),
            samples
                .saturating_mul(12)
                .saturating_add(cells.saturating_mul(864)),
        )?;
        let mut views = Vec::with_capacity(source.len());
        for view in source {
            let view = record(view)?;
            fields(view, &["image", "clip_from_local"])?;
            let raster = self.rgba(required(view, "image")?)?;
            let matrix = list(required(view, "clip_from_local")?)?;
            let matrix = matrix
                .iter()
                .map(vector::<4>)
                .collect::<Result<Vec<_>, _>>()?;
            let clip: [[f64; 4]; 4] = matrix
                .try_into()
                .map_err(|_| "Visual hull camera requires four matrix rows")?;
            if raster.width == 0
                || raster.height == 0
                || clip.iter().flatten().any(|v| v.abs() > 1e9)
                || clip.iter().any(|row| row.iter().all(|&v| v == 0.0))
                || views.iter().any(|view: &Silhouette<'_>| view.clip == clip)
            {
                return Err("Invalid or duplicate calibrated silhouette camera".into());
            }
            views.push(Silhouette { raster, clip });
        }
        let spacing = std::array::from_fn::<_, 3, _>(|axis| {
            (max[axis] - min[axis]) / (resolution - 1) as f64
        });
        let mut grid = vec![1.0_f64; samples];
        let mut inside = 0usize;
        for z in 0..resolution {
            for y in 0..resolution {
                for x in 0..resolution {
                    let index = [x, y, z];
                    let point =
                        std::array::from_fn(|axis| min[axis] + index[axis] as f64 * spacing[axis]);
                    if views.iter().all(|view| view.contains(point, threshold)) {
                        if index.iter().any(|&v| v == 0 || v + 1 == resolution) {
                            return Err(
                                "Visual hull reaches the volume boundary; supply enclosing bounds"
                                    .into(),
                            );
                        }
                        grid[x + resolution * (y + resolution * z)] = -1.0;
                        inside += 1;
                    }
                }
            }
        }
        if inside == 0 {
            return Err("The calibrated silhouettes have no common sampled volume".into());
        }
        let mut vertices = vec![Point3::new(0.0, 0.0, 0.0); cells * 30];
        let mut triangles = vec![[0; 3]; cells * 10];
        let (vertex_count, face_count) = marching_cubes(
            &grid,
            resolution,
            resolution,
            resolution,
            spacing[0],
            spacing[1],
            spacing[2],
            min[0],
            min[1],
            min[2],
            0.0,
            &mut vertices,
            &mut triangles,
        )
        .map_err(|err| format!("Visual hull isosurface: {err:?}"))?;
        if !(1..=200_000).contains(&vertex_count) || !(1..=400_000).contains(&face_count) {
            return Err("Visual hull exceeds admitted mesh complexity or has no isosurface".into());
        }
        triangles.truncate(face_count);
        let mut indices = BTreeMap::<[u32; 3], u32>::new();
        let mut positions = Vec::with_capacity(vertex_count);
        let remap: Vec<_> = vertices[..vertex_count]
            .iter()
            .map(|p| {
                let position = [p.x as f32, p.y as f32, p.z as f32];
                let bits = position.map(|v| if v == 0.0 { 0 } else { v.to_bits() });
                *indices.entry(bits).or_insert_with(|| {
                    let slot = positions.len() as u32;
                    positions.push(position);
                    slot
                })
            })
            .collect();
        drop(vertices);
        for triangle in &mut triangles {
            *triangle = triangle.map(|slot| remap[slot as usize]);
        }
        if triangles.iter().any(|[a, b, c]| a == b || b == c || c == a) {
            return Err("Visual hull mesh degenerates at output coordinate precision".into());
        }
        let mut bounds_min = positions[0];
        let mut bounds_max = positions[0];
        for position in &positions {
            for axis in 0..3 {
                bounds_min[axis] = bounds_min[axis].min(position[axis]);
                bounds_max[axis] = bounds_max[axis].max(position[axis]);
            }
        }
        let mesh =
            self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(AttributedMesh {
                surface: ImportedMesh {
                    mesh: Mesh {
                        positions,
                        triangles,
                        min: bounds_min,
                        max: bounds_max,
                    },
                    normals: None,
                    tangents: None,
                    texture_coordinates_0: None,
                    materials: vec![],
                    texture_dependencies: vec![],
                    submeshes: vec![],
                },
                uv1: vec![],
                vertex_colors: vec![],
                skin_weights: vec![],
                morph_targets: Arc::from([]),
                protected_faces: vec![],
            })))?;
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![resolution as u32; 3],
                samples: grid.into_iter().map(|v| v as f32).collect(),
            },
        )))?;
        Ok(Value::Record(Args::from([
            ("mesh".into(), mesh),
            ("field".into(), field),
            ("occupied_samples".into(), Value::U64(inside as u64)),
            (
                "origin_m".into(),
                Value::List(min.into_iter().map(Value::F64).collect()),
            ),
            (
                "spacing_m".into(),
                Value::List(spacing.into_iter().map(Value::F64).collect()),
            ),
        ])))
    }
}
