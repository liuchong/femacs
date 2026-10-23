# emacs/lib/pthread_sigmask.c (84 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] pthread_sigmask  (line 33)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The file has no live text in this configuration, and the port reads it in the one where it
  is built.**  `HAVE_PTHREAD_SIGMASK` is 1 and none of `PTHREAD_SIGMASK_INEFFECTIVE`,
  `PTHREAD_SIGMASK_NOT_IN_LIBC`, `PTHREAD_SIGMASK_FAILS_WITH_ERRNO` is defined, so gnulib does
  not compile the file (there is no `build/emacs-ref/lib/pthread_sigmask.o`).  C's `# undef
  pthread_sigmask` sits *between* the declarator (C 17) and the body (C 19), which is gnulib's
  wrap idiom: in the configuration where the replacement exists, the generated `<signal.h>`
  has `#define pthread_sigmask rpl_pthread_sigmask` active for C 17 (so the definition is
  `rpl_pthread_sigmask`) and the `# undef` makes the body's call the *system* one.
- So the port emits the renamed symbol with `#[unsafe(export_name = "rpl_pthread_sigmask")]`
  while keeping C's source name on the item (PORTING.md 10.13①, the mempcpy precedent), and
  the body calls the system entry through the signal mirror's `signal_decls` module path.  The
  qualified path is deliberate: the definition shadows the glob import of the same name
  (PORTING.md 0.1-10), so a bare `pthread_sigmask (…)` would resolve to the definition itself.
  Measured on the port's object: `nm -g build/femacs/pthread_sigmask.o` prints
  `T _rpl_pthread_sigmask` and `U _pthread_sigmask`.
- **Same idiom, landed file, defect (report only, not fixed here)**: `lib/nanosleep.rs`
  ports nanosleep.c's identical `# undef nanosleep` between signature and body by naming the
  item `nanosleep` *and* calling `nanosleep (…)` in its body — which resolves to the definition
  itself, i.e. the 24-day-chunk loop would recurse instead of calling the system sleep, and the
  object exports `_nanosleep` where the reference exports `_rpl_nanosleep` (measured:
  `nm build/emacs-ref/lib/nanosleep.o` → `T _rpl_nanosleep`, `U _nanosleep`; the port's object
  has no `U _nanosleep`).  It needs the same `export_name` + qualified-path treatment.
- The dead arms (`PTHREAD_SIGMASK_INEFFECTIVE`'s two blocks and `PTHREAD_SIGMASK_FAILS_WITH_ERRNO`'s
  one) are in cond-eval-verified.txt's table and are named in a comment rather than carried
  (P-75); `#include <stddef.h>` and `<string.h>` are theirs too, and the port includes the
  string mirror because C's include closure has it.
- **Mirror edit**: `lib/include/signal.rs` gained `sigset_t` (Darwin's
  `struct __darwin_sigset_t { __uint32_t __val; }`, 4 bytes, measured), `SIG_BLOCK`/`SIG_UNBLOCK`/
  `SIG_SETMASK` (1/2/3) and a uniquely-named `signal_decls` module with `sigemptyset`/
  `sigaddset`/`sigprocmask`/`pthread_sigmask` (the stdio.rs shape, so a unit that defines one
  shadows the glob).
- Line-count drift (C 84 vs the port's 65) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): the body's `pthread_sigmask` call is the system symbol
  (as in C); `sigemptyset`/`sigaddset`/`sigprocmask` are only named in the dead-arm comment.
