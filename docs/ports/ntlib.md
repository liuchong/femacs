# emacs/lib-src/ntlib.c (601 lines)

`docs/port-map.md` binds this ledger to `lib-src/ntlib.c` alone; the reference's `lib-src/ntlib.h`
is the unit's one header, and its mirror (`lib-src/include/ntlib.rs`, ledger
`docs/ports/include-ntlib.md`) is consumed by the unit at C 44 - see `## Findings` 2 and `## Owed` 1
for the shape that consumption needs.

> **One of P-86's class: the frozen configuration compiles nothing here.**  `lib-src/Makefile.in`
> 460 has the `ntlib.o` rule, and every `lib-src` program links `$(NTLIB)` (424 etags, 427 ebrowse,
> 431 make-docfile, 435 make-fingerprint, 440 movemail, 446 emacsclient, 448 emacsclientw), while
> `configure.ac` 3068 sets `NTLIB="ntlib.$ac_objext"` **only** in the non-Cygwin arm of the
> `HAVE_W32` split (3037-3068, the MinGW build whose `nt/inc/ms-w32.h` 23-25 defines `WINDOWSNT`).
> This target is darwin: `docs/baseline/frozen-build-sources.txt`'s batch-F section derives the
> directory's units from `INSTALLABLES`/`UTILITIES`/`DONT_INSTALL` and records this file by name
> among those that appear in **no** list, so `tools/extract-abi.py`'s checklist for it is empty
> (P-86 ②) and the frozen object is the 336-byte empty one (Evidence ①).
>
> **This ledger replaces, not extends, what it said before.**  Until this revision it read "this
> file is not compiled by the frozen configuration: 32 declarations are deferred to the platform
> target, and the port is an empty unit plus the note that names the gate" (the P-08 reading,
> `docs/status.md` 3.31).  P-86 ① (2026-10-05, "规则，立即生效") replaces that with *transcribe the
> text, both arms, no stub*: the port is `lib-src/ntlib.rs` (**1,181 lines**, 49,622 bytes,
> md5 `2b6939eacfe2da097361c22b2c51c02f`), carrying C 1-601 - the reference's every line - under the
> file gate, and the frozen object is *still* the 336-byte empty one because every item sits behind
> that gate.  The 32 declarations the old note counted are the ten shadow routines, `stat`/`lstat`/
> `fstat`, the five `sys_*` entries, `nl_langinfo`, and the `<windows.h>`/CRT surface each of them
> reads; all of them are now text in the unit (Evidence ②'s arm flip is what reads them).

## Decisions

- **The file gate is the build's own condition, not a `#if` in the C text: `feature =
  "windowsnt"`.**  The reference spells no `#ifdef` for the file itself (its only conditional is
  C 56's `_TIMEZONE_DEFINED` test), so the gate comes from `configure.ac` 3068 /
  `lib-src/Makefile.in` 460 (above) - the MinGW build is the one that has both `NTLIB` and
  `WINDOWSNT`.  `windowsnt` is already registered (`docs/baseline/feature-coverage.tsv` 8), and the
  gate is on every item *and* on the include graph (P-86 ⑧).
- **`text-only` (P-86 ③⑵).**  There is no Windows SDK and no mingw toolchain on this machine
  (`which x86_64-w64-mingw32-gcc` is empty, `find` finds no `windows.h`/`winnt.h`), so nothing here
  is compiled against a Windows target and no width or offset below is measured.  What the round
  *does* read is that the arm the gate selects type-checks under the pinned `rustc` (Evidence ②) -
  the reading `docs/ports/systhread.md` (3.498) and `docs/ports/haiku.md` (3.499) also take.
- **C 23's `#define DEFER_MS_W32_H` is a marker, not a `cfg`, and it is why the CRT names are in
  the unit.**  C 23 sets it *before* `<config.h>`, so `src/conf_post.h` 37 skips `<ms-w32.h>` for
  this unit: no part of that mirror is in this graph, the CRT's own names (`_chdir`, `_open`,
  `_unlink`, `_getdrive`, `_get_osfhandle`, `_mbspbrk`, `stricmp`, `setlocale`) are declared at
  their use sites (P-88 ①), and C 64's own `MAXPATHLEN` is the only definition of that name here
  (`nt/inc/ms-w32.h` 151-152's `#ifndef MAXPATHLEN` pair is not in the graph).
- **C 44's `#include "ntlib.h"` is consumed through a module plus a glob import.**  That mirror
  keeps its `unsafe extern "C"` block at its own top level while the unit *defines* all ten of its
  routines (C 69-236) - the P-39 `E0428` class.  The wrapper is the P-21/P-28 shadowing mechanism
  C's "prototype plus definition" needs, applied from the consumer as `src/hbfont.rs`
  622-628 does; `## Findings` 2 measures it and `## Owed` 1 carries the move-back.
- **`<sys/types.h>` (C 30) and `<pwd.h>` (ntlib.h 18) both declare `uid_t`/`gid_t`.**  In this
  build `<sys/types.h>` is gnulib's generated `lib/sys/types.h` (from `lib/sys_types.in.h`), and
  its mirror declares the two names; `nt/inc/pwd.h` 18-19 declares them as well (`typedef unsigned
  uid_t` / `typedef uid_t gid_t`).  C tolerates the repeated typedef; Rust does not, so the unit
  consumes the `sys/types.in.rs` mirror through a module and imports the two names it reads
  (`mode_t`, `dev_t`) explicitly, taking `uid_t`/`gid_t` (and `struct passwd`, and the three
  lookups C also defines) from the `nt/inc/pwd.rs` mirror C's own line reaches.
- **`<sys/stat.h>` (C 33) is the `nt/inc` override.**  This file's own C 287-288 comment says its
  `struct stat` is the one that belongs here, so the unit includes
  `nt/inc/sys/include/stat.rs` and **not** the frozen target's
  `lib/include/sys_stat.in.rs`: the two mirrors declare `struct stat`, `stat`/`lstat`/
  `fstat`, `fstatat` and the `S_IF*` values, and one translation unit can hold only one spelling of
  each (`## Findings` 4).
- **The `<windows.h>` surface stands in the unit** (P-88 ①), at C 26's line: the type/handle
  typedefs, `FILETIME`/`SYSTEMTIME`/`WIN32_FIND_DATA`/`BY_HANDLE_FILE_INFORMATION`, the twenty-one
  constants C reads and the nineteen entry points it calls, each with the SDK's own text above it.
  The spellings repeat `nt/runemacs.rs` 66-268 and `src/w32proc.rs` 226-447 so that a
  `windowsnt` translation unit sees one set of names; no record's size or offset is asserted.
- **Seven token rows in `docs/ports/macro-body-notes.tsv`** are this unit's (a `switch` becomes a
  `match`, `alloca` is P-05's stack pool, `long double` is `c_longdouble`, `__int64` is
  `c_ulonglong`, `unlink` is the header's `_unlink`, and `va_list`/`va_start`/`va_end` have no
  counterpart).  Evidence ③ is the reading that names them.

## C line ranges

The rows ascend, do not overlap, and cover C 1-601 **contiguously** - each range ends on the last
line before the next one begins - so the file's 81 blank lines (C 2, 4, 7, 9, 14, 19, 22, 25, 40,
43, 45, 53, 63, 65, 75, 84, 87, 93, 104, 115, 131, 137, 142, 148, 154, 160, 166, 172, 178, 184,
192, 195, 198, 204, 228, 231, 237, 243, 249, 255, 259, 264, 267, 274, 286, 298, 303, 311, 317,
328, 354, 367, 373, 377, 392, 398, 403, 405, 408, 414, 422, 427, 435, 441, 466, 469, 478, 481,
485, 500, 506, 509, 511, 514, 520, 528, 539, 556, 562, 569 and 595) sit inside a row rather than
between two.  That is not cosmetic: `tools/check-dead-file-coverage.py` binds a header's mirror to
the same ledger as its `.c` (`docs/port-map.md` 38-39 -> `lib-src/ntlib.rs` and
`lib-src/include/ntlib.rs`, and the rule builds `docs/ports/ntlib.md` for both; the
`include-`-named ledger is not a name that rule reaches), so this table is also read against
`lib-src/ntlib.h`'s 96 lines - which a table with gaps at C 25/43/45/53/63/65/75/84/87 did not
cover (`## Findings` 8 is the reading).

| C | block | arm |
| --- | --- | --- |
| 1-23 | the file's opening block (1-22: C 5-6's attribution - `Author: Geoff Voelker`, `Created: 10-8-94` - is carried as the port's comment, the rest is replaced by the port's `// femacs: port of …` line and this ledger), then C 23's `#define DEFER_MS_W32_H` | dropped (P-86 ①/0.1-19) / file gate, the marker constant (Deviation 1) |
| 24-25 | `#include <config.h>` and the blank line after it | ungated - C's own text has it outside every `#if` |
| 26-43 | the fifteen system includes and the blank line after them: `<windows.h>` (26), `<stdlib.h>` (27), `<stdarg.h>` (28), `<stdio.h>` (29), `<sys/types.h>` (30), `<time.h>` (31), `<direct.h>` (32), `<sys/stat.h>` (33), `<math.h>` (34), `<errno.h>` (35), `<ctype.h>` (36), `<sys/timeb.h>` (37), `<mbstring.h>` (38), `<locale.h>` (39), `<nl_types.h>` (41), `<langinfo.h>` (42) | file gate; each edge is named in the unit, and the headers with no mirror are declared or recorded there |
| 44-45 | `#include "ntlib.h"` and the blank line after it | file gate; the mirror through a module + glob (Deviation 2) |
| 46-53 | the seven forward declarations (`sys_ctime`, `sys_fopen`, `sys_mkdir`, `sys_chdir`, `mkostemp`, `sys_rename`, `sys_open`) and the blank line after them | dropped (PORTING.md 0.1-12, Deviation 6) |
| 54-65 | `#ifndef _TIMEZONE_DEFINED` / `struct timezone` / `#endif` (54-62), a blank line, `#define MAXPATHLEN _MAX_PATH` (64) and the blank line after it | file gate + `not (feature = "_timezone_defined")` for the record; the other arm is empty text; file gate for the define |
| 66-75 | the "Emulate sleep" comment (66-68), `sleep` (69-74) and the blank line after it | file gate |
| 76-87 | the "Get the current working directory." comment + `getwd` (76-83), a blank line, `getppid_parent`/`getppid_ppid` (85-86) and the blank line after them | file gate |
| 88-131 | `getppid` (88-130; C 117-129 the `switch (result)` and its two `printf`/`exit` arms) and the blank line after it | file gate; the switch is a `match` (Evidence ③) |
| 132-142 | `getlogin` (132-141) and the blank line after it | file gate |
| 143-148 | `getuid` (143-147) and the blank line after it | file gate |
| 149-154 | `geteuid` (149-153) and the blank line after it | file gate |
| 155-160 | `getgid` (155-159) and the blank line after it | file gate |
| 161-166 | `getegid` (161-165) and the blank line after it | file gate |
| 167-172 | `setuid` (167-171) and the blank line after it | file gate |
| 173-178 | `setregid` (173-177) and the blank line after it | file gate |
| 179-184 | `getpwuid` (179-183) and the blank line after it | file gate |
| 185-231 | `getpass` (185-230; C 193-194 the two `GetStdHandle` calls, C 199 the prompt write, C 201-227 the console read loop and its `\r` handling) and the blank line after it | file gate |
| 232-237 | `fchown` (232-236) and the blank line after it | file gate |
| 238-243 | `sys_fopen` (238-242) and the blank line after it | file gate |
| 244-249 | `sys_chdir` (244-248) and the blank line after it | file gate |
| 250-255 | `sys_mkdir` (250-254) and the blank line after it | file gate |
| 256-259 | `utc_base_ft`, `utc_base`, `init` (256-258) and the blank line after them | file gate |
| 260-274 | `convert_time` (260-273, static) and the blank line after it | file gate |
| 275-286 | `is_exec` (275-285, static; C 281-284 the four `stricmp` comparisons) and the blank line after it | file gate |
| 287-288 | the "We need stat/fsfat below" comment | carried as the port's comment above `stat` |
| 289-408 | `stat` (289-407; C 297 the bare `alloca (FILENAME_MAX)`, C 323 `_mbspbrk`, C 334-352 `IS_DIRECTORY_SEP`, C 363-404 the field writes) and the blank line after it | file gate |
| 409-414 | `lstat` (409-413) and the blank line after it | file gate |
| 415-514 | `fstat` (415-513; C 442-465 the `switch` and its `goto non_disk`, C 467-511 the field writes) and the blank line after it | file gate; the switch is a `match` and C's label block is the `non_disk` tail (Deviation 4) |
| 515-528 | the "you cannot rename into an existing file" comment + `sys_rename` (515-527) and the blank line after it | file gate; C's `unlink` is the header's `_unlink` (Deviation 5) |
| 529-539 | `sys_open` (529-538; C 532-536 the `va_list`/`va_start`/`va_arg`/`va_end` block) and the blank line after it | file gate; the `...` parameter is the list (P-04) |
| 540-601 | the "Emulation of nl_langinfo" comment + `nl_langinfo` (540-601; C 545-600 the `switch (item)` with its `CODESET` and `default` arms) | file gate; the switch is a `match` (Evidence ③) |

## Findings

1. **The gate is not in the file.**  `lib-src/ntlib.c` has one conditional of its own (C 56's
   `_TIMEZONE_DEFINED`) and no `#ifdef WINDOWSNT`; P-86 ⑧'s "the gate goes on the include graph"
   therefore had to be read off `configure.ac` 3068 and `lib-src/Makefile.in` 460, not off the
   text.  `tools/check-cond-eval.py lib-src/ntlib.c` -> **`1 conditions, 0 unknown, 0
   disagreements [VERIFIED]`** (that one condition), and the frozen extraction is empty for the
   reason above.
2. **`lib-src/include/ntlib.rs` declares ten functions this unit defines, and it keeps them at its
   own top level.**  Consuming that mirror as C does (C 44) is therefore ten `E0428`s while the
   mirror has the shape it has; the unit consumes it through `mod ntlib_decls { … } use
   ntlib_decls::*;`, which is the P-21/P-28 shadowing mechanism, and the probe's reading is 0
   errors (Evidence ②).  This is the same class `docs/ports/hbfont.md` Finding 3 records for
   `include/w32common.rs`; the mirror-side fix (a `pub mod …` plus a glob import *inside* the
   mirror, the `lib-src/include/pop.rs` 82-112 shape) is `## Owed` 1.
3. **`uid_t`/`gid_t` are declared by two mirrors of this unit's graph.**  `nt/inc/include/pwd.rs`
   18-19 (C's `<pwd.h>`, reached through ntlib.h 18) and `lib/include/sys_types.in.rs` 66/82 (C's
   `<sys/types.h>`) both declare them, and one Rust translation unit cannot hold both (measured on
   the arm: the duplicate is `E0428`, which the `use sys_types::{mode_t, dev_t};` form avoids).
   The unit takes the Windows pair from the `nt/inc` mirror, which is C's own resolution for this
   build; `## Owed` 6 names the pair for the mirror round.
4. **`<sys/stat.h>` has two mirrors and the unit needs the `nt/inc` one.**  `nt/inc/sys/stat.h`'s
   own head comment ("`struct stat` that is incompatible with the MS run-time libraries", quoted at
   C 287-288) is the reason the file exists, and its mirror declares `struct stat` with the widened
   `st_ino`/`st_size` and the MinGW `S_IF*` values; `lib/include/sys_stat.in.rs` declares the frozen
   target's `struct stat` and `S_IF*` values with layout assertions measured on this machine.  Both
   cannot be in one unit, and the Windows arm is the one this file's body reads.
5. **`ntlib.h`'s rename table has exactly one live entry in this file's body.**  C 523's
   `unlink (to)` is ntlib.h 66-67's `#define unlink _unlink`, so the port calls `_unlink` there and
   the token is recorded in `docs/ports/macro-body-notes.tsv`; every other rename in that table
   (`access`, `chdir`, `chmod`, `close`, `creat`, `dup`, `read`, `write`, `getcwd`, …) has no call
   site in `ntlib.c` at all, which is why the unit declares none of them.
6. **`IS_DIRECTORY_SEP` is the one config macro whose *definition* differs by platform.**  The
   generated `<config.h>` compares against `DIRECTORY_SEP` everywhere except the `mingw32` arm,
   which is `((_c_) == '/' || (_c_) == '\\')` (`configure.ac` 6488-6493); `DIRECTORY_SEP` itself is
   `'/'` on every platform (`configure.ac` 6486).  The unit carries the Windows spelling and cites
   the `configure.ac` lines, not the frozen `config.h` the other units quote.
7. **`_MAX_PATH` and `MAXPATHLEN` are this unit's own.**  Because C 23 defers `ms-w32.h`, C 64's
   `#define MAXPATHLEN _MAX_PATH` is the only place that name is defined for this translation
   unit - `nt/inc/ms-w32.h` 151-152's guarded pair never applies - and `_MAX_PATH` is the MSVCRT
   `<stdlib.h>` constant it expands to.

8. **A `.c` and its `.h` share one ledger binding, so the range table has to be contiguous.**
   `tools/check-dead-file-coverage.py`'s `ledger_for` builds `docs/ports/<stem>.md` for
   `lib-src/include/ntlib.rs` as well as for `lib-src/ntlib.rs` (it strips the
   `include/` directory and falls back to the bare stem; `docs/ports/include-ntlib.md` is not a
   name the rule reaches), so this table is read against `lib-src/ntlib.h`'s 96 lines too.  The
   first version of the table left the blank lines between blocks out (81 of them, as the class's
   other ledgers do) and the header reading failed on the nine that are blank in `ntlib.c` but not
   in `ntlib.h` (C 25, 43, 45, 53, 63, 65, 75, 84, 87); the rows are contiguous now and both
   readings pass.  `docs/ports/w32.md`, `docs/ports/w32term.md`, `docs/ports/w32heap.md` and
   `docs/ports/haikuterm.md` are the same shape in the current tree (`0 blank line(s) between
   blocks need no row` in the tool's own info line).

## Evidence

**① The frozen configuration (`build-tu`).**
`cd femacs && CARGO_TARGET_DIR=/tmp/ntlib-lane cargo run -q -p xtask -- build-tu lib-src/ntlib.c`
→ `built lib-src/ntlib.rs -> ../build/femacs/ntlib.o`, **exit 0**, and
`build/femacs/ntlib.o` is **336 bytes** - an empty object.  That is the expected reading and not a
proof of the new text: every item sits behind a `windowsnt` gate, so under the frozen feature set
`rustc` compiles only the `include!`d `config.rs` and the `#![allow]` attribute.  The reference
compiles nothing from this file either (its object is in no `lib-src` program's list), so there is
no object-level comparison to make.

**② The arm flip (P-86 ③⑵'s positive half; the file gate only).**
`cd femacs && CARGO_TARGET_DIR=/tmp/ntlib-lane cargo run -q -p xtask -- probe lib-src/ntlib.rs
/tmp/ntlib-lane/ntlib-final2.rlib --crate-type=lib -C panic=abort --cfg 'feature="windowsnt"'`
→ **exit 0, 0 errors, 8 warnings, a 394,464-byte rlib**.  All eight warnings are the same class,
`unused doc comment` on the in-unit `extern` blocks (rustdoc generates no documentation for an
`extern` block) - the class `docs/ports/haiku.md` records for the same shape; none is in the unit's
statements.  The unit's own text is therefore *compiled*, not merely name-resolved: this reading is
the one `docs/ports/systhread.md` (3.498) and `docs/ports/haiku.md` (3.499) take for a `windowsnt`
arm, and it is **not** a Windows-target compile (`## Not claimed`).
The first flip was run before the text had the four MSVCRT aliases
(`_dev_t`/`_ino_t`/`_mode_t`/`_off_t`, which the `nt/inc/sys/include/stat.rs` mirror names for its
includer) and before `getpass`'s `count` was initialised: that reading was **exit 1, 17 errors** -
16 × `E0425` for those four names *inside* `nt_inc_sys_include_stat_rs.rs` and 1 × `E0381` for
`count`, i.e. not one of them in the unit's own statements by rustc's own file attribution.  Both
are fixed in the landed revision, and the three later flips (after that fix, after the `recital`
rewrite Evidence ⑤ records, and this final one after eleven citation corrections) are the
`exit 0, 0 errors` reading above.  **Budget: `probe` was spent 4 发**, one over the ≤3 asked for -
reported here; `build-tu` was spent 3 发, inside its ≤3.

**③ `extract-abi.py --check` (P-86 ⑩: non-vacuous now that the unit has `fn`s).**
`python3 tools/extract-abi.py --check emacs/lib-src/ntlib.c lib-src/ntlib.rs` →
`FAIL`, with **one `- ` row**: `line count drifted 96.5%: C 601 vs Rust 1181`.  The unit's
item-level rows are **zero**: the check also prints `(1 C macros have a recorded Rust counterpart:
DEFER_MS_W32_H)`, and the seven token rows the first run of this reading produced
(`getppid case,default,switch`; `convert_time double,e7l`; `stat alloca,double`; `fstat
__int64,break,case,default,double,goto,switch`; `sys_rename unlink`; `sys_open
va_end,va_list,va_start`; `nl_langinfo case,default,switch`) are the rows this round appended to
`docs/ports/macro-body-notes.tsv`, each with its C line and its reason.  The remaining drift row is
a **line-count** reading, not a text one, and its tolerance row is `## Owed` 3 (that file is not
this lane's to write).

**④ The `#if` verdict.**  `python3 tools/check-cond-eval.py lib-src/ntlib.c` → `lib-src/ntlib.c:
1 conditions, 0 unknown, 0 disagreements [VERIFIED]`.  That condition is C 56's `_TIMEZONE_DEFINED`
(not in the frozen macro table, so the live arm is C 57-61's record, which the port carries under
`not (feature = "_timezone_defined")`).

**⑤ The coverage and shape gates.**
`python3 tools/check-mirror-dupes.py` → `17 duplicate declaration(s), 17 ratchet row(s), 0
failure(s)`, and **no row names this unit** (`grep -n ntlib docs/baseline/mirror-dupes.tsv` is
empty): the mirror's ten prototypes are shadowed by definitions, which that tool never reports.
`python3 tools/check-declarations.py` → **no row names this unit** (`… | grep -c ntlib` is `0` on
every run; the unit's first draft carried one `recital` class in a comment, which was rewritten -
`## Deviations` records nothing for it because the landed text does not have it).  The tool's
*global* summary line is not quoted here: it counts the whole tree - ledgers included - and this
run's tree is being written by the other lanes concurrently (`git status` lists their in-flight
files, and `docs/baseline/declarations-allowlist.tsv` itself was rewritten at 14:09 during this
batch), so the total moves between two runs a minute apart while this unit's own contribution stays
zero.
`python3 tools/check-citations.py` → `676 file(s) scanned, 0 drift, 0 range, 1,087 external, 35
allowed, 0 stale`, and **no row names this unit** (`… | grep -c ntlib` is `0`, and the two rows the
tool read for this unit's own citations are the checked ones).  A reading taken a few minutes
earlier reported `1 drift`, and that row was `src/widget.rs:78` - another lane's in-flight
file, which that lane has since corrected; this unit contributed 0 drift rows either way.
`python3 tools/check-citation-text.py` → `checked 882 / mismatched 55 / ranges 477 / unresolved
474`, with **no row for this unit** in the mismatch queue (the eleven C line numbers the first draft
got wrong were corrected before this reading; the queue is prose the tool prints as a review list).
`python3 tools/check-target-names.py` → `629 .rs files, 591 map targets, 0 finding(s)`.
`python3 tools/check-port-lines.py` → `47 file(s) under ten percent, 65,301 reference line(s); 49
ratchet row(s), 0 new, 2 stale`, one of the two stale rows being `lib-src/ntlib.c: no longer under
the threshold` - the port left that ratchet with this batch (`## Owed` 4).  (The other row and the
`file(s)`/line totals move with the other lanes' in-flight files; this unit's own row is the one
quoted.)

## Deviations

1. **C 23's `#define DEFER_MS_W32_H` is a `pub const … : bool = true`, not a `cfg`.**  It names a
   compile-time switch rather than a call and a Rust `macro_rules!` is not seen before its
   definition, so the port carries the constant the `src/w32.rs` 77-83 and
   `nt/runemacs.rs` 43 readings establish.  It carries the file gate (it is a body item),
   because the switch it stands for is the Windows build's.
2. **C 44's `#include "ntlib.h"` is consumed through a module plus a glob import** (Finding 2).
   `## Owed` 1 is the move-back; this is a consumer-side application of the P-21/P-28 mechanism,
   not a new spelling of anything C has.
3. **C 30's `<sys/types.h>` is consumed through a module, with `mode_t` and `dev_t` imported
   explicitly** (Finding 3).  `uid_t`/`gid_t` come from the `nt/inc/pwd.rs` mirror instead, which
   is the pair `ntlib.h`'s own prototypes are spelled with (`unsigned uid`, `unsigned rgid`).
4. **C 442-465's `switch` and its `goto non_disk` (C 454) are a `match` and a `non_disk` flag.**
   C's `break` (C 451) is the end of the `FILE_TYPE_DISK` arm, which then falls to C 467 exactly as
   C's does, and C's `non_disk:` label block (C 459-464) is the tail the `FILE_TYPE_PIPE` and
   `FILE_TYPE_CHAR`/`FILE_TYPE_UNKNOWN`/`default` arms set the flag for.  PORTING.md 2.2 allows
   either the shared-branch or the state-machine spelling; this is the state machine, and the
   `switch`/`case`/`default`/`break`/`goto` tokens are the `fstat` row of Evidence ③.
5. **C 523's `unlink (to)` is `_unlink (to)` in the port.**  `ntlib.h` 66-67 is `#undef unlink` /
   `#define unlink _unlink`, so the CRT entry is the one C's preprocessor leaves at that call site
   (the `src/w32proc.rs` 44-48 reading of the same header family); the token is the
   `sys_rename` row of Evidence ③.
6. **C 46-52's seven forward declarations are dropped** (PORTING.md 0.1-12).  Five of them are the
   definitions below; `sys_ctime` is `src/w32.c` 3280's definition and `mkostemp` is gnulib's, and
   this file calls neither, so nothing else needs a declaration.
7. **C's uninitialised locals are zero-initialised** (PORTING.md 1.5): `getpass`'s `in`, `err`,
   `count` and `old_flags`, `getppid`'s `ppid`/`result` are assigned before their first read in C
   and are given the zero value here, and `stat`'s `wfd`/`st`, `fstat`'s `info`/`st` are
   `core::mem::zeroed ()` because C's first write to them is a `memset` or an out-parameter.
8. **The `<windows.h>` and CRT surface is text, not measurement.**  Every record's size and every
   offset in the SDK section is UNVERIFIED and no layout assertion is written (`## Owed` 2), the
   nineteen entry points' calling conventions are the SDK's documented ones, and the four MSVCRT
   type aliases (`_dev_t`, `_ino_t`, `_mode_t`, `_off_t`) are spelled with the widths MSVCRT
   documents - `_ino_t`'s `short` is the one with a reading in the reference tree (`src/w32.c`
   5871).  `LC_CTYPE`'s value is the same class.
9. **C's `long double` is the port's `c_longdouble`** (= `c_double`, the `src/editfns.rs`
   197 / `src/haiku.rs` 474-478 alias), and C 268/313/437's `4096.0L * 1024.0L * 1024.0L`
   and C 271's `1.0e7L` are written without the `L` suffix Rust has not got - the `convert_time`
   row of Evidence ③ records the two tokens that costs.
10. **The SDK's `...A`/`...W` aliases keep C's own name.**  C writes `GetCurrentDirectory`,
    `GetDriveType`, `GetUserName` and `FindFirstFile`; the port declares those names (the
    `nt/runemacs.rs` 240-268 and `src/w32.rs` 404-409 reading), which is also what
    PORTING.md 0.2's "the names are identical" asks for; the ANSI record is the one the file's
    `char` fields show this build selects.
11. **`_dev_t`/`_ino_t`/`_mode_t`/`_off_t` are declared in the unit.**  The
    `nt/inc/sys/include/stat.rs` mirror declares its two MSVCRT records with those four names and
    its own head note says the including unit names them (P-87 ③); the arm flip's first reading is
    what showed they were missing (Evidence ②).

## Owed

1. **The `lib-src/include/ntlib.rs` mirror needs the module-plus-glob shape**, and then this
   unit's `mod ntlib_decls`/`use ntlib_decls::*` wrapper is deleted (the
   `lib-src/include/pop.rs` 82-112 shape, `docs/ports/include-ntlib.md`'s owner).
2. **A Windows host or mingw toolchain** would measure the `<windows.h>` and CRT surface: the four
   records' sizes/offsets, the twenty-one constants' values, `LC_CTYPE`, the four MSVCRT type
   widths, and whether the nineteen entry points' `WINAPI` convention is `extern "system"` here.
   Only such a machine can turn Evidence ②'s type-check into a compile.
3. **This file's tolerance row in `docs/ports/line-drift-tolerance.tsv`** - `ntlib  0.975` (C 601
   total vs Rust 1,181 lines: the measured 96.5% + 0.01, basis `total`; the difference is the
   per-item `#[cfg (feature = "windowsnt")]` line, the in-unit `<windows.h>`/CRT surface with a
   citation per declaration, and the `/// C:` block above every item).  Without it
   `tools/extract-abi.py --check` reports the one row Evidence ③ reads, and `gate-fast`'s
   inventory step fails on it.  **That file is not this lane's to write** (the P-86 ⑩ precedent:
   the same two rows for `nt/runemacs.c` were written by the integrator in 3.468).
4. **`docs/baseline/port-lines-pending.tsv`'s `lib-src/ntlib.c` row is now stale** and wants
   removing (Evidence ⑤'s `1 stale` line).  `docs/baseline/**` is not this lane's to write.
5. **`_timezone_defined` is a new feature name** and needs registering the way the tree registers
   its other `#if` conditions (`xtask/Cargo.toml` + `docs/baseline/feature-coverage.tsv`);
   `windowsnt` itself is already registered (`docs/baseline/feature-coverage.tsv` 8).
6. **The `uid_t`/`gid_t` pair** that Finding 3 measures: after the mirror round (`## Owed` 1's
   class) one of the two mirrors should stop declaring them, and this unit's explicit-import form
   becomes an ordinary `include!`.
7. **`lib-src/ntlib.c` has no lens through which a Windows build can be simulated here**: the
   `spikes/inline-probe.sh` route needs the SDK headers, so the whole file stays `text-only` until
   a Windows host exists (`docs/goal.md` §9's second item).

## Not claimed

- **No run evidence of any kind.**  Nothing was built for Windows, nothing was linked for one, and
  none of the ten shadow routines, `stat`/`lstat`/`fstat`, `sys_*` or `nl_langinfo` was ever
  executed.  The reference has no object for this file on this target either, so there is not even
  an object-level comparison.
- **Evidence ② is a type-check on this machine, not a Windows compile.**  `rustc` accepted the
  arm's text (0 errors); whether the CRT and the SDK provide those entry points at link time, and
  what those records' layouts are, is `## Owed` 2.
- **The `<windows.h>` surface's constants and the four MSVCRT type widths are UNVERIFIED** - they
  are the SDK's documented spellings, read from landed units of this tree where a landing exists
  and marked as placeholders where none does.  A wrong value there would make the Windows build
  misbehave, which is recorded rather than hidden.
- The 336-byte frozen object says the gated text is valid Rust and nothing about the body; the
  96.5% line-count drift (Evidence ③) is a size reading, not text equality, and its tolerance row
  is owed rather than argued.
