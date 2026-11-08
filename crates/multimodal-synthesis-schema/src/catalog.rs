//! Multimodal generation model catalog and capability descriptors.
//!
//! Provides descriptors for image, video, and audio synthesis models, parameter snapping,
//! duration resolution, aspect ratio parsing, and family grouping.

use serde::{Deserialize, Serialize};

/// Default model identifiers across multimodal modalities.
pub const DEFAULT_IMAGE_MODEL: &str = "nano_banana_pro";
pub const DEFAULT_VIDEO_MODEL: &str = "seedance_2p0";
pub const DEFAULT_AUDIO_MODEL: &str = "suno_music";

/// The fallback prompt limit.
pub const DEFAULT_PROMPT_MAX: usize = 1000;

/// The threshold for grouping models by family in pickers.
const GROUP_THRESHOLD: usize = 8;

/// Audio models that require an existing audio reference track.
const AUDIO_MODELS_REQUIRING_AUDIO_REF: [&str; 2] = ["suno_remix", "suno_sample"];

/// Musical keys for audio synthesis: value, label, short label.
pub const MUSICAL_KEYS: [(&str, &str, &str); 13] = [
    ("auto", "Auto", "Auto"),
    ("c_major", "C Major", "C"),
    ("c_minor", "C Minor", "Cm"),
    ("d_major", "D Major", "D"),
    ("d_minor", "D Minor", "Dm"),
    ("f_major", "F Major", "F"),
    ("f_minor", "F Minor", "Fm"),
    ("g_major", "G Major", "G"),
    ("g_minor", "G Minor", "Gm"),
    ("a_major", "A Major", "A"),
    ("a_minor", "A Minor", "Am"),
    ("b_major", "B Major", "B"),
    ("b_minor", "B Minor", "Bm"),
];

/// Multimodal media kind selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaKind {
    Image,
    Video,
    Audio,
}

impl MediaKind {
    pub fn label(self) -> &'static str {
        match self {
            MediaKind::Image => "Image",
            MediaKind::Video => "Video",
            MediaKind::Audio => "Audio",
        }
    }
}

/// Comprehensive model capability descriptor.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub creator: String,
    pub family: String,
    pub disabled: bool,
    pub prompt_supported: bool,
    pub prompt_max: Option<usize>,
    pub aspect_ratios: Vec<String>,
    pub aspect_default: Option<String>,
    pub resolutions: Vec<String>,
    pub resolution_default: Option<String>,
    pub qualities: Vec<String>,
    pub quality_default: Option<String>,
    pub batch_sizes: Vec<u16>,
    pub batch_default: u16,
    pub image_refs_max: usize,
    // Video-specific capabilities
    pub text_to_video: bool,
    pub start_frame: bool,
    pub start_frame_required: bool,
    pub end_frame: bool,
    pub sound_toggle: bool,
    pub video_refs_max: usize,
    pub video_refs_max_secs: Option<u16>,
    pub audio_refs_max: usize,
    pub audio_refs_max_secs: Option<u16>,
    pub character_refs_max: usize,
    pub duration_options: Vec<u16>,
    pub duration_min: Option<u16>,
    pub duration_max: Option<u16>,
    pub duration_max_with_refs: Option<u16>,
    pub duration_default: Option<u16>,
    pub bitrates: Vec<String>,
    pub bitrate_default: Option<String>,
    pub output_formats: Vec<String>,
    pub output_format_default: Option<String>,
    // Audio-specific capabilities
    pub audio: AudioCaps,
    pub expected_secs: f32,
}

/// Audio-specific synthesis parameters.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AudioCaps {
    pub style_prompt: bool,
    pub instrumental: bool,
    pub keep_lyrics: bool,
    pub loopable: bool,
    pub bpm: bool,
    pub musical_key: bool,
    pub sample_rates: Vec<u32>,
    pub sample_rate_default: Option<u32>,
    pub speed: bool,
    pub volume: bool,
    pub pitch: bool,
    pub requires_audio_ref: bool,
}

impl AudioCaps {
    pub fn has_tuning(&self) -> bool {
        !self.sample_rates.is_empty() || self.speed || self.volume || self.pitch
    }

    pub fn sample_rate(&self, picked: Option<u32>) -> Option<u32> {
        picked
            .filter(|hz| self.sample_rates.contains(hz))
            .or(self.sample_rate_default)
            .or_else(|| self.sample_rates.first().copied())
    }
}

impl ModelInfo {
    pub fn batch_options(&self) -> Vec<u16> {
        if self.batch_sizes.is_empty() {
            vec![1]
        } else {
            self.batch_sizes.clone()
        }
    }

    pub fn valid_batch(&self, count: u16) -> u16 {
        let options = self.batch_options();
        if options.contains(&count) {
            count
        } else if options.contains(&self.batch_default) {
            self.batch_default
        } else {
            options[0]
        }
    }

    pub fn resolve(current: Option<&str>, options: &[String], default: Option<&String>) -> Option<String> {
        match current {
            Some(c) if options.iter().any(|o| o == c) => Some(c.to_owned()),
            _ => default
                .filter(|d| options.is_empty() || options.contains(d))
                .cloned()
                .or_else(|| options.first().cloned()),
        }
    }

    pub fn supports_reference_mode(&self) -> bool {
        self.image_refs_max > 0 || self.video_refs_max > 0 || self.audio_refs_max > 0
    }

    pub fn duration_range(&self, with_refs: bool) -> Option<(u16, u16)> {
        let max = if with_refs {
            self.duration_max_with_refs.or(self.duration_max)
        } else {
            self.duration_max
        };
        match (self.duration_min, max) {
            (Some(min), Some(max)) if max > min => Some((min, max)),
            _ => {
                let lo = self.duration_options.iter().min()?;
                let hi = self.duration_options.iter().max()?;
                (hi > lo).then_some((*lo, *hi))
            }
        }
    }

    pub fn resolve_duration(&self, current: Option<u16>, with_refs: bool) -> Option<u16> {
        let default = self
            .duration_default
            .or_else(|| self.duration_options.first().copied())
            .or(self.duration_min);
        let current = current.or(default)?;
        if !self.duration_options.is_empty() {
            let cap = if with_refs {
                self.duration_max_with_refs
            } else {
                None
            };
            let mut options: Vec<u16> = self
                .duration_options
                .iter()
                .copied()
                .filter(|o| cap.is_none_or(|c| *o <= c))
                .collect();
            options.sort_unstable();
            return options
                .iter()
                .copied()
                .find(|&o| o >= current)
                .or(options.last().copied())
                .or(default);
        }
        match self.duration_range(with_refs) {
            Some((min, max)) => Some(current.clamp(min, max)),
            None => default,
        }
    }
}

/// Catalog holding image, video, and audio model entries with filtering.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Catalog {
    pub image: Vec<ModelInfo>,
    pub video: Vec<ModelInfo>,
    pub audio: Vec<ModelInfo>,
    pub image_page: Vec<ModelInfo>,
    pub video_page: Vec<ModelInfo>,
    pub audio_page: Vec<ModelInfo>,
}

impl Catalog {
    pub fn set_image(&mut self, models: Vec<ModelInfo>) {
        self.image_page = image_page_models(&models);
        self.image = models;
    }

    pub fn set_video(&mut self, models: Vec<ModelInfo>) {
        self.video_page = video_page_models(&models);
        self.video = models;
    }

    pub fn set_audio(&mut self, models: Vec<ModelInfo>) {
        self.audio_page = models.iter().filter(|m| !m.disabled).cloned().collect();
        self.audio = models;
    }

    pub fn find(&self, id: &str) -> Option<&ModelInfo> {
        self.image
            .iter()
            .chain(&self.video)
            .chain(&self.audio)
            .find(|m| m.id == id)
    }

    pub fn display_name(&self, model_id: &str) -> String {
        self.find(model_id)
            .map_or_else(|| display_name(model_id), |m| m.name.clone())
    }

    pub fn creator(&self, model_id: &str) -> String {
        self.find(model_id)
            .map_or_else(|| creator_for(model_id).to_owned(), |m| m.creator.clone())
    }
}

pub fn display_name(id: &str) -> String {
    let special = match id {
        "nano_banana" | "gemini_25_flash" => "Nano Banana",
        "grok_image" => "Grok Image",
        "grok_imagine_image" => "Grok",
        "grok_imagine_video" => "Grok Video",
        "grok_imagine_image_q" => "Grok Imagine Quality",
        "gpt_image_1" => "GPT Image 1 (GPT-4o)",
        "flux_pro_1" => "Flux Pro (Inpainting)",
        "seedance_2p0_bp" => "Seedance 2.0 Plus",
        "seedance_2p0_bp_fast" => "Seedance 2.0 Plus Fast",
        "seedance_2p0_bp_mini" => "Seedance 2.0 Plus Mini",
        "seedance_2p0_bpu" => "Seedance 2.0 Plus Ultra",
        "seedance_2p0_bpu_fast" => "Seedance 2.0 Plus Ultra Fast",
        "seedance_2p0_bpu_mini" => "Seedance 2.0 Plus Ultra Mini",
        "marble_0p1_mini" => "Marble Mini",
        "marble_0p1_plus" => "Marble Plus",
        "minimax_h3" => "MiniMax H3",
        _ => "",
    };
    if !special.is_empty() {
        return special.to_owned();
    }
    let words: Vec<String> = id
        .split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let bytes = w.as_bytes();
            if !bytes.is_empty()
                && bytes[0].is_ascii_digit()
                && w.contains('p')
                && w.chars().all(|c| c.is_ascii_digit() || c == 'p')
            {
                return w.replace('p', ".");
            }
            match w {
                "gpt" => "GPT".to_owned(),
                "veo" => "Google Veo".to_owned(),
                "u" => "Ultra".to_owned(),
                "q" => "Quality".to_owned(),
                _ => {
                    let mut c = w.chars();
                    c.next()
                        .map_or_else(String::new, |f| f.to_uppercase().chain(c).collect())
                }
            }
        })
        .collect();
    words.join(" ")
}

pub fn default_prompt_max(id: &str) -> Option<usize> {
    const LIMITS: [(&str, Option<usize>); 15] = [
        ("nano_banana", None),
        ("gpt_image", None),
        ("seedance_2", None),
        ("seedance_1", Some(3000)),
        ("grok_imagine_video", Some(4096)),
        ("grok_video", Some(4096)),
        ("grok", Some(8000)),
        ("midjourney", Some(6000)),
        ("flux", Some(4000)),
        ("seedream", Some(4000)),
        ("qwen", Some(800)),
        ("kling", Some(2500)),
        ("happy_horse", Some(2500)),
        ("sora", Some(2000)),
        ("veo", Some(5000)),
    ];
    LIMITS
        .iter()
        .find(|(prefix, _)| id.starts_with(prefix))
        .map_or(Some(DEFAULT_PROMPT_MAX), |(_, limit)| *limit)
}

pub fn description(id: &str) -> &'static str {
    match id {
        "grok_image" | "grok_imagine_image" => "Fast generation",
        "midjourney" | "midjourney_7" | "midjourney_7_niji" | "midjourney_8" => "Stunning style and quality",
        "flux_pro_1p1_ultra" => "Higher quality model",
        "flux_pro_1p1" => "High quality model",
        "flux_1_dev" => "Fast, but lower quality",
        "flux_1_schnell" => "Fastest image gen, lowest quality",
        "nano_banana" | "nano_banana_2" => "Fast instructive editing",
        "nano_banana_pro" => "Powerful instructive editing",
        "gpt_image_1" => "Accurate instruction following",
        "gpt_image_1p5" => "Faster, improved",
        "gpt_image_2" => "Smart with high text rendering support",
        "gpt_image_2p5_flare" => "Precise edits, keeps subjects intact",
        "gpt_image_2p5_sunburst" => "Premium detail rendering",
        "seedream_4" | "seedream_4p5" | "seedream_5_lite" => "Fast batch generation",
        "grok_video" | "grok_imagine_video" => "Fastest video model",
        "sora_2" => "Photorealistic video model",
        "veo_3" => "High-quality video model",
        "suno_music" => "Full songs from text prompt",
        "suno_remix" => "Remix an existing track",
        "suno_sounds" => "Sound effects with beat control",
        "suno_sample" => "Build a song from a sample",
        "seed_audio_1p0" => "Sound generation with fine tuning",
        _ => "General generative model",
    }
}

pub fn expected_secs(id: &str, kind: MediaKind) -> f32 {
    match id {
        "grok_image" | "grok_imagine_image" | "flux_pro_1p1" | "flux_1_dev" | "flux_1_schnell" => 10.0,
        "grok_video" | "grok_imagine_video" => 50.0,
        "nano_banana" | "nano_banana_2" | "nano_banana_pro" => 25.0,
        "flux_pro_1p1_ultra" => 35.0,
        "midjourney" | "midjourney_7" | "midjourney_7_niji" | "midjourney_8" => 45.0,
        "gpt_image_1" | "gpt_image_1p5" | "gpt_image_2p5_flare" | "seedream_4" | "seedream_4p5"
        | "seedream_5_lite" => 60.0,
        "gpt_image_2" | "gpt_image_2p5_sunburst" => 120.0,
        _ if kind == MediaKind::Video => 900.0,
        _ if kind == MediaKind::Audio => 120.0,
        _ => 30.0,
    }
}

pub fn creator_for(id: &str) -> &'static str {
    const PREFIXES: [(&str, &str); 18] = [
        ("suno", "suno"),
        ("seed_audio", "bytedance"),
        ("flux", "black_forest_labs"),
        ("nano_banana", "google"),
        ("gemini", "google"),
        ("veo", "google"),
        ("imagen", "google"),
        ("gpt_image", "openai"),
        ("sora", "openai"),
        ("seedream", "bytedance"),
        ("seedance", "bytedance"),
        ("seededit", "bytedance"),
        ("kling", "kling"),
        ("grok", "grok"),
        ("midjourney", "midjourney"),
        ("qwen", "alibaba"),
        ("happy_horse", "alibaba"),
        ("minimax", "minimax"),
    ];
    PREFIXES
        .iter()
        .find(|(p, _)| id.starts_with(p))
        .map_or("community", |(_, c)| c)
}

pub fn family(id: &str) -> &'static str {
    const FAMILIES: [(&str, &str); 23] = [
        ("seed_audio", "Seed Audio"),
        ("seedance", "Seedance"),
        ("kling", "Kling"),
        ("veo", "Veo"),
        ("sora", "Sora"),
        ("happy_horse", "Happy Horse"),
        ("wan", "Wan"),
        ("vidu", "Vidu"),
        ("preview", "Preview"),
        ("nano_banana", "Nano Banana"),
        ("gemini_25_flash", "Nano Banana"),
        ("seedream", "Seedream"),
        ("seededit", "SeedEdit"),
        ("gpt_image", "GPT Image"),
        ("flux", "Flux"),
        ("midjourney", "Midjourney"),
        ("grok", "Grok"),
        ("qwen", "Qwen"),
        ("redesign", "Redesign"),
        ("imagen", "Imagen"),
        ("minimax", "MiniMax"),
        ("marble", "World Labs"),
        ("suno", "Suno"),
    ];
    FAMILIES
        .iter()
        .find(|(p, _)| id.starts_with(p))
        .map_or("Other", |(_, f)| f)
}

pub fn family_rank(fam: &str) -> (usize, String) {
    const ORDER: [&str; 17] = [
        "Seedance",
        "Kling",
        "Veo",
        "Sora",
        "Happy Horse",
        "Wan",
        "Vidu",
        "Grok",
        "Nano Banana",
        "Seedream",
        "GPT Image",
        "Flux",
        "Midjourney",
        "Qwen",
        "SeedEdit",
        "Redesign",
        "Imagen",
    ];
    match ORDER.iter().position(|f| *f == fam) {
        Some(i) => (i, String::new()),
        None if fam == "Other" => (usize::MAX, String::new()),
        None => (ORDER.len(), fam.to_owned()),
    }
}

pub struct Family<'a> {
    pub name: String,
    pub models: Vec<&'a ModelInfo>,
}

pub fn grouped(models: &[ModelInfo]) -> Option<Vec<Family<'_>>> {
    if models.len() < GROUP_THRESHOLD {
        return None;
    }
    let mut families: Vec<Family<'_>> = Vec::new();
    for m in models {
        match families.iter_mut().find(|f| f.name == m.family) {
            Some(f) => f.models.push(m),
            None => families.push(Family {
                name: m.family.clone(),
                models: vec![m],
            }),
        }
    }
    families.sort_by_key(|f| family_rank(&f.name));
    Some(families)
}

fn image_page_models(models: &[ModelInfo]) -> Vec<ModelInfo> {
    const HIDDEN: [&str; 2] = ["minimax_h3_turbo", "minimax_h3_ultra"];
    let mut list: Vec<ModelInfo> = models
        .iter()
        .filter(|m| m.prompt_supported && !m.disabled && !HIDDEN.contains(&m.id.as_str()))
        .cloned()
        .collect();
    list.sort_by_key(|m| m.name.to_lowercase());
    list
}

fn video_page_models(models: &[ModelInfo]) -> Vec<ModelInfo> {
    let mut list: Vec<ModelInfo> = models
        .iter()
        .filter(|m| !m.disabled && m.id != "switch_x")
        .cloned()
        .collect();
    list.sort_by_key(|m| m.name.to_lowercase());
    list
}

pub fn aspect_label(value: &str) -> String {
    match value {
        "auto" => "Auto",
        "square" => "Square",
        "wide_five_by_four" => "5:4 (Wide)",
        "wide_four_by_three" => "4:3 (Wide)",
        "wide_three_by_two" => "3:2 (Wide)",
        "wide_sixteen_by_nine" => "16:9 (Wide)",
        "wide_twenty_one_by_nine" => "21:9 (Wide)",
        "tall_four_by_five" => "4:5 (Tall)",
        "tall_three_by_four" => "3:4 (Tall)",
        "tall_two_by_three" => "2:3 (Tall)",
        "tall_nine_by_sixteen" => "9:16 (Tall)",
        "tall_nine_by_twenty_one" => "9:21 (Tall)",
        "auto_2k" => "Auto (2K)",
        "auto_3k" => "Auto (3K)",
        "auto_4k" => "Auto (4K)",
        "square_hd" => "Square (HD)",
        "wide" => "Wide",
        "tall" => "Tall",
        other => other,
    }
    .to_owned()
}

pub fn aspect_proportion(value: &str) -> Option<(f32, f32)> {
    Some(match value {
        "square" | "square_hd" => (1.0, 1.0),
        "wide" => (16.0, 10.0),
        "wide_five_by_four" => (5.0, 4.0),
        "wide_four_by_three" => (4.0, 3.0),
        "wide_three_by_two" => (3.0, 2.0),
        "wide_sixteen_by_nine" => (16.0, 9.0),
        "wide_twenty_one_by_nine" => (21.0, 9.0),
        "tall" => (10.0, 16.0),
        "tall_four_by_five" => (4.0, 5.0),
        "tall_three_by_four" => (3.0, 4.0),
        "tall_two_by_three" => (2.0, 3.0),
        "tall_nine_by_sixteen" => (9.0, 16.0),
        "tall_nine_by_twenty_one" => (9.0, 21.0),
        _ => return None,
    })
}

pub fn resolution_label(value: &str) -> String {
    match value {
        "half_k" => "0.5K",
        "four_eighty_p" => "480p",
        "seven_twenty_p" => "720p",
        "one_k" => "1K",
        "ten_eighty_p" => "1080p",
        "two_k" => "2K",
        "three_k" => "3K",
        "four_k" => "4K",
        other => other,
    }
    .to_owned()
}

pub fn resolution_is_hd(value: &str) -> bool {
    matches!(value, "two_k" | "three_k" | "four_k")
}

pub fn quality_label(value: &str) -> String {
    match value {
        "auto" => "Auto",
        "max" => "Max",
        "xhigh" => "Extra high",
        "high" => "High",
        "medium" => "Medium",
        "low" => "Low",
        other => other,
    }
    .to_owned()
}

pub fn sample_rate_label(hz: u32) -> String {
    if hz % 1000 == 0 {
        format!("{} kHz", hz / 1000)
    } else {
        format!("{:.1} kHz", hz as f32 / 1000.0)
    }
}

pub fn bitrate_label(value: &str) -> String {
    match value {
        "normal" => "Normal".to_owned(),
        "high" => "High".to_owned(),
        other => other.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_and_duration_resolution() {
        let m = ModelInfo {
            id: "test_vid".into(),
            batch_sizes: vec![1, 2, 4],
            batch_default: 2,
            duration_options: vec![5, 10, 15],
            duration_default: Some(5),
            ..Default::default()
        };
        assert_eq!(m.valid_batch(4), 4);
        assert_eq!(m.valid_batch(3), 2); // fallback to default
        assert_eq!(m.resolve_duration(Some(7), false), Some(10));
        assert_eq!(m.resolve_duration(None, false), Some(5));
    }

    #[test]
    fn aspect_proportions_and_labels() {
        assert_eq!(aspect_proportion("wide_sixteen_by_nine"), Some((16.0, 9.0)));
        assert_eq!(aspect_proportion("square"), Some((1.0, 1.0)));
        assert_eq!(aspect_proportion("invalid"), None);
        assert_eq!(aspect_label("wide_sixteen_by_nine"), "16:9 (Wide)");
    }

    #[test]
    fn display_name_and_creator() {
        assert_eq!(display_name("nano_banana"), "Nano Banana");
        assert_eq!(display_name("seedance_2p0_bp"), "Seedance 2.0 Plus");
        assert_eq!(display_name("flux_1_dev"), "Flux 1 Dev");
        assert_eq!(creator_for("flux_1_dev"), "black_forest_labs");
        assert_eq!(creator_for("gpt_image_1"), "openai");
    }
}
