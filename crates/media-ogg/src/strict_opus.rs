//! Strict, bounded Ogg Opus admission ahead of the existing permissive demuxer.
//! This profile accepts one complete logical stream without packet continuation,
//! page loss or chained streams. It does not establish source authority.
use super::{crc32, open, opus_packet_samples, Codec, OggFile};

#[derive(Clone, Copy)]
pub struct OpusLimits {
    pub input_bytes: usize,
    pub pages: usize,
    pub packets: usize,
    pub packet_bytes: usize,
}

// OpusTags contains a little-endian vendor length, vendor bytes, comment
// count and length-prefixed comments. Ignore textual contents; never walk past
// the admitted packet or permit an unbounded number of comments.
fn validate_tags(packet: &[u8]) -> Result<(), String> {
    fn number(packet: &[u8], pos: &mut usize) -> Result<usize, String> {
        let end = pos.checked_add(4).ok_or("OpusTags offset overflow")?;
        let word: [u8; 4] = packet.get(*pos..end).ok_or("Truncated OpusTags length")?.try_into().map_err(|_| "OpusTags length")?;
        *pos = end;
        Ok(u32::from_le_bytes(word) as usize)
    }
    if !packet.starts_with(b"OpusTags") { return Err("Ogg Opus tags missing".into()); }
    let mut pos = 8usize;
    let vendor = number(packet, &mut pos)?;
    pos = pos.checked_add(vendor).filter(|end| *end <= packet.len()).ok_or("Truncated OpusTags vendor")?;
    let comments = number(packet, &mut pos)?;
    if comments > 4096 { return Err("OpusTags comment count exceeds profile".into()); }
    for _ in 0..comments {
        let len = number(packet, &mut pos)?;
        pos = pos.checked_add(len).filter(|end| *end <= packet.len()).ok_or("Truncated OpusTags comment")?;
    }
    // Binary metadata extensions are outside this playback profile. Zero
    // padding is harmless and is not interpreted as a comment.
    if packet[pos..].iter().any(|byte| *byte != 0) { return Err("Unsupported OpusTags extension".into()); }
    Ok(())
}
/// Validate framing, checksums, sequence and timing before invoking the
/// original permissive parser. `input_bytes` also bounds all parser allocations.
/// Each packet must finish on the page on which it begins. Header packets must
/// each occupy a page; audio granules must agree with accumulated packet time,
/// allowing the final page to trim only its own audio. This deliberately small
/// profile is not a general Ogg Opus conformance implementation.
pub fn open_bounded_opus(bytes: &[u8], limits: OpusLimits) -> Result<OggFile, String> {
    if bytes.len() > limits.input_bytes || limits.pages < 3 || limits.packets < 3 || limits.packet_bytes == 0 {
        return Err("Ogg input exceeds profile".into());
    }
    let (mut pos, mut pages, mut packets) = (0usize, 0usize, 0usize);
    let mut serial = None;
    let mut next_sequence = 0u32;
    let mut ended = false;
    let mut total = 0u64;
    let mut pre_skip = 0u64;
    while pos < bytes.len() {
        pages += 1;
        if pages > limits.pages || bytes.len() - pos < 27 || ended { return Err("Ogg page count or trailing data outside profile".into()); }
        let h = &bytes[pos..pos + 27];
        if &h[..4] != b"OggS" || h[4] != 0 || h[5] & !0x06 != 0 { return Err("Invalid or continued Ogg page".into()); }
        let bos = h[5] & 2 != 0;
        let eos = h[5] & 4 != 0;
        if bos != (pages == 1) || (pages <= 2 && eos) { return Err("Invalid Ogg stream boundaries".into()); }
        let sid = u32::from_le_bytes(h[14..18].try_into().map_err(|_| "Ogg serial")?);
        let seq = u32::from_le_bytes(h[18..22].try_into().map_err(|_| "Ogg sequence")?);
        if serial.is_some_and(|old| old != sid) || seq != next_sequence { return Err("Ogg serial or sequence discontinuity".into()); }
        serial = Some(sid);
        next_sequence = next_sequence.checked_add(1).ok_or("Ogg sequence overflow")?;
        let count = h[26] as usize;
        let body_start = pos.checked_add(27).and_then(|p| p.checked_add(count)).filter(|p| *p <= bytes.len()).ok_or("Truncated Ogg lacing")?;
        let laces = &bytes[pos + 27..body_start];
        if laces.is_empty() || laces.last() == Some(&255) { return Err("Empty or continued Ogg page".into()); }
        let body_len: usize = laces.iter().map(|n| usize::from(*n)).sum();
        let page_end = body_start.checked_add(body_len).filter(|p| *p <= bytes.len()).ok_or("Truncated Ogg body")?;
        let mut checksum_page = bytes[pos..page_end].to_vec();
        let expected_crc = u32::from_le_bytes(h[22..26].try_into().map_err(|_| "Ogg CRC")?);
        checksum_page[22..26].fill(0);
        if crc32(&checksum_page) != expected_crc { return Err("Ogg CRC mismatch".into()); }
        let page_before = total;
        let mut packet_start = body_start;
        let mut packet_size = 0usize;
        let mut page_packets = 0usize;
        for &lace in laces {
            packet_size = packet_size.checked_add(usize::from(lace)).ok_or("Ogg packet size overflow")?;
            if packet_size > limits.packet_bytes { return Err("Ogg packet exceeds profile".into()); }
            if lace < 255 {
                page_packets += 1; packets += 1;
                if packets > limits.packets { return Err("Ogg packet count exceeds profile".into()); }
                let packet_end = packet_start.checked_add(packet_size).filter(|p| *p <= page_end).ok_or("Ogg packet extent")?;
                let packet = &bytes[packet_start..packet_end];
                if pages == 1 {
                    if packet.len() != 19 || &packet[..8] != b"OpusHead" || packet[8] != 1 || !(1..=2).contains(&packet[9]) || packet[18] != 0 { return Err("Ogg Opus header outside mono/stereo family-0 profile".into()); }
                    pre_skip = u64::from(u16::from_le_bytes([packet[10], packet[11]]));
                } else if pages == 2 {
                    validate_tags(packet)?;
                } else {
                    let prefix = [packet.first().copied().unwrap_or(0),packet.get(1).copied().unwrap_or(0)];
                    let duration = opus_packet_samples(prefix, packet_size as u32).ok_or("Invalid Opus packet duration")?;
                    total = total.checked_add(u64::from(duration)).ok_or("Opus duration overflow")?;
                }
                packet_start = packet_end; packet_size = 0;
            }
        }
        if packet_start != page_end || packet_size != 0 || (pages <= 2 && page_packets != 1) { return Err("Ogg packet layout outside profile".into()); }
        let granule = i64::from_le_bytes(h[6..14].try_into().map_err(|_| "Ogg granule")?);
        if pages <= 2 {
            if granule != 0 { return Err("Ogg header granule must be zero".into()); }
        } else {
            let g = u64::try_from(granule).map_err(|_| "Missing audio granule")?;
            if (!eos && g != total) || (eos && (g < page_before || g > total || g < pre_skip)) { return Err("Ogg granule discontinuity or invalid final trim".into()); }
        }
        ended = eos; pos = page_end;
    }
    if !ended || pages < 3 || packets < 3 { return Err("Incomplete Ogg Opus stream".into()); }
    let file = open(bytes).map_err(|e| format!("Ogg demux: {e}"))?;
    if !file.warnings.is_empty() || file.streams.len() != 1 || file.streams[0].codec != Codec::Opus || !file.streams[0].saw_eos || file.streams[0].headers.len() != 2 {
        return Err("Ogg demux did not retain the admitted stream".into());
    }
    Ok(file)
}

#[cfg(test)]
mod strict_tests {
    use super::*;
    fn page(out: &mut Vec<u8>, flags: u8, granule: i64, seq: u32, packet: &[u8]) {
        assert!(packet.len() < 255);
        let start = out.len();
        out.extend_from_slice(b"OggS");out.push(0);out.push(flags);
        out.extend_from_slice(&granule.to_le_bytes());out.extend_from_slice(&42u32.to_le_bytes());
        out.extend_from_slice(&seq.to_le_bytes());out.extend_from_slice(&[0;4]);out.push(1);out.push(packet.len() as u8);
        out.extend_from_slice(packet);
        let checksum = crc32(&out[start..]);out[start+22..start+26].copy_from_slice(&checksum.to_le_bytes());
    }
    fn sample() -> Vec<u8> {
        let mut file=Vec::new();let mut head=b"OpusHead".to_vec();
        head.extend_from_slice(&[1,1,120,0]);head.extend_from_slice(&48_000u32.to_le_bytes());head.extend_from_slice(&[0,0,0]);
        let mut tags=b"OpusTags".to_vec();tags.extend_from_slice(&[0;8]);
        page(&mut file,2,0,0,&head);page(&mut file,0,0,1,&tags);page(&mut file,4,900,2,&[0xf8,0xff,0xfe]);file
    }
    fn limits() -> OpusLimits { OpusLimits{input_bytes:2048,pages:8,packets:8,packet_bytes:100} }
    #[test] fn admits_complete_single_stream_with_final_trim(){
        let file=sample();let parsed=open_bounded_opus(&file,limits()).unwrap();
        assert_eq!(parsed.streams[0].packets.len(),1);assert_eq!(parsed.streams[0].last_granule,Some(900));
    }
    #[test] fn tags_require_bounded_complete_lengths(){
        let mut tags=b"OpusTags".to_vec();tags.extend_from_slice(&[0;8]);
        assert!(validate_tags(&tags).is_ok());
        let mut invalid=tags.clone();invalid[8..12].copy_from_slice(&100u32.to_le_bytes());assert!(validate_tags(&invalid).is_err());
        let mut invalid=tags.clone();invalid[12..16].copy_from_slice(&4097u32.to_le_bytes());assert!(validate_tags(&invalid).is_err());
        let mut invalid=tags.clone();invalid[12..16].copy_from_slice(&1u32.to_le_bytes());assert!(validate_tags(&invalid).is_err());
        let mut invalid=tags;invalid.push(1);assert!(validate_tags(&invalid).is_err());
    }
    #[test] fn refuses_corruption_sequence_limits_and_trailing_bytes(){
        let original=sample();let mut changed=original.clone();*changed.last_mut().unwrap()^=1;
        assert!(open_bounded_opus(&changed,limits()).is_err());
        assert!(open_bounded_opus(&original,OpusLimits{packet_bytes:3,..limits()}).is_err());
        let mut trailing=original.clone();trailing.push(0);assert!(open_bounded_opus(&trailing,limits()).is_err());
        let mut truncated=original.clone();truncated.pop();assert!(open_bounded_opus(&truncated,limits()).is_err());
        let mut bad=Vec::new();let mut head=b"OpusHead".to_vec();head.extend_from_slice(&[1,1,0,0]);head.extend_from_slice(&48_000u32.to_le_bytes());head.extend_from_slice(&[0,0,0]);
        page(&mut bad,2,0,0,&head);page(&mut bad,0,0,2,b"OpusTags");page(&mut bad,4,960,3,&[0xf8,0xff,0xfe]);
        assert!(open_bounded_opus(&bad,limits()).is_err());
    }
}
