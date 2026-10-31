# Original macro expansion, variables, patterns and generic functions

Four unchanged pinned files run on the installed reference and accepted P435
normal terminal/Cocoa products: **147 expected results = 141 ordinary passes
and six declared expected failures**, zero skips or unexpected failures. All
twelve file processes complete with exit 0, no timeout, protocol/runner error
or remaining process group. ID, tag and status maps match across products.
Independent source, original-event, identity and controller review accepts this
bounded coverage; it does not accept the whole compatibility goal.

| Original file | IDs per product | Input and expectation |
| --- | ---: | --- |
| `macroexp-tests.el` | 6 | Tracked pinned source |
| `gv-tests.el` | 10 | Two original expected failures per product |
| `pcase-tests.el` | 14 | Pinned overlay; the tracked fifteen-ID file is not used |
| `cl-generic-tests.el` | 19 | Tracked pinned source |

The two expected-failure IDs are `gv-dont-define-expander-in-file` and
`gv-define-expander-in-function`. All six produce `ert-test-failed`, expected
true and runner status passed. They are successful expectation classifications,
not six successful assertion bodies. Complete condition text differs across
products; equal status maps do not claim byte-identical error output.

## Inputs and real execution

Pinned commit: `a360712c9d272d950d8d8255ef74570f7e90b7d9`.
All four inputs match `compatibility/stable-31.1/sources.json`; pcase comes from
`compatibility/stable-31.1/test/lisp/emacs-lisp/pcase-tests.el`. Each isolated
copy starts with 1,536 manifest files / 20,996,952 bytes. All discovered IDs run
in one file process, without filtering or changes to tests, guards or tags.

Ten explicit input identities are equal before/after; program, dump and full
resource identities also match. Terminal is bound to `p435-functional-normal-1`;
Cocoa program/dump/bundle/resources to `current-macos-core-products-1` normal.
These are reused accepted P435 products, not a P444 rebuild or checking run.
Recorded reference version is 31.1; current terminal/Cocoa report 32.0.50. All
three run in batch mode. The receipt binds version text and configuration.

These tests perform real synchronous child-editor and in-process byte
compilation. Source bounds include at most three macroexp and six variable
child invocations per product, fixed small forms, no network or positive sleep.
Child waits and process-group cleanup complete; no independent child-dump
fingerprint acceptance is claimed. The reference also automatically creates a
native message trampoline. No active product build request does not mean no
automatic compilation occurred.

Each copy gains exactly `macroexp-resources/m1.elc` and `vk.elc`: six generated
files, 5,623 bytes total, recorded separately from pinned inputs. Their hashes
are not all equal across products. Passing original assertions does not prove
byte-identical compiler output. Existing ignored worktree bytecode was not
copied or modified.

Deadlines are 30 seconds per load/case and 300 seconds for the controlled owner;
estimated 10--90 seconds / 512 MiB. The estimate is not a limit. The run uses a
stricter 1 GiB hard tree limit with zero growth. Driver time is 9.447262 seconds;
controller 10.720 seconds, 213,336,064 accounted peak bytes, exit 0 and cleanup
complete. The original controller log is independently reviewed. These figures
describe coverage and resource use, not a speed comparison.

## Delivery boundary

No production edit or new protection test follows. Whole ERT, checking, full
G3, native/collector/UI and performance acceptance remain open; no trial App
replacement. G1/G2/G4 stay **3/7**, original Goal active. Continue remaining
compatibility and evidence-led runtime work without replaying unchanged passed
or capped workloads as new acceptance.

CLN175 retires 4,618 files, 885 directories and eighteen links, 63,086,871 logical
/ 75,390,976 allocated bytes. This includes 4,614 copied/generated test files,
one already-retired trampoline and three consumed operational helpers. All
123 automatic directories, including 24 parents, count once. Raw results,
events, logs, private plans, the sole test replay driver, live products, pinned
inputs and unrelated WIP remain. The three test/environment/context roots are
absent; context contained eighteen links, not empty directories.

[Result receipt](2026-10-31-original-macroexp-variables-results.json).
[Cleanup report](../cleanup/reports/2026-10-31-175-consumed-macroexp-variable-state.md).
