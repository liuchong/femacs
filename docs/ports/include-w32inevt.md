# emacs/src/w32inevt.h (32 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_W32INEVT_H  (line 20, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 1
- [x] extern int w32_console_unicode_input  (line 22)

## Decisions

- **The header is unconditional, and nothing on this target compiles it.**  C 19-20 is the include
  guard, C 22-30 are live text (there is no `#ifdef` anywhere in the file), and the header includes
  nothing of its own.  The frozen configuration compiles no Windows unit at all - this target's
  platform object variables (`WINDOW_SYSTEM_OBJ=`, `HAIKU_OBJ = `, `MSDOS_OBJ =`, `CYGWIN_OBJ=`)
  are empty (`docs/baseline/frozen-build-sources.txt`, docs/status.md 3.31) - so
  `src/w32inevt.rs` is the empty-shape note that names the exclusion, and the mirror
  carries the declarations anyway, because the header's *text* is what a mirror mirrors (the
  `src/w32select.h` precedent, status 3.158).
- **P-72 decides every name here: the mirror may only name what its own text reaches.**  The header
  includes nothing, so `struct terminal *`, `struct input_event *`, `struct frame **` and `Time *`
  degrade to `*mut c_void`, each with the `/// C:` line P-55 requires, and `enum scroll_bar_part *`
  becomes `*mut c_int` (termhooks.h's enum is an `int` at the ABI - its members are ported in
  `include/termhooks.rs` 54-60).  `Lisp_Object *` stays named, because `lisp.h` is the one header
  every translation unit reaches through its own includes (PORTING.md 0.1-9) - the same reading
  `include/w32select.rs` records for its `Lisp_Object` arguments.  What is *not* deferred is
  signature shape: arity, pointer-ness and the `int`/`void` returns are all C's.
- **`--check` asks for less than fidelity does.**  The extractor's item list for this header is the
  guard macro plus the one `extern int` object (`## Functions defined — 0` counts definitions, not
  declarations), so a mirror holding only those two items would already check OK; the two
  `unsafe extern "C"` entry points are here because C 24-30 is live text, and this paragraph keeps
  that from looking like a required item later.
- **The guard is materialized as a const, and the mirror says why.**  `EMACS_W32INEVT_H` has no Rust
  counterpart (`include!` has no include guard, PORTING.md 0.1-4), but the extractor's item list
  names it, so the mirror spells it the way `include/w32select.rs` spells its own guard.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/w32inevt.h src/include/w32inevt.rs`
  -> `OK emacs/src/w32inevt.h -> src/include/w32inevt.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/w32inevt.h` -> `1 conditions, 0 unknown to the evaluator,
  0 disagreements` (that condition is the guard).
- `python3 tools/extract-abi.py emacs/src/w32inevt.h` -> this ledger: 0 subrs, 0 functions defined,
  1 macro (the guard), 0 types, 1 file-scope object.
- **No unit includes this mirror** (`rg -n 'include/w32inevt' femacs` -> no hits), so the compile
  evidence is a probe: `spikes/win_inc_probe.rs` supplies the names a Windows translation unit
  takes from the system and `include!`s this mirror together with `include/w32select.rs` (the other
  orphan of the same cause - both units are empty shapes).  `rustc +nightly-2026-10-01 --edition
  2024 --crate-type bin --emit metadata spikes/win_inc_probe.rs` -> OK, and the probe joined
  `xtask`'s `probe_programs` gate next to `f_inc_probe.rs`, so it is re-measured every round.
- `rg -n "w32_console_unicode_input|w32_console_read_socket|w32_console_mouse_position"
  src` -> this mirror, plus three copied C *comments* (`src/w32notify.rs` 40 and 49,
  `src/keyboard.rs` 8928) - there is no use-site declaration to retire (unlike 3.143/3.153).
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-mirror-dupes` 1 duplicate declaration(s) / 1 ratchet row(s) / 0 failure(s);
  `check-globals` 283 units / 0; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 0 findings.

## 3.589: the three declarations moved into a glob-imported module

The note above ("No unit includes this mirror") is out of date: `src/w32inevt.rs` 453,
`src/w32console.rs` 362 and `src/w32fns.rs` 144 all `include!` it now.  All three names this mirror
declares are `src/w32inevt.c`'s **definitions**, so at the mirror's crate root the `windowsnt` arm
read three `E0428 ... is defined multiple times`; they now sit in `pub mod w32inevt_decls` that the
mirror glob-imports (a crate-root definition shadows a glob import, PORTING.md 0.1-10 / P-21 - the
`include/w32term.rs` `w32term_fn_decls` shape).

Reading (`xtask probe src/w32inevt.rs … --cfg 'feature="windowsnt"'`, 2026-10-05): the three
`E0428`s are **gone**; the arm still reports **69 errors**, and **none of them is in this mirror** -
26 are in `_root.rs` (the unit's own `<windows.h>` stand-ins), 3 in `include/dispextern.rs`,
1 in `include/w32term.rs` and 1 in `include/keyboard.rs`.  `check-mirror-dupes` still reads
17 duplicate declaration(s) / 17 ratchet row(s) / 0 failure(s), because a declaration inside a
module the mirror glob-imports is counted.
