# 001: ConservationError

## Representation

Why a conservation check could not be completed, or found a discrepancy.
Renamed from `exact_audit::AuditError` per the preferred design, with its
shape changed too: `AuditError::AccumulatorOverflow { at_entry }` becomes the
field-less `Overflow` — the position-tracking `at_entry` is dropped rather
than preserved, since the doc specifies this crate's error shape explicitly
(module doc comment, `src/lib.rs:56-63`). `NotZero { got : i128 }` is new,
added for the typed `money_sum_assert_zero`; `i128` is chosen to match
`Report::discrepancy_minor`, so both outcomes count in the same unit.

## Kind

Enum (§ Item Kind Taxonomy : Stable Item Kinds #7)

## Definition

`module/exact_conserve/src/lib.rs:122-133`

```rust
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum ConservationError
{
  NotZero
  {
    got : i128,
  },
  Overflow,
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 123-133 | Declaration |
| `src/lib.rs` | 151, 242, 282, 290 | Constructed on overflow/not-zero: in the private `KindError` mapping both `*_conserve_into` functions use (151), in `verify` (242), and in `money_sum_assert_zero` (282, 290) |
| `tests/conservation_test.rs` | throughout | Every error-path test |
| `cluster_economy/src/error.rs:19,79-81` | — | **Production** — wrapped into `MarketError::Audit` via a `From` impl |
| `cluster_economy/tests/economy_test.rs:493,549,559` | — | Constructed directly and downcast-matched |
| `exact_arith/tests/facade_test.rs:61` | — | Asserts `Overflow`'s rendered message |
| `exact_arith/src/lib.rs:141` | — | Facade re-export |

**Real production consumer outside this crate's own tier**:
`cluster_economy` wraps this error type into its own domain error enum — the
only error type in this entire migration confirmed to cross into a
downstream exchange/market crate's own error handling, not just its tests.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Every fallible function's error type |
| `exact_arith` | `src/lib.rs` | Re-export; own test asserts `Display` output |
| `cluster_economy` | `src/error.rs` | **Production** — wrapped via `From< ConservationError > for MarketError` |
