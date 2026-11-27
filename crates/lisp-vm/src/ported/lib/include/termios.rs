// femacs: mirror of the frozen target's <termios.h> (system header, P-26/P-50).
#![forbid(unsafe_code)]
//
// gnulib has no `termios` wrapper in this build; the reference includes the system header, so
// this carries the entries the ported units name.  The layout and the typedef widths are the
// frozen target's, measured with clang:
//
//     $ cat > /tmp/tm.c <<'EOF'
//     #include <stdio.h>
//     #include <stddef.h>
//     #include <termios.h>
//     int main (void) { printf ("sizeof=%zu iflag=%zu oflag=%zu cflag=%zu lflag=%zu cc=%zu
//         ispeed=%zu ospeed=%zu NCCS=%d\n", sizeof (struct termios),
//         offsetof (struct termios, c_iflag), offsetof (struct termios, c_oflag),
//         offsetof (struct termios, c_cflag), offsetof (struct termios, c_lflag),
//         offsetof (struct termios, c_cc), offsetof (struct termios, c_ispeed),
//         offsetof (struct termios, c_ospeed), NCCS); return 0; }
//     EOF
//     $ clang -o /tmp/tm /tmp/tm.c && /tmp/tm
//     sizeof=72 iflag=0 oflag=8 cflag=16 lflag=24 cc=32 ispeed=56 ospeed=64 NCCS=20
//
// i.e. four 8-byte flag words, the 20-byte `c_cc` array, four padding bytes, then the two
// 8-byte speed words.
//
// The `tcgetattr`/`tcsetattr`/`tcflow`/… entry points and the `ioctl` request numbers are not
// here yet: the three units that need them today (`src/sysdep.rs` 112-130,
// `src/process.rs` 7390-7430, `src/minibuf.rs` 87-95) each carry their own
// measured copy, and that convergence is a separate, lane-visible step recorded in
// docs/ports/include-termios.md.

// [U1-MODULE-MIRROR]
use super::*;
/// <termios.h>: `typedef unsigned long tcflag_t;` (measured `sizeof` 8 on the host; bionic's is
/// `unsigned int`, 4 bytes - status 4.795).
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
pub type tcflag_t = c_ulong;
#[cfg (any (feature = "have_android", target_os = "linux"))]
pub type tcflag_t = c_uint;

/// <termios.h>: `typedef unsigned long speed_t;` (8 on the host, 4 on bionic/Linux).
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
pub type speed_t = c_ulong;
#[cfg (any (feature = "have_android", target_os = "linux"))]
pub type speed_t = c_uint;

/// <termios.h>: `typedef unsigned char cc_t;`.
pub type cc_t = c_uchar;

/// <termios.h>: `#define NCCS 20` (Darwin), `19` (bionic), `32` (Linux glibc).
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
pub const NCCS: c_int = 20;
#[cfg (feature = "have_android")]
pub const NCCS: c_int = 19;
#[cfg (target_os = "linux")]
pub const NCCS: c_int = 32;

/// <termios.h>'s `struct termios`: 72 bytes on the host.  ``(measured: 72 bytes, the offsets
/// above)``.
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct termios
{
  pub c_iflag: tcflag_t,   /* 0 */
  pub c_oflag: tcflag_t,   /* 8 */
  pub c_cflag: tcflag_t,   /* 16 */
  pub c_lflag: tcflag_t,   /* 24 */
  pub c_cc: [cc_t; NCCS as usize], /* 32 */
  pub c_ispeed: speed_t,   /* 56 */
  pub c_ospeed: speed_t,   /* 64 */
}

#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::size_of::<termios> () == 72);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (termios, c_iflag) == 0);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (termios, c_oflag) == 8);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (termios, c_cflag) == 16);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (termios, c_lflag) == 24);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (termios, c_cc) == 32);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (termios, c_ispeed) == 56);
#[cfg (not (any (feature = "have_android", target_os = "linux")))]
const _: () = core::assert! (std::mem::offset_of! (termios, c_ospeed) == 64);

/// glibc Linux `struct termios`: 60 bytes, `tcflag_t`/`speed_t` 4 bytes, `c_line` at 16,
/// `c_cc[32]` at 17, `c_ispeed` at 52, `c_ospeed` at 56.
#[cfg (target_os = "linux")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct termios
{
  pub c_iflag: tcflag_t,           /* 0 */
  pub c_oflag: tcflag_t,           /* 4 */
  pub c_cflag: tcflag_t,           /* 8 */
  pub c_lflag: tcflag_t,           /* 12 */
  pub c_line: cc_t,                /* 16 */
  pub c_cc: [cc_t; NCCS as usize], /* 17 */
  pub _pad0: [u8; 3],              /* 49 */
  pub c_ispeed: speed_t,           /* 52 */
  pub c_ospeed: speed_t,           /* 56 */
}

#[cfg (target_os = "linux")]
const _: () = core::assert! (std::mem::size_of::<termios> () == 60);
#[cfg (target_os = "linux")]
const _: () = core::assert! (std::mem::offset_of! (termios, c_line) == 16);
#[cfg (target_os = "linux")]
const _: () = core::assert! (std::mem::offset_of! (termios, c_cc) == 17);
#[cfg (target_os = "linux")]
const _: () = core::assert! (std::mem::offset_of! (termios, c_ispeed) == 52);
#[cfg (target_os = "linux")]
const _: () = core::assert! (std::mem::offset_of! (termios, c_ospeed) == 56);

/// bionic's `struct termios`: 36 bytes, `tcflag_t` 4 bytes, `c_line` before `c_cc[19]`, and no
/// `c_ispeed`/`c_ospeed` members at all (NDK, aarch64-linux-android21: size 36, `c_cc` at 17,
/// `c_cc` is 19 bytes, status 4.795).
#[cfg (feature = "have_android")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct termios
{
  pub c_iflag: tcflag_t,   /* 0 */
  pub c_oflag: tcflag_t,   /* 4 */
  pub c_cflag: tcflag_t,   /* 8 */
  pub c_lflag: tcflag_t,   /* 12 */
  pub c_line: cc_t,        /* 16 */
  pub c_cc: [cc_t; NCCS as usize], /* 17 */
}

#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<termios> () == 36);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (termios, c_iflag) == 0);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (termios, c_oflag) == 4);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (termios, c_cflag) == 8);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (termios, c_lflag) == 12);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (termios, c_line) == 16);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (termios, c_cc) == 17);
