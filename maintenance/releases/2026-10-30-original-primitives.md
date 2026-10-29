# Original stable primitive and Lisp I/O comparison

Six complete original stable files run on the installed macOS reference and
current normal terminal/Cocoa products. The unchanged pinned corpus commit is
`a360712c9d272d950d8d8255ef74570f7e90b7d9`. Each product uses a separate,
complete manifest-verified test copy and its own program, explicit selected
dump and Lisp libraries through the existing per-file ERT bridge. Original
bodies, guards, registration and assertions are unchanged; all discovered IDs
are selected without tag filtering.

## Actual results

Names below are relative to `test/src/`. Each product registers 240 original
IDs, of which 237 pass and three skip.

| Original file | Reference passed | Terminal passed | Cocoa passed | Skipped per product |
| --- | ---: | ---: | ---: | ---: |
| `floatfns-tests.el` | 29 | 29 | 29 | 0 |
| `casefiddle-tests.el` | 10 | 10 | 10 | 1 |
| `charset-tests.el` | 2 | 2 | 2 | 0 |
| `fns-tests.el` | 93 | 93 | 93 | 2 |
| `lread-tests.el` | 55 | 55 | 55 | 0 |
| `print-tests.el` | 48 | 48 | 48 | 0 |
| Total | 237 | 237 | 237 | 3 |

There are **711 passes and nine skips**, with zero assertion failures or
timeouts. All three ID/status maps match. The aggregate remains
`passed=false`, and its controller exits 1; equal skipped status is not a
pass or proof of the skipped behavior. All eighteen individual file processes
complete with exit 0 and no remaining process group terminated.

The three unchanged guards skip the same IDs on all products:

- `casefiddle-turkish` requires the exact locale name `tr_TR.utf8`. The actual
  diagnostic lacks that spelling and contains `tr_TR.UTF-8`. This records a
  naming mismatch, not a successful Turkish case-conversion test. The original
  locale list or guard is not replaced, and no host locale is installed.
- `fns-tests-collate-strings` and `fns-tests-collate-sort` require
  `fns-tests--collate-enabled-p`, which returns nil. That helper checks both
  non-fallback collation behavior and acceptance of its named locale. The
  guard result alone does not establish which condition failed; it is not
  reported as a proven missing locale or library.

## Resource and identity evidence

The prior R42 inspection identified at most 100,000 sort items, a 3,000-item
sort with forced GC every 500 comparator calls, seven sizes by 32 keyword-sort
combinations, four bounded weak-table modes with joined threads, and a reader
fixture with 10,000 nested levels. Floating-point fixtures include roughly
2,100-bit ratios; huge exponents are used with trivial bases. Inputs are
bounded and cases run serially, with a 45-second per-case deadline and a
600-second owner deadline. There is no external compiler or network workload.

The group takes 10.179 seconds; its controller takes 12.092 seconds, peaks at
114,147,328 accounted bytes and fully cleans its process tree. These elapsed
values are execution evidence, not a performance improvement comparison.

All 1,536 original input files per copy, manifest/harness/helper identities,
programs, selected dumps and complete selected Lisp libraries are checked
unchanged. No generated resource files remain in the three test copies.
Per-case events, timings, source/product fingerprints, original skips and
false aggregate remain in `build/acceptance/g3-primitives-1/results.json` and
its adjacent bridge records. Product hashes can be checked against the bound
`products_before` and `products_after` fields; no new product is built.

## Replay and limits

Use `tools/stable-suite.py --materialize` with
`compatibility/stable-31.1/sources.json` to create three fresh isolated test
roots from the pinned corpus. Recreate each context's `test` link to its own
copy, `lisp` link to that product's libraries, and recorded read-only source
context. Use the existing `tools/stable-suite-batch.py` per-file path and
`tools/fixtures/stable-suite-runner.el`, explicit selected dump, all IDs and
45-second case deadline for the six files above. Run the whole protected job
with the same resource boundaries. After cleanup these copies must be
rematerialized; saved results are not a runnable source tree.

This adds limited normal-product primitive and Lisp reader/printer coverage.
It does not close the complete stable corpus, every platform/feature/mode,
native compilation, collector candidates, performance or G3. Original native
bootstrap failures, P427, remote Base64, common DNS and interactive EIEIO
failures remain open. The pending single-bootstrap acceptance-policy question
has no human approval; its original failure is unchanged. G1/G2/G4 remain
three of seven delivered outcomes, and the full Goal remains active.

Independent review accepts this bounded result after three actual read-only
tool calls (four focused checks self-reported). It independently checks the
raw counts/status maps, original guards and documentation limits. Consumed
copies are retired in [CLN141](../cleanup/reports/2026-10-30-141-consumed-primitive-test-copies.md);
original records and identities remain, and replay needs rematerialization.
