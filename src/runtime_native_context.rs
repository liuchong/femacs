// P-346: private native compiler resource scopes. Mounted as `mod native_context` by comp.rs.
// No Rust destructor crosses a Lisp nonlocal exit; specpdl owns stack guards.
use super::*;

    pub(super) struct CompInitGuard {
        token: u64,
        owned_ctxt: *mut gcc_jit_context,
        armed: bool,
        had_emitters: bool,
        emitters_ready: bool,
    }
    impl CompInitGuard {
        pub(super) fn empty() -> Self {
            Self { token: 0, owned_ctxt: std::ptr::null_mut(), armed: false,
                had_emitters: false, emitters_ready: false }
        }
    }
    pub(super) struct CompScopeGuard {
        token: u64,
        previous_scope: *mut CompScopeGuard,
        pub(super) callback: Lisp_Object,
        pub(super) result: Lisp_Object,
        armed: bool,
    }
    impl CompScopeGuard {
        pub(super) unsafe fn empty(callback: Lisp_Object) -> Self {
            Self { token: 0, previous_scope: std::ptr::null_mut(), callback,
                result: Qnil(), armed: false }
        }
    }
    static mut NEXT_CONTEXT_EPOCH: u64 = 0;
    static mut CURRENT_CONTEXT_EPOCH: u64 = 0;
    static mut CONTEXT_SCOPE_TOKEN: u64 = 0;
    static mut INIT_IN_PROGRESS: bool = false;
    static mut CURRENT_SCOPE: *mut CompScopeGuard = std::ptr::null_mut();
    static mut CLEANUP_STATUS: u32 = 0;
    pub(super) unsafe fn note_cleanup_failure(bits: u32) { CLEANUP_STATUS |= bits; }

    unsafe fn reserve() -> u64 {
        let Some(token) = NEXT_CONTEXT_EPOCH.checked_add(1) else {
            error(c"Native compiler context identity exhausted".as_ptr());
        };
        NEXT_CONTEXT_EPOCH = token;
        token
    }
    pub(super) unsafe fn reset_process() {
        // Dumped pointers describe the writing process. Do not close/dereference
        // them in the fresh process, and never reuse cached emitter addresses.
        comp.ctxt = std::ptr::null_mut();
        logfile = std::ptr::null_mut();
        comp.emitter_dispatcher = Qnil();
        super::comp_u3 = None;
        NEXT_CONTEXT_EPOCH = 0;
        CURRENT_CONTEXT_EPOCH = 0;
        CONTEXT_SCOPE_TOKEN = 0;
        INIT_IN_PROGRESS = false;
        CURRENT_SCOPE = std::ptr::null_mut();
        CLEANUP_STATUS = 0;
    }
    unsafe fn release_matching(epoch: u64) {
        if epoch == 0 || CURRENT_CONTEXT_EPOCH != epoch { return; }
        let context = comp.ctxt;
        let stream = logfile;
        // Clear all published ownership BEFORE backend/stdio cleanup.
        comp.ctxt = std::ptr::null_mut();
        logfile = std::ptr::null_mut();
        super::comp_u3 = None;
        CURRENT_CONTEXT_EPOCH = 0;
        CONTEXT_SCOPE_TOKEN = 0;
        if !context.is_null() {
            #[cfg(feature = "windowsnt")]
            {
                if fn_gcc_jit_context_release.is_none() { CLEANUP_STATUS |= 2; }
                else { gcc_jit_context_release(context); }
            }
            #[cfg(not(feature = "windowsnt"))]
            { gcc_jit_context_release(context); }
        }
        if !stream.is_null() && emacs_fclose!(stream) != 0 { CLEANUP_STATUS |= 1; }
    }
    pub(super) unsafe fn release_public() { release_matching(CURRENT_CONTEXT_EPOCH); }

    unsafe extern "C-unwind" fn cleanup_init(pointer: *mut c_void) {
        let guard = &mut *(pointer as *mut CompInitGuard);
        if !guard.armed { return; }
        if !guard.owned_ctxt.is_null() && guard.owned_ctxt == comp.ctxt {
            release_matching(guard.token);
        }
        if !guard.had_emitters && !guard.emitters_ready {
            comp.emitter_dispatcher = Qnil();
        }
        INIT_IN_PROGRESS = false;
        guard.armed = false;
    }
    pub(super) unsafe fn begin_init(guard: &mut CompInitGuard) {
        if INIT_IN_PROGRESS || !comp.ctxt.is_null() {
            xsignal1(Qnative_ice(), build_string(c"compiler context already taken".as_ptr()));
        }
        guard.token = reserve();
        guard.had_emitters = !NILP(comp.emitter_dispatcher);
        // The caller owns this stack address until unbind_to before return.
        record_unwind_protect_ptr(Some(cleanup_init), (guard as *mut CompInitGuard).cast());
        guard.armed = true;
        INIT_IN_PROGRESS = true;
    }
    pub(super) unsafe fn emitters_ready(guard: &mut CompInitGuard) { guard.emitters_ready = true; }
    pub(super) unsafe fn acquire(guard: &mut CompInitGuard) {
        let context = gcc_jit_context_acquire();
        if context.is_null() {
            xsignal1(Qnative_ice(), build_string(c"native compiler context acquisition failed".as_ptr()));
        }
        guard.owned_ctxt = context;
        comp.ctxt = context;
        CURRENT_CONTEXT_EPOCH = guard.token;
        CONTEXT_SCOPE_TOKEN = if CURRENT_SCOPE.is_null() { 0 } else { (*CURRENT_SCOPE).token };
    }
    pub(super) unsafe fn finish_init(guard: &mut CompInitGuard) {
        let message = gcc_jit_context_get_first_error(comp.ctxt);
        if !message.is_null() { xsignal1(Qnative_ice(), build_string(message)); }
        INIT_IN_PROGRESS = false;
        guard.armed = false;
    }
    unsafe extern "C-unwind" fn mark_scope(pointer: *mut c_void) {
        let guard = &*(pointer as *const CompScopeGuard);
        mark_object(guard.callback);
        mark_object(guard.result);
    }
    unsafe extern "C-unwind" fn cleanup_scope(pointer: *mut c_void) {
        let guard = &mut *(pointer as *mut CompScopeGuard);
        if !guard.armed { return; }
        if CONTEXT_SCOPE_TOKEN == guard.token { release_matching(CURRENT_CONTEXT_EPOCH); }
        if CURRENT_SCOPE == guard as *mut CompScopeGuard { CURRENT_SCOPE = guard.previous_scope; }
        else { CLEANUP_STATUS |= 4; }
        guard.armed = false;
    }
    pub(super) unsafe fn begin_scope(guard: &mut CompScopeGuard) {
        if INIT_IN_PROGRESS || !comp.ctxt.is_null() {
            xsignal1(Qnative_ice(), build_string(c"compiler context already taken".as_ptr()));
        }
        guard.token = reserve();
        guard.previous_scope = CURRENT_SCOPE;
        record_unwind_protect_ptr_mark(Some(cleanup_scope),
            (guard as *mut CompScopeGuard).cast(), Some(mark_scope));
        guard.armed = true;
        CURRENT_SCOPE = guard;
    }
    pub(super) unsafe fn verify_scope(guard: &CompScopeGuard) {
        if comp.ctxt.is_null() || CURRENT_CONTEXT_EPOCH == 0
            || CONTEXT_SCOPE_TOKEN != guard.token
            || CURRENT_SCOPE != guard as *const CompScopeGuard as *mut CompScopeGuard {
            xsignal1(Qnative_ice(), build_string(c"advised init did not create an owned compiler context".as_ptr()));
        }
    }
