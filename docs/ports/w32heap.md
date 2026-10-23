# emacs/src/w32heap.c (554 lines)

Port `src/w32heap.rs` (**888 lines**, whole file, C 1-554); ledger written to P-86's shape
(clause ②: the C line-range table replaces the empty checklist, plus `## Evidence`, `## Deviations`,
`## Owed`).  Reference `emacs/src/w32heap.c` md5 `2f105bcc301c18f873c7bd29f1a5437c`; the port's md5
is `100749b735779b672d9562b86d5e017e` (the 31-line stub it replaced was
`154d289353f88710f6c548ee66965ee4`).

## Decisions

- **The file gate is `feature = "windowsnt"`**, on every item *and on the include graph*, which is
  why the frozen configuration compiles nothing from this file.  The evidence is the build's own
  object list: `emacs/src/Makefile.in` 482 puts `w32heap.o` in `W32_OBJ` (`W32_OBJ=@W32_OBJ@`),
  filled by the non-Cygwin MinGW arm of `configure.ac` 3051's `HAVE_W32` split - the build whose
  `nt/inc/ms-w32.h` 25-27 `#define`s `WINDOWSNT`.  It is `WINDOWSNT`, not `HAVE_NTGUI`: `w32heap.o`
  is in `W32_OBJ` on both arms of the split (`src/w32fns.o`/`src/w32term.o` are the GUI pair), and
  the file's only dependence on the GUI half is `DebPrint`, which it reaches through
  `config.h`'s `conf_post.h` 160-166 - so the arm flip sets both features, the `src/w32proc.rs`
  reading.  `docs/baseline/frozen-build-sources.txt` lists no `w32heap.o` for this target.
- **This piece is `text-only`** (P-86 clause 3⑵): the machine has no Windows and no mingw toolchain
  (`which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty, no Homebrew
  `mingw-w64`, no `windows.h`/`winnt.h` anywhere under `find`), so there is **no arm flip on a
  Windows target and no layout measurement**.  What the round *can* show is that the `windowsnt` arm
  type-checks under this machine's `rustc` (see `## Evidence`), and that every error the flip prints
  is anchored in an include-graph file rather than in the unit.
- **The `<windows.h>` surface stands in the unit** (P-88 clause 1): the header has no file in the
  reference tree, and C 45 `#include "w32common.h"` is the file's first reach of it
  (`w32common.h` 24).  Each name carries the SDK's own text in a `/// C:` line; the spellings repeat
  `src/w32proc.rs`'s and `src/w32.rs`'s sections (`DWORD` = the 32-bit Windows unsigned, `HANDLE`/
  `HMODULE`/`LPVOID`/`PVOID` = `void *`, `BOOL` = C's `int`, `SIZE_T` = `usize`, `WINAPI` =
  `extern "system"`).  `DWORD_PTR` and the `ROUND_UP`/`get_page_size`/`get_allocation_unit` macros
  are `include/w32common.rs`'s and are not repeated.
- **Both `#ifndef MINGW_W64` arms are carried** (C 93-99 and C 119-134) as
  `not (feature = "mingw_w64")`; `tools/check-cond-eval.py src/w32heap.c` reads `2 conditions,
  0 unknown, 0 disagreements [VERIFIED]`, i.e. those two and no other conditional text in the file.
- **`include/w32heap.rs` is now `include!`d by this unit** (C 46).  The mirror's head note says
  "no unit `include!`s this mirror"; that sentence is stale as of this batch (the mirror's
  declarations are what this unit's `mmap_*`/`sbrk`/`init_heap` definitions collide with - Owed 2).

## C line ranges

The rows are contiguous and ascending and cover C 1-554 (blank lines included, so the gate has no
uncovered line).  `docs/port-map.md` 574-575 lists two targets, and the `src-` prefix rule
`check-dead-file-coverage.py` shares with `check-ledger-ticks.py` binds **both** to this ledger
(`src/w32heap.c`, 554 lines, and `src/w32heap.h`, 69 lines: there is no `src-w32heap.md`, and this
table starts at C 1 and ends past the header's last line, so both readings clear).  The header
mirror's own ledger is `docs/ports/include-w32heap.md`, which the port map's `include/` target row
reaches; that file is not this batch's to write.

| C | block | arm |
| --- | --- | --- |
| 1-17 | the file's licence/declaration block | dropped (P-86 clause 1 / PORTING.md 0.1-19) |
| 18-37 | C 19-25's two attribution comments and C 27-37's memory-allocation design comment | carried as C's own text |
| 38-49 | the include graph: `config.h` (C 39), `<stdio.h>`, `<errno.h>`, `<sys/mman.h>`, `<sys/resource.h>`, `w32common.h`, `w32heap.h`, `lisp.h`, `w32.h` | C 39 and its `conf_post.h` chain ungated; C 40-48 file gate |
| 50-64 | `data_region_base`, `data_region_end` (C 51-52), the private heap's `HANDLE` (C 58) and `the_malloc_fn`/`the_realloc_fn`/`the_free_fn` (C 61-63) | file gate |
| 65-83 | `heap_alloc` (C 65-72) and `heap_realloc` (C 74-83), both `static` | file gate |
| 84-99 | the file-mapping comment, `/* Heap creation.  */` and C 93-99's `#ifndef MINGW_W64` group (`HEAP_INFORMATION_CLASS`, `HeapSetInformation_Proc`) | file gate; C 93-99 `not (feature = "mingw_w64")` |
| 100-151 | `init_heap` - C 119-134's LFH arm inside it | file gate; C 119-134 `not (feature = "mingw_w64")` |
| 152-162 | C 153's form feed, the `malloc, realloc, free` banner, C 156-158's three `#undef`s and C 161-162's `FREEABLE_P` | file gate (`#undef`s carried as text; the macro as `macro_rules!`) |
| 163-182 | `malloc_after_dump` | file gate |
| 183-211 | `realloc_after_dump` | file gate |
| 212-223 | `free_after_dump` | file gate |
| 224-240 | the Windows 9X alignment comment and `malloc_after_dump_9x` | file gate |
| 241-273 | `realloc_after_dump_9x` | file gate |
| 274-282 | `free_after_dump_9x` | file gate |
| 283-299 | `sys_calloc` (C 284-292) and `getpagesize` (C 294-299) | file gate |
| 300-310 | `sbrk` | file gate |
| 311-316 | C 312's form feed, the `MMAP allocation for buffers` banner and `MAX_BUFFER_SIZE` (C 316) | file gate (constant) |
| 317-363 | `mmap_alloc` | file gate |
| 364-374 | `mmap_free` | file gate |
| 375-479 | `mmap_realloc` | file gate |
| 480-482 | C 481's form feed and C 482's `Emulation of getrlimit and setrlimit` banner | carried (form feed dropped) |
| 483-537 | `getrlimit` - C 489's `switch` with `RLIMIT_STACK`/`RLIMIT_NOFILE`/`default` | file gate (PORTING.md 2.2 `match`) |
| 538-554 | `setrlimit` - both `case` labels share one arm | file gate (PORTING.md 2.2 `match`) |

## Findings

1. **The arm's 33 errors are all in include-graph copies; the unit's own text is clean.**  The flip
   reports `E0428`/`E0425` in `lib/include/signal.rs` (13), `src/include/w32heap.rs` (5),
   `nt/inc/include/ms-w32.rs` (4), `src/include/w32.rs` (4), `nt/inc/sys/include/resource.rs` (3),
   `lib/include/errno.rs` (2) and `src/include/w32common.rs` (2), with **0 in `_root.rs`** - the
   by-file reading P-86 ⑦ asks for, and the same file set (and the same classes)
   `docs/ports/w32proc.md` Evidence ② recorded for `src/w32proc.rs`.  **On this file that reading is
   empty on its own** (Finding 9): the graph's errors stop the check before the unit's bodies, so the
   reading that bears on the text is Finding 10's pre-flight.
2. **`src/include/w32heap.rs`'s five declarations are the one class this batch *creates*.**  The
   mirror declares `mmap_realloc`/`mmap_free`/`mmap_alloc`/`sbrk`/`init_heap` inside a top-level
   `unsafe extern "C" { … }`, i.e. textually in this unit's crate root, while C 46 is the mirror's
   `#include`; a unit that *defines* them is `E0428` (rustc 1.98.1 anchors the error in the mirror
   and the "previous definition" note in the unit).  The remedy is PORTING.md P-39's shape - the
   declarations into a glob-imported `pub mod` + `use` (the `ms-w32.rs` `ms_w32_decls` and
   `include/w32.rs` `w32_fn_decls` precedents), which is the mirror round's job (Owed 2).
3. **`nt/inc/sys/include/resource.rs` has the same shape plus one missing import**: 2 × `E0428`
   (`getrlimit`, `setrlimit`, which this file defines) and `E0425: cannot find type c_longlong`
   (`rlim_t`'s spelling needs the `use core::ffi::*` the other mirrors carry).
4. **`src/include/w32common.rs`'s two deferrals are the path this file takes**: `SYSTEM_INFO` and
   `OSVERSIONINFO` are declared by that mirror only as names (`pub static mut sysinfo_cache:
   SYSTEM_INFO;` under `windowsnt`), so C 298's `sysinfo_cache.dwPageSize` and C 332/338's
   `get_allocation_unit ()` are `E0425` in the mirror, not here.
5. **C's `size | !size` (C 68, C 78) is the *logical* not.**  `!` in C is `(size == 0) ? 1 : 0`, not
   a bitwise complement; the port spells the operand `(size == 0) as size_t` so the `HeapAlloc`/
   `HeapReAlloc` byte count keeps C's value (1 for a zero size, the size itself otherwise).
6. **`ROUND_UP` (C 332/338) relies on an implicit promotion the port must spell.**
   `include/w32common.rs` 45-49's macro substitutes `$align` uncast in its first term
   (`… + $align - 1`), where C's own macro relies on the usual arithmetic conversions (a `DWORD`
   `get_allocation_unit ()` widens to `DWORD_PTR`); the port passes
   `get_allocation_unit! () as DWORD_PTR`, which is C's conversion written out.  Without it the
   `usize + u32` sum is an `E0308` in the unit's own text.
7. **`MEMORY_BASIC_INFORMATION` carries all seven members, not only the five C's text names.**  C
   393/400/502 pass `sizeof` the record as `VirtualQuery`'s buffer length, so a record shortened to
   the read members would move every field the file reads; the widths are the documented ones and
   are **UNVERIFIED** (no `<windows.h>` here), and the assertion is owed with the rest of the SDK
   surface.
8. **Two citations the checker could not resolve were rephrased, not dropped**: `lisp.h 30` with a
   trailing `` `memset` `` read as a citation of `memset` at `lisp.h:30` (`check-citations` drift);
   the line now names the header without the number.  `check-citations` is back to `0 drift`.
9. **The arm flip is vacuous on this file, and the mutation test is what says so** (P-86 ⑦).  With
   a type error injected into `mmap_alloc`'s body (`let _mutation_probe: c_int = p;`) the flip's
   output is **byte-identical** to the clean run (`diff` of the two logs' `error`/`-->` lines is
   empty): the include graph breaks at name resolution, so the unit's bodies are never checked and
   "0 errors in `_root.rs`" means nothing on its own.  The substitute reading is Finding 10.
10. **The arm-reach pre-flight (the same clause's fallback) is live, and it caught four real defects
   in this lane's own first draft**: (a) `VirtualFree`'s `LPVOID` argument needed the `char *`->
   `LPVOID` conversion C leaves implicit (C 470); (b) and (c) C 506-509's `DWORD_PTR`-typed
   arithmetic needed the `as rlim_t` conversion C leaves implicit (twice); (d) `#[link_name =
   "memcpy"]`/`"memmove"` is refused by rustc ("invalid definition of the runtime `memcpy` symbol
   used by the standard library"), so the two renames are `macro_rules!`s over the CRT entries the
   rename chain ends at (Deviations 12) and their call sites carry the `!` PORTING.md's macro rule
   requires (Deviations 13).  All four were fixed before the readings below, and the same mutation
   the flip ignores makes the pre-flight print `E0308` naming `_mutation_probe`.

## Evidence

- **frozen `build-tu`** (`cd femacs && CARGO_TARGET_DIR=/tmp/w32heap-lane cargo run -q -p xtask --
  build-tu src/w32heap.c`) -> `built src/w32heap.rs -> ../build/femacs/w32heap.o`, **exit 0,
  336 bytes** - the same empty object the pre-port stub produced (every item is
  `#[cfg (feature = "windowsnt")]`, so the frozen compile sees only `config.rs`/`conf_post.rs`,
  exactly the `src/w32proc.rs` reading).
- **arm flip** (`cd femacs && CARGO_TARGET_DIR=/tmp/w32heap-lane cargo run -q -p xtask -- probe
  src/w32heap.rs /tmp/w32heap-lane/w32heap_arm2.o --crate-type=lib --emit=obj
  --cfg 'feature="windowsnt"' --cfg 'feature="have_ntgui"'`) -> **exit 1, 33 errors, 0 of them
  anchored in the unit, 0 warnings printed**.  By file: `lib/include/signal.rs` 13 (`SIGALRM`,
  `SIGCHLD`, `SIGHUP`, `SIGKILL`, `SIGPIPE`, `SIGPROF`, `SIGQUIT`, `SIGTRAP`, `NSIG`, `SIG_BLOCK`,
  `SIG_UNBLOCK`, `SIG_SETMASK`, `sigaction`: the frozen `<signal.h>` versus the MinGW one a
  `windowsnt` unit also pulls through `include/lisp.rs` -> `include/thread.rs`),
  `src/include/w32heap.rs` 5 (`mmap_realloc`, `mmap_free`, `mmap_alloc`, `sbrk`, `init_heap` -
  Finding 2), `nt/inc/include/ms-w32.rs` 4 (`HAVE___BUILTIN_UNWIND_INIT` against `config.rs`;
  `ENOTSUP`/`EOPNOTSUPP` against `errno.rs`; `_MAX_PATH`; `pid_t`/`mode_t`), `src/include/w32.rs` 4
  (`mode_t` x3, `pid_t`), `nt/inc/sys/include/resource.rs` 3 (Finding 3), `lib/include/errno.rs` 2
  (`ENOTSUP`, `EOPNOTSUPP`), `src/include/w32common.rs` 2 (Finding 4).  **This reading is vacuous
  on its own** (Finding 9); it is *not* a Windows compile either.
- **arm-reach pre-flight** (P-86 ⑦'s fallback, Finding 10): the unit's own text with its `include!`
  edges (and the `#[cfg]` above each) dropped and the `#![allow]` moved to the head, preceded by a
  stub prelude carrying exactly the names the graph supplies - `size_t`, `DWORD_PTR`, `intptr_t`,
  `PTRDIFF_MAX`, `malloc_fn`/`realloc_fn`/`free_fn`, `rlim_t`/`rlimit_resource_t`/`RLIMIT_STACK`/
  `RLIMIT_NOFILE`/`rlimit`, `ENOMEM`/`EPERM`/`ENOSYS`/`EINVAL`, `FD_SETSIZE`, `OS_SUBTYPE_9X`,
  `VOIDFNPTR`, a `SYSTEM_INFO_ROW` behind `sysinfo_cache`/`os_subtype`, `memset`/`memcpy`/`memmove`,
  `get_proc_addr`, `cache_system_info`, `errno`, and the five macros `eassert!`, `DebPrint!`,
  `ROUND_UP!`, `get_page_size!`, `get_allocation_unit!` (`/tmp/w32heap-lane/preflight.rs`,
  915 lines; removed at closeout with the rest of that tree) -> `rustc --edition 2024
  --crate-type=lib --emit=obj --cfg 'feature="windowsnt"'` -> **exit 0, a 33,920-byte object,
  1 warning** (`unused doc comment` on the in-unit `extern` block - the house warning class).
  Live: the same injected type error yields `E0308` there.  It is *not* the flip - the stubs are
  this lane's, so it says the unit's text is well-typed against the names its graph supplies, not
  that the SDK's prototypes or the record layouts are right.
- **`python3 tools/extract-abi.py --check emacs/src/w32heap.c src/w32heap.rs`** (non-vacuous
  once the port has `fn`s, P-86 ⑩) -> 3 rows, all named: `- function getrlimit does not mention
  break, case, default, switch`, `- function setrlimit does not mention …` (the two `match`es of
  PORTING.md 2.2) and `- line count drifted 60.3%: C 554 vs Rust 888`, then `FAIL`.  No
  `missing`/`does not mention` row names any other item: the whole file's functions, macros
  (`FREEABLE_P`, `MAX_BUFFER_SIZE`), types (`HEAP_INFORMATION_CLASS`, `HeapSetInformation_Proc`,
  `MEMORY_BASIC_INFORMATION`) and file-scope objects are accounted for.  The two token rows and the
  tolerance row are Owed 1 (neither file is this lane's).
- **`python3 tools/check-cond-eval.py src/w32heap.c`** -> `2 conditions, 0 unknown, 0 disagreements
  [VERIFIED]`.
- **`python3 tools/check-mirror-dupes.py`** -> `17 duplicate declaration(s), 17 ratchet row(s),
  0 failure(s)` - **no row names this unit**.
- **`python3 tools/check-declarations.py`** -> `10 declaration sites (0 forbidden, 10 allowed
  literals), 0 failures` (unchanged; the unit's first line is the `// femacs: port of …` form and no
  comment in it names the reference program).
- **`python3 tools/check-citations.py`** -> `676 file(s) scanned, 0 drift, 0 range, 1044 external,
  35 allowed, 0 stale` (the one drift this batch introduced was fixed - Finding 8).
- **`python3 tools/check-dead-file-coverage.py`** -> `73 ledger(s) bound (a C range table),
  0 failure(s)`; this ledger's row is `info docs/ports/w32heap.md: 23 range row(s) cover
  src/w32heap.c (554 lines; 0 blank line(s) between blocks need no row)`.
- **`python3 tools/check-port-lines.py`** -> it read `51 ratchet row(s), 0 new, 2 stale` with this
  batch's row among them (`stale src/w32heap.c: no longer under the threshold` - the stub's 31
  lines are gone) during the batch, and `49 ratchet row(s), 0 new, 0 stale` at closeout: a
  concurrent `--write-pending` run rewrote `docs/baseline/port-lines-pending.tsv` (mtime 14:01:29,
  not this lane's action - this lane ran no writer) and cleared both stale rows.  The stem's
  ratchet row is therefore cleared by someone else's run, which is what P-86 ④ asks for.

## Deviations

1. **The `<windows.h>` section stands in the unit** (P-88 clause 1) at C 45's `#include` line, and
   every record/scalar in it is **UNVERIFIED** - no SDK header on this machine (the acquisition
   note).  The two function-like macros C 200/259/270/431/455 invoke (`CopyMemory`/`MoveMemory`)
   are `macro_rules!`s resolving winnt.h's and MinGW-w64's rename chain to the CRT entries
   (Deviations 12); the `#[link_name]` spelling `src/charset.rs` 116 uses for a *non-intrinsic*
   symbol does not work here (Finding 10d).
2. **`MEMORY_BASIC_INFORMATION`'s layout is not asserted** and its field widths are the documented
   ones (Finding 7).
3. **C's implicit conversions are spelled** where Rust has no promotion: `size <= PTRDIFF_MAX`
   (C 68/77) -> `size <= PTRDIFF_MAX as size_t`; `nbytes < MAX_BUFFER_SIZE` (C 331) ->
   `as size_t`; `nbytes + getpagesize ()` (C 444) and the two `get_page_size ()` terms (C 470-471)
   -> `as size_t`; `sysinfo_cache.dwPageSize` -> `as c_int` (C 298); `DWORD` -> `DWORD_PTR` in
   `ROUND_UP` (Finding 6); `(uint64_t)` casts (C 415-419) -> `as u64`.
4. **The two `switch`es (C 489, C 542) are `match`es** (PORTING.md 2.2) with `_` for `default:`;
   `setrlimit`'s two `case` labels are one arm (`RLIMIT_STACK | RLIMIT_NOFILE`).  The
   `RLIMIT_*` labels are `nt/inc/sys/include/resource.rs`'s constants and the `switch`/`case`/
   `default`/`break` tokens are the two `does not mention` rows of `## Evidence` (Owed 1).
5. **C's `switch` keywords are not the only token substitution**: `enable_lfh` is declared inside a
   `#[cfg (not (feature = "mingw_w64"))] { … }` block so C 119's arm keeps C's scope; C 123's
   function-pointer cast is a `core::mem::transmute` (a Rust `as` cannot spell that pair -
   `src/w32proc.rs` 620-625's reading); C 128's `(PVOID) heap` is kept as `heap as PVOID`.
6. **C's uninitialized locals get C's own zero values** (PORTING.md 0.1-13/1.5): `p`/`ptr`/`pa`/
   `retval`/`old_ptr`/`new_brk` and the three `MEMORY_BASIC_INFORMATION`/`rlimit` loci
   (`core::mem::zeroed ()` for the records).  `realloc_after_dump`'s `p` is the one left
   uninitialized-but-assigned-in-every-branch, written as C's two-branch assignment.
7. **`(*the_malloc_fn) (nbytes)` (C 288) is `.unwrap () (…)`**: the CRT's function-pointer object is
   a nullable pointer and the port's `Option<extern fn>` is that pointer, so a call before
   `init_heap` panics in the port where C would fault - the `src/w32proc.rs` reading of the same
   indirection.  The three objects themselves are `#[unsafe(no_mangle)] pub static mut … = None`
   (C's zero-initialized definitions, PORTING.md 0.2).
8. **`#undef malloc`/`realloc`/`free` (C 156-158) are carried as C's text** in a comment: the
   `nt/inc/ms-w32.h` 443-445 renames are `macro_rules!`s in the mirror, which fire only on the
   `name! (…)` form, so the port's plain calls reach the entries C's `#undef` selects by
   construction (the `docs/ports/w32proc.md` Deviations 10 reading).
9. **`FREEABLE_P` is a `macro_rules!`** whose body is `($addr) as *mut u8 as DWORD_PTR > 0` - C's
   `(DWORD_PTR)(unsigned char *)(addr) > 0` with the cast spelled; `#define MAX_BUFFER_SIZE` is
   `pub const MAX_BUFFER_SIZE: c_int`.  `HEAP_INFORMATION_CLASS` is PORTING.md 1.3's C-enum shape
   (`pub type … = c_int` plus one constant per member), which keeps C 128's bare member name.
10. **C's form feeds (C 153, 312, 481) and the two comments around them are recorded in the range
    table rather than carried** (FORM FEED has no Rust counterpart; the banners are kept as
    comments at their positions).
11. **The mirror of C 46 is `include!`d although its own note says no unit includes it**
    (Decisions, last bullet) - the note is stale and the mirror round owns it (Owed 2).
12. **`CopyMemory`/`MoveMemory` are `macro_rules!`s over `memcpy`/`memmove`**
    (`lib/include/string.rs` 20/22, already in this unit's graph): the C source names are kept and
    the rename chain (`CopyMemory` -> `RtlCopyMemory` -> the CRT entry) is carried as the macro
    body, the `nt/inc/include/ms-w32.rs` 275-278 shape.  **UNVERIFIED**: the chain's end is only as
    good as the MinGW header, which is not on this machine.
13. **C's function-like macros carry the `!` at their call sites** (PORTING.md's macro rule):
    `CopyMemory!`, `MoveMemory!`, `FREEABLE_P!`, `ROUND_UP!`, `get_page_size!`,
    `get_allocation_unit!` - the `/// C:` lines keep C's own text, and C's implicit
    `char *`->`LPVOID` and `DWORD_PTR`->`rlim_t` conversions are spelled (`as LPVOID`,
    `as rlim_t`; Finding 10a-c).

## Owed

1. **`docs/ports/macro-body-notes.tsv`**: two rows, `getrlimit` and `setrlimit`
   (`switch,case,default,break`: C's `switch` is the port's `match`, PORTING.md 2.2 - the
   `src/w32proc.rs` Owed 4 shape).  **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem
   `w32heap`, tolerance ≥ 0.603, basis `total`, reason from Deviations 3/5/9 (the spelled
   conversions, the `<windows.h>` stand-in section and the per-item `/// C:` citations).  Neither
   file is in this lane's write set; until they land `extract-abi.py --check` stays `FAIL` with the
   three rows `## Evidence` records.
2. **The mirror round's reconciliation of the `windowsnt` arm** (Findings 2-4): (a)
   `src/include/w32heap.rs`'s five declarations (`mmap_realloc`/`mmap_free`/`mmap_alloc`/`sbrk`/
   `init_heap`) belong in a glob-imported `pub mod` (P-39's shape; its head note's "no unit
   `include!`s this mirror" is stale too); (b) `nt/inc/sys/include/resource.rs`'s two prototypes
   need the same treatment and its `rlim_t` row needs the `use core::ffi::*` its siblings carry;
   (c) `src/include/w32common.rs`'s `SYSTEM_INFO`/`OSVERSIONINFO` deferral is what C 298/C 332/C 338
   reach, so a Windows target must land those two records (or the file's `sysinfo_cache` reads stay
   unbuildable) - `include/gtk.rs` `GtkWidgetClass`/`include/w32heap.rs` `file_data` is the shape;
   (d) the cross-mirror `E0428` set (`signal.rs` vs `nt/inc/include/ms-w32.rs`, `errno.rs` vs
   `ENOTSUP`/`EOPNOTSUPP`, `mode_t`/`pid_t` absent from `src/include/w32.rs`'s graph) is the same
   class `docs/ports/w32proc.md` Owed 5 names and its resolution unblocks this unit's flip too.
3. **A layout assertion for `MEMORY_BASIC_INFORMATION`** (and for the SDK scalars): needs a
   Windows/mingw clang to measure `sizeof`/`offsetof`; until then the record stays `UNVERIFIED`
   (Deviations 1-2) and belongs in `tools/gen-layout-fields.py`'s input for that target.
4. **A real Windows-target acceptance**: the arm flip above is this machine's `rustc` type-check of
   the arm; installing/compiling on a mingw toolchain (P-86 ③⑴) is what would move `text-only` to a
   measured reading, and it is also what would let `HeapAlloc`/`VirtualAlloc`/`HeapSetInformation`
   be checked against the SDK's prototypes.
5. **`tools/check-port-lines.py --write-pending`** (P-86 ④) for the stem `w32heap` - **observed
   cleared during the batch** (Evidence, last bullet): `docs/baseline/port-lines-pending.tsv` was
   rewritten at 14:01:29 by a concurrent run, and the closeout reading is `0 new, 0 stale`.  If that
   run is not the one the integrator counts, the row to (re)write is `docs/port-map.md`'s /
   `docs/baseline/port-lines-pending.tsv`'s for the stem `w32heap` (31 -> 888 lines) - neither file
   is this lane's.

## Not claimed

- **The frozen configuration compiles nothing here** and the 336-byte object is the empty object;
  the port is not "built" by any reading of this tree.
- **The arm flip is a type-check under this machine's `rustc`, not a Windows compile**: the 33
  errors it prints are include-graph defects (none in this unit's text), and a green flip would
  still be no evidence about the SDK's prototypes, the `#[link_name]` symbols or the record layout.
- **No link, no run, no runtime behaviour**: the allocator switch, the LFH request, the mapping
  arithmetic and the two `getrlimit`/`setrlimit` emulations are transcribed, not exercised;
  `docs/baseline/frozen-build-sources.txt` lists no `w32heap.o` and no Windows binary was produced.
- **Every record/scalar the `<windows.h>` section declares is UNVERIFIED** (widths documented, not
  measured); no offset or `sizeof` assertion exists for them, and the `ERROR_NOT_ENOUGH_MEMORY`
  value and the `CopyMemory`/`MoveMemory` symbols are the documented ones, not quotations.
- **Nothing about the reference's behaviour is claimed from this ledger**: the C line-range table,
  the `#if` verdict and the check readings are the whole evidence, and the two `does not mention`
  rows plus the line-drift row mean `extract-abi.py --check` still exits `FAIL`.
- **The arm flip's "0 errors in the unit" is not evidence about this unit** (Finding 9: the
  mutation test leaves the output byte-identical).  The reading that does bear on the text is the
  arm-reach pre-flight, and it is *this lane's* stub set: a green pre-flight says the unit's text
  type-checks against the names its graph supplies, not that those names are the SDK's.
- **Nothing about `MEMORY_BASIC_INFORMATION`'s offsets, the `ERROR_NOT_ENOUGH_MEMORY` value or the
  `CopyMemory`/`MoveMemory` rename chain's end is quoted from a header**: those three rows are the
  documented values, and the ledger asks the next Windows round to measure them (Owed 3/4).
