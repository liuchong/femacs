# G4 delivery: tooling renovation and recovered local products

## Delivered unit

Implementation commit: `5118ed08ae3d12fa6693290f5a030499f49d6ad5`.
Its signature verifies as good (`git log -1 --format=%G?` reports `G`).
The actual push advanced `origin/master` from `01bfbb7f` to `5118ed08`;
a subsequent remote `refs/heads/master` query returned the full commit above.
The commit contains166 changed paths and the required skip-CI marker.
No remote CI was used. This document records that already delivered unit;
its own later documentation commit is discoverable through Git history.

The [G4 construction contract](../../docs/tooling-modernization.md) owns the
full C01-C13/T01-T48 scope. The delivered changes replace legacy build/probe/
package policy with typed owners, independent manual generation, bounded device
evidence and documented retention decisions. Necessary platform bridges and
source-data generators remain. A filename's language alone was not a reason
for removal; no additional platform was silently retired.

Consolidated validation also repaired actual target file flags, Android startup
initialization, CCL finish handling, Android key-name capacity and platform
function-key offsets. It does not change the Lisp bindings, bytecode algorithm
or collector policy. P-336 through P-345 in `PORTING.md` retain the precise
decisions and failures that led to these corrections.

## Current artifacts and identities

| Artifact | Retained location | SHA-256 |
| --- | --- | --- |
| Cocoa executable | `dist/candidates/g4-macos-2/Femacs.app/Contents/MacOS/Femacs` | `2caf5cf997fffb646be5558d573ae19e91ded50af7fe121356ccb62eb43f8559` |
| Cocoa dump | App resources, identified in `dist/candidates/g4-macos-2/identities.json` | `fa6c868ae3438ba0eaa9d9d3e43c0d479dddc5313237eeafdae11a7fccbb4df7` |
| Android APK | `dist/candidates/g4-android-10/femacs.apk` | `ed2ca84b00f2fc31a09c68ba32ee209b69d5024dbf4b81bd88bc61fd7bc8da73` |
| Android normal native library | `dist/candidates/g4-android-10/libemacs.so` | `f8cde15b4dc8010c61b07f2219b2e3b1db2051e377d642b0e896994903f08bd2` |

Generated candidates and raw receipts are local ignored outputs, not source
inputs or Git-delivered binaries. The declared G4 Mac artifact is the candidate
App above. The previously tried `dist/macos/Femacs.app` remains unchanged until
G1 native acceptance. Retained terminal executable/dump copies require their
original exported resource tree; they are not a relocated terminal distribution.

## Actual validation and its bounds

| Work | Retained receipt | Proven result |
| --- | --- | --- |
| Full fresh Mac source export | `build/acceptance/g4-macos-products-restored-2/receipt.json` | 6800 input identities preserved; optional reference tree excluded. Terminal build537.413s and Cocoa526.680s both succeeded. |
| CCL behavior | Same Mac receipt, actual per-product logs |15/15 each terminal/Cocoa; eight unchanged stable-control cases and seven additional cases. No universal Lisp-compatibility claim. |
| Info and package relocation | Same Mac receipt and retained App identities |68 manuals plus non-Top lookup; relocated Cocoa succeeds with source export unavailable; original/relocated strict signatures and binary/dump identities preserved. |
| Android native/package gates | `.femacs-cache/g4-keyboard-product-10/receipt.json` | Normal/checking link, ELF/JNI/import gates, signature/alignment,2972 unchanged nonnative APK members. This incremental rebuild reuses358 verified unchanged objects. |
| Actual Android normal product | `.femacs-cache/g4-android-device-22/receipt.json` | Exact APK/library installed on API36 ARM emulator;11 named cases pass, including points1/6/5/6, exact Chinese clipboard, per-run rendering and both rotations. Process survives20.492s, zero fatal markers, cleanup complete. |
| Capacity and key-table regressions | Source-extraction receipts under `.femacs-cache/android-keysym-capacity` and `.femacs-cache/keyboard-platform-offset` |11 capacity cases and20 platform/table cases. Old capacity code fails the adjacent-byte canary; this is distinct from device/JNI coverage. |
| Geometry/key/render evidence | `build/pair-assessment-20261024/next-stage/g4-d21-geometry-keys-1.log` |14 affected component cases; malformed PNG/XML and observation/capture errors remain failures, initial geometry retries stay bounded. Capture is not atomic. |
| Probe integration repair | `build/acceptance/g4-all-probes-2/receipt.json` and `build/acceptance/g4-inline-3/report.json` | Original aggregate remains failed for inline; repaired actual inline probe passes separately. Unchanged passing routes are reused, not relabelled as a rerun. |
| Platform floor | `build/acceptance/g4-api-floor-3/receipt.json` | Seven actual extracted source-method values pass; not an API21 device certificate. |
| Final cleanup and inventory | `build/pair-assessment-20261024/next-stage/g4-cleanup-recovery-14.log` and `g4-final-inventory-1.log` | Recovery2/2 covers52 actual withdrawn source entries; inventory1/1 covers239 baseline plus16 additions. No whole-product rollback claim. |

The recovered Mac build's controller completed1073.449s, peak871890944 bytes.
The final Android device controller completed72.294s, peak5031985152 bytes.
Owned children were reaped in both runs. These are individual measured runs,
not total project resource consumption, developer effort or performance gains.
No total token, money or wall-clock estimate is invented from those receipts.

## Independent findings and traceability

The independent D22 reviewer inspected the frozen affected source and actual
current receipts. It accepted P-343/P-344/P-345 and the remaining affected G4
changes with no new blocking source defect. It retracted the adjacent timeout
root corruption claim and the mistaken Ctrl-A failure attribution. The retained
device20 commands show Ctrl-A and Ctrl-E succeeded, followed by an undefined
left-arrow key. P-345 then fixes that actual failure on device22.

The review identified stale progress, inconsistent CLN03-06 recovery fields,
artifact publication ambiguity and missing signed delivery. Metadata was
corrected using actual isolated recovery; the candidate publication decision is
explicit above; signed code delivery is recorded here. Production/test/config
identities stayed unchanged after the final source review. Private conversations,
native session identifiers and raw collaboration logs are deliberately outside
Git; this summary retains the technical findings and public decision anchors.

The subsequent adoption reply accepts the delivered identities and closure,
and finds no substantive source/product/device mismatch. It catches two stale
cross-references: the CLN03-06 ledger still says recovery/delivery pending, and
the G4 D0 stage checkbox remains unchecked. Those are now synchronized with the
same existing evidence. This is a documentation correction, not another product
validation pass or permission to expand the acceptance claim.

Every cleanup batch is linked from the [cleanup ledger](../cleanup/LOG.md).
CLN01-02 belong to stable-delivery candidate/guard work; CLN03-26 cover this G4
source withdrawal, archive and owned-cache/AVD cleanup. Each report names its
manifest, reasons, replacement, identity and recovery/rebuild boundary. Source
withdrawals have isolated Git-blob recovery evidence. Reconstructible stopped
AVD files do not promise exact guest-state restoration; pre-removal allocation
is not measured volume space released. This documentation delivery deletes no
files and requires no new cleanup batch.

The [artifact-loss incident](../incidents/2026-10-25-g4-build-artifact-loss.md)
remains unresolved as to actor and exact removal time. Earlier passing logs did
not prove products still existed. Fresh recovery now supplies current products;
it does not erase the incident or old failed receipts. Initial fixture compile,
probe, device and controller-admission failures remain recorded as failures.

## Open work after G4

G4's scoped tooling/cleanup delivery is complete. It does not close the seven-part
Goal or the user's100% compatibility and real-performance-improvement requirements.
G1 Finder external drop and multi-window IME still need unlocked physical native
acceptance. G2/G3 final combined acceptance remains separate from these bounded
build/behavior checks. API21 devices, Windows binaries, all JNI failure paths
and universal plugin compatibility are not certified by this cohort.

G5-G7 remain the next runtime stage: finish the real-source instrumentation,
compiler/backend and collector design; write all cases without running; complete
the whole implementation; then perform consolidated stable-baseline comparison,
stress and independent acceptance. No native/JIT or generational-GC candidate
has been enabled by this G4 delivery, and no speedup is claimed.
