// The compiler for `femacs/src/include/android-asset.rs`, the port's emulation of the Android
// asset-manager API (emacs/src/android-asset.h).
//
// The mirror has no unit of its own on the frozen target: `android.c` is the only file that
// includes it, and that unit is an empty shell here (docs/ports/android.md).  `xtask layout-check`
// does not reach it either - its single crate compiles every `src/include` mirror together and
// this one pulls in the NDK's `<jni.h>` mirror (`include/jni.rs`) beside a libc surface, so the
// file gets its own compiler the way the platform mirror sets get
// `spikes/{haiku,android}_inc_probe.rs` (docs/status.md 3.102).  One flat namespace holding what
// that unit sees, with the `include!` graph materialised by `xtask probe` the way a unit builds it.
//
// Evidence:
//
//     cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir-asset cargo run -q -p xtask -- \
//       probe ../spikes/android_asset_probe.rs ../build/probes/android_asset_probe.rmeta \
//       --crate-type=lib --emit=metadata
//
// What a translation unit supplies itself (PORTING.md 0.1-9): `config.rs` is the generated frozen
// configuration and `lisp.rs` the lisp.h mirror, which carries the `c_int`/`c_void`/... aliases
// and the macros the other mirrors expand.  The file's own `include!`s supply the JNI ABI and the
// libc entries it names.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         clashing_extern_declarations)]
include!("../femacs/src/include/config.rs");
include!("../femacs/src/include/lisp.rs");
include!("../femacs/src/include/android-asset.rs");

fn main () {}
