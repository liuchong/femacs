# emacs/lib/stat-time.h (266 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 10
- [ ] get_stat_atime_ns  (line 71)
- [ ] get_stat_ctime_ns  (line 84)
- [ ] get_stat_mtime_ns  (line 97)
- [ ] get_stat_birthtime_ns  (line 110)
- [ ] _gl_make_timespec  (line 137, static)
- [ ] get_stat_atime  (line 148)
- [ ] get_stat_ctime  (line 159)
- [ ] get_stat_mtime  (line 170)
- [ ] get_stat_birthtime  (line 182)
- [ ] stat_time_normalize  (line 226)

## Macros defined — 6
- [ ] STAT_TIMESPEC  (line 61, body a480da412230)
- [ ] STAT_TIMESPEC_NS  (line 66, body cf3ba08b866f)
- [ ] STAT_TIMESPEC_OFFSETOF  (line 62, body 42d7c7703d6d)
- [ ] STAT_TIME_H  (line 21, body 356a192b7913)
- [ ] _GL_STAT_TIME_INLINE  (line 38, body a960df3ab827)
- [ ] _gl_make_timespec  (line 128, body f0b597be7693)

## Types and their fields — 0

## File-scope objects — 0

## Decisions (WIP, PORTING.md P-26)

`emacs/lib/stat-time.h` @ `fd7afced403`; port `femacs/lib/include/stat-time.rs`.

The header's nine `_GL_STAT_TIME_INLINE` functions are the mirror's ordinary Rust items
(PORTING.md 10.6/10.12); the `.c` wrapper `femacs/lib/stat-time.rs` keeps the
`_GL_STAT_TIME_INLINE`/`_GL_EXTERN_INLINE` relationship, which is why the qualifier's box is
unchecked here.  The frozen configuration defines HAVE_STRUCT_STAT_ST_ATIMESPEC_TV_NSEC and
HAVE_STRUCT_STAT_ST_BIRTHTIMESPEC_TV_NSEC, so `STAT_TIMESPEC (st, st_atim)` is
`(st)->st_atimespec` on this target: the port reads the `*timespec` members of
`sys_stat.in.rs`'s `struct stat` and the four private macros (STAT_TIMESPEC,
STAT_TIMESPEC_NS, STAT_TIMESPEC_OFFSETOF, _gl_make_timespec) are recorded in
`docs/ports/excluded-macros.txt`.  `stat_time_normalize` is the only function with a
configuration-dependent body and the darwin arm is the live one (a `ckd_add` spelled
`overflowing_add`, P-15).
