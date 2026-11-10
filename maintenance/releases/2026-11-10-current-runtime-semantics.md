# Current-source runtime semantics and relocation boundary

The complete semantics phase used the61 profiles built from frozen source
`59d4215c9d0eb3706f366941d3c40e3a2b879dd5`, with outer coordinator
`3be2e1b9b5e75a2530a303d21f5762762fb4a889`. Products digest remains
`cae6cecc38754885f981760870c80b68c3a541b926351e3e38634d1a150227aa`.
The [build and preparation record](2026-11-10-current-runtime-matrix-build.md)
describes provenance and exact configuration scope.

| Action | Complete observed scope | Result | Completion SHA256 |
| --- | --- | --- | --- |
| ERT | 903 discoveries and9,432 exact tests;10,335 processes | all pass | `6304b60b2d7339f273ff2ad9249063eaccd9ddce8872149890fe002f848a688b` |
| Native format controls | 608 cases across32 profiles | all pass | `b29b0046238e562cdfeda117104409d7edd6269ab6cfa21d733c6016233b53fc` |
| Dump and restore | 48 applicable profiles | all pass | `ac86e35772a8fd38dbebe735706d5e70714722a7dcf2574b893c74043bc7f258` |
| Source-denied relocation | 60 profiles;124 language/native cases | all functional cases/signatures/denial checks pass;14 copy-retirement failures | `39a487a1d156be2d096b59a0a9294cae6fca15841fd2c7a4f3916e75ed534446` |

The original outer result remains **passed=false**, execution_complete=true.
It is10,238 bytes, SHA256
`4fffc79874f912cff355ef8ee97ec7949ce7b861c4e019459b30a43d0da61f1b`,
under `build/acceptance/runtime-review-p453-semantics-1/results.json`.
All four processes exited naturally and were collected; no timeout or remaining
process group. Whole duration6,389.838s, peak3,353,346,048 accounted bytes,
initial4GiB/hard16GiB, no growth. This was a normal exit1, not a monitor or
memory-pressure interruption. These are execution/protection readings, not G5
workload performance measurements.

Every original product, full receipt and frozen input remains unchanged.
File retention verifies198,270 files and20 links, without a duplicate archive;
the journal is released without failures. A separate streaming recount of the
200,355,934-byte ERT detail verifies all10,335 outer process rows passed.

The14 relocation failures came from the strict no-holder cleanup gate:
recorded `mdworker_` read-only descriptors held temporary copied packages.
All124 relocated functional cases and signature checks completed successfully;
the gate retained the copies. Original failed rows and occupancy evidence remain
unchanged. Fresh checks subsequently found all14 copies unoccupied. A separate
bound late-retirement transaction then passed, retiring62,947 leaf nodes and
1,597 package directories, followed by62 exact empty parents. It preserved all
85 bound inputs and original products, ran no editor/compiler/semantic cases,
and left the original failed result unchanged. Logical payload was3,896,765,173
bytes, with4,040,814,592 prior per-path allocated bytes; no exclusive physical
volume-reclamation attribution is made. Result SHA256
`563cb75813650b8438b78517792c023483869c288793c931c6a4a771a60249e8`,
controller exit0/full cleanup,166.169s, peak61,276,424 bytes. Independent review
accepts both the closed semantic boundary and this separate physical completion.

Future copies use an owned `relocation-work.noindex` parent (P456), retaining
all strict no-holder and identity checks. This follows Apple's documented
[Spotlight path exclusion](https://developer.apple.com/documentation/xcode/locating-a-missing-debug-symbol-file).
No system indexing settings are changed. The frozen run used its original
harness; the namespace correction is not presented as a replay of that run.

[CLN221](../cleanup/reports/2026-11-10-221-consumed-runtime-semantic-state.md)
publishes complete consumed-state/copy/parent identities and records retirement
of the audit mirrors after independent reconstruction of every projection scope.

This phase does not execute the full555-file stable corpus or quiet-host
performance matrix. G1/G2/G4 remain accepted; G3/G5/G6/G7 and final adoption,
installation and whole-Goal acceptance remain open. Physical native UI evidence
is separate from these batch semantic and relocation observations.
