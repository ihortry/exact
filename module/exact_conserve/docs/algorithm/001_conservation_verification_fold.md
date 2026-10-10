# Algorithm: Conservation Verification Fold

### Scope

- **Purpose**: State exactly how a log or a typed slice is checked for conservation, so a reader can predict the verdict on any input without running it.
- **Responsibility**: `verify`'s plain-log fold, and the typed layer (`money_conserve_into`/`qty_conserve_into`, `money_sum_assert_zero`) built on top of it.
- **In Scope**: The fold's accumulator width, its overflow handling, the balance comparison, and why per-account totals are not part of any of this.
- **Out of Scope**: A split that fails to conserve, which this audit would catch only as an aggregate imbalance rather than at its source (→ [Equal-Parts Dust Split](../../../exact_dust/docs/algorithm/001_equal_parts_dust_split.md)); the checked arithmetic `money_conserve_into`/`qty_conserve_into` dispatch to (→ `exact_add`'s own crate).

### Procedure — Plain-Log Fold (`verify`)

1. Start with no accumulators — one `i128` per asset, in a `BTreeMap` keyed by asset.
2. For each `Entry`, in order, add `i128::from(entry.amount_minor)` via `checked_add` to its own asset's accumulator, starting it at `0` on the asset's first posting. Amounts of different assets are never added together.
3. If any step overflows `i128`, stop immediately and return `ConservationError::Overflow`.
4. Otherwise, return `Report { entries: entries.len(), nets }`.
5. `Report::is_balanced` is then exact equality of every asset's net with `0` — no tolerance window of any kind. A log that invents one unit of one asset and loses one unit of another is unbalanced twice over, not balanced on the whole.

### Procedure — Typed Layer

`money_conserve_into`/`qty_conserve_into` are not a second fold implementation — each is one step of the same shape, delegated to `exact_add::money_add`/`exact_add::qty_add` and suitable directly as an `Iterator::try_fold` closure. `money_sum_assert_zero` runs the identical accumulate-then-compare shape as `verify` above, but over a typed `&[Money]` slice instead of `&[Entry]`, widening each leg's `minor()` into the same `i128` accumulator and comparing it to zero under the same no-tolerance rule.

There is no `qty_sum_assert_zero`: every `Quantity` is individually non-negative, so a slice of them sums to zero only when every leg is zero — it could never check a transfer, whose giving side is negative. A quantity's movements are audited through `verify` instead, as signed `i64` amounts under the quantity's own asset key (→ [Conservation Is Checked Per Asset](../decisions/002_conservation_checked_per_asset.md)).

### Why `i128`, Not A Declared Maximum Log Length

Postings are `i64`, matching the family's backing width; every accumulator above is `i128`, strictly wider. The fold is still checked throughout, so even the point at which `i128` itself would run out (somewhere past `2⁶⁴` maximal-magnitude postings) returns `ConservationError::Overflow` rather than silently wrapping. That edge is unreachable from any real log.

### Why Exact Zero, No Tolerance

A tolerance is how a conservation check comes to pass the only errors small enough to be worth hiding: an off-by-one-unit leak repeated across a million transactions is precisely the failure this crate exists to make impossible, and it is invisible to any check that ignores single units. `one_unit_stays_visible_against_a_million_units_of_turnover` (`tests/conservation_test.rs`) is this property under test.

### Why Per-Account Totals Are Not Computed

Deliberately out of scope, at every grain this crate checks: an account's own total is its balance, and a non-zero balance is the normal state of an account, not a finding. Reporting balances alongside a conservation verdict would put a column of expected non-zeros next to the one non-zero that actually means something — the one `Report`/`*_sum_assert_zero` already isolates. This is a stated design boundary, not a deferred feature, and it is a separate point from the next one.

### Why This Fold, Not Per-Transaction Grouping Or Terminal Reconciliation

The check here is a single whole-log (or whole-slice) sum compared to zero — it does not group postings into transactions and check each one individually, and it does not reconcile a running total against separately-supplied initial/final balance snapshots. Either would narrow a failing audit from "the log's net is nonzero" to "this specific transaction" or "this specific account," and neither is implemented: there is no transaction-grouping rule and no snapshot input anywhere in this crate's real surface. A caller needing that finer attribution re-derives it externally from the same `Entry` log today.

### Decisions

| File | Relationship |
|------|--------------|
| [001_dependency_contract_retired_for_typed_layer.md](../decisions/001_dependency_contract_retired_for_typed_layer.md) | Why this crate's manifest is no longer dependency-free, and what stays dependency-free anyway |

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:11-36` | The module doc's "What conservation means here" and "Widths" sections — the `i64`/`i128` framing and the per-account-totals rationale, verbatim source for the two "Why" sections above |
| `src/lib.rs:234-247` | `verify`'s implementation (steps 1-4) |
| `src/lib.rs:176-179` | `Report::is_balanced` — step 5 |
| `src/lib.rs:255-269` | `money_conserve_into`/`qty_conserve_into` — the typed layer's single-step fold, delegated to `exact_add` |
| `src/lib.rs:277-292` | `money_sum_assert_zero` — the typed layer's accumulate-then-compare check |

### Tests

| File | Relationship |
|------|--------------|
| `tests/conservation_test.rs` | Plain-log balance and discrepancy-sign coverage (ported from `exact_audit`); `money_conserve_into`/`qty_conserve_into` fold-and-overflow coverage; `money_sum_assert_zero` cancellation coverage; per-asset netting (`a_leak_in_one_asset_does_not_cancel_a_forgery_in_another`, `a_log_balances_when_every_asset_balances`), under a `&str` or an enum key (`a_log_can_be_keyed_by_the_callers_own_asset_type`) |
