# 003: Decimal::EPSILON

## Representation

The smallest non-zero magnitude this type can express — one minor unit.

## Kind

Associated Constant (§ Item Kind Taxonomy : Associated Item Kinds #2)

## Definition

`module/exact_kind/src/lib.rs:182`

```rust
pub const EPSILON : Self = Self { minor : minor_from_i64( 1 ) };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 182,431 | Declaration; `Qty::EPSILON`'s own definition wraps this value |
| `tests/checked_arithmetic_test.rs` | throughout | Boundary-adjacent values (`MAX.checked_add(EPSILON)` etc.) |
| `exact_add/tests/checked_and_saturating_add_test.rs:60,61,62` | — | Saturation-boundary test inputs |
| `exact_conserve/tests/conservation_test.rs:243` | — | Overflow-boundary test input |

No production (non-test) file outside `exact_kind` uses `Decimal::EPSILON` —
an honest gap: every external reference found is a boundary-condition test
input, not a runtime dependency.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::EPSILON`; exercised by its own boundary tests |
| `exact_add`, `exact_conserve` | `tests/*.rs` | Test-only boundary input |
