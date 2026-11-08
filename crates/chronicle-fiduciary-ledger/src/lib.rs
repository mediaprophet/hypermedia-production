//! `chronicle-fiduciary-ledger`
//!
//! UI-agnostic categorical fiduciary ledger engine:
//! - Exact fixed-point multi-currency money arithmetic (`money`)
//! - Chart of accounts and balance sheet classifications (`accounts`)
//! - Balanced double-entry journal posting with period lock enforcement (`journal`)
//! - ASC 606 / IFRS 15 multi-period revenue recognition amortization schedules (`revrec`)
//! - Fixed-asset depreciation schedules with residual salvage protection (`depreciation`)
//! - Multi-entity currency translation & intercompany elimination consolidation (`consolidation`)
//! - Tamper-evident hash-chained fiduciary audit trail grounded in UDHR/ICCPR/ICESCR (`audit`)

#![forbid(unsafe_code)]

pub mod accounts;
pub mod audit;
pub mod consolidation;
pub mod depreciation;
pub mod journal;
pub mod money;
pub mod revrec;

pub use accounts::{Account, AccountClass, ChartOfAccounts};
pub use audit::{FiduciaryAuditEntry, FiduciaryAuditTrail, FIDUCIARY_CITATION_UDHR_12_18_19_27};
pub use consolidation::{
    consolidate_entities, ConsolidatedReport, ConsolidatedRow, EntityTrialBalance,
};
pub use depreciation::{
    annual_rate_bp, generate_depreciation_schedule, month_depreciation, AssetDepreciationSchedule,
    DepreciationMethod, MonthlyDepreciation,
};
pub use journal::{JLine, JournalEntry, JournalLedger};
pub use money::{
    div_round, div_round_bankers, fmt_fixed, fmt_money, fmt_money_grouped, line_amount,
    parse_fixed, to_base, Cents,
};
pub use revrec::{RevenueSchedule, RevenueScheduleEntry};
