# GIO startup stream correction — 2026-10-29

The GIO process filter could cache an unknown monitor after receiving only the
startup heading. A later backend line then could not correct that defined-nil
cache. Actual C/candidate chunk and cache observations establish this state
error; they do not establish lost notification events or missing Rust buffering.

P423 changes only incomplete startup handling in the production Lisp filter.
It retains partial records using the existing rest-string path, before caching
nil. Positive recognition, first-backend priority20 selection, complete unknown
responses, error handling and event mapping remain unchanged. It does not wait
for a file event before resolving a complete unknown response: original tests
can query the monitor before generating events. Missing/incomplete startup
output is not given an invented timeout or a different backend-selection rule.

## Actual integration and unresolved failure

All four new terminal/Cocoa normal/checking products build successfully; both
Cocoa strict signatures pass. Source and compiled-library identities are bound
to these products rather than borrowing acceptance from older binaries.

The first normal, uninstrumented Cocoa original26 file completes21 passes,
4 original conditional skips and1 remote09 Base64 failure. The formerly extra
remote05/11/12 monitor-detection skips now pass; remote03 also passes. The09
failure remains unresolved and its original failed record is retained.
An isolated09 reference/candidate observation and a subsequent full-order
observation do not reproduce it. Their success is not a repair or full-file
acceptance; observation changes scheduling and captures no new failing payload.

On the other three products, unchanged original local13 and remote03/05/11/12
cases complete48 passes/3 original conditional skips/zero failures or timeouts.
The three skips are the original local11 kqueue limitation, also skipped on the
installed pinned stable31.1 comparator. All affected remote cases pass, with no
reference-passing case regressing in this related set. Executable/dump/resource,
source, bridge and original-input identities remain unchanged. The strict raw
passed:false and controller exit1 remain; skips are not counted as passes.
This related integration takes868.525seconds; native cleanup completes in872.201.
It does not establish full26, complete stable-corpus or G3 acceptance.

## Post-integration protection

`tools/fixtures/gio-stream-protection.el` runs independently with ERT. Its eight
component cases use real pipe-process and Tramp cache state, with an isolated
editor-event sink. They cover the captured heading split, all81 two-piece
boundaries of the80-character startup, bytewise delivery, complete unknown
responses without events, unchanged first-backend selection, normalized/split
events and complete/split unsupported-monitor errors. They do not replace the
real GIO integration above.

All eight cases pass on each of the four new products. The old owned Cocoa
product, selected only for the captured-heading negative control, fails at the
premature defined cache exactly as expected. The negative result is not erased
or labelled a passing old implementation. Actual loaded library hashes match
the corresponding packaged bytecode. Products and fixture remain unchanged.
The complete protection/control run takes17.848seconds; native exit0 and full
cleanup take22.098seconds, with183353344 bytes peak accounted memory.
These are test timings, not performance measurements.

With a matching built program/dump and an existing output directory, replay the
component cases using the public fixture; no reference checkout is required:

```sh
FE_GIO_RESULT=build/gio-protection.json "$PROGRAM" --dump-file "$DUMP" \
  --batch -Q -l tools/fixtures/gio-stream-protection.el -f fe-gio-protection-run
```

Filter source SHA256:
`fcb610677a30744e3da17edff7014ffefd9aa9d90d96c70b57c380602944ba53`.
Fixture SHA256:
`fbbcce0782a55a88c2bb72353c5eac63ec8f14213610eb7943b2d32ea1845890`.

| Product | Executable SHA256 | Actual packaged dump SHA256 |
| --- | --- | --- |
| cocoa-normal | `0caf91ff449d3283af71050e7187dd81d401e2fa5b05a02d620faba6ad9982d9` | `31c21a1c3054762d6bfee6c070a401836a57d55b6edeae7725d7ab98ffaf5ea3` |
| cocoa-checking | `ff268de8e4ea617b719268b431089380fef5ef70ca05ae2cc03cda899b7f1024` | `85bb98365a933e4c0cc76c995968f6fba6ad7cfd58f2c9c2d1a60308040c99c3` |
| terminal-normal | `c07e149f50660930b4ab65848eab291d4d4f0ca2c8b8f6f4ed522436d1ec09a8` | `5aa7ef4d1b6ebe4ad278993877706c58bd0cdfc5ab0adcb21204821e2ed9ab68` |
| terminal-checking | `82c6c89d09bc74aad9e2bcf08c0d91832f5640569b642c321bd97baa53d0827b` | `e6116e26c7ccb4fb0c8756fe59052a43c7c0c569c9d031d9bfc7086c5861d558` |
| old-cocoa-negative-control | `008b20b65ebea8f1fe2426a6e1630536267f849a6c29fbe9fa8adcf6a16dc028` | `8164be808762948b4c2cde06fb3d84181540147ece815a33ec40d1b992e6095d` |

Raw integration, protection and negative-control receipts remain under
`build/acceptance/runtime-p423-variant-regression-1` and
`build/acceptance/runtime-p423-gio-protection-1`; original09 failure and both
non-reproductions remain in their separate original records. Consumed context
copies have the regeneration boundary in CLN126; current actual products remain.
The independent partner read the actual source, product/library identities,
related integration and complete protection/control records, accepted the
original-selection rebuttal and this bounded technical correction. Source
delivery is established by actual Git signature and remote-branch receipts,
not by these runtime results. Whole G1/G3 and the seven-part Goal remain open,
including the intermittent09 failure.
