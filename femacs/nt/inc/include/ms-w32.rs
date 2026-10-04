// femacs: mirror of nt/inc/ms-w32.h @ fd7afced403.
//
// The Windows port's system description header: `src/conf_post.h` includes it, and it is where
// NT's substitutions live - the `sys_*` renames for the calls Emacs emulates, the CRT's own
// names for the rest, the signal numbers and errno values MinGW's headers lack, the
// `malloc`/`free`/`realloc` indirection through `the_*_fn`, and the declarations of every
// emulated entry point.
//
// Conditions: this file is not on `docs/baseline/config/cond-eval-verified.txt`, so the
// extractor resolves no `#if` in it - every arm (`__GNUC__`, `MINGW_W64`, `emacs`, `__MINGW32__`,
// `_WIN64`, `WINDOWSNT`, …) is live text and the mirror carries all of them.  Nothing in the
// frozen configuration includes it either: no `nt/` translation unit is compiled
// (docs/baseline/frozen-build-sources.txt's batch-F section) and no `src/` unit includes a
// `nt/inc/` header, so `xtask layout-check` never compiles this mirror.  In a MinGW build
// `src/conf_post.h`, `src/w32.c`, `src/w32notify.c` and `src/w32proc.c` are among the units
// whose `#include <ms-w32.h>` resolves here.
//
// Three conventions this mirror records:
//  * C's object-like renames (`#define close sys_close`) also rename bare identifier uses,
//    including taking a function's address.  A Rust `macro_rules!` covers only the call form,
//    so a ported unit that passes one of these names as a value must name the target itself
//    (`_VARARGS_`/`_CALLBACK_` are the other half of the same problem: `__cdecl` has no Rust
//    spelling, and they are recorded in `docs/ports/excluded-macros.txt`).
//  * the system names - `sigset_t`, `mode_t`, `pid_t`, `time_t`, `struct tm`, `intptr_t`,
//    `size_t`, `_MAX_PATH`, and the CRT's `_ptr`/`_base` fields - have no mirror in the port:
//    they are the MinGW runtime's own types and constants, so they stay spelled the way C
//    spells them and are named from the including unit, as `nt/inc/sys/include/acl.rs` does for
//    `ssize_t`.
//  * a value C spells with a cast or an alias keeps the alias in the constant's initialiser, so
//    the C token is still visible (`MAXPATHLEN`, `MAX_UTF8_PATH`, `ENOTSUP`).

include!("mingw_time.rs");   // C 27: `<mingw_time.h>`

/// C 23-25: `#ifndef WINDOWSNT` / `#define WINDOWSNT` - the switch the rest of the tree tests.
pub const WINDOWSNT: bool = true;
/// C 34-36: `#ifdef __MINGW64_VERSION_MAJOR` / `#define MINGW_W64`.
pub const MINGW_W64: bool = true;
/// C 39-40: `#undef`/`#define __POSIX_2008_DEPRECATED` (avoid the deprecated-`gettimeofday`
/// warnings).
pub const __POSIX_2008_DEPRECATED: bool = true;

/// C 44-46: `#define _WIN32_WINNT_WIN2K 0x0500`
pub const _WIN32_WINNT_WIN2K: c_int = 0x0500;
/// C 47-49: `#define _WIN32_WINNT_WINXP 0x0501`
pub const _WIN32_WINNT_WINXP: c_int = 0x0501;
/// C 50-52: `#define _WIN32_WINNT_WS03 0x0502`
pub const _WIN32_WINNT_WS03: c_int = 0x0502;
/// C 53-55: `#define _WIN32_WINNT_VISTA 0x0600`
pub const _WIN32_WINNT_VISTA: c_int = 0x0600;
/// C 56-58: `#define _WIN32_WINNT_WIN7 0x0601`
pub const _WIN32_WINNT_WIN7: c_int = 0x0601;
/// C 59-61: `#define _WIN32_WINNT_WIN8 0x0602`
pub const _WIN32_WINNT_WIN8: c_int = 0x0602;
/// C 62-64: `#define _WIN32_WINNT_WINBLUE 0x0603`
pub const _WIN32_WINNT_WINBLUE: c_int = 0x0603;
/// C 65-67: `#define _WIN32_WINNT_WIN10 0x0A00`
pub const _WIN32_WINNT_WIN10: c_int = 0x0A00;

/// C 74-77: `#define PENDING_OUTPUT_COUNT(FILE) …` - two arms (`__GNU_LIBRARY__` and the CRT);
/// C keeps the second, and both are the CRT's `FILE` internals, which the port has no mirror
/// for.
macro_rules! PENDING_OUTPUT_COUNT {
  ($FILE:expr) => { (*$FILE)._ptr.offset_from ((*$FILE)._base) };
}

// C 81 and 86: `#define _VARARGS_ __cdecl` and `#define _CALLBACK_ __cdecl` - the calling
// convention.  `__cdecl` has no Rust spelling (the port writes `extern "C"`, and the two names
// are attribute prefixes on a declaration or a function-pointer type, which a `macro_rules!`
// cannot produce), so both are recorded in `docs/ports/excluded-macros.txt` instead of being
// carried as items (`alignas`'s shape).

/// C 90: `#define HAVE_TIMEVAL 1`
pub const HAVE_TIMEVAL: c_int = 1;
/// C 94: `#define BROKEN_DATAGRAM_SOCKETS 1`  (`/* our select emulation does read-ahead */`)
pub const BROKEN_DATAGRAM_SOCKETS: c_int = 1;
/// C 96: `#define MAIL_USE_SYSTEM_LOCK 1`
pub const MAIL_USE_SYSTEM_LOCK: c_int = 1;
/// C 100: `#define HAVE_ATTRIBUTE_ALIGNED 1`  (the `__GNUC__` arm)
pub const HAVE_ATTRIBUTE_ALIGNED: c_int = 1;
/// C 105: `#define HAVE_C99_STRTOLD 1`  (the `__GNUC__` arm)
pub const HAVE_C99_STRTOLD: c_int = 1;
/// C 110: `#define HAVE___BUILTIN_UNWIND_INIT 1`
pub const HAVE___BUILTIN_UNWIND_INIT: c_int = 1;

/// C 143: `#define _WIN32_WINNT 0x0400` (the `#ifndef MINGW_W64` arm)
pub const _WIN32_WINNT: c_int = 0x0400;
/// C 147: `#define WIN32_LEAN_AND_MEAN`
pub const WIN32_LEAN_AND_MEAN: bool = true;

/// C 152: `#define MAXPATHLEN _MAX_PATH`
pub const MAXPATHLEN: c_int = _MAX_PATH;
/// C 156: `#define MAX_UTF8_PATH (MAXPATHLEN * 4)`
pub const MAX_UTF8_PATH: c_int = MAXPATHLEN * 4;

/// C 160-161: `#define HAVE_WINDOW_SYSTEM 1` / `#define POLL_FOR_INPUT 1` (the `HAVE_NTGUI`
/// arm).
pub const HAVE_WINDOW_SYSTEM: c_int = 1;
/// C 161: `#define POLL_FOR_INPUT 1`
pub const POLL_FOR_INPUT: c_int = 1;

/// C 524: `#define NULL ((void *)0)` (the `__cplusplus` arm writes a bare `0`; C keeps the
/// second).
pub const NULL: *mut c_void = core::ptr::null_mut();

/// C 323-330: the signal numbers `nt` adds to MinGW's `<signal.h>`.
pub const SIGHUP: c_int = 1;
pub const SIGQUIT: c_int = 3;
pub const SIGTRAP: c_int = 5;
pub const SIGKILL: c_int = 9;
pub const SIGPIPE: c_int = 13;
pub const SIGALRM: c_int = 14;
pub const SIGCHLD: c_int = 18;
pub const SIGPROF: c_int = 19;
/// C 333: `#define NSIG 23`
pub const NSIG: c_int = 23;
/// C 337: `#define ENOTSUP ENOSYS` - the port's `<errno.h>` mirror carries `ENOSYS`.
pub const ENOTSUP: c_int = ENOSYS;
/// C 342: `#define EOPNOTSUPP 130`
pub const EOPNOTSUPP: c_int = 130;

/// C 359-362: `SA_RESTART` for `sigaction`'s `sa_flags`, and the three `sigprocmask` selectors.
pub const SA_RESTART: c_int = 0;
/// C 360: `#define SIG_BLOCK 1`
pub const SIG_BLOCK: c_int = 1;
/// C 361: `#define SIG_SETMASK 2`
pub const SIG_SETMASK: c_int = 2;
/// C 362: `#define SIG_UNBLOCK 3`
pub const SIG_UNBLOCK: c_int = 3;

/// C 347-350: `#ifdef MINGW_W64 / #ifndef _POSIX` typedefs `sigset_t` as the runtime's
/// `_sigset_t`; the name is left to the including unit here (see the head note).
/// C 352: `typedef void (_CALLBACK_ *signal_handler) (int);`
pub type signal_handler = Option<unsafe extern "C" fn (c_int)>;

/// C 355-358: `struct sigaction` - the runtime's own layout, three members in C's order.
#[repr(C)]
pub struct sigaction
{
  pub sa_flags: c_int,
  pub sa_handler: Option<unsafe extern "C" fn (c_int)>,
  pub sa_mask: sigset_t,
}

/// C 401: `#define _WINSOCKAPI_ 1`
pub const _WINSOCKAPI_: c_int = 1;
/// C 406: `# define _WINSOCK_H` (the arm for runtimes older than MinGW 5.x)
pub const _WINSOCK_H: bool = true;

/// C 422-424: `typedef void * (* malloc_fn)(size_t);` and its two siblings.
pub type malloc_fn = Option<unsafe extern "C" fn (size_t) -> *mut c_void>;
/// C 423: `typedef void * (* realloc_fn)(void *, size_t);`
pub type realloc_fn = Option<unsafe extern "C" fn (*mut c_void, size_t) -> *mut c_void>;
/// C 424: `typedef void (* free_fn)(void *);`
pub type free_fn = Option<unsafe extern "C" fn (*mut c_void)>;

/// C 461-462: `#define sys_nerr _sys_nerr`
macro_rules! sys_nerr { () => { _sys_nerr }; }

/// C 483-484: `#define HAVE_LONG_LONG_INT 1`
pub const HAVE_LONG_LONG_INT: c_int = 1;
/// C 486-489: `#define HAVE_UNSIGNED_LONG_LONG_INT 1`
pub const HAVE_UNSIGNED_LONG_LONG_INT: c_int = 1;

/// C 493: `#define DATA_START get_data_start ()`
macro_rules! DATA_START { () => { get_data_start () }; }
/// C 394: `#define EMACS_CONFIGURATION get_emacs_configuration ()`
macro_rules! EMACS_CONFIGURATION { () => { get_emacs_configuration () }; }
/// C 395: `#define EMACS_CONFIG_OPTIONS get_emacs_configuration_options ()`
macro_rules! EMACS_CONFIG_OPTIONS { () => { get_emacs_configuration_options () }; }
/// C 499: `#define W32_DAEMON_EVENT "EmacsServerEvent"`
pub const W32_DAEMON_EVENT: &core::ffi::CStr = c"EmacsServerEvent";

// C 207-247: the calls Emacs emulates, renamed to their `sys_*` entry points.
/// C 207: `#define chdir sys_chdir`
macro_rules! chdir { ($($args:tt)*) => { sys_chdir ($($args)*) }; }
/// C 209: `#define chmod sys_chmod`
macro_rules! chmod { ($($args:tt)*) => { sys_chmod ($($args)*) }; }
/// C 211: `#define close sys_close`
macro_rules! close { ($($args:tt)*) => { sys_close ($($args)*) }; }
/// C 213: `#define creat sys_creat`
macro_rules! creat { ($($args:tt)*) => { sys_creat ($($args)*) }; }
/// C 214: `#define ctime sys_ctime`
macro_rules! ctime { ($($args:tt)*) => { sys_ctime ($($args)*) }; }
/// C 216: `#define dup sys_dup`
macro_rules! dup { ($($args:tt)*) => { sys_dup ($($args)*) }; }
/// C 218: `#define dup2 sys_dup2`
macro_rules! dup2 { ($($args:tt)*) => { sys_dup2 ($($args)*) }; }
/// C 219: `#define fopen sys_fopen`
macro_rules! fopen { ($($args:tt)*) => { sys_fopen ($($args)*) }; }
/// C 220: `#define link sys_link`
macro_rules! link { ($($args:tt)*) => { sys_link ($($args)*) }; }
/// C 221: `#define localtime sys_localtime`
macro_rules! localtime { ($($args:tt)*) => { sys_localtime ($($args)*) }; }
/// C 223: `#define rename sys_rename`
macro_rules! rename { ($($args:tt)*) => { sys_rename ($($args)*) }; }
/// C 224: `#define rmdir sys_rmdir`
macro_rules! rmdir { ($($args:tt)*) => { sys_rmdir ($($args)*) }; }
/// C 225: `#define select sys_select`
macro_rules! select { ($($args:tt)*) => { sys_select ($($args)*) }; }
/// C 226: `#define pselect sys_select`
macro_rules! pselect { ($($args:tt)*) => { sys_select ($($args)*) }; }
/// C 227: `#define sleep sys_sleep`
macro_rules! sleep { ($($args:tt)*) => { sys_sleep ($($args)*) }; }
/// C 229: `#define unlink sys_unlink`
macro_rules! unlink { ($($args:tt)*) => { sys_unlink ($($args)*) }; }
/// C 231: `#define opendir sys_opendir`
macro_rules! opendir { ($($args:tt)*) => { sys_opendir ($($args)*) }; }
/// C 233: `#define closedir sys_closedir`
macro_rules! closedir { ($($args:tt)*) => { sys_closedir ($($args)*) }; }
/// C 235: `#define readdir sys_readdir`
macro_rules! readdir { ($($args:tt)*) => { sys_readdir ($($args)*) }; }
/// C 237: `#define seekdir sys_seekdir`
macro_rules! seekdir { ($($args:tt)*) => { sys_seekdir ($($args)*) }; }
/// C 243: `#define write sys_write`
macro_rules! write { ($($args:tt)*) => { sys_write ($($args)*) }; }
/// C 245: `#define umask sys_umask`
macro_rules! umask { ($($args:tt)*) => { sys_umask ($($args)*) }; }
/// C 247: `#define clock sys_clock`
macro_rules! clock { ($($args:tt)*) => { sys_clock ($($args)*) }; }
/// C 250: `#define spawnve sys_spawnve`
macro_rules! spawnve { ($($args:tt)*) => { sys_spawnve ($($args)*) }; }
/// C 251: `#define kill sys_kill`
macro_rules! kill { ($($args:tt)*) => { sys_kill ($($args)*) }; }
/// C 252: `#define signal sys_signal`
macro_rules! signal { ($($args:tt)*) => { sys_signal ($($args)*) }; }
/// C 255: `#define emacs_raise(sig) emacs_abort()`
macro_rules! emacs_raise { ($sig:expr) => { emacs_abort () }; }
/// C 258: `#define tputs sys_tputs`
macro_rules! tputs { ($($args:tt)*) => { sys_tputs ($($args)*) }; }
/// C 259: `#define tgetstr sys_tgetstr`
macro_rules! tgetstr { ($($args:tt)*) => { sys_tgetstr ($($args)*) }; }
/// C 262: `#define chcheckmagic sys_chcheckmagic`
macro_rules! chcheckmagic { ($($args:tt)*) => { sys_chcheckmagic ($($args)*) }; }
/// C 263: `#define cmcostinit sys_cmcostinit`
macro_rules! cmcostinit { ($($args:tt)*) => { sys_cmcostinit ($($args)*) }; }
/// C 264: `#define cmgoto sys_cmgoto`
macro_rules! cmgoto { ($($args:tt)*) => { sys_cmgoto ($($args)*) }; }
/// C 265: `#define cmputc sys_cmputc`
macro_rules! cmputc { ($($args:tt)*) => { sys_cmputc ($($args)*) }; }
/// C 266: `#define Wcm_clear sys_Wcm_clear`
macro_rules! Wcm_clear { ($($args:tt)*) => { sys_Wcm_clear ($($args)*) }; }
/// C 272: `#define strerror sys_strerror`
macro_rules! strerror { ($($args:tt)*) => { sys_strerror ($($args)*) }; }
/// C 278: `#define tcdrain _commit`
macro_rules! tcdrain { ($($args:tt)*) => { _commit ($($args)*) }; }
/// C 279: `#define fdopen _fdopen`
macro_rules! fdopen { ($($args:tt)*) => { _fdopen ($($args)*) }; }
/// C 280: `#define fsync _commit`
macro_rules! fsync { ($($args:tt)*) => { _commit ($($args)*) }; }
/// C 281: `#define ftruncate _chsize`
macro_rules! ftruncate { ($($args:tt)*) => { _chsize ($($args)*) }; }
/// C 282: `#define getpid _getpid`
macro_rules! getpid { ($($args:tt)*) => { _getpid ($($args)*) }; }
/// C 283: `#define isatty _isatty`
macro_rules! isatty { ($($args:tt)*) => { _isatty ($($args)*) }; }
/// C 284: `#define _longjmp longjmp`
macro_rules! _longjmp { ($($args:tt)*) => { longjmp ($($args)*) }; }
/// C 285: `#define execvp _execvp`
macro_rules! execvp { ($($args:tt)*) => { _execvp ($($args)*) }; }
/// C 298: `#define tzname _tzname`
macro_rules! tzname { () => { _tzname }; }
/// C 308: `#define fileno _fileno`
macro_rules! fileno { ($($args:tt)*) => { _fileno ($($args)*) }; }
/// C 313: `#define read sys_read`
macro_rules! read { ($($args:tt)*) => { sys_read ($($args)*) }; }
/// C 319: `#define open sys_open`
macro_rules! open { ($($args:tt)*) => { sys_open ($($args)*) }; }
/// C 384: `#define getdisk() (_getdrive () - 1)`
macro_rules! getdisk { () => { _getdrive () - 1 }; }
/// C 386 and 388: `#define getdefdir(_drv, _buf) …` - two arms (the `emacs` one builds the
/// path in place, the other calls the CRT).  C keeps the second.
macro_rules! getdefdir { ($_drv:expr, $_buf:expr) => { _getdcwd ($_drv, $_buf, MAXPATHLEN) }; }
/// C 417: `#define mkdir(d,f) sys_mkdir(d,f)`
macro_rules! mkdir { ($d:expr, $f:expr) => { sys_mkdir ($d, $f) }; }
/// C 443: `#define malloc(size) (*the_malloc_fn)(size)`
macro_rules! malloc { ($size:expr) => { (*the_malloc_fn) ($size) }; }
/// C 444: `#define free(ptr) (*the_free_fn)(ptr)`
macro_rules! free { ($ptr:expr) => { (*the_free_fn) ($ptr) }; }
/// C 445: `#define realloc(ptr, size) (*the_realloc_fn)(ptr, size)`
macro_rules! realloc { ($ptr:expr, $size:expr) => { (*the_realloc_fn) ($ptr, $size) }; }
/// C 446: `#define calloc(num, size) sys_calloc(num, size)`
macro_rules! calloc { ($num:expr, $size:expr) => { sys_calloc ($num, $size) }; }
/// C 468: `#define putenv sys_putenv`
macro_rules! putenv { ($($args:tt)*) => { sys_putenv ($($args)*) }; }
/// C 201-203: `#define sigsetjmp(j,m) …` - two arms (`__builtin_setjmp` under `MINGW_W64`,
/// `setjmp` otherwise); C keeps the second.
macro_rules! sigsetjmp { ($j:expr, $m:expr) => { setjmp ($j) }; }

pub mod ms_w32_decls {
  use super::*;

  unsafe extern "C" {
    /// C 134: `extern char *getenv ();`
    pub fn getenv () -> *mut c_char;
    /// C 158: `extern struct tm * sys_localtime (const time_t *);`
    pub fn sys_localtime (time: *const time_t) -> *mut tm;
    /// C 174: `extern void w32_reset_stack_overflow_guard (void);`
    pub fn w32_reset_stack_overflow_guard ();
    /// C 199: `extern int sys_unlink (const char *);`
    pub fn sys_unlink (name: *const c_char) -> c_int;
    /// C 215: `extern int sys_umask (int);`
    pub fn sys_umask (mask: c_int) -> c_int;
    /// C 232: `char *sys_strerror (int);`
    pub fn sys_strerror (errnum: c_int) -> *mut c_char;
    /// C 246: `extern intptr_t _execvp (const char *, char **);`
    pub fn _execvp (file: *const c_char, argv: *mut *mut c_char) -> intptr_t;
    /// C 249: `int _getpid (void);`
    pub fn _getpid () -> c_int;
    /// C 302: `extern struct tm *gmtime_r (time_t const * restrict, struct tm * restrict);`
    pub fn gmtime_r (time: *const time_t, result: *mut tm) -> *mut tm;
    /// C 303: `extern struct tm *localtime_r (time_t const * restrict, struct tm * restrict);`
    pub fn localtime_r (time: *const time_t, result: *mut tm) -> *mut tm;
    /// C 316: `int sys_read (int, char *, unsigned int);`
    pub fn sys_read (fd: c_int, buffer: *mut c_char, count: c_uint) -> c_int;
    /// C 321: `int sys_open (const char *, int, ...);`
    pub fn sys_open (path: *const c_char, flags: c_int, ...) -> c_int;
    /// C 352: `extern signal_handler sys_signal (int, signal_handler);`
    pub fn sys_signal (sig: c_int, handler: signal_handler) -> signal_handler;
    /// C 366: `extern int sigemptyset (sigset_t *);`
    pub fn sigemptyset (set: *mut sigset_t) -> c_int;
    /// C 367: `extern int sigaddset (sigset_t *, int);`
    pub fn sigaddset (set: *mut sigset_t, sig: c_int) -> c_int;
    /// C 368: `extern int sigfillset (sigset_t *);`
    pub fn sigfillset (set: *mut sigset_t) -> c_int;
    /// C 369: `extern int sigprocmask (int, const sigset_t *, sigset_t *);`
    pub fn sigprocmask (how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    /// C 373: `extern int pthread_sigmask (int, const sigset_t *, sigset_t *);`
    pub fn pthread_sigmask (how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    /// C 374: `extern int sigismember (const sigset_t *, int);`
    pub fn sigismember (set: *const sigset_t, sig: c_int) -> c_int;
    /// C 375: `extern int setpgrp (int, int);`
    pub fn setpgrp (pid: c_int, pgid: c_int) -> c_int;
    /// C 376: `extern int sigaction (int, const struct sigaction *, struct sigaction *);`
    pub fn sigaction (sig: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    /// C 377: `extern int alarm (int);`
    pub fn alarm (seconds: c_int) -> c_int;
    /// C 379: `extern int sys_kill (pid_t, int);`
    pub fn sys_kill (pid: pid_t, sig: c_int) -> c_int;
    /// C 381: `extern void explicit_bzero (void *, size_t);`
    pub fn explicit_bzero (buffer: *mut c_void, length: size_t);
    /// C 392: `extern char *get_emacs_configuration (void);`
    pub fn get_emacs_configuration () -> *mut c_char;
    /// C 393: `extern char *get_emacs_configuration_options (void);`
    pub fn get_emacs_configuration_options () -> *mut c_char;
    /// C 418: `int sys_mkdir (const char *, mode_t);`
    pub fn sys_mkdir (path: *const c_char, mode: mode_t) -> c_int;
    /// C 426-435: the `*_before_dump` / `*_after_dump` / `*_after_dump_9x` allocator entry
    /// points the dumpable-allocator switch uses.
    pub fn malloc_before_dump (size: size_t) -> *mut c_void;
    pub fn realloc_before_dump (ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free_before_dump (ptr: *mut c_void);
    pub fn malloc_after_dump (size: size_t) -> *mut c_void;
    pub fn realloc_after_dump (ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free_after_dump (ptr: *mut c_void);
    pub fn malloc_after_dump_9x (size: size_t) -> *mut c_void;
    pub fn realloc_after_dump_9x (ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free_after_dump_9x (ptr: *mut c_void);
    /// C 437: `extern void *sys_calloc(size_t, size_t);`
    pub fn sys_calloc (num: size_t, size: size_t) -> *mut c_void;
    /// C 439-441: `extern malloc_fn the_malloc_fn;` and its two siblings - the indirection
    /// C's `malloc`/`free`/`realloc` macros go through.
    pub static mut the_malloc_fn: malloc_fn;
    pub static mut the_realloc_fn: realloc_fn;
    pub static mut the_free_fn: free_fn;
    /// C 469: `extern int sys_putenv (char *);`
    pub fn sys_putenv (string: *mut c_char) -> c_int;
    /// C 471: `extern int getloadavg (double *, int);`
    pub fn getloadavg (loadavg: *mut c_double, nelem: c_int) -> c_int;
    /// C 472: `extern int getpagesize (void);`
    pub fn getpagesize () -> c_int;
    /// C 474: `extern void * memrchr (void const *, int, size_t);`
    pub fn memrchr (s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    /// C 477: `extern int mkostemp (char *, int);`
    pub fn mkostemp (template: *mut c_char, flags: c_int) -> c_int;
  }
}
use ms_w32_decls::*;
