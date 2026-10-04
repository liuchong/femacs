# emacs/src/xsettings.h (51 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] XSETTINGS_H  (line 21, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header's includes are conditional, and the frozen configuration picks the X arm.**  C 23-25
  is `#ifndef HAVE_PGTK` (true here: neither PGTK nor any window system is configured), so
  `dispextern.h` and `<X11/Xlib.h>` are what C reaches; C 26-28's cairo arm is dead text.
  `check-cond-eval.py src/xsettings.h` reports 6 conditions, 0 unknown, 0 disagreements, so the
  extractor decides every arm from the frozen table.
- **Nothing on this target compiles the header.**  `src/xsettings.c` is not in the frozen build's
  source list (its platform object variables are empty, docs/baseline/frozen-build-sources.txt),
  so `femacs/src/xsettings.rs` is the empty-shape note that names the exclusion, and the mirror
  carries the live declarations the way `src/w32select.h` (3.158) and `src/xgselect.h` (3.162) do.
- **P-72 decides the names, and the header owns none of them.**  `Display_Info` is C 33-35's
  `#ifdef HAVE_PGTK` typedef - *dead text* on this target - and the live declarations reach it as
  dispextern.h's record; `XEvent` is Xlib's.  Both parameters therefore take the P-55 stand-in
  with C's prototype text in `/// C:` lines, and a consumer on a PGTK/X build converts at the call
  site (PORTING.md 0.1-9).  What is *not* deferred is signature shape: arity, pointer-ness, the
  `const` on `XEvent *` and the `bool`/`const char *` returns are all C's.
- **Two dead arms are recorded, not mirrored**: C 42-44's `xsettings_get_system_normal_font` is
  under `#ifdef USE_LUCID` (undefined here) and C 46-49's `xsettings_get_font_options` /
  `xg_get_system_dark_mode` are under `#ifdef HAVE_PGTK`.  Their `/// C:` lines stay in the mirror
  with the guard named, so a future configuration knows which `#if` to re-read (P-51's rule).
- **`--check` asks for less than fidelity does.**  The extractor's item list for this header is the
  guard macro alone (`## Functions defined — 0` counts definitions, not declarations), so a mirror
  holding only `XSETTINGS_H` would already check OK; the three live entry points are here because
  C 37-41 is live text, and this paragraph keeps that from looking like a required item later.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/xsettings.h femacs/src/include/xsettings.rs`
  -> `OK emacs/src/xsettings.h -> femacs/src/include/xsettings.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/xsettings.h` -> `6 conditions, 0 unknown to the evaluator,
  0 disagreements`.
- `python3 tools/extract-abi.py emacs/src/xsettings.h` -> this ledger: 0 subrs, 0 functions
  defined, 1 macro (the guard), 0 types, 0 file-scope objects.
- **No unit includes this mirror** (`rg -n 'include/xsettings' femacs` -> no hits), so the compile
  evidence is the probe: `spikes/x_inc_probe.rs` now `include!`s this mirror next to
  `xgselect.rs`, and it is registered in `xtask`'s `PROBES`, so the declarations are compiled
  every round.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures;
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures.
