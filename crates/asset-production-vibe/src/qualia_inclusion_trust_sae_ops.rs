//! Authored six-dot tactile plates and accessible semantic bindings.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    artifact::AttributedMesh, fields, list, num, required, text, vector, Args, AssetHost,
    TypedArtifact, Value,
};
use qualia_core_db::{
    bundle::semantic_model::SemanticModelBuilder,
    container_10d::{self as ten, sdf_brick_section as sdf},
    q_hash,
    render::{
        assets::{ImportedMesh, Mesh},
        compile_10d::upgrade_legacy_10d_to_holographic_v2,
        material_compile::compile_mesh_to_10d_with_materials,
    },
    specialized_libs::computational_geometry::{
        indexed_tetra_surface::{indexed_tetra_surface_capped_into, TetraEdgeSlot},
        Point3,
    },
    NQuin,
};
use serde_json::json;
use std::{collections::BTreeMap, mem::size_of, sync::Arc};

const MAX_AXIS: usize = 256;
const MAX_VOXELS: usize = 1_500_000;
const MAX_VERTICES: usize = 200_000;
const MAX_TRIANGLES: usize = 400_000;
const EDGE_SLOTS: usize = 1 << 20;
const MAX_10D_BYTES: usize = 128 << 20;
const GRAPH: &str = "https://qualiadb.org/contexts/authored-tactile-braille-v1";
const KIND: &str = "https://qualiadb.org/ontology/accessibility#TactileBraillePlate";
const TRANSCRIPTION: &str = "https://qualiadb.org/ontology/accessibility#brailleTranscription";
const TRANSCRIPTION_SYSTEM: &str =
    "https://qualiadb.org/ontology/accessibility#transcriptionSystem";
const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const MATERIAL_IRI: &str = "urn:qualia:material:legacy-default";

struct Plate {
    diameter: f64,
    height: f64,
    pitch: f64,
    cell_pitch: f64,
    line_pitch: f64,
    thickness: f64,
    margin: f64,
    width: f64,
    depth: f64,
    radius: f64,
    dots: Vec<[f64; 2]>,
    braille: String,
}

struct Sampling {
    grid: sdf::SdfBrickGrid,
    samples: [usize; 3],
    origin_mm: [f64; 3],
    stride_mm: [f64; 3],
    band_mm: f64,
}

impl AssetHost<'_> {
    pub(crate) fn synthesize_iso17049_tactile_braille(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "braille_lines",
                "accessible_text",
                "entity_iri",
                "language_bcp47",
                "transcription_system_iri",
                "dot_diameter_mm",
                "dot_height_mm",
                "dot_pitch_mm",
                "cell_pitch_mm",
                "line_pitch_mm",
                "plate_thickness_mm",
                "margin_mm",
                "voxel_spacing_mm",
            ],
        )?;
        let entity = text(required(args, "entity_iri")?)?;
        let label = text(required(args, "accessible_text")?)?;
        if entity.is_empty() || entity.len() > 4096 {
            return Err("entity_iri must be a bounded canonical RDF IRI".into());
        }
        if label.trim().is_empty()
            || label.len() > 8192
            || label
                .chars()
                .any(|ch| ch.is_control() && !matches!(ch, '\n' | '\r' | '\t'))
        {
            return Err(
                "accessible_text must contain authored UTF-8 text (up to 8192 bytes)".into(),
            );
        }
        let language = args.get("language_bcp47").map(text).transpose()?;
        if language.is_some_and(|tag| !language_tag_syntax(tag)) {
            return Err("language_bcp47 requires a bounded hyphen-delimited language tag".into());
        }
        let system = args.get("transcription_system_iri").map(text).transpose()?;
        if system.is_some_and(|iri| iri.is_empty() || iri.len() > 4096) {
            return Err("transcription_system_iri must be a bounded canonical RDF IRI".into());
        }
        let plate = Plate::from_args(args)?;
        let sampling = Sampling::from_args(args, &plate)?;
        let identity = q_hash(entity);
        let surface_iri = format!("urn:qualia:tactile-plate:{identity:016x}:surface");
        let void_role_iri = format!("urn:qualia:tactile-plate:{identity:016x}:unused-void-role");
        let transcription_iri = format!("urn:qualia:tactile-plate:{identity:016x}:braille");
        let wall = q_hash(&surface_iri);
        let void_role = q_hash(&void_role_iri);
        let labels = build_labels(
            entity,
            label,
            language,
            system,
            &transcription_iri,
            &void_role_iri,
            &plate.braille,
        )?;
        SemanticModelBuilder::new(64 << 10)
            .merge_cbld(&labels)
            .map_err(|error| error.to_string())?;

        let node_count = sampling.samples.into_iter().product::<usize>();
        let brick_count = node_count / sdf::SDFB_SAMPLES;
        let field_len =
            sdf::sdfb_openvdb_encoded_len(brick_count).map_err(|error| error.to_string())?;
        let scratch = node_count * 8
            + brick_count
                * (size_of::<sdf::SdfBrickRecord>() + size_of::<sdf::OpenVdbActiveMask>())
            + field_len
            + MAX_VERTICES * (size_of::<Point3>() + size_of::<[usize; 2]>())
            + MAX_TRIANGLES * size_of::<[u32; 3]>()
            + EDGE_SLOTS * size_of::<TetraEdgeSlot>();
        self.charge(node_count * 30, scratch)?;
        let (records, masks, distances) = bake(self, &plate, &sampling, wall, void_role, scratch)?;
        let mut field = vec![0; field_len];
        sdf::encode_sdfb_with_openvdb_active_masks_into(
            sampling.grid,
            &records,
            &masks,
            &mut field,
        )
        .map_err(|error| error.to_string())?;
        drop(records);
        drop(masks);
        let mesh = extract_plate_mesh(&sampling, &distances)?;
        drop(distances);
        self.charge(mesh.triangles.len() * 20, scratch)?;

        let material = ten::MaterialRecord::legacy_default();
        let ranges = [ten::SubmeshRange {
            first_index: 0,
            index_count: (mesh.triangles.len() * 3) as u32,
            material_id: material.id,
            semantic_id: wall,
        }];
        let compiled = compile_mesh_to_10d_with_materials(&mesh, &[material], &ranges)
            .map_err(|error| format!("Tactile plate mesh sealing: {error:?}"))?;
        let [x, y, z] = mesh.centroid();
        let holo = upgrade_legacy_10d_to_holographic_v2(
            &compiled,
            entity,
            1,
            [0.0, 3.0, 2.0, x, y, z, 0.0, 1.0, 1.0, 0.5],
        )
        .map_err(|error| format!("Tactile plate HOLO sealing: {error:?}"))?;
        let with_sdf = sdf::seal_10d_with_sdfb(&holo, &field, &surface_iri, &void_role_iri)?;
        let asset = attach_labels(&with_sdf, &labels)?;
        let summary = json!({
            "schema": "qualia.authored-tactile-plate/v1",
            "entity_iri": entity, "surface_iri": surface_iri, "transcription_iri": transcription_iri,
            "dot_count": plate.dots.len(), "dot_diameter_mm": plate.diameter, "dot_height_mm": plate.height,
            "cap_sphere_radius_mm": plate.radius, "dot_pitch_mm": plate.pitch,
            "cell_pitch_mm": plate.cell_pitch, "line_pitch_mm": plate.line_pitch,
            "plate_dimensions_mm": [plate.width, plate.depth, plate.thickness],
            "margin_mm": plate.margin, "voxel_spacing_mm": sampling.stride_mm,
            "sdfb_inner_version": 3, "cavity_present": false,
            "proxy_triangle_count": mesh.triangles.len(),
            "transcription_source": "authored_unicode_braille",
            "validation": "authored_dimensions_and_geometry_bounds",
        });
        let receipt = serde_json::to_vec(&summary).map_err(|error| error.to_string())?;
        let mesh = ImportedMesh {
            mesh,
            normals: None,
            tangents: None,
            texture_coordinates_0: None,
            materials: vec![material],
            texture_dependencies: Vec::new(),
            submeshes: ranges.to_vec(),
        };
        let asset = self.admit_encoded("10d", asset)?;
        let field = self.admit_encoded("sdfb", field)?;
        let mesh =
            self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(AttributedMesh {
                surface: mesh,
                uv1: Vec::new(),
                vertex_colors: Vec::new(),
                skin_weights: Vec::new(),
                morph_targets: Arc::from(Vec::new()),
                protected_faces: Vec::new(),
            })))?;
        let receipt = self.admit_encoded("json", receipt)?;
        Ok(Value::Record(BTreeMap::from([
            ("asset".into(), asset),
            ("field".into(), field),
            ("mesh".into(), mesh),
            ("receipt".into(), receipt),
            ("dot_count".into(), Value::U64(plate.dots.len() as u64)),
        ])))
    }
}

impl Plate {
    fn from_args(args: &Args) -> Result<Self, String> {
        let diameter = number(args, "dot_diameter_mm", None, 0.8, 2.5)?;
        let height = number(args, "dot_height_mm", None, 0.15, diameter * 0.5)?;
        let thickness = number(args, "plate_thickness_mm", None, 0.6, 8.0)?;
        let margin = number(args, "margin_mm", Some(6.0), 6.0, 30.0)?;
        let pitch = number(args, "dot_pitch_mm", Some(2.5), diameter + 0.15, 5.0)?;
        let cell_pitch = number(
            args,
            "cell_pitch_mm",
            Some(6.0),
            pitch + diameter + 0.15,
            15.0,
        )?;
        let line_pitch = number(
            args,
            "line_pitch_mm",
            Some(10.0),
            2.0 * pitch + diameter + 0.15,
            25.0,
        )?;
        let lines = list(required(args, "braille_lines")?)?;
        if !(1..=12).contains(&lines.len()) {
            return Err("braille_lines requires 1 to 12 authored lines".into());
        }
        let line_count = lines.len();
        let braille = lines.iter().map(text).collect::<Result<Vec<_>, _>>()?;
        let depth = 2.0 * margin + diameter + (line_count - 1) as f64 * line_pitch + 2.0 * pitch;
        let mut cols = 0;
        let mut dots = Vec::new();
        for (line, cells) in braille.iter().enumerate() {
            let count = cells.chars().count();
            if !(1..=32).contains(&count) {
                return Err("Each authored Braille line requires 1 to 32 Unicode cells".into());
            }
            cols = cols.max(count);
            let top = depth - margin - diameter * 0.5 - line as f64 * line_pitch;
            for (column, cell) in cells.chars().enumerate() {
                let code = cell as u32;
                if !(0x2800..=0x283f).contains(&code) {
                    return Err(format!(
                        "Expected six-dot Unicode Braille at line {}, cell {}",
                        line + 1,
                        column + 1
                    ));
                }
                let first = margin + diameter * 0.5 + column as f64 * cell_pitch;
                for bit in 0..6 {
                    if (code - 0x2800) & (1 << bit) != 0 {
                        dots.push([
                            first + (bit / 3) as f64 * pitch,
                            top - (bit % 3) as f64 * pitch,
                        ]);
                    }
                }
            }
        }
        if dots.is_empty() {
            return Err("Authored Braille plate must contain a raised dot".into());
        }
        let width = 2.0 * margin + diameter + (cols - 1) as f64 * cell_pitch + pitch;
        let radius = ((diameter * 0.5).powi(2) + height * height) / (2.0 * height);
        Ok(Self {
            diameter,
            height,
            pitch,
            cell_pitch,
            line_pitch,
            thickness,
            margin,
            width,
            depth,
            radius,
            dots,
            braille: braille.join("\n"),
        })
    }

    fn signed_distance(&self, [x, y, z]: [f64; 3], candidates: &[[f64; 2]]) -> f64 {
        let px = x.clamp(0.0, self.width);
        let py = y.clamp(0.0, self.depth);
        let pz = z.clamp(-self.thickness, 0.0);
        let dx = (x - px).powi(2);
        let dy = (y - py).powi(2);
        let dz = (z - pz).powi(2);
        let horizontal_edge = x.abs().min((x - self.width).abs());
        let vertical_edge = y.abs().min((y - self.depth).abs());
        let mut distance_sq = (dx + dy + (z + self.thickness).powi(2))
            .min(horizontal_edge.powi(2) + dy + dz)
            .min(vertical_edge.powi(2) + dx + dz);
        let mut top_xy_sq = dx + dy;
        let mut inside = x == px && y == py && z == pz;
        let rim = self.diameter * 0.5;
        let sphere_z = self.height - self.radius;
        for &[cx, cy] in candidates {
            let radial = (x - cx).hypot(y - cy);
            let sphere_length = radial.hypot(z - sphere_z);
            let cap_distance_sq = if sphere_length > 0.0
                && sphere_z + self.radius * (z - sphere_z) / sphere_length >= 0.0
            {
                (sphere_length - self.radius).powi(2)
            } else {
                (radial - rim).powi(2) + z * z
            };
            distance_sq = distance_sq.min(cap_distance_sq);
            let projected_radius = (px - cx).hypot(py - cy);
            if projected_radius < rim {
                top_xy_sq = (rim - projected_radius).powi(2);
            }
            inside |= z >= 0.0 && sphere_length <= self.radius;
        }
        let distance = distance_sq.min(top_xy_sq + z * z).sqrt();
        if inside {
            -distance
        } else {
            distance
        }
    }
}

impl Sampling {
    fn from_args(args: &Args, plate: &Plate) -> Result<Self, String> {
        let xy = (plate.diameter * 0.25).min(0.30);
        let vertical = (plate.height / 3.5).min(plate.thickness / 3.0).min(0.14);
        let step = args
            .get("voxel_spacing_mm")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([xy, xy, vertical]);
        if step.iter().any(|value| !(0.02..=0.8).contains(value))
            || step[0].max(step[1]) > plate.diameter * 0.25 + 1e-12
            || step[2] > plate.height.min(plate.thickness) / 3.0 + 1e-12
        {
            return Err("Voxel spacing must represent each diameter by at least four XY samples and relief/thickness by at least three Z samples".into());
        }
        let band_mm = 3.0 * step.into_iter().fold(0.0, f64::max);
        let padding = band_mm + 0.37 * step[0].min(step[1]).min(step[2]);
        let spans = [plate.width, plate.depth, plate.thickness + plate.height];
        let samples = std::array::from_fn(|axis| {
            (((spans[axis] + 2.0 * padding) / step[axis]).ceil() as usize + 1).next_multiple_of(8)
        });
        if samples.iter().any(|&count| count > MAX_AXIS)
            || samples.into_iter().product::<usize>() > MAX_VOXELS
        {
            return Err("Tactile plate exceeds the 256-sample axis or voxel budget at the requested resolution".into());
        }
        let grid = sdf::SdfBrickGrid {
            origin_m: [-padding, -padding, -plate.thickness - padding]
                .map(|value| (value * 0.001) as f32),
            voxel_spacing_m: step.map(|value| (value * 0.001) as f32),
            base_bricks: samples.map(|count| (count / 8) as u32),
            encoding: sdf::SdfSampleEncoding::ScaledI16,
            meters_per_i16: (band_mm * 0.001 / f64::from(i16::MAX)) as f32,
        };
        Ok(Self {
            grid,
            samples,
            band_mm,
            origin_mm: grid.origin_m.map(|value| f64::from(value) * 1000.0),
            stride_mm: grid.voxel_spacing_m.map(|value| f64::from(value) * 1000.0),
        })
    }
}

type BakedSdf = (
    Vec<sdf::SdfBrickRecord>,
    Vec<sdf::OpenVdbActiveMask>,
    Vec<f64>,
);

fn bake(
    host: &mut AssetHost<'_>,
    plate: &Plate,
    sampling: &Sampling,
    wall: u64,
    void_role: u64,
    scratch: usize,
) -> Result<BakedSdf, String> {
    let [nx, ny, nz] = sampling.samples;
    let [bx, by, bz] = sampling.grid.base_bricks;
    let total = nx * ny * nz;
    let mut distances = vec![0.0; total];
    let mut records = Vec::with_capacity(total / sdf::SDFB_SAMPLES);
    let mut masks = Vec::with_capacity(records.capacity());
    let reach = plate.diameter * 0.5 + sampling.band_mm;
    let quant_m = f64::from(sampling.grid.meters_per_i16);
    for x_brick in 0..bx {
        for y_brick in 0..by {
            let [ox, oy, _] = sampling.origin_mm;
            let [sx, sy, _] = sampling.stride_mm;
            let x0 = ox + f64::from(x_brick) * 8.0 * sx;
            let y0 = oy + f64::from(y_brick) * 8.0 * sy;
            let candidates = plate
                .dots
                .iter()
                .copied()
                .filter(|[x, y]| {
                    (x0 - reach..=x0 + 7.0 * sx + reach).contains(x)
                        && (y0 - reach..=y0 + 7.0 * sy + reach).contains(y)
                })
                .collect::<Vec<_>>();
            host.charge(
                plate.dots.len() + candidates.len() * bz as usize * sdf::SDFB_SAMPLES * 8,
                scratch,
            )?;
            for z_brick in 0..bz {
                let coordinates = [x_brick, y_brick, z_brick];
                let mut record = sdf::SdfBrickRecord {
                    key: sdf::SdfBrickKey {
                        refinement_level: 0,
                        coordinate: coordinates,
                    },
                    wall_semantic_id: wall,
                    lumen_semantic_id: void_role,
                    samples: [0; sdf::SDFB_SAMPLES],
                    occupancy: [0; sdf::SDFB_OCCUPANCY_BYTES],
                };
                let mut mask = [0; sdf::SDFB_OPENVDB_ACTIVE_MASK_BYTES];
                for sample in 0..sdf::SDFB_SAMPLES {
                    let local = [sample % 8, (sample / 8) % 8, sample / 64];
                    let index = std::array::from_fn::<_, 3, _>(|axis| {
                        coordinates[axis] as usize * 8 + local[axis]
                    });
                    let point = std::array::from_fn(|axis| {
                        sampling.origin_mm[axis] + index[axis] as f64 * sampling.stride_mm[axis]
                    });
                    let phi = plate.signed_distance(point, &candidates);
                    let solid = phi < 0.0;
                    let bits = (phi * 0.001 / quant_m).round().clamp(-32767.0, 32767.0) as i16;
                    record.samples[sample] = bits as u16;
                    if solid {
                        record.occupancy[sample / 8] |= 1 << (sample % 8);
                    }
                    if phi.abs() <= sampling.band_mm {
                        let bit = local[2] + 8 * local[1] + 64 * local[0];
                        mask[bit / 8] |= 1 << (bit % 8);
                    }
                    let native = (index[2] * ny + index[1]) * nx + index[0];
                    distances[native] = if bits == 0 {
                        quant_m * if solid { -0.25 } else { 0.25 }
                    } else {
                        f64::from(bits) * quant_m
                    };
                }
                records.push(record);
                masks.push(mask);
            }
        }
    }
    Ok((records, masks, distances))
}

fn extract_plate_mesh(sampling: &Sampling, distances: &[f64]) -> Result<Mesh, String> {
    let [ox, oy, oz] = sampling.grid.origin_m.map(f64::from);
    let [sx, sy, sz] = sampling.grid.voxel_spacing_m.map(f64::from);
    let mut vertices = vec![Point3::default(); MAX_VERTICES];
    let mut triangles = vec![[0; 3]; MAX_TRIANGLES];
    let mut edges = vec![[0; 2]; MAX_VERTICES];
    let mut slots = vec![TetraEdgeSlot::default(); EDGE_SLOTS];
    let (vertex_count, triangle_count) = indexed_tetra_surface_capped_into(
        distances,
        sampling.samples,
        Point3::new(ox, oy, oz),
        Point3::new(sx, sy, sz),
        0.0,
        MAX_VOXELS,
        32_000_000,
        0.0,
        &mut vertices,
        &mut triangles,
        &mut edges,
        &mut slots,
    )
    .map_err(|error| format!("Tactile plate indexed surface: {error:?}"))?;
    if triangle_count == 0 {
        return Err("Tactile plate resolution produced no enclosed surface".into());
    }
    let positions = vertices[..vertex_count]
        .iter()
        .map(|vertex| [vertex.x as f32, vertex.y as f32, vertex.z as f32])
        .collect::<Vec<_>>();
    triangles.truncate(triangle_count);
    let (min, max) = positions.iter().fold(
        ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]),
        |(min, max), p| {
            (
                std::array::from_fn(|axis| min[axis].min(p[axis])),
                std::array::from_fn(|axis| max[axis].max(p[axis])),
            )
        },
    );
    Ok(Mesh {
        positions,
        triangles,
        min,
        max,
    })
}

fn build_labels(
    entity: &str,
    label: &str,
    language: Option<&str>,
    system: Option<&str>,
    transcript: &str,
    void_role: &str,
    braille: &str,
) -> Result<Vec<u8>, String> {
    let mut terms = BTreeMap::new();
    let mut add = |iri: &str, label: &str, lang: Option<&str>| {
        let term_hash = q_hash(iri);
        let entry = ten::MultilingualLexicalTerm {
            term_hash,
            iri_utf8: iri.into(),
            lang_bcp47: lang.map(str::to_owned),
            label_utf8: label.into(),
        };
        if term_hash == 0 || terms.insert(term_hash, entry).is_some() {
            return Err("Tactile plate semantic terms require distinct nonzero identities");
        }
        Ok(())
    };
    for iri in [
        GRAPH,
        KIND,
        TYPE,
        TRANSCRIPTION,
        TRANSCRIPTION_SYSTEM,
        MATERIAL_IRI,
    ] {
        add(iri, "", None)?;
    }
    add(entity, label, language)?;
    add(transcript, braille, None)?;
    add(void_role, "Unused cavity classification role", None)?;
    let graph = q_hash(GRAPH);
    let triple = |s: &str, p: &str, o: &str| {
        let [s, p, o] = [s, p, o].map(q_hash);
        [
            s,
            p,
            o,
            graph,
            0,
            NQuin::calculate_parity(s, p, o, graph, 0),
        ]
    };
    let mut nquins = vec![
        triple(entity, TYPE, KIND),
        triple(entity, TRANSCRIPTION, transcript),
    ];
    if let Some(system) = system {
        add(system, "", None)?;
        nquins.push(triple(transcript, TRANSCRIPTION_SYSTEM, system));
    }
    let payload = ten::SymbolicRdfCborLdPayload {
        context_iri: GRAPH.into(),
        lexicon: terms.into_values().collect(),
        nquins,
        ..Default::default()
    };
    ten::encode_symbolic_rdf_cbor_ld(&payload).map_err(|error| error.to_string())
}

fn attach_labels(bytes: &[u8], labels: &[u8]) -> Result<Vec<u8>, String> {
    let header = ten::Container10dHeader::parse(bytes).map_err(|error| error.to_string())?;
    let mut scratch = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut decoded = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let view = ten::TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut decoded)
        .map_err(|error| error.to_string())?;
    let identity = view.identity().ok_or("Tactile plate is missing MID2")?;
    let mut model = SemanticModelBuilder::new(42 << 20);
    for entry in view.payloads_of_kind(ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD) {
        model = model
            .merge_cbld(entry.map_err(|error| error.to_string())?.bytes)
            .map_err(|error| error.to_string())?;
    }
    model
        .merge_cbld(labels)
        .map_err(|error| error.to_string())?;
    let prior = view
        .section_payload(ten::SectionType::FieldSidecar)
        .unwrap_or_default();
    let mut sidecar = prior.to_vec();
    sidecar.extend_from_slice(labels);
    let mut fields = view.fields().to_vec();
    let field_id = fields
        .last()
        .map_or(0, |field| field.field_id)
        .checked_add(1)
        .ok_or("Tactile plate exhausted MID2 field IDs")?;
    fields.push(ten::TypedFieldRecord {
        field_id,
        kind: ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD,
        payload_section_id: ten::SectionType::FieldSidecar as u32,
        payload_offset: prior.len() as u64,
        payload_length: labels.len() as u64,
        ..Default::default()
    });
    let mut mid2 = vec![
        0;
        ten::manifold_identity_v2_encoded_len(fields.len())
            .map_err(|error| format!("{error:?}"))?
    ];
    ten::encode_manifold_identity_v2(identity, &fields, &mut mid2)
        .map_err(|error| format!("{error:?}"))?;
    let mut sections = Vec::new();
    for descriptor in ten::parse_section_table(bytes, &header).map_err(|error| error.to_string())? {
        let section_type = descriptor
            .typ()
            .ok_or("Tactile plate has an unknown section")?;
        let alignment_tier = descriptor
            .tier()
            .ok_or("Tactile plate has an invalid alignment")?;
        let (stride, element_count, payload) = match section_type {
            ten::SectionType::FieldSidecar => (0, 0, sidecar.as_slice()),
            ten::SectionType::ManifoldIdentityV2 => (0, 0, mid2.as_slice()),
            kind => (
                descriptor.stride,
                descriptor.element_count,
                view.section_payload(kind)
                    .ok_or("Tactile plate section payload is absent")?,
            ),
        };
        sections.push(ten::SectionInput {
            section_type,
            alignment_tier,
            stride,
            element_count,
            payload,
        });
    }
    let needed = match ten::encode_container(&header, &sections, &mut []) {
        Err(ten::SectionTableError::OutputBufferTooSmall { needed, .. })
            if needed <= MAX_10D_BYTES =>
        {
            needed
        }
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Err("Tactile plate container did not report a size".into()),
    };
    let mut result = vec![0; needed];
    let written = ten::encode_container(&header, &sections, &mut result)
        .map_err(|error| error.to_string())?;
    result.truncate(written);
    ten::seal_whole_file_crc32c(&mut result);
    Ok(result)
}

fn language_tag_syntax(tag: &str) -> bool {
    let mut parts = tag.split('-');
    tag.len() <= 64
        && parts.next().is_some_and(|first| {
            (1..=8).contains(&first.len()) && first.bytes().all(|byte| byte.is_ascii_alphabetic())
        })
        && parts.all(|part| {
            (1..=8).contains(&part.len()) && part.bytes().all(|byte| byte.is_ascii_alphanumeric())
        })
}

fn number(args: &Args, key: &str, default: Option<f64>, lo: f64, hi: f64) -> Result<f64, String> {
    let value = match args.get(key) {
        Some(value) => num(value)?,
        None => default.ok_or_else(|| format!("Missing {key}"))?,
    };
    (lo..=hi)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} requires a value in {lo}..={hi}"))
}
