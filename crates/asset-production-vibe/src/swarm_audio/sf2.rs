//! Bounded SF2 PCM playback with MIDI and Scala timing and tuning.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use acoustics_daw_session::AudioBuffer;
use qualia_audio::midi::{
    smf::{build_tempo_map, read_smf, Division, MetaEvent, TrackEvent},
    tuning::{build_tuning_table, parse_scl, KbmMapping},
};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{
    count, fields, integer, list, num, number, record, required, Args, AssetHost,
    TypedArtifact, Value,
};

const MAX_BANK: usize = 32 << 20;
const MAX_MIDI: usize = 128 << 10;
const MAX_NOTES: usize = 4096;
const MAX_REGIONS: usize = 128;
const MAX_FRAMES: usize = 1_048_576;

#[derive(Clone, Copy)]
struct Note {
    start_s: f64,
    duration_s: f64,
    key: u8,
    velocity: u8,
}

#[derive(Clone, Copy)]
struct Region {
    start: usize,
    end: usize,
    loop_start: usize,
    loop_end: usize,
    loop_mode: u16,
    rate: f64,
    root: u8,
    tune_cents: f64,
    key_range: (u8, u8),
    velocity_range: (u8, u8),
    attenuation: f64,
    pan: f64,
    delay: f64,
    attack: f64,
    hold: f64,
    decay: f64,
    sustain: f64,
    release: f64,
}

#[derive(Clone, Copy)]
struct Zone {
    generators: [Option<u16>; 61],
    selector: Option<usize>,
}

impl Default for Zone {
    fn default() -> Self {
        Self {
            generators: [None; 61],
            selector: None,
        }
    }
}

impl Zone {
    fn merged(self, local: Self) -> Self {
        let mut result = self;
        for (destination, source) in result.generators.iter_mut().zip(local.generators) {
            if source.is_some() {
                *destination = source;
            }
        }
        result.selector = local.selector;
        result
    }

    fn signed(self, operator: usize, default: i16) -> i16 {
        self.generators[operator]
            .map(|value| value as i16)
            .unwrap_or(default)
    }

    fn range(self, operator: usize) -> (u8, u8) {
        self.generators[operator].map_or((0, 127), |bits| {
            let [low, high] = bits.to_le_bytes();
            (low, high)
        })
    }
}

impl AssetHost<'_> {
    pub(crate) fn synthesize_midi_sf2(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "soundfont",
                "note_events",
                "midi",
                "bank",
                "program",
                "sample_rate",
                "scala",
                "gain",
            ],
        )?;
        let bank = args
            .get("bank")
            .map(|v| integer(v, "bank"))
            .transpose()?
            .unwrap_or(0);
        let program = args
            .get("program")
            .map(|v| integer(v, "program"))
            .transpose()?
            .unwrap_or(0);
        if bank > 128 || program > 127 {
            return Err("SF2 bank or program is out of range".into());
        }
        let sample_rate = count(args, "sample_rate", 48_000, 96_000)? as u32;
        if sample_rate < 8_000 {
            return Err("SF2 output sample_rate must be at least 8000".into());
        }
        let gain = number(args, "gain", 0.5)?;
        if !(0.0..=1.0).contains(&gain) {
            return Err("SF2 gain must be within [0, 1]".into());
        }
        let soundfont = required(args, "soundfont")?;
        let source = super::retained(self, soundfont)?;
        if source.kind != "sf2" || source.bytes.len() > self.limits.resource_bytes.min(MAX_BANK) {
            return Err("Expected a bounded retained SF2 bank".into());
        }
        let bank_size = source.bytes.len();
        self.charge(bank_size * 2, 256 * MAX_REGIONS)?;

        let (notes, midi_digest) = match (args.get("note_events"), args.get("midi")) {
            (Some(events), None) => (parse_notes(events)?, None),
            (None, Some(value)) => {
                let midi = super::retained(self, value)?;
                if !matches!(midi.kind.as_str(), "mid" | "midi") || midi.bytes.len() > MAX_MIDI {
                    return Err("Expected bounded retained Standard MIDI File".into());
                }
                let bytes = midi.bytes.clone();
                self.charge(bytes.len() * 24, bytes.len() * 64)?;
                (
                    parse_midi(&bytes, bank as u16, program as u16)?,
                    Some(format!("{:x}", Sha256::digest(&bytes))),
                )
            }
            _ => return Err("Supply exactly one of note_events and midi".into()),
        };
        let (tuning, scala_digest) = if let Some(value) = args.get("scala") {
            let retained = super::retained(self, value)?;
            if retained.kind != "scl" || retained.bytes.len() > 32_768 {
                return Err("Expected bounded retained Scala scale".into());
            }
            let source =
                std::str::from_utf8(&retained.bytes).map_err(|_| "Scala scale must be UTF-8")?;
            let scale = parse_scl(source).map_err(|e| format!("Invalid Scala scale: {e:?}"))?;
            if !(1..=128).contains(&scale.count()) {
                return Err("Unexpected Scala scale extent".into());
            }
            let map = KbmMapping::linear(scale.count(), 60, 69, 440.0);
            (
                build_tuning_table(&scale, &map),
                Some(format!("{:x}", Sha256::digest(&retained.bytes))),
            )
        } else {
            (
                std::array::from_fn(|key| 440.0 * 2.0f64.powf((key as f64 - 69.0) / 12.0)),
                None,
            )
        };
        let bank_data = super::retained(self, soundfont)?;
        let (samples, regions) = parse_soundfont(&bank_data.bytes, bank as u16, program as u16)?;
        let digest = format!("{:x}", Sha256::digest(&bank_data.bytes));
        let mut work = 0usize;
        let mut frames = 0usize;
        for note in &notes {
            let mut matched = false;
            for region in regions.iter().filter(|region| accepts(region, note)) {
                matched = true;
                sample_step(region, &tuning, note, sample_rate)?;
                let (start, held, tail) = render_extent(note, region, sample_rate)?;
                frames = frames.max(start + held + tail);
                work = work
                    .checked_add((held + tail) * 24)
                    .ok_or("SF2 render work overflow")?;
            }
            if !matched {
                return Err(format!(
                    "SF2 preset does not map MIDI note {} and its velocity",
                    note.key
                ));
            }
        }
        if frames > MAX_FRAMES || frames == 0 || frames * 8 > self.limits.resource_bytes {
            return Err("SF2 render exceeds the output duration or memory bound".into());
        }
        let sample_bytes = samples.len();
        self.charge(work, frames * 8 + sample_bytes)?;
        // The bank and its offsets are immutable between planning and rendering.
        let bank_data = super::retained(self, soundfont)?;
        let (samples, _) = parse_soundfont(&bank_data.bytes, bank as u16, program as u16)?;
        let mut output = AudioBuffer::new(sample_rate, 2, frames);
        for note in &notes {
            for region in regions.iter().filter(|region| accepts(region, note)) {
                render_voice(&mut output, samples, *region, note, &tuning, gain)?;
            }
        }
        let peak = output
            .channels
            .iter()
            .flatten()
            .map(|sample| sample.abs())
            .fold(0.0f32, f32::max);
        if !peak.is_finite() || f64::from(peak) > 1.0 {
            return Err("SF2 mix exceeded digital full scale; lower the explicit gain".into());
        }
        let receipt = json!({
            "render": "sf2_pcm16_polyphonic_linear_interpolation",
            "sf2_sha256": digest, "midi_sha256": midi_digest, "scala_sha256": scala_digest,
            "bank": bank, "program": program, "sample_rate": sample_rate,
            "frames": frames, "peak": peak, "notes": notes.len(), "regions": regions.len(),
        });
        let audio = self.admit_artifact(TypedArtifact::AudioMultiChannel(Arc::new(output)))?;
        let receipt = self.encode(
            "json",
            serde_json::to_vec(&receipt).map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record(Args::from([
            ("audio".into(), audio),
            ("receipt".into(), receipt),
        ])))
    }
}

fn parse_notes(value: &Value) -> Result<Vec<Note>, String> {
    let rows = list(value)?;
    if !(1..=MAX_NOTES).contains(&rows.len()) {
        return Err("Invalid number of SF2 note events".into());
    }
    rows.iter()
        .map(|row| {
            let row = record(row)?;
            fields(row, &["start_s", "duration_s", "key", "velocity"])?;
            note(
                num(required(row, "start_s")?)?,
                num(required(row, "duration_s")?)?,
                integer(required(row, "key")?, "key")?,
                integer(required(row, "velocity")?, "velocity")?,
            )
        })
        .collect()
}

fn note(start_s: f64, duration_s: f64, key: usize, velocity: usize) -> Result<Note, String> {
    if key > 127
        || !(1..=127).contains(&velocity)
        || !(0.0..=130.0).contains(&start_s)
        || !(0.0001..=130.0).contains(&duration_s)
        || start_s + duration_s > 130.0
    {
        return Err("SF2 note time, key or velocity exceeds the admitted range".into());
    }
    Ok(Note {
        start_s,
        duration_s,
        key: key as u8,
        velocity: velocity as u8,
    })
}

fn parse_midi(bytes: &[u8], bank: u16, program: u16) -> Result<Vec<Note>, String> {
    if bytes.len() < 14 || u16::from_be_bytes([bytes[10], bytes[11]]) > 32 {
        return Err("MIDI needs a valid header and at most 32 tracks".into());
    }
    let tracks = u16::from_be_bytes([bytes[10], bytes[11]]);
    let mut rest = &bytes[14..];
    for _ in 0..tracks {
        let header = rest.get(..8).ok_or("Truncated MIDI track header")?;
        let len = u32::from_be_bytes(header[4..8].try_into().unwrap()) as usize;
        rest = rest
            .get(
                8usize
                    .checked_add(len)
                    .ok_or("MIDI track extent overflow")?..,
            )
            .ok_or("Truncated MIDI track")?;
    }
    if !rest.is_empty() {
        return Err("MIDI has unexpected bytes following its declared tracks".into());
    }
    let file = read_smf(bytes).map_err(|error| format!("Native SMF parse failed: {error:?}"))?;
    if !matches!(file.division, Division::Ppq(1..=32767))
        || file.format > 1
        || file.tracks.is_empty()
        || (file.format == 0 && file.tracks.len() != 1)
    {
        return Err("SF2 playback admits metrical MIDI formats 0 and 1".into());
    }
    let mut merged = Vec::new();
    let mut tempo_count = 0;
    for track in &file.tracks {
        let mut tick = 0u64;
        let mut terminated = false;
        for event in &track.events {
            if terminated {
                return Err("MIDI track contains events after its terminator".into());
            }
            tick = tick
                .checked_add(u64::from(event.delta_ticks))
                .ok_or("MIDI tick overflow")?;
            match &event.event {
                TrackEvent::Midi { status, data } => {
                    if merged.len() >= MAX_NOTES * 4 || data.iter().any(|byte| *byte >= 128) {
                        return Err("Invalid or oversized MIDI channel event sequence".into());
                    }
                    merged.push((tick, *status, data.as_slice()));
                }
                TrackEvent::Meta(MetaEvent::Tempo(0)) => {
                    return Err("MIDI tempo cannot be zero".into())
                }
                TrackEvent::Meta(MetaEvent::Tempo(_)) => tempo_count += 1,
                TrackEvent::Meta(MetaEvent::EndOfTrack) => terminated = true,
                TrackEvent::Meta(_) => {}
                TrackEvent::SysEx { .. } => {
                    return Err("MIDI SysEx playback is outside the SF2 profile".into())
                }
            }
        }
        if !terminated {
            return Err("MIDI tracks require an end-of-track marker".into());
        }
    }
    if tempo_count > 128 {
        return Err("MIDI has too many tempo changes for this profile".into());
    }
    let tempo = build_tempo_map(&file);
    merged.sort_by_key(|event| event.0);
    let mut starts: BTreeMap<(u8, u8), VecDeque<(u64, u8)>> = BTreeMap::new();
    let mut notes = Vec::new();
    for (tick, status, data) in merged {
        let channel = status & 15;
        if channel == 9 && bank != 128 && matches!(status & 0xf0, 0x80 | 0x90) {
            return Err("MIDI channel 10 requires an explicitly selected percussion bank".into());
        }
        match (status & 0xf0, data) {
            (0x90, [key, velocity]) if *velocity != 0 => starts
                .entry((channel, *key))
                .or_default()
                .push_back((tick, *velocity)),
            (0x80, [key, _]) | (0x90, [key, 0]) => {
                let (start, velocity) = starts
                    .get_mut(&(channel, *key))
                    .and_then(VecDeque::pop_front)
                    .ok_or("MIDI note-off lacks a matching note-on")?;
                let start_s = tempo.ticks_to_seconds(start);
                notes.push(note(
                    start_s,
                    tempo.ticks_to_seconds(tick) - start_s,
                    usize::from(*key),
                    usize::from(velocity),
                )?);
                if notes.len() > MAX_NOTES {
                    return Err("MIDI exceeds the note budget".into());
                }
            }
            (0xc0, [value]) if u16::from(*value) == program => {}
            (0xb0, [0, value]) if u16::from(*value) == bank >> 7 => {}
            (0xb0, [32, value]) if u16::from(*value) == bank & 127 => {}
            _ => {
                return Err(
                    "MIDI controller or patch change is outside single-preset playback".into(),
                )
            }
        }
    }
    if notes.is_empty() || starts.values().any(|starts| !starts.is_empty()) {
        return Err(
            "MIDI needs at least one note and all onsets must have matching note-offs".into(),
        );
    }
    Ok(notes)
}

fn parse_soundfont(bytes: &[u8], bank: u16, program: u16) -> Result<(&[u8], Vec<Region>), String> {
    if bytes.len() < 12
        || &bytes[..4] != b"RIFF"
        || &bytes[8..12] != b"sfbk"
        || (le32(bytes, 4) as usize).checked_add(8) != Some(bytes.len())
    {
        return Err("Malformed SF2 RIFF header or extent".into());
    }
    let info = chunk(&bytes[12..], b"INFO", true)?.ok_or("Missing SF2 INFO list")?;
    let version = chunk(info, b"ifil", false)?.ok_or("Missing SF2 version")?;
    if version.len() != 4 || le16(version, 0) != 2 || le16(version, 2) > 4 {
        return Err("This adapter supports SoundFont 2.x PCM files".into());
    }
    let sample_list = chunk(&bytes[12..], b"sdta", true)?.ok_or("Missing SF2 sample data")?;
    let samples = chunk(sample_list, b"smpl", false)?.ok_or("Missing SF2 PCM samples")?;
    if samples.is_empty() || samples.len() % 2 != 0 || chunk(sample_list, b"sm24", false)?.is_some()
    {
        return Err("Expected SF2 PCM16 without a separate low-byte plane".into());
    }
    let hydra = chunk(&bytes[12..], b"pdta", true)?.ok_or("Missing SF2 preset data")?;
    let presets = table(hydra, b"phdr", 38, 2)?;
    let preset_bags = table(hydra, b"pbag", 4, 2)?;
    let preset_generators = table(hydra, b"pgen", 4, 2)?;
    let preset_modulators = table(hydra, b"pmod", 10, 1)?;
    let instruments = table(hydra, b"inst", 22, 2)?;
    let instrument_bags = table(hydra, b"ibag", 4, 2)?;
    let instrument_generators = table(hydra, b"igen", 4, 2)?;
    let instrument_modulators = table(hydra, b"imod", 10, 1)?;
    let headers = table(hydra, b"shdr", 46, 2)?;
    let mut matching = presets[..presets.len() - 38]
        .chunks_exact(38)
        .enumerate()
        .filter(|(_, preset)| le16(preset, 20) == program && le16(preset, 22) == bank);
    let (index, _) = matching.next().ok_or("SF2 bank/preset was not found")?;
    if matching.next().is_some() {
        return Err("Ambiguous SF2 bank/preset address".into());
    }
    let preset_zones = zones(
        le16(presets, index * 38 + 24),
        le16(presets, (index + 1) * 38 + 24),
        preset_bags,
        preset_generators,
        preset_modulators.len() / 10,
        41,
    )?;
    let mut regions = Vec::new();
    for preset in preset_zones {
        let instrument = preset
            .selector
            .ok_or("Unresolved SF2 instrument selector")?;
        let current = instruments
            .get(instrument * 22..instrument * 22 + 44)
            .ok_or("Invalid SF2 instrument index")?;
        for zone in zones(
            le16(current, 20),
            le16(current, 42),
            instrument_bags,
            instrument_generators,
            instrument_modulators.len() / 10,
            53,
        )? {
            if regions.len() == MAX_REGIONS {
                return Err("SF2 preset exceeds the mapped region budget".into());
            }
            let index = zone.selector.ok_or("Unresolved SF2 sample selector")?;
            let sample = headers
                .get(index * 46..index * 46 + 92)
                .ok_or("Invalid SF2 sample index")?;
            if let Some(region) = region(preset, zone, sample, samples.len() / 2)? {
                regions.push(region);
            }
        }
    }
    if regions.is_empty() {
        return Err("SF2 preset has no playable regions".into());
    }
    Ok((samples, regions))
}

fn chunk<'a>(
    mut list: &'a [u8],
    tag: &[u8; 4],
    list_form: bool,
) -> Result<Option<&'a [u8]>, String> {
    let mut found = None;
    while !list.is_empty() {
        let header = list.get(..8).ok_or("Truncated SF2 chunk header")?;
        let len = le32(header, 4) as usize;
        let end = 8usize.checked_add(len).ok_or("SF2 chunk extent overflow")?;
        let data = list.get(8..end).ok_or("Truncated SF2 chunk")?;
        let matches = if list_form {
            &header[..4] == b"LIST" && data.get(..4) == Some(&tag[..])
        } else {
            &header[..4] == tag
        };
        if matches
            && found
                .replace(if list_form { &data[4..] } else { data })
                .is_some()
        {
            return Err("Duplicate SF2 chunk".into());
        }
        list = list
            .get(end.checked_add(len % 2).ok_or("SF2 padding overflow")?..)
            .ok_or("Missing SF2 RIFF padding")?;
    }
    Ok(found)
}

fn table<'a>(
    hydra: &'a [u8],
    tag: &[u8; 4],
    stride: usize,
    minimum: usize,
) -> Result<&'a [u8], String> {
    let bytes = chunk(hydra, tag, false)?.ok_or("Missing SF2 hydra table")?;
    if bytes.len() % stride != 0 || !(minimum..=65_536).contains(&(bytes.len() / stride)) {
        return Err("Malformed or oversized SF2 hydra table".into());
    }
    Ok(bytes)
}

fn zones(
    start: u16,
    end: u16,
    bags: &[u8],
    generators: &[u8],
    mod_count: usize,
    selector: u16,
) -> Result<Vec<Zone>, String> {
    let (start, end) = (usize::from(start), usize::from(end));
    if end <= start || end >= bags.len() / 4 || end - start > MAX_REGIONS {
        return Err("Invalid SF2 bag range".into());
    }
    let mut result = Vec::new();
    let mut global = Zone::default();
    for i in start..end {
        let first = usize::from(le16(bags, i * 4));
        let terminator = usize::from(le16(bags, (i + 1) * 4));
        let mods = usize::from(le16(bags, i * 4 + 2));
        if mods >= mod_count || mods != usize::from(le16(bags, (i + 1) * 4 + 2)) {
            return Err(
                "Custom SoundFont modulators are outside the bounded playback profile".into(),
            );
        }
        if first >= terminator || terminator >= generators.len() / 4 {
            return Err("Invalid SF2 generator range".into());
        }
        let mut zone = Zone::default();
        for j in first..terminator {
            let operator = le16(generators, j * 4);
            let value = le16(generators, j * 4 + 2);
            if operator == selector {
                if j + 1 != terminator {
                    return Err("SF2 selector must terminate its zone".into());
                }
                zone.selector = Some(usize::from(value));
                continue;
            }
            if !matches!(operator, 17 | 33..=38 | 43 | 44 | 48 | 51 | 52)
                && !(selector == 53 && matches!(operator, 54 | 58))
            {
                return Err(format!(
                    "SF2 generator {operator} is outside the bounded playback profile"
                ));
            }
            if matches!(operator, 43 | 44) {
                let [lo, hi] = value.to_le_bytes();
                if lo > hi || hi > 127 {
                    return Err("Invalid SF2 key/velocity range".into());
                }
                if (operator == 43 && j != first)
                    || (operator == 44
                        && j != first
                        && !(j == first + 1 && zone.generators[43].is_some()))
                {
                    return Err("SF2 key and velocity ranges must precede other generators".into());
                }
            }
            if zone.generators[operator as usize].replace(value).is_some() {
                return Err("Duplicate generator in SF2 zone".into());
            }
        }
        if zone.selector.is_some() {
            result.push(global.merged(zone));
        } else if i == start && zone.generators[54].is_none() {
            global = zone;
        } else {
            return Err("Only the first SF2 zone can be global".into());
        }
    }
    if result.is_empty() {
        return Err("SF2 preset/instrument has no local zones".into());
    }
    Ok(result)
}

fn region(
    preset: Zone,
    zone: Zone,
    sample: &[u8],
    extent: usize,
) -> Result<Option<Region>, String> {
    let key_range = intersect(preset.range(43), zone.range(43));
    let velocity_range = intersect(preset.range(44), zone.range(44));
    if key_range.0 > key_range.1 || velocity_range.0 > velocity_range.1 {
        return Ok(None);
    }
    let start = le32(sample, 20) as usize;
    let end = le32(sample, 24) as usize;
    let loop_start = le32(sample, 28) as usize;
    let loop_end = le32(sample, 32) as usize;
    let rate = f64::from(le32(sample, 36));
    let original_pitch = sample[40].min(128);
    let root = zone.signed(
        58,
        if original_pitch <= 127 {
            i16::from(original_pitch)
        } else {
            60
        },
    );
    let loop_mode = zone.generators[54].unwrap_or(0);
    if le16(sample, 44) != 1
        || !(400..=96_000).contains(&(rate as u32))
        || !(0..=127).contains(&root)
        || end > extent
        || start >= end
        || !matches!(loop_mode, 0 | 1 | 3)
        || (loop_mode != 0
            && !(start <= loop_start && loop_end.saturating_sub(loop_start) > 1 && loop_end <= end))
    {
        return Err("Invalid or unsupported SF2 mono sample or loop descriptor".into());
    }
    let total = |operator, default| {
        i32::from(zone.signed(operator, default)) + i32::from(preset.signed(operator, 0))
    };
    let timing = |operator| {
        let value = total(operator, -12000);
        if value <= -12000 {
            0.0
        } else {
            2.0f64.powf(f64::from(value.clamp(-12000, 8000)) / 1200.0)
        }
    };
    Ok(Some(Region {
        start,
        end,
        loop_start,
        loop_end,
        loop_mode,
        rate,
        root: root as u8,
        tune_cents: f64::from(total(51, 0) * 100 + total(52, 0) + i32::from(sample[41] as i8)),
        key_range,
        velocity_range,
        pan: f64::from(total(17, 0).clamp(-500, 500)) / 500.0,
        attenuation: 10.0f64.powf(-f64::from(total(48, 0).clamp(0, 1440)) / 200.0),
        delay: timing(33),
        attack: timing(34),
        hold: timing(35),
        decay: timing(36),
        sustain: 10.0f64.powf(-f64::from(total(37, 0).clamp(0, 1440)) / 200.0),
        release: timing(38).max(0.005),
    }))
}

fn intersect(a: (u8, u8), b: (u8, u8)) -> (u8, u8) {
    (a.0.max(b.0), a.1.min(b.1))
}

fn accepts(region: &Region, note: &Note) -> bool {
    (region.key_range.0..=region.key_range.1).contains(&note.key)
        && (region.velocity_range.0..=region.velocity_range.1).contains(&note.velocity)
}

fn sample_step(
    region: &Region,
    tuning: &[f64; 128],
    note: &Note,
    rate: u32,
) -> Result<f64, String> {
    let root_hz = 440.0 * 2.0f64.powf((f64::from(region.root) - 69.0) / 12.0);
    let step = region.rate * tuning[usize::from(note.key)] / (f64::from(rate) * root_hz)
        * 2.0f64.powf(region.tune_cents / 1200.0);
    if !(0.01..=16.0).contains(&step) {
        return Err("SF2 playback ratio exceeds the admitted range".into());
    }
    Ok(step)
}

fn render_extent(note: &Note, region: &Region, rate: u32) -> Result<(usize, usize, usize), String> {
    let rate = f64::from(rate);
    let start = (note.start_s * rate).round() as usize;
    let held = (note.duration_s * rate).round() as usize;
    let tail = (region.release * rate).ceil() as usize;
    if start + held + tail > MAX_FRAMES || held == 0 {
        return Err("SF2 note exceeds the output duration budget".into());
    }
    Ok((start, held, tail))
}

fn envelope(region: Region, t: f64) -> f64 {
    let t = t - region.delay;
    if t < 0.0 {
        return 0.0;
    }
    if t < region.attack {
        return t / region.attack;
    }
    let t = t - region.attack - region.hold;
    if t <= 0.0 {
        return 1.0;
    }
    if t < region.decay {
        return region.sustain.powf(t / region.decay);
    }
    region.sustain
}

fn render_voice(
    output: &mut AudioBuffer,
    samples: &[u8],
    region: Region,
    note: &Note,
    tuning: &[f64; 128],
    gain: f64,
) -> Result<(), String> {
    let step = sample_step(&region, tuning, note, output.sample_rate)?;
    let (start, held, tail) = render_extent(note, &region, output.sample_rate)?;
    let rate = f64::from(output.sample_rate);
    let release_start = envelope(region, held as f64 / rate);
    let velocity = (f64::from(note.velocity) / 127.0).powi(2);
    let pan = (region.pan + 1.0) * std::f64::consts::FRAC_PI_4;
    let gains = [pan.cos(), pan.sin()].map(|value| value * gain * velocity * region.attenuation);
    let read = |frame: usize| f64::from(le16(samples, frame * 2) as i16) / 32768.0;
    let mut cursor = region.start as f64;
    for index in 0..held + tail {
        let looping = region.loop_mode == 1 || (region.loop_mode == 3 && index < held);
        if looping && cursor >= region.loop_end as f64 {
            cursor = region.loop_start as f64
                + (cursor - region.loop_start as f64)
                    % (region.loop_end - region.loop_start) as f64;
        }
        let integer = cursor as usize;
        if integer >= region.end {
            break;
        }
        let next = if looping && integer + 1 == region.loop_end {
            region.loop_start
        } else {
            (integer + 1).min(region.end - 1)
        };
        let fraction = cursor - integer as f64;
        let sample = read(integer) * (1.0 - fraction) + fraction * read(next);
        let weight = if index < held {
            envelope(region, index as f64 / rate)
        } else {
            release_start * (1.0 - (index - held) as f64 / tail as f64)
        };
        for (channel, gain) in output.channels.iter_mut().zip(gains) {
            channel[start + index] += (sample * weight * gain) as f32;
            if !channel[start + index].is_finite() {
                return Err("SF2 mix overflow".into());
            }
        }
        cursor += step;
    }
    Ok(())
}

fn le16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap())
}
fn le32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}
