# emacs/lib/stdlib.in.h (1991 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [ ] _gl_MB_CUR_MAX  (line 761)
- [x] rpl_realloc  (line 1468)

## Macros defined — 75
- [x] EXIT_FAILURE  (line 227, body 356a192b7913)
- [x] EXIT_SUCCESS  (line 219, body b6589fc6ab0d)
- [x] GNULIB_defined_MB_CUR_MAX  (line 773, body 356a192b7913)
- [x] GNULIB_defined_canonicalize_file_name  (line 478, body 9ef8afe89142)
- [x] GNULIB_defined_print_stack_trace  (line 1658, body 356a192b7913)
- [x] GNULIB_defined_ptsname_r  (line 1104, body 5b235cd3e8f9)
- [x] GNULIB_defined_qsort_r_fn_types  (line 1167, body 356a192b7913)
- [x] GNULIB_defined_strtod_function  (line 1669, body 356a192b7913)
- [x] GNULIB_defined_strtof_function  (line 1700, body 356a192b7913)
- [x] GNULIB_defined_strtol_function  (line 1767, body 356a192b7913)
- [x] GNULIB_defined_strtold_function  (line 1731, body 356a192b7913)
- [x] GNULIB_defined_strtoll_function  (line 1809, body 356a192b7913)
- [x] GNULIB_defined_strtoul_function  (line 1848, body 356a192b7913)
- [x] GNULIB_defined_strtoull_function  (line 1890, body 356a192b7913)
- [x] GNULIB_defined_struct_random_data  (line 101, body 356a192b7913)
- [x] MB_CUR_MAX  (line 772, body 2c68f29348c2)
- [x] RAND_MAX  (line 1210, body 75878664309b)
- [x] _  (line 53, body 9f3e5dee33b9)
- [x] _Exit  (line 249, body 5a5c37288c19)
- [x] _GL_ATTRIBUTE_DEALLOC  (line 142, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_DEALLOC_FREE  (line 150, body f761bb1d7fb2)
- [x] _GL_ATTRIBUTE_MALLOC  (line 160, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_NONNULL_IF_NONZERO  (line 172, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_NOTHROW  (line 193, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_PURE  (line 204, body da39a3ee5e6b)
- [x] _GL_INLINE_RPL_REALLOC  (line 1463, body 356a192b7913)
- [x] _GL_REALLOC_INLINE  (line 132, body a960df3ab827)
- [x] _GL_STDLIB_INLINE  (line 129, body a960df3ab827)
- [x] abort  (line 277, body f3a29ed39f38)
- [x] aligned_alloc  (line 320, body 5650465e7eb9)
- [x] calloc  (line 397, body 2191c51a5bf7)
- [x] canonicalize_file_name  (line 450, body 98a71b39d333)
- [x] ecvt  (line 517, body a0735c199cb1)
- [x] fcvt  (line 540, body 70457fd87f92)
- [x] free  (line 294, body 673c8b447685)
- [x] gcvt  (line 563, body 32dc94d6feca)
- [x] getloadavg  (line 584, body ba212981a800)
- [x] getprogname  (line 612, body 2c1d6ee37940)
- [x] getsubopt  (line 655, body 48e2fcbeebaa)
- [x] initstate  (line 1288, body 46604de2445e)
- [x] initstate_r  (line 1399, body a8f79deeafe5)
- [x] malloc  (line 706, body 7d415fafcb1d)
- [x] mbstowcs  (line 782, body 0db72ecb9205)
- [x] mbtowc  (line 811, body af870c594f64)
- [x] mkostemp  (line 871, body 176201c49f37)
- [x] mkostemps  (line 910, body 733de5cb9f87)
- [x] mkstemp  (line 948, body 28a9c813730e)
- [x] mktemp  (line 999, body 438fbce7d918)
- [x] posix_memalign  (line 1013, body 8cb5f1892c03)
- [x] posix_openpt  (line 1042, body ee328d9ea77c)
- [x] print_stack_trace  (line 1657, body da39a3ee5e6b)
- [x] ptsname  (line 1068, body 143e6e49d52b)
- [x] ptsname_r  (line 1093, body f738d5c67b5c)
- [x] putenv  (line 1143, body 5c2abb6ca4c2)
- [x] qsort_r  (line 1175, body fc34456603aa)
- [x] rand  (line 1219, body 83fe29b090b6)
- [x] random  (line 1236, body 0fa7179661e2)
- [x] random_r  (line 1347, body 49c4ac208f3c)
- [x] realloc  (line 1479, body 7d6e078f5f15)
- [x] reallocarray  (line 1533, body babfe5f7c886)
- [x] realpath  (line 1562, body 1c15e1b87fba)
- [x] setenv  (line 1624, body bdd38f4c7869)
- [x] setstate  (line 1320, body 95f3841c4013)
- [x] setstate_r  (line 1433, body ae2adf211e6a)
- [x] srandom  (line 1262, body f603453ed604)
- [x] srandom_r  (line 1371, body 670c5bac7ba7)
- [x] strtod  (line 1667, body e710349e3dbf)
- [x] strtof  (line 1698, body 65ef68fba0c9)
- [x] strtol  (line 1765, body 57fb235bacbd)
- [x] strtold  (line 1729, body 92a65c54cf6a)
- [x] strtoll  (line 1807, body f72c3c97cd63)
- [x] strtoul  (line 1846, body fe3b109fb5f9)
- [x] strtoull  (line 1888, body 141ff0f35e49)
- [x] unsetenv  (line 1937, body 47da023af30a)
- [x] wctomb  (line 1962, body 07ce4a0761ae)

## Types and their fields — 1
- [x] random_data: int32_t* fptr, int32_t* rptr, int32_t* state, int rand_type, int rand_deg, int rand_sep, int32_t* end_ptr

## File-scope objects — 0

## Decisions

- **The criterion is the generated header, and it exists.**  `ls -l build/emacs-ref/lib/stdlib.h` ->
  `94510` bytes, 2,581 lines, generated from this template by `build/emacs-ref/lib/gnulib.mk`'s
  rule (the header is *not* pre-digested: every `@PLACEHOLDER@` becomes a literal number there).
  The diff that names every substitution is in the mirror's head note:
  `diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/stdlib.in.h) build/emacs-ref/lib/stdlib.h` ->
  1,258 lines (most of them the `c++defs.h`/`arg-nonnull.h`/`warn-on-use.h` text the build inlines,
  plus the build's own `@GNULIB_FOO@` legend near the end).
- **`tools/check-cond-eval.py lib/stdlib.in.h` -> `393 conditions, 169 unknown, 218 disagreements
  [NOT VERIFIED]`**, so the extractor resolves nothing and its checklist is the union of every arm
  (78 items: 2 functions defined, 75 macros, 1 type).  The arms are decided by the **frozen
  preprocessor** on the generated header instead, and the result is the live/dead table of this
  round:
  - **the two live renames**: `clang -E -dM … build/emacs-ref/lib/stdlib.h | grep '^#define '`
    prints `#define free rpl_free` and `#define realpath rpl_realpath` and *no other* `X rpl_X`
    rename - so `@REPLACE_FREE@` and `@REPLACE_REALPATH@` are the only two replacement selectors
    this build takes (the generated header shows `#if 1`/`# if 1` at 880-881 and 2149-2150);
  - **42 of the 45 function names are declared** by the frozen translation unit (measured by
    preprocesssing the generated header and grepping each name, then dumping the declarations with
    `clang -std=gnu23 -Xclang -ast-dump`), 7 are not (below);
  - **the live macro values**: `EXIT_SUCCESS`/`EXIT_FAILURE` (0/1), `RAND_MAX` (`0x7fffffff`), and
    `MB_CUR_MAX`, whose frozen spelling is `__mb_cur_max` - the *system's*
    (`_stdlib.h` 133-139; the SDK takes the `__mb_cur_max` arm, not the
    `_USE_EXTENDED_LOCALES_` one, measured by preprocesssing `MB_CUR_MAX`).
- **The system half is `stdlib.rs`** (C 38/50's `# @INCLUDE_NEXT@ @NEXT_STDLIB_H@`) and the
  attribute spellings are `attribute.rs` (C 142-215's `_GL_ATTRIBUTE_*`, each guarded by
  `#ifndef` so `lib/attribute.h`'s own text wins) - the `dirent.in.rs`/`errno.in.rs` relation.
  Nothing is repeated from either: `EXIT_SUCCESS`/`EXIT_FAILURE`, `malloc`, `realloc`, `calloc`,
  `free`, `strtod`, `strtol`, `strtoul` and `getenv` are `stdlib.rs`'s, and a second declaration of
  one name in one crate is `E0428`.
- **What this mirror carries, and the shape of each half.**  (a) The two exit constants' kin:
  `RAND_MAX` (gnulib's value, and the system's too), `MB_CUR_MAX` (a macro over a declaration of
  the libc variable `__mb_cur_max`), the two `_GL_*_INLINE` qualifier spellings as *macros* (the
  `_GL_STDC_*_INLINE` shape: `lib/stdlib.rs` and `lib/realloc.rs` hold the same C names as
  `const`s, and a macro and a const are different namespaces), the eleven `GNULIB_defined_*`
  witnesses plus `_GL_INLINE_RPL_REALLOC` as the values C's own arms set, and
  `GNULIB_defined_canonicalize_file_name` twice over (a macro carrying the template's placeholder
  tokens, because the recorded body is an expression of two `@PLACEHOLDER@` names - the
  `sys_types.in.rs` 109 `_GL_WINDOWS_STAT_INODES` shape - and the `pub const` beside it for the
  frozen value, the `_GL_MEMEQ_INLINE` rule).  (b) The 34 live declarations `stdlib.rs` does not
  have, in a whole-import module (`PORTING.md` 0.1-10, the `stdio.rs` `stdio_decls` shape) so that
  a unit which declares one of them itself - `lib/getloadavg.rs`, `lib/mkostemp.rs`,
  `lib/fstatat.rs` and others do - shadows the glob instead of colliding with it.  Their
  signatures are the ones `-Xclang -ast-dump` prints for the frozen translation unit, with
  `wchar_t` as `c_int` (4 bytes on this target, `wchar.rs` 10 spells it so) and `long double` as
  `c_double` (8 bytes, measured).  (c) The two live renames: C's own text declares the *renamed*
  functions (`_GL_FUNCDECL_RPL (free, …)` -> `rpl_free`) and defines `# define free rpl_free`, so
  the port declares `rpl_free`/`rpl_realpath` (which the unit, the port of gnulib's `free.c`, emits
  with `#[unsafe(export_name = "rpl_free")]`, `PORTING.md` P-81) and spells the redirect as the
  macro `free!`/`realpath!`, exactly as C does.  `stdlib.rs`'s plain `free` stays the *system*
  symbol: which of the two names a unit should reach is a decision about the include sites (the
  `stdbit.in.rs` ownership move's shape), so it is **recorded here, not taken** - no unit was
  repointed this round.
- **The 11 unchecked boxes, named (this is the whole remaining gap).**
  - `_` (C 53, the guard prefix placeholder) is the *shared* `docs/ports/excluded-macros.txt` row
    the stdbit/string mirrors use too (the gate prints *"N C macros have a recorded Rust
    counterpart: _"*).
  - `random_data` (C 84-92, 7 fields) is the one **type**: it lives inside C 83's
    `#if @GNULIB_RANDOM_R@`, which the generated header substitutes to `#if 0`, and macOS has no
    `struct random_data` either, so the mirror carries nothing and this box stays unchecked - which
    is what makes `--check` report it as *"missing type deferred"* in words rather than as a
    failure (`tools/extract-abi.py`'s ledger escape hatch, the `haikuterm.h` precedent).
  - **nine names are requested as `docs/ports/excluded-macros.txt` rows** (`kind=config`), all of
    them dead arms plus two template definitions whose gates are 0; the rows are listed verbatim
    under `## Evidence`.  Nothing is carried for them in the mirror: P-66 rule 2 asks for the
    boundary and the C line numbers rather than a transcription, and the integrator's ruling for
    this file named that table as the mechanism.
- **One `docs/ports/macro-body-notes.tsv` row is requested**: `MB_CUR_MAX`'s recorded body is the
  *dead* arm's `_gl_MB_CUR_MAX ()` (C 772), which the port cannot spell - the live macro is the
  system's `__mb_cur_max`.  The row exempts that one token; the name itself is live and carried.
- **The three question-shaped arms, for the integrator** (asked rather than decided, per the
  round's ruling): (1) whether the five dead renames (`initstate_r`, `random_r`, `srandom_r`,
  `setstate_r`, `reallocarray`) and the four other dead names should stay excluded rows or become
  `#[cfg (feature = "gnulib_<name>")]` macros carrying C's rename tokens - the shape
  `lib/include/string.in.rs` (another lane, in flight) uses for its 31 dead renames, which would
  cost one undeclared feature name each and no shared-table row; (2) whether the include sites
  should move from `stdlib.rs` to this mirror (`stdlib.in.rs` includes it, so a repointed unit gets
  both halves) - 13+ units include `stdlib.rs` today; (3) whether `struct random_data` should stay
  the ledger's deferral or be carried as a `#[cfg]`-gated struct.

## Evidence

- **The judge**: `ls -l build/emacs-ref/lib/stdlib.h` -> `94510` bytes; `wc -l` -> `2581`;
  `diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/stdlib.in.h) build/emacs-ref/lib/stdlib.h | wc -l`
  -> `1258`.
- **`tools/check-cond-eval.py lib/stdlib.in.h`** -> `lib/stdlib.in.h: 393 conditions, 169 unknown
  to the evaluator, 218 disagreements` (the `[NOT VERIFIED]` line is what the extractor's output
  prints).
- **The frozen preprocessor's readings** (the arms' authority):
  - `clang -E -dM -std=gnu23 -include build/emacs-ref/src/config.h -I build/emacs-ref/lib
    -I build/emacs-ref/src -I emacs/src -I emacs/lib build/emacs-ref/lib/stdlib.h | grep '^#define'`
    -> exactly two live renames, `#define free rpl_free` and `#define realpath rpl_realpath`; the
    same run prints `#define MB_CUR_MAX __mb_cur_max`, `#define RAND_MAX 0x7fffffff`,
    `#define EXIT_SUCCESS 0`, `#define EXIT_FAILURE 1`.
  - `clang -E … build/emacs-ref/lib/stdlib.h | grep -c '\bX\b'` for each of the 45 function names
    -> 42 present, and these 7 absent: `_gl_MB_CUR_MAX`, `rpl_realloc`, `initstate_r`, `random_r`,
    `reallocarray`, `setstate_r`, `srandom_r` (`print_stack_trace` and
    `GNULIB_defined_ptsname_r` are *macros*, so they are classified by the `-dM` table instead:
    neither appears there, i.e. their arms are `#if 0`).
  - `clang -std=gnu23 -fsyntax-only -Xclang -ast-dump -include build/emacs-ref/src/config.h …` on a
    one-line `#include <stdlib.h>` file -> the 42 `FunctionDecl`s this mirror's `/// C:` lines
    quote, and `nm -gU /usr/lib/libSystem.B.dylib` + a grep of
    `$(xcrun --show-sdk-path)/usr/include` -> none of the five dead names exist on this target at
    all (macOS has no `*_r` family and no `reallocarray`).
  - `printf '#include <stdlib.h>\nMB_CUR_MAX\n' | clang -E -std=gnu23 …` -> `__mb_cur_max` (the
    `_USE_EXTENDED_LOCALES_` arm's `___mb_cur_max ()` is not the frozen one).
- **`--check` verdict with the ten requested rows**: a `/tmp` copy of each shared table plus the
  ten rows, passed through `--excluded`/`--macro-notes`,
  `python3 tools/extract-abi.py --check --excluded … --macro-notes … emacs/lib/stdlib.in.h
  femacs/lib/include/stdlib.in.rs` -> `OK emacs/lib/stdlib.in.h -> femacs/lib/include/stdlib.in.rs`
  with `grep -c '^- '` -> `0`; with the tree's tables it reports exactly those ten lines (nine
  `missing`, one `does not mention _gl_mb_cur_max`).  The type is not one of them: the same command
  prints on stderr `missing type deferred: lib/stdlib.in.h random_data (the ledger has unchecked
  boxes)` - this ledger's unchecked box is what turns the missing type into a recorded deferral
  instead of a failure.  The ten rows
  are listed verbatim below.
- **The ten rows requested** (`<TAB>` is a literal tab):
  - to `docs/ports/excluded-macros.txt`:
    `reallocarray<TAB>config<TAB>stdlib.in.h 1530-1560: the whole block is C 1530's `#if @GNULIB_REALLOCARRAY@`, which the generated header substitutes to `#if 0` (line 2119), so the frozen header declares nothing for the name - and macOS has no `reallocarray` either (measured: absent from the SDK headers and from `nm -gU /usr/lib/libSystem.B.dylib`); the port carries nothing`
    `initstate_r<TAB>config<TAB>stdlib.in.h 1395-1417: inside C 1394's `#if @GNULIB_RANDOM_R@` (`#if 0` at generated line 1738) + `# if @REPLACE_RANDOM_R@`; on this target the header's own `#  define initstate_r rpl_initstate_r` and the `_GL_FUNCDECL_SYS` are both dead text and macOS has no `initstate_r` (measured)`
    `random_r<TAB>config<TAB>stdlib.in.h 1343-1365: the same dead `@GNULIB_RANDOM_R@` arm (`#  define random_r rpl_random_r`); macOS has no `random_r` (measured)`
    `srandom_r<TAB>config<TAB>stdlib.in.h 1367-1389: the same dead arm (`#  define srandom_r rpl_srandom_r`); macOS has no `srandom_r` (measured)`
    `setstate_r<TAB>config<TAB>stdlib.in.h 1429-1447: the same dead arm (`#  define setstate_r rpl_setstate_r`); macOS has no `setstate_r` (measured)`
    `print_stack_trace<TAB>config<TAB>stdlib.in.h 1650-1661: inside C 1650's `#if @GNULIB_STACK_TRACE@` (`#if 0` at generated line 2240); the arm's own body is *empty* (`#   define print_stack_trace() /* nothing */`, the extractor records an empty body), so there is no Rust item that could carry the name - and the port has no stack-trace printer`
    `GNULIB_defined_ptsname_r<TAB>config<TAB>stdlib.in.h 1103-1105: the witness of C 1085's `#if @GNULIB_PTSNAME_R@` block, which the generated header substitutes to `#if 0` (so `ptsname_r` itself is the *system's*, declared in the module below), and its body is an expression of two placeholder names (`(!@HAVE_PTSNAME_R@ || @REPLACE_PTSNAME_R@`) - unlike `GNULIB_defined_canonicalize_file_name` its arm is not taken, so there is no value to carry either`
    `_gl_MB_CUR_MAX<TAB>config<TAB>stdlib.in.h 759-775: the definition is inside C 758's `#if @REPLACE_MB_CUR_MAX@` (`#if 0` at generated line 1348), so the frozen header has no such function - the live `MB_CUR_MAX` is the system's `__mb_cur_max`, which the mirror carries`
    `rpl_realloc<TAB>config<TAB>stdlib.in.h 1462-1471: inside C 1461's `#if @GNULIB_REALLOC_POSIX@` + `#  if @REPLACE_REALLOC_FOR_REALLOC_POSIX@` (`# if 0` at generated line 2051), so neither the `#   define _GL_INLINE_RPL_REALLOC 1` (carried as the constant C's arm sets) nor the `rpl_realloc` definition is live; the frozen `realloc` is the system's, which `stdlib.rs` declares`
  - to `docs/ports/macro-body-notes.tsv`:
    `MB_CUR_MAX<TAB>_gl_MB_CUR_MAX<TAB>stdlib.in.h 772: the recorded body is the *dead* `#if @REPLACE_MB_CUR_MAX@` arm's `#  define MB_CUR_MAX _gl_MB_CUR_MAX ()`; the live macro is the frozen `<stdlib.h>`'s `#define MB_CUR_MAX __mb_cur_max` (measured with the frozen preprocessor), which the mirror spells, so C's `_gl_MB_CUR_MAX` has no counterpart in the mirror's body`
- **The `E0428` rule, checked by hand before writing**: the layout probe `include!`s every mirror
  into **one** module (no isolation), so a name this mirror declares must not be declared by
  another mirror that the probe compiles.  The three groups that share a name with this file are
  each out of the way: `src/include/w32.rs` (which has `random`, `realpath`, `srandom`) gates every
  item behind `#[cfg (feature = "windowsnt")]` (lines 78-468; the feature is off in the probe),
  `src/include/msdos.rs`'s `unsetenv` sits under `#[cfg (feature = "msdos")]` (153-158), and
  `nt/inc/include/ms-w32.rs` (`free`, `putenv`, `getloadavg`, `mkostemp`), the two
  `scratch_buffer.rs` copies (`rpl_free`) and `nt/inc/include/unistd.rs` are **not in the probe's
  include list at all** (`grep -c` of each path in `build/femacs/layout_probe.rs` -> 0) - which is
  also why the probe reports one error and not a duplicate-definition cascade.  (A hand-rolled
  two-mirror probe cannot settle this: `include!` has no dedupe, so it either includes `stdio.rs`
  twice or pulls `w32.rs`'s `termhooks.rs`/`frame.rs` without the foundation macros
  (`cannot find macro eassert`); the include-graph dedupe and the foundation front-loading are
  `xtask`'s, and its `layout-check` run below is the measurement.)
- **Compile evidence (the mirror compiles, in the shared crate and standalone)**:
  - `rustc +nightly-2026-10-01 --edition 2024 --crate-type bin -C panic=abort -o /tmp/abip/P
    /tmp/abip/P.rs` (the probe `include!`s this mirror with the `std::ffi` alias set) -> exit 0, and
    running it prints `stdlib.in.rs probe ok: RAND_MAX=2147483647, MB_CUR_MAX=1`, exit 0.
  - **A correction to this bullet, made in the `stdio.in.h` round (2026-10-04)**: it used to say
    "this mirror is one of the 97, not of the 11 skips, so it went through rustc inside the shared
    single-module crate".  That is **wrong**.  `xtask layout-check` seeds `femacs/src/include/*.rs`
    plus the two `foundation` mirrors and compiles what the `include!` graph *reaches*
    (`femacs/xtask/src/main.rs` 795-812), so the "97" was the `src/include` seed count and says
    nothing about a `lib/include` file.  Measured in the later round:
    `grep -c 'lib/include/stdlib.in.rs' build/femacs/layout_probe.rs` -> `0` and
    `build/femacs/layout_probe_src/` has no `…_stdlib_in_rs.rs`, while `…_stdbit_in_rs.rs` *is*
    there (it is reached through `src/include/lisp.rs`).  **The standalone probe above is this
    file's compile evidence**; `layout-check` compiles it only once some `src/include` mirror
    includes it (the include-site migration the integrator scheduled for a later round).  The
    command's own reading was as quoted: `layout-check: 86 of 97 mirror(s) compiled, 11
    platform-exclusive skipped (…)`, `layout probe ok`, `layout-check passed`, exit 0 - for the
    crate, not for this file.  (That round the command was red at one point and the *only*
    diagnostic was another lane's untracked `femacs/src/include/textconv.rs`
    (`error: extern block is not supported in `extern` blocks` at 174:5); no diagnostic ever named
    `stdlib.in.rs`, and that lane has since fixed the file - I did not touch it.)
  - No unit includes this mirror yet (`grep -rn 'include!.*stdlib.in.rs' femacs` -> nothing), so
    there is no `build-tu` consumer to show; the units that include `stdlib.rs` keep
    working unchanged (nothing here is added to that mirror).
- **Gates on this worktree**: `check-declarations` -> `13 declaration sites (0 forbidden, 13 allowed
  literals), 0 failures`; `check-globals` -> `283 units, 0 DEFVAR_* variables without a definition
  in their own unit`; `check-mirror-dupes` -> `1 duplicate declaration(s), 1 ratchet row(s),
  0 failure(s)`; `check-verbatim` -> `6 trees, 4607 files, 0 failures`; `check-target-names` ->
  `609 .rs files, 593 map targets, 0 finding(s)`.

## Decisions (stale-box sweep: **not stale**, 2026-10-04)

The 1 box(es) this sweep tentatively ticked are **real pending work, not stale bookkeeping**,
and the tick was reverted.  Measured: while the ledger has unchecked boxes,
`python3 tools/extract-abi.py --check emacs/lib/stdlib.in.h femacs/lib/include/stdlib.in.rs` prints `0` `- ` lines **because the tool
defers this file's missing items to stderr** (`missing type deferred: lib/stdlib.in.h <name> (the ledger has
unchecked boxes)`); with the boxes ticked the deferral lifts and the same command prints

    - missing type random_data

i.e. the type is genuinely absent from femacs/lib/include/stdlib.in.rs.  So: lib/stdlib.in.h's `struct random_data` is a template arm: the frozen arm is decided by the generated header (`build/emacs-ref/lib/stdlib.h`, P-66/3.207's `sys_time.in.h` precedent) - measure it there before writing the record.
