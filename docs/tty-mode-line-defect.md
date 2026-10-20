# Open defect: a terminal frame is drawn without its mode line (2026-10-21, status 4.1010)

Every other check in this repository runs `--batch`, which never builds a terminal frame.  A pty
probe (`tools/tui-smoke.py`) does, and it shows a difference the batch checks cannot:

| program | 24x80 | 10x80 |
| --- | --- | --- |
| `build/emacs-ref/src/emacs` | mode line present (1,774 bytes) | present (1,659 bytes) |
| `build/port/src/emacs` | **MISSING** (1,096 bytes) | **MISSING** (984 bytes) |

"Present" means the captured stream contains both `*scratch*` and `Lisp Interaction` after escape
sequences are stripped.  The port's stream carries the menu bar and the buffer text but no mode
line, at either size.

Not an initial-paint race: the mode line is still absent after typing a character, and after `C-l`.
The reference shows it in both readings.

## Why the existing gates missed it

`smoke.sh`, `differential.sh`, `check-loadup.py` and the whole L5 sweep run `--batch`.  A terminal
frame is built by `term.c`'s redisplay path, which those runs never enter.  The Android GUI has its
own device probe; the terminal frame had none until this round.

## What is open

- Where the row is lost: `xdisp` builds the mode-line glyph row, and `term.c`'s
  `tty_write_glyphs`/`tty_draw_row` writes rows.  Either the row is not marked for output, or the
  row is written with the wrong geometry (a frame height that puts it off-screen would explain the
  smaller byte count at both sizes).
- The face: the mode line is drawn with the `mode_line` face; a face lookup that fails on a tty
  frame could leave the row blank, but the row's text would still be emitted - it is not, so the
  row is more likely never written.

## Boundary

This defect is **not** one of the eight completion rows, and it does not change their readings: the
macOS row is backed by the batch smoke and the differentials, both of which still pass.  It does
mean the terminal deliverable is not visually complete, so it is recorded here rather than folded
into a row that would then be quietly reinterpreted.

## Narrowing (4.1011)

Three readings take geometry and formatting out of the picture:

- Inside the pty, both programs report `(frame-height)` 23, `(window-height)` 22 and
  `(window-mode-line-height)` 1 - the mode-line row exists and has height 1.
- A `message` from `--eval` is painted in the echo area by the port (`H=23 W=22 ML=1` reaches the
  screen), so the row-writing loop does reach the last rows; the missing row is the one above the
  echo area.
- `(format-mode-line mode-line-format)` written to a file is byte-identical between the two
  programs (`-=--:--- F1  *scratch*  All L1  (Lisp Interaction ...)`).

So the loss is in producing or writing the mode-line row's glyphs, not in the frame height and not
in the format machinery.  Next reading needs a temporary probe in `display_mode_line`: after
`display_mode_element` returns, print the row's `used` count, `enabled_p` and its `y`, then compare
with the same reading from the reference built with the same probe.
