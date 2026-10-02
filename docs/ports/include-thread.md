# emacs/src/thread.h (348 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 9
- [x] THREADP  (line 223)
- [ ] CHECK_THREAD  (line 229)
- [ ] XTHREAD  (line 235)
- [x] MUTEXP  (line 270)
- [ ] CHECK_MUTEX  (line 276)
- [ ] XMUTEX  (line 282)
- [x] CONDVARP  (line 304)
- [ ] CHECK_CONDVAR  (line 310)
- [ ] XCONDVAR  (line 316)

## Macros defined — 18
- [ ] NEAR_STACK_TOP  (line 48, body 6cdaa7a1d50c)
- [x] THREAD_H  (line 20, body da39a3ee5e6b)
- [ ] catchlist  (line 116, body cb5c09e99277)
- [ ] current_buffer  (line 147, body eb15fc402cec)
- [ ] getcjmp  (line 192, body 0fe4438edfd4)
- [ ] handlerlist  (line 124, body 08be7e85272b)
- [ ] handlerlist_sentinel  (line 127, body 741e3b4a60b4)
- [ ] last_thing_searched  (line 70, body fb4d9e1171db)
- [ ] lisp_eval_depth  (line 143, body 13e5ef8950d8)
- [ ] saved_last_thing_searched  (line 73, body eedb9d369660)
- [ ] saved_search_regs  (line 170, body 159a26365f27)
- [ ] search_regs  (line 167, body f4f45536979f)
- [ ] specpdl  (line 131, body 637bcc207c70)
- [ ] specpdl_end  (line 135, body efeee5859651)
- [ ] specpdl_ptr  (line 139, body da130d7467d7)
- [ ] stack_bottom  (line 104, body e83207b0d71c)
- [ ] waiting_for_input  (line 185, body a40d0ead008a)
- [ ] waiting_for_user_input_p  (line 181, body a9a31aad1a21)

## Types and their fields — 4
- [ ] Lisp_CondVar: union vectorlike_header header, Lisp_Object mutex, Lisp_Object name, sys_cond_t cond
- [ ] Lisp_Mutex: union vectorlike_header header, Lisp_Object name, lisp_mutex_t mutex
- [ ] bc_thread_state: struct bc_frame* fp, char* stack, char* stack_end
- [ ] thread_state: union vectorlike_header header, Lisp_Object m_last_thing_searched, Lisp_Object m_saved_last_thing_searched, Lisp_Object name, Lisp_Object function, Lisp_Object result, Lisp_Object error_symbol, Lisp_Object error_data, Lisp_Object buffer_disposition, Lisp_Object event_object, JNIEnv* java_env, char const* m_stack_bottom, void const* stack_top, struct catchtag* m_catchlist, struct handler* m_handlerlist, struct handler* m_handlerlist_sentinel, union specbinding* m_specpdl, union specbinding* m_specpdl_end, union specbinding* m_specpdl_ptr, intmax_t m_lisp_eval_depth, struct buffer* m_current_buffer, struct re_registers m_search_regs, struct re_registers m_saved_search_regs, int m_waiting_for_user_input_p, bool m_waiting_for_input, sys_jmp_buf m_getcjmp, sys_thread_t thread_id, sys_cond_t thread_condvar, sys_cond_t* wait_condvar, char* thread_name, int not_holding_lock, struct thread_state* next_thread, struct bc_thread_state bc

## File-scope objects — 2
- [ ] extern struct thread_state current_thread  (line 322)
- [ ] extern struct thread_state all_threads  (line 323)
