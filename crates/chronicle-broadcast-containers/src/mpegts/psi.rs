//! Program Specific Information (PSI): Program Association Table (PAT) and
//! Program Map Table (PMT) with CRC-32 MPEG-2 checksum.

/// CRC-32 MPEG-2 (polynomial 0x04C11DB7, initial 0xFFFFFFFF).
pub fn crc32_mpeg(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= (b as u32) << 24;
        for _ in 0..8 {
            crc = if crc & 0x8000_0000 != 0 {
                (crc << 1) ^ 0x04C1_1DB7
            } else {
                crc << 1
            };
        }
    }
    crc
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatProgram {
    pub program_number: u16,
    pub pmt_pid: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pat {
    pub transport_stream_id: u16,
    pub version: u8,
    pub programs: Vec<PatProgram>,
}

pub fn parse_pat(section: &[u8]) -> Option<Pat> {
    if section.len() < 8 || section[0] != 0x00 {
        return None;
    }
    let section_len = (((section[1] & 0x0F) as usize) << 8) | (section[2] as usize);
    if section.len() < 3 + section_len || section_len < 9 {
        return None;
    }
    if crc32_mpeg(&section[..3 + section_len]) != 0 {
        return None;
    }

    let ts_id = u16::from_be_bytes([section[3], section[4]]);
    let version = (section[5] >> 1) & 0x1F;

    let mut programs = Vec::new();
    let mut pos = 8;
    let end = 3 + section_len - 4; // exclude CRC
    while pos + 4 <= end {
        let prog_num = u16::from_be_bytes([section[pos], section[pos + 1]]);
        let pid = (((section[pos + 2] & 0x1F) as u16) << 8) | (section[pos + 3] as u16);
        programs.push(PatProgram {
            program_number: prog_num,
            pmt_pid: pid,
        });
        pos += 4;
    }

    Some(Pat {
        transport_stream_id: ts_id,
        version,
        programs,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct PmtStream {
    pub stream_type: u8,
    pub elementary_pid: u16,
    pub descriptors: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pmt {
    pub program_number: u16,
    pub version: u8,
    pub pcr_pid: u16,
    pub streams: Vec<PmtStream>,
}

pub fn parse_pmt(section: &[u8]) -> Option<Pmt> {
    if section.len() < 12 || section[0] != 0x02 {
        return None;
    }
    let section_len = (((section[1] & 0x0F) as usize) << 8) | (section[2] as usize);
    if section.len() < 3 + section_len || section_len < 13 {
        return None;
    }
    if crc32_mpeg(&section[..3 + section_len]) != 0 {
        return None;
    }

    let prog_num = u16::from_be_bytes([section[3], section[4]]);
    let version = (section[5] >> 1) & 0x1F;
    let pcr_pid = (((section[8] & 0x1F) as u16) << 8) | (section[9] as u16);
    let prog_info_len = (((section[10] & 0x0F) as usize) << 8) | (section[11] as usize);

    let mut pos = 12 + prog_info_len;
    let end = 3 + section_len - 4;
    let mut streams = Vec::new();

    while pos + 5 <= end {
        let stream_type = section[pos];
        let elem_pid = (((section[pos + 1] & 0x1F) as u16) << 8) | (section[pos + 2] as u16);
        let es_info_len = (((section[pos + 3] & 0x0F) as usize) << 8) | (section[pos + 4] as usize);
        pos += 5;
        if pos + es_info_len > end {
            break;
        }
        let descriptors = section[pos..pos + es_info_len].to_vec();
        pos += es_info_len;

        streams.push(PmtStream {
            stream_type,
            elementary_pid: elem_pid,
            descriptors,
        });
    }

    Some(Pmt {
        program_number: prog_num,
        version,
        pcr_pid,
        streams,
    })
}

/// Build a PAT section including CRC-32.
pub fn build_pat(transport_stream_id: u16, version: u8, programs: &[PatProgram]) -> Vec<u8> {
    let mut sec = Vec::new();
    sec.push(0x00); // Table ID
    let section_len = (5 + programs.len() * 4 + 4) as u16;
    sec.push(0xB0 | ((section_len >> 8) as u8 & 0x0F));
    sec.push(section_len as u8);
    sec.extend_from_slice(&transport_stream_id.to_be_bytes());
    sec.push(0xC1 | ((version & 0x1F) << 1)); // current/next = 1
    sec.push(0x00); // Section number
    sec.push(0x00); // Last section number
    for p in programs {
        sec.extend_from_slice(&p.program_number.to_be_bytes());
        sec.push(0xE0 | ((p.pmt_pid >> 8) as u8 & 0x1F));
        sec.push(p.pmt_pid as u8);
    }
    let crc = crc32_mpeg(&sec);
    sec.extend_from_slice(&crc.to_be_bytes());
    sec
}

/// Build a PMT section including CRC-32.
pub fn build_pmt(program_number: u16, version: u8, pcr_pid: u16, streams: &[PmtStream]) -> Vec<u8> {
    let mut sec = Vec::new();
    sec.push(0x02); // Table ID
    let mut stream_bytes = Vec::new();
    for s in streams {
        stream_bytes.push(s.stream_type);
        stream_bytes.push(0xE0 | ((s.elementary_pid >> 8) as u8 & 0x1F));
        stream_bytes.push(s.elementary_pid as u8);
        let desc_len = s.descriptors.len() as u16;
        stream_bytes.push(0xF0 | ((desc_len >> 8) as u8 & 0x0F));
        stream_bytes.push(desc_len as u8);
        stream_bytes.extend_from_slice(&s.descriptors);
    }
    let section_len = (9 + stream_bytes.len() + 4) as u16;
    sec.push(0xB0 | ((section_len >> 8) as u8 & 0x0F));
    sec.push(section_len as u8);
    sec.extend_from_slice(&program_number.to_be_bytes());
    sec.push(0xC1 | ((version & 0x1F) << 1));
    sec.push(0x00);
    sec.push(0x00);
    sec.push(0xE0 | ((pcr_pid >> 8) as u8 & 0x1F));
    sec.push(pcr_pid as u8);
    sec.push(0xF0); // Program info length = 0
    sec.push(0x00);
    sec.extend_from_slice(&stream_bytes);
    let crc = crc32_mpeg(&sec);
    sec.extend_from_slice(&crc.to_be_bytes());
    sec
}
