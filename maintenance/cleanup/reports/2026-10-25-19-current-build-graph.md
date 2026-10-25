# CLN-20261025-19: archive current completed build graph, retain products

Archive only regular non-JSON/non-log files from the completed Android5 isolated source/build export. The actual signed APK, normal/checking shared libraries and their hashes stay expanded at their existing paths. All JSON receipts/configuration and log files, original project source and root Mac trial app remain expanded. The APK is self-contained and is the unchanged input for device acceptance; generated source/objects are not runtime dependencies. Every archived member is content-checked and a representative member is actually restored before expanded removal. Existing symlinks remain unchanged, without following their targets.

The [manifest](../manifests/2026-10-25-19-current-build-graph.json) binds every archived regular member, protected product identities and the private exact-path ledger. Recovery restores regular members into the recorded owned root; receipts, directories and symlinks remain. No restored-build success or physical-volume recovery amount is inferred from logical byte counts.

Completed: 21343 expanded files, 848237653 logical bytes and 895565824 pre-removal allocated-byte accounting; verified archives occupy 225859709 bytes.

Public role names additionally redact machine-root-encoded generated components using deterministic SHA256 names. Content identities, byte counts and operation results are unchanged. Exact original recovery paths remain in the private ledger; these logical names are not extraction paths.
