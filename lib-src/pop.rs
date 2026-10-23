// femacs: port of lib-src/pop.c @ fd7afced403.
//
// One-to-one port (PORTING.md P-86 ①): the reference is transcribed line by line in C's order,
// both arms of every `#if` are carried, and each arm's gate is the lower-case feature of the
// macro the reference tests.  The **file gate is `feature = "mail_use_pop"`** (C 26's
// `#ifdef MAIL_USE_POP`; `undef` in docs/baseline/config/darwin-arm64.macros.tsv 2691), on every
// item *and on the include graph* (P-86 ⑧).  With the frozen feature set nothing but
// `include/config.rs` and this header compiles - which is the reference's own reading: the
// frozen build compiles `lib-src/pop.o` (lib-src/Makefile 431/435 list it for the movemail
// program) yet `nm` reports no symbol and `size` reports `0 0 0 0 0 0` text, because the whole
// body is inside that false `#if`.
//
// The sub-gates, each decided by the same frozen table (`python3 tools/check-cond-eval.py
// lib-src/pop.c` -> `31 conditions, 0 unknown, 0 disagreements [VERIFIED]`; all of these macros
// are `undef` there):
//
//   * C 29/956/1005/1537/1563's `#ifdef WINDOWSNT`            -> `windowsnt`
//     (already registered, docs/baseline/feature-coverage.tsv 8; the other names are Owed 1);
//   * C 33's `#if defined __MINGW32_VERSION && __MINGW32_VERSION >= 5000002L`
//                                                            -> `mingw32_version_ge_5000002`
//     (the `mingw32_w32api_lt_3_15` naming shape `src/w32.rs` established for a version test);
//   * C 59/204/1022's `#ifdef HESIOD`                         -> `hesiod`;
//   * C 219's `#ifdef MAILHOST`                               -> `mailhost`;
//   * C 79/102/233/984/1014/1043/1106's `#ifdef KERBEROS`     -> `kerberos`, with
//     C 103/985/1111's `KERBEROS5`                            -> `kerberos5`
//     and the header selectors of C 80/83/86/89/94            -> `have_krb5_h`, `have_krb_h`,
//     `have_kerberosiv_krb_h`, `have_kerberos_krb_h`, `have_com_err_h`;
//   * C 1161/1168's `HAVE_KRB5_ERROR_TEXT`/`HAVE_KRB5_ERROR_E_TEXT`
//                                                    -> `have_krb5_error_text`/`have_krb5_error_e_text`.
//
// `MAIL_USE_LOCKF`, `MAIL_USE_SSL`, `MAIL_USE_TLS` and `HAVE_GETADDRINFO` appear **nowhere** in
// this file (they belong to lib-src/movemail.c); the 31 conditions above are the whole set.
//
// The two `#if 0` regions (C 117-119's `gettermination` prototype, C 1455-1485's definition) are
// dead in every configuration: they are carried as C's text inside comments (the cm.rs
// precedent, Deviation 6).
//
// Names C takes from headers the tree has no mirror for are declared in this unit at the
// `#include` line that brings them (P-88 ①), each with a `/// C:` line quoting the header; the
// layouts and constants that can be measured on this machine were measured with clang against
// the frozen target (readings in docs/ports/pop.md's Evidence), and the Windows and Kerberos
// surfaces - no toolchain here can compile them - are marked UNVERIFIED there (Owed 5).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports,
         unused_macros, unused_variables, unused_mut, unused_assignments, unused_unsafe,
         static_mut_refs, unsafe_op_in_unsafe_fn, clippy::all)]

// The include graph's mirrors spell their parameters with the whole `c_*` family, so the unit's
// `use` list brings every one of them (the movemail.rs / update-game-score.rs reading).
use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short,
              c_uchar, c_uint, c_ulong, c_ulonglong, c_ushort, c_void};

include!("../src/include/config.rs");        // C 24: `#include <config.h>`

/* C 28: `#include <sys/types.h>` (`uid_t` for `getpwuid`).  */
#[cfg (feature = "mail_use_pop")]
include!("../lib/include/sys_types.in.rs");

/* --- C 29-56: `#ifdef WINDOWSNT` / … socket surface … / `#else` / … POSIX surface … / `#endif`.
   The frozen target takes the `#else` arm; the Windows arm is carried (P-86 ①) with its names
   declared where no mirror carries them.  -------------------------------------------------- */

#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
include!("include/ntlib.rs");                // C 30: `#include "ntlib.h"`

/* C 31-38: `_WIN32_WINNT` is re-pinned for the `getaddrinfo` declarations, and `<windows.h>` (or
   `<winsock2.h>` under an older MinGW) plus `<ws2tcpip.h>` are included.  C 33's
   `__MINGW32_VERSION >= 5000002L` test cannot be evaluated without a MinGW toolchain, so the
   version it asks for is carried as the named constant below (Deviation 7).  */
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
pub const _WIN32_WINNT: c_int = 0x0501;      // C 31-32: `#undef _WIN32_WINNT` / `#define … 0x0501`

/* C 34-38 + C 46-49 + C 1603-1701: the Windows socket surface.  No mirror carries
   `<windows.h>`/`<winsock2.h>`/`<ws2tcpip.h>`, this machine has no Windows cross toolchain
   (`which x86_64-w64-mingw32-gcc` is empty), and `docs/ports/pop.md` records the whole arm as
   text-only: the declarations below are the names its text uses and their layout/values are
   UNVERIFIED (Owed 5).  `#undef SOCKET_ERROR` (C 46) has no Rust counterpart.  */
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
#[repr (C)]
pub struct WSADATA { pub data: [u8; 400] }   /* C: <winsock2.h>'s WSADATA - UNVERIFIED (Owed 5) */

#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
pub type HANDLE = *mut c_void;               /* C: <windows.h> - UNVERIFIED (Owed 5) */

#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
pub const SOCK_DGRAM: c_int = 2;             // C: <winsock2.h> - UNVERIFIED (Owed 5)
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
pub const EAI_AGAIN: c_int = 0;              // C: <ws2tcpip.h> `EAI_AGAIN` - UNVERIFIED (Owed 5):
                                             // the port carries the name with a placeholder, not
                                             // a number from memory (P-88 ②).
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
pub const ENETDOWN: c_int = 116;             // C: this target's <errno.h> on Windows: the
                                             // measured arm lib/include/errno.in.rs 144 carries.
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
pub const WSANO_RECOVERY: c_int = 11003;     // C: <winsock2.h> - UNVERIFIED (Owed 5)
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
pub const WSAEINVAL: c_int = 10022;          // C: <winsock2.h> - UNVERIFIED (Owed 5)
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
pub const WSAHOST_NOT_FOUND: c_int = 11001;  // C: <winsock2.h> - UNVERIFIED (Owed 5)
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
pub const WSA_NOT_ENOUGH_MEMORY: c_int = 8;  // C: <winsock2.h> - UNVERIFIED (Owed 5)

/* C 38's `<ws2tcpip.h>`: `struct addrinfo`.  The Windows layout is UNVERIFIED (no Windows
   toolchain here, Owed 5); the port carries the record the arm's text reads field by field.  */
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
unsafe extern "C" {
  /// C: <winsock2.h>: `int WSAStartup (WORD, LPWSADATA);`
  pub fn WSAStartup (wVersionRequested: c_ushort, lpWSAData: *mut WSADATA) -> c_int;
  /// C: <winsock2.h>: `int WSACleanup (void);`
  pub fn WSACleanup () -> c_int;
  /// C: <winsock2.h>: `int recv (SOCKET, char *, int, int);` (C 47's `RECV` macro)
  pub fn recv (s: c_int, buf: *mut c_char, len: c_int, flags: c_int) -> c_int;
  /// C: <winsock2.h>: `int send (SOCKET, const char *, int, int);` (C 48's `SEND` macro)
  pub fn send (s: c_int, buf: *const c_char, len: c_int, flags: c_int) -> c_int;
  /// C: <winsock2.h>: `int closesocket (SOCKET);` (C 49's `CLOSESOCKET` macro)
  pub fn closesocket (s: c_int) -> c_int;
  /// C: <windows.h>: `HMODULE LoadLibraryA (LPCSTR);` (C's `LoadLibrary` macro picks the A/W
  /// entry by UNICODE; the port names the A one, which C's `"Ws2_32.dll"` literal takes)
  pub fn LoadLibraryA (lpLibFileName: *const c_char) -> HANDLE;
  /// C: <windows.h>: `FARPROC GetProcAddress (HMODULE, LPCSTR);`
  pub fn GetProcAddress (hModule: HANDLE, lpProcName: *const c_char) -> *mut c_void;
}

/* C 39-45: `#define getaddrinfo sys_getaddrinfo` / `#define freeaddrinfo sys_freeaddrinfo` and
   the two forward declarations of the definitions at C 1603/1684.  The `#define`s are renames
   whose call sites the port writes with the `sys_` spelling inside the Windows gate (Deviation
   8); the forward declarations are dropped (PORTING.md 0.1-12, Deviation 2).  C 36's
   `<winsock2.h>` `recv`/`send` renames (`#undef` + `#define`, C 46-49) are the macros below.  */

/* C 51-52: `#include <netinet/in.h>` / `#include <sys/socket.h>` - the POSIX socket surface.
   No mirror carries either header, so the records, the aliases and the entries are declared
   here (P-88 ①).  Measured with clang on the frozen target (docs/ports/pop.md's Evidence):
   `struct sockaddr_in` is `sizeof` 16 with `sin_family`@1, `sin_port`@2, `sin_addr`@4;
   `struct sockaddr` is 16; `struct in_addr` 4; `socklen_t` is `u32`, `in_port_t` `u16`,
   `sa_family_t` `u8`.  */
#[cfg (feature = "mail_use_pop")]
#[repr (C)]
#[derive (Clone, Copy)]
pub struct in_addr { pub s_addr: u32 }       /* C: <netinet/in.h>: `struct in_addr { in_addr_t s_addr; }` */

#[cfg (feature = "mail_use_pop")]
#[repr (C)]
pub struct sockaddr_in
{
  pub sin_len: u8,                           /* Darwin's prefix (measured `sin_family`@1) */
  pub sin_family: u8,                        /* C: <netinet/in.h>: `sa_family_t sin_family;` */
  pub sin_port: u16,                         /* C: <netinet/in.h>: `in_port_t sin_port;` */
  pub sin_addr: in_addr,                     /* C: <netinet/in.h>: `struct in_addr sin_addr;` */
  pub sin_zero: [u8; 8],
}

#[cfg (feature = "mail_use_pop")]
#[repr (C)]
pub struct sockaddr
{
  pub sa_len: u8,
  pub sa_family: u8,
  pub sa_data: [u8; 14],
}

#[cfg (feature = "mail_use_pop")]
pub type socklen_t = u32;                    /* C: <sys/socket.h>: `__uint32_t socklen_t;` */

/* C 72's `<netdb.h>`: `struct addrinfo`, measured with clang on the frozen target - `sizeof`
   48, `ai_flags`@0, `ai_family`@4, `ai_socktype`@8, `ai_protocol`@12, `ai_addrlen`@16,
   `ai_canonname`@24, `ai_addr`@32, `ai_next`@40.  */
#[cfg (feature = "mail_use_pop")]
#[repr (C)]
pub struct addrinfo
{
  pub ai_flags: c_int,
  pub ai_family: c_int,
  pub ai_socktype: c_int,
  pub ai_protocol: c_int,
  pub ai_addrlen: socklen_t,
  pub ai_canonname: *mut c_char,
  pub ai_addr: *mut sockaddr,
  pub ai_next: *mut addrinfo,
}

#[cfg (feature = "mail_use_pop")]
pub const AF_INET: c_int = 2;                /* C: <sys/socket.h> - measured */
#[cfg (feature = "mail_use_pop")]
pub const PF_INET: c_int = 2;                /* C: <sys/socket.h> - measured */
#[cfg (feature = "mail_use_pop")]
pub const SOCK_STREAM: c_int = 1;            /* C: <sys/socket.h> - measured */
#[cfg (feature = "mail_use_pop")]
pub const AI_CANONNAME: c_int = 2;           /* C: <netdb.h> - measured */
#[cfg (all (feature = "mail_use_pop", not (feature = "windowsnt")))]
pub const EAI_AGAIN: c_int = 2;              /* C: <netdb.h> - measured */

#[cfg (feature = "mail_use_pop")]
unsafe extern "C" {
  /// C: <sys/socket.h>: `int socket (int, int, int);`
  pub fn socket (domain: c_int, type_: c_int, protocol: c_int) -> c_int;
  /// C: <sys/socket.h>: `int connect (int, const struct sockaddr *, socklen_t);`
  pub fn connect (socket: c_int, address: *mut sockaddr, address_len: socklen_t) -> c_int;
  /// C: <netinet/in.h>: `uint16_t htons (uint16_t);`
  pub fn htons (hostshort: c_ushort) -> c_ushort;
  /// C: <netdb.h>: `struct servent *getservbyname (const char *, const char *);`
  pub fn getservbyname (name: *const c_char, proto: *const c_char) -> *mut servent;
  /// C: <netdb.h>: `int getaddrinfo (const char *, const char *, const struct addrinfo *,
  /// struct addrinfo **);`
  pub fn getaddrinfo (node: *const c_char, service: *const c_char, hints: *const addrinfo,
                      res: *mut *mut addrinfo) -> c_int;
  /// C: <netdb.h>: `void freeaddrinfo (struct addrinfo *);`
  pub fn freeaddrinfo (res: *mut addrinfo);
}

/* C 53: `#include <pop.h>` - the declarations this unit defines (the eighteen functions and the
   two objects).  The mirror is live for lib-src/movemail.c; here it is reached exactly the way
   C reaches it, inside the file gate (P-86 ⑧'s include-graph half).  */
#[cfg (feature = "mail_use_pop")]
include!("include/pop.rs");                  // C 53: `#include <pop.h>`

/* C 59-68: `#ifdef HESIOD` / `#include <hesiod.h>` / the `hes_getservbyname` declaration /
   `#endif`.  No mirror carries `<hesiod.h>` and the frozen configuration leaves `HESIOD` undef,
   so the arm is carried as text with its two names declared here (P-88 ①); their layout is
   UNVERIFIED (Owed 5).  */
#[cfg (all (feature = "mail_use_pop", feature = "hesiod"))]
#[repr (C)]
pub struct hes_postoffice
{
  pub po_type: *mut c_char,
  pub po_name: *mut c_char,
  pub po_host: *mut c_char,
}

#[cfg (all (feature = "mail_use_pop", feature = "hesiod"))]
unsafe extern "C" {
  /// C 67: `extern struct servent *hes_getservbyname (/* char *, char * */);`
  pub fn hes_getservbyname (name: *const c_char, proto: *const c_char) -> *mut servent;
  /// C: <hesiod.h>: `struct hes_postoffice *hes_getmailhost (const char *);`
  pub fn hes_getmailhost (name: *const c_char) -> *mut hes_postoffice;
}

/* C 70: `#include <alloca.h>` - the disposition mirror (`alloca`'s live arm is clang's
   `__builtin_alloca`, which the port spells with the stack pool, P-05/`excluded-macros.txt`).  */
#[cfg (feature = "mail_use_pop")]
include!("../lib/include/alloca.in.rs");

/* C 71: `#include <pwd.h>` (`struct passwd`, `getpwuid`).  */
/* `<pwd.h>`'s `struct passwd` spells `pw_change`/`pw_expire` with `time_t`, which C's
   `<sys/types.h>` chain provides and the `pwd.rs` mirror asks its includer for; the `time.in.rs`
   mirror is where the port's `time_t` lives (the update-game-score.rs 42 reading).  */
#[cfg (feature = "mail_use_pop")]
include!("../lib/include/time.in.rs");
#[cfg (feature = "mail_use_pop")]
include!("../lib/include/pwd.rs");

/* C 72: `#include <netdb.h>` - no mirror carries it.  The two records and the two lookups the
   POSIX arm and the Windows arm of C 1603-1701 share are declared here.  Measured on the frozen
   target: `struct servent` is `sizeof` 32 (`s_port`@16 `int`, `s_proto`@24), `struct hostent` 32
   (`h_addrtype`@16, `h_length`@20, `h_addr_list`@24); the Windows layout is UNVERIFIED (Owed 5).
   `struct addrinfo` (the 48-byte record the POSIX arm's `getaddrinfo` uses, `ai_flags`@0,
   `ai_family`@4, `ai_socktype`@8, `ai_protocol`@12, `ai_addrlen`@16, `ai_canonname`@24,
   `ai_addr`@32, `ai_next`@40) is declared with that arm.  */
#[cfg (feature = "mail_use_pop")]
#[repr (C)]
pub struct servent
{
  pub s_name: *mut c_char,
  pub s_aliases: *mut *mut c_char,
  pub s_port: c_int,
  pub s_proto: *mut c_char,
}

#[cfg (feature = "mail_use_pop")]
#[repr (C)]
pub struct hostent
{
  pub h_name: *mut c_char,
  pub h_aliases: *mut *mut c_char,
  pub h_addrtype: c_int,
  pub h_length: c_int,
  pub h_addr_list: *mut *mut c_char,
}

#[cfg (feature = "mail_use_pop")]
unsafe extern "C" {
  /// C: <netdb.h>: `struct hostent *gethostbyname (const char *);`
  pub fn gethostbyname (name: *const c_char) -> *mut hostent;
}

/* C 73-77: `#include <errno.h>` / `<stdio.h>` / `<stdlib.h>` / `<string.h>` / `<unistd.h>`.  */
#[cfg (feature = "mail_use_pop")]
include!("../lib/include/errno.rs");         // C 73: `#include <errno.h>` (the port's `errno ()`)
#[cfg (feature = "mail_use_pop")]
include!("../lib/include/stdio.rs");         // C 74: `#include <stdio.h>` (`FILE`, `snprintf`,
                                             //         `sprintf`, `fprintf`, `stderr`)
#[cfg (feature = "mail_use_pop")]
include!("../lib/include/stdlib.rs");        // C 75: `#include <stdlib.h>` (`malloc`, `realloc`,
                                             //         `calloc`, `free`, `getenv`, `strtol`)
#[cfg (feature = "mail_use_pop")]
include!("../lib/include/string.rs");        // C 76: `#include <string.h>`
#[cfg (feature = "mail_use_pop")]
include!("../lib/include/unistd.in.rs");     // C 77: `#include <unistd.h>` (`close`, `read`,
                                             //         `write`, `getuid`)

/* C 75-77's use-site declarations (P-88 ①): the names those three headers give the file that no
   mirror in the graph carries.  Each is declared with the header's own line; the two Windows
   spellings come from C 30's `ntlib.h` mirror instead, which is why they are gated.  */
#[cfg (feature = "mail_use_pop")]
unsafe extern "C" {
  /// C: <stdlib.h>: `int atoi (const char *);`
  pub fn atoi (nptr: *const c_char) -> c_int;
  /// C: <string.h>: `size_t strnlen (const char *, size_t);` (the `string.rs` mirror carries
  /// the `strlen`/`strcmp`/`strcpy`/`strchr`/`strrchr`/`stpcpy` family, not this one)
  pub fn strnlen (s: *const c_char, maxlen: usize) -> usize;
}

#[cfg (all (feature = "mail_use_pop", not (feature = "windowsnt")))]
unsafe extern "C" {
  /// C: <unistd.h>: `char *getlogin (void);`
  pub fn getlogin () -> *mut c_char;
  /// C: <unistd.h>: `char *getpass (const char *);`
  pub fn getpass (prompt: *const c_char) -> *mut c_char;
}

#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
unsafe extern "C" {
  /// C: <string.h>: `char *strdup (const char *);` (C 1665's canonical-name copy)
  pub fn strdup (s: *const c_char) -> *mut c_char;
}

/* C 70's `alloca` sites (C 1091, C 1374) - the port has no `alloca` and the decision is the
   stack pool (`docs/ports/excluded-macros.txt`'s `alloca` row, PORTING.md P-05), whose entry
   `src/alloc.rs` defines and this unit declares.  */
#[cfg (feature = "mail_use_pop")]
unsafe extern "C" {
  /// C: `#define alloca __builtin_alloca` (lib/alloca.in.h 47): the port's spelling is the
  /// stack pool's allocator (`PORTING.md` P-05; `src/alloc.rs` 1039 defines it).
  pub fn sa_stack_alloc (size: usize) -> *mut c_void;
}

/* C 79-97: `#ifdef KERBEROS` / the five header selectors / `#endif`.  No mirror carries
   `<krb5.h>`, `<krb.h>`, `<kerberosIV/krb.h>`, `<kerberos/krb.h>` or `<com_err.h>`, and every
   one of those selectors is `undef` in the frozen table: the arm is carried as text (P-86 ①)
   and the names its text uses are declared where they appear below, all UNVERIFIED (Owed 5).  */

/* C 99-100: `#include <c-ctype.h>` / `#include <min-max.h>`.  */
#[cfg (feature = "mail_use_pop")]
include!("../lib/include/c-ctype.rs");       // C 99: `#include <c-ctype.h>` (`c_tolower`)
#[cfg (feature = "mail_use_pop")]
include!("../lib/include/min-max.rs");       // C 100: `#include <min-max.h>` (`min`/`max`)

/* C 102-110: `#ifdef KERBEROS` / `#ifndef KERBEROS5` / the two declarations of the V4 library's
   `krb_sendauth` and `krb_realmofhost` / `#endif`s.  These name functions from `<krb.h>`, not
   definitions of this file, so they stay declarations (P-91 does not apply).  */
#[cfg (all (feature = "mail_use_pop", feature = "kerberos", not (feature = "kerberos5")))]
unsafe extern "C" {
  /// C 104-108: `extern int krb_sendauth (/* long, int, KTEXT, char *, char *, char *,
  /// u_long, MSG_DAT *, CREDENTIALS *, Key_schedule, struct sockaddr_in *, struct sockaddr_in *,
  /// char * */);` - the comment is C's own K&R-style prototype suppression, kept.
  pub fn krb_sendauth (a: c_long, b: c_int, c: KTEXT, d: *mut c_char, e: *mut c_char,
                       f: *mut c_char, g: c_ulong, h: *mut MSG_DAT, i: *mut CREDENTIALS,
                       j: Key_schedule, k: *mut sockaddr_in, l: *mut sockaddr_in,
                       m: *mut c_char) -> c_int;
  /// C 109: `extern char *krb_realmofhost (/* char * */);`
  pub fn krb_realmofhost (host: *mut c_char) -> *mut c_char;
}

/* C 112-121: the seven static forward declarations (`socket_connection`, `pop_getline`,
   `sendline`, `fullwrite`, `getok`, C 117-119's `#if 0` `gettermination`, `pop_trash`,
   `find_crlf`) are dropped (PORTING.md 0.1-12, Deviation 2) - Rust has no declaration without a
   definition, and every definition is below at its own C position.  */

/* C 123-131: the file's macros.  */
#[cfg (feature = "mail_use_pop")]
pub const ERROR_MAX: usize = 160;            // C 123-125: `#define ERROR_MAX 160`
#[cfg (feature = "mail_use_pop")]
pub const POP_PORT: c_int = 110;             // C 126: `#define POP_PORT 110`
#[cfg (feature = "mail_use_pop")]
const POP_SERVICE: &core::ffi::CStr = c"pop3";    // C 127: `#define POP_SERVICE "pop3"`
#[cfg (all (feature = "mail_use_pop", feature = "kerberos"))]
pub const KPOP_PORT: c_int = 1109;           // C 129: `#define KPOP_PORT 1109`
#[cfg (all (feature = "mail_use_pop", feature = "kerberos"))]
const KPOP_SERVICE: &core::ffi::CStr = c"kpop";   // C 130: `#define KPOP_SERVICE "kpop"`

/* C 133-134: the file's two file-scope objects.  `pop_error` is `char[ERROR_MAX]` here and
   `extern char pop_error[]` (unknown bound) in the mirror, which is what C's header says; the
   difference is the `clashing_extern_declarations` allowance docs/ports/include-pop.md records.  */
/* C 46-49: `#undef SOCKET_ERROR` and the Windows arm's three macros
   (`RECV`/`SEND`/`CLOSESOCKET` -> the winsock entries).  The port writes them as
   `macro_rules!` invoked with `!` - the token C's preprocessor needs, spelled in Rust's syntax
   (Deviation 4).  */
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
macro_rules! RECV { ($s:expr, $buf:expr, $len:expr, $flags:expr) => { recv ($s, $buf, $len, $flags) }; }
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
macro_rules! SEND { ($s:expr, $buf:expr, $len:expr, $flags:expr) => { send ($s, $buf, $len, $flags) }; }
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
macro_rules! CLOSESOCKET { ($s:expr) => { closesocket ($s) }; }

/* C 53-55: the POSIX arm's three macros (`RECV`/`SEND`/`CLOSESOCKET` -> `read`/`write`/
   `close`), with the pointer and width conversions C's call sites leave to the prototype
   (Deviation 5).  */
#[cfg (all (feature = "mail_use_pop", not (feature = "windowsnt")))]
macro_rules! RECV {
  ($s:expr, $buf:expr, $len:expr, $flags:expr) =>
    { read ($s, $buf as *mut c_void, $len as usize) as c_int };
}
#[cfg (all (feature = "mail_use_pop", not (feature = "windowsnt")))]
macro_rules! SEND {
  ($s:expr, $buf:expr, $len:expr, $flags:expr) =>
    { write ($s, $buf as *const c_void, $len as usize) as c_int };
}
#[cfg (all (feature = "mail_use_pop", not (feature = "windowsnt")))]
macro_rules! CLOSESOCKET { ($s:expr) => { close ($s) }; }

#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub static mut pop_error: [c_char; ERROR_MAX] = [0; ERROR_MAX];
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub static mut pop_debug: bool = false;

/* C 219-224: `#ifdef MAILHOST` / `host = MAILHOST;` - `MAILHOST` is the configure parameter
   (`--with-mailhost`), and the frozen table has it `undef`, so the arm's *value* is not
   measured: the port carries the name with a placeholder (Owed 3, the P-88 ② rule - no string
   from memory).  */
#[cfg (all (feature = "mail_use_pop", feature = "mailhost"))]
const MAILHOST: &core::ffi::CStr = c"";      /* Owed 3: placeholder, not measured */

/*
 * Function: pop_open (char *host, char *username, char *password,
 * 		       int flags)
 *
 * Purpose: Establishes a connection with a post-office server, and
 * 	completes the authorization portion of the session.
 *
 * Arguments:
 * 	host	The server host with which the connection should be
 * 		established.  Optional.  If omitted, internal
 * 		heuristics will be used to determine the server host,
 * 		if possible.
 * 	username
 * 		The username of the mail-drop to access.  Optional.
 * 		If omitted, internal heuristics will be used to
 * 		determine the username, if possible.
 * 	password
 * 		The password to use for authorization.  If omitted,
 * 		internal heuristics will be used to determine the
 * 		password, if possible.
 * 	flags	A bit mask containing flags controlling certain
 * 		functions of the routine.  Valid flags are defined in
 * 		the file pop.h
 *
 * Return value: Upon successful establishment of a connection, a
 * 	non-null popserver will be returned.  Otherwise, null will be
 * 	returned, and the string variable pop_error will contain an
 * 	explanation of the error.
 */
// C 136-164: C's doc comment above, verbatim; C 165-166's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_open (mut host: *mut c_char, mut username: *mut c_char,
                                   mut password: *mut c_char, mut flags: c_int)
  -> popserver
{
  let mut sock: c_int;
  let mut server: popserver;

  /* Determine the user name */
  if username.is_null ()
    {
      username = getenv (c"USER".as_ptr ());
      if ! (! username.is_null () && *username != 0)
	{
	  username = getlogin ();
	  if ! (! username.is_null () && *username != 0)
	    {
	      let mut passwd: *mut passwd;
	      passwd = getpwuid (getuid ());
	      if ! passwd.is_null () && ! (*passwd).pw_name.is_null () && *(*passwd).pw_name != 0
		{
		  username = (*passwd).pw_name;
		}
	      else
		{
		  strcpy (pop_error.as_mut_ptr (), c"Could not determine username".as_ptr ());
		  return std::ptr::null_mut ();
		}
	    }
	}
    }

  /*
   *  Determine the mail host.
   */

  if host.is_null ()
    {
      host = getenv (c"MAILHOST".as_ptr ());
    }

  /* C 204-217: `#ifdef HESIOD` - the Hesiod post-office lookup.  */
  #[cfg (feature = "hesiod")]
  if host.is_null () && (flags & POP_NO_HESIOD) == 0
    {
      let mut office: *mut hes_postoffice;
      office = hes_getmailhost (username as *const c_char);
      if ! office.is_null () && ! (*office).po_type.is_null ()
	  && 0 == strcmp ((*office).po_type, c"POP".as_ptr ())
	  && ! (*office).po_name.is_null () && *(*office).po_name != 0
	  && ! (*office).po_host.is_null () && *(*office).po_host != 0
	{
	  host = (*office).po_host;
	  username = (*office).po_name;
	}
    }

  /* C 219-224: `#ifdef MAILHOST` / `host = MAILHOST;`  - `MAILHOST` is a configure parameter
     (`--with-mailhost`) and the frozen table has it `undef`, so its *value* is not measured:
     the arm is carried with a placeholder and named as Owed 3 (P-88 ② - no value from memory).  */
  #[cfg (feature = "mailhost")]
  if host.is_null ()
    {
      host = MAILHOST.as_ptr () as *mut c_char;
    }

  if host.is_null ()
    {
      strcpy (pop_error.as_mut_ptr (), c"Could not determine POP server".as_ptr ());
      return std::ptr::null_mut ();
    }

  /* Determine the password */
  /* C 233-237: `#ifdef KERBEROS` / `#define DONT_NEED_PASSWORD (! (flags & POP_NO_KERBEROS))` /
     `#else` / `#define DONT_NEED_PASSWORD 0` / `#endif`  */
  #[cfg (feature = "kerberos")]
  let DONT_NEED_PASSWORD: bool = (flags & POP_NO_KERBEROS) == 0;
  #[cfg (not (feature = "kerberos"))]
  let DONT_NEED_PASSWORD: bool = false;

  if password.is_null () && ! DONT_NEED_PASSWORD
    {
      if (flags & POP_NO_GETPASS) == 0
	{
	  password = getpass (c"Enter POP password:".as_ptr ());
	}
      if password.is_null ()
	{
	  strcpy (pop_error.as_mut_ptr (), c"Could not determine POP password".as_ptr ());
	  return std::ptr::null_mut ();
	}
    }
  if ! password.is_null ()		/* always true, detected 20060515 */
    { flags |= POP_NO_KERBEROS; }
  else
    { password = username; }		/* dead code, detected 20060515 */
  /* "kpop" service is  never used: look for 20060515 to see why */

  sock = socket_connection (host, flags);
  if sock == -1
    { return std::ptr::null_mut (); }

  server = malloc (std::mem::size_of::<_popserver> ()) as popserver;
  if server.is_null ()
    {
      strcpy (pop_error.as_mut_ptr (), c"Out of memory in pop_open".as_ptr ());
      return std::ptr::null_mut ();
    }
  (*server).buffer = malloc (GETLINE_MIN as usize) as *mut c_char;
  if (*server).buffer.is_null ()
    {
      strcpy (pop_error.as_mut_ptr (), c"Out of memory in pop_open".as_ptr ());
      free (server as *mut c_void);
      return std::ptr::null_mut ();
    }

  (*server).file = sock;
  (*server).data = 0;
  (*server).buffer_index = 0;
  (*server).buffer_size = GETLINE_MIN;
  popserver_bits! (in_multi, *server, false);
  popserver_bits! (trash_started, *server, false);

  if getok (server) != 0
    { return std::ptr::null_mut (); }

  /*
   * I really shouldn't use the pop_error variable like this, but....
   */
  if strnlen (username as *const c_char, ERROR_MAX - 6 + 1) == ERROR_MAX - 6 + 1
    {
      pop_close (server);
      strcpy (pop_error.as_mut_ptr (),
	      c"Username too long; recompile pop.c with larger ERROR_MAX".as_ptr ());
      return std::ptr::null_mut ();
    }
  sprintf (pop_error.as_mut_ptr (), c"USER %s".as_ptr (), username);

  if sendline (server, pop_error.as_mut_ptr ()) != 0 || getok (server) != 0
    { return std::ptr::null_mut (); }

  if strnlen (password as *const c_char, ERROR_MAX - 6 + 1) == ERROR_MAX - 6 + 1
    {
      pop_close (server);
      strcpy (pop_error.as_mut_ptr (),
	      c"Password too long; recompile pop.c with larger ERROR_MAX".as_ptr ());
      return std::ptr::null_mut ();
    }
  sprintf (pop_error.as_mut_ptr (), c"PASS %s".as_ptr (), password);

  if sendline (server, pop_error.as_mut_ptr ()) != 0 || getok (server) != 0
    { return std::ptr::null_mut (); }

  server
}

/*
 * Function: pop_stat (C 319-331)
 *
 * Purpose: Issue the STAT command to the server and return (in the value parameters) the number
 * of messages in the maildrop and the total size of the maildrop.
 *
 * Return value: 0 on success, or non-zero with an error in pop_error in failure.
 *
 * Side effects: On failure, may make further operations on the connection impossible.
 */
// C 319-331: C's doc comment; C 332-333's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_stat (server: popserver, count: *mut c_int, size: *mut c_int)
  -> c_int
{
  let mut fromserver: *mut c_char = std::ptr::null_mut ();
  let mut end_ptr: *mut c_char = std::ptr::null_mut ();

  if popserver_bits! (in_multi, *server)
    {
      strcpy (pop_error.as_mut_ptr (), c"In multi-line query in pop_stat".as_ptr ());
      return -1;
    }

  if sendline (server, c"STAT".as_ptr ()) != 0 || pop_getline (server, &mut fromserver) < 0
    { return -1; }

  if strncmp (fromserver, c"+OK ".as_ptr (), 4) != 0
    {
      if 0 == strncmp (fromserver, c"-ERR".as_ptr (), 4)
	{ snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s".as_ptr (), fromserver); }
      else
	{
	  strcpy (pop_error.as_mut_ptr (),
		  c"Unexpected response from POP server in pop_stat".as_ptr ());
	  pop_trash (server);
	}
      return -1;
    }

  *errno () = 0;
  *count = strtol (fromserver.offset (4) as *const c_char, &mut end_ptr, 10) as c_int;
  /* Check validity of string-to-integer conversion. */
  if fromserver.offset (4) == end_ptr || *end_ptr != b' ' as c_char || *errno () != 0
    {
      strcpy (pop_error.as_mut_ptr (),
	      c"Unexpected response from POP server in pop_stat".as_ptr ());
      pop_trash (server);
      return -1;
    }

  fromserver = end_ptr;

  *errno () = 0;
  *size = strtol (fromserver.offset (1) as *const c_char, &mut end_ptr, 10) as c_int;
  if fromserver.offset (1) == end_ptr || *errno () != 0
    {
      strcpy (pop_error.as_mut_ptr (),
	      c"Unexpected response from POP server in pop_stat".as_ptr ());
      pop_trash (server);
      return -1;
    }

  0
}

/*
 * Function: pop_list (C 384-402)
 *
 * Purpose: Performs the POP "list" command and returns (in value parameters) two malloc'd
 * zero-terminated arrays -- one of message IDs, and a parallel one of sizes.
 *
 * Arguments: server, message (0 for all messages).
 *
 * Return value: 0 on success, non-zero with error in pop_error on failure.
 *
 * Side effects: On failure, may make further operations on the connection impossible.
 */
// C 384-402: C's doc comment; C 403-404's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_list (server: popserver, message: c_int, IDs: *mut *mut c_int,
                                   sizes: *mut *mut c_int) -> c_int
{
  let mut how_many: c_int;
  let mut i: c_int;
  let mut fromserver: *mut c_char = std::ptr::null_mut ();

  if popserver_bits! (in_multi, *server)
    {
      strcpy (pop_error.as_mut_ptr (), c"In multi-line query in pop_list".as_ptr ());
      return -1;
    }

  if message != 0
    { how_many = 1; }
  else
    {
      let mut count: c_int = 0;
      let mut size: c_int = 0;
      if pop_stat (server, &mut count, &mut size) != 0
	{ return -1; }
      how_many = count;
    }

  *IDs = malloc (((how_many + 1) as usize) * std::mem::size_of::<c_int> ()) as *mut c_int;
  *sizes = malloc (((how_many + 1) as usize) * std::mem::size_of::<c_int> ()) as *mut c_int;
  if ! (! (*IDs).is_null () && ! (*sizes).is_null ())
    {
      strcpy (pop_error.as_mut_ptr (), c"Out of memory in pop_list".as_ptr ());
      return -1;
    }

  if message != 0
    {
      sprintf (pop_error.as_mut_ptr (), c"LIST %d".as_ptr (), message);
      if sendline (server, pop_error.as_mut_ptr ()) != 0
	{
	  free (*IDs as *mut c_void);
	  free (*sizes as *mut c_void);
	  return -1;
	}
      if pop_getline (server, &mut fromserver) < 0
	{
	  free (*IDs as *mut c_void);
	  free (*sizes as *mut c_void);
	  return -1;
	}
      if strncmp (fromserver, c"+OK ".as_ptr (), 4) != 0
	{
	  if 0 == strncmp (fromserver, c"-ERR".as_ptr (), 4)
	    { snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s".as_ptr (), fromserver); }
	  else
	    {
	      strcpy (pop_error.as_mut_ptr (),
		      c"Unexpected response from server in pop_list".as_ptr ());
	      pop_trash (server);
	    }
	  free (*IDs as *mut c_void);
	  free (*sizes as *mut c_void);
	  return -1;
	}
      *(*IDs).offset (0) = atoi (fromserver.offset (4) as *const c_char);
      fromserver = strchr (fromserver.offset (4) as *const c_char, b' ' as c_int);
      if fromserver.is_null ()
	{
	  strcpy (pop_error.as_mut_ptr (),
		  c"Badly formatted response from server in pop_list".as_ptr ());
	  pop_trash (server);
	  free (*IDs as *mut c_void);
	  free (*sizes as *mut c_void);
	  return -1;
	}
      *(*sizes).offset (0) = atoi (fromserver as *const c_char);
      *(*IDs).offset (1) = 0;
      *(*sizes).offset (1) = 0;
      return 0;
    }
  else
    {
      if pop_multi_first (server, c"LIST".as_ptr (), &mut fromserver) != 0
	{
	  free (*IDs as *mut c_void);
	  free (*sizes as *mut c_void);
	  return -1;
	}
      i = 0;
      while i < how_many
	{
	  if pop_multi_next (server, &mut fromserver) <= 0
	    {
	      free (*IDs as *mut c_void);
	      free (*sizes as *mut c_void);
	      return -1;
	    }
	  *(*IDs).offset (i as isize) = atoi (fromserver as *const c_char);
	  fromserver = strchr (fromserver as *const c_char, b' ' as c_int);
	  if fromserver.is_null ()
	    {
	      strcpy (pop_error.as_mut_ptr (),
		      c"Badly formatted response from server in pop_list".as_ptr ());
	      free (*IDs as *mut c_void);
	      free (*sizes as *mut c_void);
	      pop_trash (server);
	      return -1;
	    }
	  *(*sizes).offset (i as isize) = atoi (fromserver as *const c_char);
	  i += 1;
	}
      if pop_multi_next (server, &mut fromserver) < 0
	{
	  free (*IDs as *mut c_void);
	  free (*sizes as *mut c_void);
	  return -1;
	}
      else if ! fromserver.is_null ()
	{
	  strcpy (pop_error.as_mut_ptr (),
		  c"Too many response lines from server in pop_list".as_ptr ());
	  free (*IDs as *mut c_void);
	  free (*sizes as *mut c_void);
	  return -1;
	}
      *(*IDs).offset (i as isize) = 0;
      *(*sizes).offset (i as isize) = 0;
      return 0;
    }
}

/*
 * Function: pop_retrieve (C 525-544)
 *
 * Purpose: Retrieve a specified message from the maildrop.
 *
 * Arguments: server, message, markfrom (mark "From " lines with '>'), msg_buf.
 *
 * Return value: The number of bytes in msg_buf, which may contain embedded nulls, not including
 * its final null, or -1 on error with pop_error set.
 *
 * Side effects: May kill connection on error.
 */
// C 525-544: C's doc comment; C 545-546's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_retrieve (server: popserver, message: c_int, markfrom: c_int,
                                       msg_buf: *mut *mut c_char) -> c_int
{
  let mut IDs: *mut c_int = std::ptr::null_mut ();
  let mut sizes: *mut c_int = std::ptr::null_mut ();
  let mut bufsize: c_int;
  let mut fromcount: c_int = 0;
  let mut cp: c_int = 0;
  let mut ptr: *mut c_char;
  let mut fromserver: *mut c_char = std::ptr::null_mut ();
  let mut ret: c_int;

  if popserver_bits! (in_multi, *server)
    {
      strcpy (pop_error.as_mut_ptr (), c"In multi-line query in pop_retrieve".as_ptr ());
      return -1;
    }

  if pop_list (server, message, &mut IDs, &mut sizes) != 0
    { return -1; }

  if pop_retrieve_first (server, message, &mut fromserver) != 0
    { return -1; }

  /*
   * The "5" below is an arbitrary constant -- I assume that if there
   * are "From" lines in the text to be marked, there probably won't
   * be more than 5 of them.  If there are, I allocate more space for
   * them below.
   */
  bufsize = *sizes.offset (0) + (if markfrom != 0 { 5 } else { 0 });
  ptr = malloc (bufsize as usize) as *mut c_char;
  free (IDs as *mut c_void);
  free (sizes as *mut c_void);

  if ptr.is_null ()
    {
      strcpy (pop_error.as_mut_ptr (), c"Out of memory in pop_retrieve".as_ptr ());
      pop_retrieve_flush (server);
      return -1;
    }

  loop
    {
      ret = pop_retrieve_next (server, &mut fromserver);
      if ! (ret >= 0) { break; }
      if fromserver.is_null ()
	{
	  *ptr.offset (cp as isize) = 0;
	  *msg_buf = ptr;
	  return cp;
	}
      if markfrom != 0 && *fromserver.offset (0) == b'F' as c_char
	  && *fromserver.offset (1) == b'r' as c_char
	  && *fromserver.offset (2) == b'o' as c_char
	  && *fromserver.offset (3) == b'm' as c_char
	  && *fromserver.offset (4) == b' ' as c_char
	{
	  fromcount += 1;
	  if fromcount == 5
	    {
	      bufsize += 5;
	      ptr = realloc (ptr as *mut c_void, bufsize as usize) as *mut c_char;
	      if ptr.is_null ()
		{
		  strcpy (pop_error.as_mut_ptr (), c"Out of memory in pop_retrieve".as_ptr ());
		  pop_retrieve_flush (server);
		  return -1;
		}
	      fromcount = 0;
	    }
	  *ptr.offset (cp as isize) = b'>' as c_char;
	  cp += 1;
	}
      memcpy (ptr.offset (cp as isize) as *mut c_void, fromserver as *const c_void,
	      ret as usize);
      cp += ret;
      *ptr.offset (cp as isize) = b'\n' as c_char;
      cp += 1;
    }

  free (ptr as *mut c_void);
  -1
}

// C 619-624: `int pop_retrieve_first (popserver server, int message, char **response)`
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_retrieve_first (server: popserver, message: c_int,
                                             response: *mut *mut c_char) -> c_int
{
  sprintf (pop_error.as_mut_ptr (), c"RETR %d".as_ptr (), message);
  pop_multi_first (server, pop_error.as_mut_ptr (), response)
}

/*
  Returns a negative number on error, 0 to indicate that the data has
  all been read (i.e., the server has returned a "." termination
  line), or a positive number indicating the number of bytes in the
  returned buffer (which is null-terminated and may contain embedded
  nulls, but the returned bytecount doesn't include the final null).
  */
// C 626-632: C's comment above; C 634-635's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_retrieve_next (server: popserver, line: *mut *mut c_char) -> c_int
{
  pop_multi_next (server, line)
}

// C 640-644: `int pop_retrieve_flush (popserver server)`
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_retrieve_flush (server: popserver) -> c_int
{
  pop_multi_flush (server)
}

// C 646-651: `int pop_top_first (popserver server, int message, int lines, char **response)`
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_top_first (server: popserver, message: c_int, lines: c_int,
                                        response: *mut *mut c_char) -> c_int
{
  sprintf (pop_error.as_mut_ptr (), c"TOP %d %d".as_ptr (), message, lines);
  pop_multi_first (server, pop_error.as_mut_ptr (), response)
}

/*
  Returns a negative number on error, 0 to indicate that the data has
  all been read (i.e., the server has returned a "." termination
  line), or a positive number indicating the number of bytes in the
  returned buffer (which is null-terminated and may contain embedded
  nulls, but the returned bytecount doesn't include the final null).
  */
// C 653-659: C's comment above; C 661-662's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_top_next (server: popserver, line: *mut *mut c_char) -> c_int
{
  pop_multi_next (server, line)
}

// C 667-671: `int pop_top_flush (popserver server)`
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_top_flush (server: popserver) -> c_int
{
  pop_multi_flush (server)
}

// C 673-674: `int pop_multi_first (popserver server, const char *command, char **response)`
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_multi_first (server: popserver, command: *const c_char,
                                          response: *mut *mut c_char) -> c_int
{
  if popserver_bits! (in_multi, *server)
    {
      strcpy (pop_error.as_mut_ptr (),
	      c"Already in multi-line query in pop_multi_first".as_ptr ());
      return -1;
    }

  if sendline (server, command) != 0 || pop_getline (server, response) < 0
    { return -1; }

  if 0 == strncmp (*response, c"-ERR".as_ptr (), 4)
    {
      snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s".as_ptr (), *response);
      return -1;
    }
  else if 0 == strncmp (*response, c"+OK".as_ptr (), 3)
    {
      /* C 695: `for (*response += 3; **response == ' '; (*response)++) /* empty */;` - the
	 port's `while` is that loop with its empty body (Deviation 4).  */
      *response = (*response).offset (3);
      while **response == b' ' as c_char
	{ *response = (*response).offset (1); }
      popserver_bits! (in_multi, *server, true);
      return 0;
    }
  else
    {
      strcpy (pop_error.as_mut_ptr (),
	      c"Unexpected response from server in pop_multi_first".as_ptr ());
      return -1;
    }
}

/*
  Read the next line of data from SERVER and place a pointer to it
  into LINE.  Return -1 on error, 0 if there are no more lines to read
  (i.e., the server has returned a line containing only "."), or a
  positive number indicating the number of bytes in the LINE buffer
  (not including the final null).  The data in that buffer may contain
  embedded nulls, but does not contain the final CRLF. When returning
  0, LINE is set to null. */
// C 707-714: C's comment above; C 716-717's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_multi_next (server: popserver, line: *mut *mut c_char) -> c_int
{
  let mut fromserver: *mut c_char = std::ptr::null_mut ();
  let mut ret: c_int;

  if ! popserver_bits! (in_multi, *server)
    {
      strcpy (pop_error.as_mut_ptr (), c"Not in multi-line query in pop_multi_next".as_ptr ());
      return -1;
    }

  ret = pop_getline (server, &mut fromserver);
  if ret < 0
    { return -1; }

  if *fromserver.offset (0) == b'.' as c_char
    {
      if ! (*fromserver.offset (1) != 0)
	{
	  *line = std::ptr::null_mut ();
	  popserver_bits! (in_multi, *server, false);
	  return 0;
	}
      else
	{
	  *line = fromserver.offset (1);
	  return ret - 1;
	}
    }
  else
    {
      *line = fromserver;
      return ret;
    }
}

// C 755-756: `int pop_multi_flush (popserver server)`
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_multi_flush (server: popserver) -> c_int
{
  let mut line: *mut c_char = std::ptr::null_mut ();
  let mut ret: c_int;

  if ! popserver_bits! (in_multi, *server)
    { return 0; }

  loop
    {
      /* C 766-770: `while ((ret = pop_multi_next (server, &line)))` / `if (ret < 0) return -1`.  */
      ret = pop_multi_next (server, &mut line);
      if ! (ret != 0) { break; }
      if ret < 0
	{ return -1; }
    }

  0
}

/* Function: pop_delete (C 775-785)
 *
 * Purpose: Delete a specified message.
 *
 * Arguments: server, message.
 *
 * Return value: 0 on success, non-zero with error in pop_error otherwise.
 */
// C 775-785: C's doc comment; C 786-787's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_delete (server: popserver, message: c_int) -> c_int
{
  if popserver_bits! (in_multi, *server)
    {
      strcpy (pop_error.as_mut_ptr (), c"In multi-line query in pop_delete".as_ptr ());
      return -1;
    }

  sprintf (pop_error.as_mut_ptr (), c"DELE %d".as_ptr (), message);

  if sendline (server, pop_error.as_mut_ptr ()) != 0 || getok (server) != 0
    { return -1; }

  0
}

/*
 * Function: pop_noop (C 803-815)
 *
 * Purpose: Send a noop command to the server.
 *
 * Argument: server.
 *
 * Return value: 0 on success, non-zero with error in pop_error otherwise.
 *
 * Side effects: Closes connection on error.
 */
// C 803-815: C's doc comment; C 816-817's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_noop (server: popserver) -> c_int
{
  if popserver_bits! (in_multi, *server)
    {
      strcpy (pop_error.as_mut_ptr (), c"In multi-line query in pop_noop".as_ptr ());
      return -1;
    }

  if sendline (server, c"NOOP".as_ptr ()) != 0 || getok (server) != 0
    { return -1; }

  0
}

/*
 * Function: pop_last (C 831-844)
 *
 * Purpose: Find out the highest seen message from the server.
 *
 * Arguments: server.
 *
 * Return value: If successful, the highest seen message, which is greater than or equal to 0.
 * Otherwise, a negative number with the error explained in pop_error.
 *
 * Side effects: Closes the connection on error.
 */
// C 831-844: C's doc comment; C 845-846's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_last (server: popserver) -> c_int
{
  let mut fromserver: *mut c_char = std::ptr::null_mut ();

  if popserver_bits! (in_multi, *server)
    {
      strcpy (pop_error.as_mut_ptr (), c"In multi-line query in pop_last".as_ptr ());
      return -1;
    }

  if sendline (server, c"LAST".as_ptr ()) != 0
    { return -1; }

  if pop_getline (server, &mut fromserver) < 0
    { return -1; }

  if 0 == strncmp (fromserver, c"-ERR".as_ptr (), 4)
    {
      snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s".as_ptr (), fromserver);
      return -1;
    }
  else if strncmp (fromserver, c"+OK ".as_ptr (), 4) != 0
    {
      strcpy (pop_error.as_mut_ptr (), c"Unexpected response from server in pop_last".as_ptr ());
      pop_trash (server);
      return -1;
    }
  else
    {
      let mut end_ptr: *mut c_char = std::ptr::null_mut ();
      let mut count: c_int;
      *errno () = 0;
      count = strtol (fromserver.offset (4) as *const c_char, &mut end_ptr, 10) as c_int;
      if fromserver.offset (4) == end_ptr || *errno () != 0
	{
	  strcpy (pop_error.as_mut_ptr (),
		  c"Unexpected response from server in pop_last".as_ptr ());
	  pop_trash (server);
	  return -1;
	}
      return count;
    }
}

/*
 * Function: pop_reset (C 889-901)
 *
 * Purpose: Reset the server to its initial connect state
 *
 * Arguments: server.
 *
 * Return value: 0 for success, non-0 with error in pop_error otherwise.
 *
 * Side effects: Closes the connection on error.
 */
// C 889-901: C's doc comment; C 902-903's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_reset (server: popserver) -> c_int
{
  if pop_retrieve_flush (server) != 0
    { return -1; }

  if sendline (server, c"RSET".as_ptr ()) != 0 || getok (server) != 0
    { return -1; }

  0
}

/*
 * Function: pop_quit (C 916-929)
 *
 * Purpose: Quit the connection to the server,
 *
 * Arguments: server.
 *
 * Return value: 0 for success, non-zero otherwise with error in pop_error.
 *
 * Side Effects: The popserver passed in is unusable after this function is called, even if an
 * error occurs.
 */
// C 916-929: C's doc comment; C 930-931's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_quit (server: popserver) -> c_int
{
  let mut ret: c_int = 0;

  if (*server).file >= 0
    {
      if pop_retrieve_flush (server) != 0
	{ ret = -1; }

      if sendline (server, c"QUIT".as_ptr ()) != 0 || getok (server) != 0
	{ ret = -1; }

      close ((*server).file);
    }

  free ((*server).buffer as *mut c_void);
  free (server as *mut c_void);

  ret
}

/* C 956-958: `#ifdef WINDOWSNT` / `static int have_winsock = 0;` / `#endif`  */
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
static mut have_winsock: c_int = 0;

/* C 79-97's KERBEROS headers (C 80 `HAVE_KRB5_H` -> `<krb5.h>`, C 83-93 the V4 chain, C 94
   `HAVE_COM_ERR_H` -> `<com_err.h>`): the types and entries the C 984-999, C 1106-1198 and
   C 1603-1701 arms name.  No mirror carries any of those headers and every selector is `undef`
   in the frozen table, so the arm is carried as text (P-86 ①) and this whole surface is
   UNVERIFIED (Owed 5: no Kerberos SDK here).  */
#[cfg (all (feature = "mail_use_pop", feature = "kerberos5"))]
pub type krb5_error_code = c_int;
#[cfg (all (feature = "mail_use_pop", feature = "kerberos5"))]
pub type krb5_context = *mut c_void;
#[cfg (all (feature = "mail_use_pop", feature = "kerberos5"))]
pub type krb5_auth_context = *mut c_void;
#[cfg (all (feature = "mail_use_pop", feature = "kerberos5"))]
pub type krb5_ccache = *mut c_void;
#[cfg (all (feature = "mail_use_pop", feature = "kerberos5"))]
pub type krb5_principal = *mut c_void;
#[cfg (all (feature = "mail_use_pop", feature = "kerberos5"))]
pub type krb5_pointer = *mut c_void;
#[cfg (all (feature = "mail_use_pop", feature = "kerberos5"))]
#[repr (C)]
pub struct krb5_data { pub length: c_int, pub data: *mut c_char }
#[cfg (all (feature = "mail_use_pop", feature = "kerberos5"))]
#[repr (C)]
pub struct krb5_error
{
  pub err_code: krb5_error_code,
  pub text: krb5_data,                    /* C 1162-1166: `err_ret->text.length`/`.data` */
  pub e_text: *mut *mut c_char,            /* C 1169-1171: `err_ret->e_text` */
}
#[cfg (all (feature = "mail_use_pop", feature = "kerberos5"))]
pub const AP_OPTS_MUTUAL_REQUIRED: c_int = 0;   /* UNVERIFIED (Owed 5) */
#[cfg (all (feature = "mail_use_pop", feature = "kerberos5"))]
pub const FALSE: c_int = 0;
#[cfg (all (feature = "mail_use_pop", feature = "kerberos5"))]
unsafe extern "C" {
  /// C 1112: `krb5_init_context`
  pub fn krb5_init_context (context: *mut krb5_context) -> krb5_error_code;
  /// C 1117/1126: `krb5_auth_con_free` / `krb5_auth_con_init`
  pub fn krb5_auth_con_free (context: krb5_context, auth_context: krb5_auth_context)
    -> krb5_error_code;
  pub fn krb5_auth_con_init (context: krb5_context, auth_context: *mut krb5_auth_context)
    -> krb5_error_code;
  /// C 1119/1176: `krb5_free_context`
  pub fn krb5_free_context (context: krb5_context) -> krb5_error_code;
  /// C 1130: `krb5_cc_default`
  pub fn krb5_cc_default (context: krb5_context, ccache: *mut krb5_ccache) -> krb5_error_code;
  /// C 1134: `krb5_cc_get_principal`
  pub fn krb5_cc_get_principal (context: krb5_context, ccache: krb5_ccache,
                                principal: *mut krb5_principal) -> krb5_error_code;
  /// C 1141: `krb5_sname_to_principal`
  pub fn krb5_sname_to_principal (context: krb5_context, hostname: *const c_char,
                                  sname: *const c_char, unknown: c_int,
                                  principal: *mut krb5_principal) -> krb5_error_code;
  /// C 1146: `krb5_sendauth`
  pub fn krb5_sendauth (context: krb5_context, auth_context: *mut krb5_auth_context,
                        fd: krb5_pointer, appl_version: *mut c_char,
                        client: krb5_principal, server: krb5_principal,
                        ap_req_options: c_int, checksum: *mut c_void, creds: *mut c_void,
                        ccache: krb5_ccache, error: *mut *mut krb5_error,
                        keyblock: *mut c_void, reply: *mut c_void) -> krb5_error_code;
  /// C 1156: `krb5_free_principal`
  pub fn krb5_free_principal (context: krb5_context, principal: krb5_principal);
  /// C 1174: `krb5_free_error`
  pub fn krb5_free_error (context: krb5_context, error: *mut krb5_error);
  /// C 1121/1160: `error_message` (from `<com_err.h>`, C 94's `HAVE_COM_ERR_H`)
  pub fn error_message (code: krb5_error_code) -> *const c_char;
}

/* C 83-93's `<krb.h>` chain (the V4 library, C 103's `#ifndef KERBEROS5` arm).  UNVERIFIED.  */
#[cfg (all (feature = "mail_use_pop", feature = "kerberos", not (feature = "kerberos5")))]
pub type KTEXT = *mut KTEXT_ST;
#[cfg (all (feature = "mail_use_pop", feature = "kerberos", not (feature = "kerberos5")))]
#[repr (C)]
pub struct KTEXT_ST { pub dat: [c_char; 8], pub length: c_ulong }
#[cfg (all (feature = "mail_use_pop", feature = "kerberos", not (feature = "kerberos5")))]
#[repr (C)]
pub struct MSG_DAT { pub app_data: *mut c_char, pub app_length: c_ulong }
#[cfg (all (feature = "mail_use_pop", feature = "kerberos", not (feature = "kerberos5")))]
#[repr (C)]
pub struct CREDENTIALS { pub service: [c_char; 40] }
#[cfg (all (feature = "mail_use_pop", feature = "kerberos", not (feature = "kerberos5")))]
pub type Key_schedule = *mut c_void;
#[cfg (all (feature = "mail_use_pop", feature = "kerberos", not (feature = "kerberos5")))]
pub const KSUCCESS: c_int = 0;                 /* UNVERIFIED (Owed 5) */
#[cfg (all (feature = "mail_use_pop", feature = "kerberos", not (feature = "kerberos5")))]
unsafe extern "C" {
  /// C 1192: `krb_err_txt[rem]` - `<krb.h>`'s message table
  pub static mut krb_err_txt: [*const c_char; 0];
}

/*
 * Function: socket_connection (C 960-973)
 *
 * Purpose: Opens the network connection with the mail host, without doing any sort of I/O with
 * it or anything.
 *
 * Arguments: host, flags.
 *
 * Return value: A file descriptor indicating the connection, or -1 indicating failure, in which
 * case an error has been copied into pop_error.
 */
// C 960-973: C's doc comment; C 974-975's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
unsafe fn socket_connection (host: *mut c_char, flags: c_int) -> c_int
{
  let mut res: *mut addrinfo = std::ptr::null_mut ();
  let mut it: *mut addrinfo = std::ptr::null_mut ();
  let mut ret: c_int;
  let mut servent: *mut servent = std::ptr::null_mut ();
  let mut found_port: c_char = 0;
  let mut service: *const c_char;
  let mut sock: c_int;
  let mut realhost: *mut c_char = std::ptr::null_mut ();
  /* C 984-1000: `#ifdef KERBEROS` / `#ifdef KERBEROS5` - the two arms' locals / `#endif`s.  */
  #[cfg (all (feature = "kerberos", feature = "kerberos5"))]
  let mut rem: krb5_error_code = 0;
  #[cfg (all (feature = "kerberos", feature = "kerberos5"))]
  let mut kcontext: krb5_context = std::ptr::null_mut ();
  #[cfg (all (feature = "kerberos", feature = "kerberos5"))]
  let mut auth_context: krb5_auth_context = std::ptr::null_mut ();
  #[cfg (all (feature = "kerberos", feature = "kerberos5"))]
  let mut ccdef: krb5_ccache = std::ptr::null_mut ();
  #[cfg (all (feature = "kerberos", feature = "kerberos5"))]
  let mut client: krb5_principal = std::ptr::null_mut ();
  #[cfg (all (feature = "kerberos", feature = "kerberos5"))]
  let mut server: krb5_principal = std::ptr::null_mut ();
  #[cfg (all (feature = "kerberos", feature = "kerberos5"))]
  let mut err_ret: *mut krb5_error = std::ptr::null_mut ();
  #[cfg (all (feature = "kerberos", feature = "kerberos5"))]
  let mut cp: *mut c_char;
  #[cfg (all (feature = "kerberos", not (feature = "kerberos5")))]
  let mut ticket: KTEXT = std::ptr::null_mut ();
  #[cfg (all (feature = "kerberos", not (feature = "kerberos5")))]
  let mut msg_data: MSG_DAT = std::mem::zeroed ();
  #[cfg (all (feature = "kerberos", not (feature = "kerberos5")))]
  let mut cred: CREDENTIALS = std::mem::zeroed ();
  #[cfg (all (feature = "kerberos", not (feature = "kerberos5")))]
  let mut schedule: Key_schedule = std::ptr::null_mut ();
  #[cfg (all (feature = "kerberos", not (feature = "kerberos5")))]
  let mut rem: c_int = 0;

  let mut try_count: c_int = 0;
  let mut connect_ok: c_int;

  /* C 1005-1011: `#ifdef WINDOWSNT` / the `WSAStartup` block / `#endif`.  */
  #[cfg (feature = "windowsnt")]
  {
    let mut winsockData: WSADATA = std::mem::zeroed ();
    if WSAStartup (0x101, &mut winsockData) == 0
      { have_winsock = 1; }
  }

  /* "kpop" service is  never used: look for 20060515 to see why */
  /* C 1014-1018: `#ifdef KERBEROS` / the `? :` on `POP_NO_KERBEROS` / `#else` / `#endif`.  */
  #[cfg (feature = "kerberos")]
  { service = if (flags & POP_NO_KERBEROS) != 0 { POP_SERVICE.as_ptr () }
              else { KPOP_SERVICE.as_ptr () }; }
  #[cfg (not (feature = "kerberos"))]
  { service = POP_SERVICE.as_ptr (); }

  /* C 1020: `struct sockaddr_in addr = {.sin_family = AF_INET};` - zero-initialised first
     (PORTING.md 0.1-13, Deviation 3).  */
  let mut addr: sockaddr_in = std::mem::zeroed ();
  addr.sin_family = AF_INET as u8;

  /* C 1022-1032: `#ifdef HESIOD` / the `hes_getservbyname` lookup / `#endif`.  */
  #[cfg (feature = "hesiod")]
  if (flags & POP_NO_HESIOD) == 0
    {
      servent = hes_getservbyname (service, c"tcp".as_ptr ());
      if ! servent.is_null ()
	{
	  addr.sin_port = (*servent).s_port as u16;
	  found_port = 1;
	}
    }
  if found_port == 0
    {
      servent = getservbyname (service, c"tcp".as_ptr ());
      if ! servent.is_null ()
	{
	  addr.sin_port = (*servent).s_port as u16;
	}
      else
	{
	  /* "kpop" service is  never used: look for 20060515 to see why */
	  /* C 1043-1048: `#ifdef KERBEROS` / `htons` / `#else` / `#endif`.  */
	  #[cfg (feature = "kerberos")]
	  { addr.sin_port = htons ((if (flags & POP_NO_KERBEROS) != 0 { POP_PORT }
				    else { KPOP_PORT }) as c_ushort); }
	  #[cfg (not (feature = "kerberos"))]
	  { addr.sin_port = htons (POP_PORT as c_ushort); }
	}
    }

  /* C 1052: `#define POP_SOCKET_ERROR "Could not create socket for POP connection: "`  */
  const POP_SOCKET_ERROR: &core::ffi::CStr = c"Could not create socket for POP connection: ";

  sock = socket (PF_INET, SOCK_STREAM, 0);
  if sock < 0
    {
      snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s%s".as_ptr (),
		POP_SOCKET_ERROR.as_ptr (), strerror (*errno ()));
      return -1;
    }

  /* C 1063-1068: `struct addrinfo hints = { .ai_socktype = SOCK_STREAM, .ai_flags =
     AI_CANONNAME, .ai_family = AF_INET };` (zero-initialised first, Deviation 3).  */
  let mut hints: addrinfo = std::mem::zeroed ();
  hints.ai_socktype = SOCK_STREAM;
  hints.ai_flags = AI_CANONNAME;
  hints.ai_family = AF_INET;
  loop
    {
      /* C 1069-1078: `do { … } while (ret != 0);`  */
      #[cfg (not (feature = "windowsnt"))]
      { ret = getaddrinfo (host as *const c_char, service, &hints, &mut res); }
      #[cfg (feature = "windowsnt")]
      { ret = sys_getaddrinfo (host as *const c_char, service, &hints, &mut res); }
      try_count += 1;
      if ret != 0 && (ret != EAI_AGAIN || try_count == 5)
	{
	  strcpy (pop_error.as_mut_ptr (), c"Could not determine POP server's address".as_ptr ());
	  return -1;
	}
      if ret == 0 { break; }
    }

  /* C 1080-1087: the `for (it = res; it; it = it->ai_next)` connect loop.  */
  it = res;
  while ! it.is_null ()
    {
      if (*it).ai_addrlen as usize == std::mem::size_of::<sockaddr_in> ()
	{
	  let in_a: *mut sockaddr_in = (*it).ai_addr as *mut sockaddr_in;
	  addr.sin_addr = (*in_a).sin_addr;
	  if connect (sock, &addr as *const sockaddr_in as *mut sockaddr,
		      std::mem::size_of::<sockaddr_in> () as socklen_t) == 0
	    { break; }
	}
      it = (*it).ai_next;
    }
  connect_ok = if ! it.is_null () { 1 } else { 0 };
  if connect_ok != 0
    {
      /* C 1091-1092: `realhost = alloca (strlen (it->ai_canonname) + 1);` - the port's stack
	 pool (P-05; the declaration is at C 70's section above).  */
      realhost = sa_stack_alloc (strlen ((*it).ai_canonname) + 1) as *mut c_char;
      strcpy (realhost, (*it).ai_canonname);
    }
  /* C 1094: `freeaddrinfo (res);` (C 39-42's `#define freeaddrinfo sys_freeaddrinfo`).  */
  #[cfg (not (feature = "windowsnt"))]
  { freeaddrinfo (res); }
  #[cfg (feature = "windowsnt")]
  { sys_freeaddrinfo (res); }

  /* C 1096: `#define CONNECT_ERROR "Could not connect to POP server: "`  */
  const CONNECT_ERROR: &core::ffi::CStr = c"Could not connect to POP server: ";

  if connect_ok == 0
    {
      CLOSESOCKET! (sock);
      snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s%s".as_ptr (),
		CONNECT_ERROR.as_ptr (), strerror (*errno ()));
      return -1;
    }

  /* C 1106-1198: `#ifdef KERBEROS` … the KPOP authentication … `#endif`.  The arm is text-only
     here (no Kerberos SDK, Owed 5) and C's `goto krb5error` is written as the flag-and-tail
     state machine PORTING.md D9 asks for (Deviation 9).  */
  #[cfg (feature = "kerberos")]
  {
    /* C 1108: `#define KRB_ERROR "Kerberos error connecting to POP server: "`  */
    const KRB_ERROR: &core::ffi::CStr = c"Kerberos error connecting to POP server: ";
    if (flags & POP_NO_KERBEROS) == 0
      {
	#[cfg (feature = "kerberos5")]
	{
	  let mut krb5_failed: bool = false;
	  rem = krb5_init_context (&mut kcontext);
	  if rem != 0 { krb5_failed = true; }
	  if ! krb5_failed
	    {
	      rem = krb5_auth_con_init (kcontext, &mut auth_context);
	      if rem != 0 { krb5_failed = true; }
	    }
	  if ! krb5_failed
	    {
	      rem = krb5_cc_default (kcontext, &mut ccdef);
	      if rem != 0 { krb5_failed = true; }
	    }
	  if ! krb5_failed
	    {
	      rem = krb5_cc_get_principal (kcontext, ccdef, &mut client);
	      if rem != 0 { krb5_failed = true; }
	    }
	  if ! krb5_failed
	    {
	      cp = realhost;
	      while *cp != 0
		{
		  *cp = c_tolower (*cp as c_int) as c_char;
		  cp = cp.offset (1);
		}
	      rem = krb5_sname_to_principal (kcontext, realhost, POP_SERVICE.as_ptr (), FALSE,
					     &mut server);
	      if rem != 0 { krb5_failed = true; }
	    }
	  if ! krb5_failed
	    {
	      rem = krb5_sendauth (kcontext, &mut auth_context,
				   &raw mut sock as krb5_pointer,
				   c"KPOPV1.0".as_ptr () as *mut c_char, client, server,
				   AP_OPTS_MUTUAL_REQUIRED,
				   std::ptr::null_mut (),	/* no checksum */
				   std::ptr::null_mut (),	/* no creds, use ccache instead */
				   ccdef, &mut err_ret,
				   std::ptr::null_mut (),	/* don't need subsession key */
				   std::ptr::null_mut ());	/* don't need reply */
	      krb5_free_principal (kcontext, server);
	      if rem != 0 { krb5_failed = true; }
	    }
	  if krb5_failed
	    {
	      /* `krb5error:` - where C 1115/1128/1132/1136/1144 jump.  */
	      if ! auth_context.is_null () { krb5_auth_con_free (kcontext, auth_context); }
	      if ! kcontext.is_null () { krb5_free_context (kcontext); }
	      snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s%s".as_ptr (),
			KRB_ERROR.as_ptr (), error_message (rem));
	      if rem != 0 && ! err_ret.is_null ()
		{
		  let mut pop_error_len: c_int =
		    snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s%s".as_ptr (),
			      KRB_ERROR.as_ptr (), error_message (rem));
		  /* C 1161-1172: the two arms' server text (both `undef` here).  */
		  #[cfg (feature = "have_krb5_error_text")]
		  if ! err_ret.is_null () && (*err_ret).text.length != 0
		    {
		      let errlen: c_int = (*err_ret).text.length;
		      snprintf (pop_error.as_mut_ptr ().offset (pop_error_len as isize),
				ERROR_MAX - pop_error_len as usize, c" [server says '%.*s']".as_ptr (),
				errlen, (*err_ret).text.data);
		    }
		  #[cfg (feature = "have_krb5_error_e_text")]
		  if ! err_ret.is_null () && ! (*err_ret).e_text.is_null ()
		      && *(*err_ret).e_text != std::ptr::null_mut ()
		      && **(*err_ret).e_text != 0
		    {
		      snprintf (pop_error.as_mut_ptr ().offset (pop_error_len as isize),
				ERROR_MAX - pop_error_len as usize, c" [server says '%s']".as_ptr (),
				*(*err_ret).e_text);
		    }
		  let _ = &mut pop_error_len;
		}
	      if ! err_ret.is_null () { krb5_free_error (kcontext, err_ret); }
	      krb5_auth_con_free (kcontext, auth_context);
	      krb5_free_context (kcontext);

	      CLOSESOCKET! (sock);
	      return -1;
	    }
	}
	#[cfg (not (feature = "kerberos5"))]
	{
	  ticket = malloc (std::mem::size_of::<KTEXT_ST> ()) as KTEXT;
	  rem = krb_sendauth (0, sock, ticket, c"pop".as_ptr () as *mut c_char, realhost,
			      krb_realmofhost (realhost), 0, &mut msg_data, &mut cred, schedule,
			      std::ptr::null_mut (), std::ptr::null_mut (),
			      c"KPOPV0.1".as_ptr () as *mut c_char);
	  free (ticket as *mut c_void);
	  if rem != KSUCCESS
	    {
	      snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s%s".as_ptr (),
			KRB_ERROR.as_ptr (), *krb_err_txt.as_ptr ().offset (rem as isize));
	      CLOSESOCKET! (sock);
	      return -1;
	    }
	}
      }
  }

  sock
}

/*
 * Function: pop_getline (C 1203-1225)
 *
 * Purpose: Get a line of text from the connection and return a pointer to it.  The carriage
 * return and linefeed at the end of the line are stripped, but periods at the beginnings of
 * lines are NOT dealt with in any special way.
 *
 * Arguments: server.
 *
 * Returns: The number of characters in the line, which is returned in LINE, not including the
 * final null.  A return value of 0 indicates a blank line.  A negative return value indicates an
 * error (in which case the contents of LINE are undefined.  In case of error, an error message is
 * copied into pop_error.
 *
 * Notes: The line returned is overwritten with each call to pop_getline.
 *
 * Side effects: Closes the connection on error.
 *
 * THE RETURNED LINE MAY CONTAIN EMBEDDED NULLS!
 */
// C 1203-1225: C's doc comment; C 1226-1227's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
unsafe fn pop_getline (server: popserver, line: *mut *mut c_char) -> c_int
{
  /* C 1229: `#define GETLINE_ERROR "Error reading from server: "`  */
  const GETLINE_ERROR: &core::ffi::CStr = c"Error reading from server: ";

  let mut ret: c_int;
  let mut search_offset: c_int = 0;

  if (*server).data != 0
    {
      let mut cp: *mut c_char =
	find_crlf ((*server).buffer.offset ((*server).buffer_index as isize), (*server).data);
      if ! cp.is_null ()
	{
	  let mut found: c_int;
	  let mut data_used: c_int;

	  found = (*server).buffer_index;
	  data_used = (cp.offset (2).offset_from ((*server).buffer) as c_int) - found;

	  *cp = 0;		/* terminate the string to be returned */
	  (*server).data -= data_used;
	  (*server).buffer_index += data_used;

	  if pop_debug
	    {
	      /* Embedded nulls will truncate this output prematurely, but that's OK because
		 it's just for debugging anyway.  */
	      fprintf (stderr, c"<<< %s\n".as_ptr (), (*server).buffer.offset (found as isize));
	    }
	  *line = (*server).buffer.offset (found as isize);
	  return data_used - 2;
	}
      else
	{
	  memmove ((*server).buffer as *mut c_void,
		   (*server).buffer.offset ((*server).buffer_index as isize) as *const c_void,
		   (*server).data as usize);
	  /* Record the fact that we've searched the data already in
             the buffer for a CRLF, so that when we search below, we
             don't have to search the same data twice.  There's a "-
             1" here to account for the fact that the last character
             of the data we have may be the CR of a CRLF pair, of
             which we haven't read the second half yet, so we may have
             to search it again when we read more data. */
	  search_offset = (*server).data - 1;
	  (*server).buffer_index = 0;
	}
    }
  else
    {
      (*server).buffer_index = 0;
    }

  loop
    {
      /* There's a "- 1" here to leave room for the null that we put
         at the end of the read data below.  We put the null there so
         that find_crlf knows where to stop when we call it. */
      if (*server).data == (*server).buffer_size - 1
	{
	  (*server).buffer_size += GETLINE_INCR;
	  (*server).buffer = realloc ((*server).buffer as *mut c_void,
				      (*server).buffer_size as usize) as *mut c_char;
	  if (*server).buffer.is_null ()
	    {
	      strcpy (pop_error.as_mut_ptr (), c"Out of memory in pop_getline".as_ptr ());
	      pop_trash (server);
	      break;
	    }
	}
      ret = RECV! ((*server).file,
		   (*server).buffer.offset ((*server).data as isize),
		   (*server).buffer_size - (*server).data - 1, 0) as c_int;
      if ret < 0
	{
	  snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s%s".as_ptr (),
		    GETLINE_ERROR.as_ptr (), strerror (*errno ()));
	  pop_trash (server);
	  break;
	}
      else if ret == 0
	{
	  strcpy (pop_error.as_mut_ptr (), c"Unexpected EOF from server in pop_getline".as_ptr ());
	  pop_trash (server);
	  break;
	}
      else
	{
	  let mut cp: *mut c_char;
	  (*server).data += ret;
	  *(*server).buffer.offset ((*server).data as isize) = 0;

	  cp = find_crlf ((*server).buffer.offset (search_offset as isize),
			  (*server).data - search_offset);
	  if ! cp.is_null ()
	    {
	      let mut data_used: c_int = cp.offset (2).offset_from ((*server).buffer) as c_int;
	      *cp = 0;
	      (*server).data -= data_used;
	      (*server).buffer_index = data_used;

	      if pop_debug
		{ fprintf (stderr, c"<<< %s\n".as_ptr (), (*server).buffer); }
	      *line = (*server).buffer;
	      return data_used - 2;
	    }
	  /* As above, the "- 1" here is to account for the fact that
	     we may have read a CR without its accompanying LF. */
	  search_offset += ret - 1;
	}
    }

  -1
}

/*
 * Function: sendline (C 1337-1354)
 *
 * Purpose: Sends a line of text to the POP server.  The line of text passed into this function
 * should NOT have the carriage return and linefeed on the end of it.  Periods at beginnings of
 * lines will NOT be treated specially by this function.
 *
 * Arguments: server, line.
 *
 * Return value: Upon successful completion, a value of 0 will be returned.  Otherwise, a
 * non-zero value will be returned, and an error will be copied into pop_error.
 *
 * Side effects: Closes the connection on error.
 */
// C 1337-1354: C's doc comment; C 1355-1356's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
unsafe fn sendline (server: popserver, line: *const c_char) -> c_int
{
  /* C 1358: `#define SENDLINE_ERROR "Error writing to POP server: "`  */
  const SENDLINE_ERROR: &core::ffi::CStr = c"Error writing to POP server: ";
  let mut ret: c_int;
  let mut buf: *mut c_char;

  /* Combine the string and the CR-LF into one buffer.  Otherwise, two
     reasonable network stack optimizations, Nagle's algorithm and
     delayed acks, combine to delay us a fraction of a second on every
     message we send.  (Movemail writes line without \r\n, client
     kernel sends packet, server kernel delays the ack to see if it
     can combine it with data, movemail writes \r\n, client kernel
     waits because it has unacked data already in its outgoing queue,
     client kernel eventually times out and sends.)

     This can be something like 0.2s per command, which can add up
     over a few dozen messages, and is a big chunk of the time we
     spend fetching mail from a server close by.  */
  buf = sa_stack_alloc (strlen (line) + 3) as *mut c_char;
  strcpy (stpcpy (buf, line), c"\r\n".as_ptr ());
  ret = fullwrite ((*server).file, buf, strlen (buf) as c_int);

  if ret < 0
    {
      pop_trash (server);
      snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s%s".as_ptr (),
		SENDLINE_ERROR.as_ptr (), strerror (*errno ()));
      return ret;
    }

  if pop_debug
    { fprintf (stderr, c">>> %s\n".as_ptr (), line); }

  0
}

/*
 * Procedure: fullwrite (C 1391-1398)
 *
 * Purpose: Just like write, but keeps trying until the entire string has been written.
 *
 * Return value: Same as write.  Pop_error is not set.
 */
// C 1391-1398: C's doc comment; C 1399-1400's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
unsafe fn fullwrite (fd: c_int, buf: *mut c_char, mut nbytes: c_int) -> c_int
{
  let mut cp: *mut c_char;
  let mut ret: c_int = 0;

  cp = buf;
  loop
    {
      /* C 1406-1410: `while (nbytes && ((ret = SEND (fd, cp, nbytes, 0)) > 0))`.  */
      if ! (nbytes != 0) { break; }
      ret = SEND! (fd, cp, nbytes, 0);
      if ! (ret > 0) { break; }
      cp = cp.offset (ret as isize);
      nbytes -= ret;
    }

  ret
}

/*
 * Procedure getok (C 1415-1428)
 *
 * Purpose: Reads a line from the server.  If the return indicator is positive, return with a zero
 * exit status.  If not, return with a negative exit status.
 *
 * Arguments: server.
 *
 * Returns: 0 for success, else for failure and puts error in pop_error.
 *
 * Side effects: On failure, may make the connection unusable.
 */
// C 1415-1428: C's doc comment; C 1429-1430's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
unsafe fn getok (server: popserver) -> c_int
{
  let mut fromline: *mut c_char = std::ptr::null_mut ();

  if pop_getline (server, &mut fromline) < 0
    { return -1; }

  if 0 == strncmp (fromline, c"+OK".as_ptr (), 3)
    { return 0; }
  else if 0 == strncmp (fromline, c"-ERR".as_ptr (), 4)
    {
      snprintf (pop_error.as_mut_ptr (), ERROR_MAX, c"%s".as_ptr (), fromline);
      return -1;
    }
  else
    {
      strcpy (pop_error.as_mut_ptr (),
	      c"Unexpected response from server; expecting +OK or -ERR".as_ptr ());
      pop_trash (server);
      return -1;
    }
}

/* C 1455-1485 is `#if 0`: a stand-alone `gettermination`, in K&R style - dead text in every
   configuration, carried as C's own text (the cm.rs precedent, Deviation 6).

#if 0
/*
 * Function: gettermination
 *
 * Purpose: Gets the next line and verifies that it is a termination
 * \tline (nothing but a dot).
 *
 * Return value: 0 on success, non-zero with pop_error set on error.
 *
 * Side effects: Closes the connection on error.
 */
static int
gettermination (server)
     popserver server;
{
  char *fromserver;

  if (pop_getline (server, &fromserver) < 0)
    return (-1);

  if (strcmp (fromserver, "."))
    {
      strcpy (pop_error,
\t      "Unexpected response from server in gettermination");
      pop_trash (server);
      return (-1);
    }

  return (0);
}
#endif
*/

/*
 * Function pop_close (C 1487-1498)
 *
 * Purpose: Close a pop connection, sending a "RSET" command to try to preserve any changes that
 * were made and a "QUIT" command to try to get the server to quit, but ignoring any responses
 * that are received.
 *
 * Side effects: The server is unusable after this function returns.  Changes made to the
 * maildrop since the session was started (or since the last pop_reset) may be lost.
 */
// C 1487-1498: C's doc comment; C 1499-1500's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pop_close (server: popserver)
{
  pop_trash (server);
  free (server as *mut c_void);

  return;
}

/*
 * Function: pop_trash (C 1508-1514)
 *
 * Purpose: Like pop_close or pop_quit, but doesn't deallocate the memory associated with the
 * server.  It is valid to call pop_close or pop_quit after this function has been called.
 */
// C 1508-1514: C's doc comment; C 1515-1516's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
unsafe fn pop_trash (server: popserver)
{
  if (*server).file >= 0
    {
      /* avoid recursion; sendline can call pop_trash */
      if popserver_bits! (trash_started, *server)
	{ return; }
      popserver_bits! (trash_started, *server, true);

      sendline (server, c"RSET".as_ptr ());
      sendline (server, c"QUIT".as_ptr ());

      CLOSESOCKET! ((*server).file);
      (*server).file = -1;
      if ! (*server).buffer.is_null ()
	{
	  free ((*server).buffer as *mut c_void);
	  (*server).buffer = std::ptr::null_mut ();
	}
    }

  /* C 1537-1540: `#ifdef WINDOWSNT` / `if (have_winsock) WSACleanup ();` / `#endif`.  */
  #[cfg (feature = "windowsnt")]
  if have_winsock != 0
    { WSACleanup (); }
}

/* Return a pointer to the first CRLF in IN_STRING, which can contain
   embedded nulls and has LEN characters in it not including the final
   null, or 0 if it does not contain one.  */
// C 1543-1546: C's comment above; C 1547-1548's K&R header line is the signature below.
#[cfg (feature = "mail_use_pop")]
unsafe fn find_crlf (mut in_string: *mut c_char, mut len: c_int) -> *mut c_char
{
  loop
    {
      /* C 1550-1559: `while (len--) { … }` - the test reads LEN's old value and decrements it
	 (Deviation 4).  */
      let t: c_int = len;
      len -= 1;
      if t == 0 { break; }
      if *in_string == b'\r' as c_char
	{
	  in_string = in_string.offset (1);
	  if *in_string == b'\n' as c_char
	    { return in_string.offset (-1); }
	}
      else
	{ in_string = in_string.offset (1); }
    }
  std::ptr::null_mut ()
}

/* --- C 1563-1702: `#ifdef WINDOWSNT` … the getaddrinfo/freeaddrinfo fallbacks … `#endif`.  The
   frozen configuration leaves `WINDOWSNT` undef, so this arm is text-only here (no Windows
   toolchain; Owed 5).  ------------------------------------------------------------------ */

#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub static mut pfn_getaddrinfo: Option<unsafe extern "C" fn (*const c_char, *const c_char,
                                                             *const addrinfo,
                                                             *mut *mut addrinfo) -> c_int>
  = None;                                    // C 1567-1569
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub static mut pfn_freeaddrinfo: Option<unsafe extern "C" fn (*mut addrinfo)> = None; // C 1570

// C 1564-1565: "The following 2 functions are only available since XP, so we load them
// dynamically and provide fallbacks."  C 1572-1600's `load_ws2` (the `static int ws2_loaded`
// function-local static, the `LoadLibrary`/`GetProcAddress` pair and the paranoia swap) is
// below at its own C position; the `WINAPI` calling convention on the two pointers is the
// `extern "C"` above.
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
unsafe fn load_ws2 () -> c_int
{
  static mut ws2_loaded: c_int = 0;

  if ws2_loaded == 0
    {
      let ws2_lib: HANDLE = LoadLibraryA (c"Ws2_32.dll".as_ptr ());

      if ! ws2_lib.is_null ()
	{
	  ws2_loaded = 1;
	  /* C 1584-1585: `pfn_getaddrinfo = (void *) GetProcAddress (ws2_lib, "getaddrinfo");`
	     and the same for `freeaddrinfo` - C's cast from `FARPROC`, with the null test turned
	     into the `Option` the two statics carry.  */
	  let g = GetProcAddress (ws2_lib, c"getaddrinfo".as_ptr ());
	  let f = GetProcAddress (ws2_lib, c"freeaddrinfo".as_ptr ());
	  pfn_getaddrinfo = if g.is_null () { None }
	    else { Some (std::mem::transmute::<*mut c_void,
			  unsafe extern "C" fn (*const c_char, *const c_char, *const addrinfo,
						 *mut *mut addrinfo) -> c_int> (g)) };
	  pfn_freeaddrinfo = if f.is_null () { None }
	    else { Some (std::mem::transmute::<*mut c_void,
			  unsafe extern "C" fn (*mut addrinfo)> (f)) };
	  /* Paranoia: these two functions should go together, so if
	     one is absent, we cannot use the other.  */
	  if pfn_getaddrinfo.is_none ()
	    { pfn_freeaddrinfo = None; }
	  else if pfn_freeaddrinfo.is_none ()
	    { pfn_getaddrinfo = None; }
	}
    }
  if ws2_loaded == 0
    {
      *errno () = ENETDOWN;
      return -1;
    }
  0
}

// C 1603-1605: `int sys_getaddrinfo (const char *node, const char *service, const struct
// addrinfo *hints, struct addrinfo **res)` - the definition C 44-45 forward-declares.
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_getaddrinfo (node: *const c_char, service: *const c_char,
                                          hints: *const addrinfo, res: *mut *mut addrinfo)
  -> c_int
{
  let mut rc: c_int;

  if load_ws2 () != 0
    {
      *errno () = ENETDOWN;
      return WSANO_RECOVERY;
    }

  if let Some (pfn) = pfn_getaddrinfo
    { rc = pfn (node, service, hints, res); }
  else
    {
      let mut port: c_int = 0;
      let mut host_info: *mut hostent;
      /* C 1621-1624: the function-local `struct gai_storage { struct addrinfo addrinfo;
	 struct sockaddr_in sockaddr_in; }` (its Windows layout is UNVERIFIED, Owed 5).  */
      #[repr (C)]
      struct gai_storage
      {
	addrinfo: addrinfo,
	sockaddr_in: sockaddr_in,
      }
      let mut gai_storage: *mut gai_storage;

      /* We don't support any flags besides AI_CANONNAME.  */
      if ! hints.is_null () && ((*hints).ai_flags & ! (AI_CANONNAME)) != 0
	{ return WSAEINVAL; }
      /* NODE cannot be NULL, since pop.c has fallbacks for that.  */
      if node.is_null ()
	{ return WSAHOST_NOT_FOUND; }

      if ! service.is_null ()
	{
	  let protocol: *const c_char =
	    if ! hints.is_null () && (*hints).ai_socktype == SOCK_DGRAM
	      { c"udp".as_ptr () } else { c"tcp".as_ptr () };
	  let srv: *mut servent = getservbyname (service, protocol);

	  if ! srv.is_null ()
	    { port = (*srv).s_port; }
	  else
	    { return WSAHOST_NOT_FOUND; }
	}

      gai_storage = calloc (1, std::mem::size_of::<gai_storage> ()) as *mut gai_storage;
      (*gai_storage).sockaddr_in.sin_port = port as u16;
      host_info = gethostbyname (node);
      if ! host_info.is_null ()
	{
	  memcpy (&raw mut (*gai_storage).sockaddr_in.sin_addr as *mut c_void,
		  *(*host_info).h_addr_list as *const c_void, (*host_info).h_length as usize);
	  (*gai_storage).sockaddr_in.sin_family = (*host_info).h_addrtype as u8;
	}
      else
	{
	  free (gai_storage as *mut c_void);
	  return WSAHOST_NOT_FOUND;
	}

      (*gai_storage).addrinfo.ai_addr =
	&raw mut (*gai_storage).sockaddr_in as *mut sockaddr;
      (*gai_storage).addrinfo.ai_addrlen = std::mem::size_of::<sockaddr_in> () as socklen_t;
      if ! hints.is_null () && ((*hints).ai_flags & AI_CANONNAME) != 0
	{
	  (*gai_storage).addrinfo.ai_canonname = strdup ((*host_info).h_name);
	  if (*gai_storage).addrinfo.ai_canonname.is_null ()
	    {
	      free (gai_storage as *mut c_void);
	      return WSA_NOT_ENOUGH_MEMORY;
	    }
	}
      (*gai_storage).addrinfo.ai_protocol = if ! hints.is_null () { (*hints).ai_protocol } else { 0 };
      (*gai_storage).addrinfo.ai_socktype = if ! hints.is_null () { (*hints).ai_socktype } else { 0 };
      (*gai_storage).addrinfo.ai_family = (*gai_storage).sockaddr_in.sin_family as c_int;
      (*gai_storage).addrinfo.ai_next = std::ptr::null_mut ();

      *res = &mut (*gai_storage).addrinfo;
      rc = 0;
    }

  rc
}

// C 1684-1685: `void sys_freeaddrinfo (struct addrinfo *ai)`
#[cfg (all (feature = "mail_use_pop", feature = "windowsnt"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_freeaddrinfo (ai: *mut addrinfo)
{
  if load_ws2 () != 0
    {
      *errno () = ENETDOWN;
      return;
    }

  if let Some (pfn) = pfn_freeaddrinfo
    { pfn (ai); }
  else
    {
      if ! (*ai).ai_canonname.is_null ()
	{ free ((*ai).ai_canonname as *mut c_void); }
      free (ai as *mut c_void);
    }
}

// C 1702: `#endif	/* WINDOWSNT */`
// C 1703: `#endif /* MAIL_USE_POP */`
