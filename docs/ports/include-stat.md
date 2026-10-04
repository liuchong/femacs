# emacs/nt/inc/sys/stat.h (175 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 50
- [x] INC_SYS_STAT_H_  (line 22, body da39a3ee5e6b)
- [x] S_IEXEC  (line 55, body 7232895c5fbc)
- [x] S_IFBLK  (line 48, body e3160804a958)
- [x] S_IFCHR  (line 49, body 3daa00115a66)
- [x] S_IFDIR  (line 47, body da533c139833)
- [x] S_IFIFO  (line 50, body 987a3bdd4907)
- [x] S_IFLNK  (line 51, body 96d5e6218d19)
- [x] S_IFMT  (line 44, body b933ffb807d2)
- [x] S_IFREG  (line 46, body 2cedb007191f)
- [x] S_IREAD  (line 53, body ecdf501256ce)
- [x] S_IRGRP  (line 73, body 6f14d4531dca)
- [x] S_IROTH  (line 74, body 00cc7bd4a8d2)
- [x] S_IRUSR  (line 57, body 6309002f578f)
- [x] S_IRWXU  (line 60, body 82f065e981e6)
- [x] S_ISBLK  (line 64, body b615d099257e)
- [x] S_ISCHR  (line 65, body 69501407b210)
- [x] S_ISCTG  (line 81, body b6589fc6ab0d)
- [x] S_ISDIR  (line 63, body 9ea2eb562718)
- [x] S_ISDOOR  (line 82, body b6589fc6ab0d)
- [x] S_ISFIFO  (line 66, body fbcaeb0d0da0)
- [x] S_ISGID  (line 71, body b6589fc6ab0d)
- [x] S_ISLNK  (line 67, body 3f49236f84ed)
- [x] S_ISMPB  (line 83, body b6589fc6ab0d)
- [x] S_ISMPC  (line 84, body b6589fc6ab0d)
- [x] S_ISMPX  (line 85, body b6589fc6ab0d)
- [x] S_ISNWK  (line 86, body b6589fc6ab0d)
- [x] S_ISPORT  (line 87, body b6589fc6ab0d)
- [x] S_ISREG  (line 62, body 06f92e7f7e4c)
- [x] S_ISSOCK  (line 80, body b6589fc6ab0d)
- [x] S_ISUID  (line 70, body b6589fc6ab0d)
- [x] S_ISVTX  (line 72, body b6589fc6ab0d)
- [x] S_ISWHT  (line 88, body b6589fc6ab0d)
- [x] S_IWGRP  (line 75, body 8f8092f9b1c3)
- [x] S_IWOTH  (line 76, body e66e49c63a9d)
- [x] S_IWRITE  (line 54, body 3be01b55ecdc)
- [x] S_IWUSR  (line 58, body b0c3a6e5613c)
- [x] S_IXGRP  (line 77, body f47b98f25206)
- [x] S_IXOTH  (line 78, body 056d68d3df12)
- [x] S_IXUSR  (line 59, body dba1956c48c3)
- [x] S_TYPEISMQ  (line 89, body b6589fc6ab0d)
- [x] S_TYPEISSEM  (line 90, body b6589fc6ab0d)
- [x] S_TYPEISSHM  (line 91, body b6589fc6ab0d)
- [x] S_TYPEISTMO  (line 92, body b6589fc6ab0d)
- [x] UTIME_NOW  (line 94, body eb883130e83d)
- [x] UTIME_OMIT  (line 95, body e5e5462997f0)
- [x] _INC_STAT_H  (line 38, body da39a3ee5e6b)
- [x] _STAT_DEFINED  (line 154, body da39a3ee5e6b)
- [x] _SYS_STAT_H  (line 35, body da39a3ee5e6b)
- [x] _WSTAT_DEFINED  (line 161, body da39a3ee5e6b)
- [x] __MINGW_NOTHROW  (line 30, body da39a3ee5e6b)

## Types and their fields — 3
- [x] _stat: _dev_t st_dev, _ino_t st_ino, _mode_t st_mode, short st_nlink, short st_uid, short st_gid, _dev_t st_rdev, _off_t st_size, time_t st_atime, time_t st_mtime, time_t st_ctime
- [x] _stati64: _dev_t st_dev, _ino_t st_ino, _mode_t st_mode, short st_nlink, short st_uid, short st_gid, _dev_t st_rdev, __int64 st_size, time_t st_atime, time_t st_mtime, time_t st_ctime
- [x] stat: unsigned __int64 st_ino, dev_t st_dev, unsigned short st_mode, short st_nlink, unsigned st_uid, unsigned st_gid, unsigned __int64 st_size, dev_t st_rdev, time_t st_atime, time_t st_mtime, time_t st_ctime, char st_uname, char st_gname, int st_atimensec, int st_mtimensec, int st_ctimensec

## File-scope objects — 1
- [x] extern int w32_stat_get_owner_group  (line 151)
## Decisions

Mirror of nt/inc/sys/stat.h, complete for this target: the three include-guard constants,
`__MINGW_NOTHROW`, `_SYS_STAT_H`/`_INC_STAT_H`, the 20 `S_*` constants, the six `S_IS*`
predicates, the nine other mode macros (all `0`), `UTIME_NOW`/`UTIME_OMIT`, the three structs
with every field in C's order, `_STAT_DEFINED`/`_WSTAT_DEFINED` and the six prototypes plus
`w32_stat_get_owner_group` in `stat_decls`.

**Which arm**: the file is not on `docs/baseline/config/cond-eval-verified.txt`, so no `#if` in it
is resolved: the `__MINGW32__` (`<mingw.h>`), `__MINGW_NOTHROW`, `_SYS_STAT_H`, `_INC_STAT_H` and
`__MSVCRT__` (the `struct _stati64` block) arms are all live text and every name is carried once.
No name is defined twice.

**Conversions**: `#define S_IRUSR S_IREAD` and the `>> 3`/`>> 6` group/other bits keep their C
initialisers, so the value and every token stay visible; the six `S_IS*` predicates and the
thirteen constant-`0` predicates are `macro_rules!` with C's body; the `(-1)`/`(-2)` selectors are
the `-1`/`-2` literals (the same tokens).

**Names with no mirror**: `dev_t`, `time_t`, `_dev_t`, `_ino_t`, `_mode_t`, `_off_t` and `struct
timespec` are the MinGW runtime's and MSVCRT's own type spellings, so they stay as C spells them
and are named from the including unit (`nt/inc/sys/include/acl.rs`'s `ssize_t` shape); the
`unsigned __int64` fields (`st_ino`, `st_size`) are spelled `c_ulonglong` and `_stati64`'s
`__int64 st_size` as `c_longlong`, PORTING.md 1.1's mapping for MSVC's 64-bit builtin (the same
choice `nt/inc/include/dirent.rs` records).  `__cdecl`/`__MINGW_NOTHROW` on the prototypes are
convention spellings with no Rust counterpart: the port writes them in an `unsafe extern "C"`
block.

Nothing in the frozen configuration includes a `nt/inc/` header (no `nt/` or `msdos/` translation
unit is compiled - `docs/baseline/frozen-build-sources.txt`'s batch-F section records the
derivation - and no `src/` unit includes one), so `xtask layout-check`,
which compiles the `src/include` and `lib/include` mirrors, does not reach it; in a MinGW build
`nt/inc` is on the include path instead.
Evidence: `python3 tools/extract-abi.py --check emacs/nt/inc/sys/stat.h
femacs/nt/inc/sys/include/stat.rs` -> `OK`.
