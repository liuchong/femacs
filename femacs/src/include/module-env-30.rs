// femacs: port of src/module-env-30.h @ fd7afced403.
//
// Substituted-fragment mirror (PORTING.md 0.1, P-26).  This file is not a header in the usual
// sense: `configure` substitutes its **whole body** for the `@module_env_snippet_30@` placeholder
// in `src/emacs-module.in.h`'s `struct emacs_env_30` (C 146-235), so no translation unit
// `#include`s it - the only mentions of the name outside the fragments are the comments in
// `src/emacs-module.c` 23-58 that describe the release procedure:
//
//     $ rg -n 'module-env-' emacs/src | grep -v '^emacs/src/module-env'
//     emacs/src/emacs-module.c:23:the version-specific environment fragments in module-env-*.h.
//     emacs/src/emacs-module.c:41:  module-env-VER.h.  Add functions solely at the end of the fragment
//     emacs/src/emacs-module.c:44:  module-env-27.h.
//     emacs/src/emacs-module.c:58:1. Add a new function pointer field at the end of the module-env-*.h
//
// The judge is the header those substitutions produce,
// `build/emacs-ref/src/emacs-module.h`: snippet 30's text is inlined in its `struct emacs_env_30`
// (judge line 930) and in every later `struct emacs_env_NN` (the last one, `struct emacs_env_32`,
// is the judge's line 1291).
//
// **Its members cannot be declared in this file.**  Rust pastes an `include!` where a macro or an
// item may appear, and a struct's member list is not such a place - measured with the pinned
// toolchain (`error: unexpected token: `!` … note: macros cannot expand to struct fields`, quoted
// in `include/emacs-module.in.rs`'s note).  So this mirror is the fragment's *text*, and the Rust
// spellings of the members live in `include/emacs-module.in.rs`, in the `#[repr(C)]` records the
// placeholder would fill, in C's member order.
//
// **This fragment is empty in C (0 bytes).**  The version-30 API added no environment member,
// which the judge confirms: `struct emacs_env_29`, `_30` and `_31` are the same record as
// `struct emacs_env_28` (measured - all four are 320 bytes with align 8, and their member lists are
// identical).  There is nothing to carry, so the mirror records the fact rather than inventing an
// item - the disposition-record shape `lib/assert.in.h` established for a file with no entries
// (status 3.157).
