//! LandXML civil engineering alignments and TIN surface parser.

use crate::geometry::{Point2D, Point3D};
use serde::{Deserialize, Serialize};

/// A civil horizontal alignment element.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AlignmentElement {
    Line { start: Point2D, end: Point2D },
    Curve { start: Point2D, center: Point2D, end: Point2D, radius: f64 },
    Spiral { start: Point2D, end: Point2D, length: f64 },
}

/// A horizontal road/rail civil alignment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CivilAlignment {
    pub name: String,
    pub length: f64,
    pub elements: Vec<AlignmentElement>,
}

/// A Triangulated Irregular Network (TIN) surface.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TinSurface {
    pub name: String,
    pub vertices: Vec<Point3D>,
    pub faces: Vec<[usize; 3]>,
}

/// Parses LandXML content into alignments and TIN surfaces without external XML crate dependencies.
pub fn parse_landxml(xml_text: &str) -> (Vec<CivilAlignment>, Vec<TinSurface>) {
    let mut alignments = Vec::new();
    let mut surfaces = Vec::new();

    // Simple robust tag-based parser
    let mut pos = 0;
    while let Some(start_align) = xml_text[pos..].find("<Alignment") {
        let align_pos = pos + start_align;
        let end_align = match xml_text[align_pos..].find("</Alignment>") {
            Some(e) => align_pos + e + 12,
            None => break,
        };

        let block = &xml_text[align_pos..end_align];
        if let Some(align) = parse_alignment_block(block) {
            alignments.push(align);
        }
        pos = end_align;
    }

    // Parse surfaces
    pos = 0;
    while let Some(start_surf) = xml_text[pos..].find("<Surface") {
        let surf_pos = pos + start_surf;
        let end_surf = match xml_text[surf_pos..].find("</Surface>") {
            Some(e) => surf_pos + e + 10,
            None => break,
        };

        let block = &xml_text[surf_pos..end_surf];
        if let Some(surf) = parse_surface_block(block) {
            surfaces.push(surf);
        }
        pos = end_surf;
    }

    (alignments, surfaces)
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let pattern = format!("{attr}=\"");
    let start = tag.find(&pattern)? + pattern.len();
    let end = tag[start..].find('"')? + start;
    Some(tag[start..end].to_string())
}

fn parse_alignment_block(block: &str) -> Option<CivilAlignment> {
    let tag_end = block.find('>')?;
    let name = extract_attr(&block[..tag_end], "name").unwrap_or_else(|| "Alignment".to_string());
    let length = extract_attr(&block[..tag_end], "length")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);

    let mut elements = Vec::new();
    let mut pos = 0;

    while let Some(start_tag) = block[pos..].find("<Line") {
        let l_pos = pos + start_tag;
        let end_tag = match block[l_pos..].find("</Line>") {
            Some(e) => l_pos + e + 7,
            None => break,
        };
        let line_content = &block[l_pos..end_tag];
        if let (Some(start), Some(end)) = (extract_point_tag(line_content, "Start"), extract_point_tag(line_content, "End")) {
            elements.push(AlignmentElement::Line { start, end });
        }
        pos = end_tag;
    }

    Some(CivilAlignment { name, length, elements })
}

fn parse_surface_block(block: &str) -> Option<TinSurface> {
    let tag_end = block.find('>')?;
    let name = extract_attr(&block[..tag_end], "name").unwrap_or_else(|| "TIN".to_string());

    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    // Parse Pnts
    if let (Some(pnts_start), Some(pnts_end)) = (block.find("<Pnts>"), block.find("</Pnts>")) {
        let pnts_block = &block[pnts_start + 6..pnts_end];
        for line in pnts_block.split("<P") {
            if let Some(content_start) = line.find('>') {
                if let Some(content_end) = line.find("</P>") {
                    let text = line[content_start + 1..content_end].trim();
                    let nums: Vec<f64> = text.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                    if nums.len() >= 3 {
                        vertices.push(Point3D::new(nums[0], nums[1], nums[2]));
                    }
                }
            }
        }
    }

    // Parse Faces
    if let (Some(faces_start), Some(faces_end)) = (block.find("<Faces>"), block.find("</Faces>")) {
        let faces_block = &block[faces_start + 7..faces_end];
        for line in faces_block.split("<F>") {
            if let Some(end) = line.find("</F>") {
                let text = line[..end].trim();
                let nums: Vec<usize> = text.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                if nums.len() >= 3 {
                    faces.push([nums[0].saturating_sub(1), nums[1].saturating_sub(1), nums[2].saturating_sub(1)]);
                }
            }
        }
    }

    Some(TinSurface { name, vertices, faces })
}

fn extract_point_tag(block: &str, tag_name: &str) -> Option<Point2D> {
    let open = format!("<{tag_name}>");
    let close = format!("</{tag_name}>");
    let start = block.find(&open)? + open.len();
    let end = block.find(&close)?;
    let text = block[start..end].trim();
    let nums: Vec<f64> = text.split_whitespace().filter_map(|s| s.parse().ok()).collect();
    if nums.len() >= 2 {
        Some(Point2D::new(nums[0], nums[1]))
    } else {
        None
    }
}
