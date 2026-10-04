// femacs: port of src/module-env-32.h @ fd7afced403.
//
// Substituted-fragment mirror (PORTING.md 0.1, P-26).  This file is not a header in the usual
// sense: `configure` substitutes its **whole body** for the `@module_env_snippet_32@` placeholder
// in `src/emacs-module.in.h`'s `struct emacs_env_32` (C 146-235), so no translation unit
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
// `build/emacs-ref/src/emacs-module.h`: snippet 32's text is inlined in its `struct emacs_env_32`
// (judge line 1291) and in every later `struct emacs_env_NN` (the last one, `struct emacs_env_32`,
// is the judge's line 1291).
//
// **Its members cannot be declared in this file.**  Rust pastes an `include!` where a macro or an
// item may appear, and a struct's member list is not such a place - measured with the pinned
// toolchain (`error: unexpected token: `!` … note: macros cannot expand to struct fields`, quoted
// in `include/emacs-module.in.rs`'s note).  So this mirror is the fragment's *text*, and the Rust
// spellings of the members live in `include/emacs-module.in.rs`, in the `#[repr(C)]` records the
// placeholder would fill, in C's member order.
//
// C's text, verbatim, is below; the parent mirror's `struct emacs_env_32` carries each member's
// Rust spelling in the same order, and the layout assertions there are measured from the judge:
//
// C 1:   /* Add module environment functions newly added in Emacs 32 here.
// C 2:      Before Emacs 32 is released, remove this comment and start
// C 3:      module-env-33.h on master (see admin/release-branch.txt).  */
// C 4: 
// C 5:   /* Get pointer to the pixel buffer of CANVAS.  The buffer is in row
// C 6:      major order and has the size width * height.  The pixel format is
// C 7:      ARGB32 on all platforms.  The pointer will be valid as long as
// C 8:      CANVAS is alive, and as long as its dimensions have not been
// C 9:      changed.  Return NULL in case of error.   */
// C 10: 
// C 11:   uint32_t *(*canvas_data) (emacs_env *env, emacs_value canvas)
// C 12:     EMACS_ATTRIBUTE_NONNULL(1);
