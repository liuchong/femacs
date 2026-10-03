# emacs/src/regex-emacs.h (202 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_REGEX_H  (line 20, body 356a192b7913)

## Types and their fields — 2
- [x] re_pattern_buffer: unsigned char* buffer, ptrdiff_t allocated, ptrdiff_t used, int charset_unibyte, char* fastmap, Lisp_Object translate, ptrdiff_t re_nsub, bool_bf can_be_null : 1, unsigned regs_allocated : 2, bool_bf fastmap_accurate : 1, bool_bf used_syntax : 1, bool_bf multibyte : 1, bool_bf target_multibyte : 1
- [x] re_registers: ptrdiff_t num_regs, ptrdiff_t* start, ptrdiff_t* end

## File-scope objects — 3
- [x] extern Lisp_Object re_match_object  (line 50)
- [x] extern ptrdiff_t emacs_re_max_failures  (line 53)
- [x] extern ptrdiff_t emacs_re_safe_alloca  (line 56)

## Decisions

Partial header mirror (PORTING.md P-66/P-24): this file carries what its first consumers name.
`struct re_registers` (regex-emacs.h 28-35, 24 bytes, `num_regs@0`, `start@8`, `end@16`) landed
with `src/thread.c`: C declares it *before* including lisp.h precisely because thread.h embeds
two of them, and `free_search_regs` reads their three fields.  The guard body
(`#define EMACS_REGEX_H 1`) and the three file-scope objects the checker required
(`re_match_object`, `emacs_re_max_failures`, `emacs_re_safe_alloca`, now in
`regex_emacs_decls` + glob import) landed with `src/thread.c`, so `--check` is OK; what
remains WIP is `re_pattern_buffer` and the `re_*` API, which belong with `src/regex-emacs.c`'s
own port (the one unchecked row below is `re_pattern_buffer`, not a gate failure).

The mirror's `include!` list is empty by design, as it was before this change: it is included by
the units that need it, after `lisp.rs` (and now by `thread.rs`'s own mirror, exactly as C's
thread.h 21 includes it).  `ptrdiff_t` and `c_int` come from the including unit.

**The engine half lands with its definition unit (2026-10-03, the `docs/ports/thread.md`
precedent).**  `struct re_pattern_buffer` (C 67-121) and the eight declarations are in the
mirror now, and `src/regex-emacs.c`'s first consumer is `femacs/src/regex-emacs.rs`.  C's two
`bool_bf` runs and its `unsigned regs_allocated : 2` are one storage unit, so the mirror packs
them into `bits: u8` with one mask per C member (`RE_PATTERN_*`, PORTING.md 1.3/P-80) and one
`#[inline(always)]` accessor pair per member; that is the spelling `src/search.c`'s port already
used, and `docs/ports/token-aliases.tsv` carries all six families (`can_be_null`,
`regs_allocated`, `fastmap_accurate`, the last three from the search lane).  `REGS_UNALLOCATED` /
`REGS_REALLOCATE` / `REGS_FIXED` are C 862's `enum` in the **.c**, not the header, so they stay
with the unit (and with `search.rs`'s own use site); the mirror does not carry them.

**Every function declaration moved into `regex_emacs_decls` + glob import** (PORTING.md 0.1-10):
`re_iswctype` and `re_wctype_parse` used to sit at mirror top level because only `syntax.rs`
(which declares nothing) named them; now `src/regex-emacs.c`'s port defines all eight, and a
same-named declaration plus definition in one unit is an `E0428`.  The definition shadows the
glob import, which is exactly C's prototype-plus-definition.  `print_compiled_pattern` is
`#[cfg (enable_checking)]`, as the C text's `#if ENABLE_CHECKING` is.

**`struct re_pattern_buffer`'s layout is pinned by three units' assertions** (this mirror's, and
`src/search.rs`'s and `src/regex-emacs.rs`'s own copies went away with this change): clang on the
frozen target reports `sizeof` 64 / alignment 8 with `buffer`@0, `allocated`@8, `used`@16,
`charset_unibyte`@24, `fastmap`@32, `translate`@40, `re_nsub`@48 and the bit run at 56; the ten
`const _` assertions above are those numbers.

**Line-count relation:** C 202 lines → 255 Rust lines; the mirror is not in
`docs/ports/line-drift-tolerance.tsv` because the whole header lands at once here and the
declaration-per-line shape is the recorded deviation (the `ccl.rs`/`thread.rs` precedent).
