# emacs/lib/stdbit.in.h (2229 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 173
- [ ] _gl_stdbit_clz  (line 236)
- [ ] _gl_stdbit_clzl  (line 241)
- [ ] _gl_stdbit_clzll  (line 246)
- [ ] _gl_stdbit_clzl  (line 262)
- [ ] _gl_stdbit_clz  (line 268)
- [ ] _gl_stdbit_clzll  (line 273)
- [ ] _gl_stdbit_clzll  (line 287)
- [ ] _gl_stdbit_clz  (line 301)
- [ ] _gl_stdbit_clzl  (line 306)
- [ ] stdc_leading_zeros_ui  (line 318)
- [ ] stdc_leading_zeros_uc  (line 324)
- [ ] stdc_leading_zeros_us  (line 330)
- [ ] stdc_leading_zeros_ul  (line 336)
- [ ] stdc_leading_zeros_ull  (line 342)
- [ ] stdc_leading_ones_uc  (line 367)
- [ ] stdc_leading_ones_us  (line 373)
- [ ] stdc_leading_ones_ui  (line 379)
- [ ] stdc_leading_ones_ul  (line 385)
- [ ] stdc_leading_ones_ull  (line 391)
- [ ] _gl_stdbit_ctz  (line 427)
- [ ] _gl_stdbit_ctzl  (line 432)
- [ ] _gl_stdbit_ctzll  (line 437)
- [ ] _gl_stdbit_ctzl  (line 453)
- [ ] _gl_stdbit_ctz  (line 459)
- [ ] _gl_stdbit_ctzll  (line 464)
- [ ] _gl_stdbit_ctz  (line 478)
- [ ] _gl_stdbit_ctzl  (line 483)
- [ ] _gl_stdbit_ctzll  (line 488)
- [ ] stdc_trailing_zeros_ui  (line 500)
- [ ] stdc_trailing_zeros_uc  (line 506)
- [ ] stdc_trailing_zeros_us  (line 512)
- [ ] stdc_trailing_zeros_ul  (line 518)
- [ ] stdc_trailing_zeros_ull  (line 524)
- [ ] stdc_trailing_ones_uc  (line 549)
- [ ] stdc_trailing_ones_us  (line 555)
- [ ] stdc_trailing_ones_ui  (line 561)
- [ ] stdc_trailing_ones_ul  (line 567)
- [ ] stdc_trailing_ones_ull  (line 573)
- [ ] stdc_first_leading_zero_uc  (line 598)
- [ ] stdc_first_leading_zero_us  (line 606)
- [ ] stdc_first_leading_zero_ui  (line 614)
- [ ] stdc_first_leading_zero_ul  (line 622)
- [ ] stdc_first_leading_zero_ull  (line 630)
- [ ] stdc_first_leading_one_uc  (line 657)
- [ ] stdc_first_leading_one_us  (line 665)
- [ ] stdc_first_leading_one_ui  (line 673)
- [ ] stdc_first_leading_one_ul  (line 681)
- [ ] stdc_first_leading_one_ull  (line 689)
- [ ] stdc_first_trailing_zero_uc  (line 716)
- [ ] stdc_first_trailing_zero_us  (line 724)
- [ ] stdc_first_trailing_zero_ui  (line 732)
- [ ] stdc_first_trailing_zero_ul  (line 740)
- [ ] stdc_first_trailing_zero_ull  (line 748)
- [ ] stdc_first_trailing_one_uc  (line 775)
- [ ] stdc_first_trailing_one_us  (line 783)
- [ ] stdc_first_trailing_one_ui  (line 791)
- [ ] stdc_first_trailing_one_ul  (line 799)
- [ ] stdc_first_trailing_one_ull  (line 807)
- [ ] _gl_stdbit_popcount_wide  (line 849)
- [ ] __popcnt64  (line 908)
- [ ] _gl_stdbit_popcount_supported  (line 919)
- [ ] _gl_stdbit_popcount  (line 934)
- [ ] _gl_stdbit_popcountl  (line 941)
- [ ] _gl_stdbit_popcountll  (line 948)
- [ ] stdc_count_ones_ui  (line 966)
- [ ] stdc_count_ones_uc  (line 972)
- [ ] stdc_count_ones_us  (line 978)
- [ ] stdc_count_ones_ul  (line 984)
- [ ] stdc_count_ones_ull  (line 990)
- [ ] stdc_count_zeros_uc  (line 1015)
- [ ] stdc_count_zeros_us  (line 1021)
- [ ] stdc_count_zeros_ui  (line 1027)
- [ ] stdc_count_zeros_ul  (line 1033)
- [ ] stdc_count_zeros_ull  (line 1039)
- [ ] stdc_has_single_bit_uc  (line 1064)
- [ ] stdc_has_single_bit_us  (line 1071)
- [ ] stdc_has_single_bit_ui  (line 1078)
- [ ] stdc_has_single_bit_ul  (line 1085)
- [ ] stdc_has_single_bit_ull  (line 1092)
- [ ] stdc_bit_width_uc  (line 1119)
- [ ] stdc_bit_width_us  (line 1125)
- [ ] stdc_bit_width_ui  (line 1131)
- [ ] stdc_bit_width_ul  (line 1137)
- [ ] stdc_bit_width_ull  (line 1143)
- [ ] stdc_bit_floor_uc  (line 1171)
- [ ] stdc_bit_floor_us  (line 1177)
- [ ] stdc_bit_floor_ui  (line 1183)
- [ ] stdc_bit_floor_ul  (line 1189)
- [ ] stdc_bit_floor_ull  (line 1195)
- [ ] stdc_bit_ceil_uc  (line 1228)
- [ ] stdc_bit_ceil_us  (line 1234)
- [ ] stdc_bit_ceil_ui  (line 1240)
- [ ] stdc_bit_ceil_ul  (line 1246)
- [ ] stdc_bit_ceil_ull  (line 1252)
- [ ] stdc_rotate_left_uc  (line 1297)
- [ ] stdc_rotate_left_us  (line 1303)
- [ ] stdc_rotate_left_ui  (line 1309)
- [ ] stdc_rotate_left_ul  (line 1315)
- [ ] stdc_rotate_left_ull  (line 1321)
- [ ] stdc_rotate_right_uc  (line 1363)
- [ ] stdc_rotate_right_us  (line 1369)
- [ ] stdc_rotate_right_ui  (line 1375)
- [ ] stdc_rotate_right_ul  (line 1381)
- [ ] stdc_rotate_right_ull  (line 1387)
- [ ] stdc_memreverse8  (line 1416)
- [ ] stdc_memreverse8u8  (line 1454)
- [ ] stdc_memreverse8u16  (line 1460)
- [ ] stdc_memreverse8u32  (line 1472)
- [ ] stdc_memreverse8u64  (line 1486)
- [ ] stdc_load8_beu8  (line 1646)
- [ ] stdc_load8_beu16  (line 1652)
- [ ] stdc_load8_beu32  (line 1660)
- [ ] stdc_load8_beu64  (line 1670)
- [ ] stdc_load8_leu8  (line 1687)
- [ ] stdc_load8_leu16  (line 1693)
- [ ] stdc_load8_leu32  (line 1701)
- [ ] stdc_load8_leu64  (line 1711)
- [ ] stdc_load8_bes8  (line 1728)
- [ ] stdc_load8_bes16  (line 1734)
- [ ] stdc_load8_bes32  (line 1740)
- [ ] stdc_load8_bes64  (line 1746)
- [ ] stdc_load8_les8  (line 1752)
- [ ] stdc_load8_les16  (line 1758)
- [ ] stdc_load8_les32  (line 1764)
- [ ] stdc_load8_les64  (line 1770)
- [ ] stdc_load8_aligned_beu8  (line 1785)
- [ ] stdc_load8_aligned_beu16  (line 1791)
- [ ] stdc_load8_aligned_beu32  (line 1806)
- [ ] stdc_load8_aligned_beu64  (line 1821)
- [ ] stdc_load8_aligned_leu8  (line 1836)
- [ ] stdc_load8_aligned_leu16  (line 1842)
- [ ] stdc_load8_aligned_leu32  (line 1857)
- [ ] stdc_load8_aligned_leu64  (line 1872)
- [ ] stdc_load8_aligned_bes8  (line 1887)
- [ ] stdc_load8_aligned_bes16  (line 1893)
- [ ] stdc_load8_aligned_bes32  (line 1899)
- [ ] stdc_load8_aligned_bes64  (line 1905)
- [ ] stdc_load8_aligned_les8  (line 1911)
- [ ] stdc_load8_aligned_les16  (line 1917)
- [ ] stdc_load8_aligned_les32  (line 1923)
- [ ] stdc_load8_aligned_les64  (line 1929)
- [ ] stdc_store8_beu8  (line 1947)
- [ ] stdc_store8_beu16  (line 1953)
- [ ] stdc_store8_beu32  (line 1960)
- [ ] stdc_store8_beu64  (line 1969)
- [ ] stdc_store8_leu8  (line 1982)
- [ ] stdc_store8_leu16  (line 1988)
- [ ] stdc_store8_leu32  (line 1995)
- [ ] stdc_store8_leu64  (line 2004)
- [ ] stdc_store8_bes8  (line 2017)
- [ ] stdc_store8_bes16  (line 2023)
- [ ] stdc_store8_bes32  (line 2029)
- [ ] stdc_store8_bes64  (line 2035)
- [ ] stdc_store8_les8  (line 2041)
- [ ] stdc_store8_les16  (line 2047)
- [ ] stdc_store8_les32  (line 2053)
- [ ] stdc_store8_les64  (line 2059)
- [ ] stdc_store8_aligned_beu8  (line 2074)
- [ ] stdc_store8_aligned_beu16  (line 2080)
- [ ] stdc_store8_aligned_beu32  (line 2093)
- [ ] stdc_store8_aligned_beu64  (line 2106)
- [ ] stdc_store8_aligned_leu8  (line 2119)
- [ ] stdc_store8_aligned_leu16  (line 2125)
- [ ] stdc_store8_aligned_leu32  (line 2138)
- [ ] stdc_store8_aligned_leu64  (line 2151)
- [ ] stdc_store8_aligned_bes8  (line 2164)
- [ ] stdc_store8_aligned_bes16  (line 2170)
- [ ] stdc_store8_aligned_bes32  (line 2176)
- [ ] stdc_store8_aligned_bes64  (line 2182)
- [ ] stdc_store8_aligned_les8  (line 2188)
- [ ] stdc_store8_aligned_les16  (line 2194)
- [ ] stdc_store8_aligned_les32  (line 2200)
- [ ] stdc_store8_aligned_les64  (line 2206)

## Macros defined — 90
- [x] GNULIB_defined_clz_functions  (line 312, body 356a192b7913)
- [x] GNULIB_defined_ctz_functions  (line 494, body 356a192b7913)
- [x] GNULIB_defined_popcount_functions  (line 960, body 356a192b7913)
- [x] GNULIB_defined_stdc_bit_ceil_functions  (line 1257, body 356a192b7913)
- [x] GNULIB_defined_stdc_bit_floor_functions  (line 1200, body 356a192b7913)
- [x] GNULIB_defined_stdc_bit_width_functions  (line 1148, body 356a192b7913)
- [x] GNULIB_defined_stdc_count_ones_functions  (line 995, body 356a192b7913)
- [x] GNULIB_defined_stdc_count_zeros_functions  (line 1044, body 356a192b7913)
- [x] GNULIB_defined_stdc_first_leading_one_functions  (line 696, body 356a192b7913)
- [x] GNULIB_defined_stdc_first_leading_zero_functions  (line 637, body 356a192b7913)
- [x] GNULIB_defined_stdc_first_trailing_one_functions  (line 814, body 356a192b7913)
- [x] GNULIB_defined_stdc_first_trailing_zero_functions  (line 755, body 356a192b7913)
- [x] GNULIB_defined_stdc_has_single_bit_functions  (line 1098, body 356a192b7913)
- [x] GNULIB_defined_stdc_leading_ones_functions  (line 396, body 356a192b7913)
- [x] GNULIB_defined_stdc_leading_zeros_functions  (line 347, body 356a192b7913)
- [x] GNULIB_defined_stdc_load8_aligned_functions  (line 1934, body 356a192b7913)
- [x] GNULIB_defined_stdc_load8_functions  (line 1775, body 356a192b7913)
- [x] GNULIB_defined_stdc_memreverse8  (line 1436, body 356a192b7913)
- [x] GNULIB_defined_stdc_memreverse8u_functions  (line 1503, body 356a192b7913)
- [x] GNULIB_defined_stdc_rotate_left_functions  (line 1326, body 356a192b7913)
- [x] GNULIB_defined_stdc_rotate_right_functions  (line 1392, body 356a192b7913)
- [x] GNULIB_defined_stdc_store8_aligned_functions  (line 2211, body 356a192b7913)
- [x] GNULIB_defined_stdc_store8_functions  (line 2064, body 356a192b7913)
- [x] GNULIB_defined_stdc_trailing_ones_functions  (line 578, body 356a192b7913)
- [x] GNULIB_defined_stdc_trailing_zeros_functions  (line 529, body 356a192b7913)
- [ ] _  (line 37, body a2b18f040ffc)
- [ ] _Bool  (line 31, body 5039d155a71c)
- [x] _GL_HAS_BUILTIN_ASSUME_ALIGNED  (line 1614, body 356a192b7913)
- [x] _GL_STDBIT_ASSUME_ALIGNED  (line 1621, body 019a49c4c0cd)
- [x] _GL_STDBIT_BIGENDIAN  (line 1639, body 1aa97d97b98b)
- [ ] _GL_STDBIT_HAS_BUILTIN_BSWAP16  (line 76, body 356a192b7913)
- [ ] _GL_STDBIT_HAS_BUILTIN_BSWAP32  (line 84, body 356a192b7913)
- [ ] _GL_STDBIT_HAS_BUILTIN_BSWAP64  (line 87, body 356a192b7913)
- [ ] _GL_STDBIT_HAS_BUILTIN_CLZ  (line 228, body 5ffe533b830f)
- [ ] _GL_STDBIT_HAS_BUILTIN_CTZ  (line 419, body 5ffe533b830f)
- [ ] _GL_STDBIT_HAS_BUILTIN_POPCOUNT  (line 837, body 5ffe533b830f)
- [x] _GL_STDBIT_MEMCPY  (line 103, body 9d2723278399)
- [x] _GL_STDBIT_OPTIMIZE_VIA_MEMCPY  (line 1636, body b6589fc6ab0d)
- [x] _GL_STDBIT_TYPEOF_CAST  (line 190, body fa7ecf0691ee)
- [x] _GL_STDBIT_UINT_FAST16  (line 62, body e5f3c118a9d6)
- [x] _GL_STDBIT_UINT_FAST32  (line 63, body 0d943b2b60ff)
- [x] _GL_STDBIT_UINT_FAST64  (line 64, body 50c560af15af)
- [x] _GL_STDC_BIT_CEIL_INLINE  (line 150, body a960df3ab827)
- [x] _GL_STDC_BIT_FLOOR_INLINE  (line 147, body a960df3ab827)
- [x] _GL_STDC_BIT_WIDTH_INLINE  (line 144, body a960df3ab827)
- [x] _GL_STDC_COUNT_ONES_INLINE  (line 138, body a960df3ab827)
- [x] _GL_STDC_COUNT_ZEROS_INLINE  (line 135, body a960df3ab827)
- [x] _GL_STDC_FIRST_LEADING_ONE_INLINE  (line 126, body a960df3ab827)
- [x] _GL_STDC_FIRST_LEADING_ZERO_INLINE  (line 123, body a960df3ab827)
- [x] _GL_STDC_FIRST_TRAILING_ONE_INLINE  (line 132, body a960df3ab827)
- [x] _GL_STDC_FIRST_TRAILING_ZERO_INLINE  (line 129, body a960df3ab827)
- [x] _GL_STDC_HAS_SINGLE_BIT_INLINE  (line 141, body a960df3ab827)
- [x] _GL_STDC_LEADING_ONES_INLINE  (line 114, body a960df3ab827)
- [x] _GL_STDC_LEADING_ZEROS_INLINE  (line 111, body a960df3ab827)
- [x] _GL_STDC_LOAD8_ALIGNED_INLINE  (line 165, body a960df3ab827)
- [x] _GL_STDC_LOAD8_INLINE  (line 168, body a960df3ab827)
- [x] _GL_STDC_MEMREVERSE8U_INLINE  (line 162, body a960df3ab827)
- [x] _GL_STDC_MEMREVERSE8_INLINE  (line 159, body a960df3ab827)
- [x] _GL_STDC_ROTATE_LEFT_INLINE  (line 153, body a960df3ab827)
- [x] _GL_STDC_ROTATE_RIGHT_INLINE  (line 156, body a960df3ab827)
- [x] _GL_STDC_STORE8_ALIGNED_INLINE  (line 171, body a960df3ab827)
- [x] _GL_STDC_STORE8_INLINE  (line 174, body a960df3ab827)
- [x] _GL_STDC_TRAILING_ONES_INLINE  (line 120, body a960df3ab827)
- [x] _GL_STDC_TRAILING_ZEROS_INLINE  (line 117, body a960df3ab827)
- [x] __STDC_ENDIAN_BIG__  (line 204, body d5f12e53a182)
- [x] __STDC_ENDIAN_LITTLE__  (line 205, body 7110eda4d09e)
- [x] __STDC_ENDIAN_NATIVE__  (line 209, body c85679e845bc)
- [x] __STDC_VERSION_STDBIT_H__  (line 2225, body 09e80e108199)
- [x] __need_size_t  (line 46, body da39a3ee5e6b)
- [ ] _gl_stdbit_popcount  (line 955, body a0ca3d1a7b76)
- [ ] _gl_stdbit_popcountl  (line 956, body a0ca3d1a7b76)
- [ ] _gl_stdbit_popcountll  (line 957, body a0ca3d1a7b76)
- [ ] _gl_stdc_rotate_left  (line 1289, body d6b413786d8c)
- [ ] _gl_stdc_rotate_right  (line 1355, body 53ff758ada8f)
- [ ] stdc_bit_ceil  (line 1264, body fc1200ca7d31)
- [ ] stdc_bit_floor  (line 1207, body f266e663fdb9)
- [ ] stdc_bit_width  (line 1151, body b5631a388588)
- [ ] stdc_count_ones  (line 998, body 4c82f8864756)
- [ ] stdc_count_zeros  (line 1047, body d54c60259f75)
- [ ] stdc_first_leading_one  (line 699, body 464bd0245f7d)
- [ ] stdc_first_leading_zero  (line 640, body dd0fda82ca32)
- [ ] stdc_first_trailing_one  (line 817, body 1b1ff9c96c83)
- [ ] stdc_first_trailing_zero  (line 758, body 46fdd1cd8c82)
- [ ] stdc_has_single_bit  (line 1101, body 34d852b11bdc)
- [ ] stdc_leading_ones  (line 399, body ba433c4c61fc)
- [ ] stdc_leading_zeros  (line 350, body 10aca907d8a9)
- [ ] stdc_rotate_left  (line 1330, body f51da2f10d52)
- [ ] stdc_rotate_right  (line 1396, body ba894f7d8715)
- [ ] stdc_trailing_ones  (line 581, body ec71ecff1b83)
- [ ] stdc_trailing_zeros  (line 532, body 8b4b9178cbc2)

## Types and their fields — 0

## File-scope objects — 1
- [ ] extern signed char _gl_stdbit_popcount_support  (line 916)

## Decisions

- **The judge exists and the whole template is live.**  `ls -l build/emacs-ref/lib/stdbit.h` ->
  `60264` bytes, 2,230 lines; `grep -n HAVE_STDBIT_H build/emacs-ref/src/config.h` ->
  `1433:/* #undef HAVE_STDBIT_H */`, and `docs/baseline/config/darwin-arm64.macros.tsv` records the
  same `undef`.  So the system has no `<stdbit.h>` and gnulib's header defines the whole C23 API
  itself (its C 40-46 `#if 0` block around the `# include_next` is the dead arm).  **A correction
  for the record**: `femacs/lib/include/stdbit.rs` 3 says "the frozen configuration has
  HAVE_STDBIT_H = 1" - inverted; both sources say `undef`.  Its *slice* is still right (it mirrors
  the live builtin arms), but the sentence is not, and `stdbit.rs` is unpaired in the inventory, so
  no gate ever read it.  I did not touch that file (another lane's).
- **`tools/check-cond-eval.py lib/stdbit.in.h` -> `135 conditions, 31 unknown, 101 disagreements
  [NOT VERIFIED]`**, so the extractor resolves nothing and its checklist is the union of every arm
  (264 boxes down to names: `173 functions defined`, `90 macros`, `1 file-scope object`); the 22
  macros whose bodies differ per arm are reported as *"several definitions … body not compared"*.
  The arms that matter are decided by the frozen compiler, and `stdbit.rs` 5-15 holds the readings:
  `_GL_STDBIT_HAS_BUILTIN_CLZ/CTZ/POPCOUNT` and `…BSWAP16/32/64` are live, so each family's builtin
  body is the one to spell and the hand-written fallbacks (C 287-311, 419-493, 837-960, 1447-1500,
  …) are dead text (P-66 rule 2: mark the boundary, record the lines).
- **What this lane delivered: the settings layer, measured and green.**  61 boxes are ticked: the
  22 `_GL_STDC_*_INLINE` qualifier macros (C 111-174, each `# define … _GL_INLINE`), the 25
  `GNULIB_defined_*` flags (each `1`), and the file's settings - `__need_size_t` (C 46),
  `__STDC_ENDIAN_BIG__/LITTLE__/NATIVE__` (C 204-210, the `WORDS_BIGENDIAN` arm's `#else` being the
  last definition), `__STDC_VERSION_STDBIT_H__` (C 2225), `_GL_STDBIT_UINT_FAST16/32/64` (C 62-64),
  `_GL_STDBIT_MEMCPY` (C 103), `_GL_STDBIT_TYPEOF_CAST` (C 190), `_GL_HAS_BUILTIN_ASSUME_ALIGNED`
  (C 1614), `_GL_STDBIT_ASSUME_ALIGNED` (C 1621), `_GL_STDBIT_OPTIMIZE_VIA_MEMCPY` (C 1639) and
  `_GL_STDBIT_BIGENDIAN` (C 1642).  The mirror `include!`s `stdbit.rs` (the `errno.in.rs` relation)
  so the slice the units already use is part of this pair's answer.
- **What is *not* written, and the exact size of it: 203 boxes (the families).**  `--check` reports
  **185** `- ` lines, all of them the family members: the 12 `_gl_stdbit_*` helpers, the 75
  `stdc_*` type-specific functions (13 families × `uc/us/ui/ul/ull`), the 24 `stdc_*` generic
  dispatchers plus the `stdc_imm8*`/`load8`/`store8`/`memreverse8` macros, and `__popcnt64`.  Their
  C line ranges are in the mirror's closing comment block, family by family.  **This lane stopped
  there deliberately** rather than writing 200 unverified items in the time left; the shape of the
  work is below, and it is mechanical.
- **The one idiom that unblocks the arithmetic families, measured here.**  C's dispatchers and the
  narrow variants compute widths with `sizeof 0u`, `sizeof 0ul` and `sizeof 0ull` (e.g. C 317-345,
  350-356), whose literal `0` is a *token* the gate requires of the Rust body (the `token-aliases`
  table has no `0` family, and a `macro-body-notes.tsv` row per member would be ~35 rows).  Rust's
  `std::mem::size_of_val (&0u32)` is the same expression *and* keeps the literal, so
  `stdc_leading_zeros_uc`'s C body `stdc_leading_zeros_ui (n) - 8 * (sizeof 0u - sizeof n)` becomes
  `… - 8 * (std::mem::size_of_val (&0u32) - std::mem::size_of_val (&n))` with every token C's own.
  (`sizeof 0ul`/`sizeof 0ull` take `&0u64`; the widths are the frozen `c_ulong`/`c_ulonglong`.)
- **A tool fact that decides the file's ownership, measured: `include!` does not contribute function
  *bodies* to `--check`.**  With an empty mirror the gate reports 278 `- ` lines; with
  `include!("stdbit.rs")` it reports 274 - the four macros/constants that the include adds count,
  but the ~10 `fn`s `stdbit.rs` declares are still reported missing, because `rust_inventory` scans
  `bodies` over the mirror's *own* text while `macros`/`constants`/`statics` are scanned over the
  text plus its includes.  Consequence for whoever finishes this file: the family functions have to
  be declared *here*, and `stdbit.rs`'s overlapping declarations must then retire - the clean shape
  is for `stdbit.in.rs` to become the owner and for `include/lisp.rs` (and the four extern-inline
  units) to reach it, since two mirrors declaring one function is `E0428` in any crate that sees
  both.  That is an integrator decision about another lane's file, so it is recorded, not taken.

## Evidence

- **Judge**: `ls -l build/emacs-ref/lib/stdbit.h` -> `60264` bytes; `grep -c "" build/emacs-ref/lib/stdbit.h`
  -> `2230`; `grep -n HAVE_STDBIT_H build/emacs-ref/src/config.h build/emacs-ref/lib/stdbit.h` ->
  `1433:/* #undef HAVE_STDBIT_H */` (and the header's C 40-46 `#if 0` block around its
  `# include_next`).
- `python3 tools/check-cond-eval.py lib/stdbit.in.h` -> `lib/stdbit.in.h: 135 conditions, 31
  unknown, 101 disagreements [NOT VERIFIED]` (one sample line:
  `DISAGREE lib/stdbit.in.h:2224 `ifndef __STDC_VERSION_STDBIT_H__` — clang False, evaluator True`).
- `python3 tools/extract-abi.py emacs/lib/stdbit.in.h` -> this ledger: 0 subrs, **173 functions
  defined**, **90 macros**, 0 types, **1 file-scope object** (`_gl_stdbit_popcount_support`, C 916).
- `python3 tools/extract-abi.py --check emacs/lib/stdbit.in.h femacs/lib/include/stdbit.in.rs`
  -> **FAIL**, 185 `- ` lines, of which the first are
  `- missing function __popcnt64`, `- missing function _gl_stdbit_clz`, `- missing function
  _gl_stdbit_clzl`, `- missing function _gl_stdbit_clzll`, `- missing function _gl_stdbit_ctz`; the
  task-shaped membership is in the mirror's closing comment block and in the 203 unchecked boxes
  above.
- **Negative verification (the values really are compared)**: a copy of the mirror with one value
  changed, `python3 tools/extract-abi.py --check emacs/lib/stdbit.in.h /tmp/modprobe3/neg.rs` ->
  `- constant __STDC_ENDIAN_BIG__: C 4321 vs Rust 4322` and `FAIL` (the copy was in `/tmp` and is
  deleted with the probe directory).
- **Compile evidence** (no unit includes the mirror, and `lib/include` is not the set
  `xtask layout-check` compiles):
  `rustc +nightly-2026-10-01 --edition 2024 --crate-type bin -C panic=abort -o /tmp/modprobe3/stdbit_probe /tmp/modprobe3/stdbit_probe.rs`
  -> exit 0, and running it prints `stdbit probe ok: 0 bytes slice`; the probe `include!`s
  `lib/include/stdint.in.rs` (for the `uint_leastN_t` names `stdbit.rs` uses), then this mirror, and
  asserts `__STDC_ENDIAN_*`, `__STDC_VERSION_STDBIT_H__`, two `GNULIB_defined_*` flags,
  `_GL_STDBIT_BIGENDIAN`, `_GL_STDBIT_OPTIMIZE_VIA_MEMCPY`, a `_GL_STDBIT_MEMCPY!` round trip and
  `stdc_leading_zeros! (0u32) == 32`.  Both `/tmp/modprobe3` and `/tmp/femacs-target-win` are
  deleted.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 598 .rs files / 593 map targets / 0 findings; `layout-check` passed
  (80 of 91 mirror(s) compiled) - this mirror is not in that crate.
