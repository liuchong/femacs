#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]
//! Single-process headless VM runtime boundary (`C1`).
//!
//! Exposes a safe, returnable host interface over `femacs-vm` without activating a GUI
//! toolkit, opening a controlling TTY, or entering the interactive editor command loop.
//! `unsafe` is confined strictly to VM initialization, stack-bottom anchoring, `handlerlist`
//! condition-case guards, `specpdl` unwinding, and short `NoGcToken` raw-byte reads.

use core::ffi::{c_char, c_int, c_void};
use core::fmt;
use core::hint::black_box;
use core::marker::PhantomData;
use std::ffi::{CString, OsString};
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, AtomicUsize, Ordering};

use crate::alloc::Fcons;
use crate::common::{
    AREF, ASET, ASIZE, BIGNUMP, CONDITION_CASE, CONSP, EMACS_INT, EQ, FD_CLOEXEC,
    FLOATP, F_DUPFD_CLOEXEC, F_GETFD, F_SETFD, Fboundp, Fdefault_toplevel_value,
    Ferror_message_string, Fexpand_file_name, Ffboundp, Ffile_name_as_directory, Ffile_regular_p,
    Ffuncall, Fgarbage_collect, Fget, Findirect_variable, Fintern, Fload, Fmake_hash_table,
    Fmake_vector, Fmemq, Fprin1_to_string, Fputhash, Fset, Fsymbol_value, HASH_KEY,
    HASH_TABLE_P, HASH_TABLE_SIZE, HASH_VALUE, Lisp_Object, NILP, O_RDWR, QCtest,
    Qcommand_line_processed, Qend_of_file, Qequal, Qerror_conditions, Qfile_error,
    Qinvalid_read_syntax, Qkill_emacs_hook, Qlexical_binding, Qquit, Qt, Qunbound, SBYTES,
    SDATA, STRINGP, SYMBOLP, VECTORP, Vcharset_map_path, Vdata_directory, Vdoc_directory,
    Vlexical_binding, Vload_path, Vpdumper_fingerprint, Vpurify_flag, Vquit_flag, Vtop_level,
    XCAR, XCDR, XFIXNUM, XFLOAT_DATA, XHASH_TABLE, build_utf8_str, close, current_thread,
    dumped_with_pdumper_p, dup2, fcntl, fflush, handler, hash_unused_entry_key_p,
    integer_to_intmax_opt, intern_c_string, intmax_t, lisp_h_Qnil, make_fixnum, make_float,
    make_int, open, safe_run_hooks, specbind, stderr, stdout, unbind_to_safe, wrap_specpdl_ref,
};
use crate::data::{Fsymbol_name, find_symbol_value};
use crate::eval::{Feval, push_handler_nosignal};
use crate::frame::{Fframe_list, Fframep};
use crate::lread::Fread_from_string;
use crate::nonlocal::{
    HostUnwindOutcome, ScopedLispRoot, VmResult, catch_vm_nonlocal,
    catch_vm_nonlocal_or_host_exit, raise_host_exit,
};
use crate::pdumper::Fdump_emacs_portable;
use crate::roots::{GcContext, RootingCounters, current_thread_key, unbind_vm_lock_owner};
use crate::terminal::{Fterminal_list, Fterminal_live_p};

/// Host VM lifecycle state: `0` = uninitialized, `1` = active, `2` = shut down / failed.
static HOST_INIT_STATE: AtomicU8 = AtomicU8::new(0);
static HOST_ANCHOR_DEPTH: AtomicUsize = AtomicUsize::new(0);
static HEADLESS_INTERCEPT_KILL_EMACS: AtomicBool = AtomicBool::new(false);
static HOST_EXIT_UNWINDING: AtomicBool = AtomicBool::new(false);
static HOST_KILL_HOOK_RUNNING: AtomicBool = AtomicBool::new(false);
static PENDING_HOST_EXIT: AtomicU64 = AtomicU64::new(0);

#[cfg(target_os = "macos")]
const SYS_SOL_SOCKET: c_int = 0xffff;
#[cfg(target_os = "macos")]
const SYS_SO_TYPE: c_int = 0x1008;
#[cfg(not(target_os = "macos"))]
const SYS_SOL_SOCKET: c_int = 1;
#[cfg(not(target_os = "macos"))]
const SYS_SO_TYPE: c_int = 3;
const SYS_SOCK_STREAM: c_int = 1;
const SYS_AF_UNIX: u16 = 1;

#[repr(C)]
struct RawSockaddrStorageHeader {
    first: u8,
    second: u8,
    _rest: [u8; 126],
}

impl RawSockaddrStorageHeader {
    fn zeroed() -> Self {
        Self {
            first: 0,
            second: 0,
            _rest: [0u8; 126],
        }
    }

    #[inline]
    fn family(&self) -> u16 {
        #[cfg(target_os = "macos")]
        {
            u16::from(self.second)
        }
        #[cfg(not(target_os = "macos"))]
        {
            u16::from_ne_bytes([self.first, self.second])
        }
    }
}

unsafe extern "C" {
    fn getsockopt(
        socket: c_int,
        level: c_int,
        option_name: c_int,
        option_value: *mut c_void,
        option_len: *mut u32,
    ) -> c_int;
    fn getsockname(socket: c_int, address: *mut c_void, address_len: *mut u32) -> c_int;
}

/// Return whether the single-process headless host currently intercepts `(kill-emacs)`.
#[inline]
pub(crate) fn is_headless_host_active() -> bool {
    HEADLESS_INTERCEPT_KILL_EMACS.load(Ordering::Acquire)
}

/// Return whether a headless `(kill-emacs)` unwind is currently skipping Lisp cleanup handlers.
#[inline]
pub(crate) fn is_host_exit_unwinding() -> bool {
    HOST_EXIT_UNWINDING.load(Ordering::Acquire)
}

/// Clear the headless host-exit unwind flag after an ABI/thread boundary intercepts `HostExit`
/// for clean local teardown before resuming propagation on the VM unwind stack.
#[inline]
pub(crate) fn clear_host_exit_unwinding() {
    HOST_EXIT_UNWINDING.store(false, Ordering::Release);
}

/// Resume or initiate a headless host-exit unwind (`VmHostExitUnwind(code)`) with
/// `HOST_EXIT_UNWINDING` armed so enclosing Lisp cleanup handlers cannot swallow or alter `code`.
#[inline(never)]
pub(crate) fn resume_host_exit_unwind(code: i32) -> ! {
    HOST_EXIT_UNWINDING.store(true, Ordering::Release);
    raise_host_exit(code)
}

/// Record a process-wide pending host exit from a subthread or deferred boundary without unwinding
/// across `extern "C"` frames. Preserves the first recorded exit code until consumed by main thread.
pub(crate) fn record_pending_host_exit(code: i32) {
    let encoded = (1u64 << 32) | u64::from(code as u32);
    let _ = PENDING_HOST_EXIT.compare_exchange(0, encoded, Ordering::AcqRel, Ordering::Acquire);
    // SAFETY: Caller holds the active VM global lock or is tearing down a Lisp thread; setting
    // `pending_signals` wakes any active Lisp loop via `probably_quit()`, and the non-blocking
    // self-pipe write wakes any blocking main-thread `pselect` via `thread_select`.
    unsafe {
        if current_thread_key() != 0 {
            *crate::common::pending_signals.as_mut_ptr() = true;
            crate::process::notify_host_exit_io_wakeup();
        }
    }
}

/// Return whether a deferred subthread/host exit request is waiting for main-thread propagation.
#[inline]
pub(crate) fn has_pending_host_exit() -> bool {
    PENDING_HOST_EXIT.load(Ordering::Acquire) != 0
}

/// Atomically consume any pending deferred host-exit request, returning its exit code.
#[inline]
pub(crate) fn take_pending_host_exit() -> Option<i32> {
    let raw = PENDING_HOST_EXIT.swap(0, Ordering::AcqRel);
    if raw == 0 {
        None
    } else {
        // SAFETY: Only drains the non-blocking self-pipe when an active VM thread is bound.
        unsafe {
            if current_thread_key() != 0 {
                crate::process::drain_host_exit_io_wakeup();
            }
        }
        Some((raw & 0xffff_ffff) as u32 as i32)
    }
}

/// If running on the VM main thread while headless mode is active and a deferred host exit is
/// pending, consume it and initiate `VmHostExitUnwind(code)`.
///
/// # Safety
/// Must be called on the active VM-lock-owning thread inside an active `with_host_condition_catch_ctx`
/// unwind scope.
pub(crate) unsafe fn propagate_pending_host_exit_if_main() {
    if !has_pending_host_exit() || !is_headless_host_active() {
        return;
    }
    // SAFETY: Caller holds VM global lock on active Lisp thread.
    unsafe {
        let ct = *current_thread.as_mut_ptr();
        if !ct.is_null() && crate::thread::main_thread_p(ct.cast()) {
            if let Some(code) = take_pending_host_exit() {
                resume_host_exit_unwind(code);
            }
        }
    }
}

struct HostExitUnwindResetGuard;

impl Drop for HostExitUnwindResetGuard {
    #[inline]
    fn drop(&mut self) {
        HOST_EXIT_UNWINDING.store(false, Ordering::Release);
    }
}

struct HostKillHookGuard;

impl Drop for HostKillHookGuard {
    #[inline]
    fn drop(&mut self) {
        HOST_KILL_HOOK_RUNNING.store(false, Ordering::Release);
    }
}

/// Intercept `(kill-emacs)` in headless mode, running `kill-emacs-hook` if bound and raising an
/// uncatchable host-exit unwind (`VmHostExitUnwind`) caught strictly at `with_host_condition_catch_ctx`.
///
/// # Safety
/// Must only be called on the active VM-lock-owning thread inside `with_host_condition_catch_ctx`
/// after `Fkill_emacs` confirms non-restart batch termination.
pub(crate) unsafe fn intercept_headless_kill_emacs(arg: Lisp_Object) -> ! {
    // SAFETY: Caller verified active VM lock ownership and headless host state.
    unsafe {
        let exit_code: i32 = if crate::common::FIXNUMP(arg) {
            let n = XFIXNUM(arg);
            if n < 0 {
                (n | i32::MIN as EMACS_INT) as i32
            } else {
                (n & i32::MAX as EMACS_INT) as i32
            }
        } else {
            0
        };
        let hook_sym = Qkill_emacs_hook();
        let hook_val = find_symbol_value(hook_sym);
        if !NILP(hook_val)
            && !EQ(hook_val, Qunbound())
            && !is_host_exit_unwinding()
            && !HOST_KILL_HOOK_RUNNING.swap(true, Ordering::AcqRel)
        {
            let _hook_guard = HostKillHookGuard;
            safe_run_hooks(hook_sym);
        }
        flush_c_stdio_raw();
        let ct = *current_thread.as_mut_ptr();
        if !ct.is_null() && !crate::thread::main_thread_p(ct.cast()) {
            record_pending_host_exit(exit_code);
        }
        resume_host_exit_unwind(exit_code);
    }
}

/// Bootstrap source selected when initializing a single-process headless `lisp-vm` instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HostBootstrap {
    /// Initialize VM subsystems only (no boot script, no portable dump).
    ColdMinimal,
    /// Initialize VM subsystems and cold-load `loadup.el` with `command-line-processed = t`.
    ColdLoadup,
    /// Initialize VM subsystems and cold-load a custom Lisp bootstrap script path.
    ColdBootScript(PathBuf),
    /// Restore pre-dumped VM state from a verified `.pdmp` file before finishing runtime setup.
    DumpFile(PathBuf),
    /// Try restoring from `dump_path`, falling back cleanly to cold-booting `boot_script` if dump load fails.
    DumpOrBootScript {
        dump_path: PathBuf,
        boot_script: PathBuf,
    },
}

/// Safe host-owned Lisp value representation exchanged across the `HeadlessVm` boundary.
#[derive(Clone, Debug, PartialEq)]
pub enum VmValue {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),
    List(Vec<VmValue>),
    Vector(Vec<VmValue>),
    Map(Vec<(String, VmValue)>),
    Opaque(String),
}

/// Configuration for headless `HeadlessVm::init`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostInitConfig {
    pub program_name: String,
    pub bootstrap: HostBootstrap,
    pub load_paths: Vec<PathBuf>,
    pub data_dir: Option<PathBuf>,
    pub replace_load_path: bool,
    pub quiet_bootstrap: bool,
    pub defer_delayed_custom_init: bool,
}

impl HostInitConfig {
    /// Cold-start config running only core VM initialization (no boot script).
    pub fn cold_minimal() -> Self {
        Self {
            program_name: "lisp-vm".to_string(),
            bootstrap: HostBootstrap::ColdMinimal,
            load_paths: Vec::new(),
            data_dir: None,
            replace_load_path: false,
            quiet_bootstrap: true,
            defer_delayed_custom_init: false,
        }
    }

    /// Cold-start config loading `loadup.el` from `lisp_dir`.
    pub fn cold_loadup(lisp_dir: impl Into<PathBuf>) -> Self {
        Self {
            program_name: "lisp-vm".to_string(),
            bootstrap: HostBootstrap::ColdLoadup,
            load_paths: vec![lisp_dir.into()],
            data_dir: None,
            replace_load_path: false,
            quiet_bootstrap: true,
            defer_delayed_custom_init: false,
        }
    }

    /// Cold-start config loading a custom boot script file (with `lisp_dir` on `load-path`).
    pub fn cold_boot_script(lisp_dir: impl Into<PathBuf>, boot_script: impl Into<PathBuf>) -> Self {
        Self {
            program_name: "lisp-vm".to_string(),
            bootstrap: HostBootstrap::ColdBootScript(boot_script.into()),
            load_paths: vec![lisp_dir.into()],
            data_dir: None,
            replace_load_path: false,
            quiet_bootstrap: true,
            defer_delayed_custom_init: false,
        }
    }

    /// Portable-dump startup config restoring from `dump_path`.
    pub fn with_dump(dump_path: impl Into<PathBuf>) -> Self {
        Self {
            program_name: "lisp-vm".to_string(),
            bootstrap: HostBootstrap::DumpFile(dump_path.into()),
            load_paths: Vec::new(),
            data_dir: None,
            replace_load_path: false,
            quiet_bootstrap: true,
            defer_delayed_custom_init: false,
        }
    }

    /// Try restoring from `dump_path` and fall back to cold-booting `boot_script` on dump load failure.
    pub fn dump_or_boot_script(
        lisp_dir: impl Into<PathBuf>,
        dump_path: impl Into<PathBuf>,
        boot_script: impl Into<PathBuf>,
    ) -> Self {
        Self {
            program_name: "lisp-vm".to_string(),
            bootstrap: HostBootstrap::DumpOrBootScript {
                dump_path: dump_path.into(),
                boot_script: boot_script.into(),
            },
            load_paths: vec![lisp_dir.into()],
            data_dir: None,
            replace_load_path: false,
            quiet_bootstrap: true,
            defer_delayed_custom_init: false,
        }
    }

    /// Set the program identity string reported if `current_exe()` is unavailable.
    pub fn with_program_name(mut self, program_name: impl Into<String>) -> Self {
        self.program_name = program_name.into();
        self
    }

    /// Append an extra directory to prepend to Lisp `load-path` after VM bootstrap.
    pub fn with_load_path(mut self, dir: impl Into<PathBuf>) -> Self {
        self.load_paths.push(dir.into());
        self
    }

    /// Override `data-directory`, `doc-directory`, and `charset-map-path` for relocatable startup.
    pub fn with_data_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.data_dir = Some(dir.into());
        self
    }

    /// Replace any `.pdmp`-recorded `load-path` entries with `load_paths` (and their `subdirs.el`).
    pub fn with_replace_load_path(mut self, replace: bool) -> Self {
        self.replace_load_path = replace;
        self
    }

    /// Keep `custom-delayed-init-variables` unevaluated (used before writing a portable dump).
    pub fn with_defer_delayed_custom_init(mut self, defer: bool) -> Self {
        self.defer_delayed_custom_init = defer;
        self
    }
}

/// Recoverable host errors returned by `HeadlessVm::init` and host Lisp operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VmHostError {
    AlreadyInitialized,
    InitFailed(String),
    ExitRequest(i32),
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
            Self::ExitRequest(code) => write!(f, "Lisp exit requested with code {code}"),
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
    // error or host-exit unwind while formatting `error-message-string` so host error conversion
    // never unwinds.
    let inner_res = unsafe {
        let sym = intern_c_string(c"error-message-string".as_ptr());
        if NILP(Ffboundp(sym)) {
            HostUnwindOutcome::Ok(Fprin1_to_string(err_val, lisp_h_Qnil(), lisp_h_Qnil()))
        } else {
            let h = push_handler_nosignal(Qt(), CONDITION_CASE);
            let mut pop_guard = HandlerPopGuard {
                expected_handler: h,
                active: !h.is_null(),
            };
            let res = catch_vm_nonlocal_or_host_exit(|| Ferror_message_string(err_val));
            if let HostUnwindOutcome::HostExit(code) = res {
                record_pending_host_exit(code);
                if !h.is_null() {
                    crate::eval::restore_vm_state_to_host_handler(h);
                }
                clear_host_exit_unwinding();
            }
            pop_guard.pop_now();
            res
        }
    };
    match inner_res {
        HostUnwindOutcome::Ok(rendered) if STRINGP(rendered) => {
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

unsafe fn flush_c_stdio_raw() {
    // SAFETY: Caller holds VM lock or is shutting down the sole headless host; flushes C stdio streams
    // when initialized so piped RPC/REPL/batch output becomes visible immediately.
    unsafe {
        if !(*stdout.as_mut_ptr()).is_null() {
            fflush(*stdout.as_mut_ptr());
        }
        if !(*stderr.as_mut_ptr()).is_null() {
            fflush(*stderr.as_mut_ptr());
        }
    }
}

fn with_host_condition_catch_ctx<R>(
    ctx: &mut GcContext,
    body: impl FnOnce(&mut GcContext) -> VmResult<R>,
) -> Result<R, VmHostError> {
    with_host_stack_anchor(ctx, |ctx| {
        // SAFETY: Pushes an outermost `CONDITION_CASE(t)` catch frame on the VM-lock-owning thread,
        // intercepts `VmNonLocalUnwind`, returned `Err(NonLocalExit)`, and `VmHostExitUnwind`
        // (`kill-emacs`), restoring VM state and clearing `Vquit_flag` cleanly.
        let exit = unsafe {
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
            match catch_vm_nonlocal_or_host_exit(|| body(ctx)) {
                HostUnwindOutcome::Ok(Ok(val)) => {
                    pop_guard.pop_now();
                    flush_c_stdio_raw();
                    if let Some(code) = take_pending_host_exit() {
                        *Vquit_flag.as_mut_ptr() = lisp_h_Qnil();
                        return Err(VmHostError::ExitRequest(code));
                    }
                    return Ok(val);
                }
                HostUnwindOutcome::Ok(Err(exit)) | HostUnwindOutcome::NonLocal(exit) => {
                    pop_guard.pop_now();
                    flush_c_stdio_raw();
                    if let Some(code) = take_pending_host_exit() {
                        *Vquit_flag.as_mut_ptr() = lisp_h_Qnil();
                        return Err(VmHostError::ExitRequest(code));
                    }
                    exit
                }
                HostUnwindOutcome::HostExit(code) => {
                    let _reset_guard = HostExitUnwindResetGuard;
                    crate::eval::restore_vm_state_to_host_handler(c);
                    pop_guard.pop_now();
                    *Vquit_flag.as_mut_ptr() = lisp_h_Qnil();
                    let _ = take_pending_host_exit();
                    flush_c_stdio_raw();
                    drop(_reset_guard);
                    return Err(VmHostError::ExitRequest(code));
                }
            }
        };
        // SAFETY: `exit.value` stays reachable until caught here and is immediately
        // rooted on this thread across error classification and string formatting.
        let _exit_root = unsafe { ScopedLispRoot::new(&[exit.value]) };
        // SAFETY: Clears `quit-flag` on the single VM thread so subsequent calls run cleanly.
        unsafe {
            *Vquit_flag.as_mut_ptr() = lisp_h_Qnil();
        }
        let classified = classify_vm_exit(exit.value);
        if let Some(code) = take_pending_host_exit() {
            return Err(VmHostError::ExitRequest(code));
        }
        Err(classified)
    })
}

fn with_host_condition_catch<R>(
    ctx: &mut GcContext,
    body: impl FnOnce() -> R,
) -> Result<R, VmHostError> {
    with_host_condition_catch_ctx(ctx, |ctx| Ok(ctx.with_reentrant_safepoint(body)))
}

fn add_load_path_and_subdirs_raw(path: &Path) {
    let raw_s = path.to_string_lossy();
    // SAFETY: Expands `raw_s` against `default-directory` under active VM lock, keeps strings
    // rooted in `ScopedLispRoot`, prepends to `Vload_path`, and loads `subdirs.el` with
    // `default-directory` dynamically bound via `UnbindGuard`.
    unsafe {
        let raw_obj = build_utf8_str(&raw_s);
        let mut roots = ScopedLispRoot::new(&[raw_obj, lisp_h_Qnil(), lisp_h_Qnil()]);
        let expanded_obj = Fexpand_file_name(raw_obj, lisp_h_Qnil());
        roots.set(0, expanded_obj);
        let expanded_s = raw_string_to_owned(expanded_obj).unwrap_or_else(|| raw_s.into_owned());
        let trimmed = if expanded_s.len() > 1 {
            expanded_s.trim_end_matches('/')
        } else {
            expanded_s.as_str()
        };
        let path_obj = if trimmed.len() == expanded_s.len() {
            expanded_obj
        } else {
            let obj = build_utf8_str(trimmed);
            roots.set(0, obj);
            obj
        };
        let cur = *Vload_path.as_mut_ptr();
        if NILP(Fmemq(path_obj, cur)) {
            *Vload_path.as_mut_ptr() = Fcons(path_obj, cur);
        }
        let subdirs_el = PathBuf::from(trimmed).join("subdirs.el");
        if subdirs_el.is_file() {
            let add_sym = intern_c_string(c"normal-top-level-add-to-load-path".as_ptr());
            if !NILP(Ffboundp(add_sym)) {
                let count = current_specpdl_index();
                let _unbind = UnbindGuard { target: count };
                let dir_slash = Ffile_name_as_directory(path_obj);
                roots.set(1, dir_slash);
                let dd_sym = intern_c_string(c"default-directory".as_ptr());
                specbind(dd_sym, dir_slash);
                let sub_s = subdirs_el.to_string_lossy();
                let sub_obj = build_utf8_str(&sub_s);
                roots.set(2, sub_obj);
                Fload(sub_obj, Qt(), Qt(), Qt(), lisp_h_Qnil());
            }
        }
    }
}

fn apply_configured_paths_raw(config: &HostInitConfig) {
    if let Some(data_dir) = &config.data_dir {
        let s = data_dir.to_string_lossy();
        // SAFETY: Updates `Vdata_directory`, `Vdoc_directory`, and `Vcharset_map_path` under active
        // VM lock while keeping allocated strings rooted across expansion.
        unsafe {
            let raw_obj = build_utf8_str(&s);
            let mut roots = ScopedLispRoot::new(&[raw_obj, lisp_h_Qnil()]);
            let dir_obj = Ffile_name_as_directory(raw_obj);
            roots.set(0, dir_obj);
            *Vdata_directory.as_mut_ptr() = dir_obj;
            *Vdoc_directory.as_mut_ptr() = dir_obj;
            let charsets_rel = build_utf8_str("charsets");
            roots.set(1, charsets_rel);
            let charsets_dir = Fexpand_file_name(charsets_rel, dir_obj);
            roots.set(1, charsets_dir);
            *Vcharset_map_path.as_mut_ptr() = Fcons(charsets_dir, lisp_h_Qnil());
        }
    }
    if config.replace_load_path && !config.load_paths.is_empty() {
        // SAFETY: Replaces stale dumped `Vload_path` entries under active VM lock.
        unsafe {
            *Vload_path.as_mut_ptr() = lisp_h_Qnil();
        }
    }
    for path in config.load_paths.iter().rev() {
        add_load_path_and_subdirs_raw(path);
    }
}

fn bind_argv_list_raw(argv_strs: &[String]) {
    // SAFETY: Caller holds VM lock inside an `UnbindGuard` scope and has already captured `specpdl`
    // depth; dynamically binds `command-line-args-left` and `argv` cleanly.
    unsafe {
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
    }
}

/// Read the current Lisp `command-line-args-left` list back as owned host `OsString` entries.
///
/// # Safety
/// Caller holds the active VM lock inside `with_host_condition_catch` while all traversed list
/// links and string elements are rooted via `ScopedLispRoot`.
unsafe fn read_command_line_args_left_raw() -> Vec<OsString> {
    // SAFETY: Guaranteed by caller contract; keeps `cur` and `item` rooted across conversion.
    unsafe {
        let cmd_left_sym = intern_c_string(c"command-line-args-left".as_ptr());
        if NILP(Fboundp(cmd_left_sym)) {
            return Vec::new();
        }
        let mut cur = Fsymbol_value(cmd_left_sym);
        let mut roots = ScopedLispRoot::new(&[cur, lisp_h_Qnil()]);
        let mut out = Vec::new();
        while CONSP(cur) {
            let item = XCAR(cur);
            roots.set(1, item);
            let text = if STRINGP(item) {
                raw_string_to_owned(item).unwrap_or_default()
            } else {
                raw_prin1_to_owned(item)
            };
            out.push(OsString::from(text));
            cur = XCDR(cur);
            roots.set(0, cur);
            roots.set(1, lisp_h_Qnil());
        }
        out
    }
}

fn resolve_abs_path(path: &Path) -> Result<PathBuf, VmHostError> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        std::env::current_dir()
            .map_err(|e| VmHostError::Io {
                symbol: "file-error".to_string(),
                message: e.to_string(),
                data: path.display().to_string(),
            })
            .map(|cwd| cwd.join(path))
    }
}

/// Initialize the single-process headless Lisp VM cleanly and return its exclusive `GcContext`.
fn init_headless_host(config: &HostInitConfig) -> Result<GcContext, VmHostError> {
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

    let (dump_c, allow_dump_fallback) = match &config.bootstrap {
        HostBootstrap::DumpFile(path) => (Some(path), false),
        HostBootstrap::DumpOrBootScript { dump_path, .. } => (Some(dump_path), true),
        HostBootstrap::ColdMinimal | HostBootstrap::ColdLoadup | HostBootstrap::ColdBootScript(_) => {
            (None, false)
        }
    };
    let dump_c = match dump_c {
        Some(path) => {
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
        None => None,
    };
    let dump_ptr = dump_c.as_ref().map_or(core::ptr::null(), |c| c.as_ptr());

    let data_c = match &config.data_dir {
        Some(dir) => {
            let dir_s = dir.to_string_lossy();
            match CString::new(dir_s.as_ref()) {
                Ok(c) => Some(c),
                Err(_) => {
                    // SAFETY: Reclaims leaked `argv0_raw` allocated above before returning early.
                    unsafe { drop(CString::from_raw(argv0_raw)) };
                    HOST_INIT_STATE.store(2, Ordering::Release);
                    return Err(VmHostError::InitFailed(format!(
                        "data_dir contains interior NUL byte: {}",
                        dir.display()
                    )));
                }
            }
        }
        None => None,
    };
    let data_ptr = data_c.as_ref().map_or(core::ptr::null(), |c| c.as_ptr());

    HEADLESS_INTERCEPT_KILL_EMACS.store(true, Ordering::Release);
    // SAFETY: Single-init enforced by `HOST_INIT_STATE` CAS (`0 -> 1`); `argv0_raw` intentionally
    // stays live for process lifetime because `copy_args` stores `argv[0]` in `initial_argv0`.
    let mut init_res = unsafe { crate::emacs::init_headless_vm_body(argv0_raw, dump_ptr, data_ptr) };
    if init_res.is_err() && allow_dump_fallback && !dump_ptr.is_null() {
        // SAFETY: `init_headless_vm_body` only errors on `pdumper_load` before initializing VM
        // threads, globals, or FDs; retrying with `NULL` dump initializes the process cleanly.
        init_res = unsafe {
            crate::emacs::init_headless_vm_body(argv0_raw, core::ptr::null(), data_ptr)
        };
    }
    if let Err(msg) = init_res {
        HEADLESS_INTERCEPT_KILL_EMACS.store(false, Ordering::Release);
        if current_thread_key() != 0 {
            // SAFETY: Best-effort lock unbind on early bootstrap error while on owner thread.
            unsafe { unbind_vm_lock_owner() };
        }
        HOST_INIT_STATE.store(2, Ordering::Release);
        return Err(VmHostError::InitFailed(msg));
    }

    let Some(mut ctx) = GcContext::current() else {
        HEADLESS_INTERCEPT_KILL_EMACS.store(false, Ordering::Release);
        HOST_INIT_STATE.store(2, Ordering::Release);
        return Err(VmHostError::InitFailed(
            "VM initialized without active GcContext lease".to_string(),
        ));
    };

    if config.data_dir.is_some() || !config.load_paths.is_empty() {
        let lp_res = with_host_condition_catch(&mut ctx, || {
            apply_configured_paths_raw(config);
        });
        if let Err(err) = lp_res {
            shutdown_headless_host(ctx);
            return Err(err);
        }
    }

    let cold_boot_target: Option<String> = match &config.bootstrap {
        HostBootstrap::ColdLoadup if !dumped_with_pdumper_p() => Some("loadup.el".to_string()),
        HostBootstrap::ColdBootScript(path) if !dumped_with_pdumper_p() => {
            Some(path.to_string_lossy().into_owned())
        }
        HostBootstrap::DumpOrBootScript { boot_script, .. } if !dumped_with_pdumper_p() => {
            Some(boot_script.to_string_lossy().into_owned())
        }
        _ => None,
    };
    if let Some(boot_file) = cold_boot_target {
        let loadup_res = with_host_condition_catch(&mut ctx, || {
            let count = current_specpdl_index();
            // SAFETY: `count` captures current `specpdl` offset; `Qcommand_line_processed`,
            // `inhibit_message`, boot script, and `Vtop_level` are manipulated under the VM
            // condition catch guard on the active VM-lock-owning thread.
            unsafe {
                Fset(Qcommand_line_processed(), Qt());
                if config.quiet_bootstrap {
                    let sym = intern_c_string(c"inhibit-message".as_ptr());
                    specbind(sym, Qt());
                }
                let file = build_utf8_str(&boot_file);
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
            apply_configured_paths_raw(config);
        });
        if let Err(err) = loadup_res {
            shutdown_headless_host(ctx);
            return Err(err);
        }
    }

    if !matches!(config.bootstrap, HostBootstrap::ColdMinimal) && !config.defer_delayed_custom_init {
        let init_res = with_host_condition_catch(&mut ctx, || {
            // SAFETY: Runs standard runtime locale, coding-system path decoding, `lisp-directory`,
            // and delayed-custom setup on the VM-lock-owning thread matching command-line startup.
            unsafe {
                eval_forms_raw(
                    r#"(progn
  (when (fboundp 'set-locale-environment)
    (set-locale-environment nil))
  (when (and (boundp 'locale-coding-system)
             locale-coding-system
             (fboundp 'decode-coding-string))
    (let ((coding locale-coding-system))
      (dolist (pathsym '(load-path exec-path charset-map-path))
        (when (boundp pathsym)
          (let ((path (symbol-value pathsym)))
            (when (listp path)
              (set pathsym
                   (mapcar (lambda (dir)
                             (if (stringp dir) (decode-coding-string dir coding t) dir))
                           path))))))
      (dolist (filesym '(data-directory doc-directory exec-directory
                         installation-directory invocation-directory
                         invocation-name source-directory))
        (when (boundp filesym)
          (let ((file (symbol-value filesym)))
            (when (stringp file)
              (set filesym (decode-coding-string file coding t))))))))
  (when (and (boundp 'lisp-directory)
             (fboundp 'locate-file)
             (fboundp 'get-load-suffixes))
    (let ((simple-file (locate-file "simple" load-path (get-load-suffixes))))
      (when simple-file
        (setq lisp-directory (file-name-directory simple-file))
        (when (boundp 'load-history)
          (setq load-history
                (mapcar (lambda (elt)
                          (if (and (consp elt)
                                   (stringp (car elt))
                                   (not (file-name-absolute-p (car elt))))
                              (cons (concat lisp-directory (car elt)) (cdr elt))
                            elt))
                        load-history))))))
  (when (and (fboundp 'custom-reevaluate-setting)
             (boundp 'custom-delayed-init-variables)
             (listp custom-delayed-init-variables))
    (let ((user-emacs-directory-warning nil))
      (mapc #'custom-reevaluate-setting
            (reverse custom-delayed-init-variables)))
    (setq custom-delayed-init-variables t))
  (when (and (boundp 'temporary-file-directory)
             (not (and (stringp temporary-file-directory)
                       (> (length temporary-file-directory) 0))))
    (setq temporary-file-directory
          (file-name-as-directory
           (or (getenv "TMPDIR") (getenv "TMP") (getenv "TEMP") "/tmp")))))"#,
                );
            }
        });
        if let Err(err) = init_res {
            shutdown_headless_host(ctx);
            return Err(err);
        }
    }

    Ok(ctx)
}

/// Shut down the headless host, flushing output streams and marking the process-wide VM state spent.
fn shutdown_headless_host(ctx: GcContext) {
    HOST_KILL_HOOK_RUNNING.store(false, Ordering::Release);
    HOST_EXIT_UNWINDING.store(false, Ordering::Release);
    PENDING_HOST_EXIT.store(0, Ordering::Release);
    HEADLESS_INTERCEPT_KILL_EMACS.store(false, Ordering::Release);
    drop(ctx);
    // SAFETY: Ownership of the sole `GcContext` was released by `drop(ctx)` above, so zero active
    // VM/root borrows remain on this OS thread; drains the self-pipe, clears `Vquit_flag`, flushes
    // stdio, and unbinds VM lock ownership before marking `HOST_INIT_STATE` spent.
    unsafe {
        if current_thread_key() != 0 {
            crate::process::drain_host_exit_io_wakeup();
            *Vquit_flag.as_mut_ptr() = lisp_h_Qnil();
        }
        flush_c_stdio_raw();
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

unsafe fn eval_forms_raw(source: &str) -> Lisp_Object {
    // SAFETY: Caller runs inside `with_host_condition_catch` under active VM lock ownership;
    // `ScopedLispRoot` keeps `src_lisp`, current `form`, and `last_val` rooted across every read
    // and evaluation safepoint.
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

        last_val
    }
}

const VM_VALUE_MAX_DEPTH: usize = 128;

unsafe fn raw_to_vm_value(val: Lisp_Object, depth: usize) -> VmValue {
    if depth >= VM_VALUE_MAX_DEPTH {
        // SAFETY: Caller holds VM lock and roots `val`.
        return VmValue::Opaque(unsafe { raw_prin1_to_owned(val) });
    }
    // SAFETY: Caller holds VM lock inside `with_host_condition_catch` with `val` rooted.
    unsafe {
        let _root = ScopedLispRoot::new(&[val]);
        if NILP(val) {
            return VmValue::Nil;
        }
        if EQ(val, Qt()) {
            return VmValue::Bool(true);
        }
        if crate::common::FIXNUMP(val) {
            return VmValue::Int(XFIXNUM(val) as i64);
        }
        if BIGNUMP(val) {
            if let Some(n) = integer_to_intmax_opt(val) {
                if let Ok(exact) = i64::try_from(n) {
                    return VmValue::Int(exact);
                }
            }
            return VmValue::Opaque(raw_prin1_to_owned(val));
        }
        if FLOATP(val) {
            return VmValue::Float(XFLOAT_DATA(val));
        }
        if STRINGP(val) {
            return VmValue::String(raw_string_to_owned(val).unwrap_or_default());
        }
        if SYMBOLP(val) {
            return VmValue::Symbol(raw_symbol_to_owned(val));
        }
        if VECTORP(val) {
            let len = ASIZE(val);
            let mut out = Vec::with_capacity(len.max(0) as usize);
            for idx in 0..len {
                let elem = AREF(val, idx);
                let _elem_root = ScopedLispRoot::new(&[elem]);
                out.push(raw_to_vm_value(elem, depth + 1));
            }
            return VmValue::Vector(out);
        }
        if HASH_TABLE_P(val) {
            let ht = XHASH_TABLE(val);
            let size = HASH_TABLE_SIZE(ht);
            let mut entries = Vec::new();
            for idx in 0..size {
                let key = HASH_KEY(ht, idx);
                if hash_unused_entry_key_p(key) {
                    continue;
                }
                let entry_val = HASH_VALUE(ht, idx);
                let _pair_root = ScopedLispRoot::new(&[key, entry_val]);
                let key_str = if STRINGP(key) {
                    raw_string_to_owned(key).unwrap_or_default()
                } else if SYMBOLP(key) {
                    raw_symbol_to_owned(key)
                } else {
                    raw_prin1_to_owned(key)
                };
                entries.push((key_str, raw_to_vm_value(entry_val, depth + 1)));
            }
            return VmValue::Map(entries);
        }
        if CONSP(val) {
            let mut out = Vec::new();
            let mut cur = val;
            let mut cur_root = ScopedLispRoot::new(&[cur]);
            while CONSP(cur) {
                let head = XCAR(cur);
                let tail = XCDR(cur);
                cur_root.set(0, cur);
                let _head_root = ScopedLispRoot::new(&[head, tail]);
                out.push(raw_to_vm_value(head, depth + 1));
                cur = tail;
                cur_root.set(0, cur);
            }
            if !NILP(cur) {
                out.push(raw_to_vm_value(cur, depth + 1));
            }
            return VmValue::List(out);
        }
        VmValue::Opaque(raw_prin1_to_owned(val))
    }
}

unsafe fn vm_value_to_raw(value: &VmValue) -> Lisp_Object {
    // SAFETY: Caller holds VM lock inside `with_host_condition_catch`; every intermediate Lisp
    // object is rooted via `ScopedLispRoot` across further allocations.
    unsafe {
        match value {
            VmValue::Nil | VmValue::Bool(false) => lisp_h_Qnil(),
            VmValue::Bool(true) => Qt(),
            VmValue::Int(n) => make_int(*n as intmax_t),
            VmValue::Float(f) => make_float(*f),
            VmValue::String(s) | VmValue::Opaque(s) => build_utf8_str(s),
            VmValue::Symbol(s) => {
                let s_obj = build_utf8_str(s);
                let _root = ScopedLispRoot::new(&[s_obj]);
                Fintern(s_obj, lisp_h_Qnil())
            }
            VmValue::List(items) => {
                let mut acc = lisp_h_Qnil();
                let mut roots = ScopedLispRoot::new(&[acc, lisp_h_Qnil()]);
                for item in items.iter().rev() {
                    let elem = vm_value_to_raw(item);
                    roots.set(1, elem);
                    acc = Fcons(elem, acc);
                    roots.set(0, acc);
                    roots.set(1, lisp_h_Qnil());
                }
                acc
            }
            VmValue::Vector(items) => {
                let vec = Fmake_vector(make_fixnum(items.len() as EMACS_INT), lisp_h_Qnil());
                let mut roots = ScopedLispRoot::new(&[vec, lisp_h_Qnil()]);
                for (idx, item) in items.iter().enumerate() {
                    let elem = vm_value_to_raw(item);
                    roots.set(1, elem);
                    ASET(vec, idx as isize, elem);
                    roots.set(1, lisp_h_Qnil());
                }
                vec
            }
            VmValue::Map(entries) => {
                let mut ht_args = [QCtest(), Qequal()];
                let ht = Fmake_hash_table(2, ht_args.as_mut_ptr());
                let mut roots = ScopedLispRoot::new(&[ht, lisp_h_Qnil(), lisp_h_Qnil()]);
                for (k, v) in entries {
                    let k_obj = build_utf8_str(k);
                    roots.set(1, k_obj);
                    let v_obj = vm_value_to_raw(v);
                    roots.set(2, v_obj);
                    Fputhash(k_obj, v_obj, ht);
                    roots.set(1, lisp_h_Qnil());
                    roots.set(2, lisp_h_Qnil());
                }
                ht
            }
        }
    }
}

struct UnbindGuard {
    target: isize,
}

impl Drop for UnbindGuard {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: `target` was captured from `current_specpdl_index()` on this VM thread; only
        // binds `command-line-args-left`/`argv` list values so `unbind_to_safe` restores bindings
        // without running Lisp callbacks. During `HOST_EXIT_UNWINDING`, `restore_vm_state_to_host_handler`
        // unwinds `specpdl` top-to-bottom per handler so nested handler `pdlcount`s stay ordered.
        unsafe {
            if !is_host_exit_unwinding() && current_specpdl_index() > self.target {
                unbind_to_safe(wrap_specpdl_ref(self.target), lisp_h_Qnil());
            }
        }
    }
}

/// Opaque, non-`Copy`, `!Send + !Sync` owner of the single headless `femacs-vm` instance (`C1`).
///
/// Confines the raw `GcContext` lease inside the VM runtime boundary so external crates cannot
/// bypass condition protection or leak unowned `Rooted` handles past VM shutdown. Every entry
/// runs under stack anchoring + `CONDITION_CASE(t)` protection and returns host-owned values.
pub struct HeadlessVm {
    ctx: Option<GcContext>,
    _not_send_sync: PhantomData<*mut ()>,
}

impl HeadlessVm {
    /// Initialize the process-wide headless VM once and return an owning `HeadlessVm` handle.
    pub fn init(config: &HostInitConfig) -> Result<Self, VmHostError> {
        let ctx = init_headless_host(config)?;
        Ok(Self {
            ctx: Some(ctx),
            _not_send_sync: PhantomData,
        })
    }

    #[inline]
    fn ctx_mut(&mut self) -> &mut GcContext {
        self.ctx
            .as_mut()
            .expect("HeadlessVm context present until Drop")
    }

    /// Read and sequentially evaluate every top-level Lisp form in `source`, returning the last
    /// value converted into a host-owned `VmValue`.
    pub fn read_eval_value(&mut self, source: &str) -> Result<VmValue, VmHostError> {
        with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Runs inside `with_host_condition_catch` on the VM-locked active thread;
            // `eval_forms_raw` and `raw_to_vm_value` keep all temporary values rooted.
            unsafe {
                let last_val = eval_forms_raw(source);
                raw_to_vm_value(last_val, 0)
            }
        })
    }

    /// Call Lisp function `func_name` with typed host `args`, returning its result as `VmValue`.
    pub fn call_value(&mut self, func_name: &str, args: &[VmValue]) -> Result<VmValue, VmHostError> {
        let func_name = func_name.to_string();
        with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Runs inside `with_host_condition_catch`; `RootedVec` keeps function symbol
            // and converted arguments rooted across `Ffuncall`, and `raw_to_vm_value` roots output.
            unsafe {
                let fn_str = build_utf8_str(&func_name);
                let _fn_str_root = ScopedLispRoot::new(&[fn_str]);
                let fn_sym = Fintern(fn_str, lisp_h_Qnil());
                let mut call_args = crate::roots::RootedVec::with_capacity(1 + args.len());
                call_args.push(fn_sym);
                for arg in args {
                    let obj = vm_value_to_raw(arg);
                    call_args.push(obj);
                }
                let res = Ffuncall(call_args.len() as isize, call_args.as_mut_slice().as_mut_ptr());
                raw_to_vm_value(res, 0)
            }
        })
    }

    /// Read and sequentially evaluate every top-level Lisp form in `source`, returning the last
    /// result printed with `prin1-to-string`.
    pub fn read_eval_to_string(&mut self, source: &str) -> Result<String, VmHostError> {
        with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Runs inside `with_host_condition_catch` on the VM-locked active thread;
            // `eval_forms_raw` keeps source and intermediate forms rooted, and `raw_prin1_to_owned`
            // roots `last_val` across printing.
            unsafe {
                let last_val = eval_forms_raw(source);
                raw_prin1_to_owned(last_val)
            }
        })
    }

    /// Read and sequentially evaluate all top-level Lisp forms in `source` without printing the value.
    pub fn eval_silent(&mut self, source: &str) -> Result<(), VmHostError> {
        with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Runs inside `with_host_condition_catch` with all forms rooted in `eval_forms_raw`.
            unsafe {
                let _ = eval_forms_raw(source);
            }
        })
    }

    /// Evaluate all top-level Lisp forms in `source` with `command-line-args-left` and `argv`
    /// dynamically bound to `args_left`, returning optional `prin1` output and remaining args.
    pub fn eval_with_args(
        &mut self,
        source: &str,
        print_result: bool,
        args_left: &[OsString],
    ) -> Result<(Option<String>, Vec<OsString>), VmHostError> {
        let argv_strs: Vec<String> = args_left
            .iter()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();
        with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Runs inside `with_host_condition_catch`; `UnbindGuard` restores `argv` and
            // `command-line-args-left` after reading remaining items, and `ScopedLispRoot` roots `val`.
            unsafe {
                let count = current_specpdl_index();
                let unbind = UnbindGuard { target: count };
                bind_argv_list_raw(&argv_strs);
                let last_val = eval_forms_raw(source);
                let _val_root = ScopedLispRoot::new(&[last_val]);
                let printed = if print_result {
                    Some(raw_prin1_to_owned(last_val))
                } else {
                    None
                };
                let remaining = read_command_line_args_left_raw();
                drop(unbind);
                (printed, remaining)
            }
        })
    }

    /// Prepend a directory (and its `subdirs.el` if present) to Lisp `load-path`.
    pub fn prepend_load_path(&mut self, dir: &Path) -> Result<(), VmHostError> {
        let dir = dir.to_path_buf();
        with_host_condition_catch(self.ctx_mut(), || {
            add_load_path_and_subdirs_raw(&dir);
        })
    }

    /// Resolve CLI `-l` path relative to `default-directory` when it names a regular file
    /// (matching reference `command-line-1`), falling back to `file_lisp` for `load-path` lookup.
    ///
    /// # Safety
    /// Caller holds the active VM lock inside `with_host_condition_catch` and roots the returned
    /// `Lisp_Object` across `Fload`.
    unsafe fn load_target_raw(file_lisp: Lisp_Object) -> Lisp_Object {
        // SAFETY: Roots `file_lisp` and `candidate` in `ScopedLispRoot` across expansion,
        // `file-truename`, and `Ffile_regular_p`.
        unsafe {
            let mut roots = ScopedLispRoot::new(&[file_lisp, lisp_h_Qnil()]);
            let mut candidate = Fexpand_file_name(file_lisp, lisp_h_Qnil());
            roots.set(1, candidate);
            let truename_sym = intern_c_string(c"file-truename".as_ptr());
            if !NILP(Ffboundp(truename_sym)) {
                let mut call = [truename_sym, candidate];
                candidate = Ffuncall(2, call.as_mut_ptr());
                roots.set(1, candidate);
            }
            if !NILP(Ffile_regular_p(candidate)) {
                candidate
            } else {
                file_lisp
            }
        }
    }

    /// Load a Lisp library or file using standard `-l` lookup semantics (`nosuffix = nil`).
    pub fn load_library(&mut self, file: &str) -> Result<(), VmHostError> {
        let file = file.to_string();
        with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Runs inside `with_host_condition_catch`; roots `file_lisp` and resolved
            // `target` across `Fload`.
            unsafe {
                let file_lisp = build_utf8_str(&file);
                let mut roots = ScopedLispRoot::new(&[file_lisp, lisp_h_Qnil()]);
                let target = Self::load_target_raw(file_lisp);
                roots.set(1, target);
                Fload(
                    target,
                    lisp_h_Qnil(),
                    Qt(),
                    lisp_h_Qnil(),
                    lisp_h_Qnil(),
                );
            }
        })
    }

    /// Load a Lisp library or file with `command-line-args-left` and `argv` bound to `args_left`,
    /// returning whatever items remain in `command-line-args-left` after loading completes.
    pub fn load_library_with_args(
        &mut self,
        file: &str,
        args_left: &[OsString],
    ) -> Result<Vec<OsString>, VmHostError> {
        let file = file.to_string();
        let argv_strs: Vec<String> = args_left
            .iter()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();
        with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Runs inside `with_host_condition_catch`; `UnbindGuard` restores bindings after
            // `read_command_line_args_left_raw` reads remaining items, and `file_lisp`/`target`
            // remain rooted across `Fload`.
            unsafe {
                let count = current_specpdl_index();
                let unbind = UnbindGuard { target: count };
                bind_argv_list_raw(&argv_strs);
                let file_lisp = build_utf8_str(&file);
                let mut roots = ScopedLispRoot::new(&[file_lisp, lisp_h_Qnil()]);
                let target = Self::load_target_raw(file_lisp);
                roots.set(1, target);
                Fload(
                    target,
                    lisp_h_Qnil(),
                    Qt(),
                    lisp_h_Qnil(),
                    lisp_h_Qnil(),
                );
                let remaining = read_command_line_args_left_raw();
                drop(unbind);
                remaining
            }
        })
    }

    /// Call zero-arg batch function `func_name` with `command-line-args-left` and `argv` dynamically
    /// bound to `argv`.
    pub fn funcall_with_args(&mut self, func_name: &str, argv: &[OsString]) -> Result<(), VmHostError> {
        let _ = self.funcall_with_args_left(func_name, argv)?;
        Ok(())
    }

    /// Call zero-arg batch function `func_name` with `command-line-args-left` and `argv` dynamically
    /// bound to `args_left`, returning any remaining items in `command-line-args-left`.
    pub fn funcall_with_args_left(
        &mut self,
        func_name: &str,
        args_left: &[OsString],
    ) -> Result<Vec<OsString>, VmHostError> {
        let func_c = CString::new(func_name).map_err(|_| VmHostError::LispCondition {
            symbol: "invalid-function".to_string(),
            message: format!("function name contains NUL byte: {func_name}"),
            data: "nil".to_string(),
        })?;
        let argv_strs: Vec<String> = args_left
            .iter()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();
        with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Runs inside `with_host_condition_catch`; `UnbindGuard` restores `argv` and
            // `command-line-args-left` after `read_command_line_args_left_raw` reads remaining args.
            unsafe {
                let count = current_specpdl_index();
                let unbind = UnbindGuard { target: count };
                bind_argv_list_raw(&argv_strs);
                let mut sym = intern_c_string(func_c.as_ptr());
                Ffuncall(1, &raw mut sym);
                let remaining = read_command_line_args_left_raw();
                drop(unbind);
                remaining
            }
        })
    }

    /// Invoke two-argument Lisp function `(fn_name arg1 arg2)` directly without reader quoting
    /// and return a host-owned UTF-8 `String` from its Lisp string result (or `prin1` fallback).
    pub fn funcall_two_strings_to_string(
        &mut self,
        fn_name: &str,
        arg1: &str,
        arg2: &str,
    ) -> Result<String, VmHostError> {
        let func_c = CString::new(fn_name).map_err(|_| VmHostError::LispCondition {
            symbol: "invalid-function".to_string(),
            message: format!("function name contains NUL byte: {fn_name}"),
            data: "nil".to_string(),
        })?;
        with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Runs inside `with_host_condition_catch`; roots `s1`, `s2`, and `res` across
            // `Ffuncall` and string conversion on the VM-lock-owning thread.
            unsafe {
                let s1 = build_utf8_str(arg1);
                let mut roots = ScopedLispRoot::new(&[s1, lisp_h_Qnil(), lisp_h_Qnil()]);
                let s2 = build_utf8_str(arg2);
                roots.set(1, s2);
                let sym = intern_c_string(func_c.as_ptr());
                let mut args = [sym, s1, s2];
                let res = Ffuncall(3, args.as_mut_ptr());
                roots.set(2, res);
                if STRINGP(res) {
                    raw_string_to_owned(res).unwrap_or_default()
                } else {
                    raw_prin1_to_owned(res)
                }
            }
        })
    }

    /// Load a Lisp script file with `command-line-args-left` (`argv`) dynamically bound to `argv`.
    pub fn load_script_file(&mut self, path: &Path, argv: &[OsString]) -> Result<(), VmHostError> {
        let abs_path = resolve_abs_path(path)?;
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

        with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Runs inside `with_host_condition_catch`; roots `file_lisp` and `arg_list` across
            // script loading and restores `command-line-args-left` via `UnbindGuard` on every exit.
            unsafe {
                let count = current_specpdl_index();
                let _unbind = UnbindGuard { target: count };
                bind_argv_list_raw(&argv_strs);

                let file_lisp = build_utf8_str(&abs_s);
                let mut root = ScopedLispRoot::new(&[file_lisp, lisp_h_Qnil()]);
                let script_loader = intern_c_string(c"command-line--load-script".as_ptr());
                if !NILP(Ffboundp(script_loader)) {
                    let form = Fcons(script_loader, Fcons(file_lisp, lisp_h_Qnil()));
                    root.set(1, form);
                    Feval(form, Qt());
                } else {
                    Fload(file_lisp, lisp_h_Qnil(), Qt(), Qt(), lisp_h_Qnil());
                }
            }
        })
    }

    /// Write a relocatable portable dump bound to the current executable's fingerprint at `out_path`.
    pub fn dump_portable(&mut self, out_path: &Path) -> Result<(), VmHostError> {
        let abs_path = resolve_abs_path(out_path)?;
        let abs_s = abs_path.to_string_lossy().into_owned();
        with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Clears `Vpurify_flag`, restores `Vlexical_binding` to its default top-level
            // value (matching `loadup.el`), and invokes `Fdump_emacs_portable` with rooted path.
            unsafe {
                *Vpurify_flag.as_mut_ptr() = lisp_h_Qnil();
                *Vlexical_binding.as_mut_ptr() = Fdefault_toplevel_value(Qlexical_binding());
                let file_lisp = build_utf8_str(&abs_s);
                let _root = ScopedLispRoot::new(&[file_lisp]);
                Fdump_emacs_portable(file_lisp, lisp_h_Qnil());
            }
        })
    }

    /// Trigger VM garbage collection under host stack anchoring and condition protection.
    pub fn garbage_collect(&mut self) -> Result<(), VmHostError> {
        with_host_condition_catch(self.ctx_mut(), || {
            Fgarbage_collect();
        })
    }

    /// Exercise the `.eln` C-ABI helper pending-host-exit boundary: verifies pending host exit
    /// cannot be consumed by an `.eln` handler and unwinds through `with_host_condition_catch` as
    /// `Err(VmHostError::ExitRequest(code))` with thread pending state cleared afterward.
    #[doc(hidden)]
    pub fn verify_comp_native_host_exit_state(&mut self, code: i32) -> Result<(), VmHostError> {
        let res = with_host_condition_catch(self.ctx_mut(), || {
            // SAFETY: Runs inside `with_host_condition_catch` on the active VM-lock-owning thread.
            unsafe {
                let h = push_handler_nosignal(Qt(), CONDITION_CASE);
                let mut pop_guard = HandlerPopGuard {
                    expected_handler: h,
                    active: !h.is_null(),
                };
                crate::nonlocal::set_comp_native_host_exit(code);
                assert_eq!(
                    crate::nonlocal::comp_native_exit_pending_for_current_thread(),
                    1
                );
                assert_eq!(
                    crate::nonlocal::consume_comp_native_exit_for_handler(core::ptr::null_mut()),
                    0
                );
                assert_eq!(crate::nonlocal::consume_comp_native_exit_for_handler(h), 0);
                assert_eq!(
                    crate::nonlocal::comp_native_exit_pending_for_current_thread(),
                    1
                );
                pop_guard.pop_now();
                crate::nonlocal::check_and_propagate_comp_native_exit();
                unreachable!("comp native host exit must unwind cleanly");
            }
        });
        assert_eq!(
            crate::nonlocal::comp_native_exit_pending_for_current_thread(),
            0
        );
        res
    }

    /// Inspect observable headless frame/terminal/GC/pdumper invariants inside the active VM.
    pub fn inspect_environment(&mut self) -> Result<HeadlessDiagnostics, VmHostError> {
        let ctx = self.ctx_mut();
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

    /// Run all `crate::safe_domain` checks under anchored stack bounds and condition protection,
    /// extracting an owned summary string before the `Rooted` result leaves scope.
    pub fn exercise_safe_domains(&mut self, temp_file_path: &Path) -> Result<String, VmHostError> {
        let path_s = temp_file_path.to_string_lossy().into_owned();
        with_host_condition_catch_ctx(self.ctx_mut(), |ctx| {
            let res = crate::safe_domain::exercise_all_safe_domains(ctx, &path_s)?;
            Ok(crate::vm_primitives::lisp_string_to_owned(ctx, res.handle()).unwrap_or_default())
        })
    }

    /// Evaluate `source` through `crate::safe_domain::read_eval_and_format_utf8` under anchored
    /// stack bounds and condition protection, returning only host-owned output or `VmHostError`.
    pub fn safe_domain_read_eval(&mut self, source: &str) -> Result<String, VmHostError> {
        with_host_condition_catch_ctx(self.ctx_mut(), |ctx| {
            crate::safe_domain::read_eval_and_format_utf8(ctx, source)
        })
    }
}

impl Drop for HeadlessVm {
    fn drop(&mut self) {
        if let Some(ctx) = self.ctx.take() {
            shutdown_headless_host(ctx);
        }
    }
}

/// Validate and adopt an inherited `AF_UNIX` `SOCK_STREAM` socket descriptor for internal RPC,
/// moving stdio descriptors (`<= 2`) onto a dedicated `FD_CLOEXEC` descriptor `>= 3` and
/// replacing the stdio slot with `/dev/null` so Lisp code or child processes never see the pipe.
pub fn adopt_inherited_rpc_socket(explicit_fd: Option<i32>) -> io::Result<UnixStream> {
    let raw_fd: c_int = explicit_fd.unwrap_or(0);
    if raw_fd < 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid RPC socket descriptor {raw_fd}"),
        ));
    }

    let mut sock_type: c_int = 0;
    let mut sock_type_len = core::mem::size_of::<c_int>() as u32;
    // SAFETY: `getsockopt` queries `SO_TYPE` into initialized `sock_type` for `raw_fd` before
    // constructing an owning descriptor; non-sockets return `ENOTSOCK` (`rc != 0`).
    let rc_type = unsafe {
        getsockopt(
            raw_fd,
            SYS_SOL_SOCKET,
            SYS_SO_TYPE,
            (&raw mut sock_type).cast::<c_void>(),
            &raw mut sock_type_len,
        )
    };
    if rc_type != 0 {
        return Err(io::Error::last_os_error());
    }
    if sock_type != SYS_SOCK_STREAM {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("RPC descriptor {raw_fd} is not a SOCK_STREAM socket (type={sock_type})"),
        ));
    }

    let mut addr = RawSockaddrStorageHeader::zeroed();
    let mut addr_len = core::mem::size_of::<RawSockaddrStorageHeader>() as u32;
    // SAFETY: `getsockname` inspects the socket family into an initialized stack buffer.
    let rc_name = unsafe {
        getsockname(
            raw_fd,
            (&raw mut addr).cast::<c_void>(),
            &raw mut addr_len,
        )
    };
    if rc_name != 0 {
        return Err(io::Error::last_os_error());
    }
    if addr_len < 2 || addr.family() != SYS_AF_UNIX {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("RPC descriptor {raw_fd} is not an AF_UNIX socket"),
        ));
    }

    let adopted_fd: c_int = if raw_fd <= 2 {
        // SAFETY: Duplicates the validated inherited stdio socket to a dedicated `>= 3` descriptor
        // with `FD_CLOEXEC`, then redirects `raw_fd` cleanly onto `/dev/null` (`O_RDWR`).
        unsafe {
            let duped = fcntl(raw_fd, F_DUPFD_CLOEXEC, 3 as c_int);
            if duped < 0 {
                return Err(io::Error::last_os_error());
            }
            let devnull = open(c"/dev/null".as_ptr(), O_RDWR, 0 as c_int);
            if devnull >= 0 {
                let _ = dup2(devnull, raw_fd);
                if devnull != raw_fd {
                    let _ = close(devnull);
                }
            }
            duped
        }
    } else {
        // SAFETY: Ensures `FD_CLOEXEC` is set on validated inherited descriptor `raw_fd >= 3`.
        unsafe {
            let cur_flags = fcntl(raw_fd, F_GETFD, 0 as c_int);
            if cur_flags < 0 {
                return Err(io::Error::last_os_error());
            }
            if (cur_flags & FD_CLOEXEC) == 0 && fcntl(raw_fd, F_SETFD, cur_flags | FD_CLOEXEC) < 0 {
                return Err(io::Error::last_os_error());
            }
        }
        raw_fd
    };

    // SAFETY: `adopted_fd` is a validated open `AF_UNIX` `SOCK_STREAM` descriptor with `FD_CLOEXEC`
    // and unique ownership inside this process after stdio isolation above.
    let owned = unsafe { OwnedFd::from_raw_fd(adopted_fd) };
    Ok(UnixStream::from(owned))
}

/// Inspect whether `stream`'s underlying descriptor has `FD_CLOEXEC` enabled.
pub fn stream_has_cloexec(stream: &UnixStream) -> io::Result<bool> {
    let fd = stream.as_raw_fd();
    // SAFETY: `stream` owns an active open file descriptor; `F_GETFD` is read-only.
    let flags = unsafe { fcntl(fd, F_GETFD, 0 as c_int) };
    if flags < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok((flags & FD_CLOEXEC) != 0)
    }
}
