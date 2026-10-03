# emacs/lib/pselect.c (115 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] rpl_pselect  (line 92)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- The file is in cond-eval-verified.txt and `HAVE_PSELECT` is 1 (macros.tsv 1469), so C 36-85's
  `#if !HAVE_PSELECT` arm — the whole `pselect` emulation — is dead and C 87-115's `#else` arm
  is the live text.  The dead arm's tokens are not required (P-75) and the port names its shape
  (the `struct timeval` construction with `(tv_nsec + 999) / 1000`, the `pthread_sigmask
  (SIG_SETMASK, …)` swap and the `select` call) in a comment.
- C's definition carries the renamed name gnulib's `<sys/select.h>` gives it (`rpl_pselect`),
  and C 89's `# undef pselect` makes the body's `pselect (…)` the *system* entry — so the port
  needs neither an `export_name` nor a qualified path: `rpl_pselect` is a plain non-`static`
  definition (`#[unsafe(no_mangle)]`, P-52) and the call goes to the mirror's `pselect`.
- C 103's `for (int i = 0; …)` is the port's `while` with an explicit step; the three
  `FD_ISSET` uses keep C's short-circuit (`rfds && FD_ISSET (i, rfds)` → `! rfds.is_null () &&
  FD_ISSET (…)`), and `restrict` has no Rust spelling (PORTING.md 0.3).
- **Mirror edits**: `femacs/lib/include/sys_select.rs` is new (the measured `fd_set` =
  32 × `__int32_t`, `FD_SETSIZE`, `__DARWIN_NFDBITS`, the `FD_ISSET` inline rewritten over the
  measurement, and the `select`/`pselect`/`__darwin_check_fd_set_overflow` declarations);
  `femacs/lib/include/signal.rs` gained `sigset_t` (docs/ports/pthread_sigmask.md).
- The one deviation in the mirror is recorded in docs/ports/include-sys_select.md: C's
  `__darwin_check_fd_set` tests the address of `__darwin_check_fd_set_overflow` for null (a
  weak-link guard for old systems); the port calls it unconditionally, since this target's
  libSystem exports it.
- Line-count drift (C 115 vs the port's 77) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): the body's `pselect`/`dup2` calls are the system ones,
  as in C.
