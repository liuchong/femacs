# Original stable editing and command-loop comparison

Six complete original stable files run unchanged on the installed macOS
reference and current normal terminal/Cocoa products. Each uses a separate
complete manifest-verified copy of the pinned stable corpus
`a360712c9d272d950d8d8255ef74570f7e90b7d9`, its own program, explicit selected
dump and complete selected Lisp libraries. Existing materializer, per-file
bridge and ERT runner are reused, selecting all discovered IDs without tag
filtering. Registration includes the original generated tests.

## Batch results

| Original file under `test/src/` | Reference passed | Terminal passed | Cocoa passed | Skips per product |
| --- | ---: | ---: | ---: | ---: |
| `buffer-tests.el` | 406 | 406 | 406 | 1 |
| `cmds-tests.el` | 2 | 2 | 2 | 0 |
| `editfns-tests.el` | 32 | 32 | 32 | 0 |
| `indent-tests.el` | 3 | 3 | 3 | 0 |
| `keyboard-tests.el` | 25 | 25 | 25 | 0 |
| `minibuf-tests.el` | 65 | 65 | 65 | 0 |
| Total | 533 | 533 | 533 | 1 |

Each product registers 534 IDs: **1,599 passes and three skips** in total,
zero failed assertions/timeouts, and identical ID/status maps. All eighteen
file processes exit 0 and are fully reaped. The raw aggregate remains false
and its resource owner exits 1 because the original `test-line-spacing`
guard requires a graphical display unavailable in batch mode. Equal skipped
status is not a pass or full compatibility. No original guard is replaced.

The group takes 15.559 seconds; its controller takes 17.440 seconds, peaks at
175,570,944 accounted bytes and completes cleanup. Sources, manifest/harness/
helper, programs, dumps and full selected libraries remain unchanged. No
generated resource files remain in the copied test trees. Per-case events,
durations, registration and original false/skip results remain in
`build/acceptance/g3-editing-1/results.json` and adjacent bridge records.

## Actual graphical follow-up

The same unchanged `test-line-spacing` runs once on each actual nonbatch NS
reference and current Cocoa process. Both observations pass with no skip or
timeout. Runtime observations show `noninteractive=false`, a true graphical
display, `window_system=ns` and one live frame. Each process exits 0 and is
reaped. Terminal is not rerun for a graphical-only case.

The original loop advances x and y together from 0 through 50: 51 pairs,
not a 2,601-pair Cartesian product. It compares pixel heights of the same
two-line text for scalar and paired line spacing, making 102 queries per
product. The run takes 1.694 seconds; its controller takes 3.124 seconds,
peaks at 210,714,624 accounted bytes and exits 0 with cleanup complete.
Inputs, the prior batch result, programs/dumps/full libraries remain unchanged.
Records are retained in `build/acceptance/g3-line-spacing-interactive-1`.

These two observations use one already-counted original ID in another mode;
they do not erase the three batch skips or add two distinct corpus IDs.
This proves the original pixel-size assertion in graphical processes, not
physical mouse/keyboard/IME or visual screenshot acceptance.

## Resource, child-process and reproducibility boundaries

R42 inspection binds eighteen serial batch processes, a 45-second per-case
hard deadline and a 900-second whole-owner deadline. The heaviest original
loop performs 100,000 random overlay operations with at most 100 live overlays
and roughly 1,000 characters plus 200 slack. Its choices are not seed/trace
bound here, so equal statuses do not prove identical random operations across
products. The separate original generated trace fixtures are fixed source.
Other large text inputs are about 11,000 characters or 5,000 short strings
(roughly 30 KB); completion inputs and formatting widths are bounded.

Autosave cases mock their own prompts and use owned isolated temporary files.
Keyboard tests use queued/synthetic events, a one-second timer/two-second read
and a dedicated editor child with a three-second sleep interrupted by SIGINT;
external per-case/owner deadlines bound its wait loops. The original child
invocation specifies the same product executable but no explicit dump, so it
does not prove that child loaded its parent's selected image. Conditional
`getent` group checks remain original; the executable is absent on this host,
and that branch is not claimed executed. No host dependency is installed.
The graphical follow-up uses a 20-second per-process and 60-second whole-owner
hard deadline. The post-run inventory also finds one 34,104-byte native
trampoline ELN in the reference's fresh buffer-test environment, alongside
three four-byte `foo` autosaves (one per product); the graphical environments
are empty. The environment creator only creates directories and does not copy
that cache. Thus the original preflight did not account for this implicit
native-cache generation; no claim of zero compiler activity is made. Exact
compiler invocations were not instrumented. The protected owner still covered
its full process tree. No benchmark, user file or acceptance-contract change
is introduced.

For replay, rematerialize fresh separate test roots using
`tools/stable-suite.py --materialize` and
`compatibility/stable-31.1/sources.json`. Recreate recorded source/test/library
context links and run the six files via the existing per-file batch bridge,
explicit selected dump and all discovered IDs. For the graphical follow-up,
use the same runner's existing single-ID run mode in actual nonbatch NS
processes, retaining the original display guard and runtime observations.
Retired test paths require rematerialization; raw evidence remains available.

This is limited normal-product source-mode coverage. It does not finish the
complete stable corpus, native/checking/collector matrix, performance or G3.
Known P427, remote Base64, DNS, EIEIO and bootstrap failures remain open. The
user has not approved the proposed bootstrap acceptance change. G1/G2/G4
remain three of seven delivered outcomes; G3/G5/G6/G7 and the Goal stay open.
