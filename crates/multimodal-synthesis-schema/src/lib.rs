//! `multimodal-synthesis-schema`
//!
//! UI-agnostic multimodal synthesis schemas:
//! - Comprehensive multimodal model capability catalog (`catalog`)
//! - Reference media attachments and frame coordination (`references`)
//! - Generation job feed status and deterministic progress/ETA estimation (`job_feed`)
//! - 2D/3D composition stages and inpainting mask normalization (`stage`)
//! - 3D Gaussian Splatting and radiance field cost calculation (`splat_cost`)

#![forbid(unsafe_code)]

pub mod catalog;
pub mod job_feed;
pub mod references;
pub mod splat_cost;
pub mod stage;

pub use catalog::{
    aspect_label, aspect_proportion, bitrate_label, creator_for, default_prompt_max, description,
    display_name, expected_secs, family, family_rank, grouped, quality_label, resolution_is_hd,
    resolution_label, sample_rate_label, AudioCaps, Catalog, Family, MediaKind, ModelInfo,
    DEFAULT_AUDIO_MODEL, DEFAULT_IMAGE_MODEL, DEFAULT_PROMPT_MAX, DEFAULT_VIDEO_MODEL,
    MUSICAL_KEYS,
};
pub use job_feed::{pending_status, time_ago, FailedJob, FeedItem, PendingJob};
pub use references::{ImageSlot, RefKind, RefMedia, RefStatus, References};
pub use splat_cost::{
    calculate_splat_cost, credits_for, credits_to_usd_cents, RadianceInputType, RadianceModel,
    SplatCostBreakdown,
};
pub use stage::{
    normalize_mask_bytes, MaskNormalizationMode, SphericalAdjustment, StageCamera,
    StageComposition,
};
