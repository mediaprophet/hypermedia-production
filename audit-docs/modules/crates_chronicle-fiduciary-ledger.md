# `crates/chronicle-fiduciary-ledger`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_`chronicle-fiduciary-ledger`

UI-agnostic categorical fiduciary ledger engine:
- Exact fixed-point multi-currency money arithmetic (`money`)
- Chart of accounts and balance sheet classifications (`accounts`)
- Balanced double-entry journal posting with period lock enforcement (`journal`)
- ASC 606 / IFRS 15 multi-period revenue recognition amortization schedules (`revrec`)
- Fixed-asset depreciation schedules with residual salvage protection (`depreciation`)
- Multi-entity currency translation & intercompany elimination consolidation (`consolidation`)
- Tamper-evident hash-chained fiduciary audit trail grounded in UDHR/ICCPR/ICESCR (`audit`)_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl AccountClass::normal_balance_is_debit` | 1 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:18` | 3d |  |
| `impl ChartOfAccounts::new` | 0 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:43` | - |  |
| `impl ChartOfAccounts::add_account` | 5 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:50` | - |  |
| `impl ChartOfAccounts::get` | 2 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:72` | - |  |
| `impl ChartOfAccounts::get_by_code` | 2 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:76` | - |  |
| `impl ChartOfAccounts::get_by_name` | 2 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:80` | - |  |
| `impl ChartOfAccounts::all` | 1 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:84` | - |  |
| `impl ChartOfAccounts::standard_template` | 0 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:99` | - | Sets up a standard standard chart of accounts: |
| `chart_lookup_and_normal_balance` | 0 | `crates/chronicle-fiduciary-ledger/src/accounts.rs:120` | 3d |  |
| `impl FiduciaryAuditEntry::compute_hash` | 9 | `crates/chronicle-fiduciary-ledger/src/audit.rs:34` | - |  |
| `impl FiduciaryAuditTrail::new` | 0 | `crates/chronicle-fiduciary-ledger/src/audit.rs:59` | - |  |
| `impl FiduciaryAuditTrail::append` | 8 | `crates/chronicle-fiduciary-ledger/src/audit.rs:66` | - | Appends a new verified audit event to the tamper-evident chain. |
| `impl FiduciaryAuditTrail::verify_chain` | 1 | `crates/chronicle-fiduciary-ledger/src/audit.rs:122` | - | Validates the complete cryptographic hash chain from genesis to head. |
| `impl FiduciaryAuditTrail::filter_by_entity` | 3 | `crates/chronicle-fiduciary-ledger/src/audit.rs:159` | - |  |
| `fnv1a_hex` | 1 | `crates/chronicle-fiduciary-ledger/src/audit.rs:167` | - |  |
| `audit_trail_integrity_and_verification` | 0 | `crates/chronicle-fiduciary-ledger/src/audit.rs:181` | - |  |
| `detects_tampering` | 0 | `crates/chronicle-fiduciary-ledger/src/audit.rs:213` | - |  |
| `consolidate_entities` | 3 | `crates/chronicle-fiduciary-ledger/src/consolidation.rs:54` | - | Performs consolidation across entities with translation and elimination. |
| `multi_entity_elimination_and_consolidation` | 0 | `crates/chronicle-fiduciary-ledger/src/consolidation.rs:136` | - |  |
| `impl DepreciationMethod::as_str` | 1 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:23` | - |  |
| `annual_rate_bp` | 3 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:36` | - | Computes the effective annual rate in basis points (bp) from method and life. |
| `month_depreciation` | 6 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:56` | - | Calculates depreciation for one calendar month with residual value clamp. |
| `generate_depreciation_schedule` | 7 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:110` | - |  |
| `straight_line_depreciation` | 0 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:169` | - |  |
| `double_declining_balance_with_residual` | 0 | `crates/chronicle-fiduciary-ledger/src/depreciation.rs:190` | - |  |
| `impl JLine::debit` | 2 | `crates/chronicle-fiduciary-ledger/src/journal.rs:24` | - |  |
| `impl JLine::credit` | 2 | `crates/chronicle-fiduciary-ledger/src/journal.rs:34` | - |  |
| `impl JLine::with_desc` | 2 | `crates/chronicle-fiduciary-ledger/src/journal.rs:44` | - |  |
| `impl JournalLedger::new` | 1 | `crates/chronicle-fiduciary-ledger/src/journal.rs:74` | - |  |
| `impl JournalLedger::set_lock_date` | 2 | `crates/chronicle-fiduciary-ledger/src/journal.rs:84` | - |  |
| `impl JournalLedger::check_lock` | 2 | `crates/chronicle-fiduciary-ledger/src/journal.rs:88` | - |  |
| `impl JournalLedger::post_journal` | 6 | `crates/chronicle-fiduciary-ledger/src/journal.rs:100` | - | Post a balanced journal entry. Fails closed on imbalance, lock dates, or missing accounts. |
| `impl JournalLedger::reverse_journal` | 3 | `crates/chronicle-fiduciary-ledger/src/journal.rs:156` | - | Reverses a previously posted journal entry with inverted amounts. |
| `impl JournalLedger::account_balances` | 1 | `crates/chronicle-fiduciary-ledger/src/journal.rs:186` | - | Calculates net balances for all accounts: positive = net debit, negative = net credit. |
| `impl JournalLedger::trial_balance_totals` | 1 | `crates/chronicle-fiduciary-ledger/src/journal.rs:197` | - | Calculates total debits and total credits across all posted entries. |
| `balanced_posting_and_trial_balance` | 0 | `crates/chronicle-fiduciary-ledger/src/journal.rs:219` | - |  |
| `rejects_unbalanced_journal` | 0 | `crates/chronicle-fiduciary-ledger/src/journal.rs:244` | - |  |
| `enforces_period_lock_date` | 0 | `crates/chronicle-fiduciary-ledger/src/journal.rs:256` | - |  |
| `journal_reversal` | 0 | `crates/chronicle-fiduciary-ledger/src/journal.rs:276` | - |  |
| `div_round` | 2 | `crates/chronicle-fiduciary-ledger/src/money.rs:12` | - | Commercial rounding: divide rounding half away from zero. |
| `div_round_bankers` | 2 | `crates/chronicle-fiduciary-ledger/src/money.rs:23` | - | Banker's rounding: divide rounding half to even. |
| `line_amount` | 3 | `crates/chronicle-fiduciary-ledger/src/money.rs:37` | economics | Calculates line net amount (before tax) = qty * unit price * (1 - discount). |
| `to_base` | 4 | `crates/chronicle-fiduciary-ledger/src/money.rs:44` | - | Converts an amount in foreign currency to base currency (1 base = rate foreign). |
| `parse_fixed` | 2 | `crates/chronicle-fiduciary-ledger/src/money.rs:60` | - | Parse a decimal string like "-1,234.50" into a fixed-point integer with `dp` decimals. |
| `fmt_fixed` | 2 | `crates/chronicle-fiduciary-ledger/src/money.rs:97` | - |  |
| `fmt_money` | 1 | `crates/chronicle-fiduciary-ledger/src/money.rs:113` | - |  |
| `fmt_money_grouped` | 1 | `crates/chronicle-fiduciary-ledger/src/money.rs:118` | - | Formats money with thousands separators: e.g. `12,345.60` or `-1,234.56`. |
| `format_grouped_money` | 0 | `crates/chronicle-fiduciary-ledger/src/money.rs:136` | - |  |
| `rounding_logic` | 0 | `crates/chronicle-fiduciary-ledger/src/money.rs:144` | - |  |
| `line_amount_calculation` | 0 | `crates/chronicle-fiduciary-ledger/src/money.rs:155` | - |  |
| `parse_fixed_values` | 0 | `crates/chronicle-fiduciary-ledger/src/money.rs:163` | - |  |
| `impl RevenueSchedule::new` | 8 | `crates/chronicle-fiduciary-ledger/src/revrec.rs:37` | - | Generates a new multi-period revenue recognition schedule. |
| `impl RevenueSchedule::post_due_entries` | 3 | `crates/chronicle-fiduciary-ledger/src/revrec.rs:97` | - | Recognises all unposted periods due on or before `as_of_date` by posting to `ledger`. |
| `impl RevenueSchedule::remaining_unrecognized` | 1 | `crates/chronicle-fiduciary-ledger/src/revrec.rs:137` | - | Returns the remaining unrecognized deferred revenue balance. |
| `schedule_generation_and_rounding` | 0 | `crates/chronicle-fiduciary-ledger/src/revrec.rs:152` | - |  |
| `recognition_posting_lifecycle` | 0 | `crates/chronicle-fiduciary-ledger/src/revrec.rs:166` | - |  |
