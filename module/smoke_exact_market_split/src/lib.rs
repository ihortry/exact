//! Smoke lane `smoke_exact_market_split` — this family's slice, run end to
//! end with a control arm, through `exact_arith` — the 14-crate facade —
//! alone.
//!
//! Six steps in one process: a [`Money`] parsed and added by a fixed-point
//! decimal type, a [`Quantity`] refusing to go below zero, a log audited by a
//! conservation auditor, a market fill split among several accounts with the
//! dust accounted for and the split proven to conserve, the ten scenes of
//! `docs/scene/` printed in their golden-print shape, and every one of
//! those names imported from `exact_arith` and from nowhere else — which is
//! what makes the facade's completeness a thing this lane tests rather than
//! a thing its documentation claims.
//!
//! Ported from `smoke_exact_arithmetic`, this family's demo lane before the
//! 15-crate migration — steps 1 through 4 carry its exact content forward
//! unchanged; step 5 is new, added because a lane named for a market split
//! ought to run one, exercising `exact_dust` and `exact_conserve` together in
//! a way neither crate's own unit tests do on their own; step 6 runs the
//! proposed lane's scenes ([`golden`], [`checksum`]).
//!
//! # The control arm
//!
//! A lane that only runs the exact path proves the exact path does not
//! crash. It does not prove the path is exact, because a lane built on
//! `f64` would print the same cheerful verdict for nine of these ten steps.
//!
//! So every claim here is made twice — once through the exact types and once
//! through `f64` — and the lane **asserts that the two disagree**. If a
//! future change made the exact path inexact, the arms would agree, the
//! assertion would fail, and the lane would go red. The control arm's job is
//! to be wrong; a lane where it stops being wrong has stopped discriminating,
//! and reporting that as a pass would be worse than reporting nothing.
//!
//! `f64` appears in this file and in no other library source of the family —
//! only `exact_arith`'s timing bench, a test, also uses it.
//!
//! # Why the lane is a library and not `src/main.rs`
//!
//! No test suite can execute a bare `src/main.rs` to raise its coverage, and
//! bounding what may live in that file keeps a coverage exclusion from
//! quietly becoming somewhere to keep logic. `src/main.rs` is the
//! argument-free entry point and nothing else, and `tests/lane_test.rs`
//! drives what moved.

use exact_arith::
{
  money_dust_remainder, money_dust_split, money_from_wire, money_to_wire, minor_checked_add, minor_from_i64,
  price_snap_tick, qty_snap_lot, verify,
  Decimal, DustTo, Entry, KindError, Lot, Money, Price, Quantity, Rounding, Tick,
};

/// How many times the tenth is added, in both arms.
pub const REPEATS : i64 = 10;

/// The exact arm: ten tenths, added in the exact decimal type.
///
/// # Panics
///
/// Panics if `0.1` is not representable at the type's scale, or if ten
/// tenths overflow — either of which is the exact path failing the lane
/// rather than a condition worth handing back.
#[ must_use ]
pub fn exact_tenths() -> Money
{
  let tenth = Money::parse( "0.1" ).expect( "0.1 is representable at scale 6" );
  let mut total = Money::ZERO;
  for _ in 0..REPEATS
  {
    total = total.checked_add( tenth ).expect( "one whole unit is far below the ceiling" );
  }
  total
}

/// The control arm: the same ten tenths, added in binary floating point.
#[ must_use ]
pub fn float_tenths() -> f64
{
  let mut total = 0.0_f64;
  for _ in 0..REPEATS
  {
    total += 0.1_f64;
  }
  total
}

/// The ledger the audit runs over: a purchase settled in two postings.
///
/// Built from the exact values, then handed over as plain records — the
/// auditor never sees a `Money` or a `Quantity`, only `i64` minor units.
///
/// # Panics
///
/// If `amount.minor() - leak_minor` would overflow `i64`. `amount` is
/// bounded well inside `i64` by the exact decimal type's own ceiling, but
/// `leak_minor` is a raw, unbounded parameter this lane's caller controls;
/// every call in this lane passes `0` or `1`, so the panic is reachable only
/// by a future caller supplying an unrealistic leak, never by `run` itself.
#[ must_use ]
pub fn ledger( amount : Money, leak_minor : i64 ) -> Vec< Entry< &'static str, &'static str > >
{
  // Fix(smoke_exact_arithmetic_ledger_leak_minor_subtraction_overflow): the
  // "seller" posting computed `amount.minor() - leak_minor` with a bare `-`
  // on two `i64`s. `amount.minor()` is bounded well inside `i64` by the
  // exact decimal type's `CEILING_MINOR_UNITS` headroom, but `leak_minor` is
  // a public, unvalidated `i64` parameter with no such bound — a caller
  // passing `leak_minor` near `i64::MIN` drives the subtraction past
  // `i64::MAX`. This lane's own `run` only ever passes `0` or `1`, so the
  // defect never fired in practice, but every other arithmetic operation in
  // this crate family already routes through a `checked_*` method for
  // exactly this reason — this call site was the one exception. Carried
  // forward unchanged from `smoke_exact_arithmetic`; the fix identifier keeps
  // its original name for traceability back to where this was found.
  //
  // Root cause: a raw `-` on two `i64`s where only one operand carries a
  //   range invariant from its own type; the other is an unconstrained
  //   plain function parameter.
  // Pitfall: a value bounded by one type's own ceiling (`Money`/`Backing`)
  //   does not bound an arithmetic expression that mixes it with an
  //   unconstrained plain integer — each operand needs its own check, not
  //   just the one that happens to come from a validated type.
  let seller_minor = amount.minor()
  .checked_sub( leak_minor )
  .expect( "leak_minor is a small demo constant well within i64 range" );

  vec!
  [
    Entry::new( "buyer", "cash", -amount.minor() ),
    Entry::new( "seller", "cash", seller_minor ),
  ]
}

/// Split a market fill among `parts` accounts, folding the dust into the
/// first share, and return the shares.
///
/// # Panics
///
/// If the split cannot complete within the representable range — unreachable
/// for the small demo amounts this lane passes.
#[ must_use ]
pub fn market_split( fill : Money, parts : usize ) -> Vec< Money >
{
  money_dust_split( fill, parts, Rounding::Down, DustTo::First ).expect( "a small demo fill splits within range" )
}

/// The proposed lane's golden values — scenes 001 to 006 and 008 to 009 of
/// `docs/scene/`. Scene 007 has no value: mixing kinds does not compile.
#[ derive( Debug, Clone, PartialEq, Eq ) ]
pub struct Golden
{
  /// Scenes 001-002: `10.00 + 3.33 - 13.33`, at scale 2.
  pub sum : Decimal< 2 >,
  /// Scene 003: `10` split into 3, rounded down, the dust held back.
  pub parts : Vec< Money >,
  /// Scene 003: the held-back dust, in minor units.
  pub dust_minor : i64,
  /// Scene 004: `1.26` snapped to a `0.05` tick.
  pub tick : Price,
  /// Scene 005: `10` snapped to a lot of `3`.
  pub lot : Quantity,
  /// Scene 006: whether `"1.234"` was refused at scale 2.
  pub extra : bool,
  /// Scene 008: whether an add at `i64::MAX` was refused.
  pub overflow : bool,
  /// Scene 009: the minor units of `10` after a wire round trip.
  pub wire_minor : i64,
}

/// Run scenes 001 to 009 and return what each produced.
///
/// Scenes 001, 002 and 006 run at scale 2, as proposed. Scenes 003 and 009
/// run on `Money`, whose scale is fixed at 6, because `exact_dust` and
/// `exact_bytes` take `Money` only.
///
/// # Panics
///
/// If any scene's assertion fails.
#[ must_use ]
pub fn golden() -> Golden
{
  // Scenes 001-002: parse at scale 2, add, subtract the total, land on exactly zero.
  let a = Decimal::< 2 >::parse( "10.00" ).expect( "10.00 fits scale 2" );
  let b = Decimal::< 2 >::parse( "3.33" ).expect( "3.33 fits scale 2" );
  let total = Decimal::< 2 >::parse( "13.33" ).expect( "13.33 fits scale 2" );
  let sum = a.checked_add( b ).and_then( | s | s.checked_sub( total ) ).expect( "small values fit" );
  assert_eq!( sum, Decimal::< 2 >::ZERO, "10.00 + 3.33 - 13.33 must be exactly zero" );

  // Scene 003: split, hold the dust back, and lose nothing.
  let ten = Money::from_int( 10 ).expect( "10 is representable" );
  let parts = money_dust_split( ten, 3, Rounding::Down, DustTo::Sink ).expect( "10 splits within range" );
  let dust_minor = money_dust_remainder( ten, 3, Rounding::Down ).expect( "10 splits within range" );
  let parts_minor : i64 = parts.iter().map( | p | p.minor() ).sum();
  assert_eq!( parts_minor + dust_minor, ten.minor(), "parts plus dust must equal the total" );

  // Scenes 004-005: snap to the grid.
  let tick_size = Tick::new( Price::parse( "0.05" ).expect( "parses" ) ).expect( "a positive tick" );
  let tick = price_snap_tick( Price::parse( "1.26" ).expect( "parses" ), tick_size, Rounding::Down )
  .expect( "1.26 snaps within range" );
  assert_eq!( tick, Price::parse( "1.25" ).expect( "parses" ), "1.26 must snap down to 1.25" );
  let lot_size = Lot::new( Quantity::from_int( 3 ).expect( "3 fits" ) ).expect( "a positive lot" );
  let lot = qty_snap_lot( Quantity::from_int( 10 ).expect( "10 fits" ), lot_size, Rounding::Down )
  .expect( "10 snaps within range" );
  assert_eq!( lot, Quantity::from_int( 9 ).expect( "9 is representable" ), "10 must snap down to 9" );

  // Scene 006: a third decimal digit at scale 2 is refused, never cut off.
  let extra = matches!( Decimal::< 2 >::parse( "1.234" ), Err( KindError::ExcessPrecision { .. } ) );
  assert!( extra, "1.234 must be refused at scale 2" );

  // Scene 008: an add at `i64::MAX` is refused, never wrapped.
  let overflow = minor_checked_add( minor_from_i64( i64::MAX ), minor_from_i64( 1 ) ).is_err();
  assert!( overflow, "i64::MAX + 1 must be refused" );

  // Scene 009: the wire round trip keeps the exact minor units.
  let back = money_from_wire( money_to_wire( ten ) ).expect( "a value just encoded decodes" );
  assert_eq!( back, ten, "the wire round trip must be exact" );

  Golden { sum, parts, dust_minor, tick, lot, extra, overflow, wire_minor : back.minor() }
}

/// Scene 010's checksum: FNV-1a over every minor-unit value in `g`.
#[ must_use ]
pub fn checksum( g : &Golden ) -> u64
{
  let minors = [ g.sum.minor(), g.dust_minor, g.tick.minor(), g.lot.minor(), g.wire_minor ]
  .into_iter()
  .chain( g.parts.iter().map( | p | p.minor() ) )
  .chain( [ i64::from( g.extra ), i64::from( g.overflow ) ] );
  let mut hash : u64 = 0xcbf2_9ce4_8422_2325;
  for byte in minors.flat_map( i64::to_le_bytes )
  {
    hash = ( hash ^ u64::from( byte ) ).wrapping_mul( 0x100_0000_01b3 );
  }
  hash
}

/// Run the six steps, print what each found, and assert every claim.
///
/// # Panics
///
/// Panics on any way the slice can fall short: a parse that does not
/// round-trip, ten exact tenths that are not one, a control arm that has
/// stopped disagreeing, a withdrawal below zero that is permitted, a
/// balanced log reported as leaking, a leak whose signed magnitude is not
/// named, a market split that does not recombine to the original total, a
/// scene whose assertion fails, or two runs whose checksums differ.
pub fn run()
{
  println!( "smoke_exact_market_split — this family's slice, one process" );
  println!();

  step_1_parse_round_trip();
  let exact = step_2_exact_and_control_arms();
  step_3_quantity_refuses_below_zero();
  step_4_audit( exact );
  step_5_market_split();
  step_6_golden_print();

  println!();
  println!( "VERDICT: reached — exact arithmetic holds across the full facade," );
  println!( "         the floating-point control arm does not, and a market split conserves." );
}

/// Step 1: parse and round-trip, through the exact decimal type.
fn step_1_parse_round_trip()
{
  let tenth = Money::parse( "0.1" ).expect( "0.1 parses" );
  assert_eq!( tenth.to_string(), "0.1", "render must be the inverse of parse" );
  println!( "  parse/render   0.1 -> {tenth} (round-trips exactly)" );
}

/// Step 2: the exact arm and the control arm, on the same sum. Returns the
/// exact sum, which step 4's ledger posts.
fn step_2_exact_and_control_arms() -> Money
{
  let exact = exact_tenths();
  let control = float_tenths();
  let expected = Money::parse( "1.0" ).expect( "1.0 parses" );

  assert_eq!( exact, expected, "ten exact tenths must be exactly one" );
  #[ allow( clippy::float_cmp ) ] // Being unequal to 1.0 is the whole assertion.
  {
    assert!
    (
      control != 1.0_f64,
      "the f64 control arm agreed with the exact path, so this lane is no longer \
       discriminating between exact and inexact arithmetic and must not pass",
    );
  }
  println!( "  exact arm      0.1 x {REPEATS} = {exact}" );
  println!( "  control arm    0.1 x {REPEATS} = {control:.17} (f64, wrong by construction)" );
  println!( "  the arms disagree, which is what makes this lane a test" );
  exact
}

/// Step 3: the quantity type refuses to go below zero.
fn step_3_quantity_refuses_below_zero()
{
  let held = Quantity::from_int( 3 ).expect( "3 units is representable" );
  let taken = Quantity::from_int( 5 ).expect( "5 units is representable" );
  let short = held.checked_sub( taken );
  assert!( short.is_err(), "withdrawing more than is held must be refused" );
  println!();
  println!( "  quantity       hold {held}, withdraw {taken} -> {}", short.unwrap_err() );
}

/// Step 4: the auditor over a plain log, balanced and then leaking one minor unit.
fn step_4_audit( exact : Money )
{
  let balanced = verify( &ledger( exact, 0 ) ).expect( "a two-posting log cannot overflow i128" );
  assert!( balanced.is_balanced(), "a log with matching postings must balance" );
  println!();
  println!( "  audit, clean   {balanced}" );

  let leaky = verify( &ledger( exact, 1 ) ).expect( "a two-posting log cannot overflow i128" );
  assert!( !leaky.is_balanced(), "a one-unit leak must be detected" );
  // Negative: the seller was credited one minor unit less than the buyer was
  // debited, so value vanished. The sign is the difference between a leak
  // and a forgery, and an auditor that reported only a magnitude would lose
  // it.
  assert_eq!( leaky.discrepancy_minor( "cash" ), Some( -1 ), "the discrepancy must be named, not just flagged" );
  println!( "  audit, leaky   {leaky}" );
}

/// Step 5: a market fill split among three accounts, dust and all.
fn step_5_market_split()
{
  let fill = Money::parse( "100.000001" ).expect( "parses" );
  let shares = market_split( fill, 3 );
  let recombined = shares.iter().copied().try_fold( Money::ZERO, | a, b | a.checked_add( b ) )
  .expect( "three small shares recombine within range" );
  assert_eq!( recombined, fill, "a market split must conserve the original fill exactly, dust included" );
  println!();
  println!
  (
    "  market split   {fill} into 3 -> [{}, {}, {}] (recombines exactly)",
    shares[ 0 ], shares[ 1 ], shares[ 2 ],
  );
}

/// Step 6: the proposed lane's scenes, in its golden-print shape, with two
/// runs' checksums compared.
fn step_6_golden_print()
{
  let g = golden();
  println!();
  println!( "  sum={}", g.sum );
  let dust = Money::from_minor( g.dust_minor ).expect( "the dust is below one share" );
  println!( "  parts={},{},{} dust={dust}", g.parts[ 0 ], g.parts[ 1 ], g.parts[ 2 ] );
  println!( "  tick={} lot={}", g.tick, g.lot );
  println!( "  extra={} overflow={}", u8::from( g.extra ), u8::from( g.overflow ) );
  println!( "  wire={}", g.wire_minor );
  let ( a, b ) = ( checksum( &g ), checksum( &golden() ) );
  assert_eq!( a, b, "two runs must produce the same checksum" );
  println!( "  a=0x{a:016x} b=0x{b:016x}" );
  println!( "  ok" );
}
