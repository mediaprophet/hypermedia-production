//! Category-theoretic & ontological taxonomy, `vibescript-abi` provider descriptors,
//! 5-field parity-sealed `NQuin48` ontology triples, and `AgenticUtility` metadata.
//!
//! Every asset production operation is modeled as a pure, bounded morphism
//! `f: Domain -> Codomain` within a mathematical/physical category rather than a
//! legacy desktop-application "craft" namespace.

use serde_json::{json, Value};
use vibescript_abi::{FuncDescriptor, ParamSpec, PrimitiveKind, ResourceEnvelope};

/// Top-level mathematical/physical categories of the Hypermedia & QualiaDB asset ontology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CategoryDescriptor {
    pub id: &'static str,
    pub sub_id: u16,
    pub iri: &'static str,
    pub object_space: &'static str,
    pubmorphism_purpose: &'static str,
    pub replaces_legacy_terms: &'static [&'static str],
}

pub const CATEGORIES: &[CategoryDescriptor] = &[
    CategoryDescriptor {
        id: "locus",
        sub_id: 1,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Locus",
        object_space: "Locus1D, Locus2D, PathComplex, ConstraintSystem",
        pubmorphism_purpose: "1D/2D parametric and planar geometric loci, Bézier splines, path booleans, offsets, sketch constraint solving, and SVG/DXF projection",
        replaces_legacy_terms: &["vectorcraft", "profile", "drafting"],
    },
    CategoryDescriptor {
        id: "manifold",
        sub_id: 2,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Manifold",
        object_space: "SurfaceManifold3D, SolidManifold3D, Manifold10D",
        pubmorphism_purpose: "3D/10D spatial & solid manifolds, extrusion, revolution, lofting, CSG mesh booleans, UV atlas parameterization, and .10d container sealing",
        replaces_legacy_terms: &["cadcraft", "mesh"],
    },
    CategoryDescriptor {
        id: "surface",
        sub_id: 3,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Surface",
        object_space: "TangentFrameField, MaterialSheaf, ProceduralField",
        pubmorphism_purpose: "Intrinsic surface tangent frames, procedural material node graphs, PBR albedo/normal/ORM map synthesis, and 3D mesh map baking",
        replaces_legacy_terms: &["materialcraft", "texturecraft"],
    },
    CategoryDescriptor {
        id: "radiance",
        sub_id: 4,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Radiance",
        object_space: "RadianceLattice2D, LayerStack, MaskLattice, ColorSpace",
        pubmorphism_purpose: "2D spectral, linear-light, and straight-alpha RGBA lattices, layer composition, morphological masks, 3D LUTs, RAW development, and PNG/PSD codecs",
        replaces_legacy_terms: &["photocraft", "lightcraft", "image", "rgba"],
    },
    CategoryDescriptor {
        id: "glyph",
        sub_id: 5,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Glyph",
        object_space: "TypefaceManifold, ShapedGlyphRun, TextMetrics",
        pubmorphism_purpose: "OpenType font table parsing, HarfBuzz/HarfRust glyph shaping, outline extraction, kerning/metrics, and glyph run rendering",
        replaces_legacy_terms: &["font-kit", "typography"],
    },
    CategoryDescriptor {
        id: "layout",
        sub_id: 6,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Layout",
        object_space: "BoxTree, ConstraintLayout, ScopedThemeTokens",
        pubmorphism_purpose: "Flex and grid spatial box-tree layout solving, responsive viewport constraints, and Webizen scoped theme/state sheet synthesis",
        replaces_legacy_terms: &["figcraft", "ui-layout"],
    },
    CategoryDescriptor {
        id: "kinetics",
        sub_id: 7,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Kinetics",
        object_space: "TimebaseRational, TrajectoryCurve,KinematicChain, SceneGraph",
        pubmorphism_purpose: "Rational frame-time lattices, keyframe/spring trajectories, skeletal rigs, FABRIK/CCD inverse kinematics, and Lottie/pose compilation",
        replaces_legacy_terms: &["effectcraft", "timeline", "animation"],
    },
    CategoryDescriptor {
        id: "acoustics",
        sub_id: 8,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Acoustics",
        object_space: "PressureWave1D, SpectralFrame, MixBusGraph",
        pubmorphism_purpose: "Acoustic wave synthesis, biquad/compressor/reverb DSP, multi-track bus mixing, and PCM WAV/AAC/Opus codecs",
        replaces_legacy_terms: &["soundcraft", "audio"],
    },
    CategoryDescriptor {
        id: "electromagnetics",
        sub_id: 9,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Electromagnetics",
        object_space: "EmfVolume5D, StokesPolarization, SpectralDistribution",
        pubmorphism_purpose: "4D/5D wave-optical and electromagnetic field lattices, Poynting flux, Stokes polarization, Q42EMF64 (.emf) sealing, and spectral slice previews",
        replaces_legacy_terms: &["emf-sim", "optical-field"],
    },
    CategoryDescriptor {
        id: "geodesy",
        sub_id: 10,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Geodesy",
        object_space: "HeightfieldTile, GeodesicPatch, MatterBrickM64",
        pubmorphism_purpose: "Planetary/terrain elevation lattices, reversible sculpt/stamp/smooth brush morphisms, and .m64 matter brick compilation",
        replaces_legacy_terms: &["terrain-craft", "world-sculpt"],
    },
    CategoryDescriptor {
        id: "discourse",
        sub_id: 11,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Discourse",
        object_space: "FoliatedDocument, ProseTree, SlideDeck, RelationalGrid, ReviewGallery",
        pubmorphism_purpose: "Structured rhetorical, tabular, and archival projections: PDF foliation, DOCX prose, PPTX slide decks, XLSX/SVG charts, and self-contained HTML review galleries",
        replaces_legacy_terms: &["wordcraft", "deckcraft", "gridcraft", "poet-pdf", "preview"],
    },
    CategoryDescriptor {
        id: "chronicle",
        sub_id: 12,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Chronicle",
        object_space: "TemporalPacketStream, IsobmffContainer",
        pubmorphism_purpose: "Synchronized audio-visual packet streams, H.264/VP9/AV1 video encoding, and ISOBMFF (.mp4) container muxing",
        replaces_legacy_terms: &["filmcraft", "video-mux"],
    },
];

/// Agentic utility and ontological grounding for a single asset production morphism.
#[derive(Debug, Clone, Copy)]
pub struct AgenticMorphismSpec {
    /// Canonical categorical identifier (`hypermedia.<category>.<morphism>`).
    pub id: &'static str,
    /// Historical `hypermedia.*` or `synth.*` compatibility aliases that resolve to `id`.
    pub aliases: &'static [&'static str],
    /// Category identifier (`locus`, `manifold`, `surface`, `radiance`, etc.).
    pub category: &'static str,
    /// Morphism short name within the category.
    pub morphism: &'static str,
    /// Underlying owner crate/module in `hypermedia-production`, `qualia-core-db`, or `etalon`.
    pub owner: &'static str,
    /// Concise human & agent summary.
    pub summary: &'static str,
    /// Formal domain object signature.
    pub domain: &'static str,
    /// Formal codomain object signature.
    pub codomain: &'static str,
    /// Category-theoretic morphism classification (`Constructor`, `Endomorphism`, `Functor`, `NaturalTransformation`, `CoalgebricProjection`, `AlgebraicFold`).
    pub morphism_kind: &'static str,
    /// Teleological grounding: why an agent invokes this morphism and what real-world/UI problem it solves.
    pub teleology: &'static str,
    /// Physical units, coordinate conventions, and color/numerical invariants.
    pub units_and_frames: &'static str,
    /// Argument schema specification.
    pub arguments: &'static str,
    /// Upstream/downstream canonical morphism IDs that naturally compose with this morphism.
    pub composable_with: &'static [&'static str],
    /// How human reviewers and agents visually/auditorily inspect the codomain.
    pub human_preview_affordance: &'static str,
    /// VibeScript ABI function descriptor with 5-field parity `NQuin48` spans.
    pub abi_descriptor: FuncDescriptor,
}

pub(super) const DEFAULT_PARAMS: &[ParamSpec] = &[ParamSpec::slice("args_record", PrimitiveKind::U8, 0, u32::MAX)];
pub(super) const DEFAULT_ENVELOPE: ResourceEnvelope = ResourceEnvelope {
    max_ticks: 50_000_000,
    max_scratch_bytes: 64 * 1024 * 1024,
    max_vram_bytes: 0,
    max_output_bytes: 16 * 1024 * 1024,
    max_spawned_tasks: 0,
    flags: 0,
};

macro_rules! morphism_func {
    ($name:expr, $iri:expr, $sub_id:expr, $func_id:expr, $sig:expr, $span_idx:expr) => {
        FuncDescriptor::const_new(
            $name,
            $iri,
            StableFuncId::new($sub_id, $func_id),
            $sig,
            DEFAULT_PARAMS,
            PrimitiveKind::U64,
            1,
            EffectSet::PURE_COMPUTE,
            DeterminismGuarantee::DeterministicExact,
            ReversibilityKind::PureNoState,
            DEFAULT_ENVELOPE,
            ($span_idx * 3, 1),
            ($span_idx * 3 + 1, 1),
            ($span_idx * 3 + 2, 1),
        )
    };
}


mod morphisms_01;
mod morphisms_02;
mod morphisms_03;
mod morphisms_04;
mod morphisms_05;
mod morphisms_06;
mod morphisms_07;
mod morphisms_08;
mod morphisms_09;
mod nquins;
mod vibe_descriptors;

#[cfg(test)]
mod tests;

pub use nquins::ONTOLOGY_NQUINS;
pub use vibe_descriptors::{HypermediaOntologyProvider, HYPERMEDIA_PROVIDER_DESCRIPTOR};

const ALL_MORPHISMS: [AgenticMorphismSpec; 148] = {
    let mut out = [morphisms_01::BATCH[0]; 148];
    let batches: [&[AgenticMorphismSpec]; 9] = [
        morphisms_01::BATCH,
        morphisms_02::BATCH,
        morphisms_03::BATCH,
        morphisms_04::BATCH,
        morphisms_05::BATCH,
        morphisms_06::BATCH,
        morphisms_07::BATCH,
        morphisms_08::BATCH,
        morphisms_09::BATCH,
    ];
    let mut idx = 0;
    let mut b = 0;
    while b < batches.len() {
        let slice = batches[b];
        let mut i = 0;
        while i < slice.len() {
            out[idx] = slice[i];
            idx += 1;
            i += 1;
        }
        b += 1;
    }
    assert!(idx == 148);
    out
};

/// Complete ontological & agentic utility specification for all admitted morphisms.
pub static MORPHISMS: &[AgenticMorphismSpec] = &ALL_MORPHISMS;

/// Resolves any canonical categorical ID or historical `hypermedia.*`/`synth.*` alias
/// to its `AgenticMorphismSpec`.
pub fn resolve_morphism(id: &str) -> Option<&'static AgenticMorphismSpec> {
    MORPHISMS
        .iter()
        .find(|m| m.id == id || m.aliases.contains(&id))
}

/// Resolves any operation identifier to its canonical categorical ID (`hypermedia.<category>.<morphism>`).
pub fn canonical_morphism_id(id: &str) -> Option<&'static str> {
    resolve_morphism(id).map(|m| m.id)
}

fn hex_bytes(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

/// Serializes the category-theoretic ontology taxonomy for agentic discovery (`synth.ontology.categories`).
pub fn categories_json() -> Value {
    json!({
        "schema": "hypermedia.categorical-ontology/v1",
        "provider_iri": HYPERMEDIA_PROVIDER_DESCRIPTOR.canonical_iri.iri,
        "provider_alias": HYPERMEDIA_PROVIDER_DESCRIPTOR.short_alias,
        "naming_paradigm": "category-theoretic (Objects + Morphisms f: Domain -> Codomain; zero -craft/legacy product names)",
        "total_ontology_nquins": ONTOLOGY_NQUINS.len(),
        "categories": CATEGORIES.iter().map(|c| {
            let count = MORPHISMS.iter().filter(|m| m.category == c.id).count();
            json!({
                "id": c.id,
                "sub_namespace_id": c.sub_id,
                "iri": c.iri,
                "object_space": c.object_space,
                "morphism_purpose": c.pubmorphism_purpose,
                "replaces_legacy_terms": c.replaces_legacy_terms,
                "admitted_morphism_count": count,
            })
        }).collect::<Vec<_>>(),
    })
}

/// Serializes a single `AgenticMorphismSpec` including its `OntologicalContract` and `NQuin48` parity hashes.
pub fn morphism_to_json(m: &AgenticMorphismSpec) -> Value {
    let f = &m.abi_descriptor;
    let c = &f.contract;
    let pre_q = &ONTOLOGY_NQUINS[c.precondition_quin_span.0 as usize];
    let post_q = &ONTOLOGY_NQUINS[c.postcondition_quin_span.0 as usize];
    let inv_q = &ONTOLOGY_NQUINS[c.invariant_quin_span.0 as usize];
    json!({
        "id": m.id,
        "aliases": m.aliases,
        "category": m.category,
        "morphism": m.morphism,
        "owner": m.owner,
        "summary": m.summary,
        "arguments": m.arguments,
        "agentic_utility": {
            "domain": m.domain,
            "codomain": m.codomain,
            "morphism_kind": m.morphism_kind,
            "teleology": m.teleology,
            "units_and_frames": m.units_and_frames,
            "composable_with": m.composable_with,
            "human_preview_affordance": m.human_preview_affordance,
        },
        "ontological_contract": {
            "canonical_iri": f.canonical_iri.iri,
            "iri_fnv1a_hash": format!("{:016x}", f.canonical_iri.fnv1a_hash),
            "stable_id": {
                "sub_namespace_id": f.stable_id.sub_namespace_id,
                "func_id": f.stable_id.func_id,
            },
            "contract_digest_sha256": hex_bytes(&c.contract_digest),
            "schema_hash": format!("{:016x}", c.schema_hash),
            "effects": "PURE_COMPUTE",
            "determinism": "BitExact",
            "reversibility": "Irreversible",
            "envelope": {
                "max_ticks": f.envelope.max_ticks,
                "max_scratch_bytes": f.envelope.max_scratch_bytes,
                "max_output_bytes": f.envelope.max_output_bytes,
            },
            "nquin48_triples": {
                "precondition": {
                    "span": [c.precondition_quin_span.0, c.precondition_quin_span.1],
                    "parity_verified": pre_q.verify_parity(),
                    "parity_word": format!("{:016x}", pre_q.parity()),
                },
                "postcondition": {
                    "span": [c.postcondition_quin_span.0, c.postcondition_quin_span.1],
                    "parity_verified": post_q.verify_parity(),
                    "parity_word": format!("{:016x}", post_q.parity()),
                },
                "invariant": {
                    "span": [c.invariant_quin_span.0, c.invariant_quin_span.1],
                    "parity_verified": inv_q.verify_parity(),
                    "parity_word": format!("{:016x}", inv_q.parity()),
                },
            }
        }
    })
}

/// Serializes morphisms filtered by optional `category` (`synth.ontology.morphisms`).
pub fn morphisms_json(category: Option<&str>) -> Value {
    let filtered: Vec<_> = MORPHISMS
        .iter()
        .filter(|m| category.is_none_or(|cat| m.category == cat))
        .map(morphism_to_json)
        .collect();
    json!({
        "schema": "hypermedia.categorical-morphisms/v1",
        "filter_category": category,
        "count": filtered.len(),
        "morphisms": filtered,
    })
}
