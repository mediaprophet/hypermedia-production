//! Per-joint range of motion (RoM) for species specs.
//!
//! A spec's optional `joints` list declares, for each rig joint and movement axis, the healthy
//! range and the injury limit, plus an optional narrowed range and gait used when the joint is
//! injured. Angles are radians measured from the neutral pose, so every range must contain 0.
//!
//! ```text
//! joints: [
//!   { id: "left_ankle",
//!     axes: { flexion: { healthy: [-0.20, 0.95], injury: [-0.45, 1.20] } },
//!     when_injured: { axes: { flexion: [-0.05, 0.40] }, gait: "asymmetric_limp", guarding: 0.9 } }
//! ]
//! ```
//!
//! Angles between the healthy range and the injury limit are classified as strain; angles beyond
//! the injury limit are classified as injury. The ranges also compile into per-joint `.q42` rules.

use super::rule_pack::{parse_claim_basis, RuleKind, RuleSpec};
use serde_json::Value as JsonValue;

pub const MAX_JOINTS: usize = 64;

/// Canonical rig joint names with accepted aliases, matching the 18-joint fauna rig.
const JOINTS: &[(&str, &[&str])] = &[
    ("pelvis", &["pelvis_root", "root"]),
    ("spine", &["lumbar"]),
    ("chest", &["thorax"]),
    ("neck", &["cervical"]),
    ("head", &["cranium", "skull"]),
    ("jaw", &["mandible"]),
    ("left_hip", &["hip_l"]),
    ("left_knee", &["knee_l"]),
    ("left_ankle", &["ankle_l"]),
    ("right_hip", &["hip_r"]),
    ("right_knee", &["knee_r"]),
    ("right_ankle", &["ankle_r"]),
    ("left_shoulder", &["shoulder_l"]),
    ("left_elbow", &["elbow_l"]),
    ("left_wrist", &["left_wrist_wing", "forepaw"]),
    ("tail_base", &[]),
    ("tail_mid", &[]),
    ("tail_tip", &[]),
];

pub fn canonical_joint(name: &str) -> Option<&'static str> {
    let n = name.trim().to_ascii_lowercase();
    JOINTS
        .iter()
        .find(|(c, aliases)| *c == n || aliases.contains(&n.as_str()))
        .map(|(c, _)| *c)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Axis {
    Flexion,
    Abduction,
    Rotation,
}

impl Axis {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_ascii_lowercase().as_str() {
            "flexion" | "sagittal" => Ok(Self::Flexion),
            "abduction" | "frontal" | "lateral_flexion" => Ok(Self::Abduction),
            "rotation" | "axial" | "twist" => Ok(Self::Rotation),
            other => Err(format!("unknown joint axis '{other}'")),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Flexion => "flexion",
            Self::Abduction => "abduction",
            Self::Rotation => "rotation",
        }
    }
}

/// Inclusive `[min, max]` range in radians with `min <= 0 <= max`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AngleRange {
    pub min: f64,
    pub max: f64,
}

impl AngleRange {
    pub fn contains(&self, a: f64) -> bool {
        a >= self.min && a <= self.max
    }

    pub fn encloses(&self, other: &AngleRange) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    pub fn clamp(&self, a: f64) -> f64 {
        a.clamp(self.min, self.max)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AxisRom {
    pub axis: Axis,
    pub healthy: AngleRange,
    pub injury: AngleRange,
    pub injured_healthy: Option<AngleRange>,
    pub healthy_basis: String,
    pub injury_basis: String,
    pub injured_basis: String,
    pub healthy_assertion_ids: Vec<String>,
    pub injury_assertion_ids: Vec<String>,
    pub injured_assertion_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct JointRom {
    pub joint: &'static str,
    pub axes: Vec<AxisRom>,
    pub injured_gait: Option<String>,
    pub injured_guarding: f64,
    pub gait_basis: String,
    pub gait_assertion_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionZone {
    Healthy,
    Strain,
    Injury,
}

impl JointRom {
    pub fn axis(&self, axis: Axis) -> Option<&AxisRom> {
        self.axes.iter().find(|a| a.axis == axis)
    }
}

impl AxisRom {
    /// The range a pose may use: the narrowed range when injured, otherwise the healthy range.
    pub fn active_range(&self, injured: bool) -> AngleRange {
        if injured {
            self.injured_healthy.unwrap_or(self.healthy)
        } else {
            self.healthy
        }
    }

    pub fn classify(&self, angle: f64, injured: bool) -> MotionZone {
        if self.active_range(injured).contains(angle) {
            MotionZone::Healthy
        } else if self.injury.contains(angle) {
            MotionZone::Strain
        } else {
            MotionZone::Injury
        }
    }
}

fn parse_range(v: &JsonValue, ctx: &str) -> Result<AngleRange, String> {
    let arr = v
        .as_array()
        .filter(|a| a.len() == 2)
        .ok_or_else(|| format!("{ctx} must be [min, max]"))?;
    let min = arr[0].as_f64().ok_or_else(|| format!("{ctx}[0] must be a number"))?;
    let max = arr[1].as_f64().ok_or_else(|| format!("{ctx}[1] must be a number"))?;
    if !min.is_finite() || !max.is_finite() || min > 0.0 || max < 0.0 {
        return Err(format!(
            "{ctx} must be finite and contain the neutral angle 0 (min <= 0 <= max)"
        ));
    }
    Ok(AngleRange { min, max })
}

fn check_keys(obj: &serde_json::Map<String, JsonValue>, allowed: &[&str], ctx: &str) -> Result<(), String> {
    match obj.keys().find(|k| !allowed.contains(&k.as_str())) {
        Some(k) => Err(format!("{ctx}: unknown field '{k}'")),
        None => Ok(()),
    }
}

fn parse_joint(i: usize, v: &JsonValue) -> Result<JointRom, String> {
    let ctx = format!("joints[{i}]");
    let obj = v.as_object().ok_or_else(|| format!("{ctx} must be a record"))?;
    check_keys(obj, &["id", "axes", "when_injured"], &ctx)?;
    let id = obj
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("{ctx}: 'id' string is required"))?;
    let joint = canonical_joint(id).ok_or_else(|| format!("{ctx}: unknown rig joint '{id}'"))?;
    let axes_obj = obj
        .get("axes")
        .and_then(|v| v.as_object())
        .filter(|o| !o.is_empty())
        .ok_or_else(|| format!("{ctx}: 'axes' record with at least one axis is required"))?;

    let mut axes = Vec::with_capacity(axes_obj.len());
    for (name, spec) in axes_obj {
        let axis = Axis::parse(name).map_err(|e| format!("{ctx}: {e}"))?;
        if axes.iter().any(|a: &AxisRom| a.axis == axis) {
            return Err(format!("{ctx}: axis '{}' declared twice", axis.as_str()));
        }
        let actx = format!("{ctx}.axes.{name}");
        let s = spec
            .as_object()
            .ok_or_else(|| format!("{actx} must be a record"))?;
        check_keys(
            s,
            &["healthy", "injury", "basis", "assertion_ids"],
            &actx,
        )?;
        let healthy = parse_range(
            s.get("healthy").ok_or_else(|| format!("{actx}: 'healthy' is required"))?,
            &format!("{actx}.healthy"),
        )?;
        let injury = parse_range(
            s.get("injury").ok_or_else(|| format!("{actx}: 'injury' is required"))?,
            &format!("{actx}.injury"),
        )?;
        if !injury.encloses(&healthy) {
            return Err(format!("{actx}: injury limit must enclose the healthy range"));
        }
        let (basis, assertion_ids) = axis_citation(s, &actx)?;
        axes.push(AxisRom {
            axis,
            healthy,
            injury,
            injured_healthy: None,
            healthy_basis: basis.clone(),
            injury_basis: basis.clone(),
            injured_basis: basis,
            healthy_assertion_ids: assertion_ids.clone(),
            injury_assertion_ids: assertion_ids.clone(),
            injured_assertion_ids: assertion_ids,
        });
    }
    axes.sort_by_key(|a| a.axis);

    let mut injured_gait = None;
    let mut injured_guarding = 0.0;
    if let Some(w) = obj.get("when_injured") {
        let wctx = format!("{ctx}.when_injured");
        let w = w.as_object().ok_or_else(|| format!("{wctx} must be a record"))?;
        check_keys(w, &["axes", "gait", "guarding"], &wctx)?;
        if let Some(wa) = w.get("axes") {
            let wa = wa
                .as_object()
                .ok_or_else(|| format!("{wctx}.axes must be a record"))?;
            for (name, r) in wa {
                let axis = Axis::parse(name).map_err(|e| format!("{wctx}: {e}"))?;
                let range = parse_range(r, &format!("{wctx}.axes.{name}"))?;
                let target = axes
                    .iter_mut()
                    .find(|a| a.axis == axis)
                    .ok_or_else(|| format!("{wctx}: axis '{name}' is not declared in axes"))?;
                if !target.healthy.encloses(&range) {
                    return Err(format!(
                        "{wctx}.axes.{name}: injured range must lie within the healthy range"
                    ));
                }
                target.injured_healthy = Some(range);
            }
        }
        if let Some(g) = w.get("gait") {
            let g = g
                .as_str()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| format!("{wctx}.gait must be a non-empty string"))?;
            injured_gait = Some(g.to_string());
        }
        if let Some(g) = w.get("guarding") {
            let g = g
                .as_f64()
                .filter(|g| (0.0..=1.0).contains(g))
                .ok_or_else(|| format!("{wctx}.guarding must be in 0.0..=1.0"))?;
            injured_guarding = g;
        }
    }
    Ok(JointRom {
        joint,
        axes,
        injured_gait,
        injured_guarding,
        gait_basis: "model_assumption".into(),
        gait_assertion_ids: Vec::new(),
    })
}

fn axis_citation(
    spec: &serde_json::Map<String, JsonValue>,
    ctx: &str,
) -> Result<(String, Vec<String>), String> {
    let basis = match spec.get("basis") {
        None => "model_assumption".to_string(),
        Some(JsonValue::String(s)) => parse_claim_basis(s)
            .map_err(|e| format!("{ctx}: {e}"))?
            .to_string(),
        Some(_) => return Err(format!("{ctx}: 'basis' must be a string")),
    };
    let assertion_ids = match spec.get("assertion_ids") {
        None => Vec::new(),
        Some(JsonValue::String(s)) if !s.trim().is_empty() => vec![s.trim().to_string()],
        Some(JsonValue::Array(items)) => items
            .iter()
            .map(|v| {
                v.as_str()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .ok_or_else(|| format!("{ctx}: assertion_ids entries must be non-empty strings"))
            })
            .collect::<Result<Vec<_>, _>>()?,
        Some(_) => return Err(format!("{ctx}: 'assertion_ids' must be a string or a list")),
    };
    if matches!(basis.as_str(), "measured" | "described" | "derived") && assertion_ids.is_empty() {
        return Err(format!("{ctx}: {basis} range needs an assertion id"));
    }
    Ok((basis, assertion_ids))
}

/// Parses the spec's optional `joints` list. Returns an empty list when absent.
pub fn parse_joint_rom(spec: Option<&JsonValue>) -> Result<Vec<JointRom>, String> {
    let Some(v) = spec.and_then(|s| s.get("joints")) else {
        return Ok(Vec::new());
    };
    let arr = v
        .as_array()
        .ok_or_else(|| "spec 'joints' must be a list".to_string())?;
    if arr.len() > MAX_JOINTS {
        return Err(format!("spec 'joints' exceeds {MAX_JOINTS} entries"));
    }
    let mut joints: Vec<JointRom> = Vec::with_capacity(arr.len());
    for (i, j) in arr.iter().enumerate() {
        let parsed = parse_joint(i, j)?;
        if joints.iter().any(|x| x.joint == parsed.joint) {
            return Err(format!("joints[{i}]: joint '{}' declared twice", parsed.joint));
        }
        joints.push(parsed);
    }
    Ok(joints)
}

fn push_range(
    out: &mut Vec<RuleSpec>,
    kind: RuleKind,
    prefix: &str,
    zone: &str,
    r: AngleRange,
    basis: &str,
    assertion_ids: &[String],
) {
    for (suffix, value) in [("Neg", -r.min), ("Pos", r.max)] {
        let mut rule = RuleSpec::new(kind, &format!("{prefix}:{zone}{suffix}"), value);
        rule.basis = basis.to_string();
        rule.assertion_ids = assertion_ids.to_vec();
        out.push(rule);
    }
}

/// Compiles joint ranges into rules. Property IRIs are `q42:rom:<joint>:<axis>:<zone>{Neg,Pos}`,
/// with each value the magnitude from neutral in that direction.
pub fn joint_rom_rules(joints: &[JointRom]) -> Vec<RuleSpec> {
    let mut out = Vec::new();
    for j in joints {
        for a in &j.axes {
            let prefix = format!("q42:rom:{}:{}", j.joint, a.axis.as_str());
            push_range(
                &mut out,
                RuleKind::Permit,
                &prefix,
                "healthy",
                a.healthy,
                &a.healthy_basis,
                &a.healthy_assertion_ids,
            );
            push_range(
                &mut out,
                RuleKind::Forbid,
                &prefix,
                "injury",
                a.injury,
                &a.injury_basis,
                &a.injury_assertion_ids,
            );
            if let Some(r) = a.injured_healthy {
                push_range(
                    &mut out,
                    RuleKind::Obligate,
                    &prefix,
                    "injuredRange",
                    r,
                    &a.injured_basis,
                    &a.injured_assertion_ids,
                );
            }
        }
        if let Some(g) = &j.injured_gait {
            let guarding = if j.injured_guarding > 0.0 {
                j.injured_guarding
            } else {
                1.0
            };
            let mut rule = RuleSpec::new(
                RuleKind::Obligate,
                &format!("q42:rom:{}:injuredGait:{g}", j.joint),
                guarding,
            );
            rule.basis = j.gait_basis.clone();
            rule.assertion_ids = j.gait_assertion_ids.clone();
            out.push(rule);
        }
    }
    out
}
