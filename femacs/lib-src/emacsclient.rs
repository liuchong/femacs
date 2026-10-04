// femacs: port of lib-src/emacsclient.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names, the same
// `main`.  This is a *program* of the C tree's lib-src directory (batch F), built by the
// frozen configuration (it is in the reference `lib-src/Makefile`'s `INSTALLABLES` list) and
// linked against the port's gnulib mirrors and libSystem only -- it is not a translation unit
// of the editor core, so it does not include the `lisp.h` mirror.
//
// **Symbols (measured on the reference object).**  Compiling the C file against the frozen
// configuration (`clang -std=gnu23 -I build/emacs-ref/lib -idirafter spikes/shim
// -I build/emacs-ref/src -I emacs/src -I emacs/lib -c emacsclient.c`) exports exactly two
// symbols, `get_current_dir_name` and `main`; every other definition is `static`.  The port
// marks exactly those two `#[unsafe(no_mangle)]`.
//
// **Dead text (P-08/P-51)**, recorded here and in docs/ports/emacsclient.md, not transcribed:
// `WINDOWSNT`, `DOS_NT` and `HAVE_NTGUI` are undefined in the frozen configuration, so C 23-43
// (the winsock2 block), C 46-47, C 303-459 (`w32_get_resource` through `ttyname`), C 472-484
// (the message box), C 665-676, C 804-820, C 950-986 (`close_winsock`/`initialize_sockets`/
// `w32_connect_timer`), C 1041-1043, C 1087-1088 (`SOCK_CLOEXEC` is not defined on this target
// -- measured), C 1195-1217 (the `w32` timeout thread), C 1452-1454 (`SA_RESETHAND` *is*
// defined, so `reinstall_handler_if_needed`'s live body carries no statement), C 1505-1516,
// C 1704-1705 (`<unistd.h>` defines `_CS_DARWIN_USER_TEMP_DIR`), C 1720-1742 (the Android arm),
// C 1892-1980, C 2037-2100, C 2130-2135, C 2147-2149, C 2167-2172 and C 2217-2219 are not
// compiled for this target.  The file's conditions are decided by
// `python3 tools/check-cond-eval.py lib-src/emacsclient.c` (46 conditions, 0 unknown,
// 0 disagreements [VERIFIED]).
//
// **Use-site declarations (P-26/P-72 (3)).**  C names the following from headers the port has
// no mirror for, so each is declared below with C's prototype in a `/// C:` line and the
// measurement it was taken from: `<arpa/inet.h>`/`<netinet/in.h>`/`<sys/socket.h>`/
// `<sys/un.h>` (the socket ABI, measured with clang against the frozen target -- the same
// numbers `src/process.rs` 149-310 carries), `<inttypes.h>`'s `PRIuMAX` (`"ju"`) and
// `strtoimax`/`strtoumax`, `<stdckdint.h>`'s `ckd_add`/`ckd_mul` (spelled `overflowing_*`,
// the token-aliases family), `<sys/wait.h>`'s `WIFEXITED`/`WEXITSTATUS`/`WUNTRACED`/
// `WCONTINUED`, `<unistd.h>`'s `execvp`/`fork`/`getpgrp`/`tcgetpgrp`/`ttyname`/`STDOUT_FILENO`/
// `environ`, `<stdlib.h>`'s `exit`/`atoi`/`strdup`, `<string.h>`'s `strnlen`/`strstr`,
// `<stdio.h>`'s `getline`/`snprintf`/`vfprintf`, `<signal.h>`'s `sig_atomic_t`/`SIG_DFL`,
// `<config.h>`'s `PACKAGE_VERSION`/`DIRECTORY_SEP`/`IS_DIRECTORY_SEP`/`IS_DEVICE_SEP`, and
// `<stddef.h>`/`<stdint.h>`'s `ptrdiff_t`/`size_t`/`intmax_t`/`uintmax_t`.
//
// `<unlocked-io.h>` (C 97) renames nothing on this target: the reference object imports
// `_fread`/`_fprintf`/`_fflush`, not their `_unlocked` twins.
//
// **Attributes** (`ATTRIBUTE_MALLOC`, `ATTRIBUTE_FORMAT_PRINTF`, `_Noreturn`, the `void *`/
// `const char *` parameter attributes) have no Rust counterpart (PORTING.md 8/P-13); the C
// names stay in the comments at their definitions, and `_Noreturn` is the port's `-> !`.
//
// **Forward declarations dropped** (PORTING.md 0.1-12): C 162 (`print_help_and_exit`), C 251
// (`get_current_dir_name`) and C 793 (`act_on_signals`) declare functions defined later in the
// same file; Rust items are order-independent, so those lines are gone and the definitions
// stay where C has them.
//
// **`switch`** (C 512, C 1158) is the port's `match`; **`goto`** (C 1125/1132/1139/1151/1163/
// 1174/1179, C 2455) is the port's labelled block/loop (`break 'done`, `continue 'retry`), so
// the `goto`/`switch`/`case`/`default` tokens themselves have no Rust site (PORTING.md 2.2);
// docs/ports/macro-body-notes.tsv carries the three exemption rows.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn)]

use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uchar, c_uint, c_ulong,
              c_ushort, c_void};

// C gets `va_list` from <stdarg.h>; the port's equivalent is core::ffi::VaList (P-04).
use core::ffi::VaList;

/// `<stddef.h>`'s `ptrdiff_t`, written out the way `lib-src/make-docfile.rs` 51 does
/// (measured: 8 bytes).  This program has no `lisp.h` mirror to take it from.
pub type ptrdiff_t = isize;
/// `<stddef.h>`'s `size_t` (measured: 8 bytes).
pub type size_t = usize;
/// `<stdint.h>`'s `intmax_t` (measured: 8 bytes).
pub type intmax_t = i64;
/// `<stdint.h>`'s `uintmax_t` (measured: 8 bytes; `PRIuMAX` is `"ju"` on this target, the way
/// `lib-src/hexl.rs` measured `PRIxMAX` as `"jx"`).
pub type uintmax_t = u64;

/// `<signal.h>`'s `sig_atomic_t` -- Darwin's `int` (measured: `sizeof (sig_atomic_t)` is 4).
pub type sig_atomic_t = c_int;

/// `<signal.h>`'s handler type: `typedef void (*sig_t) (int);`.  C's `void (*handler) (int)`
/// parameters are the port's `Option<unsafe extern "C" fn (c_int)>` (PORTING.md 1.4/P-27, the
/// `src/process.rs` 8697 shape); the name is the one `syssignal.h` 74 gives the type, which
/// this program does not include.
pub type signal_handler_t = Option<unsafe extern "C" fn (c_int)>;

include!("../src/include/config.rs");        // C 22: `#include <config.h>`
include!("../lib/include/stdio.rs");         // C's `<stdio.h>` (via `<config.h>`'s conf_post.h)
include!("../src/include/syswait.rs");       // C 49: `#include "syswait.h"`
include!("../lib/include/fcntl.in.rs");      // C 52: `#include <fcntl.h>`
include!("../lib/include/sys_select.rs");    // C 54: `#include <sys/select.h>`
include!("../lib/include/timespec.rs");      // C 57: `#include <timespec.h>`
include!("../lib/include/errno.rs");         // C 73: `#include <errno.h>`
include!("../lib/include/getopt.in.rs");     // C 74: `#include <getopt.h>`
include!("../lib/include/pwd.rs");           // C 76: `#include <pwd.h>`
include!("../lib/include/signal.rs");        // C 77: `#include <signal.h>`
include!("../lib/include/stdlib.rs");        // C 82: `#include <stdlib.h>`
include!("../lib/include/string.rs");        // C 84: `#include <string.h>`
include!("../lib/include/sys_stat.in.rs");   // C 85: `#include <sys/stat.h>`
include!("../lib/include/unistd.in.rs");     // C 86: `#include <unistd.h>`
include!("../lib/include/attribute.rs");     // C 88: `#include <attribute.h>`
include!("../lib/include/c-ctype.rs");       // C 89: `#include <c-ctype.h>`
include!("../lib/include/filename.rs");      // C 90: `#include <filename.h>`
include!("../lib/include/min-max.rs");       // C 92: `#include <min-max.h>`
include!("../lib/include/stdint.in.rs");     // C 80/83's `<stddef.h>`/`<stdint.h>` (PATH_MAX too)
include!("../lib/include/acl.rs");           // C 1357: `# include <acl.h>`

/* C 95-97's `#if GNUC_PREREQ (7, 0, 0)` / `# pragma GCC diagnostic ignored
   "-Wformat-truncation=2"` / `#endif` is a preprocessor-only arm (this target's compiler
   does not satisfy the test); recorded, not transcribed.  */

// C 60: `# define SOCKETS_IN_FILE_SYSTEM` -- the `!WINDOWSNT` arm's empty marker; the
// `#define`s of the dead arm above it spell the same names (P-66's "several definitions").
pub const SOCKETS_IN_FILE_SYSTEM: bool = true;

// C 62-67's five macros, in the live (`!WINDOWSNT`) spelling.
/// C 62: `# define INVALID_SOCKET (-1)`
pub const INVALID_SOCKET: c_int = -1;
/// C 63: `# define HSOCKET int` -- a type macro, so the port keeps C's name as a macro over the
/// target's type (the `src/doc.rs` 67 `doc_fd` shape) and uses it in type position.
macro_rules! HSOCKET { () => { c_int }; }
/// C 64: `# define CLOSE_SOCKET close`
macro_rules! CLOSE_SOCKET { ($socket:expr) => { close ($socket) }; }
/// C 65: `# define INITIALIZE()` -- the `!WINDOWSNT` arm's empty initialiser.
macro_rules! INITIALIZE { () => { () }; }
/// C 67: `# define egetenv(VAR) getenv (VAR)`
macro_rules! egetenv { ($var:expr) => { getenv ($var) }; }

/// C 1359-1361: `# ifndef O_PATH` / `# define O_PATH O_SEARCH` / `# endif` -- this target's
/// `<fcntl.h>` does not define `O_PATH` (measured), so C's own arm is the live one and the
/// port keeps the macro's name over the mirror's `O_SEARCH`.
pub const O_PATH: c_int = O_SEARCH;

/// C 71: `#define DEFAULT_TIMEOUT (30)`
pub const DEFAULT_TIMEOUT: c_int = 30;

/// C 799: `enum { AUTH_KEY_LENGTH = 64 };`
pub const AUTH_KEY_LENGTH: usize = 64;

/// C 1609: `enum { socknamesize = sizeof ((struct sockaddr_un *) NULL)->sun_path };` -- C's
/// array-size idiom, kept as the same measurement over the port's type: the union's size
/// minus the member's offset is the `sun_path` array's 104 bytes (measured), and C's enum
/// member is an `int`.
pub const socknamesize: c_int =
  (std::mem::size_of::<sockaddr_un> () - std::mem::offset_of! (sockaddr_un, sun_path))
  as c_int;
const _: () = assert! (socknamesize == 104);

/// C 1363-1368: `union local_sockaddr { struct sockaddr_un un; struct sockaddr sa; };`
#[repr(C)]
#[derive(Clone, Copy)]
pub union local_sockaddr
{
  pub un: sockaddr_un,   // C 1366
  pub sa: sockaddr,      // C 1367
}
const _: () = assert! (std::mem::size_of::<local_sockaddr> () == 106);
const _: () = assert! (std::mem::offset_of! (local_sockaddr, sa) == 0);

/* The socket ABI and the other system names this program reaches through headers with no
   mirror, declared at the use site (P-26/P-72 (3)).  Every number is measured with the frozen
   target's clang; `src/process.rs` 149-310 carries the same measurements.  */

/// `<netinet/in.h>`: `typedef __uint32_t in_addr_t; struct in_addr { in_addr_t s_addr; };`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr
{
  pub s_addr: u32,
}
const _: () = assert! (std::mem::size_of::<in_addr> () == 4);

/// `<sys/socket.h>`: `struct sockaddr`, measured 16/1 (`sa_len`@0, `sa_family`@1, `sa_data`@2).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr
{
  pub sa_len: u8,
  pub sa_family: u8,
  pub sa_data: [c_char; 14],
}
const _: () = assert! (std::mem::size_of::<sockaddr> () == 16);

/// `<netinet/in.h>`: `struct sockaddr_in`, measured 16/4 (`sin_len`@0, `sin_family`@1,
/// `sin_port`@2, `sin_addr`@4, `sin_zero`@8).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in
{
  pub sin_len: u8,
  pub sin_family: u8,
  pub sin_port: u16,
  pub sin_addr: in_addr,
  pub sin_zero: [c_char; 8],
}
const _: () = assert! (std::mem::size_of::<sockaddr_in> () == 16);
const _: () = assert! (std::mem::offset_of! (sockaddr_in, sin_addr) == 4);

/// `<sys/un.h>`: `struct sockaddr_un`, measured 106/1 (`sun_len`@0, `sun_family`@1,
/// `sun_path`@2).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_un
{
  pub sun_len: u8,
  pub sun_family: u8,
  pub sun_path: [c_char; 104],
}
const _: () = assert! (std::mem::size_of::<sockaddr_un> () == 106);
const _: () = assert! (std::mem::offset_of! (sockaddr_un, sun_path) == 2);

/// `<sys/socket.h>`: `struct linger { int l_onoff; int l_linger; }`, measured 8/4.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct linger
{
  pub l_onoff: c_int,
  pub l_linger: c_int,
}
const _: () = assert! (std::mem::size_of::<linger> () == 8);

/// `<sys/socket.h>`: `typedef __uint32_t socklen_t;` -- the mirror has no `socklen_t`, so it is
/// declared here with the measured width (the `src/process.rs` 308 shape).
pub type socklen_t = u32;

/// `<sys/socket.h>`: the address family and socket type names (measured: `AF_INET` 2,
/// `AF_UNIX` 1, `SOCK_STREAM` 1).
pub const AF_INET: c_int = 2;
pub const AF_UNIX: c_int = 1;
pub const SOCK_STREAM: c_int = 1;

/// `<netinet/in.h>`: `#define IPPROTO_TCP 6` (measured).
pub const IPPROTO_TCP: c_int = 6;

/* `<sys/socket.h>`'s `SOL_SOCKET` and the three option names (measured: `SOL_SOCKET` 0xffff,
   `SO_LINGER` 0x80, `SO_ERROR` 0x1007, `SO_RCVTIMEO` 0x1006).  */
pub const SOL_SOCKET: c_int = 0xffff;
pub const SO_LINGER: c_int = 0x80;
pub const SO_ERROR: c_int = 0x1007;
pub const SO_RCVTIMEO: c_int = 0x1006;

/* `<unistd.h>`'s `STDOUT_FILENO` now comes from the unistd.in.rs mirror (2026-10-04); this
   unit's use-site copy is gone, and the mirror is the single declaration point (PORTING.md
   0.1-3/P-26).  */

/// `<sys/wait.h>`: `#define WUNTRACED 0x00000002` / `#define WCONTINUED 0x00000010`
/// (measured).
pub const WUNTRACED: c_int = 0x2;
pub const WCONTINUED: c_int = 0x10;

/// `<signal.h>`: `#define SIG_DFL ((void (*)(int)) 0)` -- the zero handler.  The mirror keeps
/// the integer for `signal` (its SIG_IGN row says why), but this program passes handlers to
/// `sigaction`, whose `__sa_handler` is `Option<unsafe extern "C" fn (c_int)>` (P-27), so the
/// port's `SIG_DFL` is `None` (the `src/process.rs` 8851 shape).
pub const SIG_DFL: signal_handler_t = None;

/// `<unistd.h>`'s `confstr` name for the per-user temporary directory (measured: 65537).  C
/// 1704-1705's `#ifndef _CS_DARWIN_USER_TEMP_DIR` / `#define … 65537` / `#endif` is dead
/// because `<unistd.h>` defines it; the value is the same.
pub const _CS_DARWIN_USER_TEMP_DIR: c_int = 65537;

/// `<config.h>`'s version string (measured: `PACKAGE_VERSION` is `"32.0.50"`; the
/// `src/emacs.rs` 359 site is the same constant).
pub const PACKAGE_VERSION: &core::ffi::CStr = c"32.0.50";

/// `<stdint.h>`'s `PTRDIFF_MAX`/`SIZE_MAX` -- the mirror has them
/// (`lib/include/stdint.in.rs` 14/24), and `get_current_dir_name` reads both.

/// `<config.h>` 120: `#define DIRECTORY_SEP '/'` (the `src/dired.rs` 99 precedent).
pub const DIRECTORY_SEP: c_char = b'/' as c_char;

/// `<config.h>` 2097: `#define IS_DIRECTORY_SEP(_c_) ((_c_) == DIRECTORY_SEP)`.
macro_rules! IS_DIRECTORY_SEP { ($c:expr) => { ($c) == DIRECTORY_SEP }; }

/// `<config.h>` 2095: `#define IS_DEVICE_SEP(_c_) 0` (the non-DOS value; the
/// `src/callproc.rs` 295 precedent).
#[inline(always)]
fn IS_DEVICE_SEP (_c_: c_char) -> bool { let _ = _c_; false }

// `<stdio.h>`'s entries no mirror carries, declared at the use site (P-26/P-72 (3)).
unsafe extern "C" {
  /// `<stdio.h>`: `int snprintf (char *, size_t, const char *restrict, ...);`
  pub fn snprintf (s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
  /// `<stdio.h>`: `int vfprintf (FILE *restrict, const char *restrict, va_list);` -- the
  /// `va_list` parameter is the port's `VaList<'_>` (P-04; the `src/pdumper.rs` 122 shape).
  pub fn vfprintf (stream: *mut FILE, format: *const c_char, ap: VaList<'_>) -> c_int;
  /// `<stdio.h>`: `ssize_t getline (char **restrict, size_t *restrict, FILE *restrict);`
  pub fn getline (lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
}

// `<stdlib.h>`: `void exit (int);` -- no mirror carries it yet (the emacs.rs/sysdep.rs
// precedent keeps it at the use site).
unsafe extern "C" {
  pub fn exit (status: c_int) -> !;
}

unsafe extern "C" {
  /// `<stdlib.h>`: `int atoi (const char *);`
  pub fn atoi (nptr: *const c_char) -> c_int;
  /// `<stdlib.h>`: `char *strdup (const char *);`
  pub fn strdup (s: *const c_char) -> *mut c_char;
  /// `<string.h>`: `size_t strnlen (const char *, size_t);`
  pub fn strnlen (s: *const c_char, maxlen: size_t) -> size_t;
  /// `<string.h>`: `char *strstr (const char *, const char *);` (the `src/font.rs` 748
  /// precedent).
  pub fn strstr (haystack: *const c_char, needle: *const c_char) -> *mut c_char;
  /// `<inttypes.h>`: `intmax_t strtoimax (const char *restrict, char **restrict, int);`
  pub fn strtoimax (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> intmax_t;
  /// `<inttypes.h>`: `uintmax_t strtoumax (const char *restrict, char **restrict, int);`
  pub fn strtoumax (nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> uintmax_t;
  /// `<unistd.h>`: `int execvp (const char *, char *const []);`
  pub fn execvp (file: *const c_char, argv: *const *mut c_char) -> c_int;
  /// `<unistd.h>`: `pid_t fork (void);`
  pub fn fork () -> pid_t;
  /// `<unistd.h>`: `pid_t getpgrp (void);`
  pub fn getpgrp () -> pid_t;
  /// `<unistd.h>`: `pid_t tcgetpgrp (int);`
  pub fn tcgetpgrp (fd: c_int) -> pid_t;
  /// `<unistd.h>`: `char *ttyname (int);`
  pub fn ttyname (fd: c_int) -> *mut c_char;
  /// `<signal.h>`: `int raise (int);` -- the mirror declares it in `signal_decls`.
  /// `<unistd.h>`: `int kill (pid_t, int);`
  pub fn kill (pid: pid_t, sig: c_int) -> c_int;
  /// `<sys/wait.h>`: `pid_t waitpid (pid_t, int *, int);` (the `lib-src/movemail.rs` 71
  /// precedent).
  pub fn waitpid (pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
  /// `<unistd.h>`: `size_t confstr (int, char *, size_t);`
  pub fn confstr (name: c_int, buf: *mut c_char, len: size_t) -> size_t;
  /* `<unistd.h>`'s `environ` now comes from the unistd.in.rs mirror (2026-10-04): C 2225
     reads it in `main`, and the mirror declares both of the header's arms - the object itself
     and `(*_NSGetEnviron ())` (which is the spelling this target's system header uses, and the
     symbol the reference object imports).  The use-site copy is gone (PORTING.md 0.1-3).  */
  /// `<arpa/inet.h>`: `in_addr_t inet_addr (const char *);`
  pub fn inet_addr (cp: *const c_char) -> u32;
  /// `<arpa/inet.h>`: `char *inet_ntoa (struct in_addr);`
  pub fn inet_ntoa (in_: in_addr) -> *mut c_char;
  /// `<netinet/in.h>`: `uint16_t htons (uint16_t);`
  pub fn htons (hostshort: c_ushort) -> c_ushort;
  /// `<sys/socket.h>`: `int socket (int, int, int);`
  pub fn socket (domain: c_int, r#type: c_int, protocol: c_int) -> c_int;
  /// `<sys/socket.h>`: `int connect (int, const struct sockaddr *, socklen_t);`
  pub fn connect (socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
  /// `<sys/socket.h>`: `ssize_t send (int, const void *, size_t, int);`
  pub fn send (socket: c_int, buffer: *const c_void, length: size_t, flags: c_int) -> ssize_t;
  /// `<sys/socket.h>`: `ssize_t recv (int, void *, size_t, int);`
  pub fn recv (socket: c_int, buffer: *mut c_void, length: size_t, flags: c_int) -> ssize_t;
  /// `<sys/socket.h>`: `int setsockopt (int, int, int, const void *, socklen_t);`
  pub fn setsockopt (socket: c_int, level: c_int, option_name: c_int,
                     option_value: *const c_void, option_len: socklen_t) -> c_int;
  /// `<sys/socket.h>`: `int getsockopt (int, int, int, void *restrict, socklen_t *restrict);`
  pub fn getsockopt (socket: c_int, level: c_int, option_name: c_int,
                     option_value: *mut c_void, option_len: *mut socklen_t) -> c_int;
}

/// `<sys/wait.h>`: `#define WIFEXITED(status) (WTERMSIG (status) == 0)` -- written as the
/// inline function `src/callproc.rs` 267 established with the same measured body.
#[inline(always)]
unsafe fn WIFEXITED (status: c_int) -> bool
{ (status & 0x7f) == 0 }

/// `<sys/wait.h>`: `#define WEXITSTATUS(status) (((status) & 0xff00) >> 8)`.
#[inline(always)]
unsafe fn WEXITSTATUS (status: c_int) -> c_int
{ (status >> 8) & 0xff }

/* C 101-164's file-scope objects, in C's order.  Each is a `static` in C (internal linkage),
   so each is a non-`pub` `static mut` here (PORTING.md 0.2), with the zero initialiser a C
   object without an initialiser gets.  The extractor's "File-scope objects" list misses the
   six `char const *` ones and the two arrays (`tools/cinventory.py`'s decls splitter drops a
   mid-declarator `const`); they are all carried, and the gap is reported in
   docs/ports/emacsclient.md.  */

// C 102: `static char const *progname;`  (the name this program was invoked with)
static mut progname: *const c_char = std::ptr::null ();

// C 105: `static int main_argc;`  (the first argument to main)
static mut main_argc: c_int = 0;

// C 108: `static char *const *main_argv;`  (the second argument to main)
static mut main_argv: *const *mut c_char = std::ptr::null ();

// C 111: `static bool nowait;`  (don't wait for a response from the server)
static mut nowait: bool = false;

// C 114: `static bool quiet;`  (don't print messages for successful operations)
static mut quiet: bool = false;

// C 117: `static bool suppress_output;`  (don't print values returned by the server)
static mut suppress_output: bool = false;

// C 120: `static bool eval;`  (the arguments are expressions to evaluate)
static mut eval: bool = false;

// C 123: `static bool create_frame;`  (open a new frame)
static mut create_frame: bool = false;

// C 126: `static bool reuse_frame;`  (reuse a frame if it already exists)
static mut reuse_frame: bool = false;

// C 129: `static char const *display;`  (the display to work on)
static mut display: *const c_char = std::ptr::null ();

// C 132: `static char const *alt_display;`  (the display to try if `display` is unsupported)
static mut alt_display: *const c_char = std::ptr::null ();

// C 135: `static char *parent_id;`  (the parent window ID, for XEmbed)
static mut parent_id: *mut c_char = std::ptr::null_mut ();

// C 138: `static bool tty;`  (open a new frame on the current terminal)
static mut tty: bool = false;

// C 142: `static char *alternate_editor;`  (the editor to fall back to)
static mut alternate_editor: *mut c_char = std::ptr::null_mut ();

// C 145-147: `static char const *socket_name;`  (the `SOCKETS_IN_FILE_SYSTEM` arm)
static mut socket_name: *const c_char = std::ptr::null ();

// C 150: `static char const *server_file;`  (the authentication file)
static mut server_file: *const c_char = std::ptr::null ();

// C 154: `static intmax_t timeout = -1;`  (negative: no --timeout, so use DEFAULT_TIMEOUT;
// zero: wait forever)
static mut timeout: intmax_t = -1;

// C 157: `static char const *tramp_prefix;`  (the prefix the server must use for the files)
static mut tramp_prefix: *const c_char = std::ptr::null ();

// C 160: `static pid_t emacs_pid;`  (the server's process ID, when known)
static mut emacs_pid: pid_t = 0;

// C 164: `static char const *frame_parameters;`  (the new frame's parameter alist)
static mut frame_parameters: *const c_char = std::ptr::null ();

/* C 168-190: `static struct option const longopts[]` -- the two option tables, in C's order.
   C's arrays are `const`, but Rust statics of a raw-pointer-bearing type are not `Sync`, so
   the port writes the `static mut` the other pointer tables use (`src/alloc.rs` 412).  */
static mut longopts: [option; 20] =
  [
    option { name: c"no-wait".as_ptr (), has_arg: no_argument,
             flag: std::ptr::null_mut (), val: 'n' as c_int },
    option { name: c"quiet".as_ptr (), has_arg: no_argument,
             flag: std::ptr::null_mut (), val: 'q' as c_int },
    option { name: c"suppress-output".as_ptr (), has_arg: no_argument,
             flag: std::ptr::null_mut (), val: 'u' as c_int },
    option { name: c"eval".as_ptr (), has_arg: no_argument,
             flag: std::ptr::null_mut (), val: 'e' as c_int },
    option { name: c"help".as_ptr (), has_arg: no_argument,
             flag: std::ptr::null_mut (), val: 'H' as c_int },
    option { name: c"version".as_ptr (), has_arg: no_argument,
             flag: std::ptr::null_mut (), val: 'V' as c_int },
    option { name: c"tty".as_ptr (), has_arg: no_argument,
             flag: std::ptr::null_mut (), val: 't' as c_int },
    option { name: c"nw".as_ptr (), has_arg: no_argument,
             flag: std::ptr::null_mut (), val: 't' as c_int },
    option { name: c"no-window-system".as_ptr (), has_arg: no_argument,
             flag: std::ptr::null_mut (), val: 't' as c_int },
    option { name: c"create-frame".as_ptr (), has_arg: no_argument,
             flag: std::ptr::null_mut (), val: 'c' as c_int },
    option { name: c"reuse-frame".as_ptr (), has_arg: no_argument,
             flag: std::ptr::null_mut (), val: 'r' as c_int },
    option { name: c"alternate-editor".as_ptr (), has_arg: required_argument,
             flag: std::ptr::null_mut (), val: 'a' as c_int },
    option { name: c"frame-parameters".as_ptr (), has_arg: required_argument,
             flag: std::ptr::null_mut (), val: 'F' as c_int },
    /* C 181-183: `#ifdef SOCKETS_IN_FILE_SYSTEM` is live.  */
    option { name: c"socket-name".as_ptr (), has_arg: required_argument,
             flag: std::ptr::null_mut (), val: 's' as c_int },
    option { name: c"server-file".as_ptr (), has_arg: required_argument,
             flag: std::ptr::null_mut (), val: 'f' as c_int },
    option { name: c"display".as_ptr (), has_arg: required_argument,
             flag: std::ptr::null_mut (), val: 'd' as c_int },
    option { name: c"parent-id".as_ptr (), has_arg: required_argument,
             flag: std::ptr::null_mut (), val: 'p' as c_int },
    option { name: c"timeout".as_ptr (), has_arg: required_argument,
             flag: std::ptr::null_mut (), val: 'w' as c_int },
    option { name: c"tramp".as_ptr (), has_arg: required_argument,
             flag: std::ptr::null_mut (), val: 'T' as c_int },
    /* C 189: `{ 0, 0, 0, 0 }` -- the table's terminator.  */
    option { name: std::ptr::null (), has_arg: 0, flag: std::ptr::null_mut (), val: 0 },
  ];

/// C 191-198: `static char const shortopts[] = "nqueHVtca:F:w:" "s:" "f:d:T:";` -- the live
/// text is the three-way concatenation, so the port writes the one literal it spells.  C's
/// array is `const`, and `&CStr` is `Sync`, so this one keeps C's non-`mut` static.
static shortopts: &core::ffi::CStr = c"nqueHVtca:F:w:s:f:d:T:";

/* Like malloc but get fatal error if memory is exhausted.  */

// C 206-209: `static void * ATTRIBUTE_MALLOC xmalloc (size_t size)` -- the attribute has no
// Rust counterpart (PORTING.md 8); the parameter is C's `size_t`.
unsafe fn xmalloc (size: size_t) -> *mut c_void
{
  let result: *mut c_void = malloc (size);
  if result.is_null ()
    {
      perror (c"malloc".as_ptr ());
      exit (EXIT_FAILURE);
    }
  return result;
}

/* Like realloc but get fatal error if memory is exhausted.  */

// C 221-223: `static void * xrealloc (void *ptr, size_t size)`
unsafe fn xrealloc (ptr: *mut c_void, size: size_t) -> *mut c_void
{
  let result: *mut c_void = realloc (ptr, size);
  if result.is_null ()
    {
      perror (c"realloc".as_ptr ());
      exit (EXIT_FAILURE);
    }
  return result;
}

/* Like strdup but get a fatal error if memory is exhausted. */

// C 235-237: `static char * ATTRIBUTE_MALLOC xstrdup (const char *s)`
unsafe fn xstrdup (s: *const c_char) -> *mut c_char
{
  let result: *mut c_char = strdup (s);
  if result.is_null ()
    {
      perror (c"strdup".as_ptr ());
      exit (EXIT_FAILURE);
    }
  return result;
}

/* From sysdep.c */
/* C 249-250: `#if !defined HAVE_GET_CURRENT_DIR_NAME || defined BROKEN_GET_CURRENT_DIR_NAME`
   is live: the frozen configuration leaves `HAVE_GET_CURRENT_DIR_NAME` undefined
   (macros.tsv).  The prototype C 251 carries is dropped (PORTING.md 0.1-12).  */

/* Return the current working directory.  Returns NULL on errors.
   Any other returned value must be freed with free.  This is used
   only when get_current_dir_name is not defined on the system.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_current_dir_name () -> *mut c_char
{
  /* The maximum size of a directory name, including the terminating NUL.
     Leave room so that the caller can append a trailing slash.  */
  let dirsize_max: ptrdiff_t = min! (PTRDIFF_MAX as usize, SIZE_MAX) as ptrdiff_t - 1;

  /* The maximum size of a buffer for a file name, including the terminating
     NUL.  This is bounded by PATH_MAX, if available.  */
  let mut bufsize_max: ptrdiff_t = dirsize_max;
  /* C 240-242: `#ifdef PATH_MAX` is live (`lib/include/limits.in.rs` 64, 1024).  */
  {
    bufsize_max = min! (bufsize_max, PATH_MAX as ptrdiff_t);
  }

  let mut dotstat: stat = std::mem::zeroed ();
  let mut pwdstat: stat = std::mem::zeroed ();
  let mut pwdlen: size_t = 0;
  /* If PWD is accurate, use it instead of calling getcwd.  PWD is
     sometimes a nicer name, and using it may avoid a fatal error if a
     parent directory is searchable but not readable.  */
  let pwd: *const c_char = egetenv! (c"PWD".as_ptr ());
  if !pwd.is_null ()
      && { pwdlen = strnlen (pwd, bufsize_max as size_t); pwdlen < bufsize_max as size_t }
      && IS_DIRECTORY_SEP! (*pwd.offset (
	   if pwdlen != 0 && IS_DEVICE_SEP (*pwd.offset (1)) { 2 } else { 0 }))
      && stat (pwd, &raw mut pwdstat) == 0
      && stat (c".".as_ptr (), &raw mut dotstat) == 0
      && dotstat.st_ino == pwdstat.st_ino
      && dotstat.st_dev == pwdstat.st_dev
    { return strdup (pwd); }
  else
    {
      let mut buf_size: ptrdiff_t = min! (bufsize_max, 1024);
      loop
	{
	  let buf: *mut c_char = malloc (buf_size as size_t) as *mut c_char;
	  if buf.is_null ()
	    { return std::ptr::null_mut (); }
	  if getcwd (buf, buf_size as size_t) == buf
	    { return buf; }
	  free (buf as *mut c_void);
	  if *errno () != ERANGE || buf_size == bufsize_max
	    { return std::ptr::null_mut (); }
	  buf_size = if buf_size <= bufsize_max / 2 { 2 * buf_size } else { bufsize_max };
	}
    }
}

/* Display a normal or error message.  */

// C 462-464: the prototype; the definition below is the one that counts, and
// `ATTRIBUTE_FORMAT_PRINTF (2, 3)` has no Rust counterpart (PORTING.md 8).
// C 465-466: `static void message (bool is_error, const char *format, ...)` -- C's
// `va_list args; va_start (args, format); ... va_end (args);` have no port site: the `...`
// parameter *is* the list (P-04).
unsafe extern "C" fn message (is_error: bool, format: *const c_char, mut args: ...)
{
  /* C 468-470: `va_list args; va_start (args, format);` -- the `...` parameter is the list
     (P-04).  */
  {
    /* C 472-485 is the dead `WINDOWSNT` arm (the message box); the live text is the `else`
       body C 486-491.  */
    let f: *mut FILE = if is_error { stderr } else { stdout };

    vfprintf (f, format, args);
    fflush (f);
  }
  /* C 493: `va_end (args);` -- same (P-04).  */
}

/* Decode the options from argv and argc.
   The global variable 'optind' will say how many arguments we used up.  */

// C 499-500: `static void decode_options (int argc, char **argv)`
unsafe fn decode_options (argc: c_int, argv: *mut *mut c_char)
{
  alternate_editor = egetenv! (c"ALTERNATE_EDITOR".as_ptr ());
  tramp_prefix = egetenv! (c"EMACSCLIENT_TRAMP".as_ptr ());

  loop
    {
      let opt: c_int = getopt_long_only (argc, argv, shortopts.as_ptr (),
					 &raw const longopts as *const option,
					 std::ptr::null_mut ());
      if opt < 0
	{ break; }

      let mut endptr: *mut c_char = std::ptr::null_mut ();
      match opt
	{
	  /* C 514-517: `case 0:` -- getopt has already processed a long-named option.  */
	  0 =>
	    {
	    }

	  /* C 519-521: `case 'a':` */
	  97 /* 'a' */ =>
	    {
	      alternate_editor = optarg;
	    }

	  /* C 523-527: `#ifdef SOCKETS_IN_FILE_SYSTEM` is live.  */
	  115 /* 's' */ =>
	    {
	      socket_name = optarg;
	    }

	  /* C 529-531: `case 'f':` */
	  102 /* 'f' */ =>
	    {
	      server_file = optarg;
	    }

	  /* We used to disallow this argument in w32, but it seems better
	     to allow it, for the occasional case where the user is
	     connecting with a w32 client to a server compiled with X11
	     support.  */
	  /* C 537-539: `case 'd':` */
	  100 /* 'd' */ =>
	    {
	      display = optarg;
	    }

	  /* C 541-543: `case 'n':` */
	  110 /* 'n' */ =>
	    {
	      nowait = true;
	    }

	  /* C 545-552: `case 'w':` */
	  119 /* 'w' */ =>
	    {
	      timeout = strtoimax (optarg, &raw mut endptr, 10);
	      if timeout < 0 || endptr == optarg || *endptr != 0
		{
		  fprintf (stderr, c"Invalid timeout: \"%s\"\n".as_ptr (), optarg);
		  exit (EXIT_FAILURE);
		}
	    }

	  /* C 554-556: `case 'e':` */
	  101 /* 'e' */ =>
	    {
	      eval = true;
	    }

	  /* C 558-560: `case 'q':` */
	  113 /* 'q' */ =>
	    {
	      quiet = true;
	    }

	  /* C 562-564: `case 'u':` */
	  117 /* 'u' */ =>
	    {
	      suppress_output = true;
	    }

	  /* C 566-569: `case 'V':` */
	  86 /* 'V' */ =>
	    {
	      message (false, c"emacsclient %s\n".as_ptr (), PACKAGE_VERSION.as_ptr ());
	      exit (EXIT_SUCCESS);
	    }

	  /* C 571-575: `case 't':` */
	  116 /* 't' */ =>
	    {
	      tty = true;
	      create_frame = true;
	      reuse_frame = false;
	    }

	  /* C 577-579: `case 'c':` */
	  99 /* 'c' */ =>
	    {
	      create_frame = true;
	    }

	  /* C 581-585: `case 'r':` */
	  114 /* 'r' */ =>
	    {
	      create_frame = true;
	      if !tty
		{ reuse_frame = true; }
	    }

	  /* C 587-590: `case 'p':` */
	  112 /* 'p' */ =>
	    {
	      parent_id = optarg;
	      create_frame = true;
	    }

	  /* C 592-594: `case 'H':` */
	  72 /* 'H' */ =>
	    {
	      print_help_and_exit ();
	    }

	  /* C 596-598: `case 'F':` */
	  70 /* 'F' */ =>
	    {
	      frame_parameters = optarg;
	    }

	  /* C 600-602: `case 'T':` */
	  84 /* 'T' */ =>
	    {
	      tramp_prefix = optarg;
	    }

	  /* C 604-607: `default:` */
	  _ =>
	    {
	      message (true, c"Try '%s --help' for more information\n".as_ptr (), progname);
	      exit (EXIT_FAILURE);
	    }
	}
    }

  /* If the -c option is used (without -t) and no --display argument
     is provided, try $DISPLAY.
     Without the -c option, we used to set 'display' to $DISPLAY by
     default, but this changed the default behavior and is sometimes
     inconvenient.  So we force users to use "--display $DISPLAY" if
     they want Emacs to connect to their current display.

     Some window systems have a notion of default display not
     reflected in the DISPLAY variable.  If the user didn't give us an
     explicit display, try this platform-specific after trying the
     display in DISPLAY (if any).  */
  if create_frame && !tty && display.is_null ()
    {
      /* C 624-634: `#ifndef HAVE_ANDROID` is live, and `NS_IMPL_COCOA`, `HAVE_NTGUI` and
	 `HAVE_HAIKU` are all undefined, so the whole `alt_display` chain (C 627-632) is dead
	 text with no compiled statement.  */
      /* C 635-640: `#ifdef HAVE_PGTK` is dead, so the `#else` arm is the live one.  */
      display = egetenv! (c"DISPLAY".as_ptr ());
      /* C 641-648's `#else /* HAVE_ANDROID */` arm is dead text.  */
    }

  if display.is_null ()
    {
      display = alt_display;
      alt_display = std::ptr::null ();
    }

  /* A null-string display is invalid.  */
  if !display.is_null () && *display == 0
    { display = std::ptr::null (); }

  /* If no display is available, new frames are tty frames.  */
  if create_frame && display.is_null ()
    { tty = true; }

  /* C 665-676: the dead `WINDOWSNT` arm (it clears `display` and forces `tty`); recorded,
     not transcribed.  */
}

// C 681-682: `static _Noreturn void print_help_and_exit (void)` -- `_Noreturn` is the port's
// `-> !`.
unsafe fn print_help_and_exit () -> !
{
  /* Spaces and tabs are significant in this message; they're chosen so the
     message aligns properly both in a tty and in a Windows message box.
     Please try to preserve them; otherwise the output is very hard to read
     when using emacsclientw.  */
  message (false,
	   c"Usage: %s [OPTIONS] FILE...\n%s%s%s".as_ptr (), progname,
	   c"Tell the Emacs server to visit the specified files.\nEvery FILE can be either just a FILENAME or [+LINE[:COLUMN]] FILENAME.\n\nThe following OPTIONS are accepted:\n-V, --version\t\tJust print version info and return\n-H, --help    \t\tPrint this usage information message\n-nw, -t, --tty, --no-window-system\n\t\t\tOpen a new Emacs frame on the current terminal\n-c, --create-frame    \tCreate a new frame instead of trying to\n\t\t\tuse the current Emacs frame\n-r, --reuse-frame\tCreate a new frame if none exists, otherwise\n\t\t\tuse the current Emacs frame\n".as_ptr (),
	   c"-F ALIST, --frame-parameters=ALIST\n\t\t\tSet the parameters of a new frame\n-e, --eval    \t\tEvaluate the FILE arguments as Elisp expressions\n-n, --no-wait\t\tDon't wait for the server to return\n-w, --timeout=SECONDS\tSeconds to wait before timing out\n-q, --quiet\t\tDon't display messages on success\n-u, --suppress-output   Don't display return values from the server\n-d DISPLAY, --display=DISPLAY\n\t\t\tVisit the file in the given display\n".as_ptr (),
	   c"--parent-id=ID          Open in parent window ID, via XEmbed\n-s SOCKET, --socket-name=SOCKET\n\t\t\tSet filename of the UNIX socket for communication\n-f SERVER, --server-file=SERVER\n\t\t\tSet filename of the TCP authentication file\n-a EDITOR, --alternate-editor=EDITOR\n\t\t\tEditor to fallback to if the server is not running\n\t\t\tIf EDITOR is the empty string, start Emacs in daemon\n\t\t\tmode and try connecting again\n-T PREFIX, --tramp=PREFIX\n                        PREFIX to prepend to filenames sent by emacsclient\n                        for locating files remotely via Tramp\n\nReport bugs with M-x report-emacs-bug.\n".as_ptr ());
  exit (EXIT_SUCCESS);
}

/* Try to run a different command, or --if no alternate editor is
   defined-- exit with an error code.
   Uses argv, but gets it from the global variable main_argv.  */

// C 736-737: `static _Noreturn void fail (void)`
unsafe fn fail () -> !
{
  if !alternate_editor.is_null ()
    {
      let extra_args_size: size_t =
	((main_argc - optind + 1) as size_t) * std::mem::size_of::<*mut c_char> ();
      let new_argv_size: size_t = extra_args_size;
      let mut new_argv: *mut *mut c_char = xmalloc (new_argv_size) as *mut *mut c_char;
      let s: *mut c_char = xstrdup (alternate_editor);
      let mut toks: ptrdiff_t = 0;

      /* Unpack alternate_editor's space-separated tokens into new_argv.  */
      let mut tok: *mut c_char = s;
      loop
	{
	  /* Skip leading delimiters, and set separator, skipping any
	     opening quote.  Break out of loop if no remaining tokens.  */
	  while *tok == b' ' as c_char
	    { tok = tok.offset (1); }
	  if *tok == 0
	    { break; }
	  let mut sep: c_char = b' ' as c_char;
	  if *tok == b'"' as c_char
	    {
	      tok = tok.offset (1);
	      sep = b'"' as c_char;
	    }

	  /* Allocate new token.  */
	  toks += 1;
	  new_argv = xrealloc (new_argv as *mut c_void,
			       new_argv_size
			       + (toks as size_t) * std::mem::size_of::<*mut c_char> ())
	             as *mut *mut c_char;

	  /* Record start of token.  */
	  *new_argv.offset (toks - 1) = tok;

	  /* Find end of token and overwrite it with NUL.  */
	  tok = strchr (tok, sep as c_int);
	  if tok.is_null ()
	    { break; }
	  *tok = 0;
	  tok = tok.offset (1);
	}

      if toks != 0
	{
	  /* Append main_argv arguments to new_argv.  */
	  memcpy (&raw mut *new_argv.offset (toks) as *mut *mut c_char as *mut c_void,
		  main_argv.offset (optind as isize) as *const c_void, extra_args_size);

	  execvp (*new_argv, new_argv);
	  message (true, c"%s: error executing alternate editor \"%s\": %s\n".as_ptr (),
		   progname, alternate_editor, strerror (*errno ()));
	}
    }
  exit (EXIT_FAILURE);
}

// C 801-802: `static void sock_err_message (const char *function_name)`
unsafe fn sock_err_message (function_name: *const c_char)
{
  /* C 804-820: the dead `WINDOWSNT` arm (the `FormatMessage` text); the live text is the
     `#else` body C 821-823.  */
  {
    message (true, c"%s: %s: %s\n".as_ptr (), progname, function_name, strerror (*errno ()));
  }
}

/* Send to S the DATA, of size DLEN, when either
   - the data's last byte is '\n', or
   - the buffer is full.
   Otherwise, just accumulate the data.  */

// C 831-832: `static void send_to_emacs_len (HSOCKET s, const char *data, ptrdiff_t dlen)`
unsafe fn send_to_emacs_len (s: HSOCKET! (), mut data: *const c_char, mut dlen: ptrdiff_t)
{
  const SEND_BUFFER_SIZE: c_int = 4096;

  /* Buffer to accumulate data to send in TCP connections.  */
  static mut send_buffer: [c_char; SEND_BUFFER_SIZE as usize] =
    [0; SEND_BUFFER_SIZE as usize];

  /* Fill pointer for the send buffer.  */
  static mut sblen: c_int = 0;

  while dlen != 0
    {
      let part: c_int = min! (dlen, (SEND_BUFFER_SIZE - sblen) as ptrdiff_t) as c_int;
      memcpy (&raw mut send_buffer[sblen as usize] as *mut c_char as *mut c_void,
	      data as *const c_void, part as size_t);
      data = data.offset (part as isize);
      sblen += part;

      if sblen == SEND_BUFFER_SIZE
	  || (0 < sblen && send_buffer[(sblen - 1) as usize] == b'\n' as c_char)
	{
	  let mut sent: c_int = 0;
	  loop
	    {
	      sent = send (s, &raw const send_buffer as *const c_char as *const c_void,
			   sblen as size_t, 0) as c_int;
	      if !(sent < 0)
		{ break; }
	      if *errno () != EINTR
		{
		  message (true, c"%s: failed to send %d bytes to socket: %s\n".as_ptr (),
			   progname, sblen, strerror (*errno ()));
		  fail ();
		}
	      /* Act on signals not requiring communication to Emacs,
		 but defer action on the others to avoid confusing the
		 communication currently in progress.  */
	      act_on_signals (INVALID_SOCKET);
	    }
	  sblen -= sent;
	  memmove (&raw mut send_buffer as *mut c_char as *mut c_void,
		   &raw mut send_buffer[sent as usize] as *mut c_char as *const c_void,
		   sblen as size_t);
	}

      dlen -= part as ptrdiff_t;
    }
}

/* Send to S the data in the string DATA.  */
// C 875-876: `static void send_to_emacs (HSOCKET s, const char *data)`
unsafe fn send_to_emacs (s: HSOCKET! (), data: *const c_char)
{
  send_to_emacs_len (s, data, strlen (data) as ptrdiff_t);
}

/* Output to S a quoted copy of the array of bytes STR with length LEN.
   Insert a & before each &, each space, each newline, and
   any initial -.  Change spaces to underscores, too, so that the
   output never contains a space.  */
// C 886-887: `static void quote_argument_len (HSOCKET s, const char *str, ptrdiff_t len)`
unsafe fn quote_argument_len (s: HSOCKET! (), mut str: *const c_char, len: ptrdiff_t)
{
  let lim: *const c_char = str.offset (len as isize);
  let copy: *mut c_char = xmalloc ((len * 2) as size_t) as *mut c_char;
  let mut q: *mut c_char = copy;
  if str < lim && *str == b'-' as c_char
    { *q = b'&' as c_char; q = q.offset (1); }
  while str < lim
    {
      let mut c: c_char = *str;
      if c == b' ' as c_char
	{ *q = b'&' as c_char; q = q.offset (1); c = b'_' as c_char; }
      else if c == b'\n' as c_char
	{ *q = b'&' as c_char; q = q.offset (1); c = b'n' as c_char; }
      else if c == b'&' as c_char
	{ *q = b'&' as c_char; q = q.offset (1); }
      *q = c; q = q.offset (1);
      str = str.offset (1);
    }

  send_to_emacs_len (s, copy, q.offset_from (copy));

  free (copy as *mut c_void);
}

/* Output to S a quoted copy of the string STR.
   Insert a & before each &, each space, each newline, and
   any initial -.  Change spaces to underscores, too, so that the
   output never contains a space.  */
// C 915-916: `static void quote_argument (HSOCKET s, const char *str)`
unsafe fn quote_argument (s: HSOCKET! (), str: *const c_char)
{
  quote_argument_len (s, str, strlen (str) as ptrdiff_t);
}

/* The inverse of quote_argument.  Remove quoting in string STR by
   modifying the addressed string in place.  Return STR.  */

// C 924-925: `static char *unquote_argument (char *str)`
unsafe fn unquote_argument (str: *mut c_char) -> *mut c_char
{
  let mut p: *const c_char = str;
  let mut q: *mut c_char = str;
  let mut c: c_char = 0;

  loop
    {
      c = *p;
      p = p.offset (1);
      if c == b'&' as c_char
	{
	  c = *p;
	  p = p.offset (1);
	  if c == b'_' as c_char
	    { c = b' ' as c_char; }
	  else if c == b'n' as c_char
	    { c = b'\n' as c_char; }
	}
      *q = c;
      q = q.offset (1);
      if c == 0
	{ break; }
    }

  return str;
}

/* If the home directory is HOME, and XDG_CONFIG_HOME's value is XDG,
   return the configuration file with basename CONFIG_FILE.  Fail if
   the configuration file could not be opened.  */

// C 993-994: `static FILE *open_config (char const *home, char const *xdg,
// char const *config_file)`
unsafe fn open_config (home: *const c_char, xdg: *const c_char,
		       config_file: *const c_char) -> *mut FILE
{
  /* C 996: `sizeof "/emacs/server/"` is 15 (the literal and its NUL); C 997-998's
     `sizeof "/.config/emacs/server/"` is 23 and `sizeof "/.emacs.d/server/"` is 18.  */
  let xdgsubdirsize: ptrdiff_t =
    if !xdg.is_null () { (strlen (xdg) + 15) as ptrdiff_t } else { 0 };
  let homesuffixsizemax: ptrdiff_t = max! (23, 18);
  let homesubdirsizemax: ptrdiff_t =
    if !home.is_null () { (strlen (home) + homesuffixsizemax as size_t) as ptrdiff_t } else { 0 };
  let configname: *mut c_char =
    xmalloc (max! (xdgsubdirsize, homesubdirsizemax) as size_t + strlen (config_file))
    as *mut c_char;
  let mut config: *mut FILE;

  if !home.is_null ()
    {
      strcpy (stpcpy (stpcpy (configname, home), c"/.emacs.d/server/".as_ptr ()),
	      config_file);
      config = fopen (configname, c"rb".as_ptr ());
    }
  else
    { config = std::ptr::null_mut (); }

  if config.is_null () && (!xdg.is_null () || !home.is_null ())
    {
      strcpy ((if !xdg.is_null ()
		 { stpcpy (stpcpy (configname, xdg), c"/emacs/server/".as_ptr ()) }
	       else
		 { stpcpy (stpcpy (configname, home), c"/.config/emacs/server/".as_ptr ()) }),
	      config_file);
      config = fopen (configname, c"rb".as_ptr ());
    }

  free (configname as *mut c_void);
  return config;
}

/* Read the information needed to set up a TCP comm channel with
   the Emacs server: host, port, and authentication string.  */

// C 1029-1031: `static bool get_server_config (const char *config_file,
// struct sockaddr_in *server, char *authentication)`
unsafe fn get_server_config (config_file: *const c_char, server: *mut sockaddr_in,
			     authentication: *mut c_char) -> bool
{
  let config: *mut FILE;

  if IS_ABSOLUTE_FILE_NAME! (config_file)
    { config = fopen (config_file, c"rb".as_ptr ()); }
  else
    {
      let xdg: *const c_char = egetenv! (c"XDG_CONFIG_HOME".as_ptr ());
      config = open_config (egetenv! (c"HOME".as_ptr ()), xdg, config_file);
      /* C 1041-1044: the dead `WINDOWSNT` arm (the `APPDATA` fallback).  */
    }

  if config.is_null ()
    { return false; }

  let mut dotted: *mut c_char = std::ptr::null_mut ();
  let mut dottedsize: size_t = 0;
  let dottedlen: ssize_t = getline (&raw mut dotted, &raw mut dottedsize, config);
  let mut port: *mut c_char =
    if dottedlen < 0 { std::ptr::null_mut () } else { strchr (dotted, b':' as c_int) };
  if !port.is_null ()
    { *port = 0; port = port.offset (1); }
  else
    {
      message (true, c"%s: invalid configuration info\n".as_ptr (), progname);
      exit (EXIT_FAILURE);
    }

  *server = sockaddr_in
    {
      sin_len: 0,                     /* C's compound literal zero-fills the rest */
      sin_family: AF_INET as u8,
      sin_port: htons (atoi (port) as c_ushort),
      sin_addr: in_addr { s_addr: inet_addr (dotted) },
      sin_zero: [0; 8],
    };
  free (dotted as *mut c_void);

  if fread (authentication as *mut c_void, AUTH_KEY_LENGTH, 1, config) == 0
    {
      message (true, c"%s: cannot read authentication info\n".as_ptr (), progname);
      exit (EXIT_FAILURE);
    }

  fclose (config);

  return true;
}

/* Like socket (DOMAIN, TYPE, PROTOCOL), except arrange for the
   resulting file descriptor to be close-on-exec.  */

// C 1084-1085: `static HSOCKET cloexec_socket (int domain, int type, int protocol)`
unsafe fn cloexec_socket (domain: c_int, r#type: c_int, protocol: c_int) -> HSOCKET! ()
{
  /* C 1087-1088: `#ifdef SOCK_CLOEXEC` is dead (`SOCK_CLOEXEC` is not defined on this
     target -- measured); the live text is the `#else` arm C 1089-1096.  */
  let s: HSOCKET! () = socket (domain, r#type, protocol);
  /* C 1091-1094: `# ifndef WINDOWSNT` is live.  */
  {
    if 0 <= s
      { fcntl (s, F_SETFD, FD_CLOEXEC); }
  }
  return s;
}

/* Like connect(2), but possibly with a timeout, exiting if the timeout
   is reached.  */

// C 1102-1104: `static int connect_with_timeout (HSOCKET sockfd,
// const struct sockaddr *addr, int addr_len)`
unsafe fn connect_with_timeout (sockfd: HSOCKET! (), addr: *const sockaddr, addr_len: c_int)
  -> c_int
{
  if timeout == 0
    { return connect (sockfd, addr, addr_len as socklen_t); }

  /* C 1109-1218: `#ifndef WINDOWSNT` is live; the `#else` arm (the timer thread) is dead
     text.  */
  {
    let flags: c_int = fcntl (sockfd, F_GETFL);
    if flags == -1 || fcntl (sockfd, F_SETFL, flags | O_NONBLOCK) == -1
      { return -1; }

    let limit: intmax_t = if timeout < 0 { DEFAULT_TIMEOUT as intmax_t } else { timeout };
    let interval: timespec = make_timespec (0, (TIMESPEC_HZ / 20) as c_long);

    let mut rc: c_int = 0;
    let mut res: c_int = 0;
    let mut xerrno: c_int = 0;
    let mut fdset: fd_set = std::mem::zeroed ();
    let end: timespec = timespec_add (current_timespec (), make_timespec (limit, 0));

    'timeout: {
      'done: {
	loop
	  {
	    if connect (sockfd, addr, addr_len as socklen_t) == 0
	      { break 'done; }
	    else if *errno () == EAGAIN
		 || (EWOULDBLOCK != EAGAIN && *errno () == EWOULDBLOCK)
	      {
		/* For connections to Unix domain sockets under the Linux
		   kernel, we get EAGAIN if the listen queue is full.  */
		if timespec_cmp (end, current_timespec ()) <= 0
		  { break 'timeout; }
		pselect (0, std::ptr::null_mut (), std::ptr::null_mut (),
			 std::ptr::null_mut (), &raw const interval, std::ptr::null ());
	      }
	    else if *errno () != EINPROGRESS
	      {
		res = -1;
		xerrno = *errno ();
		break 'done;
	      }
	    else
	      { break; }
	  }

	loop
	  {
	    FD_ZERO (&raw mut fdset);
	    FD_SET (sockfd, &raw mut fdset);
	    let now: timespec = current_timespec ();
	    if timespec_cmp (end, now) <= 0
	      { break 'timeout; }
	    let remaining: timespec = timespec_sub (end, now);
	    rc = pselect (sockfd + 1, std::ptr::null_mut (), &raw mut fdset,
			  std::ptr::null_mut (), &raw const remaining, std::ptr::null ());
	    if rc != -1 || *errno () != EINTR
	      { break; }
	  }

	match rc
	  {
	    /* C 1160-1163: `case -1:` */
	    -1 =>
	      {
		res = -1;
		xerrno = *errno ();
		break 'done;
	      }
	    /* C 1164-1165: `case 0:` */
	    0 =>
	      {
		break 'timeout;
	      }
	    /* C 1166-1182: `default:` */
	    _ =>
	      {
		let mut xlen: socklen_t = std::mem::size_of::<c_int> () as socklen_t;
		if getsockopt (sockfd, SOL_SOCKET, SO_ERROR,
			       &raw mut xerrno as *mut c_void, &raw mut xlen) == -1
		  {
		    res = -1;
		    xerrno = *errno ();
		    break 'done;
		  }
		else if xerrno != 0
		  {
		    res = -1;
		    break 'done;
		  }
		/* C 1181's `break` leaves the `switch`; a `match` arm needs none.  */
	      }
	  }
      }

      /* C 1185-1194: ` done:` */
      /* If we can't switch the socket flags back we have to abort because
	 other code later on assumes it's blocking.  */
      if fcntl (sockfd, F_SETFL, flags) == -1
	{
	  message (true, c"%s: couldn't unset non-blocking on socket\n".as_ptr (),
		   progname);
	  exit (EXIT_FAILURE);
	}
      *errno () = xerrno;
      /* FIXME: Subtract time used up in this function from TIMEOUT?  */
      return res;
    }

    /* C 1222-1227: ` timeout:` */
    /* Timeout, but in the -a '' case we don't want to respond by starting
       another server, so exit immediately.  */
    message (true, c"%s: Connection timed out after %jd %s\n".as_ptr (),
	     progname, limit,
	     if limit == 1 { c"second".as_ptr () } else { c"seconds".as_ptr () });
    exit (EXIT_FAILURE)
  }
}

// C 1230-1231: `static HSOCKET set_tcp_socket (const char *local_server_file)`
unsafe fn set_tcp_socket (local_server_file: *const c_char) -> HSOCKET! ()
{
  /* C 1233-1236: `union { struct sockaddr_in in; struct sockaddr sa; } server;` -- the
     anonymous union is declared here, where C declares it; Rust needs the type named
     (PORTING.md 1.3/P-67), and the port names it after the variable it declares.  */
  #[repr(C)]
  #[derive(Clone, Copy)]
  union set_tcp_socket_server
  {
    r#in: sockaddr_in,  // C 1234
    sa: sockaddr,       // C 1235
  }
  const _: () = assert! (std::mem::size_of::<set_tcp_socket_server> () == 16);

  let mut server: set_tcp_socket_server = std::mem::zeroed ();
  let l_arg: linger = linger { l_onoff: 1, l_linger: 1 };
  let mut auth_buf: [c_char; AUTH_KEY_LENGTH] = [0; AUTH_KEY_LENGTH];

  if !get_server_config (local_server_file, &raw mut server.r#in,
			 &raw mut auth_buf as *mut c_char)
    { return INVALID_SOCKET; }

  if server.r#in.sin_addr.s_addr != inet_addr (c"127.0.0.1".as_ptr ()) && !quiet
    { message (false, c"%s: connected to remote socket at %s\n".as_ptr (),
	       progname, inet_ntoa (server.r#in.sin_addr)); }

  /* Open up an AF_INET socket.  */
  let s: HSOCKET! () = cloexec_socket (AF_INET, SOCK_STREAM, IPPROTO_TCP);
  if s < 0
    {
      /* Since we have an alternate to try out, this is not an error
	 yet; popping out a modal dialog at this stage would make -a
	 option totally useless for emacsclientw -- the user will
	 still get an error message if the alternate editor fails.  */
      sock_err_message (c"socket".as_ptr ());
      return INVALID_SOCKET;
    }

  /* Set up the socket.  */
  if connect_with_timeout (s, &raw const server.sa,
			   std::mem::size_of::<sockaddr_in> () as c_int) != 0
    {
      sock_err_message (c"connect".as_ptr ());
      CLOSE_SOCKET! (s);
      return INVALID_SOCKET;
    }

  /* The cast to 'const char *' is to avoid a compiler warning when
     compiling for MS-Windows sockets.  */
  let ret: c_int =
    setsockopt (s, SOL_SOCKET, SO_LINGER, &raw const l_arg as *const c_void,
		std::mem::size_of::<linger> () as socklen_t);
  if ret < 0
    { sock_err_message (c"setsockopt".as_ptr ()); }

  /* Send the authentication.  */
  send_to_emacs (s, c"-auth ".as_ptr ());
  send_to_emacs_len (s, &raw const auth_buf as *const c_char, AUTH_KEY_LENGTH as ptrdiff_t);
  send_to_emacs (s, c" ".as_ptr ());

  return s;
}

/* Return true if PREFIX is a prefix of STRING. */
// C 1283-1284: `static bool strprefix (const char *prefix, const char *string)`
unsafe fn strprefix (prefix: *const c_char, string: *const c_char) -> bool
{
  return !(strncmp (prefix, string, strlen (prefix)) != 0);
}

/* Get tty name and type.  If successful, store the type into
   *TTY_TYPE and the name into *TTY_NAME, and return true.
   Otherwise, fail if NOABORT is zero, or return false if NOABORT.  */

// C 1293-1294: `static bool find_tty (const char **tty_type, const char **tty_name,
// bool noabort)`
unsafe fn find_tty (tty_type: *mut *const c_char, tty_name: *mut *const c_char,
		    noabort: bool) -> bool
{
  let r#type: *const c_char = egetenv! (c"TERM".as_ptr ());
  let name: *const c_char = ttyname (STDOUT_FILENO);

  if name.is_null ()
    {
      if noabort
	{ return false; }
      message (true, c"%s: could not get terminal name\n".as_ptr (), progname);
      fail ();
    }

  if r#type.is_null ()
    {
      if noabort
	{ return false; }
      message (true, c"%s: please set the TERM variable to your terminal type\n".as_ptr (),
	       progname);
      fail ();
    }

  let inside_emacs: *const c_char = egetenv! (c"INSIDE_EMACS".as_ptr ());
  if !inside_emacs.is_null () && !strstr (inside_emacs, c",term:".as_ptr ()).is_null ()
      && strprefix (c"eterm".as_ptr (), r#type)
    {
      if noabort
	{ return false; }
      /* This causes nasty, MULTI_KBOARD-related input lockouts. */
      message (true, (c"%s: opening a frame in an Emacs term buffer is not supported\n").as_ptr (),
	       progname);
      fail ();
    }

  *tty_name = name;
  *tty_type = r#type;
  return true;
}

/* Return the process group if in the foreground, the negative of the
   process group if in the background, and zero if there is no
   foreground process group for the controlling terminal.
   Unfortunately, use of this function introduces an unavoidable race,
   since whether the process is in the foreground or background can
   change at any time.  */

// C 1341-1342: `static pid_t process_grouping (void)`
unsafe fn process_grouping () -> pid_t
{
  /* C 1344-1351: `#ifdef SOCKETS_IN_FILE_SYSTEM` is live.  */
  {
    let tcpgrp: pid_t = tcgetpgrp (STDOUT_FILENO);
    if 0 <= tcpgrp
      {
	let pgrp: pid_t = getpgrp ();
	return if tcpgrp == pgrp { pgrp } else { -pgrp };
      }
  }
  return 0;
}

/* Relative to the directory DIRFD, connect the socket file named ADDR
   to the socket S.  Return 0 if successful, -1 if DIRFD is not
   AT_FDCWD and DIRFD's permissions would allow a symlink attack, an
   errno otherwise.  */

// C 1375-1376: `static int connect_socket (int dirfd, char const *addr, int s, uid_t uid)`
unsafe fn connect_socket (dirfd: c_int, addr: *const c_char, s: c_int, uid: uid_t) -> c_int
{
  let mut sock_status: c_int = 0;

  let mut server: local_sockaddr = std::mem::zeroed ();
  /* C's `sizeof server.un.sun_path` -- the `sun_path` array's 104 bytes, which is the same
     number the enum constant `socknamesize` names.  */
  if socknamesize as size_t <= strlen (addr)
    { return ENAMETOOLONG; }
  server.un.sun_family = AF_UNIX as u8;
  strcpy (&raw mut server.un.sun_path as *mut c_char, addr);

  /* If -1, WDFD is not set yet.  If nonnegative, WDFD is a file
     descriptor for the initial working directory.  Otherwise -1 - WDFD is
     the error number for the initial working directory.  */
  static mut wdfd: c_int = -1;

  if dirfd != AT_FDCWD
    {
      /* Fail if DIRFD's permissions are bogus.  */
      let mut st: stat = std::mem::zeroed ();
      if fstat (dirfd, &raw mut st) != 0
	{ return *errno (); }
      if st.st_uid != uid || (st.st_mode & (S_IWGRP | S_IWOTH)) != 0
	{ return -1; }

      if wdfd == -1
	{
	  /* Save the initial working directory.  */
	  wdfd = open (c".".as_ptr (), O_PATH | O_CLOEXEC);
	  if wdfd < 0
	    { wdfd = -1 - *errno (); }
	}
      if wdfd < 0
	{ return -1 - wdfd; }
      if fchdir (dirfd) != 0
	{ return *errno (); }

      /* Fail if DIRFD has an ACL, which means its permissions are
	 almost surely bogus.  */
      let has_acl: c_int = file_has_acl (c".".as_ptr (), &raw const st);
      if has_acl != 0
	{ sock_status = if has_acl < 0 { *errno () } else { -1 }; }
    }

  if sock_status == 0
    { sock_status = if connect_with_timeout (s, &raw const server.sa,
					     std::mem::size_of::<sockaddr_un> () as c_int) == 0
	{ 0 } else { *errno () }; }

  /* Fail immediately if we cannot change back to the initial working
     directory, as that can mess up the rest of execution.  */
  if dirfd != AT_FDCWD && fchdir (wdfd) != 0
    {
      message (true, c"%s: .: %s\n".as_ptr (), progname, strerror (*errno ()));
      exit (EXIT_FAILURE);
    }

  return sock_status;
}

/* Reinstall for SIG the signal handler HANDLER if needed.  It is
   needed on a non-POSIX or traditional platform where an interrupt
   resets the signal handler to SIG_DFL.  */
// C 1449-1450: `static void reinstall_handler_if_needed (int sig, void (*handler) (int))`
unsafe fn reinstall_handler_if_needed (sig: c_int, handler: signal_handler_t)
{
  /* C 1452-1455 is `# ifndef SA_RESETHAND` / `signal (sig, handler);` / `# endif`, and
     `SA_RESETHAND` *is* defined on this target (0x0004), so the live body carries no
     statement.  The port keeps the dead arm's text under the feature that names the other
     side, so the body is not empty and the arm's text stays reviewable (P-51; the
     `lib/acl-internal.rs` 37 shape).  The `signal` mirror keeps the handler as a `usize`
     (signal.rs 165), which is what the arm's call has to spell.  */
  #[cfg (feature = "no_sa_resethand")]
  {
    signal (sig, handler.map_or (0, |h| h as usize));
  }
}

/* Flags for each signal, and handlers that set the flags.  */

// C 1460-1461: `static sig_atomic_t volatile got_sigcont, got_sigtstp, got_sigttou,
// got_sigwinch;`
static mut got_sigcont: sig_atomic_t = 0;
static mut got_sigtstp: sig_atomic_t = 0;
static mut got_sigttou: sig_atomic_t = 0;
static mut got_sigwinch: sig_atomic_t = 0;

// C 1463-1464: `static void handle_sigcont (int sig)`
unsafe extern "C" fn handle_sigcont (sig: c_int)
{
  got_sigcont = 1;
  reinstall_handler_if_needed (sig, Some (handle_sigcont));
}
// C 1469-1470: `static void handle_sigtstp (int sig)`
unsafe extern "C" fn handle_sigtstp (sig: c_int)
{
  got_sigtstp = 1;
  reinstall_handler_if_needed (sig, Some (handle_sigtstp));
}
// C 1475-1476: `static void handle_sigttou (int sig)`
unsafe extern "C" fn handle_sigttou (sig: c_int)
{
  got_sigttou = 1;
  reinstall_handler_if_needed (sig, Some (handle_sigttou));
}
// C 1481-1482: `static void handle_sigwinch (int sig)`
unsafe extern "C" fn handle_sigwinch (sig: c_int)
{
  got_sigwinch = 1;
  reinstall_handler_if_needed (sig, Some (handle_sigwinch));
}

/* Install for signal SIG the handler HANDLER.  However, if FLAG is
   non-null and if the signal is currently being ignored, do not
   install the handler and keep *FLAG zero.  */

// C 1492-1493: `static void install_handler (int sig, void (*handler) (int),
// sig_atomic_t volatile *flag)`
unsafe fn install_handler (sig: c_int, handler: signal_handler_t, flag: *mut sig_atomic_t)
{
  /* C 1495-1517: `# ifdef SA_RESETHAND` is live (0x0004); the `# else` arm is dead text.  */
  {
    if !flag.is_null ()
      {
	let mut oact: sigaction = std::mem::zeroed ();
	/* C's `oact.sa_handler == SIG_IGN` reads the `sigaction` union's `__sa_handler`
	   member, and `<signal.h>`'s `SIG_IGN` is the integer 1 cast to that handler type
	   (the `src/process.rs` 8697 shape).  */
	let sig_ign: signal_handler_t = std::mem::transmute (SIG_IGN as usize);
	if sigaction (sig, std::ptr::null (), &raw mut oact) == 0
	    && oact.__sigaction_u.__sa_handler == sig_ign
	  { return; }
      }
    let mut act: sigaction = std::mem::zeroed ();
    act.__sigaction_u.__sa_handler = handler;
    sigemptyset (&raw mut act.sa_mask);
    sigaction (sig, &raw const act, std::ptr::null_mut ());
  }
  /* C 1505-1516: the dead `# else` arm (the traditional `signal` calls).  */
}

/* Initial installation of signal handlers.  */

// C 1522-1523: `static void init_signals (void)`
unsafe fn init_signals ()
{
  install_handler (SIGCONT, Some (handle_sigcont), &raw mut got_sigcont);
  install_handler (SIGTSTP, Some (handle_sigtstp), &raw mut got_sigtstp);
  install_handler (SIGTTOU, Some (handle_sigttou), &raw mut got_sigttou);
  install_handler (SIGWINCH, Some (handle_sigwinch), &raw mut got_sigwinch);
  /* Don't mess with SIGINT and SIGQUIT, as Emacs has no way to
     determine which terminal the signal came from.  C-g is a normal
     input event on secondary terminals.  */
}

/* Act on delivered tty-related signal SIG that normally has handler
   HANDLER.  EMACS_SOCKET connects to Emacs.  */

// C 1537-1538: `static void act_on_tty_signal (int sig, void (*handler) (int),
// HSOCKET emacs_socket)`
unsafe fn act_on_tty_signal (sig: c_int, handler: signal_handler_t, emacs_socket: HSOCKET! ())
{
  /* Notify Emacs that we are going to sleep.  Normally the suspend is
     initiated by Emacs via server-handle-suspend-tty, but if the
     server gets out of sync with reality, we may get a SIGTSTP on
     C-z.  Handling this signal and notifying Emacs about it should
     get things under control again.  */
  send_to_emacs (emacs_socket, c"-suspend \n".as_ptr ());

  /* Execute the default action by temporarily changing handling to
     the default and resignaling.  */
  install_handler (sig, SIG_DFL, std::ptr::null_mut ());
  raise (sig);
  install_handler (sig, handler, std::ptr::null_mut ());
}

/* Act on delivered signals if possible.  If EMACS_SOCKET is valid,
   use it to communicate to Emacs.  */

// C 1557-1558: `static void act_on_signals (HSOCKET emacs_socket)`
unsafe fn act_on_signals (emacs_socket: HSOCKET! ())
{
  loop
    {
      let mut took_action: bool = false;

      if emacs_socket != INVALID_SOCKET
	{
	  if got_sigcont != 0
	    {
	      got_sigcont = 0;
	      took_action = true;
	      let grouping: pid_t = process_grouping ();
	      if grouping < 0
		{
		  if tty
		    {
		      /* Cancel the continue.  */
		      kill (grouping, SIGTTIN);
		    }
		}
	      else
		{ send_to_emacs (emacs_socket, c"-resume \n".as_ptr ()); }
	    }

	  if got_sigtstp != 0
	    {
	      got_sigtstp = 0;
	      took_action = true;
	      act_on_tty_signal (SIGTSTP, Some (handle_sigtstp), emacs_socket);
	    }
	  if got_sigttou != 0
	    {
	      got_sigttou = 0;
	      took_action = true;
	      act_on_tty_signal (SIGTTOU, Some (handle_sigttou), emacs_socket);
	    }
	}

      if emacs_pid != 0 && got_sigwinch != 0
	{
	  got_sigwinch = 0;
	  took_action = true;
	  kill (emacs_pid, SIGWINCH);
	}

      if !took_action
	{ break; }
    }
}

/* Given a local socket S, create in *SOCKNAME a name for a local socket
   and connect to that socket.  The first TMPDIRLEN bytes of *SOCKNAME are
   already initialized to be the name of a temporary directory.
   Use UID and SERVER_NAME to concoct the name.  Return 0 if
   successful, -1 if the socket's parent directory is not safe, and an
   errno if there is some other problem.  */

// C 1618-1620: `static int local_sockname (int s, char sockname[socknamesize], int tmpdirlen,
// uid_t uid, char const *server_name)`
unsafe fn local_sockname (s: c_int, sockname: *mut c_char, tmpdirlen: c_int,
			  uid: uid_t, server_name: *const c_char) -> c_int
{
  /* If ! (0 <= TMPDIRLEN && TMPDIRLEN < SOCKNAMESIZE) the truncated
     temporary directory name is already in SOCKNAME, so nothing more
     need be stored.  */
  if !(0 <= tmpdirlen && tmpdirlen < socknamesize)
    { return ENAMETOOLONG; }

  /* Put the full address name into the buffer, since the caller might
     need it for diagnostics.  But don't overrun the buffer.  */
  let uidmax: uintmax_t = uid as uintmax_t;
  let suffixlen: c_int = snprintf (sockname.offset (tmpdirlen as isize),
				   (socknamesize - tmpdirlen) as size_t,
				   c"/emacs%ju/%s".as_ptr (), uidmax, server_name);
  if !(0 <= suffixlen && suffixlen < socknamesize - tmpdirlen)
    { return ENAMETOOLONG; }

  /* Make sure the address's parent directory is not a symlink and is
     this user's directory and does not let others write to it; this
     fends off some symlink attacks.  To avoid races, keep the parent
     directory open while checking.  */
  let emacsdirend: *mut c_char =
    sockname.offset (tmpdirlen as isize).offset (suffixlen as isize)
	   .offset (-(strlen (server_name) as isize) - 1);
  *emacsdirend = 0;
  let dir: c_int = open (sockname, O_PATH | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
  *emacsdirend = b'/' as c_char;
  if dir < 0
    { return *errno (); }
  let sock_status: c_int = connect_socket (dir, server_name, s, uid);
  close (dir);
  return sock_status;
}

/* Create a local socket for SERVER_NAME and connect it to Emacs.  If
   SERVER_NAME is a file name component, the local socket name
   relative to a well-known location in a temporary directory.
   Otherwise, the local socket name is SERVER_NAME.  */

// C 1657-1658: `static HSOCKET set_local_socket (char const *server_name)`
unsafe fn set_local_socket (server_name: *const c_char) -> HSOCKET! ()
{
  let mut server: local_sockaddr = std::mem::zeroed ();
  let mut sock_status: c_int = 0;
  let mut sockname: *mut c_char = &raw mut server.un.sun_path as *mut c_char;
  let mut tmpdirlen: c_int = -1;
  let mut socknamelen: c_int = -1;
  let uid: uid_t = geteuid ();
  let mut tmpdir_used: bool = false;
  let s: c_int = cloexec_socket (AF_UNIX, SOCK_STREAM, 0);
  if s < 0
    {
      message (true, c"%s: can't create socket: %s\n".as_ptr (),
	       progname, strerror (*errno ()));
      fail ();
    }

  if !strchr (server_name, b'/' as c_int).is_null ()
      || (ISSLASH! (b'\\' as c_char) && !strchr (server_name, b'\\' as c_int).is_null ())
    {
      socknamelen = snprintf (sockname, socknamesize as size_t, c"%s".as_ptr (),
			      server_name);
      sock_status = if 0 <= socknamelen && socknamelen < socknamesize
	{ connect_socket (AT_FDCWD, sockname, s, 0) } else { ENAMETOOLONG };
    }
  else
    {
      /* C 1685-1743: `#ifndef HAVE_ANDROID` is live; the `#else` arm is dead text.  */
      {
	/* socket_name is a file name component.  */
	let xdg_runtime_dir: *const c_char = egetenv! (c"XDG_RUNTIME_DIR".as_ptr ());
	if !xdg_runtime_dir.is_null ()
	  {
	    socknamelen = snprintf (sockname, socknamesize as size_t,
				    c"%s/emacs/%s".as_ptr (), xdg_runtime_dir,
				    server_name);
	    sock_status = if 0 <= socknamelen && socknamelen < socknamesize
	      { connect_socket (AT_FDCWD, sockname, s, 0) } else { ENAMETOOLONG };
	  }
	else
	  {
	    let tmpdir: *const c_char = egetenv! (c"TMPDIR".as_ptr ());
	    if !tmpdir.is_null ()
	      { tmpdirlen = snprintf (sockname, socknamesize as size_t,
				      c"%s".as_ptr (), tmpdir); }
	    else
	      {
		/* C 1703-1711: `# ifdef DARWIN_OS` is live, and its inner
		   `#  ifndef _CS_DARWIN_USER_TEMP_DIR` arm is dead because `<unistd.h>`
		   defines it.  */
		{
		  let n: size_t = confstr (_CS_DARWIN_USER_TEMP_DIR, sockname,
					   socknamesize as size_t);
		  /* C's `(size_t) -1`.  */
		  if 0 < n && n < usize::MAX
		    { tmpdirlen = min! (n - 1, socknamesize as size_t) as c_int; }
		}
		if tmpdirlen < 0
		  { tmpdirlen = snprintf (sockname, socknamesize as size_t,
					  c"/tmp".as_ptr ()); }
	      }

	    sock_status = local_sockname (s, sockname, tmpdirlen, uid, server_name);
	    tmpdir_used = true;
	  }
      }
    }

  if sock_status == 0
    { return s; }

  if sock_status == ENAMETOOLONG
    {
      message (true, c"%s: socket-name %s... too long\n".as_ptr (), progname, sockname);
      fail ();
    }

  if tmpdir_used
    {
      /* See whether LOGNAME or USER exist and differ from
	 our euid.  If so, look for a socket based on the UID
	 associated with the name.  This is reminiscent of the logic
	 that init_editfns uses to set the global Vuser_full_name.  */

      let mut user_name: *const c_char = egetenv! (c"LOGNAME".as_ptr ());

      if user_name.is_null ()
	{ user_name = egetenv! (c"USER".as_ptr ()); }

      if !user_name.is_null ()
	{
	  let pw: *mut passwd = getpwnam (user_name);

	  if !pw.is_null () && (*pw).pw_uid != uid
	    {
	      /* We're running under su, apparently. */
	      sock_status = local_sockname (s, sockname, tmpdirlen, (*pw).pw_uid,
					    server_name);
	      if sock_status == 0
		{ return s; }
	      if sock_status == ENAMETOOLONG
		{
		  message (true, c"%s: socket-name %s... too long\n".as_ptr (),
			   progname, sockname);
		  exit (EXIT_FAILURE);
		}
	    }
	}
    }

  close (s);

  if sock_status == -1
    { message (true,
	       c"%s: Invalid permissions on parent directory of socket: %s\n".as_ptr (),
	       progname, sockname); }
  else if sock_status == ENOENT
    {
      if tmpdir_used
	{
	  let id: uintmax_t = uid as uintmax_t;
	  let mut sockdirname: [c_char; socknamesize as usize] =
	    [0; socknamesize as usize];
	  let sockdirnamelen: c_int =
	    snprintf (&raw mut sockdirname as *mut c_char, socknamesize as size_t,
		      c"/run/user/%ju".as_ptr (), id);
	  if 0 <= sockdirnamelen && sockdirnamelen < socknamesize
	      && faccessat (AT_FDCWD, &raw const sockdirname as *const c_char, X_OK,
			    AT_EACCESS) == 0
	    { message (true,
		       c"%s: Should XDG_RUNTIME_DIR='%s' be in the environment?\n%s: (Be careful: XDG_RUNTIME_DIR is security-related.)\n".as_ptr (),
		       progname, sockdirname, progname); }
	}

      /* If there's an alternate editor and the user has requested
	 --quiet, don't output the warning. */
      if !quiet || alternate_editor.is_null ()
	{
	  message (true,
		   c"%s: can't find socket; have you started the server?\n%s: To start the server in Emacs, type \"M-x server-start\".\n".as_ptr (),
		   progname, progname);
	}
    }
  else
    { message (true, c"%s: can't connect to %s: %s\n".as_ptr (),
	       progname, sockname, strerror (sock_status)); }

  return INVALID_SOCKET;
}

// C 1830-1831: `static HSOCKET set_socket (bool no_exit_if_error)`
unsafe fn set_socket (no_exit_if_error: bool) -> HSOCKET! ()
{
  let mut s: HSOCKET! () = 0;   /* C leaves it indeterminate until a branch assigns it */
  let mut local_server_file: *const c_char = server_file;

  INITIALIZE! ();

  /* C 1838-1852: `#ifdef SOCKETS_IN_FILE_SYSTEM` is live.  */
  {
    if socket_name.is_null ()
      { socket_name = egetenv! (c"EMACS_SOCKET_NAME".as_ptr ()); }

    if !socket_name.is_null ()
      {
	/* Explicit --socket-name argument, or environment variable.  */
	s = set_local_socket (socket_name);
	if s != INVALID_SOCKET || no_exit_if_error
	  { return s; }
	message (true, c"%s: error accessing socket \"%s\"\n".as_ptr (),
		 progname, socket_name);
	exit (EXIT_FAILURE);
      }
  }

  /* Explicit --server-file arg or EMACS_SERVER_FILE variable.  */
  if local_server_file.is_null ()
    { local_server_file = egetenv! (c"EMACS_SERVER_FILE".as_ptr ()); }

  if !local_server_file.is_null ()
    {
      s = set_tcp_socket (local_server_file);
      if s != INVALID_SOCKET || no_exit_if_error
	{ return s; }

      message (true, c"%s: error accessing server file \"%s\"\n".as_ptr (),
	       progname, local_server_file);
      exit (EXIT_FAILURE);
    }

  /* C 1869-1874: `#ifdef SOCKETS_IN_FILE_SYSTEM` is live.  */
  {
    /* Implicit local socket.  */
    s = set_local_socket (c"server".as_ptr ());
    if s != INVALID_SOCKET
      { return s; }
  }

  /* Implicit server file.  */
  s = set_tcp_socket (c"server".as_ptr ());
  if s != INVALID_SOCKET || no_exit_if_error
    { return s; }

  /* No implicit or explicit socket, and no alternate editor.  */
  message (true, c"%s: No socket or alternate editor.  Please use:\n\n\t--socket-name\n\t--server-file      (or environment variable EMACS_SERVER_FILE)\n\t--alternate-editor (or environment variable ALTERNATE_EDITOR)\n".as_ptr (),
	   progname);
  exit (EXIT_FAILURE);
}

/* Start the emacs daemon and try to connect to it.  */

// C 1985-1986: `static HSOCKET start_daemon_and_retry_set_socket (void)`
unsafe fn start_daemon_and_retry_set_socket () -> HSOCKET! ()
{
  /* C 1988-2036: `#ifndef WINDOWSNT` is live; the `#else` arm is dead text.  */
  {
    let dpid: pid_t = fork ();

    if dpid > 0
      {
	let mut status: c_int = 0;
	let w: pid_t = waitpid (dpid, &raw mut status, WUNTRACED | WCONTINUED);

	if w < 0 || !WIFEXITED (status) || WEXITSTATUS (status) != 0
	  {
	    message (true, c"Error: Could not start the Emacs daemon\n".as_ptr ());
	    exit (EXIT_FAILURE);
	  }

	/* Try connecting, the daemon should have started by now.  */
	if !quiet
	  { message (true,
		     c"Emacs daemon should have started, trying to connect again\n".as_ptr ()); }
      }
    else if dpid < 0
      {
	fprintf (stderr, c"Error: Cannot fork!\n".as_ptr ());
	exit (EXIT_FAILURE);
      }
    else
      {
	let mut emacs: [u8; 6] = *b"emacs\0";
	let mut daemon_option: [u8; 9] = *b"--daemon\0";
	let mut d_argv: [*mut c_char; 3] = [std::ptr::null_mut (); 3];
	d_argv[0] = &raw mut emacs as *mut c_char;
	d_argv[1] = &raw mut daemon_option as *mut c_char;
	d_argv[2] = std::ptr::null_mut ();
	/* C 2022-2032: `# ifdef SOCKETS_IN_FILE_SYSTEM` is live.  */
	{
	  if !socket_name.is_null ()
	    {
	      /* Pass  --daemon=socket_name as argument.  */
	      let deq: *const c_char = c"--daemon=".as_ptr ();
	      let daemon_arg: *mut c_char =
		xmalloc (strlen (deq) + strlen (socket_name) + 1) as *mut c_char;
	      strcpy (stpcpy (daemon_arg, deq), socket_name);
	      d_argv[1] = daemon_arg;
	    }
	}
	execvp (c"emacs".as_ptr (), &raw const d_argv as *const *mut c_char);
	message (true, c"%s: error starting emacs daemon\n".as_ptr (), progname);
	exit (EXIT_FAILURE);
      }
  }

  let emacs_socket: HSOCKET! () = set_socket (true);
  if emacs_socket == INVALID_SOCKET
    {
      message (true,
	       c"Error: Cannot connect even after starting the Emacs daemon\n".as_ptr ());
      exit (EXIT_FAILURE);
    }
  return emacs_socket;
}

/* Set SOCKET's timeout to SECONDS.
   If SECONDS is zero or out of range, do not set the timeout.
   Silently ignore errors, as POSIX says it is implementation-defined as
   to whether SO_RCVTIMEO works.  Although we could fall back on
   non-blocking I/O if setsockopt fails, it's not worth the trouble.  */
// C 2118-2119: `static void set_socket_timeout (HSOCKET socket, intmax_t seconds)`
unsafe fn set_socket_timeout (socket: HSOCKET! (), seconds: intmax_t)
{
  if seconds <= 0
    { return; }

  /* C 2124-2136: `#ifndef WINDOWSNT` is live; the `#else` arm is dead text.  */
  {
    /* C's local shadows the file-scope `timeout`; Rust cannot shadow a static with a `let`,
       so the port spells the trailing-underscore form (token-equal; the `src/editfns.rs`
       3483 precedent).  */
    let mut timeout_: timeval = std::mem::zeroed ();
    let (ckd_add_value, ckd_add_overflow) = seconds.overflowing_add (0);
    timeout_.tv_sec = ckd_add_value;
    if ckd_add_overflow
      { return; }
    timeout_.tv_usec = 0;
    setsockopt (socket, SOL_SOCKET, SO_RCVTIMEO, &raw const timeout_ as *const c_void,
		std::mem::size_of::<timeval> () as socklen_t);
  }
}

// C 2139-2140: `static bool check_socket_timeout (ssize_t rl)`
unsafe fn check_socket_timeout (rl: ssize_t) -> bool
{
  /* C 2142-2150: `#ifndef WINDOWSNT` is live; the `#else` arm is dead text.  */
  {
    if rl != -1
      { return false; }
    return *errno () == EAGAIN
      || (EWOULDBLOCK != EAGAIN && *errno () == EWOULDBLOCK);
  }
}

// C 2153-2154: `int main (int argc, char **argv)` -- the other of the file's two exported
// symbols (the reference object defines `_main`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main (argc: c_int, argv: *mut *mut c_char) -> c_int
{
  main_argc = argc;
  main_argv = argv;
  progname = if !(*argv).is_null () { *argv } else { c"emacsclient".as_ptr () };

  let mut rl: ssize_t = 0;
  let mut skiplf: bool = true;
  let mut recv_buf: *mut c_char = std::ptr::null_mut ();
  let mut recv_bufsize: ptrdiff_t = 0;

  let mut exit_status: c_int = EXIT_SUCCESS;

  /* C 2167-2173: the dead `HAVE_NTGUI` arm (`w32_set_user_model_id`); recorded, not
     transcribed.  */

  /* Process options.  */
  decode_options (argc, argv);

  if !(optind < argc || eval || create_frame)
    {
      message (true, c"%s: file name or argument required\nTry '%s --help' for more information\n".as_ptr (),
	       progname, progname);
      exit (EXIT_FAILURE);
    }

  /* C 2186-2193: `#ifdef SOCKETS_IN_FILE_SYSTEM` is live.  */
  {
    if tty
      {
	let grouping: pid_t = process_grouping ();
	if grouping < 0
	  { kill (grouping, SIGTTIN); }
      }
  }

  /* If alternate_editor is the empty string, start the emacs daemon
     in case of failure to connect.  */
  let start_daemon_if_needed: bool =
    !alternate_editor.is_null () && *alternate_editor == 0;

  let mut emacs_socket: HSOCKET! () =
    set_socket (!alternate_editor.is_null () || start_daemon_if_needed);
  if emacs_socket == INVALID_SOCKET
    {
      if !start_daemon_if_needed
	{ fail (); }

      emacs_socket = start_daemon_and_retry_set_socket ();
    }

  let cwd: *mut c_char = get_current_dir_name ();
  if cwd.is_null ()
    {
      message (true, c"%s: %s\n".as_ptr (), progname,
	       c"Cannot get current working directory".as_ptr ());
      fail ();
    }

  /* C 2217-2220: the dead `HAVE_NTGUI` arm (`w32_give_focus`); recorded, not transcribed.  */

  /* Send over our environment and current directory. */
  if create_frame
    {
      /* C 2225: `for (char *const *e = environ; *e; e++)`.  */
      let mut e: *const *mut c_char = environ;
      while !(*e).is_null ()
	{
	  send_to_emacs (emacs_socket, c"-env ".as_ptr ());
	  quote_argument (emacs_socket, *e);
	  send_to_emacs (emacs_socket, c" ".as_ptr ());
	  e = e.offset (1);
	}
    }
  send_to_emacs (emacs_socket, c"-dir ".as_ptr ());
  if !tramp_prefix.is_null ()
    { quote_argument (emacs_socket, tramp_prefix); }
  quote_argument (emacs_socket, cwd);
  free (cwd as *mut c_void);
  send_to_emacs (emacs_socket, c"/".as_ptr ());
  send_to_emacs (emacs_socket, c" ".as_ptr ());

  /* C 2240: ` retry:` -- the labelled loop the file's one backward `goto` (C 2455) reaches.  */
  'retry: loop
    {
      if nowait
	{ send_to_emacs (emacs_socket, c"-nowait ".as_ptr ()); }

      if !create_frame || reuse_frame
	{ send_to_emacs (emacs_socket, c"-current-frame ".as_ptr ()); }

      if !display.is_null ()
	{
	  send_to_emacs (emacs_socket, c"-display ".as_ptr ());
	  quote_argument (emacs_socket, display);
	  send_to_emacs (emacs_socket, c" ".as_ptr ());
	}

      if !parent_id.is_null ()
	{
	  send_to_emacs (emacs_socket, c"-parent-id ".as_ptr ());
	  quote_argument (emacs_socket, parent_id);
	  send_to_emacs (emacs_socket, c" ".as_ptr ());
	}

      if !frame_parameters.is_null () && create_frame
	{
	  send_to_emacs (emacs_socket, c"-frame-parameters ".as_ptr ());
	  quote_argument (emacs_socket, frame_parameters);
	  send_to_emacs (emacs_socket, c" ".as_ptr ());
	}

      /* Unless we are certain we don't want to occupy the tty, send our
	 tty information to Emacs.  For example, in daemon mode Emacs may
	 need to occupy this tty if no other frame is available.  */
      if create_frame || !eval
	{
	  let mut tty_type: *const c_char = std::ptr::null ();
	  let mut tty_name: *const c_char = std::ptr::null ();

	  if find_tty (&raw mut tty_type, &raw mut tty_name, !tty)
	    {
	      /* Install signal handlers before opening a frame on the
		 current tty.  */
	      init_signals ();

	      send_to_emacs (emacs_socket, c"-tty ".as_ptr ());
	      quote_argument (emacs_socket, tty_name);
	      send_to_emacs (emacs_socket, c" ".as_ptr ());
	      quote_argument (emacs_socket, tty_type);
	      send_to_emacs (emacs_socket, c" ".as_ptr ());
	    }
	}

      if create_frame && !tty
	{ send_to_emacs (emacs_socket, c"-window-system ".as_ptr ()); }

      if optind < argc
	{
	  /* C 2294: `for (int i = optind; i < argc; i++)` -- the update sits at the end of
	     the body and on both `continue` paths (the for's `i++` runs before re-testing).  */
	  let mut i: c_int = optind;
	  while i < argc
	    {
	      if eval
		{
		  /* Don't prepend cwd or anything like that.  */
		  send_to_emacs (emacs_socket, c"-eval ".as_ptr ());
		  quote_argument (emacs_socket, *argv.offset (i as isize));
		  send_to_emacs (emacs_socket, c" ".as_ptr ());
		  i += 1;
		  continue;
		}

	      let mut p: *mut c_char = *argv.offset (i as isize);
	      if *p == b'+' as c_char
		{
		  let mut c: c_uchar = 0;
		  loop
		    {
		      p = p.offset (1);
		      c = *p as c_uchar;
		      if !(c_isdigit (c as c_int) || c == b':' as c_uchar)
			{ break; }
		    }

		  if c == 0
		    {
		      send_to_emacs (emacs_socket, c"-position ".as_ptr ());
		      quote_argument (emacs_socket, *argv.offset (i as isize));
		      send_to_emacs (emacs_socket, c" ".as_ptr ());
		      i += 1;
		      continue;
		    }
		}
	      /* C 2322-2341: the dead `WINDOWSNT` arm (the drive-letter `else if`); its text
		 is not compiled, so nothing is left between the `if` above and the body
		 below.  */

	      send_to_emacs (emacs_socket, c"-file ".as_ptr ());
	      if !tramp_prefix.is_null () && IS_ABSOLUTE_FILE_NAME! (*argv.offset (i as isize))
		{ quote_argument (emacs_socket, tramp_prefix); }
	      quote_argument (emacs_socket, *argv.offset (i as isize));
	      send_to_emacs (emacs_socket, c" ".as_ptr ());

	      /* C's for-update `i++`.  */
	      i += 1;
	    }
	}
      else if eval
	{
	  /* Read expressions interactively.  */
	  let mut line: *mut c_char = std::ptr::null_mut ();
	  let mut linesize: size_t = 0;
	  let mut len: ssize_t = getline (&raw mut line, &raw mut linesize, stdin);
	  while 0 <= len
	    {
	      send_to_emacs (emacs_socket, c"-eval ".as_ptr ());
	      quote_argument_len (emacs_socket, line, len as ptrdiff_t);
	      len = getline (&raw mut line, &raw mut linesize, stdin);
	    }
	  free (line as *mut c_void);
	  send_to_emacs (emacs_socket, c" ".as_ptr ());
	}

      send_to_emacs (emacs_socket, c"\n".as_ptr ());

      /* Wait for an answer. */
      if !eval && !tty && !nowait && !quiet && 0 <= process_grouping ()
	{
	  printf (c"Waiting for Emacs...".as_ptr ());
	  skiplf = false;
	}
      fflush (stdout);

      set_socket_timeout (emacs_socket,
			  if timeout < 0 { DEFAULT_TIMEOUT as intmax_t } else { timeout });
      let mut saw_response: bool = false;
      let mut nrecv: ptrdiff_t = 0;

      /* Now, wait for an answer and print any messages.  */
      while exit_status == EXIT_SUCCESS
	{
	  let mut retry: bool = true;
	  let mut msg_showed: bool = quiet;
	  loop
	    {
	      act_on_signals (emacs_socket);
	      if nrecv == recv_bufsize
		{
		  const DEFAULT_RECV_BUFSIZE: ptrdiff_t = 4096;
		  recv_bufsize = (recv_bufsize + (recv_bufsize >> 1)
				  + DEFAULT_RECV_BUFSIZE);
		  recv_buf = xrealloc (recv_buf as *mut c_void,
				       recv_bufsize as size_t) as *mut c_char;
		}
	      rl = recv (emacs_socket, recv_buf.offset (nrecv) as *mut c_void,
			 (recv_bufsize - nrecv) as size_t, 0);
	      retry = check_socket_timeout (rl);
	      if retry && !saw_response
		{
		  if timeout > 0
		    {
		      /* Don't retry if we were given a --timeout flag.  */
		      fprintf (stderr,
			       c"\nServer not responding; timed out after %jd seconds".as_ptr (),
			       timeout);
		      retry = false;
		    }
		  else if !msg_showed
		    {
		      msg_showed = true;
		      fprintf (stderr,
			       c"\nServer not responding; use Ctrl+C to break".as_ptr ());
		    }
		}
	      /* C 2410-2411's do-while condition: `while ((rl < 0 && errno == EINTR) ||
		 retry)`.  */
	      if !((rl < 0 && *errno () == EINTR) || retry)
		{ break; }
	    }

	  if rl <= 0
	    { break; }

	  nrecv += rl as ptrdiff_t;
	  saw_response = true;

	  /* Loop over all NL-terminated messages.  */
	  let mut p: *mut c_char = recv_buf;
	  let mut end_p: *mut c_char = p;
	  while end_p < recv_buf.offset (nrecv)
	    {
	      /* An unquoted newline ends a server command.  Keep reading,
		 possibly growing the buffer, until a buffer with a newline
		 is received.  This handles commands with arbitrary-long
		 arguments (actually needed in 'print' and 'error' commands,
		 which are followed by strings).  */
	      end_p = memchr (p as *const c_void, b'\n' as c_int,
			      recv_buf.offset (nrecv).offset_from (p) as size_t)
		      as *mut c_char;
	      if end_p.is_null ()
		{ break; }
	      *end_p = 0;
	      end_p = end_p.offset (1);

	      if strprefix (c"-emacs-pid ".as_ptr (), p)
		{
		  /* -emacs-pid PID: The process id of the Emacs process. */
		  emacs_pid = strtoumax (p.offset (strlen (c"-emacs-pid".as_ptr ()) as isize),
					 std::ptr::null_mut (), 10) as pid_t;
		}
	      else if strprefix (c"-window-system-unsupported ".as_ptr (), p)
		{
		  /* -window-system-unsupported: the server was built without support
		     for whatever window system we tried.  Try the alternate
		     display, or, failing that, try the terminal.  */
		  if !alt_display.is_null ()
		    {
		      display = alt_display;
		      alt_display = std::ptr::null ();
		    }
		  else
		    {
		      nowait = false;
		      tty = true;
		    }

		  /* This discards any remaining data in recv_buf.  */
		  continue 'retry;
		}
	      else if strprefix (c"-print ".as_ptr (), p)
		{
		  /* -print STRING: Print STRING, preceded by a newline, on
		     the terminal. */
		  if !suppress_output
		    {
		      let str_: *mut c_char =
			unquote_argument (p.offset (strlen (c"-print ".as_ptr ()) as isize));
		      printf (c"\n%s".as_ptr ().offset (if skiplf { 1 } else { 0 }), str_);
		      if *str_ != 0
			{ skiplf = *str_.offset (strlen (str_) as isize - 1) == b'\n' as c_char; }
		    }
		}
	      else if strprefix (c"-print-nonl ".as_ptr (), p)
		{
		  /* -print-nonl STRING: Print STRING on the terminal
		     without a preceding newline.  Used to continue a
		     preceding -print command.  Nowadays used only for
		     servers in Emacs versions before 31.  */
		  if !suppress_output
		    {
		      let str_: *mut c_char =
			unquote_argument (p.offset (strlen (c"-print-nonl ".as_ptr ())
						    as isize));
		      printf (c"%s".as_ptr (), str_);
		      if *str_ != 0
			{ skiplf = *str_.offset (strlen (str_) as isize - 1) == b'\n' as c_char; }
		    }
		}
	      else if strprefix (c"-error ".as_ptr (), p)
		{
		  /* -error DESCRIPTION: Signal an error on the terminal. */
		  let str_: *mut c_char =
		    unquote_argument (p.offset (strlen (c"-error ".as_ptr ()) as isize));
		  if !skiplf
		    { printf (c"\n".as_ptr ()); }
		  message (true, c"*ERROR*: %s".as_ptr (), str_);
		  if *str_ != 0
		    { skiplf = *str_.offset (strlen (str_) as isize - 1) == b'\n' as c_char; }
		  exit_status = EXIT_FAILURE;
		}
	      /* C 2494-2503: `#ifndef WINDOWSNT` is live.  */
	      else if strprefix (c"-suspend ".as_ptr (), p)
		{
		  /* -suspend: Suspend this terminal, i.e., stop the process. */
		  if !skiplf
		    { printf (c"\n".as_ptr ()); }
		  skiplf = true;
		  kill (0, SIGSTOP);
		}
	      else
		{
		  /* Unknown command. */
		  printf (c"\n*ERROR*: Unknown message: %s\n".as_ptr ()
			  .offset (if skiplf { 1 } else { 0 }), p);
		  skiplf = true;
		}
	    }

	  nrecv -= p.offset_from (recv_buf);
	  memmove (recv_buf as *mut c_void, p as *const c_void, nrecv as size_t);
	}

      /* C's control reaches the statement after the `while` here; the port's `'retry` loop
	 leaves at this point (PORTING.md 2.2).  */
      break;
    }

  if !skiplf && 0 <= process_grouping ()
    { printf (c"\n".as_ptr ()); }

  if rl < 0
    { exit_status = EXIT_FAILURE; }

  CLOSE_SOCKET! (emacs_socket);
  return exit_status;
}
