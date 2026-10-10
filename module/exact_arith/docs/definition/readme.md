# Definition Doc Definition

### Scope

- **Purpose**: Index every public definition reachable through this crate in one place, so a reader can find where something is actually declared without grepping.
- **Responsibility**: A flat module index — one row per public item, not a duplicate explanation of each.
- **In Scope**: Every `pub` item named in `src/lib.rs`'s `pub use` lists.
- **Out of Scope**: Rationale and invariants for any one item — link to the owning leaf crate's own doc definition instead of restating it here; this crate has no `impl` blocks of its own, so there are no associated items to enumerate beyond the 98 top-level names below.

### Module Index

Every row's "Declared" column points to the leaf crate that actually
declares the item — this facade declares none of them itself (→
[Facade Re-Exports Only](../invariant/001_facade_re_exports_only.md)). "Documented
in" is `—` throughout: an individual item's own rationale lives in its
declaring crate's own docs, out of this crate's scope per the Scope block
above; this crate's own `feature/`/`invariant/` instances document the
facade's aggregate commitments, never one re-exported name's specific
behaviour.

| Item | Kind | Declared | Documented in |
|------|------|----------|----------------|
| `Backing` | type alias | `../../../exact_minor/src/lib.rs:39` | — |
| `Minor` | struct | `../../../exact_minor/src/lib.rs:51` | — |
| `minor_from_i64` | fn | `../../../exact_minor/src/lib.rs:55` | — |
| `minor_to_i64` | fn | `../../../exact_minor/src/lib.rs:62` | — |
| `MinorError` | enum | `../../../exact_minor/src/lib.rs:206` | — |
| `minor_checked_add` | fn | `../../../exact_minor/src/lib.rs:256` | — |
| `minor_checked_neg` | fn | `../../../exact_minor/src/lib.rs:288` | — |
| `minor_checked_sub` | fn | `../../../exact_minor/src/lib.rs:272` | — |
| `minor_is_zero` | fn | `../../../exact_minor/src/lib.rs:245` | — |
| `minor_saturating_add` | fn | `../../../exact_minor/src/lib.rs:304` | — |
| `minor_saturating_sub` | fn | `../../../exact_minor/src/lib.rs:312` | — |
| `minor_zero` | fn | `../../../exact_minor/src/lib.rs:238` | — |
| `CEILING_MINOR_UNITS` | const | `../../../exact_scale/src/lib.rs:45` | — |
| `CEILING_WHOLE_UNITS` | const | `../../../exact_scale/src/lib.rs:29` | — |
| `HEADROOM_FACTOR` | const | `../../../exact_scale/src/lib.rs:22` | — |
| `MONEY_SCALE` | const | `../../../exact_scale/src/lib.rs:37` | — |
| `pow10` | fn | `../../../exact_scale/src/lib.rs:64` | — |
| `Rounding` | enum | `../../../exact_round/src/lib.rs:38` | — |
| `RoundError` | enum | `../../../exact_round/src/lib.rs:110` | — |
| `round_div` | fn | `../../../exact_round/src/lib.rs:147` | — |
| `round_div_wide` | fn | `../../../exact_round/src/lib.rs:171` | — |
| `rounding_default` | fn | `../../../exact_round/src/lib.rs:83` | — |
| `rounding_name` | fn | `../../../exact_round/src/lib.rs:93` | — |
| `Sign` | enum | `../../../exact_sign/src/lib.rs:25` | — |
| `sign_is_negative` | fn | `../../../exact_sign/src/lib.rs:57` | — |
| `sign_is_zero` | fn | `../../../exact_sign/src/lib.rs:64` | — |
| `sign_neg_allowed` | fn | `../../../exact_sign/src/lib.rs:78` | — |
| `sign_of` | fn | `../../../exact_sign/src/lib.rs:39` | — |
| `Decimal` | struct | `../../../exact_kind/src/lib.rs:157` | — |
| `KindError` | enum | `../../../exact_kind/src/lib.rs:68` | — |
| `Money` | type alias | `../../../exact_kind/src/lib.rs:57` | — |
| `Price` | struct | `../../../exact_kind/src/lib.rs:640` | — |
| `Qty` | struct | `../../../exact_kind/src/lib.rs:420` | — |
| `Quantity` | type alias | `../../../exact_kind/src/lib.rs:60` | — |
| `money_add` | fn | `../../../exact_add/src/lib.rs:53` | — |
| `money_checked_neg` | fn | `../../../exact_add/src/lib.rs:113` | — |
| `money_saturating_add` | fn | `../../../exact_add/src/lib.rs:128` | — |
| `money_sub` | fn | `../../../exact_add/src/lib.rs:63` | — |
| `price_add` | fn | `../../../exact_add/src/lib.rs:93` | — |
| `price_sub` | fn | `../../../exact_add/src/lib.rs:103` | — |
| `qty_add` | fn | `../../../exact_add/src/lib.rs:73` | — |
| `qty_saturating_add` | fn | `../../../exact_add/src/lib.rs:142` | — |
| `qty_sub` | fn | `../../../exact_add/src/lib.rs:83` | — |
| `Ratio` | struct | `../../../exact_ratio/src/lib.rs:96` | — |
| `RatioError` | enum | `../../../exact_ratio/src/lib.rs:52` | — |
| `money_div_round` | fn | `../../../exact_ratio/src/lib.rs:221` | — |
| `money_mul_ratio` | fn | `../../../exact_ratio/src/lib.rs:177` | — |
| `price_mul_qty` | fn | `../../../exact_ratio/src/lib.rs:253` | — |
| `price_mul_ratio` | fn | `../../../exact_ratio/src/lib.rs:203` | — |
| `qty_div_round` | fn | `../../../exact_ratio/src/lib.rs:235` | — |
| `qty_mul_ratio` | fn | `../../../exact_ratio/src/lib.rs:192` | — |
| `ratio_new` | fn | `../../../exact_ratio/src/lib.rs:140` | — |
| `money_from_str` | fn | `../../../exact_parse/src/lib.rs:52` | — |
| `price_from_str` | fn | `../../../exact_parse/src/lib.rs:72` | — |
| `qty_from_str` | fn | `../../../exact_parse/src/lib.rs:62` | — |
| `FmtError` | enum | `../../../exact_fmt/src/lib.rs:45` | — |
| `fmt_into` | fn | `../../../exact_fmt/src/lib.rs:95` | — |
| `money_fmt` | fn | `../../../exact_fmt/src/lib.rs:105` | — |
| `price_fmt` | fn | `../../../exact_fmt/src/lib.rs:119` | — |
| `qty_fmt` | fn | `../../../exact_fmt/src/lib.rs:112` | — |
| `KIND_MONEY` | const | `../../../exact_bytes/src/lib.rs:42` | — |
| `KIND_PRICE` | const | `../../../exact_bytes/src/lib.rs:46` | — |
| `KIND_QTY` | const | `../../../exact_bytes/src/lib.rs:44` | — |
| `Wire` | struct | `../../../exact_bytes/src/lib.rs:121` | — |
| `WireError` | enum | `../../../exact_bytes/src/lib.rs:57` | — |
| `money_from_wire` | fn | `../../../exact_bytes/src/lib.rs:209` | — |
| `money_to_wire` | fn | `../../../exact_bytes/src/lib.rs:197` | — |
| `price_from_wire` | fn | `../../../exact_bytes/src/lib.rs:248` | — |
| `price_to_wire` | fn | `../../../exact_bytes/src/lib.rs:238` | — |
| `qty_from_wire` | fn | `../../../exact_bytes/src/lib.rs:230` | — |
| `qty_to_wire` | fn | `../../../exact_bytes/src/lib.rs:217` | — |
| `Lot` | struct | `../../../exact_snap/src/lib.rs:108` | — |
| `SnapError` | enum | `../../../exact_snap/src/lib.rs:29` | — |
| `Tick` | struct | `../../../exact_snap/src/lib.rs:74` | — |
| `price_snap_tick` | fn | `../../../exact_snap/src/lib.rs:141` | — |
| `qty_snap_lot` | fn | `../../../exact_snap/src/lib.rs:166` | — |
| `money_cmp` | fn | `../../../exact_cmp/src/lib.rs:36` | — |
| `money_eq` | fn | `../../../exact_cmp/src/lib.rs:57` | — |
| `price_cmp` | fn | `../../../exact_cmp/src/lib.rs:50` | — |
| `price_max` | fn | `../../../exact_cmp/src/lib.rs:71` | — |
| `price_min` | fn | `../../../exact_cmp/src/lib.rs:64` | — |
| `qty_cmp` | fn | `../../../exact_cmp/src/lib.rs:43` | — |
| `DustError` | enum | `../../../exact_dust/src/lib.rs:78` | — |
| `DustTo` | enum | `../../../exact_dust/src/lib.rs:65` | — |
| `money_dust_remainder` | fn | `../../../exact_dust/src/lib.rs:231` | — |
| `money_dust_split` | fn | `../../../exact_dust/src/lib.rs:207` | — |
| `money_dust_split_into` | fn | `../../../exact_dust/src/lib.rs:218` | — |
| `qty_dust_remainder` | fn | `../../../exact_dust/src/lib.rs:263` | — |
| `qty_dust_split` | fn | `../../../exact_dust/src/lib.rs:243` | — |
| `qty_dust_split_into` | fn | `../../../exact_dust/src/lib.rs:253` | — |
| `ConservationError` | enum | `../../../exact_conserve/src/lib.rs:123` | — |
| `Entry` | struct | `../../../exact_conserve/src/lib.rs:99` | — |
| `Report` | struct | `../../../exact_conserve/src/lib.rs:156` | — |
| `money_conserve_into` | fn | `../../../exact_conserve/src/lib.rs:255` | — |
| `money_sum_assert_zero` | fn | `../../../exact_conserve/src/lib.rs:277` | — |
| `qty_conserve_into` | fn | `../../../exact_conserve/src/lib.rs:266` | — |
| `verify` | fn | `../../../exact_conserve/src/lib.rs:234` | — |

### Regenerate

```sh
cd "$(git rev-parse --show-toplevel)"/module/exact_arith
printf 'pub use statements in src/lib.rs: '; grep -c '^pub use' src/lib.rs
printf 'rows in Module Index:             '; grep -cE '^\| `' docs/definition/readme.md
# pub use statements in src/lib.rs: 14
# rows in Module Index:             98
```
