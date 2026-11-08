//! Gmsh .msh file format exporter.
//!
//! Grounded in open mesh format interoperability (UDHR Art. 19, 27).

use crate::mesh::tet::Point3D;

/// Export mesh to Gmsh MSH 2.2 ASCII format.
pub fn export_gmsh_22(nodes: &[Point3D], hex_elements: &[[usize; 8]]) -> String {
    let mut out = String::new();
    out.push_str("$MeshFormat\n2.2 0 8\n$EndMeshFormat\n");

    out.push_str("$Nodes\n");
    out.push_str(&format!("{}\n", nodes.len()));
    for (i, p) in nodes.iter().enumerate() {
        out.push_str(&format!("{} {} {} {}\n", i + 1, p.x, p.y, p.z));
    }
    out.push_str("$EndNodes\n");

    out.push_str("$Elements\n");
    out.push_str(&format!("{}\n", hex_elements.len()));
    // Gmsh element type 5 is 8-node hexahedron
    for (i, el) in hex_elements.iter().enumerate() {
        out.push_str(&format!(
            "{} 5 2 1 1 {} {} {} {} {} {} {} {}\n",
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
    out.push_str("$EndElements\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gmsh_format_headers() {
        let nodes = [Point3D::new(0.0, 0.0, 0.0)];
        let hex = [[0; 8]];
        let msh = export_gmsh_22(&nodes, &hex);
        assert!(msh.contains("$MeshFormat"));
        assert!(msh.contains("$Nodes"));
        assert!(msh.contains("$Elements"));
    }
}
