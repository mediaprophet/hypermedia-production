# raster-selection-morphology

Pure transforms over caller-owned row-major `f32` coverage masks. The checked `try_expand`, `try_contract`, and `try_border` functions validate exact dimensions, non-empty shape, EDT representability (including the source EDT's fixed `width * 32` band allocation), finite `0..=1` coverage, finite non-negative radius, and caller-selected pixel, work, EDT-scratch, and total algorithm-payload limits before allocating algorithm buffers.

PhotoCraft's engine caller applies product-specific radius ranges, canvas-boundary padding policy, and a padded-pixel cap before invoking the checked operations. An adopting owner must choose its own boundary policy and compatible limits. The payload plan counts logical `Vec` element storage owned by the operation, including masks, EDT arrays, output, and EDT band scratch. It excludes caller input, allocator metadata/rounding/fragmentation, and Rayon runtime/pool storage.

Work units conservatively include validation and algorithm loops: 27 per pixel for expand, 29 for contract, and 52 for border. Each EDT is budgeted at 24 visits per pixel: seed `any`/`all` scans, column sweeps, output initialization, band scratch initialization/copy, row candidate filtering/insertion/removals, envelope advancement, and output writes, with headroom for loop-condition/index checks. These are admission units, not a wall-clock prediction. Scalar dimension/work/scratch/payload admission runs before the O(N) finite-coverage scan; that scan is included in each operation's bound.

Behavior retained from PhotoCraft:

- Pixels at coverage `>= 0.5` seed the Euclidean distance transform.
- Expansion preserves existing fractional coverage and adds radius-based coverage around seed pixel centers.
- Contraction is the source's inversion, expansion, inversion sequence.
- Border computes a soft band on both sides of the binary edge. It has a measured Photoshop 27.11 oracle in the source tests; it is not `expand - contract`.
- The EDT implementation and its complete source test module are byte-identical. Default builds traverse rows serially. Consumers may explicitly enable the `parallel` feature to use Rayon on native targets; `wasm32` remains serial.

The consumer example shows the checked typed API. The host remains responsible for retaining returned values in its authoritative editable image/selection state.

## Origin and licensing

Derived from PhotoCraft's `photocraft-algo` selection module, copyright ArtCraft Team and PhotoCraft contributors. `LICENSE-MIT`, `LICENSE-APACHE`, and `NOTICE` are copied byte-for-byte from the source repository. The raw EDT entry is kept private so callers cannot bypass admission. The EDT implementation and its test module are byte-preserved; operation, helper, caller, and test mappings are in `provenance/source-map.tsv`. No PhotoCraft UI, document, serializer, assets, or unrelated algorithms are included.
