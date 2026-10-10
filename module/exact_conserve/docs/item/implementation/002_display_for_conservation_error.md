# 002: Display for ConservationError

## Representation

Renders `ConservationError`'s two variants as human-readable sentences.
Unlike most error types elsewhere in this family, this one's `Display`
output IS actually asserted somewhere — see
[Display::fmt for ConservationError](../associated_function/002_fmt_display_for_conservation_error.md).

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_conserve/src/lib.rs:135-145`

```rust
impl core::fmt::Display for ConservationError
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    match self
    {
      Self::NotZero { got } => write!( f, "expected a zero sum, got {got} minor units" ),
      Self::Overflow => write!( f, "the running total left the representable range" ),
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 135-145 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | The one hand-written `Display` impl on `ConservationError` |
| `exact_arith` | `tests/facade_test.rs:61` | **Rendered** — asserts the exact `Overflow` message text |
