# 001: money_add

## Representation

Add two money values, dispatching straight to `exact_kind`'s own checked
addition.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_add/src/lib.rs:53`

```rust
pub const fn money_add( a : Money, b : Money ) -> Result< Money, KindError >
{
  a.checked_add( b )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 53 | Declaration |
| `tests/checked_and_saturating_add_test.rs:18,71` | — | Dispatch parity with `Money::checked_add`; cross-check against `money_saturating_add` in range |
| `exact_conserve/src/lib.rs:257` | — | `money_conserve_into`'s own body |
| `exact_arith/src/lib.rs:92` | — | Facade re-export |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_add` | `(defining crate)` | Exercised by its own dispatch-parity test |
| `exact_conserve` | `src/lib.rs` | **Production** — folds one more money leg into a running total (`money_conserve_into`) |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test (see [readme](../readme.md) Notable Findings) |

## Caller Tree

- **External:** `exact_conserve::money_conserve_into` (`exact_conserve/src/lib.rs:257`)

No intra-crate caller — `money_add` is a standalone dispatcher, not called by any other function in `exact_add`.

## Callee Tree

- **External:** `exact_kind::Decimal::checked_add` (via `a.checked_add( b )`, `Money` being an alias for `Decimal< SCALE >`)
