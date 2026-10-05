# emacs/exec/exec.c (1185 lines) — whole file, C 1-1185

> **One of P-86's class: the frozen configuration compiles nothing here.**  The exec loader belongs
> to the unexec dump path, and `exec/configure.ac` 235's `OBJS="exec.o trace.o"` is reached only from
> the `--with-android` configuration (`configure.ac` 178-220), while this build is `HAVE_PDUMPER=1`;
> `docs/baseline/frozen-build-sources.txt`'s batch-F section records no `exec/` object for this
> target, so `tools/extract-abi.py`'s checklist for the file is empty (P-86 ②) and this ledger is
> that clause's substitute - the **C line-range table**, the `## Evidence` readings and the
> `## Deviations`.
>
> The previous revision of `femacs/exec/exec.rs` was the three-line P-08 note ("not compiled by the
> frozen configuration … the port is an empty unit").  This round transcribes the whole body as it
> stands (P-86 ①): **C 1-1185, both arms of all 22 conditions, nothing stubbed**.
> Port: `femacs/exec/exec.rs` (**1552 lines**, md5 `a2560ed5f8694914701fb508798058d2`, sha256
> `130ff7633c095c3abdb2937b423992fa3808d9052c5811dad8b1389abaa1ab0f`).  The reference
> `emacs/exec/exec.c` is md5 `f9517c3c32379ecc87b991a243a52c9a` (unchanged; `emacs/` is read-only).
>
> **The file has no gate of its own.**  It contains no file-level `#if` - the 22 conditions below all
> sit *inside* the body - and it is excluded from the frozen build by the build lists, not by a
> condition.  So the port follows the three sibling units of the same directory (`exec1.rs`,
> `test.rs`, `mipsfpu.rs`: no whole-item `#[cfg]`) rather than `exec/trace.rs`'s `have_android` gate,
> and the frozen `build-tu` reading is a real object rather than the 336-byte empty one
> (Evidence ①).  Every `#if` inside the body carries both its arms, and its gate names the macro the
> C text tests (P-51); the verdicts are the evaluator's own, because `exec/exec.c` is on
> `docs/baseline/config/cond-eval-verified.txt` (line 6).
>
> **Acquisition: the arm flips are `rustc` readings, not a Linux/Android build.**  The system surface
> this file needs (`<errno.h>`, `<unistd.h>`, `<fcntl.h>`, `<sys/ptrace.h>`, `<sys/param.h>`,
> `<sys/mman.h>`, `<string.h>`) is the *target's*, and its readings are this machine's clang against
> the NDK sysroots (`aarch64-linux-android24` 29.0.14206865) except where a line says UNVERIFIED; no
> Linux toolchain here links or runs the loader, so nothing in this round is behaviour.

## Gate mapping

The file gate is its absence from the frozen source list (above); the table is the body's 22
conditions in C's order, as `python3 tools/check-cond-eval.py exec/exec.c` reads them
(`22 conditions, 0 unknown, 0 disagreements [VERIFIED]`, Evidence ⑥).  The feature column is the
name the port writes, "second arm" the one the `#else`/`#elif` carries; a row with no second arm has
no `#else` in C and therefore nothing to carry.

| C `#if` | tests | feature (first arm) | second arm |
| --- | --- | --- | --- |
| 35 | `defined __mips__ && !defined MIPS_NABI` (the `"mipsfpu.h"` include) | `all (mips, not (mips_nabi))` | `not` (nothing to carry) |
| 44 | `!defined HAVE_STPCPY \|\| !defined HAVE_DECL_STPCPY` | `any (not (have_stpcpy), not (have_decl_stpcpy))` - **true** on the frozen table | `all (have_stpcpy, have_decl_stpcpy)` |
| 234 | `defined __mips__ && !defined __LP64__` (the `fpu_mode` member) | `all (mips, not (lp64))` | `not` |
| 295, 305 | `#ifndef PAGE_MASK` | `not (page_mask)` - **true** on the frozen table | `page_mask` (no `#else`) |
| 308 | `#ifdef HAVE_GETPAGESIZE` | `have_getpagesize` - **true** on the frozen table | `not` |
| 402, 439, 461, 524, 616, 659, 922, 1081, 1118 | `defined __mips__ && !defined MIPS_NABI` | `all (mips, not (mips_nabi))` | `not` |
| 540, 1031 | `#ifdef EXEC_64` | `exec_64` | `not` (**live** on the frozen table) |
| 549, 1040 | `#ifndef WORDS_BIGENDIAN` | `not (words_bigendian)` - **true** on the frozen table | `words_bigendian` |
| 841, 855 | `#ifdef __aarch64__` | `aarch64` - **true** on the frozen table | `not` |
| 1134 | `#elif defined __mips__ && !defined __LP64__` | `all (mips, mips_nabi, not (lp64))` (C's `#elif` is reached only when the arm above is off, so the port's condition is that arm's complement plus C 1134's own terms) | — |

## C line ranges

Rows are contiguous, ascending and cover C 1-1185 with no gap at all; `docs/port-map.md` binds this
ledger to `exec/exec.c` - and, through the same prefix fallback, to the `exec/exec.h` mirror
`femacs/exec/include/exec.rs` as well (`check-dead-file-coverage.py`'s `ledger_for` tries
`exec-exec.md` and then `exec.md`, so the mirror's own `docs/ports/include-exec.md` is not
consulted; the tool prints both bindings, and a gapless table satisfies both).  The blank lines
between blocks therefore sit inside a row rather than between two.

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's opening declaration block | dropped (PORTING.md 0.1-19) |
| 20-38 | the include block: `config.h` (20), `<errno.h>`-`<stdlib.h>` (22-27), `<sys/ptrace.h>`-`<sys/mman.h>` (29-31), `"exec.h"` (33), the `__mips__`-gated `"mipsfpu.h"` (35-36) and its `#endif` (38) | every system header is the target's, declared at its C line (P-88 ①); the `"mipsfpu.h"` edge carries its gate |
| 39-41 | the file's page break (a form feed) | — |
| 42-67 | "Define replacements for required string functions." and the `HAVE_STPCPY`/`HAVE_DECL_STPCPY` block: `rpl_stpcpy` (46-63), `#define stpcpy rpl_stpcpy` (65), the `#endif` (67) | C 44's pair; the first arm is the live one on the frozen table, and C 65 is the port's `use rpl_stpcpy as stpcpy;` |
| 68-86 | the page break and "Executable reading functions. …" - the loader's commentary; then the `check_interpreter` banner (87-93 is the next row's text, this one ends at the page break) | — |
| 87-94 | the `check_interpreter` banner and its "*EXTRA" sentence | — |
| 95-166 | `check_interpreter` (C 98's two function-local statics, C 106/109/114/125/140/150/158's `goto`s, C 155's `nomatch:` and C 162's `fail:`) | — |
| 167-174 | `loader_area` (168) and `loader_area_used` (171), then "Structure definitions for commands placed in the loader area. …" (175-176 is the next row) | — |
| 175-238 | "Structure definitions for commands placed in the loader area. …" and the three structs: `exec_open_command` (178-185), `exec_map_command` (187-209), `exec_jump_command` (211-238) with C 234-237's `fpu_mode` member | `all (mips, not (lp64))` for that member |
| 239-245 | the page break and the `write_open_command` banner | — |
| 246-278 | `write_open_command` | — |
| 279-286 | the page break and the `write_load_command` banner | — |
| 287-401 | `write_load_command` (C 295-297 the `#ifndef PAGE_MASK` static, C 305-318 "This system doesn't define a fixed page size." with the `HAVE_GETPAGESIZE` pair and C 316-317's two `#define`s, C 335/338/341's `PF_R`/`PF_W`/`PF_X` tests) | `not (page_mask)`, `have_getpagesize` |
| 402-422 | the `__mips__ && !MIPS_NABI` static storage (`exec_abi`/`interpreter_abi`, `exec_interpreter_header`, `interpreter_header`, `exec_abiflags`, `interpreter_abiflags`) | `all (mips, not (mips_nabi))` |
| 423-432 | the `process_interpreter_1` banner | — |
| 433-477 | `process_interpreter_1` (C 439-441's `rc1`, C 445's `default:`, C 449's `case 1`, C 461-472's `PT_MIPS_ABIFLAGS` arm) | `all (mips, not (mips_nabi))` |
| 478-483 | the `process_interpreter` banner | — |
| 484-593 | `process_interpreter` (C 524-529 the interpreter's header record, C 540-546 the `EXEC_64` pair, C 549-555 the `WORDS_BIGENDIAN` pair, C 570-579 the program-header loop, C 589's `fail:`) | `all (mips, not (mips_nabi))`, `exec_64`, `words_bigendian` |
| 594-607 | the `process_program_header` banner | — |
| 608-675 | `process_program_header` (C 622's `default:`, C 630-644 the two `write_load_command` sites with C 635/642's `(USER_WORD) -1` test, C 648's `case 3`, C 654-657's `PT_GNU_STACK` `/* TODO */`, C 659-670's mips arm) | `all (mips, not (mips_nabi))` |
| 676-682 | the `insert_args` banner | — |
| 683-865 | `insert_args` (C 697's `while (true)`, C 704-709's `errno` protocol, C 830/834's two `assert`s, C 841-847's `__aarch64__` pair, C 853's `fail:`) | `aarch64` for the register-accessor pair, `no_ndebug` for the live `assert` |
| 866-869 | the page break and the `format_pid` banner | — |
| 870-894 | `format_pid` | — |
| 895-908 | the `exec_0` banner | — |
| 909-1185 | `exec_0` (C 922-924's `fpu_mode`, C 1031-1037 and C 1040-1046's two pairs, C 1081-1086's mips clear, C 1118-1136's FPU-mode block with its `#elif`, C 1180's `fail1:` running into C 1182's `fail:`) | `all (mips, not (mips_nabi))`, `all (mips, mips_nabi, not (lp64))`, `exec_64`, `words_bigendian` |

## Evidence

**① The frozen reading.**  `cd femacs && CARGO_TARGET_DIR=/tmp/exec-lane/cargo
/tmp/exec-lane/cargo/debug/xtask build-tu exec/exec.c` → `built exec/exec.rs ->
../build/femacs/exec.o`, **exit 0**, object **49,472 bytes**.  That is a real object because this
unit carries no whole-file gate (the sibling units' shape): the body compiles with every sub-gate
off, which is exactly the frozen table's own arm for C 44, C 295/305, C 549/1040 and C 1031.

**② The arm readings.**  Two `probe` invocations (the `build-tu` flags plus `--crate-type=lib -C
panic=abort` and the cfgs below; the named set is the one the arms need, and the two sets together
cover every arm - the `mips`/`aarch64`/`exec_64` axes are independent, so one invocation cannot
carry them all):

* **P1, the maximal set** (`aarch64`, `exec_64`, `words_bigendian`, `page_mask`, `have_stpcpy`,
  `have_decl_stpcpy`, `have_getpagesize`, `mips`, `mips_nabi`, `no_ndebug`): **exit 0, 0 errors, 7
  warnings, a 176,392-byte rlib**.  This is the arm where the `__aarch64__` register accessors, the
  64-bit ELF records, the big-endian test, the system `PAGE_*` pair, the C library's `stpcpy`,
  `getpagesize` and the live `assert` are all compiled.
* **P2, the frozen-side-plus-mips set** (`mips`, `no_ndebug`, nothing else): **exit 0, 0 errors, 7
  warnings, a 200,440-byte rlib**.  This is the arm that carries C 44's live `rpl_stpcpy`, the file's
  own `PAGE_MASK`/`PAGE_SIZE`/`pagesize`, the `"mipsfpu.h"` include and all nine
  `__mips__ && !MIPS_NABI` blocks, plus the 32-bit ELF records.

Every warning in both readings is one of two classes: `unused doc comment` on a `///` line above a
`static`/`extern` item (six of them; rustdoc generates no documentation there - the class
`docs/ports/haiku.md` and `docs/ports/ntlib.md` record), and `suspicious definition of the runtime
`read` symbol used by the standard library` (one, on this unit's own `<unistd.h>` declaration of
`read`, which is C's name for it).  Neither is in the unit's statements, and neither is an error.

**③ Four more `rustc` runs, reported because they are not `probe`/`build-tu` invocations.**  The
four remaining arm combinations were read with the same flags straight on the materialized
`_root.rs` (the file `probe` had already materialized, re-compiled): the **frozen set** (no `--cfg` at
all), **`mips` + `mips_nabi` + `no_ndebug`** (C 1134's `#elif` arm), **`page_mask` +
`words_bigendian` + `have_stpcpy` + `have_decl_stpcpy`**, and **`exec_64` + `aarch64` +
`have_getpagesize` + `no_ndebug`** - each **0 errors**.  With P1/P2 that is every arm of the
`## Gate mapping` table.  These four are *not* `probe` invocations: they ran `rustc` on the file
`probe` had already materialized, and they are counted here rather than hidden.

**④ The mutation test (P-86 ⑦: the readings above are not vacuous).**  Injecting
`text_size = c"exec-mutation".as_ptr ();` above `insert_args`'s own C 721 statement (in a **copy** of
the materialized root, so the unit's text was never mutated) makes the same P2 command report
**exactly one error**, `E0308 mismatched types`, naming that line and nothing else (`aborting due to 1
previous error`).  So rustc really does compile *this unit's* statements under `mips`, rather than
stopping in a mirror.

**⑤ The measurements behind the system surface** (NDK clang, `--target=aarch64-linux-android24`:
`ENOEXEC` 8, `ENOMEM` 12, `ENAMETOOLONG` 36, `X_OK` 1, `SEEK_SET` 0, `O_RDONLY` 0, `_SC_PAGESIZE`
0x27 = 39, `PATH_MAX` 4096, `PROT_READ/WRITE/EXEC` 0x1/0x2/0x4, `MAP_PRIVATE/FIXED/ANONYMOUS`
0x02/0x10/0x20, `PTRACE_PEEKDATA/POKETEXT` 2/4; `--target=x86_64-linux-android24`:
`PTRACE_SETREGS` 13, `__LP64__` 1).  `exec/exec.h`'s own line numbers are the ones the mirror cites.

**⑥ The `#if` verdict.**  `python3 tools/check-cond-eval.py exec/exec.c` → `22 conditions, 0 unknown,
0 disagreements [VERIFIED]`, and `exec/exec.c` is one of the files that list carries (line 6), which
is why `extract-abi.py`'s inventory consults the macro table for it.

**⑦ `extract-abi.py --check` (P-86 ⑩).**  `python3 tools/extract-abi.py --check emacs/exec/exec.c
femacs/exec/exec.rs` → after this round **one `- ` row**: `line count drifted 31.0%: C 1185 vs Rust
1552`.  The six token rows the first run produced (`check_interpreter` → `continue,goto`;
`process_interpreter_1` and `process_program_header` → `break,case,default,switch`;
`process_interpreter` and `exec_0` → `goto`; `insert_args` → `goto,true`) are now **rows of
`docs/ports/macro-body-notes.tsv`** (appended once, at the end, each with its C lines and its reason
- the P-86 ⑩ mechanism, the `exec/trace.rs` precedent).  The remaining drift row is a *line-count*
reading, not a text one: the per-`#if` `#[cfg]` pairs, the in-unit system surface with a citation per
declaration, and the `/// C <n>:` lines at every statement group are what a 1,185-line C file becomes
in this port.  Its tolerance row is `## Owed` 1 (that file is not this lane's to write).

**⑧ The coverage and shape gates.**

* `python3 tools/check-citations.py` → **0 findings** for `femacs/exec/exec.rs` (0 `range`, 0 `drift`,
  0 `external` on this unit's lines).
* `python3 tools/check-declarations.py` → **0 findings** for this unit and this ledger (all four
  classes: no copied declaration block, no product-name version line, no product name in a comment,
  and no recital about what was or was not carried).
* `python3 tools/check-dead-file-coverage.py` → the new table binds this ledger:
  **0 failure(s)** (the row count and the blank-line slack are in the tool's own `info` line).
* `python3 tools/check-mirror-dupes.py` → this unit is **out of its scope by construction**
  (`UNIT_DIRS = ("femacs/src", "femacs/lib")`), and this round adds no declaration to a mirror: the
  run is recorded here only to say so.
* `python3 tools/check-ledger-ticks.py` → nothing to tick: a P-86 ledger records a range table, not
  boxes, so this file contributes no row (the run is recorded for the same reason).

**⑨ Budget, reported honestly.**  `build-tu` **3** (the initial frozen reading, the fix-pass one, and
the final one) and `probe` **3** - the first on the pre-fix revision of the unit (it is what caught
the `E0659` of Deviation 11), then P1 and P2 on the final text.  Beyond both budgets there were about
**a dozen direct `rustc` invocations on the already-materialized `_root.rs`** - the fix loop's
error-list runs, the four arm sets of Evidence ③, and the mutation test of Evidence ④ - which are
neither `build-tu` nor `probe`; the readings quoted in this ledger are the final text's.  No
whole-tree gate was run (`check-all-units.sh`, `gate-fast`, `xtask --all` are out of this lane's
budget and prohibited), and every build artifact this lane produced was deleted as it was made
(`build/femacs/units/exec`, `build/probes/exec`, `build/femacs/exec.o`).

## Deviations

1. **The file has no gate of its own.**  C's body hangs on no file-level `#if`; it is the *build
   lists* that exclude `exec/` here.  The port therefore carries no whole-item `#[cfg]` (the
   `exec1.rs`/`test.rs`/`mipsfpu.rs` shape rather than `exec/trace.rs`'s `have_android`), and the
   frozen object is a real 49,472-byte one instead of the 336-byte empty one.  Every inner `#if`
   still carries both arms (P-86 ①).
2. **The system surface is declared at its own C `#include` line** (P-88 ①) and the landed mirrors
   are deliberately *not* in this unit's graph: four of their readings are the frozen target's, not
   the loader's target's (`lib/include/errno.rs` 12's `errno` is `(*__error ())` and its
   `ENAMETOOLONG` 63, `lib/include/limits.in.rs` 131's `PATH_MAX` 1024, `lib/include/unistd.in.rs`
   143's `_SC_PAGESIZE` 29), and `lib/include/string.rs` 38 carries `stpcpy` where this file's own
   C 44 `#if` decides it - one translation unit cannot hold both answers (the `E0252` class the
   `exec/exec1.rs` round measured).  Each declaration keeps a `/// C: <header>: …` line.
3. **C's arithmetic conversions are explicit.**  C's implicit integer promotions and narrowing
   assignments read as `as` casts, and the loader's address/size expressions (C 320-331, C 352-377,
   C 749-791, C 816, C 1111-1166) are spelled with `wrapping_*` where C's own type is unsigned and can
   wrap, so a debug build (`-C opt-level=0` is what `build-tu` uses) cannot panic where C wraps
   (PORTING.md 1.1).  The expressions' values are C's.
4. **C's uninitialised locals** are written with C's own zero values (C 99-100, 249-250, 291-294,
   438/440, 488-491, 615/617, 687-690, 913-927; PORTING.md 1.5), which is the only way Rust lets a
   later write pass their address.
5. **C's control-flow shorthands** are spelt once, the tree's way: `goto` and its labels are labeled
   blocks (`break 'x`, PORTING.md 2.2 - C 999's `goto fail` targets C 1182's label while C 1014-1159's
   `goto fail1` targets C 1180's, so the port nests `'fail1` inside `'fail` and lets `fail1` run into
   `fail` as C does); `switch` is `match` with C's `default:` as the `_` arm (Rust wants it last);
   `while (true)` is `loop`; C's `for (init; cond; incr)` is the init followed by a `loop` whose
   increment sits at the body's end; and C 974's comma operator is two statements.  The tokens this
   removes are the `macro-body-notes.tsv` rows (Evidence ⑦).
6. **`USER_REGS_STRUCT` is the port's spelling of a host record** (P-72 (3)): `exec/configure.ac`
   242/276/308/341/367 names the host's register structure and 251-257 its members, and the C text
   reaches it through those macro names (`regs->SYSCALL_ARG1_REG`, C 693 and C 839), so each field
   keeps the macro's name (the `exec/trace.rs` 380-412 shape).  Its **layout and therefore its size
   are not measured** - no Linux toolchain here defines the record - so `sizeof *regs` at C 740 is
   the port's record's size, which is `## Owed` 3.  The mirror's `*mut c_void` spelling of those
   pointers is P-72 (3)'s own case, so the four call sites convert explicitly instead of shadowing
   the mirror's prototype with a second, C-spelled declaration (measured: that second declaration
   makes the frozen reading emit `clashing_extern_declarations`, which the conversion does not).
7. **Two host constants are the x86-64 arm's readings.** `EXECUTABLE_BASE` 0x555555554000 and
   `INTERPRETER_BASE` 0x600000000000 come from `exec/configure.ac` 273/274; the aarch64, i386, arm and
   mipsel arms' values (334-337, 296-299, 360-363, 400+) are **UNVERIFIED** and are named in the
   unit's note (the same class `exec/trace.rs`'s per-host table covers).
8. **Five target values replace the mirrors' frozen ones**: `PATH_MAX` 4096 (the mirror's 1024),
   `_SC_PAGESIZE` 39 (29), `ENAMETOOLONG` 36 (63), `errno` as `(*__errno_location ())` (not
   `(*__error ())`), and `PTRACE_SETREGS` 13 (the x86 family's request, absent from the aarch64
   header; needed by the `not (aarch64)` arm).
9. **`assert`'s two arms are carried** under `no_ndebug` (this unit's own `config.h` has no `NDEBUG`;
   `src/conf_post.h` 28-29's is not in its graph), and the live arm's failure entry is `abort`, since
   `<assert.h>`'s `__assert_fail (#expr, __FILE__, __LINE__, __func__)` has no Rust counterpart
   (PORTING.md 2.5; the `exec/trace.rs` 245-283 shape).  The layout assertions this unit adds are
   `core::assert!` for the same reason (2.5's later half).
10. **The `page_mask` arm's pair is Linux's** (`<asm-generic/page.h>`'s `PAGE_SHIFT` 12,
    `PAGE_SIZE (1 << PAGE_SHIFT)`, `PAGE_MASK (~(PAGE_SIZE - 1))`) and is **UNVERIFIED** as a clang
    reading, because the Android sysroots carry those names only in the kernel uapi headers.  The
    other arm's `pagesize`/`PAGE_MASK`/`PAGE_SIZE` (C 296, 316-317) are the file's own, and they sit
    at crate root rather than inside the function: a `#define` is file-scoped in C, and a
    function-local `static mut` is reachable from a `macro_rules!` in the same body only through its
    definition-site scope.
11. **The `"mipsfpu.h"` include sits at the crate root with its gate on the edge**, not in a wrapper
    module: the mirror declares its own `mipsfpu_decls` module, so a wrapper of that name is
    `E0659`-ambiguous (measured, Evidence ④'s fix round).  The edge carries both of C 35's terms.
12. **`mipsfpu_decls`' content arrives through `use mipsfpu_decls::*;` at the mirror's top level**
    (the mirror's own shape), and the port adds nothing to it: `determine_fpu_mode`,
    `cpu_supports_fr0_p`, `FP_FR0`/`FP_FRE` and `struct mips_elf_abi_flags` are used by their C
    names.
13. **A mips 64-bit configuration without `MIPS_NABI` does not compile in C either.**  C 1133 assigns
    `jump.fpu_mode` while C 234-237 gives that member only when `!__LP64__`; the port carries both
    arms as C has them, so the same configuration fails the same way (it was not probed).

## Owed

1. **`docs/ports/line-drift-tolerance.tsv` needs a row for the stem `exec`**: C 1,185 vs Rust 1,552
   is a **31.0%** drift - the per-`#if` `#[cfg]` pairs, the in-unit system surface (each declaration
   with its `/// C:` line) and the per-statement `/// C <n>:` citations.  A row such as
   `exec	0.32	total` with that reason is what `tools/extract-abi.py --check` needs; the file is not
   this lane's to write (the P-86 ⑩ precedent: the integrator wrote the same class of row for
   `exec1`/`test`/`mipsfpu` in 3.446/3.449).  Until it lands, `--check` reports the one row
   Evidence ⑦ reads.
2. **The new feature names need registering** (`femacs/xtask/Cargo.toml`'s `[features]` table and
   `docs/baseline/feature-coverage.tsv`, the mechanism `_timezone_defined` went through in 3.504):
   `mips`, `mips_nabi`, `lp64`, `page_mask`, `no_ndebug`.  `aarch64`, `exec_64`, `words_bigendian`,
   `have_stpcpy`, `have_decl_stpcpy` and `have_getpagesize` are registered; `mips` is used by
   `femacs/exec/trace.rs` already and is *not* in the table either (measured 2026-10-05), so that
   row is shared.  `tools/feature-coverage.py --write` is the tool.
3. **A per-host table for `USER_REGS_STRUCT`** (its size and its `SYSCALL_ARG1_REG` member, per
   `exec/configure.ac` 240-400): this is `docs/ports/trace.md`'s `## Owed` 1, and it is what closes
   `sizeof *regs` at C 740 and the record's spelling in both units.  Until it lands, this unit's
   record is the seven macro-named words plus the host's indexed array (Deviation 6).
4. **`EXECUTABLE_BASE`/`INTERPRETER_BASE` per host** (Deviation 7): the unit declares the x86-64
   readings only, and the other four arms' values are named but UNVERIFIED.
5. **`docs/baseline/port-lines-pending.tsv`'s `exec/exec.c` row is now stale** (the ratchet reads
   1,552 > 10% of 1,185); `tools/check-port-lines.py --write-pending` is the tool, and
   `docs/baseline/**` is not this lane's to write.
6. **`docs/status.md`, `docs/port-map.md` and `docs/baseline/feature-coverage.tsv`**: the
   integrator's regeneration and running-log entry for this revision.
7. **A tool-side binding the integrator should decide**: `tools/check-dead-file-coverage.py`'s
   `ledger_for` binds the `exec/exec.h` mirror (`femacs/exec/include/exec.rs`) to *this* ledger - its
   prefix rule tries `docs/ports/exec-exec.md` and then falls back to `docs/ports/exec.md`, so the
   mirror's own `docs/ports/include-exec.md` is never consulted, and the tool prints two `info` lines
   for one ledger (Evidence ⑧).  This table is gapless (C 1-1185, so 1-215 is covered too) and the
   gate reads `0 failure(s)` either way, but `tools/**` is not this lane's write set: the fallback
   either wants the `include-` spelling it uses elsewhere (`docs/ports/include-exec.md`), or the
   mirror's ledger wants a range table of its own.
8. **The `<asm-generic/page.h>` values** (Deviation 10) and the four other hosts' base addresses
   (Owed 4) stop being UNVERIFIED exactly when a Linux/mipsel toolchain exists here; the readings
   then belong to this ledger's Evidence, not to the unit's comments.

## Not claimed

* **No runtime evidence of any kind.**  Nothing was linked, nothing was executed, and no tracee was
  ever traced: the loader's whole job (`exec_0`'s action list, `insert_args`'s rewritten `argv`,
  `ptrace` round trips) is unexercised.  The reference compiles no object for this file on this
  target either, so there is not even an object-level comparison.
* **The arm readings are compile readings, not Linux builds.**  P1/P2 (and Evidence ③) type-check
  the arm sets under this machine's pinned `rustc` (`nightly-2026-10-01`); they are not a
  `linux-android` build, nothing in them links, and the NDK readings measure *headers*, not
  behaviour.
* **`USER_REGS_STRUCT`'s layout is documented, not measured** (Deviation 6/Owed 3), and the mips
  statistics and the `fpu_mode` computation are text: no mipsel toolchain here defines
  `struct mipsel_regs`, and `determine_fpu_mode`'s result is never checked against a real FPU mode.
* **The line-count drift (Evidence ⑦) is a size reading, not text equality**, and its tolerance row
  is owed rather than argued; `extract-abi.py --check` still reports that one row until `## Owed` 1
  lands.  Nothing in this ledger claims the file's checklist is non-empty (P-86 ②) or that the port
  is complete beyond C 1-1185.
