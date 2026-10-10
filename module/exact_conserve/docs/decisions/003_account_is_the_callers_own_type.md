# ADR-003: The Account Is The Caller's Own Type

**Date**: 2026-10-10
**Status**: Proposed
**Deciders**: Ihor Filimonov, for wandalen's review

## Context

[ADR-002](002_conservation_checked_per_asset.md) made `Entry` generic over its
asset, but left its account fixed: the field was a `String`, and `Entry::new`
took any account convertible into one.

Nothing in this crate reads `account`. The field's own doc says it is carried
for reporting, never for arithmetic, and the module doc says per-account totals
are deliberately not computed. Yet every caller had to turn its account key
into a `String`. The exchange, whose accounts are a `Copy` newtype
`AccountId( u64 )`, formats `"account:{}"` and allocates once per posting only
to satisfy this signature, and anyone reading those postings back has to parse
the string to recover the id.

## Decision

- `Entry< K, A >` takes the account type as its first parameter, matching the
  field order and `new( account, asset, … )`. `account : K` is the caller's own
  type, exactly as `asset : A` already is.
- `K` carries no bound anywhere: not on the struct, not on `Entry::new`, not on
  `verify< K, A : Ord + Clone >`. `verify` never reads `account`. The derived
  `Debug`, `Clone`, `PartialEq` and `Eq` hold per parameter, so a `K` lacking
  one of them loses only that impl.
- `Entry::new` takes `account : K` by value, not `impl Into< K >`.
- No default type parameter: `Entry< K, A >`, not `Entry< A, K = String >`.
- `Report< A >` is unchanged: it holds no accounts.

## Alternatives Considered

### Option 1: `new( account : impl Into< K > )`

Rejected: with `Into`, a call like `Entry::new( "a", … )` no longer determines
`K`. It could be `&str`, `String`, or anything else built from a `&str`, so
every caller would need a type annotation. Taking `K` directly lets inference
work.

### Option 2: A default `K = String`

Rejected: the crate is still pre-release, so a default would only preserve the
stringly-typed path this change removes. It would also force the parameter to
the end of the list, against the field order.

### Option 3: Asset first, `Entry< A, K >`

Rejected: it keeps the asset as the leading parameter, but existing
annotations `Entry< &str >` break either way, and the order would disagree with
`new( account, asset, … )` and the fields. Where both types are `&str`, as in
this crate's tests, swapping the two still type-checks. That is harmless there,
and in the exchange the two types differ, so the compiler catches a swap.

## Consequences

**Positive:**
- A caller posts against its own account key: an id, a name, or a reference
  into its own records. The exchange drops its `format!( "account:{}", … )`,
  so no posting allocates, and reads its `AccountId` back without parsing.
- An account type with no traits at all still audits
  (`an_account_of_any_type_audits`).

**Negative:**
- Breaking: every single-parameter `Entry` annotation becomes `Entry< K, A >`, an
  explicitly typed empty log names both types (`verify::< (), &str >( &[] )`),
  and a caller passing a `String`-convertible account other than the type it
  stores must convert it itself. Inside this workspace: `exact_conserve`'s own
  tests and `smoke_exact_market_split`'s `ledger`. Outside it: the exchange's
  `postings`.

**Neutral:**
- `Entry::new( "a", "cash", 5 )` still compiles unchanged: `K` is inferred as
  `&str`.
- `verify`'s arithmetic, `ConservationError` and the typed layer are unchanged.

## Related

- [ADR-002](002_conservation_checked_per_asset.md) — the asset key this mirrors
- [Entry And Report](../type/001_entry_and_report.md) — the shape this changes
- [Entry](../item/struct/001_entry.md) — the struct's item page
