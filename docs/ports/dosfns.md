# emacs/src/dosfns.c (798 lines) — whole file, C 1-798

> **One of P-86's class: the frozen configuration compiles nothing here.**  `dosfns.o` joins the
> MS-DOS build's own object list rather than this target's: `emacs/src/Makefile.in` 306 leaves
> `MSDOS_OBJ` empty for every configure, and `emacs/msdos/sed1v2.inp` 136 rewrites that line to
> `= dosfns.o msdos.o w16select.o` when the MS-DOS make files edit the generated Makefile;
> `docs/baseline/frozen-build-sources.txt` lists no `dosfns.o`.  So `tools/extract-abi.py`'s
> checklist for the file is empty (P-86 ②) and this ledger is that clause's substitute: the **C
> line-range table**, the `## Evidence` readings and the `## Deviations`.  Port:
> `src/dosfns.rs` (**1218 lines**, md5 `f17629839549f3e069b2ebb166303326`), carrying **C
> 1-798 with both arms of every `#if`**.  The reference `emacs/src/dosfns.c` is md5
> `499c403b6d64ae28b2b19dacbc514c07` (unchanged; `emacs/` is read-only), and its `docs/port-map.md`
> 419 row pairs `src/dosfns.c` 798 with `src/dosfns.rs` (the port's own count in that table
> is the integrator's regeneration — Owed 4).
>
> **This ledger replaces the reading it inherits.**  The previous `docs/ports/dosfns.md` (and the
> 12-line comment unit it described, which read the file as "@ 804 lines") concluded "the frozen
> configuration does not compile this file ⇒ the port is the empty unit plus the note that names
> the gate".  P-86 ① replaces that conclusion with *transcribe the text, both arms, no stub*; what
> the earlier note measured (not in the frozen source list, empty checklist, 336-byte `build-tu`
> object) is unchanged and still true — it is the *conclusion*, and the reference's own line count
> (798, not 804), that are withdrawn, exactly as `docs/ports/w32inevt.md`, `w32console.md`,
> `w32menu.md` and `w32term.md` record for their own pairs.
>
> **The file gate is `msdos`, on every item *and* on the include graph** (P-86 ⑧: an ungated
> `include!` would break the frozen compile at once, which is why Evidence ① is an empty object).
> C 21's `#include <config.h>` is the one edge outside it, as C has it.  Acquisition: **this piece
> is `text-only` (P-86 ③⑵)** — there is no DJGPP toolchain on this machine (`which -a
> i586-pc-msdosdjgpp-gcc i686-pc-msdosdjgpp-gcc djgpp-gcc` is empty; no `djgpp`/`go32` include
> directory under `/opt/homebrew`, `/usr/local` or `/Library`), and the reference tree carries none
> of `<dos.h>`, `<dpmi.h>`, `<go32.h>`, `<dirent.h>`, `<sys/vfs.h>`, `<unistd.h>`, `<grp.h>`,
> `<crt0.h>`.  So there is **no arm flip on a DJGPP target** and **no measurement of any record,
> value or prototype of theirs**: every one is `UNVERIFIED` and named in `## Owed`.

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (`MSDOS_OBJ`, `emacs/src/Makefile.in` 306 + `emacs/msdos/sed1v2.inp` 136; the macro is `MSDOS`, `configure.ac` 6953's `AH_TEMPLATE`) | `feature = "msdos"` (registered, `xtask/Cargo.toml` 400), on every item *and* on C 26-54's include graph | the DJGPP (MS-DOS) build; `MSDOS` is `undef` in `docs/baseline/config/darwin-arm64.macros.tsv` 2849, so no configuration this machine can build sets it |
| C 21 `#include <config.h>` | **no gate** (C has it outside `#ifdef MSDOS`; `include/config.rs`) | every build — the file's only ungated edge (P-86 ⑧) |
| C 163, C 252, C 321, C 375, C 675, C 711, C 738 `#ifndef HAVE_X_WINDOWS` | `not (feature = "have_x_windows")` (registered, `Cargo.toml` 359) | `HAVE_X_WINDOWS` is `undef` here, so this is the live side *of its condition* - the `src/msdos.rs` 20/402 spelling; the whole file is still behind the `msdos` gate |
| C 275 `#ifndef SYSTEM_MALLOC` and C 537/C 572's pair | `feature = "no_system_malloc"` (the registered `!SYSTEM_MALLOC`, `Cargo.toml` 438; the `src/msdos.rs` 659 spelling) | the two `.c` conditions have no `#else` in C: C 537-539 and C 572-577's *other* arm is the unguarded statement below them, and both arms are carried (P-51) |

`tools/check-cond-eval.py src/dosfns.c` reads **`11 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** (Evidence ④): C 23, 163, 252, 275, 321, 375, 537, 572, 675, 711 and 738 are the whole
conditional text of the file.  `MSDOS` alone decides C 23; the seven `HAVE_X_WINDOWS` rows and the
three `SYSTEM_MALLOC` rows are decided by macros the frozen table also records as `undef`, so
every one of the eleven has a verdict.

## C line ranges

`docs/port-map.md` pairs **two** references with this ledger - 419 `src/dosfns.c` (798 lines) and
420 `src/dosfns.h` (35 lines, the mirror's header) - and `tools/check-dead-file-coverage.py` binds
both rows here (`src/dosfns.h`'s ledger resolves to this file, since `docs/ports/include-dosfns.md`
does not exist), so the table below is **contiguous**: each row spans its block *and* the blank
separator that follows it, as `docs/ports/msdos.md`'s and `docs/ports/w32inevt.md`'s rows do for
the same reason.  The "C" column is `src/dosfns.c`'s numbering; the header's 35 lines fall inside
it.

| C | block | arm |
| --- | --- | --- |
| 1-19 | the file's opening ownership/title block | dropped (PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 20-22 | C 21's `#include <config.h>` and its two empty neighbours | ungated (`include/config.rs`); C's `AH_BOTTOM` `#include <conf_post.h>` is **not** carried (Deviations 3, Owed 3) |
| 23-25 | `#ifdef MSDOS` and C's "the entire file is within this conditional" note | the file gate; C's note carried as C's own comment |
| 26-34 | `#include <stdio.h>`, the `gettime`/`settime` note, the two `#define`s, `#include <dos.h>` and the two `#undef`s | file gate; C's note carried as C's comment, the rename pair itself has no counterpart (Deviations 2) |
| 35-47 | `lisp.h`, `character.h`, `buffer.h`, `termchar.h`, `frame.h`, `termhooks.h`, `blockinput.h`, `window.h`, `dosfns.h`, `msdos.h`, `dispextern.h`, `coding.h`, `process.h` | file gate; `stdcountof.rs` and `min-max.rs` join them at C 35 (Deviations 9) |
| 48-49 | `#include <dpmi.h>`, `#include <go32.h>` | file gate; both are in-place stand-in sections (P-88 ①, UNVERIFIED) |
| 50-55 | `#include <dirent.h>`, `<sys/vfs.h>`, `<unistd.h>`, `<grp.h>`, `<crt0.h>` | file gate; the first four are mirrors, `<crt0.h>`'s two names stand in-unit (Deviations 7) |
| 56-99 | `Fint86` and its `Sint86` record | file gate |
| 100-123 | `Fdos_memget` | file gate; C's `alloca` is the port's stack pool (Deviations 4) |
| 124-149 | `Fdos_memput` | file gate; same |
| 150-162 | `Fmsdos_set_keyboard` | file gate |
| 163-177 | `#ifndef HAVE_X_WINDOWS` … `Fmsdos_mouse_p` … `#endif` | `not (feature = "have_x_windows")` |
| 178-211 | `Fmsdos_mouse_init`, `Fmsdos_mouse_enable`, `Fmsdos_mouse_disable` | file gate |
| 212-235 | `Finsert_startup_screen` | file gate |
| 236-251 | `dos_country_info` and the `usa_country_info` initializer | file gate; C 236 is a tentative definition and the port spells C's zero (Deviations 6) |
| 252-267 | `#ifndef HAVE_X_WINDOWS` … `dos_windows_version`, `parent_vm_title`, C 255's forward declaration, `restore_parent_vm_title` … `#endif` | `not (feature = "have_x_windows")`; the C 255 declaration is dropped (PORTING.md 0.1-12, P-91 ④) |
| 268-374 | `init_dosfns`, including C 275-279's and C 321-367's conditions | file gate; C 275-279 `no_system_malloc`, C 321-367 `not have_x_windows` |
| 375-481 | `#ifndef HAVE_X_WINDOWS`: `vga_colors`, `msdos_stdcolor_idx`, `msdos_stdcolor_name`, `ms_windows_version`, `w95_set_virtual_machine_title`, `x_set_title` | `not (feature = "have_x_windows")` |
| 482-502 | `Ffile_system_info` | file gate |
| 503-516 | the `ps(1)` banner and `list_system_processes` | file gate |
| 517-630 | `system_process_attributes`, including C 537-539's and C 572-577's `SYSTEM_MALLOC` arms | file gate; both arms of each (P-51) |
| 631-669 | the memory-info banner and `dos_memory_info` | file gate |
| 670-695 | `dos_cleanup`, including C 675-677's condition | file gate; C 675-677 `not have_x_windows` |
| 696-798 | the "Define everything" banner and `syms_of_dosfns` (with C 711-713's and C 738-741's rows) through C 798's `#endif /* MSDOS */` | file gate; C 798 has no counterpart (every item carries the feature) |

The table is contiguous (24 rows, 1-798): the 22 empty lines of `src/dosfns.c` and the 1 empty line
of `src/dosfns.h` are inside the rows above, which is what the coverage gate's two bindings want.

## Subrs (DEFUN) — 10

- [x] Fint86 (C 56)
- [x] Fdos_memget (C 100)
- [x] Fdos_memput (C 124)
- [x] Fmsdos_set_keyboard (C 150)
- [x] Fmsdos_mouse_p (C 167, `not have_x_windows`)
- [x] Fmsdos_mouse_init (C 178)
- [x] Fmsdos_mouse_enable (C 191)
- [x] Fmsdos_mouse_disable (C 203)
- [x] Finsert_startup_screen (C 212)
- [x] Ffile_system_info (C 482)

`defsubr` order in `syms_of_dosfns` (C 702-713) is C's, including C 711-713's gated
`Smsdos_mouse_p` row.

## Functions defined — 22

- [x] Fint86 (C 56)
- [x] Fdos_memget (C 100)
- [x] Fdos_memput (C 124)
- [x] Fmsdos_set_keyboard (C 150)
- [x] Fmsdos_mouse_p (C 163)
- [x] Fmsdos_mouse_init (C 178)
- [x] Fmsdos_mouse_enable (C 191)
- [x] Fmsdos_mouse_disable (C 203)
- [x] Finsert_startup_screen (C 212)
- [x] restore_parent_vm_title (C 257)
- [x] init_dosfns (C 268)
- [x] msdos_stdcolor_idx (C 393)
- [x] msdos_stdcolor_name (C 409)
- [x] ms_windows_version (C 424)
- [x] w95_set_virtual_machine_title (C 433)
- [x] x_set_title (C 459)
- [x] Ffile_system_info (C 482)
- [x] list_system_processes (C 507)
- [x] system_process_attributes (C 517)
- [x] dos_memory_info (C 632)
- [x] dos_cleanup (C 670)
- [x] syms_of_dosfns (C 699)

## Macros defined — 2

- [x] `gettime dos_h_gettime_` (C 29) — not carried: the port declares no `<dos.h>`, so no DJGPP
  prototype is in gnulib's way (Deviations 2).
- [x] `settime dos_h_settime_` (C 30) — same.

## Types and their fields — 0

The file defines no type of its own.  The three records it *uses* from headers this machine has not
got (`union REGS`, `_go32_dpmi_registers`, `_go32_dpmi_meminfo`) plus `<go32.h>`'s
`_go32_info_block` are declared in the unit (P-88 ①) and are **UNVERIFIED** (Owed 5).

## File-scope objects — 5

- [x] `dos_country_info` (C 236) — `#[unsafe(no_mangle)] pub static mut`
- [x] `usa_country_info` (C 237) — `static mut`, the file's own `static`
- [x] `dos_windows_version` (C 253) — `static mut`, inside C 252's condition
- [x] `parent_vm_title` (C 254) — `#[unsafe(no_mangle)] pub static mut`, inside C 252's condition
- [x] `vga_colors` (C 381) — `static mut`, inside C 375's condition

## Evidence

**① The frozen configuration (`build-tu`), before and after.**
`cd femacs && CARGO_TARGET_DIR=/tmp/dosfns-lane/target cargo run -q -p xtask -- build-tu
src/dosfns.c` -> `built src/dosfns.rs -> ../build/femacs/dosfns.o`, **exit 0**, and
`build/femacs/dosfns.o` is **336 bytes, md5 `dbed68837694579185377dbb0821ab62`** — the *same*
reading the 12-line note unit took before the port was written, and the same md5 the
`src/w32inevt.c` lane reads for its own empty object.  Three readings of this door all agree - before the port, after it, and after the citation-number corrections - at **336 bytes, `dbed68837694579185377dbb0821ab62`** each time.  P-86 ⑧ predicts exactly this: every item and
every `include!` but C 21's sits behind the file gate, and an ungated `include!` would break this
compile at once.

**② The arm flip, as a per-file error census (P-86 ③⑴/⑦).**
`cd femacs && CARGO_TARGET_DIR=/tmp/dosfns-lane/target cargo run -q -p xtask -- probe src/dosfns.rs
/tmp/dosfns-lane/arm3.rlib --crate-type=lib -C panic=abort --cfg 'feature="msdos"'` -> **exit 1, 9
errors, 23 warnings**, and **not one of the errors is anchored in this unit's lines**:

| file the primary span is in | count | errors |
| --- | ---: | --- |
| `include/dosfns.rs` | 8 | `E0428 the name ... is defined multiple times` — `dos_country_info` (its 40), `dos_memory_info`, `dos_cleanup`, `syms_of_dosfns`, `init_dosfns`, `msdos_stdcolor_idx`, `msdos_stdcolor_name`, `x_set_title`: every name this unit **defines** and that mirror declares |
| `include/msdos.rs` | 1 | `E0428` — `EINPROGRESS` (its 234) against `lib/include/errno.rs` 92: a mirror-vs-mirror conflict that exists only when the `msdos` gate is on |

So the unit's own text contributes **0 errors** to that reading.  The eight `include/dosfns.rs` rows
are the class `docs/ports/w32inevt.md` Owed 1 already records (a *mirror* file is the primary span,
and the fix is the module + glob-import shape that lets a definition shadow a declaration — the
`include/msdos.rs` 88 `msdos_decls` shape); the ninth belongs to those two mirrors' owed lists.
Owed 1/2 name the rows.

**③ The "0 in the unit" reading is not vacuous, measured from this lane's own three runs.**  The
same door reported **28 errors** for the first draft of this unit, **15 of them anchored in the
unit's own lines** (one syntax error: Rust wants a block where C writes `if (c) switch (…)`;
five `E0425 cannot find value FACE_TTY_DEFAULT_*` — those three constants live in
`dispextern.rs`'s `dispextern_unit_names` module, not at its crate root; three `E0381 used binding
… isn't initialized` on `dos_get_saved_screen`'s three out-parameters; six `E0308` on `AREF`/`ASET`
index types), then **3** after the first fix round (two more `E0308` on a `*vga_colors[i]`
double-dereference, one more syntax row) and **0** after the second.  Each edit removed exactly the
errors its lines carried and nothing else, which is what P-86 ⑦ asks the census to show.  A fresh
*mutation* test (injecting a type error to prove the door reads the text) was **not** run: this
lane's probe budget (≤3) is spent, and the three readings above are the same evidence in the other
order.

**④ The `#if` verdicts.**  `python3 tools/check-cond-eval.py src/dosfns.c` -> **`11 conditions, 0
unknown, 0 disagreements [VERIFIED]`**; `src/dosfns.c` is on the evaluator's verified list
(`docs/baseline/config/cond-eval-verified.txt`), so the table above is a measurement and not a
claim.  Every one of the eleven is also decided by the frozen macro table: `MSDOS`,
`HAVE_X_WINDOWS` and `SYSTEM_MALLOC` are all `undef` in
`docs/baseline/config/darwin-arm64.macros.tsv` (2849, and the two others' rows).

**⑤ The text gates.**  `python3 tools/extract-abi.py --check emacs/src/dosfns.c
src/dosfns.rs` -> **`OK …`**, zero problem lines *and* the P-86 ② vacuous pass: `MSDOS` is
undef, so `tools/cinventory.py`'s live-line filter gives the C side an **empty checklist** and the
range table above is that clause's substitute (P-86 ②, ⑩).  `python3 tools/check-declarations.py`
-> **`17 declaration sites (0 forbidden, 17 allowed literals), 0 failures`**.
`python3 tools/check-citations.py` -> **`676 file(s) scanned, 1 drift, 0 range, 1779 external, 36
allowed, 0 stale`**, and the one `drift` row is **not this unit's**:
`src/sfntfont-android.rs:823` cites `font.h 682-698` for `harfbuzz`, which is at `font.h`
74 (another lane's in-flight file; `src/dosfns.rs` has **0 drift** and 0 `range`).  This
unit's citations name `include/config.rs`, `lisp.h`, `dispextern.h`, `unistd.h`, `string.h`,
`vm-limit.c` and the DJGPP headers, the last of which resolve as `external` (the tree cannot be
asked about a header it does not carry).  `python3 tools/check-mirror-dupes.py` -> **`18
duplicate declaration(s), 17 ratchet row(s), 1 failure(s)`**, and the failure is again **not this
unit's**: `src/ftcrfont.rs` duplicates `be_use_subpixel_antialiasing` from
`src/include/haiku_support.rs` (no `dosfns` row appears in either reading).  `python3 tools/check-dead-file-coverage.py` and
`python3 tools/check-ledger-ticks.py` bind the two tables/sections above.

**⑤a The reading `--check` cannot take, taken by hand.**  Because `MSDOS` is undef, the C side's
checklist is empty, so `--check` compares no subr name, no arity, no intspec and no docstring for
this file.  The three comparisons P-86's class therefore owes were run as a one-off against the
reference text: **10/10 `DEFUN` headers identical** (Lisp name, `F` name, `S` name, `min_args`,
`max_args` and `intspec`), **11/11 `DEFVAR_*` rows identical** (Lisp name and C variable) and
**21/21 docstrings byte-identical** after C's `/* */` delimiters are stripped (`doc: /* X  */` ->
`X`, the two spaces kept) - including the three `DEFVAR_LISP` rows whose docstrings
`--check` would normally compare through `docs/baseline/subrs.tsv`.  The one-off script is not in
this lane's writable set; the reader can re-derive it from this ledger's two ranges.

**⑥ What is **not** a reading.**  No DJGPP compile, no DOS, no BIOS/DOS interrupt, no memory
transfer, no `dos_get_saved_screen`, no `statfs`, no window title, nothing.  The three `probe`
readings are `rustc` on this machine with one `--cfg`; the frozen reading is an *empty* object, so
it measures the gates and not the code.

## Deviations

1. **The opening block is dropped, not carried.**  C 1-19 (title, authors, and the product's
   version/ownership prose) is removed whole per PORTING.md 0.1-19; the port's first line is the
   `// femacs: port of src/dosfns.c @ fd7afced403.` declaration.  The file's two author lines die
   with it: C writes them *inside* that block (C 1-2), so there is no separate comment to keep
   (unlike `src/w32inevt.c`, whose author lines sit below its block).
2. **C 29-33's `gettime`/`settime` rename pair has no counterpart.**  Both `#define`s and both
   `#undef`s exist to move `<dos.h>`'s two prototypes out of gnulib's way while that header is
   read; the port declares no `<dos.h>` and therefore no gnulib-clashing name, so the four lines
   are C's text in a comment only (the `src/msdos.rs` 171-174 reading of the identical four lines
   in the sibling file).  The two macros are listed unticked under `## Macros defined`.
3. **C's `AH_BOTTOM` `#include <conf_post.h>` is not carried.**  No name this unit uses comes from
   it, and carrying it is a *measured* loss under this gate: the mirror's three MS-DOS overrides
   (`conf_post.rs` 318-320: `_GL_DT_NOTDIR`, `ENOTSUP`, `EOPNOTSUPP`) collide with
   `lib/include/dirent.rs` 32 and `lib/include/errno.rs` 66/76 (`E0428`, three rows), so the first
   probe read 28 errors instead of 9 for a header this file never mentions.  The sibling
   `src/msdos.rs` 92 omits the same edge.  Owed 3.
4. **C's `alloca` is the port's stack pool (P-05).**  Two sites (C 115, C 138) become
   `sa_stack_alloc (len as usize)`, whose release the port does not write — C accounts neither
   request against `sa_avail` and neither does the port (the `src/msdos.rs` 1749 shape).
5. **Four chained assignments are written out.**  C 289, C 326, C 356 and C 446 assign three
   fields in one statement; the port writes three stores each, innermost first, so the source order
   of the effects is C's (the `src/msdos.rs` 2998 / `src/haikuterm.rs` 1255 shape).
6. **Zero-initialization where C declares without an initializer (PORTING.md 0.1-13).**  The two
   `union REGS` blocks (C 77), `init_dosfns`'s `regs`/`dpmiregs` (C 271-272), `w95_set_virtual_machine_title`'s
   `regs` (C 440), `Ffile_system_info`'s `stfs` (C 486), `dos_memory_info`'s `info` (C 636),
   `system_process_attributes`'s `cmd` (C 531) and `dos_get_saved_screen`'s three out-parameters
   (C 217-218: Rust cannot take `&raw mut` of an uninitialized binding, so they take C's zero first).
   C 236's `dos_country_info[34]` is a tentative definition and gets the same treatment.
7. **`<crt0.h>`'s two names stand in the unit.**  `__crt0_argc`/`__crt0_argv` (C 573, C 588, C 596…)
   have no mirror and no tree on this machine; they are declared in the unit's `<crt0.h>` section
   (P-88 ①) and are **UNVERIFIED**.  `<dirent.h>`'s `__opendir_flags`/`__OPENDIR_FIND_HIDDEN`
   (C 372) are the same case — the tree's `<dirent.h>` mirror is gnulib's and does not carry the
   DJGPP pair.
8. **Five `<string.h>`/`<unistd.h>`/`vm-limit.c` names are declared at their use sites** (P-88 ①):
   `basename`, `strpbrk`, `getlogin`, `sbrk`, `is_tty_frame`, plus `get_lim_data`/`ret_lim_data`,
   because no mirror in this unit's graph carries them (`lib-src/pop.rs` 314, `nt/cmdproxy.rs` 320
   and `src/gmalloc.rs` 1807-1810 are the tree's shapes for the same three gaps).
9. **Two mirrors join C 35's include list.**  `lib/include/stdcountof.rs` (`countof`, used at
   C 398/416) and `lib/include/min-max.rs` (`max`, used at C 655) are `lisp.h` 28 and 69-73's
   macros; neither is on `lisp.rs`'s own edge list, so the unit spells them (the `src/msdos.rs`
   146 and `src/w32console.rs` 346 readings).
10. **`<sys/vfs.h>`'s pair comes from the tree's `<sys/mount.h>` mirror.**  C 486-493's `struct
    statfs` and `statfs` are declared by `lib/include/sys_mount.rs` (its C 55-98), which is the
    frozen target's own spelling of the same pair, so neither is repeated in the unit.
11. **Three constants are addressed through their module.**  `FACE_TTY_DEFAULT_FG_COLOR`,
    `FACE_TTY_DEFAULT_BG_COLOR` and `FACE_TTY_DEFAULT_COLOR` (C 403-405, 412-415) and
    `unspecified_fg`/`unspecified_bg` (C 403/404, 413/414) live in `include/dispextern.rs`'s
    `dispextern_unit_names` module, not at its crate root, so the port qualifies them there (the
    `src/ftfont.rs` 1493 `dispextern_externs::xstrcasecmp` shape for the sibling module); the two
    `char[]`-as-`char *` uses go through `std::ptr::addr_of!` (`src/frame.rs` 4071-4075).
12. **`msdos_stdcolor_idx`'s return conversion is explicit.**  C's ternary yields the three
    `unsigned long` constants into an `int` return; the port writes `as c_int` at each arm
    (C's own implicit conversion).
13. **C's `char` is signed on DJGPP; the port keeps `c_char`.**  `buf[i]` (C 119) feeds
    `make_fixnum`, so the sign is observable; `c_char` is `i8` on some targets and `u8` on others,
    and this machine has no DJGPP dictionary to pick from.  Recorded rather than papered over.
14. **`dosmemput`'s return type disagrees with the sibling unit.**  This unit declares
    `void dosmemput (const void *, int, int)` from `<go32.h>` (C calls it and ignores the result);
    `src/msdos.rs` 603 spells the same name `-> c_int`.  Both are UNVERIFIED; Owed 7.
15. **`if (cflag) switch (…)` needs a block in Rust.**  C's `if` takes the `switch` statement
    directly (C 306-319); Rust's `if` requires a block, so the `match` is wrapped in one.  The
    `else` stays attached to the same `if`.
16. **Parameter rebindings.**  C 470-471 reassign `x_set_title`'s `name` and C 490 reassigns
    `Ffile_system_info`'s `filename`; DEFUN-less and DEFUN'd signatures alike take a same-named
    `let mut` (the `src/androidfns.rs` 581 shape).
17. **C 255's forward declaration is dropped** (PORTING.md 0.1-12): `w95_set_virtual_machine_title`
    has its definition at C 433-451 in the same condition, which is where the port carries it
    (P-91 ④'s allowed `static`-forward class does not arise here — the definition is in this
    batch).

## Owed (nothing here is claimed as done)

1. **Eight `include/dosfns.rs` rows: the mirror's declarations against this unit's definitions.**
   `include/dosfns.rs` 40/44/45/46/47/56/60/63 declare `dos_country_info`, `dos_memory_info`,
   `dos_cleanup`, `syms_of_dosfns`, `init_dosfns`, `msdos_stdcolor_idx`, `msdos_stdcolor_name` and
   `x_set_title` at the mirror's top level, and this unit defines all eight, so the `msdos` arm
   reads eight `E0428`s anchored in the mirror (Evidence ②).  The fix is the move the other
   mirrors already made (`include/msdos.rs` 88's `msdos_decls`, `include/w32inevt.rs`'s owed row):
   put the eight in a module the unit glob-imports, so the definition shadows the declaration —
   which is C's prototype + definition split.  `docs/ports/w32inevt.md` Owed 1 is the same row for
   its own three names.  **`src/include/**` is not this lane's writable set.**
2. **One mirror-vs-mirror row: `include/msdos.rs` 234's `EINPROGRESS 112` against
   `lib/include/errno.rs` 92's 36.**  The `msdos` mirror's own note says its MS-DOS overrides carry
   the feature "rather than duplicated unmarked"; under the gate they *are* duplicated, because
   `errno.rs` declares the frozen target's value unconditionally.  The arm reads one `E0428`
   (Evidence ②) and the two mirrors' owners should reconcile it the way `include/msdos.rs` 34
   describes.
3. **`conf_post.rs`'s three MS-DOS overrides collide with `errno.rs`/`dirent.rs` when `msdos` is
   on.**  Measured (Deviations 3): the same three rows cost the first probe 3 errors for a header
   this unit does not name.  A unit that *does* carry C's `AH_BOTTOM` edge cannot compile the
   `msdos` arm until the owner reconciles them.
4. **`docs/port-map.md` 419's port count.**  The row still reads `src/dosfns.rs | 12`; this
   port is 1218 lines.  The regeneration is the integrator's (`--write-pending`), and
   `docs/port-map.md` is not this lane's writable set.  `docs/ports/line-drift-tolerance.tsv` (the
   row that explains a ratio this far above 1) is the same kind of follow-up and the same
   non-writable file.
5. **No DJGPP toolchain, so every stand-in item is UNVERIFIED**: `union REGS` (both views, the
   member order and the record's size), `_go32_dpmi_registers`, `_go32_dpmi_meminfo`,
   `_go32_info_block`, `__tb`, `__opendir_flags`, `__OPENDIR_FIND_HIDDEN`, `__crt0_argc`,
   `__crt0_argv`, `int86`, `intdos`, `delay`, `dosmemget`, `dosmemput`,
   `_go32_dpmi_simulate_int`, `_go32_dpmi_get_free_memory_information`.  The layout assertions the
   other SDK stand-ins carry are owed with them; Owed 6 names the one value the port had to choose.
   *A future DJGPP arm flip is the only thing that can close this.*
6. **`__OPENDIR_FIND_HIDDEN`'s value.**  `<dirent.h>` is not in the tree and not on this machine;
   the constant carries the DJGPP header's documented `0x0001` and is **not a reading** (C 372's
   semantics — "see hidden files" — are the only thing the port can be sure of).
7. **`dosmemput`'s return type against `src/msdos.rs` 603** (Deviations 14): the two units'
   excerpts disagree and only the DJGPP header settles it.
8. **`src/include/dosfns.rs` 27-29's claim is now stale.**  That mirror's note says
   "`src/dosfns.c`'s port is the empty shape `src/dosfns.rs` records and no unit `include!`s
   this mirror yet"; this unit includes it at C 43 and is no longer the empty shape.  The mirror is
   not this lane's writable set (`src/include/**`), so the correction is owed.
9. **The Intel x86 `char` sign (Deviations 13)** — a DJGPP-only question, owed with the toolchain.

## Not claimed

Every line of this unit sits behind a gate the frozen configuration leaves false: Evidence ① is an
empty object, and ②-⑥ are compile-level or textual.  **Nothing here was executed** — no `int86`,
no `intdos`, no `_go32_dpmi_simulate_int`, no BIOS or DOS call, no `dosmemget`/`dosmemput`, no
mouse, no screen, no country information block, no `statfs`, no task-manager attribute, no window
title, no `__opendir_flags` store.  There is **no DJGPP-target compile and no layout measurement**:
the arm flip is `rustc` on this machine with one `--cfg`, and its nine errors are **all** in mirror
copies (Evidence ②).  The file is **transcribed (C 1-798)**, and "transcribed" is not "compiled",
"measured" or "working": the two owed mirror rows (Owed 1/2), the nine owed items above and the
absence of a DJGPP toolchain are what stand between this text and a green `msdos` arm.  The `msdos`
arm does not compile today, and that is the mirror graph's state, not this unit's.

## Box audit: the two `dos.h` macros (2026-10-08, status 3.954)

`## Macros defined`'s two rows (`gettime dos_h_gettime_`, C 29 and `settime dos_h_settime_`, C 30)
were unchecked rows that already state their answer: this unit declares no `<dos.h>`, so no DJGPP
prototype is in gnulib's way, and the port carries neither (Deviations 2, which the rows cite).
They are decisions with their evidence in the row itself, so they are ticked here rather than left
to look like pending work; `--check` for `src/dosfns.c` was already `OK` before the tick.
