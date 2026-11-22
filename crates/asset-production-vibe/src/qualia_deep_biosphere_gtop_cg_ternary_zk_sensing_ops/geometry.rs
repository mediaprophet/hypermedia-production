//! Advanced geometry adapters with explicit source and resource bounds.

use std::{collections::{BTreeMap, BTreeSet}, sync::Arc};

use qualia_core_db::bundle::semantic_model::resource_identity_meta;
use qualia_core_db::q_hash;
use qualia_core_db::render::assets::{ImportedMesh, Mesh};
use qualia_core_db::specialized_libs::computational_geometry::{
    advancing_front::{FrontOptions2d, FrontOptions3d, advancing_front_tetrahedralise_3d, advancing_front_triangulate_2d},
    arrangement_3d::{build_arrangement_3d, validate_arrangement},
    corefine_3d::{Mesh3D, corefine_3d},
    csg_section::{CSG_TYPE_EXPRESSION, CsgSection, encode_csg_section},
    fem_certificate::{BoundaryMarker, FEM_CERT_ENCODED_LEN, MaterialRegion, build_fem_certificate, encode_fem_certificate},
    kirkpatrick::KirkpatrickHierarchy,
    mixed_cell_topology::MixedCell,
    nary_boolean::{BoolExpr, MeshInput, nary_boolean},
    quad_octree_mesh::{OctMeshOptions, build_octtree, octtree_to_hexahedra, octtree_to_tetrahedra},
    simplify_snap::snap_round_3d,
    tet_quality_improve::{TetImproveObjective, TetImproveOptions, improve_tet_mesh, verify_improvement},
    Point2, Point3,
};

use crate::{Args, AssetHost, TypedArtifact, Value, boolean, count, fields, integer, list, number, record, required, text, vector};
use crate::artifact::AttributedMesh;

const MAX_SURFACE_VERTICES: usize = 65_536;
const MAX_SURFACE_FACES: usize = 32_768;
const MAX_COREFINE_PAIRS: usize = 65_536;
const MAX_VOLUME_CELLS: usize = 16_384;

impl AssetHost<'_> {
    pub(crate) fn advanced_cg_arrangements_and_meshing(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["kernel", "source", "parameters"])?;
        let source = required(args, "source")?;
        let parameters = record(required(args, "parameters")?)?;
        match text(required(args, "kernel")?)? {
            "advancing_front_2d" => front_2d(self, source, parameters),
            "advancing_front_3d" => front_3d(self, source, parameters),
            "corefine_weiler_3d" => corefine(self, source, parameters),
            "snap_round_3d" => snap(self, source, parameters),
            "balanced_octree_hex_tet" => octree(self, source, parameters),
            "tet_quality_sliver_exude" => improve(self, source, parameters),
            "nary_boolean_csg_section" => csg(self, source, parameters),
            "kirkpatrick_point_location" => locate(self, source, parameters),
            "fem_boundary_cert" => fem_cert(self, source, parameters),
            kernel => Err(format!("Unsupported advanced geometry kernel {kernel}")),
        }
    }
}

fn snap(host: &mut AssetHost<'_>, source: &Value, args: &Args) -> Result<Value, String> {
    fields(args, &["epsilon", "attribute_policy"])?;
    let epsilon = positive(args, "epsilon")?;
    let mesh = attributed(host, source)?;
    let rows = mesh.surface.mesh.positions.len() + mesh.surface.mesh.triangles.len();
    host.charge(rows * 16, rows * 64)?;
    let mesh = attributed(host, source)?;
    let policy = args.get("attribute_policy").map(text).transpose()?.unwrap_or("preserve");
    if !matches!(policy, "preserve" | "discard") { return Err("attribute_policy must be preserve or discard".into()); }
    if mesh.surface.mesh.triangles.iter().flatten().any(|&i| i as usize >= mesh.surface.mesh.positions.len()) {
        return Err("Snap-round input face references an invalid vertex".into());
    }
    let (rounded, _) = snap_round_3d(&points(mesh), &mesh.surface.mesh.triangles, epsilon)
        .map_err(|error| format!("3D snap rounding: {error:?}"))?;
    let positions: Vec<_> = rounded.iter().map(|p| [p.x as f32, p.y as f32, p.z as f32]).collect();
    if positions.iter().flatten().any(|p| !p.is_finite()) { return Err("Non-finite rounded position".into()); }
    let changed: Vec<_> = positions.iter().zip(&mesh.surface.mesh.positions).map(|(after, before)| after != before).collect();
    for (mask, face) in mesh.protected_faces.iter().zip(&mesh.surface.mesh.triangles) {
        if *mask != 0 && face.iter().any(|&v| changed[v as usize]) {
            return Err("Snap rounding would displace a protected face".into());
        }
    }
    let normal = |p: &[[f32; 3]], f: &[u32; 3]| {
        let [a, b, c] = f.map(|id| p[id as usize].map(f64::from));
        let [u, v] = [b, c].map(|v| [v[0] - a[0], v[1] - a[1], v[2] - a[2]]);
        [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]]
    };
    for triangle in &mesh.surface.mesh.triangles {
        let dot = normal(&positions, triangle).into_iter().zip(normal(&mesh.surface.mesh.positions, triangle))
            .map(|(a, b)| a * b).sum::<f64>();
        if !dot.is_finite() || dot <= 0.0 { return Err("Snap rounding flips or degenerates a face".into()); }
    }
    let changed_count = changed.iter().filter(|&&changed| changed).count();
    let result = if changed_count == 0 { source.clone() } else {
        if policy == "preserve" && carries_attributes(mesh) {
            return Err("Snap rounding requires explicit attribute_policy=discard for this geometry revision".into());
        }
        let triangles = mesh.surface.mesh.triangles.clone();
        admit_plain_mesh(host, positions.into_iter().map(|p| p.map(f64::from)), triangles)?
    };
    Ok(Value::Record(Args::from([("mesh".into(), result), ("changed_count".into(), Value::U64(changed_count as u64))])))
}

fn octree(host: &mut AssetHost<'_>, source: &Value, args: &Args) -> Result<Value, String> {
    fields(args, &["target_size", "max_level", "max_cells", "uniform", "element", "provenance_iri"])?;
    let h = positive(args, "target_size")?;
    let max_level = count(args, "max_level", 4, 8)?;
    let max_cells = count(args, "max_cells", 4096, MAX_VOLUME_CELLS)?;
    let uniform = boolean(args, "uniform", false)?;
    let element = text(required(args, "element")?)?;
    if !matches!(element, "hex" | "tet") { return Err("Octree element must be hex or tet".into()); }
    let mesh = attributed(host, source)?;
    let shape = &mesh.surface.mesh;
    host.charge(max_cells.saturating_mul(shape.triangles.len().max(max_cells)), (max_cells + shape.triangles.len()).saturating_mul(512))?;
    let mesh = attributed(host, source)?;
    let shape = &mesh.surface.mesh;
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for p in &shape.positions {
        for axis in 0..3 { min[axis] = min[axis].min(f64::from(p[axis])); max[axis] = max[axis].max(f64::from(p[axis])); }
    }
    let center = std::array::from_fn(|axis| 0.5 * (min[axis] + max[axis]));
    let half = (0..3).map(|axis| 0.5 * (max[axis] - min[axis])).fold(0.0, f64::max);
    let mut aabbs = Vec::with_capacity(shape.triangles.len());
    for face in &shape.triangles {
        let [a, b, c] = face.map(|id| shape.positions.get(id as usize));
        let (Some(a), Some(b), Some(c)) = (a, b, c) else { return Err("Octree input face references an invalid vertex".into()) };
        let lo: [f64; 3] = std::array::from_fn(|axis| f64::from(a[axis].min(b[axis]).min(c[axis])));
        let hi: [f64; 3] = std::array::from_fn(|axis| f64::from(a[axis].max(b[axis]).max(c[axis])));
        aabbs.push((lo, hi));
    }
    let tree = build_octtree(center, half, &|leaf| {
        if leaf.half * 2.0 <= h { return false; }
        let center = [leaf.cx, leaf.cy, leaf.cz];
        uniform || aabbs.iter().any(|(lo, hi)| (0..3).all(|axis| {
            center[axis] + leaf.half >= lo[axis] && center[axis] - leaf.half <= hi[axis]
        }))
    }, &OctMeshOptions { max_level: max_level as u8, max_cells, balance_2to1: true })
        .map_err(|error| format!("Balanced octree: {error:?}"))?;
    let leaves = tree.leaves();
    let conforming = leaves.iter().all(|leaf| leaf.level == leaves[0].level);
    if element == "tet" && !conforming { return Err("Graded octree has hanging vertices; a conforming tet output requires uniform refinement".into()); }
    let mut out = Args::from([
        ("surface".into(), source.clone()), ("node_count".into(), Value::U64(tree.node_count() as u64)),
        ("conforming".into(), Value::Bool(conforming)),
    ]);
    if element == "tet" {
        let (points, tets) = octtree_to_tetrahedra(&tree);
        let cells: Vec<_> = tets.iter().map(|t| MixedCell::tetra(t[0], t[1], t[2], t[3], 0)).collect();
        let certificate = certify(host, &points, &cells, &[], &[], provenance(args)?)?;
        out.insert("fem_certificate".into(), certificate);
        out.insert("volume_vertices".into(), Value::List(points.into_iter().map(|p| xyz([p.x, p.y, p.z])).collect()));
        out.insert("tetrahedra".into(), Value::List(tets.iter().map(|t| indices(t)).collect()));
    } else {
        let (points, hexes) = octtree_to_hexahedra(&tree);
        out.insert("volume_vertices".into(), Value::List(points.into_iter().map(|p| xyz([p.x, p.y, p.z])).collect()));
        out.insert("hexahedra".into(), Value::List(hexes.iter().map(|h| indices(h)).collect()));
    }
    Ok(Value::Record(out))
}

fn tetrahedron(value: &Value) -> Result<[u32; 4], String> {
    let [a, b, c, d] = list(value)? else { return Err("Expected four tetrahedron indices".into()) };
    let mut indices = [0; 4];
    for (index, value) in indices.iter_mut().zip([a, b, c, d]) {
        *index = u32::try_from(integer(value, "indices")?).map_err(|_| "Invalid tetrahedron index")?;
    }
    Ok(indices)
}

fn improve(host: &mut AssetHost<'_>, source: &Value, args: &Args) -> Result<Value, String> {
    fields(args, &["tetrahedra", "fixed_vertices", "max_passes", "sliver_min_dihedral_deg", "provenance_iri"])?;
    let raw = list(required(args, "tetrahedra")?)?;
    if raw.is_empty() || raw.len() > 4096 { return Err("Tet quality improvement input count exceeded".into()); }
    let passes = count(args, "max_passes", 4, 20)?;
    let min_angle = number(args, "sliver_min_dihedral_deg", 15.0)?;
    if !(0.0..60.0).contains(&min_angle) { return Err("Invalid sliver dihedral threshold in degrees".into()); }
    host.charge(raw.len().saturating_pow(2).saturating_mul(passes), raw.len().saturating_mul(1024))?;
    let mesh = attributed(host, source)?;
    let vertices = points(mesh);
    let tetrahedra = raw.iter().map(tetrahedron).collect::<Result<Vec<_>, _>>()?;
    let mut fixed = BTreeSet::new();
    if let Some(value) = args.get("fixed_vertices") {
        let fixed_rows = list(value)?;
        if fixed_rows.len() > vertices.len() { return Err("Too many fixed tetrahedral vertices".into()); }
        for value in fixed_rows {
            let index = integer(value, "fixed_vertices")?;
            if index >= vertices.len() { return Err("Invalid fixed tetrahedral vertex index".into()); }
            fixed.insert(index as u32);
        }
    }
    for (mask, face) in mesh.protected_faces.iter().zip(&mesh.surface.mesh.triangles) {
        if *mask != 0 { fixed.extend(face); }
    }
    let fixed: Vec<_> = fixed.into_iter().collect();
    let result = improve_tet_mesh(&vertices, &tetrahedra, Some(&fixed), &TetImproveOptions {
        max_passes: passes as u32, max_steiner: 128, sliver_min_dihedral_deg: min_angle, ..Default::default()
    }).map_err(|error| format!("Tet quality improvement: {error:?}"))?;
    if !verify_improvement(&result.vertices, &result.tets, TetImproveObjective::MinDihedral,
        result.stats_before.global_min_dihedral).map_err(|error| format!("Tet quality regression: {error:?}"))?
    {
        return Err("Tet improvement did not preserve the input minimum quality".into());
    }
    if fixed.iter().any(|&i| result.vertices.get(i as usize) != vertices.get(i as usize)) {
        return Err("Tet improvement would displace a protected or fixed vertex".into());
    }
    let cells: Vec<_> = result.tets.iter().map(|t| MixedCell::tetra(t[0], t[1], t[2], t[3], 0)).collect();
    let certificate = certify(host, &result.vertices, &cells, &[], &[], provenance(args)?)?;
    Ok(Value::Record(Args::from([
        ("surface".into(), source.clone()), ("fem_certificate".into(), certificate),
        ("volume_vertices".into(), Value::List(result.vertices.into_iter().map(|p| xyz([p.x, p.y, p.z])).collect())),
        ("tetrahedra".into(), Value::List(result.tets.iter().map(|t| indices(t)).collect())),
        ("minimum_dihedral_before_rad".into(), Value::F64(result.stats_before.global_min_dihedral)),
        ("minimum_dihedral_after_rad".into(), Value::F64(result.stats_after.global_min_dihedral)),
        ("exudes_applied".into(), Value::U64(u64::from(result.exudes_applied))),
    ])))
}

fn csg(host: &mut AssetHost<'_>, source: &Value, args: &Args) -> Result<Value, String> {
    fields(args, &["operation", "attribute_policy"])?;
    let operands = list(source)?;
    if !(2..=8).contains(&operands.len()) { return Err("N-ary CSG requires 2..=8 mesh operands".into()); }
    let policy = args.get("attribute_policy").map(text).transpose()?.unwrap_or("preserve");
    if !matches!(policy, "preserve" | "discard") { return Err("attribute_policy must be preserve or discard".into()); }
    let expression = match text(required(args, "operation")?)? {
        "union" => BoolExpr::Union((0..operands.len()).map(BoolExpr::Operand).collect()),
        "intersection" => BoolExpr::Intersection((0..operands.len()).map(BoolExpr::Operand).collect()),
        "difference" => BoolExpr::Difference(Box::new(BoolExpr::Operand(0)), (1..operands.len()).map(BoolExpr::Operand).collect()),
        "xor" if operands.len() == 2 => BoolExpr::xor(BoolExpr::Operand(0), BoolExpr::Operand(1)),
        _ => return Err("Invalid N-ary solid operation or operand count".into()),
    };
    let mut face_count: usize = 0;
    for operand in operands {
        let source = attributed(host, operand)?;
        if policy == "preserve" && carries_attributes(source) {
            return Err("N-ary CSG requires attribute_policy=discard for this geometry revision".into());
        }
        face_count += source.surface.mesh.triangles.len();
    }
    if face_count.saturating_pow(2) > 4 * MAX_COREFINE_PAIRS { return Err("N-ary CSG candidate bound exceeded".into()); }
    host.charge(face_count.saturating_pow(2).saturating_mul(64), face_count.saturating_mul(8192))?;
    let meshes: Vec<_> = operands.iter().map(|value| {
        let mesh = attributed(host, value)?;
        Ok::<_, String>(MeshInput { vertices: points(mesh), triangles: mesh.surface.mesh.triangles.clone() })
    }).collect::<Result<_, _>>()?;
    let (vertices, triangles) = nary_boolean(&meshes, &expression).map_err(|error| format!("N-ary CSG: {error:?}"))?;
    let section = CsgSection {
        section_type: CSG_TYPE_EXPRESSION, flags: 0, vertices, triangles, region_labels: Vec::new(), expr_tree: Some(expression),
    };
    let bytes = encode_csg_section(&section);
    let mesh = if section.triangles.is_empty() { Value::Null } else {
        admit_plain_mesh(host, section.vertices.iter().map(|p| [p.x, p.y, p.z]), section.triangles)?
    };
    let encoded = host.admit_encoded("csg", bytes)?;
    Ok(Value::Record(Args::from([("mesh".into(), mesh), ("csg".into(), encoded)])))
}

fn locate(host: &mut AssetHost<'_>, source: &Value, args: &Args) -> Result<Value, String> {
    fields(args, &["queries"])?;
    let queries = list(required(args, "queries")?)?;
    if queries.is_empty() || queries.len() > 4096 { return Err("Kirkpatrick query count outside budget".into()); }
    let mesh = attributed(host, source)?;
    let counts = mesh.surface.mesh.positions.len() + mesh.surface.mesh.triangles.len();
    host.charge(counts.saturating_pow(2).saturating_add(queries.len() * 32), counts.saturating_mul(256))?;
    let mesh = attributed(host, source)?;
    let shape = &mesh.surface.mesh;
    if shape.positions.iter().any(|p| p[2] != shape.positions[0][2]) { return Err("Kirkpatrick source must be an XY triangulation".into()); }
    let points: Vec<_> = shape.positions.iter().map(|p| Point2::new(f64::from(p[0]), f64::from(p[1]))).collect();
    let faces: Vec<_> = shape.triangles.iter().map(|row| row.map(|index| index as usize)).collect();
    if faces.iter().flatten().any(|&index| index >= points.len()) || faces.iter().any(|f| f[0] == f[1] || f[0] == f[2] || f[1] == f[2]) {
        return Err("Invalid Kirkpatrick input triangulation".into());
    }
    let labels: Vec<_> = (0..faces.len()).collect();
    let finder = KirkpatrickHierarchy::build(&points, &faces, &labels).map_err(|error| format!("Kirkpatrick build: {error:?}"))?;
    let values: Vec<Value> = queries.iter().map(|value| {
        let [x, y] = vector(value)?;
        let label = finder.locate(Point2::new(x, y)).map_err(|error| format!("Kirkpatrick locate: {error:?}"))?;
        Ok::<_, String>(label.map_or(Value::Null, |label| Value::U64(label as u64)))
    }).collect::<Result<_, _>>()?;
    Ok(Value::Record(Args::from([("faces".into(), Value::List(values))])))
}

fn attributed<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a AttributedMesh, String> {
    let TypedArtifact::AttributedMesh(mesh) = host.workspace.typed(value)? else {
        return Err("Advanced 3D geometry requires an AttributedMesh handle".into());
    };
    let shape = &mesh.surface.mesh;
    if shape.positions.is_empty() || shape.positions.len() > MAX_SURFACE_VERTICES
        || shape.triangles.is_empty() || shape.triangles.len() > MAX_SURFACE_FACES
        || shape.positions.iter().flatten().any(|p| !p.is_finite())
    {
        return Err("Invalid source mesh size or non-finite geometry".into());
    }
    Ok(mesh)
}

fn points(mesh: &AttributedMesh) -> Vec<Point3> {
    mesh.surface.mesh.positions.iter().map(|&vertex| {
        let [x, y, z] = vertex.map(f64::from);
        Point3::new(x, y, z)
    }).collect()
}

fn positive(args: &Args, key: &str) -> Result<f64, String> {
    let value = number(args, key, f64::NAN)?;
    if !value.is_finite() || value <= 0.0 { return Err(format!("{key} must be finite and positive")); }
    Ok(value)
}

fn front_2d(host: &mut AssetHost<'_>, source: &Value, args: &Args) -> Result<Value, String> {
    fields(args, &["target_size", "max_iterations"])?;
    let h = positive(args, "target_size")?;
    let iterations = count(args, "max_iterations", 512, 4096)?;
    let points = host.profile(source)?;
    if points.len() < 3 || points.len() > 4096 || points.iter().flatten().any(|value| !value.is_finite()) {
        return Err("Invalid advancing-front profile".into());
    }
    let mut boundary: Vec<_> = points.iter().map(|p| Point2::new(p[0], p[1])).collect();
    if boundary.first() != boundary.last() { boundary.push(boundary[0]); }
    host.charge((boundary.len() + iterations).saturating_pow(2), (boundary.len() + iterations).saturating_mul(128))?;
    let result = advancing_front_triangulate_2d(&boundary, &[], &|_| h, &FrontOptions2d {
        max_iterations: iterations as u32, ..Default::default()
    }).map_err(|error| format!("2D advancing front: {error:?}"))?;
    let positions = result.vertices.iter().map(|p| [p.x, p.y, 0.0]);
    let mesh = admit_plain_mesh(host, positions, result.triangles)?;
    Ok(Value::Record(Args::from([
        ("mesh".into(), mesh), ("iterations".into(), Value::U64(u64::from(result.iterations))),
        ("front_collapses".into(), Value::U64(u64::from(result.front_collapses))),
    ])))
}

fn front_3d(host: &mut AssetHost<'_>, source: &Value, args: &Args) -> Result<Value, String> {
    fields(args, &["target_size", "max_iterations", "provenance_iri"])?;
    let h = positive(args, "target_size")?;
    let iterations = count(args, "max_iterations", 512, 4096)?;
    let rows = attributed(host, source)?.surface.mesh.triangles.len() + iterations;
    host.charge(rows.saturating_pow(2), rows.saturating_mul(256))?;
    let mesh = attributed(host, source)?;
    let result = advancing_front_tetrahedralise_3d(&points(mesh), &mesh.surface.mesh.triangles, &|_| h, &FrontOptions3d {
        max_iterations: iterations as u32, ..Default::default()
    }).map_err(|error| format!("3D advancing front: {error:?}"))?;
    let cells: Vec<_> = result.tetrahedra.iter().map(|tet| MixedCell::tetra(tet[0], tet[1], tet[2], tet[3], 0)).collect();
    let cert = certify(host, &result.vertices, &cells, &[], &[], provenance(args)?)?;
    Ok(Value::Record(Args::from([
        ("surface".into(), source.clone()), ("fem_certificate".into(), cert),
        ("volume_vertices".into(), Value::List(result.vertices.iter().map(|p| xyz([p.x, p.y, p.z])).collect())),
        ("tetrahedra".into(), Value::List(result.tetrahedra.iter().map(|row| indices(row)).collect())),
        ("iterations".into(), Value::U64(u64::from(result.iterations))),
    ])))
}

fn carries_attributes(mesh: &AttributedMesh) -> bool {
    let s = &mesh.surface;
    s.normals.is_some() || s.tangents.is_some() || s.texture_coordinates_0.is_some() || !s.materials.is_empty()
        || !s.texture_dependencies.is_empty() || !s.submeshes.is_empty() || !mesh.uv1.is_empty()
        || !mesh.vertex_colors.is_empty() || !mesh.skin_weights.is_empty() || !mesh.morph_targets.is_empty()
        || !mesh.protected_faces.is_empty()
}

fn corefine(host: &mut AssetHost<'_>, source: &Value, args: &Args) -> Result<Value, String> {
    fields(args, &["other", "attribute_policy"])?;
    let other = required(args, "other")?;
    let left = attributed(host, source)?;
    let right = attributed(host, other)?;
    let policy = args.get("attribute_policy").map(text).transpose()?.unwrap_or("preserve");
    if !matches!(policy, "preserve" | "discard") { return Err("attribute_policy must be preserve or discard".into()); }
    if policy == "preserve" && (carries_attributes(left) || carries_attributes(right)) {
        return Err("Co-refinement requires attribute_policy=discard for a new geometry-only revision".into());
    }
    let pair_count = left.surface.mesh.triangles.len().saturating_mul(right.surface.mesh.triangles.len());
    if pair_count > MAX_COREFINE_PAIRS { return Err("3D co-refinement candidate bound exceeded".into()); }
    let input_rows = left.surface.mesh.positions.len() + right.surface.mesh.positions.len();
    host.charge(pair_count.saturating_mul(192), pair_count.saturating_mul(288).saturating_add(input_rows * 64))?;
    let left = attributed(host, source)?;
    let right = attributed(host, other)?;
    let result = corefine_3d(
        &Mesh3D { vertices: points(left), triangles: left.surface.mesh.triangles.clone() },
        &Mesh3D { vertices: points(right), triangles: right.surface.mesh.triangles.clone() },
    ).map_err(|error| format!("3D co-refinement: {error:?}"))?;
    let (vertices, faces) = weld_exact([&result.mesh_a, &result.mesh_b])?;
    let operations = faces.len().saturating_pow(2).saturating_mul(3);
    host.charge(operations, (vertices.len() + faces.len()).saturating_mul(384))?;
    let arrangement = build_arrangement_3d(&vertices, &faces).map_err(|error| format!("Weiler arrangement: {error:?}"))?;
    validate_arrangement(&arrangement, &faces).map_err(|error| format!("Weiler validation: {error:?}"))?;
    let a = admit_plain_mesh(host, result.mesh_a.vertices.iter().map(|p| [p.x, p.y, p.z]), result.mesh_a.triangles)?;
    let b = admit_plain_mesh(host, result.mesh_b.vertices.iter().map(|p| [p.x, p.y, p.z]), result.mesh_b.triangles)?;
    let combined = admit_plain_mesh(host, vertices.iter().map(|p| [p.x, p.y, p.z]), faces)?;
    let shells = arrangement.shells.iter().map(|shell| Value::Record(Args::from([
        ("faces".into(), indices(&shell.triangles)), ("outward".into(), Value::Bool(shell.outward_facing)),
    ]))).collect();
    let regions = arrangement.regions.iter().map(|region| Value::Record(Args::from([
        ("shells".into(), Value::List(region.shells.iter().map(|&s| Value::U64(s as u64)).collect())),
        ("boundary_faces".into(), indices(&region.boundary_triangles)),
    ]))).collect();
    Ok(Value::Record(Args::from([
        ("refined_a".into(), a), ("refined_b".into(), b), ("arrangement_mesh".into(), combined),
        ("intersecting_pair_count".into(), Value::U64(result.num_intersecting_pairs as u64)),
        ("intersection_point_count".into(), Value::U64(result.num_intersection_points as u64)),
        ("shells".into(), Value::List(shells)), ("regions".into(), Value::List(regions)),
    ])))
}

fn weld_exact(meshes: [&Mesh3D; 2]) -> Result<(Vec<Point3>, Vec<[u32; 3]>), String> {
    let mut locations = BTreeMap::new();
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    for mesh in meshes {
        let mut remap = Vec::with_capacity(mesh.vertices.len());
        for &vertex in &mesh.vertices {
            let p = [vertex.x, vertex.y, vertex.z];
            if !p.iter().all(|v| v.is_finite()) { return Err("Refined vertex is non-finite".into()); }
            let bits = p.map(|v| if v == 0.0 { 0 } else { v.to_bits() });
            let slot = *locations.entry(bits).or_insert_with(|| {
                let next = vertices.len() as u32;
                vertices.push(vertex);
                next
            });
            remap.push(slot);
        }
        for &[a, b, c] in &mesh.triangles {
            let a = *remap.get(a as usize).ok_or("Invalid refined vertex index")?;
            let b = *remap.get(b as usize).ok_or("Invalid refined vertex index")?;
            let c = *remap.get(c as usize).ok_or("Invalid refined vertex index")?;
            if a == b || a == c || b == c { return Err("Co-refinement produced degenerate facets".into()); }
            faces.push([a, b, c]);
        }
    }
    if vertices.len() > MAX_SURFACE_VERTICES || faces.len() > MAX_SURFACE_FACES {
        return Err("Co-refined geometry exceeds its output count bound".into());
    }
    Ok((vertices, faces))
}

fn xyz(v: [f64; 3]) -> Value { Value::List(v.into_iter().map(Value::F64).collect()) }
fn indices(v: &[u32]) -> Value { Value::List(v.iter().map(|&i| Value::U64(u64::from(i))).collect()) }

fn admit_plain_mesh(host: &mut AssetHost<'_>, positions: impl Iterator<Item = [f64; 3]>, triangles: Vec<[u32; 3]>) -> Result<Value, String> {
    let positions: Vec<_> = positions.map(|p| p.map(|v| v as f32)).collect();
    if positions.is_empty() || positions.len() > MAX_SURFACE_VERTICES || triangles.is_empty()
        || triangles.len() > MAX_SURFACE_FACES || positions.iter().flatten().any(|p| !p.is_finite())
        || triangles.iter().flatten().any(|&i| i as usize >= positions.len())
    {
        return Err("Invalid geometry kernel output".into());
    }
    let mut min = positions[0];
    let mut max = positions[0];
    for p in &positions {
        for axis in 0..3 { min[axis] = min[axis].min(p[axis]); max[axis] = max[axis].max(p[axis]); }
    }
    let mesh = ImportedMesh {
        mesh: Mesh { positions, triangles, min, max }, normals: None, tangents: None, texture_coordinates_0: None,
        materials: Vec::new(), texture_dependencies: Vec::new(), submeshes: Vec::new(),
    };
    host.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(AttributedMesh {
        surface: mesh, uv1: Vec::new(), vertex_colors: Vec::new(), skin_weights: Vec::new(),
        morph_targets: Arc::from(Vec::new()), protected_faces: Vec::new(),
    })))
}

fn provenance(args: &Args) -> Result<u64, String> {
    let iri = text(required(args, "provenance_iri")?)?;
    if iri.len() > 4096 { return Err("FEM provenance IRI exceeds its bound".into()); }
    resource_identity_meta(iri).map_err(|error| error.to_string())?;
    Ok(q_hash(iri))
}

fn u32_arg(args: &Args, key: &str) -> Result<u32, String> {
    u32::try_from(integer(required(args, key)?, key)?).map_err(|_| format!("Invalid FEM {key}"))
}

fn fem_cert(host: &mut AssetHost<'_>, source: &Value, args: &Args) -> Result<Value, String> {
    fields(args, &["tetrahedra", "boundary_markers", "material_regions", "provenance_iri"])?;
    let source_mesh = attributed(host, source)?;
    let vertices = points(source_mesh);
    let raw_cells = list(required(args, "tetrahedra")?)?;
    if raw_cells.is_empty() || raw_cells.len() > MAX_VOLUME_CELLS { return Err("FEM volume cell count outside budget".into()); }
    let mut cells = Vec::with_capacity(raw_cells.len());
    for cell in raw_cells {
        let cell = record(cell)?;
        fields(cell, &["indices", "material"])?;
        let indices = list(required(cell, "indices")?)?;
        let [a, b, c, d] = indices else { return Err("FEM tetrahedron requires four integer indices".into()) };
        let index = |value| -> Result<u32, String> {
            u32::try_from(integer(value, "indices")?).map_err(|_| "Invalid FEM vertex index".into())
        };
        let material = u16::try_from(u32_arg(cell, "material")?).map_err(|_| "Invalid FEM material slot")?;
        cells.push(MixedCell::tetra(index(a)?, index(b)?, index(c)?, index(d)?, material));
    }
    let mut markers = Vec::new();
    if let Some(value) = args.get("boundary_markers") {
        let rows = list(value)?;
        if rows.len() > MAX_VOLUME_CELLS * 4 { return Err("FEM boundary marker count exceeded".into()); }
        let mut seen = BTreeSet::new();
        for row in rows {
            let row = record(row)?;
            fields(row, &["entity_id", "marker"])?;
            let entity_id = u32_arg(row, "entity_id")?;
            if !seen.insert(entity_id) { return Err("Duplicate FEM boundary marker".into()); }
            markers.push(BoundaryMarker { entity_id, marker: u32_arg(row, "marker")? });
        }
    }
    let mut regions = Vec::new();
    if let Some(value) = args.get("material_regions") {
        let rows = list(value)?;
        if rows.len() > cells.len() { return Err("FEM material region count exceeded".into()); }
        for row in rows {
            let row = record(row)?;
            fields(row, &["first_cell", "cell_count", "material"])?;
            let region = MaterialRegion {
                first_cell: u32_arg(row, "first_cell")?, cell_count: u32_arg(row, "cell_count")?,
                material: u16::try_from(u32_arg(row, "material")?).map_err(|_| "Invalid FEM material slot")?,
            };
            regions.push(region);
        }
        regions.sort_unstable_by_key(|region| region.first_cell);
        let mut previous = 0;
        for region in &regions {
            let end = region.first_cell.checked_add(region.cell_count).ok_or("FEM material range overflow")?;
            let distinct = region.first_cell >= previous && region.cell_count != 0;
            let matches = cells.get(region.first_cell as usize..end as usize)
                .is_some_and(|cells| cells.iter().all(|cell| cell.material == region.material));
            if !distinct || !matches { return Err("FEM material regions must match disjoint cell ranges".into()); }
            previous = end;
        }
    }
    let certificate = certify(host, &vertices, &cells, &markers, &regions, provenance(args)?)?;
    Ok(Value::Record(Args::from([("surface".into(), source.clone()), ("fem_certificate".into(), certificate)])))
}

fn certify(host: &mut AssetHost<'_>, points: &[Point3], cells: &[MixedCell], markers: &[BoundaryMarker], regions: &[MaterialRegion], hash: u64) -> Result<Value, String> {
    if cells.is_empty() || cells.len() > MAX_VOLUME_CELLS || points.len() > MAX_SURFACE_VERTICES {
        return Err("FEM certificate input outside its volume or vertex bound".into());
    }
    host.charge(cells.len().saturating_mul(512), cells.len().saturating_mul(512))?;
    let mut certificate = build_fem_certificate(points, cells, markers, regions, hash)
        .map_err(|error| format!("FEM certificate: {error:?}"))?;
    let mut maximum = 0.0f64;
    let mut unique_tets = BTreeSet::new();
    let mut incidence = BTreeMap::<[u32; 3], (bool, bool)>::new();
    for cell in cells {
        let ids = [cell.vertices[0], cell.vertices[1], cell.vertices[2], cell.vertices[3]];
        let mut key = ids;
        key.sort_unstable();
        if !unique_tets.insert(key) { return Err("Duplicate FEM tetrahedron".into()); }
        let p = ids.map(|i| points[i as usize]);
        let vector = |a: Point3, b: Point3| [a.x - b.x, a.y - b.y, a.z - b.z];
        let [a, b, c] = [vector(p[1], p[0]), vector(p[2], p[0]), vector(p[3], p[0])];
        let cross = [b[1] * c[2] - b[2] * c[1], b[2] * c[0] - b[0] * c[2], b[0] * c[1] - b[1] * c[0]];
        let volume = a.into_iter().zip(cross).map(|(x, y)| x * y).sum::<f64>().abs() / 6.0;
        if !volume.is_finite() || volume <= 0.0 { return Err("Invalid FEM cell volume".into()); }
        maximum = maximum.max(volume);
        for omitted in 0..4 {
            let mut face = [0; 3];
            for (out, corner) in face.iter_mut().zip(ids.iter().enumerate().filter(|(corner, _)| *corner != omitted)) {
                *out = *corner.1;
            }
            let inversions = usize::from(face[0] > face[1]) + usize::from(face[0] > face[2]) + usize::from(face[1] > face[2]);
            let negative = (omitted + inversions) % 2 != 0;
            face.sort_unstable();
            let flags = incidence.entry(face).or_default();
            let repeated = if negative { &mut flags.0 } else { &mut flags.1 };
            if std::mem::replace(repeated, true) {
                return Err("FEM tetrahedra have a non-manifold or same-facing paired facet".into());
            }
        }
    }
    certificate.max_quality = maximum;
    let mut bytes = vec![0; FEM_CERT_ENCODED_LEN];
    let size = encode_fem_certificate(&certificate, &mut bytes).map_err(|error| format!("FEM certificate seal: {error:?}"))?;
    bytes.truncate(size);
    host.admit_encoded("fem_cert", bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::specialized_libs::computational_geometry::fem_certificate::decode_fem_certificate;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn seals_fem_boundary_and_distinct_volume_extrema() {
        let points = [
            Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0), Point3::new(0.0, 0.0, 1.0),
            Point3::new(0.0, 0.0, -2.0),
        ];
        let cells = [MixedCell::tetra(0, 1, 2, 3, 3), MixedCell::tetra(0, 2, 1, 4, 7)];
        let limits = crate::ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = AssetHost { workspace: Default::default(), inputs: &[], limits: &limits, cancel: &cancel, work: 0 };
        let region = MaterialRegion { material: 3, first_cell: 0, cell_count: 1 };
        let provenance = q_hash("urn:test:fem:mesh");
        let result = certify(&mut host, &points, &cells, &[], &[region], provenance).unwrap();
        let certificate = decode_fem_certificate(&host.workspace.encoded(&result).unwrap().bytes).unwrap();
        assert_eq!(certificate.cell_count, 2);
        assert_eq!(certificate.boundary_face_count, 6);
        assert_eq!(certificate.adjacency_face_count, 1);
        assert_eq!(certificate.min_quality, 1.0 / 6.0);
        assert_eq!(certificate.max_quality, 1.0 / 3.0);
        assert_eq!(certificate.provenance_hash, provenance);
    }
}
