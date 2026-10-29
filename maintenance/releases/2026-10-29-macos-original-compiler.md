# macOS original native compiler evidence

## Actual result

The unchanged stable `test/src/comp-tests.el` and its seven resources at
`a360712c9d272d950d8d8255ef74570f7e90b7d9` were run on the installed 31.1 C
reference and four fresh native-enabled frozen-workspace products. All five
registered exactly 186 identical IDs and finished every case. Each returned
185 passes and the same bootstrap binary-comparison failure: 925 passes, five
failures, zero skips/timeouts/not-run. The raw aggregate remains `passed: false`
and the controller exit code remains 1; this is not all-green acceptance.

| Profile | Passed | Failed | File seconds | Bootstrap seconds | Differing bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| reference | 185 | 1 | 329.357 | 272.041 | 78 |
| cocoa-normal | 185 | 1 | 244.699 | 201.374 | 76 |
| cocoa-checking | 185 | 1 | 252.937 | 200.691 | 78 |
| terminal-normal | 185 | 1 | 246.503 | 195.739 | 78 |
| terminal-checking | 185 | 1 | 262.309 | 200.162 | 78 |

## Binary-comparison diagnosis

In every profile both compiler bootstrap stages completed, the first generated
ELN loaded, its `native-compile` function was recognized as native, and that
compiler generated the second ELN. The unchanged final `cmp` assertion failed.
Each same-profile pair has equal size. Every differing byte lies in precisely
the Mach-O `LC_ID_DYLIB` filename, `LC_UUID`, or `LC_CODE_SIGNATURE` data ranges;
all other bytes are identical. The C reference differs by 78 bytes, Cocoa normal
by 76, and each remaining candidate by 78. This diagnosis does not rewrite the
files, remove cases, alter assertions, or turn a failure into a pass. It is not
a comparison of code bytes between different product versions.

The other 185 original cases pass in every profile, including native execution,
type inference, nonlocal exits, buffers, the original 1000-element sort and
100000-allocation loop. No additional candidate failure was observed in this
file. This does not establish complete language compatibility or interactive
GUI/GC/performance acceptance. File durations above are operational timing,
not a speedup claim between differing compiler/Lisp versions.

## Build and isolation

Cocoa and terminal normal/checking products were freshly built with the real
native backend. Both Cocoa apps passed deep strict signature verification.
The build took 1181.815 seconds, peaked at 908115968 accounted bytes and exited
0 with complete owned-process cleanup. Its frozen source manifest SHA256 is
`4d0b3c169fe838e2961b152d03a36eefcfca5a3d37fd781cc149bb14d84460c3`.

These identities cover the complete frozen workspace, including pre-existing
native/runtime work. They do not prove an independently committed clean checkout.
Old native products were not substituted for the current builds. The ignored
reference checkout was not a build dependency.

| Profile | Program SHA256 | Dump SHA256 |
| --- | --- | --- |
| reference | `5b55e93c8304368ee37cc5c537ca3e1370e3b9b3fef049bdce76834a9db75e65` | `6ac1214186022574a70e84a04bb7f2dc44e1ea1d4c90f0d9ce421ddd0f4b4536` |
| cocoa-normal | `dd807afff5feaa43c13693bfe6fd39028dcd19af927a16cbc9e7f3a8ef35fbcc` | `e59627a9a4d8a961799c4ba4a5a61192c23cfa70a0c40b74eb1920583b76bc6c` |
| cocoa-checking | `620de9eba8e52c28f096e71587032f6f98a3f1f53d3360b8daf1f640d3549593` | `555b65d3c0e2ca2b52a235bfcf40b878e21a1f5b7ebcedc03bac18d4aa125dac` |
| terminal-normal | `5f4ff8ad5f5f0461ab946e3a461f3d88f25661544873c4040210065b46b9a5a6` | `c56c7798b73c81ed314e19027974f11c15f45b20762694c9d635099d23c951a8` |
| terminal-checking | `5cf49f553a0709e920e3b3e34b4944bdfd80648b7a4b43a8728f5d26127b59d8` | `b25d0561b90c8a3d0247a0175360283081d599b52398e933c6f1829061367f2a` |

Every process proved actual `native-comp-available-p`, with a single owned
ELN load/target directory before loading the original test. Each profile used
regular hash-verified copies of the eight original inputs and its own delivered
`comp.el`/`comp.elc` bootstrap sources. No installed library or cache was written.
Program/dump, complete selected runtime resources, original corpus, frozen
source and runner identities were unchanged. Each profile retained 11 ELN files;
temporary outputs are not counted as distinct passing tests.

## Finite execution and retained attempts

The initial 180-second per-case estimate was disproved by real bootstrap work:
C stage one took 136 seconds, Cocoa normal 101, followed by a second complete
compiler build. Both initial runs timed out during stage two after 17 passing
cases; 168 cases in each file were not run. The remainder of that attempt was
cancelled rather than repeating the same inadequate budget. Its exit 130 and
complete process cleanup remain recorded; the stopped owner’s artifact flags
were subsequently released through its exact dead-owner journal. A preparation
attempt that encountered those flags did not start tests.

A bounded parent sample showed `comp--final` waiting for its actual compiler
child; CPU activity and completed stage-one ELN provided the workload evidence.
The final run used a fixed 600-second load/case deadline and a 3600-second whole
deadline. It reused the exact built products, inputs and runner. C bootstrap
then finished in 272.041 seconds, including two 135-second stages. Final wall
time was 1359.004 seconds; peak accounted memory was 2701426688 bytes; exit 1
and complete owned-process cleanup remain intact. Earlier attempts are retained
and not added again to the 930 unique final case-product observations.

Raw local evidence: `build/acceptance/runtime-g3-native-current-products-1`,
`build/acceptance/runtime-g3-compiler-original-1`,
`build/acceptance/runtime-g3-compiler-original-2`, and
`build/acceptance/runtime-g3-compiler-original-3`. The read-only Mach-O diagnosis
binds both original ELN hashes and exact excluded metadata ranges.

## Review and remaining scope

Independent review accepted this limited evidence after 15 actual read-only
checks, including its own original-input hashes, case registration/status and
Mach-O parsing of all five ELN pairs. Its observation about equal padding was
applied: filename exclusion now ends after the C-string terminator, leaving
load-command padding in the byte comparison. The conclusion is unchanged.
This round adds compatibility evidence, not a
production compiler change or a new protection test. The original bootstrap
byte-for-byte reproducibility failure remains open on both the C reference and
candidates. No original assertion or completion requirement was relaxed.

G3 remains open: full corpus convergence, terminal EBADF, remote notification
Base64 and incomplete Tree-sitter paths still need work. G1/G2 clean-checkout,
relocation and final UI boundaries, plus native/GC optimization acceptance,
remain separate. The overall Goal stays active, with only G4 delivered (1/7).

Consumed compiler duplicates were retired as [CLN131](../cleanup/reports/2026-10-29-131-consumed-native-compiler-copies.md); source, actual link objects, products/resources and original failures remain.
