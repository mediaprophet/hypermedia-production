//! Public creation-anchor marking for one fixed synthetic SDFB sphere.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use provenance_stego_physical::{decode_anchor, encode_anchor, CreationTreeAnchor, CODEWORD_BYTES};
use qualia_core_db::bundle::semantic_model::SemanticModelBuilder;
use qualia_core_db::container_10d::sdf_brick_section::{
    encode_sdfb_into, read_container_sdfb, sdfb_encoded_len, seal_10d_with_sdfb, SdfBrickGrid,
    SdfBrickKey, SdfBrickRecord, SdfSampleEncoding, SDFB_OCCUPANCY_BYTES, SDFB_SAMPLES,
};
use qualia_core_db::q_hash;
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d, upgrade_legacy_10d_to_holographic_v2,
};
use qualia_core_db::specialized_libs::computational_geometry::authoring::uv_sphere;
use sha2::{Digest, Sha256};

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::{fields, record, required, text, Args, AssetHost, Value};

const REGISTERED_PROFILE: &str = "public_synthetic_sphere_sdfb_rs_v1";
const ROOT: &str = "urn:qualia:synthetic-sphere:public-attribution-fixture:v1";
const MARKED_ROOT: &str = "urn:qualia:synthetic-sphere:public-attribution:rs64-32:v1:";
const WALL: &str = "urn:qualia:synthetic-sphere:surface:v1";
const LUMEN: &str = "urn:qualia:synthetic-sphere:interior:v1";
const ATTR: &str = "https://qualiadb.org/ontology/provenance#";
const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const BRICKS: usize = 4;
const SIDE: usize = BRICKS * 8;
const SPACING_M: f32 = 1.0 / 1024.0;
const QUANTUM_M: f32 = 1.0 / 1_048_576.0;
const RADIUS_M: f32 = 12.0 * SPACING_M;
const ORIGIN_M: f32 = -15.5 * SPACING_M;
const SLOT_COUNT: usize = CODEWORD_BYTES * 8;
const CARRIER_S1: u8 = 1;
const ASSET_LIMIT: usize = 256 * 1024;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Sample {
    brick: usize,
    within: usize,
}

struct Fixture {
    grid: SdfBrickGrid,
    records: Vec<SdfBrickRecord>,
    slots: Vec<Sample>,
}

struct Recovery {
    anchor: CreationTreeAnchor,
    changed_samples: usize,
    erased_symbols: usize,
    zero_surface_cells: usize,
}

impl AssetHost<'_> {
    pub(crate) fn embed_digital_mesh_stego(&mut self, args: &Args) -> Result<Value, String> {
        let mode = text(required(args, "mode")?)?;
        match mode {
            "prepare_public_sdfb_fixture" => fields(args, &["mode"] )?,
            "embed_public_sdfb_anchor" => fields(args, &[
                "mode", "asset", "creation_graph", "anchor", "trusted_anchor_sha256",
            ])?,
            "verify_public_sdfb_anchor" => fields(args, &[
                "mode", "asset", "creation_graph", "anchor", "trusted_anchor_sha256", "watermarked_asset",
            ])?,
            _ => return Err("Select prepare_public_sdfb_fixture, embed_public_sdfb_anchor or verify_public_sdfb_anchor".into()),
        }
        self.charge(12_000_000, 4 * ASSET_LIMIT)?;
        let fixture = Fixture::new()?;
        let limit = self.limits.resource_bytes.min(ASSET_LIMIT);
        let (baseline, baseline_field) = fixture.seal(&fixture.records, ROOT, limit)?;
        if mode == "prepare_public_sdfb_fixture" {
            let mut creation_graph =
                ReceiptGraph::new("urn:qualia:synthetic-sphere:creation-graph:v1");
            creation_graph.fact(
                ROOT,
                RDF_TYPE,
                &format!("{ATTR}PublicSyntheticAttributionFixture"),
            )?;
            creation_graph.fact(ROOT, &format!("{ATTR}carrierProfile"), &profile_iri())?;
            let graph = creation_graph.seal(self.limits.resource_bytes)?;
            let asset = self.admit_encoded("10d", baseline)?;
            let field = self.admit_encoded("sdfb", baseline_field)?;
            let creation_graph = self.admit_encoded("q42", graph)?;
            return Ok(Value::Record(Args::from([
                ("asset".into(), asset),
                ("field".into(), field),
                ("creation_graph".into(), creation_graph),
                ("root_iri".into(), Value::String(ROOT.into())),
            ])));
        }
        let source = required(args, "asset")?;
        let original = encoded_resource(self, source)?;
        if original.kind != "10d" || original.bytes != baseline {
            return Err(
                "SDFB attribution accepts the exact generated public synthetic sphere fixture"
                    .into(),
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
        let mut marker_digest = [0; 16];
        marker_digest.copy_from_slice(&pin[..16]);
        let anchor = CreationTreeAnchor {
            carrier: CARRIER_S1,
            digest_128: marker_digest,
            bom_slot: 0,
            governance_flags: 0,
        };
        let marker_hex = hex(&marker_digest);
        let marked_root = format!("{MARKED_ROOT}{marker_hex}");
        let (candidate, encoded_field) = if mode == "embed_public_sdfb_anchor" {
            let packet =
                encode_anchor(anchor).map_err(|error| format!("Native RS encode: {error:?}"))?;
            let marked = fixture.mark(&packet);
            let (asset, field) = fixture.seal(&marked, &marked_root, limit)?;
            (asset, Some(field))
        } else {
            let handle = required(args, "watermarked_asset")?;
            let resource = encoded_resource(self, handle)?;
            if resource.kind != "10d" || resource.bytes.len() > limit {
                return Err(
                    "Public attribution recovery requires a bounded sealed synthetic SDFB asset"
                        .into(),
                );
            }
            (resource.bytes.clone(), None)
        };
        let recovery = fixture.recover(&candidate, &marked_root, limit)?;
        if recovery.anchor != anchor {
            return Err(
                "The recovered S1 anchor differs from the verified public detached anchor".into(),
            );
        }
        let receipt = attribution_receipt(
            &baseline,
            &candidate,
            &pin,
            &marker_hex,
            &marked_root,
            self.limits.resource_bytes,
        )?;
        let mut output = Args::from([
            ("verified".into(), Value::Bool(true)),
            ("receipt".into(), self.admit_encoded("q42", receipt)?),
            ("lineage_receipt".into(), lineage_receipt),
            ("marker_digest_128".into(), Value::String(marker_hex)),
            ("carrier".into(), Value::String(REGISTERED_PROFILE.into())),
            (
                "changed_samples".into(),
                Value::U64(recovery.changed_samples as u64),
            ),
            (
                "erased_symbols".into(),
                Value::U64(recovery.erased_symbols as u64),
            ),
            (
                "verified_zero_surface_cells".into(),
                Value::U64(recovery.zero_surface_cells as u64),
            ),
            ("max_delta_m".into(), Value::F64(f64::from(QUANTUM_M))),
        ]);
        if let Some(field) = encoded_field {
            output.insert("asset".into(), self.admit_encoded("10d", candidate)?);
            output.insert("field".into(), self.admit_encoded("sdfb", field)?);
        }
        Ok(Value::Record(output))
    }
}

impl Fixture {
    fn new() -> Result<Self, String> {
        let grid = SdfBrickGrid {
            origin_m: [ORIGIN_M; 3],
            voxel_spacing_m: [SPACING_M; 3],
            base_bricks: [BRICKS as u32; 3],
            encoding: SdfSampleEncoding::ScaledI16,
            meters_per_i16: QUANTUM_M,
        };
        let mut records = Vec::with_capacity(BRICKS.pow(3));
        let mut eligible = Vec::new();
        for x in 0..BRICKS {
            for y in 0..BRICKS {
                for z in 0..BRICKS {
                    let mut record = SdfBrickRecord {
                        key: SdfBrickKey {
                            refinement_level: 0,
                            coordinate: [x as u32, y as u32, z as u32],
                        },
                        wall_semantic_id: q_hash(WALL),
                        lumen_semantic_id: q_hash(LUMEN),
                        samples: [0; SDFB_SAMPLES],
                        occupancy: [0; SDFB_OCCUPANCY_BYTES],
                    };
                    for within in 0..SDFB_SAMPLES {
                        let coordinates = [
                            8 * x + within % 8,
                            8 * y + within / 8 % 8,
                            8 * z + within / 64,
                        ];
                        let sample = baseline_sample(coordinates);
                        record.samples[within] = sample as u16;
                        if sample < 0 {
                            record.occupancy[within / 8] |= 1 << (within % 8);
                        }
                        if exterior_sample_is_clear(coordinates, sample) {
                            eligible.push(Sample {
                                brick: records.len(),
                                within,
                            });
                        }
                    }
                    records.push(record);
                }
            }
        }
        if eligible.len() < SLOT_COUNT {
            return Err(
                "Synthetic fixture does not contain enough exterior attribution samples".into(),
            );
        }
        let slots = (0..SLOT_COUNT)
            .map(|i| eligible[i * eligible.len() / SLOT_COUNT])
            .collect();
        Ok(Self {
            grid,
            records,
            slots,
        })
    }

    fn mark(&self, packet: &[u8; CODEWORD_BYTES]) -> Vec<SdfBrickRecord> {
        let mut modified = self.records.clone();
        for (offset, &position) in self.slots.iter().enumerate() {
            let slot = &mut modified[position.brick].samples[position.within];
            let delta = if packet[offset / 8] & (1 << (offset % 8)) != 0 {
                1
            } else {
                -1
            };
            *slot = (*slot as i16 + delta) as u16;
        }
        modified
    }

    fn seal(
        &self,
        records: &[SdfBrickRecord],
        root: &str,
        limit: usize,
    ) -> Result<(Vec<u8>, Vec<u8>), String> {
        let bytes = sdfb_encoded_len(records.len()).map_err(|error| error.to_string())?;
        if bytes > limit {
            return Err("Synthetic SDFB fixture exceeds the retained resource budget".into());
        }
        let mut field = vec![0; bytes];
        encode_sdfb_into(self.grid, records, &mut field).map_err(|error| error.to_string())?;
        let sphere =
            uv_sphere(RADIUS_M, 12, 24).map_err(|error| format!("Synthetic sphere: {error:?}"))?;
        let mesh = compile_mesh_to_10d(&sphere)
            .map_err(|error| format!("Synthetic sphere .10d: {error:?}"))?;
        let base = upgrade_legacy_10d_to_holographic_v2(
            &mesh,
            root,
            1,
            [0.0, 3.0, 2.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.5],
        )
        .map_err(|error| format!("Synthetic sphere HOLO: {error:?}"))?;
        let asset = seal_10d_with_sdfb(&base, &field, WALL, LUMEN)?;
        if asset.len() > limit {
            return Err("Synthetic sphere exceeds the retained .10d resource budget".into());
        }
        let (_, meta) = SemanticModelBuilder::new(limit)
            .merge_10d(&asset)
            .map_err(|error| error.to_string())?;
        if !meta
            .is_some_and(|identity| identity.root_entity_iri == root && identity.stratum_mask != 0)
        {
            return Err("Synthetic sphere lost its addressed HOLO/MSTR root".into());
        }
        Ok((asset, field))
    }

    fn recover(&self, asset: &[u8], marked_root: &str, limit: usize) -> Result<Recovery, String> {
        let view = read_container_sdfb(asset)?
            .ok_or("Synthetic asset has no locally addressed SDFB field")?;
        if view.grid() != self.grid
            || view.has_openvdb_active_masks()
            || view.bricks().len() != self.records.len()
        {
            return Err("Recovery requires the native fixed-grid SDFB synthetic fixture".into());
        }
        let mut marked = self.records.clone();
        let mut packet = [Some(0); CODEWORD_BYTES];
        let mut carrier = 0;
        let mut changed_samples = 0;
        for (brick, (baseline, output)) in self.records.iter().zip(&mut marked).enumerate() {
            let input = view
                .brick(baseline.key)
                .ok_or("Synthetic attribution lost a fixture brick")?;
            if input.wall_semantic_id() != baseline.wall_semantic_id
                || input.lumen_semantic_id() != baseline.lumen_semantic_id
            {
                return Err("Synthetic attribution changed a protected semantic binding".into());
            }
            for within in 0..SDFB_SAMPLES {
                let occupied = baseline.occupancy[within / 8] & (1 << (within % 8)) != 0;
                if input.occupied(within) != Some(occupied) {
                    return Err("Synthetic attribution changed a protected occupancy bit".into());
                }
                let sample = input
                    .distance_m(within)
                    .ok_or("Synthetic attribution is missing a distance sample")?;
                let code = (sample / QUANTUM_M) as i16;
                if f32::from(code) * QUANTUM_M != sample {
                    return Err(
                        "Synthetic attribution requires the canonical signed-distance quantizer"
                            .into(),
                    );
                }
                let reference = baseline.samples[within] as i16;
                let delta = i32::from(code) - i32::from(reference);
                if self.slots.get(carrier) != Some(&Sample { brick, within }) {
                    if delta != 0 {
                        return Err("Synthetic attribution changed a protected SDF sample".into());
                    }
                    continue;
                }
                let symbol = &mut packet[carrier / 8];
                match delta {
                    1 => { if let Some(value) = symbol { *value |= 1 << (carrier % 8); } }
                    -1 => {},
                    0 => *symbol = None,
                    _ => return Err("Synthetic attribution exceeds the declared one-quantum sample displacement".into()),
                }
                if delta != 0 {
                    changed_samples += 1;
                }
                output.samples[within] = code as u16;
                carrier += 1;
            }
        }
        if carrier != SLOT_COUNT {
            return Err(
                "Synthetic attribution did not traverse its declared carrier layout".into(),
            );
        }
        let zero_surface_cells = verify_zero_surface_cells(&self.records, &marked)?;
        let (canonical, _) = self.seal(&marked, marked_root, limit)?;
        if canonical != asset {
            return Err(
                "Synthetic attribution changed protected container sections or public disclosure"
                    .into(),
            );
        }
        let anchor =
            decode_anchor(&packet).map_err(|error| format!("Native RS recovery: {error:?}"))?;
        Ok(Recovery {
            anchor,
            changed_samples,
            erased_symbols: packet.iter().filter(|symbol| symbol.is_none()).count(),
            zero_surface_cells,
        })
    }
}

fn verify_zero_surface_cells(
    pristine: &[SdfBrickRecord],
    marked: &[SdfBrickRecord],
) -> Result<usize, String> {
    let sample = |set: &[SdfBrickRecord], x: usize, y: usize, z: usize| {
        let brick = (x / 8 * BRICKS + y / 8) * BRICKS + z / 8;
        set[brick].samples[x % 8 + 8 * (y % 8 + 8 * (z % 8))] as i16
    };
    let mut verified = 0;
    for x in 0..SIDE - 1 {
        for y in 0..SIDE - 1 {
            for z in 0..SIDE - 1 {
                let baseline = std::array::from_fn::<_, 8, _>(|i| {
                    sample(pristine, x + (i & 1), y + ((i >> 1) & 1), z + (i >> 2))
                });
                let output = std::array::from_fn::<_, 8, _>(|i| {
                    sample(marked, x + (i & 1), y + ((i >> 1) & 1), z + (i >> 2))
                });
                let crosses_zero =
                    |cell: [i16; 8]| cell.iter().any(|&v| v <= 0) && cell.iter().any(|&v| v >= 0);
                if crosses_zero(baseline) || crosses_zero(output) {
                    if baseline != output {
                        return Err("Public attribution changed a zero-surface cell corner".into());
                    }
                    verified += 1;
                }
            }
        }
    }
    if verified == 0 {
        return Err("Synthetic attribution validation found no zero-surface cells".into());
    }
    Ok(verified)
}

fn baseline_sample(coordinates: [usize; 3]) -> i16 {
    let point = coordinates.map(|index| f64::from(ORIGIN_M) + index as f64 * f64::from(SPACING_M));
    let radial = point.iter().map(|x| x * x).sum::<f64>().sqrt();
    ((radial - f64::from(RADIUS_M)) / f64::from(QUANTUM_M)).round() as i16
}

fn exterior_sample_is_clear(p: [usize; 3], sample: i16) -> bool {
    let delta = f32::from(sample) * QUANTUM_M;
    if !(1.5 * SPACING_M..=3.0 * SPACING_M).contains(&delta)
        || p.iter().any(|&index| index == 0 || index + 1 == SIDE)
    {
        return false;
    }
    // Every incident trilinear cell stays strictly positive even if all of its
    // vertices that belong to the public marker move down by one quantum.
    for x in p[0] - 1..=p[0] + 1 {
        for y in p[1] - 1..=p[1] + 1 {
            for z in p[2] - 1..=p[2] + 1 {
                if baseline_sample([x, y, z]) <= 1 {
                    return false;
                }
            }
        }
    }
    true
}

fn attribution_receipt(
    pristine: &[u8],
    marked: &[u8],
    pin: &[u8; 32],
    digest: &str,
    marked_root: &str,
    limit: usize,
) -> Result<Vec<u8>, String> {
    let pristine_iri = format!("urn:sha256:{:x}", Sha256::digest(pristine));
    let marked_iri = format!("urn:sha256:{:x}", Sha256::digest(marked));
    let anchor_iri = format!("urn:sha256:{}", hex(pin));
    let digest_iri = format!("urn:qualia:disclosed-anchor-digest-128:{digest}");
    let mut graph = ReceiptGraph::new("urn:qualia:public-sdfb-attribution:receipt:v1");
    graph.fact(
        marked_root,
        RDF_TYPE,
        &format!("{ATTR}PublicCreatorAttribution"),
    )?;
    graph.fact(
        marked_root,
        &format!("{ATTR}carrierProfile"),
        &profile_iri(),
    )?;
    graph.fact(marked_root, &format!("{ATTR}assetDigest"), &marked_iri)?;
    graph.fact(
        marked_root,
        &format!("{ATTR}syntheticFixtureDigest"),
        &pristine_iri,
    )?;
    graph.fact(
        marked_root,
        &format!("{ATTR}publicDetachedAnchor"),
        &anchor_iri,
    )?;
    graph.fact(marked_root, &format!("{ATTR}markerDigest"), &digest_iri)?;
    graph.fact(
        marked_root,
        &format!("{ATTR}preservesProtectedData"),
        &format!("{ATTR}ByteExactZeroCellSamplesAndOccupancy"),
    )?;
    graph.seal(limit)
}

fn profile_iri() -> String {
    format!("urn:qualia:attribution-profile:{REGISTERED_PROFILE}")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
