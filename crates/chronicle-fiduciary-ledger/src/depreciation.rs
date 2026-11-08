//! Fixed asset depreciation schedules (Straight-line and Declining-balance).
//!
//! Generates monthly depreciation schedules adhering to GAAP / IFRS standards:
//! Supports Straight Line, 100%, 150%, and 200% (Double) Declining Balance methods,
//! with residual salvage value protection.

use crate::money::{div_round, Cents};
use serde::{Deserialize, Serialize};

/// Fixed asset depreciation calculation method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DepreciationMethod {
    StraightLine,
    Diminishing100,
    Diminishing150,
    Diminishing200,
    Full,
    None,
}

impl DepreciationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            DepreciationMethod::StraightLine => "STRAIGHTLINE",
            DepreciationMethod::Diminishing100 => "DIMINISHING100",
            DepreciationMethod::Diminishing150 => "DIMINISHING150",
            DepreciationMethod::Diminishing200 => "DIMINISHING200",
            DepreciationMethod::Full => "FULL",
            DepreciationMethod::None => "NONE",
        }
    }
}

/// Computes the effective annual rate in basis points (bp) from method and life.
pub fn annual_rate_bp(
    method: DepreciationMethod,
    rate_bp_override: Option<i64>,
    life_months: usize,
) -> i64 {
    if let Some(r) = rate_bp_override.filter(|r| *r > 0) {
        return r;
    }
    if life_months == 0 {
        return 0;
    }
    let base = div_round(10_000 * 12, life_months as i128) as i64;
    match method {
        DepreciationMethod::Diminishing150 => base * 3 / 2,
        DepreciationMethod::Diminishing200 => base * 2,
        _ => base,
    }
}

/// Calculates depreciation for one calendar month with residual value clamp.
pub fn month_depreciation(
    method: DepreciationMethod,
    cost: Cents,
    residual: Cents,
    accumulated: Cents,
    rate_bp: i64,
    fraction_bp: i64,
) -> Cents {
    let depreciable = (cost - residual - accumulated).max(0);
    if depreciable == 0 {
        return 0;
    }
    let amt = match method {
        DepreciationMethod::StraightLine => {
            div_round(
                (cost - residual) as i128 * rate_bp as i128 * fraction_bp as i128,
                10_000 * 12 * 10_000,
            ) as Cents
        }
        DepreciationMethod::Diminishing100
        | DepreciationMethod::Diminishing150
        | DepreciationMethod::Diminishing200 => {
            div_round(
                (cost - accumulated) as i128 * rate_bp as i128 * fraction_bp as i128,
                10_000 * 12 * 10_000,
            ) as Cents
        }
        DepreciationMethod::Full => depreciable,
        DepreciationMethod::None => 0,
    };
    amt.min(depreciable).max(0)
}

/// A monthly record in an asset depreciation schedule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonthlyDepreciation {
    pub month_index: usize,
    pub date: String,
    pub depreciation: Cents,
    pub accumulated: Cents,
    pub book_value: Cents,
}

/// Fixed asset depreciation schedule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetDepreciationSchedule {
    pub asset_name: String,
    pub cost: Cents,
    pub residual_value: Cents,
    pub method: DepreciationMethod,
    pub life_months: usize,
    pub schedule: Vec<MonthlyDepreciation>,
}

pub fn generate_depreciation_schedule(
    asset_name: impl Into<String>,
    cost: Cents,
    residual_value: Cents,
    method: DepreciationMethod,
    life_months: usize,
    start_year: i32,
    start_month: u32,
) -> AssetDepreciationSchedule {
    let rate_bp = annual_rate_bp(method, None, life_months);
    let mut schedule = Vec::with_capacity(life_months);
    let mut accumulated = 0;

    let mut cur_year = start_year;
    let mut cur_month = start_month;

    for m in 1..=life_months {
        let dep = month_depreciation(
            method,
            cost,
            residual_value,
            accumulated,
            rate_bp,
            10_000, // full month (100% = 10,000 bp)
        );
        accumulated += dep;
        let book_value = cost - accumulated;
        let date_str = format!("{cur_year:04}-{cur_month:02}-01");

        schedule.push(MonthlyDepreciation {
            month_index: m,
            date: date_str,
            depreciation: dep,
            accumulated,
            book_value,
        });

        cur_month += 1;
        if cur_month > 12 {
            cur_month = 1;
            cur_year += 1;
        }
    }

    AssetDepreciationSchedule {
        asset_name: asset_name.into(),
        cost,
        residual_value,
        method,
        life_months,
        schedule,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn straight_line_depreciation() {
        // Asset: $12,000 cost, $0 residual, 12 months life
        let sched = generate_depreciation_schedule(
            "Test Server",
            1_200_000,
            0,
            DepreciationMethod::StraightLine,
            12,
            2026,
            1,
        );
        assert_eq!(sched.schedule.len(), 12);
        for m in &sched.schedule {
            assert_eq!(m.depreciation, 100_000); // $1,000 / month
        }
        let last = sched.schedule.last().unwrap();
        assert_eq!(last.accumulated, 1_200_000);
        assert_eq!(last.book_value, 0);
    }

    #[test]
    fn double_declining_balance_with_residual() {
        // Asset: $10,000 cost, $1,000 residual, 5 years (60 months) life
        // 200% declining balance
        let sched = generate_depreciation_schedule(
            "Workstation",
            1_000_000,
            100_000,
            DepreciationMethod::Diminishing200,
            60,
            2026,
            1,
        );
        assert_eq!(sched.schedule.len(), 60);

        // First month depreciation should be larger than straight line
        let first_dep = sched.schedule[0].depreciation;
        let sl_dep = 1_000_000 / 60;
        assert!(first_dep > sl_dep);

        // Ensure book value never drops below residual value ($1,000 = 100,000 cents)
        for m in &sched.schedule {
            assert!(m.book_value >= 100_000);
        }
    }
}
