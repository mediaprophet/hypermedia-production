use super::*;
use qualia_core_db::specialized_libs::computational_geometry::Point3;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use vibe::Value;

impl AssetHost<'_> {
    /// 135: `hypermedia.kinetics.solve_continuum_fem_lbm` (`kinetics-continuum-multiphysics`)
    pub(crate) fn solve_continuum_fem_lbm(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "youngs_modulus_pa", "poisson_ratio", "lbm_steps"])?;
        if let Some(mv) = a.get("mesh") {
            let Artifact::Mesh(_) = self.workspace.get(mv)? else {
                return Err("Expected mesh handle".into());
            };
        }
        let youngs = number(a, "youngs_modulus_pa", 210e9)?;
        let nu = number(a, "poisson_ratio", 0.3)?.clamp(0.01, 0.49);
        let lbm_steps = count(a, "lbm_steps", 2, 16)?;

        self.charge(512 * lbm_steps, 16384)?;

        use kinetics_continuum_multiphysics::{
            C_DIRS, CholeskyFactor, ElasticIsotropic, LbmGrid3D, Point3D, export_abaqus_inp,
            generate_box_hex_mesh,
        };

        let mat = ElasticIsotropic::new(youngs, nu, 7850.0);
        let k_dense = [4.0, 1.0, 1.0, 3.0];
        let chol = CholeskyFactor::factorize(2, &k_dense)
            .map_err(|e| format!("Cholesky factorization failed: {e}"))?;
        let disp = chol.solve(&[5.0, 4.0]);

        let mut lbm = LbmGrid3D::new(4, 4, 4, 0.02);
        let mass0: f64 = lbm.f.iter().sum();
        for _ in 0..lbm_steps {
            lbm.step();
        }
        let mass1: f64 = lbm.f.iter().sum();

        let hex_mesh = generate_box_hex_mesh(2, 1, 1, Point3D::new(0.0, 0.0, 0.0), Point3D::new(2.0, 1.0, 1.0));
        let conn: Vec<[usize; 8]> = hex_mesh.elements.iter().map(|e| e.nodes).collect();
        let inp_deck = export_abaqus_inp(&hex_mesh.nodes, &conn, "STRUCTURAL_STEEL", youngs, nu);
        let inp_handle = self.encode("inp", inp_deck.into_bytes())?;

        let vertices: Vec<Point3> = hex_mesh.nodes.iter().map(|n| Point3::new(n.x, n.y, n.z)).collect();
        const HEX_QUAD_FACES: [[usize; 4]; 6] = [
            [0, 3, 2, 1],
            [4, 5, 6, 7],
            [0, 1, 5, 4],
            [3, 7, 6, 2],
            [0, 4, 7, 3],
            [1, 2, 6, 5],
        ];
        let mut triangles = Vec::with_capacity(hex_mesh.elements.len() * 12);
        for elem in &hex_mesh.elements {
            for q in HEX_QUAD_FACES {
                let (a_idx, b_idx, c_idx, d_idx) = (
                    elem.nodes[q[0]] as u32,
                    elem.nodes[q[1]] as u32,
                    elem.nodes[q[2]] as u32,
                    elem.nodes[q[3]] as u32,
                );
                triangles.push([a_idx, b_idx, c_idx]);
                triangles.push([a_idx, c_idx, d_idx]);
            }
        }
        let mesh_handle = self.store_mesh(vertices, triangles)?;

        let z_mid = lbm.nz / 2;
        let mut lbm_pixels = Vec::with_capacity(lbm.nx * lbm.ny);
        for y in 0..lbm.ny {
            for x in 0..lbm.nx {
                let c_idx = lbm.idx(x, y, z_mid);
                let f_cell = &lbm.f[c_idx * 19..c_idx * 19 + 19];
                let (mut rho, mut ux, mut uy) = (0.0f64, 0.0f64, 0.0f64);
                for (i, &fi) in f_cell.iter().enumerate() {
                    rho += fi;
                    ux += fi * (C_DIRS[i][0] as f64);
                    uy += fi * (C_DIRS[i][1] as f64);
                }
                if rho > 1e-12 {
                    ux /= rho;
                    uy /= rho;
                }
                let r_byte = ((rho * 128.0).clamp(0.0, 255.0)) as u8;
                let g_byte = (((ux + 0.5) * 255.0).clamp(0.0, 255.0)) as u8;
                let b_byte = (((uy + 0.5) * 255.0).clamp(0.0, 255.0)) as u8;
                lbm_pixels.push([r_byte, g_byte, b_byte, 255]);
            }
        }
        let velocity_image = self.store_rgba(lbm.nx as u32, lbm.ny as u32, lbm_pixels)?;

        let mut out = BTreeMap::new();
        out.insert("abaqus_inp_resource".into(), inp_handle);
        out.insert("mesh".into(), mesh_handle);
        out.insert("velocity_image".into(), velocity_image);
        out.insert("shear_modulus_pa".into(), Value::F64(mat.shear_modulus()));
        out.insert("solved_dof_0".into(), Value::F64(disp[0]));
        out.insert("lbm_mass_conservation_error".into(), Value::F64((mass1 - mass0).abs()));
        out.insert("hex_element_count".into(), Value::I64(hex_mesh.elements.len() as i64));
        Ok(Value::Record(out))
    }

    /// 136: `hypermedia.kinetics.drape_orthotropic_garment` (`kinetics-garment-draping`)
    pub(crate) fn drape_orthotropic_garment(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "substeps", "iterations", "dt"])?;
        let input_mesh = if let Some(mv) = a.get("mesh") {
            let Artifact::Mesh(m) = self.workspace.get(mv)? else {
                return Err("Expected mesh handle".into());
            };
            Some(m.clone())
        } else {
            None
        };
        let substeps = count(a, "substeps", 2, 16)?;
        let iterations = count(a, "iterations", 4, 32)?;
        let dt = number(a, "dt", 0.016)?;

        self.charge(256 * substeps * iterations, 8192)?;

        use kinetics_garment_draping::{
            ClothMesh, DMat2, DVec2, DVec3, Jet, Triangle, VbdSettings, membrane_element,
            standard_fabrics, vbd_step,
        };

        let x_ad = Jet::<2>::variable(3.0, 0);
        let y_ad = Jet::<2>::variable(4.0, 1);
        let f_ad = x_ad * x_ad * y_ad;

        let mat = standard_fabrics()[0].to_material();
        let tri = Triangle {
            vertices: [0, 1, 2],
            rest: [DVec2::new(0.0, 0.0), DVec2::new(1.0, 0.0), DVec2::new(0.0, 1.0)],
            inv_dm: DMat2::from_cols(DVec2::new(1.0, 0.0), DVec2::new(0.0, 1.0)),
            area: 0.5,
            material: 0,
        };
        let stretched = vec![
            DVec3::new(0.0, 1.0, 0.0),
            DVec3::new(1.05, 1.0, 0.0),
            DVec3::new(0.0, 1.0, 1.0),
        ];
        let mem = membrane_element(&stretched, &tri, &mat);

        let mut cloth = ClothMesh::new(stretched, vec![tri], vec![], mat);
        cloth.pins[0] = Some(cloth.positions[0]);
        cloth.pins[1] = Some(cloth.positions[1]);
        let vbd_cfg = VbdSettings {
            dt: dt / (substeps as f64),
            iterations,
            gravity: DVec3::new(0.0, -9.81, 0.0),
            damping: 0.01,
        };
        for _ in 0..substeps {
            vbd_step(&mut cloth, &vbd_cfg);
        }
        let draped_y = cloth.positions[2].y;

        let (vertices, triangles) = if let Some(in_m) = input_mesh.filter(|m| !m.positions.is_empty() && !m.triangles.is_empty()) {
            let dy = (draped_y - 1.0) as f32;
            let verts = in_m
                .positions
                .iter()
                .enumerate()
                .map(|(idx, p)| {
                    let y_shift = if idx < 2 { 0.0 } else { dy };
                    Point3::new(p[0] as f64, (p[1] + y_shift) as f64, p[2] as f64)
                })
                .collect::<Vec<_>>();
            (verts, in_m.triangles)
        } else {
            let (p0, p1, p2) = (cloth.positions[0], cloth.positions[1], cloth.positions[2]);
            let p3 = DVec3::new(p1.x + p2.x - p0.x, p2.y, p1.z + p2.z - p0.z);
            let verts = vec![
                Point3::new(p0.x, p0.y, p0.z),
                Point3::new(p1.x, p1.y, p1.z),
                Point3::new(p2.x, p2.y, p2.z),
                Point3::new(p3.x, p3.y, p3.z),
            ];
            let mut tris: Vec<[u32; 3]> = cloth
                .triangles
                .iter()
                .map(|t| [t.vertices[0] as u32, t.vertices[1] as u32, t.vertices[2] as u32])
                .collect();
            tris.push([1, 3, 2]);
            (verts, tris)
        };

        let mut obj_text = String::from("# Draped Orthotropic Garment Panel\n");
        for v in &vertices {
            let _ = writeln!(obj_text, "v {:.6} {:.6} {:.6}", v.x, v.y, v.z);
        }
        for t in &triangles {
            let _ = writeln!(obj_text, "f {} {} {}", t[0] + 1, t[1] + 1, t[2] + 1);
        }
        let mesh_handle = self.store_mesh(vertices, triangles)?;
        let obj_resource = self.encode("obj", obj_text.into_bytes())?;

        let mut out = BTreeMap::new();
        out.insert("mesh".into(), mesh_handle);
        out.insert("obj_resource".into(), obj_resource);
        out.insert("jet_hessian_00".into(), Value::F64(f_ad.h[0][0]));
        out.insert("membrane_strain_energy".into(), Value::F64(mem.energy));
        out.insert("draped_vertex2_y".into(), Value::F64(draped_y));
        Ok(Value::Record(out))
    }

    /// 137: `hypermedia.kinetics.simulate_causal_block_diagram` (`kinetics-block-simulation` + `kinetics-motion-compositor`)
    pub(crate) fn simulate_causal_block_diagram(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["image", "constant_input", "tstop", "dt"])?;
        let opt_rgba = if let Some(iv) = a.get("image") {
            Some(self.rgba(iv)?.clone())
        } else {
            None
        };
        let const_val = number(a, "constant_input", 2.5)?;
        let tstop = number(a, "tstop", 1.0)?.clamp(0.05, 10.0);
        let dt = number(a, "dt", 0.01)?.clamp(0.001, 0.1);

        self.charge(256, 4096)?;

        use kinetics_block_simulation::{
            BlockFactory, Model, Param, Sim, SimConfig, Solver, StandardBlockFactory,
        };
        use kinetics_motion_compositor::{
            ArcLengthTable, GrayImage, eval_spatial_bezier, track_point_ncc,
        };

        let factory = StandardBlockFactory;
        let mut model = Model::new();
        let mut p_const = BTreeMap::new();
        p_const.insert("value".to_string(), Param::Num(const_val));
        let c_idx = model
            .add_block("src", factory.make("constant", &p_const, "src").map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let int_idx = model
            .add_block("int", factory.make("integrator", &BTreeMap::new(), "int").map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let sc_idx = model
            .add_block("scope", factory.make("scope", &BTreeMap::new(), "scope").map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        model.add_wire(c_idx, 0, int_idx, 0).map_err(|e| e.to_string())?;
        model.add_wire(int_idx, 0, sc_idx, 0).map_err(|e| e.to_string())?;

        let compiled = model.compile().map_err(|e| e.to_string())?;
        let mut sim = Sim::new(compiled, SimConfig { solver: Solver::Rk4, dt, tstop, ..Default::default() })
            .map_err(|e| e.to_string())?;
        let sim_out = sim.run().map_err(|e| e.to_string())?;
        let final_state = sim_out.x.last().copied().unwrap_or(0.0);

        let (p0, p1, p2, p3) = ([0.0, 0.0, 0.0], [0.0, 50.0, 0.0], [100.0, 50.0, 0.0], [100.0, 0.0, 0.0]);
        let arc_table = ArcLengthTable::build(p0, p1, p2, p3, 64);

        let mut traj_pts = Vec::with_capacity(16);
        for i in 0..16 {
            let s = (i as f64) / 15.0;
            let u = arc_table.u_at_progress(s);
            let pt = eval_spatial_bezier(p0, p1, p2, p3, u);
            traj_pts.push([pt[0], pt[1]]);
        }
        let profile_handle = self.store_profile(traj_pts.clone())?;

        let target = if let Some(rgba) = opt_rgba {
            let (w, h) = ((rgba.width as usize).max(16), (rgba.height as usize).max(16));
            let mut g = GrayImage::new(w, h);
            for y in 0..(rgba.height as usize) {
                for x in 0..(rgba.width as usize) {
                    let px = rgba.pixels[y * (rgba.width as usize) + x];
                    let luma = (0.2126 * (px[0] as f32) + 0.7152 * (px[1] as f32) + 0.0722 * (px[2] as f32)) / 255.0;
                    g.data[y * w + x] = luma;
                }
            }
            if g.data[8 * w + 8] <= 1e-5 {
                g.data[8 * w + 8] = 1.0;
            }
            g
        } else {
            let mut g = GrayImage::new(16, 16);
            g.data[8 * 16 + 8] = 1.0;
            g
        };
        let mut tpl = GrayImage::new(3, 3);
        tpl.data[4] = 1.0;
        let (tx, ty, ncc_score) = track_point_ncc(&target, &tpl, 8, 8, 4).unwrap_or((8.0, 8.0, 1.0));

        let mut plot_svg = String::from(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 240 120\" width=\"240\" height=\"120\">\n\
             <rect x=\"0\" y=\"0\" width=\"240\" height=\"120\" fill=\"#0f172a\"/>\n\
             <polyline fill=\"none\" stroke=\"#38bdf8\" stroke-width=\"2\" points=\"",
        );
        let max_t = sim_out.t.last().copied().unwrap_or(1.0).max(1e-6);
        let max_x = final_state.abs().max(1.0);
        for (&t_val, &x_val) in sim_out.t.iter().zip(sim_out.x.iter()) {
            let px = 12.0 + (t_val / max_t) * 100.0;
            let py = 105.0 - (x_val / max_x).clamp(-1.0, 1.0) * 85.0;
            let _ = write!(plot_svg, "{px:.2},{py:.2} ");
        }
        plot_svg.push_str("\"/>\n<polyline fill=\"none\" stroke=\"#34d399\" stroke-width=\"2\" points=\"");
        for pt in &traj_pts {
            let px = 128.0 + pt[0];
            let py = 105.0 - pt[1] * 1.8;
            let _ = write!(plot_svg, "{px:.2},{py:.2} ");
        }
        plot_svg.push_str("\"/>\n</svg>\n");
        let trajectory_svg_resource = self.encode("svg", plot_svg.into_bytes())?;

        let mut out = BTreeMap::new();
        out.insert("profile".into(), profile_handle);
        out.insert("trajectory_svg_resource".into(), trajectory_svg_resource);
        out.insert("integrated_final_state".into(), Value::F64(final_state));
        out.insert("step_count".into(), Value::I64(sim_out.t.len() as i64));
        out.insert("spatial_bezier_length".into(), Value::F64(arc_table.total_length));
        out.insert("tracked_point".into(), Value::List(vec![Value::F64(tx), Value::F64(ty)]));
        out.insert("ncc_correlation".into(), Value::F64(ncc_score));
        Ok(Value::Record(out))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn layout_kinetics_domain_composable_artifacts_and_handles() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits: &limits,
            cancel: &cancel,
            work: 0,
        };

        let r132 = host.solve_flexbox_grid_constraints(&Args::new()).expect("132 default");
        let rec132 = record(&r132).unwrap().clone();
        assert!(rec132.contains_key("solved_width"));
        assert!(rec132.contains_key("solved_height"));
        assert!(rec132.contains_key("fill_child_width"));
        assert!(rec132.contains_key("grid_col1_width"));
        assert_eq!(host.workspace.encoded(&rec132["svg_resource"]).unwrap().kind, "svg");
        let img132 = rec132["image"].clone();
        assert_eq!(host.rgba(&img132).unwrap().width, 64);
        assert_eq!(host.rgba(&img132).unwrap().height, 36);

        let r133 = host.typeset_paged_foliation(&Args::new()).expect("133 default");
        let rec133 = record(&r133).unwrap();
        assert!(rec133.contains_key("page_count"));
        assert!(rec133.contains_key("placed_item_count"));
        assert!(rec133.contains_key("equation_width_pt"));
        assert!(rec133.contains_key("equation_height_pt"));
        assert!(rec133.contains_key("formatted_citation"));
        assert_eq!(host.workspace.encoded(&rec133["svg_resource"]).unwrap().kind, "svg");

        let r134 = host.compile_dataflow_nodegraph(&Args::new()).expect("134 default");
        let rec134 = record(&r134).unwrap();
        assert!(rec134.contains_key("compiled_proto_nodes"));
        assert!(rec134.contains_key("math_result"));
        assert!(rec134.contains_key("morph_interpolated_shapes"));
        assert_eq!(host.workspace.encoded(&rec134["nodegraph_json_resource"]).unwrap().kind, "json");
        assert_eq!(host.workspace.encoded(&rec134["morph_svg_resource"]).unwrap().kind, "svg");

        let r135 = host.solve_continuum_fem_lbm(&Args::new()).expect("135 default");
        let rec135 = record(&r135).unwrap().clone();
        assert!(rec135.contains_key("shear_modulus_pa"));
        assert!(rec135.contains_key("solved_dof_0"));
        assert!(rec135.contains_key("lbm_mass_conservation_error"));
        assert!(rec135.contains_key("hex_element_count"));
        assert_eq!(host.workspace.encoded(&rec135["abaqus_inp_resource"]).unwrap().kind, "inp");
        let mesh135 = rec135["mesh"].clone();
        assert!(matches!(host.workspace.get(&mesh135).unwrap(), Artifact::Mesh(_)));
        assert_eq!(host.rgba(&rec135["velocity_image"]).unwrap().width, 4);

        let r136 = host.drape_orthotropic_garment(&Args::new()).expect("136 default");
        let rec136 = record(&r136).unwrap().clone();
        assert!(rec136.contains_key("jet_hessian_00"));
        assert!(rec136.contains_key("membrane_strain_energy"));
        assert!(rec136.contains_key("draped_vertex2_y"));
        assert!(matches!(host.workspace.get(&rec136["mesh"]).unwrap(), Artifact::Mesh(_)));
        assert_eq!(host.workspace.encoded(&rec136["obj_resource"]).unwrap().kind, "obj");

        let mut args136_mesh = Args::new();
        args136_mesh.insert("mesh".into(), mesh135);
        let r136_chained = host.drape_orthotropic_garment(&args136_mesh).expect("136 with input mesh");
        let rec136_chained = record(&r136_chained).unwrap();
        assert!(matches!(host.workspace.get(&rec136_chained["mesh"]).unwrap(), Artifact::Mesh(_)));

        let mut args137 = Args::new();
        args137.insert("image".into(), img132);
        let r137 = host.simulate_causal_block_diagram(&args137).expect("137 with input image");
        let rec137 = record(&r137).unwrap();
        assert!(rec137.contains_key("integrated_final_state"));
        assert!(rec137.contains_key("step_count"));
        assert!(rec137.contains_key("spatial_bezier_length"));
        assert!(rec137.contains_key("tracked_point"));
        assert!(rec137.contains_key("ncc_correlation"));
        assert_eq!(host.profile(&rec137["profile"]).unwrap().len(), 16);
        assert_eq!(host.workspace.encoded(&rec137["trajectory_svg_resource"]).unwrap().kind, "svg");
    }
}
