# Current-core macOS graphical product

Fresh normal and checking Cocoa baseline products contain the accepted P435
Base64 byte-domain and P427 process-wait ownership fixes. The withdrawn P436
batch candidate is absent; native compilation, experimental collector, observer
and test hooks are not enabled by this baseline selection. No new performance
claim or whole-Goal completion follows from this delivery.

## Source and build

The build freezes 7,313 current tracked/new source inputs, including unrelated
existing WIP. It is **not a clean checkout build**. The actual fns/process/thread,
collector and Cocoa event source hashes match committed source at snapshot time;
other WIP is neither committed nor reset by this unit. Both complete compiler
legs build into separate owned output roots, link/dump, generate the 68-manual
set and package a complete App. Both strict signatures pass. The resource
controller exits 0 with complete cleanup in 673.464 seconds, with 880,214,472
peak accounted bytes. Source/program/dump/full-bundle identities remain bound.

## Functional acceptance, then protections

- Two products pass 20 ERT functional observations: nine Base64 integration and
  unchanged original cases plus one original concurrent process case each.
  Four additional public process-output observations preserve nil/true
  `JUST-THIS-ONE` behavior.
- The normal App loads the user's existing configuration in 11.979455375 seconds,
  observes 443 installed package entries, completes initialization without an
  initialization error and reports a live graphical NS frame. Checking starts
  with `-Q` in 8.537672917 seconds. Configuration and product identities are
  unchanged. This is startup evidence, not acceptance of every package feature.
- Five normal-product PID-directed native key workflows pass with normal exit:
  `C-a`, `C-e`, left arrow, `C-k C-y`, and `C-g` cancellation/recovery. The prior
  verified event-sender binaries are reused with exact hashes, and the original
  input source is restored. This is not a repeat of full IME/drop acceptance.
- After all these functional checks, fourteen focused ERT observations pass:
  complete byte-domain/quartet-slot protection, EOF/payload/other-process
  delivery, 140 nested waits/unwind, and GC during suspended waits/output
  filters, in both products. No arbitrary unbounded stress run is used.

| Controlled stage | Seconds | Peak accounted bytes | Result |
| --- | ---: | ---: | --- |
| Functional | 7.930 | 69,157,392 | exit 0, reaped |
| Actual NS startup | 23.888 | 558,645,248 | exit 0, reaped |
| Native keys | 7.663 | 187,203,584 | exit 0, reaped |
| Subsequent protections | 7.142 | 78,021,136 | exit 0, reaped |

The first functional preflight rejects a wrong driver path before creating its
output or launching an editor: the pinned process test is in the compatibility
overlay, while the root logical path holds a different version. The driver now
resolves the frozen manifest's exact `source_path` and retains the hash check;
no input, assertion or product is changed. The original preflight failure is
retained. Base64's original file is already the manifest-matching root file.

The [bounded results](2026-10-30-current-macos-core-results.json) retain individual
ERT names/durations, raw-result hashes and actual source/program/dump bindings.
Original records remain in the corresponding `current-macos-core-*` acceptance
roots. Individual process limits are 15–25 seconds for the bounded core cases,
150 seconds for each startup, and the existing finite native-key waits; whole
controller limits are 220/360/250/220 seconds. No passing old product substitutes
for these new Cocoa product observations.

## Publication

Independent review verifies the frozen source, both actual products, four
acceptance stages, controller receipts and the exact publication operation.
The normal App is now published at `dist/macos/Femacs.app` through a verified
staged copy and native atomic directory exchange. The complete previous App is
retained at `dist/macos/.Femacs.before-current-core-1.app`; no user process is
terminated and no previous file is deleted. Staged and published strict
signatures pass, and the published full-bundle identity exactly matches the
accepted candidate. A new actual program process confirms its data and Lisp
resources resolve inside the published bundle, byte 255 is rejected by Base64,
and a synchronous subprocess returns the expected output. The controller exits
0, fully reaped, in 6.644 seconds with 37,814,272 peak accounted bytes.

[CLN153](../cleanup/reports/2026-10-30-153-consumed-cocoa-core-state.md)
retires only consumed driver Cargo/generated-unit copies and successful isolated
core/checking-startup environments. It preserves exact frozen source, actual
objects/configuration/receipts, both candidate Apps, the current and previous
Apps, native-key event environment and all raw results. This is generated-state
cleanup, not another compiler/test run or source retirement.

## Remaining scope

G1/G2/G4 remain the three accepted outcomes. Full G3/G5–G7, qualified performance
improvement, the original compiler acceptance-policy choice and the retained
intermittent remote09 failure remain open. This build does not waive any of them.
