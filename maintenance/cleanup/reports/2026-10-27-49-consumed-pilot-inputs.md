# CLN-20261027-49: consumed pilot corpus and generated state

Source ref: `e7ab2e8614bce9c343b3140b48772a38a633154b`. Both pilots have exited.
The first bridge was missing a closing parenthesis: reference and candidate
exited255 before loading original tests. Its five attempted processes, raw
failures and diagnostics remain. The repaired pilot actually passed116 ERT
executions from two original files across the installed control and D24
candidate. Six deliberate pass/fail/skip outcomes and a timeout with an
unexecuted following case were accounted for separately, not labelled as
corpus acceptance.

The manual pass removes3072 duplicate materialized inputs from
`build/acceptance/runtime-d25-harness-functional-{1,2}/original-corpus`.
Logical bytes41993904; allocated bytes before unlink50176000. Original1536
canonical inputs and69 retained result/log/control files have unchanged hashes.
Two lsof checks found no handles. Every selected file matched the pinned
manifest and canonical source; identities and both file/directory-fsynced
ledgers preceded unlink. Inode/mtime/size/mode/SHA were checked again; every
target is absent. The live full followup and all original product/source inputs
were outside the deletion scope. Execution timestamps are recorded as Unix
seconds1793047422.576736 through1793047423.375656 in the retained local receipt.

The new successful-file exit additionally reclaimed one consumed34096-byte
generated cache (36864 allocated bytes). Other successful states contained
only empty directories. Exact operation start/end timestamps were not separately
recorded for that automatic action. The original pre-delete ledger, per-file
result and private exact mapping remain; evidence SHA256 is
`fced9510f83b417010a5d01a7fe66d18f7905b681b2d10fb0e3e1a471566fac0`.

[Manual path/source manifest](../manifests/2026-10-27-49-consumed-pilot-inputs.jsonl.gz):
SHA256 `3eea45e8d5cc699f979aa0310dbc4dd1c531601c33c6f9bd38cd6db4b14ac908`.
[Automatic cache identity ledger](../manifests/2026-10-27-49-pilot-state.jsonl.gz):
SHA256 `80538e6321f79ad88c9fbfb71026ceb8e3178e8f71af1a7b55b51125e8426be5`,
byte-identical to its pre-delete original. Manual operational records are in
`build/cleanup/2026-10-27-49`.

Combined:3073 files and42028000 logical bytes. Restore original test inputs
from each manifest's `restore_from` into a new isolated output and verify
SHA/mode; the old pilot outputs are no longer complete materialized corpora.
Regenerate consumed caches from retained inputs; random names/inodes are not
promised. Raw failures, tests, results, manifests and the real product are kept.
Do not restart old pilots blindly or interpret retired inputs as compatibility
failures. This is bounded harness integration, not full stable-corpus, GC,
performance or native UI acceptance. Whole-volume free-space changes are not
attributed to this batch.

[Cleanup ledger](../LOG.md).
