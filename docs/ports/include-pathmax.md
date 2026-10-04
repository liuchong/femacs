# emacs/lib/pathmax.h (88 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] PATH_MAX  (line 85, body 09d66f6e5482)
- [x] _PATHMAX_H  (line 19, body da39a3ee5e6b)
- [x] _POSIX_PATH_MAX  (line 52, body dd7c1a3d9d56)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The file's live text is the guard and `_POSIX_PATH_MAX`; every `PATH_MAX` arm is dead here.**
  C 42-45 is C's "include config.h first" check, C 47/C 49 the two includes, C 51-53 the POSIX
  default.  The frozen macro table (`docs/baseline/config/darwin-arm64.macros.tsv`) records
  `PATH_MAX 1024` - the *system* `<limits.h>`'s value, which C 49 includes - so all four arms that
  try to supply the name are false on this target: C 55-58 (`<sys/param.h>`'s `MAXPATHLEN`), C 60-62
  (the `MAXPATHLEN` alias), C 64-71 (`__hpux`) and C 73-86 (`_WIN32`).
- **The frozen `PATH_MAX` is not spelled here.**  `lib/include/limits.in.rs` 131 already owns the
  name for this configuration (`pub const PATH_MAX: c_long = 1024;`, with the macro table as its
  evidence, and its 127 names this very header as the fallback's origin), so a second un-gated
  spelling would be `E0428` in any crate that includes both.  The three arms are therefore carried
  with `cfg`s naming the configurations they belong to - C 85's mingw `260` (written first: it is
  the reference's *last* definition, the `lib/root-uid.h` ordering, status 3.156), C 70's HP-UX
  `1024`, and C 61's `MAXPATHLEN` alias under `no_path_max` - which is the same shape
  `include/msdos.rs` uses for C's `EINPROGRESS`/`O_CLOEXEC` overrides of names the port already
  spells.  The gate compares no value for this name: it reports it as a branch macro (`1 C macros
  have several definitions (configuration branches), body not compared: PATH_MAX`), which is why the
  ordering is a readability choice rather than a requirement (the mirror says so).
- **`_POSIX_PATH_MAX` is `c_int`** (C's macro has no type; every use compares it against an `int`
  bound), and the guard is the usual `bool` constant (PORTING.md 0.1-4).
- **Conditions.**  `tools/check-cond-eval.py lib/pathmax.h` -> `10 conditions, 0 unknown to the
  evaluator, 1 disagreement`; the file is not on `docs/baseline/config/cond-eval-verified.txt`, so
  the extractor resolves nothing in it and P-51's "carry the dead text" governs the arms above.

## Evidence

- `python3 tools/extract-abi.py --check emacs/lib/pathmax.h femacs/lib/include/pathmax.rs`
  -> `OK emacs/lib/pathmax.h -> femacs/lib/include/pathmax.rs` (no `- ` problem lines), with the
  branch-macro note above.
- `python3 tools/check-cond-eval.py lib/pathmax.h` -> `10 conditions, 0 unknown to the evaluator,
  1 disagreement`.
- `python3 tools/extract-abi.py emacs/lib/pathmax.h` -> this ledger: 0 subrs, 0 functions defined,
  3 macros, 0 types, 0 file-scope objects.
- `rg -n 'include!\("[^"]*pathmax' femacs` -> no unit `include!`s the mirror (the C consumers are the
  units that need the bound, and on this target they take it from `<limits.h>` through
  `lib/include/limits.in.rs`).
- **Compile evidence** is a lane-local probe (no unit includes the mirror, and `lib/include` is not
  the set `xtask layout-check` compiles):
  `rustc +nightly-2026-10-01 --edition 2024 --crate-type bin -C panic=abort -o /tmp/modprobe2/lib_probe /tmp/modprobe2/lib_probe.rs`
  -> `lib mirror probe ok (16 bytes gl_directory)`; the probe `include!`s this mirror with the three
  others of the batch, asserts `_POSIX_PATH_MAX == 256` and prints the record's size.  The probe was
  deleted afterwards.
- Gates on the same worktree are in the batch report (the five gates and `xtask layout-check` pass;
  the readings are repeated in docs/ports/include-sfntfont.md, whose mirror *is* in the layout
  probe).
