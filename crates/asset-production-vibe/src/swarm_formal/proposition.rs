//! Bounded propositional CNF decision procedure with explicit work accounting.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, mem::size_of};

use sha2::{Digest, Sha256};

use crate::{
    fields, integer, list, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    required, Args, AssetHost, Value,
};

use super::{budget, count_value};

const MAX_CLAUSES: usize = 256;
const MAX_VARIABLES: usize = 64;
const MAX_CLAUSE_CHECKS: usize = 8_000_000;
const CONTEXT: &str = "urn:qualia:logic:bounded-propositional-cnf-v1";

#[derive(Clone, Copy)]
struct Clause {
    positive: u64,
    negative: u64,
}

#[derive(Clone, Copy)]
struct Assignment {
    assigned: u64,
    truth: u64,
}

impl Assignment {
    const EMPTY: Self = Self {
        assigned: 0,
        truth: 0,
    };

    fn set(self, mask: u64, truth: bool) -> Self {
        Self {
            assigned: self.assigned | mask,
            truth: if truth {
                self.truth | mask
            } else {
                self.truth & !mask
            },
        }
    }
}

enum Decision {
    Satisfiable(Assignment),
    Unsatisfiable,
    UnknownWorkCap,
}

impl AssetHost<'_> {
    pub(crate) fn solve_cdcl_smt_and_egraph_arena(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "variable_count",
                "clauses",
                "clause_cap",
                "clause_check_cap",
                "arena_budget_bytes",
            ],
        )?;
        let variables = integer(required(args, "variable_count")?, "variable_count")?;
        if !(1..=MAX_VARIABLES).contains(&variables) {
            return Err("This propositional CNF solver admits 1 through 64 variables".into());
        }
        let clause_cap = args
            .get("clause_cap")
            .map(|value| integer(value, "clause_cap"))
            .transpose()?
            .unwrap_or(MAX_CLAUSES);
        let check_cap = args
            .get("clause_check_cap")
            .map(|value| integer(value, "clause_check_cap"))
            .transpose()?
            .unwrap_or(1_000_000);
        if !(1..=MAX_CLAUSES).contains(&clause_cap) || !(1..=MAX_CLAUSE_CHECKS).contains(&check_cap)
        {
            return Err("Propositional CNF clause or clause-check cap is out of range".into());
        }
        let clauses = list(required(args, "clauses")?)?;
        if clauses.len() > clause_cap {
            return Err("Propositional CNF exceeds clause_cap".into());
        }
        let scratch = MAX_CLAUSES * size_of::<Clause>() + MAX_VARIABLES * size_of::<Assignment>();
        budget(args, scratch)?;
        self.charge(check_cap + clauses.len() * MAX_VARIABLES * 2, scratch)?;
        let mut formula = [Clause {
            positive: 0,
            negative: 0,
        }; MAX_CLAUSES];
        for (clause, value) in formula.iter_mut().zip(clauses) {
            let literals = list(value)?;
            if literals.len() > MAX_VARIABLES * 2 {
                return Err("CNF clause exceeds its literal cap".into());
            }
            for value in literals {
                let (variable, negative) = match value {
                    Value::I64(n) => (n.unsigned_abs(), *n < 0),
                    Value::U64(n) => (*n, false),
                    _ => return Err("A CNF literal must be a signed or unsigned integer".into()),
                };
                if variable == 0 || variable > variables as u64 {
                    return Err(
                        "A CNF literal is outside the declared 1-based variable range".into(),
                    );
                }
                let bit = 1 << (variable - 1);
                if negative {
                    clause.negative |= bit;
                } else {
                    clause.positive |= bit;
                }
            }
        }
        let formula = &formula[..clauses.len()];
        let (status, checks) = solve(formula, check_cap);
        let (status, assignment) = match status {
            Decision::Satisfiable(assignment) => {
                // Unassigned variables can be set false after every clause is already satisfied.
                let complete = Assignment {
                    assigned: u64::MAX,
                    truth: assignment.truth,
                };
                if formula.iter().any(|clause| !satisfied(*clause, complete)) {
                    return Err("Internal CNF witness failed verification".into());
                }
                let answers = (0..variables)
                    .map(|offset| Value::Bool(assignment.truth & (1 << offset) != 0))
                    .collect();
                ("satisfiable", Value::List(answers))
            }
            Decision::Unsatisfiable => ("unsatisfiable", Value::Null),
            Decision::UnknownWorkCap => ("unknown_work_cap", Value::Null),
        };
        let mut sha = Sha256::new();
        sha.update(b"Qualia propositional CNF v1\0");
        sha.update((variables as u64).to_le_bytes());
        sha.update((formula.len() as u64).to_le_bytes());
        for clause in formula {
            sha.update(clause.positive.to_le_bytes());
            sha.update(clause.negative.to_le_bytes());
        }
        let formula_iri = format!("urn:sha256:{:x}", sha.finalize());
        let mut receipt = ReceiptGraph::new(CONTEXT);
        receipt.fact(
            &formula_iri,
            "urn:qualia:logic:cnf-decision",
            &format!("{CONTEXT}:{status}"),
        )?;
        receipt.fact(
            &formula_iri,
            "urn:qualia:logic:clause-check-count",
            &format!("urn:qualia:integer:{checks}"),
        )?;
        if let Value::List(values) = &assignment {
            for (slot, value) in values.iter().enumerate() {
                let variable = format!("{formula_iri}:variable:{}", slot + 1);
                receipt.fact(
                    &formula_iri,
                    "urn:qualia:logic:assigned-variable",
                    &variable,
                )?;
                receipt.fact(
                    &variable,
                    "urn:qualia:logic:boolean-value",
                    if matches!(value, Value::Bool(true)) {
                        "urn:qualia:boolean:true"
                    } else {
                        "urn:qualia:boolean:false"
                    },
                )?;
            }
        }
        let sealed = self.encode("q42", receipt.seal(self.limits.resource_bytes)?)?;
        Ok(Value::Record(BTreeMap::from([
            ("receipt".into(), sealed),
            ("status".into(), Value::String(status.into())),
            ("assignment".into(), assignment),
            ("clause_checks".into(), count_value(checks)),
        ])))
    }
}

fn satisfied(clause: Clause, assignment: Assignment) -> bool {
    clause.positive & clause.negative != 0
        || (clause.positive & assignment.truth) != 0
        || (clause.negative & assignment.assigned & !assignment.truth) != 0
}

fn solve(clauses: &[Clause], cap: usize) -> (Decision, usize) {
    // Every deferred branch already assigns one new variable. At most one frame
    // per variable is live, with no heap allocation or recursive call stack.
    let mut deferred = [Assignment::EMPTY; MAX_VARIABLES];
    let mut pending = 0;
    let mut assignment = Assignment::EMPTY;
    let mut checks = 0;
    'search: loop {
        let mut choose = 0_u64;
        for clause in clauses {
            if checks == cap {
                return (Decision::UnknownWorkCap, checks);
            }
            checks += 1;
            if satisfied(*clause, assignment) {
                continue;
            }
            let unset = (clause.positive | clause.negative) & !assignment.assigned;
            if unset == 0 {
                if pending == 0 {
                    return (Decision::Unsatisfiable, checks);
                }
                pending -= 1;
                assignment = deferred[pending];
                continue 'search;
            }
            if unset.is_power_of_two() {
                assignment = assignment.set(unset, clause.positive & unset != 0);
                continue 'search;
            }
            if choose == 0 {
                choose = unset & unset.wrapping_neg();
            }
        }
        if choose == 0 {
            return (Decision::Satisfiable(assignment), checks);
        }
        deferred[pending] = assignment.set(choose, false);
        pending += 1;
        assignment = assignment.set(choose, true);
    }
}
