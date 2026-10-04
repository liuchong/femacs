# emacs/lib/fcntl.in.h (504 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 48
- [x] AT_EACCESS  (line 485, body 1b6453892473)
- [x] AT_FDCWD  (line 466, body c999ee6dae0f)
- [x] AT_NO_AUTOMOUNT  (line 490, body b6589fc6ab0d)
- [x] AT_REMOVEDIR  (line 476, body 356a192b7913)
- [x] AT_SYMLINK_FOLLOW  (line 481, body da4b9237bacc)
- [x] AT_SYMLINK_NOFOLLOW  (line 472, body 6124cb94c33d)
- [x] FD_CLOEXEC  (line 291, body 356a192b7913)
- [x] F_DUPFD  (line 306, body 356a192b7913)
- [x] F_DUPFD_CLOEXEC  (line 298, body 6b7093bcc3fe)
- [x] F_GETFD  (line 310, body da4b9237bacc)
- [x] GNULIB_defined_F_DUPFD_CLOEXEC  (line 302, body b6589fc6ab0d)
- [x] GNULIB_defined_O_CLOEXEC  (line 340, body b6589fc6ab0d)
- [x] GNULIB_defined_O_NONBLOCK  (line 382, body 356a192b7913)
- [x] GNULIB_defined_fcntl  (line 160, body 356a192b7913)
- [x] GNULIB_defined_rpl_fcntl  (line 154, body 356a192b7913)
- [x] O_ACCMODE  (line 430, body 5d6bbea86296)
- [x] O_BINARY  (line 448, body b6589fc6ab0d)
- [x] O_CLOEXEC  (line 337, body 6b7093bcc3fe)
- [x] O_DIRECT  (line 344, body b6589fc6ab0d)
- [x] O_DIRECTORY  (line 348, body be5cdd8812ea)
- [x] O_DSYNC  (line 352, body b6589fc6ab0d)
- [x] O_EXEC  (line 356, body 6b98d858d292)
- [x] O_IGNORE_CTTY  (line 360, body b6589fc6ab0d)
- [x] O_NDELAY  (line 364, body b6589fc6ab0d)
- [x] O_NOATIME  (line 368, body b6589fc6ab0d)
- [x] O_NOCTTY  (line 389, body b6589fc6ab0d)
- [x] O_NOFOLLOW  (line 393, body b6589fc6ab0d)
- [x] O_NOLINK  (line 397, body b6589fc6ab0d)
- [x] O_NOLINKS  (line 401, body b6589fc6ab0d)
- [x] O_NONBLOCK  (line 384, body 6b7093bcc3fe)
- [x] O_NOTRANS  (line 405, body b6589fc6ab0d)
- [x] O_RSYNC  (line 409, body b6589fc6ab0d)
- [x] O_SEARCH  (line 417, body 6b98d858d292)
- [x] O_SYNC  (line 421, body b6589fc6ab0d)
- [x] O_TEXT  (line 449, body b6589fc6ab0d)
- [x] O_TTY_INIT  (line 425, body b6589fc6ab0d)
- [x] RESOLVE_BENEATH  (line 266, body 432a3e71f41d)
- [x] RESOLVE_CACHED  (line 268, body a6d133efc0d7)
- [x] RESOLVE_IN_ROOT  (line 267, body 9042d4228c01)
- [x] RESOLVE_NO_MAGICLINKS  (line 264, body d6c81010612a)
- [x] RESOLVE_NO_SYMLINKS  (line 265, body cc35a7395782)
- [x] RESOLVE_NO_XDEV  (line 263, body 2113f7bfab26)
- [x] _  (line 84, body 25a93e24928c)
- [x] _GL_OPENAT_ESYMLINK  (line 499, body 1eeb27393bb4)
- [x] creat  (line 135, body 41f39a55ac99)
- [x] fcntl  (line 149, body 6d79a8dbbff0)
- [x] open  (line 207, body 9bbb60901d2d)
- [x] openat  (line 224, body a1762eb713ff)

## Types and their fields — 1
- [x] open_how: __UINT64_TYPE__ flags, __UINT64_TYPE__ mode, __UINT64_TYPE__ resolve, unsigned long long int flags, unsigned long long int mode, unsigned long long int resolve

## File-scope objects — 0

## Decisions

`emacs/lib/fcntl.in.h` @ `fd7afced403`; port `femacs/lib/include/fcntl.in.rs`.

**The template.**  `@GUARD_PREFIX@`, `@PRAGMA_SYSTEM_HEADER@`, `@INCLUDE_NEXT@`,
`@NEXT_FCNTL_H@` and the `@GNULIB_*@`/`@REPLACE_*@`/`@HAVE_*@` selectors cannot be evaluated by
`tools/check-cond-eval.py`, so the header stays off the verified list and `extract-abi.py`
reads every arm as live (P-66).  The frozen substitution is the generated
`build/emacs-ref/lib/fcntl.h`, with `@GNULIB_CREAT@` 0, `@GNULIB_FCNTL@` 1 /
`@REPLACE_FCNTL@` 0 / `@HAVE_FCNTL@` 1, `@GNULIB_OPEN@` 1 / `@REPLACE_OPEN@` 0,
`@GNULIB_OPENAT@` 0, `@GNULIB_OPENAT2@` 0 and `@GNULIB_NONBLOCKING@` 0.

**The frozen values are the preprocessor's, not the template's fallbacks.**
`docs/baseline/config/darwin-arm64.macros.tsv` is `clang -dM -E` over the frozen includes
(`tools/gen-cond-macros.py`), i.e. the macro table *after* gnulib's `<fcntl.h>` ran, so it
records the values the reference build uses.  Two rows show why the template alone is not
enough:

```
O_ACCMODE   (O_RDONLY | O_WRONLY | O_RDWR | O_EXEC | O_SEARCH)   # C 428-431's condition is
                                                                 # TRUE here, so gnulib
                                                                 # *redefines* it
O_SEARCH    (O_EXEC | O_DIRECTORY)                O_NDELAY  O_NONBLOCK
```

`O_ACCMODE` is therefore the one fallback that is live *and* whose text the extractor compares:
the mirror spells it with C's five-flag mask (0x40100003 as an `int`), which is both the frozen
value and the compared arm.

**Both halves.**  The mirror carries the system's values (from the table above: `O_NOFOLLOW`
0x100, `O_SYNC` 0x80, `O_CLOEXEC` 0x1000000, `O_DIRECTORY` 0x100000, `O_NOCTTY` 0x20000,
`O_DSYNC` 0x400000, `O_EXEC` 0x40000000, `O_SEARCH` `(O_EXEC | O_DIRECTORY)`, `F_DUPFD` 0,
`F_GETFD` 1, `F_DUPFD_CLOEXEC` 67, `FD_CLOEXEC` 1, `AT_FDCWD` -2, `AT_SYMLINK_NOFOLLOW` 0x20,
`AT_REMOVEDIR` 0x80, `AT_SYMLINK_FOLLOW` 0x40, `AT_EACCESS` 0x10, `_GL_OPENAT_ESYMLINK`
`ELOOP`) and the template's own `#ifndef` fallbacks, which are live for the names this target
lacks (`O_DIRECT` 0, `O_IGNORE_CTTY` 0, `O_NOATIME` 0, `O_NOLINK` 0, `O_NOLINKS` 0,
`O_NOTRANS` 0, `O_RSYNC` 0, `O_TTY_INIT` 0, `AT_NO_AUTOMOUNT` 0, `O_BINARY` 0, `O_TEXT` 0) and
dead for the names it has.  The dead fallbacks stay in the mirror as P-51 text under a feature
that names their own condition (`#ifndef O_NOFOLLOW` -> `feature = "no_o_nofollow"`), the way
lib/include/acl-internal.rs spells `MODE_INSIDE_ACL`'s three arms; the arm C's template spells
is written first and the frozen arm second, because the extractor's constant reader takes the
first definition of a name and so compares C's arm (the pair is therefore `OK`), while rustc
compiles only the ungated arm.

| C lines | name | template arm (feature) | frozen arm |
| --- | --- | --- | --- |
| 305-307 | `F_DUPFD` | 1 (`no_f_dupfd`) | 0 |
| 309-311 | `F_GETFD` | 2 (`no_f_getfd`) | 1 |
| 297-303 | `F_DUPFD_CLOEXEC` + `GNULIB_defined_F_DUPFD_CLOEXEC` | 0x40000000 / 1 (`no_f_dupfd_cloexec`) | 67 / 0 |
| 336-341 | `O_CLOEXEC` / `GNULIB_defined_O_CLOEXEC` | - (two template arms, body not compared) | 0x1000000 / 0 |
| 347-349 | `O_DIRECTORY` | 0x20000000 (`no_o_directory`) | 0x100000 |
| 351-353 | `O_DSYNC` | 0 (`no_o_dsync`) | 0x400000 |
| 355-357 | `O_EXEC` | `O_RDONLY` (`no_o_exec`) | 0x40000000 |
| 363-365 | `O_NDELAY` | 0 (`no_o_ndelay`) | `O_NONBLOCK` |
| 392-394 | `O_NOFOLLOW` | 0 (`no_o_nofollow`) | 0x100 |
| 416-418 | `O_SEARCH` | `O_RDONLY` (`no_o_search`) | `O_EXEC \| O_DIRECTORY` |
| 420-422 | `O_SYNC` | 0 (`no_o_sync`) | 0x80 |
| 388-390 | `O_NOCTTY` | 0 (`no_o_noctty`) | 0x20000 |
| 465-467 | `AT_FDCWD` | `(-3041965)` (`no_at_fdcwd`) | -2 |
| 471-473 | `AT_SYMLINK_NOFOLLOW` | 4096 (`no_at_symlink_nofollow`) | 0x20 |
| 475-477 | `AT_REMOVEDIR` | 1 (`no_at_removedir`) | 0x80 |
| 480-482 | `AT_SYMLINK_FOLLOW` | 2 (`no_at_symlink_follow`) | 0x40 |
| 484-486 | `AT_EACCESS` | 4 (`no_at_eaccess`) | 0x10 |

**Dead blocks carried as P-51 text** (their condition's non-frozen side is the feature):

- C 248-286's openat2 module (`GNULIB_OPENAT2` 0): `struct open_how` and the six `RESOLVE_*`
  values, under `feature = "gnulib_openat2"`.  The struct is the one "Types and their fields"
  box; its three `unsigned long long` fields are Rust `u64`, and C's `__UINT64_TYPE__` arm is
  the live one.
- C 145-171's `GNULIB_defined_rpl_fcntl` / `GNULIB_defined_fcntl` (`REPLACE_FCNTL` 0 /
  `HAVE_FCNTL` 1), under `feature = "replace_fcntl"` / `"no_have_fcntl"`.
- C 375-386's `GNULIB_defined_O_NONBLOCK`, under `feature = "gnulib_nonblocking"`;
  `# if O_NONBLOCK` is true on this target, so a target with the module would define the
  witness 0 (the template's last line, 1, is its dead inner `#else`).
- C 25-50's special-invocation arm, C 41/69's `<sys/stat.h>` include and C 435-445's
  `_O_BINARY`/BeOS arms have no text to carry: no preprocessor, and the arms are Windows/BeOS.

**Declarations.**  All four entries `creat`/`fcntl`/`open`/`openat` resolve to
`_GL_CXXALIAS_SYS` here, so they live in the whole-import module `fcntl_decls` (PORTING.md
0.1-10: a definition in `lib/cloexec.rs`/`lib/open.rs` shadows the glob).  `_GL_OPENAT_ESYMLINK`
keeps C's identifier `ELOOP` as its value, which is why the mirror includes the `<errno.h>`
mirror (C's arm relies on its includer having `<errno.h>`).

**Convergence (PORTING.md 0.1-3, same batch).**  Two use-site copies of names this mirror
added, both deleted with the consumer left reading the mirror:

| unit | copy | now |
| --- | --- | --- |
| `src/fcntl.rs` 49 | `pub const GNULIB_defined_F_DUPFD_CLOEXEC: c_int = 0;` (a "mirror gap", P-26) | the mirror's `#else` arm |
| `src/term.rs` 220 | `pub const O_IGNORE_CTTY: c_int = 0;` (a "mirror gap") | the mirror's live fallback |

`src/sysdep.rs` was *not* touched: its `O_IGNORE_CTTY` is a separate unit that does not include
this mirror (the ledger of that unit owns it).  No other unit spells the names this mirror adds.

## Evidence

```
$ python3 tools/extract-abi.py --check emacs/lib/fcntl.in.h femacs/lib/include/fcntl.in.rs
  (1 C macros have a recorded Rust counterpart: _)
  (10 C macros have several definitions (configuration branches), body not compared:
   GNULIB_defined_F_DUPFD_CLOEXEC, GNULIB_defined_O_CLOEXEC, GNULIB_defined_O_NONBLOCK,
   O_BINARY, O_CLOEXEC, O_NONBLOCK ...)
OK emacs/lib/fcntl.in.h -> femacs/lib/include/fcntl.in.rs

$ cd /tmp/femacs-probe2 && rustc --edition 2024 --crate-type=lib --emit=obj -C panic=abort \
    fcntl_probe.rs -o fcntl_probe.o
exit=0        # the probe include!s the mirror and asserts every frozen value: O_ACCMODE ==
              # O_RDONLY|O_WRONLY|O_RDWR|O_EXEC|O_SEARCH == 0x40100003, O_SEARCH ==
              # O_EXEC|O_DIRECTORY, O_NDELAY == O_NONBLOCK, O_NOFOLLOW 0x100, O_SYNC 0x80,
              # O_DIRECTORY 0x100000, O_NOCTTY 0x20000, O_DSYNC 0x400000, O_EXEC 0x40000000,
              # F_DUPFD 0 / F_GETFD 1 / F_DUPFD_CLOEXEC 67 / FD_CLOEXEC 1, AT_FDCWD -2 /
              # AT_SYMLINK_NOFOLLOW 32 / AT_REMOVEDIR 128 / AT_SYMLINK_FOLLOW 64 / AT_EACCESS
              # 16 / AT_NO_AUTOMOUNT 0, _GL_OPENAT_ESYMLINK == ELOOP, and the two witnesses 0

$ cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-mir2 cargo run -q -p xtask -- build-tu <unit.c>
built lib/fcntl.rs / lib/open.rs / lib/tempname.rs / lib/openat-proc.rs / lib/save-cwd.rs /
      lib/lchmod.rs / lib/utimens.rs / lib/pipe2.rs / lib/fstatat.rs / lib/fchmodat.rs /
      lib/binary-io.rs / lib/careadlinkat.rs / lib/canonicalize-lgpl.rs    # all exit 0
built src/term.rs -> ../build/femacs/term.o          # exit 0 (the edited consumer)
$ python3 tools/extract-abi.py --check emacs/<unit.c> femacs/<unit.rs>
OK emacs/lib/fcntl.c -> femacs/lib/fcntl.rs
OK emacs/src/term.c -> femacs/src/term.rs

$ python3 tools/check-mirror-dupes.py
1 duplicate declaration(s), 1 ratchet row(s), 0 failure(s)
$ python3 tools/check-declarations.py
13 declaration sites (0 forbidden, 13 allowed literals), 0 failures
$ python3 tools/check-verbatim.py
6 trees, 4607 files, 0 failures
$ python3 tools/check-globals.py
check-globals: 283 units, 0 DEFVAR_* variables without a definition in their own unit
```
