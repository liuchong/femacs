# Current macOS native workflow observations

## Current product and scope

The current trial App is `dist/macos/Femacs.app`, executable SHA-256
`b2fe7420ad1c3a041ea33ea9d0ccf80b852b818ea3315ffa65dd1aff84b2bcb0`, dump
`95eae81514a0998426ce39e778802776ed4579987ff66344141b448d2ad5aec2`.
It contains the P432 default-face receiver repair. The input/focus/mouse
implementation is unchanged from the standard delivered source. Earlier
Finder, keys/files and configuration observations retain their actual product
identities; they are reused for unchanged scope, not relabelled as fresh
current-product runs.

G1 is accepted through the complementary, exact-product observations below.
This is per-assertion acceptance, not one passing end-to-end run. Complete stable
compatibility, arbitrary package features and performance remain outside these
observations. The seven-part Goal remains active.

## Current product observations

- Current terminal and Cocoa products each pass all six existing TTY cases:
  missing, regular-file and directory device errors; detached no-controlling-TTY
  exact diagnostics; named invalid terminal type; and ten recoverable failures
  followed by terminal/frame-list invariance, GC and exact UTF-8 save. Actual
  products and source remain unchanged; all subprocesses return their expected
  statuses without a crash.
  Receipt: `build/acceptance/g1-current-tty-1`.
- Actual normal configuration startup completed with no initialization error,
  443 installed package entries and normal scheduled exit. Earlier unfinished
  installations and the old GC failure are retained in the
  [P432 record](../incidents/2026-10-29-cocoa-background-receiver.md).
- In a new exact-PID current-product instance, physical Pinyin keys committed
  `你好` independently on A and B. Automation selected frames by command; its
  attempted mouse click was retargeted to the front window. This is composition
  evidence, not a passing mouse workflow. The owner reached its deadline and
  stopped the process; no normal-exit claim follows. Raw result remains false
  in `build/acceptance/g1-current-native-multiframe-2`.

## Actual human mouse and composition sequence

In another exact-PID current-product instance, the user completed the requested
B/A mouse switching, Chinese entry, pending `ni` transfer from A to B and B's
Escape cancellation, and answered that this was normal. Native callbacks
independently record six A and five B mouse-downs, the actual A/B key-window
transitions, one `你好` commit on each, deletion of A's working composition on
transfer, and B's Escape/delete-working-text sequence. Further `dd`/`hh` entry
also reached the expected buffers. Final snapshot contains exactly `你好dd` on A
and `你好hh` on B, with no active marked composition.

Program, packaged dump, full resource and input-source identities remain
unchanged. The raw receipt is
`build/acceptance/g1-current-native-multiframe-3/results.json`; callbacks,
snapshot and separate derived `human-native-analysis.json` retain the actual
PID/window binding and human response boundary.

The developer did not finish normal quitting before this owner's deadline.
The process was stopped by its owner, observed exit 15, with complete controller
cleanup. Original `passed=false` and missing natural exit remain unchanged.
This is not a natural application crash and is not a single passing end-to-end
mouse/save/quit workflow. The human response cannot erase this limitation.

## Separate normal save and quit control

A separate current-product instance with two native frames first received ASCII
`ni`; that attempt is not Chinese composition acceptance. It was cleared, and
the already observed final texts were pasted into the owned A/B buffers. This
control covers ordinary editing, GC, saving and quitting, not mouse transfer or
physical IME composition.

The editor's normal `write-file` operations saved exact UTF-8 bytes matching
both final buffer snapshots, including their appended newline. Physical
`C-x C-c` then exited normally with code 0; the controller confirmed complete
cleanup. Program, dump, resources and input source remained unchanged. Its four
checkpoints cover native views, the two buffer contents and saved bytes; its
raw `passed=true` must not be interpreted as a mouse/IME suite pass. The raw
wrapper also inherited an overly broad physical-IME scope string; it remains
unchanged as original evidence. The separately bound `assessment.json` corrects
that interpretation to ordinary editing, GC, save and quit only.

Evidence: `build/acceptance/g1-current-native-multiframe-4` with actual ownership,
callbacks, four checkpoints, final snapshots, two files and natural exit. Files:

- A: `你好dd` plus newline, nine UTF-8 bytes.
- B: `你好hh` plus newline, nine UTF-8 bytes.

## Final acceptance boundary

These are complementary current-product observations from different processes.
The human mouse/composition process did not itself exit naturally; the separate
save/quit control did. Four actual read-only independent checks accepted G1
under the original per-assertion requirement: each observation binds its actual
PID, product and outcome; the written contract does not require every assertion
to occur in one process. No failed result is relabelled as a pass and no user
acceptance waiver is used. The scope correction above preserves the original
raw receipt and gives the normal control its actual interpretation.

Together with the retained unchanged Finder/keys/files/configuration evidence,
current normal startup and current 12-case TTY acceptance, these observations
complete G1. G1, G2 and G4 are now delivered: three of seven outcomes. G3, G5,
G6 and G7 remain open. This does not certify a single mouse-to-save-to-quit
end-to-end run, all input methods/plugins, complete stable compatibility,
performance or the whole Goal.
