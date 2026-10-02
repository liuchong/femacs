// femacs: the parts of the system <signal.h> the port needs (PORTING.md 0.1, the errno.rs
// shape).  The values were measured from the frozen target's clang
// (`clang -dM -E` on a file that includes <signal.h>); `c_int` comes from the including unit.
// The signals this target does not define (SIGPWR, SIGSTKFLT and the other platform-specific
// ones sigdescr_np.c guards with `#if defined`) are absent on purpose: docs/ports/sigdescr_np.md
// records them.
pub const SIGABRT: c_int = 6;
pub const SIGALRM: c_int = 14;
pub const SIGBUS: c_int = 10;
pub const SIGCHLD: c_int = 20;
pub const SIGCONT: c_int = 19;
pub const SIGEMT: c_int = 7;
pub const SIGFPE: c_int = 8;
pub const SIGHUP: c_int = 1;
pub const SIGILL: c_int = 4;
pub const SIGINFO: c_int = 29;
pub const SIGINT: c_int = 2;
pub const SIGIO: c_int = 23;
pub const SIGKILL: c_int = 9;
pub const SIGPIPE: c_int = 13;
pub const SIGPROF: c_int = 27;
pub const SIGQUIT: c_int = 3;
pub const SIGSEGV: c_int = 11;
pub const SIGSTOP: c_int = 17;
pub const SIGSYS: c_int = 12;
pub const SIGTERM: c_int = 15;
pub const SIGTRAP: c_int = 5;
pub const SIGTSTP: c_int = 18;
pub const SIGTTIN: c_int = 21;
pub const SIGTTOU: c_int = 22;
pub const SIGURG: c_int = 16;
pub const SIGUSR1: c_int = 30;
pub const SIGUSR2: c_int = 31;
pub const SIGVTALRM: c_int = 26;
pub const SIGWINCH: c_int = 28;
pub const SIGXCPU: c_int = 24;
pub const SIGXFSZ: c_int = 25;
