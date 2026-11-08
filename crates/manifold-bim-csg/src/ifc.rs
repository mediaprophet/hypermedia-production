//! IFC4 / ISO 16739 architectural entity exporter.
//!
//! Produces ISO 10303-21 STEP physical files containing IFC4 building models:
//! projects, sites, buildings, storeys, walls, slabs, openings, and property sets.

use std::fmt::Write;

/// Formats a string as an ISO 10303-21 STEP string literal with Unicode escapes.
pub fn step_string(text: &str) -> String {
    let mut out = String::from("'");
    for c in text.chars() {
        match c {
            '\'' => out.push_str("''"),
            '\\' => out.push_str(r"\\"),
            ' '..='~' => out.push(c),
            _ if u32::from(c) > 0xFFFF => {
                let _ = write!(out, r"\X4\{:08X}\X0\", u32::from(c));
            }
            _ => {
                let _ = write!(out, r"\X2\{:04X}\X0\", u32::from(c));
            }
        }
    }
    out.push('\'');
    out
}

/// Generates a deterministic 22-character IFC GlobalId from a 64-bit integer seed.
pub fn ifc_guid(seed: u64, domain: u64) -> String {
    const B64: &[u8; 64] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$";
    let mut h = seed ^ (domain.wrapping_mul(0x517cc1b727220a95));
    let mut out = String::with_capacity(22);
    for _ in 0..22 {
        let idx = (h % 64) as usize;
        out.push(B64[idx] as char);
        h = h.rotate_right(5) ^ 0x9e3779b97f4a7c15;
    }
    out
}

/// Structured writer for generating IFC4 ISO 10303-21 text files.
pub struct IfcWriter {
    next_id: u64,
    body: String,
}

impl Default for IfcWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl IfcWriter {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            body: String::with_capacity(8192),
        }
    }

    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn write_entity(&mut self, name: &str, args: &str) -> u64 {
        let id = self.next_id();
        let _ = writeln!(self.body, "#{id}={name}({args});");
        id
    }

    /// Finishes the document, wrapping it in standard ISO-10303-21 HEADER and DATA sections.
    pub fn finish(self, project_name: &str) -> String {
        let mut out = String::with_capacity(self.body.len() + 512);
        out.push_str("ISO-10303-21;\nHEADER;\n");
        let _ = writeln!(
            out,
            "FILE_DESCRIPTION(('ViewDefinition [CoordinationView]'),'2;1');"
        );
        let _ = writeln!(
            out,
            "FILE_NAME({},'2026-11-08T00:00:00',('Qualia-Etalon-Manifold'),('Ecosystem'),'Etalon-IFC4-Engine','IFC4','');",
            step_string(project_name)
        );
        out.push_str("FILE_SCHEMA(('IFC4'));\nENDSEC;\nDATA;\n");
        out.push_str(&self.body);
        out.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
        out
    }
}

/// Architectural project exporter creating standard hierarchy: Project -> Site -> Building -> Storey -> Wall.
pub fn export_minimal_building(
    project_name: &str,
    building_name: &str,
    storey_name: &str,
    elevation: f64,
    wall_name: &str,
    wall_length: f64,
    wall_height: f64,
    wall_thickness: f64,
) -> String {
    let mut writer = IfcWriter::new();

    // Context & Units
    let length_unit = writer.write_entity("IFCSIUNIT", "*, .LENGTHUNIT., .MILLI., .METRE.");
    let area_unit = writer.write_entity("IFCSIUNIT", "*, .AREAUNIT., $, .SQUARE_METRE.");
    let volume_unit = writer.write_entity("IFCSIUNIT", "*, .VOLUMEUNIT., $, .CUBIC_METRE.");
    let unit_assignment = writer.write_entity(
        "IFCUNITASSIGNMENT",
        &format!("(#{length_unit}, #{area_unit}, #{volume_unit})"),
    );

    let origin = writer.write_entity("IFCCARTESIANPOINT", "((0.,0.,0.))");
    let axis3 = writer.write_entity("IFCDIRECTION", "((0.,0.,1.))");
    let axis1 = writer.write_entity("IFCDIRECTION", "((1.,0.,0.))");
    let world_placement =
        writer.write_entity("IFCAXIS2PLACEMENT3D", &format!("#{origin}, #{axis3}, #{axis1}"));
    let geom_context = writer.write_entity(
        "IFCGEOMETRICREPRESENTATIONCONTEXT",
        &format!("$, 'Model', 3, 1.E-05, #{world_placement}, $"),
    );

    // Project
    let proj_guid = ifc_guid(1, 100);
    let project = writer.write_entity(
        "IFCPROJECT",
        &format!(
            "'{}', $, {}, $, $, $, $, (#{geom_context}), #{unit_assignment}",
            proj_guid,
            step_string(project_name)
        ),
    );

    // Site & Building
    let site_guid = ifc_guid(2, 200);
    let site = writer.write_entity(
        "IFCSITE",
        &format!("'{}', $, 'Default Site', $, $, $, $, $, .ELEMENT., (0,0,0), (0,0,0), 0., $, $", site_guid),
    );

    let bldg_guid = ifc_guid(3, 300);
    let building = writer.write_entity(
        "IFCBUILDING",
        &format!(
            "'{}', $, {}, $, $, $, $, $, .ELEMENT., $, $, $",
            bldg_guid,
            step_string(building_name)
        ),
    );

    let storey_guid = ifc_guid(4, 400);
    let storey_placement = writer.write_entity(
        "IFCLOCALPLACEMENT",
        &format!("$, #{world_placement}"),
    );
    let storey = writer.write_entity(
        "IFCBUILDINGSTOREY",
        &format!(
            "'{}', $, {}, $, $, #{storey_placement}, $, $, .ELEMENT., {:.3}",
            storey_guid,
            step_string(storey_name),
            elevation
        ),
    );

    // Wall geometry (swept solid)
    let p_start = writer.write_entity("IFCCARTESIANPOINT", "((0., 0.))");
    let p_x = writer.write_entity("IFCDIRECTION", "((1., 0.))");
    let p_y = writer.write_entity("IFCDIRECTION", "((0., 1.))");
    let profile_placement = writer.write_entity("IFCAXIS2PLACEMENT2D", &format!("#{p_start}, #{p_x}"));
    let rect_profile = writer.write_entity(
        "IFCRECTANGLEPROFILEDEF",
        &format!(".AREA., 'WallProfile', #{profile_placement}, {:.3}, {:.3}", wall_length, wall_thickness),
    );

    let extrude_dir = writer.write_entity("IFCDIRECTION", "((0., 0., 1.))");
    let solid = writer.write_entity(
        "IFCEXTRUDEDAREASOLID",
        &format!("#{rect_profile}, #{world_placement}, #{extrude_dir}, {:.3}", wall_height),
    );

    let shape_rep = writer.write_entity(
        "IFCSHAPEREPRESENTATION",
        &format!("#{geom_context}, 'Body', 'SweptSolid', (#{solid})"),
    );
    let prod_shape = writer.write_entity("IFCPRODUCTDEFINITIONSHAPE", &format!("$, $, (#{shape_rep})"));

    let wall_placement = writer.write_entity(
        "IFCLOCALPLACEMENT",
        &format!("#{storey_placement}, #{world_placement}"),
    );
    let wall_guid = ifc_guid(5, 500);
    let wall = writer.write_entity(
        "IFCWALLSTANDARDCASE",
        &format!(
            "'{}', $, {}, $, $, #{wall_placement}, #{prod_shape}, $",
            wall_guid,
            step_string(wall_name)
        ),
    );

    // Relationships: RelAggregates (Project -> Site -> Building -> Storey) and RelContainedInSpatialStructure
    writer.write_entity("IFCRELAGGREGATES", &format!("'{}', $, $, $, #{project}, (#{site})", ifc_guid(6, 600)));
    writer.write_entity("IFCRELAGGREGATES", &format!("'{}', $, $, $, #{site}, (#{building})", ifc_guid(7, 700)));
    writer.write_entity("IFCRELAGGREGATES", &format!("'{}', $, $, $, #{building}, (#{storey})", ifc_guid(8, 800)));
    writer.write_entity("IFCRELCONTAINEDINSPATIALSTRUCTURE", &format!("'{}', $, $, $, (#{wall}), #{storey}", ifc_guid(9, 900)));

    writer.finish(project_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ifc_export_and_guid() {
        let guid = ifc_guid(42, 10);
        assert_eq!(guid.len(), 22);

        let ifc_text = export_minimal_building(
            "EcoStudio",
            "Main Pavilion",
            "Ground Level",
            0.0,
            "Exterior North Wall",
            6000.0,
            3000.0,
            250.0,
        );

        assert!(ifc_text.contains("FILE_SCHEMA(('IFC4'));"));
        assert!(ifc_text.contains("IFCPROJECT"));
        assert!(ifc_text.contains("IFCWALLSTANDARDCASE"));
        assert!(ifc_text.contains("IFCEXTRUDEDAREASOLID"));
        assert!(ifc_text.contains("6000"));
        assert!(ifc_text.contains("END-ISO-10303-21;"));
    }
}
