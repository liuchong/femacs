# Real workloads after permanent collector fallback

P439 adds an explicit `--permanent-full-fallback` performance input to
`tools/runtime-execution.py`. The wrapper first exercises a real failed
`module-load` of an absent owned file, verifies the original public error, then
loads the unchanged workload fixture. It does not change the production
collector, workload algorithms, default measurement mode or acceptance policy.

## Actual integration and protection

The retained installed stable reference and prior/current normal-release
major/young products form five profiles. Interpreted and bytecode modes use
the actual observed function representation, observer off, two warmups and
seven measured rounds. Startup and native modes are outside this explicit
post-startup condition; their ordinary measurement path is unchanged.

All 90 workload processes completed, yielding 420 measured samples in 60 groups
and 24 direct prior/current comparisons. Every candidate context verifies the
unknown-producer flag, invalid registry and unchanged minor-collection count.
Original module-open errors and correctness observations are retained. Old
young-mode tables still contain about 100k entries after the workload; current
young tables contain zero. Full tracing, not that unusable index, retains the
live graph. This corroborates P437's work removal, not an elapsed-time gain.

After integration, five protective processes passed: missing owned root and an
existing reserved input each produce the intended ordinary Lisp error on the
reference and current young product, without measured workload records or
overwriting the reserved file; an invalid non-performance action rejects the
flag before creating output. No intentional crash/abort test was run. The first
protection attempt incorrectly required empty stdout even though batch error
backtraces use stdout. That false aggregate remains false; the intermediate
marker check was also corrected before final acceptance.

The [portable evidence](2026-10-30-permanent-fallback-measurement-results.json)
binds source/product/resource identities, original result hashes, summaries,
controller exits and all-round retention. The full original samples and case
receipts remain in their runner output. Independent paired source/result review
confirmed the final three source hashes and all five protections.

## Performance conclusion and cleanup

All 60 groups exceed the unchanged quiet-host criterion; all 24 direct pairs
are ineligible. The diagnostic medians and intervals include both favorable
and unfavorable differences. Collection success is true; qualified speedup and
performance acceptance are false. Short pause samples keep unavailable
quantiles null. No gain, whole-language compatibility, default promotion or
G5/G7 completion is inferred. The trial App is unchanged; G1/G2/G4 remain the
three accepted outcomes, with G3/G5/G6/G7 still open.

The actual workload helper took 61.485 seconds; its enclosing controller took
75.894 seconds, exited zero and fully reaped its tree. Final protections took
4.601 controller seconds, also exit zero/reaped. These are operation durations.
[CLN156](../cleanup/reports/2026-10-30-156-consumed-permanent-fallback-workloads.md)
retired only 540 empty case directories, preserving products, inputs, raw
results and all measured rounds; it did not recover regular-file bytes.
