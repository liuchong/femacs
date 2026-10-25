# CLN-20261025-18: archive superseded local product copies

Four exact stopped owned subtrees are compressed into persistent private recovery archives. These are two old Mac build-tool trees and two superseded app backups; original project source, the new Android5 export, G4 terminal/Cocoa products, their runtime resources and the root trial app remain expanded. Every regular member content hash and four representative recovery samples are checked before expanded removal. Existing symlinks stay unchanged and are never traversed. Internal hard links must resolve to an already validated named archive member and pass the same original-content hash.

The [manifest](../manifests/2026-10-25-18-old-product-archives.json) binds per-member content identities and the private exact-path/recovery ledger. Restore archived regular members to their recorded owned roots; original directory structure and symlinks remain. Only regular file content and recorded archive member metadata are covered; no restored-app runtime, extended-attribute or signature acceptance is claimed. Current expanded trial products remain usable. Logical or allocated-byte sums are not a physical-volume recovery claim.

Completed: 7307 expanded files, 420120926 logical bytes and 437248000 pre-removal allocated-byte accounting; verified archives occupy 106724815 bytes.
