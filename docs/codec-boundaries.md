# Existing encoding, differential updates and media execution

**2026-10-30 source trace.** Timothy identified differential/base-plus-delta
updates as relevant prior work, and confirmed that media-codec execution and
versioned containers remain important too. These are complementary responsibilities.
This document records reuse boundaries, not a new wire format or a passed codec gate.

| Responsibility | Located existing owner / work | Migration rule and evidence limit |
|---|---|---|
| Differential 3D prediction and progressive presentation | `qualia-sim/src/differential_codec.rs`: KinematicExtrapolator, DisocclusionForecaster, SaliencePerceptualGovernor and ProgressiveMonteCarloGovernor; exported by `qualia-sim/src/lib.rs` | Reuse the simulation owner when the selected consumer requires these operations. Source implements prediction/governor pieces; module naming is not proof of a complete asset delta bitstream or active render consumer. No execution of these kernels occurred in this migration round. |
| Lossless base/anchor and delta tile bytes | Core `inference/operator_package/lossless_tile.rs`: Raw, SharedRef, XorDelta, SplitFloat16; encode_tile_xor and decode_tile_into | Existing raw/shared/XOR delta functionality is present. Do not create another tile codec in terrain or image libraries. Asset adoption must qualify anchor identity, lengths/bounds, fallback, target digest and exact source/profile semantics. The existing module lives in operator packaging; general asset consumption is not established by finding it. |
| Base-plus-delta distribution | Core `inference/operator_package/delta_dist.rs`, EOS-081 in EXECUTABLE_OPERATOR_SWARM_PLAN_2026-09-19 | Scope is model/operator anchors, specialist deltas and KV replay/rollback. Reuse matching distribution semantics without treating a tensor/model policy as a universal HMC/image/terrain codec. Its digest-shaped u64 fields do not independently authenticate a package. Prior reported tests are historical evidence, not runs performed here. |
| Media codec selection and execution isolation | Poet `wasm/src/browser/media_codecs.rs`, Spec 17: pure Rust/WASM, hardware/neural, isolated sidecar tiers, per-domain descriptor catalogue and view | Preserve/reconcile the useful capability descriptions with existing native host/job/provider contracts. A descriptor, tier enum or DOM view does not prove an available native codec, sandbox or hardware route. No standalone application event loop is extracted with an algorithm. |
| Native resource wire formats | Core Q42 volume, M64 checked reader, container_10d typed sections and bundle/HMC owners; qualia-geospatial raster_m64 conversion | Preserve accepted versions, units/frames/nodata, source and unknown sections according to the actual operation. Typed media buffers and local brush journals do not become Q42/M64/10D/HMC merely by receiving a suffix. |
| Format evolution and cooking profiles | Ecopunk format-sdk-foundation and Biosphere Algorithms v2 reconciliation R-04/BR-01/BR-02; QualiaDB q42-format-internal-draft | Preserve current portable normative contracts. New alignment/quantisation/runtime layouts require explicit versioned profiles and compatibility evidence; do not silently rewrite older resources to an aspirational layout. |

Changes, bases and releases retain separate identifiers/revisions. A delta must
identify the exact required base and interpretation context. Applying it must
verify the reconstructed result and preserve the previous admitted resource on
failure; anchor availability, ordering, stale bases, unknown versions, corruption,
revocation and resource limits are explicit consumer cases. Avoid duplicate graph,
codec, journal, authorisation or package owners. An unrelated matching name or
digest does not establish authority or semantic equivalence.

The three new authoring libraries return typed authoring values or borrowed/
owned sample buffers. They define no new persisted format. Terrain tile patches
and timeline proposals are local editing results; the authorised host performs
revision checks, durable source/resource changes, provenance and existing codec
invocation. Source timecode and field/frame interpretation must survive that
adapter. Measured, inferred, authored and illustrative values remain distinct.

Scientific/master resources retain their explicit fidelity and uncertainty.
Target/device lossy profiles cannot silently lower the master or overwrite its
evidence. Capability availability and any native/provider execution must be
proved per selected codec and target. Reusing these structures is not a claim
that all codec layers or native consumers have already been integrated.
