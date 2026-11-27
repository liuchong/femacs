// femacs: mirror of src/systty.h @ fd7afced403.
//
// Live text on the frozen target (`DOS_NT`, `HPUX`, `AIX` are all undefined - macros.tsv):
// the include guard, C 26-31's `#include <termios.h>` / `<fcntl.h>` / `<sys/ioctl.h>` /
// `<unistd.h>`, the `CDISABLE` macro (C 50), `struct emacs_tty` (C 78-86) and the five
// declarations of C 89-93.
//
//  * `<termios.h>` becomes `include!("../../lib/include/termios.rs")`, the port's system mirror
//    for the four typedefs, `NCCS` and the measured `struct termios`; `<fcntl.h>` and
//    `<unistd.h>` have their mirrors.  `<sys/ioctl.h>` has none yet - its request numbers
//    (`TIOCGWINSZ` and friends) are named at the use sites that need them, and nothing in this
//    header names one.
//  * `CDISABLE`'s C body is the *system* macro `_POSIX_VDISABLE`, which the frozen target
//    defines as `((unsigned char)'\377')` (macros.tsv) - measured 255.  The value is therefore
//    written out; `docs/ports/macro-body-notes.tsv` records why the body is a number instead of
//    C's token (the same shape as the other system-macro rows), and the two units that already
//    spell it (`src/process.rs` 7417, `src/sysdep.rs` 624) use this exact form.
//  * `struct emacs_tty` holds one `struct termios main` because `DOS_NT` is undefined.
//
// C 20: the include guard has no Rust counterpart (PORTING.md 0.1).
// [U1-MODULE-MIRROR]
use super::*;
pub const EMACS_SYSTTY_H: bool = true;

// [U1] ../../lib/include/termios.rs imported via super
// [U1] ../../lib/include/fcntl.in.rs imported via super
// [U1] ../../lib/include/unistd.in.rs imported via super

/// C 50: `#define CDISABLE _POSIX_VDISABLE` - the frozen target's `_POSIX_VDISABLE` is
/// `((unsigned char)'\377')`, i.e. 255 (measured; macros.tsv).
#[cfg (not (target_os = "linux"))]
pub const CDISABLE: cc_t = 255;
#[cfg (target_os = "linux")]
pub const CDISABLE: cc_t = 0;

/// C 78-86: `struct emacs_tty { struct termios main; };` (the `DOS_NT` arm is dead).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct emacs_tty
{
  pub main: termios,
}

#[cfg (not (any (feature = "have_android", target_os = "linux", target_os = "windows")))]
const _: () = core::assert! (std::mem::size_of::<emacs_tty> () == 72);
/* `emacs_tty` is C's `struct termios` alone: 36B bionic, 60B Linux glibc, 72B Darwin. */
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<emacs_tty> () == 36);
#[cfg (target_os = "linux")]
const _: () = core::assert! (std::mem::size_of::<emacs_tty> () == 60);

pub mod systty_decls {
  use super::*;

  unsafe extern "C-unwind" {
    /* C 89: `extern int emacs_get_tty (int, struct emacs_tty *) EXTERNALLY_VISIBLE;`  */
    pub fn emacs_get_tty (fd: c_int, tty: *mut emacs_tty) -> c_int;
    /* C 90: `extern int emacs_set_tty (int, struct emacs_tty *, bool) EXTERNALLY_VISIBLE;`  */
    pub fn emacs_set_tty (fd: c_int, tty: *mut emacs_tty, flushp: bool) -> c_int;
    /* C 91: `extern void suppress_echo_on_tty (int);`  */
    pub fn suppress_echo_on_tty (fd: c_int);
    /* C 92: `extern int serial_open (Lisp_Object);`  */
    pub fn serial_open (port: Lisp_Object) -> c_int;
    /* C 93: `extern void serial_configure (struct Lisp_Process *, Lisp_Object);`
       - C names the incomplete type as a pointer only, so the port uses `*mut c_void`
       (PORTING.md P-72 (3)).  */
    pub fn serial_configure (p: *mut c_void, childp: Lisp_Object);
  }
}
pub use self::systty_decls::*;
