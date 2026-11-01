# raster-layer-mask-adapter

A cold, checked conversion seam between the existing QualiaDB `ImageLayer.mask: Option<Vec<u8>>` field and the staged `raster-selection-morphology` operations. It transforms an explicit U8 mask to normalized f32 coverage, applies Expand/Contract/Border, rounds back to U8, and returns replacement bytes only on success.

`None` is interpreted as full coverage, matching `image_composite.rs` (`mask == None` contributes coverage 1.0). This adapter creates no document, selection identity, persistence format, history, codec, UI, or host authority. The adopting `ImageLayer` owner assigns the returned bytes to its existing mask field after success. The caller remains responsible for its existing session/authority/history checks.

Admission is caller-selected and occurs before conversion: morphology dimensions/radius/pixel/work/scratch/payload preflight, exact source-mask length, adapter conversion+output bytes, and aggregate work/payload caps. Aggregate logical payload includes the morphology plan plus adapter-owned normalized input and U8 output (conservatively 5 bytes/pixel). Allocator metadata, fragmentation and total process memory are outside that logical bound.

The source morphology crate is linked as a dependency and retains its PhotoCraft MIT/Apache notices. This adapter copies no PhotoCraft implementation. The manifest expects the installed sibling `../raster-selection-morphology`. The separate `provenance/preflight-dimensions.patch` proposes the public dimension-only admission helper the adapter imports; the patch is pinned against the installed canonical morphology bytes and has not been applied.

## Source seam and limit

The current Core image owner has `ImageLayer.mask`, `set_rect_mask`, and `clear_mask`; the compositor consumes U8 mask bytes. `ImageDocument.selections` instead stores geometric `SelectionShape` values (rectangle, ellipse, lasso). This adapter operates only on a layer mask. It does not rasterize or modify those geometric selection records, and it adds no active-selection state.

Poet's current image invoke IDs and toolbox actions expose mask and selection names. The `poet_host::invoke::hypermedia::image_set_mask`, `image_clear_mask`, `image_composite`, and `image_add_selection` implementations currently return status records without resolving or mutating an `ImageDocument`. A real host caller therefore needs the existing image-resource/session authority seam before this adapter can become a user-facing Vibe operation.


## Round-three preflight patch

The staged adapter calls preflight_dimensions so it can refuse invalid dimensions and caller budgets before converting U8 data. The morphology API currently has only preflight, which requires an f32 mask. The exact proposed refactor/addition is in provenance/preflight-dimensions.patch, with byte-preserved before/after files and SHA-256 pins in source-pins.tsv. The patch base is the installed canonical morphology source hash recorded there. Root owns applying it during integration. No round-two source was changed for this proposal.

