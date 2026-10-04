// The Android platform mirrors' compiler: `femacs/src/include/androidgui.rs` and
// `femacs/src/include/android.rs`.
//
// `xtask layout-check` compiles every `src/include` mirror into ONE crate, which cannot hold the
// platform-exclusive mirrors: `androidgui.h` and `haikugui.h` declare the same X-compat block and
// the same `XCharStruct`/`Emacs_Pixmap`/`Emacs_Window`/`Emacs_Cursor`/`Drawable` aliases, and C
// includes only one of them behind its platform `#if` (`HAVE_ANDROID`).  This probe is the
// Android side's compiler, the shape `spikes/f_inc_probe.rs` gave `femacs/nt/inc`
// (docs/status.md 3.102): one flat namespace holding what an Android translation unit sees.
//
// `android.h` includes `<jni.h>` and `<android/bitmap.h>` (the NDK's headers, no mirror) and
// `"androidgui.h"`/`"lisp.h"`, so `android.rs` pulls `androidgui.rs`, `lisp.rs` and the lib
// mirrors in itself; this probe adds only `config.rs` (the generated frozen configuration every
// unit includes first) and no `use std::ffi` of its own - `lisp.rs` carries that import, which is
// how a unit gets the `c_int`/`c_char`/... aliases (PORTING.md 0.1-9).
//
// Evidence:
//
//     cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-probe cargo run -q -p xtask -- \
//       probe ../spikes/android_inc_probe.rs ../build/probes/android_inc_probe.rmeta \
//       --crate-type=lib --emit=metadata
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         clashing_extern_declarations)]
include!("../femacs/src/include/config.rs");
include!("../femacs/src/include/android.rs");

fn main () {}
