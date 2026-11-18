# CLN253 — consumed matrix bootstrap cache (2026-11-14)

The release Cargo bootstrap process completed successfully before its driver was copied to the separately owned live build path. Removed its consumed cache after verifying the two executable hashes, distinct inodes and absence of cache readers. The active matrix continues from its unchanged independent driver; its source snapshot and products remain in use.

The directory retirement used the unchanged canonical owned-state reader/deleter. Two preliminary attempts deleted no paths: the first occupancy query covered the active parent, and the second rejected ordinary metadata files as directory selections. The successful exact-cache scope retired the release directory, then verified and removed the two cache files and empty cache root.

Counts: {"directories": 223, "files": 439, "logical_bytes": 55543312, "prior_allocated_bytes": 56963072}

All per-node public identities are retained in the [manifest](../manifests/2026-11-14-253-consumed-matrix-bootstrap.json). Its canonical ledger projection was compared line by line to EOF before the three consumed private retirement metadata files were removed. Counts include those metadata files and the empty root; node-level allocated bytes do not establish unique physical reclamation.

The active build driver hash, all119 existing WIP contents and the Git index matched after cache deletion. No immutable source or product file was unlocked or deleted, no archive was created, and the full20-configuration/61-profile build remains in progress. Regenerate a future bootstrap with the maintained release xtask Cargo command; this live independent driver needs no cache restoration.
