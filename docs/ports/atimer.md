# emacs/src/atimer.c (625 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 14
- [x] block_atimers  (line 71, static)
- [x] unblock_atimers  (line 80, static)
- [x] start_atimer  (line 111)
- [x] cancel_atimer  (line 172)
- [x] append_atimer_lists  (line 215, static)
- [x] stop_other_atimers  (line 236)
- [x] run_all_atimers  (line 272)
- [x] set_alarm  (line 299, static)
- [x] schedule_atimer  (line 365, static)
- [x] run_timers  (line 383, static)
- [x] handle_alarm_signal  (line 413, static)
- [x] do_pending_atimers  (line 453)
- [x] turn_on_atimers  (line 469)
- [x] init_atimer  (line 594)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 3
- [x] static struct atimer free_atimers  (line 42, static)
- [x] static struct atimer stopped_atimers  (line 47, static)
- [x] static struct atimer atimers  (line 52, static)

## Decisions

- **Four regions are not transcribed, and the ledger's liveness is the reason.**  The frozen
  configuration (`docs/baseline/config/darwin-arm64.macros.tsv`) has `HAVE_TIMER_SETTIME`
  undefined, and `src/syssignal.h` 44-46 turns that into "`HAVE_ITIMERSPEC` is not defined";
  `HAVE_TIMERFD` is undefined as well.  So C's `#ifdef HAVE_ITIMERSPEC` regions (54-66's
  `alarm_timer`/`alarm_timer_ok`/`timerfd`, 303-336's `if (0 <= timerfd || alarm_timer_ok)`
  block, 475-482's two `timer_settime` calls) and `#ifdef HAVE_TIMERFD` regions (28-34's
  includes, 418-448's `timerfd_callback`, 580-591's `have_buggy_timerfd`) are dropped whole by the preprocessor.  They are
  *also* unimplementable here: this target's headers have not got `timer_t`, `struct
  itimerspec`, `timer_create`, `timer_settime`, `timerfd_create`, `timerfd_settime` or
  `TFD_TIMER_ABSTIME` (measured: `clang -std=gnu23` rejects each of them on darwin-arm64), and
  `add_timer_wait_descriptor` has no mirror.  C's text for them is named in the unit's
  comments at each site, with the C line ranges; the `xml.c`/`nanosleep` precedent (a wholly
  dead arm whose target has not got the API) is what this follows.
- **`#ifdef ENABLE_CHECKING` (C 489-576) is dead** (`config.h` does not define it), and the
  arm is not carried for two reasons: its `wait_reading_process_output` (C 549/562) is not
  declared by any mirror, and its `defsubr (&Sdebug_timer_check)` (C 623) must not appear in
  Rust at all — the port's source-level `defsubr` scan does not consult the condition table, so
  the call would read as a live registration (P-76's `data.c` finding, the `xml.rs` precedent).
  The names (`MAXTIMERS`, `struct atimer_result`, `debug_timer_callback`, the
  `debug-timer-check` subr) and the reason are recorded here and at the site.
- **`struct sigaction` is now the measured record, not P-70's stand-in.**  `init_atimer`
  allocates one on the stack and `emacs_sigaction_init` writes into it, so an empty stand-in
  would be a size-0 object handed to a C function that fills 16 bytes.  The record therefore
  lives in the mirror of the header that declares it, `femacs/lib/include/signal.rs`
  (`sizeof 16`, handler union 8 bytes at 0, `sa_mask` at 8, `sa_flags` at 12, all pinned by
  `const _` assertions measured with the frozen target's clang), and `include/syssignal.rs`'s
  2-line stand-in is deleted in the same commit, as P-70 requires.
- **`sigemptyset`/`sigaddset` are the Darwin *macros*, not entries.**  The reference
  `atimer.o` imports `_pthread_sigmask`/`_sigaction`/`_raise` and has no `_sigemptyset`;
  `<signal.h>` 125-129 defines both as macros over `__sigbits`.  The mirror now carries each
  macro's body as an `#[inline(always)]` function, so the call sites keep C's spelling and the
  port emits no entry the reference has not got (recorded in `signal.rs`).
- **The two null pointer arguments keep C's literal**: `pthread_sigmask (SIG_SETMASK, oldset,
  0 as *mut sigset_t)` and `sigaction (SIGALRM, &action, 0 as *mut sigaction)`.  C writes a bare
  `0`; spelling it `std::ptr::null_mut ()` would have forced a `macro-body-notes.tsv` exemption
  for the token `0` (a hole in the gate), so the cast is used instead — the `callint.rs`
  `0 as *const c_char` / `doc.rs` `-1 as *mut c_void` shape.
- **C's `switch (type)` is a `match`** (PORTING.md 2.2) with a `_ => { }` arm, which is C's
  missing `default:` (a value outside the three members leaves the expiration unset, exactly as
  in C).  `switch`/`case`/`break` are accordingly exempted in
  `docs/ports/macro-body-notes.tsv`, the `set_internal`/`floatop_arith_driver` class.
- **`pending_signals = 1` is `true`** (lisp.h 3879 declares it `bool`); the literal is one of
  the two rows this file adds to `macro-body-notes.tsv` (the `Fset_case_table` bool class, with
  the same residual: the exemption hides a `true`/`false` swap in that one-statement body).
- **`t->fn (t)` is `((*t).r#fn).unwrap () (t)`**: the mirror's `atimer_callback` is
  `Option<unsafe extern "C" fn (…)>` (P-27), and every record on `atimers` was filled in by
  `start_atimer` (the free list's zeroed records are never scheduled), so the `unwrap` cannot
  fire — the `json.rs`/`vm-limit.rs` shape, recorded in the unit next to the call.
- **The include list is C 19-26's minus `keyboard.h`.**  `pending_signals` — the only name this
  unit takes from C's `keyboard.h`/`lisp.h` chain — is declared by `lisp.h` 3879, which is
  where C's own text gets it; `include/keyboard.rs` names `struct input_event` and the rest of
  termhooks.h's display closure in declarations this unit never touches, so it is left out
  (recorded at the include block).
- **`unblock_atimers`, `start_atimer` and the three lists keep C's shape**: the chains
  `free_atimers = stopped_atimers = atimers = NULL` (C 614) and the `for (…) ;` search loops
  (C 187, 220, 254) plus `run_all_atimers`'s `while (t)` (C 288) are Rust's named-once assignment and `while`-with-a-body, the
  `region-cache.rs`/`scroll.rs` precedent; `&stopped_atimers`/`&atimers` (C 185) are
  `&raw mut …`, the `buffer.rs` shape.
- **Symbol evidence** (the frozen build compiles `src/atimer.c`, though it has no `src/*.o`):
  compiling C 1-625 with the frozen flags (`gcc -std=gnu23 -c -Demacs -I build/emacs-ref/src
  -I emacs/src -I emacs/lib -o /tmp/atimer-ref.o`) yields exactly seven `T` entries —
  `_start_atimer`, `_cancel_atimer`, `_stop_other_atimers`, `_run_all_atimers`,
  `_do_pending_atimers`, `_turn_on_atimers`, `_init_atimer` — and the port's object exports the
  same seven `#[unsafe(no_mangle)]` names (plus Rust-mangled items).  The reference imports
  `_alarm`/`_current_timespec`/`_emacs_sigaction_init`/`_make_timeval`/`_pending_signals`/
  `_pthread_sigmask`/`_raise`/`_setitimer`/`_sigaction`/`_timespec_add`/`_timespec_sub`/
  `_xzalloc`; the port adds `_memset`/`_xfree` (C's compiler inlined `memset` and `xfree`, and
  the port must keep both spellings in its text) and the mirror helpers, which is the known
  inline-vs-call class.
- **Line drift**: C 625 vs Rust 482 (22.9%) — the four dead regions carry no Rust text and the
  port's fixed costs have no C counterpart; `docs/ports/line-drift-tolerance.tsv` records the
  0.30 tolerance and the reason (the `xml` row's class).
- **Compile evidence**: `xtask build-tu src/atimer.c` → `built src/atimer.rs ->
  ../build/femacs/atimer.o` (0 errors, 5 warnings, all of them the "value never read" class the
  `SAFE_ALLOCA`/`p` null-initializers produce elsewhere); `extract-abi.py --check` is OK, and
  the two mirrors this file touched (`src/atimer.h`, `src/syssignal.h`) are OK as well.
