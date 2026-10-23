// The `src/include` mirrors whose unit is an empty shape had no compiler: no translation unit
// of the frozen configuration includes `src/sfnt.h` or `src/fontset.h` (this target's platform
// object variables are empty - `docs/baseline/frozen-build-sources.txt`, docs/status.md 3.31),
// and `xtask layout-check`'s single-crate probe compiles them only as part of a crate whose
// foundation list happens to put `lisp.rs` first.  This probe is the consumer those two mirrors
// would otherwise not have: it supplies the names the two C headers take from `lisp.h` and
// `<setjmp.h>` and includes both into one flat namespace, which is what a translation unit
// taking them at once is.  Same shape as `win_inc_probe.rs` (docs/status.md 3.159) and
// `f_inc_probe.rs` (docs/status.md 3.102), for the same reason: `layout-check` skips the
// platform-exclusive mirrors by data, and these two are the ones the *hash-tag* exclusion does
// not cover.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         clashing_extern_declarations, improper_ctypes_definitions, suspicious_runtime_symbol_definitions)]
// Every `c_*` name the graph needs (`c_long`/`c_ulong` come with `lib/include/limits.in.rs`,
// which `lib/include/stdint.in.rs` includes at C 77).
use std::ffi::*;

/// `lisp.h`'s tagged value.  `src/fontset.h` names it (its ten prototypes take `Lisp_Object`
/// by value) and `src/sfnt.h` reaches `lisp.h` only for `jmp_buf`; neither includes `lisp.h`
/// itself - C's translation units reach it through their own includes (PORTING.md 0.1-9),
/// which is what this stub stands in for.  It is one machine word, the frozen configuration's
/// `Lisp_Object`, and the probe stubs it instead of pulling `include/lisp.rs` in (the
/// `win_inc_probe.rs` reading).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_Object { pub word: usize }

/// `<setjmp.h>`'s `jmp_buf` on this target: 192 bytes.  This is the same opaque spelling
/// `include/lisp.rs` carries (lisp.rs 1058, from the frozen layout baseline: `sizeof jmp_buf`
/// 192) - C's type is `int[48]`, so its alignment is 4 where the byte array's is 1; the one
/// member that embeds it (`sfnt_interpreter.trap`, sfnt.h 2015) keeps C's offsets either way
/// (the mirror's own assertion records the measurement).
pub type jmp_buf = [u8; 192];

include!("../src/include/sfnt.rs");
include!("../src/include/fontset.rs");

// The probe is a compile proof, not a program: every item of both mirrors is checked whether
// or not anything uses it (the ~150 `sfnt_*` entry points are declared and never defined here,
// exactly as the header declares them), and no configuration compiles either mirror on this
// target.  There is no C side to compare against, because no configuration compiles a
// `src/sfnt.c`/`src/fontset.c` object on this target either.
fn main () {}
