# emacs/src/syswait.h (62 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_SYSWAIT_H  (line 24, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

Mirror of src/syswait.h, complete: the include guard and the two extern declarations.  The
status-macro fallbacks (`WCONTINUED`, `WCOREDUMP`, `WEXITSTATUS`, `WIFEXITED`, `WIFSTOPPED`,
`WIFSIGNALED`, `WSTOPSIG`, `WTERMSIG`) are dead on the frozen target - `HAVE_SYS_WAIT_H` is 1,
C 30 includes the system `<sys/wait.h>`, and it defines all eight, so every `#ifndef` below is
false (the extractor's live inventory is the guard alone, body `da39a3ee5e6b` = the empty
`#define EMACS_SYSWAIT_H`).  They are listed here rather than transcribed (PORTING.md P-08).

Both declarations are declarations only: `wait_for_termination` and `child_status_changed` are
defined by src/sysdep.c, which is ported (`src/sysdep.rs` owns them), so the mirror uses
the P-21 module plus glob import (`syswait_decls`).  Evidence: `extract-abi.py --check
emacs/src/syswait.h src/include/syswait.rs` -> `OK`; `layout-check` passes (it compiles
every `src/include/*.rs`, so the module and its `pid_t`/`c_int` names resolve).
