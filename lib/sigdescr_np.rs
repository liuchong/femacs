// femacs: port of lib/sigdescr_np.c @ fd7afced403.
//! Safe signal-description table lookup (`lib/sigdescr_np.c`) backed by `femacs_sys::signal_names`.
#![deny(static_mut_refs, unsafe_op_in_unsafe_fn)]

use crate::common::*;

static SIGDESCR_TABLE: [(c_int, &'static CStr); 31] = [
    (SIGABRT, c"Aborted"),
    (SIGFPE, c"Arithmetic exception"),
    (SIGILL, c"Illegal instruction"),
    (SIGINT, c"Interrupt"),
    (SIGSEGV, c"Segmentation fault"),
    (SIGTERM, c"Terminated"),
    (SIGALRM, c"Alarm clock"),
    (SIGBUS, c"Bus error"),
    (SIGCHLD, c"Child stopped or exited"),
    (SIGCONT, c"Continued"),
    (SIGHUP, c"Hangup"),
    (SIGKILL, c"Killed"),
    (SIGPIPE, c"Broken pipe"),
    (SIGQUIT, c"Quit"),
    (SIGSTOP, c"Stopped (signal)"),
    (SIGTSTP, c"Stopped"),
    (SIGTTIN, c"Stopped (tty input)"),
    (SIGTTOU, c"Stopped (tty output)"),
    (SIGUSR1, c"User defined signal 1"),
    (SIGUSR2, c"User defined signal 2"),
    (SIGPROF, c"Profiling timer expired"),
    (SIGSYS, c"Bad system call"),
    (SIGTRAP, c"Trace/breakpoint trap"),
    (SIGURG, c"Urgent I/O condition"),
    (SIGVTALRM, c"Virtual timer expired"),
    (SIGXCPU, c"CPU time limit exceeded"),
    (SIGXFSZ, c"File size limit exceeded"),
    (SIGEMT, c"Instruction emulation needed"),
    (SIGINFO, c"Information request"),
    (SIGIO, c"I/O possible"),
    (SIGWINCH, c"Window size changed"),
];

#[unsafe(no_mangle)]
pub extern "C" fn sigdescr_np(sig: c_int) -> *const c_char {
    match ::femacs_sys::signal_names::sigdescr_lookup(sig, &SIGDESCR_TABLE) {
        Some(desc) => desc.as_ptr(),
        None => core::ptr::null(),
    }
}
