# emacs/lib-src/seccomp-filter.c (409 lines)

> **This file is P-86's class: the frozen configuration compiles nothing from it, and the gate is
> the build's own, not a `#if` in the text.**  `lib-src/Makefile.in` 232-246 sets `SECCOMP_FILTER=1`
> only when `HAVE_SECCOMP=yes` (`configure.ac` 5918-5927: `<linux/seccomp.h>`, `<linux/filter.h>`
> and the two `SECCOMP_*` declarations), `HAVE_LIBSECCOMP=yes` (`configure.ac` 5929:
> `libseccomp >= 2.5.2`), `uname -m` is `x86_64`, `SIZEOF_LONG` is 8 and the kernel is at least
> 4.14; `lib-src/Makefile.in` 248-250 then adds the program to `DONT_INSTALL`, which is the list
> that builds it.  This target is darwin: `docs/baseline/frozen-build-sources.txt` records no
> `lib-src/seccomp-filter.o` (its batch-F section names this file among those in no list at all),
> `tools/extract-abi.py`'s checklist for the file is empty (P-86 2), and the port's frozen object
> is the 336-byte empty one (Evidence 1) - so this ledger carries the **C line-range table**, the
> `## Evidence` readings and the `## Deviations` instead of ticked boxes (P-86 2).
>
> Port: `femacs/lib-src/seccomp-filter.rs`, **the whole file, C 1-409** (1,071 Rust lines), both
> arms of every gate carried (P-86 1) and nothing stubbed.  The old revision's conclusion ("not in
> the frozen build, so the port is the empty unit plus a note") was P-86 1's "not started" case;
> its measurements are still true and are where the file gate comes from.

## Gate mapping

| C condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`lib-src/Makefile.in` 232-250's `SECCOMP_FILTER`, under 232-246's five tests) | `feature = "seccomp_filter"`, on every item *and* on the include graph (P-86 8) | an x86-64 GNU/Linux configuration with libseccomp and a 4.14+ kernel |
| C 65-67 `#ifndef ARCH_CET_STATUS` / `#define ARCH_CET_STATUS 0x3001` | the C 49 `<asm/prctl.h>` section carries `all (feature = "seccomp_filter", feature = "arch_cet_status")`; C 65-67 carries `not (feature = "arch_cet_status")` | one arm of the pair, per system header |
| C 69-72 `#ifndef MAP_DROPPABLE` / `#define MAP_DROPPABLE 0x08` | the same pair, with `feature = "map_droppable"` (C 51's `<sys/mman.h>` section is the other arm) | one arm of the pair, per system header |
| C 259-261 `#ifdef __NR_faccessat2` | `feature = "__nr_faccessat2"` | a kernel header that defines `__NR_faccessat2` |

`configure.ac` 5929's `HAVE_LIBSECCOMP` and the two `SECCOMP_*` declarations of 5919-5926 are
*configure* results, not macros this file's text tests, so they have no port gate of their own:
the `seccomp_filter` gate stands for the whole `SECCOMP_FILTER` conjunction, exactly as
`docs/ports/update-game-score.md`'s `use_gamedir` stands for its own Makefile variable.
C 259-261's `#endif` has no `#else`, so the arm where the architecture has no `__NR_faccessat2` is
empty in C: nothing is transcribed for it, and setting the feature only adds the one rule.

## C line ranges

One row per C block, in C's order, contiguous (each row absorbs the blank lines around it),
covering C 1-409.  `docs/port-map.md` 42 pairs `lib-src/seccomp-filter.c` with this unit (there is
no `lib-src/seccomp-filter.h` binding), so one table is what `check-dead-file-coverage.py` reads.

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's declaration block (the `/* Generate a Secure Computing filter definition file.` head through the closing comment) | dropped (P-86 1, PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 20-34 | C 21-34's description: usage, the two raw files and the two `-exec` variants | carried as the port's header prose, file gate |
| 35-38 | C 36 `#include "config.h"`, C 38 `#include <assert.h>` | file gate; C 36 is the port's only mirror include (`src/include/config.rs`), C 38 is the port's own `assert!` macro with the frozen `NDEBUG` body |
| 39-45 | `<errno.h>` (39), `<limits.h>` (40), `<stdarg.h>` (41), `<stdcountof.h>` (42), `<stdlib.h>` (43), `<stdint.h>` (44), `<stdio.h>` (45) | file gate; declared in-unit (P-88 1) - `<stdarg.h>` names nothing (P-04's `...` parameter) |
| 46-54 | `<asm/termbits.h>` (46), `<time.h>` (47), `<asm/prctl.h>` (49, with the `arch_cet_status` arm), `<sys/ioctl.h>` (50), `<sys/mman.h>` (51, with the `map_droppable` arm), `<sys/prctl.h>` (52), `<sys/types.h>` (53), `<sys/stat.h>` (54) | file gate; C 49/51 carry the sub-gates; C 53/54 name nothing |
| 55-63 | `<linux/futex.h>` (55), `<linux/filter.h>` (56), `<linux/seccomp.h>` (57), `<fcntl.h>` (58), `<sched.h>` (59), `<seccomp.h>` (60), `<unistd.h>` (61), `<attribute.h>` (63) | file gate; declared in-unit; C 56 names nothing and C 63 is an attribute with no Rust counterpart (PORTING.md 8) |
| 64-67 | `#ifndef ARCH_CET_STATUS` / `#define ARCH_CET_STATUS 0x3001` / `#endif` (C 65-67) | `not (feature = "arch_cet_status")`; the header's arm is C 49's |
| 68-72 | `#ifndef MAP_DROPPABLE` / `#define MAP_DROPPABLE 0x08` / `#endif` (C 69-72) | `not (feature = "map_droppable")`; the header's arm is C 51's |
| 73-91 | `fail` (C 74-91: the `ATTRIBUTE_FORMAT_PRINTF`/`_Noreturn` prototype, the `vfprintf`, the `fputc`/`fputs`/`errno`/`perror` split, `fflush`, `exit`) | file gate; the `va_list`/`va_start`/`va_end` row is in `macro-body-notes.tsv` (P-04) |
| 92-96 | the single-context comment (93-94) and `static scmp_filter_ctx ctx;` (96) | file gate; zero-initialised (PORTING.md 1.5) |
| 97-102 | `release_context` (C 98-102) | file gate; `unsafe extern "C" fn` because `atexit` takes its address (PORTING.md 1.4) |
| 103-113 | the "wrapper functions and macros" comment (104-105) and `set_attribute` (107-113) | file gate |
| 114-139 | the `RULE` comment (115-117), `RULE` (119-130) and `RULE0` (131-139) | file gate; both macro-body rows (the `do`/`while (false)` shape, `__VA_ARGS__`) |
| 140-159 | `export_filter` (C 141-159: the `open` retry loop, the export call, the `close` check) | file gate; C 149's flag set is the `O_CLOEXEC` deviation's site |
| 160-162 | `EXPORT_FILTER` (C 161-162) | file gate |
| 163-409 | `main` (C 164-409: the `argc` check, `seccomp_init`/`atexit`, the three attributes, the `static_assert`/`assert` block, 78 `SCMP_SYS` rules, the four `EXPORT_FILTER` sites) | file gate; C 259-261 `feature = "__nr_faccessat2"` (no `#else`) |

## Evidence

1. **Frozen.**  `cd femacs && CARGO_TARGET_DIR=/tmp/seccomp-lane cargo run -q -p xtask -- build-tu
   lib-src/seccomp-filter.c` -> `built lib-src/seccomp-filter.rs -> ../build/femacs/seccomp-filter.o`,
   exit 0, and the object is **336 bytes** (md5 `dbed68837694579185377dbb0821ab62`) - the empty
   object, which is the judgement P-86 8 uses for "the include graph is under the file gate".
2. **The arm flip, with the file gate *and* every sub-gate (P-86 6).**  `… -- probe
   lib-src/seccomp-filter.rs /tmp/seccomp-lane/probeB.rlib --crate-type=lib -C panic=abort
   --cfg 'feature="seccomp_filter"' --cfg 'feature="arch_cet_status"' --cfg
   'feature="map_droppable"' --cfg 'feature="__nr_faccessat2"'` -> **exit 0, 0 errors**, a
   **129,664-byte** rlib.  A second reading of the same materialized root with the file gate
   *alone* (which selects C 65-72's fallback arms and leaves out C 260's one rule) is `rustc`'s,
   exit 0, a 128,728-byte rlib - taken directly so the `probe` budget stays inside its three runs.
3. **Mutation reading (P-86 7).**  The materialized root `build/probes/seccomp-filter/_root.rs`
   compiles with arm 2's gate set: exit 0, **0 errors**, a 129,624-byte rlib.  With one type error
   injected into the unit's own text (`let _mutation: c_int = c"injected".as_ptr ();`, before
   C 182's `static_assert! (CHAR_BIT == 8);`), the same compile exits **1** with the `E0308` that
   names the injected line (839) - so the 0-error reading is a reading of *this unit's text*, not
   the "broken include graph stops before the unit" empty reading.  The graph here is the unit plus
   one 1,463-byte copy of `config.rs`.  The toolchain is the pinned one
   (`femacs/rust-toolchain.toml`): from the repository root a bare `rustc` reports `E0658:
   C-variadic functions are unstable` on `fail`, which is P-04's pin, not this unit's error.
4. **Budget, as run.**  `build-tu` 2 runs (the pre-fix revision and the final one) of the 3
   allowed; `probe` 3 runs (arm B twice - once before the final comment-only edit - and arm A
   once) of the 3 allowed; `cargo test`/`--all`/`gate-fast`/`check-all-units.sh` were not run.  The
   readings above are all of the final revision of `femacs/lib-src/seccomp-filter.rs`.

5. **The Linux half is measured, not remembered** (P-88 2).  Readings are `clang`'s, via the
   NDK 29.0.14206865 sysroot as the closest local x86-64 Linux target
   (`~/Library/Android/sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/darwin-x86_64/bin/clang
   --target=x86_64-linux-android29 --sysroot=<that ndk>/sysroot -dM -E -include <hdr>` for the
   macros and `-Xclang -fdump-record-layouts` for the record): `SECCOMP_SET_MODE_FILTER` 1,
   `SECCOMP_FILTER_FLAG_TSYNC` `(1UL << 0)`, `SECCOMP_RET_KILL_PROCESS` `0x80000000U`,
   `FUTEX_WAKE` 1 / `FUTEX_PRIVATE_FLAG` 128, `ARCH_SET_FS` 0x1002, `TCGETS` 0x5401,
   `TIOCGPGRP` 0x540F, `PROT_*` 0/1/2/4, `MAP_SHARED` 0x01 / `MAP_PRIVATE` 0x02 / `MAP_FILE` 0 /
   `MAP_ANONYMOUS` 0x20 / `MAP_FIXED` 0x10 / `MAP_DENYWRITE` 0x0800 / `MAP_STACK` 0x20000 /
   `MAP_NORESERVE` 0x4000 / `MAP_DROPPABLE` 0x08, the nine `CLONE_*`, `PR_SET_NAME` 15 /
   `PR_CAPBSET_READ` 23 / `PR_SET_NO_NEW_PRIVS` 38, `O_RDONLY` 00000000 / `O_WRONLY` 00000001 /
   `O_RDWR` 00000002 / `O_CREAT` 00000100 / `O_TRUNC` 00001000 / **`O_CLOEXEC` 02000000** /
   **`O_PATH` 010000000** / **`O_DIRECTORY` 00200000** / **`O_NOFOLLOW` 00400000** / `O_BINARY` 0
   (the gnulib wrapper's own fallback) / `F_GETFL` 3, `CLOCK_REALTIME` 0, `STDIN_FILENO` 0 /
   `STDERR_FILENO` 2, `EPERM` 1 / `EACCES` 13 / `EINVAL` 22 / `EINTR` 4, `EXIT_FAILURE` 1,
   `CHAR_BIT` 8, `INT_MIN`/`INT_MAX`/`INT32_MIN`/`INT32_MAX`, `LONG_MIN`/`LONG_MAX`,
   `INT64_MIN`/`INT64_MAX`; `struct scmp_arg_cmp` `[sizeof=24, align=8]` with `arg`@0, `op`@4,
   `datum_a`@8, `datum_b`@16; and the 72 `SCMP_SYS` names' numbers, 66 of them from
   `<asm/unistd_64.h>` (`exit` 60, `mmap` 9, `openat` 257, `clone3` 435, `seccomp` 317, …).
   The four flags in bold are the ones the frozen target's own mirror carries *differently*:
   `lib/include/fcntl.in.rs` 94/118 records `O_CLOEXEC=16777216 O_DIRECTORY=1048576
   O_NOFOLLOW=256` (darwin's), which is Deviation 1's worked example.
6. **The libseccomp half is read from the library's own headers.**  The library is not installed on
   this machine (`find` finds no `seccomp.h`/`seccomp-syscalls.h`; `/opt/homebrew/include` has
   none), so `include/seccomp.h.in` and `include/seccomp-syscalls.h` were fetched at **v2.5.2** -
   the version `configure.ac` 5929 requires at minimum - and read: `SCMP_CMP_NE` 1 / `SCMP_CMP_EQ`
   4 / `SCMP_CMP_MASKED_EQ` 7 (`seccomp.h.in` 88-94), `SCMP_FLTATR_ACT_BADARCH` 2 / `CTL_NNP` 3 /
   `CTL_TSYNC` 4 (65-68), `SCMP_ACT_KILL_PROCESS` `0x80000000U` / `SCMP_ACT_ALLOW` `0x7fff0000U` /
   `SCMP_ACT_ERRNO(x)` `(0x00050000U | ((x) & 0x0000ffffU))` (332-364), `typedef void
   *scmp_filter_ctx` (58), `typedef uint64_t scmp_datum_t` (103), the four `struct scmp_arg_cmp`
   members (106-111), `SCMP_CMP64`'s compound literal / `SCMP_CMP32`'s `_SCMP_CMP32_{1,2}`
   dispatcher / the nine `SCMP_A*` wrappers (202-306), the seven prototypes (443/468/580/662/682/
   800/811), `SCMP_SYS(x) (__SNR_##x)` (223), and the six `__PNR_*` pseudo numbers
   `seccomp-syscalls.h` maps the x86-64-absent names to (`fcntl64` -10009, `fstatat64` -10011,
   `lstat64` -10028, `sigaction` -10054, `sigprocmask` -10057, `stat64` -10062; lines
   78/80/97/123/126/131 and their `__SNR_*` pairs at 619/681/1065/1911/1939/2001).
7. **The ABI check.**  `python3 tools/extract-abi.py --check emacs/lib-src/seccomp-filter.c
   femacs/lib-src/seccomp-filter.rs` -> one row, `line count drifted 161.9%: C 409 vs Rust 1071`
   (Owed 1).  The three token rows this port does have (`RULE`, `RULE0`, `fail`) are rows 1422-1424
   of `docs/ports/macro-body-notes.tsv`, which this lane wrote; there is no `missing` row, so every
   C name (5 functions, 5 macros, 2 constants, 1 file-scope object) is carried.
8. **The other gates.**  `tools/check-declarations.py`: 0 findings for this unit (the product name
   is not in the file's text or comments; the description's own "Emacs" is C's word, not the
   product-name pattern).  `tools/check-mirror-dupes.py`: `lib-src` is outside its `UNIT_DIRS`
   scope, so the unit has no rows (its 7 FAILs are other lanes' in-flight files).
   `tools/check-citations.py`: 0 findings for this unit (the system headers are counted as
   `external`, never as failures).

## Deviations

1. **The system headers are declared in this unit, and the frozen target's mirrors are not
   consumed** (P-88 1).  Twenty-four of C 36-63's includes are headers of the absent x86-64
   GNU/Linux toolchain; the reference tree has no file for any of them, so each one's names are
   declared at its own `#include` line.  The tree *does* have mirrors for some of the same names
   (`lib/include/fcntl.in.rs`, `limits.in.rs`, `stdio.rs`, `stdlib.rs`, `errno.rs`,
   `unistd.in.rs`, `stdint.in.rs`), but their values are the **frozen target's** and this program
   is built only for x86-64 GNU/Linux, so consuming them would put the wrong flag values into the
   filter: `O_CLOEXEC` is the worked example (the mirror's measured 0x1000000 against Linux's
   0x80000), and `O_PATH`/`O_DIRECTORY`/`O_NOFOLLOW` join it (Evidence 5).
2. **C 38's `assert` carries the frozen `NDEBUG` body.**  `src/conf_post.h` 28-29 derives `NDEBUG`
   from the absent `ENABLE_CHECKING`, and `docs/baseline/config/darwin-arm64.macros.tsv` 2892
   records it for the frozen configuration, so C's macro is `((void) 0)` and does not evaluate its
   operand (PORTING.md 2.5; the `lib-src/ebrowse.rs` 93 reading).  This file includes only
   `config.h` (not `lisp.h`, hence not `conf_post.h`), so a real `--with-seccomp` lib-src build may
   well have the *live* `assert` - the port spells the frozen branch (P-08) and no
   `__assert_fail` is declared.
3. **`errno` is the port's accessor.**  C's `errno` is a macro over a function; the port keeps C's
   name at the assignment sites (`*errno () = error;`) with `lib/include/errno.rs` 17's shape, and
   the accessor below it is the x86-64 Linux spelling `__errno_location ()` (this machine's is
   `__error ()`).
4. **`SCMP_SYS`'s token pasting has no Rust counterpart** (the `comp.rs` `DEF_DLL_FN` reading):
   the one macro keeps C's name and each of the 72 names the text passes gets an arm carrying the
   number `__SNR_##x` has on x86-64.  The `__SNR_*`/`__NR_*`/`__PNR_*` layer of
   `seccomp-syscalls.h` is folded into those arms and is not spelled out as its own constants.
5. **`SCMP_CMP64`'s compound literal** `((struct scmp_arg_cmp){__VA_ARGS__})` zero-fills the
   members `__VA_ARGS__` does not name and widens them to `scmp_datum_t`; the port writes both
   (`datum_b: 0`, `($a) as scmp_datum_t`).  `SCMP_CMP32` keeps C's own two `(uint32_t)` casts, so
   the negative masks C passes (C 201-223's `~(…)`) go through the same truncation.
6. **`RULE`/`RULE0`'s `do { … } while (false)` is a block expression** (PORTING.md 2.1);
   `#action`/`#syscall`/`#__VA_ARGS__` are `stringify!`; `enum { arg_cnt = countof (arg_array) };`
   is `countof!` over the same array, which makes it a value rather than a compile-time constant.
   The `arg_array` declaration keeps C's element type (`[scmp_arg_cmp; _]`).  The message the two
   macros hand `fail` is C's format string with C's four conversions; what fills them is this
   port's own tokens, so the last one reads `SCMP_A2_32! (SCMP_CMP_MASKED_EQ, !(PROT_NONE | ...))`
   where C's `#__VA_ARGS__` reads `SCMP_A2_32 (SCMP_CMP_MASKED_EQ, ~(PROT_NONE | ...))` (C's `~`
   and Rust's `!` are the same operator, Deviation 8).
7. **`release_context` is `unsafe extern "C" fn`**: C hands its address to `atexit`, and C's one
   function-pointer ABI is the port's one (PORTING.md 1.4).
8. **C's `~` is the port's `!`** (eight masks, C 201-356), and C's three `sizeof (T)` are
   `core::mem::size_of::<T> ()` (the `sizeof` alias family).
9. **Uninitialised C locals are zero-initialised** (PORTING.md 1.5): `fd` in `export_filter`.
   `ctx` (C 96) is a `static` and takes `core::ptr::null_mut ()` the same way.
10. **`argv[n]` is `*argv.offset (n)`** at the four `EXPORT_FILTER` sites; `0644` is `0o644`, and
    the octal `#define`s C writes without a prefix are Rust's `0o…` literals (the values are C's).
11. **`static_assert` is the port's own macro** (PORTING.md 2.5: a `const` item, written with the
    path-qualified `core::assert!` so this unit's own `assert!` cannot shadow it - status 3.432).
    C 182-188's `static_assert`/`assert` block is otherwise C's statements in C's order.
12. **C's `main` ends without a `return`** (C 409: C99 gives it an implicit `return 0`); the port
    writes `return 0;` as the last statement of the function.
13. **C 1-19 is dropped** (PORTING.md 0.1-19); C 21-34's description is carried, with its own
    wording (the program's three "Emacs" mentions are C's text and are not the product-name
    pattern `tools/check-declarations.py` reads).
14. **Five of the includes declare nothing**: `<stdarg.h>` (P-04's `...` parameter),
    `<linux/filter.h>`, `<sys/types.h>`, `<sys/stat.h>` and `<attribute.h>` - the text names no
    entity from them.  They are still recorded at their C lines, and C 56's `struct sock_filter`
    appears only in C 21-34's description.
15. **`ARCH_CET_STATUS`'s header arm is UNVERIFIED as a header reading** (Owed 2); the
    `map_droppable` arm is measured (Evidence 5), and the `__nr_faccessat2` arm is a kernel-header
    test the NDK sysroot does define.
16. **Line count**: C 409 versus Rust 1,071 (**161.9 %** drift).  The expansion is the P-88 1
    declaration block: 113 gate lines and 100 `/// C:` lines for the 24 absent headers' names
    (~120 declarations), the 72 `SCMP_SYS` arms and their comment, and the Rust spellings of C's
    statement shapes.  A `docs/ports/line-drift-tolerance.tsv` row is owed (Owed 1).

## Owed by the integrator (nothing here is claimed as done)

1. `docs/ports/line-drift-tolerance.tsv`: **one row, stem `seccomp-filter`, tolerance ≥ 1.62,
   basis `total`** (Deviation 16; this is a whole-file port, so a row may be requested - P-86 10's
   rule needs it, and without it `extract-abi --check`'s only row is the drift).
2. **`arch_cet_status`'s value as a *header* reading** (Deviation 15): the system header is not on
   this machine and the NDK sysroot's `asm/prctl.h` does not define the name, so the arm carries
   the value C's own fallback arm gives (0x3001).  A GNU/Linux build should re-measure it.
3. `docs/baseline/feature-coverage.tsv`: `seccomp_filter`, `arch_cet_status`, `map_droppable` and
   `__nr_faccessat2` are new; the file is generated (`tools/feature-coverage.py --write`).
4. `docs/baseline/port-lines-pending.tsv`: the `lib-src/seccomp-filter.c` row (409 / 12 / 40,
   "known-unported target (plan batch F)") is stale now that the port is the worked file -
   `tools/check-port-lines.py --write-pending`.
5. The **mirrors' target valve** (Deviation 1) is a tree question, not this unit's: if the mirrors
   ever carry a per-target value, the four `<fcntl.h>` flags and the `errno` accessor are the names
   this unit could then take from them instead of declaring.
6. A **glibc x86-64 build should re-measure the Linux half** (Evidence 5's proxy is the NDK's
   bionic sysroot; the kernel-ABI and `_64`-suffixed syscall numbers are the kernel's own, while
   the libc-provided constants are that sysroot's x86-64 reading).

## Not claimed

- **Nothing runs.**  No filter was generated, no `seccomp_rule_add`/`seccomp_export_*` was called,
  no file was written and no `open` retry loop was exercised.  The two arm readings are *compile*
  readings (a 128,768-byte and a 129,664-byte rlib that no one linked or executed), and the frozen
  object is 336 bytes of empty object by construction.
- **This is P-86 3(2)'s `text-only` case**, not 3(1)'s "the class can be verified on this
  machine": there is no x86-64 GNU/Linux toolchain here, no glibc header tree and no installed
  libseccomp, so the values above are a *proxy* (the NDK's x86-64 sysroot) and a *fetched* header
  (libseccomp v2.5.2) rather than the target's own build inputs.
- The libseccomp prototypes, `scmp_filter_ctx`'s pointee and the six pseudo numbers are readings of
  that fetched header, not measurements of an installed library of this target; the units of
  `seccomp_attr_set`/`seccomp_rule_add`'s variadic arguments are C's, unchecked.
- C 259-261's arm is compiled under `__nr_faccessat2` but the *rule it adds* was never handed to a
  kernel: whether `faccessat2` exists at run time is the kernel's, not this reading's.
- Only `femacs/lib-src/seccomp-filter.rs`, this ledger, and rows 1422-1424 of
  `docs/ports/macro-body-notes.tsv` were written by this lane.
