# emacs/src/profiler.c (733 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 9
- [x] profiler-cpu-start  (Fprofiler_cpu_start, 1/1, doc babefe724579, line 459)
- [x] profiler-cpu-stop  (Fprofiler_cpu_stop, 0/0, doc 065b30f94c3c, line 489)
- [x] profiler-cpu-running-p  (Fprofiler_cpu_running_p, 0/0, doc d4e966250fb5, line 524)
- [x] profiler-cpu-log  (Fprofiler_cpu_log, 0/0, doc 269d98413652, line 533)
- [x] profiler-memory-start  (Fprofiler_memory_start, 0/0, doc 73e5370d2fe2, line 602)
- [x] profiler-memory-stop  (Fprofiler_memory_stop, 0/0, doc 1138f4005dd3, line 622)
- [x] profiler-memory-running-p  (Fprofiler_memory_running_p, 0/0, doc da6cd2a1a30c, line 635)
- [x] profiler-memory-log  (Fprofiler_memory_log, 0/0, doc d721a768d5f1, line 644)
- [x] function-equal  (Ffunction_equal, 2/2, doc b35c6d5f9326, line 680)

## Functions defined — 24
- [x] saturated_add  (line 30, static)
- [x] mark_log  (line 55, static)
- [x] make_log  (line 67, static)
- [x] free_log  (line 101, static)
- [x] get_log_count  (line 113, static)
- [x] set_log_count  (line 120, static)
- [x] get_key_vector  (line 127, static)
- [x] log_hash_index  (line 134, static)
- [x] remove_log_entry  (line 141, static)
- [x] trace_equal  (line 160, static)
- [x] trace_hash  (line 169, static)
- [x] make_profiler_log  (line 191, static)
- [x] free_profiler_log  (line 200, static)
- [x] approximate_median  (line 224, static)
- [x] evict_lower_half  (line 250, static)
- [x] record_backtrace  (line 274, static)
- [x] add_sample  (line 329, static)
- [x] handle_profiler_signal  (line 372, static)
- [x] deliver_profiler_signal  (line 387, static)
- [x] setup_cpu_timer  (line 393, static)
- [x] export_log  (line 563, static)
- [x] malloc_probe  (line 675)
- [x] mark_profiler  (line 697)
- [x] syms_of_profiler  (line 706)

## Macros defined — 0

## Types and their fields — 1
- [x] profiler_log: log_t* log, EMACS_INT gc_count, EMACS_INT discarded

## File-scope objects — 5
- [x] static Lisp_Object profiler_cpu_interval  (line 346, static)
- [x] static struct profiler_log cpu  (line 366, static)
- [x] static EMACS_INT current_sampling_interval  (line 369, static)
- [x] static struct profiler_log memory  (line 597, static)
- [x] bool profiler_memory_running  (line 600)

## Decisions

`emacs/src/profiler.c` @ `fd7afced403`; port `femacs/src/profiler.rs` (701 lines, 4.4% shorter
than C's 733 - inside the 5% the rule allows, so no tolerance row); the gate is
`python3 tools/extract-abi.py --check emacs/src/profiler.c femacs/src/profiler.rs` -> `OK`,
and `build-tu src/profiler.c` prints `built`.

### Dead text (not transcribed)

`#ifdef HAVE_ITIMERSPEC` is not defined on this target (`HAVE_TIMER_SETTIME` is `undef` in
`docs/baseline/config/darwin-arm64.macros.tsv` and syssignal.h 44-46 requires it), so the
following C text is the other build's and is recorded here instead:

- C 352-355: `static timer_t profiler_timer;` and `static bool profiler_timer_ok;`
  (`timer_t` does not exist in this target's headers).
- C 363: the `TIMER_SETTIME_RUNNING` enumerator of `enum profiler_cpu_running`.
- C 404-406: `handle_profiler_signal`'s `#if defined HAVE_ITIMERSPEC && defined
  HAVE_TIMER_GETOVERRUN` block (`timer_getoverrun`, `overruns`).
- C 432-459: `setup_cpu_timer`'s `system_clock` table (`CLOCK_THREAD_CPUTIME_ID`,
  `CLOCK_PROCESS_CPUTIME_ID`, `CLOCK_MONOTONIC`, `CLOCK_REALTIME`), the `struct sigevent`
  setup and the `timer_create`/`timer_settime` pair (`struct itimerspec` is not in this
  target's headers either).
- C 495-500: `Fprofiler_cpu_stop`'s `case TIMER_SETTIME_RUNNING` arm.

`HAVE_SETITIMER` is 1, so C 461-466 (the `setitimer` tail of `setup_cpu_timer`) and C
510-517 (the stop arm) *are* live and are transcribed.  `PROFILER_CPU_SUPPORT` is defined, so
the whole CPU-profiler region is live text - see the tool note below.

### Structural deviations

- **`enum profiler_cpu_running` is not a Rust `enum`.**  C 358-366 has the enumerators
  `NOT_RUNNING`, `TIMER_SETTIME_RUNNING` (dead) and `SETITIMER_RUNNING`, and assigns an `int`
  to the variable (`profiler_cpu_running = status;`, C 476).  A Rust `enum` cannot take that
  assignment, so the port declares C's *live* enumerators as `pub const c_int` with their C
  values (`NOT_RUNNING` 0, `SETITIMER_RUNNING` 1) and keeps the variable a `c_int`; the
  assignment is then written exactly as C wrote it.  `TIMER_SETTIME_RUNNING` is deliberately
  absent (its arm is dead): the ledger lists only the live enumerators.
- **C 188's `static Lisp_Object export_log (struct profiler_log *);` is dropped**
  (PORTING.md 0.1-12): Rust items are order-independent, so the definition stays at C 562's
  position and needs no forward declaration.
- **`saturated_add` is a plain `fn`** (its arguments are values), while the helpers that take
  pointers, `static mut`s or `unsafe extern` calls are `unsafe fn`; `handle_profiler_signal`
  and `deliver_profiler_signal` are `unsafe extern "C" fn` because `emacs_sigaction_init` and
  `deliver_process_signal` take `signal_handler_t` (the atimer.c `handle_alarm_signal`
  shape).
- **`malloc_probe`'s `min (size, MOST_POSITIVE_FIXNUM)`** compares a `size_t` with an
  `EMACS_INT` in C (the usual arithmetic conversions make the comparison unsigned); the port
  writes `min (size as EMACS_INT, MOST_POSITIVE_FIXNUM)`, which agrees for every allocation
  size below `EMACS_INT_MAX` (the value is immediately converted to `EMACS_INT` anyway).
- **`ckd_mul (&size_x_depth, size, depth)`** is `overflowing_mul`'s (value, flag) pair
  (P-15, `docs/ports/token-aliases.tsv`), with the store kept C's way.
- **`switch` -> `match`, `?:` -> `if`/`else`**: `Fprofiler_cpu_stop`'s `switch` and the four
  truthiness tests (`? Qt : Qnil`) are recorded in `docs/ports/macro-body-notes.tsv`
  (`Fprofiler_cpu_stop switch,case,break` and `syms_of_profiler doc`), the same class as
  `Fterminal_live_p` and `syms_of_terminal`.

### Mirror changes this unit needed

- `lib/include/signal.rs`: `SIG_IGN` (Darwin's `((void (*)(int)) 1)`) and the `signal`
  declaration.  The handler word is passed as `usize`, because `signal_handler_t` lives in the
  src-side syssignal.h mirror, which this lib mirror cannot name (P-72); `SIG_IGN` is then the
  integer 1 that Darwin defines.
- `src/include/lisp.rs`: `xcalloc` (lisp.h 5539, C's own neighbour of the `xnmalloc`
  declaration already there) and `get_backtrace`/`backtrace_top_function` (lisp.h 4862-4863,
  eval.c's exports that `record_backtrace`/`add_sample` call).
- `src/include/pdumper.rs` + `lib/include/fingerprint.rs`: this is the first unit to include
  C 24's `pdumper.h`.

### Tool and baseline change: `PROFILER_CPU_SUPPORT` (2026-10-03)

`PROFILER_CPU_SUPPORT` is defined by **src/syssignal.h** (53-54), not by config.h, so the
frozen macro table did not know it.  The extractor therefore called C 343-524 dead and
rejected the port's four `defsubr (&raw mut Sprofiler_cpu_*)` as "unexpected defsubr (...):
the reference file does not register it" - a gate that answered *wrongly* rather than
`unknown`.  Fix, in the port's own terms:

- `docs/baseline/config/source-header-macros.tsv` (new): the macros Emacs' own headers define
  that config.h cannot decide, one `<header>	<macro>` per row, starting with
  `syssignal.h	PROFILER_CPU_SUPPORT`.
- `tools/gen-cond-macros.py` measures each row with clang on the frozen include paths and
  merges it into the table, so a stale row cannot freeze a wrong branch silently.
- `docs/baseline/config/darwin-arm64.macros.tsv` gains exactly that one row (7223 macros now).
- `src/profiler.c` joins `docs/baseline/config/cond-eval-verified.txt` (429 files): all 13 of
  its conditions agree with clang now, and the re-run of the whole list reports 0
  disagreements, so nothing else regressed.
