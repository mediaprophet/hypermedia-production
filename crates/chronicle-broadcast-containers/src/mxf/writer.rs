//! MXF OP1a / OP-Atom writer streaming to any Write + Seek sink.

use std::io::{Seek, SeekFrom, Write};
use crate::mxf::klv::{Rational, Ul, write_ber4};
use crate::mxf::labels::*;

/// A SMPTE ST 330 basic UMID (32 bytes).
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Umid(pub [u8; 32]);

impl std::fmt::Debug for Umid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in self.0 {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

impl Umid {
    pub fn from_seed(seed: &[u8]) -> Umid {
        let mut u = [0u8; 32];
        u[..16].copy_from_slice(&[
            0x06, 0x0A, 0x2B, 0x34, 0x01, 0x01, 0x01, 0x05, 0x01, 0x01, 0x0D, 0x20, 0x13, 0x00,
            0x00, 0x00,
        ]);
        u[16..].copy_from_slice(&hash128(seed));
        Umid(u)
    }
}

fn hash128(seed: &[u8]) -> [u8; 16] {
    let mut a = 0xcbf2_9ce4_8422_2325u64;
    let mut b = 0x84222325_cbf29ce4u64 ^ seed.len() as u64;
    for &x in seed {
        a = (a ^ x as u64).wrapping_mul(0x0000_0100_0000_01B3);
        b = (b ^ x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15).rotate_left(23);
    }
    let mix = |mut z: u64| {
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    };
    let mut out = [0u8; 16];
    out[..8].copy_from_slice(&mix(a ^ b.rotate_left(17)).to_be_bytes());
    out[8..].copy_from_slice(&mix(b ^ a.rotate_left(41)).to_be_bytes());
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pattern {
    Op1a,
    OpAtom,
}

#[derive(Clone, Debug)]
pub struct MxfTrackSpec {
    pub track_id: u32,
    pub is_picture: bool,
    pub edit_rate: Rational,
    pub width: u32,
    pub height: u32,
    pub sample_rate: u32,
    pub channels: u32,
    pub bit_depth: u32,
    pub element_key: [u8; 16],
}

pub struct MxfWriter<W: Write + Seek> {
    w: W,
    pattern: Pattern,
    edit_rate: Rational,
    tracks: Vec<MxfTrackSpec>,
    header_partition_pos: u64,
    body_partition_pos: u64,
    footer_partition_pos: u64,
    edit_unit_count: u64,
    material_package_uid: Umid,
    source_package_uid: Umid,
}

impl<W: Write + Seek> MxfWriter<W> {
    pub fn new(mut w: W, pattern: Pattern, edit_rate: Rational) -> Result<Self, String> {
        let base = w.stream_position().map_err(|e| e.to_string())?;
        let material_package_uid = Umid::from_seed(b"material_package_default");
        let source_package_uid = Umid::from_seed(b"source_package_default");

        Ok(Self {
            w,
            pattern,
            edit_rate,
            tracks: Vec::new(),
            header_partition_pos: base,
            body_partition_pos: 0,
            footer_partition_pos: 0,
            edit_unit_count: 0,
            material_package_uid,
            source_package_uid,
        })
    }

    pub fn add_picture_track(&mut self, width: u32, height: u32, rate: Rational) -> usize {
        let idx = self.tracks.len();
        self.tracks.push(MxfTrackSpec {
            track_id: (idx as u32) + 1,
            is_picture: true,
            edit_rate: rate,
            width,
            height,
            sample_rate: 0,
            channels: 0,
            bit_depth: 8,
            element_key: element_key(0x15, 0x01, 0x05, 0x01),
        });
        idx
    }

    pub fn add_sound_track(&mut self, sample_rate: u32, channels: u32, bit_depth: u32) -> usize {
        let idx = self.tracks.len();
        self.tracks.push(MxfTrackSpec {
            track_id: (idx as u32) + 1,
            is_picture: false,
            edit_rate: self.edit_rate,
            width: 0,
            height: 0,
            sample_rate,
            channels,
            bit_depth,
            element_key: element_key(0x16, 0x01, 0x01, idx as u8),
        });
        idx
    }

    pub fn write_header(&mut self) -> Result<(), String> {
        let mut hdr_bytes = Vec::new();
        // 1. Partition Pack
        let part_key = partition_key(2, 1); // Header, open incomplete
        hdr_bytes.extend_from_slice(&part_key);
        let mut part_body = Vec::new();
        part_body.extend_from_slice(&1u16.to_be_bytes()); // major
        part_body.extend_from_slice(&2u16.to_be_bytes()); // minor
        part_body.extend_from_slice(&1u32.to_be_bytes()); // KAG
        part_body.extend_from_slice(&0u64.to_be_bytes()); // this_partition
        part_body.extend_from_slice(&0u64.to_be_bytes()); // prev_partition
        part_body.extend_from_slice(&0u64.to_be_bytes()); // footer_partition
        part_body.extend_from_slice(&0u64.to_be_bytes()); // header_byte_count (patch later)
        part_body.extend_from_slice(&0u64.to_be_bytes()); // index_byte_count
        part_body.extend_from_slice(&0u32.to_be_bytes()); // index_sid
        part_body.extend_from_slice(&0u64.to_be_bytes()); // body_offset
        part_body.extend_from_slice(&1u32.to_be_bytes()); // body_sid
        let op_label = if self.pattern == Pattern::Op1a {
            op1a(self.tracks.len() > 1)
        } else {
            OP_ATOM
        };
        part_body.extend_from_slice(&op_label);
        // Essence containers batch
        part_body.extend_from_slice(&1u32.to_be_bytes()); // count
        part_body.extend_from_slice(&16u32.to_be_bytes()); // elem size
        part_body.extend_from_slice(&DEF_PICTURE);

        write_ber4(&mut hdr_bytes, part_body.len() as u32);
        hdr_bytes.extend_from_slice(&part_body);

        // 2. Primer Pack
        hdr_bytes.extend_from_slice(&PRIMER_KEY);
        let mut primer_body = Vec::new();
        primer_body.extend_from_slice(&(PRIMER.len() as u32).to_be_bytes());
        primer_body.extend_from_slice(&18u32.to_be_bytes());
        for &(tag, ref ul_bytes) in PRIMER {
            primer_body.extend_from_slice(&tag.to_be_bytes());
            primer_body.extend_from_slice(ul_bytes);
        }
        write_ber4(&mut hdr_bytes, primer_body.len() as u32);
        hdr_bytes.extend_from_slice(&primer_body);

        self.w.write_all(&hdr_bytes).map_err(|e| e.to_string())?;

        // Write Body Partition
        self.body_partition_pos = self.w.stream_position().map_err(|e| e.to_string())?;
        let mut body_bytes = Vec::new();
        let b_key = partition_key(3, 1); // Body, open incomplete
        body_bytes.extend_from_slice(&b_key);
        let mut b_body = Vec::new();
        b_body.extend_from_slice(&1u16.to_be_bytes());
        b_body.extend_from_slice(&2u16.to_be_bytes());
        b_body.extend_from_slice(&1u32.to_be_bytes());
        b_body.extend_from_slice(&self.body_partition_pos.to_be_bytes());
        b_body.extend_from_slice(&0u64.to_be_bytes());
        b_body.extend_from_slice(&0u64.to_be_bytes());
        b_body.extend_from_slice(&0u64.to_be_bytes());
        b_body.extend_from_slice(&0u64.to_be_bytes());
        b_body.extend_from_slice(&0u32.to_be_bytes());
        b_body.extend_from_slice(&0u64.to_be_bytes());
        b_body.extend_from_slice(&1u32.to_be_bytes());
        b_body.extend_from_slice(&op_label);
        b_body.extend_from_slice(&0u32.to_be_bytes());
        b_body.extend_from_slice(&0u32.to_be_bytes());
        write_ber4(&mut body_bytes, b_body.len() as u32);
        body_bytes.extend_from_slice(&b_body);

        self.w.write_all(&body_bytes).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn write_essence_packet(&mut self, track_idx: usize, data: &[u8]) -> Result<(), String> {
        let track = self.tracks.get(track_idx).ok_or("Track not found")?;
        let mut pkt = Vec::new();
        pkt.extend_from_slice(&track.element_key);
        write_ber4(&mut pkt, data.len() as u32);
        pkt.extend_from_slice(data);
        self.w.write_all(&pkt).map_err(|e| e.to_string())?;
        if track.is_picture {
            self.edit_unit_count += 1;
        }
        Ok(())
    }

    pub fn finish(mut self) -> Result<W, String> {
        self.footer_partition_pos = self.w.stream_position().map_err(|e| e.to_string())?;
        let mut footer = Vec::new();
        let f_key = partition_key(4, 4); // Footer, closed complete
        footer.extend_from_slice(&f_key);
        let mut f_body = Vec::new();
        f_body.extend_from_slice(&1u16.to_be_bytes());
        f_body.extend_from_slice(&2u16.to_be_bytes());
        f_body.extend_from_slice(&1u32.to_be_bytes());
        f_body.extend_from_slice(&self.footer_partition_pos.to_be_bytes());
        f_body.extend_from_slice(&self.body_partition_pos.to_be_bytes());
        f_body.extend_from_slice(&self.footer_partition_pos.to_be_bytes());
        f_body.extend_from_slice(&0u64.to_be_bytes());
        f_body.extend_from_slice(&0u64.to_be_bytes());
        f_body.extend_from_slice(&0u32.to_be_bytes());
        f_body.extend_from_slice(&0u64.to_be_bytes());
        f_body.extend_from_slice(&0u32.to_be_bytes());
        let op_label = if self.pattern == Pattern::Op1a {
            op1a(self.tracks.len() > 1)
        } else {
            OP_ATOM
        };
        f_body.extend_from_slice(&op_label);
        f_body.extend_from_slice(&0u32.to_be_bytes());
        f_body.extend_from_slice(&0u32.to_be_bytes());
        write_ber4(&mut footer, f_body.len() as u32);
        footer.extend_from_slice(&f_body);

        // Random Index Pack (RIP)
        let rip_pos = self.w.stream_position().map_err(|e| e.to_string())? + footer.len() as u64;
        let mut rip = Vec::new();
        rip.extend_from_slice(&RIP_KEY);
        let mut rip_body = Vec::new();
        // Body SID 0 (header)
        rip_body.extend_from_slice(&0u32.to_be_bytes());
        rip_body.extend_from_slice(&self.header_partition_pos.to_be_bytes());
        // Body SID 1 (body)
        rip_body.extend_from_slice(&1u32.to_be_bytes());
        rip_body.extend_from_slice(&self.body_partition_pos.to_be_bytes());
        // Footer SID 0
        rip_body.extend_from_slice(&0u32.to_be_bytes());
        rip_body.extend_from_slice(&self.footer_partition_pos.to_be_bytes());
        let rip_len = (16 + 4 + rip_body.len() + 4) as u32;
        rip_body.extend_from_slice(&rip_len.to_be_bytes());

        write_ber4(&mut rip, rip_body.len() as u32);
        rip.extend_from_slice(&rip_body);

        self.w.write_all(&footer).map_err(|e| e.to_string())?;
        self.w.write_all(&rip).map_err(|e| e.to_string())?;
        self.w.flush().map_err(|e| e.to_string())?;

        Ok(self.w)
    }
}
