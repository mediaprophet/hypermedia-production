//! Reference media attached to a prompt: images, keyframes, video clips, and audio tracks.

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};

static NEXT_REF_ID: AtomicU64 = AtomicU64::new(1);

/// The kind of reference media.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefKind {
    Image,
    Video,
    Audio,
}

impl RefKind {
    pub fn noun(self) -> &'static str {
        match self {
            RefKind::Image => "image",
            RefKind::Video => "video",
            RefKind::Audio => "audio",
        }
    }

    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_ascii_lowercase();
        match ext.as_str() {
            "png" | "jpg" | "jpeg" | "webp" | "gif" | "bmp" | "avif" => Some(RefKind::Image),
            "mp4" | "mov" | "webm" | "mkv" | "avi" | "m4v" => Some(RefKind::Video),
            "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" | "opus" => Some(RefKind::Audio),
            _ => None,
        }
    }
}

/// Status of reference attachment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefStatus {
    Uploading,
    Ready,
}

/// Individual reference media element.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RefMedia {
    pub id: u64,
    pub kind: RefKind,
    pub status: RefStatus,
    pub token: Option<String>,
    pub preview: Option<String>,
    pub full_url: Option<String>,
    pub duration_secs: f32,
}

impl RefMedia {
    pub fn uploading(kind: RefKind, preview: Option<String>) -> Self {
        Self {
            id: NEXT_REF_ID.fetch_add(1, Ordering::Relaxed),
            kind,
            status: RefStatus::Uploading,
            token: None,
            preview,
            full_url: None,
            duration_secs: 0.0,
        }
    }

    pub fn from_library(
        kind: RefKind,
        token: String,
        preview: Option<String>,
        full_url: Option<String>,
        duration_secs: f32,
    ) -> Self {
        Self {
            id: NEXT_REF_ID.fetch_add(1, Ordering::Relaxed),
            kind,
            status: RefStatus::Ready,
            token: Some(token),
            preview,
            full_url,
            duration_secs,
        }
    }
}

/// Target image placement slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageSlot {
    Reference,
    FirstFrame,
    LastFrame,
}

/// Composite references container attached to a generation prompt.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct References {
    pub images: Vec<RefMedia>,
    pub videos: Vec<RefMedia>,
    pub audios: Vec<RefMedia>,
    pub first_frame: Option<RefMedia>,
    pub last_frame: Option<RefMedia>,
}

impl References {
    pub fn is_empty(&self) -> bool {
        self.images.is_empty()
            && self.videos.is_empty()
            && self.audios.is_empty()
            && self.first_frame.is_none()
            && self.last_frame.is_none()
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn list(&self, kind: RefKind) -> &Vec<RefMedia> {
        match kind {
            RefKind::Image => &self.images,
            RefKind::Video => &self.videos,
            RefKind::Audio => &self.audios,
        }
    }

    pub fn list_mut(&mut self, kind: RefKind) -> &mut Vec<RefMedia> {
        match kind {
            RefKind::Image => &mut self.images,
            RefKind::Video => &mut self.videos,
            RefKind::Audio => &mut self.audios,
        }
    }

    pub fn total_secs(&self, kind: RefKind) -> f32 {
        self.list(kind).iter().map(|r| r.duration_secs).sum()
    }

    pub fn any_uploading(&self) -> bool {
        let slots = self.first_frame.iter().chain(self.last_frame.iter());
        self.images
            .iter()
            .chain(&self.videos)
            .chain(&self.audios)
            .chain(slots)
            .any(|r| r.status == RefStatus::Uploading)
    }

    pub fn find_mut(&mut self, id: u64) -> Option<&mut RefMedia> {
        let slots = self.first_frame.iter_mut().chain(self.last_frame.iter_mut());
        self.images
            .iter_mut()
            .chain(&mut self.videos)
            .chain(&mut self.audios)
            .chain(slots)
            .find(|r| r.id == id)
    }

    pub fn remove(&mut self, id: u64) {
        for list in [&mut self.images, &mut self.videos, &mut self.audios] {
            list.retain(|r| r.id != id);
        }
        if self.first_frame.as_ref().is_some_and(|r| r.id == id) {
            self.first_frame = None;
        }
        if self.last_frame.as_ref().is_some_and(|r| r.id == id) {
            self.last_frame = None;
        }
    }

    pub fn swap_frames(&mut self) {
        std::mem::swap(&mut self.first_frame, &mut self.last_frame);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_files_by_extension() {
        assert_eq!(RefKind::from_path(Path::new("photo.png")), Some(RefKind::Image));
        assert_eq!(RefKind::from_path(Path::new("video.mp4")), Some(RefKind::Video));
        assert_eq!(RefKind::from_path(Path::new("sound.wav")), Some(RefKind::Audio));
        assert_eq!(RefKind::from_path(Path::new("doc.pdf")), None);
    }

    #[test]
    fn reference_lifecycle_and_frames() {
        let mut refs = References::default();
        let ref1 = RefMedia::uploading(RefKind::Image, Some("preview1.jpg".into()));
        let id1 = ref1.id;
        refs.images.push(ref1);
        assert!(refs.any_uploading());

        let frame_start = RefMedia::from_library(RefKind::Image, "tok1".into(), None, None, 0.0);
        let frame_end = RefMedia::from_library(RefKind::Image, "tok2".into(), None, None, 0.0);
        let id_start = frame_start.id;
        let id_end = frame_end.id;
        refs.first_frame = Some(frame_start);
        refs.last_frame = Some(frame_end);

        refs.swap_frames();
        assert_eq!(refs.first_frame.as_ref().map(|f| f.id), Some(id_end));
        assert_eq!(refs.last_frame.as_ref().map(|f| f.id), Some(id_start));

        refs.remove(id1);
        assert!(refs.images.is_empty());
    }
}
