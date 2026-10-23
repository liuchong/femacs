// The window-system mirror's compiler, and the only place `dispextern.rs`'s `#[cfg (feature =
// "have_window_system")]` text is ever type-checked.
//
// `dispextern.h`'s `struct glyph_string` (C 1362-1507, inside C 1358's `#ifdef HAVE_WINDOW_SYSTEM`)
// is dead text in the frozen configuration, so `xtask layout-check` strips it and nothing else
// compiles it either — `layout-check` compiles every `src/include` mirror in one crate, but with
// every feature off, exactly as the frozen build does.  This probe turns the feature *on* and
// supplies the names the record takes from elsewhere, which is what turns "the tokens are there"
// into "rustc resolved the field names and types".
//
// **It must be compiled with `--cfg 'feature="have_window_system"'`** (the feature name is the C
// macro `HAVE_WINDOW_SYSTEM` lower-cased, the tree's convention; the whole namespace is declared in
// `xtask/Cargo.toml`'s `[features]` table, all off by default).  `xtask probe` passes extra
// rustc arguments through, so the invocation is
//
//     cd femacs && cargo run -q -p xtask -- probe ../spikes/window_system_probe.rs \
//         ../build/probes/window_system_probe.rmeta --crate-type=lib --emit=metadata \
//         --cfg 'feature="have_window_system"'
//
// The probe is not registered in `xtask`'s `PROBES` array: that array compiles its entries with no
// extra `--cfg`, so this one needs its own list (the integrator's call, status 3.167).
//
// What it stands in for (stubs, the `win_inc_probe.rs` shape): the four owner *records* the record's
// pointers name — `frame` (`include/frame.rs`), `window` (`include/window.rs`), `font`
// (`include/font.rs`) and `composition` (`include/composite.rs`) — and the names `dispextern.rs`'s
// *live* text takes from `lisp.h`/`character.h`.  The record's other pointees (`face`, `image`,
// `xwidget`) and the platform `gc` types have no mirror yet, so the mirror degrades them to
// `*mut c_void`; nothing here needs a stub for them.
//
// Negative control (run once, against a copy of the mirror — the tree is never mutated): renaming
// `pub f: *mut frame` to `*mut frame_typo` in a `/tmp` copy of `src/include/dispextern.rs`
// and pointing this probe at the copy makes rustc report
// `error[E0425]: cannot find type `frame_typo` in this scope`, i.e. the probe really resolves the
// record's field types rather than skipping a `cfg`'d-out item.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         clashing_extern_declarations, improper_ctypes_definitions, unused_doc_comments)]
use std::ffi::*;

/// `lisp.h`'s tagged value (one machine word on this target).  `dispextern.rs`'s live text uses it
/// in the glyph-code predicates and in several records.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_Object { pub word: usize }

/// `lisp.h`'s `ptrdiff_t` alias.
pub type ptrdiff_t = isize;

/// The four owner records `struct glyph_string` points at, as opaque stubs: the probe exists to
/// compile the *record*, and what it says about them is only "a pointer to one".
#[repr(C)]
#[derive(Clone, Copy)]
pub struct frame { pub p: usize }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct window { pub p: usize }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct font { pub p: usize }
#[repr(C)]
#[derive(Clone, Copy)]
pub struct composition { pub p: usize }

// The rest of what the live text of the mirror reaches, spelled as `lisp.rs`/`character.rs` spell it
// (signatures only; no body here is meaningful).
pub type EMACS_INT = i64;
pub type intmax_t = i64;
pub const CHARACTERBITS: usize = 22;
pub const MAX_CHAR: EMACS_INT = 0x3FFFFF;
pub const EMACS_INT_MAX: EMACS_INT = i64::MAX;
pub fn CONSP (_x: Lisp_Object) -> bool { false }
pub fn XCAR (x: Lisp_Object) -> Lisp_Object { x }
pub fn XCDR (x: Lisp_Object) -> Lisp_Object { x }
pub fn XFIXNUM (_x: Lisp_Object) -> EMACS_INT { 0 }
pub fn RANGED_FIXNUMP (_lo: EMACS_INT, _x: Lisp_Object, _hi: EMACS_INT) -> bool { false }
pub fn CHARACTERP (_x: Lisp_Object) -> bool { false }
pub fn CHAR_VALID_P (_c: intmax_t) -> bool { false }

include!("../src/include/dispextern.rs");

// The probe is a compile proof, not a program: it touches the record and its bit accessors so both
// the struct and `glyph_string_bits!` are instantiated, and nothing else.
fn main ()
{
  let s: *mut glyph_string = std::ptr::null_mut ();
  if !s.is_null ()
    {
      unsafe {
        glyph_string_bits! (padding_p, *s, true);
        glyph_string_bits! (for_overlaps, *s, 1);
      }
    }
  let _ = std::mem::size_of::<glyph_string> ();
}
