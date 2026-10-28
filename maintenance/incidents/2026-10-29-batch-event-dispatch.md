# Conditional batch event dispatch

The original autorevert file on the earlier terminal release exits at closed
stdin before its first case completes. The keyboard reader entered a direct
batch getchar branch that the original source excludes when DBus, file
notification or threads are enabled. The current products enable threads.

P417 restores that negative compile condition. The legacy branch and EOF
behavior remain available when all three features are absent; timed input,
event ordering, original tests and timer semantics are unchanged.

## Actual verification

Four fresh terminal/Cocoa normal/checking products build; both Cocoa package
signatures verify. The reference and four products complete all original
keyboard25 and native-thread38 cases: 315 passing cases. Five closed-stdin
probes run a timer and return from timed input. The standalone regression in
`tools/test-batch-events.py` accepts those five products and correctly rejects
the old product with exit1: six expected outcomes, unchanged program/dump bytes.

The integration report remains false. Autorevert's first local case emits a
pass, but the next remote case exits255 with an invalid file-notify special
event; later cases are not-run. The body pass in that failed process is not
accepted as a completed process result. Original timer/Lisp-thread skips and
one reference autorevert failure are preserved. P418 addresses the separate
notification-backend integration; this commit does not claim autorevert, the
complete stable corpus, GUI interaction or the entire Goal has passed.

Evidence: `build/acceptance/runtime-p417-batch-event-products-1/results.json`
and `build/acceptance/runtime-p417-batch-events-regression-1/results.json`.
The integration controller exits1 after1252.652s with complete owned-process
cleanup; regression exits0 with complete cleanup. Independent source review
accepts the bounded conditional repair and this explicit acceptance boundary.
