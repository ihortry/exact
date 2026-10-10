# 005: Display for Report

## Representation

Renders a `Report` as either `"balanced: entries N, net 0"` or
`"UNBALANCED: entries N, net A M, B K minor units"` — every asset whose net
is not zero, with its net, in asset order — branching on `is_balanced()`.
Implemented only for an asset key `A : Display`, since each unbalanced asset
is printed by name.

## Kind

Implementation (§ Item Kind Taxonomy : Stable Item Kinds #12)

## Definition

`module/exact_conserve/src/lib.rs:197-217`

```rust
impl< A : core::fmt::Display > core::fmt::Display for Report< A >
{
  fn fmt( &self, f : &mut core::fmt::Formatter< '_ > ) -> core::fmt::Result
  {
    if self.is_balanced()
    {
      write!( f, "balanced: entries {}, net 0", self.entries )
    }
    else
    {
      write!( f, "UNBALANCED: entries {}, net", self.entries )?;
      let mut first = true;
      for ( asset, net ) in self.nets.iter().filter( | ( _, net ) | **net != 0 )
      {
        write!( f, "{} {asset} {net}", if first { "" } else { "," } )?;
        first = false;
      }
      write!( f, " minor units" )
    }
  }
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 197-217 | Declaration |
| `tests/conservation_test.rs:76-85` | — | `the_report_renders_both_outcomes_in_words` — asserts both branches' exact text |
| `tests/conservation_test.rs:147,172` | — | A balanced two-asset log, and two unbalanced assets named in asset order with a balanced one left out |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | **Rendered** — its own test exercises both the balanced and unbalanced branches by exact string match |
