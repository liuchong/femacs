// The Haiku platform mirrors' compiler: `haikugui.rs`, `haiku_support.rs` and `haikuterm.rs`.
//
// `xtask layout-check` compiles every `src/include` mirror into ONE crate, which is the right
// model for the portable headers but cannot hold the platform-exclusive ones: `haikugui.h` and
// `androidgui.h` declare the same X-compat block (`NativeRectangle`, `STORE_NATIVE_RECT`, the
// gravity/value/position constants) and the same `XCharStruct`/`Emacs_Pixmap`/`Emacs_Window`/
// `Emacs_Cursor`/`Drawable` aliases, and C only ever includes one of them behind its platform
// `#if` (`HAVE_HAIKU`).  So the two platform mirror sets get a probe each, the way the
// `femacs/nt/inc` mirrors have `spikes/f_inc_probe.rs` (docs/status.md 3.102): one flat
// namespace holding what a Haiku translation unit sees.
//
// The one name the Haiku mirrors themselves declare twice is `Emacs_Cursor`: on a Haiku build
// `HAVE_WINDOW_SYSTEM` is defined, so it is `haikugui.h`'s typedef and `dispextern.h`'s
// `#ifndef HAVE_WINDOW_SYSTEM` fallback is dead text - C accepts both because the two typedefs
// name the same type, while Rust has one definition point per name.  The probe reads
// `dispextern.rs` through a module and glob-imports it: an explicit item shadows a glob
// (PORTING.md 0.1-10/P-21), which is exactly C's rule here, and it is the module order below
// (before `haikuterm.rs`, whose own `include!` of the mirror then becomes the include-guard
// no-op `xtask`'s materialiser writes) that makes the platform's own arm the surviving one.
// The whole thing is recorded in docs/ports/include-haikuterm.md and include-haikugui.md.
//
// The probe is a compile proof, not a program.  Evidence:
//
//     cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-probe cargo run -q -p xtask -- \
//       probe ../spikes/haiku_inc_probe.rs ../build/probes/haiku_inc_probe.rmeta \
//       --crate-type=lib --emit=metadata
//
// (`xtask probe` materialises the `include!` graph the way the units are built - each file once,
// C's include-guard order - and then runs rustc, which resolves the pinned toolchain from
// femacs/rust-toolchain.toml.)
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs)]

// What a translation unit supplies itself (PORTING.md 0.1-9): `config.rs` is the generated
// frozen configuration and `lisp.rs` the lisp.h mirror, which carries the `c_int`/`c_char`/...
// aliases and the `eassert!`/`eassume!`/`XUNTAG!` macros the other mirrors expand.  The probe
// therefore declares no `use std::ffi` of its own - lisp.rs carries that import, exactly as it
// does for a unit.
include!("../femacs/src/include/config.rs");
include!("../femacs/src/include/lisp.rs");

// The transitive mirrors those two need and that the probe therefore supplies the way a unit
// does: `frame.rs` reads `WINDOWP`/`XWINDOW` (window.h) and `termhooks.rs` names `coding_system`
// (coding.h) - the reference's haikuterm.c includes both headers itself.
include!("../femacs/src/include/itree.rs");
include!("../femacs/src/include/buffer.rs");
include!("../femacs/src/include/window.rs");
include!("../femacs/src/include/composite.rs");
include!("../femacs/src/include/coding.rs");

include!("../femacs/src/include/haikugui.rs");
include!("../femacs/src/include/haiku_support.rs");

mod dispextern_dead_arm {
  use super::*;
  include!("../femacs/src/include/dispextern.rs");
}
use dispextern_dead_arm::*;

include!("../femacs/src/include/haikuterm.rs");

fn main () {}
