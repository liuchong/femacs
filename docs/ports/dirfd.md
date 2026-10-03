# emacs/lib/dirfd.c (44 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] dirfd  (line 30)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The two arms of C 32's `#if GNULIB_defined_DIR` are both carried** (blind file, P-66): the
  frozen configuration does not define `GNULIB_defined_DIR`, so C 38-42's `DIR_TO_FD` arm is
  the live one and C 33-36 is dead.  Its header (`lib/dirent-private.h`) is not in this build,
  so the dead arm's two tokens (`fd_to_close`, `EINVAL`) are *named at the site* rather than
  compiled (P-51; the tempname.rs precedent for text whose header the target has not got).
- `DIR_TO_FD` is `<dirent.h>`'s object-like macro (docs/baseline/config/darwin-arm64.macros.tsv
  757: `(Dir_p) -1`), kept as a `macro_rules!` (P-37) whose body is C's expansion; its own
  parameter name `Dir_p` is the macro's argument, which the checker exempts.
- `dirfd` is a non-`static` definition, so the port exports it (`#[unsafe(no_mangle)]`, P-52).
- **Mirror edit**: `femacs/lib/include/dirent.rs` now keeps its declarations in a
  uniquely-named `pub mod dirent_decls` + glob import, because this unit *defines* `dirfd`, a
  name that mirror declares — a crate-root definition and a crate-root declaration collide
  (E0428; PORTING.md 0.1-10, the utimens.rs/stdio.rs shape).
- Line-count drift (C 44 vs the port's 55) is declared in docs/ports/line-drift-tolerance.tsv.
- Call-relationship differences (D7): none.
