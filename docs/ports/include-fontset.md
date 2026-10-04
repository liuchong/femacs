# emacs/src/fontset.h (48 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] EMACS_FONTSET_H  (line 27, body da39a3ee5e6b)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **The header is unconditional and entirely live text, and the *unit* is the empty shape.**
  C 26-27 is the include guard and it is the file's only condition (`tools/check-cond-eval.py
  src/fontset.h` -> `1 conditions, 0 unknown, 0 disagreements [VERIFIED]`, and `src/fontset.h` is
  in `docs/baseline/config/cond-eval-verified.txt`); C 29-47 is live text, so the ten prototypes
  are mirrored.  Nothing in the frozen configuration includes `fontset.h` on this target -
  `src/fontset.c` is not in `docs/baseline/frozen-build-sources.txt` and the `HAVE_WINDOW_SYSTEM`
  arms that reach the header (`font.c` 43-44, `frame.c`'s `GUI` block) are dead (PORTING.md P-08)
  - so `femacs/src/fontset.rs` is the empty-shape note the 3.31 policy requires.  The header's
  text being live while its unit is empty is the `src/w32select.h` reading (status 3.158).
- **Every name here is decided by P-55/P-72, and only two of them degrade.**  The header includes
  `lisp.h` and nothing else (C 29) and forward-declares `struct face` (C 31).  `Lisp_Object` and
  `ptrdiff_t` stay named - `lisp.h` is the one header every translation unit reaches through its
  own includes (PORTING.md 0.1-9), so `layout-check`'s foundation list puts `lisp.rs` first and a
  unit that uses this mirror has it too.  `struct frame *` and `struct face *` degrade to
  `*mut c_void`: neither record is declared by this header, and Rust's type identity is global, so
  naming `frame` (which lives in `frame.rs`) or `face` (which has *no* mirror at all - seven units
  carry their own `pub struct face`) would make a unit that takes this mirror without those
  records fail.  Each degraded parameter carries the `/// C:` line P-55 requires, inside the
  `unsafe extern "C"` block above the declaration it describes; the use site casts.
- **The declarations `font.c`'s port keeps today are use-site copies and are reported, not
  edited.**  `femacs/src/font.rs` 627-634 declares `face_for_char` (annotated
  `/// C: dispextern.h 3811`, which is the declaration `font.c` actually sees - `dispextern.h`
  re-declares it) and `font.rs` 713-724 declares `fs_query_fontset`/`fontset_ascii` with the note
  "`emacs/src/fontset.h` has no mirror at all".  That note is now false, and once `font.rs`
  includes this mirror the three declarations and the note go away (the `struct font` use-site
  retirement precedent, status 3.158 - five copies are retired by the integrator, not by a lane).
  This ledger is the lead.
- **What compiles the mirror.**  No unit `include!`s it, so the two things that do are
  `xtask layout-check` (the single-crate probe, which compiles every `src/include` mirror with
  `lisp.rs` forced first) and `spikes/sfnt_inc_probe.rs`, the probe this round added for the two
  orphan mirrors of `src/sfnt.h`/`src/fontset.h`.  The header-form `build-tu src/fontset.h`
  **cannot** work for any mirror that names a `c_int`-class type - the crate root *is* the mirror,
  which carries no `use` - and that is a property of the tool, not of this mirror: measured the
  same for `src/w32gui.h`, `src/menu.h` and `src/cygw32.h` (`error[E0425]: cannot find type
  c_int`), and for `src/w32inevt.h` with 11 errors of the same class in the already-landed
  precedent mirror.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/fontset.h femacs/src/include/fontset.rs`
  -> `OK emacs/src/fontset.h -> femacs/src/include/fontset.rs` (no `- ` problem lines).
- `python3 tools/check-cond-eval.py src/fontset.h` -> `1 conditions, 0 unknown to the evaluator,
  0 disagreements` (the condition is the guard).
- This ledger: 0 subrs, 0 functions defined, 1 macro (the guard), 0 types, 0 file-scope objects.
- Compile proof: `cd femacs && CARGO_TARGET_DIR=/tmp/femacs-target-sfnt cargo run -q -p xtask --
  probe ../spikes/sfnt_inc_probe.rs ../build/probes/sfnt_inc_probe.rmeta --crate-type=lib
  --emit=metadata` -> exit 0, 0 warnings (the probe includes both this mirror and
  `include/sfnt.rs`); `cargo run -q -p xtask -- layout-check` -> `layout probe ok` /
  `layout-check passed`.
- Consumer smoke (the two units whose C text includes `fontset.h` and which the frozen
  configuration builds as empty shapes): `cargo run -q -p xtask -- build-tu src/fontset.c` ->
  `built src/fontset.rs -> ../build/femacs/fontset.o`.
- Gate readings on this worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed
  literals) / 0 failures; `check-globals` 283 units / 0; `check-mirror-dupes` 1 duplicate
  declaration / 1 ratchet row / 0 failures; `check-verbatim` 6 trees / 4607 files / 0 failures;
  `check-target-names` 580 `.rs` / 592 map targets / 0 findings.
