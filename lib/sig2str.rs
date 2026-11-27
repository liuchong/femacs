// femacs: port of lib/sig2str.c @ fd7afced403.
//! Thin C-ABI boundary wrapper delegating signal name/number conversion to `femacs_sys::signal_names`.
#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]

use crate::common::*;

pub const SIGRTMIN: c_int = 0;
pub const SIGRTMAX: c_int = SIGRTMIN - 1;

static SIGNAL_TABLE: [(c_int, &[u8]); 33] = [
    (SIGHUP, b"HUP"),
    (SIGINT, b"INT"),
    (SIGQUIT, b"QUIT"),
    (SIGILL, b"ILL"),
    (SIGTRAP, b"TRAP"),
    (SIGABRT, b"ABRT"),
    (SIGFPE, b"FPE"),
    (SIGKILL, b"KILL"),
    (SIGSEGV, b"SEGV"),
    (SIGBUS, b"BUS"),
    (SIGPIPE, b"PIPE"),
    (SIGALRM, b"ALRM"),
    (SIGTERM, b"TERM"),
    (SIGUSR1, b"USR1"),
    (SIGUSR2, b"USR2"),
    (SIGCHLD, b"CHLD"),
    (SIGURG, b"URG"),
    (SIGSTOP, b"STOP"),
    (SIGTSTP, b"TSTP"),
    (SIGCONT, b"CONT"),
    (SIGTTIN, b"TTIN"),
    (SIGTTOU, b"TTOU"),
    (SIGSYS, b"SYS"),
    (SIGVTALRM, b"VTALRM"),
    (SIGPROF, b"PROF"),
    (SIGXCPU, b"XCPU"),
    (SIGXFSZ, b"XFSZ"),
    (SIGIOT, b"IOT"),
    (SIGEMT, b"EMT"),
    (SIGWINCH, b"WINCH"),
    (SIGINFO, b"INFO"),
    (SIGIO, b"IO"),
    (0, b"EXIT"),
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn str2sig(signame: *const c_char, signum: *mut c_int) -> c_int {
    if signame.is_null() || signum.is_null() {
        return -1;
    }
    // SAFETY: `signame` is a NUL-terminated C string and `signum` points to a writable `c_int`.
    let bytes = unsafe { CStr::from_ptr(signame).to_bytes() };
    let parsed = ::femacs_sys::signal_names::str2signum_bytes(
        bytes,
        &SIGNAL_TABLE,
        SIGNUM_BOUND,
        SIGRTMIN,
        SIGRTMAX,
    );
    // SAFETY: `signum` is non-null and writable per `str2sig` C contract.
    unsafe {
        *signum = parsed;
    }
    if parsed < 0 { -1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sig2str(signum: c_int, signame: *mut c_char) -> c_int {
    if signame.is_null() {
        return -1;
    }
    let mut buf = [0u8; 16];
    if !::femacs_sys::signal_names::sig2str_bytes(
        signum,
        &SIGNAL_TABLE,
        SIGRTMIN,
        SIGRTMAX,
        &mut buf,
    ) {
        return -1;
    }
    let len = buf.iter().position(|&b| b == 0).unwrap_or(15) + 1;
    // SAFETY: Caller supplies a buffer of at least `SIG2STR_MAX` (16) bytes (`len <= 16`).
    unsafe {
        core::ptr::copy_nonoverlapping(buf.as_ptr(), signame.cast::<u8>(), len);
    }
    0
}
