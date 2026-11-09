# Darwin mock shell output contaminates strict Base64 transfer

The original autorevert file fails intermittently at different remote case IDs.
The installed C reference also fails, so the observations do not establish a
candidate-only decoder regression. A bounded, error-only observer captured the
actual73-byte region input: a shell child-setpgid permission diagnostic followed
by the valid13-byte Base64 line for `any text`. The diagnostic is already in
Tramp's transfer buffer when file-local-copy invokes base64-decode-region.

On the same captured dirty input, the unchanged C reference and Rust candidate
both reject string and region decoding with the original exact error; the region
buffer is unchanged. Both APIs on both products decode the clean captured tail
to the exact eight expected bytes. Input/product identities remain unchanged;
the native controller exits0 with complete cleanup in7.504seconds. This proves
only these specific differential observations, not complete decoder correctness.
No decoder, production Tramp or original test assertion is changed.

## Source-bound fixture repair

The original ert-x mock method starts an interactive sh; the local implementation
has monitor/job-control support. Tramp re-execs the remote shell and explicitly
sets ENV from its remote-process-environment, so exporting local ENV alone is
insufficient. Independent source discussion confirms this shadowing and the
bounded component/whole-suite distinction.

The corpus driver creates a case-owned startup file containing only a conditional
`set +m`. The local process uses that file as ENV; the Lisp bridge sets the same
file in Tramp's explicit remote environment using shell quoting. This also handles
paths containing spaces rather than inventing a path restriction. The change is
limited to the two original Darwin file-notification/autorevert mock fixture IDs
and is identical on reference and candidates. It does not edit a user shell
configuration, redirect/filter stderr, loosen Base64 decoding or relabel skips.
The bridge records the actual startup-file content hash and local/remote ENV
settings in each raw environment record. General remote-server behavior remains
outside this test-initialization fix.

## Evidence and remaining work

The unmodified P421 batch has155 cases:109 passed,42 skipped,4 Base64 failures,
including two on the official reference. Its products and inputs remain unchanged;
the runtime controller exits1/complete cleanup in356.967seconds. The captured
paired diagnostic is separate and does not replace those failures.

Evidence: `build/acceptance/runtime-p422-base64-diagnostic-3`,
`build/acceptance/runtime-p422-payload-differential-1` and the fresh repaired
integration `build/acceptance/runtime-p421-p422-local-functional-1`.
The repaired155-case integration completes110 passes/45 original skips/zero
failures with the same original inputs, FD256,45/120-second case deadlines and
900-second whole-group deadline. All five autorevert case-status maps match.
Every raw environment record binds identical initialization content and explicit
local/remote ENV. Product/input/harness identities remain unchanged; the native
controller completes cleanup in358.926seconds. Its exit1 and raw passed:false
remain because the strict bridge does not count skipped cases as passes; this is
not a full-corpus acceptance or a waiver of the45 skipped outcomes.

Post-integration protection passes the existing isolation case and two new
scope/permission and content/symlink-tampering guards. Real C/Rust Tramp mock
connections, with owned paths containing spaces and an apostrophe, both observe
initial flags hiBH, a deliberate monitor-on negative control himBH, and restored
flags hiBH. Thus initialization is actually consumed, beyond metadata alone.
The controller exits0 with complete cleanup in8.94seconds and product identities
unchanged. Earlier failures remain; final independent adoption is pending.
Full remote notification, full stable corpus, UI and Goal completion are unproven.
