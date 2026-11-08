//! surface-pbr-studio: UI-agnostic categorical PBR surface and radiance studio.
//!
//! Provides zero-allocation BVH ray acceleration, multi-channel PBR layer compositing,
//! 3D UV texture painting, ray-cast mesh map baking, OpenPBR/MaterialX 1.39 export,
//! and clean-room brush dynamics.
//!
//! Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
//! and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod bake;
pub mod bvh;
pub mod compose;
pub mod material;
pub mod materialx;
pub mod math;
pub mod mesh;
pub mod paint;
pub mod probrush;
pub mod raster;

pub use bake::{BakeMapKind, BakeSettings, SurfaceAttributes, bake_map};
pub use bvh::Bvh;
pub use compose::{BlendMode, ChannelSource, PbrLayer, PbrLayerStack};
pub use material::{MaterialPreset, PbrChannel, standard_presets};
pub use materialx::{MaterialXOptions, NormalConvention, ShaderModel, generate_materialx_document};
pub use math::{Bounds, Hit, Ray, Vec2, Vec3, Vec4};
pub use mesh::{Mesh, Submesh};
pub use paint::{BrushSettings, StrokePoint, ToolKind, apply_dab, paint_stroke, polyfill};
pub use probrush::{BrushDynamics, BrushTipStamp};
pub use raster::RasterImage;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_bvh_quad_intersection() {
        let quad = Mesh::create_quad(2.0);
        let bvh = Bvh::build(&quad);
        assert!(!bvh.is_empty());

        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, -1.0));
        let hit = bvh.intersect(&quad, &ray);
        assert!(hit.is_some());
        let h = hit.unwrap();
        assert!((h.distance - 5.0).abs() < 1e-4);
        assert!((h.point.z - 0.0).abs() < 1e-4);
    }

    #[test]
    fn test_bake_position_and_normal() {
        let quad = Mesh::create_quad(2.0);
        let bvh = Bvh::build(&quad);
        let settings = BakeSettings {
            resolution: 32,
            samples: 4,
            max_distance: 0.1,
            ao_distance: 0.1,
            seed: 1,
        };

        let pos_map = bake_map(&quad, &bvh, BakeMapKind::Position, &settings);
        assert_eq!(pos_map.width, 32);
        assert_eq!(pos_map.height, 32);

        let norm_map = bake_map(&quad, &bvh, BakeMapKind::WorldNormal, &settings);
        assert_eq!(norm_map.width, 32);
        // Center pixel should face +Z (normal [0, 0, 1] encoded as [0.5, 0.5, 1.0])
        let center_pix = norm_map.pixel(16, 16);
        assert!((center_pix[2] - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_paint_and_polyfill() {
        let mut img = RasterImage::new(64, 64);
        let uvs = [
            Vec2::new(0.1, 0.1),
            Vec2::new(0.9, 0.1),
            Vec2::new(0.5, 0.9),
        ];
        polyfill(&mut img, &uvs, [1.0, 0.0, 0.0, 1.0]);

        // Centroid should be red
        let centroid_x = (0.5 * 64.0) as u32;
        let centroid_y = (0.35 * 64.0) as u32;
        let p = img.pixel(centroid_x, centroid_y);
        assert!((p[0] - 1.0).abs() < 1e-3);
        assert_eq!(p[3], 1.0);

        // Brush dabbing
        apply_dab(&mut img, Vec2::new(0.5, 0.5), 10.0, 0.8, [0.0, 1.0, 0.0, 1.0], 1.0);
        let center_p = img.pixel(32, 32);
        assert!(center_p[1] > 0.5);
    }

    #[test]
    fn test_layer_stack_composition() {
        let mut stack = PbrLayerStack::new();

        let base = PbrLayer::new_fill("Base Gold", [1.0, 0.8, 0.2, 1.0], 0.2, 1.0);
        let mut dirt = PbrLayer::new_fill("Dirt", [0.2, 0.1, 0.05, 1.0], 0.9, 0.0);
        dirt.opacity = 0.5;
        dirt.blend_mode = BlendMode::Normal;

        stack.add_layer(base);
        stack.add_layer(dirt);

        let result = stack.composite(32, 32);
        assert!(result.contains_key(&PbrChannel::BaseColor));
        assert!(result.contains_key(&PbrChannel::Roughness));
        assert!(result.contains_key(&PbrChannel::Metallic));

        let color_img = result.get(&PbrChannel::BaseColor).unwrap();
        let p = color_img.pixel(16, 16);
        // Blended color should be between gold and dirt
        assert!(p[0] < 1.0 && p[0] > 0.2);
    }

    #[test]
    fn test_materialx_export() {
        let mut maps = BTreeMap::new();
        maps.insert(PbrChannel::BaseColor, "textures/gold_basecolor.png".to_string());
        maps.insert(PbrChannel::Roughness, "textures/gold_roughness.png".to_string());
        maps.insert(PbrChannel::Metallic, "textures/gold_metallic.png".to_string());
        maps.insert(PbrChannel::Normal, "textures/gold_normal.png".to_string());

        let options = MaterialXOptions::default();
        let xml = generate_materialx_document("Polished_Gold", &maps, &options);

        assert!(xml.contains("<materialx version=\"1.39\""));
        assert!(xml.contains("open_pbr_surface"));
        assert!(xml.contains("gold_basecolor.png"));
        assert!(xml.contains("surfacematerial"));
    }

    #[test]
    fn test_probrush_dynamics_and_stamp() {
        let stamp = BrushTipStamp::procedural_round(32, 0.5);
        assert_eq!(stamp.width, 32);
        assert_eq!(stamp.height, 32);
        // Center of stamp should be fully opaque (255)
        let center_val = stamp.mask[(16 * 32 + 16) as usize];
        assert_eq!(center_val, 255);
        // Outer corner should be 0
        assert_eq!(stamp.mask[0], 0);
    }
}
