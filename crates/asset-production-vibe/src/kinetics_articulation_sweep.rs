//! Bounded engineering verification of one hinge between verified HMC leaf solids.

use crate::{fields, integer, list, num, record, required, text, vector, Args, AssetHost, Value};
use qualia_core_db::{
    bundle::{
        semantic_model::{open_semantic_model, SemanticModelBuilder},
        BundleReadLimits, BundleReader,
    },
    container_10d::{
        self as ten, SectionType, SubmeshRange, TypedFieldRecord, TypedPayloadResolver,
    },
    q_hash,
    query::resolver::INLINE_TAG_FLOAT,
    render::{
        portal_load_10d::{admit_leaf_assembly, AdmittedAssemblyLeaf},
        skeletal::{valid_motor, Motor},
    },
    specialized_libs::computational_geometry::{
        orient_3d, point_segment_distance_sq_3d, point_triangle_distance_sq_3d,
        tri_tri_intersect_3, Point3,
    },
    NQuin,
};
use serde_json::{json, Value as Json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const MAX_SOLID_TRIANGLES: usize = 96;
const MAX_SOLID_VERTICES: usize = 64;
const MAX_POSES: usize = 64;
const MAX_VOIDS: usize = 8;
const Q42_BUDGET: usize = 4 * 1024 * 1024;
const PIVOT_DRIFT_LIMIT_M: f64 = 1e-12;
const NS: &str = "https://qualiadb.org/ns/kinematics#";

type V3 = [f64; 3];

struct Solid {
    vertices: Vec<V3>,
    faces: Vec<[u32; 3]>,
    edges: Vec<[u32; 2]>,
    surface_ids: Vec<u64>,
    rest: Motor,
    scale: f64,
    slot: usize,
}

impl Solid {
    fn world(&self, delta: &Motor) -> Vec<V3> {
        self.vertices
            .iter()
            .map(|&point| {
                self.rest
                    .transform_point(delta.transform_point(point).map(|coord| coord * self.scale))
            })
            .collect()
    }
}

struct Geometry {
    fixed: Solid,
    moving: Solid,
    protected: Vec<Solid>,
    digest: String,
}

#[derive(Clone, Copy)]
struct Anchor<'a> {
    face: usize,
    barycentric: V3,
    iri: &'a str,
}

#[derive(Clone, Copy)]
struct Pose {
    seconds: f64,
    angle: f64,
}

struct Separation {
    gap: f64,
    tolerance: f64,
    distance: f64,
    overlap_bound: f64,
    allowed_hits: usize,
    forbidden_hits: usize,
}

impl AssetHost<'_> {
    pub(crate) fn fit_and_verify_articulation_sweep(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "assembly_key",
                "receipt_iri",
                "context_iri",
                "fixed_leaf_iri",
                "moving_leaf_iri",
                "hinge_pairs",
                "max_landmark_residual_mm",
                "minimum_clearance_mm",
                "poses",
                "allowed_contact_zones",
                "protected_voids",
            ],
        )?;
        let source = required(args, "source")?;
        let key = text(required(args, "assembly_key")?)?;
        let receipt = iri(required(args, "receipt_iri")?)?;
        let context = iri(required(args, "context_iri")?)?;
        let fixed = iri(required(args, "fixed_leaf_iri")?)?;
        let moving = iri(required(args, "moving_leaf_iri")?)?;
        if key.is_empty()
            || key.len() > 4096
            || fixed == moving
            || receipt == moving
            || context == receipt
        {
            return Err("Invalid articulation assembly key or repeated identity".into());
        }
        let hinge = parse_hinge(required(args, "hinge_pairs")?)?;
        let poses = parse_poses(required(args, "poses")?)?;
        let allowance = nonnegative(required(args, "minimum_clearance_mm")?)? * 0.001;
        let fit_limit = nonnegative(required(args, "max_landmark_residual_mm")?)? * 0.001;
        let zones = parse_zones(args.get("allowed_contact_zones"))?;
        let voids = parse_voids(args.get("protected_voids"), fixed, moving)?;

        let source_len = sealed_hmc(self, source)?.len();
        self.charge(source_len, 0)?;
        let geometry = load_geometry(
            sealed_hmc(self, source)?,
            key,
            fixed,
            moving,
            &hinge,
            &zones,
            &voids,
            self.limits.resources,
        )?;
        let (axis, pivot, residual) = fit_hinge(&geometry.fixed, &geometry.moving, hinge)?;
        if residual > fit_limit {
            return Err(format!(
                "Paired facet mismatch {residual:.12e} m exceeds {fit_limit:.12e} m"
            ));
        }

        let movers = &geometry.moving;
        let work = std::iter::once(&geometry.fixed)
            .chain(&geometry.protected)
            .map(|other| {
                (movers.edges.len() * other.edges.len() + movers.faces.len() + other.faces.len())
                    * (movers.vertices.len() + other.vertices.len())
                    + 64 * movers.faces.len() * other.faces.len()
            })
            .sum::<usize>()
            * poses.len();
        self.charge(work, 1024 * 1024)?;
        let measurements =
            measure_sweep(&geometry, &poses, axis, pivot, allowance, &zones, &voids)?;
        let status = measurements.status;
        let rom = poses.iter().fold([0.0_f64; 2], |[min, max], pose| {
            [min.min(pose.angle), max.max(pose.angle)]
        });
        let q42 = seal_receipt(
            receipt,
            context,
            moving,
            status,
            axis,
            pivot,
            rom,
            &geometry.digest,
            self.limits.resource_bytes,
        )?;
        let report = json!({
            "schema": "qualia.articulation-sweep/v1", "source_sha256": geometry.digest,
            "assembly_key": key, "receipt_iri": receipt, "context_iri": context, "fixed_leaf_iri": fixed,
            "moving_leaf_iri": moving, "moving_leaf_slot": movers.slot, "engineering_geometry_status": status,
            "anatomical_review_status": "pending_clinical_review",
            "measurement_method": "convex_sat_and_triangle_feature_distances_f64",
            "penetration_volume_method": "world_aabb_overlap_upper_bound_cm3",
            "continuous_method": "sample_endpoint_axis_gap_minus_maximum_nearest_endpoint_rotation_travel",
            "axis_local": axis, "pivot_local_m": pivot, "landmark_residual_mm": residual * 1000.0,
            "max_landmark_residual_mm": fit_limit * 1000.0,
            "pivot_drift_m": measurements.pivot_drift, "rom_radians": rom,
            "minimum_clearance_mm": allowance * 1000.0,
            "continuous_clearance_margin_lower_bound_mm": measurements.clearance_lower_bound * 1000.0,
            "poses": measurements.poses,
        });
        let json = serde_json::to_vec(&report).map_err(|error| error.to_string())?;
        Ok(Value::Record(BTreeMap::from([
            ("receipt".into(), self.encode("json", json)?),
            ("model".into(), self.encode("q42", q42)?),
            (
                "engineering_geometry_status".into(),
                Value::String(status.into()),
            ),
        ])))
    }
}

fn sealed_hmc<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a [u8], String> {
    use crate::artifact::HmcWorkspaceView;
    if let Ok(crate::TypedArtifact::HmcBundle(bundle)) = host.workspace.typed(value) {
        return match bundle.as_ref() {
            HmcWorkspaceView::Sealed(bytes) => Ok(bytes),
            HmcWorkspaceView::Staged(_) => Err("Articulation requires a sealed HMC".into()),
        };
    }
    let source = crate::qualia_containers_governance_poet_ops::encoded_resource(host, value)?;
    (source.kind == "hmc")
        .then_some(source.bytes.as_slice())
        .ok_or_else(|| "Articulation requires a sealed HMC".into())
}

fn iri(value: &Value) -> Result<&str, String> {
    let value = text(value)?;
    let (scheme, name) = value
        .split_once(':')
        .ok_or("Expected an absolute RDF IRI")?;
    let mut scheme = scheme.bytes();
    if value.len() > 4096
        || !scheme
            .next()
            .is_some_and(|first| first.is_ascii_alphabetic())
        || !scheme.all(|byte| byte.is_ascii_alphanumeric() || b"+-.".contains(&byte))
        || name.is_empty()
        || name
            .chars()
            .any(|ch| ch.is_whitespace() || ch.is_control() || "<>\"{}|\\^`".contains(ch))
    {
        return Err("Expected a valid absolute RDF IRI".into());
    }
    Ok(value)
}

fn nonnegative(value: &Value) -> Result<f64, String> {
    let value = num(value)?;
    (value >= 0.0)
        .then_some(value)
        .ok_or_else(|| "Expected a nonnegative metric tolerance".into())
}

fn parse_anchor(value: &Value) -> Result<Anchor<'_>, String> {
    let anchor = record(value)?;
    fields(anchor, &["triangle", "barycentric", "facet_iri"])?;
    let barycentric = vector(required(anchor, "barycentric")?)?;
    if barycentric.iter().any(|&v| !(0.0..=1.0).contains(&v))
        || (barycentric.iter().sum::<f64>() - 1.0).abs() > 1e-12
    {
        return Err("Facet barycentrics must be in [0, 1] and sum to one".into());
    }
    Ok(Anchor {
        face: integer(required(anchor, "triangle")?, "triangle")?,
        barycentric,
        iri: iri(required(anchor, "facet_iri")?)?,
    })
}

fn parse_hinge(value: &Value) -> Result<[[Anchor<'_>; 2]; 2], String> {
    let [first, second] = list(value)? else {
        return Err("One hinge requires two ordered pairs of facet anchors".into());
    };
    let pair = |value| {
        let value = record(value)?;
        fields(value, &["fixed", "moving"])?;
        Ok::<_, String>([
            parse_anchor(required(value, "fixed")?)?,
            parse_anchor(required(value, "moving")?)?,
        ])
    };
    Ok([pair(first)?, pair(second)?])
}

fn parse_poses(value: &Value) -> Result<Vec<Pose>, String> {
    let poses = list(value)?;
    if !(2..=MAX_POSES).contains(&poses.len()) {
        return Err("Articulation requires 2..=64 ordered poses".into());
    }
    let mut output = Vec::with_capacity(poses.len());
    for pose in poses {
        let pose = record(pose)?;
        fields(pose, &["seconds", "angle_rad"])?;
        let seconds = nonnegative(required(pose, "seconds")?)?;
        let angle = num(required(pose, "angle_rad")?)?;
        if angle.abs() > std::f64::consts::TAU {
            return Err("A hinge pose angle exceeds one turn".into());
        }
        let angle = f64::from(angle as f32);
        if output
            .last()
            .is_some_and(|last: &Pose| last.seconds >= seconds)
        {
            return Err("Pose times must increase strictly".into());
        }
        output.push(Pose { seconds, angle });
    }
    if output[0].seconds != 0.0
        || output[0].angle != 0.0
        || output.iter().all(|pose| pose.angle == 0.0)
    {
        return Err(
            "Trajectory must start at neutral time zero and move through a nonzero angle".into(),
        );
    }
    Ok(output)
}

fn parse_zones(value: Option<&Value>) -> Result<Vec<[&str; 2]>, String> {
    let zones = value.map(list).transpose()?.unwrap_or(&[]);
    if zones.len() > 32 {
        return Err("Allowed contact zones exceed 32 pairs".into());
    }
    zones
        .iter()
        .map(|zone| {
            let [a, b] = list(zone)? else {
                return Err("A zone requires [fixed facet IRI, moving facet IRI]".into());
            };
            Ok([iri(a)?, iri(b)?])
        })
        .collect()
}

fn parse_voids<'a>(
    value: Option<&'a Value>,
    fixed: &str,
    moving: &str,
) -> Result<Vec<(&'a str, f64)>, String> {
    let voids = value.map(list).transpose()?.unwrap_or(&[]);
    if voids.len() > MAX_VOIDS {
        return Err("Protected void count exceeds 8".into());
    }
    let mut seen = BTreeSet::from([fixed, moving]);
    voids
        .iter()
        .map(|value| {
            let value = record(value)?;
            fields(value, &["leaf_iri", "minimum_clearance_mm"])?;
            let leaf = iri(required(value, "leaf_iri")?)?;
            if !seen.insert(leaf) {
                return Err("Protected void must have its own unique leaf identity".into());
            }
            Ok((
                leaf,
                nonnegative(required(value, "minimum_clearance_mm")?)? * 0.001,
            ))
        })
        .collect()
}

fn load_geometry(
    bytes: &[u8],
    key: &str,
    fixed: &str,
    moving: &str,
    hinge: &[[Anchor<'_>; 2]; 2],
    zones: &[[&str; 2]],
    voids: &[(&str, f64)],
    max_entries: usize,
) -> Result<Geometry, String> {
    let bundle = BundleReader::parse_with_limits(
        bytes,
        BundleReadLimits {
            file_bytes: bytes.len(),
            entries: max_entries,
            ..BundleReadLimits::default()
        },
    )
    .map_err(|error| error.to_string())?;
    let admitted = admit_leaf_assembly(&bundle, key)?;
    let model = open_semantic_model(&bundle).map_err(|error| error.to_string())?;
    let mut page = vec![0; 1024 * 1024];
    let mut token = [0; 4096];
    for identity in [fixed, moving]
        .into_iter()
        .chain(hinge.iter().flatten().map(|a| a.iri))
        .chain(zones.iter().flatten().copied())
        .chain(voids.iter().map(|(name, _)| *name))
    {
        let size = model
            .lookup_lexicon_hash_into(q_hash(identity), &mut page, &mut token)
            .map_err(|error| error.to_string())?
            .ok_or("Articulation identity is absent from the HMC model")?;
        if token.get(..size) != Some(identity.as_bytes()) {
            return Err("Articulation identity does not round-trip through Q42LEX".into());
        }
    }
    let solid = |identity: &str| {
        let leaf = admitted
            .iter()
            .find(|leaf| leaf.semantic_id == q_hash(identity))
            .ok_or("Requested articulation or void leaf is absent")?;
        load_solid(leaf)
    };
    let fixed = solid(fixed)?;
    let moving = solid(moving)?;
    for &[a, b] in zones {
        if !fixed.surface_ids.contains(&q_hash(a)) || !moving.surface_ids.contains(&q_hash(b)) {
            return Err(
                "Allowed contact zones must name authored MAT surface ranges on both leaves".into(),
            );
        }
    }
    let protected = voids
        .iter()
        .map(|(identity, _)| solid(identity))
        .collect::<Result<_, _>>()?;
    Ok(Geometry {
        fixed,
        moving,
        protected,
        digest: format!("{:x}", Sha256::digest(bytes)),
    })
}

fn load_solid(leaf: &AdmittedAssemblyLeaf<'_>) -> Result<Solid, String> {
    if leaf
        .coefficients
        .iter()
        .any(|record| record.sigma_weight != 0.0)
    {
        return Err("Articulation collision solids must have no active SSM coefficients".into());
    }
    ten::integrity::check_whole_file_crc32c(leaf.bytes).map_err(|error| error.to_string())?;
    let mut scratch = [TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = scratch;
    let view = TypedPayloadResolver::new(leaf.bytes, &[], &mut scratch, &mut fields)
        .map_err(|error| error.to_string())?;
    if view
        .payloads_of_kind(ten::manifold_identity_v2::FIELD_KIND_SIGNED_DISTANCE_BRICK)
        .next()
        .is_some()
    {
        return Err(
            "Articulation convex sweep requires an authored QuantizedMesh collision boundary"
                .into(),
        );
    }
    let bytes = view
        .section_payload(SectionType::QuantizedMesh)
        .ok_or("Collision leaf has no QuantizedMesh")?;
    let (header, _) = ten::parse_mesh_header(bytes).map_err(|error| error.to_string())?;
    if !(4..=MAX_SOLID_VERTICES).contains(&(header.vertex_count as usize))
        || !(4..=MAX_SOLID_TRIANGLES).contains(&(header.triangle_count as usize))
    {
        return Err(
            "Each convex collision leaf requires 4..=64 vertices and 4..=96 triangles".into(),
        );
    }
    let mesh = ten::decode_mesh_section(bytes).map_err(|error| error.to_string())?;
    let vertices: Vec<_> = mesh
        .positions
        .into_iter()
        .map(|p| p.map(f64::from))
        .collect();
    let edges = validate_convex(&vertices, &mesh.triangles)?;
    let mut surface_ids = vec![0; mesh.triangles.len()];
    if let Some(material) = view.section_payload(SectionType::Materials) {
        let (materials, ranges) =
            ten::material_section_counts(material).map_err(|error| error.to_string())?;
        let mut materials = vec![ten::MaterialRecord::legacy_default(); materials];
        let mut ranges = vec![
            SubmeshRange { first_index: 0, index_count: 0, material_id: 0, semantic_id: 0 };
            ranges
        ];
        let (_, count) = ten::decode_material_section_into(
            material,
            mesh.triangles.len() as u32 * 3,
            &mut materials,
            &mut ranges,
        )
        .map_err(|error| error.to_string())?;
        for range in &ranges[..count] {
            let first = (range.first_index / 3) as usize;
            let end = first + (range.index_count / 3) as usize;
            surface_ids[first..end].fill(range.semantic_id);
        }
    }
    let motor = decode_iovr_motor(leaf.overrides.local_pga_motor);
    let scale = f64::from(leaf.overrides.uniform_scale);
    if !valid_motor(&motor) || !scale.is_finite() || scale <= 0.0 {
        return Err("Invalid sealed collision leaf placement".into());
    }
    Ok(Solid {
        vertices,
        faces: mesh.triangles,
        edges,
        surface_ids,
        rest: motor,
        scale,
        slot: leaf.slot,
    })
}

fn decode_iovr_motor(pga: [f32; 8]) -> Motor {
    let [s, x, y, z, dx, dy, dz, ds] = pga.map(f64::from);
    Motor {
        r_w: s,
        r_x: x,
        r_y: y,
        r_z: z,
        d_w: ds,
        d_x: -dx,
        d_y: -dy,
        d_z: -dz,
    }
}

fn validate_convex(vertices: &[V3], faces: &[[u32; 3]]) -> Result<Vec<[u32; 2]>, String> {
    if vertices
        .iter()
        .flatten()
        .any(|v| !v.is_finite() || v.abs() > 1e8)
    {
        return Err("Collision leaf contains coordinates outside its finite working range".into());
    }
    let points: Vec<_> = vertices.iter().copied().map(point).collect();
    let mut edges: BTreeMap<[u32; 2], (usize, i32)> = BTreeMap::new();
    let mut used = vec![false; vertices.len()];
    let mut inside_side = None;
    for &face in faces {
        let [a, b, c] = face.map(|i| points[i as usize]);
        let mut signs = points
            .iter()
            .map(|&p| orient_3d(a, b, c, p) as i32)
            .filter(|&sign| sign != 0);
        let first = signs
            .next()
            .ok_or("Convex collision boundary has a degenerate face or zero volume")?;
        if *inside_side.get_or_insert(first) != first || signs.any(|sign| sign != first) {
            return Err("Collision leaf must use consistently convex authored geometry".into());
        }
        for (a, b) in [(face[0], face[1]), (face[1], face[2]), (face[2], face[0])] {
            used[a as usize] = true;
            let entry = edges.entry([a.min(b), a.max(b)]).or_default();
            entry.0 += 1;
            entry.1 += if a < b { 1 } else { -1 };
        }
    }
    if used.contains(&false)
        || edges
            .values()
            .any(|&(count, orientation)| count != 2 || orientation != 0)
    {
        return Err("Collision leaf must be a closed consistently oriented triangle shell".into());
    }
    Ok(edges.into_keys().collect())
}

fn fit_hinge(
    fixed: &Solid,
    moving: &Solid,
    pairs: [[Anchor<'_>; 2]; 2],
) -> Result<(V3, V3, f64), String> {
    let anchor = |solid: &Solid, site: Anchor<'_>| {
        let face = solid
            .faces
            .get(site.face)
            .ok_or("Facet anchor is outside the authored collision mesh")?;
        if solid.surface_ids[site.face] != q_hash(site.iri) {
            return Err("Facet IRI differs from the anchored MAT surface".into());
        }
        let local: V3 = std::array::from_fn(|k| {
            (0..3)
                .map(|i| site.barycentric[i] * solid.vertices[face[i] as usize][k])
                .sum()
        });
        Ok::<_, String>(solid.rest.transform_point(local.map(|v| v * solid.scale)))
    };
    let [a, b] = pairs;
    let [af, am, bf, bm] = [
        anchor(fixed, a[0])?,
        anchor(moving, a[1])?,
        anchor(fixed, b[0])?,
        anchor(moving, b[1])?,
    ];
    let (rotation, translation) = moving.rest.to_rotation_translation();
    let rest_norm_sq = rotation.iter().map(|v| v * v).sum::<f64>();
    let inverse_rotation =
        [rotation[0], -rotation[1], -rotation[2], -rotation[3]].map(|v| v / rest_norm_sq.sqrt());
    let inverse = Motor::from_rotation_translation(inverse_rotation, [0.0; 3]);
    let to_local = |p| {
        inverse
            .transform_point(sub(p, translation))
            .map(|v| v / (moving.scale * rest_norm_sq))
    };
    let a = to_local(mul(add(af, am), 0.5));
    let b = to_local(mul(add(bf, bm), 0.5));
    let direction = normalize(sub(b, a))
        .ok_or("The two paired landmark midpoints do not define a hinge axis")?;
    if len(sub(b, a)) * moving.scale <= PIVOT_DRIFT_LIMIT_M {
        return Err("Paired facet stations are too close to define a hinge".into());
    }
    // Use the same binary32 components the inline Q42 consumer will decode.
    let axis = direction.map(|v| f64::from(v as f32));
    let pivot = mul(add(a, b), 0.5).map(|v| f64::from(v as f32));
    let spread = len(sub(af, am)).max(len(sub(bf, bm)));
    Ok((axis, pivot, spread))
}

fn point(v: V3) -> Point3 {
    Point3::new(v[0], v[1], v[2])
}
fn add(a: V3, b: V3) -> V3 {
    std::array::from_fn(|i| a[i] + b[i])
}
fn sub(a: V3, b: V3) -> V3 {
    std::array::from_fn(|i| a[i] - b[i])
}
fn mul(a: V3, scale: f64) -> V3 {
    a.map(|v| v * scale)
}
fn dot(a: V3, b: V3) -> f64 {
    (0..3).map(|i| a[i] * b[i]).sum()
}
fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn len(a: V3) -> f64 {
    dot(a, a).sqrt()
}
fn normalize(a: V3) -> Option<V3> {
    let length = len(a);
    (length > 0.0 && length.is_finite()).then(|| mul(a, 1.0 / length))
}

fn projection(points: &[V3], base: V3, axis: V3) -> [f64; 2] {
    points
        .iter()
        .map(|&p| dot(sub(p, base), axis))
        .fold([f64::INFINITY, f64::NEG_INFINITY], |[min, max], n| {
            [min.min(n), max.max(n)]
        })
}

fn sat(a: &Solid, av: &[V3], b: &Solid, bv: &[V3]) -> (f64, f64, f64) {
    let origin = av[0];
    let mut max_gap = f64::NEG_INFINITY;
    let mut test = |axis| {
        if let Some(axis) = normalize(axis) {
            let [a0, a1] = projection(av, origin, axis);
            let [b0, b1] = projection(bv, origin, axis);
            max_gap = max_gap.max((b0 - a1).max(a0 - b1));
        }
    };
    for (solid, points) in [(a, av), (b, bv)] {
        for &face in &solid.faces {
            let [x, y, z] = face.map(|i| points[i as usize]);
            test(cross(sub(y, x), sub(z, x)));
        }
    }
    for &[a0, a1] in &a.edges {
        let e = sub(av[a1 as usize], av[a0 as usize]);
        for &[b0, b1] in &b.edges {
            test(cross(e, sub(bv[b1 as usize], bv[b0 as usize])));
        }
    }
    let extent = av
        .iter()
        .chain(bv)
        .map(|&p| len(sub(p, origin)))
        .fold(0.0, f64::max);
    let epsilon = 1e-12_f64.max(512.0 * f64::EPSILON * extent);
    let volume_bound = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]
        .into_iter()
        .map(|axis| {
            let [a0, a1] = projection(av, origin, axis);
            let [b0, b1] = projection(bv, origin, axis);
            (a1.min(b1) - a0.max(b0) + epsilon).max(0.0)
        })
        .product::<f64>();
    (max_gap, epsilon, volume_bound)
}

fn measure_pair(
    a: &Solid,
    av: &[V3],
    b: &Solid,
    bv: &[V3],
    allowed: &BTreeSet<[u64; 2]>,
) -> Separation {
    let (gap, tolerance, volume) = sat(a, av, b, bv);
    let mut distance = f64::INFINITY;
    let mut allowed_hits = 0;
    let mut forbidden_hits = 0;
    for (i, &af) in a.faces.iter().enumerate() {
        let fa = af.map(|v| av[v as usize]);
        for (j, &bf) in b.faces.iter().enumerate() {
            let fb = bf.map(|v| bv[v as usize]);
            let [a0, a1, a2] = fa.map(point);
            let [b0, b1, b2] = fb.map(point);
            if gap <= tolerance && tri_tri_intersect_3(a0, a1, a2, b0, b1, b2).0 {
                distance = 0.0;
                if allowed.contains(&[a.surface_ids[i], b.surface_ids[j]]) {
                    allowed_hits += 1;
                } else {
                    forbidden_hits += 1;
                }
            } else if distance > 0.0 {
                distance = distance.min(triangle_distance_sq(fa, fb));
            }
        }
    }
    Separation {
        gap,
        tolerance,
        distance: distance.sqrt(),
        overlap_bound: if gap > tolerance { 0.0 } else { volume },
        allowed_hits,
        forbidden_hits,
    }
}

fn triangle_distance_sq(a: [V3; 3], b: [V3; 3]) -> f64 {
    let [a0, a1, a2] = a.map(point);
    let [b0, b1, b2] = b.map(point);
    let face_distances = [a0, a1, a2]
        .into_iter()
        .map(|p| point_triangle_distance_sq_3d(p, b0, b1, b2))
        .chain(
            [b0, b1, b2]
                .into_iter()
                .map(|p| point_triangle_distance_sq_3d(p, a0, a1, a2)),
        );
    let mut best = face_distances.fold(f64::INFINITY, f64::min);
    for i in 0..3 {
        for j in 0..3 {
            best = best.min(segment_distance_sq(
                a[i],
                a[(i + 1) % 3],
                b[j],
                b[(j + 1) % 3],
            ));
        }
    }
    best
}

fn segment_distance_sq(a: V3, b: V3, c: V3, d: V3) -> f64 {
    let [pa, pb, pc, pd] = [a, b, c, d].map(point);
    let u = sub(b, a);
    let v = sub(d, c);
    let w = sub(a, c);
    let n = cross(u, v);
    let determinant = dot(n, n);
    if determinant > 0.0 {
        let u_time = dot(cross(v, w), n) / determinant;
        let v_time = dot(cross(u, w), n) / determinant;
        if (0.0..=1.0).contains(&u_time) && (0.0..=1.0).contains(&v_time) {
            let delta = sub(add(w, mul(u, u_time)), mul(v, v_time));
            return dot(delta, delta);
        }
    }
    [
        point_segment_distance_sq_3d(pa, pc, pd),
        point_segment_distance_sq_3d(pb, pc, pd),
        point_segment_distance_sq_3d(pc, pa, pb),
        point_segment_distance_sq_3d(pd, pa, pb),
    ]
    .into_iter()
    .fold(f64::INFINITY, f64::min)
}

struct Measurements {
    status: &'static str,
    poses: Vec<Json>,
    clearance_lower_bound: f64,
    pivot_drift: f64,
}

fn measure_sweep(
    geometry: &Geometry,
    poses: &[Pose],
    axis: V3,
    pivot: V3,
    allowance: f64,
    zones: &[[&str; 2]],
    voids: &[(&str, f64)],
) -> Result<Measurements, String> {
    let axis = normalize(axis).ok_or("Invalid fitted hinge direction")?;
    let fixed = geometry.fixed.world(&Motor::identity());
    let protected: Vec<_> = geometry
        .protected
        .iter()
        .map(|solid| solid.world(&Motor::identity()))
        .collect();
    let zones = zones
        .iter()
        .map(|zone| zone.map(q_hash))
        .collect::<BTreeSet<_>>();
    let rest_rot = geometry.moving.rest.to_rotation_translation().0;
    let world_scale = geometry.moving.scale * rest_rot.iter().map(|v| v * v).sum::<f64>();
    let radius = geometry
        .moving
        .vertices
        .iter()
        .map(|&p| len(cross(sub(p, pivot), axis)))
        .fold(0.0, f64::max)
        * world_scale;
    let mut output = Vec::with_capacity(poses.len());
    let mut samples = Vec::with_capacity(poses.len());
    let mut fail = false;
    let mut pivot_drift = 0.0_f64;
    for pose in poses {
        let (sin, cos) = (pose.angle * 0.5).sin_cos();
        let q = [cos, axis[0] * sin, axis[1] * sin, axis[2] * sin];
        let spin = Motor::from_rotation_translation(q, [0.0; 3]);
        let offset = sub(pivot, spin.transform_point(pivot));
        let delta = Motor::from_rotation_translation(q, offset);
        let ideal = geometry.moving.world(&delta);
        if ideal
            .iter()
            .flatten()
            .chain(fixed.iter().flatten())
            .chain(protected.iter().flatten().flatten())
            .any(|v| !v.is_finite() || v.abs() > 1e8)
        {
            return Err(
                "Articulation placement is outside the supported finite coordinate range".into(),
            );
        }
        pivot_drift = pivot_drift.max(len(sub(pivot, delta.transform_point(pivot))) * world_scale);
        let io = geometry.moving.rest.mul(&Motor::from_rotation_translation(
            q,
            mul(offset, geometry.moving.scale),
        ));
        let iovr = [
            io.r_w, io.r_x, io.r_y, io.r_z, -io.d_x, -io.d_y, -io.d_z, io.d_w,
        ]
        .map(|v| v as f32);
        let stored = decode_iovr_motor(iovr);
        if !valid_motor(&stored) {
            return Err("Pose cannot be encoded as a native IOVR motor".into());
        }
        let moved: Vec<_> = geometry
            .moving
            .vertices
            .iter()
            .map(|&point| stored.transform_point(mul(point, geometry.moving.scale)))
            .collect();
        let quantization_error = ideal
            .iter()
            .zip(&moved)
            .map(|(&a, &b)| len(sub(a, b)))
            .fold(0.0, f64::max);
        let scaled_pivot = mul(pivot, geometry.moving.scale);
        let encoded_pivot_drift = len(sub(
            geometry.moving.rest.transform_point(scaled_pivot),
            stored.transform_point(scaled_pivot),
        ));
        let pair = measure_pair(&geometry.fixed, &fixed, &geometry.moving, &moved, &zones);
        let mut clear = pair.gap - pair.tolerance - allowance;
        fail |= pair.gap < -pair.tolerance
            || pair.forbidden_hits > 0
            || pair.distance + pair.tolerance < allowance;
        let mut void_samples = Vec::with_capacity(voids.len());
        for ((solid, positions), (identity, gap_required)) in
            geometry.protected.iter().zip(&protected).zip(voids)
        {
            let guard = measure_pair(solid, positions, &geometry.moving, &moved, &BTreeSet::new());
            clear = clear.min(guard.gap - guard.tolerance - gap_required);
            fail |= guard.gap < -guard.tolerance
                || guard.forbidden_hits > 0
                || guard.distance + guard.tolerance < *gap_required;
            void_samples.push(json!({"leaf_iri": identity, "surface_clearance_mm": guard.distance * 1000.0,
                "separation_lower_bound_mm": (guard.gap - guard.tolerance) * 1000.0, "surface_intersection_pairs": guard.forbidden_hits}));
        }
        samples.push(clear - quantization_error);
        output.push(json!({"seconds": pose.seconds, "angle_rad": pose.angle, "local_pga_motor": iovr,
            "iovr_quantization_displacement_mm": quantization_error * 1000.0,
            "iovr_pivot_drift_m": encoded_pivot_drift,
            "surface_clearance_mm": pair.distance * 1000.0, "penetration_sat_mm": (-pair.gap).max(0.0) * 1000.0,
            "penetration_volume_upper_bound_cm3": 1e6 * pair.overlap_bound,
            "allowed_surface_intersection_pairs": pair.allowed_hits, "forbidden_surface_intersection_pairs": pair.forbidden_hits,
            "protected_voids": void_samples}));
    }
    if pivot_drift > PIVOT_DRIFT_LIMIT_M {
        return Err(format!(
            "Fitted pivot drift {pivot_drift:.12e} m exceeds {PIVOT_DRIFT_LIMIT_M:.12e} m"
        ));
    }
    let clearance_lower_bound = poses
        .windows(2)
        .zip(samples.windows(2))
        .map(|(pose, clear)| {
            clear[0].min(clear[1]) - radius * (pose[1].angle - pose[0].angle).abs() * 0.5
        })
        .fold(f64::INFINITY, f64::min);
    let status = if fail {
        "fail"
    } else if clearance_lower_bound > 0.0 {
        "pass"
    } else {
        "unverified"
    };
    Ok(Measurements {
        status,
        poses: output,
        clearance_lower_bound,
        pivot_drift,
    })
}

fn seal_receipt(
    receipt: &str,
    context: &str,
    moving: &str,
    status: &str,
    axis: V3,
    pivot: V3,
    rom: [f64; 2],
    source_digest: &str,
    limit: usize,
) -> Result<Vec<u8>, String> {
    let mut model = SemanticModelBuilder::new(limit.min(Q42_BUDGET));
    let mut rows = Vec::with_capacity(16);
    let context = model.bind_iri(context).map_err(|error| error.to_string())?;
    let target = model.bind_iri(moving).map_err(|error| error.to_string())?;
    let receipt = model.bind_iri(receipt).map_err(|error| error.to_string())?;
    let status = match status {
        "pass" => "EngineeringVerified",
        "fail" => "EngineeringFailed",
        _ => "EngineeringUnverified",
    };
    let status_id = model
        .bind_iri(&format!("{NS}{status}"))
        .map_err(|error| error.to_string())?;
    let clinical_review = model
        .bind_iri(&format!("{NS}PendingClinicalReview"))
        .map_err(|error| error.to_string())?;
    let source = model
        .bind_iri(&format!("urn:sha256:{source_digest}"))
        .map_err(|error| error.to_string())?;
    let mut push = |subject, property: &str, object| -> Result<(), String> {
        let predicate = model
            .bind_iri(&format!("{NS}{property}"))
            .map_err(|error| error.to_string())?;
        let parity = NQuin::calculate_parity(subject, predicate, object, context, 0);
        rows.push(NQuin {
            subject,
            predicate,
            object,
            context,
            metadata: 0,
            parity,
        });
        Ok(())
    };
    push(target, "fitReceipt", receipt)?;
    push(receipt, "target", target)?;
    push(receipt, "engineeringGeometryStatus", status_id)?;
    push(receipt, "anatomicalReviewStatus", clinical_review)?;
    push(receipt, "sourceBundleDigest", source)?;
    for (property, number) in [
        "axisX",
        "axisY",
        "axisZ",
        "pivotXMetres",
        "pivotYMetres",
        "pivotZMetres",
        "romMinRadians",
        "romMaxRadians",
        "contactMinRadians",
        "contactMaxRadians",
    ]
    .into_iter()
    .zip(axis.into_iter().chain(pivot).chain(rom).chain(rom))
    {
        push(
            receipt,
            property,
            INLINE_TAG_FLOAT | u64::from((number as f32).to_bits()),
        )?;
    }
    model
        .seal_projection(rows)
        .map_err(|error| error.to_string())
}
