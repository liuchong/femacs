# CLN144: consumed measurement output and readiness directories

Retire only the empty `out` directory of each completed case under
`build/acceptance/native-measurement-entry-1`, plus the seven isolated
`home/cache/config/state/tmp/eln/out` roots for each of the five readiness
processes under `build/acceptance/native-measurement-inputs-1`. All 185 process
owners have exited successfully and their controllers report complete cleanup.
These paths have no remaining consumer; results and product inputs are retained.

## Actual operation and identities

Removed 215 directories, zero files and zero file bytes. Their measured allocated
blocks were zero; no disk-space recovery is claimed. The 29.567-second operation
records each path/device/inode/mode/mtime before removal, then verifies the same
identity in the production retirement plan. P434 rechecks ownership at deletion.
All selected paths are absent. Public and private inventories agree exactly.

The [215-row manifest](../manifests/2026-10-30-144-consumed-measurement-directories.jsonl.gz)
has SHA-256 `651e58116fffc2d35ed04345530c572142b90b6319b2c18f1f5f650a74bd4d4b`.
It includes repository-relative paths, captured directory identities and the
corresponding hash-only operation ledger hashes. No personal absolute paths or
compiler-encoded leaf names are published.

## Preserved evidence and reconstruction

All 1,419 pre-existing measurement/readiness evidence files are hash-checked
unchanged. Preserve program/dump/resource identities, every workload/startup
result, native preparation observations, all rounds, failure history and original
automatic-retirement decisions. No actual product, source, user configuration,
package, Git object, build input or unrelated root is selected. No editor or
measurement is rerun for cleanup. Recreate the owned empty directories through
the recorded runner/environment setup for replay; original inodes/times are not
promised reproducible.

The earlier automatic removal of empty `home/cache/config/state/tmp/eln` trees
had file-only ledgers. Their file count was zero, but their directory identities
were not recorded. This is an explicit historical evidence gap, not a claim that
the new manifest reconstructs those earlier removals. The later 215-directory
operation uses the repaired owner and has complete captured directory metadata.
See [P434 validation](../../incidents/2026-10-30-retirement-directory-identities.md)
and the [measurement scope](../../releases/2026-10-30-native-measurement-entry.md).
