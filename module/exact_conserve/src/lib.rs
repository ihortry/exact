//! Conservation auditing: does a set of postings sum to zero?
//!
//! Carries `exact_audit`'s whole-log auditor forward — [`Entry`], [`Report`],
//! and [`verify`] still take a plain `i64`-amount posting and compute an
//! `i128`-widened net total, now one per asset — and adds a typed per-kind
//! convenience layer on top: [`money_conserve_into`]/
//! [`qty_conserve_into`] for folding one typed leg at a time via `exact_add`,
//! and [`money_sum_assert_zero`] for asserting a whole slice of money legs
//! conserves.
//!
//! # What conservation means here
//!
//! A log is a sequence of signed postings. Value is neither created nor
//! destroyed by a transfer, so every transfer contributes one credit and one
//! matching debit, and the whole log therefore sums to zero. A non-zero sum
//! is a *discrepancy*: value appeared or vanished between two postings.
//!
//! The sum is taken per asset. Each [`Entry`] names what moved — a currency or
//! an instrument — and amounts of different assets are never added together:
//! a log that invents one unit of cash and loses one unit of an instrument
//! sums to zero as a whole, yet conserves neither. A log balances only when
//! every asset's own sum is zero.
//!
//! Per-account totals are deliberately not computed. An account's total is
//! its balance, and a non-zero balance is the normal state of an account,
//! not a finding — reporting balances alongside a conservation verdict would
//! put a column of expected non-zeros next to the one non-zero that means
//! something.
//!
//! # Widths
//!
//! Postings are `i64`, matching the family's backing width; [`verify`]'s and
//! [`money_sum_assert_zero`]'s accumulators are `i128`, strictly wider
//! — the fold is still checked, so even the length at which `i128` would run
//! out — somewhere past `2⁶⁴` maximal postings — returns an error rather
//! than wrapping.
//!
//! # Disclosed deviations from `exact_audit` and the preferred design
//!
//! - **No longer zero-dependency.** `exact_audit`'s own
//!   `docs/decisions/001_zero_dependency_by_contract.md` fixed its manifest
//!   at no `[dependencies]` at all, enforced by a manifest-reading test, so
//!   the auditor could never reach into this family's other value types.
//!   The preferred design's own dependency-tree edge
//!   (`exact_conserve → exact_add, exact_kind`) retires that Contract for
//!   this crate: the new typed layer genuinely needs `exact_kind`'s `Money`/
//!   `Quantity` and `exact_add`'s checked arithmetic. [`Entry`], [`Report`],
//!   and [`verify`] themselves still touch neither — they remain exactly as
//!   dependency-free *in their own logic* as before, carrying forward the
//!   original Contract's actual engineering value (an auditor testable
//!   against a log from any source, not only live in-process values) even
//!   though the crate's manifest as a whole is no longer empty. The
//!   manifest-reading test (`the_manifest_declares_no_dependencies`) is
//!   therefore dropped rather than ported — it tests a property this crate
//!   deliberately no longer holds.
//! - **`AuditError` is renamed `ConservationError`, per the preferred
//!   design**, and its single variant's shape changes with it:
//!   `AuditError::AccumulatorOverflow { at_entry }` becomes the doc's bare
//!   `ConservationError::Overflow` (no field) — the doc specifies this
//!   crate's error shape explicitly, unlike most others in this family that
//!   leave it to be inferred, so the position-tracking `at_entry` field is
//!   dropped rather than preserved as a deviation. A caller that needs to
//!   bisect a failing log can still do so externally.
//! - **`ConservationError::NotZero { got : i128 }`.** The doc does not say
//!   what type `got` carries. `i128` matches [`Report::discrepancy_minor`]'s
//!   own type, so a typed-layer failure and a plain-log discrepancy are
//!   counted in the same unit.
//! - **`money_conserve_into`/`qty_conserve_into` are prefixed per kind**,
//!   matching this family's established convention (`exact_add`,
//!   `exact_ratio`, `exact_cmp`, …), rather than the doc's single generic
//!   `conserve_into(acc, leg)` — the same choice already made for
//!   `exact_dust`'s `money_dust_split`/`qty_dust_split`.
//! - **[`verify`] nets each asset separately**, where `exact_audit` summed the
//!   whole log into one total: an [`Entry`] carries an `asset`, and a
//!   [`Report`] one net per asset. One total let a leak in one asset cancel a
//!   forgery in another.
//! - **No `qty_sum_assert_zero`.** The doc names one beside
//!   [`money_sum_assert_zero`], but every `Quantity` leg is non-negative, so
//!   its sum is zero only when every leg is — it could never check a transfer,
//!   whose giving side is negative. A quantity's movements are audited through
//!   [`verify`] with an asset key instead.

use exact_kind::{ KindError, Money, Quantity };
use std::collections::BTreeMap;
use std::borrow::Borrow;

/// One posting in a transaction log.
///
/// Plain data, constructed by anyone, carrying no invariant of its own. The
/// signed amount is a count of minor units of `asset`, at whatever scale the
/// log's producer and consumer have agreed on; [`verify`] never interprets the
/// scale, because conservation is a property of the integers and holds at
/// every scale.
///
/// Both keys are the caller's own types. [`verify`] reads only `asset`, so the
/// account type carries no bound at all: an id, a name, or a reference into
/// the caller's own records.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct Entry< K, A >
{
  /// The account the posting is against. Carried for reporting, never for
  /// arithmetic — see the module docs on why balances are not totalled.
  pub account : K,
  /// What moved — a currency or an instrument. [`verify`] nets each asset
  /// separately: amounts of different assets are never added together.
  pub asset : A,
  /// Signed minor units: positive credits the account, negative debits it.
  pub amount_minor : i64,
}

impl< K, A > Entry< K, A >
{
  /// Build a posting.
  #[ must_use ]
  pub fn new( account : K, asset : A, amount_minor : i64 ) -> Self
  {
    Self { account, asset, amount_minor }
  }
}

/// Why a conservation check could not be completed, or found a discrepancy.
#[ derive( Debug, Clone, Copy, PartialEq, Eq ) ]
pub enum ConservationError
{
  /// A typed slice's sum was not exactly zero.
  NotZero
  {
    /// The actual signed sum, in minor units.
    got : i128,
  },
  /// The running total left the representable range.
  Overflow,
}

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

impl core::error::Error for ConservationError {}

fn kind_error_to_conservation_error( _e : KindError ) -> ConservationError
{
  ConservationError::Overflow
}

/// The outcome of auditing a log.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct Report< A >
{
  /// How many postings were folded.
  pub entries : usize,
  /// Each asset's signed sum, in minor units, keyed by asset — a `BTreeMap`,
  /// so a report lists its assets in the same order every time. Every sum
  /// zero is a balanced log.
  pub nets : BTreeMap< A, i128 >,
}

impl< A > Report< A >
{
  /// Whether the log conserves value — every asset's sum is zero.
  ///
  /// Exact equality with zero, with no tolerance window. A tolerance is how
  /// an auditor comes to pass the only errors small enough to be worth
  /// hiding: an off-by-one-unit leak repeated across a million transactions
  /// is the failure mode this whole crate exists to make impossible, and it
  /// is invisible to any check that ignores single units.
  #[ must_use ]
  pub fn is_balanced( &self ) -> bool
  {
    self.nets.values().all( | net | *net == 0 )
  }

  /// One asset's discrepancy, in minor units — `Some( 0 )` when it balances,
  /// and `None` when the log never moved it, so a misspelt asset cannot read
  /// as balanced.
  ///
  /// Signed on purpose: the sign says whether value appeared or vanished,
  /// and those are different investigations.
  #[ must_use ]
  pub fn discrepancy_minor< Q >( &self, asset : &Q ) -> Option< i128 >
  where
    A : Borrow< Q > + Ord,
    Q : Ord + ?Sized,
  {
    self.nets.get( asset ).copied()
  }
}

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

/// Audit a log for conservation, netting each asset separately.
///
/// ```
/// use exact_conserve::{ Entry, verify };
///
/// let log = [ Entry::new( "hold", "cash", 1_000_000 ), Entry::new( "ship", "cash", -1_000_000 ) ];
/// assert!( verify( &log ).unwrap().is_balanced() );
///
/// let leaky = [ Entry::new( "hold", "cash", 1_000_000 ), Entry::new( "ship", "cash", -999_999 ) ];
/// assert_eq!( verify( &leaky ).unwrap().discrepancy_minor( "cash" ), Some( 1 ) );
/// ```
///
/// # Errors
///
/// [`ConservationError::Overflow`] if an asset's running total leaves `i128`.
pub fn verify< K, A : Ord + Clone >( entries : &[ Entry< K, A > ] ) -> Result< Report< A >, ConservationError >
{
  let mut nets : BTreeMap< A, i128 > = BTreeMap::new();
  for entry in entries
  {
    let amount = i128::from( entry.amount_minor );
    match nets.get_mut( &entry.asset )
    {
      Some( net ) => *net = net.checked_add( amount ).ok_or( ConservationError::Overflow )?,
      None => { nets.insert( entry.asset.clone(), amount ); }
    }
  }
  Ok( Report { entries : entries.len(), nets } )
}

/// Fold one more money leg into a running total, via `exact_add`'s own
/// checked arithmetic. Suitable for [`Iterator::try_fold`].
///
/// # Errors
///
/// [`ConservationError::Overflow`] on overflow or ceiling breach.
pub fn money_conserve_into( acc : Money, leg : Money ) -> Result< Money, ConservationError >
{
  exact_add::money_add( acc, leg ).map_err( kind_error_to_conservation_error )
}

/// Fold one more quantity leg into a running total, via `exact_add`'s own
/// checked arithmetic. Suitable for [`Iterator::try_fold`].
///
/// # Errors
///
/// [`ConservationError::Overflow`] on overflow or ceiling breach.
pub fn qty_conserve_into( acc : Quantity, leg : Quantity ) -> Result< Quantity, ConservationError >
{
  exact_add::qty_add( acc, leg ).map_err( kind_error_to_conservation_error )
}

/// Assert a slice of money legs sums to exactly zero.
///
/// # Errors
///
/// [`ConservationError::NotZero`] when the sum is not zero.
/// [`ConservationError::Overflow`] if the running total leaves `i128`.
pub fn money_sum_assert_zero( legs : &[ Money ] ) -> Result< (), ConservationError >
{
  let mut net : i128 = 0;
  for leg in legs
  {
    net = net.checked_add( i128::from( leg.minor() ) ).ok_or( ConservationError::Overflow )?;
  }
  if net == 0
  {
    Ok( () )
  }
  else
  {
    Err( ConservationError::NotZero { got : net } )
  }
}
