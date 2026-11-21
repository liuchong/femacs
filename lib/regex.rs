// femacs: port of lib/regex.c @ fd7afced403.
//
// This is the aggregate translation unit: C 57-59 `#include`s `regex_internal.c`, `regcomp.c`
// and `regexec.c`, which is why the reference has one object (`build/emacs-ref/lib/regex.o`)
// and not three.  The port keeps the three as their own units (`lib/regex_internal.rs`,
// `lib/regcomp.rs`, `lib/regexec.rs`) - PORTING.md 0.1's one `.c` = one `.rs`
// rule applies to the three files C actually writes - and the union of their symbols is what
// regex.o carries (docs/ports/regex.md, docs/ports/regcomp.md and docs/ports/regexec.md record
// the per-unit shape; this file's own text is the two macros below plus the dead `_LIBC` arm).
//
// What is left of *this* file's text after the frozen configuration's `#if`s is C 20's
// `#define __STDC_WANT_IEC_60559_BFP_EXT__`, C 23's `#define _GL_USE_STDLIB_ALLOC 1`, the
// includes and C 75-82's `#if _LIBC` block (`re_max_failures`), which is dead: `_LIBC` is
// undefined.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn)]
// [U1-MODULE-UNIT]
use crate::common::*;

use std::ffi::c_int;

// C 20: `#define __STDC_WANT_IEC_60559_BFP_EXT__` (an empty body: the feature-test macro).
pub const __STDC_WANT_IEC_60559_BFP_EXT__: bool = true;

// C 23: `# define _GL_USE_STDLIB_ALLOC 1` (the live arm; `_LIBC` is undefined).
pub const _GL_USE_STDLIB_ALLOC: c_int = 1;

// C 57-59's three `#include`s: the aggregate's text, owned by the sibling units (see above).
