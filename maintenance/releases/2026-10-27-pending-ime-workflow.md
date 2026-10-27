# Native pending-composition workflow — 2026-10-27

The current signed trial App passes this bounded two-window workflow on the
local Apple Silicon host with the installed Doubao Pinyin input source. This
closes the previously unobserved pending-composition mouse-transfer example;
it does not close all of G1 or establish complete input-method compatibility.
No editor implementation was changed for this acceptance.

## Actual observations

- Physical `n`, `i` input on A produced marked `ni`; the native callbacks identify
  the expected key window and first responder.
- The user then clicked B during composition and reported that the spelling
  disappeared normally, repeated A/B switching worked, and further text input
  remained usable. This is human mouse evidence, separate from automation.
- On both A and B, newly started real `ni` composition was cancelled with
  `Escape`, leaving both committed buffers unchanged.
- Physical `nihao` followed by Space committed `你好` on both windows. Each
  already contained the user's `你`; both final saved files contain exactly
  `你你好` plus a newline. Their SHA-256 is
  `449e3acd66c0c5eb3c63eeae8d257648102acbb7e0f0ae883f524f9474d5ddce`.
- Files were saved through the editor's normal `C-x C-w` workflow. The later
  frame selection used Command-grave; it does not replace the user's mouse test.
- Normal `C-x C-c` exited the native process with code 0. The resource controller
  also exited 0 and confirmed complete process cleanup. The input source was
  unchanged; no programmatic input-source selection was used.

## Identity and retained evidence

The tested executable SHA-256 is
`7727b278a23797e904ef121212dbf8efc446c607d571efb470934e7c90dbfa73`;
the dump is
`1a632e0654756c0ec8635df31c4b562294cea134e53c3be8a68e3f8c8691085d`.
Executable, frozen source-bundle dump, keyboard source, fixture and passive
observer hashes were unchanged before and after the run. The actual trial
bundle dump was additionally verified equal after exit; this launcher did not
separately hash that actual bundle dump before launch. This is the intermediate-icon trial
App, not the newer unselected artwork draft or unbuilt runtime WIP.

Private local receipts are retained under
`build/acceptance/g1-pending-ime-ui-1`: `results.json`, `steps.jsonl`, native
`callbacks.jsonl`, bounded `commands.jsonl`, input-source metadata, final
snapshot, the two UTF-8 files, original launcher/observer, owner/exit records
and controller log. Total controller wall time was 985.568 seconds, including
human interaction and waits; peak accounted memory for this run was
180633600 bytes. This is not a CPU-only benchmark. No compile or performance
claim follows from these readings.

## Differences and limits

`C-g` while `ni` was marked left composition active; `Escape` cancelled it.
The installed reference has not yet been tested with the same input method
and sequence. This observation is therefore neither a declared regression
nor a passing `C-g` cancellation case; it remains a specific comparison item.

The passive Lisp command observer reached its 256-row bound during A's filename
entry. Later save/quit claims rely on fresh AX observations, continuing native
callbacks, final buffer snapshots, exact file bytes and native/controller exit
receipts, not an invented complete Lisp command chain. One CUA paste operation
reported clipboard interference; readback found no saved file. The ASCII path
was then entered with direct text input. Chinese composition used physical
keys throughout.

Other input methods, all configurations/plugins, modified-file menu quit,
full stable-version compatibility and whole G1 completion remain outside this
receipt. The existing independent partner read the raw steps, callbacks, file
bytes, exit/controller receipts and separately hashed the trial/retained
releases. It explicitly adopted this exact current-App scope and the stated
limits. Its initial reply confused an older retained package with the current
App; actual hash readback corrected that before adoption. No independent UI
rerun is claimed.
