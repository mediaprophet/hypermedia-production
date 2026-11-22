//! Explicit, machine-independent attribution profile for a fixed synthetic coupon.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::{fields, record, required, text, Args, AssetHost, Value};
use provenance_stego_physical::{decode_anchor, encode_anchor, CreationTreeAnchor, CODEWORD_BYTES};
use qualia_core_db::{
    bundle::semantic_model::SemanticModelBuilder,
    container_10d::VertexTetBinding,
    render::{
        assets::Mesh,
        compile_10d::{compile_mesh_to_10d, upgrade_legacy_10d_to_holographic_v2},
    },
    specialized_libs::computer_vision::spatial::{export_3mf::mesh_ir_to_3mf, geometry_ir::MeshIR},
};
use serde_json::{json, Value as Json};
use sha2::{Digest, Sha256};

const ROOT: &str = "urn:qualia:synthetic-coupon:public-print-attribution-fixture:v1";
const REGISTERED_PROFILE: &str = "public_3mf_directional_finish_rs_v1";
const RECEIPT_CONTEXT: &str = "urn:qualia:contexts:public-coupon-attribution-v1";
const PROVENANCE: &str = "https://qualiadb.org/ontology/provenance#";
const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const REST_BINDING_MODEL: &str = "synthetic_unloaded_tetrahedral_reference";
const STRESS_WITNESS: &str = "synthetic_declared_coupon_zones";
const EXCLUDED_VM_YIELD_PERMILLE: usize = 250;
const CARRIER_S2: u8 = 2;
const LAYERS: usize = 64;
const LAYER_HEIGHT_UM: usize = 200;
const DATUM_LAYERS: usize = 9; // Inclusive layer indices 0 through 8.
const SLOT_COUNT: usize = CODEWORD_BYTES * 8;
const BEZEL_FIRST_LAYER: usize = 32;
const EDGE_SEGMENTS: usize = 4;
const RING_VERTICES: usize = 4 * EDGE_SEGMENTS;
const WIDTH_UM: usize = 16_000;
const HEIGHT_UM: usize = LAYERS * LAYER_HEIGHT_UM;
const ASSET_LIMIT: usize = 512 * 1024;
const PROFILE_LIMIT: usize = 128 * 1024;
const MINUS: [i32; 2] = [0, -10];
const PLUS: [i32; 2] = [90, 10];
const ANGLE_TOLERANCE: f64 = 5.0;
const STEP_TOLERANCE: f64 = 2.0;

struct Coupon {
    print_archive: Vec<u8>,
    asset: Vec<u8>,
    layout: Json,
    protected_faces: usize,
}

struct Recovery {
    anchor: CreationTreeAnchor,
    corrected_symbols: usize,
    erased_symbols: usize,
}

impl AssetHost<'_> {
    pub(crate) fn embed_physical_print_stego(&mut self, args: &Args) -> Result<Value, String> {
        let mode = text(required(args, "mode")?)?;
        match mode {
            "prepare_public_print_fixture" => fields(args, &["mode"])?,
            "embed_public_print_anchor" => fields(
                args,
                &["mode", "asset", "creation_graph", "anchor", "trusted_anchor_sha256"],
            )?,
            "verify_public_print_anchor" => fields(
                args,
                &[
                    "mode", "asset", "creation_graph", "anchor", "trusted_anchor_sha256",
                    "print_archive", "finish_profile",
                ],
            )?,
            _ => return Err("Select prepare_public_print_fixture, embed_public_print_anchor or verify_public_print_anchor".into()),
        }
        self.charge(32_000_000, 6 * ASSET_LIMIT + 3 * PROFILE_LIMIT)?;
        let limit = self.limits.resource_bytes.min(ASSET_LIMIT);
        let coupon = Coupon::new(limit)?;
        let profile_iri = format!("urn:qualia:print-profile:{REGISTERED_PROFILE}");
        if mode == "prepare_public_print_fixture" {
            let mut creation = ReceiptGraph::new(RECEIPT_CONTEXT);
            creation.fact(
                ROOT,
                RDF_TYPE,
                &format!("{PROVENANCE}PublicSyntheticAttributionFixture"),
            )?;
            creation.fact(ROOT, &format!("{PROVENANCE}carrierProfile"), &profile_iri)?;
            creation.fact(
                ROOT,
                "http://www.w3.org/ns/prov#hadPrimarySource",
                &digest_iri(&coupon.print_archive),
            )?;
            let graph = creation.seal(limit)?;
            let nominal = coupon.profile(None, None)?;
            return Ok(Value::Record(Args::from([
                ("asset".into(), self.admit_encoded("10d", coupon.asset)?),
                (
                    "print_archive".into(),
                    self.admit_encoded("3mf", coupon.print_archive)?,
                ),
                (
                    "finish_profile".into(),
                    self.admit_encoded("json", nominal)?,
                ),
                ("creation_graph".into(), self.admit_encoded("q42", graph)?),
                ("root_iri".into(), Value::String(ROOT.into())),
            ])));
        }

        let source = required(args, "asset")?;
        let asset = encoded_resource(self, source)?;
        if asset.kind != "10d" || asset.bytes != coupon.asset {
            return Err(
                "Print attribution accepts the exact generated public synthetic coupon".into(),
            );
        }
        let detached = required(args, "anchor")?;
        let lineage = self.decode_stego_and_verify_lineage(&Args::from([
            (
                "mode".into(),
                Value::String("verify_detached_creation_anchor".into()),
            ),
            ("asset".into(), source.clone()),
            (
                "creation_graph".into(),
                required(args, "creation_graph")?.clone(),
            ),
            ("root_iri".into(), Value::String(ROOT.into())),
            ("anchor".into(), detached.clone()),
            (
                "trusted_anchor_sha256".into(),
                required(args, "trusted_anchor_sha256")?.clone(),
            ),
        ]))?;
        let lineage_receipt = required(record(&lineage)?, "receipt")?.clone();
        let pin: [u8; 32] = Sha256::digest(&encoded_resource(self, detached)?.bytes).into();
        let mut marker = [0; 16];
        marker.copy_from_slice(&pin[..16]);
        let expected = CreationTreeAnchor {
            carrier: CARRIER_S2,
            digest_128: marker,
            bom_slot: 0,
            governance_flags: 0,
        };
        let sidecar = if mode == "embed_public_print_anchor" {
            let packet = encode_anchor(expected)
                .map_err(|e| format!("Native print anchor encoding: {e:?}"))?;
            coupon.profile(Some(&pin), Some(&packet))?
        } else {
            let archive = encoded_resource(self, required(args, "print_archive")?)?;
            if archive.kind != "3mf" || archive.bytes != coupon.print_archive {
                return Err("Print verification requires the exact fixture archive, including every protected facet".into());
            }
            let profile = encoded_resource(self, required(args, "finish_profile")?)?;
            if profile.kind != "json" || profile.bytes.len() > PROFILE_LIMIT {
                return Err("Print verification requires a bounded JSON finish profile".into());
            }
            profile.bytes.clone()
        };
        let recovery = coupon.recover(&sidecar, &pin)?;
        if recovery.anchor != expected {
            return Err(
                "The recovered print-profile anchor differs from the verified detached anchor"
                    .into(),
            );
        }
        let marker_hex = hex(&marker);
        let receipt = coupon.receipt(&sidecar, &pin, &profile_iri, &recovery, limit)?;
        Ok(Value::Record(Args::from([
            ("verified".into(), Value::Bool(true)),
            (
                "print_archive".into(),
                self.admit_encoded("3mf", coupon.print_archive)?,
            ),
            (
                "finish_profile".into(),
                self.admit_encoded("json", sidecar)?,
            ),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
            ("lineage_receipt".into(), lineage_receipt),
            ("carrier".into(), Value::String(REGISTERED_PROFILE.into())),
            ("marker_digest_128".into(), Value::String(marker_hex)),
            (
                "corrected_symbols".into(),
                Value::U64(recovery.corrected_symbols as u64),
            ),
            (
                "erased_symbols".into(),
                Value::U64(recovery.erased_symbols as u64),
            ),
            ("protected_layers".into(), Value::U64(DATUM_LAYERS as u64)),
            (
                "protected_faces".into(),
                Value::U64(coupon.protected_faces as u64),
            ),
            ("max_protected_deviation_um".into(), Value::U64(0)),
        ])))
    }
}

impl Coupon {
    fn new(limit: usize) -> Result<Self, String> {
        let mesh = coupon_mesh();
        let mut protected = vec![false; mesh.triangles.len()];
        protected[..DATUM_LAYERS * RING_VERTICES * 2].fill(true);
        let first_cap = LAYERS * RING_VERTICES * 2;
        protected[first_cap..first_cap + RING_VERTICES].fill(true);
        let face_mask: Vec<_> = protected
            .iter()
            .enumerate()
            .filter_map(|(i, on)| on.then_some(i))
            .collect();
        let protected_faces = face_mask.len();
        // An authored witness exercises the threshold gate. These values label
        // fictional zones and do not report a stress computation or measurement.
        let stress_zones: [usize; LAYERS] =
            std::array::from_fn(|layer| if layer < DATUM_LAYERS { 300 } else { 0 });
        let mut slots = Vec::with_capacity(SLOT_COUNT);
        for index in 0..SLOT_COUNT {
            let layer = BEZEL_FIRST_LAYER + index / RING_VERTICES;
            let segment = index % RING_VERTICES;
            let first = (layer * RING_VERTICES + segment) * 2;
            if layer >= LAYERS || layer < DATUM_LAYERS || protected[first] || protected[first + 1] {
                return Err("Attribution cell intersects a protected coupon facet".into());
            }
            if stress_zones[layer] >= EXCLUDED_VM_YIELD_PERMILLE {
                return Err(
                    "Attribution cell intersects the synthetic excluded-stress zone".into(),
                );
            }
            check_unloaded_face(&mesh, first)?;
            check_unloaded_face(&mesh, first + 1)?;
            slots.push([layer, first, first + 1]);
        }

        let mut print_mesh = MeshIR::empty();
        print_mesh.positions = mesh
            .positions
            .iter()
            .map(|p| p.map(|v| v * 1000.0))
            .collect();
        print_mesh.indices = mesh.triangles.iter().flatten().copied().collect();
        print_mesh.recompute_bounds_and_hash();
        let mut print_archive = vec![0; limit];
        let size = mesh_ir_to_3mf(&print_mesh, &mut print_archive)
            .map_err(|e| format!("Public coupon native 3MF: {e:?}"))?;
        print_archive.truncate(size);

        let legacy =
            compile_mesh_to_10d(&mesh).map_err(|e| format!("Public coupon .10d: {e:?}"))?;
        let asset = upgrade_legacy_10d_to_holographic_v2(
            &legacy,
            ROOT,
            1,
            [0.0, 3.0, 2.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.5],
        )
        .map_err(|e| format!("Public coupon HOLO: {e:?}"))?;
        if asset.len() > limit {
            return Err("Public coupon exceeds the fixed resource budget".into());
        }
        let (_, identity) = SemanticModelBuilder::new(limit)
            .merge_10d(&asset)
            .map_err(|e| e.to_string())?;
        if !identity.is_some_and(|meta| meta.root_entity_iri == ROOT && meta.stratum_mask != 0) {
            return Err("Public coupon lost its addressed HOLO/MSTR identity".into());
        }
        let nominal_layers: Vec<_> = (0..LAYERS)
            .map(|i| {
                json!({
                    "layer": i, "nominal_height_um": LAYER_HEIGHT_UM,
                    "direction_deg": 0, "delta_um": 0, "protected": i < DATUM_LAYERS
                })
            })
            .collect();
        let layout = json!({
            "schema": REGISTERED_PROFILE,
            "disclosure": "Public creator attribution; offline finish-profile observations only",
            "fixture_root_iri": ROOT,
            "asset_10d_sha256": hex(&Sha256::digest(&asset)),
            "print_archive_3mf_sha256": hex(&Sha256::digest(&print_archive)),
            "nominal_layers": nominal_layers,
            "protected_face_mask": face_mask,
            "slot_layout": slots,
            "slot_columns": ["layer", "face_index_0", "face_index_1"],
            "observation_columns": ["direction_deg", "layer_step_um"],
            "calibration": {
                "reference_0": MINUS, "reference_1": PLUS,
                "angle_tolerance_deg": ANGLE_TOLERANCE, "step_tolerance_um": STEP_TOLERANCE
            },
            "stress_witness": {
                "domain": STRESS_WITNESS,
                "reference_vm_yield_permille_by_layer": stress_zones.as_slice(),
                "excluded_vm_yield_permille": EXCLUDED_VM_YIELD_PERMILLE,
            },
            "femc_rest_binding_model": REST_BINDING_MODEL,
        });
        Ok(Self {
            print_archive,
            asset,
            layout,
            protected_faces,
        })
    }

    fn profile(
        &self,
        pin: Option<&[u8; 32]>,
        packet: Option<&[u8; CODEWORD_BYTES]>,
    ) -> Result<Vec<u8>, String> {
        let marks: Vec<_> = match packet {
            Some(packet) => (0..SLOT_COUNT)
                .map(|i| {
                    if packet[i / 8] & (1 << (7 - i % 8)) == 0 {
                        MINUS
                    } else {
                        PLUS
                    }
                })
                .collect(),
            None => vec![[0, 0]; SLOT_COUNT],
        };
        let bytes = serde_json::to_vec(&json!({
            "layout": &self.layout,
            "trusted_anchor_sha256": pin.map(|value| hex(value)),
            "observations": marks,
        }))
        .map_err(|e| e.to_string())?;
        if bytes.len() > PROFILE_LIMIT {
            return Err("Generated finish profile exceeds the fixed byte budget".into());
        }
        Ok(bytes)
    }

    fn recover(&self, bytes: &[u8], pin: &[u8; 32]) -> Result<Recovery, String> {
        let profile: Json =
            serde_json::from_slice(bytes).map_err(|e| format!("Finish profile: {e}"))?;
        let object = profile
            .as_object()
            .ok_or("Finish profile must be a JSON object")?;
        if object.len() != 3 || object.get("layout") != Some(&self.layout) {
            return Err(
                "Finish profile modifies the trusted layout, datum settings or calibration".into(),
            );
        }
        let trusted = hex(pin);
        if object.get("trusted_anchor_sha256").and_then(Json::as_str) != Some(trusted.as_str()) {
            return Err("Finish profile is bound to a different detached anchor".into());
        }
        let observations = object
            .get("observations")
            .and_then(Json::as_array)
            .filter(|rows| rows.len() == SLOT_COUNT)
            .ok_or("Finish profile requires 512 ordered observations")?;
        let mut packet = [None; CODEWORD_BYTES];
        for (symbol, rows) in observations.chunks_exact(8).enumerate() {
            let mut measured = 0;
            let mut erased = false;
            for row in rows {
                measured <<= 1;
                match classify(row)? {
                    Some(bit) => measured |= bit,
                    None => erased = true,
                }
            }
            if !erased {
                packet[symbol] = Some(measured);
            }
        }
        let anchor = decode_anchor(&packet)
            .map_err(|e| format!("Calibrated finish-profile recovery: {e:?}"))?;
        let canonical =
            encode_anchor(anchor).map_err(|e| format!("Recovered print anchor framing: {e:?}"))?;
        let corrected_symbols = packet
            .iter()
            .zip(canonical)
            .filter(|(seen, expected)| seen.is_some_and(|observed| observed != *expected))
            .count();
        let erased_symbols = packet.iter().filter(|sample| sample.is_none()).count();
        if 2 * corrected_symbols + erased_symbols > 32 {
            return Err("Finish profile exceeds the RS(64,32) error and erasure budget".into());
        }
        Ok(Recovery {
            anchor,
            corrected_symbols,
            erased_symbols,
        })
    }

    fn receipt(
        &self,
        sidecar: &[u8],
        pin: &[u8; 32],
        profile_iri: &str,
        recovery: &Recovery,
        limit: usize,
    ) -> Result<Vec<u8>, String> {
        let profile = digest_iri(sidecar);
        let mut graph = ReceiptGraph::new(RECEIPT_CONTEXT);
        graph.fact(
            &profile,
            RDF_TYPE,
            &format!("{PROVENANCE}VerifiedPublicPrintAttribution"),
        )?;
        for (predicate, object) in [
            ("carrierProfile", profile_iri.to_owned()),
            ("fixture", ROOT.to_owned()),
            ("source10dDigest", digest_iri(&self.asset)),
            ("printArchiveDigest", digest_iri(&self.print_archive)),
            ("trustedAnchorDigest", format!("urn:sha256:{}", hex(pin))),
            (
                "markerDigest128",
                format!(
                    "urn:qualia:anchor-digest-128:{}",
                    hex(&recovery.anchor.digest_128)
                ),
            ),
            (
                "stressWitness",
                format!("urn:qualia:print-stress-domain:{STRESS_WITNESS}"),
            ),
            (
                "restBindingModel",
                format!("urn:qualia:print-rest-binding:{REST_BINDING_MODEL}"),
            ),
            ("protectedDeviation", "urn:qualia:micrometre:0".into()),
            (
                "protectedLayerCount",
                format!("urn:qualia:integer:{DATUM_LAYERS}"),
            ),
            (
                "protectedFaceCount",
                format!("urn:qualia:integer:{}", self.protected_faces),
            ),
            (
                "correctedSymbolCount",
                format!("urn:qualia:integer:{}", recovery.corrected_symbols),
            ),
            (
                "erasedSymbolCount",
                format!("urn:qualia:integer:{}", recovery.erased_symbols),
            ),
        ] {
            graph.fact(&profile, &format!("{PROVENANCE}{predicate}"), &object)?;
        }
        graph.seal(limit)
    }
}

fn classify(row: &Json) -> Result<Option<u8>, String> {
    if row.is_null() {
        return Ok(None);
    }
    let [direction, delta] = row
        .as_array()
        .map(Vec::as_slice)
        .ok_or("Expected a directional finish observation or null")?
    else {
        return Err("Directional finish observations require an angle and a step".into());
    };
    let (Some(direction), Some(delta)) = (direction.as_f64(), delta.as_f64()) else {
        return Err("Directional finish observations must contain numeric measurements".into());
    };
    Ok([MINUS, PLUS]
        .into_iter()
        .position(|reference| {
            (direction - f64::from(reference[0])).abs() <= ANGLE_TOLERANCE
                && (delta - f64::from(reference[1])).abs() <= STEP_TOLERANCE
        })
        .map(|index| index as u8))
}

fn coupon_mesh() -> Mesh {
    let metres = |um| (um as f64 * 1e-6) as f32;
    let mut positions = Vec::with_capacity((LAYERS + 1) * RING_VERTICES + 2);
    for layer in 0..=LAYERS {
        for i in 0..RING_VERTICES {
            let offset = i % EDGE_SEGMENTS * (WIDTH_UM / EDGE_SEGMENTS);
            let [x, y] = match i / EDGE_SEGMENTS {
                0 => [offset, 0],
                1 => [WIDTH_UM, offset],
                2 => [WIDTH_UM - offset, WIDTH_UM],
                _ => [0, WIDTH_UM - offset],
            };
            positions.push([metres(x), metres(y), metres(layer * LAYER_HEIGHT_UM)]);
        }
    }
    let bottom = positions.len() as u32;
    let top = bottom + 1;
    let center = metres(WIDTH_UM / 2);
    positions.extend([[center, center, 0.0], [center, center, metres(HEIGHT_UM)]]);

    let mut triangles = Vec::with_capacity((LAYERS + 1) * RING_VERTICES * 2);
    for layer in 0..LAYERS {
        for segment in 0..RING_VERTICES {
            let a = (layer * RING_VERTICES + segment) as u32;
            let b = (layer * RING_VERTICES + (segment + 1) % RING_VERTICES) as u32;
            let step = RING_VERTICES as u32;
            triangles.extend([[a, b, b + step], [a, b + step, a + step]]);
        }
    }
    let last = (LAYERS * RING_VERTICES) as u32;
    for segment in 0..RING_VERTICES as u32 {
        triangles.push([bottom, (segment + 1) % RING_VERTICES as u32, segment]);
    }
    for segment in 0..RING_VERTICES as u32 {
        triangles.push([
            top,
            last + segment,
            last + (segment + 1) % RING_VERTICES as u32,
        ]);
    }
    Mesh {
        positions,
        triangles,
        min: [0.0; 3],
        max: [metres(WIDTH_UM), metres(WIDTH_UM), metres(HEIGHT_UM)],
    }
}

fn check_unloaded_face(mesh: &Mesh, face: usize) -> Result<(), String> {
    let [a, b, c] = mesh.triangles[face].map(|index| mesh.positions[index as usize]);
    let interior = mesh.max.map(|axis| axis * 0.5);
    let difference = |p: [f32; 3], q: [f32; 3]| {
        [
            f64::from(p[0]) - f64::from(q[0]),
            f64::from(p[1]) - f64::from(q[1]),
            f64::from(p[2]) - f64::from(q[2]),
        ]
    };
    let [ux, uy, uz] = difference(b, a);
    let [vx, vy, vz] = difference(c, a);
    let [wx, wy, wz] = difference(a, interior);
    let normal = [uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx];
    let determinant = normal[0] * wx + normal[1] * wy + normal[2] * wz;
    if determinant <= 0.0 || !determinant.is_finite() {
        return Err("Print coupon has an invalid outward-facing tetrahedral correspondence".into());
    }
    let rest = [a, b, c, interior];
    for vertex in 0..3 {
        let mut weights = [0.0; 4];
        weights[vertex] = 1.0;
        let bound =
            VertexTetBinding::from_barycentric(face as u32, weights, normal.map(|v| v as f32));
        if bound.interpolate_deformed_vertex(rest).map(f32::to_bits)
            != rest[vertex].map(f32::to_bits)
        {
            return Err("Unloaded FEMC correspondence moves a coupon bezel vertex".into());
        }
    }
    Ok(())
}

fn digest_iri(bytes: &[u8]) -> String {
    format!("urn:sha256:{}", hex(&Sha256::digest(bytes)))
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(out, "{byte:02x}");
    }
    out
}
