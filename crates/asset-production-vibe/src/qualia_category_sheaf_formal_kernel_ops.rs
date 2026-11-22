//! WP-31 finite attributed graph constructions and Q42 witnesses.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::{BTreeMap, BTreeSet};

use qualia_core_db::{
    bundle::semantic_model::resource_identity_meta,
    q_hash,
    solvers::category_kan_operad::{
        self as native, Counts, Diagram, Edge, Graph, GraphMap, Operation, PairWitness,
        PullbackOutput, PushoutOutput, QuotientScratch, MAX_INPUT_EDGES, MAX_INPUT_VERTICES,
    },
};
use sha2::{Digest, Sha256};

use crate::{
    fields, integer, list, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    record, required, text, Args, AssetHost, Value,
};

const SENTINEL: usize = 42 * 1024 * 1024;
const TERM_BYTES: usize = 1024;

macro_rules! cat {
    ($name:literal) => {
        concat!("https://qualiadb.org/ontology/finite-graph#", $name)
    };
}

struct AuthoredGraph<'a> {
    vertex_iris: Vec<&'a str>,
    edge_iris: Vec<&'a str>,
    vertices: Vec<u64>,
    edges: Vec<Edge>,
}

impl AuthoredGraph<'_> {
    fn view(&self) -> Graph<'_> {
        Graph {
            vertices: &self.vertices,
            edges: &self.edges,
        }
    }
}

struct AuthoredMap {
    vertices: Vec<u32>,
    edges: Vec<u32>,
}

impl AuthoredMap {
    fn view(&self) -> GraphMap<'_> {
        GraphMap {
            vertices: &self.vertices,
            edges: &self.edges,
        }
    }
}

enum Witness {
    Projections {
        vertices: Vec<[u32; 2]>,
        edges: Vec<[u32; 2]>,
    },
    Injections {
        vertices: Vec<u32>,
        edges: Vec<u32>,
    },
}

impl AssetHost<'_> {
    pub(crate) fn compute_pushout_pullback_and_kan_extension(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "operation",
                "base",
                "left",
                "right",
                "left_map",
                "right_map",
                "arena_budget_bytes",
            ],
        )?;
        let operation = match text(required(args, "operation")?)? {
            "pullback" => Operation::Pullback,
            "pushout" => Operation::Pushout,
            _ => {
                return Err("Finite attributed graph operation must be pullback or pushout".into())
            }
        };
        let budget = args
            .get("arena_budget_bytes")
            .map(|value| integer(value, "arena_budget_bytes"))
            .transpose()?
            .unwrap_or(SENTINEL);
        if budget > SENTINEL {
            return Err("Finite graph arena exceeds the 42 MiB symbolic limit".into());
        }
        let mut attributes = BTreeMap::new();
        let base = authored_graph(required(args, "base")?, &mut attributes)?;
        let left = authored_graph(required(args, "left")?, &mut attributes)?;
        let right = authored_graph(required(args, "right")?, &mut attributes)?;
        let (l_domain, r_domain) = match operation {
            Operation::Pullback => (&left, &right),
            Operation::Pushout => (&base, &base),
        };
        let left_map = authored_map(required(args, "left_map")?, l_domain)?;
        let right_map = authored_map(required(args, "right_map")?, r_domain)?;
        let diagram = Diagram {
            base: base.view(),
            left: left.view(),
            right: right.view(),
            left_map: left_map.view(),
            right_map: right_map.view(),
        };
        let input_cells = [diagram.base, diagram.left, diagram.right]
            .into_iter()
            .map(|g| g.vertices.len() + g.edges.len())
            .sum::<usize>();
        let work = match operation {
            Operation::Pullback => {
                diagram.left.vertices.len() * diagram.right.vertices.len()
                    + diagram.left.edges.len() * diagram.right.edges.len()
            }
            Operation::Pushout => input_cells.saturating_mul(24),
        };
        self.charge(work.saturating_mul(5), input_cells.saturating_mul(128))?;
        let capacity = native::output_capacity(diagram, operation).map_err(problem)?;
        let scratch_slots = capacity.vertices.max(capacity.edges);
        let output_bytes = capacity.vertices * std::mem::size_of::<u64>()
            + capacity.edges * std::mem::size_of::<Edge>();
        let arena_bytes = output_bytes
            + match operation {
                Operation::Pullback => {
                    (capacity.vertices + capacity.edges) * std::mem::size_of::<[u32; 2]>()
                }
                Operation::Pushout => {
                    (capacity.vertices + capacity.edges + 2 * scratch_slots)
                        * std::mem::size_of::<u32>()
                }
            };
        if arena_bytes > budget {
            return Err(format!(
                "Finite graph solver requires {arena_bytes} caller-buffer bytes"
            ));
        }
        let output_cells = capacity.vertices + capacity.edges;
        self.charge(
            output_cells.saturating_mul(256),
            arena_bytes.saturating_add(output_cells.saturating_mul(1400)),
        )?;
        let mut vertices = vec![0; capacity.vertices];
        let mut edges = vec![Edge::default(); capacity.edges];
        let witness = match operation {
            Operation::Pullback => {
                let mut vertex_pairs = vec![[0; 2]; capacity.vertices];
                let mut edge_pairs = vec![[0; 2]; capacity.edges];
                let counts = native::pullback_into(
                    diagram,
                    PullbackOutput {
                        vertices: &mut vertices,
                        edges: &mut edges,
                        vertex_pairs: &mut vertex_pairs,
                        edge_pairs: &mut edge_pairs,
                    },
                )
                .map_err(problem)?;
                trim(counts, &mut vertices, &mut edges);
                native::verify_pullback(
                    diagram,
                    Graph {
                        vertices: &vertices,
                        edges: &edges,
                    },
                    PairWitness {
                        vertices: &vertex_pairs,
                        edges: &edge_pairs,
                    },
                )
                .map_err(problem)?;
                Witness::Projections {
                    vertices: vertex_pairs,
                    edges: edge_pairs,
                }
            }
            Operation::Pushout => {
                let mut vertex_images = vec![0; capacity.vertices];
                let mut edge_images = vec![0; capacity.edges];
                let mut parent = vec![0; scratch_slots];
                let mut class = vec![0; scratch_slots];
                let counts = native::pushout_into(
                    diagram,
                    QuotientScratch {
                        parent: &mut parent,
                        class: &mut class,
                    },
                    PushoutOutput {
                        vertices: &mut vertices,
                        edges: &mut edges,
                        vertex_images: &mut vertex_images,
                        edge_images: &mut edge_images,
                    },
                )
                .map_err(problem)?;
                trim(counts, &mut vertices, &mut edges);
                native::verify_pushout(
                    diagram,
                    Graph {
                        vertices: &vertices,
                        edges: &edges,
                    },
                    GraphMap {
                        vertices: &vertex_images,
                        edges: &edge_images,
                    },
                    QuotientScratch {
                        parent: &mut parent,
                        class: &mut class,
                    },
                )
                .map_err(problem)?;
                Witness::Injections {
                    vertices: vertex_images,
                    edges: edge_images,
                }
            }
        };
        let context = diagram_iri(
            operation,
            [&base, &left, &right],
            [&left_map, &right_map],
            &attributes,
        );
        let result = Graph {
            vertices: &vertices,
            edges: &edges,
        };
        let output = authored_output(&context, result, &attributes)?;
        let receipt = graph_receipt(
            &context,
            result,
            [&base, &left, &right],
            [&left_map, &right_map],
            &attributes,
            &witness,
            self.limits.resource_bytes.min(SENTINEL),
        )?;
        let witness_value = witness.value(&left);
        let receipt = self.admit_encoded("q42", receipt)?;
        Ok(Value::Record(Args::from([
            ("graph".into(), output),
            ("witness".into(), witness_value),
            ("receipt".into(), receipt),
            ("arena_bytes".into(), Value::U64(arena_bytes as u64)),
        ])))
    }
}

fn problem(error: native::Error) -> String {
    format!("Finite graph construction: {error:?}")
}

fn trim(counts: Counts, vertices: &mut Vec<u64>, edges: &mut Vec<Edge>) {
    vertices.truncate(counts.vertices);
    edges.truncate(counts.edges);
}

fn valid_iri(iri: &str) -> Result<u64, String> {
    if iri.len() > TERM_BYTES {
        return Err("Finite graph term exceeds its IRI bound".into());
    }
    resource_identity_meta(iri).map_err(|error| error.to_string())?;
    let hash = q_hash(iri);
    if hash == 0 {
        return Err("Finite graph term has a zero identity hash".into());
    }
    Ok(hash)
}

fn authored_graph<'a>(
    value: &'a Value,
    attributes: &mut BTreeMap<u64, &'a str>,
) -> Result<AuthoredGraph<'a>, String> {
    let row = record(value)?;
    fields(row, &["vertices", "edges"])?;
    let vertices = list(required(row, "vertices")?)?;
    let edges = list(required(row, "edges")?)?;
    if vertices.len() > MAX_INPUT_VERTICES || edges.len() > MAX_INPUT_EDGES {
        return Err("Finite graph input exceeds its vertex/edge bound".into());
    }
    let mut seen = BTreeSet::new();
    let mut put_term = |row: &'a Args| -> Result<(&'a str, u64), String> {
        let iri = text(required(row, "iri")?)?;
        if !seen.insert(valid_iri(iri)?) {
            return Err("Repeated or colliding graph cell identity".into());
        }
        let attribute = text(required(row, "attribute")?)?;
        let hash = valid_iri(attribute)?;
        if attributes
            .get(&hash)
            .is_some_and(|previous| *previous != attribute)
        {
            return Err("Conflicting finite graph attribute hashes".into());
        }
        attributes.entry(hash).or_insert(attribute);
        Ok((iri, hash))
    };
    let mut graph = AuthoredGraph {
        vertex_iris: Vec::with_capacity(vertices.len()),
        vertices: Vec::with_capacity(vertices.len()),
        edge_iris: Vec::with_capacity(edges.len()),
        edges: Vec::with_capacity(edges.len()),
    };
    for value in vertices {
        let row = record(value)?;
        fields(row, &["iri", "attribute"])?;
        let (iri, attr) = put_term(row)?;
        graph.vertex_iris.push(iri);
        graph.vertices.push(attr);
    }
    for value in edges {
        let row = record(value)?;
        fields(row, &["iri", "attribute", "source", "target"])?;
        let (iri, attribute) = put_term(row)?;
        graph.edge_iris.push(iri);
        graph.edges.push(Edge {
            source: index(required(row, "source")?)?,
            target: index(required(row, "target")?)?,
            attribute,
        });
    }
    Ok(graph)
}

fn index(value: &Value) -> Result<u32, String> {
    u32::try_from(integer(value, "finite graph slot")?)
        .map_err(|_| "Finite graph slot exceeds u32".into())
}

fn authored_map(value: &Value, domain: &AuthoredGraph<'_>) -> Result<AuthoredMap, String> {
    let row = record(value)?;
    fields(row, &["vertices", "edges"])?;
    let vertices = list(required(row, "vertices")?)?;
    let edges = list(required(row, "edges")?)?;
    if vertices.len() != domain.vertices.len() || edges.len() != domain.edges.len() {
        return Err("Finite graph map must be total on both vertex and edge slots".into());
    }
    Ok(AuthoredMap {
        vertices: vertices.iter().map(index).collect::<Result<_, _>>()?,
        edges: edges.iter().map(index).collect::<Result<_, _>>()?,
    })
}

fn diagram_iri(
    operation: Operation,
    inputs: [&AuthoredGraph<'_>; 3],
    maps: [&AuthoredMap; 2],
    attributes: &BTreeMap<u64, &str>,
) -> String {
    let mut digest = Sha256::new();
    digest.update(b"qualia:finite-attributed-graph:v1");
    digest.update([match operation {
        Operation::Pullback => 0,
        Operation::Pushout => 1,
    }]);
    fn name(digest: &mut Sha256, value: &str) {
        digest.update((value.len() as u64).to_le_bytes());
        digest.update(value.as_bytes());
    }
    for graph in inputs {
        digest.update((graph.vertices.len() as u64).to_le_bytes());
        for (&iri, attr) in graph.vertex_iris.iter().zip(&graph.vertices) {
            name(&mut digest, iri);
            name(&mut digest, attributes[attr]);
        }
        digest.update((graph.edges.len() as u64).to_le_bytes());
        for (&iri, edge) in graph.edge_iris.iter().zip(&graph.edges) {
            name(&mut digest, iri);
            name(&mut digest, attributes[&edge.attribute]);
            digest.update(edge.source.to_le_bytes());
            digest.update(edge.target.to_le_bytes());
        }
    }
    for map in maps {
        for images in [&map.vertices, &map.edges] {
            digest.update((images.len() as u64).to_le_bytes());
            for image in images {
                digest.update(image.to_le_bytes());
            }
        }
    }
    format!("urn:qualia:finite-graph:{:x}", digest.finalize())
}

#[derive(Clone, Copy)]
enum Cell {
    Vertex,
    Edge,
}

impl Cell {
    fn token(self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Edge => "edge",
        }
    }
    fn iri(self, graph: &str, slot: usize) -> String {
        format!("{graph}:{}:{slot}", self.token())
    }
}

fn authored_output(
    context: &str,
    graph: Graph<'_>,
    attributes: &BTreeMap<u64, &str>,
) -> Result<Value, String> {
    let node = |cell: Cell, index: usize, attribute| -> Result<Args, String> {
        Ok(Args::from([
            (
                "iri".into(),
                Value::String(cell.iri(&format!("{context}:result"), index)),
            ),
            (
                "attribute".into(),
                Value::String(
                    attributes
                        .get(&attribute)
                        .ok_or("Unknown output attribute")
                        .copied()?
                        .into(),
                ),
            ),
        ]))
    };
    let vertices = graph
        .vertices
        .iter()
        .enumerate()
        .map(|(i, &attr)| node(Cell::Vertex, i, attr).map(Value::Record))
        .collect::<Result<Vec<_>, _>>()?;
    let edges = graph
        .edges
        .iter()
        .enumerate()
        .map(|(i, edge)| {
            let mut value = node(Cell::Edge, i, edge.attribute)?;
            value.insert("source".into(), Value::U64(u64::from(edge.source)));
            value.insert("target".into(), Value::U64(u64::from(edge.target)));
            Ok(Value::Record(value))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Value::Record(Args::from([
        ("vertices".into(), Value::List(vertices)),
        ("edges".into(), Value::List(edges)),
    ])))
}

impl Witness {
    fn value(&self, left: &AuthoredGraph<'_>) -> Value {
        fn map(vertices: impl Iterator<Item = u32>, edges: impl Iterator<Item = u32>) -> Value {
            let numbers = |n| Value::U64(u64::from(n));
            Value::Record(Args::from([
                (
                    "vertices".into(),
                    Value::List(vertices.map(numbers).collect()),
                ),
                ("edges".into(), Value::List(edges.map(numbers).collect())),
            ]))
        }
        let (left, right, direction) = match self {
            Self::Projections { vertices, edges } => (
                map(
                    vertices.iter().map(|pair| pair[0]),
                    edges.iter().map(|pair| pair[0]),
                ),
                map(
                    vertices.iter().map(|pair| pair[1]),
                    edges.iter().map(|pair| pair[1]),
                ),
                "projections",
            ),
            Self::Injections { vertices, edges } => (
                map(
                    vertices[..left.vertices.len()].iter().copied(),
                    edges[..left.edges.len()].iter().copied(),
                ),
                map(
                    vertices[left.vertices.len()..].iter().copied(),
                    edges[left.edges.len()..].iter().copied(),
                ),
                "injections",
            ),
        };
        Value::Record(Args::from([
            ("left".into(), left),
            ("right".into(), right),
            ("direction".into(), Value::String(direction.into())),
        ]))
    }
}

fn graph_receipt(
    context: &str,
    result: Graph<'_>,
    input: [&AuthoredGraph<'_>; 3],
    maps: [&AuthoredMap; 2],
    attributes: &BTreeMap<u64, &str>,
    witness: &Witness,
    budget: usize,
) -> Result<Vec<u8>, String> {
    let mut graph = ReceiptGraph::new(context);
    let [base, left, right, output] =
        ["base", "left", "right", "result"].map(|name| format!("{context}:{name}"));
    let kind = match witness {
        Witness::Projections { .. } => cat!("Pullback"),
        Witness::Injections { .. } => cat!("Pushout"),
    };
    graph.fact(context, cat!("construction"), kind)?;
    graph.fact(context, cat!("base"), &base)?;
    graph.fact(context, cat!("left"), &left)?;
    graph.fact(context, cat!("right"), &right)?;
    graph.fact(context, cat!("result"), &output)?;
    for (name, source) in [&base, &left, &right].into_iter().zip(input) {
        record_graph(&mut graph, name, source.view(), attributes)?;
        for (cell, iris) in [
            (Cell::Vertex, &source.vertex_iris),
            (Cell::Edge, &source.edge_iris),
        ] {
            for (slot, iri) in iris.iter().enumerate() {
                graph.fact(&cell.iri(name, slot), cat!("authoredIdentity"), iri)?;
            }
        }
    }
    record_graph(&mut graph, &output, result, attributes)?;
    for (map, side) in maps.into_iter().zip([&left, &right]) {
        let (domain, codomain, relation) = match witness {
            Witness::Projections { .. } => (side, &base, cat!("diagramImage")),
            Witness::Injections { .. } => (
                &base,
                side,
                if side == &left {
                    cat!("leftDiagramImage")
                } else {
                    cat!("rightDiagramImage")
                },
            ),
        };
        for (cell, images) in [(Cell::Vertex, &map.vertices), (Cell::Edge, &map.edges)] {
            for (slot, &image) in images.iter().enumerate() {
                graph.fact(
                    &cell.iri(domain, slot),
                    relation,
                    &cell.iri(codomain, image as usize),
                )?;
            }
        }
    }
    match witness {
        Witness::Projections { vertices, edges } => {
            for (cell, pairs) in [(Cell::Vertex, vertices), (Cell::Edge, edges)] {
                for (slot, &[a, b]) in pairs.iter().enumerate() {
                    let id = cell.iri(&output, slot);
                    graph.fact(&id, cat!("leftProjection"), &cell.iri(&left, a as usize))?;
                    graph.fact(&id, cat!("rightProjection"), &cell.iri(&right, b as usize))?;
                }
            }
        }
        Witness::Injections { vertices, edges } => {
            for (cell, left_count, images) in [
                (Cell::Vertex, input[1].vertices.len(), vertices),
                (Cell::Edge, input[1].edges.len(), edges),
            ] {
                for (side, images) in [
                    (&left, &images[..left_count]),
                    (&right, &images[left_count..]),
                ] {
                    for (slot, &image) in images.iter().enumerate() {
                        graph.fact(
                            &cell.iri(side, slot),
                            cat!("quotientImage"),
                            &cell.iri(&output, image as usize),
                        )?;
                    }
                }
            }
        }
    }
    graph.seal(budget)
}

fn record_graph(
    receipt: &mut ReceiptGraph,
    name: &str,
    graph: Graph<'_>,
    attributes: &BTreeMap<u64, &str>,
) -> Result<(), String> {
    for (slot, attribute) in graph.vertices.iter().enumerate() {
        let iri = Cell::Vertex.iri(name, slot);
        receipt.fact(name, cat!("vertex"), &iri)?;
        receipt.fact(&iri, cat!("attribute"), attributes[attribute])?;
    }
    for (slot, edge) in graph.edges.iter().enumerate() {
        let iri = Cell::Edge.iri(name, slot);
        receipt.fact(name, cat!("edge"), &iri)?;
        receipt.fact(&iri, cat!("attribute"), attributes[&edge.attribute])?;
        receipt.fact(
            &iri,
            cat!("source"),
            &Cell::Vertex.iri(name, edge.source as usize),
        )?;
        receipt.fact(
            &iri,
            cat!("target"),
            &Cell::Vertex.iri(name, edge.target as usize),
        )?;
    }
    Ok(())
}
