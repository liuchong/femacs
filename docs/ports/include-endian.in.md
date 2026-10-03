# emacs/lib/endian.in.h (301 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 12
- [x] be16toh  (line 125)
- [x] be32toh  (line 138)
- [x] be64toh  (line 152)
- [x] htobe16  (line 168)
- [x] htobe32  (line 181)
- [x] htobe64  (line 195)
- [x] le16toh  (line 211)
- [x] le32toh  (line 224)
- [x] le64toh  (line 238)
- [x] htole16  (line 254)
- [x] htole32  (line 267)
- [x] htole64  (line 281)

## Macros defined — 30
- [x] BIG_ENDIAN  (line 67, body d5f12e53a182)
- [x] BYTE_ORDER  (line 73, body be4c98d225b1)
- [x] GNULIB_defined_be16toh  (line 133, body 356a192b7913)
- [x] GNULIB_defined_be32toh  (line 146, body 356a192b7913)
- [x] GNULIB_defined_be64toh  (line 160, body 356a192b7913)
- [x] GNULIB_defined_htobe16  (line 176, body 356a192b7913)
- [x] GNULIB_defined_htobe32  (line 189, body 356a192b7913)
- [x] GNULIB_defined_htobe64  (line 203, body 356a192b7913)
- [x] GNULIB_defined_htole16  (line 262, body 356a192b7913)
- [x] GNULIB_defined_htole32  (line 275, body 356a192b7913)
- [x] GNULIB_defined_htole64  (line 289, body 356a192b7913)
- [x] GNULIB_defined_le16toh  (line 219, body 356a192b7913)
- [x] GNULIB_defined_le32toh  (line 232, body 356a192b7913)
- [x] GNULIB_defined_le64toh  (line 246, body 356a192b7913)
- [x] LITTLE_ENDIAN  (line 66, body 7110eda4d09e)
- [x] PDP_ENDIAN  (line 68, body 07b6e84910d4)
- [x] _  (line 47, body c28d1ed366ea)
- [x] _GL_ENDIAN_INLINE  (line 63, body a960df3ab827)
- [x] be16toh  (line 93, body 46998b0b8abb)
- [x] be32toh  (line 94, body 94b306dbcddf)
- [x] be64toh  (line 95, body 69ded6fd55a8)
- [x] htobe16  (line 96, body db94ab5fa0e8)
- [x] htobe32  (line 97, body 9d15da9ff4f3)
- [x] htobe64  (line 98, body 45d3196b7721)
- [x] htole16  (line 102, body 2e53487f7eff)
- [x] htole32  (line 103, body 00b521a38785)
- [x] htole64  (line 104, body 1a75b726122e)
- [x] le16toh  (line 99, body 2c8f7c1a9e61)
- [x] le32toh  (line 100, body 333bb5650fd3)
- [x] le64toh  (line 101, body 5fa87f497884)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **PORTING.md 0.1**: the mirror is `femacs/lib/include/endian.in.rs`; the generated
  header (build/emacs-ref/lib/endian.h) substitutes `_GL_ENDIAN_H`, `HAVE_ENDIAN_H` = 0,
  `HAVE_SYS_ENDIAN_H` = 1 and `ENDIAN_H_JUST_MISSING_STDINT` = 0, so gnulib's own arm is
  the live one and the twelve identifiers are `#define`d to `rpl_endian_*` - which is why
  the definitions keep C's spelling and the unit (lib/endian.rs) exports the renamed
  symbols (P-81); the renames are one row of docs/ports/macro-body-notes.tsv.
- `_GL_ENDIAN_INLINE` is in docs/ports/excluded-macros.txt (kind=syntax); the twelve
  `GNULIB_defined_*` markers are `pub const … : c_int = 1;`.
