//! ASC 606 / IFRS 15 multi-period revenue recognition amortization schedules.
//!
//! Provides over-time revenue deferral and recognition schedules:
//! Upfront income is moved to a Deferred Revenue liability account on the invoice date,
//! then systematically recognised into earned revenue across discrete monthly periods.
//! The final period absorbs any rounding difference to ensure exact cent reconciliation.

use crate::journal::{JLine, JournalLedger};
use crate::money::{div_round, Cents};
use serde::{Deserialize, Serialize};

/// An individual scheduled recognition entry for one monthly period.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevenueScheduleEntry {
    pub period: usize,
    pub date: String,
    pub amount: Cents,
    pub journal_id: Option<i64>,
}

/// An ASC 606 / IFRS 15 amortization schedule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevenueSchedule {
    pub id: i64,
    pub document_reference: String,
    pub deferral_account_id: i64,
    pub revenue_account_id: i64,
    pub start_date: String,
    pub total_months: usize,
    pub total_amount: Cents,
    pub entries: Vec<RevenueScheduleEntry>,
}

impl RevenueSchedule {
    /// Generates a new multi-period revenue recognition schedule.
    /// Distributes `total_amount` equally across `months` periods, absorbing rounding in the final entry.
    pub fn new(
        id: i64,
        document_reference: impl Into<String>,
        deferral_account_id: i64,
        revenue_account_id: i64,
        start_year: i32,
        start_month: u32,
        months: usize,
        total_amount: Cents,
    ) -> Result<Self, String> {
        if months == 0 || months > 120 {
            return Err("recognition schedule must span between 1 and 120 months".to_string());
        }
        if total_amount <= 0 {
            return Err("schedule total amount must be positive".to_string());
        }

        let doc_ref = document_reference.into();
        let each = div_round(total_amount as i128, months as i128) as Cents;
        let mut entries = Vec::with_capacity(months);

        let mut cur_year = start_year;
        let mut cur_month = start_month;

        for period in 1..=months {
            let amount = if period == months {
                total_amount - each * (months as i64 - 1)
            } else {
                each
            };

            let date_str = format!("{cur_year:04}-{cur_month:02}-01");
            entries.push(RevenueScheduleEntry {
                period,
                date: date_str,
                amount,
                journal_id: None,
            });

            cur_month += 1;
            if cur_month > 12 {
                cur_month = 1;
                cur_year += 1;
            }
        }

        let start_date = format!("{start_year:04}-{start_month:02}-01");
        Ok(Self {
            id,
            document_reference: doc_ref,
            deferral_account_id,
            revenue_account_id,
            start_date,
            total_months: months,
            total_amount,
            entries,
        })
    }

    /// Recognises all unposted periods due on or before `as_of_date` by posting to `ledger`.
    pub fn post_due_entries(
        &mut self,
        as_of_date: &str,
        ledger: &mut JournalLedger,
    ) -> Result<usize, String> {
        let mut posted_count = 0;
        let doc_ref = self.document_reference.clone();
        let deferral_acct = self.deferral_account_id;
        let rev_acct = self.revenue_account_id;

        for entry in &mut self.entries {
            if entry.journal_id.is_none() && entry.date.as_str() <= as_of_date {
                // Post recognition: Debit Deferred Revenue (reduce liability), Credit Operating Revenue (earn income)
                let lines = vec![
                    JLine::debit(deferral_acct, entry.amount).with_desc(format!(
                        "RevRec: {doc_ref} period {}/{}",
                        entry.period, entry.period
                    )),
                    JLine::credit(rev_acct, entry.amount).with_desc(format!(
                        "RevRec: {doc_ref} period {}/{}",
                        entry.period, entry.period
                    )),
                ];

                let jid = ledger.post_journal(
                    entry.date.clone(),
                    format!("REVREC-{doc_ref}-P{}", entry.period),
                    format!("Recognised revenue period {}", entry.period),
                    lines,
                    None,
                )?;

                entry.journal_id = Some(jid);
                posted_count += 1;
            }
        }
        Ok(posted_count)
    }

    /// Returns the remaining unrecognized deferred revenue balance.
    pub fn remaining_unrecognized(&self) -> Cents {
        self.entries
            .iter()
            .filter(|e| e.journal_id.is_none())
            .map(|e| e.amount)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounts::{AccountClass, ChartOfAccounts};

    #[test]
    fn schedule_generation_and_rounding() {
        // $100.00 across 3 months -> 33.33, 33.33, 33.34
        let sched = RevenueSchedule::new(1, "INV-100", 2050, 4000, 2026, 1, 3, 10000).unwrap();
        assert_eq!(sched.entries.len(), 3);
        assert_eq!(sched.entries[0].amount, 3333);
        assert_eq!(sched.entries[1].amount, 3333);
        assert_eq!(sched.entries[2].amount, 3334);
        assert_eq!(
            sched.entries.iter().map(|e| e.amount).sum::<Cents>(),
            10000
        );
    }

    #[test]
    fn recognition_posting_lifecycle() {
        let mut chart = ChartOfAccounts::new();
        let bank = chart.add_account("1000", "Bank", AccountClass::Asset, None);
        let deferred = chart.add_account("2050", "Deferred Revenue", AccountClass::Liability, None);
        let revenue = chart.add_account("4000", "Revenue", AccountClass::Revenue, None);

        let mut ledger = JournalLedger::new(chart);

        // 1. Initial invoice upfront cash received: Debit Bank $1,200, Credit Deferred Revenue $1,200
        ledger
            .post_journal(
                "2026-01-01",
                "INV-1200",
                "Annual contract upfront payment",
                vec![JLine::debit(bank, 120_000), JLine::credit(deferred, 120_000)],
                None,
            )
            .unwrap();

        // 2. Setup 12-month schedule
        let mut sched =
            RevenueSchedule::new(1, "INV-1200", deferred, revenue, 2026, 1, 12, 120_000).unwrap();
        assert_eq!(sched.remaining_unrecognized(), 120_000);

        // 3. Recognise Q1 (months 1, 2, 3 as of 2026-03-31)
        let posted = sched.post_due_entries("2026-03-31", &mut ledger).unwrap();
        assert_eq!(posted, 3);
        assert_eq!(sched.remaining_unrecognized(), 90_000);

        let balances = ledger.account_balances();
        // Deferred revenue was -120,000, then debited by 3 * 10,000 = 30,000 -> net -90,000
        assert_eq!(balances[&deferred], -90_000);
        // Revenue credited by 30,000 -> -30,000
        assert_eq!(balances[&revenue], -30_000);
    }
}
