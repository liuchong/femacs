# ERT unlimited-backtrace diagnostic

One unchanged `ert-test-run-tests-batch-expensive` ID completes on the installed
31.1 reference through the existing exact-ID runner. This is an attribution
diagnostic. **Whole-file ERT, checking and full G3 remain unaccepted.**

## Source finding

The test disables backtrace line-length abbreviation. `backtrace--print-to-string`
then calls `cl-print-to-string-with-limit` with a nil limit, which binds
`print-length`, `print-level` and `cl-print-string-length` to nil. The test's
6/11 print settings therefore do not bound the backtrace arguments at this
layer; they still apply to subsequent condition/info formatting. Installed
`ert.el`, `backtrace.el` and `cl-print.el` match the three current source files
byte for byte. Compiled library inputs are identified separately.

Captured failure frames include callers and their arguments. In the existing
batch entry, outer ERT statistics retain the full selected test vector and
earlier results. Exact-ID execution reduces that outer selection/statistics
to one test, while keeping the original nested failure and assertions.
It changes the workload and process state: it cannot establish that the
all-ID, one-process file run is safe or equivalent. Runtime definitions
`ert-test-abc/def` are created inside an original test after selection; they
are not additional load-time IDs. No source, assertion, tag or runner changes
are made to obtain this diagnostic.

Sources: [test](../../test/lisp/emacs-lisp/ert-tests.el),
[ERT](../../lisp/emacs-lisp/ert.el), [backtrace](../../lisp/emacs-lisp/backtrace.el),
[printer](../../lisp/emacs-lisp/cl-print.el),
[existing runner](../../tools/fixtures/stable-suite-runner.el).

## Actual result and limits

- One expected pass, no skip or timeout; child exit 0 and complete.
- Body 0.799237 s; child 1.246188 s; driver 2.168325 s.
- Controlled owner: 2.816 s, exit 0, complete process cleanup;
  accounted tree peak **221,118,464 bytes**, about 211 MiB.
- The stricter per-run hard limit is **1 GiB**, with no growth. Owner deadline
  is 60 s; the existing child wait has a 45 s deadline. The uncertain
  512 MiB pre-run estimate is not a hard limit.
- `RUSAGE_CHILDREN` separately reports 203,522,048 bytes maximum child RSS;
  it is not the controller's tree peak.
- Reference program, dump, 3,316 Lisp resource identities and twelve input
  identities remain unchanged, with explicit before/after maps retained.
- No product build was requested. The reference does automatically generate
  one native message trampoline in the isolated environment; the raw budget's
  no-native-build wording does not mean no automatic compilation occurred.

Independent source and raw-result review accepts this diagnostic only.
It does not isolate a causal explanation for the old all-ID caps, prove a
runtime defect, measure a speedup, replace the trial App, or accept whole-file
compatibility. Previous capped results remain unchanged. Do not merge isolated
ID passes into a whole-file pass or repeat an unchanged capped matrix.
G1/G2/G4 remain **3/7**; the original Goal continues with remaining compatible
behavior and evidence-led runtime work.

[Receipt](2026-10-31-ert-backtrace-diagnostic-results.json).
[Consumed-state cleanup](../cleanup/reports/2026-10-31-173-consumed-ert-diagnostic-state.md).
