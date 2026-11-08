//! MXF Partition packs (SMPTE ST 377-1 §7) and Operational Patterns (ST 378 / ST 390).

use crate::mxf::klv::{Cur, Ul, batch_of, ul_of};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PartitionKind {
    Header,
    Body,
    Footer,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partition {
    pub offset: u64,
    pub kind: PartitionKind,
    pub status: u8,
    pub major_version: u16,
    pub minor_version: u16,
    pub kag_size: u32,
    pub this_partition: u64,
    pub previous_partition: u64,
    pub footer_partition: u64,
    pub header_byte_count: u64,
    pub index_byte_count: u64,
    pub index_sid: u32,
    pub body_offset: u64,
    pub body_sid: u32,
    pub operational_pattern: Ul,
    pub essence_containers: Vec<Ul>,
}

impl Partition {
    pub fn parse(offset: u64, kind_byte: u8, status: u8, v: &[u8]) -> Option<Partition> {
        let mut c = Cur::new(v);
        let mut p = Partition {
            offset,
            kind: match kind_byte {
                2 => PartitionKind::Header,
                3 => PartitionKind::Body,
                _ => PartitionKind::Footer,
            },
            status,
            major_version: c.u16()?,
            minor_version: c.u16()?,
            kag_size: c.u32()?,
            this_partition: c.u64()?,
            previous_partition: c.u64()?,
            footer_partition: c.u64()?,
            header_byte_count: c.u64()?,
            index_byte_count: c.u64()?,
            index_sid: c.u32()?,
            body_offset: c.u64()?,
            body_sid: c.u32()?,
            operational_pattern: c.ul()?,
            essence_containers: Vec::new(),
        };
        let rest = &v[c.pos..];
        p.essence_containers = batch_of(rest).into_iter().filter_map(ul_of).collect();
        Some(p)
    }

    pub fn closed(&self) -> bool {
        self.status == 2 || self.status == 4
    }

    pub fn complete(&self) -> bool {
        self.status >= 3
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OperationalPattern {
    #[default]
    Op1a,
    OpAtom,
    Unknown,
}

impl OperationalPattern {
    pub fn from_ul(ul: &Ul) -> Self {
        if !ul.is_smpte() || !ul.item_starts_with(&[0x0D, 0x01, 0x02, 0x01]) {
            return OperationalPattern::Unknown;
        }
        match (ul.0[12], ul.0[13]) {
            (1, 1) => OperationalPattern::Op1a,
            (0x10, _) => OperationalPattern::OpAtom,
            _ => OperationalPattern::Unknown,
        }
    }
}
