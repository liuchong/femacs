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
    static NEXT_CONTEXT_EPOCH: VmGlobal<u64> = VmGlobal::<u64>::new (0);
    static CURRENT_CONTEXT_EPOCH: VmGlobal<u64> = VmGlobal::<u64>::new (0);
    static CONTEXT_SCOPE_TOKEN: VmGlobal<u64> = VmGlobal::<u64>::new (0);
    static INIT_IN_PROGRESS: VmGlobal<bool> = VmGlobal::<bool>::new (false);
    static CURRENT_SCOPE: VmGlobal<*mut CompScopeGuard> = VmGlobal::<*mut CompScopeGuard>::new (std::ptr::null_mut());
    static CLEANUP_STATUS: VmGlobal<u32> = VmGlobal::<u32>::new (0);
    pub(super) unsafe fn note_cleanup_failure(bits: u32) {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe { (*CLEANUP_STATUS.as_mut_ptr ()) |= bits; 
  }
}

    unsafe fn reserve() -> u64 {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe {
        let Some(token) = (*NEXT_CONTEXT_EPOCH.as_mut_ptr ()).checked_add(1) else {
            error(c"Native compiler context identity exhausted".as_ptr());
        };
        (*NEXT_CONTEXT_EPOCH.as_mut_ptr ()) = token;
        token
    
  }
}
    pub(super) unsafe fn reset_process() {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe {
        // Dumped pointers describe the writing process. Do not close/dereference
        // them in the fresh process, and never reuse cached emitter addresses.
        (*comp.as_mut_ptr ()).ctxt = std::ptr::null_mut();
        (*logfile.as_mut_ptr ()) = std::ptr::null_mut();
        (*comp.as_mut_ptr ()).emitter_dispatcher = Qnil();
        (*super::comp_u3.as_mut_ptr ()) = None;
        (*NEXT_CONTEXT_EPOCH.as_mut_ptr ()) = 0;
        (*CURRENT_CONTEXT_EPOCH.as_mut_ptr ()) = 0;
        (*CONTEXT_SCOPE_TOKEN.as_mut_ptr ()) = 0;
        (*INIT_IN_PROGRESS.as_mut_ptr ()) = false;
        (*CURRENT_SCOPE.as_mut_ptr ()) = std::ptr::null_mut();
        (*CLEANUP_STATUS.as_mut_ptr ()) = 0;
    
  }
}
    unsafe fn release_matching(epoch: u64) {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe {
        if epoch == 0 || (*CURRENT_CONTEXT_EPOCH.as_mut_ptr ()) != epoch { return; }
        let context = (*comp.as_mut_ptr ()).ctxt;
        let stream = (*logfile.as_mut_ptr ());
        // Clear all published ownership BEFORE backend/stdio cleanup.
        (*comp.as_mut_ptr ()).ctxt = std::ptr::null_mut();
        (*logfile.as_mut_ptr ()) = std::ptr::null_mut();
        (*super::comp_u3.as_mut_ptr ()) = None;
        (*CURRENT_CONTEXT_EPOCH.as_mut_ptr ()) = 0;
        (*CONTEXT_SCOPE_TOKEN.as_mut_ptr ()) = 0;
        if !context.is_null() {
            #[cfg(feature = "windowsnt")]
            {
                if fn_gcc_jit_context_release.is_none() { (*CLEANUP_STATUS.as_mut_ptr ()) |= 2; }
                else { gcc_jit_context_release(context); }
            }
            #[cfg(not(feature = "windowsnt"))]
            { gcc_jit_context_release(context); }
        }
        if !stream.is_null() && emacs_fclose!(stream) != 0 { (*CLEANUP_STATUS.as_mut_ptr ()) |= 1; }
    
  }
}
    pub(super) unsafe fn release_public() {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe { release_matching((*CURRENT_CONTEXT_EPOCH.as_mut_ptr ())); 
  }
}

    unsafe extern "C-unwind" fn cleanup_init(pointer: *mut c_void) {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe {
        let guard = &mut *(pointer as *mut CompInitGuard);
        if !guard.armed { return; }
        if !guard.owned_ctxt.is_null() && guard.owned_ctxt == (*comp.as_mut_ptr ()).ctxt {
            release_matching(guard.token);
        }
        if !guard.had_emitters && !guard.emitters_ready {
            (*comp.as_mut_ptr ()).emitter_dispatcher = Qnil();
        }
        (*INIT_IN_PROGRESS.as_mut_ptr ()) = false;
        guard.armed = false;
    
  }
}
    pub(super) unsafe fn begin_init(guard: &mut CompInitGuard) {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe {
        if (*INIT_IN_PROGRESS.as_mut_ptr ()) || !(*comp.as_mut_ptr ()).ctxt.is_null() {
            xsignal1(Qnative_ice(), build_string(c"compiler context already taken".as_ptr()));
        }
        guard.token = reserve();
        guard.had_emitters = !NILP((*comp.as_mut_ptr ()).emitter_dispatcher);
        // The caller owns this stack address until unbind_to before return.
        record_unwind_protect_ptr(Some(cleanup_init), (guard as *mut CompInitGuard).cast());
        guard.armed = true;
        (*INIT_IN_PROGRESS.as_mut_ptr ()) = true;
    
  }
}
    pub(super) unsafe fn emitters_ready(guard: &mut CompInitGuard) { guard.emitters_ready = true; }
    pub(super) unsafe fn acquire(guard: &mut CompInitGuard) {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe {
        let context = gcc_jit_context_acquire();
        if context.is_null() {
            xsignal1(Qnative_ice(), build_string(c"native compiler context acquisition failed".as_ptr()));
        }
        guard.owned_ctxt = context;
        (*comp.as_mut_ptr ()).ctxt = context;
        (*CURRENT_CONTEXT_EPOCH.as_mut_ptr ()) = guard.token;
        (*CONTEXT_SCOPE_TOKEN.as_mut_ptr ()) = if (*CURRENT_SCOPE.as_mut_ptr ()).is_null() { 0 } else { (*(*CURRENT_SCOPE.as_mut_ptr ())).token };
    
  }
}
    pub(super) unsafe fn finish_init(guard: &mut CompInitGuard) {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe {
        let message = gcc_jit_context_get_first_error((*comp.as_mut_ptr ()).ctxt);
        if !message.is_null() { xsignal1(Qnative_ice(), build_string(message)); }
        (*INIT_IN_PROGRESS.as_mut_ptr ()) = false;
        guard.armed = false;
    
  }
}
    unsafe extern "C-unwind" fn mark_scope(pointer: *mut c_void) {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe {
        let guard = &*(pointer as *const CompScopeGuard);
        mark_object(guard.callback);
        mark_object(guard.result);
    
  }
}
    unsafe extern "C-unwind" fn cleanup_scope(pointer: *mut c_void) {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe {
        let guard = &mut *(pointer as *mut CompScopeGuard);
        if !guard.armed { return; }
        if (*CONTEXT_SCOPE_TOKEN.as_mut_ptr ()) == guard.token { release_matching((*CURRENT_CONTEXT_EPOCH.as_mut_ptr ())); }
        if (*CURRENT_SCOPE.as_mut_ptr ()) == guard as *mut CompScopeGuard { (*CURRENT_SCOPE.as_mut_ptr ()) = guard.previous_scope; }
        else { (*CLEANUP_STATUS.as_mut_ptr ()) |= 4; }
        guard.armed = false;
    
  }
}
    pub(super) unsafe fn begin_scope(guard: &mut CompScopeGuard) {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe {
        if (*INIT_IN_PROGRESS.as_mut_ptr ()) || !(*comp.as_mut_ptr ()).ctxt.is_null() {
            xsignal1(Qnative_ice(), build_string(c"compiler context already taken".as_ptr()));
        }
        guard.token = reserve();
        guard.previous_scope = (*CURRENT_SCOPE.as_mut_ptr ());
        record_unwind_protect_ptr_mark(Some(cleanup_scope),
            (guard as *mut CompScopeGuard).cast(), Some(mark_scope));
        guard.armed = true;
        (*CURRENT_SCOPE.as_mut_ptr ()) = guard;
    
  }
}
    pub(super) unsafe fn verify_scope(guard: &CompScopeGuard) {
  // SAFETY: native `.eln` JIT/`libgccjit`/dynamic-linker boundary; upholds ABI_VERSION "14" status-helper and relocation table invariants without unwinding across C JIT frames.
  unsafe {
        if (*comp.as_mut_ptr ()).ctxt.is_null() || (*CURRENT_CONTEXT_EPOCH.as_mut_ptr ()) == 0
            || (*CONTEXT_SCOPE_TOKEN.as_mut_ptr ()) != guard.token
            || (*CURRENT_SCOPE.as_mut_ptr ()) != guard as *const CompScopeGuard as *mut CompScopeGuard {
            xsignal1(Qnative_ice(), build_string(c"advised init did not create an owned compiler context".as_ptr()));
        }
    
  }
}
