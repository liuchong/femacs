// femacs: mirror of lib/readutmp.h @ fd7afced403.
//
// Header mirror (PORTING.md 0.1, P-26).  The interface of gnulib's read-utmp module: the
// platform-independent `struct gl_utmp`, the `UT_*` accessor macros, the `UTMP_STRUCT_NAME`/
// `SET_UTMP_ENT`/... bindings the low-level file access uses, `UTMP_FILE`/`WTMP_FILE` and the
// two entry points.  Its only frozen consumer is `lib/boot-time.c` (measured: `grep -l` over
// emacs/lib and emacs/src), which declared the names it needs at the use site until now - see
// docs/ports/boot-time.md's "Names declared at the use site" and this mirror's ledger for the
// wiring the unit's owner will do.
//
// **The criterion is measured, because the file is not in the verified list.**  `python3
// tools/check-cond-eval.py lib/readutmp.h` -> `25 conditions, 1 unknown, 6 disagreements [NOT
// VERIFIED]`: the six disagreements are all *system-header* macros the configuration's table
// cannot see (`_PATH_UTMP`, `UTMPX_FILE`, `_PATH_WTMP`, `WTMPX_FILE` and the two
// "already defined?" guards around them), and the one unknown is C 278's
// `(HAVE_UTMPX_H ? HAVE_STRUCT_UTMPX_UT_TYPE : HAVE_STRUCT_UTMP_UT_TYPE)` ternary.  So
// `tools/extract-abi.py` reads the file blind (both arms, P-66) and the live arm per name was
// measured with the frozen clang - `config.h` alone defines *none* of these names, and after
// `readutmp.h` its own live macros and `BOOT_TIME`/`USER_PROCESS`/`LOGIN_PROCESS`/
// `_PATH_*`/`UTMPX_FILE` from the system `<utmpx.h>` survive (command in the ledger):
//
//   * live: `UT_USER`/`UT_TIME_MEMBER`/`UT_PID`/`UT_TYPE_EQ`/`UT_TYPE_NOT_DEFINED`/
//     `UT_EXIT_E_TERMINATION`/`UT_EXIT_E_EXIT` (C 83-89), `UTMP_STRUCT_NAME` `utmpx` (C 172),
//     `SET_UTMP_ENT`/`GET_UTMP_ENT`/`END_UTMP_ENT` (C 174-176), `UTMP_NAME_FUNCTION`
//     `utmpxname` (C 178), `UTMP_FILE` `UTMPX_FILE` (C 242, so the `/etc/utmp` fallback C 245
//     is dead), `WTMP_FILE` `_PATH_WTMP` (C 251), `WTMPDB_FILE` (C 264),
//     `UT_TYPE_BOOT_TIME`/`UT_TYPE_USER_PROCESS`/`UT_TYPE_LOGIN_PROCESS` (C 286/291/296, their
//     `0` arms C 288/293/298 are dead because the system header defines the names),
//     `IS_USER_PROCESS` (C 302) and `READ_UTMP_SUPPORTED` (C 307).
//   * dead, carried below under the feature that names the condition selecting them (P-51):
//     C 50-54's AIX `utmp_data` rename (`_THREAD_SAFE && UTMP_DATA_INIT`), C 136-143's glibc
//     `_GL_UT_*_SIZE` and C 146-168's `struct utmpx32` (`__GLIBC__ && _TIME_BITS == 64`),
//     C 170's `UTMP_STRUCT_NAME utmpx32`, C 187-218's whole `#elif HAVE_UTMP_H` arm
//     (`HAVE_UTMPX_H` is 1, so `utmp`/`setutent`/`getutent`/`endutent`/`utmpname` are dead),
//     C 180/184's other `UTMP_NAME_FUNCTION` arms (`UTXDB_ACTIVE`, `__ANDROID__`), C 238/245's
//     `UTMP_FILE` alternatives, C 255/258's `WTMP_FILE` alternatives, C 270-273's Android
//     block and C 278-282's "no `ut_type` field" block (`HAVE_STRUCT_UTMPX_UT_TYPE` is 1; the
//     evaluator cannot decide its ternary, clang says false), and C 38-41's AIX workaround
//     (`HAVE_STRUCT_UTMP_UT_EXIT` is undef, so it is not taken).
//
// `struct gl_utmp`'s layout is the frozen target's, measured with clang and pinned by the
// `const _` assertions below: size 72, align 8, `ut_user` 0, `ut_id` 8, `ut_line` 16,
// `ut_host` 24, `ut_ts` 32, `ut_pid` 48, `ut_session` 52, `ut_type` 56, `ut_exit` 60 (the
// assertions below are the compile-time half; the probe that printed them is in the ledger).
// The anonymous `struct { int e_termination; int e_exit; }` member C 79 spells is named
// `gl_utmp_ut_exit` (Rust has no anonymous aggregate member), which is the only shape this
// file invents.
//
// This mirror takes its scalars and the three port types from the including unit (PORTING.md
// 0.1-9: a mirror must not carry a `use`), exactly as C 30-34's own includes do: `idx.rs`,
// `sys_types.in.rs` (`pid_t`) and `time.in.rs` (`struct timespec`) are included below, and
// `c_char`/`c_int`/`c_long`/`c_short` come from the unit's `use` line - the shape
// `include-lib-malloc-scratch_buffer.md` records for a mirror that is not a crate root.

// [U1-MODULE-MIRROR]
use super::*;
// [U1] idx.rs imported via super
// [U1] sys_types.in.rs imported via super
// [U1] time.in.rs imported via super

/* C 20-21: `#ifndef __READUTMP_H__ / #define __READUTMP_H__` - the guard has no Rust
   counterpart (PORTING.md 0.1-4); the name stays as its marker.  */
pub const __READUTMP_H__: bool = true;

/* C 26-28: the `_GL_CONFIG_H_INCLUDED` requirement (`#error "Please include config.h
   first."`) is what the includer's `config.rs` satisfies; C 44-46's `<utmp.h>` and C 49-56's
   `<utmpx.h>` are the system headers whose frozen values the port gets from
   `include/utmpx.rs`, which the unit includes in their place.  */

/* C 38-41: `#if (HAVE_UTMPX_H && HAVE_UTMP_H && HAVE_STRUCT_UTMP_UT_EXIT
   && ! HAVE_STRUCT_UTMPX_UT_EXIT) # undef HAVE_UTMPX_H` - the AIX 4.3.3 workaround.  Dead:
   `HAVE_STRUCT_UTMP_UT_EXIT` is `undef` (macros.tsv 1602) and `HAVE_STRUCT_UTMPX_UT_EXIT` too
   (1592), so the conjunction is false and `HAVE_UTMPX_H` (1) stays defined.  */
#[cfg (feature = "utmp_h_ut_exit_no_utmpx_ut_exit")]
macro_rules! HAVE_UTMPX_H { () => {}; }

/* C 65-80: `struct gl_utmp`, "type of entries returned by read_utmp on all platforms".  All
   four `char *` members point to storage with the lifetime of the struct (C 67-68).  */
#[repr(C)]
pub struct gl_utmp
{
  /* char *ut_user;  User name.  */
  pub ut_user: *mut c_char,
  /* char *ut_id;  Session ID.  */
  pub ut_id: *mut c_char,
  /* char *ut_line;  seat / device.  */
  pub ut_line: *mut c_char,
  /* char *ut_host;  for remote sessions: user@host or host, for local sessions: the X11
     display :N.  */
  pub ut_host: *mut c_char,
  /* struct timespec ut_ts;  time.  */
  pub ut_ts: timespec,
  /* pid_t ut_pid;  process ID of ? */
  pub ut_pid: pid_t,
  /* pid_t ut_session;  process ID of session leader.  */
  pub ut_session: pid_t,
  /* short ut_type;  BOOT_TIME, USER_PROCESS, LOGIN_PROCESS, or other.  */
  pub ut_type: c_short,
  /* struct { int e_termination; int e_exit; } ut_exit;  */
  pub ut_exit: gl_utmp_ut_exit,
}

/* C 79's anonymous member, given the name Rust needs for it.  */
#[repr(C)]
pub struct gl_utmp_ut_exit
{
  pub e_termination: c_int,
  pub e_exit: c_int,
}

/* The measured layout (clang -std=gnu23, the probe in the ledger).  `struct gl_utmp` is 72
   bytes with 8-byte alignment, and every member sits where the C compiler puts it.  */
const _: () = core::assert! (std::mem::size_of::<gl_utmp> () == 72);
const _: () = core::assert! (std::mem::align_of::<gl_utmp> () == 8);
const _: () = core::assert! (std::mem::offset_of! (gl_utmp, ut_user) == 0);
const _: () = core::assert! (std::mem::offset_of! (gl_utmp, ut_id) == 8);
const _: () = core::assert! (std::mem::offset_of! (gl_utmp, ut_line) == 16);
const _: () = core::assert! (std::mem::offset_of! (gl_utmp, ut_host) == 24);
const _: () = core::assert! (std::mem::offset_of! (gl_utmp, ut_ts) == 32);
const _: () = core::assert! (std::mem::offset_of! (gl_utmp, ut_pid) == 48);
const _: () = core::assert! (std::mem::offset_of! (gl_utmp, ut_session) == 52);
const _: () = core::assert! (std::mem::offset_of! (gl_utmp, ut_type) == 56);
const _: () = core::assert! (std::mem::offset_of! (gl_utmp, ut_exit) == 60);
const _: () = core::assert! (std::mem::size_of::<gl_utmp_ut_exit> () == 8);

/* C 82-89: the macros describing `struct gl_utmp`.  */
macro_rules! UT_USER { ($ut:expr) => { (*$ut).ut_user } }
macro_rules! UT_TIME_MEMBER { ($ut:expr) => { (*$ut).ut_ts.tv_sec } }
macro_rules! UT_PID { ($ut:expr) => { (*$ut).ut_pid } }
macro_rules! UT_TYPE_EQ { ($ut:expr, $v:expr) => { (*$ut).ut_type == ($v) } }
/* C 87: `#define UT_TYPE_NOT_DEFINED 0` - a constant and a macro at once; the port spells
   the one item that both demand (the value the token check compares is the `0`).  */
pub const UT_TYPE_NOT_DEFINED: c_short = 0;
macro_rules! UT_EXIT_E_TERMINATION { ($ut:expr) => { (*$ut).ut_exit.e_termination } }
macro_rules! UT_EXIT_E_EXIT { ($ut:expr) => { (*$ut).ut_exit.e_exit } }

/* C 91-92: `typedef struct gl_utmp STRUCT_UTMP;` - the name the two entry points and
   `extract_trimmed_name` use.  */
pub type STRUCT_UTMP = gl_utmp;

/* C 94-103: the four "size of the member, or -1 if unbounded" enums.  */
pub const UT_USER_SIZE: c_int = -1;
pub const UT_ID_SIZE: c_int = -1;
pub const UT_LINE_SIZE: c_int = -1;
pub const UT_HOST_SIZE: c_int = -1;

/* C 112-188's `#if HAVE_UTMPX_H` arm, the frozen one.  Its `__GLIBC__ && _TIME_BITS == 64`
   sub-arm (C 136-143) is dead on this target: */
#[cfg (feature = "glibc_time_bits_64")]
macro_rules! _GL_UT_USER_SIZE { () => { sizeof (((struct utmpx *) 0)->ut_user) }; }
#[cfg (feature = "glibc_time_bits_64")]
macro_rules! _GL_UT_ID_SIZE { () => { sizeof (((struct utmpx *) 0)->ut_id) }; }
#[cfg (feature = "glibc_time_bits_64")]
macro_rules! _GL_UT_LINE_SIZE { () => { sizeof (((struct utmpx *) 0)->ut_line) }; }
#[cfg (feature = "glibc_time_bits_64")]
macro_rules! _GL_UT_HOST_SIZE { () => { sizeof (((struct utmpx *) 0)->ut_host) }; }

/* C 146-168: `struct utmpx32`, "a near-copy of glibc's struct utmpx, which stops working
   after the year 2038" - the file format the same dead arm describes.  Its array members are
   sized by the `_GL_UT_*_SIZE` macros above, so the declaration stands only with them.  */
#[cfg (feature = "glibc_time_bits_64")]
#[repr(C)]
pub struct utmpx32
{
  pub ut_type: c_short,
  pub ut_pid: pid_t,
  pub ut_line: [c_char; _GL_UT_LINE_SIZE],
  pub ut_id: [c_char; _GL_UT_ID_SIZE],
  pub ut_user: [c_char; _GL_UT_USER_SIZE],
  pub ut_host: [c_char; _GL_UT_HOST_SIZE],
  pub ut_exit: utmpx32_exit,
  pub ut_session: c_int,
  pub ut_tv: utmpx32_ut_tv,
  pub ut_addr_v6: [c_int; 4],
  pub ut_reserved: [c_char; 20],
}
#[cfg (feature = "glibc_time_bits_64")]
#[repr(C)]
pub struct utmpx32_exit
{
  pub e_termination: c_short,
  pub e_exit: c_short,
}
#[cfg (feature = "glibc_time_bits_64")]
#[repr(C)]
pub struct utmpx32_ut_tv
{
  pub tv_sec: c_uint,
  pub tv_usec: c_int,
}

/* C 169-173: `UTMP_STRUCT_NAME`.  The live arm is C 172's `utmpx`; C 170's `utmpx32` needs
   the dead arm above, and C 210's `utmp` belongs to the dead `#elif HAVE_UTMP_H` branch.
   C spells the name as a macro because C 149's use is a type position; the port's own type
   spellings are the unit's (`lib/boot-time.rs` 73's `pub type UTMP_STRUCT_NAME = utmpx;`),
   which a macro does not collide with (different namespaces).  */
#[cfg (feature = "glibc_time_bits_64")]
macro_rules! UTMP_STRUCT_NAME { () => { utmpx32 }; }
#[cfg (feature = "utmp_h")]
macro_rules! UTMP_STRUCT_NAME { () => { utmp }; }
#[cfg (not (feature = "glibc_time_bits_64"))]
#[cfg (not (feature = "utmp_h"))]
macro_rules! UTMP_STRUCT_NAME { () => { utmpx }; }

/* C 174-176: the three low-level entry points the frozen arm binds to `<utmpx.h>`.  */
macro_rules! SET_UTMP_ENT { () => { setutxent () } }
macro_rules! GET_UTMP_ENT { () => { getutxent () } }
macro_rules! END_UTMP_ENT { () => { endutxent () } }

/* C 177-185: `UTMP_NAME_FUNCTION`.  Live: C 178's `utmpxname` (macOS defines HAVE_UTMPXNAME,
   macros.tsv 1668).  Dead: C 180's FreeBSD `setutxdb (UTXDB_ACTIVE, x)` and C 184's Android
   `((void) (x))`; C 215's `utmpname` belongs to the dead `utmp.h` branch.  */
#[cfg (feature = "utxdb_active")]
macro_rules! UTMP_NAME_FUNCTION {
  ($x:expr) => { setutxdb (UTXDB_ACTIVE, $x) };
}
#[cfg (feature = "android")]
macro_rules! UTMP_NAME_FUNCTION { ($x:expr) => { ((void) ($x)) }; }
#[cfg (feature = "utmp_h")]
macro_rules! UTMP_NAME_FUNCTION { ($x:expr) => { utmpname ($x) }; }
#[cfg (not (feature = "utxdb_active"))]
#[cfg (not (feature = "android"))]
#[cfg (not (feature = "utmp_h"))]
macro_rules! UTMP_NAME_FUNCTION { ($file:expr) => { utmpxname ($file) }; }

/* C 187-218's `#elif HAVE_UTMP_H` arm: the whole branch is dead (`HAVE_UTMPX_H` is 1, so
   C 112's first arm was taken).  Its `UTMP_STRUCT_NAME`/`SET_UTMP_ENT`/`GET_UTMP_ENT`/
   `END_UTMP_ENT`/`UTMP_NAME_FUNCTION` bindings are the `utmp_h` rows above; its
   `HAVE_UTMPNAME` guard is false (macros.tsv 1667 `undef`).  */

/* C 221-233: the three `HAVE_STRUCT_XTMP_*` queries.  Their bodies are the C expressions the
   file spells, and they name `READUTMP_USE_SYSTEMD` and the `HAVE_STRUCT_UTMP*_*` selectors -
   names the *consumer* supplies (`readutmp.c` defines `READUTMP_USE_SYSTEMD` before this
   include), so they stay C's text and nothing in the port expands them.  */
macro_rules! HAVE_STRUCT_XTMP_UT_ID {
  () => { (READUTMP_USE_SYSTEMD || (HAVE_UTMPX_H ? HAVE_STRUCT_UTMPX_UT_ID : HAVE_STRUCT_UTMP_UT_ID)) };
}
macro_rules! HAVE_STRUCT_XTMP_UT_PID {
  () => { (READUTMP_USE_SYSTEMD || (HAVE_UTMPX_H ? HAVE_STRUCT_UTMPX_UT_PID : HAVE_STRUCT_UTMP_UT_PID)) };
}
macro_rules! HAVE_STRUCT_XTMP_UT_HOST {
  () => { (READUTMP_USE_SYSTEMD || (HAVE_UTMPX_H ? HAVE_STRUCT_UTMPX_UT_HOST : HAVE_STRUCT_UTMP_UT_HOST)) };
}

/* C 235-246: `UTMP_FILE`, defined up to three times in C.  The frozen chain is C 237-238
   (`_PATH_UTMP`, measured "/var/run/utmp") overridden by C 240-242 (`UTMPX_FILE`, measured
   `UTMP_FILE UTMPX_FILE` with `UTMPX_FILE` = `_PATH_UTMPX` = "/var/run/utmpx" from the
   system `<utmpx.h>`), so the live value is the latter's bytes and C 245's "/etc/utmp"
   fallback is dead.  The port's value is the NUL-terminated byte array the call sites pass
   to `utmpxname` (`lib/boot-time.rs` 124's `utmp_file` static).  */
#[cfg (all (not (feature = "no_utmp_file"), not (feature = "no_utmpx_file")))]
pub const UTMP_FILE: &[u8; 15] = b"/var/run/utmpx\0";
#[cfg (feature = "no_utmpx_file")]
pub const UTMP_FILE: &[u8; 14] = b"/var/run/utmp\0";
#[cfg (feature = "no_utmp_file")]
pub const UTMP_FILE: &[u8; 10] = b"/etc/utmp\0";

/* C 248-259: `WTMP_FILE`.  Live: C 251's `_PATH_WTMP` = "/var/log/wtmp" (measured, and the
   same `#undef`-and-redefine chain as `UTMP_FILE`'s); C 258's "/etc/wtmp" fallback is dead,
   and C 255's `WTMPX_FILE` arm is recorded in the ledger (`WTMPX_FILE` is not defined for
   this target, so its bytes are another system's).  */
#[cfg (all (not (feature = "no_wtmp_file"), not (feature = "no_path_wtmp")))]
pub const WTMP_FILE: &[u8; 14] = b"/var/log/wtmp\0";
#[cfg (feature = "no_path_wtmp")]
pub const WTMP_FILE: &[u8; 10] = b"/etc/wtmp\0";

/* C 261-265: `WTMPDB_FILE`, "on Debian and Devuan systems /var/log/wtmp.db".  Live, and the
   gate compares its bytes.  */
pub const WTMPDB_FILE: &[u8; 17] = b"/var/log/wtmp.db\0";

/* C 267-273: the early-Android block (`defined __ANDROID__ && !defined BOOT_TIME`).  Dead:
   the target is not Android, and the system `<utmpx.h>` defines both names.  */
#[cfg (all (feature = "android", not (feature = "no_utmp_x_ut_type")))]
pub const BOOT_TIME: c_short = 2;
#[cfg (all (feature = "android", not (feature = "no_utmp_x_ut_type")))]
pub const LOGIN_PROCESS: c_short = 6;

/* C 275-282: the "no ut_type field" block (OpenBSD and friends).  Dead: `HAVE_UTMPX_H` is 1
   and `HAVE_STRUCT_UTMPX_UT_TYPE` is 1 (macros.tsv 1600), so the ternary is false - clang
   agrees, while the evaluator answers `unknown` and keeps the branch live.  */
#[cfg (feature = "no_utmp_x_ut_type")]
pub const BOOT_TIME: c_short = 2;
#[cfg (feature = "no_utmp_x_ut_type")]
pub const USER_PROCESS: c_short = 0;
#[cfg (feature = "no_utmp_x_ut_type")]
pub const LOGIN_PROCESS: c_short = 6;

/* C 284-299: the three `ut_type` tests.  The `#ifdef` arms are live because the system
   `<utmpx.h>` defines the three names (BOOT_TIME 2, USER_PROCESS 7, LOGIN_PROCESS 6 on the
   frozen target; `include/utmpx.rs` carries BOOT_TIME).  The `0` arms are dead.  */
#[cfg (feature = "no_boot_time_defined")]
macro_rules! UT_TYPE_BOOT_TIME { ($ut:expr) => { 0 }; }
#[cfg (feature = "no_user_process_defined")]
macro_rules! UT_TYPE_USER_PROCESS { ($ut:expr) => { 0 }; }
#[cfg (feature = "no_login_process_defined")]
macro_rules! UT_TYPE_LOGIN_PROCESS { ($ut:expr) => { 0 }; }
#[cfg (not (feature = "no_boot_time_defined"))]
macro_rules! UT_TYPE_BOOT_TIME { ($ut:expr) => { (*$ut).ut_type == BOOT_TIME }; }
#[cfg (not (feature = "no_user_process_defined"))]
macro_rules! UT_TYPE_USER_PROCESS {
  ($ut:expr) => { (*$ut).ut_type == USER_PROCESS };
}
#[cfg (not (feature = "no_login_process_defined"))]
macro_rules! UT_TYPE_LOGIN_PROCESS {
  ($ut:expr) => { (*$ut).ut_type == LOGIN_PROCESS };
}

/* C 301-303: "determines whether an entry *UT corresponds to a user process".  */
macro_rules! IS_USER_PROCESS {
  ($ut:expr) => { (*$ut).ut_user[0] != 0 && UT_TYPE_USER_PROCESS! ($ut) };
}

/* C 305-308: "define if read_utmp is not just a dummy".  Live: `HAVE_UTMPX_H` is 1.  */
pub const READ_UTMP_SUPPORTED: c_int = 1;

/* C 310-317: the `read_utmp` options (an unnamed enum, so the port spells its members as
   constants of C's type).  */
pub const READ_UTMP_CHECK_PIDS: c_int = 1;
pub const READ_UTMP_USER_PROCESS: c_int = 2;
pub const READ_UTMP_BOOT_TIME: c_int = 4;
pub const READ_UTMP_NO_BOOT_TIME: c_int = 8;

/* C 319-323, 344-345: the two entry points.  C's `_GL_ATTRIBUTE_MALLOC`,
   `_GL_ATTRIBUTE_DEALLOC_FREE` and `_GL_ATTRIBUTE_RETURNS_NONNULL` are attributes and are
   dropped structurally (PORTING.md 0.3/8).  They are declared in a whole-import module so
   that a unit keeping its own copy shadows the glob (PORTING.md 0.1-10).  */
pub mod readutmp_decls {
  use super::*;

  unsafe extern "C" {
    /* char *extract_trimmed_name (const STRUCT_UTMP *ut);  */
    pub fn extract_trimmed_name (ut: *const STRUCT_UTMP) -> *mut c_char;
    /* int read_utmp (char const *file, idx_t *n_entries, STRUCT_UTMP **utmp_buf,
                     int options);  */
    pub fn read_utmp (file: *const c_char, n_entries: *mut idx_t,
                      utmp_buf: *mut *mut STRUCT_UTMP, options: c_int) -> c_int;
  }
}
pub use self::readutmp_decls::*;

/* C 50-54's AIX `_THREAD_SAFE` rename, "work around the duplicate struct utmp_data
   declaration".  Dead: `_THREAD_SAFE`/`UTMP_DATA_INIT` are not defined for the target.  */
#[cfg (feature = "thread_safe_utmp_data")]
macro_rules! utmp_data { () => { gl_aix_4_3_workaround_utmp_data }; }
