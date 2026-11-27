// femacs: mirror of the frozen target's <utmpx.h> (system header, P-26; the
// include/sys_sysctl.rs and include/sys_mount.rs shape).  There is no reference file to pair
// with, so this carries what the ported unit names: `struct utmpx`, `BOOT_TIME` and the four
// utmp entries.  lib/boot-time.c reaches the header through `#include "readutmp.h"` (C 54),
// whose frozen-configuration arm (readutmp.h 170-178) is `UTMP_STRUCT_NAME utmpx` with
// `SET_UTMP_ENT setutxent` / `GET_UTMP_ENT getutxent` / `END_UTMP_ENT endutxent` /
// `UTMP_NAME_FUNCTION utmpxname`; it is a relative include, not a system one, so the
// readutmp-side names stay in the unit (see docs/ports/boot-time.md).
//
// The layout is the frozen target's, measured with the frozen configuration:
//
//     $ cat > /tmp/ut.c <<'EOF'
//     #include <config.h>
//     #include "readutmp.h"
//     #include <stddef.h>
//     #include <stdio.h>
//     int main (void) {
//       printf ("%zu | %zu %zu %zu %zu %zu %zu %zu | %zu %zu %zu\n", sizeof (struct utmpx),
//               offsetof (struct utmpx, ut_user), offsetof (struct utmpx, ut_id),
//               offsetof (struct utmpx, ut_line), offsetof (struct utmpx, ut_pid),
//               offsetof (struct utmpx, ut_type), offsetof (struct utmpx, ut_tv),
//               offsetof (struct utmpx, ut_host), sizeof (struct timeval),
//               offsetof (struct timeval, tv_sec), offsetof (struct timeval, tv_usec));
//       return 0; }
//     EOF
//     $ clang -std=gnu23 -I build/emacs-ref/src -I emacs/src -I emacs/lib \
//             -I build/emacs-ref/lib /tmp/ut.c -o /tmp/ut && /tmp/ut
//     640 | 0 256 260 292 296 304 320 | 16 0 8
//
// i.e. `ut_user` is a 256-byte array at 0, `ut_id` 4 bytes at 256, `ut_line` 32 bytes at 260,
// `ut_pid` (a `pid_t`, 4 bytes) at 292, `ut_type` (2 bytes) at 296, `ut_tv` (a `struct
// timeval`, 16 bytes) at 304, `ut_host` 256 bytes at 320 and 64 bytes of `ut_pad[16]` tail;
// the assertions below pin every one of those numbers.

// [U1-MODULE-MIRROR]
use super::*;
// [U1] sys_time.rs imported via super
// [U1] sys_types.in.rs imported via super

/// <utmpx.h> 82-85: the four field sizes (`_UTX_USERSIZE` 256, `_UTX_LINESIZE` 32,
/// `_UTX_IDSIZE` 4, `_UTX_HOSTSIZE` 256 on the frozen target).
pub const _UTX_USERSIZE: usize = 256;
pub const _UTX_LINESIZE: usize = 32;
pub const _UTX_IDSIZE: usize = 4;
pub const _UTX_HOSTSIZE: usize = 256;

/* struct utmpx
   {
     char ut_user[_UTX_USERSIZE];
     char ut_id[_UTX_IDSIZE];
     char ut_line[_UTX_LINESIZE];
     pid_t ut_pid;
     short ut_type;
     struct timeval ut_tv;
     char ut_host[_UTX_HOSTSIZE];
     __uint32_t ut_pad[16];
   };  */
/// <utmpx.h>'s `struct utmpx` on the host: 640 bytes, `ut_user` first.
#[cfg (not (feature = "have_android"))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct utmpx
{
  pub ut_user: [c_char; _UTX_USERSIZE],
  pub ut_id: [c_char; _UTX_IDSIZE],
  pub ut_line: [c_char; _UTX_LINESIZE],
  pub ut_pid: pid_t,
  pub ut_type: c_short,
  pub ut_tv: timeval,
  pub ut_host: [c_char; _UTX_HOSTSIZE],
  pub ut_pad: [c_uint; 16],
}

#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::size_of::<utmpx> () == 640);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_user) == 0);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_id) == 256);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_line) == 260);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_pid) == 292);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_type) == 296);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_tv) == 304);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_host) == 320);
#[cfg (not (feature = "have_android"))]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_pad) == 576);

/// bionic's `struct utmpx`: 400 bytes, `ut_type` first and the string fields 32/4/32 bytes
/// (NDK, aarch64-linux-android21: size 400, `ut_pid` 4, `ut_line` 8, `ut_id` 40, `ut_user` 44,
/// `ut_host` 76, `ut_exit` 332, `ut_session` 336, `ut_tv` 344, `ut_addr_v6` 360 - status 4.796).
#[cfg (feature = "have_android")]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct utmpx
{
  pub ut_type: c_short,          /* 0 */
  pub _pad0: [u8; 2],            /* 2 */
  pub ut_pid: pid_t,             /* 4 */
  pub ut_line: [c_char; 32],     /* 8 */
  pub ut_id: [c_char; 4],        /* 40 */
  pub ut_user: [c_char; 32],     /* 44 */
  pub ut_host: [c_char; 256],    /* 76 */
  pub ut_exit: [c_short; 2],     /* 332 */
  pub ut_session: c_int,         /* 336 */
  pub _pad1: [u8; 4],            /* 340 */
  pub ut_tv: timeval,            /* 344 */
  pub ut_addr_v6: [c_int; 4],    /* 360 */
  pub ut_reserved: [c_int; 6],   /* 376 */
}

#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::size_of::<utmpx> () == 400);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_pid) == 4);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_line) == 8);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_id) == 40);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_user) == 44);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_host) == 76);
#[cfg (feature = "have_android")]
const _: () = core::assert! (std::mem::offset_of! (utmpx, ut_tv) == 344);

/// <utmpx.h> 89: `#define BOOT_TIME 2`, the `ut_type` value lib/boot-time.c compares
/// against (`USER_PROCESS` is 7 and is not named by this unit).
pub const BOOT_TIME: c_short = 2;

unsafe extern "C" {
  /* struct utmpx *getutxent (void);  */
  pub fn getutxent () -> *mut utmpx;
  /* void setutxent (void);  */
  pub fn setutxent ();
  /* void endutxent (void);  */
  pub fn endutxent ();
  /* int utmpxname (const char *);  */
  pub fn utmpxname (file: *const c_char) -> c_int;
}
