# `crates/surface-layered-codecs`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_# Surface Layered Codecs & Multi-Layer Document Exchange

Provides pure-Rust, memory-bounded, zero-unsafe codecs for layered raster document formats:
- Adobe Photoshop PSD (v1) and PSB (v2 64-bit Large Document Format) encoder and decoder
- Channel compression: Apple PackBits RLE and Raw codecs
- Layer records: geometry, clipping masks, blend mode tags (`8BIM`), and Pascal layer names
- Adobe Brush Resource (`.abr`) parser for legacy v1/v2 and modern v6+ sampled brush tips
- Serif Affinity suite container format reader (`.afphoto`, `.afdesign`, `.afpub`)
- GIMP XCF (`.xcf`) and OpenRaster (`.ora`) multi-layer image decoders

All execution and provenance guarantees are grounded in international human rights instruments
(UDHR Articles 12, 18, 19, 27; ICCPR Articles 17, 19; ICESCR Article 15) and Informatics Fiduciary duties._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `read_affinity_header` | 1 | `crates/surface-layered-codecs/src/affinity.rs:72` | - | Reads and validates the 72-byte Affinity container header. |
| `parse_affinity_archive` | 1 | `crates/surface-layered-codecs/src/affinity.rs:118` | - | Parses an Affinity archive, indexing its FAT entries and preview metadata. |
| `extract_affinity_preview` | 2 | `crates/surface-layered-codecs/src/affinity.rs:228` | - | Inspects the embedded preview thumbnail record without full image decode. |
| `test_packbits_rle_round_trip` | 0 | `crates/surface-layered-codecs/src/lib.rs:43` | - |  |
| `test_psd_encode_decode_round_trip` | 0 | `crates/surface-layered-codecs/src/lib.rs:59` | - |  |
| `test_psb_64bit_encode_decode` | 0 | `crates/surface-layered-codecs/src/lib.rs:110` | - |  |
| `test_abr_computed_and_sampled_parsing` | 0 | `crates/surface-layered-codecs/src/lib.rs:142` | - |  |
| `test_affinity_container_header_inspection` | 0 | `crates/surface-layered-codecs/src/lib.rs:171` | - |  |
| `test_xcf_parsing_synthetic` | 0 | `crates/surface-layered-codecs/src/lib.rs:191` | - |  |
| `test_open_raster_parsing_synthetic` | 0 | `crates/surface-layered-codecs/src/lib.rs:209` | - |  |
| `test_ontology_contracts` | 0 | `crates/surface-layered-codecs/src/lib.rs:247` | semantics |  |
| `impl NQuin48::new` | 5 | `crates/surface-layered-codecs/src/ontology.rs:27` | - |  |
| `impl NQuin48::verify_parity` | 1 | `crates/surface-layered-codecs/src/ontology.rs:39` | - |  |
| `q_hash` | 1 | `crates/surface-layered-codecs/src/ontology.rs:44` | - |  |
| `impl OntologicalContract::new` | 4 | `crates/surface-layered-codecs/src/ontology.rs:66` | semantics |  |
| `get_morphism_specs` | 0 | `crates/surface-layered-codecs/src/ontology.rs:93` | - |  |
| `get_descriptors` | 0 | `crates/surface-layered-codecs/src/ontology.rs:135` | - |  |
| `read_u16` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:10` | - |  |
| `read_i16` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:15` | - |  |
| `read_u32` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:20` | - |  |
| `read_i32` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:25` | - |  |
| `read_u64` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:30` | - |  |
| `encode` | 1 | `crates/surface-layered-codecs/src/psd/codec.rs:46` | - | Encodes data using PackBits run-length encoding. |
| `decode` | 2 | `crates/surface-layered-codecs/src/psd/codec.rs:84` | - | Decodes PackBits compressed data until `expected_len` bytes are produced. |
| `impl PsdDocument::new` | 4 | `crates/surface-layered-codecs/src/psd/codec.rs:130` | - | Creates a blank document with given dimensions and color mode. |
| `impl PsdDocument::decode` | 1 | `crates/surface-layered-codecs/src/psd/codec.rs:154` | - | Decodes a PSD or PSB document from bytes. |
| `impl PsdDocument::decode_layer_section` | 3 | `crates/surface-layered-codecs/src/psd/codec.rs:222` | - |  |
| `impl PsdDocument::encode` | 1 | `crates/surface-layered-codecs/src/psd/codec.rs:436` | - | Serializes the document into PSD or PSB binary format with PackBits compression. |
| `impl PsdVersion::is_psb` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:33` | - |  |
| `impl PsdVersion::to_u16` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:37` | - |  |
| `impl PsdVersion::from_u16` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:44` | - |  |
| `impl PsdColorMode::from_u16` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:67` | - |  |
| `impl PsdColorMode::channel_count` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:81` | - |  |
| `impl PsdCompression::from_u16` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:104` | - |  |
| `impl PsdHeader::decode` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:129` | - |  |
| `impl PsdHeader::encode` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:161` | - |  |
| `impl PsdBlendMode::from_key` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:214` | - |  |
| `impl PsdBlendMode::to_key` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:243` | - |  |
| `impl PsdLayer::width` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:290` | - |  |
| `impl PsdLayer::height` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:294` | - |  |
| `impl PsdLayer::to_rgba8` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:299` | - | Converts layer channel data to standard 8-bit RGBA pixels. |
| `parse_abr` | 1 | `crates/surface-layered-codecs/src/psd/mod.rs:383` | - | Parses an Adobe Brush (`.abr`) file from bytes. |
| `test_psd_header_and_blend_mode_keys` | 0 | `crates/surface-layered-codecs/src/psd/tests.rs:5` | - |  |
| `test_layer_rgba8_conversion` | 0 | `crates/surface-layered-codecs/src/psd/tests.rs:36` | - |  |
| `impl XcfColorType::from_u32` | 1 | `crates/surface-layered-codecs/src/xcf_ora.rs:30` | - |  |
| `parse_xcf` | 1 | `crates/surface-layered-codecs/src/xcf_ora.rs:67` | - | Decodes an XCF image file from bytes. |
| `parse_ora` | 1 | `crates/surface-layered-codecs/src/xcf_ora.rs:329` | - | Decodes an OpenRaster package (.ora) from raw ZIP archive bytes. |
| `read_zip_entries` | 1 | `crates/surface-layered-codecs/src/xcf_ora.rs:361` | - | Minimal pure-Rust ZIP local file record parser for stored (uncompressed) or raw members. |
| `parse_stack_xml` | 1 | `crates/surface-layered-codecs/src/xcf_ora.rs:410` | - | Parses dimensions and layers from `stack.xml`. |
| `extract_attribute` | 2 | `crates/surface-layered-codecs/src/xcf_ora.rs:471` | - | Helper to extract XML attribute value. |
