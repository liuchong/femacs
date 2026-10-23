# emacs/lib/fchmodat.c (159 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 2
- [x] orig_fchmodat  (line 31, static)
- [x] fchmodat  (line 65)

## Macros defined — 2
- [x] AT_EMPTY_PATH  (line 52, body b6589fc6ab0d)
- [x] __need_system_sys_stat_h  (line 22, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0


## Decisions

`emacs/lib/fchmodat.c` @ `fd7afced403`; port `lib/fchmodat.rs` (110 lines).  All
checklist items ticked; `build-tu lib/fchmodat.c` → `built lib/fchmodat.rs ->
../build/femacs/fchmodat.o`, `--check` → `OK`.

- **Which arm is live.**  `HAVE_FCHMODAT` is 1, so `orig_fchmodat` (C 31-34) and the plain
  `fchmodat` (C 64-153) are the live text; C 155-159's `#else` arm - the at-func.c template
  with `AT_FUNC_NAME` = `fchmodat`, `AT_FUNC_F1`/`F2` = `lchmod`/`chmod` - is dead and is not
  written out (P-51).  Inside the live function the `HAVE_NEARLY_WORKING_FCHMODAT` arm (the
  trailing-slash ENOTDIR check) and the `NEED_FCHMODAT_NONSYMLINK_FIX` arm (the
  `O_PATH`/`/proc/self/fd` non-symlink fix built on openat/issymlinkat/sprintf) are both dead;
  what remains is the `AT_EMPTY_PATH` argument handling and the `orig_fchmodat` call.  Hence
  `line-drift-tolerance.tsv` records `fchmodat 0.40` (C 159 vs the port's 110, 30.8%).
- **The frozen build does not compile this file**: `build/emacs-ref/lib/fchmodat.o` does not
  exist (HAVE_FCHMODAT is 1 and no replacement is needed), so the reference object could not
  be used to cross-check the symbol names; the evidence for the live arm is the frozen
  `sys/stat.h`, whose `# undef fchmodat / # define fchmodat rpl_fchmodat` pair sits inside
  `# if 0` (i.e. no rename: C's own symbol is the plain `fchmodat`).
- **The system entry.**  C 22's `#define __need_system_sys_stat_h` makes C 26's first
  `<sys/stat.h>` the *system* one, so `orig_fchmodat` calls the system `fchmodat`; the port
  keeps that entry in `mod system` (`fchmodat`, `fchmod`), because the sys_stat mirror
  declares `fchmodat` at the crate root and this unit defines the same name (two items with
  one name in a module are E0428 - the readlink.rs shape).  The system `fchmod` is reached the
  same way in the `AT_EMPTY_PATH` branch.
- `AT_EMPTY_PATH` (the file's own `#ifndef`, 0) and `__need_system_sys_stat_h` (empty body) are
  the two macros of the checklist; no token exemptions were needed.
- Not verified: gate-fast/gate-probes, ERT, and any behavioral test of the AT_EMPTY_PATH
  handling.
