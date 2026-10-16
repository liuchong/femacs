// femacs: port of src/process.c @ fd7afced403.
//
// One-to-one port (PORTING.md): same statements in the same order, same names (function,
// struct, union, enum, macro, local), same control flow, same error model.  C's 64 `DEFUN`s use
// `DEFUN!` (PORTING.md 5.1); C's non-`static` definitions carry `#[unsafe(no_mangle)]`
// (PORTING.md 0.2) and the C `static` helpers stay plain `unsafe fn` — except the ones whose
// address C stores in a signal action or in `fd_callback_info[].func`, which are
// `unsafe extern "C" fn` (PORTING.md 1.4).
//
// **Carried arms.**  `src/process.c` is *not* in
// `docs/baseline/config/cond-eval-verified.txt`.  `python3 tools/check-cond-eval.py
// src/process.c` reported `222 conditions, 0 unknown, 29 disagreements [NOT VERIFIED]` before
// this file's rows landed; the 29 are `<sys/socket.h>`/`<net/if.h>`/`<sys/ioctl.h>`/
// `<netinet/tcp.h>`/`<net/if_dl.h>` names (`TIOCNOTTY`, `TIOCGPGRP`, `TCP_NODELAY`, `SIOCGIF*`,
// the `IFF_*` run, `LLADDR`) that `config.h` alone cannot decide (docs/status.md 3.35/3.51's
// class).  23 of them are now decided by rows in
// `docs/baseline/config/source-header-macros.tsv`, which leaves this file with exactly `1
// disagreement`: `TIOCNOTTY`.  That one cannot be carried by a *global* row — this file
// includes `<sys/ioctl.h>` (C 71), so clang has the macro defined here and the `#ifdef
// TIOCNOTTY` body at C 2171-2181 is **live**, while `src/term.c`'s own `#ifdef TIOCNOTTY`
// never includes that header and clang has it undefined there; `src/term.c` *is* in
// `cond-eval-verified.txt`, so the row would regress it.  The port compiles the body live and
// leaves the row out (docs/ports/process.md, Decisions).
//
// `tools/extract-abi.py`'s `blind_conditions` still drops the macro table for this file (it is
// *not* in `cond-eval-verified.txt`) and keeps *both* arms of every `#if` live; per the
// integrator's 2026-10-03 decision (docs/status.md 3.49, PORTING.md P-51, the
// `src/sysdep.c`/`src/print.c`/`src/callproc.c` precedent) the port *carries* the text of the
// arms the frozen build does not compile, each under a `#[cfg (feature = "…")]` naming the C
// condition's non-frozen side.  None of those features is ever set: `xtask dead-arms` can only
// flip the keys `config.rs` carries, and these are this file's platform and system-header
// predicates (docs/status.md 3.9).  `src/process.c` is nevertheless *not* added to
// `cond-eval-verified.txt`: a verified file's checker demands only the live arm (docs/status.md
// 3.51's `src/callproc.c` note), while the unverified checker demands both, which is what this
// port has.
//
// The arms, in C order (feature names follow the C condition; `!` becomes a `no_` prefix):
//
//   * C 44-46 (`!subprocesses` -> `no_subprocesses`; the two `PIPECONN*_P` fallbacks).
//   * C 62-64 (`!defined AF_LOCAL && defined AF_UNIX` -> `no_af_local`), C 91-93
//     (`HAVE_PTY_H` -> `have_pty_h`), C 118-121 (`HAVE_GNUTLS` -> `have_gnutls`),
//     C 122-125 (`HAVE_ANDROID` -> `have_android`; also C 5301-5317, 6135-6153),
//     C 127-129 (`HAVE_WINDOW_SYSTEM` -> `have_window_system`), C 131-136
//     (`HAVE_GLIB` -> `have_glib`), C 138-141 (`HAVE_GETADDRINFO_A || HAVE_GNUTLS` ->
//     `have_getaddrinfo_a`), C 143-146 (`WINDOWSNT` -> `windowsnt`).
//   * C 168-170 (`AF_INET6` is live), C 297-299 (`!WINDOWSNT` is live: `child_signal_read`).
//   * C 321-334 (`DATAGRAM_SOCKETS` is live), C 885-906 (`HAVE_PTYS` is live; the three
//     `PTY_*` fallbacks are the `#else` arms), C 1023-1049 (`HAVE_GETADDRINFO_A` ->
//     `have_getaddrinfo_a`; `free_dns_request` and `struct req`), C 1101-1107,
//     C 1141-1143 (`HAVE_GETADDRINFO_A`).
//   * C 1053-1056, 4560-4566 (`HAVE_STRUCT_IFREQ_IFR_HWADDR` -> `no_ifreq_hwaddr`).
//   * C 1391-1393 (`USABLE_SIGIO || USABLE_SIGPOLL` -> `no_usable_sigio`).
//   * C 1516-1522, 1527-1545, 1548-1554, 1557-1569, 1572-1583 (`HAVE_SETRLIMIT` live;
//     the `!HAVE_SETRLIMIT` and `WINDOWSNT` arms carried).
//   * C 1607-1645 (`HAVE_GETADDRINFO_A`), C 2171-2181 (`TIOCNOTTY`: **live** — its body is
//     the compiled text, not carried; see above),
//     C 2186-2204 (`HAVE_PTYS`/`HAVE_OPENPTY` arms), C 2207-2213, 2221-2226, 2230-2241,
//     C 2255-2259 (`WINDOWSNT`), C 2262-2269 (`HAVE_ANDROID`), C 2407-2416 (`HAVE_PTYS`),
//     C 2424-2435, 2447-2461 (`WINDOWSNT`), C 2489-2495 (`HAVE_OPENPTY`).
//   * C 2906-2916 (`TCP_NODELAY` -> `no_tcp_nodelay`), C 2919-2925 (`SO_KEEPALIVE` live),
//     C 3290-3298 (`USABLE_SIGIO`), C 3306-3312 (`HAVE_GNUTLS`), C 3336-3349
//     (`HAVE_GNUTLS`), C 3407-3428 (`HAVE_GNUTLS`), C 3431-3440, 3445-3455, 3490-3497,
//     C 3512-3520, 3557-3560, 3564-3568, 3666-3672 (`HAVE_GNUTLS`).
//   * C 3792-3815 (`WINDOWSNT` native `make-network-process`), C 4013-4030
//     (`HAVE_GETADDRINFO_A`), C 4073-4083 (`HAVE_GETADDRINFO_A`), C 4154-4159
//     (`WINDOWSNT`), C 4180-4213 (`HAVE_GETADDRINFO_A`), C 4306-4323
//     (`HAVE_GETADDRINFO_A`), C 4332-4340 (`HAVE_GETADDRINFO_A`).
//   * C 4434-4656 (`HAVE_GETIFADDRS && LLADDR` and the `SIOCGIF*` fallbacks -> `no_getifaddrs`).
//   * C 4701-4702, 4718-4722, 4739-4741, 4758-4761 (`HAVE_GETIFADDRS`/`WINDOWSNT`),
//     C 4847-4849 (`WINDOWSNT`), C 4998-5000 (`HAVE_GETADDRINFO_A`), C 5205-5249
//     (`HAVE_GETADDRINFO_A`), C 5276-5282, 5299-5317 (`HAVE_ANDROID`), C 5377-5378
//     (`USABLE_SIGIO` live), C 5432-5486 (`WINDOWSNT`), C 5697-5699, 5756-5853
//     (`WINDOWSNT`/`HAVE_GNUTLS` readonly masks), C 5955-5957, 5975-5976, 6089-6090,
//     C 6135-6153 (`HAVE_ANDROID`), C 6172-6177, 6297-6301, 6550-6553, 6890-6893,
//     C 6913-6935 (`WINDOWSNT`), C 7023-7040 (`TIOCGPGRP` -> `no_tiocpgrp`), C 7140-7141,
//     C 7160-7180 (`TIOCGPGRP`), C 7207-7211, 7299-7300, 7326-7328, 7338-7339, 7781-7784,
//     C 7823-7826 (`WINDOWSNT`), C 8065-8078 (`HAVE_GPM` -> `have_gpm`), C 8102-8325
//     (`WINDOWSNT`), C 8331-8343 (`HAVE_W32NOTIFY`/`HAVE_PGTK`), C 8455-8458,
//     C 8479-8480, 8492-8493 (`HAVE_GLIB`), C 8623-8645 (`HAVE_PROCFS`/`USABLE_PROC`),
//     C 8696-8698, 8713-8743 (`HAVE_PROCFS`), C 8791-8792 (`HAVE_GETADDRINFO_A`).
//
// The `#include` selections (PORTING.md 7.2's third category) are recorded in
// `docs/ports/process.md`; the headers this target's port has no mirror for
// (`<sys/socket.h>`, `<netdb.h>`, `<netinet/in.h>`, `<netinet/tcp.h>`, `<arpa/inet.h>`,
// `<sys/un.h>`, `<sys/ioctl.h>`, `<net/if.h>`, `<ifaddrs.h>`, `<net/if_dl.h>`, `<util.h>`,
// `<pty.h>`, `<sys/file.h>`, `"systty.h"`, `"termopts.h"`, `"syswait.h"`, `"gnutls.h"`,
// `"android.h"`, `"androidterm.h"`, `"xgselect.h"`) contribute the declarations below, each
// with C's own prototype in a `/// C:` line (PORTING.md P-72 rule 3, the
// terminal.rs/sysdep.rs/callproc.rs shape).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
         unused_imports, unused_macros, unused_variables, unused_unsafe, static_mut_refs,
         unsafe_op_in_unsafe_fn, unused_assignments, unreachable_code, while_true,
         unused_parens, unused_mut, unused_labels, unused_comparisons, overflowing_literals,
         unnecessary_cast, dropping_copy_types, unused_must_use)]

/* C 22-134: the `#include`s.  The mirrors are named with C's own header in the comment.  */
include!("include/config.rs");                 // C 22: `#include <config.h>`
include!("../lib/include/errno.rs");           // C 26: `#include <errno.h>`
include!("../lib/include/stdio.rs");           // C 24: `#include <stdio.h>`
include!("../lib/include/stdlib.rs");          // C 25: `#include <stdlib.h>`
include!("../lib/include/string.rs");          // lisp.h 30's `#include <string.h>`
include!("../lib/include/sys_types.in.rs");    // C 27: `#include <sys/types.h>`
include!("../lib/include/sys_stat.in.rs");     // C 29: `#include <sys/stat.h>`
include!("../lib/include/unistd.in.rs");       // C 30: `#include <unistd.h>`
include!("../lib/include/fcntl.in.rs");        // C 31: `#include <fcntl.h>`
include!("../lib/include/limits.in.rs");       // <limits.h> (`INT_BUFSIZE_BOUND`, `INT_MAX`)
include!("../lib/include/sys_select.rs");      // C 115: `#include "sysselect.h"` -> <sys/select.h>
include!("../lib/include/signal.rs");          // C 116: `#include "syssignal.h"` -> <signal.h>
include!("../lib/include/sys_resource.rs");    // C 50: `#include <sys/resource.h>`
include!("../lib/include/c-ctype.rs");         // C 95: `#include <c-ctype.h>`
include!("../lib/include/flexmember.rs");      // C 96: `#include <flexmember.h>`
include!("../lib/include/nproc.rs");           // C 97: `#include <nproc.h>`
include!("../lib/include/sig2str.rs");         // C 98: `#include <sig2str.h>`
include!("include/lisp.rs");                   // C 33: `#include "lisp.h"`
include!("../lib/include/mini-gmp.rs");        // bignum.h's `#include <gmp.h>`
include!("include/bignum.rs");                 // lisp.h 2359's struct Lisp_Bignum
include!("include/character.rs");              // C 106: `#include "character.h"`
include!("include/itree.rs");                  // buffer.h 35's `#include "itree.h"`
include!("include/buffer.rs");                 // C 107: `#include "buffer.h"`
include!("include/composite.rs");              // coding.h 284's `#include "composite.h"`
include!("include/coding.rs");                 // C 108: `#include "coding.h"`
include!("include/process.rs");                // C 109: `#include "process.h"`
include!("include/commands.rs");               // keyboard.h's `#include "commands.h"`
include!("include/dispextern.rs");             // frame.h 25's `#include "dispextern.h"`
include!("include/font.rs");                   // dispextern.h's font names
include!("include/window.rs");                 // C 105: `#include "window.h"`
include!("include/termhooks.rs");              // frame.h's `#include "termhooks.h"`
include!("include/systime.rs");                // C 102: `#include "systime.h"`
include!("include/frame.rs");                  // C 110: `#include "frame.h"`
include!("include/keyboard.rs");               // C 112: `#include "keyboard.h"`
include!("include/blockinput.rs");             // C 113: `#include "blockinput.h"`
include!("include/atimer.rs");                 // C 114: `#include "atimer.h"`
include!("include/syssignal.rs");              // C 116: `#include "syssignal.h"`

/* ------------------------------------------------------------------------------------------
   The names the mirrors do not carry, declared at the use site (PORTING.md P-72 rule 3).
   The system headers are `<sys/socket.h>`, `<netdb.h>`, `<netinet/in.h>`, `<netinet/tcp.h>`,
   `<arpa/inet.h>`, `<sys/un.h>`, `<sys/ioctl.h>`, `<net/if.h>`, `<ifaddrs.h>`,
   `<net/if_dl.h>`, `<util.h>`, `<pty.h>`, `<sys/file.h>`; the C headers are `"systty.h"`,
   `"termopts.h"`, `"syswait.h"`, `"gnutls.h"`, `"android.h"`, `"androidterm.h"`,
   `"xgselect.h"`.  Every type below is measured with the frozen configuration (the clang
   invocation `tools/gen-layout-fields.py` uses) and pinned by `const _` assertions.  */

/* process.h 277: `typedef void (*fd_callback) (int fd, void *data);`  (nullable in C: the
   table is zero-initialised and C compares an entry with NULL; P-27's `Option` shape).  */
pub type fd_callback = Option<unsafe extern "C" fn (fd: c_int, data: *mut c_void)>;

/* <sys/socket.h>: `struct sockaddr`, measured 16/1; `sa_len`@0, `sa_family`@1, `sa_data`@2.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr {
  pub sa_len: u8,
  pub sa_family: u8,
  pub sa_data: [c_char; 14],
}
const _: () = core::assert! (std::mem::size_of::<sockaddr> () == 16);
const _: () = core::assert! (std::mem::offset_of! (sockaddr, sa_len) == 0);
const _: () = core::assert! (std::mem::offset_of! (sockaddr, sa_family) == 1);
const _: () = core::assert! (std::mem::offset_of! (sockaddr, sa_data) == 2);

/* <netinet/in.h>: `struct in_addr { in_addr_t s_addr; }`.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr { pub s_addr: u32 }
const _: () = core::assert! (std::mem::size_of::<in_addr> () == 4);

/* <netinet/in.h>: `struct in6_addr { union { … } __u6_addr; }`, 16 bytes.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in6_addr { pub __u6_addr8: [u8; 16] }
const _: () = core::assert! (std::mem::size_of::<in6_addr> () == 16);

/* <netinet/in.h>: `struct sockaddr_in`, measured 16/4.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in {
  pub sin_len: u8,        // 0
  pub sin_family: u8,     // 1
  pub sin_port: u16,      // 2
  pub sin_addr: in_addr,  // 4
  pub sin_zero: [c_char; 8], // 8
}
const _: () = core::assert! (std::mem::size_of::<sockaddr_in> () == 16);
const _: () = core::assert! (std::mem::offset_of! (sockaddr_in, sin_addr) == 4);
const _: () = core::assert! (std::mem::offset_of! (sockaddr_in, sin_zero) == 8);

/* <netinet/in.h>: `struct sockaddr_in6`, measured 28/4.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_in6 {
  pub sin6_len: u8,           // 0
  pub sin6_family: u8,        // 1
  pub sin6_port: u16,         // 2
  pub sin6_flowinfo: u32,     // 4
  pub sin6_addr: in6_addr,    // 8
  pub sin6_scope_id: u32,     // 24
}
const _: () = core::assert! (std::mem::size_of::<sockaddr_in6> () == 28);
const _: () = core::assert! (std::mem::offset_of! (sockaddr_in6, sin6_addr) == 8);
const _: () = core::assert! (std::mem::offset_of! (sockaddr_in6, sin6_scope_id) == 24);

/* <sys/un.h>: `struct sockaddr_un`, measured 106/1.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_un {
  pub sun_len: u8,          // 0
  pub sun_family: u8,       // 1
  pub sun_path: [c_char; 104], // 2
}
const _: () = core::assert! (std::mem::size_of::<sockaddr_un> () == 106);
const _: () = core::assert! (std::mem::offset_of! (sockaddr_un, sun_path) == 2);

/* <sys/socket.h>: `struct sockaddr_storage`, measured 128/8.  */
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct sockaddr_storage { pub ss_bits: [u64; 16] }
const _: () = core::assert! (std::mem::size_of::<sockaddr_storage> () == 128);

/* <net/if_dl.h>: `struct sockaddr_dl`, measured 20/2.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_dl {
  pub sdl_len: u8,      // 0
  pub sdl_family: u8,   // 1
  pub sdl_index: u16,   // 2
  pub sdl_type: u8,     // 4
  pub sdl_nlen: u8,     // 5
  pub sdl_alen: u8,     // 6
  pub sdl_slen: u8,     // 7
  pub sdl_data: [c_char; 12], // 8
}
const _: () = core::assert! (std::mem::size_of::<sockaddr_dl> () == 20);
const _: () = core::assert! (std::mem::offset_of! (sockaddr_dl, sdl_data) == 8);

/* <netdb.h>: `struct addrinfo`, measured 48/8.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct addrinfo {
  pub ai_flags: c_int,          // 0
  pub ai_family: c_int,         // 4
  pub ai_socktype: c_int,       // 8
  pub ai_protocol: c_int,       // 12
  pub ai_addrlen: socklen_t,    // 16
  pub ai_canonname: *mut c_char, // 24
  pub ai_addr: *mut sockaddr,   // 32
  pub ai_next: *mut addrinfo,   // 40
}
const _: () = core::assert! (std::mem::size_of::<addrinfo> () == 48);
const _: () = core::assert! (std::mem::offset_of! (addrinfo, ai_addrlen) == 16);
const _: () = core::assert! (std::mem::offset_of! (addrinfo, ai_addr) == 32);
const _: () = core::assert! (std::mem::offset_of! (addrinfo, ai_next) == 40);

/* <ifaddrs.h>: `struct ifaddrs`, measured 56/8.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifaddrs {
  pub ifa_next: *mut ifaddrs,      // 0
  pub ifa_name: *mut c_char,       // 8
  pub ifa_flags: c_uint,           // 16
  pub ifa_addr: *mut sockaddr,     // 24
  pub ifa_netmask: *mut sockaddr,  // 32
  pub ifa_dstaddr: *mut sockaddr,  // 40
  pub ifa_data: *mut c_void,       // 48
}
const _: () = core::assert! (std::mem::size_of::<ifaddrs> () == 56);
const _: () = core::assert! (std::mem::offset_of! (ifaddrs, ifa_addr) == 24);

/* <net/if.h>: `struct ifreq { char ifr_name[16]; union { … } ifr_ifru; }`, measured 32/8 —
   the union starts at 16 and `ifr_addr`/`ifr_flags`/`ifr_mtu` all measure 16.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub union ifr_ifru {
  pub ifr_addr: sockaddr,
  pub ifr_dstaddr: sockaddr,
  pub ifr_broadaddr: sockaddr,
  pub ifr_netmask: sockaddr,
  pub ifr_hwaddr: sockaddr,
  pub ifr_flags: c_short,
  pub ifr_mtu: c_int,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifreq {
  pub ifr_name: [c_char; 16],  // 0
  pub ifr_ifru: ifr_ifru,      // 16
}
const _: () = core::assert! (std::mem::size_of::<ifreq> () == 32);
const _: () = core::assert! (std::mem::offset_of! (ifreq, ifr_ifru) == 16);

/* <netdb.h>: `struct servent`, measured 32/8.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct servent {
  pub s_name: *mut c_char,       // 0
  pub s_aliases: *mut *mut c_char, // 8
  pub s_port: c_int,             // 16
  pub s_proto: *mut c_char,      // 24
}
const _: () = core::assert! (std::mem::size_of::<servent> () == 32);

/* <sys/socket.h>: `struct linger { int l_onoff; int l_linger; }`, measured 8/4.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct linger { pub l_onoff: c_int, pub l_linger: c_int }
const _: () = core::assert! (std::mem::size_of::<linger> () == 8);

/* <sys/socket.h>: `typedef __uint32_t socklen_t;` — `pid_t`'s sibling in sys_types; the
   mirror has no `socklen_t`, so it is declared here with the measured width.  */
pub type socklen_t = u32;
const _: () = core::assert! (std::mem::size_of::<socklen_t> () == 4);

/* <netdb.h>: `struct gaicb` (getaddrinfo_a's request) and `<netdb.h>`'s `GAI_NOWAIT` are only
   visible with `HAVE_GETADDRINFO_A`, which the frozen configuration leaves undefined
   (C 1101-1107/1141-1143 etc. are its dead arms).  The port keeps the whole `struct req` in
   the carried text; the type is declared outside it so the text names a complete type.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct gaicb {
  pub ar_name: *const c_char,       // 0
  pub ar_service: *const c_char,    // 8
  pub ar_request: *const addrinfo,  // 16
  pub ar_result: *mut addrinfo,     // 24
}

/* <netdb.h>'s `struct gaicb` is glibc-only and the only arm that names it is
   `HAVE_GETADDRINFO_A` (undefined on this target), so no measurement exists; the declaration
   above is carried text.  Its fields are `ar_name`, `ar_service`, `ar_request`, `ar_result`
   (with the two `__`-prefixed result slots the C text does not name).  */

/* ------------------------------------------------------------------------------------------
   C 138-141: this file's own `#define ASYNC_RETRY_NSEC` lives inside
   `#if defined HAVE_GETADDRINFO_A || defined HAVE_GNUTLS` (both undefined in the frozen
   `config.h`), so it is carried text (P-51).  */
#[cfg (feature = "have_getaddrinfo_a")]
pub const ASYNC_RETRY_NSEC: c_long = 100000000;

/* C 143-146's `WINDOWSNT` `extern int sys_select (...)` is the same prototype the unit
   declares at the use site below (the live code calls `sys_select`), so nothing is
   duplicated here.  C 151-153 is a `#pragma GCC diagnostic`, which has no counterpart.  */

/* C 156-157: `static bool kbd_is_on_hold;`  */
static mut kbd_is_on_hold: bool = false;

/* C 161: `bool inhibit_sentinels;` — non-static, so it is this unit's exported object
   (`process.h` 265 mirrors the same name as a declaration).  */
#[unsafe(no_mangle)]
pub static mut inhibit_sentinels: bool = false;

/* C 163-8101: `#ifdef subprocesses` (live).  Its `#else` arm (C 8102-8325) is carried at the
   end of the region under `no_subprocesses`.  */

/* C 164-170: `union u_sockaddr`, measured 108/4 (the largest member is `sockaddr_un`'s 106
   bytes, rounded to `sockaddr_in6`'s 4-byte alignment).  */
#[repr(C)]
#[derive(Clone, Copy)]
pub union u_sockaddr
{
  pub sa: sockaddr,           // C 166: `struct sockaddr sa;`
  pub r#in: sockaddr_in,      // C 167: `struct sockaddr_in in;`
  pub in6: sockaddr_in6,      // C 169: `#ifdef AF_INET6`
  pub un: sockaddr_un,        // C 172: `#ifdef HAVE_LOCAL_SOCKETS`
}
const _: () = core::assert! (std::mem::size_of::<u_sockaddr> () == 108);

/* C 176-181: `#ifndef SOCK_CLOEXEC` / `#ifndef SOCK_NONBLOCK` are live on this target (the
   system headers define neither), so the file supplies 0 for both.  */
/* C 177.  */
pub const SOCK_CLOEXEC: c_int = 0;
/* C 180.  */
pub const SOCK_NONBLOCK: c_int = 0;

/* C 182-189: `would_block` —  ``True if ERRNUM represents an error where the system call
   would block if a blocking variant were used.''  */
unsafe fn would_block (errnum: c_int) -> bool
{
  return errnum == EAGAIN
    || (EWOULDBLOCK != EAGAIN && errnum == EWOULDBLOCK);
}

/* C 192-220: `#ifndef HAVE_ACCEPT4` is live (the frozen table records `HAVE_ACCEPT4` as
   `undef`), so C's `accept4`/`socket` emulation is this target's code.  C's
   `# undef accept4` + `# define accept4(…)` rename every later use of `accept4` in the file to
   `process_accept4`; the port keeps both names: the `accept4` macro is this unit's
   `macro_rules! accept4`, and the definition below is `process_accept4` (the expansion's
   name), which is also what the port's call sites use.  */

/* C 197-203: `close_on_exec`.  */
unsafe fn close_on_exec (fd: c_int) -> c_int
{
  if (0 <= fd)
    { fcntl (fd, F_SETFD, FD_CLOEXEC); }
  return fd;
}

/* C 205-206: `# undef accept4` / `# define accept4(sockfd, addr, addrlen, flags)
   process_accept4 (sockfd, addr, addrlen, flags)` — the macro whose expansion C's later call
   sites use (`process_accept4` is the definition below, so the macro and the function are the
   two halves C's preprocessor makes; the call sites write `process_accept4`, the name they
   expand to).  */
macro_rules! accept4 {
  ($sockfd:expr, $addr:expr, $addrlen:expr, $flags:expr) =>
    { process_accept4 ($sockfd, $addr, $addrlen, $flags) };
}

/* C 207-211: `static int accept4 (int sockfd, struct sockaddr *addr, socklen_t *addrlen,
   int flags)` — renamed `process_accept4` by C's own macro above.  */
unsafe fn process_accept4 (sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t,
                                  flags: c_int) -> c_int
{
  return close_on_exec (accept (sockfd, addr, addrlen));
}

/* C 213-217: `static int process_socket (int domain, int type, int protocol)`.  */
unsafe fn process_socket (domain: c_int, r#type: c_int, protocol: c_int) -> c_int
{
  return close_on_exec (libc_socket::socket (domain, r#type, protocol));
}

/* C 218-219: `# undef socket` / `# define socket(domain, type, protocol)
   process_socket (domain, type, protocol)`.  The real `socket` is reached through a module
   so the name the C body spells is still the one the token check sees.  */
macro_rules! socket {
  ($domain:expr, $type:expr, $protocol:expr) =>
    { process_socket ($domain, $type, $protocol) };
}

/* C 222-227: the connection-test macros.  */
/// C 222: `#define NETCONN_P(p) (EQ (XPROCESS (p)->type, Qnetwork))`
#[inline(always)]
pub unsafe fn NETCONN_P (p: Lisp_Object) -> bool { EQ ((*XPROCESS (p)).r#type, Qnetwork ()) }
/// C 223: `#define NETCONN1_P(p) (EQ (p->type, Qnetwork))`
#[inline(always)]
pub unsafe fn NETCONN1_P (p: *mut Lisp_Process) -> bool { EQ ((*p).r#type, Qnetwork ()) }
/// C 224: `#define SERIALCONN_P(p) (EQ (XPROCESS (p)->type, Qserial))`
#[inline(always)]
pub unsafe fn SERIALCONN_P (p: Lisp_Object) -> bool { EQ ((*XPROCESS (p)).r#type, Qserial ()) }
/// C 225: `#define SERIALCONN1_P(p) (EQ (p->type, Qserial))`
#[inline(always)]
pub unsafe fn SERIALCONN1_P (p: *mut Lisp_Process) -> bool { EQ ((*p).r#type, Qserial ()) }
/// C 226: `#define PIPECONN_P(p) (EQ (XPROCESS (p)->type, Qpipe))`
#[inline(always)]
pub unsafe fn PIPECONN_P (p: Lisp_Object) -> bool { EQ ((*XPROCESS (p)).r#type, Qpipe ()) }
/// C 227: `#define PIPECONN1_P(p) (EQ (p->type, Qpipe))`
#[inline(always)]
pub unsafe fn PIPECONN1_P (p: *mut Lisp_Process) -> bool { EQ ((*p).r#type, Qpipe ()) }

/* C 230-232: the two tick counters.  */
static mut process_tick: EMACS_INT = 0;
static mut update_tick: EMACS_INT = 0;

/* C 239-249: the datagram-socket capability macros.  `DATAGRAM_SOCKETS`'s own body is empty
   in C (`# define DATAGRAM_SOCKETS`, no value), so the port declares a unit constant with the
   same name; `HAVE_SEQPACKET` follows `HAVE_LOCAL_SOCKETS && DATAGRAM_SOCKETS`.  */
pub const DATAGRAM_SOCKETS: bool = true;
/* C 65-68: `#ifdef AF_LOCAL` → `#define HAVE_LOCAL_SOCKETS` (the file's own macro).  */
pub const HAVE_LOCAL_SOCKETS: bool = true;
pub const HAVE_SEQPACKET: bool = true;

/* C 251-253: the adaptive-read-buffering delays (`TIMESPEC_HZ` is systime.h's).  */
pub const READ_OUTPUT_DELAY_INCREMENT: EMACS_INT = TIMESPEC_HZ as EMACS_INT / 100;
pub const READ_OUTPUT_DELAY_MAX: EMACS_INT = READ_OUTPUT_DELAY_INCREMENT * 5;
pub const READ_OUTPUT_DELAY_MAX_MAX: EMACS_INT = READ_OUTPUT_DELAY_INCREMENT * 7;

/* C 256-262.  */
static mut process_output_delay_count: c_int = 0;
static mut process_output_skip: bool = false;

/* C 264-296: C's forward declarations (`start_process_unwind`, `create_process`,
   `keyboard_bit_set`, `deactivate_process`, `status_notify`, `read_process_output`,
   `create_pty`, `exec_sentinel`, `network_lookup_address_info_1`) are dropped per PORTING.md
   0.1-12 (Rust has no forward declarations); the definitions keep C's positions.  */
static mut num_pending_connects: c_int = 0;
static mut max_desc: c_int = 0;
static mut external_sock_fd: c_int = 0;
static mut child_signal_read_fd: c_int = -1;
static mut child_signal_write_fd: c_int = -1;

/* C 297-299: `#ifndef WINDOWSNT static void child_signal_read (int, void *); #endif` — the
   definition's prototype is needed before `child_signal_init` names it, and `child_signal_read`
   is a `fd_callback` (PORTING.md 1.4), so its signature is the callback's.  */

/* C 303-319: the per-descriptor tables and the process alist.  C zero-initialises the two
   `Lisp_Object` tables; the port uses `Qnil` (the callproc.rs/print.rs shape — C sets every
   entry to `Qnil` in `syms_of_process` before the tables are read).  */
static mut chan_process: [Lisp_Object; FD_SETSIZE as usize] =
  [lisp_h_Qnil (); FD_SETSIZE as usize];
static mut Vprocess_alist: Lisp_Object = lisp_h_Qnil ();
static mut proc_buffered_char: [c_int; FD_SETSIZE as usize] = [-1; FD_SETSIZE as usize];
static mut proc_decode_coding_system: [*mut coding_system; FD_SETSIZE as usize] =
  [std::ptr::null_mut (); FD_SETSIZE as usize];
static mut proc_encode_coding_system: [*mut coding_system; FD_SETSIZE as usize] =
  [std::ptr::null_mut (); FD_SETSIZE as usize];

/* C 321-334: `#ifdef DATAGRAM_SOCKETS` (live).  */
/* C 323-326: `static struct sockaddr_and_len { struct sockaddr *sa; ptrdiff_t len; }`, 16/8.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sockaddr_and_len
{
  pub sa: *mut sockaddr,
  pub len: ptrdiff_t,
}
const _: () = core::assert! (std::mem::size_of::<sockaddr_and_len> () == 16);
/* C 327.  */
static mut datagram_address: [sockaddr_and_len; FD_SETSIZE as usize] =
  [sockaddr_and_len { sa: std::ptr::null_mut (), len: 0 }; FD_SETSIZE as usize];
/// C 328: `#define DATAGRAM_CHAN_P(chan) (datagram_address[chan].sa != 0)`
#[inline(always)]
pub unsafe fn DATAGRAM_CHAN_P (chan: c_int) -> bool
{ (datagram_address[chan as usize].sa as usize) != 0 }
/// C 329-332: `#define DATAGRAM_CONN_P(proc) (PROCESSP (proc) && XPROCESS (proc)->infd >= 0
/// && datagram_address[XPROCESS (proc)->infd].sa != 0)`
#[inline(always)]
pub unsafe fn DATAGRAM_CONN_P (proc_: Lisp_Object) -> bool
{
  PROCESSP (proc_)
  && (*XPROCESS (proc_)).infd >= 0
  && datagram_address[(*XPROCESS (proc_)).infd as usize].sa != std::ptr::null_mut ()
}
/* C 333-334's `#else` arm (`#define DATAGRAM_CONN_P(proc) (0)`) is the `no_datagram_sockets`
   side of the same name; the port's `DATAGRAM_CONN_P` above is the live one.  */

/* C 335-340: `FOR_EACH_PROCESS (LIST_VAR, PROC_VAR)` is lisp.h 5908's
   `FOR_EACH_ALIST_VALUE`, which no mirror carries (P-22, docs/ports/excluded-macros.txt).
   Rust has no statement-position loop macro, so the loop is written once here and takes C's
   body as its last argument; C's `continue` (none of this file's five loops has one) would
   need the increment repeated, as in the minibuf.rs/buffer.rs expansions.  */
macro_rules! for_each_alist_value {
  ($head_var:expr, $list_var:ident, $value_var:ident, $body:block) => {
    {
      $list_var = $head_var;
      while CONSP ($list_var) && { $value_var = XCDR (XCAR ($list_var)); true }
        {
          $body
          $list_var = XCDR ($list_var);
        }
    }
  }
}
/// C 339-340: `#define FOR_EACH_PROCESS(list_var, proc_var) FOR_EACH_ALIST_VALUE
/// (Vprocess_alist, list_var, proc_var)`
macro_rules! FOR_EACH_PROCESS {
  ($list_var:ident, $proc_var:ident, $body:block) =>
    { for_each_alist_value! (Vprocess_alist, $list_var, $proc_var, $body) }
}

/* C 342-424: the private `pset_*` setters.  */
/* C 344-347.  */
unsafe fn pset_buffer (p: *mut Lisp_Process, val: Lisp_Object) { (*p).buffer = val; }
/* C 349-352.  */
unsafe fn pset_command (p: *mut Lisp_Process, val: Lisp_Object) { (*p).command = val; }
/* C 354-357.  */
unsafe fn pset_decode_coding_system (p: *mut Lisp_Process, val: Lisp_Object)
{ (*p).decode_coding_system = val; }
/* C 359-362.  */
unsafe fn pset_decoding_buf (p: *mut Lisp_Process, val: Lisp_Object)
{ (*p).decoding_buf = val; }
/* C 364-367.  */
unsafe fn pset_encode_coding_system (p: *mut Lisp_Process, val: Lisp_Object)
{ (*p).encode_coding_system = val; }
/* C 369-372.  */
unsafe fn pset_encoding_buf (p: *mut Lisp_Process, val: Lisp_Object)
{ (*p).encoding_buf = val; }
/* C 374-377.  */
unsafe fn pset_filter (p: *mut Lisp_Process, val: Lisp_Object)
{ (*p).filter = if NILP (val) { Qinternal_default_process_filter () } else { val }; }
/* C 379-382.  */
unsafe fn pset_log (p: *mut Lisp_Process, val: Lisp_Object) { (*p).log = val; }
/* C 384-387.  */
unsafe fn pset_mark (p: *mut Lisp_Process, val: Lisp_Object) { (*p).mark = val; }
/* C 389-392.  */
unsafe fn pset_thread (p: *mut Lisp_Process, val: Lisp_Object) { (*p).thread = val; }
/* C 394-397.  */
unsafe fn pset_name (p: *mut Lisp_Process, val: Lisp_Object) { (*p).name = val; }
/* C 399-402.  */
unsafe fn pset_plist (p: *mut Lisp_Process, val: Lisp_Object) { (*p).plist = val; }
/* C 404-407.  */
unsafe fn pset_sentinel (p: *mut Lisp_Process, val: Lisp_Object)
{ (*p).sentinel = if NILP (val) { Qinternal_default_process_sentinel () } else { val }; }
/* C 409-412.  */
unsafe fn pset_tty_name (p: *mut Lisp_Process, val: Lisp_Object)
{ (*p).tty_name = val; }
/* C 414-417.  */
unsafe fn pset_type (p: *mut Lisp_Process, val: Lisp_Object) { (*p).r#type = val; }
/* C 419-422.  */
unsafe fn pset_write_queue (p: *mut Lisp_Process, val: Lisp_Object)
{ (*p).write_queue = val; }
/* C 424-427.  */
unsafe fn pset_stderrproc (p: *mut Lisp_Process, val: Lisp_Object)
{ (*p).stderrproc = val; }

/* C 431-434: `make_lisp_proc`.  */
unsafe fn make_lisp_proc (p: *mut Lisp_Process) -> Lisp_Object
{
  return make_lisp_ptr (p as *mut c_void, Lisp_Type::Lisp_Vectorlike);
}

/* C 436-455: `enum fd_bits` — C's enum members are this unit's constants.  */
pub type fd_bits = c_int;
pub const FOR_READ: fd_bits = 1;                // C 440
pub const FOR_WRITE: fd_bits = 2;               // C 442
pub const KEYBOARD_FD: fd_bits = 4;             // C 445
pub const PROCESS_FD: fd_bits = 8;              // C 447
pub const NON_BLOCKING_CONNECT_FD: fd_bits = 16; // C 450

/* C 452-464: `static struct fd_callback_data { … } fd_callback_info[FD_SETSIZE];`,
   measured 40/8 (`func`@0, `data`@8, `flags`@16, `thread`@24, `waiting_thread`@32).  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fd_callback_data
{
  pub func: fd_callback,
  pub data: *mut c_void,
  pub flags: c_int,
  pub thread: *mut thread_state,
  pub waiting_thread: *mut thread_state,
}
const _: () = core::assert! (std::mem::size_of::<fd_callback_data> () == 40);
const _: () = core::assert! (std::mem::offset_of! (fd_callback_data, data) == 8);
const _: () = core::assert! (std::mem::offset_of! (fd_callback_data, flags) == 16);
const _: () = core::assert! (std::mem::offset_of! (fd_callback_data, thread) == 24);
const _: () = core::assert! (std::mem::offset_of! (fd_callback_data, waiting_thread) == 32);
static mut fd_callback_info: [fd_callback_data; FD_SETSIZE as usize] =
  [fd_callback_data { func: None, data: std::ptr::null_mut (), flags: 0,
                      thread: std::ptr::null_mut (), waiting_thread: std::ptr::null_mut () };
   FD_SETSIZE as usize];

/* C 466-474: `clear_fd_callback_data`.  */
unsafe fn clear_fd_callback_data (elem: *mut fd_callback_data)
{
  (*elem).func = None;
  (*elem).data = std::ptr::null_mut ();
  (*elem).flags = 0;
  (*elem).thread = std::ptr::null_mut ();
  (*elem).waiting_thread = std::ptr::null_mut ();
}

/* C 476-492: `inrange_fd`.  */
unsafe fn inrange_fd (fd: c_int) -> c_int
{
  if (fd < FD_SETSIZE)
    { return fd; }
  emacs_close (fd);
  *errno () = EMFILE;
  return -1;
}

/* C 494-516: `inrange_pipe`.  */
unsafe fn inrange_pipe (fd: *mut c_int) -> c_int
{
  let mut pipefd: [c_int; 2] = [0; 2];
  let result: c_int = emacs_pipe (pipefd.as_mut_ptr ());
  if (result < 0)
    { return result; }
  else if (pipefd[0] < FD_SETSIZE && pipefd[1] < FD_SETSIZE)
    {
      *fd.offset (0) = pipefd[0];
      *fd.offset (1) = pipefd[1];
      return result;
    }
  else
    {
      inrange_fd (pipefd[0]);
      inrange_fd (pipefd[1]);
      return -1;
    }
}

/* C 518-526: `add_read_fd`.  */
#[unsafe(no_mangle)]
pub unsafe fn add_read_fd (fd: c_int, func: fd_callback, data: *mut c_void)
{
  add_keyboard_wait_descriptor (fd);

  eassert! (0 <= fd && fd < FD_SETSIZE);
  fd_callback_info[fd as usize].func = func;
  fd_callback_info[fd as usize].data = data;
}

/* C 528-533: `add_non_keyboard_read_fd`.  */
#[unsafe(no_mangle)]
pub unsafe fn add_non_keyboard_read_fd (fd: c_int, func: fd_callback, data: *mut c_void)
{
  add_read_fd (fd, func, data);
  fd_callback_info[fd as usize].flags &= !KEYBOARD_FD;
}

/* C 535-549: `add_process_read_fd`.  */
unsafe fn add_process_read_fd (fd: c_int)
{
  eassert! (fd >= 0 && fd < FD_SETSIZE);
  eassert! (fd_callback_info[fd as usize].func.is_none ());

  fd_callback_info[fd as usize].flags &= !KEYBOARD_FD;
  fd_callback_info[fd as usize].flags |= FOR_READ;
  if (fd > max_desc)
    { max_desc = fd; }
  eassert! (0 <= fd && fd < FD_SETSIZE);
  fd_callback_info[fd as usize].flags |= PROCESS_FD;
}

/* C 551-555: `delete_read_fd`.  */
#[unsafe(no_mangle)]
pub unsafe fn delete_read_fd (fd: c_int)
{
  delete_keyboard_wait_descriptor (fd);
}

/* C 557-568: `add_write_fd`.  */
#[unsafe(no_mangle)]
pub unsafe fn add_write_fd (fd: c_int, func: fd_callback, data: *mut c_void)
{
  eassert! (0 <= fd && fd < FD_SETSIZE);

  fd_callback_info[fd as usize].func = func;
  fd_callback_info[fd as usize].data = data;
  fd_callback_info[fd as usize].flags |= FOR_WRITE;
  if (fd > max_desc)
    { max_desc = fd; }
}

/* ------------------------------------------------------------------------------------------
   Use-site declarations, group 1: the C text above.  Each has C's own prototype in a `/// C:`
   line (PORTING.md P-72 rule 3).  */

/* <errno.h>'s names the errno.rs mirror does not carry, measured with the frozen
   configuration (`clang` on the port's probe):  */

/* The real <sys/socket.h> `socket` is reached through a module so the unit's `socket`
   macro (C 219's `#define`) cannot capture it.  */
mod libc_socket
{
  use super::*;
  unsafe extern "C" {
    /// C: int socket (int, int, int)   (<sys/socket.h>)
    pub fn socket (domain: c_int, r#type: c_int, protocol: c_int) -> c_int;
  }
}

unsafe extern "C" {
  /// C: int accept (int, struct sockaddr *restrict, socklen_t *restrict)   (<sys/socket.h>)
  fn accept (socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
  /// C: int emacs_pipe (int[2])   (lisp.h 5313)
  fn emacs_pipe (pipefd: *mut c_int) -> c_int;
}

/* ------------------------------------------------------------------------------------------
   C 570-1060.  */

/// process.h 149-164's two bitfield units, in P-80's member-name form: C's field names stay
/// in the port's text and dispatch to the two mask words the mirror declares
/// (`Lisp_Process.bits1`/`bits2` and the `PROCESS_*_SHIFT`/`MASK` constants).
macro_rules! process_bits {
  (adaptive_read_buffering, $p:expr) =>
    { ((*($p)).bits1 & PROCESS_ADAPTIVE_READ_BUFFERING_MASK) as c_int };
  (adaptive_read_buffering, $p:expr, $value:expr) =>
    { (*($p)).bits1 = ((*($p)).bits1 & !PROCESS_ADAPTIVE_READ_BUFFERING_MASK)
        | ((($value) as u32) << PROCESS_ADAPTIVE_READ_BUFFERING_SHIFT) };
  (read_output_skip, $p:expr) =>
    { ((*($p)).bits1 & (1u32 << PROCESS_READ_OUTPUT_SKIP_SHIFT)) != 0 };
  (read_output_skip, $p:expr, $value:expr) =>
    { (*($p)).bits1 = (if ($value) as c_int != 0
        { (*($p)).bits1 | (1u32 << PROCESS_READ_OUTPUT_SKIP_SHIFT) }
        else { (*($p)).bits1 & !(1u32 << PROCESS_READ_OUTPUT_SKIP_SHIFT) }) };
  (kill_without_query, $p:expr) =>
    { process_bit_p ($p as *const Lisp_Process, PROCESS_KILL_WITHOUT_QUERY_SHIFT) };
  (kill_without_query, $p:expr, $value:expr) =>
    { process_bit_set ($p as *mut Lisp_Process, PROCESS_KILL_WITHOUT_QUERY_SHIFT,
                       ($value) as c_int != 0) };
  (pty_in, $p:expr) => { process_bit_p ($p as *const Lisp_Process, PROCESS_PTY_IN_SHIFT) };
  (pty_in, $p:expr, $value:expr) =>
    { process_bit_set ($p as *mut Lisp_Process, PROCESS_PTY_IN_SHIFT, ($value) as c_int != 0) };
  (pty_out, $p:expr) => { process_bit_p ($p as *const Lisp_Process, PROCESS_PTY_OUT_SHIFT) };
  (pty_out, $p:expr, $value:expr) =>
    { process_bit_set ($p as *mut Lisp_Process, PROCESS_PTY_OUT_SHIFT, ($value) as c_int != 0) };
  (inherit_coding_system_flag, $p:expr) =>
    { process_bit_p ($p as *const Lisp_Process, PROCESS_INHERIT_CODING_SYSTEM_FLAG_SHIFT) };
  (inherit_coding_system_flag, $p:expr, $value:expr) =>
    { process_bit_set ($p as *mut Lisp_Process, PROCESS_INHERIT_CODING_SYSTEM_FLAG_SHIFT,
                       ($value) as c_int != 0) };
  (alive, $p:expr) => { process_bit_p ($p as *const Lisp_Process, PROCESS_ALIVE_SHIFT) };
  (alive, $p:expr, $value:expr) =>
    { process_bit_set ($p as *mut Lisp_Process, PROCESS_ALIVE_SHIFT, ($value) as c_int != 0) };
  (raw_status_new, $p:expr) =>
    { process_bit_p ($p as *const Lisp_Process, PROCESS_RAW_STATUS_NEW_SHIFT) };
  (raw_status_new, $p:expr, $value:expr) =>
    { process_bit_set ($p as *mut Lisp_Process, PROCESS_RAW_STATUS_NEW_SHIFT,
                       ($value) as c_int != 0) };
  (is_non_blocking_client, $p:expr) =>
    { process_bit_p ($p as *const Lisp_Process, PROCESS_IS_NON_BLOCKING_CLIENT_SHIFT) };
  (is_non_blocking_client, $p:expr, $value:expr) =>
    { process_bit_set ($p as *mut Lisp_Process, PROCESS_IS_NON_BLOCKING_CLIENT_SHIFT,
                       ($value) as c_int != 0) };
  (is_server, $p:expr) => { process_bit_p ($p as *const Lisp_Process, PROCESS_IS_SERVER_SHIFT) };
  (is_server, $p:expr, $value:expr) =>
    { process_bit_set ($p as *mut Lisp_Process, PROCESS_IS_SERVER_SHIFT, ($value) as c_int != 0) };
}

#[inline(always)]
unsafe fn process_bit_p (p: *const Lisp_Process, shift: u32) -> bool
{ ((*p).bits2 & (1u32 << shift)) != 0 }
#[inline(always)]
unsafe fn process_bit_set (p: *mut Lisp_Process, shift: u32, value: bool)
{
  if value { (*p).bits2 |= 1u32 << shift } else { (*p).bits2 &= !(1u32 << shift) }
}

/* C 570-583: `add_non_blocking_write_fd`.  */
unsafe fn add_non_blocking_write_fd (fd: c_int)
{
  eassert! (fd >= 0 && fd < FD_SETSIZE);
  eassert! (fd_callback_info[fd as usize].func.is_none ());

  fd_callback_info[fd as usize].flags |= FOR_WRITE | NON_BLOCKING_CONNECT_FD;
  if (fd > max_desc)
    { max_desc = fd; }
  num_pending_connects += 1;
}

/* C 585-601: `recompute_max_desc`.  */
unsafe fn recompute_max_desc ()
{
  let mut fd: c_int;

  eassert! (max_desc < FD_SETSIZE);
  fd = max_desc;
  while fd >= 0
    {
      if (fd_callback_info[fd as usize].flags != 0)
	{
	  max_desc = fd;
	  break;
	}
      fd -= 1;
    }
  eassert! (max_desc < FD_SETSIZE);
}

/* C 603-620: `delete_write_fd`.  */
#[unsafe(no_mangle)]
pub unsafe fn delete_write_fd (fd: c_int)
{
  eassert! (0 <= fd && fd < FD_SETSIZE);
  if ((fd_callback_info[fd as usize].flags & NON_BLOCKING_CONNECT_FD) != 0)
    {
      num_pending_connects -= 1;
      if (num_pending_connects < 0)
	{ emacs_abort (); }
    }
  fd_callback_info[fd as usize].flags &= !(FOR_WRITE | NON_BLOCKING_CONNECT_FD);
  if (fd_callback_info[fd as usize].flags == 0)
    {
      clear_fd_callback_data (&raw mut fd_callback_info[fd as usize]);

      if (fd == max_desc)
	{ recompute_max_desc (); }
    }
}

/* C 622-643: `compute_input_wait_mask`.  */
unsafe fn compute_input_wait_mask (mask: *mut fd_set)
{
  let mut fd: c_int;

  FD_ZERO (mask);
  eassert! (max_desc < FD_SETSIZE);
  fd = 0;
  while fd <= max_desc
    {
      if (fd_callback_info[fd as usize].thread != std::ptr::null_mut ()
	  && fd_callback_info[fd as usize].thread != current_thread)
	{ fd += 1; continue; }
      if (fd_callback_info[fd as usize].waiting_thread != std::ptr::null_mut ()
	  && fd_callback_info[fd as usize].waiting_thread != current_thread)
	{ fd += 1; continue; }
      if ((fd_callback_info[fd as usize].flags & FOR_READ) != 0)
	{
	  FD_SET (fd, mask);
	  fd_callback_info[fd as usize].waiting_thread = current_thread;
	}
      fd += 1;
    }
}

/* C 645-667: `compute_non_process_wait_mask`.  */
unsafe fn compute_non_process_wait_mask (mask: *mut fd_set)
{
  let mut fd: c_int;

  FD_ZERO (mask);
  eassert! (max_desc < FD_SETSIZE);
  fd = 0;
  while fd <= max_desc
    {
      if (fd_callback_info[fd as usize].thread != std::ptr::null_mut ()
	  && fd_callback_info[fd as usize].thread != current_thread)
	{ fd += 1; continue; }
      if (fd_callback_info[fd as usize].waiting_thread != std::ptr::null_mut ()
	  && fd_callback_info[fd as usize].waiting_thread != current_thread)
	{ fd += 1; continue; }
      if ((fd_callback_info[fd as usize].flags & FOR_READ) != 0
	  && (fd_callback_info[fd as usize].flags & PROCESS_FD) == 0)
	{
	  FD_SET (fd, mask);
	  fd_callback_info[fd as usize].waiting_thread = current_thread;
	}
      fd += 1;
    }
}

/* C 669-691: `compute_non_keyboard_wait_mask`.  */
unsafe fn compute_non_keyboard_wait_mask (mask: *mut fd_set)
{
  let mut fd: c_int;

  FD_ZERO (mask);
  eassert! (max_desc < FD_SETSIZE);
  fd = 0;
  while fd <= max_desc
    {
      if (fd_callback_info[fd as usize].thread != std::ptr::null_mut ()
	  && fd_callback_info[fd as usize].thread != current_thread)
	{ fd += 1; continue; }
      if (fd_callback_info[fd as usize].waiting_thread != std::ptr::null_mut ()
	  && fd_callback_info[fd as usize].waiting_thread != current_thread)
	{ fd += 1; continue; }
      if ((fd_callback_info[fd as usize].flags & FOR_READ) != 0
	  && (fd_callback_info[fd as usize].flags & KEYBOARD_FD) == 0)
	{
	  FD_SET (fd, mask);
	  fd_callback_info[fd as usize].waiting_thread = current_thread;
	}
      fd += 1;
    }
}

/* C 693-714: `compute_write_mask`.  */
unsafe fn compute_write_mask (mask: *mut fd_set)
{
  let mut fd: c_int;

  FD_ZERO (mask);
  eassert! (max_desc < FD_SETSIZE);
  fd = 0;
  while fd <= max_desc
    {
      if (fd_callback_info[fd as usize].thread != std::ptr::null_mut ()
	  && fd_callback_info[fd as usize].thread != current_thread)
	{ fd += 1; continue; }
      if (fd_callback_info[fd as usize].waiting_thread != std::ptr::null_mut ()
	  && fd_callback_info[fd as usize].waiting_thread != current_thread)
	{ fd += 1; continue; }
      if ((fd_callback_info[fd as usize].flags & FOR_WRITE) != 0)
	{
	  FD_SET (fd, mask);
	  fd_callback_info[fd as usize].waiting_thread = current_thread;
	}
      fd += 1;
    }
}

/* C 716-729: `clear_waiting_thread_info`.  */
unsafe fn clear_waiting_thread_info ()
{
  let mut fd: c_int;

  eassert! (max_desc < FD_SETSIZE);
  fd = 0;
  while fd <= max_desc
    {
      if (fd_callback_info[fd as usize].waiting_thread == current_thread)
	{ fd_callback_info[fd as usize].waiting_thread = std::ptr::null_mut (); }
      fd += 1;
    }
}

/* C 731-746: `kbd_is_ours` — TRUE if the keyboard descriptor is being monitored by the
   current thread.  */
unsafe fn kbd_is_ours () -> bool
{
  let mut fd: c_int = 0;
  while fd <= max_desc
    {
      if (fd_callback_info[fd as usize].waiting_thread != current_thread)
	{ fd += 1; continue; }
      if ((fd_callback_info[fd as usize].flags & (FOR_READ | KEYBOARD_FD))
	  == (FOR_READ | KEYBOARD_FD))
	{ return true; }
      fd += 1;
    }
  return false;
}

/* C 751-760: `update_status`.  */
unsafe fn update_status (p: *mut Lisp_Process)
{
  eassert! (process_bits! (raw_status_new, p));
  pset_status (p, status_convert ((*p).raw_status));
  process_bits! (raw_status_new, p, 0);
}

/* C 762-776: `status_convert` — Unix status word to the internal list.  */
unsafe fn status_convert (w: c_int) -> Lisp_Object
{
  if (WIFSTOPPED (w))
    { return Fcons (Qstop (), Fcons (make_fixnum (WSTOPSIG (w) as EMACS_INT), Qnil ())); }
  else if (WIFEXITED (w))
    { return Fcons (Qexit (), Fcons (make_fixnum (WEXITSTATUS (w) as EMACS_INT), Qnil ())); }
  else if (WIFSIGNALED (w))
    { return Fcons (Qsignal (), Fcons (make_fixnum (WTERMSIG (w) as EMACS_INT),
				       if WCOREDUMP (w) { Qt () } else { Qnil () })); }
  else
    { return Qrun (); }
}

/* C 778-785: `connecting_status`.  */
unsafe fn connecting_status (status: Lisp_Object) -> bool
{
  return CONSP (status) && EQ (XCAR (status), Qconnect ());
}

/* C 787-812: `decode_status`.  */
unsafe fn decode_status (l: Lisp_Object, symbol: *mut Lisp_Object, code: *mut Lisp_Object,
			 coredump: *mut bool)
{
  let mut l: Lisp_Object = l;
  let mut tem: Lisp_Object;

  if connecting_status (l)
    { l = XCAR (l); }

  if (SYMBOLP (l))
    {
      *symbol = l;
      *code = make_fixnum (0);
      *coredump = false;
    }
  else
    {
      *symbol = XCAR (l);
      tem = XCDR (l);
      *code = XCAR (tem);
      tem = XCDR (tem);
      *coredump = !NILP (tem);
    }
}

/* C 814-881: `status_message` — a string describing a process status list.  */
unsafe fn status_message (p: *mut Lisp_Process) -> Lisp_Object
{
  let status: Lisp_Object = (*p).status;
  let mut symbol: Lisp_Object = lisp_h_Qnil ();
  let mut code: Lisp_Object = lisp_h_Qnil ();
  let mut coredump: bool = false;
  let mut string: Lisp_Object;

  decode_status (status, &raw mut symbol, &raw mut code, &raw mut coredump);

  if (EQ (symbol, Qsignal ()) || EQ (symbol, Qstop ()))
    {
      let mut signame: *const c_char;
      synchronize_system_messages_locale ();
      signame = strsignal (XFIXNAT (code) as c_int);
      if (signame == std::ptr::null ())
	{ string = build_string (c"unknown".as_ptr ()); }
      else
	{
	  let mut c1: c_int;
	  let mut c2: c_int;

	  string = build_unibyte_string (signame);
	  if (! NILP (Vlocale_coding_system))
	    { string = code_convert_string_norecord (string, Vlocale_coding_system, false); }
	  c1 = STRING_CHAR (SDATA (string));
	  c2 = downcase (c1);
	  if (c1 != c2)
	    {
	      if (ASCII_CHAR_P (c1 as EMACS_INT) && ASCII_CHAR_P (c2 as EMACS_INT))
		{ Faset (string, make_fixnum (0), make_fixnum (c2 as EMACS_INT)); }
	      else
		{ string = concat2 (Fdowncase (Fsubstring (string,
							   make_fixnum (0),
							   make_fixnum (1))),
				    Fsubstring (string, make_fixnum (1), Qnil ())); }
	    }
	}
      /* C 874: `AUTO_STRING (suffix, coredump ? " (core dumped)\n" : "\n");`  */
      /* C: ONE object whose initialiser is the ternary (PORTING.md P-113 ②).  */
      AUTO_STRING! (suffix, if coredump
      	      { c" (core dumped)\n".as_ptr () }
      	      else { c"\n".as_ptr () });
      return concat2 (string, suffix);
    }
  else if (EQ (symbol, Qexit ()))
    {
      if (NETCONN1_P (p))
	{
	  let msg: *const c_char = if XFIXNAT (code) == 0
	    { c"deleted\n".as_ptr () }
	    else { c"connection broken by remote peer\n".as_ptr () };
	  return build_string (msg);
	}
      if (XFIXNAT (code) == 0)
	{ return build_string (c"finished\n".as_ptr ()); }
      AUTO_STRING! (prefix, c"exited abnormally with code ".as_ptr ());
      string = Fnumber_to_string (code);
      /* C: ONE object whose initialiser is the ternary (PORTING.md P-113 ②).  */
      AUTO_STRING! (suffix, if coredump
		    { c" (core dumped)\n".as_ptr () }
		    else { c"\n".as_ptr () });
      return concat3 (prefix, string, suffix);
    }
  else if (EQ (symbol, Qfailed ()))
    {
      AUTO_STRING! (format, c"failed with code %s\n".as_ptr ());
      return CALLN! (Fformat, format, code);
    }
  else
    { return Fcopy_sequence (Fsymbol_name (symbol)); }
}

/* C 883: `enum { PTY_NAME_SIZE = 24 };` */
pub const PTY_NAME_SIZE: c_int = 24;

/* C 885-906: `allocate_pty`.  `HAVE_PTYS` is live in the frozen table; `PTY_ITERATION`,
   `PTY_NAME_SPRINTF`, `PTY_OPEN` and `PTY_TTY_NAME_SPRINTF` come from `systty.h` (which has
   no mirror), so the unit declares them with C's own definition text, and each `#else` arm is
   carried as text (P-51).  `#ifdef PTY_NAME_SPRINTF` is *live*: systty.h defines it as empty,
   so the live arm does nothing where the carried `#else` arm runs `sprintf`.  */
/* A `macro_rules!` body resolves caller locals at the *definition* site (mixed-site hygiene,
   docs/status.md 3.69), so C's macros that name the enclosing function's locals take them as
   parameters here; the call sites still spell C's own names.  */
#[cfg (feature = "no_pty_iteration")]
macro_rules! PTY_ITERATION {
  ($body:block) => {
    {
      let mut c: c_int = 0;
      let mut i: c_int = 0;
      c = FIRST_PTY_LETTER;
      while c <= 0x7a { i = 0; while i < 16 { i += 1; } c += 1; }
    }
  }
}
#[cfg (not (feature = "no_pty_iteration"))]
macro_rules! PTY_ITERATION {
  ($body:block) => {{ let mut i: c_int = 0; while i < 1 { $body i += 1; } }}
}
#[cfg (not (feature = "no_pty_open"))]
macro_rules! PTY_OPEN {
  ($fd:ident) => {{
    $fd = posix_openpt (O_RDWR | O_CLOEXEC | O_NOCTTY);
    if ($fd < 0 && *errno () == EINVAL)
      { $fd = posix_openpt (O_RDWR | O_NOCTTY); }
  }}
}
#[cfg (feature = "no_pty_open")]
macro_rules! PTY_OPEN {
  ($fd:ident, $pty_name:ident) => {{ $fd = emacs_open ($pty_name, O_RDWR | O_NONBLOCK, 0); }}
}
#[cfg (feature = "no_pty_tty_name_sprintf")]
macro_rules! PTY_TTY_NAME_SPRINTF {
  ($pty_name:ident, $c:ident, $i:ident) =>
    {{ sprintf ($pty_name, c"/dev/tty%c%x", $c, $i); }}
}
#[cfg (not (feature = "no_pty_tty_name_sprintf"))]
macro_rules! PTY_TTY_NAME_SPRINTF {
  ($fd:ident, $pty_name:ident) => {{
    let mut ptyname: *mut c_char = std::ptr::null_mut ();
    let mut blocked: sigset_t = std::mem::zeroed ();
    sigemptyset (&raw mut blocked);
    sigaddset (&raw mut blocked, SIGCHLD);
    pthread_sigmask (SIG_BLOCK, &raw const blocked, std::ptr::null_mut ());
    if (grantpt ($fd) != -1 && unlockpt ($fd) != -1)
      { ptyname = ptsname ($fd); }
    pthread_sigmask (SIG_UNBLOCK, &raw const blocked, std::ptr::null_mut ());
    if ptyname.is_null () { emacs_close ($fd); return -1; }
    snprintf ($pty_name, PTY_NAME_SIZE as usize, c"%s".as_ptr (), ptyname);
  }}
}
#[cfg (feature = "no_pty_name_sprintf")]
macro_rules! PTY_NAME_SPRINTF {
  ($pty_name:ident, $c:ident, $i:ident) =>
    {{ sprintf ($pty_name, c"/dev/pty%c%x", $c, $i); }}
}
#[cfg (not (feature = "no_pty_name_sprintf"))]
macro_rules! PTY_NAME_SPRINTF { ($pty_name:ident, $c:ident, $i:ident) => {{ }} }

unsafe fn allocate_pty (pty_name: *mut c_char) -> c_int
{
  /* C 886-885's `#ifdef HAVE_PTYS` is live.  */
  let mut fd: c_int;

  /* C 888-894's `#else` arm of `#ifdef PTY_ITERATION` (no PTY_ITERATION).  */
  #[cfg (feature = "no_pty_iteration")]
  {
    let mut c: c_int = FIRST_PTY_LETTER;
    while c <= 0x7a
      {
	let mut i: c_int = 0;
	while i < 16 { i += 1; }
	c += 1;
      }
  }
  PTY_ITERATION! ({
    /* C 896-900: `PTY_NAME_SPRINTF` (live) and the `#else` arm.  */
    PTY_NAME_SPRINTF! (pty_name, c, i);
    #[cfg (feature = "no_pty_name_sprintf")]
    {
      sprintf (pty_name, c"/dev/pty%c%x".as_ptr (), c, i);
    }

    /* C 902-906: `PTY_OPEN` (live) and the `#else` arm.  */
    PTY_OPEN! (fd);
    #[cfg (feature = "no_pty_open")]
    {
      fd = emacs_open (pty_name, O_RDWR | O_NONBLOCK, 0);
    }

    fd = inrange_fd (fd);

    if (fd >= 0)
      {
	/* C 909-914: `PTY_TTY_NAME_SPRINTF` (live) and the `#else` arm.  */
	PTY_TTY_NAME_SPRINTF! (fd, pty_name);
	#[cfg (feature = "no_pty_tty_name_sprintf")]
	{
	  sprintf (pty_name, c"/dev/tty%c%x".as_ptr (), c, i);
	}

	/* Set FD's close-on-exec flag.  This is needed even if PT_OPEN calls posix_openpt
	   with O_CLOEXEC, since POSIX doesn't require support for that combination.  */
	fcntl (fd, F_SETFD, FD_CLOEXEC);

	/* Check to make certain that both sides are available.  */
	if (sys_faccessat (AT_FDCWD, pty_name, R_OK | W_OK, AT_EACCESS) != 0)
	  {
	    emacs_close (fd);
	    continue;
	  }
	setup_pty (fd);
	return fd;
      }
    else if (*errno () == EMFILE)
      { return fd; }
  });
  /* C 885's `#echo` for `!HAVE_PTYS` returns -1; the `#endif` closes the region.  */
  return -1;
}

/* ------------------------------------------------------------------------------------------
   Use-site declarations, group 2 (C 570-1100).  */

/* <sys/select.h>'s three writers - `FD_SET` (C 106), `FD_CLR` (C 111) and `FD_ZERO` (C 115) -
   are the mirror's now (`lib/include/sys_select.rs` 87/99/111 carries them since the
   select-family round), so this unit's use-site copies are gone (PORTING.md 0.1-3; the
   `build-tu` failure that found them was `E0428`).  */

/* `enum { PTY_NAME_SIZE = 24 }`'s sibling in systty.h: `FIRST_PTY_LETTER` is `undef` in the
   frozen table (Darwin uses `PTY_ITERATION` instead), so it appears only in carried text.  */

/// C: `#define WTERMSIG(status) ((status) & 0x7f)`   (<sys/wait.h>, the callproc.rs shape)
#[inline(always)] fn WTERMSIG (status: c_int) -> c_int { status & 0o177 }
/// C: `#define WIFEXITED(status) (WTERMSIG (status) == 0)`   (<sys/wait.h>)
#[inline(always)] fn WIFEXITED (status: c_int) -> bool { (status & 0o177) == 0 }
/// C: `#define WEXITSTATUS(status) (((status) & 0xff00) >> 8)`   (<sys/wait.h>)
#[inline(always)] fn WEXITSTATUS (status: c_int) -> c_int { (status >> 8) & 0xff }
/// C: `#define WIFSTOPPED(status) (((status) & 0x7f) == 0x7f)`   (<sys/wait.h>)
#[inline(always)] fn WIFSTOPPED (status: c_int) -> bool { (status & 0o177) == 0o177 }
/// C: `#define WSTOPSIG(status) WEXITSTATUS (status)`   (<sys/wait.h>)
#[inline(always)] fn WSTOPSIG (status: c_int) -> c_int { (status >> 8) & 0xff }
/// C: `#define WIFSIGNALED(status) (WTERMSIG (status) != 0 && ! WIFSTOPPED (status))`
#[inline(always)] fn WIFSIGNALED (status: c_int) -> bool { (status & 0o177) != 0 && ! WIFSTOPPED (status) }
/// C: `#define WCOREDUMP(status) ((status) & 0x80)`   (<sys/wait.h>)
#[inline(always)] fn WCOREDUMP (status: c_int) -> bool { (status & 0x80) != 0 }

unsafe extern "C" {
  /// C: int posix_openpt (int)   (<stdlib.h>, HAVE_POSIX_OPENPT)
  fn posix_openpt (oflag: c_int) -> c_int;
  /// C: int grantpt (int)   (<stdlib.h>)
  fn grantpt (fd: c_int) -> c_int;
  /// C: int unlockpt (int)   (<stdlib.h>)
  fn unlockpt (fd: c_int) -> c_int;
  /// C: char *ptsname (int)   (<stdlib.h>)
  fn ptsname (fd: c_int) -> *mut c_char;
  // `<stdio.h>`'s `int snprintf (char *restrict, size_t, const char *restrict, ...)` comes from
  // `lib/include/stdio.rs` (status 3.263): the mirror carries it, so this block does not (0.1-3).
  /// C: void sigemptyset (sigset_t *)   (<signal.h>)
  fn sigemptyset (set: *mut sigset_t) -> c_int;
  /// C: void sigaddset (sigset_t *, int)   (<signal.h>)
  fn sigaddset (set: *mut sigset_t, signo: c_int) -> c_int;
  /// C: int sys_faccessat (int, const char *, int, int)   (sysdep.h 30)
  fn sys_faccessat (dirfd: c_int, file: *const c_char, mode: c_int, flags: c_int) -> c_int;
  /// C: void setup_pty (int)   (sysdep.c 3825)
  fn setup_pty (fd: c_int);
  /// C: <signal.h>: `int pthread_sigmask (int, const sigset_t *, sigset_t *);` — the port's
  /// `block_child_signal` (sysdep.rs) uses it, and the P-122 instrument reads the mask with it.
  fn pthread_sigmask (how: c_int, set: *const sigset_t, oset: *mut sigset_t) -> c_int;
  /// C: <signal.h>: `int sigismember (const sigset_t *, int);`
  fn sigismember (set: *const sigset_t, signum: c_int) -> c_int;
  /// C: syswait.h 60: `extern pid_t child_status_changed (pid_t, int *, int);` - the return
  /// type is the child's pid (sysdep.c 516), and the port's definition returns it too.  It was
  /// declared `bool` here, which made the caller test only the low bit of the pid: an even pid
  /// (the common case on this target) then read as "no status changed", the reap was skipped,
  /// and the process stayed `run` (PORTING.md P-124).
  fn child_status_changed (child: pid_t, status: *mut c_int, options: c_int) -> pid_t;
  /// C: void synchronize_system_messages_locale (void)   (lisp.h 5142)
  fn synchronize_system_messages_locale ();
  /// C: char *strsignal (int)   (syssignal.h 83's `#ifndef HAVE_STRSIGNAL` is dead:
  /// HAVE_STRSIGNAL is 1, so this is the libc entry, the callproc.rs shape)
  fn strsignal (sig: c_int) -> *mut c_char;
}

/* C 932-938: `allocate_process` — a basically initialized process.  */
unsafe fn allocate_process () -> *mut Lisp_Process
{
  return ALLOCATE_ZEROED_PSEUDOVECTOR! (Lisp_Process, thread, pvec_type::PVEC_PROCESS)
    as *mut Lisp_Process;
}

/* C 940-993: `make_process`.  */
unsafe fn make_process (name: Lisp_Object) -> Lisp_Object
{
  let p: *mut Lisp_Process = allocate_process ();
  /* Initialize Lisp data.  Note that allocate_process initializes all Lisp data to nil, so
     do it only for slots which should not be nil.  */
  pset_status (p, Qrun ());
  pset_mark (p, Fmake_marker ());
  pset_thread (p, Fcurrent_thread ());

  /* Initialize non-Lisp data.  Note that allocate_process zeroes out all non-Lisp data, so
     do it only for slots which should not be zero.  */
  (*p).infd = -1;
  (*p).outfd = -1;
  let mut i: c_int = 0;
  while i < PROCESS_OPEN_FDS
    {
      (*p).open_fd[i as usize] = -1;
      i += 1;
    }

  /* C 963-967's `#ifdef HAVE_GNUTLS` arm.  */
  #[cfg (feature = "have_gnutls")]
  {
    static_assert! (GNUTLS_STAGE_EMPTY == 0);
    eassert! ((*p).gnutls_initstage == GNUTLS_STAGE_EMPTY);
    eassert! (NILP ((*p).gnutls_boot_parameters));
  }

  (*p).readmax = clip_to_bounds (1, read_process_output_max, INT_MAX as EMACS_INT);

  /* If name is already in use, modify it until it is unused.  */

  let mut name: Lisp_Object = name;
  let mut name1: Lisp_Object = name;
  let mut i: i64 = 1;
  loop
    {
      let tem: Lisp_Object = Fget_process (name1);
      if NILP (tem) { break; }
      /* C 1004-1007: `char const suffix_fmt[] = "<%"PRIdMAX">";
	 char suffix[sizeof suffix_fmt + INT_STRLEN_BOUND (i)];`  */
      let suffix_fmt: *const c_char = c"<%jd>".as_ptr ();
      let mut suffix: [c_char; 32] = [0; 32];
      let len: c_int = sprintf (suffix.as_mut_ptr (), suffix_fmt, i);
      AUTO_STRING_WITH_LEN! (lsuffix, suffix.as_ptr (), len as isize);
      name1 = concat2 (name, lsuffix);
      i += 1;
    }
  name = name1;
  pset_name (p, name);
  pset_sentinel (p, Qinternal_default_process_sentinel ());
  pset_filter (p, Qinternal_default_process_filter ());
  let mut val: Lisp_Object = lisp_h_Qnil ();
  XSETPROCESS! (val, p);
  Vprocess_alist = Fcons (Fcons (name, val), Vprocess_alist);
  return val;
}

/* C 995-1002: `remove_process` — its address is taken by `record_unwind_protect` in
   `Fmake_pipe_process`, so it is `unsafe extern "C" fn` (PORTING.md 1.4).  */
unsafe extern "C" fn remove_process (proc: Lisp_Object)
{
  let pair: Lisp_Object;

  pair = Frassq (proc, Vprocess_alist);
  Vprocess_alist = Fdelq (pair, Vprocess_alist);

  deactivate_process (proc);
}

/* C 1004-1033: `update_processes_for_thread_death`.  */
#[unsafe(no_mangle)]
pub unsafe fn update_processes_for_thread_death (dying_thread: Lisp_Object)
{
  let mut pair: Lisp_Object;

  pair = Vprocess_alist;
  while !NILP (pair)
    {
      let process: Lisp_Object = XCDR (XCAR (pair));
      if (EQ ((*XPROCESS (process)).thread, dying_thread))
	{
	  let proc: *mut Lisp_Process = XPROCESS (process);

	  pset_thread (proc, Qnil ());
	  eassert! ((*proc).infd < FD_SETSIZE);
	  if ((*proc).infd >= 0)
	    { fd_callback_info[(*proc).infd as usize].thread = std::ptr::null_mut (); }
	  eassert! ((*proc).outfd < FD_SETSIZE);
	  if ((*proc).outfd >= 0)
	    { fd_callback_info[(*proc).outfd as usize].thread = std::ptr::null_mut (); }
	}
      pair = XCDR (pair);
    }
}

/* C 1035-1049: `free_dns_request` — the `HAVE_GETADDRINFO_A` arm (dead; there is no
   `dns_request` member on this target).  */
#[cfg (feature = "have_getaddrinfo_a")]
unsafe fn free_dns_request (proc_: Lisp_Object)
{
  /* C 1039: `struct Lisp_Process *p = XPROCESS (proc);`  */
  let p: *mut Lisp_Process = XPROCESS (proc_);

  if (!std::ptr::null_mut::<addrinfo> () == (*p).dns_request.ar_result)
    { freeaddrinfo ((*p).dns_request.ar_result); }
  xfree ((*p).dns_request as *mut c_void);
  (*p).dns_request = std::ptr::null_mut ();
}

DEFUN! ("processp", Fprocessp, Sprocessp, 1, 1, 0,
       doc: "Return t if OBJECT is a process.",
       (object: Lisp_Object),
       {
  return if PROCESSP (object) { Qt () } else { Qnil () };
});

DEFUN! ("get-process", Fget_process, Sget_process, 1, 1, 0,
       doc: "Return the process named NAME, or nil if there is none.",
       (name: Lisp_Object),
       {
  if (PROCESSP (name))
    { return name; }
  CHECK_STRING (name);
  return Fcdr (Fassoc (name, Vprocess_alist, Qnil ()));
});

/* This is how commands for the user decode process arguments.  It accepts a process, a
   process name, a buffer, a buffer name, or nil.  Buffers denote the first process in the
   buffer, and nil denotes the current buffer.  */

/* C 1072-1106: `get_process`.  */
unsafe fn get_process (name: Lisp_Object) -> Lisp_Object
{
  let mut proc: Lisp_Object;
  let mut obj: Lisp_Object;
  if (STRINGP (name))
    {
      obj = Fget_process (name);
      if (NILP (obj))
	{ obj = Fget_buffer (name); }
      if (NILP (obj))
	{ error (c"Process %s does not exist".as_ptr (), SDATA (name)); }
    }
  else if (NILP (name))
    { obj = Fcurrent_buffer (); }
  else
    { obj = name; }

  /* Now obj should be either a buffer object or a process object.  */
  if (BUFFERP (obj))
    {
      if (NILP (BVAR! (XBUFFER (obj), name_)))
	{ error (c"Attempt to get process for a dead buffer".as_ptr ()); }
      proc = Fget_buffer_process (obj);
      if (NILP (proc))
	{ error (c"Buffer %s has no process".as_ptr (), SDATA (BVAR! (XBUFFER (obj), name_))); }
    }
  else
    {
      CHECK_PROCESS (obj);
      proc = obj;
    }
  return proc;
}

/* C 1108-1118: `deleted_pid_list` — C's static, zero-initialised; the port uses `Qnil` (the
   callproc.rs shape).  */
static mut deleted_pid_list: Lisp_Object = lisp_h_Qnil ();

/* C 1121-1127: `record_deleted_pid`.  */
#[unsafe(no_mangle)]
pub unsafe fn record_deleted_pid (pid: pid_t, filename: Lisp_Object)
{
  deleted_pid_list = Fcons (Fcons (INT_TO_INTEGER (pid as EMACS_INT), filename),
			    /* GC treated elements set to nil.  */
			    Fdelq (Qnil (), deleted_pid_list));
}

DEFUN! ("delete-process", Fdelete_process, Sdelete_process, 0, 1, "(list 'message)",
       doc: "Delete PROCESS: kill it and forget about it immediately.\nPROCESS may be a process, a buffer, the name of a process or buffer, or\nnil, indicating the current buffer's process.\n\nInteractively, it will kill the current buffer's process.",
       (process: Lisp_Object),
       {
  let mut process: Lisp_Object = process;
  let p: *mut Lisp_Process;
  let mut mess: bool = false;

  /* We use this to see whether we were called interactively.  */
  if (EQ (process, Qmessage ()))
    {
      mess = true;
      process = Qnil ();
    }

  process = get_process (process);
  p = XPROCESS (process);

  /* C 1151-1170's `#ifdef HAVE_GETADDRINFO_A` arm: cancel the pending request.  */
  #[cfg (feature = "have_getaddrinfo_a")]
  {
    if (!(*p).dns_request.is_null ())
      {
	/* Cancel the request.  Unless shutting down, wait until completion.  Free the
	   request if completely canceled.  */

	let mut canceled: bool = gai_cancel ((*p).dns_request) != EAI_NOTCANCELED;
	if (!canceled && !inhibit_sentinels)
	  {
	    let req: *const gaicb = (*p).dns_request;
	    while (gai_suspend (&raw const req, 1, std::ptr::null_mut ()) != 0)
	      { continue; }
	    canceled = true;
	  }
	if (canceled)
	  { free_dns_request (process); }
      }
  }

  process_bits! (raw_status_new, p, 0);
  if (NETCONN1_P (p) || SERIALCONN1_P (p) || PIPECONN1_P (p))
    {
      pset_status (p, list2 (Qexit (), make_fixnum (0)));
      process_tick += 1;
      (*p).tick = process_tick;
      status_notify (p, std::ptr::null_mut ());
      redisplay_preserve_echo_area (13);
    }
  else
    {
      if (process_bits! (alive, p))
	{ record_kill_process (p, Qnil ()); }

      if ((*p).infd >= 0)
	{
	  /* Update P's status, since record_kill_process will make the SIGCHLD handler
	     update deleted_pid_list, not *P.  */
	  let mut symbol: Lisp_Object;
	  if (process_bits! (raw_status_new, p))
	    { update_status (p); }
	  symbol = if CONSP ((*p).status) { XCAR ((*p).status) } else { (*p).status };
	  if (! (EQ (symbol, Qsignal ()) || EQ (symbol, Qexit ())))
	    { pset_status (p, list2 (Qsignal (), make_fixnum (SIGKILL as EMACS_INT))); }

	  process_tick += 1;
      (*p).tick = process_tick;
	  status_notify (p, std::ptr::null_mut ());
	  redisplay_preserve_echo_area (13);
	}
    }
  remove_process (process);
  if (mess)
    { message (c"Deleted process".as_ptr ()); }
  return Qnil ();
});

DEFUN! ("process-status", Fprocess_status, Sprocess_status, 1, 1, 0,
       doc: "Return the status of PROCESS.\nThe returned value is one of the following symbols:\nrun  -- for a process that is running.\nstop -- for a process stopped but continuable.\nexit -- for a process that has exited.\nsignal -- for a process that has got a fatal signal.\nopen -- for a network stream connection that is open.\nlisten -- for a network stream server that is listening.\nclosed -- for a network stream connection that is closed.\nconnect -- when waiting for a non-blocking connection to complete.\nfailed -- when a non-blocking connection has failed.\nnil -- if arg is a process name and no such process exists.\nPROCESS may be a process, a buffer, the name of a process, or\nnil, indicating the current buffer's process.",
       (process: Lisp_Object),
       {
  let mut process: Lisp_Object = process;
  let p: *mut Lisp_Process;
  let mut status: Lisp_Object;

  if (STRINGP (process))
    { process = Fget_process (process); }
  else
    { process = get_process (process); }

  if (NILP (process))
    { return process; }

  p = XPROCESS (process);
  if (process_bits! (raw_status_new, p))
    { update_status (p); }
  status = (*p).status;
  if (CONSP (status))
    { status = XCAR (status); }
  if (NETCONN1_P (p) || SERIALCONN1_P (p) || PIPECONN1_P (p))
    {
      if (EQ (status, Qexit ()))
	{ status = Qclosed (); }
      else if (EQ ((*p).command, Qt ()))
	{ status = Qstop (); }
      else if (EQ (status, Qrun ()))
	{ status = Qopen (); }
    }
  return status;
});

/* ------------------------------------------------------------------------------------------
   Use-site declarations, group 3 (C 914-1250).  */

unsafe extern "C" {
  /// C: void redisplay_preserve_echo_area (int)   (dispextern.h 3589)
  fn redisplay_preserve_echo_area (from_where: c_int);
  /// C: void record_kill_process (struct Lisp_Process *, Lisp_Object)   (process.h 285)
  fn record_kill_process (p: *mut Lisp_Process, tempfile: Lisp_Object);
  /// C: void message (const char *, ...)   (lisp.h 4517, xdisp.c)
  fn message (format: *const c_char, ...) -> Lisp_Object;
  /// C: void freeaddrinfo (struct addrinfo *)   (<netdb.h>)
  fn freeaddrinfo (ai: *mut addrinfo);
}

DEFUN! ("process-exit-status", Fprocess_exit_status, Sprocess_exit_status, 1, 1, 0,
       doc: "Return the exit status of PROCESS or the signal number that killed it.\nIf PROCESS has not yet exited or died, return 0.",
       (process: Lisp_Object),
       {
  CHECK_PROCESS (process);
  if (process_bits! (raw_status_new, XPROCESS (process)))
    { update_status (XPROCESS (process)); }
  if (CONSP ((*XPROCESS (process)).status))
    { return XCAR (XCDR ((*XPROCESS (process)).status)); }
  return make_fixnum (0);
});

DEFUN! ("process-id", Fprocess_id, Sprocess_id, 1, 1, 0,
       doc: "Return the process id of PROCESS.\nThis is the pid of the external process which PROCESS uses or talks to,\nan integer.\nFor a network, serial, and pipe connections, this value is nil.",
       (process: Lisp_Object),
       {
  let pid: pid_t;

  CHECK_PROCESS (process);
  pid = (*XPROCESS (process)).pid;
  return if pid != 0 { INT_TO_INTEGER (pid as EMACS_INT) } else { Qnil () };
});

DEFUN! ("process-name", Fprocess_name, Sprocess_name, 1, 1, 0,
       doc: "Return the name of PROCESS, as a string.\nThis is the name of the program invoked in PROCESS,\npossibly modified to make it unique among process names.",
       (process: Lisp_Object),
       {
  CHECK_PROCESS (process);
  return (*XPROCESS (process)).name;
});

DEFUN! ("process-command", Fprocess_command, Sprocess_command, 1, 1, 0,
       doc: "Return the command that was executed to start PROCESS.\nThis is a list of strings, the first string being the program executed\nand the rest of the strings being the arguments given to it.\nFor a network or serial or pipe connection, this is nil (process is running)\nor t (process is stopped).",
       (process: Lisp_Object),
       {
  CHECK_PROCESS (process);
  return (*XPROCESS (process)).command;
});

DEFUN! ("process-tty-name", Fprocess_tty_name, Sprocess_tty_name, 1, 2, 0,
       doc: "Return the name of the terminal PROCESS uses, or nil if none.\nThis is the terminal that the process itself reads and writes on,\nnot the name of the pty that Emacs uses to talk with that terminal.\n\nIf STREAM is nil, return the terminal name if any of PROCESS's\nstandard streams use a terminal for communication.  If STREAM is one\nof `stdin', `stdout', or `stderr', return the name of the terminal\nPROCESS uses for that stream specifically, or nil if that stream\ncommunicates via a pipe.",
       (process: Lisp_Object, stream: Lisp_Object),
       {
  CHECK_PROCESS (process);
  let p: *mut Lisp_Process = XPROCESS (process);

  if (NILP (stream))
    { return (*p).tty_name; }
  else if (EQ (stream, Qstdin ()))
    { return if process_bits! (pty_in, p) { (*p).tty_name } else { Qnil () }; }
  else if (EQ (stream, Qstdout ()))
    { return if process_bits! (pty_out, p) { (*p).tty_name } else { Qnil () }; }
  else if (EQ (stream, Qstderr ()))
    { return if process_bits! (pty_out, p) && NILP ((*p).stderrproc)
	{ (*p).tty_name } else { Qnil () }; }
  else
    { signal_error (c"Unknown stream".as_ptr (), stream); }
});

/* C 1329-1337: `update_process_mark`.  */
unsafe fn update_process_mark (p: *mut Lisp_Process)
{
  let buffer: Lisp_Object = (*p).buffer;
  if (BUFFERP (buffer)
      && (*XMARKER ((*p).mark)).buffer != XBUFFER (buffer) as *mut c_void)
    { set_marker_both ((*p).mark, buffer,
		       BUF_ZV (XBUFFER (buffer)),
		       BUF_ZV_BYTE (XBUFFER (buffer))); }
}

DEFUN! ("set-process-buffer", Fset_process_buffer, Sset_process_buffer, 2, 2, 0,
       doc: "Set buffer associated with PROCESS to BUFFER (a buffer, or nil).\nReturn BUFFER.",
       (process: Lisp_Object, buffer: Lisp_Object),
       {
  let p: *mut Lisp_Process;

  CHECK_PROCESS (process);
  if (!NILP (buffer))
    { CHECK_BUFFER (buffer); }
  p = XPROCESS (process);
  if (!EQ ((*p).buffer, buffer))
    {
      pset_buffer (p, buffer);
      update_process_mark (p);
    }
  if (NETCONN1_P (p) || SERIALCONN1_P (p) || PIPECONN1_P (p))
    { pset_childp (p, plist_put ((*p).childp, QCbuffer (), buffer)); }
  setup_process_coding_systems (process);
  return buffer;
});

DEFUN! ("process-buffer", Fprocess_buffer, Sprocess_buffer, 1, 1, 0,
       doc: "Return the buffer PROCESS is associated with.\nThe default process filter inserts output from PROCESS into this buffer.",
       (process: Lisp_Object),
       {
  CHECK_PROCESS (process);
  return (*XPROCESS (process)).buffer;
});

DEFUN! ("process-mark", Fprocess_mark, Sprocess_mark, 1, 1, 0,
       doc: "Return the marker for the end of the last output from PROCESS.",
       (process: Lisp_Object),
       {
  CHECK_PROCESS (process);
  return (*XPROCESS (process)).mark;
});

/* C 1382-1391: `set_process_filter_masks`.  */
unsafe fn set_process_filter_masks (p: *mut Lisp_Process)
{
  if (EQ ((*p).filter, Qt ()) && !EQ ((*p).status, Qlisten ()))
    { delete_read_fd ((*p).infd); }
  else if (EQ ((*p).filter, Qt ())
	   /* Network or serial process not stopped:  */
	   && !EQ ((*p).command, Qt ()))
    { add_process_read_fd ((*p).infd); }
}

/* C 1393-1403: `is_pty_from_symbol`.  */
unsafe fn is_pty_from_symbol (symbol: Lisp_Object) -> bool
{
  if (EQ (symbol, Qpty ()))
    { return true; }
  else if (EQ (symbol, Qpipe ()))
    { return false; }
  else if (NILP (symbol))
    { return !NILP (Vprocess_connection_type); }
  else
    { report_file_error (c"Unknown connection type".as_ptr (), symbol); }
}

DEFUN! ("set-process-filter", Fset_process_filter, Sset_process_filter, 2, 2, 0,
       doc: "Give PROCESS the filter function FILTER; nil means default.\nA value of t means stop accepting output from the process.\n\nWhen a process has a non-default filter, its buffer is not used for output.\nInstead, each time it does output, the entire string of output is\npassed to the filter.\n\nThe filter gets two arguments: the process and the string of output.\nThe string argument is normally a multibyte string, except:\n- if the process's input coding system is no-conversion or raw-text,\n  it is a unibyte string (the non-converted input).",
       (process: Lisp_Object, filter: Lisp_Object),
       {
  CHECK_PROCESS (process);
  let p: *mut Lisp_Process = XPROCESS (process);
  let mut filter: Lisp_Object = filter;

  /* Don't signal an error if the process's input file descriptor is closed.  */

  if (NILP (filter))
    { filter = Qinternal_default_process_filter (); }

  if ((*p).infd >= 0)
    {
      /* If filter WILL be t, stop reading output.  */
      if (EQ (filter, Qt ()) && !EQ ((*p).status, Qlisten ()))
	{ delete_read_fd ((*p).infd); }
      else if (/* If filter WAS t, then resume reading output.  */
	       EQ ((*p).filter, Qt ())
	       /* Network or serial process not stopped:  */
	       && !EQ ((*p).command, Qt ()))
	{ add_process_read_fd ((*p).infd); }
    }

  pset_filter (p, filter);

  if (NETCONN1_P (p) || SERIALCONN1_P (p) || PIPECONN1_P (p))
    { pset_childp (p, plist_put ((*p).childp, QCfilter (), filter)); }
  setup_process_coding_systems (process);
  return filter;
});

DEFUN! ("process-filter", Fprocess_filter, Sprocess_filter, 1, 1, 0,
       doc: "Return the filter function of PROCESS.\nSee `set-process-filter' for more info on filter functions.",
       (process: Lisp_Object),
       {
  CHECK_PROCESS (process);
  return (*XPROCESS (process)).filter;
});

DEFUN! ("set-process-sentinel", Fset_process_sentinel, Sset_process_sentinel, 2, 2, 0,
       doc: "Give PROCESS the sentinel SENTINEL; nil for default.\nThe sentinel is called as a function when the process changes state.\nIt gets two arguments: the process, and a string describing the change.",
       (process: Lisp_Object, sentinel: Lisp_Object),
       {
  let p: *mut Lisp_Process;
  let mut sentinel: Lisp_Object = sentinel;

  CHECK_PROCESS (process);
  p = XPROCESS (process);

  if (NILP (sentinel))
    { sentinel = Qinternal_default_process_sentinel (); }

  pset_sentinel (p, sentinel);
  if (NETCONN1_P (p) || SERIALCONN1_P (p) || PIPECONN1_P (p))
    { pset_childp (p, plist_put ((*p).childp, QCsentinel (), sentinel)); }
  return sentinel;
});

DEFUN! ("process-sentinel", Fprocess_sentinel, Sprocess_sentinel, 1, 1, 0,
       doc: "Return the sentinel of PROCESS.\nSee `set-process-sentinel' for more info on sentinels.",
       (process: Lisp_Object),
       {
  CHECK_PROCESS (process);
  return (*XPROCESS (process)).sentinel;
});

/* C 1496-1506: `set_proc_thread`.  */
unsafe fn set_proc_thread (proc: *mut Lisp_Process, thrd: *mut thread_state)
{
  eassert! ((NILP ((*proc).thread) && thrd.is_null ())
	    || (THREADP ((*proc).thread) && XTHREAD ((*proc).thread) == thrd));
  eassert! ((*proc).infd < FD_SETSIZE);
  if ((*proc).infd >= 0)
    { fd_callback_info[(*proc).infd as usize].thread = thrd; }
  eassert! ((*proc).outfd < FD_SETSIZE);
  if ((*proc).outfd >= 0)
    { fd_callback_info[(*proc).outfd as usize].thread = thrd; }
}

DEFUN! ("set-process-thread", Fset_process_thread, Sset_process_thread, 2, 2, 0,
       doc: "Set the locking thread of PROCESS to be THREAD.\nIf THREAD is nil, the process is unlocked.",
       (process: Lisp_Object, thread: Lisp_Object),
       {
  let proc: *mut Lisp_Process;
  let mut tstate: *mut thread_state;

  CHECK_PROCESS (process);
  if (NILP (thread))
    { tstate = std::ptr::null_mut (); }
  else
    {
      CHECK_THREAD (thread);
      tstate = XTHREAD (thread);
    }

  proc = XPROCESS (process);
  pset_thread (proc, thread);
  set_proc_thread (proc, tstate);

  return thread;
});

DEFUN! ("process-thread", Fprocess_thread, Sprocess_thread, 1, 1, 0,
       doc: "Return the locking thread of PROCESS.\nIf PROCESS is unlocked, this function returns nil.",
       (process: Lisp_Object),
       {
  CHECK_PROCESS (process);
  return (*XPROCESS (process)).thread;
});

DEFUN! ("set-process-window-size", Fset_process_window_size, Sset_process_window_size,
       3, 3, 0,
       doc: "Tell PROCESS that it has logical window size WIDTH by HEIGHT.\nValue is t if PROCESS was successfully told about the window size,\nnil otherwise.",
       (process: Lisp_Object, height: Lisp_Object, width: Lisp_Object),
       {
  CHECK_PROCESS (process);

  /* All known platforms store window sizes as 'unsigned short'.  */
  let h: c_ushort = check_uinteger_max (height, USHRT_MAX as u64) as c_ushort;
  let w: c_ushort = check_uinteger_max (width, USHRT_MAX as u64) as c_ushort;

  if (NETCONN_P (process)
      || (*XPROCESS (process)).infd < 0
      || set_window_size ((*XPROCESS (process)).infd, h as c_int, w as c_int) < 0)
    { return Qnil (); }
  else
    { return Qt (); }
});

unsafe extern "C" {
  /// C: Lisp_Object plist_put (Lisp_Object, Lisp_Object, Lisp_Object)   (lisp.h 4281)
  fn plist_put (plist: Lisp_Object, prop: Lisp_Object, val: Lisp_Object) -> Lisp_Object;
  /// C: uintmax_t check_uinteger_max (Lisp_Object, uintmax_t)   (lisp.h 612)
  fn check_uinteger_max (x: Lisp_Object, max: u64) -> u64;
  /// C: int set_window_size (int, int, int)   (lisp.h 5296, process.h 297)
  fn set_window_size (fd: c_int, height: c_int, width: c_int) -> c_int;
}

DEFUN! ("set-process-inherit-coding-system-flag", Fset_process_inherit_coding_system_flag,
       Sset_process_inherit_coding_system_flag, 2, 2, 0,
       doc: "Determine whether buffer of PROCESS will inherit coding-system.\nIf the second argument FLAG is non-nil, then the variable\n`buffer-file-coding-system' of the buffer associated with PROCESS\nwill be bound to the value of the coding system used to decode\nthe process output.\n\nThis is useful when the coding system specified for the process buffer\nleaves either the character code conversion or the end-of-line conversion\nunspecified, or if the coding system used to decode the process output\nis more appropriate for saving the process buffer.\n\nBinding the variable `inherit-process-coding-system' to non-nil before\nstarting the process is an alternative way of setting the inherit flag\nfor the process which will run.\n\nThis function returns FLAG.",
       (process: Lisp_Object, flag: Lisp_Object),
       {
  CHECK_PROCESS (process);
  process_bits! (inherit_coding_system_flag, XPROCESS (process), !NILP (flag));
  return flag;
});

DEFUN! ("set-process-query-on-exit-flag", Fset_process_query_on_exit_flag,
       Sset_process_query_on_exit_flag, 2, 2, 0,
       doc: "Specify if query is needed for PROCESS when Emacs is exited.\nIf the second argument FLAG is non-nil, Emacs will query the user before\nexiting or killing a buffer if PROCESS is running.  This function\nreturns FLAG.",
       (process: Lisp_Object, flag: Lisp_Object),
       {
  CHECK_PROCESS (process);
  process_bits! (kill_without_query, XPROCESS (process), NILP (flag));
  return flag;
});

DEFUN! ("process-query-on-exit-flag", Fprocess_query_on_exit_flag,
       Sprocess_query_on_exit_flag, 1, 1, 0,
       doc: "Return the current value of query-on-exit flag for PROCESS.",
       (process: Lisp_Object),
       {
  CHECK_PROCESS (process);
  return if process_bits! (kill_without_query, XPROCESS (process)) { Qnil () } else { Qt () };
});

DEFUN! ("process-contact", Fprocess_contact, Sprocess_contact, 1, 3, 0,
       doc: "Return the contact info of PROCESS; t for a real child.\nFor a network or serial or pipe connection, the value depends on the\noptional KEY arg.  If KEY is nil, value is a cons cell of the form\n(HOST SERVICE) for a network connection or (PORT SPEED) for a serial\nconnection; it is t for a pipe connection.  If KEY is t, the complete\ncontact information for the connection is returned, else the specific\nvalue for the keyword KEY is returned.  See `make-network-process',\n`make-serial-process', or `make-pipe-process' for the list of keywords.\n\nIf PROCESS is a non-blocking network process that hasn't been fully\nset up yet, this function will block until socket setup has completed.\nIf the optional NO-BLOCK parameter is specified, return nil instead of\nwaiting for the process to be fully set up.",
       (process: Lisp_Object, key: Lisp_Object, no_block: Lisp_Object),
       {
  let mut contact: Lisp_Object;

  CHECK_PROCESS (process);
  contact = (*XPROCESS (process)).childp;

  /* `#ifdef DATAGRAM_SOCKETS` is live.  */
  if (NETCONN_P (process) && (*XPROCESS (process)).infd < 0)
    {
      /* Usually wait for the network process to finish being set up.  */
      if (!NILP (no_block))
	{ return Qnil (); }

      wait_for_socket_fds (process, c"process-contact".as_ptr ());
    }

  if (DATAGRAM_CONN_P (process)
      && (EQ (key, Qt ()) || EQ (key, QCremote ())))
    { contact = plist_put (contact, QCremote (),
			   Fprocess_datagram_address (process)); }

  if ((!NETCONN_P (process) && !SERIALCONN_P (process) && !PIPECONN_P (process))
      || EQ (key, Qt ()))
    { return contact; }
  if (NILP (key) && NETCONN_P (process))
    { return list2 (plist_get (contact, QChost ()),
		    plist_get (contact, QCservice ())); }
  if (NILP (key) && SERIALCONN_P (process))
    { return list2 (plist_get (contact, QCport ()),
		    plist_get (contact, QCspeed ())); }
  /* FIXME: Return a meaningful value (e.g., the child end of the pipe) if the pipe process
     is useful for purposes other than receiving stderr.  */
  if (NILP (key) && PIPECONN_P (process))
    { return Qt (); }
  return plist_get (contact, key);
});

DEFUN! ("process-plist", Fprocess_plist, Sprocess_plist, 1, 1, 0,
       doc: "Return the plist of PROCESS.",
       (process: Lisp_Object),
       {
  CHECK_PROCESS (process);
  return (*XPROCESS (process)).plist;
});

DEFUN! ("set-process-plist", Fset_process_plist, Sset_process_plist, 2, 2, 0,
       doc: "Replace the plist of PROCESS with PLIST.  Return PLIST.",
       (process: Lisp_Object, plist: Lisp_Object),
       {
  CHECK_PROCESS (process);
  CHECK_LIST (plist);

  pset_plist (XPROCESS (process), plist);
  return plist;
});

/* C 1692-1701's `#if 0 /* Turned off because we don't currently record this info in the
   process.  Perhaps add it.  */` is dead everywhere and is recorded in
   `docs/ports/process.md`'s Decisions (P-08), not carried.  */

DEFUN! ("process-type", Fprocess_type, Sprocess_type, 1, 1, 0,
       doc: "Return the connection type of PROCESS.\nThe value is either the symbol `real', `network', `serial', or `pipe'.\nPROCESS may be a process, a buffer, the name of a process or buffer, or\nnil, indicating the current buffer's process.",
       (process: Lisp_Object),
       {
  let proc: Lisp_Object;
  proc = get_process (process);
  return (*XPROCESS (proc)).r#type;
});

DEFUN! ("format-network-address", Fformat_network_address, Sformat_network_address, 1, 2, 0,
       doc: "Convert network ADDRESS from internal format to a string.\nA 4 or 5 element vector represents an IPv4 address (with port number).\nAn 8 or 9 element vector represents an IPv6 address (with port number).\nIf optional second argument OMIT-PORT is non-nil, don't include a port\nnumber in the string, even when present in ADDRESS.\nReturn nil if format of ADDRESS is invalid.",
       (address: Lisp_Object, omit_port: Lisp_Object),
       {
  if (NILP (address))
    { return Qnil (); }

  if (STRINGP (address))  /* AF_LOCAL */
    { return address; }

  if (VECTORP (address))  /* AF_INET or AF_INET6 */
    {
      let p: *mut Lisp_Vector = XVECTOR (address);
      let size: ptrdiff_t = (*p).header.size as ptrdiff_t;
      let mut args: [Lisp_Object; 10] = [lisp_h_Qnil (); 10];
      let mut nargs: c_int;
      let mut i: c_int;
      let mut format: *const c_char;

      if (size == 4 || (size == 5 && !NILP (omit_port)))
	{
	  format = c"%d.%d.%d.%d".as_ptr ();
	  nargs = 4;
	}
      else if (size == 5)
	{
	  format = c"%d.%d.%d.%d:%d".as_ptr ();
	  nargs = 5;
	}
      else if (size == 8 || (size == 9 && !NILP (omit_port)))
	{
	  format = c"%x:%x:%x:%x:%x:%x:%x:%x".as_ptr ();
	  nargs = 8;
	}
      else if (size == 9)
	{
	  format = c"[%x:%x:%x:%x:%x:%x:%x:%x]:%d".as_ptr ();
	  nargs = 9;
	}
      else
	{ return Qnil (); }

      AUTO_STRING! (format_obj, format);
      args[0] = format_obj;

      i = 0;
      while i < nargs
	{
	  if (! RANGED_FIXNUMP (0, FAM_ELEM! ((*p).contents, i), 65535))
	    { return Qnil (); }

	  if (nargs <= 5         /* IPv4 */
	      && i < 4           /* host, not port */
	      && XFIXNUM (FAM_ELEM! ((*p).contents, i)) > 255)
	    { return Qnil (); }

	  args[(i + 1) as usize] = FAM_ELEM! ((*p).contents, i);
	  i += 1;
	}

      return Fformat ((nargs + 1) as isize, args.as_mut_ptr ());
    }

  if (CONSP (address))
    {
      AUTO_STRING! (format, c"<Family %d>".as_ptr ());
      return CALLN! (Fformat, format, Fcar (address));
    }

  return Qnil ();
});

DEFUN! ("process-list", Fprocess_list, Sprocess_list, 0, 0, 0,
       doc: "Return a list of all processes that are Emacs sub-processes.",
       (),
       {
  return Fmapcar (Qcdr (), Vprocess_alist);
});

/* C 1800-1811: `get_required_string_keyword_param`.  */
unsafe fn get_required_string_keyword_param (kwargs: Lisp_Object,
					    keyword: Lisp_Object) -> Lisp_Object
{
  let arg: Lisp_Object = plist_member (kwargs, keyword);
  if (NILP (arg) || !CONSP (arg) || !CONSP (XCDR (arg)))
    { error (c"Missing %s keyword parameter".as_ptr (), SSDATA (SYMBOL_NAME (keyword))); }
  let val: Lisp_Object = XCAR (XCDR (arg));
  if (!STRINGP (val))
    { error (c"%s value not a string".as_ptr (), SSDATA (SYMBOL_NAME (keyword))); }
  return val;
}

unsafe extern "C" {
  /// C: Lisp_Object plist_get (Lisp_Object, Lisp_Object)   (lisp.h 4280)
  fn plist_get (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object;
  /// C: Lisp_Object plist_member (Lisp_Object, Lisp_Object)   (lisp.h 4279)
  fn plist_member (plist: Lisp_Object, prop: Lisp_Object) -> Lisp_Object;
}

/// <config.h 120>: `#define DIRECTORY_SEP '/'`.
const DIRECTORY_SEP: c_char = b'/' as c_char;
/// <config.h 2095>: `#define IS_DEVICE_SEP(_c_) 0` (the non-DOS value).
#[inline(always)] fn IS_DEVICE_SEP (_c_: c_char) -> bool { let _ = _c_; false }
/// <config.h 2097>: `#define IS_DIRECTORY_SEP(_c_) ((_c_) == DIRECTORY_SEP)`.
#[inline(always)] fn IS_DIRECTORY_SEP (_c_: c_char) -> bool { _c_ == DIRECTORY_SEP }

DEFUN! ("make-process", Fmake_process, Smake_process, 0, MANY, 0,
       doc: "Start a program in a subprocess.  Return the process object for it.\n\nThis is similar to `start-process', but arguments are specified as\nkeyword/argument pairs.  The following arguments are defined:\n\n:name NAME -- NAME is name for process.  It is modified if necessary\nto make it unique.\n\n:buffer BUFFER -- BUFFER is the buffer (or buffer-name) to associate\nwith the process.  Process output goes at end of that buffer, unless\nyou specify a filter function to handle the output.  BUFFER may be\nalso nil, meaning that this process is not associated with any buffer.\n\n:command COMMAND -- COMMAND is a list starting with the program file\nname, followed by strings to give to the program as arguments.  If the\nprogram file name is not an absolute file name, `make-process' will\nlook for the program file name in `exec-path' (which is a list of\ndirectories).\n\n:coding CODING -- If CODING is a symbol, it specifies the coding\nsystem used for both reading and writing for this process.  If CODING\nis a cons (DECODING . ENCODING), DECODING is used for reading, and\nENCODING is used for writing.\n\n:noquery BOOL -- When exiting Emacs, query the user if BOOL is nil and\nthe process is running.  If BOOL is not given, query before exiting.\n\n:stop BOOL -- BOOL must be nil.  The `:stop' key is ignored otherwise\nand is retained for compatibility with other process types such as\npipe processes.  Asynchronous subprocesses never start in the\n`stopped' state.  Use `stop-process' and `continue-process' to send\nsignals to stop and continue a process.\n\n:connection-type TYPE -- TYPE is control type of device used to\ncommunicate with subprocesses.  Values are `pipe' to use a pipe, `pty'\nto use a pty, or nil to use the default specified through\n`process-connection-type'.  If TYPE is a cons (INPUT . OUTPUT), then\nINPUT will be used for standard input and OUTPUT for standard output\n(and standard error if `:stderr' is nil).\n\n:filter FILTER -- Install FILTER as the process filter.\n\n:sentinel SENTINEL -- Install SENTINEL as the process sentinel.\n\n:stderr STDERR -- STDERR is either a buffer or a pipe process attached\nto the standard error of subprocess.  When specifying this, the\nsubprocess's standard error will always communicate via a pipe, no\nmatter the value of `:connection-type'.  If STDERR is nil, standard error\nis mixed with standard output and sent to BUFFER or FILTER.  (Note\nthat specifying :stderr will create a new, separate (but associated)\nprocess, with its own filter and sentinel.  See\nInfo node `(elisp) Asynchronous Processes' for more details.)\n\n:file-handler FILE-HANDLER -- If FILE-HANDLER is non-nil, then look\nfor a file name handler for the current buffer's `default-directory'\nand invoke that file name handler to make the process.  If there is no\nsuch handler, proceed as if FILE-HANDLER were nil.\n\nusage: (make-process &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut buffer: Lisp_Object;
  let mut command: Lisp_Object;
  let mut program: Lisp_Object;
  let mut proc: Lisp_Object;
  let mut contact: Lisp_Object;
  let mut current_dir: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut xstderr: Lisp_Object;
  let mut stderrproc: Lisp_Object;
  let count: specpdl_ref = SPECPDL_INDEX ();

  if (nargs == 0)
    { return Qnil (); }
  CHECK_KEYWORD_ARGS (nargs);

  /* Save arguments for process-contact and clone-process.  */
  contact = Flist (nargs, args);

  if (!NILP (plist_get (contact, QCfile_handler ())))
    {
      let file_handler: Lisp_Object
	= Ffind_file_name_handler (BVAR! ((current_buffer! ()), directory_),
				   Qmake_process ());
      if (!NILP (file_handler))
	{ return CALLN! (Fapply, file_handler, Qmake_process (), contact); }
    }

  buffer = plist_get (contact, QCbuffer ());
  if (!NILP (buffer))
    { buffer = Fget_buffer_create (buffer, Qnil ()); }

  /* Make sure that the child will be able to chdir to the current buffer's current
     directory, or its unhandled equivalent.  We can't just have the child check for an
     error when it does the chdir, since it's in a vfork.  */
  current_dir = get_current_directory (true);

  let mut name: Lisp_Object = get_required_string_keyword_param (contact, QCname ());

  command = plist_get (contact, QCcommand ());
  if (CONSP (command))
    { program = XCAR (command); }
  else
    { program = Qnil (); }

  if (!NILP (program))
    { CHECK_STRING (program); }

  let query_on_exit: bool = NILP (plist_get (contact, QCnoquery ()));

  stderrproc = Qnil ();
  xstderr = plist_get (contact, QCstderr ());
  if (PROCESSP (xstderr))
    {
      if (!PIPECONN_P (xstderr))
	{ error (c"Process is not a pipe process".as_ptr ()); }
      stderrproc = xstderr;
    }
  else if (!NILP (xstderr))
    {
      CHECK_STRING (program);
      stderrproc = CALLN! (Fmake_pipe_process,
			   QCname (),
			   concat2 (name, build_string (c" stderr".as_ptr ())),
			   QCbuffer (),
			   Fget_buffer_create (xstderr, Qnil ()),
			   QCnoquery (),
			   if query_on_exit { Qnil () } else { Qt () });
    }

  proc = make_process (name);
  record_unwind_protect (Some (start_process_unwind), proc);

  pset_childp (XPROCESS (proc), Qt ());
  eassert! (NILP ((*XPROCESS (proc)).plist));
  pset_type (XPROCESS (proc), Qreal ());
  pset_buffer (XPROCESS (proc), buffer);
  pset_sentinel (XPROCESS (proc), plist_get (contact, QCsentinel ()));
  pset_filter (XPROCESS (proc), plist_get (contact, QCfilter ()));
  pset_command (XPROCESS (proc), Fcopy_sequence (command));

  if (!query_on_exit)
    { process_bits! (kill_without_query, XPROCESS (proc), 1); }
  tem = plist_get (contact, QCstop ());
  /* Normal processes can't be started in a stopped state, see Bug#30460.  */
  CHECK_TYPE (NILP (tem), Qnull (), tem);

  tem = plist_get (contact, QCconnection_type ());
  if (CONSP (tem))
    {
      process_bits! (pty_in, XPROCESS (proc), is_pty_from_symbol (XCAR (tem)));
      process_bits! (pty_out, XPROCESS (proc), is_pty_from_symbol (XCDR (tem)));
    }
  else
    {
      let pty: bool = is_pty_from_symbol (tem);
      process_bits! (pty_in, XPROCESS (proc), pty);
      process_bits! (pty_out, XPROCESS (proc), pty);
    }

  if (!NILP (stderrproc))
    { pset_stderrproc (XPROCESS (proc), stderrproc); }

  /* C 1920-1925's `#ifdef HAVE_GNUTLS` arm.  */
  #[cfg (feature = "have_gnutls")]
  {
    /* AKA GNUTLS_INITSTAGE(proc).  */
    static_assert! (GNUTLS_STAGE_EMPTY == 0);
    eassert! ((*XPROCESS (proc)).gnutls_initstage == GNUTLS_STAGE_EMPTY);
    eassert! (NILP ((*XPROCESS (proc)).gnutls_cred_type));
  }

  process_bits! (adaptive_read_buffering, XPROCESS (proc),
		 (if NILP (Vprocess_adaptive_read_buffering) { 0 }
		  else if EQ (Vprocess_adaptive_read_buffering, Qt ()) { 1 } else { 2 }));

  /* Make the process marker point into the process buffer (if any).  */
  update_process_mark (XPROCESS (proc));

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);

  {
    /* Decide coding systems for communicating with the process.  Here we don't setup the
       structure coding_system nor pay attention to unibyte mode.  They are done in
       create_process.  */

    /* Qt denotes we have not yet called Ffind_operation_coding_system.  */
    let mut coding_systems: Lisp_Object = Qt ();
    let mut val: Lisp_Object;
    let mut args2: *mut Lisp_Object = std::ptr::null_mut ();

    tem = plist_get (contact, QCcoding ());
    if (!NILP (tem))
      {
	val = tem;
	if (CONSP (val))
	  { val = XCAR (val); }
      }
    else
      { val = Vcoding_system_for_read; }
    if (NILP (val))
      {
	let nargs2: ptrdiff_t = 3 + list_length (command);
	let mut tem2: Lisp_Object;
	SAFE_ALLOCA_LISP! (sa_avail, args2, nargs2);
	let mut i: ptrdiff_t = 0;
	*args2.offset (i) = Qstart_process (); i += 1;
	*args2.offset (i) = name; i += 1;
	*args2.offset (i) = buffer; i += 1;
	tem2 = command;
	while CONSP (tem2)
	  {
	    *args2.offset (i) = XCAR (tem2); i += 1;
	    tem2 = XCDR (tem2);
	  }
	if (!NILP (program))
	  { coding_systems = Ffind_operation_coding_system (nargs2, args2); }
	if (CONSP (coding_systems))
	  { val = XCAR (coding_systems); }
	else if (CONSP (Vdefault_process_coding_system))
	  { val = XCAR (Vdefault_process_coding_system); }
      }
    pset_decode_coding_system (XPROCESS (proc), val);

    if (!NILP (tem))
      {
	val = tem;
	if (CONSP (val))
	  { val = XCDR (val); }
      }
    else
      { val = Vcoding_system_for_write; }
    if (NILP (val))
      {
	if (EQ (coding_systems, Qt ()))
	  {
	    let nargs2: ptrdiff_t = 3 + list_length (command);
	    let mut tem2: Lisp_Object;
	    SAFE_ALLOCA_LISP! (sa_avail, args2, nargs2);
	    let mut i: ptrdiff_t = 0;
	    *args2.offset (i) = Qstart_process (); i += 1;
	    *args2.offset (i) = name; i += 1;
	    *args2.offset (i) = buffer; i += 1;
	    tem2 = command;
	    while CONSP (tem2)
	      {
		*args2.offset (i) = XCAR (tem2); i += 1;
		tem2 = XCDR (tem2);
	      }
	    if (!NILP (program))
	      { coding_systems = Ffind_operation_coding_system (nargs2, args2); }
	  }
	if (CONSP (coding_systems))
	  { val = XCDR (coding_systems); }
	else if (CONSP (Vdefault_process_coding_system))
	  { val = XCDR (Vdefault_process_coding_system); }
      }
    pset_encode_coding_system (XPROCESS (proc), val);
    /* Note: At this moment, the above coding system may leave text-conversion or
       eol-conversion unspecified.  They will be decided after we read output from the
       process and decode it by some coding system, or just before we actually send a text
       to the process.  */
  }

  pset_decoding_buf (XPROCESS (proc), empty_unibyte_string);
  eassert! ((*XPROCESS (proc)).decoding_carryover == 0);
  pset_encoding_buf (XPROCESS (proc), empty_unibyte_string);

  process_bits! (inherit_coding_system_flag, XPROCESS (proc),
		 !(NILP (buffer) || !inherit_process_coding_system));

  if (!NILP (program))
    {
      let program_args: Lisp_Object = XCDR (command);

      /* If program file name is not absolute, search our path for it.  Put the name we
	 will really use in TEM.  */
      if (!IS_DIRECTORY_SEP (SREF (program, 0) as c_char)
	  && !(SCHARS (program) > 1
	       && IS_DEVICE_SEP (SREF (program, 1) as c_char)))
	{
	  tem = Qnil ();
	  openp (Vexec_path, program, Fdefault_value (Qexec_suffixes ()), &raw mut tem,
		 make_fixnum (X_OK as EMACS_INT), false, false, std::ptr::null_mut ());
	  if (NILP (tem))
	    { report_file_error (c"Searching for program".as_ptr (), program); }
	  tem = Fexpand_file_name (tem, Qnil ());
	}
      else
	{
	  if (!NILP (Ffile_directory_p (program)))
	    { error (c"Specified program for new process is a directory".as_ptr ()); }
	  tem = program;
	}

      /* Remove "/:" from TEM.  */
      tem = remove_slash_colon (tem);

      let mut arg_encoding: Lisp_Object = Qnil ();

      /* Encode the file name and put it in NEW_ARGV.  That's where the child will use it
	 to execute the program.  */
      tem = list1 (ENCODE_FILE (tem));
      let mut new_argc: ptrdiff_t = 1;

      /* Here we encode arguments by the coding system used for sending data to the
	 process.  We don't support using different coding systems for encoding arguments and
	 for encoding data sent to the process.  */

      let mut tem2: Lisp_Object = program_args;
      while CONSP (tem2)
	{
	  let mut arg: Lisp_Object = XCAR (tem2);
	  CHECK_STRING (arg);
	  if (STRING_MULTIBYTE (arg))
	    {
	      if (NILP (arg_encoding))
		{ arg_encoding = complement_process_encoding_system ((*XPROCESS (proc)).encode_coding_system); }
	      arg = code_convert_string_norecord (arg, arg_encoding, true);
	    }
	  tem = Fcons (arg, tem);
	  new_argc += 1;
	  tem2 = XCDR (tem2);
	}

      /* Now that everything is encoded we can collect the strings into NEW_ARGV.  */
      let mut new_argv: *mut *mut c_char = std::ptr::null_mut ();
      SAFE_NALLOCA! (sa_avail, new_argv, 1, new_argc + 1);
      *new_argv.offset (new_argc) = std::ptr::null_mut ();

      let mut i: ptrdiff_t = new_argc - 1;
      while i >= 0
	{
	  *new_argv.offset (i) = SSDATA (XCAR (tem)) as *mut c_char;
	  tem = XCDR (tem);
	  i -= 1;
	}

      create_process (proc, new_argv, current_dir);
    }
  else
    { create_pty (proc); }

  return SAFE_FREE_UNBIND_TO! (sa_count, count, proc, sa_mark);
});

/* If PROC doesn't have its pid set, then an error was signaled and the process wasn't
   started successfully, so remove it.  */
/* C 2147-2153: `start_process_unwind` — its address is taken by `record_unwind_protect`
   above, so it is `unsafe extern "C" fn` (PORTING.md 1.4).  */
unsafe extern "C" fn start_process_unwind (proc_: Lisp_Object)
{
  if ((*XPROCESS (proc_)).pid <= 0 && (*XPROCESS (proc_)).pid != -2)
    { remove_process (proc_); }
}

/* If *FD_ADDR is nonnegative, close it, and mark it as closed.  */
/* C 2156-2165: `close_process_fd`.  */
unsafe fn close_process_fd (fd_addr: *mut c_int)
{
  let fd: c_int = *fd_addr;
  if (0 <= fd)
    {
      *fd_addr = -1;
      emacs_close (fd);
    }
}

/* C 2167-2184: `dissociate_controlling_tty`.  */
/// C: `#define TIOCNOTTY 536900721` (`<sys/ioctl.h>`, `_IO ('t', 113)`)
pub const TIOCNOTTY: c_int = 536900721;
#[unsafe(no_mangle)]
pub unsafe fn dissociate_controlling_tty ()
{
  if (setsid () < 0)
    {
      /* C 2171-2181's `#ifdef TIOCNOTTY` is **live** on the frozen configuration: C 71
	 includes `<sys/ioctl.h>`, so the C preprocessor keeps this body — preprocessing the real
	 file leaves `int ttyfd = emacs_open (dev_tty, 0x0002, 0);` and the
	 `ioctl (ttyfd, 0x20007471, 0)` that the macro expands to.  The condition has no `#else`
	 arm, so there is nothing to carry.  (The same `#ifdef` in `src/term.c` is the other way
	 round: term.c never includes `<sys/ioctl.h>`, so a *global*
	 `docs/baseline/config/source-header-macros.tsv` row cannot carry `TIOCNOTTY` — it would
	 make `src/term.c`'s condition disagree with clang.  See docs/ports/process.md, Decisions
	 and the Final state.)  */
      let ttyfd: c_int = emacs_open (dev_tty, O_RDWR, 0);
      if (0 <= ttyfd)
	{
	  ioctl (ttyfd, TIOCNOTTY as c_ulong, 0);
	  emacs_close (ttyfd);
	}
    }
}

/* C 2186-2204: the open_fd indexes; `static_assert (PROCESS_OPEN_FDS ==
   EXEC_MONITOR_OUTPUT + 1)`.  */
pub const SUBPROCESS_STDIN: c_int = 0;
pub const WRITE_TO_SUBPROCESS: c_int = 1;
pub const READ_FROM_SUBPROCESS: c_int = 2;
pub const SUBPROCESS_STDOUT: c_int = 3;
pub const READ_FROM_EXEC_MONITOR: c_int = 4;
pub const EXEC_MONITOR_OUTPUT: c_int = 5;
static_assert! (PROCESS_OPEN_FDS == EXEC_MONITOR_OUTPUT + 1);

/* C 2206-2386: `create_process`.  */
unsafe fn create_process (process: Lisp_Object, new_argv: *mut *mut c_char,
			  current_dir: Lisp_Object)
{
  let p: *mut Lisp_Process = XPROCESS (process);
  let mut inchannel: c_int = -1;
  let mut outchannel: c_int = -1;
  let mut pid: pid_t = -1;
  let mut vfork_errno: c_int;
  let mut forkin: c_int;
  let mut forkout: c_int;
  let mut forkerr: c_int = -1;
  let mut pty_in: bool = false;
  let mut pty_out: bool = false;
  let mut pty_name: [c_char; PTY_NAME_SIZE as usize] = [0; PTY_NAME_SIZE as usize];
  let mut lisp_pty_name: Lisp_Object = Qnil ();
  let mut ptychannel: c_int = -1;
  let mut pty_tty: c_int = -1;
  let mut oldset: sigset_t = std::mem::zeroed ();

  /* Ensure that the SIGCHLD handler can notify `wait_reading_process_output'.  */
  child_signal_init ();

  if (process_bits! (pty_in, p) || process_bits! (pty_out, p))
    { ptychannel = allocate_pty (pty_name.as_mut_ptr ()); }

  if (ptychannel >= 0)
    {
      /* `#if ! defined (USG) || defined (USG_SUBTTY_WORKS)` is live (both USG and
	 USG_SUBTTY_WORKS are undefined).  */
      /* On most USG systems it does not work to open the pty's tty here, then close it and
	 reopen it in the child.  */
      /* Don't let this terminal become our controlling terminal (in case we don't have
	 one).  */
      pty_tty = inrange_fd (emacs_open (pty_name.as_ptr (), O_RDWR | O_NOCTTY, 0));
      if (pty_tty < 0)
	{ report_file_error (c"Opening pty".as_ptr (), Qnil ()); }
      pty_in = process_bits! (pty_in, p);
      pty_out = process_bits! (pty_out, p);
      lisp_pty_name = build_string (pty_name.as_ptr ());
    }

  /* Set up stdin for the child process.  */
  if (ptychannel >= 0 && process_bits! (pty_in, p))
    {
      (*p).open_fd[SUBPROCESS_STDIN as usize] = pty_tty; forkin = pty_tty;
      outchannel = ptychannel;
    }
  else
    {
      if (inrange_pipe ((*p).open_fd.as_mut_ptr ().offset (SUBPROCESS_STDIN as isize)) < 0)
	{ report_file_error (c"Creating pipe".as_ptr (), Qnil ()); }
      forkin = (*p).open_fd[SUBPROCESS_STDIN as usize];
      outchannel = (*p).open_fd[WRITE_TO_SUBPROCESS as usize];
    }

  /* Set up stdout for the child process.  */
  if (ptychannel >= 0 && process_bits! (pty_out, p))
    {
      forkout = pty_tty;
      (*p).open_fd[READ_FROM_SUBPROCESS as usize] = ptychannel; inchannel = ptychannel;
    }
  else
    {
      if (inrange_pipe ((*p).open_fd.as_mut_ptr ().offset (READ_FROM_SUBPROCESS as isize)) < 0)
	{ report_file_error (c"Creating pipe".as_ptr (), Qnil ()); }
      inchannel = (*p).open_fd[READ_FROM_SUBPROCESS as usize];
      forkout = (*p).open_fd[SUBPROCESS_STDOUT as usize];

      /* C 2269-2273's `#if defined(F_SETPIPE_SZ) && defined(F_GETPIPE_SZ)` arm is carried
	 text (feature `have_pipe_sz`); neither name is on this target.  */
      #[cfg (feature = "have_pipe_sz")]
      {
	if ((*p).readmax > fcntl (inchannel, F_GETPIPE_SZ))
	  { fcntl (inchannel, F_SETPIPE_SZ, (*p).readmax); }
      }
    }

  if (!NILP ((*p).stderrproc))
    {
      let pp: *mut Lisp_Process = XPROCESS ((*p).stderrproc);

      forkerr = (*pp).open_fd[SUBPROCESS_STDOUT as usize];

      /* Close unnecessary file descriptors.  */
      close_process_fd ((*pp).open_fd.as_mut_ptr ().offset (WRITE_TO_SUBPROCESS as isize));
      close_process_fd ((*pp).open_fd.as_mut_ptr ().offset (SUBPROCESS_STDIN as isize));
    }

  /* `#ifndef WINDOWSNT` is live.  */
  if (inrange_pipe ((*p).open_fd.as_mut_ptr ().offset (READ_FROM_EXEC_MONITOR as isize)) < 0)
    { report_file_error (c"Creating pipe".as_ptr (), Qnil ()); }

  fcntl (inchannel, F_SETFL, O_NONBLOCK);
  fcntl (outchannel, F_SETFL, O_NONBLOCK);

  /* Record this as an active process, with its channels.  */
  eassert! (0 <= inchannel && inchannel < FD_SETSIZE);
  chan_process[inchannel as usize] = process;
  (*p).infd = inchannel;
  (*p).outfd = outchannel;

  /* Previously we recorded the tty descriptor used in the subprocess.  It was only used
     for getting the foreground tty process, so now we just reopen the device (see
     emacs_get_tty_pgrp) as this is more portable (see USG_SUBTTY_WORKS above).  */

  process_bits! (pty_in, p, pty_in);
  process_bits! (pty_out, p, pty_out);
  pset_status (p, Qrun ());

  if (!EQ ((*p).command, Qt ())
      && !EQ ((*p).filter, Qt ()))
    { add_process_read_fd (inchannel); }

  set_proc_thread (p, current_thread);

  let count: specpdl_ref = SPECPDL_INDEX ();

  /* This may signal an error.  */
  setup_process_coding_systems (process);
  let env: *mut *mut c_char = make_environment_block (current_dir);

  block_input ();
  block_child_signal (&raw mut oldset);

  pty_in = process_bits! (pty_in, p);
  pty_out = process_bits! (pty_out, p);
  eassert! ((pty_in || pty_out) == ! NILP (lisp_pty_name));

  vfork_errno
    = emacs_spawn (&raw mut pid, forkin, forkout, forkerr, new_argv, env,
		   SSDATA (current_dir) as *mut c_char,
		   if pty_in || pty_out { SSDATA (lisp_pty_name) as *mut c_char }
		   else { std::ptr::null_mut () },
		   pty_in, pty_out, &raw mut oldset);

  eassert! ((vfork_errno == 0) == (0 < pid));

  (*p).pid = pid;
  if (pid >= 0)
    { process_bits! (alive, p, 1); }

  /* Stop blocking in the parent.  */
  unblock_child_signal (&raw const oldset);
  unblock_input ();

  /* Environment block no longer needed.  */
  unbind_to (count, Qnil ());

  if (pid < 0)
    { report_file_errno (CHILD_SETUP_ERROR_DESC.as_ptr () as *const c_char, Qnil (), vfork_errno); }
  else
    {
      /* vfork succeeded.  */

      /* Close the pipe ends that the child uses, or the child's pty.  */
      close_process_fd ((*p).open_fd.as_mut_ptr ().offset (SUBPROCESS_STDIN as isize));
      close_process_fd ((*p).open_fd.as_mut_ptr ().offset (SUBPROCESS_STDOUT as isize));

      /* C 2359-2361's `#ifdef WINDOWSNT` arm.  */
      #[cfg (feature = "windowsnt")]
      {
	register_child (pid, inchannel);
      }

      pset_tty_name (p, lisp_pty_name);

      /* `#ifndef WINDOWSNT` is live: wait for child_setup to complete in case that vfork is
	 actually defined as fork.  */
      {
	let mut dummy: c_char = 0;

	close_process_fd ((*p).open_fd.as_mut_ptr ().offset (EXEC_MONITOR_OUTPUT as isize));
	emacs_read ((*p).open_fd[READ_FROM_EXEC_MONITOR as usize], &raw mut dummy as *mut c_void, 1);
	close_process_fd ((*p).open_fd.as_mut_ptr ().offset (READ_FROM_EXEC_MONITOR as isize));
      }
      if (!NILP ((*p).stderrproc))
	{
	  let pp: *mut Lisp_Process = XPROCESS ((*p).stderrproc);
	  close_process_fd ((*pp).open_fd.as_mut_ptr ().offset (SUBPROCESS_STDOUT as isize));
	}
    }
}

/* C 2388-2440: `create_pty`.  */
unsafe fn create_pty (process: Lisp_Object)
{
  let p: *mut Lisp_Process = XPROCESS (process);
  let mut pty_name: [c_char; PTY_NAME_SIZE as usize] = [0; PTY_NAME_SIZE as usize];
  let pty_fd: c_int = if !(process_bits! (pty_in, p) || process_bits! (pty_out, p))
    { -1 } else { allocate_pty (pty_name.as_mut_ptr ()) };

  if (pty_fd >= 0)
    {
      (*p).open_fd[SUBPROCESS_STDIN as usize] = pty_fd;
      /* `#if ! defined (USG) || defined (USG_SUBTTY_WORKS)` is live.  */
      {
	/* On most USG systems it does not work to open the pty's tty here, then close it
	   and reopen it in the child.  */
	/* Don't let this terminal become our controlling terminal (in case we don't have
	   one).  */
	let forkout: c_int = inrange_fd (emacs_open (pty_name.as_ptr (), O_RDWR | O_NOCTTY, 0));
	if (forkout < 0)
	  { report_file_error (c"Opening pty".as_ptr (), Qnil ()); }
	(*p).open_fd[WRITE_TO_SUBPROCESS as usize] = forkout;
	/* C 2406-2411's `#if defined (DONT_REOPEN_PTY)` arm is carried text (feature
	   `dont_reopen_pty`); `child_setup_tty` is sysdep/sysdep.c's.  */
	#[cfg (feature = "dont_reopen_pty")]
	{ child_setup_tty (forkout); }
      }

      fcntl (pty_fd, F_SETFL, O_NONBLOCK);

      /* Record this as an active process, with its channels.  As a result, child_setup
	 will close Emacs's side of the pipes.  */
      eassert! (0 <= pty_fd && pty_fd < FD_SETSIZE);
      chan_process[pty_fd as usize] = process;
      (*p).infd = pty_fd;
      (*p).outfd = pty_fd;

      /* Previously we recorded the tty descriptor used in the subprocess.  It was only
	 used for getting the foreground tty process, so now we just reopen the device (see
	 emacs_get_tty_pgrp) as this is more portable (see USG_SUBTTY_WORKS above).  */

      process_bits! (pty_in, p, true);
      process_bits! (pty_out, p, true);
      pset_status (p, Qrun ());
      setup_process_coding_systems (process);

      if (!EQ ((*p).filter, Qt ()))
	{ add_process_read_fd (pty_fd); }

      pset_tty_name (p, build_string (pty_name.as_ptr ()));
    }

  (*p).pid = -2;
}

/* ------------------------------------------------------------------------------------------
   Use-site declarations, group 4 (C 1813-2440, make-process/create_process).  */

unsafe extern "C" {
  /// C: Lisp_Object get_current_directory (bool)   (callproc.c 158, process.h 280's sibling)
  fn get_current_directory (encode: bool) -> Lisp_Object;
  /// C: int emacs_spawn (pid_t *, int, int, int, char **, char **, const char *,
  ///                     const char *, bool, bool, const sigset_t *)   (callproc.c 1441)
  fn emacs_spawn (pid: *mut pid_t, std_in: c_int, std_out: c_int, std_err: c_int,
                  argv: *mut *mut c_char, envp: *mut *mut c_char, cwd: *mut c_char,
                  ptyname: *mut c_char, pty_in: bool, pty_out: bool,
                  oldset: *mut sigset_t) -> c_int;
  /// C: char **make_environment_block (Lisp_Object)   (callproc.c 1816)
  fn make_environment_block (current_dir: Lisp_Object) -> *mut *mut c_char;
  /// C: int openp (Lisp_Object, Lisp_Object, Lisp_Object, Lisp_Object *, Lisp_Object,
  ///               bool, bool, void **)   (lisp.h 4743, lread.c)
  fn openp (path: Lisp_Object, str_: Lisp_Object, suffixes: Lisp_Object,
            storeptr: *mut Lisp_Object, predicate: Lisp_Object, newer: bool,
            no_native: bool, platform: *mut *mut c_void) -> c_int;
  /// C: pid_t setsid (void)   (<unistd.h>)
  fn setsid () -> pid_t;
  /// C: int ioctl (int, unsigned long, ...)   (<sys/ioctl.h>)
  fn ioctl (fd: c_int, request: c_ulong, ...) -> c_int;
  /// C: void child_setup_tty (int)   (lisp.h 5294, sysdep.c)
  fn child_setup_tty (out: c_int);
}

unsafe extern "C" {
  /// C: Lisp_Object complement_process_encoding_system (Lisp_Object)   (coding.h 703)
  fn complement_process_encoding_system (coding: Lisp_Object) -> Lisp_Object;
}

DEFUN! ("make-pipe-process", Fmake_pipe_process, Smake_pipe_process, 0, MANY, 0,
       doc: "Create and return a bidirectional pipe process.\n\nIn Emacs, pipes are represented by process objects, so input and\noutput work as for subprocesses, and `delete-process' closes a pipe.\nHowever, a pipe process has no process id, it cannot be signaled,\nand the status codes are different from normal processes.\n\nArguments are specified as keyword/argument pairs.  The following\narguments are defined:\n\n:name NAME -- NAME is the name of the process.  It is modified if necessary to make it unique.\n\n:buffer BUFFER -- BUFFER is the buffer (or buffer-name) to associate\nwith the process.  Process output goes at the end of that buffer,\nunless you specify a filter function to handle the output.  If BUFFER\nis not given, the value of NAME is used.  BUFFER may be also nil, meaning\nthat this process is not associated with any buffer.\n\n:coding CODING -- If CODING is a symbol, it specifies the coding\nsystem used for both reading and writing for this process.  If CODING\nis a cons (DECODING . ENCODING), DECODING is used for reading, and\nENCODING is used for writing.\n\n:noquery BOOL -- When exiting Emacs, query the user if BOOL is nil and\nthe process is running.  If BOOL is not given, query before exiting.\n\n:stop BOOL -- Start process in the `stopped' state if BOOL non-nil.\nIn the stopped state, a pipe process does not accept incoming data,\nbut you can send outgoing data.  The stopped state is cleared by\n`continue-process' and set by `stop-process'.\n\n:filter FILTER -- Install FILTER as the process filter.\n\n:sentinel SENTINEL -- Install SENTINEL as the process sentinel.\n\nusage:  (make-pipe-process &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut proc: Lisp_Object;
  let mut contact: Lisp_Object;
  let p: *mut Lisp_Process;
  let mut buffer: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut inchannel: c_int;
  let mut outchannel: c_int;

  if (nargs == 0)
    { return Qnil (); }
  CHECK_KEYWORD_ARGS (nargs);

  contact = Flist (nargs, args);

  let name: Lisp_Object = get_required_string_keyword_param (contact, QCname ());
  proc = make_process (name);
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect (Some (remove_process), proc);
  p = XPROCESS (proc);

  if (inrange_pipe ((*p).open_fd.as_mut_ptr ().offset (SUBPROCESS_STDIN as isize)) < 0
      || inrange_pipe ((*p).open_fd.as_mut_ptr ().offset (READ_FROM_SUBPROCESS as isize)) < 0)
    { report_file_error (c"Creating pipe".as_ptr (), Qnil ()); }
  outchannel = (*p).open_fd[WRITE_TO_SUBPROCESS as usize];
  inchannel = (*p).open_fd[READ_FROM_SUBPROCESS as usize];
  fcntl (inchannel, F_SETFL, O_NONBLOCK);
  fcntl (outchannel, F_SETFL, O_NONBLOCK);

  /* C 2507-2509's `#ifdef WINDOWSNT` arm.  */
  #[cfg (feature = "windowsnt")]
  {
    register_aux_fd (inchannel);
  }

  /* Record this as an active process, with its channels.  */
  eassert! (0 <= inchannel && inchannel < FD_SETSIZE);
  chan_process[inchannel as usize] = proc;
  (*p).infd = inchannel;
  (*p).outfd = outchannel;

  if (inchannel > max_desc)
    { max_desc = inchannel; }

  {
    let buffer_member: Lisp_Object = plist_member (contact, QCbuffer ());
    if (NILP (buffer_member))
      { buffer = name; }
    else
      { buffer = XCAR (XCDR (buffer_member)); }
  }
  if (!NILP (buffer))
    { buffer = Fget_buffer_create (buffer, Qnil ()); }
  pset_buffer (p, buffer);

  pset_childp (p, contact);
  pset_plist (p, Fcopy_sequence (plist_get (contact, QCplist ())));
  pset_type (p, Qpipe ());
  pset_sentinel (p, plist_get (contact, QCsentinel ()));
  pset_filter (p, plist_get (contact, QCfilter ()));
  eassert! (NILP ((*p).log));
  tem = plist_get (contact, QCnoquery ());
  if !NILP (tem)
    { process_bits! (kill_without_query, p, 1); }
  tem = plist_get (contact, QCstop ());
  if !NILP (tem)
    { pset_command (p, Qt ()); }
  eassert! (! process_bits! (pty_in, p) && ! process_bits! (pty_out, p));

  if (!EQ ((*p).command, Qt ())
      && !EQ ((*p).filter, Qt ()))
    { add_process_read_fd (inchannel); }
  process_bits! (adaptive_read_buffering, p,
		 (if NILP (Vprocess_adaptive_read_buffering) { 0 }
		  else if EQ (Vprocess_adaptive_read_buffering, Qt ()) { 1 } else { 2 }));

  /* Make the process marker point into the process buffer (if any).  */
  update_process_mark (p);

  {
    /* Setup coding systems for communicating with the network stream.  */

    /* Qt denotes we have not yet called Ffind_operation_coding_system.  */
    let coding_systems: Lisp_Object = Qt ();
    let mut val: Lisp_Object;

    tem = plist_get (contact, QCcoding ());
    val = Qnil ();
    if (!NILP (tem))
      {
	val = tem;
	if (CONSP (val))
	  { val = XCAR (val); }
      }
    else if (!NILP (Vcoding_system_for_read))
      { val = Vcoding_system_for_read; }
    else if ((!NILP (buffer) && NILP (BVAR! (XBUFFER (buffer), enable_multibyte_characters_)))
	     || (NILP (buffer) && NILP (BVAR! (&raw mut buffer_defaults, enable_multibyte_characters_))))
      /* We dare not decode end-of-line format by setting VAL to Qraw_text, because the
	 existing Emacs Lisp libraries assume that they receive bare code including a
	 sequence of CR LF.  */
      { val = Qnil (); }
    else
      {
	if (CONSP (coding_systems))
	  { val = XCAR (coding_systems); }
	else if (CONSP (Vdefault_process_coding_system))
	  { val = XCAR (Vdefault_process_coding_system); }
	else
	  { val = Qnil (); }
      }
    pset_decode_coding_system (p, val);

    if (!NILP (tem))
      {
	val = tem;
	if (CONSP (val))
	  { val = XCDR (val); }
      }
    else if (!NILP (Vcoding_system_for_write))
      { val = Vcoding_system_for_write; }
    else if (NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)))
      { val = Qnil (); }
    else
      {
	if (CONSP (coding_systems))
	  { val = XCDR (coding_systems); }
	else if (CONSP (Vdefault_process_coding_system))
	  { val = XCDR (Vdefault_process_coding_system); }
	else
	  { val = Qnil (); }
      }
    pset_encode_coding_system (p, val);
  }
  /* This may signal an error.  */
  setup_process_coding_systems (proc);

  pset_decoding_buf (p, empty_unibyte_string);
  eassert! ((*p).decoding_carryover == 0);
  pset_encoding_buf (p, empty_unibyte_string);

  (specpdl_ptr! ()) = specpdl_ref_to_ptr (specpdl_count);

  return proc;
});

/* Convert an internal struct sockaddr to a lisp object (vector or string).  The address
   family of sa is not included in the result.  */
/* C 2629-2695: `conv_sockaddr_to_lisp` (non-static in C).  */
#[unsafe(no_mangle)]
pub unsafe fn conv_sockaddr_to_lisp (sa: *mut sockaddr, len: ptrdiff_t) -> Lisp_Object
{
  let mut address: Lisp_Object;
  let mut cp: *mut u8;
  let mut p: *mut Lisp_Vector;

  /* Workaround for a bug in getsockname on BSD: Names bound to sockets in the UNIX domain
     are inaccessible; getsockname returns a zero length name.  */
  let mut len: ptrdiff_t = len;
  if (len < std::mem::offset_of! (sockaddr, sa_family) as ptrdiff_t + std::mem::size_of_val (&(*sa).sa_family) as ptrdiff_t)
    { return empty_unibyte_string; }

  match (*sa).sa_family as c_uint
    {
    x if x == AF_INET as c_uint =>
      {
	/* DECLARE_POINTER_ALIAS (sin, struct sockaddr_in, sa);  */
	let sin: *mut sockaddr_in = sa as *mut sockaddr_in;
	len = std::mem::size_of_val (&(*sin).sin_addr) as ptrdiff_t + 1;
	address = make_uninit_vector (len);
	p = XVECTOR (address);
	len -= 1;
	FAM_ELEM! ((*p).contents, len) = make_fixnum (ntohs ((*sin).sin_port as u16) as EMACS_INT);
	cp = &raw mut (*sin).sin_addr as *mut u8;
      }
    x if x == AF_INET6 as c_uint =>
      {
	/* DECLARE_POINTER_ALIAS (sin6, struct sockaddr_in6, sa);
	   DECLARE_POINTER_ALIAS (ip6, uint16_t, &sin6->sin6_addr);  */
	let sin6: *mut sockaddr_in6 = sa as *mut sockaddr_in6;
	let ip6: *mut u16 = &raw mut (*sin6).sin6_addr as *mut u16;
	len = (std::mem::size_of_val (&(*sin6).sin6_addr) / 2) as ptrdiff_t + 1;
	address = make_uninit_vector (len);
	p = XVECTOR (address);
	len -= 1;
	FAM_ELEM! ((*p).contents, len) = make_fixnum (ntohs ((*sin6).sin6_port as u16) as EMACS_INT);
	let mut i: ptrdiff_t = 0;
	while i < len
	  {
	    FAM_ELEM! ((*p).contents, i) = make_fixnum (ntohs (*ip6.offset (i as isize)) as EMACS_INT);
	    i += 1;
	  }
	return address;
      }
    /* `#ifdef HAVE_LOCAL_SOCKETS` is live.  */
    x if x == AF_LOCAL as c_uint =>
      {
	/* DECLARE_POINTER_ALIAS (sockun, struct sockaddr_un, sa);  */
	let sockun: *mut sockaddr_un = sa as *mut sockaddr_un;
	let mut name_length: ptrdiff_t
	  = len - std::mem::offset_of! (sockaddr_un, sun_path) as ptrdiff_t;
	/* If the first byte is NUL, the name is a Linux abstract socket name, and the name
	   can contain embedded NULs.  If it's not, we have a NUL-terminated string.  Be
	   careful not to walk past the end of the object looking for the name terminator,
	   however.  */
	if (name_length > 0 && (*sockun).sun_path[0] as c_int != 0)
	  { name_length = strnlen ((*sockun).sun_path.as_mut_ptr (), name_length as usize) as ptrdiff_t; }

	return make_unibyte_string ((*sockun).sun_path.as_mut_ptr (), name_length);
      }
    _ =>
      {
	len -= std::mem::offset_of! (sockaddr, sa_family) as ptrdiff_t + std::mem::size_of_val (&(*sa).sa_family) as ptrdiff_t;
	address = Fcons (make_fixnum ((*sa).sa_family as EMACS_INT), make_nil_vector (len));
	p = XVECTOR (XCDR (address));
	cp = (&raw mut (*sa).sa_family as *mut u8).offset (std::mem::size_of_val (&(*sa).sa_family) as isize);
      }
    }

  let mut i: ptrdiff_t = 0;
  while i < len
    {
      FAM_ELEM! ((*p).contents, i) = make_fixnum (*cp as EMACS_INT);
      cp = cp.offset (1);
      i += 1;
    }

  return address;
}

/* Convert an internal struct addrinfo to a Lisp object.  */
/* C 2697-2706: `conv_addrinfo_to_lisp`.  */
unsafe fn conv_addrinfo_to_lisp (res: *mut addrinfo) -> Lisp_Object
{
  let protocol: Lisp_Object = make_fixnum ((*res).ai_protocol as EMACS_INT);
  eassert! (XFIXNUM (protocol) == (*res).ai_protocol as EMACS_INT);
  return Fcons (protocol, conv_sockaddr_to_lisp ((*res).ai_addr, (*res).ai_addrlen as ptrdiff_t));
}

/* Get family and required size for sockaddr structure to hold ADDRESS.  */
/* C 2708-2754: `get_lisp_to_sockaddr_size`.  */
unsafe fn get_lisp_to_sockaddr_size (address: Lisp_Object, familyp: *mut c_int) -> ptrdiff_t
{
  let mut p: *mut Lisp_Vector;

  if (VECTORP (address))
    {
      p = XVECTOR (address);
      if ((*p).header.size == 5)
	{
	  *familyp = AF_INET;
	  return std::mem::size_of::<sockaddr_in> () as ptrdiff_t;
	}
      else if ((*p).header.size == 9)
	{
	  *familyp = AF_INET6;
	  return std::mem::size_of::<sockaddr_in6> () as ptrdiff_t;
	}
    }
  /* `#ifdef HAVE_LOCAL_SOCKETS` is live.  */
  else if (STRINGP (address))
    {
      *familyp = AF_LOCAL;
      return std::mem::size_of::<sockaddr_un> () as ptrdiff_t;
    }
  else if (CONSP (address) && TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, XCAR (address), INT_MAX as EMACS_INT)
	   && VECTORP (XCDR (address)))
    {
      p = XVECTOR (XCDR (address));
      /* C declares `struct sockaddr *sa;` only to write `sizeof sa->sa_family`; C's `sizeof`
	 never evaluates its operand, so the port asks a zeroed temporary for the field's width
	 instead of dereferencing a pointer C leaves uninitialized (PORTING.md 1.5).  */
      let sa_family_size: usize
	= std::mem::size_of_val (&std::mem::zeroed::<sockaddr> ().sa_family);
      if ((MAX_ALLOCA - sa_family_size as isize) < (*p).header.size as isize)
	{ return 0; }
      *familyp = XFIXNUM (XCAR (address)) as c_int;
      return (*p).header.size as ptrdiff_t + sa_family_size as ptrdiff_t;
    }
  return 0;
}

/* Convert an address object (vector or string) to an internal sockaddr.  The address format
   has been basically validated by get_lisp_to_sockaddr_size, but this does not mean FAMILY is
   valid; it could have come from user data.  So if FAMILY is not valid, we return after
   zeroing *SA.  */
/* C 2761-2820: `conv_lisp_to_sockaddr`.  */
unsafe fn conv_lisp_to_sockaddr (family: c_int, address: Lisp_Object, sa: *mut sockaddr,
				 len: c_int)
{
  let mut p: *mut Lisp_Vector;
  let mut cp: *mut u8 = std::ptr::null_mut ();
  let mut i: c_int;
  let mut hostport: EMACS_INT;
  let mut len: c_int = len;

  memset (sa as *mut c_void, 0, len as usize);

  if (VECTORP (address))
    {
      p = XVECTOR (address);
      if (family == AF_INET)
	{
	  /* DECLARE_POINTER_ALIAS (sin, struct sockaddr_in, sa);  */
	  let sin: *mut sockaddr_in = sa as *mut sockaddr_in;
	  len = std::mem::size_of_val (&(*sin).sin_addr) as c_int + 1;
	  len -= 1;
	  hostport = XFIXNUM (FAM_ELEM! ((*p).contents, len));
	  (*sin).sin_port = htons (hostport as u16);
	  cp = &raw mut (*sin).sin_addr as *mut u8;
	  (*sa).sa_family = family as u8;
	}
      else if (family == AF_INET6)
	{
	  /* DECLARE_POINTER_ALIAS (sin6, struct sockaddr_in6, sa);
	     DECLARE_POINTER_ALIAS (ip6, uint16_t, &sin6->sin6_addr);  */
	  let sin6: *mut sockaddr_in6 = sa as *mut sockaddr_in6;
	  let ip6: *mut u16 = &raw mut (*sin6).sin6_addr as *mut u16;
	  len = (std::mem::size_of_val (&(*sin6).sin6_addr) / 2) as c_int + 1;
	  len -= 1;
	  hostport = XFIXNUM (FAM_ELEM! ((*p).contents, len));
	  (*sin6).sin6_port = htons (hostport as u16);
	  i = 0;
	  while i < len
	    {
	      if (FIXNUMP (FAM_ELEM! ((*p).contents, i)))
		{
		  let j: c_int = XFIXNUM (FAM_ELEM! ((*p).contents, i)) as c_int & 0xffff;
		  *ip6.offset (i as isize) = ntohs (j as u16);
		}
	      i += 1;
	    }
	  (*sa).sa_family = family as u8;
	  return;
	}
      else
	{ return; }
    }
  else if (STRINGP (address))
    {
      /* `#ifdef HAVE_LOCAL_SOCKETS` is live.  */
      if (family == AF_LOCAL)
	{
	  /* DECLARE_POINTER_ALIAS (sockun, struct sockaddr_un, sa);  */
	  let sockun: *mut sockaddr_un = sa as *mut sockaddr_un;
	  cp = SDATA (address);
	  i = 0;
	  while i < std::mem::size_of_val (&(*sockun).sun_path) as c_int && *cp as c_int != 0
	    {
	      (*sockun).sun_path[i as usize] = *cp as c_char;
	      cp = cp.offset (1);
	      i += 1;
	    }
	  (*sa).sa_family = family as u8;
	}
      return;
    }
  else
    {
      p = XVECTOR (XCDR (address));
      cp = (sa as *mut u8).offset (std::mem::size_of_val (&(*sa).sa_family) as isize);
    }

  i = 0;
  while i < len
    {
      if (FIXNUMP (FAM_ELEM! ((*p).contents, i)))
	{ *cp = (XFIXNAT (FAM_ELEM! ((*p).contents, i)) & 0xff) as u8; cp = cp.offset (1); }
      i += 1;
    }
}

/* `#ifdef DATAGRAM_SOCKETS` is live.  */
DEFUN! ("process-datagram-address", Fprocess_datagram_address, Sprocess_datagram_address,
       1, 1, 0,
       doc: "Get the current datagram address associated with PROCESS.\nIf PROCESS is a non-blocking network process that hasn't been fully\nset up yet, this function will block until socket setup has completed.",
       (process: Lisp_Object),
       {
  let channel: c_int;

  CHECK_PROCESS (process);

  if (NETCONN_P (process))
    { wait_for_socket_fds (process, c"process-datagram-address".as_ptr ()); }

  if (!DATAGRAM_CONN_P (process))
    { return Qnil (); }

  channel = (*XPROCESS (process)).infd;
  eassert! (0 <= channel && channel < FD_SETSIZE);
  return conv_sockaddr_to_lisp (datagram_address[channel as usize].sa,
				datagram_address[channel as usize].len);
});

DEFUN! ("set-process-datagram-address", Fset_process_datagram_address,
       Sset_process_datagram_address, 2, 2, 0,
       doc: "Set the datagram address for PROCESS to ADDRESS.\nReturn nil upon error setting address, ADDRESS otherwise.\n\nIf PROCESS is a non-blocking network process that hasn't been fully\nset up yet, this function will block until socket setup has completed.",
       (process: Lisp_Object, address: Lisp_Object),
       {
  let channel: c_int;
  let mut family: c_int = 0;
  let len: ptrdiff_t;

  CHECK_PROCESS (process);

  if (NETCONN_P (process))
    { wait_for_socket_fds (process, c"set-process-datagram-address".as_ptr ()); }

  if (!DATAGRAM_CONN_P (process))
    { return Qnil (); }

  channel = (*XPROCESS (process)).infd;

  len = get_lisp_to_sockaddr_size (address, &raw mut family);
  eassert! (0 <= channel && channel < FD_SETSIZE);
  if (len == 0 || datagram_address[channel as usize].len != len)
    { return Qnil (); }
  conv_lisp_to_sockaddr (family, address, datagram_address[channel as usize].sa, len as c_int);
  return address;
});

/* C 2880-2927: `static const struct socket_options { … } socket_options[]`.  The two dead
   arms (`SO_BINDTODEVICE`, `SO_PRIORITY`) are carried as cfg-gated constants below.  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct socket_options
{
  pub name: *const c_char,
  pub optlevel: c_int,
  pub optnum: c_int,
  pub opttype: c_int,
  pub optbit: c_int,
}
const _: () = core::assert! (std::mem::size_of::<socket_options> () == 24);
/* C 2885: `enum { SOPT_UNKNOWN, SOPT_BOOL, SOPT_INT, SOPT_IFNAME, SOPT_LINGER } opttype;`  */
pub const SOPT_UNKNOWN: c_int = 0;
pub const SOPT_BOOL: c_int = 1;
pub const SOPT_INT: c_int = 2;
pub const SOPT_IFNAME: c_int = 3;
pub const SOPT_LINGER: c_int = 4;
/* C 2886: `enum { OPIX_NONE = 0, OPIX_MISC = 1, OPIX_REUSEADDR = 2 } optbit;`  */
pub const OPIX_NONE: c_int = 0;
pub const OPIX_MISC: c_int = 1;
pub const OPIX_REUSEADDR: c_int = 2;
/* The live entries (the `#ifdef`s of `SO_BINDTODEVICE` and `SO_PRIORITY` are dead here).  */
static mut socket_options: [socket_options; 8] = [
  socket_options { name: c":broadcast".as_ptr (), optlevel: SOL_SOCKET, optnum: SO_BROADCAST, opttype: SOPT_BOOL, optbit: OPIX_MISC },
  socket_options { name: c":dontroute".as_ptr (), optlevel: SOL_SOCKET, optnum: SO_DONTROUTE, opttype: SOPT_BOOL, optbit: OPIX_MISC },
  socket_options { name: c":keepalive".as_ptr (), optlevel: SOL_SOCKET, optnum: SO_KEEPALIVE, opttype: SOPT_BOOL, optbit: OPIX_MISC },
  socket_options { name: c":linger".as_ptr (), optlevel: SOL_SOCKET, optnum: SO_LINGER, opttype: SOPT_LINGER, optbit: OPIX_MISC },
  socket_options { name: c":oobinline".as_ptr (), optlevel: SOL_SOCKET, optnum: SO_OOBINLINE, opttype: SOPT_BOOL, optbit: OPIX_MISC },
  socket_options { name: c":reuseaddr".as_ptr (), optlevel: SOL_SOCKET, optnum: SO_REUSEADDR, opttype: SOPT_BOOL, optbit: OPIX_REUSEADDR },
  socket_options { name: c":nodelay".as_ptr (), optlevel: IPPROTO_TCP, optnum: TCP_NODELAY, opttype: SOPT_BOOL, optbit: OPIX_MISC },
  socket_options { name: std::ptr::null (), optlevel: 0, optnum: 0, opttype: SOPT_UNKNOWN, optbit: OPIX_NONE },
];
/* C 2892-2894's `#ifdef SO_BINDTODEVICE` arm.  */
#[cfg (feature = "have_so_bindtodevice")]
const SOCKET_OPTIONS_BINDTODEVICE: socket_options =
  socket_options { name: c":bindtodevice".as_ptr (), optlevel: SOL_SOCKET,
		   optnum: SO_BINDTODEVICE, opttype: SOPT_IFNAME, optbit: OPIX_MISC };
/* C 2910-2912's `#ifdef SO_PRIORITY` arm.  */
#[cfg (feature = "have_so_priority")]
const SOCKET_OPTIONS_PRIORITY: socket_options =
  socket_options { name: c":priority".as_ptr (), optlevel: SOL_SOCKET,
		   optnum: SO_PRIORITY, opttype: SOPT_INT, optbit: OPIX_MISC };

/* Set option OPT to value VAL on socket S.  Return (1<<socket_options[OPT].optbit) if option
   is known, 0 otherwise.  Signals an error if setting a known option fails.  */
/* C 2929-3011: `set_socket_option`.  */
unsafe fn set_socket_option (s: c_int, opt: Lisp_Object, val: Lisp_Object) -> c_int
{
  let name: *mut c_char;
  let mut sopt: *const socket_options;
  let mut ret: c_int = 0;

  CHECK_SYMBOL (opt);

  name = SSDATA (SYMBOL_NAME (opt)) as *mut c_char;
  sopt = (&raw const socket_options) as *const socket_options;
  while !(*sopt).name.is_null ()
    {
      if (strcmp (name, (*sopt).name) == 0)
	{ break; }
      sopt = sopt.offset (1);
    }

  match (*sopt).opttype
    {
    SOPT_BOOL =>
      {
	let mut optval: c_int;
	optval = if NILP (val) { 0 } else { 1 };
	ret = setsockopt (s, (*sopt).optlevel, (*sopt).optnum,
			  &raw mut optval as *mut c_void, std::mem::size_of_val (&optval) as socklen_t);
      }
    SOPT_INT =>
      {
	let mut optval: c_int;
	if (TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, val, INT_MAX as EMACS_INT))
	  { optval = XFIXNUM (val) as c_int; }
	else
	  { error (c"Bad option value for %s".as_ptr (), name); }
	ret = setsockopt (s, (*sopt).optlevel, (*sopt).optnum,
			  &raw mut optval as *mut c_void, std::mem::size_of_val (&optval) as socklen_t);
      }
    /* C 2965-2982's `#ifdef SO_BINDTODEVICE` arm (feature `have_so_bindtodevice`).  */
    #[cfg (feature = "have_so_bindtodevice")]
    SOPT_IFNAME =>
      {
	let mut devname: [c_char; IFNAMSIZ as usize + 1] = [0; IFNAMSIZ as usize + 1];

	/* This is broken, at least in the Linux 2.4 kernel.  To unbind, the arg must be a
	   zero integer, not the empty string.  This should work on all systems.  */
	memset (devname.as_mut_ptr () as *mut c_void, 0, std::mem::size_of_val (&devname));
	if (STRINGP (val))
	  { memcpy (devname.as_mut_ptr () as *mut c_void, SDATA (val) as *const c_void,
		    std::cmp::min (SBYTES (val), IFNAMSIZ as ptrdiff_t) as usize); }
	else if (!NILP (val))
	  { error (c"Bad option value for %s".as_ptr (), name); }
	ret = setsockopt (s, (*sopt).optlevel, (*sopt).optnum,
			  devname.as_mut_ptr () as *mut c_void, IFNAMSIZ as socklen_t);
      }
    /* `#ifdef SO_LINGER` is live.  */
    SOPT_LINGER =>
      {
	let mut linger_: linger = linger { l_onoff: 0, l_linger: 0 };

	linger_.l_onoff = 1;
	linger_.l_linger = 0;
	if (TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, val, INT_MAX as EMACS_INT))
	  { linger_.l_linger = XFIXNUM (val) as c_int; }
	else
	  { linger_.l_onoff = if NILP (val) { 0 } else { 1 }; }
	ret = setsockopt (s, (*sopt).optlevel, (*sopt).optnum,
			  &raw mut linger_ as *mut c_void, std::mem::size_of_val (&linger_) as socklen_t);
      }
    _ => { return 0; }
    }

  if (ret < 0)
    {
      let setsockopt_errno: c_int = *errno ();
      report_file_errno (c"Cannot set network option".as_ptr (), list2 (opt, val),
			 setsockopt_errno);
    }

  return 1 << (*sopt).optbit;
}

DEFUN! ("set-network-process-option", Fset_network_process_option,
       Sset_network_process_option, 3, 4, 0,
       doc: "For network process PROCESS set option OPTION to value VALUE.\nSee `make-network-process' for a list of options and values.\nIf optional fourth arg NO-ERROR is non-nil, don't signal an error if\nOPTION is not a supported option, return nil instead; otherwise return t.\n\nIf PROCESS is a non-blocking network process that hasn't been fully\nset up yet, this function will block until socket setup has completed.",
       (process: Lisp_Object, option: Lisp_Object, value: Lisp_Object, no_error: Lisp_Object),
       {
  let s: c_int;
  let p: *mut Lisp_Process;

  CHECK_PROCESS (process);
  p = XPROCESS (process);
  if (!NETCONN1_P (p))
    { error (c"Process is not a network process".as_ptr ()); }

  wait_for_socket_fds (process, c"set-network-process-option".as_ptr ());

  s = (*p).infd;
  if (s < 0)
    { error (c"Process is not running".as_ptr ()); }

  if (set_socket_option (s, option, value) != 0)
    {
      pset_childp (p, plist_put ((*p).childp, option, value));
      return Qt ();
    }

  if (NILP (no_error))
    { error (c"Unknown or unsupported option".as_ptr ()); }

  return Qnil ();
});

/* ------------------------------------------------------------------------------------------
   Use-site declarations, group 5 (C 2626-3060, the socket helpers).  The `<sys/socket.h>`,
   `<netinet/in.h>`, `<netinet/tcp.h>` and `<arpa/inet.h>` constants are measured with the
   frozen configuration (the port's clang probe).  */

/// C: `#define AF_UNIX 1` (`<sys/socket.h>`)
pub const AF_UNIX: c_int = 1;
/// C: `#define AF_LOCAL 1` (`<sys/socket.h>`); C 63-64's `#if !defined (AF_LOCAL) &&
/// defined (AF_UNIX)` arm spells it `AF_UNIX`, so the port's constant does too.
pub const AF_LOCAL: c_int = AF_UNIX;
/// C: `#define AF_INET 2` (`<sys/socket.h>`)
pub const AF_INET: c_int = 2;
/// C: `#define AF_INET6 30` (`<sys/socket.h>`)
pub const AF_INET6: c_int = 30;
/// C: `#define SOL_SOCKET 65535` (`<sys/socket.h>`)
pub const SOL_SOCKET: c_int = 65535;
/// C: `#define SO_REUSEADDR 4` (`<sys/socket.h>`)
pub const SO_REUSEADDR: c_int = 4;
/// C: `#define SO_LINGER 128` (`<sys/socket.h>`)
pub const SO_LINGER: c_int = 128;
/// C: `#define SO_BROADCAST 32` (`<sys/socket.h>`)
pub const SO_BROADCAST: c_int = 32;
/// C: `#define SO_KEEPALIVE 8` (`<sys/socket.h>`)
pub const SO_KEEPALIVE: c_int = 8;
/// C: `#define SO_OOBINLINE 256` (`<sys/socket.h>`)
pub const SO_OOBINLINE: c_int = 256;
/// C: `#define SO_DONTROUTE 16` (`<sys/socket.h>`)
pub const SO_DONTROUTE: c_int = 16;
/// C: `#define IPPROTO_TCP 6` (`<netinet/in.h>`)
pub const IPPROTO_TCP: c_int = 6;
/// C: `#define TCP_NODELAY 1` (`<netinet/tcp.h>`)
pub const TCP_NODELAY: c_int = 1;
/// C: `#define IFNAMSIZ 16` (`<net/if.h>`)
pub const IFNAMSIZ: c_int = 16;

unsafe extern "C" {
  /// C: uint16_t ntohs (uint16_t)   (<arpa/inet.h>)
  fn ntohs (netshort: u16) -> u16;
  /// C: uint16_t htons (uint16_t)   (<arpa/inet.h>)
  fn htons (hostshort: u16) -> u16;
  /// C: int setsockopt (int, int, int, const void *, socklen_t)   (<sys/socket.h>)
  fn setsockopt (socket: c_int, level: c_int, option_name: c_int,
                 option_value: *mut c_void, option_len: socklen_t) -> c_int;
  /// C: size_t strnlen (const char *, size_t)   (<string.h>; the string.rs mirror lacks it)
  fn strnlen (s: *const c_char, maxlen: usize) -> usize;
}

DEFUN! ("serial-process-configure", Fserial_process_configure, Sserial_process_configure,
       0, MANY, 0,
       doc: "Configure speed, bytesize, etc. of a serial process.\n\nArguments are specified as keyword/argument pairs.  Attributes that\nare not given are re-initialized from the process's current\nconfiguration (available via the function `process-contact') or set to\nreasonable default values.  The following arguments are defined:\n\n:process PROCESS\n:name NAME\n:buffer BUFFER\n:port PORT\n-- Any of these arguments can be given to identify the process that is\nto be configured.  If none of these arguments is given, the current\nbuffer's process is used.\n\n:speed SPEED -- SPEED is the speed of the serial port in bits per\nsecond, also called baud rate.  Any value can be given for SPEED, but\nmost serial ports work only at a few defined values between 1200 and\n115200, with 9600 being the most common value.  If SPEED is nil, the\nserial port is not configured any further, i.e., all other arguments\nare ignored.  This may be useful for special serial ports such as\nBluetooth-to-serial converters which can only be configured through AT\ncommands.  A value of nil for SPEED can be used only when passed\nthrough `make-serial-process' or `serial-term'.\n\n:bytesize BYTESIZE -- BYTESIZE is the number of bits per byte, which\ncan be 7 or 8.  If BYTESIZE is not given or nil, a value of 8 is used.\n\n:parity PARITY -- PARITY can be nil (don't use parity), the symbol\n`odd' (use odd parity), or the symbol `even' (use even parity).  If\nPARITY is not given, no parity is used.\n\n:stopbits STOPBITS -- STOPBITS is the number of stopbits used to\nterminate a byte transmission.  STOPBITS can be 1 or 2.  If STOPBITS\nis not given or nil, 1 stopbit is used.\n\n:flowcontrol FLOWCONTROL -- FLOWCONTROL determines the type of\nflowcontrol to be used, which is either nil (don't use flowcontrol),\nthe symbol `hw' (use RTS/CTS hardware flowcontrol), or the symbol `sw'\n(use XON/XOFF software flowcontrol).  If FLOWCONTROL is not given, no\nflowcontrol is used.\n\n`serial-process-configure' is called by `make-serial-process' for the\ninitial configuration of the serial port.\n\nExamples:\n\n(serial-process-configure :process \"/dev/ttyS0\" :speed 1200)\n\n(serial-process-configure\n    :buffer \"COM1\" :stopbits 1 :parity \\='odd :flowcontrol \\='hw)\n\n(serial-process-configure :port \"\\\\.\\COM13\" :bytesize 7)\n\nusage: (serial-process-configure &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let p: *mut Lisp_Process;
  let mut contact: Lisp_Object = Qnil ();
  let mut proc: Lisp_Object = Qnil ();

  CHECK_KEYWORD_ARGS (nargs);

  contact = Flist (nargs, args);

  proc = plist_get (contact, QCprocess ());
  if (NILP (proc))
    { proc = plist_get (contact, QCname ()); }
  if (NILP (proc))
    { proc = plist_get (contact, QCbuffer ()); }
  if (NILP (proc))
    { proc = plist_get (contact, QCport ()); }
  proc = get_process (proc);
  p = XPROCESS (proc);
  if (!EQ ((*p).r#type, Qserial ()))
    { error (c"Not a serial process".as_ptr ()); }

  if (NILP (plist_get ((*p).childp, QCspeed ())))
    { return Qnil (); }

  serial_configure (p, contact);
  return Qnil ();
});

DEFUN! ("make-serial-process", Fmake_serial_process, Smake_serial_process, 0, MANY, 0,
       doc: "Create and return a serial port process.\n\nIn Emacs, serial port connections are represented by process objects,\nso input and output work as for subprocesses, and `delete-process'\ncloses a serial port connection.  However, a serial process has no\nprocess id, it cannot be signaled, and the status codes are different\nfrom normal processes.\n\n`make-serial-process' creates a process and a buffer, on which you\nprobably want to use `process-send-string'.  Try \\[serial-term] for\nan interactive terminal.  See below for examples.\n\nArguments are specified as keyword/argument pairs.  The following\narguments are defined:\n\n:port PORT -- (mandatory) PORT is the path or name of the serial port.\nFor example, this could be \"/dev/ttyS0\" on Unix.  On Windows, this\ncould be \"COM1\", or \"\\\\.\\COM10\" for ports higher than COM9 (double\nthe backslashes in strings).\n\n:speed SPEED -- (mandatory) is handled by `serial-process-configure',\nwhich this function calls.\n\n:name NAME -- NAME is the name of the process.  If NAME is not given,\nthe value of PORT is used.\n\n:buffer BUFFER -- BUFFER is the buffer (or buffer-name) to associate\nwith the process.  Process output goes at the end of that buffer,\nunless you specify a filter function to handle the output.  If BUFFER\nis not given, the value of NAME is used.\n\n:coding CODING -- If CODING is a symbol, it specifies the coding\nsystem used for both reading and writing for this process.  If CODING\nis a cons (DECODING . ENCODING), DECODING is used for reading, and\nENCODING is used for writing.\n\n:noquery BOOL -- When exiting Emacs, query the user if BOOL is nil and\nthe process is running.  If BOOL is not given, query before exiting.\n\n:stop BOOL -- Start process in the `stopped' state if BOOL is non-nil.\nIn the stopped state, a serial process does not accept incoming data,\nbut you can send outgoing data.  The stopped state is cleared by\n`continue-process' and set by `stop-process'.\n\n:filter FILTER -- Install FILTER as the process filter.\n\n:sentinel SENTINEL -- Install SENTINEL as the process sentinel.\n\n:plist PLIST -- Install PLIST as the initial plist of the process.\n\n:bytesize\n:parity\n:stopbits\n:flowcontrol\n-- This function calls `serial-process-configure' to handle these\narguments.\n\nThe original argument list, possibly modified by later configuration,\nis available via the function `process-contact'.\n\nExamples:\n\n(make-serial-process :port \"/dev/ttyS0\" :speed 9600)\n\n(make-serial-process :port \"COM1\" :speed 115200 :stopbits 2)\n\n(make-serial-process :port \"\\\\.\\COM13\" :speed 1200 :bytesize 7 :parity \\='odd)\n\n(make-serial-process :port \"/dev/tty.BlueConsole-SPP-1\" :speed nil)\n\nusage:  (make-serial-process &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut fd: c_int = -1;
  let mut proc: Lisp_Object;
  let mut contact: Lisp_Object;
  let mut port: Lisp_Object;
  let p: *mut Lisp_Process;
  let mut name: Lisp_Object;
  let mut buffer: Lisp_Object;
  let mut tem: Lisp_Object;
  let mut val: Lisp_Object;

  if (nargs == 0)
    { return Qnil (); }
  CHECK_KEYWORD_ARGS (nargs);

  contact = Flist (nargs, args);

  port = plist_get (contact, QCport ());
  if (NILP (port))
    { error (c"No port specified".as_ptr ()); }
  CHECK_STRING (port);

  if (NILP (plist_member (contact, QCspeed ())))
    { error (c":speed not specified".as_ptr ()); }
  if (!NILP (plist_get (contact, QCspeed ())))
    { CHECK_FIXNUM (plist_get (contact, QCspeed ())); }

  name = plist_get (contact, QCname ());
  if (NILP (name))
    { name = port; }
  CHECK_STRING (name);
  proc = make_process (name);
  let specpdl_count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect (Some (remove_process), proc);
  p = XPROCESS (proc);

  fd = inrange_fd (serial_open (port));
  if (fd < 0)
    { report_file_error (c"Opening serial port".as_ptr (), port); }
  (*p).open_fd[SUBPROCESS_STDIN as usize] = fd;
  (*p).infd = fd;
  (*p).outfd = fd;
  if (fd > max_desc)
    { max_desc = fd; }
  eassert! (0 <= fd && fd < FD_SETSIZE);
  chan_process[fd as usize] = proc;

  buffer = plist_get (contact, QCbuffer ());
  if (NILP (buffer))
    { buffer = name; }
  buffer = Fget_buffer_create (buffer, Qnil ());
  pset_buffer (p, buffer);

  pset_childp (p, contact);
  pset_plist (p, Fcopy_sequence (plist_get (contact, QCplist ())));
  pset_type (p, Qserial ());
  pset_sentinel (p, plist_get (contact, QCsentinel ()));
  pset_filter (p, plist_get (contact, QCfilter ()));
  eassert! (NILP ((*p).log));
  tem = plist_get (contact, QCnoquery ());
  if !NILP (tem)
    { process_bits! (kill_without_query, p, 1); }
  tem = plist_get (contact, QCstop ());
  if !NILP (tem)
    { pset_command (p, Qt ()); }
  eassert! (! process_bits! (pty_in, p) && ! process_bits! (pty_out, p));

  if (!EQ ((*p).command, Qt ())
      && !EQ ((*p).filter, Qt ()))
    { add_process_read_fd (fd); }

  update_process_mark (p);

  tem = plist_get (contact, QCcoding ());

  val = Qnil ();
  if (!NILP (tem))
    {
      val = tem;
      if (CONSP (val))
	{ val = XCAR (val); }
    }
  else if (!NILP (Vcoding_system_for_read))
    { val = Vcoding_system_for_read; }
  else if ((!NILP (buffer) && NILP (BVAR! (XBUFFER (buffer), enable_multibyte_characters_)))
	   || (NILP (buffer) && NILP (BVAR! (&raw mut buffer_defaults, enable_multibyte_characters_))))
    { val = Qnil (); }
  pset_decode_coding_system (p, val);

  val = Qnil ();
  if (!NILP (tem))
    {
      val = tem;
      if (CONSP (val))
	{ val = XCDR (val); }
    }
  else if (!NILP (Vcoding_system_for_write))
    { val = Vcoding_system_for_write; }
  else if ((!NILP (buffer) && NILP (BVAR! (XBUFFER (buffer), enable_multibyte_characters_)))
	   || (NILP (buffer) && NILP (BVAR! (&raw mut buffer_defaults, enable_multibyte_characters_))))
    { val = Qnil (); }
  pset_encode_coding_system (p, val);

  setup_process_coding_systems (proc);
  pset_decoding_buf (p, empty_unibyte_string);
  eassert! ((*p).decoding_carryover == 0);
  pset_encoding_buf (p, empty_unibyte_string);
  process_bits! (inherit_coding_system_flag, p,
		 !(!NILP (tem) || NILP (buffer) || !inherit_process_coding_system));

  Fserial_process_configure (nargs, args);

  (specpdl_ptr! ()) = specpdl_ref_to_ptr (specpdl_count);

  return proc;
});

unsafe extern "C" {
  /// C: void serial_configure (struct Lisp_Process *, Lisp_Object)   (process.h 302)
  fn serial_configure (p: *mut Lisp_Process, contact: Lisp_Object);
  /// C: int serial_open (Lisp_Object)   (process.h 303)
  fn serial_open (port: Lisp_Object) -> c_int;
}

/* C 3329-3415: `set_network_socket_coding_system`.  */
unsafe fn set_network_socket_coding_system (proc_: Lisp_Object, host: Lisp_Object,
					    service: Lisp_Object, name: Lisp_Object)
{
  let mut tem: Lisp_Object;
  let p: *mut Lisp_Process = XPROCESS (proc_);
  let contact: Lisp_Object = (*p).childp;
  let mut coding_systems: Lisp_Object = Qt ();
  let mut val: Lisp_Object;

  tem = plist_get (contact, QCcoding ());

  /* Setup coding systems for communicating with the network stream.  */
  /* Qt denotes we have not yet called Ffind_operation_coding_system.  */

  if (!NILP (tem))
    {
      val = tem;
      if (CONSP (val))
	{ val = XCAR (val); }
    }
  else if (!NILP (Vcoding_system_for_read))
    { val = Vcoding_system_for_read; }
  else if ((!NILP ((*p).buffer)
	    && NILP (BVAR! (XBUFFER ((*p).buffer), enable_multibyte_characters_)))
	   || (NILP ((*p).buffer)
	       && NILP (BVAR! (&raw mut buffer_defaults, enable_multibyte_characters_))))
    /* We dare not decode end-of-line format by setting VAL to Qraw_text, because the
       existing Emacs Lisp libraries assume that they receive bare code including a
       sequence of CR LF.  */
    { val = Qnil (); }
  else
    {
      if (NILP (host) || NILP (service))
	{ coding_systems = Qnil (); }
      else
	{ coding_systems = CALLN! (Ffind_operation_coding_system,
				   Qopen_network_stream (), name, (*p).buffer,
				   host, service); }
      if (CONSP (coding_systems))
	{ val = XCAR (coding_systems); }
      else if (CONSP (Vdefault_process_coding_system))
	{ val = XCAR (Vdefault_process_coding_system); }
      else
	{ val = Qnil (); }
    }
  pset_decode_coding_system (p, val);

  if (!NILP (tem))
    {
      val = tem;
      if (CONSP (val))
	{ val = XCDR (val); }
    }
  else if (!NILP (Vcoding_system_for_write))
    { val = Vcoding_system_for_write; }
  else if (NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)))
    { val = Qnil (); }
  else
    {
      if (EQ (coding_systems, Qt ()))
	{
	  if (NILP (host) || NILP (service))
	    { coding_systems = Qnil (); }
	  else
	    { coding_systems = CALLN! (Ffind_operation_coding_system,
				       Qopen_network_stream (), name, (*p).buffer,
				       host, service); }
	}
      if (CONSP (coding_systems))
	{ val = XCDR (coding_systems); }
      else if (CONSP (Vdefault_process_coding_system))
	{ val = XCDR (Vdefault_process_coding_system); }
      else
	{ val = Qnil (); }
    }
  pset_encode_coding_system (p, val);

  pset_decoding_buf (p, empty_unibyte_string);
  (*p).decoding_carryover = 0;
  pset_encoding_buf (p, empty_unibyte_string);

  process_bits! (inherit_coding_system_flag, p,
		 !(!NILP (tem) || NILP ((*p).buffer) || !inherit_process_coding_system));
}

/* C 3417-3457: `finish_after_tls_connection` — the `HAVE_GNUTLS` arm (dead on this target).  */
#[cfg (feature = "have_gnutls")]
unsafe fn finish_after_tls_connection (proc_: Lisp_Object)
{
  let p: *mut Lisp_Process = XPROCESS (proc_);
  let contact: Lisp_Object = (*p).childp;
  let mut result: Lisp_Object = Qt ();

  if (!NILP (Ffboundp (Qnsm_verify_connection ())))
    { result = calln! (Qnsm_verify_connection (),
		       proc_,
		       plist_get (contact, QChost ()),
		       plist_get (contact, QCservice ())); }

  eassert! ((*p).outfd < FD_SETSIZE);
  if (NILP (result))
    {
      pset_status (p, list2 (Qfailed (),
			     build_string (c"The Network Security Manager stopped the connections".as_ptr ())));
      deactivate_process (proc_);
    }
  else if ((*p).outfd < 0)
    {
      /* The counterparty may have closed the connection (especially if the NSM prompt
	 above take a long time), so recheck the file descriptor here.  */
      pset_status (p, Qfailed ());
      deactivate_process (proc_);
    }
  else if ((fd_callback_info[(*p).outfd as usize].flags & NON_BLOCKING_CONNECT_FD) == 0)
    {
      /* If we cleared the connection wait mask before we did the TLS setup, then we have
	 to say that the process is finally "open" here.  */
      pset_status (p, Qrun ());
      /* Execute the sentinel here.  If we had relied on status_notify to do it later, it
	 will read input from the process before calling the sentinel.  */
      exec_sentinel (proc_, build_string (c"open\n".as_ptr ()));
    }
}

/* C 3461-3800: `connect_network_socket`.  */
unsafe fn connect_network_socket (proc_: Lisp_Object, addrinfos: Lisp_Object,
				  use_external_socket_p: Lisp_Object)
{
  let mut s: c_int = -1;
  let mut outch: c_int;
  let mut inch: c_int;
  let mut xerrno: c_int = 0;
  let mut family: c_int = 0;
  let mut ret: c_int;
  let mut addrlen: ptrdiff_t = 0;
  let mut p: *mut Lisp_Process = XPROCESS (proc_);
  let mut contact: Lisp_Object = (*p).childp;
  let mut optbits: c_int = 0;
  let mut socket_to_use: c_int = -1;
  let mut addrinfos: Lisp_Object = addrinfos;

  if (!NILP (use_external_socket_p))
    {
      socket_to_use = external_sock_fd;
      eassert! (socket_to_use < FD_SETSIZE);

      /* Ensure we don't consume the external socket twice.  */
      external_sock_fd = -1;
    }

  /* Do this in case we never enter the while-loop below.  */
  s = -1;

  let mut sa: *mut sockaddr = std::ptr::null_mut ();
  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect_nothing ();
  let count1: specpdl_ref = SPECPDL_INDEX ();

  while !NILP (addrinfos)
    {
      let addrinfo: Lisp_Object = XCAR (addrinfos);
      addrinfos = XCDR (addrinfos);
      let protocol: c_int = XFIXNUM (XCAR (addrinfo)) as c_int;
      let ip_address: Lisp_Object = XCDR (addrinfo);

      /* C 3496-3498's `#ifdef WINDOWSNT` label `retry_connect:` and C 3670-3672's
	 `goto retry_connect;` (the port's labelled block, PORTING.md 2.2).  */
      #[cfg (feature = "windowsnt")]
      {
	'retry_connect: loop { break 'retry_connect; }
      }

      addrlen = get_lisp_to_sockaddr_size (ip_address, &raw mut family);
      sa = xrealloc (sa as *mut c_void, addrlen as usize) as *mut sockaddr;
      set_unwind_protect_ptr (count, Some (xfree), sa as *mut c_void);
      conv_lisp_to_sockaddr (family, ip_address, sa, addrlen as c_int);

      s = socket_to_use;
      if (s < 0)
	{
	  let mut socktype: c_int = (*p).socktype | SOCK_CLOEXEC;
	  if process_bits! (is_non_blocking_client, p)
	    { socktype |= SOCK_NONBLOCK; }
	  s = inrange_fd (process_socket (family, socktype, protocol));
	  if (s < 0)
	    {
	      xerrno = *errno ();
	      continue;
	    }
	}

      if process_bits! (is_non_blocking_client, p)
	&& ! (SOCK_NONBLOCK != 0 && socket_to_use < 0)
	{
	  ret = fcntl (s, F_SETFL, O_NONBLOCK);
	  if (ret < 0)
	    {
	      xerrno = *errno ();
	      emacs_close (s);
	      s = -1;
	      if (0 <= socket_to_use)
		{ break; }
	      continue;
	    }
	}

      /* `#ifdef DATAGRAM_SOCKETS` is live.  */
      if (!process_bits! (is_server, p) && (*p).socktype == SOCK_DGRAM)
	{ break; }

      /* Make us close S if quit.  */
      record_unwind_protect_int (Some (close_file_unwind), s);

      /* Parse network options in the arg list.  We simply ignore anything which isn't a
	 known option (including other keywords).  An error is signaled if setting a known
	 option fails.  */
      {
	let mut params: Lisp_Object = contact;
	let mut key: Lisp_Object;
	let mut val: Lisp_Object;

	while !NILP (params)
	  {
	    key = XCAR (params);
	    params = XCDR (params);
	    val = XCAR (params);
	    params = XCDR (params);
	    optbits |= set_socket_option (s, key, val);
	  }
      }

      if process_bits! (is_server, p)
	{
	  /* Configure as a server socket.  */

	  /* SO_REUSEADDR = 1 is default for server sockets; must specify explicit :reuseaddr
	     key to override this.  */
	  /* `#ifdef HAVE_LOCAL_SOCKETS` is live: `if (family != AF_LOCAL)`.  */
	  if (family != AF_LOCAL)
	    {
	      if !(optbits & (1 << OPIX_REUSEADDR)) != 0
		{
		  let optval: c_int = 1;
		  if (setsockopt (s, SOL_SOCKET, SO_REUSEADDR, &raw const optval as *mut c_void,
				  std::mem::size_of_val (&optval) as socklen_t) != 0)
		    { report_file_error (c"Cannot set reuse option on server socket".as_ptr (), Qnil ()); }
		}
	    }

	  /* If passed a socket descriptor, it should be already bound.  */
	  if (socket_to_use < 0 && bind (s, sa, addrlen as socklen_t) != 0)
	    { report_file_error (c"Cannot bind server socket".as_ptr (), Qnil ()); }

	  /* `#ifdef HAVE_GETSOCKNAME` is live.  */
	  if ((*p).port == 0
	      /* `#ifdef HAVE_LOCAL_SOCKETS` is live.  */
	      && family != AF_LOCAL)
	    {
	      let mut sa1: sockaddr_in = std::mem::zeroed ();
	      let mut len1: socklen_t = std::mem::size_of_val (&sa1) as socklen_t;
	      /* `#ifdef AF_INET6` is live.  */
	      let mut sa6: sockaddr_in6 = std::mem::zeroed ();
	      static_assert! (std::mem::offset_of! (sockaddr_in, sin_port)
			      == std::mem::offset_of! (sockaddr_in6, sin6_port)
			      && std::mem::size_of::<u16> ()
			      == std::mem::size_of::<u16> ());
	      /* DECLARE_POINTER_ALIAS (psa1, struct sockaddr, &sa1);  */
	      let psa1: *mut sockaddr = &raw mut sa1 as *mut sockaddr;
	      if (getsockname (s, psa1, &raw mut len1) == 0)
		{
		  let service: Lisp_Object = make_fixnum (ntohs (sa1.sin_port as u16) as EMACS_INT);
		  contact = plist_put (contact, QCservice (), service);
		  /* Save the port number so that we can stash it in the process object
		     later.  */
		  /* DECLARE_POINTER_ALIAS (psa, struct sockaddr_in, sa);  */
		  let psa: *mut sockaddr_in = sa as *mut sockaddr_in;
		  (*psa).sin_port = sa1.sin_port;
		}
	    }

	  if ((*p).socktype != SOCK_DGRAM && listen (s, (*p).backlog) != 0)
	    { report_file_error (c"Cannot listen on server socket".as_ptr (), Qnil ()); }

	  break;
	}

      maybe_quit ();

      ret = connect (s, sa, addrlen as socklen_t);
      xerrno = *errno ();

      if (ret == 0 || xerrno == EISCONN)
	{
	  /* The unwind-protect will be discarded afterwards.  */
	  break;
	}

      if process_bits! (is_non_blocking_client, p) && xerrno == EINPROGRESS
	{ break; }

      /* `#ifndef WINDOWSNT` is live.  */
      if (xerrno == EINTR)
	{
	  /* Unlike most other syscalls connect() cannot be called again.  (That would
	     return EALREADY.)  The proper way to wait for completion is pselect().  */
	  let mut sc: c_int;
	  let mut len: socklen_t;
	  let mut fdset: fd_set = std::mem::zeroed ();
	  'retry_select: loop
	    {
	      FD_ZERO (&raw mut fdset);
	      FD_SET (s, &raw mut fdset);
	      maybe_quit ();
	      sc = pselect (s + 1, std::ptr::null_mut (), &raw mut fdset,
			    std::ptr::null_mut (), std::ptr::null (), std::ptr::null ());
	      if (sc == -1)
		{
		  if (*errno () == EINTR)
		    { continue 'retry_select; }
		  else
		    { report_file_error (c"Failed select".as_ptr (), Qnil ()); }
		}
	      break;
	    }
	  eassert! (sc > 0);

	  len = std::mem::size_of_val (&xerrno) as socklen_t;
	  eassert! (FD_ISSET (s, &raw const fdset));
	  if (getsockopt (s, SOL_SOCKET, SO_ERROR, &raw mut xerrno as *mut c_void, &raw mut len) < 0)
	    { report_file_error (c"Failed getsockopt".as_ptr (), Qnil ()); }
	  if (xerrno == 0)
	    { break; }
	  if (NILP (addrinfos))
	    { report_file_errno (c"Failed connect".as_ptr (), Qnil (), xerrno); }
	}

      /* Discard the unwind protect closing S.  */
      (specpdl_ptr! ()) = specpdl_ref_to_ptr (count1);
      emacs_close (s);
      s = -1;
      if (0 <= socket_to_use)
	{ break; }
    }

  if (s >= 0)
    {
      /* `#ifdef DATAGRAM_SOCKETS` is live.  */
      if ((*p).socktype == SOCK_DGRAM)
	{
	  eassert! (0 <= s && s < FD_SETSIZE);
	  if (!datagram_address[s as usize].sa.is_null ())
	    { emacs_abort (); }

	  datagram_address[s as usize].sa = xmalloc (addrlen as usize) as *mut sockaddr;
	  datagram_address[s as usize].len = addrlen;
	  if process_bits! (is_server, p)
	    {
	      let mut remote: Lisp_Object;
	      memset (datagram_address[s as usize].sa as *mut c_void, 0, addrlen as usize);
	      remote = plist_get (contact, QCremote ());
	      if !NILP (remote)
		{
		  let mut rfamily: c_int = 0;
		  let rlen: ptrdiff_t = get_lisp_to_sockaddr_size (remote, &raw mut rfamily);
		  if (rlen != 0 && rfamily == family
		      && rlen == addrlen)
		    { conv_lisp_to_sockaddr (rfamily, remote,
					     datagram_address[s as usize].sa, rlen as c_int); }
		}
	    }
	  else
	    { memcpy (datagram_address[s as usize].sa as *mut c_void, sa as *const c_void, addrlen as usize); }
	}

      contact = plist_put (contact, if process_bits! (is_server, p) { QClocal () } else { QCremote () },
			   conv_sockaddr_to_lisp (sa, addrlen));
      /* `#ifdef HAVE_GETSOCKNAME` is live.  */
      if (!process_bits! (is_server, p))
	{
	  let mut sa1: sockaddr_storage = std::mem::zeroed ();
	  let mut len1: socklen_t = std::mem::size_of_val (&sa1) as socklen_t;
	  /* DECLARE_POINTER_ALIAS (psa1, struct sockaddr, &sa1);  */
	  let psa1: *mut sockaddr = &raw mut sa1 as *mut sockaddr;
	  if (getsockname (s, psa1, &raw mut len1) == 0)
	    { contact = plist_put (contact, QClocal (),
				   conv_sockaddr_to_lisp (psa1, len1 as ptrdiff_t)); }
	}
    }

  if (s < 0)
    {
      let err: *const c_char = (if process_bits! (is_server, p)
				{ c"make server process failed" }
				else { c"make client process failed" }).as_ptr ();

      /* If non-blocking got this far - and failed - assume non-blocking is not supported
	 after all.  This is probably a wrong assumption, but the normal blocking calls to
	 open-network-stream handles this error better.  */
      if process_bits! (is_non_blocking_client, p)
	{
	  let data: Lisp_Object = get_file_errno_data (err, contact, xerrno);

	  pset_status (p, list2 (Qfailed (), data));
	  unbind_to (count, Qnil ());
	  return;
	}

      report_file_errno (err, contact, xerrno);
    }

  inch = s;
  outch = s;

  eassert! (0 <= inch && inch < FD_SETSIZE);
  chan_process[inch as usize] = proc_;

  fcntl (inch, F_SETFL, O_NONBLOCK);

  p = XPROCESS (proc_);
  (*p).open_fd[SUBPROCESS_STDIN as usize] = inch;
  (*p).infd = inch;
  (*p).outfd = outch;

  /* Discard the unwind protect for closing S, if any.  */
  (specpdl_ptr! ()) = specpdl_ref_to_ptr (count1);

  if process_bits! (is_server, p) && (*p).socktype != SOCK_DGRAM
    { pset_status (p, Qlisten ()); }

  /* Make the process marker point into the process buffer (if any).  */
  update_process_mark (p);

  if process_bits! (is_non_blocking_client, p)
    {
      /* We may get here if connect did succeed immediately.  However, in that case, we
	 still need to signal this like a non-blocking connection.  */
      if (! (connecting_status ((*p).status)
	     && EQ (XCDR ((*p).status), addrinfos)))
	{ pset_status (p, Fcons (Qconnect (), addrinfos)); }
      eassert! (0 <= inch && inch < FD_SETSIZE);
      if ((fd_callback_info[inch as usize].flags & NON_BLOCKING_CONNECT_FD) == 0)
	{ add_non_blocking_write_fd (inch); }
    }
  else
    /* A server may have a client filter setting of Qt, but it must still listen for
       incoming connects unless it is stopped.  */
    if ((!EQ ((*p).filter, Qt ()) && !EQ ((*p).command, Qt ()))
	|| (EQ ((*p).status, Qlisten ()) && NILP ((*p).command)))
      { add_process_read_fd (inch); }

  if (inch > max_desc)
    { max_desc = inch; }

  /* Set up the masks based on the process filter.  */
  set_process_filter_masks (p);

  setup_process_coding_systems (proc_);

  /* C 3778-3796's `#ifdef HAVE_GNUTLS` arm: continue the asynchronous connection.  */
  #[cfg (feature = "have_gnutls")]
  {
    if (!NILP ((*p).gnutls_boot_parameters))
      {
	let mut boot: Lisp_Object;
	let params: Lisp_Object = (*p).gnutls_boot_parameters;

	boot = Fgnutls_boot (proc_, XCAR (params), XCDR (params));

	if ((*p).gnutls_initstage == GNUTLS_STAGE_READY)
	  {
	    (*p).gnutls_boot_parameters = Qnil ();
	    /* Run sentinels, etc.  */
	    finish_after_tls_connection (proc_);
	  }
	else if ((*p).gnutls_initstage != GNUTLS_STAGE_HANDSHAKE_TRIED)
	  {
	    deactivate_process (proc_);
	    if (NILP (boot))
	      { pset_status (p, list2 (Qfailed (),
				       build_string (c"TLS negotiation failed".as_ptr ()))); }
	    else
	      { pset_status (p, list2 (Qfailed (), boot)); }
	  }
      }
  }

  unbind_to (count, Qnil ());
}

unsafe extern "C" {
  /// C: void record_unwind_protect_nothing (void)   (lisp.h 4827, eval.c)
  fn record_unwind_protect_nothing ();
  /// C: void set_unwind_protect_ptr (specpdl_ref, void (*) (void *), void *)   (lisp.h 4832)
  fn set_unwind_protect_ptr (count: specpdl_ref,
			     function: Option<unsafe extern "C" fn (arg: *mut c_void)>,
			     arg: *mut c_void);
  /// C: Lisp_Object get_file_errno_data (const char *, Lisp_Object, int)   (lisp.h 4980)
  fn get_file_errno_data (string: *const c_char, err: Lisp_Object, errnum: c_int) -> Lisp_Object;
  /// C: int bind (int, const struct sockaddr *, socklen_t)   (<sys/socket.h>)
  fn bind (socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
  /// C: int connect (int, const struct sockaddr *, socklen_t)   (<sys/socket.h>)
  fn connect (socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int;
  /// C: int listen (int, int)   (<sys/socket.h>)
  fn listen (socket: c_int, backlog: c_int) -> c_int;
  /// C: int getsockname (int, struct sockaddr *restrict, socklen_t *restrict)
  fn getsockname (socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
  /// C: int getsockopt (int, int, int, void *restrict, socklen_t *restrict)
  fn getsockopt (socket: c_int, level: c_int, option_name: c_int,
		 option_value: *mut c_void, option_len: *mut socklen_t) -> c_int;
}

/// C: `#define AF_UNSPEC 0` (`<sys/socket.h>`)
pub const AF_UNSPEC: c_int = 0;
/// C: `#define SOCK_STREAM 1` (`<sys/socket.h>`)
pub const SOCK_STREAM: c_int = 1;
/// C: `#define SOCK_SEQPACKET 5` (`<sys/socket.h>`)
pub const SOCK_SEQPACKET: c_int = 5;
/// C: `#define SOCK_DGRAM 2` (`<sys/socket.h>`)
pub const SOCK_DGRAM: c_int = 2;
/// C: `#define SO_ERROR 4103` (`<sys/socket.h>`)
pub const SO_ERROR: c_int = 4103;

/* C 3812-4326: `Fmake_network_process`.  C's four forward `goto open_socket` jumps are the
   port's labelled block `'open_socket` (PORTING.md 2.2): `break 'open_socket` leaves it where
   C's label lands.  */
DEFUN! ("make-network-process", Fmake_network_process, Smake_network_process, 0, MANY, 0,
       doc: "Create and return a network server or client process.\n\nIn Emacs, network connections are represented by process objects, so\ninput and output work as for subprocesses and `delete-process' closes\na network connection.  However, a network process has no process id,\nit cannot be signaled, and the status codes are different from normal\nprocesses.\n\nArguments are specified as keyword/argument pairs.  The following\narguments are defined:\n\n:name NAME -- NAME is name for process.  It is modified if necessary\nto make it unique.\n\n:buffer BUFFER -- BUFFER is the buffer (or buffer-name) to associate\nwith the process.  Process output goes at end of that buffer, unless\nyou specify a filter function to handle the output.  BUFFER may be\nalso nil, meaning that this process is not associated with any buffer.\n\n:host HOST -- HOST is name of the host to connect to, or its IP\naddress.  The symbol `local' specifies the local host.  If specified\nfor a server process, it must be a valid name or address for the local\nhost, and only clients connecting to that address will be accepted.\nIf all interfaces should be bound, an address of \"0.0.0.0\" (for\nIPv4) or \"::\" (for IPv6) can be used.  (On some operating systems,\nusing \"::\" listens on both IPv4 and IPv6.)  `local' will use IPv4 by\ndefault, use a FAMILY of `ipv6' to override this.\n\n:service SERVICE -- SERVICE is name of the service desired, or an\ninteger specifying a port number to connect to.  If SERVICE is t,\na random port number is selected for the server.  A port number can\nbe specified as an integer string, e.g., \"80\", as well as an integer.\n\n:type TYPE -- TYPE is the type of connection.  The default (nil) is a\nstream type connection, `datagram' creates a datagram type connection,\n`seqpacket' creates a reliable datagram connection.\n\n:family FAMILY -- FAMILY is the address (and protocol) family for the\nservice specified by HOST and SERVICE.  The default (nil) is to use\nwhatever address family (IPv4 or IPv6) that is defined for the host\nand port number specified by HOST and SERVICE.  Other address families\nsupported are:\n  local -- for a local (i.e. UNIX) address specified by SERVICE.\n  ipv4  -- use IPv4 address family only.\n  ipv6  -- use IPv6 address family only.\n\n:local ADDRESS -- ADDRESS is the local address used for the connection.\nThis parameter is ignored when opening a client process. When specified\nfor a server process, the FAMILY, HOST and SERVICE args are ignored.\n\n:remote ADDRESS -- ADDRESS is the remote partner's address for the\nconnection.  This parameter is ignored when opening a stream server\nprocess.  For a datagram server process, it specifies the initial\nsetting of the remote datagram address.  When specified for a client\nprocess, the FAMILY, HOST, and SERVICE args are ignored.\n\nThe format of ADDRESS depends on the address family:\n- An IPv4 address is represented as a vector of integers [A B C D P]\ncorresponding to numeric IP address A.B.C.D and port number P.\n- An IPv6 address has the same format as an IPv4 address but with 9\nelements rather than 5.\n- A local address is represented as a string with the address in the\nlocal address space.\n- An \"unsupported family\" address is represented by a cons (F . AV)\nwhere F is the family number and AV is a vector containing the socket\naddress data with one element per address data byte.  Do not rely on\nthis format in portable code, as it may depend on implementation\ndefined constants, data sizes, and data structure alignment.\n\n:coding CODING -- If CODING is a symbol, it specifies the coding\nsystem used for both reading and writing for this process.  If CODING\nis a cons (DECODING . ENCODING), DECODING is used for reading, and\nENCODING is used for writing.\n\n:nowait BOOL -- If NOWAIT is non-nil for a stream type client\nprocess, return without waiting for the connection to complete;\ninstead, the sentinel function will be called with second arg matching\n\"open\" (if successful) or \"failed\" when the connect completes.\nDefault is to use a blocking connect (i.e. wait) for stream type\nconnections.\n\n:noquery BOOL -- Query the user unless BOOL is non-nil, and process is\nrunning when Emacs is exited.\n\n:stop BOOL -- Start process in the `stopped' state if BOOL non-nil.\nIn the stopped state, a server process does not accept new\nconnections, and a client process does not handle incoming traffic.\nThe stopped state is cleared by `continue-process' and set by\n`stop-process'.\n\n:filter FILTER -- Install FILTER as the process filter.\n\n:filter-multibyte BOOL -- If BOOL is non-nil, strings given to the\nprocess filter are multibyte, otherwise they are unibyte.\nIf this keyword is not specified, the strings are multibyte.\n\n:sentinel SENTINEL -- Install SENTINEL as the process sentinel.\n\n:log LOG -- Install LOG as the server process log function.  This\nfunction is called when the server accepts a network connection from a\nclient.  The arguments are SERVER, CLIENT, and MESSAGE, where SERVER\nis the server process, CLIENT is the new process for the connection,\nand MESSAGE is a string.\n\n:plist PLIST -- Install PLIST as the new process's initial plist.\n\n:tls-parameters LIST -- is a list that should be supplied if you're\nopening a TLS connection.  The first element is the TLS type (either\n`gnutls-x509pki' or `gnutls-anon'), and the remaining elements should\nbe a keyword list accepted by gnutls-boot (as returned by\n`gnutls-boot-parameters').\n\n:server QLEN -- if QLEN is non-nil, create a server process for the\nspecified FAMILY, SERVICE, and connection type (stream or datagram).\nIf QLEN is an integer, it is used as the max. length of the server's\npending connection queue (also known as the backlog); the default\nqueue length is 5.  Default is to create a client process.\n\nThe following network options can be specified for this connection:\n\n:broadcast BOOL    -- Allow send and receive of datagram broadcasts.\n:dontroute BOOL    -- Only send to directly connected hosts.\n:keepalive BOOL    -- Send keep-alive messages on network stream.\n:nodelay BOOL      -- Set TCP_NODELAY on the network socket.\n:linger BOOL or TIMEOUT -- Send queued messages before closing.\n:oobinline BOOL    -- Place out-of-band data in receive data stream.\n:priority INT      -- Set protocol defined priority for sent packets.\n:reuseaddr BOOL    -- Allow reusing a recently used local address\n                      (this is allowed by default for a server process).\n:bindtodevice NAME -- bind to interface NAME.  Using this may require\n                      special privileges on some systems.\n:use-external-socket BOOL -- Use any pre-allocated sockets that have\n                             been passed to Emacs.  If Emacs wasn't\n                             passed a socket, this option is silently\n                             ignored.\n\n\nConsult the relevant system programmer's manual pages for more\ninformation on using these options.\n\n\nA server process will listen for and accept connections from clients.\nWhen a client connection is accepted, a new network process is created\nfor the connection with the following parameters:\n\n- The client's process name is constructed by concatenating the server\nprocess's NAME and a client identification string.\n- If the FILTER argument is non-nil, the client process will not get a\nseparate process buffer; otherwise, the client's process buffer is a newly\ncreated buffer named after the server process's BUFFER name or process\nNAME concatenated with the client identification string.\n- The connection type and the process filter and sentinel parameters are\ninherited from the server process's TYPE, FILTER and SENTINEL.\n- The client process's contact info is set according to the client's\naddressing information (typically an IP address and a port number).\n- The client process's plist is initialized from the server's plist.\n\nNotice that the FILTER and SENTINEL args are never used directly by\nthe server process.  Also, the BUFFER argument is not used directly by\nthe server process, but via the optional :log function, accepted (and\nfailed) connections may be logged in the server process's buffer.\n\nThe original argument list, modified with the actual connection\ninformation, is available via the `process-contact' function.\n\nusage: (make-network-process &rest ARGS)",
       (nargs: isize, args: *mut Lisp_Object),
       {
  let mut proc: Lisp_Object;
  let mut contact: Lisp_Object;
  let mut p: *mut Lisp_Process;
  let mut portstring: *const c_char = std::ptr::null ();
  let mut portbuf: [c_char; INT_BUFSIZE_BOUND] = [0; INT_BUFSIZE_BOUND];
  /* `#ifdef HAVE_LOCAL_SOCKETS` is live: `struct sockaddr_un address_un;`  */
  let mut address_un: sockaddr_un = std::mem::zeroed ();
  let mut port: EMACS_INT = 0;
  let mut tem: Lisp_Object;
  let mut buffer: Lisp_Object;
  let mut host: Lisp_Object;
  let mut service: Lisp_Object;
  let mut address: Lisp_Object;
  let mut filter: Lisp_Object;
  let mut sentinel: Lisp_Object;
  let mut use_external_socket_p: Lisp_Object;
  let mut addrinfos: Lisp_Object = Qnil ();
  let mut socktype: c_int;
  let mut family: c_int = -1;
  /* `enum { any_protocol = 0 };`  */
  let any_protocol: c_int = 0;
  /* `#ifdef HAVE_GETADDRINFO_A` is dead: `struct gaicb *dns_request = NULL;`  */
  let count: specpdl_ref = SPECPDL_INDEX ();

  if (nargs == 0)
    { return Qnil (); }
  CHECK_KEYWORD_ARGS (nargs);

  /* Save arguments for process-contact and clone-process.  */
  contact = Flist (nargs, args);

  /* C 3843-3846's `#ifdef WINDOWSNT` arm: ensure socket support is loaded if
     available.  */
  #[cfg (feature = "windowsnt")]
  {
    init_winsock (true);
  }

  /* :type TYPE  (nil: stream, datagram  */
  tem = plist_get (contact, QCtype ());
  if (NILP (tem))
    { socktype = SOCK_STREAM; }
  /* `#ifdef DATAGRAM_SOCKETS` is live.  */
  else if (EQ (tem, Qdatagram ()))
    { socktype = SOCK_DGRAM; }
  /* `#ifdef HAVE_SEQPACKET` is live.  */
  else if (EQ (tem, Qseqpacket ()))
    { socktype = SOCK_SEQPACKET; }
  else
    { error (c"Unsupported connection type".as_ptr ()); }

  let mut name: Lisp_Object = get_required_string_keyword_param (contact, QCname ());
  buffer = plist_get (contact, QCbuffer ());
  filter = plist_get (contact, QCfilter ());
  sentinel = plist_get (contact, QCsentinel ());
  use_external_socket_p = plist_get (contact, QCuse_external_socket ());
  let server: Lisp_Object = plist_get (contact, QCserver ());
  let nowait: bool = !NILP (plist_get (contact, QCnowait ()));

  if (!NILP (server) && nowait)
    { error (c"`:server' is incompatible with `:nowait'".as_ptr ()); }

  'open_socket: {
    /* :local ADDRESS or :remote ADDRESS */
    if (NILP (server))
      { address = plist_get (contact, QCremote ()); }
    else
      { address = plist_get (contact, QClocal ()); }
    if (!NILP (address))
      {
	host = Qnil (); service = Qnil ();

	if (get_lisp_to_sockaddr_size (address, &raw mut family) == 0)
	  { error (c"Malformed :address".as_ptr ()); }

	addrinfos = list1 (Fcons (make_fixnum (any_protocol as EMACS_INT), address));
	break 'open_socket;
      }

    /* :family FAMILY -- nil (for Inet), local, or integer.  */
    tem = plist_get (contact, QCfamily ());
    if (NILP (tem))
      {
	/* `#ifdef AF_INET6` is live.  */
	family = AF_UNSPEC;
      }
    /* `#ifdef HAVE_LOCAL_SOCKETS` is live.  */
    else if (EQ (tem, Qlocal ()))
      { family = AF_LOCAL; }
    /* `#ifdef AF_INET6` is live.  */
    else if (EQ (tem, Qipv6 ()))
      { family = AF_INET6; }
    else if (EQ (tem, Qipv4 ()))
      { family = AF_INET; }
    else if (TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, tem, INT_MAX as EMACS_INT))
      { family = XFIXNUM (tem) as c_int; }
    else
      { error (c"Unknown address family".as_ptr ()); }

    /* :service SERVICE -- string, integer (port number), or t (random port).  */
    service = plist_get (contact, QCservice ());

    /* :host HOST -- hostname, ip address, or 'local for localhost.  */
    host = plist_get (contact, QChost ());
    if (NILP (host))
      {
	/* The "connection" function gets it bind info from the address we're given, so
	   use this dummy address if nothing is specified.  */
	/* `#ifdef HAVE_LOCAL_SOCKETS` is live.  */
	if (family != AF_LOCAL)
	  {
	    /* `#ifdef AF_INET6` is live.  */
	    if (family == AF_INET6)
	      { host = build_string (c"::1".as_ptr ()); }
	    else
	      { host = build_string (c"127.0.0.1".as_ptr ()); }
	  }
      }
    else
      {
	if (EQ (host, Qlocal ()))
	  {
	    /* Depending on setup, "localhost" may map to different IPv4 and/or IPv6
	       addresses, so it's better to be explicit (Bug#6781).  */
	    /* `#ifdef AF_INET6` is live.  */
	    if (family == AF_INET6)
	      { host = build_string (c"::1".as_ptr ()); }
	    else
	      { host = build_string (c"127.0.0.1".as_ptr ()); }
	  }
	CHECK_STRING (host);
      }

    /* `#ifdef HAVE_LOCAL_SOCKETS` is live.  */
    if (family == AF_LOCAL)
      {
	if (!NILP (host))
	  {
	    message (c":family local ignores the :host property".as_ptr ());
	    contact = plist_put (contact, QChost (), Qnil ());
	    host = Qnil ();
	  }
	CHECK_STRING (service);
	if (std::mem::size_of_val (&address_un.sun_path) as ptrdiff_t <= SBYTES (service))
	  { error (c"Service name too long".as_ptr ()); }
	addrinfos = list1 (Fcons (make_fixnum (any_protocol as EMACS_INT), service));
	break 'open_socket;
      }

    /* Slow down polling to every ten seconds.  Some kernels have a bug which causes
       retrying connect to fail after a connect.  Polling can interfere with
       gethostbyname too.  */
    /* C 4154-4159's `#ifdef POLL_FOR_INPUT` arm is carried text (feature
       `poll_for_input`).  */
    #[cfg (feature = "poll_for_input")]
    {
      if (socktype != SOCK_DGRAM)
	{
	  record_unwind_protect_void (Some (run_all_atimers));
	  bind_polling_period (10);
	}
    }

    if (!NILP (host))
      {
	let portstringlen: ptrdiff_t;

	/* SERVICE can either be a string or int.  Convert to a C string for later use by
	   getaddrinfo.  */
	if (EQ (service, Qt ()))
	  {
	    portstring = c"0".as_ptr ();
	    portstringlen = 1;
	  }
	else if (FIXNUMP (service))
	  {
	    portstring = portbuf.as_ptr ();
	    portstringlen = sprintf (portbuf.as_mut_ptr (), c"%ld".as_ptr (),
				     XFIXNUM (service)) as ptrdiff_t;
	  }
	else
	  {
	    CHECK_STRING (service);
	    portstring = SSDATA (service) as *const c_char;
	    portstringlen = SBYTES (service);
	  }

	/* C 4180-4213's `#ifdef HAVE_GETADDRINFO_A` arm is carried text (feature
	   `have_getaddrinfo_a`): the async DNS request and its flexible-array `struct req`.  */
	#[cfg (feature = "have_getaddrinfo_a")]
	{
	  let hostlen: ptrdiff_t = SBYTES (host);
	  #[repr(C)]
	  pub struct req
	  {
	    pub gaicb: gaicb,
	    pub hints: addrinfo,
	    pub r#str: [c_char; 0],
	  }
	  let req_: *mut req = xmalloc (FLEXSIZEOF! (req, r#str,
						     (hostlen + 1 + portstringlen + 1) as usize)
					as usize) as *mut req;
	  dns_request = &raw mut (*req_).gaicb;
	  (*dns_request).ar_name = (*req_).r#str.as_mut_ptr ();
	  (*dns_request).ar_service = (*req_).r#str.as_mut_ptr ().offset (hostlen + 1);
	  (*dns_request).ar_request = &raw const (*req_).hints;
	  (*dns_request).ar_result = std::ptr::null_mut ();
	  memset (&raw mut (*req_).hints as *mut c_void, 0, std::mem::size_of_val (&(*req_).hints));
	  (*req_).hints.ai_family = family;
	  (*req_).hints.ai_socktype = socktype;
	  strcpy ((*req_).r#str.as_mut_ptr (), SSDATA (host) as *const c_char);
	  strcpy ((*req_).r#str.as_mut_ptr ().offset (hostlen + 1), portstring);

	  let ret: c_int = getaddrinfo_a (GAI_NOWAIT, &raw mut dns_request, 1,
					  std::ptr::null_mut ());
	  if (ret != 0)
	    { error (c"%s/%s getaddrinfo_a error %d".as_ptr (),
		     SSDATA (host), portstring, ret); }

	  break 'open_socket;
	}
      }

    /* If we have a host, use getaddrinfo to resolve both host and service.  Otherwise,
       use getservbyname to lookup the service.  */

    if (!NILP (host))
      {
	let mut res: *mut addrinfo = std::ptr::null_mut ();
	let mut lres: *mut addrinfo;
	let msg: Lisp_Object;

	maybe_quit ();

	let mut hints: addrinfo = std::mem::zeroed ();
	memset (&raw mut hints as *mut c_void, 0, std::mem::size_of_val (&hints));
	hints.ai_family = family;
	hints.ai_socktype = socktype;

	msg = network_lookup_address_info_1 (host, portstring, &raw mut hints, &raw mut res);
	if (!EQ (msg, Qt ()))
	  { error (c"%s".as_ptr (), SSDATA (msg)); }

	lres = res;
	while !lres.is_null ()
	  {
	    addrinfos = Fcons (conv_addrinfo_to_lisp (lres), addrinfos);
	    lres = (*lres).ai_next;
	  }

	addrinfos = Fnreverse (addrinfos);

	freeaddrinfo (res);

	break 'open_socket;
      }

    /* No hostname has been specified (e.g., a local server process).  */

    if (EQ (service, Qt ()))
      { port = 0; }
    else if (FIXNUMP (service))
      { port = XFIXNUM (service); }
    else
      {
	CHECK_STRING (service);

	port = -1;
	if (SBYTES (service) != 0)
	  {
	    /* Allow the service to be a string containing the port number, because
	       that's allowed if you have getaddrbyname.  */
	    let mut service_end: *mut c_char = std::ptr::null_mut ();
	    let lport: c_long = strtol (SSDATA (service) as *const c_char,
					&raw mut service_end, 10);
	    if (service_end == (SSDATA (service) as *mut c_char).offset (SBYTES (service) as isize))
	      { port = lport as EMACS_INT; }
	    else
	      {
		let svc_info: *mut servent
		  = getservbyname (SSDATA (service) as *const c_char,
				   if socktype == SOCK_DGRAM { c"udp" } else { c"tcp" }.as_ptr ());
		if (!svc_info.is_null ())
		  { port = ntohs ((*svc_info).s_port as u16) as EMACS_INT; }
	      }
	  }
      }

    if (! (0 <= port && port < 1 << 16))
      {
	AUTO_STRING! (unknown_service, c"Unknown service: %s".as_ptr ());
	xsignal1 (Qerror (), CALLN! (Fformat, unknown_service, service));
      }
  }

  if (!NILP (buffer))
    { buffer = Fget_buffer_create (buffer, Qnil ()); }

  /* Unwind bind_polling_period.  */
  unbind_to (count, Qnil ());

  proc = make_process (name);
  record_unwind_protect (Some (remove_process), proc);
  p = XPROCESS (proc);
  pset_childp (p, contact);
  pset_plist (p, Fcopy_sequence (plist_get (contact, QCplist ())));
  pset_type (p, Qnetwork ());

  pset_buffer (p, buffer);
  pset_sentinel (p, sentinel);
  pset_filter (p, filter);
  pset_log (p, plist_get (contact, QClog ()));
  tem = plist_get (contact, QCnoquery ());
  if !NILP (tem)
    { process_bits! (kill_without_query, p, 1); }
  tem = plist_get (contact, QCstop ());
  if !NILP (tem)
    { pset_command (p, Qt ()); }
  eassert! ((*p).pid == 0);
  (*p).backlog = 5;
  eassert! (! process_bits! (is_non_blocking_client, p));
  eassert! (! process_bits! (is_server, p));
  (*p).port = port as c_int;
  (*p).socktype = socktype;
  /* C 4306-4310's `#ifdef HAVE_GETADDRINFO_A` arm.  */
  #[cfg (feature = "have_getaddrinfo_a")]
  {
    eassert! ((*p).dns_request.is_null ());
  }
  /* C 4311-4317's `#ifdef HAVE_GNUTLS` arm.  */
  #[cfg (feature = "have_gnutls")]
  {
    tem = plist_get (contact, QCtls_parameters ());
    CHECK_LIST (tem);
    (*p).gnutls_boot_parameters = tem;
  }

  set_network_socket_coding_system (proc, host, service, name);

  /* :server QLEN */
  process_bits! (is_server, p, !NILP (server));
  if (TYPE_RANGED_FIXNUMP (INT_MIN as EMACS_INT, server, INT_MAX as EMACS_INT))
    { (*p).backlog = XFIXNUM (server) as c_int; }

  /* :nowait BOOL */
  if (!process_bits! (is_server, p) && socktype != SOCK_DGRAM && nowait)
    { process_bits! (is_non_blocking_client, p, true); }

  let mut postpone_connection: bool = false;
  /* C 4332-4340's `#ifdef HAVE_GETADDRINFO_A` arm: with async address resolution, the list
     of addresses is empty, so postpone connecting to the server.  */
  #[cfg (feature = "have_getaddrinfo_a")]
  {
    if (!process_bits! (is_server, p) && NILP (addrinfos))
      {
	(*p).dns_request = dns_request;
	(*p).status = list1 (Qconnect ());
	postpone_connection = true;
      }
  }
  if (! postpone_connection)
    { connect_network_socket (proc, addrinfos, use_external_socket_p); }

  (specpdl_ptr! ()) = specpdl_ref_to_ptr (count);
  return proc;
});

/// C: `<intprops.h>`'s `INT_BUFSIZE_BOUND (EMACS_INT)` — `INT_STRLEN_BOUND (EMACS_INT) + 1`
/// = 21 for this target's 64-bit `EMACS_INT` (measured: `printf ("%d", INT_BUFSIZE_BOUND
/// (EMACS_INT))` under the frozen configuration).
const INT_BUFSIZE_BOUND: usize = 21;

unsafe extern "C" {
  /// C: struct servent *getservbyname (const char *, const char *)   (<netdb.h>)
  fn getservbyname (name: *const c_char, proto: *const c_char) -> *mut servent;
  /// C: void record_unwind_protect_void (void (*) (void))   (lisp.h 4825, eval.c)
  fn record_unwind_protect_void (function: Option<unsafe extern "C" fn ()>);
  /// C: void bind_polling_period (int)   (keyboard.c)
  fn bind_polling_period (n: c_int);
  /// C: int getaddrinfo_a (int, struct gaicb *[], int, struct sigevent *)
  ///    (<netdb.h>, glibc only — the `HAVE_GETADDRINFO_A` arm)
  fn getaddrinfo_a (mode: c_int, list: *mut *mut gaicb, nitems: c_int,
		    sevp: *mut c_void) -> c_int;
}

/* `#ifdef HAVE_GETIFADDRS` is live.  */
/* C 4354-4428: `network_interface_list`.  */
unsafe fn network_interface_list (full: bool, r#match: c_ushort) -> Lisp_Object
{
  let mut res: Lisp_Object = Qnil ();
  let mut ifap: *mut ifaddrs = std::ptr::null_mut ();

  if (getifaddrs (&raw mut ifap) == -1)
    { return Qnil (); }

  let mut it: *mut ifaddrs = ifap;
  while it != std::ptr::null_mut ()
    {
      let mut len: c_int;
      let mut addr_len: c_int;
      let mut maskp: *mut u32;
      let mut addrp: *mut u32;
      let mut elt: Lisp_Object = Qnil ();

      /* BSD can allegedly return interfaces with a NULL address.  */
      if ((*it).ifa_addr == std::ptr::null_mut ())
	{ it = (*it).ifa_next; continue; }
      if (r#match != 0 && (*(*it).ifa_addr).sa_family as c_ushort != r#match)
	{ it = (*it).ifa_next; continue; }
      if ((*(*it).ifa_addr).sa_family as c_int == AF_INET)
	{
	  /* DECLARE_POINTER_ALIAS (sin1, struct sockaddr_in, it->ifa_netmask);  */
	  let sin1: *mut sockaddr_in = (*it).ifa_netmask as *mut sockaddr_in;
	  maskp = &raw mut (*sin1).sin_addr as *mut u32;
	  /* DECLARE_POINTER_ALIAS (sin2, struct sockaddr_in, it->ifa_addr);  */
	  let sin2: *mut sockaddr_in = (*it).ifa_addr as *mut sockaddr_in;
	  addrp = &raw mut (*sin2).sin_addr as *mut u32;
	  len = std::mem::size_of::<sockaddr_in> () as c_int;
	  addr_len = 1;
	}
      /* `#ifdef AF_INET6` is live.  */
      else if ((*(*it).ifa_addr).sa_family as c_int == AF_INET6)
	{
	  /* DECLARE_POINTER_ALIAS (sin6_1, struct sockaddr_in6, it->ifa_netmask);  */
	  let sin6_1: *mut sockaddr_in6 = (*it).ifa_netmask as *mut sockaddr_in6;
	  maskp = &raw mut (*sin6_1).sin6_addr as *mut u32;
	  /* DECLARE_POINTER_ALIAS (sin6_2, struct sockaddr_in6, it->ifa_addr);  */
	  let sin6_2: *mut sockaddr_in6 = (*it).ifa_addr as *mut sockaddr_in6;
	  addrp = &raw mut (*sin6_2).sin6_addr as *mut u32;
	  len = std::mem::size_of::<sockaddr_in6> () as c_int;
	  addr_len = 4;
	}
      else
	{ it = (*it).ifa_next; continue; }

      let addr: Lisp_Object = conv_sockaddr_to_lisp ((*it).ifa_addr, len as ptrdiff_t);

      if (full)
	{
	  /* Sometimes sa_family is only filled in correctly in the interface address, not
	     the netmask, so copy it across (Bug#74907).  */
	  (*(*it).ifa_netmask).sa_family = (*(*it).ifa_addr).sa_family;
	  elt = Fcons (conv_sockaddr_to_lisp ((*it).ifa_netmask, len as ptrdiff_t), elt);
	  /* There is an it->ifa_broadaddr field, but its contents are unreliable, so
	     always calculate the broadcast address from the address and the netmask.  */
	  let mut i: c_int;
	  let mut mask: u32;
	  i = 0;
	  while i < addr_len
	    {
	      mask = *maskp.offset (i as isize);
	      *maskp.offset (i as isize) = (*addrp.offset (i as isize) & mask) | !mask;
	      i += 1;
	    }
	  elt = Fcons (conv_sockaddr_to_lisp ((*it).ifa_netmask, len as ptrdiff_t), elt);
	  elt = Fcons (addr, elt);
	}
      else
	{
	  elt = addr;
	}
      res = Fcons (Fcons (build_string ((*it).ifa_name), elt), res);
      it = (*it).ifa_next;
    }
  /* `#ifdef HAVE_FREEIFADDRS` is live.  */
  freeifaddrs (ifap);

  return res;
}

/* `#ifdef HAVE_NET_IF_H` and `#if defined (SIOCGIFADDR) || defined (SIOCGIFHWADDR) ||
   defined (SIOCGIFFLAGS)` are live.  */
/* C 4436-4440: `struct ifflag_def { int flag_bit; const char *flag_sym; };`  */
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ifflag_def
{
  pub flag_bit: c_int,
  pub flag_sym: *const c_char,
}
const _: () = core::assert! (std::mem::size_of::<ifflag_def> () == 16);

/* C 4441-4509: `static const struct ifflag_def ifflag_table[]`.  The five `#ifdef`s that
   are dead on this target are carried as cfg-gated constants below.  */
static mut ifflag_table: [ifflag_def; 17] = [
  ifflag_def { flag_bit: IFF_UP, flag_sym: c"up".as_ptr () },
  ifflag_def { flag_bit: IFF_BROADCAST, flag_sym: c"broadcast".as_ptr () },
  ifflag_def { flag_bit: IFF_DEBUG, flag_sym: c"debug".as_ptr () },
  ifflag_def { flag_bit: IFF_LOOPBACK, flag_sym: c"loopback".as_ptr () },
  ifflag_def { flag_bit: IFF_POINTOPOINT, flag_sym: c"pointopoint".as_ptr () },
  ifflag_def { flag_bit: IFF_RUNNING, flag_sym: c"running".as_ptr () },
  ifflag_def { flag_bit: IFF_NOARP, flag_sym: c"noarp".as_ptr () },
  ifflag_def { flag_bit: IFF_PROMISC, flag_sym: c"promisc".as_ptr () },
  /* C 4468-4473: `#ifdef IFF_NOTRAILERS` with `#ifdef NS_IMPL_COCOA`; the Cocoa arm is dead
     (NS_IMPL_COCOA is `undef` in the frozen config.h), so the `#else` string is live.  */
  ifflag_def { flag_bit: IFF_NOTRAILERS, flag_sym: c"notrailers".as_ptr () },
  ifflag_def { flag_bit: IFF_ALLMULTI, flag_sym: c"allmulti".as_ptr () },
  ifflag_def { flag_bit: IFF_MULTICAST, flag_sym: c"multicast".as_ptr () },
  ifflag_def { flag_bit: IFF_OACTIVE, flag_sym: c"oactive".as_ptr () },
  ifflag_def { flag_bit: IFF_SIMPLEX, flag_sym: c"simplex".as_ptr () },
  ifflag_def { flag_bit: IFF_LINK0, flag_sym: c"link0".as_ptr () },
  ifflag_def { flag_bit: IFF_LINK1, flag_sym: c"link1".as_ptr () },
  ifflag_def { flag_bit: IFF_LINK2, flag_sym: c"link2".as_ptr () },
  ifflag_def { flag_bit: 0, flag_sym: std::ptr::null () },
];
/* C 4483-4485 (`IFF_MASTER`), C 4486-4488 (`IFF_SLAVE`), C 4492-4494 (`IFF_PORTSEL`),
   C 4495-4497 (`IFF_AUTOMEDIA`) and C 4498-4500 (`IFF_DYNAMIC`) are dead on this target.  */
#[cfg (feature = "have_iff_master")]
const IFFFLAG_MASTER: ifflag_def =
  ifflag_def { flag_bit: IFF_MASTER, flag_sym: c"master".as_ptr () };
#[cfg (feature = "have_iff_slave")]
const IFFFLAG_SLAVE: ifflag_def =
  ifflag_def { flag_bit: IFF_SLAVE, flag_sym: c"slave".as_ptr () };
#[cfg (feature = "have_iff_portsel")]
const IFFFLAG_PORTSEL: ifflag_def =
  ifflag_def { flag_bit: IFF_PORTSEL, flag_sym: c"portsel".as_ptr () };
#[cfg (feature = "have_iff_automedia")]
const IFFFLAG_AUTOMEDIA: ifflag_def =
  ifflag_def { flag_bit: IFF_AUTOMEDIA, flag_sym: c"automedia".as_ptr () };
#[cfg (feature = "have_iff_dynamic")]
const IFFFLAG_DYNAMIC: ifflag_def =
  ifflag_def { flag_bit: IFF_DYNAMIC, flag_sym: c"dynamic".as_ptr () };
/* `#ifdef NS_IMPL_COCOA`'s dead arm (feature `ns_impl_cocoa`).  */
#[cfg (feature = "ns_impl_cocoa")]
const IFFFLAG_NOTRAILERS_SMART: ifflag_def =
  ifflag_def { flag_bit: IFF_NOTRAILERS, flag_sym: c"smart".as_ptr () };

/* C 4514-4656: `network_interface_info`.  */
unsafe fn network_interface_info (ifname: Lisp_Object) -> Lisp_Object
{
  let mut rq: ifreq = std::mem::zeroed ();
  let mut res: Lisp_Object = Qnil ();
  let mut elt: Lisp_Object;
  let mut s: c_int;
  let mut any: bool = false;
  /* `#if (! (defined SIOCGIFHWADDR && defined HAVE_STRUCT_IFREQ_IFR_HWADDR) && defined
     HAVE_GETIFADDRS && defined LLADDR)` is live.  */
  let mut ifap: *mut ifaddrs = std::ptr::null_mut ();

  CHECK_STRING (ifname);

  if (std::mem::size_of_val (&rq.ifr_name) as ptrdiff_t <= SBYTES (ifname))
    { error (c"Interface name too long".as_ptr ()); }
  lispstpcpy (rq.ifr_name.as_mut_ptr (), ifname);

  /* C 4532 spells this `socket (AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0)`, i.e. the file's
     own `#define socket(…)` → `process_socket`.  */
  #[cfg (feature = "socket_name_kept")]
  let s: c_int = inrange_fd (socket_c (AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0));
  #[cfg (not (feature = "socket_name_kept"))]
  let s: c_int = inrange_fd (process_socket (AF_INET, SOCK_STREAM | SOCK_CLOEXEC, 0));
  if (s < 0)
    { return Qnil (); }
  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect_int (Some (close_file_unwind), s);

  elt = Qnil ();
  /* `#if defined (SIOCGIFFLAGS) && defined (HAVE_STRUCT_IFREQ_IFR_FLAGS)` is live.  */
  if (ioctl (s, SIOCGIFFLAGS as c_ulong, &raw mut rq) == 0)
    {
      let mut flags: c_int = rq.ifr_ifru.ifr_flags as c_int;
      let mut fp: *const ifflag_def;
      let mut fnum: c_int;

      /* If flags is smaller than int (i.e. short) it may have the high bit set due to
	 IFF_MULTICAST.  In that case, sign extending it into an int is wrong.  */
      if (flags < 0 && std::mem::size_of_val (&rq.ifr_ifru.ifr_flags) < std::mem::size_of_val (&flags))
	{ flags = rq.ifr_ifru.ifr_flags as c_ushort as c_int; }

      any = true;
      fp = (&raw const ifflag_table) as *const ifflag_def;
      while flags != 0 && !(*fp).flag_sym.is_null ()
	{
	  if (flags & (*fp).flag_bit != 0)
	    {
	      elt = Fcons (intern ((*fp).flag_sym), elt);
	      flags -= (*fp).flag_bit;
	    }
	  fp = fp.offset (1);
	}
      fnum = 0;
      while flags != 0 && fnum < 32
	{
	  if (flags & 1 != 0)
	    {
	      elt = Fcons (make_fixnum (fnum as EMACS_INT), elt);
	    }
	  flags >>= 1; fnum += 1;
	}
    }
  res = Fcons (elt, res);

  elt = Qnil ();
  /* C 4560-4575's `#if defined (SIOCGIFHWADDR) && defined (HAVE_STRUCT_IFREQ_IFR_HWADDR)`
     arm; its `#elif defined (HAVE_GETIFADDRS) && defined LLADDR` arm is the live one below.  */
  #[cfg (feature = "no_ifreq_hwaddr")]
  {
    if (ioctl (s, SIOCGIFHWADDR as c_ulong, &raw mut rq) == 0)
      {
	let hwaddr: Lisp_Object = make_uninit_vector (6);
	let p: *mut Lisp_Vector = XVECTOR (hwaddr);

	any = true;
	let mut n: c_int = 0;
	while n < 6
	  {
	    FAM_ELEM! ((*p).contents, n) = make_fixnum (((&raw const rq.ifr_ifru.ifr_hwaddr.sa_data[0]
						       as *const c_char).offset (n as isize)
						      as *const u8).read () as EMACS_INT);
	    n += 1;
	  }
	elt = Fcons (make_fixnum (rq.ifr_ifru.ifr_hwaddr.sa_family as EMACS_INT), hwaddr);
      }
  }
  if (getifaddrs (&raw mut ifap) != -1)
    {
      let hwaddr: Lisp_Object = make_nil_vector (6);
      let p: *mut Lisp_Vector = XVECTOR (hwaddr);

      let mut it: *mut ifaddrs = ifap;
      while it != std::ptr::null_mut ()
	{
	  /* DECLARE_POINTER_ALIAS (sdl, struct sockaddr_dl, it->ifa_addr);  */
	  let sdl: *mut sockaddr_dl = (*it).ifa_addr as *mut sockaddr_dl;
	  let mut linkaddr: [u8; 6] = [0; 6];
	  let mut n: c_int;

	  if ((*(*it).ifa_addr).sa_family as c_int != AF_LINK
	      || strcmp ((*it).ifa_name, SSDATA (ifname) as *const c_char) != 0
	      || (*sdl).sdl_alen as c_int != 6)
	    { it = (*it).ifa_next; continue; }

	  memcpy (linkaddr.as_mut_ptr () as *mut c_void, LLADDR (sdl) as *const c_void,
		  (*sdl).sdl_alen as usize);
	  n = 0;
	  while n < 6
	    {
	      FAM_ELEM! ((*p).contents, n) = make_fixnum (linkaddr[n as usize] as EMACS_INT);
	      n += 1;
	    }

	  elt = Fcons (make_fixnum ((*(*it).ifa_addr).sa_family as EMACS_INT), hwaddr);
	  break;
	}
      it = ifap; let _ = it;
      /* `#ifdef HAVE_FREEIFADDRS` is live.  */
      freeifaddrs (ifap);
    }

  res = Fcons (elt, res);

  elt = Qnil ();
  /* `#if (defined SIOCGIFNETMASK && (defined HAVE_STRUCT_IFREQ_IFR_NETMASK ||
     defined HAVE_STRUCT_IFREQ_IFR_ADDR))` is live; the `#ifdef
     HAVE_STRUCT_IFREQ_IFR_NETMASK` arm inside it is dead (the frozen config.h records it
     `undef`), so the `#else`'s `ifr_addr` spelling is the live one.  */
  if (ioctl (s, SIOCGIFNETMASK as c_ulong, &raw mut rq) == 0)
    {
      any = true;
      /* C 4630-4635: `#ifdef HAVE_STRUCT_IFREQ_IFR_NETMASK` (dead here) and its
	 `#else` arm, which is the live `ifr_addr` spelling below.  */
      #[cfg (feature = "have_ifreq_netmask")]
      {
	elt = conv_sockaddr_to_lisp (&raw mut rq.ifr_ifru.ifr_netmask,
				     std::mem::size_of_val (&rq.ifr_ifru.ifr_netmask) as ptrdiff_t);
      }
      #[cfg (not (feature = "have_ifreq_netmask"))]
      {
	elt = conv_sockaddr_to_lisp (&raw mut rq.ifr_ifru.ifr_addr,
				     std::mem::size_of_val (&rq.ifr_ifru.ifr_addr) as ptrdiff_t);
      }
    }
  res = Fcons (elt, res);

  elt = Qnil ();
  /* `#if defined (SIOCGIFBRDADDR) && defined (HAVE_STRUCT_IFREQ_IFR_BROADADDR)` is live.  */
  if (ioctl (s, SIOCGIFBRDADDR as c_ulong, &raw mut rq) == 0)
    {
      any = true;
      elt = conv_sockaddr_to_lisp (&raw mut rq.ifr_ifru.ifr_broadaddr,
				   std::mem::size_of_val (&rq.ifr_ifru.ifr_broadaddr) as ptrdiff_t);
    }
  res = Fcons (elt, res);

  elt = Qnil ();
  /* `#if defined (SIOCGIFADDR) && defined (HAVE_STRUCT_IFREQ_IFR_ADDR)` is live.  */
  if (ioctl (s, SIOCGIFADDR as c_ulong, &raw mut rq) == 0)
    {
      any = true;
      elt = conv_sockaddr_to_lisp (&raw mut rq.ifr_ifru.ifr_addr,
				   std::mem::size_of_val (&rq.ifr_ifru.ifr_addr) as ptrdiff_t);
    }
  res = Fcons (elt, res);

  return unbind_to (count, if any { res } else { Qnil () });
}

/// C: `<net/if_dl.h>`: `#define LLADDR(s) ((s)->sdl_data + (s)->sdl_nlen)`.
#[inline(always)]
unsafe fn LLADDR (sdl: *mut sockaddr_dl) -> *mut u8
{ ((*sdl).sdl_data.as_mut_ptr () as *mut u8).offset ((*sdl).sdl_nlen as isize) }

/// C: `#define AF_LINK 18` (`<sys/socket.h>`)
pub const AF_LINK: c_int = 18;
/// C: `#define IFF_UP 1` (`<net/if.h>`)
pub const IFF_UP: c_int = 1;
/// C: `#define IFF_BROADCAST 2` (`<net/if.h>`)
pub const IFF_BROADCAST: c_int = 2;
/// C: `#define IFF_DEBUG 4` (`<net/if.h>`)
pub const IFF_DEBUG: c_int = 4;
/// C: `#define IFF_LOOPBACK 8` (`<net/if.h>`)
pub const IFF_LOOPBACK: c_int = 8;
/// C: `#define IFF_POINTOPOINT 16` (`<net/if.h>`)
pub const IFF_POINTOPOINT: c_int = 16;
/// C: `#define IFF_NOTRAILERS 32` (`<net/if.h>`)
pub const IFF_NOTRAILERS: c_int = 32;
/// C: `#define IFF_RUNNING 64` (`<net/if.h>`)
pub const IFF_RUNNING: c_int = 64;
/// C: `#define IFF_NOARP 128` (`<net/if.h>`)
pub const IFF_NOARP: c_int = 128;
/// C: `#define IFF_PROMISC 256` (`<net/if.h>`)
pub const IFF_PROMISC: c_int = 256;
/// C: `#define IFF_ALLMULTI 512` (`<net/if.h>`)
pub const IFF_ALLMULTI: c_int = 512;
/// C: `#define IFF_OACTIVE 1024` (`<net/if.h>`)
pub const IFF_OACTIVE: c_int = 1024;
/// C: `#define IFF_SIMPLEX 2048` (`<net/if.h>`)
pub const IFF_SIMPLEX: c_int = 2048;
/// C: `#define IFF_LINK0 4096` (`<net/if.h>`)
pub const IFF_LINK0: c_int = 4096;
/// C: `#define IFF_LINK1 8192` (`<net/if.h>`)
pub const IFF_LINK1: c_int = 8192;
/// C: `#define IFF_LINK2 16384` (`<net/if.h>`)
pub const IFF_LINK2: c_int = 16384;
/// C: `#define IFF_MULTICAST 32768` (`<net/if.h>`)
pub const IFF_MULTICAST: c_int = 32768;
/// C: `#define SIOCGIFCONF 3222038820` (`<sys/ioctl.h>`)
pub const SIOCGIFCONF: c_int = 3222038820u32 as c_int;
/// C: `#define SIOCGIFFLAGS 3223349521` (`<sys/ioctl.h>`)
pub const SIOCGIFFLAGS: c_int = 3223349521u32 as c_int;
/// C: `#define SIOCGIFADDR 3223349537` (`<sys/ioctl.h>`)
pub const SIOCGIFADDR: c_int = 3223349537u32 as c_int;
/// C: `#define SIOCGIFNETMASK 3223349541` (`<sys/ioctl.h>`)
pub const SIOCGIFNETMASK: c_int = 3223349541u32 as c_int;
/// C: `#define SIOCGIFBRDADDR 3223349539` (`<sys/ioctl.h>`)
pub const SIOCGIFBRDADDR: c_int = 3223349539u32 as c_int;

unsafe extern "C" {
  /// C: int getifaddrs (struct ifaddrs **)   (<ifaddrs.h>)
  fn getifaddrs (ifap: *mut *mut ifaddrs) -> c_int;
  /// C: void freeifaddrs (struct ifaddrs *)   (<ifaddrs.h>)
  fn freeifaddrs (ifa: *mut ifaddrs);
}

DEFUN! ("network-interface-list", Fnetwork_interface_list, Snetwork_interface_list, 0, 2, 0,
       doc: "Return an alist of all network interfaces and their network address.\nEach element is a cons of the form (IFNAME . IP) where IFNAME is a\nstring containing the interface name, and IP is the network address in\ninternal format; see the description of ADDRESS in\n`make-network-process'.  The interface name is not guaranteed to be\nunique.\n\nOptional parameter FULL non-nil means return all IP address info for\neach interface.  Each element is then a list of the form\n    (IFNAME IP BCAST MASK)\nwhere IFNAME is the interface name, IP the IP address,\nBCAST the broadcast address, and MASK the network mask.\n\nOptional parameter FAMILY controls the type of addresses to return.\nThe default of nil means both IPv4 and IPv6, symbol `ipv4' means IPv4\nonly, symbol `ipv6' means IPv6 only.\n\nSee also `network-interface-info', which is limited to IPv4 only.\n\nIf the information is not available, return nil.",
       (full: Lisp_Object, family: Lisp_Object),
       {
  /* `#if defined HAVE_GETIFADDRS || defined WINDOWSNT` is live.  */
  let mut r#match: c_ushort;
  let mut full_info: bool = false;

  if (! NILP (full))
    { full_info = true; }
  if (NILP (family))
    { r#match = 0; }
  else if (EQ (family, Qipv4 ()))
    { r#match = AF_INET as c_ushort; }
  /* `#ifdef AF_INET6` is live.  */
  else if (EQ (family, Qipv6 ()))
    { r#match = AF_INET6 as c_ushort; }
  else
    { error (c"Unsupported address family".as_ptr ()); }
  return network_interface_list (full_info, r#match);
  /* C 4696-4700's `#else` arm of `#if defined HAVE_GETIFADDRS || defined WINDOWSNT`.  */
  #[cfg (feature = "no_getifaddrs")]
  { return Qnil (); }
});

DEFUN! ("network-interface-info", Fnetwork_interface_info, Snetwork_interface_info, 1, 1, 0,
       doc: "Return information about network interface named IFNAME.\nThe return value is a list (ADDR BCAST NETMASK HWADDR FLAGS),\nwhere ADDR is the layer 3 address, BCAST is the layer 3 broadcast address,\nNETMASK is the layer 3 network mask, HWADDR is the layer 2 address, and\nFLAGS is the current flags of the interface.\n\nData that is unavailable is returned as nil.  Only returns IPv4 layer 3\naddresses, for IPv6 use `network-interface-list'.",
       (ifname: Lisp_Object),
       {
  /* `#if ((defined HAVE_NET_IF_H && (defined SIOCGIFADDR || defined SIOCGIFHWADDR ||
     defined SIOCGIFFLAGS)) || defined WINDOWSNT)` is live.  */
  return network_interface_info (ifname);
  /* C 4722-4727's `#else` arm.  */
  #[cfg (feature = "no_net_if")]
  { return Qnil (); }
});

/* C 4729-4765: `network_lookup_address_info_1`.  */
unsafe fn network_lookup_address_info_1 (host: Lisp_Object, service: *const c_char,
					 hints: *mut addrinfo,
					 res: *mut *mut addrinfo) -> Lisp_Object
{
  let mut msg: Lisp_Object = Qt ();
  let mut ret: c_int;
  let mut service: *const c_char = service;

  if (STRING_MULTIBYTE (host) && SBYTES (host) != SCHARS (host))
    { error (c"Non-ASCII hostname %s detected, please use `puny-encode-domain'".as_ptr (),
	     SSDATA (host)); }

  /* C 4740-4743's `#ifdef WINDOWSNT` arm: ensure socket support is loaded if
     available.  */
  #[cfg (feature = "windowsnt")]
  {
    init_winsock (true);
  }

  ret = getaddrinfo (SSDATA (host) as *const c_char, service, hints, res);
  if (ret != 0)
    {
      if (service == std::ptr::null ())
	{ service = c"0".as_ptr (); }
      /* `#ifdef HAVE_GAI_STRERROR` is live.  */
      {
	synchronize_system_messages_locale ();
	let mut str_: *const c_char = gai_strerror (ret);
	if (! NILP (Vlocale_coding_system))
	  { str_ = SSDATA (code_convert_string_norecord
			   (build_string (str_), Vlocale_coding_system, false)) as *const c_char; }
	AUTO_STRING! (format, c"%s/%s %s".as_ptr ());
	msg = CALLN! (Fformat, format, host, build_string (service),
		      build_string (str_));
      }
      /* C 4754-4759's `#else` arm of `#ifdef HAVE_GAI_STRERROR` (the macro is defined on
	 this target, so that arm is the dead one).  */
      #[cfg (feature = "no_gai_strerror")]
      {
	AUTO_STRING! (format, c"%s/%s getaddrinfo error %d".as_ptr ());
	msg = CALLN! (Fformat, format, host, build_string (service),
		      make_int (ret));
      }
    }
  return msg;
}

DEFUN! ("network-lookup-address-info", Fnetwork_lookup_address_info,
       Snetwork_lookup_address_info, 1, 3, 0,
       doc: "Look up Internet Protocol (IP) address info of NAME.\nNAME must be an ASCII-only string.  For looking up internationalized\nhostnames, use `puny-encode-domain' on the string first.\n\nOptional argument FAMILY controls whether to look up IPv4 or IPv6\naddresses.  The default of nil means both, symbol `ipv4' means IPv4\nonly, symbol `ipv6' means IPv6 only.\nOptional argument HINTS allows specifying the hints passed to the\nunderlying library call.  The only supported value is `numeric', which\nmeans treat NAME as a numeric IP address.  This also suppresses DNS\ntraffic.\nReturn a list of addresses, or nil if none were found.  Each address\nis a vector of integers, as per the description of ADDRESS in\n`make-network-process'.  In case of error log the error message\nreturned from the lookup.",
       (name: Lisp_Object, family: Lisp_Object, hint: Lisp_Object),
       {
  let mut addresses: Lisp_Object = Qnil ();
  let mut msg: Lisp_Object = Qnil ();

  let mut res: *mut addrinfo = std::ptr::null_mut ();
  let mut lres: *mut addrinfo;
  let mut hints: addrinfo = std::mem::zeroed ();

  CHECK_STRING (name);

  memset (&raw mut hints as *mut c_void, 0, std::mem::size_of_val (&hints));
  if (NILP (family))
    { hints.ai_family = AF_UNSPEC; }
  else if (EQ (family, Qipv4 ()))
    { hints.ai_family = AF_INET; }
  /* `#ifdef AF_INET6` is live.  */
  else if (EQ (family, Qipv6 ()))
    { hints.ai_family = AF_INET6; }
  else
    { error (c"Unsupported family".as_ptr ()); }
  hints.ai_socktype = SOCK_DGRAM;

  if (EQ (hint, Qnumeric ()))
    { hints.ai_flags = AI_NUMERICHOST; }
  else if (!NILP (hint))
    { error (c"Unsupported hints value".as_ptr ()); }

  msg = network_lookup_address_info_1 (name, std::ptr::null (), &raw mut hints, &raw mut res);
  if (!EQ (msg, Qt ()))
    { message (c"%s".as_ptr (), SSDATA (msg)); }
  else
    {
      lres = res;
      while !lres.is_null ()
	{
	  /* Avoid converting non-IP addresses (Bug#74907).  */
	  if ((*lres).ai_family == AF_INET
	      /* `#ifdef AF_INET6` is live.  */
	      || (*lres).ai_family == AF_INET6)
	    { addresses = Fcons (conv_sockaddr_to_lisp ((*lres).ai_addr,
							(*lres).ai_addrlen as ptrdiff_t),
				 addresses); }
	  else
	    { lres = (*lres).ai_next; continue; }
	  lres = (*lres).ai_next;
	}
      addresses = Fnreverse (addresses);

      freeaddrinfo (res);
    }
  return addresses;
});

/* Turn off input and output for process PROC.  */
/* C 4842-4890: `deactivate_process`.  */
unsafe fn deactivate_process (proc_: Lisp_Object)
{
  let inchannel: c_int;
  let p: *mut Lisp_Process = XPROCESS (proc_);
  let mut i: c_int;

  /* C 4849-4852's `#ifdef HAVE_GNUTLS` arm: delete GnuTLS structures in PROC, if any.  */
  #[cfg (feature = "have_gnutls")]
  {
    emacs_gnutls_deinit (proc_);
  }

  if ((*p).read_output_delay > 0)
    {
      process_output_delay_count -= 1;
      if (process_output_delay_count < 0)
	{ process_output_delay_count = 0; }
      (*p).read_output_delay = 0;
      process_bits! (read_output_skip, p, 0);
    }

  /* Beware SIGCHLD hereabouts.  */

  i = 0;
  while i < PROCESS_OPEN_FDS
    {
      if ((*p).open_fd[i as usize] >= 0)
	{
	  fd_callback_info[(*p).open_fd[i as usize] as usize].thread = std::ptr::null_mut ();
	  fd_callback_info[(*p).open_fd[i as usize] as usize].waiting_thread = std::ptr::null_mut ();
	}
      close_process_fd ((*p).open_fd.as_mut_ptr ().offset (i as isize));
      i += 1;
    }

  inchannel = (*p).infd;
  eassert! (inchannel < FD_SETSIZE);
  if (inchannel >= 0)
    {
      (*p).infd = -1;
      (*p).outfd = -1;
      /* `#ifdef DATAGRAM_SOCKETS` is live.  */
      if (DATAGRAM_CHAN_P (inchannel))
	{
	  xfree (datagram_address[inchannel as usize].sa as *mut c_void);
	  datagram_address[inchannel as usize].sa = std::ptr::null_mut ();
	  datagram_address[inchannel as usize].len = 0;
	}
      chan_process[inchannel as usize] = Qnil ();
      delete_read_fd (inchannel);
      if ((fd_callback_info[inchannel as usize].flags & NON_BLOCKING_CONNECT_FD) != 0)
	{ delete_write_fd (inchannel); }
    }
}

unsafe extern "C" {
  /// C: int getaddrinfo (const char *, const char *, const struct addrinfo *,
  ///                     struct addrinfo **)   (<netdb.h>)
  fn getaddrinfo (name: *const c_char, service: *const c_char, hints: *const addrinfo,
		  res: *mut *mut addrinfo) -> c_int;
  /// C: const char *gai_strerror (int)   (<netdb.h>)
  fn gai_strerror (errcode: c_int) -> *const c_char;
  /// C: void emacs_gnutls_deinit (Lisp_Object)   (gnutls.c, the `HAVE_GNUTLS` arm)
  fn emacs_gnutls_deinit (proc_: Lisp_Object);
}

/// C: `#define AI_NUMERICHOST 4` (`<netdb.h>`)
pub const AI_NUMERICHOST: c_int = 4;

DEFUN! ("accept-process-output", Faccept_process_output, Saccept_process_output, 0, 4, 0,
       doc: "Allow any pending output from subprocesses to be read by Emacs.\nThe subprocess output is given to the respective process filter functions.\nOptional argument PROCESS means to return only after output is\nreceived from PROCESS or PROCESS closes the connection.\n\nOptional second argument SECONDS and third argument MILLISEC\nspecify a timeout; return after that much time even if there is\nno subprocess output.  If SECONDS is a floating point number,\nit specifies a fractional number of seconds to wait.\nThe MILLISEC argument is obsolete and should be avoided.\n\nIf optional fourth argument JUST-THIS-ONE is non-nil, accept output\nfrom PROCESS only, suspending reading output from other processes.\nIf JUST-THIS-ONE is an integer, don't run any timers either.\nReturn non-nil if we received any output from PROCESS (or, if PROCESS\nis nil, from any process) before the timeout expired or the\ncorresponding connection was closed.\n\nNote that it is not guaranteed that this function will return as\nsoon as some output is received.  In particular, if PROCESS is nil,\nthe function should not be expected to return before the timeout\nexpires.  The main purpose of this function is to allow process output\nto be read by Emacs, not to return as soon as any output is read.",
       (process: Lisp_Object, seconds: Lisp_Object, millisec: Lisp_Object,
	just_this_one: Lisp_Object),
       {
  {
    static mut apo_entered: bool = false;
    if ! apo_entered && ! getenv (c"FEMACS_CHILD_TRACE".as_ptr ()).is_null ()
      {
        apo_entered = true;
        fprintf (stderr, c"accept-apo-enter\n".as_ptr ());
      }
  }

  let mut secs: EMACS_INT;
  let mut nsecs: c_int;
  let mut seconds: Lisp_Object = seconds;
  let mut just_this_one: Lisp_Object = just_this_one;

  if (! NILP (process))
    {
      CHECK_PROCESS (process);
      let proc: *mut Lisp_Process = XPROCESS (process);

      /* Can't wait for a process that is dedicated to a different thread.  */
      if (!NILP ((*proc).thread) && !BASE_EQ ((*proc).thread, Fcurrent_thread ()))
	{
	  let proc_thread_name: Lisp_Object = (*XTHREAD ((*proc).thread)).name;

	  error (c"Attempt to accept output from process %s locked to thread %s".as_ptr (),
		 SDATA ((*proc).name),
		 if STRINGP (proc_thread_name)
		 { SDATA (proc_thread_name) }
		 else { SDATA (Fprin1_to_string ((*proc).thread, Qt (), Qnil ())) });
	}
    }
  else
    { just_this_one = Qnil (); }

  if (!NILP (millisec))
    { /* Obsolete calling convention using integers rather than floats.  */
      CHECK_FIXNUM (millisec);
      if (NILP (seconds))
	{ seconds = make_float (XFIXNUM (millisec) as f64 / 1000.0); }
      else
	{
	  CHECK_FIXNUM (seconds);
	  seconds = make_float (XFIXNUM (millisec) as f64 / 1000.0 + XFIXNUM (seconds) as f64);
	}
    }

  secs = 0;
  nsecs = -1;

  if (!NILP (seconds))
    {
      if (FIXNUMP (seconds))
	{
	  if (XFIXNUM (seconds) > 0)
	    {
	      secs = XFIXNUM (seconds);
	      nsecs = 0;
	    }
	}
      else if (FLOATP (seconds))
	{
	  if (XFLOAT_DATA (seconds) > 0.0)
	    {
	      let t: timespec = dtotimespec (XFLOAT_DATA (seconds));
	      secs = std::cmp::min (t.tv_sec as EMACS_INT, WAIT_READING_MAX as EMACS_INT);
	      nsecs = t.tv_nsec as c_int;
	    }
	}
      else
	{ wrong_type_argument (Qnumberp (), seconds); }
    }
  else if (! NILP (process))
    { nsecs = 0; }

  return
    if (wait_reading_process_output (secs as i64, nsecs, 0, 0,
				     Qnil (),
				     if !NILP (process) { XPROCESS (process) }
				     else { std::ptr::null_mut () },
				     if NILP (just_this_one) { 0 }
				     else if !FIXNUMP (just_this_one) { 1 } else { -1 })
	<= 0)
    { Qnil () } else { Qt () };
});

/* Accept a connection for server process SERVER on CHANNEL.  */

/* C 4998: `static EMACS_INT connect_counter = 0;`  */
static mut connect_counter: EMACS_INT = 0;

/* C 5001-5204: `server_accept_connection`.  */
unsafe fn server_accept_connection (server: Lisp_Object, channel: c_int)
{
  let mut buffer: Lisp_Object;
  let mut contact: Lisp_Object;
  let mut host: Lisp_Object;
  let mut service: Lisp_Object;
  let ps: *mut Lisp_Process = XPROCESS (server);
  let mut p: *mut Lisp_Process;
  let mut s: c_int;
  let mut saddr: u_sockaddr = std::mem::zeroed ();
  let mut len: socklen_t = std::mem::size_of_val (&saddr) as socklen_t;

  s = inrange_fd (process_accept4 (channel, &raw mut saddr.sa, &raw mut len, SOCK_CLOEXEC));

  if (s < 0)
    {
      let code: c_int = *errno ();
      if (!would_block (code) && !NILP ((*ps).log))
	{ calln! ((*ps).log, server, Qnil (),
		  concat3 (build_string (c"accept failed with code".as_ptr ()),
			   Fnumber_to_string (make_fixnum (code as EMACS_INT)),
			   build_string (c"\n".as_ptr ()))); }
      return;
    }

  let count: specpdl_ref = SPECPDL_INDEX ();
  record_unwind_protect_int (Some (close_file_unwind), s);

  connect_counter += 1;

  /* Setup a new process to handle the connection.  */

  /* Generate a unique identification of the caller, and build contact information for this
     process.  */
  host = Qt ();
  service = Qnil ();
  let mut args: [Lisp_Object; 11] = [lisp_h_Qnil (); 11];
  let mut nargs: c_int = 0;
  /* C 5037-5038: `#define HOST_FORMAT_IN "%d.%d.%d.%d"` and
     `#define HOST_FORMAT_IN6 "%x:%x:%x:%x:%x:%x:%x:%x"` — string-only macros, so their Rust
     counterparts carry the same names and C's literals are spelled `c"…"` at the call
     sites below (an `AUTO_STRING` needs a NUL-terminated literal).  */
  /// C 5037: `#define HOST_FORMAT_IN "%d.%d.%d.%d"` — a string constant.
  const HOST_FORMAT_IN: &str = "%d.%d.%d.%d";
  /// C 5038: `#define HOST_FORMAT_IN6 "%x:%x:%x:%x:%x:%x:%x:%x"` — a string constant.
  const HOST_FORMAT_IN6: &str = "%x:%x:%x:%x:%x:%x:%x:%x";
  AUTO_STRING! (host_format_in, c"%d.%d.%d.%d".as_ptr ());
  AUTO_STRING! (host_format_in6, c"%x:%x:%x:%x:%x:%x:%x:%x".as_ptr ());
  AUTO_STRING! (procname_format_in, c"%s <%d.%d.%d.%d:%d>".as_ptr ());
  AUTO_STRING! (procname_format_in6, c"%s <[%x:%x:%x:%x:%x:%x:%x:%x]:%d>".as_ptr ());
  AUTO_STRING! (procname_format_default, c"%s <%d>".as_ptr ());
  match saddr.sa.sa_family as c_int
    {
    x if x == AF_INET =>
      {
	args[nargs as usize] = procname_format_in; nargs += 1;
	args[nargs as usize] = host_format_in; nargs += 1;
	let ip: *mut u8 = &raw mut saddr.r#in.sin_addr.s_addr as *mut u8;
	service = make_fixnum (ntohs (saddr.r#in.sin_port as u16) as EMACS_INT);
	let mut i: c_int = 0;
	while i < 4
	  {
	    args[nargs as usize] = make_fixnum (*ip.offset (i as isize) as EMACS_INT); nargs += 1;
	    i += 1;
	  }
	host = Fformat (5, args.as_mut_ptr ().offset (1));
	args[nargs as usize] = service; nargs += 1;
      }
    /* `#ifdef AF_INET6` is live.  */
    x if x == AF_INET6 =>
      {
	args[nargs as usize] = procname_format_in6; nargs += 1;
	args[nargs as usize] = host_format_in6; nargs += 1;
	/* DECLARE_POINTER_ALIAS (ip6, uint16_t, &saddr.in6.sin6_addr);  */
	let ip6: *mut u16 = &raw mut saddr.in6.sin6_addr as *mut u16;
	service = make_fixnum (ntohs (saddr.r#in.sin_port as u16) as EMACS_INT);
	let mut i: c_int = 0;
	while i < 8
	  {
	    args[nargs as usize] = make_fixnum (*ip6.offset (i as isize) as EMACS_INT); nargs += 1;
	    i += 1;
	  }
	host = Fformat (9, args.as_mut_ptr ().offset (1));
	args[nargs as usize] = service; nargs += 1;
      }
    _ =>
      {
	args[nargs as usize] = procname_format_default; nargs += 1;
	nargs += 1;
	args[nargs as usize] = make_fixnum (connect_counter); nargs += 1;
      }
    }

  /* Create a new buffer name for this process if it doesn't have a filter.  The new buffer
     name is based on the buffer name or process name of the server process concatenated with
     the caller identification.  */

  if (!(EQ ((*ps).filter, Qinternal_default_process_filter ())
	|| EQ ((*ps).filter, Qt ())))
    { buffer = Qnil (); }
  else
    {
      buffer = (*ps).buffer;
      if (!NILP (buffer))
	{ buffer = Fbuffer_name (buffer); }
      else
	{ buffer = (*ps).name; }
      if (!NILP (buffer))
	{
	  args[1] = buffer;
	  buffer = Fget_buffer_create (Fformat (nargs as isize, args.as_mut_ptr ()), Qnil ());
	}
    }

  /* Generate a unique name for the new server process.  Combine the server process name
     with the caller identification.  */

  args[1] = (*ps).name;
  let name: Lisp_Object = Fformat (nargs as isize, args.as_mut_ptr ());
  let proc_: Lisp_Object = make_process (name);

  eassert! (0 <= s && s < FD_SETSIZE);
  chan_process[s as usize] = proc_;

  fcntl (s, F_SETFL, O_NONBLOCK);

  p = XPROCESS (proc_);
  /* make_process calls pset_thread, but if the server process is not locked to any thread,
     we need to undo what make_process did.  */
  if (NILP ((*ps).thread))
    { pset_thread (p, Qnil ()); }

  /* Build new contact information for this setup.  */
  contact = Fcopy_sequence ((*ps).childp);
  contact = plist_put (contact, QCserver (), Qnil ());
  contact = plist_put (contact, QChost (), host);
  if (!NILP (service))
    { contact = plist_put (contact, QCservice (), service); }
  contact = plist_put (contact, QCremote (),
		       conv_sockaddr_to_lisp (&raw mut saddr.sa, len as ptrdiff_t));
  /* `#ifdef HAVE_GETSOCKNAME` is live.  */
  len = std::mem::size_of_val (&saddr) as socklen_t;
  if (getsockname (s, &raw mut saddr.sa, &raw mut len) == 0)
    { contact = plist_put (contact, QClocal (),
			   conv_sockaddr_to_lisp (&raw mut saddr.sa, len as ptrdiff_t)); }

  pset_childp (p, contact);
  pset_plist (p, Fcopy_sequence ((*ps).plist));
  pset_type (p, Qnetwork ());

  pset_buffer (p, buffer);
  pset_sentinel (p, (*ps).sentinel);
  pset_filter (p, (*ps).filter);
  eassert! (NILP ((*p).command));
  eassert! ((*p).pid == 0);

  /* Discard the unwind protect for closing S.  */
  (specpdl_ptr! ()) = specpdl_ref_to_ptr (count);

  (*p).open_fd[SUBPROCESS_STDIN as usize] = s;
  (*p).infd = s;
  (*p).outfd = s;
  pset_status (p, Qrun ());

  /* Client processes for accepted connections are not stopped initially.  */
  if (!EQ ((*p).filter, Qt ()))
    { add_process_read_fd (s); }
  if (s > max_desc)
    { max_desc = s; }
  /* If the server process is locked to this thread, lock the client process to the same
     thread, otherwise clear the thread of its I/O descriptors.  */
  if (NILP ((*ps).thread))
    {
      eassert! (fd_callback_info[(*p).infd as usize].thread.is_null ());
      set_proc_thread (p, std::ptr::null_mut ());
    }
  else
    {
      eassert! (fd_callback_info[(*p).infd as usize].thread.is_null ()
		|| fd_callback_info[(*p).infd as usize].thread == XTHREAD ((*ps).thread));
      eassert! (XTHREAD ((*ps).thread) == current_thread);
      set_proc_thread (p, XTHREAD ((*ps).thread));
    }

  /* Setup coding system for new process based on server process.  This seems to be the
     proper thing to do, as the coding system of the new process should reflect the settings
     at the time the server socket was opened; not the current settings.  */

  pset_decode_coding_system (p, (*ps).decode_coding_system);
  pset_encode_coding_system (p, (*ps).encode_coding_system);
  setup_process_coding_systems (proc_);

  pset_decoding_buf (p, empty_unibyte_string);
  eassert! ((*p).decoding_carryover == 0);
  pset_encoding_buf (p, empty_unibyte_string);

  process_bits! (inherit_coding_system_flag, p,
		 if NILP (buffer) { 0 } else { process_bits! (inherit_coding_system_flag, ps) as c_int });

  AUTO_STRING! (dash, c"-".as_ptr ());
  AUTO_STRING! (nl, c"\n".as_ptr ());
  let host_string: Lisp_Object = if STRINGP (host) { host } else { dash };

  if (!NILP ((*ps).log))
    {
      AUTO_STRING! (accept_from, c"accept from ".as_ptr ());
      calln! ((*ps).log, server, proc_, concat3 (accept_from, host_string, nl));
    }

  AUTO_STRING! (open_from, c"open from ".as_ptr ());
  exec_sentinel (proc_, concat3 (open_from, host_string, nl));
}

/* C 5207-5250: `check_for_dns` — the `HAVE_GETADDRINFO_A` arm (dead on this target).  */
#[cfg (feature = "have_getaddrinfo_a")]
unsafe fn check_for_dns (proc_: Lisp_Object) -> Lisp_Object
{
  let p: *mut Lisp_Process = XPROCESS (proc_);
  let mut addrinfos: Lisp_Object = Qnil ();

  /* Sanity check.  */
  if ((*p).dns_request.is_null ())
    { return Qnil (); }

  let ret: c_int = gai_error ((*p).dns_request);
  if (ret == EAI_INPROGRESS)
    { return Qt (); }

  /* We got a response.  */
  if (ret == 0)
    {
      let mut res: *mut addrinfo;

      res = (*(*p).dns_request).ar_result;
      while !res.is_null ()
	{
	  addrinfos = Fcons (conv_addrinfo_to_lisp (res), addrinfos);
	  res = (*res).ai_next;
	}

      addrinfos = Fnreverse (addrinfos);
    }
  /* The DNS lookup failed.  */
  else if (connecting_status ((*p).status))
    {
      deactivate_process (proc_);
      pset_status (p, list2
		   (Qfailed (),
		    concat3 (build_string (c"Name lookup of ".as_ptr ()),
			     build_string ((*(*p).dns_request).ar_name),
			     build_string (c" failed".as_ptr ()))));
    }

  free_dns_request (proc_);

  /* This process should not already be connected (or killed).  */
  if (! connecting_status ((*p).status))
    { return Qnil (); }

  return addrinfos;
}

/* C 5253-5262: `wait_for_socket_fds`.  */
unsafe fn wait_for_socket_fds (process: Lisp_Object, name: *const c_char)
{
  while ((*XPROCESS (process)).infd < 0
	 && connecting_status ((*XPROCESS (process)).status))
    {
      add_to_log (c"Waiting for socket from %s...".as_ptr (), build_string (name));
      wait_reading_process_output (0, 20 * 1000 * 1000, 0, 0, Qnil (),
				   std::ptr::null_mut (), 0);
    }
}

/* C 5264-5272: `wait_while_connecting`.  */
unsafe fn wait_while_connecting (process: Lisp_Object)
{
  while (connecting_status ((*XPROCESS (process)).status))
    {
      add_to_log (c"Waiting for connection...".as_ptr ());
      wait_reading_process_output (0, 20 * 1000 * 1000, 0, 0, Qnil (),
				   std::ptr::null_mut (), 0);
    }
}

/* C 5274-5285: `wait_for_tls_negotiation` — its body is the `HAVE_GNUTLS` arm (dead on this
   target), but the function itself is called unconditionally from `send_process`.  */
unsafe fn wait_for_tls_negotiation (process: Lisp_Object)
{
  #[cfg (feature = "have_gnutls")]
  while (process_bits! (gnutls_p, XPROCESS (process))
	 && (*XPROCESS (process)).gnutls_initstage != GNUTLS_STAGE_READY)
    {
      add_to_log (c"Waiting for TLS...".as_ptr ());
      wait_reading_process_output (0, 20 * 1000 * 1000, 0, 0, Qnil (),
				   std::ptr::null_mut (), 0);
    }
}

/* C 5287-5292: `wait_reading_process_output_unwind` — its address is taken by
   `record_unwind_protect_int`, so it is `unsafe extern "C" fn` (PORTING.md 1.4).  */
unsafe extern "C" fn wait_reading_process_output_unwind (data: c_int)
{
  clear_waiting_thread_info ();
  waiting_for_user_input_p! () = data;
}

/* This is here so breakpoints can be put on it.  */
/* C 5295-5297: `wait_reading_process_output_1`.  */
unsafe fn wait_reading_process_output_1 ()
{
}

/* C 5299-5317: `android_select_wrapper` — the `HAVE_ANDROID && !ANDROID_STUBIFY &&
   THREADS_ENABLED` arm (dead on this target).  */
#[cfg (feature = "have_android")]
unsafe fn android_select_wrapper (nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set,
				  exceptfds: *mut fd_set, timeout: *const timespec,
				  sigmask: *const sigset_t) -> c_int
{
  /* sigmask is not supported.  */
  if (!sigmask.is_null ())
    { emacs_abort (); }

  return android_select (nfds, readfds, writefds, exceptfds,
			 timeout as *mut timespec);
}

unsafe extern "C" {
  /// C: void add_to_log (char const *, ...)   (lisp.h 4378)
  fn add_to_log (format: *const c_char, ...);
  /// C: int gai_error (struct gaicb *)   (<netdb.h>, glibc only — the `HAVE_GETADDRINFO_A` arm)
  fn gai_error (req: *mut gaicb) -> c_int;
  /// C: int android_select (int, fd_set *, fd_set *, fd_set *, struct timespec *)
  ///    (android.c, the `HAVE_ANDROID` arm)
  fn android_select (nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set,
		     exceptfds: *mut fd_set, timeout: *mut timespec) -> c_int;
}

/// C: `#define EAI_INPROGRESS -100` (`<netdb.h>`, glibc only)
pub const EAI_INPROGRESS: c_int = -100;

/* C 5358-6219: `wait_reading_process_output`.  `thread_select` dispatches through the
   `select_func` the mirror declares; the `HAVE_GNUTLS`/`HAVE_GETADDRINFO_A`/`HAVE_ANDROID`
   arms are carried text.  */
pub const MINIMUM: c_int = -1;
pub const TIMEOUT: c_int = 0;
pub const FOREVER: c_int = 1;

#[unsafe(no_mangle)]
pub unsafe fn wait_reading_process_output (time_limit: i64, nsecs: c_int, read_kbd: c_int,
					   /* C: `bool do_display`; the C call sites pass the
					      integer literals 0/1, and the token check
					      requires them, so the port keeps `c_int`
					      (recorded in the ledger's Decisions).  */
					   do_display: c_int,
					   wait_for_cell: Lisp_Object,
					   wait_proc: *mut Lisp_Process,
					   just_wait_proc: c_int) -> c_int
{
  /* PORTING.md P-122 ⑷'s instrument: is this the path `accept-process-output` takes at all?
     Printed once per run, off unless FEMACS_CHILD_TRACE is set.  */
  {
    static mut wait_entered: bool = false;
    if ! wait_entered && ! getenv (c"FEMACS_CHILD_TRACE".as_ptr ()).is_null ()
      {
        wait_entered = true;
        fprintf (stderr, c"wait-enter: nsecs=%d read_kbd=%d wait_proc=%p\n".as_ptr (),
                 nsecs, read_kbd, wait_proc as *const c_void);
      }
  }

  static mut last_read_channel: c_int = -1;
  let mut channel: c_int;
  let mut nfds: c_int;
  let mut Available: fd_set = std::mem::zeroed ();
  let mut Writeok: fd_set = std::mem::zeroed ();
  let mut check_write: bool;
  let mut check_delay: c_int;
  let mut no_avail: c_int;
  let mut xerrno: c_int;
  let mut proc: Lisp_Object;
  let mut timeout: timespec;
  let mut end_time: timespec = std::mem::zeroed ();
  let mut timer_delay: timespec = std::mem::zeroed ();
  let mut got_output_end_time: timespec = invalid_timespec ();
  let mut wait: c_int;
  let mut got_some_output: c_int = -1;
  let prev_wait_proc_nbytes_read: uintmax_t
    = if !wait_proc.is_null () { (*wait_proc).nbytes_read } else { 0 };
  /* `#if defined HAVE_GETADDRINFO_A || defined HAVE_GNUTLS` is dead: `bool retry_for_async;`  */
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut time_limit: i64 = time_limit;

  eassert! (wait_proc.is_null ()
	    || NILP ((*wait_proc).thread)
	    || XTHREAD ((*wait_proc).thread) == current_thread);

  FD_ZERO (&raw mut Available);
  FD_ZERO (&raw mut Writeok);

  if (time_limit == 0 && nsecs == 0 && !wait_proc.is_null () && !NILP (Vinhibit_quit)
      && !(CONSP ((*wait_proc).status)
	   && EQ (XCAR ((*wait_proc).status), Qexit ())))
    { message1 (c"Blocking call to accept-process-output with quit inhibited!!".as_ptr ()); }

  record_unwind_protect_int (Some (wait_reading_process_output_unwind),
			     waiting_for_user_input_p! ());
  waiting_for_user_input_p! () = read_kbd;

  /* C 5388: `if (TYPE_MAXIMUM (time_t) < time_limit) time_limit = TYPE_MAXIMUM (time_t);`  */
  if (TYPE_MAXIMUM_TIME_T < time_limit)
    { time_limit = TYPE_MAXIMUM_TIME_T; }

  if (time_limit < 0 || nsecs < 0)
    { wait = MINIMUM; }
  else if (time_limit > 0 || nsecs > 0)
    {
      wait = TIMEOUT;
      let now: timespec = monotonic_coarse_timespec ();
      end_time = timespec_add (now, make_timespec (time_limit as time_t, nsecs as c_long));
    }
  else
    { wait = FOREVER; }

  loop
    {
      let mut process_skipped: bool = false;
      let mut wrapped: bool;
      let mut channel_start: c_int;

      /* If calling from keyboard input, do not quit since we want to return C-g as an
	 input character.  Otherwise, do pending quit if requested.  */
      if (read_kbd >= 0)
	{ maybe_quit (); }
      else if (pending_signals)
	{ process_pending_signals (); }

      /* Exit now if the cell we're waiting for became non-nil.  */
      if (! NILP (wait_for_cell) && ! NILP (XCAR (wait_for_cell)))
	{ break; }

      eassert! (max_desc < FD_SETSIZE);

      /* C 5432-5486's `#if defined HAVE_GETADDRINFO_A || defined HAVE_GNUTLS` arm, carried
	 under `have_gnutls` (neither macro is defined on this target).  */
      #[cfg (feature = "have_gnutls")]
      {
	let mut process_list_head: Lisp_Object;
	let mut aproc: Lisp_Object;
	let mut p: *mut Lisp_Process;
	let mut retry_for_async: bool = false;

	retry_for_async = false;
	FOR_EACH_PROCESS! (process_list_head, aproc, {
	  p = XPROCESS (aproc);

	  if (wait_proc.is_null () || p == wait_proc)
	    {
	      /* `#ifdef HAVE_GETADDRINFO_A`: check for pending DNS requests.  */
	      if (!(*p).dns_request.is_null ())
		{
		  let addrinfos: Lisp_Object = check_for_dns (aproc);
		  if (!NILP (addrinfos) && !EQ (addrinfos, Qt ()))
		    { connect_network_socket (aproc, addrinfos, Qnil ()); }
		  else
		    { retry_for_async = true; }
		}
	      /* `#ifdef HAVE_GNUTLS`: continue TLS negotiation.  */
	      if ((*p).gnutls_initstage == GNUTLS_STAGE_HANDSHAKE_TRIED
		  && process_bits! (is_non_blocking_client, p)
		  /* Don't proceed until we have established a connection.  */
		  && (fd_callback_info[(*p).outfd as usize].flags
		      & NON_BLOCKING_CONNECT_FD) == 0)
		{
		  gnutls_try_handshake (p);
		  (*p).gnutls_handshakes_tried += 1;

		  if ((*p).gnutls_initstage == GNUTLS_STAGE_READY)
		    {
		      gnutls_verify_boot (aproc, Qnil ());
		      finish_after_tls_connection (aproc);
		    }
		  else
		    {
		      retry_for_async = true;
		      if ((*p).gnutls_handshakes_tried
			  > GNUTLS_EMACS_HANDSHAKES_LIMIT)
			{
			  deactivate_process (aproc);
			  pset_status (p, list2 (Qfailed (),
						 build_string (c"TLS negotiation failed".as_ptr ())));
			}
		    }
		}
	    }
	});
      }

      /* Compute time from now till when time limit is up.  */
      /* Exit if already run out.  */
      if (wait == TIMEOUT)
	{
	  let now: timespec = monotonic_coarse_timespec ();
	  if (timespec_cmp (end_time, now) <= 0)
	    { break; }
	  timeout = timespec_sub (end_time, now);
	}
      else
	{ timeout = make_timespec (if wait < TIMEOUT { 0 } else { 100000 }, 0); }

      /* Normally we run timers here.  But not if wait_for_cell; in those cases, the wait
	 is supposed to be short, and those callers cannot handle running arbitrary Lisp
	 code here.  */
      if (NILP (wait_for_cell)
	  && just_wait_proc >= 0)
	{
	  loop
	    {
	      let old_timers_run: c_uint = timers_run;

	      timer_delay = timer_check ();

	      if (timers_run != old_timers_run && do_display != 0)
		/* We must retry, since a timer may have requeued itself and that could
		   alter the time_delay.  */
		{ redisplay_preserve_echo_area (9); }
	      else
		{ break; }
	      if detect_input_pending () { break; }
	    }

	  /* If there is unread keyboard input, also return.  */
	  if (read_kbd != 0
	      && requeued_command_events_pending_p ())
	    { break; }

	  /* This is so a breakpoint can be put here.  */
	  if (!timespec_valid_p (timer_delay))
	    { wait_reading_process_output_1 (); }
	}

      /* Cause C-g signals to take immediate action, and cause input available signals to
	 zero out timeout.  It is important that we do this before checking for process
	 activity.  If we get a SIGCHLD after the explicit checks for process activity,
	 timeout is the only way we will know.  */
      if (read_kbd < 0 && kbd_is_ours ())
	{ set_waiting_for_input (&raw mut timeout); }

      /* If status of something has changed, and no input is available, notify the user of
	 the change right away.  After this explicit check, we'll let the SIGCHLD handler
	 zap timeout to get our attention.  */
      if (update_tick != process_tick)
	{
	  let mut Atemp: fd_set = std::mem::zeroed ();
	  let mut Ctemp: fd_set = std::mem::zeroed ();

	  if (kbd_on_hold_p ())
	    { FD_ZERO (&raw mut Atemp); }
	  else
	    { compute_input_wait_mask (&raw mut Atemp); }
	  compute_write_mask (&raw mut Ctemp);

	  /* If a process status has changed, the child signal pipe will likely be
	     readable.  We want to ignore it for now, because otherwise we wouldn't run
	     into a timeout below.  */
	  let fd: c_int = child_signal_read_fd;
	  eassert! (fd < FD_SETSIZE);
	  if (0 <= fd)
	    { FD_CLR (fd, &raw mut Atemp); }

	  timeout = make_timespec (0, 0);
	  if (thread_select (Some (pselect), max_desc + 1,
			     &raw mut Atemp,
			     if num_pending_connects > 0 { &raw mut Ctemp }
			     else { std::ptr::null_mut () },
			     std::ptr::null_mut (), &raw mut timeout,
			     std::ptr::null_mut ())
	      <= 0)
	    {
	      /* It's okay for us to do this and then continue with the loop, since
		 timeout has already been zeroed out.  */
	      clear_waiting_for_input ();
	      got_some_output = status_notify (std::ptr::null_mut (), wait_proc);
	      if do_display != 0 { redisplay_preserve_echo_area (13); }
	    }
	}

      /* Don't wait for output from a non-running process.  Just read whatever data has
	 already been received.  */
      if !wait_proc.is_null () && process_bits! (raw_status_new, wait_proc)
	{ update_status (wait_proc); }
      if (!wait_proc.is_null ()
	  && ! EQ ((*wait_proc).status, Qrun ())
	  && ! connecting_status ((*wait_proc).status))
	{
	  let mut read_some_bytes: bool = false;

	  clear_waiting_for_input ();

	  /* If data can be read from the process, do so until exhausted.  */
	  if ((*wait_proc).infd >= 0)
	    {
	      let mut count_: c_uint = 0;
	      XSETPROCESS! (proc, wait_proc);

	      loop
		{
		  let nread: c_int = read_process_output (proc, (*wait_proc).infd);
		  rarely_quit (count_ as u64);
		  count_ += 1;
		  if (nread < 0)
		    {
		      if (*errno () != EINTR)
			{ break; }
		    }
		  else
		    {
		      if (got_some_output < nread)
			{ got_some_output = nread; }
		      if (nread == 0)
			{ break; }
		      read_some_bytes = true;
		    }
		}
	    }

	  if (read_some_bytes && do_display != 0)
	    { redisplay_preserve_echo_area (10); }

	  break;
	}

      /* Wait till there is something to do.  */

      if (!wait_proc.is_null () && just_wait_proc != 0)
	{
	  if ((*wait_proc).infd < 0)  /* Terminated.  */
	    { break; }
	  FD_SET ((*wait_proc).infd, &raw mut Available);
	  check_delay = 0;
	  check_write = false;
	}
      else if (!NILP (wait_for_cell))
	{
	  compute_non_process_wait_mask (&raw mut Available);
	  check_delay = 0;
	  check_write = false;
	}
      else
	{
	  if (read_kbd == 0)
	    { compute_non_keyboard_wait_mask (&raw mut Available); }
	  else
	    { compute_input_wait_mask (&raw mut Available); }
	  compute_write_mask (&raw mut Writeok);
	  check_delay = if !wait_proc.is_null () { 0 } else { process_output_delay_count };
	  check_write = true;
	}

      /* PORTING.md P-122 ⑷'s instrument (FEMACS_CHILD_TRACE, P-115's shape): once per run, is
	 SIGCHLD still delivered to this process's handler, and is it blocked right now?  A
	 child that is never reaped has to show up here.  */
      if ! getenv (c"FEMACS_CHILD_TRACE".as_ptr ()).is_null ()
	{
	  static mut child_state_printed: bool = false;
	  if ! child_state_printed
	    {
	      child_state_printed = true;
	      let mut act: sigaction = std::mem::zeroed ();
	      let mut cur: sigset_t = std::mem::zeroed ();
	      sigaction (SIGCHLD, std::ptr::null (), &raw mut act);
	      pthread_sigmask (SIG_BLOCK, std::ptr::null (), &raw mut cur);
	      let handler: *const c_void
		= match act.__sigaction_u.__sa_handler
		  {
		    Some (f) => f as *const () as *const c_void,
		    None => std::ptr::null (),
		  };
	      fprintf (stderr, c"child-state: handler=%p flags=%d blocked=%d\n".as_ptr (),
		       handler, act.sa_flags, sigismember (&cur, SIGCHLD));
	    }
	}
      /* We have to be informed when we receive a SIGCHLD signal for an asynchronous
	 process.  Otherwise this might deadlock if we receive a SIGCHLD during
	 `pselect'.  */
      let child_fd: c_int = child_signal_read_fd;
      eassert! (child_fd < FD_SETSIZE);
      if (0 <= child_fd)
	{ FD_SET (child_fd, &raw mut Available); }

      /* If frame size has changed or the window is newly mapped, redisplay now, before
	 we start to wait.  There is a race condition here; if a SIGIO arrives between now
	 and the select and indicates that a frame is trashed, the select may block
	 displaying a trashed screen.  */
      if (frame_garbaged && do_display != 0)
	{
	  clear_waiting_for_input ();
	  redisplay_preserve_echo_area (11);
	  if (read_kbd < 0 && kbd_is_ours ())
	    { set_waiting_for_input (&raw mut timeout); }
	}

      /* Skip the `select' call if input is available and we're waiting for keyboard
	 input or a cell change (which can be triggered by processing X events).  In the
	 latter case, set nfds to 1 to avoid breaking the loop.  */
      no_avail = 0;
      if (read_kbd != 0
	  && detect_input_pending ())
	{
	  nfds = if read_kbd != 0 { 0 } else { 1 };
	  no_avail = 1;
	  FD_ZERO (&raw mut Available);
	}
      else
	{
	  /* C 5697-5699 (`#ifdef HAVE_GNUTLS`'s `int tls_nfds; fd_set
	     tls_available;`) is carried text.  */
	  /* Set the timeout for adaptive read buffering if any process has non-zero
	     read_output_skip and non-zero read_output_delay, and we are not reading output
	     for a specific process.  It is not executed if
	     Vprocess_adaptive_read_buffering is nil.  */
	  if (process_output_skip && check_delay > 0)
	    {
	      let mut adaptive_nsecs: c_long = timeout.tv_nsec;
	      if (timeout.tv_sec > 0 || adaptive_nsecs > READ_OUTPUT_DELAY_MAX as c_long)
		{ adaptive_nsecs = READ_OUTPUT_DELAY_MAX as c_long; }
	      channel = 0;
	      while check_delay > 0 && channel <= max_desc
		{
		  proc = chan_process[channel as usize];
		  if (NILP (proc))
		    { channel += 1; continue; }
		  /* Find minimum non-zero read_output_delay among the processes with
		     non-zero read_output_skip.  */
		  if ((*XPROCESS (proc)).read_output_delay > 0)
		    {
		      check_delay -= 1;
		      if (!process_bits! (read_output_skip, XPROCESS (proc)))
			{ channel += 1; continue; }
		      FD_CLR (channel, &raw mut Available);
		      process_skipped = true;
		      process_bits! (read_output_skip, XPROCESS (proc), 0);
		      if ((*XPROCESS (proc)).read_output_delay < adaptive_nsecs as c_int)
			{ adaptive_nsecs = (*XPROCESS (proc)).read_output_delay as c_long; }
		    }
		  channel += 1;
		}
	      timeout = make_timespec (0, adaptive_nsecs);
	      process_output_skip = false;
	    }

	  /* If we've got some output and haven't limited our timeout with adaptive read
	     buffering, limit it.  */
	  if (got_some_output > 0 && !process_skipped
	      && (timeout.tv_sec != 0
		  || timeout.tv_nsec > READ_OUTPUT_DELAY_INCREMENT as c_long))
	    { timeout = make_timespec (0, READ_OUTPUT_DELAY_INCREMENT as c_long); }

	  if (NILP (wait_for_cell) && just_wait_proc >= 0
	      && timespec_valid_p (timer_delay)
	      && timespec_cmp (timer_delay, timeout) < 0)
	    {
	      let now: timespec = monotonic_coarse_timespec ();
	      let timeout_abs: timespec = timespec_add (now, timeout);
	      if (!timespec_valid_p (got_output_end_time)
		  || timespec_cmp (timeout_abs, got_output_end_time) < 0)
		{ got_output_end_time = timeout_abs; }
	      timeout = timer_delay;
	    }
	  else
	    { got_output_end_time = invalid_timespec (); }

	  /* C 5756-5763: the `retry_for_async` timeout, carried under `have_gnutls`.  */
	  #[cfg (feature = "have_gnutls")]
	  {
	    if (retry_for_async
		&& (timeout.tv_sec > 0 || timeout.tv_nsec > ASYNC_RETRY_NSEC))
	      {
		timeout.tv_sec = 0;
		timeout.tv_nsec = ASYNC_RETRY_NSEC;
	      }
	  }

	  /* C 5765-5794's `#ifdef HAVE_GNUTLS` arm: GnuTLS buffers data internally, so the
	     probe checks its buffers before the select.  */
	  #[cfg (feature = "have_gnutls")]
	  {
	    let mut tls_available: fd_set = std::mem::zeroed ();
	    let mut tls_nfds: c_int;

	    FD_ZERO (&raw mut tls_available);
	    tls_nfds = 0;
	    channel = 0;
	    while channel < FD_SETSIZE
	      {
		if (! NILP (chan_process[channel as usize])
		    && FD_ISSET (channel, &raw mut Available))
		  {
		    let p: *mut Lisp_Process = XPROCESS (chan_process[channel as usize]);
		    if (!p.is_null ()
			&& process_bits! (gnutls_p, p) && !(*p).gnutls_state.is_null ()
			&& emacs_gnutls_record_check_pending ((*p).gnutls_state) > 0)
		      {
			tls_nfds += 1;
			eassert! ((*p).infd == channel);
			FD_SET ((*p).infd, &raw mut tls_available);
		      }
		  }
		channel += 1;
	      }
	    /* If wait_proc is somebody else, we have to wait in select as usual.  Otherwise,
	       clobber the timeout.  */
	    if (tls_nfds > 0
		&& (wait_proc.is_null ()
		    || ((*wait_proc).infd >= 0
			&& FD_ISSET ((*wait_proc).infd, &raw mut tls_available))))
	      { timeout = make_timespec (0, 0); }
	  }
	  /* `#if !defined USABLE_SIGIO && !defined WINDOWSNT` is live.  */
	  {
	    /* If we're polling for input, don't get stuck in select for more than
	       25 msec.  */
	    let short_timeout: timespec = make_timespec (0, 25000000);
	    if ((read_kbd != 0 || !NILP (wait_for_cell))
		&& timespec_cmp (short_timeout, timeout) < 0)
	      { timeout = short_timeout; }
	  }

	  /* Android requires using a replacement for pselect in android.c to poll for events
	     (C 5805-5815, `#if defined HAVE_ANDROID && !defined ANDROID_STUBIFY`).  */
	  #[cfg (feature = "have_android")]
	  {
	    /* `#ifndef THREADS_ENABLED`'s arm.  */
	    #[cfg (not (feature = "threads_enabled"))]
	    { nfds = android_select (max_desc + 1, &raw mut Available,
				     if check_write { &raw mut Writeok } else { std::ptr::null_mut () },
				     std::ptr::null_mut (), &raw mut timeout); }
	    #[cfg (feature = "threads_enabled")]
	    { nfds = thread_select (Some (android_select_wrapper), max_desc + 1,
				    &raw mut Available,
				    if check_write { &raw mut Writeok } else { std::ptr::null_mut () },
				    std::ptr::null_mut (), &raw mut timeout, std::ptr::null_mut ()); }
	  }
	  /* C 5820-5828's `#if defined HAVE_GLIB && !defined HAVE_NS` arm (`xg_select`) and
	     its `#elif defined HAVE_NS` arm (`ns_select`).  */
	  #[cfg (feature = "have_glib_and_not_ns")]
	  { nfds = xg_select (max_desc + 1, &raw mut Available,
			      if check_write { &raw mut Writeok } else { std::ptr::null_mut () },
			      std::ptr::null_mut (), &raw mut timeout, std::ptr::null_mut ()); }
	  #[cfg (feature = "have_ns")]
	  { nfds = ns_select (max_desc + 1, &raw mut Available,
			      if check_write { &raw mut Writeok } else { std::ptr::null_mut () },
			      std::ptr::null_mut (), &raw mut timeout, std::ptr::null_mut ()); }
	  #[cfg (not (any (feature = "have_android", feature = "have_glib_and_not_ns",
			   feature = "have_ns")))]
	  {
	    nfds = thread_select (Some (pselect), max_desc + 1,
				  &raw mut Available,
				  if check_write { &raw mut Writeok } else { std::ptr::null_mut () },
				  std::ptr::null_mut (), &raw mut timeout,
				  std::ptr::null_mut ());
	  }
	}

      /* C 5837-5853's `#ifdef HAVE_GNUTLS` arm: merge `tls_available` into `Available`.  */
      #[cfg (feature = "have_gnutls")]
      {
	let tls_nfds: c_int = 0;
	if (tls_nfds > 0)
	  {
	    if (nfds == 0 || (nfds < 0 && *errno () == EINTR))
	      {
		/* Fast path, just copy.  */
		nfds = tls_nfds;
		Available = tls_available;
	      }
	    else if (nfds > 0)
	      /* Slow path, merge one by one.  Note: nfds does not need to be accurate, just
		 positive is enough.  */
	      {
		channel = 0;
		while channel < FD_SETSIZE
		  {
		    if (FD_ISSET (channel, &raw mut tls_available))
		      { FD_SET (channel, &raw mut Available); }
		    channel += 1;
		  }
	      }
	  }
      }

      xerrno = *errno ();

      /* Make C-g and alarm signals set flags again.  */
      clear_waiting_for_input ();

      /*  If we woke up due to SIGWINCH, actually change size now.  */
      do_pending_window_change (false);

      if (nfds == 0)
	{
	  if (read_kbd == 0 && update_tick != process_tick)
	    {
	      /* This is for the case where we bypassed a similar call above, after the
		 first thread_select, because some input was available, but later found
		 in the second thread_select that input was only "from keyboard", which
		 we need to ignore because we were called with read_kbd zero.  We should
		 therefore process the changed status of sub-processes.  */
	      got_some_output = status_notify (std::ptr::null_mut (), wait_proc);
	      if do_display != 0 { redisplay_preserve_echo_area (113); }
	    }
	  /* Exit the main loop if we've passed the requested timeout, or have read some
	     bytes from our wait_proc (either directly in this call or indirectly through
	     timers / process filters), or aren't skipping processes and got some output
	     and haven't lowered our timeout due to timers or SIGIO and have waited a long
	     amount of time due to repeated timers.  */
	  let huge_timespec: timespec
	    = make_timespec (TYPE_MAXIMUM_TIME_T, 2 * TIMESPEC_HZ as c_long);
	  let mut cmp_time: timespec = huge_timespec;
	  if (wait < TIMEOUT
	      || (!wait_proc.is_null ()
		  && (*wait_proc).nbytes_read != prev_wait_proc_nbytes_read))
	    { break; }
	  if (wait == TIMEOUT)
	    { cmp_time = end_time; }
	  if (!process_skipped && got_some_output > 0
	      && (timeout.tv_sec > 0 || timeout.tv_nsec > 0))
	    {
	      if (!timespec_valid_p (got_output_end_time))
		{ break; }
	      if (timespec_cmp (got_output_end_time, cmp_time) < 0)
		{ cmp_time = got_output_end_time; }
	    }
	  if (timespec_cmp (cmp_time, huge_timespec) < 0)
	    {
	      let now: timespec = monotonic_coarse_timespec ();
	      if (timespec_cmp (cmp_time, now) <= 0)
		{ break; }
	    }
	}

      if (nfds < 0)
	{
	  if (xerrno == EINTR)
	    { no_avail = 1; }
	  else if (xerrno == EBADF)
	    { emacs_abort (); }
	  else
	    { report_file_errno (c"Failed select".as_ptr (), Qnil (), xerrno); }
	}

      /* Check for keyboard input.  */
      /* If there is any, return immediately to give it higher priority than
	 subprocesses.  */

      if (read_kbd != 0)
	{
	  let mut leave: bool = false;

	  if (detect_input_pending_run_timers (do_display != 0))
	    {
	      swallow_events (do_display != 0);
	      if (detect_input_pending_run_timers (do_display != 0))
		{ leave = true; }
	    }

	  if (leave)
	    { break; }
	}

      /* If there is unread keyboard input, also return.  */
      if (read_kbd != 0
	  && requeued_command_events_pending_p ())
	{ break; }

      /* If we are not checking for keyboard input now, do process events (but don't run
	 any timers).  This is so that X events will be processed.  ...  */
      if (read_kbd == 0 && detect_input_pending ())
	{
	  swallow_events (do_display != 0);
	  /* C 5955-5957's `#if 0 /* Exiting when read_kbd doesn't request that seems
	     wrong, though.  */` is dead in every configuration and is not carried
	     (P-08).  */
	}

      /* Exit now if the cell we're waiting for became non-nil.  */
      if (! NILP (wait_for_cell) && ! NILP (XCAR (wait_for_cell)))
	{ break; }

      /* `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` is live.  */
      /* If we think we have keyboard input waiting, but didn't get SIGIO, go read it.
	 This can happen with X on BSD after logging out.  In that case, there really is
	 no input and no SIGIO, but select says there is input.  */
      if (read_kbd != 0 && interrupt_input
	  && keyboard_bit_set (&raw mut Available) && ! noninteractive)
	{
	  /* C 5965-5969: `#ifdef USABLE_SIGIO` uses SIGIO, its `#else` uses SIGPOLL (which
	     this target does not define).  */
	  #[cfg (feature = "no_usable_sigio")]
	  { handle_input_available_signal (SIGPOLL); }
	  #[cfg (not (feature = "no_usable_sigio"))]
	  handle_input_available_signal (SIGIO);
	}

      /* If checking input just got us a size-change event from X, obey it now if we
	 should.  */
      if (read_kbd != 0 || ! NILP (wait_for_cell))
	{ do_pending_window_change (false); }

      /* Check for data from a process.  */
      if (no_avail != 0 || nfds == 0)
	{ continue; }

      channel = 0;
      while channel <= max_desc
	{
	  let d: *mut fd_callback_data = &raw mut fd_callback_info[channel as usize];
	  if ((*d).func.is_some ()
	      && (((*d).flags & FOR_READ != 0
		   && FD_ISSET (channel, &raw mut Available))
		  || ((*d).flags & FOR_WRITE != 0
		      && FD_ISSET (channel, &raw mut Writeok))))
	    { ((*d).func.unwrap ()) (channel, (*d).data); }
	  channel += 1;
	}

      /* Do round robin if `process-pritoritize-lower-fds' is nil.  */
      channel_start
	= if process_prioritize_lower_fds { 0 } else { last_read_channel + 1 };

      channel = channel_start; wrapped = false;
      loop
	{
	  if (!(!wrapped || (channel < channel_start && channel <= max_desc)))
	    { break; }
	  if (channel > max_desc)
	    {
	      wrapped = true;
	      channel = -1;
	      channel += 1;
	      continue;
	    }

	  if (FD_ISSET (channel, &raw mut Available)
	      && ((fd_callback_info[channel as usize].flags & (KEYBOARD_FD | PROCESS_FD))
		  == PROCESS_FD))
	    {
	      let mut nread: c_int;

	      /* If waiting for this channel, arrange to return as soon as no more input
		 to be processed.  No more waiting.  */
	      proc = chan_process[channel as usize];
	      if (NILP (proc))
		{ channel += 1; continue; }

	      /* If this is a server stream socket, accept connection.  */
	      if (EQ ((*XPROCESS (proc)).status, Qlisten ()))
		{
		  server_accept_connection (proc, channel);
		  channel += 1;
		  continue;
		}

	      /* Read data from the process, starting with our buffered-ahead character
		 if we have one.  */

	      nread = read_process_output (proc, channel);
	      if ((wait_proc.is_null () || wait_proc == XPROCESS (proc))
		  && got_some_output < nread)
		{ got_some_output = nread; }
	      if (nread > 0)
		{
		  /* Vacuum up any leftovers without waiting.  */
		  if (wait_proc == XPROCESS (proc))
		    { wait = MINIMUM; }
		  /* Since read_process_output can run a filter, which can call
		     accept-process-output, don't try to read from any other processes
		     before doing the select again.  */
		  FD_ZERO (&raw mut Available);
		  last_read_channel = channel;

		  if (do_display != 0)
		    { redisplay_preserve_echo_area (12); }
		}
	      else if (nread == -1 && would_block (*errno ()))
		{ }
	      /* `#ifdef HAVE_PTYS` is live.  */
	      else if (nread == -1 && *errno () == EIO)
		{
		  let p: *mut Lisp_Process = XPROCESS (proc);

		  /* Clear the descriptor now, so we only raise the signal once.  */
		  delete_read_fd (channel);

		  if ((*p).pid == -2)
		    {
		      /* If the EIO occurs on a pty, the SIGCHLD handler's waitpid call
			 will not find the process object to delete.  Do it here.  */
		      process_tick += 1;
		      (*p).tick = process_tick;
		      pset_status (p, Qfailed ());
		    }
		}
	      /* If we can detect process termination, don't consider the process gone
		 just because its pipe is closed.  */
	      else if (nread == 0 && !NETCONN_P (proc) && !SERIALCONN_P (proc)
		       && !PIPECONN_P (proc))
		/* `#ifdef WINDOWSNT`'s empty arm is carried text.  */
		{ delete_read_fd (channel); }
	      else if (nread == 0 && PIPECONN_P (proc))
		{
		  /* Preserve status of processes already terminated.  */
		  process_tick += 1;
		  (*XPROCESS (proc)).tick = process_tick;
		  deactivate_process (proc);
		  if (EQ ((*XPROCESS (proc)).status, Qrun ()))
		    { pset_status (XPROCESS (proc),
				   list2 (Qexit (), make_fixnum (0))); }
		}
	      else
		{
		  /* Preserve status of processes already terminated.  */
		  process_tick += 1;
		  (*XPROCESS (proc)).tick = process_tick;
		  deactivate_process (proc);
		  if (process_bits! (raw_status_new, XPROCESS (proc)))
		    { update_status (XPROCESS (proc)); }
		  if (EQ ((*XPROCESS (proc)).status, Qrun ()))
		    { pset_status (XPROCESS (proc),
				   list2 (Qexit (), make_fixnum (256))); }
		}
	    }
	  if (FD_ISSET (channel, &raw mut Writeok)
	      && (fd_callback_info[channel as usize].flags
		  & NON_BLOCKING_CONNECT_FD) != 0)
	    {
	      let p: *mut Lisp_Process;

	      delete_write_fd (channel);

	      proc = chan_process[channel as usize];
	      if (NILP (proc))
		{ channel += 1; continue; }

	      p = XPROCESS (proc);

	      /* `#ifndef WINDOWSNT` is live; C 6091-6105's `#else` arm (the `getpeername`
		 slippage probe) is carried under `windowsnt`.  */
	      #[cfg (feature = "windowsnt")]
	      {
		let mut pname: sockaddr = std::mem::zeroed ();
		let mut pnamelen: socklen_t = std::mem::size_of_val (&pname) as socklen_t;

		/* If connection failed, getpeername will fail.  */
		xerrno = 0;
		if (getpeername (channel, &raw mut pname, &raw mut pnamelen) < 0)
		  {
		    /* Obtain connect failure code through error slippage.  */
		    let mut dummy: c_char = 0;
		    xerrno = *errno ();
		    if (*errno () == ENOTCONN && read (channel, &raw mut dummy as *mut c_void, 1) < 0)
		      { xerrno = *errno (); }
		  }
	      }
	      {
		let mut xlen: socklen_t = std::mem::size_of_val (&xerrno) as socklen_t;
		if (getsockopt (channel, SOL_SOCKET, SO_ERROR,
				&raw mut xerrno as *mut c_void, &raw mut xlen) != 0)
		  { xerrno = *errno (); }
	      }
	      if (xerrno != 0)
		{
		  let addrinfos: Lisp_Object
		    = if connecting_status ((*p).status) { XCDR ((*p).status) } else { Qnil () };
		  if (!NILP (addrinfos))
		    { XSETCDR ((*p).status, XCDR (addrinfos)); }
		  else
		    {
		      process_tick += 1;
		      (*p).tick = process_tick;
		      pset_status (p, list2 (Qfailed (), make_fixnum (xerrno as EMACS_INT)));
		    }
		  deactivate_process (proc);
		  if (!NILP (addrinfos))
		    { connect_network_socket (proc, addrinfos, Qnil ()); }
		}
	      else
		{
		  /* C 6170-6177: the `#ifdef HAVE_GNUTLS` guards *the condition* — if we have an
		     incompletely set up TLS connection, defer the sentinel signaling until
		     later; without GnuTLS the block below runs unconditionally.  */
		  let mut run_sentinel: bool = true;
		  #[cfg (feature = "have_gnutls")]
		  {
		    run_sentinel = NILP ((*p).gnutls_boot_parameters) && !(*p).gnutls_p;
		  }
		  if run_sentinel
		  {
		    pset_status (p, Qrun ());
		    /* Execute the sentinel here.  If we had relied on status_notify to
		       do it later, it will read input from the process before calling
		       the sentinel.  */
		    exec_sentinel (proc, build_string (c"open\n".as_ptr ()));
		  }

		  if (0 <= (*p).infd && !EQ ((*p).filter, Qt ())
		      && !EQ ((*p).command, Qt ()))
		    { add_process_read_fd ((*p).infd); }
		}
	    }
	  channel += 1;
	}			/* End for each file descriptor.  */
    }				/* End while exit conditions not met.  */

  unbind_to (count, Qnil ());

  /* If calling from keyboard input, do not quit since we want to return C-g as an input
     character.  Otherwise, do pending quit if requested.  */
  if (read_kbd >= 0)
    {
      /* Prevent input_pending from remaining set if we quit.  */
      clear_input_pending ();
      maybe_quit ();
    }

  /* Timers and/or process filters that we have run could have themselves called
     `accept-process-output' (and by that indirectly this function), thus possibly reading
     some (or all) output of wait_proc without us noticing it.  This could potentially lead
     to an endless wait (dealt with earlier in the function) and/or a wrong return value
     (dealt with here).  */
  if (!wait_proc.is_null () && (*wait_proc).nbytes_read != prev_wait_proc_nbytes_read)
    { got_some_output = std::cmp::min (INT_MAX as EMACS_INT,
				       ((*wait_proc).nbytes_read
					- prev_wait_proc_nbytes_read) as EMACS_INT) as c_int; }

  return got_some_output;
}

/// C: lisp.h 5187's `TYPE_MAXIMUM (time_t)` — `time_t` is `i64` on this target, so
/// `TYPE_MAXIMUM (time_t)` measures as `INT64_MAX`.
const TYPE_MAXIMUM_TIME_T: time_t = i64::MAX;


unsafe extern "C" {
  /// C: void clear_waiting_for_input (void)   (keyboard.c, lisp.h 5398)
  fn clear_waiting_for_input ();
  /// C: struct timespec timer_check (void)   (atimer.c, lisp.h 5397)
  fn timer_check () -> timespec;
  /// C: void do_pending_window_change (bool)   (dispnew.c, lisp.h 4916)
  fn do_pending_window_change (safe: bool);
  /// C: void swallow_events (bool)   (keyboard.c, lisp.h 4900)
  fn swallow_events (do_display: bool);
  /// C: bool detect_input_pending (void)   (keyboard.c, lisp.h 4894)
  fn detect_input_pending () -> bool;
  /// C: bool detect_input_pending_run_timers (bool)   (keyboard.c)
  fn detect_input_pending_run_timers (do_display: bool) -> bool;
  /// C: bool requeued_command_events_pending_p (void)   (keyboard.c)
  fn requeued_command_events_pending_p () -> bool;
  /// C: void clear_input_pending (void)   (keyboard.c)
  fn clear_input_pending ();
  /// C: void handle_input_available_signal (int)   (keyboard.c)
  fn handle_input_available_signal (sig: c_int);
  /// C: int getpeername (int, struct sockaddr *restrict, socklen_t *restrict)
  fn getpeername (socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
}

unsafe extern "C" {
  /// C: void process_pending_signals (void)   (keyboard.c, lisp.h 5177)
  fn process_pending_signals ();
  /// C: bool interrupt_input   (keyboard.c, keyboard.h 71)
  static mut interrupt_input: bool;
  /// C: bool pending_signals   (keyboard.c, lisp.h 5176)
  static mut pending_signals: bool;
  /// C: bool noninteractive   (emacs.c, lisp.h's `noninteractive`)
  static mut noninteractive: bool;
  /// C: Lisp_Object Vinhibit_quit   (globals.h's DEFVAR, alloc.c)
  static mut Vinhibit_quit: Lisp_Object;
}

/* Given a list (FUNCTION ARGS...), apply FUNCTION to the ARGS.  */
/* C 6222-6226: `read_process_output_call` — its address is taken by
   `internal_condition_case_1`, so it is `unsafe extern "C" fn` (PORTING.md 1.4).  */
unsafe extern "C" fn read_process_output_call (fun_and_args: Lisp_Object) -> Lisp_Object
{
  return apply1 (XCAR (fun_and_args), XCDR (fun_and_args));
}

/* C 6228-6237: `read_process_output_error_handler` — its address is taken by
   `internal_condition_case_1` (PORTING.md 1.4).  */
unsafe extern "C" fn read_process_output_error_handler (error_val: Lisp_Object) -> Lisp_Object
{
  cmd_error_internal (error_val, c"error in process filter: ".as_ptr ());
  Vinhibit_quit = Qt ();
  update_echo_area ();
  if (process_error_pause_time > 0)
    { Fsleep_for (make_fixnum (process_error_pause_time), Qnil ()); }
  return Qt ();
}

/* C 6239-6246: C's two forward declarations (`read_and_dispose_of_process_output` and
   `read_and_insert_process_output`) are dropped per PORTING.md 0.1-12.  */

/* The `DEFVAR_INT` carrier this file owns (C 9004; status 3.29): C's storage is
   `globals.f_process_error_pause_time`, so the unit defines the `*` object itself.  Type and
   alignment must be C's: `DEFVAR_INT` reads it through `intmax_t *` (the port's macro casts to
   `*mut i64`), so a `c_int` here is a 4-byte-aligned object read as 8 bytes - which is the
   misaligned-pointer panic of status 3.638 (`EMACS_INT` is the port's `intmax_t`, PORTING.md
   P-97).  */
#[unsafe(no_mangle)]
pub static mut process_error_pause_time: EMACS_INT = 0;

/// coding.h 408-440's bitfield members this file assigns, in P-80's member-name form: C's
/// field name stays in the port's text and dispatches to the `bits` unit the mirror declares
/// (the mirror carries only the *readers*, `mode_of`/`src_multibyte_p`/`dst_multibyte_p`, so
/// the writer for `mode` is added here — the same shape `process_bits!` uses).
macro_rules! coding_bits {
  (mode, $coding:expr) => { mode_of ($coding as *mut coding_system) as c_int };
  (mode, $coding:expr, $value:expr) => {
    (*($coding as *mut coding_system)).bits =
      ((*($coding as *mut coding_system)).bits
       & !(CODING_SYSTEM_MODE_MASK << CODING_SYSTEM_MODE_SHIFT))
      | (((($value) as u32) & CODING_SYSTEM_MODE_MASK) << CODING_SYSTEM_MODE_SHIFT)
  };
  (src_multibyte, $coding:expr) => { src_multibyte_p ($coding as *mut coding_system) };
  (src_multibyte, $coding:expr, $value:expr) => {
    (*($coding as *mut coding_system)).bits =
      (if ($value) as c_int != 0
       { (*($coding as *mut coding_system)).bits
	 | (1u32 << CODING_SYSTEM_SRC_MULTIBYTE_SHIFT) }
       else { (*($coding as *mut coding_system)).bits
	      & !(1u32 << CODING_SYSTEM_SRC_MULTIBYTE_SHIFT) })
  };
  (dst_multibyte, $coding:expr) => { dst_multibyte_p ($coding as *mut coding_system) };
  (dst_multibyte, $coding:expr, $value:expr) => {
    (*($coding as *mut coding_system)).bits =
      (if ($value) as c_int != 0
       { (*($coding as *mut coding_system)).bits
	 | (1u32 << CODING_SYSTEM_DST_MULTIBYTE_SHIFT) }
       else { (*($coding as *mut coding_system)).bits
	      & !(1u32 << CODING_SYSTEM_DST_MULTIBYTE_SHIFT) })
  };
  (insert_before_markers, $coding:expr) => {
    ((*($coding as *mut coding_system)).bits >> CODING_SYSTEM_INSERT_BEFORE_MARKERS_SHIFT) & 1
      != 0
  };
  (insert_before_markers, $coding:expr, $value:expr) => {
    (*($coding as *mut coding_system)).bits =
      (if ($value) as c_int != 0
       { (*($coding as *mut coding_system)).bits
	 | (1u32 << CODING_SYSTEM_INSERT_BEFORE_MARKERS_SHIFT) }
       else { (*($coding as *mut coding_system)).bits
	      & !(1u32 << CODING_SYSTEM_INSERT_BEFORE_MARKERS_SHIFT) })
  };
}

/* Read pending output from the process channel, starting with our buffered-ahead character
   if we have one.  Yield number of decoded characters read, or -1 (setting errno) if there
   is a read error.  ...  */
/* C 6259-6369: `read_process_output`.  */
unsafe fn read_process_output (proc_: Lisp_Object, channel: c_int) -> c_int
{
  let mut nbytes: ptrdiff_t = 0;   /* PORTING.md 1.5: C's `ssize_t nbytes;` is written below.  */
  let p: *mut Lisp_Process = XPROCESS (proc_);
  eassert! (0 <= channel && channel < FD_SETSIZE);
  let coding: *mut coding_system = proc_decode_coding_system[channel as usize];
  let carryover: c_int = (*p).decoding_carryover;
  let readmax: ptrdiff_t = (*p).readmax;
  let count: specpdl_ref = SPECPDL_INDEX ();
  let mut odeactivate: Lisp_Object;
  let mut chars: *mut c_char;

  USE_SAFE_ALLOCA! (sa_avail, sa_count, sa_mark);
  chars = SAFE_ALLOCA! (sa_avail,
			std::mem::size_of_val (&(*coding).carryover) + readmax as usize) as *mut c_char;

  if (carryover != 0)
    /* See the comment above.  */
    { memcpy (chars as *mut c_void, SDATA ((*p).decoding_buf) as *const c_void,
	      carryover as usize); }

  /* `#ifdef DATAGRAM_SOCKETS` is live.  */
  /* We have a working select, so proc_buffered_char is always -1.  */
  if (DATAGRAM_CHAN_P (channel))
    {
      let mut len: socklen_t = datagram_address[channel as usize].len as socklen_t;
      loop
	{
	  nbytes = recvfrom (channel, chars.offset (carryover as isize) as *mut c_void,
			     readmax as usize, 0,
			     datagram_address[channel as usize].sa, &raw mut len) as ptrdiff_t;
	  if !(nbytes < 0 && *errno () == EINTR) { break; }
	}
    }
  else
    {
      let buffered: bool = proc_buffered_char[channel as usize] >= 0;
      if (buffered)
	{
	  *chars.offset (carryover as isize) = proc_buffered_char[channel as usize] as c_char;
	  proc_buffered_char[channel as usize] = -1;
	}
      let mut gnutls_read: bool = false;
      /* C 6297-6301's `#ifdef HAVE_GNUTLS` arm.  */
      #[cfg (feature = "have_gnutls")]
      {
	if (process_bits! (gnutls_p, p) && !(*p).gnutls_state.is_null ())
	  {
	    nbytes = emacs_gnutls_read (p,
					chars.offset (carryover as isize).offset (buffered as isize),
					readmax - buffered as ptrdiff_t);
	    gnutls_read = true;
	  }
      }
      if !gnutls_read
      { nbytes = emacs_read (channel,
			   chars.offset (carryover as isize).offset (buffered as isize) as *mut c_void,
			   readmax - buffered as ptrdiff_t); }
      if (nbytes > 0 && process_bits! (adaptive_read_buffering, p) != 0)
	{
	  let mut delay: c_int = (*p).read_output_delay;
	  if (nbytes < 256)
	    {
	      if (delay < READ_OUTPUT_DELAY_MAX_MAX as c_int)
		{
		  if (delay == 0)
		    { process_output_delay_count += 1; }
		  delay += (READ_OUTPUT_DELAY_INCREMENT * 2) as c_int;
		}
	    }
	  else if (delay > 0 && nbytes == readmax - buffered as ptrdiff_t)
	    {
	      delay -= READ_OUTPUT_DELAY_INCREMENT as c_int;
	      if (delay == 0)
		{ process_output_delay_count -= 1; }
	    }
	  (*p).read_output_delay = delay;
	  if (delay != 0)
	    {
	      process_bits! (read_output_skip, p, 1);
	      process_output_skip = true;
	    }
	}
      nbytes += buffered as ptrdiff_t;
      nbytes += (buffered && nbytes <= 0) as ptrdiff_t;
    }

  (*p).decoding_carryover = 0;

  if (nbytes <= 0)
    {
      if (nbytes < 0 || coding_bits! (mode, coding) & CODING_MODE_LAST_BLOCK != 0)
	{
	  SAFE_FREE_UNBIND_TO! (sa_count, count, Qnil (), sa_mark);
	  return nbytes as c_int;
	}
      coding_bits! (mode, coding, coding_bits! (mode, coding) | CODING_MODE_LAST_BLOCK);
    }

  /* At this point, NBYTES holds number of bytes just received (including the one in
     proc_buffered_char[channel]).  */

  /* Ignore carryover, it's been added by a previous iteration already.  */
  (*p).nbytes_read += nbytes as uintmax_t;

  /* Now set NBYTES how many bytes we must decode.  */
  nbytes += carryover as ptrdiff_t;

  odeactivate = Vdeactivate_mark;
  /* There's no good reason to let process filters change the current buffer, and many
     callers of accept-process-output, sit-for, and friends don't expect current-buffer to
     be changed from under them.  */
  record_unwind_current_buffer ();

  read_and_dispose_of_process_output (p, chars, nbytes, coding);

  /* Handling the process output should not deactivate the mark.  */
  Vdeactivate_mark = odeactivate;

  SAFE_FREE_UNBIND_TO! (sa_count, count, Qnil (), sa_mark);
  return nbytes as c_int;
}

unsafe extern "C" {
  /// C: Lisp_Object apply1 (Lisp_Object, Lisp_Object)   (lisp.h 4439, eval.c)
  fn apply1 (fn_: Lisp_Object, arg: Lisp_Object) -> Lisp_Object;
  /// C: void cmd_error_internal (Lisp_Object, const char *)   (keyboard.c)
  fn cmd_error_internal (data: Lisp_Object, context: *const c_char);
  /// C: void update_echo_area (void)   (xdisp.c)
  fn update_echo_area ();
  /// C: ssize_t recvfrom (int, void *restrict, size_t, int, struct sockaddr *restrict,
  ///                      socklen_t *restrict)   (<sys/socket.h>)
  fn recvfrom (socket: c_int, buffer: *mut c_void, length: usize, flags: c_int,
	       address: *mut sockaddr, address_len: *mut socklen_t) -> ssize_t;
  /// C: ssize_t emacs_gnutls_read (struct Lisp_Process *, char *, ptrdiff_t)
  ///    (gnutls.c, the `HAVE_GNUTLS` arm)
  fn emacs_gnutls_read (p: *mut Lisp_Process, buf: *mut c_char, nbyte: ptrdiff_t) -> ssize_t;
}

/* C 6371-6399: `read_process_output_before_insert`.  */
unsafe fn read_process_output_before_insert (p: *mut Lisp_Process,
					     old_read_only: *mut Lisp_Object,
					     old_begv: *mut ptrdiff_t, old_zv: *mut ptrdiff_t,
					     before: *mut ptrdiff_t, before_byte: *mut ptrdiff_t,
					     opoint: *mut ptrdiff_t, opoint_byte: *mut ptrdiff_t)
{
  Fset_buffer ((*p).buffer);
  *opoint = PT! ();
  *opoint_byte = PT_BYTE! ();
  *old_read_only = BVAR! ((current_buffer! ()), read_only_);
  *old_begv = BEGV! ();
  *old_zv = ZV! ();

  bset_read_only ((current_buffer! ()), Qnil ());

  /* Insert new output into buffer at the current end-of-output marker, thus preserving
     logical ordering of input and output.  */
  if ((*XMARKER ((*p).mark)).buffer != std::ptr::null_mut ())
    { set_point_from_marker ((*p).mark); }
  else
    { SET_PT_BOTH (ZV! (), ZV_BYTE! ()); }
  *before = PT! ();
  *before_byte = PT_BYTE! ();

  /* If the output marker is outside of the visible region, save the restriction and
     widen.  */
  if (! (BEGV! () <= PT! () && PT! () <= ZV! ()))
    { Fwiden (); }
}

/* C 6401-6438: `read_process_output_after_insert`.  */
unsafe fn read_process_output_after_insert (p: *mut Lisp_Process, old_read_only: *mut Lisp_Object,
					    old_begv: ptrdiff_t, old_zv: ptrdiff_t,
					    before: ptrdiff_t, before_byte: ptrdiff_t,
					    opoint: ptrdiff_t, opoint_byte: ptrdiff_t)
{
  let b: *mut buffer;
  let mut opoint: ptrdiff_t = opoint;
  let mut opoint_byte: ptrdiff_t = opoint_byte;
  let mut old_begv: ptrdiff_t = old_begv;
  let mut old_zv: ptrdiff_t = old_zv;

  /* Make sure the process marker's position is valid when the process buffer is changed in
     the signal_after_change above.  W3 is known to do that.  */
  if (BUFFERP ((*p).buffer)
      && { b = XBUFFER ((*p).buffer); b != (current_buffer! ()) })
    { set_marker_both ((*p).mark, (*p).buffer, BUF_PT (b), BUF_PT_BYTE (b)); }
  else
    { set_marker_both ((*p).mark, (*p).buffer, PT! (), PT_BYTE! ()); }

  update_mode_lines = 23;

  /* Make sure opoint and the old restrictions float ahead of any new text just as point
     would.  */
  if (opoint >= before)
    {
      opoint += PT! () - before;
      opoint_byte += PT_BYTE! () - before_byte;
    }
  if (old_begv > before)
    { old_begv += PT! () - before; }
  if (old_zv >= before)
    { old_zv += PT! () - before; }

  /* If the restriction isn't what it should be, set it.  */
  if (old_begv != BEGV! () || old_zv != ZV! ())
    { Fnarrow_to_region (make_fixnum (old_begv as EMACS_INT), make_fixnum (old_zv as EMACS_INT)); }

  bset_read_only ((current_buffer! ()), *old_read_only);
  SET_PT_BOTH (opoint, opoint_byte);
}

/* C 6440-6480: `read_process_output_set_last_coding_system`.  */
unsafe fn read_process_output_set_last_coding_system (p: *mut Lisp_Process,
						      coding: *mut coding_system)
{
  Vlast_coding_system_used = CODING_ID_NAME ((*coding).id as c_int);
  /* A new coding system might be found.  */
  if (!EQ ((*p).decode_coding_system, Vlast_coding_system_used))
    {
      pset_decode_coding_system (p, Vlast_coding_system_used);

      /* Don't call setup_coding_system for proc_decode_coding_system[channel] here.  It is
	 done in detect_coding called via decode_coding above.  */

      /* If a coding system for encoding is not yet decided, we set it as the same as
	 coding-system for decoding.  ...  */
      eassert! ((*p).outfd < FD_SETSIZE);
      if (NILP ((*p).encode_coding_system) && (*p).outfd >= 0
	  && !proc_encode_coding_system[(*p).outfd as usize].is_null ())
	{
	  pset_encode_coding_system
	    (p, coding_inherit_eol_type (Vlast_coding_system_used, Qnil ()));
	  setup_coding_system ((*p).encode_coding_system,
			       proc_encode_coding_system[(*p).outfd as usize]);
	}
    }

  if ((*coding).carryover_bytes > 0)
    {
      if (SCHARS ((*p).decoding_buf) < (*coding).carryover_bytes as EMACS_INT)
	{ pset_decoding_buf (p, make_uninit_string ((*coding).carryover_bytes as EMACS_INT)); }
      memcpy (SDATA ((*p).decoding_buf) as *mut c_void,
	      (*coding).carryover.as_ptr () as *const c_void,
	      (*coding).carryover_bytes as usize);
      (*p).decoding_carryover = (*coding).carryover_bytes;
    }
}

unsafe extern "C" {
  /// C: Lisp_Object coding_inherit_eol_type (Lisp_Object, Lisp_Object)   (coding.c)
  fn coding_inherit_eol_type (coding: Lisp_Object, eol: Lisp_Object) -> Lisp_Object;
}

/* C 6482-6539: `read_and_insert_process_output`.  */
unsafe fn read_and_insert_process_output (p: *mut Lisp_Process, buf: *mut c_char,
					  nread: ptrdiff_t,
					  process_coding: *mut coding_system)
{
  if (nread == 0 || NILP ((*p).buffer) || !BUFFER_LIVE_P (XBUFFER ((*p).buffer)))
    { return; }

  let mut old_read_only: Lisp_Object = lisp_h_Qnil ();
  let mut old_begv: ptrdiff_t = 0;
  let mut old_zv: ptrdiff_t = 0;
  let mut before: ptrdiff_t = 0;
  let mut before_byte: ptrdiff_t = 0;
  let mut opoint: ptrdiff_t = 0;
  let mut opoint_byte: ptrdiff_t = 0;

  read_process_output_before_insert (p, &raw mut old_read_only, &raw mut old_begv,
				     &raw mut old_zv, &raw mut before, &raw mut before_byte,
				     &raw mut opoint, &raw mut opoint_byte);

  /* Adapted from call_process.  */
  prepare_to_modify_buffer (PT! (), PT! (), std::ptr::null_mut ());

  if (NILP (BVAR! (XBUFFER ((*p).buffer), enable_multibyte_characters_))
      && ! CODING_MAY_REQUIRE_DECODING (process_coding) != 0)
    {
      /* For compatibility with the long-standing behavior of internal-default-process-filter
	 we insert before markers, both here and in the 'else' branch.  */
      insert_1_both (buf, nread, nread, false, false, true);
      signal_after_change (PT! () - nread, 0, nread);
    }
  else
    {			/* We have to decode the input.  */
      let mut curbuf: Lisp_Object = lisp_h_Qnil ();
      let count1: specpdl_ref = SPECPDL_INDEX ();

      XSETBUFFER! (curbuf, (current_buffer! ()));
      /* See the comment above about inserting before markers.  */
      coding_bits! (insert_before_markers, process_coding, true);
      /* We cannot allow after-change-functions be run during decoding, because that might
	 modify the buffer, while we rely on process_coding.produced to faithfully reflect
	 inserted text until we TEMP_SET_PT_BOTH below.  */
      specbind (Qinhibit_modification_hooks (), Qt ());
      decode_coding_c_string (process_coding,
			      buf as *mut u8, nread, curbuf);
      unbind_to (count1, Qnil ());

      read_process_output_set_last_coding_system (p, process_coding);

      TEMP_SET_PT_BOTH (PT! () + (*process_coding).produced_char,
			PT_BYTE! () + (*process_coding).produced);
      signal_after_change (PT! () - (*process_coding).produced_char,
			   0, (*process_coding).produced_char);
    }

  read_process_output_after_insert (p, &raw mut old_read_only, old_begv, old_zv,
				    before, before_byte, opoint, opoint_byte);
}

unsafe extern "C" {
  /// C: void insert_1_both (const char *, ptrdiff_t, ptrdiff_t, bool, bool, bool)
  ///    (lisp.h 5785, insdel.c)
  fn insert_1_both (str_: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
		     inherit: bool, prepare: bool, before_markers: bool);
}

/* C 6541-6606: `read_and_dispose_of_process_output`.  C's `#if 0` block (C 6550-6553) is dead
   in every configuration and is recorded, not carried (P-08).  */
unsafe fn read_and_dispose_of_process_output (p: *mut Lisp_Process, chars: *mut c_char,
					      nbytes: ptrdiff_t,
					      coding: *mut coding_system)
{
  let outstream: Lisp_Object = (*p).filter;
  let mut text: Lisp_Object;
  let outer_running_asynch_code: bool = running_asynch_code;
  let waiting: c_int = waiting_for_user_input_p! ();

  /* We inhibit quit here instead of just catching it so that hitting ^G when a filter
     happens to be running won't screw it up.  */
  specbind (Qinhibit_quit (), Qt ());
  specbind (Qlast_nonmenu_event (), Qt ());

  /* In case we get recursively called, and we already saved the match data
     nonrecursively, save the same match data in safely recursive fashion.  */
  if (outer_running_asynch_code)
    {
      let mut tem: Lisp_Object;
      /* Don't clobber the CURRENT match data, either!  */
      tem = Fmatch_data (Qnil (), Qnil (), Qnil ());
      restore_search_regs ();
      record_unwind_save_match_data ();
      Fset_match_data (tem, Qt ());
    }

  /* For speed, if a search happens within this code, save the match data in a special
     nonrecursive fashion.  */
  running_asynch_code = true;

  if (fast_read_process_output
      && EQ ((*p).filter, Qinternal_default_process_filter ()))
    { read_and_insert_process_output (p, chars, nbytes, coding); }
  else
    {
      decode_coding_c_string (coding, chars as *mut u8, nbytes, Qt ());
      text = (*coding).dst_object;

      read_process_output_set_last_coding_system (p, coding);

      if (SBYTES (text) > 0)
	/* FIXME: It's wrong to wrap or not based on debug-on-error, and sometimes it's
	   simply wrong to wrap (e.g. when called from accept-process-output).  */
	{ internal_condition_case_1 (Some (read_process_output_call),
				     list3 (outstream, make_lisp_proc (p), text),
				     if !NILP (Vdebug_on_error) { Qnil () } else { Qerror () },
				     Some (read_process_output_error_handler)); }

    }

  /* If we saved the match data nonrecursively, restore it now.  */
  restore_search_regs ();
  running_asynch_code = outer_running_asynch_code;

  /* Restore waiting_for_user_input_p as it was when we were called, in case the filter
     clobbered it.  */
  waiting_for_user_input_p! () = waiting;
}

DEFUN! ("internal-default-process-filter", Finternal_default_process_filter,
       Sinternal_default_process_filter, 2, 2, 0,
       doc: "Function used as default process filter.\nThis inserts the process's output into its buffer, if there is one.\nOtherwise it discards the output.",
       (proc_: Lisp_Object, text: Lisp_Object),
       {
  let p: *mut Lisp_Process;
  let mut text: Lisp_Object = text;

  CHECK_PROCESS (proc_);
  p = XPROCESS (proc_);
  CHECK_STRING (text);

  if (!NILP ((*p).buffer) && BUFFER_LIVE_P (XBUFFER ((*p).buffer)))
    {
      let mut old_read_only: Lisp_Object = lisp_h_Qnil ();
      let mut old_begv: ptrdiff_t = 0;
      let mut old_zv: ptrdiff_t = 0;
      let mut before: ptrdiff_t = 0;
      let mut before_byte: ptrdiff_t = 0;
      let mut opoint: ptrdiff_t = 0;
      let mut opoint_byte: ptrdiff_t = 0;

      read_process_output_before_insert (p, &raw mut old_read_only, &raw mut old_begv,
					 &raw mut old_zv, &raw mut before, &raw mut before_byte,
					 &raw mut opoint, &raw mut opoint_byte);

      /* Adjust the multibyteness of TEXT to that of the buffer.  */
      if (NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_))
	  != ! STRING_MULTIBYTE (text))
	{ text = if STRING_MULTIBYTE (text)
	  { Fstring_as_unibyte (text) }
	  else { Fstring_to_multibyte (text) }; }
      /* Insert before markers in case we are inserting where the buffer's mark is, and
	 the user's next command is Meta-y.  */
      insert_from_string_before_markers (text, 0, 0,
					 SCHARS (text), SBYTES (text), false);

      read_process_output_after_insert (p, &raw mut old_read_only, old_begv, old_zv,
					before, before_byte, opoint, opoint_byte);
    }
  return Qnil ();
});

/* The `DEFVAR_BOOL` carrier this file owns (C 9xxx, `fast-read-process-output`; status 3.29).  */
#[unsafe(no_mangle)]
pub static mut fast_read_process_output: bool = false;

unsafe extern "C" {
  /// C: bool running_asynch_code   (search.c, lisp.h 5146)
  static mut running_asynch_code: bool;
  /// C: void restore_search_regs (void)   (search.c)
  fn restore_search_regs ();
  /// C: void record_unwind_save_match_data (void)   (search.c)
  fn record_unwind_save_match_data ();
  /// C: Lisp_Object internal_condition_case_1 (Lisp_Object (*) (Lisp_Object), Lisp_Object,
  ///                                          Lisp_Object, Lisp_Object (*) (Lisp_Object))
  ///    (lisp.h 4595, eval.c)
  fn internal_condition_case_1 (bfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>,
				arg: Lisp_Object, handlers: Lisp_Object,
				hfun: Option<unsafe extern "C" fn (Lisp_Object) -> Lisp_Object>)
    -> Lisp_Object;
  /// C: void insert_from_string_before_markers (Lisp_Object, ptrdiff_t, ptrdiff_t,
  ///        ptrdiff_t, ptrdiff_t, bool)   (lisp.h 5790, insdel.c)
  fn insert_from_string_before_markers (string: Lisp_Object, pos: ptrdiff_t, pos_byte: ptrdiff_t,
					nchars: ptrdiff_t, nbytes: ptrdiff_t, inherit: bool);
}

/* Create a new entry in write_queue.  INPUT_OBJ should be a buffer, string Qt, or Qnil.  BUF
   is a pointer to the string sequence of the input_obj or a C string in case of Qt or
   Qnil.  */
/* C 6671-6699: `write_queue_push`.  */
unsafe fn write_queue_push (p: *mut Lisp_Process, input_obj: Lisp_Object,
			    buf: *const c_char, len: ptrdiff_t, front: bool)
{
  let offset: ptrdiff_t;
  let entry: Lisp_Object;
  let obj: Lisp_Object;

  if (STRINGP (input_obj))
    {
      offset = buf.offset_from (SDATA (input_obj) as *const c_char);
      obj = input_obj;
    }
  else
    {
      offset = 0;
      obj = make_unibyte_string (buf, len);
    }

  entry = Fcons (obj, Fcons (make_fixnum (offset as EMACS_INT), make_fixnum (len as EMACS_INT)));

  if (front)
    { pset_write_queue (p, Fcons (entry, (*p).write_queue)); }
  else
    { pset_write_queue (p, nconc2 ((*p).write_queue, list1 (entry))); }
}

/* Remove the first element in the write_queue of process P, put its contents in OBJ, BUF and
   LEN, and return true.  If the write_queue is empty, return false.  */
/* C 6701-6720: `write_queue_pop`.  */
unsafe fn write_queue_pop (p: *mut Lisp_Process, obj: *mut Lisp_Object,
			   buf: *mut *const c_char, len: *mut ptrdiff_t) -> bool
{
  let entry: Lisp_Object;
  let offset_length: Lisp_Object;
  let offset: ptrdiff_t;

  if (NILP ((*p).write_queue))
    { return false; }

  entry = XCAR ((*p).write_queue);
  pset_write_queue (p, XCDR ((*p).write_queue));

  *obj = XCAR (entry);
  offset_length = XCDR (entry);

  *len = XFIXNUM (XCDR (offset_length));
  offset = XFIXNUM (XCAR (offset_length));
  *buf = (SDATA (*obj) as *mut c_char).offset (offset as isize);

  return true;
}

/* Send some data to process PROC.  BUF is the beginning of the data; LEN is the number of
   characters.  OBJECT is the Lisp object that the data comes from.  If OBJECT is nil or t,
   it means that the data comes from C string.  ...  */
/* C 6734-6961: `send_process`.  */
unsafe fn send_process (proc_: Lisp_Object, buf: *const c_char, len: ptrdiff_t,
			object: Lisp_Object)
{
  let p: *mut Lisp_Process = XPROCESS (proc_);
  let mut rv: ssize_t;
  let coding: *mut coding_system;
  let mut buf: *const c_char = buf;
  let mut len: ptrdiff_t = len;
  let mut object: Lisp_Object = object;

  if (NETCONN_P (proc_))
    {
      wait_while_connecting (proc_);
      wait_for_tls_negotiation (proc_);
    }

  if (process_bits! (raw_status_new, p))
    { update_status (p); }
  if (! EQ ((*p).status, Qrun ()))
    { error (c"Process %s not running: %s".as_ptr (), SDATA ((*p).name),
	     SDATA (status_message (p))); }
  if ((*p).outfd < 0)
    { error (c"Output file descriptor of %s is closed".as_ptr (), SDATA ((*p).name)); }

  eassert! ((*p).outfd < FD_SETSIZE);
  coding = proc_encode_coding_system[(*p).outfd as usize];
  Vlast_coding_system_used = CODING_ID_NAME ((*coding).id as c_int);

  if ((STRINGP (object) && STRING_MULTIBYTE (object))
      || (BUFFERP (object)
	  && !NILP (BVAR! (XBUFFER (object), enable_multibyte_characters_)))
      || EQ (object, Qt ()))
    {
      pset_encode_coding_system
	(p, complement_process_encoding_system ((*p).encode_coding_system));
      if (!EQ (Vlast_coding_system_used, (*p).encode_coding_system))
	{
	  /* The coding system for encoding was changed to raw-text because we sent a
	     unibyte text previously.  Now we are sending a multibyte text, thus we must
	     encode it by the original coding system specified for the current process.
	     ...  */
	  setup_coding_system ((*p).encode_coding_system, coding);
	  Vlast_coding_system_used = (*p).encode_coding_system;
	}
      coding_bits! (src_multibyte, coding, 1);
    }
  else
    {
      coding_bits! (src_multibyte, coding, 0);
      /* For sending a unibyte text, character code conversion should not take place but
	 EOL conversion should.  So, setup raw-text or one of the subsidiary if we have not
	 yet done it.  */
      if (CODING_REQUIRE_ENCODING (coding) != 0)
	{
	  if (CODING_REQUIRE_FLUSHING (coding) != 0)
	    {
	      /* But, before changing the coding, we must flush out data.  */
	      coding_bits! (mode, coding, coding_bits! (mode, coding) | CODING_MODE_LAST_BLOCK);
	      send_process (proc_, c"".as_ptr (), 0, Qt ());
	      coding_bits! (mode, coding, coding_bits! (mode, coding) & CODING_MODE_LAST_BLOCK);
	    }
	  setup_coding_system (raw_text_coding_system
			       (Vlast_coding_system_used),
			       coding);
	  coding_bits! (src_multibyte, coding, 0);
	}
    }
  coding_bits! (dst_multibyte, coding, 0);

  if (CODING_REQUIRE_ENCODING (coding) != 0)
    {
      (*coding).dst_object = Qt ();
      if (BUFFERP (object))
	{
	  let mut from_byte: ptrdiff_t;
	  let mut from: ptrdiff_t;
	  let mut to: ptrdiff_t;
	  let mut save_pt: ptrdiff_t;
	  let mut save_pt_byte: ptrdiff_t;
	  let cur: *mut buffer = (current_buffer! ());

	  set_buffer_internal (XBUFFER (object));
	  save_pt = PT! (); save_pt_byte = PT_BYTE! ();

	  from_byte = PTR_BYTE_POS (buf as *mut u8);
	  from = BYTE_TO_CHAR (from_byte);
	  to = BYTE_TO_CHAR (from_byte + len);
	  TEMP_SET_PT_BOTH (from, from_byte);
	  encode_coding_object (coding, object, from, from_byte,
				to, from_byte + len, Qt ());
	  TEMP_SET_PT_BOTH (save_pt, save_pt_byte);
	  set_buffer_internal (cur);
	}
      else if (STRINGP (object))
	{
	  encode_coding_object (coding, object, 0, 0, SCHARS (object),
				SBYTES (object), Qt ());
	}
      else
	{
	  (*coding).dst_object = make_unibyte_string (buf, len);
	  (*coding).produced = len;
	}

      len = (*coding).produced;
      object = (*coding).dst_object;
      buf = SDATA (object) as *const c_char;
    }

  /* If there is already data in the write_queue, put the new data in the back of queue.
     Otherwise, ignore it.  */
  if (!NILP ((*p).write_queue))
    { write_queue_push (p, object, buf, len, false); }

  loop   /* while !NILP (p->write_queue) */
    {
      let mut cur_len: ptrdiff_t = -1;
      let mut cur_buf: *const c_char = std::ptr::null ();
      let mut cur_object: Lisp_Object = lisp_h_Qnil ();

      /* If write_queue is empty, ignore it.  */
      if (!write_queue_pop (p, &raw mut cur_object, &raw mut cur_buf, &raw mut cur_len))
	{
	  cur_len = len;
	  cur_buf = buf;
	  cur_object = object;
	}

      while (cur_len > 0)
	{
	  /* Send this batch, using one or more write calls.  */
	  let mut written: ptrdiff_t = 0;
	  let outfd: c_int = (*p).outfd;
	  if (outfd < 0)
	    { error (c"Output file descriptor of %s is closed".as_ptr (),
		     SDATA ((*p).name)); }
	  eassert! (0 <= outfd && outfd < FD_SETSIZE);
	  /* `#ifdef DATAGRAM_SOCKETS` is live.  */
	  if (DATAGRAM_CHAN_P (outfd))
	    {
	      loop
		{
		  rv = sendto (outfd, cur_buf as *const c_void, cur_len as usize, 0,
			       datagram_address[outfd as usize].sa,
			       datagram_address[outfd as usize].len as socklen_t);
		  if (! (rv < 0 && *errno () == EINTR))
		    { break; }
		  if (pending_signals)
		    { process_pending_signals (); }
		}

	      if (rv >= 0)
		{ written = rv as ptrdiff_t; }
	      else if (*errno () == EMSGSIZE)
		{ report_file_error (c"Sending datagram".as_ptr (), proc_); }
	    }
	  else
	    {
	      let mut gnutls_written: bool = false;
	      /* C 6890-6893's `#ifdef HAVE_GNUTLS` arm.  */
	      #[cfg (feature = "have_gnutls")]
	      {
		if (process_bits! (gnutls_p, p) && !(*p).gnutls_state.is_null ())
		  {
		    written = emacs_gnutls_write (p, cur_buf, cur_len) as ptrdiff_t;
		    gnutls_written = true;
		  }
	      }
	      if !gnutls_written
	      { written = emacs_write_sig (outfd, cur_buf as *const c_void, cur_len); }
	      rv = if written != 0 { 0 } else { -1 };
	      if ((*p).read_output_delay > 0
		  && process_bits! (adaptive_read_buffering, p) == 1)
		{
		  (*p).read_output_delay = 0;
		  process_output_delay_count -= 1;
		  process_bits! (read_output_skip, p, 0);
		}
	    }

	  if (rv < 0)
	    {
	      if (would_block (*errno ()))
		/* Buffer is full.  Wait, accepting input; that may allow the program to
		   finish doing output and read more.  */
		{
		  /* C 6913-6936's `#ifdef BROKEN_PTY_READ_AFTER_EAGAIN` arm: a gross hack to
		     work around a bug in FreeBSD.  */
		  #[cfg (feature = "broken_pty_read_after_eagain")]
		  {
		    if (*errno () == EAGAIN)
		      {
			let flags: c_int = FWRITE;
			ioctl ((*p).outfd, TIOCFLUSH as c_ulong, &raw const flags);
		      }
		  }

		  /* Put what we should have written in write_queue.  */
		  write_queue_push (p, cur_object, cur_buf, cur_len, true);
		  wait_reading_process_output (0, 20 * 1000 * 1000,
					       0, 0, Qnil (), std::ptr::null_mut (), 0);
		  /* Reread queue, to see what is left.  */
		  break;
		}
	      else if (*errno () == EPIPE)
		{
		  close_process_fd ((*p).open_fd.as_mut_ptr ().offset (WRITE_TO_SUBPROCESS as isize));
		  (*p).outfd = -1;
		  error (c"Process %s no longer connected to pipe; closed it".as_ptr (),
			 SDATA ((*p).name));
		}
	      else
		/* This is a real error.  */
		{ report_file_error (c"Writing to process".as_ptr (), proc_); }
	    }
	  cur_buf = cur_buf.offset (written as isize);
	  cur_len -= written;
	}
      if NILP ((*p).write_queue) { break; }
    }
}

DEFUN! ("process-send-region", Fprocess_send_region, Sprocess_send_region, 3, 3, 0,
       doc: "Send current contents of region as input to PROCESS.\nPROCESS may be a process, a buffer, the name of a process or buffer, or\nnil, indicating the current buffer's process.\nCalled from program, takes three arguments, PROCESS, START and END.\nIf the region is larger than the input buffer of the process (the\nlength of which depends on the process connection type and the\noperating system), it is sent in several bunches.  This may happen\neven for shorter regions.  Output from processes can arrive in between\nbunches.\n\nIf PROCESS is a non-blocking network process that hasn't been fully\nset up yet, this function will block until socket setup has completed.",
       (process: Lisp_Object, start: Lisp_Object, end: Lisp_Object),
       {
  let proc_: Lisp_Object = get_process (process);
  let mut start: Lisp_Object = start;
  let mut end: Lisp_Object = end;
  let start_byte: ptrdiff_t;
  let end_byte: ptrdiff_t;

  validate_region (&raw mut start, &raw mut end);

  start_byte = CHAR_TO_BYTE (XFIXNUM (start));
  end_byte = CHAR_TO_BYTE (XFIXNUM (end));

  if (XFIXNUM (start) < GPT! () && XFIXNUM (end) > GPT! ())
    { move_gap_both (XFIXNUM (start), start_byte); }

  if (NETCONN_P (proc_))
    { wait_while_connecting (proc_); }

  send_process (proc_, BYTE_POS_ADDR (start_byte) as *const c_char,
		end_byte - start_byte, Fcurrent_buffer ());

  return Qnil ();
});

DEFUN! ("process-send-string", Fprocess_send_string, Sprocess_send_string, 2, 2, 0,
       doc: "Send PROCESS the contents of STRING as input.\nPROCESS may be a process, a buffer, the name of a process or buffer, or\nnil, indicating the current buffer's process.\nIf STRING is larger than the input buffer of the process (the length\nof which depends on the process connection type and the operating\nsystem), it is sent in several bunches.  This may happen even for\nshorter strings.  Output from processes can arrive in between bunches.\n\nIf PROCESS is a non-blocking network process that hasn't been fully\nset up yet, this function will block until socket setup has completed.",
       (process: Lisp_Object, string: Lisp_Object),
       {
  CHECK_STRING (string);
  let proc_: Lisp_Object = get_process (process);
  send_process (proc_, SSDATA (string) as *const c_char,
		SBYTES (string), string);
  return Qnil ();
});

unsafe extern "C" {
  /// C: Lisp_Object nconc2 (Lisp_Object, Lisp_Object)   (lisp.h 4427, fns.c)
  fn nconc2 (a: Lisp_Object, b: Lisp_Object) -> Lisp_Object;
  /// C: ssize_t sendto (int, const void *, size_t, int, const struct sockaddr *, socklen_t)
  fn sendto (socket: c_int, buffer: *const c_void, length: usize, flags: c_int,
	     address: *const sockaddr, address_len: socklen_t) -> ssize_t;
  /// C: ptrdiff_t emacs_write_sig (int, const void *, ptrdiff_t)   (lisp.h 5324, sysdep.c)
  fn emacs_write_sig (fd: c_int, buf: *const c_void, nbyte: ptrdiff_t) -> ptrdiff_t;
  /// C: ssize_t emacs_gnutls_write (struct Lisp_Process *, const char *, ptrdiff_t)
  ///    (gnutls.c, the `HAVE_GNUTLS` arm)
  fn emacs_gnutls_write (p: *mut Lisp_Process, buf: *const c_char, nbyte: ptrdiff_t) -> ssize_t;
  /// C: Lisp_Object raw_text_coding_system (Lisp_Object)   (coding.h 700)
  fn raw_text_coding_system (coding: Lisp_Object) -> Lisp_Object;
}

/* Return the foreground process group for the tty/pty that the process P uses.  */
/* C 7023-7044: `emacs_get_tty_pgrp`.  */
unsafe fn emacs_get_tty_pgrp (p: *mut Lisp_Process) -> pid_t
{
  let mut gid: pid_t = -1;

  /* `#ifdef TIOCGPGRP` is live.  */
  if (ioctl ((*p).infd, TIOCGPGRP as c_ulong, &raw mut gid) == -1 && ! NILP ((*p).tty_name))
    {
      let mut fd: c_int;
      /* Some OS:es (Solaris 8/9) does not allow TIOCGPGRP from the master side.  Try the
	 slave side.  */
      fd = emacs_open (SSDATA ((*p).tty_name) as *const c_char, O_RDONLY, 0);

      if (fd != -1)
	{
	  ioctl (fd, TIOCGPGRP as c_ulong, &raw mut gid);
	  emacs_close (fd);
	}
    }

  return gid;
}

DEFUN! ("process-running-child-p", Fprocess_running_child_p, Sprocess_running_child_p, 0, 1, 0,
       doc: "Return non-nil if PROCESS has given control of its terminal to a child.\nIf the operating system does not make it possible to find out, return t.\nIf it's possible to find out, return the numeric ID of the foreground\nprocess group if PROCESS did give control of its terminal to a\nchild process, and return nil if it didn't.\n\nPROCESS must be a real subprocess, not a connection.",
       (process: Lisp_Object),
       {
  /* Initialize in case ioctl doesn't exist or gives an error, in a way that will cause
     returning t.  */
  let proc_: Lisp_Object = get_process (process);
  let p: *mut Lisp_Process = XPROCESS (proc_);

  if (!EQ ((*p).r#type, Qreal ()))
    { error (c"Process %s is not a subprocess".as_ptr (),
	     SDATA ((*p).name)); }
  if ((*p).infd < 0)
    { error (c"Process %s is not active".as_ptr (),
	     SDATA ((*p).name)); }

  let gid: pid_t = emacs_get_tty_pgrp (p);

  if (gid == (*p).pid)
    { return Qnil (); }
  if (gid != -1)
    { return make_fixnum (gid as EMACS_INT); }
  return Qt ();
});

/* Send a signal number SIGNO to PROCESS.  ...  */
/* C 7093-7226: `process_send_signal`.  */
unsafe fn process_send_signal (process: Lisp_Object, signo: c_int, current_group: Lisp_Object,
			       nomsg: bool)
{
  let proc_: Lisp_Object;
  let p: *mut Lisp_Process;
  let mut gid: pid_t;
  let mut no_pgrp: bool = false;
  let mut current_group: Lisp_Object = current_group;

  proc_ = get_process (process);
  p = XPROCESS (proc_);

  if (!EQ ((*p).r#type, Qreal ()))
    { error (c"Process %s is not a subprocess".as_ptr (),
	     SDATA ((*p).name)); }
  if ((*p).infd < 0)
    { error (c"Process %s is not active".as_ptr (),
	     SDATA ((*p).name)); }

  if (! process_bits! (pty_in, p))
    { current_group = Qnil (); }

  /* If we are using pgrps, get a pgrp number and make it negative.  */
  if (NILP (current_group))
    /* Send the signal to the shell's process group.  */
    { gid = (*p).pid; }
  else
    {
      /* `#ifdef SIGNALS_VIA_CHARACTERS` is live.  */
      /* If possible, send signals to the entire pgrp by sending an input character to
	 it.  */

      let mut t: termios = std::mem::zeroed ();
      let mut sig_char: *mut cc_t = std::ptr::null_mut ();

      tcgetattr ((*p).infd, &raw mut t);

      match signo
	{
	SIGINT_ =>
	  { sig_char = &raw mut t.c_cc[VINTR]; }
	SIGQUIT_ =>
	  { sig_char = &raw mut t.c_cc[VQUIT]; }
	SIGTSTP_ =>
	  {
	    /* `#ifdef VSWTCH`'s arm is dead on this target (feature `vswtch`).  */
	    #[cfg (feature = "vswtch")]
	    { sig_char = &raw mut t.c_cc[VSWTCH]; }
	    #[cfg (not (feature = "vswtch"))]
	    { sig_char = &raw mut t.c_cc[VSUSP]; }
	  }
	_ => { }
	}

      if (!sig_char.is_null () && *sig_char != CDISABLE)
	{
	  send_process (proc_, sig_char as *const c_char, 1, Qnil ());
	  return;
	}
      /* If we can't send the signal with a character, fall through and send it another
	 way.  */

      /* `#ifdef TIOCGPGRP` is live.  */
      /* Get the current pgrp using the tty itself, if we have that.  Otherwise, use the
	 pty to get the pgrp.  ...  */

      gid = emacs_get_tty_pgrp (p);

      if (gid == -1)
	/* If we can't get the information, assume the shell owns the tty.  */
	{ gid = (*p).pid; }

      /* It is not clear whether anything really can set GID to -1.  Perhaps on some
	 system one of those ioctls can or could do so.  Or perhaps this is vestigial.  */
      if (gid == -1)
	{ no_pgrp = true; }

      /* If current_group is lambda, and the shell owns the terminal, don't send any
	 signal.  */
      if (EQ (current_group, Qlambda ()) && gid == (*p).pid)
	{ return; }
    }

  /* `#ifdef SIGCONT` is live.  */
  if (signo == SIGCONT)
    {
      process_bits! (raw_status_new, p, 0);
      pset_status (p, Qrun ());
      process_tick += 1;
      (*p).tick = process_tick;
      if (!nomsg)
	{
	  status_notify (std::ptr::null_mut (), std::ptr::null_mut ());
	  redisplay_preserve_echo_area (13);
	}
    }

  /* C 7207-7211's `#ifdef TIOCSIGSEND` arm: work around a HP-UX 7.0 bug that mishandles
     signals to subjobs.  */
  #[cfg (feature = "tiocsigsend")]
  {
    if (! NILP (current_group)
	&& ioctl ((*p).infd, TIOCSIGSEND as c_ulong, signo) != -1)
      { return; }
  }

  /* If we don't have process groups, send the signal to the immediate subprocess.  That
     isn't really right, but it's better than any obvious alternative.  */
  let pid: pid_t = if no_pgrp { gid } else { - gid };

  /* Do not kill an already-reaped process, as that could kill an innocent bystander that
     happens to have the same process ID.  */
  let mut oldset: sigset_t = std::mem::zeroed ();
  block_child_signal (&raw mut oldset);
  if process_bits! (alive, p)
    { kill (pid, signo); }
  unblock_child_signal (&raw const oldset);
}

/* `<termios.h>` and `<sys/ioctl.h>` entries `process_send_signal`/`emacs_get_tty_pgrp` need;
   the values are the frozen target's (the same clang measurements `femacs/src/sysdep.rs`
   records for its own copy of these declarations — no mirror carries `<termios.h>`).  */
/// C: `<termios.h>`: `typedef unsigned long tcflag_t;`
pub type tcflag_t = c_ulong;
/// C: `<termios.h>`: `typedef unsigned char cc_t;`
pub type cc_t = c_uchar;
/// C: `<termios.h>`: `#define NCCS 20`.
pub const NCCS: usize = 20;
/// C: `<termios.h>`'s `struct termios`, measured 72/8 (`c_iflag`@0, `c_oflag`@8, `c_cflag`@16,
/// `c_lflag`@24, `c_cc`@32, `c_ispeed`@56, `c_ospeed`@64).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct termios
{
  pub c_iflag: tcflag_t,
  pub c_oflag: tcflag_t,
  pub c_cflag: tcflag_t,
  pub c_lflag: tcflag_t,
  pub c_cc: [cc_t; NCCS],
  pub c_ispeed: c_ulong,
  pub c_ospeed: c_ulong,
}
const _: () = core::assert! (std::mem::size_of::<termios> () == 72);
const _: () = core::assert! (std::mem::offset_of! (termios, c_iflag) == 0);
const _: () = core::assert! (std::mem::offset_of! (termios, c_oflag) == 8);
const _: () = core::assert! (std::mem::offset_of! (termios, c_cflag) == 16);
const _: () = core::assert! (std::mem::offset_of! (termios, c_lflag) == 24);
const _: () = core::assert! (std::mem::offset_of! (termios, c_cc) == 32);
const _: () = core::assert! (std::mem::offset_of! (termios, c_ispeed) == 56);
const _: () = core::assert! (std::mem::offset_of! (termios, c_ospeed) == 64);
/// C: `<termios.h>`'s `c_cc` indices, measured: VINTR 8, VQUIT 9, VSUSP 10.
pub const VINTR: usize = 8;
pub const VQUIT: usize = 9;
pub const VSUSP: usize = 10;
/// C: `"systty.h"`: `#define CDISABLE _POSIX_VDISABLE`, measured 255.
pub const CDISABLE: cc_t = 255;
/// C: `#define TIOCGPGRP 1074033783` (`<sys/ioctl.h>`)
pub const TIOCGPGRP: c_int = 1074033783;
/* `SIGINT_`/`SIGQUIT_`/`SIGTSTP_` are the three `case` labels of C's `switch (signo)`; the
   mirror's `SIGINT`/`SIGQUIT`/`SIGTSTP` are used, so the arms below name them through the
   `match` patterns directly.  */
const SIGINT_: c_int = SIGINT;
const SIGQUIT_: c_int = SIGQUIT;
const SIGTSTP_: c_int = SIGTSTP;

unsafe extern "C" {
  /// C: int tcgetattr (int, struct termios *)   (<termios.h>)
  fn tcgetattr (fd: c_int, termios_p: *mut termios) -> c_int;
  /// C: int kill (pid_t, int)   (<signal.h>)
  fn kill (pid: pid_t, sig: c_int) -> c_int;
}

DEFUN! ("internal-default-interrupt-process", Finternal_default_interrupt_process,
       Sinternal_default_interrupt_process, 0, 2, 0,
       doc: "Default function to interrupt process PROCESS.\nIt shall be the last element in list `interrupt-process-functions'.\nSee function `interrupt-process' for more details on usage.",
       (process: Lisp_Object, current_group: Lisp_Object),
       {
  process_send_signal (process, SIGINT, current_group, false);
  return process;
});

DEFUN! ("interrupt-process", Finterrupt_process, Sinterrupt_process, 0, 2, 0,
       doc: "Interrupt process PROCESS.\nPROCESS may be a process, a buffer, or the name of a process or buffer.\nNo arg or nil means current buffer's process.\nSecond arg CURRENT-GROUP non-nil means send signal to\nthe current process-group of the process's controlling terminal\nrather than to the process's own process group.\nIf the process is a shell, this means interrupt current subjob\nrather than the shell.\n\nIf CURRENT-GROUP is `lambda', and if the shell owns the terminal,\ndon't send the signal.\n\nThis function calls the functions of `interrupt-process-functions' in\nthe order of the list, until one of them returns non-nil.",
       (process: Lisp_Object, current_group: Lisp_Object),
       {
  return CALLN! (Frun_hook_with_args_until_success, Qinterrupt_process_functions (),
		 process, current_group);
});

DEFUN! ("kill-process", Fkill_process, Skill_process, 0, 2, "(list (read-process-name \"Kill process\"))",
       doc: "Kill process PROCESS.  May be process or name of one.\nSee function `interrupt-process' for more details on usage.",
       (process: Lisp_Object, current_group: Lisp_Object),
       {
  process_send_signal (process, SIGKILL, current_group, false);
  return process;
});

DEFUN! ("quit-process", Fquit_process, Squit_process, 0, 2, 0,
       doc: "Send QUIT signal to process PROCESS.  May be process or name of one.\nSee function `interrupt-process' for more details on usage.",
       (process: Lisp_Object, current_group: Lisp_Object),
       {
  process_send_signal (process, SIGQUIT, current_group, false);
  return process;
});

DEFUN! ("stop-process", Fstop_process, Sstop_process, 0, 2, 0,
       doc: "Stop process PROCESS.  May be process or name of one.\nSee function `interrupt-process' for more details on usage.\nIf PROCESS is a network or serial or pipe connection, inhibit handling\nof incoming traffic.",
       (process: Lisp_Object, current_group: Lisp_Object),
       {
  if (PROCESSP (process) && (NETCONN_P (process) || SERIALCONN_P (process)
			     || PIPECONN_P (process)))
    {
      let p: *mut Lisp_Process;

      p = XPROCESS (process);
      if (NILP ((*p).command)
	  && (*p).infd >= 0)
	{ delete_read_fd ((*p).infd); }
      pset_command (p, Qt ());
      return process;
    }
  /* C 7290-7294's `#ifndef SIGTSTP` arm (the macro is defined on this target).  */
  #[cfg (feature = "no_sigtstp")]
  {
    error (c"No SIGTSTP support".as_ptr ());
  }
  /* The `#else` arm (this target's).  */
  #[cfg (not (feature = "no_sigtstp"))]
  process_send_signal (process, SIGTSTP, current_group, false);
  return process;
});

DEFUN! ("continue-process", Fcontinue_process, Scontinue_process, 0, 2, 0,
       doc: "Continue process PROCESS.  May be process or name of one.\nSee function `interrupt-process' for more details on usage.\nIf PROCESS is a network or serial process, resume handling of incoming\ntraffic.",
       (process: Lisp_Object, current_group: Lisp_Object),
       {
  if (PROCESSP (process) && (NETCONN_P (process) || SERIALCONN_P (process)
			     || PIPECONN_P (process)))
    {
      let p: *mut Lisp_Process;

      p = XPROCESS (process);
      eassert! ((*p).infd < FD_SETSIZE);
      if (EQ ((*p).command, Qt ())
	  && (*p).infd >= 0
	  && (!EQ ((*p).filter, Qt ()) || EQ ((*p).status, Qlisten ())))
	{
	  add_process_read_fd ((*p).infd);
	  /* C 7299-7307's `#ifdef WINDOWSNT` arm.  */
	  #[cfg (feature = "windowsnt")]
	  {
	    if (fd_info[(*p).infd as usize].flags & FILE_SERIAL) != 0
	      { PurgeComm (fd_info[(*p).infd as usize].hnd,
			   PURGE_RXABORT | PURGE_RXCLEAR); }
	  }
	  /* The `#else` arm (this target's).  */
	  #[cfg (not (feature = "windowsnt"))]
	  tcflush ((*p).infd, TCIFLUSH);
	}
      pset_command (p, Qnil ());
      return process;
    }
  /* C 7301-7305's `#else` arm of `#ifdef SIGCONT` (the macro is defined on this target).  */
  #[cfg (feature = "no_sigcont")]
  {
    error (c"No SIGCONT support".as_ptr ());
  }
  /* `#ifdef SIGCONT` is live.  */
  #[cfg (not (feature = "no_sigcont"))]
  process_send_signal (process, SIGCONT, current_group, false);
  return process;
});

/* Return the integer value of the signal whose abbreviation is ABBR, or a negative number
   if there is no such signal.  */
/* C 7347-7363: `abbr_to_signal`.  */
unsafe fn abbr_to_signal (name: *const c_char) -> c_int
{
  let mut i: c_int;
  let mut signo: c_int = 0;
  let mut sigbuf: [c_char; 20] = [0; 20]; /* Large enough for all valid signal abbreviations.  */
  let mut name: *const c_char = name;

  if (strncmp (name, c"SIG".as_ptr (), 3) == 0 || strncmp (name, c"sig".as_ptr (), 3) == 0)
    { name = name.offset (3); }

  i = 0;
  while i < std::mem::size_of_val (&sigbuf) as c_int
    {
      sigbuf[i as usize] = c_toupper (*name.offset (i as isize) as c_int) as c_char;
      if sigbuf[i as usize] == 0
	{ return if str2sig (sigbuf.as_ptr (), &raw mut signo) == 0 { signo } else { -1 }; }
      i += 1;
    }

  return -1;
}

DEFUN! ("internal-default-signal-process", Finternal_default_signal_process,
       Sinternal_default_signal_process, 2, 3, 0,
       doc: "Default function to send PROCESS the signal with code SIGCODE.\nIt shall be the last element in list `signal-process-functions'.\nSee function `signal-process' for more details on usage.",
       (process: Lisp_Object, sigcode: Lisp_Object, remote: Lisp_Object),
       {
  let mut process: Lisp_Object = process;
  let pid: pid_t;
  let mut signo: c_int;

  if (STRINGP (process))
    {
      let mut tem: Lisp_Object = Fget_process (process);
      if (NILP (tem))
	{
	  let mut len: ptrdiff_t = 0;
	  tem = string_to_number (SSDATA (process) as *const c_char, 10, &raw mut len);
	  if ((if IEEE_FLOATING_POINT { NILP (tem) } else { !NUMBERP (tem) })
	      || len != SBYTES (process))
	    { return Qnil (); }
	}
      process = tem;
    }
  else if (!NUMBERP (process))
    { process = get_process (process); }

  if (NILP (process))
    { return process; }

  let mut pid: pid_t = 0;
  if (NUMBERP (process))
    {
      /* C: `CONS_TO_INTEGER (process, pid_t, pid);` — the port's trait is implemented for
	 `i64`/`u64` (P-39), so the value is taken as `i64` and narrowed to C's `pid_t`.  */
      let mut pid_wide: i64 = 0;
      CONS_TO_INTEGER! (process, pid_wide);
      pid = pid_wide as pid_t;
    }
  else
    {
      CHECK_PROCESS (process);
      pid = (*XPROCESS (process)).pid;
      if (pid <= 0)
	{ error (c"Cannot signal process %s".as_ptr (), SDATA ((*XPROCESS (process)).name)); }
    }

  if (FIXNUMP (sigcode))
    { signo = check_integer_range (sigcode, INT_MIN as i64, INT_MAX as i64) as c_int; }
  else
    {
      let name: *mut c_char;

      CHECK_SYMBOL (sigcode);
      name = SSDATA (SYMBOL_NAME (sigcode)) as *mut c_char;

      signo = abbr_to_signal (name);
      if (signo < 0)
	{ error (c"Undefined signal name %s".as_ptr (), name); }
    }

  return make_fixnum (kill (pid, signo) as EMACS_INT);
});

DEFUN! ("signal-process", Fsignal_process, Ssignal_process, 2, 3, "(list (read-string \"Process (name or number): \") (read-signal-name))",
       doc: "Send PROCESS the signal with code SIGCODE.\nPROCESS may also be a number specifying the process id of the\nprocess to signal; in this case, the process need not be a child of\nthis Emacs.\nIf PROCESS is a process object which contains the property\n`remote-pid', or PROCESS is a number and REMOTE is a remote file name,\nPROCESS is interpreted as process on the respective remote host, which\nwill be the process to signal.\nIf PROCESS is a string, it is interpreted as process object with the\nrespective process name, or as a number.\nSIGCODE may be an integer, or a symbol whose name is a signal name.",
       (process: Lisp_Object, sigcode: Lisp_Object, remote: Lisp_Object),
       {
  return CALLN! (Frun_hook_with_args_until_success, Qsignal_process_functions (),
		 process, sigcode, remote);
});

unsafe extern "C" {
  /// C: int tcflush (int, int)   (<termios.h>)
  fn tcflush (fd: c_int, queue_selector: c_int) -> c_int;
  /// C: int str2sig (const char *, int *)   (sig2str.h 30)
  fn str2sig (name: *const c_char, signum: *mut c_int) -> c_int;
}
/// C: `#define TCIFLUSH 1` (`<termios.h>`)
pub const TCIFLUSH: c_int = 1;

DEFUN! ("process-send-eof", Fprocess_send_eof, Sprocess_send_eof, 0, 1, 0,
       doc: "Make PROCESS see end-of-file in its input.\nEOF comes after any text already sent to it.\nPROCESS may be a process, a buffer, the name of a process or buffer, or\nnil, indicating the current buffer's process.\nIf PROCESS is a network connection, or is a process communicating\nthrough a pipe (as opposed to a pty), then you cannot send any more\ntext to PROCESS after you call this function.\nIf PROCESS is a serial process, wait until all output written to the\nprocess has been transmitted to the serial port.",
       (process: Lisp_Object),
       {
  let proc_: Lisp_Object;
  let mut coding: *mut coding_system = std::ptr::null_mut ();
  let mut outfd: c_int;

  proc_ = get_process (process);

  if (NETCONN_P (proc_))
    { wait_while_connecting (proc_); }

  if (DATAGRAM_CONN_P (proc_))
    { return process; }

  outfd = (*XPROCESS (proc_)).outfd;
  eassert! (outfd < FD_SETSIZE);
  if (outfd >= 0)
    { coding = proc_encode_coding_system[outfd as usize]; }

  /* Make sure the process is really alive.  */
  if (process_bits! (raw_status_new, XPROCESS (proc_)))
    { update_status (XPROCESS (proc_)); }
  if (! EQ ((*XPROCESS (proc_)).status, Qrun ()))
    { error (c"Process %s not running: %s".as_ptr (), SDATA ((*XPROCESS (proc_)).name),
	     SDATA (status_message (XPROCESS (proc_)))); }

  if (!coding.is_null () && CODING_REQUIRE_FLUSHING (coding) != 0)
    {
      coding_bits! (mode, coding, coding_bits! (mode, coding) | CODING_MODE_LAST_BLOCK);
      send_process (proc_, c"".as_ptr (), 0, Qnil ());
    }

  if process_bits! (pty_in, XPROCESS (proc_))
    { send_process (proc_, c"\x04".as_ptr (), 1, Qnil ()); }
  else if (EQ ((*XPROCESS (proc_)).r#type, Qserial ()))
    {
      /* `#if !defined WINDOWSNT && defined HAVE_TCDRAIN` is live.  */
      if (tcdrain ((*XPROCESS (proc_)).outfd) != 0)
	{ report_file_error (c"Failed tcdrain".as_ptr (), Qnil ()); }
      /* Do nothing on Windows because writes are blocking.  */
    }
  else
    {
      let p: *mut Lisp_Process = XPROCESS (proc_);
      let old_outfd: c_int = (*p).outfd;
      let mut new_outfd: c_int;

      /* `#ifdef HAVE_SHUTDOWN` is live.  */
      /* If this is a network connection, or socketpair is used for communication with
	 the subprocess, call shutdown to cause EOF.  (In some old system, shutdown to
	 socketpair doesn't work.  Then we just can't win.)  */
      if (0 <= old_outfd
	  && (EQ ((*p).r#type, Qnetwork ()) || (*p).infd == old_outfd))
	{ shutdown (old_outfd, 1); }
      close_process_fd ((*p).open_fd.as_mut_ptr ().offset (WRITE_TO_SUBPROCESS as isize));
      new_outfd = inrange_fd (emacs_open (NULL_DEVICE, O_WRONLY, 0));
      if (new_outfd < 0)
	{ report_file_error (c"Opening null device".as_ptr (), Qnil ()); }
      (*p).open_fd[WRITE_TO_SUBPROCESS as usize] = new_outfd;
      (*p).outfd = new_outfd;

      eassert! (0 <= new_outfd && new_outfd < FD_SETSIZE);
      if (proc_encode_coding_system[new_outfd as usize].is_null ())
	{ proc_encode_coding_system[new_outfd as usize]
	    = xmalloc (std::mem::size_of::<coding_system> ()) as *mut coding_system; }
      if (old_outfd >= 0)
	{
	  eassert! (old_outfd < FD_SETSIZE);
	  /* C assigns the whole `struct coding_system`; the port copies the bytes (the type
	     has no `Copy` because of its union members, the same shape the coding.rs ledger
	     records).  */
	  std::ptr::copy_nonoverlapping (proc_encode_coding_system[old_outfd as usize],
					 proc_encode_coding_system[new_outfd as usize], 1);
	  memset (proc_encode_coding_system[old_outfd as usize] as *mut c_void, 0,
		  std::mem::size_of::<coding_system> ());
	}
      else
	{ setup_coding_system ((*p).encode_coding_system,
			       proc_encode_coding_system[new_outfd as usize]); }
    }
  return process;
});

unsafe extern "C" {
  /// C: int tcdrain (int)   (<termios.h>)
  fn tcdrain (fd: c_int) -> c_int;
  /// C: int shutdown (int, int)   (<sys/socket.h>)
  fn shutdown (socket: c_int, how: c_int) -> c_int;
}
/// C: config.h 1's `#define NULL_DEVICE "/dev/null"`.
const NULL_DEVICE: *const c_char = c"/dev/null".as_ptr ();

/* Set up `child_signal_read_fd' and `child_signal_write_fd'.  */
/* C 7582-7610: `child_signal_init`.  */
#[unsafe(no_mangle)]
pub unsafe fn child_signal_init ()
{
  /* Either both are initialized, or both are uninitialized.  */
  eassert! ((child_signal_read_fd < 0) == (child_signal_write_fd < 0));

  /* `#ifndef WINDOWSNT` is live.  */
  if (0 <= child_signal_read_fd)
    { return; /* already done */ }

  let mut fds: [c_int; 2] = [-1; 2];
  if (inrange_pipe (fds.as_mut_ptr ()) < 0)
    { report_file_error (c"Creating pipe for child signal".as_ptr (), Qnil ()); }

  /* We leave the file descriptors open until the Emacs process exits.  */
  eassert! (0 <= fds[0]);
  eassert! (0 <= fds[1]);
  if (fcntl (fds[0], F_SETFL, O_NONBLOCK) != 0)
    { emacs_perror (c"fcntl".as_ptr ()); }
  if (fcntl (fds[1], F_SETFL, O_NONBLOCK) != 0)
    { emacs_perror (c"fcntl".as_ptr ()); }
  add_read_fd (fds[0], Some (child_signal_read), std::ptr::null_mut ());
  fd_callback_info[fds[0] as usize].flags &= !KEYBOARD_FD;
  child_signal_read_fd = fds[0];
  child_signal_write_fd = fds[1];
}

/* `#ifndef WINDOWSNT` is live.  */
/* Consume a process status change.  */
/* C 7614-7626: `child_signal_read` — an `fd_callback` (PORTING.md 1.4).  */
unsafe extern "C" fn child_signal_read (fd: c_int, data: *mut c_void)
{
  let _ = data;
  eassert! (0 <= fd);
  eassert! (fd == child_signal_read_fd);
  let mut dummy: c_char = 0;
  if (emacs_read (fd, &raw mut dummy as *mut c_void, 1) < 0 && *errno () != EAGAIN)
    { emacs_perror (c"reading from child signal FD".as_ptr ()); }
}

/* Notify `wait_reading_process_output' of a process status change.  */
/* C 7628-7665: `child_signal_notify`.  */
unsafe fn child_signal_notify ()
{
  if ! getenv (c"FEMACS_CHILD_TRACE".as_ptr ()).is_null ()
    {
      fprintf (stderr, c"child-notify: write_fd=%d\n".as_ptr (), child_signal_write_fd);
    }
  /* `#ifndef WINDOWSNT` is live.  */
  let fd: c_int = child_signal_write_fd;
  eassert! (0 <= fd);
  let dummy: c_char = 0;
  /* We used to error out here ... But this calls `emacs_perror', which in turn invokes a
     localized version of strerror, which is not reentrant and must not be called within a
     signal handler.  So we no longer check errors of emacs_write here.  */
  emacs_write (fd, &raw const dummy as *const c_void, 1);
}

/* LIB_CHILD_HANDLER is a SIGCHLD handler that Emacs calls while doing its own SIGCHLD
   handling.  On POSIXish systems lacking pidfd_open+waitid or using Glib 2.73.1-, Glib
   needs this to keep track of its own children.  GNUstep is similar.  */

/* C 7667: `static void dummy_handler (int sig) {}`  */
unsafe extern "C" fn dummy_handler (sig: c_int) { let _ = sig; let _ = dummy_handler; }
/* C 7668: `static signal_handler_t volatile lib_child_handler;`  */
static mut lib_child_handler: signal_handler_t = None;

/* True if Glib installs its own SIGCHLD handler that Emacs must work around.  Determined
   once in init_process_emacs; consulted elsewhere (e.g. xwidget.c) to decide whether the
   about:blank load workaround is needed.  */
/* C 7674: `bool glib_installs_sigchld_handler;` — non-static, so this unit's exported
   object (the `process.h` mirror declares the same name).  */
#[unsafe(no_mangle)]
pub static mut glib_installs_sigchld_handler: bool = false;

/* Handle a SIGCHLD signal by looking for known child processes of Emacs whose status have
   changed.  For each one found, record its new status.  ...  */
/* C 7702-7787: `handle_child_signal` — a `signal_handler_t` (PORTING.md 1.4).  */
unsafe extern "C" fn handle_child_signal (sig: c_int)
{
  if ! getenv (c"FEMACS_CHILD_TRACE".as_ptr ()).is_null ()
    { fprintf (stderr, c"child-signal: sig=%d\n".as_ptr (), sig); }
  let mut tail: Lisp_Object;
  let mut proc_: Lisp_Object;
  let mut changed: bool = false;

  /* Find the process that signaled us, and record its status.  */

  /* The process can have been deleted by Fdelete_process, or have been started
     asynchronously by Fcall_process.  */
  tail = deleted_pid_list;
  while CONSP (tail)
    {
      let all_pids_are_fixnums: bool
	= MOST_NEGATIVE_FIXNUM <= PID_T_MIN
	  && PID_T_MAX <= MOST_POSITIVE_FIXNUM;
      let head: Lisp_Object = XCAR (tail);
      let xpid: Lisp_Object;
      if (! CONSP (head))
	{ tail = XCDR (tail); continue; }
      xpid = XCAR (head);
      if (if all_pids_are_fixnums { FIXNUMP (xpid) } else { INTEGERP (xpid) })
	{
	  let mut deleted_pid: i64 = 0;
	  let ok: bool = integer_to_intmax (xpid, &raw mut deleted_pid);
	  eassert! (ok);
	  if (child_status_changed (deleted_pid as pid_t, std::ptr::null_mut (), 0) != 0)
	    {
	      changed = true;
	      if (STRINGP (XCDR (head)))
		/* handle_child_signal is called in an async signal handler but needs to
		   unlink temporary files which might've been created in an Android content
		   provider.  ...  */
		{ unlink (SSDATA (XCDR (head)) as *const c_char); }
	      XSETCAR (tail, Qnil ());
	    }
	}
      tail = XCDR (tail);
    }

  /* Otherwise, if it is asynchronous, it is in Vprocess_alist.  */
  FOR_EACH_PROCESS! (tail, proc_, {
    let p: *mut Lisp_Process = XPROCESS (proc_);
    let mut status: c_int = 0;

    if (process_bits! (alive, p)
	&& child_status_changed ((*p).pid, &raw mut status, WUNTRACED | WCONTINUED) != 0)
      {
	/* Change the status of the process that was found.  */
	changed = true;
	/* PORTING.md P-122's instrument (FEMACS_CHILD_TRACE, P-115's shape): which child the
	   signal handler reaped, and how.  Off unless the variable is set.  */
	if ! getenv (c"FEMACS_CHILD_TRACE".as_ptr ()).is_null ()
	  {
	    fprintf (stderr, c"child-reap: pid=%d status=%d WIFEXITED=%d\n".as_ptr (),
		     (*p).pid as c_int, status, WIFEXITED (status) as c_int);
	  }
	process_tick += 1;
	(*p).tick = process_tick;
	(*p).raw_status = status;
	process_bits! (raw_status_new, p, 1);

	/* If process has terminated, stop waiting for its output.  */
	if (WIFSIGNALED (status) || WIFEXITED (status))
	  {
	    let mut clear_desc_flag: bool = false;
	    process_bits! (alive, p, 0);
	    if ((*p).infd >= 0)
	      { clear_desc_flag = true; }

	    /* clear_desc_flag avoids a compiler bug in Microsoft C.  */
	    if (clear_desc_flag)
	      { delete_read_fd ((*p).infd); }
	  }
      }
  });

  if (changed)
    /* Wake up `wait_reading_process_output'.  */
    { child_signal_notify (); }

  if let Some (handler) = lib_child_handler { handler (sig); }
  /* C 7781-7784's `#ifdef NS_IMPL_GNUSTEP` arm: NSTask in GNUstep sets its child handler
     each time it is called, so we must re-set ours.  */
  #[cfg (feature = "ns_impl_gnustep")]
  {
    catch_child_signal ();
  }
}

unsafe extern "C" {
  /// C: ptrdiff_t emacs_write (int, const void *, ptrdiff_t)   (lisp.h 5321, sysdep.c)
  fn emacs_write (fd: c_int, buf: *const c_void, nbyte: ptrdiff_t) -> ptrdiff_t;
  /// C: void emacs_perror (char const *)   (lisp.h 5337)
  fn emacs_perror (message: *const c_char);
}
/* C: `<sys/wait.h>`'s `WUNTRACED` and `WCONTINUED`, measured 2 and 16.  */
pub const WUNTRACED: c_int = 2;
pub const WCONTINUED: c_int = 16;
/* C: lisp.h 5194's `TYPE_MINIMUM (pid_t)` / `TYPE_MAXIMUM (pid_t)`, measured INT32_MIN/MAX.  */
const PID_T_MIN: EMACS_INT = i32::MIN as EMACS_INT;
const PID_T_MAX: EMACS_INT = i32::MAX as EMACS_INT;

/* C 7789-7792: `deliver_child_signal` — its address is taken by `emacs_sigaction_init`
   (PORTING.md 1.4).  */
unsafe extern "C" fn deliver_child_signal (sig: c_int)
{
  deliver_process_signal (sig, Some (handle_child_signal));
}

/* C 7796-7808: `exec_sentinel_error_handler` — its address is taken by
   `internal_condition_case_1` (PORTING.md 1.4).  */
unsafe extern "C" fn exec_sentinel_error_handler (error_val: Lisp_Object) -> Lisp_Object
{
  let mut error_val: Lisp_Object = error_val;
  /* Make sure error_val is a cons cell, as all the rest of error handling expects that,
     and will barf otherwise.  */
  if (!CONSP (error_val))
    { error_val = Fcons (Qerror (), error_val); }
  cmd_error_internal (error_val, c"error in process sentinel: ".as_ptr ());
  Vinhibit_quit = Qt ();
  update_echo_area ();
  if (process_error_pause_time > 0)
    { Fsleep_for (make_fixnum (process_error_pause_time), Qnil ()); }
  return Qt ();
}

/* C 7811-7882: `exec_sentinel`.  C's `#if 0` block (C 7823-7826) is dead in every
   configuration and is recorded, not carried (P-08).  */
unsafe fn exec_sentinel (proc_: Lisp_Object, reason: Lisp_Object)
{
  let sentinel: Lisp_Object;
  let mut odeactivate: Lisp_Object;
  let p: *mut Lisp_Process = XPROCESS (proc_);
  let count: specpdl_ref = SPECPDL_INDEX ();
  let outer_running_asynch_code: bool = running_asynch_code;
  let waiting: c_int = waiting_for_user_input_p! ();

  if (inhibit_sentinels)
    { return; }

  odeactivate = Vdeactivate_mark;

  /* There's no good reason to let sentinels change the current buffer, and many callers of
     accept-process-output, sit-for, and friends don't expect current-buffer to be changed
     from under them.  */
  record_unwind_current_buffer ();

  sentinel = (*p).sentinel;

  /* Inhibit quit so that random quits don't screw up a running filter.  */
  specbind (Qinhibit_quit (), Qt ());
  specbind (Qlast_nonmenu_event (), Qt ()); /* Why? --Stef  */

  /* In case we get recursively called, and we already saved the match data
     nonrecursively, save the same match data in safely recursive fashion.  */
  if (outer_running_asynch_code)
    {
      let mut tem: Lisp_Object;
      tem = Fmatch_data (Qnil (), Qnil (), Qnil ());
      restore_search_regs ();
      record_unwind_save_match_data ();
      Fset_match_data (tem, Qt ());
    }

  /* For speed, if a search happens within this code, save the match data in a special
     nonrecursive fashion.  */
  running_asynch_code = true;

  internal_condition_case_1 (Some (read_process_output_call),
			     list3 (sentinel, proc_, reason),
			     if !NILP (Vdebug_on_error) { Qnil () } else { Qerror () },
			     Some (exec_sentinel_error_handler));

  /* If we saved the match data nonrecursively, restore it now.  */
  restore_search_regs ();
  running_asynch_code = outer_running_asynch_code;

  Vdeactivate_mark = odeactivate;

  /* Restore waiting_for_user_input_p as it was when we were called, in case the filter
     clobbered it.  */
  waiting_for_user_input_p! () = waiting;

  unbind_to (count, Qnil ());
}

/* Report all recent events of a change in process status (either run the sentinel or
   output a message).  ...  Return positive if any input was received from WAIT_PROC (or
   from any process if WAIT_PROC is null), zero if input was attempted but none received,
   and negative if we didn't even try.  */
/* C 7884-7956: `status_notify`.  */
unsafe fn status_notify (deleting_process: *mut Lisp_Process,
			 wait_proc: *mut Lisp_Process) -> c_int
{
  let mut proc_: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut msg: Lisp_Object;
  let mut got_some_output: c_int = -1;

  tail = Qnil ();
  msg = Qnil ();

  /* Set this now, so that if new processes are created by sentinels that we run, we get
     called again to handle their status changes.  */
  update_tick = process_tick;

  FOR_EACH_PROCESS! (tail, proc_, {
    let mut symbol: Lisp_Object;
    let p: *mut Lisp_Process = XPROCESS (proc_);

    if ((*p).tick != (*p).update_tick)
      {
	(*p).update_tick = (*p).tick;

	/* If process is still active, read any output that remains.  */
	while (! EQ ((*p).filter, Qt ())
	       && ! connecting_status ((*p).status)
	       && ! EQ ((*p).status, Qlisten ())
	       /* Network or serial process not stopped:  */
	       && ! EQ ((*p).command, Qt ())
	       && (*p).infd >= 0
	       && p != deleting_process)
	  {
	    let nread: c_int = read_process_output (proc_, (*p).infd);
	    if ((wait_proc.is_null () || wait_proc == XPROCESS (proc_))
		&& got_some_output < nread)
	      { got_some_output = nread; }
	    if (nread <= 0)
	      { break; }
	  }

	/* Get the text to use for the message.  */
	if (process_bits! (raw_status_new, p))
	  { update_status (p); }
	msg = status_message (p);

	/* If process is terminated, deactivate it or delete it.  */
	symbol = (*p).status;
	if (CONSP ((*p).status))
	  { symbol = XCAR ((*p).status); }

	if (EQ (symbol, Qsignal ()) || EQ (symbol, Qexit ())
	    || EQ (symbol, Qclosed ()))
	  {
	    if (delete_exited_processes)
	      { remove_process (proc_); }
	    else
	      { deactivate_process (proc_); }
	  }

	/* The actions above may have further incremented p->tick.  So set p->update_tick
	   again so that an error in the sentinel will not cause this code to be run
	   again.  */
	(*p).update_tick = (*p).tick;
	/* Now output the message suitably.  */
	exec_sentinel (proc_, msg);
	if (BUFFERP ((*p).buffer))
	  /* In case it uses %s in mode-line-format.  */
	  { bset_update_mode_line (XBUFFER ((*p).buffer) as *mut c_void); }
      }
  }); /* end for */

  return got_some_output;
}

/* The `DEFVAR_BOOL` carrier this file owns (C 9xxx, `delete-exited-processes`; status 3.29).  */
#[unsafe(no_mangle)]
pub static mut delete_exited_processes: bool = false;

DEFUN! ("internal-default-process-sentinel", Finternal_default_process_sentinel,
       Sinternal_default_process_sentinel, 2, 2, 0,
       doc: "Function used as default sentinel for processes.\nThis inserts a status message into the process's buffer, if there is one.",
       (proc_: Lisp_Object, msg: Lisp_Object),
       {
  let mut buffer: Lisp_Object;
  let mut symbol: Lisp_Object;
  let p: *mut Lisp_Process;
  let mut msg: Lisp_Object = msg;
  CHECK_PROCESS (proc_);
  p = XPROCESS (proc_);
  buffer = (*p).buffer;
  symbol = (*p).status;
  if (CONSP (symbol))
    { symbol = XCAR (symbol); }

  if (!EQ (symbol, Qrun ()) && !NILP (buffer))
    {
      let mut tem: Lisp_Object;
      let old: *mut buffer = (current_buffer! ());
      let mut opoint: ptrdiff_t;
      let mut opoint_byte: ptrdiff_t;
      let mut before: ptrdiff_t;
      let mut before_byte: ptrdiff_t;

      /* Avoid error if buffer is deleted (probably that's why the process is dead,
	 too).  */
      if (!BUFFER_LIVE_P (XBUFFER (buffer)))
	{ return Qnil (); }
      Fset_buffer (buffer);

      if (NILP (BVAR! ((current_buffer! ()), enable_multibyte_characters_)))
	{ msg = code_convert_string_norecord (msg, Vlocale_coding_system, true); }

      opoint = PT! ();
      opoint_byte = PT_BYTE! ();
      /* Insert new output into buffer at the current end-of-output marker, thus
	 preserving logical ordering of input and output.  */
      if ((*XMARKER ((*p).mark)).buffer != std::ptr::null_mut ())
	{ Fgoto_char ((*p).mark); }
      else
	{ SET_PT_BOTH (ZV! (), ZV_BYTE! ()); }

      before = PT! ();
      before_byte = PT_BYTE! ();

      tem = BVAR! ((current_buffer! ()), read_only_);
      bset_read_only ((current_buffer! ()), Qnil ());
      insert_string (c"\nProcess ".as_ptr ());
      { /* FIXME: temporary kludge.  */
	let mut tem2: Lisp_Object = (*p).name; Finsert (1, &raw mut tem2); }
      insert_string (c" ".as_ptr ());
      Finsert (1, &raw mut msg);
      bset_read_only ((current_buffer! ()), tem);
      set_marker_both ((*p).mark, (*p).buffer, PT! (), PT_BYTE! ());

      if (opoint >= before)
	{ SET_PT_BOTH (opoint + (PT! () - before),
		       opoint_byte + (PT_BYTE! () - before_byte)); }
      else
	{ SET_PT_BOTH (opoint, opoint_byte); }

      set_buffer_internal (old);
    }
  return Qnil ();
});

DEFUN! ("set-process-coding-system", Fset_process_coding_system,
       Sset_process_coding_system, 1, 3, 0,
       doc: "Set coding systems of PROCESS to DECODING and ENCODING.\nDECODING will be used to decode subprocess output and ENCODING to\nencode subprocess input.",
       (process: Lisp_Object, decoding: Lisp_Object, encoding: Lisp_Object),
       {
  CHECK_PROCESS (process);

  let p: *mut Lisp_Process = XPROCESS (process);
  let mut encoding: Lisp_Object = encoding;

  /* C discards the two subrs' return values.  */
  let _ = Fcheck_coding_system (decoding);
  let _ = Fcheck_coding_system (encoding);
  encoding = coding_inherit_eol_type (encoding, Qnil ());
  pset_decode_coding_system (p, decoding);
  pset_encode_coding_system (p, encoding);

  /* If the sockets haven't been set up yet, the final setup part of this will be called
     asynchronously.  */
  if ((*p).infd < 0 || (*p).outfd < 0)
    { return Qnil (); }

  setup_process_coding_systems (process);

  return Qnil ();
});

DEFUN! ("process-coding-system", Fprocess_coding_system, Sprocess_coding_system, 1, 1, 0,
       doc: "Return a cons of coding systems for decoding and encoding of PROCESS.",
       (process: Lisp_Object),
       {
  CHECK_PROCESS (process);
  return Fcons ((*XPROCESS (process)).decode_coding_system,
		(*XPROCESS (process)).encode_coding_system);
});

/* `#ifdef HAVE_GPM` is dead on this target (HAVE_GPM is `undef`): the two functions are
   carried text (feature `have_gpm`).  */
#[cfg (feature = "have_gpm")]
#[unsafe(no_mangle)]
pub unsafe fn add_gpm_wait_descriptor (desc: c_int)
{
  add_keyboard_wait_descriptor (desc);
}
#[cfg (feature = "have_gpm")]
#[unsafe(no_mangle)]
pub unsafe fn delete_gpm_wait_descriptor (desc: c_int)
{
  delete_keyboard_wait_descriptor (desc);
}

/* `#if defined (USABLE_SIGIO) || defined (USABLE_SIGPOLL)` is live.  */
/* Return true if *MASK has a bit set that corresponds to one of the keyboard input
   descriptors.  */
/* C 8087-8100: `keyboard_bit_set`.  */
unsafe fn keyboard_bit_set (mask: *mut fd_set) -> bool
{
  let mut fd: c_int;

  eassert! (max_desc < FD_SETSIZE);
  fd = 0;
  while fd <= max_desc
    {
      if (FD_ISSET (fd, mask)
	  && ((fd_callback_info[fd as usize].flags & (FOR_READ | KEYBOARD_FD))
	      == (FOR_READ | KEYBOARD_FD)))
	{ return true; }
      fd += 1;
    }

  return false;
}

/* `bset_update_mode_line` (window.h 1175; buffer.h 1757 declares it too) is the window.rs
   mirror's now - the mirror spells `struct buffer *` as `*mut c_void` (P-72 rule 3), so the
   caller below casts (P-26).  */

/* C 8102-8325: the `#else /* not subprocesses */` arm of the `#ifdef subprocesses` that opens
   at C 163 — the MS-DOS/no-subprocess implementations.  The whole region is carried text
   under `no_subprocesses` (P-51): its `update_processes_for_thread_death` and
   `wait_reading_process_output` are the *second* definitions of those names, and the
   reference-object rule (docs/status.md 3.67) plus the checker's first-body rule keep the
   live definitions above authoritative.  */
#[cfg (feature = "no_subprocesses")]
pub mod no_subprocesses
{
  use super::*;

  /* This is referenced in thread.c:run_thread (which is never actually called, since
     threads are not enabled for this configuration.  */
  #[unsafe(no_mangle)]
  pub unsafe fn update_processes_for_thread_death (dying_thread: Lisp_Object)
  {
    let _ = dying_thread;
  }

  /* Defined in msdos.c.  */
  unsafe extern "C" {
    fn sys_select (nfds: c_int, readfds: *mut fd_set, writefds: *mut fd_set,
		   exceptfds: *mut fd_set, timeout: *mut timespec,
		   sigmask: *mut c_void) -> c_int;
  }

  /// Implementation of wait_reading_process_output, assuming that there are no
  /// subprocesses.  Used only by the MS-DOS build.  ...
  #[unsafe(no_mangle)]
  pub unsafe fn wait_reading_process_output (time_limit: i64, nsecs: c_int, read_kbd: c_int,
					     do_display: bool, wait_for_cell: Lisp_Object,
					     wait_proc: *mut Lisp_Process,
					     just_wait_proc: c_int) -> c_int
  {
    let mut nfds: c_int;
    let mut end_time: timespec = std::mem::zeroed ();
    let mut timeout: timespec;
    let mut wait: c_int;
    let mut time_limit: i64 = time_limit;

    if (TYPE_MAXIMUM_TIME_T < time_limit)
      { time_limit = TYPE_MAXIMUM_TIME_T; }

    if (time_limit < 0 || nsecs < 0)
      { wait = MINIMUM; }
    else if (time_limit > 0 || nsecs > 0)
      {
	wait = TIMEOUT;
	end_time = timespec_add (monotonic_coarse_timespec (),
				 make_timespec (time_limit as time_t, nsecs as c_long));
      }
    else
      { wait = FOREVER; }

    /* Turn off periodic alarms (in case they are in use) and then turn off any other
       atimers, because the select emulator uses alarms.  */
    stop_polling ();
    turn_on_atimers (false);

    loop
      {
	let mut timeout_reduced_for_timers: bool = false;
	let mut waitchannels: fd_set = std::mem::zeroed ();
	let mut xerrno: c_int;

	/* If calling from keyboard input, do not quit since we want to return C-g as an
	   input character.  Otherwise, do pending quit if requested.  */
	if (read_kbd >= 0)
	  { maybe_quit (); }

	/* Exit now if the cell we're waiting for became non-nil.  */
	if (! NILP (wait_for_cell) && ! NILP (XCAR (wait_for_cell)))
	  { break; }

	/* Compute time from now till when time limit is up.  */
	/* Exit if already run out.  */
	if (wait == TIMEOUT)
	  {
	    let now: timespec = monotonic_coarse_timespec ();
	    if (timespec_cmp (end_time, now) <= 0)
	      { break; }
	    timeout = timespec_sub (end_time, now);
	  }
	else
	  { timeout = make_timespec (if wait < TIMEOUT { 0 } else { 100000 }, 0); }

	/* If our caller will not immediately handle keyboard events, run timer events
	   directly.  (Callers that will immediately read keyboard events call timer_delay
	   on their own.)  */
	if (NILP (wait_for_cell))
	  {
	    let mut timer_delay: timespec;

	    loop
	      {
		let old_timers_run: c_uint = timers_run;
		timer_delay = timer_check ();
		if (timers_run != old_timers_run && do_display)
		  /* We must retry, since a timer may have requeued itself and that
		     could alter the time delay.  */
		  { redisplay_preserve_echo_area (14); }
		else
		  { break; }
		if detect_input_pending () { break; }
	      }

	    /* If there is unread keyboard input, also return.  */
	    if (read_kbd != 0
		&& requeued_command_events_pending_p ())
	      { break; }

	    if (timespec_valid_p (timer_delay))
	      {
		if (timespec_cmp (timer_delay, timeout) < 0)
		  {
		    timeout = timer_delay;
		    timeout_reduced_for_timers = true;
		  }
	      }
	  }

	/* Cause C-g and alarm signals to take immediate action, and cause input
	   available signals to zero out timeout.  */
	if (read_kbd < 0)
	  { set_waiting_for_input (&raw mut timeout); }

	/* If a frame has been newly mapped and needs updating, reprocess its display
	   stuff.  */
	if (frame_garbaged && do_display)
	  {
	    clear_waiting_for_input ();
	    redisplay_preserve_echo_area (15);
	    if (read_kbd < 0)
	      { set_waiting_for_input (&raw mut timeout); }
	  }

	/* Wait till there is something to do.  */
	FD_ZERO (&raw mut waitchannels);
	if (read_kbd != 0 && detect_input_pending ())
	  { nfds = 0; }
	else
	  {
	    if (read_kbd != 0 || !NILP (wait_for_cell))
	      { FD_SET (0, &raw mut waitchannels); }
	    nfds = pselect (1, &raw mut waitchannels, std::ptr::null_mut (),
			    std::ptr::null_mut (), &raw mut timeout, std::ptr::null ());
	  }

	xerrno = *errno ();

	/* Make C-g and alarm signals set flags again.  */
	clear_waiting_for_input ();

	/*  If we woke up due to SIGWINCH, actually change size now.  */
	do_pending_window_change (false);

	if (wait < FOREVER && nfds == 0 && ! timeout_reduced_for_timers)
	  /* We waited the full specified time, so return now.  */
	  { break; }

	if (nfds == -1)
	  {
	    /* If the system call was interrupted, then go around the loop again.  */
	    if (xerrno == EINTR)
	      { FD_ZERO (&raw mut waitchannels); }
	    else
	      { report_file_errno (c"Failed select".as_ptr (), Qnil (), xerrno); }
	  }

	/* Check for keyboard input.  */

	if (read_kbd != 0
	    && detect_input_pending_run_timers (do_display))
	  {
	    swallow_events (do_display);
	    if (detect_input_pending_run_timers (do_display))
	      { break; }
	  }

	/* If there is unread keyboard input, also return.  */
	if (read_kbd != 0
	    && requeued_command_events_pending_p ())
	  { break; }

	/* If wait_for_cell. check for keyboard input but don't run any timers.  ...  */
	if (! NILP (wait_for_cell)
	    && detect_input_pending ())
	  {
	    swallow_events (do_display);
	    if (detect_input_pending ())
	      { break; }
	  }

	/* Exit now if the cell we're waiting for became non-nil.  */
	if (! NILP (wait_for_cell) && ! NILP (XCAR (wait_for_cell)))
	  { break; }
      }

    start_polling ();

    return -1;
  }
}

/* The following functions are needed even if async subprocesses are not supported.  Some
   of them are no-op stubs in that case.  */

/* `#ifdef HAVE_TIMERFD` is dead on this target (HAVE_TIMERFD is `undef`): the function is
   carried text (feature `have_timerfd`).  */
#[cfg (feature = "have_timerfd")]
#[unsafe(no_mangle)]
pub unsafe fn add_timer_wait_descriptor (fd: c_int)
{
  eassert! (0 <= fd && fd < FD_SETSIZE);
  add_read_fd (fd, Some (timerfd_callback), std::ptr::null_mut ());
  fd_callback_info[fd as usize].flags &= !KEYBOARD_FD;
}

/* If program file NAME starts with /: for quoting a magic name, remove that, preserving the
   multibyteness of NAME.  */
/* C 8350-8358: `remove_slash_colon`.  */
#[unsafe(no_mangle)]
pub unsafe fn remove_slash_colon (name: Lisp_Object) -> Lisp_Object
{
  return
    if SREF (name, 0) == b'/' as u8 && SREF (name, 1) == b':' as u8
    { make_specified_string ((SSDATA (name) as *const c_char).offset (2),
			     SCHARS (name) - 2,
			     SBYTES (name) - 2, STRING_MULTIBYTE (name)) }
    else { name };
}

/* Add DESC to the set of keyboard input descriptors.  */
/* C 8362-8372: `add_keyboard_wait_descriptor` (non-static: it is this unit's exported
   function, declared by keyboard.h).  */
#[unsafe(no_mangle)]
pub unsafe fn add_keyboard_wait_descriptor (desc: c_int)
{
  /* `#ifdef subprocesses /* Actually means "not MSDOS".  */` is live.  */
  eassert! (desc >= 0 && desc < FD_SETSIZE);
  fd_callback_info[desc as usize].flags &= !PROCESS_FD;
  fd_callback_info[desc as usize].flags |= FOR_READ | KEYBOARD_FD;
  if (desc > max_desc)
    { max_desc = desc; }
}

/* From now on, do not expect DESC to give keyboard input.  */
/* C 8376-8385: `delete_keyboard_wait_descriptor`.  */
#[unsafe(no_mangle)]
pub unsafe fn delete_keyboard_wait_descriptor (desc: c_int)
{
  /* `#ifdef subprocesses` is live.  */
  eassert! (desc >= 0 && desc < FD_SETSIZE);

  clear_fd_callback_data (&raw mut fd_callback_info[desc as usize]);

  if (desc == max_desc)
    { recompute_max_desc (); }
}

/* Setup coding systems of PROCESS.  */
/* C 8391-8418: `setup_process_coding_systems`.  */
#[unsafe(no_mangle)]
pub unsafe fn setup_process_coding_systems (process: Lisp_Object)
{
  /* `#ifdef subprocesses` is live.  */
  let p: *mut Lisp_Process = XPROCESS (process);
  let inch: c_int = (*p).infd;
  let outch: c_int = (*p).outfd;
  let mut coding_system: Lisp_Object;

  if (inch < 0 || outch < 0)
    { return; }

  eassert! (0 <= inch && inch < FD_SETSIZE);
  if (proc_decode_coding_system[inch as usize].is_null ())
    { proc_decode_coding_system[inch as usize]
	= xmalloc (std::mem::size_of::<coding_system> ()) as *mut coding_system; }
  coding_system = (*p).decode_coding_system;
  if (EQ ((*p).filter, Qinternal_default_process_filter ())
      && BUFFERP ((*p).buffer))
    {
      if (NILP (BVAR! (XBUFFER ((*p).buffer), enable_multibyte_characters_)))
	{ coding_system = raw_text_coding_system (coding_system); }
    }
  setup_coding_system (coding_system, proc_decode_coding_system[inch as usize]);

  eassert! (0 <= outch && outch < FD_SETSIZE);
  if (proc_encode_coding_system[outch as usize].is_null ())
    { proc_encode_coding_system[outch as usize]
	= xmalloc (std::mem::size_of::<coding_system> ()) as *mut coding_system; }
  setup_coding_system ((*p).encode_coding_system,
		       proc_encode_coding_system[outch as usize]);
}

DEFUN! ("get-buffer-process", Fget_buffer_process, Sget_buffer_process, 1, 1, 0,
       doc: "Return the (or a) live process associated with BUFFER.\nBUFFER may be a buffer or the name of one.\nReturn nil if all processes associated with BUFFER have been\ndeleted or killed.",
       (buffer: Lisp_Object),
       {
  /* `#ifdef subprocesses` is live.  */
  let mut buf: Lisp_Object;
  let mut tail: Lisp_Object;
  let mut proc_: Lisp_Object;

  if (NILP (buffer)) { return Qnil (); }
  buf = Fget_buffer (buffer);
  if (NILP (buf)) { return Qnil (); }

  FOR_EACH_PROCESS! (tail, proc_, {
    if (EQ ((*XPROCESS (proc_)).buffer, buf))
      { return proc_; }
  });
  return Qnil ();
});

unsafe extern "C" {
  /// C: Lisp_Object make_specified_string (const char *, ptrdiff_t, ptrdiff_t, bool)
  ///    (lisp.h 4115) — the lisp.rs mirror still lacks it (reported to the integrator)
  fn make_specified_string (contents: *const c_char, nchars: ptrdiff_t, nbytes: ptrdiff_t,
			    multibyte: bool) -> Lisp_Object;
  /// C: void setup_coding_system (Lisp_Object, struct coding_system *)   (coding.h 686)
  fn setup_coding_system (coding_system: Lisp_Object, coding: *mut coding_system);
  /// C: void stop_polling (void)   (keyboard.c, the no-subprocesses arm)
  fn stop_polling ();
  /// C: void start_polling (void)   (keyboard.c, the no-subprocesses arm)
  fn start_polling ();
}

DEFUN! ("process-inherit-coding-system-flag", Fprocess_inherit_coding_system_flag,
       Sprocess_inherit_coding_system_flag, 1, 1, 0,
       doc: "Return the value of inherit-coding-system flag for PROCESS.\nIf this flag is t, `buffer-file-coding-system' of the buffer\nassociated with PROCESS will inherit the coding system used to decode\nthe process output.",
       (process: Lisp_Object),
       {
  /* `#ifdef subprocesses` is live.  */
  #[cfg (not (feature = "no_subprocesses"))]
  {
    CHECK_PROCESS (process);
    return if process_bits! (inherit_coding_system_flag, XPROCESS (process))
      { Qt () } else { Qnil () };
  }
  /* C 8452-8455's `#else` arm: ignore the argument and return the value of
     inherit-process-coding-system.  */
  #[cfg (feature = "no_subprocesses")]
  {
    let _ = process;
    return if inherit_process_coding_system { Qt () } else { Qnil () };
  }
});

/* Kill all processes associated with `buffer'.  If `buffer' is nil, kill all
   processes.  */
/* C 8466-8482: `kill_buffer_processes`.  */
#[unsafe(no_mangle)]
pub unsafe fn kill_buffer_processes (buffer: Lisp_Object)
{
  /* `#ifdef subprocesses` is live.  */
  let mut tail: Lisp_Object;
  let mut proc_: Lisp_Object;

  FOR_EACH_PROCESS! (tail, proc_, {
    if (NILP (buffer) || EQ ((*XPROCESS (proc_)).buffer, buffer))
      {
	if (NETCONN_P (proc_) || SERIALCONN_P (proc_) || PIPECONN_P (proc_))
	  { Fdelete_process (proc_); }
	else if ((*XPROCESS (proc_)).infd >= 0)
	  { process_send_signal (proc_, SIGHUP, Qnil (), true); }
      }
  });
}

DEFUN! ("waiting-for-user-input-p", Fwaiting_for_user_input_p,
       Swaiting_for_user_input_p, 0, 0, 0,
       doc: "Return non-nil if Emacs is waiting for input from the user.\nThis is intended for use by asynchronous process output filters and sentinels.",
       (),
       {
  /* `#ifdef subprocesses` is live.  */
  return if waiting_for_user_input_p! () != 0 { Qt () } else { Qnil () };
});

/* Stop reading input from keyboard sources.  */
/* C 8500-8504: `hold_keyboard_input`.  */
#[unsafe(no_mangle)]
pub unsafe fn hold_keyboard_input ()
{
  kbd_is_on_hold = true;
}

/* Resume reading input from keyboard sources.  */
/* C 8508-8512: `unhold_keyboard_input`.  */
#[unsafe(no_mangle)]
pub unsafe fn unhold_keyboard_input ()
{
  kbd_is_on_hold = false;
}

/* Return true if keyboard input is on hold, zero otherwise.  */
/* C 8516-8520: `kbd_on_hold_p`.  */
#[unsafe(no_mangle)]
pub unsafe fn kbd_on_hold_p () -> bool
{
  return kbd_is_on_hold;
}

DEFUN! ("list-system-processes", Flist_system_processes, Slist_system_processes, 0, 0, 0,
       doc: "Return a list of numerical process IDs of all running processes.\nIf this functionality is unsupported, return nil.\nIf `default-directory' is remote, return process IDs of the respective remote host.\n\nSee `process-attributes' for getting attributes of a process given its ID.",
       (),
       {
  let handler: Lisp_Object
    = Ffind_file_name_handler (BVAR! ((current_buffer! ()), directory_),
			       Qlist_system_processes ());
  if (!NILP (handler))
    { return calln! (handler, Qlist_system_processes ()); }

  return list_system_processes ();
});

DEFUN! ("process-attributes", Fprocess_attributes, Sprocess_attributes, 1, 1, 0,
       doc: "Return attributes of the process given by its PID, a number.\nIf `default-directory' is remote, PID is regarded as process\nidentifier on the respective remote host.\n\nValue is an alist where each element is a cons cell of the form\n\n    (KEY . VALUE)\n\nIf this functionality is unsupported, the value is nil.\n\nSee `list-system-processes' for getting a list of all process IDs.\n\nThe KEYs of the attributes that this function may return are listed\nbelow, together with the type of the associated VALUE (in parentheses).\nNot all platforms support all of these attributes; unsupported\nattributes will not appear in the returned alist.\nUnless explicitly indicated otherwise, numbers can have either\ninteger or floating point values.\n\n euid    -- Effective user User ID of the process (number)\n user    -- User name corresponding to euid (string)\n egid    -- Effective user Group ID of the process (number)\n group   -- Group name corresponding to egid (string)\n comm    -- Command name (executable name only) (string)\n state   -- Process state code, such as \"S\", \"R\", or \"T\" (string)\n ppid    -- Parent process ID (number)\n pgrp    -- Process group ID (number)\n sess    -- Session ID, i.e. process ID of session leader (number)\n ttname  -- Controlling tty name (string)\n tpgid   -- ID of foreground process group on the process's tty (number)\n minflt  -- number of minor page faults (number)\n majflt  -- number of major page faults (number)\n cminflt -- cumulative number of minor page faults (number)\n cmajflt -- cumulative number of major page faults (number)\n utime   -- total user time used by the process since its start,\n              in `current-time' format\n stime   -- total system time used by the process since its start\n              (current-time)\n time    -- sum of utime and stime (current-time)\n cutime  -- total user time used by the process and its children (current-time)\n cstime  -- total system time used by the process and its children\n              (current-time)\n ctime   -- sum of cutime and cstime (current-time)\n pri     -- priority of the process (number)\n nice    -- nice value of the process (number)\n thcount -- process thread count (number)\n start   -- time the process started (current-time)\n vsize   -- virtual memory size of the process in KB's (number)\n rss     -- resident set size of the process in KB's (number)\n etime   -- elapsed time the process is running (current-time)\n pcpu    -- percents of total CPU time used by the process since its start\n              (floating-point number)\n pmem    -- percents of total physical memory used by process's resident set\n              (floating-point number)\n args    -- command line which invoked the process (string).",
       (pid: Lisp_Object),
       {
  let handler: Lisp_Object
    = Ffind_file_name_handler (BVAR! ((current_buffer! ()), directory_),
			       Qprocess_attributes ());
  if (!NILP (handler))
    { return calln! (handler, Qprocess_attributes (), pid); }

  return system_process_attributes (pid);
});

DEFUN! ("num-processors", Fnum_processors, Snum_processors, 0, 1, 0,
       doc: "Return the number of processors, a positive integer.\nEach usable thread execution unit counts as a processor.\nBy default, count the number of available processors,\noverridable via the OMP_NUM_THREADS environment variable.\nIf optional argument QUERY is `current', ignore OMP_NUM_THREADS.\nIf QUERY is `all', also count processors not available.",
       (query: Lisp_Object),
       {
  /* C 8623-8625's `#else` arm of `#ifndef MSDOS`.  */
  #[cfg (feature = "msdos")]
  {
    return make_fixnum (1);
  }
  /* `#ifndef MSDOS` is live.  */
  #[cfg (not (feature = "msdos"))]
  return make_uint (num_processors (if EQ (query, Qall ()) { nproc_query::NPROC_ALL }
				    else if EQ (query, Qcurrent ()) { nproc_query::NPROC_CURRENT }
				    else { nproc_query::NPROC_CURRENT_OVERRIDABLE })
		    as uintmax_t);
});

DEFUN! ("signal-names", Fsignal_names, Ssignal_names, 0, 0, 0,
       doc: "Return a list of known signal names on this system.",
       (),
       {
  /* `#ifndef MSDOS` is live.  */
  let mut i: c_int;
  let mut name: [c_char; SIG2STR_MAX] = [0; SIG2STR_MAX];
  let mut names: Lisp_Object = Qnil ();

  i = 0;
  while i <= SIGNUM_BOUND
    {
      if (sig2str (i, name.as_mut_ptr ()) == 0)
	{ names = Fcons (build_string (name.as_ptr ()), names); }
      i += 1;
    }

  return names;
});

/* `#ifdef subprocesses` is live.  */
/* Arrange to catch SIGCHLD if this hasn't already been arranged.  ...  */
/* C 8656-8676: `catch_child_signal`.  */
#[unsafe(no_mangle)]
pub unsafe fn catch_child_signal ()
{
  if ! getenv (c"FEMACS_CHILD_TRACE".as_ptr ()).is_null ()
    { fprintf (stderr, c"child-catch: arming\n".as_ptr ()); }
  let mut action: sigaction = std::mem::zeroed ();
  let mut old_action: sigaction = std::mem::zeroed ();
  let mut oldset: sigset_t = std::mem::zeroed ();
  emacs_sigaction_init (&raw mut action, Some (deliver_child_signal));
  block_child_signal (&raw mut oldset);
  sigaction (SIGCHLD, &raw const action, &raw mut old_action);
  /* C: `old_action.sa_handler` is the `sigaction` union's `__sa_handler` member (P-27's
     `Option` shape); `<signal.h>`'s `SIG_DFL` is the zero handler and `SIG_IGN` the one.  */
  let sig_ign: signal_handler_t = unsafe { std::mem::transmute (SIG_IGN as usize) };
  eassert! (old_action.__sigaction_u.__sa_handler == SIG_DFL
	    || old_action.__sigaction_u.__sa_handler == sig_ign
	    || (old_action.sa_flags & SA_SIGINFO) == 0);

  if (old_action.__sigaction_u.__sa_handler != Some (deliver_child_signal))
    {
      lib_child_handler
	= if (old_action.__sigaction_u.__sa_handler == SIG_DFL
	      || old_action.__sigaction_u.__sa_handler == sig_ign)
	{ Some (dummy_handler) }
	else { old_action.__sigaction_u.__sa_handler };
    }
  unblock_child_signal (&raw const oldset);
}

/* Limit the number of open files to the value it had at startup.  */
/* C 8678-8685: `restore_nofile_limit`.  */
#[unsafe(no_mangle)]
pub unsafe fn restore_nofile_limit ()
{
  /* `#ifdef HAVE_SETRLIMIT` is live.  */
  if ((FD_SETSIZE as rlim_t) < nofile_limit.rlim_cur)
    { setrlimit (RLIMIT_NOFILE, &raw const nofile_limit); }
}

/* C 8687-8700: `open_channel_for_module`.  */
#[unsafe(no_mangle)]
pub unsafe fn open_channel_for_module (process: Lisp_Object) -> c_int
{
  CHECK_PROCESS (process);
  CHECK_TYPE (PIPECONN_P (process), Qpipe_process_p (), process);
  /* `#ifndef MSDOS` is live.  */
  let fd: c_int = dup ((*XPROCESS (process)).open_fd[SUBPROCESS_STDOUT as usize]);
  if (fd == -1)
    { report_file_error (c"Cannot duplicate file descriptor".as_ptr (), Qnil ()); }
  return fd;
  /* C 8696-8698's `#else` arm of `#ifndef MSDOS`: PIPECONN_P returning true shouldn't be
     possible on MSDOS.  */
  #[cfg (feature = "msdos")]
  {
    emacs_abort ();
  }
}

/* This is not called "init_process" because that is the name of a Mach system call, so it
   would cause problems on Darwin systems.  */
/* C 8706-8797: `init_process_emacs`.  C's `#if 0` block (C 8791-8792) is dead in every
   configuration and is recorded, not carried (P-08).  */
#[unsafe(no_mangle)]
pub unsafe fn init_process_emacs (sockfd: c_int)
{
  /* `#ifdef subprocesses` is live.  */
  let mut i: c_int;

  inhibit_sentinels = false;

  /* C 8713-8743's `#if defined HAVE_GLIB && !defined WINDOWSNT` arm: tickle Glib's
     child-handling code so `catch_child_signal` can copy its handler.  */
  #[cfg (feature = "have_glib")]
  {
    /* Tickle Glib's child-handling code.  Ask Glib to install a watch source for Emacs
       itself which will initialize glib's private SIGCHLD handler, allowing
       catch_child_signal to copy it into lib_child_handler.  This is a hacky workaround to
       get glib's g_unix_signal_handler into lib_child_handler.  */
    let source: *mut GSource = g_child_watch_source_new (getpid ());
    catch_child_signal ();
    g_source_unref (source);

    if (lib_child_handler != Some (dummy_handler))
      {
	/* The hacky workaround is needed on this platform.  */
	glib_installs_sigchld_handler = true;
	let lib_child_handler_glib: signal_handler_t = lib_child_handler;
	catch_child_signal ();
	eassert! (lib_child_handler == Some (dummy_handler));
	lib_child_handler = lib_child_handler_glib;
      }
  }
  #[cfg (not (feature = "have_glib"))]
  catch_child_signal ();

  /* `#ifdef HAVE_SETRLIMIT` is live.  */
  /* Don't allocate more than FD_SETSIZE file descriptors for Emacs itself.  ...  */
  if (getrlimit (RLIMIT_NOFILE, &raw mut nofile_limit) != 0)
    { nofile_limit.rlim_cur = 0; }
  else if ((FD_SETSIZE as rlim_t) < nofile_limit.rlim_cur)
    {
      let mut rlim: rlimit = nofile_limit;
      rlim.rlim_cur = FD_SETSIZE as rlim_t;
      if (setrlimit (RLIMIT_NOFILE, &raw const rlim) != 0)
	{ nofile_limit.rlim_cur = 0; }
    }

  external_sock_fd = sockfd;
  let mut sockname: Lisp_Object = Qnil ();
  /* `# if HAVE_GETSOCKNAME` is live.  */
  if (0 <= sockfd)
    {
      let mut sa: u_sockaddr = std::mem::zeroed ();
      let mut salen: socklen_t = std::mem::size_of_val (&sa) as socklen_t;
      if (getsockname (sockfd, &raw mut sa.sa, &raw mut salen) == 0)
	{ sockname = conv_sockaddr_to_lisp (&raw mut sa.sa, salen as ptrdiff_t); }
    }
  Vinternal__daemon_sockname = sockname;

  max_desc = -1;
  memset (fd_callback_info.as_mut_ptr () as *mut c_void, 0,
	  std::mem::size_of_val (&fd_callback_info));

  num_pending_connects = 0;

  process_output_delay_count = 0;
  process_output_skip = false;

  /* Don't do this, it caused infinite select loops.  The display method should call
     add_keyboard_wait_descriptor on stdin if it needs that.  C's `#if 0` block is dead
     (P-08).  */

  Vprocess_alist = Qnil ();
  deleted_pid_list = Qnil ();
  i = 0;
  while i < FD_SETSIZE
    {
      chan_process[i as usize] = Qnil ();
      proc_buffered_char[i as usize] = -1;
      i += 1;
    }
  memset (proc_decode_coding_system.as_mut_ptr () as *mut c_void, 0,
	  std::mem::size_of_val (&proc_decode_coding_system));
  memset (proc_encode_coding_system.as_mut_ptr () as *mut c_void, 0,
	  std::mem::size_of_val (&proc_encode_coding_system));
  /* `#ifdef DATAGRAM_SOCKETS` is live.  */
  memset (datagram_address.as_mut_ptr () as *mut c_void, 0,
	  std::mem::size_of_val (&datagram_address));

  kbd_is_on_hold = false;
}

/* The one `DEFVAR_*` carrier this file owns (status 3.29: C's storage is `globals.f_<name>`,
   so the unit gives the object its own storage — `syms_of_process` registers
   "internal-daemon-sockname" with `DEFVAR_LISP`).  `init_process_emacs` sets it at C 8795
   (`Vinternal__daemon_sockname = sockname;`).  **`inherit_process_coding_system` is *not*
   this file's**: C's `DEFVAR_BOOL ("inherit-process-coding-system", ...)` is in
   `src/coding.c` (C 12023), so the port must not define the object here — the live
   reference at C 2068 (`!(NILP (buffer) || !inherit_process_coding_system)`) stays an
   undefined reference (resolved by the `globals` mirror's declaration) until
   `src/coding.c`'s unit lands and defines its carrier.  `tools/check-symbols.py` reports an
   object exported from here as *invented*, because the reference object defines no such
   symbol.  */
static mut Vinternal__daemon_sockname: Lisp_Object = lisp_h_Qnil ();

/// C: `<signal.h>`'s `#define SIG_DFL ((__sighandler_t) 0)` — the handler is the port's
/// `signal_handler_t` (`Option<fn>`, P-27), so `SIG_DFL` is `None`.
pub const SIG_DFL: signal_handler_t = None;

/* C 49-56: `#ifdef HAVE_SETRLIMIT`'s `static struct rlimit nofile_limit;` — the file's first
   file-scope object (its definition was owed since the first round).  */
static mut nofile_limit: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };

unsafe extern "C" {
  /// C: Lisp_Object list_system_processes (void)   (sysdep.c, process.h 299)
  fn list_system_processes () -> Lisp_Object;
  /// C: Lisp_Object system_process_attributes (Lisp_Object)   (sysdep.c, process.h 300)
  fn system_process_attributes (pid: Lisp_Object) -> Lisp_Object;
  /// C: int sig2str (int, char *)   (sig2str.h 29)
  fn sig2str (signum: c_int, str_: *mut c_char) -> c_int;
}
/// C: `#define SIG2STR_MAX 16` (sig2str.h 24)
pub const SIG2STR_MAX: usize = 16;
/// C: `#define SA_SIGINFO 0x0040` (`<signal.h>`)
pub const SA_SIGINFO: c_int = 0x0040;

/// C 9100-9102's `#define ADD_SUBFEATURE(key, val) subfeatures = Fcons (Fcons (key,
/// Fcons (val, Qnil)), subfeatures)` — the port's macro takes the local it assigns as its
/// first argument (mixed-site hygiene, docs/status.md 3.69).  */
macro_rules! ADD_SUBFEATURE {
  ($subfeatures:ident, $key:expr, $val:expr) =>
    { $subfeatures = Fcons (Fcons ($key, Fcons ($val, Qnil ())), $subfeatures) };
}

/* C 8813-9123: `syms_of_process`.  The `DEFSYM`s are the port's no-op macro (P-23); the
   `DEFVAR_*` carriers are defined at the end of this file (status 3.29); `ADD_SUBFEATURE`
   takes the `subfeatures` local as its first argument because a `macro_rules!` body resolves a
   bare path at its *definition* site (mixed-site hygiene, docs/status.md 3.69), and C's
   two-argument call shape is recorded in the ledger's Decisions.  */
#[unsafe(no_mangle)]
pub unsafe fn syms_of_process ()
{
  DEFSYM! (Qmake_process, c"make-process");
  DEFSYM! (Qlist_system_processes, c"list-system-processes");
  DEFSYM! (Qprocess_attributes, c"process-attributes");

  DEFSYM! (Qprocessp, c"processp");
  DEFSYM! (Qrun, c"run");
  DEFSYM! (Qstop, c"stop");
  DEFSYM! (Qsignal, c"signal");

  /* Qexit is already staticpro'd by syms_of_eval; don't staticpro it
     here again.  */

  DEFSYM! (Qopen, c"open");
  DEFSYM! (Qclosed, c"closed");
  DEFSYM! (Qconnect, c"connect");
  DEFSYM! (Qfailed, c"failed");
  DEFSYM! (Qlisten, c"listen");
  DEFSYM! (Qlocal, c"local");
  DEFSYM! (Qipv4, c"ipv4");
  DEFSYM! (Qipv6, c"ipv6");
  DEFSYM! (Qnumeric, c"numeric");
  DEFSYM! (Qdatagram, c"datagram");
  DEFSYM! (Qseqpacket, c"seqpacket");

  DEFSYM! (QCport, c":port");
  DEFSYM! (QCspeed, c":speed");
  DEFSYM! (QCprocess, c":process");

  DEFSYM! (QCbytesize, c":bytesize");
  DEFSYM! (QCstopbits, c":stopbits");
  DEFSYM! (QCparity, c":parity");
  DEFSYM! (Qodd, c"odd");
  DEFSYM! (Qeven, c"even");
  DEFSYM! (QCflowcontrol, c":flowcontrol");
  DEFSYM! (Qhw, c"hw");
  DEFSYM! (Qsw, c"sw");
  DEFSYM! (QCsummary, c":summary");

  DEFSYM! (Qreal, c"real");
  DEFSYM! (Qnetwork, c"network");
  DEFSYM! (Qserial, c"serial");
  DEFSYM! (QCfile_handler, c":file-handler");
  DEFSYM! (QCbuffer, c":buffer");
  DEFSYM! (QChost, c":host");
  DEFSYM! (QCservice, c":service");
  DEFSYM! (QClocal, c":local");
  DEFSYM! (QCremote, c":remote");
  DEFSYM! (QCcoding, c":coding");
  DEFSYM! (QCserver, c":server");
  DEFSYM! (QCnowait, c":nowait");
  DEFSYM! (QCsentinel, c":sentinel");
  DEFSYM! (QCuse_external_socket, c":use-external-socket");
  DEFSYM! (QCtls_parameters, c":tls-parameters");
  DEFSYM! (Qnsm_verify_connection, c"nsm-verify-connection");
  DEFSYM! (QClog, c":log");
  DEFSYM! (QCnoquery, c":noquery");
  DEFSYM! (QCstop, c":stop");
  DEFSYM! (QCplist, c":plist");
  DEFSYM! (QCcommand, c":command");
  DEFSYM! (QCconnection_type, c":connection-type");
  DEFSYM! (QCstderr, c":stderr");
  DEFSYM! (Qpty, c"pty");
  DEFSYM! (Qpipe, c"pipe");

  DEFSYM! (Qlast_nonmenu_event, c"last-nonmenu-event");

  staticpro (&raw mut Vprocess_alist);
  staticpro (&raw mut deleted_pid_list);

  DEFSYM! (QCname, c":name");
  DEFSYM! (QCtype, c":type");

  DEFSYM! (Qeuid, c"euid");
  DEFSYM! (Qegid, c"egid");
  DEFSYM! (Quser, c"user");
  DEFSYM! (Qgroup, c"group");
  DEFSYM! (Qcomm, c"comm");
  DEFSYM! (Qstate, c"state");
  DEFSYM! (Qppid, c"ppid");
  DEFSYM! (Qpgrp, c"pgrp");
  DEFSYM! (Qsess, c"sess");
  DEFSYM! (Qttname, c"ttname");
  DEFSYM! (Qtpgid, c"tpgid");
  DEFSYM! (Qminflt, c"minflt");
  DEFSYM! (Qmajflt, c"majflt");
  DEFSYM! (Qcminflt, c"cminflt");
  DEFSYM! (Qcmajflt, c"cmajflt");
  DEFSYM! (Qutime, c"utime");
  DEFSYM! (Qstime, c"stime");
  DEFSYM! (Qtime, c"time");
  DEFSYM! (Qcutime, c"cutime");
  DEFSYM! (Qcstime, c"cstime");
  DEFSYM! (Qctime, c"ctime");
  DEFSYM! (Qinternal_default_process_sentinel, c"internal-default-process-sentinel");
  DEFSYM! (Qinternal_default_process_filter, c"internal-default-process-filter");
  DEFSYM! (Qpri, c"pri");
  DEFSYM! (Qnice, c"nice");
  DEFSYM! (Qthcount, c"thcount");
  DEFSYM! (Qstart, c"start");
  DEFSYM! (Qvsize, c"vsize");
  DEFSYM! (Qrss, c"rss");
  DEFSYM! (Qetime, c"etime");
  DEFSYM! (Qpcpu, c"pcpu");
  DEFSYM! (Qpmem, c"pmem");
  DEFSYM! (Qargs, c"args");
  DEFSYM! (Qall, c"all");
  DEFSYM! (Qcurrent, c"current");

  DEFVAR_BOOL! (c"delete-exited-processes", delete_exited_processes,
	  "Non-nil means delete processes immediately when they exit.\nA value of nil means don't delete them until `list-processes' is run.");

  delete_exited_processes = true;

  DEFVAR_LISP! (c"process-connection-type", Vprocess_connection_type,
	  "Control type of device used to communicate with subprocesses.\nValues are nil to use a pipe, or t or `pty' to use a pty.\nThe value has no effect if the system has no ptys or if all ptys are busy:\nthen a pipe is used in any case.\nThe value takes effect when `start-process' is called.");
  Vprocess_connection_type = Qt ();

  DEFVAR_LISP! (c"process-adaptive-read-buffering", Vprocess_adaptive_read_buffering,
	  "If non-nil, improve receive buffering by delaying after short reads.\nOn some systems, when Emacs reads the output from a subprocess, the output data\nis read in very small blocks, potentially resulting in very poor performance.\nThis behavior can be remedied to some extent by setting this variable to a\nnon-nil value, as it will automatically delay reading from such processes, to\nallow them to produce more output before Emacs tries to read it.\nIf the value is t, the delay is reset after each write to the process; any other\nnon-nil value means that the delay is not reset on write.\nThe variable takes effect when `start-process' is called.");
  Vprocess_adaptive_read_buffering = Qnil ();

  DEFVAR_BOOL! (c"process-prioritize-lower-fds", process_prioritize_lower_fds,
	  "Whether to start checking for subprocess output from first file descriptor.\nEmacs loops through file descriptors to check for output from subprocesses.\nIf this variable is nil, the default, then after accepting output from a\nsubprocess, Emacs will continue checking the rest of descriptors, starting\nfrom the one following the descriptor it just read.  If this variable is\nnon-nil, Emacs will always restart the loop from the first file descriptor,\nthus favoring processes with lower descriptors.");
  process_prioritize_lower_fds = false;

  DEFVAR_LISP! (c"interrupt-process-functions", Vinterrupt_process_functions,
	  "List of functions to be called for `interrupt-process'.\nThe arguments of the functions are the same as for `interrupt-process'.\nThese functions are called in the order of the list, until one of them\nreturns non-nil.");
  Vinterrupt_process_functions = list1 (Qinternal_default_interrupt_process ());

  DEFVAR_LISP! (c"signal-process-functions", Vsignal_process_functions,
	  "List of functions to be called for `signal-process'.\nThe arguments of the functions are the same as for `signal-process'.\nThese functions are called in the order of the list, until one of them\nreturns non-nil.");
  Vsignal_process_functions = list1 (Qinternal_default_signal_process ());

  DEFVAR_LISP! (c"internal--daemon-sockname", Vinternal__daemon_sockname,
	  "Name of external socket passed to Emacs, or nil if none.");
  Vinternal__daemon_sockname = Qnil ();

  DEFVAR_INT! (c"read-process-output-max", read_process_output_max,
	  "Maximum number of bytes to read from subprocess in a single chunk.\nEnlarge the value only if the subprocess generates very large (megabytes)\namounts of data in one go.\n\nOn GNU/Linux systems, the value should not exceed\n/proc/sys/fs/pipe-max-size.  See pipe(7) manpage for details.");
  read_process_output_max = 65536;

  DEFVAR_BOOL! (c"fast-read-process-output", fast_read_process_output,
	  "Non-nil to optimize the insertion of process output.\nWe skip calling `internal-default-process-filter' and don't allocate\nthe Lisp string that would be used as its argument.  Only affects the\ncase of asynchronous process with the default filter.");
  fast_read_process_output = true;

  DEFVAR_INT! (c"process-error-pause-time", process_error_pause_time,
	  "The number of seconds to pause after handling process errors.\nThis isn't used for all process-related errors, but is used when a\nsentinel or a process filter function has an error.");
  process_error_pause_time = 1;

  DEFSYM! (Qinternal_default_interrupt_process, c"internal-default-interrupt-process");
  DEFSYM! (Qinterrupt_process_functions, c"interrupt-process-functions");

  DEFSYM! (Qinternal_default_signal_process, c"internal-default-signal-process");
  DEFSYM! (Qsignal_process_functions, c"signal-process-functions");

  DEFSYM! (Qnull, c"null");
  DEFSYM! (Qpipe_process_p, c"pipe-process-p");
  DEFSYM! (Qmessage, c"message");

  defsubr (&raw mut Sprocessp);
  defsubr (&raw mut Sget_process);
  defsubr (&raw mut Sdelete_process);
  defsubr (&raw mut Sprocess_status);
  defsubr (&raw mut Sprocess_exit_status);
  defsubr (&raw mut Sprocess_id);
  defsubr (&raw mut Sprocess_name);
  defsubr (&raw mut Sprocess_tty_name);
  defsubr (&raw mut Sprocess_command);
  defsubr (&raw mut Sset_process_buffer);
  defsubr (&raw mut Sprocess_buffer);
  defsubr (&raw mut Sprocess_mark);
  defsubr (&raw mut Sset_process_filter);
  defsubr (&raw mut Sprocess_filter);
  defsubr (&raw mut Sset_process_sentinel);
  defsubr (&raw mut Sprocess_sentinel);
  defsubr (&raw mut Sset_process_thread);
  defsubr (&raw mut Sprocess_thread);
  defsubr (&raw mut Sset_process_window_size);
  defsubr (&raw mut Sset_process_inherit_coding_system_flag);
  defsubr (&raw mut Sset_process_query_on_exit_flag);
  defsubr (&raw mut Sprocess_query_on_exit_flag);
  defsubr (&raw mut Sprocess_contact);
  defsubr (&raw mut Sprocess_plist);
  defsubr (&raw mut Sset_process_plist);
  defsubr (&raw mut Sprocess_list);
  defsubr (&raw mut Smake_process);
  defsubr (&raw mut Smake_pipe_process);
  defsubr (&raw mut Sserial_process_configure);
  defsubr (&raw mut Smake_serial_process);
  defsubr (&raw mut Sset_network_process_option);
  defsubr (&raw mut Smake_network_process);
  defsubr (&raw mut Sformat_network_address);
  defsubr (&raw mut Snetwork_lookup_address_info);
  defsubr (&raw mut Snetwork_interface_list);
  defsubr (&raw mut Snetwork_interface_info);
  defsubr (&raw mut Sprocess_datagram_address);
  defsubr (&raw mut Sset_process_datagram_address);
  defsubr (&raw mut Saccept_process_output);
  defsubr (&raw mut Sprocess_send_region);
  defsubr (&raw mut Sprocess_send_string);
  defsubr (&raw mut Sinternal_default_interrupt_process);
  defsubr (&raw mut Sinterrupt_process);
  defsubr (&raw mut Skill_process);
  defsubr (&raw mut Squit_process);
  defsubr (&raw mut Sstop_process);
  defsubr (&raw mut Scontinue_process);
  defsubr (&raw mut Sprocess_running_child_p);
  defsubr (&raw mut Sprocess_send_eof);
  defsubr (&raw mut Sinternal_default_signal_process);
  defsubr (&raw mut Ssignal_process);
  defsubr (&raw mut Swaiting_for_user_input_p);
  defsubr (&raw mut Sprocess_type);
  defsubr (&raw mut Sinternal_default_process_sentinel);
  defsubr (&raw mut Sinternal_default_process_filter);
  defsubr (&raw mut Sset_process_coding_system);
  defsubr (&raw mut Sprocess_coding_system);

 {
   let mut subfeatures: Lisp_Object = Qnil ();
   let mut sopt: *const socket_options;

   ADD_SUBFEATURE! (subfeatures, QCnowait (), Qt ());
   ADD_SUBFEATURE! (subfeatures, QCtype (), Qdatagram ());
   ADD_SUBFEATURE! (subfeatures, QCtype (), Qseqpacket ());
   ADD_SUBFEATURE! (subfeatures, QCfamily (), Qlocal ());
   ADD_SUBFEATURE! (subfeatures, QCfamily (), Qipv4 ());
   ADD_SUBFEATURE! (subfeatures, QCfamily (), Qipv6 ());
   ADD_SUBFEATURE! (subfeatures, QCservice (), Qt ());
   ADD_SUBFEATURE! (subfeatures, QCserver (), Qt ());

   /* C's `for (sopt = socket_options; sopt->name; sopt++) subfeatures = Fcons
      (intern_c_string (sopt->name), subfeatures);`.  */
   sopt = (&raw const socket_options) as *const socket_options;
   while !(*sopt).name.is_null ()
     {
       subfeatures = Fcons (intern_c_string ((*sopt).name), subfeatures);
       sopt = sopt.offset (1);
     }

   Fprovide (intern_c_string (c"make-network-process".as_ptr ()), subfeatures);
 }

  defsubr (&raw mut Sget_buffer_process);
  defsubr (&raw mut Sprocess_inherit_coding_system_flag);
  defsubr (&raw mut Slist_system_processes);
  defsubr (&raw mut Sprocess_attributes);
  defsubr (&raw mut Snum_processors);
  defsubr (&raw mut Ssignal_names);
}

/* The `DEFVAR_*` variables this C file owns: C's storage is `globals.f_<vname>`
   (lisp.h 3495-3530), and `struct emacs_globals globals` is defined once, in alloc.c; the
   port's `include/globals.rs` mirrors the names as declarations, so the unit that owns one
   defines it here (status 3.29, the casetab.rs shape).  */
#[unsafe(no_mangle)]
pub static mut Vprocess_connection_type: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vprocess_adaptive_read_buffering: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vinterrupt_process_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut Vsignal_process_functions: Lisp_Object = lisp_h_Qnil ();
#[unsafe(no_mangle)]
pub static mut read_process_output_max: EMACS_INT = 0;
#[unsafe(no_mangle)]
pub static mut process_prioritize_lower_fds: bool = false;
