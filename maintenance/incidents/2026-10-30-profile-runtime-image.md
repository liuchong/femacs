# Standalone profiling selects the actual runtime image

The standalone profiler originally assumes the owned bundle's dump location.
The installed stable reference uses a different supported bundle location.
It also fingerprints an inferred image without explicitly passing that image
to its workload and startup commands. This prevents a reliable reference run
through the documented standalone entry; it is not a defect in Lisp evaluation.

## Repair

Resolve the executable's complete name plus `.pdmp` in its sibling directory
and, for a bundle, the two supported libexec directories. Do not truncate a
versioned executable's final numeric suffix or guess a bootstrap image.
Distinct images are ambiguous even when their bytes match; aliases of the same
resolved file are one choice. An explicit `--dump` selects the desired image.
Require the product's Lisp directory, pass the selected dump to both subprocess
kinds, and include the profiler itself in the before/after input fingerprints.
The larger runtime-stage runner already selects its image explicitly; its
execution-mode, native/collector controls and existing results are unchanged.

## Actual validation

The installed reference, current root terminal and repaired Cocoa product each
complete all six unchanged workload cases and one fresh-process startup through
the standalone CLI. All 18 workload results match their deterministic expected
values, all three startups exit zero, and all 14 selected input identities per
product remain unchanged. The serial group takes 1.978 seconds; its controller
exits zero with complete process cleanup and 101,777,408 peak accounted bytes.
Raw results remain in `build/acceptance/p433-profile-inputs-1`.

The same independent partner accepts the limited group after four focused
read-only checks (five recorded tool calls). Its suggested exact-binding
refinement is adopted: the before/after fingerprint calls use the already
selected dump, rather than discovering the selection again. The affected
three-product standalone CLI check passes again with unchanged inputs, all
18 workloads and three startups, in 3.142 seconds; controller exit is zero,
cleanup is complete and peak accounted memory is 103,989,248 bytes. This final
source-bound result is `build/acceptance/p433-profile-inputs-2`; the earlier
result is retained rather than added to the unique-case total.

After functional acceptance, four focused protection methods are added to the
existing release-baseline component suite. They cover both bundle layouts,
versioned executable names, missing/ambiguous/explicit images, resolved aliases,
the profiler identity and generated-library drift. All 11 existing and new
component methods pass, with individual timings, in 0.035 seconds; evidence is
`build/acceptance/p433-profile-protection-2`. The initial component run retains
three failed assertions caused by comparing unresolved temporary-directory
aliases with canonical paths. Only the new test paths are corrected; product
behavior and assertions about selected files are unchanged. That original
false result remains in `p433-profile-protection-1` under the same root.
After the exact-binding refinement the same 11 methods pass in 0.045 seconds,
recorded in `build/acceptance/p433-profile-protection-3`.

## Scope and replay

```sh
python3 tools/runtime-profile.py --binary build/terminal/port/src/emacs \
  --rounds 1 --out build/acceptance/profile-input-smoke
python3 tools/test-release-baseline.py --component-only \
  --out build/acceptance/profile-input-protection
```

The first command runs the actual editor and requires the current resource
controller and an appropriate hard deadline. The second selects lightweight
components only. Use fresh output paths; results are not overwritten. An
installed reference or bundle executable can be selected with `--binary`, and
`--dump` resolves deliberate nonstandard or ambiguous layouts.

Each invocation has only one smoke round and reports `quiet=false`; the first
has load/core 0.2642 and the final one 1.4900. This establishes successful input
selection and execution, not
reliable performance improvement, cold-cache startup, phase attribution,
object survival or complete compatibility. G3/G5/G6/G7 and the full Goal remain
open; G1/G2/G4 remain three of seven delivered outcomes.
