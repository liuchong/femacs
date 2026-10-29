# CLN138: consumed original compiler-test copies

The five original macro/closure/compiler files have completed on all three
normal products, and independent review has consumed their isolated fixtures.
The result remains 408 passes and three original sandbox skips, with false
aggregates preserved. This cleanup changes no test requirement or product.

## Scope and identity

Only `test` under the reference, terminal and Cocoa children of
`build/acceptance/g3-compiler-libraries-1` is retired, together with those
children's three `context/test` links. Each test tree contains its own 1,536
manifest source copies and 59 generated resource bytecode files. No other
build, source corpus, editor configuration, user package or process is selected.

The three roots contain 4,785 regular files, 753 directories and 63,052,690
logical file bytes; three context links are also removed. File allocation was
76,001,280 bytes, which is not a claim of equivalent physical space reclaimed
on a copy-on-write volume. The public
[identity manifest](../manifests/2026-10-30-138-consumed-compiler-test-copies.jsonl.gz)
has 5,541 rows and SHA-256
`329f7fe4eeddcad6999f9a4dfa0cff6d83502458f5da3dd4d161566232dae65b`.
All decoded fields match the private file/directory/link identity plan. Link
targets are expressed relative to the repository, without local machine paths.

## Execution and preservation

The compatibility owner ended with exit 1 because of the original skips and
complete process cleanup. Every selected file process completed with exit 0,
including all original test IDs; no process group remained. Review also ended
before retirement. The existing `tools/runtime-retire.py` checks open handles,
writes durable per-file ledgers, verifies identities before unlinking, removes
only the named owned child and binds receipts to unchanged result evidence.
The original `passed=false` is preserved; successful process reaping is not
misrepresented as complete test acceptance.

All selected roots and links are now absent. Git HEAD/index and existing
working changes, original result/log/event/owner evidence, current programs,
dumps and complete selected product libraries are unchanged. The complete
pinned corpus cache and source manifest remain available. No editor, compiler
test or passing group is rerun for cleanup.

To replay, materialize separate fresh product test trees from
`compatibility/stable-31.1/sources.json` through the existing stable-corpus
materializer, restore each product's context links and run the recorded
unchanged files with the existing per-file bridge. Generated resource bytecode
is reproduced by those original tests; original inodes and timestamps are not
promised. Old context paths need rematerialization before reuse. See the
[bounded compatibility result](../../releases/2026-10-30-original-core-libraries.md).
