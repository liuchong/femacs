# Original stable text-processing comparison

Four complete original stable files from pinned commit
`a360712c9d272d950d8d8255ef74570f7e90b7d9` run on the installed macOS
reference and current normal terminal/Cocoa. Each product uses its own program,
explicit dump, complete selected Lisp libraries and an isolated manifest-verified
test tree. Existing materialization, per-file bridge and ERT runner are reused;
original IDs, bodies, guards and assertions are unchanged.

| Original file under `test/src/` | Reference passed | Terminal passed | Cocoa passed |
| --- | ---: | ---: | ---: |
| `coding-tests.el` | 9 | 9 | 9 |
| `search-tests.el` | 1 | 1 | 1 |
| `syntax-tests.el` | 100 | 100 | 100 |
| `textprop-tests.el` | 3 | 3 | 3 |
| Total | 113 | 113 | 113 |

All **339 observations pass**, without skips, assertion failures, protocol
errors or timeouts. Twelve file processes exit 0, complete and leave no group
to terminate. Original status maps match the reference; all manifest inputs,
harness/helper, programs/dumps/full resources and the entire trial App remain
unchanged during acceptance. No generated resources remain. Independent review
recomputes the original case maps and accepts this bounded result, with no
confirmed defect. There is no production change requiring new component tests.

R42 binds twelve serial file processes, small fixed ASCII/Latin/binary strings,
line-ending conversions and owned temporary files, fixed search inputs, finite
comment-position cases and short text-property/face buffers. Coding's benchmark
generators are definitions only; ERT does not call the 100,000-line generators.
One syntax case byte-compiles a small Lisp lambda. No external compiler,
network, physical UI or intentional crash is requested. Per-case hard deadline
is 30 seconds and the protected owner deadline is 300 seconds. The driver takes
12.052 seconds; its controller exits 0/reaped in 13.742 seconds with 116,949,136
accounted peak bytes. Controller metrics are primary-side receipts, separate
from the independently reviewed functional result; these are not speedup data.

## Preflight failures and terminal runtime restoration

Two failed preflights launched no editor. The first tried an already retired
standalone P435 snapshot. Selecting the retained current terminal program/dump
then exposed three empty runtime-library directories; product-identity checks
rejected them before creating the successful output or starting an editor.
Both controller failures remain exit 1, fully reaped, with separate records.

The current terminal program and dump still exactly match the accepted
`build/acceptance/p435-functional-normal-1/results.json`. Eleven required
dynamic libraries, totaling 8,075,088 logical bytes, were restored from the
current App's Frameworks into `build/terminal/port/runtime-{tls,lcms,treesit}`.
Every source and destination hash, size and mode, and each complete resource
group, matches that accepted receipt. No program/dump was rebuilt or changed;
no App/source or linker settings changed. The subsequent successful terminal
file processes demonstrate startup with the restored dependencies. Review
independently checks those eleven library identities, not the entire App.
The primary driver separately fingerprints the complete App before and after.

The cause of the empty directories is **unknown**; this is not attributed to a
particular cleanup. The restored libraries are current product dependencies,
and are excluded from this round's consumed-test-copy retirement.

## Evidence, replay and limits

[Bounded public receipt](2026-10-30-original-text-processing-results.json)
binds original source hashes, IDs, per-file durations, raw result/event hashes,
program/dump identities, canonical resource-identity digests and restored files.
Raw records remain in `build/acceptance/g3-text-processing-2/results.json` and
adjacent per-file records. The two preflight records and restoration record
remain under `build/acceptance/g3-text-processing-1`.

For replay, materialize three fresh complete trees using
`compatibility/stable-31.1/sources.json` and `tools/stable-suite.py`, recreate
the recorded source/test/library contexts and use the existing per-file path
in `tools/stable-suite-batch.py` with `tools/fixtures/stable-suite-runner.el`,
explicit selected dumps, all IDs and the recorded deadlines. Retired test
paths require rematerialization. [CLN161](../cleanup/reports/2026-10-30-161-consumed-text-processing-test-copies.md)
retires only those three consumed trees and their three test links, preserving
the corpus, raw evidence and actual products.

This is normal source-mode batch coverage of four files, not all 555 stable
entrypoints, the full native/checking/collector matrix, physical UI, performance
acceptance or completed G3. Shared EIEIO/locale/collation failures or skips,
bootstrap identity policy, intermittent remote09 and host DNS retain their
existing classifications. The already accepted process-wait and byte-index
corrections are not reopened by older summaries.

Source-bound pair discussion leaves the frozen-root loop unchanged: P436
already evaluated and withdrew interval batching without qualified gain;
a current-root hint saves at most one deduplication per first-marked root while
adding checks across its descendants and new lifecycle state. Neither becomes
a production modification or a new build. G1/G2/G4 remain 3/7; G3/G5/G6/G7 and
the original Goal remain open. Continue uncovered original compatibility work
and evidence-led runtime work without replaying unchanged busy-host matrices.
