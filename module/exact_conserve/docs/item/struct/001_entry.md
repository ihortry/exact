# 001: Entry

## Representation

One posting in a transaction log — an account of the caller's own type `K`,
never read by `verify`, the asset that moved (of the caller's own key type
`A`), and a signed minor-unit amount. Plain data with no invariant of its own;
carried forward from `exact_audit`, which had no `asset` — [verify](../function/001_verify.md)
nets each asset separately, so amounts of different assets never cancel. The scale is never interpreted by [`verify`],
because conservation is a property of the raw integers and holds at every
scale (`Entry`'s doc comment, `src/lib.rs:89-93`).

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_conserve/src/lib.rs:98-109`

```rust
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct Entry< K, A >
{
  pub account : K,
  pub asset : A,
  pub amount_minor : i64,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 99-109 | Declaration |
| `src/lib.rs` | 221-229 | `verify`'s own doc-test |
| `tests/conservation_test.rs:18-21` | — | `transfer` test helper builds a matched credit/debit pair |
| `exchange_core/src/lib.rs:460-461` | — | **Production** — one credit and one matching debit posting per settled trade |
| `cluster_economy/src/market.rs:504-505,515-516` | — | **Production** — cash-leg and asset-leg postings per settlement |
| `cluster_economy/tests/economy_test.rs:255-261` | — | Builds postings for a reconciliation assertion |
| `smoke_exact_market_split/src/lib.rs:127-128` | — | Demo lane's own ledger postings |
| `exact_arith/src/lib.rs:30,141` | — | Facade doc-test and re-export |

**The most consumed Item in this crate** — unlike almost every other type in
this migration, `Entry` has real production call sites in two downstream
crates outside `module/` entirely (`exchange_core`,
`cluster_economy`), not merely tests or facade re-exports.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Exercised by its own test suite and doc-test |
| `exact_arith` | `src/lib.rs` | Re-export, and its own doc-test constructs `Entry` values |
| `exchange_core` | `src/lib.rs` | **Production** — settlement postings |
| `cluster_economy` | `src/market.rs` | **Production** — settlement postings |
| `smoke_exact_market_split` | `src/lib.rs` | Demo-lane ledger postings |
