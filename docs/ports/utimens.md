# emacs/lib/utimens.c (690 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [x] is_valid_timespec  (line 84, static)
- [x] is_valid_timespecs  (line 92, static)
- [x] validate_timespec  (line 106, static)
- [x] update_timespec  (line 145, static)
- [x] fdutimens  (line 182)

## Macros defined — 2
- [x] REPLACE_FUNC_STAT_FILE  (line 64, body b6589fc6ab0d)
- [x] _GL_UTIMENS_INLINE  (line 22, body a9f37094e6e8)

## Types and their fields — 0

## File-scope objects — 2
- [x] static int utimensat_works_really  (line 79, static)
- [x] static int lutimensat_works_really  (line 80, static)


## Decisions

`emacs/lib/utimens.c` @ `fd7afced403`; port `lib/utimens.rs` (478 lines).  All
checklist items ticked; `build-tu lib/utimens.c` → `built lib/utimens.rs ->
../build/femacs/utimens.o`, `--check` → `OK`.

- **Conditional arms.**  `lib/utimens.c` is in `docs/baseline/config/cond-eval-verified.txt`,
  so the evaluator decides the arms and only the live text is compared.  Live for this
  target: `HAVE_UTIMENSAT`/`HAVE_FUTIMENS` (both 1), `HAVE_FUTIMES`/`HAVE_WORKING_UTIMES`
  (both 1), `__APPLE__ && __MACH__` (the trailing-slash ENOTDIR guard) and
  `REPLACE_FUNC_STAT_FILE` (the file's own `#ifndef` defines it 0).  Dead, and named at their
  C line numbers instead of being written out (P-51; `lib/utimens.c` is not a file whose
  ledger can carry dead text without losing the tokens the gate wants): the Windows
  `USE_SETFILETIME` path (`EMACS_CONFIGURATION` is defined), `HAVE_BUGGY_NFS_TIME_STAMPS`,
  the `__linux__`/`__sun`/`__NetBSD__` UTIME_OMIT preparatory stat, the `__linux__`
  ENOSYS-280 fixups, the `__linux__ && __GLIBC__` rounding fixup, `HAVE_FUTIMESAT` /
  `FUTIMESAT_NULL_BUG`, `HAVE_LUTIMES && !HAVE_UTIMENSAT`, the `errno = ENOSYS` guard and the
  `utime` fallback.  That is why `line-drift-tolerance.tsv` records `utimens 0.35` (C 690 vs
  478, 30.7%).
- **Gate inventory gap - needs adjudication.**  `utimens` (C 536) and `lutimens` (C 558) are
  *not* in the checklist: `tools/extract-abi.py` cannot pair their declarator with the body
  because C 538/560 puts `#undef utimens` / `#undef lutimens` between them (`fdutimens` and
  the four statics are listed, the two wrappers are not).  Emitting the C symbols therefore
  trips the "invented exported symbol" rule, so both bodies are ported faithfully but without
  `#[unsafe(no_mangle)]` (their Rust names are mangled and nothing calls them).  A tool fix
  (skip a `#undef` line between declarator and body) followed by adding the attribute is the
  one-line change that makes the pair complete.
- **`issymlink`** is `_GL_ISSYMLINK_INLINE` in issymlink.h - a header-inline definition with
  no symbol - so the port carries that definition (`#[inline(always)]`, P-12) in the unit;
  `include/issymlink.rs` is today only the guard, and `lib/issymlink.rs` is the
  `_GL_ISSYMLINK_INLINE` wrapper.
- **`lstat`** is gnulib-renamed to `rpl_lstat` in this build (measured: `nm -u
  build/emacs-ref/lib/utimens.o` lists `_rpl_lstat`), so the unit reaches it through
  `mod system` with `#[link_name = "rpl_lstat"]` (PORTING.md 10.13, the readlink.rs shape) -
  a crate-root `lstat` declaration would collide with the sys_stat mirror's.
- **`_GL_UTIMENS_INLINE`** (the header's spelling macro) is in
  `docs/ports/excluded-macros.txt` (the `_GL_TIMESPEC_INLINE` precedent); the unit's own
  `#define _GL_UTIMENS_INLINE _GL_EXTERN_INLINE` is `pub const _GL_UTIMENS_INLINE: () =
  _GL_EXTERN_INLINE;` (the c-ctype.rs shape).
- `REPLACE_FUNC_STAT_FILE` (the file's own `#ifndef` default) is a `pub const = 0`, and the
  two cache variables (`utimensat_works_really`, `lutimensat_works_really`) are `static mut`
  file-scope objects, as C has them.
- Not verified: gate-fast/gate-probes, ERT, any link test, and the runtime behaviour of the
  utimensat-workarounds (they cannot be exercised on this target's frozen configuration).
