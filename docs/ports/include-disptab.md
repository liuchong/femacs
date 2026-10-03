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
- [ ] GLYPH_ALIAS_P  (line 90, body 67d46f9d4886)
- [ ] GLYPH_FOLLOW_ALIASES  (line 95, body 779ac3bd1c98)
- [ ] GLYPH_LENGTH  (line 106, body da6feb2167bf)
- [ ] GLYPH_SIMPLE_P  (line 85, body 5999168d4661)
- [ ] GLYPH_STRING  (line 107, body e7fe4f2bebd0)
- [ ] GLYPH_TABLE_BASE  (line 79, body 25355ea345c8)
- [ ] GLYPH_TABLE_LENGTH  (line 74, body a0b8c63f242e)
- [ ] NULL_GLYPH  (line 112, body fb96549631c8)
- [ ] SET_GLYPH_FROM_CHAR  (line 114, body a05658b49d09)
- [ ] SPACEGLYPH  (line 111, body 0471315ef8e6)

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
