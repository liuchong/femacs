# Current-core remote09 diagnostic

The original intermittent `Invalid base64 data` failure remains open. Its failed
input and traceback were not captured. This run checks the unchanged pinned
remote09 case once per installed reference, normal candidate and checking
candidate after the accepted process-wait and Base64 bounds fixes. It uses the
same existing error-only signal observer on all three; it is an isolated
instrumented diagnostic, not original full-file acceptance or a repair claim.

| Product | Result | Seconds | Captured errors | Actual mock environment adopted |
| --- | --- | ---: | ---: | --- |
| Installed reference | Passed | 126.892866125 | 0 | Yes |
| Normal candidate | Passed | 126.524799875 | 0 | Yes |
| Checking candidate | Passed | 126.785657917 | 0 | Yes |

The original test body, runner, observer and all three program/dump/resource
identities remain unchanged. Three serial editors use a local mock shell, not a
real remote host or user data. Original assertions, tags and limits are unchanged.
The source owner finished in 383.540742625 seconds; the resource controller
exited 0 with complete cleanup in 385.136 seconds and 141,869,056 peak accounted
bytes. Per-case and whole-run deadlines were 300 and 1,050 seconds.

The first preflight stopped before creating a test output directory or launching
an editor. An old embedded product identity covered program/dump/Lisp but omitted
runtime libraries already present in accepted product receipts. The corrected
preflight retains all old field checks and additionally binds full current
resources to those unchanged successful receipts; no source/product was changed
and the failed preflight is preserved.

Source reading identifies one conditional path from an append write to a remote
copy and a local decoder. The original ten-iteration loop alternates five
appends to the watched existing file with five non-append writes to fresh names.
Local-host and out-of-band paths may bypass that decoder chain. This does not
prove the actual decoder used at the old failure, a unique path, or a cause.
No new payload was captured here, so another identical diagnostic is not useful.

[Bounded results](2026-10-30-current-core-remote09-results.json) bind these readings
to the retained raw result hash, original test/observer hashes and accepted core
commit. Original failure, previous nonreproductions and all process evidence
remain. [CLN152](../cleanup/reports/2026-10-30-152-current-remote09.md) records
retirement of only consumed copied inputs, contexts and isolated environments.
This does not complete G3, establish 100% compatibility or prove the old
intermittent failure fixed. The next delivery is a current macOS GUI build
containing the accepted core fixes; the withdrawn owner-batch candidate remains
disabled.
