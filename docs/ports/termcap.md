# emacs/src/termcap.c (640 lines)

> **P-86's class: the frozen configuration compiles nothing here.**  The file's own C 18-19 says
> it is used by the MS-DOS port alone, and the build system agrees: `configure.ac` 6179-6183 sets
> `TERMCAP_OBJ=terminfo.o` when TERMINFO is yes and `tparam.o` otherwise, so `termcap.o` is never
> in it, and only the MS-DOS build adds it (`emacs/msdos/sed1v2.inp` 91 rewrites the generated
> Makefile's empty `TERMCAP_OBJ` to `termcap.o tparam.o`; `emacs/src/Makefile.in` 194 and 308
> record the rule, 485 lists the object in `SOME_MACHINE_OBJECTS`, which feeds the DOC file
> rather than the binary).  `docs/baseline/frozen-build-sources.txt` therefore lists no
> `termcap.o`, `tools/extract-abi.py`'s checklist for the file is empty (P-86 ②) and this
> **C line-range table** - with `## Evidence`, the gate table and `## Deviations` - is that
> clause's substitute.  What the earlier revision of this ledger measured (the exclusion, the empty
> checklist, the 336-byte object) is unchanged; what P-86 ① replaces is its *conclusion*, "the
> empty unit plus the note that names the gate", with **transcribe the text, both arms, no stub**
> (the same replacement `docs/ports/msdos.md`, `w32.md`, `w32proc.md` and `androidvfs.md` record).
>
> Port: `femacs/src/termcap.rs` (**883 lines**, 35,922 bytes, md5 `b581da3898fbce63a4413d46ff2f986a`),
> carrying **C 1-640** - every line of the reference - with both arms of every `#if`.

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 12

## Macros defined — 2

## Types and their fields — 1 + 1

## File-scope objects — 4

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself (the MS-DOS build's `TERMCAP_OBJ`, i.e. C 32's `#ifdef MSDOS`) | `feature = "msdos"` (on every item *and* on C 23-34's include graph) | the MS-DOS (djgpp) build |
| C 43 `#ifndef BUFSIZE` | `not (feature = "bufsize")` (the `#else` side is the build's own `-DBUFSIZE=…`, which has no text) | `BUFSIZE` is not defined by the build |
| C 44 `#ifdef DEBUG` | `feature = "debug"` (registered, `feature-coverage.tsv`; first used by `tparam.rs` 353) | a debugging build |
| C 44-47's `#define BUFSIZE bufsize` + `int bufsize = 128;` | the `debug` arm's object, and C 425 (the one use site) reads it | `DEBUG` and no `-DBUFSIZE` |
| C 53 `#ifndef TERMCAP_FILE` | `not (feature = "termcap_file")` (same shape) | `TERMCAP_FILE` is not defined by the build |
| C 111 `#ifdef IS_EBCDIC_HOST` … C 121 `#else` | `feature = "is_ebcdic_host"` / `not (…)` (**not registered** - Owed 3) | an EBCDIC host / everything else |
| C 193 `#ifdef IS_EBCDIC_HOST` … C 200 `#else` (inside `tgetst1`) | the same feature, both arms carried | each side |
| C 326 `#ifdef MSDOS` (inside `valid_filename_p`) | the file gate's own condition; C 328-330's `#else` is unreachable and is quoted (Deviations 6) | the file is compiled only when `MSDOS` is defined |
| C 359 `#ifdef INTERNAL_TERMINAL` | `feature = "internal_terminal"` (**not registered** - Owed 3) | the MS-DOS build, which defines it (`emacs/msdos/sed2v2.inp` 91) |
| C 383 `#if defined (MSDOS) && !defined (TEST)` | `not (feature = "test")` (nested in the file gate; `test` is registered) | every build but a `TEST` compile |

## C line ranges

The rows ascend and cover every non-blank line of the reference (the reference's own blank lines,
including its three form-feed page breaks at C 57/263/304, need no row).  `docs/port-map.md` binds
this ledger to `src/termcap.c` alone - the reference has no `src/termcap.h` and no `lib/termcap.h`
(`ls emacs/src/termcap.h emacs/lib/termcap.h` is empty), so there is no mirror to confirm and the
termcap surface comes from `tparam.h`'s mirror (see `## Decisions`).

| C | block | arm |
| --- | --- | --- |
| 1-16 | the file's opening declaration block | dropped (P-86 ①; the port opens with the `// femacs: port of …` line) |
| 18-19 | the file's own note that it is used by the MS-DOS port alone | carried as the port's comment, with the two occurrences of the program's own name left as placeholders (0.1-19's second half, the `emacs.rs` `RCS_Id` shape) |
| 21 | `#include <config.h>` | ungated (every unit includes it) |
| 23-34 | `<stdlib.h>`, `<sys/file.h>`, `<fcntl.h>`, `<unistd.h>`, `<intprops.h>`, `"lisp.h"`, `"tparam.h"`, and C 32-34's `#ifdef MSDOS` / `"msdos.h"` / `#endif` | file gate; `<sys/file.h>` has no mirror and contributes `O_RDONLY` through `fcntl.in.rs`; the `atoi` and `<string.h>` chain edges are declared/spelled in the unit (Findings 3/4) |
| 36-41 | the `BUFSIZE` comment | file gate (C's comment, carried) |
| 43-51 | `#ifndef BUFSIZE` / `#ifdef DEBUG` / `#define BUFSIZE bufsize` / `int bufsize = 128;` / `#else` / `#define BUFSIZE 2048` / `#endif` / `#endif` | `not (feature = "bufsize")` and, inside it, `debug` vs `not (feature = "debug")`: C 49's `2048` is the live arm and is the `BUFSIZE` constant, C 45/47's `bufsize` arm is the object + C 425's use site (two arms of one C name cannot be two Rust items) |
| 53-55 | `#ifndef TERMCAP_FILE` / `#define TERMCAP_FILE "/etc/termcap"` / `#endif` | `not (feature = "termcap_file")`; the constant is a `&CStr` (Findings 5) |
| 58-64 | the "looking up capabilities" comment, `static char *term_entry;`, C 64's `static char *tgetst1 (…)` prototype | file gate; the prototype is dropped (0.1-12, Deviation 1) |
| 66-79 | `find_capability` | file gate; C's `for (; *bp; bp++)` increment point kept; `register` dropped (Deviation 2) |
| 81-88 | `tgetnum` | file gate |
| 90-95 | `tgetflag` | file gate; C's `ptr && ptr[-1] == ':'` is a `bool as c_int` |
| 97-109 | the string-capability comment and `tgetstr` | file gate |
| 111-131 | the two `static const char esctab[]` tables (`#ifdef IS_EBCDIC_HOST` and its `#else`) | `is_ebcdic_host` / `not (…)`, both arms; the elements are spelled byte by byte (Deviation 3) |
| 133-262 | `tgetst1` (C 193-199 + C 200-206 the EBCDIC/else pair inside it, C 221-255 the `%pN` pass, C 224 the anonymous `struct` for `cut[11]`) | file gate + `is_ebcdic_host` on the first arm; the anonymous element type is named `tgetst1_cut` (Deviation 9) |
| 264-266 | the "outputting a string with padding" comment and `char PC;` | file gate; `PC` is a definition here (`tparam.rs`'s mirror declares it; the shadowing shape P-21/P-39) |
| 268-303 | `tputs` (C 297-299 the `intmax_t padfactor` declaration and C 298's `ckd_mul`) | file gate; `ckd_mul` is the `overflowing_mul` pair (P-15, `token-aliases.tsv`), C 301-302's padding loop is the same `(*outfun) (PC)` call |
| 305-314 | the "finding the termcap entry" comment and `struct termcap_buffer` | file gate (`#[repr(C)]`, fields in C's order) |
| 316-321 | "Forward declarations of static functions" and the four prototypes | file gate; prototypes dropped (0.1-12, Deviation 1) |
| 323-331 | `valid_filename_p` with its `#ifdef MSDOS` / `#else` pair | file gate; C 326-327's arm is the file gate's own text, C 328-330's `#else` is unreachable and quoted (Deviation 6) |
| 333-342 | the `tgetent` comment | file gate (C's comment, carried) |
| 344-496 | `tgetent` (C 359-372 the `#ifdef INTERNAL_TERMINAL` arm, C 371 and C 407 the two `goto ret`, C 383-387 the `MSDOS && !TEST` arm, C 425 the `BUFSIZE` use site, C 463-467 the realloc bookkeeping, C 472-479 the line copy, C 493-495 the `ret:` label's own statements) | file gate + `internal_terminal` + `not (feature = "test")`; the forward `goto` pair is the port's labelled block (PORTING.md 2.2, Deviation 7) |
| 498-502 | the `scan_file` comment | file gate (C's comment, carried) |
| 504-536 | `scan_file` (C 520-525 the `do … while` continuation loop, C 526's `end[-2]`, C 528-530 the match test, C 533 the discard) | file gate; C's `return 1`/`return 0` are the port's `true`/`false` (`macro-body-notes.tsv` row) |
| 538-539 | the `name_match` comment | file gate (C's comment, carried) |
| 541-554 | `name_match` | file gate |
| 556-581 | `compare_contin` (C 563-568 the continuation skip with its `while (1)`) | file gate; C's `while (1)` is the port's `loop` |
| 583-594 | the `gobble_line` comment | file gate (C's comment, carried) |
| 596-640 | `gobble_line` (C 603-604 the null `append_end`, C 608-611 the line scan and its `return`, C 614-627 the grow path with C 622's `xpalloc`, C 628-633 the discard path with C 631's `full -=` inside `memcpy`, C 634-637 the `read` and the NUL store) | file gate |

## Decisions

- **The file gate is the MS-DOS build, and P-86 ⑧ applies to the include graph too.**  `MSDOS` is
  what makes this file a translation unit at all (C 18-19, the build files quoted above), so every
  item and every `include!` carries `#[cfg (feature = "msdos")]`; the frozen arm therefore compiles
  the unit down to `include/config.rs` and the `#![allow]` attribute and produces the **336-byte
  empty object** the criterion names (Evidence ①).  The gate is *not* C's own text - the body has
  no `#ifdef MSDOS` around it - so the authority is recorded here rather than in the file.
- **The termcap surface is not a third-party header, and nothing is declared twice.**  `tparam.h`
  (C 31) declares `tputs`/`tgetent`/`tgetflag`/`tgetnum`/`tgetstr`/`tparam`/`tgoto` and the globals
  `PC`/`BC`/`UP`, and its mirror `include/tparam.rs` is this file's import; this unit *defines* the
  five entry points and `PC`, which is the shadowing shape P-21/P-39 sanctions (the mirror's
  declarations are glob-imported, a same-named item in the unit takes over).  `PC`'s definition is
  also C's: `terminfo.c`'s live text defines it only when the terminfo library does not
  (`TERMINFO_DEFINES_BC`), and an MS-DOS build is exactly the configuration that compiles this file
  instead of `terminfo.c`.
- **Two `#ifndef` knobs are constants with a fallback, and the `DEBUG` arm of one of them cannot be
  a second `BUFSIZE`.**  C 49's `2048` and C 54's `"/etc/termcap"` are what the frozen
  configuration would use (nothing in the reference tree defines either - `configure.ac` has no
  `TERMCAP_FILE` row), so they are `pub const`s under `not (feature = "bufsize")` /
  `not (feature = "termcap_file")` (the `sound.rs` `DEFAULT_ALSA_SOUND_DEVICE` shape).  C 45's
  `#define BUFSIZE bufsize` turns the name into a *read* of C 47's global, so that arm keeps the
  object (`pub static mut bufsize`) and C 425 - the single use site - spells the read under
  `debug`; `E0428` forbids two items of one name, and `extract-abi --check` requires the `BUFSIZE`
  *constant* to carry C 49's value.
- **The budget.**  C 640 lines against the port's 883 (38.0%).  The per-item `#[cfg]` gates (both
  arms carried), Rust's statement-per-line spelling of C's pointer walks and the two in-unit
  chain-edge declarations are the whole of it; the tolerance row is owed (Owed 2), not written by
  this lane.
- **Call-relationship differences (D7).**  None beyond the declarations Findings 3/4/5 name.

## Findings

1. **The `#ifdef INTERNAL_TERMINAL` arm is *live* in the only configuration that compiles this
   file.**  `emacs/msdos/sed2v2.inp` 91 replaces the generated `#undef INTERNAL_TERMINAL` with
   `#define INTERNAL_TERMINAL "pc|bios|IBM PC with color display::co#80:li#25:Co#16:pa#256:km:ms:
   cm=<CM>:cl=<CL>:ce=<CE>::se=</SO>:so=<SO>:us=<UL>:ue=</UL>:md=<BD>:mh=<DIM>:mb=<BL>:mr=<RV>:
   me=<NV>::AB=<BG %d>:AF=<FG %d>:op=<DefC>:"`, and the same sed file defines `MSDOS` (26-28); the
   frozen table's `undef` (`docs/baseline/config/darwin-arm64.macros.tsv` 1869) is the *frozen*
   target's value, not the file's own configuration.  So C 359-373 is not dead text: it is the arm
   an MS-DOS build takes, it is carried as real code (`#[cfg (all (feature = "msdos",
   feature = "internal_terminal"))]`), and its macro's value is the in-unit declaration measured
   from that sed line (no header in the reference tree declares `INTERNAL_TERMINAL`;
   `configure.ac` 6454 only gives the `AH_TEMPLATE` text).  The arm compiles (Evidence ③).
2. **`valid_filename_p`'s `#else` arm can never be reached by a configuration that compiles the
   file.**  C 326's `#ifdef MSDOS` is the same condition as the file gate (Finding 1), so the arm C
   326-327 spells is the file gate's own text and C 328-330 is unreachable; carrying it as a second
   item would either gate on `all (feature = "msdos", not (feature = "msdos"))` or compile in the
   frozen arm and put a symbol into the object that must stay empty.  Its text is quoted
   (Deviation 6).
3. **`lisp.h` 30's `<string.h>` edge is missing from the `lisp.rs` chain.**  `strcmp`, `strcpy` and
   `memcpy` (C 398/406/440/472-478/631) are undefined until the unit spells the edge
   (`include!("../lib/include/string.rs")`, the `gfilenotify.rs` 48 precedent); the mirror round
   owes the chain edge (Owed 5).  A direct `rustc` of the unit without it reported exactly those
   four call sites (the same class the first arm flip measured).
4. **`atoi` (C 87) has no home in `lib/include/stdlib.rs`**, which carries `getenv`, `strtol`,
   `strtod`, `strtoul` and the allocator family.  Declared in the unit (P-88 ①, the `font.rs` 386
   and `tparam.rs` 35 shape); its home is that mirror (Owed 4).
5. **A C string macro must be spelled as a bare Rust literal, not `c"…".as_ptr ()`.**  The first
   draft wrote `pub const TERMCAP_FILE: *const c_char = c"/etc/termcap".as_ptr ();` and
   `tools/extract-abi.py` reported `constant TERMCAP_FILE: C "/etc/termcap" vs Rust
   c"/etc/termcap".as_ptr () (different bytes)`: the constants rule hands the whole value text to
   `rust_string_bytes`, which reads a literal and nothing else.  The passing spellings are the bare
   literal ones (`&CStr`, the `alloc.rs` 5497 / `emacs.rs` 359 shape); the use site (C 417) then
   takes `.as_ptr ()` and C's `(char *)` cast.  This is the same class as the `sound.rs` 1293
   `DEFAULT_ALSA_SOUND_DEVICE` row, which the ledger's own `--check` reading may not have covered
   (`alloc.rs`/`emacs.rs` are the units that prove the accepted form).
6. **The msdos.h mirror blocks the arm flip, in two ways** (Evidence ②/③, Owed 1/6): including
   `include/msdos.rs` together with `fcntl.in.rs`/`errno.rs` under `feature = "msdos"` is
   `E0428` on `O_CLOEXEC` and `EINPROGRESS` - the mirror's *overrides* carry that cfg while the
   portable mirrors own the names unconditionally, the same reconciliation `docs/ports/msdos.md`
   Owed 2 already records - and the mirror's own chain misses `character.h` (MAX_CHAR,
   CHAR_VALID_P, CHARACTERP, STRING_CHAR, UNIBYTE_TO_CHAR, BYTES_BY_CHAR_HEAD,
   `string_char_and_length`, `raw_prev_char_len`) and `systime.h` (`Time`), which its
   `termchar.rs`/`buffer.rs`/`dispextern.rs`/`termhooks.rs` copies name.
7. **`extract-abi --check` stops being vacuous for this file.**  `empty_unbuilt` (extract-abi
   1362) exempts only a port with *no* `fn`; this unit has twelve, so the full comparison - the
   twelve bodies, the two constants, the stub rule, the `no_mangle` rule and the line-count check -
   applies, and two of its rows needed the machine-readable baseline: `tgetent`'s `goto` and
   `scan_file`'s `1` (Owed 7, both written by this lane).

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/termcap-lane cargo run -q -p xtask -- build-tu src/termcap.c`
→ `built src/termcap.rs -> ../build/femacs/termcap.o`, **exit 0**, and `build/femacs/termcap.o` is
**336 bytes** with no exported symbol (`nm` shows one local `ltmp0` only) - the empty object P-86 ⑧
names as the criterion for "the gate reaches the include graph".  The same file compiled by hand
without any `--cfg` (`rustc --edition 2024 --crate-type lib --emit obj -C panic=abort`) is 336
bytes too.  **This object is the port's frozen-arm object, not the reference's**: the reference
never compiles this file for this target (the build files above), so there is no C object to
compare against - the `build-tu` reading says "the port is empty here", not "the port matches C".

**② The arm flip, as a per-file error census (P-86 ③⑴/⑦).**
`cd femacs && CARGO_TARGET_DIR=/tmp/termcap-lane cargo run -q -p xtask -- probe src/termcap.rs
/tmp/termcap-lane/x.rlib --crate-type=lib -C panic=abort --cfg 'feature="msdos"'`
→ **exit 1, 17 errors** (plus the abort line), and the census by file - re-measured on xtask's own
materialization (`build/probes/termcap/_root.rs`) with the pinned toolchain from `femacs/` - is:

| file | errors |
| --- | ---: |
| `..._src_include_buffer_rs.rs` | 8 |
| `..._src_include_dispextern_rs.rs` | 4 |
| `..._src_include_termhooks_rs.rs` | 3 |
| `..._src_include_msdos_rs.rs` | 2 |
| **`_root.rs` (this unit's own text)** | **0** |

Named: 2 × `E0428` (`include/msdos.rs` 232 `EINPROGRESS` vs `errno.rs` 92, 241 `O_CLOEXEC` vs
`fcntl.in.rs` 118) and 15 × `E0425` (`MAX_CHAR` ×2, `Time` ×3, `CHAR_VALID_P`, `CHARACTERP`,
`STRING_CHAR` ×2, `UNIBYTE_TO_CHAR` ×2, `string_char_and_length` ×2, `BYTES_BY_CHAR_HEAD`,
`raw_prev_char_len`) - all of them in mirror copies, none in the unit's own text (Finding 6).  13
warnings accompany them (the house classes: `unused doc comment`, `unreachable expression`).

**③ The gap-stub arm, and the mutation test that proves it is not an empty reading (P-86 ⑦).**
Because the real flip is blocked by the msdos.h mirror (Finding 6), the missing *edges* were
restored in a **/tmp copy** of the materialization (`include/msdos.rs`'s two overridden constants
commented out; `include/character.rs` and `systime.h`'s `Time` added as the edges that mirror's C
chain supplies) and the same `rustc` command re-run:

| `--cfg` set | exit | errors | object |
| --- | ---: | ---: | ---: |
| `msdos` | 0 | 0 | 14,089,984 bytes |
| `msdos`, `internal_terminal` (the MS-DOS build's own pair, Finding 1) | 0 | 0 | 14,092,880 |
| `msdos`, `is_ebcdic_host` | 0 | 0 | 14,089,584 |
| `msdos`, `debug` (the `bufsize` arm) | 0 | 0 | 14,091,544 |
| `msdos`, `internal_terminal`, `debug`, `is_ebcdic_host` | 0 | 0 | 14,094,128 |
| `msdos`, `bufsize` | **1** | 1 `E0425 BUFSIZE` | - |
| `msdos`, `termcap_file` | **1** | 1 `E0425 TERMCAP_FILE` | - |

The byte counts are one reading each and are **not** reproducible to the byte: the materialization
directory is part of what is compiled (its path reaches the object), so the same unit text
materialized in a second `/tmp` directory differs by a few hundred bytes - re-measured on the final
revision as 14,089,712 for the same `msdos` row.  The exit status and the error counts are what
repeat.

**Mutation (the reach proof)**: appending `fn _termcap_lane_mutation () -> c_int { let _x: c_int =
c"mutation"; 0 }` to the *unit's own text* in the same tree turns the `msdos` reading into **exit 1
with `error[E0308]: mismatched types` in `_root.rs`** (13 warnings), so the compile really reaches
this unit's statements - it is not the "the include graph broke first" reading P-86 ⑦ warns about.
The last two rows are the named hole: the `bufsize`/`termcap_file` arms are C's *build-supplied*
macros and have no text in the port (the `sound.rs` `default_alsa_sound_device` class).

**④ The ABI gate.**  `python3 tools/extract-abi.py --check emacs/src/termcap.c
femacs/src/termcap.rs` → **one `- ` row, the line-count drift** (`line count drifted 38.0%: C 640
vs Rust 883`), and nothing else: the first run's other three rows are fixed (the `TERMCAP_FILE`
bytes → `&CStr`, Finding 5; `tgetent`'s `goto` and `scan_file`'s `1` → the two
`docs/ports/macro-body-notes.tsv` rows Owed 7 names).  The drift row is the tolerance row the main
agent writes (Owed 2).

**⑤ The four static gates, on this revision.**
`python3 tools/check-mirror-dupes.py` → **17 duplicate declaration(s), 17 ratchet row(s), 0
failure(s)** - no row names `femacs/src/termcap.rs`.
`python3 tools/check-declarations.py` → **0 failures** (10 allowed literals tree-wide, none here).
`python3 tools/check-citations.py` → **676 files scanned, 2 drift, 0 range, 0 stale**; the two drift
rows are `femacs/src/ftfont.rs` 65/66 (another lane's, pre-existing), this unit contributes none.
`python3 tools/check-dead-file-coverage.py` → **0 failures**; this ledger is one of the bound ones
(28 range rows covering C 1-640, the same reading the table's own line carries).
`python3 tools/check-cond-eval.py src/termcap.c` → `9 conditions, 0 unknown, 0 disagreements
[VERIFIED]`: every branch this table lists is decided by the frozen table, and seven of its nine
sites are false there (`MSDOS` at C 32/326/383, `DEBUG` at C 44, `IS_EBCDIC_HOST` at C 111/193,
`INTERNAL_TERMINAL` at C 359); the two `#ifndef` knobs (C 43/53) are true, so their arms are C's
own fallbacks.

## Deviations

1. **C's static forward declarations are dropped** (C 64's `tgetst1` and C 318-321's `scan_file`,
   `gobble_line`, `compare_contin`, `name_match`).  Rust has no declaration/definition pair: an
   item orders itself, and a second item of one name is `E0428` (PORTING.md 0.1-12), so each
   definition stands at its own C position.
2. **`register` is dropped** (C 71, 84, 93, 105, 143-147, 269, 347-350, 597-601).  It is a
   storage-class hint with no Rust counterpart and no semantic content in either language.
3. **`esctab` is spelled byte by byte** (C 116-120, C 126-130).  C's `static const char x[] = "…"`
   appends the terminating NUL, and Rust's `*b"…"` is `[u8; N]` while the element type here is
   `char`: the port lists the 64+1 and 32+1 elements (with each non-space entry's index in a
   comment) rather than casting the array's type.
4. **`intmax_t padfactor;` (C 297) is a Rust `let mut padfactor: intmax_t;`** initialised from
   C 298's `ckd_mul`, which is the `overflowing_mul` pair (P-15): the wrapped product is stored and
   the overflow flag chooses C 299's `INTMAX_MIN`/`INTMAX_MAX`.
5. **Zero-valued locals** (0.1-13): `buf` (C 349, C's record assigned field by field at C 425-427),
   `cut[11]` (C 224, C leaves it uninitialised), `c1` and `size` (C 145/147, written before their
   first read on every path), `nread` (C 600), `end` (C 507, C's own `end = NULL` is the first
   statement of the loop body).  Nothing else in the file reads a local before writing it.
6. **`valid_filename_p`'s `#else` arm is quoted, not compiled** (Finding 2): its gate cannot be
   satisfied by any configuration that compiles the file.
7. **The live `goto ret` pair is the port's labelled block** (C 371, C 407 → `'ret: { … break 'ret;
   }`, with C 493-495's statements after the block), the PORTING.md 2.2 spelling; the `goto` token
   is exempted by the `tgetent` row of `docs/ports/macro-body-notes.tsv`.
8. **Renames for Rust keywords/primitive names**: C's `str` parameter of `tputs` is `str_`, C's
   `fn` parameter of `valid_filename_p` is `fn_`, the `filep` local keeps its name (PORTING.md
   0.2's renaming rule).
9. **C's anonymous `struct { char *beg; int len; }` (C 224) is named `tgetst1_cut`** (PORTING.md
   1.3: a nested anonymous struct must be named), with the fields and their order unchanged.
10. **C's implicit conversions that Rust spells**: `term = INTERNAL_TERMINAL as *mut c_char`
    (C 364, a `const char *` into `char *`, the `gfilenotify.rs` Deviations 3 shape),
    `termcap_name = TERMCAP_FILE.as_ptr () as *mut c_char` (C 417's `(char *)` cast), and the
    `as c_char`/`as c_int`/`as size_t`/`as ptrdiff_t` narrowings and widenings C performs
    implicitly at C 87, 159, 289, 298, 417, 425-427, 432, 522-524, 526, 631-636.
11. **C's `while (1)` (C 559) is the port's `loop`**, and C's `do { … } while (…)` (C 520-526) is
    the port's `loop { …; if !(cond) { break; } }` (PORTING.md 2.2), the same statements in the same
    order.
12. **C 57/263/304 are form-feed page breaks**; the port keeps the blank line they stand for and
    the C line-range table above leaves them uncovered as blank lines (`strip()` treats `\f` as
    whitespace, which is how this gate reads them).

## Owed

The integrator's list; nothing here is claimed as done by this lane.

1. **The msdos.h mirror's two override rows** (`include/msdos.rs` 232/241): under
   `feature = "msdos"` they collide with `lib/include/errno.rs` 92 and `lib/include/fcntl.in.rs`
   118 (`E0428`), so **no unit that includes `msdos.h` can flip its arm** today - the item
   `docs/ports/msdos.md` Owed 2 already records.  This lane's flip is blocked on it (Evidence ②).
2. **`docs/ports/line-drift-tolerance.tsv`: one row**, stem `termcap`, tolerance ≥ 0.38, basis
   `total`, with this ledger's reason (C 640 vs the port's 883, 38.0%).  **DONE (status 3.495,
   integrator):** the `total`/`0.39` row is appended and `extract-abi.py --check
   emacs/src/termcap.c femacs/src/termcap.rs` is now **OK with zero `-` rows**.
3. **Feature registrations** (`femacs/xtask/Cargo.toml` + `docs/baseline/feature-coverage.tsv`):
   `is_ebcdic_host` (C 111/193) and `internal_terminal` (C 359).  `msdos`, `debug`, `test`,
   `bufsize` and `termcap_file` are already spelled in the tree (`msdos`/`test`/`debug` by other
   units; `bufsize`/`termcap_file` by this one - `feature-coverage.py --write` will list them).
   **DONE (status 3.495, integrator):** all four names (`is_ebcdic_host`, `internal_terminal`,
   `bufsize`, `termcap_file`) are in `femacs/xtask/Cargo.toml`'s `[features]` table with their C
   judgement lines, and `feature-coverage.py --write` has been re-run.
4. **`lib/include/stdlib.rs`: `atoi`** (C 87's only use; declared in this unit, Finding 4).
5. **`include/lisp.rs`'s chain: `lisp.h` 30's `<string.h>`** (`lib/include/string.rs`), which
   `strcmp`/`strcpy`/`memcpy` need (Finding 3; spelled in this unit).
6. **The msdos.h mirror's missing chain edges**: `character.h` (MAX_CHAR, CHAR_VALID_P,
   CHARACTERP, STRING_CHAR, UNIBYTE_TO_CHAR, BYTES_BY_CHAR_HEAD, `string_char_and_length`,
   `raw_prev_char_len`) and `systime.h` (`Time`) - its `termchar.rs`/`buffer.rs`/
   `dispextern.rs`/`termhooks.rs` copies name them and the C headers supply them (Evidence ②).
7. **`docs/ports/macro-body-notes.tsv`: two rows, written by this lane** - `tgetent` → `goto` (the
   labelled block, PORTING.md 2.2) and `scan_file` → `1` (C's `static bool` returning 1/0; the
   `Fset_standard_case_table` precedent).  Kept here as a record, not as a request.
8. **`tools/check-port-lines.py --write-pending`**: the `src/termcap.c` ratchet row is stale (the
   port is 883 lines against the reference's 640) and the P-86 threshold exit should be recorded.
   **DONE (status 3.492/3.495, integrator):** the row is off (`53 ratchet row(s), 0 new, 0 stale`;
   53 files under ten percent / 69,407 reference lines) and the exit is written up in
   `docs/status.md` 3.495.
9. **`docs/status.md`**: this file's §3.31 reading ("the empty unit plus the note that names the
   gate") is withdrawn as a *conclusion* by P-86 ①; the measurements it recorded (the empty object,
   the empty checklist) still hold.  The line should be rewritten the way `msdos.c`'s, `w32.c`'s
   and `androidvfs.c`'s were.
10. **`docs/ports/termcap.md` is now a bound ledger** for
    `tools/check-dead-file-coverage.py`; if the main agent's tolerance row changes the line count,
    the table itself does not move (it counts C lines, not Rust ones).

## Not claimed

- **Zero run evidence**: the 14 MB object is a compile artifact.  No termcap entry was ever
  found, no `/etc/termcap` was read, `tgetent`/`tgetstr`/`tputs` were never called, no terminal was
  driven, and `PC` was never a pad character.
- **The arm flip is rustc on this host**, not an MS-DOS compile: there is no DJGPP toolchain here
  (the same absence `docs/ports/msdos.md` records), the `<dos.h>`/`<go32.h>` surfaces are not
  measured, and the readings in Evidence ③ come from a **/tmp** gap stub that stands in for the
  mirror fix Owed 1/6 names - the *unstubbed* arm is exit 1, so the whole flip is **not passed**.
- **The `is_ebcdic_host` arm**, the `internal_terminal` arm and the `debug` arm have now each been
  compiled once (Evidence ③) but never *run*; the EBCDIC table's *values* are C's bytes, and no
  byte of them was exercised.
- **The `bufsize` and `termcap_file` arms have never been compiled** (Evidence ③'s last two rows):
  C takes those values from the build's `-D`, and the port has no counterpart, so those two
  configurations do not build.  The same is true of C's two `#ifndef` defaults if a build defines
  them differently.
- **`esctab`'s terminating NUL** is carried because C's array has it; nothing reads it.
- The frozen `build-tu` object says nothing about the body (it is the empty object by construction),
  and no link-time reading was taken: this unit defines `tgetent`/`tgetstr`/`tputs`/`tgetnum`/
  `tgetflag`/`PC` only under `feature = "msdos"`, where no other unit of this tree is compiled
  either.
