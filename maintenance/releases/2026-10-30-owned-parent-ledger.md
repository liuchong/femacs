# Durable identities for automatically removed environment parents

The per-file runner previously recorded its selected case-state children, then
removed two empty parents outside that ledger. Its owner receipt recorded that
removal, but full pre-delete parent identities were unavailable. CLN166 and the
CLN164 correction disclose this historical limitation; old manifests stay intact.

P443 moves both parent removals into the existing retirement operation. The
caller supplies the original case/container nodes. The operation accepts only
that exact pair, checks canonical paths and device/inode ownership, rejects
unselected members and evidence inside the container, and queries open handles
across that owned container. It writes both fresh identities to the same public
ledger/private plan and synchronizes those files and their directory before
deleting anything. Parent-inclusive schema 3 counts both parents once; existing
child-only callers retain schema 2. Normal owner receipts are written only after
the operation returns. A failure does not acquire a completion claim; partial
deletions retain the pre-delete plan for diagnosis.

Actual filesystem integration preserves the old child-only 3-file/5-directory
case, verifies a parent-inclusive 3-file/7-directory case with ten ledger rows,
and preserves a real failed producer's state. Its three bounded Python child
processes are reaped; no editor compatibility is inferred from that test.

Affected real-runner integration executes the complete original marker file on
the installed reference and normal terminal: three IDs each, six passes. Both
case retirements contain ten directory rows, including the two fully recorded
parents. Original file results and owner receipts retain matching hashes. Full
pinned inputs, products/resources and the trial App remain unchanged. This
checks the changed cleanup path, rather than adding a new compatibility claim.
The complete test copies contain no new generated resources.

After integration, 14 retirement protections and eight affected stable-runner
contracts pass. They cover durable parent identities before the first unlink,
parent replacement, unexpected siblings, evidence containment, occupancy,
failed durable writes and partial final-parent removal. Existing directory
replacement/mode/symlink, new-member, failed-process and occupancy protections
also remain. These component tests use tiny filesystem fixtures and injected
failure boundaries; they do not run the VM or prove editor/GC correctness.

The actual marker driver has a 120-second protected-owner deadline and a
20-second per-load/case deadline, two serial editor processes, and two complete
20,996,952-byte input copies. Expected runtime was 2–20 seconds below 512 MiB;
observed driver time is 3.309251792 seconds. The primary controller records
exit 0, complete owned cleanup, 5.103 seconds and 104,333,312 accounted peak
bytes. Its metrics are primary-side receipts. Filesystem integration took
0.369769833 seconds with five-second child deadlines and a 45-second outer
deadline. The 14/8 component groups took 1.109/0.011 seconds, bounded by
90/60-second outer deadlines. These are execution costs, not performance gains.

Independent review finds no confirmed defect in P443 and the six tools. The
review reads the original results without rerunning products; live App/corpus
hashes and controller metrics remain primary-side proof. The partial-removal
protection directly drives retirement, not the batch exception handler.
The former `_retire_empty_parents` body is removed after its work moves into
the ledger-bound operation; no fallback unrecorded deletion remains.

[CLN167](../cleanup/reports/2026-10-30-167-consumed-parent-ledger-state.md)
retires the two consumed test copies and context links after review, retaining
original evidence, inputs and products. Its twenty automatically removed
directories include all four parents with complete private pre-delete identities.
See the [bounded receipt](2026-10-30-owned-parent-ledger-results.json).
Original raw observations are in
`build/acceptance/empty-parent-{filesystem,original-file,protection,stable-protection}-1`.
No product build, policy/default change or trial App replacement occurs.
G1/G2/G4 remain 3/7; G3/G5/G6/G7 and the original Goal remain open.
