# Original stable core-library comparison

Twelve unchanged stable-corpus files were executed on the installed reference,
the current root terminal and the repaired Cocoa product. Tests, library bytes,
runner assertions and product files were not changed for this comparison.
Each file runs every discovered original test ID; all three products register
the same IDs and produce identical per-case status maps.

## Batch results

Paths below are relative to `test/lisp/emacs-lisp/`. Counts are per product.

| Original file | Passed | Skipped |
| --- | ---: | ---: |
| `seq-tests.el` | 52 | 0 |
| `map-tests.el` | 62 | 0 |
| `cl-lib-tests.el` | 27 | 0 |
| `cl-generic-tests.el` | 19 | 0 |
| `generator-tests.el` | 92 | 0 |
| `nadvice-tests.el` | 13 | 0 |
| `pcase-tests.el` | 14 | 0 |
| `subr-x-tests.el` | 46 | 0 |
| `cl-seq-tests.el` | 61 | 0 |
| `cl-macs-tests.el` | 77 | 0 |
| `cl-extra-tests.el` | 38 | 0 |
| `eieio-tests/eieio-tests.el` | 41 | 1 |
| Total per product | 542 | 1 |

This is 543 distinct original IDs per product: **1,626 passed and three
skipped batch observations**, zero failures/timeouts. The first six-file group
passes all 795 observations. The second group retains `passed=false` because
its three skips are not passes. Per-case timings, original registration and
event identities remain in `build/acceptance/g3-core-libraries-1` and
`build/acceptance/g3-core-libraries-2`.

The groups take 8.415 and 13.458 seconds respectively. Their resource owners
exit 0 and 1, both with complete process cleanup; peak accounted memory is
195,182,592 and 606,863,360 bytes. Input test/runner and complete selected
product resource identities remain unchanged around execution. These times
describe this compatibility run, not comparable performance gains.

## Preserve the uncovered message assertion

The skipped ID is `eieio-test-01-mix-alloc-initarg`. Its original guard requires
`message` followed by `current-message` to retain the text `foo`; batch mode
does not satisfy that condition. The guard and test body were not changed.

A bounded follow-up runs this exact ID through the same existing original-test
bridge in genuine non-batch processes: the reference and candidate Cocoa
frontends have `window-system=ns`, while the terminal runs on an owned real
PTY. Each observation records `noninteractive=false` and one frame. The case
now executes on all three products, and **fails on all three**: its final
assertion expects `Class allocated slots do not need :initarg`, but the actual
message remains `foo`. Four relevant EIEIO source files are byte-identical
between the installed reference and candidate. No candidate-specific difference
is established, and no speculative product or upstream-library fix is made.

The source identity claim binds these exact files, relative to each product's
Lisp directory. Each hash is the same on all three products; source equality
does not independently prove bytecode identity or explain the failed assertion.

| File | SHA-256 on reference, terminal and Cocoa |
| --- | --- |
| `emacs-lisp/eieio.el` | `3fa2814941bc67b902edd08e9aa43ac14d36d9a046483ad7dce9e9efac0f608d` |
| `emacs-lisp/eieio-core.el` | `f3ba943ea6015a2206bea7ab9def637b34b8ee1438f33142ca405da998d651ab` |
| `emacs-lisp/eieio-opt.el` | `fbe430e3f63985210af179865a1d1ccdffa6bfe61ad64d34491cf1195202b3a0` |
| `emacs-lisp/eieio-base.el` | `e64c8ce09ccc7ef6d1c575e71e85169b5f5f698b81c8049f55fb71bb3d4f5f28` |

The follow-up takes 4.230 seconds, exits 1 through its owner and completes
cleanup, with 177,061,888 peak accounted bytes. Original per-case failures and
observed interactive identities remain in
`build/acceptance/g3-eieio-interactive-1`. All source/product identities are
unchanged. Its individual editor processes exit normally; those exits do not
turn failed ERT assertions into passes. These are startup-time interactive
test executions, not keyboard/IME or complete editing workflow acceptance.

The three interactive failures are separate execution-mode observations of
the same three case/product pairs that skipped in batch; they do not add
three distinct corpus IDs. Original skips and false results are preserved.
The missing expected warning remains an unresolved original assertion, not a
waived gate or a claimed complete compatibility result.

## Scope

Only the installed stable version and these two current normal candidate
frontends are covered. This is not the complete 555-entrypoint corpus, checking
builds, native/compiler/collector profile matrix or a performance comparison.
Existing P427, remote Base64, DNS and compiler metadata failures keep their
own evidence and scope. G3/G5/G6/G7 and the full Goal remain open; G1/G2/G4
remain three of seven accepted outcomes.
