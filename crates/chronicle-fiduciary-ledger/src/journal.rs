//! Balanced double-entry journal posting and period lock enforcement.
//!
//! Enforces the fundamental accounting identity: `sum(debits) == sum(credits)`.
//! Debits are represented as positive integer amounts; credits as negative.
//! Every write must strictly sum to zero (`sum(lines) == 0`).

use std::collections::BTreeMap;
use crate::accounts::ChartOfAccounts;
use crate::money::{fmt_money, Cents};
use serde::{Deserialize, Serialize};

/// An individual line in a double-entry journal entry.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct JLine {
    pub account_id: i64,
    /// Amount in base currency minor units: debit is positive, credit is negative.
    pub amount: Cents,
    pub description: String,
    pub currency: Option<String>,
    pub fx_amount: Option<Cents>,
}

impl JLine {
    pub fn debit(account_id: i64, amount: Cents) -> Self {
        Self {
            account_id,
            amount: amount.abs(),
            description: String::new(),
            currency: None,
            fx_amount: None,
        }
    }

    pub fn credit(account_id: i64, amount: Cents) -> Self {
        Self {
            account_id,
            amount: -amount.abs(),
            description: String::new(),
            currency: None,
            fx_amount: None,
        }
    }

    pub fn with_desc(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}

/// A posted journal entry in the general ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: i64,
    pub entry_number: i64,
    pub date: String,
    pub reference: String,
    pub narration: String,
    pub lines: Vec<JLine>,
    pub reverses_journal_id: Option<i64>,
}

/// The general ledger posting engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalLedger {
    pub chart: ChartOfAccounts,
    pub journals: Vec<JournalEntry>,
    /// Date (YYYY-MM-DD) through which the ledger is locked against prior-period adjustments.
    pub period_lock_date: Option<String>,
    next_journal_id: i64,
    next_entry_number: i64,
}

impl JournalLedger {
    pub fn new(chart: ChartOfAccounts) -> Self {
        Self {
            chart,
            journals: Vec::new(),
            period_lock_date: None,
            next_journal_id: 1,
            next_entry_number: 1,
        }
    }

    pub fn set_lock_date(&mut self, date: Option<String>) {
        self.period_lock_date = date;
    }

    pub fn check_lock(&self, date: &str) -> Result<(), String> {
        if let Some(lock) = &self.period_lock_date {
            if date <= lock.as_str() {
                return Err(format!(
                    "cannot post on {date}: period locked through {lock}"
                ));
            }
        }
        Ok(())
    }

    /// Post a balanced journal entry. Fails closed on imbalance, lock dates, or missing accounts.
    pub fn post_journal(
        &mut self,
        date: impl Into<String>,
        reference: impl Into<String>,
        narration: impl Into<String>,
        lines: Vec<JLine>,
        reverses: Option<i64>,
    ) -> Result<i64, String> {
        let date = date.into();
        self.check_lock(&date)?;

        let active_lines: Vec<JLine> = lines.into_iter().filter(|l| l.amount != 0).collect();
        if active_lines.is_empty() {
            return Err("journal has no non-zero lines".to_string());
        }

        // Validate double-entry balance: sum(debits) + sum(credits) == 0
        let sum: i64 = active_lines.iter().map(|l| l.amount).sum();
        if sum != 0 {
            return Err(format!(
                "journal does not balance (out by {})",
                fmt_money(sum)
            ));
        }

        // Verify all accounts exist and are active
        for l in &active_lines {
            match self.chart.get(l.account_id) {
                None => return Err(format!("account {} not found", l.account_id)),
                Some(a) if !a.is_active && reverses.is_none() => {
                    return Err(format!("cannot post to inactive account {}", a.name))
                }
                _ => {}
            }
        }

        let id = self.next_journal_id;
        self.next_journal_id += 1;
        let entry_number = self.next_entry_number;
        self.next_entry_number += 1;

        let entry = JournalEntry {
            id,
            entry_number,
            date,
            reference: reference.into(),
            narration: narration.into(),
            lines: active_lines,
            reverses_journal_id: reverses,
        };

        self.journals.push(entry);
        Ok(id)
    }

    /// Reverses a previously posted journal entry with inverted amounts.
    pub fn reverse_journal(&mut self, journal_id: i64, reverse_date: &str) -> Result<i64, String> {
        let orig = self
            .journals
            .iter()
            .find(|j| j.id == journal_id)
            .cloned()
            .ok_or_else(|| format!("journal {journal_id} not found"))?;

        let rev_lines: Vec<JLine> = orig
            .lines
            .into_iter()
            .map(|l| JLine {
                account_id: l.account_id,
                amount: -l.amount,
                description: format!("Reversal of J#{}: {}", orig.entry_number, l.description),
                currency: l.currency,
                fx_amount: l.fx_amount.map(|f| -f),
            })
            .collect();

        self.post_journal(
            reverse_date,
            format!("REV-{}", orig.reference),
            format!("Reversal of Journal #{}", orig.entry_number),
            rev_lines,
            Some(journal_id),
        )
    }

    /// Calculates net balances for all accounts: positive = net debit, negative = net credit.
    pub fn account_balances(&self) -> BTreeMap<i64, Cents> {
        let mut balances = BTreeMap::new();
        for j in &self.journals {
            for l in &j.lines {
                *balances.entry(l.account_id).or_insert(0) += l.amount;
            }
        }
        balances
    }

    /// Calculates total debits and total credits across all posted entries.
    pub fn trial_balance_totals(&self) -> (Cents, Cents) {
        let mut total_debits = 0;
        let mut total_credits = 0;
        for j in &self.journals {
            for l in &j.lines {
                if l.amount > 0 {
                    total_debits += l.amount;
                } else {
                    total_credits += -l.amount;
                }
            }
        }
        (total_debits, total_credits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounts::AccountClass;

    #[test]
    fn balanced_posting_and_trial_balance() {
        let mut chart = ChartOfAccounts::new();
        let cash = chart.add_account("1000", "Cash", AccountClass::Asset, None);
        let revenue = chart.add_account("4000", "Revenue", AccountClass::Revenue, None);

        let mut ledger = JournalLedger::new(chart);
        let lines = vec![
            JLine::debit(cash, 100_000),    // $1,000 debit cash
            JLine::credit(revenue, 100_000), // $1,000 credit revenue
        ];
        let jid = ledger
            .post_journal("2026-11-01", "INV-001", "Customer sale", lines, None)
            .unwrap();
        assert_eq!(jid, 1);

        let (debits, credits) = ledger.trial_balance_totals();
        assert_eq!(debits, 100_000);
        assert_eq!(credits, 100_000);

        let balances = ledger.account_balances();
        assert_eq!(balances[&cash], 100_000);
        assert_eq!(balances[&revenue], -100_000);
    }

    #[test]
    fn rejects_unbalanced_journal() {
        let mut chart = ChartOfAccounts::new();
        let cash = chart.add_account("1000", "Cash", AccountClass::Asset, None);
        let mut ledger = JournalLedger::new(chart);

        let lines = vec![JLine::debit(cash, 50_000)];
        let res = ledger.post_journal("2026-11-01", "ERR", "Unbalanced", lines, None);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("does not balance"));
    }

    #[test]
    fn enforces_period_lock_date() {
        let mut chart = ChartOfAccounts::new();
        let cash = chart.add_account("1000", "Cash", AccountClass::Asset, None);
        let rev = chart.add_account("4000", "Revenue", AccountClass::Revenue, None);

        let mut ledger = JournalLedger::new(chart);
        ledger.set_lock_date(Some("2026-10-31".into()));

        let lines = vec![JLine::debit(cash, 1000), JLine::credit(rev, 1000)];
        // Attempt posting prior to or on lock date
        let res_locked = ledger.post_journal("2026-10-15", "OLD", "Closed", lines.clone(), None);
        assert!(res_locked.is_err());
        assert!(res_locked.unwrap_err().contains("period locked"));

        // Posting after lock date succeeds
        let res_open = ledger.post_journal("2026-11-01", "NEW", "Open", lines, None);
        assert!(res_open.is_ok());
    }

    #[test]
    fn journal_reversal() {
        let mut chart = ChartOfAccounts::new();
        let cash = chart.add_account("1000", "Cash", AccountClass::Asset, None);
        let rev = chart.add_account("4000", "Revenue", AccountClass::Revenue, None);

        let mut ledger = JournalLedger::new(chart);
        let jid = ledger
            .post_journal(
                "2026-11-01",
                "ORIG",
                "Original",
                vec![JLine::debit(cash, 25000), JLine::credit(rev, 25000)],
                None,
            )
            .unwrap();

        let rev_id = ledger.reverse_journal(jid, "2026-11-02").unwrap();
        assert_eq!(rev_id, 2);

        let balances = ledger.account_balances();
        assert_eq!(balances[&cash], 0);
        assert_eq!(balances[&rev], 0);
    }
}
