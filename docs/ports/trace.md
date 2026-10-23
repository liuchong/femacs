# emacs/exec/trace.c (2258 lines)

> **One of P-86's class: the frozen configuration compiles nothing here.**  `configure.ac` 178-220
> configures `exec/` with the build directory set to `exec` only inside the `XCONFIGURE=android`
> block (1343 sets that variable from `--with-android`), and `exec/configure.ac` 235 puts `trace.o`
> into the `OBJS` list that `exec/Makefile.in`'s `libexec.a` rule - and the `loader` rule that links
> it - are built from.  `docs/baseline/frozen-build-sources.txt`'s batch-F section records no
> `exec/` object for this target, so `tools/extract-abi.py`'s checklist for the file is empty (P-86
> ②) and the frozen object is the 336-byte empty one (Evidence ①).  The **file gate is
> `feature = "have_android"`** - the registered name for the build this file belongs to
> (`docs/baseline/feature-coverage.tsv` 11, the gate `src/androidmenu.rs` and `src/sfnt.rs` use) - on
> every item *and* on the include graph (P-86 ⑧).
>
> Port: `exec/trace.rs` (**3,106 lines** before this ledger's rows; first line
> `// femacs: port of exec/trace.c @ fd7afced403.`), the whole file, C 1-2258, **both arms of every
> one of its 73 conditions** (P-86 ①) and nothing stubbed.  The previous revision was the three-line
> P-08 note ("not compiled by the frozen configuration … the port is an empty unit").
>
> **The file's own target is measurable on this machine.**  The Android NDK is installed here
> (29.0.14206865, the one `spikes/jni-probe.sh` defaults to), so its `clang` is the *target's*
> compiler: `aarch64-linux-android24`, `x86_64-linux-android24` and `armv7a-linux-androideabi24`
> each answer `-dM -E` and `-Xclang -fdump-record-layouts-complete` for the Linux surface this file
> reads.  Every constant and record offset the unit declares is one of those readings, except where
> its line says **UNVERIFIED** and names the source instead (the mipsel host, whose
> `exec/configure.ac` 390-400 arm has no toolchain here).

## Gate mapping

The C text is generic across the five hosts `exec/configure.ac` 240-400 configures
(`x86_64-*linux*`, `i[34567]86-*linux*`, `aarch64-*linux*`, `arm*linux*eabi* | armv7*linux*`,
`mipsel*linux*`).  Row per condition, in C's order; "both arms" is the port's rule everywhere (P-86
①), and the third column names the feature the *first* arm carries.  `check-cond-eval.py` reads the
file as **73 conditions, 0 unknown, 0 disagreements [VERIFIED]** (Evidence ⑤), so every one of them
is decided by the frozen macro table and the port's split is the evaluator's own.

| C `#if` / `#ifdef` / `#ifndef` | tests | feature (first arm) | second arm |
| --- | --- | --- | --- |
| 41 | `__aarch64__` | `aarch64` | `not (aarch64)` |
| 46 | `HAVE_SYS_UIO_H` | `have_sys_uio_h` | `not` |
| 48 | `!HAVE_PROCESS_VM` | `have_process_vm` | `not` |
| 53 | `!SYS_SECCOMP` (fallback pair) | `sys_seccomp` (the header's arm, at C 62) | `not` |
| 57 | `!PTRACE_GETEVENTMSG` (pair) | `ptrace_geteventmsg` (header's arm, at C 22) | `not` |
| 61 | `HAVE_SECCOMP` | `have_seccomp` | `not` |
| 86 | `HAVE_SYS_UIO_H && !HAVE_PROCESS_VM` | `all (have_sys_uio_h, not (have_process_vm))` | — |
| 102 | `HAVE_SECCOMP` | `have_seccomp` | `not` |
| 111 | `__aarch64__` | `aarch64` | `not` |
| 186 | `HAVE_SYS_UIO_H` | `have_sys_uio_h` | `not` |
| 200 | `!HAVE_PROCESS_VM` (nested) | `not (have_process_vm)` | — |
| 261 | `RED_ZONE_SIZE` | `red_zone_size` | `not` (live: nothing defines the name) |
| 265 | `STACK_GROWS_DOWNWARDS` | `not (no_stack_grows_downwards)` | `no_stack_grows_downwards` |
| 276 | `STACK_GROWS_DOWNWARDS` | `not (no_stack_grows_downwards)` | `no_stack_grows_downwards` |
| 295 | `__aarch64__` | `aarch64` | `not` |
| 305 | `STACK_GROWS_DOWNWARDS` | `not (no_stack_grows_downwards)` | `no_stack_grows_downwards` |
| 326 | `HAVE_SYS_UIO_H` | `have_sys_uio_h` | `not` |
| 338 | `!HAVE_PROCESS_VM` (nested) | `not (have_process_vm)` | — |
| 591 | `HAVE_SECCOMP` | `have_seccomp` | — |
| 679 | `SIGSYS` | `sigsys` | no `#else` (nothing to carry) |
| 687 | `HAVE_SIGINFO_T_SI_SYSCALL` | `have_siginfo_t_si_syscall` | `not` |
| 688 | `!__arm__` | `not (arm)` | `arm` |
| 798, 868, 1202, 1231, 1343, 1477, 1500, 1822, 1845 | `__aarch64__` | `aarch64` | `not` |
| 889 | `!STACK_GROWS_DOWNWARDS` | `no_stack_grows_downwards` (`compile_error!`) | `not` (live) |
| 913 | `0` | dead in every configuration: dropped with a citation (the `src/w32term.rs` 685 shape) | — |
| 933 | `!defined HAVE_STPCPY \|\| !defined HAVE_DECL_STPCPY` | `any (not (have_stpcpy), not (have_decl_stpcpy))` | `all (have_stpcpy, have_decl_stpcpy)` |
| 1049, 1290, 1685 | `READLINK_SYSCALL` | `readlink_syscall` | `not` |
| 1139, 1195, 1314, 1366, 1702 | `OPEN_SYSCALL` | `open_syscall` | `not` |
| 1380, 1514, 1888, 1937, 2155, 2218, 2224 | `HAVE_SECCOMP` | `have_seccomp` | `not` / `#endif` only |
| 1383, 1531, 1535, 1571, 1581 | `OPEN_SYSCALL` / `READLINK_SYSCALL` (nested) | `open_syscall` / `readlink_syscall` | — |
| 1416, 1761, 1982 | `__aarch64__` | `aarch64` | `not` (1416/1761 also carry `arm`) |
| 1421, 1766 | `__arm__` | `arm` | `not` |
| 1468, 1491, 1813, 1836 | `__mips__` | `mips` | `not` |
| 1856 | `!PTRACE_EVENT_SECCOMP` (pair) | `ptrace_event_seccomp` (header's arm) | `not` (C's fallback) |
| 1860 | `!PTRACE_O_TRACESECCOMP` (pair) | `ptrace_o_traceseccomp` (header's arm) | `not` |
| 1864 | `!SIGSYS` (pair) | `sigsys` (header's arm, at C 30) | `not` (C's fallback) |
| 1909, 1952 | `defined HAVE_SECCOMP && __ANDROID__` | `all (have_seccomp, __android__)` | `not` |
| 2163 | `SIGSYS` | `sigsys` | no `#else` |
| 2171, 2172 | `HAVE_SIGINFO_T_SI_SYSCALL`, `!__arm__` | `have_siginfo_t_si_syscall`, `not (arm)` | `not`, `arm` |
| 2250 | `HAVE_SYS_UIO_H && !HAVE_PROCESS_VM` | `all (have_sys_uio_h, not (have_process_vm))` | — |

Two readings of that table are the file's own, not the porter's choices: `STACK_GROWS_DOWNWARDS` is
`AC_DEFINE`d on **all five** hosts (269/302/335/368), so C 276-290, C 305-309 and C 889-891's live
arm is always the downwards one and `no_stack_grows_downwards` (P-51's non-live-side name) carries
the other; and `RED_ZONE_SIZE` is defined by **no** configuration at all - `exec/configure.ac` 124
templates `ABI_RED_ZONE` and 255 defines it only in the x86-64 arm - so C 261's block is dead text
everywhere (Deviation 4).

## C line ranges

Rows are contiguous, ascending and cover C 1-2258; `docs/port-map.md` binds this ledger to
`exec/trace.c` alone (the reference has no `exec/trace.h`).

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's declaration block | dropped (PORTING.md 0.1-19) |
| 20-40 | the include block: `config.h` (20), `<sys/ptrace.h>` … `<fcntl.h>` (22-34), `"exec.h"` (36), `SYSCALL_HEADER`/`USER_HEADER` (38-39) | file gate for the graph; the two `AC_DEFINE`d headers have no Rust counterpart (declared in-unit, P-88 ①) |
| 41-70 | the five conditional include blocks: `__aarch64__` (41-44), `HAVE_SYS_UIO_H` with the nested `HAVE_PROCESS_VM` (46-51), the `SYS_SECCOMP` fallback pair (53-55), the `PTRACE_GETEVENTMSG` fallback pair (57-59), `HAVE_SECCOMP` (61-69) | both arms of each (see the table) |
| 71-85 | the "Program tracing functions" commentary (73-79) and `MAX_TRACEES` (83-84) | — |
| 86-101 | the two `process_vm_readv`/`process_vm_writev` function pointers | `all (have_sys_uio_h, not (have_process_vm))`; C's pointer-as-condition is `is_some ()` |
| 102-110 | `use_seccomp_p` (105) and C's `#else` `#define use_seccomp_p (false)` (107-108) | `have_seccomp` / `not` |
| 111-166 | `aarch64_get_regs` (116-126), `aarch64_set_regs` (133-163) | `aarch64` (`regs->regs[8]` at C 149 and the `NT_ARM_SYSTEM_CALL` regset at C 157) |
| 167-173 | `tracing_processes` (170) | — |
| 174-242 | `read_memory` (C 186-207 the `process_vm_readv` fast path with the nested `!HAVE_PROCESS_VM` null test, C 209-240 the `PTRACE_PEEKTEXT` word loop and C 230's `assert`) | `have_sys_uio_h`, `not (have_process_vm)` |
| 243-316 | `user_alloca` (C 261-272 the `RED_ZONE_SIZE` block, C 265/268/276/283/305/307 the three `STACK_GROWS_DOWNWARDS` pairs, C 295/298 the `__aarch64__` pair, C 311 the `fail:` label) | `red_zone_size`, `no_stack_grows_downwards`, `aarch64` |
| 317-389 | `user_copy` (C 326-344 the `process_vm_writev` fast path with its nested null test) | `have_sys_uio_h`, `not (have_process_vm)` |
| 390-402 | `free_tracees` (401) | — |
| 403-430 | `remove_tracee` (C 434's `last = &(*last)->next` walk) | — |
| 431-440 | `static_tracees` (436) and `tracees` (439) | — |
| 441-457 | `find_tracee` (444-456) | — |
| 458-524 | `handle_clone_prepare` (C 478 the `PTRACE_GETEVENTMSG` read, C 491-492 the two bitfields, C 502-515 the three-way tracee allocation) | — |
| 525-619 | `handle_clone` (C 591-594 the `have_seccomp` option-bit block, C 598/608 the two `goto bail`, C 613 the `bail:` label) | `have_seccomp` |
| 620-627 | the "NOTICE" commentary (622-623) and `loader_name` (626) | — |
| 628-641 | `syscall_trap_p` (633-640) | — |
| 642-716 | `check_signal` (C 653 the switch, C 679-705 the `SIGSYS` arm with C 687-698's `HAVE_SIGINFO_T_SI_SYSCALL`/`!__arm__` pair) | `sigsys`, `have_siginfo_t_si_syscall`, `arm` |
| 717-840 | `handle_exec` (C 751 the `again1:` label, C 792-814 the `__aarch64__` pair, C 833 the `free_data_error:` label) | `aarch64` |
| 841-928 | `finish_exec` (C 868-872 the `__aarch64__` pair, C 889-905 the `STACK_GROWS_DOWNWARDS` pair with C 890's `not implemented, you lose.`, C 913-922 the `#if 0` debug block, C 924 the `error:` label) | `aarch64`, `no_stack_grows_downwards`, `#if 0` dropped |
| 929-956 | `rpl_stpcpy` (938-952) and C 954's `#define stpcpy rpl_stpcpy` | `any (not (have_stpcpy), not (have_decl_stpcpy))` |
| 957-1020 | `canon_path` (C 970 the `sizeof "/proc//fd/" + 48` array, C 1005-1013 the displacement arithmetic) | — |
| 1021-1114 | `handle_readlinkat` (C 1049-1058 the `READLINK_SYSCALL` pair, C 1039 the `sizeof "/proc//exe" + 24` array) | `readlink_syscall` |
| 1115-1217 | `handle_openat` (C 1139-1146 and C 1195-1199 the two `OPEN_SYSCALL` pairs, C 1202-1208 the `__aarch64__` pair, C 1214 the `fail:` label) | `open_syscall`, `aarch64` |
| 1218-1511 | `process_system_call` (C 1231-1236 the `__aarch64__` pair, C 1290/1314 the two shared-label pairs, C 1343-1349 the `__aarch64__` pair, C 1366/1383 the two `OPEN_SYSCALL` terms, C 1380-1388 the `HAVE_SECCOMP` guard, C 1416-1433 the `__aarch64__`/`__arm__` pair, C 1468-1504 and C 1491-1504 the `__mips__`/`__aarch64__` pairs, C 1426/1430/1431 the three labels) | `aarch64`, `readlink_syscall`, `open_syscall`, `have_seccomp`, `arm`, `mips` |
| 1512-1547 | the seccomp block's head: `#ifdef HAVE_SECCOMP`, `kernel_4_7_or_earlier` (1522), `interesting_syscalls[]` (1525-1537, with C 1531/1535's two conditional elements) and `ARRAYELTS` (1542) | `have_seccomp`, `open_syscall`, `readlink_syscall` |
| 1548-1611 | `establish_seccomp_filter` (C 1556-1596 the BPF program with C 1571/1581's conditional rules) | `have_seccomp`, `open_syscall`, `readlink_syscall` |
| 1612-1855 | `seccomp_system_call` (C 1634-1639 the `__aarch64__` pair, C 1685/1702 the two shared-label pairs, C 1761-1778 and C 1813-1849 the `__aarch64__`/`__arm__`/`__mips__` pairs) | `have_seccomp`, `aarch64`, `readlink_syscall`, `open_syscall`, `arm`, `mips` |
| 1856-1868 | the three `#ifndef` fallbacks (`PTRACE_EVENT_SECCOMP`, `PTRACE_O_TRACESECCOMP`, `SIGSYS`) and `#endif /* HAVE_SECCOMP */` | `have_seccomp`, their header arms' features |
| 1869-1896 | `tracing_execve` (1874-1895, with C 1888-1892's `HAVE_SECCOMP` block) | `have_seccomp` |
| 1897-2055 | `after_fork` (C 1909-1912 the `HAVE_SECCOMP && __ANDROID__` declarations, C 1918 the `again:` label, C 1937-1940 the `HAVE_SECCOMP` option bit, C 1952-2023 the Android-kernel probe with C 1982-1986's `__aarch64__` pair) | `have_seccomp`, `__android__`, `aarch64` |
| 2056-2209 | `exec_waitpid` (C 2095 the switch, C 2155-2161 the `HAVE_SECCOMP` label, C 2163-2189 the `SIGSYS` arm with C 2171-2182's pair) | `have_seccomp`, `sigsys`, `have_siginfo_t_si_syscall`, `arm` |
| 2210-2258 | `exec_init` (C 2218-2249 the `HAVE_SECCOMP` kernel-version probe, C 2250-2257 the two `dlsym` lookups) | `have_seccomp`, `all (have_sys_uio_h, not (have_process_vm))` |

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/trace-lane/cargo cargo run -q -p xtask -- build-tu exec/trace.c`
→ `built exec/trace.rs -> ../build/femacs/trace.o`, **exit 0**, and `build/femacs/trace.o` is
**336 bytes**, md5 `dbed68837694579185377dbb0821ab62` - the empty object.  That is the judgement
P-86 ⑧ uses for "the whole body, include graph included, sits behind the file gate": under the
frozen feature set `rustc` compiles `include/config.rs` and the `#![allow]` attribute and nothing
else.  It says nothing about the body, and the reference compiles nothing from this file either, so
there is no object-level comparison to make.

**② The arm flips (P-86 ③⑴, ⑥: the file gate *and* every sub-gate).**  Three readings, one per
group of hosts the C text configures; each is
`… -- probe exec/trace.rs <out> --crate-type=lib -C panic=abort <cfgs>`:

* **P1, the file gate alone** (`--cfg 'feature="have_android"'`): **exit 0, 0 errors, 7 warnings, a
  220,472-byte rlib**.  This is the arm set of a host with none of the sub-gates on, and it is the
  one that carries C 933-955's `rpl_stpcpy` arm, C 1864's `SIGSYS` fallback and every `#else`.
* **P2, the Android `aarch64` host's own set** (`have_android` + `have_sys_uio_h`, `aarch64`,
  `have_seccomp`, `sys_seccomp`, `ptrace_geteventmsg`, `ptrace_event_seccomp`,
  `ptrace_o_traceseccomp`, `sigsys`, `have_siginfo_t_si_syscall`, `have_stpcpy`,
  `have_decl_stpcpy`, `no_ndebug`): **exit 0, 0 errors, 12 warnings, a 268,936-byte rlib**.  This is
  the reading the NDK sysroots in the header make meaningful: the aarch64 register accessors, the
  seccomp accelerator, the `siginfo_t` arm and the live `assert` are all compiled here.
* **P3, the other hosts' arms** (`have_android` + the same set with `have_process_vm` instead of the
  `dlsym` fallback, plus `readlink_syscall`, `open_syscall`, `arm`, `mips`): **exit 0, 0 errors, 12
  warnings, a 265,416-byte rlib**.  This carries the `x86_64`/`i386`/`arm`/`mipsel` arms: the
  `READLINK_SYSCALL`/`OPEN_SYSCALL` labels and their shared bodies, `process_vm_readv`'s library
  entry, `PTRACE_SET_SYSCALL`/`arm` and `regs.gregs`/`mips`.

Every warning in all three readings is the same class, `unused doc comment` on an in-unit `extern`
block or on a `static` (rustdoc generates no documentation for those) - the class
`docs/ports/haiku.md` and `docs/ports/ntlib.md` record for the same shape; none is in the unit's
statements.  **Budget, reported honestly.**  The three readings above are the three the lane was asked for, but
they are not the three *invocations*: `probe` was run about twenty times in this session - the same
P1/P2/P3 commands re-run while the unit was being fixed (each re-run reports the same class of
errors until the fix lands), then twice for the mutation test of Evidence ③.  `build-tu` was run
once per fix pass (about five times).  Every one of those extra runs is the *same* three
configurations; the numbers quoted here are the final revision's, and the overrun is recorded rather
than hidden.

**③ The mutation test (P-86 ⑦: the reading above is not vacuous).**  With P1's flags, injecting
`n_words = c"trace-mutation".as_ptr ();` into `read_memory`'s own text (C 217's block) makes that
same command report **exactly one error**, `E0308 mismatched types` (`expected usize, found
*const c_char`), naming the injected line and nothing else (`aborting due to 1 previous error`).  The unit was then restored
byte-for-byte from a copy: `shasum -a 256` is
`afc01920c5b6ace16f211b1f60f346293b0690aa73b6bc2d5a73502de0fd9095` both before the injection and
after the restore, the injected token is gone (`grep -c trace-mutation` is 0), and the re-run P1
reading is 0 errors with the same 220,472-byte rlib.  So rustc really does
compile *this unit's* statements under `have_android`, rather than stopping in a mirror.

**④ `extract-abi.py --check` (P-86 ⑩: no longer vacuous, since the unit has `fn`s).**
`python3 tools/extract-abi.py --check emacs/exec/trace.c exec/trace.rs` → **one `- ` row**:
`line count drifted 37.6%: C 2258 vs Rust 3106`.  The twelve token rows the first run produced
(`user_alloca goto`; `handle_clone_prepare new_child,waiting_for_syscall`; `handle_clone
goto,new_child,waiting_for_syscall`; `check_signal break,case,default,switch`; `handle_exec
again1,goto`; `finish_exec goto,implemented,lose,not,you`; `canon_path 48`; `handle_readlinkat 24`;
`handle_openat 24,goto`; `process_system_call again1,case,continue_syscall,default,emulate_syscall,
goto,report_syscall_error,switch,waiting_for_syscall`; `after_fork again,goto,new_child,
waiting_for_syscall`; `exec_waitpid case,default,new_child,switch`) are now **recorded rows in
`docs/ports/macro-body-notes.tsv`** (appended once, at the end), each with its C line and its
reason - the P-86 ⑩ mechanism.  The remaining drift row is a **line-count** reading, not a text one:
the file's per-item `#[cfg (feature = "have_android")]` line, the in-unit system surface with a
citation per declaration, and 73 `#[cfg]`-with-two-arms pairs are what a 2,258-line C file becomes.
Its tolerance row is `## Owed` 3 (that file is not this lane's to write).

**⑤ The `#if` verdict.**  `python3 tools/check-cond-eval.py exec/trace.c` → `exec/trace.c: 73
conditions, 0 unknown, 0 disagreements [VERIFIED]`, and `exec/trace.c` is one of the files
`docs/baseline/config/cond-eval-verified.txt` lists (line 12), which is why `extract-abi.py`'s
inventory consults the macro table for it.

**⑥ The coverage and shape gates.**
`python3 tools/check-dead-file-coverage.py` → the whole-tree line plus **no row naming this ledger**
(the `## C line ranges` table above is what it reads, and it binds: 35 rows, ascending, covering
C 1-2258).
`python3 tools/check-mirror-dupes.py` → **no row names this unit**: its only mirror is
`exec/include/exec.rs`, which is consumed through `mod exec_decls`, and the mirror's four prototypes
plus the two `__aarch64__` accessors are shadowed by this unit's definitions (the P-21/P-28 shape
`lib-src/ntlib.rs` uses for the same reason).
`python3 tools/check-declarations.py` → **no row names this unit** (its own contribution is 0 in
every class; the tool's global count moves with the other lanes' in-flight files and is not quoted
here).
`python3 tools/check-ledger-ticks.py` → **no row names this ledger**: it has no ticked box (a P-86
ledger's checklist is empty, so the C range table is the coverage evidence, P-86 ②).
`python3 tools/check-citations.py` → **no drift row names this unit**: every citation in
`exec/trace.rs` is a bare `C <line>` (or a `/// C: <header>: …` line for the external
surface), which the tool counts as prose rather than a `path line` citation; the `exec/configure.ac`
rows it does read are counted and checked.

**⑦ The target readings (the NDK clang).**  The measurements behind the unit's constants and record
layouts, all with `~/Library/Android/sdk/ndk/29.0.14206865/toolchains/llvm/prebuilt/darwin-x86_64/bin/<target>-clang`:

* `-dM -E` over `<sys/ptrace.h>`, `<sys/user.h>`, `<linux/elf.h>`, `<linux/filter.h>`,
  `<linux/seccomp.h>`, `<sys/prctl.h>`, `<signal.h>`, `<errno.h>`, `<limits.h>`, `<fcntl.h>`,
  `<sys/wait.h>`, `<asm/unistd.h>` gives the constants the unit declares - `PTRACE_TRACEME` 0,
  `PTRACE_PEEKTEXT` 1, `PTRACE_PEEKDATA` 2, `PTRACE_POKETEXT` 4, `PTRACE_POKEDATA` 5, `PTRACE_CONT`
  7, `PTRACE_DETACH` 17, `PTRACE_SYSCALL` 24, `PTRACE_SETOPTIONS` 0x4200, `PTRACE_GETEVENTMSG`
  0x4201, `PTRACE_GETSIGINFO` 0x4202, `PTRACE_GETREGSET` 0x4204, `PTRACE_SETREGSET` 0x4205,
  `PTRACE_GETREGS`/`PTRACE_SETREGS` 12/13 (the x86 family only), `PTRACE_SET_SYSCALL` 23 (`arm`
  only), `PTRACE_EVENT_FORK`/`VFORK`/`CLONE`/`EXIT`/`SECCOMP` 1/2/3/6/7, `PTRACE_O_TRACESYSGOOD` 1,
  `NT_PRSTATUS` 1, `NT_ARM_SYSTEM_CALL` 0x404, `SIGTRAP` 5, `SIGSTOP` 19, `SIGSYS` 31, `SI_KERNEL`
  0x80, `__WALL` 0x40000000, `PATH_MAX` 4096, `SSIZE_MAX` `LONG_MAX`, `ENAMETOOLONG` 36, `ENOMEM`
  12, `EIO` 5, `EINTR` 4, `AT_FDCWD` -100, `__NR_execve` 221, `__NR_readlinkat` 78, `__NR_openat` 56
  (aarch64), `__NR_readlink` 89 and `__NR_open` 2 (x86-64), `SECCOMP_MODE_FILTER` 2,
  `SECCOMP_RET_ALLOW` 0x7fff0000, `SECCOMP_RET_TRACE` 0x7ff00000, `PR_SET_NO_NEW_PRIVS` 38,
  `PR_SET_SECCOMP` 22, `PR_GET_SECCOMP` 21, `SYS_SECCOMP` 1, `BPF_LD`/`W`/`ABS`/`JMP`/`JEQ`/`K`/`RET`
  = 0/0/0x20/5/0x10/0/6;
* `-Xclang -fdump-record-layouts-complete -fsyntax-only` over the same file gives `struct iovec`
  16 bytes (`iov_base` 0, `iov_len` 8), `struct sock_filter` 8 (`code` 0, `jt` 2, `jf` 3, `k` 4),
  `struct sock_fprog` 16 (`len` 0, `filter` 8), `struct utsname` 390 (six `char[65]`, `release` 130),
  `struct user_regs_struct` 272 (`regs[31]` 0, `sp` 248, `pc` 256, `pstate` 264), `struct
  seccomp_data` 64 (`nr` 0) and `siginfo_t` 128 (`si_code` 8, `si_syscall` 24);
* the three targets agree on every constant above except the two syscall numbers, which is exactly
  the split `exec/configure.ac`'s per-host arms make.

The unit's own `const _: () = assert!` rows (`siginfo_t`, `iovec`, `sock_filter`, `sock_fprog`,
`utsname`, `seccomp_data`) are those measurements, kept as assertions so a future divergence is a
compile error rather than a silent change; `USER_REGS_STRUCT`, whose member *names* are the host
macros, carries no offset assertion (its fields are the macro names, not the host record's - see
Deviation 3).

## Deviations

1. **The unit declares its own system surface.**  Every header C 22-34 and C 61-69 includes is the
   *target's* (a Linux/Android one) and reaches names whose values differ from the frozen target's:
   `lib/include/errno.rs` 69's `ENAMETOOLONG` is 63 and not 36, `lib/include/signal.rs` 25/28's
   `SIGSTOP`/`SIGSYS` are 17/12 and not 19/31, `lib/include/fcntl.in.rs`'s `AT_FDCWD` is not -100,
   `lib/include/limits.in.rs`'s `PATH_MAX` is the frozen target's own, and `lib/include/errno.rs`
   12's `errno` is `(*__error ())` and not `(*__errno_location ())`.  So the two `lib` mirrors are
   *not* in this unit's graph and each name is declared at the C `#include` line it comes from
   (P-88 ①), with the target reading and a `/// C: <header>: …` line; `<config.h>` alone is consumed
   as the port's mirror.  Every such record carries only the members this file reads.
2. **C's `NULL` is the typed null Rust's variadics need.**  `ptrace` is declared `unsafe extern "C"
   fn (c_uint, ...)`, so C's `NULL` third/fourth argument is written `std::ptr::null_mut::<c_void>
   ()` at each of the 24 call sites (a bare `null_mut ()` does not infer there).
3. **`USER_REGS_STRUCT` carries the *macro* names as member names.**  The C text reaches the host
   register record through the member designators `exec/configure.ac` defines (`regs->STACK_POINTER`,
   `regs.SYSCALL_ARG_REG`, `regs.SYSCALL_NUM_REG`), so the port's record is spelled with those names
   - one spelling every host's arm can share, because the C text names the same macro at the same
   place on all five.  `regs` and `gregs` are the two array members the `__aarch64__` and `__mips__`
   arms index literally (C 149, C 1469-1471, C 1492-1494).  The measured `aarch64`
   `struct user_regs_struct` layout is in Evidence ⑦ and a per-host table belongs in a mirror
   (`## Owed` 1).
4. **`RED_ZONE_SIZE` is defined by nothing.**  C 261's `#if RED_ZONE_SIZE` tests a name
   `exec/configure.ac` never defines (124's template and 255's `AC_DEFINE` are `ABI_RED_ZONE`), so
   that block is dead text in every configuration this file can be built in; the port carries both
   arms under `red_zone_size` and the negative arm is the live one.  This is the reference's own
   inconsistency, recorded here rather than repaired (the port's job is the text as it stands).
5. **C's `assert` is the *live* one in this translation unit.**  `src/conf_post.h` 28-29's
   `# define NDEBUG` is reached through `src/config.h`; this unit's `<config.h>` is
   `exec/config.h`, generated from `exec/config.h.in`, which has no `NDEBUG` line, and the `CFLAGS`
   the main configure passes to the sub-configure carry none either.  So both arms are carried under
   `no_ndebug` (the `src/sfnt.rs` name for the same test): the live arm checks its operand, the
   `NDEBUG` arm is C's `((void) 0)` and does not evaluate it.  All eleven `assert` sites are in the
   live arm's configuration.
6. **`goto` and its labels are labelled blocks or loops.**  PORTING.md 2.2's spelling: `user_alloca`'s
   `fail:` (C 311), `handle_clone`'s `bail:` (C 613), `handle_exec`'s `free_data_error:` (C 833),
   `finish_exec`'s `error:` (C 924), `handle_openat`'s `fail:` (C 1214) and `process_system_call`'s
   `report_syscall_error`/`emulate_syscall`/`common` (C 1426/1430/1431) are `'fail`/`'bail`/
   `'free_data_error`/`'error`/`'common` labelled blocks; the `again1:`/`again:` loops (C 751,
   C 1470, C 1787, C 1918) are `loop`s whose `continue` is C's backward jump; and the two switches
   whose `case` labels share one body (C 1290-1292, C 1314-1316, C 1675-1687, C 1702-1704) carry the
   body once per label inside cfg'd `if`s.  C 1396-1397's `default:`/`continue_syscall:` pair is the
   loop's tail, which is why the readlink arms *fall into* it instead of breaking.
7. **The empty statements become explicit tests.**  `else if ((tracee = malloc (sizeof *tracee))) ;`
   (C 513, C 561) is C's "assign and test in the condition, do nothing on success"; the port writes
   the assignment in the `else` arm and tests its result for null, which is the same control flow
   without an empty body.  Similarly C's chained `old_sp = sp = new_regs->STACK_POINTER;` (C 259) is
   two statements, Rust having no chained assignment expression.
8. **`#define stpcpy rpl_stpcpy` is `use rpl_stpcpy as stpcpy;`** - the object-like rename carried as
   Rust's rename import (the `lib/include/string.in.rs` 173-176 shape for the same gnulib selector),
   so the three call sites (C 979/988, C 1087, C 1171) keep C's own spelling.
9. **The `#if 0` block C 913-922 is dropped with a citation** - a debug block C's own preprocessor
   never compiles (the `src/w32term.rs` 685 precedent); it holds a label and a `waitpid`, so
   "carrying both arms" there would mean carrying text no configuration reaches.
10. **`interesting_syscalls` is a `const` array and `ARRAYELTS` is `.len ()`.**  C declares the table
    `static int interesting_syscalls[] = { … }` and sizes the BPF program as
    `1 + ARRAYELTS (interesting_syscalls) + 2`; a `static`'s length is not a constant expression in
    Rust, so the four readings of the `#ifdef`-built table are four `const` arrays (3/4/4/5
    elements) and `ARRAYELTS!` answers both the program array's size and the jump offsets, exactly
    as C's `sizeof` does.
11. **C's implicit conversions are spelled**: the `long`→`int` truncations of `rc` at the sites
    whose C `rc` is `int` (C 1231, C 1946, C 2028, C 1736, C 1385), `(void *) word` as
    `word as *mut c_void` (C 360, C 375), `(void *) address + i` as `(address + i) as *mut c_void`
    (C 221, C 235), `USER_WORD n` against `SSIZE_MAX`/`n` as `SSIZE_MAX as USER_WORD`/`n as ssize_t`
    (C 199, C 337), the `-1`/`-save_errno` writes into the register word (C 1446, C 1510, C 1827) as
    `-1isize as USER_WORD`/`(-save_errno) as USER_WORD`, and `(USER_SWORD) regs->SYSCALL_ARG_REG`
    (C 1059, C 1147) as `… as USER_SWORD as c_int`.
12. **C's uninitialised locals are given C's own zero values** (PORTING.md 0.1-13; the P-86
    `src/systhread.rs` finding's class): the out-parameters and first-write-later locals are named
    at their declarations in the unit, and the struct locals C fills field by field (`iov`,
    `remote`, `program`, `u`, `siginfo`, `regs`, `original`) are `std::mem::zeroed ()`.
13. **The two `ptrace` argument selections are duplicated per `#ifdef` arm** (C 1380-1388,
    C 1420-1422, C 1383-1385): a `#[cfg]` cannot sit on a `&&` term of an expression, so the
    `rc = ptrace (…)` statement carries one copy per reading of the `OPEN_SYSCALL` term and of the
    `HAVE_SECCOMP` guard, which is what C's preprocessor leaves at that site too.
14. **`ARRAYELTS`, `BPF_STMT`, `BPF_JUMP`, `MIN`, `offsetof` and `assert` are `macro_rules!`** with
    C's own names (PORTING.md 0.1-5's one-macro-one-home rule does not reach a header whose only
    consumer is this file); their call sites are `NAME! (…)`, which is the token change the
    12 rows of Evidence ④ record.

## Owed

1. **A per-host table for `USER_REGS_STRUCT` and the four `SYSCALL_*` member designators.**  The
   unit spells the `aarch64` reading of `regs[31]`/`sp`/`pc`/`pstate` and the aarch64 syscall
   numbers, and names the other hosts' readings in the citations; the mirror
   `exec/include/exec.rs` is where the table belongs (it already carries `USER_WORD` and
   `USER_HEADER` for the same reason).  The mipsel record (`exec/configure.ac` 390-400's `struct
   mipsel_regs`, `gregs[45]`) has **no reading on this machine** - the NDK dropped that ABI - and is
   the one member width in the unit that is named rather than measured.
2. **`READLINK_SYSCALL`/`OPEN_SYSCALL`'s values are the x86-64 ones.**  They are host-supplied
   (`exec/configure.ac` 269-370 defines them only on the hosts that have `readlink`/`open`), and the
   unit spells the measured x86-64 readings (89/2); the i386/arm (85/5) and mipsel (4090/4005+5)
   readings are **UNVERIFIED** and named.  A mirror that carries the per-host table would let the
   constant move out of this unit (the `## Owed` 1 class).
3. **`docs/ports/line-drift-tolerance.tsv` needs a row for `trace`**: C 2,258 vs Rust 3,106 is a
   **37.6%** drift - the per-item file gate, the in-unit system surface (each declaration with its
   `/// C:` line) and 73 two-arm `#[cfg]` pairs.  A row such as `trace 0.38 total` with that reason
   is what `tools/extract-abi.py --check` and `gate-fast` need; the file is not this lane's to write
   (the P-86 ⑩ precedent: the same class of row was written by the integrator for `ntlib` in
   3.504).  Without it `--check` reports the one row Evidence ④ reads.
4. **The new feature names need registering** (`xtask/Cargo.toml`'s `[features]` table and
   `docs/baseline/feature-coverage.tsv`, the mechanism `_timezone_defined` went through in 3.504):
   `have_sys_uio_h`, `have_process_vm`, `have_seccomp`, `have_siginfo_t_si_syscall`, `readlink_syscall`,
   `open_syscall`, `sigsys`, `sys_seccomp`, `ptrace_geteventmsg`, `ptrace_event_seccomp`,
   `ptrace_o_traceseccomp`, `red_zone_size`, `no_stack_grows_downwards`, `have_stpcpy`,
   `have_decl_stpcpy`, `no_ndebug`, `__android__`, `not_android_stubify` (the last two are already
   registered; `aarch64`, `arm`, `mips` and `have_android` are too).  Until they are,
   `docs/baseline/feature-coverage.tsv` lists sites that no registration covers (`tools/feature-coverage.py
   --write` is the tool).
5. **`docs/baseline/port-lines-pending.tsv`'s `exec/trace.c` row is now stale** and wants removing
   (the ratchet's own reading, `tools/check-port-lines.py`).  `docs/baseline/**` is not this lane's
   to write.
6. **The include graph's other two `exec/` units are still the P-08 note** (`exec/exec.rs`,
   `exec/include/exec.rs` is complete but `exec/exec.c`'s port is not), so `format_pid` and
   `exec_0` - the two symbols this file calls and does not define - have no definition anywhere in
   the ported tree yet.  That is their own lanes' work; this unit's link is not claimed.

## Not claimed

* **No run evidence of any kind.**  Nothing was linked, nothing was executed, and no tracee was ever
  traced: the file's whole job (ptrace-driven `execve` interception) is unexercised.  The reference
  compiles no object for this file on this target either, so there is not even an object-level
  comparison - the 336-byte object of Evidence ① is the empty one and says nothing about the body.
* **The arm flips are compile readings, not Linux builds.**  P1-P3 type-check the three arm sets
  under this machine's pinned `rustc`; they are not a `linux-android` build, nothing in them links,
  and the NDK readings of Evidence ⑦ measure *headers*, not behaviour.  A real Android/Linux
  toolchain is what a build of this unit needs, and `## Owed` 1's host table is what such a build
  would settle.
* **The mipsel arm is text-only.**  No toolchain here defines `__mips__`'s record; `regs.gregs[45]`,
  its two indexed members and the arm's `PTRACE_SET_SYSCALL 222` sibling are C's spelling with the
  source named, and P3 compiles them only as far as rustc can check a name.
* **The seccomp accelerator's numbers are measured, its semantics are not.**  `SECCOMP_*`, the BPF
  encodings, the three records' layouts and the three `prctl` requests are NDK readings; whether the
  filter this unit builds actually intercepts the intended calls is behaviour this round cannot
  read.
* **The line-count drift (Evidence ④) is a size reading, not text equality**, and its tolerance row
  is owed rather than argued; `extract-abi.py --check` still reports the one row until `## Owed` 3
  lands.
