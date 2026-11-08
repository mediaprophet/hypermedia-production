//! Compression-ID parameters (Annex C), zig-zag order (Figure 48) and VLC lookup tables built from
//! the Annex E codeword lists in [`crate::dnx::spec_tables`].

use crate::dnx::spec_tables::*;
use std::sync::OnceLock;

/// Bitstream coefficient index `r` -> raster position `v*8 + u` (Figure 48).
pub const ZIGZAG: [u8; 64] = {
    const GRID: [[u8; 8]; 8] = [
        [0, 1, 5, 6, 14, 15, 27, 28],
        [2, 4, 7, 13, 16, 26, 29, 42],
        [3, 8, 12, 17, 25, 30, 41, 43],
        [9, 11, 18, 24, 31, 40, 44, 53],
        [10, 19, 23, 32, 39, 45, 52, 54],
        [20, 22, 33, 38, 46, 51, 55, 60],
        [21, 34, 37, 47, 50, 56, 59, 61],
        [35, 36, 48, 49, 57, 58, 62, 63],
    ];
    let mut z = [0u8; 64];
    let mut v = 0;
    while v < 8 {
        let mut u = 0;
        while u < 8 {
            z[GRID[v][u] as usize] = (v * 8 + u) as u8;
            u += 1;
        }
        v += 1;
    }
    z
};

/// Raster profile of a compression ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Hd {
        width: u16,
        height: u16,
        interlaced: bool,
        depth: u8,
        frame_size: u32,
    },
    Ri {
        c0: u32,
    },
}

/// Parameters of one compression ID.
#[derive(Debug, Clone, Copy)]
pub struct CidInfo {
    pub cid: u32,
    pub kind: Kind,
    pub weights: &'static [[u8; 63]; 2],
    pub vlc: usize,
    pub p: u32,
}

const fn hd(
    cid: u32,
    width: u16,
    height: u16,
    interlaced: bool,
    depth: u8,
    frame_size: u32,
    weights: &'static [[u8; 63]; 2],
    vlc: usize,
) -> CidInfo {
    let p = if cid == 1235 || cid == 1241 || cid == 1250 {
        8
    } else {
        32
    };
    CidInfo {
        cid,
        kind: Kind::Hd {
            width,
            height,
            interlaced,
            depth,
            frame_size,
        },
        weights,
        vlc,
        p,
    }
}

const fn ri(cid: u32, c0: u32, weights: &'static [[u8; 63]; 2], vlc: usize) -> CidInfo {
    CidInfo {
        cid,
        kind: Kind::Ri { c0 },
        weights,
        vlc,
        p: 32,
    }
}

pub const CIDS: [CidInfo; 20] = [
    hd(1235, 1920, 1080, false, 10, 917504, &W_D_1, 0),
    hd(1237, 1920, 1080, false, 8, 606208, &W_D_2, 1),
    hd(1238, 1920, 1080, false, 8, 917504, &W_D_3, 2),
    hd(1241, 1920, 1080, true, 10, 917504, &W_D_4, 0),
    hd(1242, 1920, 1080, true, 8, 606208, &W_D_5, 1),
    hd(1243, 1920, 1080, true, 8, 917504, &W_D_6, 2),
    hd(1244, 1440, 1080, true, 8, 606208, &W_D_7, 1),
    hd(1250, 1280, 720, false, 10, 458752, &W_D_8, 3),
    hd(1251, 1280, 720, false, 8, 458752, &W_D_9, 4),
    hd(1252, 1280, 720, false, 8, 303104, &W_D_10, 5),
    hd(1253, 1920, 1080, false, 8, 188416, &W_D_2, 1),
    hd(1256, 1920, 1080, false, 10, 1835008, &W_D_11, 0),
    hd(1258, 960, 720, false, 8, 212992, &W_D_10, 5),
    hd(1259, 1440, 1080, false, 8, 417792, &W_D_2, 1),
    hd(1260, 1440, 1080, true, 8, 417792, &W_D_7, 1),
    ri(1270, 1835008, &W_D_11, 0),
    ri(1271, 917504, &W_D_4, 0),
    ri(1272, 917504, &W_D_3, 2),
    ri(1273, 606208, &W_D_2, 1),
    ri(1274, 188416, &W_D_2, 1),
];

pub fn cid_info(cid: u32) -> Option<&'static CidInfo> {
    CIDS.iter().find(|c| c.cid == cid)
}

pub fn ri_frame_size(width: u32, height: u32, c0: u32, alpha: bool) -> u32 {
    let nw = width.div_ceil(16) as u64;
    let ns = height.div_ceil(16) as u64;
    let mut size = c0 as u64 * nw * ns / 8160;
    if alpha {
        size += size / 2;
    }
    let rem = size % 4096;
    if rem >= 2048 {
        size += 4096 - rem;
    } else {
        size -= rem;
    }
    size.max(8192) as u32
}

pub struct Lut {
    pub primary: Vec<u32>,
    pub secondary: Vec<u32>,
    pub bits: u32,
}

pub const LINK: u32 = 1 << 31;

impl Lut {
    fn build(codes: &[(u32, u32, u32)], bits: u32) -> Lut {
        let max_len = codes.iter().map(|c| c.1).max().unwrap_or(1);
        let bits = bits.min(max_len);
        let mut primary = vec![0u32; 1 << bits];
        let mut sub_len = vec![0u32; 1 << bits];
        for &(code, len, _) in codes {
            if len > bits {
                let pre = (code >> (len - bits)) as usize;
                sub_len[pre] = sub_len[pre].max(len - bits);
            }
        }
        let mut secondary = Vec::new();
        for pre in 0..1usize << bits {
            if sub_len[pre] > 0 {
                let off = secondary.len() as u32;
                secondary.resize(secondary.len() + (1 << sub_len[pre]), 0);
                primary[pre] = LINK | sub_len[pre] << 24 | off;
            }
        }
        for &(code, len, value) in codes {
            if len <= bits {
                let shift = bits - len;
                let base = (code << shift) as usize;
                for e in &mut primary[base..base + (1 << shift)] {
                    *e = len << 16 | value;
                }
            } else {
                let pre = (code >> (len - bits)) as usize;
                let link = primary[pre];
                let sb = (link >> 24) & 0x7f;
                let off = (link & 0xff_ffff) as usize;
                let rest_len = len - bits;
                let rest = code & ((1 << rest_len) - 1);
                let shift = sb - rest_len;
                let base = off + ((rest << shift) as usize);
                for e in &mut secondary[base..base + (1 << shift)] {
                    *e = len << 16 | value;
                }
            }
        }
        Lut {
            primary,
            secondary,
            bits,
        }
    }
}

pub const AMP_FRUN: u32 = 1 << 8;
pub const AMP_FINDEX: u32 = 1 << 9;
pub const AMP_EOB: u32 = 1 << 10;

pub struct VlcSet {
    pub amp: Lut,
    pub run: Lut,
    pub dc: Lut,
    pub amp_code: [[[(u16, u8); 64]; 2]; 2],
    pub eob: (u16, u8),
    pub run_code: [(u16, u8); 63],
    pub dc_code: Vec<(u16, u8)>,
}

fn build_set(
    amp: &[(u16, u8, u8, u8, u8)],
    run: &[(u16, u8, u8); 62],
    dc: &[(u16, u8)],
) -> VlcSet {
    let ac: Vec<(u32, u32, u32)> = amp
        .iter()
        .map(|&(c, l, a, fr, fi)| {
            let v = if a == 0 {
                AMP_EOB
            } else {
                a as u32 | (fr as u32) << 8 | (fi as u32) << 9
            };
            (c as u32, l as u32, v)
        })
        .collect();
    let rc: Vec<(u32, u32, u32)> = run
        .iter()
        .map(|&(c, l, r)| (c as u32, l as u32, r as u32))
        .collect();
    let dcc: Vec<(u32, u32, u32)> = dc
        .iter()
        .enumerate()
        .map(|(i, &(c, l))| (c as u32, l as u32, i as u32))
        .collect();
    let mut amp_code = [[[(0u16, 0u8); 64]; 2]; 2];
    let mut eob = (0, 0);
    for &(c, l, a, fr, fi) in amp {
        if a == 0 {
            eob = (c, l);
        } else {
            amp_code[fr as usize][fi as usize][a as usize - 1] = (c, l);
        }
    }
    let mut run_code = [(0u16, 0u8); 63];
    for &(c, l, r) in run {
        run_code[r as usize] = (c, l);
    }
    VlcSet {
        amp: Lut::build(&ac, 11),
        run: Lut::build(&rc, 10),
        dc: Lut::build(&dcc, 8),
        amp_code,
        eob,
        run_code,
        dc_code: dc.to_vec(),
    }
}

pub fn vlc(i: usize) -> &'static VlcSet {
    static SETS: [OnceLock<VlcSet>; 6] = [const { OnceLock::new() }; 6];
    SETS[i].get_or_init(|| match i {
        0 => build_set(&AMP_E_1, &RUN_E_1, &DC_E_1),
        1 => build_set(&AMP_E_4, &RUN_E_4, &DC_E_4),
        2 => build_set(&AMP_E_7, &RUN_E_7, &DC_E_7),
        3 => build_set(&AMP_E_10, &RUN_E_10, &DC_E_10),
        4 => build_set(&AMP_E_13, &RUN_E_13, &DC_E_13),
        _ => build_set(&AMP_E_16, &RUN_E_16, &DC_E_16),
    })
}

pub const BASIS: [[f32; 8]; 8] = [
    [
        0.353_553_39,
        0.353_553_39,
        0.353_553_39,
        0.353_553_39,
        0.353_553_39,
        0.353_553_39,
        0.353_553_39,
        0.353_553_39,
    ],
    [
        0.490_392_64,
        0.415_734_8,
        0.277_785_12,
        0.097_545_16,
        -0.097_545_16,
        -0.277_785_12,
        -0.415_734_8,
        -0.490_392_64,
    ],
    [
        0.461_939_77,
        0.191_341_72,
        -0.191_341_72,
        -0.461_939_77,
        -0.461_939_77,
        -0.191_341_72,
        0.191_341_72,
        0.461_939_77,
    ],
    [
        0.415_734_8,
        -0.097_545_16,
        -0.490_392_64,
        -0.277_785_12,
        0.277_785_12,
        0.490_392_64,
        0.097_545_16,
        -0.415_734_8,
    ],
    [
        0.353_553_39,
        -0.353_553_39,
        -0.353_553_39,
        0.353_553_39,
        0.353_553_39,
        -0.353_553_39,
        -0.353_553_39,
        0.353_553_39,
    ],
    [
        0.277_785_12,
        -0.490_392_64,
        0.097_545_16,
        0.415_734_8,
        -0.415_734_8,
        -0.097_545_16,
        0.490_392_64,
        -0.277_785_12,
    ],
    [
        0.191_341_72,
        -0.461_939_77,
        0.461_939_77,
        -0.191_341_72,
        -0.191_341_72,
        0.461_939_77,
        -0.461_939_77,
        0.191_341_72,
    ],
    [
        0.097_545_16,
        -0.277_785_12,
        0.415_734_8,
        -0.490_392_64,
        0.490_392_64,
        -0.415_734_8,
        0.277_785_12,
        -0.097_545_16,
    ],
];
