# emacs/src/systty.h (93 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 2
- [x] CDISABLE  (line 50, body e2a45b084d77)
- [x] EMACS_SYSTTY_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 1
- [x] emacs_tty: struct termios main

## File-scope objects — 0

## Decisions

Mirror of src/systty.h, complete for the frozen target (`DOS_NT`, `HPUX`, `AIX` undefined).

1. **Includes.**  `<termios.h>` → the new system mirror `lib/include/termios.rs`
   (measured: `sizeof (struct termios)` 72, offsets 0/8/16/24/32/56/64, `NCCS` 20);
   `<fcntl.h>`/`<unistd.h>` have mirrors.  `<sys/ioctl.h>` has none yet, and nothing in this
   header names one of its requests - the units that need `TIOCGWINSZ` and friends carry them at
   the use site (docs/ports/include-termios.md records the convergence plan).
2. **`CDISABLE` is a value, not a token.**  C's body is the *system* macro `_POSIX_VDISABLE`,
   which the frozen target defines as `((unsigned char)'\377')` (macros.tsv) - measured 255.  The
   mirror writes `pub const CDISABLE: cc_t = 255;`, the same form the two units that already
   spell it use (`src/process.rs` 7417, `src/sysdep.rs` 624), and
   `docs/ports/macro-body-notes.tsv` carries the exemption row (the token check's escape hatch
   for exactly this class).
3. **`struct emacs_tty`** holds one `struct termios main` (the `DOS_NT` arm is dead); the mirror
   asserts the measured size (72).
4. **Declarations.**  `emacs_get_tty`/`emacs_set_tty`/`suppress_echo_on_tty` are defined by
   src/sysdep.c (ported), `serial_open`/`serial_configure` by src/sysdep.c's serial section;
   they live in `systty_decls` + glob so the defining unit shadows the import (P-21).  C's
   `EXTERNALLY_VISIBLE` on the first two is an attribute the port drops (PORTING.md 0.2's
   attribute rule).  `struct Lisp_Process *` is an incomplete type in C here, so the port writes
   `*mut c_void` with the C prototype above it (P-72 (3)).

Evidence: `extract-abi.py --check emacs/src/systty.h src/include/systty.rs` -> `OK`
(3/3 ledger items); `layout-check` passes with both mirrors in the probe.
