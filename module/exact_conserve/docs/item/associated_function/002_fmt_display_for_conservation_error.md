# 002: Display::fmt for ConservationError

## Representation

Renders `ConservationError::NotZero`/`Overflow` as their respective
sentences.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_conserve/src/lib.rs:137-144`

```rust
fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
{
  match self
  {
    Self::NotZero { got } => write!( f, "expected a zero sum, got {got} minor units" ),
    Self::Overflow => write!( f, "the running total left the representable range" ),
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 137-144 | Declaration |
| `exact_arith/tests/facade_test.rs:61` | — | Asserts the exact `Overflow` rendered text |

**An exception to the pattern elsewhere in this family**: every other error
type's `Display` impl so far (`exact_kind::KindError`, `exact_ratio::RatioError`,
`exact_snap::SnapError`, `exact_dust::DustError`) is never actually rendered
anywhere in the workspace. This one is — the facade's own test exercises it
directly by exact string match.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Declared only — not invoked by this crate's own tests |
| `exact_arith` | `tests/facade_test.rs` | **Rendered** |

## Caller Tree

- **External:** `exact_arith`'s own test, via `ConservationError::Overflow.to_string()` (`exact_arith/tests/facade_test.rs:61`) — the blanket `ToString` impl every `Display` type gets

No intra-crate caller.

## Callee Tree

- **External:** `core::write!` macro expansion over the given `Formatter`
