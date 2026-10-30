# Error-only remote notification decoder evidence

The intermittent `Invalid base64 data` failure is still open. This delivery
adds a reproducible diagnostic, not a production decoder fix or a compatibility
waiver. It runs the pinned stable 31.1 case
`file-notify-test09-watched-file-in-watched-dir-remote` from
`test/lisp/filenotify-tests.el`, using the existing process runner and isolated
environments. Neither the original test nor production Tramp/Base64 code changes.

## Capture and resource boundaries

`tools/fixtures/file-notify-remote-diagnostic.el` forwards the original decoder
arguments and returns its successful result. On error/quit it records the
original condition, re-signals it, and retains input bytes, buffer bounds/point,
process state and a backtrace. Unibyte inputs retain their exact bytes;
multibyte text is explicitly a UTF-8 representation, not original input bytes.
Process arguments containing percent signs are formatted literally. Normal
decoding is not replaced; diagnostic wrappers do change the stack and timing.
One successful run cannot disprove a race or establish an original root cause.

The driver validates labels, finite positive deadlines, bounded iteration count
(1 through 50), owned fresh output and actual source/product identities. Run
the whole driver under the existing runtime controller. Retained text has
explicit limits, but taking buffer substrings, hashing and scanning can still
copy or traverse complete inputs. This is not a constant-memory guarantee.
Case environments and receipts remain until separate identity-checked cleanup;
the driver performs no silent tree deletion. Captured data can contain user
text, command arguments or paths: original raw captures stay private in the
runner output, not in the portable report.

## Evidence and original failure preservation

The first combined run remains **failed**: all three capture verifications had
a fixture-registration load error, and the enclosing controller exited one.
Its separate real remote case passed in 128.874 seconds, with zero decoder
captures. That child result belongs to the original fixture hash recorded in
the [portable evidence](2026-10-30-remote-notification-diagnostic-results.json),
not the final fixture. The independent reviewer corrected an earlier mistaken
aggregate-success/exit-zero report after checking the original records.

The registration guard was repaired. Subsequent focused capture verification
passed; primary source review then fixed percent-sign process arguments and
added complete driver/runner/manifest/fixture source bindings. The final run
passes four explicit checks across three synthetic cases: byte 255 and caller
state preservation; Base64url/multibyte distinction; unchanged success and
no-error behavior without fabricated captures; and retained forensic state.
The region case intentionally retains the original failed ERT record while
the enclosing verification succeeds by checking that exact error and evidence.
No deliberate process crash or system crash-dialog test is involved.

Final helper time is 1.156 seconds; enclosing controller time is 1.290 seconds,
exit zero and fully reaped, peak accounted 39,190,528 bytes. This short final
verification does not rerun or relabel the earlier 128-second remote case.
Both source hashes and original result hashes are bound separately; product
and resource identities match before/after. Independent paired review read and
adopted the final changes. The historical intermittent failure has no captured
bad input yet and remains a G3 compatibility gap under the unchanged Goal.


## Final fixture on the actual remote case

A separate single run now exercises the final six source bindings against the
unchanged trial Cocoa program, dump and resources. The pinned original case
passes: ERT 126.201362 seconds, child process 126.560347 seconds, full driver
127.776397 seconds. The enclosing runtime controller exits zero and is fully
reaped in 128.038 seconds, peak accounted memory 93,634,560 bytes. No bad Base64
input was captured. This closes the final-version real-run evidence gap only;
it does not fix or disprove the intermittent failure or close G3.

The first launch attempt specified a nonexistent dump location. Preflight
rejected it before creating the output directory or starting the editor; the
controller exited one and was fully reaped. The corrected run used the actual
resource dump. Both outcomes are preserved, without counting the rejection as
an executed test. No unchanged remote case is repeatedly rerun.

[CLN158](../cleanup/reports/2026-10-30-158-consumed-final-remote-diagnostic-state.md)
records the consumed isolated environment and materialized test copy. Original
results, process logs and source/product bindings remain unchanged. The earlier
retention checks describe their original verification-time state; this separate
cleanup does not rewrite those receipts.

Independent read-only review confirmed the final six-source/product/test bindings,
four retained process/result hashes, original failed aggregate, preflight scope
and all 17 cleanup-ledger rows. No archive defect was found; no editor/matrix was
rerun. This review accepts the bounded evidence, not a fix or full compatibility.
