# Young cadence: safe Automatic minors under retention caps

Technical design for the G7 nonmoving young-cons collector. This document
binds call sites, roots/stores/barriers and the cadence rule that makes a real
`GcKind::Young` reachable without weakening the existing retention caps or
explicit-full semantics.

## Problem (verified)

On the formal observer/collector product with `FEMACS_GC_COLLECTOR=young` and
`FEMACS_GC_OBSERVATION=all`, a real allocation load completed **19 majors /
0 minors**. Reasons were startup `1` once and allocation/retention cap `5`
eighteen times. The major-layout control stayed all-major as designed.

Facts that bound the design:

| Fact | Source |
| --- | --- |
| `CONS_BLOCK_SIZE = 1998` | `alloc.rs` generational `cons_block` |
| After a major, live cons ≈ 3875 so `cons_cap ≈ totals[6] + 1998 ≈ 5873` | `choose_cycle` / `commit_cycle` |
| Between ordinary heap-threshold GCs the mutator allocates ≈ 84600 conses | observer-1 young runs |
| `gc-cons-threshold` 5000 / 20000 / 50000 does not change that rhythm | same runs; heap percentage dominates |
| Cap left/right units and epochs already match | `totals[15]`/`[16]` since last major vs `totals[6]+CONS_BLOCK_SIZE` and `LAST_MAJOR_BYTES.max(memory_full_cons_threshold)` |

Rejected approaches (must not be retried):

1. Arm `consing_until_gc = -1` when allocation reaches the **retention** cap, relying on existing `maybe_gc`. `maybe_garbage_collect` first runs `bump_consing_until_gc`, which can restore a positive threshold and skip the cycle; when a cycle does run it is already reason 5. Result: ~250 fulls, 0 minors.
2. In `maybe_garbage_collect`, treat “at retention cap” as `young_due` and skip bump. Arming at/over the cap still makes `choose_cycle` select Full reason 5. Result: ~253 fulls, 0 minors.

Both changed Automatic cadence without ever becoming Young. The retention caps
remain necessary remembered-set bounds; they are not the young **trigger**.

## Goal

Make Automatic collection request a cycle **while still under** the retention
caps, so `choose_cycle` can return `GcKind::Young` (reason 0), then keep the
existing three-minor and retention Full fallbacks. Do not:

- rename counters or status slots
- enlarge / disable retention caps
- weaken required Full fallbacks
- change default products, major-layout control, or explicit `garbage-collect`
- call GC from allocation primitives (`Fcons`, `tally_consing`, barriers)

## Safety boundary

| Layer | Rule |
| --- | --- |
| Allocation primitives | May only update counters and a non-signalling cadence pending flag. No `garbage_collect_impl`, no `consing_until_gc` mutation for young policy. |
| Barriers / stores | Unchanged: `XSETCAR`/`XSETCDR` → `femacs_gc_cons_written`; `XCONS`/`xcar_addr`/`xcdr_addr` → `femacs_gc_cons_escaped`; mark setters → `note_marked_noncons`; free/sweep age clears unchanged. |
| Roots | Young still premarks old conses, traces dirty/raw-tainted old children, freezes and marks the noncons registry prefix, marks finalizer lists, then the ordinary root set / weak fixpoint / sweep. |
| Mode selection | `choose_cycle` retention tests stay strict `>`; reason codes 0–12 unchanged. Cadence never overrides reason 2 (explicit Full), 3 (major control), 4 (three-minor), 5 (retention), 6–12. |
| Safepoints | Only `maybe_gc` → `maybe_garbage_collect` (and the Automatic branch it already owns) may start a cadence-driven cycle. |

## Cadence rule

Young growth is measured with watermarks that are independent of the retention
counters' **meaning**, but share the same physical totals:

- `totals[15]` / `totals[16]` remain “allocated cons/bytes since last **major**”.
- `cadence_cons_base` / `cadence_bytes_base` record those totals at the last
  completed Automatic or Full cycle that the cadence policy may use as a
  growth origin (reset to 0 on Full commit together with the totals; advanced
  to the current totals on Young commit).

Lead time (提前量) keeps a cycle Young-eligible even if the mutator allocates
between arming and `choose_cycle`:

- cons lead = `CONS_BLOCK_SIZE`
- bytes lead = `memory_full_cons_threshold` (= `sizeof(cons_block)`, 32752 on
  the generational layout)

A cadence step carves three young windows under the room that remains after
the lead:

```text
cons_cap  = totals[6] + CONS_BLOCK_SIZE          # Option, overflow → not due
bytes_cap = LAST_MAJOR_BYTES.max(memory_full_cons_threshold)

cons_step  = max(1, (cons_cap  - cons_lead)  / 3)
bytes_step = max(1, (bytes_cap - bytes_lead) / 3)

room_ok =
    totals[15] + cons_lead  <= cons_cap
 && totals[16] + bytes_lead <= bytes_cap

growth_due =
    totals[15] - cadence_cons_base  >= cons_step
 || totals[16] - cadence_bytes_base >= bytes_step
```

`young_cadence_due` is true only when all of the following hold:

1. Collector mode is young, owner ready, current thread owns the runtime.
2. `garbage_collection_inhibited == 0` and `gc_in_progress` is false. Cadence
   bypasses a positive `consing_until_gc` (including the `HI_THRESHOLD` latch
   that `inhibit_garbage_collection` installs), so these guards must fail the
   pending predicate itself — not rely on `garbage_collect_impl`'s later early
   return after it has already asserted an empty weak-table worklist.
3. The same non-retention predicates that `choose_cycle` uses for an Automatic
   request would not already force Full (no pending doomed finalizers, no
   foreign/unknown producer, registry valid, counters finite, not
   `Vmemory_full` / `Vpurify_flag`, consecutive minors `< 3`).
4. `room_ok` (still under retention **with** lead).
5. `growth_due`.

When due, `note_allocation` sets a process-local `CADENCE_PENDING` flag. It
never starts GC. `maybe_gc` and `maybe_garbage_collect` re-check inhibit /
`gc_in_progress` before entering `garbage_collect_impl` on the cadence path.

## Call graph

```text
Fcons / tally_consing
  └─ gc_collector::note_allocation
       └─ maybe set CADENCE_PENDING          // no GC

eval / bytecode / keyboard safepoints
  └─ maybe_gc                                // lisp.rs
       ├─ consing_until_gc < 0
       └─ or femacs_gc_young_cadence_pending()
            └─ maybe_garbage_collect         // alloc.rs
                 ├─ if young_cadence_pending → garbage_collect_impl(Automatic)
                 │     (do not bump first; heap threshold state untouched)
                 └─ else bump_consing_until_gc < 0 → Automatic as today

garbage_collect / Fgarbage_collect
  └─ garbage_collect_impl(Full)              // unchanged

garbage_collect_impl(Automatic|Full)
  ├─ choose_cycle                            // caps / reasons unchanged
  ├─ prepare_cycle / roots / barriers / sweep
  └─ commit_cycle
       ├─ Full: reset totals[15]/[16], watermarks, consecutive minors
       └─ Young: advance watermarks to current totals[15]/[16]
```

`femacs_gc_young_cadence_pending` revalidates `young_cadence_due` so a stale
flag after a race past the lead, an inhibit window, or an in-progress cycle
cannot force a pointless entry; if the mutator did cross the retention cap
before the safepoint, `choose_cycle` still returns Full reason 5 (safe
fallback, not a cadence success).

Bytecode safepoints that matter for cadence:

| Site | Behavior |
| --- | --- |
| `do_call` (`Bcall`..`Bcall5`) | Fast path must treat `femacs_gc_young_cadence_pending()` like `consing_until_gc < 0` and enter `bc_call_housekeeping_slow` → `maybe_gc`. |
| `Bcall6` / `Bcall7` | Already always call `bc_call_housekeeping_slow`; no extra predicate. |
| Interpreter loop `bc_periodic_gc_and_quit` | Already calls `maybe_gc`; inherits the lisp.rs cadence check. |

No other `consing_until_gc < 0` fast-path skip exists in this candidate's
bytecode/eval/keyboard call sites.

## Why this is not the rejected design

| Rejected | This design |
| --- | --- |
| Trigger predicate = retention cap | Trigger = young growth step **below** cap with explicit lead |
| Often arms when `choose_cycle` must return 5 | Arms only while `choose_cycle` can still return 0 |
| Mutated `consing_until_gc` or skipped bump at cap | Pending flag + safepoint; normal bump path unchanged when not pending |
| Counted “more Fulls” as progress | Success is completed minors (reason 0) under unchanged caps |

## Fixture

`tools/fixtures/gc-young-cadence.el` drives Automatic allocation without
calling `garbage-collect` to invent a minor. Young mode uses a **byte-compiled**
call/allocation ring (`femacs-gc-young-cadence--byte-worker` calling
`femacs-gc-young-cadence--inc`) so `do_call` housekeeping is required. Major
control uses a finite absolute `gc-cons-threshold` with `gc-cons-percentage`
0.0 so ordinary Automatic Full (reason 3) is not left to
`most-positive-fixnum` or incidental harness allocation. Explicit
`garbage-collect` remains Full reason 2. Caps are not enlarged; a reason-5
Full is never accepted as Young success.

## Acceptance (frozen-batch reachability / correctness)

The 2026-11-18 frozen Root verified **young reachability and directed
correctness** for this candidate. Evidence index:
`build/acceptance/frozen-batch-20261118-index.json` (6238 bytes,
sha256 `49b33748de04f32e157d669d0d130267f63a64c6a497c3a34cad25f88afcf911`).
Product identity in that index: cocoa collector+observer
`build/runtime/frozen-2/port/src/emacs` /
`build/runtime/frozen-2/port/src/emacs.pdmp`. This does **not** close G7 or the
seven-part Goal, and it is not a performance comparison against G5/G6.

| Check | Result |
| --- | --- |
| Fixture `gyc01` / `gyc02` | 2/2 passed (`build/acceptance/g7-frozen2/g7-run.log`) |
| Young Automatic (byte-compiled call ring) | `minors_delta=8`, `reason0_delta=8`; two Fulls at the three-minor boundary (`reason4_delta=2`); last kind Young |
| Young explicit `garbage-collect` | `minors_delta=0`, `majors_delta=1`, kind Full, `last_reason=2` |
| Major-layout Automatic | `minors_delta=0`, `majors_delta=20`, `reason3_delta=20`, last kind Full |
| Major-layout explicit Full | kind Full, `last_reason=2` |
| `test/src/alloc-tests.el` | 4/4 |
| `tools/fixtures/bytecode-runtime.el` (LC_ALL=C) | 10/10; covers live-root minor-sweep child tracing (tvm5), forced-GC deep-call stress (tvm6), non-local exit / handler restoration |

### Still open

- G5/G6/G7 **performance** attribution, quiet-host timing, and any speedup /
  pause tradeoff claims.
- Broader collector stress beyond this directed batch (retention races under
  unknown producers, full native+young matrix, production default enablement).
- Whole G7 / Goal closure. Commits that land this candidate (including
  `e3454ea7` and the follow-up safepoint/inhibit patch) record verified
  reachability source, not Goal completion.
