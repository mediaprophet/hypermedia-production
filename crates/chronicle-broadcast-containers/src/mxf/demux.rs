//! MXF demuxer for parsing partitions, metadata, and essence packets.

use crate::mxf::klv::{Cur, KeyClass, Rational, Ul, ber_length, classify};
use crate::mxf::meta::{Metadata, Primer, Set};
use crate::mxf::partition::{OperationalPattern, Partition};

#[derive(Clone, Debug)]
pub struct MxfEssencePacket {
    pub key: Ul,
    pub offset: u64,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct MxfDemuxer {
    pub partitions: Vec<Partition>,
    pub operational_pattern: OperationalPattern,
    pub metadata: Metadata,
    pub packets: Vec<MxfEssencePacket>,
}

pub fn demux_mxf(data: &[u8]) -> Result<MxfDemuxer, String> {
    let mut demux = MxfDemuxer::default();
    let mut primer = Primer::default();
    let mut pos = 0;

    while pos + 16 <= data.len() {
        let key_bytes: [u8; 16] = data[pos..pos + 16].try_into().unwrap();
        let key = Ul(key_bytes);
        pos += 16;

        let Some((len, len_bytes)) = ber_length(&data[pos..]) else {
            break;
        };
        pos += len_bytes;
        let len = len as usize;
        if pos + len > data.len() {
            break;
        }
        let val = &data[pos..pos + len];

        match classify(&key) {
            KeyClass::Partition { kind, status } => {
                if let Some(p) = Partition::parse(pos as u64 - 16 - len_bytes as u64, kind, status, val) {
                    if demux.partitions.is_empty() {
                        demux.operational_pattern = OperationalPattern::from_ul(&p.operational_pattern);
                    }
                    demux.partitions.push(p);
                }
            }
            KeyClass::Primer => {
                primer = Primer::parse(val);
            }
            KeyClass::LocalSet => {
                let set = Set::parse(key, val, &primer);
                demux.metadata.push(set);
            }
            KeyClass::Essence => {
                demux.packets.push(MxfEssencePacket {
                    key,
                    offset: pos as u64,
                    payload: val.to_vec(),
                });
            }
            _ => {}
        }
        pos += len;
    }

    Ok(demux)
}
