//! ISO/IEC 13818-1 188-byte Transport Stream (TS) packet parser and serializer.

pub const TS_PACKET_SIZE: usize = 188;
pub const TS_SYNC_BYTE: u8 = 0x47;

/// A parsed 188-byte TS packet.
#[derive(Clone, Debug, PartialEq)]
pub struct TsPacket<'a> {
    pub pusi: bool,
    pub pid: u16,
    pub transport_error: bool,
    pub transport_priority: bool,
    pub scrambling_control: u8,
    pub adaptation_field_control: u8,
    pub continuity_counter: u8,
    pub discontinuity: bool,
    pub random_access: bool,
    pub pcr: Option<i64>,
    pub payload: &'a [u8],
}

/// Parse a single 188-byte TS packet.
pub fn parse_ts_packet(p: &[u8]) -> Option<TsPacket<'_>> {
    if p.len() < TS_PACKET_SIZE || p[0] != TS_SYNC_BYTE {
        return None;
    }
    let transport_error = (p[1] & 0x80) != 0;
    let pusi = (p[1] & 0x40) != 0;
    let transport_priority = (p[1] & 0x20) != 0;
    let pid = (((p[1] & 0x1F) as u16) << 8) | (p[2] as u16);
    let scrambling_control = (p[3] >> 6) & 0x03;
    let afc = (p[3] >> 4) & 0x03;
    let continuity_counter = p[3] & 0x0F;

    let mut start = 4;
    let mut pcr = None;
    let mut discontinuity = false;
    let mut random_access = false;

    if afc & 2 != 0 {
        let al = p[4] as usize;
        start = 5 + al;
        if start > TS_PACKET_SIZE {
            return None;
        }
        if al > 0 {
            let flags = p[5];
            discontinuity = (flags & 0x80) != 0;
            random_access = (flags & 0x40) != 0;
            if (flags & 0x10) != 0 && al >= 7 {
                let base = ((p[6] as i64) << 25)
                    | ((p[7] as i64) << 17)
                    | ((p[8] as i64) << 9)
                    | ((p[9] as i64) << 1)
                    | (p[10] as i64 >> 7);
                let ext = (((p[10] & 1) as i64) << 8) | p[11] as i64;
                pcr = Some(base * 300 + ext);
            }
        }
    }

    let has_payload = (afc & 1) != 0;
    let payload = if has_payload && start <= TS_PACKET_SIZE {
        &p[start..TS_PACKET_SIZE]
    } else {
        &[]
    };

    Some(TsPacket {
        pusi,
        pid,
        transport_error,
        transport_priority,
        scrambling_control,
        adaptation_field_control: afc,
        continuity_counter,
        discontinuity,
        random_access,
        pcr,
        payload,
    })
}

/// Write a 188-byte TS packet into `out`.
pub fn write_ts_packet(
    pid: u16,
    pusi: bool,
    continuity_counter: u8,
    pcr: Option<i64>,
    payload: &[u8],
    out: &mut [u8; TS_PACKET_SIZE],
) {
    out[0] = TS_SYNC_BYTE;
    out[1] = (if pusi { 0x40 } else { 0 }) | ((pid >> 8) as u8 & 0x1F);
    out[2] = pid as u8;

    let has_pcr = pcr.is_some();
    let needed_af_len = if has_pcr { 7 } else { 0 };
    let max_payload_without_af = 184;

    if payload.len() <= max_payload_without_af && !has_pcr {
        let stuffing = max_payload_without_af - payload.len();
        if stuffing == 0 {
            out[3] = 0x10 | (continuity_counter & 0x0F); // AFC = 01 (payload only)
            out[4..188].copy_from_slice(payload);
        } else if stuffing == 1 {
            out[3] = 0x30 | (continuity_counter & 0x0F); // AFC = 11 (adaptation + payload)
            out[4] = 0; // AF length = 0
            out[5..5 + payload.len()].copy_from_slice(payload);
        } else {
            out[3] = 0x30 | (continuity_counter & 0x0F);
            out[4] = (stuffing - 1) as u8;
            out[5] = 0; // flags
            for i in 6..4 + stuffing {
                out[i] = 0xFF;
            }
            out[4 + stuffing..188].copy_from_slice(payload);
        }
    } else {
        // Adaptation field with optional PCR
        out[3] = 0x30 | (continuity_counter & 0x0F);
        let af_len = 184 - payload.len();
        out[4] = (af_len - 1) as u8;
        if has_pcr {
            out[5] = 0x10; // PCR flag
            let p = pcr.unwrap();
            let base = p / 300;
            let ext = p % 300;
            out[6] = (base >> 25) as u8;
            out[7] = (base >> 17) as u8;
            out[8] = (base >> 9) as u8;
            out[9] = (base >> 1) as u8;
            out[10] = (((base & 1) << 7) | 0x7E | ((ext >> 8) & 1)) as u8;
            out[11] = ext as u8;
            for i in 12..4 + af_len {
                out[i] = 0xFF;
            }
        } else {
            out[5] = 0;
            for i in 6..4 + af_len {
                out[i] = 0xFF;
            }
        }
        out[4 + af_len..188].copy_from_slice(payload);
    }
}
