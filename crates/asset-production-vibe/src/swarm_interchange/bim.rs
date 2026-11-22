//! IFC Part 21 spatial, boundary and service topology extraction.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    qualia_containers_governance_poet_ops::encoded_resource, *,
};
use manifold_bim_csg::{StepEntity, StepVal, parse_step};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

const BIM: &str = "urn:qualia:bim:";

impl AssetHost<'_> {
    pub(crate) fn compile_bim_ifc_mep(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mode", "source"])?;
        if text(required(a, "mode")?)? != "ifc_spatial_topology" {
            return Err(
                "Expected ifc_spatial_topology with a retained IFC4 or IFC4X3 source".into(),
            );
        }
        let handle = required(a, "source")?;
        let input = encoded_resource(self, handle)?;
        if input.kind != "ifc" || !(1..=512 * 1024).contains(&input.bytes.len()) {
            return Err("Expected a retained IFC STEP document within 512 KiB".into());
        }
        let size = input.bytes.len();
        self.charge(size * 100, size * 20)?;
        let input = encoded_resource(self, handle)?;
        let document = std::str::from_utf8(&input.bytes).map_err(|_| "IFC input must use UTF-8")?;
        let ids = canonical_record_ids(document)?;
        let parsed = parse_step(&input.bytes)?;
        if !matches!(
            parsed.schema.as_str(),
            "IFC4" | "IFC4X3" | "IFC4X3_ADD1" | "IFC4X3_ADD2"
        ) || parsed.entities.len() != ids.len()
            || parsed.entities.keys().any(|id| !ids.contains(id))
        {
            return Err(
                "IFC schema or admitted STEP entity directory does not match the source".into(),
            );
        }
        let digest = format!("{:x}", Sha256::digest(&input.bytes));
        let subject = format!("{BIM}sha256:{digest}");
        let mut graph = ReceiptGraph::new(&subject);
        graph.fact(
            &subject,
            &format!("{BIM}schema"),
            &format!("{BIM}{}", parsed.schema),
        )?;
        let mut storeys = Vec::new();
        let mut spaces = Vec::new();
        let mut containment = Vec::new();
        let mut boundaries = Vec::new();
        let mut service_links = Vec::new();
        let mut parents = BTreeMap::new();
        let node = |id| format!("{subject}:entity:{id}");
        let get = |id| {
            parsed
                .entities
                .get(&id)
                .ok_or_else(|| format!("Unresolved IFC entity #{id}"))
        };
        let mut seen = BTreeSet::new();
        for id in ids {
            let entity = get(id)?;
            match entity.name.as_str() {
                "IFCBUILDINGSTOREY" | "IFCSPACE" => {
                    graph.fact(&subject, &format!("{BIM}hasSpatialElement"), &node(id))?;
                    graph.fact(
                        &node(id),
                        &format!("{BIM}class"),
                        &format!("{BIM}{}", entity.name),
                    )?;
                    let row = spatial_row(id, entity)?;
                    if entity.name == "IFCSPACE" {
                        spaces.push(row);
                    } else {
                        storeys.push(row);
                    }
                }
                "IFCRELAGGREGATES" | "IFCRELCONTAINEDINSPATIALSTRUCTURE" | "IFCRELNESTS" => {
                    let reversed = entity.name == "IFCRELCONTAINEDINSPATIALSTRUCTURE";
                    let parent = reference(entity, if reversed { 5 } else { 4 })?;
                    get(parent)?;
                    let children = list_references(entity, if reversed { 4 } else { 5 })?;
                    for child in children {
                        get(child)?;
                        if child == parent {
                            return Err("IFC spatial or nested element refers to itself".into());
                        }
                        // IFC nesting and aggregation each express decomposition; spatial containment
                        // is independent of nesting (for example ports nested in contained pipes).
                        let relation = if reversed { "containedIn" } else { "partOf" };
                        if parents
                            .insert((child, relation), parent)
                            .is_some_and(|old| old != parent)
                        {
                            return Err(
                                "IFC element has conflicting parents on a relationship axis".into(),
                            );
                        }
                        if seen.insert((child, relation, parent)) {
                            graph.fact(&node(child), &format!("{BIM}{relation}"), &node(parent))?;
                            containment.push(edge(child, parent, relation));
                        }
                    }
                }
                "IFCRELSPACEBOUNDARY"
                | "IFCRELSPACEBOUNDARY1STLEVEL"
                | "IFCRELSPACEBOUNDARY2NDLEVEL" => {
                    let space = reference(entity, 4)?;
                    get(space)?;
                    if matches!(entity.args.get(5), Some(StepVal::Null)) {
                        continue;
                    }
                    let component = reference(entity, 5)?;
                    get(component)?;
                    if seen.insert((space, "boundedBy", component)) {
                        graph.fact(&node(space), &format!("{BIM}boundedBy"), &node(component))?;
                        boundaries.push(edge(space, component, "boundedBy"));
                    }
                }
                "IFCRELCONNECTSPORTS"
                | "IFCRELCONNECTSPORTTOELEMENT"
                | "IFCRELCONNECTSELEMENTS" => {
                    let slot = if entity.name == "IFCRELCONNECTSELEMENTS" {
                        5
                    } else {
                        4
                    };
                    let from = reference(entity, slot)?;
                    let to = reference(entity, slot + 1)?;
                    get(from)?;
                    get(to)?;
                    if from == to {
                        return Err("IFC service connection has identical ends".into());
                    }
                    if seen.insert((from, "connectedTo", to)) {
                        graph.fact(&node(from), &format!("{BIM}connectedTo"), &node(to))?;
                        service_links.push(edge(from, to, "connectedTo"));
                    }
                }
                _ => {}
            }
            if seen.len() > 8_192 {
                return Err("IFC graph exceeds 8,192 supported relations".into());
            }
        }
        validate_acyclic(&parents)?;
        if storeys.is_empty() && spaces.is_empty() && service_links.is_empty() {
            return Err("IFC source has no supported spatial or service topology".into());
        }
        let native = graph.seal(self.limits.resource_bytes)?;
        let unchanged = input.bytes.clone();
        let graph = self.encode("q42", native)?;
        let ifc = self.encode("ifc", unchanged)?;
        Ok(Value::Record(Args::from([
            ("ifc".into(), ifc),
            ("graph".into(), graph),
            ("source_sha256".into(), Value::String(digest)),
            ("schema".into(), Value::String(parsed.schema)),
            ("storeys".into(), Value::List(storeys)),
            ("spaces".into(), Value::List(spaces)),
            ("containment".into(), Value::List(containment)),
            ("boundaries".into(), Value::List(boundaries)),
            ("service_connections".into(), Value::List(service_links)),
        ])))
    }
}

fn canonical_record_ids(input: &str) -> Result<BTreeSet<u64>, String> {
    let lines = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    if !lines.starts_with(&["ISO-10303-21;", "HEADER;"])
        || !lines.ends_with(&["ENDSEC;", "END-ISO-10303-21;"])
        || lines.iter().filter(|line| **line == "ENDSEC;").count() != 2
    {
        return Err("Expected one canonical IFC Part 21 HEADER and DATA section".into());
    }
    let data = lines
        .iter()
        .position(|line| *line == "DATA;")
        .ok_or("IFC DATA section missing")?;
    if data < 3 || lines[data - 1] != "ENDSEC;" || data >= lines.len() - 2 {
        return Err("Invalid canonical IFC HEADER/DATA boundary".into());
    }
    let mut ids = BTreeSet::new();
    for line in &lines[data + 1..lines.len() - 2] {
        let (lhs, rhs) = line
            .strip_prefix('#')
            .and_then(|s| s.split_once('='))
            .ok_or("IFC subset requires one entity per line")?;
        let id = lhs
            .trim()
            .parse::<u64>()
            .map_err(|_| "Invalid IFC entity id")?;
        if id == 0
            || !ids.insert(id)
            || !canonical_statement(rhs)
            || !rhs.trim_start().starts_with("IFC")
            || ids.len() > 8_192
        {
            return Err(
                "Invalid IFC entity statement, duplicate id or exceeded directory limit".into(),
            );
        }
    }
    Ok(ids)
}

fn canonical_statement(rhs: &str) -> bool {
    let Some(body) = rhs.strip_suffix(';') else {
        return false;
    };
    let mut quoted = false;
    for byte in body.bytes() {
        // Two adjacent apostrophes inside a STEP string toggle twice, leaving the
        // string state unchanged. Only separators outside strings are structural.
        match byte {
            b'\'' => quoted = !quoted,
            b';' | b'/' if !quoted => return false,
            _ => {}
        }
    }
    !quoted
}

fn reference(entity: &StepEntity, index: usize) -> Result<u64, String> {
    match entity.args.get(index) {
        Some(StepVal::Ref(value)) => Ok(*value),
        _ => Err(format!(
            "{} needs a reference at parameter {}",
            entity.name,
            index + 1
        )),
    }
}

fn list_references(entity: &StepEntity, index: usize) -> Result<Vec<u64>, String> {
    match entity.args.get(index) {
        Some(StepVal::List(children)) if !children.is_empty() && children.len() <= 4_096 => {
            children
                .iter()
                .map(|child| {
                    if let StepVal::Ref(value) = child {
                        Ok(*value)
                    } else {
                        Err("IFC child must be an entity reference".into())
                    }
                })
                .collect()
        }
        _ => Err("Invalid or overbudget IFC relation endpoint list".into()),
    }
}

fn spatial_row(id: u64, entity: &StepEntity) -> Result<Value, String> {
    let Some(StepVal::Str(guid)) = entity.args.first() else {
        return Err("IFC spatial element lacks a GlobalId".into());
    };
    if guid.len() != 22
        || !guid
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'$')
    {
        return Err(
            "IFC spatial GlobalId is outside the admitted compressed identifier syntax".into(),
        );
    }
    let name = match entity.args.get(2) {
        Some(StepVal::Str(s)) => Value::String(s.clone()),
        _ => Value::Null,
    };
    Ok(Value::Record(Args::from([
        ("id".into(), Value::U64(id)),
        ("global_id".into(), Value::String(guid.clone())),
        ("name".into(), name),
    ])))
}

fn edge(from: u64, to: u64, relation: &str) -> Value {
    Value::Record(Args::from([
        ("from".into(), Value::U64(from)),
        ("to".into(), Value::U64(to)),
        ("relation".into(), Value::String(relation.into())),
    ]))
}

fn validate_acyclic(parents: &BTreeMap<(u64, &str), u64>) -> Result<(), String> {
    for &(start, relation) in parents.keys() {
        let mut id = start;
        for _ in 0..=64 {
            let Some(parent) = parents.get(&(id, relation)) else {
                break;
            };
            id = *parent;
            if id == start {
                return Err("Cyclic IFC decomposition or containment".into());
            }
        }
        if parents.contains_key(&(id, relation)) {
            return Err("IFC hierarchy is cyclic or deeper than 64 levels".into());
        }
    }
    Ok(())
}
