# Permanent Full fallback: stop rebuilding an unusable registry

P437 changes only the opt-in collector. Once a foreign-thread or unknown-producer
flag makes all future collections Full, marked non-cons objects no longer
need to be inserted into an index that can never become valid again. Original
marking, child traversal, weak processing, barriers, sweep, errors and ABI stay.
A per-cycle latch replaces the old marking-phase read; flags raised mid-cycle
take effect next cycle, preserving the current frozen-prefix traversal.

## Source and actual functional evidence

[Portable results](2026-10-30-persistent-fallback-results.json) bind exact hashes
and the original per-process results. Frozen source has 7,323 inputs, including
current WIP; it is not a clean-HEAD build. Relative to the retained prior, the
only selected production differences are the collector and P438's ten byte-copy
resource recipes; the added charset protection script is a declared manual input.
Normal and checking collector/observer terminal products were built separately.
The original immutable-copy generation failure remains false and recoverable.

The complete affected language/observer/collector cases passed in 490 new
processes: 441 ERT executions and 49 discovery processes. Permanent fallback
checks added eight new candidate processes and reused eight exact verified prior
processes, for 506 functional receipts. Reuse verifies product/resources,
source/harness/fixture identities and actual JSON observations, not a summary.

Four paired major/young normal/checking observations preserve the exported Lisp
object graph and original module-load error. In young mode, pre-latch registry
counts stay 94,023 (normal) / 94,041 (checking). Across later explicit and two
automatic Full collections, old counts 94,044/94,056/94,067 and
94,062/94,074/94,085 become zero. Major controls remain zero. Initial private
counts differ because fixtures and startup allocations differ; this is not an
assertion that every numerical implementation counter is identical.

After integration passed, the finalizer protection added two ERT executions and
two discoveries. A real finalizer triggers a failed module load, allocates live
objects and calls nested Full GC. Unknown-producer/invalid/Full/empty-registry
state and live graphs pass in normal/checking young profiles; callback failures
are captured explicitly rather than swallowed. This does not inject a foreign
thread, an allocation failure or a flag during marking.

## Timing, adoption and remaining scope

Build helper: 974.986 seconds; functional helper: 257.277 seconds; protection
helper: 2.636 seconds. These are operation durations, not performance gains.
The host is above the existing quiet-load criterion. Roughly 94k avoided index
entries per later Full are proven work removal, not a qualified latency,
throughput or memory improvement. No busy-host speedup or G5/G7 completion is
claimed. The candidate remains experimental, retained beside the prior; the
separately accepted nonexperimental trial App is unchanged.

Independent pair review covers the source lifecycle and actual result files;
its limitations match the cases above. Subsequent qualified performance work
must retain same-machine official/default and matched controls, all rounds,
pause distributions, memory and workload identities. No numerical goal or
acceptance threshold changes.

## Resource retirement

[CLN155](../cleanup/reports/2026-10-30-155-consumed-fallback-candidate-state.md)
retires consumed driver/unit copies and successful case environments. Frozen
source, real objects/products, raw results, per-process receipts and observation
JSON remain. This cleanup does not rerun workloads or change default products.
