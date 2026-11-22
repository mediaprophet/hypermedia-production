//! Detached creation-graph commitments and immutable lineage verification.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use qualia_core_db::bundle::semantic_model::resource_identity_meta;
use qualia_core_db::q42_volume::{
    Q42RangeQueryCursor, Q42RangeQueryPattern, Q42RangeQueryPlan, WasmSliceQ42Reader,
    MAX_COMPRESSED_SUPERBLOCK_SIZE, SUPERBLOCK_SIZE,
};
use qualia_core_db::{q_hash, NQuin};

use crate::{fields, required, text, Args, AssetHost, Value};

use super::{digest_iri, encode_hex, parse_hex, retained, review_fact, review_graph, sha256};

const MAX_GRAPH_BYTES: usize = 16 * 1024 * 1024;
const MAX_ANCHOR_BYTES: usize = 1024 * 1024;
const MAX_IRI_BYTES: usize = 4096;
const LEXICON_PAGE_BYTES: usize = 1024 * 1024;
const READER_SCRATCH: usize =
    SUPERBLOCK_SIZE + MAX_COMPRESSED_SUPERBLOCK_SIZE + LEXICON_PAGE_BYTES + MAX_IRI_BYTES;

impl AssetHost<'_> {
    pub(crate) fn encode_creation_tree_payload(&mut self, args: &Args) -> Result<Value, String> {
        if text(required(args, "mode")?)? != "seal_detached_creation_anchor" {
            return Err("Select seal_detached_creation_anchor".into());
        }
        fields(args, &["mode", "asset", "creation_graph", "root_iri"])?;
        let (asset, graph, root) = self.read_creation_binding(args)?;
        let anchor = creation_anchor(&asset, &graph, root, self.limits.resource_bytes)?;
        let commitment = sha256(&anchor);
        Ok(Value::Record(Args::from([
            ("anchor".into(), self.admit_encoded("q42", anchor)?),
            (
                "anchor_sha256".into(),
                Value::String(encode_hex(&commitment)),
            ),
        ])))
    }

    pub(crate) fn decode_stego_and_verify_lineage(&mut self, args: &Args) -> Result<Value, String> {
        if text(required(args, "mode")?)? != "verify_detached_creation_anchor" {
            return Err("Select verify_detached_creation_anchor".into());
        }
        fields(
            args,
            &[
                "mode",
                "asset",
                "creation_graph",
                "root_iri",
                "anchor",
                "trusted_anchor_sha256",
            ],
        )?;
        let anchor_input = required(args, "anchor")?;
        let pin = parse_hex::<32>(
            text(required(args, "trusted_anchor_sha256")?)?,
            "trusted_anchor_sha256",
        )?;
        let anchor_size = {
            let anchor = retained(self, anchor_input)?;
            if anchor.kind != "q42" || !(1..=MAX_ANCHOR_BYTES).contains(&anchor.bytes.len()) {
                return Err("Provide a bounded detached Q42 creation anchor".into());
            }
            anchor.bytes.len()
        };
        self.charge(anchor_size * 4, MAX_ANCHOR_BYTES)?;
        if sha256(&retained(self, anchor_input)?.bytes) != pin {
            return Err(
                "The detached creation anchor does not match its trusted SHA-256 pin".into(),
            );
        }
        let (asset, graph, root) = self.read_creation_binding(args)?;
        let expected = creation_anchor(&asset, &graph, root, self.limits.resource_bytes)?;
        if retained(self, anchor_input)?.bytes != expected {
            return Err("The pinned anchor does not bind these exact retained asset and creation graph bytes".into());
        }
        let mut receipt = review_graph("VerifiedDetachedCreationAnchor")?;
        review_fact(&mut receipt, "creationRoot", root)?;
        review_fact(&mut receipt, "retainedAssetDigest", &digest_iri(&asset))?;
        review_fact(&mut receipt, "creationGraphDigest", &digest_iri(&graph))?;
        review_fact(&mut receipt, "trustedAnchorDigest", &digest_iri(&pin))?;
        Ok(Value::Record(Args::from([
            ("verified".into(), Value::Bool(true)),
            ("receipt".into(), self.admit_review(receipt)?),
        ])))
    }

    fn read_creation_binding<'args>(
        &mut self,
        args: &'args Args,
    ) -> Result<([u8; 32], [u8; 32], &'args str), String> {
        let asset_input = required(args, "asset")?;
        let graph_input = required(args, "creation_graph")?;
        let root = text(required(args, "root_iri")?)?;
        if root.len() > MAX_IRI_BYTES || resource_identity_meta(root).is_err() {
            return Err("Creation root must be a bounded absolute RDF IRI".into());
        }
        let bytes = {
            let asset = retained(self, asset_input)?;
            let graph = retained(self, graph_input)?;
            if !matches!(asset.kind.as_str(), "hmc" | "10d" | "q42")
                || asset.bytes.is_empty()
                || graph.kind != "q42"
                || !(1..=MAX_GRAPH_BYTES).contains(&graph.bytes.len())
            {
                return Err(
                    "Provide a retained HMC, 10d, or Q42 asset and a creation graph up to 16 MiB"
                        .into(),
                );
            }
            asset.bytes.len().saturating_add(graph.bytes.len())
        };
        self.charge(
            bytes.saturating_mul(16).saturating_add(50_000),
            READER_SCRATCH + MAX_ANCHOR_BYTES,
        )?;
        let asset = &retained(self, asset_input)?.bytes;
        let graph = &retained(self, graph_input)?.bytes;
        check_graph_root(graph, root)?;
        Ok((sha256(asset), sha256(graph), root))
    }
}

fn creation_anchor(
    asset: &[u8; 32],
    graph: &[u8; 32],
    root: &str,
    budget: usize,
) -> Result<Vec<u8>, String> {
    let mut anchor = review_graph("DetachedCreationAnchorV1")?;
    review_fact(&mut anchor, "retainedAssetDigest", &digest_iri(asset))?;
    review_fact(&mut anchor, "creationGraphDigest", &digest_iri(graph))?;
    review_fact(&mut anchor, "creationRoot", root)?;
    anchor.seal(budget.min(MAX_ANCHOR_BYTES))
}

fn check_graph_root(graph: &[u8], root: &str) -> Result<(), String> {
    let reader = WasmSliceQ42Reader::open(graph).map_err(|_| "Invalid Q42 creation graph")?;
    if !reader.is_unified_volume() || reader.block_count() == 0 || reader.block_count() > 2_048 {
        return Err(
            "The creation graph must be a nonempty headered Q42 volume within its block budget"
                .into(),
        );
    }
    let subject = q_hash(root);
    let mut lexical = [0; MAX_IRI_BYTES];
    let length = reader
        .lookup_lexicon_hash_into(subject, &mut vec![0; LEXICON_PAGE_BYTES], &mut lexical)
        .map_err(|_| "Unable to resolve the creation root in Q42LEX")?
        .ok_or("The creation root is absent from Q42LEX")?;
    if lexical.get(..length) != Some(root.as_bytes()) {
        return Err("The creation root and its Q42LEX entry disagree".into());
    }
    let plan = Q42RangeQueryPlan::for_pattern(Q42RangeQueryPattern {
        subject: Some(subject),
        ..Default::default()
    });
    let mut rows = [NQuin::default(); 1];
    let page = reader
        .execute_query_page_into(
            plan,
            Q42RangeQueryCursor::default(),
            &mut vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE],
            &mut vec![0; SUPERBLOCK_SIZE],
            &mut rows,
        )
        .map_err(|_| "Unable to query the bounded creation graph")?;
    if page.returned != 1 || rows[0].subject != subject || !rows[0].verify_ecc_parity() {
        return Err("The creation root must be the subject of an intact graph fact".into());
    }
    Ok(())
}
