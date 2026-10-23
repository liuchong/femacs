// femacs: mirror of nt/inc/sys/socket.h @ fd7afced403.
//
// The Windows port's workable `<sys/socket.h>`, built on `winsock2.h`/`ws2tcpip.h`: it defeats
// the winsock include guards, re-points the socket calls at the port's own `sys_*` wrappers, gives
// `select`/`fd_set` back to the program, supplies the `fcntl` the runtime has not got, and renames
// winsock's error codes to the errno spellings the tree uses.  The file is not on
// `docs/baseline/config/cond-eval-verified.txt`, so the extractor resolves no `#if` in it - every
// runtime-version arm is live text.  Nothing in
// the frozen configuration includes it (no `nt/` translation unit is compiled -
// docs/baseline/frozen-build-sources.txt's batch-F section - and no `src/` unit includes a
// `nt/inc/` header), so `xtask layout-check` never compiles this mirror; in a MinGW build
// `src/process.c`, `src/sysdep.c`, `src/callproc.c`, `src/filelock.c` and `src/w32.c` are among
// the units whose `#include <sys/socket.h>` resolves here.
//
// Names with no mirror (the winsock/CRT side of every rename): `WSA*` (the error codes),
// `ws_fd_set` (the port's own `fd_set`, declared in `src/w32.h`, C 76), `O_NOINHERIT`, and the
// winsock types the prototypes take - `u_short`, `u_long`, `struct sockaddr`, `struct hostent`,
// `struct servent`, `struct addrinfo`.  They stay spelled the way C spells them and are named
// from the including unit (`nt/inc/sys/include/acl.rs`'s `ssize_t` shape); `fd_set` is a type
// rename, so it is a `pub type` whose row this round appends to `docs/ports/excluded-macros.txt`.
//
// The preprocessor manipulation C 25-44 and 61-77 does (`#undef _WINSOCKAPI_`, `#undef select`
// with `MUST_REDEF_SELECT`, `#undef FD_SET`/`FD_CLR`/`FD_ISSET`/`FD_ZERO`) has no Rust
// counterpart: it exists so the winsock headers do not define what C 74-77 then replaces.  The
// two names C 35 and 63 define are carried below; the `#undef`s are recorded here.

// C 21: the include guard has no Rust counterpart (PORTING.md 0.1).
pub const _SOCKET_H_: bool = true;
/// C 35: `#define MUST_REDEF_SELECT` (under `#ifdef select`, and `#undef`ed again at C 63 once
/// C 62 has run).
pub const MUST_REDEF_SELECT: bool = true;

// C 62: `#define select sys_select` (the `MUST_REDEF_SELECT` arm).
macro_rules! select { ($($args:tt)*) => { sys_select ($($args)*) }; }

/// C 75: `#define fd_set ws_fd_set` - a type rename (the row is in
/// `docs/ports/excluded-macros.txt`); `ws_fd_set` is `src/w32.h`'s own declaration, which has no
/// mirror here.
pub type fd_set = ws_fd_set;

// C 80-100: the socket calls, renamed to their `sys_*` entry points.
/// C 80: `#define socket sys_socket`
macro_rules! socket { ($($args:tt)*) => { sys_socket ($($args)*) }; }
/// C 81: `#define bind sys_bind`
macro_rules! bind { ($($args:tt)*) => { sys_bind ($($args)*) }; }
/// C 82: `#define connect sys_connect`
macro_rules! connect { ($($args:tt)*) => { sys_connect ($($args)*) }; }
/// C 83: `#define htons sys_htons`
macro_rules! htons { ($($args:tt)*) => { sys_htons ($($args)*) }; }
/// C 84: `#define ntohs sys_ntohs`
macro_rules! ntohs { ($($args:tt)*) => { sys_ntohs ($($args)*) }; }
/// C 85: `#define htonl sys_htonl`
macro_rules! htonl { ($($args:tt)*) => { sys_htonl ($($args)*) }; }
/// C 86: `#define ntohl sys_ntohl`
macro_rules! ntohl { ($($args:tt)*) => { sys_ntohl ($($args)*) }; }
/// C 87: `#define inet_addr sys_inet_addr`
macro_rules! inet_addr { ($($args:tt)*) => { sys_inet_addr ($($args)*) }; }
/// C 88: `#define gethostname sys_gethostname`
macro_rules! gethostname { ($($args:tt)*) => { sys_gethostname ($($args)*) }; }
/// C 89: `#define gethostbyname sys_gethostbyname`
macro_rules! gethostbyname { ($($args:tt)*) => { sys_gethostbyname ($($args)*) }; }
/// C 90: `#define getpeername sys_getpeername`
macro_rules! getpeername { ($($args:tt)*) => { sys_getpeername ($($args)*) }; }
/// C 91: `#define getservbyname sys_getservbyname`
macro_rules! getservbyname { ($($args:tt)*) => { sys_getservbyname ($($args)*) }; }
/// C 92: `#define shutdown sys_shutdown`
macro_rules! shutdown { ($($args:tt)*) => { sys_shutdown ($($args)*) }; }
/// C 93: `#define setsockopt sys_setsockopt`
macro_rules! setsockopt { ($($args:tt)*) => { sys_setsockopt ($($args)*) }; }
/// C 94: `#define listen sys_listen`
macro_rules! listen { ($($args:tt)*) => { sys_listen ($($args)*) }; }
/// C 95: `#define getsockname sys_getsockname`
macro_rules! getsockname { ($($args:tt)*) => { sys_getsockname ($($args)*) }; }
/// C 96: `#define accept sys_accept`
macro_rules! accept { ($($args:tt)*) => { sys_accept ($($args)*) }; }
/// C 97: `#define recvfrom sys_recvfrom`
macro_rules! recvfrom { ($($args:tt)*) => { sys_recvfrom ($($args)*) }; }
/// C 98: `#define sendto sys_sendto`
macro_rules! sendto { ($($args:tt)*) => { sys_sendto ($($args)*) }; }
/// C 99: `#define getaddrinfo sys_getaddrinfo`
macro_rules! getaddrinfo { ($($args:tt)*) => { sys_getaddrinfo ($($args)*) }; }
/// C 100: `#define freeaddrinfo sys_freeaddrinfo`
macro_rules! freeaddrinfo { ($($args:tt)*) => { sys_freeaddrinfo ($($args)*) }; }

// C 130-135: the `fcntl` flags the port's own `fcntl` (C 129) understands.
/// C 130: `#define F_SETFL 4`
pub const F_SETFL: c_int = 4;
/// C 131: `#define F_SETFD 2`
pub const F_SETFD: c_int = 2;
/// C 132: `#define O_NONBLOCK 04000` - C's octal spelling, kept as Rust's `0o4000`.  The
/// constant check did not read Rust's `0o` prefix when this mirror first landed (the token
/// scanner did; status 3.34), so the line said `0x800` for one round; `tools/extract-abi.py`'s
/// `norm_const` reads the prefix now (status 3.112).
pub const O_NONBLOCK: c_int = 0o4000;
/// C 133: `#define O_CLOEXEC O_NOINHERIT` - the CRT's own name for the flag.
pub const O_CLOEXEC: c_int = O_NOINHERIT;
/// C 134: `#define F_DUPFD_CLOEXEC 0x40000000`
pub const F_DUPFD_CLOEXEC: c_int = 0x40000000;
/// C 135: `#define FD_CLOEXEC 1`
pub const FD_CLOEXEC: c_int = 1;

// C 141-283: winsock's error codes, renamed to the errno spellings the tree tests for
// (`WSAEWOULDBLOCK` and the other `WSA*` names come from `winsock2.h`, which has no mirror).
/// C 145: `#define EWOULDBLOCK WSAEWOULDBLOCK`
pub const EWOULDBLOCK: c_int = WSAEWOULDBLOCK;
/// C 149: `#define EINPROGRESS WSAEINPROGRESS`
pub const EINPROGRESS: c_int = WSAEINPROGRESS;
/// C 153: `#define EALREADY WSAEALREADY`
pub const EALREADY: c_int = WSAEALREADY;
/// C 157: `#define ENOTSOCK WSAENOTSOCK`
pub const ENOTSOCK: c_int = WSAENOTSOCK;
/// C 161: `#define EDESTADDRREQ WSAEDESTADDRREQ`
pub const EDESTADDRREQ: c_int = WSAEDESTADDRREQ;
/// C 165: `#define EMSGSIZE WSAEMSGSIZE`
pub const EMSGSIZE: c_int = WSAEMSGSIZE;
/// C 169: `#define EPROTOTYPE WSAEPROTOTYPE`
pub const EPROTOTYPE: c_int = WSAEPROTOTYPE;
/// C 173: `#define ENOPROTOOPT WSAENOPROTOOPT`
pub const ENOPROTOOPT: c_int = WSAENOPROTOOPT;
/// C 177: `#define EPROTONOSUPPORT WSAEPROTONOSUPPORT`
pub const EPROTONOSUPPORT: c_int = WSAEPROTONOSUPPORT;
/// C 181: `#define ESOCKTNOSUPPORT WSAESOCKTNOSUPPORT`
pub const ESOCKTNOSUPPORT: c_int = WSAESOCKTNOSUPPORT;
/// C 185: `#define EOPNOTSUPP WSAEOPNOTSUPP`
pub const EOPNOTSUPP: c_int = WSAEOPNOTSUPP;
/// C 189: `#define EPFNOSUPPORT WSAEPFNOSUPPORT`
pub const EPFNOSUPPORT: c_int = WSAEPFNOSUPPORT;
/// C 193: `#define EAFNOSUPPORT WSAEAFNOSUPPORT`
pub const EAFNOSUPPORT: c_int = WSAEAFNOSUPPORT;
/// C 197: `#define EADDRINUSE WSAEADDRINUSE`
pub const EADDRINUSE: c_int = WSAEADDRINUSE;
/// C 201: `#define EADDRNOTAVAIL WSAEADDRNOTAVAIL`
pub const EADDRNOTAVAIL: c_int = WSAEADDRNOTAVAIL;
/// C 205: `#define ENETDOWN WSAENETDOWN`
pub const ENETDOWN: c_int = WSAENETDOWN;
/// C 209: `#define ENETUNREACH WSAENETUNREACH`
pub const ENETUNREACH: c_int = WSAENETUNREACH;
/// C 213: `#define ENETRESET WSAENETRESET`
pub const ENETRESET: c_int = WSAENETRESET;
/// C 217: `#define ECONNABORTED WSAECONNABORTED`
pub const ECONNABORTED: c_int = WSAECONNABORTED;
/// C 221: `#define ECONNRESET WSAECONNRESET`
pub const ECONNRESET: c_int = WSAECONNRESET;
/// C 225: `#define ENOBUFS WSAENOBUFS`
pub const ENOBUFS: c_int = WSAENOBUFS;
/// C 229: `#define EISCONN WSAEISCONN`
pub const EISCONN: c_int = WSAEISCONN;
/// C 233: `#define ENOTCONN WSAENOTCONN`
pub const ENOTCONN: c_int = WSAENOTCONN;
/// C 237: `#define ESHUTDOWN WSAESHUTDOWN`
pub const ESHUTDOWN: c_int = WSAESHUTDOWN;
/// C 241: `#define ETOOMANYREFS WSAETOOMANYREFS`
pub const ETOOMANYREFS: c_int = WSAETOOMANYREFS;
/// C 245: `#define ETIMEDOUT WSAETIMEDOUT`
pub const ETIMEDOUT: c_int = WSAETIMEDOUT;
/// C 249: `#define ECONNREFUSED WSAECONNREFUSED`
pub const ECONNREFUSED: c_int = WSAECONNREFUSED;
/// C 253: `#define ELOOP WSAELOOP`
pub const ELOOP: c_int = WSAELOOP;
/// C 258: `#define EHOSTDOWN WSAEHOSTDOWN`
pub const EHOSTDOWN: c_int = WSAEHOSTDOWN;
/// C 262: `#define EHOSTUNREACH WSAEHOSTUNREACH`
pub const EHOSTUNREACH: c_int = WSAEHOSTUNREACH;
/// C 267: `#define EPROCLIM WSAEPROCLIM`
pub const EPROCLIM: c_int = WSAEPROCLIM;
/// C 271: `#define EUSERS WSAEUSERS`
pub const EUSERS: c_int = WSAEUSERS;
/// C 275: `#define EDQUOT WSAEDQUOT`
pub const EDQUOT: c_int = WSAEDQUOT;
/// C 279: `#define ESTALE WSAESTALE`
pub const ESTALE: c_int = WSAESTALE;
/// C 283: `#define EREMOTE WSAEREMOTE`
pub const EREMOTE: c_int = WSAEREMOTE;

pub mod socket_decls {
  use super::*;

  unsafe extern "C" {
    /// C 102: `int sys_socket(int af, int type, int protocol);`
    pub fn sys_socket (af: c_int, type_: c_int, protocol: c_int) -> c_int;
    /// C 103: `int sys_bind (int s, const struct sockaddr *addr, int namelen);`
    pub fn sys_bind (s: c_int, addr: *const sockaddr, namelen: c_int) -> c_int;
    /// C 104: `int sys_connect (int s, const struct sockaddr *addr, int namelen);`
    pub fn sys_connect (s: c_int, addr: *const sockaddr, namelen: c_int) -> c_int;
    /// C 105: `u_short sys_htons (u_short hostshort);`
    pub fn sys_htons (hostshort: u_short) -> u_short;
    /// C 106: `u_short sys_ntohs (u_short netshort);`
    pub fn sys_ntohs (netshort: u_short) -> u_short;
    /// C 107: `u_long sys_htonl (u_long hostlong);`
    pub fn sys_htonl (hostlong: u_long) -> u_long;
    /// C 108: `u_long sys_ntohl (u_long netlong);`
    pub fn sys_ntohl (netlong: u_long) -> u_long;
    /// C 109: `unsigned long sys_inet_addr (const char * cp);`
    pub fn sys_inet_addr (cp: *const c_char) -> c_ulong;
    /// C 110: `int sys_gethostname (char * name, int namelen);`
    pub fn sys_gethostname (name: *mut c_char, namelen: c_int) -> c_int;
    /// C 111: `struct hostent * sys_gethostbyname (const char * name);`
    pub fn sys_gethostbyname (name: *const c_char) -> *mut hostent;
    /// C 112: `struct servent * sys_getservbyname (const char * name, const char * proto);`
    pub fn sys_getservbyname (name: *const c_char, proto: *const c_char) -> *mut servent;
    /// C 113: `int sys_getpeername (int s, struct sockaddr *addr, int * namelen);`
    pub fn sys_getpeername (s: c_int, addr: *mut sockaddr, namelen: *mut c_int) -> c_int;
    /// C 114: `int sys_shutdown (int socket, int how);`
    pub fn sys_shutdown (socket: c_int, how: c_int) -> c_int;
    /// C 115: `int sys_setsockopt (int s, int level, int oname, const void * oval, int olen);`
    pub fn sys_setsockopt (s: c_int, level: c_int, oname: c_int, oval: *const c_void,
                           olen: c_int) -> c_int;
    /// C 116: `int sys_listen (int s, int backlog);`
    pub fn sys_listen (s: c_int, backlog: c_int) -> c_int;
    /// C 117: `int sys_getsockname (int s, struct sockaddr * name, int * namelen);`
    pub fn sys_getsockname (s: c_int, name: *mut sockaddr, namelen: *mut c_int) -> c_int;
    /// C 118: `int sys_accept (int s, struct sockaddr *addr, int *addrlen);`
    pub fn sys_accept (s: c_int, addr: *mut sockaddr, addrlen: *mut c_int) -> c_int;
    /// C 119-120: `int sys_recvfrom (int s, char *buf, int len, int flags, struct sockaddr
    /// *from, int * fromlen);`
    pub fn sys_recvfrom (s: c_int, buf: *mut c_char, len: c_int, flags: c_int,
                         from: *mut sockaddr, fromlen: *mut c_int) -> c_int;
    /// C 121-122: `int sys_sendto (int s, const char * buf, int len, int flags, const struct
    /// sockaddr *to, int tolen);`
    pub fn sys_sendto (s: c_int, buf: *const c_char, len: c_int, flags: c_int,
                       to: *const sockaddr, tolen: c_int) -> c_int;
    /// C 123-124: `int sys_getaddrinfo (const char * node, const char * service, const struct
    /// addrinfo * hints, struct addrinfo ** res);`
    pub fn sys_getaddrinfo (node: *const c_char, service: *const c_char,
                            hints: *const addrinfo, res: *mut *mut addrinfo) -> c_int;
    /// C 125: `void sys_freeaddrinfo (struct addrinfo * ai);`
    pub fn sys_freeaddrinfo (ai: *mut addrinfo);
    /// C 129: `int fcntl (int s, int cmd, int options);`
    pub fn fcntl (s: c_int, cmd: c_int, options: c_int) -> c_int;
    /// C 139: `extern int h_errno;` - the file's one file-scope object (C 138 `#undef`s the
    /// winsock macro of the same name first, so this is a real variable).
    pub static mut h_errno: c_int;
  }
}
use socket_decls::*;
