//! Multi-entity currency translation and intercompany elimination consolidation.
//!
//! Aggregates multiple legal entities' financial statements side-by-side,
//! translates foreign currencies into a group reporting currency, eliminates
//! reciprocal intercompany accounts, and produces a consolidated balance sheet
//! and income statement.

use std::collections::BTreeMap;
use crate::accounts::AccountClass;
use crate::money::{to_base, Cents};
use serde::{Deserialize, Serialize};

/// Financial statement summary for an individual entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityTrialBalance {
    pub entity_name: String,
    pub currency: String,
    /// Exchange rate from this entity's currency to group base currency (1 group base = rate foreign, e.g. 1 USD = 1.50 AUD).
    pub fx_rate_e8: i64,
    /// Account code -> (Account Name, Account Class, Balance in minor units).
    pub accounts: BTreeMap<String, (String, AccountClass, Cents)>,
}

/// A consolidated financial statement line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsolidatedRow {
    pub account_code: String,
    pub account_name: String,
    pub class: AccountClass,
    /// Balance per entity, translated to group base currency.
    pub entity_amounts: Vec<Cents>,
    /// Elimination adjustment amount.
    pub elimination_amount: Cents,
    /// Consolidated group total.
    pub consolidated_total: Cents,
    pub is_eliminated: bool,
}

/// Consolidated multi-entity financial statement report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedReport {
    pub group_currency: String,
    pub entity_names: Vec<String>,
    pub rows: Vec<ConsolidatedRow>,
    pub total_revenue: Cents,
    pub total_expense: Cents,
    pub net_income: Cents,
    pub total_assets: Cents,
    pub total_liabilities: Cents,
    pub total_equity: Cents,
}

/// Performs consolidation across entities with translation and elimination.
pub fn consolidate_entities(
    entities: &[EntityTrialBalance],
    group_currency: impl Into<String>,
    elimination_codes: &[String],
) -> ConsolidatedReport {
    let entity_names: Vec<String> = entities.iter().map(|e| e.entity_name.clone()).collect();
    let num_entities = entities.len();

    // Map: code -> (Name, Class)
    let mut account_defs: BTreeMap<String, (String, AccountClass)> = BTreeMap::new();
    for e in entities {
        for (code, (name, class, _)) in &e.accounts {
            account_defs
                .entry(code.clone())
                .or_insert_with(|| (name.clone(), *class));
        }
    }

    let mut rows = Vec::new();
    let mut total_rev = 0;
    let mut total_exp = 0;
    let mut total_assets = 0;
    let mut total_liab = 0;
    let mut total_equity = 0;

    for (code, (name, class)) in account_defs {
        let is_elim = elimination_codes.iter().any(|c| c == &code);
        let mut entity_amounts = Vec::with_capacity(num_entities);

        for e in entities {
            let local_balance = e.accounts.get(&code).map(|(_, _, b)| *b).unwrap_or(0);
            let translated = if e.fx_rate_e8 == 0 || e.currency == "USD" {
                local_balance
            } else {
                to_base(local_balance, e.fx_rate_e8, 2, 2)
            };
            entity_amounts.push(translated);
        }

        let sum_entities: Cents = entity_amounts.iter().sum();
        let elimination_amount = if is_elim { -sum_entities } else { 0 };
        let consolidated_total = sum_entities + elimination_amount;

        match class {
            AccountClass::Revenue => total_rev += consolidated_total.abs(),
            AccountClass::Expense => total_exp += consolidated_total.abs(),
            AccountClass::Asset => total_assets += consolidated_total,
            AccountClass::Liability => total_liab += consolidated_total.abs(),
            AccountClass::Equity => total_equity += consolidated_total.abs(),
        }

        rows.push(ConsolidatedRow {
            account_code: code,
            account_name: name,
            class,
            entity_amounts,
            elimination_amount,
            consolidated_total,
            is_eliminated: is_elim,
        });
    }

    let net_income = total_rev - total_exp;

    ConsolidatedReport {
        group_currency: group_currency.into(),
        entity_names,
        rows,
        total_revenue: total_rev,
        total_expense: total_exp,
        net_income,
        total_assets,
        total_liabilities: total_liab,
        total_equity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_entity_elimination_and_consolidation() {
        // Entity A (Parent in USD)
        let mut accts_a = BTreeMap::new();
        accts_a.insert(
            "1000".into(),
            ("Cash".into(), AccountClass::Asset, 500_000), // $5,000
        );
        accts_a.insert(
            "1250".into(),
            ("Intercompany Loan to B".into(), AccountClass::Asset, 200_000), // $2,000 receivable
        );
        accts_a.insert(
            "4000".into(),
            ("Revenue".into(), AccountClass::Revenue, -300_000),
        );

        let entity_a = EntityTrialBalance {
            entity_name: "Holdings LLC".into(),
            currency: "USD".into(),
            fx_rate_e8: 0,
            accounts: accts_a,
        };

        // Entity B (Sub in USD)
        let mut accts_b = BTreeMap::new();
        accts_b.insert(
            "1000".into(),
            ("Cash".into(), AccountClass::Asset, 150_000), // $1,500
        );
        accts_b.insert(
            "2250".into(),
            (
                "Intercompany Loan from A".into(),
                AccountClass::Liability,
                -200_000,
            ), // $2,000 payable
        );
        accts_b.insert(
            "5000".into(),
            ("Operating Expense".into(), AccountClass::Expense, 100_000),
        );

        let entity_b = EntityTrialBalance {
            entity_name: "Operating Corp".into(),
            currency: "USD".into(),
            fx_rate_e8: 0,
            accounts: accts_b,
        };

        let eliminations = vec!["1250".to_string(), "2250".to_string()];
        let report = consolidate_entities(&[entity_a, entity_b], "USD", &eliminations);

        assert_eq!(report.entity_names.len(), 2);
        // Cash should be 500,000 + 150,000 = 650,000
        let cash_row = report.rows.iter().find(|r| r.account_code == "1000").unwrap();
        assert_eq!(cash_row.consolidated_total, 650_000);

        // Intercompany loan receivable (1250) eliminated to 0
        let ic_rec = report.rows.iter().find(|r| r.account_code == "1250").unwrap();
        assert!(ic_rec.is_eliminated);
        assert_eq!(ic_rec.consolidated_total, 0);

        // Intercompany loan payable (2250) eliminated to 0
        let ic_pay = report.rows.iter().find(|r| r.account_code == "2250").unwrap();
        assert!(ic_pay.is_eliminated);
        assert_eq!(ic_pay.consolidated_total, 0);
    }
}
