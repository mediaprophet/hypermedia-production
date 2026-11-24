//! Bounded, composable adapters over the existing production libraries.
//! This module owns handles and admission, not geometry/image/audio algorithms.
mod acoustics_discourse_chronicle_ext;
mod acoustics_ops;
mod boolean_conforming;
mod chronicle_ops;
mod chronicle_sequence;
mod discourse_docs;
pub mod domain_extensions;
pub mod emf_qac_anatomy_bridge;
mod electromagnetics_geodesy_ext;
mod electromagnetics_ops;
mod electromagnetics_compile;
mod electromagnetics_current;
mod geodesy_ops;
mod geometry;
mod glyph;
mod glyph_layout_diegetic;
mod graphics;
mod graphics_pack;
mod kinetics_ops;
mod kinetics_sim;
mod layout_ops;
pub mod living_world_anatomy;
mod locus_generative;
mod locus_ops;
mod manifold_fields_sdf;
mod native_authenticated_mpc_psi_gate;
mod native_public_print_attribution;
mod native_public_sdfb_attribution;
mod native_poet_event_grant;
mod swarm_completion_spatial;
mod swarm_civic;
mod swarm_fabrication;
mod swarm_geometry;
mod swarm_mechanics;
mod swarm_render;
mod swarm_cosmos;
mod swarm_quantum;
mod swarm_earth;
mod swarm_formal;
mod swarm_formats;
mod swarm_interchange;
mod swarm_privacy;
mod swarm_inverse;
mod swarm_continuum;
mod swarm_audio;
mod kinetics_skeletal_skl2;
mod kinetics_articulation_sweep;
mod kinetics_semantic_solvers;
mod kinetics_shallow_water;
mod kinetics_marine_vehicle;
mod kinetics_navmesh;
mod qualia_cas_stats_modalities_geospatial_network_ops;
mod qualia_audit_exposure_science_gis_econ_geom_vibe_ops;
mod qualia_containers_governance_poet_ops;
mod qualia_deep_biosphere_gtop_cg_ternary_zk_sensing_ops;
mod qualia_instruments_coop_wellfair_qdnf_seal10d_ops;
mod qualia_extended_domain_solver_interchange_ops;
mod manifold_ops;
mod manifold_lod_sections;
mod native_absm;
mod geodesy_ocean_water;
mod native_ephemeris;
mod native_rve_homogenization;
mod native_licensed_10d;
mod native_branch_colonization;
mod native_drafting;
mod electromagnetics_yee;
mod native_cartoon_kinetics;
mod native_cosserat;
mod native_aov_graph;
mod native_attribute_transfer;
mod native_domain_governance;
mod native_geospatial_analysis;
mod native_knowledge_query;
mod native_motion_schedule;
mod native_radiance_scopes;
mod native_raw_hdr;
mod native_sfnt;
mod native_skin_bind;
mod native_structural_fea;
mod native_surface_paint;
mod native_terrain_stages;
mod native_vbd_soft_tissue;
mod native_video_master;
mod native_vision_track;
mod acoustics_continuum_wave;
mod native_sewn_garment;
mod geodesy_ocean_spectrum;
mod native_p64_surrogate;
mod native_spatiotemporal_semiotics;
mod hypermedia_scientific_studio_interchange_ops;
mod hypermedia_earth_civic_optics_laban_icvfx_ops;
mod qualia_bidirectional_roundtrip_twins_ops;
mod qualia_category_sheaf_formal_kernel_ops;
mod qualia_inclusion_trust_sae_ops;
mod qualia_specialist_metrology_focus_workingset_ops;
mod native_room_hoa;
mod primal_replay;
mod native_path_trace;
mod native_pbr_layers;
mod geodesy_volumetric_cloud;
mod native_biome_scatter;
mod compute_wgsl_forge;
mod native_ssm1_affordances;
mod native_persistent_topology;
mod native_visual_hull;
mod native_workbook_cube;
mod manifold_interactive_cutaway;
mod manifold_topology;
mod mesh_edit_policy;
pub mod ontology;
mod operations_catalog;
mod preview;
mod radiance_color;
mod radiance_precision;
mod radiance_compositor_gi;
mod radiance_photo;
mod raster;
mod sound;
mod surface_bsdf_uv;

pub use biosphere_authoring_forge;
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

fn asset_entity_iri(authored: Option<&Value>, encoded_geometry: &[u8]) -> Result<String, String> {
    use sha2::{Digest, Sha256};

    match authored {
        Some(iri) => Ok(text(iri)?.to_owned()),
        None => Ok(format!("urn:sha256:{:x}", Sha256::digest(encoded_geometry))),
    }
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
    Rgba16(radiance_precision::Rgba16),
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
            Self::Rgba16(_) => "rgba16",
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
            Self::Rgba16(r) => r.pixels.len() * 8,
            Self::Audio { samples, .. } => samples.len() * 4,
            Self::Encoded(r) => r.bytes.len(),
        }
    }
}

/// Typed artifacts shared across workspace revisions. Edit a cloned payload
/// with `Arc::make_mut` and admit it as a new handle.
pub mod artifact {
    use std::{collections::BTreeMap, io, mem::size_of_val, ops::Range, sync::Arc};

    use qualia_core_db::{
        bundle::{BundleError, BundleReader},
        container_10d::{
            decode_container_manifold_identity, Container10dHeader, TypedFieldRecord,
            FIELD_KIND_ARCHETYPE_REF, FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE,
            FIELD_KIND_INSTANCE_BATCH, MANIFOLD_IDENTITY_V2_MAX_FIELDS,
        },
        render::{
            assets::ImportedMesh,
            skeletal::{Key, SkinWeight, Track},
        },
    };

    pub use acoustics_daw_session::codecs::AudioBuffer;
    pub use chronicle_tabular_engine::Workbook as TabularFrame;
    pub use layout_nodegraph_compiler::ProtoNetwork as BsdfShaderIr;
    pub use qualia_core_db::container_10d::skeletal_section::SkeletalSectionPayload as Skl2Rig;

    use super::Resource;

    const MAX_10D_BYTES: usize = 128 * 1024 * 1024;

    #[derive(Clone)]
    pub enum Artifact {
        AttributedMesh(Arc<AttributedMesh>),
        SceneGraph10d(Arc<SceneGraph10d>),
        BranchGraph10d(Arc<BranchGraph10d>),
        SkeletalRig(Arc<Skl2Rig>),
        AnimationClip(Arc<AnimClip>),
        FieldLattice(Arc<FieldLattice>),
        MaterialGraph(Arc<BsdfShaderIr>),
        RadianceLattice(Arc<RadianceLattice>),
        AovBundle(Arc<AovBundle>),
        CurveGroomBundle(Arc<CurveGroomBundle>),
        ParticleSet(Arc<ParticleSet>),
        AudioMultiChannel(Arc<AudioBuffer>),
        HmcBundle(Arc<HmcWorkspaceView>),
        ComputeCartridge {
            kind: CartridgeKind,
            bytes: Arc<[u8]>,
        },
        TabularCube(Arc<TabularFrame>),
        StegoPayloadPacket(Arc<StegoBitPacket>),
    }

    #[derive(Clone)]
    pub struct AttributedMesh {
        pub surface: ImportedMesh,
        pub uv1: Vec<[f32; 2]>,
        pub vertex_colors: Vec<[f32; 4]>,
        pub skin_weights: Vec<SkinWeight>,
        pub morph_targets: Arc<[u8]>,
        /// Face-aligned mask for datum and articular surfaces.
        pub protected_faces: Vec<u8>,
    }

    impl AttributedMesh {
        fn bytes(&self) -> usize {
            let surface = &self.surface;
            let image_bytes = surface
                .texture_dependencies
                .iter()
                .fold(0usize, |size, texture| {
                    size.saturating_add(texture.bytes.len())
                        .saturating_add(texture.mime_type.len())
                });
            [
                size_of_val(surface.mesh.positions.as_slice()),
                size_of_val(surface.mesh.triangles.as_slice()),
                size_of_val(surface.normals.as_deref().unwrap_or_default()),
                size_of_val(surface.tangents.as_deref().unwrap_or_default()),
                size_of_val(surface.texture_coordinates_0.as_deref().unwrap_or_default()),
                size_of_val(surface.materials.as_slice()),
                size_of_val(surface.submeshes.as_slice()),
                size_of_val(surface.texture_dependencies.as_slice()),
                image_bytes,
                size_of_val(self.uv1.as_slice()),
                size_of_val(self.vertex_colors.as_slice()),
                size_of_val(self.skin_weights.as_slice()),
                self.morph_targets.len(),
                self.protected_faces.len(),
            ]
            .into_iter()
            .fold(0, usize::saturating_add)
        }
    }

    /// Validated ranges backed by the original shared .10d bytes.
    #[derive(Clone)]
    pub struct TenDFields {
        bytes: Arc<[u8]>,
        fields: Vec<((u16, u64), Range<usize>)>,
    }

    impl TenDFields {
        pub(crate) fn as_10d_bytes(&self) -> &[u8] {
            &self.bytes
        }

        pub fn parse(bytes: Arc<[u8]>) -> Result<Self, String> {
            if bytes.len() > MAX_10D_BYTES {
                return Err("10d field backing exceeds the 128 MiB ring budget".into());
            }
            let header = Container10dHeader::parse(&bytes)
                .map_err(|error| format!("Invalid 10d header: {error:?}"))?;
            let empty = TypedFieldRecord::default();
            let mut scratch = [empty; MANIFOLD_IDENTITY_V2_MAX_FIELDS];
            let mut records = [empty; MANIFOLD_IDENTITY_V2_MAX_FIELDS];
            let (_, count) =
                decode_container_manifold_identity(&bytes, &header, &[], &mut scratch, &mut records)
                    .map_err(|error| format!("Invalid 10d manifold identity: {error:?}"))?;
            let sections = qualia_core_db::container_10d::parse_section_table(&bytes, &header)
                .map_err(|error| format!("Invalid 10d section table: {error:?}"))?;
            let fields = records[..count]
                .iter()
                .filter_map(|record| {
                    let section = sections.iter().find(|section| {
                        u32::from(section.section_type) == record.payload_section_id
                    })?;
                    let start = section.byte_offset as usize + record.payload_offset as usize;
                    let end = start + record.payload_length as usize;
                    Some(((record.kind, record.field_id), start..end))
                })
                .collect();
            let view = Self { bytes, fields };
            if !view
                .fields(FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE)
                .any(|(_, payload)| payload.starts_with(b"HOLO"))
            {
                return Err("10d field backing requires a HOLO field".into());
            }
            Ok(view)
        }

        pub fn fields(&self, kind: u16) -> impl Iterator<Item = (u64, &[u8])> + '_ {
            self.fields
                .iter()
                .filter(move |((field_kind, _), _)| *field_kind == kind)
                .map(|((_, field_id), range)| (*field_id, &self.bytes[range.clone()]))
        }

        fn bytes(&self) -> usize {
            self.bytes.len().saturating_add(size_of_val(self.fields.as_slice()))
        }
    }

    #[derive(Clone)]
    pub struct SceneGraph10d {
        fields: TenDFields,
    }

    impl SceneGraph10d {
        pub fn from_fields(fields: TenDFields) -> Result<Self, String> {
            if fields.fields(FIELD_KIND_ARCHETYPE_REF).next().is_none()
                || fields.fields(FIELD_KIND_INSTANCE_BATCH).next().is_none()
            {
                return Err("10d scene graph requires AREF and INST fields".into());
            }
            Ok(Self { fields })
        }

        pub fn fields(&self) -> &TenDFields {
            &self.fields
        }
    }

    #[derive(Clone)]
    pub struct BranchGraph10d {
        pub nodes: Vec<[f64; 3]>,
        pub edges: Vec<BranchEdge>,
    }

    #[derive(Clone)]
    pub struct BranchEdge {
        pub nodes: [u32; 2],
        pub radius_m: f64,
        pub allometric_exponent: f64,
        pub water_potential_pa: f64,
        pub hydraulic_conductance: f64,
        pub metabolite_concentration: f64,
        pub defense_signal: f64,
    }

    #[derive(Clone)]
    pub struct AnimClip {
        pub duration_seconds: f64,
        pub looping: bool,
        pub tracks: Vec<Track>,
        pub motor_keys: Vec<Key>,
        pub morph_keys: Vec<MorphKey>,
    }

    #[derive(Clone)]
    pub struct MorphKey {
        pub time_seconds: f64,
        pub target_id: u64,
        pub weight: f32,
    }

    #[derive(Clone)]
    pub enum FieldLattice {
        TenD(TenDFields),
        DenseF32 {
            axes: Vec<u32>,
            samples: Vec<f32>,
        },
        M64(Arc<[u8]>),
        Emf(Arc<[u8]>),
    }

    #[derive(Clone)]
    pub struct RadianceLattice {
        pub width: u32,
        pub height: u32,
        pub channels: u8,
        pub primaries_iri: String,
        pub transfer_iri: String,
        pub samples: LatticeSamples,
    }

    #[derive(Clone)]
    pub enum LatticeSamples {
        U8(Vec<u8>),
        U16(Vec<u16>),
        F16Bits(Vec<u16>),
        F32(Vec<f32>),
    }

    impl RadianceLattice {
        fn bytes(&self) -> usize {
            let sample_bytes = match &self.samples {
                LatticeSamples::U8(samples) => samples.len(),
                LatticeSamples::U16(samples) | LatticeSamples::F16Bits(samples) => {
                    size_of_val(samples.as_slice())
                }
                LatticeSamples::F32(samples) => size_of_val(samples.as_slice()),
            };
            sample_bytes
                .saturating_add(self.primaries_iri.len())
                .saturating_add(self.transfer_iri.len())
        }
    }

    #[derive(Clone)]
    pub struct AovBundle {
        pub planes: BTreeMap<String, AovPlane>,
    }

    #[derive(Clone)]
    pub enum AovPlane {
        Planar(Arc<RadianceLattice>),
        /// Per-pixel offsets into a deep sample stream, including its end offset.
        Deep {
            sample_offsets: Vec<u32>,
            samples: Arc<RadianceLattice>,
        },
    }

    #[derive(Clone)]
    pub struct CurveGroomBundle {
        pub strands: Vec<CurveStrand>,
    }

    #[derive(Clone)]
    pub struct CurveStrand {
        pub root_uv: Option<[f32; 2]>,
        pub positions: Vec<[f32; 3]>,
    }

    #[derive(Clone)]
    pub struct ParticleSet {
        pub positions: Vec<[f32; 3]>,
        pub velocities: Vec<[f32; 3]>,
        pub mass_kg: Vec<f32>,
        pub radii_m: Vec<f32>,
    }

    #[derive(Clone)]
    pub enum HmcWorkspaceView {
        Staged(Vec<Resource>),
        Sealed(Arc<[u8]>),
    }

    impl HmcWorkspaceView {
        pub fn reader(&self) -> Option<Result<BundleReader<'_>, BundleError>> {
            match self {
                Self::Staged(_) => None,
                Self::Sealed(bytes) => Some(BundleReader::parse(bytes)),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub enum CartridgeKind {
        P64,
        Np64,
    }

    #[derive(Clone)]
    pub struct StegoBitPacket {
        pub bytes: Arc<[u8]>,
        pub bit_len: usize,
    }

    struct CountBytes {
        bytes: usize,
        limit: usize,
    }

    impl io::Write for CountBytes {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.bytes = self
                .bytes
                .checked_add(bytes.len())
                .filter(|&size| size <= self.limit)
                .ok_or_else(|| io::Error::other("Artifact exceeds byte budget"))?;
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Artifact {
        pub fn kind(&self) -> &'static str {
            match self {
                Self::AttributedMesh(_) => "attributed_mesh",
                Self::SceneGraph10d(_) => "scene_graph_10d",
                Self::BranchGraph10d(_) => "branch_graph_10d",
                Self::SkeletalRig(_) => "skeletal_rig",
                Self::AnimationClip(_) => "animation_clip",
                Self::FieldLattice(_) => "field_lattice",
                Self::MaterialGraph(_) => "material_graph",
                Self::RadianceLattice(_) => "radiance_lattice",
                Self::AovBundle(_) => "aov_bundle",
                Self::CurveGroomBundle(_) => "curve_groom_bundle",
                Self::ParticleSet(_) => "particle_set",
                Self::AudioMultiChannel(_) => "audio_multichannel",
                Self::HmcBundle(_) => "hmc_bundle",
                Self::ComputeCartridge { .. } => "compute_cartridge",
                Self::TabularCube(_) => "tabular_cube",
                Self::StegoPayloadPacket(_) => "stego_payload_packet",
            }
        }

        pub(super) fn bytes(&self, limit: usize) -> Result<usize, String> {
            let mut encoded = CountBytes { bytes: 0, limit };
            let bytes = match self {
                Self::AttributedMesh(mesh) => mesh.bytes(),
                Self::SceneGraph10d(scene) => scene.fields.bytes(),
                Self::BranchGraph10d(graph) => size_of_val(graph.nodes.as_slice())
                    .saturating_add(size_of_val(graph.edges.as_slice())),
                Self::SkeletalRig(rig) => [
                    size_of_val(rig.joints.as_slice()), size_of_val(rig.loop_closures.as_slice()),
                    size_of_val(rig.landmarks.as_slice()), size_of_val(rig.actuation_pathways.as_slice()),
                    size_of_val(rig.ssm_headers.as_slice()), size_of_val(rig.ssm_modes.as_slice()),
                ].into_iter().fold(0, usize::saturating_add),
                Self::AnimationClip(clip) => size_of_val(clip.tracks.as_slice())
                    .saturating_add(size_of_val(clip.motor_keys.as_slice()))
                    .saturating_add(size_of_val(clip.morph_keys.as_slice())),
                Self::FieldLattice(lattice) => match lattice.as_ref() {
                    FieldLattice::TenD(fields) => fields.bytes(),
                    FieldLattice::DenseF32 { axes, samples } => size_of_val(axes.as_slice())
                        .saturating_add(size_of_val(samples.as_slice())),
                    FieldLattice::M64(bytes) | FieldLattice::Emf(bytes) => bytes.len(),
                },
                Self::MaterialGraph(graph) => {
                    ciborium::into_writer(graph.as_ref(), &mut encoded).map_err(|e| e.to_string())?;
                    encoded.bytes
                }
                Self::RadianceLattice(lattice) => lattice.bytes(),
                Self::AovBundle(bundle) => bundle.planes.iter().fold(0usize, |size, (iri, plane)| {
                    let plane_bytes = match plane {
                        AovPlane::Planar(lattice) => lattice.bytes(),
                        AovPlane::Deep { sample_offsets, samples } =>
                            size_of_val(sample_offsets.as_slice()).saturating_add(samples.bytes()),
                    };
                    size.saturating_add(iri.len()).saturating_add(plane_bytes)
                }),
                Self::CurveGroomBundle(bundle) => bundle.strands.iter().fold(
                    size_of_val(bundle.strands.as_slice()),
                    |size, strand| size.saturating_add(size_of_val(strand.positions.as_slice())),
                ),
                Self::ParticleSet(particles) => [
                    size_of_val(particles.positions.as_slice()), size_of_val(particles.velocities.as_slice()),
                    size_of_val(particles.mass_kg.as_slice()), size_of_val(particles.radii_m.as_slice()),
                ].into_iter().fold(0, usize::saturating_add),
                Self::AudioMultiChannel(audio) => audio.channels.iter().fold(
                    size_of_val(audio.channels.as_slice()),
                    |size, channel| size.saturating_add(size_of_val(channel.as_slice())),
                ),
                Self::HmcBundle(bundle) => match bundle.as_ref() {
                    HmcWorkspaceView::Staged(resources) => resources.iter().fold(
                        size_of_val(resources.as_slice()),
                        |size, resource| size.saturating_add(resource.key.len())
                            .saturating_add(resource.kind.len()).saturating_add(resource.bytes.len()),
                    ),
                    HmcWorkspaceView::Sealed(bytes) => bytes.len(),
                },
                Self::ComputeCartridge { bytes, .. } => bytes.len(),
                Self::TabularCube(frame) => {
                    ciborium::into_writer(frame.as_ref(), &mut encoded).map_err(|e| e.to_string())?;
                    encoded.bytes
                }
                Self::StegoPayloadPacket(packet) => {
                    if packet.bit_len.div_ceil(8) != packet.bytes.len() {
                        return Err("Steganographic packet bit length does not match its bytes".into());
                    }
                    packet.bytes.len()
                }
            };
            Ok(bytes)
        }
    }
}

pub use artifact::Artifact as TypedArtifact;

#[derive(Clone)]
enum WorkspaceEntry {
    Legacy(Arc<Artifact>),
    Typed(TypedArtifact),
}

impl WorkspaceEntry {
    fn kind(&self) -> &str {
        match self {
            Self::Legacy(artifact) => artifact.kind(),
            Self::Typed(artifact) => artifact.kind(),
        }
    }
}

/// Immutable artifacts are shared across tentative revisions. Failed source cells
/// never publish their workspace; old handles continue to refer to old artifacts.
#[derive(Clone, Default)]
pub struct AssetWorkspace {
    entries: Vec<WorkspaceEntry>,
    imported: Vec<Option<Arc<qualia_core_db::render::assets::ImportedMesh>>>,
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
        match self.get_entry(value)? {
            WorkspaceEntry::Legacy(item) => Ok(item),
            WorkspaceEntry::Typed(_) => Err("Expected an existing production artifact type".into()),
        }
    }
    pub fn typed(&self, value: &Value) -> Result<&TypedArtifact, String> {
        match self.get_entry(value)? {
            WorkspaceEntry::Typed(item) => Ok(item),
            WorkspaceEntry::Legacy(_) => Err("Expected a typed production artifact".into()),
        }
    }
    fn get_entry(&self, value: &Value) -> Result<&WorkspaceEntry, String> {
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
        self.insert_entry(WorkspaceEntry::Legacy(Arc::new(artifact)), bytes, limits)
    }
    fn insert_entry(
        &mut self,
        entry: WorkspaceEntry,
        bytes: usize,
        limits: &ToolLimits,
    ) -> Result<Value, String> {
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
        let kind = entry.kind().to_owned();
        self.entries.push(entry);
        self.imported.push(None);
        self.bytes = total;
        Ok(Value::Record(BTreeMap::from([
            ("handle".into(), Value::U64(handle as u64)),
            ("type".into(), Value::String(kind)),
        ])))
    }
}

#[cfg(test)]
mod typed_workspace_tests {
    use super::{artifact::ParticleSet, *};

    #[test]
    fn typed_revisions_share_payloads_and_enforce_byte_budgets() {
        let limits = ToolLimits {
            resources: 4,
            resource_bytes: 16,
            total_resource_bytes: 26,
            ..ToolLimits::default()
        };
        let cancel = AtomicBool::new(false);
        let mut host = AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits: &limits,
            cancel: &cancel,
            work: 0,
        };
        let original = Arc::new(ParticleSet {
            positions: vec![[1.0, 2.0, 3.0]],
            velocities: Vec::new(),
            mass_kg: Vec::new(),
            radii_m: Vec::new(),
        });
        let first = host
            .admit_artifact(TypedArtifact::ParticleSet(original.clone()))
            .unwrap();
        let snapshot = host.workspace.clone();
        let TypedArtifact::ParticleSet(shared) = snapshot.typed(&first).unwrap() else {
            panic!("expected a particle workspace handle");
        };
        assert!(Arc::ptr_eq(&original, shared));
        assert_eq!(host.workspace.bytes, 12);

        let mut revision = Arc::clone(shared);
        Arc::make_mut(&mut revision).positions[0][0] = 4.0;
        let second = host
            .admit_artifact(TypedArtifact::ParticleSet(revision.clone()))
            .unwrap();
        assert_eq!(original.positions[0][0], 1.0);
        assert!(snapshot.typed(&second).is_err());
        assert_eq!(host.workspace.bytes, 24);
        assert!(host.admit_artifact(TypedArtifact::ParticleSet(revision)).is_err());
        assert_eq!(host.workspace.bytes, 24);

        let encoded = host.admit_encoded("bin", vec![42, 43]).unwrap();
        assert_eq!(host.workspace.encoded(&encoded).unwrap().bytes, [42, 43]);
        assert!(host.workspace.typed(&encoded).is_err());
        assert!(host.workspace.encoded(&first).is_err());
        assert_eq!(host.workspace.bytes, 26);
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
    /// Admit a typed payload using the same handle and workspace limits as encoded resources.
    pub fn admit_artifact(&mut self, artifact: TypedArtifact) -> Result<Value, String> {
        self.charge(0, 0)?;
        let remaining = self.limits.total_resource_bytes.saturating_sub(self.workspace.bytes);
        let bytes = artifact.bytes(self.limits.resource_bytes.min(remaining))?;
        self.workspace.insert_entry(WorkspaceEntry::Typed(artifact), bytes, self.limits)
    }

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
            self.workspace.imported.truncate(mark.0);
            self.workspace.bytes = mark.1;
        }
        result
    }
    fn invoke_inner(&mut self, id: &str, value: &Value) -> Result<Value, String> {
        let Some(canonical_id) = ontology::canonical_morphism_id(id) else {
            return Err(format!("Production capability not admitted: {id}"));
        };
        self.charge(1, 0)?;
        let original_args = record(value)?;
        let prepared_args = self.prepare_geometry_edit(canonical_id, original_args)?;
        let args = prepared_args.as_ref().unwrap_or(original_args);
        let result = if canonical_id == "hypermedia.radiance.compute_scopes_and_icc" {
            self.compute_scopes_and_icc(args)
        } else if canonical_id == "hypermedia.kinetics.track_planar_and_animate_vector" {
            self.track_planar_and_animate_vector(args)
        } else if canonical_id == "hypermedia.kinetics.simulate_control_blocks_and_cpm" {
            self.simulate_control_blocks_and_cpm(args)
        } else if canonical_id == "hypermedia.manifold.compose_assembly_10d" {
            self.compose_assembly_10d(args)
        } else if canonical_id == "hypermedia.manifold.transfer_attributes" {
            self.transfer_attributes(args)
        } else if canonical_id == "hypermedia.kinetics.evaluate_absm_blendspace" {
            self.evaluate_absm_blendspace(args)
        } else if canonical_id == "hypermedia.chronicle.evaluate_tabular_workbook_cube" {
            self.evaluate_tabular_workbook_cube(args)
        } else if canonical_id == "hypermedia.chronicle.encode_video_master" {
            self.encode_video_master(args)
        } else if canonical_id == "hypermedia.chronicle.decode_video_master" {
            self.decode_video_master(args)
        } else if canonical_id == "qualia.knowledge.query_q42_sparql_and_n3" {
            self.query_q42_sparql_and_n3(args)
        } else if canonical_id == "qualia.knowledge.solve_causal_temporal_rcc8" {
            self.solve_causal_temporal_rcc8(args)
        } else if canonical_id == "hypermedia.chronicle.reconstruct_multiview_3d" {
            self.reconstruct_multiview_3d(args)
        } else if canonical_id == "hypermedia.kinetics.build_navmesh" {
            self.build_navmesh(args)
        } else if canonical_id == "hypermedia.radiance.composite_aov_graph" {
            self.composite_aov_graph(args)
        } else if canonical_id == "hypermedia.geodesy.solve_geomorph_stage" {
            self.solve_geomorph_stage(args)
        } else if canonical_id == "hypermedia.manifold.accumulate_snow_lakes" {
            self.accumulate_snow_lakes(args)
        } else if canonical_id == "hypermedia.kinetics.step_vbd_soft_tissue" {
            self.step_vbd_soft_tissue(args)
        } else if canonical_id == "hypermedia.geodesy.gp_spatial_analyze" {
            self.gp_spatial_analyze(args)
        } else if canonical_id == "hypermedia.kinetics.solve_shallow_water_25d" {
            self.solve_shallow_water_25d(args)
        } else if canonical_id == "qualia.rights.evaluate_agreement_and_pool" {
            self.evaluate_agreement_and_pool(args)
        } else if canonical_id == "hypermedia.radiance.develop_raw_hdr_panorama" {
            self.develop_raw_hdr_panorama(args)
        } else if canonical_id == "hypermedia.surface.paint_3d_texture" {
            self.paint_3d_texture(args)
        } else if canonical_id == "hypermedia.kinetics.bind_skin_and_morphs" {
            self.bind_skin_and_morphs(args)
        } else if canonical_id == "qualia.glyph.shape_opentype_sfnt" {
            self.shape_opentype_sfnt(args)
        } else if canonical_id == "qualia.governance.validate_domain_shacl_and_tk_bc" {
            self.validate_domain_shacl_and_tk_bc(args)
        } else if canonical_id == "hypermedia.kinetics.solve_structural_fea_multiphysics" {
            self.solve_structural_fea_multiphysics(args)
        } else if canonical_id == "hypermedia.electromagnetics.solve_maxwell_yee_3d" {
            self.solve_maxwell_yee_3d(args)
        } else if canonical_id == "hypermedia.kinetics.step_cartoon_kinetics" {
            self.step_cartoon_kinetics(args)
        } else if canonical_id == "hypermedia.kinetics.solve_cosserat_strand" {
            self.solve_cosserat_strand(args)
        } else if canonical_id == "hypermedia.manifold.colonize_branching_graph" {
            self.colonize_branching_graph(args)
        } else if canonical_id == "hypermedia.locus.draft_cad_and_trace_vector" {
            self.draft_cad_and_trace_vector(args)
        } else if canonical_id == "qualia.manifold.seal_10d_with_cbld_licensing" {
            self.seal_10d_with_cbld_licensing(args)
        } else if canonical_id == "hypermedia.geodesy.simulate_ocean_water" {
            self.simulate_ocean_water(args)
        } else if canonical_id == "hypermedia.cosmos.ephemeris_propagate" {
            self.ephemeris_propagate(args)
        } else if canonical_id == "qualia.physics.homogenize_to_continuum" {
            self.homogenize_to_continuum(args)
        } else if canonical_id == "qualia.vision.process_and_track" {
            self.process_and_track(args)
        } else if canonical_id == "hypermedia.acoustics.solve_continuum_wave_3d" {
            self.solve_continuum_wave_3d(args)
        } else if canonical_id == "hypermedia.kinetics.sew_and_drape_garment" {
            self.sew_and_drape_garment(args)
        } else if canonical_id == "hypermedia.geodesy.synthesize_ocean_spectrum" {
            self.synthesize_ocean_spectrum(args)
        } else if canonical_id == "qualia.compute.compile_p64_surrogate_cartridge" {
            self.compile_p64_surrogate_cartridge(args)
        } else if canonical_id == "qualia.linguistics.analyze_spatiotemporal_semiotics" {
            self.analyze_spatiotemporal_semiotics(args)
        } else if canonical_id == "hypermedia.acoustics.room_rir_and_hoa_render" {
            self.room_rir_and_hoa_render(args)
        } else if canonical_id == "hypermedia.surface.node_graph_eval" {
            self.node_graph_eval(args)
        } else if canonical_id == "hypermedia.geodesy.synthesize_volumetric_cloud" {
            self.synthesize_volumetric_cloud(args)
        } else if canonical_id == "hypermedia.manifold.scatter_biome_instances" {
            self.scatter_biome_instances(args)
        } else if canonical_id == "qualia.compute.dispatch_wgsl_forge" {
            self.dispatch_wgsl_forge(args)
        } else if canonical_id == "qualia.manifold.compile_ssm1_and_eont_affordances" {
            self.compile_ssm1_and_eont_affordances(args)
        } else if canonical_id == "qualia.topology.compute_persistent_homology_ddg" {
            self.compute_persistent_homology_ddg(args)
        } else if canonical_id == "hypermedia.radiance.path_trace_scene" {
            self.path_trace_scene(args)
        } else if canonical_id == "qualia.interchange.transcode_scientific_cae_bio_and_qudt" {
            self.transcode_scientific_cae_bio_and_qudt(args)
        } else if canonical_id == "hypermedia.kinetics.synthesize_laban_choreography" {
            self.synthesize_laban_choreography(args)
        } else if canonical_id == "qualia.compute.inspect_and_dequantize_p64" {
            self.inspect_and_dequantize_p64(args)
        } else if canonical_id == "hypermedia.geodesy.export_cog_zarr_netcdf4" {
            self.export_cog_zarr_netcdf4(args)
        } else if canonical_id == "qualia.category.compute_pushout_pullback_and_kan_extension" {
            self.compute_pushout_pullback_and_kan_extension(args)
        } else if canonical_id == "qualia.accessibility.synthesize_iso17049_tactile_braille" {
            self.synthesize_iso17049_tactile_braille(args)
        } else if canonical_id == "qualia.category.solve_adjoint_inverse_morphism" {
            self.solve_adjoint_inverse_morphism(args)
        } else if canonical_id == "hypermedia.kinetics.solve_cfd_flip_smoke" {
            self.solve_cfd_flip_smoke(args)
        } else if canonical_id == "hypermedia.acoustics.master_opus_flac_and_stems" {
            self.master_opus_flac_and_stems(args)
        } else if canonical_id == "qualia.quantum.simulate_openqasm_circuit" {
            self.simulate_openqasm_circuit(args)
        } else if canonical_id == "hypermedia.geodesy.ingest_gis_corpus" {
            self.ingest_gis_corpus(args)
        } else if canonical_id == "hypermedia.geodesy.krige_subsurface_strata_m64" {
            self.krige_subsurface_strata_m64(args)
        } else if canonical_id == "qualia.topology.solve_cellular_sheaf_cohomology" {
            self.solve_cellular_sheaf_cohomology(args)
        } else if canonical_id == "qualia.narrative.solve_linear_logic_world_grammar" {
            self.solve_linear_logic_world_grammar(args)
        } else if canonical_id == "qualia.logic.solve_cdcl_smt_and_egraph_arena" {
            self.solve_cdcl_smt_and_egraph_arena(args)
        } else if canonical_id == "hypermedia.chronicle.transcode_av1_cmaf_mxf" {
            self.transcode_av1_cmaf_mxf(args)
        } else if canonical_id == "hypermedia.chronicle.demux_and_parse_mxf_cmaf" {
            self.demux_and_parse_mxf_cmaf(args)
        } else if canonical_id == "hypermedia.radiance.encode_exr_jxl_ktx2" {
            self.encode_exr_jxl_ktx2(args)
        } else if canonical_id == "hypermedia.radiance.import_exr_jxl_ktx2" {
            self.import_exr_jxl_ktx2(args)
        } else if canonical_id == "hypermedia.manifold.compile_bim_ifc_mep" {
            self.compile_bim_ifc_mep(args)
        } else if canonical_id == "hypermedia.fabrication.unfold_sheet_and_cam_25d" {
            self.unfold_sheet_and_cam_25d(args)
        } else if canonical_id == "qualia.privacy.redact_clinical_and_biometric_phi" {
            self.redact_clinical_and_biometric_phi(args)
        } else if canonical_id == "qualia.accessibility.certify_pse_cvd_and_plain_language" {
            self.certify_pse_cvd_and_plain_language(args)
        } else if canonical_id == "hypermedia.acoustics.synthesize_midi_sf2" {
            self.synthesize_midi_sf2(args)
        } else if canonical_id == "qualia.acoustics.synthesize_articulatory_speech_and_holosonic" {
            self.synthesize_articulatory_speech_and_holosonic(args)
        } else if canonical_id == "hypermedia.manifold.retopologize_quad" {
            self.retopologize_quad(args)
        } else if canonical_id == "hypermedia.manifold.sculpt_multires" {
            self.sculpt_multires(args)
        } else if canonical_id == "hypermedia.surface.groom_hair_and_pelage" {
            self.groom_hair_and_pelage(args)
        } else if canonical_id == "qualia.anatomy.bind_enthesis_and_retinaculum" {
            self.bind_enthesis_and_retinaculum(args)
        } else if canonical_id == "hypermedia.manifold.populate_allometric_vascular_c1" {
            self.populate_allometric_vascular_c1(args)
        } else if canonical_id == "hypermedia.manifold.amplify_terminal_phyllotaxis" {
            self.amplify_terminal_phyllotaxis(args)
        } else if canonical_id == "hypermedia.surface.stylize_npr_cartoon" {
            self.stylize_npr_cartoon(args)
        } else if canonical_id == "hypermedia.radiance.render_gpu_hybrid" {
            self.render_gpu_hybrid(args)
        } else if canonical_id == "qualia.rights.issue_pq_vc_and_agent_state" {
            self.issue_pq_vc_and_agent_state(args)
        } else if canonical_id == "qualia.physics.couple_exergetic_port_hamiltonian_operad" {
            self.couple_exergetic_port_hamiltonian_operad(args)
        } else if canonical_id == "qualia.category.compose_parametric_optics_crdc" {
            self.compose_parametric_optics_crdc(args)
        } else if canonical_id == "hypermedia.kinetics.solve_tribology_ehl_and_wear" {
            self.solve_tribology_ehl_and_wear(args)
        } else if canonical_id == "hypermedia.kinetics.solve_4dvar_fluid_and_contact_adjoint" {
            self.solve_4dvar_fluid_and_contact_adjoint(args)
        } else if canonical_id == "hypermedia.kinetics.solve_free_surface_3d" {
            self.solve_free_surface_3d(args)
        } else if canonical_id == "hypermedia.cosmos.synth_planetary_body" {
            self.synth_planetary_body(args)
        } else if canonical_id == "hypermedia.cosmos.render_planetary_limb_sky" {
            self.render_planetary_limb_sky(args)
        } else if canonical_id == "hypermedia.cosmos.synth_planetary_ring" {
            self.synth_planetary_ring(args)
        } else if canonical_id == "hypermedia.cosmos.step_space_weather_aurora" {
            self.step_space_weather_aurora(args)
        } else if canonical_id == "hypermedia.cosmos.eval_nbody_transfer" {
            self.eval_nbody_transfer(args)
        } else if canonical_id == "hypermedia.cosmos.deep_space_rf_link" {
            self.deep_space_rf_link(args)
        } else if canonical_id == "hypermedia.manifold.step_cad_brep" {
            self.step_cad_brep(args)
        } else if canonical_id == "hypermedia.manifold.export_step_ap242_brep" {
            self.export_step_ap242_brep(args)
        } else if canonical_id == "hypermedia.surface.simulate_weathering_and_wear" {
            self.simulate_weathering_and_wear(args)
        } else if canonical_id == "hypermedia.interchange.transcode_openusd_openvdb_alembic" {
            self.transcode_openusd_openvdb_alembic(args)
        } else if canonical_id == "hypermedia.fabrication.reconstruct_toolpath_to_solid" {
            self.reconstruct_toolpath_to_solid(args)
        } else if canonical_id == "qualia.rights.seal_qecp_and_work_graph" {
            self.seal_qecp_and_work_graph(args)
        } else if canonical_id == "hypermedia.fabrication.route_pcb_and_wire_harness" {
            self.route_pcb_and_wire_harness(args)
        } else if canonical_id == "qualia.infrastructure.solve_microgrid_and_lca_commons" {
            self.solve_microgrid_and_lca_commons(args)
        } else if canonical_id == "hypermedia.control.bridge_iec61131_plc_and_ros2_hil" {
            self.bridge_iec61131_plc_and_ros2_hil(args)
        } else if canonical_id == "hypermedia.infrastructure.solve_multimodal_transit_and_crowd" {
            self.solve_multimodal_transit_and_crowd(args)
        } else if canonical_id == "hypermedia.infrastructure.solve_hydraulic_pipe_and_stormwater" {
            self.solve_hydraulic_pipe_and_stormwater(args)
        } else if canonical_id == "hypermedia.chronicle.calibrate_icvfx_led_volume_stage" {
            self.calibrate_icvfx_led_volume_stage(args)
        } else if canonical_id == "hypermedia.interchange.transcode_eda_spice_touchstone_odb" {
            self.transcode_eda_spice_touchstone_odb(args)
        } else if canonical_id == "hypermedia.fabrication.certify_dfm_manifold" {
            self.certify_dfm_manifold(args)
        } else if canonical_id == "hypermedia.fabrication.synth_tpms_lattice" {
            self.synth_tpms_lattice(args)
        } else if canonical_id == "hypermedia.fabrication.orient_nest_tree_support" {
            self.orient_nest_tree_support(args)
        } else if canonical_id == "hypermedia.fabrication.slice_and_toolpath" {
            self.slice_and_toolpath(args)
        } else if canonical_id == "hypermedia.fabrication.export_print_package" {
            self.export_print_package(args)
        } else if canonical_id == "hypermedia.fabrication.certify_cmm_metrology_gdt" {
            self.certify_cmm_metrology_gdt(args)
        } else if canonical_id == "hypermedia.surface.transcode_layered_raster" {
            self.transcode_layered_raster(args)
        } else if canonical_id == "qualia.logic.check_cic_dependent_proof_kernel" {
            self.check_cic_dependent_proof_kernel(args)
        } else if canonical_id == "qualia.environment.step_fire_and_soil_coupled" {
            self.step_fire_and_soil_coupled(args)
        } else if canonical_id == "hypermedia.radiance.solve_mueller_polarimetry_and_thin_film" {
            self.solve_mueller_polarimetry_and_thin_film(args)
        } else if canonical_id == "hypermedia.radiance.synthesize_cgh_wave_diffraction" {
            self.synthesize_cgh_wave_diffraction(args)
        } else if canonical_id == "hypermedia.radiance.bake_refractive_caustics" {
            self.bake_refractive_caustics(args)
        } else if canonical_id == "qualia.anatomy.validate_topological_body" {
            self.validate_topological_body(args)
        } else if canonical_id == "qualia.botany.step_fspm_vascular_and_leaf" {
            self.step_fspm_vascular_and_leaf(args)
        } else if canonical_id == "hypermedia.kinetics.solve_pyro_buoyancy_3d" {
            self.solve_pyro_buoyancy_3d(args)
        } else if canonical_id == "hypermedia.kinetics.solve_granular_mpm" {
            self.solve_granular_mpm(args)
        } else if canonical_id == "hypermedia.cosmos.raymarch_relativistic_disk" {
            self.raymarch_relativistic_disk(args)
        } else if canonical_id == "hypermedia.fabrication.split_joinery_assembly" {
            self.split_joinery_assembly(args)
        } else if canonical_id == "qualia.anvil.attest_and_route_np64" {
            self.attest_and_route_np64(args)
        } else if canonical_id == "qualia.sensing.simulate_hardware_sensor_twin" {
            self.simulate_hardware_sensor_twin(args)
        } else if canonical_id == "qualia.storage.offload_zns_csd_and_qubo_commons" {
            self.offload_zns_csd_and_qubo_commons(args)
        } else if canonical_id == "qualia.provenance.encode_creation_tree_payload" {
            self.encode_creation_tree_payload(args)
        } else if canonical_id == "qualia.provenance.decode_stego_and_verify_lineage" {
            self.decode_stego_and_verify_lineage(args)
        } else if canonical_id == "qualia.physics.solve_piezoelectric_multiferroic_coupling" {
            self.solve_piezoelectric_multiferroic_coupling(args)
        } else if canonical_id == "hypermedia.fabrication.simulate_molding_casting_and_composites" {
            self.simulate_molding_casting_and_composites(args)
        } else if canonical_id == "hypermedia.kinetics.solve_compliant_soft_robotics_origami" {
            self.solve_compliant_soft_robotics_origami(args)
        } else if canonical_id == "hypermedia.geodesy.solve_glaciology_and_permafrost" {
            self.solve_glaciology_and_permafrost(args)
        } else if canonical_id == "hypermedia.geodesy.simulate_seismology_and_tsunami_coupling" {
            self.simulate_seismology_and_tsunami_coupling(args)
        } else if canonical_id == "qualia.environment.solve_limnology_estuarine_biogeochemistry" {
            self.solve_limnology_estuarine_biogeochemistry(args)
        } else if canonical_id == "hypermedia.infrastructure.solve_geotechnical_slope_and_tunnel" {
            self.solve_geotechnical_slope_and_tunnel(args)
        } else if canonical_id == "hypermedia.radiance.raytrace_multi_element_optical_lens" {
            self.raytrace_multi_element_optical_lens(args)
        } else if canonical_id == "qualia.provenance.verify_microstructure_puf_fuzzy" {
            self.verify_microstructure_puf_fuzzy(args)
        } else if canonical_id == "qualia.accessibility.synthesize_sigml_sign_language_avatar" {
            self.synthesize_sigml_sign_language_avatar(args)
        } else if canonical_id == "hypermedia.interchange.transcode_musicxml_and_mei_scores" {
            self.transcode_musicxml_and_mei_scores(args)
        } else if canonical_id == "hypermedia.manifold.fracture_voronoi_3d" {
            self.fracture_voronoi_3d(args)
        } else if canonical_id == "hypermedia.layout.paginate_foliation_and_pdf_ua" {
            self.paginate_foliation_and_pdf_ua(args)
        } else if canonical_id == "qualia.physics.solve_boltzmann_radiation_and_bateman" {
            self.solve_boltzmann_radiation_and_bateman(args)
        } else if canonical_id == "qualia.environment.solve_atmospheric_photochemistry_dispersion" {
            self.solve_atmospheric_photochemistry_dispersion(args)
        } else if canonical_id == "hypermedia.acoustics.solve_inverse_rir_prb_and_bss" {
            self.solve_inverse_rir_prb_and_bss(args)
        } else if canonical_id == "hypermedia.geodesy.solve_elastic_fwi_adjoint_tomography" {
            self.solve_elastic_fwi_adjoint_tomography(args)
        } else if canonical_id == "qualia.medical.ingest_and_reconstruct_dicom" {
            self.ingest_and_reconstruct_dicom(args)
        } else if canonical_id == "qualia.matter.solve_m64_thermochem_electro" {
            self.solve_m64_thermochem_electro(args)
        } else if canonical_id == "hypermedia.fabrication.plan_cnc_subtractive_multiaxis" {
            self.plan_cnc_subtractive_multiaxis(args)
        } else if canonical_id == "qualia.anatomy.synthesize_invertebrate_and_ergonomic_10d" {
            self.synthesize_invertebrate_and_ergonomic_10d(args)
        } else if canonical_id == "qualia.sensing.bridge_etalon_nfc_radio_and_fusion" {
            self.bridge_etalon_nfc_radio_and_fusion(args)
        } else if canonical_id == "hypermedia.radiance.solve_adaptive_optics_and_nonlinear_nlse" {
            self.solve_adaptive_optics_and_nonlinear_nlse(args)
        } else if canonical_id == "qualia.ai.interpret_p64_sae_and_renyi_dp_federation" {
            self.interpret_p64_sae_and_renyi_dp_federation(args)
        } else if canonical_id == "qualia.crypto.attest_vdf_roughtime_and_pq_threshold" {
            self.attest_vdf_roughtime_and_pq_threshold(args)
        } else if canonical_id == "hypermedia.radiance.solve_adjoint_prb_inverse_rendering" {
            self.solve_adjoint_prb_inverse_rendering(args)
        } else if canonical_id == "qualia.quantum.derive_hellmann_feynman_force_field" {
            self.derive_hellmann_feynman_force_field(args)
        } else if canonical_id == "hypermedia.layout.decompile_tagged_pdf_ua" {
            self.decompile_tagged_pdf_ua(args)
        } else if canonical_id == "qualia.quantum.solve_dft_scf" {
            self.solve_dft_scf(args)
        } else if canonical_id == "hypermedia.electromagnetics.solve_vlasov_maxwell_pic_plasma" {
            self.solve_vlasov_maxwell_pic_plasma(args)
        } else if canonical_id == "qualia.quantum.solve_tdgl_cryogenic_superconductivity" {
            self.solve_tdgl_cryogenic_superconductivity(args)
        } else if canonical_id == "hypermedia.manifold.build_cluster_dag_and_impostor" {
            self.build_cluster_dag_and_impostor(args)
        } else if canonical_id == "hypermedia.electromagnetics.trace_rf_multipath_and_radar" {
            self.trace_rf_multipath_and_radar(args)
        } else if canonical_id == "qualia.chemistry.solve_reaction_network" {
            self.solve_reaction_network(args)
        } else if canonical_id == "qualia.quantum.evaluate_bio_coherence" {
            self.evaluate_bio_coherence(args)
        } else if canonical_id == "qualia.vision.extract_biometrics_and_pose468" {
            self.extract_biometrics_and_pose468(args)
        } else if canonical_id == "qualia.chemistry.invert_flux_balance_and_rates" {
            self.invert_flux_balance_and_rates(args)
        } else if canonical_id == "qualia.chemistry.step_molecular_dynamics" {
            self.step_molecular_dynamics(args)
        } else if canonical_id == "hypermedia.electromagnetics.solve_adjoint_fdtd_inverse_design" {
            self.solve_adjoint_fdtd_inverse_design(args)
        } else if canonical_id == "qualia.medical.segment_volume_to_10d" {
            self.segment_volume_to_10d(args)
        } else if canonical_id == "qualia.medical.analyze_histopathology_and_radiomics" {
            self.analyze_histopathology_and_radiomics(args)
        } else if canonical_id == "qualia.crypto.prove_zk_deontic_pq_and_sanctuary" {
            self.prove_zk_deontic_pq_and_sanctuary(args)
        } else if canonical_id == "qualia.bio.solve_pbpk_pd_adme_clearance" {
            self.solve_pbpk_pd_adme_clearance(args)
        } else if canonical_id == "qualia.biosphere.step_funga_and_trophic_web" {
            self.step_funga_and_trophic_web(args)
        } else if canonical_id == "qualia.anatomy.simulate_pathology_regime" {
            self.simulate_pathology_regime(args)
        } else if canonical_id == "qualia.neuro.solve_hodgkin_huxley_connectome_gtop" {
            self.solve_hodgkin_huxley_connectome_gtop(args)
        } else if canonical_id == "qualia.anatomy.synthesize_organ_system_10d" {
            self.synthesize_organ_system_10d(args)
        } else if canonical_id == "qualia.wellfare.evaluate_clinical_burden_and_vault" {
            self.evaluate_clinical_burden_and_vault(args)
        } else if canonical_id == "qualia.anatomy.incise_resect_and_suture" {
            self.incise_resect_and_suture(args)
        } else if canonical_id == "qualia.poet.bind_pulse_aura_hypermedia" {
            self.bind_pulse_aura_hypermedia(args)
        } else if canonical_id == "qualia.medical.export_dicom_seg_rtstruct" {
            self.export_dicom_seg_rtstruct(args)
        } else if canonical_id == "qualia.biology.fold_macromolecule_and_cell_sheet" {
            self.fold_macromolecule_and_cell_sheet(args)
        } else if canonical_id == "qualia.bio.solve_grn_epigenetic_morphogenesis" {
            self.solve_grn_epigenetic_morphogenesis(args)
        } else if canonical_id == "hypermedia.fabrication.embed_digital_mesh_stego" {
            self.embed_digital_mesh_stego(args)
        } else if canonical_id == "qualia.biosphere.step_funga_microbial_trophic_coupling" {
            self.step_funga_microbial_trophic_coupling(args)
        } else if canonical_id == "hypermedia.fabrication.embed_physical_print_stego" {
            self.embed_physical_print_stego(args)
        } else if canonical_id == "qualia.bio.simulate_immunology_wound_and_seir_network" {
            self.simulate_immunology_wound_and_seir_network(args)
        } else if canonical_id == "qualia.crypto.evaluate_zk_mpc_and_vole_psi" {
            self.evaluate_zk_mpc_and_vole_psi(args)
        } else if canonical_id == "hypermedia.electromagnetics.reconstruct_sar_isar_tomography" {
            self.reconstruct_sar_isar_tomography(args)
        } else if canonical_id == "hypermedia.manifold.measure_semantic_sections" {
            self.measure_semantic_sections(args)
        } else if canonical_id == "qualia.accessibility.compile_scene_focus_graph" {
            self.compile_scene_focus_graph(args)
        } else if canonical_id == "qualia.container.plan_hmc_semantic_working_set" {
            self.plan_hmc_semantic_working_set(args)
        } else if canonical_id == "qualia.provenance.rebase_hmc_review_anchors" {
            self.rebase_hmc_review_anchors(args)
        } else if canonical_id == "qualia.accessibility.audit_affordance_equivalence" {
            self.audit_affordance_equivalence(args)
        } else if canonical_id == "hypermedia.manifold.verify_semantic_lod_transition" {
            self.verify_semantic_lod_transition(args)
        } else if canonical_id == "hypermedia.chronicle.bind_timed_semantic_cues" {
            self.bind_timed_semantic_cues(args)
        } else if canonical_id == "hypermedia.discourse.bind_evidence_fragments" {
            self.bind_evidence_fragments(args)
        } else if canonical_id == "hypermedia.radiance.plan_semantic_viewpoints" {
            self.plan_semantic_viewpoints(args)
        } else if canonical_id == "hypermedia.kinetics.retarget_semantic_clip" {
            self.retarget_semantic_clip(args)
        } else if canonical_id == "qualia.container.pack_and_diff_hmc" {
            self.pack_and_diff_hmc(args)
        } else if canonical_id == "qualia.math.solve_symbolic_cas_and_ode" {
            self.solve_symbolic_cas_and_ode(args)
        } else if canonical_id == "qualia.statistics.evaluate_inference_and_econometrics" {
            self.evaluate_inference_and_econometrics(args)
        } else if canonical_id == "qualia.knowledge.evaluate_epistemic_jural_and_blackboard" {
            self.evaluate_epistemic_jural_and_blackboard(args)
        } else if canonical_id == "qualia.epistemics.audit_research_and_grounding" {
            self.audit_research_and_grounding(args)
        } else if canonical_id == "qualia.geospatial.synthesize_corridor_and_isogloss" {
            self.synthesize_corridor_and_isogloss(args)
        } else if canonical_id == "qualia.inference.compile_conditioning_and_q42_journal" {
            self.compile_conditioning_and_q42_journal(args)
        } else if canonical_id == "hypermedia.live.author_dmx_hbbtv_and_portal_world" {
            self.author_dmx_hbbtv_and_portal_world(args)
        } else if canonical_id == "qualia.network.sync_solid_pod_and_qdnf_peer" {
            self.sync_solid_pod_and_qdnf_peer(args)
        } else if canonical_id == "hypermedia.physics.homogenize_multiphase_and_cartoon_kinetics" {
            self.homogenize_multiphase_and_cartoon_kinetics(args)
        } else if canonical_id == "qualia.topology.harbour_gtop_geometry_graphs" {
            self.harbour_gtop_geometry_graphs(args)
        } else if canonical_id == "qualia.graph.prove_equivalence_scrub_and_rdf_star" {
            self.prove_equivalence_scrub_and_rdf_star(args)
        } else if canonical_id == "qualia.twin.evaluate_ethogram_and_entity_affordance" {
            self.evaluate_ethogram_and_entity_affordance(args)
        } else if canonical_id == "qualia.geometry.advanced_cg_arrangements_and_meshing" {
            self.advanced_cg_arrangements_and_meshing(args)
        } else if canonical_id == "qualia.inference.transcode_p64_ternary_and_sieve" {
            self.transcode_p64_ternary_and_sieve(args)
        } else if canonical_id == "qualia.governance.evaluate_semantic_instruments_and_tk_bc" {
            self.evaluate_semantic_instruments_and_tk_bc(args)
        } else if canonical_id == "qualia.cooperative.evaluate_nine_stage_lifecycle_and_abac" {
            self.evaluate_nine_stage_lifecycle_and_abac(args)
        } else if canonical_id == "qualia.anatomy.seal_cardiac_m64_emf_and_work_graph" {
            self.seal_cardiac_m64_emf_and_work_graph(args)
        } else if canonical_id == "qualia.wellfair.evaluate_personal_health_and_guardianship" {
            self.evaluate_personal_health_and_guardianship(args)
        } else if canonical_id == "qualia.science.validate_c02_passivity_and_oracles" {
            self.validate_c02_passivity_and_oracles(args)
        } else if canonical_id == "qualia.geometry.repair_polygon_soup_and_ddg_poisson" {
            self.repair_polygon_soup_and_ddg_poisson(args)
        } else if canonical_id == "qualia.q42.encode_spatial_predicates_and_situated_place_time" {
            self.encode_spatial_predicates_and_situated_place_time(args)
        } else if canonical_id == "qualia.solvers.evaluate_graph_opt_active_learning_and_special_fns" {
            self.evaluate_graph_opt_active_learning_and_special_fns(args)
        } else if canonical_id == "hypermedia.acoustics.bridge_qac_ultrasound_haptic_and_bioacoustics" {
            self.bridge_qac_ultrasound_haptic_and_bioacoustics(args)
        } else if canonical_id == "qualia.economics.solve_macro_market_portfolio_and_engineering" {
            self.solve_macro_market_portfolio_and_engineering(args)
        } else if canonical_id == "qualia.geospatial.evaluate_gis_gp_terrain_and_datacubes" {
            self.evaluate_gis_gp_terrain_and_datacubes(args)
        } else if canonical_id == "qualia.vision.super_resolve_and_compile_10d_observations" {
            self.super_resolve_and_compile_10d_observations(args)
        } else if canonical_id == "qualia.wellfair.enact_safeguards_dead_mans_and_accountability" {
            self.enact_safeguards_dead_mans_and_accountability(args)
        } else if canonical_id == "qualia.vibe.decompile_merge_and_legislate_cof" {
            self.decompile_merge_and_legislate_cof(args)
        } else if canonical_id == "hypermedia.manifold.seal_leaf_assembly_10d" {
            self.seal_leaf_assembly_10d(args)
        } else if canonical_id == "hypermedia.manifold.configure_interactive_cutaway" {
            self.configure_interactive_cutaway(args)
        } else if canonical_id == "hypermedia.kinetics.compile_skl2_rig" {
            self.compile_skl2_rig(args)
        } else if canonical_id == "hypermedia.kinetics.simulate_vehicle_aero_hydro_6dof" {
            self.simulate_vehicle_aero_hydro_6dof(args)
        } else if canonical_id == "hypermedia.kinetics.solve_multibody_dca_ik" {
            self.solve_multibody_dca_ik(args)
        } else if canonical_id == "hypermedia.kinetics.solve_homeostatic_graph_flux" {
            self.solve_homeostatic_graph_flux(args)
        } else if canonical_id == "hypermedia.kinetics.bake_aeroelastic_wind" {
            self.bake_aeroelastic_wind(args)
        } else if canonical_id == "hypermedia.kinetics.fit_and_verify_articulation_sweep" {
            self.fit_and_verify_articulation_sweep(args)
        } else if canonical_id == "qualia.network.dispatch_qdnf_ilp_ebpf_and_os_services" {
            self.dispatch_qdnf_ilp_ebpf_and_os_services(args)
        } else if canonical_id == "hypermedia.chronicle.interchange_aaf_omf_mxf_and_yin_midi" {
            self.interchange_aaf_omf_mxf_and_yin_midi(args)
        } else if canonical_id == "qualia.sensing.optimize_csi_privshield_and_bvp" {
            self.optimize_csi_privshield_and_bvp(args)
        } else if canonical_id == "qualia.geospatial.erode_multires_rivers_and_ce_droplets" {
            self.erode_multires_rivers_and_ce_droplets(args)
        } else if canonical_id == "qualia.cooperative.reconcile_landed_cost_retainage_and_web_gis" {
            self.reconcile_landed_cost_retainage_and_web_gis(args)
        } else if canonical_id == "hypermedia.kinetics.solve_explicit_dynamic_fem_and_buckling" {
            self.solve_explicit_dynamic_fem_and_buckling(args)
        } else if canonical_id == "hypermedia.vector.compute_rolling_ball_concave_hull_and_offset" {
            self.compute_rolling_ball_concave_hull_and_offset(args)
        } else if canonical_id == "qualia.discourse.ingest_office_smartart_xlsb_ods_and_formcalc" {
            self.ingest_office_smartart_xlsb_ods_and_formcalc(args)
        } else if canonical_id == "hypermedia.image.segment_slic_grabcut_sky_and_roto_matte" {
            self.segment_slic_grabcut_sky_and_roto_matte(args)
        } else if canonical_id == "hypermedia.kinetics.filter_gpu_vbd_contact_and_seam_links" {
            self.filter_gpu_vbd_contact_and_seam_links(args)
        } else if canonical_id == "hypermedia.image.roundtrip_psd_smart_filters_and_multi_app_brushes" {
            self.roundtrip_psd_smart_filters_and_multi_app_brushes(args)
        } else if canonical_id == "hypermedia.manifold.bake_gaussian_surfels_10d" {
            self.bake_gaussian_surfels_10d(args)
        } else if matches!(
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
                | "hypermedia.manifold.calibrate_biological_subject"
                | "hypermedia.kinetics.evaluate_mobility_fracture_risk"
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
                | "hypermedia.manifold.fit_holographic_v2"
                | "hypermedia.manifold.seal_sdf_brick_10d"
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
                | "hypermedia.manifold.loft"
        ) {
            let produced = self.manifold_ext(canonical_id, args)?;
            if canonical_id == "hypermedia.manifold.compile_10d_pbr" {
                self.consume_unwrap_uv01(args, produced)
            } else {
                Ok(produced)
            }
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

    /// When unwrap sealed UV01 into `.10d`, public compile_10d_pbr consumes it.
    fn consume_unwrap_uv01(&mut self, a: &Args, compiled: Value) -> Result<Value, String> {
        let Some(src) = a.get("source_10d") else {
            return Ok(compiled);
        };
        let src_bytes = self.workspace.encoded(src)?.bytes.clone();
        let nv = match self.workspace.get(required(a, "mesh")?)? {
            Artifact::Mesh(m) => m.positions.len(),
            _ => {
                return Err("source_10d UV01 consume requires a mesh handle".into());
            }
        };
        if nv == 0 {
            return Err("source_10d UV01 consume requires a non-empty mesh".into());
        }
        let mut uvs = vec![[0.0f32; 2]; nv];
        let n = qualia_core_db::container_10d::read_texture_coordinate_section_into(
            &src_bytes, nv, &mut uvs,
        )
        .map_err(|e| format!("source_10d UV01 consume failed: {e:?}"))?;
        if n != nv {
            return Err(format!(
                "source_10d UV01 count {n} != mesh vertex count {nv}"
            ));
        }
        let compiled_bytes = self.workspace.encoded(&compiled)?.bytes.clone();
        let mut existing = vec![[0.0f32; 2]; nv];
        if let Ok(n) = qualia_core_db::container_10d::read_texture_coordinate_section_into(
            &compiled_bytes,
            nv,
            &mut existing,
        ) {
            if n != nv {
                return Err(format!(
                    "compiled UV01 count {n} != mesh vertex count {nv}"
                ));
            }
            return Ok(compiled);
        }
        let with_uv = qualia_core_db::container_10d::attach_texture_coordinate_section(
            &compiled_bytes,
            &uvs,
        )
        .map_err(|e| format!("UV01 attach failed: {e:?}"))?;
        let mut resealed = self.encode("10d", with_uv)?;
        if let (Value::Record(old), Value::Record(new)) = (&compiled, &mut resealed) {
            if let Some(textures) = old.get("textures") {
                new.insert("textures".into(), textures.clone());
            }
        }
        Ok(resealed)
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
