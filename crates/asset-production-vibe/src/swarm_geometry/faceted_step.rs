//! Single-solid planar faceted exchange using ISO 10303-21 and the AP242 schema.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{fmt::Write, sync::Arc};

use manifold_bim_csg::{parse_step, step_string, StepEntity, StepFile, StepVal};
use qualia_core_db::render::assets::{ImportedMesh, Mesh};
use sha2::{Digest, Sha256};

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::qualia_containers_governance_poet_ops::encoded_resource;

use super::*;

const AP242: &str = "AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF";
const FACET_CAP: usize = 2048;
const STEP_CAP: usize = 4 * 1024 * 1024;

impl AssetHost<'_> {
    pub(crate) fn export_step_ap242_brep(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &["mesh", "source_10d", "part_iri", "brep_chord_tol_m"],
        )?;
        let part = text(required(args, "part_iri")?)?;
        valid_part_identity(part)?;
        let tolerance = geometry_tolerance(args)?;
        let geometry = match (args.get("mesh"), args.get("source_10d")) {
            (Some(handle), None) => {
                let source = source_mesh(self, handle)?;
                let geometry = &source.surface.mesh;
                check_size(geometry.positions.len(), geometry.triangles.len())?;
                geometry.clone()
            }
            (None, Some(value)) => read_authored_leaf(self, value)?,
            _ => return Err("Supply exactly one of mesh or source_10d".into()),
        };
        let points = geometry
            .positions
            .iter()
            .copied()
            .map(point)
            .collect::<Vec<_>>();
        let triangles = &geometry.triangles;
        self.charge(
            points
                .len()
                .saturating_mul(triangles.len())
                .saturating_mul(20),
            STEP_CAP.saturating_mul(4),
        )?;
        let volume = validate_convex_shell(&points, triangles, tolerance)?;
        let bytes = write_step(part, &points, triangles, tolerance)?;
        let (checked, output_triangles) = read_step(&bytes, tolerance)?;
        if checked != points || output_triangles.as_slice() != triangles.as_slice() {
            return Err("Generated AP242 faceted geometry does not round-trip exactly".into());
        }
        let (digest, receipt) = step_receipt(
            &bytes,
            part,
            "FacetedBrepExport",
            self.limits.resource_bytes,
        )?;
        let asset = self.encode("step", bytes)?;
        let receipt = self.encode("q42", receipt)?;
        Ok(Value::Record(BTreeMap::from([
            ("asset".into(), asset),
            ("receipt".into(), receipt),
            ("step_sha256".into(), Value::String(digest)),
            ("solid_volume_m3".into(), Value::F64(volume)),
            ("facet_count".into(), Value::U64(triangles.len() as u64)),
            (
                "geometry_profile".into(),
                Value::String("single_convex_planar_faceted_shell".into()),
            ),
        ])))
    }

    pub(crate) fn step_cad_brep(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &["source", "operation", "part_iri", "brep_chord_tol_m"],
        )?;
        let operation = args
            .get("operation")
            .map(text)
            .transpose()?
            .unwrap_or("import_faceted");
        if operation != "import_faceted" {
            return Err("Admitted CAD branch is import_faceted".into());
        }
        let part = text(required(args, "part_iri")?)?;
        valid_part_identity(part)?;
        let tolerance = geometry_tolerance(args)?;
        let resource = required(args, "source")?;
        let source = encoded_resource(self, resource)?;
        let size = source.bytes.len();
        if !matches!(source.kind.as_str(), "step" | "stp") || size > STEP_CAP {
            return Err("Faceted CAD import needs retained STEP bytes up to four MiB".into());
        }
        self.charge(
            size.saturating_mul(120),
            size.saturating_mul(6).max(STEP_CAP),
        )?;
        let source = encoded_resource(self, resource)?;
        let (positions, triangles) = read_step(&source.bytes, tolerance)?;
        let (digest, receipt) = step_receipt(
            &source.bytes,
            part,
            "FacetedBrepImport",
            self.limits.resource_bytes,
        )?;
        self.charge(
            positions
                .len()
                .saturating_mul(triangles.len())
                .saturating_mul(20),
            STEP_CAP,
        )?;
        let volume = validate_convex_shell(&positions, &triangles, tolerance)?;
        let geometry = plain_mesh(&positions, triangles);
        let raw = &geometry.surface.mesh;
        let surface = raw.positions.iter().copied().map(point).collect::<Vec<_>>();
        let residual = positions
            .iter()
            .zip(&surface)
            .map(|(&a, &b)| length(sub(a, b)))
            .fold(0.0, f64::max);
        if residual > tolerance {
            return Err(format!(
                "Faceted STEP to mesh rounding residual {residual:e} m exceeds {tolerance:e} m"
            ));
        }
        validate_convex_shell(&surface, &raw.triangles, tolerance)?;
        let mesh = self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(geometry)))?;
        let graph = self.encode("q42", receipt)?;
        Ok(Value::Record(BTreeMap::from([
            ("mesh".into(), mesh),
            ("bom_port_graph".into(), graph),
            ("source_sha256".into(), Value::String(digest)),
            ("solid_volume_m3".into(), Value::F64(volume)),
            ("part_count".into(), Value::U64(1)),
            ("port_count".into(), Value::U64(0)),
        ])))
    }
}

fn valid_part_identity(value: &str) -> Result<(), String> {
    if value.len() > 512 {
        return Err("Faceted part identity exceeds 512 UTF-8 bytes".into());
    }
    qualia_core_db::bundle::semantic_model::resource_identity_meta(value)
        .map_err(|error| error.to_string())?;
    Ok(())
}

fn geometry_tolerance(args: &Args) -> Result<f64, String> {
    let tolerance = number(args, "brep_chord_tol_m", 1e-6)?;
    if !(1e-8..=1e-2).contains(&tolerance) {
        return Err("brep_chord_tol_m outside the admitted metre range".into());
    }
    Ok(tolerance)
}

fn read_authored_leaf(host: &mut AssetHost<'_>, value: &Value) -> Result<Mesh, String> {
    use qualia_core_db::container_10d::{
        self as ten, Container10dHeader, SectionType, TypedFieldRecord, TypedPayloadResolver,
        FIELD_KIND_ARCHETYPE_REF, FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE,
        MANIFOLD_IDENTITY_V2_MAX_FIELDS,
    };
    let source = encoded_resource(host, value)?;
    if source.kind != "10d" {
        return Err("Faceted export requires an authored .10d leaf".into());
    }
    let size = source.bytes.len();
    host.charge(size.saturating_mul(5), size.saturating_mul(2))?;
    let bytes = &encoded_resource(host, value)?.bytes;
    ten::integrity::check_whole_file_crc32c(bytes).map_err(|e| e.to_string())?;
    let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = scratch;
    let index = TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut fields)
        .map_err(|e| e.to_string())?;
    if index
        .payloads_of_kind(FIELD_KIND_ARCHETYPE_REF)
        .next()
        .is_some()
        || index
            .payloads_of_kind(FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE)
            .next()
            .is_none()
    {
        return Err("Faceted export requires a holographic leaf rather than an assembly".into());
    }
    let header = Container10dHeader::parse(bytes).map_err(|e| e.to_string())?;
    let table = ten::parse_section_table(bytes, &header).map_err(|e| e.to_string())?;
    let section = table
        .iter()
        .find(|row| row.typ() == Some(SectionType::QuantizedMesh))
        .ok_or("Faceted export requires authored triangles rather than proxy bounds")?;
    let offset = section.byte_offset as usize;
    let end = offset
        .checked_add(section.byte_length as usize)
        .ok_or("Quantized mesh length overflow")?;
    let (mesh_header, _) = ten::parse_mesh_header(
        bytes
            .get(offset..end)
            .ok_or("Truncated authored mesh section")?,
    )
    .map_err(|e| format!("{e:?}"))?;
    check_size(
        mesh_header.vertex_count as usize,
        mesh_header.triangle_count as usize,
    )?;
    qualia_core_db::render::compile_10d::decode_10d_mesh(bytes).map_err(|e| format!("{e:?}"))
}

fn check_size(vertices: usize, triangles: usize) -> Result<(), String> {
    if !(4..=FACET_CAP).contains(&vertices) || !(4..=FACET_CAP).contains(&triangles) {
        return Err(
            "Faceted exchange admits one closed shell of up to 2048 vertices and triangles".into(),
        );
    }
    Ok(())
}

pub(super) fn validate_convex_shell(
    points: &[Point],
    triangles: &[[u32; 3]],
    tolerance: f64,
) -> Result<f64, String> {
    check_size(points.len(), triangles.len())?;
    let mut unique = BTreeSet::new();
    for &p in points {
        if p.iter().any(|x| !x.is_finite() || x.abs() > 1e6)
            || !unique.insert(p.map(|x| if x == 0.0 { 0 } else { x.to_bits() }))
        {
            return Err("Faceted shell needs unique finite local-metre vertices".into());
        }
    }
    let mut edges = BTreeMap::<Edge, (usize, u32, bool)>::new();
    let mut links = vec![Vec::new(); points.len()];
    let mut adjacency = vec![[usize::MAX; 3]; triangles.len()];
    let mut degrees = vec![0; triangles.len()];
    let mut volume = 0.0;
    for (face, &triangle) in triangles.iter().enumerate() {
        if triangle.iter().any(|&i| i as usize >= points.len()) {
            return Err("Faceted shell index is outside its vertex set".into());
        }
        let [p, q, r] = triangle.map(|i| points[i as usize]);
        let normal = face_normal([p, q, r])?;
        let residual = points
            .iter()
            .map(|&v| dot(normal, sub(v, p)))
            .fold(0.0, f64::max);
        if residual > tolerance {
            return Err(format!(
                "Faceted shell is nonconvex, inward, or nonplanar. Face {face} half-space residual {residual:e} m exceeds {tolerance:e} m"
            ));
        }
        volume += dot(
            sub(p, points[0]),
            cross(sub(q, points[0]), sub(r, points[0])),
        ) / 6.0;
        for i in 0..3 {
            let [a, b, c] = std::array::from_fn(|offset| triangle[(i + offset) % 3]);
            links[a as usize].push((b, c));
            let edge = sorted_edge(a, b);
            if let Some((prior, prior_from, paired)) = edges.get_mut(&edge) {
                if *paired || *prior_from == a {
                    return Err("Faceted shell has a nonmanifold or misoriented edge".into());
                }
                *paired = true;
                adjacency[face][degrees[face]] = *prior;
                degrees[face] += 1;
                adjacency[*prior][degrees[*prior]] = face;
                degrees[*prior] += 1;
            } else {
                edges.insert(edge, (face, a, false));
            }
        }
    }
    if degrees.iter().any(|&n| n != 3)
        || points.len() + triangles.len() != edges.len() + 2
        || volume <= tolerance.powi(3)
    {
        return Err("Faceted shell fails its closed, outward, genus-zero topology gate".into());
    }
    let mut seen = vec![false; triangles.len()];
    let mut todo = vec![0];
    seen[0] = true;
    while let Some(face) = todo.pop() {
        for twin in adjacency[face] {
            if !seen[twin] {
                seen[twin] = true;
                todo.push(twin);
            }
        }
    }
    if seen.iter().any(|&connected| !connected) {
        return Err("Faceted shell must be connected".into());
    }
    for outgoing in links {
        let Some(&(start, _)) = outgoing.first() else {
            return Err("Faceted shell contains an unused vertex".into());
        };
        let order = outgoing.iter().copied().collect::<BTreeMap<_, _>>();
        if order.len() != outgoing.len() {
            return Err("Faceted shell contains a nonmanifold vertex".into());
        }
        let mut at = start;
        for i in 0..outgoing.len() {
            at = *order.get(&at).ok_or("Faceted vertex link is open")?;
            if (at == start) != (i + 1 == outgoing.len()) {
                return Err("Faceted vertex link is not one cycle".into());
            }
        }
    }
    Ok(volume)
}

pub(super) fn plain_mesh(points: &[Point], triangles: Vec<[u32; 3]>) -> AttributedMesh {
    let positions = points
        .iter()
        .map(|p| p.map(|x| x as f32))
        .collect::<Vec<_>>();
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in &positions {
        for i in 0..3 {
            min[i] = min[i].min(p[i]);
            max[i] = max[i].max(p[i]);
        }
    }
    AttributedMesh {
        surface: ImportedMesh {
            mesh: Mesh {
                positions,
                triangles,
                min,
                max,
            },
            normals: None,
            tangents: None,
            texture_coordinates_0: None,
            materials: Vec::new(),
            texture_dependencies: Vec::new(),
            submeshes: Vec::new(),
        },
        uv1: Vec::new(),
        vertex_colors: Vec::new(),
        skin_weights: Vec::new(),
        morph_targets: Arc::from([]),
        protected_faces: Vec::new(),
    }
}

fn write_step(
    part: &str,
    points: &[Point],
    triangles: &[[u32; 3]],
    tolerance: f64,
) -> Result<Vec<u8>, String> {
    let part = step_string(part);
    let mut writer = StepWriter {
        text: format!(
            "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('Single faceted solid'),'2;1');\nFILE_NAME('solid.step','1970-01-01T00:00:00',(''),(''),'QualiaDB','Hypermedia Production','');\nFILE_SCHEMA(('{AP242} {{ 1 0 10303 442 1 1 4 }}'));\nENDSEC;\nDATA;\n"
        ),
        next: 1,
    };
    let app = writer.row(format_args!(
        "APPLICATION_CONTEXT('managed model based 3d engineering')"
    ));
    writer.row(format_args!("APPLICATION_PROTOCOL_DEFINITION('international standard','ap242_managed_model_based_3d_engineering',2014,#{app})"));
    let product_context = writer.row(format_args!("PRODUCT_CONTEXT('',#{app},'mechanical')"));
    let product = writer.row(format_args!(
        "PRODUCT({part},{part},'',(#{product_context}))"
    ));
    let formation = writer.row(format_args!(
        "PRODUCT_DEFINITION_FORMATION('','',#{product})"
    ));
    let definition_context = writer.row(format_args!(
        "PRODUCT_DEFINITION_CONTEXT('part definition',#{app},'design')"
    ));
    let definition = writer.row(format_args!(
        "PRODUCT_DEFINITION('design','',#{formation},#{definition_context})"
    ));
    let shape = writer.row(format_args!(
        "PRODUCT_DEFINITION_SHAPE('','',#{definition})"
    ));
    let metric = writer.row(format_args!(
        "(LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($,.METRE.))"
    ));
    let angle = writer.row(format_args!(
        "(NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.))"
    ));
    let solid_angle = writer.row(format_args!(
        "(NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT())"
    ));
    let uncertainty = writer.row(format_args!("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE({tolerance:.16E}),#{metric},'distance_accuracy_value','')"));
    let context = writer.row(format_args!("(GEOMETRIC_REPRESENTATION_CONTEXT(3) GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#{uncertainty})) GLOBAL_UNIT_ASSIGNED_CONTEXT((#{metric},#{angle},#{solid_angle})) REPRESENTATION_CONTEXT('','3D'))"));
    let vertices = points
        .iter()
        .map(|&[a, b, c]| {
            writer.row(format_args!(
                "CARTESIAN_POINT('',({a:.16E},{b:.16E},{c:.16E}))"
            ))
        })
        .collect::<Vec<_>>();
    let mut faces = Vec::with_capacity(triangles.len());
    for &triangle in triangles {
        let [a, b, c] = triangle.map(|i| vertices[i as usize]);
        let [p, q, r] = triangle.map(|i| points[i as usize]);
        let [nx, ny, nz] = face_normal([p, q, r])?;
        let [tx, ty, tz] = normalized(sub(q, p))?;
        let normal = writer.row(format_args!(
            "DIRECTION('',({nx:.16E},{ny:.16E},{nz:.16E}))"
        ));
        let direction = writer.row(format_args!(
            "DIRECTION('',({tx:.16E},{ty:.16E},{tz:.16E}))"
        ));
        let axis = writer.row(format_args!(
            "AXIS2_PLACEMENT_3D('',#{a},#{normal},#{direction})"
        ));
        let plane = writer.row(format_args!("PLANE('',#{axis})"));
        let loop_id = writer.row(format_args!("POLY_LOOP('',(#{a},#{b},#{c}))"));
        let bound = writer.row(format_args!("FACE_OUTER_BOUND('',#{loop_id},.T.)"));
        faces.push(writer.row(format_args!("FACE_SURFACE('',(#{bound}),#{plane},.T.)")));
    }
    let shell_ids = references(&faces);
    let shell = writer.row(format_args!("CLOSED_SHELL('',({shell_ids}))"));
    let solid = writer.row(format_args!("FACETED_BREP({part},#{shell})"));
    let rep = writer.row(format_args!(
        "FACETED_BREP_SHAPE_REPRESENTATION('',(#{solid}),#{context})"
    ));
    writer.row(format_args!(
        "SHAPE_DEFINITION_REPRESENTATION(#{shape},#{rep})"
    ));
    writer.text.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
    Ok(writer.text.into_bytes())
}

struct StepWriter {
    text: String,
    next: u64,
}

impl StepWriter {
    fn row(&mut self, row: std::fmt::Arguments<'_>) -> u64 {
        let id = self.next;
        self.next += 1;
        writeln!(&mut self.text, "#{id}={row};")
            .expect("writing a format into String is infallible");
        id
    }
}

fn references(ids: &[u64]) -> String {
    ids.iter()
        .map(|id| format!("#{id}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn read_step(bytes: &[u8], tolerance: f64) -> Result<(Vec<Point>, Vec<[u32; 3]>), String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "Faceted STEP input must be UTF-8")?;
    if bytes.len() > STEP_CAP
        || !text.trim().starts_with("ISO-10303-21;")
        || !text.trim().ends_with("END-ISO-10303-21;")
    {
        return Err("Faceted STEP document is missing its complete exchange envelope".into());
    }
    let lexical = step_entities_without_strings(text)?;
    let step = parse_step(bytes)?;
    if !step
        .schema
        .split_whitespace()
        .next()
        .is_some_and(|schema| schema == AP242)
    {
        return Err("Faceted CAD import admits the AP242 schema".into());
    }
    let reps = step
        .entities
        .values()
        .filter(|entity| entity.name == "FACETED_BREP_SHAPE_REPRESENTATION")
        .collect::<Vec<_>>();
    let [representation] = reps.as_slice() else {
        return Err("Expected one FACETED_BREP_SHAPE_REPRESENTATION".into());
    };
    let [StepVal::Str(_), StepVal::List(items), StepVal::Ref(context)] =
        representation.args.as_slice()
    else {
        return Err("Invalid faceted shape representation".into());
    };
    let [StepVal::Ref(solid)] = items.as_slice() else {
        return Err("Expected a single faceted shape item".into());
    };
    let scale = cad_length_scale(&lexical, *context)?;
    if step
        .entities
        .values()
        .filter(|e| {
            matches!(
                e.name.as_str(),
                "FACETED_BREP" | "MANIFOLD_SOLID_BREP" | "BREP_WITH_VOIDS"
            )
        })
        .count()
        != 1
    {
        return Err(
            "Faceted STEP importer requires one standalone solid without void shells".into(),
        );
    }
    let [StepVal::Str(_), StepVal::Ref(shell)] = entity(&step, *solid, "FACETED_BREP")? else {
        return Err("Invalid faceted solid".into());
    };
    let [StepVal::Str(_), StepVal::List(faces)] = entity(&step, *shell, "CLOSED_SHELL")? else {
        return Err("Invalid faceted shell".into());
    };
    if !(4..=FACET_CAP).contains(&faces.len()) {
        return Err("Faceted shell face count exceeds the admitted bounds".into());
    }
    let mut used = BTreeMap::<u64, Point>::new();
    let mut triangle_points = Vec::with_capacity(faces.len());
    for face in faces {
        let StepVal::Ref(face) = face else {
            return Err("Faceted shell faces must be references".into());
        };
        let [StepVal::Str(_), StepVal::List(bounds), StepVal::Ref(plane), StepVal::Enum(sense)] =
            entity(&step, *face, "FACE_SURFACE")?
        else {
            return Err("Expected a planar face surface with a single boundary".into());
        };
        let [StepVal::Ref(bound)] = bounds.as_slice() else {
            return Err("Faceted STEP faces must have one outer loop and no holes".into());
        };
        let [StepVal::Str(_), StepVal::Ref(polyloop), StepVal::Enum(orientation)] =
            entity(&step, *bound, "FACE_OUTER_BOUND")?
        else {
            return Err("Invalid faceted outer boundary".into());
        };
        let [StepVal::Str(_), StepVal::List(vertices)] = entity(&step, *polyloop, "POLY_LOOP")?
        else {
            return Err("Faceted boundary must be a polygon".into());
        };
        let [StepVal::Ref(a), StepVal::Ref(b), StepVal::Ref(c)] = vertices.as_slice() else {
            return Err("Faceted STEP branch admits only triangular facets".into());
        };
        let ids = match orientation.as_str() {
            "T" => [*a, *b, *c],
            "F" => [*a, *c, *b],
            _ => return Err("Invalid STEP bound orientation".into()),
        };
        let data = ids.map(|id| xyz(&step, id, "CARTESIAN_POINT", scale));
        let [a, b, c] = data;
        let points = [a?, b?, c?];
        for (id, p) in ids.into_iter().zip(points) {
            used.insert(id, p);
        }
        let [StepVal::Str(_), StepVal::Ref(axis)] = entity(&step, *plane, "PLANE")? else {
            return Err("Faceted plane is not explicit".into());
        };
        let [StepVal::Str(_), StepVal::Ref(origin), StepVal::Ref(normal), StepVal::Ref(reference)] =
            entity(&step, *axis, "AXIS2_PLACEMENT_3D")?
        else {
            return Err("Expected explicit 3D face plane placement".into());
        };
        let center = xyz(&step, *origin, "CARTESIAN_POINT", scale)?;
        let mut n = normalized(xyz(&step, *normal, "DIRECTION", 1.0)?)?;
        let tangent = normalized(xyz(&step, *reference, "DIRECTION", 1.0)?)?;
        if length(cross(n, tangent)) <= 1e-8 {
            return Err("Faceted STEP plane axes are collinear".into());
        }
        match sense.as_str() {
            "T" => {}
            "F" => n = mul(n, -1.0),
            _ => return Err("Invalid STEP face sense".into()),
        }
        if points
            .iter()
            .any(|&p| dot(n, sub(p, center)).abs() > tolerance)
            || dot(face_normal(points)?, n) < 1.0 - 1e-6
        {
            return Err("Faceted STEP plane conflicts with its oriented boundary".into());
        }
        triangle_points.push(ids);
    }
    let vertex_ids = used
        .keys()
        .enumerate()
        .map(|(index, &id)| (id, index as u32))
        .collect::<BTreeMap<_, _>>();
    let points = used.into_values().collect::<Vec<_>>();
    let triangles = triangle_points
        .into_iter()
        .map(|ids| ids.map(|id| vertex_ids[&id]))
        .collect::<Vec<_>>();
    check_size(points.len(), triangles.len())?;
    Ok((points, triangles))
}

fn entity<'a>(step: &'a StepFile, id: u64, expected: &str) -> Result<&'a [StepVal], String> {
    let Some(StepEntity { name, args }) = step.entities.get(&id) else {
        return Err(format!("Missing STEP entity #{id}"));
    };
    if name != expected {
        return Err(format!("STEP entity #{id} must be {expected}"));
    }
    Ok(args)
}

fn xyz(step: &StepFile, id: u64, kind: &str, scale: f64) -> Result<Point, String> {
    let [StepVal::Str(_), StepVal::List(values)] = entity(step, id, kind)? else {
        return Err(format!("Invalid STEP {kind} coordinates"));
    };
    let [x, y, z] = values.as_slice() else {
        return Err(format!("STEP {kind} needs 3 coordinates"));
    };
    let number = |value: &StepVal| -> Result<f64, String> {
        let n = match value {
            StepVal::Real(v) => *v,
            StepVal::Int(v) => *v as f64,
            _ => return Err("Expected a STEP real coordinate".into()),
        } * scale;
        if !n.is_finite() || n.abs() > 1e6 {
            return Err("STEP coordinate lies outside finite local bounds".into());
        }
        Ok(n)
    };
    Ok([number(x)?, number(y)?, number(z)?])
}

fn step_entities_without_strings(text: &str) -> Result<String, String> {
    let mut clean = String::with_capacity(text.len());
    let mut source = text.chars().peekable();
    let mut quoted = false;
    let mut comment = false;
    while let Some(ch) = source.next() {
        if comment {
            if ch == '*' && source.next_if_eq(&'/').is_some() {
                comment = false;
            }
        } else if quoted {
            if ch == '\'' && source.next_if_eq(&'\'').is_none() {
                quoted = false;
            }
        } else if ch == '/' && source.next_if_eq(&'*').is_some() {
            comment = true;
        } else if ch == '\'' {
            quoted = true;
            clean.push('?');
        } else if !ch.is_whitespace() {
            clean.push(ch.to_ascii_uppercase());
        }
    }
    if quoted || comment {
        return Err("Unclosed STEP string or comment".into());
    }
    if clean.split(';').filter(|row| row.starts_with('#')).count() > 8 * FACET_CAP + 64 {
        return Err("STEP entity count exceeds the faceted exchange budget".into());
    }
    Ok(clean)
}

fn cad_length_scale(clean: &str, context: u64) -> Result<f64, String> {
    // The native parser skips complex entities. Inspect only the uniquely referenced SI declarations.
    let mut needed = BTreeMap::new();
    let mut context_ids = None;
    for row in clean.split(';') {
        if let Some((id, value)) = row.strip_prefix('#').and_then(|r| r.split_once('=')) {
            let id = id
                .parse::<u64>()
                .map_err(|_| "Invalid STEP entity identifier")?;
            if needed.insert(id, value).is_some() {
                return Err("Duplicate STEP entity identifier".into());
            }
            if id == context {
                let tail = value
                    .strip_prefix("(GEOMETRIC_REPRESENTATION_CONTEXT(3)")
                    .ok_or("Expected a 3D STEP representation context")?;
                let units = tail
                    .split_once("GLOBAL_UNIT_ASSIGNED_CONTEXT((")
                    .and_then(|(_, t)| t.split_once("))"))
                    .map(|(u, _)| u)
                    .ok_or("Faceted STEP context lacks explicit units")?;
                context_ids = Some(
                    units
                        .split(',')
                        .map(|u| {
                            u.strip_prefix('#')
                                .and_then(|u| u.parse::<u64>().ok())
                                .ok_or("Invalid STEP unit reference")
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                );
            }
        }
    }
    let mut scale = None;
    for id in context_ids.ok_or("Missing STEP shape representation context")? {
        let value = *needed.get(&id).ok_or("Missing referenced STEP unit")?;
        let declared = match value {
            "(LENGTH_UNIT()NAMED_UNIT(*)SI_UNIT($,.METRE.))" => Some(1.0),
            "(LENGTH_UNIT()NAMED_UNIT(*)SI_UNIT(.MILLI.,.METRE.))" => Some(0.001),
            "(NAMED_UNIT(*)PLANE_ANGLE_UNIT()SI_UNIT($,.RADIAN.))" => None,
            "(NAMED_UNIT(*)SI_UNIT($,.STERADIAN.)SOLID_ANGLE_UNIT())" => None,
            _ => return Err(
                "Faceted STEP context uses a unit outside the admitted metre/millimetre SI profile"
                    .into(),
            ),
        };
        if let Some(declared) = declared {
            if scale.replace(declared).is_some() {
                return Err("STEP context declares more than one length unit".into());
            }
        }
    }
    scale.ok_or_else(|| "STEP context has no length unit".into())
}

fn step_receipt(
    bytes: &[u8],
    part: &str,
    operation: &str,
    limit: usize,
) -> Result<(String, Vec<u8>), String> {
    let hash = format!("{:x}", Sha256::digest(bytes));
    let asset = format!("urn:sha256:{hash}");
    let mut graph = ReceiptGraph::new(&format!("urn:qualia:step:{operation}:v1"));
    graph.fact(
        &asset,
        "https://qualiadb.org/ontology/cad#representsPart",
        part,
    )?;
    graph.fact(
        &asset,
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
        "https://qualiadb.org/ontology/cad#FacetedAp242Exchange",
    )?;
    graph.fact(
        part,
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
        "https://qualiadb.org/ontology/cad#ConvexFacetedSolid",
    )?;
    Ok((hash, graph.seal(limit)?))
}
