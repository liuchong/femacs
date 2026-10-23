# emacs/lib/signal.in.h (536 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 26
- [x] GNULIB_defined_SIGPIPE  (line 134, body 356a192b7913)
- [x] GNULIB_defined_function_taking_int_returning_void_t  (line 380, body 356a192b7913)
- [x] GNULIB_defined_sig_atomic_t  (line 103, body 356a192b7913)
- [x] GNULIB_defined_sighandler_t  (line 122, body 356a192b7913)
- [x] GNULIB_defined_siginfo_types  (line 473, body 356a192b7913)
- [x] GNULIB_defined_signal_blocking  (line 253, body 356a192b7913)
- [x] GNULIB_defined_sigset_t  (line 111, body 356a192b7913)
- [x] GNULIB_defined_struct_sigaction  (line 505, body 356a192b7913)
- [x] GNULIB_defined_verify_NSIG_constraint  (line 264, body 356a192b7913)
- [x] NSIG  (line 258, body cb4e5208b4cd)
- [x] SA_NODEFER  (line 530, body b6589fc6ab0d)
- [x] SA_RESETHAND  (line 501, body 356a192b7913)
- [x] SA_RESTART  (line 503, body 1b6453892473)
- [x] SIG2STR_MAX  (line 154, body ce6ec6b75307)
- [x] SIGPIPE  (line 133, body bd307a3ec329)
- [x] SIG_BLOCK  (line 359, body b6589fc6ab0d)
- [x] SIG_SETMASK  (line 360, body 356a192b7913)
- [x] SIG_UNBLOCK  (line 361, body da4b9237bacc)
- [x] WIN_PTHREADS_SIGNAL_H  (line 26, body da39a3ee5e6b)
- [x] _  (line 62, body 74e8ad4e2466)
- [x] pthread_sigmask  (line 192, body d477ffd6818b)
- [x] raise  (line 229, body d377daed2a01)
- [x] sa_handler  (line 498, body 4b75e6f7717c)
- [x] sa_sigaction  (line 513, body 41964b1ac667)
- [x] sig_atomic_t  (line 102, body f993887d153b)
- [x] signal  (line 387, body 4b31e0fcf233)

## Types and their fields — 3
- [x] sigaction: void (*) _sa_handler, void (*) _sa_sigaction, <aggregate> _sa_func, sigset_t sa_mask, int sa_flags
- [x] siginfo_t: int si_signo, int si_code, int si_errno, pid_t si_pid, uid_t si_uid, void* si_addr, int si_status, long si_band, union sigval si_value
- [x] sigval: int sival_int, void* sival_ptr

## File-scope objects — 0

# ----------------------------------------------------------------------
## Decisions

- **The criterion is the generated header**: `build/emacs-ref/lib/signal.h` (41,625 bytes), written
  from this template by `gnulib.mk`'s rule.  `python3 tools/check-cond-eval.py lib/signal.in.h` ->
  `87 conditions, 34 unknown, 44 disagreements [NOT VERIFIED]` (3.136 already listed this unit as
  one that "needs P-51/P-66 carrying"), so every `#if` carries both arms and the mirror's header
  table is the measured substitution table (values read by walking template and generated file in
  step, so each is the literal the build wrote).
- **What is live on this target**: C 117-128's `sighandler_t` (`@HAVE_SIGHANDLER_T@` 0 - the SDK
  has no such type), C 150-153's `SIG2STR_MAX`, C 155-185's `sig2str` / `str2sig`
  (`@GNULIB_SIG2STR@` 1 with `@HAVE_SIG2STR@` / `@HAVE_STR2SIG@` **0**, so `_GL_FUNCDECL_SYS` is the
  live declaration), C 188-217's `pthread_sigmask` (`@GNULIB_PTHREAD_SIGMASK@` 1,
  `@REPLACE_PTHREAD_SIGMASK@` 0 -> SYS), C 519-523's `SA_NODEFER` fallback, and the `<pthread.h>`
  arm at C 78-86 (its `__APPLE__ && __MACH__` operand is true).
- **What is dead, and why the mirror still carries it**: `@GNULIB_SIGPROCMASK@` 0 (the whole
  C 249-441 block: `SIG_BLOCK`/`SIG_SETMASK`/`SIG_UNBLOCK`,
  `GNULIB_defined_signal_blocking`, the `verify_NSIG_constraint` typedef,
  `_gl_function_taking_int_returning_void_t`, the `signal` rename),
  `@GNULIB_SIGACTION@` 0 (C 444-515: `union sigval`, `struct siginfo_t`, `struct sigaction` with
  its anonymous `_sa_func`, `sa_handler` / `sa_sigaction`, `SA_RESETHAND` / `SA_RESTART`),
  `@GNULIB_SIGNAL_H_SIGPIPE@` 0, `@GNULIB_RAISE@` 0, `@HAVE_SIGSET_T@` 1, and
  `@HAVE_TYPE_VOLATILE_SIG_ATOMIC_T@` 1.  They are the arm where gnulib defines the type itself.
- **The two halves are named.**  C 41/57's `#include_next` is the frozen `<signal.h>`; the port's
  mirror of what its units name from it is `signal.rs` (signal numbers, `sigset_t`, Darwin's
  `union __sigaction_u` + `struct sigaction` with measured layout assertions, `SIGPIPE` = 13).
  This file is the gnulib wrapper: on this target it renames and defines nothing, which the
  `@HAVE_*@` / `@GNULIB_*@` readings above prove.
- The demanded macro `_` is the extractor reading `_@GUARD_PREFIX@_ALREADY_INCLUDING_SIGNAL_H`
  (P-66's template blindness); the guard text satisfies it.
- **No `build-tu` consumer** - the same reading the sibling template mirrors give (3.207/3.208):
  `.in.h` mirrors take their scalars (`c_int`, `c_uint`, `c_char`, `pid_t`, `uid_t`, `c_long`,
  `c_void`) from the including unit.

## Evidence

    $ python3 tools/extract-abi.py --check emacs/lib/signal.in.h lib/include/signal.in.rs
    OK emacs/lib/signal.in.h -> lib/include/signal.in.rs

    $ python3 tools/check-cond-eval.py lib/signal.in.h
    lib/signal.in.h: 87 conditions, 34 unknown, 44 disagreements [NOT VERIFIED]

    $ diff <(sed 's/@GUARD_PREFIX@/GL/g' emacs/lib/signal.in.h) build/emacs-ref/lib/signal.h | wc -l
    (the inlined c++defs.h / arg-nonnull.h / warn-on-use.h text plus the @NAME@ lines)

    $ rg -n 'if 1$|if 0$|if !1|elif 0' build/emacs-ref/lib/signal.h
    45:#if 1 / 46:#if 0   (the two guard-then-include chains)
    71/79:#if (1 || defined GNULIB_POSIXCHECK)
    645:#if ! 1             <- @HAVE_TYPE_VOLATILE_SIG_ATOMIC_T@ (value 1, arm dead)
    655:#if !1               <- @HAVE_SIGSET_T@ (value 1, arm dead)
    663:#if !0              <- @HAVE_SIGHANDLER_T@ (value 0, so this arm is LIVE)
    677:#if 0               <- @GNULIB_SIGNAL_H_SIGPIPE@ (dead)
    704/705:#if 1 / # if !0 <- @GNULIB_SIG2STR@ / @HAVE_SIG2STR@ (live)
    772:#if 0               <- @GNULIB_RAISE@
    796:#if 0               <- @GNULIB_SIGPROCMASK@
    991:#if 0               <- @GNULIB_SIGACTION@
    992/994:# if !1 / #  if !1 <- @HAVE_SIGACTION@ / @HAVE_SIGINFO_T@
    1058:# elif !1           <- @HAVE_STRUCT_SIGACTION_SA_SIGACTION@
