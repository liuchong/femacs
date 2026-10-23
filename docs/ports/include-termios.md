# emacs/… the frozen target's <termios.h> (system header, P-26/P-50)

`lib/include/termios.rs`.  gnulib has no `termios` wrapper in this build and the
reference includes the system header, so this is a system mirror: the four typedefs
(`tcflag_t`/`speed_t` = `unsigned long`, `cc_t` = `unsigned char`), `NCCS`, and
`struct termios` with the layout measured from clang, each offset asserted:

    sizeof=72 iflag=0 oflag=8 cflag=16 lflag=24 cc=32 ispeed=56 ospeed=64 NCCS=20

## What it carries now, and what is still local

The mirror carries exactly what `src/systty.h` names: the typedefs, `NCCS` and the struct.  The
`tcgetattr`/`tcsetattr`/`tcflow`/`tcflush`/`tcgetpgrp`/`tcsendbreak` entry points and the ioctl
request numbers are **not** here yet, because three units carry measured copies of their own:

| unit | carries | lines |
| --- | --- | --- |
| `src/sysdep.rs` | `tcflag_t`/`speed_t`/`cc_t`/`NCCS`/`struct termios` | 112-130 |
| `src/process.rs` | `struct termios` (measured, real fields) | 7390-7430 |
| `src/minibuf.rs` | `struct termios` as an opaque `[u8; 72]` stand-in | 87-95 |

**Convergence (2026-10-04, first two done):** `src/sysdep.rs` and `src/minibuf.rs`
dropped their copies in the two commits that follow this mirror (`sysdep.rs`'s `tcflag_t`/
`speed_t`/`cc_t`/`NCCS`/`struct termios` block and its local `emacs_tty`/`CDISABLE`;
`minibuf.rs`'s opaque stand-in, its `emacs_tty` and its three use-site declarations), each
with `build-tu` + `--check` + `--cfg enable_checking` green.  `src/process.rs` is still
in flight in a lane, so its copy stays until that lane settles.  The rule for each of the
remaining ones is unchanged:
and `include!` this mirror - `minibuf.rs`'s opaque stand-in is the one that is *wrong* in kind
(it cannot be read field by field), the other two are exact duplicates.  `process.rs` was in
flight in a lane when this mirror landed, and `check-mirror-dupes.py` compares the include
graph, so the convergence has to be done as one commit per unit, after the lanes settle: add
the `include!`, delete the local block, `build-tu`, `--check`, then `check-mirror-dupes.py` must
stay at its ratchet row.

Evidence for this file: `layout-check` passes (all `lib/include` mirrors are pulled into the
probe through `src/include/systty.rs` now, so the struct's assertions run), and the clang
measurement above is reproducible with the command in the file's header comment.
