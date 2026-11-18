# Bootstrap container identity evidence

The maintained `tools/native-bootstrap-identity.py` reads two explicit native
containers without executing or modifying them. It verifies the supported
ARM64 dynamic-library layout, exact metadata ranges, all code/data sections,
and every other byte. Before isolating the ad-hoc signature as metadata it
checks its CodeDirectory structure, executable-range bounds and every SHA256
page hash; normalized signature comparison excludes only its identifier and
derived hash slots. [Format declarations](https://github.com/apple-oss-distributions/xnu/blob/xnu-11215.81.4/osfmk/kern/cs_blobs.h)
provide the signing layout. No old diagnostic script is an input.

Actual file integration read both retained bootstrap pairs from the current
matrix's earlier diagnostic. The reference pair had74 differing bytes:
13 in its declared library name,16 in UUID and45 in signature. The owned
Cocoa native pair had76 differing bytes in the same three regions. Both
pairs had zero differing bytes elsewhere, identical sections and equal
normalized signatures. All243 signed pages of each of the four images
were rehashed successfully; input identities were unchanged at exit.

The first parser invocation incorrectly required the executable signature
extent to equal the entire executable mapping size and rejected both pairs
before writing results. The corrected check bounds that extent inside its
executable mapping; signed-page coverage independently includes the whole
image prefix through the declared code limit. Two subsequent actual file
requests completed. No editor or compiler was started for this integration.

These are container facts, not a passing bootstrap test, a proof of runtime
behavior equivalence or accepted compatibility. Fresh original ERT events,
closed retained-state provenance and each side's actual capabilities must
still be bound by classification before any case can be accepted. Native-off
bootstrap and its different reference/candidate failures remain blocking.

Reproduce with fresh original bootstrap outputs:

```sh
python3 tools/native-bootstrap-identity.py --first FIRST.eln --second SECOND.eln --out build/acceptance/NEW
```

The output records raw input hashes, parsed ranges/sections/signature facts
and raw difference counts. A zero command exit means collection completed;
inspect `differs_only_in_validated_metadata` separately. Unsupported or invalid
formats fail, with no alternate parser. Re-running creates new measurements,
not historical UUIDs or signatures. Consumed retained images and diagnostic
code are retired under CLN232; current products and the active semantics run
are retained.

The additional file-closure mode passed5 synthetic file/CLI/reader flows. It reuses the original file-protocol validator and retirement-state
verifier, binds the exact owner/progress/event hash chain, and reads image
paths only from the original comparison message. Both images must match the
closure-time retained-state snapshot before and after analysis. Its output
still does not certify the outer stage or accept compatibility:

```sh
python3 tools/native-bootstrap-identity.py --pair-root PAIR --file-result PAIR/results/candidate/FILE.json --out build/acceptance/NEW
```

The corresponding typed source rule requires the original comparison failure
and this proof on every native-enabled side. A native-disabled candidate must
instead have the exact unavailable-variable outcome and actual absent native
capability; its different failure stays explicit. Fresh full-corpus binding,
full real-data acceptance and key protections remain pending. No original
failed observation has been promoted.
