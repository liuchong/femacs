# Known cons locations: bounded sweep bookkeeping delivery

P441 removes redundant heap-cell tree lookups in the two partial-word sweep
hooks. The caller already holds the allocated cons block and cell index; the
block remains registered and alive until the scan ends. Pass that location to
`survivor` and `swept_dead`, preserving the owner gate, counter order, old/dirty
bits, cell contents and free-list order. The all-marked-word path is unchanged.

`note_cons_marked`, allocation/free validation, written/escaped paths, remembered
roots, marking, scheduling, layouts and defaults are unchanged. The reviewer
rejected extending the shortcut to `note_cons_marked`: ordinary builds do not
perform the preceding allocation/live check. That proposal was not implemented.

## Actual validation

Separate normal/checking terminal products were built from 7,355 frozen inputs,
including existing WIP. This is not a clean-HEAD build. The only production input
differences from retained P440 products are `src/alloc.rs` and
`src/runtime_gc_collector.rs`. Four candidate profiles use collector+observer,
major/young modes, native disabled and no test hooks. The reference retains its
actual installed feature set; it is not presented as matched native parity.

After complete implementation, affected real language/observer/collector
integration passed: 480 candidate and ten fresh reference process receipts,
eight new fallback receipts and eight verified reused prior fallback receipts.
Total: 498 new + 8 reused = 506. All 48 ordered registration mappings, 16 reference
language ID comparisons, 153 language observations, 17 shared and 17 language
registrations, and four fallback pairs passed. Original raw aggregation is true
and remains untouched. Source/resource identities and trial App are unchanged.

The first unexecuted integration driver looked up reference IDs by each candidate
observer mode even though reference only provides `off`. This was corrected and
reviewed before execution; all 16 comparisons use that verified reference list.
It was not a product failure or a waived test.

Only after integration passed, four protective tests passed using the actual
source hooks, bitmap helper, counter method and cons-block body. They cover
owner-disabled behavior, promotion once, dead/reused cells, counter saturation,
and word/last-cell boundaries 0/63/64/1997. Object fields, owner/state and primitive
values are explicit substitutes: these are isolated component tests, not VM,
ABI, memory-tree, `note_cons_marked` or sweep-cursor acceptance. The caller's
location precondition is supported by source review plus real integration.

Cursor independently checked the frozen production change, original integration
records and protection assertions without rerunning products or editing source.
No confirmed defect was found; its review limits remain disclosed above.

## Limits and retained evidence

Build driver: 716.020 seconds; integration: 179.599 seconds; protection driver:
0.545 seconds. Controller peaks respectively 582,467,584, 197,459,968 and
96,894,976 bytes; all three exited zero and fully reaped their trees.

No new performance matrix was run: observed load/core 0.25928 exceeds the existing
0.10 quiet-host limit. Fewer lookups alone do not prove throughput, pause or memory
improvement. P441 does not explain the pre-static-root interval or the previously
unequal live-object counts; missed marking is not excluded by those aggregates.
Full compatibility, qualified performance and G3/G5/G6/G7 remain open. G1/G2/G4
remain the three completed modernization items. The trial App is not replaced.

[Portable results](2026-10-30-known-cons-locations-results.json) bind original
results, source and driver hashes. [CLN159](../cleanup/reports/2026-10-30-159-consumed-known-cons-state.md)
records consumed-state retirement and recovery boundaries. Original process
results and observations, current/prior products and frozen inputs are retained.
