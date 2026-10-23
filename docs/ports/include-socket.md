# emacs/nt/inc/sys/socket.h (287 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 66
- [x] EADDRINUSE  (line 197, body af75181983a6)
- [x] EADDRNOTAVAIL  (line 201, body 0676eef4e8ec)
- [x] EAFNOSUPPORT  (line 193, body 2089cac38aa0)
- [x] EALREADY  (line 153, body a18d2bed12cf)
- [x] ECONNABORTED  (line 217, body 88cc28d30a9a)
- [x] ECONNREFUSED  (line 249, body 12ceb6571f14)
- [x] ECONNRESET  (line 221, body daaed2a76d06)
- [x] EDESTADDRREQ  (line 161, body 0d5c4898688d)
- [x] EDQUOT  (line 275, body 6978d040319e)
- [x] EHOSTDOWN  (line 258, body 8110587299cf)
- [x] EHOSTUNREACH  (line 262, body 0eb5423e9edb)
- [x] EINPROGRESS  (line 149, body e4dd1f131a35)
- [x] EISCONN  (line 229, body 820163b851bb)
- [x] ELOOP  (line 253, body bfde79e98ef1)
- [x] EMSGSIZE  (line 165, body 05972a3df0c4)
- [x] ENETDOWN  (line 205, body 5ffdf7f25182)
- [x] ENETRESET  (line 213, body f7ddceccd9ea)
- [x] ENETUNREACH  (line 209, body 77575eded39e)
- [x] ENOBUFS  (line 225, body e5a97a09e7b9)
- [x] ENOPROTOOPT  (line 173, body b9fda0d509ad)
- [x] ENOTCONN  (line 233, body cf9dfb8ce96d)
- [x] ENOTSOCK  (line 157, body 51abd0b30380)
- [x] EOPNOTSUPP  (line 185, body 514288d37890)
- [x] EPFNOSUPPORT  (line 189, body 9fb482606489)
- [x] EPROCLIM  (line 267, body 18a859f45354)
- [x] EPROTONOSUPPORT  (line 177, body 297825135ac1)
- [x] EPROTOTYPE  (line 169, body 51ca8de66e52)
- [x] EREMOTE  (line 283, body 41fb1812b56a)
- [x] ESHUTDOWN  (line 237, body 02a278f49cae)
- [x] ESOCKTNOSUPPORT  (line 181, body 1b8efd9e0493)
- [x] ESTALE  (line 279, body 0a0abbb9afba)
- [x] ETIMEDOUT  (line 245, body d07d682527eb)
- [x] ETOOMANYREFS  (line 241, body e717c7ce2389)
- [x] EUSERS  (line 271, body 44155d47f6d1)
- [x] EWOULDBLOCK  (line 145, body fc78f0aeaa34)
- [x] FD_CLOEXEC  (line 135, body 356a192b7913)
- [x] F_DUPFD_CLOEXEC  (line 134, body 6b7093bcc3fe)
- [x] F_SETFD  (line 131, body da4b9237bacc)
- [x] F_SETFL  (line 130, body 1b6453892473)
- [x] MUST_REDEF_SELECT  (line 35, body da39a3ee5e6b)
- [x] O_CLOEXEC  (line 133, body 017373a3c63c)
- [x] O_NONBLOCK  (line 132, body 5967c2cac272)
- [x] _SOCKET_H_  (line 22, body da39a3ee5e6b)
- [x] accept  (line 96, body d4871569b497)
- [x] bind  (line 81, body 0f2ea60e844c)
- [x] connect  (line 82, body 53bb260783d1)
- [x] fd_set  (line 75, body 9db843f926ca)
- [x] freeaddrinfo  (line 100, body 2e1baa6be6d0)
- [x] getaddrinfo  (line 99, body 1d9053ea59c3)
- [x] gethostbyname  (line 89, body 2ebf9964fb26)
- [x] gethostname  (line 88, body 7e9bc5f3f8f6)
- [x] getpeername  (line 90, body 6c1eb543fc87)
- [x] getservbyname  (line 91, body aee09a0a9f91)
- [x] getsockname  (line 95, body d14100941ebf)
- [x] htonl  (line 85, body 3ec702605046)
- [x] htons  (line 83, body 811f78659a26)
- [x] inet_addr  (line 87, body 7b296dd3849c)
- [x] listen  (line 94, body 17e2ecfe06a7)
- [x] ntohl  (line 86, body a340ce6f5d48)
- [x] ntohs  (line 84, body 63a02fef359c)
- [x] recvfrom  (line 97, body 79ae52fad8fb)
- [x] select  (line 62, body 890de6aab39c)
- [x] sendto  (line 98, body a373469ffcba)
- [x] setsockopt  (line 93, body 6c657eec0c52)
- [x] shutdown  (line 92, body 595abdc425e3)
- [x] socket  (line 80, body de6551f0af97)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern int h_errno  (line 139)
## Decisions

Mirror of nt/inc/sys/socket.h, complete for this target: the include guard and
`MUST_REDEF_SELECT`, the `fd_set` type rename, the 21 socket calls as `macro_rules!` renames, the
`fcntl` flags, all 35 winsock-error renames and the `sys_*`/`fcntl` prototypes plus `h_errno` in
`socket_decls`.

**Which arm**: the file is not on `docs/baseline/config/cond-eval-verified.txt`, so no `#if` in it
is resolved - the `__MINGW32_VERSION` and `EMACS_CONFIG_H` arms are all live text, and every name
is defined exactly once.  The preprocessor manipulation (`#undef _WINSOCKAPI_`, `#undef select`
with `MUST_REDEF_SELECT`, the four `#undef FD_*` in C 39-44 and 68-72) has no Rust counterpart and
is recorded in the head note.

**Names with no mirror**: the whole winsock/CRT side of every rename - `WSA*`, `ws_fd_set`
(`src/w32.h`'s own `fd_set`, C 76), `O_NOINHERIT`, and the types the prototypes take (`u_short`,
`u_long`, `struct sockaddr`, `struct hostent`, `struct servent`, `struct addrinfo`).  They stay
spelled the way C spells them and are named from the including unit, as `nt/inc/sys/include/acl.rs`
does for `ssize_t`.  `fd_set` itself is a type rename, so it is a `pub type` with a row this round
appends to `docs/ports/excluded-macros.txt`.

**One spelling the constant check does not accept**: C 132's `O_NONBLOCK` is octal `04000`.  The
mirror writes `0x800` (the same value) because `tools/extract-abi.py`'s constant comparison
normalises C's leading-zero octal but not Rust's `0o` spelling - the value ends up compared as the
string `0o4000` and fails, even though the token scanner (`body_token_seq`) does handle `0o`
(status 3.34).  Reported as a tool gap, not worked around silently.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/sys/socket.h
nt/inc/sys/include/socket.rs` -> `OK` (with the expected note that `fd_set` has a recorded
counterpart).
