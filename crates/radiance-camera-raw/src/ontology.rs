//! Category-theoretic ontology and agentic morphism specification for camera RAW radiance processing.
//!
//! Grounded in UDHR Articles 12, 19, 27; ICESCR Article 15; and Informatics Fiduciary duties.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NQuin48 {
    pub subject: u64,
    pub predicate: u64,
    pub object: u64,
    pub context: u64,
    pub metadata: u64,
}

impl NQuin48 {
    pub const fn new(subject: u64, predicate: u64, object: u64, context: u64, metadata: u64) -> Self {
        Self { subject, predicate, object, context, metadata }
    }

    pub const fn precondition(morphism_hash: u64, condition_hash: u64, seq: u64) -> Self {
        Self::new(morphism_hash, 0x1001, condition_hash, 0x7FFF_0001, seq)
    }

    pub const fn postcondition(morphism_hash: u64, condition_hash: u64, seq: u64) -> Self {
        Self::new(morphism_hash, 0x1002, condition_hash, 0x7FFF_0001, seq)
    }

    pub const fn invariant(morphism_hash: u64, condition_hash: u64, seq: u64) -> Self {
        Self::new(morphism_hash, 0x1003, condition_hash, 0x7FFF_0001, seq)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OntologicalContract {
    pub precondition: NQuin48,
    pub postcondition: NQuin48,
    pub invariant: NQuin48,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuncDescriptor {
    pub name: &'static str,
    pub stable_id: u32,
    pub deterministic: bool,
    pub pure_compute: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct AgenticMorphismSpec {
    pub id: &'static str,
    pub category: &'static str,
    pub morphism: &'static str,
    pub summary: &'static str,
    pub domain: &'static str,
    pub codomain: &'static str,
    pub morphism_kind: &'static str,
    pub teleology: &'static str,
    pub units_and_frames: &'static str,
    pub composable_with: &'static [&'static str],
    pub human_preview_affordance: &'static str,
    pub descriptor: FuncDescriptor,
    pub contract: OntologicalContract,
}

pub static RAW_DEMOSAIC_MORPHISM: AgenticMorphismSpec = AgenticMorphismSpec {
    id: "hypermedia.radiance.raw_demosaic",
    category: "radiance",
    morphism: "raw_demosaic",
    summary: "Demosaics Bayer or X-Trans raw sensor arrays into linear RGB radiance images",
    domain: "RawBufferU16, CfaPattern, BlackLevelU16, WhiteLevelU16",
    codomain: "RgbImageF32 { width, height, channels: 3 }",
    morphism_kind: "Constructor",
    teleology: "Reconstructs full-color RGB radiance from single-channel mosaic sensor captures",
    units_and_frames: "Normalized scene-linear floating point [0.0, 1.0]",
    composable_with: &[
        "hypermedia.radiance.raw_develop",
        "hypermedia.radiance.hdr_fusion",
    ],
    human_preview_affordance: "Linear RGB viewport render with 100% pixel loupe zoom",
    descriptor: FuncDescriptor {
        name: "raw_demosaic",
        stable_id: 0x0410,
        deterministic: true,
        pure_compute: true,
    },
    contract: OntologicalContract {
        precondition: NQuin48::precondition(0x0410, 0xD001, 1),
        postcondition: NQuin48::postcondition(0x0410, 0xD002, 1),
        invariant: NQuin48::invariant(0x0410, 0xD003, 1),
    },
};

pub static RAW_DEVELOP_MORPHISM: AgenticMorphismSpec = AgenticMorphismSpec {
    id: "hypermedia.radiance.raw_develop",
    category: "radiance",
    morphism: "raw_develop",
    summary: "Applies non-destructive exposure, tone, HSL, and lens corrections with FNV-1a digest caching",
    domain: "RgbImageF32, DevelopSettings",
    codomain: "RgbImageF32",
    morphism_kind: "Endomorphism",
    teleology: "Provides creative and technical non-destructive photographic development",
    units_and_frames: "Photometric exposure EV, Kelvin temperature, parametric tone curve [0.0, 1.0]",
    composable_with: &[
        "hypermedia.radiance.icc_transform",
        "hypermedia.radiance.gamut_check",
    ],
    human_preview_affordance: "Before/after split view with parametric tone histogram display",
    descriptor: FuncDescriptor {
        name: "raw_develop",
        stable_id: 0x0411,
        deterministic: true,
        pure_compute: true,
    },
    contract: OntologicalContract {
        precondition: NQuin48::precondition(0x0411, 0xD004, 2),
        postcondition: NQuin48::postcondition(0x0411, 0xD005, 2),
        invariant: NQuin48::invariant(0x0411, 0xD006, 2),
    },
};

pub static HDR_FUSION_MORPHISM: AgenticMorphismSpec = AgenticMorphismSpec {
    id: "hypermedia.radiance.hdr_fusion",
    category: "radiance",
    morphism: "hdr_fusion",
    summary: "Merges multiple exposure brackets into a high dynamic range radiance map with deghosting",
    domain: "List<ExposureBracket>, HdrOptions",
    codomain: "RadianceMapF32",
    morphism_kind: "AlgebraicFold",
    teleology: "Expands dynamic range beyond physical camera sensor constraints without ghosting",
    units_and_frames: "Scene-referred physical radiance W/(sr·m²)",
    composable_with: &["hypermedia.radiance.raw_develop"],
    human_preview_affordance: "False-color luminance heat map and EV bracket exposure scrubber",
    descriptor: FuncDescriptor {
        name: "hdr_fusion",
        stable_id: 0x0412,
        deterministic: true,
        pure_compute: true,
    },
    contract: OntologicalContract {
        precondition: NQuin48::precondition(0x0412, 0xD007, 3),
        postcondition: NQuin48::postcondition(0x0412, 0xD008, 3),
        invariant: NQuin48::invariant(0x0412, 0xD009, 3),
    },
};

pub static GPX_GEOTAG_MORPHISM: AgenticMorphismSpec = AgenticMorphismSpec {
    id: "hypermedia.geodesy.gpx_geotag",
    category: "geodesy",
    morphism: "gpx_geotag",
    summary: "Interpolates GPS coordinates for photos from GPX timestamped tracklogs",
    domain: "PhotoTimestampUtc, GpxTracklog, MaxGapSeconds",
    codomain: "Option<GpsCoordinate>",
    morphism_kind: "CoalgebraicProjection",
    teleology: "Binds physical spatial provenance and international geodetic context to photographic evidence",
    units_and_frames: "WGS-84 decimal degrees (lat/lon), altitude meters, seconds UTC epoch",
    composable_with: &["hypermedia.chronicle.catalog_journal"],
    human_preview_affordance: "Interactive satellite/terrain map pinpointing capture location",
    descriptor: FuncDescriptor {
        name: "gpx_geotag",
        stable_id: 0x0A01,
        deterministic: true,
        pure_compute: true,
    },
    contract: OntologicalContract {
        precondition: NQuin48::precondition(0x0A01, 0xD010, 4),
        postcondition: NQuin48::postcondition(0x0A01, 0xD011, 4),
        invariant: NQuin48::invariant(0x0A01, 0xD012, 4),
    },
};
