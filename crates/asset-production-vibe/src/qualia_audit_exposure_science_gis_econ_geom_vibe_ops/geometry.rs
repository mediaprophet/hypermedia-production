//! Topology repair and differential geometry over retained or authored data.

use std::{
    collections::{BTreeMap, BTreeSet},
    mem::size_of,
    sync::{atomic::Ordering, Arc},
};

use bytemuck::Zeroable;
use qualia_core_db::{
    render::assets::{ImportedMesh, Mesh},
    specialized_libs::computational_geometry::{
        alpha_shape::{self, AlphaEdge, TriangleClass},
        ddg_operators::SparseEntry,
        ddg_production::{self as ddg, OperatorScratch, ResidualRequest, SolverLimits},
        delaunay_3::{delaunay_tetrahedralization_3, required_tetrahedra_3, verify_delaunay_3},
        geometry_workspace::{
            production_budget::{BufferClass, CapacityPlan, ProductionBudget},
            Cancellation,
        },
        natural_neighbour::{self, NnWeight},
        point_cloud_diff_geom as cloud,
        polygon_soup::{self, FaceAdjacencyEntry},
        power_diagram::{regular_triangulation_2d, WeightedSite2d},
        required_edge_slots,
        sos::orient_3d_sos,
        EdgeSlot, HalfEdge, Point2, Point3,
    },
};

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{
    artifact::AttributedMesh, count, fields, integer, list, num, record, required, text, vector,
    Args, Artifact, AssetHost, TypedArtifact, Value,
};

const GRAPH: &str = "urn:qualia:geometry:validation:v1";

impl AssetHost<'_> {
    pub(crate) fn repair_polygon_soup_and_ddg_poisson(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        let op = text(required(args, "op")?)?;
        let parameters = record(required(args, "parameters")?)?;
        let (mut output, source) = match op {
            "repair_polygon_soup" => repair(self, parameters)?,
            "cotangent_poisson" => poisson(self, parameters)?,
            "laplace_natural_neighbour" => (natural(self, parameters)?, None),
            "alpha_shape_2d" => (alpha_2d(self, parameters)?, None),
            "alpha_shape_3d" => (alpha_3d(self, parameters)?, None),
            "power_diagram_2d" => (power(self, parameters)?, None),
            "monge_curvature" | "orient_normals" | "geometric_median" | "enclosing_sphere" => {
                cloud_operation(self, op, parameters)?
            }
            "orient_3d_sos" => {
                fields(parameters, &["points"])?;
                let points = list(required(parameters, "points")?)?;
                let [a, b, c, d] = points else {
                    return Err("SoS orientation requires exactly four points".into());
                };
                let sign = orient_3d_sos(point(a)?, point(b)?, point(c)?, point(d)?);
                (
                    BTreeMap::from([("sign".into(), Value::I64(sign as i64))]),
                    None,
                )
            }
            _ => return Err(format!("Unknown geometry validation operation {op}")),
        };
        let param = crate::living_world_anatomy::species_spec::vibe_value_to_json(&Value::Record(
            parameters.clone(),
        ));
        let mut hash = blake3::Hasher::new();
        hash.update(&serde_json::to_vec(&(op, param)).map_err(|error| error.to_string())?);
        if let Some(source) = source {
            hash.update(source.as_bytes());
        }
        let subject = format!("urn:qualia:geometry:operation:{}", hash.finalize().to_hex());
        let mut graph = ReceiptGraph::new(GRAPH);
        graph.fact(
            &subject,
            "urn:qualia:geometry:operator",
            &format!("urn:qualia:geometry:{op}"),
        )?;
        for (name, value) in &output {
            let spelling = match value {
                Value::F64(value) if value.is_finite() => {
                    format!("urn:qualia:geometry:double:{:016x}", value.to_bits())
                }
                Value::F64(_) => {
                    return Err("Geometry operation returned a non-finite metric".into())
                }
                Value::U64(value) => format!("urn:qualia:geometry:count:{value}"),
                Value::I64(value) => format!("urn:qualia:geometry:integer:{value}"),
                Value::Bool(value) => format!("urn:qualia:geometry:boolean:{value}"),
                _ => continue,
            };
            graph.fact(&subject, &format!("urn:qualia:geometry:{name}"), &spelling)?;
        }
        let bytes = graph.seal(self.limits.resource_bytes.min(4 * 1024 * 1024))?;
        output.insert("certificate".into(), self.encode("q42", bytes)?);
        Ok(Value::Record(output))
    }
}

type Output = (Args, Option<blake3::Hash>);

fn mesh<'a>(
    host: &'a AssetHost<'_>,
    source: &Value,
) -> Result<(&'a Mesh, Option<&'a AttributedMesh>), String> {
    if let Ok(TypedArtifact::AttributedMesh(value)) = host.workspace.typed(source) {
        return Ok((&value.surface.mesh, Some(value)));
    }
    match host.workspace.get(source)? {
        Artifact::Mesh(value) => Ok((value, None)),
        _ => Err("Expected a surface mesh handle".into()),
    }
}

fn validate_mesh(mesh: &Mesh, max_vertices: usize, max_triangles: usize) -> Result<(), String> {
    if mesh.positions.is_empty()
        || mesh.positions.len() > max_vertices
        || mesh.triangles.is_empty()
        || mesh.triangles.len() > max_triangles
        || mesh.positions.iter().flatten().any(|p| !p.is_finite())
        || mesh
            .triangles
            .iter()
            .flatten()
            .any(|&index| index as usize >= mesh.positions.len())
    {
        return Err("Mesh size, coordinates, or triangle indices are invalid".into());
    }
    Ok(())
}

fn mesh_digest(mesh: &Mesh) -> blake3::Hash {
    let mut hash = blake3::Hasher::new();
    hash.update(&(mesh.positions.len() as u64).to_le_bytes());
    for &word in mesh.positions.iter().flatten() {
        hash.update(&word.to_bits().to_le_bytes());
    }
    hash.update(&(mesh.triangles.len() as u64).to_le_bytes());
    for &word in mesh.triangles.iter().flatten() {
        hash.update(&word.to_le_bytes());
    }
    hash.finalize()
}

fn repair(host: &mut AssetHost<'_>, args: &Args) -> Result<Output, String> {
    fields(args, &["mesh", "tolerance_m", "attribute_policy"])?;
    let source = required(args, "mesh")?;
    let tolerance = num(required(args, "tolerance_m")?)?;
    if tolerance <= 0.0 {
        return Err("Repair tolerance must be positive".into());
    }
    let policy = args
        .get("attribute_policy")
        .map(text)
        .transpose()?
        .unwrap_or("preserve");
    if !matches!(policy, "preserve" | "discard") {
        return Err("Unknown attribute policy".into());
    }
    let (input, attributed) = mesh(host, source)?;
    validate_mesh(input, 16_384, 16_384)?;
    if attributed.is_some_and(|m| m.protected_faces.iter().any(|&flag| flag != 0)) {
        return Err("Polygon soup repair cannot revise protected faces".into());
    }
    if policy == "preserve" && attributed.is_some_and(has_attributes) {
        return Err("Polygon soup repair needs explicit attribute_policy=discard to revise attributed geometry".into());
    }
    let (n, faces) = (input.positions.len(), input.triangles.len());
    let slots = required_edge_slots(faces);
    let work = (n * n)
        .saturating_mul(4)
        .saturating_add((faces * faces).saturating_mul(12));
    host.charge(work, n * 64 + faces * 160 + slots * size_of::<EdgeSlot>())?;
    let (input, _) = mesh(host, source)?;
    let digest = mesh_digest(input);
    let flat: Vec<f64> = input
        .positions
        .iter()
        .flatten()
        .map(|&p| f64::from(p))
        .collect();
    let mut remap = vec![0; n];
    let mut merged = vec![0.0; n * 3];
    let mut triangles = vec![[0; 3]; faces];
    let mut original = triangles.clone();
    let mut adjacency = vec![FaceAdjacencyEntry::default(); faces * 3];
    let mut visited = vec![false; faces];
    let mut queue = vec![0; faces];
    let mut half_edges = vec![HalfEdge::default(); faces * 3];
    let mut edge_slots = vec![EdgeSlot::default(); slots];
    let report = polygon_soup::repair_polygon_soup(
        &flat,
        n as u32,
        &input.triangles,
        tolerance,
        &mut remap,
        &mut merged,
        &mut triangles,
        &mut original,
        &mut adjacency,
        &mut visited,
        &mut queue,
        &mut half_edges,
        &mut edge_slots,
    )
    .map_err(|error| format!("{error:?}"))?;
    let topology = report
        .topology
        .ok_or("Repair produced no surviving manifold faces")?;
    triangles.truncate(report.surviving_triangle_count);
    merged.truncate(report.merged_vertex_count as usize * 3);
    let positions: Vec<_> = merged
        .chunks_exact(3)
        .map(|p| [p[0] as f32, p[1] as f32, p[2] as f32])
        .collect();
    let first = *positions.first().ok_or("Repair produced no vertices")?;
    let (mut min, mut max) = (first, first);
    for point in &positions {
        for axis in 0..3 {
            min[axis] = min[axis].min(point[axis]);
            max[axis] = max[axis].max(point[axis]);
        }
    }
    let shape = Mesh {
        positions,
        triangles,
        min,
        max,
    };
    host.charge(0, 0)?;
    let mesh = host.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(AttributedMesh {
        surface: ImportedMesh {
            mesh: shape,
            normals: None,
            tangents: None,
            texture_coordinates_0: None,
            materials: Vec::new(),
            submeshes: Vec::new(),
            texture_dependencies: Vec::new(),
        },
        uv1: Vec::new(),
        vertex_colors: Vec::new(),
        skin_weights: Vec::new(),
        morph_targets: Arc::from([]),
        protected_faces: Vec::new(),
    })))?;
    Ok((
        BTreeMap::from([
            ("mesh".into(), mesh),
            (
                "vertices".into(),
                Value::U64(u64::from(topology.vertex_count)),
            ),
            (
                "triangles".into(),
                Value::U64(report.surviving_triangle_count as u64),
            ),
            (
                "flipped_faces".into(),
                Value::U64(report.flipped_face_count as u64),
            ),
            (
                "components".into(),
                Value::U64(report.oriented_component_count as u64),
            ),
            (
                "boundary_half_edges".into(),
                Value::U64(u64::from(topology.boundary_half_edges)),
            ),
        ]),
        Some(digest),
    ))
}

fn has_attributes(mesh: &AttributedMesh) -> bool {
    let surface = &mesh.surface;
    surface.normals.is_some()
        || surface.tangents.is_some()
        || surface.texture_coordinates_0.is_some()
        || !surface.materials.is_empty()
        || !surface.submeshes.is_empty()
        || !surface.texture_dependencies.is_empty()
        || !mesh.uv1.is_empty()
        || !mesh.vertex_colors.is_empty()
        || !mesh.skin_weights.is_empty()
        || !mesh.morph_targets.is_empty()
}

fn poisson(host: &mut AssetHost<'_>, args: &Args) -> Result<Output, String> {
    fields(args, &["mesh", "rhs", "fixed", "iterations"])?;
    let source = required(args, "mesh")?;
    let limits = SolverLimits::default();
    let (shape, _) = mesh(host, source)?;
    validate_mesh(
        shape,
        limits.effective_vertex_cap(),
        limits.effective_triangle_cap(),
    )?;
    let n = shape.positions.len();
    let triangles = shape.triangles.len();
    if triangles * 9 > limits.max_nnz {
        return Err("Cotangent operator exceeds sparse storage cap".into());
    }
    let iterations = count(
        args,
        "iterations",
        256,
        limits.effective_jacobi_cap() as usize,
    )?;
    let raw_rhs = list(required(args, "rhs")?)?;
    let pins = list(required(args, "fixed")?)?;
    if raw_rhs.len() != n || pins.is_empty() || pins.len() >= n {
        return Err(
            "Poisson requires one load per vertex and at least one free and one fixed vertex"
                .into(),
        );
    }
    let work = (n * n)
        .saturating_mul(triangles + 1)
        .saturating_add(((iterations + 16) * n).saturating_mul(triangles * 9 + 1));
    let bytes = n * 128 + triangles * 256;
    host.charge(work, bytes)?;
    let (shape, _) = mesh(host, source)?;
    let digest = mesh_digest(shape);
    let points: Vec<_> = shape
        .positions
        .iter()
        .map(|p| Point3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2])))
        .collect();
    let rhs = raw_rhs.iter().map(num).collect::<Result<Vec<_>, _>>()?;
    let mut fixed = vec![false; n];
    let mut solution = vec![0.0; n];
    for pin in pins {
        let pin = record(pin)?;
        fields(pin, &["vertex", "value"])?;
        let index = integer(required(pin, "vertex")?, "vertex")?;
        if index >= n || fixed[index] {
            return Err("Invalid or repeated fixed Poisson vertex".into());
        }
        fixed[index] = true;
        solution[index] = num(required(pin, "value")?)?;
    }
    let cancel = Cancellation::new();
    if host.cancel.load(Ordering::Acquire) {
        cancel.cancel();
    }
    let mut plan = CapacityPlan::default();
    plan.add_bytes(BufferClass::Scratch, bytes)
        .map_err(|error| error.to_string())?;
    let mut budget =
        ProductionBudget::admit(plan, host.limits.total_resource_bytes, work as u64, &cancel)
            .map_err(|error| error.to_string())?;
    let mut laplacian = vec![SparseEntry::zeroed(); triangles * 9];
    let mut mass = vec![0.0; n];
    let mut residual = vec![0.0; n];
    let conventions = ddg::default_conventions_for_fixed(&fixed);
    let matrix = ddg::assemble_cotangent_admitted(
        &points,
        &shape.triangles,
        OperatorScratch {
            laplacian: &mut laplacian,
            mass: &mut mass,
            residual: &mut residual,
        },
        conventions,
        limits,
        &mut budget,
    )
    .map_err(|error| format!("{error:?}"))?;
    let report = ddg::solve_poisson_instrumented(
        n,
        &laplacian[..matrix.nnz],
        &rhs,
        &fixed,
        &mut solution,
        &mut residual,
        iterations as u32,
        ResidualRequest::inf_norm(),
        conventions,
        limits,
        &mut budget,
    )
    .map_err(|error| format!("{error:?}"))?;
    host.charge(0, 0)?;
    if solution.iter().any(|value| !value.is_finite()) {
        return Err("Non-finite Poisson solution".into());
    }
    Ok((
        BTreeMap::from([
            ("values".into(), floats(solution)),
            (
                "converged".into(),
                Value::Bool(report.scale_aware.status == ddg::ConvergenceStatus::Converged),
            ),
            (
                "iterations".into(),
                Value::U64(u64::from(report.jacobi.iterations)),
            ),
            (
                "residual_inf".into(),
                Value::F64(report.algebraic_residual.value),
            ),
            (
                "condition_estimate".into(),
                Value::F64(report.condition_estimate),
            ),
        ]),
        Some(digest),
    ))
}

fn natural(host: &mut AssetHost<'_>, args: &Args) -> Result<Args, String> {
    fields(args, &["sites", "query"])?;
    let sites = list(required(args, "sites")?)?;
    if !(3..=512).contains(&sites.len()) {
        return Err("Natural neighbours requires 3–512 sites".into());
    }
    host.charge(sites.len().pow(2) * 16, sites.len() * 64)?;
    let point = |value| {
        let [x, y] = vector(value)?;
        Ok::<_, String>(Point2::new(x, y))
    };
    let sites = sites.iter().map(point).collect::<Result<Vec<_>, _>>()?;
    let query = point(required(args, "query")?)?;
    let mut weights = vec![
        NnWeight {
            site: 0,
            weight: 0.0
        };
        sites.len()
    ];
    let n = natural_neighbour::laplace_coordinates(&sites, query, &mut weights)
        .map_err(|error| format!("{error:?}"))?;
    let weights = &weights[..n];
    if !natural_neighbour::verify_partition_of_unity(weights)
        || !natural_neighbour::verify_non_negative(weights)
    {
        return Err("Laplace natural-neighbour weights violate their invariants".into());
    }
    let records = weights
        .iter()
        .map(|weight| {
            Value::Record(BTreeMap::from([
                ("site".into(), Value::U64(u64::from(weight.site))),
                ("weight".into(), Value::F64(weight.weight)),
            ]))
        })
        .collect();
    Ok(BTreeMap::from([
        ("weights".into(), Value::List(records)),
        ("neighbour_count".into(), Value::U64(n as u64)),
    ]))
}

fn cloud_operation(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Output, String> {
    let input = list(required(args, "points")?)?;
    let limit = match op {
        "enclosing_sphere" => 64,
        "orient_normals" => 256,
        _ => 16_384,
    };
    if input.is_empty() || input.len() > limit {
        return Err("Point count exceeds the kernel budget".into());
    }
    let points = input.iter().map(point).collect::<Result<Vec<_>, _>>()?;
    let n = points.len();
    let output = match op {
        "geometric_median" => {
            fields(args, &["points", "iterations", "tolerance"])?;
            let iters = count(args, "iterations", 64, 256)?;
            let tolerance = num(required(args, "tolerance")?)?;
            if tolerance <= 0.0 {
                return Err("Median tolerance must be positive".into());
            }
            host.charge(n * iters * 16, n * 24)?;
            let median = cloud::weiszfeld_geometric_median_3d(&points, iters, tolerance)
                .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([("median".into(), point_value(median)?)])
        }
        "enclosing_sphere" => {
            fields(args, &["points"])?;
            host.charge(n.pow(4) * 8, n * 24)?;
            let sphere = cloud::smallest_enclosing_sphere_3d(&points)
                .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([
                ("center".into(), point_value(sphere.center)?),
                ("radius".into(), Value::F64(sphere.radius)),
                (
                    "support".into(),
                    Value::List(
                        sphere.support[..usize::from(sphere.support_count)]
                            .iter()
                            .map(|&index| Value::U64(u64::from(index)))
                            .collect(),
                    ),
                ),
            ])
        }
        "monge_curvature" => {
            fields(args, &["points", "center", "neighbours"])?;
            let center = integer(required(args, "center")?, "center")?;
            let raw = list(required(args, "neighbours")?)?;
            if center >= n || !(3..=n.min(4096)).contains(&raw.len()) {
                return Err("Invalid Monge neighbourhood".into());
            }
            let neighbours: BTreeSet<_> = raw
                .iter()
                .map(|value| integer(value, "neighbours"))
                .collect::<Result<_, _>>()?;
            if neighbours.len() != raw.len() || neighbours.iter().any(|&i| i == center || i >= n) {
                return Err("Monge neighbours must be distinct valid non-center indices".into());
            }
            host.charge(raw.len() * 32, n * 32)?;
            let neighbours: Vec<_> = neighbours.into_iter().collect();
            let report = cloud::monge_jet_principal_curvatures_3d(&points, center, &neighbours)
                .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([
                ("normal".into(), point_value(report.normal)?),
                ("max_curvature".into(), Value::F64(report.max_curvature)),
                ("min_curvature".into(), Value::F64(report.min_curvature)),
                ("max_direction".into(), point_value(report.max_direction)?),
                ("min_direction".into(), point_value(report.min_direction)?),
            ])
        }
        "orient_normals" => {
            fields(args, &["points", "normals", "seed"])?;
            let seed = integer(required(args, "seed")?, "seed")?;
            let raw = list(required(args, "normals")?)?;
            if seed >= n || raw.len() != n {
                return Err("Expected one normal per point and a valid seed".into());
            }
            let mut normals = raw.iter().map(point).collect::<Result<Vec<_>, _>>()?;
            if normals
                .iter()
                .any(|p| ((p.x * p.x + p.y * p.y + p.z * p.z) - 1.0).abs() > 1e-6)
            {
                return Err("Point cloud normals must be unit length".into());
            }
            host.charge(n.pow(3) * 16, n * 64)?;
            cloud::orient_normals_riemannian_mst(
                &points,
                &mut normals,
                seed,
                n.saturating_sub(1).max(1),
            )
            .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([(
                "normals".into(),
                Value::List(
                    normals
                        .into_iter()
                        .map(point_value)
                        .collect::<Result<_, _>>()?,
                ),
            )])
        }
        _ => return Err("Unknown point-cloud operator".into()),
    };
    Ok((output, None))
}

fn point(value: &Value) -> Result<Point3, String> {
    let [x, y, z] = vector(value)?;
    if [x, y, z].into_iter().any(|v: f64| v.abs() > 1e12) {
        return Err("Point exceeds the numerical range".into());
    }
    Ok(Point3::new(x, y, z))
}

fn point_value(point: Point3) -> Result<Value, String> {
    let coordinates = [point.x, point.y, point.z];
    if coordinates.iter().any(|value| !value.is_finite()) {
        return Err("Point result is non-finite".into());
    }
    Ok(floats(coordinates))
}

fn bounded_alpha(args: &Args) -> Result<f64, String> {
    let alpha = num(required(args, "alpha_m")?)?;
    if alpha <= 0.0 || alpha > 1e12 {
        return Err("Alpha radius must be positive and at most 10¹² metres".into());
    }
    Ok(alpha)
}

fn planar(value: &Value) -> Result<Point2, String> {
    let [x, y] = vector::<2>(value)?;
    if x.abs() > 1e12 || y.abs() > 1e12 {
        return Err("Planar point exceeds the numerical range".into());
    }
    Ok(Point2::new(x, y))
}

fn alpha_2d(host: &mut AssetHost<'_>, args: &Args) -> Result<Args, String> {
    fields(args, &["points", "alpha_m"])?;
    let alpha = bounded_alpha(args)?;
    let rows = list(required(args, "points")?)?;
    let n = rows.len();
    if !(3..=512).contains(&n) {
        return Err("Planar alpha shapes require 3–512 points".into());
    }
    let limit = alpha_shape::max_triangles(n);
    host.charge(n * n * 32, n * 24 + limit * 64)?;
    let points = rows.iter().map(planar).collect::<Result<Vec<_>, _>>()?;
    let mut scratch = vec![0; n];
    let mut triangles = vec![[0; 3]; limit];
    let mut classes = vec![TriangleClass::Exterior; limit];
    let mut edges = vec![AlphaEdge::default(); limit * 3];
    let (triangle_count, edge_count, report) = alpha_shape::alpha_shape_2d(
        &points,
        alpha,
        &mut scratch,
        &mut triangles,
        &mut classes,
        &mut edges,
    )
    .map_err(|error| error.to_string())?;
    let triangles = triangles[..triangle_count]
        .iter()
        .zip(&classes)
        .filter_map(|(triangle, class)| {
            (*class != TriangleClass::Exterior).then(|| indices(*triangle))
        })
        .collect();
    let edges = edges[..edge_count]
        .iter()
        .filter_map(|edge| {
            (edge.class == 1 || edge.class == 2).then(|| {
                Value::Record(BTreeMap::from([
                    ("vertices".into(), indices([edge.i, edge.j])),
                    ("singular".into(), Value::Bool(edge.class == 2)),
                ]))
            })
        })
        .collect();
    Ok(BTreeMap::from([
        ("triangles".into(), Value::List(triangles)),
        ("edges".into(), Value::List(edges)),
        (
            "boundary_edges".into(),
            Value::U64(report.boundary_edges as u64),
        ),
        (
            "singular_edges".into(),
            Value::U64(report.singular_edges as u64),
        ),
    ]))
}

fn alpha_3d(host: &mut AssetHost<'_>, args: &Args) -> Result<Args, String> {
    fields(args, &["points", "alpha_m"])?;
    let alpha = bounded_alpha(args)?;
    let rows = list(required(args, "points")?)?;
    let n = rows.len();
    if !(4..=128).contains(&n) {
        return Err("Spatial alpha shapes require 4–128 points".into());
    }
    let limit = required_tetrahedra_3(n);
    host.charge(n * limit * 256, n * 24 + limit * 80)?;
    let points = rows.iter().map(point).collect::<Result<Vec<_>, _>>()?;
    let mut tetrahedra = vec![[0; 4]; limit];
    let count = delaunay_tetrahedralization_3(&points, &mut tetrahedra)
        .map_err(|error| format!("Delaunay tetrahedralization: {error:?}"))?;
    tetrahedra.truncate(count);
    if !verify_delaunay_3(&points, &tetrahedra) {
        return Err("Could not verify the alpha-shape Delaunay tetrahedra".into());
    }
    let mut classes = vec![false; count];
    let mut boundary = vec![[0; 3]; count * 4];
    let (_, surface_count) =
        alpha_shape::alpha_shape_3d(&points, alpha, &tetrahedra, &mut classes, &mut boundary)
            .map_err(|error| error.to_string())?;
    let interior_count = classes.into_iter().filter(|class| *class).count();
    let surface = boundary
        .into_iter()
        .take(surface_count)
        .map(indices)
        .collect();
    Ok(BTreeMap::from([
        ("surface_triangles".into(), Value::List(surface)),
        (
            "interior_tetrahedra".into(),
            Value::U64(interior_count as u64),
        ),
        (
            "surface_triangle_count".into(),
            Value::U64(surface_count as u64),
        ),
    ]))
}

fn power(host: &mut AssetHost<'_>, args: &Args) -> Result<Args, String> {
    fields(args, &["sites"])?;
    let rows = list(required(args, "sites")?)?;
    let n = rows.len();
    if !(3..=512).contains(&n) {
        return Err("Power diagrams require 3–512 weighted sites".into());
    }
    host.charge(n * n * 128, n * 1024)?;
    let sites = rows
        .iter()
        .map(|row| {
            let site = record(row)?;
            fields(site, &["point_m", "weight_m2"])?;
            let pos = planar(required(site, "point_m")?)?;
            let weight = num(required(site, "weight_m2")?)?;
            if !(0.0..=1e24).contains(&weight) {
                return Err(
                    "Power weight must be nonnegative and at most 10²⁴ square metres".into(),
                );
            }
            Ok(WeightedSite2d { pos, weight })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let mut triangles = vec![[0; 3]; n * 2];
    let mut vertices = vec![Point2::new(0.0, 0.0); n * 2];
    let count = regular_triangulation_2d(&sites, &mut triangles, &mut vertices)
        .map_err(|error| format!("Power diagram: {error:?}"))?;
    if vertices[..count]
        .iter()
        .any(|point| !point.x.is_finite() || !point.y.is_finite())
    {
        return Err("Power diagram produced a non-finite dual vertex".into());
    }
    Ok(BTreeMap::from([
        (
            "regular_triangles".into(),
            Value::List(triangles.into_iter().take(count).map(indices).collect()),
        ),
        (
            "dual_vertices_m".into(),
            Value::List(
                vertices
                    .into_iter()
                    .take(count)
                    .map(|point| floats([point.x, point.y]))
                    .collect(),
            ),
        ),
        ("dual_count".into(), Value::U64(count as u64)),
    ]))
}

fn indices<const N: usize>(indices: [u32; N]) -> Value {
    Value::List(
        indices
            .into_iter()
            .map(|index| Value::U64(u64::from(index)))
            .collect(),
    )
}

fn floats(values: impl IntoIterator<Item = f64>) -> Value {
    Value::List(values.into_iter().map(Value::F64).collect())
}
