# 004: money_sum_assert_zero

## Representation

Assert a slice of money legs sums to exactly zero — the general
credit/debit conservation check, expressed directly over a typed slice
rather than `verify`'s untyped `Entry` log.

## Kind

Function (§ Item Kind Taxonomy : Stable Item Kinds #4)

## Definition

`module/exact_conserve/src/lib.rs:277-292`

```rust
pub fn money_sum_assert_zero( legs : &[ Money ] ) -> Result< (), ConservationError >
{
  let mut net : i128 = 0;
  for leg in legs
  {
    net = net.checked_add( i128::from( leg.minor() ) ).ok_or( ConservationError::Overflow )?;
  }
  if net == 0
  {
    Ok( () )
  }
  else
  {
    Err( ConservationError::NotZero { got : net } )
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 277-292 | Declaration |
| `tests/conservation_test.rs:257-272` | — | A cancelling slice, a one-unit leak, and an empty slice |
| `exact_arith/src/lib.rs:141` | — | Facade re-export |

Confirmed via a full-workspace grep: no call site anywhere outside this
crate's own tests.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Exercised by 3 of its own tests |
| `exact_arith` | `src/lib.rs` | Re-export only — not called by the facade's own test suite |

## Caller Tree

No caller anywhere, intra-crate or external — an honest empty tree.

## Callee Tree

No callee of its own — folds via `i128::from`/`i128::checked_add` directly
and constructs [ConservationError](../enum/001_conservation_error.md) on
failure; no further hop into another Item Instance. Notably does not reuse
[verify](001_verify.md)'s identical fold logic — the two duplicate the same
accumulation loop rather than one calling the other, since `verify` folds
`Entry::amount_minor` while this folds `Money::minor()` over a differently
shaped input.
