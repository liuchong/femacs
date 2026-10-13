// femacs: port of lib/unistd.in.h @ fd7afced403 (gnulib's <unistd.h>; PORTING.md P-26).
//
// The reference is a gnulib template (`@GUARD_PREFIX@`, `@PRAGMA_SYSTEM_HEADER@`,
// `@INCLUDE_NEXT@`, `@NEXT_UNISTD_H@`, `@GNULIB_*@`, `@REPLACE_*@`, `@HAVE_*@`), so
// tools/check-cond-eval.py cannot decide its conditions and it stays off the verified list:
// tools/extract-abi.py reads every arm as live, and this mirror names every macro the template
// can define (91 names) - the endian.in.rs/time.in.rs shape.  The frozen substitution is the
// generated build/emacs-ref/lib/unistd.h, and what that configuration actually compiles was
// measured by preprocessing it with the frozen compiler:
//
//     $ printf '#include <unistd.h>\n' > /tmp/u.c
//     $ clang -E -dD -I build/emacs-ref/lib -I build/emacs-ref/src -I emacs/src -I emacs/lib \
//         -include config.h /tmp/u.c | grep '^#define'
//     #define STDIN_FILENO 0                  #define _GL_UNISTD_INLINE _GL_INLINE
//     #define STDOUT_FILENO 1                 #define _GL_GETPAGESIZE_INLINE _GL_INLINE
//     #define STDERR_FILENO 2                 #define _GL_USE_CRT_EXTERNS
//     #define environ (*_NSGetEnviron ())     #define pipe2 rpl_pipe2
//     int rpl_pipe2 (int fd[2], int flags);   <- the one declaration of gnulib's own text
//
// So the live header is the system <unistd.h> the template includes next (its C 32/33), plus
// gnulib's `pipe2` replacement, the macOS `environ` macro, the three descriptor constants and
// `_GL_USE_CRT_EXTERNS`; every other `@GNULIB_*@`/`@REPLACE_*@` arm is dead here.  The port has
// no `#include_next`, so this mirror plays both halves: the *system* entries the ported units
// name are declared below (they are what the frozen header's `_GL_CXXALIAS_SYS` reaches), and
// the template's own names are carried - the live ones with the value the frozen substitution
// gives, the dead arms as text under the feature naming the C condition (P-51) - because the
// extractor asks for every name and cannot evaluate the template's conditions.
//
// `ssize_t` is the frozen target's `long` (`sizeof (ssize_t) == 8`, measured).

include!("sys_types.in.rs");   // <sys/types.h>

/// The frozen target's `ssize_t` (`long`, 8 bytes).
pub type ssize_t = c_long;

/* C declares `chmod` in both <unistd.h> and <sys/stat.h>; the port keeps one declaration per
   name (PORTING.md 0.1-3), so it lives in sys_stat.in.rs and is *not* repeated here.  */
unsafe extern "C" {
  /* **`readlink`, `symlink` and `readlinkat` are three names `src/w32.c` defines** (C 6331,
     6118 and 6475), so - like the `getwd`/`getppid`/`getgid`/`getegid`/`fchown` rows above -
     they are gated out of the `windowsnt` arm: a declaration at this scope is an `E0428`
     against those definitions (`docs/ports/w32.md` Owed 17 measured the first two,
     Owed 19 the third).  */
  #[cfg (not (feature = "windowsnt"))]
  pub fn readlink (file: *const c_char, buf: *mut c_char, bufsize: usize) -> ssize_t;
  #[cfg (not (feature = "windowsnt"))]
  pub fn symlink (contents: *const c_char, name: *const c_char) -> c_int;
  #[cfg (not (feature = "windowsnt"))]
  pub fn readlinkat (fd: c_int, file: *const c_char, buf: *mut c_char, bufsize: usize)
    -> ssize_t;
  pub fn unlink (file: *const c_char) -> c_int;
  pub fn unlinkat (fd: c_int, file: *const c_char, flag: c_int) -> c_int;
  pub fn rmdir (file: *const c_char) -> c_int;
  pub fn link (from: *const c_char, to: *const c_char) -> c_int;
  pub fn linkat (fd1: c_int, from: *const c_char, fd2: c_int, to: *const c_char, flag: c_int)
    -> c_int;
  pub fn chdir (file: *const c_char) -> c_int;
  pub fn fchdir (fd: c_int) -> c_int;
  pub fn getcwd (buf: *mut c_char, size: usize) -> *mut c_char;
  /// The frozen SDK's `<unistd.h>` 578: `char *getwd (char *)` - declared there with
  /// `__POSIX_C_DEPRECATED(200112L)` (measured on this target's header, not recalled);
  /// `src/msdos.c`'s DJGPP arm is its only caller in the reference (status 3.479).
  /* The six names `lib-src/ntlib.h` (C 29-38) declares for a `WINDOWSNT` build - `sleep`,
     `getwd`, `getppid`, `getegid`, `getgid`, `fchown` - are masked here **for that build
     only**: C lets one name carry two compatible declarations (Windows spells them with
     `int`/`unsigned`, this mirror with `pid_t`/`gid_t`/`uid_t`, which the Windows headers
     make the same types), Rust does not, and the platform header's own spelling is the one
     a Windows translation unit should see.  Measured before the mask: a `windowsnt` arm
     that includes both mirrors (e.g. `lib-src/pop.rs`) reported six `E0428`s; after it the
     arm compiles (status 3.483, docs/ports/pop.md Owed 7).  */
  #[cfg (not (feature = "windowsnt"))]
  pub fn getwd (buf: *mut c_char) -> *mut c_char;
  pub fn access (file: *const c_char, mode: c_int) -> c_int;
  pub fn close (fd: c_int) -> c_int;
  pub fn read (fd: c_int, buf: *mut c_void, n: usize) -> ssize_t;
  pub fn write (fd: c_int, buf: *const c_void, n: usize) -> ssize_t;
  pub fn lseek (fd: c_int, offset: off_t, whence: c_int) -> off_t;
  pub fn ftruncate (fd: c_int, length: off_t) -> c_int;
  pub fn truncate (file: *const c_char, length: off_t) -> c_int;
  pub fn fsync (fd: c_int) -> c_int;
  pub fn dup (fd: c_int) -> c_int;
  pub fn dup2 (fd: c_int, fd2: c_int) -> c_int;
  pub fn pipe (fildes: *mut c_int) -> c_int;
  pub fn getpid () -> pid_t;
  #[cfg (not (feature = "windowsnt"))]
  pub fn getppid () -> pid_t;
  pub fn getuid () -> uid_t;
  pub fn geteuid () -> uid_t;
  #[cfg (not (feature = "windowsnt"))]
  pub fn getgid () -> gid_t;
  #[cfg (not (feature = "windowsnt"))]
  pub fn getegid () -> gid_t;
  pub fn chown (file: *const c_char, owner: uid_t, group: gid_t) -> c_int;
  #[cfg (not (feature = "windowsnt"))]
  pub fn fchown (fd: c_int, owner: uid_t, group: gid_t) -> c_int;
  pub fn lchown (file: *const c_char, owner: uid_t, group: gid_t) -> c_int;
  pub fn isatty (fd: c_int) -> c_int;
  /* unsigned int alarm (unsigned int);  (src/atimer.c's two live calls)  */
  pub fn alarm (seconds: c_uint) -> c_uint;
  #[cfg (not (feature = "windowsnt"))]
  pub fn sleep (seconds: c_uint) -> c_uint;
  pub fn usleep (useconds: c_uint) -> c_int;
  pub fn sysconf (name: c_int) -> c_long;
  pub fn getpagesize () -> c_int;

  /* C 1831-1860's `# if @REPLACE_PIPE2@` arm is 1 in the frozen configuration: the generated
     header renames `pipe2` to `rpl_pipe2` and declares it, and that is the only declaration
     gnulib's own text contributes (see the file note).  lib/pipe2.rs defines it, with
     `#[unsafe(export_name = "rpl_pipe2")]`.
     The types are fully qualified: this mirror is `include!`d and cannot add a `use`, and the
     names below are not in every consumer's import list (PORTING.md 0.1-9/3.18).  */
  pub fn rpl_pipe2 (fd: *mut core::ffi::c_int, flags: core::ffi::c_int) -> core::ffi::c_int;
  /* C 606-613's macOS arm is the live one for `environ`: `#include <crt_externs.h>` plus
     `# define environ (*_NSGetEnviron ())` (the frozen substitution of `@HAVE_DECL_ENVIRON@`
     is 0 and `TARGET_OS_IPHONE` is 0).  `<crt_externs.h>` declares
     `char ***_NSGetEnviron (void);`, and the arm's `# else` branch - `extern char **environ;`
     - is the declaration this mirror keeps so that a unit reading the variable itself (the
     2011 C source's `alloc.c` does, and the port's alloc.rs includes this mirror) resolves.  */
  pub fn _NSGetEnviron () -> *mut *mut *mut core::ffi::c_char;
  pub static mut environ: *mut *mut core::ffi::c_char;
}

/* <unistd.h>'s `access` mode flags (measured: F_OK 0, X_OK 1, W_OK 2, R_OK 4).  */
pub const F_OK: c_int = 0;
pub const X_OK: c_int = 1;
pub const W_OK: c_int = 2;
pub const R_OK: c_int = 4;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

/* C 264/267/270: `# ifndef STDIN_FILENO / # define STDIN_FILENO 0` and its two siblings.  The
   frozen target's own <unistd.h> defines all three with these values (measured in the
   preprocessing run above), so the template's `#ifndef` arms are dead and these are the
   system's numbers.  */
pub const STDIN_FILENO: core::ffi::c_int = 0;
pub const STDOUT_FILENO: core::ffi::c_int = 1;
pub const STDERR_FILENO: core::ffi::c_int = 2;

/* C 602: `#  define _GL_USE_CRT_EXTERNS` - the live arm's empty marker for "the macOS
   `environ` comes from <crt_externs.h>" (the `#if !TARGET_OS_IPHONE` test above it is true).
   C's macro has an empty body; the port's marker is the value the `#ifdef` reads.  */
pub const _GL_USE_CRT_EXTERNS: bool = true;

/* <unistd.h>: the two `sysconf` names lib/nproc.c's live branches test.  Values measured
   from the frozen target (`docs/baseline/config/darwin-arm64.macros.tsv` has
   `_SC_NPROCESSORS_CONF 57` and `_SC_NPROCESSORS_ONLN 58`), and `_SC_PAGESIZE` (29) is the
   argument the dead `_gl_getpagesize` arm below names.  */
pub const _SC_NPROCESSORS_CONF: c_int = 57;
pub const _SC_NPROCESSORS_ONLN: c_int = 58;
pub const _SC_PAGESIZE: core::ffi::c_int = 29;

/* ---- the template's own half (PORTING.md P-26) -----------------------------------------
   Everything below is a `@PLACEHOLDER@` arm that the frozen substitution settles.  A name
   under `#[cfg (feature = "...")]` is the arm of the *other* configuration, kept as text
   because the extractor reads this template blind (P-08/P-51); the feature names the C
   condition that selects it.  */

/* C 38-39: `# define _GL_INCLUDING_UNISTD_H` around the `#include_next <unistd.h>`, the split
   double-inclusion guard of the special invocation convention (C 26-30).  The port has no
   include of its own to guard; the *unit* that needs the system `<unistd.h>` first sets the
   marker itself (`lib/faccessat.rs` 39 mirrors C 22 with a `pub const`), so the mirror cannot
   hold the name as a value item (that would be an E0428 with that unit).  A `macro_rules!`
   lives in the namespace C's `#define` also uses, which is what makes both stand.  */
macro_rules! _GL_INCLUDING_UNISTD_H { () => {} }

/* C 64-65: the same guard around the `#include <winsock2.h>` of the Windows arm
   (C 60-69, `@GNULIB_GETHOSTNAME@ && @UNISTD_H_HAVE_WINSOCK2_H@` - dead, the second
   substitution is 0).  */
pub const _GL_INCLUDING_WINSOCK2_H: bool = true;

/* C 122-124: `# define __need_system_stdlib_h` around `#include <stdlib.h>` (the
   `#if !defined __GLIBC__` arm C 116-125; the frozen compiler defines `__GLIBC__` as 0, so
   this one is live as a marker - the system <stdlib.h> is the port's own stdlib.rs mirror).  */
pub const __need_system_stdlib_h: bool = true;

/* C 1449: `#  ifndef CLSIZE / #   define CLSIZE 1` - inside `#  if !@HAVE_GETPAGESIZE@`
   (false here, HAVE_GETPAGESIZE is 1), i.e. inside C 1437-1470's dead fallback chain.  */
#[cfg (feature = "gnulib_getpagesize")]
pub const CLSIZE: core::ffi::c_int = 1;

/* C 1468: `#      define GNULIB_defined_getpagesize_function 1`, the marker of that chain.  */
#[cfg (feature = "gnulib_getpagesize")]
pub const GNULIB_defined_getpagesize_function: core::ffi::c_int = 1;

/* C 1627: `#  define GNULIB_defined_isatty 1`, the marker of the `@REPLACE_ISATTY@` arm.  */
#[cfg (feature = "replace_isatty")]
pub const GNULIB_defined_isatty: core::ffi::c_int = 1;

/* C 1454/1456-1490: `_gl_getpagesize ()` is the fallback the dead `getpagesize` macro below
   expands to; the template gives it eight bodies, one per platform arm (POSIX
   `sysconf (_SC_PAGESIZE)`, VMS 8192/512, BeOS `B_PAGE_SIZE`, AmigaOS4 2048,
   `<sys/param.h>`'s `EXEC_PAGESIZE`/`NBPG * CLSIZE`/`NBPC`).  Nothing expands it in this
   configuration (see the file note), so only the first arm - the one the frozen substitution
   of `@HAVE_OS_H@` 0 / `@HAVE_SYS_PARAM_H@` 1 leaves live under the dead guard - is spelled
   out; the presence of the name is what the template's blind reading asks for.  */
#[cfg (feature = "gnulib_getpagesize")]
macro_rules! _gl_getpagesize { () => { sysconf (_SC_PAGESIZE) } }

/* C 1460-1464: `#     define getpagesize() _gl_getpagesize ()` and, three lines later in the
   C++ arm, `_GL_GETPAGESIZE_INLINE int getpagesize (void) { return _gl_getpagesize (); }`.
   The frozen configuration declares neither (GNULIB_GETPAGESIZE is 0); the system
   `getpagesize` declared above is the live entry, and this macro is C's other spelling of
   the name.  */
#[cfg (feature = "gnulib_getpagesize")]
macro_rules! getpagesize { () => { _gl_getpagesize! () } }

/* C 623-626: the dead `GNULIB_POSIXCHECK` arm's accessor,
   `_GL_UNISTD_INLINE char *** rpl_environ (void) { return &environ; }`, which the arm's
   `# define environ (*rpl_environ ())` (C 628) expands to.  C defines it as a function, so
   the port keeps a function; `environ` is the static declared above, and the address of an
   `extern` static needs no read of it.  */
#[inline(always)]
pub unsafe fn rpl_environ () -> *mut *mut *mut core::ffi::c_char
{
  return &raw mut environ;
}

/* C 628: `#  define environ (*rpl_environ ())` is the POSIXCHECK spelling of the name; C 613
   is the live macOS one, `# define environ (*_NSGetEnviron ())`.  A macro and the static
   above live in different namespaces, so both can stand here as C has both arms.  */
/* C 1837-1839: `#   undef pipe2 / #   define pipe2 rpl_pipe2` - this one is *live*
   (`@REPLACE_PIPE2@` is 1 in the frozen substitution, and the preprocessing run at the head of
   the file shows the rename in the generated header), so it stands without a feature: the
   port's `pipe2` is the replacement lib/pipe2.rs defines (`#[unsafe(export_name =
   "rpl_pipe2")]`), and a unit reads it through this macro.  */
macro_rules! pipe2 { ($fd:expr, $flags:expr) => { rpl_pipe2 ($fd, $flags) } }

/* C 628: `#  define environ (*rpl_environ ())` is the POSIXCHECK arm's spelling of the name,
   while C 613 - the live arm - is `# define environ (*_NSGetEnviron ())`.  The two arms are
   the two `#define`s the extractor counts for `environ`; the live one is spelled out here
   (the port has no preprocessor rewriting the identifier, and a macro and the static above
   live in different namespaces, so both can stand).  */
macro_rules! environ { () => { (*_NSGetEnviron ()) } }

/* C 167-249: inside `# if @GNULIB_GETHOSTNAME@ && @UNISTD_H_HAVE_WINSOCK2_H@` (the second
   substitution is 0, so the whole block is dead here) and its `#  if !defined
   _@GUARD_PREFIX@_SYS_SOCKET_H` - the arm that makes the socket identifiers unusable until
   <sys/socket.h> is included.  Every name is C's rename, and the extractor compares the
   body, which is the single identifier C spells.  */
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! socket { () => { socket_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! connect { () => { connect_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! accept { () => { accept_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! bind { () => { bind_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! getpeername { () => { getpeername_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! getsockname { () => { getsockname_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! getsockopt { () => { getsockopt_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! listen { () => { listen_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! recv { () => { recv_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! send { () => { send_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! recvfrom { () => { recvfrom_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! sendto { () => { sendto_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! setsockopt { () => { setsockopt_used_without_including_sys_socket_h } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! shutdown { () => { shutdown_used_without_including_sys_socket_h } }

/* C 250-258: the same block's `select` arm, which names the other header
   (`#   define select select_used_without_including_sys_select_h`).  */
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! select { () => { select_used_without_including_sys_select_h } }

/* C 1305-1316: the two `gethostname` arms.  The `@REPLACE_GETHOSTNAME@` arm (C 1305-1311)
   renames it to `rpl_gethostname`; the `#elif @UNISTD_H_HAVE_WINSOCK2_H@` arm (C 1315-1316)
   renames it to `gethostname_used_without_requesting_gnulib_module_gethostname`.  Both are
   dead here (`@UNISTD_H_HAVE_WINSOCK2_H@` is 0, the frozen table has no
   `REPLACE_GETHOSTNAME`), and C defines the name twice, so the extractor asks for presence
   rather than the body.  */
#[cfg (feature = "replace_gethostname")]
macro_rules! gethostname { () => { rpl_gethostname } }
#[cfg (feature = "unistd_h_have_winsock2_h")]
macro_rules! gethostname { () => { gethostname_used_without_requesting_gnulib_module_gethostname } }

/* ---- the `@REPLACE_*@` renames of the other arms -----------------------------------------
   Each is the `#  if @REPLACE_X@ / #   undef X / #   define X rpl_X` text of its block, dead
   because no `REPLACE_*` of the frozen configuration is set for these.  The names C does set
   (`REPLACE_PIPE2`, seen above) are spelled out at the head of the file.  */
#[cfg (feature = "replace_copy_file_range")]
macro_rules! copy_file_range { () => { rpl_copy_file_range } }
#[cfg (feature = "replace_dup3")]
macro_rules! dup3 { () => { rpl_dup3 } }
#[cfg (feature = "replace_getusershell")]
macro_rules! getusershell { () => { rpl_getusershell } }
#[cfg (feature = "replace_getusershell")]
macro_rules! setusershell { () => { rpl_setusershell } }
#[cfg (feature = "replace_getusershell")]
macro_rules! endusershell { () => { rpl_endusershell } }
#[cfg (feature = "replace_faccessat")]
macro_rules! faccessat { () => { rpl_faccessat } }
#[cfg (feature = "replace_fchownat")]
macro_rules! fchownat { () => { rpl_fchownat } }
#[cfg (feature = "replace_fdatasync")]
macro_rules! fdatasync { () => { rpl_fdatasync } }
#[cfg (feature = "replace_getdomainname")]
macro_rules! getdomainname { () => { rpl_getdomainname } }
#[cfg (feature = "replace_getdtablesize")]
macro_rules! getdtablesize { () => { rpl_getdtablesize } }
#[cfg (feature = "replace_getentropy")]
macro_rules! getentropy { () => { rpl_getentropy } }
#[cfg (feature = "replace_getgroups")]
macro_rules! getgroups { () => { rpl_getgroups } }
#[cfg (feature = "replace_getlogin")]
macro_rules! getlogin { () => { rpl_getlogin } }
#[cfg (feature = "replace_getlogin_r")]
macro_rules! getlogin_r { () => { rpl_getlogin_r } }
#[cfg (feature = "replace_getpass")]
macro_rules! getpass { () => { rpl_getpass } }
#[cfg (feature = "replace_pread")]
macro_rules! pread { () => { rpl_pread } }
#[cfg (feature = "replace_pwrite")]
macro_rules! pwrite { () => { rpl_pwrite } }
#[cfg (feature = "replace_sethostname")]
macro_rules! sethostname { () => { rpl_sethostname } }
#[cfg (feature = "replace_sleep")]
macro_rules! sleep { () => { rpl_sleep } }
#[cfg (feature = "replace_symlinkat")]
macro_rules! symlinkat { () => { rpl_symlinkat } }
#[cfg (feature = "replace_ttyname_r")]
macro_rules! ttyname_r { () => { rpl_ttyname_r } }
#[cfg (feature = "replace_usleep")]
macro_rules! usleep { () => { rpl_usleep } }

/* ---- the native-Windows renames ---------------------------------------------------------
   The `#  if !(defined __cplusplus && defined GNULIB_NAMESPACE) / #   undef X / #   define X
   _X` text of the `# if defined _WIN32 && !defined __CYGWIN__` arms (and of the
   `@GNULIB_MDA_*@` arms that repeat it), dead on this target.  `_execl`'s family and `_swab`
   are the ones the extractor asks for; the `_chdir`/`_close`/`_dup`/`_access` family is
   reached through the declarations above (their names are already declared as functions).  */
#[cfg (feature = "windows")]
macro_rules! execl { ($($arg:tt)*) => { _execl ($($arg)*) } }
#[cfg (feature = "windows")]
macro_rules! execle { ($($arg:tt)*) => { _execle ($($arg)*) } }
#[cfg (feature = "windows")]
macro_rules! execlp { ($($arg:tt)*) => { _execlp ($($arg)*) } }
#[cfg (feature = "windows")]
macro_rules! execv { ($($arg:tt)*) => { _execv ($($arg)*) } }
#[cfg (feature = "windows")]
macro_rules! execve { ($($arg:tt)*) => { _execve ($($arg)*) } }
#[cfg (feature = "windows")]
macro_rules! execvp { ($($arg:tt)*) => { _execvp ($($arg)*) } }
#[cfg (feature = "windows")]
macro_rules! execvpe { ($($arg:tt)*) => { _execvpe ($($arg)*) } }
#[cfg (feature = "windows")]
macro_rules! swab { ($($arg:tt)*) => { _swab ($($arg)*) } }
