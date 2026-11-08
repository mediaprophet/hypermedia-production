//! Content-addressed immutable change envelope DAG and semantic conflict advisory.
//!
//! Implements local-first collaboration and auditable versioned history:
//! - **Role permissions (`SyncRole`):** `Owner`, `Editor`, `Commenter`, `Viewer`, `Guest`
//! - **Content-addressed change DAG (`ChangeEnvelope`, `ChangeDag`):** every mutation or
//!   comment is recorded as an immutable envelope with a deterministic 64-bit hex ID
//!   and parent pointer, doubling as an enterprise audit log grounded in Informatics Fiduciary duties.
//! - **Social annotations (`TaskComment`):** comments and `@mentions` aligned with UDHR Art. 19.
//! - **Post-merge semantic conflict advisories (`SemanticConflict`):** detects when concurrent
//!   edits create resource overallocations or negative total float.

use std::collections::BTreeSet;
use std::fmt;

use crate::model::Project;
use crate::schedule::{compute_workload, schedule_project};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

/// Maximum envelopes retained before compaction.
pub const MAX_LOG_ENVELOPES: usize = 50_000;

/// Role-based access control levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncRole {
    Guest,
    Viewer,
    Commenter,
    Editor,
    Owner,
}

impl SyncRole {
    pub fn as_str(self) -> &'static str {
        match self {
            SyncRole::Guest => "guest",
            SyncRole::Viewer => "viewer",
            SyncRole::Commenter => "commenter",
            SyncRole::Editor => "editor",
            SyncRole::Owner => "owner",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "owner" | "admin" => SyncRole::Owner,
            "editor" | "member" | "write" => SyncRole::Editor,
            "commenter" | "comment" => SyncRole::Commenter,
            "guest" => SyncRole::Guest,
            _ => SyncRole::Viewer,
        }
    }

    pub fn can_read(self) -> bool {
        true
    }

    pub fn can_comment(self) -> bool {
        matches!(self, SyncRole::Commenter | SyncRole::Editor | SyncRole::Owner)
    }

    pub fn can_write_tasks(self) -> bool {
        matches!(self, SyncRole::Editor | SyncRole::Owner)
    }

    pub fn can_manage_roles(self) -> bool {
        matches!(self, SyncRole::Owner)
    }

    pub fn allows_command(self, cmd_id: &str) -> bool {
        if cmd_id.starts_with("export.")
            || matches!(
                cmd_id,
                "engine.list"
                    | "project.summary"
                    | "task.list"
                    | "schedule.critical_path"
                    | "schedule.monte_carlo"
                    | "baseline.variance"
                    | "resource.workload"
                    | "evm.compute"
                    | "sprint.burndown"
                    | "sync.activity_feed"
                    | "sync.check_permission"
                    | "sync.export_bundle"
            )
        {
            return self.can_read();
        }
        if cmd_id == "sync.add_comment" {
            return self.can_comment();
        }
        self.can_write_tasks()
    }
}

/// Sync or permission error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncError {
    PermissionDenied { role: String, action: String },
    InvalidEnvelope(String),
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncError::PermissionDenied { role, action } => {
                write!(f, "permission denied: role `{role}` cannot perform `{action}`")
            }
            SyncError::InvalidEnvelope(m) => write!(f, "invalid sync envelope: {m}"),
        }
    }
}

impl std::error::Error for SyncError {}

/// A comment attached to a work item inside the synced document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskComment {
    pub id: String,
    pub task_id: u32,
    pub author: String,
    pub body: String,
    pub mentions: Vec<String>,
    pub seq: u64,
}

/// Post-merge semantic schedule conflict surfaced in the activity feed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticConflict {
    pub after_change_id: String,
    pub kind: String,
    pub description: String,
}

/// Immutable change-set envelope in the append-only audit DAG.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeEnvelope {
    pub seq: u64,
    pub change_id: String,
    pub parent_id: Option<String>,
    pub author: String,
    pub role: SyncRole,
    pub command: String,
    pub params: Json,
}

impl ChangeEnvelope {
    pub fn new(
        seq: u64,
        parent_id: Option<String>,
        author: impl Into<String>,
        role: SyncRole,
        command: impl Into<String>,
        params: Json,
    ) -> Self {
        let author = author.into();
        let command = command.into();
        let payload = format!(
            "{seq}:{}:{author}:{}:{command}:{params}",
            parent_id.as_deref().unwrap_or("root"),
            role.as_str()
        );
        let change_id = fnv1a_hex(payload.as_bytes());
        ChangeEnvelope {
            seq,
            change_id,
            parent_id,
            author,
            role,
            command,
            params,
        }
    }
}

/// Content-addressed change DAG exchanged between collaborating peers.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ChangeDag {
    pub head_change_id: Option<String>,
    pub envelopes: Vec<ChangeEnvelope>,
    pub comments: Vec<TaskComment>,
    pub conflicts: Vec<SemanticConflict>,
}

impl ChangeDag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_command(
        &mut self,
        author: impl Into<String>,
        role: SyncRole,
        command: impl Into<String>,
        params: Json,
    ) -> Result<ChangeEnvelope, SyncError> {
        let command = command.into();
        if !role.allows_command(&command) {
            return Err(SyncError::PermissionDenied {
                role: role.as_str().to_string(),
                action: command,
            });
        }
        let seq = (self.envelopes.len() as u64).saturating_add(1);
        let env = ChangeEnvelope::new(seq, self.head_change_id.clone(), author, role, command, params);
        self.head_change_id = Some(env.change_id.clone());
        if self.envelopes.len() >= MAX_LOG_ENVELOPES {
            self.envelopes.remove(0);
        }
        self.envelopes.push(env.clone());
        Ok(env)
    }

    pub fn add_comment(
        &mut self,
        author: impl Into<String>,
        role: SyncRole,
        task_id: u32,
        body: impl Into<String>,
    ) -> Result<TaskComment, SyncError> {
        if !role.can_comment() {
            return Err(SyncError::PermissionDenied {
                role: role.as_str().to_string(),
                action: "sync.add_comment".to_string(),
            });
        }
        let author = author.into();
        let body = body.into();
        let mentions = extract_mentions(&body);
        let env = self.record_command(
            author.clone(),
            role,
            "sync.add_comment",
            serde_json::json!({"task_id": task_id, "body": body, "mentions": mentions}),
        )?;
        let comment = TaskComment {
            id: env.change_id,
            task_id,
            author,
            body,
            mentions,
            seq: env.seq,
        };
        self.comments.push(comment.clone());
        Ok(comment)
    }

    pub fn merge_remote(&mut self, remote: &ChangeDag) -> usize {
        let known: BTreeSet<String> = self.envelopes.iter().map(|e| e.change_id.clone()).collect();
        let mut added = 0usize;
        for env in &remote.envelopes {
            if !known.contains(&env.change_id) && self.envelopes.len() < MAX_LOG_ENVELOPES {
                self.head_change_id = Some(env.change_id.clone());
                self.envelopes.push(env.clone());
                added = added.saturating_add(1);
            }
        }
        let known_comments: BTreeSet<String> = self.comments.iter().map(|c| c.id.clone()).collect();
        for c in &remote.comments {
            if !known_comments.contains(&c.id) {
                self.comments.push(c.clone());
            }
        }
        added
    }

    pub fn inspect_post_merge_conflicts(&mut self, project: &mut Project) -> Vec<SemanticConflict> {
        let _ = schedule_project(project);
        let head = self.head_change_id.clone().unwrap_or_else(|| "initial".to_string());
        let mut found = Vec::new();

        let wl = compute_workload(project);
        if wl.total_overallocated_person_days > 0 {
            found.push(SemanticConflict {
                after_change_id: head.clone(),
                kind: "resource_overallocation".to_string(),
                description: format!(
                    "Post-merge schedule has {} overallocated person-days across resources; run `level_resources` to resolve.",
                    wl.total_overallocated_person_days
                ),
            });
        }

        for t in &project.tasks {
            if t.early_finish > t.late_finish + 1e-6 {
                found.push(SemanticConflict {
                    after_change_id: head.clone(),
                    kind: "constraint_violation".to_string(),
                    description: format!(
                        "Task {} ({}) early finish ({:.1}d) exceeds late finish ({:.1}d).",
                        t.key, t.title, t.early_finish, t.late_finish
                    ),
                });
            }
        }

        self.conflicts = found.clone();
        found
    }
}

fn extract_mentions(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    for word in body.split_whitespace() {
        if let Some(rest) = word.strip_prefix('@') {
            let clean: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
                .collect();
            if !clean.is_empty() && !out.contains(&clean) {
                out.push(clean);
            }
        }
    }
    out
}

fn fnv1a_hex(bytes: &[u8]) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{h:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_dag_records_and_enforces_roles() {
        let mut dag = ChangeDag::new();
        let res = dag.record_command("Alice", SyncRole::Owner, "task.create", serde_json::json!({"title": "Test"}));
        assert!(res.is_ok());

        let denied = dag.record_command("Bob", SyncRole::Viewer, "task.create", serde_json::json!({"title": "Test"}));
        assert!(denied.is_err());
    }

    #[test]
    fn comments_and_mentions() {
        let mut dag = ChangeDag::new();
        let comment = dag
            .add_comment("Alice", SyncRole::Editor, 1, "Reviewing with @omar and @lina")
            .unwrap();
        assert_eq!(comment.mentions, vec!["omar", "lina"]);
    }

    #[test]
    fn merge_remote_idempotent() {
        let mut dag_a = ChangeDag::new();
        let _ = dag_a.record_command("Alice", SyncRole::Owner, "task.create", serde_json::json!({"id": 1}));

        let mut dag_b = ChangeDag::new();
        let added = dag_b.merge_remote(&dag_a);
        assert_eq!(added, 1);

        let added_again = dag_b.merge_remote(&dag_a);
        assert_eq!(added_again, 0);
    }
}
