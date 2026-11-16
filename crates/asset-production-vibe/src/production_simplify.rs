//! Admitted semantic decimate (`decimate_semantic_admitted_3`) + P07 `CollapseTransfer`.
//! Geometric QEM (`mode=decimate`) stays in `manifold_ops`; this module is the semantic path.
use crate::{fields, integer, list, num, required, Args};
use qualia_core_db::specialized_libs::computational_geometry::attribute_transfer::{
    CollapseTransfer, ProtectedDomain, TransferError, TransferRow,
};
use qualia_core_db::specialized_libs::computational_geometry::decimate_3_semantic::admission::{
    decimate_semantic_admitted_3, AdmissionError, AdmissionIdentity, AdmissionInput, AdmissionLimits,
    AdmissionOutput, AdmissionPolicy, AdmissionReport, AdmissionScratch, AdmissionSlot, DomainLayout,
    SemanticDomains, Termination, TopologyMode,
};
use qualia_core_db::specialized_libs::computational_geometry::decimate_3_semantic::normalized_cost::NormalizedCostProfile;
use qualia_core_db::specialized_libs::computational_geometry::geometry_workspace::production_budget::{
    BudgetError, CapacityPlan, ProductionBudget,
};
use qualia_core_db::specialized_libs::computational_geometry::geometry_workspace::Cancellation;
use qualia_core_db::specialized_libs::computational_geometry::production_contracts::{
    CollapseEvent, ContentId, ContractError, CornerDelta, CornerId, FaceDelta, ReasonCode, RemapBuffers,
};
use qualia_core_db::specialized_libs::computational_geometry::{
    Point3, SemanticVertexAttributes, SparseEntry,
};
use std::mem::size_of;
use vibe::Value;

pub(super) const SEMANTIC_FIELDS: &[&str] = &[
    "mesh",
    "target_triangles",
    "max_error",
    "mode",
    "domains",
    "locks",
];

pub(super) fn is_semantic_mode(mode: &str) -> bool {
    matches!(mode, "semantic" | "admitted")
}

#[derive(Debug)]
pub(super) struct SemanticRequest {
    pub target_triangles: usize,
    pub max_error: Option<f64>,
}

pub(super) fn parse_semantic_request(a: &Args) -> Result<SemanticRequest, String> {
    fields(a, SEMANTIC_FIELDS)?;
    let target_triangles = integer(required(a, "target_triangles")?, "target_triangles")?;
    if target_triangles < 4 || target_triangles > 200_000 {
        return Err("target_triangles outside 4..=200000 range".into());
    }
    let max_error = match a.get("max_error") {
        None => None,
        Some(v) => {
            let value = num(v)?;
            if !value.is_finite() || value < 0.0 || value > 1e4 {
                return Err("max_error outside 0..=1e4 range".into());
            }
            Some(value)
        }
    };
    Ok(SemanticRequest {
        target_triangles,
        max_error,
    })
}

pub(super) fn validate_semantic_input(
    vertex_count: usize,
    triangle_count: usize,
    target_triangles: usize,
) -> Result<(), String> {
    if vertex_count == 0 || triangle_count == 0 {
        return Err("invalid_input: empty mesh".into());
    }
    if target_triangles > triangle_count {
        return Err(format!(
            "invalid_input: impossible target_triangles {target_triangles} exceeds input face count {triangle_count}"
        ));
    }
    Ok(())
}

pub(super) fn parse_domain_list(value: &Value, vertex_count: usize) -> Result<Vec<u64>, String> {
    let items = list(value)?;
    if items.len() != vertex_count {
        return Err(format!(
            "domains length {} does not match vertex count {vertex_count}",
            items.len()
        ));
    }
    items
        .iter()
        .map(|item| integer(item, "domains").map(|n| n as u64))
        .collect()
}

pub(super) fn parse_lock_list(value: &Value, vertex_count: usize) -> Result<Vec<bool>, String> {
    let items = list(value)?;
    if items.is_empty() {
        return Ok(vec![false; vertex_count]);
    }
    if items.iter().all(|item| matches!(item, Value::Bool(_))) {
        if items.len() != vertex_count {
            return Err(format!(
                "locks length {} does not match vertex count {vertex_count}",
                items.len()
            ));
        }
        return Ok(items
            .iter()
            .map(|item| matches!(item, Value::Bool(true)))
            .collect());
    }
    if items.len() == vertex_count {
        return items
            .iter()
            .map(|item| integer(item, "locks").map(|n| n != 0))
            .collect();
    }
    let mut locked = vec![false; vertex_count];
    for item in items {
        let index = integer(item, "locks")?;
        if index >= vertex_count {
            return Err(format!("locks index {index} is outside vertex count {vertex_count}"));
        }
        locked[index] = true;
    }
    Ok(locked)
}

pub(super) fn decode_domain_bytes(bytes: &[u8], vertex_count: usize) -> Result<Vec<u64>, String> {
    let need = vertex_count
        .checked_mul(8)
        .ok_or("capacity_exhausted: domains byte length overflow")?;
    if bytes.len() != need {
        return Err(format!(
            "domains handle byte length {} does not match vertex count {vertex_count} * 8",
            bytes.len()
        ));
    }
    Ok(bytes
        .chunks_exact(8)
        .map(|chunk| u64::from_le_bytes(chunk.try_into().expect("domains chunk")))
        .collect())
}

pub(super) fn decode_lock_bytes(bytes: &[u8], vertex_count: usize) -> Result<Vec<bool>, String> {
    if bytes.len() == vertex_count {
        return Ok(bytes.iter().map(|flag| *flag != 0).collect());
    }
    let need = vertex_count
        .checked_mul(8)
        .ok_or("capacity_exhausted: locks byte length overflow")?;
    if bytes.len() != need {
        return Err(format!(
            "locks handle byte length {} does not match vertex count {vertex_count} or {vertex_count} * 8",
            bytes.len()
        ));
    }
    Ok(bytes
        .chunks_exact(8)
        .map(|chunk| u64::from_le_bytes(chunk.try_into().expect("locks chunk")) != 0)
        .collect())
}

pub(super) fn transfer_entry_count(vertex_count: usize) -> Result<usize, String> {
    vertex_count
        .checked_mul(vertex_count.max(1))
        .ok_or_else(|| "capacity_exhausted: transfer entry count overflow".into())
}

pub(super) fn estimated_scratch_bytes(vertex_count: usize, triangle_count: usize) -> Result<usize, String> {
    let corners = triangle_count
        .checked_mul(3)
        .ok_or("capacity_exhausted: corner count overflow")?;
    let entries = transfer_entry_count(vertex_count)?;
    let mut total = 0usize;
    for (count, size) in [
        (vertex_count, size_of::<Point3>()),
        (vertex_count, size_of::<SemanticVertexAttributes>()),
        (vertex_count, size_of::<SemanticDomains>()),
        (vertex_count, size_of::<ProtectedDomain>()),
        (vertex_count, size_of::<u64>()),
        (vertex_count, size_of::<TransferRow>()),
        (entries, size_of::<SparseEntry>()),
        (vertex_count.max(triangle_count), size_of::<AdmissionSlot>()),
        (triangle_count, size_of::<FaceDelta>()),
        (corners, size_of::<CornerDelta>()),
        (vertex_count.max(1).saturating_mul(16), 1),
        (vertex_count, size_of::<Point3>()),
        (vertex_count, size_of::<SemanticVertexAttributes>()),
        (vertex_count, size_of::<SemanticDomains>()),
        (triangle_count, size_of::<[u32; 3]>()),
        (vertex_count, size_of::<u32>()),
        (vertex_count, size_of::<u32>()),
        (triangle_count, size_of::<u32>()),
        (corners, size_of::<Option<CornerId>>()),
        (vertex_count.saturating_sub(1), size_of::<Option<CollapseEvent>>()),
    ] {
        let bytes = count
            .checked_mul(size)
            .ok_or("capacity_exhausted: semantic buffer size overflow")?;
        total = total
            .checked_add(bytes)
            .ok_or("capacity_exhausted: semantic buffer size overflow")?;
    }
    Ok(total)
}

fn reason_status(code: ReasonCode) -> &'static str {
    match code {
        ReasonCode::CapacityExhausted => "capacity_exhausted",
        ReasonCode::UnsatisfiedConstraint => "unsatisfied_constraint",
        ReasonCode::Cancelled => "cancelled",
        ReasonCode::UnsupportedCapability => "unsupported_capability",
        _ => "invalid_input",
    }
}

fn transfer_status(error: TransferError) -> String {
    let status = reason_status(ContractError::from(error).code);
    format!("semantic simplify failed: {status}: {error:?}")
}

fn admission_status(error: AdmissionError) -> String {
    match error {
        AdmissionError::Contract(contract) => format!(
            "semantic simplify failed: {}: {}",
            reason_status(contract.code),
            contract.field
        ),
        AdmissionError::Budget(BudgetError::Cancelled) => {
            "semantic simplify failed: cancelled: admission cancelled".into()
        }
        AdmissionError::Budget(budget) => {
            format!("semantic simplify failed: capacity_exhausted: {budget}")
        }
    }
}

fn receipt_status(report: &AdmissionReport, target_faces: usize) -> Result<(), String> {
    match report.termination {
        Termination::ReachedTarget if report.faces <= target_faces && report.vertices > 0 && report.faces > 0 => {
            Ok(())
        }
        Termination::WorkExhausted | Termination::EventCapacityExhausted => Err(format!(
            "semantic simplify failed: capacity_exhausted: {:?}",
            report.termination
        )),
        Termination::Cancelled => Err("semantic simplify failed: cancelled: admission cancelled".into()),
        Termination::TransferFailure(code) => Err(format!(
            "semantic simplify failed: {}: transfer {:?}",
            reason_status(code),
            report.termination
        )),
        _ => Err(format!(
            "semantic simplify failed: unsatisfied_constraint: {:?}",
            report.termination
        )),
    }
}

pub(super) fn decimate_semantic_admitted(
    vertices: &[Point3],
    triangles: &[[u32; 3]],
    target_faces: usize,
    max_error: Option<f64>,
    domain_tags: &[u64],
    locked: &[bool],
    byte_limit: usize,
    work_limit: u64,
    cancelled: bool,
) -> Result<(Vec<Point3>, Vec<[u32; 3]>), String> {
    let n = vertices.len();
    let m = triangles.len();
    validate_semantic_input(n, m, target_faces)?;
    if domain_tags.len() != n {
        return Err(format!(
            "domains length {} does not match vertex count {n}",
            domain_tags.len()
        ));
    }
    if locked.len() != n {
        return Err(format!("locks length {} does not match vertex count {n}", locked.len()));
    }
    if cancelled {
        return Err("semantic simplify failed: cancelled: production cancelled".into());
    }
    let scratch_bytes = estimated_scratch_bytes(n, m)?;
    if scratch_bytes > byte_limit {
        return Err(format!(
            "semantic simplify failed: capacity_exhausted: scratch {scratch_bytes} exceeds byte budget {byte_limit}"
        ));
    }
    let required_work = n
        .checked_mul(n)
        .and_then(|x| x.checked_add(n))
        .and_then(|x| x.checked_add(m.saturating_mul(4)))
        .and_then(|x| u64::try_from(x).ok())
        .ok_or("semantic simplify failed: capacity_exhausted: work overflow")?;
    if required_work > work_limit {
        return Err(format!(
            "semantic simplify failed: capacity_exhausted: work {required_work} exceeds remaining {work_limit}"
        ));
    }

    let mut attributes = vec![SemanticVertexAttributes::default(); n];
    let mut semantic_domains = vec![SemanticDomains::default(); n];
    let mut protected = vec![ProtectedDomain::default(); n];
    let mut stable_ids = vec![0_u64; n];
    for i in 0..n {
        let tag = domain_tags[i];
        let subpart = u32::try_from(tag).unwrap_or(u32::MAX);
        let lock_flag = u64::from(locked[i]);
        attributes[i] = SemanticVertexAttributes {
            subpart_tag: subpart,
            cluster_border_locked: locked[i],
            ..SemanticVertexAttributes::default()
        };
        semantic_domains[i] = SemanticDomains {
            subpart: tag,
            protected_region: lock_flag,
            ..SemanticDomains::default()
        };
        protected[i] = ProtectedDomain {
            subpart: tag,
            protected: lock_flag,
            ..ProtectedDomain::default()
        };
        stable_ids[i] = i as u64 + 1;
    }

    let identity = AdmissionIdentity {
        run: ContentId::sha256(b"hypermedia.manifold.simplify/semantic/run"),
        source: ContentId::sha256(b"hypermedia.manifold.simplify/semantic/source"),
        policy: ContentId::sha256(b"hypermedia.manifold.simplify/semantic/policy"),
    };
    let cancel = Cancellation::new();
    let mut construct = ProductionBudget::admit(CapacityPlan::default(), byte_limit, work_limit, &cancel)
        .map_err(|e| format!("semantic simplify failed: capacity_exhausted: {e}"))?;
    let row_capacity = n.max(1);
    let mut rows = vec![TransferRow::default(); n];
    let mut entries = vec![SparseEntry { row: 0, col: 0, value: 0.0 }; transfer_entry_count(n)?];
    let mut transfer = CollapseTransfer::identity(
        identity.run,
        identity.source,
        identity.policy,
        vertices,
        &protected,
        &stable_ids,
        m,
        row_capacity,
        &mut rows,
        &mut entries,
        1e-9,
        &mut construct,
    )
    .map_err(transfer_status)?;

    let mut slots = vec![AdmissionSlot::empty(); n.max(m)];
    let mut face_scratch = vec![
        FaceDelta {
            original_face: 0,
            before: [0; 3],
            after: None,
        };
        m
    ];
    let mut corner_scratch = vec![
        CornerDelta {
            source: CornerId { face: 0, slot: 0 },
            target: None,
        };
        m.saturating_mul(3)
    ];
    let mut transfer_scratch = vec![0_u8; n.max(1).saturating_mul(16)];
    let mut out_vertices = vec![Point3::default(); n];
    let mut out_attributes = vec![SemanticVertexAttributes::default(); n];
    let mut out_domains = vec![SemanticDomains::default(); n];
    let mut out_triangles = vec![[0_u32; 3]; m];
    let mut lineage = vec![0_u32; n];
    let mut vertex_compact = vec![0_u32; n];
    let mut face_compact = vec![0_u32; m];
    let mut corner_compact = vec![None; m.saturating_mul(3)];
    let mut events = vec![None::<CollapseEvent>; n.saturating_sub(1)];

    let report = decimate_semantic_admitted_3(
        AdmissionInput {
            vertices,
            attributes: &attributes,
            triangles,
            domains: &semantic_domains,
            stable_vertex_ids: &stable_ids,
        },
        AdmissionPolicy {
            topology: TopologyMode::StrictManifold,
            domain_layout: DomainLayout::VertexAlignedSplitCorners,
            target_faces,
            preserve_boundary_vertices: true,
            cost: NormalizedCostProfile::default(),
            maximum_normalized_cost: max_error,
        },
        identity,
        AdmissionLimits {
            bytes: byte_limit,
            work: work_limit,
        },
        &cancel,
        AdmissionScratch {
            slots: &mut slots,
            faces: &mut face_scratch,
            corners: &mut corner_scratch,
            transfer: &mut transfer_scratch,
        },
        AdmissionOutput {
            vertices: &mut out_vertices,
            attributes: &mut out_attributes,
            domains: &mut out_domains,
            triangles: &mut out_triangles,
            remaps: RemapBuffers {
                vertex_lineage: &mut lineage,
                surviving_vertex_compaction: &mut vertex_compact,
                face_compaction: &mut face_compact,
                corner_compaction: &mut corner_compact,
            },
            events: &mut events,
        },
        &mut transfer,
    )
    .map_err(admission_status)?;
    receipt_status(&report, target_faces)?;
    out_vertices.truncate(report.vertices);
    out_triangles.truncate(report.faces);
    if out_vertices.is_empty() || out_triangles.is_empty() {
        return Err("semantic simplify failed: invalid_input: empty simplification output".into());
    }
    Ok((out_vertices, out_triangles))
}

#[cfg(test)]
mod tests {
    use super::*;
    use vibe::Value;

    fn args(pairs: &[(&str, Value)]) -> Args {
        pairs.iter().cloned().map(|(k, v)| (k.to_string(), v)).collect()
    }

    #[test]
    fn semantic_request_requires_target_and_optional_max_error() {
        let missing = parse_semantic_request(&args(&[("mode", Value::String("semantic".into()))]));
        assert!(missing.unwrap_err().contains("Missing target_triangles"));

        let range = parse_semantic_request(&args(&[
            ("mode", Value::String("semantic".into())),
            ("target_triangles", Value::U64(3)),
        ]));
        assert!(range.unwrap_err().contains("4..=200000"));

        let parsed = parse_semantic_request(&args(&[
            ("mode", Value::String("admitted".into())),
            ("target_triangles", Value::U64(24)),
        ]))
        .expect("optional max_error");
        assert_eq!(parsed.target_triangles, 24);
        assert_eq!(parsed.max_error, None);

        let with_error = parse_semantic_request(&args(&[
            ("target_triangles", Value::U64(24)),
            ("max_error", Value::F64(0.05)),
        ]))
        .expect("max_error");
        assert_eq!(with_error.max_error, Some(0.05));

        let bad_error = parse_semantic_request(&args(&[
            ("target_triangles", Value::U64(24)),
            ("max_error", Value::F64(-1.0)),
        ]));
        assert!(bad_error.unwrap_err().contains("max_error"));
    }

    #[test]
    fn empty_mesh_and_impossible_target_are_invalid_input() {
        assert!(validate_semantic_input(0, 0, 24).unwrap_err().contains("empty mesh"));
        assert!(validate_semantic_input(8, 0, 4).unwrap_err().contains("empty mesh"));
        assert!(validate_semantic_input(8, 12, 13)
            .unwrap_err()
            .contains("impossible target_triangles"));
        validate_semantic_input(8, 12, 8).expect("reachable target");
    }

    #[test]
    fn domain_and_lock_lists_validate_length_and_indices() {
        let domains = Value::List(vec![Value::U64(101), Value::U64(202)]);
        assert!(parse_domain_list(&domains, 3).unwrap_err().contains("does not match"));
        assert_eq!(parse_domain_list(&domains, 2).unwrap(), vec![101, 202]);

        let flags = Value::List(vec![Value::Bool(true), Value::Bool(false), Value::Bool(true)]);
        assert_eq!(parse_lock_list(&flags, 3).unwrap(), vec![true, false, true]);
        let indices = Value::List(vec![Value::U64(0), Value::U64(2)]);
        assert_eq!(parse_lock_list(&indices, 3).unwrap(), vec![true, false, true]);
        assert!(parse_lock_list(&Value::List(vec![Value::U64(9)]), 3)
            .unwrap_err()
            .contains("outside vertex count"));
    }

    #[test]
    fn encoded_domain_and_lock_handles_roundtrip() {
        let mut domain_bytes = Vec::new();
        for tag in [101_u64, 202_u64] {
            domain_bytes.extend_from_slice(&tag.to_le_bytes());
        }
        assert_eq!(decode_domain_bytes(&domain_bytes, 2).unwrap(), vec![101, 202]);
        assert!(decode_domain_bytes(&[0; 3], 2).unwrap_err().contains("byte length"));
        assert_eq!(decode_lock_bytes(&[1, 0, 1], 3).unwrap(), vec![true, false, true]);
        let mut lock_u64 = Vec::new();
        for flag in [1_u64, 0_u64] {
            lock_u64.extend_from_slice(&flag.to_le_bytes());
        }
        assert_eq!(decode_lock_bytes(&lock_u64, 2).unwrap(), vec![true, false]);
    }

    #[test]
    fn admitted_driver_rejects_empty_and_impossible_targets_without_success_mesh() {
        let vertices = [Point3::new(0.0, 0.0, 0.0)];
        let err = decimate_semantic_admitted(
            &vertices,
            &[],
            4,
            None,
            &[0],
            &[false],
            16 * 1024 * 1024,
            1_000_000,
            false,
        )
        .unwrap_err();
        assert!(err.contains("empty mesh"), "{err}");

        let cube_v = [
            Point3::new(-1.0, -1.0, -1.0),
            Point3::new(1.0, -1.0, -1.0),
            Point3::new(1.0, 1.0, -1.0),
            Point3::new(-1.0, 1.0, -1.0),
            Point3::new(-1.0, -1.0, 1.0),
            Point3::new(1.0, -1.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(-1.0, 1.0, 1.0),
        ];
        let cube_t = [
            [0, 3, 2],
            [0, 2, 1],
            [4, 5, 6],
            [4, 6, 7],
            [0, 1, 5],
            [0, 5, 4],
            [3, 7, 6],
            [3, 6, 2],
            [0, 4, 7],
            [0, 7, 3],
            [1, 2, 6],
            [1, 6, 5],
        ];
        let domains = [0_u64; 8];
        let locks = [false; 8];
        let impossible = decimate_semantic_admitted(
            &cube_v,
            &cube_t,
            13,
            None,
            &domains,
            &locks,
            16 * 1024 * 1024,
            1_000_000,
            false,
        )
        .unwrap_err();
        assert!(impossible.contains("impossible target_triangles"), "{impossible}");

        let exhausted = decimate_semantic_admitted(
            &cube_v,
            &cube_t,
            8,
            None,
            &domains,
            &locks,
            1,
            1_000_000,
            false,
        )
        .unwrap_err();
        assert!(exhausted.contains("capacity_exhausted"), "{exhausted}");

        let reduced = decimate_semantic_admitted(
            &cube_v,
            &cube_t,
            8,
            None,
            &domains,
            &locks,
            16 * 1024 * 1024,
            1_000_000,
            false,
        );
        let (out_v, out_t) = reduced.expect("reachable cube target");
        assert!(!out_v.is_empty());
        assert!(!out_t.is_empty());
        assert!(out_t.len() <= 8, "faces {}", out_t.len());
    }

    #[test]
    fn catalog_and_help_list_semantic_mode() {
        let help = crate::arguments("hypermedia.manifold.simplify");
        assert!(help.contains("semantic"), "{help}");
        assert!(help.contains("decimate"), "{help}");
        let catalog = crate::OPERATIONS
            .iter()
            .find(|op| op.0 == "hypermedia.manifold.simplify")
            .expect("simplify catalog row");
        assert!(catalog.2.contains("semantic"), "{}", catalog.2);
        let spec = crate::ontology::resolve_morphism("hypermedia.manifold.simplify").expect("spec");
        assert!(spec.summary.contains("semantic"), "{}", spec.summary);
        assert!(spec.arguments.contains("semantic"), "{}", spec.arguments);
    }

    #[test]
    fn semantic_and_admitted_are_the_admitted_mode() {
        assert!(is_semantic_mode("semantic"));
        assert!(is_semantic_mode("admitted"));
        assert!(!is_semantic_mode("decimate"));
        assert!(!is_semantic_mode("remesh"));
        assert!(!is_semantic_mode("anisotropic"));
    }
}
