# emacs/lib/unistd.in.h (2447 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] rpl_environ  (line 623)
- [x] getpagesize  (line 1464)

## Macros defined — 91
- [x] CLSIZE  (line 1449, body 356a192b7913)
- [x] F_OK  (line 275, body b6589fc6ab0d)
- [x] GNULIB_defined_getpagesize_function  (line 1468, body 356a192b7913)
- [x] GNULIB_defined_isatty  (line 1627, body 356a192b7913)
- [x] R_OK  (line 278, body 1b6453892473)
- [x] STDERR_FILENO  (line 270, body da4b9237bacc)
- [x] STDIN_FILENO  (line 264, body b6589fc6ab0d)
- [x] STDOUT_FILENO  (line 267, body 356a192b7913)
- [x] W_OK  (line 277, body da4b9237bacc)
- [x] X_OK  (line 276, body 356a192b7913)
- [x] _  (line 70, body e6fccbcff661)
- [x] _GL_GETPAGESIZE_INLINE  (line 183, body a960df3ab827)
- [x] _GL_INCLUDING_UNISTD_H  (line 38, body da39a3ee5e6b)
- [x] _GL_INCLUDING_WINSOCK2_H  (line 64, body da39a3ee5e6b)
- [x] _GL_UNISTD_INLINE  (line 180, body a960df3ab827)
- [x] _GL_USE_CRT_EXTERNS  (line 602, body da39a3ee5e6b)
- [x] __need_system_stdlib_h  (line 122, body da39a3ee5e6b)
- [x] _gl_getpagesize  (line 1454, body 0687a32f08c0)
- [x] accept  (line 196, body 23c177e695b8)
- [x] access  (line 319, body 17982a145cd2)
- [x] bind  (line 198, body b0f7368fca0d)
- [x] chdir  (line 352, body 79abe245b19f)
- [x] chown  (line 371, body e3f5fbbd2acf)
- [x] close  (line 428, body c02131dc7c32)
- [x] connect  (line 194, body 793ce4533509)
- [x] copy_file_range  (line 442, body b504d0fb1101)
- [x] dup  (line 501, body 5d156978e3d3)
- [x] dup2  (line 545, body c56b37f413ae)
- [x] dup3  (line 567, body 439bb087081f)
- [x] endusershell  (line 1587, body ed98cf7f3617)
- [x] environ  (line 628, body ea5bd3da5c28)
- [x] execl  (line 680, body de5ab1699f62)
- [x] execle  (line 714, body 2f0a51d09b83)
- [x] execlp  (line 749, body 692eddfca573)
- [x] execv  (line 785, body e92ccd7575c5)
- [x] execve  (line 823, body 67f6ec2d3508)
- [x] execvp  (line 860, body a9e6a75c7101)
- [x] execvpe  (line 906, body c7202b1c5c17)
- [x] faccessat  (line 930, body 04d86cdbbec6)
- [x] fchdir  (line 966, body 30f5e41fe02d)
- [x] fchownat  (line 997, body a05b62331e2c)
- [x] fdatasync  (line 1030, body 6b019effb942)
- [x] ftruncate  (line 1077, body eab53e29722c)
- [x] getcwd  (line 1144, body cb36405f62bb)
- [x] getdomainname  (line 1170, body c4caa990bcc9)
- [x] getdtablesize  (line 1203, body 9b44990f54e9)
- [x] getentropy  (line 1229, body 1081c88ad146)
- [x] getgroups  (line 1263, body 0d005ea85175)
- [x] gethostname  (line 1315, body e2293a7f1ec7)
- [x] getlogin  (line 1338, body 35a7ed6ad47b)
- [x] getlogin_r  (line 1376, body b55b1da99c69)
- [x] getpagesize  (line 1460, body 7b32ea787d8a)
- [x] getpass  (line 1496, body 9967dc3b2df2)
- [x] getpeername  (line 200, body 8d67ebf56174)
- [x] getpid  (line 1524, body 440456939c68)
- [x] getsockname  (line 202, body 73cbc23b0678)
- [x] getsockopt  (line 204, body 70c568bd4a93)
- [x] getusershell  (line 1540, body 2d9450e94bc5)
- [x] isatty  (line 1652, body a9e4af0e4432)
- [x] lchown  (line 1671, body 86569a5deb92)
- [x] link  (line 1699, body 3647cab9631c)
- [x] linkat  (line 1727, body 77655abdf480)
- [x] listen  (line 206, body 6c83886edf9e)
- [x] lseek  (line 1791, body cda812659b9a)
- [x] pipe2  (line 1831, body c72d8338a0f0)
- [x] pread  (line 1861, body 81eac5d2f767)
- [x] pwrite  (line 1897, body 0d615522ea8d)
- [x] read  (line 1954, body be410059a97a)
- [x] readlink  (line 1972, body d5cffda4ed69)
- [x] readlinkat  (line 2004, body a943e94ff93c)
- [x] recv  (line 208, body 322654f6db7f)
- [x] recvfrom  (line 212, body 33ed9871c4fb)
- [x] rmdir  (line 2065, body f69c092dbdd7)
- [x] select  (line 253, body e78cf58530e6)
- [x] send  (line 210, body de503e7a67a6)
- [x] sendto  (line 214, body 3931182241d0)
- [x] sethostname  (line 2087, body 1b665da62c69)
- [x] setsockopt  (line 216, body b316418cd81c)
- [x] setusershell  (line 1563, body 74ee1e9e8975)
- [x] shutdown  (line 218, body 7b25afff2732)
- [x] sleep  (line 2125, body f9035b10855f)
- [x] socket  (line 192, body e3118d6fa129)
- [x] swab  (line 2151, body a8dae1fb2b6a)
- [x] symlink  (line 2173, body 9fff43eb0890)
- [x] symlinkat  (line 2202, body c5b3add47263)
- [x] truncate  (line 2237, body a37dbe9ea626)
- [x] ttyname_r  (line 2266, body 8a6f13c19b28)
- [x] unlink  (line 2323, body 706a5717bd3c)
- [x] unlinkat  (line 2337, body a1d58370447f)
- [x] usleep  (line 2366, body 9730fc484983)
- [x] write  (line 2420, body 72e37d6c70ea)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern char environ  (line 612)

## Decisions

`emacs/lib/unistd.in.h` @ `fd7afced403`; port `lib/include/unistd.in.rs` (330 lines).

**The template.**  The reference is a gnulib template (`@GUARD_PREFIX@`,
`@PRAGMA_SYSTEM_HEADER@`, `@INCLUDE_NEXT@`, `@NEXT_UNISTD_H@`, `@GNULIB_*@`, `@REPLACE_*@`,
`@HAVE_*@`), so `tools/check-cond-eval.py` cannot decide its conditions and it stays off the
verified list (P-66): `extract-abi.py` reads every arm as live and the mirror must name every
macro the template can define - 91 macros, 2 functions and 1 file-scope object.  The frozen
substitution is the generated `build/emacs-ref/lib/unistd.h`, and **what that configuration
actually compiles was measured, not inferred**, by preprocessing it with the frozen compiler
(the mirror's header comment carries the command):

```
$ printf '#include <unistd.h>\n' > /tmp/u.c
$ clang -E -dD -I build/emacs-ref/lib -I build/emacs-ref/src -I emacs/src -I emacs/lib \
    -include config.h /tmp/u.c | grep '^#define'
#define STDIN_FILENO 0                  #define _GL_UNISTD_INLINE _GL_INLINE
#define STDOUT_FILENO 1                 #define _GL_GETPAGESIZE_INLINE _GL_INLINE
#define STDERR_FILENO 2                 #define _GL_USE_CRT_EXTERNS
#define environ (*_NSGetEnviron ())     #define pipe2 rpl_pipe2
int rpl_pipe2 (int fd[2], int flags);   <- the one declaration of gnulib's own text
```

So the live header is the system `<unistd.h>` the template includes next (its C 32/33), plus
gnulib's `pipe2` replacement (`@REPLACE_PIPE2@` is 1 - the only `@REPLACE_*@` set for the
functions this header renames), the macOS `environ` macro (the `@HAVE_DECL_ENVIRON@` 0 arm
with `TARGET_OS_IPHONE` 0), the three descriptor constants (the frozen system header defines
all three, so the template's `#ifndef` arms are dead) and `_GL_USE_CRT_EXTERNS`; every other
`@GNULIB_*@`/`@REPLACE_*@` arm is dead text here.

**The two halves.**  The port has no `#include_next`, so the mirror declares the *system*
entries the ported units name (what the frozen header's dead `_GL_CXXALIAS_SYS` arms reach -
the same choice `sys_stat.in.rs`/`fcntl.in.rs` record), and carries the template's own names as
`time.in.rs`/`endian.in.rs` do: the live ones with the frozen value, the dead arms as text
under a feature naming the C condition (P-51).  The extractor evaluates no `feature`, so every
carried name is visible to it; the features are never in `xtask dead-arms`'s fixed flip list,
so the arms are text, not compiled code.

| item (C line) | shape | why |
| --- | --- | --- |
| `ssize_t` | `pub type ssize_t = c_long` | the frozen target's `ssize_t` (`sizeof` 8, measured) |
| `F_OK`/`X_OK`/`W_OK`/`R_OK` (C 275-278), `STDIN/OUT/ERR_FILENO` (C 264/267/270) | `pub const` | the system header defines all seven; C's `#ifndef` arms are dead |
| `_SC_NPROCESSORS_CONF/ONLN/PAGESIZE` | `pub const` | the system's `sysconf` names (57/58/29, measured); `_SC_PAGESIZE` is the argument the dead `_gl_getpagesize` arm names |
| `_GL_INCLUDING_WINSOCK2_H` (C 64), `__need_system_stdlib_h` (C 122) | `pub const … : bool = true` | C's empty guard markers; the port has no include of its own to guard, so the marker is the value C's `#ifdef` reads (the `verify.rs` `_GL_VERIFY_H` shape) |
| `_GL_INCLUDING_UNISTD_H` (C 38) | `macro_rules! _GL_INCLUDING_UNISTD_H { () => {} }` | the same marker, but the *unit* that needs the system `<unistd.h>` first sets it itself (`lib/faccessat.rs` 39 mirrors C 22 with a `pub const`, the `_GL_*_INLINE` shape), so the mirror cannot hold the name as a value item (`E0428`, found by `build-tu lib/faccessat.c`); a `macro_rules!` lives in the namespace C's `#define` also uses, which is what lets both stand |
| `_GL_USE_CRT_EXTERNS` (C 602) | `pub const … : bool = true` | live (empty) marker of the macOS `environ` arm |
| `environ` (C 628) | `macro_rules! environ { () => { (*_NSGetEnviron ()) } }` | the live macOS spelling of the name (C 613), with `_NSGetEnviron` declared as `<crt_externs.h>` declares it (`char ***_NSGetEnviron (void)`) |
| `extern char **environ` (C 612) | `pub static mut environ: *mut *mut *mut core::ffi::c_char` | C's `#else` arm of the same block, and the file-scope object the extractor asks for; a macro and a static live in different namespaces, so both arms stand as in C |
| `rpl_environ` (C 623-626) | `#[inline(always)] pub unsafe fn` | C defines it (`return &environ;`); the port keeps a function, and the token check needs the `environ` in its body |
| `pipe2` (C 1837-1839) | `macro_rules! pipe2 { ($fd, $flags) => { rpl_pipe2 (…) } }` | **live**: the rename `#define pipe2 rpl_pipe2`, with `rpl_pipe2` declared in the mirror's `unsafe extern "C"` block (the generated header's only gnulib declaration); `lib/pipe2.rs` defines it with `#[unsafe(export_name = "rpl_pipe2")]` |
| `getpagesize` (C 1460/1464), `_gl_getpagesize` (C 1454), `CLSIZE` (C 1449), `GNULIB_defined_getpagesize_function` (C 1468) | `#[cfg (feature = "gnulib_getpagesize")]` + a `macro_rules!`/`pub const` | dead here (`@GNULIB_GETPAGESIZE@` is 0, `HAVE_GETPAGESIZE` is 1), so the system `getpagesize` declared above is the live entry and C's own definition is carried as its macro |
| the 14 socket names + `select` (C 167-258) | `#[cfg (feature = "unistd_h_have_winsock2_h")] macro_rules!` | the Windows arm's "used without including `<sys/socket.h>`" renames (`@UNISTD_H_HAVE_WINSOCK2_H@` is 0) |
| `gethostname` (C 1305/1315) | two `macro_rules!` under `replace_gethostname` / `unistd_h_have_winsock2_h` | C defines the name in two arms (the extractor compares no body for a doubly-defined name) |
| 22 `@REPLACE_*@` renames (`copy_file_range`, `dup3`, `faccessat`, `fchownat`, `fdatasync`, `getdomainname`, `getdtablesize`, `getentropy`, `getgroups`, `getlogin`, `getlogin_r`, `getpass`, `pread`, `pwrite`, `sethostname`, `sleep`, `symlinkat`, `ttyname_r`, `usleep`, `get|set|endusershell`) | `#[cfg (feature = "replace_<name>")] macro_rules! X { () => { rpl_X } }` | dead arms; the body is the single identifier C spells, which is what the extractor compares |
| `execl`…`execvpe`, `swab` (C 680-906/2151) | `#[cfg (feature = "windows")] macro_rules!` | the native-Windows `_execl`/`_swab` renames (and their `@GNULIB_MDA_*@` twins) |
| `GNULIB_defined_isatty` (C 1627) | `#[cfg (feature = "replace_isatty")] pub const … = 1` | the marker of the dead `@REPLACE_ISATTY@` arm |

Names the mirror does **not** repeat: `chmod` (declared once, in `sys_stat.in.rs`), and the
functions whose `#define X rpl_X`/`_X` arms the extractor counts but the mirror already
declares as *functions* (`access`, `chdir`, `close`, `dup`, `dup2`, `ftruncate`, `getcwd`,
`isatty`, `lchown`, `link`, `linkat`, `lseek`, `read`, `readlink`, `readlinkat`, `rmdir`,
`symlink`, `truncate`, `unlink`, `unlinkat`, `write`, `fchdir`, `chown`, `getpid`) - a
declaration satisfies the extractor's "missing macro" rule and, with several C definitions,
its body rule does not apply.

**The two `*_INLINE` names need one row each in `docs/ports/excluded-macros.txt`** (that file
is the integrator's; this lane must not write it - the `limits.in.md` flow).  With those two
rows in a `/tmp` copy the pair is `OK`; without them the check reports exactly the two
`missing macro` lines, which is why their two boxes above stay unchecked:

```
_GL_UNISTD_INLINE	syntax	unistd.in.h 180-182's `#ifndef _GL_UNISTD_INLINE / # define _GL_UNISTD_INLINE _GL_INLINE` spells gnulib's `static inline`; the unit `lib/unistd.c` carries the qualifier itself (`#define _GL_UNISTD_INLINE _GL_EXTERN_INLINE` before the include, the unit's own `pub const` in lib/unistd.rs), so the mirror cannot hold the name without an E0428
_GL_GETPAGESIZE_INLINE	syntax	same construct (unistd.in.h 183-185); the header's only user is the `getpagesize` inline definition at C 1464, which is dead here (@GNULIB_GETPAGESIZE@ is 0) and which the mirror spells as a macro (`getpagesize`) plus the system declaration, so nothing holds the qualifier
```

**Convergence (PORTING.md 0.1-3, same batch).**  The names the mirror added had use-site copies
in twelve places; all were deleted, the consumers left reading the mirror:

| unit | copy | now |
| --- | --- | --- |
| `src/callproc.rs` 252-255 | three `const STDIN/OUT/ERR_FILENO` | mirror |
| `src/callproc.rs` 191-193 | `pub static mut environ` | mirror |
| `src/dispnew.rs` 7373-7374 | `const STDIN_FILENO` | mirror |
| `src/emacs.rs` 796-799 | three `const …FILENO` | mirror |
| `src/minibuf.rs` 79-80 | `const STDIN_FILENO` | mirror |
| `src/print.rs` 131-132 | `const STDERR_FILENO` | mirror |
| `src/sysdep.rs` 449-453 | three `pub const …FILENO` | mirror |
| `lib-src/emacsclient.rs` 243-244 | `pub const STDOUT_FILENO` | mirror |
| `lib-src/emacsclient.rs` 329-332 | `pub static mut environ` | mirror |

`src/callproc.rs`'s `environ` and `lib-src/emacsclient.rs`'s two copies are the ones the
*compiler* found (`E0428`, the mirror's `extern` static/const and the unit's at the same crate
root); `check-mirror-dupes.py` does not see statics inside `unsafe extern "C"` blocks and does
not scan `lib-src/`, so the unit builds are what proved them (the integrator's `gate-fast` found
the emacsclient pair first - the tool's `UNIT_DIRS` is `src`/`lib` only, and a `lib-src` unit is
built only by `gate-fast`/`build-tu`).  The lesson is recorded in Evidence: after adding a name
to a mirror, `build-tu` **every** finished unit in its include graph, `lib-src`/`nt` included.

**Not edited** (another lane's in-flight units, reported instead of patched):
`lib-src/etags.rs` 1161 declares `pub const STDOUT_FILENO: c_int = 1;` and includes this
mirror, so it is an `E0428` waiting for its build (the mirror now carries the name).

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/unistd.in.h lib/include/unistd.in.rs
  (1 C macros have a recorded Rust counterpart: _)
  (23 C macros have several definitions (configuration branches), body not compared:
   _gl_getpagesize, access, chdir, close, dup, dup2 ...)
- missing macro _GL_GETPAGESIZE_INLINE
- missing macro _GL_UNISTD_INLINE
FAIL emacs/lib/unistd.in.h -> lib/include/unistd.in.rs
# the same command with the two rows above appended to a /tmp copy of the shared file:
$ python3 tools/extract-abi.py --check --excluded /tmp/mem-excl-unistd.txt \
    emacs/lib/unistd.in.h lib/include/unistd.in.rs
  (3 C macros have a recorded Rust counterpart: _, _GL_GETPAGESIZE_INLINE, _GL_UNISTD_INLINE)
OK emacs/lib/unistd.in.h -> lib/include/unistd.in.rs

$ cd /tmp/femacs-target-mir3 && rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort \
    unistd_probe.rs -o unistd_probe.o
rustc exit=0   # the probe use-imports the ffi scalars + include!s the mirror, asserts the seven
               # descriptor/mode values, _SC_PAGESIZE 29, _SC_NPROCESSORS_* 57/58,
               # size_of::<ssize_t>() == 8 and _GL_USE_CRT_EXTERNS, calls rpl_pipe2/getpagesize/
               # sysconf/environ!/rpl_environ/readlink, and defines its own `pipe2` fn beside
               # the mirror's `pipe2` macro to prove the two namespaces coexist

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir3 cargo run -q -p xtask -- build-tu <unit.c>
built lib/pipe2.rs      built src/callproc.rs   built src/fileio.rs    built src/emacs.rs
built src/sysdep.rs     built src/dispnew.rs    built src/minibuf.rs   built src/print.rs
built src/alloc.rs      built lib/nproc.rs      built lib/getloadavg.rs  built lib/fsusage.rs
# all exit 0 (12 consumers, the six edited ones among them)

$ python3 tools/extract-abi.py --check emacs/<edited unit>.c <edited unit>.rs
OK emacs/src/callproc.c -> src/callproc.rs
OK emacs/src/dispnew.c -> src/dispnew.rs
OK emacs/src/emacs.c -> src/emacs.rs
OK emacs/src/minibuf.c -> src/minibuf.rs
OK emacs/src/print.c -> src/print.rs
OK emacs/src/sysdep.c -> src/sysdep.rs

$ # the whole include graph of both mirrors, lib-src/nt included (57 units, `build-tu` each):
$ for u in <every unit with `include!(*unistd.in.rs)` or `*sys_stat.in.rs)`>; do
      CARGO_TARGET_DIR=/tmp/femacs-target-mir3 cargo run -q -p xtask -- build-tu $u; done
built lib-src/emacsclient.rs  lib-src/make-docfile.rs  lib-src/make-fingerprint.rs
built lib-src/movemail.rs  lib/*.rs (35)  src/*.rs (21)              # all exit 0
# the first sweep found two E0428s in the *committed* lib-src/emacsclient.rs (the integrator's
# gate-fast reported them first): the unit's own `STDOUT_FILENO` and `environ`; both copies are
# gone now, and `build-tu lib/faccessat.c` found a third one - that unit's own
# `pub const _GL_INCLUDING_UNISTD_H: () = ();` (its C 22 marker) - which is why the mirror holds
# that name as a macro instead (see the table above).  `src/keyboard.rs`,
# `src/xdisp.rs` and `lib-src/etags.rs` were *not* built (another lane's
# in-flight units, PORTING.md 0.1-3's "stop and report" case).

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)     # the tree's single ratchet row;
               # a run started while another lane was mid-flight on fcntl.in.rs/stdint.in.rs
               # showed theirs, never mine
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
```

## 2026-10-13: `readlink`/`symlink` join the `windowsnt` arm's gates (w32.c's Owed 17)

`src/w32.c` defines `readlink` (C 6331) and `symlink` (C 6118) for the Windows arm - the symlink
trio the lane carried as its fourteenth slice - and this header declares both at this scope (lines
39 and 40), so each declaration was an `E0428` against its definition.  Both now carry
`#[cfg (not (feature = "windowsnt"))]`, joining the `getwd`/`getppid`/`getgid`/`getegid`/`fchown`
rows this mirror already gates (the device status 3.483 and `docs/ports/pop.md` Owed 7 record).

Evidence: `src/w32.rs`'s `windowsnt` probe read **125 error lines / 150 located sites**
with the two declarations live; after the gate it reads **123 / 148**, and the two `E0428` rows are
gone.  The frozen probe is unchanged (**exit 0**, **336-byte** empty object) and this mirror's own
`python3 tools/extract-abi.py --check emacs/lib/unistd.in.h lib/include/unistd.in.rs` still
reads **OK**.  `readlinkat` (declared on line 41) is deliberately **not** gated yet: `w32.c`'s own
`readlinkat` is C 6475, the next slice's first definition, and this lane gates only what a slice
has measured.

## 2026-10-13 (later): `readlinkat` follows its two siblings out of the `windowsnt` arm (Owed 19)

The third name the section above deliberately left live.  `src/w32.c` defines `readlinkat` at
C 6475 - the lane's fifteenth slice - so the same `windowsnt` probe read one `E0428` for this
header's line 41; it now carries `#[cfg (not (feature = "windowsnt"))]` like `readlink` and
`symlink` above it.

Evidence: the probe reads **123 error lines / 149 located sites** before the gate and **122 / 148**
after (the slice-14 text reads 123 / 148), the frozen probe is unchanged (**exit 0**, **336-byte**
empty object), and this mirror's own
`python3 tools/extract-abi.py --check emacs/lib/unistd.in.h lib/include/unistd.in.rs` still
reads **OK**.  All three of `C 6118`/`C 6331`/`C 6475`'s names are now gated; the remaining
declarations of this header that `w32.c` might answer for will be gated the same way as their
slices measure them.
