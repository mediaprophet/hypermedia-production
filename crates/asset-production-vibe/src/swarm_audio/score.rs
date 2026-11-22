//! Exact single-layer MusicXML 4.0 and MEI 5.1 interchange.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeMap;

use chronicle_office_codecs::xml::{parse_xml, XEl, XNode};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{fields, required, Args, AssetHost, Value};

const MAX_XML_BYTES: usize = 512 * 1024;
const TICKS: u32 = 256;
const MEI_NS: &str = "http://www.music-encoding.org/ns/mei";
const SCORE_LENGTHS: &[&str] = &[
    "whole", "half", "quarter", "eighth", "16th", "32nd", "64th", "128th",
];

#[derive(Clone, Copy, PartialEq, Eq)]
struct Duration {
    denominator: u32,
    dots: u32,
}

impl Duration {
    fn checked(denominator: u32, dots: u32) -> Result<Self, String> {
        if denominator == 0 || denominator > 128 || !denominator.is_power_of_two() || dots > 3 {
            return Err(
                "Score requires a binary duration from whole through 128th with at most three dots"
                    .into(),
            );
        }
        Ok(Self { denominator, dots })
    }

    fn ticks(self) -> u32 {
        let base = TICKS * 4 / self.denominator;
        base * 2 - (base >> self.dots)
    }

    fn from_ticks(ticks: u32) -> Result<Self, String> {
        (0..8)
            .flat_map(|power| {
                (0..4).map(move |dots| Self {
                    denominator: 1 << power,
                    dots,
                })
            })
            .find(|duration| duration.ticks() == ticks)
            .ok_or_else(|| {
                "MusicXML duration is outside the exact binary and dotted interchange profile"
                    .into()
            })
    }

    fn musicxml_type(self) -> &'static str {
        SCORE_LENGTHS[self.denominator.trailing_zeros() as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pitch {
    step: u8,
    octave: u8,
    alter: i8,
}

struct Event {
    duration: Duration,
    pitches: Vec<Pitch>,
}

struct Measure {
    number: String,
    events: Vec<Event>,
}

struct Score {
    title: String,
    part_name: String,
    count: u32,
    unit: u32,
    tempo: Option<String>,
    measures: Vec<Measure>,
}

impl AssetHost<'_> {
    pub(crate) fn transcode_musicxml_and_mei_scores(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["source"])?;
        let source = required(args, "source")?;
        let resource = super::retained(self, source)?;
        let size = resource.bytes.len();
        if size > MAX_XML_BYTES || !matches!(resource.kind.as_str(), "musicxml" | "mei") {
            return Err(
                "Score source requires a MusicXML or MEI resource of at most 512 KiB".into(),
            );
        }
        self.charge(size * 120, size * 24 + 512 * 1024)?;
        let resource = super::retained(self, source)?;
        let input_text =
            std::str::from_utf8(&resource.bytes).map_err(|_| "Score XML must be UTF-8")?;
        let body = preflight(input_text)?;
        let root = parse_xml(body).map_err(|e| e.to_string())?;
        let input_sha = format!("{:x}", Sha256::digest(&resource.bytes));
        let source_kind = resource.kind.clone();
        let (score, kind) = match source_kind.as_str() {
            "musicxml" => (read_musicxml(&root)?, "mei"),
            "mei" => (read_mei(&root)?, "musicxml"),
            _ => unreachable!(),
        };
        let mut event_count = 0;
        let mut pitch_count = 0;
        let measure_ticks = score.count * TICKS * 4 / score.unit;
        if score.measures.is_empty() || score.measures.len() > 512 {
            return Err("Score requires 1 to 512 measures".into());
        }
        for measure in &score.measures {
            event_count += measure.events.len();
            for event in &measure.events {
                pitch_count += event.pitches.len();
            }
            if measure
                .events
                .iter()
                .map(|event| event.duration.ticks())
                .sum::<u32>()
                != measure_ticks
            {
                return Err(format!(
                    "Measure {} does not fill the declared meter exactly",
                    measure.number
                ));
            }
        }
        if event_count > 8192 || pitch_count > 8192 {
            return Err("Score exceeds its event or pitch budget".into());
        }
        let xml = if kind == "mei" {
            write_mei(&score)
        } else {
            write_musicxml(&score)
        };
        let xml = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}",
            xml.to_xml_string()
        );
        if xml.len() > MAX_XML_BYTES || preflight(&xml).is_err() {
            return Err("Converted XML exceeds the reciprocal score interchange budget".into());
        }
        let bytes = xml.into_bytes();
        let sha = format!("{:x}", Sha256::digest(&bytes));
        let proof = self.score_receipt(&input_sha, &sha, kind)?;
        let output = self.encode(kind, bytes)?;
        let q42 = self.encode("q42", proof)?;
        let receipt = self.encode(
            "json",
            serde_json::to_vec(&json!({
                "source_kind": source_kind,
                "output_kind": kind,
                "source_sha256": input_sha,
                "output_sha256": sha,
                "measures": score.measures.len(),
                "events": event_count,
                "pitches": pitch_count,
                "ticks_per_quarter": TICKS,
                "quarter_notes_per_minute": score.tempo,
            }))
            .map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record(
            [
                ("score".into(), output),
                ("q42".into(), q42),
                ("receipt".into(), receipt),
            ]
            .into(),
        ))
    }

    fn score_receipt(&self, input: &str, output: &str, kind: &str) -> Result<Vec<u8>, String> {
        let subject = format!("urn:sha256:{output}");
        let notation = if kind == "mei" {
            "mei-5.1"
        } else {
            "musicxml-4.0"
        };
        let mut graph = ReceiptGraph::new("urn:qualia:interchange:score-receipt");
        graph.fact(
            &subject,
            "http://www.w3.org/ns/prov#wasDerivedFrom",
            &format!("urn:sha256:{input}"),
        )?;
        graph.fact(
            &subject,
            "http://purl.org/dc/terms/format",
            &format!("urn:qualia:notation:{notation}"),
        )?;
        graph.seal(self.limits.resource_bytes)
    }
}

fn preflight(source: &str) -> Result<&str, String> {
    if source.chars().any(|ch| {
        ch < ' ' && !matches!(ch, '\t' | '\r' | '\n') || matches!(ch, '\u{fffe}' | '\u{ffff}')
    }) {
        return Err("Score XML contains invalid XML 1.0 characters".into());
    }
    let mut body = source.trim_start_matches('\u{feff}').trim();
    if body.starts_with("<?xml ") {
        let (declaration, remainder) = body
            .split_once("?>")
            .ok_or("An XML declaration must end with ?>")?;
        if !matches!(
            declaration,
            "<?xml version=\"1.0\""
                | "<?xml version='1.0'"
                | "<?xml version=\"1.0\" encoding=\"UTF-8\""
                | "<?xml version='1.0' encoding='UTF-8'"
        ) {
            return Err("Score interchange accepts XML 1.0 with a UTF-8 declaration".into());
        }
        body = remainder.trim_start();
    }
    for (index, _) in body.match_indices('&') {
        if !["&amp;", "&lt;", "&gt;", "&quot;", "&apos;"]
            .iter()
            .any(|entity| body[index..].starts_with(*entity))
        {
            return Err("Score XML uses an unsupported entity".into());
        }
    }
    let mut cursor = body;
    let mut depth = 0usize;
    let mut roots = 0usize;
    let mut tags = 0usize;
    while let Some((before, tag)) = cursor.split_once('<') {
        if (depth == 0 && !before.trim().is_empty()) || tag.starts_with(['!', '?']) {
            return Err("Score XML contains markup outside the bounded document profile".into());
        }
        let mut quote = None;
        let end = tag
            .bytes()
            .position(|byte| {
                match (quote, byte) {
                    (None, b'\'' | b'"') => quote = Some(byte),
                    (Some(expected), found) if expected == found => quote = None,
                    (None, b'>') => return true,
                    _ => {}
                }
                false
            })
            .ok_or("Unclosed XML tag")?;
        if tag[..end].contains('<') {
            return Err("XML tag contains an unescaped opening delimiter".into());
        }
        if tag.starts_with('/') {
            depth = depth.checked_sub(1).ok_or("Unexpected XML closing tag")?;
        } else {
            if depth == 0 {
                roots += 1;
            }
            if !tag[..end].ends_with('/') {
                depth += 1;
            }
        }
        tags += 1;
        if depth > 32 || roots > 1 || tags > 45_000 {
            return Err("XML document exceeds the depth, root or tag bound".into());
        }
        cursor = &tag[end + 1..];
    }
    if depth != 0 || roots != 1 || !cursor.trim().is_empty() {
        return Err("Score XML requires one complete root element".into());
    }
    Ok(body)
}

fn elements(parent: &XEl) -> impl Iterator<Item = &XEl> {
    parent.children.iter().filter_map(|node| match node {
        XNode::Element(child) => Some(child),
        XNode::Text(_) => None,
    })
}

fn children_allowed(parent: &XEl, allowed: &[&str]) -> Result<(), String> {
    for node in &parent.children {
        match node {
            XNode::Element(child) if !allowed.contains(&child.name.as_str()) => {
                return Err(format!("Unsupported {} child {}", parent.name, child.name))
            }
            XNode::Text(text) if !text.trim().is_empty() => {
                return Err(format!("Unexpected text within {}", parent.name))
            }
            _ => {}
        }
    }
    Ok(())
}

fn attrs_allowed(parent: &XEl, allowed: &[&str]) -> Result<(), String> {
    for (index, (key, _)) in parent.attrs.iter().enumerate() {
        if !allowed.contains(&key.as_str())
            || parent.attrs[..index].iter().any(|(prior, _)| prior == key)
        {
            return Err(format!(
                "Unsupported or duplicate {} attribute {key}",
                parent.name
            ));
        }
    }
    Ok(())
}

fn optional<'a>(parent: &'a XEl, name: &str) -> Result<Option<&'a XEl>, String> {
    let mut matches = elements(parent).filter(|child| child.name == name);
    let first = matches.next();
    if matches.next().is_some() {
        return Err(format!("{} requires at most one {name}", parent.name));
    }
    Ok(first)
}

fn one<'a>(parent: &'a XEl, name: &str) -> Result<&'a XEl, String> {
    optional(parent, name)?.ok_or_else(|| format!("{} requires {name}", parent.name))
}

fn content(element: &XEl) -> Result<String, String> {
    attrs_allowed(element, &[])?;
    if elements(element).next().is_some() {
        return Err(format!("{} requires plain text", element.name));
    }
    Ok(element.text_content().trim().to_owned())
}

fn attr<'a>(parent: &'a XEl, name: &str) -> Result<&'a str, String> {
    parent
        .attr(name)
        .ok_or_else(|| format!("{} requires {name}", parent.name))
}

fn integer(input: &str) -> Result<u32, String> {
    input
        .parse()
        .map_err(|_| format!("Expected an unsigned score integer, found {input:?}"))
}

fn meter(count: u32, unit: u32) -> Result<(u32, u32), String> {
    if !(1..=32).contains(&count) || unit == 0 || unit > 32 || !unit.is_power_of_two() {
        return Err("Score requires a fixed binary meter with 1 to 32 beats".into());
    }
    Ok((count, unit))
}

fn tempo(input: &str) -> Result<String, String> {
    if input.len() > 12
        || !input
            .bytes()
            .all(|value| value.is_ascii_digit() || value == b'.')
        || !input
            .parse::<f64>()
            .is_ok_and(|value| (10.0..=400.0).contains(&value))
    {
        return Err("Score tempo requires a decimal from 10 to 400 quarter notes per minute".into());
    }
    Ok(input.into())
}

fn step(input: &str) -> Result<u8, String> {
    match input.as_bytes() {
        [value @ (b'A'..=b'G' | b'a'..=b'g')] => Ok(value.to_ascii_uppercase()),
        _ => Err("Score requires an A to G pitch step".into()),
    }
}

fn pitch(input_step: &str, input_octave: &str, alter: i8) -> Result<Pitch, String> {
    let octave = integer(input_octave)?;
    if octave > 8 || !(-2..=2).contains(&alter) {
        return Err("Score pitch exceeds octave or accidental bounds".into());
    }
    Ok(Pitch {
        step: step(input_step)?,
        octave: octave as u8,
        alter,
    })
}

fn push_pitch(event: &mut Event, pitch: Pitch) -> Result<(), String> {
    if event.pitches.len() >= 16
        || event
            .pitches
            .iter()
            .any(|other| other.step == pitch.step && other.octave == pitch.octave)
    {
        return Err(
            "Score chord requires up to 16 pitches at nonconflicting staff positions".into(),
        );
    }
    event.pitches.push(pitch);
    Ok(())
}

fn read_musicxml(root: &XEl) -> Result<Score, String> {
    attrs_allowed(root, &["version"])?;
    if root.name != "score-partwise" || attr(root, "version")? != "4.0" {
        return Err("MusicXML score requires the 4.0 score-partwise root".into());
    }
    children_allowed(root, &["movement-title", "part-list", "part"])?;
    let title = optional(root, "movement-title")?
        .map(content)
        .transpose()?
        .unwrap_or_default();
    let parts = one(root, "part-list")?;
    attrs_allowed(parts, &[])?;
    children_allowed(parts, &["score-part"])?;
    let declaration = one(parts, "score-part")?;
    attrs_allowed(declaration, &["id"])?;
    children_allowed(declaration, &["part-name"])?;
    let part_name = content(one(declaration, "part-name")?)?;
    let part = one(root, "part")?;
    attrs_allowed(part, &["id"])?;
    if attr(part, "id")? != attr(declaration, "id")? {
        return Err("MusicXML declared and performed part IDs differ".into());
    }
    children_allowed(part, &["measure"])?;
    let mut signature = None;
    let mut initial_tempo = None;
    let mut measures = Vec::new();
    for measure in elements(part) {
        attrs_allowed(measure, &["number"])?;
        children_allowed(measure, &["attributes", "note", "sound"])?;
        let mut events: Vec<Event> = Vec::new();
        for node in elements(measure) {
            if node.name == "attributes" {
                if !events.is_empty() || signature.is_some() {
                    return Err("MusicXML timing attributes are only supported at the start of the first measure".into());
                }
                attrs_allowed(node, &[])?;
                children_allowed(node, &["divisions", "time"])?;
                let divisions = integer(&content(one(node, "divisions")?)?)?;
                if divisions == 0 || divisions > 65_536 {
                    return Err("MusicXML divisions per quarter exceeds the supported range".into());
                }
                let time = one(node, "time")?;
                attrs_allowed(time, &[])?;
                children_allowed(time, &["beats", "beat-type"])?;
                let (count, unit) = meter(
                    integer(&content(one(time, "beats")?)?)?,
                    integer(&content(one(time, "beat-type")?)?)?,
                )?;
                signature = Some((count, unit, divisions));
                continue;
            }
            if node.name == "sound" {
                if !measures.is_empty() || !events.is_empty() || initial_tempo.is_some() {
                    return Err(
                        "MusicXML tempo is only supported at the start of the first measure".into(),
                    );
                }
                attrs_allowed(node, &["tempo"])?;
                children_allowed(node, &[])?;
                initial_tempo = Some(tempo(attr(node, "tempo")?)?);
                continue;
            }
            let (_, _, divisions) =
                signature.ok_or("MusicXML requires divisions and meter before notes")?;
            let (duration, pitch, chord) = read_musicxml_note(node, divisions)?;
            if chord {
                let previous = events
                    .last_mut()
                    .ok_or("MusicXML chord lacks a leading note")?;
                if previous.pitches.is_empty() || previous.duration != duration {
                    return Err("MusicXML chord notes must share one duration and onset".into());
                }
                push_pitch(
                    previous,
                    pitch.ok_or("A rest cannot extend a MusicXML chord")?,
                )?;
            } else {
                events.push(Event {
                    duration,
                    pitches: pitch.into_iter().collect(),
                });
            }
        }
        measures.push(Measure {
            number: attr(measure, "number")?.into(),
            events,
        });
    }
    let (count, unit, _) = signature.ok_or("MusicXML is missing its initial meter")?;
    Ok(Score {
        title,
        part_name,
        count,
        unit,
        tempo: initial_tempo,
        measures,
    })
}

fn read_musicxml_note(
    note: &XEl,
    divisions: u32,
) -> Result<(Duration, Option<Pitch>, bool), String> {
    attrs_allowed(note, &[])?;
    children_allowed(
        note,
        &[
            "chord",
            "pitch",
            "rest",
            "duration",
            "voice",
            "type",
            "dot",
            "accidental",
            "staff",
        ],
    )?;
    for name in ["voice", "staff"] {
        if let Some(element) = optional(note, name)? {
            if content(element)? != "1" {
                return Err("MusicXML score requires one voice and one staff".into());
            }
        }
    }
    let raw = integer(&content(one(note, "duration")?)?)?;
    let exact = raw
        .checked_mul(TICKS)
        .ok_or("Overflowing MusicXML duration")?;
    if exact % divisions != 0 {
        return Err("MusicXML duration cannot be represented exactly".into());
    }
    let duration = Duration::from_ticks(exact / divisions)?;
    let kind = optional(note, "type")?;
    let mut dots = 0;
    for dot in note.find_children("dot") {
        attrs_allowed(dot, &[])?;
        children_allowed(dot, &[])?;
        dots += 1;
    }
    if let Some(kind) = kind {
        if content(kind)? != duration.musicxml_type() || duration.dots != dots {
            return Err("MusicXML sound and notation durations disagree".into());
        }
    } else if dots != 0 {
        return Err("MusicXML dots require a notation type".into());
    }
    let chord = optional(note, "chord")?;
    if let Some(chord) = chord {
        attrs_allowed(chord, &[])?;
        children_allowed(chord, &[])?;
    }
    let resolved = match (optional(note, "pitch")?, optional(note, "rest")?) {
        (Some(node), None) => {
            attrs_allowed(node, &[])?;
            children_allowed(node, &["step", "octave", "alter"])?;
            let alter = optional(node, "alter")?
                .map(|element| {
                    content(element)?
                        .parse::<i8>()
                        .map_err(|_| "MusicXML pitch alter must be an integer".to_owned())
                })
                .transpose()?
                .unwrap_or(0);
            Some(pitch(
                &content(one(node, "step")?)?,
                &content(one(node, "octave")?)?,
                alter,
            )?)
        }
        (None, Some(rest)) => {
            attrs_allowed(rest, &[])?;
            children_allowed(rest, &[])?;
            None
        }
        _ => return Err("MusicXML note requires exactly one pitch or rest".into()),
    };
    if let Some(element) = optional(note, "accidental")? {
        let accidental = content(element)?;
        let displayed = match accidental.as_str() {
            "natural" => 0,
            "sharp" => 1,
            "flat" => -1,
            "double-sharp" => 2,
            "flat-flat" => -2,
            _ => return Err("Unsupported displayed MusicXML accidental".into()),
        };
        if resolved.is_none_or(|pitch| pitch.alter != displayed) {
            return Err("MusicXML notated and performed accidentals disagree".into());
        }
    }
    Ok((duration, resolved, chord.is_some()))
}

fn read_mei(root: &XEl) -> Result<Score, String> {
    attrs_allowed(root, &["xmlns", "meiversion"])?;
    if root.name != "mei" || attr(root, "meiversion")? != "5.1" || attr(root, "xmlns")? != MEI_NS {
        return Err("MEI score requires an MEI 5.1 root in the MEI namespace".into());
    }
    children_allowed(root, &["meiHead", "music"])?;
    let title = if let Some(header) = optional(root, "meiHead")? {
        let file = sole_child(header, "fileDesc")?;
        attrs_allowed(file, &[])?;
        children_allowed(file, &["titleStmt", "pubStmt"])?;
        if let Some(publication) = optional(file, "pubStmt")? {
            attrs_allowed(publication, &[])?;
            children_allowed(publication, &[])?;
        }
        content(sole_child(one(file, "titleStmt")?, "title")?)?
    } else {
        String::new()
    };
    let body = sole_child(one(root, "music")?, "body")?;
    let score = sole_child(sole_child(body, "mdiv")?, "score")?;
    attrs_allowed(score, &[])?;
    children_allowed(score, &["scoreDef", "section"])?;
    let definition = one(score, "scoreDef")?;
    attrs_allowed(definition, &["meter.count", "meter.unit", "midi.bpm"])?;
    children_allowed(definition, &["staffGrp"])?;
    let (count, unit) = meter(
        integer(attr(definition, "meter.count")?)?,
        integer(attr(definition, "meter.unit")?)?,
    )?;
    let tempo = definition.attr("midi.bpm").map(tempo).transpose()?;
    let staff_definition = sole_child(one(definition, "staffGrp")?, "staffDef")?;
    attrs_allowed(staff_definition, &["n", "lines"])?;
    children_allowed(staff_definition, &["label"])?;
    if attr(staff_definition, "n")? != "1" || attr(staff_definition, "lines")? != "5" {
        return Err("MEI score requires one five-line staff".into());
    }
    let part_name = optional(staff_definition, "label")?
        .map(content)
        .transpose()?
        .unwrap_or_default();
    let section = one(score, "section")?;
    attrs_allowed(section, &[])?;
    children_allowed(section, &["measure"])?;
    let mut measures = Vec::new();
    for measure in elements(section) {
        attrs_allowed(measure, &["n"])?;
        children_allowed(measure, &["staff"])?;
        let staff = one(measure, "staff")?;
        attrs_allowed(staff, &["n"])?;
        children_allowed(staff, &["layer"])?;
        let layer = one(staff, "layer")?;
        attrs_allowed(layer, &["n"])?;
        children_allowed(layer, &["note", "rest", "chord"])?;
        if attr(staff, "n")? != "1" || attr(layer, "n")? != "1" {
            return Err("MEI score requires a single staff and layer numbered 1".into());
        }
        let mut events = Vec::new();
        let mut accidentals = BTreeMap::new();
        for node in elements(layer) {
            let duration = Duration::checked(
                integer(attr(node, "dur")?)?,
                node.attr("dots").map(integer).transpose()?.unwrap_or(0),
            )?;
            let pitches = match node.name.as_str() {
                "note" => vec![read_mei_note(node, true, &mut accidentals)?],
                "rest" => {
                    attrs_allowed(node, &["dur", "dots"])?;
                    children_allowed(node, &[])?;
                    Vec::new()
                }
                "chord" => {
                    attrs_allowed(node, &["dur", "dots"])?;
                    children_allowed(node, &["note"])?;
                    let mut event = Event {
                        duration,
                        pitches: Vec::new(),
                    };
                    for note in elements(node) {
                        push_pitch(&mut event, read_mei_note(note, false, &mut accidentals)?)?;
                    }
                    if event.pitches.len() < 2 {
                        return Err("MEI chord requires at least two pitches".into());
                    }
                    event.pitches
                }
                _ => unreachable!(),
            };
            events.push(Event { duration, pitches });
        }
        measures.push(Measure {
            number: attr(measure, "n")?.into(),
            events,
        });
    }
    Ok(Score {
        title,
        part_name,
        count,
        unit,
        tempo,
        measures,
    })
}

fn sole_child<'a>(container: &'a XEl, child: &str) -> Result<&'a XEl, String> {
    attrs_allowed(container, &[])?;
    children_allowed(container, &[child])?;
    one(container, child)
}

fn read_mei_note(
    node: &XEl,
    standalone: bool,
    accidentals: &mut BTreeMap<(u8, u8), i8>,
) -> Result<Pitch, String> {
    let allowed: &[&str] = if standalone {
        &["pname", "oct", "accid", "accid.ges", "dur", "dots"]
    } else {
        &["pname", "oct", "accid", "accid.ges"]
    };
    attrs_allowed(node, allowed)?;
    children_allowed(node, &[])?;
    let parse = |value| match value {
        "n" => Ok(0),
        "s" => Ok(1),
        "f" => Ok(-1),
        "ss" | "x" => Ok(2),
        "ff" => Ok(-2),
        _ => Err("MEI pitch uses an accidental outside the interchange profile"),
    };
    let mut pitch = pitch(attr(node, "pname")?, attr(node, "oct")?, 0)?;
    let written = node.attr("accid").map(parse).transpose()?;
    let key = (pitch.step, pitch.octave);
    if let Some(written) = written {
        accidentals.insert(key, written);
    }
    let gestural = node
        .attr("accid.ges")
        .map(|value| {
            if value == "x" {
                return Err("The MEI gestural double sharp symbol is ss");
            }
            parse(value)
        })
        .transpose()?;
    let written_pitch = *accidentals.get(&key).unwrap_or(&0);
    if written
        .zip(gestural)
        .is_some_and(|(written, performed)| written != performed)
    {
        return Err("MEI notated and performed accidentals disagree".into());
    }
    pitch.alter = gestural.unwrap_or(written_pitch);
    Ok(pitch)
}

fn leaf(name: &str, value: impl Into<String>) -> XEl {
    XEl::new(name).with_text(value)
}

fn musicxml_note(event: &Event, pitch: Option<Pitch>, chord: bool, displayed: bool) -> XEl {
    let mut note = XEl::new("note");
    if chord {
        note = note.with_child(XEl::new("chord"));
    }
    if let Some(pitch) = pitch {
        let mut position =
            XEl::new("pitch").with_child(leaf("step", char::from(pitch.step).to_string()));
        if pitch.alter != 0 {
            position = position.with_child(leaf("alter", pitch.alter.to_string()));
        }
        note = note.with_child(position.with_child(leaf("octave", pitch.octave.to_string())));
    } else {
        note = note.with_child(XEl::new("rest"));
    }
    note = note
        .with_child(leaf("duration", event.duration.ticks().to_string()))
        .with_child(leaf("type", event.duration.musicxml_type()));
    for _ in 0..event.duration.dots {
        note = note.with_child(XEl::new("dot"));
    }
    if let Some(pitch) = pitch.filter(|_| displayed) {
        const ACCIDENTALS: [&str; 5] = ["flat-flat", "flat", "natural", "sharp", "double-sharp"];
        note = note.with_child(leaf("accidental", ACCIDENTALS[(pitch.alter + 2) as usize]));
    }
    note
}

fn write_musicxml(score: &Score) -> XEl {
    let mut part = XEl::new("part").with_attr("id", "P1");
    for (index, measure) in score.measures.iter().enumerate() {
        let mut output = XEl::new("measure").with_attr("number", &measure.number);
        if index == 0 {
            output = output.with_child(
                XEl::new("attributes")
                    .with_child(leaf("divisions", TICKS.to_string()))
                    .with_child(
                        XEl::new("time")
                            .with_child(leaf("beats", score.count.to_string()))
                            .with_child(leaf("beat-type", score.unit.to_string())),
                    ),
            );
            if let Some(tempo) = &score.tempo {
                output = output.with_child(XEl::new("sound").with_attr("tempo", tempo));
            }
        }
        let mut accidentals = BTreeMap::new();
        for event in &measure.events {
            if event.pitches.is_empty() {
                output = output.with_child(musicxml_note(event, None, false, false));
            }
            for (voice, &pitch) in event.pitches.iter().enumerate() {
                let displayed = change_accidental(pitch, &mut accidentals);
                output = output.with_child(musicxml_note(event, Some(pitch), voice > 0, displayed));
            }
        }
        part = part.with_child(output);
    }
    XEl::new("score-partwise")
        .with_attr("version", "4.0")
        .with_child(leaf("movement-title", &score.title))
        .with_child(
            XEl::new("part-list").with_child(
                XEl::new("score-part")
                    .with_attr("id", "P1")
                    .with_child(leaf("part-name", &score.part_name)),
            ),
        )
        .with_child(part)
}

fn change_accidental(pitch: Pitch, accidentals: &mut BTreeMap<(u8, u8), i8>) -> bool {
    accidentals
        .insert((pitch.step, pitch.octave), pitch.alter)
        .unwrap_or(0)
        != pitch.alter
}

fn mei_note(pitch: Pitch, accidentals: &mut BTreeMap<(u8, u8), i8>) -> XEl {
    const ACCIDENTALS: [&str; 5] = ["ff", "f", "n", "s", "ss"];
    let accidental = ACCIDENTALS[(pitch.alter + 2) as usize];
    let mut note = XEl::new("note")
        .with_attr(
            "pname",
            char::from(pitch.step.to_ascii_lowercase()).to_string(),
        )
        .with_attr("oct", pitch.octave.to_string())
        .with_attr("accid.ges", accidental);
    if change_accidental(pitch, accidentals) {
        note = note.with_attr("accid", accidental);
    }
    note
}

fn write_mei(score: &Score) -> XEl {
    let mut definition = XEl::new("scoreDef")
        .with_attr("meter.count", score.count.to_string())
        .with_attr("meter.unit", score.unit.to_string())
        .with_child(
            XEl::new("staffGrp").with_child(
                XEl::new("staffDef")
                    .with_attr("n", "1")
                    .with_attr("lines", "5")
                    .with_child(leaf("label", &score.part_name)),
            ),
        );
    if let Some(tempo) = &score.tempo {
        definition = definition.with_attr("midi.bpm", tempo);
    }
    let mut section = XEl::new("section");
    for measure in &score.measures {
        let mut layer = XEl::new("layer").with_attr("n", "1");
        let mut accidentals = BTreeMap::new();
        for event in &measure.events {
            let mut node = match event.pitches.as_slice() {
                [] => XEl::new("rest"),
                [pitch] => mei_note(*pitch, &mut accidentals),
                pitches => pitches.iter().fold(XEl::new("chord"), |chord, &pitch| {
                    chord.with_child(mei_note(pitch, &mut accidentals))
                }),
            }
            .with_attr("dur", event.duration.denominator.to_string());
            if event.duration.dots != 0 {
                node = node.with_attr("dots", event.duration.dots.to_string());
            }
            layer = layer.with_child(node);
        }
        section = section.with_child(
            XEl::new("measure")
                .with_attr("n", &measure.number)
                .with_child(XEl::new("staff").with_attr("n", "1").with_child(layer)),
        );
    }
    XEl::new("mei")
        .with_attr("xmlns", MEI_NS)
        .with_attr("meiversion", "5.1")
        .with_child(
            XEl::new("meiHead").with_child(
                XEl::new("fileDesc")
                    .with_child(XEl::new("titleStmt").with_child(leaf("title", &score.title)))
                    .with_child(XEl::new("pubStmt")),
            ),
        )
        .with_child(
            XEl::new("music").with_child(
                XEl::new("body").with_child(
                    XEl::new("mdiv")
                        .with_child(XEl::new("score").with_child(definition).with_child(section)),
                ),
            ),
        )
}
