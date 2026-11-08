//! VTK XML UnstructuredGrid (.vtu) file format exporter.
//!
//! Grounded in open scientific visualization and verifiable continuum results (UDHR Art. 19, 27).

use crate::mesh::tet::Point3D;

/// VTK cell types.
pub const VTK_TETRA: u8 = 10;
pub const VTK_HEXAHEDRON: u8 = 12;

/// Export hexahedral mesh to VTK XML UnstructuredGrid (.vtu) format with nodal displacements.
pub fn export_vtu_hex(
    nodes: &[Point3D],
    hex_elements: &[[usize; 8]],
    displacements: Option<&[[f64; 3]]>,
) -> String {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\"?>\n");
    out.push_str("<VTKFile type=\"UnstructuredGrid\" version=\"0.1\" byte_order=\"LittleEndian\">\n");
    out.push_str("  <UnstructuredGrid>\n");
    out.push_str(&format!(
        "    <Piece NumberOfPoints=\"{}\" NumberOfCells=\"{}\">\n",
        nodes.len(),
        hex_elements.len()
    ));

    // Points
    out.push_str("      <Points>\n");
    out.push_str("        <DataArray type=\"Float64\" NumberOfComponents=\"3\" format=\"ascii\">\n");
    for p in nodes {
        out.push_str(&format!("          {} {} {}\n", p.x, p.y, p.z));
    }
    out.push_str("        </DataArray>\n");
    out.push_str("      </Points>\n");

    // Cells
    out.push_str("      <Cells>\n");
    out.push_str("        <DataArray type=\"Int64\" Name=\"connectivity\" format=\"ascii\">\n");
    for el in hex_elements {
        out.push_str(&format!(
            "          {} {} {} {} {} {} {} {}\n",
            el[0], el[1], el[2], el[3], el[4], el[5], el[6], el[7]
        ));
    }
    out.push_str("        </DataArray>\n");

    out.push_str("        <DataArray type=\"Int64\" Name=\"offsets\" format=\"ascii\">\n");
    for i in 1..=hex_elements.len() {
        out.push_str(&format!("          {}\n", i * 8));
    }
    out.push_str("        </DataArray>\n");

    out.push_str("        <DataArray type=\"UInt8\" Name=\"types\" format=\"ascii\">\n");
    for _ in 0..hex_elements.len() {
        out.push_str(&format!("          {}\n", VTK_HEXAHEDRON));
    }
    out.push_str("        </DataArray>\n");
    out.push_str("      </Cells>\n");

    // PointData (optional displacements)
    if let Some(disp) = displacements {
        out.push_str("      <PointData Vectors=\"Displacement\">\n");
        out.push_str("        <DataArray type=\"Float64\" Name=\"Displacement\" NumberOfComponents=\"3\" format=\"ascii\">\n");
        for d in disp {
            out.push_str(&format!("          {} {} {}\n", d[0], d[1], d[2]));
        }
        out.push_str("        </DataArray>\n");
        out.push_str("      </PointData>\n");
    }

    out.push_str("    </Piece>\n");
    out.push_str("  </UnstructuredGrid>\n");
    out.push_str("</VTKFile>\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vtu_xml_structure_test() {
        let nodes = [Point3D::new(0.0, 0.0, 0.0)];
        let hex = [[0; 8]];
        let vtu = export_vtu_hex(&nodes, &hex, None);
        assert!(vtu.contains("<VTKFile type=\"UnstructuredGrid\""));
        assert!(vtu.contains("<Points>"));
        assert!(vtu.contains("<Cells>"));
    }
}
