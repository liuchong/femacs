# Preparation interruption after the complete D23 build

The original `runtime-d23-release-host-cohort` build completed all20 normal/checking
configurations and61 profiles from6956 uniform frozen inputs. Its final receipt
passes; the original controller exits0 after9017.025seconds with complete cleanup.
All20 individual product proofs and the final actual `read_build` validation
pass. The136048 selected artifact paths are released without errors;10757 unique
contents are retained in a566002539-byte verified archive. These are build and
preservation results, not semantic, compatibility or performance acceptance.

The first `runtime-d23-prepare` action was interrupted at the tool boundary.
On continuation its original exec handle was unavailable, its monitor was absent
and its original stage was reparented to init. The stage's exact PID and birth
matched its own journal before SIGTERM was sent. Native fixture compilation had
not started and the negative fixture output root did not exist.

SIGTERM interrupted a journal write. Its partial process-named temporary file
remained; unwind reused that name, producing FileExistsError while recording
release. The original fatal receipt, partial write and exact pre-recovery journal
are retained. After confirming the owner had exited, the existing dead-owner
release command restored only its recorded inodes. The journal then reports
released with zero release failures. No editor product, frozen input, source or
failure evidence was deleted or overwritten.

P-378 gives every artifact-journal temporary file a unique name, retaining
exclusive creation, flush/fsync and atomic replacement. Eight actual owner/OS
integration cases pass, including a real SIGTERM after a partial write: the same
process successfully records release, all owned flags are restored and the
partial-write evidence remains. The controlled check exits0 with complete
cleanup in1.013seconds. This does not approve native/collector runtime behavior.

The corrected module is a coordinator dependency; D23's frozen source and
product identities remain unchanged. Independent scoped review confirms P-378 and the unchanged product boundary.
The fresh `runtime-d23-prepare-2` passes all32 native profiles and full identity
checks, then exits0/reaped after447.430seconds; its preservation journal is released
without failures. The runtime execute phase has started, with protective components
still subsequent. The original interrupted action remains a failure, not a completed
preparation. Current seven-part Goal status remains open.
