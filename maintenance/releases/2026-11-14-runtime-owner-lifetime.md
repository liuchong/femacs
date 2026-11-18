# Runtime owner lifetime (2026-11-14)

## Current follow-up

The current runner also records child birth identities while a command runs and
keeps ownership when children create new sessions or become reparented. Cancel,
deadline and unwind collect those observed descendants; PID reuse never grants
signal authority. Unknown identity/signal errors prevent a complete cleanup
claim. Group signalling on Darwin reaps an exited owned child before retrying an
EPERM result; live-child and persistent permission errors remain failures.

Real functional integration8/8 passed in2.285739s, including two parent-loss
flows with separate child sessions and a separate-session hard-deadline flow.
Existing ordinary exit1, metrics/exit timing and before-spawn cancellation remain
covered. Every fixture was retired, no later work started, and the controller
exited0 with cleanup complete (peak84197376B). Functional SHA256
`bac9f58b3671ad2c96e7cc2de91010d1a1dc583652f149a560d07ddb776622ec`.
Two earlier failed attempts remain failures: the first retained only the outer
assertion; the second preserved the child EPERM result. They are not successful
validation evidence.

The occupancy reader now treats a pipe-read error as unknown and blocks
deletion. Actual empty/held-file queries and an injected EIO on a real query
pipe passed3/3 in the existing retirement integration (1.273769s total), alongside
ordinary and parent-inclusive retirement and failed-producer preservation.
The EIO case retained original data, created no retirement ledger, reaped the
query child and retired state only after a new clear query. A real held-file
query returned1 with267 stdout bytes and was correctly refused.

Native preparation now captures the same current authored Python harness used
by semantic/performance actions. The actual32-profile run passed19/19 units each
(608 total), with products, fixtures, drivers and captured harness unchanged.
Its controller exited0 with cleanup complete in584.566s, peak3442625992B. Source
and product provenance stayed on the completed cohort; no VM was recompiled.

The consolidated related regressions passed measurement7/7, phase dispatch5/5
and stable dispatch8/8. Post-functional protection passed ownership4/4,
retirement16/16 and measurement4/4. Ownership protection binds the functional
SHA above; retirement protection binds functional SHA
`cfddc55e1fffed1d070eb78b85681b56ab2c14b2b8b21812a5cd3e8dec6efbfe`.
The actual preparation capture and ownership protection name the same current
execution-source SHA. A protection fixture initially omitted `timed_out` in four
stub observations; those four errors remain failed records. After correcting
that fixture, only retirement protection and the not-yet-run measurement
protection ran; the four unaffected passing groups were reused.

Protection result SHA256 values: ownership
`c5c457f932fe6a8cc3a49f684e1e331f6962ce2159224cd20f1b045f7cd56123`,
retirement `9228c66585d1a3edab3a2cef9ae5b78268a2310be6540e0b219f34ae7b75a5cc`,
measurement `87b35e2dd583a5b38ad8bf5dd9cde9702f192ebedd88fd17b6dbf4143af49c35`.
The first mixed batch exited1 because of those fixture errors; its individual
passing rows remain valid, not a passing whole-batch claim. The repair batch
exited0/cleanup complete,1.550s, peak40354056B. This closes the Python helper
follow-up, without accepting original555/48 corpus, VM performance or the Goal.

## Earlier limited verification

An interrupted supervisor previously left its coordinator adopted by PID1 while
compilation continued. The existing process helper now binds the importing
coordinator PID and its immediate launching parent. It checks that relationship
before spawn, during waits/measurement and before accepting completion. A lost
parent cancels the owned child, records incomplete/cancelled owner loss, unwinds
the artifact owner and exits125 instead of starting later work. An inherited
helper in a fork cannot silently assume the original coordinator's authority.

The child group receives TERM with a thirty-second cleanup opportunity, then any
remaining owned group is killed/reaped. Existing timeout and explicit-cancel
behavior is retained. No external holder is signalled. This adds lifetime
cancellation inside the existing runner; the resource controller remains the
sole memory authority. The thirty-second grace covers the previously observed
19.391s owner-release operation; a five-second grace could interrupt that
legitimate unwind. The helper also rechecks after sampler preparation,
immediately before spawn. The final affected integration/protection both passed
again after these changes; earlier small tests are not used as final-source
proof. Forced termination still cannot guarantee a nested unresponsive process
released its own guards; retain its failed journal for the existing explicit
dead-owner release rather than claiming that release succeeded.

Immediate-parent identity is the boundary: this does not
prove detection of a detached controller while that parent remains alive.

## Actual verification

- Functional5/5 passed in0.535650s: parent loss in ordinary waits, loss while
  sampling with actual exit timing, loss before spawn, a live-parent exit1
  control and the original process deadline. Both launched workloads and their
  grandchildren exited; actual immutable inputs were released by ArtifactRun,
  the journals ended released without errors, and later work did not start.
- Post-functional protection2/2 passed in30.228062s: a TERM-ignoring workload
  was killed after the bounded grace period, and a real fork was rejected
  before spawning any work. The protection binds the functional receipt SHA.
- Controllers exited0 with cleanup complete: functional0.792s with recorded
  peak33,161,216bytes; protection30.329s with recorded peak62,586,880bytes.
- No compiler/editor/original corpus ran. These are Python process/real file
  integration and protection results, not G3 compatibility or G5/G6/G7 data.
  Original interrupted runs remain interrupted and unaccepted.

Functional SHA256: `0b28d78089d876872979804753cc89694f26a93f8e219882f78d30a58201f8a1`.
Protection SHA256: `a27971d133edc437173d58ab7a39ea262d3015ebf4bfa31811104933ab79f879`.

```sh
python3 -B tools/test-runtime-process-owner.py \
  --out "$FEMACS_OWNED_CACHE/owner-functional"
python3 -B tools/test-runtime-process-owner.py --mode protection \
  --functional "$FEMACS_OWNED_CACHE/owner-functional/results.json" \
  --out "$FEMACS_OWNED_CACHE/owner-protection"
```

All temporary fixtures are removed by the driver; consumed result directories
are cleaned after this compact record. The production build and builder cache
remain absent. Remaining full compatibility/performance/workflow, installation
and final review requirements are unchanged; no replacement matrix is built.
