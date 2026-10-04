// femacs: port of src/module-env-25.h @ fd7afced403.
//
// Substituted-fragment mirror (PORTING.md 0.1, P-26).  This file is not a header in the usual
// sense: `configure` substitutes its **whole body** for the `@module_env_snippet_25@` placeholder
// in `src/emacs-module.in.h`'s `struct emacs_env_25` (C 146-235), so no translation unit
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
// `build/emacs-ref/src/emacs-module.h`: snippet 25's text is inlined in its `struct emacs_env_25`
// (judge line 141) and in every later `struct emacs_env_NN` (the last one, `struct emacs_env_32`,
// is the judge's line 1291).
//
// **Its members cannot be declared in this file.**  Rust pastes an `include!` where a macro or an
// item may appear, and a struct's member list is not such a place - measured with the pinned
// toolchain (`error: unexpected token: `!` … note: macros cannot expand to struct fields`, quoted
// in `include/emacs-module.in.rs`'s note).  So this mirror is the fragment's *text*, and the Rust
// spellings of the members live in `include/emacs-module.in.rs`, in the `#[repr(C)]` records the
// placeholder would fill, in C's member order.
//
// C's text, verbatim, is below; the parent mirror's `struct emacs_env_25` carries each member's
// Rust spelling in the same order, and the layout assertions there are measured from the judge:
//
// C 1:   /* Structure size (for version checking).  */
// C 2:   ptrdiff_t size;
// C 3: 
// C 4:   /* Private data; users should not touch this.  */
// C 5:   struct emacs_env_private *private_members;
// C 6: 
// C 7:   /* Memory management.  */
// C 8: 
// C 9:   emacs_value (*make_global_ref) (emacs_env *env, emacs_value value)
// C 10:     EMACS_ATTRIBUTE_NONNULL(1);
// C 11: 
// C 12:   void (*free_global_ref) (emacs_env *env, emacs_value global_value)
// C 13:     EMACS_ATTRIBUTE_NONNULL(1);
// C 14: 
// C 15:   /* Non-local exit handling.  */
// C 16: 
// C 17:   enum emacs_funcall_exit (*non_local_exit_check) (emacs_env *env)
// C 18:     EMACS_ATTRIBUTE_NONNULL(1);
// C 19: 
// C 20:   void (*non_local_exit_clear) (emacs_env *env)
// C 21:     EMACS_ATTRIBUTE_NONNULL(1);
// C 22: 
// C 23:   enum emacs_funcall_exit (*non_local_exit_get)
// C 24:     (emacs_env *env, emacs_value *symbol, emacs_value *data)
// C 25:     EMACS_ATTRIBUTE_NONNULL(1, 2, 3);
// C 26: 
// C 27:   void (*non_local_exit_signal) (emacs_env *env,
// C 28: 				 emacs_value symbol, emacs_value data)
// C 29:     EMACS_ATTRIBUTE_NONNULL(1);
// C 30: 
// C 31:   void (*non_local_exit_throw) (emacs_env *env,
// C 32: 				emacs_value tag, emacs_value value)
// C 33:     EMACS_ATTRIBUTE_NONNULL(1);
// C 34: 
// C 35:   /* Function registration.  */
// C 36: 
// C 37:   emacs_value (*make_function) (emacs_env *env,
// C 38: 				ptrdiff_t min_arity,
// C 39: 				ptrdiff_t max_arity,
// C 40: 				emacs_value (*func) (emacs_env *env,
// C 41:                                                      ptrdiff_t nargs,
// C 42:                                                      emacs_value* args,
// C 43:                                                      void *data)
// C 44: 				  EMACS_NOEXCEPT
// C 45:                                   EMACS_ATTRIBUTE_NONNULL(1),
// C 46: 				const char *docstring,
// C 47: 				void *data)
// C 48:     EMACS_ATTRIBUTE_NONNULL(1, 4);
// C 49: 
// C 50:   emacs_value (*funcall) (emacs_env *env,
// C 51:                           emacs_value func,
// C 52:                           ptrdiff_t nargs,
// C 53:                           emacs_value* args)
// C 54:     EMACS_ATTRIBUTE_NONNULL(1);
// C 55: 
// C 56:   emacs_value (*intern) (emacs_env *env, const char *name)
// C 57:     EMACS_ATTRIBUTE_NONNULL(1, 2);
// C 58: 
// C 59:   /* Type conversion.  */
// C 60: 
// C 61:   emacs_value (*type_of) (emacs_env *env, emacs_value arg)
// C 62:     EMACS_ATTRIBUTE_NONNULL(1);
// C 63: 
// C 64:   bool (*is_not_nil) (emacs_env *env, emacs_value arg)
// C 65:     EMACS_ATTRIBUTE_NONNULL(1);
// C 66: 
// C 67:   bool (*eq) (emacs_env *env, emacs_value a, emacs_value b)
// C 68:     EMACS_ATTRIBUTE_NONNULL(1);
// C 69: 
// C 70:   intmax_t (*extract_integer) (emacs_env *env, emacs_value arg)
// C 71:     EMACS_ATTRIBUTE_NONNULL(1);
// C 72: 
// C 73:   emacs_value (*make_integer) (emacs_env *env, intmax_t n)
// C 74:     EMACS_ATTRIBUTE_NONNULL(1);
// C 75: 
// C 76:   double (*extract_float) (emacs_env *env, emacs_value arg)
// C 77:     EMACS_ATTRIBUTE_NONNULL(1);
// C 78: 
// C 79:   emacs_value (*make_float) (emacs_env *env, double d)
// C 80:     EMACS_ATTRIBUTE_NONNULL(1);
// C 81: 
// C 82:   /* Copy the content of the Lisp string VALUE to BUFFER as an utf8
// C 83:      null-terminated string.
// C 84: 
// C 85:      SIZE must point to the total size of the buffer.  If BUFFER is
// C 86:      NULL or if SIZE is not big enough, write the required buffer size
// C 87:      to SIZE and return true.
// C 88: 
// C 89:      Note that SIZE must include the last null byte (e.g. "abc" needs
// C 90:      a buffer of size 4).
// C 91: 
// C 92:      Return true if the string was successfully copied.  */
// C 93: 
// C 94:   bool (*copy_string_contents) (emacs_env *env,
// C 95:                                 emacs_value value,
// C 96:                                 char *buf,
// C 97:                                 ptrdiff_t *len)
// C 98:     EMACS_ATTRIBUTE_NONNULL(1, 4);
// C 99: 
// C 100:   /* Create a Lisp string from a utf8 encoded string.  */
// C 101:   emacs_value (*make_string) (emacs_env *env,
// C 102: 			      const char *str, ptrdiff_t len)
// C 103:     EMACS_ATTRIBUTE_NONNULL(1, 2);
// C 104: 
// C 105:   /* Embedded pointer type.  */
// C 106:   emacs_value (*make_user_ptr) (emacs_env *env,
// C 107: 				void (*fin) (void *) EMACS_NOEXCEPT,
// C 108: 				void *ptr)
// C 109:     EMACS_ATTRIBUTE_NONNULL(1);
// C 110: 
// C 111:   void *(*get_user_ptr) (emacs_env *env, emacs_value arg)
// C 112:     EMACS_ATTRIBUTE_NONNULL(1);
// C 113:   void (*set_user_ptr) (emacs_env *env, emacs_value arg, void *ptr)
// C 114:     EMACS_ATTRIBUTE_NONNULL(1);
// C 115: 
// C 116:   void (*(*get_user_finalizer) (emacs_env *env, emacs_value uptr))
// C 117:     (void *) EMACS_NOEXCEPT EMACS_ATTRIBUTE_NONNULL(1);
// C 118:   void (*set_user_finalizer) (emacs_env *env, emacs_value arg,
// C 119: 			      void (*fin) (void *) EMACS_NOEXCEPT)
// C 120:     EMACS_ATTRIBUTE_NONNULL(1);
// C 121: 
// C 122:   /* Vector functions.  */
// C 123:   emacs_value (*vec_get) (emacs_env *env, emacs_value vector, ptrdiff_t index)
// C 124:     EMACS_ATTRIBUTE_NONNULL(1);
// C 125: 
// C 126:   void (*vec_set) (emacs_env *env, emacs_value vector, ptrdiff_t index,
// C 127: 		   emacs_value value)
// C 128:     EMACS_ATTRIBUTE_NONNULL(1);
// C 129: 
// C 130:   ptrdiff_t (*vec_size) (emacs_env *env, emacs_value vector)
// C 131:     EMACS_ATTRIBUTE_NONNULL(1);
