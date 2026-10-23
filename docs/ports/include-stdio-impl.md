# emacs/lib/stdio-impl.h (308 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 31
- [x] HASUB  (line 221, body 278e62466e6a)
- [x] WINDOWS_OPAQUE_FILE  (line 284, body da39a3ee5e6b)
- [x] _IOEOF  (line 305, body 34dfcaaf6c38)
- [x] _IOERR  (line 306, body 9042d4228c01)
- [x] _IOREAD  (line 302, body 1cb7a2e9afbe)
- [x] _IORW  (line 304, body 60a17847c279)
- [x] _IOWRT  (line 303, body 6b865327cda5)
- [x] _IO_EOF_SEEN  (line 66, body 9042d4228c01)
- [x] _IO_IN_BACKUP  (line 69, body bd60cfec3dba)
- [x] _IO_LINE_BUF  (line 72, body f3504e0d733d)
- [x] _IO_UNBUFFERED  (line 63, body 6b865327cda5)
- [x] __SEOF  (line 234, body a6d133efc0d7)
- [x] __SERR  (line 235, body bf3d90382fd9)
- [x] __SLBF  (line 229, body 356a192b7913)
- [x] __SNBF  (line 230, body da4b9237bacc)
- [x] __SOFF  (line 238, body 987a3bdd4907)
- [x] __SRD  (line 231, body 1b6453892473)
- [x] __SRW  (line 233, body 9042d4228c01)
- [x] __SWR  (line 232, body fe5dbbcea5ce)
- [x] _base  (line 277, body 5257bfeb683c)
- [x] _cnt  (line 275, body 2918c1f9d1f8)
- [x] _flag  (line 278, body 2baca7ae4fcc)
- [x] _flags  (line 108, body 8a67afa35101)
- [x] _gl_file_offset_t  (line 167, body a27521b94e91)
- [x] _gl_flags_file_t  (line 161, body a0f4ea7d9149)
- [x] _p  (line 107, body f27062bddb08)
- [x] _ptr  (line 276, body f62b4ab33b7c)
- [x] _r  (line 109, body 3a97f7e1a2ad)
- [x] _w  (line 110, body 40574b2d8632)
- [x] fp_  (line 298, body 5a33c0cd7c41)
- [x] fp_ub  (line 218, body 1e0b479bc03b)

## Types and their fields — 2
- [x] __sfileext: struct  __sbuf _ub
- [x] _gl_real_FILE: unsigned char* _ptr, unsigned char* _base, int _cnt, int _flag, int _file, int _charbuf, int _bufsiz

## File-scope objects — 0

## Decisions

`emacs/lib/stdio-impl.h` @ `fd7afced403`; port `lib/include/stdio-impl.rs`
(251 lines).  Every checklist item is ticked and `extract-abi.py --check` reports `OK` for the
pair with no row needed in `docs/ports/excluded-macros.txt`.

**The arms the frozen target takes.**  The header has one arm per stdio implementation, and
which one this target compiles was measured with the frozen compiler - `<stdio.h>` first,
because `__sferror` is one of *its* macros, so without it the whole file looks empty:

```
$ printf '#include <stdio.h>\n#include "stdio-impl.h"\n' > /tmp/si.c
$ clang -E -dD -I emacs/lib -I build/emacs-ref/lib -I build/emacs-ref/src -I emacs/src \
    -include config.h /tmp/si.c | grep '^#define'
#define fp_ fp                              #define __SLBF 0x0001
#define fp_ub fp_->_ub                      #define __SNBF 0x0002   ... __SERR 0x0040
#define HASUB(fp) (fp_ub._base != NULL)     #define __SOFF 0x1000
```

So the *live* text is the BSD-family block's `# else` chain (`fp_` = `fp` at C 198, `fp_ub` =
`fp_->_ub` at C 218, `HASUB` at C 221) plus the `__S*` flag family, which Darwin's `<stdio.h>`
defines for itself (the eight measured values are the mirror's constants).  Everything else -
the glibc `_IO_*` markers, the Haiku arm, the DragonFly field renames, the OpenBSD/Android type
macros and `fp_` layouts, the NetBSD/Minix `struct __sfileext`, the SCO field renames, the
Tandem `_IO*` values and the MSVC arm with its `_gl_real_FILE` - is dead here and is carried as
text under the feature naming the C condition (P-51).  `extract-abi.py` cannot evaluate this
header's `#if`s (it is not in `docs/baseline/config/cond-eval-verified.txt`), so every name the
header can define is present.

| item (C line) | shape | why |
| --- | --- | --- |
| `__sbuf`, `__sFILE`, `FILE`, the nine layout assertions | the measured 152-byte `FILE` | the three units that include this mirror read `_p`/`_bf._base`/`_bf._size`/`_flags`; the struct is cut off after `_bf` with a padding tail, so `sizeof` stays the measured 152 |
| `fp_` (C 198), `fp_ub` (C 218), `HASUB` (C 221) | `macro_rules! fp_ { ($fp:expr) => { $fp } }`, `macro_rules! fp_ub { ($fp:expr) => { (*fp_! ($fp))._ub } }`, `macro_rules! HASUB { ($fp:expr) => { (fp_ub! ($fp)._base != core::ptr::null_mut ()) } }` | the live arm's three macros; C's object-like `fp_` becomes a macro that takes the stream (`fp_! (fp)._p`), and nothing in the port expands `fp_ub`/`HASUB` (the units read the measured fields directly, and the mirror's cut `__sFILE` has no `_ub` - it is inside `_rest`) |
| `__SLBF`…`__SOFF` (C 223-240) | `pub const … : core::ffi::c_int = 0x…` | the system header's values (measured above); C's `#ifndef __SEOF` fallbacks are dead and are carried under `#[cfg (feature = "openbsd_or_android_flags")]` |
| `_IO_UNBUFFERED`/`_IO_IN_BACKUP` (C 26/29), `_IO_EOF_SEEN`/`_IO_LINE_BUF` (C 66/72) | `#[cfg (feature = "glibc_io_eof_seen"/"haiku")] pub const` | the glibc and Haiku arms' markers |
| `_p`/`_flags`/`_r`/`_w` (C 107-110) | `#[cfg (feature = "dragonfly")] macro_rules! _p { ($fp:expr) => { $fp.pub._p } }` … | DragonFly's field renames; the extractor compares their single tokens |
| `_gl_flags_file_t` (C 113/125/141/148), `_gl_file_offset_t` (C 150/156) | `#[cfg (feature = …)] macro_rules! … { () => { <type> } }` | type macros (the `struct_stat64` shape); C defines each in four/two mutually exclusive arms |
| `__sfileext` (C 204-215) | `#[cfg (feature = "netbsd_minix_fileext")] #[repr(C)] pub struct __sfileext { pub _ub: __sbuf }` | the NetBSD/Minix arm's record; the field the extractor names is `_ub` |
| `_cnt`/`_ptr`/`_base`/`_flag` (C 275-278) | `#[cfg (feature = "sco")] macro_rules! _cnt { () => { __cnt } }` … | the SCO/UnixWare field renames |
| `_IOERR`…`_IORW` (C 251-254), `_IOREAD`…`_IOERR` (C 302-306) | `#[cfg (feature = "tandem"/"windows_msvc")] pub const` | the Tandem family's values and the MSVC arm's own five |
| `WINDOWS_OPAQUE_FILE` (C 284) | `#[cfg (feature = "windows_msvc")] pub const … : bool = true` | the MSVC arm's empty marker ("`<stdio.h>` does not define the innards of `FILE` any more") |
| `_gl_real_FILE` (C 286-297) | `#[cfg (feature = "windows_msvc")] #[repr(C)] pub struct _gl_real_FILE { … }` | the layout that arm declares, with its seven members in C's order |

**Convergence (PORTING.md 0.1-3).**  Nothing to converge: no unit declared any name this mirror
added (`lib/fpending.rs` and `lib/fseterr.rs` are the only units that include it, and both read
the measured fields).

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/stdio-impl.h lib/include/stdio-impl.rs
  (10 C macros have several definitions (configuration branches), body not compared:
   _IOERR, _IOREAD, _IORW, _IOWRT, _IO_IN_BACKUP, _IO_UNBUFFERED ...)
OK emacs/lib/stdio-impl.h -> lib/include/stdio-impl.rs

$ cd /tmp/femacs-target-mir3 && rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort \
    stdio_probe.rs -o stdio_probe.o
rustc exit=0   # the probe asserts size_of::<__sFILE>() == 152, the `_bf` offset, the eight
               # `__S*` values, and reads `_p`/`_bf._base`/`_bf._size`/`_flags` through `fp_!`

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir3 cargo run -q -p xtask -- build-tu <unit.c>
built lib/fpending.rs      built lib/fseterr.rs            # both exit 0, both consumers

$ python3 tools/extract-abi.py --check emacs/lib/fpending.c lib/fpending.rs
OK emacs/lib/fpending.c -> lib/fpending.rs
OK emacs/lib/fseterr.c -> lib/fseterr.rs

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
```
