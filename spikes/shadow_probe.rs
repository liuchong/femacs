// Regression guard for PORTING.md P-39: a unit that *defines* a name lisp.h declares
// (alloc.c's `staticpro`, marker.c's `marker_position`, casetab.c's
// `Vascii_downcase_table`, …) must shadow lisp.rs's declaration instead of colliding
// with it (E0428).  The declarations live in `other_units` and are reached through a
// glob import, so a local definition shadows them — the shape P-21/P-28 use.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]
mod m {
    include!("../femacs/src/include/config.rs");
    include!("../femacs/src/include/lisp.rs");
    include!("../femacs/src/include/itree.rs");
    include!("../femacs/src/include/character.rs");
    include!("../femacs/src/include/buffer.rs");
}
use m::*;
#[unsafe(no_mangle)]
pub static mut Vascii_downcase_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn marker_position (_marker: Lisp_Object) -> ptrdiff_t { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn staticpro (_var: *const Lisp_Object) {}
fn main () {
  unsafe {
    println! ("shadow-probe {} {}", Vascii_downcase_table == lisp_h_Qnil (),
              marker_position (Qnil ()));
  }
}
