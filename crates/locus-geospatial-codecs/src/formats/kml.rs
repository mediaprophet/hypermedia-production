//! Safe Keyhole Markup Language (KML) vector parser and serializer.

use crate::geometry::{Geometry, Point2D, Polygon2D};
use serde::{Deserialize, Serialize};

/// A KML placemark feature.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KmlPlacemark {
    pub name: String,
    pub description: String,
    pub geometry: Option<Geometry>,
}

/// Parses KML XML text into a collection of Placemarks without external parser crates.
pub fn parse_kml(kml_text: &str) -> Vec<KmlPlacemark> {
    let mut placemarks = Vec::new();
    let mut pos = 0;

    while let Some(start) = kml_text[pos..].find("<Placemark") {
        let p_start = pos + start;
        let end = match kml_text[p_start..].find("</Placemark>") {
            Some(e) => p_start + e + 12,
            None => break,
        };

        let block = &kml_text[p_start..end];
        let name = extract_tag_value(block, "name").unwrap_or_default();
        let description = extract_tag_value(block, "description").unwrap_or_default();
        let geometry = parse_kml_geometry(block);

        placemarks.push(KmlPlacemark {
            name,
            description,
            geometry,
        });

        pos = end;
    }

    placemarks
}

fn extract_tag_value(block: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = block.find(&open)? + open.len();
    let end = block[start..].find(&close)? + start;
    Some(block[start..end].trim().to_string())
}

fn parse_coordinates(coord_text: &str) -> Vec<Point2D> {
    let mut points = Vec::new();
    for token in coord_text.split_whitespace() {
        let parts: Vec<f64> = token.split(',').filter_map(|s| s.parse().ok()).collect();
        if parts.len() >= 2 {
            points.push(Point2D::new(parts[0], parts[1]));
        }
    }
    points
}

fn parse_kml_geometry(block: &str) -> Option<Geometry> {
    if let Some(p_start) = block.find("<Point>") {
        if let Some(p_end) = block[p_start..].find("</Point>") {
            let p_block = &block[p_start..p_start + p_end];
            if let Some(coords_str) = extract_tag_value(p_block, "coordinates") {
                let pts = parse_coordinates(&coords_str);
                if let Some(pt) = pts.first() {
                    return Some(Geometry::Point(*pt));
                }
            }
        }
    }

    if let Some(l_start) = block.find("<LineString>") {
        if let Some(l_end) = block[l_start..].find("</LineString>") {
            let l_block = &block[l_start..l_start + l_end];
            if let Some(coords_str) = extract_tag_value(l_block, "coordinates") {
                let pts = parse_coordinates(&coords_str);
                if !pts.is_empty() {
                    return Some(Geometry::LineString(pts));
                }
            }
        }
    }

    if let Some(poly_start) = block.find("<Polygon>") {
        if let Some(poly_end) = block[poly_start..].find("</Polygon>") {
            let poly_block = &block[poly_start..poly_start + poly_end];
            if let Some(coords_str) = extract_tag_value(poly_block, "coordinates") {
                let pts = parse_coordinates(&coords_str);
                if !pts.is_empty() {
                    return Some(Geometry::Polygon(Polygon2D::new(pts, Vec::new())));
                }
            }
        }
    }

    None
}

/// Serializes placemarks into a valid KML 2.2 XML document string.
pub fn serialize_kml(name: &str, placemarks: &[KmlPlacemark]) -> String {
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<kml xmlns=\"http://www.opengis.net/kml/2.2\">\n  <Document>\n");
    out.push_str(&format!("    <name>{name}</name>\n"));

    for pm in placemarks {
        out.push_str("    <Placemark>\n");
        if !pm.name.is_empty() {
            out.push_str(&format!("      <name>{}</name>\n", pm.name));
        }
        if !pm.description.is_empty() {
            out.push_str(&format!("      <description>{}</description>\n", pm.description));
        }
        if let Some(geom) = &pm.geometry {
            match geom {
                Geometry::Point(p) => {
                    out.push_str(&format!("      <Point><coordinates>{},{}</coordinates></Point>\n", p.x, p.y));
                }
                Geometry::LineString(pts) => {
                    out.push_str("      <LineString><coordinates>");
                    for (i, p) in pts.iter().enumerate() {
                        if i > 0 { out.push(' '); }
                        out.push_str(&format!("{},{}", p.x, p.y));
                    }
                    out.push_str("</coordinates></LineString>\n");
                }
                Geometry::Polygon(poly) => {
                    out.push_str("      <Polygon><outerBoundaryIs><LinearRing><coordinates>");
                    for (i, p) in poly.exterior.iter().enumerate() {
                        if i > 0 { out.push(' '); }
                        out.push_str(&format!("{},{}", p.x, p.y));
                    }
                    out.push_str("</coordinates></LinearRing></outerBoundaryIs></Polygon>\n");
                }
                _ => {}
            }
        }
        out.push_str("    </Placemark>\n");
    }

    out.push_str("  </Document>\n</kml>\n");
    out
}
