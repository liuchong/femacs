// femacs: port of lib/stdio-impl.h @ fd7afced403 (PORTING.md P-26).
#![forbid(unsafe_code)]
//
// The header decides, from the platform macros, how a stream's fields are named, and carries
// one arm per stdio implementation (glibc's private `_IO_*`, Haiku, the BSD family, DragonFly,
// OpenBSD, Android, the System-V family, MSVC's opaque `FILE`).  The frozen configuration
// compiles the BSD family arm, and what it defines was measured with the frozen compiler
// (`<stdio.h>` first: `__sferror` is one of *its* macros, so without it the whole arm is
// skipped and the header looks empty):
//
//     $ printf '#include <stdio.h>\n#include "stdio-impl.h"\n' > /tmp/si.c
//     $ clang -E -dD -I emacs/lib -I build/emacs-ref/lib -I build/emacs-ref/src -I emacs/src \
//         -include config.h /tmp/si.c | grep '^#define'
//     #define fp_ fp                        <- C 198, the "FreeBSD, NetBSD <= 1.5Z,
//     #define fp_ub fp_->_ub                   DragonFly, Mac OS X, Cygwin" `#else` arms
//     #define HASUB(fp) (fp_ub._base != NULL)
//
// i.e. on this target `fp_` is the stream itself and `fp_ub` its ungetc buffer; every other
// arm (`_IO_*`, `_IOEOF`, EPLAN9, `__EMX__`, `__UCLIBC__`, `__QNX__`, `__MINT__`, the Windows
// `_gl_real_FILE`, the NetBSD/Minix `struct __sfileext`) is dead text here, and C's own
// `__S*` fallbacks are dead because Darwin's `<stdio.h>` defines the whole family (measured:
// `__SLBF` 1, `__SNBF` 2, `__SRD` 4, `__SWR` 8, `__SRW` 0x10, `__SEOF` 0x20, `__SERR` 0x40,
// `__SOFF` 0x1000).  `tools/extract-abi.py` reads a header's `#if`s blind unless it is in
// docs/baseline/config/cond-eval-verified.txt, and this one is not, so the mirror names every
// macro the header can define - the live three as macros, the dead arms as text under the
// feature naming the C condition (P-51).
//
// `FILE` is `struct __sFILE` (Darwin's <stdio.h>): the layout below is measured with the
// frozen configuration's clang, and the struct is cut off after `_bf` (the last field the
// ported units read) with a padding tail that keeps `sizeof` at the measured 152:
//
//     $ cat > /tmp/fl.c <<'EOF'
//     #include <stdio.h>
//     #include <stddef.h>
//     int main (void) { printf ("%zu %zu %zu %zu %zu %zu %d\n", sizeof (FILE),
//         offsetof (FILE, _p), offsetof (FILE, _flags), offsetof (FILE, _bf),
//         offsetof (FILE, _bf._base), offsetof (FILE, _bf._size), __SERR); return 0; }
//     EOF
//     $ clang -o /tmp/fl /tmp/fl.c && /tmp/fl
//     152 0 16 24 24 32 64
//
// The fields the units read are `_p`, `_bf._base`, `_bf._size` (lib/fpending.c) and `_flags`
// (lib/fseterr.c, lib/close-stream.c); the `fp_`/`fp_ub`/`HASUB` macros below are C's spelling
// of the same accesses for the units that use them.

// [U1-MODULE-MIRROR]
use super::*;
/// Darwin's `struct __sbuf`: `unsigned char *_base; int _size;`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct __sbuf
{
  pub _base: *mut u8,
  pub _size: core::ffi::c_int,
}

/// The frozen target's `FILE`, i.e. Darwin's `struct __sFILE`.  The declaration is cut off
/// after `_bf`: C's remaining members (`_lbfsize`, `_cookie`, the four operations, `_ub`,
/// `_extra`, `_ur`, `_ubuf`, `_nbuf`, `_lb`, `_blksize`, `_offset`) are not read by any
/// ported unit, and `_rest` keeps the total size at the measured 152 so a wrong prefix
/// cannot pass unnoticed.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct __sFILE
{
  pub _p: *mut u8,
  /* Absolute paths, not the including unit's bare aliases: this mirror adds a type the
     closure of older units does not import (PORTING.md 0.1-9/3.18①, the stdio.rs
     `core::ffi::c_void` precedent).  */
  pub _r: core::ffi::c_int,
  pub _w: core::ffi::c_int,
  pub _flags: core::ffi::c_short,
  pub _file: core::ffi::c_short,
  pub _bf: __sbuf,
  pub _rest: [u8; 112],
}

/// The name C's <stdio.h> gives it, so every unit keeps writing `*mut FILE`.
pub type FILE = __sFILE;

const _: () = core::assert! (std::mem::size_of::<__sFILE> () == 152);
const _: () = core::assert! (std::mem::offset_of! (__sFILE, _p) == 0);
const _: () = core::assert! (std::mem::offset_of! (__sFILE, _r) == 8);
const _: () = core::assert! (std::mem::offset_of! (__sFILE, _w) == 12);
const _: () = core::assert! (std::mem::offset_of! (__sFILE, _flags) == 16);
const _: () = core::assert! (std::mem::offset_of! (__sFILE, _file) == 18);
const _: () = core::assert! (std::mem::offset_of! (__sFILE, _bf) == 24);
const _: () = core::assert! (std::mem::offset_of! (__sbuf, _base) == 0);
const _: () = core::assert! (std::mem::offset_of! (__sbuf, _size) == 8);

/* `<stdio.h>`'s stream flags: C 223-240 falls back to its own values only when the system
   header has none, and Darwin defines all eight with the numbers measured above.  */
pub const __SLBF: core::ffi::c_int = 0x0001;
pub const __SNBF: core::ffi::c_int = 0x0002;
pub const __SRD: core::ffi::c_int = 0x0004;
pub const __SWR: core::ffi::c_int = 0x0008;
pub const __SRW: core::ffi::c_int = 0x0010;
pub const __SEOF: core::ffi::c_int = 0x0020;
/// `<stdio.h>`'s `#define __SERR 0x0040` (measured: `__SERR` prints 64).
pub const __SERR: core::ffi::c_int = 64;
pub const __SOFF: core::ffi::c_int = 0x1000;

/* C 197-199: `# define fp_ fp` - the arm this target takes (the `# else` of the DragonFly /
   OpenBSD / Android chain inside the BSD family block; see the measurement above).  C's macro
   is object-like, so a C site writes `fp_->_p`; the port's macro takes the stream, and its
   call site writes `fp_! (fp)._p` (the `MINI_GMP_LIMB_TYPE`/type-macro shape of P-49's
   siblings).  Nothing in the port expands it - lib/fpending.rs and lib/fseterr.rs read the
   measured fields directly - so it is C's spelling of the same access, carried because the
   extractor asks for the name.  */
macro_rules! fp_ { ($fp:expr) => { $fp } }

/* C 217-219: `# define fp_ub fp_->_ub` - the same arm's ungetc buffer (the `# else` of the
   `__minix`/`__ANDROID__` chain).  The mirror's `__sFILE` is cut off after `_bf`, so C's
   `_ub` member is inside `_rest` here: the macro is C's spelling of the access, and nothing
   in the port expands it (no ported unit reads the ungetc buffer).  */
macro_rules! fp_ub { ($fp:expr) => { (*fp_! ($fp))._ub } }

/* C 221: `# define HASUB(fp) (fp_ub._base != NULL)` - "the stream's ungetc buffer is not
   empty", which only that arm defines.  */
macro_rules! HASUB { ($fp:expr) => { (fp_ub! ($fp)._base != core::ptr::null_mut ()) } }
/* ---- the arms of the other platforms (PORTING.md P-51) ----------------------------------
   Every item below is text the frozen configuration does not compile; the feature names the
   C condition that selects it.  The extractor reads this header blind, so each name has to be
   here.  */

/* C 24-31 (`#if defined _IO_EOF_SEEN`, glibc) and C 62-73 (Haiku): the four `_IO_*` markers
   the arm defines for itself.  */
#[cfg (feature = "glibc_io_eof_seen")]
pub const _IO_UNBUFFERED: core::ffi::c_int = 0x2;
#[cfg (feature = "glibc_io_eof_seen")]
pub const _IO_IN_BACKUP: core::ffi::c_int = 0x100;
#[cfg (feature = "have_haiku")]
pub const _IO_EOF_SEEN: core::ffi::c_int = 0x10;
#[cfg (feature = "have_haiku")]
pub const _IO_LINE_BUF: core::ffi::c_int = 0x200;

/* C 107-110: the DragonFly arm's field renames (`# define _p pub._p`, …) - the field names of
   DragonFly's `struct __FILE_public`; C never expands them here.  */
#[cfg (feature = "dragonfly")]
macro_rules! _p { ($fp:expr) => { $fp.pub._p } }
#[cfg (feature = "dragonfly")]
macro_rules! _flags { ($fp:expr) => { $fp.pub._flags } }
#[cfg (feature = "dragonfly")]
macro_rules! _r { ($fp:expr) => { $fp.pub._r } }
#[cfg (feature = "dragonfly")]
macro_rules! _w { ($fp:expr) => { $fp.pub._w } }

/* C 113-121: `_gl_flags_file_t` is OpenBSD's `short` (OpenBSD <= 7.7) or `int` (>= 7.8), and
   Android's `int` (`__LP64__`) or `short`; a type macro, so the port's spelling is the type
   (`struct_stat64`-shape, lib/tempname.rs 77).  */
#[cfg (feature = "openbsd_sferror")]
macro_rules! _gl_flags_file_t { () => { core::ffi::c_short } }
#[cfg (feature = "openbsd")]
macro_rules! _gl_flags_file_t { () => { core::ffi::c_int } }
#[cfg (feature = "android_64bit")]
macro_rules! _gl_flags_file_t { () => { core::ffi::c_int } }
#[cfg (feature = "android_32bit")]
macro_rules! _gl_flags_file_t { () => { core::ffi::c_short } }

/* C 148-156: the Android arm's file-offset type (`int64_t`, or `__kernel_off_t` on 32-bit
   Android).  */
#[cfg (feature = "android_64bit")]
macro_rules! _gl_file_offset_t { () => { i64 } }
#[cfg (feature = "android_32bit")]
macro_rules! _gl_file_offset_t { () => { __kernel_off_t } }

/* C 201-219: the `# if (__NetBSD__ …) || defined __minix` arm's `struct __sfileext`, whose
   only member is the ungetc buffer (the `# elif defined __ANDROID__ || defined __OpenBSD__`
   arm spells the member as an anonymous `struct { unsigned char *_base; size_t _size; }`; the
   field the extractor names is `_ub`, and this target's `__sbuf` is the same pair).  */
#[cfg (feature = "netbsd_minix_fileext")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct __sfileext
{
  pub _ub: __sbuf,
}

/* C 209/216: the two dead `fp_ub` arms, which read the buffer out of the extended record.  */
#[cfg (feature = "netbsd_minix_fileext")]
macro_rules! fp_ub { ($fp:expr) => { (*(*fp_! ($fp))._ext._base)._ub } }
#[cfg (feature = "openbsd_fileext")]
macro_rules! fp_ub { ($fp:expr) => { (*(*fp_! ($fp))._ext._base)._ub } }

/* C 223-240: the Android/OpenBSD arm's `__S*` fallbacks.  Darwin defines the whole family
   (the measured values are the constants above), so these arms are dead.  `__SEOF` is the
   guard and `__SOFF` its sibling.  */
#[cfg (feature = "openbsd_or_android_flags")]
pub const __SLBF: core::ffi::c_int = 1;
#[cfg (feature = "openbsd_or_android_flags")]
pub const __SNBF: core::ffi::c_int = 2;
#[cfg (feature = "openbsd_or_android_flags")]
pub const __SRD: core::ffi::c_int = 4;
#[cfg (feature = "openbsd_or_android_flags")]
pub const __SWR: core::ffi::c_int = 8;
#[cfg (feature = "openbsd_or_android_flags")]
pub const __SRW: core::ffi::c_int = 0x10;
#[cfg (feature = "openbsd_or_android_flags")]
pub const __SEOF: core::ffi::c_int = 0x20;
#[cfg (feature = "openbsd_or_android_flags")]
pub const __SERR: core::ffi::c_int = 0x40;
#[cfg (feature = "openbsd_or_android_flags")]
pub const __SOFF: core::ffi::c_int = 0x1000;

/* C 247-256: the Tandem (NonStop Kernel) arm's System-V flag values.  */
#[cfg (feature = "tandem")]
pub const _IOERR: core::ffi::c_int = 0x40;
#[cfg (feature = "tandem")]
pub const _IOREAD: core::ffi::c_int = 0x80;
#[cfg (feature = "tandem")]
pub const _IOWRT: core::ffi::c_int = 0x4;
#[cfg (feature = "tandem")]
pub const _IORW: core::ffi::c_int = 0x100;

/* C 274-279: the SCO / UnixWare arm's `# define _cnt __cnt` family - the field renames of
   another FILE layout.  */
#[cfg (feature = "sco")]
macro_rules! _cnt { () => { __cnt } }
#[cfg (feature = "sco")]
macro_rules! _ptr { () => { __ptr } }
#[cfg (feature = "sco")]
macro_rules! _base { () => { __base } }
#[cfg (feature = "sco")]
macro_rules! _flag { () => { __flag } }

/* C 281-307: the MSVC arm - an opaque `FILE`, so C 286-297 declares the layout it walks and
   C 302-306 the flag values it tests.  */
#[cfg (feature = "windows_msvc")]
pub const WINDOWS_OPAQUE_FILE: bool = true;
#[cfg (feature = "windows_msvc")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _gl_real_FILE
{
  /* Note: Compared to older Windows and to mingw, it has the fields
     _base and _cnt swapped.  */
  pub _ptr: *mut u8,
  pub _base: *mut u8,
  pub _cnt: core::ffi::c_int,
  pub _flag: core::ffi::c_int,
  pub _file: core::ffi::c_int,
  pub _charbuf: core::ffi::c_int,
  pub _bufsiz: core::ffi::c_int,
}
#[cfg (feature = "windows_msvc")]
pub const _IOREAD: core::ffi::c_int = 0x1;
#[cfg (feature = "windows_msvc")]
pub const _IOWRT: core::ffi::c_int = 0x2;
#[cfg (feature = "windows_msvc")]
pub const _IORW: core::ffi::c_int = 0x4;
#[cfg (feature = "windows_msvc")]
pub const _IOEOF: core::ffi::c_int = 0x8;
#[cfg (feature = "windows_msvc")]
pub const _IOERR: core::ffi::c_int = 0x10;
