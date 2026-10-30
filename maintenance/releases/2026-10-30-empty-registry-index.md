# Empty registry index: local candidate evidence

Status: locally validated, independent result review pending. The P440 production
change remains uncommitted; it has not been promoted into the trial App.

The candidate skips the 2,097,152-byte index fill only when Registry length is
already zero. Successful insertion establishes a nonzero length immediately
after writing an index slot; clearing a populated registry still zeros the full
index. Length, frozen length, validity and overflow reset unconditionally.
This removes redundant writes, not roots, marking, sweep or error behavior.

## Actual products and functional results

The frozen source contains 7,348 declared inputs, including current WIP; it is
not a clean-HEAD build. The only selected production difference from the retained
P437 prior is `src/runtime_gc_collector.rs`. Measurement tooling differences are
disclosed in the portable results. Normal and checking terminal collector/observer
products were built, each with major and young profiles and without test hooks.

The complete affected language/observer/collector integration passes: 480 new
existing-case processes and eight new fallback processes, plus ten verified
reference and eight verified prior receipts, for 506 functional receipts.
All 48 registration keys and their ordered case IDs match the retained prior;
153 language observations and four fallback comparisons pass. Public object
graphs, module errors and post-latch collection deltas agree. This does not close
the full stable compatibility corpus or native UI acceptance for this candidate.

The candidate-only runner's original aggregate is **false** because it omits
reference processes; all 480 of its cases pass. The untouched original is retained
and hashed. A separate combined result uses the ten verified reference receipts;
no original failure is overwritten or relabeled.

After integration passed, five isolated protection tests passed using the actual
numeric source module. They cover initial state, populated clear, duplicates and
freeze, repeated empty metadata reset, reinsertion and real 262,144-entry capacity
failure followed by reuse. The roughly 4 MiB registry has static storage, avoiding
test-thread stack overflow. Explicit 64-bit primitive substitutes isolate this
component; these tests do not simulate GC roots or replace real product acceptance.

## Performance and remaining decision

Ordinary lifecycle measurement ran 315 workload processes and 45 startups;
explicit permanent-fallback measurement ran 90 workload processes. Both retain
two warmups, seven measured rounds, all 1,890 workload samples, pause observations,
memory and original results. There are 96 ordinary and 24 fallback direct pairs.
The installed reference includes its actual native mode; matched prior/candidate
comparisons use interpreted and bytecode modes without pretending native parity.

All 270 measured groups exceed the existing quiet-host load criterion. Therefore
zero direct pairs qualify for an improvement claim. Source proof and fewer writes
do not establish throughput, pause-latency or memory improvement. No threshold,
default, ABI or compatibility requirement changes; G5/G7 remain open.

The partner supplied and corrected the integration driver, which the primary
reviewed and ran. The partner's next result review was blocked by a provider access
error, so independent review of these executed results and the new protection
driver remains pending. This report records local evidence, not final adoption.

[Portable results](2026-10-30-empty-registry-index-results.json) bind exact hashes,
summary statistics and original raw outputs. [CLN157](../cleanup/reports/2026-10-30-157-consumed-empty-index-state.md)
records retirement of consumed compiler and case state while preserving actual
sources, products, raw observations and the trial App.
