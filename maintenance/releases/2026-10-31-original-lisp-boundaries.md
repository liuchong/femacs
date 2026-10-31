# Original Lisp utility and timer coverage

Four complete unchanged stable files run on the installed macOS reference and
current normal terminal/Cocoa products. The pinned source commit is
`a360712c9d272d950d8d8255ef74570f7e90b7d9`. The actual program/dump/resource
identities match the accepted P435 cohort: the driver asserts the terminal
binding; independent review compares Cocoa with the accepted normal core
receipt outside the driver's input list. These are not new P444 or checking
builds. No test ID/tag filter or original guard/assertion is changed.

| File under `test/lisp/emacs-lisp/` | Each product | Three-product total |
| --- | --- | --- |
| `ert-x-tests.el` | 19 passes | 57 passes |
| `warnings-tests.el` | 4 expected results | 12 expected results |
| `thunk-tests.el` | 9 passes | 27 passes |
| `timer-tests.el` | 5 passes, 1 skip | 15 passes, 3 skips |

All twelve file processes finish with exit 0, complete records, no timeout,
protocol error or remaining process group. Each product registers the same
38 original IDs; the status maps agree. Total: **111 expected results and
three skips**. The original aggregate remains `passed=false`, with protected
owner exit 1: the checking-only `timer-tests-debug-timer-check` skips on all
three products because `debug-timer-check` is not bound. This does not validate
that checking primitive and does not justify a product change.

Three of the 111 expected results are **expected failures**, one per product:
`test-display-warning/warning-minimum-level` declares `:expected-result :failed`.
Its `should-not` fails because the warning appears in `*Messages*`, exactly as
that original batch expectation prescribes. The runner retains
`result=ert-test-failed expected=t` and classifies it as passed. These are not
successful negative assertions, ignored failures or three extra test IDs.
The other 108 expected results have passing bodies.

The complete `ert-x` utility file exercises nested test reporting, message-buffer
handling, owned temporary files/directories, simulated key/command and minibuffer
paths. It is batch execution, not physical UI acceptance. Warnings use three
severity levels; thunks exercise lazy/cached evaluation and macro errors; timers
exercise arithmetic, repetition argument validation and a zero-time event loop.
Temporary state is isolated per file. No network, external child program or
deliberate process abort is requested by these files. No new product build is
requested. The installed reference nevertheless creates one automatic native
trampoline in its isolated `ert-x` home; the runner retires its 34,096 bytes
with a full identity ledger. Empty `generated_resources` describes the copied
test subtree, not an absence of transient files elsewhere in the environment.

R42: twelve serial file processes, 30-second per-load/case deadlines and a
300-second protected-owner hard deadline. The earlier task estimate is below
512 MiB; the executed driver records a 1 GiB peak estimate instead. Neither
estimate is a separate hard limit; the installed runtime controller keeps its
standard dynamic policy. The 30/300-second deadlines are enforced. Actual driver
wall time is 8.242030583 seconds. The primary
controller records exit 1, complete owned cleanup, 10.691 seconds and
216,825,856 accounted peak bytes; its metrics remain primary-side receipts.
These timings are coverage data, not performance comparisons.

The separate `ert-tests.el` file remains **unaccepted**, including its unstable
backtrace-printing self-test with historical capped high-memory runs. It was
not hidden by selecting a green tag subset. Old default/all sweeps, the current
four-file result, checking coverage and full G3 are distinct evidence. Normal
coverage here does not establish checking, native/collector, physical GUI,
complete compatibility or a performance improvement. G1/G2/G4 remain **3/7**;
the original Goal and all four remaining outcomes stay open.

Inputs, programs, dumps, full resources and trial App remain unchanged. The raw
result stores `inputs_before` and unchanged assertions, without a separate
`inputs_after` map; independent review rehashes the nine inputs. Program/dump
and resource before/after identities are stored. Raw per-file metadata also
records the reference version 31.1 and `aarch64-apple-darwin25.6.0`, with its
full reported version text retained privately and hashed in the public receipt.
No generated test resources, production edit, new protection or App replacement.
The [bounded receipt](2026-10-31-original-lisp-boundaries-results.json) retains
all IDs/statuses/timings, diagnostic hashes, input/product identities and raw
event/result hashes. Raw diagnostics remain in
`build/acceptance/g3-lisp-boundaries-1`; personal source paths in message text
are not copied into the public receipt. Replay rematerializes the full pinned
corpus and uses the same existing stable-suite runner and explicit products.

Independent review checks all 114 event identities/ordinals and classifications,
the input and product identities, and the original skip and expected-failure
conditions. Consumed-state retirement, including the first cleanup assertion
failure and verified partial-operation resume, is recorded with
[CLN172](../cleanup/reports/2026-10-31-172-consumed-lisp-boundary-state.md).
