# Unsigned reader construction: checking assertion correction

The checking runtime aborted while loading the unchanged stable DBus test file,
before any test ID was registered. Its high unsigned literal reached
`make_biguint`, whose assertion first narrowed `uintmax_t` to signed `EMACS_INT`.
Values from `0xe000000000000000` through `0xffffffffffffffff` then look like
negative fixnums, so the signed overflow predicate alone rejects a valid bignum.
Earlier normal runs loaded the same 26 IDs. Original abort records remain.

`src/bignum.rs` now retains the signed predicate and also checks the unsigned
upper bound, matching the existing `make_uint` condition and the C comparison.
The conversion, allocation, signed constructor and surrounding control flow are
unchanged. Before SHA-256: `af8bb41541034acb815f4ffa4ece30cd3f49374cb707ae8064b547e78b179b32`;
after: `03ce25a0d2757a17324417cc9d4f35460dc6d4b52ef183f26e0317f93da226c3`.

## Build and functional evidence

A fresh source epoch contains 7,500 inputs, manifest SHA-256
`8c39af5715834ea679856729d0a7b7f414af2649936ffb5982c8be1828705f06`.
Its terminal-native normal/checking configuration completed both builds, links
and dumps. The new catalog contains those two profiles plus the installed
reference; it is not the full 61-profile baseline. Source digest, configuration
digest and native ABI differ from the old epoch. Old source, products and
cohort results were retained without relabelling; the trial App was not replaced.
The export captures current tracked/new inputs, including existing workspace WIP;
it is not claimed to differ from the old epoch solely by this assertion.

| Observation | Actual result |
| --- | --- |
| Original stable DBus file, two native pairs, reference/candidate on each | Four complete exit-0 files, 26 IDs each; each has 2 passed + 24 original skips, zero failures/timeouts |
| Checking candidate loads the original high unsigned literal | Complete 26-ID event stream; original pre-load abort is absent in this observed configuration |
| Eight unsigned boundaries on reference, normal and checking | 24 passed observations: read, decimal/hex conversion, printer/read round trip, arithmetic, shifts, masking and bignum boundary |
| `UINT64_MAX + 1` | Three exact `18446744073709551616` results |
| Post-functional `rtl07-reader-printer-roundtrip` protection | Three ERT passes, including nine high unsigned input observations; one valid reference run reused, two candidate runs newly executed |

All original DBus file-level `passed=False` values and skip statuses remain.
These files load the problematic literals before their test guards; the result
does not establish live D-Bus session communication. The public reader/printer
fixture gains three assertions inside its existing test ID, preserving the other
eight IDs and original observations. It is a separately bound test input; the
already-built source export was not modified to add the protection.

The numeric completion has empty stderr on all three products. Loading the
shared protection fixture emits four known warnings from its separate intentional
wrong-arity definitions on every product. Those raw 652-byte diagnostics remain;
the exact expected warning class is recorded separately from the real ERT result.
No output is filtered or suppressed.

## Execution and failure boundaries

The finite single-file regression uses the same owned fixture copies, original
runner, complete 54-event/26-ID validation and host filesystem sandbox. It has
384 MiB preparation and 256 MiB per-invocation disk admission, appropriate to its
fixed input set; the full 555-file matrix keeps its unchanged 3 GiB reserve.
No original test body, assertion, tag or ID is changed or omitted from the
requested file. Numeric and protection processes have 60-second limits; build
and runtime trees retain the 16 GiB cap. All completed owners were reaped and
artifact journals released with no release failures.

Build elapsed 660.377 seconds; successful numeric completion 10.464 seconds;
successful protection continuation 10.459 seconds. These are execution times,
not performance comparisons. Retained earlier attempts include the full-matrix
storage-reserve interruption, a missing lexical declaration in the owned numeric
harness, the initial over-strict protection stderr check, and an
`inspect_owned` monitor error before any new protection editor launched. The
last had an empty dead-owner journal, separately released without semantic work.
None of those original results was rewritten as a pass. Completed original-file
and reference protection observations were verified and reused explicitly.

| Completed private receipt | SHA-256 |
| --- | --- |
| New source / terminal-native build | `889b5ca9f12453377b92f02574c34c5e84b68391abc25e7e114cc5c2f54ad0b8` |
| Functional completion, including unchanged four original-file observations | `1344a1f67538a82d0393d9037e318bcfb463b16be7f2a127263bf431136f919f` |
| Post-functional protection completion | `15d3078e505081aaaf13a08729e0c31613f395ff91e3525c1b1bfc9885d3f37e` |

Independent source, functional and protection review accepted
these limited observations. Consumed generated units and successful verification
state are recorded in [CLN193](../cleanup/reports/2026-11-02-193-unsigned-bignum-build-state.md).
Cocoa, the remaining new-source configurations, full stable G3, qualified G5
performance and the whole Goal remain pending. G1/G2/G4 remain the three delivered
Goal items; this component does not change that count.
