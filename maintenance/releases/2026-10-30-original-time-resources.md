# Original stable time and resource comparison

Four complete original stable files at pinned commit
`a360712c9d272d950d8d8255ef74570f7e90b7d9` run on the installed macOS
reference and current normal terminal/Cocoa. Each uses its explicit dump,
complete libraries and isolated manifest-verified test tree. Existing bridge
and runner preserve all original IDs, bodies, guards and assertions.

| Original file under `test/src/` | Reference pass/skip | Terminal pass/skip | Cocoa pass/skip |
| --- | ---: | ---: | ---: |
| `timefns-tests.el` | 16/0 | 16/0 | 16/0 |
| `sqlite-tests.el` | 12/1 | 12/1 | 12/1 |
| `lcms-tests.el` | 6/0 | 6/0 | 6/0 |
| `xml-tests.el` | 1/0 | 1/0 | 1/0 |
| Total | 35/1 | 35/1 | 35/1 |

The 36 IDs per product produce **105 passes and three skips**, no failed
assertion, protocol error or timeout. All twelve file processes exit 0,
complete and leave no group to terminate; the original status maps agree.
The aggregate **`passed=false` and owner exit 1 remain unchanged**: the
original assessment does not count skips as passes.

The sole skipped ID is `sqlite-load-extension`. All three products pass the
initial `sqlite-available-p` guard, then skip because the original
`(fboundp 'sqlite-load-extension)` guard returns nil. No extension load is
attempted. The twelve other SQLite cases pass, including `sqlite-returning`
whose minimum-version guard does not skip. This common capability omission
is not a candidate-only failure or an extension-loading pass. No product
change is made to bypass that guard.

## Bounds and observations

R42 uses twelve serial processes, a 30-second per-case limit and protected
300-second owner deadline, estimated 10–90 seconds and below 1 GiB. Time
arithmetic uses a fixed roughly fifty-value pair matrix; binary64 precision
checks halve to zero and double until conversion rejects, with eight fixed
factors per step. The zone string is 2,048 characters. There is no unbounded
search or growing input stream. SQLite uses a few in-memory rows, fixed
strings/blobs and original capability/version guards. LCMS uses fixed color
triples and original tolerances. XML parses three short fixed documents,
including preserved comments. Inputs and assertions are not reduced. No
network, physical UI, external compiler or intentional crash is requested.

The driver completes in 7.187232667 seconds. Its primary controller exits 1,
fully reaped, in 8.406 seconds with 82,280,448 accounted peak bytes. These
controller metrics are primary-side receipts, separate from independent
functional review; they do not establish speedup. All original inputs,
programs/dumps/full resources and the trial App remain unchanged. No generated
resources remain. Terminal program, dump and four resource groups still
match the accepted normal-product identities.

Independent review recomputes original case maps and checks the actual guard
diagnostics, with no confirmed defect. There is no production change requiring
new component protections, rebuild or trial-App replacement. These are ERT
states, not proof of child dump loading, native compilation or performance.

## Evidence, retirement and remaining scope

The [bounded public receipt](2026-10-30-original-time-resources-results.json)
binds source hashes, every ID/status/time, raw file/event hashes, program/dump
identities and canonical resource-identity digests. Original results and
retirement receipts remain in `build/acceptance/g3-time-resources-1`, including
the false aggregate and skipped extension observation.

[CLN163](../cleanup/reports/2026-10-30-163-consumed-time-resource-state.md)
distinguishes the runner's earlier nine successful environment retirements
from post-review deletion of consumed test/environment trees and test links.
Raw evidence, pinned corpus, products, full App and live terminal libraries
remain. Replay uses fresh complete manifest-verified copies and environments,
recreated context/test links, the same explicit products/dumps and deadlines,
`tools/stable-suite.py`, `tools/stable-suite-batch.py` and
`tools/fixtures/stable-suite-runner.el`. Retired paths cannot be reused.

This is four-file normal batch coverage, not all 555 stable entrypoints or
native/checking/collector/physical UI acceptance. G1/G2/G4 remain 3/7;
G3/G5/G6/G7 and the original Goal remain open. Continue untested original
compatibility work and evidence-led runtime implementation; do not turn shared
capability skips into invented product defects or repeat unchanged busy-host
performance matrices.
