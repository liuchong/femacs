#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_macros, unused_variables, unused_mut, unused_unsafe)]
// [U1-MODULE-UNIT]
use crate::sys_common::*;
use std::ffi::{c_char, c_double, c_int, c_long, c_short, c_uint, c_ulong, c_void};
// femacs: port of lib/fpending.c @ fd7afced403.  Blind file: the frozen
// configuration defines `__sferror` (macros.tsv 7030) and neither `_IO_EOF_SEEN` nor
// `_IO_ftrylockfile` nor `__GNU_LIBRARY__`, so C 42-44's BSD arm is the live one and
// `stdio-impl.h` 198 gives `fp_` as plain `fp`.  C's other eight arms need other platforms'
// <stdio.h> layouts and are carried here as the reference's own text (P-51; blanking them
// would hide the `__sFILE`/glibc/musl field names the ledger compares).  The reference object
// confirms the symbol: `nm build/emacs-ref/lib/fpending.o` prints `T ___fpending`.
// [U1] config.rs provided by common
// [U1] include/gnulib_config.rs imported via common
// [U1] include/stdio.rs imported via common
// [U1] include/fpending.rs imported via common
// [U1] include/stdio-impl.rs imported via common

/* This file is not used on systems that already have the __fpending function,
   namely glibc >= 2.2, OpenBSD >= 7.6, Solaris >= 7, UnixWare >= 7.1.4.MP4,
   Cygwin >= 1.7.34, Android API >= 23, musl libc, Haiku >= hrev58760.  */

/* Return the number of pending (aka buffered, unflushed)
   bytes on the stream, FP, that is open for writing.  */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __fpending (fp: *mut FILE) -> usize
{
  // SAFETY: POSIX/gnulib C-ABI boundary wrapper; caller upholds pointer/descriptor preconditions and checks `errno`/return status on failure.
  unsafe {
  /* Most systems provide FILE as a struct and the necessary bitmask in
     <stdio.h>, because they need it for implementing getc() and putc() as
     fast macros.  */

  /* verbatim: only C 42-44 is this target's arm
  // #if defined _IO_EOF_SEEN || defined _IO_ftrylockfile || __GNU_LIBRARY__ == 1
  //   /* GNU libc, BeOS, Haiku, Linux libc5 */
  //   return fp->_IO_write_ptr - fp->_IO_write_base;
  // #elif defined __sferror || defined __DragonFly__ || defined __ANDROID__
  //   /* FreeBSD, NetBSD, OpenBSD < 7.6, DragonFly, Mac OS X, Cygwin < 1.7.34, Minix 3, Android */
  //   return fp_->_p - fp_->_bf._base;
  // #elif defined __EMX__                /* emx+gcc */
  //   return fp->_ptr - fp->_buffer;
  // #elif defined __minix                /* Minix */
  //   return fp_->_ptr - fp_->_buf;
  // #elif defined _IOERR                 /* AIX, HP-UX, Solaris, OpenServer, UnixWare, mingw, MSVC, NonStop Kernel, OpenVMS */
  //   return (fp_->_ptr ? fp_->_ptr - fp_->_base : 0);
  // #elif defined __UCLIBC__             /* uClibc */
  //   return (fp->__modeflags & __FLAG_WRITING ? fp->__bufpos - fp->__bufstart : 0);
  // #elif defined __QNX__                /* QNX */
  //   return (fp->_Mode & 0x2000 /*_MWRITE*/ ? fp->_Next - fp->_Buf : 0);
  // #elif defined __MINT__               /* Atari FreeMiNT */
  //   return fp->__bufp - fp->__buffer;
  // #elif defined EPLAN9                 /* Plan9 */
  //   return fp->wp - fp->buf;
  // #else
  // # error "Please port gnulib fpending.c to your platform!"
  //   return 1;
  // #endif
     */

  return ((*fp)._p as isize - (*fp)._bf._base as isize) as usize;

  }
}
