# emacs/lib/sys_stat.in.h (995 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [ ] rpl_mkdir  (line 656, static)
- [ ] rpl_mkdir  (line 678, static)

## Macros defined — 71
- [ ] GNULIB_defined_rpl_mkdir  (line 682, body 356a192b7913)
- [ ] GNULIB_defined_struct_stat  (line 174, body 356a192b7913)
- [ ] S_IFIFO  (line 192, body 54c3f5b01ff2)
- [ ] S_IFMT  (line 197, body b0114ab87659)
- [ ] S_IRGRP  (line 375, body 6f14d4531dca)
- [ ] S_IROTH  (line 378, body 00cc7bd4a8d2)
- [ ] S_IRUSR  (line 372, body 793a3f8290c4)
- [ ] S_IRWXG  (line 411, body 9650316727dd)
- [ ] S_IRWXO  (line 414, body 84b31b84969b)
- [ ] S_IRWXU  (line 408, body 61dd832429af)
- [ ] S_IRWXUGO  (line 423, body 03709dd2c0d8)
- [ ] S_ISBLK  (line 218, body b6589fc6ab0d)
- [ ] S_ISCHR  (line 226, body b6589fc6ab0d)
- [ ] S_ISCTG  (line 336, body b6589fc6ab0d)
- [ ] S_ISDIR  (line 234, body b6589fc6ab0d)
- [ ] S_ISDOOR  (line 239, body b6589fc6ab0d)
- [ ] S_ISFIFO  (line 246, body b6589fc6ab0d)
- [ ] S_ISGID  (line 360, body 790776b09216)
- [ ] S_ISLNK  (line 254, body b6589fc6ab0d)
- [ ] S_ISMPB  (line 263, body b6589fc6ab0d)
- [ ] S_ISMPC  (line 264, body b6589fc6ab0d)
- [ ] S_ISMPX  (line 269, body b6589fc6ab0d)
- [ ] S_ISNAM  (line 276, body b6589fc6ab0d)
- [ ] S_ISNWK  (line 284, body b6589fc6ab0d)
- [ ] S_ISOFD  (line 341, body b6589fc6ab0d)
- [ ] S_ISOFL  (line 346, body b6589fc6ab0d)
- [ ] S_ISPORT  (line 289, body b6589fc6ab0d)
- [ ] S_ISREG  (line 296, body b6589fc6ab0d)
- [ ] S_ISSOCK  (line 304, body b6589fc6ab0d)
- [ ] S_ISUID  (line 357, body 5967c2cac272)
- [ ] S_ISVTX  (line 365, body 19ac7b498eca)
- [ ] S_ISWHT  (line 351, body b6589fc6ab0d)
- [ ] S_IWGRP  (line 388, body 8f8092f9b1c3)
- [ ] S_IWOTH  (line 391, body e66e49c63a9d)
- [ ] S_IWUSR  (line 385, body 1f06964b7094)
- [ ] S_IXGRP  (line 401, body f47b98f25206)
- [ ] S_IXOTH  (line 404, body 056d68d3df12)
- [ ] S_IXUGO  (line 420, body e0528301b62f)
- [ ] S_IXUSR  (line 398, body b53ebbb22df0)
- [ ] S_TYPEISMQ  (line 310, body b6589fc6ab0d)
- [ ] S_TYPEISSEM  (line 322, body b6589fc6ab0d)
- [ ] S_TYPEISSHM  (line 330, body b6589fc6ab0d)
- [ ] S_TYPEISTMO  (line 314, body b6589fc6ab0d)
- [ ] UTIME_NOW  (line 428, body eb883130e83d)
- [ ] UTIME_OMIT  (line 429, body e5e5462997f0)
- [ ] _  (line 56, body 0daf643c44af)
- [ ] _GL_ATTRIBUTE_NOTHROW  (line 82, body da39a3ee5e6b)
- [ ] _GL_WINDOWS_STAT_TIMESPEC  (line 172, body 356a192b7913)
- [ ] _stat32  (line 875, body 89136c37b6dc)
- [ ] _stat32i64  (line 865, body 89136c37b6dc)
- [ ] _stat64  (line 869, body 89136c37b6dc)
- [ ] _stat64i32  (line 879, body 89136c37b6dc)
- [ ] _stati64  (line 859, body 89136c37b6dc)
- [ ] chmod  (line 465, body 88298a118fc2)
- [ ] fchmodat  (line 480, body e6882885ce48)
- [ ] fstat  (line 527, body 9ef5c334ac81)
- [ ] fstatat  (line 564, body 3539dbbdbf79)
- [ ] futimens  (line 582, body 7f9e8947588f)
- [ ] lstat  (line 930, body 5fbb7bcb405d)
- [ ] mkdir  (line 686, body e7271a644470)
- [ ] mkfifo  (line 720, body cf9691ca38b5)
- [ ] mkfifoat  (line 745, body 8ea08bc6721a)
- [ ] mknod  (line 772, body 9bd0b5f8983f)
- [ ] mknodat  (line 797, body 50d9ced31f2b)
- [ ] st_atime  (line 168, body f4103b5cf914)
- [ ] st_ctime  (line 170, body 5797dc79d77c)
- [ ] st_mtime  (line 169, body f1d6715b5cfe)
- [ ] stat  (line 883, body 89136c37b6dc)
- [ ] stat64  (line 834, body 89136c37b6dc)
- [ ] umask  (line 947, body 9fbd57c76a6a)
- [ ] utimensat  (line 966, body 2b3cfdc16047)

## Types and their fields — 1
- [ ] stat: dev_t st_dev, ino_t st_ino, mode_t st_mode, nlink_t st_nlink, short st_uid, short st_gid, dev_t st_rdev, off_t st_size, struct timespec st_atim, struct timespec st_mtim, struct timespec st_ctim, time_t st_atime, time_t st_mtime, time_t st_ctime

## File-scope objects — 0

## Decisions (WIP, PORTING.md P-26)

`emacs/lib/sys_stat.in.h` @ `fd7afced403`; port `femacs/lib/include/sys_stat.in.rs`.

Deliberately partial.  The mirror carries the frozen target's measured `struct stat` (the
clang probe that produced every offset is in the file), the `S_IF*`/`S_IS*` family, the two
`UTIME_*` values and the declarations the ported `lib/*.c` units name (stat/lstat/fstat/
fstatat/chmod/fchmod/fchmodat/faccessat/mkdir/mkdirat/mknod/mknodat/futimens/utimensat/umask).
Every `REPLACE_*` macro is zero in the frozen configuration, so gnulib's `_GL_CXXALIAS_RPL`
arms are dead and the aliases resolve to the system entries the mirror declares.
The unchecked boxes are the rest of the 995-line reference: the `_GL_WARN_ON_USE` machinery,
the alias/warning macros and the declarations this mirror has not needed yet.
