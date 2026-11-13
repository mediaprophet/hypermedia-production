//! Explicitly budgeted conforming union adapter. Candidate scratch is unpublished
//! until the native operation and the closed, oriented edge postcondition pass.
use qualia_core_db::specialized_libs::computational_geometry::{Point3, boolean_3::{bounded::{SourceFace, UnionBudget, UnionBuffers}, conforming::{union_buffer_bytes, union_conforming_bounded_into}}};

pub(super) fn closed_oriented(triangles: &[[u32; 3]], vertices: usize) -> Result<(), String> {
    if triangles.is_empty() { return Err("boolean surface is empty".into()); }
    let count = triangles.len().checked_mul(3).ok_or("topology size overflow")?;
    let mut edges = Vec::with_capacity(count);
    for &[a,b,c] in triangles {
        if a == b || b == c || a == c || [a,b,c].iter().any(|&x| x as usize >= vertices) {
            return Err("boolean mesh has an invalid triangle".into());
        }
        for (x,y) in [(a,b),(b,c),(c,a)] {
            edges.push((x.min(y), x.max(y), x < y));
        }
    }
    edges.sort_unstable();
    let mut i = 0;
    while i < edges.len() {
        if i+1 >= edges.len() || edges[i].0 != edges[i+1].0 || edges[i].1 != edges[i+1].1 || edges[i].2 == edges[i+1].2
            || (i+2 < edges.len() && edges[i].0 == edges[i+2].0 && edges[i].1 == edges[i+2].1) {
            return Err("boolean surface is open, nonmanifold or inconsistently oriented".into());
        }
        i += 2;
    }
    Ok(())
}

/// Caller must supply the existing mesh/work/byte budgets; no provider cap is raised.
pub(super) fn conforming_union(
    va: &[Point3], ta: &[[u32;3]], vb: &[Point3], tb: &[[u32;3]],
    budget_bytes: usize, budget_work: usize, cancelled: &dyn Fn() -> bool,
) -> Result<(Vec<Point3>, Vec<[u32;3]>, usize), String> {
    let vcap = va.len().checked_add(vb.len()).and_then(|n| ta.len().checked_mul(tb.len()).and_then(|p| p.checked_mul(2)).and_then(|p| n.checked_add(p))).unwrap_or(usize::MAX).min(200_000);
    let tcap = ta.len().checked_add(tb.len()).and_then(|n| ta.len().max(tb.len()).checked_add(1).and_then(|m| n.checked_mul(m))).and_then(|n| n.checked_mul(3)).unwrap_or(usize::MAX).min(400_000);
    let edge_bytes = ta.len().max(tb.len()).max(tcap).checked_mul(3).and_then(|v| v.checked_mul(std::mem::size_of::<(u32,u32,bool)>())).ok_or("topology budget overflow")?;
    let native_bytes = union_buffer_bytes(ta.len(), tb.len(), 0, vcap, tcap).map_err(|e| format!("conforming boolean budget: {e:?}"))?;
    let logical = native_bytes.checked_add(edge_bytes).ok_or("boolean byte budget overflow")?;
    if logical > budget_bytes { return Err(format!("conforming boolean requires {logical} bytes, exceeding available budget {budget_bytes}")); }
    closed_oriented(ta, va.len())?;
    closed_oriented(tb, vb.len())?;
    let mut corners_a = vec![[Point3::default();3]; ta.len()];
    let mut corners_b = vec![[Point3::default();3]; tb.len()];
    let mut current = [];
    let mut next = [];
    let mut vertices = vec![Point3::default(); vcap];
    let mut triangles = vec![[0u32;3]; tcap];
    let mut source_faces = vec![SourceFace::default(); tcap];
    let receipt = union_conforming_bounded_into(va,ta,vb,tb,UnionBudget { bytes:native_bytes,work:budget_work },UnionBuffers {
        corners_a:&mut corners_a,corners_b:&mut corners_b,current:&mut current,next:&mut next,
        vertices:&mut vertices,triangles:&mut triangles,source_faces:&mut source_faces,
    },cancelled).map_err(|e| format!("conforming boolean failed: {e:?}"))?;
    triangles.truncate(receipt.triangles); vertices.truncate(receipt.vertices);
    if triangles.is_empty() { return Err("conforming union returned an empty surface".into()); }
    closed_oriented(&triangles, vertices.len())?;
    Ok((vertices, triangles, receipt.work))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn topology_postcondition_requires_closed_opposite_edge_pairs() {
        let tetra = [[0,2,1],[0,1,3],[1,2,3],[2,0,3]];
        assert!(closed_oriented(&tetra,4).is_ok());
        assert!(closed_oriented(&tetra[..3],4).is_err());
        let bad = [[0,2,1],[0,1,3],[1,2,3],[0,2,3]];
        assert!(closed_oriented(&bad,4).is_err());
    }

    #[test]
    fn crossing_closed_tetrahedra_produce_one_closed_union() {
        let a = [Point3::new(0.,0.,0.),Point3::new(2.,0.,0.),Point3::new(0.,2.,0.),Point3::new(0.,0.,2.)];
        let b = [Point3::new(0.3,0.3,0.3),Point3::new(2.3,0.3,0.3),Point3::new(0.3,2.3,0.3),Point3::new(0.3,0.3,2.3)];
        let faces = [[0,2,1],[0,1,3],[1,2,3],[2,0,3]];
        let (vertices, triangles, work) = conforming_union(&a,&faces,&b,&faces,80*1024*1024,50_000_000,&|| false).unwrap();
        assert!(work > 0);
        assert!(vertices.len() >= 8 && triangles.len() > 8);
        closed_oriented(&triangles,vertices.len()).unwrap();
    }
}
