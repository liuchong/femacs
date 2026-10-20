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

## Probe round 1 (4.1012)

Temporary probes in `xdisp.rs` (all reverted, source back at `HEAD`, rebuild reproduces the defect):

| probe | where | hits in 3 s |
| --- | --- | --- |
| MLP5 | after the `glyphs_initialized_p` guard | 94,109 (`glyphs=1 initial=0`) |
| MLP10 | inside the `'retry_frame` loop | 0 |
| MLP8 | at the `frame_redisplay_p (f)` test | 0 |
| MLP9 | the cursor fast path in `redisplay_internal` | 0 |
| MLP6/7 | `redisplay_windows` / `redisplay_window_0` | 0 |
| MLP1 | inside `display_mode_line` | 0 |

So `redisplay_internal` is entered about 31,000 times a second, passes the guards, and leaves
before the `consider_all_windows_p` branch - neither the frame loop nor the selected-window path
runs.  The next reading is that flag itself, plus whether `FOR_EACH_FRAME` iterates at all
(`Vframe_list` is non-nil at that point).

The 31,000 calls per second are their own finding: something in the command loop keeps asking for
a redisplay that returns without doing anything.

## Probe round 2 (4.1013)

Two more probes, each in its own build (so the readings below pair only with each other):

- MLP11, immediately before `if consider_all_windows_p`: **0 hits**.
- MLP12, at the entry of `redisplay_window`: **0 hits**.

With round 1 (the glyphs guard at 94,109 hits, everything downstream zero), the flow leaves
`redisplay_internal` somewhere between that guard and the `consider_all_windows_p` branch.

Cross-build inference is what made the last two rounds hard to read: each build carried a
different probe set.  The next round puts **one** trace in **one** build - markers at the fast-path
entry, at each `break 'cancel` / `break 'update` / `break 'retry`, right after the labelled block,
inside the `if consider_all_windows_p` branch, at `redisplay_window` entry and at
`display_mode_line` - so a single run says which exit was taken.

## Probe round 3 (4.1014)

One build, one run: markers at entry, after the glyphs guard, and after each of the five statements
that follow it (`redisplay_counter`, `record_unwind_protect_void (unwind_redisplay)`,
`redisplaying_p = true`, `specbind`, `reset_outermost_restrictions`).  The run printed

    B-past-guards, C1, C2, C3, C4, C5   (then the same six again)

and nothing after `reset_outermost_restrictions ()`.  The marker meant for the `'retry` loop head
landed in a **different** `'retry: loop` (line 9525, not 16829), so the run says only that
execution reaches `reset_outermost_restrictions ()`; it does not yet say whether the loop is
entered.

Two later patch attempts produced invalid Rust (a trace inserted between an `if` and its `{`), and
were reverted.  Source is back at `HEAD`, rebuilt, and the defect reproduces (`tui-smoke` 1,096
bytes, mode line MISSING).

Next round: either patch by hand at three exact statements (loop head, fast-path block entry, the
`redisplay_window` call) instead of a pattern-matched insert, or switch to lldb - break on
`redisplay_internal`, `finish`, and read which caller path returned.

## Probe round 4 (4.1015)

Four markers, each placed by hand at a unique anchor: T1 just before the `'retry` label, T2 after
`sw = w;` in the loop body, T3 just before `if consider_all_windows_p`, T4 at
`redisplay_window`'s entry.  One build, one run:

    T2-loop-body, T2-loop-body        (everything else zero)

So the loop body runs, and neither the branch nor `redisplay_window` is reached.  The exit is one
of the `break 'retry` sites inside the fast-path block.  T1 not printing while T2 does is not
explained by this run: a linear entry passes T1, and a `continue 'retry` jumps over it, so the
missing print means the observed entries were all jumps, not the first fall-through - worth
re-reading with an uncapped counter before drawing a conclusion from it.

Next reading: C enters the fast-path block only when `CHARPOS (this_line_start_pos) > 0`, which is
false at startup.  Check what the port's `this_line_start_pos` holds at the first redisplay; if it
is positive, the port takes a shortcut C cannot take, and that is the bug to fix.

## Probe round 5 (4.1016)

Uncapped-to-eight markers at the `'retry` label, in the loop body with the value of
`CHARPOS (this_line_start_pos)`, and just before `if consider_all_windows_p`.  Counting the
**raw** byte stream (the line-based reader had missed one line):

    T1-loop-head: 8    T5-tlbufpos=0: 8    T3-before-branch: 0

So the loop head is reached eight times, `this_line_start_pos` is **0** at each of them - which
makes C's fast-path condition (`CHARPOS (tlbufpos) > 0`) false - and yet the flow never arrives at
`if consider_all_windows_p`.  With the fast path skipped, the exit between the loop body and the
branch is unexplained by the source as read: the only `break 'retry`/`continue 'retry` in that
range sit inside the skipped `if`.

Next reading, one build: a marker at the **top** of the fast-path block body (inside the `if`) and
one immediately after the block's closing brace.  If the first is zero and the second is zero, the
flow leaves the block without executing either - which points at a `break 'frame_pass` placed
differently than C's `goto`, or at a longjmp out of the block.
