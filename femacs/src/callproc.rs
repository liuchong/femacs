// femacs: port of src/callproc.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// macro, enum member, global, local), same control flow, same error model.  The three
// `DEFUN`s use `DEFUN!` (PORTING.md 5.1).  C's non-`static` definitions
// (`get_current_directory`, `record_kill_process`, `emacs_spawn`, `make_environment_block`,
// `egetenv_internal`, `init_callproc_1`, `init_callproc`, `set_initial_environment`,
// `syms_of_callproc`) carry `#[unsafe(no_mangle)]` (PORTING.md 0.2); the C `static` helpers
// stay plain `unsafe fn` (the ones that never return are `-> !`: C's `AVOID` and
// `CHILD_SETUP_TYPE`).
//
// **Carried arms.**  `src/callproc.c` is *not* in
// `docs/baseline/config/cond-eval-verified.txt`:
// `python3 tools/check-cond-eval.py --show-unknown src/callproc.c` reports
// `75 conditions, 0 unknown, 1 disagreements [NOT VERIFIED]` — `#ifdef TIOCSCTTY` (C 1561),
// which the frozen build resolves *true* because the macro comes from `<sys/ioctl.h>`, not
// from `config.h`, so the frozen macro table cannot see it (docs/status.md 3.35's class;
// report to the integrator, who owns the baselines).  `tools/extract-abi.py`'s blind
// evaluator therefore keeps *both* arms of every `#if` live, and per the integrator's
// 2026-10-03 decision (docs/status.md 3.49, PORTING.md P-51) the port *carries* the text of
// the arms the frozen build does not compile, each under a `#[cfg (feature = "…")]` naming
// the C condition's non-frozen side.  The arms, in C order:
//
//   * C 28-30 (`environ`), C 66-69, C 83-85, C 116-117, C 372-373, C 514-516, C 557-627,
//     C 628-664, C 1239-1243, C 1295-1317, C 2091-2094: `MSDOS` (feature `msdos`; the
//     `sys/stat.h`/`sys/param.h`/`msdos.h` include selections are recorded in the ledger).
//   * C 54-57 (`SETUP_SLAVE_PTY`), C 1616-1621: the same feature, and C 1567-1575 (`LDISC1`
//     -> `ldisc1`), C 1577-1583 (`NTTYDISC && TIOCSETD` -> `nttydisc`), C 1587-1613
//     (`!DONT_REOPEN_PTY` -> `no_dont_reopen_pty`) — the terminal-ldisc arms.
//   * C 59-64 (`WINDOWSNT`, feature `windowsnt`; including `_P_NOWAIT`), C 992-1005,
//     C 1244-1246, C 1268-1275, C 1513-1551, C 1655-1656, C 1689-1691, C 1726-1735,
//     C 2050-2053.
//   * C 168-180, C 1446-1455, C 2039-2043, C 2177-2180, C 2187-2190, C 2198-2201,
//     C 2211-2215, C 2223-2226, C 2234-2237: `HAVE_ANDROID && !ANDROID_STUBIFY`
//     (feature `have_android`).
//   * C 87-89 (`HAVE_NS` -> `have_ns`), C 91-93 and C 1882-1890 (`HAVE_PGTK` ->
//     `have_pgtk`), C 266-267 (`DOS_NT` -> `dos_nt`, also C 975-977, C 1832-1838,
//     C 2059-2061), C 396-400 (`!subprocesses` -> `no_subprocesses`), C 1355-1358
//     (`HAVE_POSIX_SPAWN_FILE_ACTIONS_ADDCHDIR && !HAIKU` -> `no_posix_spawn_addchdir`),
//     C 1527-1536 (`!DARWIN_OS` -> `no_darwin_os`).
//
// The one arm clang *does* compile and the table cannot see is `TIOCSCTTY` (C 1561): the
// port keeps that `ioctl` call as live text with the target's measured value, so the port
// compiles exactly what the frozen build compiles; the blind evaluator requires the tokens
// either way.
//
// `#include` selections (PORTING.md 7.2's third category, recorded in the ledger):
// C 54-57, 59-64, 66-69, 83-85, 87-89, 91-93, 95-97, 992-1005 and the `<epaths.h>` values
// of C 1985, 2010, 2019, 2032, 2047, 2124, 2172 — mirrors exist for every live one (the
// include list below).  `epaths.h`'s `PATH_*` values are the frozen build's measured ones.
//
// Names no mirror carries are declared at their use site (the terminal.rs/filelock.rs
// shape), each with C's own prototype in the comment.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_parens, unreachable_code, unused_assignments)]

include!("include/config.rs");                 // C 22: `#include <config.h>`
include!("../lib/include/errno.rs");           // C 23: `#include <errno.h>`
include!("../lib/include/stdlib.rs");          // C 24: `#include <stdlib.h>`
include!("../lib/include/sys_types.in.rs");    // C 25: `#include <sys/types.h>`
include!("../lib/include/unistd.in.rs");       // C 26: `#include <unistd.h>`
include!("../lib/include/fcntl.in.rs");        // C 33: `#include <fcntl.h>`
include!("include/lisp.rs");                   // C 52: `#include "lisp.h"`
include!("../lib/include/stdio.rs");           // C 40: `#include <stdio.h>` (mini-gmp's FILE)
include!("../lib/include/string.rs");          // lisp.h 30: `#include <string.h>`
include!("../lib/include/mini-gmp.rs");        // bignum.h's `#include <gmp.h>`
include!("include/bignum.rs");                 // lisp.h 2359's struct Lisp_Bignum
include!("include/character.rs");              // buffer.h's character-table names
include!("include/itree.rs");                  // buffer.h 35's `#include "itree.h"`
include!("include/composite.rs");              // coding.h 284's `#include "composite.h"`
include!("include/coding.rs");                 // C 73: `#include "coding.h"`
include!("include/dispextern.rs");             // frame.h 25's `#include "dispextern.h"`
include!("include/font.rs");                   // dispextern.h's font names
include!("include/termhooks.rs");              // frame.h's `#include "termhooks.h"`
include!("include/systime.rs");                // frame.h's `#include "systime.h"`
include!("include/window.rs");                 // keyboard.h 26's `#include "window.h"`
include!("include/buffer.rs");                 // C 72: `#include "buffer.h"`
include!("include/commands.rs");               // C 71: `#include "commands.h"`
include!("include/process.rs");                // C 75: `#include "process.h"`
include!("include/syssignal.rs");              // C 76: `#include "syssignal.h"`
include!("include/blockinput.rs");             // C 78: `#include "blockinput.h"`
include!("include/frame.rs");                  // C 79: `#include "frame.h"`
include!("include/keyboard.rs");               // C 81: `#include "keyboard.h"`

/* The names C gets from lisp.h, process.h, syswait.h and the system headers above, declared
   at the use site (the terminal.rs/filelock.rs shape); none of them has a mirror.  */
unsafe extern "C" {
  /// C: Lisp_Object expand_and_dir_to_file (Lisp_Object)   (lisp.h 4972, fileio.c)
  fn expand_and_dir_to_file (filename: Lisp_Object) -> Lisp_Object;
  /// C: bool file_accessible_directory_p (Lisp_Object)   (lisp.h 4988, fileio.c)
  fn file_accessible_directory_p (filename: Lisp_Object) -> bool;
  /// C: Lisp_Object remove_slash_colon (Lisp_Object)   (process.h 311)
  fn remove_slash_colon (string: Lisp_Object) -> Lisp_Object;
  /// C: int openp (Lisp_Object path, Lisp_Object str, Lisp_Object suffixes,
  ///               Lisp_Object *storeptr, Lisp_Object predicate, bool newer,
  ///               bool no_native, void **platform)   (lisp.h 4743, lread.c 1756)
  fn openp (path: Lisp_Object, str_: Lisp_Object, suffixes: Lisp_Object,
            storeptr: *mut Lisp_Object, predicate: Lisp_Object, newer: bool,
            no_native: bool, platform: *mut *mut c_void) -> c_int;
  /// C: void setup_coding_system (Lisp_Object, struct coding_system *)   (coding.h 686)
  fn setup_coding_system (coding_system: Lisp_Object, coding: *mut coding_system);
  /// C: Lisp_Object raw_text_coding_system (Lisp_Object)   (coding.h 700)
  fn raw_text_coding_system (coding: Lisp_Object) -> Lisp_Object;
  /// C: Lisp_Object complement_process_encoding_system (Lisp_Object)   (coding.h 703)
  fn complement_process_encoding_system (coding: Lisp_Object) -> Lisp_Object;
  /// C: Lisp_Object decode_env_path (const char *, const char *, bool)   (lisp.h 5135)
  fn decode_env_path (envvar: *const c_char, deflt: *const c_char,
                      empty_is_default: bool) -> Lisp_Object;
  /// C: void dir_warning (const char *, Lisp_Object) AVOID   (lisp.h 4750)
  fn dir_warning (format: *const c_char, argument: Lisp_Object) -> !;
  /// C: Lisp_Object write_region (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object,
  ///                              Lisp_Object, Lisp_Object, Lisp_Object, int)  (lisp.h 4973)
  fn write_region (start: Lisp_Object, end: Lisp_Object, filename: Lisp_Object,
                   append: Lisp_Object, visit: Lisp_Object, lockname: Lisp_Object,
                   mustbenew: Lisp_Object, desc: c_int) -> Lisp_Object;
  /// C: bool wait_for_termination (pid_t, int *, bool)   (syswait.h 59)
  fn wait_for_termination (pid: pid_t, status: *mut c_int, hang: bool) -> bool;
  /// C: void record_deleted_pid (pid_t, Lisp_Object)   (process.h 289)
  fn record_deleted_pid (pid: pid_t, tempfile: Lisp_Object);
  /// C: void child_signal_init (void)   (lisp.h 5179, process.c)
  fn child_signal_init ();
  /// C: void child_setup_tty (int)   (lisp.h 5294)
  fn child_setup_tty (out: c_int);
  /// C: void dissociate_controlling_tty (void)   (process.h 314, sysdep.c)
  fn dissociate_controlling_tty ();
  /// C: void restore_nofile_limit (void)   (process.h 304)
  fn restore_nofile_limit ();
  /// C: void emacs_perror (char const *)   (lisp.h 5337)
  fn emacs_perror (message: *const c_char);
  /// C: int emacs_pipe (int[2])   (lisp.h 5313)
  fn emacs_pipe (pipefd: *mut c_int) -> c_int;
  /// C: int emacs_exec_file (char const *, char *const *, char *const *)   (lisp.h 5282)
  fn emacs_exec_file (file: *const c_char, argv: *const *mut c_char,
                      envp: *const *mut c_char) -> c_int;
  /// C: int emacs_open_noquit (const char *, int, int)   (lisp.h 5312)
  #[cfg (feature = "no_dont_reopen_pty")]
  fn emacs_open_noquit (file: *const c_char, oflags: c_int, mode: c_int) -> c_int;
  /// C: void synchronize_system_messages_locale (void)   (lisp.h 5142)
  fn synchronize_system_messages_locale ();
  /// C: void record_unwind_protect_nothing (void)   (lisp.h 4827)
  fn record_unwind_protect_nothing ();
  /// C: void set_unwind_protect (specpdl_ref, void (*) (Lisp_Object), Lisp_Object)
  ///    (lisp.h 4830)
  fn set_unwind_protect (count: specpdl_ref,
                         func: Option<unsafe extern "C" fn (Lisp_Object)>,
                         arg: Lisp_Object);
  /// C: void clear_unwind_protect (specpdl_ref)   (lisp.h 4829)
  fn clear_unwind_protect (count: specpdl_ref);
  /// C: void labeled_restrictions_remove_in_current_buffer (void)   (lisp.h 4931)
  fn labeled_restrictions_remove_in_current_buffer ();
  /// C: void insert_1_both (const char *, ptrdiff_t, ptrdiff_t, bool, bool, bool)
  ///    (lisp.h 4331, insdel.c)
  fn insert_1_both (string: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
                    inherit: bool, before_markers: bool, prepare: bool);
  /// C: Lisp_Object del_range_2 (ptrdiff_t, ptrdiff_t, ptrdiff_t, ptrdiff_t, bool)
  ///    (lisp.h 4349, insdel.c)
  fn del_range_2 (from: ptrdiff_t, to: ptrdiff_t, from_byte: ptrdiff_t,
                  to_byte: ptrdiff_t, prepare: bool) -> Lisp_Object;
  /// C: void redisplay_preserve_echo_area (int)   (dispextern.h 3589)
  fn redisplay_preserve_echo_area (from_where: c_int);
  /// C: void record_unwind_protect_nothing's and close_file_unwind's shapes are in lisp.rs.
  /// C: Lisp_Object nconc2 (Lisp_Object, Lisp_Object)   (lisp.h 4271, fns.c)
  fn nconc2 (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
  /// C: int emacs_unlink (const char *)   (the filelock.rs shape, lisp.h 5325)
  fn emacs_unlink (name: *const c_char) -> c_int;
  /// C: int kill (pid_t, int)   -- <signal.h>
  fn kill (pid: pid_t, sig: c_int) -> c_int;
  /// C: int mkostemp (char *, int)   -- <stdlib.h>
  fn mkostemp (template: *mut c_char, flags: c_int) -> c_int;
  /// C: char *strerror (int)   -- <string.h>
  /// C: char *strcat (char *, const char *)   -- <string.h>
  fn strcat (dest: *mut c_char, src: *const c_char) -> *mut c_char;
  /// C: char *strsignal (int)   -- <string.h>; HAVE_STRSIGNAL is 1 on this target, so
  /// syssignal.h's `#define strsignal(sig) safe_strsignal (sig)` is dead text.
  fn strsignal (sig: c_int) -> *mut c_char;
  /// C: int setpgid (pid_t, pid_t)   -- <unistd.h>
  fn setpgid (pid: pid_t, pgid: pid_t) -> c_int;
  /// C: int tcsetpgrp (int, pid_t)   -- <unistd.h>
  fn tcsetpgrp (fd: c_int, pgrp: pid_t) -> c_int;
  /// C: int ioctl (int, unsigned long, ...)   -- <sys/ioctl.h>
  fn ioctl (fd: c_int, request: c_ulong, ...) -> c_int;
  /// C: pid_t fork (void)   -- <unistd.h>
  fn fork () -> pid_t;
  /// C: pid_t VFORK (void)   -- process.h 40's `#define VFORK vfork` (the DARWIN_OS arm of
  /// C 1527-1536 uses it; `vfork` itself is declared there on this target).
  fn VFORK () -> pid_t;
  /// C: void _exit (int) _Noreturn   -- <unistd.h>
  fn _exit (status: c_int) -> !;
}

/* `<spawn.h>`: the live `USABLE_POSIX_SPAWN` arm (C 37-50).  Darwin defines
   `posix_spawn_file_actions_t` and `posix_spawnattr_t` as pointers to opaque records;
   measured with the frozen clang: `sizeof` is 8 for both, `POSIX_SPAWN_SETSID` 1024,
   `POSIX_SPAWN_SETSIGDEF` 4 and `POSIX_SPAWN_SETSIGMASK` 8.  */
#[repr(C)] pub struct _posix_spawn_file_actions { _private: [u8; 0] }
#[repr(C)] pub struct _posix_spawnattr { _private: [u8; 0] }
pub type posix_spawn_file_actions_t = *mut _posix_spawn_file_actions;
pub type posix_spawnattr_t = *mut _posix_spawnattr;
/// <spawn.h>: `#define POSIX_SPAWN_SETSID 0x0400`.
const POSIX_SPAWN_SETSID: c_short = 1024;
/// <spawn.h>: `#define POSIX_SPAWN_SETSIGDEF 0x0004`.
const POSIX_SPAWN_SETSIGDEF: c_short = 4;
/// <spawn.h>: `#define POSIX_SPAWN_SETSIGMASK 0x0008`.
const POSIX_SPAWN_SETSIGMASK: c_short = 8;

unsafe extern "C" {
  /// C: int posix_spawn (pid_t *, const char *, const posix_spawn_file_actions_t *,
  ///                    const posix_spawnattr_t *, char *const [], char *const [])
  fn posix_spawn (pid: *mut pid_t, path: *const c_char,
                  file_actions: *const posix_spawn_file_actions_t,
                  attrp: *const posix_spawnattr_t, argv: *const *mut c_char,
                  envp: *const *mut c_char) -> c_int;
  /// C: int posix_spawn_file_actions_init (posix_spawn_file_actions_t *)
  fn posix_spawn_file_actions_init (actions: *mut posix_spawn_file_actions_t) -> c_int;
  /// C: int posix_spawn_file_actions_adddup2 (posix_spawn_file_actions_t *, int, int)
  fn posix_spawn_file_actions_adddup2 (actions: *mut posix_spawn_file_actions_t,
                                       fd: c_int, newfd: c_int) -> c_int;
  /// C: int posix_spawn_file_actions_addchdir (posix_spawn_file_actions_t *, const char *)
  fn posix_spawn_file_actions_addchdir (actions: *mut posix_spawn_file_actions_t,
                                        path: *const c_char) -> c_int;
  /// C: int posix_spawn_file_actions_destroy (posix_spawn_file_actions_t *)
  fn posix_spawn_file_actions_destroy (actions: *mut posix_spawn_file_actions_t) -> c_int;
  /// C: int posix_spawnattr_init (posix_spawnattr_t *)
  fn posix_spawnattr_init (attr: *mut posix_spawnattr_t) -> c_int;
  /// C: int posix_spawnattr_setflags (posix_spawnattr_t *, short)
  fn posix_spawnattr_setflags (attr: *mut posix_spawnattr_t, flags: c_short) -> c_int;
  /// C: int posix_spawnattr_setsigdefault (posix_spawnattr_t *, const sigset_t *)
  fn posix_spawnattr_setsigdefault (attr: *mut posix_spawnattr_t,
                                    sigdefault: *const sigset_t) -> c_int;
  /// C: int posix_spawnattr_setsigmask (posix_spawnattr_t *, const sigset_t *)
  fn posix_spawnattr_setsigmask (attr: *mut posix_spawnattr_t,
                                 sigmask: *const sigset_t) -> c_int;
  /// C: int posix_spawnattr_destroy (posix_spawnattr_t *)
  fn posix_spawnattr_destroy (attr: *mut posix_spawnattr_t) -> c_int;
}

/// <sys/ioctl.h>: `#define TIOCSCTTY _IO('T', 97)` — measured with the frozen clang:
/// 536900705 (0x20007461).  This is the frozen macro table's one blind spot for this file:
/// `#ifdef TIOCSCTTY` (C 1561) *is* true for the frozen build (the macro comes from the
/// system header, not `config.h`), and the port follows clang (see the file header).
const TIOCSCTTY: c_ulong = 536900705;

/// <signal.h>: `#define SIG_DFL ((void (*)(int)) 0)` — the integer 0 (signal.rs's `SIG_IGN`
/// row's twin).
const SIG_DFL: usize = 0;

/* <unistd.h>: `STDIN_FILENO`/`STDOUT_FILENO`/`STDERR_FILENO` now come from the unistd.in.rs
   mirror (2026-10-04); this unit's use-site copies are gone, and the mirror is the single
   declaration point (PORTING.md 0.1-3/P-26).  */

/// <unistd.h>: `#define _SC_ARG_MAX 1` (the frozen macro table's value; the lib mirrors do
/// not carry the `sysconf` names).
const _SC_ARG_MAX: c_int = 1;

/// <sys/wait.h> on the frozen target (docs/baseline/config/darwin-arm64.macros.tsv):
/// `_WSTATUS(x)` is `((x) & 0177)`, `_WSTOPPED` is `0177`, and `_W_INT` is the identity.
#[inline(always)] fn WTERMSIG (status: c_int) -> c_int { status & 0o177 }
#[inline(always)] fn WIFSIGNALED (status: c_int) -> bool
{ (status & 0o177) != 0o177 && (status & 0o177) != 0 }
#[inline(always)] fn WIFEXITED (status: c_int) -> bool { (status & 0o177) == 0 }
#[inline(always)] fn WEXITSTATUS (status: c_int) -> c_int { (status >> 8) & 0xff }

/// <config.h>/<paths.h>: `#define NULL_DEVICE "/dev/null"` (the frozen macro table).
const NULL_DEVICE: &std::ffi::CStr = c"/dev/null";

/// <epaths.h> (the frozen build's `build/emacs-ref/src/epaths.h`, measured; the file is
/// generated, so the port keeps the values).  `PATH_GAME` is the null pointer there.
const PATH_DATA: &std::ffi::CStr = c"/usr/local/share/emacs/32.0.50/etc";
const PATH_DOC: &std::ffi::CStr = c"/usr/local/share/emacs/32.0.50/etc";
const PATH_EXEC: &std::ffi::CStr =
  c"/usr/local/libexec/emacs/32.0.50/aarch64-apple-darwin25.6.0";
const PATH_INFO: &std::ffi::CStr = c"/usr/local/share/info";
const PATH_DUMPLOADSEARCH: &std::ffi::CStr =
  c"/Users/liu/projects/src/github.com/liuchong/femacs/build/emacs-ref/../../emacs/lisp";
const PATH_GAME: *const c_char = std::ptr::null ();

/// C 45-47's `#define USABLE_POSIX_SPAWN 1` (the frozen target has `<spawn.h>`,
/// `posix_spawn`, `posix_spawnattr_setflags`, `posix_spawn_file_actions_addchdir` and
/// `HAVE_DECL_POSIX_SPAWN_SETSID == 1`, and it is not Haiku).
const USABLE_POSIX_SPAWN: bool = true;
/// C 49-50's `#else` arm; clang rejects it (feature never set).  `USABLE_POSIX_SPAWN` has
/// two definitions in C, so the extractor compares presence, not the body.
#[cfg (feature = "no_usable_posix_spawn")]
const USABLE_POSIX_SPAWN: bool = false;

/// <config.h 120>: `#define DIRECTORY_SEP '/'`.
const DIRECTORY_SEP: c_char = b'/' as c_char;
/// <config.h 2095>: `#define IS_DEVICE_SEP(_c_) 0` (the non-DOS value).
#[inline(always)] fn IS_DEVICE_SEP (_c_: c_char) -> bool { let _ = _c_; false }
/// <config.h 2097>: `#define IS_DIRECTORY_SEP(_c_) ((_c_) == DIRECTORY_SEP)`.
#[inline(always)] fn IS_DIRECTORY_SEP (_c_: c_char) -> bool { _c_ == DIRECTORY_SEP }

/* Pattern used by call-process-region to make temp files.  */
static mut Vtemp_file_name_pattern: Lisp_Object = lisp_h_Qnil ();

/* The next two variables are used while record-unwind-protect is in place
   during call-process for a subprocess for which record_deleted_pid has
   not yet been called.  At other times, synch_process_pid is zero and
   synch_process_tempfile's contents are irrelevant.  Doing this via static
   C variables is more convenient than putting them into the arguments
   of record-unwind-protect, as they need to be updated at randomish
   times in the code, and Lisp cannot always store these values as
   Emacs integers.  It's safe to use static variables here, as the
   code is never invoked reentrantly.  */

/* If nonzero, a process-ID that has not been reaped.  */
static mut synch_process_pid: pid_t = 0;

/* If a string, the name of a temp file that has not been removed.  */
#[cfg (feature = "msdos")]
static mut synch_process_tempfile: Lisp_Object = lisp_h_Qnil ();
/// C 119-120: `#else # define synch_process_tempfile make_fixnum (0)` — the non-MSDOS arm,
/// P-17's shape (a value-like macro is a same-named inline fn).
#[inline(always)]
fn synch_process_tempfile () -> Lisp_Object { make_fixnum (0) }

/* Indexes of file descriptors that need closing on call_process_kill.  */
pub const CALLPROC_STDOUT: c_int = 0;
pub const CALLPROC_STDERR: c_int = 1;
/* How to read from a pipe (or substitute) from the subsidiary process.  */
pub const CALLPROC_PIPEREAD: c_int = 2;
/* A bound on the number of file descriptors.  */
pub const CALLPROC_FDS: c_int = 3;

/* C 28-30's `#ifdef MSDOS` arm declares `extern char **environ;`.  It now comes from the
   unistd.in.rs mirror (2026-10-04): the frozen target's <unistd.h> declares the same object,
   and the port keeps one declaration per name (PORTING.md 0.1-3; two declarations were
   E0428).  */

/* C 59-64's `#ifdef WINDOWSNT` arm's `#define _P_NOWAIT 1` (from process.h); its three
   `#include`s are the ledger's.  */
#[cfg (feature = "windowsnt")]
const _P_NOWAIT: c_int = 1;

/* Return the current buffer's working directory, or the home
   directory if it's unreachable.  If ENCODE is true, return as a string
   suitable for a system call; otherwise, return a string in its
   internal representation.  Signal an error if the result would not be
   an accessible directory.

   If the default directory lies inside a special directory which
   cannot be made the current working directory, and ENCODE is also
   set, simply return the home directory.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_current_directory (encode: bool) -> Lisp_Object
{
  let curdir: Lisp_Object = BVAR! (current_buffer, directory_);
  let mut dir: Lisp_Object = Funhandled_file_name_directory (curdir);

  /* If the file name handler says that dir is unreachable, use
     a sensible default. */
  if NILP (dir)
    { dir = build_string (c"~".as_ptr ()); }

  /* C 168-180's `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` arm.  */
  #[cfg (feature = "have_android")]
  {
    /* If DIR is an asset directory or a content directory, return
       the home directory instead.  */

    if encode
      && (android_is_special_directory (SSDATA (dir), c"/assets".as_ptr ())
	  || android_is_special_directory (SSDATA (dir), c"/content".as_ptr ()))
      { dir = build_string (c"~".as_ptr ()); }
  }

  dir = expand_and_dir_to_file (dir);
  let encoded_dir: Lisp_Object = ENCODE_FILE (remove_slash_colon (dir));

  if ! file_accessible_directory_p (encoded_dir)
    { report_file_error (c"Setting current directory".as_ptr (), curdir); }

  return if encode { encoded_dir } else { dir };
}

/* If P is reapable, record it as a deleted process and kill it.
   Do this in a critical section.  Unless PID is wedged it will be
   reaped on receipt of the first SIGCHLD after the critical section.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn record_kill_process (p: *mut Lisp_Process, tempfile: Lisp_Object)
{
  let mut oldset: sigset_t = std::mem::zeroed ();
  block_child_signal (&raw mut oldset);

  /* C reads `p->alive`, the 1-bit field process.rs packs into `bits2`
     (PROCESS_ALIVE_SHIFT, PORTING.md 1.3).  */
  if (*p).bits2 >> PROCESS_ALIVE_SHIFT & 1 != 0
    {
      record_deleted_pid ((*p).pid, tempfile);
      (*p).bits2 &= ! (1 << PROCESS_ALIVE_SHIFT);
      kill (- (*p).pid, SIGKILL);
    }

  unblock_child_signal (&raw const oldset);
}

/* Clean up files, file descriptors and processes created by Fcall_process.  */

unsafe extern "C" fn delete_temp_file (name: Lisp_Object)
{
  emacs_unlink (SSDATA (name));
}

unsafe extern "C" fn call_process_kill (ptr: *mut c_void)
{
  let callproc_fd: *mut c_int = ptr as *mut c_int;
  { let mut i: c_int = 0;
    while i < CALLPROC_FDS
      {
	if 0 <= *callproc_fd.offset (i as isize)
	  { emacs_close (*callproc_fd.offset (i as isize)); }
	i += 1;
      } }

  if synch_process_pid != 0
    {
      // PORTING.md 1.5: C declares `struct Lisp_Process proc;` uninitialized.
      let mut proc: Lisp_Process = std::mem::zeroed ();
      proc.bits2 |= 1 << PROCESS_ALIVE_SHIFT;   /* C: `proc.alive = 1;` */
      proc.pid = synch_process_pid;
      record_kill_process (&raw mut proc, synch_process_tempfile ());
      synch_process_pid = 0;
    }
  else if STRINGP (synch_process_tempfile ())
    { delete_temp_file (synch_process_tempfile ()); }
}

/* Clean up when exiting Fcall_process: restore the buffer, and
   kill the subsidiary process group if the process still exists.  */

unsafe extern "C" fn call_process_cleanup (buffer: Lisp_Object)
{
  Fset_buffer (buffer);

  if synch_process_pid != 0
    {
      kill (- synch_process_pid, SIGINT);
      message1 (c"Waiting for process to die...(type C-g again to kill it instantly)".as_ptr ());

      /* This will quit on C-g.  */
      let wait_ok: bool = wait_for_termination (synch_process_pid, std::ptr::null_mut (), true);
      synch_process_pid = 0;
      message1 ((if wait_ok
		 { c"Waiting for process to die...done" }
		 else
		 { c"Waiting for process to die...internal error" }).as_ptr ());
    }
}

/* C 266-270's `#ifdef DOS_NT` arm is carried text (feature `dos_nt`).  */
#[cfg (feature = "dos_nt")]
const default_output_mode: mode_t = S_IREAD | S_IWRITE;
const default_output_mode: mode_t = 0o666;     /* C: `static mode_t const default_output_mode = 0666;` */
DEFUN! ("call-process", Fcall_process, Scall_process, 1, MANY, 0,
       doc: "Call PROGRAM synchronously in separate process.\nThe remaining arguments are optional.\n\nThe program's input comes from file INFILE (nil means `null-device').\nIf INFILE is a relative path, it will be looked for relative to the\ndirectory where the process is run (see below).  If you want to make the\ninput come from an Emacs buffer, use `call-process-region' instead.\n\nThird argument DESTINATION specifies how to handle program's output.\n(\"Output\" here means both standard output and standard error\noutput.)\nIf DESTINATION is a buffer or the name of a buffer, or t (which stands for\nthe current buffer), it means insert output in that buffer before point.\nIf DESTINATION is nil, it means discard output; 0 means discard\n and don't wait for the program to terminate.\nIf DESTINATION is `(:file FILE)', where FILE is a file name string,\n it means that output should be written to that file (if the file\n already exists it is overwritten).\nDESTINATION can also have the form (REAL-BUFFER STDERR-FILE); in that case,\n REAL-BUFFER says what to do with standard output, as above,\n while STDERR-FILE says what to do with standard error in the child.\n STDERR-FILE may be nil (discard standard error output),\n t (mix it with ordinary output), or a file name string.\n\nFourth arg DISPLAY non-nil means redisplay buffer as output is inserted.\nRemaining arguments ARGS are strings passed as command arguments to PROGRAM.\n\nIf PROGRAM is not an absolute file name, `call-process' will look for\nPROGRAM in `exec-path' (which is a list of directories).\n\nIf executable PROGRAM can't be found as an executable, `call-process'\nsignals a Lisp error.  `call-process' reports errors in execution of\nthe program only through its return and output.\n\nIf DESTINATION is 0, `call-process' returns immediately with value nil.\nOtherwise it waits for PROGRAM to terminate\nand returns a numeric exit status or a signal description string.\nIf you quit, the process is killed with SIGINT, or SIGKILL if you quit again.\n\nThe process runs in `default-directory' if that is local (as\ndetermined by `unhandled-file-name-directory'), or \"~\" otherwise.  If\nyou want to run a process in a remote directory use `process-file'.\n\nusage: (call-process PROGRAM &optional INFILE DESTINATION DISPLAY &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut infile: Lisp_Object;
  let encoded_infile: Lisp_Object;
  let filefd: c_int;
  let count: specpdl_ref = SPECPDL_INDEX ();

  if nargs >= 2 && ! NILP (*args.offset (1))
    {
      /* Expand infile relative to the current buffer's current
	 directory, or its unhandled equivalent ("~").  */
      infile = Fexpand_file_name (*args.offset (1), get_current_directory (false));
      CHECK_STRING (infile);
    }
  else
    { infile = build_string (NULL_DEVICE.as_ptr ()); }

  /* Remove "/:" from INFILE.  */
  infile = remove_slash_colon (infile);

  encoded_infile = ENCODE_FILE (infile);

  filefd = emacs_open (SSDATA (encoded_infile), O_RDONLY, 0);
  if filefd < 0
    { report_file_error (c"Opening process input file".as_ptr (), infile); }
  record_unwind_protect_int (Some (close_file_unwind), filefd);
  return unbind_to (count, call_process (nargs, args, filefd,
					 make_invalid_specpdl_ref ()));
});

/* Like Fcall_process (NARGS, ARGS), except use FILEFD as the input file.

   If TEMPFILE_INDEX is valid, it is the specpdl index of an
   unwinder that is intended to remove the input temporary file; in
   this case NARGS must be at least 2 and ARGS[1] is the file's name.

   At entry, the specpdl stack top entry must be close_file_unwind (FILEFD).  */

unsafe fn call_process (nargs: ptrdiff_t, args: *mut Lisp_Object, filefd: c_int,
			tempfile_index: specpdl_ref) -> Lisp_Object
{
  let mut buffer: Lisp_Object;
  let current_dir: Lisp_Object;
  /* PORTING.md 1.5: C declares PATH uninitialized (openp writes it through the pointer).  */
  let mut path: Lisp_Object = Qnil ();
  let display_p: bool;
  let fd0: c_int;
  /* PORTING.md 1.5: C declares the array uninitialized; the loop below fills it.  */
  let mut callproc_fd: [c_int; CALLPROC_FDS as usize] = [0; CALLPROC_FDS as usize];
  let mut status: c_int = 0;
  let mut i: ptrdiff_t = 0;
  let count: specpdl_ref = SPECPDL_INDEX ();
  USE_SAFE_ALLOCA! (sa_avail, sa_count);

  /* PORTING.md 1.5: C leaves NEW_ARGV uninitialized; SAFE_NALLOCA passes it by value,
     so the port starts it at null (lisp.rs's pointee_size note).  */
  let mut new_argv: *mut *mut c_char = std::ptr::null_mut ();
  /* File to use for stderr in the child.
     t means use same as standard output.  */
  let mut error_file: Lisp_Object;
  let mut output_file: Lisp_Object = Qnil ();
  /* C 372-377's `#ifdef MSDOS` arm.  */
  #[cfg (feature = "msdos")]
  let mut tempfile: *mut c_char = std::ptr::null_mut ();
  /* PORTING.md 1.5: C declares `sigset_t oldset;` uninitialized.  */
  let mut oldset: sigset_t = std::mem::zeroed ();
  let mut pid: pid_t = -1;
  let child_errno: c_int;
  let fd_output: c_int;
  let mut fd_error: c_int;
  /* PORTING.md 1.5: C declares both coding systems uninitialized.  */
  let mut process_coding: coding_system = std::mem::zeroed ();   /* coding-system of process output */
  let mut argument_coding: coding_system = std::mem::zeroed ();  /* coding-system of arguments */
  /* Set to the return value of Ffind_operation_coding_system.  */
  let mut coding_systems: Lisp_Object;
  let discard_output: bool;

  if synch_process_pid != 0
    { error (c"call-process invoked recursively".as_ptr ()); }

  /* Qt denotes that Ffind_operation_coding_system is not yet called.  */
  coding_systems = Qt ();

  CHECK_STRING (*args);

  error_file = Qt ();

  /* C 396-401's `#ifndef subprocesses` arm.  */
  #[cfg (feature = "no_subprocesses")]
  {
    /* Without asynchronous processes we cannot have BUFFER == 0.  */
    if nargs >= 3
      && (FIXNUMP (if CONSP (*args.offset (2)) { XCAR (*args.offset (2)) } else { *args.offset (2) }))
      { error (c"Operating system cannot handle asynchronous subprocesses".as_ptr ()); }
  }

  /* Decide the coding-system for giving arguments.  */
  {
    let mut val: Lisp_Object;
    /* PORTING.md 1.5: C leaves ARGS2 uninitialized; SAFE_NALLOCA passes it by value.  */
    let mut args2: *mut Lisp_Object = std::ptr::null_mut ();

    /* If arguments are supplied, we may have to encode them.  */
    if nargs >= 5
      {
	let mut must_encode: bool = false;
	let coding_attrs: Lisp_Object;

	{ let mut i: ptrdiff_t = 4;
	  while i < nargs
	    { CHECK_STRING (*args.offset (i as isize)); i += 1; } }

	{ let mut i: ptrdiff_t = 4;
	  while i < nargs
	    {
	      if STRING_MULTIBYTE (*args.offset (i as isize))
		{ must_encode = true; }
	      i += 1;
	    } }

	if ! NILP (Vcoding_system_for_write)
	  { val = Vcoding_system_for_write; }
	else if ! must_encode
	  { val = Qraw_text (); }
	else
	  {
	    SAFE_NALLOCA! (sa_avail, args2, 1, nargs + 1);
	    *args2 = Qcall_process ();
	    { let mut i: ptrdiff_t = 0;
	      while i < nargs
		{ *args2.offset ((i + 1) as isize) = *args.offset (i as isize); i += 1; } }
	    coding_systems = Ffind_operation_coding_system (nargs + 1, args2);
	    val = if CONSP (coding_systems) { XCDR (coding_systems) } else { Qnil () };
	  }
	val = complement_process_encoding_system (val);
	setup_coding_system (Fcheck_coding_system (val), &raw mut argument_coding);
	coding_attrs = CODING_ID_ATTRS (argument_coding.id as hash_idx_t);
	if NILP (CODING_ATTR_ASCII_COMPAT (coding_attrs))
	  {
	    /* We should not use an ASCII incompatible coding system.  */
	    val = raw_text_coding_system (val);
	    setup_coding_system (val, &raw mut argument_coding);
	  }
      }
  }

  if nargs < 3
    { buffer = Qnil (); }
  else
    {
      buffer = *args.offset (2);

      /* If BUFFER is a list, its meaning is (BUFFER-FOR-STDOUT
	 FILE-FOR-STDERR), unless the first element is :file, in which case see
	 the next paragraph. */
      if CONSP (buffer) && ! EQ (XCAR (buffer), QCfile ())
	{
	  if CONSP (XCDR (buffer))
	    {
	      let stderr_file: Lisp_Object = XCAR (XCDR (buffer));

	      if NILP (stderr_file) || EQ (Qt (), stderr_file)
		{ error_file = stderr_file; }
	      else
		{ error_file = Fexpand_file_name (stderr_file, Qnil ()); }
	    }

	  buffer = XCAR (buffer);
	}

      /* If the buffer is (still) a list, it might be a (:file "file") spec. */
      if CONSP (buffer) && EQ (XCAR (buffer), QCfile ())
	{
	  let mut ofile: Lisp_Object = XCDR (buffer);
	  if CONSP (ofile)
	    { ofile = XCAR (ofile); }
	  CHECK_STRING (ofile);
	  output_file = Fexpand_file_name (ofile, BVAR! (current_buffer, directory_));
	  CHECK_STRING (output_file);
	  buffer = Qnil ();
	}

      if ! (NILP (buffer) || EQ (buffer, Qt ()) || FIXNUMP (buffer))
	{
	  let spec_buffer: Lisp_Object = buffer;
	  buffer = Fget_buffer_create (buffer, Qnil ());
	  /* Mention the buffer name for a better error message.  */
	  if NILP (buffer)
	    { CHECK_BUFFER (spec_buffer); }
	  CHECK_BUFFER (buffer);
	}
    }

  /* Make sure that the child will be able to chdir to the current
     buffer's current directory, or its unhandled equivalent.  We
     can't just have the child check for an error when it does the
     chdir, since it's in a vfork.  */
  current_dir = get_current_directory (true);

  if STRINGP (error_file)
    {
      error_file = remove_slash_colon (error_file);
      error_file = ENCODE_FILE (error_file);
    }
  if STRINGP (output_file)
    {
      output_file = remove_slash_colon (output_file);
      output_file = ENCODE_FILE (output_file);
    }

  display_p = INTERACTIVE! () && nargs >= 4 && ! NILP (*args.offset (3));

  i = 0;
  while i < CALLPROC_FDS as ptrdiff_t
    { callproc_fd[i as usize] = -1; i += 1; }
  /* C 514-516's `#ifdef MSDOS` arm.  */
  #[cfg (feature = "msdos")]
  { synch_process_tempfile = make_fixnum (0); }
  record_unwind_protect_ptr (Some (call_process_kill),
			     callproc_fd.as_mut_ptr () as *mut c_void);

  /* Search for program; barf if not found.  */
  {
    let ok: c_int;

    ok = openp (Vexec_path, *args, Fdefault_value (Qexec_suffixes ()), &raw mut path,
		make_fixnum (X_OK as EMACS_INT), false, false, std::ptr::null_mut ());
    if ok < 0
      { report_file_error (c"Searching for program".as_ptr (), *args); }
  }

  /* Remove "/:" from PATH.  */
  path = remove_slash_colon (path);

  SAFE_NALLOCA! (sa_avail, new_argv, 1, if nargs < 4 { 2 } else { nargs - 2 });

  if nargs > 4
    {
      let mut i: ptrdiff_t = 0;

      /* C: `argument_coding.dst_multibyte = 0;` — coding.rs packs the field into `bits`
	 (CODING_SYSTEM_DST_MULTIBYTE_SHIFT, PORTING.md 1.3).  */
      argument_coding.bits &= ! (1 << CODING_SYSTEM_DST_MULTIBYTE_SHIFT);
      i = 4;
      while i < nargs
	{
	  /* C: `argument_coding.src_multibyte = STRING_MULTIBYTE (args[i]);` */
	  argument_coding.bits =
	    (argument_coding.bits & ! (1 << CODING_SYSTEM_SRC_MULTIBYTE_SHIFT))
	    | ((STRING_MULTIBYTE (*args.offset (i as isize)) as u32)
	       << CODING_SYSTEM_SRC_MULTIBYTE_SHIFT);
	  if CODING_REQUIRE_ENCODING (&raw mut argument_coding) != 0
	    /* We must encode this argument.  */
	    { *args.offset (i as isize)
		= encode_coding_string (&raw mut argument_coding, *args.offset (i as isize), true); }
	  i += 1;
	}
      i = 4;
      while i < nargs
	{ *new_argv.offset ((i - 3) as isize) = SSDATA (*args.offset (i as isize)); i += 1; }
      *new_argv.offset ((i - 3) as isize) = std::ptr::null_mut ();
    }
  else
    { *new_argv.offset (1) = std::ptr::null_mut (); }
  path = ENCODE_FILE (path);
  *new_argv = SSDATA (path);

  discard_output = FIXNUMP (buffer) || (NILP (buffer) && NILP (output_file));

  /* C 557-627's `#ifdef MSDOS` arm.  */
  #[cfg (feature = "msdos")]
  {
    if ! discard_output && ! STRINGP (output_file)
      {
	let tmpdir: *const c_char = egetenv (c"TMPDIR".as_ptr ());
	let outf: *const c_char = if ! tmpdir.is_null () { tmpdir } else { c"".as_ptr () };
	tempfile = alloca (strlen (outf) + 20) as *mut c_char;
	strcpy (tempfile, outf);
	dostounix_filename (tempfile);
	if *tempfile == 0 || *tempfile.offset (strlen (tempfile) as isize - 1) != b'/' as c_char
	  { strcat (tempfile, c"/".as_ptr ()); }
	strcat (tempfile, c"emXXXXXX".as_ptr ());
	mktemp (tempfile);
	if *tempfile == 0
	  { report_file_error (c"Opening process output file".as_ptr (), Qnil ()); }
	output_file = build_string (tempfile);
	synch_process_tempfile = output_file;
      }
  }

  if discard_output
    {
      fd_output = emacs_open (NULL_DEVICE.as_ptr (), O_WRONLY, 0);
      if fd_output < 0
	{ report_file_error (c"Opening null device".as_ptr (), Qnil ()); }
    }
  else if STRINGP (output_file)
    {
      fd_output = emacs_open (SSDATA (output_file),
			      O_WRONLY | O_CREAT | O_TRUNC | O_TEXT,
			      default_output_mode as c_int);
      if fd_output < 0
	{
	  let open_errno: c_int = *errno ();
	  output_file = DECODE_FILE (output_file);
	  report_file_errno (c"Opening process output file".as_ptr (),
			     output_file, open_errno);
	}
    }
  else
    {
      let mut fd: [c_int; 2] = [0; 2];
      if emacs_pipe (fd.as_mut_ptr ()) != 0
	{ report_file_error (c"Creating process pipe".as_ptr (), Qnil ()); }
      callproc_fd[CALLPROC_PIPEREAD as usize] = fd[0];
      fd_output = fd[1];
    }
  callproc_fd[CALLPROC_STDOUT as usize] = fd_output;

  fd_error = fd_output;

  if STRINGP (error_file) || (NILP (error_file) && ! discard_output)
    {
      fd_error = emacs_open ((if STRINGP (error_file)
			      { SSDATA (error_file) }
			      else
			      { NULL_DEVICE.as_ptr () }),
			     O_WRONLY | O_CREAT | O_TRUNC | O_TEXT,
			     default_output_mode as c_int);
      if fd_error < 0
	{
	  let open_errno: c_int = *errno ();
	  report_file_errno (c"Cannot redirect stderr".as_ptr (),
			     (if STRINGP (error_file)
			      { DECODE_FILE (error_file) }
			      else
			      { build_string (NULL_DEVICE.as_ptr ()) }),
			     open_errno);
	}
      callproc_fd[CALLPROC_STDERR as usize] = fd_error;
    }

  let env: *mut *mut c_char = make_environment_block (current_dir);

  /* C 628-664's `#ifdef MSDOS` arm (its C comment names MW, July 1993).  */
  #[cfg (feature = "msdos")]
  {
    status = child_setup (filefd, fd_output, fd_error, new_argv, env,
			  SSDATA (current_dir));

    if status < 0
      {
	child_errno = *errno ();
	unbind_to (count, Qnil ());
	synchronize_system_messages_locale ();
	return code_convert_string_norecord (build_string (strerror (child_errno)),
					     Vlocale_coding_system, false);
      }

    { let mut i: c_int = 0;
      while i < CALLPROC_FDS
	{
	  if 0 <= callproc_fd[i as usize]
	    {
	      emacs_close (callproc_fd[i as usize]);
	      callproc_fd[i as usize] = -1;
	    }
	  i += 1;
	} }
    emacs_close (filefd);
    clear_unwind_protect (specpdl_ref_add (count, -1));

    if ! tempfile.is_null ()
      {
	/* Since CRLF is converted to LF within `decode_coding', we
	   can always open a file with binary mode.  */
	callproc_fd[CALLPROC_PIPEREAD as usize] = emacs_open (tempfile, O_RDONLY, 0);
	if callproc_fd[CALLPROC_PIPEREAD as usize] < 0
	  {
	    let open_errno: c_int = *errno ();
	    report_file_errno (c"Cannot re-open temporary file".as_ptr (),
			       build_string (tempfile), open_errno);
	  }
      }
  }

  /* Do the unwind-protect now, even though the pid is not known, so
     that no storage allocation is done in the critical section.
     The actual PID will be filled in during the critical section.  */
  record_unwind_protect (Some (call_process_cleanup), Fcurrent_buffer ());

  /* C 671-717's `#ifndef MSDOS` arm.  */

  child_signal_init ();
  block_input ();
  block_child_signal (&raw mut oldset);

  child_errno
    = emacs_spawn (&raw mut pid, filefd, fd_output, fd_error, new_argv, env,
                   SSDATA (current_dir), std::ptr::null (), false, false,
		   &raw const oldset);
  eassert! ((child_errno == 0) == (0 < pid));

  if pid > 0
    {
      synch_process_pid = pid;

      if FIXNUMP (buffer)
	{
	  if ! specpdl_ref_valid_p (tempfile_index)
	    { record_deleted_pid (pid, Qnil ()); }
	  else
	    {
	      eassert! (1 < nargs);
	      record_deleted_pid (pid, *args.offset (1));
	      clear_unwind_protect (tempfile_index);
	    }
	  synch_process_pid = 0;
	}
    }

  unblock_child_signal (&raw const oldset);
  unblock_input ();

  if pid < 0
    { report_file_errno (CHILD_SETUP_ERROR_DESC.as_ptr () as *const c_char, Qnil (),
		       child_errno); }

  /* Close our file descriptors, except for callproc_fd[CALLPROC_PIPEREAD]
     since we will use that to read input from.  */
  { let mut i: c_int = 0;
    while i < CALLPROC_FDS
      {
	if i != CALLPROC_PIPEREAD && 0 <= callproc_fd[i as usize]
	  {
	    emacs_close (callproc_fd[i as usize]);
	    callproc_fd[i as usize] = -1;
	  }
	i += 1;
      } }
  emacs_close (filefd);
  clear_unwind_protect (specpdl_ref_add (count, -1));

  if FIXNUMP (buffer)
    { return unbind_to (count, Qnil ()); }

  if BUFFERP (buffer)
    { Fset_buffer (buffer); }

  fd0 = callproc_fd[CALLPROC_PIPEREAD as usize];

  if 0 <= fd0
    {
      let mut val: Lisp_Object;
      /* PORTING.md 1.5: C leaves ARGS2 uninitialized; SAFE_NALLOCA passes it by value.  */
      let mut args2: *mut Lisp_Object = std::ptr::null_mut ();

      val = Qnil ();
      if ! NILP (Vcoding_system_for_read)
	{ val = Vcoding_system_for_read; }
      else
	{
	  if EQ (coding_systems, Qt ())
	    {
	      let mut i: ptrdiff_t = 0;

	      SAFE_NALLOCA! (sa_avail, args2, 1, nargs + 1);
	      *args2 = Qcall_process ();
	      while i < nargs
		{ *args2.offset ((i + 1) as isize) = *args.offset (i as isize); i += 1; }
	      coding_systems
		= Ffind_operation_coding_system (nargs + 1, args2);
	    }
	  if CONSP (coding_systems)
	    { val = XCAR (coding_systems); }
	  else if CONSP (Vdefault_process_coding_system)
	    { val = XCAR (Vdefault_process_coding_system); }
	  else
	    { val = Qnil (); }
	}
      Fcheck_coding_system (val);
      /* In unibyte mode, character code conversion should not take
	 place but EOL conversion should.  So, setup raw-text or one
	 of the subsidiary according to the information just setup.  */
      if NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	  && ! NILP (val)
	{ val = raw_text_coding_system (val); }
      setup_coding_system (val, &raw mut process_coding);
      /* C: `process_coding.dst_multibyte
	 = ! NILP (BVAR (current_buffer, enable_multibyte_characters));` */
      if ! NILP (BVAR! (current_buffer, enable_multibyte_characters_))
	{ process_coding.bits |= 1 << CODING_SYSTEM_DST_MULTIBYTE_SHIFT; }
      else
	{ process_coding.bits &= ! (1 << CODING_SYSTEM_DST_MULTIBYTE_SHIFT); }
      process_coding.bits &= ! (1 << CODING_SYSTEM_SRC_MULTIBYTE_SHIFT);
    }

  if 0 <= fd0
    {
      const CALLPROC_BUFFER_SIZE_MIN: c_int = 16 * 1024;
      const CALLPROC_BUFFER_SIZE_MAX: c_int = 4 * CALLPROC_BUFFER_SIZE_MIN;
      /* PORTING.md 1.5: C declares the buffer uninitialized.  */
      let mut buf: [u8; CALLPROC_BUFFER_SIZE_MAX as usize] =
	[0; CALLPROC_BUFFER_SIZE_MAX as usize];
      let mut bufsize: c_int = CALLPROC_BUFFER_SIZE_MIN;
      let mut nread: c_int;
      let mut total_read: EMACS_INT = 0;
      let mut carryover: c_int = 0;
      let mut display_on_the_fly: bool = display_p;
      /* C's `struct coding_system saved_coding = process_coding;` is a byte copy; the mirror
	 does not derive `Copy` for `coding_system` (a mirror gap for the integrator), so the
	 port spells C's struct assignment as the byte copies it is.  */
      let mut saved_coding: coding_system = std::ptr::read (&process_coding);
      let mut prepared_pos: ptrdiff_t = 0; /* prepare_to_modify_buffer was last
					      called here.  */

      'give_up: loop
	{
	  /* Repeatedly read until we've filled as much as possible
	     of the buffer size we have.  But don't read
	     less than 1024--save that for the next bufferful.  */
	  nread = carryover;
	  while nread < bufsize - 1024
	    {
	      let this_read: c_int
		= emacs_read_quit (fd0, buf.as_mut_ptr ().offset (nread as isize) as *mut c_void,
				   (bufsize - nread) as ptrdiff_t) as c_int;

	      if this_read < 0
		{ break 'give_up; }

	      if this_read == 0
		{
		  /* C: `process_coding.mode |= CODING_MODE_LAST_BLOCK;` */
		  process_coding.bits =
		    (process_coding.bits
		     & ! (CODING_SYSTEM_MODE_MASK << CODING_SYSTEM_MODE_SHIFT))
		    | ((CODING_MODE_LAST_BLOCK as u32) << CODING_SYSTEM_MODE_SHIFT);
		  break;
		}

	      nread += this_read;
	      total_read += this_read as EMACS_INT;

	      if display_on_the_fly
		{ break; }
	    }
          /* CHANGE FUNCTIONS
             For each iteration of the enclosing while (1) loop which
             yields data (i.e. nread > 0), before- and
             after-change-functions are each invoked exactly once.
             This is done directly from the current function only, by
             calling prepare_to_modify_buffer and signal_after_change.
             It is not done here by directing another function such as
             insert_1_both to call them.  The call to
             prepare_to_modify_buffer follows this comment, and there
             is one call to signal_after_change in each of the
             branches of the next `else if'.

             Exceptionally, the insertion into the buffer is aborted
             at the call to del_range_2 ~45 lines further down, this
             function removing the newly inserted data.  At this stage
             prepare_to_modify_buffer has been called, but
             signal_after_change hasn't.  A continue statement
             restarts the enclosing while (1) loop.  A second,
             unwanted, call to `prepare_to_modify_buffer' is inhibited
	     by the test prepared_pos < PT.  The data are inserted
             again, and this time signal_after_change gets called,
             balancing the previous call to prepare_to_modify_buffer.  */
          if prepared_pos < PT! () && nread != 0
            {
              prepare_to_modify_buffer (PT! (), PT! (), std::ptr::null_mut ());
              prepared_pos = PT! ();
            }

	  /* Now NREAD is the total amount of data in the buffer.  */

	  if nread == 0
	    { }
	  else if NILP (BVAR! (current_buffer, enable_multibyte_characters_))
		   && CODING_MAY_REQUIRE_DECODING (&raw mut process_coding) == 0
            {
              insert_1_both (buf.as_ptr () as *const c_char, nread as ptrdiff_t,
			     nread as ptrdiff_t, false, false, false);
              signal_after_change (PT! () - nread as ptrdiff_t, 0, nread as ptrdiff_t);
            }
	  else
	    {			/* We have to decode the input.  */
	      let curbuf: Lisp_Object;
	      let count1: specpdl_ref = SPECPDL_INDEX ();

	      XSETBUFFER! (curbuf, current_buffer);
	      /* We cannot allow after-change-functions be run
		 during decoding, because that might modify the
		 buffer, while we rely on process_coding.produced to
		 faithfully reflect inserted text until we
		 TEMP_SET_PT_BOTH below.  */
	      specbind (Qinhibit_modification_hooks (), Qt ());
	      decode_coding_c_string (&raw mut process_coding,
				      buf.as_mut_ptr (), nread as ptrdiff_t, curbuf);
	      unbind_to (count1, Qnil ());
	      if display_on_the_fly
		  && CODING_REQUIRE_DETECTION (&raw mut saved_coding) != 0
		  && CODING_REQUIRE_DETECTION (&raw mut process_coding) == 0
		{
		  /* We have detected some coding system, but the
		     detection may have been via insufficient data.
		     So give up displaying on the fly.  */
		  if process_coding.produced > 0
		    { del_range_2 (process_coding.dst_pos,
				   process_coding.dst_pos_byte,
				   (process_coding.dst_pos
				    + process_coding.produced_char),
				   (process_coding.dst_pos_byte
				    + process_coding.produced),
				   false); }
		  display_on_the_fly = false;
		  process_coding = std::ptr::read (&saved_coding);
		  carryover = nread;
		  /* Make the above condition always fail in the future.  */
		  saved_coding.bits &= ! CODING_REQUIRE_DETECTION_MASK;
		  continue 'give_up;
		}

	      TEMP_SET_PT_BOTH (PT! () + process_coding.produced_char,
				PT_BYTE! () + process_coding.produced);
              signal_after_change (PT! () - process_coding.produced_char,
                                   0, process_coding.produced_char);
	      carryover = process_coding.carryover_bytes;
	      if carryover > 0
		{ std::ptr::copy_nonoverlapping (process_coding.carryover.as_ptr (),
						 buf.as_mut_ptr (), carryover as usize); }
	    }

	  if (process_coding.bits >> CODING_SYSTEM_MODE_SHIFT) & CODING_SYSTEM_MODE_MASK
	     & CODING_MODE_LAST_BLOCK as u32 != 0
	    { break; }

	  /* Make the buffer bigger as we continue to read more data,
	     but not past CALLPROC_BUFFER_SIZE_MAX.  */
	  if bufsize < CALLPROC_BUFFER_SIZE_MAX && total_read > 32 * bufsize as EMACS_INT
	    {
	      bufsize *= 2;
	      if bufsize > CALLPROC_BUFFER_SIZE_MAX
		{ bufsize = CALLPROC_BUFFER_SIZE_MAX; }
	    }

	  if display_p
	    {
	      redisplay_preserve_echo_area (1);
	      /* This variable might have been set to 0 for code
		 detection.  In that case, set it back to 1 because
		 we should have already detected a coding system.  */
	      display_on_the_fly = true;
	    }
	}
      /* give_up: ; */

      Vlast_coding_system_used = CODING_ID_NAME (process_coding.id as hash_idx_t);
      /* If the caller required, let the buffer inherit the
	 coding-system used to decode the process output.  */
      if inherit_process_coding_system
	{ calln! (Qafter_insert_file_set_buffer_file_coding_system (),
		  make_fixnum (total_read)); }
    }

  let mut wait_ok: bool = true;
  /* Wait for it to terminate, unless it already has.  */
  wait_ok = wait_for_termination (pid, &raw mut status, fd0 < 0);

  /* Don't kill any children that the subprocess may have left behind
     when exiting.  */
  synch_process_pid = 0;

  SAFE_FREE_UNBIND_TO! (sa_count, count, Qnil ());

  if ! wait_ok
    { return build_unibyte_string (c"internal error".as_ptr ()); }

  if WIFSIGNALED (status)
    {
      let mut signame: *const c_char;

      synchronize_system_messages_locale ();
      signame = strsignal (WTERMSIG (status));

      if signame.is_null ()
	{ signame = c"unknown".as_ptr (); }

      return code_convert_string_norecord (build_string (signame),
					   Vlocale_coding_system, false);
    }

  eassert! (WIFEXITED (status));
  return make_fixnum (WEXITSTATUS (status) as EMACS_INT);
}

/* Create a temporary file suitable for storing the input data of
   call-process-region.  NARGS and ARGS are the same as for
   call-process-region.  Store into *FILENAME_STRING_PTR a Lisp string
   naming the file, and return a file descriptor for reading.
   Unwind-protect the file, so that the file descriptor will be closed
   and the file removed when the caller unwinds the specpdl stack.  */

unsafe fn create_temp_file (nargs: ptrdiff_t, args: *mut Lisp_Object,
			    filename_string_ptr: *mut Lisp_Object) -> c_int
{
  let fd: c_int;
  let filename_string: Lisp_Object;
  let mut val: Lisp_Object;
  let start: Lisp_Object;
  let end: Lisp_Object;
  let mut tmpdir: Lisp_Object;
  let mut outf: *const c_char;

  if STRINGP (Vtemporary_file_directory)
    { tmpdir = Vtemporary_file_directory; }
  else
    {
      outf = getenv (c"TMPDIR".as_ptr ());
      tmpdir = build_string (if ! outf.is_null () { outf } else { c"/tmp/".as_ptr () });
    }
  /* C 978-985's DOS_NT-side `#else` arm.  */
  #[cfg (feature = "dos_nt")]
  {
    if { outf = egetenv (c"TMPDIR".as_ptr ()); ! outf.is_null () }
	|| { outf = egetenv (c"TMP".as_ptr ()); ! outf.is_null () }
	|| { outf = egetenv (c"TEMP".as_ptr ()); ! outf.is_null () }
      { tmpdir = build_string (outf); }
    else
      { tmpdir = Ffile_name_as_directory (build_string (c"c:/temp".as_ptr ())); }
  }

  {
    let mut pattern: Lisp_Object = Fexpand_file_name (Vtemp_file_name_pattern, tmpdir);
    let tempfile: *mut c_char;

    /* C 992-1005's `#ifdef WINDOWSNT` arm.  */
    #[cfg (feature = "windowsnt")]
    {
      /* Cannot use the result of Fexpand_file_name, because it
	 downcases the XXXXXX part of the pattern, and mktemp then
	 doesn't recognize it.  */
      if ! NILP (Vw32_downcase_file_names)
	{
	  let dirname: Lisp_Object = Ffile_name_directory (pattern);

	  if NILP (dirname)
	    { pattern = Vtemp_file_name_pattern; }
	  else
	    { pattern = concat2 (dirname, Vtemp_file_name_pattern); }
	}
    }

    filename_string = Fcopy_sequence (ENCODE_FILE (pattern));
    tempfile = SSDATA (filename_string);

    let count: specpdl_ref = SPECPDL_INDEX ();
    record_unwind_protect_nothing ();
    fd = mkostemp (tempfile, O_BINARY | O_CLOEXEC);
    if fd < 0
      { report_file_error (c"Failed to open temporary file using pattern".as_ptr (),
			   pattern); }
    set_unwind_protect (count, Some (delete_temp_file), filename_string);
    record_unwind_protect_int (Some (close_file_unwind), fd);
  }

  start = *args;
  end = *args.offset (1);
  /* Decide coding-system of the contents of the temporary file.  */
  if ! NILP (Vcoding_system_for_write)
    { val = Vcoding_system_for_write; }
  else if NILP (BVAR! (current_buffer, enable_multibyte_characters_))
    { val = Qraw_text (); }
  else
    {
      let coding_systems: Lisp_Object;
      /* PORTING.md 1.5: C leaves ARGS2 uninitialized; SAFE_NALLOCA passes it by value.  */
      let mut args2: *mut Lisp_Object = std::ptr::null_mut ();
      USE_SAFE_ALLOCA! (sa_avail, sa_count);
      SAFE_NALLOCA! (sa_avail, args2, 1, nargs + 1);
      *args2 = Qcall_process_region ();
      std::ptr::copy_nonoverlapping (args, args2.offset (1),
				     (nargs * std::mem::size_of::<Lisp_Object> () as ptrdiff_t) as usize);
      coding_systems = Ffind_operation_coding_system (nargs + 1, args2);
      val = if CONSP (coding_systems) { XCDR (coding_systems) } else { Qnil () };
      SAFE_FREE! (sa_count);
    }
  val = complement_process_encoding_system (val);

  {
    let count1: specpdl_ref = SPECPDL_INDEX ();

    specbind (Qcoding_system_for_write (), val);
    /* POSIX lets mk[s]temp use "."; don't invoke jka-compr if we
       happen to get a ".Z" suffix.  */
    specbind (Qfile_name_handler_alist (), Qnil ());
    write_region (start, end, filename_string, Qnil (), Qlambda (), Qnil (), Qnil (),
		  fd);

    unbind_to (count1, Qnil ());
  }

  if lseek (fd, 0, SEEK_SET) < 0
    { report_file_error (c"Setting file position".as_ptr (), filename_string); }

  /* Note that Fcall_process takes care of binding
     coding-system-for-read.  */

  *filename_string_ptr = filename_string;
  return fd;
}

DEFUN! ("call-process-region", Fcall_process_region, Scall_process_region, 3, MANY, 0,
       doc: "Send text from START to END to a synchronous process running PROGRAM.\n\nSTART and END are normally buffer positions specifying the part of the\nbuffer to send to the process.\nIf START is nil, that means to use the entire buffer contents; END is\nignored.\nIf START is a string, then send that string to the process\ninstead of any buffer contents; END is ignored.\nThe remaining arguments are optional.\nDelete the text if fourth arg DELETE is non-nil.\n\nInsert output in BUFFER before point; t means current buffer; nil for\n BUFFER means discard it; 0 means discard and don't wait; and `(:file\n FILE)', where FILE is a file name string, means that it should be\n written to that file (if the file already exists it is overwritten).\nBUFFER can be a string which is the name of a buffer.\nBUFFER can also have the form (REAL-BUFFER STDERR-FILE); in that case,\nREAL-BUFFER says what to do with standard output, as above,\nwhile STDERR-FILE says what to do with standard error in the child.\nSTDERR-FILE may be nil (discard standard error output),\nt (mix it with ordinary output), or a file name string.\n\nSixth arg DISPLAY non-nil means redisplay buffer as output is inserted.\nRemaining arguments ARGS are passed to PROGRAM at startup as command-line\narguments.\n\nIf PROGRAM is not an absolute file name, `call-process-region' will\nlook for PROGRAM in `exec-path' (which is a list of directories).\n\nIf BUFFER is 0, `call-process-region' returns immediately with value nil.\nOtherwise it waits for PROGRAM to terminate\nand returns a numeric exit status or a signal description string.\nIf you quit, the process is killed with SIGINT, or SIGKILL if you quit again.\n\nusage: (call-process-region START END PROGRAM &optional DELETE BUFFER DISPLAY &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  /* PORTING.md 1.5: C leaves INFILE uninitialized (create_temp_file fills it).  */
  let mut infile: Lisp_Object = Qnil ();
  let val: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut start: Lisp_Object = *args;
  let mut end: Lisp_Object = *args.offset (1);
  let empty_input: bool;
  let fd: c_int;

  if STRINGP (start)
    { empty_input = SCHARS (start) == 0; }
  else if NILP (start)
    { empty_input = BEG as ptrdiff_t == Z! (); }
  else
    {
      validate_region (args, args.offset (1));
      start = *args;
      end = *args.offset (1);
      empty_input = XFIXNUM (start) == XFIXNUM (end);
    }

  if ! empty_input
    { fd = create_temp_file (nargs, args, &raw mut infile); }
  else
    {
      infile = Qnil ();
      fd = emacs_open (NULL_DEVICE.as_ptr (), O_RDONLY, 0);
      if fd < 0
	{ report_file_error (c"Opening null device".as_ptr (), Qnil ()); }
      record_unwind_protect_int (Some (close_file_unwind), fd);
    }

  if nargs > 3 && ! NILP (*args.offset (3))
    {
      if NILP (start)
        {
          /* No need to save restrictions since we delete everything
             anyway.  */
          labeled_restrictions_remove_in_current_buffer ();
          Fwiden ();
          del_range (BEG as ptrdiff_t, Z! ());
        }
      else
        { Fdelete_region (start, end); }
    }

  let mut args: *mut Lisp_Object = args;
  let mut nargs: isize = nargs;
  if nargs > 3
    {
      args = args.offset (2);
      nargs -= 2;
    }
  else
    {
      *args = *args.offset (2);
      nargs = 2;
    }
  *args.offset (1) = infile;

  val = call_process (nargs, args, fd,
		      if empty_input { make_invalid_specpdl_ref () } else { count });
  return unbind_to (count, val);
});

unsafe fn add_env (env: *mut *mut c_char, new_env: *mut *mut c_char,
		   string: *mut c_char) -> *mut *mut c_char
{
  let mut ep: *mut *mut c_char;
  let mut ok: bool = true;
  let mut new_env: *mut *mut c_char = new_env;
  if string.is_null ()
    { return new_env; }

  /* See if this string duplicates any string already in the env.
     If so, don't put it in.
     When an env var has multiple definitions,
     we keep the definition that comes first in process-environment.  */
  ep = env;
  while ok && ep != new_env
    {
      let mut p: *mut c_char = *ep;
      let mut q: *mut c_char = string;
      while ok
	{
	  if *p != 0 && *q != *p
	    { break; }
	  if *q == 0
	    /* The string is a lone variable name; keep it for now, we
	       will remove it later.  It is a placeholder for a
	       variable that is not to be included in the environment.  */
	    { break; }
	  if *q == b'=' as c_char
	    { ok = false; }
	  p = p.offset (1);
	  q = q.offset (1);
	}
      ep = ep.offset (1);
    }
  if ok
    { *new_env = string; new_env = new_env.offset (1); }
  return new_env;
}

/* 'exec' failed inside a child running NAME, with error number ERR.
   Possibly a vforked child needed to allocate a large vector on the
   stack; such a child cannot fall back on malloc because that might
   mess up the allocator's data structures in the parent.
   Report the error and exit the child.  */

unsafe fn exec_failed (name: *const c_char, err: c_int) -> !
{
  /* Avoid deadlock if the child's perror writes to a full pipe; the
     pipe's reader is the parent, but with vfork the parent can't
     run until the child exits.  Truncate the diagnostic instead.  */
  fcntl (STDERR_FILENO, F_SETFL, O_NONBLOCK);

  *errno () = err;
  emacs_perror (name);
  _exit (if err == ENOENT { EXIT_ENOENT } else { EXIT_CANNOT_INVOKE });
}

/* This is the last thing run in a newly forked inferior
   either synchronous or asynchronous.
   Copy descriptors IN, OUT and ERR as descriptors 0, 1 and 2.
   Initialize inferior's priority, pgrp, connected dir and environment.
   then exec another program based on new_argv.

   CURRENT_DIR is an elisp string giving the path of the current
   directory the subprocess should have.  Since we can't really signal
   a decent error from within the child, this should be verified as an
   executable directory by the parent.

   On GNUish hosts, either exec or return an error number.
   On MS-Windows, either return a pid or return -1 and set errno.
   On MS-DOS, either return an exit status or signal an error.  */

unsafe fn child_setup (in_: c_int, out: c_int, err: c_int, new_argv: *mut *mut c_char,
		       env: *mut *mut c_char, current_dir: *const c_char) -> !
{
  /* C 1239-1243's `#ifdef MSDOS` arm.  */
  #[cfg (feature = "msdos")]
  let mut pwd_var: *mut c_char;
  #[cfg (feature = "msdos")]
  let mut temp: *mut c_char;
  #[cfg (feature = "msdos")]
  let mut i: ptrdiff_t = 0;
  /* C 1244-1249's `#ifdef WINDOWSNT` arm.  */
  #[cfg (feature = "windowsnt")]
  type HANDLE = *mut c_void;
  #[cfg (feature = "windowsnt")]
  let mut cpid: c_int;
  #[cfg (feature = "windowsnt")]
  let mut handles: [HANDLE; 3] = [std::ptr::null_mut (); 3];
  /* PORTING.md 1.5: C declares `pid_t pid = getpid ();`.  */
  let mut pid: pid_t = getpid ();

  /* Note that use of alloca is always safe here.  It's obvious for systems
     that do not have true vfork or that have true (stack) alloca.
     If using vfork and C_ALLOCA (when Emacs used to include
     src/alloca.c) it is safe because that changes the superior's
     static variables as if the superior had done alloca and will be
     cleaned up in the usual way. */

  /* We can't signal an Elisp error here; we're in a vfork.  Since
     the callers check the current directory before forking, this
     should only return an error if the directory's permissions
     are changed between the check and this chdir, but we should
     at least check.  */
  if chdir (current_dir) < 0
    { _exit (EXIT_CANCELED); }

  /* C 1268-1275's `#ifdef WINDOWSNT` arm.  */
  #[cfg (feature = "windowsnt")]
  {
    prepare_standard_handles (in_, out, err, handles.as_mut_ptr ());
    set_process_dir (current_dir);
    /* Spawn the child.  (See w32proc.c:sys_spawnve).  */
    cpid = spawnve (_P_NOWAIT, *new_argv, new_argv, env);
    reset_standard_handles (in_, out, err, handles.as_mut_ptr ());
    return cpid as i32;
  }

  /* C 1276-1319's not-WINDOWSNT-side `#else` arm.  */

  restore_nofile_limit ();

  /* Redirect file descriptors and clear the close-on-exec flag on the
     redirected ones.  IN, OUT, and ERR are close-on-exec so they
     need not be closed explicitly.  */
  dup2 (in_, STDIN_FILENO);
  dup2 (out, STDOUT_FILENO);
  dup2 (err, STDERR_FILENO);

  setpgid (0, 0);
  tcsetpgrp (0, pid);

  let errnum: c_int = emacs_exec_file (*new_argv, new_argv, env);
  exec_failed (*new_argv, errnum);

  /* C 1295-1317's MSDOS-side `#else` arm.  */
  #[cfg (feature = "msdos")]
  {
    i = strlen (current_dir) as ptrdiff_t;
    pwd_var = xmalloc (i as usize + 5) as *mut c_char;
    temp = pwd_var.offset (4);
    std::ptr::copy_nonoverlapping (c"PWD=".as_ptr (), pwd_var, 4);
    stpcpy (temp, current_dir);

    if i > 2 && IS_DEVICE_SEP (*temp.offset (1)) && IS_DIRECTORY_SEP (*temp.offset (2))
      {
	temp = temp.offset (2);
	i -= 2;
      }

    /* Strip trailing slashes for PWD, but leave "/" and "//" alone.  */
    while i > 2 && IS_DIRECTORY_SEP (*temp.offset (i - 1))
      { i -= 1; *temp.offset (i) = 0; }

    pid = run_msdos_command (new_argv, pwd_var.offset (4), in_, out, err, env);
    xfree (pwd_var as *mut c_void);
    if pid == -1
      /* An error occurred while trying to run the subprocess.  */
      { report_file_error (c"Spawning child process".as_ptr (), Qnil ()); }
    return pid;
  }
}

/* Set up ACTIONS and ATTRIBUTES for `posix_spawn'.  Return an error
   number.  */

unsafe fn emacs_posix_spawn_init_actions (actions: *mut posix_spawn_file_actions_t,
					  std_in: c_int, std_out: c_int, std_err: c_int,
					  cwd: *const c_char) -> c_int
{
  let mut error: c_int = posix_spawn_file_actions_init (actions);
  if error != 0
    { return error; }

  'out: loop
    {
      error = posix_spawn_file_actions_adddup2 (actions, std_in, STDIN_FILENO);
      if error != 0
	{ break 'out; }

      error = posix_spawn_file_actions_adddup2 (actions, std_out, STDOUT_FILENO);
      if error != 0
	{ break 'out; }

      error = posix_spawn_file_actions_adddup2 (actions,
						if std_err < 0 { std_out } else { std_err },
						STDERR_FILENO);
      if error != 0
	{ break 'out; }

      /* Haiku appears to have linkable posix_spawn_file_actions_chdir,
	 but it always fails.  So use the _np function instead.  */
      #[cfg (feature = "no_posix_spawn_addchdir")]
      { error = posix_spawn_file_actions_addchdir_np (actions, cwd); }
      #[cfg (not (feature = "no_posix_spawn_addchdir"))]
      { error = posix_spawn_file_actions_addchdir (actions, cwd); }
      if error != 0
	{ break 'out; }
      break 'out;
    }

  if error != 0
    { posix_spawn_file_actions_destroy (actions); }
  return error;
}

unsafe fn emacs_posix_spawn_init_attributes (attributes: *mut posix_spawnattr_t,
					     oldset: *const sigset_t) -> c_int
{
  let mut error: c_int = posix_spawnattr_init (attributes);
  if error != 0
    { return error; }

  'out: loop
    {
      error = posix_spawnattr_setflags (attributes,
					POSIX_SPAWN_SETSID
					| POSIX_SPAWN_SETSIGDEF
					| POSIX_SPAWN_SETSIGMASK);
      if error != 0
	{ break 'out; }

      /* PORTING.md 1.5: C declares `sigset_t sigdefault;` uninitialized.  */
      let mut sigdefault: sigset_t = std::mem::zeroed ();
      sigemptyset (&raw mut sigdefault);

      /* C 1387-1393's `#ifdef DARWIN_OS` arm (live: DARWIN_OS is defined here).  */
      /* Work around a macOS bug, where SIGCHLD is apparently
	 delivered to a vforked child instead of to its parent.  See:
	 https://lists.gnu.org/r/emacs-devel/2017-05/msg00342.html
      */
      sigaddset (&raw mut sigdefault, SIGCHLD);

      sigaddset (&raw mut sigdefault, SIGINT);
      sigaddset (&raw mut sigdefault, SIGQUIT);
      /* C 1397-1399's `#ifdef SIGPROF` arm (live: SIGPROF is 27).  */
      sigaddset (&raw mut sigdefault, SIGPROF);

      /* Emacs ignores SIGPIPE, but the child should not.  */
      sigaddset (&raw mut sigdefault, SIGPIPE);
      /* Likewise for SIGPROF.  */
      sigaddset (&raw mut sigdefault, SIGPROF);

      error = posix_spawnattr_setsigdefault (attributes, &raw const sigdefault);
      if error != 0
	{ break 'out; }

      /* Stop blocking SIGCHLD in the child.  */
      error = posix_spawnattr_setsigmask (attributes, oldset);
      if error != 0
	{ break 'out; }
      break 'out;
    }

  if error != 0
    { posix_spawnattr_destroy (attributes); }

  return error;
}

/* Start a new asynchronous subprocess.  If successful, return zero
   and store the process identifier of the new process in *NEWPID.
   Use STDIN, STDOUT, and STDERR as standard streams for the new
   process.  Use ARGV as argument vector for the new process; use
   process image file ARGV[0].  Use ENVP for the environment block for
   the new process.  Use CWD as working directory for the new process.
   If PTY is not NULL, it must be a pseudoterminal device.  If PTY is
   NULL, don't perform any terminal setup.  OLDSET must be a pointer
   to a signal set initialized by `block_child_signal'.  Before
   calling this function, call `block_input' and `block_child_signal';
   afterwards, call `unblock_input' and `unblock_child_signal'.  Be
   sure to call `unblock_child_signal' only after registering NEWPID
   in a list where `handle_child_signal' can find it!  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn emacs_spawn (mut newpid: *mut pid_t, mut std_in: c_int,
				      mut std_out: c_int, mut std_err: c_int,
				      mut argv: *mut *mut c_char, mut envp: *mut *mut c_char,
				      mut cwd: *const c_char, mut pty_name: *const c_char,
				      mut pty_in: bool, mut pty_out: bool,
				      mut oldset: *const sigset_t) -> c_int
{
  /* C 1446-1455's `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` arm.  */
  #[cfg (feature = "have_android")]
  {
    /* Android 10 and later don't allow directly executing programs
       installed in the application data directory.  Emacs provides a
       loader binary which replaces the `execve' system call for it and
       all its children.  On these systems, rewrite the command line to
       call that loader binary instead.  */

    if android_rewrite_spawn_argv (&raw mut argv as *mut *mut *const c_char)
      { return 1; }
  }

  /* Prefer the simpler `posix_spawn' if available.  `posix_spawn'
     doesn't yet support setting up pseudoterminals, so we fall back
     to `vfork' if we're supposed to use a pseudoterminal.  */

  let use_posix_spawn: bool = pty_name.is_null ();

  /* PORTING.md 1.5: Darwin's two spawn types are pointers, so C's
     uninitialized declarations start at null.  */
  let mut actions: posix_spawn_file_actions_t = std::ptr::null_mut ();
  let mut attributes: posix_spawnattr_t = std::ptr::null_mut ();

  if use_posix_spawn
    {
      /* Initialize optional attributes before blocking. */
      let mut error: c_int = emacs_posix_spawn_init_actions (&raw mut actions, std_in,
							     std_out, std_err, cwd);
      if error != 0
	{ return error; }

      error = emacs_posix_spawn_init_attributes (&raw mut attributes, oldset);
      if error != 0
	{ return error; }
    }

  /* PORTING.md 1.5: C declares both uninitialized.  */
  let mut pid: c_int = 0;
  let mut vfork_error: c_int;

  eassert! (input_blocked_p ());

  'fork_done: loop
    {
      if use_posix_spawn
	{
	  vfork_error = posix_spawn (&raw mut pid, *argv, &raw const actions,
				     &raw const attributes, argv, envp);
	  if vfork_error != 0
	    { pid = -1; }

	  let mut error: c_int = posix_spawn_file_actions_destroy (&raw mut actions);
	  if error != 0
	    {
	      *errno () = error;
	      emacs_perror (c"posix_spawn_file_actions_destroy".as_ptr ());
	    }

	  error = posix_spawnattr_destroy (&raw mut attributes);
	  if error != 0
	    {
	      *errno () = error;
	      emacs_perror (c"posix_spawnattr_destroy".as_ptr ());
	    }

	  break 'fork_done;
	}

      /* vfork, and prevent local vars from being clobbered by the vfork.  The C
	 `volatile` qualifiers have no Rust counterpart; the names and the
	 saving/restoring statements are kept (a recorded deviation).  */
      let mut newpid_volatile: *mut pid_t = newpid;
      let mut cwd_volatile: *const c_char = cwd;
      let mut ptyname_volatile: *const c_char = pty_name;
      let mut ptyin_volatile: bool = pty_in;
      let mut ptyout_volatile: bool = pty_out;
      let mut argv_volatile: *mut *mut c_char = argv;
      let mut stdin_volatile: c_int = std_in;
      let mut stdout_volatile: c_int = std_out;
      let mut stderr_volatile: c_int = std_err;
      let mut envp_volatile: *mut *mut c_char = envp;
      let mut oldset_volatile: *const sigset_t = oldset;

      /* C 1527-1536's DARWIN_OS arm is live here; its `#else` is carried.  */
      #[cfg (not (feature = "no_darwin_os"))]
      {
	/* Darwin doesn't let us run setsid after a vfork, so use fork when
	   necessary.  Below, we reset SIGCHLD handling after a vfork, as
	   apparently macOS can mistakenly deliver SIGCHLD to the child.  */
	if pty_in || pty_out
	  { pid = fork (); }
	else
	  { pid = VFORK (); }
      }
      #[cfg (feature = "no_darwin_os")]
      { pid = vfork (); }

      newpid = newpid_volatile;
      cwd = cwd_volatile;
      pty_name = ptyname_volatile;
      pty_in = ptyin_volatile;
      pty_out = ptyout_volatile;
      argv = argv_volatile;
      std_in = stdin_volatile;
      std_out = stdout_volatile;
      std_err = stderr_volatile;
      envp = envp_volatile;
      oldset = oldset_volatile;

      if pid == 0
	{
	  /* Make the pty be the controlling terminal of the process.  */
	  dissociate_controlling_tty ();

	  /* Make the pty's terminal the controlling terminal.  */
	  if pty_in && std_in >= 0
	    {
	      /* C 1561-1563's `#ifdef TIOCSCTTY`: the frozen build compiles it (the macro
		 comes from `<sys/ioctl.h>`; see the file header).  We ignore the return
		 value because faith@cs.unc.edu says that is necessary on Linux.  */
	      ioctl (std_in, TIOCSCTTY, 0);
	    }
	  /* C 1567-1575's `#if defined (LDISC1)` arm.  */
	  #[cfg (feature = "ldisc1")]
	  {
	    if pty_in && std_in >= 0
	      {
		let mut t: termios = std::mem::zeroed ();
		tcgetattr (std_in, &raw mut t);
		t.c_lflag = LDISC1;
		if tcsetattr (std_in, TCSANOW, &raw const t) < 0
		  { emacs_perror (c"create_process/tcsetattr LDISC1".as_ptr ()); }
	      }
	  }
	  /* C 1577-1583's `#else` / `#if defined (NTTYDISC) && defined (TIOCSETD)` arm.  */
	  #[cfg (feature = "nttydisc")]
	  {
	    if pty_in && std_in >= 0
	      {
		/* Use new line discipline.  */
		let ldisc: c_int = NTTYDISC;
		ioctl (std_in, TIOCSETD, &raw const ldisc);
	      }
	  }

	  /* C 1587-1613's `#if !defined (DONT_REOPEN_PTY)` arm.  */
	  #[cfg (feature = "no_dont_reopen_pty")]
	  {
	    /* Now close the pty (if we had it open) and reopen it.
	       This makes the pty the controlling terminal of the subprocess.  */
	    if ! pty_name.is_null ()
	      {
		/* I wonder if emacs_close (emacs_open (pty, ...))
		   would work?  */
		if pty_in && std_in >= 0
		  { emacs_close (std_in); }
		let ptyfd: c_int = emacs_open_noquit (pty_name, O_RDWR, 0);
		if pty_in
		  { std_in = ptyfd; }
		if pty_out
		  { std_out = ptyfd; }
		if std_in < 0
		  {
		    emacs_perror (pty_name);
		    _exit (EXIT_CANCELED);
		  }
	      }
	  }

	  /* C 1616-1621's `#ifdef SETUP_SLAVE_PTY` arm.  */
	  #[cfg (feature = "setup_slave_pty")]
	  { if pty_in && std_in >= 0 { SETUP_SLAVE_PTY; } }

	  /* C 1624-1630's `#ifdef DARWIN_OS` arm (live here).  */
	  /* Work around a macOS bug, where SIGCHLD is apparently
	     delivered to a vforked child instead of to its parent.  See:
	     https://lists.gnu.org/r/emacs-devel/2017-05/msg00342.html
	  */
	  signal (SIGCHLD, SIG_DFL);

	  signal (SIGINT, SIG_DFL);
	  signal (SIGQUIT, SIG_DFL);
	  signal (SIGPROF, SIG_DFL);

	  /* Emacs ignores SIGPIPE, but the child should not.  */
	  signal (SIGPIPE, SIG_DFL);
	  /* Likewise for SIGPROF.  */
	  signal (SIGPROF, SIG_DFL);

	  /* Stop blocking SIGCHLD in the child.  */
	  unblock_child_signal (oldset);

	  if pty_out
	    { child_setup_tty (std_out); }

	  if std_err < 0
	    { std_err = std_out; }
	  /* C 1655-1659's `#ifdef WINDOWSNT` arm.  */
	  #[cfg (feature = "windowsnt")]
	  { pid = child_setup (std_in, std_out, std_err, argv, envp, cwd); }
	  #[cfg (not (feature = "windowsnt"))]
	  { child_setup (std_in, std_out, std_err, argv, envp, cwd); }
	}

      /* Back in the parent process.  */

      vfork_error = if pid < 0 { *errno () } else { 0 };

      break 'fork_done;
    }

  if pid < 0
    {
      eassert! (0 < vfork_error);
      return vfork_error;
    }

  eassert! (0 < pid);
  *newpid = pid;
  return 0;
}

unsafe fn getenv_internal_1 (var: *const c_char, varlen: ptrdiff_t,
			     value: *mut *mut c_char, valuelen: *mut ptrdiff_t,
			     mut env: Lisp_Object) -> bool
{
  while CONSP (env)
    {
      let entry: Lisp_Object = XCAR (env);
      if STRINGP (entry)
	  && SBYTES (entry) >= varlen
	  && {
	       /* C 1689-1694's `#ifdef WINDOWSNT` arm (`strnicmp`) is carried; the frozen
		  build's comparison is the `#else`'s `memcmp`.  */
	       #[cfg (feature = "windowsnt")]
	       { strnicmp (SSDATA (entry), var, varlen) == 0 }
	       #[cfg (not (feature = "windowsnt"))]
	       { memcmp (SDATA (entry) as *const c_void, var as *const c_void,
			 varlen as usize) == 0 }
	     }
	{
	  if SBYTES (entry) > varlen && SREF (entry, varlen as usize) == b'=' as u8
	    {
	      *value = SSDATA (entry).offset (varlen + 1);
	      *valuelen = SBYTES (entry) - (varlen + 1);
	      return true;
	    }
	  else if SBYTES (entry) == varlen
	    {
	      /* Lone variable names in Vprocess_environment mean that
		 variable should be removed from the environment. */
	      *value = std::ptr::null_mut ();
	      return true;
	    }
	}
      env = XCDR (env);
    }
  return false;
}

unsafe fn getenv_internal (var: *const c_char, varlen: ptrdiff_t,
			   value: *mut *mut c_char, valuelen: *mut ptrdiff_t,
			   frame: Lisp_Object) -> bool
{
  /* Try to find VAR in Vprocess_environment first.  */
  if getenv_internal_1 (var, varlen, value, valuelen,
			Vprocess_environment)
    { return ! (*value).is_null (); }

  /* C 1726-1735's `#ifdef WINDOWSNT` arm: on that target we make some modifications to
     Emacs's environment without recording them in Vprocess_environment.  */
  #[cfg (feature = "windowsnt")]
  {
    let tmpval: *mut c_char = getenv (var);
    if ! tmpval.is_null ()
      {
        *value = tmpval;
        *valuelen = strlen (tmpval) as ptrdiff_t;
        return true;
      }
  }

  /* Setting DISPLAY under Android hinders attempts to display other
     programs within X servers that are available for Android.  */

  /* C 1741-1760's `#ifndef HAVE_ANDROID` arm (live: HAVE_ANDROID is undefined).  */
  /* For DISPLAY try to get the values from the frame or the initial env.  */
  if strcmp (var, c"DISPLAY".as_ptr ()) == 0
    {
      /* C 1745-1754's `#ifndef HAVE_PGTK` arm (live: HAVE_PGTK is undefined).  */
      let display: Lisp_Object
	= Fframe_parameter (if NILP (frame) { selected_frame } else { frame }, Qdisplay ());
      if STRINGP (display)
	{
	  *value    = SSDATA (display);
	  *valuelen = SBYTES (display);
	  return true;
	}
      /* If still not found, Look for DISPLAY in Vinitial_environment.  */
      if getenv_internal_1 (var, varlen, value, valuelen,
			    Vinitial_environment)
	{ return ! (*value).is_null (); }
    }

  return false;
}

DEFUN! ("getenv-internal", Fgetenv_internal, Sgetenv_internal, 1, 2, 0,
       doc: "Get the value of environment variable VARIABLE.\nVARIABLE should be a string.  Value is nil if VARIABLE is undefined in\nthe environment.  Otherwise, value is a string.\n\nThis function searches `process-environment' for VARIABLE.\n\nIf optional parameter ENV is a list, then search this list instead of\n`process-environment', and return t when encountering a negative entry\n(an entry for a variable with no value).",
       (variable: Lisp_Object, env: Lisp_Object),
       {
  /* PORTING.md 1.5: C declares both uninitialized.  */
  let mut value: *mut c_char = std::ptr::null_mut ();
  let mut valuelen: ptrdiff_t = 0;

  CHECK_STRING (variable);
  if CONSP (env)
    {
      if getenv_internal_1 (SSDATA (variable), SBYTES (variable),
			    &raw mut value, &raw mut valuelen, env)
	{ return if ! value.is_null () { make_string (value, valuelen) } else { Qt () }; }
      else
	{ return Qnil (); }
    }
  else if getenv_internal (SSDATA (variable), SBYTES (variable),
			   &raw mut value, &raw mut valuelen, env)
    { return make_string (value, valuelen); }
  else
    { return Qnil (); }
});

/* A version of getenv that consults the Lisp environment lists,
   easily callable from C.  This is usually called from egetenv.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn egetenv_internal (var: *const c_char, len: ptrdiff_t) -> *mut c_char
{
  let mut value: *mut c_char = std::ptr::null_mut ();
  let mut valuelen: ptrdiff_t = 0;

  if getenv_internal (var, len, &raw mut value, &raw mut valuelen, Qnil ())
    { return value; }
  else
    { return std::ptr::null_mut (); }
}

/* Create a new environment block.  You can pass the returned pointer
   to `execve'.  Add unwind protections for all newly-allocated
   objects.  Don't call any Lisp code or the garbage collector while
   the block is active.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn make_environment_block (current_dir: Lisp_Object) -> *mut *mut c_char
{
  let env: *mut *mut c_char;
  let pwd_var: *mut c_char;

  {
    let mut temp: *mut c_char;
    let mut i: ptrdiff_t;

    i = SBYTES (current_dir);
    pwd_var = xmalloc (i as usize + 5) as *mut c_char;
    record_unwind_protect_ptr (Some (xfree), pwd_var as *mut c_void);
    temp = pwd_var.offset (4);
    std::ptr::copy_nonoverlapping (c"PWD=".as_ptr (), pwd_var, 4);
    lispstpcpy (temp, current_dir);

    /* C 1832-1838's `#ifdef DOS_NT` arm.  */
    #[cfg (feature = "dos_nt")]
    {
      /* Get past the drive letter, so that d:/ is left alone.  */
      if i > 2 && IS_DEVICE_SEP (*temp.offset (1)) && IS_DIRECTORY_SEP (*temp.offset (2))
	{
	  temp = temp.offset (2);
	  i -= 2;
	}
    }

    /* Strip trailing slashes for PWD, but leave "/" and "//" alone.  */
    while i > 2 && IS_DIRECTORY_SEP (*temp.offset (i - 1))
      { i -= 1; *temp.offset (i) = 0; }
  }

  /* Set `env' to a vector of the strings in the environment.  */

  {
    let mut tem: Lisp_Object;
    let mut new_env: *mut *mut c_char;
    let mut p: *mut *mut c_char;
    let mut q: *mut *mut c_char;
    let mut new_length: c_int;
    let mut display: Lisp_Object = Qnil ();

    new_length = 0;

    tem = Vprocess_environment;
    while CONSP (tem) && STRINGP (XCAR (tem))
      {
	/* C 1863-1869's `#ifndef HAVE_ANDROID` arm (live).  */
	if strncmp (SSDATA (XCAR (tem)), c"DISPLAY".as_ptr (), 7) == 0
	    && (*SDATA (XCAR (tem)).offset (7) == 0
		|| *SDATA (XCAR (tem)).offset (7) == b'=' as u8)
	  /* DISPLAY is specified in process-environment.  */
	  { display = Qt (); }
	new_length += 1;
	tem = XCDR (tem);
      }

    /* Setting DISPLAY under Android hinders attempts to display other
       programs within X servers that are available for Android.  */

    /* C 1876-1903's `#ifndef HAVE_ANDROID` arm (live).  */
    /* If not provided yet, use the frame's DISPLAY.  */
    if NILP (display)
      {
	let mut tmp: Lisp_Object = Fframe_parameter (selected_frame, Qdisplay ());

	/* C 1882-1890's `#ifdef HAVE_PGTK` arm.  */
	#[cfg (feature = "have_pgtk")]
	{
	  /* The only time GDK actually returns correct information is
	     when it's running under X Windows.  DISPLAY shouldn't be
	     set to a Wayland display either, since that's an X specific
	     variable.  */
	  if FRAME_WINDOW_P (SELECTED_FRAME ())
	      && strcmp (G_OBJECT_TYPE_NAME (FRAME_X_DISPLAY (SELECTED_FRAME ())),
			 c"GdkX11Display".as_ptr ()) != 0
	    { tmp = Qnil (); }
	}

	if ! STRINGP (tmp) && CONSP (Vinitial_environment)
	  /* If still not found, Look for DISPLAY in Vinitial_environment.  */
	  { tmp = Fgetenv_internal (build_string (c"DISPLAY".as_ptr ()),
				    Vinitial_environment); }
	if STRINGP (tmp)
	  {
	    display = tmp;
	    new_length += 1;
	  }
      }

    /* new_length + 2 to include PWD and terminating 0.  */
    env = xnmalloc (new_length as isize + 2,
		    std::mem::size_of::<*mut c_char> () as isize) as *mut *mut c_char;
    new_env = env;
    record_unwind_protect_ptr (Some (xfree), env as *mut c_void);
    /* If we have a PWD envvar, pass one down,
       but with corrected value.  */
    if ! egetenv (c"PWD".as_ptr ()).is_null ()
      { *new_env = pwd_var; new_env = new_env.offset (1); }

    /* C 1913-1921's `#ifndef HAVE_ANDROID` arm (live).  */
    if STRINGP (display)
      {
	let vdata: *mut c_char
	  = xmalloc ((std::mem::size_of::<[u8; 9]> () as ptrdiff_t   /* C: sizeof "DISPLAY=" */
		      + SBYTES (display)) as usize) as *mut c_char;
	record_unwind_protect_ptr (Some (xfree), vdata as *mut c_void);
	lispstpcpy (stpcpy (vdata, c"DISPLAY=".as_ptr ()), display);
	new_env = add_env (env, new_env, vdata);
      }

    /* Overrides.  */
    tem = Vprocess_environment;
    while CONSP (tem) && STRINGP (XCAR (tem))
      {
	new_env = add_env (env, new_env, SSDATA (XCAR (tem)));
	tem = XCDR (tem);
      }

    *new_env = std::ptr::null_mut ();

    /* Remove variable names without values.  */
    p = env;
    q = env;
    while ! (*p).is_null ()
      {
	while ! (*q).is_null () && strchr (*q, b'=' as c_int).is_null ()
	  { q = q.offset (1); }
	*p = *q;
	q = q.offset (1);
	if ! (*p).is_null ()
	  { p = p.offset (1); }
      }
  }

  return env;
}

/* This is run before init_cmdargs.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_callproc_1 ()
{
  Vdata_directory = decode_env_path (c"EMACSDATA".as_ptr (), PATH_DATA.as_ptr (), false);
  Vdata_directory = Ffile_name_as_directory (Fcar (Vdata_directory));

  Vdoc_directory = decode_env_path (c"EMACSDOC".as_ptr (), PATH_DOC.as_ptr (), false);
  Vdoc_directory = Ffile_name_as_directory (Fcar (Vdoc_directory));

  /* Check the EMACSPATH environment variable, defaulting to the
     PATH_EXEC path from epaths.h.  */
  Vexec_path = decode_env_path (c"EMACSPATH".as_ptr (), PATH_EXEC.as_ptr (), false);
  Vexec_directory = Ffile_name_as_directory (Fcar (Vexec_path));
  /* FIXME?  For ns, path_exec should go at the front?  */
  Vexec_path = nconc2 (decode_env_path (c"PATH".as_ptr (), std::ptr::null (), false),
		       Vexec_path);
}

/* This is run after init_cmdargs, when Vinstallation_directory is valid.  */

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_callproc ()
{
  let data_dir: bool = ! egetenv (c"EMACSDATA".as_ptr ()).is_null ();

  let sh: *const c_char;
  /* PORTING.md 1.5: C declares TEMPDIR uninitialized; it is assigned twice below.  */
  let mut tempdir: Lisp_Object = Qnil ();

  if ! NILP (Vinstallation_directory)
    {
      /* Add to the path the lib-src subdir of the installation dir.  */
      let mut tem: Lisp_Object;
      tem = Fexpand_file_name (build_string (c"lib-src".as_ptr ()),
			       Vinstallation_directory);
      /* C 1982-1992's `#ifndef MSDOS` arm (live).
	 MSDOS uses wrapped binaries, so don't do this.  */
      if NILP (Fmember (tem, Vexec_path))
	{
	  /* Running uninstalled, so default to tem rather than PATH_EXEC.  */
	  Vexec_path = decode_env_path (c"EMACSPATH".as_ptr (), SSDATA (tem), false);
	  Vexec_path = nconc2 (decode_env_path (c"PATH".as_ptr (), std::ptr::null (), false),
			       Vexec_path);
	}

      Vexec_directory = Ffile_name_as_directory (tem);

      /* Maybe use ../etc as well as ../lib-src.  */
      if ! data_dir
	{
	  tem = Fexpand_file_name (build_string (c"etc".as_ptr ()),
				   Vinstallation_directory);
	  Vdoc_directory = Ffile_name_as_directory (tem);
	}
    }

  /* Look for the files that should be in etc.  We don't use
     Vinstallation_directory, because these files are never installed
     near the executable, and they are never in the build
     directory when that's different from the source directory.

     Instead, if these files are not in the nominal place, we try the
     source directory.  */
  if ! data_dir
    {
      let lispdir: Lisp_Object
	= Fcar (decode_env_path (std::ptr::null (), PATH_DUMPLOADSEARCH.as_ptr (), false));
      if ! NILP (Vinstallation_directory)
	  || ! NILP (Fequal (Fexpand_file_name (AUTO_STR! (c"../src/".as_ptr ()), lispdir),
			     Vinvocation_directory))
	  || NILP (Ffile_exists_p (Fexpand_file_name (AUTO_STR! (c"NEWS".as_ptr ()),
						      Vdata_directory)))
	{
	  let mut newdir: Lisp_Object;
	  newdir = Fexpand_file_name (AUTO_STR! (c"../etc/".as_ptr ()), lispdir);
	  if ! NILP (Ffile_exists_p (Fexpand_file_name (AUTO_STR! (c"NEWS".as_ptr ()),
							newdir)))
	    { Vdata_directory = newdir; }
	}
    }

  if ! will_dump_p ()
    {
      tempdir = Fdirectory_file_name (Vexec_directory);
      if ! file_accessible_directory_p (tempdir)
	{ dir_warning (c"arch-dependent data dir".as_ptr (), Vexec_directory); }
    }

  tempdir = Fdirectory_file_name (Vdata_directory);
  if ! file_accessible_directory_p (tempdir)
    { dir_warning (c"arch-independent data dir".as_ptr (), Vdata_directory); }

  sh = getenv (c"SHELL".as_ptr ());
  /* C 2039-2043's `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY` arm.  */
  #[cfg (feature = "have_android")]
  {
    /* The Android shell is found under /system/bin, not /bin.  */
    Vshell_file_name = build_string (if ! sh.is_null () { sh } else { c"/system/bin/sh".as_ptr () });
  }
  #[cfg (not (feature = "have_android"))]
  { Vshell_file_name = build_string (if ! sh.is_null () { sh } else { c"/bin/sh".as_ptr () }); }

  let mut gamedir: Lisp_Object = Qnil ();
  if ! PATH_GAME.is_null ()
    {
      let mut cpath_game: *const c_char = PATH_GAME;
      /* C 2050-2053's `#ifdef WINDOWSNT` arm.  */
      #[cfg (feature = "windowsnt")]
      {
	/* On MS-Windows, PATH_GAME normally starts with a literal
	   "%emacs_dir%", so it will never work without some tweaking.  */
	cpath_game = w32_relocate (cpath_game);
      }
      let path_game: Lisp_Object = build_unibyte_string (cpath_game);
      if file_accessible_directory_p (path_game)
	{ gamedir = path_game; }
      else if *errno () != ENOENT && *errno () != ENOTDIR
	       && {
		    /* C 2059-2061's `#ifdef DOS_NT` arm: DOS/Windows sometimes return
		       EACCES for bad file names.  */
		    #[cfg (feature = "dos_nt")]
		    { *errno () != EACCES }
		    #[cfg (not (feature = "dos_nt"))]
		    { true }
		  }
	{ dir_warning (c"game dir".as_ptr (), path_game); }
    }
  Vshared_game_score_directory = gamedir;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_initial_environment ()
{
  /* PORTING.md 1.5: C declares ENVP uninitialized; the loop assigns it first.  */
  let mut envp: *mut *mut c_char = std::ptr::null_mut ();
  envp = environ;
  while ! (*envp).is_null ()
    {
      Vprocess_environment = Fcons (build_string (*envp),
				    Vprocess_environment);
      envp = envp.offset (1);
    }
  /* Ideally, the `copy' shouldn't be necessary, but it seems it's frequent
     to use `delete' and friends on process-environment.  */
  Vinitial_environment = Fcopy_sequence (Vprocess_environment);
}
/* The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
   (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
   port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
   defines it here (status section 3.29, the casetab.rs shape).  */
#[unsafe(no_mangle)]
pub static mut Vshell_file_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vexec_path: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vexec_suffixes: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vexec_directory: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vdata_directory: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vdoc_directory: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vconfigure_info_directory: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vshared_game_score_directory: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vinitial_environment: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vprocess_environment: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vctags_program_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vetags_program_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vhexl_program_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vemacsclient_program_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vmovemail_program_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vebrowse_program_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vrcs2log_program_name: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut command_line_max_length: EMACS_INT = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syms_of_callproc ()
{
  /* C 2084-2088's `#ifndef DOS_NT` arm (live: DOS_NT is undefined).  */
  Vtemp_file_name_pattern = build_string (c"emacsXXXXXX".as_ptr ());
  #[cfg (feature = "dos_nt")]
  { Vtemp_file_name_pattern = build_string (c"emXXXXXX".as_ptr ()); }
  staticpro (&raw const Vtemp_file_name_pattern);

  /* C 2091-2094's `#ifdef MSDOS` arm.  */
  #[cfg (feature = "msdos")]
  {
    synch_process_tempfile = make_fixnum (0);
    staticpro (&raw const synch_process_tempfile);
  }

  DEFVAR_LISP! (c"shell-file-name", Vshell_file_name,
	       "File name to load inferior shells from.\nInitialized from the SHELL environment variable, or to a system-dependent\ndefault if SHELL is unset.  See Info node `(elisp)Security Considerations'.");

  DEFVAR_LISP! (c"exec-path", Vexec_path,
	       "List of directories to search programs to run in subprocesses.\nEach element is a string (directory name) or nil (try default directory).\n\nBy default the last element of this list is `exec-directory'. The\nlast element is not always used, for example in shell completion\n(`shell-dynamic-complete-command').");

  DEFVAR_LISP! (c"exec-suffixes", Vexec_suffixes,
	       "List of suffixes to try to find executable file names.\nEach element is a string.");
  Vexec_suffixes = Qnil ();

  DEFVAR_LISP! (c"exec-directory", Vexec_directory,
	       "Directory for executables for Emacs to invoke.\nMore generally, this includes any architecture-dependent files\nthat are built and installed from the Emacs distribution.");

  DEFVAR_LISP! (c"data-directory", Vdata_directory,
	       "Directory of machine-independent files that come with GNU Emacs.\nThese are files intended for Emacs to use while it runs.");

  DEFVAR_LISP! (c"doc-directory", Vdoc_directory,
	       "Directory containing the DOC file that comes with GNU Emacs.\nThis is usually the same as `data-directory'.");

  DEFVAR_LISP! (c"configure-info-directory", Vconfigure_info_directory,
	       "For internal use by the build procedure only.\nThis is the name of the directory in which the build procedure installed\nEmacs's info files; the default value for `Info-default-directory-list'\nincludes this.");
  Vconfigure_info_directory = build_string (PATH_INFO.as_ptr ());

  DEFVAR_LISP! (c"shared-game-score-directory", Vshared_game_score_directory,
	       "Directory of score files for games which come with GNU Emacs.\nIf this variable is nil, then Emacs is unable to use a shared directory.");

  DEFVAR_LISP! (c"initial-environment", Vinitial_environment,
	       "List of environment variables inherited from the parent process.\nEach element should be a string of the form ENVVARNAME=VALUE.\nThe elements must normally be decoded (using `locale-coding-system') for use.");
  Vinitial_environment = Qnil ();

  DEFVAR_LISP! (c"process-environment", Vprocess_environment,
	       "List of overridden environment variables for subprocesses to inherit.\nEach element should be a string of the form ENVVARNAME=VALUE.\n\nEntries in this list take precedence to those in the frame-local\nenvironments.  Therefore, let-binding `process-environment' is an easy\nway to temporarily change the value of an environment variable,\nirrespective of where it comes from.  To use `process-environment' to\nremove an environment variable, include only its name in the list,\nwithout \"=VALUE\".\n\nThis variable is set to nil when Emacs starts.\n\nIf multiple entries define the same variable, the first one always\ntakes precedence.\n\nNon-ASCII characters are encoded according to the initial value of\n`locale-coding-system', i.e. the elements must normally be decoded for\nuse.\n\nSee `setenv' and `getenv'.");
  Vprocess_environment = Qnil ();

  DEFVAR_LISP! (c"ctags-program-name", Vctags_program_name,
	       "Name of the `ctags' program.\nUse this instead of calling `ctags' directly, as `ctags' may have been\nrenamed to comply with executable naming restrictions on the system.");

  DEFVAR_LISP! (c"etags-program-name", Vetags_program_name,
	       "Name of the `etags' program distributed with Emacs.\nUse this instead of calling `etags' directly, as `etags' may have been\nrenamed to comply with executable naming restrictions on the system.");
  /* C 2177-2180 android arm.  */
  Vetags_program_name = build_string (c"etags".as_ptr ());
  #[cfg (feature = "have_android")]
  { Vetags_program_name = build_string (c"libetags.so".as_ptr ()); }

  DEFVAR_LISP! (c"hexl-program-name", Vhexl_program_name,
	       "Name of the `hexl' program distributed with Emacs.\nUse this instead of calling `hexl' directly, as `hexl' may have been\nrenamed to comply with executable naming restrictions on the system.");
  /* C 2187-2190 android arm.  */
  Vhexl_program_name = build_string (c"hexl".as_ptr ());
  #[cfg (feature = "have_android")]
  { Vhexl_program_name = build_string (c"libhexl.so".as_ptr ()); }

  DEFVAR_LISP! (c"emacsclient-program-name", Vemacsclient_program_name,
	       "Name of the `emacsclient' program distributed with Emacs.\nUse this instead of calling `emacsclient' directly, as `emacsclient'\nmay have been renamed to comply with executable naming restrictions on\nthe system.");
  /* C 2198-2201 android arm.  */
  Vemacsclient_program_name = build_string (c"emacsclient".as_ptr ());
  #[cfg (feature = "have_android")]
  { Vemacsclient_program_name = build_string (c"libemacsclient.so".as_ptr ()); }

  DEFVAR_LISP! (c"movemail-program-name", Vmovemail_program_name,
	       "Name of the `movemail' program distributed with Emacs.\nUse this instead of calling `movemail' directly, as `movemail'\nmay have been renamed to comply with executable naming restrictions on\nthe system.");
  /* C 2211-2215 android arm.  */
  Vmovemail_program_name = build_string (c"movemail".as_ptr ());
  #[cfg (feature = "have_android")]
  { Vmovemail_program_name = build_string (c"libmovemail.so".as_ptr ()); }

  DEFVAR_LISP! (c"ebrowse-program-name", Vebrowse_program_name,
	       "Name of the `ebrowse' program distributed with Emacs.\nUse this instead of calling `ebrowse' directly, as `ebrowse'\nmay have been renamed to comply with executable naming restrictions on\nthe system.");
  /* C 2223-2226 android arm.  */
  Vebrowse_program_name = build_string (c"ebrowse".as_ptr ());
  #[cfg (feature = "have_android")]
  { Vebrowse_program_name = build_string (c"libebrowse.so".as_ptr ()); }

  DEFVAR_LISP! (c"rcs2log-program-name", Vrcs2log_program_name,
	       "Name of the `rcs2log' program distributed with Emacs.\nUse this instead of calling `rcs2log' directly, as `rcs2log'\nmay have been renamed to comply with executable naming restrictions on\nthe system.");
  /* C 2234-2237 android arm.  */
  Vrcs2log_program_name = build_string (c"rcs2log".as_ptr ());
  #[cfg (feature = "have_android")]
  { Vrcs2log_program_name = build_string (c"librcs2log.so".as_ptr ()); }

  DEFVAR_INT! (c"command-line-max-length", command_line_max_length,
    "Maximum length of a command and its arguments on this system.\nThis is measured in characters.\nUsed by `multiple-command-partition-arguments'.  Other code calls that\nfunction for cases in which it's known to be safe to run the command\nmultiple times on subsequent partitions of the list of arguments.\n(In a shell script, you might use the `xargs' utility.)");
  /* Divide it by 4 as a crude way to go bytes->characters.  */
  command_line_max_length = sysconf (_SC_ARG_MAX) as EMACS_INT / 4;
  /* C 2250-2251's `#else` arm (feature `no_sc_arg_max`).  */
  #[cfg (feature = "no_sc_arg_max")]
  { command_line_max_length = 4096; }

  defsubr (&raw mut Scall_process);
  defsubr (&raw mut Sgetenv_internal);
  defsubr (&raw mut Scall_process_region);

  DEFSYM! (Qafter_insert_file_set_buffer_file_coding_system,
	  c"after-insert-file-set-buffer-file-coding-system");
  DEFSYM! (Qcoding_system_for_write, c"coding-system-for-write");
  DEFSYM! (Qexec_suffixes, c"exec-suffixes");
}
