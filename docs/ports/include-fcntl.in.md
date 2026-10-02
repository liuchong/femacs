# emacs/lib/fcntl.in.h (504 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 48
- [ ] AT_EACCESS  (line 485, body 1b6453892473)
- [ ] AT_FDCWD  (line 466, body c999ee6dae0f)
- [ ] AT_NO_AUTOMOUNT  (line 490, body b6589fc6ab0d)
- [ ] AT_REMOVEDIR  (line 476, body 356a192b7913)
- [ ] AT_SYMLINK_FOLLOW  (line 481, body da4b9237bacc)
- [ ] AT_SYMLINK_NOFOLLOW  (line 472, body 6124cb94c33d)
- [ ] FD_CLOEXEC  (line 291, body 356a192b7913)
- [ ] F_DUPFD  (line 306, body 356a192b7913)
- [ ] F_DUPFD_CLOEXEC  (line 298, body 6b7093bcc3fe)
- [ ] F_GETFD  (line 310, body da4b9237bacc)
- [ ] GNULIB_defined_F_DUPFD_CLOEXEC  (line 302, body b6589fc6ab0d)
- [ ] GNULIB_defined_O_CLOEXEC  (line 340, body b6589fc6ab0d)
- [ ] GNULIB_defined_O_NONBLOCK  (line 382, body 356a192b7913)
- [ ] GNULIB_defined_fcntl  (line 160, body 356a192b7913)
- [ ] GNULIB_defined_rpl_fcntl  (line 154, body 356a192b7913)
- [ ] O_ACCMODE  (line 430, body 5d6bbea86296)
- [ ] O_BINARY  (line 448, body b6589fc6ab0d)
- [ ] O_CLOEXEC  (line 337, body 6b7093bcc3fe)
- [ ] O_DIRECT  (line 344, body b6589fc6ab0d)
- [ ] O_DIRECTORY  (line 348, body be5cdd8812ea)
- [ ] O_DSYNC  (line 352, body b6589fc6ab0d)
- [ ] O_EXEC  (line 356, body 6b98d858d292)
- [ ] O_IGNORE_CTTY  (line 360, body b6589fc6ab0d)
- [ ] O_NDELAY  (line 364, body b6589fc6ab0d)
- [ ] O_NOATIME  (line 368, body b6589fc6ab0d)
- [ ] O_NOCTTY  (line 389, body b6589fc6ab0d)
- [ ] O_NOFOLLOW  (line 393, body b6589fc6ab0d)
- [ ] O_NOLINK  (line 397, body b6589fc6ab0d)
- [ ] O_NOLINKS  (line 401, body b6589fc6ab0d)
- [ ] O_NONBLOCK  (line 384, body 6b7093bcc3fe)
- [ ] O_NOTRANS  (line 405, body b6589fc6ab0d)
- [ ] O_RSYNC  (line 409, body b6589fc6ab0d)
- [ ] O_SEARCH  (line 417, body 6b98d858d292)
- [ ] O_SYNC  (line 421, body b6589fc6ab0d)
- [ ] O_TEXT  (line 449, body b6589fc6ab0d)
- [ ] O_TTY_INIT  (line 425, body b6589fc6ab0d)
- [ ] RESOLVE_BENEATH  (line 266, body 432a3e71f41d)
- [ ] RESOLVE_CACHED  (line 268, body a6d133efc0d7)
- [ ] RESOLVE_IN_ROOT  (line 267, body 9042d4228c01)
- [ ] RESOLVE_NO_MAGICLINKS  (line 264, body d6c81010612a)
- [ ] RESOLVE_NO_SYMLINKS  (line 265, body cc35a7395782)
- [ ] RESOLVE_NO_XDEV  (line 263, body 2113f7bfab26)
- [ ] _  (line 84, body 25a93e24928c)
- [ ] _GL_OPENAT_ESYMLINK  (line 499, body 1eeb27393bb4)
- [ ] creat  (line 135, body 41f39a55ac99)
- [ ] fcntl  (line 149, body 6d79a8dbbff0)
- [ ] open  (line 207, body 9bbb60901d2d)
- [ ] openat  (line 224, body a1762eb713ff)

## Types and their fields — 1
- [ ] open_how: __UINT64_TYPE__ flags, __UINT64_TYPE__ mode, __UINT64_TYPE__ resolve, unsigned long long int flags, unsigned long long int mode, unsigned long long int resolve

## File-scope objects — 0

## Decisions (WIP, PORTING.md P-26)

`emacs/lib/fcntl.in.h` @ `fd7afced403`; port `femacs/lib/include/fcntl.in.rs`.

Deliberately partial.  Values are measured from the frozen target (the clang probe is in the
file): the `O_*` open flags, the `F_*`/`FD_CLOEXEC` commands and the `AT_*` family - every one
of which the *system* header defines, so gnulib's `#ifndef` fallbacks (`-3041965`, `4096`, `1`,
`4`) and its `REPLACE_*` alias arms are dead.  `AT_EMPTY_PATH` is not defined by this target at
all, which is why `lib/fstatat.c` defines it itself.  The unchecked boxes are the rest of the
reference header.
