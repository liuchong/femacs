# P444: reuse a validated cons allocation for child location checks

The opt-in collector keeps its original parent validation. An exactly aligned
cons child inside that parent's allocated block prefix can reuse this location
proof, avoiding a second memory-tree lookup. Current-head limits are respected.
Other-block, dump, null, metadata, unallocated and misaligned children retain
the original check. Car-before-cdr short circuit, dirty-bit writes, actual
marking, root coverage/order, policy and layouts are unchanged.

This delivers a bounded implementation, **not performance acceptance or a
trial App replacement**. G1/G2/G4 remain 3/7; G3/G5/G6/G7 remain open.
The [bound result](2026-10-31-gc-local-child-results.json) records source hashes,
all workload medians/variation, descriptive interval aggregates and raw-result
identities. The independent partner reviewed source, integration and protections
without rerunning products.

## Actual products and integration

Separate normal/checking terminal collector+observer products build in
736.848 seconds. Only `src/runtime_gc_collector.rs` differs from the P442
production input cohort; the 7,406 frozen inputs include unrelated working
changes and must not be described as a clean HEAD build. Native and test hooks
are disabled. The build receipt's historical pending flag is not rewritten.

Integration passes in 182.443 seconds: **530 new processes plus eight verified
prior fallback receipts**, 538 total. The 490 existing processes include 480
candidate and ten fresh reference processes. All raw results pass. Forty-eight
ordered registration comparisons, sixteen reference-language ID comparisons,
153 language observations, four fallback pairs and 32 phase-contract processes
pass. Unequal pre-latch heap sizes are not an equality predicate. Public graphs,
module errors, post-latch empty registries and subsequent deltas agree.

## Protections and descriptive measurement

Four extracted-source component tests pass in 0.628 seconds. They protect
allocated-prefix boundaries, partial-head limits, fallback checks, owner and
parent validation, short circuit, unchanged metadata and the exact child
lookup omission. The memory resolver, tags, dump predicate and owner are
explicit substitutes: these tests are not a VM, allocator or ABI proof.

Twenty-four serial prior/candidate observer processes run six fixed workloads
in interpreted/bytecode modes over three rotated rounds: 144 case executions,
1,512 cycles and 48 three-round workload groups, in 10.937 seconds. The host
fails the existing quiet policy. These are inclusive elapsed intervals and
short descriptive workload observations, with no official-reference speedup
comparison. Performance and full compatibility acceptance remain false.
Fewer lookups do not establish a reliable gain.

## Retained failures and cleanup

The first descriptive driver rejects the prior path before starting an editor:
its owning-repository boundary still points at the frozen tree. Rebinding that
boundary fixes the driver; the original failure remains. The first component
extractor omits `repr(C)`, producing one pass and three failures. Restoring the
attribute fixes extraction without changing production. Both original failures
and their outputs remain. Controller metrics are primary-side receipts,
explicitly not independently reviewed.

CLN171 records the consumed compiler objects/cargo, case state and generated
test binaries. Current/prior products, immutable inputs and raw observations
remain for the next affected integration and qualified comparison; retire them
once those consumers are replaced. Unique failed generated source remains to
explain its extraction failure. Three consumed partner notes have reported
sizes but no pre-delete content identities; that gap is disclosed rather than
reconstructed. See the [cleanup report](../cleanup/reports/2026-10-31-171-consumed-local-child-state.md).
