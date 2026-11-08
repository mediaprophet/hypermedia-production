//! ICC v4.3 binary profile serializer.

use crate::icc::curve::Curve;
use crate::icc::math::Mat3;
use crate::icc::profile::{ColorSpace, Pcs, Profile, ProfileClass};

fn s15(v: f64) -> [u8; 4] {
    ((v * 65536.0).round() as i32).to_be_bytes()
}

fn xyz_type(v: [f64; 3]) -> Vec<u8> {
    let mut o = b"XYZ \0\0\0\0".to_vec();
    for c in v {
        o.extend_from_slice(&s15(c));
    }
    o
}

fn mluc(text: &str) -> Vec<u8> {
    let units: Vec<u16> = text.encode_utf16().collect();
    let mut o = b"mluc\0\0\0\0".to_vec();
    o.extend_from_slice(&1u32.to_be_bytes());
    o.extend_from_slice(&12u32.to_be_bytes());
    o.extend_from_slice(b"enUS");
    o.extend_from_slice(&((units.len() * 2) as u32).to_be_bytes());
    o.extend_from_slice(&28u32.to_be_bytes());
    for u in units {
        o.extend_from_slice(&u.to_be_bytes());
    }
    o
}

fn curve_elem(c: &Curve) -> Vec<u8> {
    match c {
        Curve::Identity => {
            let mut o = b"curv\0\0\0\0".to_vec();
            o.extend_from_slice(&0u32.to_be_bytes());
            o
        }
        Curve::Gamma(g) => {
            let mut o = b"curv\0\0\0\0".to_vec();
            o.extend_from_slice(&1u32.to_be_bytes());
            let q = (g * 256.0).round().clamp(0.0, 65535.0) as u16;
            o.extend_from_slice(&q.to_be_bytes());
            o
        }
        Curve::Parametric { kind, p } => {
            let mut o = b"para\0\0\0\0".to_vec();
            o.extend_from_slice(&kind.to_be_bytes());
            o.extend_from_slice(&[0, 0]);
            for v in p.iter().take(Curve::param_count(*kind)) {
                o.extend_from_slice(&s15(*v));
            }
            o
        }
        Curve::Table(t) => {
            let mut o = b"curv\0\0\0\0".to_vec();
            o.extend_from_slice(&(t.len() as u32).to_be_bytes());
            for v in t {
                let q = (v.clamp(0.0, 1.0) * 65535.0 + 0.5) as u16;
                o.extend_from_slice(&q.to_be_bytes());
            }
            o
        }
    }
}

pub fn write_profile(p: &Profile) -> Vec<u8> {
    let mut tags: Vec<([u8; 4], Vec<u8>)> = Vec::new();

    tags.push((*b"desc", mluc(&p.description)));
    tags.push((*b"wtpt", xyz_type(p.white_point)));
    tags.push((*b"cprt", mluc("No copyright: CC0-1.0 public domain dedication.")));

    if let Some(m) = &p.matrix {
        tags.push((*b"rXYZ", xyz_type([m[0][0], m[1][0], m[2][0]])));
        tags.push((*b"gXYZ", xyz_type([m[0][1], m[1][1], m[2][1]])));
        tags.push((*b"bXYZ", xyz_type([m[0][2], m[1][2], m[2][2]])));
    }
    if let Some(trc) = &p.trc {
        tags.push((*b"rTRC", curve_elem(&trc[0])));
        tags.push((*b"gTRC", curve_elem(&trc[1])));
        tags.push((*b"bTRC", curve_elem(&trc[2])));
    }
    if let Some(k) = &p.gray_trc {
        tags.push((*b"kTRC", curve_elem(k)));
    }

    let header_len = 128;
    let tag_table_len = 4 + tags.len() * 12;
    let mut data_offset = header_len + tag_table_len;
    data_offset = (data_offset + 3) & !3;

    let mut out = vec![0u8; header_len];
    // Version 4.3.0.0
    out[8] = p.version.0;
    out[9] = p.version.1;
    // Class
    out[12..16].copy_from_slice(&p.class.sig().to_be_bytes());
    // Color space
    out[16..20].copy_from_slice(&p.color_space.sig().to_be_bytes());
    // PCS
    out[20..24].copy_from_slice(match p.pcs {
        Pcs::Xyz => b"XYZ ",
        Pcs::Lab => b"Lab ",
    });
    // Signature
    out[36..40].copy_from_slice(b"acsp");
    // Illuminant
    out[68..72].copy_from_slice(&s15(p.white_point[0]));
    out[72..76].copy_from_slice(&s15(p.white_point[1]));
    out[76..80].copy_from_slice(&s15(p.white_point[2]));

    // Tag table
    out.extend_from_slice(&(tags.len() as u32).to_be_bytes());
    let mut cur_offset = data_offset;
    for (sig, data) in &tags {
        out.extend_from_slice(sig);
        out.extend_from_slice(&(cur_offset as u32).to_be_bytes());
        out.extend_from_slice(&(data.len() as u32).to_be_bytes());
        cur_offset += (data.len() + 3) & !3;
    }

    // Pad to data_offset
    while out.len() < data_offset {
        out.push(0);
    }

    // Write tag data
    for (_, data) in &tags {
        out.extend_from_slice(data);
        while out.len() % 4 != 0 {
            out.push(0);
        }
    }

    let total_len = out.len() as u32;
    out[0..4].copy_from_slice(&total_len.to_be_bytes());
    out
}
