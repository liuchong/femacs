# emacs/src/comp.h (106 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [ ] NATIVE_COMP_UNITP  (line 57)
- [ ] XNATIVE_COMP_UNIT  (line 63)
- [x] maybe_defer_native_compilation  (line 92, static)

## Macros defined — 1
- [x] COMP_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 1
- [ ] Lisp_Native_Comp_Unit: union vectorlike_header header, Lisp_Object file, Lisp_Object optimize_qualities, Lisp_Object lambda_gc_guard_h, Lisp_Object lambda_c_name_idx_h, Lisp_Object data_fdoc_v, Lisp_Object data_vec, Lisp_Object* data_relocs, bool loaded_once, bool load_ongoing, dynlib_handle_ptr handle

## File-scope objects — 0
