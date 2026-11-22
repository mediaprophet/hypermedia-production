//! WP-39 production descriptors.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{AgenticMorphismSpec, DEFAULT_ENVELOPE, DEFAULT_PARAMS};
use vibescript_abi::{DeterminismGuarantee, EffectSet, FuncDescriptor, PrimitiveKind, ReversibilityKind, StableFuncId};

pub(super) const START: usize = super::morphisms_32::START + super::morphisms_32::BATCH.len();

pub(super) const BATCH: &[AgenticMorphismSpec] = &[
    AgenticMorphismSpec {
        id: "qualia.compute.inspect_and_dequantize_p64",
        aliases: &[],
        category: "compute",
        morphism: "inspect_and_dequantize_p64",
        owner: "QualiaDB and Hypermedia Production",
        summary: "Inspect a digest-pinned native P64 tensor directory and export a bounded set of F32, F16 or ternary tensors as flattened F32 SafeTensors.",
        domain: "DigestPinnedP64AndTensorOrdinals",
        codomain: "TensorDirectoryQ42ReceiptAndOptionalSafeTensors",
        morphism_kind: "Functor",
        teleology: "Keep original tensor order and shape inspectable while preserving the admitted stored element order in the archive.",
        units_and_frames: "Native tensor element order, manifest dimensions, byte extents and F32 binary payload lanes.",
        arguments: "source, source_sha256, mode=inspect|dequantize, optional or required tensor_ordinals respectively.",
        composable_with: &[],
        human_preview_affordance: "Inspect the native tensor directory and source digest before reading an optional decoded archive.",
        abi_descriptor: morphism_func!(
            "inspect_and_dequantize_p64",
            "https://qualiadb.org/ext/qualia/compute#inspect_and_dequantize_p64",
            42, 340, "(DigestPinnedP64AndTensorOrdinals) -> TensorDirectoryQ42ReceiptAndOptionalSafeTensors", START + 0,
            DEFAULT_PARAMS, EffectSet::PURE_COMPUTE,
            DeterminismGuarantee::DeterministicTolerance, ReversibilityKind::PureNoState
        ),
    },
    AgenticMorphismSpec {
        id: "hypermedia.geodesy.export_cog_zarr_netcdf4",
        aliases: &[],
        category: "geodesy",
        morphism: "export_cog_zarr_netcdf4",
        owner: "Hypermedia Production",
        summary: "Export bounded scalar fields as an uncompressed or indexed-sharded Zarr v3 archive with a source-bound Q42 receipt.",
        domain: "AdmittedScalarFieldAndExplicitAxes",
        codomain: "ZarrV3ArrayArchiveAndQ42Receipt",
        morphism_kind: "Functor",
        teleology: "Retain axis order and declared units while packaging exact IEEE-754 array samples.",
        units_and_frames: "Declared axis units, slowest-to-fastest row-major axes, and little-endian F32 samples.",
        arguments: "format=zarr_v3, source, axes, dense_axis_order for multidimensional DenseF32, optional source_sha256, field_id, field_index, coordinate_reference_iri, chunk_size_bytes, chunk_shape and shard_shape.",
        composable_with: &[],
        human_preview_affordance: "Inspect archive keys, axis metadata and the digest-bound export receipt.",
        abi_descriptor: morphism_func!(
            "export_cog_zarr_netcdf4",
            "https://qualiadb.org/ext/hypermedia/geodesy#export_cog_zarr_netcdf4",
            10, 333, "(AdmittedScalarFieldAndExplicitAxes) -> ZarrV3ArrayArchiveAndQ42Receipt", START + 1,
            DEFAULT_PARAMS, EffectSet::PURE_COMPUTE,
            DeterminismGuarantee::DeterministicExact, ReversibilityKind::PureNoState
        ),
    },
];
