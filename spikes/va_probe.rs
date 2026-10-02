// The Rust side of spikes/va_probe.c: the same three shapes with `...` parameters,
// `VaList::next_arg` (= C's `va_arg`), `VaList::clone` (= C's `va_copy`) and no
// `va_start`/`va_end` (the parameter is the list; dropping it is `va_end`).  Every read
// reads an argument that was actually passed (review D's F7).
#![allow(unsafe_op_in_unsafe_fn, unused_unsafe)]
use core::ffi::{c_char, c_double, c_int, c_long, c_ulong, VaList};

unsafe fn take_va (tag: *const c_char, mut ap: VaList) {
  let i = ap.next_arg::<c_int> ();
  let d = ap.next_arg::<c_double> ();
  let l = ap.next_arg::<c_long> ();
  let ul = ap.next_arg::<c_ulong> ();
  let s = ap.next_arg::<*const c_char> ();
  let name = std::ffi::CStr::from_ptr (tag);
  let text = std::ffi::CStr::from_ptr (s);
  println! ("{} take {} {:.1} {} {} {}", name.to_str ().unwrap (), i, d, l, ul,
            text.to_str ().unwrap ());
}

unsafe fn copy_then_read (tag: *const c_char, mut ap: VaList) {
  let mut c = ap.clone ();                       // va_copy
  let from_copy = c.next_arg::<c_int> ();        // the copy advances on its own
  let from_orig = ap.next_arg::<c_int> ();       // the caller's cursor is untouched
  let more = c.next_arg::<c_double> ();          // and the copy keeps its own position
  let name = std::ffi::CStr::from_ptr (tag);
  println! ("{} copy {} orig {} more {:.1}", name.to_str ().unwrap (), from_copy,
            from_orig, more);
}

unsafe extern "C" fn run (tag: *const c_char, mut ap: ...) {
  take_va (tag, ap.clone ());      // C passes the list by value: an independent cursor
  copy_then_read (tag, ap.clone ());
  let name = std::ffi::CStr::from_ptr (tag);
  println! ("{} after {} {:.1}", name.to_str ().unwrap (), ap.next_arg::<c_int> (),
            ap.next_arg::<c_double> ());
}

fn main () {
  let s = c"str".as_ptr ();
  unsafe {
    run (c"A".as_ptr (), 1 as c_int, 2.5 as c_double, 3 as c_long, 4 as c_ulong, s,
         11 as c_int, 22 as c_int, 33 as c_int);
    run (c"B".as_ptr (), -7 as c_int, -0.5 as c_double, -9 as c_long, 10 as c_ulong, s,
         44 as c_int, 55 as c_int, 66 as c_int);
  }
}
