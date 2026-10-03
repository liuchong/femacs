// femacs: the part of lib/stdio-impl.h the port reaches (PORTING.md P-26), together with the
// frozen target's `FILE` layout that lib/fpending.c and lib/fseterr.c read through `fp_`.
//
// stdio-impl.h 85-242 decides, from the platform macros, how a stream's fields are named; on
// this target `__sferror` is defined (macros.tsv 7030), so C 85's BSD family arm is live, its
// C 217-219 `# else / # define fp_ub fp_->_ub / # endif` is the arm for Mac OS X and C 197-199
// gives `fp_` as plain `fp`.  The `_IO_*`/`_IOERR`/`EPLAN9`/`__EMX__`/`__UCLIBC__`/`__QNX__`/
// `__MINT__` families are other platforms' arms and are not carried.
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
// The fields the three units read are `_p`, `_bf._base`, `_bf._size` (lib/fpending.c) and
// `_flags` (lib/fseterr.c, lib/close-stream.c).

/// Darwin's `struct __sbuf`: `unsigned char *_base; int _size;`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct __sbuf
{
  pub _base: *mut u8,
  pub _size: c_int,
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
  pub _r: c_int,
  pub _w: c_int,
  pub _flags: c_short,
  pub _file: c_short,
  pub _bf: __sbuf,
  pub _rest: [u8; 112],
}

/// The name C's <stdio.h> gives it, so every unit keeps writing `*mut FILE`.
pub type FILE = __sFILE;

const _: () = assert! (std::mem::size_of::<__sFILE> () == 152);
const _: () = assert! (std::mem::offset_of! (__sFILE, _p) == 0);
const _: () = assert! (std::mem::offset_of! (__sFILE, _r) == 8);
const _: () = assert! (std::mem::offset_of! (__sFILE, _w) == 12);
const _: () = assert! (std::mem::offset_of! (__sFILE, _flags) == 16);
const _: () = assert! (std::mem::offset_of! (__sFILE, _file) == 18);
const _: () = assert! (std::mem::offset_of! (__sFILE, _bf) == 24);
const _: () = assert! (std::mem::offset_of! (__sbuf, _base) == 0);
const _: () = assert! (std::mem::offset_of! (__sbuf, _size) == 8);

/// `<stdio.h>`'s `#define __SERR 0x0040` (measured: `__SERR` prints 64).
pub const __SERR: c_int = 64;
