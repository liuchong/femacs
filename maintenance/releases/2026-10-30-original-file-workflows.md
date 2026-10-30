# Original stable undo and file-workflow comparison

Four complete pinned stable files at commit
`a360712c9d272d950d8d8255ef74570f7e90b7d9` run on the installed macOS
reference and current normal terminal/Cocoa, each with its explicit dump,
complete libraries and isolated manifest-verified test tree. The existing
per-file bridge and runner keep every original ID, guard, body and assertion.

| Original file under `test/src/` | Reference pass/skip | Terminal pass/skip | Cocoa pass/skip |
| --- | ---: | ---: | ---: |
| `undo-tests.el` | 19/0 | 19/0 | 19/0 |
| `fileio-tests.el` | 15/4 | 15/4 | 15/4 |
| `filelock-tests.el` | 6/0 | 6/0 | 6/0 |
| `callproc-tests.el` | 1/2 | 1/2 | 1/2 |
| Total | 41/6 | 41/6 | 41/6 |

There are **123 passed and 18 skipped observations**, 47 IDs per product.
No assertion failure, timeout or protocol error occurs; all twelve file
processes exit 0, complete and leave no group to terminate. Original status
maps match across all three products. The aggregate **`passed=false` and
outer exit 1 are retained**, because the original assessment does not count
skips as passes. Equal maps do not turn this into complete compatibility.

The six skipped IDs per product are original platform guards on Darwin:

- `fileio-tests--directory-file-name-dos-nt` and
  `fileio-tests--file-name-as-directory-dos-nt` require DOS or Windows.
- `fileio-tests-invalid-UNC` and `fileio-tests-w32-time-stamp-granularity`
  require Windows.
- `call-process-w32-debug-spawn-error` and `initial-environment-preserved`
  also require Windows.

The random-device file-I/O guard did not skip. The local file-lock utilities
were available and all six lock cases passed per product. The executed process
case calls the selected program with `--version`; it is not proof that a child
loads the parent's dump. These common platform skips require no product patch.

## Bounds, evidence and independent review

R42 binds twelve serial file processes and owned small shell/version children,
60 seconds per case and a protected 360-second owner deadline. `undo-test4`
retains all 1,048,576 alternating seven/six-byte inserts: 6,815,744 text bytes,
with original undo and equality checks. Other undo inputs are short fixed edits,
markers and properties. File-I/O uses 127 non-NUL ASCII symlink targets, bounded
random input and small owned files; file-lock uses six bounded local fixtures.
No network, physical UI or intentional crash is requested. The estimate is
20–150 seconds and below 1 GiB; actual driver time is 10.774973708 seconds.
The primary controller exits 1 and fully reaps in 13.764 seconds, with
239,763,456 accounted peak bytes. Those controller metrics are primary-side
receipts, separate from independent functional review; they are not speedup.

All original inputs, programs/dumps, full resources and the trial App remain
unchanged. No generated resources remain. Independent review recomputes the
three case maps, checks original platform guards and unchanged full undo input,
and reports no confirmed defect. There is no production change requiring new
component protections or App replacement.

The [bounded public receipt](2026-10-30-original-file-workflows-results.json)
binds original file hashes, every observed ID/status/time, raw result/event
hashes, program/dump identities and canonical resource-identity digests. Raw
records remain in `build/acceptance/g3-file-workflows-1` with the original false
aggregate and retained per-file retirement records. Terminal program, dump and
four resource groups remain equal to the accepted normal-product identities;
the eleven restored runtime libraries remain live dependencies.

## Retirement, replay and remaining scope

[CLN162](../cleanup/reports/2026-10-30-162-consumed-file-workflow-state.md)
records the runner's earlier automatic retirement separately from subsequent
retirement of three consumed test trees, three test links and remaining owned
environment trees. Original skip observations, raw evidence, source corpus,
products and resources remain. Replay requires fresh manifest-verified test
copies and isolated environments, restored context/test links, the same product
identities, all original IDs/guards/assertions and the recorded deadlines.
Use `tools/stable-suite.py`, `tools/stable-suite-batch.py` and
`tools/fixtures/stable-suite-runner.el`; old retired paths cannot be reused.

This is normal batch coverage of four full files, not all 555 stable entrypoints,
native/checking/collector or physical UI acceptance, speedup or completed G3.
G1/G2/G4 remain 3/7; G3/G5/G6/G7 and the original Goal remain open. Continue
uncovered original data/time/resource files; preserve existing shared failure,
locale/feature skip and intermittent-remote classifications. Interval marking
and current-root hints remain outside production.
