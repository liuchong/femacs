# Owned controlling-terminal original-file execution

P449 adds an explicit owned PTY execution context to the existing per-file
runner. Default batch remains available. A small same-PID exec launcher owns
its session/controlling terminal, closes inherited descriptors and drains
combined terminal output with bounded deadline checks. Terminal startup sets
OMP_THREAD_LIMIT=1 and the async native job count to one; native compilation
remains enabled. Actual terminal identity, interactive state and configured
processor/job observations are required. Permission retries occur once only
after the owned child is confirmed exited; live/persistent denial stays fatal.

The shared JSON progress serializer previously called delete-char and changed
the outer command's undo-amalgamation state. Dynamic nil bindings inside its
undo-disabled temporary buffer now preserve the outer flag and boundary cause,
including error unwinding. This affects batch too, so final acceptance reran
all affected files; earlier batch controls were not reused.

## Scoped functional evidence

The unchanged pinned marker file (3 IDs, default batch) and simple overlay
(54 IDs, owned PTY) ran on the installed 31.1 reference and eight current
terminal/Cocoa normal/checking major/young profiles: four actual candidate
products, nine contexts,18 entirely new serial processes. All exit0/complete,
without timeout, protocol/runner error or residual group.513 observations are
504 ordinary passes and9 original noninteractive-only skips. The three echo
bodies/context execute. ID/tag/status/outcome maps agree with the reference.
Raw passed=false and controller exit1 remain because of those skips;
outcome_contract_passed=true is the limited contract, not whole compatibility.

Functional3 took98.450369292s. The controller recorded98.532s and353,320,960B
accounted peak,1GiB fixed/no growth, cleanup complete. Controller figures remain
primary receipts. Source export7,459files includes WIP, not clean HEAD;411
runtime/build inputs and program/dump/resources/App identities remain bound.
No product build, VM change or App replacement occurred. Automatic native
compilation did occur; empty generated_resources describes only test trees.

## Preserved failures and protections

Functional1 retains the checking undo assertion leaving24 text bytes and its
separate memory interruption. Diagnosis1 also hit the unchanged1GiB cap with
concurrent native workers. Diagnosis2 passed only its instrumented context.
Source and timestamps support a prearmed undo timer mechanism, but original
arming/firing was not directly observed. Both observers' JSON changed command
state; neither is acceptance. first_unexpected_undo_resolved=false and
original_concurrency_accepted=false remain. Functional2 (11new+7reused files)
is history under the older serializer; functional3 uses no result reuse.

After functional acceptance, protections establish7 distinct OS contracts
and27 real VM observations (3 event-state tests across9 profiles). This is a
four-run union, not a single all-pass run: first4/6 OS plus27/27 VM; second
mismatch passes while the timeout probe exposes a missing psutil path; third
only timeout passes; fourth adds exited/live/persistent permission boundaries.
The original EPERM error, short-timeout failure and missing-dependency error
remain false in their original results. Only the synthetic probe receives its
required package path. The intermediate source was byte-exact reconstructed
with provenance, not recovered with the original filesystem identity.

The final batch module differs from functional3 only by the bounded permission
handler; removing it yields the prior module AST. The changed negative paths
have specific protections. Successful functional/VM and first four OS evidence
remain valid under that documented reuse boundary. Independent review accepted
the exact source, original receipts, seals, event identities and union.

## Delivery limits

No full G3, whole ERT, physical UI, native-stage, collector-cycle completeness,
performance or App adoption follows. G1/G2/G4 stay3/7; the persistent Goal stays
active. The repeated-message format-string defect is a separate next batch.
CLN188 retains all original results/events/logs/undo traces, source snapshots,
controller receipts and a minimal first-failure skeleton with original
birthtime identity. A copied file does not inherit original inode/birthtime.

[Receipt](2026-11-01-owned-pty-execution-results.json) ·
[CLN188](../cleanup/reports/2026-11-01-188-consumed-owned-pty-state.md)
