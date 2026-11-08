//! # Categorical & Ontological Contracts for Layered Surface Codecs
//!
//! Provides static 5-field parity-verified `NQuin48` triples, `FuncDescriptor`, and
//! `AgenticMorphismSpec` definitions for VibeScript hypermedia operations under:
//! - `hypermedia.surface.psd_transcode`
//! - `hypermedia.surface.affinity_inspect`
//! - `hypermedia.surface.open_raster_decode`
//!
//! Grounded in international human rights instruments (UDHR Articles 12, 18, 19, 27; ICCPR Articles 17, 19;
//! ICESCR Article 15) and Informatics Fiduciary duties. Zero legacy suite naming.

use serde::{Deserialize, Serialize};

/// 48-byte Super-Quin representing an ontological relationship.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NQuin48 {
    pub subject: u64,
    pub predicate: u64,
    pub object: u64,
    pub context: u64,
    pub metadata: u64,
    pub parity: u64,
}

impl NQuin48 {
    pub const fn new(subject: u64, predicate: u64, object: u64, context: u64, metadata: u64) -> Self {
        let parity = subject ^ predicate ^ object ^ context ^ metadata;
        Self {
            subject,
            predicate,
            object,
            context,
            metadata,
            parity,
        }
    }

    pub fn verify_parity(&self) -> bool {
        self.parity == (self.subject ^ self.predicate ^ self.object ^ self.context ^ self.metadata)
    }
}

pub const fn q_hash(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
        i += 1;
    }
    hash
}

pub const HAS_PRECONDITION: u64 = q_hash(b"hypermedia:ontology:hasPrecondition");
pub const HAS_POSTCONDITION: u64 = q_hash(b"hypermedia:ontology:hasPostcondition");
pub const PRESERVES_INVARIANT: u64 = q_hash(b"hypermedia:ontology:preservesInvariant");
pub const FIDUCIARY_CONTEXT: u64 = q_hash(b"context:informatics_fiduciary:udhr_art27_icescr_art15");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OntologicalContract {
    pub triples: [NQuin48; 3],
}

impl OntologicalContract {
    pub const fn new(operation_iri: &[u8], pre: &[u8], post: &[u8], inv: &[u8]) -> Self {
        let op = q_hash(operation_iri);
        let t1 = NQuin48::new(op, HAS_PRECONDITION, q_hash(pre), FIDUCIARY_CONTEXT, 0x01);
        let t2 = NQuin48::new(op, HAS_POSTCONDITION, q_hash(post), FIDUCIARY_CONTEXT, 0x02);
        let t3 = NQuin48::new(op, PRESERVES_INVARIANT, q_hash(inv), FIDUCIARY_CONTEXT, 0x03);
        Self { triples: [t1, t2, t3] }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuncDescriptor {
    pub identifier: String,
    pub doc: String,
    pub contract: OntologicalContract,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgenticMorphismSpec {
    pub domain: String,
    pub codomain: String,
    pub morphism_kind: String,
    pub teleology: String,
    pub units_and_frames: String,
    pub composable_with: Vec<String>,
    pub human_preview_affordance: String,
}

pub fn get_morphism_specs() -> Vec<AgenticMorphismSpec> {
    vec![
        AgenticMorphismSpec {
            domain: "surface:LayeredPsdStream".to_string(),
            codomain: "surface:DocumentComposition".to_string(),
            morphism_kind: "Isomorphism".to_string(),
            teleology: "Lossless round-trip encoding and decoding of multi-layer PSD/PSB documents preserving clipping, blending, and mask semantics under UDHR Art. 27".to_string(),
            units_and_frames: "Channels: [i16; N], Coordinates: 32/64-bit integer pixels, Opacity: [0..255] discrete level".to_string(),
            composable_with: vec![
                "hypermedia.surface.raster_mask_feather".to_string(),
                "hypermedia.surface.blend_if_evaluate".to_string(),
                "hypermedia.radiance.icc_pcs_transform".to_string(),
            ],
            human_preview_affordance: "Composite canvas rendering and interactive per-layer visibility toggle".to_string(),
        },
        AgenticMorphismSpec {
            domain: "surface:AffinityContainerStream".to_string(),
            codomain: "surface:ContainerMetadataIndex".to_string(),
            morphism_kind: "Projection".to_string(),
            teleology: "Bounded safe container FAT traversal and thumbnail preview extraction for Affinity document assets without unbounded memory allocation".to_string(),
            units_and_frames: "Offsets: 64-bit unsigned bytes, Preview dimensions: 32-bit discrete pixels".to_string(),
            composable_with: vec![
                "hypermedia.surface.psd_transcode".to_string(),
                "hypermedia.chronicle.catalog_journal".to_string(),
            ],
            human_preview_affordance: "Embedded PNG thumbnail preview and document save revision history view".to_string(),
        },
        AgenticMorphismSpec {
            domain: "surface:OpenRasterPackage".to_string(),
            codomain: "surface:LayerTreeManifest".to_string(),
            morphism_kind: "Morphism".to_string(),
            teleology: "Interoperable open raster layer stack parsing and composite preview validation under ICESCR Art. 15".to_string(),
            units_and_frames: "Canvas dimensions: 32-bit pixels, Layer offsets: [x: i32, y: i32], Opacity: [0.0..1.0] continuous".to_string(),
            composable_with: vec![
                "hypermedia.surface.raster_mask_feather".to_string(),
                "hypermedia.radiance.icc_pcs_transform".to_string(),
            ],
            human_preview_affordance: "Full-resolution mergedimage.png preview with SVG compositing stack tree".to_string(),
        },
    ]
}

pub fn get_descriptors() -> Vec<FuncDescriptor> {
    vec![
        FuncDescriptor {
            identifier: "hypermedia.surface.psd_transcode".to_string(),
            doc: "Encodes and decodes multi-layer Photoshop PSD/PSB documents with PackBits compression and blend modes".to_string(),
            contract: OntologicalContract::new(
                b"hypermedia.surface.psd_transcode",
                b"ValidPsdHeaderAndChannelDescriptors",
                b"ExactPlanarPixelReconstruction",
                b"PreservesLayerGeometryAndBlendModeParity",
            ),
        },
        FuncDescriptor {
            identifier: "hypermedia.surface.affinity_inspect".to_string(),
            doc: "Traverses Affinity suite container allocation tables and extracts embedded PNG thumbnails".to_string(),
            contract: OntologicalContract::new(
                b"hypermedia.surface.affinity_inspect",
                b"AffinityMagicAndFatBoundaryVerified",
                b"CompleteAssetManifestAndPreviewMetadata",
                b"StrictDecompressionMemoryCeilingMaintained",
            ),
        },
        FuncDescriptor {
            identifier: "hypermedia.surface.open_raster_decode".to_string(),
            doc: "Parses OpenRaster (ORA) ZIP package layer stack and extracts merged raster preview".to_string(),
            contract: OntologicalContract::new(
                b"hypermedia.surface.open_raster_decode",
                b"ValidZipContainerWithStackXml",
                b"HierarchicalLayerManifestExtracted",
                b"UniversalOpenStandardCompatibility",
            ),
        },
    ]
}
