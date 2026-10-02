// Regression guard for PORTING.md P-39: a unit that *defines* a name lisp.h declares
// (alloc.c's `staticpro`, marker.c's `marker_position`, casetab.c's
// `Vascii_downcase_table`, …) must shadow lisp.rs's declaration instead of colliding with
// it (E0428).  The mirrors are included at the **crate root**, exactly as a translation
// unit includes them, because that is what makes the declarations and the definitions
// share one namespace: wrapping lisp.rs in a `mod` (as this probe used to) hides a
// declaration that leaked to lisp.rs's top level, which is the very regression the probe
// exists for (review B's B-4).
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]
include!("../femacs/src/include/config.rs");
include!("../femacs/lib/include/stdio.rs");
include!("../femacs/src/include/lisp.rs");
include!("../femacs/src/include/itree.rs");
include!("../femacs/src/include/character.rs");
include!("../femacs/src/include/buffer.rs");
#[unsafe(no_mangle)]
pub static mut Vascii_downcase_table: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub unsafe extern "C" fn marker_position (_marker: Lisp_Object) -> ptrdiff_t { 0 }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn staticpro (_var: *const Lisp_Object) {}
// The decisive discriminator (review C's F5): lisp.rs declares `xmalloc (_n: usize)`, so
// this **0-argument** local definition can only coexist with it if the local item is the
// one that resolves — the coercion below would be a type error against the import.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn xmalloc () -> *mut c_void { std::ptr::null_mut () }
fn main () {
  unsafe {
    let local_xmalloc: unsafe extern "C" fn () -> *mut c_void = xmalloc;
    println! ("shadow-probe {} {} {}", Vascii_downcase_table == lisp_h_Qnil (),
              marker_position (Qnil ()), local_xmalloc ().is_null ());
  }
}
