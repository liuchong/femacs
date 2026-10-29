# Original interactive-call, documentation, profiler and display cases

Run six complete pinned stable files unchanged on the installed 31.1 reference,
current normal terminal program and current P432 Cocoa package. Bind each actual
program, selected dump and complete Lisp tree before/after the comparison.
These are batch/source cases, including synthetic queued input, rather than
physical keyboard/mouse or whole compatibility acceptance.

| Original file | Registered IDs per product | Result per product |
| --- | ---: | --- |
| `test/src/callint-tests.el` | 4 | Four pass |
| `test/src/doc-tests.el` | 5 | Five pass |
| `test/src/profiler-tests.el` | 2 | Two pass |
| `test/src/terminal-tests.el` | 1 | One passes |
| `test/src/xdisp-tests.el` | 11 | Ten pass; original batch guard skips one |
| `test/src/xfaces-tests.el` | 3 | Three pass |

All three products have the same 26-ID registration/status map: 75 passes and
three skips overall, zero assertion failures/timeouts. The false aggregate and
owner exit 1 remain because skipped cases are not passes. All eighteen file
processes exit 0 and leave no surviving group. The outer owner completes cleanup
in 10.495 seconds, with 91,324,416 peak accounted bytes. It has a 180-second hard
deadline and each original case has a 15-second deadline.

The tests keep fixed short buffers/events, documentation queries and color
tables. Both real CPU and memory profiler start/stop/log interfaces execute;
there is no throughput or allocation benchmark in those two tests. The mode-line
test's large integer is a face argument, not an allocation size. No network,
external compiler, source/assertion mutation or intentional crash is requested.

## Actual graphical follow-up

The same `xdisp-test-format-mode-line` ID runs unchanged in separate actual NS
reference and current Cocoa processes, with `noninteractive=false`, a graphical
display and one frame observed. Both pass in 1.538 seconds; the outer owner exits
0 and completely reaps its tree in 2.927 seconds, with 182,927,360 peak accounted
bytes and a 60-second deadline. This is two additional mode observations, not
new distinct IDs or a waiver of the three batch skips. It is not human input or
Finder acceptance. All selected program/dump/library and harness/input identities
remain unchanged; the isolated test trees gain no generated resource files.

Raw evidence is `build/acceptance/g3-display-services-1` and
`build/acceptance/g3-mode-line-interactive-1`. Restore consumed owned test copies,
environments and context links before replay; the complete pinned corpus and
manifest remain available. [CLN145](../cleanup/reports/2026-10-30-145-consumed-display-service-test-copies.md)
records that later classified cleanup without rewriting raw statuses.

## Separate parser confirmation and remaining scope

Current original C-parser preprocessing and GCC-output parser cases also pass
on reference/current terminal/current Cocoa: six observations, two original IDs.
This confirms the existing P415 resource-generation repair at the current inputs,
not a new VM fix. The initial auxiliary attempt registered no cases because the
driver's required input path was absent. That raw false result remains; only the
auxiliary input copy was corrected. The corrected run exits 0 with complete
cleanup in 7.862 seconds. Evidence is `build/acceptance/g3-current-semantic-1`
and `build/acceptance/g3-current-semantic-2`; these counts are separate from the
26-ID table above.

G1/G2/G4 remain accepted, three of seven outcomes. Full G3, P427, intermittent
remote decoding, common DNS/EIEIO/compiler metadata failures and G5-G7/performance
remain open. No policy or original bootstrap assertion is relaxed, and the Goal
is not completed by this additional bounded coverage.
