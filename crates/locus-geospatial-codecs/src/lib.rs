//! Pure-Rust parsers and serializers for OGC WKB, ESRI FileGDB, ASPRS LAS LiDAR, LandXML,
//! MapInfo MIF/MID, KML, PMTiles v3, MVT, and ADS-B / AIS streaming telemetry.
//!
//! Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
//! and Informatics Fiduciary duties ensuring transparent open-codec interoperation.

#![forbid(unsafe_code)]

pub mod formats;
pub mod geometry;
pub mod telemetry;
pub mod tiles;
pub mod wkb;

pub use formats::{
    AlignmentElement, CivilAlignment, FileGdbShapeType, KmlPlacemark, LasHeader, LasPoint,
    MapInfoDataset, MifColumn, TinSurface, parse_filegdb_geometry, parse_kml, parse_landxml,
    parse_las_header, parse_las_points, parse_mif_mid, serialize_kml,
};
pub use geometry::{Geometry, Point2D, Point3D, Polygon2D};
pub use telemetry::{
    AircraftMessage, AisMessage, AttributeValue, ComparisonOp, CprFrame, FilterExpr,
    decode_adsb_packet, decode_aivdm_sentence, decode_cpr_global,
};
pub use tiles::{
    Compression, MvtFeature, MvtGeomType, PmTilesEntry, PmTilesHeader, TileType,
    decode_mvt_geometry, decode_pmtiles_directory, decode_zigzag, parse_pmtiles_header,
    zxy_to_tile_id,
};
pub use wkb::{decode_wkb, encode_wkb};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_wkb_roundtrip() {
        let pt = Geometry::Point(Point2D::new(12.34, 56.78));
        let bytes = encode_wkb(&pt);
        let decoded = decode_wkb(&bytes).expect("wkb point decode failed");
        assert_eq!(pt, decoded);

        let line = Geometry::LineString(vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(10.0, 10.0),
            Point2D::new(20.0, 0.0),
        ]);
        let line_bytes = encode_wkb(&line);
        let line_decoded = decode_wkb(&line_bytes).expect("wkb line decode failed");
        assert_eq!(line, line_decoded);

        let poly = Geometry::Polygon(Polygon2D::new(
            vec![
                Point2D::new(0.0, 0.0),
                Point2D::new(10.0, 0.0),
                Point2D::new(10.0, 10.0),
                Point2D::new(0.0, 10.0),
                Point2D::new(0.0, 0.0),
            ],
            vec![],
        ));
        let poly_bytes = encode_wkb(&poly);
        let poly_decoded = decode_wkb(&poly_bytes).expect("wkb poly decode failed");
        assert_eq!(poly, poly_decoded);
    }

    #[test]
    fn test_landxml_parsing() {
        let xml = r#"
        <LandXML>
            <Alignments>
                <Alignment name="MainSt" length="200.0">
                    <CoordGeom>
                        <Line>
                            <Start>0.0 0.0</Start>
                            <End>100.0 0.0</End>
                        </Line>
                    </CoordGeom>
                </Alignment>
            </Alignments>
            <Surfaces>
                <Surface name="Ground">
                    <Definition surfType="TIN">
                        <Pnts>
                            <P id="1">0.0 0.0 10.0</P>
                            <P id="2">10.0 0.0 10.0</P>
                            <P id="3">0.0 10.0 15.0</P>
                        </Pnts>
                        <Faces>
                            <F>1 2 3</F>
                        </Faces>
                    </Definition>
                </Surface>
            </Surfaces>
        </LandXML>
        "#;

        let (alignments, surfaces) = parse_landxml(xml);
        assert_eq!(alignments.len(), 1);
        assert_eq!(alignments[0].name, "MainSt");
        assert_eq!(surfaces.len(), 1);
        assert_eq!(surfaces[0].vertices.len(), 3);
        assert_eq!(surfaces[0].faces.len(), 1);
    }

    #[test]
    fn test_kml_parsing() {
        let kml = r#"
        <kml xmlns="http://www.opengis.net/kml/2.2">
          <Document>
            <Placemark>
              <name>TestPoint</name>
              <description>A test location</description>
              <Point>
                <coordinates>-122.0822035425683,37.42228990140251,0</coordinates>
              </Point>
            </Placemark>
          </Document>
        </kml>
        "#;

        let placemarks = parse_kml(kml);
        assert_eq!(placemarks.len(), 1);
        assert_eq!(placemarks[0].name, "TestPoint");
        if let Some(Geometry::Point(p)) = &placemarks[0].geometry {
            assert!((p.x - (-122.0822)).abs() < 1e-3);
            assert!((p.y - 37.4222).abs() < 1e-3);
        } else {
            panic!("Expected Point geometry");
        }
    }

    #[test]
    fn test_pmtiles_header_parsing() {
        let mut header_bytes = [0u8; 127];
        header_bytes[0..7].copy_from_slice(b"PMTiles");
        header_bytes[7] = 3; // version 3
        header_bytes[99] = 1; // Mvt tile type
        header_bytes[100] = 0; // min zoom
        header_bytes[101] = 14; // max zoom

        let header = parse_pmtiles_header(&header_bytes).expect("PMTiles header parse failed");
        assert_eq!(header.tile_type, TileType::Mvt);
        assert_eq!(header.min_zoom, 0);
        assert_eq!(header.max_zoom, 14);

        // Hilbert ID
        let tile_id = zxy_to_tile_id(1, 0, 0);
        assert!(tile_id > 0);
    }

    #[test]
    fn test_mvt_geometry_decoding() {
        // MoveTo (1, 1), LineTo (2, 2)
        // Command 1, count 1: (1 << 3) | 1 = 9
        // Zigzag for +1: 2, +1: 2
        // Command 2, count 1: (1 << 3) | 2 = 10
        // Zigzag for +1: 2, +1: 2
        let cmds = vec![9, 2, 2, 10, 2, 2];
        let geom = decode_mvt_geometry(MvtGeomType::LineString, &cmds).expect("MVT line decode failed");
        if let Geometry::LineString(line) = geom {
            assert_eq!(line.len(), 2);
            assert_eq!(line[0], Point2D::new(1.0, 1.0));
            assert_eq!(line[1], Point2D::new(2.0, 2.0));
        } else {
            panic!("Expected LineString");
        }
    }

    #[test]
    fn test_query_filter_evaluator() {
        let mut record = HashMap::new();
        record.insert("callsign".to_string(), AttributeValue::Text("QFA123".to_string()));
        record.insert("altitude".to_string(), AttributeValue::Number(35000.0));

        let expr = FilterExpr::And(vec![
            FilterExpr::Compare {
                field: "callsign".to_string(),
                op: ComparisonOp::Like,
                value: AttributeValue::Text("QFA%".to_string()),
            },
            FilterExpr::Compare {
                field: "altitude".to_string(),
                op: ComparisonOp::Gt,
                value: AttributeValue::Number(30000.0),
            },
        ]);

        assert!(expr.evaluate(&record));
    }
}
