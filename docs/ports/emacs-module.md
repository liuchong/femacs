# emacs/src/emacs-module.c (1798 lines)

> **This file is not compiled by the frozen configuration** (docs/baseline/frozen-build-sources.txt, docs/status.md 3.31): 92 declaration(s) are deferred to the platform target, and the port is an empty unit plus the note that names the gate.  Nothing in this file is required here.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 0

## Types and their fields — 6
- [x] Lisp_Module_Function: union vectorlike_header header, Lisp_Object documentation, Lisp_Object interactive_form, Lisp_Object command_modes, ptrdiff_t min_arity, ptrdiff_t max_arity, emacs_function subr, void* data, emacs_finalizer finalizer
- [x] emacs_env_private: enum emacs_funcall_exit pending_non_local_exit, Lisp_Object non_local_exit_symbol, Lisp_Object non_local_exit_data, struct emacs_value_storage storage
- [x] emacs_runtime_private: emacs_env* env
- [x] emacs_value_frame: struct emacs_value_tag objects, int offset, struct emacs_value_frame* next
- [x] emacs_value_storage: struct emacs_value_frame initial, struct emacs_value_frame* current
- [x] module_global_reference: union vectorlike_header header, struct emacs_value_tag value, ptrdiff_t refcount

## File-scope objects — 0

## Decisions

- **The frozen configuration does not compile this file** (docs/baseline/frozen-build-sources.txt,
  generated from `make -n` in `build/emacs-ref/src`): this target's platform object variables
  (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`) are all empty, so the file
  is not in the build's source list at all, and its text needs headers this target has not got.
  The port is therefore the empty unit `femacs/src/emacs-module.rs` — C's own license header plus the
  note that names the exclusion and points here — and the declarations above are **deferred to
  the platform target** rather than dropped (docs/status.md §3.31).
