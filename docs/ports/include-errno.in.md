# emacs/lib/errno.in.h (270 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 74
- [x] EADDRINUSE  (line 108, body 310b86e0b62b)
- [x] EADDRNOTAVAIL  (line 109, body dbc0f0048544)
- [x] EAFNOSUPPORT  (line 107, body c8306ae139ac)
- [x] EALREADY  (line 99, body 934385f53d1b)
- [x] EBADMSG  (line 187, body 23a0538f53cc)
- [x] ECANCELED  (line 222, body 527dc687fa58)
- [x] ECONNABORTED  (line 207, body 084b3af47af3)
- [x] ECONNREFUSED  (line 117, body 524e05dc7723)
- [x] ECONNRESET  (line 112, body 17503a6b2326)
- [x] EDESTADDRREQ  (line 101, body a1422e6a1686)
- [x] EDQUOT  (line 217, body e22cd461c068)
- [x] EHOSTDOWN  (line 140, body 272a88806d77)
- [x] EHOSTUNREACH  (line 119, body 5e796e48332a)
- [x] EIDRM  (line 167, body 9195f873d171)
- [x] EILSEQ  (line 259, body 9cdda67ded3f)
- [x] EINPROGRESS  (line 98, body 601ca99d55f0)
- [x] EISCONN  (line 114, body e993215bfdaa)
- [x] ELOOP  (line 118, body ecb7937db58e)
- [x] EMSGSIZE  (line 102, body efa6e44dfa01)
- [x] EMULTIHOP  (line 182, body 667e624fb374)
- [x] ENETDOWN  (line 110, body 683e725c03a8)
- [x] ENETRESET  (line 202, body 32a70a32da27)
- [x] ENETUNREACH  (line 111, body 12f0de3dc76e)
- [x] ENOBUFS  (line 113, body a2e33d344f27)
- [x] ENODATA  (line 126, body 775bc5c30e27)
- [x] ENOLINK  (line 172, body 2e8c0277e396)
- [x] ENOMSG  (line 162, body a4ac914c09d7)
- [x] ENOPROTOOPT  (line 104, body 40bd00156308)
- [x] ENOSR  (line 127, body f38cfe2e2fac)
- [x] ENOSTR  (line 128, body 0ca9277f91e4)
- [x] ENOTCONN  (line 115, body 114d4eefde1d)
- [x] ENOTRECOVERABLE  (line 251, body 39e21432a7dc)
- [x] ENOTSOCK  (line 100, body b4182bff4b3c)
- [x] ENOTSUP  (line 197, body aca6d6e0ac7c)
- [x] EOPNOTSUPP  (line 106, body 2a7541babb57)
- [x] EOTHER  (line 130, body e794a80eb109)
- [x] EOVERFLOW  (line 192, body 1938b79762f0)
- [x] EOWNERDEAD  (line 250, body d08b10a32612)
- [x] EPFNOSUPPORT  (line 137, body e0e035dd0b22)
- [x] EPROCLIM  (line 141, body 4bdf46b0f3b8)
- [x] EPROTO  (line 177, body ab165cb90d19)
- [x] EPROTONOSUPPORT  (line 105, body 40f7c01f4189)
- [x] EPROTOTYPE  (line 103, body 9e071a3a594a)
- [x] EREMOTE  (line 145, body 708df9980cbd)
- [x] ESHUTDOWN  (line 138, body d9dc46f1c09c)
- [x] ESOCKTNOSUPPORT  (line 265, body ab39c5423911)
- [x] ESTALE  (line 212, body 7263d678abae)
- [x] ETIME  (line 129, body e1a864f0b77f)
- [x] ETIMEDOUT  (line 116, body 56ad4d4deaec)
- [x] ETOOMANYREFS  (line 139, body 9e64311e9e1b)
- [x] ETXTBSY  (line 125, body fa755791d050)
- [x] EUSERS  (line 142, body 932b682c389c)
- [x] EWOULDBLOCK  (line 120, body c28aca23f1ef)
- [x] GNULIB_defined_EBADMSG  (line 188, body 356a192b7913)
- [x] GNULIB_defined_ECANCELED  (line 223, body 356a192b7913)
- [x] GNULIB_defined_ECONNABORTED  (line 208, body 356a192b7913)
- [x] GNULIB_defined_EDQUOT  (line 218, body 356a192b7913)
- [x] GNULIB_defined_EIDRM  (line 168, body 356a192b7913)
- [x] GNULIB_defined_EILSEQ  (line 260, body 356a192b7913)
- [x] GNULIB_defined_EMULTIHOP  (line 183, body 356a192b7913)
- [x] GNULIB_defined_ENETRESET  (line 203, body 356a192b7913)
- [x] GNULIB_defined_ENOLINK  (line 173, body 356a192b7913)
- [x] GNULIB_defined_ENOMSG  (line 163, body 356a192b7913)
- [x] GNULIB_defined_ENOTRECOVERABLE  (line 254, body 356a192b7913)
- [x] GNULIB_defined_ENOTSUP  (line 198, body 356a192b7913)
- [x] GNULIB_defined_EOVERFLOW  (line 193, body 356a192b7913)
- [x] GNULIB_defined_EOWNERDEAD  (line 253, body 356a192b7913)
- [x] GNULIB_defined_EPROTO  (line 178, body 356a192b7913)
- [x] GNULIB_defined_ESOCK  (line 121, body 356a192b7913)
- [x] GNULIB_defined_ESOCKTNOSUPPORT  (line 266, body 356a192b7913)
- [x] GNULIB_defined_ESTALE  (line 213, body 356a192b7913)
- [x] GNULIB_defined_ESTREAMS  (line 131, body 356a192b7913)
- [x] GNULIB_defined_EWINSOCK  (line 146, body 356a192b7913)
- [x] _  (line 29, body 98874e0ab21c)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/errno.in.h` @ `fd7afced403`; port `lib/include/errno.in.rs` (375 lines,
`include!("errno.rs")` among them).  The batch's other file, `lib/dirent.in.h`, has its own
ledger (`include-dirent.in.md`).

**The criterion is the header the units see, and this target generates none.**  gnulib's
`errno-h` rule substitutes the template when `GL_GENERATE_ERRNO_H_CONDITION` is non-empty and
otherwise runs `rm -f errno.h` (`build/emacs-ref/lib/gnulib.mk` 2051-2067); the frozen
configuration's value is the empty string (`config.log` 5238, `config.status` 1340,
`gnulib.mk` 364 - Evidence), so `build/emacs-ref/lib/errno.h` does not exist *by rule*: the
header every unit sees is the system `<errno.h>` this template would have `#include_next`ed.
That is the situation `emacs/lib/sys_time.in.h` is in as well (its `sys_time.rs` records that
the template "is not used by this build"); the difference is that this template's text is
mirrored rather than left out.

**Both halves.**  `errno.rs` is the system half - the 31 codes the ported units name, each
with its own measurement note in that file - and the mirror `include!`s it, then carries the
template's own text, all of it dead on this target: C 33-148's
`# if defined _WIN32 && ! defined __CYGWIN__` block under `feature = "windows"` (the name
`unistd.in.rs` gives that same condition) and C 161-267's fourteen `# ifndef X` fallbacks
under `feature = "no_<name>"`.  Where the frozen `<errno.h>` defines a name with a different
value, C's arm is written **first** - `tools/extract-abi.py` compares the first definition of
a name - and the frozen value second, ungated, so rustc compiles only the latter (the
`fcntl.in.rs` shape).  The 27 names C defines in two branches are compared by presence, which
the `--check` note in Evidence prints.

**The dead arms are text with a feature each (P-51)** - 106 gated constants beside the 45
ungated ones - and the probe asserts every one of those features off in this configuration.
The table is the whole template: 106 definitions of the 73 named macros (the 74th checklist
entry is the guard, whose name reads as `_`):

| C line | name | feature | C arm | frozen arm |
| --- | --- | --- | --- | --- |
| 38 | `ENOMSG` | `windows` | 122 | 91 |
| 39 | `GNULIB_defined_ENOMSG` | `windows` | 1 | - |
| 43 | `EIDRM` | `windows` | 111 | 90 |
| 44 | `GNULIB_defined_EIDRM` | `windows` | 1 | - |
| 48 | `ENOLINK` | `windows` | 121 | 97 |
| 49 | `GNULIB_defined_ENOLINK` | `windows` | 1 | - |
| 53 | `EPROTO` | `windows` | 134 | 100 |
| 54 | `GNULIB_defined_EPROTO` | `windows` | 1 | - |
| 58 | `EBADMSG` | `windows` | 104 | 94 |
| 59 | `GNULIB_defined_EBADMSG` | `windows` | 1 | - |
| 63 | `EOVERFLOW` | `windows` | 132 | `errno.rs` |
| 64 | `GNULIB_defined_EOVERFLOW` | `windows` | 1 | - |
| 68 | `ENOTSUP` | `windows` | 129 | `errno.rs` |
| 69 | `GNULIB_defined_ENOTSUP` | `windows` | 1 | - |
| 73 | `ENETRESET` | `windows` | 117 | 52 |
| 74 | `GNULIB_defined_ENETRESET` | `windows` | 1 | - |
| 78 | `ECONNABORTED` | `windows` | 106 | 53 |
| 79 | `GNULIB_defined_ECONNABORTED` | `windows` | 1 | - |
| 83 | `ECANCELED` | `windows` | 105 | 89 |
| 84 | `GNULIB_defined_ECANCELED` | `windows` | 1 | - |
| 88 | `EOWNERDEAD` | `windows` | 133 | 105 |
| 89 | `GNULIB_defined_EOWNERDEAD` | `windows` | 1 | - |
| 93 | `ENOTRECOVERABLE` | `windows` | 127 | 104 |
| 94 | `GNULIB_defined_ENOTRECOVERABLE` | `windows` | 1 | - |
| 98 | `EINPROGRESS` | `windows` | 112 | `errno.rs` |
| 99 | `EALREADY` | `windows` | 103 | 37 |
| 100 | `ENOTSOCK` | `windows` | 128 | 38 |
| 101 | `EDESTADDRREQ` | `windows` | 109 | 39 |
| 102 | `EMSGSIZE` | `windows` | 115 | `errno.rs` |
| 103 | `EPROTOTYPE` | `windows` | 136 | 41 |
| 104 | `ENOPROTOOPT` | `windows` | 123 | 42 |
| 105 | `EPROTONOSUPPORT` | `windows` | 135 | 43 |
| 106 | `EOPNOTSUPP` | `windows` | 130 | `errno.rs` |
| 107 | `EAFNOSUPPORT` | `windows` | 102 | 47 |
| 108 | `EADDRINUSE` | `windows` | 100 | 48 |
| 109 | `EADDRNOTAVAIL` | `windows` | 101 | 49 |
| 110 | `ENETDOWN` | `windows` | 116 | 50 |
| 111 | `ENETUNREACH` | `windows` | 118 | 51 |
| 112 | `ECONNRESET` | `windows` | 108 | 54 |
| 113 | `ENOBUFS` | `windows` | 119 | 55 |
| 114 | `EISCONN` | `windows` | 113 | `errno.rs` |
| 115 | `ENOTCONN` | `windows` | 126 | 57 |
| 116 | `ETIMEDOUT` | `windows` | 138 | 60 |
| 117 | `ECONNREFUSED` | `windows` | 107 | 61 |
| 118 | `ELOOP` | `windows` | 114 | `errno.rs` |
| 119 | `EHOSTUNREACH` | `windows` | 110 | 65 |
| 120 | `EWOULDBLOCK` | `windows` | 140 | `errno.rs` |
| 121 | `GNULIB_defined_ESOCK` | `windows` | 1 | - |
| 125 | `ETXTBSY` | `windows` | 139 | 26 |
| 126 | `ENODATA` | `windows` | 120 | 96 |
| 127 | `ENOSR` | `windows` | 124 | 98 |
| 128 | `ENOSTR` | `windows` | 125 | 99 |
| 129 | `ETIME` | `windows` | 137 | 101 |
| 130 | `EOTHER` | `windows` | 131 | - |
| 131 | `GNULIB_defined_ESTREAMS` | `windows` | 1 | - |
| 136 | `ESOCKTNOSUPPORT` | `windows` | 10044 | 44 |
| 137 | `EPFNOSUPPORT` | `windows` | 10046 | 46 |
| 138 | `ESHUTDOWN` | `windows` | 10058 | 58 |
| 139 | `ETOOMANYREFS` | `windows` | 10059 | 59 |
| 140 | `EHOSTDOWN` | `windows` | 10064 | 64 |
| 141 | `EPROCLIM` | `windows` | 10067 | 67 |
| 142 | `EUSERS` | `windows` | 10068 | 68 |
| 143 | `EDQUOT` | `windows` | 10069 | 69 |
| 144 | `ESTALE` | `windows` | 10070 | 70 |
| 145 | `EREMOTE` | `windows` | 10071 | 71 |
| 146 | `GNULIB_defined_EWINSOCK` | `windows` | 1 | - |
| 162 | `ENOMSG` | `no_enomsg` | 2000 | 91 |
| 163 | `GNULIB_defined_ENOMSG` | `no_enomsg` | 1 | - |
| 167 | `EIDRM` | `no_eidrm` | 2001 | 90 |
| 168 | `GNULIB_defined_EIDRM` | `no_eidrm` | 1 | - |
| 172 | `ENOLINK` | `no_enolink` | 2002 | 97 |
| 173 | `GNULIB_defined_ENOLINK` | `no_enolink` | 1 | - |
| 177 | `EPROTO` | `no_eproto` | 2003 | 100 |
| 178 | `GNULIB_defined_EPROTO` | `no_eproto` | 1 | - |
| 182 | `EMULTIHOP` | `no_emultihop` | 2004 | 95 |
| 183 | `GNULIB_defined_EMULTIHOP` | `no_emultihop` | 1 | - |
| 187 | `EBADMSG` | `no_ebadmsg` | 2005 | 94 |
| 188 | `GNULIB_defined_EBADMSG` | `no_ebadmsg` | 1 | - |
| 192 | `EOVERFLOW` | `no_eoverflow` | 2006 | `errno.rs` |
| 193 | `GNULIB_defined_EOVERFLOW` | `no_eoverflow` | 1 | - |
| 197 | `ENOTSUP` | `no_enotsup` | 2007 | `errno.rs` |
| 198 | `GNULIB_defined_ENOTSUP` | `no_enotsup` | 1 | - |
| 202 | `ENETRESET` | `no_enetreset` | 2011 | 52 |
| 203 | `GNULIB_defined_ENETRESET` | `no_enetreset` | 1 | - |
| 207 | `ECONNABORTED` | `no_econnaborted` | 2012 | 53 |
| 208 | `GNULIB_defined_ECONNABORTED` | `no_econnaborted` | 1 | - |
| 212 | `ESTALE` | `no_estale` | 2009 | 70 |
| 213 | `GNULIB_defined_ESTALE` | `no_estale` | 1 | - |
| 217 | `EDQUOT` | `no_edquot` | 2010 | 69 |
| 218 | `GNULIB_defined_EDQUOT` | `no_edquot` | 1 | - |
| 222 | `ECANCELED` | `no_ecanceled` | 2008 | 89 |
| 223 | `GNULIB_defined_ECANCELED` | `no_ecanceled` | 1 | - |
| 233 | `EOWNERDEAD` | `sun` | 58 | 105 |
| 234 | `ENOTRECOVERABLE` | `sun` | 59 | 104 |
| 241 | `EOWNERDEAD` | `mingw32_no_use_windows_threads` | 43 | 105 |
| 242 | `ENOTRECOVERABLE` | `mingw32_no_use_windows_threads` | 44 | 104 |
| 246 | `EOWNERDEAD` | `windows_no_mingw32_no_use_windows_threads` | 133 | 105 |
| 247 | `ENOTRECOVERABLE` | `windows_no_mingw32_no_use_windows_threads` | 127 | 104 |
| 250 | `EOWNERDEAD` | `no_sun_no_windows` | 2013 | 105 |
| 251 | `ENOTRECOVERABLE` | `no_sun_no_windows` | 2014 | 104 |
| 253 | `GNULIB_defined_EOWNERDEAD` | `no_eownerdead` | 1 | - |
| 254 | `GNULIB_defined_ENOTRECOVERABLE` | `no_eownerdead` | 1 | - |
| 259 | `EILSEQ` | `no_eilseq` | 2015 | 92 |
| 260 | `GNULIB_defined_EILSEQ` | `no_eilseq` | 1 | - |
| 265 | `ESOCKTNOSUPPORT` | `no_esocktnosupport` | 2016 | 44 |
| 266 | `GNULIB_defined_ESOCKTNOSUPPORT` | `no_esocktnosupport` | 1 | - |

**Names with no frozen counterpart.**  C 130's `EOTHER` is the one `E*` value this target's
`<errno.h>` does not define at all, and the 24 `GNULIB_defined_*` names are gnulib's own
markers; those exist only under their arm's feature.

**The nested chain** (C 229-255) has four arms for each of its two names: C's *recorded*
definition is the `#  else` (C 250/251), the Solaris pair is C 233/234, the native-Windows
`#if defined __MINGW32__ && !defined USE_WINDOWS_THREADS` pair is C 241/242, the same
condition's `#else` pair is C 246/247 (the values C 88/93 already spell under `windows`), and
the witnesses C 253/254 are carried under `no_eownerdead`.

**Not carried.**  The guard pair (C 18/269-270) and the prologue (C 20-26) have no Rust
counterpart (PORTING.md 0.1, P-08); the guard's own name reads as `_` to the extractor, which
`docs/ports/excluded-macros.txt` already records, and the mirror writes the frozen spelling
`_GL_ERRNO_H` (the `fcntl.in.rs` shape).

**The consumer.**  `lib/dup2.rs` 31 reads this mirror now (it named `include/errno.rs`
before, and its unit check stays `OK`); the other 75 units that include the system half are
the integrator's mechanical sweep, since the mirror is a superset of `errno.rs` - two of them,
`src/keyboard.rs` and `src/xdisp.rs`, belong to another lane that is in flight.
No unit declares one of the 47 codes the mirror adds (measured: 0 hits), so the sweep is an
include-line change.

**The read caught one stale use-site note** (fixed in this batch): `lib/dup2.rs` 35-37
said `EMFILE` was "the one code the live path names that the errno mirror has not got", while
`errno.rs` 89-90 has carried `EMFILE 24` since status 3.80 retired that unit's own copy; the
note is rewritten to point at the mirror.  `lib/dirent.rs`'s two citations of
`dirent.in.h` were off by the generated file's numbering (its 953-959 is the generated
`build/emacs-ref/lib/dirent.h`'s range, the template's is 406-413); they are corrected in the
dirent half of this batch.

## Evidence

```
$ grep -n GL_GENERATE_ERRNO_H_CONDITION build/emacs-ref/config.log build/emacs-ref/config.status build/emacs-ref/lib/gnulib.mk
build/emacs-ref/config.log:5238:GL_GENERATE_ERRNO_H_CONDITION=''
build/emacs-ref/config.status:1340:S["GL_GENERATE_ERRNO_H_CONDITION"]=""
build/emacs-ref/lib/gnulib.mk:364:GL_GENERATE_ERRNO_H_CONDITION =
$ ls build/emacs-ref/lib/errno.h
ls: build/emacs-ref/lib/errno.h: No such file or directory
# gnulib.mk 2051-2067 is the rule itself: `ifneq (,$(GL_GENERATE_ERRNO_H_CONDITION))` -> sed the
# template into errno.h, `else` -> `rm -f $@`.

# the frozen values (this target's <errno.h>), one command; the 61 names below the template
$ clang -std=gnu23 -E -dM -include errno.h /tmp/empty2.c | grep -E '^#define E[A-Z0-9_]+ '
E2BIG 7 EACCES 13 EADDRINUSE 48 EADDRNOTAVAIL 49 EAFNOSUPPORT 47
EAGAIN 35 EALREADY 37 EAUTH 80 EBADARCH 86 EBADEXEC 85
EBADF 9 EBADMACHO 88 EBADMSG 94 EBADRPC 72 EBUSY 16
ECANCELED 89 ECHILD 10 ECONNABORTED 53 ECONNREFUSED 61 ECONNRESET 54
EDEADLK 11 EDESTADDRREQ 39 EDEVERR 83 EDOM 33 EDQUOT 69
EEXIST 17 EFAULT 14 EFBIG 27 EFTYPE 79 EHOSTDOWN 64
EHOSTUNREACH 65 EIDRM 90 EILSEQ 92 EINPROGRESS 36 EINTR 4
EINVAL 22 EIO 5 EISCONN 56 EISDIR 21 ELAST 107
ELOOP 62 EMFILE 24 EMLINK 31 EMSGSIZE 40 EMULTIHOP 95
ENAMETOOLONG 63 ENEEDAUTH 81 ENETDOWN 50 ENETRESET 52 ENETUNREACH 51
ENFILE 23 ENOATTR 93 ENOBUFS 55 ENODATA 96 ENODEV 19
ENOENT 2 ENOEXEC 8 ENOLCK 77 ENOLINK 97 ENOMEM 12
ENOMSG 91 ENOPOLICY 103 ENOPROTOOPT 42 ENOSPC 28 ENOSR 98
ENOSTR 99 ENOSYS 78 ENOTBLK 15 ENOTCAPABLE 107 ENOTCONN 57
ENOTDIR 20 ENOTEMPTY 66 ENOTRECOVERABLE 104 ENOTSOCK 38 ENOTSUP 45
ENOTTY 25 ENXIO 6 EOPNOTSUPP 102 EOVERFLOW 84 EOWNERDEAD 105
EPERM 1 EPFNOSUPPORT 46 EPIPE 32 EPROCLIM 67 EPROCUNAVAIL 76
EPROGMISMATCH 75 EPROGUNAVAIL 74 EPROTO 100 EPROTONOSUPPORT 43 EPROTOTYPE 41
EPWROFF 82 EQFULL 106 ERANGE 34 EREMOTE 71 EROFS 30
ERPCMISMATCH 73 ESHLIBVERS 87 ESHUTDOWN 58 ESOCKTNOSUPPORT 44 ESPIPE 29
ESRCH 3 ESTALE 70 ETIME 101 ETIMEDOUT 60 ETOOMANYREFS 59
ETXTBSY 26 EUSERS 68 EWOULDBLOCK EAGAIN EXDEV 18
# (`EOTHER`, the template's C 130, is the one name this table has not got.)

# the negative control: the check compares values and tokens, not just name presence
$ mkdir -p /tmp/gt-neg && cp lib/include/errno.rs lib/include/errno.in.rs /tmp/gt-neg/
$ sed 's/pub const ELOOP: c_int = 114;/pub const ELOOP: c_int = 999;/' \
      lib/include/errno.in.rs > /tmp/gt-neg/errno.in.rs
$ python3 tools/extract-abi.py --check emacs/lib/errno.in.h /tmp/gt-neg/errno.in.rs
- macro ELOOP does not mention 114 (C body: 114)
- constant ELOOP: C 114 vs Rust 999
FAIL emacs/lib/errno.in.h -> /tmp/gt-neg/errno.in.rs

$ python3 tools/extract-abi.py --check emacs/lib/errno.in.h lib/include/errno.in.rs
  (1 C macros have a recorded Rust counterpart: _)
  (27 C macros have several definitions (configuration branches), body not compared:
   EBADMSG, ECANCELED, ECONNABORTED, EDQUOT, EIDRM, ENETRESET ...)
OK emacs/lib/errno.in.h -> lib/include/errno.in.rs

# the mirror compiles, and every value it ends up with is pinned in the probe
$ cd /tmp/femacs-gt-probe-errno && rustc --edition 2024 --crate-type=lib --emit=obj \
    -C panic=abort errno_probe.rs -o errno_probe.o
exit=0        # 45 frozen-value assertions + the seven values `errno.rs` carries + the
              # `errno ()` accessor + 21 `assert! (!cfg! (feature = "…"))` guards (P-51)

# one consumer reads the mirror end to end
$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-gt cargo run -q -p xtask -- build-tu lib/dup2.c
built lib/dup2.rs -> ../build/femacs/dup2.o
$ python3 tools/extract-abi.py --check emacs/lib/dup2.c lib/dup2.rs
OK emacs/lib/dup2.c -> lib/dup2.rs

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
$ python3 tools/extract-abi.py --all
checked 523 ported files: 516 done, 7 in progress, 0 failing
```
