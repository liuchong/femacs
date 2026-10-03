# emacs/src/thread.c (1314 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 21
- [x] make-mutex  (Fmake_mutex, 0/1, doc ff95872d4fd3, line 304)
- [x] mutex-lock  (Fmutex_lock, 1/1, doc 36a24a2dcd47, line 349)
- [x] mutex-unlock  (Fmutex_unlock, 1/1, doc 457ca03f0be9, line 381)
- [x] mutex-name  (Fmutex_name, 1/1, doc a776d011aa0a, line 397)
- [x] make-condition-variable  (Fmake_condition_variable, 1/2, doc fd1802cbc36b, line 418)
- [x] condition-wait  (Fcondition_wait, 1/1, doc f8984848501a, line 478)
- [x] condition-notify  (Fcondition_notify, 1/2, doc 5e4133b70202, line 536)
- [x] condition-mutex  (Fcondition_mutex, 1/1, doc 45f952478781, line 567)
- [x] condition-name  (Fcondition_name, 1/1, doc 0c750427b0fe, line 579)
- [x] thread-yield  (Fthread_yield, 0/0, doc abde77302bc0, line 734)
- [x] make-thread  (Fmake_thread, 1/3, doc c15e32ccd2ce, line 885)
- [x] current-thread  (Fcurrent_thread, 0/0, doc 0b70fe518e42, line 952)
- [x] thread-name  (Fthread_name, 1/1, doc 44701b0b3792, line 961)
- [x] thread-signal  (Fthread_signal, 3/3, doc ff867d169b09, line 993)
- [x] thread-live-p  (Fthread_live_p, 1/1, doc 59ca52fcb1c1, line 1036)
- [x] thread-buffer-disposition  (Fthread_buffer_disposition, 1/1, doc 43a79b8a0563, line 1048)
- [x] thread-set-buffer-disposition  (Fthread_set_buffer_disposition, 2/2, doc a5bc71fdb54d, line 1062)
- [x] thread--blocker  (Fthread_blocker, 1/1, doc bb77f094114e, line 1083)
- [x] thread-join  (Fthread_join, 1/1, doc bca4ac6f5da1, line 1118)
- [x] all-threads  (Fall_threads, 0/0, doc cbaf821998aa, line 1146)
- [x] thread-last-error  (Fthread_last_error, 0/1, doc aca81a71e04f, line 1167)

## Functions defined — 41
- [x] release_global_lock  (line 81, static)
- [x] rebind_for_thread_switch  (line 87, static)
- [x] unbind_for_thread_switch  (line 95, static)
- [x] post_acquire_global_lock  (line 105, static)
- [x] acquire_global_lock  (line 158, static)
- [x] maybe_reacquire_global_lock  (line 171)
- [x] lisp_mutex_init  (line 190, static)
- [x] lisp_mutex_lock_for_thread  (line 212, static)
- [x] lisp_mutex_lock  (line 247, static)
- [x] lisp_mutex_unlock  (line 259, static)
- [x] lisp_mutex_unlock_for_wait  (line 276, static)
- [x] lisp_mutex_destroy  (line 291, static)
- [x] lisp_mutex_owned_p  (line 297, static)
- [x] mutex_lock_callback  (line 330, static)
- [x] do_unwind_mutex_lock  (line 344, static)
- [x] mutex_unlock_callback  (line 372, static)
- [x] finalize_one_mutex  (line 411)
- [x] condition_wait_callback  (line 446, static)
- [x] condition_notify_callback  (line 514, static)
- [x] finalize_one_condvar  (line 593)
- [x] really_call_select  (line 613, static)
- [x] thread_select  (line 643)
- [x] mark_one_thread  (line 666, static)
- [x] mark_threads_callback  (line 696, static)
- [x] mark_threads  (line 711)
- [x] unmark_main_thread  (line 717)
- [x] yield_callback  (line 725, static)
- [x] invoke_thread_function  (line 743, static)
- [x] record_thread_error  (line 754, static)
- [x] run_thread  (line 761, static)
- [x] free_search_regs  (line 867, static)
- [x] finalize_one_thread  (line 877)
- [x] thread_signal_callback  (line 975, static)
- [x] thread_set_error  (line 985, static)
- [x] thread_join_callback  (line 1101, static)
- [x] thread_check_current_buffer  (line 1183)
- [x] thread_all_before_buffer_killed  (line 1200)
- [x] main_thread_p  (line 1232)
- [x] in_current_thread  (line 1238)
- [x] init_threads  (line 1246)
- [x] syms_of_threads  (line 1262)

## Macros defined — 2
- [x] release_select_lock  (line 38, body cce2980cc6ca)
- [x] thread_live_p  (line 76, body 454f8972b6de)

## Types and their fields — 4
- [x] <anon@765>: Lisp_Object o, void* p, char c
- [x] aligned_thread_state: struct thread_state s
- [x] notify_args: struct Lisp_CondVar* cvar, int all
- [x] select_args: select_func* func, int max_fds, fd_set* rfds, fd_set* wfds, fd_set* efds, struct timespec* timeout, sigset_t* sigmask, int result

## File-scope objects — 4
- [x] struct thread_state current_thread  (line 64)
- [x] struct thread_state all_threads  (line 66)
- [x] static sys_mutex_t global_lock  (line 68, static)
- [x] static Lisp_Object last_thread_error  (line 751, static)

## Decisions

`emacs/src/thread.c` @ `fd7afced403`; port `femacs/src/thread.rs` (1319 lines vs C's 1314,
**1255** live - the 5.1% drift against the live baseline is declared in
`docs/ports/line-drift-tolerance.tsv` with the `live` basis, because the six dead regions this
port records instead of carrying are 60 C lines).  The gates are
`build-tu src/thread.c` -> `built src/thread.rs -> ../build/femacs/thread.o`,
`python3 tools/extract-abi.py --check emacs/src/thread.c femacs/src/thread.rs` -> `OK` and
`python3 tools/check-symbols.py --pending docs/baseline/symbol-pending.txt` ->
`78 units compared, 0 mismatched, 1 still pending` (the compared set is the `lib/` units with a
reference object; it is not this port's).  `src/thread.c` is in
`docs/baseline/frozen-build-sources.txt`, so the frozen configuration compiles it: there is no
3.31 exemption here and the full comparison applies.

### Dead text (not transcribed)

`THREADS_ENABLED`, `HAVE_NS`, `HAVE_GLIB`, `HAVE_ANDROID` and `ANDROID_STUBIFY` are `undef` in
`docs/baseline/config/darwin-arm64.macros.tsv`; 60 of the file's 1314 lines are dead and are
recorded here instead of transcribed (P-08; thread.c is a `src/` file, so no dead text is kept
as `#[cfg (feature = …)]`).  Every dead region is a whole `#if`/`#ifdef` block, not a
statement:

- C 31-33: `#ifdef HAVE_NS`'s `#include "nsterm.h"`.
- C 35-39: the `#if defined HAVE_GLIB && ! defined (HAVE_NS)` arm (`#include <xgselect.h>`);
  C 38's `#else` - `#define release_select_lock() do { } while (0)` - is the live text and the
  port's macro has an empty body (`docs/ports/macro-body-notes.tsv`).
- C 111-113: `post_acquire_global_lock`'s `android_java_env = self->java_env;`.
- C 135-139: `post_acquire_global_lock`'s `android_check_query ()`.
- C 656-658: `thread_select`'s `android_before_select ()`.
- C 774-778 and 789-797: `run_thread`'s `#ifdef THREADS_ENABLED` / `HAVE_ANDROID` arms (`jint rc`
  and the `AttachCurrentThread` block).
- C 780-787 and 841-843: `run_thread`'s `#ifdef HAVE_NS` autorelease-pool pair
  (`ns_alloc_autorelease_pool`/`ns_release_autorelease_pool`).
- C 845-851: `run_thread`'s `#ifdef THREADS_ENABLED` / `HAVE_ANDROID` `DetachCurrentThread` arm.
- C 937-938: `Fmake_thread`'s `#ifdef THREADS_ENABLED` arm (`error ("Could not start a new
  thread");`); C 940's `#else` - `error ("Concurrency is not supported in this configuration");`
  - is the live text.  This is the only Lisp-visible consequence of the configuration and it is
  preserved verbatim.
- C 1010-1024: `Fthread_signal`'s `#ifdef THREADS_ENABLED` arm (`main_thread_p` building a
  `THREAD_EVENT` and calling `kbd_buffer_store_event`); C 1024's `else` is attached to nothing
  in the live text, so the port's block is unconditional (the same shape C compiles).
- C 1252-1253: `init_threads`' `current_thread->java_env = android_java_env;`.
- C 1309-1310: `syms_of_threads`' `XSETTHREAD (Vmain_thread, &main_thread.s);`; C 1312's
  `Vmain_thread = Qnil ();` is the live text.

`src/thread.c` is in `docs/baseline/config/cond-eval-verified.txt`, and
`python3 tools/check-cond-eval.py src/thread.c` reports
`18 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the extractor's branch selection is
the clang-verified one; no `source-header-macros.tsv` row was needed.

### Deviations recorded

1. **`self` is reserved.**  C names the parameter `self` in `post_acquire_global_lock`,
   `acquire_global_lock`, `lisp_mutex_lock_for_thread`, `really_call_select`,
   `yield_callback`, `run_thread`, `thread_signal_callback`, `thread_join_callback` and
   `mutex_lock_callback`; Rust reserves it and it cannot be spelled `r#self`, so the port writes
   `self_`.  The token comparison folds a trailing underscore, so the bodies' tokens still match
   C's (the `r#match` precedent, PORTING.md 0.2); the parameter *name* is the one lexical
   deviation here.
2. **`main_thread`'s initializer is a `const` expression.**  C 52-67's initializer calls
   `PVECHEADERSIZE` and `LISPSYM_INITIALLY`, which the lisp.rs mirror defines as ordinary
   (non-`const`) functions, so the port writes `MAIN_THREAD_HEADER_SIZE` as the same expression
   with `const` arithmetic and the nine `LISPSYM_INITIALLY (Qnil)` members as `lisp_h_Qnil ()`
   (symbol index 0 with tag 0 - the same value).  Every other member is C's zero.  The record is
   `static` (C 52), so it carries no `#[unsafe(no_mangle)]`.
3. **C's `struct buffer *` positions are untyped where the mirror is.**  `thread_state`'s
   `m_current_buffer` is `struct buffer *` in C but thread.h's mirror cannot name `buffer` (it is
   included by every unit, and most do not include buffer.rs), so it is `*mut c_void` with a
   `/// C:` line (P-55), and so are `set_buffer_internal_2`'s parameter (declared in buffer.rs
   with `*mut buffer`, the call site casts) and `thread_check_current_buffer`'s parameter (the
   mirror's declaration is untyped, so the body needs no cast).  `thread_all_before_buffer_killed`
   keeps C's two `struct buffer *` locals - buffer.rs *is* included here - and casts at the two
   places they meet the untyped field (`b as *mut c_void`, `other as *mut c_void`).
4. **C's chained assignments are two statements** (`run_thread`'s
   `self->m_stack_bottom = self->stack_top = &stack_pos.c;` in C's right-to-left order, and
   `handlerlist = handlerlist_sentinel->nextfree = handlerlist_sentinel;`); the dired.rs
   precedent.
5. **Rust has no pointer arithmetic on raw pointers**: C's `pdlptr - pdl` is
   `.offset_from (…)` and C's `pdlvec + 1` / `m_specpdl + size` are `.offset (…)`
   (`rebind_for_thread_switch`, `unbind_for_thread_switch`, `Fmake_thread`); the itree.rs
   precedent.
6. **C's uninitialized locals are zero-initialized** (PORTING.md 1.5) where Rust needs a value
   before C's first assignment: `sigset_t oldset` (`really_call_select`),
   `struct select_args sa` and `struct notify_args args` (`std::mem::zeroed ()`, the menu.rs
   `skp` precedent) and `union { … } stack_pos` in `run_thread`.  Locals every path assigns
   before its first read stay uninitialized as C declares them (`lmutex`, `cvar`, `tstate`,
   `b`/`other`'s first assignment, `cond`, `saved_count`, `pdlvec`, `iter`, `c_next`, …).
7. **`--`/`++`**: C's `if (--mutex->count > 0)` is the two-statement
   `(*mutex).count -= 1; if (*mutex).count > 0` (the dired.rs `ind ++` precedent).
8. **`bool`-to-`int` conversions Rust will not insert**: `lisp_mutex_owned_p` returns
   `if … { 1 } else { 0 }`, `args.all = if !NILP (all) { 1 } else { 0 }`, the
   `if (*mutex).owner.is_null ()` / `new_count != 0` / `not_holding_lock != 0` tests, and
   `Fthread_live_p`'s `? Qt : Qnil` is an `if`/`else` (PORTING.md 2.2).
9. **Explicit conversions**: `saved_count as c_int` (`lisp_mutex_lock_for_thread`'s `int
   new_count`), `-distance as c_int` (`specpdl_unrewind`'s `int distance`),
   `stack_top as *const c_char` (`mark_c_stack`'s `char const *`),
   `(*sa).timeout as *const timespec` / `(*sa).sigmask as *const sigset_t` (the `select_func`
   typedef's `const` parameters), `xfree (p as *mut c_void)` (five sites),
   `b as *mut c_void` / `other as *mut c_void`, `((1 + size) as usize) * size_of::<specbinding> ()`
   and `tstate as *const c_void` (`main_thread_p`'s `const void *`).
10. **`if (0)` is `if false`** in `syms_of_threads` (C 1265's `#ifndef THREADS_ENABLED` arm keeps
    it as live text); the token is in `docs/ports/macro-body-notes.tsv` (the dup2_nothrow
    precedent).  The twenty-one `defsubr`s are therefore *not* registered on this target, exactly
    as C does not register them.
11. **`list (…)` is the mirror's `list!` macro** in `syms_of_threads` (`list` has no function
    form in the port, the lisp.rs `calln!`/`list!` shapes).
12. **The anonymous union in `run_thread`** (C 766-772, the ledger's `<anon@765>`) is a named
    local `union Stack_pos` with C's three members; the port takes `addr_of!` of the `c` member
    (C's `&stack_pos.c`), which is the union's address.  cinventory names the container
    `<anon@NNN>` and skips its fields, so no field check applies.
13. **`struct handler` is declared in the thread.h mirror**, not in this unit: see
    "Changes outside the unit" below.
14. **`thread_live_p` and `release_select_lock`** keep C's names as `macro_rules!`, both defined
    at their C positions (C 38 and C 75-76); `thread_live_p`'s `(STATE)->m_specpdl != NULL` is
    the port's `(*( $state)).m_specpdl != std::ptr::null_mut ()` (`NULL` folds to the
    `null`/`null_mut`/`is_null` family of `token-aliases.tsv`).
15. **The ledger's file-scope list omits `main_thread`.**  cinventory does not report
    `static union aligned_thread_state main_thread = {{ … }}` (C 52-67) because of its
    initializer's nested braces; there is no box for it, and it is ported at its C place with the
    assertion above.

### Names written at their use site

The mirrors this unit includes do not carry these, so each is declared where it is called (the
terminal.rs/filelock.rs shape), with C's prototype in a `/// C:` line and C's own spelling of
the pointer-only types (P-55): `specpdl_unrewind` (lisp.h 4834), `mark_specpdl` (4861),
`mark_c_stack` (4441), `mark_bytecode` (5231), `mark_object` (4436), `push_handler` (4812),
`internal_condition_case` (4805), `record_unwind_protect_void` (4825), `init_bc_thread`/
`free_bc_thread` (5229-5230) and `update_processes_for_thread_death` (process.h 313).  C 70's
`extern volatile int interrupt_input_blocked;` is *not* repeated: blockinput.rs (which this unit
includes, as C's keyboard.h pulls blockinput.h) declares the object, and the mirror owns it.

### Macro-body notes

`docs/ports/macro-body-notes.tsv` carries two rows for this unit: `release_select_lock` (`0` -
C's `do { } while (0)` no-op arm has an empty Rust body, `do`/`while` are already skipped tokens)
and `syms_of_threads` (`0,doc` - C's `if (0)` is the port's `if false`, and the
`DEFVAR_LISP`/`DEFSYM` invocations take C's docstring positionally).  `thread_live_p`'s body
needs no row (`NULL` folds to `null_mut`).  No new `token-aliases.tsv` family and no
`excluded-macros.txt` row was needed.

### Condition table

Every `#if` in this file is decided by `docs/baseline/config/darwin-arm64.macros.tsv`:
`THREADS_ENABLED` undef (3678), `HAVE_NS` undef, `HAVE_GLIB` undef, `HAVE_ANDROID` undef,
`ANDROID_STUBIFY` undef, `HAVE_PDUMPER` 1, `HAVE_PTHREAD` 1.  `src/thread.c` is in
`cond-eval-verified.txt` and re-running `python3 tools/check-cond-eval.py src/thread.c` reports
`18 conditions, 0 unknown, 0 disagreements [VERIFIED]`.

### Line count

`docs/ports/line-drift-tolerance.tsv`'s `thread` row declares the `live` basis with a 0.10
tolerance: the 60 dead lines are recorded rather than carried, so the denominator is the 1255
live lines, and the port's 1319 (+5.1%) is the unit's fixed costs (the file header, the allow
list, the eleven use-site declarations, the comments that name each dead region) less the 71 physical
doc-comment lines that the twenty-one `DEFUN!` docstrings and the one `DEFVAR_LISP!` docstring
collapse into single-line literals.

### Changes outside the unit (for integrator review)

1. `femacs/xtask/src/main.rs`: `materialize` now expands the `include!` graph **in place** (C's
   textual order, with the include-guard dedupe kept) instead of emitting every dependency
   before the file that includes it.  This is what lets thread.h's mirror use `lisp.h`'s
   `eassert!`/`XUNTAG!` in `XTHREAD`/`XMUTEX`/`XCONDVAR`: C's thread.h 2289 sits *inside* lisp.h,
   so it sees lisp.h 181-196's `eassert` and 750-752's `XUNTAG`, while the old "dependencies
   first" flattening put the whole thread mirror ahead of lisp.h and reported
   `cannot find macro eassert`.  The mirror is written exactly as C's text is, so the fix is the
   tool's, not a workaround in the port.  Two details the change had to keep: an `include!`
   whose target does not exist relative to the file that writes it stays commented out *in a
   copy* (the old `neutralize` step did that for every mirror - `lib/include/md5.rs` reaches
   `../src/include/config.rs` from a directory where it does not resolve), while the root's own
   lines are kept verbatim so a typo in a unit still fails the compile; and the three functions
   the old flattening used (`include_graph_ordered`, `include_graph`, `neutralize`) are deleted,
   since nothing calls them any more.
   Verification: `xtask layout-check` passes (791 assertions, 0 pending) and every finished unit
   was compiled with `xtask build-tu`: **238 units, 3 failures, all three pre-existing and
   reproducible with the unmodified `materialize`** (`lib/allocator.rs` cannot find `c_ulong` -
   its `use` line predates the `strtoul` entry commit `16cf988` added to the shared
   `include/stdlib.rs`; `lib/sigdescr_np.rs` cannot find `c_void`/`c_uint`;
   `lib/canonicalize-lgpl.rs` sees two `F_OK` definitions in unistd.in.rs/fcntl.in.rs).  They are
   other units' files and are left for their owners; `materialize` itself is not implicated
   (`git stash push -- femacs/xtask/src/main.rs` reproduces all three).
2. `femacs/src/include/regex-emacs.rs`: `struct re_registers` (regex-emacs.h 28-35) is added -
   C declares it *before* including lisp.h precisely because thread.h needs it, and this port is
   its first consumer (`free_search_regs`); docs/ports/include-regex-emacs.md's box is ticked.
   The mirror's other pending differences (`EMACS_REGEX_H`'s `1`, `re_match_object`,
   `emacs_re_max_failures`, `emacs_re_safe_alloca`) are unchanged - they are that mirror's own
   WIP, not this unit's.
3. `docs/baseline/layout/`: `asserted-keys.txt` grows by the six `handler` keys and
   `counts.tsv`'s floor moves 785 -> 791 asserted / 6 -> 0 pending, with the round note.  The
   thread.h mirror is what declares `struct handler`, so the baseline's six keys (measured by
   the frozen C compiler: `sizeof handler 288`, `tag_or_ch@8`, `val@24`, `next@32`,
   `nextfree@40`, `jmp@60`) assert and pin the record - and with it the mirror's `sys_jmp_buf`
   (192 bytes, alignment 4, where lisp.rs's `jmp_buf` is the opaque `[u8; 192]`).  The
   assertions come from the C compiler's numbers; nothing was hand-tuned.
4. `docs/ports/macro-body-notes.tsv`: the two rows above plus one for the thread.h mirror's
   `NEAR_STACK_TOP`.
5. `docs/ports/include-thread.md`: the remaining boxes are ticked and its Decisions record the
   mirror.
