# 004: discrepancy_minor

## Representation

One asset's signed discrepancy in minor units — `Some( 0 )` when that asset
balances, and `None` when the log never moved it, so a misspelt asset cannot
read as balanced. The asset is looked up by any `Q` the key type `A` borrows
as — `&str` for `String` or `&str` keys, the key itself for an enum. Signed
deliberately: the sign distinguishes value appearing from value vanishing,
which the doc comment calls "different investigations" (`src/lib.rs:185-186`).

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_conserve/src/lib.rs:188-194`

```rust
pub fn discrepancy_minor< Q >( &self, asset : &Q ) -> Option< i128 >
where
  A : Borrow< Q > + Ord,
  Q : Ord + ?Sized,
{
  self.nets.get( asset ).copied()
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 188-194 | Declaration |
| `src/lib.rs` | 228 | `verify`'s own doc-test |
| `tests/conservation_test.rs:34,56,59,72,93,128-129,156,181-182,202-203` | — | Asserts the exact signed leftover per asset — one cash leak and a cash-and-`BTC` pair that cancel only as a whole — `None` for an asset never moved or misspelt, and a log keyed by an enum |
| `exchange_core/tests/submission_test.rs:259` | — | Integration test |
| `smoke_exact_market_split/src/lib.rs:319`, `tests/lane_test.rs:68` | — | Demo-lane leak-magnitude assertion |

Confirmed via grep across the full workspace (not only `exact_conserve`'s
direct dependents): no production call site in `cluster_economy` or
`exchange_core`'s own non-test source — both consume `is_balanced` in
production but read the discrepancy amount, where they need it at all, only
in their own tests. A real, verified asymmetry between these two sibling
methods, not an oversight in this catalog.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Exercised across 5 distinct scenarios in its own tests |
| `exchange_core` | `tests/submission_test.rs` | Integration test |
| `smoke_exact_market_split` | `src/lib.rs`, `tests/lane_test.rs` | Demo-lane leak-magnitude assertion |

## Caller Tree

- **External:** `exchange_core`'s own integration test (`tests/submission_test.rs:259`)
- **External:** `smoke_exact_market_split`'s own demo-lane code and test (`src/lib.rs:319`, `tests/lane_test.rs:68`)

No intra-crate caller, and no confirmed production (non-test, non-demo-lane)
caller — narrower reach than its sibling `is_balanced`, which `cluster_economy`
does call from real settlement code.

## Callee Tree

No callee of its own — reads `self.nets` directly through `BTreeMap::get`;
no further hop into another Item Instance. No longer a `const fn`, for the
same reason as [is_balanced](003_is_balanced.md).
