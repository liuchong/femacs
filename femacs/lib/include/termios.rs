// femacs: mirror of the frozen target's <termios.h> (system header, P-26/P-50).
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
// here yet: the three units that need them today (`femacs/src/sysdep.rs` 112-130,
// `femacs/src/process.rs` 7390-7430, `femacs/src/minibuf.rs` 87-95) each carry their own
// measured copy, and that convergence is a separate, lane-visible step recorded in
// docs/ports/include-termios.md.

/// <termios.h>: `typedef unsigned long tcflag_t;` (measured `sizeof` 8).
pub type tcflag_t = c_ulong;

/// <termios.h>: `typedef unsigned long speed_t;` (measured `sizeof` 8).
pub type speed_t = c_ulong;

/// <termios.h>: `typedef unsigned char cc_t;`.
pub type cc_t = c_uchar;

/// <termios.h>: `#define NCCS 20` (measured).
pub const NCCS: c_int = 20;

/// <termios.h>'s `struct termios` (measured: 72 bytes, the offsets above).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct termios
{
  pub c_iflag: tcflag_t,   /* 0 */
  pub c_oflag: tcflag_t,   /* 8 */
  pub c_cflag: tcflag_t,   /* 16 */
  pub c_lflag: tcflag_t,   /* 24 */
  pub c_cc: [cc_t; 20],    /* 32 */
  pub c_ispeed: speed_t,   /* 56 */
  pub c_ospeed: speed_t,   /* 64 */
}

const _: () = assert! (std::mem::size_of::<termios> () == 72);
const _: () = assert! (std::mem::offset_of! (termios, c_iflag) == 0);
const _: () = assert! (std::mem::offset_of! (termios, c_oflag) == 8);
const _: () = assert! (std::mem::offset_of! (termios, c_cflag) == 16);
const _: () = assert! (std::mem::offset_of! (termios, c_lflag) == 24);
const _: () = assert! (std::mem::offset_of! (termios, c_cc) == 32);
const _: () = assert! (std::mem::offset_of! (termios, c_ispeed) == 56);
const _: () = assert! (std::mem::offset_of! (termios, c_ospeed) == 64);
