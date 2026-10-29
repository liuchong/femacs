# macOS Tree-sitter feature (P430)

## Change and provenance

The initial feature-disabled port contained only the loader prefix and fallback
availability interfaces. The parser/node/query core, shared layouts and editor
mutation hooks were missing. Restore the parser/node/query and navigation groups,
including all 58 original Lisp entrypoints, against the pinned stable source
`a360712c9d272d950d8d8255ef74570f7e90b7d9`. The 58 macro entrypoint names
independently match the stable source. An earlier
132-name construction count is not a Rust function-token count or acceptance
coverage metric.

The restored flow connects edits, narrowing and indirect buffers to line/column
caches, incremental parse edits and timestamps; actual native trees are exposed
through rooted parser/node/query objects. Query native handles are nulled in
portable images and reconstructed lazily. GC, printing, node equality and
buffer lifecycle consumers share the same object declarations. The working-source
CLN132 draft records the removed development-only cache and duplicate
declarations; it remains pending with the prerequisite source group.

Both macOS configurations enable the feature. The independently installed
Tree-sitter 0.27.1 SDK is pinned by exact API header and package version, with
language ABI 13–15. Its real signed library closure and licenses travel with
both product packages through relative runtime paths. The installed C reference
bundle and ignored reference checkout are not product build inputs.

## Actual acceptance and retained failures

- Four new complete frozen-workspace products built successfully: normal and
  checking Cocoa and terminal. Both Cocoa apps passed strict signature checks.
  This build took 1233.821 seconds, with an 874103240-byte accounted tree peak
  and complete process cleanup.
- The actual SDK and Lisp/buffer mirrors agreed on 123 size/alignment/offset,
  enum and constant observations. Compile-time contracts covered all 55 used
  foreign functions and three callbacks. The C Lisp/buffer comparison used the
  configured read-only fd7 mirror; its Tree-sitter header is byte-identical to
  the stable header. This is not a claim of a full stable-source C build.
- All 40 unchanged original Tree-sitter cases passed on the installed stable
  C reference and all four candidates: **200 case-products**, zero failures,
  skips, timeouts or unexecuted cases. Eight fixed real grammar inputs, including
  jsdoc required by the JavaScript mode, are recorded in
  `compatibility/tree-sitter/grammars.json`.
- Three real parser/node/query/GC lifecycle integrations passed on all five
  products: **15 unique case-products**. These unchanged successful results were
  reused after matching exact source, fixture, program/dump and resource identities.
- Five separate two-process query dump/restore checks passed. Each restored image
  retained query language/source, cleared native handles, then rebuilt them for
  an actual JSON capture. These are five scenarios, not ten extra ERT cases.
- Original inputs, selected source, programs, dumps, packaged resources, grammar
  libraries and harness identities remained unchanged during each integration
  run. The corrected run took 35.493 seconds including controller overhead,
  peaked at 313704448 accounted bytes and completely reclaimed its processes.
- Two subsequent constant-size configuration unit guards passed: disabled targets
  need no dependency binding, and contradictory feature declarations fail before
  capture. No production change was added for those guards; both case timings
  are retained. Compilation and execution took 9.409 seconds, with a
  921518080-byte peak and complete cleanup.

The first build failed because an unchecked historical checklist suppressed a
required release object, and two checking comparisons used the wrong integer
width. Product selection now follows the explicit link manifest for existing
core Rust units; the checklist fast gate is unchanged. Two diagnostic TU runs
used an old driver pinned to its original frozen source and do not verify the
new root changes. All original diagnostics remain retained.

The initial ABI preparation failed on nested callback type parsing, then a
second probe was rejected by the frozen driver's owned-output guard. The final
probe ran under that driver's own build root; no guard was weakened. Both
preparation failures remain distinct from the successful ABI measurement.

The first full original run passed 195 cases but failed the same JavaScript
case on all five products: the additional jsdoc grammar was missing and the
original mode prompted for installation in a noninteractive process. Its raw
failure remains. Installing the fixed real grammar resolved that prerequisite;
no original assertion or production behavior was changed. Initial dump restore
checks also failed on all five products because the runner reloaded the fixture
already present in the new image. Restoring the saved functions directly fixed
the runner; the original failed logs remain. No editor rebuild was needed.

## Bound candidates

These are complete frozen-workspace identities, not proof that an isolated
commit or clean checkout already builds independently.

| Product | Program SHA256 | Dump SHA256 |
| --- | --- | --- |
| cocoa-normal | `cf286f80f34cfc73279103412524859ffb1294802d7a232ce89bac4ba94a7d49` | `484f66e99fb0c1e0d35b59f00a8605f6f628fa987918d3689c9a2cc3601cb7a3` |
| cocoa-checking | `e922aee119a39caf7c144e57a803777ffbe44e374a17f3003c385ff9728d91fe` | `e13b4f07911ca8d59c52eebaaf273e06431d9ddd9b3f63b3cca00522e8022a8d` |
| terminal-normal | `a46d3a2ca7c7cfa74536487747a47f3e873542c0c5776c39fc91016485a247f1` | `998677eb1d0bb2d5350f6592f7a2b7000b45beeaef49e11f65101358f552f104` |
| terminal-checking | `0dfe0c633732546818bc05153fdd3b4b2f2b5d779b26830db4a302b9697e0744` | `63e1a1970bb7df80b85d53cd2308cbb7147ede21ad0f46a98c0175958298e756` |

Local raw evidence: `build/acceptance/runtime-g3-treesit-current-products-1`
and `-2`, `runtime-p430-tu-integration-1` and `-2`,
`runtime-p430-sdk-lisp-abi-1` and `-2`; the passing ABI result is under the second
frozen source's `build/acceptance/runtime-p430-sdk-lisp-abi-3`.
Grammar compile receipts are `runtime-g3-treesit-grammars-1` and
`runtime-g3-treesit-grammars-jsdoc-1`. Original integration attempts are
`runtime-g3-treesit-original-1` and `-2`; configuration guards are
`runtime-p430-protection-1`. Paths without a prefix are relative to
`build/acceptance/`. The new functional fixture is
`tools/fixtures/treesit-integration.el`, and the measured ABI tool is
`tools/treesit-abi.py`.

## Delivery boundary

The existing independent reviewer made 20 actual read-only source/evidence
checks and accepted this limited implementation and observed product evidence,
with no concrete source failure found in that scope. The review took 301.815
seconds, peaked at 2506342400 accounted bytes and fully reclaimed its processes.
One trailing-whitespace-only source cleanup followed the review; no behavior,
assertion or bound product changed. Coherent prerequisite source delivery remains
pending. The tested workspace includes native/runtime, TLS, LCMS and
owned-copy work not yet independently committed. Do not publish a half-wired
feature commit omitting those modules. No clean-checkout, relocation, standalone
SDK absence, physical GUI, other platform, performance or full G3 claim is made.
P427 terminal checking EBADF, the remote Base64 issue, the original compiler
metadata comparison failures, G1/G2 completion and native/GC work remain open.
The seven-part Goal remains active at 1/7.

Consumed compiler copies were retired with source/product identities preserved;
see [CLN133](../cleanup/reports/2026-10-29-133-consumed-treesit-compiler-copies.md).
Coherent prerequisite implementation delivery remains pending.
