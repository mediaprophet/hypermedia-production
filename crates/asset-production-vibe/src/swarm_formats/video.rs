//! AV1/VP9 packet remux and lossless structural broadcast indexing.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use chronicle_broadcast_containers::{
    KeyClass, MxfOpPattern, MxfPartition, MxfPartitionKind, MxfUl, ber_length, classify,
};
use media_isobmff::{
    Brand, CodecConfig, DemuxLimits, FragmentedWriter, Mp4File, Sample, Track, TrackConfig,
    TrackKind, WriteSample, WriterOptions, open_with_limits,
};
use serde_json::{Value as Json, json};
use sha2::{Digest, Sha256};

use super::{encode_json, receipt, result, sha256};
use crate::{
    Args, AssetHost, Value, fields, integer,
    qualia_containers_governance_poet_ops::encoded_resource, required,
};

const MAX_INPUT: usize = 64 * 1024 * 1024;
const MAX_PACKETS: usize = 8_192;
const MAX_FRAGMENTS: usize = 1_024;

impl AssetHost<'_> {
    pub(crate) fn transcode_av1_cmaf_mxf(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "target_fragment_samples"])?;
        let source = required(args, "source")?;
        let target = args
            .get("target_fragment_samples")
            .map(|value| integer(value, "target_fragment_samples"))
            .transpose()?
            .unwrap_or(48);
        if !(1..=512).contains(&target) {
            return Err("Fragment target must be within 1 to 512 samples".into());
        }
        let input = encoded_resource(self, source)?;
        if !matches!(input.kind.as_str(), "mp4" | "mov") {
            return Err("Provide a retained MP4 or MOV containing one AV1 or VP9 track".into());
        }
        let size = input.bytes.len();
        check_extent(size, self.limits.resource_bytes)?;
        self.charge(size.saturating_mul(14), size.saturating_mul(3))?;

        let bytes = encoded_resource(self, source)?.bytes.as_slice();
        let file = open_bmff(bytes)?;
        let [track] = file.tracks.as_slice() else {
            return Err("This video remux requires exactly one video track".into());
        };
        let [entry] = track.entries.as_slice() else {
            return Err("This video remux requires exactly one sample description".into());
        };
        let identity = [65536, 0, 0, 0, 65536, 0, 0, 0, 1073741824];
        if track.kind != TrackKind::Video
            || !matches!(&entry.codec, CodecConfig::Av1(_) | CodecConfig::Vp9(_))
            || track.timescale == 0
            || file.timescale == 0
            || !track.edits.is_empty()
            || track.matrix != identity
            || !entry.video.as_ref().is_some_and(|video| {
                track.width == u32::from(video.width) && track.height == u32::from(video.height)
            })
            || !track.samples.first().is_some_and(|sample| sample.is_sync)
        {
            return Err("This video remux requires an unrotated AV1 or VP9 track with no edit list and an initial sync sample".into());
        }
        let media = media_ranges(bytes)?;
        let mut opts = WriterOptions::new(Brand::Mp4);
        opts.movie_timescale = file.timescale;
        let mut writer = FragmentedWriter::new(Vec::new(), opts);
        let mut config = TrackConfig::new(entry.clone(), track.timescale);
        config.language = track.language.clone();
        let track_id = writer
            .add_track(config)
            .map_err(|error| error.to_string())?;
        let mut next_dts = 0i64;
        let mut samples_in_fragment = 0usize;
        let mut fragments = 0usize;
        let mut payload_bytes = 0usize;
        for sample in &track.samples {
            if sample.description_index != 1 || sample.duration == 0 || sample.dts != next_dts {
                return Err("Remux input must have one description and a contiguous decode timeline beginning at zero".into());
            }
            let data = sample_data(bytes, &media, sample)?;
            payload_bytes = payload_bytes
                .checked_add(data.len())
                .ok_or("Packet byte accounting overflow")?;
            if payload_bytes > self.limits.resource_bytes.min(MAX_INPUT) {
                return Err("Remux sample payloads exceed the resource budget".into());
            }
            if sample.is_sync && samples_in_fragment >= target {
                writer.flush_fragment().map_err(|error| error.to_string())?;
                samples_in_fragment = 0;
                fragments += 1;
            }
            if fragments >= MAX_FRAGMENTS || samples_in_fragment >= MAX_PACKETS {
                return Err("Video fragmentation exceeds v1 remux limits".into());
            }
            let composition_offset = sample
                .pts
                .checked_sub(sample.dts)
                .and_then(|value| i32::try_from(value).ok())
                .ok_or("Composition timestamp is out of range")?;
            writer
                .write_sample(
                    track_id,
                    WriteSample {
                        data,
                        duration: sample.duration,
                        composition_offset,
                        is_sync: sample.is_sync,
                    },
                )
                .map_err(|error| error.to_string())?;
            samples_in_fragment += 1;
            next_dts = next_dts
                .checked_add(i64::from(sample.duration))
                .ok_or("Video timing overflow")?;
        }
        let source_sha256 = sha256(bytes);
        let output = writer.finish().map_err(|error| error.to_string())?;
        check_extent(output.len(), self.limits.resource_bytes)?;
        let packet_timeline_sha256 =
            verify_fragmented_remux(bytes, &media, file.timescale, track, &output, fragments + 1)?;
        let output_sha256 = sha256(&output);
        let metadata = json!({
            "schema":"qualia-broadcast-remux/1",
            "codec":entry.codec.name(), "mode":"packet-preserving", "container":"fragmented_mp4",
            "track_samples":track.samples.len(), "fragments":fragments + 1, "timescale":track.timescale,
            "source_sha256":source_sha256, "output_sha256":output_sha256,
            "packet_timeline_sha256":packet_timeline_sha256
        });
        let video = self.encode("mp4", output)?;
        let report = encode_json(self, &metadata)?;
        let proof = receipt(
            self,
            "urn:qualia:broadcast:fragmented-mp4",
            &source_sha256,
            &output_sha256,
        )?;
        Ok(result(&[
            ("video", video),
            ("report", report),
            ("receipt", proof),
        ]))
    }

    pub(crate) fn demux_and_parse_mxf_cmaf(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source"])?;
        let source = required(args, "source")?;
        let input = encoded_resource(self, source)?;
        let size = input.bytes.len();
        check_extent(size, self.limits.resource_bytes)?;
        self.charge(
            size.saturating_mul(9),
            size.saturating_add(MAX_PACKETS * 512),
        )?;
        let input = encoded_resource(self, source)?;
        let sha = sha256(&input.bytes);
        let mut timeline = match input.kind.as_str() {
            "mp4" | "mov" => bmff_timeline(&input.bytes)?,
            "mxf" => mxf_timeline(&input.bytes)?,
            _ => return Err("Broadcast demux accepts retained MP4, MOV, or MXF".into()),
        };
        timeline["source_sha256"] = json!(sha);
        let timeline = serde_json::to_vec(&timeline).map_err(|error| error.to_string())?;
        let output_sha256 = sha256(&timeline);
        let handle = self.encode("json", timeline)?;
        let proof = receipt(
            self,
            "urn:qualia:broadcast:essence-timeline",
            &sha,
            &output_sha256,
        )?;
        Ok(result(&[("timeline", handle), ("receipt", proof)]))
    }
}

fn check_extent(input: usize, host_limit: usize) -> Result<(), String> {
    if !(16..=MAX_INPUT.min(host_limit)).contains(&input) {
        return Err("Broadcast container exceeds the bounded input extent".into());
    }
    Ok(())
}

fn open_bmff(bytes: &[u8]) -> Result<Mp4File, String> {
    open_with_limits(
        bytes,
        DemuxLimits {
            max_input_bytes: MAX_INPUT as u64,
            max_header_bytes: 8 * 1024 * 1024,
            max_tracks: 16,
            max_fragments: MAX_FRAGMENTS,
            max_table_entries: MAX_PACKETS,
            max_samples_per_track: MAX_PACKETS,
            require_self_contained_data: true,
            ..Default::default()
        },
    )
    .map_err(|error| error.to_string())
}

fn media_ranges(mut bytes: &[u8]) -> Result<Vec<std::ops::Range<usize>>, String> {
    let mut ranges = Vec::new();
    let mut position = 0usize;
    let mut boxes = 0usize;
    while !bytes.is_empty() {
        let header = bytes.get(..8).ok_or("Truncated BMFF box header")?;
        let mut header_len = 8;
        let length = match u32::from_be_bytes(header[..4].try_into().unwrap()) {
            0 => bytes.len(),
            1 => {
                header_len = 16;
                let ext = bytes
                    .get(8..16)
                    .ok_or("Truncated extended BMFF box length")?;
                usize::try_from(u64::from_be_bytes(ext.try_into().unwrap()))
                    .map_err(|_| "BMFF box length exceeds this address space")?
            }
            n => n as usize,
        };
        boxes += 1;
        if length < header_len || length > bytes.len() || boxes > MAX_PACKETS {
            return Err("Invalid or excessive top-level BMFF boxes".into());
        }
        if &header[4..8] == b"mdat" {
            ranges.push(position + header_len..position + length);
        }
        position += length;
        bytes = &bytes[length..];
    }
    if ranges.is_empty() {
        return Err("BMFF source has no media data".into());
    }
    Ok(ranges)
}

fn sample_data<'a>(
    bytes: &'a [u8],
    media: &[std::ops::Range<usize>],
    sample: &Sample,
) -> Result<&'a [u8], String> {
    let start = usize::try_from(sample.offset).map_err(|_| "BMFF packet offset is out of range")?;
    let end = start
        .checked_add(sample.size as usize)
        .ok_or("BMFF packet endpoint overflow")?;
    let after_start = media.partition_point(|span| span.start <= start);
    if sample.size == 0 || after_start == 0 || end > media[after_start - 1].end {
        return Err("BMFF sample extent is outside media data".into());
    }
    bytes
        .get(start..end)
        .ok_or_else(|| "BMFF packet extent exceeds source".into())
}

fn packet_timeline_digest(timescale: u32) -> Sha256 {
    let mut digest = Sha256::new();
    digest.update(b"qualia:broadcast:packet-timeline:v1\0");
    digest.update(timescale.to_le_bytes());
    digest
}

fn hash_packet(digest: &mut Sha256, sample: &Sample, packet: &[u8]) {
    digest.update(sample.dts.to_le_bytes());
    digest.update(sample.pts.to_le_bytes());
    digest.update(sample.duration.to_le_bytes());
    digest.update(sample.description_index.to_le_bytes());
    digest.update([u8::from(sample.is_sync)]);
    digest.update(sample.size.to_le_bytes());
    digest.update(packet);
}

fn verify_fragmented_remux(
    source: &[u8],
    source_media: &[std::ops::Range<usize>],
    movie_timescale: u32,
    source_track: &Track,
    output: &[u8],
    fragment_count: usize,
) -> Result<String, String> {
    let file = open_bmff(output)?;
    let [track] = file.tracks.as_slice() else {
        return Err("Remux readback does not contain exactly one track".into());
    };
    if !file.fragmented
        || file.fragment_count != fragment_count
        || file.timescale != movie_timescale
        || track.timescale != source_track.timescale
        || track.kind != source_track.kind
        || track.language != source_track.language
        || track.width != source_track.width
        || track.height != source_track.height
        || track.entries != source_track.entries
        || track.samples.len() != source_track.samples.len()
    {
        return Err("Remux changed the admitted track or fragment description".into());
    }
    let output_media = media_ranges(output)?;
    let mut digest = packet_timeline_digest(track.timescale);
    for (before, after) in source_track.samples.iter().zip(&track.samples) {
        let packet = sample_data(output, &output_media, after)?;
        if *after
            != (Sample {
                offset: after.offset,
                ..*before
            })
            || packet != sample_data(source, source_media, before)?
        {
            return Err("Remux changed an admitted media packet or timestamp".into());
        }
        hash_packet(&mut digest, after, packet);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn bmff_timeline(bytes: &[u8]) -> Result<Json, String> {
    let file = open_bmff(bytes)?;
    let media = media_ranges(bytes)?;
    let count = file
        .tracks
        .iter()
        .map(|track| track.samples.len())
        .sum::<usize>();
    if count == 0 || count > MAX_PACKETS {
        return Err("BMFF timeline has no samples or exceeds the packet budget".into());
    }
    let mut tracks = Vec::new();
    let mut payload_bytes = 0usize;
    for track in &file.tracks {
        if track.timescale == 0 {
            return Err("Zero BMFF media timescale".into());
        }
        let mut samples = Vec::with_capacity(track.samples.len());
        let mut digest = packet_timeline_digest(track.timescale);
        for sample in &track.samples {
            let packet = sample_data(bytes, &media, sample)?;
            payload_bytes = payload_bytes
                .checked_add(packet.len())
                .filter(|total| *total <= bytes.len())
                .ok_or("BMFF packet extents exceed the bounded source byte budget")?;
            if sample.duration == 0
                || sample.description_index == 0
                || sample.description_index as usize > track.entries.len()
            {
                return Err("BMFF timeline has an invalid sample duration or description".into());
            }
            hash_packet(&mut digest, sample, packet);
            samples.push(json!({
                "offset":sample.offset, "length":sample.size, "dts":sample.dts, "pts":sample.pts,
                "duration":sample.duration, "sync":sample.is_sync, "description":sample.description_index
            }));
        }
        let descriptions = track
            .entries
            .iter()
            .map(|entry| {
                json!({
                    "fourcc":String::from_utf8_lossy(&entry.format.0), "codec":entry.codec.name()
                })
            })
            .collect::<Vec<_>>();
        let edits = track
            .edits
            .iter()
            .map(|edit| {
                json!({
                    "segment_duration_in_movie_ticks":edit.segment_duration,
                    "media_time":edit.media_time, "media_rate_16_16":edit.media_rate
                })
            })
            .collect::<Vec<_>>();
        tracks.push(json!({
            "id":track.id, "kind":format!("{:?}", track.kind), "language":track.language,
            "timescale":track.timescale, "descriptions":descriptions, "edits":edits, "samples":samples,
            "packet_timeline_sha256":format!("{:x}", digest.finalize())
        }));
    }
    Ok(
        json!({"schema":"qualia-broadcast-timeline/1", "container":"isobmff",
        "fragmented":file.fragmented, "fragment_count":file.fragment_count,
        "movie_timescale":file.timescale, "tracks":tracks}),
    )
}

fn mxf_timeline(bytes: &[u8]) -> Result<Json, String> {
    let mut at = 0usize;
    let mut klvs = 0usize;
    let mut packets = Vec::new();
    let mut partitions = Vec::new();
    let mut op = MxfOpPattern::Unknown;
    while at < bytes.len() {
        klvs += 1;
        if klvs > MAX_PACKETS {
            return Err("MXF KLV packet budget exceeded".into());
        }
        let key_bytes = bytes
            .get(at..at + 16)
            .ok_or("Truncated MXF universal label")?;
        let key = MxfUl(key_bytes.try_into().unwrap());
        if !key.is_smpte() {
            return Err("Invalid SMPTE MXF KLV universal label".into());
        }
        let (length, ber_bytes) = ber_length(&bytes[at + 16..]).ok_or("Invalid MXF BER length")?;
        let start = at + 16 + ber_bytes;
        let end = usize::try_from(length)
            .ok()
            .and_then(|n| start.checked_add(n))
            .ok_or("MXF extent overflow")?;
        let payload = bytes
            .get(start..end)
            .ok_or("MXF KLV extent exceeds source")?;
        match classify(&key) {
            KeyClass::Partition { kind, status } => {
                if payload.len() < 88 || payload.len() > 88 + 256 * 16 {
                    return Err(
                        "MXF partition extent is outside its supported UL directory bounds".into(),
                    );
                }
                let n = u32::from_be_bytes(payload[80..84].try_into().unwrap()) as usize;
                let stride = u32::from_be_bytes(payload[84..88].try_into().unwrap());
                if n > 256 || (n != 0 && stride != 16) || payload.len() != 88 + n * 16 {
                    return Err("Invalid MXF essence container label directory".into());
                }
                let pack = MxfPartition::parse(at as u64, kind, status, payload)
                    .ok_or("Invalid MXF partition pack")?;
                if partitions.is_empty() {
                    if pack.kind != MxfPartitionKind::Header || at != 0 {
                        return Err("MXF decoder requires a header partition at offset zero".into());
                    }
                    op = MxfOpPattern::from_ul(&pack.operational_pattern);
                }
                if pack.this_partition != at as u64
                    || op == MxfOpPattern::Unknown
                    || MxfOpPattern::from_ul(&pack.operational_pattern) != op
                {
                    return Err(
                        "MXF partition offsets or operational patterns are inconsistent".into(),
                    );
                }
                partitions.push(json!({
                    "offset":pack.offset, "kind":format!("{:?}",pack.kind),
                    "body_sid":pack.body_sid,"index_sid":pack.index_sid
                }));
            }
            KeyClass::Essence => {
                if partitions.is_empty() || payload.is_empty() {
                    return Err("MXF essence must follow a partition and contain bytes".into());
                }
                packets.push(json!({
                    "ordinal":packets.len(), "key":key.to_string(), "offset":start, "length":length
                }));
            }
            _ => {}
        }
        at = end;
    }
    if packets.is_empty() {
        return Err("MXF source has no essence packets".into());
    }
    Ok(
        json!({"schema":"qualia-broadcast-timeline/1", "container":"mxf",
        "operational_pattern":format!("{op:?}"), "partitions":partitions, "essence_packets":packets}),
    )
}
