# `crates/locus-geospatial-codecs`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Geospatial container and exchange file format codecs._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl FileGdbShapeType::from_u32` | 1 | `crates/locus-geospatial-codecs/src/formats/filegdb.rs:27` | - |  |
| `parse_filegdb_geometry` | 1 | `crates/locus-geospatial-codecs/src/formats/filegdb.rs:47` | - | Decodes an ESRI FileGDB shape binary buffer into standard Geometry. |
| `parse_kml` | 1 | `crates/locus-geospatial-codecs/src/formats/kml.rs:15` | - | Parses KML XML text into a collection of Placemarks without external parser crates. |
| `extract_tag_value` | 2 | `crates/locus-geospatial-codecs/src/formats/kml.rs:43` | - |  |
| `parse_coordinates` | 1 | `crates/locus-geospatial-codecs/src/formats/kml.rs:51` | - |  |
| `parse_kml_geometry` | 1 | `crates/locus-geospatial-codecs/src/formats/kml.rs:62` | - |  |
| `serialize_kml` | 2 | `crates/locus-geospatial-codecs/src/formats/kml.rs:103` | - | Serializes placemarks into a valid KML 2.2 XML document string. |
| `parse_landxml` | 1 | `crates/locus-geospatial-codecs/src/formats/landxml.rs:31` | - | Parses LandXML content into alignments and TIN surfaces without external XML crate dependencies. |
| `extract_attr` | 2 | `crates/locus-geospatial-codecs/src/formats/landxml.rs:70` | - |  |
| `parse_alignment_block` | 1 | `crates/locus-geospatial-codecs/src/formats/landxml.rs:77` | - |  |
| `parse_surface_block` | 1 | `crates/locus-geospatial-codecs/src/formats/landxml.rs:103` | - |  |
| `extract_point_tag` | 2 | `crates/locus-geospatial-codecs/src/formats/landxml.rs:143` | - |  |
| `parse_las_header` | 1 | `crates/locus-geospatial-codecs/src/formats/lidar.rs:46` | - | Parses LAS header block from raw file bytes. |
| `parse_las_points` | 3 | `crates/locus-geospatial-codecs/src/formats/lidar.rs:104` | - | Decodes points from LAS bytes according to the header. |
| `parse_mif_mid` | 2 | `crates/locus-geospatial-codecs/src/formats/mapinfo.rs:21` | - | Parses a MapInfo MIF geometry stream and corresponding MID attribute lines. |
| `impl Point2D::new` | 2 | `crates/locus-geospatial-codecs/src/geometry.rs:13` | - |  |
| `impl Point3D::new` | 3 | `crates/locus-geospatial-codecs/src/geometry.rs:27` | - |  |
| `impl Point3D::to_2d` | 1 | `crates/locus-geospatial-codecs/src/geometry.rs:31` | - |  |
| `impl Polygon2D::new` | 2 | `crates/locus-geospatial-codecs/src/geometry.rs:44` | - |  |
| `test_wkb_roundtrip` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:38` | - |  |
| `test_landxml_parsing` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:69` | - |  |
| `test_kml_parsing` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:108` | - |  |
| `test_pmtiles_header_parsing` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:135` | - |  |
| `test_mvt_geometry_decoding` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:154` | - |  |
| `test_query_filter_evaluator` | 0 | `crates/locus-geospatial-codecs/src/lib.rs:172` | - |  |
| `decode_adsb_packet` | 1 | `crates/locus-geospatial-codecs/src/telemetry/aircraft.rs:28` | - | Decodes 14-byte (112-bit) raw ADS-B Mode S packet. |
| `decode_cpr_global` | 2 | `crates/locus-geospatial-codecs/src/telemetry/aircraft.rs:118` | - | Decodes geographic position (latitude, longitude) from an even and an odd CPR frame. |
| `nl_table` | 1 | `crates/locus-geospatial-codecs/src/telemetry/aircraft.rs:165` | - |  |
| `impl AttributeValue::as_f64` | 1 | `crates/locus-geospatial-codecs/src/telemetry/query.rs:16` | - |  |
| `impl AttributeValue::as_str` | 1 | `crates/locus-geospatial-codecs/src/telemetry/query.rs:24` | - |  |
| `impl FilterExpr::evaluate` | 2 | `crates/locus-geospatial-codecs/src/telemetry/query.rs:64` | - | Evaluates the expression against an attribute dictionary. |
| `compare_values` | 3 | `crates/locus-geospatial-codecs/src/telemetry/query.rs:82` | - |  |
| `match_sql_like` | 2 | `crates/locus-geospatial-codecs/src/telemetry/query.rs:110` | - | Simple SQL LIKE pattern matcher (% matches any sequence, _ matches single character). |
| `decode_ais_char` | 1 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:25` | - | Decodes an AIS 6-bit ASCII character to its 6-bit integer value (0..63). |
| `impl BitReader<'a>::new` | 1 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:42` | - |  |
| `impl BitReader<'a>::read_u32` | 2 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:46` | - |  |
| `impl BitReader<'a>::read_i32` | 2 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:65` | - |  |
| `impl BitReader<'a>::read_ascii` | 2 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:75` | - |  |
| `decode_aivdm_sentence` | 1 | `crates/locus-geospatial-codecs/src/telemetry/vessels.rs:91` | - | Decodes an NMEA 0183 `!AIVDM` sentence into an `AisMessage`. |
| `decode_zigzag` | 1 | `crates/locus-geospatial-codecs/src/tiles/mvt.rs:26` | - | Decodes MVT zigzag-encoded integer parameter: `(n >> 1) ^ -(n & 1)`. |
| `decode_mvt_geometry` | 2 | `crates/locus-geospatial-codecs/src/tiles/mvt.rs:31` | - | Decodes an MVT command integer sequence into vector Geometry. |
| `impl Compression::from_u8` | 1 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:16` | - |  |
| `impl TileType::from_u8` | 1 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:39` | - |  |
| `parse_pmtiles_header` | 1 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:81` | - | Parses PMTiles v3 header from a 127-byte slice. |
| `read_varint` | 2 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:164` | - | Decodes varint from byte stream. |
| `decode_pmtiles_directory` | 1 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:183` | - | Decodes an uncompressed PMTiles v3 directory into tile entries. |
| `zxy_to_tile_id` | 3 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:219` | - | Converts (zoom, x, y) tile coordinates to PMTiles Hilbert tile ID. |
| `hilbert_xy2d` | 3 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:230` | - |  |
| `rot` | 5 | `crates/locus-geospatial-codecs/src/tiles/pmtiles.rs:243` | - |  |
| `impl WkbReader<'a>::new` | 1 | `crates/locus-geospatial-codecs/src/wkb.rs:15` | - |  |
| `impl WkbReader<'a>::remaining` | 1 | `crates/locus-geospatial-codecs/src/wkb.rs:19` | - |  |
| `impl WkbReader<'a>::read_u8` | 1 | `crates/locus-geospatial-codecs/src/wkb.rs:23` | - |  |
| `impl WkbReader<'a>::read_u32` | 2 | `crates/locus-geospatial-codecs/src/wkb.rs:33` | - |  |
| `impl WkbReader<'a>::read_f64` | 2 | `crates/locus-geospatial-codecs/src/wkb.rs:43` | - |  |
| `impl WkbReader<'a>::read_point2d` | 4 | `crates/locus-geospatial-codecs/src/wkb.rs:53` | - |  |
| `impl WkbReader<'a>::read_point3d` | 3 | `crates/locus-geospatial-codecs/src/wkb.rs:65` | - |  |
| `impl WkbReader<'a>::read_geometry` | 2 | `crates/locus-geospatial-codecs/src/wkb.rs:75` | - |  |
| `decode_wkb` | 1 | `crates/locus-geospatial-codecs/src/wkb.rs:211` | - | Decodes a geometry from binary WKB bytes. |
| `encode_wkb` | 1 | `crates/locus-geospatial-codecs/src/wkb.rs:217` | - | Encodes a geometry as ISO WKB little-endian bytes. |
| `encode_geom_into` | 2 | `crates/locus-geospatial-codecs/src/wkb.rs:223` | - |  |
