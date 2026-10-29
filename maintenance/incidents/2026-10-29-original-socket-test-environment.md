# Original socket test blocked by its owned temporary path

## Cause and bounded repair

The batch owner repeated a full logical file ID in a deep container and again
in a case directory. The unchanged original local-socket descriptor test then
exceeded Darwin's 104-byte socket address before reaching the descriptor-limit
assertion. This was a test-environment defect, not evidence of a product socket
or descriptor repair. PORTING.md P431 records the source-bound design.

`tools/stable-suite.py` now uses a fixed `case-<16hexSHA>` directory component;
full logical IDs remain in reports and mock-shell selection. For the original
POSIX `test/src/process-tests.el` entrypoint only, the batch owner measures the
final temporary directory plus ERT's actual default socket leaf and eight bytes
of headroom. When necessary it creates an exclusive short container under the
runtime user's AI cache directory and checks the resulting path again. Other
entrypoints and original test source/assertions are unchanged.

A preparation receipt identifies the container before case setup. Setup errors
retain partial state and a failure receipt without starting an editor. Successful
and fully reaped cases retain the existing member-identity cleanup, followed by
identity-checked removal of empty case/container directories only. Replaced,
nonempty, failed or unknown state is retained; no recursive fallback removal,
old-state migration or shared executor change is introduced.

## Actual integration and limits

The installed official macOS reference, terminal and Cocoa candidates each ran
the full unchanged 39-case original process file. The first observation had
35 passes, two failures and two original skips per product. One failure is the
host DNS response for `emacs.invalid`; the other was diagnostic loader output
polluting a debug-output assertion while running with an interposer. That
original observation remains a failure, not an all-green result.

The second observation removed that interposition and retained all original
cases/assertions: each product had 36 passes, one DNS failure and two original
skips. The original local-socket/descriptor case passed on all three. The DNS
assertion still fails identically because the host returns `198.18.0.230` for
the invalid name; it is not waived, masked or changed. Whole original-file
results remain false. Unchanged LCMS lifecycle coverage from the first run
passed seven cases per product and verified successful empty-parent retirement.
The combined distinct passing case/product observations are 129 (108 plus 21),
with three retained common DNS failures and six original skips in the final
process-file observation. This is limited environment-owner acceptance, not
complete compatibility, a P427 EBADF repair or a performance result.

After integration, all 25 existing and new lightweight contract cases passed.
New protection covers bounded distinct names for long logical IDs, preservation
of replaced parents and unexpected state, and preparation-failure receipts
before any editor spawn. Per-case timings and original raw failures remain.

Private evidence is retained under `build/acceptance/p431-integration-1`,
`p431-integration-2` and `p431-protection-1`. The first two records bind their
actual older product/source identities; the later P432 Cocoa receiver repair
is not claimed to have rerun these tests. No product was rebuilt or retested
for documentation. The reusable contract entrypoint is
`tools/test-stable-suite.py`; actual original-suite execution uses the existing
batch runner and pinned stable test inputs.

## Final review follow-up

Independent review accepted the bounded path behavior and initial 25 guards,
but found that the immutable per-file result predates cleanup and lacked its
final owner outcome. The completed implementation names a separate exclusive
owner-retirement receipt in the original file result. That receipt persists the
member/empty-parent outcome, including retained failures, and binds it to the
unchanged original file result hash. It does not rewrite the evidence used by
the existing member-retirement ledger.

The earlier private integration summaries used `passed=true` for the narrow
environment contract despite retained failed original files. Those raw
observations remain; interpret that flag only as historical scope acceptance,
never whole-suite success. The follow-up reporter separates
`environment_contract_passed` from actual `passed`, which remains false when
an original assertion fails or a requested case skips. Protection evidence is
in the build acceptance directory, not an auxiliary cache path.

The third integration uses the installed reference, current terminal and current
P432 Cocoa App with executable SHA-256
`b2fe7420ad1c3a041ea33ea9d0ccf80b852b818ea3315ffa65dd1aff84b2bcb0`.
Each has 36 passes, one common DNS failure and two original skips in the process
file, plus seven LCMS passes. All six raw per-file results locate the new owner
receipt; all receipt hashes, original result identities and recorded retirement
outcomes agree. The environment contract passes; whole-suite `passed` remains
false. Products, dumps and inputs are unchanged. The protected run completed
with full cleanup in 16.644 seconds, peak accounted 187,531,264 bytes. A prior
host-pressure admission rejection did not execute the editor and is not a test
failure or pass.

After this integration, all 29 lightweight guards passed with individual case
timings. The four added guards cover completed outcome/hash preservation,
retained failure reasons, changed-evidence rejection and existing-receipt
preservation. New evidence is `build/acceptance/p431-integration-3` and
`p431-protection-2`. Four actual read-only incremental checks accepted this
limited infrastructure and regression group. Receipt contents are fsynced,
without a claim of crash-durable directory metadata. P427, complete G3 and
performance acceptance remain open.
