# 003: qty_add

## Representation

Add two quantities, dispatching straight to `exact_kind`'s own checked
addition.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_add/src/lib.rs:73`

```rust
pub const fn qty_add( a : Quantity, b : Quantity ) -> Result< Quantity, KindError >
{
  a.checked_add( b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 73 | Declaration |
| `tests/checked_and_saturating_add_test.rs:30,82` | — | Non-negativity refusal carried through; cross-check against `qty_saturating_add` in range |
| `exact_conserve/src/lib.rs:268` | — | `qty_conserve_into`'s own body |
| `exact_arith/src/lib.rs:98` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_add` | `(defining crate)` | Exercised by its own non-negativity test |
| `exact_conserve` | `src/lib.rs` | **Production** — folds one more quantity leg into a running total (`qty_conserve_into`) |
| `exact_arith` | `src/lib.rs` | Re-export only |

## Caller Tree

- **External:** `exact_conserve::qty_conserve_into` (`exact_conserve/src/lib.rs:268`)

No intra-crate caller.

## Callee Tree

- **External:** `exact_kind::Qty::checked_add` (via `a.checked_add( b )`, `Quantity` being an alias for `Qty< SCALE >`)
