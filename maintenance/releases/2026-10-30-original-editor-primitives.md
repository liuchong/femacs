# Original stable character, keymap and marker comparison

Four complete original files from pinned stable commit
`a360712c9d272d950d8d8255ef74570f7e90b7d9` run on the installed macOS
reference and current normal terminal/Cocoa. Each uses its own program,
explicit selected dump, full selected Lisp libraries and a separate complete
manifest-verified test copy. Existing materializer/per-file bridge/ERT runner
are reused, with all discovered IDs and unchanged bodies/guards/assertions.

| Original file under `test/src/` | Reference passed | Terminal passed | Cocoa passed |
| --- | ---: | ---: | ---: |
| `character-tests.el` | 1 | 1 | 1 |
| `chartab-tests.el` | 5 | 5 | 5 |
| `keymap-tests.el` | 47 | 47 | 47 |
| `marker-tests.el` | 3 | 3 | 3 |
| Total | 56 | 56 | 56 |

All **168 observations pass**, without skipped IDs, failed assertions or
timeouts. Each product registers the same 56 IDs, with matching status maps.
All twelve file processes exit 0, complete and leave no remaining group to
terminate. The aggregate stays true, with unchanged input/harness/helper and
program/dump/full-library identities. No generated resource files remain.

The pinned marker file has three cases; the checkout's newer fourth
`marker-markers-in` case is not part of this pinned corpus and is not claimed
as tested. The other three pinned files are byte-identical to their checkout
copies. Cases cover short multibyte/composed string widths, character-table
ranges/parents/slots, keymap inheritance/remapping/menus/help, and moving a
marker from generated help into another buffer/window. The original keymap
position fixture is synthetic, not a physical click. These batch observations
do not replace real keyboard, display/font, mouse or multiwindow IME evidence.

R42 inspection binds twelve serial file processes, eight fixed character-table
ranges (widest 65,537 codepoints), small keymap fixtures/two binding orders and
bounded preloaded-map/help traversal, short strings and three marker cases.
There is no external compiler, network, sleep or requested crash. Per-case
hard deadline is 45 seconds; the protected owner deadline is 300 seconds.
Execution takes 8.770 seconds; its controller takes 10.845 seconds, peaks at
216,530,944 accounted bytes and exits 0 with complete cleanup. These timings
are not a performance improvement claim.

Raw per-case events, durations, registration and input/product fingerprints
remain in `build/acceptance/g3-editor-primitives-1/results.json` and adjacent
bridge records. For replay, use `tools/stable-suite.py --materialize` with
`compatibility/stable-31.1/sources.json` for fresh separate test roots,
recreate each recorded context's source/test/library links and use the
existing `tools/stable-suite-batch.py` per-file path plus
`tools/fixtures/stable-suite-runner.el`, explicit selected dump, all IDs and
recorded deadlines. Retired test paths require rematerialization; the pinned
source cache and raw evidence remain available.

This is limited normal-product source-mode coverage, not a complete stable
corpus, feature/checking/native/collector matrix or finished G3/performance
acceptance. P427, remote Base64, DNS, interactive EIEIO and original bootstrap
failures remain open. The user's request to explain the proposed bootstrap
check is not approval; that acceptance contract remains unchanged. G1/G2/G4
remain three of seven delivered outcomes; G3/G5/G6/G7 and the Goal remain open.

Independent review accepts this limited result after three actual read-only
tool calls and three focused checks. Consumed test copies are retired in
[CLN142](../cleanup/reports/2026-10-30-142-consumed-editor-primitive-test-copies.md),
without rerunning tests or changing raw evidence/products. Replay requires
rematerializing the isolated trees before execution.
