# emacs/src/syntax.h (215 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 11
- [x] syntax_property_entry  (line 98)
- [x] SYNTAX_ENTRY  (line 107)
- [x] syntax_property_with_flags  (line 116)
- [x] SYNTAX_WITH_FLAGS  (line 122)
- [x] syntax_property  (line 128)
- [x] SYNTAX  (line 133)
- [x] RE_SYNTAX_TABLE_BYTE_TO_CHAR  (line 156)
- [x] UPDATE_SYNTAX_TABLE_FORWARD  (line 175)
- [x] UPDATE_SYNTAX_TABLE_BACKWARD  (line 185)
- [x] UPDATE_SYNTAX_TABLE  (line 194)
- [x] SETUP_BUFFER_SYNTAX_TABLE  (line 203)

## Macros defined — 2
- [x] EMACS_SYNTAX_H  (line 22, body da39a3ee5e6b)
- [x] Vstandard_syntax_table  (line 34, body 31d51ea67eca)

## Types and their fields — 1
- [x] gl_state_s: Lisp_Object object, ptrdiff_t start, ptrdiff_t stop, bool use_global, Lisp_Object global_code, Lisp_Object current_syntax_table, Lisp_Object old_prop, ptrdiff_t b_property, ptrdiff_t e_property, bool e_property_truncated, INTERVAL forward_i, INTERVAL backward_i

## File-scope objects — 1
- [x] extern struct gl_state_s gl_state  (line 90)

## Decisions

- **The C `INLINE` definitions** (syntax_property_entry … SETUP_BUFFER_SYNTAX_TABLE, 11 of
  them) are `#[inline(always)] pub fn`s, as every other header mirror spells a header's
  inline function (PORTING.md P-12/P-37); none of them emits a symbol.
- **`enum syntaxcode` (syntax.h 43-63)** is spelled as its C type plus one constant per
  member (PORTING.md 1.3, the coding.rs precedent), because `syntax_property` masks a
  character-table value with `& 0xff` and the callers compare the result against `Sword`
  and friends.
- **`#define Vstandard_syntax_table BVAR (&buffer_defaults, syntax_table)`** becomes a
  `macro_rules!` of the same name; `macro_rules!` cannot paste onto an identifier, so the
  call site names the field with the `_` C's `BVAR` appends (the buffer.rs `BVAR!` row).
- **`struct gl_state_s` is defined here and `gl_state` is only declared**: the definition of
  the storage C writes in syntax.c (`struct gl_state_s gl_state;`) belongs to that unit and
  lands with it, exactly as the header/mirror split requires (PORTING.md 0.1-3).  Its two
  `INTERVAL` members use lisp.rs's opaque `*mut c_void` (P-55).
- **`RE_SYNTAX_TABLE_BYTE_TO_CHAR`** needs `string_byte_to_char`, which lisp.h 4277 declares
  and no mirror carried; it was added to lisp.rs's `other_units` group in the same change
  (single definition point, PORTING.md 0.1-3).
- `BEGV_BYTE` is the buffer.rs macro `BEGV_BYTE! ()`; `parse_sexp_lookup_properties` is the
  generated global, read in an `unsafe` block.
- Evidence: `tools/extract-abi.py --check emacs/src/syntax.h
  femacs/src/include/syntax.rs` -> OK; the mirror is compiled by
  `xtask build-tu src/casefiddle.c` (casefiddle.c includes syntax.h).
