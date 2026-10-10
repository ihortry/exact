# 003: Decimal::minor

## Representation

The count of minor units this value holds — the one escape hatch from the
type back to a raw integer.

## Kind

Associated Function/Method (§ Item Kind Taxonomy : Associated Item Kinds #1)

## Definition

`module/exact_kind/src/lib.rs:227`

```rust
pub const fn minor( self ) -> Backing
{
  minor_to_i64( self.minor )
}
```

## File Usage

| File | Line(s) | Context |
|------|---------|---------|
| `src/lib.rs` | 227,443,494 | Declaration; `Qty::from_decimal`'s negativity check; `Qty::minor`'s delegation |
| `tests/*.rs` (all 3) | throughout | Minor-count assertions |
| `exact_bytes/src/lib.rs:199,240` | — | `money_to_wire`/`price_to_wire` |
| `exact_conserve/src/lib.rs:282` | — | `money_sum_assert_zero`'s per-leg accumulation |
| `exact_dust/src/lib.rs:209,220,233` | — | `money_dust_split`/`_into`/`_remainder` |
| `exact_snap/src/lib.rs:86,91,152-153` | — | `Tick::new`'s zero check and magnitude; `price_snap_tick` (price and tick) |
| `exact_ratio/src/lib.rs:179,206,223` | — | `money_mul_ratio`, `price_mul_ratio`, `money_div_round` |

## Crate Usage

| Crate | Via File | Purpose |
|-------|----------|---------|
| `exact_kind` | `(defining crate)` | Backs `Qty::from_decimal`'s refusal and `Qty::minor`'s delegation |
| `exact_bytes`, `exact_conserve`, `exact_dust`, `exact_snap`, `exact_ratio` | `src/lib.rs` | **Production** — the standard way every one of these 5 crates reads a `Money`/`Price` value's raw minor count |

## Caller Tree

- [Qty::from_decimal](010_from_decimal_qty.md) (`src/lib.rs:443`)
- [Qty::minor](014_minor_qty.md) (`src/lib.rs:494`)
- **External:** `exact_bytes::money_to_wire` (`:199`), `price_to_wire` (`:240`)
- **External:** `exact_conserve::money_sum_assert_zero` (`:282`)
- **External:** `exact_dust::money_dust_split` (`:209`), `money_dust_split_into` (`:220`), `money_dust_remainder` (`:233`)
- **External:** `exact_snap::Tick::new` (`:86,91`), `price_snap_tick` (`:152-153`)
- **External:** `exact_ratio::money_mul_ratio` (`:179`), `price_mul_ratio` (`:205`), `money_div_round` (`:223`)

## Callee Tree

- None — a pure field access.
