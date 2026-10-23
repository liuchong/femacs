# emacs/lib/faccessat.c (98 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] orig_faccessat  (line 37, static)
- [x] rpl_faccessat  (line 56)

## Macros defined — 2
- [x] _GL_INCLUDING_UNISTD_H  (line 22, body da39a3ee5e6b)
- [x] access  (line 50, body 35dc44684e44)

## Types and their fields — 0

## File-scope objects — 0


## Decisions

`emacs/lib/faccessat.c` @ `fd7afced403`; port `lib/faccessat.rs` (121 lines).  All
checklist items ticked; `build-tu lib/faccessat.c` -> `built lib/faccessat.rs ->
../build/femacs/faccessat.o` (0 errors, 2 pre-existing warnings from the unistd mirror's
`read`/`write` declarations), `--check` -> `OK`.

- **Which arm is live.**  `HAVE_FACCESSAT` is 1, so `orig_faccessat` (C 36-40) and
  `rpl_faccessat` (C 55-78) are the live text.  C 80-98's `#else` arm - the at-func.c
  template with `AT_FUNC_NAME` = `faccessat`, `AT_FUNC_F1` = `euidaccess`, `AT_FUNC_F2` =
  `access`, `AT_FUNC_USE_F1_COND` = `AT_EACCESS` - is dead and is not expanded (P-51); its
  six `AT_FUNC_*` renames are kept as `#[cfg (feature = "no_faccessat")]` `macro_rules!`
  items (the fstatat.rs shape for the same template), and the `# include "at-func.c"`
  expansion (the save_cwd/fchdir/(access|euidaccess)/restore_cwd simulation, whose headers
  this target has not got) is named at its site rather than written out.  The feature name
  is the non-frozen side, `no_faccessat` (the `no_readlinkat`/`no_lstat`/`no_f_setfd`
  spelling).
- **`#ifndef HAVE_ACCESS` (C 46-51) is live**: the frozen table leaves `HAVE_ACCESS`
  undefined, so C's `# undef access` / `# define access euidaccess` pair applies.  It is the
  only live macro of the checklist besides the empty include marker, and the port keeps it
  as a `macro_rules!` twin with the same name and call shape (the dup2.rs `# define dup2 ...`
  shape, so the token `euidaccess` is compared like C's body).  Nothing in the live text
  calls it; `euidaccess` itself is not declared by any mirror in this configuration
  (`HAVE_EUIDACCESS` and `GNULIB_EUIDACCESS` are both undefined), exactly as in C.
- **`orig_faccessat` calls the *system* `faccessat`.**  C 22's `#define
  _GL_INCLUDING_UNISTD_H` before C 23's `<config.h>` makes C 26's `<unistd.h>` the system
  header only, so C's `faccessat` there is the system entry; the port reaches it through the
  sys_stat mirror's plain declaration, and this unit's own symbol is `rpl_faccessat`
  (`REPLACE_FACCESSAT` is not set, so no header renames the name).  `_GL_INCLUDING_UNISTD_H`
  is the empty-body constant; C 33's `#undef` has no counterpart because the `include!`
  graph names each mirror once (PORTING.md 0.1-4).
- **`struct stat st;`** (C 65) is written by `fstatat` before it is read, so the port starts
  it `std::mem::zeroed ()` (PORTING.md 1.5); the trailing-slash test uses `S_ISDIR!`, the
  mirror's copy of gnulib's macro.
- **Not compiled in the frozen build**: `HAVE_FACCESSAT` is 1 and no replacement is needed,
  so `build/emacs-ref/lib/faccessat.o` does not exist and the reference object could not be
  cross-checked; the evidence for the live arm is the frozen `unistd.h`/`<sys/stat.h>`
  configuration itself.
- **Line drift**: `line-drift-tolerance.tsv` records `faccessat 0.25` (C 98 vs the port's
  121, 23.5%), because the dead arm's `#[cfg]`-guarded text and the port's fixed
  header/provenance costs are large against a 98-line reference.
- Not verified: gate-fast/gate-probes, ERT, and any behavioural test of `rpl_faccessat`.
