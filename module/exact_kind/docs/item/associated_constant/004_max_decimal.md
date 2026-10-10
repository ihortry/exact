# 004: Decimal::MAX

## Representation

The largest value this type can hold — exactly the declared ceiling. The
clamp target for saturating arithmetic: a wider clamp (to the raw backing
width) would produce a minor count `from_minor` itself would refuse.

## Kind

Associated Constant (§ Item Kind Taxonomy : Associated Item Kinds #2)

## Definition

`module/exact_kind/src/lib.rs:189`

```rust
pub const MAX : Self = Self { minor : minor_from_i64( CEILING_MINOR_UNITS ) };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 189 | Declaration |
| `tests/checked_arithmetic_test.rs` | throughout | Ceiling-boundary checks |
| `exact_bytes/tests/wire_roundtrip_test.rs:87` | — | Past-ceiling boundary input |
| `exact_add/src/lib.rs:133` | — | **Production** — `money_saturating_add`'s positive clamp target |
| `exact_add/tests/checked_and_saturating_add_test.rs:61` | — | Saturation test |
| `exact_conserve/tests/conservation_test.rs:243` | — | Overflow-boundary test input |
| `exact_ratio/tests/ratio_and_div_round_test.rs:45` | — | Saturation-adjacent test |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Exercised by its own ceiling-boundary tests |
| `exact_add` | `src/lib.rs` | **Production** — `money_saturating_add`'s clamp target when the checked add overflows positively |
| `exact_bytes`, `exact_conserve`, `exact_ratio` | `tests/*.rs` | Test-only boundary input |
