# emacs/src/disptab.h (117 lines)

# ----------------------------------------------------------------------
## Subrs (DEFUN) — 0

## Functions defined — 0

## Macros defined — 20
- [x] DISP_BORDER_GLYPH  (line 36, body 38a9f5e82e14)
- [x] DISP_CHAR_VECTOR  (line 57, body a93816704844)
- [x] DISP_CONTINUE_GLYPH  (line 32, body 80b267c8d990)
- [x] DISP_CTRL_GLYPH  (line 34, body 5857d19d458c)
- [x] DISP_ESCAPE_GLYPH  (line 33, body 1ab444f473bc)
- [x] DISP_INVIS_VECTOR  (line 35, body 6401ad88734d)
- [x] DISP_TABLE_EXTRA_SLOTS  (line 52, body 57d4fad8f181)
- [x] DISP_TABLE_P  (line 26, body 266b96509423)
- [x] DISP_TRUNC_GLYPH  (line 31, body 0c94632e1e0e)
- [x] EMACS_DISPTAB_H  (line 22, body da39a3ee5e6b)
- [x] GLYPH_ALIAS_P  (line 90, body 67d46f9d4886)
- [x] GLYPH_FOLLOW_ALIASES  (line 95, body 779ac3bd1c98)
- [x] GLYPH_LENGTH  (line 106, body da6feb2167bf)
- [x] GLYPH_SIMPLE_P  (line 85, body 5999168d4661)
- [x] GLYPH_STRING  (line 107, body e7fe4f2bebd0)
- [x] GLYPH_TABLE_BASE  (line 79, body 25355ea345c8)
- [x] GLYPH_TABLE_LENGTH  (line 74, body a0b8c63f242e)
- [x] NULL_GLYPH  (line 112, body fb96549631c8)
- [x] SET_GLYPH_FROM_CHAR  (line 114, body a05658b49d09)
- [x] SPACEGLYPH  (line 111, body 0471315ef8e6)

## Types and their fields — 0

## File-scope objects — 0

## Decisions

- `character.c` is the first unit on this target that reads the display table, so the
  mirror carries the accessors that need no glyph datatype: `DISP_TABLE_P`, the six
  `DISP_*_GLYPH` slots, `enum box` (whose last member fixes `DISP_TABLE_EXTRA_SLOTS`),
  `disp_char_vector`'s prototype, `DISP_CHAR_VECTOR`, and the `window_display_table` /
  `buffer_display_table` prototypes.  The nine glyph macros and the two glyph constants at
  the bottom of the header need dispextern.h's `GLYPH_*` accessors, which are not mirrored
  on this target yet, so they stay unchecked and the file is WIP (PORTING.md P-66).
- `enum box` is a Rust keyword: the type keeps C's name as the raw identifier `r#box`, and
  the members are constants of that type with C's *values* (the `draw_glyphs_face` shape in
  dispextern.rs), because cinventory checks enum members as constants.
- `DISP_CHAR_VECTOR` is C's multi-line `#define`; the port writes it as an `#[inline(always)]`
  function with C's expression order, and the checker's token comparison accepts it.
- `SPACEGLYPH`/`NULL_GLYPH` are C octal literals (`040`, `00`) with no Rust spelling; they
  are left unmirrored with the rest of the glyph block rather than exempted one by one.

## Decisions

- **The glyph-table family landed** (disptab.h 72-122): `GLYPH_TABLE_LENGTH`/`GLYPH_TABLE_BASE`,
  `GLYPH_SIMPLE_P`/`GLYPH_ALIAS_P`/`GLYPH_FOLLOW_ALIASES`, `GLYPH_LENGTH`/`GLYPH_STRING`,
  `SPACEGLYPH` (C's octal 040 = 32), `NULL_GLYPH` (0) and `SET_GLYPH_FROM_CHAR`.  The blocker the
  earlier WIP note named - dispextern.h's `GLYPH_CHAR`/`GLYPH_FACE`/`SET_GLYPH_CHAR`/
  `GLYPH_CHAR_VALID_P`/`SET_GLYPH` and `DEFAULT_FACE_ID` - is gone: that mirror carries them now.
  Every macro is a `macro_rules!` because C's is (PORTING.md P-33).
- **Use-site copies retired in the same batch** (0.1-3): `term.rs` carried seven of the macros plus
  both constants and `dispnew.rs` carried five plus `SET_GLYPH_FROM_CHAR`, each with its own "the
  disptab mirror stops before this" note.  Both units include this mirror, so the copies were
  duplicates the moment the family landed.  The other seven consumers (doc, indent, xfaces, print,
  window, character and - in flight - xdisp/keyboard) needed no edit: they never declared them.
- One shared row: `GLYPH_FOLLOW_ALIASES`'s `false`, C's `do { … } while (false)` test, which a Rust
  block has no site for (`docs/ports/macro-body-notes.tsv`, the `SET_GLYPH_FROM_GLYPH_CODE` shape).

## Evidence

- `python3 tools/extract-abi.py --check emacs/src/disptab.h src/include/disptab.rs`
  -> `OK emacs/src/disptab.h -> src/include/disptab.rs`.
- `xtask build-tu` -> `built` for all ten units that include this mirror: `src/term.c`,
  `src/dispnew.c`, `src/doc.c`, `src/indent.c`, `src/xfaces.c`, `src/print.c`, `src/window.c`,
  `src/character.c`, `src/xdisp.c` and `src/keyboard.c` (the last two are the in-flight units).
- `python3 tools/check-mirror-dupes.py` -> `1 duplicate declaration(s), 1 ratchet row(s),
  0 failure(s)` (the surviving row is insdel.rs's).
