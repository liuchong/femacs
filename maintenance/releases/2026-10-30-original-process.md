# Original stable process comparison

The complete original `test/src/process-tests.el` at pinned stable commit
`a360712c9d272d950d8d8255ef74570f7e90b7d9` runs unchanged on the installed
macOS reference and current normal terminal/Cocoa, with explicit products/dumps,
full libraries and isolated manifest-verified test trees. No ID is filtered.

Each product registers 39 IDs: **37 pass and two skip**, for **111 passes and
six skips**. Three file processes exit 0, complete, without timeout, protocol
error or surviving process group. All three status maps agree. The original
aggregate remains **`passed=false`**, and the protected owner exits 1; shared
platform skips are not passes. All inputs, programs/dumps, full resources and
the entire trial App remain unchanged. No generated test resources remain.

The source contains 40 static definitions. `process-test-quoted-batfile` is
inside an original Windows-only `when`, so it does not register on macOS.
The registered `make-process-w32-debug-spawn-error` and
`process-sentinel-interrupt-event` skip because their original Windows guards
are false. No product change is justified by these common platform conditions.

DNS lookup, network-interface, local socket, pipe/PTY, stderr/filter/sentinel,
thread, reinvocation and four FD-pressure cases pass. Loading performs DNS
before any guard; five guarded DNS cases actually pass, rather than skip.
Their loading query has no separate case receipt. FD tests attempt up to 1025
pipe objects per pass, with at most ten local clients, child processes or
threads. Passing means the original assertions hold; it **does not prove that
1025 descriptors were reached**. This host has no `prlimit`; the original
conditional helper therefore does not raise this process's limit. No global
resource setting is changed. Child reinvocation uses the original current
binary/dump helper, but no separate child dump fingerprint was recorded.
Parent identity alone is not child dump acceptance.

R42 uses three serial file processes, an implemented 65-second per-load/case
deadline and 600-second protected-owner deadline. Original 60-second Lisp
waits and all guards remain. Expected time was 20–180 seconds, below 1 GiB;
actual driver time is 17.259801708 seconds. The longest case is reference
`lookup-google`, 1.329287 seconds. Short owned temporary directories keep the
original local socket under the platform limit; recorded socket length is
91 bytes, plus an eight-byte margin. PTY children can form separate sessions;
the controller tracks observed descendants. Its primary-side receipt records
exit 1, complete cleanup, 18.611 seconds and 209,453,056 accounted peak bytes.
Those controller metrics are separate from independent functional review.

Independent review finds no confirmed defect, checks original guards, complete
maps, input identities and the implemented deadline, and retains the limits
above. No production change, new component protection, build or App replacement
is needed. This is normal batch coverage, not GUI, native, complete G3 or
performance adoption. G1/G2/G4 remain 3/7; the original Goal stays active.

The [bounded public receipt](2026-10-30-original-process-results.json) binds
source, driver, raw result/event hashes and every ID/status/time. Raw evidence
remains in `build/acceptance/g3-process-1`.
[CLN165](../cleanup/reports/2026-10-30-165-consumed-process-state.md) retires
only consumed copies, links and exact owned empty short environments. Replay
rematerializes the full pinned corpus, fresh isolated contexts/short socket
paths and the same products/dumps through `tools/stable-suite.py`,
`tools/stable-suite-batch.py` and `tools/fixtures/stable-suite-runner.el`, with
all original assertions/guards and the same hard deadlines.

The earlier [JSON report](2026-10-30-original-json.md) is corrected alongside
this delivery: its driver actually used 60 seconds per load/case, while the
original descriptive budget said 20. The original raw result and driver remain
unchanged; the implemented whole-owner deadline was 180 seconds. No JSON case
is rerun and none of its 75 passing results changes.
