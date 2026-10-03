# emacs/src/regex-emacs.h (202 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [ ] EMACS_REGEX_H  (line 20, body 356a192b7913)

## Types and their fields — 2
- [ ] re_pattern_buffer: unsigned char* buffer, ptrdiff_t allocated, ptrdiff_t used, int charset_unibyte, char* fastmap, Lisp_Object translate, ptrdiff_t re_nsub, bool_bf can_be_null : 1, unsigned regs_allocated : 2, bool_bf fastmap_accurate : 1, bool_bf used_syntax : 1, bool_bf multibyte : 1, bool_bf target_multibyte : 1
- [ ] re_registers: ptrdiff_t num_regs, ptrdiff_t* start, ptrdiff_t* end

## File-scope objects — 3
- [ ] extern Lisp_Object re_match_object  (line 50)
- [ ] extern ptrdiff_t emacs_re_max_failures  (line 53)
- [ ] extern ptrdiff_t emacs_re_safe_alloca  (line 56)
