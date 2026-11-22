//! Offline authoring for live environments. The deployed host owns device I/O.

use super::{exact_u64, external_resource, ReceiptGraph, SYMBOLIC_LIMIT};
use crate::{
    artifact::HmcWorkspaceView, boolean, fields, list, num, record, required, text, vector, Args,
    AssetHost, TypedArtifact, Value,
};
use qualia_core_db::{
    bundle::{
        semantic_model::{pack_semantic_hmc, HmcPackAsset, HmcPackLimits},
        BundleReadLimits, BundleReader,
    },
    hypermedia_authoring::{
        dmx::FixtureType,
        interactive::{InteractivePage, Trigger, TriggerEvent},
        portals::MovementMode,
        Avatar, Cue, CueStack, DmxFixture, DmxUniverse, HbbTVApp, Portal, SecondScreen, World,
    },
};
use serde_json::{json, Value as Json};
use std::collections::{BTreeMap, BTreeSet};

const MANIFEST_KEY: &str = "live/manifest.json";
const SEMANTICS_KEY: &str = "live/model.q42";
const VOCABULARY: &str = "https://qualiadb.org/ontology/live#";
const MANIFEST_BYTES: usize = 4 * 1024 * 1024;
const MAX_RECORDS: usize = 128;

impl AssetHost<'_> {
    pub(crate) fn author_dmx_hbbtv_and_portal_world(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["domain", "source", "spec", "artifact_iri"])?;
        let domain = text(required(args, "domain")?)?;
        let source = required(args, "source")?;
        let spec = record(required(args, "spec")?)?;
        let root = uri(required(args, "artifact_iri")?)?;
        let mut graph = ReceiptGraph::new(VOCABULARY);
        graph.fact(
            root,
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
            &format!("{VOCABULARY}LiveManifest"),
        )?;
        let mut keys = BTreeSet::new();
        let manifest_body = match domain {
            "dmx512_cue_stack" => dmx_manifest(root, spec, &mut graph)?,
            "hbbtv_second_screen" => hbbtv_manifest(root, spec, &mut graph, &mut keys)?,
            "portal_world_avatar" => world_manifest(root, spec, &mut graph, &mut keys)?,
            "spatial_xr_bindings" => xr_manifest(root, spec, &mut graph)?,
            _ => return Err(format!("Unknown live authoring domain {domain}")),
        };
        let manifest = serde_json::to_vec(&json!({
            "schema": "https://qualiadb.org/ontology/live#ManifestV1",
            "artifact_iri": root,
            "domain": domain,
            "spec": manifest_body
        }))
        .map_err(|e| format!("Live manifest encoding: {e}"))?;
        if manifest.len() > MANIFEST_BYTES.min(self.limits.resource_bytes) {
            return Err("Live manifest exceeds the byte budget".into());
        }
        let semantic = graph.seal(self.limits.resource_bytes)?;

        let file_budget = self.limits.resource_bytes.min(
            self.limits
                .total_resource_bytes
                .saturating_sub(self.workspace.bytes)
                .saturating_sub(manifest.len()),
        );
        let read_limits = BundleReadLimits {
            file_bytes: self.limits.resource_bytes,
            entries: self.limits.resources,
            ..BundleReadLimits::default()
        };
        let limits = HmcPackLimits {
            bundle: BundleReadLimits {
                file_bytes: file_budget,
                ..read_limits
            },
            resource_bytes: self.limits.resource_bytes,
            total_resource_bytes: self.limits.total_resource_bytes,
            semantic_bytes: SYMBOLIC_LIMIT,
        };
        let read;
        let assets = match self.workspace.typed(source) {
            Ok(TypedArtifact::HmcBundle(bundle)) => match bundle.as_ref() {
                HmcWorkspaceView::Staged(resources) => resources
                    .iter()
                    .map(|resource| asset(&resource.key, &resource.kind, &resource.bytes, None))
                    .collect(),
                HmcWorkspaceView::Sealed(bytes) => {
                    read = admit_hmc(bytes, read_limits)?;
                    bundle_assets(&read)?
                }
            },
            Ok(TypedArtifact::SceneGraph10d(scene)) => {
                vec![asset(
                    "scene.10d",
                    "10d",
                    scene.fields().as_10d_bytes(),
                    None,
                )]
            }
            _ => {
                let resource = external_resource(self, source)?;
                match resource.kind.as_str() {
                    "hmc" => {
                        read = admit_hmc(&resource.bytes, read_limits)?;
                        bundle_assets(&read)?
                    }
                    "10d" => vec![asset("scene.10d", "10d", &resource.bytes, None)],
                    _ => return Err("Live authoring requires a sealed 10D scene or an HMC".into()),
                }
            }
        };
        for (key, kind) in keys {
            if !assets
                .iter()
                .any(|entry| entry.key == key && entry.kind == kind)
            {
                return Err(format!(
                    "Live manifest references missing {kind} entry {key}"
                ));
            }
        }
        let mut assets = assets;
        assets.extend([
            asset(MANIFEST_KEY, "json", &manifest, None),
            asset(SEMANTICS_KEY, "q42", &semantic, None),
        ]);
        let work = assets
            .iter()
            .try_fold(0usize, |sum, asset| sum.checked_add(asset.bytes.len()))
            .ok_or("Live package size overflow")?;
        if self.work.saturating_add(work as u64) > self.limits.asset_work_units {
            return Err("Live HMC exceeds the production work budget".into());
        }
        let sealed = pack_semantic_hmc(&assets, None, limits, self.cancel)
            .map_err(|e| format!("Live HMC packing: {e}"))?;
        self.charge(work, sealed.hmc.len().saturating_add(manifest.len()))?;
        Ok(Value::Record(BTreeMap::from([
            ("hmc".into(), self.encode("hmc", sealed.hmc)?),
            ("manifest".into(), self.encode("json", manifest)?),
            ("manifest_key".into(), Value::String(MANIFEST_KEY.into())),
        ])))
    }
}

fn asset<'a>(
    key: &'a str,
    kind: &'a str,
    bytes: &'a [u8],
    metadata: Option<&'a [u8]>,
) -> HmcPackAsset<'a> {
    HmcPackAsset {
        key,
        kind,
        bytes,
        metadata,
        salience: None,
    }
}

fn admit_hmc(bytes: &[u8], limits: BundleReadLimits) -> Result<BundleReader<'_>, String> {
    BundleReader::parse_with_limits(bytes, limits).map_err(|e| format!("Live source HMC: {e}"))
}

fn bundle_assets<'a>(reader: &'a BundleReader<'_>) -> Result<Vec<HmcPackAsset<'a>>, String> {
    reader
        .entries()
        .iter()
        .map(|entry| {
            let bytes = reader.get(&entry.key).ok_or("Missing verified HMC entry")?;
            Ok(asset(&entry.key, &entry.kind, bytes, entry.meta.as_deref()))
        })
        .collect()
}

fn dmx_manifest(root: &str, spec: &Args, graph: &mut ReceiptGraph) -> Result<Json, String> {
    fields(
        spec,
        &["name", "universes", "fixtures", "cues", "auto_follow"],
    )?;
    let mut stack = CueStack::new(root, label(required(spec, "name")?)?);
    stack.auto_follow = boolean(spec, "auto_follow", false)?;
    let mut universes = BTreeMap::new();
    for universe in rows(spec, "universes", 32)? {
        let id = u16::try_from(exact_u64(universe)?).map_err(|_| "DMX universe exceeds u16")?;
        if universes.insert(id, DmxUniverse::new(id)).is_some() {
            return Err("Duplicate DMX universe ID".into());
        }
    }
    let mut fixtures = BTreeMap::new();
    let mut patched = BTreeSet::new();
    let mut fixture_manifest = Vec::new();
    for input in optional_rows(spec, "fixtures", MAX_RECORDS)? {
        let row = record(input)?;
        fields(
            row,
            &[
                "iri",
                "name",
                "type",
                "universe",
                "start_address",
                "channel_count",
                "functions",
            ],
        )?;
        let id = uri(required(row, "iri")?)?;
        if fixtures.contains_key(id) {
            return Err("Duplicate DMX fixture IRI".into());
        }
        let name = label(required(row, "name")?)?;
        let kind = text(required(row, "type")?)?;
        let fixture_kind = match kind {
            "moving_head" => FixtureType::MovingHead,
            "par_can" => FixtureType::ParCan,
            "strip_light" => FixtureType::StripLight,
            "laser" => FixtureType::Laser,
            "hazer" => FixtureType::Hazer,
            "fog_machine" => FixtureType::FogMachine,
            "spotlight" => FixtureType::Spotlight,
            "generic" => FixtureType::Generic,
            _ => return Err("Unknown DMX fixture type".into()),
        };
        let universe = u16::try_from(exact_u64(required(row, "universe")?)?)
            .map_err(|_| "Invalid fixture universe")?;
        let start = exact_u64(required(row, "start_address")?)?;
        let count = exact_u64(required(row, "channel_count")?)?;
        if !universes.contains_key(&universe)
            || !(1..=512).contains(&start)
            || !(1..=513 - start).contains(&count)
        {
            return Err("DMX fixture addresses exceed the declared universe".into());
        }
        let slot = start as u16 - 1;
        for address in slot..slot + count as u16 {
            if !patched.insert((universe, address)) {
                return Err("DMX fixtures overlap within one universe".into());
            }
        }
        let mut fixture = DmxFixture::new(id, name, fixture_kind, universe, slot, count as u16);
        let functions = record(required(row, "functions")?)?;
        if functions.is_empty() || functions.len() > count as usize {
            return Err("DMX fixture functions exceed the channel count".into());
        }
        let mut assigned = BTreeSet::new();
        let mut function_offsets = BTreeMap::new();
        for (function, value) in functions {
            let offset = exact_u64(value)?;
            if function.is_empty()
                || function.len() > 128
                || !(1..=count).contains(&offset)
                || !assigned.insert(offset)
            {
                return Err("Invalid or repeated fixture function address".into());
            }
            fixture.map_channel(function, offset as u16 - 1);
            function_offsets.insert(function, offset);
        }
        graph.fact(root, &format!("{VOCABULARY}fixture"), id)?;
        fixture_manifest.push(json!({
            "iri": id, "name": name, "type": kind, "universe": universe,
            "start_address": start, "channel_count": count, "functions": function_offsets
        }));
        fixtures.insert(id, fixture);
    }
    let mut authored = Vec::new();
    let mut ids = BTreeSet::new();
    for item in rows(spec, "cues", MAX_RECORDS)? {
        let row = record(item)?;
        fields(
            row,
            &[
                "iri",
                "name",
                "duration_s",
                "fade_in_s",
                "fade_out_s",
                "channels",
                "fixture_values",
            ],
        )?;
        let id = uri(required(row, "iri")?)?;
        if !ids.insert(id) {
            return Err("Duplicate DMX cue IRI".into());
        }
        let mut cue = Cue::new(id, label(required(row, "name")?)?);
        let duration = num(required(row, "duration_s")?)?;
        let fade_in = num(required(row, "fade_in_s")?)?;
        let fade_out = num(required(row, "fade_out_s")?)?;
        if !(0.0..=86_400.0).contains(&duration)
            || fade_in < 0.0
            || fade_out < 0.0
            || fade_in + fade_out > duration
        {
            return Err("DMX fade durations must fit the cue duration".into());
        }
        cue.duration = duration;
        cue.set_fade(fade_in, fade_out);
        for channel in optional_rows(row, "channels", 512)? {
            let [universe, address, value] = list(channel)? else {
                return Err("DMX updates require [universe, address_1_to_512, value]".into());
            };
            let universe =
                u16::try_from(exact_u64(universe)?).map_err(|_| "Invalid DMX universe")?;
            let address = exact_u64(address)?;
            let value =
                u8::try_from(exact_u64(value)?).map_err(|_| "Invalid DMX intensity byte")?;
            if !universes.contains_key(&universe) || !(1..=512).contains(&address) {
                return Err(
                    "DMX update references an undeclared universe or invalid 1-based address"
                        .into(),
                );
            }
            let slot = address as u16 - 1;
            if cue.channel_values.contains_key(&(universe, slot)) {
                return Err("Repeated DMX channel within a cue".into());
            }
            cue.set_channel(universe, slot, value);
        }
        for level in optional_rows(row, "fixture_values", 512)? {
            let [fixture, function, level] = list(level)? else {
                return Err("DMX fixture updates require [fixture_iri, function, value]".into());
            };
            let fixture = fixtures
                .get(uri(fixture)?)
                .ok_or("DMX cue references an unknown fixture")?;
            let function = text(function)?;
            let offset = *fixture
                .channel_map
                .get(function)
                .ok_or("DMX cue references an unknown fixture function")?;
            let level =
                u8::try_from(exact_u64(level)?).map_err(|_| "Invalid DMX intensity byte")?;
            let channel = fixture.start_channel + offset;
            if cue
                .channel_values
                .contains_key(&(fixture.universe, channel))
            {
                return Err("DMX cue writes the same fixture or absolute channel twice".into());
            }
            cue.set_channel(fixture.universe, channel, level);
        }
        let channels: Vec<_> = cue
            .channel_values
            .iter()
            .map(|(&(universe, address), &value)| json!([universe, address + 1, value]))
            .collect();
        graph.fact(root, &format!("{VOCABULARY}cue"), id)?;
        authored.push(json!({
            "iri": cue.id, "name": cue.name, "duration_s": cue.duration,
            "fade_in_s": cue.fade_in, "fade_out_s": cue.fade_out, "channels": channels
        }));
        stack.add_cue(cue);
        if !stack.go(&mut universes) {
            return Err("DMX cue did not advance natively".into());
        }
    }
    let final_levels: Vec<_> = universes
        .values()
        .map(|universe| {
            let nonzero: Vec<_> = universe
                .channels
                .iter()
                .enumerate()
                .filter(|(_, value)| **value != 0)
                .map(|(slot, &value)| json!([slot + 1, value]))
                .collect();
            json!({"id": universe.id, "nonzero_channels": nonzero})
        })
        .collect();
    Ok(json!({
        "name": stack.name, "auto_follow": stack.auto_follow,
        "fixtures": fixture_manifest, "cues": authored, "universes_at_last_cue": final_levels
    }))
}

fn hbbtv_manifest(
    root: &str,
    spec: &Args,
    graph: &mut ReceiptGraph,
    keys: &mut BTreeSet<(String, String)>,
) -> Result<Json, String> {
    fields(
        spec,
        &["name", "pages", "start_page", "state", "companions"],
    )?;
    let mut app = HbbTVApp::new(root, label(required(spec, "name")?)?);
    let mut authored = Vec::new();
    let mut destinations = Vec::new();
    for item in rows(spec, "pages", MAX_RECORDS)? {
        let row = record(item)?;
        fields(row, &["iri", "title", "html_key", "navigation"])?;
        let id = uri(required(row, "iri")?)?;
        if app.pages.contains_key(id) {
            return Err("Duplicate HbbTV page IRI".into());
        }
        let title = label(required(row, "title")?)?;
        let html = label(required(row, "html_key")?)?;
        keys.insert((html.into(), "html".into()));
        let mut triggers = Vec::new();
        let mut navigation = Vec::new();
        let mut seen = BTreeSet::new();
        for item in optional_rows(row, "navigation", 8)? {
            let nav = record(item)?;
            fields(nav, &["event", "target"])?;
            let event = text(required(nav, "event")?)?;
            let target = uri(required(nav, "target")?)?;
            if !seen.insert(event) {
                return Err("Duplicate HbbTV navigation event".into());
            }
            triggers.push(Trigger {
                id: event.into(),
                event: trigger(event)?,
                action: target.into(),
            });
            destinations.push(target);
            navigation.push(json!({"event": event, "target": target}));
        }
        app.add_page(InteractivePage {
            id: id.into(),
            title: title.into(),
            elements: Vec::new(),
            triggers,
        });
        graph.fact(root, &format!("{VOCABULARY}page"), id)?;
        authored
            .push(json!({"iri": id, "title": title, "html_key": html, "navigation": navigation}));
    }
    if destinations.iter().any(|id| !app.pages.contains_key(*id)) {
        return Err("HbbTV navigation targets an undeclared page".into());
    }
    let start = uri(required(spec, "start_page")?)?;
    if !app.navigate_to(start) {
        return Err("HbbTV start_page is undeclared".into());
    }
    if let Some(value) = spec.get("state") {
        let state = record(value)?;
        if state.len() > MAX_RECORDS {
            return Err("HbbTV state count exceeds the authoring bound".into());
        }
        for (key, value) in state {
            if key.is_empty() || key.len() > 128 {
                return Err("Invalid HbbTV state key".into());
            }
            app.set_state(key, label(value)?);
        }
    }
    let mut companions = Vec::new();
    let mut ids = BTreeSet::new();
    for item in optional_rows(spec, "companions", 32)? {
        let row = record(item)?;
        fields(row, &["iri", "device_role", "content_iri", "offset_s"])?;
        let id = uri(required(row, "iri")?)?;
        if !ids.insert(id) {
            return Err("Duplicate HbbTV companion IRI".into());
        }
        let device = label(required(row, "device_role")?)?;
        let content = uri(required(row, "content_iri")?)?;
        let offset = num(required(row, "offset_s")?)?;
        if !(-3600.0..=3600.0).contains(&offset) {
            return Err("HbbTV companion offset exceeds one hour".into());
        }
        let mut screen = SecondScreen::new(id, device);
        screen.sync_to_content(content, offset);
        graph.fact(root, &format!("{VOCABULARY}companion"), id)?;
        graph.fact(id, &format!("{VOCABULARY}synchronizedContent"), content)?;
        companions.push(json!({
            "iri": screen.id, "device_role": screen.device_id,
            "content_iri": screen.synced_content, "offset_s": screen.sync_offset
        }));
    }
    Ok(json!({
        "name": app.name, "pages": authored, "start_page": app.current_page,
        "state": app.state, "companions": companions
    }))
}

fn trigger(event: &str) -> Result<TriggerEvent, String> {
    match event {
        "red" => Ok(TriggerEvent::RedButton),
        "green" => Ok(TriggerEvent::GreenButton),
        "yellow" => Ok(TriggerEvent::YellowButton),
        "blue" => Ok(TriggerEvent::BlueButton),
        "ok" => Ok(TriggerEvent::OkButton),
        "back" => Ok(TriggerEvent::BackButton),
        "timer" => Ok(TriggerEvent::Timer),
        "stream_event" => Ok(TriggerEvent::StreamEvent),
        _ => Err("Invalid HbbTV trigger event".into()),
    }
}

fn world_manifest(
    root: &str,
    spec: &Args,
    graph: &mut ReceiptGraph,
    keys: &mut BTreeSet<(String, String)>,
) -> Result<Json, String> {
    fields(spec, &["worlds", "entry_world"])?;
    let rows = rows(spec, "worlds", 32)?;
    let mut worlds = BTreeMap::new();
    for input in rows {
        let row = record(input)?;
        fields(
            row,
            &[
                "iri",
                "name",
                "scene_key",
                "bounds_m",
                "gravity_m_s2",
                "portals",
                "avatars",
            ],
        )?;
        let id = uri(required(row, "iri")?)?;
        let mut world = World::new(id, label(required(row, "name")?)?);
        let [min, max] = list(required(row, "bounds_m")?)? else {
            return Err("World bounds require [min_xyz, max_xyz] in meters".into());
        };
        world.bounds.min = f32_vector(min)?;
        world.bounds.max = f32_vector(max)?;
        if (0..3).any(|axis| world.bounds.min[axis] >= world.bounds.max[axis]) {
            return Err("World bounds have an empty axis".into());
        }
        let [x, y, z] = f32_vector(required(row, "gravity_m_s2")?)?;
        if [x, y, z].iter().any(|v| v.abs() > 100.0) {
            return Err("World gravity exceeds the offline solver range".into());
        }
        world.set_gravity(x, y, z);
        if worlds.insert(id, world).is_some() {
            return Err("Duplicate world IRI".into());
        }
    }
    let entry = uri(required(spec, "entry_world")?)?;
    if !worlds.contains_key(entry) {
        return Err("Entry world is undeclared".into());
    }
    let bounds: BTreeMap<_, _> = worlds
        .iter()
        .map(|(&id, world)| (id, world.bounds.clone()))
        .collect();
    let mut linked = Vec::new();
    let mut identities = BTreeSet::new();
    for input in rows {
        let row = record(input)?;
        let id = uri(required(row, "iri")?)?;
        let scene = label(required(row, "scene_key")?)?;
        keys.insert((scene.into(), "10d".into()));
        let world = worlds.get_mut(id).ok_or("World declaration is missing")?;
        let mut portals = Vec::new();
        let mut avatars = Vec::new();
        for item in optional_rows(row, "portals", MAX_RECORDS)? {
            let port = record(item)?;
            fields(
                port,
                &[
                    "iri",
                    "position_m",
                    "rotation_xyzw",
                    "target_world",
                    "target_position_m",
                    "radius_m",
                    "active",
                ],
            )?;
            let portal_id = uri(required(port, "iri")?)?;
            if bounds.contains_key(portal_id) || !identities.insert(portal_id) {
                return Err("Duplicate portal or avatar IRI".into());
            }
            let target = uri(required(port, "target_world")?)?;
            let target_bounds = bounds
                .get(target)
                .ok_or("Portal target world is undeclared")?;
            let [x, y, z] = f32_vector(required(port, "position_m")?)?;
            let [tx, ty, tz] = f32_vector(required(port, "target_position_m")?)?;
            if !inside([x, y, z], world.bounds.min, world.bounds.max)
                || !inside([tx, ty, tz], target_bounds.min, target_bounds.max)
            {
                return Err(
                    "Portal entrance or landing point is outside its declared world".into(),
                );
            }
            let radius = num(required(port, "radius_m")?)?;
            if !(0.01..=1000.0).contains(&radius) {
                return Err("Portal radius must be in 0.01..=1000 meters".into());
            }
            let mut portal = Portal::new(portal_id, target);
            portal.set_position(x, y, z);
            portal.set_target(target, tx, ty, tz);
            portal.rotation = rotation(required(port, "rotation_xyzw")?)?;
            portal.radius = radius as f32;
            if !boolean(port, "active", true)? {
                portal.deactivate();
            }
            graph.fact(id, &format!("{VOCABULARY}portal"), portal_id)?;
            graph.fact(portal_id, &format!("{VOCABULARY}targetWorld"), target)?;
            portals.push(json!({
                "iri": portal.id, "position_m": portal.position, "rotation_xyzw": portal.rotation,
                "target_world": portal.target_world, "target_position_m": portal.target_position,
                "radius_m": portal.radius, "active": portal.active
            }));
            world.add_portal(portal);
        }
        for item in optional_rows(row, "avatars", MAX_RECORDS)? {
            let av = record(item)?;
            fields(
                av,
                &[
                    "iri",
                    "user_did",
                    "position_m",
                    "rotation_xyzw",
                    "model_key",
                    "height_m",
                    "scale",
                    "movement",
                ],
            )?;
            let avatar_id = uri(required(av, "iri")?)?;
            if bounds.contains_key(avatar_id) || !identities.insert(avatar_id) {
                return Err("Duplicate portal or avatar IRI".into());
            }
            let did = uri(required(av, "user_did")?)?;
            if !did.starts_with("did:") {
                return Err("Avatar requires an authored user DID".into());
            }
            let position = f32_vector(required(av, "position_m")?)?;
            if !inside(position, world.bounds.min, world.bounds.max) {
                return Err("Avatar spawn is outside its declared world".into());
            }
            let height = num(required(av, "height_m")?)?;
            let scale = num(required(av, "scale")?)?;
            if !(0.05..=100.0).contains(&height) || !(0.01..=100.0).contains(&scale) {
                return Err("Avatar height or scale exceeds the authoring bounds".into());
            }
            let movement = text(required(av, "movement")?)?;
            let mode = match movement {
                "walk" => MovementMode::Walking,
                "fly" => MovementMode::Flying,
                "teleport" => MovementMode::Teleporting,
                "sit" => MovementMode::Sitting,
                _ => return Err("Avatar movement requires walk, fly, teleport, or sit".into()),
            };
            let model = label(required(av, "model_key")?)?;
            keys.insert((model.into(), "10d".into()));
            let mut avatar = Avatar::new(avatar_id, did);
            avatar.move_to(position[0], position[1], position[2]);
            avatar.rotation = rotation(required(av, "rotation_xyzw")?)?;
            avatar.set_appearance(model, height as f32, scale as f32);
            avatar.set_movement_mode(mode);
            graph.fact(id, &format!("{VOCABULARY}avatar"), avatar_id)?;
            avatars.push(json!({
                "iri": avatar.id, "user_did": avatar.user_did, "position_m": avatar.position,
                "rotation_xyzw": avatar.rotation, "model_key": avatar.appearance.model_id,
                "height_m": avatar.appearance.height, "scale": avatar.appearance.scale, "movement": movement
            }));
            world.add_avatar(avatar);
        }
        graph.fact(root, &format!("{VOCABULARY}world"), id)?;
        linked.push(json!({
            "iri": world.id, "name": world.name, "scene_key": scene,
            "bounds_m": [world.bounds.min, world.bounds.max],
            "gravity_m_s2": world.physics_config.gravity,
            "fixed_timestep_s": world.physics_config.fixed_timestep,
            "substeps": world.physics_config.substeps,
            "portals": portals, "avatars": avatars
        }));
    }
    Ok(json!({"entry_world": entry, "worlds": linked}))
}

fn xr_manifest(root: &str, spec: &Args, graph: &mut ReceiptGraph) -> Result<Json, String> {
    fields(spec, &["reference_space", "bindings"])?;
    let reference = text(required(spec, "reference_space")?)?;
    if !matches!(
        reference,
        "viewer" | "local" | "local-floor" | "bounded-floor"
    ) {
        return Err("Unknown XR reference space".into());
    }
    let mut bindings = Vec::new();
    let mut ids = BTreeSet::new();
    for input in rows(spec, "bindings", MAX_RECORDS)? {
        let binding = record(input)?;
        let id = uri(required(binding, "iri")?)?;
        if !ids.insert(id) {
            return Err("Duplicate XR binding IRI".into());
        }
        let target = uri(required(binding, "target_iri")?)?;
        let kind = text(required(binding, "kind")?)?;
        let data = match kind {
            "head_pose" | "gaze_ray" => {
                fields(binding, &["iri", "kind", "target_iri"])?;
                json!({})
            }
            "hand_skeleton" => {
                fields(binding, &["iri", "kind", "target_iri", "handedness"])?;
                let hand = text(required(binding, "handedness")?)?;
                if !matches!(hand, "left" | "right") {
                    return Err("XR hand skeleton requires left or right handedness".into());
                }
                json!({"handedness": hand})
            }
            "haptic_pattern" => {
                fields(binding, &["iri", "kind", "target_iri", "pulses"])?;
                let mut duration = 0.0;
                let mut pulses = Vec::new();
                for value in rows(binding, "pulses", 32)? {
                    let [time, amplitude] = vector::<2>(value)?;
                    if !(0.001..=2.0).contains(&time) || !(0.0..=1.0).contains(&amplitude) {
                        return Err("Invalid XR haptic [duration_s, amplitude]".into());
                    }
                    duration += time;
                    pulses.push([time, amplitude]);
                }
                if duration > 30.0 {
                    return Err("XR haptic pattern exceeds 30 seconds".into());
                }
                json!({"pulses": pulses})
            }
            "biosignal" => {
                fields(
                    binding,
                    &[
                        "iri",
                        "kind",
                        "target_iri",
                        "signal_iri",
                        "consent_iri",
                        "max_rate_hz",
                    ],
                )?;
                let signal = uri(required(binding, "signal_iri")?)?;
                let consent = uri(required(binding, "consent_iri")?)?;
                let rate = num(required(binding, "max_rate_hz")?)?;
                if !(0.01..=1000.0).contains(&rate) {
                    return Err("Biosignal binding rate exceeds the authoring limits".into());
                }
                graph.fact(id, &format!("{VOCABULARY}consentPolicy"), consent)?;
                graph.fact(id, &format!("{VOCABULARY}signal"), signal)?;
                json!({"signal_iri": signal, "consent_iri": consent, "max_rate_hz": rate})
            }
            _ => return Err("Unknown XR binding kind".into()),
        };
        graph.fact(root, &format!("{VOCABULARY}binding"), id)?;
        graph.fact(id, &format!("{VOCABULARY}target"), target)?;
        bindings.push(json!({"iri": id, "kind": kind, "target_iri": target, "config": data}));
    }
    Ok(json!({"reference_space": reference, "bindings": bindings}))
}

fn rows<'a>(args: &'a Args, key: &str, limit: usize) -> Result<&'a [Value], String> {
    let values = list(required(args, key)?)?;
    if values.is_empty() || values.len() > limit {
        return Err(format!("Live {key} requires between 1 and {limit} rows"));
    }
    Ok(values)
}

fn optional_rows<'a>(args: &'a Args, key: &str, limit: usize) -> Result<&'a [Value], String> {
    let rows = args.get(key).map(list).transpose()?.unwrap_or(&[]);
    if rows.len() > limit {
        return Err(format!("Live {key} exceeds {limit} rows"));
    }
    Ok(rows)
}

fn f32_vector<const N: usize>(value: &Value) -> Result<[f32; N], String> {
    let point = vector::<N>(value)?;
    if point.iter().any(|v| v.abs() > 1_000_000.0) {
        return Err("Live world vector exceeds the finite local coordinate bounds".into());
    }
    Ok(point.map(|v| v as f32))
}

fn rotation(value: &Value) -> Result<[f32; 4], String> {
    let rotation = f32_vector(value)?;
    let norm = rotation.iter().map(|v| v * v).sum::<f32>();
    if (norm - 1.0).abs() > 1e-4 {
        return Err("Portal and avatar rotations require normalized XYZW quaternions".into());
    }
    Ok(rotation)
}

fn inside(point: [f32; 3], min: [f32; 3], max: [f32; 3]) -> bool {
    (0..3).all(|axis| (min[axis]..=max[axis]).contains(&point[axis]))
}

fn label(value: &Value) -> Result<&str, String> {
    let value = text(value)?;
    if value.is_empty() || value.len() > 4096 || value.chars().any(char::is_control) {
        return Err("Live label must be non-empty bounded Unicode text".into());
    }
    Ok(value)
}

fn uri(value: &Value) -> Result<&str, String> {
    let value = label(value)?;
    let (scheme, body) = value
        .split_once(':')
        .ok_or("Live identity requires an absolute RDF IRI")?;
    if !scheme.starts_with(|c: char| c.is_ascii_alphabetic())
        || !scheme
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"+.-".contains(&b))
        || body.is_empty()
        || body
            .chars()
            .any(|c| c.is_whitespace() || "<>{}|\\^\"".contains(c))
    {
        return Err("Live identity requires an absolute RDF IRI".into());
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn string(value: &str) -> Value {
        Value::String(value.into())
    }

    #[test]
    fn dmx_fixture_uses_the_last_universe_address_without_shifting_it() {
        let fixture = Value::Record(BTreeMap::from([
            ("iri".into(), string("urn:test:fixture")),
            ("name".into(), string("Spot")),
            ("type".into(), string("spotlight")),
            ("universe".into(), Value::U64(1)),
            ("start_address".into(), Value::U64(512)),
            ("channel_count".into(), Value::U64(1)),
            (
                "functions".into(),
                Value::Record(BTreeMap::from([("intensity".into(), Value::U64(1))])),
            ),
        ]));
        let cue = Value::Record(BTreeMap::from([
            ("iri".into(), string("urn:test:cue")),
            ("name".into(), string("Fade up")),
            ("duration_s".into(), Value::F64(1.0)),
            ("fade_in_s".into(), Value::F64(0.5)),
            ("fade_out_s".into(), Value::F64(0.0)),
            (
                "fixture_values".into(),
                Value::List(vec![Value::List(vec![
                    string("urn:test:fixture"),
                    string("intensity"),
                    Value::U64(123),
                ])]),
            ),
        ]));
        let args = BTreeMap::from([
            ("name".into(), string("Show")),
            ("universes".into(), Value::List(vec![Value::U64(1)])),
            ("fixtures".into(), Value::List(vec![fixture])),
            ("cues".into(), Value::List(vec![cue])),
        ]);
        let mut graph = ReceiptGraph::new(VOCABULARY);
        let manifest = dmx_manifest("urn:test:show", &args, &mut graph).unwrap();
        assert_eq!(manifest["cues"][0]["channels"], json!([[1, 512, 123]]));
        assert_eq!(
            manifest["universes_at_last_cue"][0]["nonzero_channels"],
            json!([[512, 123]])
        );
    }

    #[test]
    fn hbbtv_rejects_an_unresolved_navigation_target() {
        let page = Value::Record(BTreeMap::from([
            ("iri".into(), string("urn:test:start")),
            ("title".into(), string("Start")),
            ("html_key".into(), string("start.html")),
            (
                "navigation".into(),
                Value::List(vec![Value::Record(BTreeMap::from([
                    ("event".into(), string("red")),
                    ("target".into(), string("urn:test:missing")),
                ]))]),
            ),
        ]));
        let args = BTreeMap::from([
            ("name".into(), string("Program")),
            ("pages".into(), Value::List(vec![page])),
            ("start_page".into(), string("urn:test:start")),
        ]);
        let mut graph = ReceiptGraph::new(VOCABULARY);
        assert!(
            hbbtv_manifest("urn:test:program", &args, &mut graph, &mut BTreeSet::new())
                .unwrap_err()
                .contains("undeclared page")
        );
    }
}
