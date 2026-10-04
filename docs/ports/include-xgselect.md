# emacs/src/xgselect.h (35 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] XGSELECT_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header is unconditional, and nothing on this target compiles it.**  C 20-21 is the
  include guard, C 28-33 are live text (there is no `#if` in the file at all), and the header
  includes `lisp.h` and `sysselect.h` (C 23-24) for `fd_set`/`sigset_t`.  Its only definer,
  `src/xgselect.c`, is wholly inside `#ifdef HAVE_GLIB`, which the frozen configuration leaves
  undefined, so that unit is an empty shape (`femacs/src/xgselect.rs`) and no translation unit
  reaches this header here.  The mirror carries the declarations anyway, because the header's
  *text* is what a mirror mirrors (the `src/w32select.h` precedent, status 3.158).
- **P-72 decides every name: the mirror names no type it does not declare.**  The header owns no
  type; `fd_set` comes from `sysselect.h`'s own includes and `sigset_t` from `<signal.h>`, both
  of which a *consumer* reaches itself (PORTING.md 0.1-9) - the same reading `include/w32inevt.rs`
  records for `struct terminal`/`Time`.  C 26's `struct timespec;` is a forward declaration of
  time.h's record, not a definition, so it is carried as a `/// C:` line and the parameter is
  `*mut c_void`.  All six pointer parameters therefore take the P-55 stand-in with C's prototype
  text above them; arity, pointer-ness and the `int`/`void` returns are C's.
- **`--check` asks for less than fidelity does.**  The extractor's item list for this header is
  the guard macro alone (`## Functions defined — 0` counts definitions, not declarations), so a
  mirror holding only `XGSELECT_H` would already check OK; the four `unsafe extern "C"` entry
  points are here because C 28-33 is live text, and this paragraph keeps that from looking like
  a required item later.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/xgselect.h femacs/src/include/xgselect.rs`
  -> `OK emacs/src/xgselect.h -> femacs/src/include/xgselect.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/xgselect.h` -> `1 conditions, 0 unknown to the
  evaluator, 0 disagreements` (that condition is the guard).
- `python3 tools/extract-abi.py emacs/src/xgselect.h` -> this ledger: 0 subrs, 0 functions
  defined, 1 macro (the guard), 0 types, 0 file-scope objects.
- **No unit includes this mirror** (`rg -n 'include/xgselect' femacs` -> no hits), so the compile
  evidence is a probe: `spikes/x_inc_probe.rs` supplies the names an X translation unit takes
  from the system and `include!`s this mirror; it is registered in `xtask`'s `PROBES`, so the
  declarations are compiled every round instead of being measured by `--check` alone.
- `rg -n "xg_select|suppress_xg_select|release_xg_select|release_select_lock" femacs/src` ->
  this mirror only (the unit `femacs/src/xgselect.rs` is the empty-shape note and names none of
  them), so there is no use-site declaration to retire.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) /
  0 failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures;
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 0 findings.
