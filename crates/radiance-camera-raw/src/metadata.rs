//! EXIF, XMP, IPTC metadata models, GPX tracklog geotagging, and reversible catalog journal.
//!
//! Grounded in UDHR Articles 12, 19, 27 and ICESCR Article 15.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExifMetadata {
    pub make: Option<String>,
    pub model: Option<String>,
    pub lens: Option<String>,
    pub exposure_time: Option<f64>,
    pub f_number: Option<f64>,
    pub iso: Option<u32>,
    pub focal_length: Option<f64>,
    pub timestamp_utc: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct XmpMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub creator: Option<String>,
    pub rights: Option<String>,
    pub subject_tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IptcMetadata {
    pub keywords: Vec<String>,
    pub copyright: Option<String>,
    pub credit: Option<String>,
    pub caption: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GpsCoord {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackPoint {
    pub time: f64,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GpxTracklog {
    pub points: Vec<TrackPoint>,
}

impl GpxTracklog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_point(&mut self, pt: TrackPoint) {
        self.points.push(pt);
        self.points.sort_by(|a, b| a.time.total_cmp(&b.time));
    }

    /// Interpolates GPS location at `time` (seconds since epoch) with a maximum gap limit.
    pub fn interpolate_position(&self, time: f64, max_gap_sec: f64) -> Option<GpsCoord> {
        if self.points.is_empty() || !time.is_finite() {
            return None;
        }

        let idx = self.points.partition_point(|p| p.time < time);
        if idx == 0 {
            let p0 = &self.points[0];
            return if (p0.time - time).abs() <= max_gap_sec {
                Some(GpsCoord { latitude: p0.latitude, longitude: p0.longitude, altitude: p0.altitude })
            } else {
                None
            };
        }
        if idx >= self.points.len() {
            let p_last = self.points.last()?;
            return if (time - p_last.time).abs() <= max_gap_sec {
                Some(GpsCoord { latitude: p_last.latitude, longitude: p_last.longitude, altitude: p_last.altitude })
            } else {
                None
            };
        }

        let p0 = &self.points[idx - 1];
        let p1 = &self.points[idx];
        let gap = p1.time - p0.time;
        if gap <= 0.0 || gap > max_gap_sec {
            // Check nearest
            let d0 = (time - p0.time).abs();
            let d1 = (p1.time - time).abs();
            return if d0.min(d1) <= max_gap_sec {
                let near = if d0 <= d1 { p0 } else { p1 };
                Some(GpsCoord { latitude: near.latitude, longitude: near.longitude, altitude: near.altitude })
            } else {
                None
            };
        }

        let frac = (time - p0.time) / gap;
        // Handle longitude meridian wrap
        let mut dlon = p1.longitude - p0.longitude;
        if dlon > 180.0 {
            dlon -= 360.0;
        } else if dlon < -180.0 {
            dlon += 360.0;
        }

        let mut lon = p0.longitude + dlon * frac;
        if lon > 180.0 {
            lon -= 360.0;
        } else if lon < -180.0 {
            lon += 360.0;
        }

        let lat = p0.latitude + (p1.latitude - p0.latitude) * frac;
        let alt = match (p0.altitude, p1.altitude) {
            (Some(a), Some(b)) => Some(a + (b - a) * frac),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };

        Some(GpsCoord { latitude: lat, longitude: lon, altitude: alt })
    }
}

// ---------------------------------------------------------------------------
// Reversible Catalog Journal & Smart Albums
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JournalAction {
    SetRating { photo_id: u64, old_rating: u8, new_rating: u8 },
    AddTag { photo_id: u64, tag: String },
    RemoveTag { photo_id: u64, tag: String },
    SetAlbum { photo_id: u64, old_album: Option<String>, new_album: Option<String> },
}

impl JournalAction {
    pub fn inverse(&self) -> JournalAction {
        match self {
            JournalAction::SetRating { photo_id, old_rating, new_rating } => JournalAction::SetRating {
                photo_id: *photo_id,
                old_rating: *new_rating,
                new_rating: *old_rating,
            },
            JournalAction::AddTag { photo_id, tag } => JournalAction::RemoveTag {
                photo_id: *photo_id,
                tag: tag.clone(),
            },
            JournalAction::RemoveTag { photo_id, tag } => JournalAction::AddTag {
                photo_id: *photo_id,
                tag: tag.clone(),
            },
            JournalAction::SetAlbum { photo_id, old_album, new_album } => JournalAction::SetAlbum {
                photo_id: *photo_id,
                old_album: new_album.clone(),
                new_album: old_album.clone(),
            },
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CatalogJournal {
    pub history: Vec<JournalAction>,
    pub redo_stack: Vec<JournalAction>,
}

impl CatalogJournal {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_action(&mut self, action: JournalAction) {
        self.history.push(action);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<JournalAction> {
        let action = self.history.pop()?;
        let inv = action.inverse();
        self.redo_stack.push(action);
        Some(inv)
    }

    pub fn redo(&mut self) -> Option<JournalAction> {
        let action = self.redo_stack.pop()?;
        self.history.push(action.clone());
        Some(action)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SmartRule {
    RatingGte(u8),
    HasTag(String),
    InAlbum(String),
    And(Vec<SmartRule>),
    Or(Vec<SmartRule>),
    Not(Box<SmartRule>),
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PhotoRecord {
    pub id: u64,
    pub rating: u8,
    pub tags: Vec<String>,
    pub album: Option<String>,
}

impl SmartRule {
    pub fn matches(&self, photo: &PhotoRecord) -> bool {
        match self {
            SmartRule::RatingGte(min) => photo.rating >= *min,
            SmartRule::HasTag(tag) => photo.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)),
            SmartRule::InAlbum(alb) => photo.album.as_ref().map_or(false, |a| a == alb),
            SmartRule::And(rules) => rules.iter().all(|r| r.matches(photo)),
            SmartRule::Or(rules) => rules.iter().any(|r| r.matches(photo)),
            SmartRule::Not(r) => !r.matches(photo),
        }
    }
}
