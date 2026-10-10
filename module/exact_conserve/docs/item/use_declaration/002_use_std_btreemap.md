# 002: use std::collections::BTreeMap

## Representation

Brings in the map [Report](../struct/002_report.md) keeps one net per asset
in, and [verify](../function/001_verify.md) folds into. A `BTreeMap` rather
than a `HashMap` because it iterates in key order: the same log always lists
its assets in the same order, so `Report`'s `Display` text and `Debug` output
never vary between runs (→ [Conservation Is Checked Per Asset](../../decisions/002_conservation_checked_per_asset.md)).

## Kind

Use Declaration (§ Item Kind Taxonomy : Stable Item Kinds #3)

## Definition

`module/exact_conserve/src/lib.rs:84`

```rust
use std::collections::BTreeMap;
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 84 | Declaration |
| `src/lib.rs` | 163 | `Report::nets` field type |
| `src/lib.rs` | 236 | `verify`'s per-asset accumulator |

The mention at line 160 is a doc comment, not a usage.

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_conserve` | `(defining crate)` | The per-asset net map of `Report` and `verify` |
