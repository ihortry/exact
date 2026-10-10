# 001: new for Entry

## Representation

Build a posting from an account of the caller's own type `K`, an asset of
the caller's own key type `A`, and a signed minor-unit amount. Takes `K` by
value rather than `impl Into< K >`, so `Entry::new( "a", … )` still infers `K`.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_conserve/src/lib.rs:115-118`

```rust
pub fn new( account : K, asset : A, amount_minor : i64 ) -> Self
{
  Self { account, asset, amount_minor }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 115-118 | Declaration |
| `src/lib.rs` | 224,227 | `verify`'s own doc-test |
| `tests/conservation_test.rs` | 20,52,58,67,82,91,105,120-123,138-141,166-169,180,195-198,215 | `transfer` helper, the single- and multi-asset logs, a log keyed by an enum, and an account of a type with no traits |
| `exchange_core/src/lib.rs:460-461` | — | **Production** |
| `cluster_economy/src/market.rs:504-505,515-516` | — | **Production** |
| `cluster_economy/tests/economy_test.rs:255-261` | — | Reconciliation assertion setup |
| `smoke_exact_market_split/src/lib.rs:127-128` | — | Demo-lane ledger postings |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Exercised throughout its own tests and doc-test |
| `exchange_core` | `src/lib.rs` | **Production** — settlement postings |
| `cluster_economy` | `src/market.rs` | **Production** — settlement postings |
| `smoke_exact_market_split` | `src/lib.rs` | Demo-lane postings |

## Caller Tree

- **External:** `exchange_core::<settlement path>` (`exchange_core/src/lib.rs:460-461`)
- **External:** `cluster_economy::market::<settlement path>` (`cluster_economy/src/market.rs:504-505,515-516`)
- **External:** `smoke_exact_market_split::<ledger path>` (`smoke_exact_market_split/src/lib.rs:127-128`)

No intra-crate caller — `Entry::new` is a leaf constructor within
`exact_conserve` itself. The crate's most externally-called Item by a wide
margin: real production callers in two independent downstream crates, not
counting the demo lane or either crate's own tests.

## Callee Tree

No callee — moves its three arguments into the struct as given.
