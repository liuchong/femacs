# emacs/src/composite.h (362 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [x] composition_registered_p  (line 61)
- [x] COMPOSITION_ENCODE_RULE_VALID  (line 128)
- [x] composition_method  (line 208)
- [x] composition_valid_p  (line 227)
- [x] lgstring_glyph_addr  (line 276)

## Macros defined — 53
- [x] CHECK_ALL  (line 192, body 97f18ea047bc)
- [x] CHECK_BORDER  (line 191, body f554c4819687)
- [x] CHECK_HEAD  (line 188, body 356a192b7913)
- [x] CHECK_INSIDE  (line 190, body 1b6453892473)
- [x] CHECK_TAIL  (line 189, body da4b9237bacc)
- [x] COMPOSITION_COMPONENTS  (line 76, body 73fcf73bb2ad)
- [x] COMPOSITION_DECODE_REFS  (line 107, body 70c806bc295d)
- [x] COMPOSITION_DECODE_RULE  (line 118, body be91dbccc676)
- [x] COMPOSITION_ENCODE_RULE  (line 135, body 58c6c01ea833)
- [x] COMPOSITION_GLYPH  (line 91, body 1a852f289152)
- [x] COMPOSITION_ID  (line 67, body 8c35dc19ddd8)
- [x] COMPOSITION_KEY  (line 87, body 9bf3e2fc77e4)
- [x] COMPOSITION_LENGTH  (line 70, body a897f8ed9b84)
- [x] COMPOSITION_MODIFICATION_FUNC  (line 82, body 9781ba3458fc)
- [x] COMPOSITION_RULE  (line 99, body c6b88a55e5dc)
- [x] EMACS_COMPOSITE_H  (line 26, body da39a3ee5e6b)
- [x] LGLYPH_ADJUSTMENT  (line 312, body 9d6e48334dc0)
- [x] LGLYPH_ASCENT  (line 310, body 65b36a5d936a)
- [x] LGLYPH_CHAR  (line 302, body 77fe347b44b8)
- [x] LGLYPH_CODE  (line 303, body 6cdeafce40c8)
- [x] LGLYPH_DESCENT  (line 311, body 0dc7e456dae1)
- [x] LGLYPH_FROM  (line 300, body 3f6a1038772e)
- [x] LGLYPH_LBEARING  (line 308, body 1a80c85b9913)
- [x] LGLYPH_NEW  (line 299, body 51a92b4d884d)
- [x] LGLYPH_RBEARING  (line 309, body 398f9cd4f074)
- [x] LGLYPH_SET_ADJUSTMENT  (line 326, body 37ddfed43e3f)
- [x] LGLYPH_SET_ASCENT  (line 324, body a5404e6ab7a3)
- [x] LGLYPH_SET_CHAR  (line 315, body 08508bbf1092)
- [x] LGLYPH_SET_CODE  (line 317, body b27f26724317)
- [x] LGLYPH_SET_DESCENT  (line 325, body b6c946c96ab0)
- [x] LGLYPH_SET_FROM  (line 313, body ce91dd688510)
- [x] LGLYPH_SET_LBEARING  (line 322, body 6c072719ea03)
- [x] LGLYPH_SET_RBEARING  (line 323, body efcebf89ee35)
- [x] LGLYPH_SET_TO  (line 314, body 2aaca5e16c2a)
- [x] LGLYPH_SET_WIDTH  (line 321, body bff070da18c0)
- [x] LGLYPH_TO  (line 301, body 8bbecd07a9bd)
- [x] LGLYPH_WADJUST  (line 332, body 821e032db80e)
- [x] LGLYPH_WIDTH  (line 307, body f6e4da62046d)
- [x] LGLYPH_XOFF  (line 328, body f27362335157)
- [x] LGLYPH_YOFF  (line 330, body 4e4d2eca98d3)
- [x] LGSTRING_CHAR  (line 259, body 10c2428b1bea)
- [x] LGSTRING_CHAR_LEN  (line 260, body bcf8967387b8)
- [x] LGSTRING_FONT  (line 258, body 355b16539740)
- [x] LGSTRING_GLYPH  (line 273, body 98a9c02d9752)
- [x] LGSTRING_GLYPH_LEN  (line 272, body dc7a5b209df8)
- [x] LGSTRING_HEADER  (line 251, body aa0656a6bb0c)
- [x] LGSTRING_ID  (line 265, body 01c7241f3549)
- [x] LGSTRING_SET_CHAR  (line 263, body ca5183bb66d8)
- [x] LGSTRING_SET_FONT  (line 262, body a729d99585ff)
- [x] LGSTRING_SET_GLYPH  (line 274, body 24ce9489ca27)
- [x] LGSTRING_SET_HEADER  (line 252, body 1d427054052c)
- [x] LGSTRING_SET_ID  (line 266, body 54824e869d73)
- [x] MAX_COMPOSITION_COMPONENTS  (line 50, body 1574bddb75c7)

## Types and their fields — 1
- [x] composition: int glyph_len, short pixel_width, short ascent, short descent, short lbearing, short rbearing, int width, enum composition_method method, Lisp_Object key, void* font, short* offsets

## File-scope objects — 2
- [x] extern struct composition composition_table  (line 182)
- [x] extern ptrdiff_t n_compositions  (line 184)

## Decisions (2026-10-03, completed with the composite.c port)

`emacs/src/composite.h` @ `fd7afced403`; mirror `femacs/src/include/composite.rs` (469 lines).
The header was a WIP until `composite.c` landed (it is the unit that defines every function
and both objects the header declares), and this round completed it:

* The 53 macros are all present.  The value accessors are `#[inline(always)] pub fn`s whose
  bodies are C's (`COMPOSITION_ID`, `_LENGTH`, `_COMPONENTS`, `_MODIFICATION_FUNC`, `_KEY`,
  `_GLYPH`, `_RULE`, `_ENCODE_RULE`, the 44 `LGSTRING_*`/`LGLYPH_*` pairs) — the header's own
  pre-existing shape for `LGSTRING_GLYPH_LEN`/`LGSTRING_GLYPH`, which `character.rs` calls as
  functions, and the task's "safe `fn` for value-only helpers"; a fn keeps C's call text at
  every use site.  Two stay `macro_rules!` (`LGLYPH_CODE`, `LGLYPH_SET_CODE`), because their
  bodies name font.h's `FONT_INVALID_CODE`, which this mirror cannot declare without colliding
  with font.rs.  The two *lvalue* statement macros (`COMPOSITION_DECODE_REFS`,
  `COMPOSITION_DECODE_RULE`) are `pub fn` helpers taking `*mut c_int` (Rust cannot write its
  caller's locals, PORTING.md 3.8); their C `do { … } while (false)` tail means their `false`
  token is exempted in `docs/ports/macro-body-notes.tsv`, and `LGLYPH_CODE`'s `TYPE_MAXIMUM
  (unsigned)` (gnulib `<intprops.h>`) is written `u32::MAX as u64` with the token named in the
  same table (P-26).  `COMPOSITION_ENCODE_RULE_VALID` and the five INLINE functions keep C's
  bodies.
* The five INLINE functions are `#[inline(always)] pub fn` with C's bodies
  (`composition_registered_p`, `COMPOSITION_ENCODE_RULE_VALID`, `composition_method`,
  `composition_valid_p`, `lgstring_glyph_addr`); `EMACS_COMPOSITE_H` and the
  `LGLYPH_IX_*`/`LGLYPH_SIZE` constants keep their C values.
* The nine `extern` prototypes and the two `extern` objects moved into
  `pub mod composite_decls` / `pub mod composite_objs` + glob imports (PORTING.md 0.1-10,
  P-21): `composite.c` defines all of them, and a declaration at the same scope would be
  `E0428`.  Three prototypes spell an unmirrored parameter type as the ABI-equal
  `*mut c_void` (`struct font_metrics *`, `struct composition_it *`, `struct face *`), each
  with the C prototype in a `/// C:` line above it (P-72 rule 3); the defining unit spells the
  precise type where it has one.
* `tools/extract-abi.py --check emacs/src/composite.h femacs/src/include/composite.rs`
  reports `OK`; all 61 ledger rows are ticked.

The two offsets measured for the unit (`struct font`'s `driver` at 176, `struct
font_driver`'s `encode_char` at 96) are recorded in `docs/ports/composite.md`, where the
use-site declarations of those unmirrored records live.
