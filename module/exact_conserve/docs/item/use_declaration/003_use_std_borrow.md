# 003: use std::borrow::Borrow

## Representation

Brings in the trait [discrepancy_minor](../associated_function/004_discrepancy_minor.md)
bounds its lookup with: `A : Borrow< Q >` lets a report keyed by `String` or
`&str` be asked about `"cash"` directly, and a report keyed by an enum be
asked about the key itself — the same bound `BTreeMap::get` takes.

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_conserve/src/lib.rs:85`

```rust
use std::borrow::Borrow;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 85 | Declaration |
| `src/lib.rs` | 190 | `discrepancy_minor`'s `A : Borrow< Q >` bound |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | The lookup bound of `Report::discrepancy_minor` |
