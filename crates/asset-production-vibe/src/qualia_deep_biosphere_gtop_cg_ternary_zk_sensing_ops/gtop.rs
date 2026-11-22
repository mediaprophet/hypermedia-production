//! Bounded authoring of GTOP host complexes and multiple anchored CSR overlays.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use qualia_core_db::{NQuin, q_hash};
use qualia_core_db::bundle::semantic_model::resource_identity_meta;
use qualia_core_db::container_10d::GRAPHICS_ASSET_RING_BYTES;
use qualia_core_db::q42_volume::payload_catalogue::encode_gtop_section;
use qualia_core_db::specialized_libs::computational_geometry::{
    EdgeSlot, HalfEdge, INVALID_INDEX, build_triangle_half_edges, required_edge_slots,
};
use qualia_core_db::topology::{
    AnchorDomain, CellAnchor, GeometryComplexKind, GeometryMetricKind, GraphEmbedRole, GraphMorphology,
    PoetGeometryView, TopologicalDeltaRing, TopologyInvariants, UnsealedGeometrySpec, UnsealedGraphSpec,
    GtopRestrictionCsrSpec, GtopSheafSpec, append_gtop_sheaf_metadata, compile_geometry_segment,
    induce_primal_and_dual_from_half_edges, induce_simplicial_boundary_incidence,
    required_geometry_segment_bytes, required_gtop_sheaf_segment_bytes, seal_harboured_graph_deltas,
    verify_geometry_complex_homology, verify_graph_invariants,
};

use crate::{Args, AssetHost, TypedArtifact, Value, fields, integer, list, num, record, required, text};
use crate::artifact::FieldLattice;

const MAX_BASE_CELLS: usize = 65_536;
const MAX_TOTAL_OVERLAY_ROWS: usize = 65_536;
const MAX_GRAPHS: usize = 64;
const SCHEMA: &str = "urn:qualia:gtop:schema:v1";
const HAS_STALK_SCHEMA: &str = "urn:qualia:gtop:hasStalkSchema";

struct RestrictionMatrix {
    rows: u32,
    columns: u32,
    row_offsets: Vec<u32>,
    column_indices: Vec<u32>,
    coefficients: Vec<f64>,
}

impl RestrictionMatrix {
    fn as_spec(&self) -> GtopRestrictionCsrSpec<'_> {
        GtopRestrictionCsrSpec {
            rows: self.rows, columns: self.columns, row_offsets: &self.row_offsets,
            column_indices: &self.column_indices, coefficients: &self.coefficients,
        }
    }
}

impl AssetHost<'_> {
    pub(crate) fn harbour_gtop_geometry_graphs(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &[
            "source", "geometry_iri", "metric_regime", "graphs", "mutations", "evidence", "active_graph_slot", "hops", "sheaf",
        ])?;
        let geometry_iri = text(required(args, "geometry_iri")?)?;
        let geometry_id = iri(geometry_iri)?;
        let metric = match text(required(args, "metric_regime")?)? {
            "Euclidean" => GeometryMetricKind::Euclidean,
            "Toroidal" => GeometryMetricKind::ToroidalCyclic,
            "HyperbolicPoincare" => GeometryMetricKind::HyperbolicPoincare,
            "Spherical" => GeometryMetricKind::SphericalGeodesic,
            "FisherRaoSimplex" => GeometryMetricKind::FisherRaoSimplex,
            "LorentzianCausal" => GeometryMetricKind::LorentzianCausal,
            _ => return Err("Unknown GTOP metric regime".into()),
        };
        let budget = self.limits.resource_bytes.min(GRAPHICS_ASSET_RING_BYTES)
            .min(self.limits.total_resource_bytes);
        let input = required(args, "source")?;
        let source = self.workspace.typed(input)?;
        let count = list_size(args, "graphs")?.saturating_add(list_size(args, "mutations")?)
            .saturating_add(list_size(args, "hops")?).saturating_add(list_size(args, "evidence")?);
        if count > MAX_TOTAL_OVERLAY_ROWS {
            return Err("GTOP operation exceeds its graph row limit".into());
        }
        let (mut geometry, beta2, shape_estimate) = match source {
            TypedArtifact::AttributedMesh(mesh) => {
                let mesh = &mesh.surface.mesh;
                let estimate = estimate_surface(mesh.positions.len(), mesh.triangles.len(), budget)?;
                validate_coordinates(&mesh.positions, metric)?;
                let (spec, beta2) = surface(geometry_id, metric, mesh.positions.len(), &mesh.triangles, budget)?;
                (spec, beta2, estimate)
            }
            TypedArtifact::FieldLattice(field) => match field.as_ref() {
                FieldLattice::DenseF32 { axes, samples } => {
                    let (spec, estimated) = lattice(geometry_id, metric, axes, samples, budget)?;
                    (spec, 0, estimated)
                }
                _ => return Err("GTOP FieldLattice input requires dense 1D, 2D or 3D samples".into()),
            },
            _ => return Err("GTOP source must be an attributed mesh or a dense field lattice".into()),
        };
        let mut lexicon = BTreeMap::new();
        register(&mut lexicon, geometry_iri)?;
        register(&mut lexicon, SCHEMA)?;
        for graph in &mut geometry.graphs {
            let graph_iri = format!("urn:qualia:gtop:{geometry_id:016x}:graph:{}", graph.embed_role as u8);
            graph.graph_id = register(&mut lexicon, &graph_iri)?;
            for anchor in &mut graph.anchors {
                anchor.entity_hash = register(&mut lexicon, &cell_iri(geometry_id, anchor.anchor_domain, anchor.cell_index))?;
            }
        }
        let mut evidence = read_evidence(args)?;
        let builtin = geometry.graphs.len();
        if let Some(raw) = args.get("graphs") {
            let extra = list(raw)?;
            if extra.len() > MAX_GRAPHS - builtin { return Err("GTOP graph slot limit exceeded".into()); }
            let total = extra.iter().try_fold(0usize, |total, value| {
                let graph = record(value)?;
                Ok::<_, String>(total.saturating_add(list_size(graph, "anchors")?).saturating_add(list_size(graph, "edges")?))
            })?;
            if total > MAX_TOTAL_OVERLAY_ROWS || shape_estimate.saturating_add(total.saturating_mul(320)) > budget {
                return Err("GTOP overlays exceed their compilation budget".into());
            }
            for value in extra {
                let graph = parse_graph(value, &geometry, &evidence, &mut lexicon)?;
                if geometry.graphs.iter().any(|prior| prior.graph_id == graph.graph_id) {
                    return Err("GTOP graph identity is duplicated".into());
                }
                geometry.graphs.push(graph);
            }
        }
        if let Some(value) = args.get("active_graph_slot") {
            geometry.active_graph_slot = u16::try_from(integer(value, "active_graph_slot")?)
                .map_err(|_| "Invalid active GTOP graph slot")?;
        }
        let mut mutations = TopologicalDeltaRing::<256>::new();
        if let Some(raw) = args.get("mutations") {
            let rows = list(raw)?;
            if rows.len() > 256 { return Err("GTOP delta ring capacity exceeded".into()); }
            for row in rows { mutation(record(row)?, &geometry, builtin, evidence.len(), &mut mutations)?; }
            for (slot, graph) in geometry.graphs.iter_mut().enumerate().skip(builtin) {
                if mutations.dirty_slot_mask() & (1u64 << slot) != 0 {
                    seal_harboured_graph_deltas(graph, mutations.entries(), slot as u16);
                }
            }
        }
        for graph in &mut geometry.graphs[builtin..] {
            for anchor in &mut graph.anchors {
                if anchor.anchor_domain != AnchorDomain::SemanticQuin as u8 && anchor.entity_hash == 0 {
                    anchor.entity_hash = register(&mut lexicon, &cell_iri(geometry_id, anchor.anchor_domain, anchor.cell_index))?;
                }
            }
        }
        let work = geometry.graphs.iter().try_fold(0usize, |work, graph| {
            validate_graph(graph, &evidence)?;
            let quadratic = if graph.invariants.contains(TopologyInvariants::ACYCLIC)
                || graph.morphology == GraphMorphology::DeterministicDAG { graph.anchors.len().saturating_pow(2) } else { 0 };
            Ok::<_, String>(work.saturating_add(quadratic).saturating_add(graph.edges.len().saturating_mul(8)))
        })?;
        let (schema_hash, restriction_matrix) = if let Some(value) = args.get("sheaf") {
            let sheaf = record(value)?;
            fields(sheaf, &["stalk_schema_iri", "restrictions"])?;
            let schema_hash = register(&mut lexicon, text(crate::required(sheaf, "stalk_schema_iri")?)?)?;
            let restrictions = sheaf.get("restrictions").map(|v| self.restriction_matrix(v, budget)).transpose()?;
            (Some(schema_hash), restrictions)
        } else {
            (None, None)
        };
        let sheaf = schema_hash.map(|stalk_schema_q_hash| GtopSheafSpec {
            stalk_schema_q_hash,
            restrictions: restriction_matrix.as_ref().map(RestrictionMatrix::as_spec),
        });
        let base_bytes = required_geometry_segment_bytes(&geometry).map_err(|error| format!("GTOP layout: {error:?}"))?;
        let required = sheaf.map_or(Ok(base_bytes), |spec| required_gtop_sheaf_segment_bytes(base_bytes, spec))
            .map_err(|error| error.to_string())?;
        let staged = required.saturating_mul(2).saturating_add(16);
        if staged > budget { return Err("Encoded GTOP exceeds its byte budget".into()); }
        self.charge(work, staged)?;
        let mut storage = vec![0u8; required + 63];
        let start = storage.as_ptr().align_offset(64);
        let output = &mut storage[start..start + required];
        compile_geometry_segment(&geometry, budget - (required - base_bytes), &mut output[..base_bytes])
            .map_err(|error| format!("GTOP compaction: {error:?}"))?;
        if let Some(spec) = sheaf {
            append_gtop_sheaf_metadata(output, base_bytes, spec).map_err(|error| error.to_string())?;
        }
        let view = PoetGeometryView::from_segment_bytes(output).map_err(|error| format!("GTOP view: {error:?}"))?;
        let chi = verify_geometry_complex_homology(&view).map_err(|error| format!("GTOP host homology: {error:?}"))?;
        let graphs = graph_records(&view, &evidence)?;
        let b0 = graphs.first().map(|entry| entry.0).ok_or("GTOP primal graph is missing")?;
        let b1 = if geometry.complex_kind == GeometryComplexKind::ToroidalWaveLattice {
            u32::from(geometry.intrinsic_dim)
        } else if geometry.complex_kind == GeometryComplexKind::SurfaceManifold2D {
            u32::try_from(i64::from(b0) + i64::from(beta2) - i64::from(chi)).map_err(|_| "Inconsistent surface homology")?
        } else { 0 };
        let hits = requested_hops(args, &view)?;
        let sheaf = view.sheaf_metadata();
        if let Some(metadata) = sheaf {
            let predicate = register(&mut lexicon, HAS_STALK_SCHEMA)?;
            let context = q_hash(SCHEMA);
            let object = metadata.stalk_schema_q_hash;
            evidence.push(NQuin {
                subject: geometry_id, predicate, object, context, metadata: 0,
                parity: NQuin::calculate_parity(geometry_id, predicate, object, context, 0),
            });
        }
        let cbld = encode_lexicon(lexicon, &evidence, budget)?;
        let encoded = self.admit_encoded("gtop", encode_gtop_section(output)?)?;
        let semantic = self.admit_encoded("cbld", cbld)?;
        let mut output = BTreeMap::from([
            ("gtop".into(), encoded), ("cbld".into(), semantic),
            ("betti_0".into(), Value::U64(u64::from(b0))), ("betti_1".into(), Value::U64(u64::from(b1))),
            ("euler_chi".into(), Value::I64(i64::from(chi))),
            ("graphs".into(), Value::List(graphs.into_iter().map(|(_, value)| value).collect())),
            ("cell_hops".into(), Value::List(hits)),
        ]);
        if let Some(metadata) = sheaf {
            output.insert("stalk_schema_q_hash".into(), Value::U64(metadata.stalk_schema_q_hash));
            output.insert("restriction_csr_offset".into(), metadata.restriction_csr_offset
                .map_or(Value::Null, |offset| Value::U64(offset.into())));
        }
        Ok(Value::Record(output))
    }

    fn restriction_matrix(&mut self, value: &Value, budget: usize) -> Result<RestrictionMatrix, String> {
        let args = record(value)?;
        fields(args, &["rows", "columns", "row_offsets", "column_indices", "coefficients"])?;
        let rows = index(args, "rows")?;
        let columns = index(args, "columns")?;
        let offsets = list(required(args, "row_offsets")?)?;
        let indices = list(required(args, "column_indices")?)?;
        let coefficients = list(required(args, "coefficients")?)?;
        if rows == 0 || columns == 0 || rows as usize > MAX_TOTAL_OVERLAY_ROWS
            || columns as usize > MAX_TOTAL_OVERLAY_ROWS || offsets.len() != rows as usize + 1
            || indices.len() != coefficients.len() || coefficients.len() > MAX_TOTAL_OVERLAY_ROWS
        {
            return Err("GTOP sheaf CSR exceeds its shape or element bounds".into());
        }
        let bytes = offsets.len() * 4 + coefficients.len() * 12;
        if bytes > budget {
            return Err("GTOP sheaf CSR exceeds its storage budget".into());
        }
        self.charge(bytes.saturating_mul(4), bytes)?;
        let unsigned = |v: &Value| -> Result<u32, String> {
            u32::try_from(integer(v, "GTOP CSR index")?).map_err(|_| "GTOP CSR index exceeds u32".into())
        };
        Ok(RestrictionMatrix {
            rows, columns,
            row_offsets: offsets.iter().map(unsigned).collect::<Result<_, _>>()?,
            column_indices: indices.iter().map(unsigned).collect::<Result<_, _>>()?,
            coefficients: coefficients.iter().map(num).collect::<Result<_, _>>()?,
        })
    }
}

fn list_size(args: &Args, key: &str) -> Result<usize, String> {
    args.get(key).map_or(Ok(0), |value| list(value).map(<[Value]>::len))
}

fn iri(value: &str) -> Result<u64, String> {
    if value.len() > 4096 { return Err("GTOP IRI exceeds its lexical bound".into()); }
    resource_identity_meta(value).map_err(|error| error.to_string())?;
    Ok(q_hash(value))
}

fn register(dictionary: &mut BTreeMap<u64, String>, value: &str) -> Result<u64, String> {
    let hash = iri(value)?;
    if dictionary.get(&hash).is_some_and(|old| old != value) { return Err("GTOP q_hash collision".into()); }
    dictionary.entry(hash).or_insert_with(|| value.into());
    Ok(hash)
}

fn cell_iri(geometry: u64, domain: u8, index: u32) -> String {
    format!("urn:qualia:gtop:{geometry:016x}:cell:{domain}:{index}")
}

fn estimate_surface(vertices: usize, triangles: usize, budget: usize) -> Result<usize, String> {
    let estimate = vertices.saturating_mul(192).saturating_add(triangles.saturating_mul(1024));
    if vertices == 0 || triangles == 0 || vertices.saturating_add(triangles) > MAX_BASE_CELLS || estimate > budget {
        return Err("GTOP surface exceeds its vertex, triangle or memory bound".into());
    }
    Ok(estimate)
}

fn validate_coordinates(positions: &[[f32; 3]], metric: GeometryMetricKind) -> Result<(), String> {
    for &p in positions {
        let p = p.map(f64::from);
        if !p.iter().all(|v| v.is_finite()) { return Err("GTOP source contains non-finite coordinates".into()); }
        let square = p.iter().map(|x| x * x).sum::<f64>();
        let valid = match metric {
            GeometryMetricKind::HyperbolicPoincare => square < 1.0,
            GeometryMetricKind::SphericalGeodesic => (square - 1.0).abs() <= 1.0e-6,
            GeometryMetricKind::FisherRaoSimplex => p.iter().all(|&x| x >= 0.0) && (p.iter().sum::<f64>() - 1.0).abs() <= 1.0e-6,
            _ => true,
        };
        if !valid { return Err(format!("GTOP coordinates outside declared {metric:?} domain")); }
    }
    Ok(())
}

fn surface(
    id: u64, metric: GeometryMetricKind, vertices: usize, triangles: &[[u32; 3]], budget: usize,
) -> Result<(UnsealedGeometrySpec, u32), String> {
    let mut half_edges = vec![HalfEdge::default(); triangles.len() * 3];
    let mut slots = vec![EdgeSlot::default(); required_edge_slots(triangles.len())];
    build_triangle_half_edges(vertices as u32, triangles, &mut half_edges, &mut slots)
        .map_err(|error| format!("GTOP half-edge construction: {error:?}"))?;
    let (primal, dual) = induce_primal_and_dual_from_half_edges(id, &half_edges, vertices, triangles.len(), budget)
        .map_err(|error| format!("GTOP primal and dual induction: {error:?}"))?;
    let incidence = induce_simplicial_boundary_incidence(id, triangles, vertices as u32, budget)
        .map_err(|error| format!("GTOP boundary incidence: {error:?}"))?;
    let beta2 = closed_face_components(&half_edges);
    Ok((UnsealedGeometrySpec {
        geometry_id: id, construct_schema_hash: q_hash(SCHEMA), manifold_id: id as u32,
        complex_kind: GeometryComplexKind::SurfaceManifold2D, metric_kind: metric,
        ambient_dim: 3, intrinsic_dim: 2, cell_counts: [vertices as u32, (primal.edges.len() / 2) as u32, triangles.len() as u32, 0],
        active_graph_slot: 0, graphs: vec![primal, dual, incidence],
    }, beta2))
}

fn closed_face_components(half_edges: &[HalfEdge]) -> u32 {
    let mut seen = vec![false; half_edges.len() / 3];
    let mut visit = Vec::new();
    let mut closed_components = 0;
    for face in 0..seen.len() {
        if seen[face] { continue; }
        visit.push(face);
        seen[face] = true;
        let mut closed = true;
        while let Some(face) = visit.pop() {
            for edge in &half_edges[face * 3..face * 3 + 3] {
                if edge.twin == INVALID_INDEX {
                    closed = false;
                } else {
                    let next = half_edges[edge.twin as usize].face as usize;
                    if !seen[next] {
                        seen[next] = true;
                        visit.push(next);
                    }
                }
            }
        }
        closed_components += u32::from(closed);
    }
    closed_components
}

fn lattice(
    id: u64, metric: GeometryMetricKind, axes: &[u32], samples: &[f32], budget: usize,
) -> Result<(UnsealedGeometrySpec, usize), String> {
    let periodic = metric == GeometryMetricKind::ToroidalCyclic;
    if !(1..=3).contains(&axes.len()) || axes.iter().any(|&n| n < if periodic { 3 } else { 2 }) {
        return Err("Invalid GTOP dense lattice axis count or extent".into());
    }
    if !matches!(metric, GeometryMetricKind::Euclidean | GeometryMetricKind::ToroidalCyclic) {
        return Err("Dense GTOP lattice requires Euclidean or Toroidal geometry".into());
    }
    let vertices = axes.iter().try_fold(1usize, |n, &axis| n.checked_mul(axis as usize)).ok_or("GTOP lattice size overflow")?;
    let estimate = vertices.saturating_mul(384);
    if vertices > MAX_BASE_CELLS || estimate > budget || vertices != samples.len() || samples.iter().any(|x| !x.is_finite()) {
        return Err("GTOP lattice samples exceed their shape or memory bound".into());
    }
    let mut counts = [0u32; 4];
    for mask in 0..(1u32 << axes.len()) {
        let count = axes.iter().enumerate().map(|(axis, &n)| n - u32::from(!periodic && mask & (1 << axis) != 0)).product::<u32>();
        counts[mask.count_ones() as usize] += count;
    }
    let anchors = (0..vertices).map(|index| CellAnchor {
        cell_index: index as u32, anchor_domain: AnchorDomain::Cell0Vertex as u8, orientation_sign: 1,
        ..Default::default()
    }).collect();
    let mut edges = Vec::with_capacity(counts[1] as usize * 2);
    for vertex in 0..vertices as u32 {
        let mut stride = 1;
        for &extent in axes {
            let coord = vertex / stride % extent;
            let neighbor = if coord + 1 < extent { Some(vertex + stride) }
                else if periodic { Some(vertex - coord * stride) } else { None };
            if let Some(next) = neighbor {
                edges.extend([(vertex, next, u32::MAX), (next, vertex, u32::MAX)]);
            }
            stride *= extent;
        }
    }
    Ok((UnsealedGeometrySpec {
        geometry_id: id, construct_schema_hash: q_hash(SCHEMA), manifold_id: id as u32,
        complex_kind: if periodic { GeometryComplexKind::ToroidalWaveLattice } else { GeometryComplexKind::MixedCellComplex3D },
        metric_kind: metric, ambient_dim: axes.len() as u8, intrinsic_dim: axes.len() as u8,
        cell_counts: counts, active_graph_slot: 0,
        graphs: vec![UnsealedGraphSpec {
            graph_id: 0, embed_role: GraphEmbedRole::Primal1Skeleton, morphology: GraphMorphology::SymmetricGraph,
            invariants: TopologyInvariants::SYMMETRIC, context_and_state_mask: 0, anchors, edges, edge_weights: None,
        }],
    }, estimate))
}

fn evidence_word(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(v) => Ok(*v),
        Value::I64(v) => u64::try_from(*v).map_err(|_| "GTOP evidence word must be unsigned".into()),
        _ => Err("GTOP evidence word must be an exact u64".into()),
    }
}

fn read_evidence(args: &Args) -> Result<Vec<NQuin>, String> {
    let Some(value) = args.get("evidence") else { return Ok(Vec::new()) };
    list(value)?.iter().map(|row| {
        let words = list(row)?;
        let [s, p, o, c, m, parity] = words else { return Err("GTOP evidence needs six unsigned NQuin columns".into()) };
        let quin = NQuin {
            subject: evidence_word(s)?, predicate: evidence_word(p)?, object: evidence_word(o)?,
            context: evidence_word(c)?, metadata: evidence_word(m)?, parity: evidence_word(parity)?,
        };
        if !quin.verify_ecc_parity() { return Err("GTOP evidence parity mismatch".into()); }
        Ok(quin)
    }).collect()
}

fn index(args: &Args, key: &str) -> Result<u32, String> {
    u32::try_from(integer(required(args, key)?, key)?).map_err(|_| format!("Invalid GTOP {key}"))
}

fn domain(args: &Args) -> Result<AnchorDomain, String> {
    AnchorDomain::from_name(text(required(args, "domain")?)?).ok_or_else(|| "Unknown GTOP anchor domain".into())
}

fn validate_cell(kind: AnchorDomain, index: u32, geometry: &UnsealedGeometrySpec, evidence: usize) -> Result<(), String> {
    let count = match kind {
        AnchorDomain::SemanticQuin => evidence,
        AnchorDomain::CellKSimplex => return Err("GTOP source has no cells above dimension three".into()),
        _ => geometry.cell_counts[kind as usize] as usize,
    };
    if index as usize >= count { return Err(format!("GTOP {kind:?} cell index {index} outside count {count}")); }
    Ok(())
}

fn parse_graph(
    value: &Value, geometry: &UnsealedGeometrySpec, evidence: &[NQuin], lexicon: &mut BTreeMap<u64, String>,
) -> Result<UnsealedGraphSpec, String> {
    let args = record(value)?;
    fields(args, &["graph_iri", "role", "morphology", "invariants", "anchors", "edges"])?;
    let role = GraphEmbedRole::from_name(text(required(args, "role")?)?).ok_or("Unknown GTOP graph role")?;
    let morphology = GraphMorphology::from_name(text(required(args, "morphology")?)?).ok_or("Unknown GTOP graph morphology")?;
    let mut invariants = TopologyInvariants::EMPTY;
    if let Some(raw) = args.get("invariants") {
        for value in list(raw)? {
            let invariant = TopologyInvariants::from_name(text(value)?).ok_or("Unknown GTOP invariant")?;
            if ![
                TopologyInvariants::ACYCLIC, TopologyInvariants::SYMMETRIC, TopologyInvariants::REIFIED_EDGES,
                TopologyInvariants::TEMPORAL_MONOTONIC, TopologyInvariants::DEONTIC_GUARDED,
                TopologyInvariants::PARACONSISTENT_ISOLATED, TopologyInvariants::SENSITIVITY_MONOTONIC,
            ].contains(&invariant) {
                return Err(format!("GTOP overlay invariant {} requires a host-specific validator", text(value)?));
            }
            invariants |= invariant;
        }
    }
    if morphology == GraphMorphology::SymmetricGraph { invariants |= TopologyInvariants::SYMMETRIC; }
    if morphology == GraphMorphology::HyperGraph { invariants |= TopologyInvariants::REIFIED_EDGES; }
    if role == GraphEmbedRole::DeonticGovernance { invariants |= TopologyInvariants::DEONTIC_GUARDED; }
    let mut anchors = Vec::new();
    for raw in list(required(args, "anchors")?)? {
        let row = record(raw)?;
        fields(row, &["domain", "cell_index", "entity_iri", "orientation", "material_or_stratum"])?;
        let domain = domain(row)?;
        let cell_index = index(row, "cell_index")?;
        validate_cell(domain, cell_index, geometry, evidence.len())?;
        let entity_hash = row.get("entity_iri").map_or(Ok(0), |value| register(lexicon, text(value)?))?;
        if domain == AnchorDomain::SemanticQuin && (entity_hash == 0 || evidence[cell_index as usize].subject != entity_hash) {
            return Err("Semantic GTOP anchor must resolve to its evidence subject IRI".into());
        }
        let orientation_sign = match row.get("orientation").map(num).transpose()?.unwrap_or(1.0) {
            -1.0 => -1, 0.0 => 0, 1.0 => 1,
            _ => return Err("GTOP orientation must be -1, 0 or 1".into()),
        };
        let material_or_stratum = row.get("material_or_stratum").map_or(Ok(0), |value| -> Result<u16, String> {
            u16::try_from(integer(value, "material_or_stratum")?).map_err(|_| "Invalid GTOP material or stratum".into())
        })?;
        anchors.push(CellAnchor { entity_hash, cell_index, anchor_domain: domain as u8, orientation_sign, material_or_stratum });
    }
    if anchors.is_empty() { return Err("GTOP overlay has no anchors".into()); }
    let rows = list(required(args, "edges")?)?;
    let mut edges = Vec::with_capacity(rows.len());
    let mut weights = Vec::with_capacity(rows.len());
    let mut weighted = false;
    for raw in rows {
        let row = record(raw)?;
        fields(row, &["source", "target", "evidence_index", "weight"])?;
        let proof = match row.get("evidence_index") {
            Some(_) => {
                let proof = index(row, "evidence_index")?;
                if proof as usize >= evidence.len() { return Err("GTOP edge evidence index out of bounds".into()); }
                proof
            }
            None => u32::MAX,
        };
        edges.push((index(row, "source")?, index(row, "target")?, proof));
        let weight = row.get("weight").map(num).transpose()?.unwrap_or(1.0) as f32;
        if !weight.is_finite() { return Err("GTOP edge weight outside finite f32 range".into()); }
        weights.push(weight);
        weighted |= row.contains_key("weight");
    }
    Ok(UnsealedGraphSpec {
        graph_id: register(lexicon, text(required(args, "graph_iri")?)?)?, embed_role: role,
        morphology, invariants, context_and_state_mask: 0, anchors, edges,
        edge_weights: weighted.then_some(weights),
    })
}

fn mutation(
    args: &Args, geometry: &UnsealedGeometrySpec, builtin: usize, evidence_len: usize,
    ring: &mut TopologicalDeltaRing<256>,
) -> Result<(), String> {
    fields(args, &["op", "graph_slot", "source", "target", "domain", "evidence_index"])?;
    let slot = index(args, "graph_slot")?;
    if (slot as usize) < builtin || slot as usize >= geometry.graphs.len() {
        return Err("GTOP delta mutations require an authored overlay slot".into());
    }
    let source = index(args, "source")?;
    let target = index(args, "target")?;
    let graph = &geometry.graphs[slot as usize];
    if source as usize >= graph.anchors.len() { return Err("GTOP delta source vertex out of bounds".into()); }
    let op = text(required(args, "op")?)?;
    if op != "bind_anchor" && target as usize >= graph.anchors.len() {
        return Err("GTOP delta target vertex out of bounds".into());
    }
    let proof = if args.contains_key("evidence_index") { index(args, "evidence_index")? } else { u32::MAX };
    if proof != u32::MAX && proof as usize >= evidence_len {
        return Err("GTOP delta evidence index outside admitted rows".into());
    }
    match op {
        "insert_edge" => ring.insert_edge(slot as u8, source, target, proof),
        "tombstone_edge" => ring.tombstone_edge(slot as u8, source, target),
        "reify_edge" if proof != u32::MAX => ring.reify_edge(slot as u8, source, target, proof),
        "bind_anchor" => {
            let domain = domain(args)?;
            if domain == AnchorDomain::SemanticQuin {
                return Err("GTOP semantic rebinding requires an authored evidence anchor".into());
            }
            validate_cell(domain, target, geometry, 0)?;
            ring.bind_anchor(slot as u8, source, target, domain)
        }
        _ => return Err("Unknown or incomplete GTOP delta mutation".into()),
    }.map_err(|error| format!("GTOP delta ring: {error:?}"))?;
    Ok(())
}

fn validate_graph(graph: &UnsealedGraphSpec, evidence: &[NQuin]) -> Result<(), String> {
    let mut keys = BTreeSet::new();
    let guarded = graph.invariants.contains(TopologyInvariants::DEONTIC_GUARDED)
        || graph.invariants.contains(TopologyInvariants::REIFIED_EDGES)
        || graph.invariants.contains(TopologyInvariants::PARACONSISTENT_ISOLATED)
        || graph.invariants.contains(TopologyInvariants::SENSITIVITY_MONOTONIC)
        || graph.invariants.contains(TopologyInvariants::TEMPORAL_MONOTONIC);
    for &(from, to, proof) in &graph.edges {
        let source = graph.anchors.get(from as usize).ok_or("GTOP edge source vertex outside graph")?;
        let target = graph.anchors.get(to as usize).ok_or("GTOP edge target vertex outside graph")?;
        if !keys.insert((from, to)) { return Err("Duplicate directed GTOP edge".into()); }
        if guarded {
            let quin = evidence.get(proof as usize).ok_or("Guarded GTOP edge requires admitted evidence")?;
            if source.entity_hash != quin.subject || target.entity_hash != quin.object {
                return Err("Guarded GTOP evidence must bind the source and target anchors".into());
            }
        }
        if graph.invariants.contains(TopologyInvariants::TEMPORAL_MONOTONIC)
            && [source, target].iter().any(|anchor| anchor.anchor_domain != AnchorDomain::SemanticQuin as u8)
        {
            return Err("Temporal GTOP overlays require source and target Lamport evidence anchors".into());
        }
    }
    if graph.invariants.contains(TopologyInvariants::SYMMETRIC)
        && keys.iter().any(|&(from, to)| from == to || !keys.contains(&(to, from)))
    {
        return Err("GTOP symmetry requires reciprocal non-self edges".into());
    }
    Ok(())
}

fn graph_records(view: &PoetGeometryView<'_>, evidence: &[NQuin]) -> Result<Vec<(u32, Value)>, String> {
    let mut rows = Vec::with_capacity(view.harboured_graph_count());
    for slot in 0..view.harboured_graph_count() {
        let graph = view.graph_at_slot(slot as u16).map_err(|error| format!("GTOP graph readback: {error:?}"))?;
        let vertices = graph.vertex_count() as usize;
        let mut bits = vec![0; vertices.div_ceil(64)];
        let mut queue = vec![0; vertices];
        let result = verify_graph_invariants(&graph, evidence, None, &mut bits, &mut queue)
            .map_err(|error| format!("GTOP graph invariant at slot {slot}: {error:?}"))?;
        let flags = graph.header().invariants;
        let edges = if flags.contains(TopologyInvariants::SYMMETRIC) { graph.edge_count() / 2 } else { graph.edge_count() };
        let beta1 = (i64::from(edges) + i64::from(result.betti_0) - vertices as i64).max(0);
        rows.push((result.betti_0, Value::Record(BTreeMap::from([
            ("slot".into(), Value::U64(slot as u64)), ("graph_id".into(), Value::U64(graph.header().graph_id)),
            ("vertex_count".into(), Value::U64(vertices as u64)), ("edge_count".into(), Value::U64(u64::from(edges))),
            ("betti_0".into(), Value::U64(u64::from(result.betti_0))), ("betti_1".into(), Value::U64(beta1 as u64)),
            ("euler_chi".into(), Value::I64(vertices as i64 - i64::from(edges))),
            ("acyclic_enforced".into(), Value::Bool(flags.contains(TopologyInvariants::ACYCLIC)
                || graph.header().morphology == GraphMorphology::DeterministicDAG as u8)),
            ("deontic_guarded".into(), Value::Bool(flags.contains(TopologyInvariants::DEONTIC_GUARDED))),
        ]))));
    }
    Ok(rows)
}

fn requested_hops(args: &Args, view: &PoetGeometryView<'_>) -> Result<Vec<Value>, String> {
    let Some(raw) = args.get("hops") else { return Ok(Vec::new()) };
    let hops = list(raw)?;
    if hops.len() > 256 { return Err("GTOP cell hop count exceeds bound".into()); }
    let mut parsed = Vec::with_capacity(hops.len());
    let mut destinations = BTreeSet::new();
    for raw in hops {
        let row = record(raw)?;
        fields(row, &["from_slot", "vertex", "to_slot"])?;
        let from = u16::try_from(index(row, "from_slot")?).map_err(|_| "Invalid GTOP source slot")?;
        let to = u16::try_from(index(row, "to_slot")?).map_err(|_| "Invalid GTOP target slot")?;
        let source = view.graph_at_slot(from).map_err(|error| format!("GTOP hop source: {error:?}"))?;
        let vertex = source.cell_anchor(index(row, "vertex")?).map_err(|error| format!("GTOP hop vertex: {error:?}"))?;
        parsed.push((to, anchor_key(vertex)));
        destinations.insert(to);
    }
    let mut index = HashMap::<(u16, (u8, u64)), Vec<u32>>::new();
    for to in destinations {
        let target = view.graph_at_slot(to).map_err(|error| format!("GTOP hop target: {error:?}"))?;
        for i in 0..target.vertex_count() {
            let vertex = target.cell_anchor(i).map_err(|error| format!("GTOP target vertex: {error:?}"))?;
            index.entry((to, anchor_key(vertex))).or_default().push(i);
        }
    }
    if parsed.iter().map(|address| index.get(address).map_or(0, Vec::len)).sum::<usize>() > MAX_TOTAL_OVERLAY_ROWS {
        return Err("GTOP cell hops exceed their result bound".into());
    }
    Ok(parsed.into_iter().map(|address| {
        Value::List(index.get(&address).into_iter().flatten().map(|&vertex| Value::U64(u64::from(vertex))).collect())
    }).collect())
}

fn anchor_key(anchor: &CellAnchor) -> (u8, u64) {
    if anchor.anchor_domain == AnchorDomain::SemanticQuin as u8 {
        (anchor.anchor_domain, anchor.entity_hash)
    } else {
        (anchor.anchor_domain, u64::from(anchor.cell_index))
    }
}

fn encode_lexicon(lexicon: BTreeMap<u64, String>, evidence: &[NQuin], budget: usize) -> Result<Vec<u8>, String> {
    use qualia_core_db::container_10d::{MultilingualLexicalTerm, SymbolicRdfCborLdPayload, encode_symbolic_rdf_cbor_ld};
    let source_size = lexicon.values().map(|iri| iri.len().saturating_add(64)).sum::<usize>()
        .saturating_add(core::mem::size_of_val(evidence));
    if source_size > budget { return Err("GTOP lexical dictionary exceeds its byte budget".into()); }
    let mut payload = SymbolicRdfCborLdPayload {
        context_iri: SCHEMA.into(),
        lexicon: lexicon.into_iter().map(|(term_hash, iri_utf8)| MultilingualLexicalTerm {
            term_hash, iri_utf8, lang_bcp47: None, label_utf8: String::new(),
        }).collect(),
        ..Default::default()
    };
    for quin in evidence { payload.push_nquin(quin); }
    encode_symbolic_rdf_cbor_ld(&payload).map_err(|error| format!("GTOP lexical dictionary: {error:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::q42_volume::payload_catalogue::decode_gtop_section;
    use std::{sync::{Arc, atomic::AtomicBool}};

    #[test]
    fn torus_seals_as_two_cycles_with_a_cross_graph_cell_hop() {
        let limits = crate::ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = AssetHost { workspace: Default::default(), inputs: &[], limits: &limits, cancel: &cancel, work: 0 };
        let source = host.admit_artifact(TypedArtifact::FieldLattice(Arc::new(FieldLattice::DenseF32 {
            axes: vec![3, 3], samples: vec![0.0; 9],
        }))).unwrap();
        let u = |value| Value::U64(value);
        let s = |value: &str| Value::String(value.into());
        let args = Args::from([
            ("source".into(), source), ("geometry_iri".into(), s("urn:test:gtop:torus")),
            ("metric_regime".into(), s("Toroidal")),
            ("graphs".into(), Value::List(vec![Value::Record(Args::from([
                ("graph_iri".into(), s("urn:test:gtop:overlay")), ("role".into(), s("SemanticOverlay")),
                ("morphology".into(), s("DirectedMultigraph")),
                ("anchors".into(), Value::List(vec![Value::Record(Args::from([
                    ("domain".into(), s("Vertex")), ("cell_index".into(), u(4)),
                ]))])),
                ("edges".into(), Value::List(vec![])),
            ]))])),
            ("hops".into(), Value::List(vec![Value::Record(Args::from([
                ("from_slot".into(), u(0)), ("vertex".into(), u(4)), ("to_slot".into(), u(1)),
            ]))])),
        ]);
        let result = host.harbour_gtop_geometry_graphs(&args).unwrap();
        let result = record(&result).unwrap();
        assert!(matches!(result["betti_0"], Value::U64(1)));
        assert!(matches!(result["betti_1"], Value::U64(2)));
        assert!(matches!(result["euler_chi"], Value::I64(0)));
        let hops = list(&result["cell_hops"]).unwrap();
        assert!(matches!(list(&hops[0]).unwrap(), [Value::U64(0)]));
        let encoded = host.workspace.encoded(&result["gtop"]).unwrap();
        assert!(decode_gtop_section(&encoded.bytes).is_ok());
    }
}
