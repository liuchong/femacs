# emacs/lib/inttypes.in.h (1029 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 167
- [x] GNULIB_defined_imaxdiv_t  (line 938, body 356a192b7913)
- [x] INTTYPES_H  (line 47, body da39a3ee5e6b)
- [x] PRIX16  (line 143, body 2ee91bd5aa84)
- [x] PRIX32  (line 173, body 2ee91bd5aa84)
- [x] PRIX64  (line 209, body 823448e934f2)
- [x] PRIX8  (line 113, body 2ee91bd5aa84)
- [x] PRIXFAST16  (line 371, body 2ee91bd5aa84)
- [x] PRIXFAST32  (line 413, body 2ee91bd5aa84)
- [x] PRIXFAST64  (line 435, body 03098d6dfcbc)
- [x] PRIXFAST8  (line 329, body 2ee91bd5aa84)
- [x] PRIXLEAST16  (line 247, body 2ee91bd5aa84)
- [x] PRIXLEAST32  (line 265, body 2ee91bd5aa84)
- [x] PRIXLEAST64  (line 286, body 03098d6dfcbc)
- [x] PRIXLEAST8  (line 229, body 2ee91bd5aa84)
- [x] PRIXMAX  (line 478, body 7978bdd6bb99)
- [x] PRIXPTR  (line 509, body 70978d49b1d5)
- [x] PRId16  (line 118, body 03b7121d3d9f)
- [x] PRId32  (line 148, body 03b7121d3d9f)
- [x] PRId64  (line 185, body f70906666993)
- [x] PRId8  (line 88, body 03b7121d3d9f)
- [x] PRIdFAST16  (line 336, body 03b7121d3d9f)
- [x] PRIdFAST32  (line 378, body 03b7121d3d9f)
- [x] PRIdFAST64  (line 418, body 39780201f63a)
- [x] PRIdFAST8  (line 294, body 03b7121d3d9f)
- [x] PRIdLEAST16  (line 232, body 03b7121d3d9f)
- [x] PRIdLEAST32  (line 250, body 03b7121d3d9f)
- [x] PRIdLEAST64  (line 269, body 39780201f63a)
- [x] PRIdLEAST8  (line 214, body 03b7121d3d9f)
- [x] PRIdMAX  (line 443, body 7556969412a3)
- [x] PRIdPTR  (line 484, body 45e520b24830)
- [x] PRIi16  (line 123, body 75f5b6b545ad)
- [x] PRIi32  (line 153, body 75f5b6b545ad)
- [x] PRIi64  (line 188, body 47e2b3564828)
- [x] PRIi8  (line 93, body 75f5b6b545ad)
- [x] PRIiFAST16  (line 343, body 75f5b6b545ad)
- [x] PRIiFAST32  (line 385, body 75f5b6b545ad)
- [x] PRIiFAST64  (line 421, body 238d5c392b0e)
- [x] PRIiFAST8  (line 301, body 75f5b6b545ad)
- [x] PRIiLEAST16  (line 235, body 75f5b6b545ad)
- [x] PRIiLEAST32  (line 253, body 75f5b6b545ad)
- [x] PRIiLEAST64  (line 272, body 238d5c392b0e)
- [x] PRIiLEAST8  (line 217, body 75f5b6b545ad)
- [x] PRIiMAX  (line 450, body c614987e3b11)
- [x] PRIiPTR  (line 489, body 0595ea0c4d3f)
- [x] PRIo16  (line 128, body 6edbf0c89d20)
- [x] PRIo32  (line 158, body 6edbf0c89d20)
- [x] PRIo64  (line 200, body 24f1dc4d276f)
- [x] PRIo8  (line 98, body 6edbf0c89d20)
- [x] PRIoFAST16  (line 350, body 6edbf0c89d20)
- [x] PRIoFAST32  (line 392, body 6edbf0c89d20)
- [x] PRIoFAST64  (line 426, body ae12623464d9)
- [x] PRIoFAST8  (line 308, body 6edbf0c89d20)
- [x] PRIoLEAST16  (line 238, body 6edbf0c89d20)
- [x] PRIoLEAST32  (line 256, body 6edbf0c89d20)
- [x] PRIoLEAST64  (line 277, body ae12623464d9)
- [x] PRIoLEAST8  (line 220, body 6edbf0c89d20)
- [x] PRIoMAX  (line 457, body 0f306f61b2da)
- [x] PRIoPTR  (line 494, body 4e6ec2864d88)
- [x] PRIu16  (line 133, body 5dff53c670f4)
- [x] PRIu32  (line 163, body 5dff53c670f4)
- [x] PRIu64  (line 203, body 35d3a0f31dfd)
- [x] PRIu8  (line 103, body 5dff53c670f4)
- [x] PRIuFAST16  (line 357, body 5dff53c670f4)
- [x] PRIuFAST32  (line 399, body 5dff53c670f4)
- [x] PRIuFAST64  (line 429, body 262ba8c69183)
- [x] PRIuFAST8  (line 315, body 5dff53c670f4)
- [x] PRIuLEAST16  (line 241, body 5dff53c670f4)
- [x] PRIuLEAST32  (line 259, body 5dff53c670f4)
- [x] PRIuLEAST64  (line 280, body 262ba8c69183)
- [x] PRIuLEAST8  (line 223, body 5dff53c670f4)
- [x] PRIuMAX  (line 464, body c70d5977aa4f)
- [x] PRIuPTR  (line 499, body 7b75e6146ba0)
- [x] PRIx16  (line 138, body a81fa20d625f)
- [x] PRIx32  (line 168, body a81fa20d625f)
- [x] PRIx64  (line 206, body fb5a1a5c5cff)
- [x] PRIx8  (line 108, body a81fa20d625f)
- [x] PRIxFAST16  (line 364, body a81fa20d625f)
- [x] PRIxFAST32  (line 406, body a81fa20d625f)
- [x] PRIxFAST64  (line 432, body f19c84fdbb8e)
- [x] PRIxFAST8  (line 322, body a81fa20d625f)
- [x] PRIxLEAST16  (line 244, body a81fa20d625f)
- [x] PRIxLEAST32  (line 262, body a81fa20d625f)
- [x] PRIxLEAST64  (line 283, body f19c84fdbb8e)
- [x] PRIxLEAST8  (line 226, body a81fa20d625f)
- [x] PRIxMAX  (line 471, body 0fbe0e59fa83)
- [x] PRIxPTR  (line 504, body 72853bf95a3c)
- [x] SCNd16  (line 540, body 0ee599646b48)
- [x] SCNd32  (line 565, body 03b7121d3d9f)
- [x] SCNd64  (line 597, body b863a181cd88)
- [x] SCNd8  (line 515, body c903f647a759)
- [x] SCNdFAST16  (line 748, body 03b7121d3d9f)
- [x] SCNdFAST32  (line 791, body 03b7121d3d9f)
- [x] SCNdFAST64  (line 824, body 1187f1efcc9c)
- [x] SCNdFAST8  (line 695, body 03b7121d3d9f)
- [x] SCNdLEAST16  (line 638, body 0ee599646b48)
- [x] SCNdLEAST32  (line 653, body 03b7121d3d9f)
- [x] SCNdLEAST64  (line 669, body 1187f1efcc9c)
- [x] SCNdLEAST8  (line 623, body c903f647a759)
- [x] SCNdMAX  (line 846, body 7556969412a3)
- [x] SCNdPTR  (line 880, body 45e520b24830)
- [x] SCNi16  (line 545, body 1e639e49b198)
- [x] SCNi32  (line 570, body 75f5b6b545ad)
- [x] SCNi64  (line 600, body ce999e564045)
- [x] SCNi8  (line 520, body 8372f461da23)
- [x] SCNiFAST16  (line 757, body 75f5b6b545ad)
- [x] SCNiFAST32  (line 798, body 75f5b6b545ad)
- [x] SCNiFAST64  (line 827, body 33348bb6596a)
- [x] SCNiFAST8  (line 706, body 75f5b6b545ad)
- [x] SCNiLEAST16  (line 641, body 1e639e49b198)
- [x] SCNiLEAST32  (line 656, body 75f5b6b545ad)
- [x] SCNiLEAST64  (line 672, body 33348bb6596a)
- [x] SCNiLEAST8  (line 626, body 8372f461da23)
- [x] SCNiMAX  (line 853, body c614987e3b11)
- [x] SCNiPTR  (line 885, body 0595ea0c4d3f)
- [x] SCNo16  (line 550, body 277197648e72)
- [x] SCNo32  (line 575, body 6edbf0c89d20)
- [x] SCNo64  (line 612, body d15c44f4d314)
- [x] SCNo8  (line 525, body 28529fce28d8)
- [x] SCNoFAST16  (line 766, body 6edbf0c89d20)
- [x] SCNoFAST32  (line 805, body 6edbf0c89d20)
- [x] SCNoFAST64  (line 832, body 6378056e797a)
- [x] SCNoFAST8  (line 717, body 6edbf0c89d20)
- [x] SCNoLEAST16  (line 644, body 277197648e72)
- [x] SCNoLEAST32  (line 659, body 6edbf0c89d20)
- [x] SCNoLEAST64  (line 677, body 6378056e797a)
- [x] SCNoLEAST8  (line 629, body 28529fce28d8)
- [x] SCNoMAX  (line 860, body 0f306f61b2da)
- [x] SCNoPTR  (line 890, body 4e6ec2864d88)
- [x] SCNu16  (line 555, body 3184957c40ed)
- [x] SCNu32  (line 580, body 5dff53c670f4)
- [x] SCNu64  (line 615, body 21c56079b932)
- [x] SCNu8  (line 530, body 1a5a30b85f49)
- [x] SCNuFAST16  (line 775, body 5dff53c670f4)
- [x] SCNuFAST32  (line 812, body 5dff53c670f4)
- [x] SCNuFAST64  (line 835, body 696ede666a4a)
- [x] SCNuFAST8  (line 728, body 5dff53c670f4)
- [x] SCNuLEAST16  (line 647, body 3184957c40ed)
- [x] SCNuLEAST32  (line 662, body 5dff53c670f4)
- [x] SCNuLEAST64  (line 680, body 696ede666a4a)
- [x] SCNuLEAST8  (line 632, body 1a5a30b85f49)
- [x] SCNuMAX  (line 867, body c70d5977aa4f)
- [x] SCNuPTR  (line 895, body 7b75e6146ba0)
- [x] SCNx16  (line 560, body 71dda43dfdf5)
- [x] SCNx32  (line 585, body a81fa20d625f)
- [x] SCNx64  (line 618, body 7ba84ca7ec81)
- [x] SCNx8  (line 535, body 2361d7344925)
- [x] SCNxFAST16  (line 784, body a81fa20d625f)
- [x] SCNxFAST32  (line 819, body a81fa20d625f)
- [x] SCNxFAST64  (line 838, body a0dd53d67bd8)
- [x] SCNxFAST8  (line 739, body a81fa20d625f)
- [x] SCNxLEAST16  (line 650, body 71dda43dfdf5)
- [x] SCNxLEAST32  (line 665, body a81fa20d625f)
- [x] SCNxLEAST64  (line 683, body a0dd53d67bd8)
- [x] SCNxLEAST8  (line 635, body 2361d7344925)
- [x] SCNxMAX  (line 874, body 0fbe0e59fa83)
- [x] SCNxPTR  (line 900, body 72853bf95a3c)
- [x] _GL_FINISHED_INCLUDING_SYSTEM_INTTYPES_H  (line 42, body da39a3ee5e6b)
- [x] _LONG_LONG_FORMAT_PREFIX  (line 83, body 888b68a3d6da)
- [x] _PRI64_PREFIX  (line 182, body c011b67df613)
- [x] _PRIu64_PREFIX  (line 197, body c011b67df613)
- [x] _SCN64_PREFIX  (line 594, body c011b67df613)
- [x] _SCNu64_PREFIX  (line 609, body c011b67df613)
- [x] __STDC_FORMAT_MACROS  (line 37, body 356a192b7913)
- [x] imaxabs  (line 914, body 395cacf53f20)
- [x] imaxdiv  (line 944, body 08bad030b339)
- [x] strtoimax  (line 968, body 179f983b6a83)
- [x] strtoumax  (line 997, body f610925eca3b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The criterion is the generated header, and it is the system `<inttypes.h>` that answers.**
  gnulib's `inttypes-h` rule does substitute here: `build/emacs-ref/lib/inttypes.h` exists
  (46,752 bytes, `/* DO NOT EDIT! GENERATED AUTOMATICALLY! */` on line 1).  Its substitutions,
  from the same build: `HAVE_INTTYPES_H` 1, `APPLE_UNIVERSAL_BUILD` 0,
  `INT64_MAX_EQ_LONG_MAX`/`UINT64_MAX_EQ_ULONG_MAX` `defined _LP64`,
  `INT32_MAX_LT_INTMAX_MAX`/`UINT32_MAX_LT_UINTMAX_MAX` 1, `PRIPTR_PREFIX` `__PRIPTR_PREFIX`, every
  `REPLACE_*` 0 and every `HAVE_DECL_*` 1 (the `GNULIB_*` selectors are unset, so the generated
  file's blocks read `#if 0` except `strtoimax`'s, which reads `#if 1`).
- **Every name the template defines is nevertheless dead text on this target, and the reason is
  the include order** (measured, Evidence): the generated header's C 41 `# include_next
  <inttypes.h>` pulls the *system* header, whose own `#include <stdint.h>` resolves to gnulib's
  generated `stdint.h`; that file defines `_GL_JUST_INCLUDE_SYSTEM_INTTYPES_H` (its C 35) before
  including `<inttypes.h>`, and the generated header's body is guarded by C 47's
  `#if ! defined INTTYPES_H && ! defined _GL_JUST_INCLUDE_SYSTEM_INTTYPES_H` - so C 47-1574 is
  skipped, `INTTYPES_H` is never defined, and the `PRI*`/`SCN*` names come from the system header.
  The frozen values are therefore measured with the configuration's compiler, one name per line,
  `__STR`-wrapped so an empty or undefined expansion is still one line (the command is in
  Evidence; 155 of the 167 names are defined afterwards).
- **The four shapes the mirror uses** (167 names = 1 guard + 53 `same` + 57 `diff/literal` +
  45 `diff/expression` + 11 `undefined`):
  * `same` (53): C's fallback and the frozen value agree, so one ungated constant answers for both
    arms (the `fcntl.in.rs` `FD_CLOEXEC` shape) - all of `SCN*` except the 64/MAX/PTR families,
    and `PRIx32`/`PRIX32`/`PRIu32`/… whatever the system spells identically.
  * `diff/literal` (57): C's body is a literal and the system adds a length modifier, so C's value
    is a **gated constant** - `tools/extract-abi.py` compares a constant's value against C's, which
    is why the C value cannot be a macro - and the frozen spelling is a `macro_rules!` of the same
    name (the other namespace) whose `concat! (...)` contains C's literal, so the macro-body
    comparison is satisfied too.  These are the 8- and 16-bit `PRI*` families plus
    `PRI*MAX`/`SCN*FAST8`/`SCN*FAST16`/`SCN*MAX`.
  * `diff/expression` (45): C's body is an expression (`_PRI64_PREFIX "d"`, `PRId64`,
    `@PRIPTR_PREFIX@ "d"`, `_LONG_LONG_FORMAT_PREFIX`), so the gated arm is a `macro_rules!`
    naming C's tokens (the checker compares the *first* definition) and the frozen spelling is the
    second, ungated macro - 64/MAX/PTR and the `LEAST`/`FAST` 64 families.
  * `undefined` (11): this target defines no such name at all, so only the gated arm exists:
    `__STDC_FORMAT_MACROS` (C 36-38's C++-only define), `_LONG_LONG_FORMAT_PREFIX` and the four
    `_*64_PREFIX` helpers, `GNULIB_defined_imaxdiv_t` and the four rename macros
    `imaxabs`/`imaxdiv`/`strtoimax`/`strtoumax` (their live arms are the system declarations, not
    macros).
  The features are named after the name they carry (`no_prid8`, `no_pri64_prefix`, ...), the
  `no_<name>` convention `include-dirent.in.md`/`include-fcntl.in.md` use for their `#ifndef` arms.
- **What the text check cannot see, and what pins it instead.**  `--check` compares the *first*
  definition of a name, so for the 57 `diff/literal` and 45 `diff/expression` names it compares
  C's arm and says nothing about the ungated frozen arm.  The `/tmp` probe is that guard: it
  `include!`s the mirror, asserts every one of the 166 features off (P-51) and pins all 154
  non-empty frozen values with `assert_eq!` (Evidence).  The mirror's own `--check` is `OK`.
- **Not carried** (PORTING.md 0.1; the `include-dirent.in.md` "Not carried" paragraph): C 23-25's
  `__GNUC__ >= 3` test with the two pragma placeholders, C 32-45's and C 47's guard pair, C 50-52's
  `_GL_CONFIG_H_INCLUDED` check, C 56-58's `<stdint.h>`, C 60's `<limits.h>`, C 62-64's
  `_WIN32`-only `<stdio.h>`, C 66-69's `int`-width check and the comment blocks have no Rust
  counterpart; the `_GL_*`/`_GL_CXXALIAS_*`/`_GL_FUNCDECL_*`/`_GL_WARN_ON_USE` text the build
  inlines from `c++defs.h`/`arg-nonnull.h`/`warn-on-use.h` belongs to those headers' own mirrors
  (`c++defs.rs`, `arg-nonnull.rs`); and C 900-1029's four function blocks define no name of their
  own beyond the four rename macros carried above, their live arms being the system declarations.
- **No shared-table row is needed**: every name has a Rust item, none is in `excluded-macros.txt`,
  and the three `_`-reading guards are already covered by the existing `_` row.

## Evidence

```
$ ls -la build/emacs-ref/lib/inttypes.h && head -1 build/emacs-ref/lib/inttypes.h
-rw-r--r-- 1 liu staff 46752 Oct  3 04:44 build/emacs-ref/lib/inttypes.h
/* DO NOT EDIT! GENERATED AUTOMATICALLY! */

$ grep -m1 '^S\["\(HAVE_INTTYPES_H\|APPLE_UNIVERSAL_BUILD\|PRIPTR_PREFIX\|INT64_MAX_EQ_LONG_MAX\)"\]' \
    build/emacs-ref/config.status
S["HAVE_INTTYPES_H"]="1"          S["APPLE_UNIVERSAL_BUILD"]="0"
S["PRIPTR_PREFIX"]="__PRIPTR_PREFIX"   S["INT64_MAX_EQ_LONG_MAX"]="defined _LP64"

# why the body is skipped: the include order, and the guard gnulib's stdint.h arms
$ clang -std=gnu23 -I build/emacs-ref/lib -E -include inttypes.h /tmp/empty.c \
    | grep -nE '^# [0-9]+ "(.*(inttypes|stdint)\.h|/tmp/empty\.c)"' | head -4
1:# 1 "/tmp/empty.c"
7:# 1 "build/emacs-ref/lib/inttypes.h" 1
9:# 41 "build/emacs-ref/lib/inttypes.h" 3
10:# 1 ".../usr/lib/clang/21/include/inttypes.h" 1 3
226:# 1 "build/emacs-ref/lib/stdint.h" 1 3 4
$ grep -n '_GL_JUST_INCLUDE_SYSTEM_INTTYPES_H' build/emacs-ref/lib/stdint.h | head -2
35:#define _GL_JUST_INCLUDE_SYSTEM_INTTYPES_H
119:# undef _GL_JUST_INCLUDE_SYSTEM_INTTYPES_H

# the frozen state and the frozen values, one name per line, `__STR`-wrapped
$ clang -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src -I emacs/lib \
    -include config.h -include inttypes.h -E -dM /tmp/empty.c | grep -E 'PREFIX|INTTYPES_H'
#define _GL_FINISHED_INCLUDING_SYSTEM_INTTYPES_H
#define _GL_JUST_INCLUDE_SYSTEM_INTTYPES_H
(no `INTTYPES_H`, no `_PRI64_PREFIX`, no `_LONG_LONG_FORMAT_PREFIX`)
$ clang ... -E -P /tmp/itt_expand2.c | tail -167     # `__STR(NAME)` per line, 167/167 lines
PRId8 => "hh" "d"     PRId16 => "hd"      PRId32 => "d"      PRId64 => "ll" "d"
PRIdMAX => "j" "d"    PRIdPTR => "ld"     PRIxMAX => "j" "x"  SCNd8 => "hh" "d"
SCNd64 => "ll" "d"    SCNdMAX => "j" "d"  SCNxPTR => "lx"     (155 defined, 12 undefined)

$ python3 tools/extract-abi.py --check emacs/lib/inttypes.in.h femacs/lib/include/inttypes.in.rs
  (49 C macros have several definitions (configuration branches), body not compared: ...)
OK emacs/lib/inttypes.in.h -> femacs/lib/include/inttypes.in.rs

# the negative controls: C's side is compared, and the checker bites
$ sed 's/pub const PRId8: &core::ffi::CStr = c"d";/pub const PRId8: \&core::ffi::CStr = c"q";/' \
    femacs/lib/include/inttypes.in.rs > /tmp/gt-neg-itt/c1.rs
$ python3 tools/extract-abi.py --check emacs/lib/inttypes.in.h /tmp/gt-neg-itt/c1.rs
- constant PRId8: C "d" vs Rust c"q" (different bytes)
FAIL emacs/lib/inttypes.in.h -> /tmp/gt-neg-itt/c1.rs
$ sed 's/  () => { concat! (_PRI64_PREFIX! (), "d") };/  () => { concat! ("l", "d") };/' \
    femacs/lib/include/inttypes.in.rs > /tmp/gt-neg-itt/c2.rs
$ python3 tools/extract-abi.py --check emacs/lib/inttypes.in.h /tmp/gt-neg-itt/c2.rs
- macro PRId64 does not mention _pri64_prefix (C body: _PRI64_PREFIX "d")
FAIL emacs/lib/inttypes.in.h -> /tmp/gt-neg-itt/c2.rs
$ sed 's/pub const PRId32: &core::ffi::CStr = c"d";/pub const PRId32: \&core::ffi::CStr = c"q";/' \
    femacs/lib/include/inttypes.in.rs > /tmp/gt-neg-itt/c5.rs        # a `same`-case value
$ python3 tools/extract-abi.py --check emacs/lib/inttypes.in.h /tmp/gt-neg-itt/c5.rs
- constant PRId32: C "d" vs Rust c"q" (different bytes)
FAIL emacs/lib/inttypes.in.h -> /tmp/gt-neg-itt/c5.rs
$ sed 's/pub const GNULIB_defined_imaxdiv_t: c_int = 1;/pub const GNULIB_defined_imaxdiv_t: c_int = 2;/' \
    femacs/lib/include/inttypes.in.rs > /tmp/gt-neg-itt/c6.rs
$ python3 tools/extract-abi.py --check emacs/lib/inttypes.in.h /tmp/gt-neg-itt/c6.rs
- constant GNULIB_defined_imaxdiv_t: C 1 vs Rust 2
FAIL emacs/lib/inttypes.in.h -> /tmp/gt-neg-itt/c6.rs
# and the measured gap, recorded rather than papered over: mutating an *ungated* frozen macro
# (`concat! ("hX")` -> `concat! ("hZ")`) is not reported, because the checker compares the first
# definition of the name (`/tmp/gt-neg-itt/c4.rs` -> OK); that arm is the probe's below.

# the probe: every dead feature off, every frozen value pinned.  It is a transient /tmp crate
# (the `errno.in`/`dirent.in` practice - not committed), built from the mirror plus the measured
# table above: `#![allow(…)]`, `include!` of the mirror, one
# `const _: () = assert! (!cfg! (feature = "…"));` per gated arm, and one
# `assert_eq! (NAME! (), "…")` / `assert_eq! (NAME.to_bytes (), b"…")` per defined name.
$ cd /tmp/femacs-gt-probe-inttypes && rustup run nightly-2026-10-01 rustc --edition 2024 \
    --crate-type bin -C panic=abort -o probe_inttypes probe_inttypes.rs
$ ./probe_inttypes
exit=0        # 166 `assert! (!cfg! (feature = "…"))` lines and 154 value assertions

# one consumer reads the mirror end to end (its C file includes <inttypes.h>)
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-gt cargo run -q -p xtask -- build-tu lib-src/hexl.c
built lib-src/hexl.rs -> ../build/femacs/hexl.o
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-gt cargo run -q -p xtask -- build-tu lib/strtoimax.c
built lib/strtoimax.rs -> ../build/femacs/strtoimax.o
$ python3 tools/extract-abi.py --check emacs/lib-src/hexl.c femacs/lib-src/hexl.rs
OK emacs/lib-src/hexl.c -> femacs/lib-src/hexl.rs
$ python3 tools/extract-abi.py --check emacs/lib/strtoimax.c femacs/lib/strtoimax.rs
OK emacs/lib/strtoimax.c -> femacs/lib/strtoimax.rs

# the remaining units whose C file includes <inttypes.h> (the integrator's sweep, one line each)
$ rg -n '#include <inttypes.h>' emacs/lib emacs/lib-src
emacs/lib/strtoimax.c:24   (wired)     emacs/lib-src/hexl.c:24        (wired)
emacs/lib-src/etags.c:108  emacs/lib-src/emacsclient.c:75  emacs/lib-src/update-game-score.c:38

# the file is a gnulib template with `@PLACEHOLDER@` conditions, so it stays off the verified
# list (P-66) and `--check` reads every arm as live - the reason the mirror carries C's arms
$ python3 tools/check-cond-eval.py lib/inttypes.in.h
lib/inttypes.in.h: 321 conditions, 18 unknown to the evaluator, 20 disagreements [NOT VERIFIED]
```
