# emacs/lib/c-ctype.h (366 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 15
- [x] c_isalnum  (line 169)
- [x] c_isalpha  (line 183)
- [x] c_isascii  (line 198)
- [x] c_isblank  (line 215)
- [x] c_iscntrl  (line 221)
- [x] c_isdigit  (line 233)
- [x] c_isgraph  (line 245)
- [x] c_islower  (line 260)
- [x] c_isprint  (line 272)
- [x] c_ispunct  (line 288)
- [x] c_isspace  (line 300)
- [x] c_isupper  (line 312)
- [x] c_isxdigit  (line 324)
- [x] c_tolower  (line 337)
- [x] c_toupper  (line 349)

## Macros defined — 12
- [x] C_CTYPE_ASCII  (line 74, body 356a192b7913)
- [x] C_CTYPE_H  (line 24, body da39a3ee5e6b)
- [x] C_CTYPE_INLINE  (line 33, body a960df3ab827)
- [x] _C_CTYPE_A_THRU_F  (line 131, body 0ec0844ac38d)
- [x] _C_CTYPE_CNTRL  (line 87, body 0a278799ffbe)
- [x] _C_CTYPE_DIGIT  (line 134, body db6476d5c221)
- [x] _C_CTYPE_LOWER  (line 138, body a832eced7a79)
- [x] _C_CTYPE_LOWER_A_THRU_F_N  (line 118, body f6c0d077c421)
- [x] _C_CTYPE_LOWER_N  (line 121, body 17bab3488a7f)
- [x] _C_CTYPE_OTHER_CNTRL  (line 106, body 31e463ad89d1)
- [x] _C_CTYPE_PUNCT  (line 139, body 21c00e9e0e06)
- [x] _C_CTYPE_UPPER  (line 148, body ea43f23f7ac0)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/c-ctype.h` @ `fd7afced403`; port `lib/include/c-ctype.rs`.

- **WIP**: the port carries the ASCII arm's three case-macro sets and the three predicates
  filevercmp.c needs (`c_isalpha`/`c_isalnum`/`c_isdigit`); the remaining predicates and the
  `c_to*` pair are unchecked, as is `C_CTYPE_INLINE`'s `_GL_INLINE` marker (the wrapper unit
  `lib/c-ctype.rs` carries `_GL_CTYPE_INLINE`).
- The frozen target takes the `C_CTYPE_ASCII` arm, so the EBCDIC case lists and the two
  `#error` arms are not carried (P-24 does not compare a macro that has several definitions).
- `--check` reports the unimplemented functions; the ledger's unchecked boxes are what keeps
  the pair at WIP rather than failing.

## Decisions (stale-box sweep, 2026-10-04)

The 19 unchecked box(es) above were **stale bookkeeping, not missing port work**: every
name they list is already satisfied, measured on this worktree with
`python3 tools/extract-abi.py --check emacs/lib/c-ctype.h lib/include/c-ctype.rs` -> **0 `- ` problem
lines** (`lib/include/c-ctype.rs` carries the names; `lib/include/` and `src/include/` mirrors are the ones the
tools compare here).  Two of `c-ctype.h`'s names are satisfied by an exclusion rather than a
definition - `_C_CTYPE_LOWER_A_THRU_F_N` and `_C_CTYPE_LOWER_N` own rows in
`docs/ports/excluded-macros.txt` ("c-ctype.h 126-129's parameterised case family: a Rust
pattern cannot add an offset to a literal ... this family has no counterpart") - and
`widgetprv.h`'s `<anon@27>` is an anonymous aggregate the container check skips by design
("anonymous aggregates without an alias skipped - they have no name the two sides could agree
on").  No mirror text changed in this sweep; only the boxes were ticked.
