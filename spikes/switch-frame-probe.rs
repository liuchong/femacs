//! `handle_one_xevent`'s switch frame, as a runnable check (status 4.031, Decision 126).
//!
//! Run: `rustc --edition 2024 -O spikes/switch-frame-probe.rs -o /tmp/switch-frame && /tmp/switch-frame`
//!
//! C's `switch (event->type)` closes at C 25588 (`    }`, four spaces inside the function) and
//! the `done:` label stands at C 25590 with **one** space of indent - *outside* the switch.  So
//! a C `break` inside a case reaches `done:` **without** running the `OTHER:` block, while a
//! `goto OTHER` runs that block and then falls into `done:`.  The four slices that landed the
//! frame (111-114) spelled exactly the opposite (`break 'other` at the thirty `goto OTHER` sites,
//! and nothing at the C-`break` sites).  This probe prints both shapes as paths and asserts the
//! repaired one.

/// Event 1 is a site like C 19476: `if (cond) goto OTHER;` in the middle of an arm that ends
/// with C's `break`.  Event 2 is a site like C 19485: an arm that simply ends with `break;`.
fn old (event: u8) -> String
{
  let mut log = String::new ();
  'done: {
    'other: {
      match event
	{
	  1 => { if true { break 'other; } log.push_str ("tail "); }
	  2 => { log.push_str ("body "); }
	  _ => {}
	}
      log.push_str ("OTHER ");
    }
    log.push_str ("done");
  }
  log
}

fn new (event: u8) -> String
{
  let mut log = String::new ();
  'done: {
    'other: {
      'switch: {
	match event
	  {
	    1 => { if true { break 'switch; } log.push_str ("tail "); }
	    2 => { log.push_str ("body "); break 'other; }
	    _ => {}
	  }
      }
      log.push_str ("OTHER ");
    }
    log.push_str ("done");
  }
  log
}

fn main ()
{
  // The repaired shape: C's `goto OTHER` runs the block, C's `break` skips it.
  assert_eq! (new (1), "OTHER done", "goto OTHER must run the OTHER block");
  assert_eq! (new (2), "body done", "C's break must skip the OTHER block");
  // The shape the four earlier slices carried, for the record.
  assert_eq! (old (1), "done");
  assert_eq! (old (2), "body OTHER done");
  println! ("switch-frame-probe: ok (old shape read [{}, {}])", old (1), old (2));
}
