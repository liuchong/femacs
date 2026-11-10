# Complete runtime measurement contracts (P459)

The performance phase now schedules both primary measurements and the explicit
permanent-Full fallback control in separate owned output roots. Both use the
canonical performance action, strict input/completion mode binding and all61
product checks. An internally valid receipt for the wrong mode is rejected.

Startup measurements use an owned exit waiter and monotonic spawn-to-reaped-exit
lifetime, while total process duration remains separate. The parent continues
memory/deadline monitoring and reaps its process group and waiter on completion
or failure. This measures process lifetime; it is neither readiness-marker
latency nor a claim about cold disk startup. Missing positive memory samples
or incomplete exit timing fail the measurement.

Native sampler bindings are prepared before measured spawn and retained without
caching process identities or memory values. Post-read identity verification
uses a fresh process object because psutil caches creation time on one object.
Reused identities fail; a retired tree produces no sample. The shared external
resource controller and its limits are unchanged.

Successful startup rounds enter the existing seven-round statistics and
same-round reference/major-young comparisons under their distinct startup
mode. Pause median/MAD and observed physical-footprint/accounted maxima are
reported; absent observations remain null. Body compilation/setup boundaries,
quiet-host requirements and minimum tail sample counts are preserved.

The bounded report reader admits256MiB,32,768 sample rows and4,096 cases,
including the current primary shape. Strict types, fallback scope, safe output
and false acceptance flags remain enforced.

## Actual tool acceptance

| Scope | Result |
| --- | --- |
| Phase/receipt integration | 16 real Python child calls; both performance modes, mode mismatch and invalid stable corpus completion handled correctly |
| Timing/lifecycle integration | Three real finite Python children: normal exit, nonzero exit and timeout; positive memory observations and reaped waiter |
| Measurement mechanism | Explicit synthetic editor executable,108 primary and54 fallback Python children through the real orchestration/parser/sampler/statistics paths |
| Full-shape report CLI | Two real reader subprocesses;79,036,726-byte primary and25,609,707-byte fallback synthetic inputs |
| Primary shape | 13,069 measured samples,1,867 summary cases,1,024 comparisons |
| Fallback shape | 4,116 measured samples,588 summary cases,768 comparisons; no repeated startup/native/observer sweep |
| Subsequent protections | 15 phase,4 sampler/lifecycle,37 accounting,14 report and16 stable-scope cases:86 passed |

The final measurement integration completes in47.463s; its controller exits0,
fully reaps the tree in47.857s and observes345,145,344 peak accounted bytes,
below the4GiB initial and16GiB hard limits. Other accepted tool runs also exit0
with complete controller cleanup. Large synthetic input trees, mock programs
and owned integration fixtures are retired; small results remain for review.

Full-shape values and mock editor responses are synthetic. They prove file,
process and measurement-tool contracts, not actual editor native compilation,
GC behavior, quiet-host performance or an adopted speedup.

## Preserved failures and remaining work

Two short40ms fixture runs ended with no positive memory sample. Their failed
results are retained; the first did not preserve enough per-process detail to
prove its exact cause. The final fixture provides150ms observable child
lifetimes and a250ms timeout window. Production sampling gates remain strict;
this does not turn missing observations into valid zeroes or change editor
workload timings. Earlier passing runs remain scoped to their source versions.

The first protection batch remains failed because a newly added synthetic
startup report retained stale load/native inconclusive labels. Only that
fixture metadata was corrected; the affected14 report protections then pass.
No original failure, test corpus or historical execution receipt is rewritten.

Independent source and scoped-result review accepts P459, with zero confirmed
defects. Extremely short real startups may still fail strict positive-memory
sampling; they must remain failed rather than use invented zeroes. The next
frozen matrix must include P458 and
these current tools. Actual full-source semantic/stable/performance execution,
G5/G6/G7 adoption decisions and safe installed-App replacement remain open.
Goal remains active3/7.
