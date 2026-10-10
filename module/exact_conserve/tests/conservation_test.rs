//! Ported from `exact_audit/tests/conservation_test.rs`'s T07/T08 coverage
//! for the plain-log [`Entry`]/[`verify`] auditor, plus new coverage for the
//! typed `exact_add`-backed convenience layer this crate adds on top.
//!
//! Not ported: `the_manifest_declares_no_dependencies` (T09) — this crate's
//! manifest is deliberately no longer empty; see the disclosed deviation in
//! `src/lib.rs`'s module doc comment. `the_overflow_error_names_where_it_happened`
//! is replaced by `the_overflow_error_names_the_representable_range` below,
//! since `ConservationError::Overflow` carries no `at_entry` field.

use exact_conserve::{ money_conserve_into, money_sum_assert_zero, qty_conserve_into, ConservationError, Entry, Report, verify };
use exact_kind::{ Money, Quantity };

/// The asset the single-asset logs below move.
const CASH : &str = "cash";

/// A transfer of cash, as the two postings it really is.
fn transfer< 'a >( from : &'a str, to : &'a str, amount_minor : i64 ) -> Vec< Entry< &'a str, &'static str > >
{
  vec![ Entry::new( from, CASH, -amount_minor ), Entry::new( to, CASH, amount_minor ) ]
}

/// T07 — a log whose credits match its debits reports balanced.
#[ test ]
fn a_log_of_matched_postings_balances()
{
  let mut log = transfer( "buyer", "seller", 1_250_000 );
  log.extend( transfer( "seller", "carrier", 90_000 ) );
  log.extend( transfer( "carrier", "buyer", 90_000 ) );

  let report = verify( &log ).unwrap();

  assert!( report.is_balanced() );
  assert_eq!( report.discrepancy_minor( CASH ), Some( 0 ) );
  assert_eq!( report.entries, 6 );
}

/// T07 — an empty log balances, and says so without pretending it audited anything.
#[ test ]
fn an_empty_log_balances_at_zero_entries()
{
  let report = verify::< (), &str >( &[] ).unwrap();

  assert!( report.is_balanced() );
  assert_eq!( report.entries, 0 );
}

/// T08 — one minor unit short is detected, and the shortfall is named.
#[ test ]
fn a_single_minor_unit_of_leakage_is_detected_and_named()
{
  let log = [ Entry::new( "buyer", CASH, -1_000_000 ), Entry::new( "seller", CASH, 999_999 ) ];
  let report = verify( &log ).unwrap();

  assert!( !report.is_balanced() );
  assert_eq!( report.discrepancy_minor( CASH ), Some( -1 ) );

  let forged = [ Entry::new( "buyer", CASH, -1_000_000 ), Entry::new( "seller", CASH, 1_000_001 ) ];
  assert_eq!( verify( &forged ).unwrap().discrepancy_minor( CASH ), Some( 1 ) );
}

/// T08 — a single unit stays visible in a log large enough to hide it.
#[ test ]
fn one_unit_stays_visible_against_a_million_units_of_turnover()
{
  let mut log : Vec< Entry< &str, &str > > = ( 0..1_000 ).flat_map( | i | transfer( "a", "b", i64::from( i ) * 1_000 ) ).collect();
  log.push( Entry::new( "leak", CASH, -1 ) );

  let report = verify( &log ).unwrap();

  assert_eq!( report.entries, 2_001 );
  assert_eq!( report.discrepancy_minor( CASH ), Some( -1 ) );
}

/// The report renders the two outcomes distinguishably.
#[ test ]
fn the_report_renders_both_outcomes_in_words()
{
  assert_eq!( verify::< (), &str >( &[] ).unwrap().to_string(), "balanced: entries 0, net 0" );
  assert_eq!
  (
    verify( &[ Entry::new( "x", CASH, -1 ) ] ).unwrap().to_string(),
    "UNBALANCED: entries 1, net cash -1 minor units",
  );
}

/// The accumulator holds a total the posting type could not.
#[ test ]
fn the_accumulator_holds_a_total_the_posting_type_could_not()
{
  let log : Vec< Entry< &str, &str > > = ( 0..4 ).map( | _ | Entry::new( "x", CASH, i64::MAX ) ).collect();

  let total = verify( &log ).unwrap().discrepancy_minor( CASH ).unwrap();

  assert_eq!( total, i128::from( i64::MAX ) * 4 );
  assert!( total > i128::from( u64::MAX ), "the total must exceed what 64 bits can hold" );
}

/// The log's records are plain data anyone can build.
#[ test ]
fn a_log_can_be_built_from_nothing_but_integers()
{
  let log : Vec< Entry< &str, &str > > = [ ( "a", -5_i64 ), ( "b", 5_i64 ) ]
  .into_iter()
  .map( | ( account, amount ) | Entry::new( account, CASH, amount ) )
  .collect();

  let report : Report< &str > = verify( &log ).unwrap();
  assert!( report.is_balanced() );
}

/// A cash forgery and an instrument leak of the same size cancel in one
/// total; netted per asset, each is caught and named. The case a single
/// whole-log sum passed as balanced.
#[ test ]
fn a_leak_in_one_asset_does_not_cancel_a_forgery_in_another()
{
  let log =
  [
    Entry::new( "buyer", CASH, -100 ),
    Entry::new( "seller", CASH, 101 ),
    Entry::new( "seller", "BTC", -5 ),
    Entry::new( "buyer", "BTC", 4 ),
  ];
  let report = verify( &log ).unwrap();

  assert!( !report.is_balanced() );
  assert_eq!( report.discrepancy_minor( CASH ), Some( 1 ) );
  assert_eq!( report.discrepancy_minor( "BTC" ), Some( -1 ) );
}

/// A log moving several assets balances when every asset balances on its own.
#[ test ]
fn a_log_balances_when_every_asset_balances()
{
  let log =
  [
    Entry::new( "buyer", CASH, -100 ),
    Entry::new( "seller", CASH, 100 ),
    Entry::new( "seller", "BTC", -5 ),
    Entry::new( "buyer", "BTC", 5 ),
  ];
  let report = verify( &log ).unwrap();

  assert!( report.is_balanced() );
  assert_eq!( report.nets.len(), 2 );
  assert_eq!( report.to_string(), "balanced: entries 4, net 0" );
}

/// An asset the log never moved is absent from the report, not zero.
#[ test ]
fn an_asset_the_log_never_moved_has_no_discrepancy()
{
  let report = verify( &transfer( "buyer", "seller", 100 ) ).unwrap();

  assert_eq!( report.discrepancy_minor( "BTC" ), None );
}

/// The report names every unbalanced asset, in asset order, and leaves out
/// the ones that balance.
#[ test ]
fn the_report_names_each_unbalanced_asset_in_order()
{
  let log =
  [
    Entry::new( "seller", CASH, 1 ),
    Entry::new( "buyer", "ETH", 7 ),
    Entry::new( "seller", "ETH", -7 ),
    Entry::new( "buyer", "BTC", -1 ),
  ];

  assert_eq!( verify( &log ).unwrap().to_string(), "UNBALANCED: entries 4, net BTC -1, cash 1 minor units" );
}

/// A misspelt asset reads as absent, not as balanced: `cash` is off by one,
/// and asking for `csah` cannot make it look otherwise.
#[ test ]
fn a_misspelt_asset_is_none_not_a_zero_discrepancy()
{
  let report = verify( &[ Entry::new( "a", CASH, 5 ), Entry::new( "b", CASH, -4 ) ] ).unwrap();
  assert_eq!( report.discrepancy_minor( "csah" ), None );
  assert_eq!( report.discrepancy_minor( CASH ), Some( 1 ) );
}

/// A caller keys its log by its own asset type; a misspelt variant does not
/// compile, and copying the key allocates nothing.
#[ test ]
fn a_log_can_be_keyed_by_the_callers_own_asset_type()
{
  #[ derive( Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord ) ]
  enum Asset { Cash, Btc }

  let log =
  [
    Entry::new( "buyer", Asset::Cash, -100 ),
    Entry::new( "seller", Asset::Cash, 101 ),
    Entry::new( "seller", Asset::Btc, -5 ),
    Entry::new( "buyer", Asset::Btc, 5 ),
  ];
  let report = verify( &log ).unwrap();
  assert!( !report.is_balanced() );
  assert_eq!( report.discrepancy_minor( &Asset::Cash ), Some( 1 ) );
  assert_eq!( report.discrepancy_minor( &Asset::Btc ), Some( 0 ) );
}

/// An account key the auditor must accept without asking anything of it: no
/// `Clone`, no `Ord`, no `Debug`, no conversion to a string.
struct Opaque;

/// The account is the caller's own type, and `verify` asks nothing of it — an
/// account key with no traits at all still audits.
#[ test ]
fn an_account_of_any_type_audits()
{
  let log = [ Entry::new( Opaque, CASH, 5 ), Entry::new( Opaque, CASH, -5 ) ];
  assert!( verify( &log ).unwrap().is_balanced() );
}

/// `ConservationError::Overflow` carries no position — it names the
/// condition, not a place to bisect to, unlike `exact_audit`'s own
/// `AccumulatorOverflow { at_entry }`.
#[ test ]
fn the_overflow_error_names_the_representable_range()
{
  assert_eq!( ConservationError::Overflow.to_string(), "the running total left the representable range" );
}

/// `money_conserve_into` folds legs the same way `exact_add::money_add`
/// does, and is usable directly as a `try_fold` step.
#[ test ]
fn money_conserve_into_folds_legs_via_exact_add_and_is_usable_with_try_fold()
{
  let legs = [ Money::from_minor( 500 ).unwrap(), Money::from_minor( -500 ).unwrap(), Money::from_minor( 125 ).unwrap() ];
  let total = legs.iter().copied().try_fold( Money::ZERO, money_conserve_into ).unwrap();
  assert_eq!( total, Money::from_minor( 125 ).unwrap() );
}

/// `money_conserve_into` reports overflow the same way `exact_add::money_add`
/// does, rather than wrapping or panicking.
#[ test ]
fn money_conserve_into_reports_overflow_past_the_declared_ceiling()
{
  assert_eq!( money_conserve_into( Money::MAX, Money::EPSILON ), Err( ConservationError::Overflow ) );
}

/// `qty_conserve_into` folds legs the same way `exact_add::qty_add` does.
#[ test ]
fn qty_conserve_into_folds_legs_via_exact_add()
{
  let legs = [ Quantity::from_minor( 3 ).unwrap(), Quantity::from_minor( 4 ).unwrap() ];
  let total = legs.iter().copied().try_fold( Quantity::ZERO, qty_conserve_into ).unwrap();
  assert_eq!( total, Quantity::from_minor( 7 ).unwrap() );
}

/// `money_sum_assert_zero` passes when a typed slice of legs cancels
/// exactly, and reports the exact signed discrepancy otherwise.
#[ test ]
fn money_sum_assert_zero_passes_when_legs_cancel_and_reports_the_exact_discrepancy_otherwise()
{
  let balanced = [ Money::from_minor( 1_000_000 ).unwrap(), Money::from_minor( -1_000_000 ).unwrap() ];
  assert_eq!( money_sum_assert_zero( &balanced ), Ok( () ) );

  let leaky = [ Money::from_minor( 1_000_000 ).unwrap(), Money::from_minor( -999_999 ).unwrap() ];
  assert_eq!( money_sum_assert_zero( &leaky ), Err( ConservationError::NotZero { got : 1 } ) );
}

/// An empty slice of money legs vacuously sums to zero.
#[ test ]
fn money_sum_assert_zero_passes_on_an_empty_slice()
{
  assert_eq!( money_sum_assert_zero( &[] ), Ok( () ) );
}

/// A slice whose running total passes the ceiling partway, but ends at
/// zero, still conserves — only the final sum is judged, which is why the
/// sum runs in `i128` rather than through `money_add`.
#[ test ]
fn money_sum_assert_zero_judges_the_final_sum_not_the_running_total()
{
  let minus_max = Money::MIN;
  assert_eq!( money_sum_assert_zero( &[ Money::MAX, Money::MAX, minus_max, minus_max ] ), Ok( () ) );
}

/// Folding a quantity past the ceiling is refused, the same as for money.
#[ test ]
fn qty_conserve_into_reports_overflow_past_the_declared_ceiling()
{
  assert_eq!( qty_conserve_into( Quantity::MAX, Quantity::EPSILON ), Err( ConservationError::Overflow ) );
}

/// Credits and debits fold back to exactly zero.
#[ test ]
fn money_conserve_into_folds_credits_and_debits_back_to_zero()
{
  let legs = [ 5, -3, -2 ].map( | m | Money::from_minor( m ).unwrap() );
  assert_eq!( legs.into_iter().try_fold( Money::ZERO, money_conserve_into ), Ok( Money::ZERO ) );
}

/// A failed assertion names the signed discrepancy in its message.
#[ test ]
fn the_not_zero_error_names_the_signed_discrepancy()
{
  let error = ConservationError::NotZero { got : -1 };
  assert_eq!( error.to_string(), "expected a zero sum, got -1 minor units" );
}
