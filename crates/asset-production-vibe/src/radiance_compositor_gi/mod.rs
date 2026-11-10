//! Radiance compositor and global illumination operations:
//! 1. Anisotropic Kuwahara structure-tensor edge-preserving filter
//! 2. Cryptomatte Q42 ID coverage matte extraction and isolated compositing
//! 3. Vector motion blur line-integral convolution with Hann shutter window
//! 4. Cascaded SDFGI irradiance probes and 3D froxel atmospheric scattering
//! 5. Hierarchical light tree bounding-cone clustering and direct lighting preview

mod filters_cryptomatte;
mod gpu_gi;
mod sdfgi_light_tree;

use crate::*;
use vibe::Value;

// Deterministic 32-bit MurmurHash3 for Cryptomatte manifest hashing.
pub(super) fn cryptomatte_murmur3(data: &[u8]) -> u32 {
    let mut h = 0u32;
    let nblocks = data.len() / 4;
    for i in 0..nblocks {
        let chunk = &data[i * 4..i * 4 + 4];
        let mut k = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        k = k.wrapping_mul(0xcc9e2d51);
        k = k.rotate_left(15);
        k = k.wrapping_mul(0x1b873593);

        h ^= k;
        h = h.rotate_left(13);
        h = h.wrapping_mul(5).wrapping_add(0xe6546b64);
    }
    let tail = &data[nblocks * 4..];
    let mut k1 = 0u32;
    if tail.len() == 3 {
        k1 ^= (tail[2] as u32) << 16;
    }
    if tail.len() >= 2 {
        k1 ^= (tail[1] as u32) << 8;
    }
    if !tail.is_empty() {
        k1 ^= tail[0] as u32;
        k1 = k1.wrapping_mul(0xcc9e2d51);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(0x1b873593);
        h ^= k1;
    }
    h ^= data.len() as u32;
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;
    h
}

#[inline]
pub(super) fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    if (edge1 - edge0).abs() < 1e-9 {
        if x < edge0 {
            0.0
        } else {
            1.0
        }
    } else {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
}

impl AssetHost<'_> {
    pub(crate) fn radiance_compositor_gi_op(
        &mut self,
        id: &str,
        a: &Args,
    ) -> Result<Value, String> {
        match id {
            "hypermedia.radiance.filter_kuwahara_anisotropic"
            | "radiance.filter_kuwahara_anisotropic"
            | "synth.radiance.filter_kuwahara_anisotropic" => {
                self.radiance_filter_kuwahara_anisotropic(a)
            }
            "hypermedia.radiance.decode_cryptomatte_q42"
            | "radiance.decode_cryptomatte_q42"
            | "synth.radiance.decode_cryptomatte_q42" => self.radiance_decode_cryptomatte_q42(a),
            "hypermedia.radiance.vector_motion_blur"
            | "radiance.vector_motion_blur"
            | "synth.radiance.vector_motion_blur" => self.radiance_vector_motion_blur(a),
            "hypermedia.radiance.solve_sdfgi_froxel"
            | "radiance.solve_sdfgi_froxel"
            | "synth.radiance.solve_sdfgi_froxel" => self.radiance_solve_sdfgi_froxel(a),
            "hypermedia.radiance.build_light_tree"
            | "radiance.build_light_tree"
            | "synth.radiance.build_light_tree" => self.radiance_build_light_tree(a),
            _ => Err(format!("Unknown radiance compositor/GI operation: {id}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::BTreeMap;
    use std::sync::atomic::AtomicBool;
    use vibe::Value;

    fn create_test_host<'a>(
        limits: &'a ToolLimits,
        cancel: &'a AtomicBool,
        inputs: &'a [Resource],
    ) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs,
            limits,
            cancel,
            work: 0,
        }
    }

    #[test]
    fn test_filter_kuwahara_anisotropic() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        // 8x8 test pattern with vertical edge
        let mut pixels = Vec::with_capacity(64);
        for _y in 0..8 {
            for x in 0..8 {
                if x < 4 {
                    pixels.push([220, 20, 20, 255]); // red
                } else {
                    pixels.push([20, 20, 220, 255]); // blue
                }
            }
        }
        let img = host.store_rgba(8, 8, pixels).expect("store image");

        let args = BTreeMap::from([
            ("image".into(), img.clone()),
            ("radius".into(), Value::U64(2)),
            ("sharpness".into(), Value::F64(8.0)),
            ("eccentricity".into(), Value::F64(1.0)),
        ]);

        let res = host
            .radiance_compositor_gi_op("hypermedia.radiance.filter_kuwahara_anisotropic", &args)
            .expect("anisotropic kuwahara");
        let filtered = host.rgba(&res).expect("retrieve rgba");
        assert_eq!(filtered.width, 8);
        assert_eq!(filtered.height, 8);
        assert_eq!(filtered.pixels.len(), 64);

        // Also test aliases
        let res2 = host
            .radiance_compositor_gi_op("radiance.filter_kuwahara_anisotropic", &args)
            .expect("alias 1");
        assert!(host.rgba(&res2).is_ok());

        let res3 = host
            .radiance_compositor_gi_op("synth.radiance.filter_kuwahara_anisotropic", &args)
            .expect("alias 2");
        assert!(host.rgba(&res3).is_ok());
    }

    #[test]
    fn test_decode_cryptomatte_q42() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        // 4x4 image with a dark corner, bright center, and medium surround
        let mut pixels = vec![[50, 50, 50, 255]; 16];
        pixels[5] = [200, 200, 200, 255];
        pixels[6] = [220, 220, 220, 255];
        pixels[9] = [210, 210, 210, 255];
        pixels[10] = [230, 230, 230, 255];
        let img = host.store_rgba(4, 4, pixels).expect("store image");

        let args = BTreeMap::from([
            ("image".into(), img),
            (
                "target_id".into(),
                Value::String("did:q42:entity:hero".into()),
            ),
            ("luma_min".into(), Value::F64(0.2)),
            ("luma_max".into(), Value::F64(0.9)),
            ("feather".into(), Value::F64(0.05)),
        ]);

        let res = host
            .radiance_compositor_gi_op("hypermedia.radiance.decode_cryptomatte_q42", &args)
            .expect("decode cryptomatte");
        let rec = record(&res).expect("record");

        let target = text(required(rec, "target_id").expect("target_id")).expect("string");
        assert_eq!(target, "did:q42:entity:hero");

        let hash_hex = text(required(rec, "manifest_hash_hex").expect("hash_hex")).expect("string");
        assert_eq!(hash_hex.len(), 8);

        let cov_ratio = num(required(rec, "coverage_ratio").expect("cov")).expect("num");
        assert!(cov_ratio > 0.0 && cov_ratio <= 1.0);

        let matte_val = required(rec, "matte_rgba").expect("matte_rgba");
        let matte_img = host.rgba(matte_val).expect("matte rgba");
        assert_eq!(matte_img.width, 4);
        assert_eq!(matte_img.height, 4);

        let isolated_val = required(rec, "isolated_rgba").expect("isolated_rgba");
        let isolated_img = host.rgba(isolated_val).expect("isolated rgba");
        assert_eq!(isolated_img.width, 4);
        assert_eq!(isolated_img.height, 4);
    }

    #[test]
    fn test_vector_motion_blur() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        // 8x8 image with white dot at center
        let mut pixels = vec![[0, 0, 0, 255]; 64];
        pixels[4 * 8 + 4] = [255, 255, 255, 255];
        let img = host.store_rgba(8, 8, pixels).expect("store image");

        let args = BTreeMap::from([
            ("image".into(), img),
            (
                "velocity_px".into(),
                Value::List(vec![Value::F64(4.0), Value::F64(0.0)]),
            ),
            ("samples".into(), Value::U64(7)),
            ("shutter".into(), Value::F64(0.5)),
        ]);

        let res = host
            .radiance_compositor_gi_op("hypermedia.radiance.vector_motion_blur", &args)
            .expect("vector motion blur");
        let blurred = host.rgba(&res).expect("blurred rgba");
        assert_eq!(blurred.width, 8);
        assert_eq!(blurred.height, 8);

        // Dot should be horizontally blurred: pixel to the right should have non-zero value
        let center_y = 4;
        let right_pixel = blurred.pixels[center_y * 8 + 5];
        assert!(right_pixel[0] > 0);
    }

    #[test]
    fn test_solve_sdfgi_froxel() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        let pixels = vec![[100, 100, 100, 255]; 16];
        let img = host.store_rgba(4, 4, pixels).expect("store image");

        let args = BTreeMap::from([
            ("image".into(), img),
            ("cascades".into(), Value::U64(3)),
            ("fog_density".into(), Value::F64(0.25)),
            ("scattering_anisotropy_g".into(), Value::F64(0.5)),
            (
                "sun_direction".into(),
                Value::List(vec![Value::F64(0.0), Value::F64(1.0), Value::F64(0.0)]),
            ),
            (
                "sky_radiance".into(),
                Value::List(vec![Value::F64(0.4), Value::F64(0.6), Value::F64(0.9)]),
            ),
            ("bounce_intensity".into(), Value::F64(0.5)),
        ]);

        let res = host
            .radiance_compositor_gi_op("hypermedia.radiance.solve_sdfgi_froxel", &args)
            .expect("solve sdfgi froxel");
        let rec = record(&res).expect("record");

        let cascades = num(required(rec, "cascades").expect("cascades")).expect("num");
        assert_eq!(cascades, 3.0);

        let mean_irr =
            num(required(rec, "mean_irradiance").expect("mean_irradiance")).expect("num");
        assert!(mean_irr > 0.0);

        let trans = num(required(rec, "transmittance").expect("transmittance")).expect("num");
        assert!(trans > 0.0 && trans <= 1.0);

        let img_val = required(rec, "image").expect("image");
        let lit_img = host.rgba(img_val).expect("lit rgba");
        assert_eq!(lit_img.width, 4);
        assert_eq!(lit_img.height, 4);

        let backend = text(required(rec, "backend").expect("backend")).expect("string");
        assert_eq!(backend, "cpu");
    }

    #[test]
    fn test_solve_sdfgi_froxel_backend_cpu() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        let pixels = vec![[80, 90, 100, 255]; 16];
        let img = host.store_rgba(4, 4, pixels).expect("store image");
        let args = BTreeMap::from([
            ("image".into(), img),
            ("cascades".into(), Value::U64(2)),
            ("backend".into(), Value::String("cpu".into())),
        ]);

        let res = host
            .radiance_compositor_gi_op("hypermedia.radiance.solve_sdfgi_froxel", &args)
            .expect("cpu backend");
        let rec = record(&res).expect("record");
        let backend = text(required(rec, "backend").expect("backend")).expect("string");
        assert_eq!(backend, "cpu");
        let img_val = required(rec, "image").expect("image");
        let lit_img = host.rgba(img_val).expect("lit rgba");
        assert_eq!(lit_img.width, 4);
        assert_eq!(lit_img.height, 4);
        assert_eq!(lit_img.pixels.len(), 16);
    }

    #[test]
    fn test_solve_sdfgi_froxel_backend_gpu() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        let pixels = vec![[120, 110, 90, 255]; 16];
        let img = host.store_rgba(4, 4, pixels).expect("store image");
        let args = BTreeMap::from([
            ("image".into(), img),
            ("cascades".into(), Value::U64(2)),
            ("fog_density".into(), Value::F64(0.2)),
            ("scattering_anisotropy_g".into(), Value::F64(0.4)),
            ("backend".into(), Value::String("gpu".into())),
        ]);

        match host.radiance_compositor_gi_op("hypermedia.radiance.solve_sdfgi_froxel", &args) {
            Ok(res) => {
                let rec = record(&res).expect("record");
                let backend = text(required(rec, "backend").expect("backend")).expect("string");
                assert_eq!(
                    backend, "gpu",
                    "caller asked gpu; must not label a cpu receipt as gpu or return cpu"
                );
                let img_val = required(rec, "image").expect("image");
                let lit_img = host.rgba(img_val).expect("lit rgba");
                assert_eq!(lit_img.width, 4);
                assert_eq!(lit_img.height, 4);
                let adapter = text(required(rec, "adapter").expect("adapter")).expect("string");
                let mean_irr =
                    num(required(rec, "mean_irradiance").expect("mean_irradiance")).expect("num");
                assert!(mean_irr.is_finite());
                let cascades = num(required(rec, "cascades").expect("cascades")).expect("num");
                assert_eq!(cascades, 2.0);
                eprintln!(
                    "gpu sdfgi dispatched compute passes adapter={adapter} mean_irradiance={mean_irr}"
                );
            }
            Err(e) => {
                assert!(
                    e.contains("gpu backend requires"),
                    "fail-closed GPU error must contain 'gpu backend requires', got: {e}"
                );
                eprintln!("gpu sdfgi fail-closed: {e}");
            }
        }
    }

    #[test]
    fn test_solve_sdfgi_froxel_unknown_backend() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        let pixels = vec![[10, 10, 10, 255]; 4];
        let img = host.store_rgba(2, 2, pixels).expect("store image");
        let args = BTreeMap::from([
            ("image".into(), img),
            ("backend".into(), Value::String("metal".into())),
        ]);
        let err = host
            .radiance_compositor_gi_op("hypermedia.radiance.solve_sdfgi_froxel", &args)
            .expect_err("unknown backend");
        assert!(
            err.contains("Unknown backend"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn test_solve_sdfgi_froxel_unknown_arg_fail_closed() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        let pixels = vec![[10, 10, 10, 255]; 4];
        let img = host.store_rgba(2, 2, pixels).expect("store image");
        let args = BTreeMap::from([
            ("image".into(), img),
            ("not_a_field".into(), Value::U64(1)),
        ]);
        let err = host
            .radiance_compositor_gi_op("hypermedia.radiance.solve_sdfgi_froxel", &args)
            .expect_err("unknown arg");
        assert!(
            err.contains("Unknown argument"),
            "fields() must fail-closed, got: {err}"
        );
    }

    #[test]
    fn test_build_light_tree() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        let light1 = Value::Record(BTreeMap::from([
            (
                "position".into(),
                Value::List(vec![Value::F64(0.0), Value::F64(5.0), Value::F64(0.0)]),
            ),
            (
                "direction".into(),
                Value::List(vec![Value::F64(0.0), Value::F64(-1.0), Value::F64(0.0)]),
            ),
            (
                "color".into(),
                Value::List(vec![Value::F64(1.0), Value::F64(0.9), Value::F64(0.8)]),
            ),
            ("power".into(), Value::F64(10.0)),
            ("angle_rad".into(), Value::F64(std::f64::consts::FRAC_PI_3)),
        ]));

        let light2 = Value::Record(BTreeMap::from([
            (
                "position".into(),
                Value::List(vec![Value::F64(3.0), Value::F64(4.0), Value::F64(2.0)]),
            ),
            (
                "direction".into(),
                Value::List(vec![Value::F64(-0.5), Value::F64(-0.8), Value::F64(0.0)]),
            ),
            (
                "color".into(),
                Value::List(vec![Value::F64(0.2), Value::F64(0.5), Value::F64(1.0)]),
            ),
            ("power".into(), Value::F64(5.0)),
            ("angle_rad".into(), Value::F64(std::f64::consts::FRAC_PI_2)),
        ]));

        let args = BTreeMap::from([
            ("lights".into(), Value::List(vec![light1, light2])),
            ("width".into(), Value::U64(64)),
            ("height".into(), Value::U64(64)),
        ]);

        let res = host
            .radiance_compositor_gi_op("hypermedia.radiance.build_light_tree", &args)
            .expect("build light tree");
        let rec = record(&res).expect("record");

        let leaf_count = num(required(rec, "leaf_count").expect("leaf_count")).expect("num");
        assert_eq!(leaf_count, 2.0);

        let node_count = num(required(rec, "node_count").expect("node_count")).expect("num");
        assert_eq!(node_count, 3.0); // 2 * 2 - 1 = 3

        let total_power = num(required(rec, "total_power").expect("total_power")).expect("num");
        assert!((total_power - 15.0).abs() < 1e-5);

        let preview_val = required(rec, "preview_rgba").expect("preview_rgba");
        let preview_img = host.rgba(preview_val).expect("preview rgba");
        assert_eq!(preview_img.width, 64);
        assert_eq!(preview_img.height, 64);
    }

    #[test]
    fn test_error_handling() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        let empty_args = BTreeMap::new();
        assert!(host
            .radiance_compositor_gi_op("hypermedia.radiance.vector_motion_blur", &empty_args)
            .is_err());

        assert!(host
            .radiance_compositor_gi_op("hypermedia.radiance.unknown_op", &empty_args)
            .is_err());
    }
}
