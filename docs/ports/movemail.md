# emacs/lib-src/movemail.c (871 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [x] main  (line 164)
- [x] fatal  (line 548, static)
- [x] error  (line 560, static)
- [x] pfatal_with_name  (line 573, static)
- [x] pfatal_and_delete  (line 579, static)

## Macros defined — 2
- [x] ARGSTR  (line 185, body d0d0de99229a)
- [x] MAIL_USE_SYSTEM_LOCK  (line 119, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 1
- [x] static char delete_lockname  (line 161, static)

## Decisions

Port of lib-src/movemail.c, complete for the frozen configuration: five functions (`main`, the
three static error helpers and `pfatal_with_name`/`pfatal_and_delete`) plus the
`delete_lockname` object - the eight ledger boxes are ticked.  It is a live program (the
reference `lib-src/Makefile` lists movemail under `UTILITIES`), the third real batch-F unit.

**Dead text, recorded not transcribed** (P-08).  `python3 tools/check-cond-eval.py
lib-src/movemail.c` reports `37 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the
ledger's live set is the frozen configuration's:

1. `MAIL_USE_POP` is undefined: the whole POP half is not compiled - `#include "pop.h"` (C 84-86),
   C 240-244's branch in `main`, and C 616-871 (`popmail`, `pop_retr`, `mbx_write`,
   `mbx_delimit_begin`, `mbx_delimit_end`, `movemail_strftime`).
2. `MAIL_USE_FLOCK` is 1, so `MAIL_USE_SYSTEM_LOCK` is defined: the lock-file machinery of
   C 252-330 (the `EXXXXXX` temp file, `link`, the five-minute stale-lock rule, `xmalloc`) and
   the `#ifndef MAIL_USE_SYSTEM_LOCK` tail of C 474-486 are dead; the live lock is
   `flock (indesc, LOCK_EX)`.
3. `MAIL_USE_MAILLOCK`/`HAVE_LIBMAIL`/`HAVE_MAILLOCK_H` are undefined: `mail_spool_name`
   (C 511-547) and the `maillock`/`touchlock`/`mailunlock` arms are dead.
4. `WINDOWSNT`, `MSDOS`, `DISABLE_DIRECT_ACCESS` and `MAIL_UNLINK_SPOOL` are undefined.
   `lockname` and `spool_name` are therefore null at every live use, exactly as in C.

**Deviations.**

1. **Use-site handling** (P-26/P-72 (3)): `flock` and `LOCK_EX` (`<sys/file.h>`; `LOCK_EX`
   measured as 2), `mkostemp` (only the dead arm calls it), `waitpid`, `fork`, `setuid`,
   `setregid`, `exit`, and `SIG_DFL` (`signal.rs` keeps the handler as a `usize`, its own note
   says why).  `WIFEXITED`/`WEXITSTATUS` are the two inline functions `src/callproc.rs` 267-268
   established, with the same measured bodies (`(status & 0o177) == 0`, `(status >> 8) & 0xff`).
2. **The backward `goto retry_lock`** (C 407) is a labelled loop with `continue 'retry_lock`
   (PORTING.md 2.2); `docs/ports/macro-body-notes.tsv` carries the row for the missing `goto`
   token.
3. **Explicit conversions**: `setregid (-1_i32 as gid_t, …)` writes C's `(gid_t) -1`, and
   `strerror (*errno ())` is the port's spelling of `strerror (errno)`.
4. **`ARGSTR`** is a `&CStr` constant holding the live `"p"` (the `MAIL_USE_POP` arm would add
   `r`), and `MAIL_USE_SYSTEM_LOCK` is the `bool` constant C's empty `#define` stands for.
5. **Line-count basis `live`**: the C file is 871 lines but a third of it is dead by
   configuration, so the tolerance row declares the `live` basis (docs/ports/line-drift-tolerance.tsv).

Evidence: `cargo run -q -p xtask -- build-tu lib-src/movemail.c` ->
`built lib-src/movemail.rs -> ../build/femacs/movemail.o`; `python3 tools/extract-abi.py --check
emacs/lib-src/movemail.c femacs/lib-src/movemail.rs` -> `OK` (8/8 boxes, two notes rows in
place); `rustc --cfg enable_checking` on the materialized unit -> exit 0; `check-declarations` 0;
`check-verbatim` 6 trees / 0 failures.  The token gate earned its keep here: it caught the port
dropping C's `strerror (errno)` in `main`'s two `fatal` calls (a wrong-but-compiling pointer
cast), which the first `--check` reported as `function main does not mention strerror`.
