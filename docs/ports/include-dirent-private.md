# emacs/lib/dirent-private.h (67 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 4
- [x] DIR  (line 34, body 3bcf922654d9)
- [x] WIN32_FIND_DATA  (line 43, body 5729e25a8f88)
- [x] WIN32_LEAN_AND_MEAN  (line 38, body da39a3ee5e6b)
- [x] _DIRENT_PRIVATE_H  (line 18, body 356a192b7913)

## Types and their fields — 1
- [x] gl_directory: int fd_to_close, DIR* real_dirp, int fd_to_close, int status, HANDLE current, WIN32_FIND_DATA entry, char dir_name_mask

## File-scope objects — 0

## Decisions

- **Two arms for two systems that make gnulib replace `<dirent.h>`, and the frozen configuration
  picks the first.**  C 20-34 is the "mingw" arm (C 22 `# undef DIR`, C 24-31 the record, C 34 the
  redefinition) and C 36-63 the "MSVC" arm (C 38-39 `<windows.h>`, C 42-43 the `WIN32_FIND_DATA`
  rename, C 45-63 the record).  The frozen macro table records `HAVE_DIRENT_H 1`, so C 21-34 is the
  arm this configuration would compile and C 36-63 is dead text, carried under
  `#[cfg (feature = "msvc")]` (PORTING.md P-51 names the non-frozen side; the arm's own comment names
  the compiler).  No unit `include!`s the mirror: its only C consumers are gnulib's own
  `dirent.c`/`readdir.c`, which the frozen build does not compile.
- **The extractor's field list merges the arms, so the record carries both.**  It reports
  `gl_directory: int fd_to_close, DIR *real_dirp, int fd_to_close, int status, HANDLE current,
  WIN32_FIND_DATA entry, char dir_name_mask` (with `fd_to_close` twice, once per arm); the mirror
  keeps C's member order per arm and puts the five MSVC-only members behind the same `cfg`, which is
  what gives each configuration C's own layout.  Nothing asserts those layouts: no compiler on this
  target has `<windows.h>` and the record is compiled by neither arm here (the `include/w32heap.rs`
  `file_data` rule).
- **`DIR` is carried as a nullary `macro_rules!`** - C 34's `# define DIR struct gl_directory` is a
  *type* macro, so it is the `include/w32gui.rs` `NativeRectangle` / `include/msdos.rs` `PIX_TYPE`
  spelling.  `lib/include/dirent.in.rs` 107 spells the *same* redefinition for the other
  configuration's fallback arm under `feature = "no_dir_has_fd_member"`; the two are arms of one
  construct and never stand together (this one is un-gated because C 21's condition is satisfied).  A
  unit that ever includes both mirrors *with* that feature on would see two `macro_rules! DIR`s - the
  `androidterm.rs` 35-37 `Emacs_Cursor` style of recorded interaction, and one reason the two
  mirrors' `cfg`s are kept distinct.
- **The Windows names keep C's spelling**: `HANDLE` and `WIN32_FIND_DATA` reach the mirror from
  `<windows.h>` (C 39), which the port has no mirror of, so the record leaves them for a Windows
  target's unit (P-72 rule 3, the `lwlib/include/xlwmenuP.rs` 15-23 reading); the `/// C:` lines
  above each member hold the reference text.  `WIN32_LEAN_AND_MEAN` (C 38) is a marker macro, spelled
  as the constant that says "defined" (the `nt/inc/include/ms-w32.rs` 147 `_WINSOCK_H` spelling), and
  it already has a row in `docs/ports/excluded-macros.txt` 72 (kind=config, from the nanosleep.c
  lane) - which the gate reports as `(1 C macros have a recorded Rust counterpart:
  WIN32_LEAN_AND_MEAN)`, i.e. an informational line rather than a problem.
- **Conditions.**  `tools/check-cond-eval.py lib/dirent-private.h` -> `2 conditions, 0 unknown to
  the evaluator, 2 disagreements`; the file is not on the verified list, so the extractor resolves
  nothing and *both* arms count as live for it - which is exactly why the mirror answers for both.

## Evidence

- `python3 tools/extract-abi.py --check emacs/lib/dirent-private.h femacs/lib/include/dirent-private.rs`
  -> `OK emacs/lib/dirent-private.h -> femacs/lib/include/dirent-private.rs` (no `- ` problem lines),
  with the recorded-counterpart line for `WIN32_LEAN_AND_MEAN` above.
- `python3 tools/check-cond-eval.py lib/dirent-private.h` -> `2 conditions, 0 unknown to the
  evaluator, 2 disagreements`.
- `python3 tools/extract-abi.py emacs/lib/dirent-private.h` -> this ledger: 0 subrs, 0 functions
  defined, 4 macros, 1 type (`gl_directory`, 7 member names across the two arms), 0 file-scope
  objects.
- `rg -n 'include!\("[^"]*dirent-private' femacs` -> no unit `include!`s the mirror.
- **Compile evidence** is the lane-local probe of this batch
  (`/tmp/modprobe2/lib_probe.rs`, deleted afterwards): it `include!`s `lib/include/dirent.rs` for
  `DIR`, then this mirror, constructs `gl_directory { fd_to_close: -1, real_dirp: null_mut () }` and
  prints its size -> `lib mirror probe ok (16 bytes gl_directory)` (the live arm's layout: `int` then
  a pointer, 8-byte aligned, 16 bytes).
