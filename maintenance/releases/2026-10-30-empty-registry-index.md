# Empty registry index: local candidate evidence

Status: implementation, affected integration and protection reviewed independently.
This delivery commits the P440 source and protection driver. It does not promote
the measured candidate into the trial App or accept its performance.

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

The initial partner supplied and corrected the integration driver, which the
primary reviewed and ran. After a provider access failure, the temporary reviewer
independently checked the current production change, protection driver, observation
checks and acceptance predicates, registrations and original result hashes.
No confirmed defect was found in the clear invariant or recorded predicates.
The review did not cover every integration-driver line or the entire compatibility
corpus; passing this review is not full candidate or performance adoption.

The review also identified unequal pre-latch registry lengths in young mode:
normal prior/candidate initial 92,800/93,113 and pre-latch 94,023/93,982;
checking initial 92,651/93,055 and pre-latch 94,041/94,000. The immediate latched
length equals each product's own pre-latch length; post-latch collection stages
are zero on both sides and public observations/deltas agree. Cross-process
pre-latch lengths are not an equality predicate in this fixture. The repeated
41-entry difference is not explained by the current evidence; it is retained as
an unresolved observation, not dismissed as noise or proof of marking equivalence.
The initial review and implementation-delivery report reversed the two product
labels; these values are corrected directly against old/new raw comparisons.

Follow-up source and receipt review finds the same pre-latch live-cons difference
in both major and young mode: normal prior/candidate 38,782/38,741; checking
38,788/38,747. This counter is `snapshots.<stage>[1][6]`, copied from
`gcstat.total_conses`; the registry counter is `[1][11]`. Major does not register
non-cons owners and its registry stays zero. Thus the live-cons difference is
not solely an artifact of young registry counting. Different dump hashes/sizes
prove unequal input bytes, not the cause of this difference. There is no retained
object-level census; smaller live heaps and missed marking are not separated by
these aggregates. The empty-clear invariant remains supported independently by
the source proof, not proved or disproved by the equal count differences.

The protection tests lock the reachable clear postconditions, not reduced writes:
restoring unconditional fill would still pass. They do not manufacture unreachable
empty-length/dirty-index states or assert timing. The source invariant and real
functional results support this bounded implementation delivery; the unresolved
observation and unqualified performance still prohibit broader acceptance claims.

[Portable results](2026-10-30-empty-registry-index-results.json) bind exact hashes,
summary statistics and original raw outputs. [CLN157](../cleanup/reports/2026-10-30-157-consumed-empty-index-state.md)
records retirement of consumed compiler and case state while preserving actual
sources, products, raw observations and the trial App.
