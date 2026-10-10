# Invariant: Sum-To-Zero Is Exact

### Scope

- **Purpose**: State that a balanced log's net is exactly zero, never merely close to it, so a caller can treat `is_balanced`/`money_sum_assert_zero` as a definitive verdict rather than one more check to re-verify.
- **Responsibility**: `verify`'s plain-log fold and `Report::is_balanced`; `money_sum_assert_zero`'s typed-slice equivalent.
- **In Scope**: The accumulator width, the overflow handling that protects it, and the zero comparison itself.
- **Out of Scope**: Where a non-zero net is attributed to (a specific transaction or account) — this crate reports only the aggregate (→ `docs/algorithm/001_conservation_verification_fold.md`'s "Why Per-Account Totals Are Not Computed" and "...Not Per-Transaction Grouping" sections); the checked arithmetic `money_conserve_into`/`qty_conserve_into` dispatch to (→ `exact_add`'s own crate).

### Statement

A log this crate audits is balanced if and only if every asset's signed sum
is exactly zero, and a typed slice if and only if its signed sum is — no
tolerance window, no rounding, no "close enough." `verify`
(`src/lib.rs:234-247`) accumulates every `Entry::amount_minor` (`i64`) into
its own asset's `i128` total via `checked_add`, never adding amounts of
different assets together; `Report::is_balanced` (`src/lib.rs:176-179`) is
then bare equality of every total with zero.
`money_sum_assert_zero` (`src/lib.rs:277-292`) runs the identical
accumulate-then-compare shape over a typed `&[Money]` slice instead of `&[Entry]`, and apply the same
`== 0` equality with the same no-tolerance rule. The accumulator is strictly
wider than any single posting (`i128` against `i64`), and the fold is
`checked_add` throughout, so the zero comparison is never reached by a value
that already silently wrapped on the way there — a failure to complete
returns `ConservationError::Overflow` instead.

### Rationale

A tolerance is how a conservation check comes to pass the only errors small
enough to be worth hiding: an off-by-one-unit leak repeated across a million
transactions is exactly the failure this crate exists to make impossible, and
it is invisible to any check that ignores single units. Widening the
accumulator to `i128` before comparing to zero is what makes the exact
comparison safe to make unconditionally — without that headroom, a
legitimately large but balanced log could overflow the accumulator before the
comparison ever runs, forcing a choice between a tolerance and a false
`Overflow` on healthy input. With the headroom, neither compromise is needed:
the comparison stays exact, and `Overflow` is reserved for logs that
genuinely exceed what even `i128` can hold.

### Sources

| File | Relationship |
|------|--------------|
| `src/lib.rs:234-247` | `verify` — the plain-log fold, `checked_add` into an `i128` accumulator |
| `src/lib.rs:176-179` | `Report::is_balanced` — bare equality with zero, no tolerance |
| `src/lib.rs:277-292` | `money_sum_assert_zero` — the typed-slice equivalent |
| `../algorithm/001_conservation_verification_fold.md` | The fold's full procedure, including why `i128` and not a declared maximum log length |

### Tests

| File | Relationship |
|------|--------------|
| `tests/conservation_test.rs` | `one_unit_stays_visible_against_a_million_units_of_turnover` — a single unit of leakage survives detection against a thousand-transfer log; `the_accumulator_holds_a_total_the_posting_type_could_not` — the `i128` headroom itself, under test |
