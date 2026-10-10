# 001: use exact_kind::{ KindError, Money, Quantity }

## Representation

Brings in the conserved value types and their error type for the typed
per-kind convenience layer (`money_conserve_into`, `qty_conserve_into`,
`money_sum_assert_zero`). `Entry`, `Report`, and
`verify` — the carried-forward `exact_audit` surface — touch none of these;
they stay dependency-free in their own logic exactly as before, per the
module doc comment's disclosed deviation (`src/lib.rs:46-55`).

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_conserve/src/lib.rs:83`

```rust
use exact_kind::{ KindError, Money, Quantity };
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 83 | Declaration |
| `src/lib.rs` | 149 | `KindError` — private error-mapping function's parameter |
| `src/lib.rs` | 255, 266 | `Money`/`Quantity` — `money_conserve_into`/`qty_conserve_into` parameter types |
| `src/lib.rs` | 277 | `Money` — `money_sum_assert_zero`'s slice element type |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Parameter/element types for the 3 typed convenience functions |
