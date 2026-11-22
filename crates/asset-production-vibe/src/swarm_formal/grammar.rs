//! Finite linear-logic multiset reachability for story world grammars.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{
    collections::{BTreeMap, BTreeSet},
    mem::size_of,
};

use qualia_core_db::bundle::semantic_model::resource_identity_meta;
use sha2::{Digest, Sha256};

use crate::{
    fields, integer, list, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    record, required, text, Args, AssetHost, Value,
};

use super::{budget, count_value};

const MAX_ATOMS: usize = 32;
const MAX_RULES: usize = 64;
const MAX_STATES: usize = 8192;
const MAX_PROBES: usize = 64;
const CONTEXT: &str = "urn:qualia:narrative:linear-multiset-reachability-v1";

type Tokens = [u16; MAX_ATOMS];

#[derive(Clone, Copy)]
struct Node {
    inventory: Tokens,
    parent: u32,
    via_rule: u16,
    depth: u16,
}

impl Node {
    const EMPTY: Self = Self {
        inventory: [0; MAX_ATOMS],
        parent: u32::MAX,
        via_rule: 0,
        depth: 0,
    };
}

struct Rule<'a> {
    iri: &'a str,
    consume: Tokens,
    produce: Tokens,
}

impl AssetHost<'_> {
    pub(crate) fn solve_linear_logic_world_grammar(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "atoms",
                "initial",
                "goal",
                "rules",
                "plan_horizon_steps",
                "state_cap",
                "arena_budget_bytes",
            ],
        )?;
        let atom_input = list(required(args, "atoms")?)?;
        if !(1..=MAX_ATOMS).contains(&atom_input.len()) {
            return Err("This narrative grammar admits 1 through 32 atom identities".into());
        }
        let mut seen = BTreeSet::new();
        let atoms = atom_input
            .iter()
            .map(|value| {
                let iri = bounded_iri(value)?;
                if !seen.insert(iri) {
                    return Err("Repeated atom identity".into());
                }
                Ok(iri)
            })
            .collect::<Result<Vec<_>, String>>()?;
        let initial = tokens(required(args, "initial")?, atoms.len())?;
        let goal = tokens(required(args, "goal")?, atoms.len())?;
        let rule_input = list(required(args, "rules")?)?;
        if rule_input.len() > MAX_RULES {
            return Err("This narrative grammar admits at most 64 rewrite rules".into());
        }
        let rules = rule_input
            .iter()
            .map(|value| {
                let row = record(value)?;
                fields(row, &["iri", "consume", "produce"])?;
                let iri = bounded_iri(required(row, "iri")?)?;
                if !seen.insert(iri) {
                    return Err("Repeated atom or rule identity".into());
                }
                Ok(Rule {
                    iri,
                    consume: tokens(required(row, "consume")?, atoms.len())?,
                    produce: tokens(required(row, "produce")?, atoms.len())?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let horizon = integer(required(args, "plan_horizon_steps")?, "plan_horizon_steps")?;
        if !(1..=4096).contains(&horizon) {
            return Err("plan_horizon_steps must be in 1..=4096".into());
        }
        let state_cap = args
            .get("state_cap")
            .map(|value| integer(value, "state_cap"))
            .transpose()?
            .unwrap_or(2048);
        if !(1..=MAX_STATES).contains(&state_cap) {
            return Err("state_cap must be in 1..=8192".into());
        }
        let hash_slots = state_cap.next_power_of_two() * 2;
        let scratch_bytes = state_cap * size_of::<Node>()
            + hash_slots * size_of::<u32>()
            + (rules.len() + 2) * size_of::<Rule<'_>>();
        budget(args, scratch_bytes)?;
        let work = state_cap * rules.len().max(1) * atoms.len() * 6;
        self.charge(work, scratch_bytes)?;
        let mut nodes = vec![Node::EMPTY; state_cap];
        let mut slots = vec![u32::MAX; hash_slots];
        let (found, visited) = search(
            initial,
            &goal,
            &rules,
            atoms.len(),
            horizon,
            &mut nodes,
            &mut slots,
            work,
        )?;
        let status = if found.is_some() {
            "reachable"
        } else {
            "unreachable_within_horizon"
        };
        let mut path = Vec::new();
        if let Some(mut at) = found {
            while nodes[at].parent != u32::MAX {
                path.push(nodes[at].via_rule as usize);
                at = nodes[at].parent as usize;
            }
            path.reverse();
        }
        let problem = problem_identity(&atoms, &initial, &goal, &rules, horizon);
        let state = format!("{problem}:{status}");
        let mut receipt = ReceiptGraph::new(CONTEXT);
        receipt.fact(&problem, "urn:qualia:narrative:result", &state)?;
        for (step, &rule) in path.iter().enumerate() {
            let part = format!("{state}:step:{step}");
            receipt.fact(&state, "urn:qualia:narrative:plan-step", &part)?;
            receipt.fact(&part, "urn:qualia:narrative:applied-rule", rules[rule].iri)?;
        }
        let sealed = self.encode("q42", receipt.seal(self.limits.resource_bytes)?)?;
        let final_state = found
            .map(|index| {
                Value::List(
                    nodes[index].inventory[..atoms.len()]
                        .iter()
                        .map(|&value| Value::U64(value.into()))
                        .collect(),
                )
            })
            .unwrap_or(Value::Null);
        Ok(Value::Record(BTreeMap::from([
            ("receipt".into(), sealed),
            ("status".into(), Value::String(status.into())),
            ("visited_states".into(), count_value(visited)),
            ("final_state".into(), final_state),
            (
                "rule_trace".into(),
                Value::List(
                    path.into_iter()
                        .map(|rule| Value::String(rules[rule].iri.into()))
                        .collect(),
                ),
            ),
        ])))
    }
}

fn bounded_iri(value: &Value) -> Result<&str, String> {
    let iri = text(value)?;
    if iri.len() > 1024 {
        return Err("Narrative IRI exceeds the lexical cap".into());
    }
    resource_identity_meta(iri).map_err(|error| error.to_string())?;
    Ok(iri)
}

fn tokens(value: &Value, atoms: usize) -> Result<Tokens, String> {
    let values = list(value)?;
    if values.len() != atoms {
        return Err("Token vector length must equal the atom count".into());
    }
    let mut result = [0; MAX_ATOMS];
    for (count, input) in result.iter_mut().zip(values) {
        *count = u16::try_from(integer(input, "token multiplicity")?)
            .map_err(|_| "Token multiplicity exceeds 65535")?;
    }
    Ok(result)
}

fn includes(state: &[u16], costs: &[u16]) -> bool {
    state.iter().zip(costs).all(|(have, need)| have >= need)
}

fn search(
    initial: Tokens,
    goal: &Tokens,
    rules: &[Rule<'_>],
    atoms: usize,
    horizon: usize,
    nodes: &mut [Node],
    slots: &mut [u32],
    mut work: usize,
) -> Result<(Option<usize>, usize), String> {
    nodes[0].inventory = initial;
    let first_slot = slot(&initial, atoms, nodes, slots, &mut work)?.0;
    slots[first_slot] = 0;
    let (mut read, mut written) = (0, 1);
    while read < written {
        let node = nodes[read];
        if includes(&node.inventory[..atoms], &goal[..atoms]) {
            return Ok((Some(read), written));
        }
        if usize::from(node.depth) < horizon {
            for (rule_id, rule) in rules.iter().enumerate() {
                debit(&mut work, atoms)?;
                if !includes(&node.inventory[..atoms], &rule.consume[..atoms]) {
                    continue;
                }
                debit(&mut work, atoms)?;
                let mut next = node.inventory;
                for (atom, amount) in next[..atoms].iter_mut().enumerate() {
                    *amount = (*amount - rule.consume[atom])
                        .checked_add(rule.produce[atom])
                        .ok_or("A reachable state exceeds the token multiplicity cap")?;
                }
                let (key, was_seen) = slot(&next, atoms, nodes, slots, &mut work)?;
                if was_seen {
                    continue;
                }
                let target = nodes
                    .get_mut(written)
                    .ok_or("Narrative search reached its state cap before deciding reachability")?;
                *target = Node {
                    inventory: next,
                    parent: read as u32,
                    via_rule: rule_id as u16,
                    depth: node.depth + 1,
                };
                slots[key] = written as u32;
                written += 1;
                if includes(&next[..atoms], &goal[..atoms]) {
                    return Ok((Some(written - 1), written));
                }
            }
        }
        read += 1;
    }
    Ok((None, written))
}

fn slot(
    inventory: &Tokens,
    atoms: usize,
    nodes: &[Node],
    slots: &[u32],
    work: &mut usize,
) -> Result<(usize, bool), String> {
    debit(work, atoms)?;
    let hash = inventory[..atoms]
        .iter()
        .fold(0xcbf29ce484222325u64, |hash, value| {
            (hash ^ u64::from(*value)).wrapping_mul(0x100000001b3)
        });
    let mask = slots.len() - 1;
    let mut at = hash as usize & mask;
    for _ in 0..MAX_PROBES {
        let value = slots[at];
        if value == u32::MAX {
            return Ok((at, false));
        }
        debit(work, atoms)?;
        if nodes[value as usize].inventory == *inventory {
            return Ok((at, true));
        }
        at = (at + 1) & mask;
    }
    Err("Narrative search reached its state-index probe cap before deciding reachability".into())
}

fn debit(work: &mut usize, amount: usize) -> Result<(), String> {
    *work = work
        .checked_sub(amount)
        .ok_or("Narrative search exhausted its work budget before deciding reachability")?;
    Ok(())
}

fn problem_identity(
    atoms: &[&str],
    initial: &Tokens,
    goal: &Tokens,
    rules: &[Rule<'_>],
    horizon: usize,
) -> String {
    let mut hash = Sha256::new();
    hash.update(b"Qualia linear narrative v1\0");
    hash.update((horizon as u64).to_le_bytes());
    hash.update((atoms.len() as u64).to_le_bytes());
    for atom in atoms {
        hash.update((atom.len() as u64).to_le_bytes());
        hash.update(atom.as_bytes());
    }
    for vector in [initial, goal] {
        for value in &vector[..atoms.len()] {
            hash.update(value.to_le_bytes());
        }
    }
    for rule in rules {
        hash.update((rule.iri.len() as u64).to_le_bytes());
        hash.update(rule.iri.as_bytes());
        for vector in [&rule.consume, &rule.produce] {
            for value in &vector[..atoms.len()] {
                hash.update(value.to_le_bytes());
            }
        }
    }
    format!("urn:sha256:{:x}", hash.finalize())
}
