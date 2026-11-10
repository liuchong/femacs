# Stable source-context path repair (P460)

The original help-description file failed five cases on both products because
the test bridge replaced physical product Lisp directories in `load-path`.
Preloaded definitions still named physical files. Help could no longer shorten
those names, so paragraph filling moved filenames beyond the first line tested
by the unchanged original assertions.

`stable-suite--source-context-load-path` keeps mapped context directories first
and appends the original product directories, retaining their spelling and
unrelated/nil entries. Original libraries, tests, data, case selection and
assessment are unchanged. The compiled cohort remains signed `9e257276`,
source inventory `6eeeadb287d270c6e45d9faaffc3d1750c627b996ba64e017c79b5038e546f38`.
This harness-only repair requires no product rebuild.

## Actual functional acceptance

The complete manifest-pinned `test/lisp/help-fns-tests.el` ran through the
canonical materializer, source context, per-file working fixture and batch
runner on the reference plus terminal/Cocoa baseline/native normal products.
Each product passed all22 original IDs:110 observations, matching registrations
and zero waivers. The five previously unexpected failures all passed.
Product/resource identities, all61 completed product receipts, frozen source,
harness, original corpus and source context passed postflight; case state was
retired by the existing canonical implementation.

Functional result SHA256:
`42f19d1804e8515585f1e684c7c1092aa7186fe3f5129a7913d242e0e5d8dd38`.
Business time58.088s; controller102.590s, exit0, cleanup complete,
peak accounted111,411,200 bytes, growth0, under16GiB hard limit.

Only after functional acceptance, six path protections were added: precedence,
subdirectory/original spelling, prefix boundary, unrelated/nil entries,
unchanged input and empty path. Five products passed30/30 observations;
products, source, harness and receipts stayed unchanged and case state retired.
Protection result SHA256:
`f50277fd77d0f6fcfc3050bec8613b4cad0f1efac73d584082df418f88720d4b`.
Business48.587s; controller118.682s, exit0, cleanup complete,
peak accounted101,613,568 bytes, growth0.

The owned original-corpus/context copies were then retired under
[CLN223](../cleanup/reports/2026-11-10-223-consumed-stable-context-copies.md).
Original functional and protective receipts remain unchanged, including the
functional receipt's historical pending-cleanup field.

## Scope

This closes the five-case harness defect and its bounded original-file check.
It does not complete the555-file/48-profile stable corpus, waive the four other
shared unstable/FIXME failures, or accept compatibility/performance/the Goal.
Independent final review accepted the complete repair and cleanup unit with
zero confirmed defects. The frozen matrix coordinator still defaults to its
original runner; selecting the corrected stable runner explicitly is required
before its next formal stable phase. The process-reporting bridge does not
make that selection. Current authored semantics is unaffected, and its active
coordinator remains unchanged until completion.
