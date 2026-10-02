# emacs/lib/getgroups.c (121 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] getgroups  (line 33)
- [x] rpl_getgroups  (line 69)

## Macros defined — 2
- [x] GETGROUPS_ZERO_BUG  (line 43, body b6589fc6ab0d)
- [x] getgroups  (line 58, body 58f23a5cdbb2)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The frozen configuration leaves `HAVE_GETGROUPS` undefined**, so C 28's `#if !HAVE_GETGROUPS`
  arm - the five-line stub that fails with ENOSYS - is the live text, and C 41-119's `#else` arm
  (the replacement for systems whose getgroups rejects N==0) is dead.  Its text is kept under
  `#[cfg (feature = "have_getgroups")]` (P-51): `GETGROUPS_ZERO_BUG`, the Apple declaration, the
  `getgroups` rename as a macro, and `rpl_getgroups` itself, translated statement for statement.
- **The `getgroups` rename needed three spellings.**  C 41 `#undef getgroups`, C 57-58's
  `posix_getgroups (...) __asm ("_getgroups")` and `#define getgroups posix_getgroups`, and the
  calls inside `rpl_getgroups`: the port puts the system entry in its own `system` module (with
  `#[link_name = "_getgroups"]` for the Apple arm, the category.rs shape for a name this unit
  also defines), and records C's rename as a `macro_rules!` whose body holds its expansion.
- **`gid_t` joined the sys/types mirror in the previous round**, and `GETGROUPS_T` joined it
  here (gid_t on this target); `EINVAL` (22) and `ENOSYS` (78) joined the errno mirror with the
  values measured from the frozen clang, and `SIZE_MAX` comes from the stdint mirror.
- **`malloc`/`free` are declared in the unit** (C 25's `<stdlib.h>`, as in the other gnulib
  units), and `<unistd.h>` has no mirror, so the live arm's own name is the one this unit defines.
- **The line-count drift (C 121 vs the port's 139, 14.9%) is declared** in
  docs/ports/line-drift-tolerance.tsv with this file's reason.
- **Call-relationship differences (D7)**: none beyond the system entry above.
