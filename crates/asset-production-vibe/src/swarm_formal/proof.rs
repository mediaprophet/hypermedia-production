//! Predicative dependent Π kernel with bounded β-conversion.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, mem::size_of};

use sha2::{Digest, Sha256};

use crate::{
    fields, integer, list, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    record, required, text, Args, AssetHost, Value,
};

use super::{budget, count_value};

const MAX_INPUT_TERMS: usize = 4096;
const MAX_ARENA_TERMS: usize = 65_536;
const MAX_DEPTH: usize = 64;
const MAX_FUEL: usize = 10_000_000;
const CONTEXT: &str = "urn:qualia:logic:predicative-dependent-pi-v1";
type Id = usize;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Term {
    Type(u8),
    Var(u16),
    Pi(Id, Id),
    Lambda(Id, Id),
    App(Id, Id),
}

struct Context {
    assumptions: [Id; MAX_DEPTH],
    length: usize,
}

impl Context {
    fn push(&mut self, domain: Id) -> Result<(), String> {
        let slot = self
            .assumptions
            .get_mut(self.length)
            .ok_or("Dependent proof exceeded the binder-depth cap")?;
        *slot = domain;
        self.length += 1;
        Ok(())
    }
}

struct Kernel {
    terms: Vec<Term>,
    cap: usize,
    remaining: usize,
}

impl AssetHost<'_> {
    pub(crate) fn check_cic_dependent_proof_kernel(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "terms",
                "expression",
                "expected_type",
                "max_expr_terms",
                "reduction_fuel",
                "arena_budget_bytes",
            ],
        )?;
        let input = list(required(args, "terms")?)?;
        if !(1..=MAX_INPUT_TERMS).contains(&input.len()) {
            return Err("The dependent Π branch admits 1 through 4096 input terms".into());
        }
        let cap = args
            .get("max_expr_terms")
            .map(|value| integer(value, "max_expr_terms"))
            .transpose()?
            .unwrap_or(8192);
        let fuel = args
            .get("reduction_fuel")
            .map(|value| integer(value, "reduction_fuel"))
            .transpose()?
            .unwrap_or(200_000);
        if cap < input.len() || cap > MAX_ARENA_TERMS || !(1..=MAX_FUEL).contains(&fuel) {
            return Err(
                "Dependent proof term or reduction cap is outside the admitted range".into(),
            );
        }
        let expression = integer(required(args, "expression")?, "expression")?;
        let expected = integer(required(args, "expected_type")?, "expected_type")?;
        let scratch = cap * size_of::<Term>() + 256 * MAX_DEPTH + size_of::<Context>();
        budget(args, scratch)?;
        self.charge(fuel + input.len(), scratch)?;
        if expression >= input.len() || expected >= input.len() {
            return Err("Expression and expected type must index the authored term table".into());
        }
        let mut kernel = Kernel {
            terms: Vec::with_capacity(cap),
            cap,
            remaining: fuel,
        };
        for (id, term) in input.iter().enumerate() {
            kernel.intern(decode_term(term, id)?)?;
        }
        let problem = problem_identity(&kernel.terms, expression, expected);
        let mut context = Context {
            assumptions: [0; MAX_DEPTH],
            length: 0,
        };
        kernel.ensure_sort(expected, &mut context, 0)?;
        let inferred = kernel.infer(expression, &mut context, 0)?;
        if !kernel.convertible(inferred, expected, 0)? {
            return Err("Inferred term type is not β-convertible to the expected type".into());
        }
        let mut receipt = ReceiptGraph::new(CONTEXT);
        receipt.fact(
            &problem,
            "urn:qualia:logic:checked-dependent-proof",
            CONTEXT,
        )?;
        receipt.fact(
            &problem,
            "urn:qualia:logic:kernel-step-count",
            &format!("urn:qualia:integer:{}", fuel - kernel.remaining),
        )?;
        let sealed = self.encode("q42", receipt.seal(self.limits.resource_bytes)?)?;
        Ok(Value::Record(BTreeMap::from([
            ("receipt".into(), sealed),
            ("verified".into(), Value::Bool(true)),
            ("used_terms".into(), count_value(kernel.terms.len())),
            ("kernel_steps".into(), count_value(fuel - kernel.remaining)),
        ])))
    }
}

fn decode_term(value: &Value, ordinal: usize) -> Result<Term, String> {
    let row = record(value)?;
    let preceding = |name| {
        let index = integer(required(row, name)?, name)?;
        if index >= ordinal {
            return Err("Term child must refer to an earlier authored term".into());
        }
        Ok::<usize, String>(index)
    };
    match text(required(row, "kind")?)? {
        "type" => {
            fields(row, &["kind", "level"])?;
            let level = integer(required(row, "level")?, "level")?;
            if level > 63 {
                return Err("Authored universe level must be in 0..=63".into());
            }
            Ok(Term::Type(level as u8))
        }
        "variable" => {
            fields(row, &["kind", "index"])?;
            let index = integer(required(row, "index")?, "index")?;
            if index >= MAX_DEPTH {
                return Err("De Bruijn index exceeds the authored binder cap".into());
            }
            Ok(Term::Var(index as u16))
        }
        kind @ ("pi" | "lambda") => {
            fields(row, &["kind", "domain", "body"])?;
            let domain = preceding("domain")?;
            let body = preceding("body")?;
            Ok(if kind == "pi" {
                Term::Pi(domain, body)
            } else {
                Term::Lambda(domain, body)
            })
        }
        "apply" => {
            fields(row, &["kind", "function", "argument"])?;
            Ok(Term::App(preceding("function")?, preceding("argument")?))
        }
        _ => Err("Unknown term in the predicative Π fragment".into()),
    }
}

impl Kernel {
    fn intern(&mut self, expression: Term) -> Result<Id, String> {
        let index = self.terms.len();
        if index == self.cap {
            return Err("Dependent proof exhausted its term arena".into());
        }
        self.terms.push(expression);
        Ok(index)
    }

    fn tick(&mut self, depth: usize) -> Result<(), String> {
        if depth > MAX_DEPTH || self.remaining == 0 {
            return Err("Dependent proof exhausted its depth or reduction budget".into());
        }
        self.remaining -= 1;
        Ok(())
    }

    fn ensure_sort(&mut self, term: Id, context: &mut Context, depth: usize) -> Result<u8, String> {
        let sort = self.infer(term, context, depth + 1)?;
        let reduced = self.whnf(sort, depth + 1)?;
        match self.terms[reduced] {
            Term::Type(level) => Ok(level),
            _ => Err("Expected a type in a predicative universe".into()),
        }
    }

    fn infer(&mut self, term: Id, context: &mut Context, depth: usize) -> Result<Id, String> {
        self.tick(depth)?;
        match self.terms[term] {
            Term::Type(level) => self.intern(Term::Type(
                level.checked_add(1).ok_or("Universe level overflow")?,
            )),
            Term::Var(index) => {
                let shift = usize::from(index) + 1;
                let slot = context
                    .length
                    .checked_sub(shift)
                    .ok_or("Unbound De Bruijn variable")?;
                self.shift(context.assumptions[slot], shift as u16, 0, depth + 1)
            }
            Term::Pi(domain, codomain) => {
                let left = self.ensure_sort(domain, context, depth + 1)?;
                context.push(domain)?;
                let right = self.ensure_sort(codomain, context, depth + 1);
                context.length -= 1;
                self.intern(Term::Type(left.max(right?)))
            }
            Term::Lambda(domain, body) => {
                self.ensure_sort(domain, context, depth + 1)?;
                context.push(domain)?;
                let codomain = self.infer(body, context, depth + 1);
                context.length -= 1;
                self.intern(Term::Pi(domain, codomain?))
            }
            Term::App(function, argument) => {
                let function_type = self.infer(function, context, depth + 1)?;
                let head = self.whnf(function_type, depth + 1)?;
                let Term::Pi(domain, codomain) = self.terms[head] else {
                    return Err(
                        "Application function does not have a dependent function type".into(),
                    );
                };
                let argument_type = self.infer(argument, context, depth + 1)?;
                if !self.convertible(argument_type, domain, depth + 1)? {
                    return Err(
                        "Application argument type is not β-convertible to the domain".into(),
                    );
                }
                self.instantiate(codomain, argument, 0, depth + 1)
            }
        }
    }

    fn whnf(&mut self, mut term: Id, depth: usize) -> Result<Id, String> {
        loop {
            self.tick(depth)?;
            let Term::App(function, argument) = self.terms[term] else {
                return Ok(term);
            };
            let head = self.whnf(function, depth + 1)?;
            if let Term::Lambda(_, body) = self.terms[head] {
                term = self.instantiate(body, argument, 0, depth + 1)?;
            } else if head == function {
                return Ok(term);
            } else {
                return self.intern(Term::App(head, argument));
            }
        }
    }

    fn convertible(&mut self, left: Id, right: Id, depth: usize) -> Result<bool, String> {
        self.tick(depth)?;
        if left == right {
            return Ok(true);
        }
        let a = self.whnf(left, depth + 1)?;
        let b = self.whnf(right, depth + 1)?;
        match (self.terms[a], self.terms[b]) {
            (Term::Pi(x, b), Term::Pi(y, c))
            | (Term::Lambda(x, b), Term::Lambda(y, c))
            | (Term::App(x, b), Term::App(y, c)) => {
                Ok(self.convertible(x, y, depth + 1)? && self.convertible(b, c, depth + 1)?)
            }
            (a, b) => Ok(a == b),
        }
    }

    fn shift(&mut self, term: Id, amount: u16, cutoff: u16, depth: usize) -> Result<Id, String> {
        self.tick(depth)?;
        let rewritten = match self.terms[term] {
            Term::Var(index) if index >= cutoff => Term::Var(
                index
                    .checked_add(amount)
                    .ok_or("De Bruijn shift overflow")?,
            ),
            Term::Pi(domain, body) => Term::Pi(
                self.shift(domain, amount, cutoff, depth + 1)?,
                self.shift(body, amount, cutoff + 1, depth + 1)?,
            ),
            Term::Lambda(domain, body) => Term::Lambda(
                self.shift(domain, amount, cutoff, depth + 1)?,
                self.shift(body, amount, cutoff + 1, depth + 1)?,
            ),
            Term::App(function, argument) => Term::App(
                self.shift(function, amount, cutoff, depth + 1)?,
                self.shift(argument, amount, cutoff, depth + 1)?,
            ),
            _ => return Ok(term),
        };
        if rewritten == self.terms[term] {
            Ok(term)
        } else {
            self.intern(rewritten)
        }
    }

    fn instantiate(
        &mut self,
        term: Id,
        argument: Id,
        binder: u16,
        depth: usize,
    ) -> Result<Id, String> {
        self.tick(depth)?;
        let rewritten = match self.terms[term] {
            Term::Var(index) if index == binder => {
                return self.shift(argument, binder, 0, depth + 1)
            }
            Term::Var(index) if index > binder => Term::Var(index - 1),
            Term::Pi(domain, body) => Term::Pi(
                self.instantiate(domain, argument, binder, depth + 1)?,
                self.instantiate(body, argument, binder + 1, depth + 1)?,
            ),
            Term::Lambda(domain, body) => Term::Lambda(
                self.instantiate(domain, argument, binder, depth + 1)?,
                self.instantiate(body, argument, binder + 1, depth + 1)?,
            ),
            Term::App(function, value) => Term::App(
                self.instantiate(function, argument, binder, depth + 1)?,
                self.instantiate(value, argument, binder, depth + 1)?,
            ),
            _ => return Ok(term),
        };
        if rewritten == self.terms[term] {
            Ok(term)
        } else {
            self.intern(rewritten)
        }
    }
}

fn problem_identity(terms: &[Term], root: Id, expected: Id) -> String {
    let mut sha = Sha256::new();
    sha.update(b"Qualia predicative dependent Pi v1\0");
    sha.update((root as u64).to_le_bytes());
    sha.update((expected as u64).to_le_bytes());
    for term in terms {
        let (tag, a, b) = match *term {
            Term::Type(level) => (0, usize::from(level), 0),
            Term::Var(index) => (1, usize::from(index), 0),
            Term::Pi(a, b) => (2, a, b),
            Term::Lambda(a, b) => (3, a, b),
            Term::App(a, b) => (4, a, b),
        };
        sha.update([tag]);
        sha.update((a as u64).to_le_bytes());
        sha.update((b as u64).to_le_bytes());
    }
    format!("urn:sha256:{:x}", sha.finalize())
}
