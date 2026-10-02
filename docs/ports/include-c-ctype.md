# emacs/lib/c-ctype.h (366 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 15
- [x] c_isalnum  (line 169)
- [x] c_isalpha  (line 183)
- [ ] c_isascii  (line 198)
- [ ] c_isblank  (line 215)
- [ ] c_iscntrl  (line 221)
- [x] c_isdigit  (line 233)
- [ ] c_isgraph  (line 245)
- [ ] c_islower  (line 260)
- [ ] c_isprint  (line 272)
- [ ] c_ispunct  (line 288)
- [ ] c_isspace  (line 300)
- [ ] c_isupper  (line 312)
- [ ] c_isxdigit  (line 324)
- [ ] c_tolower  (line 337)
- [ ] c_toupper  (line 349)

## Macros defined — 12
- [x] C_CTYPE_ASCII  (line 74, body 356a192b7913)
- [x] C_CTYPE_H  (line 24, body da39a3ee5e6b)
- [ ] C_CTYPE_INLINE  (line 33, body a960df3ab827)
- [ ] _C_CTYPE_A_THRU_F  (line 131, body 0ec0844ac38d)
- [ ] _C_CTYPE_CNTRL  (line 87, body 0a278799ffbe)
- [x] _C_CTYPE_DIGIT  (line 134, body db6476d5c221)
- [x] _C_CTYPE_LOWER  (line 138, body a832eced7a79)
- [ ] _C_CTYPE_LOWER_A_THRU_F_N  (line 118, body f6c0d077c421)
- [ ] _C_CTYPE_LOWER_N  (line 121, body 17bab3488a7f)
- [ ] _C_CTYPE_OTHER_CNTRL  (line 106, body 31e463ad89d1)
- [ ] _C_CTYPE_PUNCT  (line 139, body 21c00e9e0e06)
- [x] _C_CTYPE_UPPER  (line 148, body ea43f23f7ac0)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/c-ctype.h` @ `fd7afced403`; port `femacs/lib/include/c-ctype.rs`.

- **WIP**: the port carries the ASCII arm's three case-macro sets and the three predicates
  filevercmp.c needs (`c_isalpha`/`c_isalnum`/`c_isdigit`); the remaining predicates and the
  `c_to*` pair are unchecked, as is `C_CTYPE_INLINE`'s `_GL_INLINE` marker (the wrapper unit
  `femacs/lib/c-ctype.rs` carries `_GL_CTYPE_INLINE`).
- The frozen target takes the `C_CTYPE_ASCII` arm, so the EBCDIC case lists and the two
  `#error` arms are not carried (P-24 does not compare a macro that has several definitions).
- `--check` reports the unimplemented functions; the ledger's unchecked boxes are what keeps
  the pair at WIP rather than failing.
