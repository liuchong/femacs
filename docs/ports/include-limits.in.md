# emacs/lib/limits.in.h (151 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 28
- [ ] BOOL_MAX  (line 133, body 356a192b7913)
- [ ] BOOL_WIDTH  (line 130, body 356a192b7913)
- [ ] CHAR_WIDTH  (line 112, body c91c491e54d6)
- [ ] INT_WIDTH  (line 117, body df6cb1beed2c)
- [ ] LLONG_MAX  (line 62, body 1a916b81d003)
- [ ] LLONG_MIN  (line 53, body 85c2c80cd328)
- [ ] LLONG_WIDTH  (line 121, body 7b8b518ebea4)
- [ ] LONG_BIT  (line 98, body c66c65175fec)
- [ ] LONG_WIDTH  (line 119, body 915e12b91f4e)
- [ ] MB_LEN_MAX  (line 104, body 1574bddb75c7)
- [ ] SCHAR_WIDTH  (line 113, body bd79bc2c3a55)
- [ ] SHRT_WIDTH  (line 115, body f135b7f02e36)
- [ ] SSIZE_MAX  (line 145, body f5c01bcbc752)
- [ ] UCHAR_WIDTH  (line 114, body b2bac1efaa85)
- [ ] UINT_WIDTH  (line 118, body 6d5d59c996c8)
- [ ] ULLONG_MAX  (line 71, body 01d100f1e347)
- [ ] ULLONG_WIDTH  (line 122, body ad3b3ae9220e)
- [ ] ULONG_WIDTH  (line 120, body 0e0b44b7ebd5)
- [ ] USHRT_WIDTH  (line 116, body ce2f65e126aa)
- [ ] WORD_BIT  (line 91, body cb4e5208b4cd)
- [ ] _  (line 45, body cc2c62e673e9)
- [ ] _GL_COB128  (line 82, body e46b9f95e123)
- [ ] _GL_COB16  (line 85, body e769a204b41a)
- [ ] _GL_COB32  (line 84, body 5a2f3796fcdb)
- [ ] _GL_COB4  (line 87, body 795c8c573e42)
- [ ] _GL_COB64  (line 83, body 9aeac64b1c0e)
- [ ] _GL_COB8  (line 86, body 854f7f3b7ba8)
- [ ] _GL_INTEGER_WIDTH  (line 81, body 261b6c61c5f3)

## Types and their fields — 0

## File-scope objects — 0

## Decisions (WIP, PORTING.md P-26)

`emacs/lib/limits.in.h` @ `fd7afced403`; port `femacs/lib/include/limits.in.rs`.

Deliberately partial, like `stdint.in.rs`: it carries the integer-width machinery
(`_GL_INTEGER_WIDTH` and the `_GL_COB*` helpers, `limits.in.h` 73-87) and the one value a
ported header names (`UCHAR_MAX`, which sizes `character.c`'s `hexdigit[]`).  The
unchecked boxes are the rest of the header.
