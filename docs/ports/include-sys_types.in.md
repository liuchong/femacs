# emacs/lib/sys_types.in.h (138 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 12
- [ ] GNULIB_defined_blkcnt_t  (line 132, body 356a192b7913)
- [ ] GNULIB_defined_blksize_t  (line 124, body 356a192b7913)
- [ ] GNULIB_defined_dev_t  (line 84, body 356a192b7913)
- [ ] GNULIB_defined_ino_t  (line 104, body 356a192b7913)
- [ ] GNULIB_defined_off64_t  (line 68, body 356a192b7913)
- [ ] _  (line 49, body 1ff62f6206d8)
- [ ] _GL_INCLUDING_SYS_TYPES_H  (line 44, body da39a3ee5e6b)
- [ ] _GL_WINDOWS_64_BIT_OFF_T  (line 60, body 356a192b7913)
- [ ] _GL_WINDOWS_STAT_INODES  (line 110, body 92f878d71f0f)
- [ ] dev_t  (line 83, body 16a951da420a)
- [ ] ino_t  (line 103, body 11a0d16dc1c7)
- [ ] off_t  (line 57, body c7a965515c55)

## Types and their fields — 0

## File-scope objects — 0

## Decisions (WIP, PORTING.md P-26)

`emacs/lib/sys_types.in.h` @ `fd7afced403`; port `femacs/lib/include/sys_types.in.rs`.

Deliberately partial: it carries `off_t`, which `buffer.h` names (`modtime_size`).
The unchecked boxes are the rest of the header.
