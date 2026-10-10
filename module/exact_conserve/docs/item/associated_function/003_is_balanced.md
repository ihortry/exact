# 003: is_balanced

## Representation

Whether every asset's net in a `Report` is exactly zero — no tolerance window, by
design (doc comment, `src/lib.rs:170-174`): a tolerance is exactly how an
auditor comes to pass the one-unit-per-transaction leak this crate exists to
catch.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_conserve/src/lib.rs:176-179`

```rust
pub fn is_balanced( &self ) -> bool
{
  self.nets.values().all( | net | *net == 0 )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 176-179 | Declaration |
| `src/lib.rs` | 201 | Called from `Display for Report`'s own `fmt` |
| `src/lib.rs` | 225 | `verify`'s own doc-test (a real `cargo test --doc` execution, not just a mention) |
| `tests/conservation_test.rs` | throughout | Nearly every test's final assertion |
| `exact_arith/src/lib.rs:30` | — | Facade's own module-level doc-test |
| `exact_arith/tests/facade_test.rs:30` | — | Facade's own integration test |
| `cluster_economy/src/market.rs:508,519` | — | **Production** — gates whether a settlement's cash/asset legs are accepted |
| `cluster_economy/tests/economy_test.rs:263-264` | — | Reconciliation assertions |
| `exchange_core/tests/submission_test.rs:258` | — | Integration test |
| `smoke_exchange_core/src/lib.rs:107` | — | Demo-lane settlement check |
| `smoke_exact_market_split/src/lib.rs:309,314`, `tests/lane_test.rs:64,67` | — | Demo-lane ledger checks |

**The crate's most heavily used method by far, and corrects an error in
this file's own first draft** — an earlier pass wrongly reported this as an
honest-empty finding (zero external callers) before a full grep across the
whole workspace (not just `exact_conserve`'s immediate dependents) surfaced
genuine production usage. See § Notable Findings in the readme for why the
first pass missed it: it only checked `exact_conserve`'s direct `Cargo.toml`
dependents, not every crate reachable through the `exact_arith` facade.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Used throughout its own tests and by `Display for Report` itself |
| `exact_arith` | `src/lib.rs`, `tests/facade_test.rs` | Doc-test and integration test |
| `cluster_economy` | `src/market.rs` | **Production** — settlement acceptance gate |
| `exchange_core` | `tests/submission_test.rs` | Integration test |
| `smoke_exchange_core` | `src/lib.rs` | Demo-lane settlement check |
| `smoke_exact_market_split` | `src/lib.rs`, `tests/lane_test.rs` | Demo-lane ledger checks |

## Caller Tree

- [Display::fmt for Report](005_fmt_display_for_report.md) (`src/lib.rs:201`, intra-crate)
- **External:** `cluster_economy::market::<settlement path>` (`cluster_economy/src/market.rs:508,519`) — the one confirmed production caller
- **External:** `smoke_exchange_core`'s and `smoke_exact_market_split`'s own demo-lane settlement/ledger checks (not production, but not test-only either)

## Callee Tree

No callee of its own — reads `self.nets` directly through `BTreeMap::values`
and `Iterator::all`; no further hop into another Item Instance. No longer a
`const fn`: a map cannot be read in `const` code, and no caller used it there.
