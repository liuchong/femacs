# emacs/src/conf_post.h (445 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 66
- [x] ADDRESS_SANITIZER  (line 93, body 7cb6efb98ba5)
- [x] ADDRESS_SANITIZER_WORKAROUND  (line 246, body ce8c863dbf92)
- [x] ARG_NONNULL  (line 218, body ee603f578f7b)
- [x] ATTRIBUTE_FORMAT_PRINTF  (line 215, body c09082baac3a)
- [x] ATTRIBUTE_MALLOC_SIZE  (line 234, body 774eb9c22e13)
- [x] ATTRIBUTE_NO_SANITIZE_ADDRESS  (line 259, body da39a3ee5e6b)
- [x] ATTRIBUTE_NO_SANITIZE_UNDEFINED  (line 270, body da39a3ee5e6b)
- [x] ATTRIBUTE_SECTION  (line 231, body da39a3ee5e6b)
- [x] DATA_START  (line 157, body 10a28a6a830b)
- [x] DECLARE_POINTER_ALIAS  (line 225, body 38f098829d6a)
- [x] DebPrint  (line 165, body a29646b15239)
- [x] ENOTSUP  (line 388, body 2e4c602e5ae5)
- [x] EOPNOTSUPP  (line 146, body bf1e810d7d52)
- [x] EOVERFLOW  (line 126, body 027a5d6bd60d)
- [x] EXTERNALLY_VISIBLE  (line 186, body bc3484efd7f1)
- [x] EXTERN_INLINE  (line 360, body 750f8892694e)
- [x] GNUC_PREREQ  (line 49, body 749985c0d5b6)
- [x] GNULIB_SUPPORT_ONLY_AT_FDCWD  (line 143, body da39a3ee5e6b)
- [x] HAS_ATTRIBUTE  (line 73, body 13f0d72edf24)
- [x] HAS_ATTR_cleanup  (line 74, body 79a417f37c56)
- [x] HAS_ATTR_no_address_safety_analysis  (line 75, body 7cb6efb98ba5)
- [x] HAS_ATTR_no_sanitize  (line 76, body 7cb6efb98ba5)
- [x] HAS_ATTR_no_sanitize_address  (line 77, body 6b4c10ccd2a1)
- [x] HAS_ATTR_no_sanitize_undefined  (line 78, body fd506ae29751)
- [x] HAS_FEATURE  (line 86, body 7cb6efb98ba5)
- [x] HAVE_LSTAT  (line 122, body 356a192b7913)
- [x] IFTODT  (line 389, body aa6c9c53cdd8)
- [x] INLINE  (line 358, body 21a632056285)
- [x] INLINE_HEADER_BEGIN  (line 361, body da39a3ee5e6b)
- [x] INLINE_HEADER_END  (line 362, body da39a3ee5e6b)
- [x] NDEBUG  (line 29, body da39a3ee5e6b)
- [x] NO_INLINE  (line 185, body 42374afc111d)
- [x] NTGUI_UNICODE  (line 170, body da39a3ee5e6b)
- [x] PRINTF_ARCHETYPE  (line 213, body c1bb0190e235)
- [x] REQUIRE_GNUISH_STRFTIME_AM_PM  (line 381, body 7cb6efb98ba5)
- [x] SIZE_MAX  (line 127, body f1711e6df3e6)
- [x] SUPPORT_NON_GREG_CALENDARS_IN_STRFTIME  (line 382, body 7cb6efb98ba5)
- [x] UNINIT  (line 371, body da39a3ee5e6b)
- [x] VFORK  (line 290, body 8e1688c9a24e)
- [x] _GL_DT_NOTDIR  (line 387, body bd60cfec3dba)
- [x] _NAIVE_DOS_REGS  (line 112, body da39a3ee5e6b)
- [x] _REGEX_AVOID_UCHAR_H  (line 397, body 356a192b7913)
- [x] _WIN32_WINNT  (line 171, body bccfd9402ab8)
- [x] btowc  (line 134, body 1323437e30c6)
- [x] emacs_raise  (line 154, body ad50790a11eb)
- [x] fprintf  (line 432, body 6232691acf83)
- [x] fwrite  (line 431, body ef0701aead85)
- [x] getenv_TZ  (line 180, body bbb20eb09653)
- [x] iswalnum  (line 137, body 6c7b58e205dd)
- [x] iswctype  (line 139, body 7cb6efb98ba5)
- [x] likely  (line 377, body 1370e2d51ff0)
- [x] lstat  (line 124, body fc165ac76180)
- [x] mbrtowc  (line 132, body 13f2791bfb71)
- [x] mbsinit  (line 140, body 356a192b7913)
- [x] opendir  (line 150, body 26f542b6b968)
- [x] printf  (line 433, body 2993eec38537)
- [x] setenv_TZ  (line 181, body 3ea8a8a3387c)
- [x] strnicmp  (line 174, body 017ba27bde23)
- [x] strtold  (line 119, body 849a7327aa77)
- [x] towlower  (line 136, body b0426753ead2)
- [x] towupper  (line 135, body 9d2a2dc29cf5)
- [x] vfork  (line 277, body 3f673f2a8e0a)
- [x] vfprintf  (line 434, body d6297140058f)
- [x] vprintf  (line 435, body c7dd1f43f598)
- [x] wcrtomb  (line 133, body 5e8b55076c4b)
- [x] wctype  (line 138, body 32998bbde3a4)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of src/conf_post.h, the AH_BOTTOM block `configure.ac` appends to `config.h`.  445 lines,
0 functions, 0 subrs, 66 macros (5 of them object-like macros with a value: `HAVE_LSTAT`,
`SIZE_MAX`, `_WIN32_WINNT`, `_GL_DT_NOTDIR`, `_REGEX_AVOID_UCHAR_H`), 0 types, 0 file-scope
objects.

1. **Liveness is measured, not read off the comments.**  `clang -std=gnu23 -E -dD` on a probe
   that includes `<config.h>` (include path `build/emacs-ref/src`, `build/emacs-ref/lib`,
   `emacs/src`, `emacs/lib`) prints the live definitions with their bodies, which is what the
   mirror spells; `docs/baseline/config/darwin-arm64.macros.tsv` (the same preprocessor run)
   supplies the predicates.  Live on this target: `NDEBUG` (29, `ENABLE_CHECKING` is `#undef` at
   macros.tsv 872), `GNUC_PREREQ` (49; `__GNUC_MINOR__` 2, `__GNUC_PATCHLEVEL__` 1,
   `__GNUC__` 4), `bool_bf` = `bool` (61; `NS_IMPL_GNUSTEP` `#undef`, `__MINGW32__` undefined),
   `ADDRESS_SANITIZER` = `false` (93), `getenv_TZ`/`setenv_TZ` (180-181), `NO_INLINE`/
   `EXTERNALLY_VISIBLE` (185-186), `PRINTF_ARCHETYPE` = `__printf__` (213), `ATTRIBUTE_FORMAT_PRINTF`
   (215), `ARG_NONNULL` (218), `DECLARE_POINTER_ALIAS` (225), `ATTRIBUTE_SECTION` (229),
   `ATTRIBUTE_MALLOC_SIZE` (234), `ADDRESS_SANITIZER_WORKAROUND` (244, empty),
   `ATTRIBUTE_NO_SANITIZE_ADDRESS` (253), `ATTRIBUTE_NO_SANITIZE_UNDEFINED` (267), `VFORK` (284;
   `DARWIN_OS` defined, macros.tsv 733), `INLINE`/`EXTERN_INLINE`/`INLINE_HEADER_BEGIN`/`END`
   (358-362; `EMACS_EXTERN_INLINE` undefined), `UNINIT` = `= {0,}` (369; `GCC_LINT` 1,
   macros.tsv 1162), `likely` (377), `REQUIRE_GNUISH_STRFTIME_AM_PM`/`SUPPORT_NON_GREG_CALENDARS_IN_STRFTIME`
   = `false` (381-382), `_REGEX_AVOID_UCHAR_H` 1, `HAVE_LSTAT` 1.
2. **P-66: the file stays off the verified list, so the mirror must name all 66 macros.**
   `tools/check-cond-eval.py src/conf_post.h` -> `41 conditions, 4 unknown, 3 disagreements
   [NOT VERIFIED]`: the evaluator cannot decide `defined __has_attribute` (C 68) or
   `#ifdef __has_feature` (C 83), which are clang builtins rather than table macros, and it
   answers "dead" where clang answers "live".  `tools/cinventory.py` is therefore blind here
   (both arms live), and the dead arms are **carried**, each under `#[cfg (feature = "…")]`
   naming its condition's non-frozen side (PORTING.md P-51):

   | C lines | condition | feature | names carried |
   | --- | --- | --- | --- |
   | 108-158 | `#ifdef MSDOS` | `msdos` | `_NAIVE_DOS_REGS`, `strtold`, `mbrtowc`, `wcrtomb`, `btowc`, `towupper`, `towlower`, `iswalnum`, `wctype`, `iswctype`, `mbsinit`, `GNULIB_SUPPORT_ONLY_AT_FDCWD`, `EOPNOTSUPP`, `opendir`, `emacs_raise`, `DATA_START` |
   | 121-127 | `#if __DJGPP__ > 2 \|\| __DJGPP_MINOR__ > 3` else | `djgpp_2_3` | `lstat`, `EOVERFLOW`, `SIZE_MAX` (the `true` arm's `HAVE_LSTAT` is live, see 3) |
   | 160-167 | `#if defined HAVE_NTGUI && !defined DebPrint` | `have_ntgui` | `DebPrint` |
   | 169-176 | `#if defined CYGWIN && defined HAVE_NTGUI` | `cygwin` | `NTGUI_UNICODE`, `_WIN32_WINNT`, `strnicmp` |
   | 384-398 | `#ifdef MSDOS` | `msdos` | `_GL_DT_NOTDIR`, `ENOTSUP`, `IFTODT` |
   | 400-445 | `#if defined WINDOWSNT && …` | `windowsnt` | `fwrite`, `fprintf`, `printf`, `vfprintf`, `vprintf` |

   No gate enables those features, so this text is parsed but not compiled: `xtask dead-arms`
   flips only its own six (`no_rint`, `no_trunc`, `no_signbit`, `have_haiku`, `have_native_comp`,
   an unnamed `HAVE_LOG2` row), and the features are named for conditions outside `config.rs`.
   That is the same situation the gnulib dead arms are in (status 3.9); the C text of every arm
   is quoted at the item it belongs to.
3. **Two names of the MS-DOS block are live on this target and carried without a feature.**
   `HAVE_LSTAT` (C 122) and `_REGEX_AVOID_UCHAR_H` (C 397) are defined to the same values by the
   frozen `config.h` itself (measured: `#define HAVE_LSTAT 1` at its 1077 and
   `#define _REGEX_AVOID_UCHAR_H 1` at its 2675), so the mirror spells each once, live, with the
   measured value; `docs/baseline/config/darwin-arm64.json`'s curated list does not carry either
   name, which is why this mirror is their only Rust home today.
4. **Ten names have no Rust item; they need rows in `docs/ports/excluded-macros.txt`** (that file
   is shared and read-only for this lane - the rows are proposed here, verbatim, for the owner to
   append).  Until they are added `--check` prints exactly ten `- missing macro …` lines; with
   them appended it prints `OK` (see Evidence).  Five are dead-arm text and are recorded in the
   arm table above but not carried (`vfork`'s carrier feature would have to be invented for a
   dead rename, and the `HAS_ATTR_*` table exists only for the excluded predicate that reads it);
   five have no Rust spelling at all (attribute/predicate classes the tree already records this
   way - `SUBR_SECTION_ATTRIBUTE`, `_GL_ARG_NONNULL`, `_Alignas`, `EMACS_ATTRIBUTE_NONNULL`).

   | name | kind | reason (also the proposed row's third column) |
   | --- | --- | --- |
   | `HAS_ATTRIBUTE` | syntax | conf_post.h 71/73: clang's `__has_attribute (__##a##__)` (the live arm) / the dead arm's identifier pasting; Rust has neither |
   | `HAS_FEATURE` | syntax | conf_post.h 84/86: clang's `__has_feature (a)` (live) / `false`; its one consumer, `ADDRESS_SANITIZER`, is spelled in the mirror |
   | `HAS_ATTR_cleanup` | dead-text | conf_post.h 74: the dead fallback arm's `GNUC_PREREQ (3, 4, 0)`; measured answer true |
   | `HAS_ATTR_no_address_safety_analysis` | dead-text | conf_post.h 75: the same arm's `false`; measured false |
   | `HAS_ATTR_no_sanitize` | dead-text | conf_post.h 76: the same arm's `false`; measured true |
   | `HAS_ATTR_no_sanitize_address` | dead-text | conf_post.h 77: the same arm's `GNUC_PREREQ (4, 8, 0)`; measured true |
   | `HAS_ATTR_no_sanitize_undefined` | dead-text | conf_post.h 78: the same arm's `GNUC_PREREQ (4, 9, 0)`; measured false |
   | `UNINIT` | syntax | conf_post.h 369/371: the live `= {0,}` / the dead empty arm; a declaration suffix, which the port writes as the declaration's zero initializer (PORTING.md 1.5) |
   | `likely` | syntax | conf_post.h 377: `__builtin_expect (!!(cond), 1)`; the port's spelling is `pub fn likely (cond: bool) -> bool` in `src/include/lisp.rs` 704 (a second item here would be an E0428 against it) |
   | `vfork` | dead-text | conf_post.h 277: `# define vfork fork`, dead (`ADDRESS_SANITIZER` false) and with no non-frozen-side feature name in the tree's table |
5. **The four names other files already spell are carried here anyway** - this mirror is their C
   definition point, and the copies go when those files start including it (P-26, one spelling per
   construct): `ADDRESS_SANITIZER` (`src/sysdep.rs` 769, `src/alloc.rs` 121),
   `getenv_TZ`/`setenv_TZ` (`lib/time_rz.rs` 75-76), `VFORK` (`src/sysdep.rs`
   961), `INLINE`/`EXTERN_INLINE` (`src/emacs.rs` 110-111, where C's own emacs.c defines
   them ahead of config.h).  `emacs_getenv_TZ`/`emacs_setenv_TZ` (C 182-183) are declared in
   `conf_post_decls` + glob (P-21) rather than at the crate root: src/timefns.c defines them
   (`src/timefns.rs` 1998/2016), and a crate-root declaration would collide with that
   definition; `lib/time_rz.rs` 69-72 keeps its own copy for now.
6. **`c_int`/`c_char` are spelled `core::ffi::…` in the five live uses.**  This header includes
   nothing (its only `#include` is the MS-Windows `ms-w32.h`), so the mirror cannot take the bare
   names from an included foundation the way the display mirrors take them from lisp.rs - a second
   `use std::ffi::…` at the including crate root is an E0252 (measured with `layout-check`), and
   lisp.rs already imports them there for every unit.  The absolute spelling is what the system
   mirrors use for the same reason (`lib/include/stdint.in.rs`, `ftoastr.rs`, `stdio-impl.rs`),
   and it is what lets `xtask build-tu src/conf_post.h` compile this mirror on its own.
7. **Attribute spellings are carried as the no-ops their sites do not need** (PORTING.md 8):
   `NO_INLINE`/`EXTERNALLY_VISIBLE`/`ARG_NONNULL`/`ATTRIBUTE_FORMAT_PRINTF`/
   `ATTRIBUTE_MALLOC_SIZE` forward to the `_GL_ATTRIBUTE_*` markers of
   `lib/include/attribute.rs` (the pair it keeps for its own public names),
   `ATTRIBUTE_SECTION`/`ATTRIBUTE_NO_SANITIZE_ADDRESS`/`ATTRIBUTE_NO_SANITIZE_UNDEFINED`/
   `ADDRESS_SANITIZER_WORKAROUND`/`PRINTF_ARCHETYPE` expand to nothing, and `INLINE` /
   `EXTERN_INLINE` / `INLINE_HEADER_BEGIN` / `INLINE_HEADER_END` are the markers P-12 asks for
   (the port's functions are ordinary `#[inline]` items).  `DECLARE_POINTER_ALIAS` is carried as a
   usable statement macro in C's shape - `let name: *mut type = { _GL_ATTRIBUTE_MAY_ALIAS! (); addr
   as *mut type };` - while its C use sites (`process.c`, `w32.c`) write the declaration directly
   in the port (`src/process.rs` 2878-2909), so the macro has no call site yet.
8. **`bool_bf`** is the one typedef: `pub type bool_bf = bool;` with a size assertion (measured 1),
   the width every mirror that packs a `bool_bf` run already reproduces.
9. **Recorded, not carried**: the HPUX block (C 96-106, four `#undef`s), the `PROFILING`/`noinline`
   blocks (C 293-300), C 109-111's deliberate compile failure, and the dead *declarations*
   (`_DebPrint` at C 162; the five `_gl_consolesafe_*` prototypes at C 414-430; the
   `getdelim`/`getline` pairs at C 438-443, whose live home is `lib/include/stdio.rs`).
   The extractor requires no function from this file, and declaring the dead Windows prototypes
   would duplicate names the system mirrors own.
10. **Found while measuring**: C 369's arm is the live one, so `UNINIT` is `= {0,}` on this target
    (`GCC_LINT` 1) - which `src/data.rs` 2031 states and `src/window.rs` 3346 states
    the other way round ("`/* empty */` on this target"), a stale comment in that unit's ledger
    (not this lane's file; recorded in the lane's report).

## Evidence

- Live text: `clang -std=gnu23 -E -dD -I build/emacs-ref/src -I build/emacs-ref/lib -I emacs/src
  -I emacs/lib <probe.c>` where the probe is `#include <config.h>`; the definitions this mirror
  spells are the `#define`s it prints from `emacs/src/conf_post.h` on (e.g. `#define NDEBUG`,
  `#define GNUC_PREREQ(v,w,x) ((v) < __GNUC__ + …)`, `#define EXTERN_INLINE static inline`,
  `#define UNINIT = {0,}`, `#define VFORK() (_Pragma(…) vfork () …)`).
- Branch verdicts: `python3 tools/check-cond-eval.py src/conf_post.h` -> `41 conditions, 4 unknown
  to the evaluator, 3 disagreements [NOT VERIFIED]`; the 3 are `src/conf_post.h:68` and `:83`
  (evaluator false, clang true) and `:264` (unknown).  Per-condition clang verdicts were read with
  the same tool's probe machinery: the 21 conditions of the MS-DOS/MS-Windows/Cygwin blocks are
  false for clang, `3 <= __GNUC__` (228), C 241, C 252, C 266, `DARWIN_OS` (283), C 293 and
  `defined GCC_LINT` (368) are true.
- Inventory: `python3 tools/extract-abi.py emacs/src/conf_post.h` -> this ledger (66 macros, 0
  functions, 0 types, 0 objects).
- `python3 tools/extract-abi.py --check emacs/src/conf_post.h src/include/conf_post.rs` ->
  `FAIL` with exactly the ten `- missing macro …` lines of Decision 4 and nothing else.
- With the proposed rows appended (`--excluded <copy of docs/ports/excluded-macros.txt plus the
  ten rows of Decision 4>`): `python3 tools/extract-abi.py --check --excluded
  /tmp/cp-probe/excluded-macros.txt emacs/src/conf_post.h src/include/conf_post.rs` ->
  `OK emacs/src/conf_post.h -> src/include/conf_post.rs` (0 problem lines; the run prints
  `(10 C macros have a recorded Rust counterpart: …)`).
- `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-cp cargo run -q -p xtask -- build-tu
  src/conf_post.h` -> `built src/include/conf_post.rs -> ../build/femacs/include_conf_post.o`.
- `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-cp cargo run -q -p xtask -- layout-check` ->
  `layout-check: 88 of 99 mirror(s) compiled, 11 platform-exclusive skipped (…)` and
  `layout-check passed`.
- `python3 tools/check-declarations.py` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`; `python3 tools/check-citations.py` -> `612 file(s) scanned, 12 drift,
  3 range, 284 external, 36 allowed, 0 stale`, none of them in this mirror (it cites C lines as
  bare `C <n>`, the mirror convention).
