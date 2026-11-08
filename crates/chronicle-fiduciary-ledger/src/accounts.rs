//! Chart of accounts and financial classifications.

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

/// High-level balance sheet and income statement account classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountClass {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl AccountClass {
    pub fn normal_balance_is_debit(self) -> bool {
        matches!(self, AccountClass::Asset | AccountClass::Expense)
    }
}

/// An individual account within the general ledger chart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub class: AccountClass,
    pub currency: Option<String>,
    pub is_active: bool,
    pub description: String,
}

/// In-memory Chart of Accounts manager.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChartOfAccounts {
    accounts: BTreeMap<i64, Account>,
    next_id: i64,
}

impl ChartOfAccounts {
    pub fn new() -> Self {
        Self {
            accounts: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn add_account(
        &mut self,
        code: impl Into<String>,
        name: impl Into<String>,
        class: AccountClass,
        currency: Option<String>,
    ) -> i64 {
        let id = self.next_id;
        self.next_id += 1;
        let acct = Account {
            id,
            code: code.into(),
            name: name.into(),
            class,
            currency,
            is_active: true,
            description: String::new(),
        };
        self.accounts.insert(id, acct);
        id
    }

    pub fn get(&self, id: i64) -> Option<&Account> {
        self.accounts.get(&id)
    }

    pub fn get_by_code(&self, code: &str) -> Option<&Account> {
        self.accounts.values().find(|a| a.code == code)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Account> {
        self.accounts.values().find(|a| a.name.eq_ignore_ascii_case(name))
    }

    pub fn all(&self) -> impl Iterator<Item = &Account> {
        self.accounts.values()
    }

    /// Sets up a standard standard chart of accounts:
    /// - 1000 Cash / Operating Bank (Asset)
    /// - 1200 Accounts Receivable (Asset)
    /// - 1500 Fixed Assets (Asset)
    /// - 1510 Accumulated Depreciation (Asset, Contra)
    /// - 2000 Accounts Payable (Liability)
    /// - 2050 Deferred Revenue (Liability)
    /// - 3000 Retained Earnings (Equity)
    /// - 4000 Operating Revenue (Revenue)
    /// - 5000 Operating Expenses (Expense)
    /// - 5100 Depreciation Expense (Expense)
    pub fn standard_template() -> Self {
        let mut chart = Self::new();
        chart.add_account("1000", "Operating Bank Account", AccountClass::Asset, None);
        chart.add_account("1200", "Accounts Receivable", AccountClass::Asset, None);
        chart.add_account("1500", "Fixed Assets", AccountClass::Asset, None);
        chart.add_account("1510", "Accumulated Depreciation", AccountClass::Asset, None);
        chart.add_account("2000", "Accounts Payable", AccountClass::Liability, None);
        chart.add_account("2050", "Deferred Revenue", AccountClass::Liability, None);
        chart.add_account("3000", "Retained Earnings", AccountClass::Equity, None);
        chart.add_account("4000", "Operating Revenue", AccountClass::Revenue, None);
        chart.add_account("5000", "Operating Expenses", AccountClass::Expense, None);
        chart.add_account("5100", "Depreciation Expense", AccountClass::Expense, None);
        chart
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chart_lookup_and_normal_balance() {
        let chart = ChartOfAccounts::standard_template();
        let cash = chart.get_by_code("1000").unwrap();
        assert_eq!(cash.class, AccountClass::Asset);
        assert!(cash.class.normal_balance_is_debit());

        let rev = chart.get_by_code("4000").unwrap();
        assert_eq!(rev.class, AccountClass::Revenue);
        assert!(!rev.class.normal_balance_is_debit());
    }
}
