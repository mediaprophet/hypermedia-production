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
use qualia_core_db::render::skeletal::ik_motor::SwingTwistConeConstraint;
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
    ("right_shoulder", &["shoulder_r"]),
    ("right_elbow", &["elbow_r"]),
    ("right_wrist", &["right_wrist_wing", "forepaw_r"]),
    ("left_scapula", &["scapula_l", "left_scapulothoracic"]),
    ("right_scapula", &["scapula_r", "right_scapulothoracic"]),
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

/// Coupled Quaternion Swing-Twist reach cone specification for a canonical or `did:q42:` joint.
#[derive(Clone, Debug, PartialEq)]
pub struct SwingTwistConeSpec {
    pub joint: String,
    pub twist_axis: [f32; 3],
    pub healthy_swing_y_rad: f64,
    pub healthy_swing_z_rad: f64,
    pub healthy_twist: AngleRange,
    pub injury_swing_y_rad: f64,
    pub injury_swing_z_rad: f64,
    pub injury_twist: AngleRange,
    pub injured_swing_y_rad: Option<f64>,
    pub injured_swing_z_rad: Option<f64>,
    pub injured_twist: Option<AngleRange>,
    pub twist_coupling_narrowing: f64,
    pub healthy_basis: String,
    pub injury_basis: String,
    pub injured_basis: String,
    pub healthy_assertion_ids: Vec<String>,
    pub injury_assertion_ids: Vec<String>,
    pub injured_assertion_ids: Vec<String>,
}

/// Pose violation record emitted by [`validate_pose_swing_twist`] and [`project_pose_to_swing_twist_rom`].
#[derive(Clone, Debug, PartialEq)]
pub struct JointPoseViolation {
    pub joint: String,
    pub zone: MotionZone,
    pub geodesic_correction_rad: f64,
}

/// Aggregate Swing-Twist validation report across a multi-joint pose.
#[derive(Clone, Debug, PartialEq)]
pub struct PoseSwingTwistReport {
    pub valid: bool,
    pub worst_zone: MotionZone,
    pub violations: Vec<JointPoseViolation>,
}

#[inline]
fn range_half_angle(r: AngleRange) -> f64 {
    r.min.abs().max(r.max.abs())
}

#[inline]
fn normalize_quat_f64(q: [f32; 4]) -> [f64; 4] {
    if q.iter().any(|v| !v.is_finite()) {
        return [1.0, 0.0, 0.0, 0.0];
    }
    let n = (q[0] as f64).powi(2)
        + (q[1] as f64).powi(2)
        + (q[2] as f64).powi(2)
        + (q[3] as f64).powi(2);
    if n < 1e-12 {
        return [1.0, 0.0, 0.0, 0.0];
    }
    let inv = 1.0 / n.sqrt();
    [
        q[0] as f64 * inv,
        q[1] as f64 * inv,
        q[2] as f64 * inv,
        q[3] as f64 * inv,
    ]
}

#[inline]
fn quat_geodesic_rad(a: [f32; 4], b: [f32; 4]) -> f64 {
    let qa = normalize_quat_f64(a);
    let qb = normalize_quat_f64(b);
    let dot = qa[0] * qb[0] + qa[1] * qb[1] + qa[2] * qb[2] + qa[3] * qb[3];
    let sign = if dot < 0.0 { -1.0 } else { 1.0 };
    let mut diff_sq = 0.0_f64;
    let mut sum_sq = 0.0_f64;
    for i in 0..4 {
        let sb = sign * qb[i];
        let d = qa[i] - sb;
        let s = qa[i] + sb;
        diff_sq += d * d;
        sum_sq += s * s;
    }
    4.0 * diff_sq.sqrt().atan2(sum_sq.sqrt())
}

fn resolve_joint_identifier(name: &str) -> Option<String> {
    if let Some(c) = canonical_joint(name) {
        return Some(c.to_string());
    }
    let trimmed = name.trim();
    if trimmed.starts_with("did:q42:") && trimmed.len() > "did:q42:".len() {
        return Some(trimmed.to_string());
    }
    None
}

impl SwingTwistConeSpec {
    /// Upgrades a legacy axis-based [`JointRom`] into a coupled [`SwingTwistConeSpec`].
    pub fn from_joint_rom(rom: &JointRom) -> Self {
        let flex = rom.axis(Axis::Flexion);
        let abd = rom.axis(Axis::Abduction);
        let rot = rom.axis(Axis::Rotation);

        let default_range = AngleRange { min: 0.0, max: 0.0 };
        let healthy_twist = rot.map(|a| a.healthy).unwrap_or(default_range);
        let injury_twist = rot.map(|a| a.injury).unwrap_or(default_range);

        let primary = flex.or(abd).or(rot);
        let healthy_basis = primary
            .map(|a| a.healthy_basis.clone())
            .unwrap_or_else(|| "model_assumption".to_string());
        let injury_basis = primary
            .map(|a| a.injury_basis.clone())
            .unwrap_or_else(|| "model_assumption".to_string());
        let injured_basis = primary
            .map(|a| a.injured_basis.clone())
            .unwrap_or_else(|| "model_assumption".to_string());

        let mut healthy_ids = Vec::new();
        let mut injury_ids = Vec::new();
        let mut injured_ids = Vec::new();
        for a in &rom.axes {
            for id in &a.healthy_assertion_ids {
                if !healthy_ids.contains(id) {
                    healthy_ids.push(id.clone());
                }
            }
            for id in &a.injury_assertion_ids {
                if !injury_ids.contains(id) {
                    injury_ids.push(id.clone());
                }
            }
            for id in &a.injured_assertion_ids {
                if !injured_ids.contains(id) {
                    injured_ids.push(id.clone());
                }
            }
        }

        Self {
            joint: rom.joint.to_string(),
            twist_axis: [1.0, 0.0, 0.0],
            healthy_swing_y_rad: flex.map(|a| range_half_angle(a.healthy)).unwrap_or(0.0),
            healthy_swing_z_rad: abd.map(|a| range_half_angle(a.healthy)).unwrap_or(0.0),
            healthy_twist,
            injury_swing_y_rad: flex.map(|a| range_half_angle(a.injury)).unwrap_or(0.0),
            injury_swing_z_rad: abd.map(|a| range_half_angle(a.injury)).unwrap_or(0.0),
            injury_twist,
            injured_swing_y_rad: flex.and_then(|a| a.injured_healthy).map(range_half_angle),
            injured_swing_z_rad: abd.and_then(|a| a.injured_healthy).map(range_half_angle),
            injured_twist: rot.and_then(|a| a.injured_healthy),
            twist_coupling_narrowing: 0.0,
            healthy_basis,
            injury_basis,
            injured_basis,
            healthy_assertion_ids: healthy_ids,
            injury_assertion_ids: injury_ids,
            injured_assertion_ids: injured_ids,
        }
    }

    /// Converts the active healthy (or narrowed injured) bounds into [`SwingTwistConeConstraint`].
    pub fn to_healthy_constraint(&self, injured: bool) -> SwingTwistConeConstraint {
        let sy = if injured {
            self.injured_swing_y_rad.unwrap_or(self.healthy_swing_y_rad)
        } else {
            self.healthy_swing_y_rad
        };
        let sz = if injured {
            self.injured_swing_z_rad.unwrap_or(self.healthy_swing_z_rad)
        } else {
            self.healthy_swing_z_rad
        };
        let twist = if injured {
            self.injured_twist.unwrap_or(self.healthy_twist)
        } else {
            self.healthy_twist
        };
        SwingTwistConeConstraint::elliptical(
            self.twist_axis,
            sy as f32,
            sz as f32,
            twist.min as f32,
            twist.max as f32,
        )
        .with_twist_coupling(self.twist_coupling_narrowing as f32)
    }

    /// Converts the injury limit bounds into [`SwingTwistConeConstraint`].
    pub fn to_injury_constraint(&self) -> SwingTwistConeConstraint {
        SwingTwistConeConstraint::elliptical(
            self.twist_axis,
            self.injury_swing_y_rad as f32,
            self.injury_swing_z_rad as f32,
            self.injury_twist.min as f32,
            self.injury_twist.max as f32,
        )
        .with_twist_coupling(self.twist_coupling_narrowing as f32)
    }

    /// Classifies quaternion `q` (`[w, x, y, z]`) into `Healthy`, `Strain`, or `Injury`.
    pub fn classify_quaternion(&self, q: [f32; 4], injured: bool, laxity_scale: f32) -> MotionZone {
        let tol = 1e-4_f32;
        if self
            .to_healthy_constraint(injured)
            .is_quaternion_within(q, laxity_scale, tol)
        {
            MotionZone::Healthy
        } else if self
            .to_injury_constraint()
            .is_quaternion_within(q, laxity_scale, tol)
        {
            MotionZone::Strain
        } else {
            MotionZone::Injury
        }
    }

    /// Projects quaternion `q` (`[w, x, y, z]`) into the active Swing-Twist reach cone.
    pub fn project_quaternion(&self, q: [f32; 4], injured: bool, laxity_scale: f32) -> [f32; 4] {
        self.to_healthy_constraint(injured)
            .project_quaternion(q, laxity_scale)
    }

    /// Compiles this Swing-Twist cone into `Q42LOGIC` `Permit` / `Forbid` / `Obligate` rules.
    pub fn compile_rules(&self) -> Vec<RuleSpec> {
        let mut out = Vec::new();
        let prefix = format!("q42:cone:{}", self.joint);
        let push = |out: &mut Vec<RuleSpec>,
                    kind: RuleKind,
                    suffix: &str,
                    value: f64,
                    basis: &str,
                    ids: &[String]| {
            let mut rule = RuleSpec::new(kind, &format!("{prefix}:{suffix}"), value.max(0.0));
            rule.basis = basis.to_string();
            rule.assertion_ids = ids.to_vec();
            out.push(rule);
        };

        push(
            &mut out,
            RuleKind::Permit,
            "healthySwingYRad",
            self.healthy_swing_y_rad,
            &self.healthy_basis,
            &self.healthy_assertion_ids,
        );
        push(
            &mut out,
            RuleKind::Permit,
            "healthySwingZRad",
            self.healthy_swing_z_rad,
            &self.healthy_basis,
            &self.healthy_assertion_ids,
        );
        push(
            &mut out,
            RuleKind::Permit,
            "healthyTwistNeg",
            -self.healthy_twist.min,
            &self.healthy_basis,
            &self.healthy_assertion_ids,
        );
        push(
            &mut out,
            RuleKind::Permit,
            "healthyTwistPos",
            self.healthy_twist.max,
            &self.healthy_basis,
            &self.healthy_assertion_ids,
        );

        push(
            &mut out,
            RuleKind::Forbid,
            "injurySwingYRad",
            self.injury_swing_y_rad,
            &self.injury_basis,
            &self.injury_assertion_ids,
        );
        push(
            &mut out,
            RuleKind::Forbid,
            "injurySwingZRad",
            self.injury_swing_z_rad,
            &self.injury_basis,
            &self.injury_assertion_ids,
        );
        push(
            &mut out,
            RuleKind::Forbid,
            "injuryTwistNeg",
            -self.injury_twist.min,
            &self.injury_basis,
            &self.injury_assertion_ids,
        );
        push(
            &mut out,
            RuleKind::Forbid,
            "injuryTwistPos",
            self.injury_twist.max,
            &self.injury_basis,
            &self.injury_assertion_ids,
        );

        if self.twist_coupling_narrowing > 0.0 {
            push(
                &mut out,
                RuleKind::Obligate,
                "twistCouplingNarrowing01",
                self.twist_coupling_narrowing,
                &self.healthy_basis,
                &self.healthy_assertion_ids,
            );
        }
        if let Some(sy) = self.injured_swing_y_rad {
            push(
                &mut out,
                RuleKind::Obligate,
                "injuredSwingYRad",
                sy,
                &self.injured_basis,
                &self.injured_assertion_ids,
            );
        }
        if let Some(sz) = self.injured_swing_z_rad {
            push(
                &mut out,
                RuleKind::Obligate,
                "injuredSwingZRad",
                sz,
                &self.injured_basis,
                &self.injured_assertion_ids,
            );
        }
        if let Some(t) = self.injured_twist {
            push(
                &mut out,
                RuleKind::Obligate,
                "injuredTwistNeg",
                -t.min,
                &self.injured_basis,
                &self.injured_assertion_ids,
            );
            push(
                &mut out,
                RuleKind::Obligate,
                "injuredTwistPos",
                t.max,
                &self.injured_basis,
                &self.injured_assertion_ids,
            );
        }
        out
    }
}

/// Converts a slice of legacy [`JointRom`] definitions into coupled [`SwingTwistConeSpec`]s.
pub fn joint_rom_to_swing_twist_specs(joints: &[JointRom]) -> Vec<SwingTwistConeSpec> {
    joints.iter().map(SwingTwistConeSpec::from_joint_rom).collect()
}

/// Compiles coupled [`SwingTwistConeSpec`] definitions into `Q42LOGIC` rules.
pub fn swing_twist_cone_rules(specs: &[SwingTwistConeSpec]) -> Vec<RuleSpec> {
    specs.iter().flat_map(SwingTwistConeSpec::compile_rules).collect()
}

fn parse_nonneg_rad(v: &JsonValue, ctx: &str) -> Result<f64, String> {
    let val = v
        .as_f64()
        .ok_or_else(|| format!("{ctx} must be a number"))?;
    if !val.is_finite() || val < 0.0 || val > std::f64::consts::PI {
        return Err(format!("{ctx} must be finite and in 0.0..=PI"));
    }
    Ok(val)
}

fn parse_twist_axis(v: Option<&JsonValue>, ctx: &str) -> Result<[f32; 3], String> {
    let Some(v) = v else {
        return Ok([1.0, 0.0, 0.0]);
    };
    let arr = v
        .as_array()
        .filter(|a| a.len() == 3)
        .ok_or_else(|| format!("{ctx} must be [x, y, z]"))?;
    let x = arr[0].as_f64().ok_or_else(|| format!("{ctx}[0] must be a number"))?;
    let y = arr[1].as_f64().ok_or_else(|| format!("{ctx}[1] must be a number"))?;
    let z = arr[2].as_f64().ok_or_else(|| format!("{ctx}[2] must be a number"))?;
    if !x.is_finite() || !y.is_finite() || !z.is_finite() {
        return Err(format!("{ctx} components must be finite"));
    }
    let len = (x * x + y * y + z * z).sqrt();
    if len <= 1e-6 {
        return Err(format!("{ctx} must be non-zero"));
    }
    Ok([(x / len) as f32, (y / len) as f32, (z / len) as f32])
}

fn parse_swing_twist_entry(i: usize, v: &JsonValue) -> Result<SwingTwistConeSpec, String> {
    let ctx = format!("joints[{i}]");
    let obj = v.as_object().ok_or_else(|| format!("{ctx} must be a record"))?;
    if !obj.contains_key("swing_twist") {
        // Backward-compatible upgrade from legacy `axes` declaration.
        let legacy = parse_joint(i, v)?;
        return Ok(SwingTwistConeSpec::from_joint_rom(&legacy));
    }
    check_keys(obj, &["id", "swing_twist", "when_injured"], &ctx)?;
    let id = obj
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("{ctx}: 'id' string is required"))?;
    let joint = resolve_joint_identifier(id)
        .ok_or_else(|| format!("{ctx}: unknown rig joint '{id}'"))?;

    let st_ctx = format!("{ctx}.swing_twist");
    let st = obj
        .get("swing_twist")
        .and_then(|v| v.as_object())
        .ok_or_else(|| format!("{st_ctx} must be a record"))?;
    check_keys(
        st,
        &[
            "twist_axis",
            "healthy_swing_y_rad",
            "healthy_swing_z_rad",
            "healthy_twist",
            "injury_swing_y_rad",
            "injury_swing_z_rad",
            "injury_twist",
            "twist_coupling_narrowing",
            "basis",
            "assertion_ids",
        ],
        &st_ctx,
    )?;

    let twist_axis = parse_twist_axis(st.get("twist_axis"), &format!("{st_ctx}.twist_axis"))?;
    let healthy_swing_y_rad = parse_nonneg_rad(
        st.get("healthy_swing_y_rad")
            .ok_or_else(|| format!("{st_ctx}: 'healthy_swing_y_rad' is required"))?,
        &format!("{st_ctx}.healthy_swing_y_rad"),
    )?;
    let healthy_swing_z_rad = parse_nonneg_rad(
        st.get("healthy_swing_z_rad")
            .ok_or_else(|| format!("{st_ctx}: 'healthy_swing_z_rad' is required"))?,
        &format!("{st_ctx}.healthy_swing_z_rad"),
    )?;
    let healthy_twist = parse_range(
        st.get("healthy_twist")
            .ok_or_else(|| format!("{st_ctx}: 'healthy_twist' is required"))?,
        &format!("{st_ctx}.healthy_twist"),
    )?;

    let injury_swing_y_rad = parse_nonneg_rad(
        st.get("injury_swing_y_rad")
            .ok_or_else(|| format!("{st_ctx}: 'injury_swing_y_rad' is required"))?,
        &format!("{st_ctx}.injury_swing_y_rad"),
    )?;
    let injury_swing_z_rad = parse_nonneg_rad(
        st.get("injury_swing_z_rad")
            .ok_or_else(|| format!("{st_ctx}: 'injury_swing_z_rad' is required"))?,
        &format!("{st_ctx}.injury_swing_z_rad"),
    )?;
    let injury_twist = parse_range(
        st.get("injury_twist")
            .ok_or_else(|| format!("{st_ctx}: 'injury_twist' is required"))?,
        &format!("{st_ctx}.injury_twist"),
    )?;

    if injury_swing_y_rad < healthy_swing_y_rad
        || injury_swing_z_rad < healthy_swing_z_rad
        || !injury_twist.encloses(&healthy_twist)
    {
        return Err(format!(
            "{st_ctx}: injury limits must enclose healthy swing-twist bounds"
        ));
    }

    let twist_coupling_narrowing = match st.get("twist_coupling_narrowing") {
        None => 0.0,
        Some(v) => {
            let val = v
                .as_f64()
                .ok_or_else(|| format!("{st_ctx}.twist_coupling_narrowing must be a number"))?;
            if !val.is_finite() || !(0.0..=0.95).contains(&val) {
                return Err(format!(
                    "{st_ctx}.twist_coupling_narrowing must be in 0.0..=0.95"
                ));
            }
            val
        }
    };

    let (basis, assertion_ids) = axis_citation(st, &st_ctx)?;

    let mut injured_swing_y_rad = None;
    let mut injured_swing_z_rad = None;
    let mut injured_twist = None;
    if let Some(w) = obj.get("when_injured") {
        let wctx = format!("{ctx}.when_injured");
        let w = w.as_object().ok_or_else(|| format!("{wctx} must be a record"))?;
        check_keys(
            w,
            &["swing_y_rad", "swing_z_rad", "twist", "gait", "guarding"],
            &wctx,
        )?;
        if let Some(vy) = w.get("swing_y_rad") {
            let val = parse_nonneg_rad(vy, &format!("{wctx}.swing_y_rad"))?;
            if val > healthy_swing_y_rad {
                return Err(format!(
                    "{wctx}.swing_y_rad must lie within healthy_swing_y_rad"
                ));
            }
            injured_swing_y_rad = Some(val);
        }
        if let Some(vz) = w.get("swing_z_rad") {
            let val = parse_nonneg_rad(vz, &format!("{wctx}.swing_z_rad"))?;
            if val > healthy_swing_z_rad {
                return Err(format!(
                    "{wctx}.swing_z_rad must lie within healthy_swing_z_rad"
                ));
            }
            injured_swing_z_rad = Some(val);
        }
        if let Some(vt) = w.get("twist") {
            let r = parse_range(vt, &format!("{wctx}.twist"))?;
            if !healthy_twist.encloses(&r) {
                return Err(format!("{wctx}.twist must lie within healthy_twist"));
            }
            injured_twist = Some(r);
        }
    }

    Ok(SwingTwistConeSpec {
        joint,
        twist_axis,
        healthy_swing_y_rad,
        healthy_swing_z_rad,
        healthy_twist,
        injury_swing_y_rad,
        injury_swing_z_rad,
        injury_twist,
        injured_swing_y_rad,
        injured_swing_z_rad,
        injured_twist,
        twist_coupling_narrowing,
        healthy_basis: basis.clone(),
        injury_basis: basis.clone(),
        injured_basis: basis,
        healthy_assertion_ids: assertion_ids.clone(),
        injury_assertion_ids: assertion_ids.clone(),
        injured_assertion_ids: assertion_ids,
    })
}

/// Parses `joints` declarations as coupled [`SwingTwistConeSpec`] entries (supporting both
/// explicit `swing_twist` cones and legacy `axes` entries, plus `did:q42:*` joint graphs).
pub fn parse_swing_twist_specs(spec: Option<&JsonValue>) -> Result<Vec<SwingTwistConeSpec>, String> {
    let Some(v) = spec.and_then(|s| s.get("joints")) else {
        return Ok(Vec::new());
    };
    let arr = v
        .as_array()
        .ok_or_else(|| "spec 'joints' must be a list".to_string())?;
    if arr.len() > MAX_JOINTS {
        return Err(format!("spec 'joints' exceeds {MAX_JOINTS} entries"));
    }
    let mut out: Vec<SwingTwistConeSpec> = Vec::with_capacity(arr.len());
    for (i, j) in arr.iter().enumerate() {
        let parsed = parse_swing_twist_entry(i, j)?;
        if out.iter().any(|x| x.joint == parsed.joint) {
            return Err(format!("joints[{i}]: joint '{}' declared twice", parsed.joint));
        }
        out.push(parsed);
    }
    Ok(out)
}

fn find_cone_spec<'a>(specs: &'a [SwingTwistConeSpec], name: &str) -> Option<&'a SwingTwistConeSpec> {
    let resolved = resolve_joint_identifier(name).unwrap_or_else(|| name.trim().to_string());
    specs.iter().find(|s| s.joint == resolved)
}

fn is_joint_injured(injured_joints: &[&str], canonical_joint_name: &str) -> bool {
    injured_joints.iter().any(|name| {
        resolve_joint_identifier(name)
            .map(|r| r == canonical_joint_name)
            .unwrap_or(false)
    })
}

/// Validates every joint quaternion (`[w, x, y, z]`) in `pose` against `specs` with `laxity_scale`.
pub fn validate_pose_swing_twist(
    specs: &[SwingTwistConeSpec],
    pose: &[(&str, [f32; 4])],
    injured_joints: &[&str],
    laxity_scale: f32,
) -> PoseSwingTwistReport {
    let mut violations = Vec::new();
    let mut worst_zone = MotionZone::Healthy;

    for &(joint_name, q) in pose {
        let Some(spec) = find_cone_spec(specs, joint_name) else {
            continue;
        };
        let injured = is_joint_injured(injured_joints, &spec.joint);
        let zone = spec.classify_quaternion(q, injured, laxity_scale);
        if zone != MotionZone::Healthy {
            let proj = spec.project_quaternion(q, injured, laxity_scale);
            let geodesic_correction_rad = quat_geodesic_rad(q, proj);
            if zone == MotionZone::Injury
                || (zone == MotionZone::Strain && worst_zone == MotionZone::Healthy)
            {
                worst_zone = zone;
            }
            violations.push(JointPoseViolation {
                joint: spec.joint.clone(),
                zone,
                geodesic_correction_rad,
            });
        }
    }

    PoseSwingTwistReport {
        valid: violations.is_empty(),
        worst_zone,
        violations,
    }
}

/// Projects every joint quaternion (`[w, x, y, z]`) in `pose` into its active Swing-Twist reach cone
/// scaled by `laxity_scale`, returning the pre-projection violation report.
pub fn project_pose_to_swing_twist_rom(
    specs: &[SwingTwistConeSpec],
    pose: &mut [(&str, [f32; 4])],
    injured_joints: &[&str],
    laxity_scale: f32,
) -> PoseSwingTwistReport {
    let mut violations = Vec::new();
    let mut worst_zone = MotionZone::Healthy;

    for (joint_name, q) in pose.iter_mut() {
        let Some(spec) = find_cone_spec(specs, joint_name) else {
            continue;
        };
        let injured = is_joint_injured(injured_joints, &spec.joint);
        let zone = spec.classify_quaternion(*q, injured, laxity_scale);
        let proj = spec.project_quaternion(*q, injured, laxity_scale);
        if zone != MotionZone::Healthy {
            let geodesic_correction_rad = quat_geodesic_rad(*q, proj);
            if zone == MotionZone::Injury
                || (zone == MotionZone::Strain && worst_zone == MotionZone::Healthy)
            {
                worst_zone = zone;
            }
            violations.push(JointPoseViolation {
                joint: spec.joint.clone(),
                zone,
                geodesic_correction_rad,
            });
        }
        *q = proj;
    }

    PoseSwingTwistReport {
        valid: violations.is_empty(),
        worst_zone,
        violations,
    }
}

#[cfg(test)]
mod swing_twist_rom_tests {
    use super::super::rule_pack::{compile_rule_pack_q42, validate_q42_logic, DOMAIN_FAUNA, FAUNA_CONTEXT_DID};
    use super::*;
    use serde_json::json;

    fn axis_angle(axis: [f32; 3], angle: f32) -> [f32; 4] {
        let len = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        let (s, c) = (0.5 * angle).sin_cos();
        [c, axis[0] / len * s, axis[1] / len * s, axis[2] / len * s]
    }

    #[test]
    fn swing_twist_cone_spec_parses_did_q42_and_compiles_q42logic_rules() {
        let doc = json!({
            "joints": [
                {
                    "id": "did:q42:joint:glenohumeral_right",
                    "swing_twist": {
                        "twist_axis": [0.0, -1.0, 0.0],
                        "healthy_swing_y_rad": 1.2,
                        "healthy_swing_z_rad": 0.9,
                        "healthy_twist": [-0.6, 0.8],
                        "injury_swing_y_rad": 1.5,
                        "injury_swing_z_rad": 1.1,
                        "injury_twist": [-0.9, 1.0],
                        "twist_coupling_narrowing": 0.35,
                        "basis": "measured",
                        "assertion_ids": ["ea-gh-01"]
                    },
                    "when_injured": {
                        "swing_y_rad": 0.5,
                        "swing_z_rad": 0.4,
                        "twist": [-0.2, 0.2]
                    }
                }
            ]
        });

        let specs = parse_swing_twist_specs(Some(&doc)).unwrap();
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].joint, "did:q42:joint:glenohumeral_right");

        let rules = swing_twist_cone_rules(&specs);
        assert!(rules.iter().any(|r| r.property == "q42:cone:did:q42:joint:glenohumeral_right:healthySwingYRad" && r.kind == RuleKind::Permit));
        assert!(rules.iter().any(|r| r.property == "q42:cone:did:q42:joint:glenohumeral_right:injurySwingYRad" && r.kind == RuleKind::Forbid));
        assert!(rules.iter().any(|r| r.property == "q42:cone:did:q42:joint:glenohumeral_right:injuredSwingYRad" && r.kind == RuleKind::Obligate));

        let q42 = compile_rule_pack_q42("human-adult", DOMAIN_FAUNA, FAUNA_CONTEXT_DID, &rules).unwrap();
        let summary = validate_q42_logic(&q42).unwrap();
        assert_eq!(summary.quin_count as usize, rules.len());
    }

    #[test]
    fn validate_and_project_pose_swing_twist_supports_laxity_and_injury() {
        let doc = json!({
            "joints": [
                {
                    "id": "shoulder_r",
                    "swing_twist": {
                        "twist_axis": [1.0, 0.0, 0.0],
                        "healthy_swing_y_rad": 0.6,
                        "healthy_swing_z_rad": 0.4,
                        "healthy_twist": [-0.3, 0.3],
                        "injury_swing_y_rad": 0.9,
                        "injury_swing_z_rad": 0.7,
                        "injury_twist": [-0.5, 0.5]
                    },
                    "when_injured": {
                        "swing_y_rad": 0.3,
                        "swing_z_rad": 0.2
                    }
                }
            ]
        });
        let specs = parse_swing_twist_specs(Some(&doc)).unwrap();

        let q_strain = axis_angle([0.0, 1.0, 0.0], 0.75);
        let q_injury = axis_angle([0.0, 1.0, 0.0], 1.25);

        let r_strain = validate_pose_swing_twist(&specs, &[("right_shoulder", q_strain)], &[], 1.0);
        assert!(!r_strain.valid);
        assert_eq!(r_strain.worst_zone, MotionZone::Strain);

        // Relaxin / hypermobility laxity_scale = 1.4 brings 0.75 rad within widened 0.84 rad cone.
        let r_lax = validate_pose_swing_twist(&specs, &[("right_shoulder", q_strain)], &[], 1.4);
        assert!(r_lax.valid);
        assert_eq!(r_lax.worst_zone, MotionZone::Healthy);

        let mut pose = [("right_shoulder", q_injury)];
        let pre = project_pose_to_swing_twist_rom(&specs, &mut pose, &["shoulder_r"], 1.0);
        assert!(!pre.valid);
        assert_eq!(pre.worst_zone, MotionZone::Injury);
        assert!(pre.violations[0].geodesic_correction_rad > 0.8);

        let post = validate_pose_swing_twist(&specs, &pose, &["shoulder_r"], 1.0);
        assert!(post.valid, "projected pose must validate as Healthy: {post:?}");
    }
}
