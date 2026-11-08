//! Header metadata (SMPTE ST 377-1 §9): primer packs and local sets.

use std::collections::HashMap;
use crate::mxf::klv::{Cur, Rational, Ul, batch_of, int_of, rational_of, ul_of, utf16_of};

/// Local tag -> property UL mapping.
#[derive(Clone, Debug, Default)]
pub struct Primer(pub HashMap<u16, Ul>);

impl Primer {
    pub fn parse(v: &[u8]) -> Primer {
        let mut map = HashMap::new();
        for item in batch_of(v) {
            if item.len() >= 18 {
                map.insert(
                    u16::from_be_bytes([item[0], item[1]]),
                    Ul(item[2..18].try_into().unwrap_or([0; 16])),
                );
            }
        }
        Primer(map)
    }
}

pub mod set_type {
    pub const PREFACE: u8 = 0x2F;
    pub const IDENTIFICATION: u8 = 0x30;
    pub const CONTENT_STORAGE: u8 = 0x18;
    pub const ESSENCE_CONTAINER_DATA: u8 = 0x23;
    pub const MATERIAL_PACKAGE: u8 = 0x36;
    pub const SOURCE_PACKAGE: u8 = 0x37;
    pub const TIMELINE_TRACK: u8 = 0x3B;
    pub const SEQUENCE: u8 = 0x0F;
    pub const SOURCE_CLIP: u8 = 0x11;
    pub const TIMECODE_COMPONENT: u8 = 0x14;
    pub const FILLER: u8 = 0x09;
    pub const CDCI_DESCRIPTOR: u8 = 0x28;
    pub const RGBA_DESCRIPTOR: u8 = 0x29;
    pub const WAVE_DESCRIPTOR: u8 = 0x48;
}

/// One header metadata local set.
#[derive(Clone, Debug, Default)]
pub struct Set {
    pub key: Ul,
    pub instance_uid: [u8; 16],
    pub props: HashMap<u16, Vec<u8>>,
    pub dynamic: HashMap<Ul, Vec<u8>>,
}

impl Set {
    pub fn parse(key: Ul, v: &[u8], primer: &Primer) -> Set {
        let mut s = Set {
            key,
            ..Default::default()
        };
        let mut c = Cur::new(v);
        while c.remaining() >= 4 {
            let (Some(tag), Some(len)) = (c.u16(), c.u16()) else { break };
            let Some(val) = c.bytes(len as usize) else { break };
            if tag == 0x3C0A && val.len() >= 16 {
                s.instance_uid.copy_from_slice(&val[..16]);
            }
            if tag >= 0x8000 {
                if let Some(ul) = primer.0.get(&tag) {
                    s.dynamic.insert(*ul, val.to_vec());
                }
            } else {
                s.props.insert(tag, val.to_vec());
            }
        }
        s
    }

    pub fn kind(&self) -> Option<u8> {
        self.key
            .item_starts_with(&[0x0D, 0x01, 0x01, 0x01, 0x01, 0x01])
            .then_some(self.key.0[14])
    }

    pub fn get(&self, tag: u16) -> Option<&[u8]> {
        self.props.get(&tag).map(Vec::as_slice)
    }

    pub fn u(&self, tag: u16) -> Option<i64> {
        self.get(tag).and_then(|v| int_of(v, false))
    }

    pub fn i(&self, tag: u16) -> Option<i64> {
        self.get(tag).and_then(|v| int_of(v, true))
    }

    pub fn rational(&self, tag: u16) -> Option<Rational> {
        self.get(tag).and_then(rational_of)
    }

    pub fn ul(&self, tag: u16) -> Option<Ul> {
        self.get(tag).and_then(ul_of)
    }

    pub fn string(&self, tag: u16) -> Option<String> {
        self.get(tag).map(utf16_of).filter(|s| !s.is_empty())
    }
}

/// All sets of one partition's header metadata.
#[derive(Clone, Debug, Default)]
pub struct Metadata {
    pub sets: Vec<Set>,
    by_uid: HashMap<[u8; 16], usize>,
}

impl Metadata {
    pub fn push(&mut self, s: Set) {
        if s.instance_uid != [0; 16] {
            self.by_uid.insert(s.instance_uid, self.sets.len());
        }
        self.sets.push(s);
    }

    pub fn get(&self, uid: &[u8; 16]) -> Option<&Set> {
        self.by_uid.get(uid).map(|&i| &self.sets[i])
    }

    pub fn of_kind(&self, kind: u8) -> impl Iterator<Item = &Set> {
        self.sets.iter().filter(move |s| s.kind() == Some(kind))
    }
}

pub mod tag {
    pub const PACKAGE_UID: u16 = 0x4401;
    pub const PACKAGE_NAME: u16 = 0x4402;
    pub const PACKAGE_TRACKS: u16 = 0x4403;
    pub const SOURCE_PACKAGE_DESCRIPTOR: u16 = 0x4701;
    pub const TRACK_ID: u16 = 0x4801;
    pub const TRACK_NUMBER: u16 = 0x4804;
    pub const TRACK_NAME: u16 = 0x4802;
    pub const TRACK_SEQUENCE: u16 = 0x4803;
    pub const EDIT_RATE: u16 = 0x4B01;
    pub const ORIGIN: u16 = 0x4B02;
    pub const DATA_DEFINITION: u16 = 0x0201;
    pub const DURATION: u16 = 0x0202;
    pub const STRUCTURAL_COMPONENTS: u16 = 0x1001;
    pub const START_POSITION: u16 = 0x1201;
    pub const SOURCE_PACKAGE_ID: u16 = 0x1101;
    pub const SOURCE_TRACK_ID: u16 = 0x1102;
    pub const OPERATIONAL_PATTERN: u16 = 0x3B09;
    pub const PRODUCT_NAME: u16 = 0x3C02;
    pub const COMPANY_NAME: u16 = 0x3C01;
    pub const STORED_WIDTH: u16 = 0x3203;
    pub const STORED_HEIGHT: u16 = 0x3202;
    pub const ASPECT_RATIO: u16 = 0x320E;
    pub const AUDIO_SAMPLING_RATE: u16 = 0x3D03;
    pub const CHANNEL_COUNT: u16 = 0x3D07;
    pub const QUANTIZATION_BITS: u16 = 0x3D01;
}
