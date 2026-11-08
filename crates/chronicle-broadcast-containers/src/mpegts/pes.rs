//! Packetized Elementary Stream (PES) header parser and serializer (ISO/IEC 13818-1 §2.4.3.6).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PesHeader {
    pub stream_id: u8,
    pub packet_length: usize,
    pub header_len: usize,
    pub pts: Option<i64>,
    pub dts: Option<i64>,
}

pub fn plain_stream(id: u8) -> bool {
    matches!(id, 0xBC | 0xBE | 0xBF | 0xF0 | 0xF1 | 0xF2 | 0xF8 | 0xFF)
}

pub fn parse_timestamp(b: &[u8]) -> i64 {
    (((b[0] >> 1) & 7) as i64) << 30
        | ((b[1] as i64) << 22)
        | (((b[2] >> 1) as i64) << 15)
        | ((b[3] as i64) << 7)
        | ((b[4] >> 1) as i64)
}

pub fn encode_timestamp(prefix: u8, v: i64) -> [u8; 5] {
    [
        (prefix << 4) | ((((v >> 30) & 7) as u8) << 1) | 1,
        (v >> 22) as u8,
        ((((v >> 15) & 0x7F) as u8) << 1) | 1,
        (v >> 7) as u8,
        (((v & 0x7F) as u8) << 1) | 1,
    ]
}

pub fn parse_pes_header(b: &[u8]) -> Option<PesHeader> {
    if b.len() < 6 || b[0] != 0 || b[1] != 0 || b[2] != 1 {
        return None;
    }
    let stream_id = b[3];
    let packet_length = u16::from_be_bytes([b[4], b[5]]) as usize;
    if plain_stream(stream_id) {
        return Some(PesHeader {
            stream_id,
            packet_length,
            header_len: 6,
            pts: None,
            dts: None,
        });
    }

    if b.len() >= 9 && (b[6] & 0xC0) == 0x80 {
        let flags = b[7];
        let header_len = 9 + b[8] as usize;
        if b.len() < header_len {
            return None;
        }
        let mut pts = None;
        let mut dts = None;
        if (flags & 0x80) != 0 && header_len >= 14 {
            pts = Some(parse_timestamp(&b[9..14]));
            if (flags & 0x40) != 0 && header_len >= 19 {
                dts = Some(parse_timestamp(&b[14..19]));
            }
        }
        return Some(PesHeader {
            stream_id,
            packet_length,
            header_len,
            pts,
            dts,
        });
    }

    // ISO/IEC 11172-1 syntax
    let mut p = 6;
    let mut stuffing = 0;
    while b.get(p) == Some(&0xFF) {
        p += 1;
        stuffing += 1;
        if stuffing > 16 {
            return None;
        }
    }
    let c = *b.get(p)?;
    if c >> 6 == 1 {
        p += 2;
    }
    let c = *b.get(p)?;
    let mut pts = None;
    let mut dts = None;
    match c >> 4 {
        2 => {
            pts = Some(parse_timestamp(b.get(p..p + 5)?));
            p += 5;
        }
        3 => {
            pts = Some(parse_timestamp(b.get(p..p + 5)?));
            dts = Some(parse_timestamp(b.get(p + 5..p + 10)?));
            p += 10;
        }
        0 if c == 0x0F => p += 1,
        _ => return None,
    }
    Some(PesHeader {
        stream_id,
        packet_length,
        header_len: p,
        pts,
        dts,
    })
}

/// Create a PES packet containing `data` with optional PTS/DTS.
pub fn build_pes_packet(
    stream_id: u8,
    pts: Option<i64>,
    dts: Option<i64>,
    data: &[u8],
) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(0x00);
    out.push(0x00);
    out.push(0x01);
    out.push(stream_id);

    let has_pts = pts.is_some();
    let has_dts = dts.is_some();

    let (flags, header_data_len) = match (has_pts, has_dts) {
        (true, true) => (0xC0, 10),
        (true, false) => (0x80, 5),
        _ => (0x00, 0),
    };

    let pes_header_len = 3 + header_data_len;
    let total_payload = pes_header_len + data.len();
    let packet_len = if total_payload > 0xFFFF {
        0u16 // Unbounded (standard for video in TS)
    } else {
        total_payload as u16
    };

    out.extend_from_slice(&packet_len.to_be_bytes());
    out.push(0x80); // MPEG-2 ID
    out.push(flags);
    out.push(header_data_len as u8);

    if let (Some(p), Some(d)) = (pts, dts) {
        out.extend_from_slice(&encode_timestamp(3, p));
        out.extend_from_slice(&encode_timestamp(1, d));
    } else if let Some(p) = pts {
        out.extend_from_slice(&encode_timestamp(2, p));
    }

    out.extend_from_slice(data);
    out
}
