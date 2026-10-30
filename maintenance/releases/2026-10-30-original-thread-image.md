# Original thread, font, image and notification coverage

Four complete original files from pinned stable commit
`a360712c9d272d950d8d8255ef74570f7e90b7d9` run unchanged on the installed
macOS reference and current normal terminal/Cocoa products. Each registers
48 IDs: thread 38, font two, image five and inotify three. All twelve file
processes complete with exit 0, no failed assertion, timeout, protocol error
or remaining process group. Inputs, programs, dumps, full resources and the
trial App remain unchanged; copied test trees have no generated resources.

The result is **131 passes and 13 skips**. Original aggregate `passed=false`,
protected-owner exit 1 and `status_maps_equal_reference=false` remain intact.
An owner exit 1 is the strict aggregate here, not an editor crash. Reference
and Cocoa maps match; the terminal has one additional guarded image skip.

| Complete file | Reference | Terminal | Cocoa |
| --- | --- | --- | --- |
| `test/src/thread-tests.el` | 38 pass | 38 pass | 38 pass |
| `test/src/font-tests.el` | 2 pass | 2 pass | 2 pass |
| `test/src/image-tests.el` | 4 pass, 1 skip | 3 pass, 2 skips | 4 pass, 1 skip |
| `test/src/inotify-tests.el` | 3 skips | 3 skips | 3 skips |

All thread guards actually pass. This covers the original thread, mutex,
condition, signal, binding and buffer-disposition assertions in normal batch
products. It does not establish stress, checking-build or collector acceptance.
Font tests parse 49 fixed names with five properties each, plus five XLFD
checks; the interactive helper is not registered or invoked. These are parser
checks, not display/font-backend acceptance.

Twelve skips are shared: nine `featurep inotify` guards and three
`fboundp imagemagick-types` guards. Inotify's five-second read and unbounded
`recursive-edit` are never entered. The commented watch-aspect test is not
registered. No guard or ID was changed to obtain these results.

The thirteenth skip is terminal `image-tests-init-image-library`, guarded by
`fboundp init-image-library`. Reference and Cocoa actually execute its PBM
initialization and invalid-type checks. The terminal lacks that entry point:
its source definition and registration require `have_window_system`, enabled
in the Cocoa target configuration and absent in the headless terminal target.
This is a declared frontend capability difference, not a shared skip or proof
of complete compatibility; it does not justify enabling a window system in
the terminal product. Both configurations and source identities are recorded.

The three nongraphical image error tests pass on all products, but their
original `should-error` accepts any signal, including an undefined function.
They therefore do not prove terminal image-primitive parity. Batch mode makes
`display-images-p` false, so these are executed error assertions, not graphical
skips. Load-time `find-image` probes may initialize local image libraries;
the original lookup names and format pairings are unchanged. No physical
image rendering is accepted by this run.

R42 uses twelve serial file processes, an implemented 30-second load/case
deadline and a 300-second protected-owner deadline. Estimated duration was
10–45 seconds below 1 GiB. Driver time is 32.701476709 seconds. The longest
individual cases are the original thread join-error checks, about five
seconds each; no deadline was extended. The primary controller records exit
1, complete owned cleanup, 35.027 seconds and 106,446,848 accounted peak bytes.
Controller metrics remain primary-side receipts, not independently remeasured
performance or a speedup claim.

Independent source and raw-result review finds no confirmed execution defect
and accepts these limits. This introduces no production change, protection
test, build or App replacement. G1/G2/G4 remain 3/7; original Goal and
G3/G5/G6/G7 remain active.

The [bounded receipt](2026-10-30-original-thread-image-results.json) binds all
144 original case IDs, statuses, diagnostics, timings, raw/event hashes and
product/input identities. Raw evidence remains in
`build/acceptance/g3-thread-font-image-1`. Replay rematerializes the complete
pinned test corpus and isolated contexts, using the existing stable-suite
runner, original guards/assertions, explicit products/dumps and stated deadlines.
Consumed fixture and environment retirement is recorded in
[CLN168](../cleanup/reports/2026-10-30-168-consumed-thread-image-state.md).
