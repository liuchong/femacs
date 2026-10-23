# emacs/src/lcms.c (604 lines)

> **This ledger overturns the entry it replaces.**  The previous revision of this file recorded
> `lcms.c` as an empty unit under P-08 ("the whole body is inside a disabled `#if`, so nothing is
> demanded of the port"), with the checklist sections all at zero.  **P-86 rules that the other
> way**: a file whose body sits entirely under a condition the frozen configuration leaves false is
> *ported by transcription*, both arms of every `#if` carried, and its ledger records a **C
> line-range table** instead of ticked boxes, because `extract-abi.py`'s checklist for it is empty
> (P-73's live-line filter; the file *is* in `docs/baseline/config/cond-eval-verified.txt` 320) and
> the checkbox sections are therefore not coverage evidence.  The port is `src/lcms.rs`
> (877 lines), the file gate is `feature = "have_lcms2"`, and the frozen `build-tu` reading below
> still has to be the empty object it was - the transcription must not leak a line into this
> configuration.

`HAVE_LCMS2` is `undef` (`docs/baseline/config/darwin-arm64.macros.tsv` 1412); `WINDOWSNT` is
`undef` (4010).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`#ifdef HAVE_LCMS2`, C 21) | `feature = "have_lcms2"` (on every item and on C 23-26's include graph) | an lcms2 configuration |
| C 35 `#ifdef WINDOWSNT` … C 90 (DLL loading, `fn_*` renames) | `all (feature = "have_lcms2", feature = "windowsnt")` | a Windows configuration |
| the eight `#ifdef WINDOWSNT` blocks inside the DEFUN bodies (C 125-133, 329-337, 366-374, 406-414, 447-455, 502-510, 547-555) and C 570-584 (`lcms2-available-p`'s two arms) | `feature = "windowsnt"` / `not (feature = "windowsnt")`, nested in the file gate | both arms carried |
| C 82-88 (`#define cms… fn_cms…` × 7) | `windowsnt`, realized as the arms' two call spellings (see Deviations) | - |

## C line ranges

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's licence/declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line, PORTING.md 0.1-19) |
| 19 | `#include <config.h>` | ungated (every unit includes it) |
| 21 | `#ifdef HAVE_LCMS2` | the file gate |
| 23-24 | `#include <lcms2.h>`, `#include <math.h>` | file gate; `lcms2.h` has no file in the reference tree, so its declarations are carried in the unit (P-88), `<math.h>` is the `lib/include/math.rs` mirror |
| 26 | `#include "lisp.h"` | file gate |
| 28-33 | `lcmsJab_t` (the file's own type) | file gate |
| 35-90 | `#ifdef WINDOWSNT`: C 36-38 the three includes, C 40-52 the seven `DEF_DLL_FN`, C 54 `lcms_initialized`, C 56-72 `init_lcms_functions`, C 74-80 the `#undef`s, C 82-88 the seven `#define` renames | `windowsnt` (nested) |
| 92-109 | `parse_lab_list` (C 95-102 the `PARSE_LAB_LIST_FIELD` macro) | file gate |
| 111 | the ciede2000 reference URL | file gate (kept as the comment C has) |
| 113-153 | `DEFUN ("lcms-cie-de2000", …)` | file gate / `windowsnt` (C 125-133) |
| 155-159 | `deg2rad` | file gate |
| 161-165 | `rad2deg` | file gate |
| 167 | `illuminant_d65` | file gate |
| 169-179 | `default_viewing_conditions` | file gate |
| 181 | `/* FIXME: code duplication */` | file gate (kept as the comment C has) |
| 183-200 | `parse_xyz_list` (C 186-193 the macro) | file gate |
| 202-221 | `parse_jch_list` (C 205-212 the macro) | file gate |
| 223-240 | `parse_jab_list` (C 226-233 the macro) | file gate |
| 242-275 | `parse_viewing_conditions` (C 246-261 the two macros) | file gate |
| 277-285 | `xyz_to_jch` | file gate |
| 287-295 | `jch_to_xyz` | file gate |
| 297-304 | `jch_to_jab` | file gate |
| 306-316 | `jab_to_jch` | file gate |
| 318-352 | `DEFUN ("lcms-xyz->jch", …)` | file gate / `windowsnt` (C 329-337) |
| 354-391 | `DEFUN ("lcms-jch->xyz", …)` | file gate / `windowsnt` (C 366-374) |
| 393-432 | `DEFUN ("lcms-jch->jab", …)` | file gate / `windowsnt` (C 406-414) |
| 434-473 | `DEFUN ("lcms-jab->jch", …)` | file gate / `windowsnt` (C 447-455) |
| 475-479 | the two literature references | file gate (kept as the comment C has) |
| 481-536 | `DEFUN ("lcms-cam02-ucs", …)` | file gate / `windowsnt` (C 502-510) |
| 538-564 | `DEFUN ("lcms-temp->white-point", …)` | file gate / `windowsnt` (C 547-555) |
| 566-585 | `DEFUN ("lcms2-available-p", …)` | file gate / `windowsnt` (C 570-584, whose `#else` arm is C 582-584) |
| 588-602 | `syms_of_lcms2` (C 592-599 the eight `defsubr`s, C 601 `Fprovide`) | file gate |
| 604 | `#endif /* HAVE_LCMS2 */` | the file gate's end |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/femacs-lc-target cargo run -q -p xtask -- build-tu src/lcms.c`
→ `built src/lcms.rs -> ../build/femacs/lcms.o`, **exit 0**, and `build/femacs/lcms.o` is **336
bytes** - an empty object, unchanged from the empty-unit revision.  Every item sits behind
`have_lcms2`, so the frozen feature set compiles only `include/config.rs` and the `#![allow]`.

**② The arm flip (P-86 ③⑴).**
`cargo run -q -p xtask -- probe src/lcms.rs /tmp/femacs-port-probe/lcms_x.o --crate-type lib
--emit obj -C panic=abort --cfg 'feature="have_lcms2"'`
→ **exit 0**, **0 errors**, 3 warnings (2 `unused doc comment`, both in mirror copies -
`lib/include/stdbit.in.rs` 644 and `include/lisp.rs` 3934; 1 `` `extern` block uses type
`_cmsContext_struct`, which is not FFI-safe ``), object **13,112 bytes**.  `nm -g` shows the eight
subrs and the file's one other exported definition:
`T _Flcms_cie_de2000`, `T _Flcms_xyz_to_jch`, `T _Flcms_jch_to_xyz`, `T _Flcms_jch_to_jab`,
`T _Flcms_jab_to_jch`, `T _Flcms_cam02_ucs`, `T _Flcms_temp_to_white_point`,
`T _Flcms2_available_p`, `T _syms_of_lcms2`, with `t …parse_lab_list` internal as C's `static`
makes it - and the references are exactly the seven lcms2 entry points and the eight `<math.h>`
functions: `U _cmsCIE2000DeltaE`, `U _cmsCIECAM02Init`, `U _cmsCIECAM02Forward`,
`U _cmsCIECAM02Reverse`, `U _cmsCIECAM02Done`, `U _cmsWhitePointFromTemp`, `U _cmsxyY2XYZ`,
`U _cbrt`, `U _hypot`, `U _sqrt`, `U _cos`, `U _sin`, `U _atan2`, `U _log`, `U _exp`, plus the
Lisp-side `U _make_float`, `U _list3`, `U _signal_error`, `U _wrong_type_argument`,
`U _check_integer_range`, `U _defsubr`, `U _Fprovide`, `U _intern_c_string_1`, `U _Vlibrary_cache`,
`U _xfree`, `U __setjmp`.  **0 errors are in a mirror copy and 0 in the unit's own text.**

**③ The `windowsnt` arm is `text-only` (P-86 ③⑵).**  This machine has **no Windows cross
toolchain** (`which x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc` → empty), so C 35-90 and the eight
nested blocks cannot be compiled from this host.  What is recorded instead: every item of the arm
is under `all (have_lcms2, windowsnt)`, so reading ① is unaffected and reading ②'s `0 errors` is
*not* a reading of this arm.

**④ `<lcms2.h>`'s layout, measured (not remembered).**  The unit's P-88 section carries the types
and constants with clang readings from this machine's header
(`/opt/homebrew/include/lcms2.h` → `Cellar/little-cms2/2.19.1/include/lcms2.h`):
```
$ clang -I/opt/homebrew/include -o /tmp/femacs-port-probe/lprobe /tmp/femacs-port-probe/lprobe.c && /tmp/femacs-port-probe/lprobe
LCMS_VERSION=2190
sizeof(cmsCIELab)=24 off L=0 a=8 b=16
sizeof(cmsCIEXYZ)=24 off X=0 Y=8 Z=16
sizeof(cmsJCh)=24 off J=0 C=8 h=16
sizeof(cmsCIExyY)=24 off x=0 y=8 Y=16
sizeof(cmsViewingConditions)=56 off whitePoint=0 Yb=24 La=32 surround=40 D_value=48
sizeof(cmsFloat64Number)=8 sizeof(cmsHANDLE)=8 sizeof(cmsContext)=8 sizeof(cmsBool)=4
sizeof(cmsCIExyYTRIPLE)=72 sizeof(cmsUInt32Number)=4
AVG_SURROUND=1 DIM_SURROUND=2 DARK_SURROUND=3 CUTSHEET_SURROUND=4
cmsMAX_PATH=256 FALSE=0 TRUE=1
```
(the same readings appear in the unit's section comment):

```c
/* /tmp/<prefix>/lprobe.c - the layout probe (the task's P-88 ② shape); the temporary
   directory is cleaned up, so the source is kept here.  */
#include <lcms2.h>
#include <stdio.h>
#include <stddef.h>
int main (void)
{
  printf ("LCMS_VERSION=%d\n", LCMS_VERSION);
  printf ("sizeof(cmsCIELab)=%zu off L=%zu a=%zu b=%zu\n", sizeof (cmsCIELab),
          offsetof (cmsCIELab, L), offsetof (cmsCIELab, a), offsetof (cmsCIELab, b));
  printf ("sizeof(cmsCIEXYZ)=%zu off X=%zu Y=%zu Z=%zu\n", sizeof (cmsCIEXYZ),
          offsetof (cmsCIEXYZ, X), offsetof (cmsCIEXYZ, Y), offsetof (cmsCIEXYZ, Z));
  printf ("sizeof(cmsJCh)=%zu off J=%zu C=%zu h=%zu\n", sizeof (cmsJCh),
          offsetof (cmsJCh, J), offsetof (cmsJCh, C), offsetof (cmsJCh, h));
  printf ("sizeof(cmsCIExyY)=%zu off x=%zu y=%zu Y=%zu\n", sizeof (cmsCIExyY),
          offsetof (cmsCIExyY, x), offsetof (cmsCIExyY, y), offsetof (cmsCIExyY, Y));
  printf ("sizeof(cmsViewingConditions)=%zu off whitePoint=%zu Yb=%zu La=%zu surround=%zu D_value=%zu\n",
          sizeof (cmsViewingConditions), offsetof (cmsViewingConditions, whitePoint),
          offsetof (cmsViewingConditions, Yb), offsetof (cmsViewingConditions, La),
          offsetof (cmsViewingConditions, surround), offsetof (cmsViewingConditions, D_value));
  printf ("sizeof(cmsFloat64Number)=%zu sizeof(cmsHANDLE)=%zu sizeof(cmsContext)=%zu sizeof(cmsBool)=%zu\n",
          sizeof (cmsFloat64Number), sizeof (cmsHANDLE), sizeof (cmsContext), sizeof (cmsBool));
  printf ("sizeof(cmsCIExyYTRIPLE)=%zu sizeof(cmsUInt32Number)=%zu\n",
          sizeof (cmsCIExyYTRIPLE), sizeof (cmsUInt32Number));
  printf ("AVG_SURROUND=%d DIM_SURROUND=%d DARK_SURROUND=%d CUTSHEET_SURROUND=%d\n",
          AVG_SURROUND, DIM_SURROUND, DARK_SURROUND, CUTSHEET_SURROUND);
  printf ("cmsMAX_PATH=%d FALSE=%d TRUE=%d\n", cmsMAX_PATH, FALSE, TRUE);
  return 0;
}
```


**⑤ The gates (repo root, readings taken on this revision).**  The worktree is shared with other
lanes (P-87 ②), so the repo-wide counts move.  `python3 tools/check-dead-file-coverage.py` →
`5 ledger(s) bound (a C range table), 0 failure(s)` (this file's 33 rows cover `src/lcms.c`, 604
lines, 32 blank line(s) between blocks need no row). — `python3 tools/check-port-lines.py` → `92
file(s) under ten percent, 243,405 reference line(s); 95 ratchet row(s), 0 new, 3 stale`, one of
the stale rows being `src/lcms.c: no longer under the threshold` (the others are two other
lanes' rows); `--write-pending` is the integrator's step. — `python3
tools/check-citations.py` → `675 file(s) scanned, 1 drift, 0 range, 456 external, 33 allowed, 0
stale`; the one drift is `src/androidselect.rs:97` (another lane's file), **0 findings name
this unit**, and the external count rose from 350 to 456 because the `lcms2.h`/`math.h` citations
name headers the reference tree does not carry. — `python3 tools/check-declarations.py` → `13
declaration sites (0 forbidden, 13 allowed literals), 0 failures`. — `python3
tools/check-carried.py` → `11 carried file(s), 11 baseline row(s), 0 failure(s)`. — `python3
tools/check-ledger-ticks.py` → `235 ledger(s) bound (a function tick to compare), 0 failure(s)`;
this ledger carries no function tick to bind.

**⑤a The `#if` decisions.**  `python3 tools/check-cond-eval.py src/lcms.c` → `10 conditions, 0
unknown, 0 disagreements [VERIFIED]`: the guard, `HAVE_LCMS2`, `WINDOWSNT` and the eight
nested blocks are all decided by the frozen table, which is why the Gate mapping above and
the port's `#[cfg]`s agree item for item.

**⑥ `extract-abi.py --check` FAILs, and the seventeen lines are P-86 ⑩'s owed baseline row.**
`python3 tools/extract-abi.py --check emacs/src/lcms.c src/lcms.rs` → **FAIL**, with
exactly these seventeen lines and no others:
```
- DEFUN! Flcms2_available_p: the reference file has no such DEFUN
- DEFUN! Flcms_cam02_ucs: the reference file has no such DEFUN
- DEFUN! Flcms_cie_de2000: the reference file has no such DEFUN
- DEFUN! Flcms_jab_to_jch: the reference file has no such DEFUN
- DEFUN! Flcms_jch_to_jab: the reference file has no such DEFUN
- DEFUN! Flcms_jch_to_xyz: the reference file has no such DEFUN
- DEFUN! Flcms_temp_to_white_point: the reference file has no such DEFUN
- DEFUN! Flcms_xyz_to_jch: the reference file has no such DEFUN
- unexpected defsubr (&Slcms2_available_p): the reference file does not register it
- unexpected defsubr (&Slcms_cam02_ucs): the reference file does not register it
- unexpected defsubr (&Slcms_cie_de2000): the reference file does not register it
- unexpected defsubr (&Slcms_jab_to_jch): the reference file does not register it
- unexpected defsubr (&Slcms_jch_to_jab): the reference file does not register it
- unexpected defsubr (&Slcms_jch_to_xyz): the reference file does not register it
- unexpected defsubr (&Slcms_temp_to_white_point): the reference file does not register it
- unexpected defsubr (&Slcms_xyz_to_jch): the reference file does not register it
- fn syms_of_lcms2 carries #[unsafe(no_mangle)] but the reference does not define a non-static function of that name (invented exported symbol)
```
Every line is the same finding, and it is the same one `src/decompress.rs` reports: the file
is in `cond-eval-verified.txt` (320), the frozen table decides `HAVE_LCMS2` as `undef`, P-73's
live-line filter therefore yields an **empty** item list for the *reference* while the port
declares the items the dead arm contains, so each correct item reads as invented.  The checklist
cannot be the evidence here (P-86 ②); the extractor needs the rule (or baseline row) that says
"this file's whole body is dead - compare the port against the file's text, not the live lines".
One difference from `decompress.rs`, which is worth recording: this port's eight `static` helpers
are `fn`s that are **not** exported, so they draw no "invented exported symbol" line; only
`syms_of_lcms2` does.

## Deviations

- **The `<lcms2.h>` section is the stand-in for an external header** (P-88 ④): the reference tree
  has no `lcms2.h`, so the types, constants and prototypes this unit uses live in the unit at C 23's
  position, each carrying the header's own line (P-55), with the layout measured by the clang probe
  in Evidence ④.  A future tree that grows an `lcms2.h` mirror moves the section into it and deletes
  the copy in the same commit (P-88 ⑤).
- **`DEF_DLL_FN`/`LOAD_DLL_FN` are written in long form** (C 40-52, C 64-70), the same way
  `src/decompress.rs` (their first consumer) writes them: C pastes `W32_PFN_##func` /
  `fn_##func` from one identifier and takes the parameter list as `__VA_ARGS__`, which Rust cannot
  express, and `emacs/src/w32common.h` 77-97 is a `kind=deferred` row in
  `docs/ports/excluded-macros.txt` (P-22).  The two generated names are macro parameters, the
  parameter list a `ty` fragment, the loader's cast a `transmute` (Rust has no `as` cast to an
  `Option<fn>`; both types are one pointer wide), and C's `#func` a `c"…"` literal.  **The two
  macros are defined in this unit *and* in `decompress.rs`** because a unit is its own crate; a
  third consumer is what should move them into a shared prelude (PORTING.md 0.1-5).  Nothing of
  this arm is compiled on this host (Evidence ③).
- **`#define cmsCIE2000DeltaE fn_cmsCIE2000DeltaE` and its six siblings (C 82-88) have no Rust
  counterpart** - an object-like `#define` renames an identifier.  The port spells C's
  *preprocessed* text per arm at the call sites (`fn_cms… (…)` under `windowsnt`, `cms… (…)`
  otherwise; the `ccl.rs` "C's preprocessed text" rule), while the seven pointer objects keep C's
  own names.  The seven call sites are C 152, C 282-284, C 292-294, C 560/562 and the `fread`-free
  bodies of the DEFUNs; `#undef`s (C 74-80) have no counterpart.
- **The five `static` parse helpers are `unsafe fn`, and C's four in-body `#define`s are
  `macro_rules!` inside the same body** (the `androidmenu.rs` `FIND_METHOD` shape).  C never
  `#undef`s them, but each is used only in its own function, so Rust's block scope is equivalent;
  C's macro assigns its parameter (`lab_list = XCDR (lab_list)`), so the port rebinds the parameter
  mutable *before* the macro definition, because a `macro_rules!` resolves its names where it is
  defined.
- **Uninitialised locals are zeroed (PORTING.md 1.5/0.1-13).**  Named here: `Lab1`/`Lab2` (C 122),
  `Kl`/`Kc`/`Kh` (C 123), `vc`/`jch`/`xyz`/`xyzw` in the five view-condition DEFUNs (C 325-327,
  362-364, 400-404, 441-445, 496-500), `FL`/`k`/`k4` (C 404-405, 445, 500), `whitepoint`/`wp`/`tempK`
  (C 543-545) and `h` in `xyz_to_jch`/`jch_to_xyz` (C 280, 290), each of which C fills before its
  first read; `static bool lcms_initialized` (C 54) is C's tentative definition, i.e. zeroed, and
  `illuminant_d65`'s C designated initializer is the port's struct literal.
- **`Copy` on the five lcms2 records is C's trivial copyability** (PORTING.md 1.3), which the
  DEFUNs' `xyzw = illuminant_d65;` needs (a `static` read then assigns by value).
- **Numeric spellings the port makes explicit**: `1.0f64` for C's `1.0f` (C 140, 144, 148),
  `20.0`/`100.0`/`1.0` for C's integer-literal assignments to `cmsFloat64Number` members
  (C 175-178), `AVG_SURROUND as cmsUInt32Number` (C 177), and
  `check_integer_range (…) as cmsUInt32Number` for C's implicit `intmax_t` narrowing (C 257);
  C's `!(cmsWhitePointFromTemp (…))` (C 560) is `== 0`, and `cmsCIECAM02Init (0, vc)` (C 282, 292)
  is a null `cmsContext`.
- **Three `<math.h>` names are declared at the use site (P-72 rule 4)**: `M_PI` (math.h 751),
  `cbrt` (math.h 455) and `hypot` (math.h 459) are not in `lib/include/math.rs` yet, so the
  unit carries them - `M_PI` as a `const`, the two functions as `unsafe extern "C"` - with their
  header lines.  They belong in that mirror when it is next extended.
- **Line count.**  C 604 vs Rust 895 (48.2%): P-86's per-item `#[cfg]` gates (both arms carried),
  the P-88 declaration section and the per-arm spellings; `extract-abi`'s line-count row does not
  fire here (the reference's live set is empty - Evidence ⑥).  A
  `docs/ports/line-drift-tolerance.tsv` row is owed by the integrator if the rule later binds this
  class.

## Owed by the integrator (nothing here is claimed as done)

1. `tools/extract-abi.py`: a rule (or a baseline row) for a file whose whole body is dead - see
   Evidence ⑥.  Seventeen lines, all one finding (the `decompress.rs` ledger records the same
   class, with six).
2. `tools/check-port-lines.py --write-pending`: the `src/lcms.c` row is stale (retired).
3. `lib/include/math.rs`: `M_PI`, `cbrt` and `hypot` (Evidence's Deviations; the unit
   declares them meanwhile and must drop its copies in the commit that adds them - P-70/P-88 ⑤).
4. `docs/ports/excluded-macros.txt`: the `DEF_DLL_FN`/`LOAD_DLL_FN` rows stay `kind=deferred`;
   their reason text can now name the two consuming units, and the third consumer should move the
   macros into a shared prelude.
5. `docs/baseline/feature-coverage.tsv` (generated): gains the `have_lcms2` row.
6. `docs/port-map.md` (generated): the target's line count for `src/lcms.c` is stale (12).
