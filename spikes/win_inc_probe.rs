// The `src/include` mirrors whose unit is an empty shape had no compiler: no translation unit of
// the frozen configuration includes `src/w32inevt.h` or `src/w32select.h` (this target's platform
// object variables are empty - `docs/baseline/frozen-build-sources.txt`, docs/status.md 3.31), and
// `xtask layout-check` compiles the structural mirrors `tools/gen-layout-fields.py` lists, which
// neither of these is.  `tools/extract-abi.py --check` compares text.  This probe is the consumer
// those mirrors never had: it supplies the names their C headers take from the system and includes
// both into one flat namespace, which is what a Windows translation unit taking them at once is.
// Same shape as `f_inc_probe.rs` (docs/status.md 3.102), for the same reason.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, clashing_extern_declarations, unused_doc_comments)]
use std::ffi::{c_int, c_void};

/// `lisp.h`'s tagged value.  Both headers name it (`src/w32inevt.h` 27-29 and `src/w32select.h`
/// 24-26 take pointers to it), and neither includes `lisp.h` itself - C's translation units reach
/// it through their own includes (PORTING.md 0.1-9), which is what this stub stands in for.  It is
/// one machine word, the frozen configuration's `Lisp_Object` (docs/baseline/config), and the
/// probe stubs it instead of pulling `include/lisp.rs` in: the probe exists to compile these two
/// headers, and what they say about the type is only "a pointer to it" (P-72's reading).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Lisp_Object { pub word: usize }

include!("../femacs/src/include/w32inevt.rs");
include!("../femacs/src/include/w32select.rs");

// The probe is a compile proof, not a program: every item of both mirrors is checked whether or
// not anything uses it (`w32select.rs`'s three entry points are declared and never defined here,
// exactly as the header declares them), and no configuration compiles either mirror on this
// target.  There is no C side to compare against, because no configuration compiles a `src/w32*`
// header on this target either.
fn main () {}
