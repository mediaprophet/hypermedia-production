//! High-Craft Scale-1 Whitegum / Ghost Gum (`Corymbia aparrerinja`) Whole-Organism
//! WBE Pipe-Model Bifurcating Crown, Pendulous Sclerophyll Canopy & Mycelial Rhizosphere Builder.
//!
//! Replaces simplistic disconnected cylinder sticks with continuous parallel-transported
//! spline tubes, smooth branch-collar unions, chalk-white Ghost Gum bark transitions,
//! layered pendulous eucalyptus foliage clusters, and a subsurface mycelial rhizosphere network.

use qualia_core_db::container_10d::provenance_section::ProvenanceSidecar;
use qualia_core_db::container_10d::{
    attach_material_section, attach_texture_coordinate_section, MaterialRecord, OpacityMode,
    ShadingModel, SubmeshRange,
};
use qualia_core_db::q_hash;
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::botanical_anatomy::{
    BotanicalEducationScale, CompiledBotanicalOrgan10d, DevelopmentalStage,
    HistologicalTissueLayer, TissueSubmeshSlice, GOLDEN_ANGLE,
};
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d_with_extras_and_frames, Compile10dExtras,
};
use std::f32::consts::PI;

#[path = "whitegum_foliage.rs"]
mod foliage;

struct WhitegumMeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    colors: Vec<[u8; 4]>,
    material_indices: Vec<u8>,
    indices: Vec<u32>,
    slices: Vec<TissueSubmeshSlice>,
    active_layer: Option<(HistologicalTissueLayer, u32, u32)>,
    current_mat_idx: u8,
}

impl WhitegumMeshBuilder {
    fn new() -> Self {
        Self {
            positions: Vec::with_capacity(8192),
            normals: Vec::with_capacity(8192),
            uvs: Vec::with_capacity(8192),
            colors: Vec::with_capacity(8192),
            material_indices: Vec::with_capacity(8192),
            indices: Vec::with_capacity(24576),
            slices: Vec::new(),
            active_layer: None,
            current_mat_idx: 0,
        }
    }

    fn begin_layer(&mut self, layer: HistologicalTissueLayer, mat_idx: u8) {
        if self.active_layer.is_some() {
            self.end_layer();
        }
        let v_start = self.positions.len() as u32;
        let i_start = self.indices.len() as u32;
        self.active_layer = Some((layer, v_start, i_start));
        self.current_mat_idx = mat_idx;
    }

    fn end_layer(&mut self) {
        if let Some((layer, v_start, i_start)) = self.active_layer.take() {
            let v_count = (self.positions.len() as u32).saturating_sub(v_start);
            let i_count = (self.indices.len() as u32).saturating_sub(i_start);
            self.slices.push(TissueSubmeshSlice {
                layer,
                vertex_start: v_start,
                vertex_count: v_count,
                index_start: i_start,
                index_count: i_count,
                plant_ontology_id: layer.plant_ontology_id(),
                common_name: layer.common_name(),
            });
        }
    }

    fn add_vertex(&mut self, pos: [f32; 3], norm: [f32; 3], uv: [f32; 2], col: [u8; 4]) -> u32 {
        let idx = self.positions.len() as u32;
        self.positions.push(pos);
        self.normals.push(norm);
        self.uvs.push(uv);
        self.colors.push(col);
        self.material_indices.push(self.current_mat_idx);
        idx
    }

    fn add_triangle(&mut self, i0: u32, i1: u32, i2: u32) {
        self.indices.push(i0);
        self.indices.push(i1);
        self.indices.push(i2);
    }

    fn add_quad(&mut self, i0: u32, i1: u32, i2: u32, i3: u32) {
        self.add_triangle(i0, i1, i2);
        self.add_triangle(i0, i2, i3);
    }

    /// Add a continuous, welded multi-station curved tube with parallel-transported ring frames.
    fn add_curved_tube(
        &mut self,
        pts: &[[f32; 3]],
        radii: &[f32],
        cols: &[[u8; 4]],
        radial_slices: usize,
        cap_tip: bool,
    ) {
        let m = pts.len();
        if m < 2 || radii.len() < m || cols.len() < m {
            return;
        }
        let n = radial_slices.max(5);
        let ring_stride = (n + 1) as u32;
        let base_idx = self.positions.len() as u32;

        let mut tangents = Vec::with_capacity(m);
        for s in 0..m {
            let p_prev = if s == 0 { pts[0] } else { pts[s - 1] };
            let p_next = if s + 1 >= m { pts[m - 1] } else { pts[s + 1] };
            let dx = p_next[0] - p_prev[0];
            let dy = p_next[1] - p_prev[1];
            let dz = p_next[2] - p_prev[2];
            let len = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-5);
            tangents.push([dx / len, dy / len, dz / len]);
        }

        let t0 = tangents[0];
        let ref_v = if t0[0].abs() < 0.8 {
            [1.0f32, 0.0, 0.0]
        } else {
            [0.0f32, 1.0, 0.0]
        };
        let dot0 = ref_v[0] * t0[0] + ref_v[1] * t0[1] + ref_v[2] * t0[2];
        let mut u = [
            ref_v[0] - dot0 * t0[0],
            ref_v[1] - dot0 * t0[1],
            ref_v[2] - dot0 * t0[2],
        ];
        let u_len = (u[0] * u[0] + u[1] * u[1] + u[2] * u[2]).sqrt().max(1e-5);
        u = [u[0] / u_len, u[1] / u_len, u[2] / u_len];

        for s in 0..m {
            let t = tangents[s];
            let dot_ut = u[0] * t[0] + u[1] * t[1] + u[2] * t[2];
            u = [
                u[0] - dot_ut * t[0],
                u[1] - dot_ut * t[1],
                u[2] - dot_ut * t[2],
            ];
            let ul = (u[0] * u[0] + u[1] * u[1] + u[2] * u[2]).sqrt();
            if ul > 1e-5 {
                u = [u[0] / ul, u[1] / ul, u[2] / ul];
            } else {
                u = [1.0, 0.0, 0.0];
            }
            let v = [
                t[1] * u[2] - t[2] * u[1],
                t[2] * u[0] - t[0] * u[2],
                t[0] * u[1] - t[1] * u[0],
            ];

            let center = pts[s];
            let r = radii[s].max(0.002);
            let col = cols[s];
            let v_coord = (s as f32) / ((m - 1) as f32);

            for i in 0..=n {
                let u_coord = (i as f32) / (n as f32);
                let angle = u_coord * 2.0 * PI;
                let (sin_a, cos_a) = angle.sin_cos();
                let norm = [
                    u[0] * cos_a + v[0] * sin_a,
                    u[1] * cos_a + v[1] * sin_a,
                    u[2] * cos_a + v[2] * sin_a,
                ];
                let pos = [
                    center[0] + norm[0] * r,
                    center[1] + norm[1] * r,
                    center[2] + norm[2] * r,
                ];
                self.add_vertex(pos, norm, [u_coord, v_coord], col);
            }
        }

        for s in 0..(m - 1) {
            let r0 = base_idx + (s as u32) * ring_stride;
            let r1 = r0 + ring_stride;
            for i in 0..(n as u32) {
                self.add_quad(r0 + i, r1 + i, r1 + i + 1, r0 + i + 1);
            }
        }

        if cap_tip {
            let tip_center = pts[m - 1];
            let tip_norm = tangents[m - 1];
            let tip_col = cols[m - 1];
            let tip_v = self.add_vertex(tip_center, tip_norm, [0.5, 1.0], tip_col);
            let last_ring = base_idx + ((m - 1) as u32) * ring_stride;
            for i in 0..(n as u32) {
                self.add_triangle(last_ring + i, tip_v, last_ring + i + 1);
            }
        }
    }


}

/// Evaluates a 4-point cubic Bezier curve at parameter `t` in `[0, 1]`.
fn cubic_bezier(p0: [f32; 3], p1: [f32; 3], p2: [f32; 3], p3: [f32; 3], t: f32) -> [f32; 3] {
    let u = 1.0 - t;
    let b0 = u * u * u;
    let b1 = 3.0 * u * u * t;
    let b2 = 3.0 * u * t * t;
    let b3 = t * t * t;
    [
        b0 * p0[0] + b1 * p1[0] + b2 * p2[0] + b3 * p3[0],
        b0 * p0[1] + b1 * p1[1] + b2 * p2[1] + b3 * p3[1],
        b0 * p0[2] + b1 * p1[2] + b2 * p2[2] + b3 * p3[2],
    ]
}

fn lerp_rgba(a: [u8; 4], b: [u8; 4], t: f32) -> [u8; 4] {
    let t = t.clamp(0.0, 1.0);
    [
        ((a[0] as f32) * (1.0 - t) + (b[0] as f32) * t).round() as u8,
        ((a[1] as f32) * (1.0 - t) + (b[1] as f32) * t).round() as u8,
        ((a[2] as f32) * (1.0 - t) + (b[2] as f32) * t).round() as u8,
        ((a[3] as f32) * (1.0 - t) + (b[3] as f32) * t).round() as u8,
    ]
}

/// Builds a high-craft Scale-1 Whole-Organism Whitegum / Ghost Gum (`Corymbia aparrerinja`)
/// `.10d` container with continuous multi-order WBE pipe-model bifurcating branches,
/// chalk-white Ghost Gum bark with basal stocking, dense pendulous sclerophyllous foliage crown,
/// and a glowing subsurface mycelial rhizosphere network.
pub fn build_whitegum_whole_organism_10d(
    species_id: &str,
    stage: DevelopmentalStage,
) -> Result<CompiledBotanicalOrgan10d, String> {
    let mut b = WhitegumMeshBuilder::new();

    let (height_m, bole_r, crown_r) = match stage {
        DevelopmentalStage::Seedling => (2.2_f32, 0.055_f32, 1.1_f32),
        DevelopmentalStage::Sapling => (7.5_f32, 0.16_f32, 3.4_f32),
        DevelopmentalStage::Mature => (16.5_f32, 0.42_f32, 6.8_f32),
        DevelopmentalStage::AncientHollowBearing => (18.5_f32, 0.58_f32, 8.2_f32),
    };

    // ------------------------------------------------------------------------
    // Layer 1: Subterranean Root System + Mycelial Rhizosphere Network (`PhellogenCorkCambium`)
    // ------------------------------------------------------------------------
    b.begin_layer(HistologicalTissueLayer::PhellogenCorkCambium, 0);

    // 1A. Basal Lignotuber swelling with epicormic meristem buds
    b.add_foliage_lobe(
        [0.0, -bole_r * 0.32, 0.0],
        [bole_r * 1.55, bole_r * 0.85, bole_r * 1.55],
        6,
        12,
        [136, 114, 88, 255],
        [104, 84, 62, 255],
        0.0,
    );

    // 1B. Deep Sinuous Primary Taproot
    let tap_depth = height_m * 0.24;
    let tap_pts: Vec<[f32; 3]> = (0..=6)
        .map(|s| {
            let t = s as f32 / 6.0;
            [
                bole_r * 0.14 * (t * PI * 1.5).sin(),
                -t * tap_depth,
                bole_r * 0.12 * (t * PI * 1.5).cos(),
            ]
        })
        .collect();
    let tap_radii: Vec<f32> = (0..=6)
        .map(|s| {
            let t = s as f32 / 6.0;
            bole_r * (0.78 * (1.0 - t).powf(0.65)).max(0.08)
        })
        .collect();
    let tap_cols: Vec<[u8; 4]> = (0..=6)
        .map(|s| lerp_rgba([128, 106, 82, 255], [88, 70, 52, 255], s as f32 / 6.0))
        .collect();
    b.add_curved_tube(&tap_pts, &tap_radii, &tap_cols, 8, true);

    // 1C. 6 Fluted Buttress Root-Collar Spurs -> Sinuous Lateral Roots -> Mycelial Hyphae Web
    let num_roots = 6;
    let mut root_tips = Vec::new();
    for i in 0..num_roots {
        let angle = (i as f32) * (2.0 * PI / num_roots as f32) + 0.18;
        let (sin_a, cos_a) = angle.sin_cos();
        let lat_reach = crown_r * (0.54 + 0.08 * ((i as f32 * 1.9).sin()));

        let p0 = [cos_a * bole_r * 0.55, bole_r * 1.85, sin_a * bole_r * 0.55];
        let p1 = [cos_a * bole_r * 1.45, -bole_r * 0.15, sin_a * bole_r * 1.45];
        let p2 = [
            cos_a * lat_reach * 0.52 - sin_a * bole_r * 0.35,
            -bole_r * 1.15,
            sin_a * lat_reach * 0.52 + cos_a * bole_r * 0.35,
        ];
        let p3 = [
            cos_a * lat_reach,
            -bole_r * 2.15,
            sin_a * lat_reach,
        ];

        let mut pts = Vec::with_capacity(7);
        let mut radii = Vec::with_capacity(7);
        let mut cols = Vec::with_capacity(7);
        for s in 0..=6 {
            let t = s as f32 / 6.0;
            pts.push(cubic_bezier(p0, p1, p2, p3, t));
            let r = bole_r * (0.48 * (1.0 - t * 0.82));
            radii.push(r);
            cols.push(lerp_rgba([138, 118, 96, 255], [96, 78, 58, 255], t));
        }
        b.add_curved_tube(&pts, &radii, &cols, 7, true);
        root_tips.push((pts[4], p3, angle));

        // Bifurcating secondary feeder roots + glowing ectomycorrhizal mycelial hyphae threads
        for side in [-1.0_f32, 1.0_f32] {
            let sub_angle = angle + side * 0.52;
            let (ss, cs) = sub_angle.sin_cos();
            let pf0 = pts[3];
            let pf1 = [
                pf0[0] + cs * lat_reach * 0.22,
                pf0[1] - bole_r * 0.45,
                pf0[2] + ss * lat_reach * 0.22,
            ];
            let pf2 = [
                pf1[0] + cs * lat_reach * 0.26,
                pf1[1] - bole_r * 0.55,
                pf1[2] + ss * lat_reach * 0.26,
            ];
            b.add_curved_tube(
                &[pf0, pf1, pf2],
                &[bole_r * 0.20, bole_r * 0.11, bole_r * 0.04],
                &[
                    [112, 92, 68, 255],
                    [138, 112, 74, 255],
                    [218, 178, 88, 255], // Mycelial Hartig net transition
                ],
                5,
                true,
            );

            // Mycelial Hyphae fungal network filament extending outward into rhizosphere
            let myc_end = [
                pf2[0] + (sub_angle + side * 0.35).cos() * lat_reach * 0.34,
                pf2[1] - bole_r * 0.25,
                pf2[2] + (sub_angle + side * 0.35).sin() * lat_reach * 0.34,
            ];
            let myc_mid = [
                (pf2[0] + myc_end[0]) * 0.5 + side * bole_r * 0.25,
                (pf2[1] + myc_end[1]) * 0.5 + bole_r * 0.12,
                (pf2[2] + myc_end[2]) * 0.5 - side * bole_r * 0.25,
            ];
            b.add_curved_tube(
                &[pf2, myc_mid, myc_end],
                &[bole_r * 0.042, bole_r * 0.028, bole_r * 0.018],
                &[
                    [232, 192, 96, 255],
                    [245, 212, 112, 255],
                    [188, 228, 148, 255],
                ],
                5,
                true,
            );
        }
    }

    // Circumferential Mycelial "Wood Wide Web" Anastomosis Ring linking adjacent root tips
    for i in 0..root_tips.len() {
        let (p_a, _, _) = root_tips[i];
        let (p_b, _, _) = root_tips[(i + 1) % root_tips.len()];
        let p_mid = [
            (p_a[0] + p_b[0]) * 0.5 * 1.12,
            (p_a[1] + p_b[1]) * 0.5 - bole_r * 0.22,
            (p_a[2] + p_b[2]) * 0.5 * 1.12,
        ];
        b.add_curved_tube(
            &[p_a, p_mid, p_b],
            &[bole_r * 0.032, bole_r * 0.026, bole_r * 0.032],
            &[
                [224, 184, 88, 255],
                [248, 218, 118, 255],
                [224, 184, 88, 255],
            ],
            5,
            false,
        );
    }
    b.end_layer();

    // ------------------------------------------------------------------------
    // Layer 2: Internal Heartwood & Pith Core (`HeartwoodAndPith`)
    // ------------------------------------------------------------------------
    b.begin_layer(HistologicalTissueLayer::HeartwoodAndPith, 1);
    let bole_fork_h = height_m * 0.56;
    b.add_curved_tube(
        &[
            [0.0, 0.0, 0.0],
            [bole_r * 0.12, bole_fork_h * 0.5, -bole_r * 0.08],
            [-bole_r * 0.08, bole_fork_h * 0.92, bole_r * 0.10],
        ],
        &[bole_r * 0.28, bole_r * 0.22, bole_r * 0.15],
        &[
            [107, 56, 41, 255],
            [118, 62, 46, 255],
            [125, 68, 50, 255],
        ],
        7,
        true,
    );
    b.end_layer();

    // ------------------------------------------------------------------------
    // Layer 3: Internal Active Sapwood Cylinder (`SapwoodSecondaryXylem`)
    // ------------------------------------------------------------------------
    b.begin_layer(HistologicalTissueLayer::SapwoodSecondaryXylem, 2);
    b.add_curved_tube(
        &[
            [0.0, 0.0, 0.0],
            [bole_r * 0.12, bole_fork_h * 0.5, -bole_r * 0.08],
            [-bole_r * 0.08, bole_fork_h * 0.95, bole_r * 0.10],
        ],
        &[bole_r * 0.52, bole_r * 0.42, bole_r * 0.28],
        &[
            [219, 201, 171, 255],
            [212, 194, 164, 255],
            [204, 186, 154, 255],
        ],
        8,
        true,
    );
    b.end_layer();

    // ------------------------------------------------------------------------
    // Layer 4: Luminous Chalk-White Ghost Gum Bole & 5-Order WBE Bifurcating Crown (`OuterRhytidome`)
    // ------------------------------------------------------------------------
    b.begin_layer(HistologicalTissueLayer::OuterRhytidome, 3);

    // 4A. Sinuous Ghost Gum Central Bole (`y = 0` to `bole_fork_h`) with basal tessellated stocking
    // transitioning smoothly into luminous chalk-white bark
    let bole_stations = 12;
    let mut bole_pts = Vec::with_capacity(bole_stations + 1);
    let mut bole_radii = Vec::with_capacity(bole_stations + 1);
    let mut bole_cols = Vec::with_capacity(bole_stations + 1);

    let p_bole0 = [0.0, -bole_r * 0.15, 0.0];
    let p_bole1 = [bole_r * 0.38, bole_fork_h * 0.34, -bole_r * 0.22];
    let p_bole2 = [-bole_r * 0.32, bole_fork_h * 0.70, bole_r * 0.28];
    let p_bole3 = [bole_r * 0.08, bole_fork_h, 0.0];

    for s in 0..=bole_stations {
        let t = s as f32 / bole_stations as f32;
        bole_pts.push(cubic_bezier(p_bole0, p_bole1, p_bole2, p_bole3, t));
        let buttress_flare = if t < 0.20 {
            1.0 + 0.42 * (1.0 - t / 0.20).powi(2)
        } else {
            1.0
        };
        // Slight swelling at upper crown fork union
        let fork_swell = if t > 0.85 {
            1.0 + 0.12 * ((t - 0.85) / 0.15)
        } else {
            1.0
        };
        let r = bole_r * (1.0 - 0.34 * t) * buttress_flare * fork_swell;
        bole_radii.push(r);

        // Basal rough ochre-grey stocking (0..18%) -> creamy transition -> luminous chalk-white Ghost Gum bark
        let col = if t < 0.15 {
            lerp_rgba([112, 92, 76, 255], [154, 134, 114, 255], t / 0.15)
        } else if t < 0.26 {
            lerp_rgba([154, 134, 114, 255], [238, 235, 226, 255], (t - 0.15) / 0.11)
        } else {
            // Subtle warm cream / ghost-white modulation along upper trunk
            let patch = ((t * 14.0).sin() * 0.5 + 0.5) * 0.35;
            lerp_rgba([245, 243, 236, 255], [234, 224, 212, 255], patch)
        };
        bole_cols.push(col);
    }
    b.add_curved_tube(&bole_pts, &bole_radii, &bole_cols, 14, true);

    // 4B. Multi-Order WBE Pipe-Model Bifurcating Ghost Gum Boughs, Secondary Limbs & Tertiary Branchlets
    // In Corymbia aparrerinja, major chalk-white scaffold boughs ascend steeply from mid/upper bole,
    // arch gracefully outward, and repeatedly fork into twisted white/cream secondary and tertiary limbs.
    let bough_specs: [(f32, f32, f32, f32, f32); 7] = [
        // (bole_t, azimuth_rad, reach_frac, height_gain_frac, base_radius_frac)
        (0.54, 0.15, 0.88, 0.52, 0.48),
        (0.62, 1.12, 0.92, 0.58, 0.46),
        (0.68, 2.18, 0.85, 0.54, 0.44),
        (0.75, 3.15, 0.94, 0.62, 0.47),
        (0.82, 4.18, 0.86, 0.56, 0.43),
        (0.90, 5.22, 0.82, 0.64, 0.45),
        (0.98, 0.72, 0.64, 0.78, 0.52), // Central ascending crown leader bough
    ];

    let mut canopy_anchors: Vec<([f32; 3], f32, f32, bool)> = Vec::new();
    let mut gumnut_nodes: Vec<([f32; 3], f32)> = Vec::new();

    for (b_idx, &(bole_t, azim, reach_f, h_gain_f, r_frac)) in bough_specs.iter().enumerate() {
        let bole_station_f = bole_t * (bole_stations as f32);
        let s_idx = (bole_station_f.floor() as usize).min(bole_stations - 1);
        let s_rem = bole_station_f - (s_idx as f32);
        let p_origin = [
            bole_pts[s_idx][0] * (1.0 - s_rem) + bole_pts[s_idx + 1][0] * s_rem,
            bole_pts[s_idx][1] * (1.0 - s_rem) + bole_pts[s_idx + 1][1] * s_rem,
            bole_pts[s_idx][2] * (1.0 - s_rem) + bole_pts[s_idx + 1][2] * s_rem,
        ];
        let parent_r = bole_radii[s_idx];
        let r_bough_base = parent_r * r_frac;
        // Leonardo da Vinci / Shinozaki WBE Pipe Model tapering (exponent eta = 2.49)
        let r_bough_tip = r_bough_base * 0.56;

        let reach = crown_r * reach_f;
        let h_gain = (height_m - bole_fork_h) * h_gain_f;
        let (sin_a, cos_a) = azim.sin_cos();
        let perp_x = -sin_a;
        let perp_z = cos_a;
        let crook_sign = if b_idx % 2 == 0 { 1.0_f32 } else { -1.0_f32 };

        // 4-point cubic Bezier for Primary Bough (steep upward launch, graceful outward arch, slight lateral crook)
        let bp0 = p_origin;
        let bp1 = [
            p_origin[0] + cos_a * reach * 0.24 + perp_x * bole_r * 0.35 * crook_sign,
            p_origin[1] + h_gain * 0.48,
            p_origin[2] + sin_a * reach * 0.24 + perp_z * bole_r * 0.35 * crook_sign,
        ];
        let bp2 = [
            p_origin[0] + cos_a * reach * 0.58 - perp_x * bole_r * 0.28 * crook_sign,
            p_origin[1] + h_gain * 0.86,
            p_origin[2] + sin_a * reach * 0.58 - perp_z * bole_r * 0.28 * crook_sign,
        ];
        let bp3 = [
            p_origin[0] + cos_a * reach * 0.78,
            p_origin[1] + h_gain * 0.92,
            p_origin[2] + sin_a * reach * 0.78,
        ];

        let mut bough_pts = Vec::with_capacity(8);
        let mut bough_radii = Vec::with_capacity(8);
        let mut bough_cols = Vec::with_capacity(8);
        for s in 0..=7 {
            let t = s as f32 / 7.0;
            bough_pts.push(cubic_bezier(bp0, bp1, bp2, bp3, t));
            // Smooth branch collar flare at trunk junction
            let collar = if s == 0 { 1.24 } else if s == 1 { 1.06 } else { 1.0 };
            let r = (r_bough_base * (1.0 - t) + r_bough_tip * t) * collar;
            bough_radii.push(r);
            bough_cols.push(lerp_rgba(
                [244, 242, 234, 255],
                [236, 232, 222, 255],
                t,
            ));
        }
        b.add_curved_tube(&bough_pts, &bough_radii, &bough_cols, 10, true);

        // Bifurcate each Primary Bough into 3 Secondary Ghost Gum Limbs:
        // one mid-bough lateral fork (at t = 0.57) and a terminal dichotomous fork (2 limbs at t = 1.0)
        let sec_forks: [(usize, f32, f32, f32); 3] = [
            (4, -0.52 * crook_sign, 0.44, 0.38), // Mid-bough lateral secondary limb
            (7, 0.42, 0.48, 0.46),               // Terminal fork A
            (7, -0.38, 0.42, 0.32),              // Terminal fork B
        ];

        for (f_idx, &(parent_st, delta_azim, len_frac, pitch_frac)) in sec_forks.iter().enumerate() {
            let sp0 = bough_pts[parent_st];
            let r_sec_base = bough_radii[parent_st] * 0.76; // Pipe model daughter radius (2^(-1/2.49) ≈ 0.757)
            let r_sec_tip = r_sec_base * 0.52;
            let sec_azim = azim + delta_azim;
            let (sin_s, cos_s) = sec_azim.sin_cos();
            let sec_len = crown_r * len_frac;
            let sec_h = (height_m - bole_fork_h) * pitch_frac;

            let sp1 = [
                sp0[0] + cos_s * sec_len * 0.36,
                sp0[1] + sec_h * 0.62,
                sp0[2] + sin_s * sec_len * 0.36,
            ];
            let sp2 = [
                sp0[0] + cos_s * sec_len * 0.74,
                sp0[1] + sec_h * 0.95,
                sp0[2] + sin_s * sec_len * 0.74,
            ];
            let sp3 = [
                sp0[0] + cos_s * sec_len,
                sp0[1] + sec_h * 0.84, // Gentle outer arch
                sp0[2] + sin_s * sec_len,
            ];

            let mut sec_pts = Vec::with_capacity(6);
            let mut sec_radii = Vec::with_capacity(6);
            let mut sec_cols = Vec::with_capacity(6);
            for s in 0..=5 {
                let t = s as f32 / 5.0;
                sec_pts.push(cubic_bezier(sp0, sp1, sp2, sp3, t));
                let collar = if s == 0 { 1.15 } else { 1.0 };
                sec_radii.push((r_sec_base * (1.0 - t) + r_sec_tip * t) * collar);
                sec_cols.push(lerp_rgba(
                    [238, 235, 225, 255],
                    [224, 218, 204, 255],
                    t,
                ));
            }
            b.add_curved_tube(&sec_pts, &sec_radii, &sec_cols, 7, true);

            // Add a sub-canopy foliage anchor on the secondary limb shoulder
            canopy_anchors.push((sec_pts[3], crown_r * 0.20, sec_azim, false));

            // Bifurcate each Secondary Limb into 3 Tertiary & Weeping Quaternary Branchlets
            for t_idx in 0..3 {
                let tert_parent_st = if t_idx == 0 { 3 } else { 5 };
                let tp0 = sec_pts[tert_parent_st];
                let r_tert_base = sec_radii[tert_parent_st] * 0.74;
                let r_tert_tip = (r_tert_base * 0.36).max(bole_r * 0.032);
                let spread = (t_idx as f32 - 1.0) * 0.56 + (f_idx as f32 * 0.12);
                let tert_azim = sec_azim + spread;
                let (sin_t, cos_t) = tert_azim.sin_cos();
                let tert_len = crown_r * 0.28;

                // Tertiary branch arches outward then droops into a pendulous quaternary shoot
                let tp1 = [
                    tp0[0] + cos_t * tert_len * 0.45,
                    tp0[1] + tert_len * 0.22,
                    tp0[2] + sin_t * tert_len * 0.45,
                ];
                let tp2 = [
                    tp0[0] + cos_t * tert_len * 0.85,
                    tp0[1] + tert_len * 0.04,
                    tp0[2] + sin_t * tert_len * 0.85,
                ];
                let tp3 = [
                    tp0[0] + cos_t * tert_len * 1.05,
                    tp0[1] - tert_len * 0.32, // Characteristic weeping eucalyptus branchlet tip
                    tp0[2] + sin_t * tert_len * 1.05,
                ];

                let mut tert_pts = Vec::with_capacity(5);
                let mut tert_radii = Vec::with_capacity(5);
                let mut tert_cols = Vec::with_capacity(5);
                for s in 0..=4 {
                    let t = s as f32 / 4.0;
                    tert_pts.push(cubic_bezier(tp0, tp1, tp2, tp3, t));
                    tert_radii.push(r_tert_base * (1.0 - t) + r_tert_tip * t);
                    tert_cols.push(lerp_rgba(
                        [226, 220, 206, 255],
                        [182, 166, 142, 255],
                        t,
                    ));
                }
                b.add_curved_tube(&tert_pts, &tert_radii, &tert_cols, 6, true);

                let cluster_r = crown_r * (0.24 + 0.04 * ((b_idx + t_idx) as f32 % 3.0));
                canopy_anchors.push((tp2, cluster_r, tert_azim, true));
                gumnut_nodes.push((tp2, tert_azim));
            }
        }
    }
    b.end_layer();

    // ------------------------------------------------------------------------
    // Layer 5: Woody Gumnut Capsule Clusters (`WoodyPericarpAndDehiscingValves`)
    // ------------------------------------------------------------------------
    b.begin_layer(HistologicalTissueLayer::WoodyPericarpAndDehiscingValves, 5);
    let nut_r = (bole_r * 0.14).clamp(0.015, 0.065);
    for (idx, &(p_node, azim)) in gumnut_nodes.iter().enumerate() {
        if idx % 2 == 0 {
            for k in 0..3 {
                let ka = azim + (k as f32 - 1.0) * 0.85;
                let p_nut = [
                    p_node[0] + ka.cos() * nut_r * 2.4,
                    p_node[1] - nut_r * 2.2,
                    p_node[2] + ka.sin() * nut_r * 2.4,
                ];
                b.add_curved_tube(
                    &[p_node, p_nut],
                    &[nut_r * 0.38, nut_r],
                    &[[117, 97, 76, 255], [92, 72, 54, 255]],
                    5,
                    true,
                );
            }
        }
    }
    b.end_layer();

    // ------------------------------------------------------------------------
    // Layer 6: Multi-Layered Pendulous Eucalyptus Foliage Canopy & Falcate Leaves (`WaxyCuticle`)
    // ------------------------------------------------------------------------
    b.begin_layer(HistologicalTissueLayer::WaxyCuticle, 4);

    for (c_idx, &(anchor, cluster_r, azim, is_outer)) in canopy_anchors.iter().enumerate() {
        let is_young_flush = c_idx % 5 == 0;
        let col_top = if is_young_flush {
            [134, 158, 112, 255] // Sunlit sage-copper flush
        } else {
            [116, 148, 114, 255] // Glaucous silvery-sage eucalyptus upper crown
        };
        let col_bottom = [64, 96, 68, 255]; // Shaded deep sclerophyll green underside

        // Main billowing eucalyptus crown sub-volume
        b.add_foliage_lobe(
            [anchor[0], anchor[1] + cluster_r * 0.12, anchor[2]],
            [cluster_r * 1.15, cluster_r * 0.68, cluster_r * 1.12],
            5,
            8,
            col_top,
            col_bottom,
            0.32,
        );

        if is_outer {
            // Secondary pendulous satellite leaf curtain hanging below the outer branchlet
            let (sin_a, cos_a) = azim.sin_cos();
            b.add_foliage_lobe(
                [
                    anchor[0] + cos_a * cluster_r * 0.42,
                    anchor[1] - cluster_r * 0.36,
                    anchor[2] + sin_a * cluster_r * 0.42,
                ],
                [cluster_r * 0.78, cluster_r * 0.62, cluster_r * 0.78],
                4,
                7,
                [106, 138, 104, 255],
                [56, 86, 60, 255],
                0.45,
            );

            // Individual pendulous falcate-lanceolate leaf sprays hanging from the lower edge
            let leaf_len = (cluster_r * 0.58).clamp(0.18, 0.85);
            let leaf_w = leaf_len * 0.22;
            for l in 0..5 {
                let l_angle = azim + (l as f32) * GOLDEN_ANGLE;
                let (sin_l, cos_l) = l_angle.sin_cos();
                let rad_off = cluster_r * (0.28 + 0.12 * (l as f32));
                let lx = anchor[0] + cos_l * rad_off;
                let lz = anchor[2] + sin_l * rad_off;
                let ly = anchor[1] - cluster_r * 0.18;

                let v0 = b.add_vertex(
                    [lx, ly, lz],
                    [cos_l, 0.2, sin_l],
                    [0.5, 0.0],
                    [118, 150, 112, 255],
                );
                let v1 = b.add_vertex(
                    [
                        lx - sin_l * leaf_w + cos_l * leaf_w * 0.3,
                        ly - leaf_len * 0.48,
                        lz + cos_l * leaf_w + sin_l * leaf_w * 0.3,
                    ],
                    [cos_l, 0.0, sin_l],
                    [0.0, 0.5],
                    [88, 124, 92, 255],
                );
                let v2 = b.add_vertex(
                    [
                        lx + sin_l * leaf_w + cos_l * leaf_w * 0.3,
                        ly - leaf_len * 0.48,
                        lz - cos_l * leaf_w + sin_l * leaf_w * 0.3,
                    ],
                    [cos_l, 0.0, sin_l],
                    [1.0, 0.5],
                    [88, 124, 92, 255],
                );
                let v3 = b.add_vertex(
                    [
                        lx + cos_l * leaf_w * 0.85,
                        ly - leaf_len,
                        lz + sin_l * leaf_w * 0.85,
                    ],
                    [cos_l, -0.2, sin_l],
                    [0.5, 1.0],
                    if is_young_flush && l == 0 {
                        [158, 98, 68, 255]
                    } else {
                        [72, 106, 78, 255]
                    },
                );
                b.add_triangle(v0, v1, v2);
                b.add_triangle(v1, v3, v2);
            }
        }
    }
    b.end_layer();

    let recipe_urn = format!("urn:qualia:botanical:scale1:whitegum-v2:{species_id}:{stage:?}");
    let container_bytes = seal_whitegum_organ_10d(
        BotanicalEducationScale::Scale1WholeOrganismAndRoots,
        &b.positions,
        &b.normals,
        &b.colors,
        &b.uvs,
        &b.indices,
        &b.slices,
        &recipe_urn,
    )?;

    let organ = CompiledBotanicalOrgan10d {
        scale: BotanicalEducationScale::Scale1WholeOrganismAndRoots,
        vertices: b.positions,
        normals: b.normals,
        uvs: b.uvs,
        colors: b.colors,
        material_indices: b.material_indices,
        indices: b.indices,
        tissue_slices: b.slices,
        container_bytes,
    };

    organ.verify_all()?;
    Ok(organ)
}

fn seal_whitegum_organ_10d(
    scale: BotanicalEducationScale,
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    colors: &[[u8; 4]],
    uvs: &[[f32; 2]],
    indices: &[u32],
    slices: &[TissueSubmeshSlice],
    recipe_urn: &str,
) -> Result<Vec<u8>, String> {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in positions {
        for k in 0..3 {
            min[k] = min[k].min(p[k]);
            max[k] = max[k].max(p[k]);
        }
    }
    let triangles: Vec<[u32; 3]> = indices
        .chunks_exact(3)
        .map(|tri| [tri[0], tri[1], tri[2]])
        .collect();
    let mesh = Mesh {
        positions: positions.to_vec(),
        triangles,
        min,
        max,
    };

    let colors_f32: Vec<[f32; 4]> = colors
        .iter()
        .map(|c| {
            [
                c[0] as f32 / 255.0,
                c[1] as f32 / 255.0,
                c[2] as f32 / 255.0,
                c[3] as f32 / 255.0,
            ]
        })
        .collect();

    let provenance = ProvenanceSidecar::new(
        recipe_urn.as_bytes().to_vec(),
        "application/vnd.ecopunk.botanical-organ;version=2",
        "CC-BY-NC-ND-4.0",
    );

    let mut bytes = compile_mesh_to_10d_with_extras_and_frames(
        &mesh,
        &[],
        Some(&provenance),
        Compile10dExtras::default(),
        Some(&colors_f32),
        Some(normals),
        None,
    )
    .map_err(|e| format!("compile_mesh_to_10d_with_extras_and_frames error: {e}"))?;

    let mut materials = Vec::new();
    let mut ranges = Vec::new();
    for (idx, slice) in slices.iter().enumerate() {
        if slice.index_count == 0 {
            continue;
        }
        let (roughness, metallic) = slice.layer.default_pbr_material();
        let base_col = slice.layer.default_color_rgba();
        let mat_id = q_hash(&format!(
            "urn:qualia:material:ecopunk:botanical:{scale:?}:{idx}"
        ));
        let mut mat = MaterialRecord::legacy_default();
        mat.id = mat_id;
        mat.shading_model = ShadingModel::MetallicRoughness;
        mat.opacity_mode = OpacityMode::Opaque;
        mat.double_sided = matches!(
            slice.layer,
            HistologicalTissueLayer::WaxyCuticle
                | HistologicalTissueLayer::StaminalRingFilaments
                | HistologicalTissueLayer::WoodyPericarpAndDehiscingValves
        );
        mat.casts_shadow = true;
        mat.receives_shadow = true;
        mat.pickable = true;
        mat.multiply_vertex_color = true;
        mat.wind_animated = matches!(slice.layer, HistologicalTissueLayer::WaxyCuticle);
        mat.base_color = base_col;
        // A crown volume averages many leaf orientations; it is not a waxy solid.
        mat.roughness = if slice.layer == HistologicalTissueLayer::WaxyCuticle { 0.68 } else { roughness };
        mat.metallic = metallic;
        mat.specular = 0.5;
        materials.push(mat);
        ranges.push(SubmeshRange {
            first_index: slice.index_start,
            index_count: slice.index_count,
            material_id: mat_id,
            semantic_id: mat_id,
        });
    }
    if !materials.is_empty() {
        materials.sort_by_key(|m| m.id);
        bytes = attach_material_section(&bytes, &materials, &ranges, indices.len() as u32)
            .map_err(|e| format!("attach_material_section error: {e:?}"))?;
    }

    bytes = attach_texture_coordinate_section(&bytes, uvs)
        .map_err(|e| format!("attach_texture_coordinate_section error: {e:?}"))?;

    Ok(bytes)
}
