# emacs/lib/fcntl.c (663 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 3
- [x] fcntl  (line 202)
- [x] rpl_fcntl_DUPFD  (line 444, static)
- [x] rpl_fcntl_DUPFD_CLOEXEC  (line 485, static)

## Macros defined — 0

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/fcntl.c` @ `fd7afced403`; port `lib/fcntl.rs` (247 lines).  All checklist items
ticked; `build-tu lib/fcntl.c` -> `built lib/fcntl.rs -> ../build/femacs/fcntl.o` (0 errors),
`--check` -> `OK`.

- **Which arm is live** (the file is VERIFIED, 66 conditions).  `HAVE_FCNTL` is 1, so
  `rpl_fcntl_DUPFD`'s and `rpl_fcntl_DUPFD_CLOEXEC`'s `#if !HAVE_FCNTL` arms are dead;
  `FCNTL_DUPFD_BUGGY`/`REPLACE_FCHDIR` are undefined, so `rpl_fcntl_DUPFD`'s
  `#elif FCNTL_DUPFD_BUGGY || REPLACE_FCHDIR` arm (the `getdtablesize` check plus the
  F_GETFD/F_DUPFD/F_SETFD dance) and the `#  if REPLACE_FCHDIR` tail inside
  `rpl_fcntl_DUPFD_CLOEXEC` are dead; `__NetBSD__`/`__HAIKU__` are undefined, so that arm's
  `# define have_dupfd_cloexec -1` is dead; `GNULIB_defined_F_DUPFD_CLOEXEC` is 0, so
  `have_dupfd_cloexec` starts at 0; and C 431-433's `#else errno = EINVAL;` is dead.  Each is
  named at its site (P-51).
- **Whole dead regions**: C 33-172's `#if defined _WIN32` arm (its `dupfd` helper, needing
  `<windows.h>`/`<io.h>`/`<msvc-nothrow.h>`) and C 545-663's `#ifdef __KLIBC__` arm
  (`klibc_dupdirfd`, `klibc_fcntl`, needing `<emx/io.h>`/`<InnoTekLIBC/backend.h>`) are not
  written out - the gettimeofday.c-class treatment - and neither is C 228-247's
  `#if !HAVE_FCNTL` `case F_GETFD` (the `GetHandleInformation`/`dup2` pair).  Hence
  `line-drift-tolerance.tsv` records `fcntl 0.65` (C 663 vs 247, 62.7%).
- **The inner `switch`.**  C writes 52 `#ifdef F_*` / `case F_*` / `#endif` triples; the frozen
  target's `<fcntl.h>` defines 30 of the commands (measured with the clang probe now in
  `include/fcntl.in.rs`, which also lists the 22 names this target does *not* define), and the
  port writes them as two multi-pattern `match` arms - no-argument (13 commands) and
  `int`-argument (15) - plus the `_` arm for the pointer argument.  C's `switch`/`case`/
  `default`/`break` spellings and the `va_list arg; va_start (arg, action); … va_end (arg);`
  machinery are recorded in `docs/ports/macro-body-notes.tsv` (the function-body row for
  `fcntl`), and the two `va_arg` types are recorded in `docs/ports/va-arg-types.tsv`
  (`int` -> `c_int`, `void *` -> `*mut c_void`, the new row).
- **`#undef fcntl` / the native entry.**  C 177-206's `#undef fcntl` exists so that the
  definition's name is not the gnulib replacement macro *and* the body's `fcntl` names the
  native entry; the file's own comment (C 183-185) says the replacement "forwards all others on
  to the native fcntl".  The port keeps C's name on the definition (`#[unsafe(no_mangle)]`, so
  the object defines `_fcntl`) and reaches the native entry through `mod system` (the
  fchmodat.rs/open.rs shape) at the four call sites; a crate-root declaration would be E0428
  (PORTING.md 0.1-10).
- **ADJUDICATION ITEM: this file is not compiled by the frozen build.**  `REPLACE_FCNTL` is 0
  (the generated `build/emacs-ref/lib/fcntl.h` keeps C's `# define fcntl rpl_fcntl` inside
  `# if 0`), so there is no `build/emacs-ref/lib/fcntl.o` and the object's symbol set cannot be
  cross-checked.  Under that configuration C's own text defines `fcntl` and its body's `fcntl`
  calls would resolve to that same definition (a self-call); under the configuration where
  gnulib actually compiles this file (`REPLACE_FCNTL` = 1) the definition is renamed
  `rpl_fcntl` and the body's `fcntl` is the native entry, which is what the port follows.  The
  two readings differ only in a configuration where the file is linked; the choice here (C's
  name + `mod system` for the native calls) is the fchmodat.rs precedent, and the reviewer may
  prefer `#[unsafe(export_name = "rpl_fcntl")]` instead - it passes the same name-based gate
  and avoids shadowing libc's `_fcntl` if the unit is ever linked.
- **Forward declarations (C 170-175)** have no Rust counterpart (PORTING.md 0.1-12); the
  definitions below are the single definition point.
- **Mirror additions**: `include/fcntl.in.rs` gained the 22 `F_*` commands this target defines
  beyond the six it already carried (`F_GETOWN`/`F_SETOWN` included), each measured by the
  clang probe recorded in the mirror; appended only, nothing reordered (the external grep for
  same-named local declarations - status 3.24 - found none).
- Not verified: gate-fast/gate-probes, ERT, and any behavioural test of the replacement (the
  frozen build does not compile it).
