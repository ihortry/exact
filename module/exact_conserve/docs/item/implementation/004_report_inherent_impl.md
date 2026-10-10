# 004: Report inherent impl

## Representation

The hand-written impl on `Report`, providing its two query methods —
`is_balanced` (a yes/no verdict) and `discrepancy_minor` (the signed amount
behind that verdict). Deliberately does not compute per-account totals; the
module doc comment explains why (`src/lib.rs:24-28`): an account's non-zero
balance is normal, so reporting it alongside the one non-zero that actually
matters would bury the real signal.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_conserve/src/lib.rs:166-195`

```rust
impl< A > Report< A >
{
  pub fn is_balanced( &self ) -> bool
  {
    self.nets.values().all( | net | *net == 0 )
  }

  pub fn discrepancy_minor< Q >( &self, asset : &Q ) -> Option< i128 >
  where
    A : Borrow< Q > + Ord,
    Q : Ord + ?Sized,
  {
    self.nets.get( asset ).copied()
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 166-195 | Declaration |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | Both methods are this crate's own public read surface on `Report` |
