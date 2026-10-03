# emacs/src/regex-emacs.h (202 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_REGEX_H  (line 20, body 356a192b7913)

## Types and their fields — 2
- [ ] re_pattern_buffer: unsigned char* buffer, ptrdiff_t allocated, ptrdiff_t used, int charset_unibyte, char* fastmap, Lisp_Object translate, ptrdiff_t re_nsub, bool_bf can_be_null : 1, unsigned regs_allocated : 2, bool_bf fastmap_accurate : 1, bool_bf used_syntax : 1, bool_bf multibyte : 1, bool_bf target_multibyte : 1
- [ ] re_registers: ptrdiff_t num_regs, ptrdiff_t* start, ptrdiff_t* end

## File-scope objects — 3
- [ ] extern Lisp_Object re_match_object  (line 50)
- [ ] extern ptrdiff_t emacs_re_max_failures  (line 53)
- [ ] extern ptrdiff_t emacs_re_safe_alloca  (line 56)

## Decisions

Partial header mirror (PORTING.md P-66/P-24): this file carries what its first consumers name.
`struct re_registers` (regex-emacs.h 28-35, 24 bytes, `num_regs@0`, `start@8`, `end@16`) landed
with `src/thread.c`: C declares it *before* including lisp.h precisely because thread.h embeds
two of them, and `free_search_regs` reads their three fields.  The guard body
(`#define EMACS_REGEX_H 1`) and the three file-scope objects the checker required
(`re_match_object`, `emacs_re_max_failures`, `emacs_re_safe_alloca`, now in
`regex_emacs_decls` + glob import) landed with `src/thread.c`, so `--check` is OK; what
remains WIP is `re_pattern_buffer` and the `re_*` API, which belong with `src/regex-emacs.c`'s
own port (2 unchecked rows below, not a gate failure).

The mirror's `include!` list is empty by design, as it was before this change: it is included by
the units that need it, after `lisp.rs` (and now by `thread.rs`'s own mirror, exactly as C's
thread.h 21 includes it).  `ptrdiff_t` and `c_int` come from the including unit.
