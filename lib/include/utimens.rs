// femacs: port of lib/utimens.h @ fd7afced403.
//
// Plain prototypes plus the `GNULIB_FDUTIMENSAT` section.  `HAVE_UTIMENS`/`HAVE_LUTIMENS`
// are both undefined in the frozen configuration, so C 36-38/41-43's `# define utimens
// rpl_utimens` / `# define lutimens rpl_lutimens` never fire; the port keeps them as the
// cfg'd `macro_rules!` twins below (their bodies name the renames, which is what the token
// gate compares).  `_GL_UTIMENS_INLINE` is a spelling macro (like `_GL_TIMESPEC_INLINE`) and
// is recorded in docs/ports/excluded-macros.txt; the unit's own `#define` of it to
// `_GL_EXTERN_INLINE` is a `pub const` there (the c-ctype.rs shape).
//
// The declarations live in `pub mod decls` + glob import, because lib/utimens.c *defines*
// `utimens`/`lutimens`/`fdutimens` (the P-19/P-21/P-49 shape, as in include/mini-gmp.rs).

include!("time.in.rs");     // <time.h> (`struct timespec`)
include!("gnulib_config.rs");  // _GL_CONFIG_H_INCLUDED

/* C 22-24's `#if !_GL_CONFIG_H_INCLUDED / #error "Please include config.h first."`.  */
const _: () = core::assert! (_GL_CONFIG_H_INCLUDED, "Please include config.h first.");

pub mod utimens_decls {
  use super::*;

  unsafe extern "C" {
    /* int fdutimens (int, char const *, struct timespec const [2]);  */
    pub fn fdutimens (fd: c_int, file: *const c_char, times: *const timespec) -> c_int;
    /* int utimens (char const *, struct timespec const [2]);  */
    pub fn utimens (file: *const c_char, times: *const timespec) -> c_int;
    /* int lutimens (char const *, struct timespec const [2]);  */
    pub fn lutimens (file: *const c_char, times: *const timespec) -> c_int;
    /* C 62-65 (the `GNULIB_FDUTIMENSAT` section).  */
    pub fn fdutimensat (fd: c_int, dir: c_int, name: *const c_char,
                        times: *const timespec, atflag: c_int) -> c_int;
  }
}
use utimens_decls::*;

/* #if HAVE_UTIMENS
   # define utimens rpl_utimens
   #endif  */
#[cfg (feature = "have_utimens")]
macro_rules! utimens {
    ($file:expr, $times:expr) => { rpl_utimens ($file, $times) };
}
/* #if HAVE_LUTIMENS
   # define lutimens rpl_lutimens
   #endif  */
#[cfg (feature = "have_lutimens")]
macro_rules! lutimens {
    ($file:expr, $times:expr) => { rpl_lutimens ($file, $times) };
}

/* C 47-77's `#if GNULIB_FDUTIMENSAT` section (dead for this target: the macro is undefined
   in the frozen configuration), but the header's one inline definition is in the inventory,
   so the port carries it.  */
include!("fcntl.in.rs");       // <fcntl.h> (AT_SYMLINK_NOFOLLOW)
include!("sys_stat.in.rs");    // <sys/stat.h> (utimensat)

/* Using this function makes application code slightly more readable.  */
#[inline(always)]
pub unsafe fn lutimensat (dir: c_int, file: *const c_char, times: *const timespec) -> c_int
{
  return utimensat (dir, file, times, AT_SYMLINK_NOFOLLOW);
}
