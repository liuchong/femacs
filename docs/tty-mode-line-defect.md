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

## Probe round 6 (4.1017)

Markers at the top of the `'frame_pass` block, at the top of the inner `'update` block, and just
after the block's closing brace:

    M1-frame-pass-top: 0    M2-update-top: 0    M3-after-block: 0

With round 5's `T5-loop-body` printing, this says the flow leaves between `sw = w;` in the loop
body and the `'frame_pass` block.  That range (16818-17007) contains **no** `return`, `break` or
`continue` - checked by grep - so the remaining explanation is a **throw/longjmp** out of a call in
that range.  The three calls there are `do_pending_window_change (true)`, `clear_garbaged_frames ()`
and `prepare_menu_bars ()`.

The follow-up patch (markers after each of those three calls) failed to apply - an anchor did not
match - so it was not run.  Source is clean at `HEAD` and the defect still reproduces.

## Root cause found (4.1018)

Markers walked the gap between the loop body and the `'frame_pass` block:

    N0 loop body 8   N1 after do_pending_window_change 8   N2 after clear_garbaged_frames 8
    N3 after prepare_menu_bars 8   N4 frame-pass top 0

so the ordinary calls all return, and the flow still stops between `prepare_menu_bars` and the
block.  The call in that gap is `echo_area_display (false)` (the startup message keeps it on the
path); a marker right after it is **0**:

    N3 8   N5 after echo_area_display 0   N6 0   N4 0

and inside `echo_area_display`, `clear_garbaged_frames ()` returns but `display_echo_area (w)`
does not:

    E1 after clear_garbaged_frames 8   E2 after display_echo_area 0

**`display_echo_area` throws on every call.**  The throw unwinds out of `redisplay_internal`
(which is why `unwind_redisplay` resets `redisplaying_p` and the command loop keeps re-entering it
about 31,000 times a second), so the frame pass never runs and the mode line is never drawn.

Next: read `display_echo_area`'s tty path against the reference - the throw is either an explicit
signal in a branch the tty build should not take, or a call into an unported helper.

## Inside the throw (4.1019)

Markers inside `display_echo_area_1`, the callback `with_echo_area_buffer` runs:

    W1 entry 8    W2 after resize_mini_window 8    W3 after try_window 0

So the callback is reached and `resize_mini_window` returns; **`try_window (window, start, 0)` is what
does not return**.

That is the shared window-redisplay routine, and here it is called from the echo-area path with a
mini-window and a short message.  The next markers go inside `try_window` after `start_display` and
inside the `display_line` loop body (the first attempt put one between `if display_line (...)` and
its `{` and was reverted).

Reaching this point also explains the shape of the defect: every redisplay passes through the echo
area because the startup message is present, `try_window` throws there, the throw unwinds
`redisplay_internal`, and the frame pass that would draw the mode line never runs.

## init_iterator does not return (4.1020)

    V1 after start_display        0
    S1 after init_iterator        0

`start_display` calls `init_iterator` and never gets past it, so the throw is inside
`init_iterator` when it is called for the echo-area mini-window.

`eassert!` is compiled only under `enable_checking`, which the frozen configuration does not turn
on, so this is a real signal/longjmp from a call inside `init_iterator`, not an assertion.

Two lldb attempts did not produce a reading: a plain batch run never gets a terminal (`-nw` exits
with "Could not open file: /dev/tty" before redisplay), and the same run under `script` stopped
producing output and had to be killed after four minutes.  The next attempt should either break on
`siglongjmp` inside a real pty with `-o 'settings set target.process.stop-on-exec false'` and a
`-o 'process interrupt'` fallback, or bisect `init_iterator` with two markers (after the matrix
setup, before the first `reseat`).

## Inside init_iterator (4.1021)

    I1 before remap 8   I2 after remap 8   I3 after memclear 8   I4 after bidi_init_it 0

So the throw is between `memclear` and the end of the bidi setup block.  One follow-up marker meant to
split `bidi_unshelve_cache` from `bidi_init_it` was inserted at the **first** occurrence of that call
in the file rather than the one inside `init_iterator`, so its zero reading localises nothing and
is not used.

Next: place the split marker by line number inside `init_iterator` (the `bidi_unshelve_cache
(core::ptr::null_mut (), false)` in that block is at the time of writing line 4665), then read
whether `bidi_init_it` is the throwing call.

## Correction to 4.1021 (4.1022)

The marker right after `bidi_unshelve_cache (core::ptr::null_mut (), false)` reads **0**, and so does
one before the bidi block's comment.  Both zeros have the same simpler explanation: **the bidi block
is not entered at all** on this call.  So 4.1021's "throw between memclear and the end of the bidi
block" is right about the range but the bidi calls are not the suspect.

What is established: `memclear` returns (marker 8) and nothing from the bidi comment onward is
reached.  The throw is in the ~290 lines between them, which run unconditionally.

Next: three markers in that stretch (after the iterator's face/string setup, before the
`row != NULL` branch, and just before the bidi comment) to halve it again.

## One more halving (4.1023)

Markers before the control-character comment (4420), before the mode-line comment (4517) and before
the border-glyph comment (4569):

    Q1 0    Q2 0    Q3 0

Together with `memclear` returning, the flow stops within the ~86 lines after `memclear`, before
4420.  That stretch is mostly assignments plus `update_redisplay_ticks (0, w)`; the extra-line-spacing
block there is behind `FRAME_WINDOW_P`, false on this tty build.

Next: two markers in that ~86-line stretch (after the `(*it).f = ...` assignment and after
`update_redisplay_ticks`), and read the source around each before trusting the placement - the last
three rounds each lost a reading to a marker that landed inside a skipped branch.

## The throw is inside realize_default_face (4.1024)

    F1 after init_frame_faces 8    F2 after recompute_basic_faces 0

so `recompute_basic_faces` throws.  Inside it, `realize_basic_faces`:

    R1 after block_input 8    R2 after realize_default_face 1    R3 after mode-line face 1

`block_input` returns every time; only the **first** call gets past `realize_default_face` (and
realizes the mode-line face once).  Calls 2..8 never reach the if-body, and the process does not
abort, so `realize_default_face` throws rather than returning false.

Inside `realize_default_face`:

    U1 entry 8    U2 after lface_from_face_name 8

So the default-face lookup runs every time and the throw is in what follows it - most likely the
branch taken when the **default face already exists** (call 1 creates it, calls 2..8 re-realize it).

Next: markers after the `if NILP (lface)` body and around the final `realize_face` call in
`realize_default_face`; the re-realize path is the one that throws.

## The re-realize call inside realize_default_face (4.1025)

    X1 tty-face entry 8   X2 after make_realized_face 8   X3 after the weight mapping 8
    Y1 before realize_face 8   Y2 after realize_face 1

The X readings are eight because the ten `realize_named_face` calls on the first pass each go through
`realize_tty_face`; they do not belong to the default face's re-realization.  Y is the one that
belongs: the default face's `realize_face (c, attrs, DEFAULT_FACE_ID)` returns on the first pass and
throws on calls 2..8.

So the throwing call is **`realize_face` for `DEFAULT_FACE_ID` when face id 0 is already realized**.
The next markers go inside `realize_face` around the former-face removal/free and around
`realize_tty_face` for that specific call.
