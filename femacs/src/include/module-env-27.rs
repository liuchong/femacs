// femacs: port of src/module-env-27.h @ fd7afced403.
//
// Substituted-fragment mirror (PORTING.md 0.1, P-26).  This file is not a header in the usual
// sense: `configure` substitutes its **whole body** for the `@module_env_snippet_27@` placeholder
// in `src/emacs-module.in.h`'s `struct emacs_env_27` (C 146-235), so no translation unit
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
// `build/emacs-ref/src/emacs-module.h`: snippet 27's text is inlined in its `struct emacs_env_27`
// (judge line 415) and in every later `struct emacs_env_NN` (the last one, `struct emacs_env_32`,
// is the judge's line 1291).
//
// **Its members cannot be declared in this file.**  Rust pastes an `include!` where a macro or an
// item may appear, and a struct's member list is not such a place - measured with the pinned
// toolchain (`error: unexpected token: `!` … note: macros cannot expand to struct fields`, quoted
// in `include/emacs-module.in.rs`'s note).  So this mirror is the fragment's *text*, and the Rust
// spellings of the members live in `include/emacs-module.in.rs`, in the `#[repr(C)]` records the
// placeholder would fill, in C's member order.
//
// C's text, verbatim, is below; the parent mirror's `struct emacs_env_27` carries each member's
// Rust spelling in the same order, and the layout assertions there are measured from the judge:
//
// C 1:   /* Processes pending input events and returns whether the module
// C 2:      function should quit.  */
// C 3:   enum emacs_process_input_result (*process_input) (emacs_env *env)
// C 4:     EMACS_ATTRIBUTE_NONNULL (1);
// C 5: 
// C 6:   struct timespec (*extract_time) (emacs_env *env, emacs_value arg)
// C 7:     EMACS_ATTRIBUTE_NONNULL (1);
// C 8: 
// C 9:   emacs_value (*make_time) (emacs_env *env, struct timespec time)
// C 10:     EMACS_ATTRIBUTE_NONNULL (1);
// C 11: 
// C 12:   bool (*extract_big_integer) (emacs_env *env, emacs_value arg, int *sign,
// C 13:                                ptrdiff_t *count, emacs_limb_t *magnitude)
// C 14:     EMACS_ATTRIBUTE_NONNULL (1);
// C 15: 
// C 16:   emacs_value (*make_big_integer) (emacs_env *env, int sign, ptrdiff_t count,
// C 17:                                    const emacs_limb_t *magnitude)
// C 18:     EMACS_ATTRIBUTE_NONNULL (1);
