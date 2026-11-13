//! Bounded foliage-lobe tessellation with welded poles and surface-derived normals.
use super::WhitegumMeshBuilder;
use std::f32::consts::PI;

impl WhitegumMeshBuilder {
    pub(super) fn add_foliage_lobe(
        &mut self,
        center: [f32; 3],
        radii: [f32; 3],
        lat_segs: usize,
        lon_segs: usize,
        col_top: [u8; 4],
        col_bottom: [u8; 4],
        droop_factor: f32,
    ) {
        // Nine longitudinal cycles need more than the former 7/8 samples.
        let lat_n = lat_segs.clamp(12, 24);
        let lon_n = lon_segs.clamp(36, 72);
        let vertex_start = self.positions.len();
        let index_start = self.indices.len();
        let top = self.add_vertex(
            [center[0], center[1] + radii[1], center[2]],
            [0.; 3],
            [0.5, 0.],
            col_top,
        );
        for i in 1..lat_n {
            let v = i as f32 / lat_n as f32;
            let theta = v * PI;
            let (sin_t, cos_t) = theta.sin_cos();
            let mix = 0.5 * (1.0 - cos_t);
            let color = std::array::from_fn(|k| {
                (col_top[k] as f32 * (1.0 - mix) + col_bottom[k] as f32 * mix).round() as u8
            });
            for j in 0..=lon_n {
                let u = j as f32 / lon_n as f32;
                // Make the UV seam geometrically exact, including float rounding.
                let phi = if j == lon_n { 0.0 } else { u * 2.0 * PI };
                let (sin_p, cos_p) = phi.sin_cos();
                // Angular modulation must vanish at both poles. Otherwise the
                // old coincident X/Z pole ring becomes a jagged vertical line.
                let ripple = 1.0
                    + sin_t
                        * sin_t
                        * (0.14 * (5.0 * phi + 3.0 * theta).sin()
                            + 0.08 * (9.0 * phi - 4.0 * theta).cos());
                self.add_vertex(
                    [
                        center[0] + sin_t * cos_p * radii[0] * ripple,
                        center[1] + cos_t * radii[1] * ripple
                            - droop_factor * sin_t * sin_t * radii[1],
                        center[2] + sin_t * sin_p * radii[2] * ripple,
                    ],
                    [0.; 3],
                    [u, v],
                    color,
                );
            }
        }
        let bottom = self.add_vertex(
            [center[0], center[1] - radii[1], center[2]],
            [0.; 3],
            [0.5, 1.],
            col_bottom,
        );
        let first = top + 1;
        let stride = (lon_n + 1) as u32;
        for j in 0..lon_n as u32 {
            self.add_triangle(top, first + j + 1, first + j);
            let last = first + (lat_n as u32 - 2) * stride;
            self.add_triangle(bottom, last + j, last + j + 1);
        }
        for i in 0..lat_n as u32 - 2 {
            for j in 0..lon_n as u32 {
                let a = first + i * stride + j;
                // Longitude then latitude is outward for this Y-up parameterization.
                self.add_quad(a, a + 1, a + stride + 1, a + stride);
            }
        }
        // The deformation includes ellipsoid scale, scalloping and droop; sphere
        // normals do not describe that surface. Accumulate actual triangle areas.
        for triangle in self.indices[index_start..].chunks_exact(3) {
            let [a, b, c] = std::array::from_fn(|k| self.positions[triangle[k] as usize]);
            let ab: [f32; 3] = std::array::from_fn(|k| b[k] - a[k]);
            let ac: [f32; 3] = std::array::from_fn(|k| c[k] - a[k]);
            let n = [
                ab[1] * ac[2] - ab[2] * ac[1],
                ab[2] * ac[0] - ab[0] * ac[2],
                ab[0] * ac[1] - ab[1] * ac[0],
            ];
            for &vertex in triangle {
                for k in 0..3 {
                    self.normals[vertex as usize][k] += n[k];
                }
            }
        }
        // UV duplicates share one smooth normal across the wrap seam.
        for i in 0..lat_n - 1 {
            let a = first as usize + i * stride as usize;
            let b = a + lon_n;
            let n = std::array::from_fn(|k| self.normals[a][k] + self.normals[b][k]);
            self.normals[a] = n;
            self.normals[b] = n;
        }
        for n in &mut self.normals[vertex_start..] {
            let length = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            for component in n {
                *component /= length;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deformed_foliage_has_outward_faces_finite_normals_and_no_pole_or_wrap_cracks() {
        let mut mesh = WhitegumMeshBuilder::new();
        mesh.add_foliage_lobe([0.; 3], [1.15, 0.68, 1.12], 5, 8, [140; 4], [80; 4], 0.45);
        for tri in mesh.indices.chunks_exact(3) {
            let [a, b, c] = std::array::from_fn(|k| mesh.positions[tri[k] as usize]);
            let ab: [f32; 3] = std::array::from_fn(|k| b[k] - a[k]);
            let ac: [f32; 3] = std::array::from_fn(|k| c[k] - a[k]);
            let normal = [
                ab[1] * ac[2] - ab[2] * ac[1],
                ab[2] * ac[0] - ab[0] * ac[2],
                ab[0] * ac[1] - ab[1] * ac[0],
            ];
            assert!(
                normal.iter().map(|x| x * x).sum::<f32>() > 1e-12,
                "collapsed pole face"
            );
            let outward: f32 = (0..3).map(|k| normal[k] * (a[k] + b[k] + c[k])).sum();
            assert!(outward > 0.0, "inward foliage face");
        }
        for normal in &mesh.normals {
            assert!(normal.iter().all(|v| v.is_finite()));
            assert!((normal.iter().map(|v| v * v).sum::<f32>() - 1.0).abs() < 1e-5);
        }
        assert_eq!(
            mesh.positions
                .iter()
                .filter(|p| p[0] == 0.0 && p[2] == 0.0)
                .count(),
            2
        );
        for row in 0..11 {
            let a = 1 + row * 37;
            let b = a + 36;
            assert_eq!(mesh.positions[a], mesh.positions[b]);
            assert_eq!(mesh.normals[a], mesh.normals[b]);
        }
    }
}
