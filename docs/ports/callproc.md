# emacs/src/callproc.c (2262 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 3
- [x] call-process  (Fcall_process, 1/MANY, doc 04456e39b1a1, line 272)
- [x] call-process-region  (Fcall_process_region, 3/MANY, doc ae6a6acb80d2, line 1063)
- [x] getenv-internal  (Fgetenv_internal, 1/2, doc f65f97a0358e, line 1765)

## Functions defined — 21
- [x] get_current_directory  (line 158)
- [x] record_kill_process  (line 196)
- [x] delete_temp_file  (line 216, static)
- [x] call_process_kill  (line 222, static)
- [x] call_process_cleanup  (line 246, static)
- [x] call_process  (line 355, static)
- [x] create_temp_file  (line 962, static)
- [x] add_env  (line 1164, static)
- [x] exec_failed  (line 1206, static)
- [x] child_setup  (line 1236, static)
- [x] emacs_posix_spawn_init_actions  (line 1328, static)
- [x] emacs_posix_spawn_init_attributes  (line 1370, static)
- [x] emacs_spawn  (line 1441)
- [x] getenv_internal_1  (line 1681, static)
- [x] getenv_internal  (line 1716, static)
- [x] egetenv_internal  (line 1799)
- [x] make_environment_block  (line 1816)
- [x] init_callproc_1  (line 1950)
- [x] init_callproc  (line 1969)
- [x] set_initial_environment  (line 2070)
- [x] syms_of_callproc  (line 2082)

## Macros defined — 4
- [x] CHILD_SETUP_TYPE  (line 141, body 6a5dff1eae5e)
- [x] USABLE_POSIX_SPAWN  (line 49, body b6589fc6ab0d)
- [x] _P_NOWAIT  (line 63, body 356a192b7913)
- [x] synch_process_tempfile  (line 119, body 36e406a91c35)

## Types and their fields — 0

## File-scope objects — 4
- [x] extern char environ  (line 29)
- [x] static Lisp_Object Vtemp_file_name_pattern  (line 100, static)
- [x] static pid_t synch_process_pid  (line 113, static)
- [x] static Lisp_Object synch_process_tempfile  (line 117, static)

# ----------------------------------------------------------------------
## Decisions

`emacs/src/callproc.c` @ `fd7afced403`; port `femacs/src/callproc.rs` (2,441 lines vs C's
2,262, +7.9%, declared in `docs/ports/line-drift-tolerance.tsv` on the **`total`** basis —
the dead text is carried, not recorded).  `src/callproc.c` is in
`docs/baseline/frozen-build-sources.txt`, so the frozen configuration compiles it and the
full comparison applies (no 3.31 exemption).  The gates:

    build-tu src/callproc.c      -> built src/callproc.rs -> ../build/femacs/callproc.o
    extract-abi.py --check       -> OK emacs/src/callproc.c -> femacs/src/callproc.rs
    check-symbols.py --pending   -> check-symbols: 78 units compared, 0 mismatched, 1 still pending
    check-globals.py             -> check-globals: 241 units, 0 DEFVAR_* variables without a definition in their own unit
    ledger                       -> 32/32 rows checked, 0 unchecked

An extra compile with `--cfg enable_checking` (the repo's step 6) also succeeds
(`rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort -C codegen-units=1
-C opt-level=0 --cfg enable_checking …/_root.rs`), as P-57's class of defect requires.

### The condition table: this file is *not* verified, and its one disagreement is gone

When this lane wrote the section below, `python3 tools/check-cond-eval.py --show-unknown
src/callproc.c` reported `75 conditions, 0 unknown, 1 disagreements [NOT VERIFIED]`:

    DISAGREE src/callproc.c:1561 `ifdef TIOCSCTTY` — clang True, evaluator False

`TIOCSCTTY` comes from `<sys/ioctl.h>` (through systty.h's `<termios.h>`), **not** from
`config.h`, so the frozen macro table could not see it — docs/status.md 3.35's class
(`source-header-macros.tsv`).  The integrator added that row, and the file now reports
`75 conditions, 0 unknown, 0 disagreements [VERIFIED]`.  It is nevertheless **still not** added
to `cond-eval-verified.txt`, for the reason the next paragraph gives: this port carries both arms
of every `#if` (status 3.49), and a verified file's checker demands only the live one (status
3.51) - the two are mutually exclusive.  Consequences, in order of
importance:

1. Because the file is *not* in `cond-eval-verified.txt`, `tools/extract-abi.py`'s
   `blind_conditions` keeps **both** arms of every `#if` live and the per-function token
   check demands the tokens of C's text on both sides.  Per the integrator's 2026-10-03
   decision (docs/status.md 3.49, PORTING.md P-51) the port therefore **carries** the text
   of the arms the frozen build does not compile, each under a
   `#[cfg (feature = "…")]` naming the C condition's non-frozen side; the live arm is
   ordinary text (or, for an arm pair, `#[cfg (not (feature = …))]`, the timefns shape).
   The features are never set: `xtask dead-arms` can only flip `config.rs` keys and these
   are this file's platform/preprocessor predicates (docs/status.md 3.9).
2. The one arm clang compiles and the evaluator drops is `#ifdef TIOCSCTTY` (C 1561-1563):
   the port keeps C 1561's `ioctl (std_in, TIOCSCTTY, 0)` as **live** text with the
   target's measured value (536900705 = 0x20007461, measured with the frozen clang), so the
   port compiles exactly what the frozen build compiles.  This is the single place the
   port's live/dead split follows clang over the evaluator; the blind evaluator requires
   the tokens either way, and the disagreement is reported to the integrator (the
   `source-header-macros.tsv` route would let the table decide it).

The carried arms, in C order (each `#[cfg]` attribute carries the C line numbers in the
port):

- `msdos` (C 28-30, 66-69, 83-85, 116-117, 372-373, 514-516, 557-627, 628-664, 1239-1243,
  1295-1317, 2091-2094; plus C 54-57's SETUP_SLAVE_PTY include and `SETUP_SLAVE_PTY`
  itself), `windowsnt` (C 59-64 with `_P_NOWAIT`, 992-1005, 1244-1246, 1268-1275,
  1513-1551, 1655-1656, 1689-1691, 1726-1735, 2050-2053), `dos_nt` (C 266-267, 975-977,
  1832-1838, 2059-2061), `have_android` (C 168-180, 1446-1455, 2039-2043 and the five
  program-name arms 2177-2237), `have_ns`/`have_pgtk` (C 87-89, 91-93, 1882-1890),
  `no_subprocesses` (C 396-400), `ldisc1`/`nttydisc`/`no_dont_reopen_pty`/
  `setup_slave_pty` (C 1567-1575, 1577-1583, 1587-1613, 1616-1621), `no_darwin_os`
  (C 1535-1537, the plain `vfork`), `no_posix_spawn_addchdir` (C 1357-1359),
  `no_usable_posix_spawn` (C 49-50), `no_sc_arg_max` (C 2250-2251).
- `#include` selections (PORTING.md 7.2's third category, recorded here and not in code):
  C 54-57 (`<sys/stream.h>`/`<sys/stropts.h>`, SETUP_SLAVE_PTY), C 59-64 (`<sys/socket.h>`,
  `<windows.h>`, `"w32.h"`), C 66-69 (`<sys/stat.h>`, `<sys/param.h>`), C 83-85
  (`"msdos.h"`), C 87-89 (`"nsterm.h"`), C 91-93 (`"pgtkterm.h"`), C 95-97
  (`"android.h"`), C 37-50 (`<spawn.h>`; the live arm, declared at the use site).  Every
  live include has a mirror and is in the unit's include block, in C's order with the
  transitive mirrors the C headers pull in (`itree.rs` and `character.rs` for buffer.h,
  `composite.rs` for coding.h, `dispextern.rs`/`termhooks.rs`/`font.rs`/`systime.rs`
  for frame.h, `window.rs` for keyboard.h, `mini-gmp.rs`/`bignum.rs` for lisp.h's bignum, and
  `<epaths.h>`'s values as the measured constants below).

### The `#if`s that are constant conditions of the frozen configuration (live text)

C 198/250/671/922/1258/1278/1982 (`#ifndef MSDOS`/`#ifndef DOS_NT`), C 1322/1458/1487/1666
(`USABLE_POSIX_SPAWN`, live = 1), C 1387/1397/1404/1624/1634/1641 (DARWIN_OS, SIGPROF — all
live), C 1555 (`HAVE_PTYS`), C 1645 (`subprocesses`), C 1741/1745/1760/1853/1863/1876/1913
(`#ifndef HAVE_ANDROID`/`#ifndef HAVE_PGTK`), C 2247 (`_SC_ARG_MAX`) — all are ordinary
Rust text, because the frozen configuration compiles them.

### Statement-level deviations (each is a shape Rust cannot spell the same way)

1. **C's `goto`** becomes the labelled block/loop PORTING.md 2.2 asks for:
   `emacs_posix_spawn_init_actions`/`_attributes`' `goto out` is `'out: loop` +
   `break 'out`, `emacs_spawn`'s `goto fork_done` is `'fork_done: loop` +
   `break 'fork_done`, and `call_process`'s `goto give_up` is `'give_up: loop` +
   `break 'give_up` (the `continue` that restarts `while (1)` is `continue 'give_up`).
   The `goto` keyword has no Rust site: the four `macro-body-notes.tsv` rows record it.
2. **C's `?:` chains** become `if`/`else` (`error_file`'s stderr test, the
   `STRINGP (error_file) ? … : …` inside `emacs_open`, `outf ? outf : "/tmp/"`,
   `sh ? sh : "/bin/sh"`, `signame == 0`, `CONSP (coding_systems) ? … : …`,
   `pid < 0 ? errno : 0`).
3. **Bitfields**: `record_kill_process`/`call_process_kill` read and write
   `Lisp_Process.alive` through process.rs's `PROCESS_ALIVE_SHIFT` (`bits2`), and
   `call_process` reads and writes `coding_system`'s `common_flags`/`mode`/
   `src_multibyte`/`dst_multibyte` through coding.rs's `CODING_SYSTEM_*_SHIFT`/`_MASK`
   (PORTING.md 1.3: the storage unit is what the gate can assert).  Recorded in
   `docs/ports/macro-body-notes.tsv` (the bytecode/data.c `redirect`/`trapped_write`
   shape).  **Mirror gap for the integrator**: `coding.rs`'s comment says its three
   bitfield *writers* "land with coding.c", so the port writes `bits` directly; no mirror
   was edited.
4. **C's struct assignment** `struct coding_system saved_coding = process_coding;` is a byte
   copy, and `coding_system` has no `Copy` in the mirror, so the port spells the copy as
   `std::ptr::read (&process_coding)` (the source stays valid, exactly as C's copy leaves
   it), and the later `process_coding = saved_coding;` the same way.  This is the second
   half of the mirror gap above.
5. **`volatile`** (the `emacs_spawn` vfork clobber-prevention locals) has no Rust
   counterpart: the port keeps C's names and its eleven saving/restoring statements and
   drops only the qualifier (PORTING.md 8's "volatile … no counterpart" class).
6. **C's uninitialised locals are zero-initialised** (PORTING.md 1.5), each named in the
   port's comment where C merely declares: `path`, `callproc_fd[]`, `status`, `oldset`,
   `process_coding`, `argument_coding`, `buf[]`, `saved_coding` (via item 4),
   `tempfile` (MSDOS), `actions`/`attributes`, `outf`, `environ`'s loop variable,
   `value`/`valuelen` (twice), `pwd_var`'s `temp`, and `infile`.  `CALLPROC_BUFFER_SIZE_MIN`
   and `_MAX` are Rust `const`s (C's function-local `enum`) because an array length needs a
   constant.
7. **`SAFE_NALLOCA`/`USE_SAFE_ALLOCA`/`SAFE_FREE`/`SAFE_FREE_UNBIND_TO`** keep the port's
   P-05 signatures (`sa_avail` is the extra first argument of `SAFE_NALLOCA!`), and the
   three `args2`/`new_argv` bindings the macro passes by value start at
   `std::ptr::null_mut ()` (lisp.rs's `pointee_size` note).
8. **`synch_process_tempfile`** is C's `#define … make_fixnum (0)` on this target and a
   `Lisp_Object` object on MSDOS.  The port keeps the live arm as P-17's same-named inline
   fn (`fn synch_process_tempfile () -> Lisp_Object`) and carries the MSDOS object as
   `#[cfg (feature = "msdos")] static mut synch_process_tempfile`, exactly as the C's two
   arms are mutually exclusive.
9. **`CHILD_SETUP_TYPE`** (C 138-142) spells the child's return type — `_Noreturn void`
   here, `int` on DOS_NT (which is also what makes C's WINDOWSNT/MSDOS arms `return cpid;`
   legal).  Rust cannot alias `!`, so the port writes `-> !` and records the macro in
   `docs/ports/excluded-macros.txt` (the `AVOID` row's class); the carried arms' `return`s
   are `#[cfg]`-stripped.
10. **`staticpro`/`defsubr`** keep the port's pointer spellings (`&raw const`/`&raw mut`),
    and the three `defsubr` calls are C's own order.
11. **The `DEFVAR_*` storage** (§3.29): the seventeen variables this C file owns are
    defined here as `#[unsafe(no_mangle)] pub static mut` (`lisp_h_Qnil ()` for the
    Lisp_Objects, `0` for `command_line_max_length`), so `check-globals.py` sees the
    owners; `globals.rs` carries the declarations and the unit's definitions shadow the
    glob import (P-21).  `Vtemp_file_name_pattern` is C's file-`static` object and stays
    non-`pub`, without a symbol.
12. **`DEFVAR_*!` takes the docstring positionally** (the mirror's `$doc` has no `doc:`
    label), so the C `doc:` token has no Rust site: the `syms_of_callproc` row of
    `docs/ports/macro-body-notes.tsv` records that (the `syms_of_timefns` row's shape), and
    the names are `c"…"` (status 3.38).
13. **Measured constants** (each measured with the frozen clang / read from the frozen
    build, and each with the measurement in its comment): `POSIX_SPAWN_SETSID` 1024,
    `POSIX_SPAWN_SETSIGDEF` 4, `POSIX_SPAWN_SETSIGMASK` 8,
    `sizeof (posix_spawn_file_actions_t)` = `sizeof (posix_spawnattr_t)` = 8 (they are
    pointers on Darwin), `TIOCSCTTY` 536900705, `STDIN_FILENO`/`STDOUT_FILENO`/
    `STDERR_FILENO` 0/1/2, `_SC_ARG_MAX` 1, `SIG_DFL` 0, `NULL_DEVICE` "/dev/null", and the
    five `<epaths.h>` values (`PATH_DATA`, `PATH_DOC`, `PATH_EXEC`, `PATH_INFO`,
    `PATH_DUMPLOADSEARCH`; `PATH_GAME` is the null pointer).
14. **`IS_DIRECTORY_SEP`/`IS_DEVICE_SEP`/`DIRECTORY_SEP`** come from the frozen
    `config.h` (120, 2095, 2097: `'/'`, `0`, `(_c_) == DIRECTORY_SEP`) and have no mirror,
    so they are declared at the use site with C's macro parameter name `_c_`; the same
    block declares `WIFSIGNALED`/`WTERMSIG`/`WIFEXITED`/`WEXITSTATUS` (the *system*
    `<sys/wait.h>` spellings the frozen table records — syswait.h's `#ifndef` fallbacks are
    dead with `HAVE_SYS_WAIT_H` 1).
15. **Use-site declarations** (the terminal.rs/filelock.rs shape, none of them mirrored):
    `expand_and_dir_to_file`, `file_accessible_directory_p`, `remove_slash_colon`, `openp`,
    `setup_coding_system`, `raw_text_coding_system`, `complement_process_encoding_system`,
    `decode_env_path`, `dir_warning`, `write_region`, `wait_for_termination`,
    `record_deleted_pid`, `child_signal_init`, `child_setup_tty`,
    `dissociate_controlling_tty`, `restore_nofile_limit`, `emacs_perror`, `emacs_pipe`,
    `emacs_exec_file`, `emacs_open_noquit` (carried arm only), `synchronize_system_messages_locale`,
    `record_unwind_protect_nothing`, `set_unwind_protect`, `clear_unwind_protect`,
    `labeled_restrictions_remove_in_current_buffer`, `insert_1_both`, `del_range_2`,
    `del_range`, `redisplay_preserve_echo_area`, `emacs_unlink`, `nconc2`, `kill`,
    `mkostemp`, `strerror`, `strcat`, `strsignal`, `setpgid`, `tcsetpgrp`, `ioctl`, `fork`,
    `VFORK`, `_exit`, `environ`, and the ten `<spawn.h>` entries.  `CHILD_SETUP_ERROR_DESC`
    is *not* declared: `lisp.rs` already carries it (`&str`, so the call site casts).
16. **The unwind callbacks** `delete_temp_file`, `call_process_kill` and
    `call_process_cleanup` are `unsafe extern "C" fn` (their addresses go into
    `set_unwind_protect`/`record_unwind_protect*`), which is the shape the mirror's
    `close_file_unwind` has; C's `static` keeps them unexported.

### Macro-body notes

`docs/ports/macro-body-notes.tsv` gains eleven rows for this unit: `record_kill_process`
and `call_process_kill` (`alive`), `call_process` (`common_flags,dst_multibyte,goto,mode,
src_multibyte`), `emacs_posix_spawn_init_actions` / `emacs_posix_spawn_init_attributes` /
`emacs_spawn` (`goto`), `getenv_internal` (`1`, C's `? 1 : 0` against the port's bool),
`init_callproc_1` / `init_callproc` (`0`, C's `decode_env_path (…, 0)` against the mirror's
`bool`), and `syms_of_callproc` (`doc,have_mailutils` — the positional docstring, and
C 2211-2212's second physical line, whose token survives the preprocessor-line drop while
the frozen table has `HAVE_MAILUTILS` undefined).  `docs/ports/excluded-macros.txt` gains
`CHILD_SETUP_TYPE`.  `docs/ports/line-drift-tolerance.tsv` gains the `callproc` row (0.10,
`total`).  No `token-aliases.tsv` family was needed (`memcpy`'s four sites and `sizeof`'s
three are spelled with the family's canonical forms).

### Changes outside the unit (for integrator review)

1. `docs/ports/excluded-macros.txt`: one row (`CHILD_SETUP_TYPE`, kind=syntax).
2. `docs/ports/macro-body-notes.tsv`: the eleven rows above (appended).
3. `docs/ports/line-drift-tolerance.tsv`: the `callproc` row (appended).
4. Nothing else: no mirror, no `xtask`, no baseline file, and no tool was edited.  The two
   mirror gaps found (`coding_system`'s missing `Copy` and its three bitfield writers) are
   recorded in deviations 3 and 4 rather than fixed here, as is the `TIOCSCTTY`
   disagreement (reported for the integrator's `source-header-macros.tsv`).

## Integrator note (2026-10-03, after the port landed)

The gap this ledger reports - `TIOCSCTTY` coming from `<sys/ioctl.h>` rather than
`config.h`, so the frozen macro table could not decide `#ifdef TIOCSCTTY` - is fixed in
`docs/baseline/config/source-header-macros.tsv` (a `sys/ioctl.h\tTIOCSCTTY` row, measured by
`tools/gen-cond-macros.py`'s clang probe as `_IO('t', 97)`), and `check-cond-eval.py
src/callproc.c` now reports `75 conditions, 0 unknown, 0 disagreements [VERIFIED]`.

`src/callproc.c` is nevertheless **not** added to `docs/baseline/config/cond-eval-verified.txt`,
and that is deliberate: this port carries both arms of every `#if` (status 3.49's treatment for
a file the evaluator cannot decide), and a *verified* file's checker demands only the live arm -
which would then make `USABLE_POSIX_SPAWN`'s `#define ... 1` differ from the port's
`bool = true` spelling (the `c_int = 1` form the other guard macros use).  Staying outside the
list keeps the stricter, both-arms comparison, which is what this file's port actually
satisfies.


## 2026-10-04: `VFORK` is `conf_post.h`'s macro, not a symbol (3.251)

C 1534's `VFORK ()` is `conf_post.h` 280-291's macro; `DARWIN_OS` is defined, so the live body is
`vfork ()`, and the reference's own object says so (`nm -u build/emacs-ref/src/callproc.o` names
`_vfork`).  This unit used to declare `fn VFORK () -> pid_t` instead, which is a symbol no C object
references and no library defines - the first real link refused it.  Now: the call site is
`VFORK! ()` from `include/conf_post.rs`, `<unistd.h>`'s `vfork` is declared next to the file's
`fork` (the same shape `sysdep.rs` uses), and the invented declaration is gone.  Evidence:
`build-tu src/callproc.c` built, `--check` OK, name absent from the link's undefined list
(`docs/ports/link.md`, reading three).

## 2026-10-05: the three docstrings that named the reference as the program they come with (3.503)

`data-directory`, `doc-directory` and `shared-game-score-directory` (C 2120, 2124, 2135) said the
files "come with GNU Emacs"; the subject of that clause is the program that ships them, i.e. *this*
one, so the rule's seventh half (PORTING.md 0.1-19) writes this repository's own name instead.
Readings: `extract-abi --check emacs/src/callproc.c femacs/src/callproc.rs` stays **OK** - a
`DEFVAR_LISP` docstring is not compared by any gate (only a `DEFUN`'s digest is, `extract-abi.py`
1724-1727), which is why the three `literal` rows in `docs/baseline/declarations-allowlist.tsv`
could be deleted rather than re-reasoned: `check-declarations` falls 10 -> 7 rows on this change
alone, and the corrected reason for the one docstring that *stays* (`src/emacs.rs`,
`Vemacs_bugreport`) is recorded in the same commit.
