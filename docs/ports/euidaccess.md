# emacs/lib/euidaccess.c (217 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 1
- [x] euidaccess  (line 78)

## Macros defined — 2
- [x] EACCESS  (line 46, body c647bb6f149b)
- [x] __set_errno  (line 42, body 4465ad0435a1)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

`emacs/lib/euidaccess.c` @ `fd7afced403`; port `femacs/lib/euidaccess.rs` (263 lines).  All
checklist items ticked; `build-tu lib/euidaccess.c` -> `built lib/euidaccess.rs ->
../build/femacs/euidaccess.o` (0 errors), `--check` -> `OK`.

- **The file is on the verified list** (`docs/baseline/config/cond-eval-verified.txt`, which has
  grown to 427 files), so the gate evaluates its `#if`s: `_LIBC` is undefined, `HAVE_FACCESSAT`
  is 1 (C 80-81's `return faccessat (AT_FDCWD, file, mode, AT_EACCESS);` is the live body),
  `HAVE_EACCESS` is undefined and the `#ifdef TEST` program is dead - which is why the checklist
  carries one function and two macros.  The frozen build still does not compile the file
  (`HAVE_EUIDACCESS` is undefined and no replacement is needed, so there is no
  `build/emacs-ref/lib/euidaccess.o` and no object to compare symbols with).  For fidelity the
  port also keeps the five non-frozen arms (Interix `EFF_ONLY_OK`, AIX `accessx`/`ACC_SELF`,
  FreeBSD `eaccess`, mingw `access` and C 90-190's traditional uid/gid model) as `#[cfg]` text
  (P-51), the `#ifdef TEST` `main`, and C 57-69's eight `#ifdef _LIBC` rename macros - none of
  which the checklist requires.  `#[cfg]` text is never compiled, so the entries this target has
  not got cost nothing: `accessx`, `EFF_ONLY_OK`, `ACC_SELF`, `setreuid`, `setregid`, and the
  `group_member` that lives in another unit.
- **`root-uid.h`** (C 33) has no mirror; its whole content is `#define ROOT_UID 0`, carried as
  the `ROOT_UID` constant above the function.
- **The 14 macros.**  `EACCESS` is the file's own spelling of `EACCES`; `F_OK`/`X_OK`/`W_OK`/
  `R_OK` are the fallback values this target's `<unistd.h>` defines anyway; `__set_errno` is
  C 41-43's fallback (this file does not include `<libc-config.h>`); and C 57-69's
  `#ifdef _LIBC` rename block - the non-frozen side - is kept as eight `macro_rules!` twins that
  expand to the glibc-internal names C spells (`__access`, `__getuid`, ..., `__euidaccess`,
  `stat64`), which is what the blind reading asks for and what nothing invokes.
- **`main`** (C 203) is kept under `#[cfg (feature = "test")]` with its own declarations for
  `abort`/`atoi`/`printf`/`exit`/`error`; the checker reads this arm's declaration as *static*
  (a `#[unsafe(no_mangle)]` on it is reported as an invented symbol), so this file's `main`
  carries no attribute - group-member.c's arm has the opposite reading, and each ledger records
  its own.
- Line drift: `line-drift-tolerance.tsv` records `euidaccess 0.25` (C 217 vs 260, 19.8%).
- Not verified: gate-fast/gate-probes, ERT, and any behavioural test.
