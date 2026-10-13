# emacs/lib/sys_stat.in.h (995 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] rpl_mkdir  (line 656, static)
- [x] rpl_mkdir  (line 678, static)

## Macros defined — 71
- [x] GNULIB_defined_rpl_mkdir  (line 682, body 356a192b7913)
- [x] GNULIB_defined_struct_stat  (line 174, body 356a192b7913)
- [x] S_IFIFO  (line 192, body 54c3f5b01ff2)
- [x] S_IFMT  (line 197, body b0114ab87659)
- [x] S_IRGRP  (line 375, body 6f14d4531dca)
- [x] S_IROTH  (line 378, body 00cc7bd4a8d2)
- [x] S_IRUSR  (line 372, body 793a3f8290c4)
- [x] S_IRWXG  (line 411, body 9650316727dd)
- [x] S_IRWXO  (line 414, body 84b31b84969b)
- [x] S_IRWXU  (line 408, body 61dd832429af)
- [x] S_IRWXUGO  (line 423, body 03709dd2c0d8)
- [x] S_ISBLK  (line 218, body b6589fc6ab0d)
- [x] S_ISCHR  (line 226, body b6589fc6ab0d)
- [x] S_ISCTG  (line 336, body b6589fc6ab0d)
- [x] S_ISDIR  (line 234, body b6589fc6ab0d)
- [x] S_ISDOOR  (line 239, body b6589fc6ab0d)
- [x] S_ISFIFO  (line 246, body b6589fc6ab0d)
- [x] S_ISGID  (line 360, body 790776b09216)
- [x] S_ISLNK  (line 254, body b6589fc6ab0d)
- [x] S_ISMPB  (line 263, body b6589fc6ab0d)
- [x] S_ISMPC  (line 264, body b6589fc6ab0d)
- [x] S_ISMPX  (line 269, body b6589fc6ab0d)
- [x] S_ISNAM  (line 276, body b6589fc6ab0d)
- [x] S_ISNWK  (line 284, body b6589fc6ab0d)
- [x] S_ISOFD  (line 341, body b6589fc6ab0d)
- [x] S_ISOFL  (line 346, body b6589fc6ab0d)
- [x] S_ISPORT  (line 289, body b6589fc6ab0d)
- [x] S_ISREG  (line 296, body b6589fc6ab0d)
- [x] S_ISSOCK  (line 304, body b6589fc6ab0d)
- [x] S_ISUID  (line 357, body 5967c2cac272)
- [x] S_ISVTX  (line 365, body 19ac7b498eca)
- [x] S_ISWHT  (line 351, body b6589fc6ab0d)
- [x] S_IWGRP  (line 388, body 8f8092f9b1c3)
- [x] S_IWOTH  (line 391, body e66e49c63a9d)
- [x] S_IWUSR  (line 385, body 1f06964b7094)
- [x] S_IXGRP  (line 401, body f47b98f25206)
- [x] S_IXOTH  (line 404, body 056d68d3df12)
- [x] S_IXUGO  (line 420, body e0528301b62f)
- [x] S_IXUSR  (line 398, body b53ebbb22df0)
- [x] S_TYPEISMQ  (line 310, body b6589fc6ab0d)
- [x] S_TYPEISSEM  (line 322, body b6589fc6ab0d)
- [x] S_TYPEISSHM  (line 330, body b6589fc6ab0d)
- [x] S_TYPEISTMO  (line 314, body b6589fc6ab0d)
- [x] UTIME_NOW  (line 428, body eb883130e83d)
- [x] UTIME_OMIT  (line 429, body e5e5462997f0)
- [x] _  (line 56, body 0daf643c44af)
- [x] _GL_ATTRIBUTE_NOTHROW  (line 82, body da39a3ee5e6b)
- [x] _GL_WINDOWS_STAT_TIMESPEC  (line 172, body 356a192b7913)
- [x] _stat32  (line 875, body 89136c37b6dc)
- [x] _stat32i64  (line 865, body 89136c37b6dc)
- [x] _stat64  (line 869, body 89136c37b6dc)
- [x] _stat64i32  (line 879, body 89136c37b6dc)
- [x] _stati64  (line 859, body 89136c37b6dc)
- [x] chmod  (line 465, body 88298a118fc2)
- [x] fchmodat  (line 480, body e6882885ce48)
- [x] fstat  (line 527, body 9ef5c334ac81)
- [x] fstatat  (line 564, body 3539dbbdbf79)
- [x] futimens  (line 582, body 7f9e8947588f)
- [x] lstat  (line 930, body 5fbb7bcb405d)
- [x] mkdir  (line 686, body e7271a644470)
- [x] mkfifo  (line 720, body cf9691ca38b5)
- [x] mkfifoat  (line 745, body 8ea08bc6721a)
- [x] mknod  (line 772, body 9bd0b5f8983f)
- [x] mknodat  (line 797, body 50d9ced31f2b)
- [x] st_atime  (line 168, body f4103b5cf914)
- [x] st_ctime  (line 170, body 5797dc79d77c)
- [x] st_mtime  (line 169, body f1d6715b5cfe)
- [x] stat  (line 883, body 89136c37b6dc)
- [x] stat64  (line 834, body 89136c37b6dc)
- [x] umask  (line 947, body 9fbd57c76a6a)
- [x] utimensat  (line 966, body 2b3cfdc16047)

## Types and their fields — 1
- [x] stat: dev_t st_dev, ino_t st_ino, mode_t st_mode, nlink_t st_nlink, short st_uid, short st_gid, dev_t st_rdev, off_t st_size, struct timespec st_atim, struct timespec st_mtim, struct timespec st_ctim, time_t st_atime, time_t st_mtime, time_t st_ctime

## File-scope objects — 0

## Decisions

`emacs/lib/sys_stat.in.h` @ `fd7afced403`; port `femacs/lib/include/sys_stat.in.rs`
(290 lines).  Every checklist item is ticked and `extract-abi.py --check` reports `OK` for the
pair with no row needed in `docs/ports/excluded-macros.txt`.

**The template.**  Like `unistd.in.h` this is a gnulib template, so it stays off
`docs/baseline/config/cond-eval-verified.txt` and the extractor reads every `#if` arm as live
(P-66).  The frozen substitution is the generated `build/emacs-ref/lib/sys/stat.h`, and the
live text was measured by preprocessing it with the frozen compiler:

```
$ printf '#include <sys/stat.h>\n' > /tmp/s.c
$ clang -E -dD -I build/emacs-ref/lib -I build/emacs-ref/src -I emacs/src -I emacs/lib \
    -include config.h /tmp/s.c | grep '^#define'
#define S_IFMT 0170000    #define S_IFIFO 0010000   #define S_IFDIR 0040000
#define S_ISUID 04000     #define S_ISGID 02000     #define S_ISVTX 01000
...                   #define st_atimespec st_atim  ...   <- the system header's own names
```

So the live header is the system `<sys/stat.h>` the template includes next (its C 52) plus
nothing of gnulib's own: every `@REPLACE_*@` arm of the frozen configuration is 0, the
`@GNULIB_OVERRIDES_STRUCT_STAT@` arm (the one that would define a *second* `struct stat`) is 0,
and the `@WINDOWS_STAT_TIMESPEC@`/`WINDOWS_64_BIT_ST_SIZE` markers are 0.  The mirror plays
both halves as the other `.in.rs` mirrors do: the system entries the ported units name are
declared, and every name the template can define is carried - the dead arms as text under the
feature naming the C condition (P-51), since the extractor asks for each name.

**`struct stat`.**  No reference file in the tree defines this target's `stat`; it is measured
from the frozen target (the clang probe in the mirror's header comment gives `sizeof` 144,
alignment 8 and every offset), and the mirror's `const _` assertions pin those numbers.  C's
template *also* defines a `struct stat` of its own - the `@GNULIB_OVERRIDES_STRUCT_STAT@` arm
(C 118-175), dead here - whose members differ: `struct timespec st_atim/st_mtim/st_ctim` in its
`@WINDOWS_STAT_TIMESPEC@` arm and `time_t st_atime/st_mtime/st_ctime` in the `#else` arm.  The
port has one type named `stat`, so that arm's six members are carried as `#[cfg]`-gated fields
under the two features (`windows_stat_timespec` / `no_windows_stat_timespec`) - text for the
blind extractor, absent from the measured layout, which the assertions still check.

| item (C line) | shape | why |
| --- | --- | --- |
| `S_IFMT`, `S_IFDIR`…`S_IFSOCK`, `S_ISUID`/`S_ISGID`/`S_ISVTX`, `S_IRUSR`/`S_IWUSR`/`S_IXUSR` | `pub const … : mode_t = 0o…` | the system header's values (measured above); C's `#ifndef` arms are dead |
| `S_IRGRP`/`S_IROTH`/`S_IWGRP`/`S_IWOTH`/`S_IXGRP`/`S_IXOTH` (C 372-405) and `S_IRWXU`/`S_IRWXG`/`S_IRWXO` (C 408-415) | `pub const … = S_IRUSR >> 3` … `S_IRUSR \| S_IWUSR \| S_IXUSR` | C's body is an *expression*, and the extractor compares a body's tokens, so the mirror spells C's own expression rather than the literal it equals |
| `S_IXUGO` (C 420), `S_IRWXUGO` (C 423) | `pub const … = S_IXUSR \| S_IXGRP \| S_IXOTH` / `S_IRWXU \| S_IRWXG \| S_IRWXO` | the same, for the two masks gnulib adds for its apps |
| `S_IFIFO` (C 186-194) | the const above + `#[cfg (feature = "no_s_ififo")] macro_rules! S_IFIFO { () => { _S_IFIFO } }` | the dead `#ifdef _S_IFIFO` rename; the const is the system value |
| `S_ISNAM`, `S_ISOFD`, `S_ISOFL` (C 276/341/346) | `macro_rules! … { ($m:expr) => { 0 }; }` | the dead fallbacks whose body is `0`, next to the twelve the mirror already had |
| `_GL_ATTRIBUTE_NOTHROW` (C 78-94) | `macro_rules! _GL_ATTRIBUTE_NOTHROW { () => {}; }` | the attribute the port drops (PORTING.md 0.3); C defines it five times, and `attribute.rs` 184 has the same macro - C's `#ifndef` guard makes that a second, identical definition, and `macro_rules!` re-definition is shadowing in Rust (measured) |
| `GNULIB_defined_struct_stat` (C 174) | `pub const … : c_int = 1` | the marker of the arm above |
| `st_atime`/`st_mtime`/`st_ctime` (C 168-170) | `#[cfg (feature = "windows_stat_timespec")] macro_rules! st_atime { () => { st_atim.tv_sec } }` … | the other `struct stat` arm's field renames; the extractor compares the tokens (`st_atim`, `tv_sec`) |
| `_GL_WINDOWS_STAT_TIMESPEC` (C 172) | `#[cfg (feature = "windows_stat_timespec")] pub const … : c_int = 1` | the arm's indicator |
| `rpl_mkdir` (C 656/678) | `#[cfg (feature = "windows")] macro_rules! rpl_mkdir { ($name, $mode) => { _mkdir ($name) } }` + `GNULIB_defined_rpl_mkdir` (C 662/684, `pub const … = 1`) | the two dead `static int rpl_mkdir (…) { return _mkdir (name); }` definitions of the native-Windows arms; `_mkdir` is not on this target, so the name is carried as C's call |
| `mkfifo`, `mkfifoat` (C 720/745) | `#[cfg (feature = "replace_mkfifo"/"replace_mkfifoat")] macro_rules! X { () => { rpl_X } }` | dead `@REPLACE_*@` renames, the shape the unistd mirror uses; C's single-token body is what the extractor compares |
| `stat64`, `_stati64`, `_stat32`, `_stat32i64`, `_stat64`, `_stat64i32` (C 834-879) | `#[cfg (feature = "aix_large_files"/"windows_64_bit_st_size"/"windows")] macro_rules! X { ($name, $st) => { rpl_stat ($name, $st) } }` | the `@REPLACE_STAT@` block's six platform renames, dead here; C's body is `rpl_stat (name, st)` in each |

Names the mirror does **not** repeat: `chmod`/`fstat`/`lstat`/`stat`/`fstatat`/`mkdir`/
`mknod`/`mknodat`/`futimens`/`utimensat`/`umask`/`fchmodat` (their `#define X rpl_X`/`_X` arms
are counted by the extractor but the mirror declares the *functions*), and the four
`S_IS*`-family macros C defines twice (`S_ISDIR`, `S_ISLNK`, …), whose bodies are not compared.

**Convergence (PORTING.md 0.1-3).**  Nothing to converge: no unit declared any name this
mirror added (`check-mirror-dupes.py` reports the same single ratchet row before and after).

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/sys_stat.in.h femacs/lib/include/sys_stat.in.rs
  (1 C macros have a recorded Rust counterpart: _)
  (26 C macros have several definitions (configuration branches), body not compared:
   GNULIB_defined_rpl_mkdir, S_IRUSR, S_ISBLK, S_ISCHR, S_ISDIR, S_ISFIFO ...)
OK emacs/lib/sys_stat.in.h -> femacs/lib/include/sys_stat.in.rs

$ cd /tmp/femacs-target-mir3 && rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort \
    sys_stat_probe.rs -o sys_stat_probe.o
rustc exit=0   # the probe asserts size_of::<stat>() == 144, the st_ino/st_size offsets, the
               # mode/权限 values, S_IXUGO 0o111 / S_IRWXUGO 0o777, UTIME_NOW/OMIT, the
               # GNULIB_defined_struct_stat marker, calls stat/lstat/fstat/fstatat/mkdir/chmod/
               # umask and expands S_ISREG!/S_ISNAM!/_GL_ATTRIBUTE_NOTHROW!

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir3 cargo run -q -p xtask -- build-tu <unit.c>
built lib/lstat.rs   built lib/fstatat.rs   built lib/filemode.rs
built src/fileio.rs  built src/emacs.rs                       # all exit 0

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
```

## 2026-10-12: the `windowsnt` arm's three-name collision (w32.c's Owed 14)

`fchmod`, `fchmodat` and `faccessat` are three names this header *declares* and `src/w32.c`
*defines* for the Windows arm (C 4757, 4763 and 4177).  A declaration at this scope is an
`E0428` against those definitions, which the `windowsnt` probe of `femacs/src/w32.rs` measured
as three located sites; the three now carry `#[cfg (not (feature = "windowsnt"))]` (the
`femacs/lib/include/unistd.in.rs` 67 `getwd` device).  Evidence: the same probe reads **119**
error lines after the gate (**122** before), the frozen probe is unchanged (**exit 0**,
**336-byte** empty object), and this mirror's own
`python3 tools/extract-abi.py --check emacs/lib/sys_stat.in.h femacs/lib/include/sys_stat.in.rs`
still reads **OK**.

