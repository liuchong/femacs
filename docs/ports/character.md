# emacs/src/character.c (1167 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 10
- [x] characterp  (Fcharacterp, 1/2, doc 54d30af022d2, line 169)
- [x] max-char  (Fmax_char, 0/1, doc 0b6c2509a588, line 181)
- [x] unibyte-char-to-multibyte  (Funibyte_char_to_multibyte, 1/1, doc 64df2d255087, line 193)
- [x] multibyte-char-to-unibyte  (Fmultibyte_char_to_unibyte, 1/1, doc ddb13c87d43c, line 207)
- [x] char-width  (Fchar_width, 1/1, doc 88b52e6bd194, line 262)
- [x] string-width  (Fstring_width, 1/3, doc 536768fcd28e, line 466)
- [x] string  (Fstring, 0/MANY, doc bd9a55b6e360, line 846)
- [x] unibyte-string  (Funibyte_string, 0/MANY, doc 8af29d1ebbca, line 871)
- [x] char-resolve-modifiers  (Fchar_resolve_modifiers, 1/1, doc 2c0eabd42f7f, line 883)
- [x] get-byte  (Fget_byte, 0/2, doc beade79d86ca, line 898)

## Functions defined — 23
- [x] char_resolve_modifier_mask  (line 52)
- [x] char_string  (line 101)
- [x] translate_char  (line 151)
- [x] char_width  (line 232, static)
- [x] c_string_width  (line 290)
- [x] strwidth  (line 328)
- [x] lisp_string_width  (line 344)
- [x] chars_in_text  (line 506)
- [x] multibyte_chars_in_text  (line 522)
- [x] parse_str_as_multibyte  (line 547)
- [x] str_as_multibyte  (line 590)
- [x] count_size_as_multibyte  (line 672)
- [x] str_to_multibyte  (line 690)
- [x] str_as_unibyte  (line 713)
- [x] string_count_byte8  (line 746, static)
- [x] string_escape_byte8  (line 776)
- [x] alphabeticp  (line 959)
- [x] alphanumericp  (line 982)
- [x] graphicp  (line 1004)
- [x] printablep  (line 1022)
- [x] graphic_base_p  (line 1037)
- [x] blankp  (line 1059)
- [x] syms_of_character  (line 1081)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 1
- [x] Lisp_Object Vchar_unify_table  (line 44)

## Decisions

- **All ten subrs are `DEFUN!`** (PORTING.md 5.1) with C's Lisp name, argument limits and
  docstring.  `characterp`, `max-char`, `char-width` and `char-resolve-modifiers` carry
  `attributes: const` in C (170, 183, 264, 885); the macro has no `attributes:` form for it
  (only `attributes: noreturn`), so the subr record does not set that flag and the four
  `attributes: const` lines have no counterpart here — recorded, the same deviation
  `callint.md` records for `Finteractive`.
- **The two `MANY` subrs** (`string`, `unibyte-string`) take C's `(n, args)` pair as
  `(nargs: isize, args: *mut Lisp_Object)`: `DEFUN_ARGS_MANY` (src/lisp.h 835-854) types the
  count `ptrdiff_t`, and the `DEFUN!` table fixes the Rust spelling of every arity arm, so
  C's `n` is spelled `nargs` (`n` is ≤2 characters and invisible to the token check anyway).
  C's `args[i]` is `*args.offset (i)`.
- **`chars_in_text`'s `current_buffer == 0`** is the port's `current_buffer.is_null ()`; the
  literal is in docs/ports/macro-body-notes.tsv (the `FRAME_LIVE_P` row's class).
- **`char_resolve_modifier_mask`'s seven octal literals** (`0377`, `0177`, `0137`, `0101`,
  `0132`, `037`, `0100`) have no Rust spelling — `0o377` tokenises as `0` + `o377` — so the
  port writes `0o377` … and the seven C tokens are recorded in
  docs/ports/macro-body-notes.tsv; every other token of the body still compares (P-15).
- **The `#if 0` block at C 91-97** ("outside the scope of this function, bug#4751") never
  compiles in C either, so it stays a comment at the same place.
- **The two `#ifdef HAVE_WINDOW_SYSTEM` regions of `lisp_string_width`** (C 361-364 and
  396-420) are dead on this target: `config.h` leaves `HAVE_WINDOW_SYSTEM` undefined
  (`HAVE_NS` too — `--without-all --with-x=no --with-ns=no`, docs/baseline/config), so
  `FRAME_WINDOW_P` is the mirror's constant `false` and the font-metrics block is
  unreachable.  Their text is kept behind `#[cfg (feature = "window_system")]`, the feature
  that names the non-frozen side (P-51), with `const _: () = assert! (! cfg! (…))` binding
  it to the frozen configuration.  `config.rs` does not carry the macro yet, which is why
  the assertion names the feature alone.
- **The `ckd_add`/`ckd_mul` pairs** are `overflowing_add`/`overflowing_mul` at every site
  (P-15, token-aliases' `ckd_add`/`ckd_mul` families): C's `if (ckd_add (&w, w, d))
  string_overflow ()` becomes the two statements the pair needs, and the short-circuit of
  C's `||` between two `ckd_add`s is kept by calling `string_overflow ()` (`-> !`) between
  them, which is also what makes the second result definitely initialized in Rust.
- **`char_width` and `string_count_byte8`** are C's two `static` helpers; both work on raw
  memory, so the port declares them `unsafe fn` (the `sort.rs` shape) and they carry no
  `#[unsafe(no_mangle)]` — C's `static` has no external symbol.
- **C's `for` loops** become `while` loops with the update at the end of the body
  (PORTING.md 2.2): `translate_char`'s list walk, `char_width`'s display-vector walk,
  `lisp_string_width`'s `for (j = 0; …) if (…) break;`, `count_size_as_multibyte`'s and
  `str_to_multibyte`'s index loops, and `string_escape_byte8`'s two `while (len--) *to++ =
  *src++;` copies.  `str_as_multibyte`'s `while (true)` is `loop`.
- **The whole-file line count is 6.3% over C's** (1167 → 1240): recorded in
  docs/ports/line-drift-tolerance.tsv with the reason (the two P-51 arms, the explicit
  pointer arithmetic C spells with `p[i]`/`*p++`, and the port's fixed costs).
- **`Vchar_unify_table` and `hexdigit` are the two file-scope objects**; both are defined
  here (P-21) so that the glob-imported declarations of the `character.rs` mirror are
  shadowed, and `hexdigit`'s designated initializer becomes a `let mut` array inside the
  `static`'s initializer because Rust has no designated initializers.
- **Mirror work this port needed** (all recorded at their C line in the mirror):
  `src/disptab.h` is a new WIP mirror (`include/disptab.rs`; `DISP_CHAR_VECTOR`,
  `DISP_TABLE_P`, the six slot accessors, `enum box` and the two `*_display_table`
  prototypes — the glyph macros at its bottom stay unchecked, P-66); `dispextern.rs` gained
  `GLYPH_CODE_P`, `FACE_ID_BITS` and `MAX_FACE_ID` (2052-2062, 562, 1986);
  `composite.rs` gained `LGSTRING_GLYPH_LEN`/`LGSTRING_GLYPH`, `get_composition_id`,
  `find_composition`, `find_automatic_composition` and `composition_gstring_width`;
  `lisp.rs` gained `validate_subarray`, `args_out_of_range`, `string_overflow`,
  `make_uninit_string`, `make_uninit_multibyte_string` and `check_integer_range`;
  `lib/include/stdio.rs` gained `sprintf`; and `character.rs`'s two prototype groups moved
  into `pub mod` + glob imports (P-39), because this unit *defines* every one of them.
- **Tool fix**: `tools/cinventory.py`'s `doc_comment_value` kept a leading newline
  (`strip (" \t")`), so a `doc: /*` whose text starts on the next line hashed to a leading
  space on the C side while `extract-abi.digest` strips it on the Rust side.  `Fstring`
  (C 847-851) is the first such subr in a ported file; make-docfile drops the leading
  whitespace, which is what the one-line fix does.  `docs/baseline/subrs.tsv`'s digest
  column for that row changes with the next `tools/gen-baseline.py` run (the column is not
  read by codegen).
- Call-relationship differences (D7): none.

## Integrator note (2026-10-04, status 3.72)

`hexdigit` is a **non-`static`** file-scope object in C (`signed char const hexdigit[UCHAR_MAX +
1]`), so C's object exports `_hexdigit`; the port's array was emitted as a mangled Rust symbol
(`__RNvCs52…_9character8hexdigit`) because it lacked `#[unsafe(no_mangle)]` - the
`re_match_object` class of status 3.60, found by comparing the port's export table against
`build/emacs-ref/src/character.o`.  Fixed; `nm -g build/femacs/character.o` now shows
`S _hexdigit`.

