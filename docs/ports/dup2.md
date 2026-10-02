# emacs/lib/dup2.c (205 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 5
- [x] dup2_nothrow  (line 49, static)
- [x] ms_windows_dup2  (line 71, static)
- [x] klibc_dup2dirfd  (line 109, static)
- [x] klibc_dup2  (line 156, static)
- [x] rpl_dup2  (line 177)

## Macros defined — 3
- [x] WIN32_LEAN_AND_MEAN  (line 33, body da39a3ee5e6b)
- [x] dup2  (line 173, body 984bbed9a248)
- [x] dup2_nothrow  (line 67, body c56b37f413ae)

## Types and their fields — 0

## File-scope objects — 0


## Decisions

`emacs/lib/dup2.c` @ `fd7afced403`; port `femacs/lib/dup2.rs`.  All checklist items ticked;
`build-tu` → `built dup2.rs -> build/femacs/dup2.o`, `--check` → `OK`.

- **Live arm.**  `rpl_dup2` is unconditional; inside it `#ifdef F_GETFL` is live (the frozen
  `<fcntl.h>` defines it) while `HAVE_SETDTABLESIZE` and `REPLACE_FCHDIR` are not, so those
  two arms are `#[cfg (feature = "…")]` text (P-51).
- **The two dead arms keep C's function names, one function each** (never a renamed twin, the
  malloc.c/cloexec.c shape): `dup2_nothrow`, `ms_windows_dup2` (Windows) and
  `klibc_dup2dirfd`, `klibc_dup2` (kLIBC), each `#[cfg (feature = "windows_native" |
  "klibc")]`.  The `# define dup2 …` renames are `macro_rules!` twins guarded by the same
  features, so the live call `dup2 (fd, desired_fd)` still names the *system* entry through
  `mod system` (the readlink.rs `# undef` precedent).
- **Tokens the dead arms cannot spell** are in `docs/ports/macro-body-notes.tsv`
  (`dup2_nothrow`'s three MSVC exception macros) or in the arms' own text (`handle`, the
  `HANDLE` spelling, is a unit-local `pub type handle = *mut c_void`, which is what
  `<windows.h>` defines it as); `WIN32_LEAN_AND_MEAN` is in
  `docs/ports/excluded-macros.txt` (the nanosleep.c row).
- `EMFILE` (the live path's one errno code the errno mirror lacks) is declared in the unit.
- Line drift: `line-drift-tolerance.tsv` records `dup2 0.15` (8.8%).
- Not verified: gate-fast/gate-probes, ERT, and any behavioral test of `rpl_dup2`.
