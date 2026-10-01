# emacs/src/character.h (615 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 34
- [x] SINGLE_BYTE_CHAR_P  (line 97)
- [x] CHAR_BYTE8_P  (line 105)
- [x] BYTE8_TO_CHAR  (line 112)
- [x] UNIBYTE_TO_CHAR  (line 118)
- [x] CHAR_TO_BYTE8  (line 125)
- [x] CHAR_TO_BYTE_SAFE  (line 133)
- [x] CHAR_BYTE8_HEAD_P  (line 141)
- [x] make_char_multibyte  (line 148)
- [x] CHAR_VALID_P  (line 159)
- [x] CHARACTERP  (line 166)
- [x] CHECK_CHARACTER  (line 173)
- [x] CHECK_CHARACTER_CAR  (line 179)
- [x] CHECK_CHARACTER_CDR  (line 185)
- [x] CHAR_PRINTABLE_P  (line 192)
- [x] CHAR_BYTES  (line 200)
- [x] CHAR_LEADING_CODE  (line 211)
- [x] CHAR_STRING  (line 227)
- [x] BYTE8_STRING  (line 258)
- [x] LEADING_CODE_P  (line 268)
- [x] TRAILING_CODE_P  (line 276)
- [x] CHAR_HEAD_P  (line 285)
- [x] BYTES_BY_CHAR_HEAD  (line 294)
- [x] multibyte_length  (line 311)
- [x] raw_prev_char_len  (line 360)
- [x] string_char_and_length  (line 372)
- [x] STRING_CHAR  (line 414)
- [x] string_char_advance  (line 424)
- [x] fetch_string_char_advance  (line 438)
- [x] fetch_string_char_as_multibyte_advance  (line 464)
- [x] fetch_string_char_advance_no_check  (line 490)
- [x] CHAR_VARIATION_SELECTOR_P  (line 506)
- [x] char_surrogate_p  (line 518)
- [x] char_table_translate  (line 592)
- [x] char_hexdigit  (line 608)

## Macros defined — 3
- [x] EMACS_CHARACTER_H  (line 24, body da39a3ee5e6b)
- [x] uLSQM  (line 92, body 4df33707bcd8)
- [x] uRSQM  (line 93, body ee2c1ef48a56)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern Lisp_Object Vchar_unify_table  (line 578)

## Decisions

`emacs/src/character.h` @ `fd7afced403`; port `femacs/src/include/character.rs`; the gate is
`python3 tools/extract-abi.py --check emacs/src/character.h femacs/src/include/character.rs` -> `OK`.

- `#if` branches: none in this header.
- Layout: no structs here; the constants and the 34 inline functions keep C's bodies
  (`eassert`/`eassume` and the accessors come from `lisp.rs`, as from `lisp.h` in C).
- `uLSQM`/`uRSQM` are C string macros written with byte escapes; the port spells the same
  code point (`"\u{2018}"`) and `--check` compares string-valued constants by their
  decoded bytes (tools/extract-abi.py, `decode_c_string`).
- `hexdigit` is `extern signed char const hexdigit[]` in C; the port declares
  `[c_char; 256]`, the size of the definition `hexdigit[UCHAR_MAX + 1]` in `character.c`
  (`UCHAR_MAX` lives in the WIP `lib/include/limits.in.rs` mirror, P-26).
- `unicode_category_t` keeps C's member names and its explicit `= 0`; the unnamed enums
  (`MAX_CHAR`, the Unicode values, `MAX_MULTIBYTE_LENGTH`) become constants with the same
  names and literals.
- `char_hexdigit` reads the extern array inside an `unsafe` block (C indexes it plainly).
- Call-relationship differences (D7): none; the parameter names in the `extern` block come
  from `character.c`'s definitions.
