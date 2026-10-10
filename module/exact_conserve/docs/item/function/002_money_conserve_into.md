# 002: money_conserve_into

## Representation

Fold one more money leg into a running total via `exact_add`'s own checked
arithmetic. Named and shaped for `Iterator::try_fold`.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_conserve/src/lib.rs:255-258`

```rust
pub fn money_conserve_into( acc : Money, leg : Money ) -> Result< Money, ConservationError >
{
  exact_add::money_add( acc, leg ).map_err( kind_error_to_conservation_error )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 255-258 | Declaration |
| `tests/conservation_test.rs:231-244` | — | `try_fold` usage and an overflow case |
| `tests/conservation_test.rs:296` | — | Credits and debits folding back to exactly zero |
| `exact_arith/src/lib.rs:141` | — | Facade re-export |

Confirmed via a full-workspace grep: no call site anywhere outside this
crate's own tests — not `exact_arith`'s own test suite, not `cluster_economy`
or `exchange_core`. The typed convenience layer this function belongs to has
zero production or cross-crate test consumers, unlike `verify`/`Entry`/
`is_balanced`, which this same crate also exports.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Exercised by its own 3 tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test suite |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

- **External:** `exact_add::money_add` (`exact_add/src/lib.rs:53`)
- `kind_error_to_conservation_error` (`src/lib.rs:149`, private — no Item Instance of its own)
