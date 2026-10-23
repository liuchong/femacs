# emacs/src/commands.h (30 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 3
- [x] Ctl  (line 24, body 48a63dcc8347)
- [x] EMACS_COMMANDS_H  (line 20, body da39a3ee5e6b)
- [x] INTERACTIVE  (line 28, body 56f8a402f675)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- **`#define Ctl(c) ((c)&037)`**: Rust has no octal literal, so the port writes `0o37`
  (31, the same value); the `Ctl` row of `docs/ports/macro-body-notes.tsv` records that the
  checker compares token text and therefore cannot see C's octal spelling.
- **`#define INTERACTIVE (NILP (Vexecuting_kbd_macro) && !noninteractive)`**: the frozen C
  global `noninteractive` is spelled `noninteractive1` in the generated globals
  (`docs/baseline/globals/variables.tsv` 711, P-17), which is the name the macro must call;
  the `INTERACTIVE` row of `docs/ports/macro-body-notes.tsv` records the rename.
- commands.h has no conditional, so the mirror is the whole header; the include guard is the
  `pub const EMACS_COMMANDS_H: bool = true;` the other mirrors use (PORTING.md 0.1).
- Evidence: `tools/extract-abi.py --check emacs/src/commands.h
  src/include/commands.rs` -> OK.
