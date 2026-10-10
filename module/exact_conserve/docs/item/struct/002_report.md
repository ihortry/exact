# 002: Report

## Representation

The outcome of auditing a log: how many postings were folded, and each
asset's signed net in minor units, keyed by asset in a `BTreeMap` so its
order never varies between runs. Carried forward from `exact_audit`, whose
single `net_minor` summed every asset together; no longer `Copy`, since it
holds a map. Generic over the same asset key `A` as the log it reports on.

## Kind

Struct (§ Item Kind Taxonomy : Stable Item Kinds #6)

## Definition

`module/exact_conserve/src/lib.rs:155-164`

```rust
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct Report< A >
{
  pub entries : usize,
  pub nets : BTreeMap< A, i128 >,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 156-164 | Declaration |
| `src/lib.rs` | 246 | Constructed by `verify` as its return value |
| `tests/conservation_test.rs` | throughout | Every test inspects a `Report` returned by `verify` |
| `exact_arith/src/lib.rs:141` | — | Facade re-export |

No production call site anywhere constructs a `Report` directly — only
`verify` does (its sole constructor). `exchange_core` and `cluster_economy`
both consume `verify`'s return value but never build a `Report` by hand.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | `verify`'s return type; exercised throughout its own test suite |
| `exact_arith` | `src/lib.rs` | Re-export only |
