# Align the stable ERT library and its original tests

The required stable self-test expects alist descriptions. The prior development
ERT library instead exposes a newer record representation for its result UI.
This is a shipped-library version mismatch, not evidence that the VM assq
primitive is wrong. The new record getters have no independent production
consumer outside ERT; the matching development tests also use them.

## Component decision

Use the complete stable31.1 ERT library and its matching original root test,
verbatim from pinned commit a360712c9d272d950d8d8255ef74570f7e90b7d9. The
required stable acceptance corpus and assertions remain unchanged. No VM change,
record conversion shim or hand-edited Lisp is introduced. The development-only
diff UI and its specific test are retired together, with the old Git commit,
blob identities, full hashes and pair-restoration boundaries recorded in CLN114.
Independent discussion recommends this coherent component alignment; retaining
the future UI would require a separately specified feature and paired tests.

## Actual current evidence and limits

The fourth P418/P419 integration freshly builds terminal/Cocoa normal/checking.
Both Cocoa signatures and all five closed-stdin timer probes pass. The unchanged
stable ERT73, debugger49 and C-parser2 files pass on the installed reference and
all four products: 620 completed passing cases. In particular,
ert-test-list-of-erts-file-tests now passes on all five, instead of encountering
the earlier alist/record mismatch. The original test text is unchanged.

Evidence is `build/acceptance/runtime-p418-p419-stable-products-4/results.json`.
At this record's creation the notification/autorevert/event files are still
running. Final frozen-input/product-identity checks, controller cleanup and
independent final review remain pending. This bounded result does not establish
all shipped-library compatibility, the full original corpus, GUI input behavior
or completion of the persistent Goal.

## Bounded continuation

The initial integration driver was deliberately interrupted because it had only
per-case deadlines and no absolute whole-test wall-clock deadline. Native
cleanup completes. All26099 captured owned source/product/object files match
their original content, size and mode afterward. The fifteen complete file/
product records (620 cases) remain; the partially run remote notification file
is not promoted to complete. New test-only execution will reuse the products
and split local notification/event tests from slow remote calibration, with
explicit whole-group and per-case time limits. No original assertion is changed.

## Independent bounded acceptance

Independent read-only verification matches both current files to the exact
pinned stable blobs, verifies their CLN114 hashes/lengths and checks the real
620-case results. It accepts this complete stable ERT component alignment as
a bounded delivery. The original description-mismatch case passes on every
product. Captured owned-file integrity is verified after cancellation; the
installed reference program/dump also match the retained earlier control
identities. This does not claim continuous protection of all installed
reference resources, a successful notification file, GUI acceptance or G3.
