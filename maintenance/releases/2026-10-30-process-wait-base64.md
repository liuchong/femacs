# Unlocked process waits and the complete Base64 byte domain

Two bounded production corrections preserve public behavior. Neither this
selection nor prior passing suites establish complete language compatibility,
resolve the intermittent remote09 failure or establish a performance gain.

## Process wait ownership

`src/process.rs` tracks each active wait with a stack-based intrusive node,
registered for exact unwind cleanup and explicit process GC marking before
publication. Under the existing thread lock, a waiter avoids consuming another
active waiter's process, including its separate stderr process. Unlocked
processes without another active waiter remain available when
`JUST-THIS-ONE` is nil. Status, EOF and descriptor ownership remain coordinated
through normal and nonlocal exits. The exported process/thread ABI is unchanged;
`src/thread.rs` is unchanged.

Independent review rejected the initial broad suppression because a real
two-process/idle-thread probe lost another process's output. It also rejected
silent registration overflow, incorrect nested cleanup and the later fixed
128-entry limit. The final implementation introduces no arbitrary wait-depth
limit. Two unwind records remove the exact node before clearing/restoring
remaining outer descriptor ownership and the previous input state. The GC
marker retains the waiting process through the live thread's special-binding
stack. No heap allocation is introduced for the registry.

The original `process-tests/multiple-threads-waiting` passes in both actual
normal/checking products. A separate public-interface probe exercises nil and
true `JUST-THIS-ONE` in both products, matching the retained reference control.
Four subsequent protections pass in each product: repeated unlocked EOF,
payload/staggered EOF, unrelated output/sentinel delivery, and 140 nested
waits with nonlocal exits and preserved outer ownership. These are eight
protection observations, not a full process/network suite.

A separate post-integration protection explicitly collects three times while a
worker's wait is suspended and again in its output filter. Both normal/checking
products preserve the exact payload and process exit, two further observations
of the newly registered GC-root path.

## Base64 boundary

`src/fns.rs` previously allocated 255 lookup entries per alphabet while indexing
with every possible byte, including 255. Both alphabets now contain 256 entries
with an explicit invalid final entry. Valid mappings, whitespace, padding,
strict errors, explicit ignore-invalid handling and buffer mutation remain.
The inherited C declaration shares the undersized allocation; undefined reads
are not a compatibility requirement.

Four integration IDs cover binary string/buffer round trips, invalid input
with unchanged buffer/point/markers, explicit permissive mode, and alphabet,
padding and whitespace handling. Together with five unchanged pinned stable
original IDs, all nine pass on installed 31.1 reference, normal and checking:
27 functional observations. Two post-integration protections cover all 256
possible first bytes and byte 255 in each quartet slot on all three products:
six protection observations. No original test is edited or waived.

## Products, resources and execution

The existing source-bound Base64 normal/checking objects are compiled once.
After the partner explicitly hands back frozen process code, its unit is
compiled in both configurations. Checking and normal products are separately
linked, dumped and retained, with normal restored as the current terminal.
The build records actual object hashes, configuration, source, executable,
dump, link inputs and runtime-resource identities. This is an affected-unit
rebuild using retained objects, not a fresh full-source build or a Cocoa/App
publication. The current graphical application is not republished by this work.

The first copied checking program cannot start because the snapshot omitted its
relative-rpath runtime libraries. Its dyld failure and false result remain.
Copying the existing TLS, color-management and parser library trees completes
the snapshot; binaries/dumps and source are unchanged, and no rebuild is used
to hide this preparation failure. Corrected runs bind the full resource trees.

| Execution | Result | Wall seconds | Peak accounted bytes |
| --- | --- | ---: | ---: |
| Base64 unit, both configurations | exit 0, reaped | 2.554 | 328679424 |
| Process unit, two links and dumps | exit 0, reaped | 7.670 | 347947008 |
| Base64 reference functional | 9 pass | 1.283 | 81592320 |
| Base64 checking functional | 9 pass | 4.394 | 57311232 |
| Base64 normal functional | 9 pass | 1.545 | 27770880 |
| Process functional | 2 original passes, 4 public-contract observations | 2.820 | 52822544 |
| Process protection | 8 pass | 3.340 | 61818416 |
| Suspended-wait/filter GC protection | 2 pass | 1.797 | 27017216 |
| Base64 protection | 6 pass | 2.807 | 73891840 |

All successful controller trees finish and are reaped. The two-unit build has
a 210-second whole deadline; Base64 cases have 20-second process deadlines,
and process cases have 8/15-second deadlines. The selected algorithms use
bounded small payloads, 256 byte values, ten subprocesses per concurrency
round, twelve rounds and 140 nested waits; no unbounded stress run is started.
The table records whole-controller times, not individual test durations or
benchmark speed. Every ERT case's own duration remains in the raw result.

Actual receipts remain under `build/acceptance/`:

- `p435-p427-products-1/products.json` and `products-with-runtime.json`;
- `p435-functional-reference-1`, `p435-functional-normal-1`,
  `p435-functional-checking-2`; failed `p435-functional-checking-1` is retained;
- `p435-protection-{reference,normal,checking}-1`;
- `p427-coherent-functional-1` and `p427-coherent-protection-1`;
- `p427-coherent-gc-protection-1`;
- the untouched initial `p427-other-output-contract-1` failure and reference.

The previous second probe directory was overwritten during a partner rerun.
The earlier observed receipt hash is recorded privately, but its full old
content cannot be claimed retained or reconstructed. The initial failure is
unchanged. New runs use fresh output directories and refuse existing paths.
Earlier repeated reference stress also timed out and was reaped; a matching
timeout alone does not prove the proposed descriptor race on that reference.

CLN147 removes only two consumed generated-unit directories and fourteen
successful isolated case environments: 138 files/100 directories, 4,868,938
logical bytes. Actual objects, program/dump/runtime resources and raw records
remain unchanged. The failed snapshot environment is retained. See the
[identity and recovery report](../cleanup/reports/2026-10-30-147-consumed-core-build-copies.md).

The new cases can be run with the selected actual `PROGRAM` and matching
`DUMP`, under the current resource controller, for example:

```sh
"$PROGRAM" --dump-file="$DUMP" -Q --batch -l tools/fixtures/base64-functional.el \
  --eval '(ert-run-tests-batch-and-exit "^fe-base64-functional/")'
"$PROGRAM" --dump-file="$DUMP" -Q --batch -l tools/fixtures/base64-protection.el \
  --eval '(ert-run-tests-batch-and-exit "^fe-base64-protection/")'
"$PROGRAM" --dump-file="$DUMP" -Q --batch \
  -l tools/fixtures/process-thread-wait-protection.el \
  -l tools/fixtures/process-thread-wait-gc-protection.el \
  --eval '(ert-run-tests-batch-and-exit "^fe-process-thread-wait/")'
```

G1/G2/G4 remain accepted, three of seven outcomes. Full G3/G5-G7, measured
performance improvement, the compiler acceptance-policy decision and retained
intermittent failures remain open under the existing Goal.
