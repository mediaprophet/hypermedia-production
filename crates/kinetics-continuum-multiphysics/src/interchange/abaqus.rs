//! Abaqus finite element input deck (.inp) parser and exporter.
//!
//! Grounded in open engineering data interchange and auditability (UDHR Art. 19, 27).

use crate::mesh::tet::Point3D;

/// Export nodes and hexahedral elements to Abaqus `.inp` format.
pub fn export_abaqus_inp(
    nodes: &[Point3D],
    hex_elements: &[[usize; 8]],
    material_name: &str,
    youngs_modulus: f64,
    poissons_ratio: f64,
) -> String {
    let mut out = String::new();
    out.push_str("*HEADING\n");
    out.push_str(" Exported by Kinetics Continuum Multiphysics Engine\n");
    out.push_str("*NODE\n");
    for (i, p) in nodes.iter().enumerate() {
        out.push_str(&format!("{}, {:e}, {:e}, {:e}\n", i + 1, p.x, p.y, p.z));
    }

    out.push_str("*ELEMENT, TYPE=C3D8, ELSET=SOLID_PART\n");
    for (i, el) in hex_elements.iter().enumerate() {
        out.push_str(&format!(
            "{}, {}, {}, {}, {}, {}, {}, {}, {}\n",
            i + 1,
            el[0] + 1,
            el[1] + 1,
            el[2] + 1,
            el[3] + 1,
            el[4] + 1,
            el[5] + 1,
            el[6] + 1,
            el[7] + 1
        ));
    }

    out.push_str(&format!("*MATERIAL, NAME={}\n", material_name));
    out.push_str("*ELASTIC\n");
    out.push_str(&format!("{:e}, {}\n", youngs_modulus, poissons_ratio));

    out.push_str("*SOLID SECTION, ELSET=SOLID_PART, MATERIAL=");
    out.push_str(material_name);
    out.push('\n');

    out.push_str("*STEP\n*STATIC\n0.1, 1.0\n*END STEP\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abaqus_deck_format_test() {
        let nodes = [Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 0.0, 0.0)];
        let hex = [[0, 1, 1, 0, 0, 1, 1, 0]];
        let inp = export_abaqus_inp(&nodes, &hex, "STEEL", 210e9, 0.3);
        assert!(inp.contains("*NODE"));
        assert!(inp.contains("*ELEMENT, TYPE=C3D8"));
        assert!(inp.contains("*MATERIAL, NAME=STEEL"));
    }
}
