# CLN-20261025-14: archive superseded Android source exports

The first archive-validation attempt rejected a hard-link member before any
source-export removal. Its archive and failed operation log remain private.
The successful continuation accepts only regular data or an internal hard link
to an already validated named member, and checks its resolved bytes against the
current original file hash. It reused and fully revalidated the first archive;
the remaining archives and all recovery samples were verified before removal.

Three stopped superseded complete Android source exports are compressed into persistent private recovery archives; original project source exports, the new Android5 export and all Mac products stay expanded; earlier Android build output and products remain byte-preserved inside these archives; outer receipts, device logs and screenshots remain expanded. These are tracked-input exports and generated build copies, not the original project source. Only regular file contents are archived and removed; existing symlinks remain unchanged and are never traversed. Every archived member content hash was checked and three representative source files were actually restored before any expanded file was removed. This follows CLN11/12, whose earlier preservation statements describe their own completion time. The fresh Android5 product must have passed before this superseded-export archive is run.

The [manifest](../manifests/2026-10-25-14-superseded-source-exports.json) records content identities and the private exact-path/recovery ledger. Recovery extracts regular members from the verified archive into its recorded owned root; unchanged symlinks and directory structure remain in place. No symlink target is read or removed. Public transient numeric compiler instances are normalized. No uncompressed-byte sum is claimed as actual volume recovery.

Completed: 44677 expanded files, 1905568678 logical bytes and 2004766720 pre-removal allocated-byte accounting; verified archives occupy 528471218 bytes.

Public role names additionally redact machine-root-encoded generated components using deterministic SHA256 names. Content identities, byte counts and operation results are unchanged. Exact original recovery paths remain in the private ledger; these logical names are not extraction paths.
