# emacs/lib/stdio.in.h (2224 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] getline  (line 1155)

## Macros defined — 84
- [x] GNULIB_overrides_asprintf  (line 1823, body da39a3ee5e6b)
- [x] GNULIB_overrides_fprintf  (line 670, body 356a192b7913)
- [x] GNULIB_overrides_printf  (line 1422, body 356a192b7913)
- [x] GNULIB_overrides_snprintf  (line 1648, body 356a192b7913)
- [x] GNULIB_overrides_sprintf  (line 1713, body 356a192b7913)
- [x] GNULIB_overrides_vasprintf  (line 1847, body 356a192b7913)
- [x] GNULIB_overrides_vfprintf  (line 1968, body 356a192b7913)
- [x] GNULIB_overrides_vprintf  (line 2058, body 356a192b7913)
- [x] GNULIB_overrides_vsnprintf  (line 2139, body 356a192b7913)
- [x] GNULIB_overrides_vsprintf  (line 2195, body 356a192b7913)
- [x] _  (line 59, body defc2be6e613)
- [x] _GL_ATTRIBUTE_DEALLOC  (line 121, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_DEALLOC_FREE  (line 260, body 877a21dfea86)
- [x] _GL_ATTRIBUTE_FORMAT  (line 149, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD  (line 213, body 8597bd39607b)
- [x] _GL_ATTRIBUTE_FORMAT_PRINTF_SYSTEM  (line 220, body b1f1f7b5cbd1)
- [x] _GL_ATTRIBUTE_FORMAT_SCANF  (line 231, body db6a5e1e44e6)
- [x] _GL_ATTRIBUTE_FORMAT_SCANF_SYSTEM  (line 239, body db6a5e1e44e6)
- [x] _GL_ATTRIBUTE_MALLOC  (line 159, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_NOTHROW  (line 180, body da39a3ee5e6b)
- [x] _GL_ATTRIBUTE_SPEC_PRINTF_STANDARD  (line 193, body c1bb0190e235)
- [x] _GL_ATTRIBUTE_SPEC_PRINTF_SYSTEM  (line 206, body c1bb0190e235)
- [x] _GL_DEFINED__POSIX_C_SOURCE  (line 42, body da39a3ee5e6b)
- [x] _GL_FSEEK_WARN  (line 918, body da39a3ee5e6b)
- [x] _GL_FTELL_WARN  (line 981, body da39a3ee5e6b)
- [x] _GL_STDIO_MACROEXPAND_AND_STRINGIZE  (line 294, body 15a49ad15e55)
- [x] _GL_STDIO_STRINGIZE  (line 293, body d107e94c1dde)
- [x] _POSIX_C_SOURCE  (line 41, body c674311346c8)
- [x] _PRINTF_NAN_LEN_MAX  (line 327, body cb4e5208b4cd)
- [x] __has_feature  (line 1095, body b6589fc6ab0d)
- [x] asprintf  (line 1821, body 39cf986d4553)
- [x] dprintf  (line 381, body 29c8429408e8)
- [x] fclose  (line 410, body 61d9c99a5c1a)
- [x] fcloseall  (line 434, body e96f42e66f53)
- [x] fdopen  (line 517, body 3b19e8fce51e)
- [x] fflush  (line 536, body 2fa6d9c8a545)
- [x] fgetc  (line 556, body caf4eac54254)
- [x] fgets  (line 572, body 08358f030f31)
- [x] fileno  (line 595, body f3da3d29faba)
- [x] fopen  (line 609, body a96a683cff19)
- [x] fprintf  (line 694, body 6232691acf83)
- [x] fpurge  (line 713, body 1d7908f0ea7e)
- [x] fputc  (line 737, body fc65ea356fe7)
- [x] fputs  (line 753, body a89707f982e2)
- [x] fread  (line 773, body 45ad0c38fecb)
- [x] freopen  (line 796, body d95839532f27)
- [x] fscanf  (line 824, body fad155258b54)
- [x] fseek  (line 881, body 9589b8df1ec3)
- [x] fseeko  (line 904, body 81e198f497e4)
- [x] ftell  (line 946, body 4ab3c6970606)
- [x] ftello  (line 967, body 7b0974ae3025)
- [x] fwrite  (line 1046, body ef0701aead85)
- [x] fwrite_unlocked  (line 1037, body 0d8f81954150)
- [x] getc  (line 1054, body caf4eac54254)
- [x] getchar  (line 1070, body 07fc0b170b81)
- [x] getdelim  (line 1092, body 37ea72ae1d14)
- [x] getline  (line 1164, body 68ae23f939fe)
- [x] getw  (line 1204, body 993ed9b9f087)
- [x] obstack_printf  (line 1256, body 223077dcb765)
- [x] obstack_vprintf  (line 1277, body e436d4310324)
- [x] perror  (line 1317, body 4c32ecdbbebd)
- [x] popen  (line 1337, body 25929a148623)
- [x] printf  (line 1432, body 2993eec38537)
- [x] putc  (line 1446, body fc65ea356fe7)
- [x] putchar  (line 1462, body 2d4511679d7a)
- [x] puts  (line 1478, body 41966c72f376)
- [x] putw  (line 1497, body e50c94ce90d2)
- [x] remove  (line 1518, body 12ac7b1803c2)
- [x] rename  (line 1538, body 14ee3cb8f657)
- [x] renameat  (line 1562, body 69a1f7e95382)
- [x] rpl_getdelim  (line 1103, body 4bff44005f8b)
- [x] scanf  (line 1604, body a1b82795ecbf)
- [x] snprintf  (line 1646, body 16996b94cff3)
- [x] sprintf  (line 1711, body 4a6d16f9532e)
- [x] tempnam  (line 1741, body eb5bd9e13daa)
- [x] tmpfile  (line 1753, body 3284bd4494e4)
- [x] vasprintf  (line 1845, body ed131966e5f0)
- [x] vdprintf  (line 1911, body a28def7fc495)
- [x] vfprintf  (line 1999, body d6297140058f)
- [x] vfscanf  (line 2013, body 328126402ef7)
- [x] vprintf  (line 2082, body c7dd1f43f598)
- [x] vscanf  (line 2096, body 38d6198d2694)
- [x] vsnprintf  (line 2137, body a778dcc6e5e8)
- [x] vsprintf  (line 2193, body e42a331b5033)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The criterion is the generated header, and it exists.**  `ls -l build/emacs-ref/lib/stdio.h` ->
  `107390` bytes, 2,771 lines; `diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/stdio.in.h)
  build/emacs-ref/lib/stdio.h` -> 1,150 lines, 877 of them changed (most of it the
  `c++defs.h`/`arg-nonnull.h`/`warn-on-use.h` text the build inlines, plus the build's own
  `@GNULIB_FOO@` legend near the end).
- **`tools/check-cond-eval.py lib/stdio.in.h` -> `360 conditions, 155 unknown, 214 disagreements
  [NOT VERIFIED]`**, so the extractor resolves nothing and its checklist is the union of every arm
  (85 items: 1 function, 84 macros).  The arms are decided by the **frozen preprocessor** on the
  generated header, and the result is this round's table:
  - `clang -E -dM … build/emacs-ref/lib/stdio.h` -> **fourteen** live macros, and **not one of them
    is a `#define X rpl_X` rename**: every rename the template writes (`#  define fclose
    rpl_fclose`, `#  define printf …`, the ten `GNULIB_overrides_*` arms' renames, `#  define
    fwrite_unlocked rpl_fwrite_unlocked`, …) is dead text on this target, and the live arm declares
    the *system's* function instead.  That is why this mirror declares entry points rather than
    redirecting them, and why the *one* rename it does carry (`free` in `<stdlib.h>`) belongs to
    `stdlib.in.rs`, not here.
  - `clang -E … | grep -c '\bX\b'` per function name -> **49 declared**, and 20 template names
    absent: `fcloseall`, `fwrite_unlocked`, `obstack_printf`, `obstack_vprintf`, `rpl_getdelim`,
    `__has_feature`, the ten `GNULIB_overrides_*` markers, `_GL_FSEEK_WARN`/`_GL_FTELL_WARN`,
    `_GL_DEFINED__POSIX_C_SOURCE` (the markers and warning names are dead *arms*, and the port
    carries them - see below).
- **`getline`'s judgement** (asked for this round; the measurement is in the mirror's head note
  too): the generated header substitutes **`@GNULIB_GETLINE@` = `1`** (`#if 1` at line 1684) and
  **`@REPLACE_GETLINE@` = `0`** (`# if 0` at line 1691).  So the RPL arm - `_GL_FUNCDECL_RPL
  (getline, …)` (1685-1692), the C++ inline definition at 1696-1706 that the extractor reads as the
  file's one *function*, and `#   define getline rpl_getline` at 1711 - is dead text, and the live
  arm (`# else`, 1709) leaves only `_GL_CXXALIAS_SYS`: **`getline` is the system's function**.
  `HAVE_DECL_GETLINE` is 1 as well (1691's sibling guard at 1717 prints `#  if !1`), so even
  gnulib's own `_GL_FUNCDECL_SYS (getline, …)` at 1718 is skipped.
- **The system half is `stdio.rs`** (C 47/52's `# @INCLUDE_NEXT@ @NEXT_STDIO_H@`) and the attribute
  spellings are `attribute.rs` (C 110-290's `_GL_ATTRIBUTE_*`, each `#ifndef`-guarded) - the
  `dirent.in.rs`/`errno.in.rs` relation.  Nothing is repeated from either: the 20 entry points
  `stdio.rs` already declares (`fclose`, `fopen`, `fread`, `fwrite`, `fputs`, `fputc`, `sprintf`,
  `getchar`, `putchar`, `puts`, `printf`, `perror`, `fprintf`, `putc`, `fflush`, `fileno`, `getc`,
  …) and the five `_GL_ATTRIBUTE_*` names stay there, because a second declaration of one name in
  one crate is `E0428`.
- **What this mirror carries.**  (a) The `_POSIX_C_SOURCE` value (C 41) and its marker (C 42), the
  six format/scan attribute spellings (C 191-247), the stringification pair (C 293-294, real macros
  over Rust's `stringify!`), `_PRINTF_NAN_LEN_MAX` (the frozen BSD arm's `3`), the ten
  `GNULIB_overrides_*` markers, and the two `_GL_FSEEK_WARN`/`_GL_FTELL_WARN` category markers.
  The markers whose arms are dead are carried the way `sys_types.in.rs` 103-132 and
  `endian.in.rs` 36-49 carry theirs: the *value* C's arm sets is a fact about the header, and the
  port has no second inclusion for it to guard (`GNULIB_overrides_asprintf`, `_GL_FSEEK_WARN`,
  `_GL_FTELL_WARN` and `_GL_DEFINED__POSIX_C_SOURCE` have no value in C, so they are the no-op
  macros `attribute.rs` makes of a name whose expansion is nothing).  (b) The 32 declarations
  `stdio.rs` does not have, in a whole-import module (`PORTING.md` 0.1-10) - named
  **`stdio_in_decls`**, not `stdio_decls`, because this mirror `include!`s `stdio.rs` and that
  module name is already its.  The standalone probe measured the clash before the rename
  (`the name `stdio_decls` is defined multiple times` at `stdio.rs` 23), which is exactly the
  class of `E0428` the golden rule asks to check first.  Their signatures are the ones
  `-Xclang -ast-dump` prints for the frozen translation unit, with `off_t` spelled `i64` and
  `ssize_t` spelled `c_long` (the port's own aliases, `sys_types.in.rs` 59 and `unistd.in.rs` 34 -
  C reaches both through the system `<stdio.h>`'s own includes rather than through this header) and
  `va_list` spelled `core::ffi::VaList<'_>` (`PORTING.md` P-04, the `xdisp.rs`/`pdumper.rs` shape).
- **The 7 unchecked boxes, named (the whole remaining gap).**
  - `_` (C 59, the guard prefix placeholder) is the *shared* `docs/ports/excluded-macros.txt` row
    (`syntax`) the other gnulib mirrors use too.
  - `getline` appears twice in the checklist: the *macro* item (C 1164, the dead `#   define getline
    rpl_getline`) is **ticked** - the mirror's declaration answers it, because a declaration counts
    for a macro item (`rust_names` includes `functions`) - and the *function* item (C 1155, the dead
    C++ inline definition) stays unchecked: `tools/extract-abi.py` asks for a Rust *definition* of
    a C function, and the port has none (the live `getline` is the system's).  The excluded row
    below records that.
  - **five more names request excluded rows** (`kind=config`), listed verbatim under `## Evidence`.
- **`__has_feature` already has a row** (line 52 of `docs/ports/excluded-macros.txt`), so this
  mirror carries nothing for it and the gate prints it under *"N C macros have a recorded Rust
  counterpart"* - but the row's *reason* is inaccurate on one point and should be corrected: it
  says "the frozen configuration defines neither the builtin nor the macro", while the frozen clang
  **does** define `__has_feature` (measured: `printf '#ifdef __has_feature\nHAS_FEATURE\n#else\nNO\
  n#endif\n' | clang -E -std=gnu23 -` prints `HAS_FEATURE`).  The row's conclusion stands - the
  template's fallback `#  define __has_feature(a) 0` (C 1096-1097) is skipped, so the port has no
  counterpart - and the correction is a wording one for the integrator.
- **Compile evidence note, and a correction to the previous round's ledger.**  `xtask layout-check`
  seeds `femacs/src/include/*.rs` plus the two `foundation` mirrors and then compiles whatever the
  `include!` graph *reaches* (`femacs/xtask/src/main.rs` 795-812): a `lib/include/*.rs` mirror is
  compiled only when some `src/include` mirror includes it.  Measured: `build/femacs/layout_probe.rs`
  lists 99 mirrors = every `src/include/*.rs`, and the materialized set
  (`build/femacs/layout_probe_src/`) contains `…_lib_include_stdbit_in_rs.rs` (reached through
  `src/include/lisp.rs`) but **neither** `…_lib_include_stdlib_in_rs.rs` **nor**
  `…_lib_include_stdio_in_rs.rs` - nothing includes them yet.  So those two files' compile evidence
  is their standalone `rustc` probe, not `layout-check`; the `include-stdlib.in.md` Evidence bullet
  that read "this mirror is one of the 97" is therefore wrong and has been corrected in place (the
  count of 97 was the `src/include` seed count, not evidence about `stdlib.in.rs`).  For this file
  the standalone probe is what caught a real `E0428` (`stdio_decls`), which is the argument for
  keeping it.

## Evidence

- **The judge**: `ls -l build/emacs-ref/lib/stdio.h` -> `107390` bytes; `wc -l` -> `2771`;
  `diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/stdio.in.h) build/emacs-ref/lib/stdio.h | wc -l` ->
  `1150` (877 `>`/`<` lines).
- **`tools/check-cond-eval.py lib/stdio.in.h`** -> `lib/stdio.in.h: 360 conditions, 155 unknown to
  the evaluator, 214 disagreements` (the `[NOT VERIFIED]` half is what the extractor's own output
  prints).
- **The frozen preprocessor's readings** (the arms' authority), the same commands as
  `include-stdlib.in.md`'s Evidence with `stdlib` -> `stdio`:
  - `clang -E -dM -std=gnu23 -include build/emacs-ref/src/config.h -I build/emacs-ref/lib
    -I build/emacs-ref/src -I emacs/src -I emacs/lib build/emacs-ref/lib/stdio.h | grep '^#define'`
    -> fourteen live macros: `_GL_ATTRIBUTE_DEALLOC`, `_GL_ATTRIBUTE_DEALLOC_FREE`,
    `_GL_ATTRIBUTE_FORMAT`, `_GL_ATTRIBUTE_FORMAT_PRINTF_STANDARD`,
    `_GL_ATTRIBUTE_FORMAT_PRINTF_SYSTEM`, `_GL_ATTRIBUTE_FORMAT_SCANF`,
    `_GL_ATTRIBUTE_FORMAT_SCANF_SYSTEM`, `_GL_ATTRIBUTE_MALLOC`, `_GL_ATTRIBUTE_NOTHROW`,
    `_GL_ATTRIBUTE_SPEC_PRINTF_STANDARD` (`__printf__`), `_GL_ATTRIBUTE_SPEC_PRINTF_SYSTEM`
    (`__printf__`), `_GL_STDIO_MACROEXPAND_AND_STRINGIZE`, `_GL_STDIO_STRINGIZE`,
    `_PRINTF_NAN_LEN_MAX` (`3`) - and no `X rpl_X` rename at all.
  - `clang -E … build/emacs-ref/lib/stdio.h | grep -c '\bX\b'` -> 49 of the entry points present,
    and these 6 absent: `fcloseall`, `fwrite_unlocked`, `obstack_printf`, `obstack_vprintf`,
    `rpl_getdelim`, `__has_feature` (the last one is a macro in the template, so it is classified by
    the `-dM` table instead: it is not there, because clang's `__has_feature` is compiler-provided).
  - `printf '#ifdef __has_feature\nHAS_FEATURE\n#else\nNO_HAS_FEATURE\n#endif\n' | clang -E
    -std=gnu23 -` -> `HAS_FEATURE`.
  - `clang -std=gnu23 -fsyntax-only -Xclang -ast-dump -include build/emacs-ref/src/config.h …` on a
    one-line `#include <stdio.h>` file -> the `FunctionDecl`s this mirror's `/// C:` lines quote
    (`int asprintf (char **restrict, const char *restrict, ...)`, `ssize_t getline (char
    **restrict, size_t *restrict, FILE *restrict)`, `int vfprintf (FILE *restrict, const char
    *restrict, __builtin_va_list)`, `off_t ftello (FILE *)`, …).
  - the `getline` block's guard chain in the generated header: `1684: #if 1` ->
    `1691: # if 0` -> `1709: #  else` -> `1717: #  if !1` -> `1718: _GL_FUNCDECL_SYS (getline, …)`,
    with `1711: #   define getline rpl_getline` inside the `# if 0` arm.
- **`--check` verdict with the six requested rows**: a `/tmp` copy of the excluded table plus the
  six rows, passed through `--excluded`/`--macro-notes`,
  `python3 tools/extract-abi.py --check --excluded … --macro-notes … emacs/lib/stdio.in.h
  femacs/lib/include/stdio.in.rs` -> `OK emacs/lib/stdio.in.h -> femacs/lib/include/stdio.in.rs`
  and `grep -c '^- '` -> `0`; with the tree's tables it reports exactly those six lines.
- **The six rows requested** (`<TAB>` is a literal tab), all `kind=config`:
  `getline<TAB>config<TAB>stdio.in.h 1155-1162: the file's one C *function* is the C++ arm's inline definition inside C 1144's `# if @REPLACE_GETLINE@`, which the generated header substitutes to `# if 0` (line 1691) - dead text.  The live `getline` is the *system's* (generated header 1717 `#  if !1` = `!@HAVE_DECL_GETLINE@`, so even gnulib's own `_GL_FUNCDECL_SYS` is skipped); the mirror declares it in its `stdio_in_decls` module, so the port has the name but no definition, which is what this row records`
  `fcloseall<TAB>config<TAB>stdio.in.h 426-445: inside C 426's `#if @GNULIB_MDA_FCLOSEALL@`, whose arm is dead on this target (the generated header substitutes it to `#if 0`); the arm's own body is the Windows rename `#   define fcloseall _fcloseall`, and no macOS header has `fcloseall` (measured: the name is absent from the frozen translation unit)`
  `fwrite_unlocked<TAB>config<TAB>stdio.in.h 1035-1049: inside the glibc `#  if defined __GLIBC__ && !defined __UCLIBC__` block, dead here; the arm's body is `#   define fwrite_unlocked rpl_fwrite_unlocked` and macOS has no `fwrite_unlocked` (measured: absent from the frozen translation unit)`
  `obstack_printf<TAB>config<TAB>stdio.in.h 1248-1280: the whole block is C 1248's `#if @GNULIB_OBSTACK_PRINTF@ || @GNULIB_OBSTACK_PRINTF_POSIX@`, which the generated header substitutes to `#if 0`; on this target the `#   define obstack_printf rpl_obstack_printf` rename and the `_GL_FUNCDECL_SYS` are both dead text and the frozen translation unit has no `obstack_printf` (measured)`
  `obstack_vprintf<TAB>config<TAB>stdio.in.h 1277-1290: the same dead `@GNULIB_OBSTACK_PRINTF@ || @GNULIB_OBSTACK_PRINTF_POSIX@` block (`#   define obstack_vprintf rpl_obstack_vprintf`)`
  `rpl_getdelim<TAB>config<TAB>stdio.in.h 1103-1105: inside C 1090's `# if @REPLACE_GETDELIM@`, dead on this target (the generated header substitutes it to `# if 0`, line 1636) and the arm's body is the glibc-only `#   define rpl_getdelim __getdelim`; the frozen `getdelim` is the SDK's, which the mirror declares`
- **Compile evidence (standalone; `layout-check` does not reach this file yet)**:
  `rustc +nightly-2026-10-01 --edition 2024 --crate-type bin -C panic=abort -o /tmp/st_p/P
  /tmp/st_p/P.rs` (the probe `include!`s this mirror with the `std::ffi` alias set, takes
  `getline`'s address as an `unsafe extern "C" fn` and prints the constant) -> exit 0, and running
  it prints `stdio.in.rs probe ok: _PRINTF_NAN_LEN_MAX=3, getline=0x…`, exit 0.  The same probe is
  what measured the `stdio_decls` clash before the rename (that run printed
  `error[E0428]: the name `stdio_decls` is defined multiple times --> …/stdio.rs:23:1`).
  `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-stdio cargo run -q -p xtask -- layout-check` ->
  `layout-check: 89 of 99 mirror(s) compiled, 10 platform-exclusive skipped (…)`, `layout probe ok`,
  `layout-check passed`, exit 0 - for the *whole* crate, of which this file is not yet a member
  (see the Decisions bullet above): `grep -c 'lib/include/stdio.in.rs' build/femacs/layout_probe.rs`
  -> `0`, and `build/femacs/layout_probe_src/` has no `…_stdio_in_rs.rs`.  No unit includes this
  mirror either (`grep -rn 'include!.*stdio.in.rs' femacs` -> nothing), so there is no `build-tu`
  consumer to show.
- **Gates on this worktree** (last lines): `check-declarations` -> `13 declaration sites (0
  forbidden, 13 allowed literals), 0 failures`; `check-globals` -> `283 units, 0 DEFVAR_*
  variables without a definition in their own unit`; `check-mirror-dupes` -> `1 duplicate
  declaration(s), 1 ratchet row(s), 0 failure(s)`; `check-verbatim` -> `6 trees, 4607 files,
  0 failures`; `check-target-names` -> `615 .rs files, 593 map targets, 0 finding(s)`.
