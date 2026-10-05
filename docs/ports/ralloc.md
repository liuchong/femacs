# emacs/src/ralloc.c (1214 lines)

Port `femacs/src/ralloc.rs` (**1570 lines**, whole file, C 1-1214); ledger written to P-86's shape
(clause ②: the C line-range table replaces the empty checklist, plus `## Evidence`, `## Deviations`,
`## Owed`).  Reference `emacs/src/ralloc.c` md5 `247c259ce1c90592d18b019819e45117`; the port's md5
`cad9b3bcf14d08260c95ffeb062ef0e6` (`md5 -q`, final revision; the 17-line empty unit it replaced was
`fbc3fd62a121094213cb13f0abd16b05`, i.e. the revision at `HEAD`).

## Decisions

- **The file gate is `all (feature = "rel_alloc", feature = "no_system_malloc")`**, on every item
  *and on C 27-33's include graph*, which is why the frozen configuration compiles nothing from this
  file.  C's condition is the build's, not an `#if`: `emacs/src/Makefile.in` 495 has
  `RALLOC_OBJ=@RALLOC_OBJ@`, and `configure.ac` 7320-7326 fills it only inside
  `if test "${REL_ALLOC}" = "yes"` and only when `test "$system_malloc" != "yes"` (`REL_ALLOC`
  itself is decided at `configure.ac` 3355-3368).  The frozen build's generated `config.h` leaves
  `REL_ALLOC` undefined and defines `SYSTEM_MALLOC` (1), so `docs/baseline/frozen-build-sources.txt`
  lists no `ralloc.o` for this target.  Both features are already registered
  (`femacs/xtask/Cargo.toml` 586 `rel_alloc`, 534 `no_system_malloc`; `msdos.rs` 646-676 is their
  earlier consumer), and the two together are C's `REL_ALLOC && !SYSTEM_MALLOC` - one feature alone
  would compile text C never compiles.
- **This piece is *not* `text-only`**: the arm flip type-checks on this machine (one rustc, no
  second toolchain), so `## Evidence` ② is a measurement rather than a text-only record.  What it
  is *not* is a C compile or a link: no `rel_alloc` configuration exists here, the host's
  `<malloc.h>` does not exist (macOS), and nothing links `ralloc.o` (see `## Not claimed`).
- **Both arms of every `#if` are carried**, in C's order: C 80 `#ifdef HAVE_MALLOC_H`
  (`have_malloc_h`), C 83 `#ifndef DOUG_LEA_MALLOC` (`not (doug_lea_malloc)`), C 995 `#ifdef
  DOUG_LEA_MALLOC`, C 1013 `#ifdef DEBUG` (`debug`), and the three `#if !defined SYSTEM_MALLOC`
  (C 1161, 1180, 1193 - `no_system_malloc`).  The two `__morecore` declarations sit on opposite
  sides of `doug_lea_malloc` (C 84 declares it only when the macro is undefined; C 1003 reaches
  `<malloc.h>`'s when it is defined), so the crate root never carries both.
- **`M_TOP_PAD` and `mallopt` stand in the unit at C 80-82** (P-88 ①: the declaration site is C's
  own `#include` line, and no file is added under `femacs/src/include/`).  They are glibc's, and
  this machine has no `<malloc.h>` to measure them against, so the value is **UNVERIFIED**.

## Gate mapping

| C `#if` / build condition | Port gate | Frozen value |
| --- | --- | --- |
| the file itself (`RALLOC_OBJ`, i.e. `REL_ALLOC && !SYSTEM_MALLOC`) | `all (feature = "rel_alloc", feature = "no_system_malloc")` (every item and the include graph) | false (`REL_ALLOC` undefined, `SYSTEM_MALLOC` 1) |
| C 80 `#ifdef HAVE_MALLOC_H` | `feature = "have_malloc_h"` (nested) | false |
| C 83 `#ifndef DOUG_LEA_MALLOC` (the C 84 `__morecore` declaration) | `not (feature = "doug_lea_malloc")` | true |
| C 995 `#ifdef DOUG_LEA_MALLOC` (C 999-1010) | `feature = "doug_lea_malloc"` | false |
| C 1013 `#ifdef DEBUG` (C 1017-1103) | `feature = "debug"` | false |
| C 1161 `#if !defined SYSTEM_MALLOC` (C 1162-1186) | `feature = "no_system_malloc"` | true under the file gate |
| C 1175 `#ifdef DOUG_LEA_MALLOC` / C 1179 `#else` (C 1180-1191) | `feature = "doug_lea_malloc"` / `not (feature = "doug_lea_malloc")` | false / true |
| C 1193 `#if !defined SYSTEM_MALLOC` (C 1194-1211) | `feature = "no_system_malloc"` | true under the file gate |

## C line ranges

The rows are contiguous and ascending and cover C 1-1214, so the gate has no uncovered line.

| C | block | arm |
| --- | --- | --- |
| 1-18 | the file's opening declaration block (C 1-17) and its blank line | dropped (P-86 ① / PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 19-26 | C 19-23's NOTES comment, C 25 `#include <config.h>` | carried at its position; `config.rs` ungated (every unit includes it) |
| 27-34 | C 27 `<stddef.h>`, C 29 `"lisp.h"`, C 30 `"blockinput.h"`, C 31 `<unistd.h>`, C 33 `"getpagesize.h"` | file gate (P-86 ⑧); `getpagesize` comes from the `<unistd.h>` mirror, because `getpagesize.h`'s live text is `#ifndef HAVE_GETPAGESIZE` |
| 35-46 | C 35-41's `r_alloc_initialized` comment, the object (C 43), C 45's forward declaration of `r_alloc_init` | file gate; C 45 is a comment (Rust needs no forward declaration) |
| 47-66 | C 47's form feed, C 48-51 `real_morecore`, C 53-54 `virtual_break_value`, C 56-58 `break_value`, C 60-61 `page_size`, C 63-65 `extra_bytes` | file gate |
| 67-75 | C 67-68's comment, `PAGE` (C 69), `PAGE_ROUNDUP` (C 70-71), `MEM_ALIGN` (C 73), `MEM_ROUNDUP` (C 74-75) | file gate, all four as `macro_rules!` |
| 76-87 | C 77-78's comment, C 80-82 `<malloc.h>` arm, C 83-85 `__morecore` arm, C 87's form feed | C 80-82 `have_malloc_h` (with the `<malloc.h>` stand-in section); C 83-85 `not (doug_lea_malloc)` |
| 88-107 | C 88's form feed, the `Implementation using sbrk` banner (C 89-91) and C 93-106's heap/bloc design comment | carried as C's own text (inside the file gate) |
| 108-135 | `struct heap` + `heap_ptr` (C 108-124), `NIL_HEAP` (C 126), `heap_base` (C 131), `first_heap`/`last_heap` (C 134) | file gate |
| 136-167 | C 136-144's bloc comment, `struct bp` + `bloc_ptr` (C 146-155), `NIL_BLOC` (C 157), `BLOC_PTR_SIZE` (C 158), `first_bloc`/`last_bloc` (C 161), `use_relocatable_buffers` (C 163), `r_alloc_freeze_level` (C 166) | file gate |
| 168-183 | C 168's form feed, C 169-171's comments, `find_heap` (C 173-183) | file gate; the infinite loop is `loop` (its body is the function's tail) |
| 184-277 | C 185-201's comment, `obtain` (C 203-277) | file gate; C 244's `new_heap + 1` is `.offset (1)` |
| 278-339 | C 279-282's comment, `relinquish` (C 284-339) | file gate |
| 340-365 | C 340's form feed, C 341-344's comments, `find_bloc` (C 346-365) | file gate |
| 366-416 | C 367-369's comment, `get_bloc` (C 371-416) | file gate |
| 417-480 | C 417's form feed, C 418-424's comment, `relocate_blocs` (C 426-480) | file gate; `heap`/`address` are rebound (`let mut`) |
| 481-540 | C 481's form feed, C 482-483's comment, `update_heap_bloc_correspondence` (C 485-540) | file gate |
| 541-640 | C 541's form feed, C 542-543's comment, `resize_bloc` (C 545-640) | file gate |
| 641-715 | C 641's form feed, C 642-643's comment, `free_bloc` (C 645-715) | file gate |
| 716-869 | C 716's form feed, C 717's `Interface routines` comment, C 719-728's comment, `r_alloc_sbrk` (C 730-869) | file gate; `unsafe extern "C" fn` (its address is C 1163's hook value) |
| 870-901 | C 872-881's comment, `r_alloc` (C 883-901) | file gate, exported |
| 902-922 | C 903-904's comment, `r_alloc_free` (C 906-922) | file gate, exported; the `refill_memory_reserve` declaration sits at C 934's use site |
| 923-992 | C 924-936's comment, `r_re_alloc` (C 938-992) | file gate, exported |
| 993-1011 | C 995's `#ifdef DOUG_LEA_MALLOC`, C 997-998's comment, `r_alloc_reinit` (C 999-1010), C 1011's `#endif` | `doug_lea_malloc` |
| 1012-1105 | C 1013's `#ifdef DEBUG`, C 1015's `#include <assert.h>`, `r_alloc_check` (C 1017-1103), C 1105's `#endif`; C 1037-1041 is an `#if 0` region | `debug`; the `#if 0` text is recorded, not transcribed |
| 1106-1133 | C 1107-1111's comment, `r_alloc_reset_variable` (C 1112-1133) | file gate, exported |
| 1134-1145 | `r_alloc_inhibit_buffer_relocation` (C 1135-1144) | file gate, exported |
| 1146-1214 | C 1146's form feed, the `Initialization` banner (C 1147-1149), C 1151's comment, `r_alloc_init` (C 1153-1214) | file gate; C 1161/1180/1193's arms as the gate table above |

## Findings

1. **`refill_memory_reserve`'s declaration and its definition disagree on the ABI** (C 921; the
   mirror side is `femacs/src/include/lisp.rs`, the definition is `femacs/src/alloc.rs` 4557).
   C's prototype is `extern void refill_memory_reserve (void);` inside
   `#if defined REL_ALLOC && !defined SYSTEM_MALLOC` (lisp.h 4437-4439) - the same condition as this
   file's gate - and `alloc.rs` defines it as a plain `unsafe fn` (its own comment: C declares it
   `static`, so the port has no symbol for it).  This unit therefore declares the C-ABI prototype
   and a `rel_alloc` configuration's *link* would not find it.  **Named here, not fixed**: the
   repair belongs to the lisp.h mirror / the alloc.rs lane (see `## Owed`).
2. **`include/lisp.rs` 3756 uses an unstable library type** (`core::ffi::VaList`, the `c_variadic`
   feature) in the `verror` declaration.  With the pinned nightly (`femacs/rust-toolchain.toml`,
   `nightly-2026-10-01`) it compiles; with this machine's *default stable* (1.98.1) **every unit
   that includes `lisp.rs` fails `E0658`** - measured on this unit and on
   `build/femacs/units/alloc/_root.rs`, so it is a property of the mirror, not of this port.  The
   mirror lane owns it (`## Owed` 4).
3. **No other finding in the unit's own text.**  The arm flip's five warnings are all in included
   mirrors (`lib/include/stdbit.in.rs` 644, `src/include/lisp.rs` 3934 - unused doc comments;
   `lib/include/unistd.in.rs` 67-68 - `read`/`write` definitions suspected of shadowing the
   standard library's); none is in `ralloc.rs`.

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/ralloc-lane cargo run -q -p xtask -- build-tu src/ralloc.c`
→ `built src/ralloc.rs -> ../build/femacs/ralloc.o`, **exit 0**, and `build/femacs/ralloc.o` is
**336 bytes** - an empty object.  That is the expected reading, not a proof of the new text: every
item sits behind the two-feature file gate, so under the frozen feature set rustc compiles only
`include/config.rs` and the `#![allow]` attribute (the `androidmenu.md` ① reading, same size).

**② The arm flip (P-86 ③⑴; the file gate plus every sub-gate the frozen configuration has on).**
The frozen configuration's own values are `HAVE_MALLOC_H` undefined, `DOUG_LEA_MALLOC` undefined,
`DEBUG` undefined, `SYSTEM_MALLOC` 1, so the arm C would actually compile is
`all (rel_alloc, no_system_malloc)` with the three false sub-gates left *unset* - which is exactly
what this command passes (P-86 ⑥):

    cd femacs && CARGO_TARGET_DIR=/tmp/ralloc-lane cargo run -q -p xtask -- \
      probe src/ralloc.rs /tmp/ralloc-lane/final.rlib --crate-type=lib -C panic=abort \
      --cfg 'feature="rel_alloc"' --cfg 'feature="no_system_malloc"'

→ **exit 0, 0 `error` lines, 5 warnings**, `/tmp/ralloc-lane/final.rlib` 7,879,024 bytes, and the
log's `grep -c ralloc.rs` is **0** - no diagnostic names the unit.  The five warnings are the three
mirror sites of Finding 3; **0 of them are in the unit's own text**.

**②a The mutation test (P-86 ⑦) - the "0 errors in the unit" reading is real, not vacuous.**
A deliberate type error injected into the unit's *own* text (`let mutation_probe: c_int =
get_bloc (0);` at the head of `r_alloc`'s body) changes the output exactly as a real reading
requires:

    cd femacs && rustc --edition 2024 -O ../build/probes/ralloc/_root.rs -o /tmp/ralloc-lane/x.rlib \
      --crate-type=lib -C panic=abort --cfg 'feature="rel_alloc"' --cfg 'feature="no_system_malloc"'
    # unmutated: exit 0        mutated: exit 1, "error[E0308]: mismatched types" at the injected
    #                          line, "expected `i32`, found `*mut bp`", 1 error

This is the opposite of the `androidmenu.md` ②a result: there, name resolution aborted in the
include graph and the injected error was invisible.  Here the unit's text is reached and
type-checked, so ②'s "0 errors" bears on `ralloc.rs`.  The injected line was removed again and the
recompiled reading is byte-identical to ②'s warning set (exit 0, 0 errors).

**③ Inventory.**
`python3 tools/extract-abi.py --check emacs/src/ralloc.c femacs/src/ralloc.rs` →
**1 `- ` line**: `- line count drifted 29.3%: C 1214 vs Rust 1570`, then `FAIL`.  Everything else in
the checklist is satisfied: all 18 functions' bodies (including `r_alloc_check`'s eight `assert`
sites), all 7 macros (`PAGE`, `PAGE_ROUNDUP`, `MEM_ALIGN`, `MEM_ROUNDUP`, `NIL_HEAP`, `NIL_BLOC`,
`BLOC_PTR_SIZE`), both structs' fields, the 12 file-scope objects, and `constants` is empty on the
C side (every value-like `#define` here is an expression, so the macro rule compares it).  **No
`macro-body-notes.tsv` row is needed** - the one token finding the first draft produced
(`function obtain does not mention 1`, C 244's `new_heap + 1`) was fixed in the port by spelling
C's pointer arithmetic as `.offset (1)`, not by a note.  The line-count row is the integrator's
(`## Owed` 1).

**④ Other gates (repo root).**
`check-cond-eval.py src/ralloc.c` → `9 conditions, 0 unknown, 0 disagreements [VERIFIED]` -
the nine are C 80, 83, 995, 1013, 1161, 1175, 1179, 1193's `#if`s plus the nested `#if 0`
(C 1037), and the gate table's frozen values are the ones that evaluator decided. — `check-declarations.py` → `17 declaration sites (0 forbidden, 17 allowed literals), 0 failures` (no
site in this unit). — `check-mirror-dupes.py` → `17 duplicate declaration(s), 17 ratchet row(s),
0 failure(s)` (this unit adds no row; its `sbrk`/`refill_memory_reserve` declarations are not in any
included mirror). — `check-citations.py` → `676 file(s) scanned, 0 drift, 0 range, 1105 external, 35
allowed, 0 stale`. — `check-citation-text.py femacs/src/ralloc.rs` → `checked 0 / mismatched 0 /
ranges 0 / unresolved 0` (the file's `/// C <line>` citations name no path, so neither citation
tool reads them; the `/// C: <header text>` lines are verbatim). — `check-symbols.py --dir src` →
`75 units compared, 1 mismatched` (that one is `xdisp`, not this unit), `invented=[]`.
`check-port-lines.py` → `38 file(s) under ten percent …; 41 ratchet row(s), 0 new, 3 stale`, one of
the stale rows being `src/ralloc.c: no longer under the threshold` (this port is what moved it; the
ratchet row is the integrator's - `## Owed` 2). — `check-dead-file-coverage.py` → bound by this
ledger's table; reading in `## Evidence` ⑤.

**⑤ This ledger's own table.**
`python3 tools/check-dead-file-coverage.py` → `info docs/ports/ralloc.md: 28 range row(s) cover
src/ralloc.c (1214 lines; 0 blank line(s) between blocks need no row)`, `0 failure(s)`.

**⑥ Budgets (as used, including the overruns).**
`build-tu` **6** of 3: runs 1-3 are the first draft's compile errors (1 the wrong working directory,
which cargo rejected before `xtask` ran; 2 `E0758` on a C comment carrying `/*` inside a Rust block
comment; 3 the `as *mut c_char <` generic-parse error), runs 4-6 are green (`built … -> …/ralloc.o`,
exit 0, **336 bytes** each - 4 and 5 as the fixes landed, 6 the final revision).  `probe` **4** of 3:
run 1 on the first draft (4 errors, all in this unit: an immutable `address` parameter, `memset`'s
third argument, a `while true` tail, and the same `<`); run 2 after those four fixes (green); run 3
the same revision with the log kept; run 4 **after the last acceptance edit** (C 244's `1`,
`## Deviations` 7) so that the recorded reading is the final text.  The overruns are reported rather
than hidden.  The mutation test (②a) was measured with the same `rustc` command line on the root
`xtask probe` had materialized, not with a fifth `probe` invocation.

## Deviations

1. **The `<malloc.h>` stand-in section (C 80-82)**, per P-88 ①: `mallopt` and `M_TOP_PAD` are
   declared where C's `#include` line is, gated `any (have_malloc_h, doug_lea_malloc)` so the C 1176
   arm (which is where both are used) sees them under either configuration while the crate root can
   never hold two copies.  `M_TOP_PAD`'s value (`-2`, glibc's) is **UNVERIFIED** - this machine has
   no `<malloc.h>`; the layout/size question does not arise (it is an `int` constant).
   `mallopt`'s prototype is quoted from the header's documented form, not measured.
2. **`sbrk` is declared at C 1028's use site**, not at C 31's `#include <unistd.h>` line: C's
   `<unistd.h>` (the system header) declares it, the port's `lib/include/unistd.in.rs` mirror does
   not carry it (P-72 rule 4), and C 1028 is the file's only mention.  The declaration is inside the
   `debug` arm, so no other configuration sees it.
3. **`refill_memory_reserve` is declared at C 934's use site** for the same reason (lisp.h 4437-4439
   is inside C's own `REL_ALLOC && !SYSTEM_MALLOC`), and it is Finding 1's ABI mismatch.
4. **`assert` (C 1015's `<assert.h>`) is `core::assert!`**: the header has no mirror and nothing to
   declare (it is a macro); `-C panic=abort`, which every unit's build uses, makes the failure path
   C's `abort`.
5. **C's `#if 0` region (C 1037-1041) is recorded, not transcribed** - dead text in every
   configuration, and `tools/extract-abi.py`'s `dead_branch_map` reads it as dead, so nothing
   compares it (the `alloc.rs` 176-186 reading).  Every other `#if` arm is carried whole.
6. **The explicit null-pointer spellings.**  Where C writes a bare `0`/`NULL` in a pointer position
   (`return 0;`, `*ptr = 0;`, `return NULL;`, `!= 0`, `(heap_ptr) 0`, `(bloc_ptr) 0`) the port keeps
   C's own token: `0 as *mut c_void` / `0 as heap_ptr` / `0 as bloc_ptr` for the zeroes and
   `core::ptr::null_mut ()` where C's token is `NULL` (`alloc.rs` 973's `0 as *mut c_char` shape;
   the two are the same value, and the token keeps the check comparable).
7. **`new_heap + 1` (C 244) is `.offset (1)`** - C's pointer arithmetic on `struct heap *`, i.e.
   `sizeof (struct heap)` past the record, which is also the token the body check wants (③).
8. **Chained C assignments become two statements**, in C's own right-to-left order: `*b->variable =
   b->data = b->new_data` (C 630, 812, 868) is `(*b).data = (*b).new_data; *(*b).variable =
   (*b).data;`, and the four-way chain at C 1165-1166 (`first_heap->start = first_heap->bloc_start =
   virtual_break_value = break_value = real_morecore (0)`) is four assignments in the same order.
   Rust's assignment yields `()`, so the C text has no single-statement spelling.
9. **Three rebindings C gets from its own parameters/locals**: `obtain`'s `address` is
   `mut address: *mut c_void` (C reassigns its parameter), and `relocate_blocs`'s `heap` and
   `address` are rebound with `let mut` for the same reason (the `androidmenu.rs`
   `let mut contents = contents;` shape).
10. **`find_heap`'s infinite `for` loop is `loop`**, not `while true`: the loop is the function's
    tail, so it must have type `!` to satisfy the return type (C's `for (…; ; …)` never falls
    through).
11. **Two mirror includes that C reaches through `lisp.h`'s chain** (`<stdlib.h>` for
    `malloc`/`free`, `<string.h>` for `memmove`/`memset`) are included right after
    `include!("include/lisp.rs")`, because the port's `include/lisp.rs` does not carry either mirror
    yet - the `buffer.rs` 38/41 reading.  Verified: the frozen preprocessor's own output for this
    file declares `malloc`/`free` through `build/emacs-ref/lib/stdlib.h` and `getpagesize`/`sbrk`
    through the system `<unistd.h>`.
12. **C's `^L` page breaks are not carried as bytes.**  C 47, 88, 168, 340, 417, 481, 541, 641,
    716 and 1146 each hold a lone form feed; the port records them in the range table's rows and
    writes the following block at that position, but does not carry the character (the
    `w32heap.rs` reading).  Every tool reads such a line as blank, so nothing is lost from the
    coverage table; three ported files elsewhere in the tree do carry them.
13. **The `/// C` citations use the P-86 prose form** (`C <line>: …`), which neither
    `check-citations.py` nor `check-citation-text.py` reads (both require a path with a suffix); the
    `/// C: <header text>` lines that stand in for a system header's declarations are verbatim.

## Owed

1. **`docs/ports/line-drift-tolerance.tsv`**: one row for the stem `ralloc`, tolerance **≥ 0.30**,
   basis `total`, reason from `## Evidence` ③ - the file carries 356 lines more than C 1214 for
   reasons the C text does not have: the per-item file-gate attribute (~60 items), the `<malloc.h>`
   stand-in section, the `sbrk`/`refill_memory_reserve` declarations, the explicit `0 as *T` and
   `.offset (1)` spellings, and the two chained-assignment expansions of Deviations 6/8.  Not this
   lane's file; until it lands, `extract-abi.py --check` stays `FAIL` with the one row ③ records.
2. **`tools/check-port-lines.py --write-pending`** for the stem `ralloc` (P-86 ④): the ratchet row
   `src/ralloc.c 1214 17 121 known-unported target` is now `stale … no longer under the threshold`
   (④).  Not this lane's file.
3. **The `rel_alloc` configuration's link** (Findings 1): when some configuration first sets
   `REL_ALLOC` and clears `SYSTEM_MALLOC`, `refill_memory_reserve` needs one ABI - either
   `alloc.rs` exports it (`#[unsafe(no_mangle)]`) or the mirror + this unit declare the Rust ABI it
   actually has.  Only a link can decide it; a type-check cannot.
4. **The lisp.h mirror lane**: `include/lisp.rs` 3756's `core::ffi::VaList` makes the whole mirror
   nightly-only (Finding 2).  With the pinned nightly it is invisible; it is the reason a unit
   including `lisp.rs` cannot be compiled by this machine's default stable toolchain.
5. **A real `rel_alloc` acceptance**: the arm flip is a type-check of the text under one feature
   set, so `obtain`/`relinquish`/`r_alloc_sbrk`'s arithmetic, the two-source `__morecore` hook, the
   `DEBUG` assertion set and the `DOUG_LEA_MALLOC` reinit path are transcribed and type-checked, not
   exercised.  Link and run on a configuration that defines `REL_ALLOC` is what would move this
   piece from "type-checks" to "works".
6. **A measured `M_TOP_PAD`/`mallopt`** (Deviation 1) needs a glibc `<malloc.h>`, i.e. a Linux
   toolchain this machine has not got; until then the declaration is the documented one.

## Not claimed

- **The frozen configuration compiles nothing here**; the 336-byte object is the empty object and
  the port is not "built" by any reading of this tree.
- **The arm flip is a type-check, not a C compile, not a link, not a run**: no `ralloc.o` exists for
  any configuration on this machine, `docs/baseline/frozen-build-sources.txt` lists none, and
  `extract-abi.py --check` still exits `FAIL` (the line-count row of `## Owed` 1).
- **Nothing about the allocator's behaviour is claimed**: the heap/blob list manipulation, the
  relocation hysteresis, the `extra_bytes` accounting and the `sbrk` bookkeeping are transcribed
  from C's text; no differential run, no ERT case and no memory-pressure test exists for them.
- **No layout assertion is claimed for `struct heap`/`struct bp`** (they are file-local `static`
  structures with no frozen-baseline row - the file is not compiled by the frozen configuration, so
  there is no C compiler output to assert against); their field lists are compared by
  `extract-abi.py`'s struct-field rule, which is not a layout measurement.
- **`M_TOP_PAD`, `mallopt` and `sbrk`'s prototypes are the documented ones, not measurements**
  (Deviations 1-2); the `DEBUG` arm's `sbrk` is unavailable on this target's `<unistd.h>` mirror.
- **The `<assert.h>`/`assert!` equivalence is a spelling claim, not a semantic one** (Deviation 4):
  the port's `assert!` aborts, as C's `assert` does with `NDEBUG` undefined, but no test exercises
  it.
- **The `#if 0` region's text is not carried** (Deviation 5), and it is not counted as covered
  evidence anywhere.
