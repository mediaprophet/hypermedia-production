//! Synthetic chamber illustration and rectangular section resection.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::f64::consts::{PI, TAU};

use qualia_core_db::bundle::BundleReader;
use qualia_core_db::container_10d::sdf_brick_section::{
    read_container_sdfb_with_iris, SdfBrickKey, SdfBrickView,
};
use qualia_core_db::q_hash;
use qualia_core_db::render::portal_load_10d::admit_leaf_assembly;
use qualia_core_db::render::skeletal::Motor;

use crate::{count, fields, list, number, record, required, text, vector, Args, AssetHost, Value};

const ENTITY_PREFIX: &str = "urn:qualia:synthetic-chambers:";
const ASSEMBLY_KEY: &str = "model/assembly.10d";
const CLASS_IRI: &str = "urn:qualia:synthetic-chambers:HollowEllipsoid";
const SECTION_CLASS_IRI: &str = "urn:qualia:synthetic-chambers:SectionedSolid";
const RINGS: u32 = 12;
const SECTORS: u32 = 24;

struct Chamber {
    center: [f64; 3],
    radii: [f64; 3],
    inner: [f64; 3],
    padding: f64,
}

#[derive(Default)]
struct FixtureParts {
    definitions: Vec<Value>,
    placements: Vec<Value>,
    assets: Vec<Value>,
}

impl FixtureParts {
    fn add(&mut self, key: &str, instance: &str, class: &str, center: [f64; 3], resource: Value) {
        let [x, y, z] = center;
        let m = Motor::from_translation(x, y, z);
        let pga = [m.r_w, m.r_x, m.r_y, m.r_z, -m.d_x, -m.d_y, -m.d_z, m.d_w];
        self.definitions.push(Value::Record(Args::from([
            ("leaf_key".into(), string(key)),
            ("resource".into(), resource.clone()),
            ("taxon_or_class_iri".into(), string(class)),
        ])));
        self.placements.push(Value::Record(Args::from([
            ("leaf_key".into(), string(key)),
            ("entity_iri".into(), string(instance)),
            (
                "local_pga_motor".into(),
                Value::List(pga.into_iter().map(Value::F64).collect()),
            ),
        ])));
        self.assets.push(package_row(key, resource));
    }
}

impl AssetHost<'_> {
    pub(crate) fn synthesize_organ_system_10d(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "entity_iri",
                "container_half_extents_m",
                "clearance_m",
                "grid_resolution",
                "chambers",
            ],
        )?;
        if text(required(args, "mode")?)? != "synthetic_chamber_illustration" {
            return Err("Expected synthetic_chamber_illustration mode".into());
        }
        let entity = fixture_entity(args)?;
        let extents: [f64; 3] = vector(required(args, "container_half_extents_m")?)?;
        if extents.iter().any(|extent| !(0.10..=10.0).contains(extent)) {
            return Err("Container half-extents must be in [0.10, 10] metres".into());
        }
        let clearance = number(args, "clearance_m", 0.01)?;
        if !(0.001..=1.0).contains(&clearance) {
            return Err("Chamber clearance must be in [0.001, 1] metres".into());
        }
        let resolution = count(args, "grid_resolution", 40, 64)?;
        if resolution < 32 || resolution % 8 != 0 {
            return Err("grid_resolution must be a multiple of eight in [32, 64]".into());
        }
        let rows = list(required(args, "chambers")?)?;
        if !(2..=8).contains(&rows.len()) {
            return Err("A synthetic assembly requires 2 to 8 chambers".into());
        }
        let mut chambers: Vec<Chamber> = Vec::with_capacity(rows.len());
        for value in rows {
            let row = record(value)?;
            fields(row, &["center_m", "radii_m", "wall_m"])?;
            let center: [f64; 3] = vector(required(row, "center_m")?)?;
            let radii: [f64; 3] = vector(required(row, "radii_m")?)?;
            let wall = number(row, "wall_m", 0.0)?;
            let largest = radii.iter().copied().fold(0.0, f64::max);
            let smallest = radii.iter().copied().fold(f64::INFINITY, f64::min);
            if smallest < 0.01 || largest > 2.0 || smallest < 0.3 * largest {
                return Err("Expected bounded ellipsoid radii in metres and aspect ratio no greater than 10:3".into());
            }
            let padding = largest / 4.0;
            let max_step = 2.0 * (largest + padding) / (resolution - 1) as f64;
            if wall < 3.0 * max_step || smallest - wall < 3.0 * max_step {
                return Err("The requested grid must resolve at least three samples across the wall and inner radius".into());
            }
            // Leave room for both F32 vertex and packed PGA translation rounding.
            let guard = clearance + 1e-5;
            if (0..3).any(|axis| center[axis].abs() + radii[axis] + guard > extents[axis]) {
                return Err(
                    "Every chamber must fit inside the authored envelope with clearance".into(),
                );
            }
            if chambers.iter().any(|prior| {
                !(0..3).any(|axis| {
                    (prior.center[axis] - center[axis]).abs()
                        >= prior.radii[axis] + radii[axis] + guard
                })
            }) {
                return Err("The chambers' axis-aligned bounds must be separated by the requested clearance".into());
            }
            chambers.push(Chamber {
                center,
                radii,
                inner: radii.map(|radius| radius - wall),
                padding,
            });
        }

        let frame = self.fit_fixture_box(extents, entity)?;
        let capacity = Self::assembly_frame_leaf_capacity(&self.workspace.encoded(&frame)?.bytes)?;
        if chambers.len() > capacity {
            return Err("The assembly frame lacks fields for the requested chambers".into());
        }

        let mut parts = FixtureParts::default();
        let mut summary = Vec::with_capacity(chambers.len());
        for (index, chamber) in chambers.iter().enumerate() {
            self.charge(0, 0)?;
            let key = format!("model/chamber-{index:02}.10d");
            let leaf = format!("{entity}:chamber:{index}");
            let instance = format!("{leaf}:instance");
            let wall = format!("{leaf}:wall");
            let lumen = format!("{leaf}:lumen");
            let mut vertices = Vec::new();
            let mut triangles = Vec::new();
            append_ellipsoid(chamber.radii, false, &mut vertices, &mut triangles);
            append_ellipsoid(chamber.inner, true, &mut vertices, &mut triangles);
            let geometry = self.store_mesh_positions(vertices, triangles)?;
            let sealed = self.manifold_fields_sdf_op(
                "hypermedia.manifold.seal_sdf_brick_10d",
                &Args::from([
                    ("mesh".into(), geometry),
                    ("entity_iri".into(), string(&leaf)),
                    ("wall_iri".into(), string(&wall)),
                    ("lumen_iri".into(), string(&lumen)),
                    ("resolution".into(), Value::U64(resolution as u64)),
                    ("padding".into(), Value::F64(chamber.padding)),
                ]),
            )?;
            let resource = required(record(&sealed)?, "asset")?.clone();
            let bytes = &self.workspace.encoded(&resource)?.bytes;
            let (sdfb, terms) = read_container_sdfb_with_iris(bytes)?
                .ok_or("Generated chamber is missing its SDFB field")?;
            if terms.get(&q_hash(&wall)).map(String::as_str) != Some(wall.as_str())
                || terms.get(&q_hash(&lumen)).map(String::as_str) != Some(lumen.as_str())
            {
                return Err(
                    "Generated chamber is missing canonical wall or lumen identities".into(),
                );
            }
            verify_lumen_probe(sdfb, chamber.radii)?;

            parts.add(&key, &instance, CLASS_IRI, chamber.center, resource);
            summary.push(serde_json::json!({
                "slot": index, "entry_key": key, "leaf_iri": leaf, "instance_iri": instance,
                "wall_iri": wall, "lumen_iri": lumen, "center_m": chamber.center,
                "radii_m": chamber.radii, "inner_radii_m": chamber.inner,
            }));
        }
        self.finish_fixture(
            frame,
            parts,
            serde_json::json!({
                "scope": "synthetic_chamber_illustration",
                "assembly_key": ASSEMBLY_KEY,
                "container_half_extents_m": extents,
                "clearance_m": clearance,
                "grid_resolution": resolution,
                "chambers": summary,
            }),
        )
    }

    pub(crate) fn incise_resect_and_suture(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &["mode", "entity_iri", "half_extents_m", "axis", "interval_m"],
        )?;
        if text(required(args, "mode")?)? != "synthetic_section_resection" {
            return Err(
                "Expected synthetic_section_resection mode over a rectangular fixture".into(),
            );
        }
        let entity = fixture_entity(args)?;
        let extents: [f64; 3] = vector(required(args, "half_extents_m")?)?;
        let axis = crate::integer(required(args, "axis")?, "axis")?;
        let [low, high] = vector(required(args, "interval_m")?)?;
        if axis > 2 || extents.iter().any(|extent| !(0.01..=10.0).contains(extent)) {
            return Err("Use axis 0, 1 or 2 and half-extents in [0.01, 10] metres".into());
        }
        let span = extents[axis];
        if low + span < 0.002 || high - low < 0.002 || span - high < 0.002 {
            return Err("The internal interval and both retained solids require lengths of at least 0.002 metres".into());
        }
        let frame = self.fit_fixture_box(extents, entity)?;
        let mut parts = FixtureParts::default();
        let cross_section: f64 = extents
            .iter()
            .enumerate()
            .filter_map(|(i, value)| (i != axis).then_some(value * 2.0))
            .product();
        let mut kept_volumes = [0.0; 2];
        for (index, [from, to]) in [[-span, low], [high, span]].into_iter().enumerate() {
            let mut half = extents;
            let mut center = [0.0; 3];
            half[axis] = (to - from) / 2.0;
            center[axis] = (to + from) / 2.0;
            let leaf = format!("{entity}:section:{index}");
            let resource = self.fit_fixture_box(half, &leaf)?;
            parts.add(
                &format!("model/section-{index}.10d"),
                &format!("{leaf}:instance"),
                SECTION_CLASS_IRI,
                center,
                resource,
            );
            kept_volumes[index] = (to - from) * cross_section;
        }
        let removed_volume = (high - low) * cross_section;
        let original_volume = 2.0 * span * cross_section;
        let volume_residual = kept_volumes.iter().sum::<f64>() + removed_volume - original_volume;
        if volume_residual.abs() > original_volume * 1e-12 {
            return Err("Analytic section volume conservation failed".into());
        }
        self.finish_fixture(
            frame,
            parts,
            serde_json::json!({
                "scope": "synthetic_section_resection",
                "assembly_key": ASSEMBLY_KEY,
                "axis": axis,
                "interval_m": [low, high],
                "half_extents_m": extents,
                "analytic_original_volume_m3": original_volume,
                "analytic_removed_volume_m3": removed_volume,
                "analytic_retained_volumes_m3": kept_volumes,
                "analytic_volume_residual_m3": volume_residual,
                "retained_components": 2,
                "new_closed_cut_faces": 2,
            }),
        )
    }

    fn fit_fixture_box(&mut self, extents: [f64; 3], entity: &str) -> Result<Value, String> {
        let mesh = self.store_mesh_positions(
            (0..8)
                .map(|i| {
                    std::array::from_fn(|axis| {
                        let sign = if i & (1 << axis) == 0 { -1.0 } else { 1.0 };
                        (extents[axis] * sign) as f32
                    })
                })
                .collect(),
            vec![
                [0, 2, 1],
                [1, 2, 3],
                [4, 5, 6],
                [5, 7, 6],
                [0, 1, 4],
                [1, 5, 4],
                [2, 6, 3],
                [3, 6, 7],
                [0, 4, 2],
                [2, 4, 6],
                [1, 3, 5],
                [3, 7, 5],
            ],
        )?;
        self.manifold_fields_sdf_op(
            "hypermedia.manifold.fit_holographic_v2",
            &Args::from([("mesh".into(), mesh), ("entity_iri".into(), string(entity))]),
        )
    }

    fn finish_fixture(
        &mut self,
        frame: Value,
        parts: FixtureParts,
        report: serde_json::Value,
    ) -> Result<Value, String> {
        let FixtureParts {
            definitions,
            placements,
            mut assets,
        } = parts;
        let leaf_count = placements.len();
        let assembly_result = self.compose_assembly_10d(&Args::from([
            ("assembly_frame".into(), frame),
            ("archetypes".into(), Value::List(definitions)),
            ("instances".into(), Value::List(placements)),
        ]))?;
        let mut result = record(&assembly_result)?.clone();
        assets.push(package_row(
            ASSEMBLY_KEY,
            required(&result, "assembly")?.clone(),
        ));
        let packed =
            self.pack_and_diff_hmc(&Args::from([("bundle".into(), Value::List(assets))]))?;
        let hmc = required(record(&packed)?, "hmc")?.clone();
        let bundle = BundleReader::parse(&self.workspace.encoded(&hmc)?.bytes)
            .map_err(|error| error.to_string())?;
        if admit_leaf_assembly(&bundle, ASSEMBLY_KEY)?.len() != leaf_count {
            return Err("The sealed illustration has an unexpected native leaf count".into());
        }
        let report = serde_json::to_vec(&report).map_err(|error| error.to_string())?;
        result.insert("hmc".into(), hmc);
        result.insert("assembly_key".into(), string(ASSEMBLY_KEY));
        result.insert("report".into(), self.admit_encoded("json", report)?);
        Ok(Value::Record(result))
    }
}

fn fixture_entity(args: &Args) -> Result<&str, String> {
    let entity = text(required(args, "entity_iri")?)?;
    let label = entity
        .strip_prefix(ENTITY_PREFIX)
        .ok_or("Synthetic fixture identity must use urn:qualia:synthetic-chambers:")?;
    if label.is_empty()
        || label.len() > 128
        || !label
            .chars()
            .all(|ch| ch.is_alphanumeric() || matches!(ch, '.' | '-' | '_'))
        || q_hash(entity) == 0
    {
        return Err("Invalid synthetic fixture identity label".into());
    }
    Ok(entity)
}

fn string(value: &str) -> Value {
    Value::String(value.into())
}

fn package_row(key: &str, resource: Value) -> Value {
    Value::Record(Args::from([
        ("key".into(), string(key)),
        ("resource".into(), resource),
    ]))
}

fn append_ellipsoid(
    radius: [f64; 3],
    invert: bool,
    vertices: &mut Vec<[f32; 3]>,
    triangles: &mut Vec<[u32; 3]>,
) {
    let top = vertices.len() as u32;
    vertices.push([0.0, radius[1] as f32, 0.0]);
    for ring in 1..RINGS {
        let (s, c) = (PI * f64::from(ring) / f64::from(RINGS)).sin_cos();
        for sector in 0..SECTORS {
            let (az_s, az_c) = (TAU * f64::from(sector) / f64::from(SECTORS)).sin_cos();
            vertices.push([
                (radius[0] * s * az_c) as f32,
                (radius[1] * c) as f32,
                (radius[2] * s * az_s) as f32,
            ]);
        }
    }
    let bottom = vertices.len() as u32;
    vertices.push([0.0, -radius[1] as f32, 0.0]);
    let index = |ring: u32, sector: u32| top + 1 + (ring - 1) * SECTORS + sector % SECTORS;
    let mut add = |mut triangle: [u32; 3]| {
        if invert {
            triangle.swap(1, 2);
        }
        triangles.push(triangle);
    };
    for sector in 0..SECTORS {
        add([top, index(1, sector + 1), index(1, sector)]);
        for ring in 1..RINGS - 1 {
            let a = index(ring, sector);
            let b = index(ring, sector + 1);
            let c = index(ring + 1, sector);
            let d = index(ring + 1, sector + 1);
            add([a, b, c]);
            add([b, d, c]);
        }
        add([
            index(RINGS - 1, sector),
            index(RINGS - 1, sector + 1),
            bottom,
        ]);
    }
}

fn verify_lumen_probe(view: SdfBrickView<'_>, radii: [f64; 3]) -> Result<(), String> {
    let grid = view.grid();
    let axis = (0..3)
        .max_by(|&a, &b| radii[a].total_cmp(&radii[b]))
        .ok_or("Chamber probe must have a longitudinal axis")?;
    let mut cell: [u32; 3] =
        std::array::from_fn(|i| (-grid.origin_m[i] / grid.voxel_spacing_m[i]).round() as u32);
    let mut saw_wall = false;
    let mut saw_lumen = false;
    for sample in 0..grid.base_bricks[axis] * 8 {
        cell[axis] = sample;
        let Some(brick) = view.brick(SdfBrickKey {
            coordinate: cell.map(|coordinate| coordinate / 8),
            refinement_level: 0,
        }) else {
            continue;
        };
        let [x, y, z] = cell.map(|coordinate| (coordinate % 8) as usize);
        let index = x + 8 * y + 64 * z;
        if brick.occupied(index) == Some(true) {
            if saw_lumen {
                return Ok(());
            }
            saw_wall = true;
        } else if saw_wall
            && brick
                .distance_m(index)
                .is_some_and(|distance| distance > 0.0)
        {
            saw_lumen = true;
        }
    }
    Err("Encoded SDFB probe failed to resolve wall, enclosed lumen, and opposite wall".into())
}
