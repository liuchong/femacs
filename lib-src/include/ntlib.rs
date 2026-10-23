// femacs: mirror of lib-src/ntlib.h @ fd7afced403.
//
// The Windows support programs' shadow header: the Unix routine declarations the MSVC/CRT
// environment does not provide (`sleep`, `getwd`, `getpass`, ...) and the table of renames that
// point the POSIX names at the CRT's underscore spellings.  It is included only under
// `#ifdef WINDOWSNT` (movemail.c 86, pop.c 30, update-game-score.c 52 and ntlib.c), and the
// frozen darwin-arm64 configuration defines no `WINDOWSNT`, so nothing compiles the mirror: the
// target the port-map names for this header is text, and no unit includes it.

/* C 18-23: `#include <pwd.h>`, `<malloc.h>`, `<direct.h>`, `<fcntl.h>`, `<io.h>`, `<stdio.h>` -
   the comment says they are included up front "so we don't have to worry about include order
   dependencies in common source files".  An `#include` has no Rust counterpart and there is no
   mirror for any of the six (they are the Windows/MSVC system headers), so they are recorded
   (the binary-io.rs dead-arm note).  */

/* C 25-27: `#ifdef sleep / #undef sleep / #endif` - the port has no `#undef`; the declaration
   below is what survives of that block.  */

/* C 29-38: the routines the Windows port shadows.  They are declarations, not definitions: the
   CRT provides them (`ntlib.c` defines some of them for the programs that link it).  */
unsafe extern "C" {
  /// C 29: `unsigned sleep (unsigned seconds);`
  pub fn sleep (seconds: c_uint) -> c_uint;
  /// C 30: `char *getwd (char *dir);`
  pub fn getwd (dir: *mut c_char) -> *mut c_char;
  /// C 31: `int getppid (void);`
  pub fn getppid () -> c_int;
  /// C 32: `char * getlogin (void);`
  pub fn getlogin () -> *mut c_char;
  /// C 33: `unsigned getegid (void);`
  pub fn getegid () -> c_uint;
  /// C 34: `unsigned getgid (void);`
  pub fn getgid () -> c_uint;
  /// C 35: `int setuid (unsigned uid);`
  pub fn setuid (uid: c_uint) -> c_int;
  /// C 36: `int setregid (unsigned rgid, unsigned gid);`
  pub fn setregid (rgid: c_uint, gid: c_uint) -> c_int;
  /// C 37: `char * getpass (const char * prompt);`
  pub fn getpass (prompt: *const c_char) -> *mut c_char;
  /// C 38: `int fchown (int fd, unsigned uid, unsigned gid);`
  pub fn fchown (fd: c_int, uid: c_uint, gid: c_uint) -> c_int;
}

/* C 41: "redirect or undo interceptions created by config.h".  Each pair is `#undef NAME` /
   `#define NAME _NAME`; a `#define` of a name has no Rust counterpart (PORTING.md 0.1), so the
   port carries the table as macros that expand to the renamed target - the C body's token is
   what the gate looks for, and nothing can call them (the header is dead on this target).  */
macro_rules! access { () => { _access } }        /* C 44-45 */
macro_rules! chdir { () => { _chdir } }          /* C 46-47 */
macro_rules! chmod { () => { _chmod } }          /* C 48-49 */
macro_rules! close { () => { _close } }          /* C 50-51 */
macro_rules! creat { () => { _creat } }          /* C 52-53 */
/* C 54: `#undef ctime` has no replacement - the port carries no definition for it.  */
macro_rules! dup { () => { _dup } }              /* C 55-56 */
macro_rules! dup2 { () => { _dup2 } }            /* C 57-58 */
/* C 59: `#undef fopen` likewise.  */
macro_rules! pipe { () => { _pipe } }            /* C 60-61 */
macro_rules! read { () => { _read } }            /* C 62-63 */
macro_rules! rmdir { () => { _rmdir } }          /* C 64-65 */
macro_rules! unlink { () => { _unlink } }        /* C 66-67 */
macro_rules! write { () => { _write } }          /* C 68-69 */

/* C 71: "map to MSVC names".  */
macro_rules! execlp { () => { _execlp } }        /* C 72 */
macro_rules! execvp { () => { _execvp } }        /* C 73 */
macro_rules! fdopen { () => { _fdopen } }        /* C 74 */
/* C 75-77: `#ifndef fileno / #define fileno _fileno / #endif`.  */
macro_rules! fileno { () => { _fileno } }
macro_rules! getcwd { () => { _getcwd } }        /* C 78 */
macro_rules! getw { () => { _getw } }            /* C 79 */
macro_rules! getpid { () => { _getpid } }        /* C 80 */
macro_rules! isatty { () => { _isatty } }        /* C 81 */
macro_rules! locking { () => { _locking } }      /* C 82 */
macro_rules! logb { () => { _logb } }            /* C 83 */
/* C 84: `#define _longjmp longjmp` - the one pair that maps *to* the POSIX spelling.  */
macro_rules! _longjmp { () => { longjmp } }
macro_rules! popen { () => { _popen } }          /* C 85 */
macro_rules! pclose { () => { _pclose } }        /* C 86 */
macro_rules! umask { () => { _umask } }          /* C 87 */
macro_rules! utime { () => { _utime } }          /* C 88 */
macro_rules! index { () => { strchr } }          /* C 89 */
macro_rules! rindex { () => { strrchr } }        /* C 90 */

/* C 92-95: "Make standard winsock definitions available if needed." -
   `#undef _WINSOCKAPI_` / `#undef _WINSOCK_H`, both bare undefs: recorded, not carried (the
   port has no `#undef`).  */
