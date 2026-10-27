# macOS Finder external file-drop acceptance

The user manually dragged the owned `finder-drop.txt` from Finder into the
published native editor's separate `Femacs Drag Target` window on macOS 27.0.1.
The user confirmed that the file opened. The actual native trace, Lisp observer
and visible native window independently corroborate the same operation.

## Bound product and evidence

- Published program SHA256: `a1f6f56bad59ee4f582dd2c9c7e2daf1d4fe050be5cdf7766ff841ac7b366292`.
- Dump SHA256: `c6a13f551e94831a55c5aa4201495a61e8d207889b45fb3331c586b79c710ea8`.
- Source file: 57 UTF-8 bytes; SHA256 `a48a2a3c4d2208ea0d335bf27128e51f82b80e2aac7cbf5c490722a1fb9bd067`.
- Exact opened contents include Chinese and an emoji and match all 57 source bytes.
- Actual native callbacks: 124 `draggingUpdated`, one `performDragOperation`.
- All 124 Lisp motion observations identify the live target frame and integer
  coordinates within its 492 by 396 content area. The native callbacks bind the
  same target window. A background target is valid; no key-window requirement
  is invented for an external file drop.
- Source file, fixture, program and dump identities remain unchanged.
- Private receipts: `build/acceptance/g1-finder-drop-ui-2/results.json`,
  `ready.json`, `motion.json`, plus the original owner-bound native trace.

## Scope and retained failures

This passes the actual Finder external text-file drop and observed configured
motion callback case. It does not certify every pasteboard type, every drag
policy, complete G1, latest-stable compatibility or GC. The original editor
remains live for the pending two-window input-method workflow; a normal exit
has not yet been observed for this process. Final independent G1 review remains
pending and is not replaced by the user's manual action.

Earlier automated cross-window drags produced no native callbacks. Their
original receipts remain, including `automated-attempt.json` in this run and
the preceding `g1-finder-drop-ui-1` results. They do not prove that the editor
rejected a delivered native drag. No new production patch, fallback event
simulation, compiler or remote CI was needed for this acceptance.

The IME A/B buffers were preserved in separate windows. The second window's
Chinese-mode confirmation and complete commit/cancel/focus-transfer workflow
remain open. Overall delivery count remains one of seven outcomes; this
individual acceptance result does not complete another whole outcome.
