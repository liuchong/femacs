# emacs/src/textconv.h (163 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 1
- [x] TEXTCONV_SKIP_CONVERSION_REGION  (line 123, body ee799cd75778)

## Types and their fields — 3
- [x] textconv_callback_struct: EMACS_INT position, enum textconv_caret_direction direction, unsigned short factor, enum textconv_operation operation, struct textconv_conversion_text text
- [x] textconv_conversion_text: ptrdiff_t length, ptrdiff_t bytes, char* text
- [x] textconv_interface: void (*) reset, void (*) point_changed, void (*) compose_region_changed, void (*) notify_conversion

## File-scope objects — 0

## Decisions

- **Every record in this header is measured, not guessed.**  The frozen clang compiles
  `textconv.h`'s include chain (`lisp.h`, `frame.h`) on this target, so the three records' sizes,
  alignments and offsets are asserted in the mirror: `struct textconv_interface` 32/8 (0/8/16/24),
  `struct textconv_conversion_text` 24/8 (0/8/16), `struct textconv_callback_struct` 48/8
  (0/8/12/16/24).  `enum textconv_caret_direction` and `enum textconv_operation` are 4 bytes each,
  so the port spells them as `c_int` plus one constant per member (PORTING.md 1.3, the
  `include/dispextern.rs` 66-75 `text_cursor_kinds` shape).
- **The guard is degenerate and there is no macro to mirror.**  C 20 is `#ifndef _TEXTCONV_H_` with
  **no** `#define _TEXTCONV_H_` anywhere in the file (`rg -n '_TEXTCONV_H_' emacs/src/textconv.h`
  finds only the `#ifndef` and its `#endif` comment), so C re-reads the body on every include.  The
  mirror therefore carries no `pub const` for the name - it would claim a macro C does not define -
  and records it here instead (PORTING.md 0.1-4).
- **The declarations live in `pub mod textconv_decls` with a glob import.**  `src/textconv.c` defines
  every one of them, so a name at this scope would be an `E0428` for that unit; the glob import is
  shadowed by the unit's own definitions (P-17/P-21, PORTING.md 0.1-10).  `register_textconv_interface`
  (C 161) stays at the top level: its callers are the GUI backends' units, and no unit defines it.
- **The Android half is carried under `#[cfg (feature = "have_android")]`.**  C 129's
  `#ifdef HAVE_ANDROID` is dead here (`HAVE_ANDROID` is `undef`), and every parameter in it is a name
  this mirror can spell, so the fourteen declarations keep C's text and the `#[cfg]` sits on each of
  them - **not** on a nested `extern` block: Rust rejects `extern` blocks inside `extern` blocks
  (`error: extern block is not supported in `extern` blocks`), which `layout-check` caught when this
  mirror first landed.
- **Degraded names (P-55/P-72 rule 3).**  `struct frame` is named because this header includes
  `frame.h` (C 23) and `include/frame.rs` owns the record; `struct window` and `struct buffer` are
  only pointed at (C 42-43 and `frame.h` does not include them either), so both parameters are
  `*mut c_void` with C's prototype in the `/// C:` line above the field.
- **`TEXTCONV_SKIP_CONVERSION_REGION` (C 123) is a constant, not a macro**: C uses the name as a
  value, and the checker compares C's token pair `(1 << 0)` against the port's expression.

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/textconv.h femacs/src/include/textconv.rs`
  -> `OK emacs/src/textconv.h -> femacs/src/include/textconv.rs` (no `- ` problem lines).
- `python3 tools/extract-abi.py emacs/src/textconv.h` -> this ledger: 0 subrs, 0 functions defined,
  1 macro, 3 types, 0 file-scope objects (the two enums' members are constants).
- The measurements: `clang -std=gnu23 -O0 -I build/emacs-ref/lib -idirafter spikes/shim
  -I build/emacs-ref/src -I emacs/src -I emacs/lib` on a TU that includes `textconv.h` and prints
  `sizeof`/`_Alignof`/`offsetof`; the numbers are in the mirror's assertions, in that order.
- **No unit includes this mirror yet** (`rg -n 'include/textconv' femacs` -> no hits): the one
  committed caller, `keyboard.rs` 4061/4516/4624, reaches `detect_conversion_events` and
  `handle_pending_conversion_events` inside `#[cfg (feature = "have_text_conversion")]` arms
  (`HAVE_TEXT_CONVERSION` is `#undef`; C 48/114/3701), so nothing needs the declarations yet.  The
  units whose C includes `textconv.h` - `keyboard.c`, `xterm.c` 640, `xfns.c` 41, `androidterm.c`
  33 - can take this mirror when their owners agree.
- `CARGO_TARGET_DIR=/tmp/femacs-target-s188 cargo run -q -p xtask -- layout-check` ->
  `layout-check: 87 of 98 mirror(s) compiled, 11 platform-exclusive skipped (...)`, `layout probe ok`,
  `layout-check passed` (this run is also the one that proved the `extern`-in-`extern` fix).
- `build-tu src/textconv.h` -> `error[E0425]: cannot find value 'HAVE_PDUMPER' in this scope` - the
  mirror's own include chain reaches `lisp.rs`/`pdumper.rs`, whose bodies want the `config.rs`
  constants a *unit* includes first; the committed mirror of the same class, `src/charset.h`, fails
  the same way (`cannot find value 'HAVE_NATIVE_COMP'`/`HAVE_PDUMPER`, measured), so this mirror's
  compile evidence is `layout-check`, as those ledgers say.
- Gates on the same worktree: `check-declarations` 13 sites (0 forbidden, 13 allowed literals) / 0
  failures; `check-target-names` 612 .rs files / 593 map targets / 0 findings; `check-mirror-dupes`
  1 duplicate declaration(s) / 1 ratchet row(s) / 0 failures.
