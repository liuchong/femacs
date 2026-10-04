// The NS platform mirrors' compiler: `nsgui.rs` and `nsxwidget.rs`.
//
// `xtask layout-check` compiles every `src/include` mirror into ONE crate, which is the right
// model for the portable headers but cannot hold the platform-exclusive ones: the frozen
// configuration (darwin-arm64) has no `HAVE_NS` and no `HAVE_XWIDGETS`, so no translation unit
// of this target includes `nsgui.h` or `nsxwidget.h` (`docs/baseline/frozen-build-sources.txt`
// lists no `.m`), and `nsgui.h`'s own live text re-declares names `dispextern.h` also carries
// outside its X branches - `Emacs_Cursor` (dispextern.h 175, `include/dispextern.rs` 62), and
// the whole X-compat block `haikugui.rs`/`androidgui.rs` declare for their platforms
// (`XCharStruct`, `Emacs_Pixmap`, `Window`, `NativeRectangle`, the gravity/value/position
// constants).  C keeps those apart with its platform `#if`s and by compiling one backend at a
// time; a single crate has one definition point per name, so each platform's mirrors get their
// own probe the way the `femacs/nt/inc` mirrors have `spikes/f_inc_probe.rs` (docs/status.md
// 3.102/3.147).  `tools/extract-abi.py --check` only compares text, so without this probe
// nothing would compile either file.
//
// What the two mirrors name from outside themselves is what a Cocoa translation unit supplies
// (PORTING.md 0.1-9): the `c_*` aliases, and `Lisp_Object` - `nsxwidget.h` 29 includes
// `lisp.h`, and the port's lisp.h mirror is the one header every unit reaches through its own
// includes.  The probe stubs it the way `win_inc_probe.rs` does: one machine word, the frozen
// configuration's `Lisp_Object` (LISP_WORDS_ARE_POINTERS, `docs/baseline/config`), because
// what the header's prototypes say about it is only "by value" or "a return value".
//
// The probe is a compile proof, not a program.  Evidence:
//
//     cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-ns cargo run -q -p xtask -- \
//       probe ../spikes/ns_inc_probe.rs /tmp/probes-ns/ns.rmeta
//
// (`xtask probe` materialises the `include!` graph the way the units are built - each file
// once, C's include-guard order - and then runs rustc, which resolves the pinned toolchain
// from femacs/rust-toolchain.toml.  The probe is not in `xtask`'s `PROBES` list yet; the
// integrator decides that, and whether the two mirrors join
// `docs/baseline/layout-excluded-mirrors.txt`, which is data this lane may not edit.)
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         clashing_extern_declarations)]
use std::ffi::{c_char, c_int, c_long, c_void};

/// `lisp.h`'s tagged value.  `src/nsxwidget.h` 29 includes `lisp.h` and every prototype that
/// mentions the type takes or returns it by value; C's translation units reach it through
/// their own includes (PORTING.md 0.1-9), which is what this stub stands in for.  It is one
/// machine word (the frozen configuration's `Lisp_Object`, `docs/baseline/config`), and the
/// probe stubs it instead of pulling `include/lisp.rs` in: the probe exists to compile the two
/// headers, and what they say about the type is only "returned by value" (P-72's reading).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_Object { pub word: usize }

include!("../femacs/src/include/nsgui.rs");
include!("../femacs/src/include/nsxwidget.rs");

// The probe is a compile proof, not a program: every item of both mirrors is checked whether
// or not anything uses it (`nsxwidget.rs`'s 21 entry points are declared and never defined
// here, exactly as the header declares them), and no configuration compiles either mirror on
// this target.  There is no C side to compare against, because no configuration compiles an
// `ns*` header on this target either.
fn main () {}
