//! Authored 2D pattern to 3D drape seam correspondence and GTOP sealing.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

#[cfg(all(feature = "gpu-vbd-contact", not(target_arch = "wasm32")))]
mod gpu;

use std::collections::{BTreeMap, BTreeSet};

use qualia_core_db::{
    bundle::semantic_model::{resource_identity_meta, SemanticModelBuilder},
    container_10d::{decode_symbolic_rdf_cbor_ld, encode_symbolic_rdf_cbor_ld, MultilingualLexicalTerm, SymbolicRdfCborLdPayload},
    q_hash, NQuin,
    specialized_libs::computational_geometry::{build_triangle_half_edges, required_edge_slots, EdgeSlot, HalfEdge, INVALID_INDEX},
};

use crate::{boolean, fields, integer, list, num, record, required, text, Args, AssetHost, TypedArtifact, Value};

const CONTEXT: &str = "urn:qualia:garment:seam-correspondence:v1";
const MAX_CELLS: usize = 16_384;
const MAX_SEAMS: usize = 128;
const MAX_STITCHES: usize = 1024;
const SYMBOLIC_LIMIT: usize = 42 * 1024 * 1024;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Value, String> {
    match op {
        "extract_seam_links" => extract_seam_links(host, args),
        #[cfg(all(feature = "gpu-vbd-contact", not(target_arch = "wasm32")))]
        "spatial_filter" => gpu::spatial_filter(host, args),
        #[cfg(not(all(feature = "gpu-vbd-contact", not(target_arch = "wasm32"))))]
        "spatial_filter" => Err("CapabilityUnavailable: native gpu-vbd-contact feature is disabled".into()),
        _ => Err(format!("Unsupported garment contact operation: {op}")),
    }
}

fn extract_seam_links(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["pattern", "drape", "geometry_iri", "seams", "max_closure_gap_m"])?;
    let pattern_input = required(args, "pattern")?;
    let drape_input = required(args, "drape")?;
    let geometry = text(required(args, "geometry_iri")?)?;
    valid_iri(geometry)?;
    let seams = list(required(args, "seams")?)?;
    if !(1..=MAX_SEAMS).contains(&seams.len()) { return Err("Garment seam count exceeds its bound".into()); }
    let max_gap = args.get("max_closure_gap_m").map(num).transpose()?;
    if max_gap.is_some_and(|m| m <= 0.0) { return Err("Maximum closure gap must be positive".into()); }
    let (vertices, triangles) = match host.workspace.typed(drape_input)? {
        TypedArtifact::AttributedMesh(mesh) => (mesh.surface.mesh.positions.len(), mesh.surface.mesh.triangles.len()),
        _ => return Err("Drape requires an AttributedMesh handle".into()),
    };
    if vertices < 4 || triangles == 0 || vertices.saturating_add(triangles) > MAX_CELLS {
        return Err("Garment topology exceeds its cell bound".into());
    }
    let pair_count = seams.iter().try_fold(0usize, |n, seam| -> Result<usize, String> {
        let seam = record(seam)?;
        let a = list(required(seam, "side_a")?)?.len();
        let b = list(required(seam, "side_b")?)?.len();
        if a != b || a < 2 { return Err("Each seam requires matching ordered sides of at least two vertices".into()); }
        n.checked_add(a).filter(|count| *count <= MAX_STITCHES).ok_or_else(|| "Garment stitch bound exceeded".into())
    })?;
    host.charge(triangles * 24 + vertices * 8 + pair_count * 64, triangles * 128 + vertices * 128 + pair_count * 5120)?;

    let TypedArtifact::AttributedMesh(pattern) = host.workspace.typed(pattern_input)?
        else { return Err("Pattern requires an AttributedMesh handle".into()); };
    let TypedArtifact::AttributedMesh(drape) = host.workspace.typed(drape_input)?
        else { return Err("Drape requires an AttributedMesh handle".into()); };
    let pattern = &pattern.surface.mesh;
    let drape = &drape.surface.mesh;
    if pattern.positions.len() != vertices || pattern.triangles != drape.triangles {
        return Err("Pattern and drape must retain the same indexed oriented triangulation".into());
    }
    if pattern.positions.iter().any(|p| p[2] != 0.0)
        || pattern.positions.iter().chain(&drape.positions).flatten().any(|v| !v.is_finite())
    {
        return Err("Pattern must lie in its authored XY plane and all geometry must be finite".into());
    }
    let boundary = boundary_edges(vertices, &pattern.triangles)?;
    validate_area(&pattern.positions, &pattern.triangles)?;
    validate_area(&drape.positions, &drape.triangles)?;
    let pattern_identity = mesh_iri(&pattern.positions, &pattern.triangles);
    let drape_identity = mesh_iri(&drape.positions, &drape.triangles);
    let id = q_hash(geometry);
    let overlay_iri = format!("urn:qualia:gtop:{id:016x}:seam-links");
    let mut facts = vec![
        relation(geometry, "authoredPattern", &pattern_identity),
        relation(geometry, "authoredDrape", &drape_identity),
        relation(geometry, "seamOverlay", &overlay_iri),
    ];
    let mut seam_identities = BTreeSet::new();
    let mut overlay_slots = BTreeMap::new();
    let mut topology_edges = BTreeSet::new();
    let mut anchors = Vec::new();
    let mut reports = Vec::with_capacity(seams.len());
    for raw in seams {
        let seam = record(raw)?;
        fields(seam, &["seam_iri", "side_a", "side_b", "closed"])?;
        let seam_iri = text(required(seam, "seam_iri")?)?;
        valid_iri(seam_iri)?;
        if !seam_identities.insert(seam_iri) { return Err("Repeated garment seam identity".into()); }
        let closed = boolean(seam, "closed", false)?;
        let a = ordered_vertices(required(seam, "side_a")?, vertices)?;
        let b = ordered_vertices(required(seam, "side_b")?, vertices)?;
        if a.iter().any(|vertex| b.contains(vertex)) {
            return Err(format!("Garment seam {seam_iri} reuses a vertex on both sides"));
        }
        let a_edges = oriented_boundary(&a, closed, &boundary)?;
        let b_edges = oriented_boundary(&b, closed, &boundary)?;
        if a_edges == b_edges { return Err(format!("Garment seam {seam_iri} requires opposing boundary orientations")); }
        facts.push(relation(geometry, "seam", seam_iri));
        let mut max_seam_gap = 0.0_f64;
        for (slot, (&left, &right)) in a.iter().zip(&b).enumerate() {
            let gap = distance(drape.positions[left], drape.positions[right]);
            if max_gap.is_some_and(|limit| gap > limit) {
                return Err(format!("Garment seam {seam_iri} stitch {slot} closure residual {gap:e} m exceeds its supplied limit"));
            }
            max_seam_gap = max_seam_gap.max(gap);
            let stitch = format!("urn:qualia:garment:{id:016x}:seam:{:016x}:stitch:{slot}", q_hash(seam_iri));
            facts.push(relation(seam_iri, "stitch", &stitch));
            let lhs = anchor(id, left, &pattern_identity, &mut anchors, &mut overlay_slots, &mut facts);
            let rhs = anchor(id, right, &pattern_identity, &mut anchors, &mut overlay_slots, &mut facts);
            facts.push(relation(&stitch, "sideA", &vertex_iri(id, left)));
            facts.push(relation(&stitch, "sideB", &vertex_iri(id, right)));
            add_edge(&mut topology_edges, lhs, rhs);
        }
        for path in [&a, &b] {
            let spans = path.len() - 1 + closed as usize;
            for i in 0..spans {
                add_edge(&mut topology_edges, overlay_slots[&path[i]], overlay_slots[&path[(i + 1) % path.len()]]);
            }
        }
        reports.push(Value::Record(Args::from([
            ("seam_iri".into(), Value::String(seam_iri.into())),
            ("stitch_count".into(), Value::U64(a.len() as u64)),
            ("max_closure_gap_m".into(), Value::F64(max_seam_gap)),
        ])));
    }
    let edges = topology_edges.into_iter().map(|(source, target)| Value::Record(Args::from([
        ("source".into(), Value::U64(source)), ("target".into(), Value::U64(target)),
    ]))).collect();
    let overlay = Value::Record(Args::from([
        ("graph_iri".into(), Value::String(overlay_iri)),
        ("role".into(), Value::String("SemanticOverlay".into())),
        ("morphology".into(), Value::String("SymmetricGraph".into())),
        ("anchors".into(), Value::List(anchors)),
        ("edges".into(), Value::List(edges)),
    ]));
    let topology = host.harbour_gtop_geometry_graphs(&Args::from([
        ("source".into(), drape_input.clone()),
        ("geometry_iri".into(), Value::String(geometry.into())),
        ("metric_regime".into(), Value::String("Euclidean".into())),
        ("graphs".into(), Value::List(vec![overlay])),
    ]))?;
    let artifacts = record(&topology)?;
    let gtop = host.workspace.encoded(required(artifacts, "gtop")?)?;
    let gtop_iri = format!("urn:qualia:gtop:blake3:{}", blake3::hash(&gtop.bytes).to_hex());
    facts.push(relation(geometry, "sealedTopology", &gtop_iri));
    let dictionary = host.workspace.encoded(required(artifacts, "cbld")?)?;
    let data = decode_symbolic_rdf_cbor_ld(&dictionary.bytes).map_err(|error| error.to_string())?;
    let receipt = seal_receipt(data, facts, host.limits.resource_bytes.min(SYMBOLIC_LIMIT))?;
    Ok(Value::Record(Args::from([
        ("topology".into(), topology),
        ("receipt".into(), host.encode("q42", receipt)?),
        ("seams".into(), Value::List(reports)),
    ])))
}

fn valid_iri(iri: &str) -> Result<(), String> {
    if iri.len() > 4096 { return Err("Garment semantic IRI exceeds its bound".into()); }
    resource_identity_meta(iri).map(|_| ()).map_err(|error| error.to_string())
}

fn boundary_edges(vertices: usize, triangles: &[[u32; 3]]) -> Result<BTreeSet<(usize, usize)>, String> {
    let mut edges = vec![HalfEdge::default(); triangles.len() * 3];
    let mut slots = vec![EdgeSlot::default(); required_edge_slots(triangles.len())];
    build_triangle_half_edges(vertices as u32, triangles, &mut edges, &mut slots)
        .map_err(|error| format!("Native garment adjacency: {error:?}"))?;
    Ok(edges.iter().filter(|edge| edge.twin == INVALID_INDEX).map(|edge| {
        (edge.origin as usize, edges[edge.next as usize].origin as usize)
    }).collect())
}

fn validate_area(positions: &[[f32; 3]], triangles: &[[u32; 3]]) -> Result<(), String> {
    for &face in triangles {
        let [a, b, c] = face.map(|i| positions[i as usize].map(f64::from));
        let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        let normal = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
        if normal.iter().all(|x| *x == 0.0) { return Err("Degenerate garment source face".into()); }
    }
    Ok(())
}

fn ordered_vertices(raw: &Value, vertices: usize) -> Result<Vec<usize>, String> {
    let path = list(raw)?.iter().map(|v| integer(v, "seam vertex")).collect::<Result<Vec<_>, _>>()?;
    let unique = path.iter().copied().collect::<BTreeSet<_>>();
    if unique.len() != path.len() || unique.last().is_some_and(|&idx| idx >= vertices) {
        return Err("Repeated or out-of-range vertex in a garment seam side".into());
    }
    Ok(path)
}

fn oriented_boundary(path: &[usize], closed: bool, boundary: &BTreeSet<(usize, usize)>) -> Result<bool, String> {
    if path.len() < 2 || (closed && path.len() < 3) { return Err("Invalid seam path length".into()); }
    let forward = boundary.contains(&(path[0], path[1]));
    let span_count = path.len() - 1 + closed as usize;
    for i in 0..span_count {
        let (a, b) = (path[i], path[(i + 1) % path.len()]);
        let directed = if forward { (a, b) } else { (b, a) };
        if !boundary.contains(&directed) { return Err("Garment seam side is not a consistently oriented boundary chain".into()); }
    }
    Ok(forward)
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| (f64::from(a) - f64::from(b)).powi(2)).sum::<f64>().sqrt()
}

fn mesh_iri(positions: &[[f32; 3]], triangles: &[[u32; 3]]) -> String {
    let mut digest = blake3::Hasher::new_derive_key("Qualia garment geometry seam receipt v1");
    digest.update(&(positions.len() as u64).to_le_bytes());
    digest.update(&(triangles.len() as u64).to_le_bytes());
    for position in positions { for lane in position { digest.update(&lane.to_le_bytes()); } }
    for triangle in triangles { for index in triangle { digest.update(&index.to_le_bytes()); } }
    format!("urn:qualia:garment:mesh:blake3:{}", digest.finalize().to_hex())
}

fn vertex_iri(geometry: u64, index: usize) -> String { format!("urn:qualia:gtop:{geometry:016x}:cell:0:{index}") }

fn anchor(
    geometry: u64, index: usize, pattern_identity: &str, anchors: &mut Vec<Value>,
    slots: &mut BTreeMap<usize, u64>, facts: &mut Vec<[String; 3]>,
) -> u64 {
    *slots.entry(index).or_insert_with(|| {
        let iri = vertex_iri(geometry, index);
        let pattern_vertex = format!("{pattern_identity}:vertex:{index}");
        facts.push(relation(&iri, "patternVertex", &pattern_vertex));
        facts.push(relation(&pattern_vertex, "patternSource", pattern_identity));
        let slot = anchors.len() as u64;
        anchors.push(Value::Record(Args::from([
            ("domain".into(), Value::String("Vertex".into())),
            ("cell_index".into(), Value::U64(index as u64)),
            ("entity_iri".into(), Value::String(iri)),
        ])));
        slot
    })
}

fn add_edge(edges: &mut BTreeSet<(u64, u64)>, a: u64, b: u64) {
    edges.insert((a, b));
    edges.insert((b, a));
}

fn relation(subject: &str, predicate: &str, object: &str) -> [String; 3] {
    [subject.into(), format!("urn:qualia:garment:seam:{predicate}"), object.into()]
}

fn seal_receipt(mut data: SymbolicRdfCborLdPayload, facts: Vec<[String; 3]>, limit: usize) -> Result<Vec<u8>, String> {
    let mut lexicon = BTreeMap::new();
    for term in data.lexicon.drain(..) {
        if q_hash(&term.iri_utf8) != term.term_hash || lexicon.insert(term.term_hash, term).is_some() {
            return Err("Garment topology dictionary contains ambiguous identities".into());
        }
    }
    let context = q_hash(CONTEXT);
    for [subject, predicate, object] in facts {
        let [s, p, o] = [&subject, &predicate, &object].map(|iri| q_hash(iri));
        for (hash, iri) in [(s, subject), (p, predicate), (o, object), (context, CONTEXT.into())] {
            if let Some(term) = lexicon.get(&hash) {
                if term.iri_utf8 != iri { return Err("Garment receipt q_hash collision".into()); }
            } else {
                lexicon.insert(hash, MultilingualLexicalTerm { term_hash: hash, iri_utf8: iri, lang_bcp47: None, label_utf8: String::new() });
            }
        }
        data.nquins.push([s, p, o, context, 0, NQuin::calculate_parity(s, p, o, context, 0)]);
    }
    data.lexicon = lexicon.into_values().collect();
    let cbld = encode_symbolic_rdf_cbor_ld(&data).map_err(|error| error.to_string())?;
    SemanticModelBuilder::new(limit).merge_cbld(&cbld).and_then(SemanticModelBuilder::finish)
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{artifact::AttributedMesh, ToolLimits};
    use qualia_core_db::{
        q42_volume::{payload_catalogue::decode_gtop_section, WasmSliceQ42Reader},
        render::assets::{ImportedMesh, Mesh},
        topology::PoetGeometryView,
    };
    use std::sync::{atomic::AtomicBool, Arc};

    fn admit(host: &mut AssetHost<'_>, positions: Vec<[f32; 3]>) -> Value {
        host.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(AttributedMesh {
            surface: ImportedMesh {
                mesh: Mesh { positions, triangles: vec![[0, 1, 2], [3, 4, 5]], min: [0.0, -1.0, 0.0], max: [3.0, 1.0, 0.01] },
                normals: None, tangents: None, texture_coordinates_0: None, materials: vec![],
                texture_dependencies: vec![], submeshes: vec![],
            },
            uv1: vec![], vertex_colors: vec![], skin_weights: vec![], morph_targets: Arc::from([]), protected_faces: vec![],
        }))).unwrap()
    }

    #[test]
    fn seals_authored_stitches_on_drape_boundary_and_binds_receipt_to_gtop() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = AssetHost { workspace: Default::default(), inputs: &[], limits: &limits, cancel: &cancel, work: 0 };
        let mut positions = vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0],
            [2.0, 0.0, 0.0], [3.0, 0.0, 0.0], [2.0, 1.0, 0.0]];
        let pattern = admit(&mut host, positions.clone());
        positions[3..].copy_from_slice(&[[1.0, 0.0, 0.001], [0.0, 0.0, 0.001], [0.0, -1.0, 0.001]]);
        let drape = admit(&mut host, positions);
        let mut args = Args::from([
            ("pattern".into(), pattern), ("drape".into(), drape),
            ("geometry_iri".into(), Value::String("urn:test:garment".into())),
            ("seams".into(), Value::List(vec![Value::Record(Args::from([
                ("seam_iri".into(), Value::String("urn:test:garment:side".into())),
                ("side_a".into(), Value::List(vec![Value::U64(0), Value::U64(1)])),
                ("side_b".into(), Value::List(vec![Value::U64(4), Value::U64(3)])),
            ]))])),
        ]);
        let output = extract_seam_links(&mut host, &args).unwrap();
        let result = record(&output).unwrap();
        let artifacts = record(&result["topology"]).unwrap();
        let gtop = &host.workspace.encoded(&artifacts["gtop"]).unwrap().bytes;
        let segment = decode_gtop_section(gtop).unwrap();
        let mut storage = vec![0; segment.len() + 63];
        let start = storage.as_ptr().align_offset(64);
        storage[start..start + segment.len()].copy_from_slice(segment);
        let view = PoetGeometryView::from_segment_bytes(&storage[start..start + segment.len()]).unwrap();
        let overlay = view.graph_at_slot(3).unwrap();
        assert_eq!(overlay.vertex_count(), 4);
        assert_eq!(overlay.cell_anchor(1).unwrap().cell_index, 4);
        let receipt = &host.workspace.encoded(&result["receipt"]).unwrap().bytes;
        let model = WasmSliceQ42Reader::open(receipt).unwrap();
        let iri = format!("urn:qualia:gtop:blake3:{}", blake3::hash(gtop).to_hex());
        let mut page = vec![0; 1024 * 1024];
        let mut value = [0; 4096];
        let length = model.lookup_lexicon_hash_into(q_hash(&iri), &mut page, &mut value).unwrap().unwrap();
        assert_eq!(&value[..length], iri.as_bytes());
        args.insert("max_closure_gap_m".into(), Value::F64(0.0005));
        assert!(extract_seam_links(&mut host, &args).unwrap_err().contains("closure residual"));
    }
}
