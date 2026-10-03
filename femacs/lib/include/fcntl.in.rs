// femacs: WIP mirror of lib/fcntl.in.h (gnulib's <fcntl.h>; PORTING.md P-26).
//
// As with <sys/stat.h>, C's <fcntl.h> is the system header plus gnulib's aliases and warnings;
// the frozen configuration leaves every `REPLACE_*` macro zero, so the aliases resolve to the
// system entries, which is what this mirror declares.
//
//     $ cat > /tmp/oprobe.c <<'EOF'
//     #include <fcntl.h>
//     #include <stdio.h>
//     int main(void){
//       printf("O_RDONLY=%d O_WRONLY=%d O_RDWR=%d O_ACCMODE=%d O_CREAT=%d O_EXCL=%d O_TRUNC=%d "
//              "O_APPEND=%d O_NONBLOCK=%d O_SYNC=%d O_CLOEXEC=%d O_DIRECTORY=%d O_NOFOLLOW=%d\n",
//              O_RDONLY,O_WRONLY,O_RDWR,O_ACCMODE,O_CREAT,O_EXCL,O_TRUNC,O_APPEND,O_NONBLOCK,
//              O_SYNC,O_CLOEXEC,O_DIRECTORY,O_NOFOLLOW);
//       printf("F_GETFL=%d F_SETFL=%d F_DUPFD=%d F_DUPFD_CLOEXEC=%d F_GETFD=%d F_SETFD=%d FD_CLOEXEC=%d\n",
//              F_GETFL,F_SETFL,F_DUPFD,F_DUPFD_CLOEXEC,F_GETFD,F_SETFD,FD_CLOEXEC);
//       printf("AT_FDCWD=%d AT_SYMLINK_NOFOLLOW=%d AT_REMOVEDIR=%d AT_SYMLINK_FOLLOW=%d AT_EACCESS=%d\n",
//              (int) AT_FDCWD, (int) AT_SYMLINK_NOFOLLOW, (int) AT_REMOVEDIR,
//              (int) AT_SYMLINK_FOLLOW, (int) AT_EACCESS);
//       return 0; }
//     EOF
//     $ clang -o /tmp/oprobe /tmp/oprobe.c && /tmp/oprobe
//     O_RDONLY=0 O_WRONLY=1 O_RDWR=2 O_ACCMODE=3 O_CREAT=512 O_EXCL=2048 O_TRUNC=1024 O_APPEND=8 O_NONBLOCK=4 O_SYNC=128 O_CLOEXEC=16777216 O_DIRECTORY=1048576 O_NOFOLLOW=256
//     F_GETFL=3 F_SETFL=4 F_DUPFD=0 F_DUPFD_CLOEXEC=67 F_GETFD=1 F_SETFD=2 FD_CLOEXEC=1
//     AT_FDCWD=-2 AT_SYMLINK_NOFOLLOW=32 AT_REMOVEDIR=128 AT_SYMLINK_FOLLOW=64 AT_EACCESS=16
//
// `AT_EMPTY_PATH` is *not* defined by this target (the Linux-only name), which is why
// `lib/fstatat.c` carries its own `#ifndef AT_EMPTY_PATH # define AT_EMPTY_PATH 0`.  The
// unchecked boxes of docs/ports/include-fcntl.in.md are the rest of gnulib's header.

include!("sys_types.in.rs");   // <sys/types.h> (mode_t)

/// gnulib's <fcntl.h> defines `PROMOTED_MODE_T` as the type `open`'s mode argument is promoted
/// to at the call (`int` on this target - docs/baseline/config/darwin-arm64.macros.tsv 1502);
/// `lib/open.c` reads its variadic mode argument with that type.
pub type PROMOTED_MODE_T = c_int;

/* The access modes and open-time flags (measured above).  */
pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;
pub const O_ACCMODE: c_int = 3;
pub const O_NONBLOCK: c_int = 0x4;
pub const O_APPEND: c_int = 0x8;
pub const O_SYNC: c_int = 0x80;
pub const O_NOFOLLOW: c_int = 0x100;
pub const O_CREAT: c_int = 0x200;
pub const O_TRUNC: c_int = 0x400;
pub const O_EXCL: c_int = 0x800;
pub const O_DIRECTORY: c_int = 0x100000;
pub const O_CLOEXEC: c_int = 0x1000000;
/* gnulib's `#ifndef O_DIRECTORY # define O_DIRECTORY 0x20000000` fallback is dead on this
   target (the system defines it), so 0x100000 above is the system value.  */
/* O_SEARCH and O_EXEC are the two POSIX 2008 search/execute flags; this target defines both
   (measured: 1074790400 = 0x40100000 and 1073741824 = 0x40000000), so gnulib's
   `#ifndef O_SEARCH # define O_SEARCH O_RDONLY` fallback and its `O_EXEC` twin are dead text.
   O_PATH is *not* defined here, which is what makes lib/lchmod.c's `#ifdef O_PATH` arm dead.  */
pub const O_SEARCH: c_int = 0x40100000;
pub const O_EXEC: c_int = 0x40000000;
pub const O_NOCTTY: c_int = 0x20000;
/* gnulib's <fcntl.h> defines the text/binary distinction away on Unix: this target's system
   header has neither, so gnulib's `#ifndef O_BINARY # define O_BINARY 0` and its `O_TEXT`
   twin are the live values (they are what lib/pipe2.c's flag check compares against).  */
pub const O_BINARY: c_int = 0;
pub const O_TEXT: c_int = 0;

/* `fcntl`'s commands and the descriptor flags.  */
pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const F_DUPFD_CLOEXEC: c_int = 67;
pub const FD_CLOEXEC: c_int = 1;

/* C 454-490's `AT_*` values: the system header defines all of them on this target, so
   gnulib's fallbacks (`-3041965`, `4096`, `1`, `4`) are dead text.  */
pub const AT_FDCWD: c_int = -2;
pub const AT_SYMLINK_NOFOLLOW: c_int = 32;
pub const AT_SYMLINK_FOLLOW: c_int = 64;
pub const AT_REMOVEDIR: c_int = 128;
pub const AT_EACCESS: c_int = 16;

/* C 163-237's `_GL_CXXALIAS_SYS (fcntl, ...)`, `(open, ...)` and `(openat, ...)`: the system
   entries, since no `REPLACE_*` macro is nonzero in the frozen configuration.  */
/* The four entries in a whole-import module: `femacs/lib/cloexec.rs` and
   `femacs/lib/open.rs` define wrappers of their own, and a definition shadows a glob import
   instead of colliding with it (PORTING.md 0.1-10).  */
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
pub const F_GETOWN: c_int = 5;
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
pub const F_SETOWN: c_int = 6;
pub const F_SETPROTECTIONCLASS: c_int = 64;
pub const F_SINGLE_WRITER: c_int = 76;
