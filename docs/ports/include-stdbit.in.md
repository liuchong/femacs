# emacs/lib/stdbit.in.h (2229 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 173
- [x] _gl_stdbit_clz  (line 236)
- [x] _gl_stdbit_clzl  (line 241)
- [x] _gl_stdbit_clzll  (line 246)
- [x] _gl_stdbit_clzl  (line 262)
- [x] _gl_stdbit_clz  (line 268)
- [x] _gl_stdbit_clzll  (line 273)
- [x] _gl_stdbit_clzll  (line 287)
- [x] _gl_stdbit_clz  (line 301)
- [x] _gl_stdbit_clzl  (line 306)
- [x] stdc_leading_zeros_ui  (line 318)
- [x] stdc_leading_zeros_uc  (line 324)
- [x] stdc_leading_zeros_us  (line 330)
- [x] stdc_leading_zeros_ul  (line 336)
- [x] stdc_leading_zeros_ull  (line 342)
- [x] stdc_leading_ones_uc  (line 367)
- [x] stdc_leading_ones_us  (line 373)
- [x] stdc_leading_ones_ui  (line 379)
- [x] stdc_leading_ones_ul  (line 385)
- [x] stdc_leading_ones_ull  (line 391)
- [x] _gl_stdbit_ctz  (line 427)
- [x] _gl_stdbit_ctzl  (line 432)
- [x] _gl_stdbit_ctzll  (line 437)
- [x] _gl_stdbit_ctzl  (line 453)
- [x] _gl_stdbit_ctz  (line 459)
- [x] _gl_stdbit_ctzll  (line 464)
- [x] _gl_stdbit_ctz  (line 478)
- [x] _gl_stdbit_ctzl  (line 483)
- [x] _gl_stdbit_ctzll  (line 488)
- [x] stdc_trailing_zeros_ui  (line 500)
- [x] stdc_trailing_zeros_uc  (line 506)
- [x] stdc_trailing_zeros_us  (line 512)
- [x] stdc_trailing_zeros_ul  (line 518)
- [x] stdc_trailing_zeros_ull  (line 524)
- [x] stdc_trailing_ones_uc  (line 549)
- [x] stdc_trailing_ones_us  (line 555)
- [x] stdc_trailing_ones_ui  (line 561)
- [x] stdc_trailing_ones_ul  (line 567)
- [x] stdc_trailing_ones_ull  (line 573)
- [x] stdc_first_leading_zero_uc  (line 598)
- [x] stdc_first_leading_zero_us  (line 606)
- [x] stdc_first_leading_zero_ui  (line 614)
- [x] stdc_first_leading_zero_ul  (line 622)
- [x] stdc_first_leading_zero_ull  (line 630)
- [x] stdc_first_leading_one_uc  (line 657)
- [x] stdc_first_leading_one_us  (line 665)
- [x] stdc_first_leading_one_ui  (line 673)
- [x] stdc_first_leading_one_ul  (line 681)
- [x] stdc_first_leading_one_ull  (line 689)
- [x] stdc_first_trailing_zero_uc  (line 716)
- [x] stdc_first_trailing_zero_us  (line 724)
- [x] stdc_first_trailing_zero_ui  (line 732)
- [x] stdc_first_trailing_zero_ul  (line 740)
- [x] stdc_first_trailing_zero_ull  (line 748)
- [x] stdc_first_trailing_one_uc  (line 775)
- [x] stdc_first_trailing_one_us  (line 783)
- [x] stdc_first_trailing_one_ui  (line 791)
- [x] stdc_first_trailing_one_ul  (line 799)
- [x] stdc_first_trailing_one_ull  (line 807)
- [x] _gl_stdbit_popcount_wide  (line 849)
- [x] __popcnt64  (line 908)
- [x] _gl_stdbit_popcount_supported  (line 919)
- [x] _gl_stdbit_popcount  (line 934)
- [x] _gl_stdbit_popcountl  (line 941)
- [x] _gl_stdbit_popcountll  (line 948)
- [x] stdc_count_ones_ui  (line 966)
- [x] stdc_count_ones_uc  (line 972)
- [x] stdc_count_ones_us  (line 978)
- [x] stdc_count_ones_ul  (line 984)
- [x] stdc_count_ones_ull  (line 990)
- [x] stdc_count_zeros_uc  (line 1015)
- [x] stdc_count_zeros_us  (line 1021)
- [x] stdc_count_zeros_ui  (line 1027)
- [x] stdc_count_zeros_ul  (line 1033)
- [x] stdc_count_zeros_ull  (line 1039)
- [x] stdc_has_single_bit_uc  (line 1064)
- [x] stdc_has_single_bit_us  (line 1071)
- [x] stdc_has_single_bit_ui  (line 1078)
- [x] stdc_has_single_bit_ul  (line 1085)
- [x] stdc_has_single_bit_ull  (line 1092)
- [x] stdc_bit_width_uc  (line 1119)
- [x] stdc_bit_width_us  (line 1125)
- [x] stdc_bit_width_ui  (line 1131)
- [x] stdc_bit_width_ul  (line 1137)
- [x] stdc_bit_width_ull  (line 1143)
- [x] stdc_bit_floor_uc  (line 1171)
- [x] stdc_bit_floor_us  (line 1177)
- [x] stdc_bit_floor_ui  (line 1183)
- [x] stdc_bit_floor_ul  (line 1189)
- [x] stdc_bit_floor_ull  (line 1195)
- [x] stdc_bit_ceil_uc  (line 1228)
- [x] stdc_bit_ceil_us  (line 1234)
- [x] stdc_bit_ceil_ui  (line 1240)
- [x] stdc_bit_ceil_ul  (line 1246)
- [x] stdc_bit_ceil_ull  (line 1252)
- [x] stdc_rotate_left_uc  (line 1297)
- [x] stdc_rotate_left_us  (line 1303)
- [x] stdc_rotate_left_ui  (line 1309)
- [x] stdc_rotate_left_ul  (line 1315)
- [x] stdc_rotate_left_ull  (line 1321)
- [x] stdc_rotate_right_uc  (line 1363)
- [x] stdc_rotate_right_us  (line 1369)
- [x] stdc_rotate_right_ui  (line 1375)
- [x] stdc_rotate_right_ul  (line 1381)
- [x] stdc_rotate_right_ull  (line 1387)
- [x] stdc_memreverse8  (line 1416)
- [x] stdc_memreverse8u8  (line 1454)
- [x] stdc_memreverse8u16  (line 1460)
- [x] stdc_memreverse8u32  (line 1472)
- [x] stdc_memreverse8u64  (line 1486)
- [x] stdc_load8_beu8  (line 1646)
- [x] stdc_load8_beu16  (line 1652)
- [x] stdc_load8_beu32  (line 1660)
- [x] stdc_load8_beu64  (line 1670)
- [x] stdc_load8_leu8  (line 1687)
- [x] stdc_load8_leu16  (line 1693)
- [x] stdc_load8_leu32  (line 1701)
- [x] stdc_load8_leu64  (line 1711)
- [x] stdc_load8_bes8  (line 1728)
- [x] stdc_load8_bes16  (line 1734)
- [x] stdc_load8_bes32  (line 1740)
- [x] stdc_load8_bes64  (line 1746)
- [x] stdc_load8_les8  (line 1752)
- [x] stdc_load8_les16  (line 1758)
- [x] stdc_load8_les32  (line 1764)
- [x] stdc_load8_les64  (line 1770)
- [x] stdc_load8_aligned_beu8  (line 1785)
- [x] stdc_load8_aligned_beu16  (line 1791)
- [x] stdc_load8_aligned_beu32  (line 1806)
- [x] stdc_load8_aligned_beu64  (line 1821)
- [x] stdc_load8_aligned_leu8  (line 1836)
- [x] stdc_load8_aligned_leu16  (line 1842)
- [x] stdc_load8_aligned_leu32  (line 1857)
- [x] stdc_load8_aligned_leu64  (line 1872)
- [x] stdc_load8_aligned_bes8  (line 1887)
- [x] stdc_load8_aligned_bes16  (line 1893)
- [x] stdc_load8_aligned_bes32  (line 1899)
- [x] stdc_load8_aligned_bes64  (line 1905)
- [x] stdc_load8_aligned_les8  (line 1911)
- [x] stdc_load8_aligned_les16  (line 1917)
- [x] stdc_load8_aligned_les32  (line 1923)
- [x] stdc_load8_aligned_les64  (line 1929)
- [x] stdc_store8_beu8  (line 1947)
- [x] stdc_store8_beu16  (line 1953)
- [x] stdc_store8_beu32  (line 1960)
- [x] stdc_store8_beu64  (line 1969)
- [x] stdc_store8_leu8  (line 1982)
- [x] stdc_store8_leu16  (line 1988)
- [x] stdc_store8_leu32  (line 1995)
- [x] stdc_store8_leu64  (line 2004)
- [x] stdc_store8_bes8  (line 2017)
- [x] stdc_store8_bes16  (line 2023)
- [x] stdc_store8_bes32  (line 2029)
- [x] stdc_store8_bes64  (line 2035)
- [x] stdc_store8_les8  (line 2041)
- [x] stdc_store8_les16  (line 2047)
- [x] stdc_store8_les32  (line 2053)
- [x] stdc_store8_les64  (line 2059)
- [x] stdc_store8_aligned_beu8  (line 2074)
- [x] stdc_store8_aligned_beu16  (line 2080)
- [x] stdc_store8_aligned_beu32  (line 2093)
- [x] stdc_store8_aligned_beu64  (line 2106)
- [x] stdc_store8_aligned_leu8  (line 2119)
- [x] stdc_store8_aligned_leu16  (line 2125)
- [x] stdc_store8_aligned_leu32  (line 2138)
- [x] stdc_store8_aligned_leu64  (line 2151)
- [x] stdc_store8_aligned_bes8  (line 2164)
- [x] stdc_store8_aligned_bes16  (line 2170)
- [x] stdc_store8_aligned_bes32  (line 2176)
- [x] stdc_store8_aligned_bes64  (line 2182)
- [x] stdc_store8_aligned_les8  (line 2188)
- [x] stdc_store8_aligned_les16  (line 2194)
- [x] stdc_store8_aligned_les32  (line 2200)
- [x] stdc_store8_aligned_les64  (line 2206)

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
- [x] _  (line 37, body a2b18f040ffc)
- [x] _Bool  (line 31, body 5039d155a71c)
- [x] _GL_HAS_BUILTIN_ASSUME_ALIGNED  (line 1614, body 356a192b7913)
- [x] _GL_STDBIT_ASSUME_ALIGNED  (line 1621, body 019a49c4c0cd)
- [x] _GL_STDBIT_BIGENDIAN  (line 1639, body 1aa97d97b98b)
- [x] _GL_STDBIT_HAS_BUILTIN_BSWAP16  (line 76, body 356a192b7913)
- [x] _GL_STDBIT_HAS_BUILTIN_BSWAP32  (line 84, body 356a192b7913)
- [x] _GL_STDBIT_HAS_BUILTIN_BSWAP64  (line 87, body 356a192b7913)
- [x] _GL_STDBIT_HAS_BUILTIN_CLZ  (line 228, body 5ffe533b830f)
- [x] _GL_STDBIT_HAS_BUILTIN_CTZ  (line 419, body 5ffe533b830f)
- [x] _GL_STDBIT_HAS_BUILTIN_POPCOUNT  (line 837, body 5ffe533b830f)
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
- [x] _gl_stdbit_popcount  (line 955, body a0ca3d1a7b76)
- [x] _gl_stdbit_popcountl  (line 956, body a0ca3d1a7b76)
- [x] _gl_stdbit_popcountll  (line 957, body a0ca3d1a7b76)
- [x] _gl_stdc_rotate_left  (line 1289, body d6b413786d8c)
- [x] _gl_stdc_rotate_right  (line 1355, body 53ff758ada8f)
- [x] stdc_bit_ceil  (line 1264, body fc1200ca7d31)
- [x] stdc_bit_floor  (line 1207, body f266e663fdb9)
- [x] stdc_bit_width  (line 1151, body b5631a388588)
- [x] stdc_count_ones  (line 998, body 4c82f8864756)
- [x] stdc_count_zeros  (line 1047, body d54c60259f75)
- [x] stdc_first_leading_one  (line 699, body 464bd0245f7d)
- [x] stdc_first_leading_zero  (line 640, body dd0fda82ca32)
- [x] stdc_first_trailing_one  (line 817, body 1b1ff9c96c83)
- [x] stdc_first_trailing_zero  (line 758, body 46fdd1cd8c82)
- [x] stdc_has_single_bit  (line 1101, body 34d852b11bdc)
- [x] stdc_leading_ones  (line 399, body ba433c4c61fc)
- [x] stdc_leading_zeros  (line 350, body 10aca907d8a9)
- [x] stdc_rotate_left  (line 1330, body f51da2f10d52)
- [x] stdc_rotate_right  (line 1396, body ba894f7d8715)
- [x] stdc_trailing_ones  (line 581, body ec71ecff1b83)
- [x] stdc_trailing_zeros  (line 532, body 8b4b9178cbc2)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern signed char _gl_stdbit_popcount_support  (line 916)

## Decisions

- **The judge exists and the whole template is live.**  `ls -l build/emacs-ref/lib/stdbit.h` ->
  `60264` bytes, 2,230 lines; `grep -n HAVE_STDBIT_H build/emacs-ref/src/config.h` ->
  `1433:/* #undef HAVE_STDBIT_H */`, and `docs/baseline/config/darwin-arm64.macros.tsv` records the
  same `undef`.  So the system has no `<stdbit.h>` and gnulib's header defines the whole C23 API
  itself (its C 40-46 `#if 0` block around the `# include_next` is the dead arm).  **A correction
  for the record**: `femacs/lib/include/stdbit.rs` 3 said "the frozen configuration has
  HAVE_STDBIT_H = 1" - inverted; both sources say `undef` (the integrator fixed that sentence in
  the file).  Its *slice* was still right (it mirrors the live builtin arms), but the sentence was
  not, and `stdbit.rs` is unpaired in the inventory, so no gate ever read it.
- **`tools/check-cond-eval.py lib/stdbit.in.h` -> `135 conditions, 31 unknown, 101 disagreements
  [NOT VERIFIED]`**, so the extractor resolves nothing and its checklist is the union of every arm
  (264 boxes down to names: `173 functions defined`, `90 macros`, `1 file-scope object`); the 22
  macros whose bodies differ per arm are reported as *"several definitions … body not compared"*.
  The arms that matter are decided by the frozen compiler, and `stdbit.rs` 5-15 holds the readings:
  `_GL_STDBIT_HAS_BUILTIN_CLZ/CTZ/POPCOUNT` and `…BSWAP16/32/64` are live, so each family's builtin
  body is the one to spell and the hand-written fallbacks (C 287-311, 419-493, 837-960, 1447-1500,
  …) are dead text (P-66 rule 2: mark the boundary, record the lines).  That conservative reading is
  the reason this ledger still has five unchecked boxes (below), and why the closure needs rows in
  two *shared* tables rather than a fake spelling inside the mirror.
- **Round 1 (settings layer) and Round 2 (the ownership move) are history; this is the closure.**
  Round 1 delivered the qualifier macros and the file's settings (61 boxes) and recorded the
  families instead of writing them; Round 2 moved the 30 functions `lib/include/stdbit.rs` used to
  carry into *this* file's own text (which is what the extractor reads for function bodies), added
  the three `~n` families `stdc_leading_ones_*`/`stdc_trailing_ones_*`/`stdc_count_zeros_*`, their
  dispatchers, and the `_gl_stdbit_popcount_support` object (C 916, declaration-only), and
  repointed the seven include sites (`src/include/lisp.rs` 19,
  `lib/stdc_{leading_zeros,trailing_zeros,bit_width,count_ones,memreverse8u}.rs` 7,
  `lib/include/byteswap.in.rs` 17).  The round-1/round-2 statements that the families were "not
  written" and that `--check` had 185/162 `- ` lines are **superseded by Round 3**.
- **Round 3 closes the API: 259 of 264 boxes.**  Landed this round (all in this file's own text,
  because `tools/extract-abi.py` reads function bodies from a mirror's own text only - Round 2's
  measurement):
  - the **popcount layer**: `_gl_stdbit_popcount{,l,ll}` as **`macro_rules!`** (C 839-841's live
    `#  ifdef _GL_STDBIT_HAS_BUILTIN_POPCOUNT` arm defines them as `__builtin_popcount*`, so a macro
    is C's own shape) with the three `stdc_count_ones_*` call sites written `_gl_stdbit_popcount! (n)`
    as C's body does;
  - the **six capability constants** `_GL_STDBIT_HAS_BUILTIN_{BSWAP16,BSWAP32,BSWAP64}` (`1`,
    C 77-90) and `…{CLZ,CTZ,POPCOUNT}` (`true`, C 228/419/836) - the frozen compiler defines all
    six and C makes each a *branch* macro, so only the name and the value are compared;
  - the **four `first_*` families** (20 members, C 599-820) and their four dispatchers;
  - **`stdc_has_single_bit_*`** (5, C 1063-1104) and its dispatcher;
  - **`stdc_bit_floor_*`** / **`stdc_bit_ceil_*`** (10, C 1176-1267) and their two dispatchers;
  - **`stdc_rotate_left_*`** / **`stdc_rotate_right_*`** (10, C 1301-1399), the two
    `_gl_stdc_rotate_{left,right}` helper macros (C 1289-1291/1355-1357's `#  ifndef` arm: C's own
    shift-and-mask arithmetic, since Rust has no `__builtin_stdc_rotate_*`) and the two dispatchers;
  - **`stdc_memreverse8`** (C 1410-1436, C's loop with `ptr[i]` as `*ptr.add (i)`);
  - the **whole `load8`/`store8`/`_aligned` matrix** (64 members, C 1646-2219): C's byte assembly
    with C's shift lists and C's `& 0xFFU` stores, every `*s*` member C's own delegation, and every
    `_aligned` member C's two arms compiled (the frozen `_GL_STDBIT_OPTIMIZE_VIA_MEMCPY` is `0`, so
    the memcpy arm is not taken, but C compiles it and so does the port).
- **Three spellings the family layer needed, each measured, none of them a new shared row.**
  1. `_GL_STDBIT_UINT_FASTN` (C 60-64) is a *type* macro and Rust cannot use a macro in type
     position, so the three names the load8/store8 bodies need are also real aliases of the frozen
     compiler's answer - `clang -std=gnu23 -fsyntax-only -Xclang -ast-dump` on
     `__UINT_FAST16_TYPE__ a16; __UINT_FAST32_TYPE__ a32; __UINT_FAST64_TYPE__ a64;` reports
     `unsigned short` / `unsigned int` / `unsigned long long`, i.e. `c_ushort` / `c_uint` /
     `c_ulonglong`.  The macros keep C's spelling above them.
  2. `_GL_STDBIT_MEMCPY` (C 96-105): C's `memcpy` takes `void *` and Rust's
     `std::ptr::copy_nonoverlapping` needs one element type, so both sides go through `c_uchar` -
     a no-op for the byte copies C's `_aligned` arms make.  C's two arms (`__builtin_memcpy` and
     `memcpy`) stay in the doc comment.
  3. `_GL_STDBIT_ASSUME_ALIGNED` (C 1606-1623): the live C arm is
     `__builtin_assume_aligned (ptr, align)` and Rust has no such builtin, so the port's macro
     spells C's own `#else` arm, `(ptr)` - the same value, no fake symbol, and the live arm's C
     text is kept in the doc comment (P-55).
- **C's unsigned arithmetic is spelled `wrapping_sub`, not `-`**: `stdc_has_single_bit_uc (0)` is
  C's `255 < 255` = false, and `stdc_bit_ceil_*`'s `n - 1` is C's wrap; Rust's checked `-` would
  panic in a debug build where C wraps, so both take `wrapping_sub`.  `stdc_rotate_*`'s `-(c)` is
  `0usize.wrapping_sub (c as usize)` for the same reason (C's unary minus on an unsigned count).
- **The 5 unchecked boxes, named (this is the whole remaining gap).**
  - `_` and `_Bool` (C 37/31, the prologue's placeholder-shaped names) - already covered by the
    prologue's `docs/ports/excluded-macros.txt` rows (the gate prints *"2 C macros have a recorded
    Rust counterpart: _, _Bool"*).
  - `_gl_stdbit_popcount_wide` (C 849-905), `__popcnt64` (C 908-918) and
    `_gl_stdbit_popcount_supported` (C 919-933): all three live in the `#  else` arm of C 836-838's
    `#  ifdef _GL_STDBIT_HAS_BUILTIN_POPCOUNT` and, for the last two, inside `#  ifdef _MSC_VER`
    (C 906).  That arm is dead on this target (the compiler has `__builtin_popcount*`), and its text
    names MSVC intrinsics - `__cpuid`, `__popcnt` - that have no Rust spelling at all.  P-66 rule 2
    asks for the boundary and the C line numbers rather than a transcription, so **three
    `docs/ports/excluded-macros.txt` rows (`kind=deferred`) are requested** (listed verbatim under
    `## Evidence`); nothing is carried in the mirror for them, and the mirror's closing note says so.
- **Five `docs/ports/macro-body-notes.tsv` rows are requested** (also verbatim under `## Evidence`)
  for the tokens C's *dead* arms contribute and C's `ull` suffix that Rust cannot spell:
  `stdc_memreverse8u{16,32,64}` (the live arm is `__builtin_bswap*`, which the port spells
  `swap_bytes ()`; the `#  else` mask arms are C 1447-1451/1453-1461/1463-1476) and
  `stdc_bit_floor_ull`/`stdc_bit_ceil_ull` (C's `1ull`/`2ull`; Rust writes `1u64`/`2u64`, and the
  tokeniser folds only `u`/`l`/`ul`/`ll` - the `_GL_U64_MASK32`/`OPENSSL_API_COMPAT` rows' class).
  All five items *are* delivered in the mirror (their boxes are ticked); only the dead-arm tokens
  are exempted, which is what keeps the gate honest about the rest of each body.
- **`--check` verdict with the eight rows in place: `OK` and no `- ` line.**  Against the tree's
  *current* tables it reports exactly those eight lines, all in the two classes above; the
  verification (a `/tmp` copy of each table plus the eight rows, passed through `--excluded` /
  `--macro-notes`) is in `## Evidence`.  So this file's C23 surface is closed; the eight rows are an
  integrator edit to two shared tables, not a gap in the mirror.

## Evidence

- **`--check` with the eight requested rows (the closure verdict)**:
  `python3 tools/extract-abi.py --check --excluded /tmp/abip/tab/excluded-macros.txt
  --macro-notes /tmp/abip/tab/macro-body-notes.tsv emacs/lib/stdbit.in.h
  femacs/lib/include/stdbit.in.rs` -> `OK emacs/lib/stdbit.in.h -> femacs/lib/include/stdbit.in.rs`
  and `grep -c '^- '` -> `0`.  With the tree's unmodified tables the same command reports exactly 8
  lines: `- missing function __popcnt64`, `- missing function _gl_stdbit_popcount_supported`,
  `- missing function _gl_stdbit_popcount_wide`, `- function stdc_bit_floor_ull does not mention
  ull`, `- function stdc_bit_ceil_ull does not mention ull`, and the three
  `- function stdc_memreverse8u{16,32,64} does not mention …` lines.
- **The eight rows requested** (verbatim; `<TAB>` is a literal tab):
  - to `docs/ports/excluded-macros.txt`:
    `__popcnt64<TAB>deferred<TAB>stdbit.in.h 908-918: the MSVC intrinsic pair (`extern unsigned int __popcnt (unsigned int);` declared by hand to avoid <intrin.h>) inside `#  ifdef _MSC_VER` (C 906) - dead on this target, and `__popcnt` has no Rust spelling`
    `_gl_stdbit_popcount_supported<TAB>deferred<TAB>stdbit.in.h 919-933: the MSVC `__cpuid` POPCNT probe of the same dead arm (C 906-960 is the `#  else` arm of `#  ifdef _GL_STDBIT_HAS_BUILTIN_POPCOUNT`, C 836-838) - Rust has no `__cpuid`, and the port's popcount is the compiler builtin`
    `_gl_stdbit_popcount_wide<TAB>deferred<TAB>stdbit.in.h 848-905: the hand-written mask-table popcount of that dead arm (x555555/x333333/x0f0f0f/x010101/x000_7f, C 869-874) - P-66 rule 2 keeps the boundary and the line numbers rather than a transcription; the live arm is the `_gl_stdbit_popcount{,l,ll}` macros the mirror defines`
  - to `docs/ports/macro-body-notes.tsv`:
    `stdc_bit_floor_ull<TAB>ull<TAB>stdbit.in.h 1200-1210: C's `1ull` literal's `ull` suffix is not a Rust spelling (Rust writes `1u64`), and only `u`/`l`/`ul`/`ll` are folded by the tokeniser; the number and the width match (the `_GL_U64_MASK32` row's class)`
    `stdc_bit_ceil_ull<TAB>ull<TAB>stdbit.in.h 1257-1267: the same `2ull` suffix as stdc_bit_floor_ull`
    `stdc_memreverse8u16<TAB>0,1,8,255,__builtin_bswap16,_gl_stdbit_uint_fast16,mask<TAB>stdbit.in.h 1445-1451: the live arm is `__builtin_bswap16 (value)`, which the port spells `value.swap_bytes ()` (the builtin has no Rust token); the `#  else` arm (C 1447-1451: `mask = 0xFFU` and the `8 * 0`/`8 * 1` shifts) is dead text on this target (P-66 rule 2)`
    `stdc_memreverse8u32<TAB>0,1,2,255,3,8,__builtin_bswap32,_gl_stdbit_uint_fast32,mask<TAB>stdbit.in.h 1453-1461: the same two arms as stdc_memreverse8u16, with the `4`-byte mask table`
    `stdc_memreverse8u64<TAB>0,1,2,3,4,5,6,7,8,255,__builtin_bswap64,_gl_stdbit_uint_fast64,mask<TAB>stdbit.in.h 1463-1476: the same two arms, with the `8`-byte mask table`
- **Compile evidence (the mirror must always compile, and it does)**:
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-win cargo run -q -p xtask -- layout-check` ->
  `layout-check: 83 of 94 mirror(s) compiled, 11 platform-exclusive skipped (…dispextern.rs…)`,
  `layout probe ok`, `layout-check passed`, exit 0; the probe source regenerated for this run
  (`build/femacs/layout_probe_src/…_lib_include_stdbit_in_rs.rs`, 84,401 bytes) and
  `grep -c 'pub fn stdc_load8_aligned_beu64'` on it -> `1`, so the new content really went through
  rustc.  The unit-level consumers: `build-tu lib/stdc_{leading_zeros,bit_width,count_ones,
  trailing_zeros,memreverse8u}.c` and `build-tu lib/byteswap.c` -> six `built … .o` lines (verbatim
  in the report); `build-tu src/lisp.c` -> `src/lisp.rs does not exist yet (ports of src/lisp.c not
  written)`, so the four extern-inline units are the consumers that exist today.
- **Semantic probe, and it is the *standard library's* operation that is the expectation, not a
  literal of mine**: the probe is kept in the tree as `spikes/stdbit_family_probe.rs` (the
  `spikes/fam_probe.rs`/`spikes/win_inc_probe.rs` convention, with its command in the header):
  `rustc +nightly-2026-10-01 --edition 2024 --crate-type bin -C panic=abort -o
  /tmp/stdbit_family_probe spikes/stdbit_family_probe.rs` -> exit 0; `/tmp/stdbit_family_probe` ->
  `stdbit probe: 544 semantic checks passed`, exit 0.  The checks cover, for many values of
  each width, `has_single_bit_*` vs `is_power_of_two`, `bit_floor_*`/`bit_ceil_*` vs a
  `leading_zeros`-based reference written in the probe (including the C23 `0`-when-unrepresentable
  case), the four `first_*` families vs `leading_ones`/`leading_zeros`/`trailing_ones`/
  `trailing_zeros`, `rotate_left`/`rotate_right` (counts 0, 1, 3, 5, 8, 9, 31, 32, 33, 100 and
  `0xFFFF_FFFF`, i.e. across the mask boundary) vs Rust's own `rotate_*`, `memreverse8u*` vs
  `from_be_bytes (to_le_bytes ())`, the load8/store8 matrix vs `from_le_bytes`/`from_be_bytes`/
  `to_le_bytes`/`to_be_bytes` (plus every `_aligned` member against its unaligned original, C's
  frozen `else` arm), `memreverse8` (including `n == 0`), and `stdc_count_ones_*`/`count_zeros_*`
  vs `count_ones`/`count_zeros`.  All five widths are covered: `uc`/`us`/`ui`/`ull` in the main
  loops and `ul` in a block of its own (same width as `ull` on this target, separate bodies).
- **Two negative controls (the token gate fires; and what it cannot see, the probe can)**:
  1. `--check` on two mutated copies (same `/tmp` tables): `stdc_bit_floor_ui` calling
     `stdc_bit_width_uc` -> `- function stdc_bit_floor_ui does not mention stdc_bit_width_ui`; and
     `stdc_load8_aligned_beu16/leu16` with `_GL_STDBIT_ASSUME_ALIGNED! (ptr, 2)` replaced by `(ptr)`
     -> `- function stdc_load8_aligned_beu16 does not mention _gl_stdbit_assume_aligned` (and the
     `le` twin).  (The copies sit in `/tmp` with their two `include!` paths rewritten to absolute
     ones, because `include!` resolves relative to the file it is written in; the unmutated copy
     through the same technique prints `OK`.)
  2. **The gate is a token gate, so it cannot see a wrong value** - the measured example:
     `stdc_bit_floor_ui`'s `else { 0 }` changed to `else { 1 }` still prints
     `OK emacs/lib/stdbit.in.h -> /tmp/abip/neg/v.rs` (the `0` the body must mention is still there
     from `n != 0`), while the same mutation in the probe makes it panic
     `assertion `left == right` failed: bit_floor_ui 0 / left: 1 / right: 0` (exit 134).  That is
     why the probe exists next to the gate.
- **The popcount macros are the right shape, measured by the gate**: with the integrator's 3.185
  repair (`fn`s named `_gl_stdbit_popcount{,l,ll}`) the gate compared *those* bodies against the
  dead MSVC arm's text and added three lines - `- function _gl_stdbit_popcount does not mention
  __popcnt, _gl_stdbit_popcount_supported, _gl_stdbit_popcount_wide` (and the `l`/`ll` twins, the
  `ll` one naming `__popcnt64`).  `rust_inventory` finds a `macro_rules!` body for the *macro*
  loop and no `fn` body at all for the function loop, so C's own shape (a macro, C 839-841) both
  matches C's identity and stops the gate from demanding a dead arm's tokens - which is why the
  mirror now spells them as macros and the call sites keep the `!`.
- **Gates on this worktree** (all five, last lines):
  `check-declarations` -> `13 declaration sites (0 forbidden, 13 allowed literals), 0 failures`;
  `check-globals` -> `check-globals: 283 units, 0 DEFVAR_* variables without a definition in their
  own unit`; `check-mirror-dupes` -> `1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)`;
  `check-verbatim` -> `6 trees, 4607 files, 0 failures`; `check-target-names` ->
  `check-target-names: 601 .rs files, 593 map targets, 0 finding(s)`.
