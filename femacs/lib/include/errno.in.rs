// femacs: port of lib/errno.in.h @ fd7afced403 (gnulib's <errno.h>; PORTING.md P-26).
//
// This target generates no such header.  gnulib's `errno-h` module substitutes the template
// only when `GL_GENERATE_ERRNO_H_CONDITION` is non-empty, and otherwise removes the file
// (build/emacs-ref/lib/gnulib.mk 2051-2067); the frozen configuration's value is empty, so
// `build/emacs-ref/lib/errno.h` does not exist and the header every unit sees is the system
// `<errno.h>` this template would have `#include_next`ed:
//
//     $ grep -n GL_GENERATE_ERRNO_H_CONDITION build/emacs-ref/config.log build/emacs-ref/lib/gnulib.mk
//     build/emacs-ref/config.log:5238:GL_GENERATE_ERRNO_H_CONDITION=''
//     build/emacs-ref/lib/gnulib.mk:364:GL_GENERATE_ERRNO_H_CONDITION =
//     $ ls build/emacs-ref/lib/errno.h
//     ls: build/emacs-ref/lib/errno.h: No such file or directory
//
// Every name the template defines is therefore dead text here (P-51), and the mirror carries
// both halves the port cannot split with `#include_next`:
//
//   * `errno.rs`: the system half, the 31 codes the ported units name, measured the way that
//     file's own comments record.
//   * the template's own half, below: C 33-148's `# if defined _WIN32 && ! defined __CYGWIN__`
//     block (feature "windows") and C 161-267's `# ifndef X` fallbacks (feature "no_<name>"),
//     each followed by the frozen target's value for that name.  C's arm is written first
//     because `tools/extract-abi.py` compares the first definition of a name while rustc
//     compiles only the ungated one - the fcntl.in.rs shape.
//
// The frozen values are measured in one command (the whole table is in the ledger):
//
//     $ clang -std=gnu23 -E -dM -include errno.h /tmp/empty.c | grep -E '^#define E[A-Z0-9_]+ '
//     #define EADDRINUSE 48      #define EADDRNOTAVAIL 49    #define EAFNOSUPPORT 47
//     #define EALREADY 37        #define EBADMSG 94          #define ECANCELED 89
//     ... ELOOP 62 ... ENOMSG 91 ... ENOTSUP 45 ... ESOCKTNOSUPPORT 44 ...
//     (EOTHER is the one name the frozen <errno.h> does not define at all.)
//
// The guard pair (C 18/269-270) and the prologue (C 20-26: the `__GNUC__ >= 3` test, the
// pragma placeholder and the split guard) have no Rust counterpart (PORTING.md 0.1, P-08).

include!("errno.rs");   // C 26: `# include_next <errno.h>`, i.e. the system half

/* C 28-29: the double-inclusion guard, `_@GUARD_PREFIX@_ERRNO_H`, whose substitution would be
   `_GL_ERRNO_H`.  The template's own spelling reads as `_` to the extractor, which
   docs/ports/excluded-macros.txt records.  */
pub const _GL_ERRNO_H: c_int = 1;

/* ---- C 33-148: `# if defined _WIN32 && ! defined __CYGWIN__` --------------------------- */
/* MSVC 10's values and C 134-146's winsock2 numbers.  Dead on this target: the arm needs
   `_WIN32`, and its fourteen nested `# ifndef X` groups are the same condition, so one
   feature names all of them.  The `GNULIB_defined_*` names inside it are gnulib's own
   markers - this target's `<errno.h>` has no counterpart for any of them - so only C's arm
   is carried.  */
#[cfg (feature = "windows")]
pub const ENOMSG: c_int = 122;   // C 38
#[cfg (feature = "windows")]
pub const GNULIB_defined_ENOMSG: c_int = 1;   // C 39
#[cfg (feature = "windows")]
pub const EIDRM: c_int = 111;   // C 43
#[cfg (feature = "windows")]
pub const GNULIB_defined_EIDRM: c_int = 1;   // C 44
#[cfg (feature = "windows")]
pub const ENOLINK: c_int = 121;   // C 48
#[cfg (feature = "windows")]
pub const GNULIB_defined_ENOLINK: c_int = 1;   // C 49
#[cfg (feature = "windows")]
pub const EPROTO: c_int = 134;   // C 53
#[cfg (feature = "windows")]
pub const GNULIB_defined_EPROTO: c_int = 1;   // C 54
#[cfg (feature = "windows")]
pub const EBADMSG: c_int = 104;   // C 58
#[cfg (feature = "windows")]
pub const GNULIB_defined_EBADMSG: c_int = 1;   // C 59
#[cfg (feature = "windows")]
pub const EOVERFLOW: c_int = 132;   // C 63
#[cfg (feature = "windows")]
pub const GNULIB_defined_EOVERFLOW: c_int = 1;   // C 64
#[cfg (feature = "windows")]
pub const ENOTSUP: c_int = 129;   // C 68
#[cfg (feature = "windows")]
pub const GNULIB_defined_ENOTSUP: c_int = 1;   // C 69
#[cfg (feature = "windows")]
pub const ENETRESET: c_int = 117;   // C 73
#[cfg (feature = "windows")]
pub const GNULIB_defined_ENETRESET: c_int = 1;   // C 74
#[cfg (feature = "windows")]
pub const ECONNABORTED: c_int = 106;   // C 78
#[cfg (feature = "windows")]
pub const GNULIB_defined_ECONNABORTED: c_int = 1;   // C 79
#[cfg (feature = "windows")]
pub const ECANCELED: c_int = 105;   // C 83
#[cfg (feature = "windows")]
pub const GNULIB_defined_ECANCELED: c_int = 1;   // C 84
#[cfg (feature = "windows")]
pub const EOWNERDEAD: c_int = 133;   // C 88
#[cfg (feature = "windows")]
pub const GNULIB_defined_EOWNERDEAD: c_int = 1;   // C 89
#[cfg (feature = "windows")]
pub const ENOTRECOVERABLE: c_int = 127;   // C 93
#[cfg (feature = "windows")]
pub const GNULIB_defined_ENOTRECOVERABLE: c_int = 1;   // C 94
#[cfg (feature = "windows")]
pub const EINPROGRESS: c_int = 112;   // C 98
/* `EINPROGRESS`'s frozen value is `errno.rs`'s.  */
#[cfg (feature = "windows")]
pub const EALREADY: c_int = 103;   // C 99
/// `EALREADY 37` (frozen; this target's `<errno.h>`).
pub const EALREADY: c_int = 37;
#[cfg (feature = "windows")]
pub const ENOTSOCK: c_int = 128;   // C 100
/// `ENOTSOCK 38` (frozen; this target's `<errno.h>`).
pub const ENOTSOCK: c_int = 38;
#[cfg (feature = "windows")]
pub const EDESTADDRREQ: c_int = 109;   // C 101
/// `EDESTADDRREQ 39` (frozen; this target's `<errno.h>`).
pub const EDESTADDRREQ: c_int = 39;
#[cfg (feature = "windows")]
pub const EMSGSIZE: c_int = 115;   // C 102
/* `EMSGSIZE`'s frozen value is `errno.rs`'s.  */
#[cfg (feature = "windows")]
pub const EPROTOTYPE: c_int = 136;   // C 103
/// `EPROTOTYPE 41` (frozen; this target's `<errno.h>`).
pub const EPROTOTYPE: c_int = 41;
#[cfg (feature = "windows")]
pub const ENOPROTOOPT: c_int = 123;   // C 104
/// `ENOPROTOOPT 42` (frozen; this target's `<errno.h>`).
pub const ENOPROTOOPT: c_int = 42;
#[cfg (feature = "windows")]
pub const EPROTONOSUPPORT: c_int = 135;   // C 105
/// `EPROTONOSUPPORT 43` (frozen; this target's `<errno.h>`).
pub const EPROTONOSUPPORT: c_int = 43;
#[cfg (feature = "windows")]
pub const EOPNOTSUPP: c_int = 130;   // C 106
/* `EOPNOTSUPP`'s frozen value is `errno.rs`'s.  */
#[cfg (feature = "windows")]
pub const EAFNOSUPPORT: c_int = 102;   // C 107
/// `EAFNOSUPPORT 47` (frozen; this target's `<errno.h>`).
pub const EAFNOSUPPORT: c_int = 47;
#[cfg (feature = "windows")]
pub const EADDRINUSE: c_int = 100;   // C 108
/// `EADDRINUSE 48` (frozen; this target's `<errno.h>`).
pub const EADDRINUSE: c_int = 48;
#[cfg (feature = "windows")]
pub const EADDRNOTAVAIL: c_int = 101;   // C 109
/// `EADDRNOTAVAIL 49` (frozen; this target's `<errno.h>`).
pub const EADDRNOTAVAIL: c_int = 49;
#[cfg (feature = "windows")]
pub const ENETDOWN: c_int = 116;   // C 110
/// `ENETDOWN 50` (frozen; this target's `<errno.h>`).
pub const ENETDOWN: c_int = 50;
#[cfg (feature = "windows")]
pub const ENETUNREACH: c_int = 118;   // C 111
/// `ENETUNREACH 51` (frozen; this target's `<errno.h>`).
pub const ENETUNREACH: c_int = 51;
#[cfg (feature = "windows")]
pub const ECONNRESET: c_int = 108;   // C 112
/// `ECONNRESET 54` (frozen; this target's `<errno.h>`).
pub const ECONNRESET: c_int = 54;
#[cfg (feature = "windows")]
pub const ENOBUFS: c_int = 119;   // C 113
/// `ENOBUFS 55` (frozen; this target's `<errno.h>`).
pub const ENOBUFS: c_int = 55;
#[cfg (feature = "windows")]
pub const EISCONN: c_int = 113;   // C 114
/* `EISCONN`'s frozen value is `errno.rs`'s.  */
#[cfg (feature = "windows")]
pub const ENOTCONN: c_int = 126;   // C 115
/// `ENOTCONN 57` (frozen; this target's `<errno.h>`).
pub const ENOTCONN: c_int = 57;
#[cfg (feature = "windows")]
pub const ETIMEDOUT: c_int = 138;   // C 116
/// `ETIMEDOUT 60` (frozen; this target's `<errno.h>`).
pub const ETIMEDOUT: c_int = 60;
#[cfg (feature = "windows")]
pub const ECONNREFUSED: c_int = 107;   // C 117
/// `ECONNREFUSED 61` (frozen; this target's `<errno.h>`).
pub const ECONNREFUSED: c_int = 61;
#[cfg (feature = "windows")]
pub const ELOOP: c_int = 114;   // C 118
/* `ELOOP`'s frozen value is `errno.rs`'s.  */
#[cfg (feature = "windows")]
pub const EHOSTUNREACH: c_int = 110;   // C 119
/// `EHOSTUNREACH 65` (frozen; this target's `<errno.h>`).
pub const EHOSTUNREACH: c_int = 65;
#[cfg (feature = "windows")]
pub const EWOULDBLOCK: c_int = 140;   // C 120
/* `EWOULDBLOCK`'s frozen value is `errno.rs`'s.  */
#[cfg (feature = "windows")]
pub const GNULIB_defined_ESOCK: c_int = 1;   // C 121
#[cfg (feature = "windows")]
pub const ETXTBSY: c_int = 139;   // C 125
/// `ETXTBSY 26` (frozen; this target's `<errno.h>`).
pub const ETXTBSY: c_int = 26;
#[cfg (feature = "windows")]
pub const ENODATA: c_int = 120;   // C 126
/// `ENODATA 96` (frozen; this target's `<errno.h>`).
pub const ENODATA: c_int = 96;
#[cfg (feature = "windows")]
pub const ENOSR: c_int = 124;   // C 127
/// `ENOSR 98` (frozen; this target's `<errno.h>`).
pub const ENOSR: c_int = 98;
#[cfg (feature = "windows")]
pub const ENOSTR: c_int = 125;   // C 128
/// `ENOSTR 99` (frozen; this target's `<errno.h>`).
pub const ENOSTR: c_int = 99;
#[cfg (feature = "windows")]
pub const ETIME: c_int = 137;   // C 129
/// `ETIME 101` (frozen; this target's `<errno.h>`).
pub const ETIME: c_int = 101;
#[cfg (feature = "windows")]
pub const EOTHER: c_int = 131;   // C 130
/* This target's `<errno.h>` does not define `EOTHER`, so C's arm
   above is the only spelling the port has.  */
#[cfg (feature = "windows")]
pub const GNULIB_defined_ESTREAMS: c_int = 1;   // C 131
#[cfg (feature = "windows")]
pub const ESOCKTNOSUPPORT: c_int = 10044;   // C 136
#[cfg (feature = "windows")]
pub const EPFNOSUPPORT: c_int = 10046;   // C 137
/// `EPFNOSUPPORT 46` (frozen; this target's `<errno.h>`).
pub const EPFNOSUPPORT: c_int = 46;
#[cfg (feature = "windows")]
pub const ESHUTDOWN: c_int = 10058;   // C 138
/// `ESHUTDOWN 58` (frozen; this target's `<errno.h>`).
pub const ESHUTDOWN: c_int = 58;
#[cfg (feature = "windows")]
pub const ETOOMANYREFS: c_int = 10059;   // C 139
/// `ETOOMANYREFS 59` (frozen; this target's `<errno.h>`).
pub const ETOOMANYREFS: c_int = 59;
#[cfg (feature = "windows")]
pub const EHOSTDOWN: c_int = 10064;   // C 140
/// `EHOSTDOWN 64` (frozen; this target's `<errno.h>`).
pub const EHOSTDOWN: c_int = 64;
#[cfg (feature = "windows")]
pub const EPROCLIM: c_int = 10067;   // C 141
/// `EPROCLIM 67` (frozen; this target's `<errno.h>`).
pub const EPROCLIM: c_int = 67;
#[cfg (feature = "windows")]
pub const EUSERS: c_int = 10068;   // C 142
/// `EUSERS 68` (frozen; this target's `<errno.h>`).
pub const EUSERS: c_int = 68;
#[cfg (feature = "windows")]
pub const EDQUOT: c_int = 10069;   // C 143
#[cfg (feature = "windows")]
pub const ESTALE: c_int = 10070;   // C 144
#[cfg (feature = "windows")]
pub const EREMOTE: c_int = 10071;   // C 145
/// `EREMOTE 71` (frozen; this target's `<errno.h>`).
pub const EREMOTE: c_int = 71;
#[cfg (feature = "windows")]
pub const GNULIB_defined_EWINSOCK: c_int = 1;   // C 146

/* ---- C 161-224: the "many platforms" block's `# ifndef X` fallbacks -------------------- */
/* Each group is dead because the frozen `<errno.h>` defines the name; C's value is carried
   under the feature naming that group's own condition (P-51), and the frozen value follows.  */
#[cfg (feature = "no_enomsg")]
pub const ENOMSG: c_int = 2000;   // C 162
/// `ENOMSG 91` (frozen; this target's `<errno.h>`).
pub const ENOMSG: c_int = 91;
#[cfg (feature = "no_enomsg")]
pub const GNULIB_defined_ENOMSG: c_int = 1;   // C 163
#[cfg (feature = "no_eidrm")]
pub const EIDRM: c_int = 2001;   // C 167
/// `EIDRM 90` (frozen; this target's `<errno.h>`).
pub const EIDRM: c_int = 90;
#[cfg (feature = "no_eidrm")]
pub const GNULIB_defined_EIDRM: c_int = 1;   // C 168
#[cfg (feature = "no_enolink")]
pub const ENOLINK: c_int = 2002;   // C 172
/// `ENOLINK 97` (frozen; this target's `<errno.h>`).
pub const ENOLINK: c_int = 97;
#[cfg (feature = "no_enolink")]
pub const GNULIB_defined_ENOLINK: c_int = 1;   // C 173
#[cfg (feature = "no_eproto")]
pub const EPROTO: c_int = 2003;   // C 177
/// `EPROTO 100` (frozen; this target's `<errno.h>`).
pub const EPROTO: c_int = 100;
#[cfg (feature = "no_eproto")]
pub const GNULIB_defined_EPROTO: c_int = 1;   // C 178
#[cfg (feature = "no_emultihop")]
pub const EMULTIHOP: c_int = 2004;   // C 182
/// `EMULTIHOP 95` (frozen; this target's `<errno.h>`).
pub const EMULTIHOP: c_int = 95;
#[cfg (feature = "no_emultihop")]
pub const GNULIB_defined_EMULTIHOP: c_int = 1;   // C 183
#[cfg (feature = "no_ebadmsg")]
pub const EBADMSG: c_int = 2005;   // C 187
/// `EBADMSG 94` (frozen; this target's `<errno.h>`).
pub const EBADMSG: c_int = 94;
#[cfg (feature = "no_ebadmsg")]
pub const GNULIB_defined_EBADMSG: c_int = 1;   // C 188
#[cfg (feature = "no_eoverflow")]
pub const EOVERFLOW: c_int = 2006;   // C 192
/* `EOVERFLOW`'s frozen value is `errno.rs`'s.  */
#[cfg (feature = "no_eoverflow")]
pub const GNULIB_defined_EOVERFLOW: c_int = 1;   // C 193
#[cfg (feature = "no_enotsup")]
pub const ENOTSUP: c_int = 2007;   // C 197
/* `ENOTSUP`'s frozen value is `errno.rs`'s.  */
#[cfg (feature = "no_enotsup")]
pub const GNULIB_defined_ENOTSUP: c_int = 1;   // C 198
#[cfg (feature = "no_enetreset")]
pub const ENETRESET: c_int = 2011;   // C 202
/// `ENETRESET 52` (frozen; this target's `<errno.h>`).
pub const ENETRESET: c_int = 52;
#[cfg (feature = "no_enetreset")]
pub const GNULIB_defined_ENETRESET: c_int = 1;   // C 203
#[cfg (feature = "no_econnaborted")]
pub const ECONNABORTED: c_int = 2012;   // C 207
/// `ECONNABORTED 53` (frozen; this target's `<errno.h>`).
pub const ECONNABORTED: c_int = 53;
#[cfg (feature = "no_econnaborted")]
pub const GNULIB_defined_ECONNABORTED: c_int = 1;   // C 208
#[cfg (feature = "no_estale")]
pub const ESTALE: c_int = 2009;   // C 212
/// `ESTALE 70` (frozen; this target's `<errno.h>`).
pub const ESTALE: c_int = 70;
#[cfg (feature = "no_estale")]
pub const GNULIB_defined_ESTALE: c_int = 1;   // C 213
#[cfg (feature = "no_edquot")]
pub const EDQUOT: c_int = 2010;   // C 217
/// `EDQUOT 69` (frozen; this target's `<errno.h>`).
pub const EDQUOT: c_int = 69;
#[cfg (feature = "no_edquot")]
pub const GNULIB_defined_EDQUOT: c_int = 1;   // C 218
#[cfg (feature = "no_ecanceled")]
pub const ECANCELED: c_int = 2008;   // C 222
/// `ECANCELED 89` (frozen; this target's `<errno.h>`).
pub const ECANCELED: c_int = 89;
#[cfg (feature = "no_ecanceled")]
pub const GNULIB_defined_ECANCELED: c_int = 1;   // C 223

/* ---- C 229-255: the `# ifndef EOWNERDEAD` chain -------------------------------------- */
/* Four arms, in C's order: Solaris (C 230-234), the native-Windows pair the pthreads-win32
   build uses (C 238-242), the same condition's MSVC pair (C 243-248, C 88/93's values under
   a nested condition), and the `#  else` fallback (C 249-251).  The two witnesses (C 253-254)
   are defined whenever the chain defined either name.  */
#[cfg (feature = "sun")]
pub const EOWNERDEAD: c_int = 58;   // C 233
#[cfg (feature = "sun")]
pub const ENOTRECOVERABLE: c_int = 59;   // C 234
#[cfg (feature = "mingw32_no_use_windows_threads")]
pub const EOWNERDEAD: c_int = 43;   // C 241
#[cfg (feature = "mingw32_no_use_windows_threads")]
pub const ENOTRECOVERABLE: c_int = 44;   // C 242
#[cfg (feature = "windows_no_mingw32_no_use_windows_threads")]
pub const EOWNERDEAD: c_int = 133;   // C 246 (C 88/93's value under the nested condition)
#[cfg (feature = "windows_no_mingw32_no_use_windows_threads")]
pub const ENOTRECOVERABLE: c_int = 127;   // C 247 (C 88/93's value under the nested condition)
#[cfg (feature = "no_sun_no_windows")]
pub const EOWNERDEAD: c_int = 2013;   // C 250
/// `EOWNERDEAD 105` (frozen; this target's `<errno.h>`).
pub const EOWNERDEAD: c_int = 105;
#[cfg (feature = "no_sun_no_windows")]
pub const ENOTRECOVERABLE: c_int = 2014;   // C 251
/// `ENOTRECOVERABLE 104` (frozen; this target's `<errno.h>`).
pub const ENOTRECOVERABLE: c_int = 104;
#[cfg (feature = "no_eownerdead")]
pub const GNULIB_defined_EOWNERDEAD: c_int = 1;   // C 253
#[cfg (feature = "no_eownerdead")]
pub const GNULIB_defined_ENOTRECOVERABLE: c_int = 1;   // C 254

/* ---- C 258-267: the last two `# ifndef X` fallbacks ---------------------------------- */
#[cfg (feature = "no_eilseq")]
pub const EILSEQ: c_int = 2015;   // C 259
/// `EILSEQ 92` (frozen; this target's `<errno.h>`).
pub const EILSEQ: c_int = 92;
#[cfg (feature = "no_eilseq")]
pub const GNULIB_defined_EILSEQ: c_int = 1;   // C 260
#[cfg (feature = "no_esocktnosupport")]
pub const ESOCKTNOSUPPORT: c_int = 2016;   // C 265
/// `ESOCKTNOSUPPORT 44` (frozen; this target's `<errno.h>`).
pub const ESOCKTNOSUPPORT: c_int = 44;
#[cfg (feature = "no_esocktnosupport")]
pub const GNULIB_defined_ESOCKTNOSUPPORT: c_int = 1;   // C 266

/* The template's remaining text has no name to carry: C 20-23's `__GNUC__ >= 3` test and
   `@PRAGMA_SYSTEM_HEADER@`, C 25-26's split-guard comment, C 32/134/151-159's block comments
   and C 269-270's guard pair.  */
