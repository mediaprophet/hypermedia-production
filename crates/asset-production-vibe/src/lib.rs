//! Bounded, composable adapters over the existing production libraries.
//! This module owns handles and admission, not geometry/image/audio algorithms.
mod acoustics_discourse_chronicle_ext;
mod acoustics_ops;
mod chronicle_ops;
mod discourse_docs;
pub mod domain_extensions;
mod electromagnetics_geodesy_ext;
mod electromagnetics_ops;
mod geodesy_ops;
mod geometry;
mod glyph;
mod glyph_layout_diegetic;
mod graphics;
mod kinetics_ops;
mod kinetics_sim;
mod layout_ops;
pub mod living_world_anatomy;
mod locus_generative;
mod locus_ops;
mod manifold_fields_sdf;
mod manifold_ops;
mod manifold_topology;
pub mod ontology;
mod operations_catalog;
mod preview;
mod radiance_color;
mod radiance_compositor_gi;
mod radiance_photo;
mod raster;
mod sound;
mod surface_bsdf_uv;

pub use operations_catalog::OPERATIONS;

/// An intact retained input or encoded output; the application owns persistence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resource {
    pub key: String,
    pub kind: String,
    pub bytes: Vec<u8>,
}

/// Logical payload/work admission. Applications may impose stricter budgets.
#[derive(Clone, Debug)]
pub struct ToolLimits {
    pub resources: usize,
    pub resource_bytes: usize,
    pub total_resource_bytes: usize,
    pub asset_work_units: u64,
}
impl Default for ToolLimits {
    fn default() -> Self {
        Self {
            resources: 512,
            resource_bytes: 16 * 1024 * 1024,
            total_resource_bytes: 64 * 1024 * 1024,
            asset_work_units: 50_000_000,
        }
    }
}
use qualia_core_db::render::assets::Mesh;
use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use vibe::Value;

pub fn catalogue() -> serde_json::Value {
    serde_json::json!({
        "schema":"hypermedia.asset-operations/v1",
        "contract":"1",
        "naming_paradigm":"category-theoretic",
        "invocation":"capability.invoke",
        "declaration":"requires [capability(\"capability.invoke\")];",
        "export":"let asset = {files:[{key:\"output.ext\",resource:encodedHandle}]};",
        "ontology": ontology::categories_json(),
        "operations": ontology::MORPHISMS.iter().map(ontology::morphism_to_json).collect::<Vec<_>>()
    })
}
pub fn arguments(id: &str) -> &'static str {
    ontology::resolve_morphism(id).map(|m| m.arguments).unwrap_or("")
}

#[derive(Clone)]
pub struct Raster {
    width: u32,
    height: u32,
    rgb: Vec<u8>,
}
#[derive(Clone)]
pub enum Artifact {
    Profile(Vec<[f64; 2]>),
    Mesh(Mesh),
    Image(Raster),
    Rgba(graphics::Rgba),
    Audio { rate: u32, samples: Vec<f32> },
    Encoded(Resource),
}
impl Artifact {
    fn kind(&self) -> &str {
        match self {
            Self::Profile(_) => "profile",
            Self::Mesh(_) => "mesh",
            Self::Image(_) => "image",
            Self::Rgba(_) => "rgba",
            Self::Audio { .. } => "audio",
            Self::Encoded(r) => &r.kind,
        }
    }
    fn bytes(&self) -> usize {
        match self {
            Self::Profile(p) => p.len() * 16,
            Self::Mesh(m) => (m.positions.len() + m.triangles.len()) * 12,
            Self::Image(r) => r.rgb.len(),
            Self::Rgba(r) => r.pixels.len() * 4,
            Self::Audio { samples, .. } => samples.len() * 4,
            Self::Encoded(r) => r.bytes.len(),
        }
    }
}
/// Immutable artifacts are shared across tentative revisions. Failed source cells
/// never publish their workspace; old handles continue to refer to old artifacts.
#[derive(Clone, Default)]
pub struct AssetWorkspace {
    entries: Vec<Arc<Artifact>>,
    bytes: usize,
}
impl AssetWorkspace {
    pub fn encoded(&self, value: &Value) -> Result<&Resource, String> {
        match self.get(value)? {
            Artifact::Encoded(r) => Ok(r),
            _ => Err("Output requires an encoded resource handle".into()),
        }
    }
    fn get(&self, value: &Value) -> Result<&Artifact, String> {
        let r = record(value)?;
        let index = integer(required(r, "handle")?, "handle")?;
        let item = self.entries.get(index).ok_or("Unknown production handle")?;
        if text(required(r, "type")?)? != item.kind() {
            return Err("Production handle type mismatch".into());
        }
        Ok(item)
    }
    fn insert(&mut self, artifact: Artifact, limits: &ToolLimits) -> Result<Value, String> {
        let bytes = artifact.bytes();
        let total = self
            .bytes
            .checked_add(bytes)
            .ok_or("Workspace size overflow")?;
        if self.entries.len() >= limits.resources
            || total > limits.total_resource_bytes
            || bytes > limits.resource_bytes
        {
            return Err("Production workspace exceeds artifact/byte budget".into());
        }
        let handle = self.entries.len();
        let kind = artifact.kind().to_owned();
        self.entries.push(Arc::new(artifact));
        self.bytes = total;
        Ok(Value::Record(BTreeMap::from([
            ("handle".into(), Value::U64(handle as u64)),
            ("type".into(), Value::String(kind)),
        ])))
    }
}
pub struct AssetHost<'a> {
    pub workspace: AssetWorkspace,
    pub inputs: &'a [Resource],
    pub limits: &'a ToolLimits,
    pub cancel: &'a AtomicBool,
    pub work: u64,
}
impl AssetHost<'_> {
    fn charge(&mut self, work: usize, scratch: usize) -> Result<(), String> {
        if self.cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        self.work = self
            .work
            .checked_add(work as u64)
            .ok_or("Production work overflow")?;
        // Logical work and scratch bounds, not an OS memory guarantee.
        if self.work > self.limits.asset_work_units || scratch > self.limits.total_resource_bytes {
            return Err("Production operation exceeds work/scratch budget".into());
        }
        Ok(())
    }
    fn encode(&mut self, kind: &str, bytes: Vec<u8>) -> Result<Value, String> {
        self.admit_encoded(kind, bytes)
    }

    /// Admit bytes the production host already holds. The files producer accepts the handle.
    pub fn admit_encoded(&mut self, kind: &str, bytes: Vec<u8>) -> Result<Value, String> {
        self.workspace.insert(
            Artifact::Encoded(Resource {
                key: String::new(),
                kind: kind.into(),
                bytes,
            }),
            self.limits,
        )
    }
    pub fn invoke(&mut self, id: &str, value: &Value) -> Result<Value, String> {
        let mark = (self.workspace.entries.len(), self.workspace.bytes);
        let result = self.invoke_inner(id, value);
        if result.is_err() {
            self.workspace.entries.truncate(mark.0);
            self.workspace.bytes = mark.1;
        }
        result
    }
    fn invoke_inner(&mut self, id: &str, value: &Value) -> Result<Value, String> {
        let Some(canonical_id) = ontology::canonical_morphism_id(id) else {
            return Err(format!("Production capability not admitted: {id}"));
        };
        self.charge(1, 0)?;
        let args = record(value)?;
        let result = if matches!(
            canonical_id,
            "hypermedia.locus.solve_parametric_sketch"
                | "hypermedia.locus.pathfinder_warp_envelope"
                | "hypermedia.locus.transcode_geospatial_vector"
                | "hypermedia.manifold.sweep_assembly_profile"
                | "hypermedia.manifold.compile_bim_ifc4"
                | "hypermedia.manifold.erode_geomorphic_heightfield"
                | "hypermedia.surface.bake_pbr_materialx"
                | "hypermedia.surface.expand_procedural_brush"
                | "hypermedia.radiance.develop_camera_raw_hdr"
                | "hypermedia.radiance.grade_cdl_scopes"
                | "hypermedia.radiance.trace_vector_contours"
                | "hypermedia.glyph.compose_knuth_plass_paragraph"
                | "hypermedia.layout.solve_flexbox_grid_constraints"
                | "hypermedia.layout.typeset_paged_foliation"
                | "hypermedia.layout.compile_dataflow_nodegraph"
                | "hypermedia.kinetics.solve_continuum_fem_lbm"
                | "hypermedia.kinetics.drape_orthotropic_garment"
                | "hypermedia.kinetics.simulate_causal_block_diagram"
                | "hypermedia.kinetics.schedule_cpm_evm_montecarlo"
                | "hypermedia.acoustics.analyze_ebu_r128_camelot_daw"
                | "hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa"
                | "hypermedia.geodesy.kriging_variogram_morphometry"
                | "hypermedia.discourse.audit_tex_pdf_ua_matlang"
                | "hypermedia.chronicle.recalculate_tabular_fiduciary_archive"
        ) {
            self.domain_extension_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.manifold.compile_botanical_organ_10d"
                | "hypermedia.manifold.compile_vertebrate_anatomy_10d"
                | "hypermedia.kinetics.rig_biomechanical_archetype"
                | "hypermedia.discourse.query_anatomical_ontology_pin"
                | "hypermedia.geodesy.step_sector_continuum_3d"
                | "hypermedia.layout.synchronize_companion_frustum"
                | "qualia.species.assemble_corpus"
        ) {
            self.living_world_anatomy_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.locus.fillet"
                | "hypermedia.locus.resample"
                | "hypermedia.locus.trim"
                | "hypermedia.locus.bishop_rmf_cyclic"
                | "hypermedia.locus.shortest_edge_path"
        ) {
            self.locus_generative_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.locus.spline"
                | "hypermedia.locus.offset"
                | "hypermedia.locus.boolean"
                | "hypermedia.locus.stroke_to_locus"
        ) {
            self.locus_ext(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.manifold.radial_euler"
                | "hypermedia.manifold.bevel"
                | "hypermedia.manifold.inset"
                | "hypermedia.manifold.solidify"
                | "hypermedia.manifold.bisect_plane"
                | "hypermedia.manifold.subdivide_catmull_clark"
        ) {
            self.manifold_topology_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.manifold.evaluate_field"
                | "hypermedia.manifold.mesh_to_sdf_brick"
                | "hypermedia.manifold.sdf_brick_to_mesh"
                | "hypermedia.manifold.instance_on_points"
                | "hypermedia.manifold.realize_instances"
                | "hypermedia.manifold.fspm_vascular_grow"
        ) {
            self.manifold_fields_sdf_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.manifold.boolean_3d"
                | "hypermedia.manifold.simplify"
                | "hypermedia.manifold.import_mesh"
                | "hypermedia.manifold.project_dxf"
                | "hypermedia.manifold.compile_10d_pbr"
        ) {
            self.manifold_ext(canonical_id, args)
        } else if canonical_id.starts_with("hypermedia.locus.")
            || canonical_id.starts_with("hypermedia.manifold.")
        {
            self.geometry(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.glyph.shape_harfbuzz_bidi"
                | "hypermedia.glyph.pack_msdf_font_atlas"
                | "hypermedia.layout.project_diegetic_surface"
                | "hypermedia.layout.route_input_action"
        ) {
            self.glyph_layout_diegetic_op(canonical_id, args)
        } else if canonical_id.starts_with("hypermedia.glyph.") {
            self.glyph_op(canonical_id, args)
        } else if canonical_id.starts_with("hypermedia.layout.") {
            self.layout_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.electromagnetics.raymarch_vox0_dda"
                | "hypermedia.electromagnetics.project_metamer_nullspace"
                | "hypermedia.geodesy.advect_m64_grid"
                | "hypermedia.geodesy.couple_continuum_xpbd"
        ) {
            self.electromagnetics_geodesy_ext_op(canonical_id, args)
        } else if canonical_id.starts_with("hypermedia.electromagnetics.") {
            self.electromagnetics_op(canonical_id, args)
        } else if canonical_id.starts_with("hypermedia.geodesy.") {
            self.geodesy_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.acoustics.route_bus_dag"
                | "hypermedia.acoustics.spatialize_pga_zone"
                | "hypermedia.discourse.inspect_engine_telemetry"
                | "hypermedia.discourse.verify_fiduciary_rid"
                | "hypermedia.chronicle.capture_deterministic_movie"
                | "hypermedia.chronicle.sync_spatial_motors"
        ) {
            self.acoustics_discourse_chronicle_ext_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.radiance.rasterize_loci"
                | "hypermedia.radiance.convolve_blur"
                | "hypermedia.radiance.blend_rgb"
                | "hypermedia.radiance.transform_exposure"
                | "hypermedia.radiance.morphology_mask"
                | "hypermedia.radiance.encode_rgb_png"
                | "hypermedia.radiance.import_rgb_png"
        ) {
            self.raster(canonical_id, args)
        } else if canonical_id.starts_with("hypermedia.discourse.") {
            self.discourse_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.surface.unwrap_slim"
                | "hypermedia.surface.compute_mikktspace"
                | "hypermedia.surface.compile_bsdf_graph"
                | "hypermedia.surface.sample_procedural_field"
        ) {
            self.surface_bsdf_uv_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.radiance.filter_kuwahara_anisotropic"
                | "hypermedia.radiance.decode_cryptomatte_q42"
                | "hypermedia.radiance.vector_motion_blur"
                | "hypermedia.radiance.solve_sdfgi_froxel"
                | "hypermedia.radiance.build_light_tree"
        ) {
            self.radiance_compositor_gi_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.radiance.modify_mask"
                | "hypermedia.radiance.apply_lut"
                | "hypermedia.radiance.grade_color"
                | "hypermedia.radiance.convert_transfer"
        ) {
            self.radiance_color_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.radiance.develop_photo"
                | "hypermedia.radiance.encode_psd"
                | "hypermedia.radiance.import_psd"
        ) {
            self.radiance_photo_op(canonical_id, args)
        } else if canonical_id.starts_with("hypermedia.radiance.")
            || canonical_id.starts_with("hypermedia.surface.")
        {
            self.graphics(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.kinetics.interpolate_motor_fti"
                | "hypermedia.kinetics.step_island_xpbd"
                | "hypermedia.kinetics.query_direct_space"
                | "hypermedia.kinetics.navigate_funnel_orca"
                | "hypermedia.kinetics.evaluate_nla_channelbag"
                | "hypermedia.kinetics.step_simulation_zone"
        ) {
            self.kinetics_sim_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.kinetics.solve_two_bone_ik"
                | "hypermedia.kinetics.interpolate_keyframes"
                | "hypermedia.kinetics.export_lottie"
        ) {
            self.kinetics_op(canonical_id, args)
        } else if matches!(
            canonical_id,
            "hypermedia.acoustics.filter_biquad"
                | "hypermedia.acoustics.mix_tracks"
                | "hypermedia.acoustics.encode_aac"
        ) {
            self.acoustics_ext(canonical_id, args)
        } else if canonical_id.starts_with("hypermedia.chronicle.") {
            self.chronicle_op(canonical_id, args)
        } else {
            self.sound(canonical_id, args)
        }?;
        self.charge(0, 0)?;
        Ok(result)
    }
}
type Args = BTreeMap<String, Value>;
fn record(v: &Value) -> Result<&Args, String> {
    if let Value::Record(r) = v {
        Ok(r)
    } else {
        Err("Expected a record".into())
    }
}
fn required<'a>(a: &'a Args, k: &str) -> Result<&'a Value, String> {
    a.get(k).ok_or_else(|| format!("Missing {k}"))
}
fn text(v: &Value) -> Result<&str, String> {
    if let Value::String(v) = v {
        Ok(v)
    } else {
        Err("Expected string".into())
    }
}
fn num(v: &Value) -> Result<f64, String> {
    let n = match v {
        Value::F64(n) => *n,
        Value::I64(n) => *n as f64,
        Value::U64(n) => *n as f64,
        _ => return Err("Expected number".into()),
    };
    if !n.is_finite() {
        Err("Expected finite number".into())
    } else {
        Ok(n)
    }
}
fn integer(v: &Value, name: &str) -> Result<usize, String> {
    match v {
        Value::I64(n) => usize::try_from(*n).map_err(|_| format!("Invalid {name}")),
        Value::U64(n) => usize::try_from(*n).map_err(|_| format!("Invalid {name}")),
        _ => Err(format!("{name} requires an integer")),
    }
}
fn number(a: &Args, k: &str, default: f64) -> Result<f64, String> {
    a.get(k).map(num).unwrap_or(Ok(default))
}
fn count(a: &Args, k: &str, default: usize, max: usize) -> Result<usize, String> {
    let n = a.get(k).map(|v| integer(v, k)).unwrap_or(Ok(default))?;
    if n == 0 || n > max {
        Err(format!("{k} outside 1..{max}"))
    } else {
        Ok(n)
    }
}
fn list(v: &Value) -> Result<&[Value], String> {
    if let Value::List(v) = v {
        Ok(v)
    } else {
        Err("Expected list".into())
    }
}
fn boolean(a: &Args, k: &str, default: bool) -> Result<bool, String> {
    match a.get(k) {
        None => Ok(default),
        Some(Value::Bool(v)) => Ok(*v),
        _ => Err(format!("{k} requires boolean")),
    }
}
fn fields(a: &Args, allowed: &[&str]) -> Result<(), String> {
    if let Some(k) = a.keys().find(|k| !allowed.contains(&k.as_str())) {
        Err(format!("Unknown argument: {k}"))
    } else {
        Ok(())
    }
}
fn vector<const N: usize>(v: &Value) -> Result<[f64; N], String> {
    let values = list(v)?;
    if values.len() != N {
        return Err(format!("Expected {N} components"));
    }
    let mut out = [0.; N];
    for (i, v) in values.iter().enumerate() {
        out[i] = num(v)?;
    }
    Ok(out)
}
