# 001: Entry inherent impl

## Representation

The one hand-written impl on `Entry< K, A >`, providing its constructor for
any account type `K` and asset key type `A`, neither bounded.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_conserve/src/lib.rs:111-119`

```rust
impl< K, A > Entry< K, A >
{
  pub fn new( account : K, asset : A, amount_minor : i64 ) -> Self
  {
    Self { account, asset, amount_minor }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 111-119 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | The sole constructor for `Entry`; see [new for Entry](../associated_function/001_new_entry.md) for its heavily-used call-site evidence |
