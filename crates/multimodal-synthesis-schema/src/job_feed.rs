//! Generation job feed status, completed items, failures, and deterministic progress/ETA estimation.

use crate::catalog::MediaKind;
use serde::{Deserialize, Serialize};

/// Completed media item stored in the library feed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeedItem {
    pub token: String,
    pub kind: MediaKind,
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub full_url: String,
    pub created_at: i64,
    pub model_id: Option<String>,
    pub prompt_token: Option<String>,
    pub batch_token: Option<String>,
    pub duration_secs: Option<f32>,
    pub animated: Option<String>,
}

/// Generation job currently executing on a synthesis backend.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PendingJob {
    pub job_token: String,
    pub kind: MediaKind,
    pub prompt: String,
    pub model_id: Option<String>,
    pub created_at: i64,
    pub server_progress: Option<u8>,
    pub expected_secs: f32,
    pub batch_count: u32,
}

/// Generation job that failed or was cancelled.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailedJob {
    pub job_token: String,
    pub prompt: String,
    pub model_id: Option<String>,
    pub updated_at: i64,
    pub reason: String,
    pub message: Option<String>,
    pub ref_image: Option<String>,
}

/// Computes estimated progress percentage (0..=99) and a human-readable remaining time label.
pub fn pending_status(job: &PendingJob, now: i64) -> (u8, String) {
    let elapsed = (now - job.created_at).max(0) as f32;
    let estimated = ((elapsed / job.expected_secs.max(1.0)) * 100.0).min(95.0) as u8;
    let progress = job
        .server_progress
        .filter(|p| *p > 0)
        .unwrap_or(estimated)
        .min(99);

    if progress >= 95 {
        return (progress, "Almost done...".to_owned());
    }

    let left = (job.expected_secs - elapsed).max(1.0) as i64;
    let (h, m, s) = (left / 3600, (left % 3600) / 60, left % 60);
    let label = if h > 0 && m > 0 {
        format!("~{h}h {m}m")
    } else if h > 0 {
        format!("~{h}h")
    } else if m > 0 {
        format!("~{m}m")
    } else {
        format!("~{s}s")
    };
    (progress, label)
}

/// Relative human-readable time formatting: "just now", "5 minutes ago", etc.
pub fn time_ago(then: i64, now: i64) -> String {
    let d = (now - then).max(0);
    let plural = |n: i64, unit: &str| format!("{n} {unit}{} ago", if n == 1 { "" } else { "s" });
    match d {
        0..=59 => "just now".to_owned(),
        60..=3599 => plural(d / 60, "minute"),
        3600..=86_399 => plural(d / 3600, "hour"),
        86_400..=2_591_999 => plural(d / 86_400, "day"),
        2_592_000..=31_535_999 => plural(d / 2_592_000, "month"),
        _ => plural(d / 31_536_000, "year"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_and_eta_calculation() {
        let job = PendingJob {
            job_token: "job_42".into(),
            kind: MediaKind::Video,
            prompt: "pan across coastal ridge".into(),
            model_id: Some("seedance_2p0".into()),
            created_at: 1000,
            server_progress: None,
            expected_secs: 100.0,
            batch_count: 1,
        };

        // 50s elapsed: ~50%
        let (pct, label) = pending_status(&job, 1050);
        assert_eq!(pct, 50);
        assert_eq!(label, "~50s");

        // 98s elapsed: should cap at 95% or report almost done
        let (pct_late, label_late) = pending_status(&job, 1098);
        assert_eq!(pct_late, 95);
        assert_eq!(label_late, "Almost done...");
    }

    #[test]
    fn relative_time_formatting() {
        assert_eq!(time_ago(1000, 1030), "just now");
        assert_eq!(time_ago(1000, 1120), "2 minutes ago");
        assert_eq!(time_ago(1000, 4600), "1 hour ago");
        assert_eq!(time_ago(1000, 90000), "1 day ago");
    }
}
