# 003: Error for ConservationError

## Representation

Marks `ConservationError` as a standard error type, via the blanket-default
`core::error::Error` trait — no method body of its own.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_conserve/src/lib.rs:147`

```rust
impl core::error::Error for ConservationError {}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 147 | Declaration |

No file anywhere calls a method on this impl directly, but `cluster_economy`
depends on the trait bound it provides: `economy_test.rs:559` calls
`.source().and_then( | e | e.downcast_ref::< ConservationError >() )` against
a boxed error, which requires `ConservationError: Error` to compile.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Qualifies `ConservationError` as a standard error type |
| `cluster_economy` | `tests/economy_test.rs` | **Production-adjacent** — the `Error` bound enables `downcast_ref` through a boxed dynamic error |
