// femacs: port of lib/fcntl.in.h @ fd7afced403 (gnulib).
//
// gnulib template: "Like <fcntl.h>, but with non-working flags defined to 0."  The frozen
// target's generated header is build/emacs-ref/lib/fcntl.h, whose placeholders substitute to
// `_GL_FCNTL_H` (the guard) and `#include_next <fcntl.h>`; its GNULIB_*/REPLACE_*/HAVE_* values
// are `@GNULIB_CREAT@` 0, `@GNULIB_FCNTL@` 1 / `@REPLACE_FCNTL@` 0 / `@HAVE_FCNTL@` 1,
// `@GNULIB_OPEN@` 1 / `@REPLACE_OPEN@` 0, `@GNULIB_OPENAT@` 0, `@GNULIB_OPENAT2@` 0 and
// `@GNULIB_NONBLOCKING@` 0.  The port has neither `#include_next` nor placeholders, so the
// mirror carries both halves:
//
//   * the system half: every name the frozen `<fcntl.h>` (gnulib's, i.e. the system's plus
//     this header's fix-ups) ends up with.  The frozen values are the ones the reference
//     build's preprocessor holds, which `docs/baseline/config/darwin-arm64.macros.tsv`
//     records (`clang -dM -E -std=gnu23 -I build/emacs-ref/src -I build/emacs-ref/lib
//     -I emacs/src -I emacs/lib`, the probe `tools/gen-cond-macros.py` writes); the mirror
//     quotes the rows it uses.
//   * the template's own half: the `#ifndef X / # define X <n>` fallbacks for the flags the
//     system header does not have (live here: `O_DIRECT`, `O_IGNORE_CTTY`, `O_NOATIME`,
//     `O_NOLINK`, `O_NOLINKS`, `O_NOTRANS`, `O_RSYNC`, `O_TTY_INIT`, `AT_NO_AUTOMOUNT`,
//     `O_BINARY`, `O_TEXT`), the `GNULIB_defined_*` witness macros, and the openat2 module's
//     `struct open_how`/`RESOLVE_*` block (dead: `GNULIB_OPENAT2` is 0).
//
// **The fallbacks of the names the system header *does* define are dead text here.**  P-51 /
// P-66② keep that text in the mirror under a feature that names its own condition
// (`#ifndef O_NOFOLLOW` -> `feature = "no_o_nofollow"`), the way
// lib/include/acl-internal.rs spells MODE_INSIDE_ACL's three arms as one name with one
// `#[cfg]` each.  The arm C's template spells is written **first**, because
// `tools/extract-abi.py`'s constant reader takes the first definition of a name and therefore
// compares C's arm; the frozen arm follows, ungated.  Only rustc's live arm exists in a build
// (`xtask build-tu`), so the port's values are the frozen ones.
//
// C 25-50's special-invocation arm (`__need_system_fcntl_h`), C 41/69's `<sys/stat.h>`
// include and C 81's `#undef _GL_ALREADY_INCLUDING_FCNTL_H` have no Rust counterpart (no
// preprocessor); C 47-50/76-79's `<io.h>` arm is `_WIN32`, C 91-93's `<unistd.h>` arm is
// glibc-only, C 318-329's `_AIX` arm is AIX-only and C 435-445's `_O_BINARY`/BeOS arms are
// Windows/BeOS-only - all dead on this target (PORTING.md P-66②).

include!("sys_types.in.rs");   // C 34/62: <sys/types.h> (mode_t, off_t)
include!("errno.rs");          // C 499's `_GL_OPENAT_ESYMLINK ELOOP` (<errno.h>)

/* C 55/83-84: the double-inclusion guard, `_@GUARD_PREFIX@_FCNTL_H`, whose frozen
   substitution is `_GL_FCNTL_H` (build/emacs-ref/lib/fcntl.h 55/83).  The template's own
   spelling reads as `_` to the extractor, which docs/ports/excluded-macros.txt records.
   C 57/81's `_GL_ALREADY_INCLUDING_FCNTL_H` marks the split around the include-next; the
   port has no include-next, so it has no live use - carried as C's empty value.  */
pub const _GL_FCNTL_H: c_int = 1;
pub const _GL_ALREADY_INCLUDING_FCNTL_H: () = ();

/// gnulib's <fcntl.h> defines `PROMOTED_MODE_T` as the type `open`'s mode argument is promoted
/// to at the call (`int` on this target - docs/baseline/config/darwin-arm64.macros.tsv 3204);
/// `lib/open.c` reads its variadic mode argument with that type.
pub type PROMOTED_MODE_T = c_int;

/* C 288-292: `#ifndef FD_CLOEXEC / # define FD_CLOEXEC 1` - live in form (the system defines
   it) and equal in value (1).  */
pub const FD_CLOEXEC: c_int = 1;

/* C 294-311's `F_*` fix-ups.  The template's fallbacks are the dead arm of a target without
   the command; the frozen values follow (`darwin-arm64.macros.tsv` 1095/1103/1096).  */
/* C 305-307's `#ifndef F_DUPFD / # define F_DUPFD 1`.  */
#[cfg (feature = "no_f_dupfd")]
pub const F_DUPFD: c_int = 1;
/// `F_DUPFD 0` (frozen).
pub const F_DUPFD: c_int = 0;
/* C 309-311's `#ifndef F_GETFD / # define F_GETFD 2`.  */
#[cfg (feature = "no_f_getfd")]
pub const F_GETFD: c_int = 2;
/// `F_GETFD 1` (frozen).
pub const F_GETFD: c_int = 1;
/* C 297-303's `#ifndef F_DUPFD_CLOEXEC / # define F_DUPFD_CLOEXEC 0x40000000`; the `#else`
   arm's `GNULIB_defined_F_DUPFD_CLOEXEC 0` is the frozen one (the system defines the
   command), so the witness is 0 here and 1 only in the dead arm.  */
#[cfg (feature = "no_f_dupfd_cloexec")]
pub const F_DUPFD_CLOEXEC: c_int = 0x40000000;
/// `F_DUPFD_CLOEXEC 67` (frozen).
pub const F_DUPFD_CLOEXEC: c_int = 67;
#[cfg (feature = "no_f_dupfd_cloexec")]
pub const GNULIB_defined_F_DUPFD_CLOEXEC: c_int = 1;
/// C 301-302's `#else` arm: `GNULIB_defined_F_DUPFD_CLOEXEC 0` (frozen).
pub const GNULIB_defined_F_DUPFD_CLOEXEC: c_int = 0;
/* `fcntl`'s two remaining standard commands, not part of the template's fix-ups but the
   system's (lib/fcntl.c's `#ifdef F_*` stack names them; measured in the probe below).  */
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;

/* ---- C 313-450's `O_*` fix-ups -------------------------------------------------------- */

/* The access modes the frozen header ends up with (measured: O_RDONLY=0 O_WRONLY=1 O_RDWR=2
   ... O_CREAT=512 O_EXCL=2048 O_TRUNC=1024 O_APPEND=8):

       $ clang -I build/emacs-ref/lib -I build/emacs-ref/src -I emacs/src -I emacs/lib ...
       O_RDONLY=0 O_WRONLY=1 O_RDWR=2 O_ACCMODE=3 O_CREAT=512 O_EXCL=2048 O_TRUNC=1024
       O_APPEND=8 O_NONBLOCK=4 O_SYNC=128 O_CLOEXEC=16777216 O_DIRECTORY=1048576 O_NOFOLLOW=256
       F_GETFL=3 F_SETFL=4 F_DUPFD=0 F_DUPFD_CLOEXEC=67 F_GETFD=1 F_SETFD=2 FD_CLOEXEC=1
       AT_FDCWD=-2 AT_SYMLINK_NOFOLLOW=32 AT_REMOVEDIR=128 AT_SYMLINK_FOLLOW=64 AT_EACCESS=16

   (`darwin-arm64.macros.tsv` carries the same values as the macros' replacement text.)  */
pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;
/* C 428-431's `#if ~O_ACCMODE & (O_RDONLY | O_WRONLY | O_RDWR | O_EXEC | O_SEARCH)` is *true*
   on this target (the system's 3 lacks the O_EXEC/O_SEARCH bits), so gnulib redefines
   `O_ACCMODE` to that five-flag mask - which `darwin-arm64.macros.tsv` 3077 records and is
   both the frozen value and the arm the extractor compares.  */
pub const O_ACCMODE: c_int = O_RDONLY | O_WRONLY | O_RDWR | O_EXEC | O_SEARCH;
pub const O_CREAT: c_int = 0x200;
pub const O_TRUNC: c_int = 0x400;
pub const O_EXCL: c_int = 0x800;
pub const O_APPEND: c_int = 0x8;
/* `O_NONBLOCK` has two template arms (`#ifndef O_NONBLOCK / # define O_NONBLOCK O_NDELAY` and
   the nonblocking module's `0x40000000`), so the extractor does not compare its body
   (P-24); the frozen value is the system's 0x4 (`darwin-arm64.macros.tsv` 3104).  */
pub const O_NONBLOCK: c_int = 0x4;
/* C 336-341's `#ifndef O_CLOEXEC / # define O_CLOEXEC 0x40000000`; two arms as well, and the
   frozen value is the system's 0x1000000 (`...macros.tsv` 3082).  Its witness
   `GNULIB_defined_O_CLOEXEC` is 0 in the frozen header (the `#else` arm).

   The value is the *delivered target's* header, so the port carries one arm per target (the
   `Emacs_GC`/`O_DIRECTORY` shape): 0x1000000 is Darwin's, while bionic defines it as
   `02000000` octal = 0x80000 - measured with the target's own preprocessor,

       $ clang --target=aarch64-linux-android24 -E -dM -include fcntl.h - </dev/null
       #define O_CLOEXEC 02000000

   Status 4.780: passing Darwin's 0x1000000 to `pipe2` on Android makes it fail with `EINVAL`,
   and `setEmacsParams` then takes C's `if (pipe2 (...) < 0) emacs_abort ()` arm - which is the
   tombstone that wheel read as a native crash.  */
#[cfg (not (feature = "have_android"))]
pub const O_CLOEXEC: c_int = 0x1000000;
#[cfg (feature = "have_android")]
pub const O_CLOEXEC: c_int = 0o2000000;
pub const GNULIB_defined_O_CLOEXEC: c_int = 0;
/* C 347-349's `#ifndef O_DIRECTORY / # define O_DIRECTORY 0x20000000` - dead: the system
   defines it as 0x100000 (`...macros.tsv` 3086).  */
#[cfg (feature = "no_o_directory")]
pub const O_DIRECTORY: c_int = 0x20000000;
/// `O_DIRECTORY 0x00100000` (frozen).
pub const O_DIRECTORY: c_int = 0x100000;
/* C 392-394's `#ifndef O_NOFOLLOW / # define O_NOFOLLOW 0` - dead: the system defines it as
   0x100 (`...macros.tsv` 3100).  */
#[cfg (feature = "no_o_nofollow")]
pub const O_NOFOLLOW: c_int = 0;
/// `O_NOFOLLOW 0x00000100` (frozen).
pub const O_NOFOLLOW: c_int = 0x100;
/* C 420-422's `#ifndef O_SYNC / # define O_SYNC 0` - dead: the system's is 0x80 (3114).  */
#[cfg (feature = "no_o_sync")]
pub const O_SYNC: c_int = 0;
/// `O_SYNC 0x0080` (frozen).
pub const O_SYNC: c_int = 0x80;
/* C 343-345's `#ifndef O_DIRECT / # define O_DIRECT 0` - live (this target has no O_DIRECT;
   lchmod.c's `#ifdef O_PATH` arm is dead for the same reason).  */
pub const O_DIRECT: c_int = 0;
/* C 351-353's `#ifndef O_DSYNC / # define O_DSYNC 0` - dead: the system's is 0x400000
   (`...macros.tsv` 3090).  */
#[cfg (feature = "no_o_dsync")]
pub const O_DSYNC: c_int = 0;
/// `O_DSYNC 0x400000` (frozen).
pub const O_DSYNC: c_int = 0x400000;
/* C 355-357's `#ifndef O_EXEC / # define O_EXEC O_RDONLY` - dead: the system's is
   0x40000000 (`...macros.tsv` 3093).  */
#[cfg (feature = "no_o_exec")]
pub const O_EXEC: c_int = O_RDONLY;
/// `O_EXEC 0x40000000` (frozen).
pub const O_EXEC: c_int = 0x40000000;
/* C 416-418's `#ifndef O_SEARCH / # define O_SEARCH O_RDONLY` - dead: the system's is
   `(O_EXEC | O_DIRECTORY)` (`...macros.tsv` 3111, `O_EXEC 0x40000000` +
   `O_DIRECTORY 0x00100000`), which is 0x40100000 as an int.  */
#[cfg (feature = "no_o_search")]
pub const O_SEARCH: c_int = O_RDONLY;
/// `O_SEARCH (O_EXEC | O_DIRECTORY)` (frozen).
pub const O_SEARCH: c_int = O_EXEC | O_DIRECTORY;
/* C 359-361's `#ifndef O_IGNORE_CTTY / # define O_IGNORE_CTTY 0` - live.  */
pub const O_IGNORE_CTTY: c_int = 0;
/* C 363-365's `#ifndef O_NDELAY / # define O_NDELAY 0` - dead: the system's `O_NDELAY` is
   `O_NONBLOCK` (`...macros.tsv` 3097), which is 0x4 here.  */
#[cfg (feature = "no_o_ndelay")]
pub const O_NDELAY: c_int = 0;
/// `O_NDELAY O_NONBLOCK` (frozen).
pub const O_NDELAY: c_int = O_NONBLOCK;
/* C 367-369's `#ifndef O_NOATIME / # define O_NOATIME 0` - live.  */
pub const O_NOATIME: c_int = 0;
/* C 388-390's `#ifndef O_NOCTTY / # define O_NOCTTY 0` - dead: the system's is 0x20000
   (`...macros.tsv` 3099).  */
#[cfg (feature = "no_o_noctty")]
pub const O_NOCTTY: c_int = 0;
/// `O_NOCTTY 0x00020000` (frozen).
pub const O_NOCTTY: c_int = 0x20000;
/* C 396-406's `O_NOLINK`/`O_NOLINKS`/`O_NOTRANS` and C 408-410's `O_RSYNC`, C 424-426's
   `O_TTY_INIT` - all live fallbacks (the system has none of them).  */
pub const O_NOLINK: c_int = 0;
pub const O_NOLINKS: c_int = 0;
pub const O_NOTRANS: c_int = 0;
pub const O_RSYNC: c_int = 0;
pub const O_TTY_INIT: c_int = 0;
/* C 433-450's `O_BINARY`/`O_TEXT`: two template arms each, and this target's system header has
   neither, so the `#ifndef` fallback is the frozen value (0) - it is what lib/pipe2.c's flag
   check compares against.  */
pub const O_BINARY: c_int = 0;
pub const O_TEXT: c_int = 0;
/* C 375-386's `#if @GNULIB_NONBLOCKING@` block is dead (the frozen substitution is 0), and
   inside it `# if O_NONBLOCK` is *true* here (0x4 is nonzero), so a target that had the
   module would take the arm whose witness is 0.  Kept under the feature naming the non-frozen
   side (P-51).  */
#[cfg (feature = "gnulib_nonblocking")]
pub const GNULIB_defined_O_NONBLOCK: c_int = 0;

/* ---- C 452-491's `AT_*` fix-ups ------------------------------------------------------- */

/* All five fallbacks are dead (the system defines every one of them), at the values
   `darwin-arm64.macros.tsv` 208/219/216/218/207 records.  */
/* C 465-467's `#ifndef AT_FDCWD / # define AT_FDCWD (-3041965)`.  */
#[cfg (feature = "no_at_fdcwd")]
pub const AT_FDCWD: c_int = -3041965;
/// `AT_FDCWD -2` (frozen).
pub const AT_FDCWD: c_int = -2;
/* C 471-473's `#ifndef AT_SYMLINK_NOFOLLOW / # define AT_SYMLINK_NOFOLLOW 4096`.  */
#[cfg (feature = "no_at_symlink_nofollow")]
pub const AT_SYMLINK_NOFOLLOW: c_int = 4096;
/// `AT_SYMLINK_NOFOLLOW 0x0020` (frozen).
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x20;
/* C 475-477's `#ifndef AT_REMOVEDIR / # define AT_REMOVEDIR 1`.  */
#[cfg (feature = "no_at_removedir")]
pub const AT_REMOVEDIR: c_int = 1;
/// `AT_REMOVEDIR 0x0080` (frozen).
pub const AT_REMOVEDIR: c_int = 0x80;
/* C 480-482's `#ifndef AT_SYMLINK_FOLLOW / # define AT_SYMLINK_FOLLOW 2`.  */
#[cfg (feature = "no_at_symlink_follow")]
pub const AT_SYMLINK_FOLLOW: c_int = 2;
/// `AT_SYMLINK_FOLLOW 0x0040` (frozen).
pub const AT_SYMLINK_FOLLOW: c_int = 0x40;
/* C 484-486's `#ifndef AT_EACCESS / # define AT_EACCESS 4`.  */
#[cfg (feature = "no_at_eaccess")]
pub const AT_EACCESS: c_int = 4;
/// `AT_EACCESS 0x0010` (frozen).
pub const AT_EACCESS: c_int = 0x10;
/* C 488-491's `#ifndef AT_NO_AUTOMOUNT / # define AT_NO_AUTOMOUNT 0` - live (the Linux-only
   name); `lib/fstatat.c`'s `AT_EMPTY_PATH` has the same shape at its own use site.  */
#[cfg (not (feature = "have_android"))]
pub const AT_NO_AUTOMOUNT: c_int = 0;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const AT_NO_AUTOMOUNT: c_int = 2048;

/* C 493-500's `#if defined __FreeBSD__ ... # define _GL_OPENAT_ESYMLINK EMLINK / #elif
   __NetBSD__ EFTYPE / #else ELOOP`: the frozen target takes the `#else`, and
   `darwin-arm64.macros.tsv` 4243 records `_GL_OPENAT_ESYMLINK ELOOP`.  */
pub const _GL_OPENAT_ESYMLINK: c_int = ELOOP;

/* ---- C 248-286's openat2 module (dead: `GNULIB_OPENAT2` is 0) ------------------------- */

/// C 255-262's `struct open_how`.  The whole block is dead text here (`GNULIB_OPENAT2` is 0),
/// and the field types are whichever arm this target's compiler takes (`__UINT64_TYPE__`, i.e.
/// `unsigned long long int`); kept under the feature naming the non-frozen side (P-51), where
/// C's three `unsigned long long` fields are Rust's `u64`.
#[cfg (feature = "gnulib_openat2")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct open_how
{
  pub flags: u64,
  pub mode: u64,
  pub resolve: u64,
}

/* C 263-268's six `RESOLVE_*` values, in C's order (they are the Linux `openat2` flags).  */
#[cfg (feature = "gnulib_openat2")]
pub const RESOLVE_NO_XDEV: c_int = 0x01;
#[cfg (feature = "gnulib_openat2")]
pub const RESOLVE_NO_MAGICLINKS: c_int = 0x02;
#[cfg (feature = "gnulib_openat2")]
pub const RESOLVE_NO_SYMLINKS: c_int = 0x04;
#[cfg (feature = "gnulib_openat2")]
pub const RESOLVE_BENEATH: c_int = 0x08;
#[cfg (feature = "gnulib_openat2")]
pub const RESOLVE_IN_ROOT: c_int = 0x10;
#[cfg (feature = "gnulib_openat2")]
pub const RESOLVE_CACHED: c_int = 0x20;

/* ---- C 145-171's `fcntl` witnesses --------------------------------------------------- */

/* `#if @GNULIB_FCNTL@` is live (1) but `@REPLACE_FCNTL@` is 0 and `@HAVE_FCNTL@` is 1, so
   both witness macros are dead text and `fcntl` is the system entry (the declaration below).
   Kept under the features naming each arm's non-frozen side (P-51).  */
#[cfg (feature = "replace_fcntl")]
pub const GNULIB_defined_rpl_fcntl: c_int = 1;
#[cfg (feature = "no_have_fcntl")]
pub const GNULIB_defined_fcntl: c_int = 1;

/* ---- the system entries C 103-286 declares -------------------------------------------- */

/* C 105-143's `creat` and C 173-218's `open` resolve to `_GL_CXXALIAS_SYS` here
   (`@REPLACE_CREAT@`/`@REPLACE_OPEN@` are 0 and `@GNULIB_CREAT@`/`@GNULIB_OPEN@`'s live arms
   are the non-Windows ones), and C 220-246's `openat` to `_GL_CXXALIAS_SYS` as well
   (`@REPLACE_OPENAT@` 0, `@HAVE_OPENAT@` 1).  C 163's `fcntl` is the system entry too.
   The four live in a whole-import module: `lib/cloexec.rs` and `lib/open.rs`
   define wrappers of their own, and a definition shadows a glob import instead of colliding
   with it (PORTING.md 0.1-10).  */
pub mod fcntl_decls {
  use super::*;

  unsafe extern "C" {
    pub fn fcntl (fd: c_int, action: c_int, ...) -> c_int;
    pub fn open (filename: *const c_char, flags: c_int, ...) -> c_int;
    pub fn openat (fd: c_int, filename: *const c_char, flags: c_int, ...) -> c_int;
    pub fn creat (filename: *const c_char, mode: mode_t) -> c_int;
  }
}
use fcntl_decls::*;

/* lib/fcntl.c's `fcntl` stack of `#ifdef F_*` cases (C 262-415) keeps whichever of these the
   frozen target's <fcntl.h> defines; measured, not guessed:

       $ cat > /tmp/fprobe.c <<'EOF'
       #include <fcntl.h>
       #include <stdio.h>
       int main (void) {
         printf ("F_BARRIERFSYNC=%d F_CHKCLEAN=%d F_FLUSH_DATA=%d F_FREEZE_FS=%d F_FULLFSYNC=%d\n",
                 (int) F_BARRIERFSYNC, (int) F_CHKCLEAN, (int) F_FLUSH_DATA, (int) F_FREEZE_FS, (int) F_FULLFSYNC);
         printf ("F_GETFD=%d F_GETFL=%d F_GETLEASE=%d F_GETNOSIGPIPE=%d F_GETOWN=%d F_GETPROTECTIONCLASS=%d F_GETPROTECTIONLEVEL=%d F_THAW_FS=%d\n",
                 (int) F_GETFD, (int) F_GETFL, (int) F_GETLEASE, (int) F_GETNOSIGPIPE, (int) F_GETOWN,
                 (int) F_GETPROTECTIONCLASS, (int) F_GETPROTECTIONLEVEL, (int) F_THAW_FS);
         printf ("F_DUPFD=%d F_DUPFD_CLOEXEC=%d F_DUPFD_CLOFORK=%d F_GLOBAL_NOCACHE=%d F_NOCACHE=%d F_NODIRECT=%d F_RDAHEAD=%d F_SETBACKINGSTORE=%d\n",
                 (int) F_DUPFD, (int) F_DUPFD_CLOEXEC, (int) F_DUPFD_CLOFORK, (int) F_GLOBAL_NOCACHE,
                 (int) F_NOCACHE, (int) F_NODIRECT, (int) F_RDAHEAD, (int) F_SETBACKINGSTORE);
         printf ("F_SETFD=%d F_SETFL=%d F_SETLEASE=%d F_SETNOSIGPIPE=%d F_SETOWN=%d F_SETPROTECTIONCLASS=%d F_SINGLE_WRITER=%d\n",
                 (int) F_SETFD, (int) F_SETFL, (int) F_SETLEASE, (int) F_SETNOSIGPIPE, (int) F_SETOWN,
                 (int) F_SETPROTECTIONCLASS, (int) F_SINGLE_WRITER);
         printf ("FD_CLOEXEC=%d\n", (int) FD_CLOEXEC);
         return 0; }
       EOF
       $ clang -o /tmp/fprobe /tmp/fprobe.c && /tmp/fprobe
       F_BARRIERFSYNC=85 F_CHKCLEAN=41 F_FLUSH_DATA=40 F_FREEZE_FS=53 F_FULLFSYNC=51
       F_GETFD=1 F_GETFL=3 F_GETLEASE=107 F_GETNOSIGPIPE=74 F_GETOWN=5 F_GETPROTECTIONCLASS=63 F_GETPROTECTIONLEVEL=77 F_THAW_FS=54
       F_DUPFD=0 F_DUPFD_CLOEXEC=67 F_DUPFD_CLOFORK=115 F_GLOBAL_NOCACHE=55 F_NOCACHE=48 F_NODIRECT=62 F_RDAHEAD=45 F_SETBACKINGSTORE=70
       F_SETFD=2 F_SETFL=4 F_SETLEASE=106 F_SETNOSIGPIPE=73 F_SETOWN=6 F_SETPROTECTIONCLASS=64 F_SINGLE_WRITER=76
       FD_CLOEXEC=1

   The names lib/fcntl.c also names but this target does *not* define (so their `case` labels
   are dead text there): F_ADD_SEALS, F_BADFD, F_CHECK_OPENEVT, F_CLOSEM, F_DUP2FD,
   F_DUP2FD_CLOEXEC, F_DUP2FD_CLOFORK, F_GETCONFINED, F_GETDEFAULTPROTLEVEL, F_GETPIPE_SZ,
   F_GETXFL, F_GET_SEALS, F_GETSIG, F_MAKECOMPRESSED, F_MAXFD, F_MOVEDATAEXTENTS, F_NOTIFY,
   F_RECYCLE, F_SETCONFINED, F_SETFIFOENH, F_SETPIPE_SZ, F_SETSIG (all checked in the same
   probe: the compiler rejects each name, and the frozen table lists none of them).  */

/* The macOS-specific commands (the F_* group above), measured.  */
pub const F_BARRIERFSYNC: c_int = 85;
pub const F_CHKCLEAN: c_int = 41;
pub const F_FLUSH_DATA: c_int = 40;
pub const F_FREEZE_FS: c_int = 53;
pub const F_FULLFSYNC: c_int = 51;
pub const F_GETLEASE: c_int = 107;
pub const F_GETNOSIGPIPE: c_int = 74;
#[cfg (not (feature = "have_android"))]
pub const F_GETOWN: c_int = 5;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const F_GETOWN: c_int = 9;
pub const F_GETPROTECTIONCLASS: c_int = 63;
pub const F_GETPROTECTIONLEVEL: c_int = 77;
pub const F_THAW_FS: c_int = 54;
pub const F_DUPFD_CLOFORK: c_int = 115;
pub const F_GLOBAL_NOCACHE: c_int = 55;
pub const F_NOCACHE: c_int = 48;
pub const F_NODIRECT: c_int = 62;
pub const F_RDAHEAD: c_int = 45;
pub const F_SETBACKINGSTORE: c_int = 70;
pub const F_SETLEASE: c_int = 106;
pub const F_SETNOSIGPIPE: c_int = 73;
#[cfg (not (feature = "have_android"))]
pub const F_SETOWN: c_int = 6;
/* Measured on the delivery target (bionic), status 4.781: the frozen arm above is
   the Darwin value and the two platforms number this one differently. */
#[cfg (feature = "have_android")]
pub const F_SETOWN: c_int = 8;
pub const F_SETPROTECTIONCLASS: c_int = 64;
pub const F_SINGLE_WRITER: c_int = 76;
