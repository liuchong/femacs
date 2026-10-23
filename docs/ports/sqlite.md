# emacs/src/sqlite.c (933 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 2
- [x] sqlitep  (Fsqlitep, 1/1, doc f14047e38640, line 858)
- [x] sqlite-available-p  (Fsqlite_available_p, 0/0, doc 1cdc980fa9b9, line 869)

## Functions defined — 1
- [x] syms_of_sqlite  (line 889)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

> **One of P-86's class, with an important difference: the frozen configuration compiles part of
> this file.**  The body (C 31-856) sits inside C 31's `#ifdef HAVE_SQLITE3`, which is `undef` in
> `docs/baseline/config/darwin-arm64.macros.tsv` 1551, so the checklist `extract-abi.py` generates
> holds only the *live* tail's three definitions above (P-86 ②: that checklist is vacuous as
> coverage evidence here) - but the tail C 858-933 is **live**, so the frozen object is **not** the
> 336-byte empty one: the reference's own `build/emacs-ref/src/sqlite.o` is **7736 bytes** with
> `T _Fsqlitep`, `T _Fsqlite_available_p`, `T _syms_of_sqlite`, `s _Ssqlitep`,
> `s _Ssqlite_available_p` (Evidence ①).  P-86 ⑧'s criterion therefore reads differently for
> this file: the gates must leave that symbol set *unchanged*, and they do - the port's frozen
> object is the same **3640 bytes / sha256 `f7a85883…`** before and after the whole body was
> carried (Evidence ①).
>
> **This ledger replaces, not extends, what it said before.**  Until this revision it read "C
> 33-853 is dead, recorded here, not transcribed" (the P-08 reading) and its gate row was the
> `live`-basis one in `docs/ports/line-drift-tolerance.tsv` 145.  P-86 ① replaces that with
> *transcribe the text, both arms, no stub*: the port is `src/sqlite.rs` (**1311 lines**,
> 73,499 bytes, md5 `ba31bdb4a3b52241987f50b1985df81e`), carrying **C 1-933** - the reference's
> every line - with **both arms of every `#if`**.  What the earlier note *measured* (an empty live
> checklist, the four `#else`-only live bodies) is unchanged and still true; it is the
> *conclusion* that is withdrawn, as `docs/ports/msdos.md`, `docs/ports/w32proc.md`,
> `docs/ports/xgselect.md`, `docs/ports/sound.md` and `docs/ports/pop.md` record for their own
> files.
>
> The file implements the SQLite bindings when the feature is on: `struct Lisp_Sqlite`'s owner
> side, the eleven helpers (`load_dll_functions`, `init_sqlite_functions`, `sqlite_free`,
> `encode_string`, `make_sqlite`, `check_sqlite`, `bind_values`, `row_to_value`,
> `sqlite_prepare_errdata`, `column_names`, `sqlite_exec`), the file-scope object `db_count`, the
> fifteen `sqlite-*` subrs and the three live tail items (`sqlitep`,
> `sqlite-available-p`, `syms_of_sqlite`).

## Gate mapping

| C `#if` / build condition | Port gate | Compiled when |
| --- | --- | --- |
| the file itself, C 31's `#ifdef HAVE_SQLITE3` | `feature = "have_sqlite3"` - on every body item **and on C 27/29's include edges** (P-86 ⑧) | `HAVE_SQLITE3` is defined |
| C 39-44: `defined HAVE_SQLITE3_LOAD_EXTENSION && defined SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION` computes `HAVE_LOAD_EXTENSION`, which C 91/189/710/901 test | `all (feature = "have_sqlite3_load_extension", feature = "sqlite3_dbconfig_enable_load_extension")` - the same conjunction spelled at each of the four sites (Deviation 12) | both names are defined |
| C 46-196, C 201-221, C 874-882 (`WINDOWSNT`) | `feature = "windowsnt"` (registered, `docs/baseline/feature-coverage.tsv` 8) | a Windows build |
| C 68/113/142/174/475 (`SQLITE_VERSION_NUMBER >= 3007015`) | `feature = "sqlite_version_number_ge_3007015"` | the header is at least 3.7.15 |
| C 483 (`SQLITE_VERSION_NUMBER >= 3006005`) | `feature = "sqlite_version_number_ge_3006005"` | the header is at least 3.6.5 |
| C 296/299/311 (`SQLITE_OPEN_FULLMUTEX` / `SQLITE_OPEN_URI` / `SQLITE_OPEN_MEMORY`) | `sqlite_open_fullmutex` / `sqlite_open_uri` / `sqlite_open_memory` | the header defines each |
| C 862/873/891 (the tail's own `#ifdef HAVE_SQLITE3` arms) | `feature = "have_sqlite3"` | `HAVE_SQLITE3` is defined |

Eight names are new and are Owed 1: `have_sqlite3`, `have_sqlite3_load_extension`,
`sqlite3_dbconfig_enable_load_extension`, `sqlite_version_number_ge_3007015`,
`sqlite_version_number_ge_3006005`, `sqlite_open_fullmutex`, `sqlite_open_uri`,
`sqlite_open_memory`.  `WINDOWSNT` is the twenty-first condition and is already registered; the
21 conditions the evaluator reports are the whole set.

## Decisions

- **The file gate is measured, and so is its shape.**  `python3 tools/check-cond-eval.py
  src/sqlite.c` -> `21 conditions, 0 unknown, 0 disagreements [VERIFIED]`, but **every verdict is
  `False`**: the whole body sits inside the false `#ifdef HAVE_SQLITE3`, and the evaluator
  short-circuits a dead branch instead of evaluating what is inside it.  So the table above is
  decided by hand, not read off the table - and the gates' *reading* is the frozen object: the
  port's `build-tu` object is byte-identical (3640 bytes, sha256 `f7a85883…`) to the one the
  76-line P-08 unit produced, and the reference's own is 7736 bytes with the five tail symbols.
- **The include graph (P-86 ⑧).**  C 27's `<c-strcase.h>` and C 29's `"coding.h"` carry the file
  gate; C 25's `<config.h>` and C 28's `"lisp.h"` do not, and cannot: the three live tail items
  take `DEFUN!`/`DEFSYM!`/`defsubr`/`Q*` from `include/lisp.rs`, so gating it would leave
  C 858-933 unbuildable in the frozen configuration - which is exactly what `build-tu` measures -
  and C's own text has both of them *outside* C 31's `#ifdef` as well (Deviation 1).
- **`<sqlite3.h>` is stated in the unit** (P-88 ①): the declaration site is C 33's `#include`
  line, no file is added under `src/include/`, and every type, constant and prototype
  carries the header line it comes from.  The header **is** on this machine, so nothing here is
  guessed: the two opaque pointers, `sqlite3_int64` and all eighteen constants were measured with
  clang against `/opt/homebrew/opt/sqlite/include/sqlite3.h` (SQLite 3.53.4, `SQLITE_VERSION_NUMBER`
  3053004 - Evidence ②).  That measurement is also what decides the five header-side features:
  `>= 3007015`, `>= 3006005`, `SQLITE_OPEN_FULLMUTEX`, `SQLITE_OPEN_URI` and `SQLITE_OPEN_MEMORY`
  are all true of that header, and `SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION` is 1005.
- **`struct Lisp_Sqlite` and its accessors are `lisp.h`'s**, not this unit's: `lisp.h` 2914-2925
  defines the record, 3003-3025 the four accessors, and the mirror (`include/lisp.rs` 2335-2343,
  2548-2560) carries them as in C.  This unit is their only frozen-configuration consumer, so its
  seven-field layout is asserted here against the C compiler's own numbers for this host
  (Evidence ③).
- **The Windows arm is `text-only`, and its loader family is deferred tree-wide.**
  `include/w32common.rs` 173-183 records that `DEF_DLL_FN`, `LOAD_DLL_FN` and `LOAD_DLL_FN_OPT`
  are **deferred** - Rust has no token pasting, and `__VA_ARGS__`'s trailing-attribute form has no
  `macro_rules!` counterpart, so "the Rust shape of the three is an open design decision for the
  Windows target that first expands them (PORTING.md P-22) rather than something to invent here".
  C 52-99 and C 100-157 are therefore carried as C's own text, and `load_dll_functions`
  (C 159-195) - whose 28 statements are all `LOAD_DLL_FN` expansions - is the **one definition
  this file does not yet carry**; C 210 is the call site that waits on it (P-91 ④, Owed 6).  The
  two mirrors the tree does have (`include/w32common.rs`, `include/w32.rs`) are `include!`ed at
  C 49/50 under `windowsnt`, and `<windows.h>` (C 48) has no mirror of its own.
- **`goto exit` is a labelled block, not an early return** (PORTING.md 2.2): `Fsqlite_execute`
  (C 526, C 536) and `Fsqlite_select` (C 614, C 625, C 633) leave through `break 'exit`, and both
  C's fall-through into `exit:` and its cleanup-then-`xsignal1` are kept (Deviation 7).
- **The two live subrs keep C's text.**  `Fsqlitep`/`Fsqlite_available_p` are *outside* the file
  gate in C (after C 856's `#endif`), so their `DEFUN!`s are ungated and only their arms carry
  `have_sqlite3`; `syms_of_sqlite` is the same shape (C 891-911's fifteen `defsubr`s gated,
  C 912-913's two and the six `DEFSYM`s/`Fput`s outside the gate).

## C line ranges

The rows are **contiguous** - each opens on the line after the previous row's end - so the union is
C 1-933 with no gap and the last row reaches the reference's final line, which is what
`tools/check-dead-file-coverage.py` reads.  There is **no `src/sqlite.h`** in the reference
(`find emacs/src -name 'sqlite*'` finds `sqlite.c` alone) and `docs/port-map.md` 531 has exactly
one row for this pair, so the table covers the one reference it has.

| C | block | arm |
| --- | --- | --- |
| 1-23 | the reference's opening block | dropped (PORTING.md 0.1-19; the port opens with the `// femacs: port of …` line) |
| 24-30 | C 25's `#include <config.h>`, C 27's `<c-strcase.h>`, C 28's `"lisp.h"`, C 29's `"coding.h"` | config.h and lisp.h ungated (C has them outside the gate, and the live tail needs lisp.h); the other two **with the file gate** (P-86 ⑧, Deviation 1) |
| 31-34 | C 31's `#ifdef HAVE_SQLITE3` and C 33's `#include <sqlite3.h>` | **C 31 is the file gate**; `<sqlite3.h>`'s surface is stated in-unit (P-88 ①) |
| 35-44 | C 35-38's note and C 39-44's `HAVE_LOAD_EXTENSION` computation | the two features of C 39's condition, spelled at its four consumers (Deviation 12) |
| 45-51 | C 46's `#ifdef WINDOWSNT` and C 48-50's three includes | `windowsnt`; `<windows.h>` has no mirror, `w32common.h`/`w32.h` do |
| 52-99 | the `DEF_DLL_FN` block and C 91-99's `HAVE_LOAD_EXTENSION` arm | `windowsnt`; **carried as C's text** (the macros are deferred, P-22) |
| 100-157 | the `#undef` / `#define sqlite3_* fn_sqlite3_*` alias block | `windowsnt`; **C text** (same deferral) |
| 158-196 | C 159-195's `load_dll_functions` and C 196's `#endif` | `windowsnt`; **the one definition this file does not carry yet** (Owed 6); its body is C text, and C 210 waits on it |
| 197-227 | C 198-225's `init_sqlite_functions`, both arms | file gate; C 201's arm is `windowsnt` (and its C 202 function-local `static` stays function-local, Deviation 9) |
| 228-238 | `sqlite_free` | file gate; `extern "C"` - its address is the object's `finalizer` (0.1-6) |
| 239-247 | `encode_string` | file gate; `encode_string_utf_8` is declared in-unit (mirror gap, Owed 5) |
| 248-261 | `make_sqlite` | file gate; `ALLOCATE_PLAIN_PSEUDOVECTOR!`/`make_lisp_ptr` spell C's two macros |
| 262-277 | `check_sqlite` | file gate; `CHECK_SQLITE`/`XSQLITE` are the mirror's |
| 278-279 | `static int db_count = 0;` | file gate; C's `static` is internal linkage, so the item is not `pub` (0.2) |
| 280-327 | `sqlite-open` (`Fsqlite_open`) | file gate; C 296/299/311's three `#ifdef`s are their own features, C 322's `sdb` carries PORTING.md 1.5's zero init (Deviation 3) |
| 328-342 | `sqlite-close` (`Fsqlite_close`) | file gate |
| 343-421 | C 344-345's note and `bind_values` | file gate; C 369's `encoded` carries 1.5's zero init (Deviation 2) |
| 422-465 | `row_to_value` | file gate; C 433's `switch` becomes a `match` with the `_ => {}` arm C's missing `default` needs (Deviation 6) |
| 466-491 | `sqlite_prepare_errdata` | file gate; C 475/483's two version gates |
| 492-566 | `sqlite-execute` (`Fsqlite_execute`) | file gate; the `goto exit` pair (Deviation 7), C 545-547's `do`/`while` (Deviation 8) |
| 567-577 | `column_names` | file gate |
| 578-652 | `sqlite-select` (`Fsqlite_select`) | file gate; the three `goto exit`s of C 614/625/633 |
| 653-673 | `sqlite_exec` and `sqlite-execute-batch` | file gate |
| 674-708 | `sqlite-transaction`, `sqlite-commit`, `sqlite-rollback`, `sqlite-pragma` | file gate |
| 709-790 | `sqlite-load-extension` (`Fsqlite_load_extension`) with C 710's gate and C 790's `#endif` | `all (have_sqlite3_load_extension, sqlite3_dbconfig_enable_load_extension)`; C 763's `strcasecmp` is declared in-unit (Owed 5) |
| 791-813 | `sqlite-next` (`Fsqlite_next`) | file gate |
| 814-833 | `sqlite-columns` (`Fsqlite_columns`) and `sqlite-more-p` (`Fsqlite_more_p`) | file gate |
| 834-844 | `sqlite-finalize` (`Fsqlite_finalize`) | file gate |
| 845-856 | `sqlite-version` (`Fsqlite_version`) and C 856's `#endif /* HAVE_SQLITE3 */` | file gate; **the gate closes here**, as in C |
| 857-867 | `sqlitep` (`Fsqlitep`) | **live**: the `DEFUN!` is ungated and only C 862's arm carries `have_sqlite3` |
| 868-886 | `sqlite-available-p` (`Fsqlite_available_p`) | **live** likewise; C 874-882's inner arm is `windowsnt` |
| 887-933 | `syms_of_sqlite` | **live**: C 891-911's fifteen `defsubr`s under the gate, C 912-913's two and the six `DEFSYM`s/`Fput`s outside it |

## Evidence

① **The frozen reading (P-86 ⑧).**
`cd femacs && CARGO_TARGET_DIR=/tmp/sqlite-lane/cargo cargo run -q -p xtask -- build-tu src/sqlite.c`
-> `built src/sqlite.rs -> ../build/femacs/sqlite.o`, exit 0, and `build/femacs/sqlite.o` is
**3640 bytes**, sha256 `f7a85883d822bccb0a49c204dab499e08b06c583303daedd12df6559ba042ee8` - **the
same size and the same sha256 as the object the 76-line P-08 unit produced**, i.e. the whole dead
body adds nothing to the frozen build.  `nm` on the two objects:

    ref  build/emacs-ref/src/sqlite.o   7736 bytes: T _Fsqlitep  T _Fsqlite_available_p
                                        T _syms_of_sqlite  s _Ssqlitep  s _Ssqlite_available_p
    port build/femacs/sqlite.o          3640 bytes: T _Fsqlitep  T _Fsqlite_available_p
                                        T _syms_of_sqlite  + the two `S*` statics, mangled

`size build/emacs-ref/src/sqlite.o` prints `234 112 0 5227 5573`; the reference file is
**not** the 336-byte empty object of `src/androidmenu.c`/`src/sound.c`/`lib-src/pop.c`, because its
tail is live (the ledger note above).  The port's extra undefined references (`_strlen`, `__setjmp`,
`_cons_to_signed`, `_cons_to_unsigned`, `_xfree`) come from the ungated `include/lisp.rs` text and
are the same class every ported unit carries (`nm build/femacs/xml.o` shows the eleven mangled
`__RNv…` text symbols too).  The two warnings are mirror-side (`include/lisp.rs` 3934,
`lib/include/stdbit.in.rs` 644) `unused doc comment`s, named rather than silenced.

② **The `<sqlite3.h>` measurement (P-88 ②).**  `clang -o /tmp/sqlite-lane/probe
/tmp/sqlite-lane/probe.c -I/opt/homebrew/opt/sqlite/include -lsqlite3 && /tmp/sqlite-lane/probe`,
against `/opt/homebrew/opt/sqlite/include/sqlite3.h` (SQLite 3.53.4):

    ptr sqlite3*=8 sqlite3_stmt*=8 int64=8
    OK=0 ROW=100 DONE=101 MISMATCH=20 LOCKED=6 BUSY=5
    INTEGER=1 FLOAT=2 BLOB=4 NULL=5 TEXT=3
    OPEN_READONLY=1 READWRITE=2 CREATE=4 URI=64 MEMORY=128 FULLMUTEX=65536
    DBCONFIG_ENABLE_LOAD_EXTENSION=1005 VERSION_NUMBER=3053004 VERSION=3.53.4

`ls /usr/include/sqlite3.h /opt/homebrew/include/sqlite3.h` finds neither, which is why the probe
names the SDK path it used; `$(xcrun --show-sdk-path)/usr/include/sqlite3.h` is the second copy on
this host.  Every constant in the unit carries the value measured here, not a remembered one.

③ **The `Lisp_Sqlite` layout measurement.**  `clang -std=gnu23 -I build/emacs-ref/src -I emacs/src
-I emacs/lib -I build/emacs-ref/lib -o /tmp/sqlite-lane/lay /tmp/sqlite-lane/lay.c`, which prints

    Lisp_Sqlite sizeof=48 align=8 off db=8 stmt=16 name=24 finalizer=32 eof=40 is_statement=41

and the port asserts all seven numbers (`std::mem::size_of`/`offset_of!`, on the mirror's record).

④ **The arm flip (P-86 ③⑴ ⑥), the name-and-type reading.**
`cargo run -q -p xtask -- probe src/sqlite.rs /tmp/sqlite-lane/sqlite.rlib --crate-type=lib
-C panic=abort --cfg 'feature="have_sqlite3"' --cfg 'feature="have_sqlite3_load_extension"'
--cfg 'feature="sqlite3_dbconfig_enable_load_extension"'
--cfg 'feature="sqlite_version_number_ge_3007015"'
--cfg 'feature="sqlite_version_number_ge_3006005"' --cfg 'feature="sqlite_open_fullmutex"'
--cfg 'feature="sqlite_open_uri"' --cfg 'feature="sqlite_open_memory"'`
-> **exit 0, 0 errors, 4 warnings, rlib 8,624,064 bytes**.  That is the file gate plus **every**
sub-gate except `windowsnt` (which no toolchain here can compile, Owed 7), so every arm of
C 31-856 is compiled.  All 4 warnings are `unused doc comment`: 2 unit-side (the `/// C:` citation
lines above the two `unsafe extern` blocks, `_root.rs` 457/1121 - named, not silenced) and 2
mirror-side.  The first flip of the revision reported **3 errors, all in `_root.rs`, all fixed in
this unit's text** (never in a mirror): C 402's `XFIXNUM` is `isize` where `sqlite3_bind_int64`
takes `i64` (Deviation 4), and C 516/C 607's `NULL` sits in a `*mut *const c_char` slot where
`std::ptr::null ()` has the wrong mutability (Deviation 5).
**Revision note:** the three comment-only corrections that followed (the C 1-23 sentence, the
`DEF_DLL_FN`/alias-block line counts) are `/* */` comments - with `//`/`///` lines untouched - so
they cannot change this reading, and the frozen object was re-measured byte-identical (sha256
`f7a85883…`) after them.

⑤ **The mutation reading (P-86 ⑦): the 0 above is not vacuous.**  The same command with
`row_to_value`'s `let len: c_int = sqlite3_column_count (stmt);` (C 426) mutated to
`let len: *mut sqlite3_stmt = …` -> **exit 1, 2 `E0308` errors, no rlib**; the unit was then
restored from a copy and re-checked by md5 (the copy was taken before the mutation).  So the include graph is not the abort class P-86 ⑦ warns about: the
flip reaches this unit's text and reports on it.

⑥ **`python3 tools/extract-abi.py --check emacs/src/sqlite.c src/sqlite.rs` -> `FAIL`**,
with exactly **one** problem line and no other:

    - line count drifted 1602.6%: C 77 live (of 933) vs Rust 1311

That is P-86 ⑩: the port now has `fn`s, so the check is no longer vacuous, and what it demands is
the `total`-basis row in `docs/ports/line-drift-tolerance.tsv` 145 (which is outside this lane's
write set, Owed 2).  The two live subrs' names, arities and docstring digests compare **clean** -
there is no `- ` line for them.  The checklist half of the reading is vacuous as coverage evidence,
which is why the range table above exists (P-86 ②/⑩).

⑦ **`python3 tools/check-cond-eval.py src/sqlite.c` -> `21 conditions, 0 unknown, 0 disagreements
[VERIFIED]`** - every condition agrees with clang; all 21 verdicts are `False`, because the whole
body is inside the false `#ifdef HAVE_SQLITE3` and the evaluator short-circuits there (which is why
the gate mapping above had to be decided by hand).

⑧ **`python3 tools/check-dead-file-coverage.py`** reads this table against `docs/port-map.md` 531's
single row and prints `info docs/ports/sqlite.md: 32 range row(s) cover src/sqlite.c (933 lines; 0
blank line(s) between blocks need no row)` - the union is contiguous, so every blank line is inside
a row - and the run's own summary `54 ledger(s) bound (a C range table), 0 failure(s)`.  The reading
is in the report's section ③.

⑨ **`python3 tools/check-citations.py`** - the tree-wide reading when this lane scanned it was
`676 file(s) scanned, 0 drift, 0 range, 720 external, 35 allowed, 0 stale`, with **no finding in
this unit**: the two mirror-name citations (`coding.h` 693-694, `<strings.h>` 81) and the
twenty-eight `<sqlite3.h>` cites resolve as external (the header is not in the tree) or are quoted
in full, and every `// C nn:` citation in the unit was taken from `emacs/src/sqlite.c` itself.  A
re-run at the end of the lane, with another lane's in-flight `src/gnutls.rs` in the tree,
reads `1 drift, 51 range, 730 external` - **every one of those 52 findings is in that other file**
(its new `gnutls.h` cites resolve to the 100-line `emacs/src/gnutls.h`), and this unit still
contributes none.

⑩ **`python3 tools/check-declarations.py` -> `10 declaration sites (0 forbidden, 10 allowed
literals), 0 failures`** - the tree baseline.  The first pass of this unit reported one `recital`
finding (a comment of C 1-23's sentence named what the head block contains); it was reworded to the
shape `src/sound.rs` 60 uses, and the class is what the gate exists to find.

## Deviations

1. **The include graph's two ungated edges.**  C 27's `<c-strcase.h>` and C 29's `"coding.h"` carry
   the file gate; C 25's `<config.h>` and C 28's `"lisp.h"` do not (Deviation 1's own reason is in
   `## Decisions`).  The same shape the other P-86 files use for `config.h`, with one addition:
   `lisp.h` is ungated here because the *live* tail itself needs it.  C's own text has both outside
   C 31's `#ifdef`, so no C statement changed.
2. **C 369's `Lisp_Object encoded;` is zero-initialised** (`= Qnil ()`): C assigns it on three of
   the four paths and reads it on the `!blob` one, which Rust's maybe-initialised analysis refuses
   (PORTING.md 1.5).  Unobservable: the one path that leaves it at `Qnil` is the one that reads
   `value` instead.
3. **C 322's `sqlite3 *sdb;` is zero-initialised** (`= std::ptr::null_mut ()`): C declares it and
   lets `sqlite3_open_v2`'s out-parameter write it, and `&mut sdb` needs a live value
   (PORTING.md 1.5).
4. **`XFIXNUM (value) as sqlite3_int64`** (C 402): the mirror's `XFIXNUM` returns `EMACS_INT`
   (`isize`) and C's callee takes `sqlite3_int64` (`i64`) - C's implicit conversion spelled out.
   (C 404's `bignum_to_intmax` already returns `intmax_t`, so it needs none.)
5. **`std::ptr::null_mut ()` for C's `NULL`** in `sqlite3_prepare_v2`'s `pzTail` slot (C 516,
   C 607): the parameter is `*mut *const c_char`, and Rust picks the pointee's mutability where C's
   `NULL` adapts.
6. **C 433's `switch` has no `default`**, so a column type outside its five cases leaves C 431's
   `Qnil` in `v`; the port's `match` spells that as `_ => {}`.
7. **C's `goto exit` cleanup paths are a labelled block** (`'exit: { … break 'exit; }`,
   PORTING.md 2.2) in `Fsqlite_execute` (C 526/536) and `Fsqlite_select` (C 614/625/633); both
   functions' fall-through into `exit:` and the cleanup-then-`xsignal1` tail are kept, and no path
   returns early to skip them.
8. **C 545-547's `do { … } while (sqlite3_step (stmt) == SQLITE_ROW)`** becomes `loop { …; if
   sqlite3_step (stmt) != SQLITE_ROW { break; } }` - the same first-iteration-before-test order.
9. **C 202's function-local `static bool sqlite3_initialized;`** stays function-local in Rust
   (`static mut sqlite3_initialized: bool = false;`, the `src/androidvfs.rs` 659 shape) with C's
   zero value; it is inside the `windowsnt` arm.
10. **The `<sqlite3.h>` surface is the unit's own** (P-88 ①): the reference tree has no such file,
    so no mirror is added and each declaration quotes the header line.  Two mirror gaps are closed
    the same way: `encode_string_utf_8` (`coding.h` 693-694, absent from `include/coding.rs`) and
    `strcasecmp` (`<strings.h>` 81, which no mirror owns - `src/term.rs` 209 and `src/sysdep.rs`
    731 declare it per unit too).  All three are Owed 5, for the mirror round.
11. **The Windows loader family is C text, not Rust** (C 52-99, C 100-157, C 159-195): the three
    macros are deferred tree-wide (`include/w32common.rs` 173-183, P-22).  `load_dll_functions` is
    therefore the one C definition without an item here, and C 210's call to it is the waiting site
    (P-91 ④).
12. **`HAVE_LOAD_EXTENSION` is C's *computed* macro**, not a configuration name: the port spells
    C 39's conjunction at each of its four consumers (C 91/189/710/901) instead of inventing a
    ninth feature whose value Rust cannot derive from the other two.
13. **`make_sqlite` spells C's two macros** (`ALLOCATE_PLAIN_PSEUDOVECTOR! (Lisp_Sqlite,
    pvec_type::PVEC_SQLITE)`, `make_lisp_ptr (ptr as *mut c_void, Lisp_Type::Lisp_Vectorlike)`),
    and the `void *db`/`void *stmt` fields carry `*mut c_void` as `lisp.h` 2917-2918 declares them.
14. **C's implicit conversions/narrowings are written out**: `ASIZE (values) as c_int`,
    `list_length (values) as c_int`, `SBYTES (…) as c_int`, `sqlite3_changes (sdb) as EMACS_INT`,
    `code as EMACS_INT`, and `name.offset (allow_len)` for C's `name + allow_len` (C 759-763) -
    together with the two `// C nn:` counts corrected in the unit's own comments after the readings
    (Evidence ④'s revision note).
15. **C 756's `for (const char **allow = allowlist; *allow; allow++)`** (a NULL-terminated array)
    is a `while` with an index over the same array, `! allowlist[allow_i].is_null ()` being C's
    `*allow` test; `strncmp (…) == 0` is C's `!strncmp (…)`.
16. **The eight docstrings are carried byte-for-byte**, trailing double spaces included.  The
    pre-revision unit had dropped them (the docstring digest normalises whitespace, so
    `extract-abi --check` could not see it) - recorded here because it was a real fidelity defect,
    not a style choice.

## Owed by the integrator (status per item; nothing here is claimed as done until marked)

1. **`xtask/Cargo.toml`'s feature registration** for the eight new names:
   `have_sqlite3` (HAVE_SQLITE3), `have_sqlite3_load_extension` (HAVE_SQLITE3_LOAD_EXTENSION),
   `sqlite3_dbconfig_enable_load_extension` (`<sqlite3.h>`'s SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION),
   `sqlite_version_number_ge_3007015` / `sqlite_version_number_ge_3006005` (SQLITE_VERSION_NUMBER),
   `sqlite_open_fullmutex` / `sqlite_open_uri` / `sqlite_open_memory` (the three `SQLITE_OPEN_*`).
   `windowsnt` is already there.  Until they land, a Cargo-driven build cannot pass
   `--cfg feature="have_sqlite3"` and `unexpected_cfgs` will warn; the readings above used
   `rustc --cfg` directly, which needs no registration.  **DONE (status 3.487, integrator):** the eight
   names are in `xtask/Cargo.toml`'s `[features]` table at their alphabetical positions, each
   carrying its C judgement line (`have_sqlite3` cites macros.tsv 1551 = undef), and
   `python3 tools/feature-coverage.py --write` now lists all eight.
2. **`docs/ports/line-drift-tolerance.tsv`'s row for `sqlite`.**  The old row (145) is the
   `live`-basis `0.3` written for the empty unit; a `total`-basis row must be appended - C 933
   total vs Rust 1311 lines (**40.4% drift**, this table's `measured + 0.01` shape -> 0.41) - and
   the old row stays (append-only, the newest governs).  Until then `extract-abi --check` FAILs
   with that one line (Evidence ⑥).  **DONE (status 3.487, integrator):** the `total`/`0.41` row is
   appended (row 145's `live` row stays - append-only, newest governs), and
   `extract-abi --check emacs/src/sqlite.c src/sqlite.rs` is now **OK with zero `-` rows**.
3. **`docs/baseline/port-lines-pending.tsv`'s `src/sqlite.c` row** must leave the pending list via
   `python3 tools/check-port-lines.py --write-pending`; that file is outside this lane's write set.
   **DONE / no-op (status 3.487, integrator):** `src/sqlite.c` was never on that list - its target
   has always been over the ten-percent threshold, so the ratchet never carried it.  The
   `--write-pending` run did drop the three rows of the files being sliced right now
   (`src/gnutls.c`, `src/androidfns.c`, `src/kqueue.c`): 61 rows with 3 stale -> **58 rows, 0 stale,
   58 file(s) under ten percent / 74,423 reference lines**.  If one of those slices is ever reverted
   to a stub, its row must be written back.
4. **`docs/baseline/macro-body-notes.tsv`: no row is owed.**  Measured, not assumed:
   `extract-abi.py --check` printed **no `- ` line at all** except the line count, and the unit
   contains no code `goto` (its five `goto` tokens are all inside `// C …: \`goto exit;\``
   comments), so the `goto` token class that gave `src/androidmenu.c` and `src/sfnt.c` rows is not
   reached here.  **NO ACTION NEEDED.**
5. **The mirror gaps this file names**: `encode_string_utf_8` belongs in `src/include/coding.rs`
   (its `coding.h` 693-694 declaration), and `strcasecmp` needs whatever mirror the tree picks for
   `<strings.h>` - three units declare it in-unit today (`src/term.rs` 209, `src/sysdep.rs` 731, this
   one), so that is a mirror round rather than a one-file edit (P-88's move-back shape).
   `<sqlite3.h>` and `<windows.h>` have no mirror by design (P-88 ① / the `nt/inc/include/` rule).
   **NOT DONE.**
6. **The next definition this file owes is `load_dll_functions` (C 159-195)**, whose 28 statements
   are all `LOAD_DLL_FN` expansions; **C 210 in `init_sqlite_functions` is the site that waits on
   it** (P-91 ④).  It cannot be written before P-22 resolves `DEF_DLL_FN`/`LOAD_DLL_FN`'s Rust
   shape (`include/w32common.rs` 173-183), which is also what `docs/ports/include-w32common.md`'s
   three deferred macro rows wait on.  Until then a `windowsnt` flip of this unit reports `E0425`
   for that one name - the only missing item in the file.  **NOT DONE.**
7. **The `windowsnt` arm has no compile reading at all** (no Windows and no mingw toolchain on this
   machine: `which -a x86_64-w64-mingw32-gcc i686-w64-mingw32-gcc mingw32-gcc` is empty), so C 46-196,
   C 201-221 and C 874-882 are `text-only`; nothing here claims they compile.  **NOT DONE
   (by design).**
8. **`docs/baseline/feature-coverage.tsv`** must be regenerated (`python3 tools/feature-coverage.py
   --write`) once Owed 1 lands; it will report the eight names as "enabled by -" until then, which
   is the truthful reading for a file whose gates no configuration exercises.  **DONE (status 3.487,
   integrator):** regenerated - **848 feature(s), 841 never enabled**; the table is a reading of the
   whole tree, so it also carries the gates of the units in flight and its counts move again when
   they land.

## Not claimed

- **Zero run evidence.**  The rlib is a *compile* reading (8,624,064 bytes that no one linked):
  no database was opened, no statement prepared, bound, stepped or finalized, no extension loaded,
  no Lisp call made.
- **Not a SQLite build.**  The flip compiles against this unit's own `<sqlite3.h>` stand-in
  (Deviation 10), not against a real header, and nothing links `libsqlite3`: the "0 errors" is a
  name-and-type reading of the unit's text plus the port's mirror set (P-86 ⑦), which is weaker
  than a build against the library.
- **The frozen object's reading is the *gate's*, not the body's.**  It is byte-identical to the
  empty-unit port's, which is exactly the point (the whole body is inside the gate) - so it says
  nothing whatever about the body.
- **`extract-abi --check` does not pass**, and the one reason is Owed 2's missing tolerance row,
  not a content mismatch: the two live subrs' names, arities and docstring digests all compare
  clean (Evidence ⑥).
- **The `windowsnt` arm is carried text with no reading** (Owed 7), and `load_dll_functions` is
  not present as an item (Owed 6) - the arm is as faithful as the reference's text and no more.
- **`docs/ports/sqlite.md`'s own numbers are not measured by any gate**; the `## C line ranges`
  table and Evidence ⑤ are what `check-dead-file-coverage.py` and the flip read.
- This ledger does not claim the earlier P-08 note was wrong about its *measurements* (the empty
  live checklist, the four `#else`-only live bodies); it withdraws its *conclusion* (that the text
  need not be written), and it corrects one thing the brief expected of it: the frozen object of
  this file is **not** 336 bytes (Evidence ①), because C 858-933 is live.
