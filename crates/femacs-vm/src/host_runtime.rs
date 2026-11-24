#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]
//! Single-process headless VM runtime boundary (`C1`).
//!
//! Exposes a safe, returnable host interface over `femacs-vm` without activating a GUI
//! toolkit, opening a controlling TTY, or entering the interactive editor command loop.
//! `unsafe` is confined strictly to VM initialization, stack-bottom anchoring, `handlerlist`
//! condition-case guards, `specpdl` unwinding, and short `NoGcToken` raw-byte reads.

use core::ffi::c_char;
use core::fmt;
use core::hint::black_box;
use std::ffi::{CString, OsString};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

use crate::alloc::Fcons;
use crate::common::{
    CONDITION_CASE, CONSP, EMACS_INT, EQ, Ferror_message_string, Ffboundp, Fgarbage_collect, Fget,
    Findirect_variable, Fload, Fmemq, Fprin1_to_string, Fset, Lisp_Object, NILP,
    Qcommand_line_processed, Qend_of_file, Qerror_conditions, Qfile_error, Qinvalid_read_syntax,
    Qquit, Qt, SBYTES, SDATA, STRINGP, SYMBOLP, Vload_path, Vpdumper_fingerprint, Vquit_flag,
    Vtop_level, XCAR, XCDR, XFIXNUM, build_utf8_str, current_thread, dumped_with_pdumper_p,
    fflush, handler, intern_c_string, lisp_h_Qnil, make_fixnum, specbind, stderr, stdout,
    unbind_to_safe, wrap_specpdl_ref,
};
use crate::data::Fsymbol_name;
use crate::eval::{Feval, push_handler_nosignal};
use crate::frame::{Fframe_list, Fframep};
use crate::lread::Fread_from_string;
use crate::nonlocal::{ScopedLispRoot, catch_vm_nonlocal};
use crate::roots::{GcContext, RootingCounters, current_thread_key, unbind_vm_lock_owner};
use crate::terminal::{Fterminal_list, Fterminal_live_p};

/// Host VM lifecycle state: `0` = uninitialized, `1` = active, `2` = shut down / failed.
static HOST_INIT_STATE: AtomicU8 = AtomicU8::new(0);
static HOST_ANCHOR_DEPTH: AtomicUsize = AtomicUsize::new(0);

/// Bootstrap source selected when initializing a single-process headless `femacs-vm` instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HostBootstrap {
    /// Initialize C-level VM subsystems only (no `loadup.el`, no portable dump).
    ColdMinimal,
    /// Initialize VM subsystems and cold-load `loadup.el` with `command-line-processed = t`.
    ColdLoadup,
    /// Restore pre-dumped VM state from a verified `.pdmp` file before finishing runtime setup.
    DumpFile(PathBuf),
}

/// Configuration for `init_headless_host`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostInitConfig {
    pub program_name: String,
    pub bootstrap: HostBootstrap,
    pub load_paths: Vec<PathBuf>,
    pub quiet_bootstrap: bool,
}

impl HostInitConfig {
    /// Cold-start config running only core VM initialization (no `loadup.el`).
    pub fn cold_minimal() -> Self {
        Self {
            program_name: "femacs-host".to_string(),
            bootstrap: HostBootstrap::ColdMinimal,
            load_paths: Vec::new(),
            quiet_bootstrap: true,
        }
    }

    /// Cold-start config loading `loadup.el` from `lisp_dir`.
    pub fn cold_loadup(lisp_dir: impl Into<PathBuf>) -> Self {
        Self {
            program_name: "femacs-host".to_string(),
            bootstrap: HostBootstrap::ColdLoadup,
            load_paths: vec![lisp_dir.into()],
            quiet_bootstrap: true,
        }
    }

    /// Portable-dump startup config restoring from `dump_path`.
    pub fn with_dump(dump_path: impl Into<PathBuf>) -> Self {
        Self {
            program_name: "femacs-host".to_string(),
            bootstrap: HostBootstrap::DumpFile(dump_path.into()),
            load_paths: Vec::new(),
            quiet_bootstrap: true,
        }
    }

    /// Append an extra directory to prepend to Lisp `load-path` after VM bootstrap.
    pub fn with_load_path(mut self, dir: impl Into<PathBuf>) -> Self {
        self.load_paths.push(dir.into());
        self
    }
}

/// Recoverable host errors returned by `init_headless_host` and host Lisp operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VmHostError {
    AlreadyInitialized,
    InitFailed(String),
    Read {
        symbol: String,
        message: String,
        data: String,
    },
    LispCondition {
        symbol: String,
        message: String,
        data: String,
    },
    Quit {
        message: String,
    },
    Io {
        symbol: String,
        message: String,
        data: String,
    },
}

impl fmt::Display for VmHostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyInitialized => f.write_str("femacs VM host already initialized"),
            Self::InitFailed(msg) => write!(f, "femacs VM init failed: {msg}"),
            Self::Read {
                symbol,
                message,
                data,
            } => write!(f, "Lisp read error ({symbol}): {message} [{data}]"),
            Self::LispCondition {
                symbol,
                message,
                data,
            } => write!(f, "Lisp condition ({symbol}): {message} [{data}]"),
            Self::Quit { message } => write!(f, "Lisp quit: {message}"),
            Self::Io {
                symbol,
                message,
                data,
            } => write!(f, "Lisp I/O error ({symbol}): {message} [{data}]"),
        }
    }
}

impl std::error::Error for VmHostError {}

/// Observable frame/terminal/GC state reported by a headless host instance (`A1`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeadlessDiagnostics {
    pub noninteractive: bool,
    pub inhibit_window_system: bool,
    pub display_graphic_p: bool,
    pub frame_count: usize,
    pub frame_types: Vec<String>,
    pub terminal_count: usize,
    pub terminal_types: Vec<String>,
    pub dumped_with_pdumper: bool,
    pub pdumper_fingerprint: String,
    pub specpdl_depth: isize,
    pub rooting_counters: RootingCounters,
}

#[inline]
fn current_specpdl_index() -> isize {
    // SAFETY: `current_thread` is initialized and non-null whenever a live `GcContext` exists.
    unsafe {
        let ct = current_thread.get();
        (*ct).m_specpdl_ptr as isize - (*ct).m_specpdl as isize
    }
}

struct StackAnchorGuard {
    ct: *mut crate::common::thread_state,
    saved_bottom: *const c_char,
}

impl Drop for StackAnchorGuard {
    #[inline(always)]
    fn drop(&mut self) {
        HOST_ANCHOR_DEPTH.fetch_sub(1, Ordering::Relaxed);
        // SAFETY: `self.ct` is the VM-locked active `thread_state`; restores prior `m_stack_bottom`
        // unconditionally on normal return or unwind.
        unsafe {
            if !self.ct.is_null() {
                (*self.ct).m_stack_bottom = self.saved_bottom;
            }
        }
    }
}

/// Anchor `current_thread->m_stack_bottom` onto this live host stack frame while executing
/// `body`, replacing any dead initialization stack pointer before GC conservative stack scanning.
#[inline(never)]
fn with_host_stack_anchor<R>(ctx: &mut GcContext, body: impl FnOnce(&mut GcContext) -> R) -> R {
    let anchor: usize = 0;
    let anchor_ptr = (&raw const anchor) as *const c_char;
    // SAFETY: Caller holds exclusive `&mut GcContext` on the VM-lock-owning OS thread; updates
    // `m_stack_bottom` to a live aligned frame local and restores it in `StackAnchorGuard::drop`.
    let guard = unsafe {
        let ct = current_thread.get();
        let depth = HOST_ANCHOR_DEPTH.fetch_add(1, Ordering::Relaxed);
        let saved_bottom = if ct.is_null() {
            core::ptr::null()
        } else {
            let prev = (*ct).m_stack_bottom;
            if depth == 0 || prev.is_null() {
                (*ct).m_stack_bottom = anchor_ptr;
            }
            prev
        };
        StackAnchorGuard { ct, saved_bottom }
    };
    let result = body(ctx);
    drop(guard);
    black_box(anchor);
    result
}

/// Execute `f` with the live host stack frame anchored and exclusive `&mut GcContext` access.
pub fn with_anchored_context<R>(ctx: &mut GcContext, f: impl FnOnce(&mut GcContext) -> R) -> R {
    with_host_stack_anchor(ctx, f)
}

/// Extract an owned UTF-8 `String` from an already-rooted Lisp string under active VM lock.
///
/// # Safety
/// `raw` must be kept live across this call (rooted on the active VM thread or static), with
/// zero Lisp evaluations or GC safepoints occurring while the byte slice borrow is alive.
unsafe fn raw_string_to_owned(raw: Lisp_Object) -> Option<String> {
    if !STRINGP(raw) {
        return None;
    }
    let len = SBYTES(raw);
    if len < 0 {
        return None;
    }
    if len == 0 {
        return Some(String::new());
    }
    let ptr = SDATA(raw) as *const u8;
    // SAFETY: Caller guarantees active VM lock ownership, no safepoint during read, and valid
    // rooted/static string storage of `len > 0` bytes.
    let bytes = unsafe { core::slice::from_raw_parts(ptr, len as usize) };
    Some(String::from_utf8_lossy(bytes).into_owned())
}

#[inline]
unsafe fn raw_symbol_to_owned(sym: Lisp_Object) -> String {
    if SYMBOLP(sym) {
        // SAFETY: `Fsymbol_name` returns the symbol's live non-moving name string.
        unsafe { raw_string_to_owned(Fsymbol_name(sym)) }.unwrap_or_else(|| "error".to_string())
    } else {
        // SAFETY: `sym` is rooted by caller and `Fprin1_to_string` produces a valid string.
        unsafe {
            let formatted = Fprin1_to_string(sym, lisp_h_Qnil(), lisp_h_Qnil());
            let _fmt_root = ScopedLispRoot::new(&[formatted]);
            raw_string_to_owned(formatted).unwrap_or_else(|| "error".to_string())
        }
    }
}

#[inline]
unsafe fn raw_prin1_to_owned(val: Lisp_Object) -> String {
    // SAFETY: Caller passes a valid tagged `Lisp_Object` on the active VM thread; both `val`
    // and the printed result string are held in `ScopedLispRoot` across `raw_string_to_owned`.
    unsafe {
        let mut root = ScopedLispRoot::new(&[val]);
        let rendered = Fprin1_to_string(val, lisp_h_Qnil(), lisp_h_Qnil());
        root.set(0, rendered);
        raw_string_to_owned(rendered).unwrap_or_else(|| "nil".to_string())
    }
}

struct HandlerPopGuard {
    expected_handler: *mut handler,
    active: bool,
}

impl HandlerPopGuard {
    #[inline]
    fn pop_now(&mut self) {
        if self.active {
            self.active = false;
            // SAFETY: Pops the `CONDITION_CASE` handler pushed by `with_host_condition_catch` when
            // still at `handlerlist!()` on the active VM-lock-owning thread.
            unsafe {
                if handlerlist!() == self.expected_handler && !self.expected_handler.is_null() {
                    handlerlist!() = (*self.expected_handler).next;
                }
            }
        }
    }
}

impl Drop for HandlerPopGuard {
    #[inline]
    fn drop(&mut self) {
        self.pop_now();
    }
}

fn format_error_payload_message(err_val: Lisp_Object) -> String {
    // SAFETY: `err_val` is already rooted by the enclosing `ScopedLispRoot`; catches any secondary
    // error while formatting `error-message-string` so host error conversion never unwinds.
    let inner_res = unsafe {
        let sym = intern_c_string(c"error-message-string".as_ptr());
        if NILP(Ffboundp(sym)) {
            Ok(Fprin1_to_string(err_val, lisp_h_Qnil(), lisp_h_Qnil()))
        } else {
            let h = push_handler_nosignal(Qt(), CONDITION_CASE);
            let mut pop_guard = HandlerPopGuard {
                expected_handler: h,
                active: !h.is_null(),
            };
            let res = catch_vm_nonlocal(|| Ferror_message_string(err_val));
            pop_guard.pop_now();
            res
        }
    };
    match inner_res {
        Ok(rendered) if STRINGP(rendered) => {
            // SAFETY: Roots `rendered` across string extraction under active VM lock.
            unsafe {
                let _r = ScopedLispRoot::new(&[rendered]);
                raw_string_to_owned(rendered).unwrap_or_else(|| "error".to_string())
            }
        }
        // SAFETY: `err_val` is live and rooted by caller.
        _ => unsafe { raw_prin1_to_owned(err_val) },
    }
}

fn classify_vm_exit(err_val: Lisp_Object) -> VmHostError {
    let (err_sym, err_data) = if CONSP(err_val) {
        (XCAR(err_val), XCDR(err_val))
    } else {
        (err_val, lisp_h_Qnil())
    };
    // SAFETY: Caller roots `err_val` (and thus `err_sym`/`err_data`) under active VM lock.
    let (symbol, data, message) = unsafe {
        (
            raw_symbol_to_owned(err_sym),
            raw_prin1_to_owned(err_data),
            format_error_payload_message(err_val),
        )
    };

    if EQ(err_sym, Qquit()) {
        return VmHostError::Quit { message };
    }
    let conditions = if SYMBOLP(err_sym) {
        Fget(err_sym, Qerror_conditions())
    } else {
        lisp_h_Qnil()
    };
    if !NILP(Fmemq(Qquit(), conditions)) {
        return VmHostError::Quit { message };
    }
    if !NILP(Fmemq(Qinvalid_read_syntax(), conditions))
        || !NILP(Fmemq(Qend_of_file(), conditions))
    {
        return VmHostError::Read {
            symbol,
            message,
            data,
        };
    }
    if !NILP(Fmemq(Qfile_error(), conditions)) {
        return VmHostError::Io {
            symbol,
            message,
            data,
        };
    }
    VmHostError::LispCondition {
        symbol,
        message,
        data,
    }
}

fn with_host_condition_catch<R>(
    ctx: &mut GcContext,
    body: impl FnOnce() -> R,
) -> Result<R, VmHostError> {
    with_host_stack_anchor(ctx, |ctx| {
        // SAFETY: Pushes an outermost `CONDITION_CASE(t)` catch frame on the VM-lock-owning thread,
        // intercepts `VmNonLocalUnwind` via `catch_vm_nonlocal`, roots `exit.value`, and pops the
        // handler cleanly in `HandlerPopGuard`.
        let res = unsafe {
            let c = push_handler_nosignal(Qt(), CONDITION_CASE);
            if c.is_null() {
                return Err(VmHostError::InitFailed(
                    "failed to push VM condition handler (out of memory)".to_string(),
                ));
            }
            let mut pop_guard = HandlerPopGuard {
                expected_handler: c,
                active: true,
            };
            let eval_res = ctx.with_reentrant_safepoint(|| catch_vm_nonlocal(body));
            pop_guard.pop_now();
            eval_res
        };
        match res {
            Ok(val) => Ok(val),
            Err(exit) => {
                // SAFETY: `exit.value` was held live in `inflight_value` until caught and is
                // immediately rooted on this thread across error classification and string formatting.
                let _exit_root = unsafe { ScopedLispRoot::new(&[exit.value]) };
                // SAFETY: Clears `quit-flag` on the single VM thread so subsequent calls run cleanly.
                unsafe {
                    *Vquit_flag.as_mut_ptr() = lisp_h_Qnil();
                }
                Err(classify_vm_exit(exit.value))
            }
        }
    })
}

fn prepend_load_paths_raw(paths: &[PathBuf]) {
    for path in paths.iter().rev() {
        let s = path.to_string_lossy();
        // SAFETY: Allocates Lisp string and cons cell under VM lock before any external exposure.
        unsafe {
            let path_obj = build_utf8_str(&s);
            let _root = ScopedLispRoot::new(&[path_obj]);
            let cur = *Vload_path.as_mut_ptr();
            if NILP(Fmemq(path_obj, cur)) {
                *Vload_path.as_mut_ptr() = Fcons(path_obj, cur);
            }
        }
    }
}

/// Initialize the single-process headless Lisp VM cleanly and return its exclusive `GcContext`.
pub fn init_headless_host(config: &HostInitConfig) -> Result<GcContext, VmHostError> {
    if HOST_INIT_STATE
        .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
        .is_err()
    {
        return Err(VmHostError::AlreadyInitialized);
    }

    let exe_str = std::env::current_exe()
        .ok()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| config.program_name.clone());
    let argv0_c = match CString::new(exe_str.as_str()) {
        Ok(c) => c,
        Err(_) => CString::new("femacs-host").unwrap(),
    };
    let argv0_raw = argv0_c.into_raw();

    let dump_c = match &config.bootstrap {
        HostBootstrap::DumpFile(path) => {
            let path_s = path.to_string_lossy();
            match CString::new(path_s.as_ref()) {
                Ok(c) => Some(c),
                Err(_) => {
                    // SAFETY: Reclaims leaked `argv0_raw` allocated above before returning early.
                    unsafe { drop(CString::from_raw(argv0_raw)) };
                    HOST_INIT_STATE.store(2, Ordering::Release);
                    return Err(VmHostError::InitFailed(format!(
                        "dump path contains interior NUL byte: {}",
                        path.display()
                    )));
                }
            }
        }
        HostBootstrap::ColdMinimal | HostBootstrap::ColdLoadup => None,
    };
    let dump_ptr = dump_c.as_ref().map_or(core::ptr::null(), |c| c.as_ptr());

    // SAFETY: Single-init enforced by `HOST_INIT_STATE` CAS (`0 -> 1`); `argv0_raw` intentionally
    // stays live for process lifetime because `copy_args` stores `argv[0]` in `initial_argv0`.
    let init_res = unsafe { crate::emacs::init_headless_vm_body(argv0_raw, dump_ptr) };
    if let Err(msg) = init_res {
        if current_thread_key() != 0 {
            // SAFETY: Best-effort lock unbind on early bootstrap error while on owner thread.
            unsafe { unbind_vm_lock_owner() };
        }
        HOST_INIT_STATE.store(2, Ordering::Release);
        return Err(VmHostError::InitFailed(msg));
    }

    let Some(mut ctx) = GcContext::current() else {
        HOST_INIT_STATE.store(2, Ordering::Release);
        return Err(VmHostError::InitFailed(
            "VM initialized without active GcContext lease".to_string(),
        ));
    };

    if !config.load_paths.is_empty() {
        let lp_res = with_host_condition_catch(&mut ctx, || {
            prepend_load_paths_raw(&config.load_paths);
        });
        if let Err(err) = lp_res {
            shutdown_headless_host(ctx);
            return Err(err);
        }
    }

    if matches!(config.bootstrap, HostBootstrap::ColdLoadup) && !dumped_with_pdumper_p() {
        let loadup_res = with_host_condition_catch(&mut ctx, || {
            let count = current_specpdl_index();
            // SAFETY: `count` captures current `specpdl` offset; `Qcommand_line_processed`,
            // `inhibit_message`, `loadup.el`, and `Vtop_level` are manipulated under the VM
            // condition catch guard on the active VM-lock-owning thread.
            unsafe {
                // Prevent `(eval top-level t)` at the end of `loadup.el` from entering
                // `normal-top-level` -> `command-line` -> `(kill-emacs)`.
                Fset(Qcommand_line_processed(), Qt());
                if config.quiet_bootstrap {
                    let sym = intern_c_string(c"inhibit-message".as_ptr());
                    specbind(sym, Qt());
                }
                let file = build_utf8_str("loadup.el");
                let _root = ScopedLispRoot::new(&[file]);
                Fload(
                    file,
                    lisp_h_Qnil(),
                    if config.quiet_bootstrap {
                        Qt()
                    } else {
                        lisp_h_Qnil()
                    },
                    lisp_h_Qnil(),
                    lisp_h_Qnil(),
                );
                *Vtop_level.as_mut_ptr() = lisp_h_Qnil();
                unbind_to_safe(wrap_specpdl_ref(count), lisp_h_Qnil());
            }
            prepend_load_paths_raw(&config.load_paths);
        });
        if let Err(err) = loadup_res {
            shutdown_headless_host(ctx);
            return Err(err);
        }
    }

    Ok(ctx)
}

/// Shut down the headless host, flushing output streams and marking the process-wide VM state spent.
pub fn shutdown_headless_host(ctx: GcContext) {
    drop(ctx);
    // SAFETY: Ownership of the sole `GcContext` was released by `drop(ctx)` above, so zero active
    // VM/root borrows remain on this OS thread; clears `Vquit_flag`, flushes stdio, and unbinds
    // VM lock ownership before marking `HOST_INIT_STATE` spent.
    unsafe {
        if current_thread_key() != 0 {
            *Vquit_flag.as_mut_ptr() = lisp_h_Qnil();
        }
        if !(*stdout.as_mut_ptr()).is_null() {
            fflush(*stdout.as_mut_ptr());
        }
        if !(*stderr.as_mut_ptr()).is_null() {
            fflush(*stderr.as_mut_ptr());
        }
        if current_thread_key() != 0 {
            unbind_vm_lock_owner();
        }
    }
    HOST_INIT_STATE.store(2, Ordering::Release);
}

#[inline]
fn next_non_ws_char_index(source: &str, start_char: usize) -> Option<usize> {
    let mut in_comment = false;
    for (idx, ch) in source.chars().enumerate().skip(start_char) {
        if in_comment {
            if ch == '\n' {
                in_comment = false;
            }
            continue;
        }
        if ch == ';' {
            in_comment = true;
            continue;
        }
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\x0b' | '\x0c') {
            continue;
        }
        return Some(idx);
    }
    None
}

/// Read and sequentially evaluate every top-level Lisp form in `source`, returning the last
/// result printed with `prin1-to-string`.
pub fn read_eval_to_string(ctx: &mut GcContext, source: &str) -> Result<String, VmHostError> {
    with_host_condition_catch(ctx, || {
        // SAFETY: Runs inside `with_host_condition_catch` on the VM-locked active thread;
        // `ScopedLispRoot` keeps `src_lisp`, `last_val`, and current `form` rooted across every
        // `Fread_from_string`, `Feval`, and `Fprin1_to_string` safepoint.
        unsafe {
            let src_lisp = build_utf8_str(source);
            let mut roots = ScopedLispRoot::new(&[src_lisp, lisp_h_Qnil(), lisp_h_Qnil()]);
            let mut pos_char = 0usize;
            let mut evaluated_any = false;
            let mut last_val = lisp_h_Qnil();

            loop {
                let Some(next_start) = next_non_ws_char_index(source, pos_char) else {
                    if !evaluated_any {
                        Fread_from_string(src_lisp, make_fixnum(0), lisp_h_Qnil());
                    }
                    break;
                };
                let pair = Fread_from_string(
                    src_lisp,
                    make_fixnum(next_start as EMACS_INT),
                    lisp_h_Qnil(),
                );
                if !CONSP(pair) {
                    break;
                }
                let form = XCAR(pair);
                let next_idx = if crate::common::FIXNUMP(XCDR(pair)) {
                    XFIXNUM(XCDR(pair)).max(0) as usize
                } else {
                    next_start.saturating_add(1)
                };
                roots.set(1, form);
                last_val = Feval(form, Qt());
                roots.set(2, last_val);
                roots.set(1, lisp_h_Qnil());
                evaluated_any = true;
                pos_char = if next_idx > next_start {
                    next_idx
                } else {
                    next_start.saturating_add(1)
                };
            }

            raw_prin1_to_owned(last_val)
        }
    })
}

struct UnbindGuard {
    target: isize,
}

impl Drop for UnbindGuard {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `target` was captured from `current_specpdl_index()` on this VM thread; catches
        // and discards any unwind inside custom cleanup? Only binds `command-line-args-left` list
        // values so `unbind_to_safe` restores bindings without running Lisp callbacks.
        unsafe {
            if current_specpdl_index() > self.target {
                unbind_to_safe(wrap_specpdl_ref(self.target), lisp_h_Qnil());
            }
        }
    }
}

/// Load a Lisp script file with `command-line-args-left` (`argv`) dynamically bound to `argv`.
pub fn load_script_file(
    ctx: &mut GcContext,
    path: &Path,
    argv: &[OsString],
) -> Result<(), VmHostError> {
    let abs_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| VmHostError::Io {
                symbol: "file-error".to_string(),
                message: e.to_string(),
                data: path.display().to_string(),
            })?
            .join(path)
    };
    if abs_path.is_dir() {
        return Err(VmHostError::Io {
            symbol: "file-error".to_string(),
            message: format!("Opening script path: Is a directory, {}", abs_path.display()),
            data: format!("(\"{}\")", abs_path.display()),
        });
    }
    let abs_s = abs_path.to_string_lossy().into_owned();
    let argv_strs: Vec<String> = argv
        .iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();

    with_host_condition_catch(ctx, || {
        // SAFETY: Runs inside `with_host_condition_catch`; roots `file_lisp` and `arg_list` across
        // script loading and restores `command-line-args-left` via `UnbindGuard` on every exit.
        unsafe {
            let count = current_specpdl_index();
            let _unbind = UnbindGuard { target: count };

            let mut arg_list = lisp_h_Qnil();
            let mut list_root = ScopedLispRoot::new(&[arg_list, lisp_h_Qnil()]);
            for item in argv_strs.iter().rev() {
                let s = build_utf8_str(item);
                list_root.set(1, s);
                arg_list = Fcons(s, arg_list);
                list_root.set(0, arg_list);
                list_root.set(1, lisp_h_Qnil());
            }

            let cmd_left_sym = intern_c_string(c"command-line-args-left".as_ptr());
            let argv_sym = intern_c_string(c"argv".as_ptr());
            specbind(cmd_left_sym, arg_list);
            if !EQ(Findirect_variable(argv_sym), cmd_left_sym) {
                specbind(argv_sym, arg_list);
            }

            let file_lisp = build_utf8_str(&abs_s);
            list_root.set(1, file_lisp);
            let script_loader = intern_c_string(c"command-line--load-script".as_ptr());
            if !NILP(Ffboundp(script_loader)) {
                let form = Fcons(script_loader, Fcons(file_lisp, lisp_h_Qnil()));
                list_root.set(0, form);
                Feval(form, Qt());
            } else {
                Fload(file_lisp, lisp_h_Qnil(), Qt(), Qt(), lisp_h_Qnil());
            }
        }
    })
}

/// Trigger VM garbage collection under host stack anchoring and condition protection.
pub fn garbage_collect(ctx: &mut GcContext) -> Result<(), VmHostError> {
    with_host_condition_catch(ctx, || {
        Fgarbage_collect();
    })
}

/// Inspect observable headless frame/terminal/GC/pdumper invariants inside the active VM.
pub fn inspect_headless_environment(ctx: &mut GcContext) -> Result<HeadlessDiagnostics, VmHostError> {
    let rooting_counters = ctx.rooting_counters();
    with_host_condition_catch(ctx, || {
        // SAFETY: Reads active VM flags (`noninteractive`, `inhibit_window_system`), frame list,
        // terminal list, and pdumper state while all temporary Lisp lists stay rooted.
        unsafe {
            let noninteractive = *crate::common::noninteractive.as_ptr();
            let inhibit_ws = *crate::common::inhibit_window_system.as_ptr();

            let frames = Fframe_list();
            let mut roots = ScopedLispRoot::new(&[frames, lisp_h_Qnil(), lisp_h_Qnil()]);
            let mut frame_types = Vec::new();
            let mut tail = frames;
            while CONSP(tail) {
                let frame = XCAR(tail);
                let ftype = Fframep(frame);
                roots.set(1, ftype);
                frame_types.push(raw_prin1_to_owned(ftype));
                tail = XCDR(tail);
            }
            let display_graphic_p = frame_types.iter().any(|t| {
                matches!(
                    t.as_str(),
                    "x" | "w32" | "ns" | "pgtk" | "haiku" | "android"
                )
            });

            let terms = Fterminal_list();
            roots.set(0, terms);
            let mut terminal_types = Vec::new();
            let mut ttail = terms;
            while CONSP(ttail) {
                let term = XCAR(ttail);
                let ttype = Fterminal_live_p(term);
                roots.set(1, ttype);
                terminal_types.push(raw_prin1_to_owned(ttype));
                ttail = XCDR(ttail);
            }

            let fp_obj = *Vpdumper_fingerprint.as_ptr();
            roots.set(2, fp_obj);
            let pdumper_fingerprint = raw_string_to_owned(fp_obj).unwrap_or_default();

            HeadlessDiagnostics {
                noninteractive,
                inhibit_window_system: inhibit_ws,
                display_graphic_p,
                frame_count: frame_types.len(),
                frame_types,
                terminal_count: terminal_types.len(),
                terminal_types,
                dumped_with_pdumper: dumped_with_pdumper_p(),
                pdumper_fingerprint,
                specpdl_depth: current_specpdl_index(),
                rooting_counters,
            }
        }
    })
}
