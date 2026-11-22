//! Request-scoped WellFair fixture projections and bounded molecular analysis.

use super::{fields, from_json, record, required, text, unsigned, Args, AssetHost, Value};
use crate::{list, num};
use qualia_core_db::domains::chemical::organic_chemistry::{
    atom_economy, circular_fingerprint, compute_descriptors, detect_functional_groups,
    e_factor, parse_smiles, Molecule,
};
use std::collections::{BTreeMap, BTreeSet};
use wellfare_core::{
    clinical::{advance_claim, build_clinical_report_envelope, ClaimEvent, ClaimStatus, ClinicalReport},
    guardianship::{derive_status, GuardianshipProposal, GuardianshipVote},
    mental_wellbeing::{build_therapy_note_envelope, TherapyNote},
    record::SensitivityClass,
    sleep_analytics::{compute_sleep_debt, compute_weekly_heatmap, SleepNightSample},
};

const NIGHTS_LIMIT: usize = 366;
const GUARDIANS_LIMIT: usize = 64;
const EVENTS_LIMIT: usize = 512;
const SMILES_LIMIT: usize = 192;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "sleep_debt_fixture" => sleep_debt(host, params),
        "clinical_claim_fixture" => clinical_claim(host, params),
        "therapy_note_fixture" => therapy_note(host, params),
        "guardianship_veto_fixture" => guardianship(host, params),
        "organic_descriptors" => organic_descriptors(host, params),
        "green_mass_metrics" => green_mass_metrics(params),
        _ => Err(format!("Unsupported WellFair or organic chemistry operation: {op}")),
    }
}

fn bounded_number(value: &Value, lower: f64, upper: f64) -> Result<f64, String> {
    let value = num(value)?;
    if !(lower..=upper).contains(&value) {
        return Err("Numeric input is outside the declared fixture range".into());
    }
    Ok(value)
}

fn explicit_time(value: &Value) -> Result<u32, String> {
    u32::try_from(unsigned(value)?).map_err(|_| "Fixture time does not fit u32 Unix seconds".into())
}

fn fixture_id(value: &Value) -> Result<&str, String> {
    let input = text(value)?;
    let suffix = input.strip_prefix("fixture:").ok_or("Authored identities require a fixture: prefix")?;
    if suffix.is_empty() || suffix.len() > 96 || !suffix.chars()
        .all(|ch| ch.is_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err("Invalid fixture identity".into());
    }
    Ok(input)
}

fn identity_set(value: &Value) -> Result<BTreeSet<&str>, String> {
    let entries = list(value)?;
    if entries.len() > GUARDIANS_LIMIT {
        return Err("Fixture identity count exceeds the production limit".into());
    }
    let result: BTreeSet<&str> = entries.iter().map(fixture_id).collect::<Result<_, _>>()?;
    if result.len() != entries.len() {
        return Err("Fixture identities must be unique".into());
    }
    Ok(result)
}

fn sleep_debt(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["nights", "target_min"])?;
    let nights = list(required(params, "nights")?)?;
    if nights.len() > NIGHTS_LIMIT {
        return Err("Sleep fixture exceeds the recorded-night limit".into());
    }
    let target_min = bounded_number(required(params, "target_min")?, 1.0, 1440.0)?;
    host.charge(nights.len(), nights.len() * 96)?;
    let mut samples = Vec::with_capacity(nights.len());
    for night in nights {
        let night = record(night)?;
        fields(night, &["night_unix", "duration_min", "efficiency"])?;
        let efficiency = night.get("efficiency").filter(|value| !matches!(value, Value::Null))
            .map(|value| bounded_number(value, 0.0, 100.0)).transpose()?;
        samples.push(SleepNightSample {
            night_unix: explicit_time(required(night, "night_unix")?)?,
            duration_min: bounded_number(required(night, "duration_min")?, 0.0, 1440.0)?,
            efficiency,
        });
    }
    samples.sort_by_key(|sample| sample.night_unix);
    if samples.windows(2).any(|pair| pair[0].night_unix == pair[1].night_unix) {
        return Err("A sleep fixture may contain only one sample per night identifier".into());
    }
    let debt = compute_sleep_debt(&samples, target_min);
    let heatmap = compute_weekly_heatmap(&samples, target_min);
    Ok(from_json(serde_json::json!({ "debt": debt, "heatmap": heatmap, "fixture": true })))
}

fn clinical_claim(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["report_id", "author_id", "clinician_ids", "observed_unix", "events"])?;
    let report_id = fixture_id(required(params, "report_id")?)?;
    let author = fixture_id(required(params, "author_id")?)?;
    let clinicians = identity_set(required(params, "clinician_ids")?)?;
    if clinicians.contains(author) {
        return Err("A fixture author cannot confirm their own clinical claim".into());
    }
    let observed_unix = explicit_time(required(params, "observed_unix")?)?;
    let events = list(required(params, "events")?)?;
    if events.len() > EVENTS_LIMIT {
        return Err("Clinical fixture exceeds the event budget".into());
    }
    host.charge(events.len(), events.len() * 128)?;
    let mut trace = Vec::with_capacity(events.len());
    let mut claim_status = ClaimStatus::Draft;
    let mut last_time = observed_unix;
    for event in events {
        let item = record(event)?;
        fields(item, &["actor_id", "event", "at_unix"])?;
        let actor = fixture_id(required(item, "actor_id")?)?;
        let time = explicit_time(required(item, "at_unix")?)?;
        if time < last_time {
            return Err("Clinical fixture events require nondecreasing time".into());
        }
        last_time = time;
        let event = text(required(item, "event")?)?;
        let (event, authorized) = match event {
            "submit" => (ClaimEvent::Submit, actor == author),
            "confirm" => (ClaimEvent::Confirm, clinicians.contains(actor)),
            "dispute" => (ClaimEvent::Dispute, actor == author || clinicians.contains(actor)),
            "supersede" => (ClaimEvent::Supersede, actor == author || clinicians.contains(actor)),
            "revise" => (ClaimEvent::Revise, actor == author),
            _ => return Err("Unknown clinical fixture event".into()),
        };
        let before = claim_status;
        if authorized {
            claim_status = advance_claim(claim_status, event);
        }
        trace.push(serde_json::json!({
            "event": event,
            "allowed_fixture_actor": authorized,
            "status": claim_status,
            "advanced": before != claim_status,
        }));
    }
    let fixture = ClinicalReport {
        id: report_id.to_owned(),
        title: String::new(),
        report_type: Default::default(),
        author_label: None,
        observed_at_unix: observed_unix,
        observed_at_instant: None,
        body: String::new(),
        attachment_blob_hash: None,
        claim_status,
    };
    let projected = build_clinical_report_envelope(&fixture, author, author, last_time, None);
    Ok(from_json(serde_json::json!({
        "fixture": true,
        "status": claim_status,
        "projected_evidence": projected.evidence_type,
        "projected_epistemic_status": projected.epistemic_status,
        "projected_sensitivity": projected.sensitivity,
        "trace": trace,
    })))
}

fn therapy_note(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["note_id", "author_id", "asserted_unix", "notes"])?;
    let note_id = fixture_id(required(params, "note_id")?)?;
    let author = fixture_id(required(params, "author_id")?)?;
    let asserted_unix = explicit_time(required(params, "asserted_unix")?)?;
    let notes = text(required(params, "notes")?)?;
    if notes.trim().is_empty() || notes.len() > 8192 {
        return Err("Therapy fixture requires bounded, explicitly supplied note text".into());
    }
    host.charge(notes.len(), notes.len())?;
    let note = TherapyNote {
        id: note_id.to_owned(),
        provider_label: None,
        session_date_unix: None,
        session_date_instant: None,
        notes: notes.to_owned(),
    };
    let class = build_therapy_note_envelope(&note, author, author, asserted_unix, None);
    if class.sensitivity != SensitivityClass::Classified {
        return Err("WellFair therapy note lost its Classified sensitivity".into());
    }
    Ok(from_json(serde_json::json!({
        "fixture": true,
        "sensitivity": class.sensitivity,
        "evidence_type": class.evidence_type,
    })))
}

fn guardianship(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["proposal_id", "principal_id", "proxy_id", "guardians", "threshold", "created_unix", "votes"])?;
    let proposal_id = fixture_id(required(params, "proposal_id")?)?;
    let principal = fixture_id(required(params, "principal_id")?)?;
    let proxy = fixture_id(required(params, "proxy_id")?)?;
    let guardians = identity_set(required(params, "guardians")?)?;
    if principal == proxy || guardians.contains(proxy) {
        return Err("Fixture proxy must differ from the principal and approving guardians".into());
    }
    let threshold = unsigned(required(params, "threshold")?)?;
    if threshold == 0 || threshold > guardians.len() as u64 {
        return Err("Fixture guardian threshold must fit the distinct guardian roster".into());
    }
    let created_unix = explicit_time(required(params, "created_unix")?)?;
    let input_votes = list(required(params, "votes")?)?;
    if input_votes.len() > EVENTS_LIMIT {
        return Err("Guardianship fixture exceeds the vote budget".into());
    }
    host.charge(input_votes.len() * guardians.len(), input_votes.len() * 256)?;
    let mut by_id = BTreeMap::new();
    let mut by_time = BTreeMap::new();
    for vote in input_votes {
        let vote = record(vote)?;
        fields(vote, &["vote_id", "guardian_id", "approve", "voted_unix"])?;
        let id = fixture_id(required(vote, "vote_id")?)?;
        let guardian = fixture_id(required(vote, "guardian_id")?)?;
        let voted_unix = explicit_time(required(vote, "voted_unix")?)?;
        let Value::Bool(approve) = required(vote, "approve")? else {
            return Err("Guardianship approval requires a boolean".into());
        };
        if !guardians.contains(guardian) || voted_unix < created_unix {
            return Err("Guardianship fixture vote must name a designated guardian and valid time".into());
        }
        if by_id.insert(id, (guardian, voted_unix, *approve)).is_some_and(|previous|
            previous != (guardian, voted_unix, *approve))
        {
            return Err("Conflicting immutable vote identities in guardianship fixture".into());
        }
        if let Some((_, previous)) = by_time.insert((guardian, voted_unix), (id, *approve)) {
            if previous != *approve {
                return Err("Conflicting simultaneous votes from one guardian".into());
            }
        }
    }
    let proposal = GuardianshipProposal {
        id: proposal_id.to_owned(),
        principal_did: principal.to_owned(),
        proxy_did: proxy.to_owned(),
        threshold: threshold as u8,
        escrowed_kind: "fixture".into(),
        escrowed_envelope_json: String::new(),
        escrowed_summary: None,
        reason: String::new(),
        created_unix,
        created_instant: None,
    };
    let votes: Vec<_> = by_time.into_iter().map(|((guardian, voted_unix), (id, approve))| GuardianshipVote {
        id: id.to_owned(),
        proposal_id: proposal_id.to_owned(),
        guardian_did: guardian.to_owned(),
        approve,
        reason: None,
        voted_unix,
        voted_instant: None,
    }).collect();
    Ok(from_json(serde_json::json!({
        "fixture": true,
        "status": derive_status(&proposal, &votes),
    })))
}

fn supported_molecule(input: &str) -> Result<Molecule, String> {
    if input.is_empty() || input.len() > SMILES_LIMIT {
        return Err("SMILES fixture exceeds the supported input extent".into());
    }
    let mut bytes = input.bytes().peekable();
    let mut branch_depth = 0usize;
    let mut branch_open = false;
    let mut has_atom = false;
    let mut bond_pending = false;
    while let Some(byte) = bytes.next() {
        match byte {
            b'C' | b'B' | b'N' | b'O' | b'P' | b'S' | b'F' | b'I'
                | b'c' | b'n' | b'o' | b'p' | b's' => {
                if matches!((byte, bytes.peek().copied()), (b'C', Some(b'l')) | (b'B', Some(b'r'))) {
                    bytes.next();
                }
                has_atom = true;
                branch_open = false;
                bond_pending = false;
            }
            b'(' if has_atom && !bond_pending => {
                branch_depth += 1;
                has_atom = false;
                branch_open = true;
            }
            b')' if branch_depth > 0 && has_atom && !bond_pending => branch_depth -= 1,
            b'.' if branch_depth == 0 && has_atom && !bond_pending => has_atom = false,
            b'-' | b'=' | b'#' | b':' if (has_atom || branch_open) && !bond_pending => {
                bond_pending = true;
            }
            b'0'..=b'9' if has_atom => bond_pending = false,
            b'%' if has_atom => {
                if ![bytes.next(), bytes.next()].into_iter().all(|byte| byte.is_some_and(|byte| byte.is_ascii_digit())) {
                    return Err("SMILES ring label requires two decimal digits".into());
                }
                bond_pending = false;
            }
            _ => return Err("SMILES fixture uses syntax outside the supported organic graph subset".into()),
        }
    }
    if branch_depth != 0 || bond_pending || !has_atom {
        return Err("SMILES fixture contains an unfinished structure".into());
    }
    let molecule = parse_smiles(input);
    if !molecule.is_valid {
        return Err("Native SMILES parser rejected the graph".into());
    }
    let mut edges = BTreeSet::new();
    for bond in &molecule.bonds {
        let [a, b] = [bond.atom_a.min(bond.atom_b), bond.atom_a.max(bond.atom_b)];
        if a == b || !edges.insert((a, b)) {
            return Err("SMILES fixture contains a self-bond or repeated edge".into());
        }
    }
    Ok(molecule)
}

fn organic_descriptors(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["smiles", "fingerprint_radius"])?;
    let smiles = text(required(params, "smiles")?)?;
    let radius = params.get("fingerprint_radius").map(unsigned).transpose()?;
    if radius.is_some_and(|radius| radius > 4) || smiles.len() > SMILES_LIMIT {
        return Err("Organic chemistry fixture exceeds the descriptor budget".into());
    }
    host.charge(smiles.len().saturating_pow(2), smiles.len() * 256)?;
    let molecule = supported_molecule(smiles)?;
    let desc = compute_descriptors(&molecule);
    let groups: Vec<_> = detect_functional_groups(&molecule).into_iter()
        .map(|group| format!("{group:?}")).collect::<BTreeSet<_>>().into_iter().collect();
    let fingerprint = radius.map(|radius| circular_fingerprint(&molecule, radius as usize));
    Ok(from_json(serde_json::json!({
        "formula": desc.formula,
        "monoisotopic_molar_mass_g_mol": desc.molecular_weight,
        "heavy_atom_count": desc.heavy_atom_count,
        "hydrogen_bond_donors_estimate": desc.hb_donors,
        "hydrogen_bond_acceptors_estimate": desc.hb_acceptors,
        "rotatable_bonds_estimate": desc.rotatable_bonds,
        "aromatic_ring_count_estimate": desc.aromatic_ring_count,
        "ring_count_estimate": desc.ring_count,
        "logp_crippen_estimate": desc.logp_crippen,
        "tpsa_ertl_estimate_a2": desc.tpsa_ertl,
        "carbon_sp3_fraction_estimate": desc.fraction_csp3,
        "functional_groups": groups,
        "circular_fingerprint": fingerprint,
    })))
}

fn green_mass_metrics(params: &Args) -> Result<Value, String> {
    fields(params, &["reactant_stoich_mw_g_mol", "product_stoich_mw_g_mol", "reactant_charge_kg",
        "solvent_and_auxiliary_kg", "desired_product_kg", "yield_fraction"])?;
    let reactants = list(required(params, "reactant_stoich_mw_g_mol")?)?;
    if reactants.is_empty() || reactants.len() > 64 {
        return Err("Green chemistry metrics require bounded stoichiometric reactant contributions".into());
    }
    let reactants: Vec<_> = reactants.iter().map(|value| bounded_number(value, 1e-12, 1e12))
        .collect::<Result<_, _>>()?;
    let product_mw = bounded_number(required(params, "product_stoich_mw_g_mol")?, 1e-12, 1e12)?;
    let reactant_kg = bounded_number(required(params, "reactant_charge_kg")?, 1e-12, 1e12)?;
    let aux_kg = bounded_number(required(params, "solvent_and_auxiliary_kg")?, 0.0, 1e12)?;
    let product_kg = bounded_number(required(params, "desired_product_kg")?, 1e-12, 1e12)?;
    let yield_fraction = bounded_number(required(params, "yield_fraction")?, 0.0, 1.0)?;
    if product_mw > reactants.iter().sum::<f64>() || product_kg > reactant_kg || yield_fraction == 0.0 {
        return Err("Green chemistry fixture violates charged mass or product yield constraints".into());
    }
    let atom_economy_pct = atom_economy(&reactants, product_mw);
    let total_kg = reactant_kg + aux_kg;
    let uncredited_kg = total_kg - product_kg;
    Ok(from_json(serde_json::json!({
        "atom_economy_pct": atom_economy_pct,
        "yield_corrected_ae_pct": atom_economy_pct * yield_fraction,
        "e_factor_all_uncredited_mass": e_factor(uncredited_kg, product_kg),
        "process_mass_intensity": total_kg / product_kg,
        "reaction_mass_efficiency_pct": 100.0 * product_kg / reactant_kg,
        "uncredited_mass_kg": uncredited_kg,
    })))
}
