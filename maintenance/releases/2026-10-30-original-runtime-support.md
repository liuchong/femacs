# Original stable runtime-support comparison

Thirteen further original stable files were executed on the installed
reference and current normal terminal/Cocoa products. Original test bodies,
guards and assertions remain unchanged. Each product uses an independent,
complete manifest-verified test copy through the existing materializer and
per-file ERT bridge. All discovered IDs execute, without tag filtering.

## Normal-product results

File names are relative to `test/lisp/emacs-lisp/`. Each product registers the
same 203 original IDs.

| Original file | Reference passed | Terminal passed | Cocoa passed | Candidate skips each |
| --- | ---: | ---: | ---: | ---: |
| `byte-run-tests.el` | 2 | 2 | 2 | 0 |
| `cl-preloaded-tests.el` | 1 | 1 | 1 | 0 |
| `cl-print-tests.el` | 6 | 6 | 6 | 0 |
| `comp-cstr-tests.el` | 93 | 93 | 93 | 0 |
| `comp-tests.el` | 3 | 0 | 0 | 3 |
| `gv-tests.el` | 10 | 10 | 10 | 0 |
| `float-sup-tests.el` | 1 | 1 | 1 | 0 |
| `backtrace-tests.el` | 9 | 9 | 9 | 0 |
| `bindat-tests.el` | 29 | 29 | 29 | 0 |
| `pp-tests.el` | 5 | 5 | 5 | 0 |
| `regexp-opt-tests.el` | 2 | 2 | 2 | 0 |
| `rx-tests.el` | 36 | 36 | 36 | 0 |
| `check-declare-tests.el` | 6 | 6 | 6 | 0 |
| Total | 203 | 200 | 200 | 3 |

There are **603 passes and six skips**, no failed assertions or timeouts.
Status maps match for twelve files. The full map comparison remains false:
the three original native-cache tests pass on the reference, while both
native-disabled normal candidates skip at their unchanged native feature guard.
The aggregate stays `passed=false` and the resource owner exits 1. This feature
difference is recorded, not hidden as equivalent status or full compatibility.

The group takes 19.195 seconds; the owner takes 21.482 seconds, peaks at
146,030,592 accounted bytes and fully reaps its process tree. Per-case events,
timings, registration, feature metadata and original false outcomes remain in
`build/acceptance/g3-runtime-libraries-1`. Programs, selected dumps, complete
selected libraries, original manifest inputs and harness identities are unchanged.
No generated resource files remain in these test copies. Generalized-variable
tests invoke the same product through the original invocation path without an
explicit dump argument; they do not prove the child loaded the parent's image.

## Native-enabled cache controls

To exercise the guarded cases, the existing native-enabled terminal/Cocoa
normal and checking products run the complete unchanged
`test/lisp/emacs-lisp/comp-tests.el`. All four products pass all three IDs:

- `test-native-compile-prune-cache`;
- `test-native-compile-prune-cache/delete-only-eln`;
- `test-native-compile-prune-cache/dont-delete-in-parent-of-cache`.

These **12 additional observations pass**, without skips, assertion failures or
timeouts. Their ID/status maps match the three reference observations already
executed above. The reference is not rerun: its exact program, dump and complete
selected library identities are checked unchanged before reusing that result.
The retained native products' complete resource groups and build receipts are
bound before and after execution. This is a separate feature configuration;
it does not replace or turn the six normal-product skips into passes.

The original cases create and prune a small owned hierarchy of dummy cache
files. They do not perform native compilation. The run takes 7.480 seconds;
its owner takes 13.371 seconds, peaks at 86,474,752 accounted bytes and exits 0
with complete cleanup. Raw records remain in
`build/acceptance/g3-native-cache-1`. There is no new product build, compiler
implementation, default-feature promotion or performance claim. These retained
native products are not substituted for the repaired current GUI workflow.

## Coverage boundary

Together with the earlier [seventeen original files](2026-10-30-original-core-libraries.md),
this covers thirty files and 883 distinct original IDs per normal product:
2,637 batch passes and twelve retained skips. The twelve native-enabled cache
observations use three of those existing IDs on four other profiles; they are
not twelve additional distinct corpus IDs. The three earlier interactive
EIEIO failures likewise remain separate mode observations of already counted IDs.

The original compiler bootstrap metadata failures, P427, intermittent remote
Base64, common DNS and EIEIO warning assertion remain open. This is not the
complete 555-entrypoint corpus, a full native/collector/checking matrix,
performance acceptance or completed G3. G1/G2/G4 remain three of seven delivered
outcomes; G3/G5/G6/G7 and the full Goal remain open.

Independent review accepted this bounded evidence after inspecting the original
guards, raw records, native-product paths, reference reuse and documentation.
Consumed test copies are retired separately in
[CLN139](../cleanup/reports/2026-10-30-139-consumed-runtime-support-test-copies.md);
replay requires rematerialization, while original evidence remains unchanged.
