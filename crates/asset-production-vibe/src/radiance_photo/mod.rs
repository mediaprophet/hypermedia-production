//! Photographic RAW/studio development pipeline and Adobe Photoshop (.psd) codec morphisms.

mod develop;
mod psd_codec;

use crate::*;
use vibe::Value;

impl AssetHost<'_> {
    pub(crate) fn radiance_photo_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.radiance.develop_photo" | "radiance.develop_photo" => self.develop_photo(a),
            "hypermedia.radiance.encode_psd" | "radiance.encode_psd" => self.encode_psd(a),
            "hypermedia.radiance.import_psd" | "radiance.import_psd" => self.import_psd(a),
            _ => Err(format!("Unknown radiance photo operation: {id}")),
        }
    }
}

pub(super) fn parse_dimensions(a: &Args) -> Result<(u32, u32, usize), String> {
    let profile = crate::graphics::lattice_profile(a)?;
    let w = count(a, "width", 256, profile.max_dim as usize)? as u32;
    let h = count(a, "height", 256, profile.max_dim as usize)? as u32;
    Ok((w, h, w as usize * h as usize))
}

fn store_u16be_as_u8(src: &[u8], dst: &mut [u8]) -> Result<(), String> {
    if src.len() != dst.len() * 2 {
        return Err("16-bit channel length mismatch".into());
    }
    for (i, out) in dst.iter_mut().enumerate() {
        let v = u16::from_be_bytes([src[i * 2], src[i * 2 + 1]]);
        *out = crate::graphics::u16_to_u8_lattice(v);
    }
    Ok(())
}

pub(super) fn decode_psd_channel(
    src: &[u8],
    w: usize,
    h: usize,
    dst: &mut [u8],
    depth: u16,
) -> Result<(), String> {
    if src.len() < 2 {
        return Err("Truncated channel data".into());
    }
    let bpp = if depth == 16 { 2 } else { 1 };
    let n = w * h;
    let comp = u16::from_be_bytes(src[0..2].try_into().unwrap());
    match comp {
        0 => {
            if src.len() < 2 + n * bpp {
                return Err("Truncated raw channel data".into());
            }
            if bpp == 1 {
                dst.copy_from_slice(&src[2..2 + n]);
            } else {
                store_u16be_as_u8(&src[2..2 + n * 2], dst)?;
            }
            Ok(())
        }
        1 => {
            let table_len = h * 2;
            if src.len() < 2 + table_len {
                return Err("Truncated RLE channel row table".into());
            }
            let table = &src[2..2 + table_len];
            let mut data_pos = 2 + table_len;
            let row_bytes = w * bpp;
            let mut row_buf = vec![0u8; row_bytes];

            for row in 0..h {
                let t_idx = row * 2;
                let row_len =
                    u16::from_be_bytes(table[t_idx..t_idx + 2].try_into().unwrap()) as usize;
                if data_pos + row_len > src.len() {
                    return Err("Truncated RLE channel row".into());
                }
                decode_packbits(&src[data_pos..data_pos + row_len], &mut row_buf)?;
                if bpp == 1 {
                    dst[row * w..(row + 1) * w].copy_from_slice(&row_buf);
                } else {
                    store_u16be_as_u8(&row_buf, &mut dst[row * w..(row + 1) * w])?;
                }
                data_pos += row_len;
            }
            Ok(())
        }
        _ => Err(format!("Unsupported channel compression: {comp}")),
    }
}

pub(super) fn decode_packbits(src: &[u8], dst: &mut [u8]) -> Result<(), String> {
    let mut s_idx = 0;
    let mut d_idx = 0;
    while s_idx < src.len() && d_idx < dst.len() {
        let n = src[s_idx] as i8;
        s_idx += 1;
        if n >= 0 {
            let count = (n as usize) + 1;
            if s_idx + count > src.len() || d_idx + count > dst.len() {
                return Err("PackBits literal run exceeds bounds".into());
            }
            dst[d_idx..d_idx + count].copy_from_slice(&src[s_idx..s_idx + count]);
            s_idx += count;
            d_idx += count;
        } else if n != -128 {
            let count = (1 - n as isize) as usize;
            if s_idx >= src.len() || d_idx + count > dst.len() {
                return Err("PackBits repeat run exceeds bounds".into());
            }
            let val = src[s_idx];
            s_idx += 1;
            dst[d_idx..d_idx + count].fill(val);
            d_idx += count;
        }
    }
    if d_idx != dst.len() {
        return Err("PackBits decoded length mismatch".into());
    }
    Ok(())
}

#[inline]
pub(super) fn srgb_to_linear_f32(c: u8) -> f32 {
    let v = c as f32 / 255.0;
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

#[inline]
pub(super) fn linear_to_srgb_f32(v: f32) -> f32 {
    if v <= 0.0031308 {
        v * 12.92
    } else {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::atomic::AtomicBool;

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
    fn test_packbits_decode() {
        let mut dst = [0u8; 8];
        // 2 literal bytes: [0x01, 10, 20], repeat 3 times: [-2, 99], repeat 3 times: [-2, 77]
        let src = [1, 10, 20, -2i8 as u8, 99, -2i8 as u8, 77];
        decode_packbits(&src, &mut dst).expect("decode packbits");
        assert_eq!(dst, [10, 20, 99, 99, 99, 77, 77, 77]);
    }

    #[test]
    fn test_srgb_roundtrip() {
        assert_eq!(srgb_to_linear_f32(0), 0.0);
        assert!((srgb_to_linear_f32(255) - 1.0).abs() < 1e-5);
        for i in 0..=255u8 {
            let lin = srgb_to_linear_f32(i);
            let srgb = (linear_to_srgb_f32(lin) * 255.0 + 0.5).clamp(0.0, 255.0) as u8;
            assert_eq!(i, srgb);
        }
    }

    #[test]
    fn test_psd_encode_import_roundtrip() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        // 2x2 layer 0: solid red, opaque
        let l0 = host
            .store_rgba(2, 2, vec![[255, 0, 0, 255]; 4])
            .expect("store layer 0");
        // 2x2 layer 1: solid blue, 50% opacity
        let l1 = host
            .store_rgba(2, 2, vec![[0, 0, 255, 128]; 4])
            .expect("store layer 1");

        let mut args = BTreeMap::new();
        args.insert("width".into(), Value::U64(2));
        args.insert("height".into(), Value::U64(2));

        let layer0_rec = Value::Record(BTreeMap::from([
            ("name".into(), Value::String("Background".into())),
            ("image".into(), l0.clone()),
            ("opacity".into(), Value::F64(1.0)),
        ]));
        let layer1_rec = Value::Record(BTreeMap::from([
            ("name".into(), Value::String("BlueOverlay".into())),
            ("image".into(), l1.clone()),
            ("opacity".into(), Value::F64(1.0)),
        ]));

        args.insert(
            "layers".into(),
            Value::List(vec![layer0_rec, layer1_rec]),
        );

        let psd_res = host
            .radiance_photo_op("hypermedia.radiance.encode_psd", &args)
            .expect("encode psd");

        // Validate roundtrip: import merged composite
        let import_args = BTreeMap::from([("input".into(), psd_res.clone())]);
        let comp_res = host
            .radiance_photo_op("hypermedia.radiance.import_psd", &import_args)
            .expect("import composite");
        let comp_rgba = host.rgba(&comp_res).expect("rgba handle");
        assert_eq!(comp_rgba.width, 2);
        assert_eq!(comp_rgba.height, 2);
        // Composite has red background with 50% blue over: R should be ~128, B should be ~128, A = 255
        assert_eq!(comp_rgba.pixels.len(), 4);
        assert!(comp_rgba.pixels[0][0] > 120 && comp_rgba.pixels[0][0] < 135);
        assert_eq!(comp_rgba.pixels[0][1], 0);
        assert!(comp_rgba.pixels[0][2] > 120 && comp_rgba.pixels[0][2] < 135);
        assert_eq!(comp_rgba.pixels[0][3], 255);

        // Validate roundtrip: import layer 0
        let l0_args = BTreeMap::from([
            ("input".into(), psd_res.clone()),
            ("layer_index".into(), Value::U64(0)),
        ]);
        let imp_l0 = host
            .radiance_photo_op("hypermedia.radiance.import_psd", &l0_args)
            .expect("import layer 0");
        let imp_l0_rgba = host.rgba(&imp_l0).expect("rgba handle");
        assert_eq!(imp_l0_rgba.pixels, vec![[255, 0, 0, 255]; 4]);

        // Validate roundtrip: import layer 1
        let l1_args = BTreeMap::from([
            ("input".into(), psd_res),
            ("layer_index".into(), Value::U64(1)),
        ]);
        let imp_l1 = host
            .radiance_photo_op("hypermedia.radiance.import_psd", &l1_args)
            .expect("import layer 1");
        let imp_l1_rgba = host.rgba(&imp_l1).expect("rgba handle");
        assert_eq!(imp_l1_rgba.pixels, vec![[0, 0, 255, 128]; 4]);
    }

    #[test]
    fn test_develop_photo_default_identity() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        let orig_pixels = vec![
            [120, 80, 40, 200],
            [240, 200, 160, 255],
            [10, 20, 30, 100],
            [255, 255, 255, 0],
        ];
        let img = host
            .store_rgba(2, 2, orig_pixels.clone())
            .expect("store image");

        let args = BTreeMap::from([("image".into(), img)]);
        let dev = host
            .radiance_photo_op("hypermedia.radiance.develop_photo", &args)
            .expect("develop photo");
        let dev_rgba = host.rgba(&dev).expect("rgba handle");
        assert_eq!(dev_rgba.pixels, orig_pixels);
    }

    #[test]
    fn test_develop_photo_adjustments() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = create_test_host(&limits, &cancel, &[]);

        let orig_pixels = vec![[100, 100, 100, 175]; 9];
        let img = host.store_rgba(3, 3, orig_pixels).expect("store image");

        let args = BTreeMap::from([
            ("image".into(), img),
            ("exposure".into(), Value::F64(1.0)),
            ("temperature".into(), Value::F64(0.5)),
            ("tint".into(), Value::F64(-0.2)),
            ("highlights".into(), Value::F64(0.2)),
            ("shadows".into(), Value::F64(-0.2)),
            ("clarity".into(), Value::F64(0.3)),
            ("vibrance".into(), Value::F64(0.4)),
            ("denoise".into(), Value::F64(0.2)),
        ]);
        let dev = host
            .radiance_photo_op("hypermedia.radiance.develop_photo", &args)
            .expect("develop photo");
        let dev_rgba = host.rgba(&dev).expect("rgba handle");

        // Straight alpha 175 is preserved on all pixels
        for p in &dev_rgba.pixels {
            assert_eq!(p[3], 175);
            // With exposure +1.0 and warm temperature, red channel should be significantly boosted
            assert!(p[0] > 100);
        }
    }
}
