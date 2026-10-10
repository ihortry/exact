# 001: Money

## Representation

A value at the standard money scale — `Decimal< MONEY_SCALE >`. The
concrete, consumer-facing name every downstream crate (`exact_add`,
`exact_parse`, `exact_fmt`, `exact_bytes`, `exact_snap`, `exact_cmp`,
`exact_ratio`, `exact_dust`, `exact_conserve`) actually imports — the generic
`Decimal< const SCALE >` itself is rarely named outside this crate.

## Kind

Type Alias (§ Item Kind Taxonomy : Stable Item Kinds #5)

## Definition

`module/exact_kind/src/lib.rs:57`

```rust
pub type Money = Decimal< MONEY_SCALE >;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 57,642,648,651,660,709 | Declaration; `Price`'s wrapped `value` and the members that delegate to it (`ZERO`, `MAX`, `from_minor`, `parse`); also named by the `compile_fail` doctests on `Display for Qty` and `Price` |
| `tests/checked_arithmetic_test.rs`, `tests/parse_render_test.rs` | throughout | Concrete type exercised by this crate's own test suite |
| `tests/price_test.rs` | 6,13,21,45 | `Price`'s range checked against `Money`'s |
| `exact_parse/src/lib.rs:45,52,54` | — | Compile-time `ONE_MINOR` assert; `money_from_str`'s parameter/return/body |
| `exact_bytes/src/lib.rs:197,209,212` | — | `money_to_wire`/`money_from_wire` signatures and bodies |
| `exact_conserve/src/lib.rs:255,277` | — | `money_conserve_into`/`money_sum_assert_zero` signatures |
| `exact_dust/src/lib.rs:207,209,218,220,231` | — | `money_dust_split`/`money_dust_split_into`/`money_dust_remainder` |
| `exact_add/src/lib.rs:53,63,113,128` | — | `money_add`/`money_sub`/`money_checked_neg`/`money_saturating_add` |
| `exact_fmt/src/lib.rs`, `exact_cmp/src/lib.rs` | — | Imported (`use exact_kind::{ Money, .. }`); re-exported `Display`/derived `Ord` exercised through it |
| `exact_ratio/src/lib.rs:177,180,221,224` | — | `money_mul_ratio`/`money_div_round` |
| `exact_arith/src/lib.rs:88` | — | Facade re-export (`pub use exact_kind::{ .., Money, .. }`) |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Declared here; exercised by its own tests |
| `exact_parse`, `exact_bytes`, `exact_conserve`, `exact_dust`, `exact_add`, `exact_fmt`, `exact_cmp`, `exact_ratio` | `src/lib.rs` | The standard money type every one of these 8 Tier-2/3 crates' `money_*` functions operates on |
| `exact_arith` | `src/lib.rs` | Re-export only |
