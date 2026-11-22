//! In-memory, block-addressed sequential zone placement.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, *};
use serde_json::{Value as Json, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Empty,
    Open,
    Closed,
    Full,
}

impl State {
    fn tag(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Full => "full",
        }
    }
}

struct Zone {
    start: u64,
    capacity: u64,
    pointer: u64,
    state: State,
}

impl Zone {
    fn json(&self) -> Json {
        json!({"start_lba":self.start,"capacity_blocks":self.capacity,"written_blocks":self.pointer,"state":self.state.tag()})
    }
}

impl AssetHost<'_> {
    pub(crate) fn offload_zns_csd_and_qubo_commons(&mut self, a: &Args) -> Result<Value, String> {
        match text(required(a, "mode")?)? {
            "sequential_zone_plan" => self.sequential_zone_plan(a),
            "offline_qubo_anneal" => self.offline_qubo_anneal(a),
            "compile_qubo_constraint_fixture" => self.compile_qubo_constraint_fixture(a),
            _ => Err("Select sequential_zone_plan, offline_qubo_anneal or compile_qubo_constraint_fixture".into()),
        }
    }

    fn sequential_zone_plan(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &["mode", "block_bytes", "max_open_zones", "zones", "segments"],
        )?;
        let block = exact_u64(a, "block_bytes")?;
        if !matches!(block, 512 | 1024 | 2048 | 4096 | 8192 | 16384) {
            return Err("Block size must be a power of two between 512 and 16,384".into());
        }
        let max_open = integer(required(a, "max_open_zones")?, "max_open_zones")?;
        let rows = list(required(a, "zones")?)?;
        let segments = list(required(a, "segments")?)?;
        if !(1..=128).contains(&rows.len())
            || max_open == 0
            || max_open > rows.len()
            || !(1..=256).contains(&segments.len())
        {
            return Err("Invalid zone limit, directory size or segment count".into());
        }
        self.charge(
            rows.len() * segments.len() * 128,
            (rows.len() + segments.len()) * 16_384,
        )?;
        let mut zones = rows.iter().map(parse_zone).collect::<Result<Vec<_>, _>>()?;
        zones.sort_unstable_by_key(|z| z.start);
        if zones
            .windows(2)
            .any(|pair| pair[0].start + pair[0].capacity > pair[1].start)
        {
            return Err("Zones overlap in the synthetic LBA address space".into());
        }
        let original = zones.iter().map(Zone::json).collect::<Vec<_>>();
        let mut open = zones.iter().filter(|z| z.state == State::Open).count();
        if open > max_open {
            return Err("Input exceeds its declared open-zone limit".into());
        }
        let mut peak = open;
        let mut seen = BTreeSet::new();
        let mut placements = Vec::new();
        let mut appended = 0u64;
        for item in segments {
            let row = record(item)?;
            fields(row, &["id", "bytes"])?;
            let id = text(required(row, "id")?)?;
            let bytes = exact_u64(row, "bytes")?;
            if id.is_empty()
                || id.len() > 80
                || id.chars().any(char::is_control)
                || !seen.insert(id)
                || bytes == 0
                || bytes % block != 0
            {
                return Err(
                    "Segments require unique admitted ids and positive block-aligned lengths"
                        .into(),
                );
            }
            let mut remaining = bytes / block;
            let mut offset = 0u64;
            while remaining != 0 {
                let target = zones
                    .iter()
                    .position(|z| z.state == State::Open)
                    .or_else(|| {
                        if open >= max_open {
                            return None;
                        }
                        zones
                            .iter()
                            .position(|z| matches!(z.state, State::Empty | State::Closed))
                    })
                    .ok_or("Insufficient admitted capacity for the sequential append plan")?;
                let zone = &mut zones[target];
                if zone.state != State::Open {
                    zone.state = State::Open;
                    open += 1;
                    peak = peak.max(open);
                }
                let blocks = remaining.min(zone.capacity - zone.pointer);
                let lba = zone.start + zone.pointer;
                placements.push(
                    json!({"segment":id,"source_offset_bytes":offset,"start_lba":lba,
                    "zone_start_lba":zone.start,"blocks":blocks}),
                );
                offset += blocks * block;
                remaining -= blocks;
                zone.pointer += blocks;
                if zone.pointer == zone.capacity {
                    zone.state = State::Full;
                    open -= 1;
                }
                if placements.len() > 512 {
                    return Err("Sequential zone placement exceeds 512 spans".into());
                }
            }
            appended = appended
                .checked_add(bytes)
                .ok_or("Append total exceeds a u64 byte extent")?;
        }
        let plan = serde_json::to_vec(&json!({"schema":"qualia.sequential-zone-plan/v1","mode":"offline",
            "logical_block_bytes":block,"max_open_zones":max_open,"peak_open_zones":peak,"total_bytes":appended,
            "zones_before":original,"zones_after":zones.iter().map(Zone::json).collect::<Vec<_>>(),"placements":placements})).map_err(|e| e.to_string())?;
        let digest = format!("{:x}", Sha256::digest(&plan));
        let subject = format!("urn:qualia:zns-placement:sha256:{digest}");
        let mut graph = ReceiptGraph::new(&subject);
        graph.fact(
            &subject,
            "urn:qualia:storage:placementKind",
            "urn:qualia:storage:offline-sequential-zone-plan",
        )?;
        let receipt = graph.seal(self.limits.resource_bytes)?;
        Ok(Value::Record(Args::from([
            ("plan".into(), self.encode("json", plan)?),
            ("receipt".into(), self.encode("q42", receipt)?),
            ("bytes".into(), Value::U64(appended)),
            ("spans".into(), Value::U64(placements.len() as u64)),
            ("peak_open_zones".into(), Value::U64(peak as u64)),
        ])))
    }
}

fn parse_zone(input: &Value) -> Result<Zone, String> {
    let row = record(input)?;
    fields(
        row,
        &["start_lba", "capacity_blocks", "written_blocks", "state"],
    )?;
    let start = exact_u64(row, "start_lba")?;
    let capacity = exact_u64(row, "capacity_blocks")?;
    let pointer = exact_u64(row, "written_blocks")?;
    let state = match text(required(row, "state")?)? {
        "empty" if pointer == 0 => State::Empty,
        "open" if pointer < capacity => State::Open,
        "closed" if pointer < capacity => State::Closed,
        "full" if pointer == capacity => State::Full,
        _ => return Err("Zone state does not match its declared write pointer".into()),
    };
    if capacity == 0 || start.checked_add(capacity).is_none() {
        return Err("Invalid synthetic zone LBA extent".into());
    }
    Ok(Zone {
        start,
        capacity,
        pointer,
        state,
    })
}

fn exact_u64(a: &Args, key: &str) -> Result<u64, String> {
    match required(a, key)? {
        Value::U64(n) => Ok(*n),
        Value::I64(n) => u64::try_from(*n).map_err(|_| format!("{key} must be nonnegative")),
        _ => Err(format!("{key} requires an exact integer")),
    }
}
