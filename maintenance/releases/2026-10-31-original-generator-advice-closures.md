# Original generator, advice and closure coverage

Three unchanged pinned files run on the installed reference and the accepted
P435 normal terminal/Cocoa products: **369 expected results, comprising 363
ordinary passes and six declared expected failures**, zero skips or unexpected
failures. All nine independent file processes complete with exit 0, no timeout,
protocol error, runner error or remaining process group. ID, tag and status maps
match across products. Independent source and raw-result review accepts this scope.

| Original file | IDs per product | Registration and expectation |
| --- | ---: | --- |
| `generator-tests.el` | 92 | Twelve direct IDs plus forty macro cases with normal/noopt variants |
| `nadvice-tests.el` | 13 | Two original `:expected-result :failed` declarations per product |
| `cconv-tests.el` | 18 | Two commented deftest forms are not registered |

The advice IDs `advice-test-called-interactively-p-around` and
`advice-test-called-interactively-p-filter-args` each produce `ert-test-failed`
with expected true. The runner correctly reports these six as expected results;
they are not six additional IDs or six successful assertion bodies.

## Identity and execution

Pinned input commit: `a360712c9d272d950d8d8255ef74570f7e90b7d9`.
The three tracked files match `compatibility/stable-31.1/sources.json`.
Each isolated materialization contains 1,536 files / 20,996,952 bytes. Dynamic
all-ID discovery is unchanged: no ID, tag, guard, assertion or runner edit.

Ten explicit input maps are identical before and after; program/dump and full
resource identities also match. The driver asserts terminal identity against
`p435-functional-normal-1` and Cocoa program/dump/bundle/resources against
`current-macos-core-products-1` normal. These are reused accepted P435 products,
not a P444 rebuild or checking run. Raw metadata reports reference 31.1 and
current terminal/Cocoa 32.0.50, with the system configuration and full version
text hashes in the receipt. All products run in batch mode.

Generator work uses finite fixed graphs, two explicit GCs and one edebug entry;
advice uses isolated global function/advice state; closure conversion includes
one in-process byte compilation. No explicit child, network or positive sleep
workload. The reference automatically creates one native trampoline; absence of
an active build request does not mean no automatic compilation occurred.

Deadlines: 30 seconds per load/case, controlled owner 300 seconds; estimated
10--90 seconds / 512 MiB. The estimate is not a limit. The sampled controlled
tree has an explicitly stricter **1 GiB hard limit**, unchanged with zero growth.
Driver 10.158424 seconds; controller 11.823 seconds, 229,277,696 accounted peak
bytes, exit 0 and cleanup complete. The original controller log was independently
reviewed. These are coverage/resource observations, not a speed comparison.

## Delivery boundary

No production edit or new protection test follows from these passing original
files. No trial App replacement, full G3, checking, whole ERT, GC or performance
acceptance. G1/G2/G4 remain **3/7**; the original Goal continues. Continue the
remaining compatibility and evidence-led runtime work without replaying an
unchanged capped or busy-host workload as new acceptance.

The retained driver is the exact replay recipe, built on the existing
`stable-suite-batch` runner and complete pinned materialization. Original results,
events, logs and input/product/automatic-retirement identities remain locally
for audit. Per-process identities and aggregates are in the public receipt;
raw local paths do not substitute for the scope limits above.

CLN174 retires 4,609 files, 855 directories and eighteen links: 63,024,984 logical /
75,300,864 allocated bytes. This includes one automatic reference trampoline and
93 already-retired directories, counted once. Three test/environment/context
scopes are absent; products, inputs, original evidence and unrelated WIP remain.

[Result receipt](2026-10-31-original-generator-advice-closures-results.json).
[Cleanup report](../cleanup/reports/2026-10-31-174-consumed-generator-advice-state.md).
