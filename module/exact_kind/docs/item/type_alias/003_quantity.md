# 003: Quantity

## Representation

A non-negative quantity at the standard money scale — `Qty< MONEY_SCALE >`.
The concrete name every downstream crate imports; mirrors
[`Money`](001_money.md) for the non-negative wrapper type instead of the
signed one.

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_kind/src/lib.rs:60`

```rust
pub type Quantity = Qty< MONEY_SCALE >;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 60,415-416 | Declaration; struct-doc-comment doctest on `Qty` |
| `tests/non_negative_test.rs` | throughout | Concrete type exercised by this crate's own test suite |
| `exact_parse/src/lib.rs:62,64` | — | `qty_from_str` |
| `exact_bytes/src/lib.rs:217,219,230,233` | — | `qty_to_wire`/`qty_from_wire` |
| `exact_dust/src/lib.rs:243,245,253,255,263` | — | `qty_dust_split`/`qty_dust_split_into`/`qty_dust_remainder` |
| `exact_conserve/src/lib.rs:266` | — | `qty_conserve_into` |
| `exact_add/src/lib.rs:73,83,142` | — | `qty_add`/`qty_sub`/`qty_saturating_add` |
| `exact_fmt/src/lib.rs`, `exact_cmp/src/lib.rs` | — | Imported alongside `Money`/`Price` |
| `exact_snap/src/lib.rs:117,128,166` | — | `Lot::new`, `Lot::qty`, `qty_snap_lot` |
| `exact_ratio/src/lib.rs:192,195,235` | — | `qty_mul_ratio`/`qty_div_round` |
| `exact_arith/src/lib.rs:88` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; exercised by its own tests and its own struct-doc doctest |
| `exact_parse`, `exact_bytes`, `exact_conserve`, `exact_dust`, `exact_add`, `exact_fmt`, `exact_cmp`, `exact_snap`, `exact_ratio` | `src/lib.rs` | The standard non-negative quantity type every one of these 9 crates' `qty_*` functions operates on |
| `exact_arith` | `src/lib.rs` | Re-export only |
