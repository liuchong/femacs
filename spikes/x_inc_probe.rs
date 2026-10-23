// The X/GTK `src/include` mirrors whose header is live text but whose unit is an empty shape
// had no compiler: no translation unit of the frozen configuration includes `src/xgselect.h`
// (its only definer, `src/xgselect.c`, is wholly inside `#ifdef HAVE_GLIB`, and the frozen
// configuration leaves it undefined - `src/xgselect.rs` is the note that says so), and
// `xtask layout-check` compiles the structural mirrors only.  This probe is the consumer those
// mirrors never had: it supplies the names their C headers take from the system and includes
// them into one flat namespace, which is what an X translation unit taking them at once is.
// Same shape as `f_inc_probe.rs` and `win_inc_probe.rs` (docs/status.md 3.102, 3.159);
// `emacsgtkfixed.h` joins it when its mirror lands.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, clashing_extern_declarations, unused_doc_comments)]
use std::ffi::{c_char, c_int, c_void};

include!("../src/include/xgselect.rs");
include!("../src/include/xsettings.rs");

// The probe is a compile proof, not a program: every declaration of every mirror above is
// checked whether or not anything calls it, and no configuration compiles them on this target.
fn main () {}
