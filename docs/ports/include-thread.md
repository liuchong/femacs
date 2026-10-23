# emacs/src/thread.h (348 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 9
- [x] THREADP  (line 223)
- [x] CHECK_THREAD  (line 229)
- [x] XTHREAD  (line 235)
- [x] MUTEXP  (line 270)
- [x] CHECK_MUTEX  (line 276)
- [x] XMUTEX  (line 282)
- [x] CONDVARP  (line 304)
- [x] CHECK_CONDVAR  (line 310)
- [x] XCONDVAR  (line 316)

## Macros defined — 18
- [x] NEAR_STACK_TOP  (line 48, body 6cdaa7a1d50c)
- [x] THREAD_H  (line 20, body da39a3ee5e6b)
- [x] catchlist  (line 116, body cb5c09e99277)
- [x] current_buffer  (line 147, body eb15fc402cec)
- [x] getcjmp  (line 192, body 0fe4438edfd4)
- [x] handlerlist  (line 124, body 08be7e85272b)
- [x] handlerlist_sentinel  (line 127, body 741e3b4a60b4)
- [x] last_thing_searched  (line 70, body fb4d9e1171db)
- [x] lisp_eval_depth  (line 143, body 13e5ef8950d8)
- [x] saved_last_thing_searched  (line 73, body eedb9d369660)
- [x] saved_search_regs  (line 170, body 159a26365f27)
- [x] search_regs  (line 167, body f4f45536979f)
- [x] specpdl  (line 131, body 637bcc207c70)
- [x] specpdl_end  (line 135, body efeee5859651)
- [x] specpdl_ptr  (line 139, body da130d7467d7)
- [x] stack_bottom  (line 104, body e83207b0d71c)
- [x] waiting_for_input  (line 185, body a40d0ead008a)
- [x] waiting_for_user_input_p  (line 181, body a9a31aad1a21)

## Types and their fields — 4
- [x] Lisp_CondVar: union vectorlike_header header, Lisp_Object mutex, Lisp_Object name, sys_cond_t cond
- [x] Lisp_Mutex: union vectorlike_header header, Lisp_Object name, lisp_mutex_t mutex
- [x] bc_thread_state: struct bc_frame* fp, char* stack, char* stack_end
- [x] thread_state: union vectorlike_header header, Lisp_Object m_last_thing_searched, Lisp_Object m_saved_last_thing_searched, Lisp_Object name, Lisp_Object function, Lisp_Object result, Lisp_Object error_symbol, Lisp_Object error_data, Lisp_Object buffer_disposition, Lisp_Object event_object, JNIEnv* java_env, char const* m_stack_bottom, void const* stack_top, struct catchtag* m_catchlist, struct handler* m_handlerlist, struct handler* m_handlerlist_sentinel, union specbinding* m_specpdl, union specbinding* m_specpdl_end, union specbinding* m_specpdl_ptr, intmax_t m_lisp_eval_depth, struct buffer* m_current_buffer, struct re_registers m_search_regs, struct re_registers m_saved_search_regs, int m_waiting_for_user_input_p, bool m_waiting_for_input, sys_jmp_buf m_getcjmp, sys_thread_t thread_id, sys_cond_t thread_condvar, sys_cond_t* wait_condvar, char* thread_name, int not_holding_lock, struct thread_state* next_thread, struct bc_thread_state bc

## File-scope objects — 2
- [x] extern struct thread_state current_thread  (line 322)
- [x] extern struct thread_state all_threads  (line 323)

## Decisions

`emacs/src/thread.h` @ `fd7afced403`; port `src/include/thread.rs` (320 lines vs C's
348); the gate is
`python3 tools/extract-abi.py --check emacs/src/thread.h src/include/thread.rs` -> `OK`.
The header's seven conditions are decided by
`docs/baseline/config/darwin-arm64.macros.tsv` (`HAVE_ANDROID`, `ANDROID_STUBIFY`, `WINDOWSNT`,
`MSDOS` all `undef`, `HAVE___BUILTIN_FRAME_ADDRESS` 1), and
`python3 tools/check-cond-eval.py src/thread.h` reports
`7 conditions, 0 unknown, 0 disagreements [VERIFIED]`, so the branch selection is the
clang-verified one.

The mirror lands with `src/thread.c`, its first consumer: `lisp.h` 2289's include is why the
predicates were already here (`data.c`'s type predicates), and `thread_state`, the two Lisp
records, the checkpoint machinery and the prototypes are written now.

### Dead text (recorded, not transcribed)

- thread.h 24-26's `#ifdef WINDOWSNT` `<sys/socket.h>`; 28-31's `#ifdef MSDOS`
  `<time.h>`/`<signal.h>`; 32-38's `#ifdef HAVE_ANDROID` (`"android.h"` and the `JNIEnv
  *java_env` member of `thread_state`, C 96-99); 45-49's `#else` (`NEAR_STACK_TOP (addr) (addr)`)
  - `HAVE___BUILTIN_FRAME_ADDRESS` is 1, so the `__builtin_frame_address (0)` arm is live and is
  rendered with the frame-pointer register (PORTING.md 4.3); 3870-3872's `#ifdef HAVE_X_WINDOWS`
  `x_error_handler_depth` member of `struct handler`.
- The `INLINE_HEADER_BEGIN`/`INLINE_HEADER_END` pair has no counterpart (excluded-macros.txt).

### Included mirrors, and the names C declares elsewhere

- `include!("systhread.rs")` (thread.h 41), `signal.rs` (thread.h 40's sysselect.h ->
  `<signal.h>`, for `sigset_t`), `sys_select.rs` (thread.h 40's sysselect.h, for `fd_set` and,
  through it, `struct timespec`) and `regex-emacs.rs` (thread.h 21, for `struct re_registers`) -
  the C include list, minus the dead arms above.
- `sys_jmp_buf` (lisp.h 2272-2273) and `struct handler` (lisp.h 3839-3873) are declared in this
  mirror, because the mirror names them and a mirror may only name a type it declares or reaches
  through its own includes (P-72).  They are the two to drop when `lisp.rs` carries them; the
  `handler` record also turns the frozen baseline's six `handler` layout keys from pending into
  asserted (see docs/ports/thread.md's "Changes outside the unit").
- `lisp.rs` supplies `handlertype`, `nonlocal_exit`, `specpdl_ref`, `specbinding`, `jmp_buf`
  and the `Q*`/`XSET*`/`CHECK_TYPE`/`PSEUDOVECTORP` names the inline functions use.

### Deviations recorded

1. **`sys_jmp_buf` is a wrapper, not lisp.rs's alias.**  C's `sys_jmp_buf` is `jmp_buf`, which is
   `int[48]` with alignment 4 on this target, while lisp.rs's `jmp_buf` is the opaque
   `[u8; 192]` (alignment 1) its layout note records.  `thread_state` *embeds* one, so the port
   wraps the array with `#[repr(C, align(4))]` to keep `m_getcjmp@216` and
   `sizeof thread_state == 472` - both measured with clang against the frozen configuration and
   asserted here.  A unit that hands it to `sys_setjmp` casts to `*mut jmp_buf`.
2. **Untyped pointers (P-55).**  `m_catchlist` (`struct catchtag *`), `m_current_buffer`
   (`struct buffer *`), `bc.fp`/`handler.act_rec` (`struct bc_frame *`) have no mirror that this
   one can reach, so they keep the pointer with a `/// C:` line; the use sites cast.
3. **`thread_check_current_buffer`'s parameter** is `struct buffer *` in C and `*mut c_void`
   here (rule 2); the unit's definition matches.
4. **`NEAR_STACK_TOP`**'s live body is C's builtin arm; the port renders
   `__builtin_frame_address (0)` with the frame-pointer register, so the builtin's name and its
   `0` are in `docs/ports/macro-body-notes.tsv` (the comp.c decision, PORTING.md 4.3).
5. **Layout assertions.**  The frozen baseline has no keys for the thread records, so - like
   dired.rs's `struct dirent` - this mirror carries the measured numbers as `const _`
   assertions (`sizeof thread_state 472`, `event_object@72`, `m_specpdl@120`, `m_getcjmp@216`,
   `thread_id@408`, `not_holding_lock@432`, `bc@448`; `bc_thread_state` 24/8/16;
   `lisp_mutex_t` 16; `Lisp_Mutex` 32; `Lisp_CondVar` 32; `sys_jmp_buf` 192/align 4) instead.
   They were printed by clang against the frozen configuration (`spikes/layout_c.c`'s flags).
6. The object declarations and the prototypes the *unit* defines live in `pub mod thread_objs` /
   `pub mod thread_decls` + glob import (P-21/0.1-10), so `src/thread.rs`'s own
   `#[unsafe(no_mangle)]` definitions shadow the glob instead of colliding with it.
