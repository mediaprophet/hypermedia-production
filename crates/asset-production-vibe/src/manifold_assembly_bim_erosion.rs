use super::*;
use qualia_core_db::specialized_libs::computational_geometry::Point3;
use std::collections::BTreeMap;
use vibe::Value;

fn opt_str(a: &Args, k: &str, default: &str) -> Result<String, String> {
    match a.get(k) {
        None => Ok(default.to_string()),
        Some(v) => Ok(text(v)?.to_string()),
    }
}

impl AssetHost<'_> {
    /// 123: `hypermedia.manifold.sweep_assembly_profile`
    /// (`profile-authoring`)
    pub(crate) fn sweep_assembly_profile(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "title",
                "profile",
                "width",
                "height",
                "wall",
                "length",
                "caps",
                "parts",
                "axis",
            ],
        )?;
        let title_str = opt_str(a, "title", "Hierarchical CAD Assembly")?;
        let width = number(a, "width", 1.2)?;
        let height = number(a, "height", 0.8)?;
        let wall = number(a, "wall", 0.15)?;
        let length = number(a, "length", 2.5)?;
        let caps = boolean(a, "caps", true)?;
        let opt_prof = a
            .get("profile")
            .map(|v| self.profile(v).map(|p| p.to_vec()))
            .transpose()?;

        self.charge(512, 16384)?;

        use profile_authoring::{
            AssemblyLimits, AuthoringPart, GeometryLimits, Profile, Transform, compile_assembly,
            profile_sweep, sphere, torus,
        };
        use qualia_core_db::container_10d::{
            MaterialRecord, OpacityMode, ShadingModel, attach_material_section,
        };
        use qualia_core_db::q_hash;
        use qualia_core_db::render::compile_10d::{
            Compile10dExtras, compile_mesh_to_10d_with_extras_and_frames,
        };

        let parts = if let Some(parts_val) = a.get("parts") {
            let part_list = list(parts_val)?;
            if part_list.is_empty() || part_list.len() > 512 {
                return Err("Assembly parts list requires 1..=512 parts".into());
            }
            let mut authored = Vec::with_capacity(part_list.len());
            for (idx, pv) in part_list.iter().enumerate() {
                let pr = record(pv)?;
                fields(
                    pr,
                    &[
                        "key",
                        "parent",
                        "semantic_id",
                        "mesh",
                        "shape",
                        "radius",
                        "minor_radius",
                        "width",
                        "height",
                        "depth",
                        "wall",
                        "axis",
                        "caps",
                        "translation",
                        "rotation",
                        "scale",
                        "color",
                        "roughness",
                        "metallic",
                    ],
                )?;
                let key = opt_str(pr, "key", &format!("part_{idx}"))?;
                let parent = match pr.get("parent") {
                    Some(Value::String(s)) if !s.is_empty() => Some(s.clone()),
                    _ => None,
                };
                let semantic_id = match pr.get("semantic_id") {
                    Some(Value::String(s)) => q_hash(s).max(1),
                    Some(v) => (integer(v, "semantic_id")? as u64).max(1),
                    None => q_hash(&key).max((idx as u64) + 1),
                };
                let part_mesh = if let Some(mh) = pr.get("mesh") {
                    let Artifact::Mesh(m) = self.workspace.get(mh)? else {
                        return Err(format!("Assembly part '{key}' expected mesh handle"));
                    };
                    m.clone()
                } else {
                    let shape = opt_str(pr, "shape", "sphere")?;
                    match shape.as_str() {
                        "sphere" => {
                            let r = number(pr, "radius", 0.25)?;
                            sphere(r, 16, 12, GeometryLimits::default())
                                .map_err(|e| format!("sphere '{key}': {e}"))?
                        }
                        "torus" => {
                            let r_maj = number(pr, "radius", 0.4)?;
                            let r_min = number(pr, "minor_radius", 0.1)?;
                            torus(r_maj, r_min, 16, 12, GeometryLimits::default())
                                .map_err(|e| format!("torus '{key}': {e}"))?
                        }
                        "box" => {
                            let bw = number(pr, "width", 1.0)? as f32;
                            let bh = number(pr, "height", 1.0)? as f32;
                            let bd = number(pr, "depth", 1.0)? as f32;
                            qualia_core_db::specialized_libs::computational_geometry::authoring::box_mesh(bw, bh, bd)
                                .map_err(|e| format!("box '{key}': {e}"))?
                        }
                        "sweep" | "channel" => {
                            let pw = number(pr, "width", width)?;
                            let ph = number(pr, "height", height)?;
                            let pwall = number(pr, "wall", wall)?;
                            let pcaps = boolean(pr, "caps", caps)?;
                            let prof = Profile::OpenChannel {
                                width: pw,
                                height: ph,
                                wall: pwall,
                                corner_radius: 0.0,
                                segments: 16,
                            };
                            let ax = if let Some(ax_val) = pr.get("axis") {
                                let pts = list(ax_val)?;
                                pts.iter()
                                    .map(vector::<3>)
                                    .collect::<Result<Vec<[f64; 3]>, _>>()?
                            } else {
                                vec![[0.0, 0.0, 0.0], [0.0, 0.0, length]]
                            };
                            profile_sweep(&prof, &ax, pcaps, GeometryLimits::default())
                                .map_err(|e| format!("sweep '{key}': {e}"))?
                        }
                        other => return Err(format!("Unknown assembly part shape: {other}")),
                    }
                };
                let translation = pr
                    .get("translation")
                    .map(vector::<3>)
                    .transpose()?
                    .unwrap_or([0.0, 0.0, 0.0]);
                let rotation = pr
                    .get("rotation")
                    .map(vector::<3>)
                    .transpose()?
                    .unwrap_or([0.0, 0.0, 0.0]);
                let scale = pr
                    .get("scale")
                    .map(vector::<3>)
                    .transpose()?
                    .unwrap_or([1.0, 1.0, 1.0]);
                let base_color = if let Some(cv) = pr.get("color") {
                    let c = vector::<4>(cv)?;
                    [c[0] as f32, c[1] as f32, c[2] as f32, c[3] as f32]
                } else {
                    [0.78, 0.80, 0.84, 1.0]
                };
                let mut mat = MaterialRecord::legacy_default();
                mat.id = q_hash(&format!("mat_part_{key}"));
                mat.shading_model = ShadingModel::MetallicRoughness;
                mat.opacity_mode = OpacityMode::Opaque;
                mat.base_color = base_color;
                mat.roughness = number(pr, "roughness", 0.45)? as f32;
                mat.metallic = number(pr, "metallic", 0.05)? as f32;

                authored.push(AuthoringPart {
                    key,
                    parent,
                    semantic_id,
                    mesh: part_mesh,
                    material: mat,
                    trs: Transform {
                        translation,
                        rotation,
                        scale,
                    },
                    normals: None,
                    uv0: None,
                });
            }
            authored
        } else {
            let channel = if let Some(points) = opt_prof {
                Profile::Polyline { points }
            } else {
                Profile::OpenChannel {
                    width,
                    height,
                    wall,
                    corner_radius: 0.0,
                    segments: 16,
                }
            };
            let axis: Vec<[f64; 3]> = if let Some(ax_val) = a.get("axis") {
                list(ax_val)?
                    .iter()
                    .map(vector::<3>)
                    .collect::<Result<Vec<_>, _>>()?
            } else {
                vec![[0.0, 0.0, 0.0], [0.0, 0.0, length]]
            };
            let swept = profile_sweep(&channel, &axis, caps, GeometryLimits::default())
                .map_err(|e| format!("profile_sweep failed: {e}"))?;
            let sph = sphere(0.25, 16, 12, GeometryLimits::default())
                .map_err(|e| format!("sphere failed: {e}"))?;
            let tor = torus(0.4, 0.1, 16, 12, GeometryLimits::default())
                .map_err(|e| format!("torus failed: {e}"))?;

            vec![
                AuthoringPart {
                    key: "channel_beam".into(),
                    parent: None,
                    semantic_id: 1,
                    mesh: swept,
                    material: MaterialRecord::legacy_default(),
                    trs: Transform::default(),
                    normals: None,
                    uv0: None,
                },
                AuthoringPart {
                    key: "end_sphere".into(),
                    parent: Some("channel_beam".into()),
                    semantic_id: 2,
                    mesh: sph,
                    material: MaterialRecord::legacy_default(),
                    trs: Transform {
                        translation: [0.0, 0.0, length],
                        rotation: [0.0, 0.0, 0.0],
                        scale: [1.0, 1.0, 1.0],
                    },
                    normals: None,
                    uv0: None,
                },
                AuthoringPart {
                    key: "collar_torus".into(),
                    parent: Some("channel_beam".into()),
                    semantic_id: 3,
                    mesh: tor,
                    material: MaterialRecord::legacy_default(),
                    trs: Transform::default(),
                    normals: None,
                    uv0: None,
                },
            ]
        };

        let compiled = compile_assembly(&parts, AssemblyLimits::default())
            .map_err(|e| format!("compile_assembly failed: {e}"))?;

        let vertex_count = compiled.mesh.positions.len();
        let triangle_count = compiled.mesh.triangles.len();
        let part_count = compiled.parts.len();

        let mut vertex_colors = vec![[0.74f32, 0.78, 0.82, 1.0]; vertex_count];
        for range in &compiled.ranges {
            if let Some(mat) = compiled.materials.iter().find(|m| m.id == range.material_id) {
                let start_tri = (range.first_index as usize) / 3;
                let end_tri = ((range.first_index + range.index_count) as usize / 3)
                    .min(compiled.mesh.triangles.len());
                for tri in &compiled.mesh.triangles[start_tri..end_tri] {
                    for &vi in tri {
                        if let Some(slot) = vertex_colors.get_mut(vi as usize) {
                            *slot = mat.base_color;
                        }
                    }
                }
            }
        }

        let flat_indices: Vec<u32> = compiled
            .mesh
            .triangles
            .iter()
            .flat_map(|t| [t[0], t[1], t[2]])
            .collect();
        let turntable_bytes =
            crate::living_world_anatomy::turntable_render::render_turnaround_sheet_png(
                &compiled.mesh.positions,
                &compiled.normals,
                &vertex_colors,
                &flat_indices,
            )?;
        let blueprint_bytes =
            crate::living_world_anatomy::turntable_render::render_anatomical_blueprint_svg(
                &title_str,
                &compiled.mesh.positions,
                &vertex_colors,
                &flat_indices,
                part_count,
            );

        let mut bytes_10d = compile_mesh_to_10d_with_extras_and_frames(
            &compiled.mesh,
            &[],
            None,
            Compile10dExtras::default(),
            Some(&vertex_colors),
            Some(&compiled.normals),
            None,
        )
        .map_err(|e| format!("10D encoding failed: {e:?}"))?;
        let mut sorted_mats = compiled.materials.clone();
        sorted_mats.sort_by_key(|m| m.id);
        sorted_mats.dedup_by_key(|m| m.id);
        if let Ok(with_mats) = attach_material_section(
            &bytes_10d,
            &sorted_mats,
            &compiled.ranges,
            (triangle_count * 3) as u32,
        ) {
            bytes_10d = with_mats;
        }
        let container_10d = self.encode("10d", bytes_10d)?;
        let turntable_png = self.encode("png", turntable_bytes)?;
        let blueprint_svg = self.encode("svg", blueprint_bytes)?;
        let mesh_handle = self
            .workspace
            .insert(Artifact::Mesh(compiled.mesh), self.limits)?;

        let mut out = BTreeMap::new();
        out.insert("mesh".into(), mesh_handle);
        out.insert("container_10d".into(), container_10d);
        out.insert("turntable_png".into(), turntable_png);
        out.insert("blueprint_svg".into(), blueprint_svg);
        out.insert("part_count".into(), Value::I64(part_count as i64));
        out.insert("vertex_count".into(), Value::I64(vertex_count as i64));
        out.insert("triangle_count".into(), Value::I64(triangle_count as i64));
        out.insert("normal_count".into(), Value::I64(compiled.normals.len() as i64));
        Ok(Value::Record(out))
    }

    /// 124: `hypermedia.manifold.compile_bim_ifc4`
    /// (`manifold-bim-csg`)
    pub(crate) fn compile_bim_ifc4(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["project_name", "wall_length_m", "wall_height_m", "wall_thickness_m", "floor_to_floor_m"])?;
        let project_name = opt_str(a, "project_name", "Commons Civic Hall")?;
        let wall_length = number(a, "wall_length_m", 8.0)?;
        let wall_height = number(a, "wall_height_m", 3.2)?;
        let wall_thickness = number(a, "wall_thickness_m", 0.25)?;
        let floor_to_floor = number(a, "floor_to_floor_m", 3.2)?;

        self.charge(512, 16384)?;

        use manifold_bim_csg::{
            Meter, P3, export_minimal_building, generate_straight_stair, parse_step, subtract,
            volume,
        };

        let cuboid = |lo: P3, hi: P3| -> Vec<[P3; 3]> {
            let v = |x: usize, y: usize, z: usize| {
                [[lo[0], hi[0]][x], [lo[1], hi[1]][y], [lo[2], hi[2]][z]]
            };
            [
                [v(0, 0, 1), v(1, 0, 1), v(1, 1, 1), v(0, 1, 1)],
                [v(0, 0, 0), v(0, 1, 0), v(1, 1, 0), v(1, 0, 0)],
                [v(0, 0, 0), v(1, 0, 0), v(1, 0, 1), v(0, 0, 1)],
                [v(1, 1, 0), v(0, 1, 0), v(0, 1, 1), v(1, 1, 1)],
                [v(0, 1, 0), v(0, 0, 0), v(0, 0, 1), v(0, 1, 1)],
                [v(1, 0, 0), v(1, 1, 0), v(1, 1, 1), v(1, 0, 1)],
            ]
            .iter()
            .flat_map(|[p0, p1, p2, p3]| [[*p0, *p1, *p2], [*p0, *p2, *p3]])
            .collect()
        };

        let wall_solid = cuboid([0.0, 0.0, 0.0], [wall_length, wall_thickness, wall_height]);
        let window_void = cuboid(
            [wall_length * 0.25, -0.1, wall_height * 0.3],
            [wall_length * 0.75, wall_thickness + 0.1, wall_height * 0.75],
        );
        let mut meter = Meter::default();
        let csg_cut = subtract(&wall_solid, &window_void, &mut meter)
            .map_err(|e| format!("CSG subtract failed: {e}"))?;
        let net_volume_m3 = volume(&csg_cut).abs();

        let total_risers = ((floor_to_floor / 0.175).round().max(2.0)) as u32;
        let stair = generate_straight_stair([0.0, 0.0], 0.0, 1.2, 0.28, total_risers, 0.175, 0.0);

        let mut vertices = Vec::with_capacity(csg_cut.len() * 3 + (total_risers as usize) * 36);
        let mut triangles = Vec::with_capacity(csg_cut.len() + (total_risers as usize) * 12);
        for tri in &csg_cut {
            let base = vertices.len() as u32;
            for p in tri {
                vertices.push(Point3::new(p[0], p[1], p[2]));
            }
            triangles.push([base, base + 1, base + 2]);
        }
        if let Some(run) = stair.runs.first() {
            let risers = run.risers.max(1);
            for step_idx in 0..risers {
                let x0 = run.start[0] + (step_idx as f64) * 0.28;
                let x1 = x0 + 0.28;
                let y0 = wall_thickness + 0.2;
                let y1 = y0 + stair.width;
                let z0 = run.elevation_start + (step_idx as f64) * 0.175;
                let z1 = z0 + 0.175;
                for tri in cuboid([x0, y0, z0], [x1, y1, z1]) {
                    let base = vertices.len() as u32;
                    for p in tri {
                        vertices.push(Point3::new(p[0], p[1], p[2]));
                    }
                    triangles.push([base, base + 1, base + 2]);
                }
            }
        }
        let mesh_handle = self.store_mesh(vertices, triangles)?;

        let ifc_text = export_minimal_building(
            &project_name,
            "Main Civic Building",
            "Ground Floor",
            0.0,
            "North Load-Bearing Wall",
            wall_length,
            wall_height,
            wall_thickness,
        );
        let parsed_step =
            parse_step(ifc_text.as_bytes()).map_err(|e| format!("STEP parse failed: {e}"))?;
        let step_entity_count = parsed_step.entities.len();
        let ifc_handle = self.encode("ifc", ifc_text.into_bytes())?;

        let mut out = BTreeMap::new();
        out.insert("mesh".into(), mesh_handle);
        out.insert("ifc_resource".into(), ifc_handle);
        out.insert("csg_triangle_count".into(), Value::I64(csg_cut.len() as i64));
        out.insert("net_volume_m3".into(), Value::F64(net_volume_m3));
        out.insert("stair_riser_count".into(), Value::I64(stair.runs.first().map(|r| r.risers as i64).unwrap_or(0)));
        out.insert("ifc_step_entity_count".into(), Value::I64(step_entity_count as i64));
        Ok(Value::Record(out))
    }

    /// 125: `hypermedia.manifold.erode_geomorphic_heightfield`
    /// (`terrain-erosion-solvers`)
    pub(crate) fn erode_geomorphic_heightfield(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "image",
                "grid_size",
                "crater_radius",
                "terrace_spacing",
                "thermal_strength",
                "elevation_scale",
            ],
        )?;
        let size = count(a, "grid_size", 16, 128)?;
        let crater_radius = number(a, "crater_radius", 4.0)? as f32;
        let terrace_spacing = number(a, "terrace_spacing", 0.15)? as f32;
        let thermal_strength = number(a, "thermal_strength", 0.25)?;
        let elevation_scale = number(a, "elevation_scale", 4.0)?.clamp(0.1, 1000.0);

        let mut heights = if let Some(img_val) = a.get("image") {
            let rgba = self.rgba(img_val)?;
            let mut h = Vec::with_capacity(size * size);
            let w = rgba.width.max(1) as usize;
            let ht = rgba.height.max(1) as usize;
            for y in 0..size {
                for x in 0..size {
                    let sx = (x * w / size).min(w - 1);
                    let sy = (y * ht / size).min(ht - 1);
                    let px = rgba.pixels[sy * w + sx];
                    let luma = (0.2126 * px[0] as f32 + 0.7152 * px[1] as f32 + 0.0722 * px[2] as f32) / 255.0;
                    h.push(luma);
                }
            }
            h
        } else {
            vec![0.5f32; size * size]
        };

        self.charge(size * size * 8, size * size * 32)?;

        use terrain_erosion_solvers::{
            Crater, HydraulicConfig, TerraceConfig, apply_crater, apply_terraces,
            d8_flow_directions, flow_accumulation, hydraulic_erosion, priority_flood_fill,
            thermal_shape,
        };

        let crater = Crater {
            x: (size as f32) * 0.5,
            y: (size as f32) * 0.5,
            radius: crater_radius,
            depth: 0.3,
        };
        apply_crater(&mut heights, size, size, &crater, 0.25, 0.1);
        apply_terraces(
            &mut heights,
            size,
            size,
            &TerraceConfig {
                spacing: terrace_spacing.max(0.02),
                intensity: 0.5,
                ..Default::default()
            },
        );
        let relaxed = thermal_shape(&heights, size, thermal_strength.clamp(0.0, 1.0));
        let mut conditioned = relaxed.clone();
        priority_flood_fill(&mut conditioned, size, size);
        let dirs = d8_flow_directions(&conditioned, size, size);
        let accum = flow_accumulation(&conditioned, &dirs, size, size);
        let max_accum = accum.iter().copied().fold(0.0f32, f32::max);

        let hydro = hydraulic_erosion(
            &mut conditioned,
            size,
            size,
            &HydraulicConfig {
                droplet_count: 128,
                ..Default::default()
            },
        );
        let min_h = hydro.height.iter().copied().fold(f32::INFINITY, f32::min);
        let max_h = hydro.height.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let span = (max_h - min_h).max(1e-6);
        let accum_denom = max_accum.max(1.0);

        let mut pixels = Vec::with_capacity(size * size);
        let mut vertices = Vec::with_capacity(size * size);
        for y in 0..size {
            for x in 0..size {
                let idx = y * size + x;
                let h_val = hydro.height[idx];
                let norm_h = ((h_val - min_h) / span).clamp(0.0, 1.0);
                let norm_flow = (accum[idx] / accum_denom).sqrt().clamp(0.0, 1.0);
                let r = ((norm_h * 0.85 + 0.10) * 255.0).round().clamp(0.0, 255.0) as u8;
                let g = ((norm_h * 0.75 + norm_flow * 0.20) * 255.0).round().clamp(0.0, 255.0) as u8;
                let b = ((norm_h * 0.50 + norm_flow * 0.45) * 255.0).round().clamp(0.0, 255.0) as u8;
                pixels.push([r, g, b, 255]);

                let vx = (x as f64 / (size.saturating_sub(1).max(1) as f64) - 0.5) * 10.0;
                let vy = (y as f64 / (size.saturating_sub(1).max(1) as f64) - 0.5) * 10.0;
                let vz = (h_val as f64) * elevation_scale;
                vertices.push(Point3::new(vx, vy, vz));
            }
        }
        let mut triangles = Vec::with_capacity(size.saturating_sub(1) * size.saturating_sub(1) * 2);
        for y in 0..size.saturating_sub(1) {
            for x in 0..size.saturating_sub(1) {
                let i00 = (y * size + x) as u32;
                let i10 = i00 + 1;
                let i01 = ((y + 1) * size + x) as u32;
                let i11 = i01 + 1;
                triangles.push([i00, i10, i11]);
                triangles.push([i00, i11, i01]);
            }
        }
        let image_handle = self.store_rgba(size as u32, size as u32, pixels)?;
        let mesh_handle = self.store_mesh(vertices, triangles)?;

        let mut out = BTreeMap::new();
        out.insert("image".into(), image_handle.clone());
        out.insert("heightfield_image".into(), image_handle);
        out.insert("mesh".into(), mesh_handle);
        out.insert("grid_size".into(), Value::I64(size as i64));
        out.insert("cell_count".into(), Value::I64((size * size) as i64));
        out.insert("min_elevation".into(), Value::F64(min_h as f64));
        out.insert("max_elevation".into(), Value::F64(max_h as f64));
        out.insert("max_flow_accumulation".into(), Value::F64(max_accum as f64));
        Ok(Value::Record(out))
    }
}
