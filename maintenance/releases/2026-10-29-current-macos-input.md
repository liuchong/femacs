# Current macOS input evidence and remaining IME boundary

The normal terminal and Cocoa foundation products each passed all 26 existing
classic input cases: Control line/character movement, arrows, Home/End, Meta and
Control-Meta movement, extended commands, kill/yank, prefix cancellation, search,
selection/copy/cut and undo. All keys entered the real terminal PTY or exact-PID
native command loop; no production key rebinding or original assertion changed.

Four additional file workflows passed on the normal/checking terminal and Cocoa
products, with normal exit codes 0. They cover opening, Unicode/combining text,
saving, closing and reopening; the Cocoa workflow also checks post-GC editing
and the actual system clipboard. All selected source/product/resource identities
remained unchanged. The original input source was restored.

All 56 cases passed. Execution took 85.799 seconds; the controlled run took
100.153 seconds, peaked at 209,420,288 accounted bytes and reclaimed its processes.
Raw results are in `build/acceptance/foundation-g1-keys-1`.

## Real composition observations, not a completed multi-frame pass

A separate controlled process ran the actual two-frame fixture. Native computer
use entered physical Pinyin keys through the enabled system input method. A
showed marked `ni`, then committed the exact text `你好`; B independently
committed `你好` after window cycling, leaving A unchanged. Returning to A could
start a new `ni` composition. Native callbacks and snapshots bind these
observations to the actual owned program and its A/B windows.

The subsequent Command-grave action during composition did not transfer to B:
A instead contained a literal grave character and no longer had the marked
state. A later unmarked cycle reached B, recorded by the native key and window-focus
callbacks in `callbacks.jsonl` rather than a further checkpoint. A coordinate
click then failed in the
computer-use tool before the remaining input actions, reporting that no window
was at its computed position. This is not proof of a program crash or successful
mouse-focus acceptance. The interaction workflow subsequently reached its
330-second deadline; the controller stopped its still-owned process. The raw
controlled run exited 1, completed cleanup, and the overall IME result remains
false. The coordinate-routing error and controller deadline are retained in the private
computer-use/controller record; they are not application crash receipts.
Original input source and product/resource identities were restored or
verified unchanged. No production change or repair is claimed.

A narrow system-reference check using the same fixture and input method cycled
to B without inserting the grave character and exited normally. Its precondition
was not fully equivalent: the timer observation showed working text `ni` and the
window capture displayed it underlined, but its Lisp marked-state flag was false,
unlike the candidate. Consequently this is an unresolved observation, not a
confirmed candidate-specific regression or a whole reference IME pass. The
reference report's overall result also remains false because the controller
required a full multi-step suite, while only the narrow comparison was performed.

Raw candidate/reference records are in
`build/acceptance/foundation-g1-ime-current-1` and
`build/acceptance/foundation-g1-ime-reference-1`.
An earlier unrelated application capture is excluded: a matching application
name is insufficient to establish an exact process/window binding.

G1 remains open for the unresolved composition/focus boundary and final native
workflows. These results do not close G2, full compatibility, native/collector
performance or the complete modernization Goal. Only G4 is delivered.
