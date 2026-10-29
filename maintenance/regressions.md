# Defect regression register

Keep a confirmed failure, its producing source, the actual repair and a bounded
regression together. Reuse the existing product/process owners and component
test tools; this register does not introduce a second test framework. Each new
entry needs an incident link, actual trigger, assertion, test entrypoint and
acceptance evidence. A row is a coverage map, not a claim that the current
product has passed every entry.

| Priority | Confirmed defect and trigger | Required assertion and entrypoint | Record |
| --- | --- | --- | --- |
| P0 | Background color overwrites a frame Lisp slot; a later GC aborts during normal startup | Two real Cocoa frames, four background colors each, immediate GC before redisplay; `tools/test-macos-background-gc.py`. Actual Swift caller with disjoint receivers and an old-code negative control: `tools/test-macos-background-receiver.py` | [Background receiver](incidents/2026-10-29-cocoa-background-receiver.md), P432 |
| P0 | Mouse/display setter receives a frame as a display pointer; multi-frame input loses valid state | Real A/B composition and mouse transfer plus disjoint receiver sentinels; `tools/test-macos-ime.py` and `tools/test-macos-display-receiver.py` | [Multi-frame input](incidents/2026-10-27-multiframe-input.md), P394; [current native observations](releases/2026-10-29-current-g1-workflows.md) |
| P0 | Native live resize crosses an invalid Swift varargs boundary | Real resize, Unicode title and boundary values in the extracted production formatter; `tools/test-macos-resize-title.py` | [Cocoa layout](incidents/2026-10-27-cocoa-layout.md), P392 |
| P1 | An owned temporary path exceeds the local socket address limit before the original descriptor test runs | Bounded logical-ID directory names; actual original process/socket assertions; replaced/nonempty parent preservation, pre-spawn failure receipt, and final hash-bound cleanup/retention outcome in `tools/test-stable-suite.py` | [Original socket environment](incidents/2026-10-29-original-socket-test-environment.md), P431 |

Native integration runs under `controlled-run` with the runtime profile;
compiler-based component protection uses the build profile. Each invocation
uses a fresh owned output and a hard deadline. A crashed/missing GUI, timeout,
missing output, skipped precondition or incorrect receiver cannot count as a
pass. Keep original failures and per-case timings. Personal configuration is
only loaded for an explicitly authorized host-level check and is never copied
into a reusable fixture or committed as evidence.

The background regression can be run against a selected Cocoa product:

```sh
python3 tools/test-macos-background-gc.py \
  --binary dist/macos/Femacs.app/Contents/MacOS/Femacs \
  --out build/acceptance/background-gc
```

Wrap this workload with the current installed resource controller. An optional
`--reference` selects the installed reference executable; no reference source
checkout is needed. The program must create real native views, so `--batch`
is intentionally not used.

The existing `tools/local-acceptance.py --gui` group also invokes this native
regression. Caller/sentinel protection remains a separate compiler-based check,
`tools/test-macos-background-receiver.py --out <fresh-build-child>`; it is not
silently added to a lightweight unit-test command. The existing direct native
regression was validated before adding this group registration; unrelated
acceptance groups are not rerun merely for that registration.
