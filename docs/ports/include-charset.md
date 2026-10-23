# emacs/src/charset.h (567 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] charset_attributes_getter  (line 305)
- [x] set_charset_attr  (line 354)

## Macros defined — 63
- [x] CHARSETP  (line 362, body 3758acdb4a9e)
- [x] CHARSET_ASCII_COMPATIBLE_P  (line 325, body 505e2da4c70d)
- [x] CHARSET_ATTRIBUTES  (line 314, body 878fa571bd21)
- [x] CHARSET_ATTR_DECODER  (line 289, body e72fd0eaa96f)
- [x] CHARSET_ATTR_DEUNIFIER  (line 294, body 10e210da8ddd)
- [x] CHARSET_ATTR_ENCODER  (line 290, body e2e140798420)
- [x] CHARSET_ATTR_ID  (line 285, body 6a3de50a4d3b)
- [x] CHARSET_ATTR_MAP  (line 288, body 5a571e1c2ec1)
- [x] CHARSET_ATTR_NAME  (line 286, body 7460ae3a841c)
- [x] CHARSET_ATTR_PLIST  (line 287, body ac0a64fd7c6d)
- [x] CHARSET_ATTR_SUBSET  (line 291, body a0bf2ef35014)
- [x] CHARSET_ATTR_SUPERSET  (line 292, body e235a0ace859)
- [x] CHARSET_ATTR_UNIFY_MAP  (line 293, body 0c5367162a6c)
- [x] CHARSET_CODE_LINEAR_P  (line 319, body 05d1a4ca5288)
- [x] CHARSET_CODE_OFFSET  (line 333, body 303daf0e236e)
- [x] CHARSET_CODE_SPACE  (line 318, body 84769af5c445)
- [x] CHARSET_COMPACT_CODES_P  (line 326, body c5a60aaf51b3)
- [x] CHARSET_DECODER  (line 340, body 6ac1d96c76c3)
- [x] CHARSET_DEUNIFIER  (line 350, body ddc56b369cab)
- [x] CHARSET_DIMENSION  (line 317, body 93986672f06e)
- [x] CHARSET_EMACS_MULE_ID  (line 324, body 8e2dcbe406f7)
- [x] CHARSET_ENCODER  (line 342, body 355ce6b3dfbd)
- [x] CHARSET_FAST_MAP_REF  (line 503, body 09364f5de391)
- [x] CHARSET_FAST_MAP_SET  (line 508, body e2f614db2152)
- [x] CHARSET_FROM_ID  (line 267, body 761d0a5f238b)
- [x] CHARSET_ID  (line 316, body 0cc9f6a1ff6a)
- [x] CHARSET_INVALID_CODE  (line 330, body 572431870c2b)
- [x] CHARSET_ISO_CHARS_96  (line 320, body debfb98b9506)
- [x] CHARSET_ISO_FINAL  (line 321, body 27d5a1ee7155)
- [x] CHARSET_ISO_PLANE  (line 322, body 64edef2460ce)
- [x] CHARSET_ISO_REVISION  (line 323, body f909ae3971d1)
- [x] CHARSET_MAP  (line 338, body 9ce427c08155)
- [x] CHARSET_MAX_CHAR  (line 332, body be75cdaed8b2)
- [x] CHARSET_MAX_CODE  (line 329, body d0f934eccf65)
- [x] CHARSET_METHOD  (line 327, body e0b767ea39fc)
- [x] CHARSET_MIN_CHAR  (line 331, body 73053acc1112)
- [x] CHARSET_MIN_CODE  (line 328, body 55832579bace)
- [x] CHARSET_NAME  (line 336, body 9ec765a8adb8)
- [x] CHARSET_SUBSET  (line 344, body 4498e700ba75)
- [x] CHARSET_SUPERSET  (line 346, body 37604ca702b3)
- [x] CHARSET_SYMBOL_ATTRIBUTES  (line 282, body 73354dd622d3)
- [x] CHARSET_SYMBOL_HASH_INDEX  (line 301, body 78e05c89f280)
- [x] CHARSET_SYMBOL_ID  (line 296, body 5140074edf2d)
- [x] CHARSET_UNIFIED_P  (line 334, body 8ff86afcfbd0)
- [x] CHARSET_UNIFY_MAP  (line 348, body 5a763b1e6209)
- [x] CHAR_CHARSET  (line 404, body b960d06a2a79)
- [x] CHAR_CHARSET_P  (line 519, body b58042077d32)
- [x] CHECK_CHARSET  (line 365, body 3b12f6be8a09)
- [x] CHECK_CHARSET_GET_ATTR  (line 387, body e52922e52645)
- [x] CHECK_CHARSET_GET_CHARSET  (line 394, body 124ec1f9c831)
- [x] CHECK_CHARSET_GET_ID  (line 374, body dc3c39247f86)
- [x] DECODE_CHAR  (line 427, body 167b53ca3af7)
- [x] EMACS_CHARSET_H  (line 28, body da39a3ee5e6b)
- [x] EMACS_MULE_LEADING_CODE_PRIVATE_11  (line 538, body e08ca5545958)
- [x] EMACS_MULE_LEADING_CODE_PRIVATE_12  (line 539, body 4c3a37daadc2)
- [x] EMACS_MULE_LEADING_CODE_PRIVATE_21  (line 540, body cabe18f900a5)
- [x] EMACS_MULE_LEADING_CODE_PRIVATE_22  (line 541, body 5520abb0bc0f)
- [x] ENCODE_CHAR  (line 451, body ffaadad52529)
- [x] ISO_CHARSET_TABLE  (line 499, body 065e4dbc2682)
- [x] ISO_MAX_CHARS  (line 489, body da4b9237bacc)
- [x] ISO_MAX_DIMENSION  (line 488, body 77de68daecd8)
- [x] ISO_MAX_FINAL  (line 490, body 70070b762760)
- [x] SPLIT_CHAR  (line 484, body 51dcb19d7a56)

## Types and their fields — 2
- [x] charset: int id, int dimension, int code_space, unsigned char* code_space_mask, bool_bf code_linear_p : 1, bool_bf iso_chars_96 : 1, bool_bf ascii_compatible_p : 1, bool_bf supplementary_p : 1, bool_bf compact_codes_p : 1, bool_bf unified_p : 1, int iso_final, int iso_revision, int emacs_mule_id, enum charset_method method, unsigned min_code, unsigned max_code, unsigned char_index_offset, int min_char, int max_char, unsigned invalid_code, unsigned char fast_map, int code_offset
- [x] charset_table: struct charset* start, int size, int used, Lisp_Object attributes_table

## File-scope objects — 19
- [x] extern Lisp_Object Vcharset_hash_table  (line 244)
- [x] extern struct charset_table charset_table  (line 265)
- [x] extern Lisp_Object Vcharset_ordered_list  (line 269)
- [x] extern Lisp_Object Vcharset_non_preferred_head  (line 270)
- [x] extern EMACS_UINT charset_ordered_list_tick  (line 272)
- [x] extern Lisp_Object Viso_2022_charset_list  (line 274)
- [x] extern Lisp_Object Vemacs_mule_charset_list  (line 275)
- [x] extern int emacs_mule_charset  (line 277)
- [x] extern Lisp_Object charset_work  (line 446)
- [x] extern bool charset_map_loaded  (line 479)
- [x] extern int iso_charset_table  (line 495)
- [x] extern int charset_ascii  (line 545)
- [x] extern int charset_eight_bit  (line 545)
- [x] extern int charset_unicode  (line 546)
- [x] extern int charset_jisx0201_roman  (line 547)
- [x] extern int charset_jisx0208_1978  (line 548)
- [x] extern int charset_jisx0208  (line 549)
- [x] extern int charset_ksc5601  (line 550)
- [x] extern int charset_unibyte  (line 552)

## Decisions

- **What the header carries.** The three enums (`define_charset_arg_index`,
  `charset_attr_index`, `charset_method`), `struct charset`, `struct charset_table`, the 63
  macros, the two INLINE functions (`charset_attributes_getter`, `set_charset_attr`) and
  the nineteen `extern` objects plus seven prototypes.  The mirror lands with
  `src/charset.rs` (the composite.rs/intervals.rs precedent).
- **`bits: u8` for the six `bool_bf : 1` members** of `struct charset` (PORTING.md 1.3):
  `bool_bf` is `bool` on the frozen target (conf_post.h 56-61), so C stores all six in one
  byte.  The mirror reads and writes them with `charset_bits!` over the `CHARSET_*_P`
  masks, keeping the C member names in the port text; the six families are in
  `docs/ports/token-aliases.tsv`.  **The layout is measured, not derived**: a standalone
  clang translation of the header's declaration on the frozen target gives
  `sizeof (struct charset)` = 320 / align 8 with id 0, dimension 4, code_space 8,
  code_space_mask 72, the six bits at 80:0-80:5, iso_final 84, iso_revision 88,
  emacs_mule_id 92, method 96, min_code 100, max_code 104, char_index_offset 108,
  min_char 112, max_char 116, invalid_code 120, fast_map 124, code_offset 316; the
  mirror pins them with 20 `const _` assertions (`sizeof`, `alignof`, 18 offsets).
  `struct charset` is `#[derive (Clone, Copy)]` (a plain C record is trivially copyable,
  PORTING.md 0.1-9), which is what lets the unit's `charset_table.start[id] = charset;`
  stay a plain assignment.
- **`#if false` (charset.h 408-421)** — `Vchar_charset_set`, `CHAR_CHARSET_SET`,
  `SAME_CHARSET_P` — is literal-false text, not a configuration arm, so P-51's feature gate
  does not apply and the three names are not mirrored (the intervals.rs precedent).
- **`CHARSET_ISO_PLANE` (charset.h 322)** reads a field `struct charset` does not have; it
  is mirrored as the header spells it and is never expanded (a `macro_rules!` body is not
  resolved until expansion).
- **`charset_attributes` (charset.h 555)** is declared by the header and defined nowhere in
  the reference tree; the mirror declares it outside `charset_decls`.
- **The five prototypes this unit defines** (`char_charset`, `decode_char`, `encode_char`,
  `string_xstring_p`, `map_charset_chars`) live in `charset_decls` + glob import
  (PORTING.md 0.1-10) and the nineteen objects in `charset_objs` + glob import (P-21), so
  the unit's own definitions and `#[unsafe(no_mangle)] pub static mut` objects are not
  second declarations at the crate root (E0428).
- **Mirror gaps reported by the consumers** (not fixed here): `lisp.rs` lacks
  `larger_vector` (lisp.h 4245) and `make_specified_string` (lisp.h 4566), which ccl.rs and
  chartab.rs still declare at their use sites.  `charset.h`'s own mirror now gives those
  two units a home for their local charset declarations whenever their owners choose to
  switch (their declarations shadow the glob imports, PORTING.md 0.1-10).
- **`docs/ports/macro-body-notes.tsv`** records the six rows this mirror's macros need: the
  five `do { … } while (false)` statement macros (`false`, the PDUMPER_RESET class) and
  `ENCODE_CHAR`'s `verify_expr` (charset.h 24's `<verify.h>` wrapper; the two `sizeof`s it
  compares are the `sizeof` family and are spelled).
- **Header mirrors are not standalone translation units** in this tree: `xtask
  finished_units` skips `include-*.md` ledgers, and `build-tu src/intervals.h`,
  `src/composite.h`, `src/ccl.h` and `src/character.h` all fail the same way this mirror
  does (`config.rs` is not part of a header's own include graph).  The mirror is compiled
  through the units that include it (`build-tu src/charset.c`).
