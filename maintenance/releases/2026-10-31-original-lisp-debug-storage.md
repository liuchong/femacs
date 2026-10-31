# Original Lisp debugging, coverage and cross-session storage

Three unchanged pinned files run once on the installed reference and accepted
P435 normal terminal/Cocoa products. All **nine file processes complete with
exit 0**, without timeout, protocol/runner error or residual process groups.
**252 expected results = 249 ordinary successes + three declared expected
failures**; zero skips or unexpected failures. IDs, tags and status maps agree.

| Original file | IDs per product | Actual boundary |
| --- | ---: | --- |
| `edebug-tests.el` | 49 | 48 bodies plus one forced expected failure |
| `testcover-tests.el` | 30 | Generated from the bound resource, each compiled and interpreted |
| `multisession-tests.el` | 5 | SQLite/file storage and child editors |

The source is pinned to `a360712c9d272d950d8d8255ef74570f7e90b7d9`.
All three files and both instrumentation resources are tracked manifest inputs.
Every discovered ID runs, with one process per file; no assertion, guard, tag,
original source or selection is changed. Earlier L5/GC/all rows are broad scans,
not this current-product strict receipt. There is no new production modification.

Edebug uses original keyboard macros, step/breakpoint/trace and recursive-edit
paths on fixed small source. This is simulated Lisp input, not physical UI.
`edebug-tests-step-into-macro-error` immediately calls `ert-fail` and declares an
expected failure on each product. Its macro-error body is not covered; a green
status does not remove that original limitation.

Testcover generates thirty IDs from its 16,276-byte resource. Each small sample
is instrumented twice in temporary source, with and without in-process byte
compilation. This is actual compiler/instrumentation integration, not an external
product build. Multisession uses isolated temporary storage: two synchronous and
two asynchronous child-editor calls per product. Each busy child performs 100
increments while the parent waits with 0.1-second sleeps. The original SQLite
guards are true on all three products. Child editors finish without residual
process groups; their dumps have no separate fingerprint.

Ten explicit input maps and complete product/resource identities match before
and after. Terminal and Cocoa match the accepted P435 receipts; these are not
P444 or checking products. Three materializations contain 1,536 pinned files /
20,996,952 bytes each. Test-tree generated resources are empty. Reference Edebug
automatically compiles five native trampolines; no active build was requested.
The plans record 170,488 logical / 184,320 allocated bytes for these files.

Execution retains 30 seconds per load/case, 300 seconds for the controlled tree,
and a stricter 1 GiB initial/hard limit with zero growth. The 10--90 second /
512 MiB task estimates are not separate limits. Driver time is 20.708307 seconds;
controlled owner time 23.128 seconds, accounted peak 217,432,064 bytes and footprint
131,156,032 bytes, exit 0 with cleanup complete. These are coverage timings,
not speed comparisons. Independent review checks all 252 event hashes/ordinals,
case classifications, bindings, cleanup and controller evidence without reruns.

CLN179 retires 4,615 files, 855 directories and eighteen links,
63,173,350 logical / 75,464,704 allocated bytes. Nine automatic retirements,
including eighteen parents, count once. Original results/events/logs, private
plans, replay driver, inputs, live products and unrelated WIP remain.

No new protection test or App replacement follows from unchanged coverage.
Whole ERT, checking, full G3, native/collector and performance stay open.
G1/G2/G4 remain **3/7**, original Goal active.

[Result receipt](2026-10-31-original-lisp-debug-storage-results.json).
[Cleanup report](../cleanup/reports/2026-10-31-179-consumed-lisp-debug-storage-state.md).
