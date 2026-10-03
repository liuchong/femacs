# emacs/… the frozen target's <termios.h> (system header, P-26/P-50)

`femacs/lib/include/termios.rs`.  gnulib has no `termios` wrapper in this build and the
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
| `femacs/src/sysdep.rs` | `tcflag_t`/`speed_t`/`cc_t`/`NCCS`/`struct termios` | 112-130 |
| `femacs/src/process.rs` | `struct termios` (measured, real fields) | 7390-7430 |
| `femacs/src/minibuf.rs` | `struct termios` as an opaque `[u8; 72]` stand-in | 87-95 |

**Convergence (deliberately not done in this commit):** each of the three should drop its copy
and `include!` this mirror - `minibuf.rs`'s opaque stand-in is the one that is *wrong* in kind
(it cannot be read field by field), the other two are exact duplicates.  `process.rs` was in
flight in a lane when this mirror landed, and `check-mirror-dupes.py` compares the include
graph, so the convergence has to be done as one commit per unit, after the lanes settle: add
the `include!`, delete the local block, `build-tu`, `--check`, then `check-mirror-dupes.py` must
stay at its ratchet row.

Evidence for this file: `layout-check` passes (all `lib/include` mirrors are pulled into the
probe through `src/include/systty.rs` now, so the struct's assertions run), and the clang
measurement above is reproducible with the command in the file's header comment.
